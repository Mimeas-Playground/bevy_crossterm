use bevy::{prelude::*, window::PrimaryWindow};
use bevy_crossterm::prelude::{*, Color};

use crate::SceneRoot;

pub fn setup(
    mut commands: Commands,
    scene_root: Res<SceneRoot>,
    window: Query<&CrosstermWindow, With<PrimaryWindow>>,
    mut sprites: ResMut<Assets<Sprite>>,
    mut stylemaps: ResMut<Assets<StyleMap>>,
) {
    let window = window.single();

    let goodbye = Sprite::new("Thank you for checking out bevy_crossterm! :) <3");
    let goodbye_pos = Position::with_xy(
        window.x_center() as i32 - goodbye.x_center() as i32,
        window.y_center() as i32 - goodbye.y_center() as i32,
    );

    // Programatically generate this style map
    let mut style = StyleMap::default();
    style.map.push(Vec::new());
    for ch in goodbye.data().chars() {
        if ch == '<' || ch == '3' {
            style.map[0].push(Style::new(
                Colors::new(Color::White, Color::DarkRed),
                Attributes::from(Attribute::Bold),
            ));
        } else {
            style.map[0].push(Style::default());
        }
    }

    commands.spawn(SpriteBundle {
        sprite: sprites.add(goodbye),
        position: goodbye_pos,
        stylemap: stylemaps.add(style),
        ..Default::default()
    }).set_parent(scene_root.0);
}
