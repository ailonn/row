use bevy::prelude::{App, DefaultPlugins};
mod tutorial;
use crate::tutorial::tutorial::HelloPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}
