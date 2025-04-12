use bevy::prelude::*;
use rand::{Rng, rng};

pub const WIDTH: f32 = 576.0;
pub const HEIGHT: f32 = 1024.0;

//棋子的大小
pub const PIECE_SIZE: f32 = 64.0;
pub const PIECE_WIDTH: i32 = 8;
pub const PIECE_HEIGHT: i32 = 10;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Startup, setup_game);
        app.add_systems(Startup, setup_game_ui);
        app.add_systems(Startup, spawn_all_pieces);
    }
}

#[derive(Component)]
pub struct PiecePosition {
    x: i32,
    y: i32,
}

impl PiecePosition {
    pub fn get_position(&self) -> Vec3 {
        let x = self.x as f32 * PIECE_SIZE - 4.0 * PIECE_SIZE + PIECE_SIZE / 2.0;
        let y = self.y as f32 * PIECE_SIZE - 5.0 * PIECE_SIZE;

        Vec3 { x, y, z: 1.0 }
    }
}

fn spawn_all_pieces(mut commands: Commands, asset_server: Res<AssetServer>) {
    let piece_handles: Vec<Handle<Image>> = vec![
        asset_server.load("Pieces/Blue Piece.png"),
        asset_server.load("Pieces/Green Piece.png"),
        asset_server.load("Pieces/Light Green Piece.png"),
        asset_server.load("Pieces/Orange Piece.png"),
        asset_server.load("Pieces/Pink Piece.png"),
        asset_server.load("Pieces/Yellow Piece.png"),
    ];

    let mut rng = rng();

    for x in 0..PIECE_WIDTH {
        for y in 0..PIECE_HEIGHT {
            let piece_position = PiecePosition { x, y };
            let position = piece_position.get_position();

            let index = rng.random_range(0..piece_handles.len());

            commands.spawn((
                Sprite {
                    image: piece_handles[index].clone(),
                    custom_size: Some(Vec2::splat(PIECE_SIZE)),
                    ..default()
                },
                Transform {
                    translation: position,
                    ..default()
                },
            ));
        }
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
}

fn setup_game_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let top = asset_server.load("UI/Top UI v 2.png");
    let bottom = asset_server.load("UI/Bottom UI v 2.png");

    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        })
        .with_children(|builder| {
            builder.spawn((
                ImageNode::new(top),
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(192.0),
                    ..default()
                },
            ));

            builder.spawn((
                ImageNode::new(bottom),
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(92.0),
                    ..default()
                },
            ));
        });
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
