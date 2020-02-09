pub struct DemoHeader {
    /// 8 characters, should be "HL2DEMO"+NULL
    pub header: String,

    /// Demo protocol version (stored in little endian)
    pub demo_protocol: i32,

    /// Network protocol version number (stored in little endian)
    pub network_protocol: i32,

    /// Server display name (260 characters long)
    pub server_name: String,

    /// Client name (260 characters long)
    pub client_name: String,

    /// Server map name (260 characters long)
    pub map_name: String,

    /// Game directory (260 characters long)
    pub game_dir: String,

    /// The length of the demo, in seconds
    pub playback_time: f32,

    /// The number of ticks in the demo
    pub ticks: i32,

    /// The number of frames in the demo
    pub frames: i32,

    /// Length of the signon data (Init for first frame)
    pub signon_len: i32,
}