use bevy::{prelude::*, window::WindowResized};

pub fn on_resize_system(mut resize_reader: MessageReader<WindowResized>) {
    for e in resize_reader.read() {
        // When resolution is being changed
        println!("{:.1} x {:.1}", e.width, e.height);
    }
}
