//! 顶层日志环：`init_logging`（0x140be6a90）/`dump_log`（0x140be6b50）/`print_log_dump`（ICF thunk 0x140be6b40）。
//!
//! 机器码证实的布局（全局 0x143634e40，24B）：`{ data: *mut u8 @0, cap: usize @8, cursor: AtomicU64 @0x10 }`，
//! 即 `leekspin::RingBuffer`（crash.rs 文件头已提到）：
//! - 构造 `FUN_1418c1310(&mut ring, 0x10000)`：cap 为负 panic；cap≠0 时分配 cap 字节（对齐 1），cursor=0。
//! - 写入 `FUN_1418c1380(&ring, data, len)`：`pos = cursor.fetch_add(len)`（`lock xadd`）；cap==0 时除零 panic；
//!   `pos %= cap`；`pos+len <= cap` 单次拷贝，否则 `len > cap` 断言 `part1 <= self.data.len()`，
//!   再分两段（先拷到尾，剩余从头）。**无锁**，多线程写入靠 fetch_add 保留区间。
//! - 转储 `FUN_1418c1430(&mut Vec, &ring)`：拷出 cap 字节；`cursor <= cap` 时结果只含前 `cursor` 字节，
//!   否则对整份拷贝做 `rotate_left(cursor % cap)`（最旧在前）。拷贝时不加锁（与写入并发会读到撕裂数据）。
//! - `init_logging`：先把新环写进全局，再断言旧值为空（`assertion failed: prev.is_none()`），
//!   所以重复初始化会 panic，但全局已被新环替换；`dump_log` 全局为空时 panic。
//!
//! 谁写这个环（机器码证实）：`dispose_cb_resource<T>` 写 kind 3/4 记录，`SequentialDisposal::dispose_in_order`
//! 写 kind 0x1e 记录（见 `raw/disposal.rs`）；更多调用点（FUN_1418c1380 的 xref，约 50+ 处，分布在 raw-core/rg/managed）
//! 未逐个枚举。记录布局：`[16B 名字（ASCII 过滤，0 填充）][8B 载荷，可选][1B kind]`；`dispose_in_order` 的记录
//! 无名字，只有 `[u64][kind=0x1e]`（9B）。kind 3 = `[name16][u64 seq][3]`（25B），kind 4 = `[name16][4]`（17B）。
//! 名字参数的语义（调用方给的 &str）未证实。
use parking_lot::RwLock;
use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::{AtomicU8, AtomicU64, Ordering};

pub const LOG_RING_BYTES: usize = 0x10000;

pub struct RingBuffer {
    data: Box<[AtomicU8]>,
    cursor: AtomicU64,
}
impl RingBuffer {
    pub fn new(cap: usize) -> Self {
        Self {
            data: (0..cap).map(|_| AtomicU8::new(0)).collect(),
            cursor: AtomicU64::new(0),
        }
    }
    pub fn capacity(&self) -> usize {
        self.data.len()
    }
    pub fn cursor(&self) -> u64 {
        self.cursor.load(Ordering::Relaxed)
    }
    /// 原 FUN_1418c1380。cap==0 为除零 panic，`len > cap` 为 `part1 <= self.data.len()` 断言。
    pub fn write(&self, bytes: &[u8]) {
        let len = bytes.len() as u64;
        let pos = self.cursor.fetch_add(len, Ordering::Relaxed);
        let cap = self.data.len();
        assert!(cap != 0, "attempt to calculate the remainder with a divisor of zero");
        let pos = (pos % cap as u64) as usize;
        assert!(bytes.len() <= cap, "assertion failed: part1 <= self.data.len()");
        for (index, byte) in bytes.iter().enumerate() {
            self.data[(pos + index) % cap].store(*byte, Ordering::Relaxed);
        }
    }
    /// 原 FUN_1418c1430。
    pub fn dump(&self) -> Vec<u8> {
        let mut copy: Vec<u8> = self.data.iter().map(|byte| byte.load(Ordering::Relaxed)).collect();
        let cursor = self.cursor.load(Ordering::Relaxed);
        let cap = copy.len() as u64;
        if cursor <= cap {
            copy.truncate(cursor as usize);
        } else {
            copy.rotate_left((cursor % cap) as usize);
        }
        copy
    }
}

static RING: RwLock<Option<Arc<RingBuffer>>> = RwLock::new(None);

/// 建 0x10000 字节环；先替换全局再断言旧值为空（原顺序）。
pub fn init_logging() {
    let previous = RING.write().replace(Arc::new(RingBuffer::new(LOG_RING_BYTES)));
    assert!(previous.is_none(), "assertion failed: prev.is_none()");
}

/// 严格写入口（全局为空时 panic，对应原版除零）。
pub fn write(bytes: &[u8]) {
    let ring = RING.read().clone().expect("rhapsody log not initialised");
    ring.write(bytes);
}

/// crate 内部使用的宽松写入口：未初始化时丢弃（原版此时会 panic，见 [`write`]）。
pub fn record(bytes: &[u8]) {
    let ring = RING.read().clone();
    if let Some(ring) = ring {
        ring.write(bytes);
    }
}

/// `[u64 载荷][kind]`（9B），如 `dispose_in_order` 的 kind 0x1e 记录。
pub fn record_event(kind: u8, payload: u64) {
    let mut bytes = [0u8; 9];
    bytes[..8].copy_from_slice(&payload.to_le_bytes());
    bytes[8] = kind;
    record(&bytes);
}

/// `[name16][u64 载荷]?[kind]`，名字取前 16 个 ASCII 字符，0 填充（原 dispose_cb_resource 的记录形态）。
pub fn record_named(name: &str, payload: Option<u64>, kind: u8) {
    let mut bytes = Vec::with_capacity(25);
    bytes.extend_from_slice(&crate::leek::Name::from(name).0);
    if let Some(payload) = payload {
        bytes.extend_from_slice(&payload.to_le_bytes());
    }
    bytes.push(kind);
    record(&bytes);
}

/// 原 `dump_log`：返回按时间顺序（最旧在前）的字节；未初始化 panic。
pub fn dump_log_vec() -> Vec<u8> {
    RING.read().clone().expect("rhapsody log not initialised").dump()
}

pub fn dump_log(out: &mut impl Write) -> std::io::Result<()> {
    out.write_all(&dump_log_vec())
}

pub fn print_log_dump() -> std::io::Result<()> {
    dump_log(&mut std::io::stderr().lock())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ring_semantics_match_original_layout() {
        let ring = RingBuffer::new(8);
        ring.write(b"abc");
        assert_eq!(ring.dump(), b"abc"); // cursor <= cap：前 cursor 字节
        ring.write(b"defgh");
        assert_eq!(ring.dump(), b"abcdefgh"); // cursor == cap，不旋转
        ring.write(b"XY");
        assert_eq!(ring.cursor(), 10);
        assert_eq!(ring.dump(), b"cdefghXY"); // rotate_left(10 % 8)，最旧在前
        assert!(std::panic::catch_unwind(|| ring.write(&[0; 9])).is_err()); // len > cap
        assert!(std::panic::catch_unwind(|| RingBuffer::new(0).write(b"x")).is_err()); // 除零
    }

    // 全局单例：只在一个测试里走完整生命周期。
    #[test]
    fn global_ring_init_once_and_event_layout() {
        record(b"dropped before init");
        assert!(std::panic::catch_unwind(dump_log_vec).is_err());
        init_logging();
        record_event(0x1e, 7);
        record_named("ab", Some(9), 3);
        let out = dump_log_vec();
        assert_eq!(out.len(), 9 + 25);
        assert_eq!(&out[..9], &[7, 0, 0, 0, 0, 0, 0, 0, 0x1e]);
        assert_eq!(&out[9..11], b"ab");
        assert_eq!(out[33], 3);
        // 重复初始化 panic，但全局已被新环替换（原顺序：先写后断言）。
        assert!(std::panic::catch_unwind(init_logging).is_err());
        assert!(dump_log_vec().is_empty());
    }
}
