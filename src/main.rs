use glam::IVec2;
use raylib::prelude::*;

const WINDOW_WIDTH: i32 = 500;
const WINDOW_HEIGHT: i32 = 500;

const CELL_SIZE: i32 = 30;
const CELL_ON_COLOR: Color = Color::WHITE;
const CELL_OFF_COLOR: Color = Color::BLACK;
const CELL_PADDING: i32 = 2;

const BACKGROUND_COLOR: Color = Color::GRAY;

const FRAMES_PER_SECOND: f32 = 2.0;
const FRAME_TIME_MILLIS: f32 = 1.0 / FRAMES_PER_SECOND;

macro_rules! make_blinker {
    ($grid:expr, $position:expr) => {
        for i in -1..=1 {
            $grid[($position.x) as usize][($position.y + i) as usize].state = true;
        }
    };
}

macro_rules! make_glider {
    ($grid:expr, $position:expr) => {
        $grid[($position.x) as usize][($position.y) as usize].state = true;
        $grid[($position.x + 1) as usize][($position.y + 1) as usize].state = true;
        $grid[($position.x + 1) as usize][($position.y + 2) as usize].state = true;
        $grid[($position.x) as usize][($position.y + 2) as usize].state = true;
        $grid[($position.x + -1) as usize][($position.y + 2) as usize].state = true;
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

    make_glider!(grid, IVec2::new(1, 0));

    let mut elapsed_time = 0.0;
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(BACKGROUND_COLOR);

        while elapsed_time > FRAME_TIME_MILLIS {
            update_cells(&mut grid);
            elapsed_time -= FRAME_TIME_MILLIS;
        }

        draw_cells(&grid, &mut d);
        elapsed_time += d.get_frame_time();
    }
}

fn update_cells(cells: &mut Vec<Vec<Cell>>) {
    let new_states: Vec<Vec<bool>> = get_grid_next_state(cells);
    for x in 0..get_grid_size().x {
        for y in 0..get_grid_size().y {
            cells[x as usize][y as usize].state = new_states[x as usize][y as usize];
        }
    }
}

fn get_grid_next_state(cells: &Vec<Vec<Cell>>) -> Vec<Vec<bool>> {
    let mut states: Vec<Vec<bool>> = Vec::new();
    for _ in 0..get_grid_size().x {
        let column: Vec<bool> = Vec::new();
        states.push(column);
    }

    for x in 0..get_grid_size().x {
        for y in 0..get_grid_size().y {
            states[x as usize].push(get_cell_next_state(cells, IVec2 { x, y }));
        }
    }

    states
}

fn get_cell_next_state(cells: &Vec<Vec<Cell>>, cell_index: IVec2) -> bool {
    let num_living = get_num_of_living_neighbors(cells, cell_index);
    if cells[cell_index.x as usize][cell_index.y as usize].state == true {
        match num_living {
            2 | 3 => true,
            _ => false,
        }
    } else {
        match num_living {
            3 => true,
            _ => false,
        }
    }
}

fn get_num_of_living_neighbors(cells: &Vec<Vec<Cell>>, cell_index: IVec2) -> i32 {
    let mut living: i32 = 0;

    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }

            let next_x: i32 = cell_index.x + i;
            let next_y: i32 = cell_index.y + j;

            if next_x < 0 || next_y < 0 {
                continue;
            }

            if next_x >= get_grid_size().x || next_y >= get_grid_size().y {
                continue;
            }

            if cells[next_x as usize][next_y as usize].state == false {
                continue;
            }

            living += 1;
        }
    }

    living
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
