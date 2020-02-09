pub enum DemoProtocol {
    // Version 7 & 8
    Signon = 1,
    Packet = 2,
    SyncTick = 3,
    ConsoleCMD = 4,
    UserCMD = 5,
    DataTables = 6,
    Stop = 7,

    /// Version 14 & 15, this is stringtables
    /// Version 36+ , this is customdata
    CustomData = 8,

    /// Exists only with version 36+
    StringTables = 9,

    Unknown,
}

impl From<u8> for DemoProtocol {
    fn from(v: u8) -> Self {
        match v {
            1 => Self::Signon,
            2 => Self::Packet,
            3 => Self::SyncTick,
            4 => Self::ConsoleCMD,
            5 => Self::UserCMD,
            6 => Self::DataTables,
            7 => Self::Stop,
            8 => Self::CustomData,
            9 => Self::StringTables,
            _ => Self::Unknown,
        }
    } 
}
