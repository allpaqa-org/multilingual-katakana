use serde::Deserialize;
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
struct CyrillicDict {
    phrases: BTreeMap<String, String>,
    letters: BTreeMap<String, String>,
}

#[derive(Deserialize)]
struct SpanishDict {
    phrases: BTreeMap<String, String>,
}

#[derive(Deserialize)]
struct SlangDict {
    phrases: BTreeMap<String, String>,
    slang: BTreeMap<String, String>,
}

#[derive(Deserialize)]
struct KoreanDict {
    phrases: BTreeMap<String, String>,
    chosung_jungsung_map: Vec<Vec<String>>,
    jongsung_map: Vec<String>,
}

#[derive(Deserialize)]
struct ChineseData {
    marker_pattern: String,
    common_words: Vec<String>,
    taiwan_phrases: BTreeMap<String, String>,
    hanzi_to_katakana: BTreeMap<String, String>,
}

fn escape_str(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn main() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let crate_dicts_dir = manifest_dir.join("dicts");

    let (dicts_dir, node_dicts_dir) = if crate_dicts_dir.exists() {
        println!("cargo:rerun-if-changed=dicts");
        (crate_dicts_dir.clone(), crate_dicts_dir)
    } else {
        println!("cargo:rerun-if-changed=../../dicts");
        println!("cargo:rerun-if-changed=../../bindings/node/src/dicts");
        let root = Path::new("../../");
        (root.join("dicts"), root.join("bindings/node/src/dicts"))
    };

    // 1. Cyrillic
    let cyrillic_json =
        fs::read_to_string(dicts_dir.join("cyrillic.json")).expect("Failed to read cyrillic.json");
    let cyrillic: CyrillicDict =
        serde_json::from_str(&cyrillic_json).expect("Failed to parse cyrillic.json");

    // Sort phrases by length descending for greedy replacement
    let mut cyrillic_phrases: Vec<(String, String)> = cyrillic.phrases.into_iter().collect();
    cyrillic_phrases.sort_by(|a, b| b.0.len().cmp(&a.0.len()).then_with(|| a.0.cmp(&b.0)));

    // Letters sorted by key for binary search
    let mut cyrillic_letters: Vec<(String, String)> = cyrillic.letters.into_iter().collect();
    cyrillic_letters.sort_by(|a, b| a.0.cmp(&b.0));

    // 2. Spanish
    let spanish_json =
        fs::read_to_string(dicts_dir.join("spanish.json")).expect("Failed to read spanish.json");
    let spanish: SpanishDict =
        serde_json::from_str(&spanish_json).expect("Failed to parse spanish.json");

    let mut spanish_phrases = Vec::new();
    let mut spanish_words = Vec::new();
    for (k, v) in spanish.phrases {
        if k.contains(' ') {
            spanish_phrases.push((k.to_lowercase(), v));
        } else {
            spanish_words.push((k.to_lowercase(), v));
        }
    }
    spanish_phrases.sort_by(|a, b| b.0.len().cmp(&a.0.len()).then_with(|| a.0.cmp(&b.0)));
    spanish_words.sort_by(|a, b| a.0.cmp(&b.0));

    // 3. Slang
    let slang_json =
        fs::read_to_string(dicts_dir.join("slang.json")).expect("Failed to read slang.json");
    let slang: SlangDict = serde_json::from_str(&slang_json).expect("Failed to parse slang.json");

    let mut slang_phrases: Vec<(String, String)> = slang
        .phrases
        .into_iter()
        .map(|(k, v)| (k.to_lowercase(), v))
        .collect();
    slang_phrases.sort_by(|a, b| b.0.len().cmp(&a.0.len()).then_with(|| a.0.cmp(&b.0)));

    let mut slang_words: Vec<(String, String)> = slang
        .slang
        .into_iter()
        .map(|(k, v)| (k.to_lowercase(), v))
        .collect();
    slang_words.sort_by(|a, b| a.0.cmp(&b.0));

    // 4. Korean
    let korean_json =
        fs::read_to_string(dicts_dir.join("korean.json")).expect("Failed to read korean.json");
    let korean: KoreanDict =
        serde_json::from_str(&korean_json).expect("Failed to parse korean.json");

    let mut korean_phrases: Vec<(String, String)> = korean.phrases.into_iter().collect();
    korean_phrases.sort_by(|a, b| b.0.len().cmp(&a.0.len()).then_with(|| a.0.cmp(&b.0)));

    // 5. Chinese
    let chinese_json = fs::read_to_string(node_dicts_dir.join("chinese_data.json"))
        .expect("Failed to read chinese_data.json");
    let chinese: ChineseData =
        serde_json::from_str(&chinese_json).expect("Failed to parse chinese_data.json");

    let mut taiwan_phrases: Vec<(String, String)> = chinese.taiwan_phrases.into_iter().collect();
    taiwan_phrases.sort_by(|a, b| b.0.len().cmp(&a.0.len()).then_with(|| a.0.cmp(&b.0)));

    let mut hanzi_map: Vec<(char, String)> = Vec::new();
    for (k, v) in chinese.hanzi_to_katakana {
        if let Some(c) = k.chars().next() {
            hanzi_map.push((c, v));
        }
    }
    hanzi_map.sort_by_key(|a| a.0);

    // 6. English Words
    let english_json = fs::read_to_string(node_dicts_dir.join("english_words.json"))
        .expect("Failed to read english_words.json");
    let english_raw: BTreeMap<String, String> =
        serde_json::from_str(&english_json).expect("Failed to parse english_words.json");

    let mut english_words: Vec<(String, String)> = english_raw
        .into_iter()
        .map(|(k, v)| (k.to_lowercase(), v))
        .collect();
    english_words.sort_by(|a, b| a.0.cmp(&b.0));

    // Generate Rust source code
    let mut code = String::new();
    code.push_str("// Auto-generated by build.rs. DO NOT EDIT DIRECTLY.\n\n");

    // Cyrillic phrases
    code.push_str("pub static CYRILLIC_PHRASES: &[(&str, &str)] = &[\n");
    for (k, v) in &cyrillic_phrases {
        code.push_str(&format!(
            "    (\"{}\", \"{}\"),\n",
            escape_str(k),
            escape_str(v)
        ));
    }
    code.push_str("];\n\n");

    // Cyrillic letters
    code.push_str("pub static CYRILLIC_LETTERS: &[(&str, &str)] = &[\n");
    for (k, v) in &cyrillic_letters {
        code.push_str(&format!(
            "    (\"{}\", \"{}\"),\n",
            escape_str(k),
            escape_str(v)
        ));
    }
    code.push_str("];\n\n");

    // Spanish phrases
    code.push_str("pub static SPANISH_PHRASES: &[(&str, &str)] = &[\n");
    for (k, v) in &spanish_phrases {
        code.push_str(&format!(
            "    (\"{}\", \"{}\"),\n",
            escape_str(k),
            escape_str(v)
        ));
    }
    code.push_str("];\n\n");

    // Spanish words
    code.push_str("pub static SPANISH_WORDS: &[(&str, &str)] = &[\n");
    for (k, v) in &spanish_words {
        code.push_str(&format!(
            "    (\"{}\", \"{}\"),\n",
            escape_str(k),
            escape_str(v)
        ));
    }
    code.push_str("];\n\n");

    // Slang phrases
    code.push_str("pub static SLANG_PHRASES: &[(&str, &str)] = &[\n");
    for (k, v) in &slang_phrases {
        code.push_str(&format!(
            "    (\"{}\", \"{}\"),\n",
            escape_str(k),
            escape_str(v)
        ));
    }
    code.push_str("];\n\n");

    // Slang words
    code.push_str("pub static SLANG_WORDS: &[(&str, &str)] = &[\n");
    for (k, v) in &slang_words {
        code.push_str(&format!(
            "    (\"{}\", \"{}\"),\n",
            escape_str(k),
            escape_str(v)
        ));
    }
    code.push_str("];\n\n");

    // Korean phrases
    code.push_str("pub static KOREAN_PHRASES: &[(&str, &str)] = &[\n");
    for (k, v) in &korean_phrases {
        code.push_str(&format!(
            "    (\"{}\", \"{}\"),\n",
            escape_str(k),
            escape_str(v)
        ));
    }
    code.push_str("];\n\n");

    // Korean chosung_jungsung_map [19][21]
    code.push_str("pub static KOREAN_CHOSUNG_JUNGSUNG: &[[&str; 21]; 19] = &[\n");
    for row in &korean.chosung_jungsung_map {
        code.push_str("    [");
        for (i, cell) in row.iter().enumerate() {
            if i > 0 {
                code.push_str(", ");
            }
            code.push_str(&format!("\"{}\"", escape_str(cell)));
        }
        code.push_str("],\n");
    }
    code.push_str("];\n\n");

    // Korean jongsung_map [28]
    code.push_str("pub static KOREAN_JONGSUNG: &[&str; 28] = &[\n    ");
    for (i, cell) in korean.jongsung_map.iter().enumerate() {
        if i > 0 {
            code.push_str(", ");
        }
        code.push_str(&format!("\"{}\"", escape_str(cell)));
    }
    code.push_str("\n];\n\n");

    // Chinese marker pattern
    code.push_str(&format!(
        "pub static CHINESE_MARKER_PATTERN: &str = \"{}\";\n\n",
        escape_str(&chinese.marker_pattern)
    ));

    // Chinese common words
    code.push_str("pub static CHINESE_COMMON_WORDS: &[&str] = &[\n");
    for word in &chinese.common_words {
        code.push_str(&format!("    \"{}\",\n", escape_str(word)));
    }
    code.push_str("];\n\n");

    // Taiwan phrases
    code.push_str("pub static CHINESE_TAIWAN_PHRASES: &[(&str, &str)] = &[\n");
    for (k, v) in &taiwan_phrases {
        code.push_str(&format!(
            "    (\"{}\", \"{}\"),\n",
            escape_str(k),
            escape_str(v)
        ));
    }
    code.push_str("];\n\n");

    // Chinese hanzi map
    code.push_str("pub static CHINESE_HANZI_MAP: &[(char, &str)] = &[\n");
    for (c, v) in &hanzi_map {
        code.push_str(&format!("    ('{}', \"{}\"),\n", c, escape_str(v)));
    }
    code.push_str("];\n\n");

    // English words
    code.push_str("pub static ENGLISH_WORDS: &[(&str, &str)] = &[\n");
    for (k, v) in &english_words {
        code.push_str(&format!(
            "    (\"{}\", \"{}\"),\n",
            escape_str(k),
            escape_str(v)
        ));
    }
    code.push_str("];\n");

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let dest_path = Path::new(&out_dir).join("generated_dicts.rs");
    fs::write(&dest_path, code).expect("Failed to write generated_dicts.rs");
}
