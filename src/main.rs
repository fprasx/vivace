use std::{
    error::Error,
    io::{self, stdout, BufRead, Write},
    process,
};

const RESET: &str = "\x1b[0m";
const RED: &str = "\x1b[31m";
const RED_BG: &str = "\x1b[0;101m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";

fn handle_line(writer: &mut impl Write, line: String) -> io::Result<()> {
    if let Some(rest) = line.strip_prefix("INFO") {
        write!(writer, "{GREEN}INFO{RESET}")?;
        writeln!(writer, "{rest}")?;
    } else if let Some(rest) = line.strip_prefix("WARNING") {
        write!(writer, "{YELLOW}WARNING{RESET}")?;
        writeln!(writer, "{rest}")?;
    } else if let Some(rest) = line.strip_prefix("ERROR") {
        write!(writer, "{RED}ERROR{RESET}")?;
        writeln!(writer, "{rest}")?;
    } else if let Some(rest) = line.strip_prefix("CRITICAL WARNING") {
        write!(writer, "{RED_BG}CRITICAL WARNING{RESET}")?;
        writeln!(writer, "{rest}")?;
    } else {
        writeln!(writer, "{line}")?;
    }
    writer.flush()
}

fn die(e: impl Error) {
    eprintln!("error reading line: {e}");
    process::exit(69);
}

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let stdout = stdout();
    let mut stdout = stdout.lock();

    for line in stdin.lines() {
        match line {
            Ok(line) => {
                handle_line(&mut stdout, line).unwrap_or_else(die);
            }
            Err(e) => die(e),
        }
    }

    stdout.flush().unwrap();
}
