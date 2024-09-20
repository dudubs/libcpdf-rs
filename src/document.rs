use core::slice;
use std::{
    collections::BTreeSet,
    ffi::{c_int, CStr},
    os::raw::c_void,
};

use crate::{
    bindings::*,
    core::{Result, ToChars},
    error::Error,
    from_id,
    range::Range,
    with_result,
};

#[derive(Debug)]
pub struct Document {
    pub id: i32,
}

impl Drop for Document {
    fn drop(&mut self) {
        with_result!(cpdf_deletePdf(self.id)).unwrap();
    }
}

impl Document {
    pub fn from_file(path: impl ToChars, password: impl ToChars) -> Result<Self> {
        from_id!(cpdf_fromFile(path.to_chars()?, password.to_chars()?,))
    }

    pub fn blank(num_pages: i32, width: f64, height: f64) -> Result<Self> {
        from_id!(cpdf_blankDocument(width, height, num_pages))
    }

    pub fn from_mem(data: Vec<u8>, password: impl ToChars) -> Result<Self> {
        from_id!(cpdf_fromMemory(
            data.as_ptr() as *mut c_void,
            data.len() as i32,
            password.to_chars()?,
        ))
    }

    pub fn decrypt(&self, password: impl ToChars) -> Result {
        with_result!(cpdf_decryptPdf(self.id, password.to_chars()?))
    }

    pub fn save_as(&self, path: impl ToChars) -> Result {
        with_result!(cpdf_toFile(self.id, path.to_chars()?, 0, 0)).map(|_| ())
    }

    pub fn add_text_simple(
        &self,
        range: &Range,
        text: impl ToChars,
        pos: CpdfPosition,
        font: CpdfFont,
        font_size: f64,
    ) -> Result {
        with_result!(cpdf_addTextSimple(
            //
            self.id,
            range.id,
            text.to_chars()?,
            pos,
            font,
            font_size,
        ))
    }
    pub fn to_vec(&self) -> Result<Vec<u8>> {
        let mut length: i32 = 0;

        let ptr = with_result!(cpdf_toMemory(self.id, 0, 0, &mut length as *mut c_int))?;

        let vec = unsafe { slice::from_raw_parts_mut(ptr as *mut _, length as usize) }.to_vec();

        with_result!(cpdf_free(ptr))?;
        Ok(vec)
    }

    pub fn num_pages(&self) -> Result<i32> {
        with_result!(cpdf_pages(self.id))
    }

    pub fn scale_pages(&self, range: &Range, scale_x: f64, scale_y: f64) -> Result<()> {
        with_result!(cpdf_scalePages(self.id, range.id, scale_x, scale_y))
    }

    pub fn scale_to_fit(&self, range: &Range, width: f64, height: f64, scale: f64) -> Result<()> {
        with_result!(cpdf_scaleToFit(self.id, range.id, width, height, scale))
    }

    pub fn select_pages(&self, range: &Range) -> Result<Self> {
        from_id!(cpdf_selectPages(self.id, range.id))
    }

    pub fn get_page_label_string(&self, page_num: i32) -> Result<String> {
        let label = with_result!(cpdf_getPageLabelStringForPage(self.id, page_num))?;

        if label.is_null() {
            return Ok("".to_string());
        }

        Ok(unsafe { CStr::from_ptr(label) }
            .to_str()
            .unwrap_or_default()
            .to_string())
    }

    pub fn add_page_labels(&self, range: &Range, prefix: impl ToChars) -> Result {
        with_result!(cpdf_addPageLabels(
            self.id,
            0,
            prefix.to_chars()?,
            0,
            range.id,
            0
        ))
    }

    pub fn move_pages(&self, after: i32, pages: impl IntoIterator<Item = i32>) -> Result<Document> {
        let mut pages_before = vec![];
        let mut pages_after = vec![];
        let pages = pages
            .into_iter()
            .filter(|&p| p != after)
            .collect::<BTreeSet<_>>();

        if pages.len() == 0 {
            return Err(Error::NoPagesToMove);
        }

        for page in self.pages()? {
            if pages.contains(&page) {
                continue;
            }

            if page > after {
                pages_after.push(page);
            } else {
                pages_before.push(page);
            }
        }

        let mut docs = vec![];
        for group in [
            pages_before,
            pages.into_iter().collect::<Vec<_>>(),
            pages_after,
        ] {
            if group.len() == 0 {
                continue;
            }
            docs.push(self.select_pages(&Range::from(&group)?)?);
        }

        let doc = Self::merge(&docs, true)?;

        Ok(doc)
    }

    pub fn pages(&self) -> Result<std::ops::Range<i32>> {
        Ok(1..self.num_pages()? + 1)
    }

    pub fn rotate_pages(&self, range: &Range, times: i32) -> Result {
        assert!(3 >= times);
        with_result!(cpdf_rotateBy(self.id, range.id, times * 90))
    }

    pub fn page_size(&self, page_num: i32) -> Result<(f64, f64)> {
        Self::get_page_size(self.page_rotation(page_num)?, self.media_size(page_num)?)
    }

    pub fn get_page_size(rotation: i32, media_size: (f64, f64)) -> Result<(f64, f64)> {
        let (w, h) = media_size;
        if rotation % 2 == 0 {
            Ok((w, h))
        } else {
            Ok((h, w))
        }
    }

    pub fn media_size(&self, page_num: i32) -> Result<(f64, f64)> {
        let b = self.media_box(page_num)?;
        Ok((b.max_x - b.min_x, b.max_y - b.min_y))
    }

    pub fn media_box(&self, page_num: i32) -> Result<Box> {
        let mut r#box = Box {
            min_x: 0.0,
            min_y: 0.0,
            max_x: 0.0,
            max_y: 0.0,
        };

        with_result!(cpdf_getMediaBox(
            self.id,
            page_num,
            &mut r#box.min_x as *mut f64,
            &mut r#box.max_x as *mut f64,
            &mut r#box.min_y as *mut f64,
            &mut r#box.max_y as *mut f64,
        ))?;

        Ok(r#box)
    }

    pub fn merge(files: &Vec<Document>, remove_duplicate_fonts: bool) -> Result<Document, Error> {
        let mut ids = files.iter().map(|file| file.id).collect::<Vec<_>>();
        from_id!(cpdf_merge(
            ids.as_mut_ptr(),
            ids.len() as i32,
            0,
            remove_duplicate_fonts as i32,
        ))
    }

    pub fn fit_to_width(&self, width: f64, max_deviation: f64) -> Result<bool> {
        let mut did = false;
        for page_num in 1..self.num_pages()? + 1 {
            let rotation = self.page_rotation(page_num)?;

            let (media_width, media_height) = self.media_size(page_num)?;
            let (page_width, _) = Document::get_page_size(rotation, (media_width, media_height))?;

            let deviation = (page_width - width).abs();

            if max_deviation >= deviation {
                continue;
            }

            did = true;

            let mut scale = width / media_width;

            if rotation % 2 != 0 {
                // 90deg or 270deg
                scale *= media_width / media_height;
            }

            self.scale_pages(&Range::only(page_num)?, scale, scale)?;
        }
        Ok(did)
    }

    // can returned 0,1,2,3
    pub fn page_rotation(&self, page_num: i32) -> Result<i32> {
        with_result!(cpdf_getPageRotation(self.id, page_num) / 90)
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
