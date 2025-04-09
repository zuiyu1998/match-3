use bevy::prelude::*;

pub const WIDTH: f32 = 576.0;
pub const HEIGHT: f32 = 1024.0;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Startup, setup_game);
    }
}

fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    let bg = asset_server.load("UI/Backgrounds/background 2.png");

    commands.spawn((
        Sprite {
            image: bg,
            custom_size: Some(Vec2::new(WIDTH, HEIGHT)),
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..default()
        },
    ));
    commands.spawn(game_ui(&asset_server));
}

fn game_ui(asset_server: &AssetServer) -> impl Bundle + use<> {
    let top = asset_server.load("UI/Top UI v 2.png");
    let bottom = asset_server.load("UI/Bottom UI v 2.png");

    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        children![
            (
                ImageNode::new(top),
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(192.0),
                    ..default()
                },
            ),
            (
                ImageNode::new(bottom),
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(92.0),
                    ..default()
                },
            )
        ],
    )
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
