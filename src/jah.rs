use inquire::{Editor, InquireError, Text};
use std::process::Command;

pub struct Jah {}

impl Jah {
    pub fn new() -> Self {
        Self {}
    }

    pub fn commit(self) -> Result<(), InquireError> {
        if zuu() {
            let subject = Text::new("Enter the commit subject :").prompt()?;
            let body = Editor::new("The commit body :").prompt()?;
            let why = Editor::new("Explain the  changes :")
                .with_editor_command("vim".as_ref())
                .prompt()?;
            let message = format!("{}\n\n{}\n\n{}", subject, body, why);
            let ok = Command::new("git")
                .args(&["commit", "-m", message.as_str()])
                .current_dir(".")
                .spawn()
                .expect("Failed to run git")
                .wait()
                .expect("Failed to wait on child")
                .success();
            if ok {
                Ok(())
            } else {
                println!("Failed to run git");
                Ok(())
            }
        } else {
            eprintln!("test failed check your code");
            Ok(())
        }
    }
}

fn zuu() -> bool {
    true
}
