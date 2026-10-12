//! Disposable differential runner for recovered private renderer functions.
use std::io::{self, BufRead};
use std::panic::{AssertUnwindSafe, catch_unwind};
use tg_rhapsody_reconstruction::resource_info::{
    PassEndpoint, PassResourceAccessType, ResourceAccessRecord, ResourceLifetimeRecord,
    calculate_lifetime_events, link_up_next_access_chains,
};

fn word(s: &str) -> u32 {
    u32::from_str_radix(s, 16).expect("hex word")
}
fn bytes(s: &str) -> Vec<u8> {
    if s == "-" {
        return vec![];
    }
    assert_eq!(s.len() % 2, 0);
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}
fn u32at(b: &[u8], o: usize) -> u32 {
    u32::from_le_bytes(b[o..o + 4].try_into().unwrap())
}
fn hex(b: &[u8]) -> String {
    b.iter().map(|v| format!("{v:02x}")).collect()
}
fn main() {
    std::panic::set_hook(Box::new(|_| {}));
    assert_eq!(std::mem::size_of::<ResourceAccessRecord>(), 20);
    assert_eq!(std::mem::size_of::<ResourceLifetimeRecord>(), 24);
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let p: Vec<_> = line.split_whitespace().collect();
        match p[0] {
            "concurrent" => {
                let a = PassResourceAccessType {
                    access: word(p[1]) as u8,
                    synchronization_group: word(p[2]),
                };
                let b = PassResourceAccessType {
                    access: word(p[3]) as u8,
                    synchronization_group: word(p[4]),
                };
                // Classifications are INPUTS captured from the original public
                // dependency by the Python oracle, not a replacement is_write.
                let aw = word(p[5]) != 0;
                let bw = word(p[6]) != 0;
                let mut calls = 0;
                let result = a.can_be_concurrent_with(&b, |_| {
                    calls += 1;
                    if calls == 1 { aw } else { bw }
                });
                println!("{} {calls}", u8::from(result));
            }
            "chain" => {
                let b = bytes(p[1]);
                assert_eq!(b.len() % 20, 0);
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
                let ok = catch_unwind(AssertUnwindSafe(|| {
                    link_up_next_access_chains(&mut records)
                }))
                .is_ok();
                let mut out = Vec::with_capacity(b.len());
                for r in records {
                    for w in [
                        r.resource_index,
                        r.previous_access_index,
                        r.next_access_index,
                        r.synchronization_group,
                    ] {
                        out.extend(w.to_le_bytes());
                    }
                    out.push(r.access_code);
                    out.extend(r.unobserved_padding);
                }
                println!(
                    "{} {}",
                    if ok { "ok" } else { "panic" },
                    if out.is_empty() {
                        "-".into()
                    } else {
                        hex(&out)
                    }
                );
            }
            "lifetime" => {
                let b = bytes(p[1]);
                let initial = bytes(p[2]);
                assert_eq!(b.len() % 24, 0);
                let records: Vec<_> = b
                    .chunks_exact(24)
                    .map(|b| ResourceLifetimeRecord {
                        first_pass: PassEndpoint {
                            tag: u32at(b, 0),
                            pass_index: u32at(b, 4),
                        },
                        last_pass: PassEndpoint {
                            tag: u32at(b, 8),
                            pass_index: u32at(b, 12),
                        },
                        first_access_index: u32at(b, 16),
                        last_access_index: u32at(b, 20),
                    })
                    .collect();
                let mut events: Vec<_> = initial.chunks_exact(4).map(|b| u32at(b, 0)).collect();
                calculate_lifetime_events(&records, &mut events);
                let out: Vec<_> = events.into_iter().flat_map(u32::to_le_bytes).collect();
                println!(
                    "{}",
                    if out.is_empty() {
                        "-".into()
                    } else {
                        hex(&out)
                    }
                );
            }
            "buffer" => println!(
                "{:08x}",
                tg_rhapsody_reconstruction::resource_usage::buffer_access_mask_to_usage_flags(
                    word(p[1])
                )
                .as_raw()
            ),
            "image" => {
                let result = catch_unwind(|| {
                    tg_rhapsody_reconstruction::resource_usage::image_access_mask_to_usage_flags(
                        word(p[1]),
                    )
                });
                match result {
                    Ok(flags) => println!("{:08x}", flags.as_raw()),
                    Err(_) => println!("panic"),
                };
            }
            _ => panic!("unknown oracle command"),
        }
    }
}
