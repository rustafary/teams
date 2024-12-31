#![allow(clippy::multiple_crate_versions)]

use inquire::InquireError;
use jah::Jah;

mod jah;

fn main() -> Result<(), InquireError> {
    Jah::commit()
}
