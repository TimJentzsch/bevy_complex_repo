use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    //this should trigger the `unit_in_bundle` lint
    commands.spawn(());

    #[cfg(target_family = "wasm")]
    commands.spawn(());

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
                Text::new("lint"),
                TextColor::WHITE,
                TextFont {
                    font_size: 60.,
                    ..Default::default()
                },
            ));
        });
}
