use std::{
    ffi::{CStr, CString, NulError},
    os::raw::{c_int, c_void},
    path::{Path, PathBuf},
    ptr::null,
};

#[derive(Debug)]
pub enum Error {
    Utf8Error(std::str::Utf8Error),
    NulError(NulError),
    Message(String),
}

impl ToString for Error {
    fn to_string(&self) -> String {
        format!("{:?}", &self)
    }
}
pub type Result<T, E = Error> = std::result::Result<T, E>;
use crate::bindings::*;

pub fn startup() {
    unsafe {
        cpdf_startup([std::ptr::null()].as_ptr());
    };
}

pub fn version<'a>() -> Result<&'a str> {
    unsafe { CStr::from_ptr(cpdf_version()) }
        .to_str()
        .map_err(Error::Utf8Error)
}

pub fn set_fast() {
    unsafe { cpdf_setFast() }
}

pub fn set_slow() {
    unsafe { cpdf_setSlow() }
}

pub fn last_error<'a>() -> Option<&'a str> {
    if unsafe { cpdf_fLastError() } != 0 {
        let msg = unsafe { CStr::from_ptr(cpdf_fLastErrorString()) }
            .to_str()
            .ok();

        unsafe { cpdf_clearError() };
        return msg;
    }
    None
}

pub fn with_result<T>(f: impl FnOnce() -> Result<T>) -> Result<T> {
    unsafe { cpdf_clearError() };
    let val = match f() {
        Ok(val) => val,
        Err(err) => return Err(err),
    };

    match last_error() {
        Some(err) => Err(Error::Message(err.to_string())),
        None => Ok(val),
    }
}

#[derive(Debug)]
pub struct File {
    pub id: i32,
}

pub struct Range {
    pub id: i32,
}

impl Range {
    pub fn between(start: i32, end: i32) -> Result<Self> {
        Self::_new(|| unsafe { cpdf_range(start, end) })
    }
    pub fn only(page: i32) -> Result<Self> {
        Self::_new(|| unsafe { cpdf_range(page, page) })
    }

    pub fn from(pages: &Vec<i32>) -> Result<Self> {
        pages
            .iter()
            .map(|page| Self::only(*page))
            .reduce(|a, b| a.and_then(|a| a.merge(b?)))
            .ok_or_else(|| Error::Message("No pages".to_string()))?
    }

    pub fn merge(self, other: Self) -> Result<Self> {
        Self::_new(|| unsafe { cpdf_rangeUnion(self.id, other.id) })
    }

    pub fn even(page: i32) -> Result<Self> {
        Self::_new(|| unsafe { cpdf_even(page) })
    }

    fn _new(f: impl FnOnce() -> i32) -> Result<Self> {
        with_result(|| Ok(Self { id: f() }))
    }
}

impl Drop for Range {
    fn drop(&mut self) {
        unsafe { cpdf_deleteRange(self.id) };
    }
}
impl Drop for File {
    fn drop(&mut self) {
        unsafe { cpdf_removeId(self.id) };
    }
}
impl File {
    pub fn decrypt(&self, password: impl AsRef<str>) -> Result<(), Error> {
        with_result(|| unsafe { Ok(cpdf_decryptPdf(self.id, to_const_char(password)?)) })
    }

    pub fn save_as(&self, path: impl AsRef<str>) -> Result<(), Error> {
        with_result(|| Ok(unsafe { cpdf_toFile(self.id, to_const_char(path)?, 0, 0) })).map(|_| ())
    }

    pub fn from_file(path: impl AsRef<str>, password: impl AsRef<str>) -> Result<File> {
        with_result(|| {
            Ok(File {
                id: unsafe {
                    cpdf_fromFile(
                        //
                        to_const_char(path)?,
                        to_const_char(password)?,
                    )
                },
            })
        })
    }

    pub fn from_mem(data: Vec<u8>, password: impl AsRef<str>) -> Result<File> {
        with_result(|| {
            Ok(File {
                id: unsafe {
                    cpdf_fromMemory(
                        data.as_ptr() as *mut c_void,
                        data.len() as i32,
                        to_const_char(password)?,
                    )
                },
            })
        })
    }

    pub fn num_pages(&self) -> Result<i32> {
        with_result(|| Ok(unsafe { cpdf_pages(self.id) }))
    }

    pub fn scale_pages(&self, range: Range, scale_x: f64, scale_y: f64) -> Result<()> {
        with_result(|| Ok(unsafe { cpdf_scalePages(self.id, range.id, scale_x, scale_y) }))
    }

    // TODO Check what happen if select pages is failed
    pub fn select_pages(&self, range: Range) -> Result<File, Error> {
        with_result(|| {
            Ok(unsafe {
                File {
                    id: cpdf_selectPages(self.id, range.id),
                }
            })
        })
    }

    pub fn get_media_box(&self, page_num: i32) -> Result<Box> {
        let mut r#box = Box {
            min_x: 0.0,
            min_y: 0.0,
            max_x: 0.0,
            max_y: 0.0,
        };

        with_result(|| {
            Ok(unsafe {
                cpdf_getMediaBox(
                    self.id,
                    page_num,
                    &mut r#box.min_x as *mut f64,
                    &mut r#box.max_x as *mut f64,
                    &mut r#box.min_y as *mut f64,
                    &mut r#box.max_y as *mut f64,
                )
            })
        })?;

        Ok(r#box)
    }

    pub fn merge(files: &Vec<File>, remove_duplicate_fonts: bool) -> Result<File, Error> {
        let mut ids = files.iter().map(|file| file.id).collect::<Vec<_>>();

        with_result(|| {
            Ok(unsafe {
                File {
                    id: cpdf_merge(
                        ids.as_mut_ptr(),
                        ids.len() as i32,
                        0,
                        remove_duplicate_fonts as i32,
                    ),
                }
            })
        })
    }

    pub fn fit_to_width(&self, width: f64, deviation: f64) -> Result<bool> {
        Ok(false)
    }
}

pub fn to_const_char(s: impl AsRef<str>) -> Result<*const i8> {
    Ok(CString::new(s.as_ref())
        .map_err(Error::NulError)?
        .into_raw())
}

#[derive(Debug)]
pub struct Box {
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
}
impl Box {
    pub fn width(self) -> f64 {
        self.max_x - self.min_x
    }

    pub fn height(self) -> f64 {
        self.max_y - self.min_y
    }
}
