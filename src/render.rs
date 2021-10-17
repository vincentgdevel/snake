use std::io::{Stdout, Write};

use crossterm::{
    cursor::MoveTo,
    style::{Color, SetBackgroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand,
};

use crate::{scene::Scene, SCENE_ROWS};

pub fn render(stdout: &mut Stdout, last_scene: &Scene, curr_scene: &Scene, init: bool) {
    if init {
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        stdout.queue(MoveTo(0, (SCENE_ROWS + 1) as u16)).unwrap();
    }

    for (x, col) in curr_scene.iter().enumerate() {
        for (y, s) in col.iter().enumerate() {
            if *s != last_scene[x][y] || init {
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                print!("{}", *s)
            }
        }
    }
    stdout.flush().unwrap();
}
