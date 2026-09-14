// library.rs

use crate::book::*;
use crate::errors;
use crate::errors::*;
use crate::library::{LibBookErrors, LibUserErrors};
use crate::member::*;
use crate::record::*;

#[derive(Debug, Clone)]
pub struct Library {
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

impl Library {
    pub fn new_empty() -> Self { Self { books: Vec::new(), members: Vec::new(), records: Vec::new() } }

    pub fn new(book: Book, member: Member) -> Self {
        let mut lib = Library::new_empty();
        lib.books.push(book);
        lib.members.push(member);
        lib
    }

    // Y: GET
    pub fn get_record_ref(&self, recordid: RecordID) -> Result<&BorrowingRecord, LibBookErrors> {
        self.records.iter().find(|r| r.record_id == recordid).ok_or(LibBookErrors::NotFound)
    }

    pub fn get_record_mut(&mut self, recordid: RecordID) -> Result<&mut BorrowingRecord, LibBookErrors> {
        self.records.iter_mut().find(|r| r.record_id == recordid).ok_or(LibBookErrors::NotFound)
    }

    pub fn get_book(&mut self, boo_id: BookId) -> Result<&mut Book, LibBookErrors> { self.books.iter_mut().find(|b| b.book_id == boo_id).ok_or(errors::LibBookErrors::NotFound) }
    pub fn get_member(&mut self, memberid: MemberId) -> Result<&mut Member, LibBookErrors> {
        self.members.iter_mut().find(|m| m.member_id == memberid).ok_or(LibBookErrors::NotFound)
    }

    /// check LibInstance on why not to use this function.
    pub fn get_lib_instance<'a>(&'a mut self, recordid: RecordID) -> Result<LibInstance<'a>, LibBookErrors> {
        let record = self.records.iter_mut().find(|r| r.record_id == recordid).ok_or(LibBookErrors::NotFound)?;
        let book = self.books.iter_mut().find(|b| b.book_id == record.boo_id).ok_or(LibBookErrors::NotFound)?;
        let member = self.members.iter_mut().find(|m| m.member_id == record.mem_id).ok_or(LibBookErrors::NotFound)?;
        Ok(LibInstance { book, member, record })
    }

    pub fn update_record_return_data(&mut self, recordid: RecordID) -> Result<(), LibBookErrors> {
        let rec = self.records.iter_mut().find(|r| r.record_id == recordid).ok_or(LibBookErrors::NotFound)?;
        rec.return_at = Some(chrono::Utc::now());
        Ok(())
    }

    pub fn update_book_status(&mut self, bookid: BookId, book_status: BookStatus) -> Result<(), LibBookErrors> {
        let book = self.books.iter_mut().find(|b| b.book_id == bookid).ok_or(LibBookErrors::NotFound)?;
        book.status = book_status;
        Ok(())
    }

    pub fn update_membership_status(&mut self, memberid: MemberId, membership_status: Membership) -> Result<(), LibBookErrors> {
        let member = self.members.iter_mut().find(|m| m.member_id == memberid).ok_or(LibBookErrors::NotFound)?;
        member.membership = membership_status;
        Ok(())
    }
    pub fn get_borrowing_details(&self, recordid: RecordID) -> Result<BorrowingDetails, LibBookErrors> {
        let record = self.records.iter().find(|r| r.record_id == recordid).ok_or(LibBookErrors::NotFound)?;
        let book = self.books.iter().find(|b| b.book_id == record.boo_id).ok_or(LibBookErrors::NotFound)?;
        let member = self.members.iter().find(|m| m.member_id == record.mem_id).ok_or(LibBookErrors::NotFound)?;
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

    pub fn borrow_book(&mut self, bookid: BookId, memberid: MemberId) -> Result<BorrowingRecord, LibBookErrors> {
        let book = &mut self.get_book(bookid)?;

        match book.status {
            BookStatus::Avialable => {
                book.status = BookStatus::Borrowed;
                Ok(BorrowingRecord::new(memberid, bookid))
            },
            BookStatus::Borrowed => Err(LibBookErrors::Unavialable { message: "Book is not avialable".to_string(), id: None }),
            _ => Err(errors::LibBookErrors::NotFound),
        }
    }
    pub fn return_book(&mut self, memberid: Option<MemberId>, recordid: RecordID) -> Result<(), LibBookErrors> {
        let record = self.get_record_mut(recordid)?;
        let book_id = record.boo_id;
        record.return_at = Some(chrono::Utc::now());

        let book = self.get_book(book_id)?;
        book.status = BookStatus::Avialable;

        Ok(())
    }
}
