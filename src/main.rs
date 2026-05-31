mod argument;
mod exec;
mod numa_memory;
mod result;

use argument::ArgumentError;
use exec::ExecError;

const HELP_TEXT: &'static str = "\
Usage: bench COMMAND kernel-lib [options...]

Commands:

  dgemm
    --nthreads=INTEGER
    --layout=[row|col]
    --transa=[true|false]
    --transb=[true|false]
    --alpha=DOUBLE
    --beta=DOUBLE
    --m=INTEGER
    --n=INTEGER
    --k=INTEGER
    --warmup=INTEGER
    --iter=INTEGER
    --verify
    --summary=[min|avg|max]
    --concise

  spmv
    --n=INTEGER
    --warmup=INTEGER
    --iter=INTEGER
    --verify
    --summary=[min|avg|max]
    --concise";

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let (kernel_conf, test_conf) = argument::parse_arguments(&args[1..]).unwrap_or_else(|err| {
        match err {
            ArgumentError::InvalidArgument(arg) => eprintln!("Error: invalid argument: {}", arg),
            ArgumentError::NotEnoughArguments => eprintln!("Error: not enough arguments"),
        }
        eprintln!("{}", HELP_TEXT);
        std::process::exit(1);
    });

    let res = exec::execute(&kernel_conf, &test_conf).unwrap_or_else(|err| {
        match err {
            ExecError::LibNotFound => {
                eprintln!("Error: library '{}' not found", test_conf.lib_dir)
            }
            ExecError::FunctionNotFound => {
                eprintln!(
                    "Error: kernel function not found in library {}",
                    test_conf.lib_dir
                )
            }
            ExecError::Invalid => eprintln!("Error: INVALID result"),
            ExecError::MemAllocFailed => eprintln!("Error: memory allocation failed"),
        }
        std::process::exit(1);
    });

    result::print(&kernel_conf, &test_conf, &res);
}

enum KernelConfig {
    Dgemm(DgemmConfig),
    HpcgSpmv(HpcgSpmvConfig),
}

struct DgemmConfig {
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

struct HpcgSpmvConfig {
    n: u32,
}

enum MatrixLayout {
    RowMajor,
    ColMajor,
}

struct TestConfig {
    lib_dir: String,
    warmup: u16,
    iter: u16,
    verify: bool,
    summary: Summary,
    concise: bool,
}

enum Summary {
    Min,
    Avg,
    Max,
}

struct TestResult {
    sec: Vec<f64>,
}
