use std::env;
use std::fs::{create_dir_all, File};
use std::io::{self, Write};
use std::path::Path;
use std::process;

fn generate_ast() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: generate_ast <output directory>");
        process::exit(64);
    }

    let output_dir = &args[1];

    let types = vec![
        "Binary   : Expr left, Token operator, Expr right",
        "Grouping : Expr expression",
        "Literal  : Object value",
        "Unary    : Token operator, Expr right",
    ];

    define_ast(output_dir, "Expr", types)?;

    Ok(())
}

fn define_ast(output_dir: &str, base_name: &str, types: Vec<&str>) -> io::Result<()> {
    let file_path = format!("{}/{}.rs", output_dir, base_name.to_lowercase());
    let path = Path::new(output_dir);

    // Ensure the output directory exists
    if !path.exists() {
        create_dir_all(path)?;
    }

    let mut file = File::create(&file_path)?;

    writeln!(file, "// This file is auto-generated. Do not edit manually.")?;
    writeln!(file, "use crate::token::Token;")?;
    writeln!(file, "pub enum {} {{", base_name)?;

    for type_def in types {
        // Split the type definition into name and fields
        let parts: Vec<&str> = type_def.split(':').map(str::trim).collect();
        let variant_name = parts[0];
        let fields = parts[1]
            .split(',')
            .map(str::trim)
            .map(|field| {
                let field_parts: Vec<&str> = field.split_whitespace().collect();
                let field_type = field_parts[0];
                let field_name = field_parts[1];
                format!("        {}: {},", field_name, field_type)
            })
            .collect::<Vec<String>>()
            .join("\n");

        // Write the variant definition
        writeln!(file, "    {} {{", variant_name)?;
        writeln!(file, "{}", fields)?;
        writeln!(file, "    }},")?;
    }

    writeln!(file, "}}")?;

    

    for type_def in types {
        let parts: Vec<&str> = type_def.split(':').map(str::trim).collect();
        let class_name = parts[0];
        let fields = parts[1];
        define_type(&mut writer, base_name, class_name, fields)?;
    }
    

    println!("AST file generated at: {}", file_path);

    Ok(())
}

fn define_type(
    writer: &mut impl Write,
    base_name: &str,
    class_name: &str,
    fields: &str,
) -> std::io::Result<()> {
    writeln!(writer, "    pub struct {} {{", class_name)?;

    for field in fields.split(',').map(str::trim) {
        let parts: Vec<&str> = field.split_whitespace().collect();
        let field_type = parts[0];
        let field_name = parts[1];
        writeln!(writer, "        pub {}: {},", field_name, field_type)?;
    }

    writeln!(writer, "    }}")?;
    Ok(())
}
