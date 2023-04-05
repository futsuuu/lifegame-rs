use std::{
    io::{self, stdout, Write},
    time::Duration,
};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    queue,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use rand::{distributions::Uniform, prelude::Distribution};

static WIDTH: u8 = 50;
static HEIGHT: u8 = 30;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    queue!(stdout, cursor::Hide)?;

    let mut rng = rand::thread_rng();
    let random = Uniform::from(0..=1);
    let mut stage = vec![];
    for y in 0..HEIGHT {
        let mut row = vec![];
        for x in 0..WIDTH {
            let is_alive = random.sample(&mut rng) == 1;
            let new_cell: Cell = Cell {
                x: x as u16,
                y: y as u16,
                is_alive,
                is_alive_cache: is_alive,
            };
            row.push(new_cell);
        }
        stage.push(row);
    }

    loop {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                if let KeyCode::Char('q') = key_event.code {
                    break;
                }
            }
        }

        update(&mut stage);

        queue!(stdout, cursor::SavePosition)?;
        for row in &stage {
            for cell in row {
                queue!(stdout, Print(if cell.is_alive { "██" } else { "  " }))?;
            }
            queue!(stdout, cursor::MoveToNextLine(1))?;
        }
        queue!(stdout, cursor::RestorePosition)?;

        stdout.flush()?;
    }

    queue!(stdout, cursor::Show)?;
    disable_raw_mode()?;
    Ok(())
}

fn update(stage: &mut [Vec<Cell>]) {
    fn is_alive(stage: &Vec<Vec<Cell>>, pos: (i8, i8)) -> bool {
        let upos = (pos.0 as usize, pos.1 as usize);
        0 <= pos.1
            && 0 <= pos.0
            && upos.1 < stage.len()
            && upos.0 < stage[upos.1].len()
            && stage[upos.1][upos.0].is_alive_cache
    }

    let stage_clone = stage.to_owned();
    for row in stage.iter_mut() {
        for cell in row {
            let mut counter = 0;
            for rel_pos in [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ] {
                if is_alive(
                    &stage_clone,
                    (
                        cell.x as i8 + rel_pos.0 as i8,
                        cell.y as i8 + rel_pos.1 as i8,
                    ),
                ) {
                    counter += 1;
                }
            }
            if cell.is_alive {
                if counter != 2 && counter != 3 {
                    cell.is_alive = false
                }
            } else if counter == 3 || counter == 6 {
                cell.is_alive = true
            }
        }
    }

    for row in stage.iter_mut() {
        for cell in row {
            cell.is_alive_cache = cell.is_alive
        }
    }
}

#[derive(Clone)]
struct Cell {
    x: u16,
    y: u16,
    is_alive: bool,
    is_alive_cache: bool,
}
