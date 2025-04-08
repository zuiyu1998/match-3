use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

mod test {
    #[test]
    fn test() {
        let a = 5;

        assert_eq!(a, 5);
    }
}
