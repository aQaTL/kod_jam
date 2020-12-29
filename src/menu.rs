use crate::AppState;
use bevy::{prelude::*,app::AppExit };

pub fn setup_menu(
	commands: &mut Commands,
	asset_server: Res<AssetServer>,
	button_materials: Res<ButtonMaterials>,
) {
	commands
		.spawn(CameraUiBundle::default())
		.spawn(ButtonBundle {
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
			parent.spawn(TextBundle {
				text: Text {
					value: "Ready?".to_string(),
					font: asset_server.load("FiraSans-Bold.ttf"),
					style: TextStyle {
						font_size: 40.0,
						color: Color::rgb(0.9, 0.9, 0.9),
						..Default::default()
					},
				},
				..Default::default()
			});
		})
		.spawn(ButtonBundle {
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
			parent.spawn(TextBundle {
				text: Text {
					value: "Bored".to_string(),
					font: asset_server.load("FiraSans-Bold.ttf"),
					style: TextStyle {
						font_size: 40.0,
						color: Color::rgb(0.9, 0.9, 0.9),
						..Default::default()
					},
				},
				..Default::default()
			});
		});
}

pub fn destroy_menu(
	commands: &mut Commands,
	mut interaction_query: Query<(Entity, &Button, &Children)>,
) {
	for (but, _, _) in interaction_query.iter_mut() {
		commands.despawn_recursive(but);
	}
}

pub fn update_menu(
	button_materials: Res<ButtonMaterials>,
	mut interaction_query: Query<
		(Entity, &Interaction, &mut Handle<ColorMaterial>, &Children),
		(Mutated<Interaction>, With<Button>),
	>,
	mut text_query: Query<&mut Text>,
	mut state: ResMut<State<AppState>>,
	mut exit_signal: ResMut<Events<AppExit>>
) {
	for (id, interaction, mut material, children) in interaction_query.iter_mut() {
		let mut text = text_query.get_mut(children[0]).unwrap();
		println!("{:?}", id.id());
		match id.id() {
			5 => match *interaction {
				Interaction::Clicked => {
					text.value = "Loading...".to_string();
					*material = button_materials.pressed.clone();
					state.set_next(AppState::Game).unwrap();
				}
				Interaction::Hovered => {
					text.value = "Start!".to_string();
					*material = button_materials.hovered.clone();
				}
				Interaction::None => {
					text.value = "Ready?".to_string();
					*material = button_materials.normal.clone();
				}
			},
			7 => match *interaction {
				Interaction::Clicked => {
					text.value = "Exiting...".to_string();
					*material = button_materials.pressed.clone();
					exit_signal.send(AppExit);
				}
				Interaction::Hovered => {
					text.value = "Exit :(".to_string();
					*material = button_materials.hovered.clone();
				}
				Interaction::None => {
					text.value = "Bored?".to_string();
					*material = button_materials.normal.clone();
				}
			},
			_ => (),
		}
	}
}

pub struct ButtonMaterials {
	normal: Handle<ColorMaterial>,
	hovered: Handle<ColorMaterial>,
	pressed: Handle<ColorMaterial>,
}

impl FromResources for ButtonMaterials {
	fn from_resources(resources: &Resources) -> Self {
		let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
		ButtonMaterials {
			normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
			hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
			pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
		}
	}
}
