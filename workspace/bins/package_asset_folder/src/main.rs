use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn((
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            (
                Text::new("Package Assets"),
                TextColor::WHITE,
                TextFont {
                    font_size: 60.,
                    ..Default::default()
                }
            ),
            (ImageNode {
                image: asset_server.load("bevy.png"),
                ..default()
            },)
        ],
    ));
}
