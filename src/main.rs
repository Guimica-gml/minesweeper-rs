use std::fmt::Display;
use rand::{self, Rng};

use sdl2;
use sdl2::ttf::Font;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::mouse::MouseButton;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;

const GRID_WIDTH: usize = 8;
const GRID_HEIGHT: usize = 8;
const BOMBS_AMOUNT: u32 = 10;
const FONT_SIZE: u16 = 32;

const FIELD_WIDTH: u32 = WINDOW_WIDTH / GRID_WIDTH as u32;
const FIELD_HEIGHT: u32 = WINDOW_HEIGHT / GRID_HEIGHT as u32;

#[derive(Debug, Clone, Copy)]
enum CellValue {
    Bomb,
    Num(i32),
}

impl Display for CellValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CellValue::Bomb => write!(f, "Bomb!"),
            CellValue::Num(num) => write!(f, "{}", num)
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Cell {
    value: CellValue,
    visible: bool,
    has_flag: bool,
}

impl Cell {
    fn new(num: i32) -> Self {
        Self {
            value: CellValue::Num(num),
            visible: false,
            has_flag: false,
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

type MineCells = [[Cell; GRID_WIDTH]; GRID_HEIGHT];

fn create_mine_cells(bombs_amount: u32) -> MineCells {
    assert!(
        bombs_amount < (GRID_WIDTH * GRID_HEIGHT) as u32,
        "Amount of bombs should be less then the amount of cells"
    );

    let mut cells = [[Cell::new(0); GRID_WIDTH]; GRID_HEIGHT];
    let mut rand = rand::thread_rng();
    let mut bombs_placed = 0;

    while bombs_placed < bombs_amount {
        let randx = rand.gen_range(0..GRID_WIDTH);
        let randy = rand.gen_range(0..GRID_HEIGHT);

        if let CellValue::Num(_) = cells[randy][randx].value {
            cells[randy][randx].value = CellValue::Bomb;
            bombs_placed += 1;
        }
    }

    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            if let CellValue::Bomb = cells[y][x].value {
                continue;
            }

            let mut bomb_neighbours = 0;
            foreach_neighbor(x, y, &mut |nx, ny| {
                if let CellValue::Bomb = cells[ny][nx].value {
                    bomb_neighbours += 1;
                }
            });

            cells[y][x].value = CellValue::Num(bomb_neighbours);
        }
    }

    cells
}

fn mine_make_cell_visible(
    mine_cells: &mut MineCells,
    x: usize,
    y: usize,
) {
    if mine_cells[y][x].visible {
        return;
    }

    mine_cells[y][x].visible = true;
    mine_cells[y][x].has_flag = false;

    if let CellValue::Num(0) = mine_cells[y][x].value {
        foreach_neighbor(x, y, &mut |nx, ny| {
            mine_make_cell_visible(mine_cells, nx, ny);
        });
    }
}

fn foreach_neighbor(
    x: usize,
    y: usize,
    func: &mut dyn FnMut(usize, usize) -> (),
) {
    if x > 0 { func(x - 1, y); }
    if y > 0 { func(x, y - 1); }
    if x < GRID_WIDTH - 1 { func(x + 1, y); }
    if y < GRID_HEIGHT - 1 { func(x, y + 1); }
    if y < GRID_HEIGHT - 1 && x > 0 { func(x - 1, y + 1); }
    if y > 0 && x < GRID_WIDTH - 1 { func(x + 1, y - 1); }
    if y > 0 && x > 0 { func(x - 1, y - 1); }
    if y < GRID_HEIGHT - 1 && x < GRID_WIDTH - 1 { func(x + 1, y + 1); }
}

fn draw_text(
    canvas: &mut Canvas<Window>,
    font: &Font,
    text: String,
    pos: (i32, i32),
) -> Result<(), String> {
    let surface = font
        .render(&text)
        .blended(Color::WHITE)
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();
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

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

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

    let font = ttf_context.load_font("font/Iosevka.ttf", FONT_SIZE)?;

    let mut event_pump = sdl_context.event_pump()?;
    let mut mine_cells = create_mine_cells(BOMBS_AMOUNT);

    'gameloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'gameloop,
                Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                    let x = x as usize / FIELD_WIDTH as usize;
                    let y = y as usize / FIELD_HEIGHT as usize;

                    if mouse_btn == MouseButton::Left {
                        mine_make_cell_visible(&mut mine_cells, x, y);
                    }
                    else if mouse_btn == MouseButton::Right && !mine_cells[y][x].visible {
                        mine_cells[y][x].has_flag = !mine_cells[y][x].has_flag;
                    }
                }
                _ => {}
            }
        }

        canvas.clear();
        canvas.set_draw_color(Color::WHITE);

        // Draw horizontal lines
        for y in 0..GRID_HEIGHT {
            canvas.draw_line(
                (0, (y + 1) as i32 * FIELD_HEIGHT as i32),
                (WINDOW_WIDTH as i32, (y + 1) as i32 * FIELD_HEIGHT as i32)
            )?;
        }

        // Draw vertical lines
        for x in 0..GRID_WIDTH {
            canvas.draw_line(
                ((x + 1) as i32 * FIELD_WIDTH as i32, 0),
                ((x + 1) as i32 * FIELD_WIDTH as i32, WINDOW_HEIGHT as i32)
            )?;
        }

        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let posx = (x as u32 * FIELD_WIDTH + FIELD_WIDTH / 2) as i32;
                let posy = (y as u32 * FIELD_HEIGHT + FIELD_HEIGHT / 2) as i32;

                if mine_cells[y][x].has_flag {
                    draw_text(
                        &mut canvas,
                        &font,
                        "Flag!".to_string(),
                        (posx, posy)
                    )?;
                }
                else if mine_cells[y][x].visible {
                    draw_text(
                        &mut canvas,
                        &font,
                        mine_cells[y][x].to_string(),
                        (posx, posy)
                    )?;
                }
            }
        }

        canvas.set_draw_color(Color::BLACK);
        canvas.present();
    }

    Ok(())
}
