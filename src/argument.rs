use crate::{DgemmConfig, MatrixLayout, ResultPolicy, TestConfig};
use regex::Regex;

use std::fmt;

#[derive(Debug)]
pub enum ArgumentError {
    InvalidArgument,
    NotEnoughArguments,
}

pub fn parse_arguments(args: &[String]) -> Result<(DgemmConfig, TestConfig), ArgumentError> {
    if args.len() < 1 {
        return Err(ArgumentError::NotEnoughArguments);
    }

    let lib_dir = String::from(args[0].as_str());
    let conf = parse_options(&args[1..])?;

    let dgemm_config = DgemmConfig {
        lib_dir,
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
        warmup: conf.warmup.unwrap_or(0),
        iter: conf.iter.unwrap_or(5),
        verify: conf.verify.unwrap_or(false),
        result_policy: conf.result_policy.unwrap_or(ResultPolicy::Max),
        only_result: conf.only_result.unwrap_or(false),
    };

    Ok((dgemm_config, test_config))
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
    result_policy: Option<ResultPolicy>,
    only_result: Option<bool>,
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
        result_policy: None,
        only_result: None,
    };

    for arg in args {
        let OptionString { name, value } = parse_option(arg)?;

        match name.as_str() {
            "nthreads" | "nt" => {
                let value = value.as_ref().ok_or(ArgumentError::InvalidArgument)?;
                let nthreads =
                    str::parse::<u8>(value).map_err(|_| ArgumentError::InvalidArgument)?;

                match conf.nthreads {
                    None => {
                        conf.nthreads = Some(nthreads);
                        Ok(())
                    }
                    Some(_) => Err(ArgumentError::InvalidArgument),
                }
            }

            "layout" | "l" => {
                let value = value.as_ref().ok_or(ArgumentError::InvalidArgument)?;
                let layout = match value.as_str() {
                    "row" | "r" => Ok(MatrixLayout::RowMajor),
                    "col" | "c" => Ok(MatrixLayout::ColMajor),
                    _ => Err(ArgumentError::InvalidArgument),
                }?;

                match conf.layout {
                    None => {
                        conf.layout = Some(layout);
                        Ok(())
                    }
                    Some(_) => Err(ArgumentError::InvalidArgument),
                }
            }

            "transa" | "ta" => {
                let value = value.as_ref().ok_or(ArgumentError::InvalidArgument)?;
                let transa = match value.as_str() {
                    "true" | "t" => Ok(true),
                    "false" | "f" => Ok(false),
                    _ => Err(ArgumentError::InvalidArgument),
                }?;

                match conf.transa {
                    None => {
                        conf.transa = Some(transa);
                        Ok(())
                    }
                    Some(_) => Err(ArgumentError::InvalidArgument),
                }
            }
            "transb" | "tb" => {
                let value = value.as_ref().ok_or(ArgumentError::InvalidArgument)?;
                let transb = match value.as_str() {
                    "true" | "t" => Ok(true),
                    "false" | "f" => Ok(false),
                    _ => Err(ArgumentError::InvalidArgument),
                }?;

                match conf.transb {
                    None => {
                        conf.transb = Some(transb);
                        Ok(())
                    }
                    Some(_) => Err(ArgumentError::InvalidArgument),
                }
            }

            "alpha" | "a" => {
                let value = value.as_ref().ok_or(ArgumentError::InvalidArgument)?;
                let alpha =
                    str::parse::<f64>(&value).map_err(|_| ArgumentError::InvalidArgument)?;

                match conf.alpha {
                    None => {
                        conf.alpha = Some(alpha);
                        Ok(())
                    }
                    Some(_) => Err(ArgumentError::InvalidArgument),
                }
            }

            "beta" | "b" => {
                let value = value.as_ref().ok_or(ArgumentError::InvalidArgument)?;
                let beta = str::parse::<f64>(&value).map_err(|_| ArgumentError::InvalidArgument)?;

                match conf.beta {
                    None => {
                        conf.beta = Some(beta);
                        Ok(())
                    }
                    Some(_) => Err(ArgumentError::InvalidArgument),
                }
            }

            "m" => {
                let value = value.as_ref().ok_or(ArgumentError::InvalidArgument)?;
                let m = str::parse::<u32>(&value).map_err(|_| ArgumentError::InvalidArgument)?;

                match conf.m {
                    None => {
                        conf.m = Some(m);
                        Ok(())
                    }
                    Some(_) => Err(ArgumentError::InvalidArgument),
                }
            }

            "n" => {
                let value = value.as_ref().ok_or(ArgumentError::InvalidArgument)?;
                let n = str::parse::<u32>(&value).map_err(|_| ArgumentError::InvalidArgument)?;

                match conf.n {
                    None => {
                        conf.n = Some(n);
                        Ok(())
                    }
                    Some(_) => Err(ArgumentError::InvalidArgument),
                }
            }

            "k" => {
                let value = value.as_ref().ok_or(ArgumentError::InvalidArgument)?;
                let k = str::parse::<u32>(&value).map_err(|_| ArgumentError::InvalidArgument)?;

                match conf.k {
                    None => {
                        conf.k = Some(k);
                        Ok(())
                    }
                    Some(_) => Err(ArgumentError::InvalidArgument),
                }
            }

            "warmup" | "w" => {
                let value = value.as_ref().ok_or(ArgumentError::InvalidArgument)?;
                let warmup =
                    str::parse::<u16>(&value).map_err(|_| ArgumentError::InvalidArgument)?;

                match conf.warmup {
                    None => {
                        conf.iter = Some(warmup);
                        Ok(())
                    }
                    Some(_) => Err(ArgumentError::InvalidArgument),
                }
            }

            "iter" | "i" => {
                let value = value.as_ref().ok_or(ArgumentError::InvalidArgument)?;
                let iter = str::parse::<u16>(&value).map_err(|_| ArgumentError::InvalidArgument)?;

                match conf.iter {
                    None => {
                        conf.iter = Some(iter);
                        Ok(())
                    }
                    Some(_) => Err(ArgumentError::InvalidArgument),
                }
            }

            "verify" => {
                if value.is_none() {
                    Ok(())
                } else {
                    Err(ArgumentError::InvalidArgument)
                }?;

                match conf.verify {
                    None => {
                        conf.verify = Some(true);
                        Ok(())
                    }
                    Some(_) => Err(ArgumentError::InvalidArgument),
                }
            }

            "result-policy" => {
                let value = value.as_ref().ok_or(ArgumentError::InvalidArgument)?;
                let result_policy = match value.as_str() {
                    "min" => Ok(ResultPolicy::Min),
                    "avg" => Ok(ResultPolicy::Avg),
                    "max" => Ok(ResultPolicy::Max),
                    _ => Err(ArgumentError::InvalidArgument),
                }?;

                match conf.result_policy {
                    None => {
                        conf.result_policy = Some(result_policy);
                        Ok(())
                    }
                    Some(_) => Err(ArgumentError::InvalidArgument),
                }
            }

            "only-result" => {
                if value.is_none() {
                    Ok(())
                } else {
                    Err(ArgumentError::InvalidArgument)
                }?;

                match conf.only_result {
                    None => {
                        conf.only_result = Some(true);
                        Ok(())
                    }
                    Some(_) => Err(ArgumentError::InvalidArgument),
                }
            }

            _ => Err(ArgumentError::InvalidArgument),
        }?;
    }

    Ok(conf)
}

const REGEX_PATTERN: &'static str = r#"^--([a-z]+)(?:=([a-z0-9.]+))?$"#;

fn parse_option(arg: &str) -> Result<OptionString, ArgumentError> {
    let regex_module = Regex::new(REGEX_PATTERN).unwrap();
    let caps = regex_module.captures(arg).unwrap();

    if caps.get(1).is_none() || caps.get(3).is_some() {
        return Err(ArgumentError::InvalidArgument);
    }

    let name = String::from(caps.get(1).ok_or(ArgumentError::InvalidArgument)?.as_str());
    let value = caps.get(2).map(|x| String::from(x.as_str()));

    Ok(OptionString { name, value })
}

impl fmt::Display for ArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidArgument => write!(f, "invalid argument"),
            Self::NotEnoughArguments => write!(f, "not enough argument"),
        }
    }
}

impl std::error::Error for ArgumentError {}
