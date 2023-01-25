use sdl2;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::rwops::RWops;
use sdl2::ttf::{self, Font};
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::video::{Window, WindowContext};
use sdl2::pixels::Color;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::mouse::MouseButton;

use super::mine::*;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;

const MINE_WIDTH: usize = 8;
const MINE_HEIGHT: usize = 8;
const MINE_BOMBS_AMOUNT: u32 = 10;

const FONT_TTF_BYTES: &[u8] = include_bytes!("../font/Iosevka.ttf");
const FLAG_PNG_BYTES: &[u8] = include_bytes!("../img/flag.png");
const BOMB_PNG_BYTES: &[u8] = include_bytes!("../img/bomb.png");

macro_rules! rect {
    ($x: expr, $y: expr, $w: expr, $h: expr) => {
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    };
}

fn draw_text(
    canvas: &mut Canvas<Window>,
    texture_creator: &TextureCreator<WindowContext>,
    font: &Font,
    text: &str,
    pos: (i32, i32),
) -> Result<(), String> {
    let surface = font
        .render(text)
        .blended(Color::WHITE)
        .map_err(|e| e.to_string())?;

    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    let (w, h) = surface.rect().size();
    let target = Rect::new(
        pos.0 - w as i32 / 2,
        pos.1 - h as i32 / 2,
        w, h,
    );

    canvas.copy(&texture, None, Some(target))?;
    Ok(())
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let ttf_context = ttf::init().map_err(|e| e.to_string())?;
    let _ = image::init(InitFlag::PNG)?;

    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Minesweeper", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;
    let mut minesweeper = Minesweeper::new(MINE_WIDTH, MINE_HEIGHT, MINE_BOMBS_AMOUNT);

    let field_width: u32 = WINDOW_WIDTH / minesweeper.width() as u32;
    let field_height: u32 = WINDOW_HEIGHT / minesweeper.height() as u32;

    let font_size = (u32::min(field_height, field_width) as f32 * 0.4) as u16;
    let font = ttf_context.load_font_from_rwops(RWops::from_bytes(FONT_TTF_BYTES)?, font_size)?;
    let texture_creator = canvas.texture_creator();

    let flag_texture = texture_creator.load_texture_bytes(FLAG_PNG_BYTES)?;
    let bomb_texture = texture_creator.load_texture_bytes(BOMB_PNG_BYTES)?;

    'gameloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'gameloop,
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    minesweeper = Minesweeper::new(MINE_WIDTH, MINE_HEIGHT, MINE_BOMBS_AMOUNT);
                }
                Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                    let x = x as usize / field_width as usize;
                    let y = y as usize / field_height as usize;

                    if mouse_btn == MouseButton::Left {
                        if let CellValue::Bomb = minesweeper.get_cell(x, y).value() {
                            minesweeper.make_all_cells_visible();
                        }
                        else {
                            minesweeper.make_cell_visible(x, y);
                        }
                    }
                    else if mouse_btn == MouseButton::Right {
                        minesweeper.toggle_flag_in_cell(x, y);
                    }
                }
                _ => {}
            }
        }

        canvas.clear();
        canvas.set_draw_color(Color::WHITE);

        for y in 0..minesweeper.height() {
            for x in 0..minesweeper.width() {
                let posx = (x as u32 * field_width + field_width / 2) as i32;
                let posy = (y as u32 * field_height + field_height / 2) as i32;

                if minesweeper.get_cell(x, y).has_flag() {
                    let img_size = (u32::min(field_height, field_width) as f32 * 0.7) as i32;
                    let target = rect!(posx - img_size / 2, posy - img_size / 2, img_size, img_size);
                    canvas.copy(&flag_texture, None, Some(target))?;
                }
                else if minesweeper.get_cell(x, y).visible() {
                    canvas.set_draw_color(Color::RGB(50, 50, 50));
                    let background = rect!(
                        posx - field_width as i32 / 2,
                        posy - field_height as i32 / 2,
                        field_width,
                        field_height
                    );
                    canvas.fill_rect(background)?;
                    canvas.set_draw_color(Color::WHITE);

                    match minesweeper.get_cell(x, y).value() {
                        CellValue::Bomb => {
                            let img_size = (u32::min(field_height, field_width) as f32 * 0.7) as i32;
                            let target = rect!(posx - img_size / 2, posy - img_size / 2, img_size, img_size);
                            canvas.copy(&bomb_texture, None, Some(target))?;
                        }
                        CellValue::Num(num) if *num != 0 => {
                            draw_text(&mut canvas, &texture_creator, &font, &num.to_string(), (posx, posy))?;
                        }
                        _ => {}
                    }
                }
            }

            // Draw horizontal lines
            for y in 0..minesweeper.height() {
                canvas.draw_line(
                    (0, (y + 1) as i32 * field_height as i32),
                    (WINDOW_WIDTH as i32, (y + 1) as i32 * field_height as i32)
                )?;
            }

            // Draw vertical lines
            for x in 0..minesweeper.width() {
                canvas.draw_line(
                    ((x + 1) as i32 * field_width as i32, 0),
                    ((x + 1) as i32 * field_width as i32, WINDOW_HEIGHT as i32)
                )?;
            }
        }

        canvas.set_draw_color(Color::BLACK);
        canvas.present();
    }

    Ok(())
}
