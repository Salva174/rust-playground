use std::io::Write;
use std::os::fd::AsRawFd;
use pizzeria_frontend::input::{read_input, InputEvent};
use pizzeria_frontend::parse_arguments;
use pizzeria_frontend::state::{create_initial_state, process_transaction_fallbacks};
use pizzeria_frontend::render::render;
use pizzeria_frontend::update::update;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = std::io::stdout();
    let mut stdin = std::io::stdin();

    let arguments = parse_arguments()?;

    let mut in_buffer = [0u8; 64];

    let termios = setup_terminal()?;

    let mut state = create_initial_state(&arguments);
    render(&mut stdout, &state)?;

    loop {
        let input = read_input(&mut stdin, &mut in_buffer)?;
        if let InputEvent::Unknown { input } = &input {
            writeln!(stdout, "{input:?}")?;
        } else {
            let exit = update(input, &mut state, &mut stdout, &mut stdin, &arguments);
            if exit {
                break;
            }
            render(&mut stdout, &state)?;
            process_transaction_fallbacks(&mut state, &mut stdout);
        }
    }

    reset_terminal(termios)?;

    Ok(())
}

fn setup_terminal() -> std::io::Result<libc::termios> {
    unsafe {
        let tty;
        let mut ptr = core::mem::MaybeUninit::uninit();
        let fd = if libc::isatty(libc::STDIN_FILENO) == 1 {
            libc::STDIN_FILENO
        } else {
            tty = std::fs::File::open("/dev/tty")?;
            tty.as_raw_fd()
        };
        if libc::tcgetattr(fd, ptr.as_mut_ptr()) == 0 {
            let mut termios = ptr.assume_init();
            let old_termios = Clone::clone(&termios);
            let c_oflag = termios.c_oflag;
            libc::cfmakeraw(&mut termios);
            termios.c_oflag = c_oflag;
            if libc::tcsetattr(fd, libc::TCSADRAIN, &termios) == 0 {
                return Ok(old_termios);
            }
        }
    }
    Err(std::io::Error::last_os_error())
}

fn reset_terminal(termios: libc::termios) -> std::io::Result<()> {
    unsafe {
        let tty;
        let fd = if libc::isatty(libc::STDIN_FILENO) == 1 {
            libc::STDIN_FILENO
        } else {
            tty = std::fs::File::open("/dev/tty")?;
            tty.as_raw_fd()
        };
        if libc::tcsetattr(fd, libc::TCSADRAIN, &termios) == 0 {
            return Ok(());
        }
    }
    Err(std::io::Error::last_os_error())
}
