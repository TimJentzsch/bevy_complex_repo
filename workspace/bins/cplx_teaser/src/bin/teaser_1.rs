use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(NodeBundle {
            ..Default::default()
        })
        .with_children(|builder| {
            builder.spawn(Text2dBundle {
                text: Text::from_section(
                    "CPLX Teaser 1",
                    TextStyle {
                        color: Color::WHITE,
                        font_size: 60.,
                        ..Default::default()
                    },
                ),
                ..Default::default()
            });
        });
}
