#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn test_greet() {
        let output = Command::new("cargo")
            .arg("run")
            .arg("greet")
            .arg("Alice")
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Hello, Alice!"));
    }

    #[test]
    fn test_goodbye() {
        let output = Command::new("cargo")
            .arg("run")
            .arg("goodbye")
            .arg("Bob")
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Goodbye, Bob!"));
    }
}
