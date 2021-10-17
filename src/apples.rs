use std::usize;

use rand::{distributions::Uniform, prelude::Distribution};

use crate::{scene::Drawable, snake::Snake, SCENE_COLS, SCENE_ROWS};

pub struct Apple {
    pub x: usize,
    pub y: usize,
    pub eaten: bool,
}

pub struct Apples {
    pub apples: Vec<Apple>,
}

impl Apple {
    pub fn new(snake: &Snake) -> Self {
        let mut rng = rand::thread_rng();
        let x_die = Uniform::from(1..SCENE_COLS);
        let y_die = Uniform::from(1..SCENE_ROWS);

        let xes: Vec<usize> = snake.parts.iter().map(|part| part.x).collect();
        let yes: Vec<usize> = snake.parts.iter().map(|part| part.y).collect();

        let mut x;
        let mut y;

        loop {
            x = x_die.sample(&mut rng);
            if !xes.contains(&x) {
                break;
            }
        }

        loop {
            y = y_die.sample(&mut rng);
            if !yes.contains(&y) {
                break;
            }
        }
        Self { x, y, eaten: false }
    }
}

impl Apples {
    pub fn new(snake: &Snake) -> Self {
        let mut apples = Vec::new();
        apples.push(Apple::new(snake));
        Self { apples }
    }

    pub fn apple_eaten_at(&mut self, x: usize, y: usize) -> bool {
        if let Some(i) = self
            .apples
            .iter()
            .position(|apple| (apple.x == x) && (apple.y == y))
        {
            self.apples.remove(i);
            true
        } else {
            false
        }
    }
}

impl Drawable for Apples {
    fn draw(&self, scene: &mut crate::scene::Scene) {
        for apple in self.apples.iter() {
            scene[apple.x][apple.y] = "A"
        }
    }
}
