#[derive(PartialEq,Debug)]
pub struct Ticket {
   pub event_id: String,
   pub section_id: String,
   pub row_id: String,
   pub seat_id: u32,
}

pub trait List<T> {
   fn null() -> Self;
   fn hd(&self) -> Option<i32>;
   fn tl(&self) -> Option<T>;
   fn cons(&self, value: i32) -> Self;
   fn append(&self, other: &Self) -> Self;
}
