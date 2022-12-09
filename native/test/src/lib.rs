use std::env::{var, VarError};
use std::fs::File;
use std::io::{Error as IoError, ErrorKind as IoErrorKind, Write};

use rustler::{init, nif, Encoder};

pub struct MyError {
    message: String,
}

impl From<VarError> for MyError {
    fn from(error: VarError) -> Self {
        Self {
            message: match error {
                VarError::NotPresent => format!("Variable not set: {}", error),
                _ => format!("Error: {}", error),
            },
        }
    }
}

// If specialisation (RFC 1210) is implemented we can do a generic
// implementation for any error
impl From<IoError> for MyError {
    fn from(error: IoError) -> Self {
        Self {
            message: match error.kind() {
                IoErrorKind::PermissionDenied => format!("Permission denied: {}", error),
                _ => format!("Error: {}", error),
            },
        }
    }
}

impl Encoder for MyError {
    fn encode<'a>(&self, env: rustler::Env<'a>) -> rustler::Term<'a> {
        self.message.encode(env)
    }
}

impl From<MyError> for rustler::Error {
    fn from(my_error: MyError) -> Self {
        rustler::Error::RaiseTerm(Box::new(my_error.message))
    }
}

fn _getenv(key: String) -> Result<String, MyError> {
    let val = var(&key)?;
    let mut info_file = File::create("var_dump.txt")?;
    write!(
        info_file,
        "Environment variable {:?} has value {:?}",
        key, val
    )?;
    println!("Environment variable {:?} has value {:?}", key, val);
    Ok(val)
}

#[nif]
pub fn getenv(key: String) -> Result<String, MyError> {
    _getenv(key)
}

#[nif(name = "getenv!")]
pub fn getenv_bang(key: String) -> rustler::NifResult<String> {
    Ok(_getenv(key)?)
}

init!("Elixir.Test", [getenv, getenv_bang]);
