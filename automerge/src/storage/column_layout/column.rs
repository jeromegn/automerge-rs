use std::ops::Range;

use crate::columnar_2::column_range::generic::GenericColumnRange;

use super::{ColumnId, ColumnSpec, ColumnType};

/// A "logical" column, which is to say a column that produces a single value. A "logical" column
/// can be composed of multiple primtiive columns, access to these individual columns is via the
/// `range` function.
#[derive(Clone, Debug)]
pub(crate) struct Column {
    spec: ColumnSpec,
    range: GenericColumnRange,
}

impl Column {
    pub(crate) fn new(spec: ColumnSpec, range: GenericColumnRange) -> Column {
        Self { spec, range }
    }
}

impl Column {
    pub(crate) fn range(&self) -> Range<usize> {
        self.range.range()
    }

    pub(crate) fn into_ranges(self) -> GenericColumnRange {
        self.range
    }

    pub(crate) fn col_type(&self) -> ColumnType {
        self.spec.col_type()
    }

    pub(crate) fn id(&self) -> ColumnId {
        self.spec.id()
    }

    pub(crate) fn spec(&self) -> ColumnSpec {
        self.spec
    }
}
