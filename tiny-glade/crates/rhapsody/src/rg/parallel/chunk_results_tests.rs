#[test]
fn scoped_mutations_and_non_copy_results_preserve_chunk_order() {
    for len in [0, 1, 7, 8, 9, 17] {
        let mut items = (0..len).map(|index| vec![index]).collect::<Vec<_>>();
        let results = super::map_chunks(&mut items, |index, item| {
            if index % 2 == 0 { std::thread::yield_now(); }
            item.push(index + 1);
            format!("chunk-{index}")
        });
        assert_eq!(results.len(), len);
        for (index, (item, result)) in items.iter().zip(results).enumerate() {
            assert_eq!(item, &[index, index + 1]);
            assert_eq!(result, format!("chunk-{index}"));
        }
    }
}

#[test]
fn a_single_chunk_runs_on_the_calling_thread_and_several_chunks_do_not_need_to() {
    let caller = std::thread::current().id();
    let mut one = [0u8];
    let ids = super::map_chunks(&mut one, |_, _| std::thread::current().id());
    assert_eq!(ids.as_slice(), [caller]);
    let mut many = [0u8; 4];
    let ids = super::map_chunks(&mut many, |_, _| std::thread::current().id());
    assert_eq!(ids.len(), 4);
}
