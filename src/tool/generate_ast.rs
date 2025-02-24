use std::env::args;
use std::process;
use std::fs::File;
use std::path::Path;
use std::io::{ Write, Result };



fn define_ast(output_dir: &str, base_name: &str, types: Vec<&str>) -> Result<()> {
    let path = String::from(output_dir + "/" + base_name.to_lowercase() + ".rs");
    let mut file = File::create(Path::new(path))?;

    writeln!(file, "use crate::tokens::Token")?;
    writeln!(file, "pub enum {} {{", base_name)?;

    for type_entry in types {
        let class_name = type_entry.split(":").trim();
        let field = type_entry.split(":")[1].trim();

        define_type(&mut file, base_name, class_name, field);
    }

    writeln!(writer, "}");
    writer.flush()?;

    Ok(())
}

fn define_type(&mut writer: BufWriter, base_name: &str, class_name: &str, field_list: &str) {
    let parts: Vec<&str> = type_def.split(":").collect();
    let class_name = parts[0].trim();
    let fields = parts[1].trim();

    writeln!(file, "#[derive(Debug, Clone)]")?;
    writeln!(file, "pub struct {} {{", class_name)?;

    for field in fields.split(", ") {
        let field_parts: Vec<&str> = field.split_whitespace().collect();
        let field_type = field_parts[0];
        let field_name = field_parts[1];
        writeln!(file, "    pub {}: {},", field_name, field_type)?;
    }

    writeln!(file, "}}\n")?;

    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = args().collect();

    if args.len() != 2 {
        println!("Usage generate_ast <output directory>");
        process::exit(64);
    }

    let output_dir = &args[1];
    GenerateAst::define_ast(output_dir, "Expr", vec![
        "Binary   : Expr left, Token operator, Expr right",
        "Grouping : Expr expression",
        "Literal  : Object value",
        "Unary    : Token operator, Expr right"
    ]);

    Ok(())
}
























// use std::fs::File;
// use std::env::args;
// use std::process;
// use std::io::{BufWriter, Write, Result};

// pub struct Boy;

// pub struct GenerateAst;

// impl GenerateAst {
//     fn define_ast(&self, output_dir: &str, base_name: &str, types: Vec<&str>) -> Result<()> {
//         let path: String = String::from(output_dir + "/" + base_name + ".rs");
//         let file = File::create(path)?;
//         let mut writer = BufWriter::new(file);

//         writeln!(writer, "enum {} {", base_name)?;

//         for type_entry in types {
//             let class_name = type_entry.split(":")[0].trim();
//             let fields = type_entry.split(":")[1].trim(); 
//             self.define_type(writer, base_name, class_name, fields);
//         }

//         writeln!(writer, "}")?;

//         writer.flush()?;

//         Ok(())
//     }

//     fn define_type(mut writer: BufWriter, base_name: &str, class_name: &str, field_list: &str) {
//         writeln!(writer, "struct {} extends {" class_name);
        
//     }
// }


// fn main() -> Result<()> {
//     let args: Vec<String> = args().collect();

//     if args.len() > 2 {
//         eprintln!("Usage: generate_ast <output directory>");
//         process::exit(64);
//     }

//     let output_dir = args[0];

//     GenerateAst::define_ast(output_dir, "Expr", vec![
//         "Binary   : Expr left, Token operator, Expr right",
//         "Grouping : Expr expression",
//         "Literal  : Object value",
//         "Unary    : Token operator, Expr right"
//     ]);

//     Ok(())
// }
