use std::env::var;
use std::error::Error;
use std::fs::File;
use std::io::{Error as IoError, ErrorKind as IoErrorKind, Write};

use rustler::{init, nif, Encoder};

pub struct MyError {
    message: String,
}

impl<E: Error + 'static> From<E> for MyError {
    fn from(error: E) -> Self {
        let reference: &dyn Error = &error;
        let message = || {
            match reference.downcast_ref::<IoError>().map(IoError::kind) {
                Some(IoErrorKind::PermissionDenied) => {
                    return format!("Error: permission denied: {}", reference)
                }
                _ => (),
            }
            format!("Error: {}", reference)
        };

        Self { message: message() }
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

init!("Elixir.NIF", [getenv, getenv_bang]);
