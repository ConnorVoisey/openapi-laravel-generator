use anyhow::Result;
use askama::Template;
use colored::Colorize;
use openapiv3::{OpenAPI, ReferenceOr, RequestBody, Schema, Type};
use std::fs::File;
use std::io::prelude::*;

use crate::templates::{Field, RequestTemplate};

pub fn parse_openapi(input: &String) -> Result<OpenAPI> {
    Ok(serde_yaml::from_str(input)?)
}
fn schema_to_request_template<'a>(
    openapi: &'a OpenAPI,
    class_name: &'a String,
    schema: &'a Schema,
) -> Result<RequestTemplate<'a>> {
    let schema_type = match &schema.schema_kind {
        openapiv3::SchemaKind::Type(schema_type) => schema_type,
        openapiv3::SchemaKind::OneOf { one_of: _ } => todo!(),
        openapiv3::SchemaKind::AllOf { all_of: _ } => todo!(),
        openapiv3::SchemaKind::AnyOf { any_of: _ } => todo!(),
        openapiv3::SchemaKind::Not { not: _ } => todo!(),
        openapiv3::SchemaKind::Any(_) => todo!(),
    };
    let fields = schema_type_to_fields(openapi, schema_type);
    Ok(RequestTemplate { class_name, fields })
}
fn get_schema_from_ref(openapi: &OpenAPI, ref_key: String) -> Option<(String, Schema)> {
    let prefix = "#/components/schemas/";
    if !ref_key.starts_with(prefix) {
        return None;
    }
    let search_key = ref_key.split(prefix).collect::<Vec<&str>>()[1];
    match openapi.components.as_ref()?.schemas.get(search_key)? {
        ReferenceOr::Item(schema) => Some((search_key.to_string(), schema.clone())),
        ReferenceOr::Reference { .. } => None,
    }
}
pub fn write_request_body_to_template(
    openapi: &OpenAPI,
    request_body: &RequestBody,
    out_dir: &str,
) -> Result<()> {
    let json_content = request_body.content.get("application/json").unwrap();
    let schema_or_ref = json_content.schema.as_ref().unwrap().to_owned();
    let (schema_name, schema_item) = match schema_or_ref {
        ReferenceOr::Item(schema) => ("UnnamedSchema".to_owned(), schema),
        ReferenceOr::Reference { reference } => get_schema_from_ref(openapi, reference).unwrap(),
    };
    let model_name = format!(
        "{}Request",
        if schema_name.contains("Models.") {
            schema_name.split("Models.").collect::<Vec<&str>>()[1]
        } else {
            &schema_name
        }
    );
    let template = schema_to_request_template(openapi, &model_name, &schema_item).unwrap();
    write_request_file(template, out_dir)
}
fn write_request_file<'a>(template: RequestTemplate, out_dir: &str) -> Result<()> {
    let page_content = template.render()?;
    let mut file = File::create(format!("{out_dir}/{}.php", template.class_name))?;
    file.write_all(page_content.as_bytes()).unwrap();
    Ok(())
}

fn schema_type_to_fields<'a>(openapi: &'a OpenAPI, schema_type: &'a Type) -> Vec<Field<'a>> {
    let object_type = match schema_type {
        openapiv3::Type::Object(object) => object,
        _ => {
            eprintln!("Non object schemas are not supported");
            return Vec::new();
        }
    };

    object_type
        .properties
        .iter()
        .filter_map(|(key, ref_or_schema)| {
            let schema = match ref_or_schema {
                ReferenceOr::Reference { reference } => Box::new(
                    get_schema_from_ref(openapi, reference.to_string())
                        .unwrap()
                        .1,
                ),
                ReferenceOr::Item(item) => item.clone(),
            };
            let mut rules = vec![if object_type.required.contains(key) {
                "'required'"
            } else {
                "'nullable'"
            }];
            match schema.schema_kind {
                openapiv3::SchemaKind::Type(schema_type) => match schema_type {
                    openapiv3::Type::String(str) => {
                        match str.format {
                            openapiv3::VariantOrUnknownOrEmpty::Item(item) => match item {
                                openapiv3::StringFormat::Date => {}
                                openapiv3::StringFormat::DateTime => rules.push(
                                    r#"'date_format:Y-m-d\TH:i:s\Z', // UTC datetime format"#,
                                ),
                                openapiv3::StringFormat::Password => {}
                                openapiv3::StringFormat::Byte => {}
                                openapiv3::StringFormat::Binary => rules.push("'file'"),
                            },
                            openapiv3::VariantOrUnknownOrEmpty::Unknown(custom_format) => {
                                match custom_format.as_str() {
                                    "email" => rules.push("'email'"),
                                    _ => println!(
                                        "{:5}{}: {}",
                                        "",
                                        "unknown formatting rule doesn't do anything".yellow(),
                                        custom_format
                                    ),
                                }
                            }
                            openapiv3::VariantOrUnknownOrEmpty::Empty => {}
                        }
                        rules.push("'string'");
                    }
                    openapiv3::Type::Number(_) => {
                        rules.push("'numeric'");
                    }
                    openapiv3::Type::Integer(_) => {
                        rules.push("'integer'");
                    }
                    openapiv3::Type::Boolean(_) => {
                        rules.push("'boolean'");
                    }
                    openapiv3::Type::Object(_) => {
                        rules.push("'array'");
                    } //TODO: add proper support for both arrays and objects
                    openapiv3::Type::Array(_) => {
                        rules.push("'array'");
                    }
                },
                _ => {
                    eprintln!("Current only direct schemas are supported");
                }
            };
            Some(Field { name: key, rules })
        })
        .collect()
}
