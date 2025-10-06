use std::io::Write;
use std::os::fd::AsRawFd;
use rust_playground::input::{read_input, InputEvent};
use rust_playground::state::{create_initial_state, State};
use rust_playground::render::render;
use rust_playground::update::update;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut stdout = std::io::stdout();
    let mut stdin = std::io::stdin();

    let mut in_buffer = [0u8; 64];

    setup_terminal()?;

    let mut state = create_initial_state();
    render(&mut stdout, &state)?;

    loop {
        let input = read_input(&mut stdin, &mut in_buffer)?;
        if let InputEvent::Unknown { input } = &input {
            writeln!(stdout, "{input:?}")?;
        } else {
            let exit = update(input, &mut state, &mut stdout, &mut stdin);
            if exit {
                break;
            }
            render(&mut stdout, &state)?;
        }
    }

    Ok(())
}

fn setup_terminal() -> std::io::Result<()> {
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
            let c_oflag = termios.c_oflag;
            libc::cfmakeraw(&mut termios);
            termios.c_oflag = c_oflag;
            if libc::tcsetattr(fd, libc::TCSADRAIN, &termios) == 0 {
                return Ok(());
            }
        }
    }
    Err(std::io::Error::last_os_error())
}
