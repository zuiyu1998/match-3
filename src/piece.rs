use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use rand::{Rng, rng, rngs::ThreadRng};

use crate::{AppState, HEIGHT, WIDTH};

//棋子的大小
pub const PIECE_SIZE: f32 = 64.0;
pub const PIECE_WIDTH: i32 = 8;
pub const PIECE_HEIGHT: i32 = 10;

pub struct PiecePlugin;

impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::Game),
            (setup_game, setup_game_ui, spawn_all_pieces),
        );
    }
}

#[derive(AssetCollection, Resource)]
pub struct PieceAssets {
    #[asset(path = "Pieces/Blue Piece.png")]
    blue: Handle<Image>,
    #[asset(path = "Pieces/Green Piece.png")]
    green: Handle<Image>,
    #[asset(path = "Pieces/Light Green Piece.png")]
    light_green: Handle<Image>,
    #[asset(path = "Pieces/Orange Piece.png")]
    orange: Handle<Image>,
    #[asset(path = "Pieces/Pink Piece.png")]
    pink: Handle<Image>,
    #[asset(path = "Pieces/Yellow Piece.png")]
    yellow: Handle<Image>,
}

impl PieceAssets {
    pub fn get_random_image_handle(&self, rng: &mut ThreadRng) -> Handle<Image> {
        let piece_handles: Vec<Handle<Image>> = vec![
            self.blue.clone(),
            self.green.clone(),
            self.light_green.clone(),
            self.orange.clone(),
            self.pink.clone(),
            self.yellow.clone(),
        ];

        let index = rng.random_range(0..piece_handles.len());

        piece_handles[index].clone()
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

fn spawn_all_pieces(mut commands: Commands, piece_assets: Res<PieceAssets>) {
    let mut rng = rng();

    for x in 0..PIECE_WIDTH {
        for y in 0..PIECE_HEIGHT {
            let piece_position = PiecePosition { x, y };
            let position = piece_position.get_position();

            commands.spawn((
                Sprite {
                    image: piece_assets.get_random_image_handle(&mut rng),
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
