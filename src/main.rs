use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::render::camera::Camera;

mod console;
mod menu;

static GAME_NAME: &str = "TODO: Wymyśl jakąś nazwę";
const TILE_SIZE: f32 = 32.0;

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
			.on_state_update(Self::STAGE, AppState::Game, color_change_input.system())
			.on_state_update(
				Self::STAGE,
				AppState::Game,
				detect_portal_collision.system(),
			)
			.on_state_enter(Self::STAGE, AppState::Menu, menu::setup_menu.system())
			.on_state_exit(Self::STAGE, AppState::Menu, menu::destroy_menu.system());
	}
}

struct Textures {
	player_texture: Handle<ColorMaterial>,
	ground_tile: Handle<ColorMaterial>,
	portal_texture: Handle<ColorMaterial>,
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

	let portal_texture: Handle<Texture> = asset_server.load("portal.png");
	let portal_texture: Handle<_> = materials.add(portal_texture.into());

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
			portal_texture,
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

	//TODO use bevy_tilemap
	for j in
		((level.size.y / TILE_SIZE / 2.0 * -1.0) as i32)..((level.size.y / TILE_SIZE / 2.0) as i32)
	{
		for i in ((level.size.x / TILE_SIZE / 2.0 * -1.0) as i32)
			..((level.size.x / TILE_SIZE / 2.0) as i32)
		{
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

	match level.l_type {
		LevelType::Hub => setup_level_hub(commands, materials),
		_ => unimplemented!(),
	}
}

struct Player;

fn player_movement(
	kb_input: Res<Input<KeyCode>>,
	level: Res<Level>,
	mut q: Query<&mut Transform, Or<(With<Player>, With<Camera>)>>,
) {
	for mut transform in q.iter_mut() {
		if kb_input.pressed(KeyCode::W) {
			transform.translation.y =
				(transform.translation.y + 2.0).min(level.size.y / 2.0 - TILE_SIZE);
		}
		if kb_input.pressed(KeyCode::A) {
			transform.translation.x =
				(transform.translation.x - 2.0).max(level.size.x / -2.0 + TILE_SIZE / 2.0);
		}
		if kb_input.pressed(KeyCode::S) {
			transform.translation.y = (transform.translation.y - 2.0).max(level.size.y / -2.0);
		}
		if kb_input.pressed(KeyCode::D) {
			transform.translation.x =
				(transform.translation.x + 2.0).min(level.size.x / 2.0 - TILE_SIZE);
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
	l_type: LevelType,
}

impl Default for Level {
	fn default() -> Self {
		Self::hub()
	}
}

#[derive(Debug)]
enum LevelType {
	Hub,
	Secret1,
	Level1,
}

impl Level {
	fn hub() -> Self {
		Level {
			size: (15.0 * TILE_SIZE, 10.0 * TILE_SIZE).into(),
			l_type: LevelType::Hub,
		}
	}
}

#[derive(Debug)]
struct PortalDestination(LevelType);

fn setup_level_hub(commands: &mut Commands, materials: Res<Textures>) {
	info!("Spawning hub level entities");
	commands
		.spawn(SpriteBundle {
			material: materials.portal_texture.clone(),
			transform: Transform {
				translation: Vec3::new(3.0 * TILE_SIZE, 4.0 * TILE_SIZE, 0.0),
				..Default::default()
			},
			global_transform: Default::default(),
			..Default::default()
		})
		.with(PortalDestination(LevelType::Level1))
		.spawn(SpriteBundle {
			material: materials.portal_texture.clone(),
			transform: Transform {
				translation: Vec3::new(-7.0 * TILE_SIZE, -5.0 * TILE_SIZE, 0.0),
				..Default::default()
			},
			global_transform: Default::default(),
			..Default::default()
		})
		.with(PortalDestination(LevelType::Secret1));
}

// TODO: Brightness should probably be changed differently
fn color_change_input(kb_input: Res<Input<KeyCode>>, mut materials: ResMut<Assets<ColorMaterial>>) {
	let delta = Vec4::new(0.01, 0.01, 0.01, 0.0);
	if kb_input.pressed(KeyCode::Period) {
		let ids = materials.iter().map(|(id, _)| id).collect::<Vec<_>>();
		for id in ids {
			let material = materials.get_mut(id).unwrap();
			material.color = material.color + delta;
		}
	}
	if kb_input.pressed(KeyCode::Comma) {
		let ids = materials.iter().map(|(id, _)| id).collect::<Vec<_>>();
		for id in ids {
			let material = materials.get_mut(id).unwrap();
			material.color = material.color + delta * -1.0;
		}
	}
}

fn detect_portal_collision(
	portals: Query<(&Transform, &PortalDestination), With<PortalDestination>>,
	players: Query<&Transform, (With<Player>, Changed<Transform>)>,
) {
	for player in players.iter() {
		println!();
		let (a_x1, a_y1) = (
			player.translation.x - TILE_SIZE / 2.0,
			player.translation.y - TILE_SIZE / 2.0,
		);
		let (a_x2, a_y2) = (
			player.translation.x + TILE_SIZE / 2.0,
			player.translation.y + TILE_SIZE / 2.0,
		);
		for (portal, portal_destination) in portals.iter() {
			let (b_x1, b_y1) = (
				portal.translation.x - TILE_SIZE / 2.0,
				portal.translation.y - TILE_SIZE / 2.0,
			);
			let (b_x2, b_y2) = (
				portal.translation.x + TILE_SIZE / 2.0,
				portal.translation.y + TILE_SIZE / 2.0,
			);
			println!("player: {},{} x {},{}", a_x1, a_y1, a_x2, a_y2);
			println!("portal: {},{} x {},{}", b_x1, b_y1, b_x2, b_y2);
			// TODO: detect intersection
			if false {
				println!("player entered portal to {:?}", portal_destination);
			}
		}
	}
}
