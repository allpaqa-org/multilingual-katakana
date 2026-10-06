use allpaqa_multilingual_katakana::*;
use multilingual_katakana_core::{to_katakana, KatakanaOptions};
use serde::Deserialize;
use std::fs;
use std::path::Path;

/// Call `mk_to_katakana` and return (status, output).
fn call(text: &[u8], opts: Option<&MkOptions>) -> (i32, String) {
    let mut p: *mut u8 = std::ptr::null_mut();
    let mut n: usize = 0;
    let tp = if text.is_empty() {
        std::ptr::null()
    } else {
        text.as_ptr()
    };
    let op = opts.map_or(std::ptr::null(), |o| o as *const MkOptions);
    let code = unsafe { mk_to_katakana(tp, text.len(), op, &mut p, &mut n) };
    if code != MK_OK {
        assert!(p.is_null() && n == 0, "out must be reset on error");
        return (code, String::new());
    }
    assert!(!p.is_null());
    let s = unsafe { String::from_utf8(std::slice::from_raw_parts(p, n).to_vec()).unwrap() };
    unsafe { mk_free_string(p, n) };
    (code, s)
}

#[test]
fn round_trip_and_core_parity() {
    assert_eq!(call(b"hello", None), (MK_OK, "ハロー".to_string()));
    let jp = "了解、初見歓迎、神回";
    assert_eq!(call(jp.as_bytes(), None).1, to_katakana(jp, None));
}

#[test]
fn options_null_equals_empty_set_and_flags_apply() {
    let none = MkOptions::default();
    assert_eq!(call(b"hello", None), call(b"hello", Some(&none)));
    let off = MkOptions {
        flags_set: MK_FLAG_ENABLE_ENGLISH,
        flags_value: 0,
    };
    let expected = to_katakana(
        "hello",
        Some(&KatakanaOptions {
            enable_english: false,
            ..Default::default()
        }),
    );
    assert_eq!(call(b"hello", Some(&off)).1, expected);
}

#[test]
fn empty_input_and_free() {
    let mut p: *mut u8 = std::ptr::null_mut();
    let mut n: usize = 99;
    let code = unsafe { mk_to_katakana(std::ptr::null(), 0, std::ptr::null(), &mut p, &mut n) };
    assert_eq!(code, MK_OK);
    assert!(!p.is_null());
    assert_eq!(n, 0);
    unsafe { mk_free_string(p, n) };
    unsafe { mk_free_string(std::ptr::null_mut(), 0) };
}

#[test]
fn invalid_utf8_and_null_pointers() {
    assert_eq!(call(&[0xff, 0xfe], None).0, MK_ERR_INVALID_UTF8);

    let mut p: *mut u8 = std::ptr::null_mut();
    let mut n: usize = 5;
    let c = unsafe { mk_to_katakana(std::ptr::null(), 3, std::ptr::null(), &mut p, &mut n) };
    assert_eq!((c, p.is_null(), n), (MK_ERR_NULL_POINTER, true, 0));

    let c = unsafe {
        mk_to_katakana(
            b"a".as_ptr(),
            1,
            std::ptr::null(),
            std::ptr::null_mut(),
            &mut n,
        )
    };
    assert_eq!(c, MK_ERR_NULL_POINTER);
    let c = unsafe {
        mk_to_katakana(
            b"a".as_ptr(),
            1,
            std::ptr::null(),
            &mut p,
            std::ptr::null_mut(),
        )
    };
    assert_eq!(c, MK_ERR_NULL_POINTER);
}

#[test]
fn version_and_self_check() {
    assert_eq!(mk_abi_version(), 1);
    assert_eq!(mk_self_check(), 1);
}

#[test]
fn concurrent_calls_match_single_thread() {
    let input = "Hello World, привет мир! 안녕하세요";
    let expected = call(input.as_bytes(), None).1;
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let exp = expected.clone();
            std::thread::spawn(move || {
                for _ in 0..200 {
                    assert_eq!(call(input.as_bytes(), None).1, exp);
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
}

#[derive(Deserialize)]
struct Expected {
    canonical: Option<String>,
    #[serde(default)]
    accepted: Vec<String>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum ExpectedField {
    String(String),
    Object(Expected),
}

#[derive(Deserialize)]
struct Case {
    id: String,
    input: String,
    expected: ExpectedField,
    #[serde(default)]
    options: Option<serde_json::Value>,
}

/// Map a spec case's `options` object (`enable_*`/`normalize_prosody` bools) to flags.
fn flags_from_json(v: &serde_json::Value) -> MkOptions {
    const NAMES: [(&str, u32); 10] = [
        ("enable_cyrillic", MK_FLAG_ENABLE_CYRILLIC),
        ("enable_korean", MK_FLAG_ENABLE_KOREAN),
        ("enable_chinese", MK_FLAG_ENABLE_CHINESE),
        ("enable_spanish", MK_FLAG_ENABLE_SPANISH),
        ("enable_french", MK_FLAG_ENABLE_FRENCH),
        ("enable_vietnamese", MK_FLAG_ENABLE_VIETNAMESE),
        ("enable_thai", MK_FLAG_ENABLE_THAI),
        ("enable_slang", MK_FLAG_ENABLE_SLANG),
        ("enable_english", MK_FLAG_ENABLE_ENGLISH),
        ("normalize_prosody", MK_FLAG_NORMALIZE_PROSODY),
    ];
    let mut o = MkOptions::default();
    for (name, bit) in NAMES {
        if let Some(b) = v.get(name).and_then(|x| x.as_bool()) {
            o.flags_set |= bit;
            if b {
                o.flags_value |= bit;
            }
        }
    }
    o
}

#[test]
fn spec_cases_via_c_abi() {
    let mut files: Vec<_> = fs::read_dir(Path::new("../../spec/cases"))
        .expect("spec/cases")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "json"))
        .collect();
    files.sort();
    assert!(!files.is_empty());

    let (mut run, mut skipped, mut failures) = (0, 0, Vec::new());
    for f in files {
        let cases: Vec<Case> = serde_json::from_str(&fs::read_to_string(&f).unwrap()).unwrap();
        for tc in cases {
            let opts = tc.options.as_ref();
            if opts.is_some_and(|o| o.get("exclude").is_some()) {
                skipped += 1;
                continue;
            }
            run += 1;
            let mk = opts.map(flags_from_json);
            let actual = call(tc.input.as_bytes(), mk.as_ref()).1;
            let (canon, acc) = match &tc.expected {
                ExpectedField::String(s) => (s.clone(), vec![]),
                ExpectedField::Object(e) => {
                    (e.canonical.clone().unwrap_or_default(), e.accepted.clone())
                }
            };
            if actual != canon && !acc.contains(&actual) {
                failures.push(format!("{}: {:?} != {:?}", tc.id, actual, canon));
            }
        }
    }
    println!("ffi spec cases: run={run} skipped={skipped}");
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
