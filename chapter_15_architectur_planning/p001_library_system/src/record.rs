// Record.rs

use std::io::Result;

use chrono::{DateTime, Duration, Utc};
use uuid::{self, Uuid};

use crate::book::BookId;
use crate::errors;
use crate::member::MemberId;

/// RecordID: unique id for records.
pub type RecordID = Uuid;

#[derive(Debug, Clone, Copy)]
/// Borroing records will be here.
pub struct BorrowingRecord {
    pub record_id: RecordID,
    pub mem_id: MemberId,
    pub boo_id: BookId,
    pub borrowed_at: DateTime<Utc>,
    pub due_date: DateTime<Utc>,
    pub return_at: Option<DateTime<Utc>>,
}

impl BorrowingRecord {
    pub fn new(mem_id: MemberId, boo_id: BookId) -> BorrowingRecord {
        BorrowingRecord {
            record_id: RecordID::new_v4(),
            mem_id,
            boo_id,
            borrowed_at: chrono::Utc::now(),
            due_date: chrono::Utc::now() + chrono::Duration::days(14),
            return_at: None,
        }
    }
}
