use bevy::prelude::{App, DefaultPlugins, ClearColor, Color};
mod tutorial;
use crate::tutorial::tutorial::HelloPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}
