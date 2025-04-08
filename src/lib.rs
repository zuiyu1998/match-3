use bevy::prelude::*;

pub const WIDTH: f32 = 288.0;
pub const HEIGHT: f32 = 512.0;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Startup, setup_game);
    }
}

fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(game_ui(&asset_server));
}

fn game_ui(asset_server: &AssetServer) -> impl Bundle + use<> {
    let bg = asset_server.load("UI/Backgrounds/background 2.png");
    let top = asset_server.load("UI/Top UI v 2.png");
    let bottom = asset_server.load("UI/Bottom UI v 2.png");

    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        children![(
            ImageNode::new(bg),
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),

                ..default()
            },
            children![
                (
                    ImageNode::new(top),
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(96.0),
                        ..default()
                    },
                ),
                (
                    ImageNode::new(bottom),
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(46.0),
                        ..default()
                    },
                )
            ],
        )],
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
