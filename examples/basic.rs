use bevy::{prelude::*, render::camera::Projection};
use bevy_atmosphere::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(AtmospherePlugin::<Projection>::new())
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera3dBundle::default(), AtmosphereCamera(None)));
}
