use std::str::FromStr;
use std::time::Duration;

use ::url::Url;

use crate::pasteboard::{Pasteboard, PasteboardError};
use crate::util::StripTrackers;

pub struct Manager {
    pasteboard: Pasteboard,
}

impl Manager {
    pub fn new(pasteboard: Pasteboard) -> Self {
        Manager { pasteboard }
    }

    pub fn watch(&mut self) {
        let mut last_change_count = self.pasteboard.change_count();
        loop {
            if self.pasteboard.change_count() != last_change_count {
                let _ = Self::strip_trackers(&mut self.pasteboard);
                last_change_count = self.pasteboard.change_count();
            }
            std::thread::sleep(Duration::from_millis(50));
        }
    }

    fn strip_trackers(pasteboard: &mut Pasteboard) -> Result<(), PasteboardError> {
        let url = {
            let mut url = match pasteboard.get_string().and_then(|s| Url::from_str(&s).ok()) {
                Some(url) => url,
                None => return Ok(()),
            };
            println!("Original URL: {:}", url);

            url.strip_trackers();
            url.to_string()
        };

        let result = pasteboard.set_string(&url);
        println!("Stripped URL: {:}", url);

        result
    }
}
