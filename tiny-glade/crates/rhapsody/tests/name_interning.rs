//! Interned names: a bounded set of names costs nothing per frame. This test
//! is alone in its binary so that no concurrent test interns names meanwhile.
use rhapsody::rg::executor::{PassPgoDuration, PassPgoFrames, PassPgoNameId};

#[test]
fn repeating_the_same_names_never_grows_the_intern_table_but_new_names_do() {
    // Interned names live for the process: the contract is that a bounded set
    // of names costs nothing per frame, and only a genuinely new string grows
    // the table. (Names that embed a frame counter or address would leak.)
    let mut frames = PassPgoFrames::default();
    let names = ["pass-a", "pass-b", "pass-c"];
    for name in names {
        frames
            .current
            .record(PassPgoDuration::new(PassPgoNameId::from_name(name), 1));
    }
    let settled = rhapsody::resource::ResourceName::interned_count();
    for _ in 0..1000 {
        for name in names {
            frames
                .current
                .record(PassPgoDuration::new(PassPgoNameId::from_name(name), 1));
        }
        frames.end_frame();
    }
    assert_eq!(rhapsody::resource::ResourceName::interned_count(), settled);
    frames.current.record(PassPgoDuration::new(
        PassPgoNameId::from_name("a-new-pass"),
        1,
    ));
    assert_eq!(
        rhapsody::resource::ResourceName::interned_count(),
        settled + 1
    );
    // Looking a name up never interns it.
    assert_eq!(frames.current.duration_ns("never-recorded-anywhere"), 0);
    assert_eq!(
        rhapsody::resource::ResourceName::interned_count(),
        settled + 1
    );
}
