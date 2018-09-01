/* automatically generated by rust-bindgen */

#![allow(non_camel_case_types, non_snake_case)]

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum RLBotCoreStatus {
    Success = 0,
    BufferOverfilled = 1,
    MessageLargerThanMax = 2,
    InvalidNumPlayers = 3,
    InvalidBotSkill = 4,
    InvalidHumanIndex = 5,
    InvalidName = 6,
    InvalidTeam = 7,
    InvalidTeamColorID = 8,
    InvalidCustomColorID = 9,
    InvalidGameValues = 10,
    InvalidThrottle = 11,
    InvalidSteer = 12,
    InvalidPitch = 13,
    InvalidYaw = 14,
    InvalidRoll = 15,
    InvalidPlayerIndex = 16,
    InvalidQuickChatPreset = 17,
    InvalidRenderType = 18,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct ScoreInfo {
    pub Score: ::std::os::raw::c_int,
    pub Goals: ::std::os::raw::c_int,
    pub OwnGoals: ::std::os::raw::c_int,
    pub Assists: ::std::os::raw::c_int,
    pub Saves: ::std::os::raw::c_int,
    pub Shots: ::std::os::raw::c_int,
    pub Demolitions: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Vector3 {
    pub X: f32,
    pub Y: f32,
    pub Z: f32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Rotator {
    pub Pitch: f32,
    pub Yaw: f32,
    pub Roll: f32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Physics {
    pub Location: Vector3,
    pub Rotation: Rotator,
    pub Velocity: Vector3,
    pub AngularVelocity: Vector3,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct PlayerInfo {
    pub Physics: Physics,
    pub Score: ScoreInfo,
    pub Demolished: bool,
    pub OnGround: bool,
    pub SuperSonic: bool,
    pub Bot: bool,
    pub Jumped: bool,
    pub DoubleJumped: bool,
    pub Name: [u16; 32usize],
    pub Team: ::std::os::raw::c_uchar,
    pub Boost: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct BoostInfo {
    pub Active: bool,
    pub Timer: f32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Touch {
    pub PlayerName: [u16; 32usize],
    pub TimeSeconds: f32,
    pub HitLocation: Vector3,
    pub HitNormal: Vector3,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct BallInfo {
    pub Physics: Physics,
    pub LatestTouch: Touch,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct GameInfo {
    pub TimeSeconds: f32,
    pub GameTimeRemaining: f32,
    pub OverTime: bool,
    pub UnlimitedTime: bool,
    pub RoundActive: bool,
    pub KickoffPause: bool,
    pub MatchEnded: bool,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct LiveDataPacket {
    pub GameCars: [PlayerInfo; 10usize],
    pub NumCars: ::std::os::raw::c_int,
    pub GameBoosts: [BoostInfo; 50usize],
    pub NumBoosts: ::std::os::raw::c_int,
    pub GameBall: BallInfo,
    pub GameInfo: GameInfo,
}
impl Default for LiveDataPacket {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct GoalInfo {
    pub TeamNum: ::std::os::raw::c_uchar,
    pub Location: Vector3,
    pub Direction: Vector3,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct BoostPad {
    pub Location: Vector3,
    pub FullBoost: bool,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FieldInfo {
    pub BoostPads: [BoostPad; 50usize],
    pub NumBoosts: ::std::os::raw::c_int,
    pub Goals: [GoalInfo; 2usize],
    pub NumGoals: ::std::os::raw::c_int,
}
impl Default for FieldInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct PlayerConfiguration {
    pub Bot: bool,
    pub RLBotControlled: bool,
    pub BotSkill: f32,
    pub HumanIndex: ::std::os::raw::c_int,
    pub Name: [u16; 32usize],
    pub Team: ::std::os::raw::c_uchar,
    pub TeamColorID: ::std::os::raw::c_uchar,
    pub CustomColorID: ::std::os::raw::c_uchar,
    pub CarID: ::std::os::raw::c_int,
    pub DecalID: ::std::os::raw::c_int,
    pub WheelsID: ::std::os::raw::c_int,
    pub BoostID: ::std::os::raw::c_int,
    pub AntennaID: ::std::os::raw::c_int,
    pub HatID: ::std::os::raw::c_int,
    pub PaintFinishID: ::std::os::raw::c_int,
    pub CustomFinishID: ::std::os::raw::c_int,
    pub EngineAudioID: ::std::os::raw::c_int,
    pub TrailsID: ::std::os::raw::c_int,
    pub GoalExplosionID: ::std::os::raw::c_int,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GameMode {
    Soccer = 0,
    Hoops = 1,
    Dropshot = 2,
    Hockey = 3,
    Rumble = 4,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GameMap {
    DFHStadium = 0,
    Mannfield = 1,
    ChampionsField = 2,
    UrbanCentral = 3,
    BeckwithPark = 4,
    UtopiaColiseum = 5,
    Wasteland = 6,
    NeoTokyo = 7,
    AquaDome = 8,
    StarbaseArc = 9,
    Farmstead = 10,
    DFHStadium_Stormy = 11,
    DFHStadium_Day = 12,
    Mannfield_Stormy = 13,
    Mannfield_Night = 14,
    ChampionsField_Day = 15,
    BeckwithPark_Stormy = 16,
    BeckwithPark_Midnight = 17,
    UrbanCentral_Night = 18,
    UrbanCentral_Dawn = 19,
    UtopiaColiseum_Dusk = 20,
    DFHStadium_Snowy = 21,
    Mannfield_Snowy = 22,
    UtopiaColiseum_Snowy = 23,
    Badlands = 24,
    Badlands_Night = 25,
    TokyoUnderpass = 26,
    Arctagon = 27,
    Pillars = 28,
    Cosmic = 29,
    DoubleGoal = 30,
    Octagon = 31,
    Underpass = 32,
    UtopiaRetro = 33,
    Hoops_DunkHouse = 34,
    DropShot_Core707 = 35,
    ThrowbackStadium = 36,
    Workshop_Aerial_Map = 37,
    Workshop_BeachVolley = 38,
    Workshop_DribblingChallenge2 = 39,
    Workshop_DribblingChallenge = 40,
    Workshop_ObstacleCourse2 = 41,
    Workshop_ObstacleCourse = 42,
    Workshop_ShipYarr = 43,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MatchLength {
    Five_Minutes = 0,
    Ten_Minutes = 1,
    Twenty_Minutes = 2,
    Unlimited = 3,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MaxScore {
    Unlimited = 0,
    One_Goal = 1,
    Three_Goals = 2,
    Five_Goals = 3,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum OvertimeOption {
    Unlimited = 0,
    Five_Max_First_Score = 1,
    Five_Max_Random_Team = 2,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SeriesLengthOption {
    Unlimited = 0,
    Three_Games = 1,
    Five_Games = 2,
    Seven_Games = 3,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GameSpeedOption {
    Default = 0,
    Slo_Mo = 1,
    Time_Warp = 2,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BallMaxSpeedOption {
    Default = 0,
    Slow = 1,
    Fast = 2,
    Super_Fast = 3,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BallTypeOption {
    Default = 0,
    Cube = 1,
    Puck = 2,
    Basketball = 3,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BallWeightOption {
    Default = 0,
    Light = 1,
    Heavy = 2,
    Super_Light = 3,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BallSizeOption {
    Default = 0,
    Small = 1,
    Large = 2,
    Gigantic = 3,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BallBouncinessOption {
    Default = 0,
    Low = 1,
    High = 2,
    Super_High = 3,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BoostOption {
    Normal_Boost = 0,
    Unlimited_Boost = 1,
    Slow_Recharge = 2,
    Rapid_Recharge = 3,
    No_Boost = 4,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum RumbleOption {
    None = 0,
    Default = 1,
    Slow = 2,
    Civilized = 3,
    Destruction_Derby = 4,
    Spring_Loaded = 5,
    Spikes_Only = 6,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BoostStrengthOption {
    One = 0,
    OneAndAHalf = 1,
    Two = 2,
    Ten = 3,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GravityOption {
    Default = 0,
    Low = 1,
    High = 2,
    Super_High = 3,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DemolishOption {
    Default = 0,
    Disabled = 1,
    Friendly_Fire = 2,
    On_Contact = 3,
    On_Contact_FF = 4,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum RespawnTimeOption {
    Three_Seconds = 0,
    Two_Seconds = 1,
    One_Seconds = 2,
    Disable_Goal_Reset = 3,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MutatorSettings {
    pub MatchLength: MatchLength,
    pub MaxScore: MaxScore,
    pub OvertimeOptions: OvertimeOption,
    pub SeriesLengthOptions: SeriesLengthOption,
    pub GameSpeedOptions: GameSpeedOption,
    pub BallMaxSpeedOptions: BallMaxSpeedOption,
    pub BallTypeOptions: BallTypeOption,
    pub BallWeightOptions: BallWeightOption,
    pub BallSizeOptions: BallSizeOption,
    pub BallBouncinessOptions: BallBouncinessOption,
    pub BoostOptions: BoostOption,
    pub RumbleOptions: RumbleOption,
    pub BoostStrengthOptions: BoostStrengthOption,
    pub GravityOptions: GravityOption,
    pub DemolishOptions: DemolishOption,
    pub RespawnTimeOptions: RespawnTimeOption,
}
impl Default for MutatorSettings {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MatchSettings {
    pub PlayerConfiguration: [PlayerConfiguration; 10usize],
    pub NumPlayers: ::std::os::raw::c_int,
    pub GameMode: GameMode,
    pub GameMap: GameMap,
    pub SkipReplays: bool,
    pub InstantStart: bool,
    pub MutatorSettings: MutatorSettings,
}
impl Default for MatchSettings {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct PlayerInput {
    pub Throttle: f32,
    pub Steer: f32,
    pub Pitch: f32,
    pub Yaw: f32,
    pub Roll: f32,
    pub Jump: bool,
    pub Boost: bool,
    pub Handbrake: bool,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum QuickChatPreset {
    Information_IGotIt = 0,
    Information_NeedBoost = 1,
    Information_TakeTheShot = 2,
    Information_Defending = 3,
    Information_GoForIt = 4,
    Information_Centering = 5,
    Information_AllYours = 6,
    Information_InPosition = 7,
    Information_Incoming = 8,
    Compliments_NiceShot = 9,
    Compliments_GreatPass = 10,
    Compliments_Thanks = 11,
    Compliments_WhatASave = 12,
    Compliments_NiceOne = 13,
    Compliments_WhatAPlay = 14,
    Compliments_GreatClear = 15,
    Compliments_NiceBlock = 16,
    Reactions_OMG = 17,
    Reactions_Noooo = 18,
    Reactions_Wow = 19,
    Reactions_CloseOne = 20,
    Reactions_NoWay = 21,
    Reactions_HolyCow = 22,
    Reactions_Whew = 23,
    Reactions_Siiiick = 24,
    Reactions_Calculated = 25,
    Reactions_Savage = 26,
    Reactions_Okay = 27,
    Apologies_Cursing = 28,
    Apologies_NoProblem = 29,
    Apologies_Whoops = 30,
    Apologies_Sorry = 31,
    Apologies_MyBad = 32,
    Apologies_Oops = 33,
    Apologies_MyFault = 34,
    PostGame_Gg = 35,
    PostGame_WellPlayed = 36,
    PostGame_ThatWasFun = 37,
    PostGame_Rematch = 38,
    PostGame_OneMoreGame = 39,
    PostGame_WhatAGame = 40,
    PostGame_NiceMoves = 41,
    PostGame_EverybodyDance = 42,
    MaxQuickChatPresets = 43,
}
pub type CallbackFunction = ::std::option::Option<
    unsafe extern "C" fn(id: ::std::os::raw::c_uint, status: RLBotCoreStatus),
>;