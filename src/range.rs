use crate::{
    bindings::{
        cpdf_blankRange, cpdf_deleteRange, cpdf_even, cpdf_isInRange, cpdf_range, cpdf_rangeAdd,
        cpdf_rangeUnion,
    },
    core::{with_result, Result},
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

    pub fn add(&self, page: i32) -> Result<Self> {
        Self::_new(|| (unsafe { cpdf_rangeAdd(self.id, page) }))
    }

    pub fn from(pages: &Vec<i32>) -> Result<Self> {
        let mut range = Self::blank()?;
        for page in pages {
            range = range.add(page.clone())?;
        }
        Ok(range)
    }

    pub fn merge(self, other: Self) -> Result<Self> {
        Self::_new(|| unsafe { cpdf_rangeUnion(self.id, other.id) })
    }

    pub fn even(&self) -> Result<Self> {
        Self::_new(|| unsafe { cpdf_even(self.id) })
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
        with_result(|| unsafe { Ok(cpdf_deleteRange(self.id)) }).unwrap();
    }
}
