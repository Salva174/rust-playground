use std::io;
use std::io::{Read, Stdin, Stdout, Write};

pub fn wait_enter(stdout: &mut Stdout, stdin: &mut Stdin, msg: &str) -> io::Result<()> {
    write!(stdout, "{msg}")?;
    stdout.flush()?;

    let mut b = [0u8; 1];
    loop {
        let n = stdin.read(&mut b)?;
        if n == 0 { break; }
        if b[0] == b'\r' || b[0] == b'\n' { break; }
    }
    Ok(())
}

fn prompt(stdin: &mut Stdin, stdout: &mut Stdout, label: &str) -> io::Result<String> {
    write!(stdout, "{}", label)?;
    stdout.flush()?;

    let mut buf = Vec::new();
    let mut byte = [0u8; 1];

    loop {
        let n = stdin.read(&mut byte)?;
        if n == 0 { break; }
        match byte[0] {
            b'\r' | b'\n' => break,
            8 | 127 => { // Backspace
                if !buf.is_empty() {
                    buf.pop();
                    write!(stdout, "\x08 \x08")?;
                    stdout.flush()?;
                }
            }
            b => {
                buf.push(b);
                stdout.write_all(&[b])?;
                stdout.flush()?;
            }
        }
    }
    writeln!(stdout)?;
    Ok(String::from_utf8_lossy(&buf).trim().to_string())
}