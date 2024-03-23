use core::slice;
use std::{ffi::c_int, os::raw::c_void};

use crate::{
    bindings::*,
    core::{last_result, with_result, Result, ToChars},
    error::Error,
    range::Range,
};

#[derive(Debug)]
pub struct Document {
    pub id: i32,
}

impl Drop for Document {
    fn drop(&mut self) {
        unsafe { cpdf_deletePdf(self.id) };
    }
}

impl Document {
    pub fn from_file(path: impl ToChars, password: impl ToChars) -> Result<Document> {
        Self::_from_id(unsafe {
            cpdf_fromFile(
                //
                path.to_chars()?,
                password.to_chars()?,
            )
        })
    }

    pub fn from_mem(data: Vec<u8>, password: impl ToChars) -> Result<Document> {
        Self::_from_id(unsafe {
            cpdf_fromMemory(
                data.as_ptr() as *mut c_void,
                data.len() as i32,
                password.to_chars()?,
            )
        })
    }

    fn _from_id(id: i32) -> Result<Document> {
        last_result().map(|_| Document { id })
    }

    pub fn decrypt(&self, password: impl ToChars) -> Result {
        with_result(|| unsafe { Ok(cpdf_decryptPdf(self.id, password.to_chars()?)) })
    }

    pub fn save_as(&self, path: impl ToChars) -> Result {
        with_result(|| Ok(unsafe { cpdf_toFile(self.id, path.to_chars()?, 0, 0) })).map(|_| ())
    }

    pub fn to_vec(&self) -> Result<Vec<u8>> {
        let mut length: i32 = 0;

        let ptr =
            with_result(|| Ok(unsafe { cpdf_toMemory(self.id, 0, 0, &mut length as *mut c_int) }))?;

        let vec = unsafe { slice::from_raw_parts_mut(ptr as *mut _, length as usize) }.to_vec();

        unsafe { cpdf_free(ptr) };
        Ok(vec)
    }

    pub fn num_pages(&self) -> Result<i32> {
        with_result(|| Ok(unsafe { cpdf_pages(self.id) }))
    }

    pub fn scale_pages(&self, range: &Range, scale_x: f64, scale_y: f64) -> Result<()> {
        with_result(|| Ok(unsafe { cpdf_scalePages(self.id, range.id, scale_x, scale_y) }))
    }

    pub fn select_pages(&self, range: &Range) -> Result<Document, Error> {
        Self::_from_id(unsafe { cpdf_selectPages(self.id, range.id) })
    }

    pub fn rotate_pages(&self, range: &Range, times: i32) -> Result {
        with_result(|| Ok(unsafe { cpdf_rotateBy(self.id, range.id, times * 90) }))
    }

    pub fn page_size(&self, page_num: i32) -> Result<(f64, f64)> {
        let media_box = self.media_box(page_num)?;
        let rotation = self.get_page_rotation(page_num)?;

        let mut size = (media_box.width(), media_box.height());

        if rotation == 1 || rotation == 3 {
            size = (size.1, size.0)
        }

        Ok(size)
    }

    pub fn media_box(&self, page_num: i32) -> Result<Box> {
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

    pub fn merge(files: &Vec<Document>, remove_duplicate_fonts: bool) -> Result<Document, Error> {
        let mut ids = files.iter().map(|file| file.id).collect::<Vec<_>>();

        with_result(|| {
            Ok(unsafe {
                Document {
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

    pub fn fit_to_width(&self, width: f64, max_deviation: f64) -> Result<bool> {
        let mut did = false;
        for page_num in 1..self.num_pages()? + 1 {
            let page_width = self.page_size(page_num)?.0;
            let deviation = (page_width - width).abs();
            if max_deviation > deviation {
                continue;
            }
            did = true;
            let scale = width / page_width;
            self.scale_pages(&Range::only(page_num)?, scale, scale)?;
        }
        Ok(did)
    }

    pub fn get_page_rotation(&self, page_num: i32) -> Result<i32> {
        with_result(|| Ok(unsafe { cpdf_getPageRotation(self.id, page_num) } / 90))
    }
}

#[derive(Debug, PartialEq)]
pub struct Box {
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
}
impl Box {
    pub fn width(&self) -> f64 {
        self.max_x - self.min_x
    }

    pub fn height(&self) -> f64 {
        self.max_y - self.min_y
    }
}
