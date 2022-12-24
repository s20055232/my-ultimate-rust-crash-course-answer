use crate::frame::Frame;
use crossterm::cursor::MoveTo;
use crossterm::style::{Color, SetBackgroundColor};
use crossterm::terminal::{Clear, ClearType};
use crossterm::QueueableCommand;
use std::io::{Stdout, Write};

pub fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force: bool) {
    if force {
        // 將已經預先設定好的Frame，用藍色進行填補
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        // 將terminal/console screen buffer清除，此時螢幕會變成全藍色
        stdout.queue(Clear(ClearType::All)).unwrap();
        // 將Frame改用黑色進行填補
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
        // stdout.queue(SetBackgroundColor(Color::Green)).unwrap();
    }
    for (x, col) in curr_frame.iter().enumerate() {
        for (y, s) in col.iter().enumerate() {
            if *s != last_frame[x][y] || force {
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                print!("{}", *s);
            }
        }
    }
    stdout.flush().unwrap();
}
