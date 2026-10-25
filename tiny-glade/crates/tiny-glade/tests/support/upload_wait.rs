//! Test harness waiting; production owners yield Pending instead.
pub fn until_ready<E>(mut poll: impl FnMut() -> Result<bool, E>) -> Result<(), E> {
    let started = std::time::Instant::now();
    while !poll()? {
        assert!(
            started.elapsed() < std::time::Duration::from_secs(10),
            "test upload admission timed out"
        );
        std::thread::sleep(std::time::Duration::from_micros(100));
    }
    Ok(())
}
