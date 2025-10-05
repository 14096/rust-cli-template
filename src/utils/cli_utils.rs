use std::{process, time::Instant};

#[derive(Debug)]
pub enum Color {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Black,
}

impl Color {
    fn ansi_code(&self) -> &'static str {
        match self {
            Color::Red => "\x1b[31m",
            Color::Green => "\x1b[32m",
            Color::Yellow => "\x1b[33m",
            Color::Blue => "\x1b[34m",
            Color::Magenta => "\x1b[35m",
            Color::Cyan => "\x1b[36m",
            Color::White => "\x1b[37m",
            Color::Black => "\x1b[30m",
        }
    }
}

pub fn color(color: Color, text: &str) {
    println!("{}{}{}", color.ansi_code(), text, "\x1b[0m");
}

pub fn print_header() {
    let header = r#"
          ▜ ▘
   ▛▛▌▌▌▛▘▐ ▌
   ▌▌▌▙▌▙▖▐▖▌
      ▄▌     
"#;
    color(Color::Magenta, &header);
}

pub fn print_elapsed(start: Instant) {
    let duration = start.elapsed();
    let mins = duration.as_secs() / 60;
    let secs = duration.as_secs() % 60;
    let millis = duration.subsec_millis();

    let out = format!(
        "\n\nELAPSED\n\
        {:02}m {:02}s {:03}ms\n",
        mins, secs, millis,
    );

    color(Color::Green, &out);
}

pub fn clear_terminal() {
    let command = if cfg!(target_os = "windows") {
        "cls"
    } else {
        "clear"
    };

    let status = process::Command::new(command)
        .status()
        .expect("Failed to execute command");

    if !status.success() {
        eprintln!(
            "ERROR: Command '{}' failed with status: {}",
            command, status
        );
    }
}
