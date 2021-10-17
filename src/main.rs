use std::{
    error::Error,
    io,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use snake::{
    apples::Apples,
    render::render,
    scene::{new_scene, Drawable},
    snake::Snake,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = io::stdout();

    //initialize
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // //render channel
    let (tx, rx) = mpsc::channel();

    let render_handle = thread::spawn(move || {
        let mut last_scene = new_scene();
        let mut stdout = io::stdout();
        render(&mut stdout, &last_scene, &last_scene, true);
        loop {
            let curr_scene = match rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render(&mut stdout, &last_scene, &curr_scene, false);
            last_scene = curr_scene;
        }
    });

    let mut snake = Snake::new();
    let mut apples = Apples::new(&snake);
    let mut instant = Instant::now();

    'game_loop: loop {
        let delta = instant.elapsed();
        instant = Instant::now();

        let mut curr_scene = new_scene();

        while event::poll(Duration::default())? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Left => snake.move_left(),
                    KeyCode::Right => snake.move_right(),
                    KeyCode::Up => snake.move_up(),
                    KeyCode::Down => snake.move_down(),
                    KeyCode::Esc => {
                        break 'game_loop;
                    }
                    _ => {}
                }
            }
        }

        //update
        snake.update(delta);
        snake.detect_eats(&mut apples);

        //draw and render
        let drawables: Vec<&dyn Drawable> = vec![&snake, &apples];
        for drawable in drawables {
            drawable.draw(&mut curr_scene)
        }

        let _ = tx.send(curr_scene);
        thread::sleep(Duration::from_millis(100));

        //victory
        if snake.colided() {
            break 'game_loop;
        }
    }

    //cleanup
    drop(tx);
    render_handle.join().unwrap();
    stdout.execute(Show)?; //show cursor
    stdout.execute(LeaveAlternateScreen)?; //leave game screen
    terminal::disable_raw_mode()?; //disable input capture

    Ok(())
}
