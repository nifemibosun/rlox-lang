use std::fs::File;
use std::env::args;
use std::process;
use std::io::{BufWriter, Write, Result};

pub struct Boy;

pub struct GenerateAst;

impl GenerateAst {
    fn define_ast(&self, output_dir: &str, base_name: &str, types: Vec<&str>) -> Result<()> {
        let path: String = String::from(output_dir + "/" + base_name + ".rs");
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        writeln!(writer, "enum {} {", base_name)?;

        for type_entry in types {
            let class_name = type_entry.split(":")[0].trim();
            let fields = type_entry.split(":")[1].trim(); 
            self.define_type(writer, base_name, class_name, fields);
        }

        writeln!(writer, "}")?;

        writer.flush()?;

        Ok(())
    }

    fn define_type(mut writer: BufWriter, base_name: &str, class_name: &str, field_list: &str) {
        writeln!(writer, "struct {} extends {" class_name);
        
    }
}


fn main() -> Result<()> {
    let args: Vec<String> = args().collect();

    if args.len() > 2 {
        eprintln!("Usage: generate_ast <output directory>");
        process::exit(64);
    }

    let output_dir = args[0];

    GenerateAst::define_ast(output_dir, "Expr", vec![
        "Binary   : Expr left, Token operator, Expr right",
        "Grouping : Expr expression",
        "Literal  : Object value",
        "Unary    : Token operator, Expr right"
    ]);

    Ok(())
}
