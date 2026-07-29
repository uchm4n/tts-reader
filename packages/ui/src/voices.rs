//! Voice definitions for Kokoro TTS.

/// All available voices: (id, display_name, nationality, gender).
pub const VOICES: &[(&str, &str, &str, &str)] = &[
    ("af_heart", "Heart", "American", "Female"),
    ("af_alloy", "Alloy", "American", "Female"),
    ("af_aoede", "Aoede", "American", "Female"),
    ("af_bella", "Bella", "American", "Female"),
    ("af_jessica", "Jessica", "American", "Female"),
    ("af_kore", "Kore", "American", "Female"),
    ("af_nicole", "Nicole", "American", "Female"),
    ("af_nova", "Nova", "American", "Female"),
    ("af_river", "River", "American", "Female"),
    ("af_sarah", "Sarah", "American", "Female"),
    ("af_sky", "Sky", "American", "Female"),
    ("am_adam", "Adam", "American", "Male"),
    ("am_echo", "Echo", "American", "Male"),
    ("am_eric", "Eric", "American", "Male"),
    ("am_fenrir", "Fenrir", "American", "Male"),
    ("am_liam", "Liam", "American", "Male"),
    ("am_michael", "Michael", "American", "Male"),
    ("am_onyx", "Onyx", "American", "Male"),
    ("am_puck", "Puck", "American", "Male"),
    ("am_santa", "Santa", "American", "Male"),
    ("bf_alice", "Alice", "British", "Female"),
    ("bf_emma", "Emma", "British", "Female"),
    ("bf_isabella", "Isabella", "British", "Female"),
    ("bf_lily", "Lily", "British", "Female"),
    ("bm_daniel", "Daniel", "British", "Male"),
    ("bm_fable", "Fable", "British", "Male"),
    ("bm_george", "George", "British", "Male"),
    ("bm_lewis", "Lewis", "British", "Male"),
];

/// Return the formatted label for a voice ID, e.g. "Heart (American Female)".
/// Falls back to the raw ID if not found.
pub fn voice_label(id: &str) -> String {
    for (vid, name, nationality, gender) in VOICES {
        if *vid == id {
            return format!("{name} ({nationality} {gender})");
        }
    }
    id.to_string()
}
