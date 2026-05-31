use crate::{
    DgemmConfig, HpcgSpmvConfig, KernelConfig, MatrixLayout, TestConfig, TestResult,
    numa_memory::NumaMemory,
};
use std::fmt;

const EPSILON: f64 = 1.0e-10;

type DgemmFunc<'lib> = libloading::Symbol<
    'lib,
    unsafe extern "C" fn(
        layout: i32,
        transa: i32,
        transb: i32,
        m: i32,
        n: i32,
        k: i32,
        alpha: f64,
        a: *const f64,
        lda: i32,
        b: *const f64,
        ldb: i32,
        beta: f64,
        c: *mut f64,
        ldc: i32,
    ),
>;

type SpmvFunc<'lib> = libloading::Symbol<
    'lib,
    unsafe extern "C" fn(
        nrow: i32,
        cols: *const i32,
        vals: *const f64,
        x: *const f64,
        tmp: *mut f64,
        y: *mut f64,
    ),
>;

#[derive(Debug)]
pub enum ExecError {
    MemAllocFailed,
    LibNotFound,
    FunctionNotFound,
    Invalid,
}

pub fn execute(
    kernel_config: &KernelConfig,
    test_config: &TestConfig,
) -> Result<TestResult, ExecError> {
    match kernel_config {
        KernelConfig::Dgemm(dgemm_conf) => execute_dgemm(dgemm_conf, test_config),
        KernelConfig::HpcgSpmv(spmv_conf) => execute_spmv(spmv_conf, test_config),
    }
}

pub fn execute_dgemm(
    dgemm_config: &DgemmConfig,
    test_config: &TestConfig,
) -> Result<TestResult, ExecError> {
    let lib = load_library(&test_config.lib_dir)?;
    let dgemm = load_dgemm(&lib)?;

    let m = dgemm_config.m;
    let n = dgemm_config.n;
    let k = dgemm_config.k;
    let layout = &dgemm_config.layout;
    let transa = dgemm_config.transa;
    let transb = dgemm_config.transb;
    let alpha = dgemm_config.alpha;
    let beta = dgemm_config.beta;

    let m_usize = m as usize;
    let n_usize = n as usize;
    let k_usize = k as usize;

    #[allow(non_snake_case)]
    let mut A = vec![1.1; m_usize * k_usize];
    #[allow(non_snake_case)]
    let mut B = vec![1.1; k_usize * n_usize];
    #[allow(non_snake_case)]
    let mut C = vec![1.1; m_usize * n_usize];

    unsafe {
        std::env::set_var("OMP_NUM_THREADS", dgemm_config.nthreads.to_string());
    }

    if test_config.verify {
        verify_dgemm(
            &mut A, &mut B, &mut C, &dgemm, &layout, transa, transb, alpha, beta, m, n, k,
        )?;
    }

    for _ in 0..test_config.warmup {
        call_dgemm(
            &A, &B, &mut C, &dgemm, &layout, transa, transb, alpha, beta, m as u32, n as u32,
            k as u32,
        )?;
    }

    let mut times = Vec::new();
    for _ in 0..test_config.iter {
        let start = std::time::Instant::now();
        call_dgemm(
            &A, &B, &mut C, &dgemm, &layout, transa, transb, alpha, beta, m as u32, n as u32,
            k as u32,
        )?;
        let elapsed = start.elapsed().as_secs_f64();
        times.push(elapsed);
    }

    Ok(TestResult { sec: times })
}

fn verify_dgemm(
    #[allow(non_snake_case)] A: &mut [f64],
    #[allow(non_snake_case)] B: &mut [f64],
    #[allow(non_snake_case)] C: &mut [f64],
    dgemm: &DgemmFunc,
    layout: &MatrixLayout,
    transa: bool,
    transb: bool,
    _alpha_param: f64,
    _beta_param: f64,
    m: u32,
    n: u32,
    k: u32,
) -> Result<(), ExecError> {
    let alpha = 1.0;
    let beta = 1.0;

    let m_usize = m as usize;
    let n_usize = n as usize;
    let k_usize = k as usize;

    match layout {
        MatrixLayout::RowMajor => {
            for i in 0..m_usize {
                for p in 0..k_usize {
                    A[i * k_usize + p] = (i + 1) as f64;
                }
            }
            for p in 0..k_usize {
                for j in 0..n_usize {
                    B[p * n_usize + j] = (j + 1) as f64;
                }
            }
            for i in 0..m_usize {
                for j in 0..n_usize {
                    C[i * n_usize + j] = -1000.0;
                }
            }
        }
        MatrixLayout::ColMajor => {
            for p in 0..k_usize {
                for i in 0..m_usize {
                    A[p * m_usize + i] = (i + 1) as f64;
                }
            }
            for j in 0..n_usize {
                for p in 0..k_usize {
                    B[j * k_usize + p] = (j + 1) as f64;
                }
            }
            for j in 0..n_usize {
                for i in 0..m_usize {
                    C[j * m_usize + i] = -1000.0;
                }
            }
        }
    }

    call_dgemm(A, B, C, dgemm, layout, transa, transb, alpha, beta, m, n, k)?;

    let k_f64 = k as f64;

    match layout {
        MatrixLayout::RowMajor => {
            for i in 0..m_usize {
                for j in 0..n_usize {
                    let expected = k_f64 * ((i + 1) as f64) * ((j + 1) as f64) - 1000.0;
                    let got = C[i * n_usize + j];
                    if (expected - got).abs() > EPSILON {
                        return Err(ExecError::Invalid);
                    }
                }
            }
        }
        MatrixLayout::ColMajor => {
            for j in 0..n_usize {
                for i in 0..m_usize {
                    let expected = k_f64 * ((i + 1) as f64) * ((j + 1) as f64) - 1000.0;
                    let got = C[j * n_usize + i];
                    if (expected - got).abs() > EPSILON {
                        return Err(ExecError::Invalid);
                    }
                }
            }
        }
    }

    Ok(())
}

pub fn execute_spmv(
    spmv_config: &HpcgSpmvConfig,
    test_config: &TestConfig,
) -> Result<TestResult, ExecError> {
    let lib = load_library(&test_config.lib_dir)?;
    let spmv = load_spmv(&lib)?;

    let n = spmv_config.n as usize;
    let nrow = n * n * n;

    let mut cols = NumaMemory::<u32>::alloc_on_node(nrow * 32 + 1024, 1)
        .map_err(|_| ExecError::MemAllocFailed)?;
    let cols_slice = cols.as_mut_slice();

    let mut vals = NumaMemory::<f64>::alloc_on_node(nrow * 32 + 1024, 1)
        .map_err(|_| ExecError::MemAllocFailed)?;
    let vals_slice = vals.as_mut_slice();

    let mut x =
        NumaMemory::<f64>::alloc_on_node(nrow + 1024, 1).map_err(|_| ExecError::MemAllocFailed)?;
    let x_slice = x.as_mut_slice();

    let mut tmp = NumaMemory::<f64>::alloc_on_node(64, 1).map_err(|_| ExecError::MemAllocFailed)?;

    let mut y =
        NumaMemory::<f64>::alloc_on_node(nrow + 1024, 1).map_err(|_| ExecError::MemAllocFailed)?;
    let y_slice = y.as_mut_slice();

    for i in 0..nrow {
        x_slice[i] = 2.0;
        y_slice[i] = 999.0;
    }

    let d = [
        [-1, -1, -1],
        [0, -1, -1],
        [1, -1, -1],
        [-1, 0, -1],
        [0, 0, -1],
        [1, 0, -1],
        [-1, 1, -1],
        [0, 1, -1],
        [1, 1, -1],
        [-1, -1, 0],
        [0, -1, 0],
        [1, -1, 0],
        [-1, 0, 0],
        [0, 0, 0],
        [1, 0, 0],
        [-1, 1, 0],
        [0, 1, 0],
        [1, 1, 0],
        [-1, -1, 1],
        [0, -1, 1],
        [1, -1, 1],
        [-1, 0, 1],
        [0, 0, 1],
        [1, 0, 1],
        [-1, 1, 1],
        [0, 1, 1],
        [1, 1, 1],
    ];

    let n = n as i32;
    for i in 0..n * n * n {
        let xi = i % n;
        let yi = i / n % n;
        let zi = i / n / n;

        let mut lnnz = 0;
        let mut unnz = 0;

        for dd in d {
            let nx = xi + dd[0];
            let ny = yi + dd[1];
            let nz = zi + dd[2];

            if 0 <= nx && nx < n && 0 <= ny && ny < n && 0 <= nz && nz < n {
                let ni = nz * n * n + ny * n + nx;
                if ni < i {
                    cols_slice[i as usize * 32 + lnnz] = ni as u32;
                    vals_slice[i as usize * 32 + lnnz] = -1.0;
                    lnnz += 1;
                } else if ni == i {
                    cols_slice[i as usize * 32 + 16] = ni as u32;
                    vals_slice[i as usize * 32 + 16] = 26.0;
                } else {
                    cols_slice[i as usize * 32 + 17 + unnz] = ni as u32;
                    vals_slice[i as usize * 32 + 17 + unnz] = -1.0;
                    unnz += 1;
                }
            }
        }
    }

    if test_config.verify {
        call_spmv(
            &spmv,
            nrow as u32,
            &cols.as_slice(),
            &vals.as_slice(),
            &x.as_slice(),
            &mut tmp.as_mut_slice(),
            &mut y.as_mut_slice(),
        )?;
        verify_spmv_result(&mut y.as_mut_slice(), n)?;
    }

    for _ in 0..test_config.warmup {
        call_spmv(
            &spmv,
            nrow as u32,
            &cols.as_slice(),
            &vals.as_slice(),
            &x.as_slice(),
            &mut tmp.as_mut_slice(),
            &mut y.as_mut_slice(),
        )?;
    }

    let mut times = Vec::new();
    for _ in 0..test_config.iter {
        let start = std::time::Instant::now();
        call_spmv(
            &spmv,
            nrow as u32,
            &cols.as_slice(),
            &vals.as_slice(),
            &x.as_slice(),
            &mut tmp.as_mut_slice(),
            &mut y.as_mut_slice(),
        )?;
        let elapsed = start.elapsed().as_secs_f64();
        times.push(elapsed);
    }

    Ok(TestResult { sec: times })
}

fn verify_spmv_result(y: &mut [f64], n: i32) -> Result<(), ExecError> {
    for zi in 0..n {
        for yi in 0..n {
            for xi in 0..n {
                let row = zi * n * n + yi * n + xi;

                let mut avail = 0;
                for dz in -1..2 {
                    for dy in -1..2 {
                        for dx in -1..2 {
                            let nx = xi + dx;
                            let ny = yi + dy;
                            let nz = zi + dz;

                            if 0 <= nx && nx < n && 0 <= ny && ny < n && 0 <= nz && nz < n {
                                avail += 1;
                            }
                        }
                    }
                }

                let expected = (2 * (27 - avail)) as f64;
                let got = y[row as usize];
                if (expected - got).abs() > EPSILON {
                    eprintln!("row: {}, exp: {}, got: {}", row, expected, got);
                    return Err(ExecError::Invalid);
                }
            }
        }
    }

    Ok(())
}

fn load_library(lib_dir: &str) -> Result<libloading::Library, ExecError> {
    unsafe {
        let lib = libloading::Library::new(lib_dir).map_err(|_| ExecError::LibNotFound)?;
        Ok(lib)
    }
}

fn load_dgemm<'lib>(lib: &'lib libloading::Library) -> Result<DgemmFunc<'lib>, ExecError> {
    unsafe {
        let dgemm = lib
            .get(b"call_dgemm")
            .map_err(|_| ExecError::FunctionNotFound)?;
        Ok(dgemm)
    }
}

fn load_spmv<'lib>(lib: &'lib libloading::Library) -> Result<SpmvFunc<'lib>, ExecError> {
    unsafe {
        let spmv = lib.get(b"spmv").map_err(|_| ExecError::FunctionNotFound)?;
        Ok(spmv)
    }
}

fn call_dgemm(
    #[allow(non_snake_case)] A: &[f64],
    #[allow(non_snake_case)] B: &[f64],
    #[allow(non_snake_case)] C: &mut [f64],
    dgemm: &DgemmFunc,
    layout: &MatrixLayout,
    transa: bool,
    transb: bool,
    alpha: f64,
    beta: f64,
    m: u32,
    n: u32,
    k: u32,
) -> Result<(), ExecError> {
    let layout_code = match layout {
        MatrixLayout::RowMajor => 101,
        MatrixLayout::ColMajor => 102,
    };

    let transa_code = match transa {
        true => 112,
        false => 111,
    };

    let transb_code = match transb {
        true => 112,
        false => 111,
    };

    let (lda, ldb, ldc) = match layout {
        MatrixLayout::RowMajor => (k as i32, n as i32, n as i32),
        MatrixLayout::ColMajor => (m as i32, k as i32, m as i32),
    };

    unsafe {
        dgemm(
            layout_code,
            transa_code,
            transb_code,
            m as i32,
            n as i32,
            k as i32,
            alpha,
            A.as_ptr(),
            lda,
            B.as_ptr(),
            ldb,
            beta,
            C.as_mut_ptr(),
            ldc,
        );
    }

    Ok(())
}

fn call_spmv(
    spmv: &SpmvFunc,
    nrow: u32,
    cols: &[u32],
    vals: &[f64],
    x: &[f64],
    tmp: &mut [f64],
    y: &mut [f64],
) -> Result<(), ExecError> {
    unsafe {
        spmv(
            nrow as i32,
            cols.as_ptr() as *const i32,
            vals.as_ptr(),
            x.as_ptr(),
            tmp.as_mut_ptr(),
            y.as_mut_ptr(),
        );
    }

    Ok(())
}

impl fmt::Display for ExecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LibNotFound => write!(f, "library not found"),
            Self::FunctionNotFound => write!(f, "kernel function not found"),
            Self::Invalid => write!(f, "invalid result"),
            Self::MemAllocFailed => write!(f, "memory allocation failed"),
        }
    }
}

impl std::error::Error for ExecError {}
