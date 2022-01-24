use serde::{Deserialize, Serialize};

use crate::common::PartialRecord;

/// Delete record(s)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecordDeleteRequest {
    records: Vec<PartialRecord>,
}

impl RecordDeleteRequest {
    #[inline]
    #[must_use]
    pub fn new(records: Vec<PartialRecord>) -> Self {
        Self { records }
    }
}

impl From<PartialRecord> for RecordDeleteRequest {
    #[inline]
    fn from(record: PartialRecord) -> Self {
        let records = vec![record];
        Self { records }
    }
}

impl From<Vec<PartialRecord>> for RecordDeleteRequest {
    #[inline]
    fn from(records: Vec<PartialRecord>) -> Self {
        Self { records }
    }
}
