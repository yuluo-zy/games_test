//! 第三批 rg 收口：PGO 的 insert 语义。
use rhapsody::rg::executor::{PassPgoDuration, PassPgoFrames, PassPgoNameId};

#[test]
fn pgo_current_is_cleared_but_seed_free_and_extend_matches_insert_semantics() {
    let id = PassPgoNameId::from_name("w3_pass");
    let other = PassPgoNameId::from_name("w3_other");
    let mut frames = PassPgoFrames::default();
    frames.current.extend([
        PassPgoDuration::new(id, 40),
        PassPgoDuration::new(other, 10),
        PassPgoDuration::new(id, 25),
    ]);
    assert_eq!(frames.current.get(id), Some(25), "insert 覆盖");
    assert_eq!(frames.current.total_ns(), 75);
    assert_eq!(frames.current.max_ns(), 40);
    frames.end_frame();
    assert_eq!(frames.previous.get(id), Some(25));
    assert_eq!(frames.previous.total_ns(), 75);
    assert!(frames.current.is_empty());
    assert_eq!((frames.current.total_ns(), frames.current.max_ns()), (0, 0));
}
