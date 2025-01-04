use godot::{classes::{Button, CanvasLayer, ICanvasLayer, Label, Timer}, obj::WithBaseField, prelude::*};

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
        let mut timer = self.base_mut().get_node_as::<Timer>("MessageTimer");
        timer.start();
    }

    #[func]
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
    fn on_message_timer_timeout(&mut self) {
        let mut label = self.base().get_node_as::<Label>("Message");
        label.hide();
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
