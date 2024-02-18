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