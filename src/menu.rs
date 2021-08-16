use crate::AppState;
use bevy::app::AppExit;
use bevy::app::Events;
use bevy::prelude::*;
// use bevy::input::keyboard::KeyCode::Apps;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app.init_resource::<ButtonMaterials>()
			.add_system_set(SystemSet::on_enter(AppState::Menu).with_system(setup_menu.system()))
			.add_system_set(SystemSet::on_update(AppState::Menu).with_system(update_menu.system()))
			.add_system_set(SystemSet::on_exit(AppState::Menu).with_system(destroy_menu.system()));
	}
}

#[derive(Debug)]
pub enum ButtonBehavior {
	Exit,
	Play,
}

pub fn setup_menu(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	button_materials: Res<ButtonMaterials>,
) {
	commands.spawn_bundle(UiCameraBundle::default());
	commands
		.spawn_bundle(ButtonBundle {
			style: Style {
				size: Size::new(Val::Px(150.0), Val::Px(65.0)),
				// center button
				margin: Rect::all(Val::Auto),
				// horizontally center child text
				justify_content: JustifyContent::Center,
				// vertically center child text
				align_items: AlignItems::Center,
				..Default::default()
			},
			material: button_materials.normal.clone(),
			..Default::default()
		})
		.with_children(|parent| {
			parent
				.spawn_bundle(TextBundle {
					text: Text {
						sections: vec![TextSection {
							value: "Ready?".to_string(),
							style: TextStyle {
								font_size: 40.0,
								color: Color::rgb(0.9, 0.9, 0.9),
								font: asset_server.load("FiraSans-Bold.ttf"),
							},
						}],
						alignment: Default::default(),
					},
					..Default::default()
				})
				.insert(ButtonBehavior::Play);
		});
	commands
		.spawn_bundle(ButtonBundle {
			style: Style {
				size: Size::new(Val::Px(150.0), Val::Px(65.0)),
				// center button
				margin: Rect::all(Val::Auto),
				// horizontally center child text
				justify_content: JustifyContent::Center,
				// vertically center child text
				align_items: AlignItems::Center,
				..Default::default()
			},
			material: button_materials.normal.clone(),
			..Default::default()
		})
		.with_children(|parent| {
			parent
				.spawn_bundle(TextBundle {
					text: Text {
						sections: vec![TextSection {
							value: "Bored".to_string(),
							style: TextStyle {
								font_size: 40.0,
								color: Color::rgb(0.9, 0.9, 0.9),
								font: asset_server.load("FiraSans-Bold.ttf"),
							},
						}],
						alignment: Default::default(),
					},
					..Default::default()
				})
				.insert(ButtonBehavior::Exit);
		});
}

pub fn destroy_menu(
	mut commands: Commands,
	mut interaction_query: Query<(Entity, &Button, &Children)>,
) {
	for (but, _, _) in interaction_query.iter_mut() {
		commands.entity(but).despawn_recursive();
	}
}

pub fn update_menu(
	button_materials: Res<ButtonMaterials>,
	mut interaction_query: Query<
		(&Interaction, &mut Handle<ColorMaterial>, &Children),
		(Changed<Interaction>, With<Button>),
	>,
	mut text_query: Query<(&mut Text, &ButtonBehavior)>,
	mut state: ResMut<State<AppState>>,
	mut exit_signal: ResMut<Events<AppExit>>,
) {
	for (interaction, mut material, children) in interaction_query.iter_mut() {
		let (mut text, behavior) = text_query.get_mut(children[0]).unwrap();
		match *behavior {
			ButtonBehavior::Play => match *interaction {
				Interaction::Clicked => {
					text.sections.iter_mut().next().unwrap().value = "Loading...".to_string();
					*material = button_materials.pressed.clone();
					state.set(AppState::Game).unwrap();
				}
				Interaction::Hovered => {
					text.sections.iter_mut().next().unwrap().value = "Start!".to_string();
					*material = button_materials.hovered.clone();
				}
				Interaction::None => {
					text.sections.iter_mut().next().unwrap().value = "Ready?".to_string();
					*material = button_materials.normal.clone();
				}
			},
			ButtonBehavior::Exit => match *interaction {
				Interaction::Clicked => {
					text.sections.iter_mut().next().unwrap().value = "Exiting...".to_string();
					*material = button_materials.pressed.clone();
					exit_signal.send(AppExit);
				}
				Interaction::Hovered => {
					text.sections.iter_mut().next().unwrap().value = "Exit!".to_string();
					*material = button_materials.hovered.clone();
				}
				Interaction::None => {
					text.sections.iter_mut().next().unwrap().value = "Bored?".to_string();
					*material = button_materials.normal.clone();
				}
			},
		}
	}
}

pub struct ButtonMaterials {
	normal: Handle<ColorMaterial>,
	hovered: Handle<ColorMaterial>,
	pressed: Handle<ColorMaterial>,
}

impl FromWorld for ButtonMaterials {
	fn from_world(world: &mut World) -> Self {
		let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
		ButtonMaterials {
			normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
			hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
			pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
		}
	}
}
