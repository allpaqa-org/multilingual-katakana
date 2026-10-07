use multilingual_katakana_core::to_katakana;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::time::Instant;

#[derive(Deserialize, Debug)]
struct Expected {
    canonical: Option<String>,
    #[serde(default)]
    accepted: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum ExpectedField {
    String(String),
    Object(Expected),
}

#[derive(Deserialize, Debug)]
struct TestCase {
    id: String,
    language: String,
    input: String,
    expected: ExpectedField,
    description: String,
}

#[test]
fn test_all_spec_cases_100_percent() {
    let cases_dir = Path::new("../../spec/cases");
    let mut entries: Vec<_> = fs::read_dir(cases_dir)
        .expect("Failed to read spec/cases dir")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "json"))
        .collect();
    entries.sort_by_key(|e| e.path());

    assert!(!entries.is_empty(), "No spec files found!");

    let mut total_cases = 0;
    let mut passed_cases = 0;
    let mut failure_messages = Vec::new();

    let start_time = Instant::now();

    for entry in entries {
        let file_path = entry.path();
        let file_name = file_path.file_name().unwrap().to_str().unwrap();
        let content = fs::read_to_string(&file_path)
            .unwrap_or_else(|_| panic!("Failed to read {}", file_name));
        let cases: Vec<TestCase> = serde_json::from_str(&content)
            .unwrap_or_else(|e| panic!("Failed to parse {}: {}", file_name, e));

        for tc in cases {
            total_cases += 1;
            let actual = to_katakana(&tc.input, None);

            let (canonical, accepted) = match &tc.expected {
                ExpectedField::String(s) => (s.clone(), Vec::new()),
                ExpectedField::Object(exp) => (
                    exp.canonical.clone().unwrap_or_default(),
                    exp.accepted.clone(),
                ),
            };

            let is_match = actual == canonical || accepted.contains(&actual);
            if is_match {
                passed_cases += 1;
            } else {
                failure_messages.push(format!(
                    "[{}] in {} (lang: '{}'):\n  Input:    {:?}\n  Expected: {:?} (or accepted: {:?})\n  Actual:   {:?}\n  Desc:     {}",
                    tc.id, file_name, tc.language, tc.input, canonical, accepted, actual, tc.description
                ));
            }
        }
    }

    let elapsed = start_time.elapsed();
    let avg_us = if total_cases > 0 {
        elapsed.as_micros() as f64 / total_cases as f64
    } else {
        0.0
    };

    println!("\n=== Spec Test Summary ===");
    println!("Total cases:   {}", total_cases);
    println!("Passed:        {}", passed_cases);
    println!("Failed:        {}", total_cases - passed_cases);
    println!("Total time:    {:.2?}", elapsed);
    println!(
        "Avg per case:  {:.2} µs ({:.4} ms)",
        avg_us,
        avg_us / 1000.0
    );

    if !failure_messages.is_empty() {
        panic!(
            "\nSpec conformance failed ({} / {} passed):\n\n{}",
            passed_cases,
            total_cases,
            failure_messages.join("\n\n")
        );
    }

    assert_eq!(total_cases, 228, "Expected exactly 228 spec test cases");
    assert_eq!(passed_cases, 228, "All 228 spec test cases must pass 100%");
}
