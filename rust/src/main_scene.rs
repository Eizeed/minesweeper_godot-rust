use godot::{classes::Timer, obj::WithBaseField, prelude::*};

use crate::{grid::MineGrid, hud};



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
        hud.bind_mut().show_message("You Lose".into());
        let mut timer = self.base().get_node_as::<Timer>("GameTimer");
        timer.stop();
    }

    #[func]
    fn on_win_game(&mut self) {
        let mut hud = self.base_mut().get_node_as::<hud::Hud>("Hud");
        hud.bind_mut().show_message("You Win".into());
        let mut timer = self.base().get_node_as::<Timer>("GameTimer");
        timer.stop();
    }

    #[func]
    fn on_game_timer_timeout(&mut self) {
        let mut hud = self.base_mut().get_node_as::<hud::Hud>("Hud");
        self.time += 1;
        hud.bind_mut().update_time(self.time);
    }

    #[func]
    fn on_start_game(&mut self) {
        let mut grid = self.base_mut().get_node_as::<MineGrid>("MineGrid");
        grid.bind_mut().init_grid();

        self.time = 0;
        let mut timer = self.base().get_node_as::<Timer>("GameTimer");
        timer.start();
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

    fn ready(&mut self) {

    }
}
