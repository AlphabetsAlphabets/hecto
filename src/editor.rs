use super::terminal;
use terminal::Terminal;

use termion::event::Key;

pub struct Editor {
    should_quit: bool,
    terminal: Terminal
}

impl Editor {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::new().expect("Failed to initialize terminal.")
        }
    }

    pub fn run(&mut self) {
        loop {
            if let Err(error) = Terminal::refresh_screen() {
                Terminal::clear_screen();
                panic!(error);
            };

            if self.should_quit {
                break;
            } else {
                self.draw_tildes();
                Terminal::cursor_position(0, 0);
            }

            if let Err(error) = self.process_keypress() {
                panic!(error);
                Terminal::cursor_position(0, 0);
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

    fn draw_tildes(&self) {
        for _ in 0..self.terminal.size().1 {
            println!("~\r");
        }
    }
}
