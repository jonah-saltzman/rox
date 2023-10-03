use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};

mod lib_content {
    include!("src/keyword.rs");
}

use lib_content::Keyword;
use strum::VariantNames;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("keyword_map.rs");
    let mut file = BufWriter::new(File::create(&dest_path).unwrap());

    let mut map_code =
        String::from("static KEYWORD_MAP: phf::Map<&'static str, Keyword> = phf_map! {\n");

    for &variant_str in Keyword::VARIANTS {
        let mut variant = variant_str
            .chars()
            .next()
            .unwrap()
            .to_uppercase()
            .to_string();
        variant.extend(variant_str.chars().skip(1));
        map_code.push_str(format!("    \"{}\" => Keyword::{},\n", variant_str, variant).as_str());
    }
    map_code.push_str("};");

    write!(&mut file, "{}", map_code).unwrap();
}
