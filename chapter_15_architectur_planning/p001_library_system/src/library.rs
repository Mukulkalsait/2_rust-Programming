// library.rs

use core::fmt;
use std::fmt::write;

use chrono::DateTime;
use chrono::Utc;

use crate::book::*;
use crate::errors;
use crate::errors::*;
use crate::member::*;
use crate::record;
use crate::record::*;

#[derive(Debug, Clone)]
struct Library {
    pub books: Vec<Book>,
    pub members: Vec<Member>,
    pub records: Vec<BorrowingRecord>,
}

#[derive(Debug)]
/// THIS LOOKS LIFE FANTASTIC IDEA : BUT  R:
///
/// ⚠️ this instance holds complete library mutable references
/// hence => UNTILL ITS DROPED EVERYTIGN WILL STOP....
struct LibInstance<'a> {
    pub book: &'a mut Book,
    pub member: &'a mut Member,
    pub record: &'a mut BorrowingRecord,
}

#[derive(Debug, Clone)]
/// returning struct for data
struct BorrowingDetails {
    pub record_id: RecordID,
    pub book_id: BookId,
    pub member_id: MemberId,
    pub book_title: String,
    pub member_name: String,
    pub borrowed_at: DateTime<Utc>,
    pub due_date: DateTime<Utc>,
    pub return_at: Option<DateTime<Utc>>,
}

impl Library {
    pub fn get_record_ref(&self, recordid: Option<RecordID>, boo_id: Option<BookId>) -> errors::Result<&BorrowingRecord> {
        match (recordid, boo_id) {
            (Some(rid), None) => self.records.iter().find(|r| r.record_id == rid).ok_or(LibErrors::NotFound),
            (None, Some(bid)) => self.records.iter().find(|b| b.boo_id == bid).ok_or(LibErrors::NotFound),
            (Some(_), Some(_)) => Err(LibErrors::InvalidResponse {
                messgage: "Please inter either RecordID or Book ID".to_string(),
                expected: Some("RecordID / BookId".to_string()),
                found: Some("NONE/NONE".to_string()),
            }),
            (None, None) => Err(LibErrors::InvalidResponse {
                messgage: "Please inter either RecordID or Book ID".to_string(),
                expected: Some("RecordID / BookId".to_string()),
                found: Some("NONE/NONE".to_string()),
            }),
        }
    }

    pub fn get_book(&mut self, boo_id: BookId) -> errors::Result<&mut Book> {
        self.books.iter_mut().find(|b| b.book_id == boo_id).ok_or(errors::LibErrors::NotFound)
    }
    pub fn get_record(&mut self, recordid: RecordID) -> errors::Result<&mut BorrowingRecord> {
        self.records.iter_mut().find(|r| r.record_id == recordid).ok_or(LibErrors::NotFound)
    }
    pub fn get_member(&mut self, memberid: MemberId) -> errors::Result<&mut Member> {
        self.members.iter_mut().find(|m| m.member_id == memberid).ok_or(LibErrors::NotFound)
    }

    /// check LibInstance on why not to use this function.
    pub fn get_lib_instance<'a>(&'a mut self, recordid: RecordID) -> errors::Result<LibInstance<'a>> {
        let record = self.records.iter_mut().find(|r| r.record_id == recordid).ok_or(LibErrors::NotFound)?;
        let book = self.books.iter_mut().find(|b| b.book_id == record.boo_id).ok_or(LibErrors::NotFound)?;
        let member = self.members.iter_mut().find(|m| m.member_id == record.mem_id).ok_or(LibErrors::NotFound)?;
        Ok(LibInstance { book, member, record })
    }

    pub fn update_record_return_data(&mut self, recordid: RecordID) -> Result<()> {
        let rec = self.records.iter_mut().find(|r| r.record_id == recordid).ok_or(LibErrors::NotFound)?;
        rec.return_at = Some(chrono::Utc::now());
        Ok(())
    }

    pub fn update_book_status(&mut self, bookid: BookId, book_status: BookStatus) -> Result<()> {
        let book = self.books.iter_mut().find(|b| b.book_id == bookid).ok_or(LibErrors::NotFound)?;
        book.status = book_status;
        Ok(())
    }

    pub fn update_membership_status(&mut self, memberid: MemberId, membership_status: Membership) -> Result<()> {
        let member = self.members.iter_mut().find(|m| m.member_id == memberid).ok_or(LibErrors::NotFound)?;
        member.membership = membership_status;
        Ok(())
    }
    fn get_borrowing_details(&self, recordid: RecordID) -> Result<BorrowingDetails> {
        let record = self.records.iter().find(|r| r.record_id == recordid).ok_or(LibErrors::NotFound)?;
        let book = self.books.iter().find(|b| b.book_id == record.boo_id).ok_or(LibErrors::NotFound)?;
        let member = self.members.iter().find(|m| m.member_id == record.mem_id).ok_or(LibErrors::NotFound)?;
        Ok(BorrowingDetails {
            record_id: record.record_id,
            book_id: book.book_id,
            member_id: member.member_id,
            book_title: book.title.clone(),
            member_name: member.name.clone(),
            borrowed_at: record.borrowed_at,
            return_at: Some(record.return_at.unwrap()),
            due_date: record.due_date,
        })
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
    fn return_book(&mut self, memberid: Option<MemberId>, recordid: RecordID) -> errors::Result<()> {
        let record = self.get_record(recordid)?;
        let book_id = record.boo_id;
        record.return_at = Some(chrono::Utc::now());

        let book = self.get_book(book_id)?;
        book.status = BookStatus::Avialable;

        Ok(())
    }
}
