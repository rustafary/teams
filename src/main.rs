use inquire::InquireError;
use jah::Jah;

mod jah;

fn main() -> Result<(), InquireError> {
    let app = Jah::new();
    app.commit()
}
