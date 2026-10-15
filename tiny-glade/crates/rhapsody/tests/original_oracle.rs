//! 原版机器码 oracle 重放：生命周期与 resource-info（见 `common`）。
mod common;
use common::{jstr, ok_or_panic, replay, text, word};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::atomic::{AtomicU64, Ordering};

use rhapsody::raw::managed::tracking::{
    CommandState, ControlBlock, clone_reference, drop_reference, track_handle, track_sequence,
};

/// 原版 `track` 在状态非 Recording 时 panic；oracle 记录 `panic` 即 unwind。
fn panics_as_oracle<T>(f: impl FnOnce() -> T) -> &'static str {
    if catch_unwind(AssertUnwindSafe(f)).is_ok() { "ok" } else { "panic" }
}
use rhapsody::raw::resource_usage::{
    buffer_access_mask_to_usage_flags, image_access_mask_to_usage_flags,
};
use rhapsody::rg::resource_rules::{
    PassEndpoint, PassResourceAccessType, ResourceAccessRecord, ResourceLifetimeRecord,
    calculate_lifetime_events, link_up_next_access_chains,
};

#[test]
fn gpu_lifecycle_leaf_matches_original() {
    replay("gpu-lifecycle", "parallel-batch-02/gpu-lifecycle/leaf-oracle.jsonl.gz", |input| {
        let line = text(input);
        let f: Vec<_> = line.split_whitespace().collect();
        let old = word(f[1]);
        let out = match f[0] {
            "track" => {
                let control = ControlBlock::new(word(f[2]) as u32);
                control.last_sequence.store(old, Ordering::SeqCst);
                let verdict = panics_as_oracle(|| {
                    track_handle(
                        &control,
                        word(f[3]) as u32,
                        CommandState { tag: word(f[4]) as u32, sequence: word(f[5]) },
                    )
                });
                format!("{} {:x}", verdict, control.last_sequence.load(Ordering::SeqCst))
            }
            "seq" => {
                let tracker = AtomicU64::new(old);
                let verdict = panics_as_oracle(|| {
                    track_sequence(
                        &tracker,
                        CommandState { tag: word(f[2]) as u32, sequence: word(f[3]) },
                    )
                });
                format!("{} {:x}", verdict, tracker.load(Ordering::SeqCst))
            }
            "clone" => {
                let control = ControlBlock::new(0);
                control.references.store(old as i32, Ordering::SeqCst);
                format!("{:x}", clone_reference(&control))
            }
            "drop" => {
                let control = ControlBlock::new(0);
                control.references.store(old as i32, Ordering::SeqCst);
                let result = drop_reference(&control, word(f[2]) != 0);
                format!("{} {:x}", ok_or_panic(result), control.references.load(Ordering::SeqCst))
            }
            other => panic!("unknown op {other}"),
        };
        jstr(out)
    });
}

fn bytes(s: &str) -> Vec<u8> {
    if s == "-" {
        return Vec::new();
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap()).collect()
}
fn u32at(b: &[u8], o: usize) -> u32 {
    u32::from_le_bytes(b[o..o + 4].try_into().unwrap())
}
fn hex(b: &[u8]) -> String {
    if b.is_empty() { "-".into() } else { b.iter().map(|v| format!("{v:02x}")).collect() }
}

#[test]
fn resource_info_matches_original() {
    replay("resource-info", "resource-info-oracle.jsonl.gz", |input| {
        let line = text(input);
        let p: Vec<_> = line.split_whitespace().collect();
        let out = match p[0] {
            "concurrent" => {
                let a = PassResourceAccessType { access: word(p[1]) as u8, synchronization_group: word(p[2]) as u32 };
                let b = PassResourceAccessType { access: word(p[3]) as u8, synchronization_group: word(p[4]) as u32 };
                let (aw, bw) = (word(p[5]) != 0, word(p[6]) != 0);
                let mut calls = 0;
                let result = a.can_be_concurrent_with(&b, |_| {
                    calls += 1;
                    if calls == 1 { aw } else { bw }
                });
                format!("{} {calls}", u8::from(result))
            }
            "chain" => {
                let b = bytes(p[1]);
                let mut records: Vec<_> = b
                    .chunks_exact(20)
                    .map(|b| ResourceAccessRecord {
                        resource_index: u32at(b, 0),
                        previous_access_index: u32at(b, 4),
                        next_access_index: u32at(b, 8),
                        synchronization_group: u32at(b, 12),
                        access_code: b[16],
                        unobserved_padding: b[17..20].try_into().unwrap(),
                    })
                    .collect();
                let ok = catch_unwind(AssertUnwindSafe(|| link_up_next_access_chains(&mut records))).is_ok();
                let mut out = Vec::new();
                for r in records {
                    for w in [r.resource_index, r.previous_access_index, r.next_access_index, r.synchronization_group] {
                        out.extend(w.to_le_bytes());
                    }
                    out.push(r.access_code);
                    out.extend(r.unobserved_padding);
                }
                format!("{} {}", if ok { "ok" } else { "panic" }, hex(&out))
            }
            "lifetime" => {
                let b = bytes(p[1]);
                let records: Vec<_> = b
                    .chunks_exact(24)
                    .map(|b| ResourceLifetimeRecord {
                        first_pass: PassEndpoint { tag: u32at(b, 0), pass_index: u32at(b, 4) },
                        last_pass: PassEndpoint { tag: u32at(b, 8), pass_index: u32at(b, 12) },
                        first_access_index: u32at(b, 16),
                        last_access_index: u32at(b, 20),
                    })
                    .collect();
                let mut events: Vec<_> = bytes(p[2]).chunks_exact(4).map(|b| u32at(b, 0)).collect();
                calculate_lifetime_events(&records, &mut events);
                hex(&events.into_iter().flat_map(u32::to_le_bytes).collect::<Vec<_>>())
            }
            "buffer" => format!("{:08x}", buffer_access_mask_to_usage_flags(word(p[1]) as u32).as_raw()),
            "image" => match catch_unwind(|| image_access_mask_to_usage_flags(word(p[1]) as u32)) {
                Ok(flags) => format!("{:08x}", flags.as_raw()),
                Err(_) => "panic".into(),
            },
            other => panic!("unknown op {other}"),
        };
        jstr(out)
    });
}
