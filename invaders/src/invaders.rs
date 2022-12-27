use crate::frame::{Drawable, Frame};
use crate::{NUM_COLS, NUM_ROWS};
use rusty_time::timer::Timer;
use std::cmp::max;
use std::time::Duration;

pub struct Invader {
    pub x: usize,
    pub y: usize,
}

pub struct Invaders {
    pub army: Vec<Invader>,
    move_timer: Timer,
    direction: i32,
}

impl Invaders {
    pub fn new() -> Self {
        let mut army = Vec::new();
        for x in 0..NUM_COLS {
            for y in 0..NUM_ROWS {
                // 限制敵人生成的位置
                // x軸要大於1且小於欄位數減2，且每兩格生成一次
                // y軸要大於0且小於列數9，每兩格生成一次
                if (x > 1)
                    && (x < NUM_COLS - 2)
                    && (y > 0)
                    && (y < 9)
                    && (x % 2 == 0)
                    && (y % 2 == 0)
                {
                    army.push(Invader { x, y })
                }
            }
        }
        Self {
            army,
            move_timer: Timer::from_millis(2000),
            direction: 1,
        }
    }
    pub fn update(&mut self, delta: Duration) -> bool {
        // 先減去已過的時間
        self.move_timer.update(delta);
        // 如果時間到了
        if self.move_timer.ready {
            // 計時器重置
            self.move_timer.reset();
            let mut downwards = false;
            // direction = -1 代表向右走，direction = 1 代表向左走
            if self.direction == -1 {
                // 找到所有點最小的，如果碰到右邊牆壁，代表要往下走
                let min_x = self.army.iter().map(|invader| invader.x).min().unwrap_or(0);
                if min_x == 0 {
                    self.direction = 1;
                    downwards = true;
                }
            } else {
                // ..最大..，..左邊..，..
                let max_x = self.army.iter().map(|invader| invader.x).max().unwrap_or(0);
                if max_x == NUM_COLS - 1 {
                    self.direction = -1;
                    downwards = true;
                }
            }
            if downwards {
                // 若往下走，速度要加快，但限速是250 milliseconds
                let new_duration = max(self.move_timer.duration.as_millis() - 250, 250);
                self.move_timer = Timer::from_millis(new_duration as u64);
                for invader in self.army.iter_mut() {
                    invader.y += 1;
                }
            } else {
                for invader in self.army.iter_mut() {
                    invader.x = ((invader.x as i32) + self.direction) as usize;
                }
            }
            return true;
        }
        false
    }
}

impl Default for Invaders {
    fn default() -> Self {
        Self::new()
    }
}

impl Drawable for Invaders {
    fn draw(&self, frame: &mut Frame) {
        for invader in self.army.iter() {
            frame[invader.x][invader.y] = if (self.move_timer.time_left.as_secs_f32()
                / self.move_timer.duration.as_secs_f32())
                > 0.5
            {
                "X"
            } else {
                "+"
            };
        }
    }
}
