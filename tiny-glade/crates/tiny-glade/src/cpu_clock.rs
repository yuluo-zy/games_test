//! Platform clock reads shared by optional CPU probes and frozen-binary tests.
//! This module is always available, but adds no production hot-path calls.
//! Thread/process CPU values are cumulative nanoseconds; wall ticks use QPC
//! on Windows and monotonic nanoseconds on macOS. Query wall_frequency to
//! interpret wall ticks. The existing unsupported-platform fallbacks remain.

#[cfg(target_os = "macos")]
pub fn read_thread_cpu_ns() -> Option<u64> {
    let mut stamp = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    // SAFETY: `stamp` is a valid, fully-initialized timespec; clock_gettime
    // only writes through the pointer on success.
    if unsafe { libc::clock_gettime(libc::CLOCK_THREAD_CPUTIME_ID, &mut stamp) } != 0 {
        return None;
    }
    Some(stamp.tv_sec as u64 * 1_000_000_000 + stamp.tv_nsec as u64)
}

#[cfg(windows)]
pub fn read_thread_cpu_ns() -> Option<u64> {
    use winapi::{shared::minwindef::FILETIME,
        um::processthreadsapi::{GetCurrentThread, GetThreadTimes}};
    let mut times = [FILETIME { dwLowDateTime: 0, dwHighDateTime: 0 }; 4];
    let [create, exit, kernel, user] = &mut times;
    if unsafe { GetThreadTimes(GetCurrentThread(), create, exit, kernel, user) } == 0 { return None; }
    let raw = |time: FILETIME| (u64::from(time.dwHighDateTime) << 32) | u64::from(time.dwLowDateTime);
    raw(times[2]).checked_add(raw(times[3]))?.checked_mul(100)
}

#[cfg(not(any(windows, target_os = "macos")))]
pub fn read_thread_cpu_ns() -> Option<u64> { None }

pub fn current_thread_id() -> u64 {
    #[cfg(windows)]
    { unsafe { winapi::um::processthreadsapi::GetCurrentThreadId() as u64 } }
    #[cfg(target_os = "macos")]
    {
        let mut id = 0;
        unsafe { libc::pthread_threadid_np(0, &mut id); }
        id
    }
    #[cfg(not(any(windows, target_os = "macos")))]
    { 0 }
}

pub fn wall_ticks() -> u64 {
    #[cfg(windows)]
    {
        let mut value = unsafe { std::mem::zeroed::<winapi::shared::ntdef::LARGE_INTEGER>() };
        if unsafe { winapi::um::profileapi::QueryPerformanceCounter(&mut value) } == 0 { return 0; }
        unsafe { *value.QuadPart() as u64 }
    }
    #[cfg(target_os = "macos")]
    {
        let mut stamp = libc::timespec { tv_sec: 0, tv_nsec: 0 };
        if unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut stamp) } != 0 { return 0; }
        stamp.tv_sec as u64 * 1_000_000_000 + stamp.tv_nsec as u64
    }
    #[cfg(not(any(windows, target_os = "macos")))]
    { 0 }
}

pub fn wall_frequency() -> u64 {
    #[cfg(windows)]
    {
        let mut value = unsafe { std::mem::zeroed::<winapi::shared::ntdef::LARGE_INTEGER>() };
        if unsafe { winapi::um::profileapi::QueryPerformanceFrequency(&mut value) } == 0 { return 0; }
        unsafe { *value.QuadPart() as u64 }
    }
    #[cfg(not(windows))]
    { 1_000_000_000 }
}

pub fn process_cpu_ns() -> Option<u64> {
    #[cfg(windows)]
    {
        use winapi::{shared::minwindef::FILETIME, um::processthreadsapi::{GetCurrentProcess, GetProcessTimes}};
        let mut times = [FILETIME { dwLowDateTime: 0, dwHighDateTime: 0 }; 4];
        let [create, exit, kernel, user] = &mut times;
        if unsafe { GetProcessTimes(GetCurrentProcess(), create, exit, kernel, user) } == 0 { return None; }
        let raw = |time: FILETIME| (u64::from(time.dwHighDateTime) << 32) | u64::from(time.dwLowDateTime);
        raw(times[2]).checked_add(raw(times[3]))?.checked_mul(100)
    }
    #[cfg(not(windows))]
    { None }
}
