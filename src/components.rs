use bevy::prelude::*;

pub const TILE_SIZE: f32 = 32.0;

pub struct Missile {
	pub direction: Vec3,
	pub speed: Vec3,
}

pub struct Textures {
	pub player_texture: Handle<ColorMaterial>,
	pub ground_tile: Handle<ColorMaterial>,
	pub portal_texture: Handle<ColorMaterial>,
	pub spikes_texture: Handle<ColorMaterial>,
	pub missile_texture: Handle<ColorMaterial>,
}

pub struct MainCamera;

pub struct Player;

#[derive(Debug, Copy, Clone)]
pub struct PortalDestination(pub LevelType);

pub struct Spikes;

pub struct Level {
	pub size: Vec2,
	pub l_type: LevelType,
}

impl Default for Level {
	fn default() -> Self {
		Self::hub()
	}
}

#[derive(Debug, Copy, Clone)]
pub enum LevelType {
	Hub,
	Secret1,
	Level1,
}

impl Level {
	pub fn hub() -> Self {
		Level {
			size: (15.0 * TILE_SIZE, 10.0 * TILE_SIZE).into(),
			l_type: LevelType::Hub,
		}
	}
}
