use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum AppState {
    #[default]
    //启动屏，用于加载全局资源和logo展示
    Splash,
    //游戏状态
    Game,
}
