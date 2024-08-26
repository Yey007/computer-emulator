extern crate core;
extern crate proc_macro;

use proc_macro::TokenStream;
use std::path::Path;
use std::{fs, vec};

struct Field<'a> {
    symbol: char,
    name: &'a str,
    bits: u8,
}

// It's fine to require that these all have the same lifetime for our use case
struct InstrDef<'a> {
    name: &'a str,
    pattern: &'a str,
    fields: Vec<Field<'a>>,
}

#[proc_macro]
pub fn make_instructions(_item: TokenStream) -> TokenStream {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let defs = fs::read_to_string(manifest_dir.join("instructions.txt"))
        .expect("instruction definitions missing");
    let parsed = parse_defs(defs.as_str());
    let enum_def = gen_enum(&parsed);
    enum_def
}

fn gen_enum(parsed: &Vec<InstrDef>) -> TokenStream {
    let mut enum_str = "#[derive(PartialEq, Debug)] pub enum Instruction {".to_string();
    for def in parsed {
        enum_str.push_str(def.name);

        if def.fields.is_empty() {
            enum_str.push_str(",");
            continue;
        }

        enum_str.push_str(" {");
        for field in &def.fields {
            enum_str.push_str(field.name);
            enum_str.push_str(": U<");
            enum_str.push_str(field.bits.to_string().as_str());
            enum_str.push_str(">,");
        }
        enum_str.push_str("},");
    }
    enum_str.push_str("}");
    enum_str.parse().unwrap()
}

fn parse_defs(defs: &str) -> Vec<InstrDef> {
    let lines = defs.lines();
    lines.map(parse_line).collect()
}

fn parse_line(line: &str) -> InstrDef {
    let parts: Vec<&str> = line.split_whitespace().collect();
    match parts.as_slice() {
        [name, pattern, fields @ ..] => InstrDef {
            name,
            pattern,
            fields: extract_fields(pattern, fields),
        },
        _ => panic!("invalid instruction definition"),
    }
}

fn extract_fields<'a, 'b>(pattern: &'a str, field_names: &'b [&'a str]) -> Vec<Field<'a>>
where
    'a: 'b,
{
    let mut alphas = pattern.chars().filter(|c| c.is_alphabetic());
    let mut name_iter = field_names.iter();

    let Some(mut last) = alphas.next() else {
        return vec![];
    };
    let mut current_count = 1;

    // TODO: can we not allocate here?
    let mut result = vec![];
    for char in alphas {
        if char != last {
            let name = name_iter.next().expect("not enough field names");
            result.push(Field {
                symbol: last,
                name,
                bits: current_count,
            });
            current_count = 0;
        }

        last = char;
        current_count += 1;
    }

    result.push(Field {
        symbol: last,
        name: name_iter.next().expect("not enough field names"),
        bits: current_count,
    });

    result
}