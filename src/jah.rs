use inquire::{Editor, InquireError, Text};
use std::process::Command;

pub struct Jah {}

impl Jah {
    pub fn commit() -> Result<(), InquireError> {
        if zuu() {
            let subject = Text::new("Enter the commit subject :").prompt()?;
            let body = Editor::new("The commit body :").prompt()?;
            let why = Editor::new("Explain the  changes :")
                .with_editor_command("vim".as_ref())
                .prompt()?;
            let message = format!("{subject}\n\n{body}\n\n{why}");
            let ok = Command::new("git")
                .args(["commit", "-m", message.as_str()])
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

fn clear() -> bool {
    #[cfg(target_os = "linux")]
    Command::new("clear")
        .status()
        .expect("Failed to clear screen");
    #[cfg(target_os = "windows")]
    Command::new("cls")
        .status()
        .expect("Failed to clear screen");
    true
}

fn zuu() -> bool {
    clear();
    Command::new("tux")
        .current_dir(".")
        .spawn()
        .expect("Failed to run tux")
        .wait()
        .expect("Failed to wait on child")
        .success()
        && clear()
}
