use inquire::{Confirm, Editor, Text};
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

pub struct Jah {}

impl Jah {
    pub fn commit() {
        let status = Command::new("git")
            .args(["status"])
            .current_dir(".")
            .output()
            .expect("failed to execute git")
            .stdout
            .clone();
        println!(
            "{}",
            String::from_utf8(status).expect("Failed to parse git status")
        );
        sleep(Duration::from_secs(3));
        let _ = Command::new("git")
            .args(["diff", "-p"])
            .current_dir(".")
            .spawn()
            .expect("failed to execute git")
            .wait()
            .expect("failed to wait on git")
            .success();
        if Confirm::new("Add code to git ?")
            .with_default(false)
            .prompt()
            .expect("Failed to get editor status")
            .eq(&true)
        {
            assert!(Command::new("git")
                .args(["add", "."])
                .current_dir(".")
                .spawn()
                .expect("Failed to spawn git")
                .wait()
                .expect("Failed to spawn `git`")
                .success());

            if zuu() {
                let subject = Text::new("Enter the commit subject :")
                    .prompt()
                    .expect("Failed to get subject");
                let body = Editor::new("The commit body :")
                    .prompt()
                    .expect("Failed to get body");
                let why = Editor::new("Explain the  changes :")
                    .with_editor_command("vim".as_ref())
                    .prompt()
                    .expect("Failed to get editor text");
                let message = format!("{subject}\n\n{body}\n\n{why}\n");
                assert!(Command::new("git")
                    .args(["commit", "-m", message.as_str()])
                    .current_dir(".")
                    .spawn()
                    .expect("Failed to run git")
                    .wait()
                    .expect("Failed to wait on child")
                    .success());
            }
        }
    }
    pub fn send() {
        assert!(Command::new("git")
            .args(["push", "origin", "--all"])
            .current_dir(".")
            .spawn()
            .expect("Failed to run git")
            .wait()
            .expect("Failed to wait on child")
            .success());
        assert!(Command::new("git")
            .args(["push", "origin", "--tags"])
            .current_dir(".")
            .spawn()
            .expect("Failed to run git")
            .wait()
            .expect("Failed to wait on child")
            .success());
    }

    pub fn log() {
        assert!(Command::new("git")
            .args(["log", "--oneline", "-n", "5"])
            .current_dir(".")
            .spawn()
            .expect("Failed to run git")
            .wait()
            .expect("Failed to wait on child")
            .success());
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
    clear()
        && Command::new("tux")
            .current_dir(".")
            .spawn()
            .expect("Failed to run tux")
            .wait()
            .expect("Failed to wait on child")
            .success()
        && clear()
}
