use std::ops::RangeBounds;

use crate::exid::ExId;
use crate::op_set::{OpSet, OpSetTree};
use crate::{Automerge, ChangeHash, KeysAt, ObjType, OpObserver, Prop, ScalarValue, Value, Values};
use crate::{AutomergeError, Keys};
use crate::{ListRange, ListRangeAt, MapRange, MapRangeAt};

use super::{CommitOptions, Transactable, TransactionInner};

/// A transaction on a document.
/// Transactions group operations into a single change so that no other operations can happen
/// in-between.
///
/// Created from [`Automerge::transaction`].
///
/// ## Drop
///
/// This transaction should be manually committed or rolled back. If not done manually then it will
/// be rolled back when it is dropped. This is to prevent the document being in an unsafe
/// intermediate state.
/// This is consistent with `?` error handling.
#[derive(Debug)]
pub struct Transaction<'t, T: OpSetTree<'t>> {
    // this is an option so that we can take it during commit and rollback to prevent it being
    // rolled back during drop.
    pub(crate) inner: Option<TransactionInner>,
    pub(crate) doc: Option<Automerge<T>>,
    pub(crate) _marker: std::marker::PhantomData<&'t ()>,
}

impl<'t, T> Transaction<'t, T>
where
    T: OpSetTree<'t>,
{
    /// Get the heads of the document before this transaction was started.
    pub fn get_heads(&self) -> Vec<ChangeHash> {
        self.doc.as_ref().unwrap().get_heads()
    }

    /// Commit the operations performed in this transaction, returning the hashes corresponding to
    /// the new heads.
    pub fn commit(mut self) -> (ChangeHash, Automerge<T>) {
        (
            self.inner.take().unwrap().commit::<(), T>(
                self.doc.as_mut().unwrap(),
                None,
                None,
                None,
            ),
            self.doc.take().unwrap(),
        )
    }

    /// Commit the operations in this transaction with some options.
    ///
    /// ```
    /// # use automerge::transaction::CommitOptions;
    /// # use automerge::transaction::Transactable;
    /// # use automerge::ROOT;
    /// # use automerge::Automerge;
    /// # use automerge::ObjType;
    /// # use std::time::SystemTime;
    /// let mut doc = Automerge::new();
    /// let mut tx = doc.transaction();
    /// tx.put_object(ROOT, "todos", ObjType::List).unwrap();
    /// let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as
    /// i64;
    /// tx.commit_with::<()>(CommitOptions::default().with_message("Create todos list").with_time(now));
    /// ```
    pub fn commit_with<Obs: OpObserver>(
        mut self,
        options: CommitOptions<'_, Obs>,
    ) -> (ChangeHash, Automerge<T>) {
        (
            self.inner.take().unwrap().commit(
                self.doc.as_mut().unwrap(),
                options.message,
                options.time,
                options.op_observer,
            ),
            self.doc.take().unwrap(),
        )
    }

    /// Undo the operations added in this transaction, returning the number of cancelled
    /// operations.
    pub fn rollback(mut self) -> (usize, Automerge<T>) {
        (
            self.inner
                .take()
                .unwrap()
                .rollback(self.doc.as_mut().unwrap()),
            self.doc.take().unwrap(),
        )
    }

    pub fn document(&self) -> &Automerge<T> {
        self.doc.as_ref().unwrap()
    }

    pub fn document_mut(&mut self) -> &mut Automerge<T> {
        self.doc.as_mut().unwrap()
    }
}

impl<'t, T> Transactable<'t> for Transaction<'t, T>
where
    T: OpSetTree<'t>,
{
    type Tree = T;
    /// Get the number of pending operations in this transaction.
    fn pending_ops(&self) -> usize {
        self.inner.as_ref().unwrap().pending_ops()
    }

    /// Set the value of property `P` to value `V` in object `obj`.
    ///
    /// # Returns
    ///
    /// The opid of the operation which was created, or None if this operation doesn't change the
    /// document
    ///
    /// # Errors
    ///
    /// This will return an error if
    /// - The object does not exist
    /// - The key is the wrong type for the object
    /// - The key does not exist in the object
    fn put<O: AsRef<ExId>, P: Into<Prop>, V: Into<ScalarValue>>(
        &mut self,
        obj: O,
        prop: P,
        value: V,
    ) -> Result<(), AutomergeError> {
        self.inner
            .as_mut()
            .unwrap()
            .put(self.doc.as_mut().unwrap(), obj.as_ref(), prop, value)
    }

    fn put_object<O: AsRef<ExId>, P: Into<Prop>>(
        &mut self,
        obj: O,
        prop: P,
        value: ObjType,
    ) -> Result<ExId, AutomergeError> {
        self.inner.as_mut().unwrap().put_object(
            self.doc.as_mut().unwrap(),
            obj.as_ref(),
            prop,
            value,
        )
    }

    fn insert<O: AsRef<ExId>, V: Into<ScalarValue>>(
        &mut self,
        obj: O,
        index: usize,
        value: V,
    ) -> Result<(), AutomergeError> {
        self.inner
            .as_mut()
            .unwrap()
            .insert(self.doc.as_mut().unwrap(), obj.as_ref(), index, value)
    }

    fn insert_object<O: AsRef<ExId>>(
        &mut self,
        obj: O,
        index: usize,
        value: ObjType,
    ) -> Result<ExId, AutomergeError> {
        self.inner.as_mut().unwrap().insert_object(
            self.doc.as_mut().unwrap(),
            obj.as_ref(),
            index,
            value,
        )
    }

    fn increment<O: AsRef<ExId>, P: Into<Prop>>(
        &mut self,
        obj: O,
        prop: P,
        value: i64,
    ) -> Result<(), AutomergeError> {
        self.inner.as_mut().unwrap().increment(
            self.doc.as_mut().unwrap(),
            obj.as_ref(),
            prop,
            value,
        )
    }

    fn delete<O: AsRef<ExId>, P: Into<Prop>>(
        &mut self,
        obj: O,
        prop: P,
    ) -> Result<(), AutomergeError> {
        self.inner
            .as_mut()
            .unwrap()
            .delete(self.doc.as_mut().unwrap(), obj.as_ref(), prop)
    }

    /// Splice new elements into the given sequence. Returns a vector of the OpIds used to insert
    /// the new elements
    fn splice<O: AsRef<ExId>, V: IntoIterator<Item = ScalarValue>>(
        &mut self,
        obj: O,
        pos: usize,
        del: usize,
        vals: V,
    ) -> Result<(), AutomergeError> {
        self.inner.as_mut().unwrap().splice(
            self.doc.as_mut().unwrap(),
            obj.as_ref(),
            pos,
            del,
            vals,
        )
    }

    fn keys<O: AsRef<ExId>>(&self, obj: O) -> Keys<'_, '_, T> {
        self.doc.as_ref().unwrap().keys(obj)
    }

    fn keys_at<O: AsRef<ExId>>(&self, obj: O, heads: &[ChangeHash]) -> KeysAt<'_, '_, T> {
        self.doc.as_ref().unwrap().keys_at(obj, heads)
    }

    fn map_range<O: AsRef<ExId>, R: RangeBounds<String>>(
        &self,
        obj: O,
        range: R,
    ) -> MapRange<'_, R, T> {
        self.doc.as_ref().unwrap().map_range(obj, range)
    }

    fn map_range_at<O: AsRef<ExId>, R: RangeBounds<String>>(
        &self,
        obj: O,
        range: R,
        heads: &[ChangeHash],
    ) -> MapRangeAt<'_, R, T> {
        self.doc.as_ref().unwrap().map_range_at(obj, range, heads)
    }

    fn list_range<O: AsRef<ExId>, R: RangeBounds<usize>>(
        &self,
        obj: O,
        range: R,
    ) -> ListRange<'_, R, T> {
        self.doc.as_ref().unwrap().list_range(obj, range)
    }

    fn list_range_at<O: AsRef<ExId>, R: RangeBounds<usize>>(
        &self,
        obj: O,
        range: R,
        heads: &[ChangeHash],
    ) -> ListRangeAt<'_, R, T> {
        self.doc.as_ref().unwrap().list_range_at(obj, range, heads)
    }

    fn values<O: AsRef<ExId>>(&self, obj: O) -> Values<'_, T> {
        self.doc.as_ref().unwrap().values(obj)
    }

    fn values_at<O: AsRef<ExId>>(&self, obj: O, heads: &[ChangeHash]) -> Values<'_, T> {
        self.doc.as_ref().unwrap().values_at(obj, heads)
    }

    fn length<O: AsRef<ExId>>(&self, obj: O) -> usize {
        self.doc.as_ref().unwrap().length(obj)
    }

    fn length_at<O: AsRef<ExId>>(&self, obj: O, heads: &[ChangeHash]) -> usize {
        self.doc.as_ref().unwrap().length_at(obj, heads)
    }

    fn object_type<O: AsRef<ExId>>(&self, obj: O) -> Option<ObjType> {
        self.doc.as_ref().unwrap().object_type(obj)
    }

    fn text<O: AsRef<ExId>>(&self, obj: O) -> Result<String, AutomergeError> {
        self.doc.as_ref().unwrap().text(obj)
    }

    fn text_at<O: AsRef<ExId>>(
        &self,
        obj: O,
        heads: &[ChangeHash],
    ) -> Result<String, AutomergeError> {
        self.doc.as_ref().unwrap().text_at(obj, heads)
    }

    fn get<O: AsRef<ExId>, P: Into<Prop>>(
        &self,
        obj: O,
        prop: P,
    ) -> Result<Option<(Value<'_>, ExId)>, AutomergeError> {
        self.doc.as_ref().unwrap().get(obj, prop)
    }

    fn get_at<O: AsRef<ExId>, P: Into<Prop>>(
        &self,
        obj: O,
        prop: P,
        heads: &[ChangeHash],
    ) -> Result<Option<(Value<'_>, ExId)>, AutomergeError> {
        self.doc.as_ref().unwrap().get_at(obj, prop, heads)
    }

    fn get_all<O: AsRef<ExId>, P: Into<Prop>>(
        &self,
        obj: O,
        prop: P,
    ) -> Result<Vec<(Value<'_>, ExId)>, AutomergeError> {
        self.doc.as_ref().unwrap().get_all(obj, prop)
    }

    fn get_all_at<O: AsRef<ExId>, P: Into<Prop>>(
        &self,
        obj: O,
        prop: P,
        heads: &[ChangeHash],
    ) -> Result<Vec<(Value<'_>, ExId)>, AutomergeError> {
        self.doc.as_ref().unwrap().get_all_at(obj, prop, heads)
    }

    fn parent_object<O: AsRef<ExId>>(&self, obj: O) -> Option<(ExId, Prop)> {
        self.doc.as_ref().unwrap().parent_object(obj)
    }

    fn parents(&self, obj: ExId) -> crate::Parents<'_, T> {
        self.doc.as_ref().unwrap().parents(obj)
    }
}

// If a transaction is not commited or rolled back manually then it can leave the document in an
// intermediate state.
// This defaults to rolling back the transaction to be compatible with `?` error returning before
// reaching a call to `commit`.
impl<'t, T> Drop for Transaction<'t, T>
where
    T: OpSetTree<'t>,
{
    fn drop(&mut self) {
        if let Some(txn) = self.inner.take() {
            if let Some(mut doc) = self.doc.take() {
                txn.rollback(&mut doc);
            }
        }
    }
}
