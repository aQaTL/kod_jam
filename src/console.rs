use bevy::prelude::*;

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
	fn build(&self, app: &mut AppBuilder) {
		app.add_resource(ConsoleEventReader::default())
			.add_event::<ConsoleEvent>()
			.add_startup_system(setup_console.system())
			.add_system(console_trigger.system())
			.add_system(add_to_console.system())
			.add_system(process_console_events.system())
			.add_system(update_console_ui.system());
	}
}

pub struct ConsoleComponent;

pub struct ConsoleBuffer(String);

fn setup_console(commands: &mut Commands, asset_server: Res<AssetServer>) {
	let font = asset_server.load("FiraMono-Medium.ttf");

	let visibility = Visible {
		is_visible: false,
		is_transparent: true,
	};

	commands
		.spawn(CameraUiBundle::default())
		.with(ConsoleComponent)
		.spawn(NodeBundle {
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
		.with_children(|console_parent| {
			console_parent
				.spawn(TextBundle {
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
						value: "".to_string(),
						font: font.clone(),
						style: TextStyle {
							font_size: 40.0,
							color: Color::BLACK,
							alignment: TextAlignment::default(),
						},
					},
					..Default::default()
				})
				.with(ConsoleBuffer("Console\n".to_string()))
				.with(ConsoleComponent);
		})
		.with(ConsoleComponent);
}

fn console_trigger(
	kb_input: Res<Input<KeyCode>>,
	mut q: Query<&mut Visible, With<ConsoleComponent>>,
) {
	if kb_input.just_pressed(KeyCode::Grave) {
		for mut console_visibility in q.iter_mut() {
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
		text.value = console_buffer.0.clone();
	}
}

fn process_console_events(
	mut console_event_reader: ResMut<ConsoleEventReader>,
	console_events: Res<Events<ConsoleEvent>>,
	mut q: Query<&mut ConsoleBuffer>,
) {
	for console_event in console_event_reader.reader.iter(&console_events) {
		match console_event {
			ConsoleEvent::Log(log) => q.iter_mut().for_each(|mut buffer| buffer.0.push_str(&log)),
		}
	}
}

fn add_to_console(kb_input: Res<Input<KeyCode>>, mut console_events: ResMut<Events<ConsoleEvent>>) {
	if kb_input.just_pressed(KeyCode::Backslash) {
		console_events.send(ConsoleEvent::Log("Hello\n".to_string()));
	}
}

#[derive(Default)]
struct ConsoleEventReader {
	reader: EventReader<ConsoleEvent>,
}

pub enum ConsoleEvent {
	Log(String),
}
