use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{event, ExecutableCommand};
use invaders::frame::Drawable;
use invaders::player::Player;
use invaders::{frame, render};
use rusty_audio::Audio;
use std::error::Error;
use std::sync::mpsc;
use std::time::{Duration, Instant};
use std::{io, thread};
// 由於使用crossterm將mac terminal變更為raw mode無法成功
// 因此改為使用termion
use invaders::invaders::Invaders;
use termion::raw::IntoRawMode;

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "sounds/original/explode.wav");
    audio.add("lose", "sounds/original/lose.wav");
    audio.add("move", "sounds/original/move.wav");
    audio.add("pew", "sounds/original/pew.wav");
    audio.add("startup", "sounds/original/startup.wav");
    audio.add("win", "sounds/original/win.wav");
    audio.play("startup");
    // 建立一個標準輸出，並變更為raw mode
    // terminal有兩種模式，raw mode或是cooked mode，cooked mode為預設的模式
    // cooked mode就是我們平常使用terminal會使用的模式，stdout會接受我們的指令
    // 舉例來說：`cd ../ <Enter>`，若我們沒有點擊enter，stdout將不會執行我們的輸入
    // 而raw mode則相反，terminal會立即解析我們的所有輸入，並執行對應的操作
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    // 進入另一個視窗
    stdout.execute(EnterAlternateScreen)?;
    // 將游標隱藏
    stdout.execute(Hide)?;
    // channel用於threads之間的溝通
    let (render_tx, render_rx) = mpsc::channel();
    // 建立一個專門更新畫面的執行緒
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        while let Ok(curr_frame) = render_rx.recv() {
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();
    // 專門處理遊戲邏輯的loop，會生成畫面並透過channel傳給負責更新畫面的子執行緒
    'gameloop: loop {
        let delta = instant.elapsed();
        // 得到過去的時間之後，立刻重置時間
        instant = Instant::now();
        let mut curr_frame = frame::new_frame();
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if player.shoot() {
                            audio.play("pew");
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => (),
                }
            }
        }
        // 根據過去的時間，對畫面進行更新
        player.update(delta);
        // 如果計時器到了，畫面要更新的話，音樂播放一次
        if invaders.update(delta) {
            audio.play("move");
        }
        // player.draw(&mut curr_frame);
        // invaders.draw(&mut curr_frame);
        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders];
        for drawable in drawables {
            drawable.draw(&mut curr_frame);
        }
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));
    }
    // 將傳送者從作用域移除，確保rust可以正常關閉，舊版本的rust會有此問題
    // 新版本的rust此指令可以不必輸入
    drop(render_tx);
    // 阻塞，等待子執行緒結束
    render_handle.join().unwrap();
    // 阻塞，直到聲音都播放完畢
    audio.wait();
    // 將游標顯示
    stdout.execute(Show)?;
    // 離開另起的視窗
    stdout.execute(LeaveAlternateScreen)?;
    // 結束
    Ok(())
}
