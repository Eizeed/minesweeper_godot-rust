use godot::{classes::{Button, IButton}, global::MouseButton, obj::WithBaseField, prelude::*};

#[derive(GodotClass)]
#[class(base=Button)]
pub struct Mine {
    pub mine_amount: i32,
    pub is_mine: bool,
    pub is_flagged: bool,
    pub position: (usize, usize),

    base: Base<Button>,
}

#[godot_api]
impl Mine {
    #[signal]
    fn click_on_bomb();

    #[signal]
    fn open_cells(path: Variant);

    #[func]
    fn on_click_cell(&mut self) {
        let input = Input::singleton();
        if input.is_action_pressed("left_click") {
            if self.base().get_text() != "·".into() {
                return;
            }
            self.left_click();
        } else if input.is_action_pressed("right_click") {
            if self.base().get_text() != "·".into() && self.base().get_text() != "🚩".into() {
                return;
            }
            self.right_click();
        }
    }

    fn left_click(&mut self) {
        let path = self.base().get_path();
        let mut cell = self.base_mut().get_node_as::<Mine>(&path);

        if self.is_mine {
            cell.set_text("💣");
            self.base_mut().emit_signal("click_on_bomb", &[]);
        } else {
            let index = self.base().get_index();
            self.base_mut().emit_signal("open_cells", &[index.to_variant()]);
        }
    }

    fn right_click(&mut self) { 
        if self.is_flagged {
            self.is_flagged = !self.is_flagged;
            self.base_mut().set_text("·");
        } else {
            self.is_flagged = true;
            self.base_mut().set_text("🚩");
        }
    }
}

#[godot_api]
impl IButton for Mine {
    fn init(base: Base<Button>) -> Self {
        Self {
            mine_amount: 0,
            position: (0, 0),
            is_mine: false,
            is_flagged: false,
            base
        }
    }
}
