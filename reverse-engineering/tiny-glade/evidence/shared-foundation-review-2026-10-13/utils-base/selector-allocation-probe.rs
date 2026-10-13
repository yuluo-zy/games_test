//! Review-only allocation probe; the pre-change implementation is archived.
use std::{alloc::{GlobalAlloc,Layout,System},sync::atomic::{AtomicUsize,Ordering},time::Instant};
use serde::Deserialize;
#[path="selective-before.rs"] mod baseline;
struct Counting;
static ALLOCATIONS:AtomicUsize=AtomicUsize::new(0);
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self,layout:Layout)->*mut u8 {
        ALLOCATIONS.fetch_add(1,Ordering::Relaxed);
        unsafe {System.alloc(layout)}
    }
    unsafe fn dealloc(&self,p:*mut u8,layout:Layout) {unsafe {System.dealloc(p,layout)}}
    unsafe fn realloc(&self,p:*mut u8,layout:Layout,n:usize)->*mut u8 {
        ALLOCATIONS.fetch_add(1,Ordering::Relaxed);
        unsafe {System.realloc(p,layout,n)}
    }
}
#[global_allocator] static ALLOCATOR:Counting=Counting;
fn before(input:&[u8])->Vec<u64> {
    let mut d=serde_json::Deserializer::from_slice(input);
    baseline::SaveHistorySnapshots::<Vec<u64>>::deserialize(&mut d).unwrap().value.value
}
fn after(input:&[u8])->Vec<u64> {
    let mut d=serde_json::Deserializer::from_slice(input);
    utils::selective_deserialize::SaveHistorySnapshots::<Vec<u64>>::deserialize(&mut d).unwrap().value.value
}
fn main() {
    let keys=50_000;
    let mut input=String::from("{");
    for i in 0..keys {input.push_str(&format!("\"ignored{i}\":0,"));}
    input.push_str("\"History\":{\"snapshots\":[7,9]}}");
    let mut counts=Vec::new();
    for (name,read) in [("before",before as fn(&[u8])->Vec<u64>),("after",after)] {
        let started=Instant::now();
        ALLOCATIONS.store(0,Ordering::Relaxed);
        let result=read(input.as_bytes());
        let count=ALLOCATIONS.load(Ordering::Relaxed);
        assert_eq!(result,[7,9]);
        counts.push(count);
        println!("{name}: allocations={count}, micros={}",started.elapsed().as_micros());
    }
    assert_eq!(counts,[keys+3,1]);
    println!("50,000 ignored keys: same payload, 50,002 key allocations removed. Timing is one local debug probe, not a game benchmark.");
}
