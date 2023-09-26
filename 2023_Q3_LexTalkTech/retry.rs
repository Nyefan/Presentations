use std::cmp::min;
use std::thread::sleep;
use std::time::Duration;

use once_cell::sync::Lazy;
use rand::distributions::{Distribution, Uniform};

struct Config {
    retry_backoff_factor: f64,
    retry_initial_delay_ms: usize,
    retry_jitter_factor_ms: usize,
    retry_max_retries: usize,
    retry_max_delay_ms: usize,
    retry_retryable_error_codes: Vec<usize>,
}

static CONFIG: Lazy<Config, fn() -> Config> = Lazy::new(|| {
    Config {
        retry_backoff_factor: 1.1,
        retry_initial_delay_ms: 10,
        retry_jitter_factor_ms: 10,
        retry_max_retries: 3,
        retry_max_delay_ms: 4000,
        retry_retryable_error_codes: vec!(408, 429, 502, 503, 504),
    }
});

fn retry<O, E, F>(network_call: F) -> Result<O, E>
    where
        F: Fn() -> Result<O, E>,
{
    let mut retries = 0;
    let mut delay_ms = CONFIG.retry_initial_delay_ms;
    let mut rng = rand::thread_rng();
    let rng_distribution = Uniform::from(0.0..1.0);
    loop {
        match network_call() {
            Ok(result) => return Ok(result),
            Err(_) if retries < CONFIG.retry_max_retries && CONFIG.retry_retryable_error_codes.contains(&429) => {
                retries += 1;
                delay_ms = min(
                    CONFIG.retry_max_delay_ms,
                    (delay_ms as f64 * CONFIG.retry_backoff_factor + rng_distribution.sample(&mut rng) * CONFIG.retry_jitter_factor_ms as f64) as usize,
                );
                sleep(Duration::from_millis(delay_ms as u64));
            }
            Err(err) => return Err(err),
        }
    }
}

fn main() {
    let data = "some data2";
    let result = retry(|| {network_operation(data)});

    match result {
        Ok(val) => println!("Operation succeeded with message: {:?}", val),
        Err(err) => println!("Operation failed after {} retries: {:?}", CONFIG.retry_max_retries, err),
    }
}

fn network_operation(data: &str) -> Result<String, &'static str> {
    if data == "some data" {
        Ok("Success!".to_string())
    } else {
        println!("error");
        Err("Network error")
    }
}