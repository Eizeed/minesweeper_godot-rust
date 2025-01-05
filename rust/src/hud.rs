use godot::{classes::{Button, CanvasLayer, ICanvasLayer, Label}, obj::WithBaseField, prelude::*};

#[derive(GodotClass)]
#[class(base=CanvasLayer)]
pub struct Hud {
    base: Base<CanvasLayer>
}

#[godot_api]
impl Hud {
    #[signal]
    fn on_start_game_button_press();

    #[func]
    pub fn show_message(&mut self, text: GString) {
        let mut label = self.base().get_node_as::<Label>("Message");
        label.set_text(&text);
        label.show();
    }

    #[func]
    // Is triggered on every timeout signal from main_scene
    pub fn update_time(&mut self, time: u32) {
        let mut timer_label = self.base_mut().get_node_as::<Label>("GameTimer");
        timer_label.set_text(&time.to_string());
    }

    #[func]
    fn on_start_game_button_press(&mut self) {
        self.base_mut().emit_signal("on_start_game_button_press", &[]);
        let mut button = self.base_mut().get_node_as::<Button>("StartGame");
        button.hide();
    }

    #[func]
    // Is triggered when start button is pressed
    pub fn hide_message(&mut self) {
        let mut label = self.base().get_node_as::<Label>("Message");
        label.hide();
    }

    #[func]
    // Triggered on gameover
    // either win or lose
    pub fn show_start_button(&mut self) {
        let mut button = self.base_mut().get_node_as::<Button>("StartGame");
        button.show();
    }

    #[func]
    // Triggered every time change_flags
    // signal from CellGrid is triggered
    // and on start game to set flags for
    // the first time
    pub fn update_flags(&mut self, flags: i32) {
        let mut flags_label = self.base_mut().get_node_as::<Label>("FlagsAmount");
        flags_label.set_text(&format!("{flags}🚩"));
    }
}

#[godot_api]
impl ICanvasLayer for Hud {
    fn init(base: Base<CanvasLayer>) -> Self {
        Self {
            base
        }
    }
}
