// library.rs

use core::fmt;
use std::fmt::write;

use crate::book::*;
use crate::errors;
use crate::errors::*;
use crate::member::*;
use crate::record::*;

#[derive(Debug, Clone)]
struct Library {
    pub books: Vec<Book>,
    pub members: Vec<Member>,
    pub records: Vec<BorrowingRecord>,
}

impl Library {
    pub fn get_book(&mut self, boo_id: BookId) -> errors::Result<&mut Book> {
        self.books.iter_mut().find(|b| b.book_id == boo_id).ok_or(errors::LibErrors::NotFound)
    }

    pub fn get_record_ref(&self, recordid: Option<RecordID>, boo_id: Option<BookId>) -> errors::Result<&BorrowingRecord> {
        if recordid.is_some() || boo_id.is_none() {
            self.records.iter().find(|r| r.record_id == recordid.unwrap()).ok_or(LibErrors::NotFound)
        } else if recordid.is_none() || boo_id.is_some() {
            self.records.iter().find(|b| b.boo_id == boo_id.unwrap()).ok_or(LibErrors::NotFound)
        } else {
            Err(LibErrors::InvalidResponse {
                messgage: "Please inter either RecordID or Book ID".to_string(),
                expected: Some("RecordID / BookId".to_string()),
                found: Some("NONE/NONE".to_string()),
            })
        }
    }

    pub fn get_record(&mut self, recordid: Option<RecordID>, boo_id: Option<BookId>) -> errors::Result<&mut BorrowingRecord> {
        if recordid.is_some() || boo_id.is_none() {
            self.records.iter_mut().find(|r| r.record_id == recordid.unwrap()).ok_or(LibErrors::NotFound)
        } else if recordid.is_none() || boo_id.is_some() {
            self.records.iter_mut().find(|b| b.boo_id == boo_id.unwrap()).ok_or(LibErrors::NotFound)
        } else {
            Err(LibErrors::InvalidResponse {
                messgage: "Please inter either RecordID or Book ID".to_string(),
                expected: Some("RecordID / BookId".to_string()),
                found: Some("NONE/NONE".to_string()),
            })
        }
    }

    fn borrow_book(&mut self, bookid: BookId, memberid: MemberId) -> errors::Result<BorrowingRecord> {
        let book = &mut self.get_book(bookid)?;

        match book.status {
            BookStatus::Avialable => {
                book.status = BookStatus::Borrowed;
                Ok(BorrowingRecord::new(memberid, bookid))
            }
            BookStatus::Borrowed => Err(LibErrors::Unavialable { message: "Book is not avialable".to_string(), id: None }),
            _ => Err(errors::LibErrors::NotFound),
        }
    }
    fn return_book(&mut self, memberid: Option<MemberId>, recordid: RecordID) {
        self.get_record(Some(recordid), None).unwrap().return_at = Some(chrono::Utc::now())
    }
}
