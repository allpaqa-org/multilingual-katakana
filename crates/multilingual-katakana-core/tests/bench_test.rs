use multilingual_katakana_core::{to_katakana, KatakanaOptions};
use regex::Regex;
use std::time::Instant;

struct BenchmarkScenario {
    name: &'static str,
    input: String,
    options: Option<KatakanaOptions>,
}

#[test]
fn test_scenario_based_benchmark() {
    let url_re = Regex::new(r"https?://\S+").unwrap();
    let mention_re = Regex::new(r"@\w+").unwrap();
    let exclude_opts = KatakanaOptions::new()
        .with_exclude(url_re)
        .with_exclude(mention_re);

    let scenarios = [
        BenchmarkScenario {
            name: "Short Chat (~10 chars)",
            input: "gg wp bro".to_string(),
            options: None,
        },
        BenchmarkScenario {
            name: "Typical Stream Comment (~50 chars)",
            input: "初見です！Hello streamer! 今日の配信も楽しみにしてました！".to_string(),
            options: None,
        },
        BenchmarkScenario {
            name: "Mixed Multilingual (~100 chars)",
            input: "Hello! 你好! 안녕하세요! muchas gracias for the stream bro pog! Привет!"
                .to_string(),
            options: None,
        },
        BenchmarkScenario {
            name: "URL & Mention Protected (~80 chars)",
            input: "check https://twitch.tv/example @streamer nice play gg!".to_string(),
            options: Some(exclude_opts),
        },
        BenchmarkScenario {
            name: "Extreme Long Comment (~1,000 chars)",
            input: "Hello streamer! 안녕하세요 你好 muchas gracias Привет gg wp! ".repeat(15),
            options: None,
        },
    ];

    println!("\n=========================================================================================");
    println!(
        "             multilingual-katakana Scenario-Based Performance Benchmark                  "
    );
    println!(
        "========================================================================================="
    );
    println!("| Scenario                           | Chars | Latency (µs) | Latency (ms) | Throughput (ops/s) |");
    println!("|------------------------------------|-------|--------------|--------------|--------------------|");

    let iterations = 5000;

    for s in &scenarios {
        let opts_ref = s.options.as_ref();

        // Warm-up
        for _ in 0..100 {
            let _ = to_katakana(&s.input, opts_ref);
        }

        let start = Instant::now();
        for _ in 0..iterations {
            let _ = to_katakana(&s.input, opts_ref);
        }
        let elapsed = start.elapsed();

        let avg_us = elapsed.as_micros() as f64 / iterations as f64;
        let avg_ms = avg_us / 1000.0;
        let ops_per_sec = iterations as f64 / elapsed.as_secs_f64();

        println!(
            "| {:<34} | {:>5} | {:>12.2} | {:>12.4} | {:>18.0} |",
            s.name,
            s.input.chars().count(),
            avg_us,
            avg_ms,
            ops_per_sec
        );

        #[cfg(not(debug_assertions))]
        {
            // All short/typical scenarios must be well under 50µs (0.05ms) in release mode
            if s.input.chars().count() <= 100 {
                assert!(
                    avg_us < 50.0,
                    "Latency for {} must be <50µs, got {:.2}µs",
                    s.name,
                    avg_us
                );
            }
        }
    }
    println!("=========================================================================================\n");
}
