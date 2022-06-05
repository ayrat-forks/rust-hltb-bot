use std::str::FromStr;

#[derive(Debug)]
pub struct GamePageData {
    pub steam: String
}
impl GamePageData {
    pub fn new(steam: String) -> Self {
        GamePageData {
            steam
        }
    }
}

#[derive(Debug)]
pub struct Entry {
    pub name: String,
    pub link: String,
    pub img: String,
    pub descr: String
}
impl Entry {
    pub fn new(name: String, link: String, img: String, descr: String) -> Entry {
        Entry {
            name,
            img,
            link,
            descr
        }
    }
}

#[derive(Debug)]
pub struct FullEntry {
    pub entry: Entry,
    pub page_data: Option<GamePageData>
}
impl FullEntry {
    pub fn new(entry: Entry, page_data: Option<GamePageData>) -> FullEntry {
        FullEntry {
            entry,
            page_data
        }
    }
}

#[derive(Debug)]
pub enum RunMode {
    Polling,
    WebHook
}
#[derive(Debug)]
pub struct EnumParseError(String);

impl FromStr for RunMode {
    type Err = EnumParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_ref() {
            "polling" => Ok(RunMode::Polling),
            "webhook" => Ok(RunMode::WebHook),
            _ => Err(EnumParseError(format!("Unknown RunMode {}", s)))
        }
    }
}