use super::terminal;
use terminal::Terminal;

use super::document;
use document::{Document, Row};

use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(PartialEq)]
enum Mode {
    Insert,
    Normal,
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
    mode: Mode,
    status_bar: String,
    should_quit: bool,
    document: Document,
    terminal: Terminal,
    cursor_position: Position,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            mode: Mode::Normal,
            status_bar: "".to_string(),
            should_quit: false,
            document: Document::open(),
            terminal: Terminal::new().expect("Failed to initialize terminal."),
            cursor_position: Position { x: 0, y: 0 },
        }
    }

    pub fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        // Terminal::cursor_hide();
        Terminal::cursor_position(&Position::new(0, 0));
        if self.should_quit {
            Terminal::clear_screen();
        } else {
            self.draw_rows();
            self.draw_status_bar();
            Terminal::cursor_position(&self.cursor_position);
        }
        Terminal::flush()
    }

    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {
                Terminal::clear_screen();
                panic!(error);
            };

            if self.should_quit {
                break;
            }

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
            Key::Char('j') | Key::Char('k') | Key::Char('l') | Key::Char('h') 
                | Key::Char('G') | Key::Char('g') | Key::Char('0') | Key::Char('s') => {
                    self.check_mode(pressed_key)
                }
            Key::Esc => self.change_mode(Mode::Normal),
            Key::Char('i') => self.change_mode(Mode::Insert),
            _ => (),
        }

        Ok(())
    }

    fn change_mode(&mut self, change_to: Mode) {
        self.mode = change_to;
    }

    fn normal_mode(&mut self, key: Key) {
        let Position { mut x, mut y } = self.cursor_position;

        let size = self.terminal.size();
        let width = size.width.saturating_sub(1) as usize;
        let height = size.height.saturating_sub(1) as usize;

        match key {
            Key::Char('k') => y = y.saturating_sub(1),
            Key::Char('j') => {
                if y < height {
                    y = y.saturating_add(1)
                }
            }
            Key::Char('h') => x = x.saturating_sub(1),
            Key::Char('l') => {
                if x < width {
                    x = x.saturating_add(1)
                }
            }

            Key::Char('g') => y = 0,
            Key::Char('G') => y = height,
            Key::Char('0') => x = 0,
            Key::Char('s') => x = width,

            // changing modes
            Key::Char('i') => {
                self.mode = Mode::Insert;
            }
            _ => (),
        }

        self.cursor_position = Position { x, y }
    }

    fn insert_mode(&mut self, key: Key) {
        match key {
            Key::Esc => {
                self.mode = Mode::Normal;
            }
            _ => (),
        }
    }

    fn check_mode(&mut self, key: Key) {
        if self.mode == Mode::Normal {
            self.normal_mode(key);
        } else {
            self.insert_mode(key);
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

    fn draw_row(&self, row: &Row) {
        let start = 0;
        let end = self.terminal.size().width as usize;
        let row = row.render(start, end);
        println!("{}\r", row);
    }

    fn draw_rows(&mut self) {
        let height = self.terminal.size().height;
        for terminal_row in 0..height {
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(terminal_row as usize) {
                self.draw_row(row);
            } else if terminal_row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }

    fn draw_status_bar(&mut self) {
        if self.mode == Mode::Normal {
            self.status_bar = "MODE: NORMAL".to_string();
        } else {
            self.status_bar = "MODE: INSERT".to_string();
        }
        println!("{}", self.status_bar);
    }
}
