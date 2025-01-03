use godot::prelude::*;

mod grid;
mod mine;
mod main_scene;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
