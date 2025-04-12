mod piece;
mod splash_state;
mod state;

pub use piece::*;
pub use splash_state::*;
pub use state::*;

use bevy::prelude::*;

pub const WIDTH: f32 = 576.0;
pub const HEIGHT: f32 = 1024.0;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_plugins((SplashStatePlugin, PiecePlugin))
            .add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

mod test {
    #[test]
    fn test() {
        let a = 5;

        assert_eq!(a, 5);
    }
}
