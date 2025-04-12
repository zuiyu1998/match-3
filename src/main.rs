use bevy::{
    prelude::*,
    render::{
        RenderPlugin,
        settings::{Backends, RenderCreation, WgpuSettings},
    },
    window::WindowResolution,
};
use match_3::{GamePlugin, HEIGHT, WIDTH};

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "match_3".to_string(),
                    resolution: WindowResolution::new(WIDTH, HEIGHT),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest())
            .set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    backends: Some(Backends::VULKAN),
                    ..Default::default()
                }),
                ..Default::default()
            }),
        GamePlugin,
    ));

    app.run();
}
