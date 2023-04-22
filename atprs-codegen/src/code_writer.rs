use atprs_lex::lexicon::LexUserType;
use std::collections::HashMap;
use std::io::{Result, Write};

pub(crate) struct CodeWriter {
    buf: Vec<u8>,
}

impl CodeWriter {
    pub fn new() -> Self {
        let mut buf = Vec::new();
        writeln!(
            &mut buf,
            "// This file is generated by atprs-codegen. Do not edit"
        )
        .ok();
        Self { buf }
    }
    pub fn write_user_type(
        &mut self,
        name: &str,
        _def: &LexUserType,
        _defmap: &HashMap<String, &LexUserType>,
    ) -> Result<()> {
        // TODO
        writeln!(&mut self.buf)?;
        writeln!(&mut self.buf, "// {name}")?;
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
}
