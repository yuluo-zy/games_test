use super::*;

/// `Cell` 使 `NotThreadSafe` 不是 `Sync`，用于验证句柄的线程安全性与 `T` 无关。
/// 该类型只出现在 trait 约束断言里，从不真正构造，字段读取的 `dead_code` 提示可忽略。
#[allow(dead_code)]
struct NotThreadSafe(std::cell::Cell<u8>);

#[derive(Debug, PartialEq)]
struct Buffer;
#[derive(Debug, PartialEq)]
struct Texture;

fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

#[test]
fn handle_is_a_compact_copy_key() {
    assert_eq!(std::mem::size_of::<Handle<Buffer>>(), 8);
    let first = Handle::<Buffer>::from_ffi(0);
    let second = first;
    assert_eq!(first, second, "句柄是 Copy，赋值不会丢失");
    assert!(!first.is_null() || first.index() == u32::MAX);
}

#[test]
fn handle_is_send_and_sync_regardless_of_t() {
    // `NotThreadSafe` 不是 Sync，但 `PhantomData<fn() -> T>` 不把这一约束带到句柄上。
    assert_send::<Handle<NotThreadSafe>>();
    assert_sync::<Handle<NotThreadSafe>>();
}

#[test]
fn insert_get_remove_round_trip() {
    let mut store = ResourceStore::<Buffer>::new();
    let key = store.insert(Buffer);
    assert_eq!(store.len(), 1);
    assert!(store.contains(key));
    assert!(store.get(key).is_some());
    assert_eq!(store.remove(key).map(|_| ()), Some(()));
    assert!(!store.contains(key));
    assert_eq!(store.get(key), None);
    assert_eq!(store.remove(key), None, "重复删除同样安全");
    assert!(store.is_empty());
}

#[test]
fn removed_slot_is_reused_with_a_fresh_generation() {
    let mut store = ResourceStore::<Texture>::new();
    let stale = store.insert(Texture);
    assert!(store.remove(stale).is_some());

    let reused = store.insert(Texture);
    assert_eq!(reused.index(), stale.index(), "复用同一个槽位下标");
    assert_ne!(reused, stale, "代际不同，旧句柄不能命中新资源");
    assert_eq!(store.get(stale), None, "旧句柄已失效");
    assert!(store.get(reused).is_some());
    assert_eq!(store.len(), 1);
}

#[test]
fn distinct_type_markers_share_indices_without_conflict() {
    let mut buffers = ResourceStore::<Buffer>::new();
    let mut textures = ResourceStore::<Texture>::new();
    let b = buffers.insert(Buffer);
    let t = textures.insert(Texture);
    // 两者下标相同，但类型不同，句柄不可互用（类型系统保证），存储互不影响。
    assert_eq!(b.index(), t.index());
    assert!(buffers.get(b).is_some());
    assert!(textures.get(t).is_some());
}

#[test]
fn iter_and_values_only_yield_live_entries() {
    let mut store = ResourceStore::<u32>::new();
    let a = store.insert(1);
    let b = store.insert(2);
    let _c = store.insert(3);
    assert!(store.remove(b).is_some());

    let keys: Vec<_> = store.iter().map(|(key, _)| key).collect();
    assert_eq!(keys.len(), 2);
    assert!(keys.contains(&a));
    assert!(!keys.contains(&b));
    assert_eq!(store.values().copied().sum::<u32>(), 4);
}

#[test]
fn retain_keeps_only_matching_values() {
    let mut store = ResourceStore::<u32>::new();
    let keep_me = store.insert(2);
    let drop_me = store.insert(3);
    store.retain(|_, value| *value % 2 == 0);
    assert!(store.contains(keep_me));
    assert!(!store.contains(drop_me));
    assert_eq!(store.values().copied().collect::<Vec<_>>(), [2]);
}

#[test]
fn ffi_round_trip_preserves_identity() {
    let mut store = ResourceStore::<&'static str>::new();
    let key = store.insert("rhapsody");
    let restored = Handle::<&'static str>::from_ffi(key.as_ffi());
    assert_eq!(restored, key);
    assert_eq!(store.get(restored), Some(&"rhapsody"));
}
