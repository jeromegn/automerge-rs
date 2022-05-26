use crate::{op_set::OpSetTree, query, Automerge};

#[derive(Debug)]
pub struct Keys<'a, 'k, T> {
    keys: Option<query::Keys<'k>>,
    doc: &'a Automerge<T>,
}

impl<'a, 'k, T> Keys<'a, 'k, T> {
    pub(crate) fn new(doc: &'a Automerge<T>, keys: Option<query::Keys<'k>>) -> Self {
        Self { keys, doc }
    }
}

impl<'a, 'k, 't, T> Iterator for Keys<'a, 'k, T>
where
    T: OpSetTree<'t>,
{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.keys
            .as_mut()?
            .next()
            .map(|key| self.doc.to_string(key))
    }
}

impl<'a, 'k, 't, T> DoubleEndedIterator for Keys<'a, 'k, T>
where
    T: OpSetTree<'a>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.keys
            .as_mut()?
            .next_back()
            .map(|key| self.doc.to_string(key))
    }
}
