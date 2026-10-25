use std::io::{self, BufRead};
use tiny_glade_sequence_runtime_r8::*;

fn failure_name(f: Failure) -> String {
    format!("{f:?}")
}
fn numbers(values: impl IntoIterator<Item = u64>) -> String {
    values
        .into_iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(",")
}
fn scenario(line: &str) -> String {
    let parts = line.split_whitespace().collect::<Vec<_>>();
    let count: usize = parts[0].parse().unwrap();
    let order = parts[1]
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let mode: u32 = parts[2].parse().unwrap();
    let fail: u32 = parts[3].parse().unwrap();
    let stage: u32 = parts[4].parse().unwrap();
    let late: u32 = parts[5].parse().unwrap();
    let resources: usize = parts[6].parse().unwrap();
    let counters = DeviceCounters::new(1);
    let mut runtime = SequenceRuntime::new(1);
    let mut commands = Vec::new();
    let mut failure = "ok".to_string();
    let mut failed_command = "none".to_string();
    let queue = if mode == 1 {
        Queue::Auxiliary(0)
    } else {
        Queue::Primary
    };
    for i in 0..count {
        let q = if mode == 2 && i == 1 {
            Queue::Auxiliary(0)
        } else if fail == 4 {
            Queue::Auxiliary(99)
        } else {
            queue
        };
        let mut command = Command::new(100 + i as u64, 0);
        if fail == 3 {
            command.set_state(2);
        }
        let backend = match fail {
            1 => Err(Failure::BackendReset),
            2 => Err(Failure::BackendBegin),
            _ => Ok(()),
        };
        match runtime.begin_after_backend(&counters, command, q, backend) {
            Ok(c) => commands.push(c),
            Err(e) => {
                failed_command = format!("{}:{}", e.command.state(), e.command.sequence().0);
                failure = failure_name(e.reason);
                break;
            }
        }
    }
    let mut handles = Vec::new();
    if failure == "ok" {
        for id in 0..resources {
            let h = runtime.create_resource(id as u64, 7);
            handles.push(h);
            for c in &commands {
                runtime.track(h, c).unwrap();
            }
            runtime.cpu_retire(h);
            if late == 0 {
                runtime.drop_handle(h).unwrap();
            }
        }
        if late == 0 {
            runtime.collect_waste();
        }
        let pending = order
            .iter()
            .enumerate()
            .map(|(i, &index)| {
                let mut c = commands[index - 1].clone();
                c.set_state(if fail == 5 { 2 } else { 3 });
                if fail == 6 {
                    let tag = 1 - u32::from(mode == 1);
                    c.bytes[16..20].copy_from_slice(&tag.to_le_bytes());
                }
                if fail == 9 {
                    c.bytes[0x50..0x54].copy_from_slice(&99u32.to_le_bytes());
                }
                PendingCommand {
                    command: c,
                    threshold: Timeline(41 + i as u64),
                }
            })
            .collect::<Vec<_>>();
        runtime.accept_submission(
            SubmissionReceipt {
                queue,
                last_timeline: Timeline(40 + order.len() as u64),
            },
            pending,
        );
        let values = if stage == 0 {
            (0..order.len())
                .flat_map(|i| [40 + i as u64, 41 + i as u64])
                .collect::<Vec<_>>()
        } else {
            vec![40 + order.len() as u64]
        };
        for value in values {
            runtime.observe_timeline(queue, Timeline(value));
            if let Err(e) = runtime.reclaim_ready(queue) {
                failure = failure_name(e);
                break;
            }
        }
        if late == 1 && failure == "ok" {
            for h in &handles {
                runtime.drop_handle(*h).unwrap();
            }
            runtime.collect_waste();
        }
    }
    let next = [
        counters
            .next_value(device_counters_r6::QueueIdentity::Primary)
            .unwrap(),
        counters
            .next_value(device_counters_r6::QueueIdentity::Auxiliary(0))
            .unwrap(),
    ];
    let queues = [Queue::Primary, Queue::Auxiliary(0)];
    let states = queues
        .map(|q| {
            format!(
                "{}:{}:{}",
                runtime.prefix(q),
                numbers(runtime.ready_markers(q)),
                runtime.pending_len(q)
            )
        })
        .join("/");
    let resource_states = handles
        .iter()
        .map(|h| {
            let (s, r) = runtime.resource_state(*h).unwrap();
            format!("{s}:{r}")
        })
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{}|{}|{}|{}|{}|{}|{}|{}",
        failure,
        runtime.events.join(";"),
        numbers(next),
        states,
        numbers(runtime.bucket_keys()),
        numbers(runtime.pool_sequences()),
        resource_states,
        failed_command
    )
}
fn main() {
    for line in io::stdin().lock().lines() {
        println!("{}", scenario(&line.unwrap()));
    }
}
