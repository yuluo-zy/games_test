use std::io::{self, BufRead};
use std::panic::{AssertUnwindSafe, catch_unwind};

use tg_resource_info_candidate::{AccessDeclaration, ExportDeclaration, OriginKind, ResourceInfo};

fn number(value: &str) -> u32 {
    u32::from_str_radix(value, 16).unwrap()
}

fn main() {
    std::panic::set_hook(Box::new(|_| {}));
    let mut info = ResourceInfo::default();
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let sections: Vec<_> = line.split('|').collect();
        let origins: Vec<_> = sections[0]
            .split_whitespace()
            .map(|token| {
                let values: Vec<_> = token.split(',').map(number).collect();
                match values[0] {
                    0 => OriginKind::Tag0 {
                        descriptor_code: values[1] as u8,
                        image_creation_flags: values[2],
                        buffer_creation_flags: values[3],
                    },
                    1 => OriginKind::Tag1 {
                        type_code: values[1] as u8,
                    },
                    2 => OriginKind::Tag2 {
                        type_code: values[1] as u8,
                    },
                    _ => unreachable!(),
                }
            })
            .collect();
        let exports: Vec<_> = sections[1]
            .split_whitespace()
            .map(|token| {
                let values: Vec<_> = token.split(',').map(number).collect();
                ExportDeclaration {
                    tag: values[0] as u8,
                    resource_index: values[1],
                    access_code: values[2] as u8,
                }
            })
            .collect();
        let masks: Vec<_> = sections[2].split_whitespace().map(number).collect();
        let passes: Vec<Vec<_>> = if sections[3] == "-" {
            Vec::new()
        } else {
            sections[3]
                .split(';')
                .map(|pass| {
                    pass.split_whitespace()
                        .map(|token| {
                            let values: Vec<_> = token.split(',').map(number).collect();
                            AccessDeclaration {
                                resource_index: values[0],
                                synchronization_group: values[1],
                                access_code: values[2] as u8,
                            }
                        })
                        .collect()
                })
                .collect()
        };
        let passes: Vec<_> = passes.iter().map(Vec::as_slice).collect();
        let mut calls = Vec::new();
        let result = catch_unwind(AssertUnwindSafe(|| {
            info.calculate(&origins, &exports, &passes, |code| {
                calls.push(code);
                masks[usize::from(code)]
            });
        }));
        let accesses = info
            .accesses
            .iter()
            .map(|entry| {
                format!(
                    "{:x},{:x},{:x},{:x},{:x}",
                    entry.resource_index,
                    entry.previous_access_index,
                    entry.next_access_index,
                    entry.synchronization_group,
                    entry.access_code
                )
            })
            .collect::<Vec<_>>()
            .join(" ");
        let ranges = info
            .pass_access_ranges
            .iter()
            .map(|(start, end)| format!("{:x},{:x}", start.0, end.0))
            .collect::<Vec<_>>()
            .join(" ");
        let lifetimes = info
            .lifetimes
            .iter()
            .map(|entry| {
                format!(
                    "{:x},{:x},{:x},{:x},{:x},{:x}",
                    entry.first_pass.tag,
                    if entry.first_pass.tag == 1 {
                        entry.first_pass.pass_index
                    } else {
                        0
                    },
                    entry.last_pass.tag,
                    if entry.last_pass.tag == 1 {
                        entry.last_pass.pass_index
                    } else {
                        0
                    },
                    entry.first_access_index,
                    entry.last_access_index
                )
            })
            .collect::<Vec<_>>()
            .join(" ");
        let words = |values: &[u32]| {
            values
                .iter()
                .map(|value| format!("{value:x}"))
                .collect::<Vec<_>>()
                .join(" ")
        };
        let mut exports: Vec<_> = info
            .exported_indices
            .iter()
            .map(|&value| u32::from(value))
            .collect();
        exports.sort_unstable();
        println!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}",
            if result.is_ok() { "ok" } else { "panic" },
            accesses,
            ranges,
            lifetimes,
            words(&info.image_usage_flags),
            words(&info.buffer_usage_flags),
            words(&info.lifetime_events),
            words(&exports),
            words(
                &calls
                    .iter()
                    .map(|&value| u32::from(value))
                    .collect::<Vec<_>>()
            )
        );
    }
}
