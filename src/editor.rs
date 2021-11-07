use std::collections::HashMap;
use std::env;
use std::io::{stdout, StdoutLock};
use std::time::Duration;
use std::time::Instant;

use std::fmt;

use super::terminal;
use terminal::Terminal;

use super::modes::Mode;
use super::status_message::StatusMessage;
use super::window::Window;

use super::rows::Row;

use super::document;
use document::Document;

use crossterm::cursor::{CursorShape, position};
use crossterm::event::{read, Event, KeyCode as Key, KeyEvent, KeyModifiers as Mod};
use crossterm::style::Color;
use crossterm::terminal::disable_raw_mode;

use std::fs::OpenOptions;
use std::io::prelude::*;

const STATUS_FG_COLOUR: Color = Color::Rgb {
    r: 63,
    g: 63,
    b: 63,
};
const STATUS_BAR_BG_COLOUR: Color = Color::Rgb {
    r: 239,
    g: 239,
    b: 239,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default, Clone, Copy, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl From<(u16, u16)> for Position {
    fn from(coord: (u16, u16)) -> Self {
        Self {
            x: coord.0.into(),
            y: coord.1.into(),
        }
    }
}


impl From<(usize, usize)> for Position {
    fn from(coord: (usize, usize)) -> Self {
        Self {
            x: coord.0,
            y: coord.1,
        }
    }
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

struct Object<T: fmt::Debug> {
    obj: T,
}

impl<T: fmt::Debug> fmt::Debug for Object<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self.obj)
    }
}

impl<T: fmt::Debug> Object<T> {
    fn log(name: String, obj: T) {
        let mut file = OpenOptions::new();
        let mut file = file.append(true).open("log.txt").unwrap();

        let mut text = format!("{}:\n{:#?}\n", name, obj);
        file.write_all(text.as_bytes());
    }

    fn clear() -> Result<(), std::io::Error> {
        let mut file = OpenOptions::new();
        let mut file = file.write(true).truncate(true).open("log.txt").unwrap();

        Ok(())
    }
}

fn init_command_window(doc_width: f32, doc_height: f32) -> Window {
    let x1 = (doc_width * 0.2) as u16;
    let x2 = (doc_width * 0.8) as u16;

    let y1 = (doc_height * 0.2) as u16;
    let y2 = (doc_height * 0.8) as u16;

    Window::new("command".to_string(), x1, x2, y1, y2)
}

pub struct Editor<'a> {
    mode: Mode,
    offset: Position,
    document: Document,
    terminal: Terminal<'a>,
    cursor_position: Position,
    should_quit: bool,
    status: StatusMessage,
    windows: HashMap<String, Window>,
}

impl<'a> Editor<'a> {
    pub fn new(stdout: StdoutLock<'a>) -> Self {
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

        let terminal = Terminal::new(stdout).expect("Failed to initialize terminal.");
        let height = terminal.size().height as f32;
        let width = terminal.size().width as f32;

        let command_window = init_command_window(width, height);
        let mut windows: HashMap<String, Window> = HashMap::new();
        windows.insert("command".to_string(), command_window);

        Self {
            mode: Mode::Normal,
            offset: Position::default(),
            should_quit: false,
            document,
            terminal,
            cursor_position: Position { x: 0, y: 0 },
            status: StatusMessage::from(initial_status),
            windows,
        }
    }

    pub fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        self.terminal.set_cursor_position(&Position::new(0, 0));
        Ok(())
    }

    fn check_mode(&mut self, key: Event) {
        if self.mode == Mode::Normal {
            self.terminal.change_cursor_shape(CursorShape::Block);
            if let Some(window) = self.windows.get_mut("command") {
                if let Some(mut string) = window.string.clone() {
                    let mut text_entry = Row::from(string.clone().as_str());
                    text_entry.string = "".to_string();
                    window.string = Some(text_entry.string);
                }
            }
            self.normal_mode(key);
        } else if self.mode == Mode::Command {
            self.command_mode(key);
        } else {
            self.terminal.change_cursor_shape(CursorShape::Line);
            self.insert_mode(key);
        }
    }

    pub fn run(&mut self) {
        if let Err(error) = self.refresh_screen() {
            self.terminal.clear_screen();
            eprintln!("{}", error);
        };

        loop {
            let key = self.create_event(Key::Null, Mod::NONE);
            self.check_mode(key);

            if self.should_quit {
                self.terminal.clear_screen();
                disable_raw_mode().unwrap();
                break;
            } else if self.mode != Mode::Command {
                self.terminal.update_dimensions();
                self.draw_rows();
                self.draw_status_bar();
                self.draw_message_bar();

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
        let pressed_key = read().unwrap();
        match pressed_key {
            Event::Key(event) => match event.code {
                Key::Esc => {
                    self.change_mode(Mode::Normal);
                }
                _ => self.check_mode(pressed_key),
            },
            _ => (),
        };

        self.scroll();
        Ok(())
    }

    fn change_mode(&mut self, change_to: Mode) {
        self.mode = change_to;
    }

    fn normal_mode(&mut self, key: Event) {
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
            Event::Key(event) => match event.code {
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
                    if event.modifiers.contains(Mod::ALT) {
                        let filename = &self.document.filename;
                        if filename == "[NO FILE OPENED]" {
                            todo!("Ask the user for the name of the file.");
                        } else {
                            let status = StatusMessage::from("File written.");
                            self.document.save_file();
                            self.status = status;
                        }
                    } else {
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
                                    // NOTE: This will need fixing, different behaviour when
                                    // non ascii alphabetic characters appear.
                                    x = x.saturating_add(index + 1);
                                }
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
                Key::Char('0') => x = 0,
                Key::Char('S') => {
                    if let Some(row) = self.document.row(y) {
                        let contents = row.string.trim_start();
                        x = width - contents.len();
                    }
                }
                Key::Char('s') => x = width.saturating_sub(1),

                Key::Char(':') => self.change_mode(Mode::Command),

                // changing modes
                Key::Char('i') => {
                    self.change_mode(Mode::Insert);
                }

                Key::Char('A') => {
                    let row = self.document.row(y).unwrap();
                    x = row.len;

                    self.change_mode(Mode::Insert);
                }

                Key::Char('q') => {
                    if event.modifiers.contains(Mod::CONTROL) {
                        self.should_quit = true;
                    }
                }

                _ => (),
            },
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

    fn command_mode(&mut self, key: Event) {
        Object::log("cursor position at start".to_string(), self.cursor_position);
        
        if let Some(mut window) = self.windows.get_mut("command") {
            window.draw_border(&mut self.terminal.stdout);
            let Position { mut x, mut y } = self.cursor_position;
            let (tb_x, tb_y) = position().unwrap();
            if tb_x <= x as u16 {
                window.draw_text_box(&mut self.terminal.stdout, Some(self.cursor_position.x as u16));
            } else {
                window.draw_text_box(&mut self.terminal.stdout, None);
            }

            y = tb_y as usize;

            match key {
                Event::Key(event) => match event.code {
                    Key::Char(c) => {
                        if let Some(mut string) = window.string.clone() {
                            let mut text_entry = Row::from(string.clone().as_str());

                            // This is for typing
                            window.cursor_position.x += 2;

                            text_entry.insert(&window.cursor_position, c);
                            window.string = Some(text_entry.string);
                        } else {
                            let string = Some(String::from(c));
                            window.string = string;
                        }
                        x += 2;
                        self.cursor_position = Position::from((x, y));
                    }
                    _ => (),
                },
                _ => (),
            }

            Object::log("cursor position at the end".to_string(), self.cursor_position);
            window.draw_all(&mut self.terminal.stdout);
        }
    }

    fn insert_mode(&mut self, key: Event) {
        match key {
            Event::Key(event) => match event.code {
                Key::Esc => {
                    self.change_mode(Mode::Normal);
                }
                Key::Backspace => {
                    self.document.delete(&self.cursor_position);
                    let h_key_event = self.create_event(Key::Char('h'), Mod::NONE);
                    self.normal_mode(h_key_event);
                }
                Key::Enter => {
                    self.document.enter(&self.cursor_position);
                    let j_key_event = self.create_event(Key::Char('j'), Mod::NONE);
                    let zero_key_event = self.create_event(Key::Char('0'), Mod::NONE);

                    self.normal_mode(j_key_event);
                    self.normal_mode(zero_key_event);
                }
                Key::Tab => {
                    let space_key_event = self.create_event(Key::Char(' '), Mod::NONE);

                    self.insert_mode(space_key_event);
                    self.insert_mode(space_key_event);
                    self.insert_mode(space_key_event);
                    self.insert_mode(space_key_event);
                }
                Key::Char(c) => {
                    self.document.insert(c, &self.cursor_position);
                    let l_key_event = self.create_event(Key::Char('l'), Mod::NONE);

                    self.normal_mode(l_key_event);
                }
                _ => (),
            },
            _ => (),
        }
    }

    fn create_event(&self, key: Key, modifier: Mod) -> Event {
        Event::Key(KeyEvent {
            code: key,
            modifiers: modifier,
        })
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
