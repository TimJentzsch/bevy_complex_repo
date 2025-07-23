use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Node { ..default() },
        children![(
            Text::new("JS Snippets"),
            TextColor::WHITE,
            TextFont {
                font_size: 60.,
                ..Default::default()
            },
        )],
    ));
}
