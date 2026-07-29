use ui::voices::{voice_label, VOICES};

#[test]
fn voice_label_returns_formatted_string() {
    assert_eq!(voice_label("af_heart"), "Heart (American Female)");
    assert_eq!(voice_label("am_adam"), "Adam (American Male)");
    assert_eq!(voice_label("bf_emma"), "Emma (British Female)");
    assert_eq!(voice_label("bm_george"), "George (British Male)");
}

#[test]
fn voice_label_falls_back_to_id() {
    assert_eq!(voice_label("unknown_voice"), "unknown_voice");
}

#[test]
fn all_voices_have_unique_ids() {
    let mut ids: Vec<&str> = VOICES.iter().map(|(id, _, _, _)| *id).collect();
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), VOICES.len());
}
