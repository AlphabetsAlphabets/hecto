use std::env;
use std::time::Duration;
use std::time::Instant;

use super::terminal;
use terminal::Terminal;

use super::modes::Mode;
use super::status_message::StatusMessage;

use super::rows::Row;

use super::document;
use document::Document;

use termion::color::Rgb;
use termion::event::Key;

const STATUS_FG_COLOUR: Rgb = Rgb(63, 63, 63);
const STATUS_BAR_BG_COLOUR: Rgb = Rgb(239, 239, 239);
const VERSION: &str = env!("CARGO_PKG_VERSION");

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
    document: Document,
    terminal: Terminal,
    cursor_position: Position,
    should_quit: bool,
    status: StatusMessage,
}

impl Editor {
    pub fn new() -> Self {
        let args: Vec<String> = env::args().collect();
        let mut initial_status = "Press CTRL + Q to QUIT.".to_string();
        let document = if args.len() > 1 {
            let doc = Document::open(&args[1]);
            if doc.is_ok() {
                doc.unwrap()
            } else {
                let mut doc = Document::default();
                initial_status = format!("ERR: Could not open file: {}", doc.filename);
                doc.filename = "[ERROR COULD NOT OPEN FILE]".to_string();
                doc
            }
        } else {
            let mut doc = Document::default();
            doc.filename = "[NO FILE OPENED]".to_string();
            doc
        };

        Self {
            mode: Mode::Normal,
            offset: Position::default(),
            should_quit: false,
            document,
            terminal: Terminal::new().expect("Failed to initialize terminal."),
            cursor_position: Position { x: 0, y: 0 },
            status: StatusMessage::from(initial_status),
        }
    }

    pub fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        self.terminal.set_cursor_position(&Position::new(0, 0));
        Ok(())
    }

    pub fn run(&mut self) {
        if let Err(error) = self.refresh_screen() {
            self.terminal.clear_screen();
            eprintln!("{}", error);
        };

        loop {
            if self.should_quit {
                self.terminal.clear_screen();
                break;
            } else {
                self.draw_rows();
                self.draw_status_bar();
                self.draw_message_bar();
                // since scrolling to the left and right is implemented
                // the cursor needs to retain the current position with
                // as the cursor pos is added with the offset values
                // so to place the cursor in the right position
                // the value for the offsets needs to be subtracted from
                // the cursor's position.
                let pos = Position {
                    x: self.cursor_position.x.saturating_sub(self.offset.x),
                    y: self.cursor_position.y.saturating_sub(self.offset.y),
                };

                self.terminal.set_cursor_position(&pos);
            }

            if let Err(error) = self.process_keypress() {
                eprintln!("{}", error);
            };
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Esc => self.change_mode(Mode::Normal),
            Key::Char('i') => {
                if self.mode == Mode::Insert {
                    self.insert_mode(pressed_key)
                } else if self.mode != Mode::Command {
                    self.change_mode(Mode::Insert)
                } else {
                    self.command_mode(pressed_key)
                }
            }
            Key::Char(':') => self.change_mode(Mode::Command),
            _ => self.check_mode(pressed_key),
        }

        self.scroll();
        Ok(())
    }

    fn change_mode(&mut self, change_to: Mode) {
        self.mode = change_to;
    }

    fn normal_mode(&mut self, key: Key) {
        let terminal_height = self.terminal.size().height as usize;
        let Position { mut x, mut y } = self.cursor_position;

        let doc_height = self.document.len();
        // the width changes depending on the length of the row
        let mut width = if let Some(row) = self.document.row(y) {
            row.len
        } else {
            0
        };

        match key {
            Key::Char('k') => y = y.saturating_sub(1),
            Key::Char('j') => {
                if y < doc_height.saturating_sub(1) {
                    y = y.saturating_add(1)
                }
            }
            Key::Char('h') => {
                // lets the user move to the end of the previous line,
                // if cursor at the start of a line.
                if x > 0 {
                    x -= 1;
                } else if y > 0 {
                    y -= 1;
                    if let Some(row) = self.document.row(y) {
                        x = row.len;
                    } else {
                        x = 0;
                    }
                }
            }
            Key::Char('l') => {
                if x < width {
                    x += 1;
                } else if y < doc_height.saturating_sub(1) {
                    y += 1;
                    x = 0;
                }
            }

            Key::Char('b') => {
                if let Some(row) = self.document.row(y) {
                    if let Some(contents) = row.contents().get(..x) {
                        let mut index = 0;

                        for (count, ch) in contents.chars().rev().enumerate() {
                            if !ch.is_ascii_alphabetic() {
                                index = count + 1;
                                break;
                            }
                        }

                        if (y < doc_height && x == 0) && y > 0 {
                            y -= 1;
                            x = row.len + 2;
                        } else {
                            x = x.saturating_sub(index);
                        }
                    }
                }
            }

            Key::Char('w') => {
                if let Some(row) = self.document.row(y) {
                    if let Some(contents) = row.contents().get(x..) {
                        let mut index = 0;
                        for (count, ch) in contents.chars().enumerate() {
                            if !ch.is_ascii_alphabetic() {
                                index = count;
                                break;
                            }
                        }

                        if x >= width && y < doc_height.saturating_sub(1) {
                            y += 1;
                            x = 0;
                        } else {
                            x = x.saturating_add(index + 1);
                        }
                    }
                }
            }

            Key::Char('K') => {
                // first if only happens on the 1st screen.
                y = if y > terminal_height {
                    // saturating_add/sub not used because y and terminal_height
                    // have the same type.
                    y - terminal_height
                } else {
                    0
                }
            }
            Key::Char('J') => {
                // terminal_height is the number of visible rows on the screen.
                // height is the number of rows in the entire file
                y = if y.saturating_add(terminal_height) < doc_height.saturating_sub(1) {
                    y + terminal_height as usize
                } else {
                    // This is only true when it's at the last page
                    doc_height.saturating_sub(1)
                }
            }

            Key::Char('g') => y = 0,
            Key::Char('S') => x = 0,
            Key::Char('s') => x = width,

            Key::Char(':') => todo!("Implement command mode."),

            // changing modes
            Key::Char('i') => self.change_mode(Mode::Insert),
            Key::Char('A') => {
                let row = self.document.row(y).unwrap();
                x = row.len;
                self.change_mode(Mode::Insert);
            }

            Key::Alt('w') => self.document.save_file(),
            _ => (),
        }

        // adjusts the width the the length of the row
        width = if let Some(row) = self.document.row(y) {
            row.len
        } else {
            0
        };

        // if the cursor is further than the width
        // the x pos of the cursor will be set to the width
        // snapping it to the end of the line.
        if x > width {
            x = width;
        }

        self.cursor_position = Position { x, y }
    }

    fn command_mode(&mut self, key: Key) {
        if !self.status.text.contains(":") {
            self.status.update_status(":");
        }

        match key {
            Key::Char(c) => {
                self.status.text.push_str(&c.to_string());
                // self.change_status(text);
            }
            _ => (),
        }
    }

    fn change_status(&mut self, text: &str) {
        self.status.text = text.to_string();
    }

    fn insert_mode(&mut self, key: Key) {
        let Position { mut x, mut y } = self.cursor_position;
        match key {
            Key::Esc => self.change_mode(Mode::Command),
            Key::Backspace => {
                if self.cursor_position.x > 0 || self.cursor_position.y > 0 {
                    self.normal_mode(Key::Char('h'));
                    self.document.delete(&self.cursor_position);
                }
            }
            Key::Char(c) => {
                if c == '\n' {
                    self.document.enter(&self.cursor_position);
                    self.normal_mode(Key::Char('j'));
                } else {
                    self.document.insert(c, &self.cursor_position);
                    self.normal_mode(Key::Char('l'));
                }
            }

            _ => (),
        }
    }

    fn check_mode(&mut self, key: Key) {
        if self.mode == Mode::Normal {
            self.normal_mode(key);
        } else if self.mode == Mode::Command {
            self.command_mode(key);
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
        let width = self.terminal.size().width as usize;
        let start = self.offset.x;
        let end = self.offset.x + width;
        let row = row.render(start, end);

        println!("{}\r", row);
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

    fn draw_status_bar(&mut self) {
        let width = self.terminal.size().width as usize;
        let filename = if let Some(filename) = self.document.filename.get(..21) {
            filename.to_string()
        } else {
            self.document.filename.clone()
        };

        let status = format!("{} | {}", self.mode, filename);

        let rows = self.document.len() as f32;
        let current_line = (self.cursor_position.y + 1) as f32;

        let percentage = if rows > 0.0 {
            (current_line / rows * 100.0).trunc()
        } else {
            0 as f32
        };

        let line_number = format!("{}/{}: {}%", current_line, rows, percentage);
        let spaces = " ".repeat(width - status.len() - line_number.len());

        self.terminal.set_bg_color(STATUS_BAR_BG_COLOUR);
        self.terminal.set_fg_color(STATUS_FG_COLOUR);
        println!("{}{}{}\r", status, spaces, line_number);
        self.terminal.reset_fg_color();
        self.terminal.reset_bg_color();
    }

    fn draw_message_bar(&mut self) {
        self.terminal.clear_current_line();
        let message = &self.status;
        if Instant::now() - message.time < Duration::new(5, 0) {
            let mut text = message.text.clone();
            text.truncate(self.terminal.size().width as usize);
            print!("{}", text);
        }
    }

    fn draw_rows(&mut self) {
        let height = self.terminal.size().height;
        for terminal_row in 0..height {
            self.terminal.set_cursor_position(&Position {
                x: 0,
                y: terminal_row as _,
            });
            self.terminal.clear_current_line();

            // NOTE: index = terminal_row + self.offset.y
            if let Some(row) = self.document.row(terminal_row as usize + self.offset.y) {
                self.draw_row(row);
            } else if self.document.is_empty() && terminal_row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }
}
