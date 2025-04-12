use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::AppState;

pub struct SplashStatePlugin;

fn get_loading_state() -> LoadingState<AppState> {
    LoadingState::new(AppState::Splash).continue_to_state(AppState::Game)
}

impl Plugin for SplashStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(get_loading_state());
    }
}
