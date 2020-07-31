mod tetris;

use std::time::{Duration, Instant};
use tetris::Game;

use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window,
};

const RATIO: u8 = 1;
const BLOCK: u32 = RATIO as u32 * 25;

const B_LEN: usize = 10; // Board
const S_LEN: usize = 4; // Side Menu

const B_HEI: usize = 20;
const N_HEI: usize = 10;
const H_HEI: usize = 4;

const PAD: u8 = 2;

// Sreen Dimesions (Ratio of 1 mean 500x700)
const SCR_LEN: u32 = BLOCK * (PAD as usize * 3 + B_LEN + S_LEN) as u32;
const SCR_HEI: u32 = BLOCK * (PAD as usize * 2 + B_HEI) as u32;

// Draws the initial screen and board setup
fn init_screen(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(70, 70, 70));
    canvas.clear();
    canvas.present();
}

// Draws the Falling and Place Minos in Game Board
fn draw_board(game: &Game, canvas: &mut Canvas<Window>) {
    let left = PAD as u32 * BLOCK;
    let top = left;

    for i in 0..B_LEN {
        for (pos, j) in (0..B_HEI).rev().enumerate() {
            let c = &game.board[i][j].color;
            canvas.set_draw_color(Color::RGB(c[0], c[1], c[2]));

            let x = left + BLOCK * i as u32;
            let y = top + BLOCK * pos as u32;

            let r = Rect::new(x as i32, y as i32, BLOCK, BLOCK);
            let _ = canvas.fill_rect(r);
        }
    }
}

// Draws the held and three next minos to fall
fn draw_next_and_held(game: &Game, canvas: &mut Canvas<Window>) {
    let next_top = PAD as u32 * BLOCK;
    let held_top = next_top + (PAD as u32 + N_HEI as u32) * BLOCK;
    let left = BLOCK * (PAD as usize * 2 + B_LEN) as u32;

    for i in 0..S_LEN {
        let x = left + BLOCK * i as u32;

        for (pos, j) in (0..N_HEI).rev().enumerate() {
            let c = &game.next_board[i][j];
            canvas.set_draw_color(Color::RGB(c[0], c[1], c[2]));

            let y = next_top + BLOCK * pos as u32;

            let r = Rect::new(x as i32, y as i32, BLOCK, BLOCK);
            let _ = canvas.fill_rect(r);
        }

        for (pos, j) in (0..H_HEI).rev().enumerate() {
            let c = &game.held_board[i][j];
            canvas.set_draw_color(Color::RGB(c[0], c[1], c[2]));

            let y = held_top + BLOCK * pos as u32;

            let r = Rect::new(x as i32, y as i32, BLOCK, BLOCK);
            let _ = canvas.fill_rect(r);
        }
    }
}

fn update_all(game: &Game, canvas: &mut Canvas<Window>) {
    draw_board(game, canvas);
    draw_next_and_held(game, canvas);
    canvas.present();
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Rustris", SCR_LEN, SCR_HEI)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    init_screen(&mut canvas);

    let mut game = Game::new();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let input_delay = Duration::from_millis(150);
    let mut fall_delay = Duration::from_millis(game.drop_speed as u64);
    let (mut input_time, mut fall_time) = (Instant::now(), Instant::now());

    'Rustris: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'Rustris;
                }

                Event::KeyDown { keycode: code, .. } => {
                    if input_time.elapsed() >= input_delay {
                        match code {
                            Some(Keycode::Up) => {
                                game.rotate();
                            }
                            Some(Keycode::Down) => {
                                game.drop();
                            }
                            Some(Keycode::Left) => {
                                game.shift(false);
                            }
                            Some(Keycode::Right) => {
                                game.shift(true);
                            }
                            Some(Keycode::PageUp) | Some(Keycode::PageDown) => {
                                game.hold();
                            }
                            _ => {}
                        }

                        game.update_pos();
                        update_all(&game, &mut canvas);
                        input_time = Instant::now();
                    }
                }
                _ => {}
            }
        }

        if fall_time.elapsed() > fall_delay {
            if game.fall_or_place() {
                game.update_pos();
            }
            update_all(&game, &mut canvas);
            fall_time = Instant::now();
            fall_delay = Duration::from_millis(game.drop_speed as u64);
        }
    }

    println!("{}", game.drop_speed);
}
