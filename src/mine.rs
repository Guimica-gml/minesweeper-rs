use rand::{self, Rng};

#[derive(Debug, Clone, Copy)]
pub enum CellValue {
    Bomb,
    Num(i32),
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    value: CellValue,
    visible: bool,
    has_flag: bool,
}

impl Cell {
    pub fn value(&self) -> &CellValue { &self.value }
    pub fn visible(&self) -> bool { self.visible }
    pub fn has_flag(&self) -> bool { self.has_flag }

    fn new(num: i32) -> Self {
        Self {
            value: CellValue::Num(num),
            visible: false,
            has_flag: false,
        }
    }
}

pub struct Minesweeper {
    width: usize,
    height: usize,
    cells: Vec<Vec<Cell>>,
}

impl Minesweeper {
    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }

    pub fn new(width: usize, height: usize, bombs_amount: u32) -> Self {
        assert!(
            bombs_amount < (width * height) as u32,
            "Amount of bombs should be less than the amount of cells"
        );

        let mut cells = vec![vec![Cell::new(0); width]; height];
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
                foreach_neighbor(x, y, width, height, |nx, ny| {
                    if let CellValue::Bomb = cells[ny][nx].value {
                        bomb_neighbours += 1;
                    }
                });

                cells[y][x].value = CellValue::Num(bomb_neighbours);
            }
        }

        Self { width, height, cells }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> &Cell {
        return &self.cells[y][x];
    }

    pub fn make_cell_visible(&mut self, x: usize, y: usize) {
        if self.cells[y][x].visible {
            return;
        }

        self.cells[y][x].visible = true;
        self.cells[y][x].has_flag = false;

        if let CellValue::Num(0) = self.cells[y][x].value {
            foreach_neighbor(x, y, self.width, self.height, |nx, ny| {
                self.make_cell_visible(nx, ny);
            });
        }
    }

    pub fn make_all_cells_visible(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.make_cell_visible(x, y);
            }
        }
    }

    pub fn toggle_flag_in_cell(&mut self, x: usize, y: usize) {
        if !self.cells[y][x].visible {
            self.cells[y][x].has_flag = !self.cells[y][x].has_flag;
        }
    }
}

fn foreach_neighbor(x: usize, y: usize, w: usize, h: usize, mut func: impl FnMut(usize, usize)) {
    if x > 0 { func(x - 1, y); }
    if y > 0 { func(x, y - 1); }
    if x < w - 1 { func(x + 1, y); }
    if y < h - 1 { func(x, y + 1); }
    if y < h - 1 && x > 0 { func(x - 1, y + 1); }
    if y > 0 && x < w - 1 { func(x + 1, y - 1); }
    if y > 0 && x > 0 { func(x - 1, y - 1); }
    if y < h - 1 && x < w - 1 { func(x + 1, y + 1); }
}
