use std::time::Duration;

use rusty_time::prelude::Timer;

use crate::{
    apples::{Apple, Apples},
    scene::{Drawable, Scene},
    SCENE_COLS, SCENE_ROWS, SNAKE_SPEED,
};

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SnakePart {
    pub x: usize,
    pub y: usize,
}

pub struct Snake {
    pub parts: Vec<SnakePart>,
    timer: Timer,
    direction: Direction,
    colided: bool,
}

impl Snake {
    pub fn new() -> Self {
        let head = SnakePart {
            x: SCENE_COLS / 2,
            y: SCENE_ROWS / 2,
        };

        let part1 = SnakePart {
            x: head.x,
            y: head.y - 1,
        };

        let part2 = SnakePart {
            x: part1.x,
            y: part1.y - 1,
        };

        let part3 = SnakePart {
            x: part2.x,
            y: part2.y - 1,
        };

        Self {
            parts: vec![head, part1, part2, part3],
            timer: Timer::from_millis(SNAKE_SPEED),
            direction: Direction::DOWN,
            colided: false,
        }
    }

    pub fn update(&mut self, delta: Duration) {
        self.timer.update(delta);

        if self.timer.ready {
            let mut head = self.parts.first_mut().unwrap();
            let mut last_part = head.clone();

            match self.direction {
                Direction::LEFT => {
                    if head.x > 0 {
                        head.x -= 1
                    } else {
                        self.colided = true
                    }
                }
                Direction::UP => {
                    if head.y > 0 {
                        head.y -= 1
                    } else {
                        self.colided = true
                    }
                }
                Direction::DOWN => {
                    if head.y < SCENE_ROWS - 1 {
                        head.y += 1
                    } else {
                        self.colided = true
                    }
                }
                Direction::RIGHT => {
                    if head.x < SCENE_COLS - 1 {
                        head.x += 1
                    } else {
                        self.colided = true
                    }
                }
            }

            for (pos, part) in self.parts.iter_mut().enumerate() {
                if pos == 0 {
                    continue;
                }
                let curr_part = part.clone();
                part.x = last_part.x;
                part.y = last_part.y;

                last_part = curr_part
            }

            self.detect_colitions();
            self.timer.reset();
        }
    }

    pub fn move_left(&mut self) {
        if !matches!(self.direction, Direction::RIGHT) {
            self.direction = Direction::LEFT
        }
    }

    pub fn move_right(&mut self) {
        if !matches!(self.direction, Direction::LEFT) {
            self.direction = Direction::RIGHT
        }
    }

    pub fn move_up(&mut self) {
        if !matches!(self.direction, Direction::DOWN) {
            self.direction = Direction::UP
        }
    }

    pub fn move_down(&mut self) {
        if !matches!(self.direction, Direction::UP) {
            self.direction = Direction::DOWN
        }
    }

    pub fn colided(&mut self) -> bool {
        self.colided
    }

    fn detect_colitions(&mut self) {
        let head = self.parts.first().unwrap();
        let occurs = self.parts.iter().filter(|&part| part == head).count();
        if occurs > 1 {
            self.colided = true;
        }
    }

    pub fn detect_eats(&mut self, apples: &mut Apples) {
        let head = &self.parts[0];
        if apples.apple_eaten_at(head.x, head.y) {
            apples.apples.push(Apple::new(self));

            let last_part = self.parts.last().unwrap().clone();
            self.parts.push(last_part)
        }
    }
}

impl Drawable for Snake {
    fn draw(&self, scene: &mut Scene) {
        for part in self.parts.iter() {
            scene[part.x][part.y] = "0"
        }
    }
}
