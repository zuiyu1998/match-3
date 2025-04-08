use bevy::prelude::*;
use match_3::GamePlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, GamePlugin));

    app.run();
}
