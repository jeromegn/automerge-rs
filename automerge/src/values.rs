use crate::exid::ExId;
use crate::{Automerge, Value};
use std::fmt;

pub struct Values<'a, T> {
    range: Box<dyn 'a + ValueIter<'a, T>>,
    doc: &'a Automerge<T>,
}

impl<'a, T> fmt::Debug for Values<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Values").finish()
    }
}

pub(crate) trait ValueIter<'a, T> {
    fn next_value(&mut self, doc: &'a Automerge<T>) -> Option<(Value<'a>, ExId)>;
}

pub(crate) struct NoValues {}

impl<'a, T> ValueIter<'a, T> for NoValues {
    fn next_value(&mut self, _doc: &'a Automerge<T>) -> Option<(Value<'a>, ExId)> {
        None
    }
}

impl<'a, T> Values<'a, T> {
    pub(crate) fn new<R: 'a + ValueIter<'a, T>>(doc: &'a Automerge<T>, range: Option<R>) -> Self {
        if let Some(range) = range {
            Self {
                range: Box::new(range),
                doc,
            }
        } else {
            Self::empty(doc)
        }
    }

    pub(crate) fn empty(doc: &'a Automerge<T>) -> Self {
        Self {
            range: Box::new(NoValues {}),
            doc,
        }
    }
}

impl<'a, T> Iterator for Values<'a, T> {
    type Item = (Value<'a>, ExId);

    fn next(&mut self) -> Option<Self::Item> {
        self.range.next_value(self.doc)
    }
}

impl<'a, T> DoubleEndedIterator for Values<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}
