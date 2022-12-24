use crate::frame::{Drawable, Frame};
use rusty_time::timer::Timer;
use std::time::Duration;

pub struct Shot {
    pub x: usize,
    pub y: usize,
    pub explode: bool,
    timer: Timer,
}

impl Shot {
    pub fn new(x: usize, y: usize) -> Self {
        Shot {
            x: x,
            y: y,
            explode: false,
            timer: Timer::from_millis(50),
        }
    }
    pub fn update(&mut self, delta: Duration) {
        // update：剩餘時間扣除delta的時間
        self.timer.update(delta);
        // 若ready為true，代表時間到了
        if self.timer.ready && !self.explode {
            if self.y > 0 {
                self.y -= 1;
            }
            // 計時器重新開始倒數
            self.timer.reset();
        }
    }
    pub fn explode(&mut self) {
        self.explode = true;
        // 用來等待shot的爆炸動畫跑完
        self.timer = Timer::from_millis(250);
    }
    pub fn dead(&self) -> bool {
        // shot爆炸了，且250 millisecond的計時器跑完
        // 或者shot超出了畫面上緣
        (self.explode && self.timer.ready) || self.y == 0
    }
}

impl Drawable for Shot {
    fn draw(&self, frame: &mut Frame) {
        // 更新shot在frame上面的位置，如果爆炸了，則變成*
        // 若還沒爆炸，則為 |
        frame[self.x][self.y] = if self.explode { " * " } else { "|" }
    }
}
