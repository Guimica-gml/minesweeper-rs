use std::io::stdout;
use crossterm::{self, execute, Result};
use crossterm::cursor;
use crossterm::style::{self, Color};
use crossterm::terminal::{self, ClearType, enable_raw_mode, disable_raw_mode};
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};

use super::mine::*;

pub fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut minesweeper = Minesweeper::new(12, 5, 10);

    let mut cursor_x: usize = 0;
    let mut cursor_y: usize = 0;

    execute!(
        stdout(),
        terminal::Clear(ClearType::All),
        cursor::Hide
    )?;

    'main: loop {
        execute!(stdout(), cursor::MoveTo(0, 0))?;

        for y in 0..minesweeper.height() {
            for x in 0..minesweeper.width() {
                let mut fg_color: Color;
                let bg_color: Color;
                let char: char;

                if x == cursor_x && y == cursor_y {
                    fg_color = Color::Black;
                    bg_color = Color::White;
                }
                else {
                    fg_color = Color::Reset;
                    bg_color = Color::Reset;
                }

                if minesweeper.get_cell(x, y).has_flag() {
                    fg_color = Color::DarkBlue;
                    char = 'F';
                }
                else if minesweeper.get_cell(x, y).visible() {
                    char = match minesweeper.get_cell(x, y).value() {
                        CellValue::Bomb => {
                            fg_color = Color::Red;
                            'B'
                        }
                        CellValue::Num(num) => {
                            fg_color = Color::DarkGrey;
                            num.to_string().chars().nth(0).unwrap()
                        }
                    };
                }
                else {
                    char = '#';
                }

                execute!(
                    stdout(),
                    style::SetBackgroundColor(bg_color),
                    style::SetForegroundColor(fg_color),
                    style::Print(char),
                )?;
            }
            print!("\r\n");
        }

        execute!(
            stdout(),
            style::SetBackgroundColor(Color::Reset),
            style::SetForegroundColor(Color::Reset),
        )?;

        let event = read()?;
        match event {
            Event::Key(KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL, .. }) => break 'main,
            Event::Key(KeyEvent { code: KeyCode::Left, .. }) if cursor_x > 0 => cursor_x -= 1,
            Event::Key(KeyEvent { code: KeyCode::Right, .. }) if cursor_x < minesweeper.width() - 1 => cursor_x += 1,
            Event::Key(KeyEvent { code: KeyCode::Up, .. }) if cursor_y > 0 => cursor_y -= 1,
            Event::Key(KeyEvent { code: KeyCode::Down, .. }) if cursor_y < minesweeper.height() - 1 => cursor_y += 1,
            Event::Key(KeyEvent { code: KeyCode::Enter, .. }) => {
                if let CellValue::Bomb = minesweeper.get_cell(cursor_x, cursor_y).value() {
                    minesweeper.make_all_cells_visible();
                }
                else {
                    minesweeper.make_cell_visible(cursor_x, cursor_y);
                }
            }
            Event::Key(KeyEvent { code: KeyCode::Char(' '), .. }) => {
                minesweeper.toggle_flag_in_cell(cursor_x, cursor_y);
            }
            _ => {}
        }
    }

    execute!(stdout(), cursor::Show)?;
    disable_raw_mode()?;
    Ok(())
}
