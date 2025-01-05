use std::collections::HashSet;

use godot::{classes::{ Button, GridContainer, IGridContainer}, obj::{NewGd, WithBaseField}, prelude::*};
use rand::Rng;

use crate::cell::Cell;

#[derive(GodotClass, Debug)]
#[class(base=GridContainer)]
pub struct CellGrid {
    cell_scene: Gd<PackedScene>,
    grid_size: i32,
    cell_size: f32,
    mines_amount: u32,
    cells_opened: u32,

    // This field is same as child nodes of CellGrid
    // So the are Cells from godot perspective.
    // Elements in this vector can be changed.
    // All changes will be applied to child node
    // they are representing in godot
    cells: Vec<Vec<Gd<Cell>>>,

    // This field is used to track which cells
    // were opened and thus should be ignored if clicked
    opened: Vec<(usize, usize)>,

    pub flags: i32,

    base: Base<GridContainer>
}

// Directions is used as usize you have to add + 1
// for it to work. It's a little bit confusing,
// but for me it's better than having overhead
// converting usize to i32 and vise versa
const DIRECTIONS: [(usize, usize); 8] = [
    (2, 2), (2, 1), (2, 0), (1, 2),
    (1, 0), (0, 2), (0, 1), (0, 0),
];

#[godot_api]
impl CellGrid {
    #[signal]
    fn lose_game();
    
    #[signal]
    fn win_game();

    #[signal]
    fn change_flags();

    #[func]
    fn disable_buttons(&mut self) {
        let children = self.base().get_children();
        for child in children.iter_shared() {
            // As far as i know this can't fail
            // because all childs of CellGrid are Cells
            // and Cell base is Button
            let _ = child.try_cast::<Button>().map(|mut b| {
                b.set_disabled(true);
            });
        }
    }

    #[func]
    pub fn init_grid(&mut self) {
        let grid_size = self.grid_size;
        // CellGrid is based on GridContainer godot class
        self.base_mut().set_columns(grid_size);

        // Amount of flags should be equal to mines themselves
        self.flags = self.mines_amount as i32;

        let mut mines = HashSet::new();
        let mut rng = rand::thread_rng();

        while mines.len() < self.mines_amount.try_into().unwrap() {
            // Generating positions x and y for mines.
            // It is a hashset so it guarantees no duplicates occur
            let x = rng.gen_range(0..grid_size) as usize;
            let y = rng.gen_range(0..grid_size) as usize;
            mines.insert((x, y));

            // godot_print!("Mines created: {}", set.len());
        }
        
        // With capacity is used to remove all allocations beside first
        // row vector does allocation every loop, but idk if
        // i can eliminate creating it at all. I have to give ownership
        // and move it to vector, so need to create new one every time
        let mut matrix = Vec::with_capacity(grid_size as usize);

        // Creating grid full of empty cells
        for i in 0..grid_size {
            let mut row = Vec::with_capacity(grid_size as usize);
            for k in 0..grid_size {

                // cell_scene is used like template for creating instances
                // of what it represents. In our case Cell
                let cell_scene = self.cell_scene.instantiate_as::<Button>();
                let mut mine = cell_scene.cast::<Cell>();

                let mut mine_b = mine.bind_mut();
                mine_b.position = (i as usize, k as usize);

                // without drop it complains about 2 mutable references
                // so need to drop it manually
                drop(mine_b);
                row.push(mine);
            }
            matrix.push(row);
        }

        // Inserting mines in our empty grid
        for (i, k) in mines.iter() {

            // unwrapping is safe because mines were created based on grid_size
            // as well as our Grid, so i and k 100% valid indexes
            let mine = matrix.get_mut(*i).unwrap().get_mut(*k).unwrap();

            let mut mine = mine.bind_mut();
            mine.is_mine = true;

            // without drop it complains about 2 mutable references
            // so need to drop it manually
            drop(mine);

            // For every mine we increase mines_around field
            // of cells around in square shape by 1
            for (dx, dy) in DIRECTIONS.iter() {

                // This is mine so just skip
                if *dx == 1 && *dy == 1 {
                    continue;
                }
                
                // Checking if it is valid matrix indexes
                // if not, skip it
                let nx = match (i + 1).checked_sub(*dx) {
                    Some(nx) => {
                        if nx >= self.grid_size as usize {
                            continue;
                        }
                        nx
                    },
                    None => {
                        continue
                    },
                };
                let ny = match (k + 1).checked_sub(*dy) {
                    Some(ny) => {
                        if ny >= self.grid_size as usize {
                            continue;
                        }
                        ny
                    },
                    None => {
                        continue
                    },
                };

                // Safe to unwrap as we checked nx and ny
                // And they are valid indexes of matrix
                let mine = matrix.get_mut(nx).unwrap().get_mut(ny).unwrap();
                let mut mine = mine.bind_mut();
                if !mine.is_mine {
                    mine.mines_around += 1;
                }
            }

        }

        // Finally we have our Grid set up with
        // mines and cells with number of mines
        // around it. So we can render a grid
        for rows in matrix {
            let mut struct_row = vec![];
            for mut cell in rows {
                cell.set_custom_minimum_size(Vector2::from_tuple((self.cell_size, self.cell_size)));

                // Connecting signal from every cell to our CellGrid
                cell.connect("click_on_bomb", &self.base().callable("on_lose_game"));
                cell.connect("open_cells", &self.base().callable("open_cells"));
                cell.connect("add_flag", &self.base().callable("add_flag"));
                cell.connect("sub_flag", &self.base().callable("sub_flag"));

                // Adding cell as child node of CellGrid in Godot
                self.base_mut().add_child(&cell);

                struct_row.push(cell);
            }

            // This will contain all cells
            // and we can access them and modify
            // if needed
            self.cells.push(struct_row);
        }
    }

    #[func]
    fn open_cells(&mut self, index: Variant) {

        // Using option because compiler complains
        // about clicked_cell could be uninitialized.
        // But it's impossible because this function is called
        // on objects that are in vector in CellGrid struct
        // and thus are always valid
        let mut clicked_cell = None;
        'outer: for rows in self.cells.iter_mut() {
            for item in rows.iter_mut() {
                if item.get_index().to_variant() == index {
                    clicked_cell = Some(item);
                    break 'outer;
                }
            }
        }

        // So unwrap here is safe to use
        // No panics will occur
        let clicked_cell = clicked_cell.unwrap();
        let position = clicked_cell.bind_mut().position;

        let mut stack = vec![position];

        while let Some((x , y)) = stack.pop() {
            if self.opened.contains(&(x, y)) {
                continue;
            }
            self.opened.push((x, y));
            
            if self.cells[x][y].bind().mines_around == 0 {
                self.cells[x][y].set_text("");  
            } else {
                let amount = self.cells[x][y].bind().mines_around;
                self.cells[x][y].set_text(&amount.to_string());
            }
            
            // Need to keep track of opened cells
            // as this game win condition relies on:
            // total cells - mines == cells_opened
            self.cells_opened += 1;

            // If cell is not empty cell then continue
            // and don't add cells around it to stack
            // Only 1 cell with mines_around != 0
            // is needed to be open
            if self.cells[x][y].bind().mines_around != 0 {
                continue;
            }

            // Adding cells around current one in 8 directions
            for (dx, dy) in DIRECTIONS.iter() {
                // Checking if directions point to valid
                // index in matrix
                let nx = match (x + 1).checked_sub(*dx) {
                    Some(nx) => {
                        if nx >= self.grid_size as usize {
                            continue;
                        }
                        nx
                    },
                    None => continue,
                };
                let ny = match (y + 1).checked_sub(*dy) {
                    Some(ny) => {
                        if ny >= self.grid_size as usize {
                            continue;
                        }
                        ny
                    },
                    None => continue,
                };

                // godot_print!("{}, {}", nx, ny);
                
                stack.push((nx, ny));
            }
        } 
        // godot_print!("Found {}, mines {}", self.cells_opened, self.mines_amount);
        
        if self.cells_opened == (self.grid_size * self.grid_size) as u32 - self.mines_amount {
            self.on_win_game();
            return;
        }
    }

    #[func]
    fn on_lose_game(&mut self) {
        self.disable_buttons();
        self.base_mut().emit_signal("lose_game", &[]);
    }

    #[func]
    fn on_win_game(&mut self) {
        self.disable_buttons();
        self.base_mut().emit_signal("win_game", &[]);
    }

    #[func]
    pub fn clear_board(&mut self) {
        let cells = self.base_mut().get_children();
        // Free all previous cells they will
        // be deleted on the next frame
        for mut cell in cells.iter_shared() {
            cell.queue_free();
        }

        // Reseting fields of CellGrid structure
        self.cells_opened = 0;
        self.cells = vec![];
        self.opened = vec![];
        self.flags = 0;
    }
    
    #[func]
    fn add_flag(&mut self) {
        // godot_print!("+ flag");
        self.flags += 1;
        self.base_mut().emit_signal("change_flags", &[]);
    }

    #[func]
    fn sub_flag(&mut self) {
        // godot_print!("- flag");
        self.flags -= 1;
        self.base_mut().emit_signal("change_flags", &[]);
    }
}

#[godot_api]
impl IGridContainer for CellGrid {
    fn init(base: Base<GridContainer>) -> Self {
        Self {
            cell_scene: PackedScene::new_gd(),
            grid_size: 10,
            cell_size: 30.0,
            mines_amount: 10,
            cells_opened: 0,
            flags: 0,
            cells: vec![],
            opened: vec![],
            base
        }
    }

    fn ready(&mut self) {
        // need to load cell_scene from godot
        // in order to bind them with our template
        self.cell_scene = load("res://cell.tscn");
    }
}
