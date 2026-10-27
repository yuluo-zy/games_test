use super::*;

#[test]
fn srcname_path_and_log_line_match_the_original_pieces() {
    assert_eq!(srcname_path(0xabc), "tmp/shader-cache/0xabc.srcname");
    let words = [0, 1, 0, 0, (3 << 24) | 42, 0xabc, 5, 7];
    let names = BTreeMap::from([(0xabc, "foo.slang".to_owned())]);
    let log = parse_gpu_debug_info(&words, &names).unwrap();
    assert_eq!(
        format_debug_record_line(0, &log.records[0]),
        "\u{1f525} 0: assertion failed (5 > 7) in foo.slang:42"
    );
    let unknown =
        parse_gpu_debug_info(&[0, 1, 0, 0, 9 << 24, 1, 0, 0], &BTreeMap::new()).unwrap();
    assert_eq!(
        format_debug_record_line(2, &unknown.records[0]),
        "\u{1f525} 2: ??? in ???:0"
    );
}
