use glam::IVec2;
use raylib::prelude::*;

const WINDOW_WIDTH: i32 = 640; //640
const WINDOW_HEIGHT: i32 = 310; //480

const CELL_SIZE: i32 = 5;
const CELL_ON_COLOR: Color = Color::BLACK;
const CELL_OFF_COLOR: Color = Color::WHITE;

trait DrawableCell {
    fn update_cell(&mut self, _: bool);
    fn draw(&self, draw_handle: &mut RaylibDrawHandle);
}

impl DrawableCell for Cell {
    fn update_cell(&mut self, new_state: bool) {
        self.state = new_state;
    }
    fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        draw_handle.draw_rectangle(self.position.x, self.position.y, CELL_SIZE, CELL_SIZE, {
            if self.state {
                CELL_ON_COLOR
            } else {
                CELL_OFF_COLOR
            }
        })
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

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        draw_cells(&grid, &mut d);

        d.clear_background(Color::WHITE);
    }
}

fn draw_cells(cells: &Vec<Vec<Cell>>, draw_handle: &mut RaylibDrawHandle) {
    for x in 0..get_grid_size().x {
        for y in 0..get_grid_size().y {
            cells[x as usize][y as usize].draw(draw_handle);
        }
    }
}

fn fill_cells_grid(cells: &mut Vec<Vec<Cell>>) {
    let mut flip = false;
    let mut last_column_first_flip = false;
    for x in 0..get_grid_size().x {
        let mut new_column: Vec<Cell> = Vec::new();

        flip = last_column_first_flip;
        last_column_first_flip = !last_column_first_flip;

        for y in 0..get_grid_size().y {
            let new_cell = Cell {
                position: IVec2::new(x * CELL_SIZE, y * CELL_SIZE),
                state: flip,
            };

            flip = !flip;
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
