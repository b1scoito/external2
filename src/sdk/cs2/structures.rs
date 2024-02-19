#![allow(non_snake_case, non_camel_case_types, dead_code)]

use std::ffi::{c_char, c_void};

pub struct EntityFlag;

impl EntityFlag {
    pub const FL_ONGROUND: u32 = 1 << 0;
    pub const FL_DUCKING: u32 = 1 << 1;
    pub const FL_WATERJUMP: u32 = 1 << 2;
    // Unused 1 << 3
    pub const FL_UNKNOWN0: u32 = 1 << 4;
    pub const FL_FROZEN: u32 = 1 << 5;
    pub const FL_ATCONTROLS: u32 = 1 << 6;
    pub const FL_CLIENT: u32 = 1 << 7;
    pub const FL_FAKECLIENT: u32 = 1 << 8;
    // Unused 1 << 9
    pub const FL_FLY: u32 = 1 << 10;
    pub const FL_UNKNOWN1: u32 = 1 << 11;
    // Unused 1 << 12
    // Unused 1 << 13
    pub const FL_GODMODE: u32 = 1 << 14;
    pub const FL_NOTARGET: u32 = 1 << 15;
    pub const FL_AIMTARGET: u32 = 1 << 16;
    // Unused 1 << 17
    pub const FL_STATICPROP: u32 = 1 << 18;
    // Unused 1 << 19
    pub const FL_GRENADE: u32 = 1 << 20;
    pub const FL_DONTTOUCH: u32 = 1 << 22;
    pub const FL_BASEVELOCITY: u32 = 1 << 23;
    pub const FL_WORLDBRUSH: u32 = 1 << 24;
    pub const FL_OBJECT: u32 = 1 << 25;
    pub const FL_ONFIRE: u32 = 1 << 27;
    pub const FL_DISSOLVING: u32 = 1 << 28;
    pub const FL_TRANSRAGDOLL: u32 = 1 << 29;
    pub const FL_UNBLOCKABLE_BY_PLAYER: u32 = 1 << 30;
}

#[derive(Debug)]
#[repr(C)]
pub struct GlobalVarsBase {
    pub real_time: f32,                  // 0x0000
    pub frame_count: i32,                // 0x0004
    pub frame_time: f32,                 // 0x0008
    pub absolute_frame_time: f32,        // 0x000C
    pub max_clients: i32,                // 0x0010
    pub pad_0: [u8; 0x14],               // 0x0014
    pub frame_time_2: f32,               // 0x0028
    pub current_time: f32,               // 0x002C
    pub current_time_2: f32,             // 0x0030
    pub pad_1: [u8; 0xC],                // 0x0034
    pub tick_count: f32,                 // 0x0040
    pub pad_2: [u8; 0x4],                // 0x0044
    pub network_channel: *const c_void,  // 0x0048
    pub pad_3: [u8; 0x130],              // 0x0050
    pub current_map: *const c_char,      // 0x0180
    pub current_map_name: *const c_char, // 0x0188
}


#[derive(Debug)]
#[repr(C)]
pub enum MoveType
{
	MOVETYPE_NONE,
	MOVETYPE_OBSOLETE,
	MOVETYPE_WALK,
	MOVETYPE_STEP,
	MOVETYPE_FLY,
	MOVETYPE_FLYGRAVITY,
	MOVETYPE_VPHYSICS,
	MOVETYPE_PUSH,
	MOVETYPE_NOCLIP,
	MOVETYPE_OBSERVER,
	MOVETYPE_LADDER,
	MOVETYPE_CUSTOM,
}