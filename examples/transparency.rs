use bevy::{prelude::*, window::PrimaryWindow};
use bevy_crossterm::prelude::{*, Color};

pub fn main() {
    // Window settings must happen before the crossterm Plugin
    let mut settings = CrosstermWindowSettings::default();
    settings.set_title("Transparency example");

    App::new()
        .insert_resource(settings)
        .insert_resource(bevy::core::TaskPoolOptions::with_num_threads(1))
        .insert_resource(bevy::app::ScheduleRunnerSettings::run_loop(
            std::time::Duration::from_millis(50),
        ))
        // .add_resource(Timer::new(std::time::Duration::from_millis(250), true))
        .add_plugins(DefaultPlugins)
        .add_plugin(CrosstermPlugin)
        .add_startup_system(startup_system)
        .run();
}

// 5x5 box of spaces
static BIG_BOX: &str = "         \n         \n         \n         \n         ";
static SMALL_BOX: &str = r##"@@@@@
@ @ @
@   @
@ @ @
@@@@@"##;

fn startup_system(
    mut commands: Commands,
    mut window: Query<&mut CrosstermWindow, With<PrimaryWindow>>,
    mut cursor: ResMut<Cursor>,
    mut sprites: ResMut<Assets<Sprite>>,
    mut stylemaps: ResMut<Assets<StyleMap>>,
) {
    let mut window = window.single_mut();
    cursor.hidden = true;

    // Create our resources
    let plain = stylemaps.add(StyleMap::default());
    let white_bg = stylemaps.add(StyleMap::with_bg(Color::White));

    // Spawn two sprites into the world
    commands
        .spawn(SpriteBundle {
            sprite: sprites.add(Sprite::new(BIG_BOX)),
            position: Position {
                x: window.as_mut().x_center() as i32 - 3,
                y: window.as_mut().y_center() as i32 - 1,
                z: 0,
            },
            stylemap: white_bg.clone(),
            ..Default::default()
        });
        // Moving entity that ensures the box will get redrawn each step the entity passes over it
    commands.spawn(SpriteBundle {
            sprite: sprites.add(Sprite::new(SMALL_BOX)),
            position: Position {
                x: window.x_center() as i32 - 1,
                y: window.y_center() as i32 - 1,
                z: 1,
            },
            stylemap: plain.clone(),
            visible: Visible::transparent(),
        });
}
