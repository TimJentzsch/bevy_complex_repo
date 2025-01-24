use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn(Node {
            ..Default::default()
        })
        .with_children(|builder| {
            builder.spawn((
                Text::new("CPLX Teaser 1"),
                TextFont::from_font_size(60.),
                TextColor::WHITE,
            ));
        });
}
