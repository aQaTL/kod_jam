use bevy::prelude::*;

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
		.run();
}

#[derive(Clone, Copy)]
enum GameState {
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
			.add_resource(State::new(GameState::Game))
			.add_stage_after(
				stage::UPDATE,
				Self::STAGE,
				StateStage::<GameState>::default(),
			)
			.on_state_enter(Self::STAGE, GameState::Game, spawn_entities.system())
			.on_state_update(Self::STAGE, GameState::Game, player_movement.system())
			.on_state_enter(Self::STAGE, GameState::Menu, menu::setup_menu.system())
			.on_state_exit(Self::STAGE, GameState::Menu, menu::destroy_menu.system());
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
		.with(Player);
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
