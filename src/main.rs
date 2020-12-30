use crate::console::ConsoleComponent;
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
		.add_plugin(menu::MenuPlugin)
		.run();
}

#[derive(Clone, Copy)]
pub enum AppState {
	Game,
	Menu,
	GameOver,
}

struct GamePlugin;

impl GamePlugin {
	const STAGE: &'static str = "game_stage";
}

impl Plugin for GamePlugin {
	fn build(&self, app: &mut AppBuilder) {
		let args = std::env::args().collect::<Vec<_>>();

		app.add_startup_system(setup_game.system())
			.add_resource(State::new({
				if args
					.get(1)
					.map(|arg| arg == "--skip-menu")
					.unwrap_or_default()
				{
					AppState::Game
				} else {
					AppState::Menu
				}
			}))
			.add_resource(Level::hub())
			.add_resource(InputState::default())
			.add_resource(CollisionEventReader::default())
			.add_event::<CollisionEvent>()
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
			.on_state_update(
				Self::STAGE,
				AppState::Game,
				detect_spikes_collision.system(),
			)
			.on_state_update(
				Self::STAGE,
				AppState::Game,
				process_collision_events.system(),
			);
	}
}

struct Textures {
	player_texture: Handle<ColorMaterial>,
	ground_tile: Handle<ColorMaterial>,
	portal_texture: Handle<ColorMaterial>,
	spikes_texture: Handle<ColorMaterial>,
}

fn setup_game(
	commands: &mut Commands,
	asset_server: Res<AssetServer>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	let player_texture: Handle<Texture> = asset_server.load("saitama_fit.png");
	let player_texture: Handle<_> = materials.add(player_texture.into());

	let ground_tile: Handle<Texture> = asset_server.load("ground.png");
	let ground_tile: Handle<_> = materials.add(ground_tile.into());

	let portal_texture: Handle<Texture> = asset_server.load("portal.png");
	let portal_texture: Handle<_> = materials.add(portal_texture.into());

	let spikes_texture: Handle<Texture> = asset_server.load("spikes.png");
	let spikes_texture: Handle<_> = materials.add(spikes_texture.into());

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
			spikes_texture,
		});
}

fn spawn_entities(commands: &mut Commands, materials: Res<Textures>, level: Res<Level>) {
	commands
		.spawn(SpriteBundle {
			material: materials.player_texture.clone(),
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

const MOVEMENT_DELTA: f32 = 100.0;

fn player_movement(
	time: Res<Time>,
	kb_input: Res<Input<KeyCode>>,
	level: Res<Level>,
	mut q: Query<
		&mut Transform,
		(
			Or<(With<Player>, With<Camera>)>,
			Without<console::ConsoleComponent>,
		),
	>,
) {
	let delta = MOVEMENT_DELTA * time.delta_seconds();
	for mut transform in q.iter_mut() {
		if kb_input.pressed(KeyCode::W) {
			transform.translation.y =
				(transform.translation.y + delta).min(level.size.y / 2.0 - TILE_SIZE);
		}
		if kb_input.pressed(KeyCode::A) {
			transform.translation.x =
				(transform.translation.x - delta).max(level.size.x / -2.0 + TILE_SIZE / 2.0);
		}
		if kb_input.pressed(KeyCode::S) {
			transform.translation.y = (transform.translation.y - delta).max(level.size.y / -2.0);
		}
		if kb_input.pressed(KeyCode::D) {
			transform.translation.x =
				(transform.translation.x + delta).min(level.size.x / 2.0 - TILE_SIZE * 1.5);
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

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
struct PortalDestination(LevelType);

struct Spikes;

fn setup_level_hub(commands: &mut Commands, materials: Res<Textures>) {
	info!("Spawning hub level entities");
	commands
		.spawn(SpriteBundle {
			material: materials.portal_texture.clone(),
			transform: Transform {
				translation: Vec3::new(3.0 * TILE_SIZE, 4.0 * TILE_SIZE, 0.0),
				..Default::default()
			},
			..Default::default()
		})
		.with(PortalDestination(LevelType::Level1))
		.spawn(SpriteBundle {
			material: materials.portal_texture.clone(),
			transform: Transform {
				translation: Vec3::new(-7.0 * TILE_SIZE, -5.0 * TILE_SIZE, 0.0),
				..Default::default()
			},
			..Default::default()
		})
		.with(PortalDestination(LevelType::Secret1));

	let spikes_locations = [(-1.0, 1.0), (-1.0, 2.0), (-3.0, -1.0)];
	for (spike_x_idx, spike_y_idx) in spikes_locations.iter() {
		commands
			.spawn(SpriteBundle {
				material: materials.spikes_texture.clone(),
				transform: Transform {
					translation: Vec3::new(spike_x_idx * TILE_SIZE, spike_y_idx * TILE_SIZE, 0.0),
					..Default::default()
				},
				..Default::default()
			})
			.with(Spikes);
	}
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
	mut collision_events: ResMut<Events<CollisionEvent>>,
) {
	for player in players.iter() {
		for (portal, portal_destination) in portals.iter() {
			// To increase tolerance, increase this                ---\/
			if (player.translation.x - portal.translation.x).abs() * 2.1 < (TILE_SIZE + TILE_SIZE)
				&& (player.translation.y - portal.translation.y).abs() * 2.1
					< (TILE_SIZE + TILE_SIZE)
			{
				collision_events.send(CollisionEvent::Portal(*portal_destination));
			}
		}
	}
}

fn detect_spikes_collision(
	spikes: Query<(&Transform, &Sprite), With<Spikes>>,
	players: Query<(&Transform, &Sprite), (With<Player>, Changed<Transform>)>,
	mut collision_events: ResMut<Events<CollisionEvent>>,
) {
	for (player, player_sprite) in players.iter() {
		for (spike, spike_sprite) in spikes.iter() {
			if (player.translation.x - spike.translation.x).abs() * 2.1
				< (player_sprite.size.x + spike_sprite.size.x)
				&& (player.translation.y - spike.translation.y).abs() * 2.1
					< (player_sprite.size.y + spike_sprite.size.y)
			{
				info!("player touched spikes");
				collision_events.send(CollisionEvent::Spikes);
			}
		}
	}
}

#[derive(Debug)]
enum CollisionEvent {
	Portal(PortalDestination),
	Spikes,
}

#[derive(Default)]
struct CollisionEventReader(EventReader<CollisionEvent>);

const BRIGHTNESS_DELTA: f32 = 0.04;

fn process_collision_events(
	mut collision_event_reader: ResMut<CollisionEventReader>,
	collision_events: Res<Events<CollisionEvent>>,
	mut console_events: ResMut<Events<console::ConsoleEvent>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
	mut player_transform_query: Query<&mut Transform, Or<(With<Player>, With<Camera>)>>,
	mut state: ResMut<State<AppState>>,
	console_entities: Query<&Handle<ColorMaterial>, With<ConsoleComponent>>,
) {
	for collision_event in collision_event_reader.0.iter(&collision_events) {
		println!("collision event start");
		match collision_event {
			CollisionEvent::Spikes => {
				change_brightness(&mut materials, &console_entities, &mut state);
				reset_player_position(&mut player_transform_query);
			}
			CollisionEvent::Portal(destination) => {
				info!("player entered portal to {:?}", destination);
			}
		}
		let log_msg = format!("Collision detected with: {:?}\n", collision_event);
		console_events.send(console::ConsoleEvent::Log(log_msg))
	}
}

fn change_brightness(
	materials: &mut Assets<ColorMaterial>,
	console_entities: &Query<&Handle<ColorMaterial>, With<ConsoleComponent>>,
	state: &mut State<AppState>,
) {
	let delta = Vec4::new(BRIGHTNESS_DELTA, BRIGHTNESS_DELTA, BRIGHTNESS_DELTA, 0.0);

	let ids = materials
		.iter()
		.map(|(id, _)| id)
		.filter(|id| !console_entities.iter().any(|con_id| con_id.id == *id))
		.collect::<Vec<_>>();

	let mut all_black = true;
	for id in ids {
		let material = materials.get_mut(id).unwrap();
		material.color = material.color + delta * -1.0;
		if material.color.r() > 0.0 && material.color.g() > 0.0 && material.color.b() > 0.0 {
			all_black = false;
		}
	}

	if all_black {
		state.set_next(AppState::GameOver).unwrap();
	}
}

fn reset_player_position(
	player_transform_query: &mut Query<&mut Transform, Or<(With<Player>, With<Camera>)>>,
) {
	for mut transform in player_transform_query.iter_mut() {
		transform.translation.x = 0.0;
		transform.translation.y = 0.0;
	}
}
