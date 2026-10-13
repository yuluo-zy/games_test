//! 测试台占位：slotmap 1.0.6 的最小同形实现（离线缓存无 slotmap）。仅复刻 country-core 用到的面：
//! 哨兵槽 0、version 奇数 = 占用、`iter()` 跳过哨兵并带键。
pub trait Key: Copy {
    fn data(&self) -> (u32, u32);
    fn from_data(idx: u32, version: u32) -> Self;
}
#[macro_export]
macro_rules! new_key_type {
    ( $(#[$outer:meta])* $vis:vis struct $name:ident; ) => {
        $(#[$outer])*
        #[derive(Copy, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
        #[repr(transparent)]
        $vis struct $name(u64);
        impl $crate::Key for $name {
            fn data(&self) -> (u32, u32) { ((self.0 & 0xffff_ffff) as u32, (self.0 >> 32) as u32) }
            fn from_data(i: u32, v: u32) -> Self { $name(((v as u64) << 32) | i as u64) }
        }
    };
}
pub mod basic {
    use super::Key;
    use core::marker::PhantomData;
    #[derive(Debug, Clone)]
    struct Slot<V> { value: Option<V>, version: u32 }
    #[derive(Debug, Clone)]
    pub struct SlotMap<K: Key, V> { slots: Vec<Slot<V>>, free: Vec<u32>, num_elems: u32, _k: PhantomData<K> }
    impl<K: Key, V> Default for SlotMap<K, V> { fn default() -> Self { Self::new() } }
    impl<K: Key, V> SlotMap<K, V> {
        pub fn new() -> Self { SlotMap { slots: vec![Slot { value: None, version: 0 }], free: vec![], num_elems: 0, _k: PhantomData } }
        pub fn len(&self) -> usize { self.num_elems as usize }
        pub fn insert(&mut self, v: V) -> K {
            self.num_elems += 1;
            if let Some(i) = self.free.pop() {
                let s = &mut self.slots[i as usize]; s.version += 1; s.value = Some(v); K::from_data(i, s.version)
            } else { self.slots.push(Slot { value: Some(v), version: 1 }); K::from_data(self.slots.len() as u32 - 1, 1) }
        }
        pub fn get(&self, k: K) -> Option<&V> {
            let (i, v) = k.data(); self.slots.get(i as usize).filter(|s| s.version == v).and_then(|s| s.value.as_ref())
        }
        pub fn get_mut(&mut self, k: K) -> Option<&mut V> {
            let (i, v) = k.data(); self.slots.get_mut(i as usize).filter(|s| s.version == v).and_then(|s| s.value.as_mut())
        }
        pub fn remove(&mut self, k: K) -> Option<V> {
            let (i, v) = k.data();
            let s = self.slots.get_mut(i as usize)?; if s.version != v { return None; }
            let r = s.value.take()?; s.version += 1; self.free.push(i); self.num_elems -= 1; Some(r)
        }
        pub fn clear(&mut self) { let ks: Vec<K> = self.iter().map(|(k, _)| k).collect(); for k in ks { self.remove(k); } }
        pub fn iter(&self) -> Iter<'_, K, V> { Iter { it: self.slots.iter().enumerate().skip(1), _k: PhantomData } }
    }
    pub struct Iter<'a, K: Key, V> { it: core::iter::Skip<core::iter::Enumerate<core::slice::Iter<'a, Slot<V>>>>, _k: PhantomData<K> }
    impl<'a, K: Key, V> Iterator for Iter<'a, K, V> {
        type Item = (K, &'a V);
        fn next(&mut self) -> Option<Self::Item> {
            for (i, s) in self.it.by_ref() { if let Some(v) = &s.value { return Some((K::from_data(i as u32, s.version), v)); } }
            None
        }
    }
}
pub use basic::SlotMap;
