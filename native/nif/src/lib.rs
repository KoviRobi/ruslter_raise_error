use std::env::var;
use std::error::Error;
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

impl<E: Error + 'static, C: ErrorContext> From<E> for MyError<C> {
    fn from(error: E) -> Self {
        let reference: &dyn Error = &error;
        let message = || {
            match reference.downcast_ref::<IoError>().map(IoError::kind) {
                Some(IoErrorKind::PermissionDenied) => {
                    return format!(
                        "Error in {}: permission denied: {}",
                        C::describe(),
                        reference
                    )
                }
                _ => (),
            }
            format!("Error in {}: {}", C::describe(), reference)
        };

        Self {
            message: message(),
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

init!("Elixir.NIF", [getenv, getenv_bang]);
