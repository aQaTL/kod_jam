use crate::components::*;
use crate::console::ConsoleComponent;
use bevy::app::Events;
use bevy::input::mouse::MouseWheel;
use bevy::math::{Vec3Swizzles, Vec4Swizzles};
use bevy::prelude::*;
use bevy::render::camera::Camera;
use bevy::window::WindowResizeConstraints;

mod components;
mod console;
mod menu;

static GAME_NAME: &str = "TODO: Wymyśl jakąś nazwę";

#[bevy_main]
fn main() {
	if cfg!(debug_assertions) && std::env::var_os("RUST_LOG").is_none() {
		std::env::set_var("RUST_LOG", concat!(env!("CARGO_PKG_NAME"), "=debug"));
	}

	App::build()
		.insert_resource(WindowDescriptor {
			width: 1280.0,
			height: 720.0,
			resize_constraints: WindowResizeConstraints {
				min_width: 1280.0 / 4.0,
				min_height: 720.0 / 4.0,
				max_width: f32::INFINITY,
				max_height: f32::INFINITY,
			},
			scale_factor_override: None,
			title: GAME_NAME.to_string(),
			vsync: false,
			resizable: true,
			decorations: true,
			cursor_visible: true,
			cursor_locked: false,
			mode: bevy::window::WindowMode::Windowed,
			#[cfg(target_arch = "wasm32")]
			canvas: None,
		})
		.add_plugins(DefaultPlugins)
		.add_plugin(GamePlugin)
		.add_plugin(console::ConsolePlugin)
		.add_plugin(menu::MenuPlugin)
		.run();
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
	Game,
	Menu,
	GameOver,
}

impl Default for AppState {
	fn default() -> Self {
		if std::env::args().any(|arg| arg == "--skip-menu") {
			AppState::Game
		} else {
			AppState::Menu
		}
	}
}

struct GamePlugin;

impl Plugin for GamePlugin {
	fn build(&self, app: &mut AppBuilder) {
		app.add_startup_system(setup_game.system())
			.insert_resource(Level::hub())
			.add_event::<CollisionEvent>()
			.add_state(AppState::default())
			.add_system_set(
				SystemSet::on_enter(AppState::Game).with_system(spawn_entities.system()),
			)
			.add_system_set(SystemSet::on_update(AppState::Game).with_system(player_input.system()))
			.add_system_set(
				SystemSet::on_update(AppState::Game).with_system(player_shooting.system()),
			)
			.add_system_set(
				SystemSet::on_update(AppState::Game).with_system(camera_follow.system()),
			)
			.add_system_set(SystemSet::on_update(AppState::Game).with_system(camera_input.system()))
			.add_system_set(
				SystemSet::on_update(AppState::Game).with_system(color_change_input.system()),
			)
			.add_system_set(
				SystemSet::on_update(AppState::Game).with_system(process_moving_entities.system()),
			)
			.add_system_set(
				SystemSet::on_update(AppState::Game).with_system(detect_portal_collision.system()),
			)
			.add_system_set(
				SystemSet::on_update(AppState::Game).with_system(detect_spikes_collision.system()),
			)
			.add_system_set(
				SystemSet::on_update(AppState::Game).with_system(detect_missile_collision.system()),
			)
			.add_system_set(
				SystemSet::on_update(AppState::Game).with_system(process_collision_events.system()),
			);
	}
}

macro_rules! load_textures {
	($asset_server: ident, $materials: ident,
		$struct_name: ident {
			$($var_name: ident => $file_name: expr),* $(,)?
		}
		$(,)?
	) => {{
		$(
			let $var_name: Handle<Texture> = $asset_server.load($file_name);
			let $var_name: Handle<_> = $materials.add($var_name.into());
		)*

		$struct_name {
			$($var_name),*
		}
	}}
}

fn setup_game(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	commands
		.spawn_bundle(OrthographicCameraBundle {
			transform: Transform {
				scale: Vec3::new(0.3, 0.3, 1.0),
				..Default::default()
			},
			..OrthographicCameraBundle::new_2d()
		})
		.insert(MainCamera);

	let textures: Textures = load_textures!(
		asset_server,
		materials,
		Textures {
			player_texture => "saitama_fit.png",
			ground_tile => "ground.png",
			transparent_ground_tile => "transparent_ground.png",
			portal_texture => "portal.png",
			spikes_texture => "spikes.png",
			missile_texture => "LowPoly_Missile_command_Game_Assets_DevilsGarage_v01/2D/missile_large.png",
		},
	);

	commands.insert_resource(textures);
}

fn spawn_entities(mut commands: Commands, materials: Res<Textures>, level: Res<Level>) {
	commands
		.spawn_bundle(SpriteBundle {
			material: materials.player_texture.clone(),
			..Default::default()
		})
		.insert(Player);

	//TODO use bevy_tilemap
	for j in
		((level.size.y / TILE_SIZE / 2.0 * -1.0) as i32)..((level.size.y / TILE_SIZE / 2.0) as i32)
	{
		for i in ((level.size.x / TILE_SIZE / 2.0 * -1.0) as i32)
			..((level.size.x / TILE_SIZE / 2.0) as i32)
		{
			let (j, i) = (j as f32, i as f32);
			commands.spawn_bundle(SpriteBundle {
				material: materials.ground_tile.clone(),
				transform: Transform {
					translation: Vec3::new(i * 32.0, j * 32.0, 0.0),
					..Default::default()
				},
				..Default::default()
			});
		}
	}
	for j in (((level.size.y + 2.0 * TILE_SIZE) / TILE_SIZE / 2.0 * -1.0) as i32)
		..(((level.size.y + 2.0 * TILE_SIZE) / TILE_SIZE / 2.0) as i32)
	{
		for i in (((level.size.x + 2.0 * TILE_SIZE) / TILE_SIZE / 2.0 * -1.0) as i32)
			..(((level.size.x + 2.0 * TILE_SIZE) / TILE_SIZE / 2.0) as i32)
		{
			if j > ((level.size.y + 2.0 * TILE_SIZE) / TILE_SIZE / 2.0 * -1.0) as i32
				&& j < (((level.size.y + 2.0 * TILE_SIZE) / TILE_SIZE / 2.0) - 1.0) as i32
				&& i > ((level.size.x + 2.0 * TILE_SIZE) / TILE_SIZE / 2.0 * -1.0) as i32
				&& i < (((level.size.x + 2.0 * TILE_SIZE) / TILE_SIZE / 2.0) - 1.0) as i32
			{
				continue;
			}

			let (j, i) = (j as f32, i as f32);
			commands
				.spawn_bundle(SpriteBundle {
					material: materials.transparent_ground_tile.clone(),
					transform: Transform {
						translation: Vec3::new(i * 32.0, j * 32.0, 0.0),
						..Default::default()
					},
					..Default::default()
				})
				.insert(Collidable);
		}
	}

	match level.l_type {
		LevelType::Hub => setup_level_hub(commands, materials),
		_ => unimplemented!(),
	}
}

const MOVEMENT_DELTA: f32 = 100.0;

fn player_input(
	time: Res<Time>,
	kb_input: Res<Input<KeyCode>>,
	level: Res<Level>,
	mut player_translation: Query<&mut Transform, (With<Player>,)>,
) {
	let delta = MOVEMENT_DELTA * time.delta_seconds();
	for mut transform in player_translation.iter_mut() {
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

fn player_shooting(
	mut commands: Commands,
	materials: Res<Textures>,
	material_assets: Res<Assets<ColorMaterial>>,
	textures: Res<Assets<Texture>>,
	kb_input: Res<Input<KeyCode>>,
	windows: Res<Windows>,
	mouse_input: Res<Input<MouseButton>>,
	mut console_events: EventWriter<console::ConsoleEvent>,
	player_query: Query<(&Transform, &Sprite), (With<Player>,)>,
	camera_query: Query<&Transform, (With<MainCamera>,)>,
) {
	if mouse_input.just_pressed(MouseButton::Left) || kb_input.just_pressed(KeyCode::Space) {
		console_events.send(console::ConsoleEvent::from("fire\n"));

		let window = windows.get_primary().unwrap();
		let pos = match window.cursor_position() {
			Some(v) => v,
			None => {
				error!("Can't fire without a cursor position");
				return;
			}
		};
		debug!("Cursor pos: {:?}", pos);
		let size = Vec2::new(window.width(), window.height());
		// Offset the cursor from the left bottom origin to the screen center.
		let p = pos - size / 2.0;
		let camera_transform = camera_query.single().unwrap();
		// Translates the cursor position into the game world coordinates.
		let cursor_world_position =
			camera_transform.compute_matrix() * Vec4::new(p.x, p.y, 0.0, 1.0);
		debug!("World coords: {:?}", cursor_world_position);
		// debug!("Player coords: {:?}", player_query.single().unwrap().translation);

		let missile_texture_size = textures
			.get(
				material_assets
					.get(&materials.missile_texture)
					.unwrap()
					.texture
					.as_ref()
					.unwrap(),
			)
			.unwrap()
			.size;

		for (
			Transform {
				translation: player_translation,
				..
			},
			Sprite {
				size: player_size, ..
			},
		) in player_query.iter()
		{
			// Get a vector between the player and the cursor.
			let mut cursor_relative_to_player =
				cursor_world_position.xy() - player_translation.xy();
			// Normalize the cursor position, so that it only represents the direction (has length of 1).
			cursor_relative_to_player /= cursor_relative_to_player.length();
			// Calculate the angle between the cursor the player.
			let mut cursor_angle_relative_to_player = (cursor_relative_to_player.y
				/ cursor_relative_to_player.x)
				.atan()
				.to_degrees();
			if cursor_relative_to_player.x < 0.0 {
				cursor_angle_relative_to_player += 180.0;
			}

			let missile_direction = Vec3::new(
				cursor_relative_to_player.x,
				cursor_relative_to_player.y,
				0.0,
			);

			// We subtract 90 deg, because the missile sprite is facing up.
			let missile_angle_relative_to_player =
				(cursor_angle_relative_to_player - 90.0).to_radians();
			cursor_angle_relative_to_player = cursor_angle_relative_to_player.to_radians();

			// Radius of the player's shooting circle.
			let r = player_size.y / 2.0 + missile_texture_size.height as f32 / 2.0;
			// Calculate missile position at the circle from the cursor angle
			let missile_translation = Vec3::new(
				r * cursor_angle_relative_to_player.cos() + player_translation.x,
				r * cursor_angle_relative_to_player.sin() + player_translation.y,
				0.0,
			);

			commands
				.spawn_bundle(SpriteBundle {
					material: materials.missile_texture.clone(),
					transform: Transform {
						translation: missile_translation,
						rotation: Quat::from_rotation_z(missile_angle_relative_to_player),
						scale: Vec3::new(1.0, 1.0, 1.0),
					},
					..Default::default()
				})
				.insert(Missile {
					direction: missile_direction,
					speed: Vec3::new(1.0, 1.0, 1.0),
					// speed: Vec3::new(0.0, 0.0, 0.0),
				});
		}
	}
}

fn process_moving_entities(mut missile_query: Query<(&mut Transform, &Missile)>) {
	//TODO(aqatl): Delta time
	for (mut missile_transform, missile) in missile_query.iter_mut() {
		missile_transform.translation += missile.direction * missile.speed;
	}
}

fn camera_follow(
	mut q: QuerySet<(
		Query<&Transform, (With<Player>, Changed<Transform>)>,
		Query<&mut Transform, (With<Camera>, Without<console::ConsoleComponent>)>,
	)>,
) {
	if let Some(player_transform) = q.q0().iter().next() {
		let (x, y) = (
			player_transform.translation.x,
			player_transform.translation.y,
		);
		for mut camera_transform in q.q1_mut().iter_mut() {
			camera_transform.translation.x = x;
			camera_transform.translation.y = y;
		}
	}
}

fn camera_input(
	mut scroll_events: EventReader<MouseWheel>,
	mut q: Query<
		&mut Transform,
		(
			With<bevy::render::camera::Camera>,
			Without<console::ConsoleComponent>,
		),
	>,
) {
	for scroll_event in scroll_events.iter() {
		for mut camera_transform in q.iter_mut() {
			camera_transform.scale.y += scroll_event.y * 0.05;
			camera_transform.scale.x += scroll_event.y * 0.05;
		}
	}
}

fn setup_level_hub(mut commands: Commands, materials: Res<Textures>) {
	info!("Spawning hub level entities");
	commands
		.spawn_bundle(SpriteBundle {
			material: materials.portal_texture.clone(),
			transform: Transform {
				translation: Vec3::new(3.0 * TILE_SIZE, 4.0 * TILE_SIZE, 0.0),
				..Default::default()
			},
			..Default::default()
		})
		.insert(PortalDestination(LevelType::Level1))
		.insert(Collidable);
	commands
		.spawn_bundle(SpriteBundle {
			material: materials.portal_texture.clone(),
			transform: Transform {
				translation: Vec3::new(-7.0 * TILE_SIZE, -5.0 * TILE_SIZE, 0.0),
				..Default::default()
			},
			..Default::default()
		})
		.insert(PortalDestination(LevelType::Secret1))
		.insert(Collidable);

	let spikes_locations = [(-1.0, 1.0), (-1.0, 2.0), (-3.0, -1.0)];
	for (spike_x_idx, spike_y_idx) in spikes_locations.iter() {
		commands
			.spawn_bundle(SpriteBundle {
				material: materials.spikes_texture.clone(),
				transform: Transform {
					translation: Vec3::new(spike_x_idx * TILE_SIZE, spike_y_idx * TILE_SIZE, 0.0),
					..Default::default()
				},
				..Default::default()
			})
			.insert(Spikes)
			.insert(Collidable);
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

fn detect_missile_collision(
	mut commands: Commands,
	missile_q: Query<(&Transform, &Sprite, Entity), (With<Missile>,)>,
	collidiable_q: Query<(&Transform, &Sprite, Entity), (With<Collidable>, Without<Missile>)>,
	// mut collision_events: ResMut<Events<CollisionEvent>>,
) {
	for (missile_transform, missile_sprite, missile_entity) in missile_q.iter() {
		for (collidable_transform, collidable_sprite, collidable_entity) in collidiable_q.iter() {
			if (missile_transform.translation.x - collidable_transform.translation.x).abs() * 2.1
				< (missile_sprite.size.x / 2.0 + collidable_sprite.size.x / 1.0)
				&& (missile_transform.translation.y - collidable_transform.translation.y).abs()
					* 2.1 < (missile_sprite.size.y / 2.0 + collidable_sprite.size.y / 1.0)
			{
				//Should this system also send a CollisionEvent or is it for player's collisions
				// only?
				info!("missile collided with entity {:?}", collidable_entity);
				commands.entity(missile_entity).despawn_recursive();
			}
		}
	}
}

#[derive(Debug)]
enum CollisionEvent {
	Portal(PortalDestination),
	Spikes,
}

const BRIGHTNESS_DELTA: f32 = 0.04;

fn process_collision_events(
	mut collision_events: EventReader<CollisionEvent>,
	mut console_events: EventWriter<console::ConsoleEvent>,
	mut materials: ResMut<Assets<ColorMaterial>>,
	mut player_transform_query: Query<&mut Transform, Or<(With<Player>, With<Camera>)>>,
	mut state: ResMut<State<AppState>>,
	console_entities: Query<&Handle<ColorMaterial>, With<ConsoleComponent>>,
) {
	for collision_event in collision_events.iter() {
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
		warn!("Game over");
		state.set(AppState::GameOver).unwrap();
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
