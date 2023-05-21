#[macro_export]
macro_rules! measure_time {
    ($comment:expr => $stmt:expr) => {{
        let start = std::time::Instant::now();
        let result = { $stmt };
        let duration = start.elapsed();
        println!("Execution time for {}: {:?}", $comment, duration);
        result
    }};
}
