use crate::frame::{Drawable, Frame};
use crate::shot::Shot;
use crate::{NUM_COLS, NUM_ROWS};
use std::time::Duration;

pub struct Player {
    x: usize,
    y: usize,
    // 玩家的子彈
    shots: Vec<Shot>,
}

impl Player {
    pub fn new() -> Self {
        Player {
            x: NUM_COLS / 2,
            y: NUM_ROWS - 1,
            shots: Vec::new(),
        }
    }
    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }
    pub fn move_right(&mut self) {
        if self.x < NUM_COLS - 1 {
            self.x += 1;
        }
    }
    pub fn shoot(&mut self) -> bool {
        if self.shots.len() < 2 {
            self.shots.push(Shot::new(self.x, self.y - 1));
            true
        } else {
            false
        }
    }
    pub fn update(&mut self, delta: Duration) {
        // 更新該玩家所發射的子彈
        for shot in self.shots.iter_mut() {
            shot.update(delta);
        }
        // 進行篩選，若shot還沒死掉，則繼續保留，若死掉，則將子彈丟棄
        self.shots.retain(|shot| !shot.dead());
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "A";
        // 迭代玩家的子彈，將每個子彈畫在畫面上面
        for shot in self.shots.iter() {
            shot.draw(frame);
        }
    }
}
