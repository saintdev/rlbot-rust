/* automatically generated by rust-bindgen */

#![allow(non_camel_case_types, non_snake_case, missing_docs)]

#[repr(C)]
#[derive(Debug)]
pub struct ByteBuffer {
    pub ptr: *mut ::std::os::raw::c_void,
    pub size: ::std::os::raw::c_int,
}
impl Default for ByteBuffer {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct ScoreInfo {
    pub score: ::std::os::raw::c_int,
    pub goals: ::std::os::raw::c_int,
    pub ownGoals: ::std::os::raw::c_int,
    pub assists: ::std::os::raw::c_int,
    pub saves: ::std::os::raw::c_int,
    pub shots: ::std::os::raw::c_int,
    pub demolitions: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Rotator {
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct BoxShape {
    pub length: f32,
    pub width: f32,
    pub height: f32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct SphereShape {
    pub diameter: f32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct CylinderShape {
    pub diameter: f32,
    pub height: f32,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CollisionShapeType {
    BoxType = 0,
    SphereType = 1,
    CylinderType = 2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CollisionShape {
    pub type_: CollisionShapeType,
    pub box_: BoxShape,
    pub sphere: SphereShape,
    pub cylinder: CylinderShape,
}
impl Default for CollisionShape {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Physics {
    pub location: Vector3,
    pub rotation: Rotator,
    pub velocity: Vector3,
    pub angularVelocity: Vector3,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct PlayerInfo {
    pub physics: Physics,
    pub score: ScoreInfo,
    pub demolished: bool,
    pub onGround: bool,
    pub superSonic: bool,
    pub bot: bool,
    pub jumped: bool,
    pub doubleJumped: bool,
    pub name: [u32; 32usize],
    pub team: ::std::os::raw::c_uchar,
    pub boost: ::std::os::raw::c_int,
    pub hitbox: BoxShape,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct BoostInfo {
    pub active: bool,
    pub timer: f32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct TileInfo {
    pub tileState: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct TeamInfo {
    pub teamIndex: ::std::os::raw::c_int,
    pub score: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Touch {
    pub playerName: [u32; 32usize],
    pub timeSeconds: f32,
    pub hitLocation: Vector3,
    pub hitNormal: Vector3,
    pub team: ::std::os::raw::c_int,
    pub playerIndex: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct DropShotBallInfo {
    pub absorbedForce: f32,
    pub damageIndex: ::std::os::raw::c_int,
    pub forceAccumRecent: f32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Slice {
    pub physics: Physics,
    pub gameSeconds: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BallPredictionPacket {
    pub slice: [Slice; 360usize],
    pub numSlices: ::std::os::raw::c_int,
}
impl Default for BallPredictionPacket {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BallInfo {
    pub physics: Physics,
    pub latestTouch: Touch,
    pub dropShotInfo: DropShotBallInfo,
    pub collisionShape: CollisionShape,
}
impl Default for BallInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct GameInfo {
    pub timeSeconds: f32,
    pub gameTimeRemaining: f32,
    pub overTime: bool,
    pub unlimitedTime: bool,
    pub roundActive: bool,
    pub kickoffPause: bool,
    pub matchEnded: bool,
    pub worldGravityZ: f32,
    pub gameSpeed: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct LiveDataPacket {
    pub gameCars: [PlayerInfo; 64usize],
    pub numCars: ::std::os::raw::c_int,
    pub gameBoosts: [BoostInfo; 50usize],
    pub numBoosts: ::std::os::raw::c_int,
    pub gameBall: BallInfo,
    pub gameInfo: GameInfo,
    pub gameTiles: [TileInfo; 200usize],
    pub numTiles: ::std::os::raw::c_int,
    pub teams: [TeamInfo; 2usize],
    pub numTeams: ::std::os::raw::c_int,
}
impl Default for LiveDataPacket {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct GoalInfo {
    pub teamNum: ::std::os::raw::c_uchar,
    pub location: Vector3,
    pub direction: Vector3,
    pub width: f32,
    pub height: f32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct BoostPad {
    pub location: Vector3,
    pub fullBoost: bool,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FieldInfo {
    pub boostPads: [BoostPad; 50usize],
    pub numBoosts: ::std::os::raw::c_int,
    pub goals: [GoalInfo; 200usize],
    pub numGoals: ::std::os::raw::c_int,
}
impl Default for FieldInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct PlayerConfiguration {
    pub bot: bool,
    pub rlbotControlled: bool,
    pub botSkill: f32,
    pub humanIndex: ::std::os::raw::c_int,
    pub name: [u32; 32usize],
    pub team: ::std::os::raw::c_uchar,
    pub teamColorID: ::std::os::raw::c_uchar,
    pub customColorID: ::std::os::raw::c_uchar,
    pub carID: ::std::os::raw::c_int,
    pub decalID: ::std::os::raw::c_int,
    pub wheelsID: ::std::os::raw::c_int,
    pub boostID: ::std::os::raw::c_int,
    pub antennaID: ::std::os::raw::c_int,
    pub hatID: ::std::os::raw::c_int,
    pub paintFinishID: ::std::os::raw::c_int,
    pub customFinishID: ::std::os::raw::c_int,
    pub engineAudioID: ::std::os::raw::c_int,
    pub trailsID: ::std::os::raw::c_int,
    pub goalExplosionID: ::std::os::raw::c_int,
    pub carPaintID: ::std::os::raw::c_int,
    pub decalPaintID: ::std::os::raw::c_int,
    pub wheelsPaintID: ::std::os::raw::c_int,
    pub boostPaintID: ::std::os::raw::c_int,
    pub antennaPaintID: ::std::os::raw::c_int,
    pub hatPaintID: ::std::os::raw::c_int,
    pub trailsPaintID: ::std::os::raw::c_int,
    pub goalExplosionPaintID: ::std::os::raw::c_int,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GameMode {
    Soccer = 0,
    Hoops = 1,
    Dropshot = 2,
    Hockey = 3,
    Rumble = 4,
}
#[repr(u32)]
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
    SaltyShores = 11,
    DFHStadium_Stormy = 12,
    DFHStadium_Day = 13,
    Mannfield_Stormy = 14,
    Mannfield_Night = 15,
    ChampionsField_Day = 16,
    BeckwithPark_Stormy = 17,
    BeckwithPark_Midnight = 18,
    UrbanCentral_Night = 19,
    UrbanCentral_Dawn = 20,
    UtopiaColiseum_Dusk = 21,
    DFHStadium_Snowy = 22,
    Mannfield_Snowy = 23,
    UtopiaColiseum_Snowy = 24,
    Badlands = 25,
    Badlands_Night = 26,
    TokyoUnderpass = 27,
    Arctagon = 28,
    Pillars = 29,
    Cosmic = 30,
    DoubleGoal = 31,
    Octagon = 32,
    Underpass = 33,
    UtopiaRetro = 34,
    Hoops_DunkHouse = 35,
    DropShot_Core707 = 36,
    ThrowbackStadium = 37,
    Workshop_Aerial_Map = 38,
    Workshop_BeachVolley = 39,
    Workshop_DribblingChallenge2 = 40,
    Workshop_DribblingChallenge = 41,
    Workshop_ObstacleCourse2 = 42,
    Workshop_ObstacleCourse = 43,
    Workshop_ShipYarr = 44,
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
    Spike_Rush = 7,
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
    pub matchLength: MatchLength,
    pub maxScore: MaxScore,
    pub overtimeOptions: OvertimeOption,
    pub seriesLengthOptions: SeriesLengthOption,
    pub gameSpeedOptions: GameSpeedOption,
    pub ballMaxSpeedOptions: BallMaxSpeedOption,
    pub ballTypeOptions: BallTypeOption,
    pub ballWeightOptions: BallWeightOption,
    pub ballSizeOptions: BallSizeOption,
    pub ballBouncinessOptions: BallBouncinessOption,
    pub boostOptions: BoostOption,
    pub rumbleOptions: RumbleOption,
    pub boostStrengthOptions: BoostStrengthOption,
    pub gravityOptions: GravityOption,
    pub demolishOptions: DemolishOption,
    pub respawnTimeOptions: RespawnTimeOption,
}
impl Default for MutatorSettings {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ExistingMatchBehavior {
    Restart_If_Different = 0,
    Restart = 1,
    Continue_And_Spawn = 2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MatchSettings {
    pub playerConfiguration: [PlayerConfiguration; 64usize],
    pub numPlayers: ::std::os::raw::c_int,
    pub gameMode: GameMode,
    pub gameMap: GameMap,
    pub skipReplays: bool,
    pub instantStart: bool,
    pub mutatorSettings: MutatorSettings,
    pub existingMatchBehavior: ExistingMatchBehavior,
    pub enableLockstep: bool,
}
impl Default for MatchSettings {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct PlayerInput {
    pub throttle: f32,
    pub steer: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
    pub jump: bool,
    pub boost: bool,
    pub handbrake: bool,
    pub useItem: bool,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct RigidBodyState {
    pub frame: ::std::os::raw::c_int,
    pub location: Vector3,
    pub rotation: Quaternion,
    pub velocity: Vector3,
    pub angularVelocity: Vector3,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct PlayerRigidBodyState {
    pub state: RigidBodyState,
    pub input: PlayerInput,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct BallRigidBodyState {
    pub state: RigidBodyState,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RigidBodyTick {
    pub ball: BallRigidBodyState,
    pub players: [PlayerRigidBodyState; 64usize],
    pub numPlayers: ::std::os::raw::c_int,
}
impl Default for RigidBodyTick {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(u32)]
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
    QuickChatRateExceeded = 19,
    NotInitialized = 20,
}
#[repr(u32)]
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
    MaxPysonixQuickChatPresets = 43,
    Custom_Toxic_WasteCPU = 44,
    Custom_Toxic_GitGut = 45,
    Custom_Toxic_DeAlloc = 46,
    Custom_Toxic_404NoSkill = 47,
    Custom_Toxic_CatchVirus = 48,
    Custom_Useful_Passing = 49,
    Custom_Useful_Faking = 50,
    Custom_Useful_Demoing = 51,
    Custom_Useful_Bumping = 52,
    Custom_Compliments_TinyChances = 53,
    Custom_Compliments_SkillLevel = 54,
    Custom_Compliments_proud = 55,
    Custom_Compliments_GC = 56,
    Custom_Compliments_Pro = 57,
    MaxQuickChatPresets = 58,
}
