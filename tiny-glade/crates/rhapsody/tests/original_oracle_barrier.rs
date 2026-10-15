//! 原版机器码 oracle 重放：barrier 记录（pre / initial / export，1,344 例）。
//! case = [mode, kind, initial, reused, previous(256=无), prev_group, _, current, cur_group, _, flag, validity]。
mod common;
use common::{jstr, replay};
use rhapsody::raw::barrier::{Access, Resource, export, initial, pre};

#[test]
fn barrier_record_matches_original() {
    replay("barrier-record", "parallel-batch-03/barrier-record/native-oracle-observations.json", |input| {
        let v: Vec<u32> = input.as_array().unwrap().iter().map(|x| x.as_u64().unwrap() as u32).collect();
        let resource = Resource { kind: v[1] as u8, initial: v[2] as u8, reused: v[3] != 0, validity: v[11] as u8 };
        let previous = (v[4] != 256).then_some(Access { code: v[4] as u8, group: v[5], write: v[6] != 0 });
        let current = Access { code: v[7] as u8, group: v[8], write: v[9] != 0 };
        let outcome = match v[0] {
            0 => pre(resource, previous, current),
            1 => initial(&mut vec![(resource, (v[4] != 256).then_some(current))], v[10] != 0),
            2 => export(resource, previous, current.code, v[10] != 0),
            m => panic!("mode {m}"),
        };
        let transitions = outcome
            .transitions
            .iter()
            .map(|t| format!("{}:{}:{}", t.kind, t.from, t.to))
            .collect::<Vec<_>>()
            .join(",");
        jstr(format!(
            "{}|{}|{}|{}|{}|{}",
            outcome.panic.unwrap_or("ok"),
            transitions,
            outcome.pending_length,
            outcome.tracked,
            outcome.emitted,
            outcome.resets
        ))
    });
}
