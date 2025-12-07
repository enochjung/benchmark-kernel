use crate::{DgemmConfig, MatrixLayout, TestConfig, TestResult};

pub fn print(dgemm_config: &DgemmConfig, test_config: &TestConfig, test_result: &TestResult) -> () {
    if test_config.only_result {
        print_only_result(dgemm_config, test_config, test_result);
    } else {
        print_full_result(dgemm_config, test_config, test_result);
    }
}

fn print_full_result(
    dgemm_config: &DgemmConfig,
    test_config: &TestConfig,
    test_result: &TestResult,
) -> () {
    println!("===============================");

    let layout_str = match dgemm_config.layout {
        MatrixLayout::RowMajor => "RowMajor",
        MatrixLayout::ColMajor => "ColMajor",
    };

    let transa_str = if dgemm_config.transa { "true" } else { "false" };
    let transb_str = if dgemm_config.transb { "true" } else { "false" };

    println!("{:<10} {:>20}", "lib_dir", dgemm_config.lib_dir);
    println!("{:<10} {:>20}", "nthreads", dgemm_config.nthreads);
    println!("{:<10} {:>20}", "layout", layout_str);
    println!("{:<10} {:>20}", "transa", transa_str);
    println!("{:<10} {:>20}", "transb", transb_str);
    println!("{:<10} {:>20.1}", "alpha", dgemm_config.alpha);
    println!("{:<10} {:>20.1}", "beta", dgemm_config.beta);
    println!("{:<10} {:>20}", "m", dgemm_config.m);
    println!("{:<10} {:>20}", "n", dgemm_config.n);
    println!("{:<10} {:>20}", "k", dgemm_config.k);
    println!("{:<10} {:>20}", "warmup", test_config.warmup);

    println!("===============================");

    println!("{:>16}{:>15}", "sec", "gflops");

    for (i, &sec) in test_result.sec.iter().enumerate() {
        let gflops = get_gflops(dgemm_config, sec);
        println!("{:>3}) {:>11.4}{:>15.4}", i + 1, sec, gflops);
    }

    println!("===============================");

    let (selected_sec, label) = get_statistics(&test_config.result_policy, &test_result.sec);
    let gflops = get_gflops(dgemm_config, selected_sec);
    println!("{}: {:>11.4}{:>15.4}", label, selected_sec, gflops);

    println!("===============================");
}

fn print_only_result(
    dgemm_config: &DgemmConfig,
    test_config: &TestConfig,
    test_result: &TestResult,
) -> () {
    let (selected_sec, label) = get_statistics(&test_config.result_policy, &test_result.sec);
    let gflops = get_gflops(dgemm_config, selected_sec);
    println!("{}:     {:>10.4} {:>15.4}", label, selected_sec, gflops);
}

fn get_statistics(result_policy: &crate::ResultPolicy, values: &[f64]) -> (f64, &'static str) {
    match result_policy {
        crate::ResultPolicy::Min => {
            let min_val = values.iter().cloned().fold(f64::INFINITY, f64::min);
            (min_val, "min")
        }
        crate::ResultPolicy::Avg => {
            let avg_val = values.iter().sum::<f64>() / values.len() as f64;
            (avg_val, "avg")
        }
        crate::ResultPolicy::Max => {
            let max_val = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            (max_val, "max")
        }
    }
}

fn get_gflops(dgemm_config: &DgemmConfig, sec: f64) -> f64 {
    let m = dgemm_config.m as f64;
    let n = dgemm_config.n as f64;
    let k = dgemm_config.k as f64;
    let flops = 2.0 * m * n * k;
    flops / (sec * 1e9)
}
