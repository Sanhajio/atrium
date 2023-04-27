use atrium_lex::lexicon::*;
use heck::{ToPascalCase, ToSnakeCase};
use itertools::Itertools;
use std::collections::HashSet;
use std::io::{Result, Write};

pub(crate) struct CodeWriter {
    buf: Vec<u8>,
    schema_id: Option<String>,
}

impl CodeWriter {
    pub fn new(schema_id: Option<String>) -> Self {
        Self {
            buf: Vec::new(),
            schema_id,
        }
    }
    pub fn write_header(&mut self, description: &Option<String>) -> Result<()> {
        writeln!(
            &mut self.buf,
            "// This file is generated by atrium-codegen. Do not edit."
        )
        .ok();
        if let Some(schema_id) = &self.schema_id {
            writeln!(
                &mut self.buf,
                "//! Definitions for the `{schema_id}` namespace."
            )?;
        }
        if let Some(description) = description {
            writeln!(&mut self.buf, "//! {}", description)?;
        }
        Ok(())
    }
    pub fn write_user_type(&mut self, name: &str, def: &LexUserType, is_main: bool) -> Result<()> {
        writeln!(&mut self.buf)?;
        match def {
            LexUserType::XrpcQuery(query) => self.write_query(name, query)?,
            LexUserType::XrpcProcedure(procedure) => self.write_procedure(name, procedure)?,
            _ => {
                if let Some(schema_id) = &self.schema_id {
                    let refname = if is_main {
                        schema_id.clone()
                    } else {
                        format!("{}#{}", schema_id, name)
                    };
                    writeln!(&mut self.buf, "// {refname}")?;
                };
                let defname = if is_main {
                    String::from("Main")
                } else {
                    name.to_pascal_case()
                };
                match def {
                    LexUserType::Record(record) => self.write_record(record)?,
                    LexUserType::XrpcSubscription(subscription) => {
                        self.write_subscription(&defname, subscription)?
                    }
                    LexUserType::Token(token) => self.write_token(&defname, token)?,
                    LexUserType::Object(object) => self.write_object(&defname, object)?,
                    LexUserType::String(string) => self.write_string(&defname, string)?,
                    _ => unimplemented!(),
                }
            }
        }
        Ok(())
    }
    pub fn write_ref_unions(&mut self, ref_unions: &[(String, LexRefUnion)]) -> Result<()> {
        for (name, ref_union) in ref_unions {
            writeln!(&mut self.buf)?;
            if let Some(description) = &ref_union.description {
                writeln!(&mut self.buf, "/// {}", description)?;
            }
            writeln!(&mut self.buf, "#[allow(clippy::large_enum_variant)]")?;
            writeln!(
                &mut self.buf,
                "#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]"
            )?;
            writeln!(&mut self.buf, r#"#[serde(tag = "$type")]"#)?;
            writeln!(&mut self.buf, "pub enum {name} {{")?;
            for r#ref in &ref_union.refs {
                let ref_type = Self::ref_type(&LexRef {
                    description: None,
                    r#ref: r#ref.clone(),
                });
                let rename = if r#ref.starts_with('#') {
                    format!(
                        "{}{}",
                        self.schema_id.as_ref().expect("schema id must be set"),
                        r#ref
                    )
                } else {
                    r#ref.clone()
                };
                let name = ref_type
                    .strip_prefix("crate::")
                    .unwrap_or(&ref_type)
                    .split("::")
                    .map(str::to_pascal_case)
                    .join("");
                writeln!(&mut self.buf, r#"    #[serde(rename = "{rename}")]"#)?;
                writeln!(&mut self.buf, "    {name}({ref_type}),")?;
            }
            writeln!(&mut self.buf, "}}")?;
        }
        Ok(())
    }
    pub fn write_records(&mut self, records: &[String]) -> Result<()> {
        writeln!(&mut self.buf)?;
        writeln!(
            &mut self.buf,
            "#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]"
        )?;
        writeln!(&mut self.buf, r#"#[serde(tag = "$type")]"#)?;
        writeln!(&mut self.buf, "pub enum Record {{")?;
        for r in records {
            writeln!(&mut self.buf, r#"    #[serde(rename = "{}")]"#, r)?;
            writeln!(
                &mut self.buf,
                "    {}(crate::{}::Record),",
                r.to_pascal_case(),
                r.split('.').map(str::to_snake_case).join("::")
            )?;
        }
        writeln!(&mut self.buf, "}}")?;
        Ok(())
    }
    pub fn write_traits_macro(&mut self, traits: &[String]) -> Result<()> {
        writeln!(&mut self.buf)?;
        writeln!(&mut self.buf, "#[macro_export]")?;
        writeln!(&mut self.buf, "macro_rules! impl_traits {{")?;
        writeln!(&mut self.buf, "    ($type:ty) => {{")?;
        for t in traits {
            let parts = t.split('.').collect_vec();
            writeln!(
                &mut self.buf,
                "        impl atrium_api::{}::{} for $type {{}}",
                parts.iter().map(|s| s.to_snake_case()).join("::"),
                parts[parts.len() - 1].to_pascal_case()
            )?;
        }
        writeln!(&mut self.buf, "    }};")?;
        writeln!(&mut self.buf, "}}")?;
        Ok(())
    }
    pub fn write_mods(&mut self, mods: &[String]) -> Result<()> {
        for m in mods {
            if m == "lib" {
                continue;
            }
            writeln!(&mut self.buf, "pub mod {m};")?;
        }
        Ok(())
    }
    pub fn write_to_file(&mut self, file: &mut impl Write) -> Result<()> {
        file.write_all(&self.buf)
    }

    fn write_record(&mut self, record: &LexRecord) -> Result<()> {
        if let Some(description) = &record.description {
            writeln!(&mut self.buf, "/// {}", description)?;
        }
        let LexRecordRecord::Object(object) = &record.record;
        self.write_object("Record", object)?;
        Ok(())
    }
    fn write_query(&mut self, name: &str, query: &LexXrpcQuery) -> Result<()> {
        if let Some(description) = &query.description {
            writeln!(&mut self.buf, "/// {}", description)?;
        }
        let has_params = query.parameters.is_some();
        let has_output = query.output.as_ref().map_or(false, |o| o.schema.is_some());
        writeln!(&mut self.buf, "#[async_trait::async_trait]")?;
        writeln!(
            &mut self.buf,
            "pub trait {}: crate::xrpc::XrpcClient {{",
            name.to_pascal_case()
        )?;
        writeln!(
            &mut self.buf,
            "    async fn {}(&self{}) -> Result<{}, Box<dyn std::error::Error>> {{",
            name.to_snake_case(),
            if has_params {
                ", params: Parameters"
            } else {
                ""
            },
            if has_output { "Output" } else { "()" }
        )?;
        writeln!(&mut self.buf, "        crate::xrpc::XrpcClient::send(")?;
        writeln!(&mut self.buf, "            self,")?;
        writeln!(&mut self.buf, "            http::Method::GET,")?;
        writeln!(
            &mut self.buf,
            "            {:?},",
            self.schema_id.as_ref().expect("schema id must be set")
        )?;
        writeln!(
            &mut self.buf,
            "            {},",
            if has_params {
                "Some(params)"
            } else {
                "Option::<()>::None"
            }
        )?;
        writeln!(&mut self.buf, "            Option::<()>::None,")?;
        writeln!(&mut self.buf, "        )")?;
        writeln!(&mut self.buf, "        .await")?;
        writeln!(&mut self.buf, "    }}")?;
        writeln!(&mut self.buf, "}}")?;
        // parameters
        if let Some(LexXrpcQueryParameter::Params(parameters)) = &query.parameters {
            let required = if let Some(required) = &parameters.required {
                HashSet::from_iter(required)
            } else {
                HashSet::new()
            };

            writeln!(&mut self.buf)?;
            writeln!(
                &mut self.buf,
                "#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]"
            )?;
            writeln!(&mut self.buf, r#"#[serde(rename_all = "camelCase")]"#)?;
            writeln!(&mut self.buf, "pub struct Parameters {{")?;
            for key in parameters.properties.keys().sorted() {
                let property = match &parameters.properties[key] {
                    LexXrpcParametersProperty::Boolean(boolean) => {
                        LexObjectProperty::Boolean(boolean.clone())
                    }
                    LexXrpcParametersProperty::Integer(integer) => {
                        LexObjectProperty::Integer(integer.clone())
                    }
                    LexXrpcParametersProperty::String(string) => {
                        LexObjectProperty::String(string.clone())
                    }
                    LexXrpcParametersProperty::Unknown(unknown) => {
                        LexObjectProperty::Unknown(unknown.clone())
                    }
                    LexXrpcParametersProperty::Array(primitive_array) => {
                        let items = match &primitive_array.items {
                            LexPrimitiveArrayItem::Boolean(b) => LexArrayItem::Boolean(b.clone()),
                            LexPrimitiveArrayItem::Integer(i) => LexArrayItem::Integer(i.clone()),
                            LexPrimitiveArrayItem::String(s) => LexArrayItem::String(s.clone()),
                            LexPrimitiveArrayItem::Unknown(u) => LexArrayItem::Unknown(u.clone()),
                        };
                        LexObjectProperty::Array(LexArray {
                            description: primitive_array.description.clone(),
                            items,
                            min_length: primitive_array.min_length,
                            max_length: primitive_array.max_length,
                        })
                    }
                };
                self.write_object_property(key, &property, required.contains(key), "Parameters")?;
            }
            writeln!(&mut self.buf, "}}")?;
        }
        // output
        if let Some(output) = &query.output {
            writeln!(&mut self.buf)?;
            if let Some(description) = &output.description {
                writeln!(&mut self.buf, "/// {description}")?;
            }
            if let Some(schema) = &output.schema {
                match schema {
                    LexXrpcBodySchema::Ref(r#ref) => {
                        if let Some(description) = &r#ref.description {
                            writeln!(&mut self.buf, "/// {description}")?;
                        }
                        writeln!(
                            &mut self.buf,
                            "pub type Output = {};",
                            Self::ref_type(r#ref)
                        )?;
                    }
                    LexXrpcBodySchema::Union(_) => unimplemented!(),
                    LexXrpcBodySchema::Object(object) => {
                        self.write_object("Output", object)?;
                    }
                }
            }
        }
        // error
        self.write_xrpc_errors(&query.errors)?;
        Ok(())
    }
    fn write_procedure(&mut self, name: &str, procedure: &LexXrpcProcedure) -> Result<()> {
        if let Some(description) = &procedure.description {
            writeln!(&mut self.buf, "/// {}", description)?;
        }
        let has_input = procedure
            .input
            .as_ref()
            .map_or(false, |i| i.schema.is_some());
        let has_output = procedure
            .output
            .as_ref()
            .map_or(false, |o| o.schema.is_some());
        writeln!(&mut self.buf, "#[async_trait::async_trait]")?;
        writeln!(
            &mut self.buf,
            "pub trait {}: crate::xrpc::XrpcClient {{",
            name.to_pascal_case()
        )?;
        writeln!(
            &mut self.buf,
            "    async fn {}(&self{}) -> Result<{}, Box<dyn std::error::Error>> {{",
            name.to_snake_case(),
            if has_input { ", input: Input" } else { "" },
            if has_output { "Output" } else { "()" }
        )?;
        writeln!(&mut self.buf, "        crate::xrpc::XrpcClient::send(")?;
        writeln!(&mut self.buf, "            self,")?;
        writeln!(&mut self.buf, "            http::Method::POST,")?;
        writeln!(
            &mut self.buf,
            "            {:?},",
            self.schema_id.as_ref().expect("schema id must be set")
        )?;
        writeln!(&mut self.buf, "            Option::<()>::None,")?;
        writeln!(
            &mut self.buf,
            "            {},",
            if has_input {
                "Some(input)"
            } else {
                "Option::<()>::None"
            }
        )?;
        writeln!(&mut self.buf, "        )")?;
        writeln!(&mut self.buf, "        .await")?;
        writeln!(&mut self.buf, "    }}")?;
        writeln!(&mut self.buf, "}}")?;
        if procedure.parameters.is_some() {
            // TODO
        }
        // input
        if let Some(input) = &procedure.input {
            writeln!(&mut self.buf)?;
            if let Some(description) = &input.description {
                writeln!(&mut self.buf, "/// {description}")?;
            }
            if let Some(schema) = &input.schema {
                match schema {
                    LexXrpcBodySchema::Ref(_) => unimplemented!(),
                    LexXrpcBodySchema::Union(_) => unimplemented!(),
                    LexXrpcBodySchema::Object(object) => {
                        self.write_object("Input", object)?;
                    }
                }
            }
        }
        // output
        if let Some(output) = &procedure.output {
            writeln!(&mut self.buf)?;
            if let Some(description) = &output.description {
                writeln!(&mut self.buf, "/// {description}")?;
            }
            if let Some(schema) = &output.schema {
                match schema {
                    LexXrpcBodySchema::Ref(r#ref) => {
                        if let Some(description) = &r#ref.description {
                            writeln!(&mut self.buf, "/// {description}")?;
                        }
                        writeln!(
                            &mut self.buf,
                            "pub type Output = {};",
                            Self::ref_type(r#ref)
                        )?;
                    }
                    LexXrpcBodySchema::Union(_) => unimplemented!(),
                    LexXrpcBodySchema::Object(object) => {
                        self.write_object("Output", object)?;
                    }
                }
            }
        }
        // error
        self.write_xrpc_errors(&procedure.errors)?;
        Ok(())
    }
    fn write_xrpc_errors(&mut self, errors: &Option<Vec<LexXrpcError>>) -> Result<()> {
        writeln!(&mut self.buf)?;
        writeln!(&mut self.buf, "pub enum Error {{")?;
        if let Some(errors) = errors {
            for error in errors {
                if let Some(description) = &error.description {
                    writeln!(&mut self.buf, "    /// {}", description)?;
                }
                writeln!(&mut self.buf, "    {},", error.name.to_pascal_case())?;
            }
        }
        writeln!(&mut self.buf, "}}")?;
        Ok(())
    }
    fn write_subscription(&mut self, _: &str, _: &LexXrpcSubscription) -> Result<()> {
        // TODO
        writeln!(&mut self.buf, "// TODO")?;
        Ok(())
    }
    fn write_token(&mut self, name: &str, token: &LexToken) -> Result<()> {
        if let Some(description) = &token.description {
            writeln!(&mut self.buf, "/// {}", description)?;
        }
        // TODO: enum?
        writeln!(&mut self.buf, "pub struct {};", name.to_pascal_case())?;
        Ok(())
    }
    fn write_object(&mut self, name: &str, object: &LexObject) -> Result<()> {
        if let Some(description) = &object.description {
            writeln!(&mut self.buf, "/// {}", description)?;
        }
        let required = if let Some(required) = &object.required {
            HashSet::from_iter(required)
        } else {
            HashSet::new()
        };
        writeln!(
            &mut self.buf,
            "#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]"
        )?;
        writeln!(&mut self.buf, r#"#[serde(rename_all = "camelCase")]"#)?;
        writeln!(&mut self.buf, "pub struct {} {{", name.to_pascal_case())?;
        if let Some(properties) = &object.properties {
            for key in properties.keys().sorted() {
                self.write_object_property(key, &properties[key], required.contains(key), name)?;
            }
        }
        writeln!(&mut self.buf, "}}")?;
        Ok(())
    }
    fn write_object_property(
        &mut self,
        name: &str,
        property: &LexObjectProperty,
        required: bool,
        object_name: &str,
    ) -> Result<()> {
        match property {
            LexObjectProperty::Ref(r#ref) => {
                if let Some(description) = &r#ref.description {
                    writeln!(&mut self.buf, "    /// {}", description)?;
                }
                let ref_type = Self::ref_type(r#ref);
                let field_type = if required {
                    ref_type
                } else {
                    writeln!(
                        &mut self.buf,
                        r#"    #[serde(skip_serializing_if = "Option::is_none")]"#
                    )?;
                    format!("Option<{ref_type}>")
                };
                writeln!(
                    &mut self.buf,
                    "    pub {}: {field_type},",
                    name.to_snake_case()
                )?;
            }
            LexObjectProperty::Union(union) => {
                if let Some(description) = &union.description {
                    writeln!(&mut self.buf, "    /// {}", description)?;
                }
                // Use `Box` to avoid recursive.
                let ref_type = format!("Box<{object_name}{}Enum>", name.to_pascal_case());
                let field_type = if required {
                    ref_type
                } else {
                    writeln!(
                        &mut self.buf,
                        r#"    #[serde(skip_serializing_if = "Option::is_none")]"#
                    )?;
                    format!("Option<{ref_type}>")
                };
                writeln!(
                    &mut self.buf,
                    "    pub {}: {field_type},",
                    name.to_snake_case()
                )?;
            }
            LexObjectProperty::Bytes(bytes) => {
                if let Some(description) = &bytes.description {
                    writeln!(&mut self.buf, "    /// {}", description)?;
                }
                // TODO
                writeln!(&mut self.buf, "    // pub {}: ...,", name.to_snake_case())?;
            }
            LexObjectProperty::CidLink(cid_link) => {
                if let Some(description) = &cid_link.description {
                    writeln!(&mut self.buf, "    /// {}", description)?;
                }
                // TODO
                writeln!(&mut self.buf, "    // pub {}: ...,", name.to_snake_case())?;
            }
            LexObjectProperty::Array(array) => {
                if let Some(description) = &array.description {
                    writeln!(&mut self.buf, "    /// {}", description)?;
                }
                let item_type = match &array.items {
                    LexArrayItem::Boolean(_) => String::from("bool"),
                    LexArrayItem::Integer(_) => String::from("i32"),
                    LexArrayItem::String(_) => String::from("String"),
                    LexArrayItem::Unknown(_) => String::from(""), // TODO
                    LexArrayItem::Bytes(_) => String::from(""),   // TODO
                    LexArrayItem::CidLink(_) => String::from(""), // TODO
                    LexArrayItem::Blob(_) => String::from(""),    // TODO
                    LexArrayItem::Ref(r#ref) => Self::ref_type(r#ref),
                    LexArrayItem::Union(_) => format!("{object_name}{}Item", name.to_pascal_case()), // TODO
                };
                let field_type = if required {
                    format!("Vec<{}>", item_type)
                } else {
                    format!("Option<Vec<{}>>", item_type)
                };
                if item_type.is_empty() {
                    writeln!(
                        &mut self.buf,
                        "    // pub {}: Vec<...>",
                        name.to_snake_case()
                    )?;
                } else {
                    if !required {
                        writeln!(
                            &mut self.buf,
                            r#"    #[serde(skip_serializing_if = "Option::is_none")]"#
                        )?;
                    }
                    writeln!(
                        &mut self.buf,
                        "    pub {}: {field_type},",
                        name.to_snake_case()
                    )?;
                }
            }
            LexObjectProperty::Blob(blob) => {
                if let Some(description) = &blob.description {
                    writeln!(&mut self.buf, "    /// {}", description)?;
                }
                // TODO
                writeln!(&mut self.buf, "    // pub {}: ...,", name.to_snake_case())?;
            }
            LexObjectProperty::Boolean(boolean) => {
                if let Some(description) = &boolean.description {
                    writeln!(&mut self.buf, "    /// {}", description)?;
                }
                let field_type = if required {
                    "bool"
                } else {
                    writeln!(
                        &mut self.buf,
                        r#"    #[serde(skip_serializing_if = "Option::is_none")]"#
                    )?;
                    "Option<bool>"
                };
                writeln!(
                    &mut self.buf,
                    "    pub {}: {field_type},",
                    name.to_snake_case()
                )?;
            }
            LexObjectProperty::Integer(integer) => {
                if let Some(description) = &integer.description {
                    writeln!(&mut self.buf, "    /// {}", description)?;
                }
                // TODO: usize?
                let field_type = if required {
                    "i32"
                } else {
                    writeln!(
                        &mut self.buf,
                        r#"    #[serde(skip_serializing_if = "Option::is_none")]"#
                    )?;
                    "Option<i32>"
                };
                writeln!(
                    &mut self.buf,
                    "    pub {}: {field_type},",
                    name.to_snake_case()
                )?;
            }
            LexObjectProperty::String(string) => {
                if let Some(description) = &string.description {
                    writeln!(&mut self.buf, "    /// {}", description)?;
                }
                // TODO: enum?
                let field_type = if required {
                    "String"
                } else {
                    writeln!(
                        &mut self.buf,
                        r#"    #[serde(skip_serializing_if = "Option::is_none")]"#
                    )?;
                    "Option<String>"
                };
                writeln!(
                    &mut self.buf,
                    "    pub {}: {field_type},",
                    // TODO: other keywords?
                    if name == "type" {
                        String::from("r#type")
                    } else {
                        name.to_snake_case()
                    }
                )?;
            }
            // = Record enum
            LexObjectProperty::Unknown(unknown) => {
                if let Some(description) = &unknown.description {
                    writeln!(&mut self.buf, "    /// {}", description)?;
                }
                writeln!(
                    &mut self.buf,
                    "    pub {}: crate::records::Record,",
                    name.to_snake_case()
                )?;
            }
        }
        Ok(())
    }
    fn write_string(&mut self, name: &str, string: &atrium_lex::lexicon::LexString) -> Result<()> {
        if let Some(description) = &string.description {
            writeln!(&mut self.buf, "/// {}", description)?;
        }
        // TODO: enum?
        writeln!(
            &mut self.buf,
            "#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]"
        )?;
        writeln!(&mut self.buf, "pub struct {};", name.to_pascal_case())?;
        Ok(())
    }

    fn ref_type(r#ref: &LexRef) -> String {
        let (namespace, def) = r#ref
            .r#ref
            .split_once('#')
            .unwrap_or((&r#ref.r#ref, "main"));
        if namespace.is_empty() {
            def.to_pascal_case()
        } else {
            format!(
                "crate::{}::{}",
                namespace.split('.').map(str::to_snake_case).join("::"),
                def.to_pascal_case()
            )
        }
    }
}
