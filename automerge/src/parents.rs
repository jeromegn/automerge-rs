use crate::{exid::ExId, op_set::OpSetTree, Automerge, Prop};

#[derive(Debug)]
pub struct Parents<'a, T> {
    pub(crate) obj: ExId,
    pub(crate) doc: &'a Automerge<T>,
}

impl<'a, 't, T> Iterator for Parents<'a, T>
where
    T: OpSetTree<'t>,
{
    type Item = (ExId, Prop);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((obj, prop)) = self.doc.parent_object(&self.obj) {
            self.obj = obj.clone();
            Some((obj, prop))
        } else {
            None
        }
    }
}
