use godot::{classes::Timer, obj::WithBaseField, prelude::*};

use crate::{grid::CellGrid, hud};

#[derive(GodotClass)]
#[class(base=Node)]
struct Main {
    time: u32,
    base: Base<Node>
}

#[godot_api]
impl Main {
    #[func]
    fn on_lose_game(&mut self) {
        let mut hud = self.base_mut().get_node_as::<hud::Hud>("Hud");
        let mut hud = hud.bind_mut();
        hud.show_message("You Lose".into());
        hud.show_start_button();

        let mut timer = self.base().get_node_as::<Timer>("GameTimer");
        timer.stop();
    }

    #[func]
    fn on_win_game(&mut self) {
        let mut hud = self.base_mut().get_node_as::<hud::Hud>("Hud");
        let mut hud = hud.bind_mut();
        hud.show_message("You Win".into());
        hud.show_start_button();

        let mut timer = self.base().get_node_as::<Timer>("GameTimer");
        timer.stop();
    }

    #[func]
    fn on_game_timer_timeout(&mut self) {
        self.time += 1;

        let mut hud = self.base_mut().get_node_as::<hud::Hud>("Hud");
        hud.bind_mut().update_time(self.time);
    }

    #[func]
    fn on_start_game(&mut self) {
        // Reseting time
        self.time = 0;

        // Hiding message and reseting time in hud
        let mut hud = self.base_mut().get_node_as::<hud::Hud>("Hud");
        let mut hud = hud.bind_mut();
        hud.hide_message();
        hud.update_time(self.time);

        // Reseting CelLGrid
        let mut grid = self.base_mut().get_node_as::<CellGrid>("CellGrid");
        let mut grid = grid.bind_mut();
        grid.clear_board();
        grid.init_grid();

        // Setting flags for the first time
        hud.update_flags(grid.flags);
    
        let mut timer = self.base().get_node_as::<Timer>("GameTimer");
        timer.start();
    }

    #[func]
    // Triggered on every change_flags
    // signal trigger
    fn on_change_flags(&mut self) {

        // Getting flags from CellGrid, which is 
        // main_scene child node
        let grid = self.base_mut().get_node_as::<CellGrid>("CellGrid");
        let flags = grid.bind().flags;
        
        let mut hud = self.base_mut().get_node_as::<hud::Hud>("Hud");
        hud.bind_mut().update_flags(flags);
    }
}

#[godot_api]
impl INode for Main {
    fn init(base: Base<Node>) -> Self {
        Self {
            time: 0,
            base
        }
    }
}
