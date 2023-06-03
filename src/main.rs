use glam::IVec2;
use raylib::prelude::*;
use std::time;

const WINDOW_WIDTH: i32 = 1820; //640
const WINDOW_HEIGHT: i32 = 980; //480

const CELL_SIZE: i32 = 60;
const CELL_ON_COLOR: Color = Color::WHITE;
const CELL_OFF_COLOR: Color = Color::BLACK;
const CELL_PADDING: i32 = 6;

const BACKGROUND_COLOR: Color = Color::GRAY;

macro_rules! make_blinker {
    ($grid:expr, $position:expr) => {
        for i in -1..=1 {
            $grid[($position.x) as usize][($position.y + i) as usize].state = true;
        }
    };
}

trait DrawableCell {
    fn update_cell(&mut self, _: bool);
    fn draw(&self, draw_handle: &mut RaylibDrawHandle);
}

impl DrawableCell for Cell {
    fn update_cell(&mut self, new_state: bool) {
        self.state = new_state;
    }
    fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        draw_handle.draw_rectangle(
            self.position.x + CELL_PADDING,
            self.position.y + CELL_PADDING,
            CELL_SIZE - CELL_PADDING,
            CELL_SIZE - CELL_PADDING,
            {
                if self.state {
                    CELL_ON_COLOR
                } else {
                    CELL_OFF_COLOR
                }
            },
        )
    }
}

struct Cell {
    position: IVec2,
    state: bool,
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Hello, World")
        .build();

    let mut grid: Vec<Vec<Cell>> = Vec::new();

    fill_cells_grid(&mut grid);

    make_blinker!(grid, IVec2::new(3, 3));

    let world_start = time::Instant::now();
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(BACKGROUND_COLOR);

        draw_cells(&grid, &mut d);
    }
}

fn _update_cells(cells: &mut Vec<Vec<Cell>>) {
    let deltas: Vec<Vec<bool>> = _get_cell_deltas(cells);
}

fn _get_cell_deltas(cells: &Vec<Vec<Cell>>) -> Vec<Vec<bool>> {
    //for x in 0..get_grid_size().x
}

fn _get_cell_delta(neighbors: &Vec<Cell>, cell: &Cell) -> bool {
    false
}

fn draw_cells(cells: &Vec<Vec<Cell>>, draw_handle: &mut RaylibDrawHandle) {
    for x in 0..get_grid_size().x {
        for y in 0..get_grid_size().y {
            cells[x as usize][y as usize].draw(draw_handle);
        }
    }
}

fn fill_cells_grid(cells: &mut Vec<Vec<Cell>>) {
    for x in 0..get_grid_size().x {
        let mut new_column: Vec<Cell> = Vec::new();

        for y in 0..get_grid_size().y {
            let new_cell = Cell {
                position: IVec2::new(x * CELL_SIZE, y * CELL_SIZE),
                state: false,
            };

            new_column.push(new_cell);
        }

        cells.push(new_column);
    }
}

fn get_grid_size() -> IVec2 {
    let x = WINDOW_WIDTH / CELL_SIZE;
    let y = WINDOW_HEIGHT / CELL_SIZE;

    IVec2::new(x, y)
}
