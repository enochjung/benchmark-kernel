use crate::{DgemmConfig, MatrixLayout, TestConfig, TestResult};

use std::fmt;

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

#[derive(Debug)]
pub enum ExecError {
    LibNotFound,
    DgemmNotFound,
    Invalid,
}

#[allow(non_snake_case)]
pub fn execute(
    dgemm_config: &DgemmConfig,
    test_config: &TestConfig,
) -> Result<TestResult, ExecError> {
    let lib = load_library(&dgemm_config.lib_dir)?;
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

    let mut A = vec![1.1; m_usize * k_usize];
    let mut B = vec![1.1; k_usize * n_usize];
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

#[allow(non_snake_case)]
fn verify_dgemm(
    A: &mut [f64],
    B: &mut [f64],
    C: &mut [f64],
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

    const EPSILON: f64 = 1.0e-10;
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

fn load_library(lib_dir: &str) -> Result<libloading::Library, ExecError> {
    unsafe {
        let lib = libloading::Library::new(lib_dir).map_err(|_| ExecError::LibNotFound)?;
        Ok(lib)
    }
}

fn load_dgemm<'lib>(lib: &'lib libloading::Library) -> Result<DgemmFunc<'lib>, ExecError> {
    unsafe {
        let dgemm = lib
            .get(b"cblas_dgemm")
            .map_err(|_| ExecError::DgemmNotFound)?;
        Ok(dgemm)
    }
}

#[allow(non_snake_case)]
fn call_dgemm(
    A: &[f64],
    B: &[f64],
    C: &mut [f64],
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

impl fmt::Display for ExecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LibNotFound => write!(f, "library not found"),
            Self::DgemmNotFound => write!(f, "dgemm function not found"),
            Self::Invalid => write!(f, "invalid result"),
        }
    }
}

impl std::error::Error for ExecError {}
