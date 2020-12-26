use bevy::prelude::*;

static GAME_NAME: &str = "TODO: Wymyśl jakąś nazwę";

#[bevy_main]
fn main() {
	App::build()
		.add_plugins(DefaultPlugins)
		.add_plugin(GamePlugin)
		.add_startup_system(setup_window.system())
		.add_resource(State::new(GameState::Game))
		.run();
}

fn setup_window(mut windows: ResMut<Windows>) {
	windows
		.get_primary_mut()
		.expect("Expected to have a window")
		.set_title(GAME_NAME.to_string());
}

#[derive(Clone, Copy)]
enum GameState {
	Game,
}

struct GamePlugin;

impl Plugin for GamePlugin {
	fn build(&self, app: &mut AppBuilder) {
		app.add_startup_system(setup_game.system())
			.add_startup_stage("game_setup", SystemStage::single(spawn_entities.system()))
			.add_system(player_movement.system())
			.add_system(size_scaling.system())
			.add_system(player_scaling.system());
	}
}

struct Textures {
	player_texture: Handle<ColorMaterial>,
}

fn setup_game(
	commands: &mut Commands,
	asset_server: Res<AssetServer>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	let player_texture: Handle<Texture> = asset_server.load("bird.png");
	let player_texture: Handle<_> = materials.add(player_texture.into());

	commands
		.spawn(Camera2dBundle::default())
		.insert_resource(Textures { player_texture });
}

fn spawn_entities(commands: &mut Commands, materials: Res<Textures>) {
	commands
		.spawn(SpriteBundle {
			material: materials.player_texture.clone(),
			transform: Transform {
				translation: Default::default(),
				rotation: Quat::default(),
				scale: Vec3::new(0.1, 0.1, 0.1),
			},
			..Default::default()
		})
		.with(Player)
		.with(Position { x: 3, y: 3 })
		.with(Size { width: 0.8, height: 0.8 });
}

struct Player;

fn player_movement(
	kb_input: Res<Input<KeyCode>>,
	mut players: Query<&mut Transform, With<Player>>,
) {
	for mut player_transform in players.iter_mut() {
		if kb_input.pressed(KeyCode::W) {
			player_transform.translation.y += 2.0;
		}
		if kb_input.pressed(KeyCode::A) {
			player_transform.translation.x -= 2.0;
		}
		if kb_input.pressed(KeyCode::S) {
			player_transform.translation.y -= 2.0;
		}
		if kb_input.pressed(KeyCode::D) {
			player_transform.translation.x += 2.0;
		}
	}
}

fn player_scaling(kb_input: Res<Input<KeyCode>>, mut q: Query<(&mut Transform, &mut Sprite), With<Player>>) {
	for (mut transform, mut sprite) in q.iter_mut() {
		if kb_input.pressed(KeyCode::Up) {
			transform.scale.x += 0.1;
			println!("New x: {}", transform.scale.x);
		}
		if kb_input.pressed(KeyCode::P) {
			sprite.resize_mode = SpriteResizeMode::Manual;
			println!("Old sprite size x: {}", sprite.size.x);
			sprite.size.x += 10.0;
			println!("New sprite size x: {}", sprite.size.x);
		}
	}
}

const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;

/// Position on a game board. Not real pixels.
struct Position {
	x: i32,
	y: i32,
}

struct Size {
	width: f32,
	height: f32,
}

fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Transform), With<Sprite>>) {
	let window = windows.get_primary().unwrap();
	for (sprite_size, mut sprite_transform) in q.iter_mut() {
		// println!("sprite_transform.scale: {:?}", sprite_transform.scale);
	}
}