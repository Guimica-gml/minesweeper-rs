use sdl2;
use sdl2::ttf::Font;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::mouse::MouseButton;
use rand::{self, Rng};

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;
const FONT_SIZE: u16 = 32;

macro_rules! rect {
    ($x: expr, $y: expr, $w: expr, $h: expr) => {
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    };
}

#[derive(Debug, Clone, Copy)]
enum CellValue {
    Bomb,
    Num(i32),
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

struct Minesweeper {
    width: usize,
    height: usize,
    cells: Vec<Vec<Cell>>,
}

fn create_mine_cells(width: usize, height: usize, bombs_amount: u32) -> Minesweeper {
    assert!(
        bombs_amount < (width * height) as u32,
        "Amount of bombs should be less then the amount of cells"
    );

    let mut minesweeper = Minesweeper {
        width,
        height,
        cells: vec![vec![Cell::new(0); width]; height],
    };

    let cells = &mut minesweeper.cells;
    let mut rand = rand::thread_rng();
    let mut bombs_placed = 0;

    while bombs_placed < bombs_amount {
        let randx = rand.gen_range(0..width);
        let randy = rand.gen_range(0..height);

        if let CellValue::Num(_) = cells[randy][randx].value {
            cells[randy][randx].value = CellValue::Bomb;
            bombs_placed += 1;
        }
    }

    for y in 0..height {
        for x in 0..width {
            if let CellValue::Bomb = cells[y][x].value {
                continue;
            }

            let mut bomb_neighbours = 0;
            foreach_neighbor(x, y, minesweeper.width, minesweeper.height, |nx, ny| {
                if let CellValue::Bomb = cells[ny][nx].value {
                    bomb_neighbours += 1;
                }
            });

            cells[y][x].value = CellValue::Num(bomb_neighbours);
        }
    }

    minesweeper
}

fn mine_make_cell_visible(
    minesweeper: &mut Minesweeper,
    x: usize,
    y: usize,
) {
    if minesweeper.cells[y][x].visible {
        return;
    }

    minesweeper.cells[y][x].visible = true;
    minesweeper.cells[y][x].has_flag = false;

    if let CellValue::Num(0) = minesweeper.cells[y][x].value {
        foreach_neighbor(x, y, minesweeper.width, minesweeper.height, |nx, ny| {
            mine_make_cell_visible(minesweeper, nx, ny);
        });
    }
}

fn mine_make_all_cells_visible(minesweeper: &mut Minesweeper) {
    for y in 0..minesweeper.height {
        for x in 0..minesweeper.width {
            mine_make_cell_visible(minesweeper, x, y);
        }
    }
}

fn mine_toggle_flag_in_cell(
    minesweeper: &mut Minesweeper,
    x: usize,
    y: usize,
) {
    if !minesweeper.cells[y][x].visible {
        minesweeper.cells[y][x].has_flag = !minesweeper.cells[y][x].has_flag;
    }
}

fn foreach_neighbor(
    x: usize, y: usize,
    w: usize, h: usize,
    mut func: impl FnMut(usize, usize) -> (),
) {
    if x > 0 { func(x - 1, y); }
    if y > 0 { func(x, y - 1); }
    if x < w - 1 { func(x + 1, y); }
    if y < h - 1 { func(x, y + 1); }
    if y < h - 1 && x > 0 { func(x - 1, y + 1); }
    if y > 0 && x < w - 1 { func(x + 1, y - 1); }
    if y > 0 && x > 0 { func(x - 1, y - 1); }
    if y < h - 1 && x < w - 1 { func(x + 1, y + 1); }
}

fn draw_text(
    canvas: &mut Canvas<Window>,
    font: &Font,
    text: &str,
    pos: (i32, i32),
) -> Result<(), String> {
    let surface = font
        .render(text)
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
    let mut minesweeper = create_mine_cells(8, 8, 10);

    let field_width: u32 = WINDOW_WIDTH / minesweeper.width as u32;
    let field_height: u32 = WINDOW_HEIGHT / minesweeper.height as u32;

    'gameloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'gameloop,
                Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                    let x = x as usize / field_width as usize;
                    let y = y as usize / field_height as usize;

                    if mouse_btn == MouseButton::Left {
                        if let CellValue::Bomb = minesweeper.cells[y][x].value {
                            mine_make_all_cells_visible(&mut minesweeper);
                        }
                        else {
                            mine_make_cell_visible(&mut minesweeper, x, y);
                        }
                    }
                    else if mouse_btn == MouseButton::Right {
                        mine_toggle_flag_in_cell(&mut minesweeper, x, y);
                    }
                }
                _ => {}
            }
        }

        canvas.clear();
        canvas.set_draw_color(Color::WHITE);

        for y in 0..minesweeper.height {
            for x in 0..minesweeper.width {
                let posx = (x as u32 * field_width + field_width / 2) as i32;
                let posy = (y as u32 * field_height + field_height / 2) as i32;

                if minesweeper.cells[y][x].has_flag {
                    draw_text(&mut canvas, &font, "Flag!", (posx, posy))?;
                }
                else if minesweeper.cells[y][x].visible {
                    canvas.set_draw_color(Color::RGB(50, 50, 50));
                    let background = rect!(
                        posx - field_width as i32 / 2,
                        posy - field_height as i32 / 2,
                        field_width,
                        field_height
                    );
                    canvas.fill_rect(background)?;
                    canvas.set_draw_color(Color::WHITE);

                    if let CellValue::Bomb = minesweeper.cells[y][x].value {
                        draw_text(&mut canvas, &font, "Bomb!", (posx, posy))?;
                    }
                    else if let CellValue::Num(num) = minesweeper.cells[y][x].value {
                        if num != 0 {
                            draw_text(&mut canvas, &font, &num.to_string(), (posx, posy))?;
                        }
                    }
                }
            }

            // Draw horizontal lines
            for y in 0..minesweeper.height {
                canvas.draw_line(
                    (0, (y + 1) as i32 * field_height as i32),
                    (WINDOW_WIDTH as i32, (y + 1) as i32 * field_height as i32)
                )?;
            }

            // Draw vertical lines
            for x in 0..minesweeper.width {
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
