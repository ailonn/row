use bevy::prelude::*;
use bevy_config_cam::*;

mod game_setup;
use crate::game_setup::*;

fn main() {
    App::new()
        //        .insert_resource(Msaa { samples: 4 }) // a tester si c'est vraimment nécessaire
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(ConfigCam) //TODO to set up
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_plugin(InitPlugin)
        .run();
}
