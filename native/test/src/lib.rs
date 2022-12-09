use std::fs::File;
use std::io::{Error as IoError, ErrorKind as IoErrorKind, Write};

use libftd2xx::{FtStatus, Ftdi, FtdiCommon};
use rustler::{init, nif, Encoder};

trait ErrorContext {
    fn describe<'a>() -> &'a str;
}
pub struct MyError<Context> {
    message: String,
    phantom: std::marker::PhantomData<Context>,
}

impl<C: ErrorContext> From<FtStatus> for MyError<C> {
    fn from(error: FtStatus) -> Self {
        Self {
            message: match error {
                FtStatus::DEVICE_NOT_FOUND => {
                    format!("Error {}: FTDI device not found", C::describe())
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

pub struct OpenContext;
impl ErrorContext for OpenContext {
    fn describe<'a>() -> &'a str {
        "opening the FTDI device"
    }
}

fn _open() -> Result<(), MyError<OpenContext>> {
    let mut device = Ftdi::new()?;
    let info = device.device_info()?;
    let mut info_file = File::create("device_info.txt")?;
    write!(info_file, "Device information: {:?}", info)?;
    println!("Device information: {:?}", info);
    Ok(())
}

#[nif]
pub fn open(_args: rustler::Term) -> Result<(), MyError<OpenContext>> {
    _open()
}

#[nif]
pub fn open_bang(_args: rustler::Term) -> rustler::NifResult<()> {
    Ok(_open()?)
}

init!("Elixir.Test", [open, open_bang]);
