use std::collections::HashSet;

use godot::{classes::{ Button, GridContainer, IGridContainer}, obj::{NewGd, WithBaseField}, prelude::*};
use rand::Rng;

use crate::mine::Mine;

#[derive(GodotClass, Debug)]
#[class(base=GridContainer)]
pub struct MineGrid {
    mine_scene: Gd<PackedScene>,
    grid_size: i32,
    cell_size: f32,
    mines_amount: u32,
    cells_opened: u32,
    cells: Vec<Vec<Gd<Mine>>>,
    opened: Vec<(usize, usize)>,

    #[base]
    base: Base<GridContainer>
}

#[godot_api]
impl MineGrid {
    #[signal]
    fn lose_game();
    
    #[signal]
    fn win_game();

    #[func]
    fn disable_buttons(&mut self) {
        let children = self.base().get_children();
        for child in children.iter_shared() {
            let _ = child.try_cast::<Button>().map(|mut b| {
                b.set_disabled(true);
            });
        }
        godot_print!("Disabled buttons");
    }

    #[func]
    pub fn init_grid(&mut self) {
        let grid_size = self.grid_size.clone();
        self.base_mut().set_columns(grid_size);

        let mut set = HashSet::new();
        let mut rng = rand::thread_rng();

        while set.len() < self.mines_amount.try_into().unwrap() {
            let x = rng.gen_range(0..grid_size) as usize;
            let y = rng.gen_range(0..grid_size) as usize;
            set.insert((x, y));
            godot_print!("Mines created: {}", set.len());
        }

        let mut matrix = Vec::with_capacity(grid_size as usize);
        for i in 0..grid_size {
            let mut row = Vec::with_capacity(grid_size as usize);
            for k in 0..grid_size {
                let mine_scene = self.mine_scene.instantiate_as::<Button>();
                let mut mine = mine_scene.cast::<Mine>();
                let mut mine_b = mine.bind_mut();
                mine_b.position = (i as usize, k as usize);
                drop(mine_b);
                row.push(mine);
            }
            matrix.push(row);
        }

        for (i, k) in set.iter() {
            let mine = matrix.get_mut(*i).unwrap().get_mut(*k).unwrap();
            let mut mine = mine.bind_mut();
            mine.is_mine = true;
            godot_print!("Mine {}, {}", i, k);
            godot_print!("{}", mine.is_mine);

            drop(mine);

            for c in 0..=2 {
                for r in 0..=2 {
                    if c == 1 && r == 1 {
                        continue;
                    }
                    let c = match (i + 1).checked_sub(c) {
                        Some(c) => c,
                        None => {
                            continue
                        },
                    };
                    let r = match (k + 1).checked_sub(r) {
                        Some(r) => r,
                        None => {
                            continue
                        },
                    };

                    MineGrid::get_mine_if_exists(&mut matrix, c, r).map(|mine| {
                        let mut mine = mine.bind_mut();
                        if !mine.is_mine {
                            mine.mine_amount += 1;
                        }
                    });
                }
            }
        }

        for rows in matrix {
            let mut struct_row = vec![];
            for mut mine in rows {
                mine.set_custom_minimum_size(Vector2::from_tuple((self.cell_size, self.cell_size)));
                mine.connect("click_on_bomb", &self.base().callable("on_lose_game"));
                mine.connect("open_cells", &self.base().callable("open_cells"));
                self.base_mut().add_child(&mine);
                struct_row.push(mine);
            }
            self.cells.push(struct_row);
        }
    }

    #[func]
    fn open_cells(&mut self, index: Variant) {
        let mut clicked_cell = None;
        'outer: for rows in self.cells.iter_mut() {
            for item in rows.iter_mut() {
                if item.get_index().to_variant() == index {
                    clicked_cell = Some(item);
                    break 'outer;
                }
            }
        }

        
        let clicked_cell = clicked_cell.unwrap();
        let position = clicked_cell.bind_mut().position;

        const DIRECTIONS: [(usize, usize); 8] = [
            (2, 2), (2, 1), (2, 0), (1, 2),
            (1, 0), (0, 2), (0, 1), (0, 0),
        ];

        let mut stack = vec![position];

        while let Some((x , y)) = stack.pop() {
            if self.opened.contains(&(x, y)) {
                continue;
            }
            self.opened.push((x, y));
            
            if self.cells[x][y].bind().mine_amount == 0 {
                self.cells[x][y].set_text("");  
            } else {
                let amount = self.cells[x][y].bind().mine_amount;
                self.cells[x][y].set_text(&amount.to_string());
            }

            self.cells_opened += 1;

            if self.cells[x][y].bind().mine_amount != 0 {
                continue;
            }

            for (dx, dy) in DIRECTIONS.iter() {
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

                godot_print!("{}, {}", nx, ny);
                
                stack.push((nx, ny));
            }
        } 
        godot_print!("Found {}, mines {}", self.cells_opened, self.mines_amount);
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

    fn get_mine_if_exists(matrix: &mut Vec<Vec<Gd<Mine>>>, i: usize, k: usize) -> Option<&mut Gd<Mine>> {
        Some(matrix.get_mut(i)?.get_mut(k)?)
    }
}

#[godot_api]
impl IGridContainer for MineGrid {
    fn init(base: Base<GridContainer>) -> Self {
        Self {
            mine_scene: PackedScene::new_gd(),
            grid_size: 10,
            cell_size: 30.0,
            mines_amount: 10,
            cells_opened: 0,
            cells: vec![],
            opened: vec![],
            base
        }
    }

    fn ready(&mut self) {
        self.mine_scene = load("res://mine.tscn");
    }
}
