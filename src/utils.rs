pub struct Window {
    pub width: u32,
    pub height: u32,
}

pub struct GameInformation {
    pub window: Window,
    pub version: String,
}

impl GameInformation {
    pub fn new() -> Self {
        let version = env!("CARGO_PKG_VERSION").to_owned();
        GameInformation {
            window: Window {
                width: 240,
                height: 460,
            },
            version,
        }
    }
}