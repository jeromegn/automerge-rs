use crate::{exid::ExId, Value};

use crate::{query, Automerge};
use std::ops::RangeBounds;

#[derive(Debug)]
pub struct ListRange<'a, R: RangeBounds<usize>, T> {
    range: Option<query::ListRange<'a, R>>,
    doc: &'a Automerge<T>,
}

impl<'a, R: RangeBounds<usize>, T> ListRange<'a, R, T> {
    pub(crate) fn new(doc: &'a Automerge<T>, range: Option<query::ListRange<'a, R>>) -> Self {
        Self { range, doc }
    }
}

impl<'a, R: RangeBounds<usize>, T> Iterator for ListRange<'a, R, T> {
    type Item = (usize, Value<'a>, ExId);

    fn next(&mut self) -> Option<Self::Item> {
        self.range
            .as_mut()?
            .next()
            .map(|(idx, value, id)| (idx, value, self.doc.id_to_exid(id)))
    }
}
