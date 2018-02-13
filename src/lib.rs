#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod contracts;
mod tools;
mod book_keeper;
mod ingredients {
    pub mod ingredient;
    pub mod value;
    pub mod reference;
    pub mod raw;
    pub mod circular;
}
mod recipe;