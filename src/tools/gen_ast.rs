use std::fs::File;
use std::path::Path;
use std::io::{BufWriter, Write, Result};

fn define_ast(output_dir: &str, base_name: &str, types: Vec<&str>) -> Result<()> {
    let file_name = base_name.to_lowercase();
    let file_namestr: &str = &file_name;
    let path: String = String::from(output_dir.to_owned() + "/" + file_namestr + ".rs");
    let file = File::create(Path::new(&path))?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "use crate::tokens::Token;\n")?;

    define_visitor(&mut writer, base_name, types.clone())?;

    writeln!(writer, "pub enum {} {{", base_name)?;

    for type_entry in types {
        if let Some((struct_name, fields)) = type_entry.split_once(':') {
            let struct_name = struct_name.trim();
            let fields = fields.trim();

            writeln!(writer, "    {} {{", struct_name)?;
            define_type(&mut writer, fields)?;
            writeln!(writer, "    }},")?;
        } else {
            eprintln!("Invalid type entry: {}", type_entry);
        }
    }

    writeln!(writer, "}}\n")?;
    writer.flush()?;

    Ok(())
}

fn define_type<W: Write>(writer: &mut BufWriter<W>, field_list: &str) -> Result<()> {
    for field in field_list.split(',') {
        if let Some((field_name, field_type)) = field.trim().split_once(':') {
            writeln!(writer, "        {}: {},", field_name.trim(), field_type.trim())?;
        } else {
            eprintln!("Invalid field: {}", field);
        }
    }

    Ok(())
}

fn define_visitor<W: Write>(writer: &mut W, base_name: &str, types: Vec<&str>) -> Result<()> {
    writeln!(writer, "pub trait Visitor<R> {{")?;

    for type_entry in types {
        let type_name = type_entry.split(':').next().unwrap().trim();
        writeln!(
            writer,
            "    fn visit_{}_{}(&self, {}: &{}) -> R;",
            type_name.to_lowercase(),
            base_name.to_lowercase(),
            base_name.to_lowercase(),
            type_name
        )?;
    }

    writeln!(writer, "}}\n")?;
    Ok(())
}


fn main() {
    let output_dir = "src";

    let _ = define_ast(&output_dir, "Expr", vec![
        "Binary: left: Box<Expr>, operator: Token , right: Box<Expr>",
        "Grouping: expression: Box<Expr>",
        "Literal: value: Object",
        "Unary: operator: Token, right: Box<Expr>"
    ]);
}
