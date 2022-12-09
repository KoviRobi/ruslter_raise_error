use std::env::{var, VarError};
use std::fs::File;
use std::io::{Error as IoError, ErrorKind as IoErrorKind, Write};

use rustler::{init, nif, Encoder};

trait ErrorContext {
    fn describe<'a>() -> &'a str;
}
pub struct MyError<Context> {
    message: String,
    phantom: std::marker::PhantomData<Context>,
}

impl<C: ErrorContext> From<VarError> for MyError<C> {
    fn from(error: VarError) -> Self {
        Self {
            message: match error {
                VarError::NotPresent => {
                    format!("Error {}: variable not set: {}", C::describe(), error)
                }
                _ => format!("Error: {}", error),
            },
            phantom: std::marker::PhantomData::<C>,
        }
    }
}

// If specialisation (RFC 1210) is implemented we can do a generic
// implementation for any error
impl<C: ErrorContext> From<IoError> for MyError<C> {
    fn from(error: IoError) -> Self {
        Self {
            message: match error.kind() {
                IoErrorKind::PermissionDenied => {
                    format!("Error in {}: permission denied: {}", C::describe(), error)
                }
                _ => format!("Error: {}", error),
            },
            phantom: std::marker::PhantomData::<C>,
        }
    }
}

impl<T> Encoder for MyError<T> {
    fn encode<'a>(&self, env: rustler::Env<'a>) -> rustler::Term<'a> {
        self.message.encode(env)
    }
}

impl<T> From<MyError<T>> for rustler::Error {
    fn from(my_error: MyError<T>) -> Self {
        rustler::Error::RaiseTerm(Box::new(my_error.message))
    }
}

pub struct GetenvContext;
impl ErrorContext for GetenvContext {
    fn describe<'a>() -> &'a str {
        "getting the environment variable"
    }
}

fn _getenv(key: String) -> Result<String, MyError<GetenvContext>> {
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
pub fn getenv(key: String) -> Result<String, MyError<GetenvContext>> {
    _getenv(key)
}

#[nif(name = "getenv!")]
pub fn getenv_bang(key: String) -> rustler::NifResult<String> {
    Ok(_getenv(key)?)
}

init!("Elixir.Test", [getenv, getenv_bang]);
