// From https://github.com/rknightuk/TrackerZapper/blob/main/TrackerZapper/AppDelegate.swift
const TRACKER_PREFIXES: [&str; 46] = [
    "_bta_c",
    "_bta_tid",
    "_ga",
    "_hsenc",
    "_hsmi",
    "_ke",
    "_openstat",
    "dm_i",
    "ef_id",
    "epik",
    "fbclid",
    "gclid",
    "gclsrc",
    "gdffi",
    "gdfms",
    "gdftrk",
    "hsa_",
    "igshid",
    "matomo_",
    "mc_",
    "mkwid",
    "msclkid",
    "mtm_",
    "ns_",
    "oly_anon_id",
    "oly_enc_id",
    "otc",
    "pcrid",
    "piwik_",
    "pk_",
    "rb_clickid",
    "redirect_log_mongo_id",
    "redirect_mongo_id",
    "ref",
    "s_kwcid",
    "sb_referer_host",
    "soc_src",
    "soc_trk",
    "source", // Medium
    "spm",
    "sr_",
    "srcid",
    "stm_",
    "trk_",
    "utm_",
    "vero_",
];

pub fn should_strip(param: &str) -> bool {
    for prefix in TRACKER_PREFIXES {
        if param.starts_with(prefix) {
            return true;
        }
    }

    false
}
