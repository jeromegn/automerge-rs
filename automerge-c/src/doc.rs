use am::InMemoryTree;
use automerge as am;
use std::ops::{Deref, DerefMut};

/// \struct AMdoc
/// \brief A JSON-like CRDT.
#[derive(Clone)]
pub struct AMdoc(am::AutoCommit<InMemoryTree>);

impl AMdoc {
    pub fn new(body: am::AutoCommit<InMemoryTree>) -> Self {
        Self(body)
    }
}

impl Deref for AMdoc {
    type Target = am::AutoCommit<InMemoryTree>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AMdoc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<AMdoc> for *mut AMdoc {
    fn from(b: AMdoc) -> Self {
        Box::into_raw(Box::new(b))
    }
}
