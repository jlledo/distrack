use url::Url;

use crate::core;

pub trait StripTrackers {
    fn strip_trackers(&mut self);
}

impl StripTrackers for Url {
    fn strip_trackers(&mut self) {
        let pairs: Vec<(String, String)> = self
            .query_pairs()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect();

        self.set_query(None);

        for (key, value) in pairs {
            if core::should_strip(&key) {
                continue;
            }

            self.query_pairs_mut().append_pair(&key, &value);
        }
    }
}
