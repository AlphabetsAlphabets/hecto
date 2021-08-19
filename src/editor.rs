use super::terminal;
use terminal::Terminal;

use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(PartialEq)]
enum Mode {
    Insert,
    Normal
}

pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    mode: Mode,
    status_bar: String
}

impl Editor {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::new().expect("Failed to initialize terminal."),
            cursor_position: Position { x: 0, y: 0 },
            mode: Mode::Normal,
            status_bar: "".to_string(),
        }
    }

    pub fn refresh_screen(&self) -> Result<(), std::io::Error> {
        // Terminal::cursor_hide();
        Terminal::cursor_position(&Position::new(0, 0));
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
                // Terminal::cursor_show();
                break;
            } else {
                self.draw_tildes();
                Terminal::cursor_position(&self.cursor_position);
            }
            // Terminal::cursor_show();

            if let Err(error) = self.process_keypress() {
                panic!(error);
            };
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let Position { mut x, mut y } = self.cursor_position;
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Esc | Key::Char('j') | Key::Char('k') | Key::Char('l') | Key::Char('h') => self.move_cursor(pressed_key),
            _ => (),
        }
        Ok(())
    }

    fn move_cursor(&mut self, key: Key) {
        let Position { mut x, mut y } = self.cursor_position;

        let size = self.terminal.size();
        let width = size.width.saturating_sub(1) as usize;
        let height = size.height.saturating_sub(1) as usize;
        
        if self.mode == Mode::Normal {
            match key {
                Key::Char('j') => y = y.saturating_sub(1),
                Key::Char('k') => {
                    if y < height {
                        y = y.saturating_add(1)
                    }
                },
                Key::Char('h') => x = x.saturating_sub(1),
                Key::Char('l') => {
                    if x < width {
                        x = x.saturating_add(1)
                    }
                },

                Key::Char('i') => {
                    self.mode = Mode::Insert;
                }
                _ => (),
            }

            self.cursor_position = Position { x, y }
        }

        if self.mode == Mode::Insert {
            match key {
                Key::Esc => {
                    self.mode = Mode::Normal;
                },
                _ => ()
            }
        }
    }

    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("Hecto -- version {}\r", VERSION);         

        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();

        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));

        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);

        println!("{}\r", welcome_message);
    }

    fn draw_tildes(&mut self) {
        let height = self.terminal.size().height;
        for row in 0..height {
            Terminal::clear_current_line();
            if row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }

        if self.mode == Mode::Normal {
            self.status_bar = format!("{}{}", self.status_bar, "MODE: NORMAL");
        } else {
            self.status_bar = format!("{}{}", self.status_bar, "MODE: INSERT");
        }
        println!("{}", self.status_bar);
    }
}
