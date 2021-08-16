use bevy::app::Events;
use bevy::prelude::*;

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
	fn build(&self, app: &mut AppBuilder) {
		app.add_event::<ConsoleEvent>()
			.add_startup_system(setup_console.system())
			.add_system(console_trigger.system())
			.add_system(add_to_console.system())
			.add_system(process_console_events.system())
			.add_system(update_console_ui.system());
	}
}

pub struct ConsoleComponent;

pub struct ConsoleBuffer(String);

fn setup_console(mut commands: Commands, asset_server: Res<AssetServer>) {
	let font = asset_server.load("FiraMono-Medium.ttf");

	let visibility = Visible {
		is_visible: false,
		is_transparent: true,
	};

	commands
		.spawn_bundle(UiCameraBundle::default())
		.insert(ConsoleComponent);
	commands
		.spawn_bundle(NodeBundle {
			style: Style {
				position: Rect {
					top: Val::Px(0.0),
					left: Val::Px(0.0),
					..Default::default()
				},
				position_type: PositionType::Absolute,
				flex_direction: FlexDirection::Column,
				align_items: AlignItems::FlexStart,
				align_self: Default::default(),
				// align_content: AlignContent::FlexStart,
				justify_content: JustifyContent::Center,
				..Default::default()
			},
			visible: visibility.clone(),
			..Default::default()
		})
		.insert(ConsoleComponent)
		.with_children(|console_parent| {
			console_parent
				.spawn_bundle(TextBundle {
					style: Style {
						border: Rect {
							left: Val::Px(1.0),
							right: Val::Px(1.0),
							top: Val::Px(1.0),
							bottom: Val::Px(1.0),
						},
						..Default::default()
					},
					visible: visibility.clone(),
					text: Text {
						sections: vec![TextSection {
							value: "".to_string(),
							style: TextStyle {
								font: font.clone(),
								font_size: 40.0,
								color: Color::BLACK,
							},
						}],
						alignment: TextAlignment::default(),
					},
					..Default::default()
				})
				.insert(ConsoleBuffer("Console\n".to_string()))
				.insert(ConsoleComponent);
		});
}

fn console_trigger(
	kb_input: Res<Input<KeyCode>>,
	mut q: Query<&mut Visible, With<ConsoleComponent>>,
) {
	if kb_input.just_pressed(KeyCode::Grave) {
		for mut console_visibility in q.iter_mut().enumerate() {
			console_visibility.is_visible = !console_visibility.is_visible;
			info!(
				"Console trigger. is_visible = {}",
				if console_visibility.is_visible {
					"true"
				} else {
					"false"
				}
			);
		}
	}
}

fn update_console_ui(mut q: Query<(&mut Text, &ConsoleBuffer), Changed<ConsoleBuffer>>) {
	for (mut text, console_buffer) in q.iter_mut() {
		if let Some(text_section) = text.sections.iter_mut().next() {
			text_section.value = console_buffer.0.clone()
		}
	}
}

fn process_console_events(
	mut console_events: EventReader<ConsoleEvent>,
	mut q: Query<&mut ConsoleBuffer>,
) {
	for console_event in console_events.iter() {
		match console_event {
			ConsoleEvent::Log(log) => q.iter_mut().for_each(|mut buffer| {
				if buffer.0.lines().count() >= 10 {
					buffer.0.clear();
				}
				buffer.0.push_str(&log)
			}),
			ConsoleEvent::StaticLog(log) => q.iter_mut().for_each(|mut buffer| {
				if buffer.0.lines().count() >= 10 {
					buffer.0.clear();
				}
				buffer.0.push_str(&log)
			}),
		}
	}
}

fn add_to_console(kb_input: Res<Input<KeyCode>>, mut console_events: ResMut<Events<ConsoleEvent>>) {
	if kb_input.just_pressed(KeyCode::Backslash) {
		console_events.send(ConsoleEvent::from("Hello\n"));
	}
}

pub enum ConsoleEvent {
	Log(String),
	StaticLog(&'static str),
}

impl From<String> for ConsoleEvent {
	fn from(s: String) -> Self {
		ConsoleEvent::Log(s)
	}
}

impl From<&'static str> for ConsoleEvent {
	fn from(s: &'static str) -> Self {
		ConsoleEvent::StaticLog(s)
	}
}
