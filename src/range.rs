use crate::{
    bindings::{
        cpdf_blankRange, cpdf_deleteRange, cpdf_even, cpdf_isInRange, cpdf_range, cpdf_rangeUnion,
    },
    core::{with_result, Result},
    error::Error,
};

pub struct Range {
    pub id: i32,
}

impl Range {
    pub fn blank() -> Result<Range> {
        Self::_new(|| unsafe { cpdf_blankRange() })
    }
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

    pub fn has(&self, page: i32) -> Result<bool> {
        with_result(|| Ok(unsafe { cpdf_isInRange(self.id, page) > 0 }))
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
