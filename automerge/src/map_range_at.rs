use crate::{exid::ExId, Value};
use std::ops::RangeBounds;

use crate::{query, Automerge};

#[derive(Debug)]
pub struct MapRangeAt<'a, R: RangeBounds<String>, T> {
    range: Option<query::MapRangeAt<'a, R>>,
    doc: &'a Automerge<T>,
}

impl<'a, R: RangeBounds<String>, T> MapRangeAt<'a, R, T> {
    pub(crate) fn new(doc: &'a Automerge<T>, range: Option<query::MapRangeAt<'a, R>>) -> Self {
        Self { range, doc }
    }
}

impl<'a, R: RangeBounds<String>, T> Iterator for MapRangeAt<'a, R, T> {
    type Item = (&'a str, Value<'a>, ExId);

    fn next(&mut self) -> Option<Self::Item> {
        self.range
            .as_mut()?
            .next()
            .map(|(key, value, id)| (key, value, self.doc.id_to_exid(id)))
    }
}

impl<'a, R: RangeBounds<String>, T> DoubleEndedIterator for MapRangeAt<'a, R, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.range
            .as_mut()?
            .next_back()
            .map(|(key, value, id)| (key, value, self.doc.id_to_exid(id)))
    }
}
