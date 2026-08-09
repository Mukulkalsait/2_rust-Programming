ENTITIES:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

BOOK
├── id (unique)
├── title (string)
├── author (string)
├── ISBN (string)
├── publication_year (number)
├── status (Available | Borrowed | Lost | Damaged)
└── location (shelf/row)

MEMBER
├── id (unique)
├── name (string)
├── email (string)
├── membership_id (unique)
├── join_date (date)
├── status (Active | Suspended | Expired)
└── contact (phone)

BORROWING_RECORD
├── id (unique)
├── book_id (reference to Book)
├── member_id (reference to Member)
├── borrow_date (date)
├── due_date (date)
├── return_date (date, optional)
└── status (Active | Returned | Overdue)

LIBRARY (Coordinator)
├── books (list of Book)
├── members (list of Member)
├── records (list of BorrowingRecord)
├── borrow_book() - creates record, updates book status
├── return_book() - updates record, releases book
├── get_member_books() - filter records by member
├── get_book_borrower() - filter records by book
├── get_overdue_books() - filter by due_date
└── search_books() - filter by title/author/ISBN

RELATIONSHIPS:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Book ↔ BorrowingRecord ↔ Member
- One Book can have many BorrowingRecord (over time)
- One Member can have many BorrowingRecord
- BorrowingRecord links Book and Member via IDs
- Library owns all three collections

KEY INSIGHTS:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
- Store IDs, not nested objects (no Vec inside Book)
- BorrowingRecord is the "link" between Book and Member
- Library is the "in-memory database" coordinator
- Status changes update both Book and BorrowingRecord
