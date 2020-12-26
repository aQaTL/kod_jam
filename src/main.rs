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
		.set_title(GAME_NAME.to_string())
}

#[derive(Clone, Copy)]
enum GameState {
	Game,
}

struct GamePlugin;

impl Plugin for GamePlugin {
	fn build(&self, app: &mut AppBuilder) {
        app
			.add_startup_system(setup_game.system())
			.add_startup_stage("game_setup", SystemStage::single(spawn_entities.system()))
			.add_system(player_movement.system());
	}
}

struct Textures {
	player_texture: Handle<ColorMaterial>,
}

fn setup_game(commands: &mut Commands,
			  asset_server: Res<AssetServer>,
			  mut materials: ResMut<Assets<ColorMaterial>>,
) {
	let player_texture: Handle<Texture> = asset_server.load("bird.png");
	let player_texture: Handle<_> = materials.add(player_texture.into());

	commands
		.spawn(Camera2dBundle::default())
		.insert_resource(Textures {
			player_texture,
		});
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
	mut players: Query<&mut Transform, With<Player>>)
{
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