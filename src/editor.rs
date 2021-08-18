use super::terminal;
use terminal::Terminal;

use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

struct Position {
    x: usize,
    y: usize,
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::new().expect("Failed to initialize terminal."),
            cursor_position: Position { x: 0, y: 0 },
        }
    }

    pub fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(0, 0);
        Terminal::flush()
    }

    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {
                Terminal::clear_screen();
                panic!(error);
            };

            if self.should_quit {
                Terminal::clear_screen();
                Terminal::cursor_show();
                break;
            } else {
                self.draw_tildes();
                Terminal::cursor_position(0, 0);
            }
            Terminal::cursor_show();

            if let Err(error) = self.process_keypress() {
                Terminal::cursor_position(0, 0);
                panic!(error);
            };
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        }
        Ok(())
    }

    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("Hecto -- version {}\r", VERSION);         

        let width = self.terminal.size().0 as usize;
        let len = welcome_message.len();

        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));

        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);

        println!("{}\r", welcome_message);
    }

    fn draw_tildes(&self) {
        let height = self.terminal.size().1;
        for row in 0..height {
            Terminal::clear_current_line();
            if row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }
}
