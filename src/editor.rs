use std::collections::HashMap;
use std::env;

use std::io;
use std::time::Duration;
use std::time::Instant;

use std::fmt;

use super::terminal;
use terminal::Terminal;

use super::modes::Mode;
use super::status_message::StatusMessage;
use super::window::Window;

use super::gap_buffer::GapBuffer;

use super::ui::{ui, App, run_app};
use tui::backend::CrosstermBackend;

use super::document;
use document::Document;

use crossterm::{
    queue,
    cursor::{position, CursorShape, Hide, Show},
    event::{poll, read, Event, KeyCode as Key, KeyEvent, KeyModifiers as Mod},
    style::Color,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

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

pub struct Object<T: fmt::Debug> {
    obj: T,
}

impl<T: fmt::Debug> fmt::Debug for Object<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self.obj)
    }
}

fn init_command_window(doc_width: f32, doc_height: f32) -> Window {
    let x1 = (doc_width * 0.2) as u16;
    let x2 = (doc_width * 0.8) as u16;

    let y1 = (doc_height * 0.2) as u16;
    let y2 = (doc_height * 0.8) as u16;

    Window::new("command".to_string(), x1, x2, y1, y2)
}

fn init_save_window(doc_width: f32, doc_height: f32) -> Window {
    let x1 = (doc_width * 0.2) as u16;
    let x2 = (doc_width * 0.8) as u16;

    let y1 = (doc_height * 0.5) as u16;
    let y2 = y1 + 3;

    Window::new("save".to_string(), x1, x2, y1, y2)
}

fn update_windows_dimensions(width: f32, height: f32) -> HashMap<String, Window> {
    let command_window = init_command_window(width, height);
    let save_window = init_save_window(width, height);
    let mut windows: HashMap<String, Window> = HashMap::new();
    windows.insert("command".to_string(), command_window);
    windows.insert("save".to_string(), save_window);
    windows
}

pub struct Editor<'a> {
    mode: Mode,
    offset: Position,
    document: Document,
    terminal: Terminal,
    cursor_position: Position,
    should_quit: bool,
    status: StatusMessage,
    windows: HashMap<String, Window>,
    app: App<'a>,
    prev_cursor_position: Position,
}

impl<'a> Editor<'a> {
    pub fn new(stdout: io::Stdout) -> Self {
        let args: Vec<String> = env::args().collect();
        let mut initial_status = "Press CTRL + Q to QUIT.".to_string();
        let document = if args.len() > 1 {
            let doc = Document::open(&args[1]);
            if doc.is_ok() {
                doc
            } else {
                let mut doc = Document::default();
                initial_status = format!("ERR: Could not open file: {}", doc.filename);
                doc.filename = "[ERROR COULD NOT OPEN FILE]".to_string();
                Ok(doc)
            }
        } else {
            let mut doc = Document::default();
            doc.filename = "[NO FILE OPENED]".to_string();
            Ok(doc)
        }
        .unwrap();

        let terminal = Terminal::new(stdout).expect("Failed to initialize terminal.");
        let height = terminal.size().height as f32;
        let width = terminal.size().width as f32;

        let windows = update_windows_dimensions(width, height);
        let commands = vec!["SAVE FILE".to_string(), "QUIT".to_string()];

        let app = App::default();

        Self {
            mode: Mode::Normal,
            offset: Position::default(),
            should_quit: false,
            document,
            terminal,
            cursor_position: Position { x: 0, y: 0 },
            prev_cursor_position: Position { x: 0, y: 0 },
            status: StatusMessage::from(initial_status),
            windows,
            app
        }
    }

    pub fn has_event(&self, timeout: Duration) -> bool {
        poll(timeout).unwrap_or(false)
    }

    pub fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        self.terminal.set_cursor_position(&Position::new(0, 0));
        Ok(())
    }

    fn check_mode(&mut self, key: Event) {
        if self.mode == Mode::Normal {
            self.terminal.change_cursor_shape(CursorShape::Block);
            self.normal_mode(key);

        } else if self.mode == Mode::Command {
            self.command_mode(key);
        } else {
            self.terminal.change_cursor_shape(CursorShape::Line);
            self.insert_mode(key);
        }
    }

    pub fn run(&mut self) {
        enable_raw_mode().unwrap();
        queue!(&mut self.terminal.stdout, EnterAlternateScreen).unwrap();
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
                queue!(&mut self.terminal.stdout, EnterAlternateScreen).unwrap();
                break;

            } else if self.mode != Mode::Command {
                self.terminal.update_dimensions();
                let dimensions = self.terminal.size();
                let width = dimensions.width as f32;
                let height = dimensions.height as f32;
                self.windows = update_windows_dimensions(width, height);

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
        if let Event::Key(event) = pressed_key {
            match event.code {
                Key::Esc => {
                    if self.mode == Mode::Command {
                        self.app.input.clear();
                        queue!(&mut self.terminal.stdout, Show).unwrap();
                    } else if self.mode == Mode::Insert {
                        self.cursor_position.x = self.cursor_position.x.saturating_sub(1);
                        self.terminal.set_cursor_position(&self.cursor_position);
                    }

                    self.change_mode(Mode::Normal);
                }
                _ => self.check_mode(pressed_key),
            }
        };

        self.terminal.stdout.flush().unwrap();
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
        let mut width = if let Some(buffer) = self.document.buffer(y) {
            buffer.len.saturating_sub(1)
        } else {
            0
        };

        if let Event::Key(event) = key {
            match event.code {
                Key::Char('k') => {
                    y = y.saturating_sub(1);
                    if x == width {
                        let top = self.document.buffer(y).unwrap();
                        x = top.len.saturating_sub(1);
                    }
                }

                Key::Char('j') => {
                    if y < doc_height.saturating_sub(1) {
                        y = y.saturating_add(1);
                        if x >= width.saturating_sub(1) {
                            // NOTE: In the case of an empty line, it will move the cursor to the
                            // very end of it because the if condition is still true.
                            let bottom = self.document.buffer(y).unwrap();
                            x = bottom.len.saturating_sub(1);
                        }
                    }

                    // let s_key = self.create_event(Key::Char('s'), Mod::NONE);
                    // self.normal_mode(s_key);
                }

                Key::Char('h') => {
                    // lets the user move to the end of the previous line,
                    // if cursor at the start of a line.
                    if x > 0 {
                        x -= 1;
                    } else if y > 0 {
                        y -= 1;
                        if let Some(buffer) = self.document.buffer(y) {
                            x = buffer.len.saturating_sub(1);
                        } else {
                            x = 0;
                        }
                    }
                }

                Key::Char('l') => {
                    if x < width {
                        x += 1;
                    } else if y < doc_height.saturating_sub(1) && x >= width {
                        y += 1;
                        x = 0;
                    }
                }

                Key::Char('b') => {
                    if let Some(buffer) = self.document.buffer(y) {
                        if let Some(contents) = buffer.line().get(..x.saturating_add(1)) {
                            let mut index = 0;

                            // count starts at 0
                            for (count, ch) in contents.chars().rev().enumerate() {
                                // NOTE: Still buggy, run once in nvim then in hecto to see problem.
                                if ch == ' ' {
                                    index = count + 1;
                                    break;
                                }
                            }

                            if y > 0 && x == 0 {
                                y -= 1;
                                let buffer = self.document.buffer(y).unwrap();
                                x = buffer.len.saturating_sub(1);
                            } else if index == 0 {
                                x = 0;
                            } else {
                                x = x.saturating_sub(index);
                            }
                        } else {
                            // NOTE: If there is white space at the front, it goes through one character by one character.
                            // It doesn't skip straight to the non white-space character
                            y = y.saturating_sub(1);
                            let buffer = self.document.buffer(y).unwrap();
                            x = buffer.len.saturating_sub(1);
                        }
                    }
                }

                Key::Char('w') => {
                    if event.modifiers.contains(Mod::ALT) {
                        let filename = &self.document.filename;
                        if filename == "[NO FILE OPENED]" {
                            // NOTE: Make a window show up, and let them type the file name, but only the text box this time.
                            todo!("This is a shortcut key to save a file.");
                        } else {
                            let status = StatusMessage::from("File written.");
                            todo!("Can't save file.");
                            // self.document.save_file();
                            self.status = status;
                        }
                    } else {
                        if let Some(row) = self.document.buffer(y) {
                            // NOTE: If there is white space at the front, it goes through one character by one character.
                            // It doesn't skip straight to the non white-space character
                            if let Some(contents) = row.line().get(x..) {
                                let mut index = 0;

                                for (count, currrent_ch) in contents.chars().enumerate() {
                                    if currrent_ch == ' ' {
                                        index = count + 1;
                                        break;
                                    }
                                }

                                if y < doc_height.saturating_sub(1) && x >= width.saturating_sub(1)
                                {
                                    y += 1;
                                    x = 0;
                                } else if index == 0 {
                                    x = width.saturating_sub(1);
                                } else {
                                    x = x.saturating_add(index);
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

                Key::Char('G') => y = doc_height.saturating_sub(1),

                Key::Char('g') => {
                    let has_event = self.has_event(Duration::from_millis(500));

                    if has_event {
                        match event.code {
                            Key::Char('g') => y = 0,
                            _ => (),
                        }
                    }
                }
                Key::Char('0') => x = 0,
                Key::Char('S') => {
                    // Moves to start of the line, minus the whitespace.
                    if let Some(buffer) = self.document.buffer(y) {
                        let contents = buffer.line();
                        x = width.saturating_sub(contents.trim_start().len());
                        x = x.saturating_sub(1);
                    }
                }
                Key::Char('s') => x = width,

                // changing modes
                Key::Char('i') => self.change_mode(Mode::Insert),

                Key::Char(':') => {
                    self.prev_cursor_position = self.cursor_position;
                    self.change_mode(Mode::Command);
                }

                Key::Char('a') => {
                    x = x.saturating_add(1);
                    self.change_mode(Mode::Insert);
                }

                Key::Char('A') => {
                    let buffer = self.document.buffer(y).unwrap();
                    x = buffer.len;

                    self.change_mode(Mode::Insert);
                }

                Key::Char('q') => {
                    if event.modifiers.contains(Mod::CONTROL) {
                        self.should_quit = true;
                    }
                }

                _ => (),
            }
        }

        // adjusts the width the the length of the row
        width = if let Some(buffer) = self.document.buffer(y) {
            buffer.len
        } else {
            0
        };

        // if the cursor is further than the width
        // the x pos of the cursor will be set to the width
        // snapping it to the end of the line.
        if x > width {
            x = width
        }

        self.cursor_position = Position { x, y }
    }

    fn command_mode(&mut self, key: Event) {
        queue!(&mut self.terminal.stdout, Hide).unwrap();

        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = tui::Terminal::new(backend).unwrap();

        // When the window gets to small vertically or horizontally this breaks.
        run_app(&mut terminal, &mut self.app, key);

        self.terminal.stdout.flush().unwrap();
    }

    fn insert_mode(&mut self, key: Event) {
        let Position { mut x, mut y } = &self.cursor_position;
        if let Event::Key(event) = key {
            match event.code {
                Key::Left => {
                    let h_key = self.create_event(Key::Char('h'), Mod::NONE);
                    self.normal_mode(h_key);
                }
                Key::Right => {
                    let l_key = self.create_event(Key::Char('l'), Mod::NONE);
                    self.normal_mode(l_key);
                }

                Key::Up => {
                    let k_key = self.create_event(Key::Char('k'), Mod::NONE);
                    self.normal_mode(k_key);
                }

                Key::Down => {
                    let j_key = self.create_event(Key::Char('j'), Mod::NONE);
                    self.normal_mode(j_key);
                }

                Key::Esc => {
                    self.change_mode(Mode::Normal);
                }

                Key::Backspace => {
                    // NOTE: `above` is called here because after `delete`, `above` will have been modified.
                    let above = self.document.buffer(y - 1).unwrap().clone();
                    let shift = self.document.delete(&self.cursor_position);
                    if !shift {
                        x -= 1;
                    } else {
                        x = above.len;
                        y = y.saturating_sub(1);
                    }
                }

                Key::Enter => {
                    // let width = self.terminal.size().width as usize; yay
                    self.document.enter(&self.cursor_position);
                    let ws_count = self.check_current_then_below_for_whitespace(y);

                    x = ws_count;
                    y += 1;
                    if ws_count != 0 {
                        if let Some(buffer) = self.document.buffer_mut(y) {
                            for ws in 0..(ws_count as u8) {
                                buffer.insert(' ', ws as usize);
                            }
                        }
                    }
                }

                Key::Tab => {
                    x += 4;
                }

                Key::Char(c) => {
                    self.document.insert(c, &self.cursor_position);
                    x += 1;
                }
                _ => (),
            }
        }

        self.cursor_position.x = x;
        self.cursor_position.y = y;
    }

    fn check_current_then_below_for_whitespace(&self, y: usize) -> usize {
        let mut ws_count = 0;
        if let Some(buffer) = self.document.buffer(y) {
            let contents = buffer.line();
            for ch in contents.as_bytes() {
                let ch = *ch as char;
                if ws_count == 0 && !ch.is_whitespace() {
                    ws_count = self.check_current_then_below_for_whitespace(y + 1);
                    break;
                } else if ch.is_whitespace() {
                    ws_count += 1;
                } else if ws_count > 0 && !ch.is_whitespace() {
                    // This one is used to break after the first non-ws character is reached.
                    break;
                }
            }
        }

        ws_count
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

    fn draw_row(&self, buffer: &GapBuffer) {
        let width = self.terminal.size().width as usize;
        let start = self.offset.x;
        let end = self.offset.x.saturating_add(width);
        let row = buffer.render(start, end);

        println!("{}\r", row)
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
        // NOTE: The current issue is that the status bar will make space for the text in the document.
        // That should not happen.
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
        let left_half = width.saturating_sub(status.len());
        let right_half = left_half.saturating_sub(line_number.len());
        let spaces = " ".repeat(right_half);

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
            if let Some(buffer) = self.document.buffer(terminal_row as usize + self.offset.y) {
                self.draw_row(buffer);
            } else if self.document.is_empty() && terminal_row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }
}
