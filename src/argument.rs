use crate::{DgemmConfig, HpcgSpmvConfig, KernelConfig, MatrixLayout, Summary, TestConfig};
use regex::Regex;

use std::fmt;

#[derive(Debug)]
pub enum ArgumentError {
    InvalidArgument(String),
    NotEnoughArguments,
}

pub fn parse_arguments(args: &[String]) -> Result<(KernelConfig, TestConfig), ArgumentError> {
    if args.len() < 1 {
        return Err(ArgumentError::NotEnoughArguments);
    }

    match args[0].as_ref() {
        "dgemm" | "DGEMM" => parse_dgemm_arguments(&args[1..])
            .map(|(dgemm_conf, test_conf)| (KernelConfig::Dgemm(dgemm_conf), test_conf)),
        "spmv" | "SPMV" => parse_spmv_arguments(&args[1..])
            .map(|(spmv_conf, test_conf)| (KernelConfig::HpcgSpmv(spmv_conf), test_conf)),
        _ => Err(ArgumentError::InvalidArgument(args[0].to_string())),
    }
}

pub fn parse_dgemm_arguments(args: &[String]) -> Result<(DgemmConfig, TestConfig), ArgumentError> {
    if args.len() < 1 {
        return Err(ArgumentError::NotEnoughArguments);
    }

    let lib_dir = String::from(&args[0]);
    let conf = parse_options(&args[1..])?;

    let dgemm_config = DgemmConfig {
        nthreads: conf.nthreads.unwrap_or(1),
        layout: conf.layout.unwrap_or(MatrixLayout::RowMajor),
        transa: conf.transa.unwrap_or(false),
        transb: conf.transb.unwrap_or(false),
        alpha: conf.alpha.unwrap_or(1.0),
        beta: conf.beta.unwrap_or(1.0),
        m: conf.m.unwrap_or(100),
        n: conf.n.unwrap_or(100),
        k: conf.k.unwrap_or(100),
    };

    let test_config = TestConfig {
        lib_dir,
        warmup: conf.warmup.unwrap_or(0),
        iter: conf.iter.unwrap_or(5),
        verify: conf.verify.unwrap_or(false),
        summary: conf.summary.unwrap_or(Summary::Max),
        concise: conf.concise.unwrap_or(false),
    };

    Ok((dgemm_config, test_config))
}

pub fn parse_spmv_arguments(
    args: &[String],
) -> Result<(HpcgSpmvConfig, TestConfig), ArgumentError> {
    if args.len() < 1 {
        return Err(ArgumentError::NotEnoughArguments);
    }

    let lib_dir = String::from(&args[0]);
    let conf = parse_options(&args[1..])?;

    let spmv_config = HpcgSpmvConfig {
        n: conf.n.unwrap_or(104),
    };

    let test_config = TestConfig {
        lib_dir,
        warmup: conf.warmup.unwrap_or(0),
        iter: conf.iter.unwrap_or(5),
        verify: conf.verify.unwrap_or(false),
        summary: conf.summary.unwrap_or(Summary::Max),
        concise: conf.concise.unwrap_or(false),
    };

    Ok((spmv_config, test_config))
}

struct OptionConfig {
    nthreads: Option<u8>,
    layout: Option<MatrixLayout>,
    transa: Option<bool>,
    transb: Option<bool>,
    alpha: Option<f64>,
    beta: Option<f64>,
    m: Option<u32>,
    n: Option<u32>,
    k: Option<u32>,
    warmup: Option<u16>,
    iter: Option<u16>,
    verify: Option<bool>,
    summary: Option<Summary>,
    concise: Option<bool>,
}

struct OptionString {
    name: String,
    value: Option<String>,
}

fn parse_options(args: &[String]) -> Result<OptionConfig, ArgumentError> {
    let mut conf = OptionConfig {
        nthreads: None,
        layout: None,
        transa: None,
        transb: None,
        alpha: None,
        beta: None,
        m: None,
        n: None,
        k: None,
        warmup: None,
        iter: None,
        verify: None,
        summary: None,
        concise: None,
    };

    for arg in args {
        let OptionString { name, value } = parse_option(arg)?;

        match name.as_str() {
            "nthreads" | "nt" => {
                let value = value
                    .as_ref()
                    .ok_or(ArgumentError::InvalidArgument(arg.to_string()))?;
                let nthreads = str::parse::<u8>(value)
                    .map_err(|_| ArgumentError::InvalidArgument(arg.to_string()))?;

                match conf.nthreads {
                    None => {
                        conf.nthreads = Some(nthreads);
                        Ok(())
                    }
                    Some(_) => Err(ArgumentError::InvalidArgument(arg.to_string())),
                }
            }

            "layout" | "l" => {
                let value = value
                    .as_ref()
                    .ok_or(ArgumentError::InvalidArgument(arg.to_string()))?;
                let layout = match value.as_str() {
                    "row" | "r" => Ok(MatrixLayout::RowMajor),
                    "col" | "c" => Ok(MatrixLayout::ColMajor),
                    _ => Err(ArgumentError::InvalidArgument(arg.to_string())),
                }?;

                match conf.layout {
                    None => {
                        conf.layout = Some(layout);
                        Ok(())
                    }
                    Some(_) => Err(ArgumentError::InvalidArgument(arg.to_string())),
                }
            }

            "transa" | "ta" => {
                let value = value
                    .as_ref()
                    .ok_or(ArgumentError::InvalidArgument(arg.to_string()))?;
                let transa = match value.as_str() {
                    "true" | "t" => Ok(true),
                    "false" | "f" => Ok(false),
                    _ => Err(ArgumentError::InvalidArgument(arg.to_string())),
                }?;

                match conf.transa {
                    None => {
                        conf.transa = Some(transa);
                        Ok(())
                    }
                    Some(_) => Err(ArgumentError::InvalidArgument(arg.to_string())),
                }
            }
            "transb" | "tb" => {
                let value = value
                    .as_ref()
                    .ok_or(ArgumentError::InvalidArgument(arg.to_string()))?;
                let transb = match value.as_str() {
                    "true" | "t" => Ok(true),
                    "false" | "f" => Ok(false),
                    _ => Err(ArgumentError::InvalidArgument(arg.to_string())),
                }?;

                match conf.transb {
                    None => {
                        conf.transb = Some(transb);
                        Ok(())
                    }
                    Some(_) => Err(ArgumentError::InvalidArgument(arg.to_string())),
                }
            }

            "alpha" | "a" => {
                let value = value
                    .as_ref()
                    .ok_or(ArgumentError::InvalidArgument(arg.to_string()))?;
                let alpha = str::parse::<f64>(&value)
                    .map_err(|_| ArgumentError::InvalidArgument(arg.to_string()))?;

                match conf.alpha {
                    None => {
                        conf.alpha = Some(alpha);
                        Ok(())
                    }
                    Some(_) => Err(ArgumentError::InvalidArgument(arg.to_string())),
                }
            }

            "beta" | "b" => {
                let value = value
                    .as_ref()
                    .ok_or(ArgumentError::InvalidArgument(arg.to_string()))?;
                let beta = str::parse::<f64>(&value)
                    .map_err(|_| ArgumentError::InvalidArgument(arg.to_string()))?;

                match conf.beta {
                    None => {
                        conf.beta = Some(beta);
                        Ok(())
                    }
                    Some(_) => Err(ArgumentError::InvalidArgument(arg.to_string())),
                }
            }

            "m" => {
                let value = value
                    .as_ref()
                    .ok_or(ArgumentError::InvalidArgument(arg.to_string()))?;
                let m = str::parse::<u32>(&value)
                    .map_err(|_| ArgumentError::InvalidArgument(arg.to_string()))?;

                match conf.m {
                    None => {
                        conf.m = Some(m);
                        Ok(())
                    }
                    Some(_) => Err(ArgumentError::InvalidArgument(arg.to_string())),
                }
            }

            "n" => {
                let value = value
                    .as_ref()
                    .ok_or(ArgumentError::InvalidArgument(arg.to_string()))?;
                let n = str::parse::<u32>(&value)
                    .map_err(|_| ArgumentError::InvalidArgument(arg.to_string()))?;

                match conf.n {
                    None => {
                        conf.n = Some(n);
                        Ok(())
                    }
                    Some(_) => Err(ArgumentError::InvalidArgument(arg.to_string())),
                }
            }

            "k" => {
                let value = value
                    .as_ref()
                    .ok_or(ArgumentError::InvalidArgument(arg.to_string()))?;
                let k = str::parse::<u32>(&value)
                    .map_err(|_| ArgumentError::InvalidArgument(arg.to_string()))?;

                match conf.k {
                    None => {
                        conf.k = Some(k);
                        Ok(())
                    }
                    Some(_) => Err(ArgumentError::InvalidArgument(arg.to_string())),
                }
            }

            "warmup" | "w" => {
                let value = value
                    .as_ref()
                    .ok_or(ArgumentError::InvalidArgument(arg.to_string()))?;
                let warmup = str::parse::<u16>(&value)
                    .map_err(|_| ArgumentError::InvalidArgument(arg.to_string()))?;

                match conf.warmup {
                    None => {
                        conf.iter = Some(warmup);
                        Ok(())
                    }
                    Some(_) => Err(ArgumentError::InvalidArgument(arg.to_string())),
                }
            }

            "iter" | "i" => {
                let value = value
                    .as_ref()
                    .ok_or(ArgumentError::InvalidArgument(arg.to_string()))?;
                let iter = str::parse::<u16>(&value)
                    .map_err(|_| ArgumentError::InvalidArgument(arg.to_string()))?;

                match conf.iter {
                    None => {
                        conf.iter = Some(iter);
                        Ok(())
                    }
                    Some(_) => Err(ArgumentError::InvalidArgument(arg.to_string())),
                }
            }

            "verify" => {
                if value.is_none() {
                    Ok(())
                } else {
                    Err(ArgumentError::InvalidArgument(arg.to_string()))
                }?;

                match conf.verify {
                    None => {
                        conf.verify = Some(true);
                        Ok(())
                    }
                    Some(_) => Err(ArgumentError::InvalidArgument(arg.to_string())),
                }
            }

            "summary" => {
                let value = value
                    .as_ref()
                    .ok_or(ArgumentError::InvalidArgument(arg.to_string()))?;
                let summary = match value.as_str() {
                    "min" => Ok(Summary::Min),
                    "avg" => Ok(Summary::Avg),
                    "max" => Ok(Summary::Max),
                    _ => Err(ArgumentError::InvalidArgument(arg.to_string())),
                }?;

                match conf.summary {
                    None => {
                        conf.summary = Some(summary);
                        Ok(())
                    }
                    Some(_) => Err(ArgumentError::InvalidArgument(arg.to_string())),
                }
            }

            "concise" => {
                if value.is_none() {
                    Ok(())
                } else {
                    Err(ArgumentError::InvalidArgument(arg.to_string()))
                }?;

                match conf.concise {
                    None => {
                        conf.concise = Some(true);
                        Ok(())
                    }
                    Some(_) => Err(ArgumentError::InvalidArgument(arg.to_string())),
                }
            }

            "nrow" => {
                let value = value
                    .as_ref()
                    .ok_or(ArgumentError::InvalidArgument(arg.to_string()))?;
                let m = str::parse::<u32>(&value)
                    .map_err(|_| ArgumentError::InvalidArgument(arg.to_string()))?;

                match conf.m {
                    None => {
                        conf.m = Some(m);
                        Ok(())
                    }
                    Some(_) => Err(ArgumentError::InvalidArgument(arg.to_string())),
                }
            }

            _ => Err(ArgumentError::InvalidArgument(arg.to_string())),
        }?;
    }

    Ok(conf)
}

const REGEX_PATTERN: &'static str = r#"^--([a-z]+)(?:=([a-z0-9.]+))?$"#;

fn parse_option(arg: &str) -> Result<OptionString, ArgumentError> {
    let regex_module = Regex::new(REGEX_PATTERN).unwrap();
    let caps = regex_module
        .captures(arg)
        .ok_or(ArgumentError::InvalidArgument(arg.to_string()))?;

    if caps.get(1).is_none() || caps.get(3).is_some() {
        return Err(ArgumentError::InvalidArgument(arg.to_string()));
    }

    let name = String::from(
        caps.get(1)
            .ok_or(ArgumentError::InvalidArgument(arg.to_string()))?
            .as_str(),
    );
    let value = caps.get(2).map(|x| String::from(x.as_str()));

    Ok(OptionString { name, value })
}

impl fmt::Display for ArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidArgument(arg) => write!(f, "invalid argument: {}", arg),
            Self::NotEnoughArguments => write!(f, "not enough argument"),
        }
    }
}

impl std::error::Error for ArgumentError {}
