extern crate rustbox;

use model::commit::Commit;
use std::error::Error;
use std::default::Default;
use self::rustbox::{Color, RustBox, Key};

pub struct Screen {
    rustbox: RustBox,
    commits: Vec<Commit>,
    query: String
}

impl Screen {
    pub fn new(commits: Vec<Commit>) -> Screen {
        let rustbox = match RustBox::init(Default::default()) {
            Result::Ok(v) => v,
            Result::Err(e) => panic!("{}", e),
        };

        Screen {
            rustbox: rustbox,
            commits: commits,
            query: String::new()
        }
    }

    pub fn draw(&mut self) {
        self.rustbox.clear();
        self.draw_query();
        self.draw_commits();
        self.rustbox.present();

        loop {
            self.rustbox.clear();

            match self.rustbox.poll_event(false) {
                Ok(rustbox::Event::KeyEvent(key)) => {
                    match key {
                        Some(Key::Char(c)) => {
                            self.query.push(c);
                        },
                        Some(Key::Backspace) => {
                            self.query.pop();

                        },
                        Some(Key::Esc) => {
                            break;
                        },
                        _ => {
                        }
                    }
                },
                Err(e) => panic!("{}", e.description()),
                _ => { }
            }

            self.draw_query();
            self.draw_commits();
            self.rustbox.present();
        }
    }

    fn draw_query(&self) {
        let message = format!("QUERY> {}", &self.query);
        self.rustbox.print(0, 0, rustbox::RB_NORMAL, Color::Green, Color::Black, &message);
    }

    fn draw_commits(&self) {
        for i in 0..self.commits.len() {
            let commit = self.commits.get(i).unwrap();
            self.rustbox.print(0, i + 1, rustbox::RB_NORMAL, Color::Green, Color::Black, &commit.message);
        }
    }
}
