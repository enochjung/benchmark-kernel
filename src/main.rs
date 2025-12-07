mod argument;
mod exec;
mod result;

use argument::ArgumentError;
use exec::ExecError;

const HELP_TEXT: &'static str = "\
Usage:
dgemm-test kernel_lib_dir [options...] \
\
options:\
  --nthreads=INTEGER\
  --layout=row or col\
  --transa=true or false\
  --transb=true or false\
  --alpha=DOUBLE\
  --beta=DOUBLE\
  --m=INTEGER\
  --n=INTEGER\
  --k=INTEGER\
  --warmup=INTEGER\
  --iter=INTEGER\
  --verify\
  --result-policy=min or avg or max\
  --only-result";

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let (dconf, tconf) = argument::parse_arguments(&args[1..]).unwrap_or_else(|err| {
        match err {
            ArgumentError::InvalidArgument => eprintln!("Error: invalid argument"),
            ArgumentError::NotEnoughArguments => eprintln!("Error: not enough arguments"),
        }
        eprintln!("{}", HELP_TEXT);
        std::process::exit(1);
    });

    let res = exec::execute(&dconf, &tconf).unwrap_or_else(|err| {
        match err {
            ExecError::LibNotFound => eprintln!("Error: library '{}' not found", dconf.lib_dir),
            ExecError::DgemmNotFound => {
                eprintln!("Error: dgemm not found in library {}", dconf.lib_dir)
            }
            ExecError::Invalid => eprintln!("Error: INVALID result"),
        }
        std::process::exit(1);
    });

    result::print(&dconf, &tconf, &res);
}

struct DgemmConfig {
    lib_dir: String,
    nthreads: u8,
    layout: MatrixLayout,
    transa: bool,
    transb: bool,
    alpha: f64,
    beta: f64,
    m: u32,
    n: u32,
    k: u32,
}

enum MatrixLayout {
    RowMajor,
    ColMajor,
}

struct TestConfig {
    warmup: u16,
    iter: u16,
    verify: bool,
    result_policy: ResultPolicy,
    only_result: bool,
}

enum ResultPolicy {
    Min,
    Avg,
    Max,
}

struct TestResult {
    sec: Vec<f64>,
}
