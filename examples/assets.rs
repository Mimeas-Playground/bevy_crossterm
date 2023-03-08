use bevy::{prelude::*, window::PrimaryWindow};
use bevy_crossterm::prelude::*;

use std::default::Default;

#[derive(Clone, PartialEq, Eq, Hash, Debug, States, Default)]
enum GameState {
    #[default]
    Loading,
    Running,
}

// PROTIP: _technically_ since Sprite's are just created using strings, an easier way to load them from an external
// file is just:
//static TITLE_TEXT: &str = include_str!("assets/demo/title.txt");
// then just:
//sprites.add(Sprite::new(TITLE_TEXT));
// and boom, you have yourself a sprite in the asset system.
// That's nice and easy - don't have to worry about async, don't need to distribute files alongside your exe.
// But then you can't take advangate of hot reloading, and plus it only works for sprites. StyleMaps have to go through
// the AssetServer if you want to load them from an external file.

pub fn main() {
    // Window settings must happen before the crossterm Plugin
    let mut settings = CrosstermWindowSettings::default();
    settings.set_title("Assets example");

    App::new()
        // Add our window settings
        .insert_resource(settings)
        // Set some options in bevy to make our program a little less resource intensive - it's just a terminal game
        // no need to try and go nuts
        .insert_resource(bevy::core::TaskPoolOptions::with_num_threads(1))
        // The Crossterm runner respects the schedulerunnersettings. No need to run as fast as humanly
        // possible - 20 fps should be more than enough for a scene that never changes
        .insert_resource(bevy::app::ScheduleRunnerSettings::run_loop(
            std::time::Duration::from_millis(50),
        ))
        .add_state::<GameState>()
        .add_system(loading_system.in_schedule(OnEnter(GameState::Loading)))
        .add_system(check_for_loaded.in_set(OnUpdate(GameState::Loading)))
        .add_system(create_entities.in_schedule(OnEnter(GameState::Running)))
        .add_plugins(DefaultPlugins)
        .add_plugin(CrosstermPlugin)
        .run();
}

static ASSETS: &[&str] = &["demo/title.txt", "demo/title.stylemap"];

#[derive(Resource, Deref, DerefMut)]
struct UntypedHandles(Vec<HandleUntyped>);

// This is a simple system that loads assets from the filesystem
fn loading_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut cursor: ResMut<Cursor>,
) {
    cursor.hidden = true;

    // Load the assets we want
    let mut handles = UntypedHandles(Vec::new());
    for asset in ASSETS {
        handles.push(asset_server.load_untyped(*asset));
    }

    commands.insert_resource(handles);
}

// This function exists soely because bevy's asset loading is async.
// We need to wait until all assets are loaded before we do anyhting with them.
fn check_for_loaded(
    asset_server: Res<AssetServer>,
    handles: Res<UntypedHandles>,
    mut next_state: ResMut<NextState<GameState>>
) {
    let data = asset_server.get_group_load_state(handles.iter().map(|handle| handle.id()));

    match data {
        bevy::asset::LoadState::NotLoaded | bevy::asset::LoadState::Loading => {}
        bevy::asset::LoadState::Loaded => {
            next_state.set(GameState::Running);
        }
        bevy::asset::LoadState::Failed => {},
        bevy::asset::LoadState::Unloaded => {}
    }
}

// Now that we're sure the assets are loaded, spawn a new sprite into the world
fn create_entities(
    mut commands: Commands,
    mut windows: Query<&mut CrosstermWindow, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    sprites: Res<Assets<Sprite>>,
    stylemaps: Res<Assets<StyleMap>>
) {
    let mut window = windows.single_mut();
    // I want to center the title, so i needed to wait until it was loaded before I could actually access
    // the underlying data to see how wide the sprite is and do the math
    let title_handle = asset_server.get_handle("demo/title.txt");
    let title_sprite = sprites.get(&title_handle).unwrap();

    let center_x = window.as_mut().x_center() as i32 - title_sprite.x_center() as i32;
    let center_y = window.as_mut().y_center() as i32 - title_sprite.y_center() as i32;

    commands.spawn(SpriteBundle {
        sprite: title_handle.clone(),
        position: Position::with_xy(center_x, center_y),
        stylemap: asset_server.get_handle("demo/title.stylemap"),
        ..Default::default()
    });
}
