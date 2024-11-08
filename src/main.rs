use bevy::{prelude::*,
    ecs::world::World,
    input::common_conditions::*,
    window::{PresentMode, WindowTheme},
    tasks::IoTaskPool, 
    utils::Duration,
};

use std::{fs::File, io::Write};

// use bevy_editor_pls::prelude::*;

use minigolf::{Fonts, OpIndex};
use minigolf::level_handler::level_handler::{gltf_handler_init, setup_ground, setup_light, query_and_despawn_scene, query_and_update_scene};
use minigolf::user_interface::camera_world::setup_3d_camera;
use minigolf::user_interface::user_interface::{fire_ray, release_ray, draw_cursor, setup_ui};

fn main() {
    let mut app = App::new();
        app.add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Calculator Simulator".into(),
                    name: Some("bevy.app".into()),
                    resolution: (1280., 720.).into(),
                    resizable: true,
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: true,
                        ..Default::default()
                    },
                    present_mode: PresentMode::AutoVsync,
                    prevent_default_event_handling: false, // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                    window_theme: Some(WindowTheme::Dark),
                    visible: true,
                    ..default()
                }),
                ..default()
            }),
        ))
        // .add_plugins(EditorPlugin::default())
        .register_type::<ComponentA>()
        .register_type::<ComponentB>()
        .register_type::<ResourceA>()
        .insert_state(AppState::Game)
        .insert_resource(Fonts::new())
        .insert_resource(OpIndex::new())
        .add_systems(Startup, gltf_handler_init)
        .add_systems(Startup, setup_ground)
        .add_systems(Startup, setup_light)
        .add_systems(Startup, setup_ui)
        // .add_systems(Startup, setup_3d_camera)
        .add_systems(
            Startup,
            (save_scene_system, load_scene_system, infotext_system),
        )
        .add_systems(Update, log_system)
        .add_systems(Update, draw_cursor)
        .add_systems(Update, release_ray.run_if(input_just_released(MouseButton::Left)))
        .add_systems(Update, fire_ray.run_if(input_pressed(MouseButton::Left)))
        .add_systems(Update, query_and_despawn_scene.run_if(input_pressed(MouseButton::Right)))
        .add_systems(Update, query_and_update_scene.run_if(input_pressed(MouseButton::Right)))
        .add_systems(Update, app_state_logic)
        .add_systems(Update, app_state_cycle.run_if(input_pressed(KeyCode::ArrowUp)))
        .add_systems(OnEnter(AppState::Game), app_state_game_logic_enter)
        .add_systems(OnExit(AppState::Game), app_state_game_logic_exit)
        .add_systems(OnEnter(AppState::Menu), app_state_menu_logic_enter)
        .add_systems(OnExit(AppState::Menu), app_state_menu_logic_exit)
        .add_systems(OnEnter(AppState::Paused), app_state_paused_logic_enter)
        .add_systems(OnExit(AppState::Paused), app_state_paused_logic_exit);
        app.run();
}

#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
enum AppState {
    #[default]
    Game,
    Menu,
    Paused,
}

fn app_state_cycle(
    app_state: Res<State<AppState>>,
    mut next_game_state: ResMut<NextState<AppState>>,
) {
    match app_state.get() {
        AppState::Game => {
            next_game_state.set(AppState::Menu);
        },
        AppState::Menu => {
            next_game_state.set(AppState::Paused);
        },
        AppState::Paused => {
            next_game_state.set(AppState::Game);
        },
        _ => {},
    }
}

fn app_state_logic(
    app_state: Res<State<AppState>>,
) {
    match app_state.get() {
        AppState::Game => {
            // info!("AppState::Game");
        },
        AppState::Menu => {
            // info!("AppState::Menu");
        },
        AppState::Paused => {
            // info!("AppState::Paused");
        },
        _ => {},
    }
}

fn app_state_game_logic_enter() {
    info!("AppState::Game::OnEnter");
}

fn app_state_game_logic_exit() {
    info!("AppState::Game::OnExit");
}

fn app_state_menu_logic_enter() {
    info!("AppState::Menu::OnEnter");
}

fn app_state_menu_logic_exit() {
    info!("AppState::Menu::OnExit");
}

fn app_state_paused_logic_enter() {
    info!("AppState::Paused::OnEnter");
}

fn app_state_paused_logic_exit() {
    info!("AppState::Paused::OnExit");
}










// Registered components must implement the `Reflect` and `FromWorld` traits.
// The `Reflect` trait enables serialization, deserialization, and dynamic property access.
// `Reflect` enable a bunch of cool behaviors, so its worth checking out the dedicated `reflect.rs`
// example. The `FromWorld` trait determines how your component is constructed when it loads.
// For simple use cases you can just implement the `Default` trait (which automatically implements
// `FromWorld`). The simplest registered component just needs these three derives:
#[derive(Component, Reflect, Default)]
#[reflect(Component)] // this tells the reflect derive to also reflect component behaviors
struct ComponentA {
    pub x: f32,
    pub y: f32,
}

// Some components have fields that cannot (or should not) be written to scene files. These can be
// ignored with the #[reflect(skip_serializing)] attribute. This is also generally where the `FromWorld`
// trait comes into play. `FromWorld` gives you access to your App's current ECS `Resources`
// when you construct your component.
#[derive(Component, Reflect)]
#[reflect(Component)]
struct ComponentB {
    pub value: String,
    #[reflect(skip_serializing)]
    pub _time_since_startup: Duration,
}

impl FromWorld for ComponentB {
    fn from_world(world: &mut World) -> Self {
        let time = world.resource::<Time>();
        ComponentB {
            _time_since_startup: time.elapsed(),
            value: "Default Value".to_string(),
        }
    }
}

// Resources can be serialized in scenes as well, with the same requirements `Component`s have.
#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
struct ResourceA {
    pub score: u32,
}

// The initial scene file will be loaded below and not change when the scene is saved
const SCENE_FILE_PATH: &str = "scenes/load_scene_example.scn.ron";

// The new, updated scene data will be saved here so that you can see the changes
const NEW_SCENE_FILE_PATH: &str = "scenes/load_scene_example-new.scn.ron";

fn load_scene_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // "Spawning" a scene bundle creates a new entity and spawns new instances
    // of the given scene's entities as children of that entity.
    commands.spawn(DynamicSceneBundle {
        // Scenes are loaded just like any other asset.
        scene: asset_server.load(SCENE_FILE_PATH),
        ..default()
    });
}

// This system logs all ComponentA components in our world. Try making a change to a ComponentA in
// load_scene_example.scn. If you enable the `file_watcher` cargo feature you should immediately see
// the changes appear in the console whenever you make a change.
fn log_system(
    query: Query<(Entity, &ComponentA), Changed<ComponentA>>,
    res: Option<Res<ResourceA>>,
) {
    for (entity, component_a) in &query {
        info!("  Entity({})", entity.index());
        info!(
            "    ComponentA: {{ x: {} y: {} }}\n",
            component_a.x, component_a.y
        );
    }
    if let Some(res) = res {
        if res.is_added() {
            info!("  New ResourceA: {{ score: {} }}\n", res.score);
        }
    }
}

fn save_scene_system(world: &mut World) {
    // Scenes can be created from any ECS World.
    // You can either create a new one for the scene or use the current World.
    // For demonstration purposes, we'll create a new one.
    let mut scene_world = World::new();

    // The `TypeRegistry` resource contains information about all registered types (including components).
    // This is used to construct scenes, so we'll want to ensure that our previous type registrations
    // exist in this new scene world as well.
    // To do this, we can simply clone the `AppTypeRegistry` resource.
    let type_registry = world.resource::<AppTypeRegistry>().clone();
    scene_world.insert_resource(type_registry);

    let mut component_b = ComponentB::from_world(world);
    component_b.value = "hello".to_string();
    scene_world.spawn((
        component_b,
        ComponentA { x: 1.0, y: 2.0 },
        Transform::IDENTITY,
        Name::new("joe"),
    ));
    scene_world.spawn(ComponentA { x: 3.0, y: 4.0 });
    scene_world.insert_resource(ResourceA { score: 1 });

    // With our sample world ready to go, we can now create our scene using DynamicScene or DynamicSceneBuilder.
    // For simplicity, we will create our scene using DynamicScene:
    let scene = DynamicScene::from_world(&scene_world);

    // Scenes can be serialized like this:
    let type_registry = world.resource::<AppTypeRegistry>();
    let type_registry = type_registry.read();
    let serialized_scene = scene.serialize(&type_registry).unwrap();

    // Showing the scene in the console
    info!("{}", serialized_scene);

    // Writing the scene to a new file. Using a task to avoid calling the filesystem APIs in a system
    // as they are blocking
    // This can't work in WASM as there is no filesystem access
    #[cfg(not(target_arch = "wasm32"))]
    IoTaskPool::get()
        .spawn(async move {
            // Write the scene RON data to file
            File::create(format!("assets/{NEW_SCENE_FILE_PATH}"))
                .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                .expect("Error while writing scene to file");
        })
        .detach();
}

// This is only necessary for the info message in the UI. See examples/ui/text.rs for a standalone
// text example.
fn infotext_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(
        TextBundle::from_section(
            "Nothing to see in this window! Check the console output!",
            TextStyle {
                font_size: 50.0,
                ..default()
            },
        )
        .with_style(Style {
            align_self: AlignSelf::FlexEnd,
            ..default()
        }),
    );
}