use k8s_openapi::serde::Deserialize;
use std::path::Path;
use std::{collections::BTreeMap, fs::File, io::Write};
use tracing::{debug, info, warn};

use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrArray;
use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::{
    JSONSchemaProps, JSONSchemaPropsOrBool,
};

const INDENT: &str = "    ";
const OBJECT_META: &str = "k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta";

type Crd =
    k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition;

fn fetch_resource(resource: &str) -> anyhow::Result<Crd> {
    let bytes = reqwest::blocking::get(resource).unwrap().bytes().unwrap();

    let crd = serde_yaml::from_slice(&bytes)?;
    Ok(crd)
}

fn read_resource(file: &Path) -> anyhow::Result<Vec<Crd>> {
    let f = File::open(file)?;

    let mut crds = Vec::new();
    for document in serde_yaml::Deserializer::from_reader(f) {
        let crd = Crd::deserialize(document)?;
        crds.push(crd);
    }
    Ok(crds)
}

pub fn build_from_url<W: Write>(writer: &mut W, url: &str) -> anyhow::Result<()> {
    info!(?url, "Building from url");
    let crd = fetch_resource(url)?;
    build(writer, vec![crd])
}

pub fn build_from_urls<W: Write>(writer: &mut W, urls: &[String]) -> anyhow::Result<()> {
    info!(?urls, "Building from urls");
    let mut crds = Vec::new();
    for url in urls {
        let crd = fetch_resource(url)?;
        crds.push(crd);
    }
    build(writer, crds)?;
    Ok(())
}

pub fn build_from_path<W: Write, P: AsRef<Path>>(writer: &mut W, path: P) -> anyhow::Result<()> {
    let path = path.as_ref();
    info!(?path, "Building from path");
    let crds = read_resource(path)?;
    build(writer, crds)
}

pub fn build_from_paths<W: Write, P: AsRef<Path>>(
    writer: &mut W,
    paths: Vec<P>,
) -> anyhow::Result<()> {
    let mut crds = Vec::new();
    for path in paths {
        let path = path.as_ref();
        info!(?path, "Building from path");
        let mut crd = read_resource(path)?;
        crds.append(&mut crd);
    }
    build(writer, crds)
}

fn build<W: Write>(writer: &mut W, crds: Vec<Crd>) -> anyhow::Result<()> {
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

    write_derives(f)?;

    writeln!(
        f,
        "{}pub struct {kind} {{
{}{}pub metadata: {OBJECT_META},",
        indent, indent, INDENT
    )?;

    let mut structs = BTreeMap::new();
    let skippable_meta = ["apiVersion", "kind", "metadata"];
    for (prop, props) in schema.properties.iter().flatten() {
        if !skippable_meta.contains(&&**prop) {
            get_structs_to_make(vec![], prop, props, &mut structs);
        }
    }

    // mapping from old name (and parents) to new full name (camelcase)
    let mut rename_mapping = BTreeMap::new();

    for (n, parents_and_props) in &structs {
        make_unique_names(
            &parents_and_props.iter().map(|(p, _)| p).collect::<Vec<_>>(),
            n,
            &mut rename_mapping,
        )
    }

    for property in schema
        .properties
        .as_ref()
        .map(|t| t.keys())
        .into_iter()
        .flatten()
    {
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

fn make_unique_names(
    parents: &[&Vec<String>],
    n: &str,
    rename_mapping: &mut BTreeMap<(Vec<String>, String), String>,
) {
    let mut names = Vec::new();
    debug!(?n, len = parents.len(), "Found items for a name");
    for &parents in parents {
        names.push((parents.clone(), n.to_owned()));
    }

    loop {
        let mut counts = BTreeMap::<_, u32>::new();
        for (_, name) in &names {
            *counts.entry(name.clone()).or_default() += 1;
        }

        if names.len() == counts.len() {
            // all unique
            break;
        }

        debug!(?names,names_len=?names.len(), non_unique=?counts.len(), "Still have non-unique names");

        let mut some_parents = false;

        // otherwise go through adding part of the parent to the name.
        for (parents, name) in &mut names {
            if counts.get(name).unwrap() > &1 {
                if let Some(last) = parents.pop() {
                    some_parents = true;
                    *name = format!("{}{}", last, name);
                }
            }
        }

        if !some_parents {
            warn!(?rename_mapping, ?counts, "No more parents to add to names!");
            break;
        }
    }

    for (&parents, (_, name)) in parents.iter().zip(names) {
        debug!(?parents, ?n, ?name, "Creating rename mapping");
        rename_mapping.insert((parents.clone(), n.to_owned()), name);
    }
}

fn make_property_name(name: &str) -> String {
    // banned words as per https://doc.rust-lang.org/reference/keywords.html
    if let "as" | "break" | "continue" | "const" | "crate" | "else" | "enum" | "extern" | "false"
    | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop" | "match" | "mod" | "move"
    | "mut" | "pub" | "ref" | "return" | "self" | "Self" | "static" | "struct" | "super"
    | "trait" | "true" | "type" | "unsafe" | "use" | "where" | "while" = name
    {
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
        } else if let Some(JSONSchemaPropsOrBool::Schema(properties)) =
            props.additional_properties.as_ref()
        {
            let mut parents = parents.clone();
            parents.push(camel_case(&name.to_string()));

            get_structs_to_make(parents.clone(), "value", properties, structs);
        }
    }
    if let Some("array") = props.type_.as_deref() {
        let name = array_item_name(name);
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

    write_derives(f)?;

    writeln!(f, "{}pub struct {} {{", indent, struct_name)?;

    if let Some(properties) = props.properties.as_ref() {
        parents.push(camel_case(name));
        for (property, props) in properties {
            if let Some(description) = &props.description {
                for line in description.lines() {
                    writeln!(f, "{}{}/// {}", indent, INDENT, line)?;
                }
            }
            let ty = get_type(parents.clone(), property, props, rename_mapping);
            writeln!(
                f,
                "{}{}pub {}: {},",
                indent,
                INDENT,
                make_property_name(property),
                ty
            )?;
        }
    } else if let Some(JSONSchemaPropsOrBool::Schema(properties)) = &props.additional_properties {
        parents.push(camel_case(name));
        if let Some(description) = &props.description {
            for line in description.lines() {
                writeln!(f, "{}{}/// {}", indent, INDENT, line)?;
            }
        }
        let value_type = get_type(parents, "value", properties, rename_mapping);
        writeln!(
            f,
            "{}{}pub {}: std::collections::HashMap<String, {}>,",
            indent,
            INDENT,
            make_property_name("properties"),
            value_type
        )?;
    } else if Some(JSONSchemaPropsOrBool::Bool(true)) == props.additional_properties
        || props.additional_properties.is_none()
    {
        // defaults to true when missing
        if let Some(description) = &props.description {
            for line in description.lines() {
                writeln!(f, "{}{}/// {}", indent, INDENT, line)?;
            }
        }
        writeln!(
            f,
            "{}{}pub {}: std::collections::HashMap<String, String>,",
            indent,
            INDENT,
            make_property_name("properties"),
        )?;
    } else if props.items.is_none() {
        warn!(?name, ?props, "Missing properties");
    }

    writeln!(
        f,
        "{indent}}}
"
    )?;
    Ok(())
}

fn get_type(
    parents: Vec<String>,
    property: &str,
    props: &JSONSchemaProps,
    rename_mapping: &BTreeMap<(Vec<String>, String), String>,
) -> String {
    let t = match props.type_.as_deref() {
        Some("object") => match rename_mapping
            .get(&(parents.clone(), camel_case(property)))
            .cloned()
        {
            Some(res) => res,
            None => {
                warn!(
                    ?parents,
                    ?property,
                    "Failed to find type name in rename_mapping"
                );
                "()".to_owned()
            }
        },
        Some("boolean") => "bool".to_owned(),
        Some("string") => "String".to_owned(),
        Some("integer") => match props.format.as_deref() {
            Some("int32") => "i32".to_owned(),
            Some("int64") => "i64".to_owned(),
            Some(f) => {
                todo!("unhandled format with integer type {}", f);
            }
            None => "i64".to_owned(),
        },
        Some("number") => match props.format.as_deref() {
            Some("double") => "f64".to_owned(),
            Some(f) => {
                todo!("unhandled format with number type {}", f);
            }
            None => "f64".to_owned(),
        },
        Some("array") => {
            let inner_type = if let Some(JSONSchemaPropsOrArray::Schema(schema)) = &props.items {
                get_type(parents, &array_item_name(property), schema, rename_mapping)
            } else {
                todo!("missing schema in array");
            };
            format!("Vec<{}>", inner_type)
        }
        Some(t) => {
            todo!("unhandled type {}", t);
        }
        None => {
            if let Some(true) = props.x_kubernetes_int_or_string {
                "k8s_openapi::apimachinery::pkg::util::intstr::IntOrString".to_owned()
            } else if Some(true) == props.x_kubernetes_preserve_unknown_fields {
                "std::collections::HashMap<String, String>".to_owned()
            } else {
                todo!("no type given");
            }
        }
    };
    if props.nullable == Some(true) {
        format!("Option<{}>", t)
    } else {
        t
    }
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
    s.replace('.', "_").replace('-', "_")
}

fn array_item_name(s: &str) -> String {
    if let Some(s) = s.strip_suffix('s') {
        s.to_owned()
    } else {
        format!("{}Item", camel_case(s))
    }
}

fn write_derives<W: Write>(f: &mut W) -> anyhow::Result<()> {
    let derives = ["serde::Serialize", "serde::Deserialize", "Debug"];

    writeln!(f, "#[derive({})]", derives.join(", "))?;
    writeln!(f, "#[serde(rename_all = \"camelCase\")]")?;

    Ok(())
}
