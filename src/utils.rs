#[repr(u8)]
pub enum Commands {
    List,
    Upload,
    Delete,
    Download,
    None
}
impl From<Commands> for u8 {
    fn from(m: Commands) -> u8 {
        m as u8
    }
}

impl Into<Commands> for u8 {
    fn into(self) -> Commands {
        match self {
            0 => Commands::List,
            1 => Commands::Upload,
            2 => Commands::Delete,
            3 => Commands::Download,
            _ => Commands::None,
        }
    }
}