use crate::{exid::ExId, op_set::OpSetTree, Value};
use std::ops::RangeBounds;

use crate::{query, Automerge};

#[derive(Debug)]
pub struct ListRangeAt<'a, R: RangeBounds<usize>, T> {
    range: Option<query::ListRangeAt<'a, R>>,
    doc: &'a Automerge<T>,
}

impl<'a, R: RangeBounds<usize>, T> ListRangeAt<'a, R, T> {
    pub(crate) fn new(doc: &'a Automerge<T>, range: Option<query::ListRangeAt<'a, R>>) -> Self {
        Self { range, doc }
    }
}

impl<'a, R: RangeBounds<usize>, T> Iterator for ListRangeAt<'a, R, T>
where
    T: OpSetTree<'a>,
{
    type Item = (usize, Value<'a>, ExId);

    fn next(&mut self) -> Option<Self::Item> {
        self.range
            .as_mut()?
            .next()
            .map(|(key, value, id)| (key, value, self.doc.id_to_exid(id)))
    }
}
