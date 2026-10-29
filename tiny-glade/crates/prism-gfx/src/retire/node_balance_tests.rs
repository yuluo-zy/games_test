use super::*;
fn check<T>(root: &Option<RetirementTicket<T>>) -> u8 {
    let Some(node) = root else { return 0 };
    let left = check(&node.left);
    let right = check(&node.right);
    assert!(left.abs_diff(right) <= 1);
    assert_eq!(node.height, 1 + left.max(right));
    node.height
}
#[test]
fn ordered_and_duplicate_nodes_remain_balanced_and_fifo() {
    let mut queue = Queue::default();
    for index in 0..8192 {
        let mut ticket = RetirementTicket::new(None).unwrap();
        ticket.value = (index / 4) as u64;
        ticket.item = Some(index);
        queue.insert(ticket);
    }
    assert!(check(&queue.root) < 20);
    for index in 0..8192 {
        let mut node = queue.pop().unwrap();
        assert_eq!(node.item.take(), Some(index));
        check(&queue.root);
    }
    assert!(queue.is_empty());
}
#[test]
fn ready_callback_can_reenter_the_same_retirement_queue() {
    let retirement = Retirement::new();
    retirement.try_enqueue(SyncSet::EMPTY, 1).unwrap();
    let mut observed = Vec::new();
    retirement.collect_limited(&CompletedValues::all(), 2, 2, |item| {
        observed.push(item);
        if item == 1 {
            retirement.try_enqueue(SyncSet::EMPTY, 2).unwrap();
        }
    });
    retirement.collect(&CompletedValues::all(), &mut observed);
    assert_eq!(observed, [1, 2]);
}
