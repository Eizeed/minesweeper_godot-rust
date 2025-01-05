use godot::{classes::{Button, IButton}, obj::WithBaseField, prelude::*};

#[derive(GodotClass)]
#[class(base=Button)]
pub struct Cell {
    pub mines_around: i32,
    pub is_mine: bool,
    pub is_flagged: bool,

    // Idk if it's possible to not use
    // this field to get position, but for now
    // i have to do this in order to get
    // position of clicked cell
    pub position: (usize, usize),

    base: Base<Button>,
}

#[godot_api]
impl Cell {
    #[signal]
    fn click_on_bomb();

    #[signal]
    // This one accept index of object
    // works like id ig
    // and pass it upwards in handler function
    fn open_cells(index: Variant);

    #[signal]
    fn add_flag();
    
    #[signal]
    fn sub_flag();

    #[func]
    // "Router" to handler click with for
    // right and left mouse buttons
    fn on_cell_click(&mut self) {
        let input = Input::singleton();

        // For this if statement this code duplication is
        // necessary. If i bring check for dot on top it will
        // eliminate flag as well
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
        // get_path() gets path from the root node to object
        // that called this method
        let path = self.base().get_path();
        let mut cell = self.base_mut().get_node_as::<Cell>(&path);

        if self.is_mine {
            cell.set_text("💣");
            self.base_mut().emit_signal("click_on_bomb", &[]);
        } else {
            let index = self.base().get_index();

            // We can pass arguments with signals but it needs
            // to be variant. Idk how to pass arrays, because
            // i didn't need them here
            self.base_mut().emit_signal("open_cells", &[index.to_variant()]);
        }
    }

    fn right_click(&mut self) { 
        // Reversing state of flagged
        // and calling signals to change
        // flag field in Grid
        if self.is_flagged {
            self.is_flagged = !self.is_flagged;
            self.base_mut().set_text("·");
            self.base_mut().emit_signal("add_flag", &[]);
        } else {
            self.is_flagged = true;
            self.base_mut().set_text("🚩");
            self.base_mut().emit_signal("sub_flag", &[]);
        }
    }
}

#[godot_api]
impl IButton for Cell {
    fn init(base: Base<Button>) -> Self {
        Self {
            mines_around: 0,
            position: (0, 0),
            is_mine: false,
            is_flagged: false,
            base
        }
    }
}
