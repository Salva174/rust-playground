use std::io::{Read, Stdin, Stdout, Write};
use std::os::fd::AsRawFd;
use pizzeria_lib::table::{Table, TableCell, TableRow};
use pizzeria_lib::table_menu::TableMenu;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut stdout = std::io::stdout();
    let mut stdin = std::io::stdin();

    let mut in_buffer = [0u8; 64];

    setup_terminal()?;
    writeln!(stdout, "\x1B[1J\x1B[1;1H")?;

    let text_title = String::from("Test-Menu");
    let table = Table::new( vec![
        TableRow::new( vec![
            TableCell::new(String::from(">")),
            TableCell::new(String::from("Pizza")),
            TableCell::new(String::from("Mergherita"))
        ]),
        TableRow::new( vec! [
            TableCell::new(String::from(" ")),
            TableCell::new(String::from("Pizza")),
            TableCell::new(String::from("Hawaii"))
        ]),
        TableRow::new( vec! [
            TableCell::new(String::from(" ")),
            TableCell::new(String::from("Pizza")),
            TableCell::new(String::from("Salami"))
        ])
    ]);

    let mut table_menu = TableMenu::new(text_title, table);
    writeln!(stdout, "{table_menu}")?;
    let mut selected_row = 0;


    loop {
        let size = stdin.read(&mut in_buffer)?;
        let input = &in_buffer[..size];


        writeln!(stdout, "\x1B[1J\x1B[1;1H")?;
        match input {
            &[3] => {
                break;
            }
            &[27, 91, 65] => {  // KEY_UP
                if selected_row > 0 {
                    selected_row -= 1;
                } else {
                    selected_row = table_menu.table_mut().rows_mut().len() - 1;
                }
            }
            &[27, 91, 66] => {  // KEY_DOWN
                if selected_row < table_menu.table_mut().rows_mut().len() - 1 {
                    selected_row += 1;
                } else {
                    selected_row = 0;
                }
            }
            &[27, 91, 67] => {
                writeln!(stdout, "Right")?;
            }
            &[27, 91, 68] => {
                writeln!(stdout, "Left")?;
            }

            _ => {
                writeln!(stdout, "{input:?}")?;
            }
        }

        for (index, row) in table_menu.table_mut().rows_mut().iter_mut().enumerate() {
            let cell = &mut row.cells_mut()[0];
            cell.text_mut().clear();
            if index == selected_row {
                cell.text_mut().push('>');
            } else {
                cell.text_mut().push(' ');
            }
        }
        writeln!(stdout, "{table_menu}")?;
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