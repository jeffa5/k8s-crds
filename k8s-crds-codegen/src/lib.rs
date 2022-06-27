use std::collections::BTreeSet;
use std::path::Path;
use std::{collections::BTreeMap, fs::File, io::Write};

use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps;
use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrArray;

const INDENT: &str = "    ";
const OBJECT_META: &str = "k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta";

type Crd =
    k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition;

fn fetch_resource(resource: &str) -> anyhow::Result<Crd> {
    let bytes = reqwest::blocking::get(resource).unwrap().bytes().unwrap();

    let crd = serde_yaml::from_slice(&bytes)?;
    Ok(crd)
}

fn read_resource(file: &Path) -> anyhow::Result<Crd> {
    let f = File::open(file)?;

    let crd = serde_yaml::from_reader(f)?;
    Ok(crd)
}

pub fn build_from_url<W: Write>(writer: &mut W, url: &str) -> anyhow::Result<()> {
    let crd = fetch_resource(url)?;
    build(writer, vec![crd])
}

pub fn build_from_urls<W: Write>(writer: &mut W, urls: &[&str]) -> anyhow::Result<()> {
    let mut crds = Vec::new();
    for url in urls {
        let crd = fetch_resource(url)?;
        crds.push(crd);
    }
    build(writer, crds)?;
    Ok(())
}

pub fn build_from_path<W: Write, P: AsRef<Path>>(writer: &mut W, path: P) -> anyhow::Result<()> {
    let crd = read_resource(path.as_ref())?;
    build(writer, vec![crd])
}

fn build<W: Write>(writer: &mut W, crds: Vec<Crd>) -> anyhow::Result<()> {
    let groups = crds
        .iter()
        .map(|crd| &crd.spec.group)
        .collect::<BTreeSet<_>>();

    assert_eq!(groups.len(), 1, "only support one group for now");

    let mut crd_map = BTreeMap::new();
    for crd in crds {
        for version in crd.spec.versions {
            crd_map
                .entry(crd.spec.group.clone())
                .or_insert_with(BTreeMap::new)
                .entry(version.name.clone())
                .or_insert_with(BTreeMap::new)
                .entry(crd.spec.names.kind.clone())
                .or_insert(version);
        }
    }

    for (group, version) in crd_map {
        writeln!(writer, "pub mod {} {{", dotted_to_snake(&group))?;
        for (version_name, kinds) in version {
            writeln!(writer, "{}pub mod {} {{", INDENT, version_name)?;
            for (kind, version_spec) in kinds {
                build_resource(
                    writer,
                    &INDENT.repeat(2),
                    &group,
                    &version_name,
                    &kind,
                    version_spec
                        .schema
                        .as_ref()
                        .unwrap()
                        .open_api_v3_schema
                        .as_ref()
                        .unwrap(),
                )?;
            }
            writeln!(writer, "{}}}", INDENT)?;
        }
        writeln!(writer, "}}")?;
    }

    Ok(())
}

fn build_resource<W: Write>(
    f: &mut W,
    indent: &str,
    group: &str,
    version: &str,
    kind: &str,
    schema: &JSONSchemaProps,
) -> anyhow::Result<()> {
    writeln!(f, "{}pub mod {} {{", indent, snake_case(kind))?;

    let description = schema.description.as_deref().unwrap_or_default();
    for line in description.lines() {
        writeln!(f, "{}/// {}", indent, line)?;
    }
    writeln!(
        f,
        "{}pub struct {kind} {{
{}{}pub metadata: {OBJECT_META},",
        indent, indent, INDENT
    )?;

    let mut structs = BTreeMap::new();
    let skippable_meta = ["apiVersion", "kind", "metadata"];
    for (prop, props) in schema.properties.as_ref().unwrap() {
        if !skippable_meta.contains(&&**prop) {
            get_structs_to_make(vec![], prop, props, &mut structs);
        }
    }

    // mapping from old name (and parents) to new full name (camelcase)
    let mut rename_mapping = BTreeMap::new();

    for (n, parents_and_props) in &structs {
        for (parents, _props) in parents_and_props {
            let mut name = parents.to_owned();
            name.push(n.clone());
            let name = name.join("");
            rename_mapping.insert((parents.clone(), n.clone()), name);
        }
    }

    for property in schema.properties.as_ref().unwrap().keys() {
        if skippable_meta.contains(&property.as_str()) {
            continue;
        }

        writeln!(
            f,
            "{}{}pub {}: {},",
            indent,
            INDENT,
            make_property_name(property),
            camel_case(property)
        )?;
    }
    writeln!(
        f,
        "{}}}
",
        indent
    )?;

    for (property, parents_and_props) in structs {
        // debugging
        // writeln!(f, "{} {:#?}", property, props)?;

        for (parents, props) in parents_and_props {
            make_struct(f, indent, &property, parents, &props, &rename_mapping)?;
        }
    }

    let gv = format!("{}/{}", group, version);
    writeln!(
        f,
        "{indent}impl k8s_openapi::Resource for {kind} {{
    {indent}type Scope = k8s_openapi::ClusterResourceScope;

    {indent}const API_VERSION : &'static str = \"{gv}\";
    {indent}const GROUP : &'static str = \"{group}\";
    {indent}const KIND : &'static str = \"{kind}\";
    {indent}const VERSION : &'static str = \"{version}\";
    {indent}const URL_PATH_SEGMENT : &'static str = \"TODO\";
{indent}}}
"
    )?;

    writeln!(
        f,
        "{indent}impl k8s_openapi::Metadata for {kind} {{
    {indent}type Ty = {OBJECT_META};

    {indent}fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {{
        {indent}&self.metadata
    {indent}}}

    {indent}fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {{
        {indent}&mut self.metadata
    {indent}}}
{indent}}}
"
    )?;

    writeln!(f, "}}")?;

    Ok(())
}

fn make_property_name(name: &str) -> String {
    if let "type" | "continue" | "for" | "static" = name {
        format!("r#{}", name)
    } else {
        snake_case(name)
    }
}

fn get_structs_to_make(
    parents: Vec<String>,
    name: &str,
    props: &JSONSchemaProps,
    structs: &mut BTreeMap<String, Vec<(Vec<String>, JSONSchemaProps)>>,
) {
    if let Some("object") = props.type_.as_deref() {
        structs
            .entry(camel_case(name))
            .or_default()
            .push((parents.clone(), props.clone()));
        if let Some(props) = props.properties.as_ref() {
            let mut parents = parents.clone();
            parents.push(camel_case(&name.to_string()));
            for (prop, props) in props {
                get_structs_to_make(parents.clone(), prop, props, structs);
            }
        }
    }
    if let Some("array") = props.type_.as_deref() {
        let name = format!("{}Item", camel_case(name));
        if let Some(JSONSchemaPropsOrArray::Schema(schema)) = props.items.as_ref() {
            get_structs_to_make(parents, &name, schema, structs);
        }
    }
}

fn make_struct<W: Write>(
    f: &mut W,
    indent: &str,
    name: &str,
    mut parents: Vec<String>,
    props: &JSONSchemaProps,
    rename_mapping: &BTreeMap<(Vec<String>, String), String>,
) -> anyhow::Result<()> {
    if let Some(description) = &props.description {
        for line in description.lines() {
            writeln!(f, "{}/// {}", indent, line)?;
        }
    }

    let struct_name = rename_mapping
        .get(&(parents.clone(), name.to_owned()))
        .unwrap();
    writeln!(f, "{}pub struct {} {{", indent, struct_name)?;

    if let Some(properties) = props.properties.as_ref() {
        parents.push(camel_case(name));
        for (property, props) in properties {
            let property_typename = rename_mapping
                .get(&(parents.clone(), camel_case(property)))
                .cloned();
            if let Some(description) = &props.description {
                for line in description.lines() {
                    writeln!(f, "{}{}/// {}", indent, INDENT, line)?;
                }
            }
            let ty = match props.type_.as_deref() {
                Some("object") => property_typename.unwrap(),
                Some("boolean") => "bool".to_owned(),
                Some("string") => "String".to_owned(),
                Some("integer") => match props.format.as_deref() {
                    Some("int32") => "i32".to_owned(),
                    Some("int64") => "i64".to_owned(),
                    Some(f) => {
                        println!("unhandled format with integer type {}", f);
                        continue;
                    }
                    None => "i64".to_owned(),
                },
                Some("array") => {
                    let inner_type = if let Some(JSONSchemaPropsOrArray::Schema(schema)) =
                        &props.items
                    {
                        match schema.type_.as_deref() {
                            Some("object") => rename_mapping
                                .get(&(parents.clone(), camel_case(&format!("{}Item", property))))
                                .cloned()
                                .unwrap(),
                            Some("string") => "String".to_owned(),
                            Some("integer") => match schema.format.as_deref() {
                                Some("int32") => "i32".to_owned(),
                                Some("int64") => "i64".to_owned(),
                                Some(f) => {
                                    println!("unhandled format with integer type {} in array", f);
                                    continue;
                                }
                                None => {
                                    println!("no format given with integer in array");
                                    continue;
                                }
                            },
                            Some(i) => {
                                println!("unhandled inner type {} in array", i);
                                continue;
                            }
                            None => {
                                println!("Missing inner type in array");
                                continue;
                            }
                        }
                    } else {
                        println!("missing schema in array");
                        continue;
                    };
                    format!("Vec<{}>", inner_type)
                    // println!("skipping array");
                    // continue;
                }
                Some(t) => {
                    println!("unhandled type {}", t);
                    continue;
                }
                None => {
                    if let Some(true) = props.x_kubernetes_int_or_string {
                        "k8s_openapi::apimachinery::pkg::util::intstr::IntOrString".to_owned()
                    } else {
                        println!("no type given");
                        continue;
                    }
                }
            };
            writeln!(
                f,
                "{}{}pub {}: {},",
                indent,
                INDENT,
                make_property_name(property),
                ty
            )?;
        }
    } else if props.items.is_none() {
        println!("Missing properties {} {:?}", name, props);
    }

    writeln!(
        f,
        "{indent}}}
"
    )?;
    Ok(())
}

fn snake_case(s: &str) -> String {
    let mut n = String::new();
    for c in s.chars() {
        if c.is_uppercase() {
            if n.is_empty() {
                for c in c.to_lowercase() {
                    n.push(c);
                }
            } else {
                n.push('_');
                for c in c.to_lowercase() {
                    n.push(c);
                }
            }
        } else {
            n.push(c);
        }
    }
    n
}

fn camel_case(s: &str) -> String {
    let mut n = String::new();

    let mut found_underscore = false;
    for c in s.chars() {
        if n.is_empty() {
            for c in c.to_uppercase() {
                n.push(c);
            }
        } else if c == '_' {
            found_underscore = true;
        } else if found_underscore {
            for c in c.to_uppercase() {
                n.push(c);
            }
            found_underscore = false;
        } else {
            n.push(c);
        }
    }
    n
}

fn dotted_to_snake(s: &str) -> String {
    s.replace('.', "_")
}