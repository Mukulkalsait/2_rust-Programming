// Book.rs contain all book datastractures

use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
/// BookStatus
pub enum BookStatus {
    Avialable,
    UnAvialable,
    Borrowed,
    Damaged,
    Lost,
    Reparing,
}

pub type BookId = Uuid;

#[derive(Debug, Clone)]
pub struct Book {
    pub book_id: BookId,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub status: BookStatus,
}
