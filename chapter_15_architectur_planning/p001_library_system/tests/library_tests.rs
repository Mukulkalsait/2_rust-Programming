use p001_library_system::{
    book::{Book, BookStatus},
    errors::LibErrors,
    library::Library,
    member::{self, Member, Membership},
};
use pretty_assertions::assert_eq;
use test_case::test_case;
use uuid::Uuid;

fn create_test_lib() -> Library {
    let book = Book::new("Rust Programming".to_string(), "Steav".to_string(), "975-2348274932".to_string(), BookStatus::Avialable);
    let member = Member::new("Test User".to_string(), "test@pompom.co".to_string(), Membership::standard);

    Library::new(book, member)
}

#[test]
fn test_borrow_book() {
    let mut lib = create_test_lib();
    let mut bookid = lib.books[0].book_id;
    let mut memberid = lib.members[0].member_id;

    let res = lib.borrow_book(bookid, memberid);
    assert!(res.is_ok());

    let book = lib.get_book(bookid);
    assert_eq!(book.unwrap().status, BookStatus::Borrowed);
}

#[test]
fn test_borrow_unabialable_book() {
    let mut lib = create_test_lib();
    let bookid = lib.books[0].book_id;
    let memberid = lib.members[0].member_id;
    let member2 = Member::new("test2".to_string(), "testx@3.md".to_string(), Membership::vip);

    let _res = lib.borrow_book(bookid, memberid);
    let borrow_again = lib.borrow_book(bookid, member2.member_id);

    assert!(borrow_again.is_err());
}

#[test]
fn test_return_book() {
    let mut lib = create_test_lib();
    let b_id = lib.books[0].book_id;
    let m_id = lib.members[0].member_id;

    let borrow = lib.borrow_book(b_id, m_id);
    lib.records.push(borrow.unwrap());

    assert_eq!(lib.books[0].status, BookStatus::Borrowed);
    let record_id = lib.records[0].record_id;

    println!("📒 RECORD ID: {:?}", record_id);
    let res = lib.return_book(None, record_id);
    println!("res: {:?}", res);
    assert!(res.is_ok());

    assert_eq!(lib.books[0].status, BookStatus::Avialable);
}
