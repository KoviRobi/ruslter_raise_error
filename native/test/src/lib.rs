use std::fs::File;
use std::io::{Error as IoError, ErrorKind as IoErrorKind, Write};

use libftd2xx::{FtStatus, Ftdi, FtdiCommon};
use rustler::{init, nif, Encoder};

pub struct MyError {
    message: String,
}

impl From<FtStatus> for MyError {
    fn from(error: FtStatus) -> Self {
        Self {
            message: match error {
                FtStatus::DEVICE_NOT_FOUND => "FTDI device not found".into(),
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

fn _open() -> Result<(), MyError> {
    let mut device = Ftdi::new()?;
    let info = device.device_info()?;
    let mut info_file = File::create("device_info.txt")?;
    write!(info_file, "Device information: {:?}", info)?;
    println!("Device information: {:?}", info);
    Ok(())
}

#[nif]
pub fn open(_args: rustler::Term) -> Result<(), MyError> {
    _open()
}

#[nif]
pub fn open_bang(_args: rustler::Term) -> rustler::NifResult<()> {
    Ok(_open()?)
}

init!("Elixir.Test", [open, open_bang]);
