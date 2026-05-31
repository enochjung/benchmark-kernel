use crate::{
    DgemmConfig, HpcgSpmvConfig, KernelConfig, MatrixLayout, Summary, TestConfig, TestResult,
};

pub fn print(
    kernel_config: &KernelConfig,
    test_config: &TestConfig,
    test_result: &TestResult,
) -> () {
    match kernel_config {
        KernelConfig::Dgemm(dgemm_conf) => {
            if test_config.concise {
                dgemm_print_only_summary(dgemm_conf, test_config, test_result);
            } else {
                dgemm_print_full_result(dgemm_conf, test_config, test_result);
            }
        }
        KernelConfig::HpcgSpmv(spmv_conf) => {
            if test_config.concise {
                spmv_print_only_summary(spmv_conf, test_config, test_result);
            } else {
                spmv_print_full_result(spmv_conf, test_config, test_result);
            }
        }
    }
}

fn dgemm_print_full_result(
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

    println!("{:<10} {:>20}", "lib_dir", test_config.lib_dir);
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

    let (selected_sec, label) = get_statistics(&test_config.summary, &test_result.sec);
    let gflops = get_gflops(dgemm_config, selected_sec);
    println!("{}: {:>11.4}{:>15.4}", label, selected_sec, gflops);

    println!("===============================");
}

fn dgemm_print_only_summary(
    dgemm_config: &DgemmConfig,
    test_config: &TestConfig,
    test_result: &TestResult,
) -> () {
    let (selected_sec, label) = get_statistics(&test_config.summary, &test_result.sec);
    let gflops = get_gflops(dgemm_config, selected_sec);
    println!("{}:     {:>10.4} {:>15.4}", label, selected_sec, gflops);
}

fn spmv_print_full_result(
    spmv_config: &HpcgSpmvConfig,
    test_config: &TestConfig,
    test_result: &TestResult,
) -> () {
    println!("===============================");
    println!("{:<10} {:>20}", "lib_dir", test_config.lib_dir);
    println!("{:<10} {:>20}", "n", spmv_config.n);
    println!(
        "{:<10} {:>20}",
        "nrow",
        spmv_config.n * spmv_config.n * spmv_config.n
    );
    println!("===============================");
    println!("{:>16}{:>15}", "sec", "gflops");
    for (i, &sec) in test_result.sec.iter().enumerate() {
        let gflops = get_spmv_gflops(spmv_config, sec);
        println!("{:>3}) {:>11.4}{:>15.4}", i + 1, sec, gflops);
    }
    println!("===============================");
    let (selected_sec, label) = get_statistics(&test_config.summary, &test_result.sec);
    let gflops = get_spmv_gflops(spmv_config, selected_sec);
    println!("{}: {:>11.4}{:>15.4}", label, selected_sec, gflops);
    println!("===============================");
}

fn spmv_print_only_summary(
    spmv_config: &HpcgSpmvConfig,
    test_config: &TestConfig,
    test_result: &TestResult,
) -> () {
    let (selected_sec, label) = get_statistics(&test_config.summary, &test_result.sec);
    let gflops = get_spmv_gflops(spmv_config, selected_sec);
    println!("{}:     {:>10.4} {:>15.4}", label, selected_sec, gflops);
}

fn get_statistics(summary: &Summary, values: &[f64]) -> (f64, &'static str) {
    match summary {
        Summary::Min => {
            let max_val = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            (max_val, "min")
        }
        Summary::Avg => {
            let avg_val = values.iter().sum::<f64>() / values.len() as f64;
            (avg_val, "avg")
        }
        Summary::Max => {
            let min_val = values.iter().cloned().fold(f64::INFINITY, f64::min);
            (min_val, "max")
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

fn get_spmv_gflops(spmv_config: &HpcgSpmvConfig, sec: f64) -> f64 {
    let n = spmv_config.n as f64;
    let nrow = n * n * n;
    let flops = 2.0 * nrow * 27.0;
    flops / (sec * 1e9)
}
