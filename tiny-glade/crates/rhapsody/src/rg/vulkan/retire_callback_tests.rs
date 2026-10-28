use super::*;
use crate::rg::graph;
use std::sync::{Arc, Mutex};

#[test]
fn a_panicking_retire_callback_does_not_discard_later_cleanup() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let callbacks: Vec<graph::RetireCallback> = (0..3)
        .map(|index| {
            let calls = calls.clone();
            Box::new(move || {
                calls.lock().unwrap().push(index);
                if index == 1 {
                    panic!("injected cleanup failure");
                }
            }) as graph::RetireCallback
        })
        .collect();
    assert_eq!(
        run_retire_callbacks(callbacks).unwrap_err().kind(),
        crate::ErrorKind::InvalidUsage
    );
    assert_eq!(*calls.lock().unwrap(), [0, 1, 2]);
}
