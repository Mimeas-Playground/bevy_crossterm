use bevy::{prelude::*, log::{LogPlugin, self}};
use bevy_crossterm::prelude::*;

use std::default::Default;

mod title;
mod sprites;
mod colors;
mod animation;
mod finale;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default, States)]
pub enum GameState {
    #[default]
    Loading,
    Title,
    Sprites,
    Colors,
    Animation,
    Finale,
}

impl GameState {
    pub fn next_state(&self) -> Option<GameState> {
        use GameState::*;
        match self {
            Loading => Some(Title),
            Title => Some(Sprites),
            Sprites => Some(Colors),
            Colors => Some(Animation),
            Animation => Some(Finale),
            Finale => None,
        }
    }
}

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct HandleCollection(Vec<HandleUntyped>);

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct SceneRoot(Entity);

pub fn main() {
    // Window settings must happen before the crossterm Plugin
    let mut settings = CrosstermWindowSettings::default();
    settings.set_title("bevy_crossterm demo");

    App::new()
        .insert_resource(settings)
        .insert_resource(bevy::app::ScheduleRunnerSettings::run_loop(
            std::time::Duration::from_millis(16),
        ))
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    watch_for_changes: false,
                    ..default()
                })
                 // .set(LogPlugin {
                 //     level: log::Level::ERROR,
                 //     ..default()
                 // })
        )
        .add_plugin(CrosstermPlugin)
        .add_state::<GameState>()
        .add_systems((
            demo_setup.on_startup(),
            loading_system.on_startup(),
            just_wait_and_advance
        ))
        .add_system(
            check_for_loaded.in_set(OnUpdate(GameState::Loading))
        )
        .add_systems((
            title::setup.in_schedule(OnEnter(GameState::Title)),
            simple_teardown.in_schedule(OnExit(GameState::Title)),
        ))
        .add_systems((
            sprites::setup.in_schedule(OnEnter(GameState::Sprites)),
            simple_teardown.in_schedule(OnExit(GameState::Sprites))
        ))
        .add_systems((
            colors::setup.in_schedule(OnEnter(GameState::Colors)),
            simple_teardown.in_schedule(OnExit(GameState::Colors))
        ))
        .add_systems((
            animation::setup.in_schedule(OnEnter(GameState::Animation)),
            animation::update.in_set(OnUpdate(GameState::Animation)),
            simple_teardown.in_schedule(OnExit(GameState::Animation))
        ))
        .add_systems((
            finale::setup.in_schedule(OnEnter(GameState::Finale)),
        ))
        .run();
}

fn loading_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut cursor: ResMut<Cursor>,
) {
    cursor.hidden = true;

    // Load the assets we want
    let handles = asset_server.load_folder("demo").unwrap();

    commands.insert_resource(HandleCollection(handles));
}

// This function exists soely because bevy's asset loading is async.
// We need to wait until all assets are loaded before we do anyhting with them.
fn check_for_loaded(
    asset_server: Res<AssetServer>,
    mut handles: ResMut<HandleCollection>,
    mut next_state: ResMut<NextState<GameState>>
) {

    handles.0 = handles.iter()
           .filter_map(|h| {
               if asset_server.get_load_state(h.id()) == bevy::asset::LoadState::Failed {
                   None
               }
               else {
                   Some(h.clone())
               }
           }).collect();

    let data = asset_server.get_group_load_state(handles.iter().map(|handle| handle.id()));
    match data {
        bevy::asset::LoadState::Loaded => {
            next_state.set(GameState::Title);
        },
        _ => {}
    }
}

// Setup anything needed globally for the demo
pub fn demo_setup(mut commands: Commands) {
    let scene_root = commands.spawn(()).id();

    commands.insert_resource(SceneRoot(scene_root));
}

// Helper function to see if there was a key press this frame
pub fn detect_keypress(mut keys: EventReader<KeyEvent>) -> bool {
    keys.iter().last().is_some()
}

// Simple update function that most screens will use
pub fn just_wait_and_advance(
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>,>,
    mut app_exit: ResMut<Events<bevy::app::AppExit>>,
    keys: EventReader<KeyEvent>,
) {
    if detect_keypress(keys) {
        if let Some(next_stage) = state.0.next_state() {
            next_state.set(next_stage);
        } else {
            app_exit.send(bevy::app::AppExit);
        }
    }
}

// Looks for an entity resource and then despawns that entity and all it's children
pub fn simple_teardown(mut commands: Commands, mut scene_root: ResMut<SceneRoot>) {
    commands.entity(scene_root.0).despawn_recursive();

    // Create a new, valid scene_root
    scene_root.0 = commands.spawn(()).id();
}
