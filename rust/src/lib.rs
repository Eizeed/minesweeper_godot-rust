use godot::prelude::*;

mod grid;
mod cell;
mod hud;
mod main_scene;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
