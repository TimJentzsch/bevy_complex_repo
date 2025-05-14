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
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        })
        .with_children(|builder| {
            builder.spawn((
                Text::new("Custom Web Folder"),
                TextColor::WHITE,
                TextFont {
                    font_size: 60.,
                    ..Default::default()
                },
            ));
        });
}
