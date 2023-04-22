mod code_writer;
mod fs;

use atprs_lex::lexicon::LexUserType;
use atprs_lex::LexiconDoc;
use code_writer::CodeWriter;
use heck::{ToPascalCase, ToSnakeCase};
use std::collections::HashMap;
use std::fs::{create_dir_all, read_dir, read_to_string, File};
use std::io::Result;
use std::path::{Path, PathBuf};

pub fn genapi(lexdir: impl AsRef<Path>, outdir: impl AsRef<Path>, prefix: &str) -> Result<()> {
    let lexdir = lexdir.as_ref().canonicalize()?;
    let outdir = outdir.as_ref().canonicalize()?;
    let paths = fs::find_schemas(&lexdir)?;
    let mut schemas = Vec::with_capacity(paths.len());
    for path in &paths {
        schemas.push((read_to_string(path)?).parse::<LexiconDoc>()?);
    }
    let defmap = build_defmap(&schemas);
    for schema in schemas
        .iter()
        .filter(|schema| schema.id.starts_with(prefix))
    {
        generate_code(schema, &outdir, &defmap)?;
    }
    generate_modules(&outdir)?;
    Ok(())
}

fn build_defmap(schemas: &[LexiconDoc]) -> HashMap<String, &LexUserType> {
    let mut result = HashMap::new();
    for schema in schemas {
        for (name, def) in &schema.defs {
            let key = if name == "main" {
                schema.id.clone()
            } else {
                format!("{}#{}", schema.id, name)
            };
            assert!(!result.contains_key(&key), "duplicate key: {key}");
            result.insert(key, def);
        }
    }
    result
}

fn generate_code(
    schema: &LexiconDoc,
    outdir: &Path,
    defmap: &HashMap<String, &LexUserType>,
) -> Result<()> {
    let mut paths = schema.id.split('.').collect::<Vec<_>>();
    if let Some(name) = paths.pop() {
        create_dir_all(outdir.join(paths.join("/")))?;
        let mut writer = CodeWriter::new();
        // TODO
        let mut keys = Vec::new();
        for (key, def) in &schema.defs {
            if key == "main" {
                writer.write_user_type(&name.to_pascal_case(), def, defmap)?;
            } else {
                keys.push(key);
            }
        }
        keys.sort();
        for key in keys {
            let def = &schema.defs[key];
            assert!(!matches!(
                def,
                LexUserType::Record(_)
                    | LexUserType::XrpcProcedure(_)
                    | LexUserType::XrpcQuery(_)
                    | LexUserType::XrpcSubscription(_)
            ));
            writer.write_user_type(&key.to_pascal_case(), def, defmap)?;
        }
        let mut filename = PathBuf::from(name.to_snake_case());
        filename.set_extension("rs");
        writer.write_to_file(&mut File::create(
            outdir.join(paths.join("/")).join(filename),
        )?)?;
    }
    Ok(())
}

fn generate_modules(outdir: &Path) -> Result<()> {
    let paths = fs::find_dirs(outdir)?;
    let mut files = Vec::with_capacity(paths.len());
    // create ".rs" files
    for path in &paths {
        let mut p = path.to_path_buf();
        if path == outdir {
            p = p.join("lib.rs");
        } else {
            p.set_extension("rs");
        }
        files.push(File::create(&p)?);
    }
    // write "mod" statements
    for (path, mut file) in paths.iter().zip(&files) {
        let mut modules = read_dir(path)?
            .filter_map(Result::ok)
            .filter(|entry| entry.path().is_file())
            .filter_map(|entry| {
                entry
                    .path()
                    .file_stem()
                    .map(|s| s.to_string_lossy().into_owned())
            })
            .collect::<Vec<_>>();
        modules.sort();

        let mut writer = CodeWriter::new();
        writer.write_mods(&modules)?;
        writer.write_to_file(&mut file)?;
    }
    Ok(())
}
