use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::render::camera::Camera;

mod console;
mod menu;

static GAME_NAME: &str = "TODO: Wymyśl jakąś nazwę";

#[bevy_main]
fn main() {
	App::build()
		.add_resource(WindowDescriptor {
			width: 1280.0,
			height: 720.0,
			title: GAME_NAME.to_string(),
			vsync: false,
			resizable: true,
			decorations: true,
			cursor_visible: true,
			cursor_locked: false,
			mode: bevy::window::WindowMode::Windowed,
		})
		.add_plugins(DefaultPlugins)
		.add_plugin(GamePlugin)
		.add_plugin(console::ConsolePlugin)
		.run();
}

#[derive(Clone, Copy)]
enum AppState {
	Game,
	Menu,
}

struct GamePlugin;

impl GamePlugin {
	const STAGE: &'static str = "game_stage";
}

impl Plugin for GamePlugin {
	fn build(&self, app: &mut AppBuilder) {
		app.add_startup_system(setup_game.system())
			.add_resource(State::new(AppState::Game))
			.add_resource(Level::hub())
			.add_resource(InputState::default())
			.add_stage_after(
				stage::UPDATE,
				Self::STAGE,
				StateStage::<AppState>::default(),
			)
			.on_state_enter(Self::STAGE, AppState::Game, spawn_entities.system())
			.on_state_update(Self::STAGE, AppState::Game, player_movement.system())
			.on_state_update(Self::STAGE, AppState::Game, camera_input.system())
			.on_state_enter(Self::STAGE, AppState::Menu, menu::setup_menu.system())
			.on_state_exit(Self::STAGE, AppState::Menu, menu::destroy_menu.system())
			.add_system(save_scene.system());
	}
}

struct Textures {
	player_texture: Handle<ColorMaterial>,
	ground_tile: Handle<ColorMaterial>,
}

fn setup_game(
	commands: &mut Commands,
	asset_server: Res<AssetServer>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	let player_texture: Handle<Texture> = asset_server.load("bird.png");
	let player_texture: Handle<_> = materials.add(player_texture.into());

	let ground_tile: Handle<Texture> = asset_server.load("ground.png");
	let ground_tile: Handle<_> = materials.add(ground_tile.into());

	commands
		.spawn(Camera2dBundle {
			transform: Transform {
				scale: Vec3::new(0.3, 0.3, 1.0),
				..Default::default()
			},
			..Default::default()
		})
		.insert_resource(Textures {
			player_texture,
			ground_tile,
		});
}

fn spawn_entities(commands: &mut Commands, materials: Res<Textures>, level: Res<Level>) {
	commands
		.spawn(SpriteBundle {
			material: materials.player_texture.clone(),
			transform: Transform {
				translation: Default::default(),
				rotation: Quat::default(),
				scale: Vec3::new(0.04, 0.04, 1.0),
			},
			..Default::default()
		})
		.with(Player);

	for j in (level.size.y as i32 / 2 * -1)..(level.size.y as i32 / 2) {
		for i in (level.size.x as i32 / 2 * -1)..(level.size.x as i32 / 2) {
			let (j, i) = (j as f32, i as f32);
			commands.spawn(SpriteBundle {
				material: materials.ground_tile.clone(),
				transform: Transform {
					translation: Vec3::new(i * 32.0, j * 32.0, 0.0),
					..Default::default()
				},
				..Default::default()
			});
		}
	}
}

struct Player;

fn player_movement(
	kb_input: Res<Input<KeyCode>>,
	mut q: Query<&mut Transform, Or<(With<Player>, With<Camera>)>>,
) {
	for mut transform in q.iter_mut() {
		if kb_input.pressed(KeyCode::W) {
			transform.translation.y += 2.0;
		}
		if kb_input.pressed(KeyCode::A) {
			transform.translation.x -= 2.0;
		}
		if kb_input.pressed(KeyCode::S) {
			transform.translation.y -= 2.0;
		}
		if kb_input.pressed(KeyCode::D) {
			transform.translation.x += 2.0;
		}
	}
}

#[derive(Default)]
struct InputState {
	pub reader_scroll: EventReader<MouseWheel>,
}

fn camera_input(
	mut input: ResMut<InputState>,
	scroll_events: Res<Events<MouseWheel>>,
	mut q: Query<
		&mut Transform,
		(
			With<bevy::render::camera::Camera>,
			Without<console::ConsoleComponent>,
		),
	>,
) {
	for scroll_event in input.reader_scroll.iter(&scroll_events) {
		for mut camera_transform in q.iter_mut() {
			camera_transform.scale.y += scroll_event.y * 0.05;
			camera_transform.scale.x += scroll_event.y * 0.05;
		}
	}
}

struct Level {
	size: Vec2,
}

impl Level {
	fn hub() -> Self {
		Level {
			size: (15.0, 10.0).into(),
		}
	}
}

fn save_scene(world: &mut World, resources: &mut Resources) {
	// use bevy::reflect::TypeRegistry;
	//
	// // The TypeRegistry resource contains information about all registered types (including components). This is used to construct scenes.
	// let type_registry = resources.get::<TypeRegistry>().unwrap();
	// let scene = DynamicScene::from_world(&world, &type_registry);
	//
	// // Scenes can be serialized like this:
	// println!("{}", scene.serialize_ron(&type_registry).unwrap());
	//
	// // TODO: save scene
}
