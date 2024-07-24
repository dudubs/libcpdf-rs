use std::{
    ffi::{CStr, CString},
    path::{Path, PathBuf},
    sync::{Arc, Mutex, RwLock},
};

use crate::{
    bindings::{
        cpdf_clearError, cpdf_fLastError, cpdf_fLastErrorString, cpdf_setFast, cpdf_setSlow,
        cpdf_startup, cpdf_version,
    },
    error::Error,
};

fn with_mutex<T>(cb: impl FnOnce() -> T) -> T {
    static M: Mutex<()> = Mutex::new(());
    let _mg = M.lock().unwrap();
    cb()
}

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

#[macro_export]
macro_rules! with_result {
    ($e:expr) => {
        $crate::core::__with_result(|| {
            #[allow(unused_unsafe)]
            Ok(unsafe { $e })
        })
    };
}

#[macro_export]
macro_rules! from_id {
    ($e:expr) => {{
        Ok(Self {
            id: with_result!($e)?,
        })
    }};
}
pub fn __with_result<T>(f: impl FnOnce() -> Result<T>) -> Result<T> {
    unsafe fn last_error<'a>() -> Option<&'a str> {
        if cpdf_fLastError() == 0 {
            return None;
        }
        let msg = CStr::from_ptr(cpdf_fLastErrorString()).to_str().ok();

        cpdf_clearError();
        return msg;
    }

    with_mutex(|| {
        if unsafe { last_error().is_some() } {
            panic!("Did you cleaned error before?");
        }

        let val = {
            match f() {
                Ok(val) => val,
                Err(err) => return Err(err),
            }
        };

        match unsafe { last_error() } {
            Some(err) => Err(Error::Message(err.to_string())),
            None => Ok(val),
        }
    })
}

pub fn to_const_char(s: impl AsRef<str>) -> Result<*const i8> {
    Ok(CString::new(s.as_ref())
        .map_err(Error::NulError)?
        .into_raw())
}

pub trait ToChars {
    fn to_chars(&self) -> Result<*const i8>;
}

impl ToChars for &str {
    fn to_chars(&self) -> Result<*const i8> {
        Ok(CString::new(*self)?.into_raw())
    }
}

impl ToChars for String {
    fn to_chars(&self) -> Result<*const i8> {
        self.as_str().to_chars()
    }
}

impl ToChars for &PathBuf {
    fn to_chars(&self) -> Result<*const i8> {
        self.as_path().to_chars()
    }
}

impl ToChars for Path {
    fn to_chars(&self) -> Result<*const i8> {
        let s = self
            .to_str()
            .ok_or_else(|| Error::Message("Can't convert to string".to_string()))?
            .to_string();

        Ok(CString::new(s)?.into_raw())
    }
}

pub fn set_fast() -> Result<()> {
    with_result!(cpdf_setFast())
}

pub fn set_slow() -> Result<()> {
    with_result!(cpdf_setSlow())
}

pub fn version<'a>() -> Result<&'a str> {
    unsafe { CStr::from_ptr(with_result!(cpdf_version())?) }
        .to_str()
        .map_err(Error::Utf8Error)
}

pub fn startup() -> Result {
    with_result!(cpdf_startup([std::ptr::null()].as_ptr()))
}
