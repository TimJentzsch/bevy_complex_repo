use bevy::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(module = "/js/hello_world.js")]
extern "C" {
    fn hello_world();
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    hello_world();

    commands.spawn(Camera2d);

    commands.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            (
                Text::new("JS Snippets"),
                TextColor::WHITE,
                TextFont {
                    font_size: 60.,
                    ..Default::default()
                },
            ),
            (
                Text::new("Check the console output"),
                TextColor::WHITE,
                TextFont {
                    font_size: 25.,
                    ..Default::default()
                },
            )
        ],
    ));
}
