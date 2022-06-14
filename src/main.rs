use std::{collections::BTreeMap, fs::File, io::Write};

use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaProps;
use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::JSONSchemaPropsOrArray;

fn main() {
    let out_file = "src/lib.rs";

    let url = "https://raw.githubusercontent.com/prometheus-community/helm-charts/main/charts/kube-prometheus-stack/crds/crd-prometheuses.yaml";
    let bytes = reqwest::blocking::get(url).unwrap().bytes().unwrap();

    let crd: k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::CustomResourceDefinition = serde_yaml::from_slice(&bytes).unwrap();
    let spec = crd.spec;

    let group = spec.group;
    let kind = spec.names.kind;

    let mut f = File::create(out_file).unwrap();

    for version in spec.versions {
        build_resource(
            &mut f,
            &group,
            &version.name,
            &kind,
            version
                .schema
                .as_ref()
                .unwrap()
                .open_api_v3_schema
                .as_ref()
                .unwrap(),
        )
        .unwrap();
    }
}

fn build_resource<W: Write>(
    f: &mut W,
    group: &str,
    version: &str,
    kind: &str,
    schema: &JSONSchemaProps,
) -> anyhow::Result<()> {
    writeln!(
        f,
        "
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    "
    )?;

    let description = schema.description.as_deref().unwrap_or_default();
    for line in description.lines() {
        writeln!(f, "/// {}", line)?;
    }
    writeln!(
        f,
        "pub struct {kind} {{
    pub metadata: ObjectMeta,"
    )?;

    let mut structs = BTreeMap::new();
    for (prop, props) in schema.properties.as_ref().unwrap() {
        get_structs_to_make(prop, props, &mut structs);
    }

    let skippable_meta = ["apiVersion", "kind", "metadata"];
    for (property, _props) in schema.properties.as_ref().unwrap() {
        if skippable_meta.contains(&property.as_str()) {
            continue;
        }
        if property == "type" {
            writeln!(f, "pub r#{}: {},", property, camel_case(property))?;
        } else {
            writeln!(f, "pub {}: {},", snake_case(property), camel_case(property))?;
        }
    }
    writeln!(f, "}}")?;

    for (property, props) in structs {
        // debugging
        // writeln!(f, "{} {:#?}", property, props)?;

        make_struct(f, &property, &props)?;
    }

    let gv = format!("{}/{}", group, version);
    writeln!(
        f,
        "impl k8s_openapi::Resource for {kind} {{
    type Scope = k8s_openapi::ClusterResourceScope;

    const API_VERSION : &'static str = \"{gv}\";
    const GROUP : &'static str = \"{group}\";
    const KIND : &'static str = \"{kind}\";
    const VERSION : &'static str = \"{version}\";
    const URL_PATH_SEGMENT : &'static str = \"TODO\";
}}
"
    )?;

    writeln!(
        f,
        "impl k8s_openapi::Metadata for {kind} {{
    type Ty = ObjectMeta;

    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {{
        &self.metadata
    }}

    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {{
        &mut self.metadata
    }}
}}
"
    )?;

    Ok(())
}

fn get_structs_to_make(
    name: &str,
    props: &JSONSchemaProps,
    structs: &mut BTreeMap<String, JSONSchemaProps>,
) {
    if let Some("object") = props.type_.as_deref() {
        structs.insert(camel_case(name), props.clone());
        if let Some(props) = props.properties.as_ref() {
            for (prop, props) in props {
                get_structs_to_make(prop, props, structs);
            }
        }
    }
    if let Some("array") = props.type_.as_deref() {
        structs.insert(camel_case(name), props.clone());
        if let Some(JSONSchemaPropsOrArray::Schema(schema)) = props.items.as_ref() {
            get_structs_to_make(&format!("{}Item", name), schema, structs);
        }
    }
}

fn make_struct<W: Write>(f: &mut W, name: &str, props: &JSONSchemaProps) -> anyhow::Result<()> {
    if let Some(description) = &props.description {
        for line in description.lines() {
            writeln!(f, "/// {}", line)?;
        }
    }
    writeln!(f, "pub struct {} {{", camel_case(name))?;

    if let Some(properties) = props.properties.as_ref() {
        for (property, props) in properties {
            let property_typename = camel_case(&property);
            if let Some(description) = &props.description {
                for line in description.lines() {
                    writeln!(f, "/// {}", line)?;
                }
            }
            let ty = match props.type_.as_deref() {
                Some("object") => format!("{}", property_typename),
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
                            Some("object") => format!("{}Item", property_typename),
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
                        format!("k8s_openapi::apimachinery::pkg::util::intstr::IntOrString")
                    } else {
                        println!("no type given");
                        continue;
                    }
                }
            };
            if property == "type" {
                writeln!(f, "pub r#{}: {},", property, ty)?;
            } else {
                writeln!(f, "pub {}: {},", snake_case(property), ty)?;
            }
        }
    }

    writeln!(f, "}}")?;
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
        } else {
            if c == '_' {
                found_underscore = true;
            } else {
                if found_underscore {
                    for c in c.to_uppercase() {
                        n.push(c);
                    }
                    found_underscore = false;
                } else {
                    n.push(c);
                }
            }
        }
    }
    n
}
