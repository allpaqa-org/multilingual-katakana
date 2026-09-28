use multilingual_katakana_core::to_katakana;
use std::time::Instant;

#[test]
fn test_performance_benchmark() {
    let samples = [
        "hello world this is a test phrase",
        "Привет мир как дела",
        "안녕하세요 오늘 방송 재미있어요",
        "你好 謝謝 大家 加油",
        "muchas gracias amigo buenos dias",
        "gg wp that was a nice play ez",
        "初見歓迎です！神回了解！",
    ];

    // Warm-up
    for _ in 0..100 {
        for s in &samples {
            let _ = to_katakana(s, None);
        }
    }

    let iterations = 2000;
    let total_operations = iterations * samples.len();
    let start = Instant::now();

    for _ in 0..iterations {
        for s in &samples {
            let _ = to_katakana(s, None);
        }
    }

    let elapsed = start.elapsed();
    let avg_us = elapsed.as_micros() as f64 / total_operations as f64;
    let ops_per_sec = total_operations as f64 / elapsed.as_secs_f64();

    println!("\n=== High-Performance Benchmark ===");
    println!("Total operations: {}", total_operations);
    println!("Total time:       {:.2?}", elapsed);
    println!(
        "Average latency:  {:.2} µs ({:.4} ms) per phrase",
        avg_us,
        avg_us / 1000.0
    );
    println!("Throughput:       {:.0} phrases/sec", ops_per_sec);

    // Goal is <0.05ms (50µs) in release mode
    #[cfg(not(debug_assertions))]
    {
        assert!(
            avg_us < 50.0,
            "Average latency must be <0.05ms (50µs), got {:.2}µs",
            avg_us
        );
    }
}
