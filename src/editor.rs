use std::env;

use termion::event::Key;

use super::terminal;
use terminal::Terminal;

use super::document;
use document::{Document, Row};


const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(PartialEq)]
enum Mode {
    Insert,
    Normal,
    Command,
}

#[derive(Default)]
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
    offset: Position,
    status_bar: String,
    should_quit: bool,
    document: Document,
    terminal: Terminal,
    cursor_position: Position,
}

impl Editor {
    pub fn new() -> Self {
        let args: Vec<String> = env::args().collect();
        let document = if args.len() >= 2 {
            let file_name = &args[1];
            Document::open(file_name)
        } else {
            Document::default()
        };

        Self {
            mode: Mode::Normal,
            offset: Position::default(),
            status_bar: "".to_string(),
            should_quit: false,
            document,
            terminal: Terminal::new().expect("Failed to initialize terminal."),
            cursor_position: Position::default(),
        }
    }

    pub fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        self.terminal.cursor_position(&Position::new(0, 0));
        Ok(())
    }

    pub fn run(&mut self) {
        if let Err(error) = self.refresh_screen() {
            self.terminal.clear_screen();
            panic!(error);
        };

        loop {
            if self.should_quit {
                self.terminal.clear_screen();
                break;
            } else {
                self.draw_rows();
                self.draw_status_bar();
                self.terminal.cursor_position(&self.cursor_position);
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
                },
            Key::Esc => self.change_mode(Mode::Normal),
            Key::Char('i') => self.change_mode(Mode::Insert),
            _ => (),
        }

        self.scroll();
        Ok(())
    }

    fn change_mode(&mut self, change_to: Mode) {
        self.mode = change_to;
    }

    fn normal_mode(&mut self, key: Key) {
        let Position { mut x, mut y } = self.cursor_position;

        let size = self.terminal.size();
        let width = size.width.saturating_sub(1) as usize;
        let height = self.document.len();

        match key {
            Key::Char('k') => y = y.saturating_sub(1),
            Key::Char('j') => {
                if y < height {
                    y = y.saturating_add(1)
                }
            }
            Key::Char('h') => x = x.saturating_sub(1), // TODO: Look into the x field in position struct
            Key::Char('l') => {
                if x < width {
                    x = x.saturating_add(1)
                }
            }

            Key::Char('g') => y = 0,
            Key::Char('G') => y = height,
            Key::Char('0') => x = 0,
            Key::Char('s') => x = width,

            Key::Char(':') => todo!("Implement command mode."),

            // changing modes
            Key::Char('i') => self.change_mode(Mode::Insert),
            _ => (),
        }

        self.cursor_position = Position { x, y }
    }

    fn insert_mode(&mut self, key: Key) {
        match key {
            Key::Esc => self.change_mode(Mode::Command),
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

    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;
        let mut offset = &mut self.offset;

        if y < offset.y {
            offset.y = y;
        } else if y >= offset.y.saturating_add(height) {
            offset.y = y.saturating_sub(height).saturating_add(1);
        }

        if x < offset.x {
            offset.x = x;
        } else if x >= offset.x.saturating_add(width) {
            offset.x = x.saturating_sub(width).saturating_add(1);
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
        let width = self.terminal.size().width as usize;
        let start = self.offset.x;
        let end = self.offset.x + width;
        let row = row.render(start, end);

        println!("{}\r", row);
    }

    fn draw_rows(&mut self) {
        let height = self.terminal.size().height;
        for terminal_row in 0..height - 2 {
            self.terminal.clear_current_line();

            if let Some(row) = self.document.row(terminal_row as usize + self.offset.y) {
                self.draw_row(row);
            } else if self.document.is_empty() && terminal_row == height / 3 {
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
