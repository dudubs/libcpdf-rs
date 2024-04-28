use crate::{
    bindings::{
        cpdf_all, cpdf_blankRange, cpdf_deleteRange, cpdf_difference, cpdf_even, cpdf_isInRange,
        cpdf_range, cpdf_rangeAdd, cpdf_rangeUnion,
    },
    core::Result,
    from_id, with_result, Document,
};

pub struct Range {
    pub id: i32,
}

impl Range {
    pub fn blank() -> Result<Range> {
        from_id!(cpdf_blankRange())
    }

    pub fn between(start: i32, end: i32) -> Result<Self> {
        from_id!(cpdf_range(start, end))
    }

    pub fn only(page: i32) -> Result<Self> {
        from_id!(cpdf_range(page, page))
    }

    pub fn add(&self, page: i32) -> Result<Self> {
        from_id!(cpdf_rangeAdd(self.id, page))
    }
    pub fn all(doc: &Document) -> Result<Self> {
        from_id!(cpdf_all(doc.id))
    }

    pub fn exclude(&self, other: &Range) -> Result<Self> {
        from_id!(cpdf_difference(self.id, other.id))
    }

    pub fn from(pages: &Vec<i32>) -> Result<Self> {
        let mut range = Self::blank()?;
        for page in pages {
            range = range.add(page.clone())?;
        }
        Ok(range)
    }

    pub fn merge(self, other: Self) -> Result<Self> {
        from_id!(cpdf_rangeUnion(self.id, other.id))
    }

    pub fn even(&self) -> Result<Self> {
        from_id!(cpdf_even(self.id))
    }

    pub fn has(&self, page: i32) -> Result<bool> {
        with_result!(cpdf_isInRange(self.id, page) > 0)
    }
}

impl Drop for Range {
    fn drop(&mut self) {
        with_result!(cpdf_deleteRange(self.id)).unwrap();
    }
}
