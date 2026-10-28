//! add_autoclutter_on_windows（B 主基线；0x1422cd7d0，size 1622）。
//!
//! 证据（2026-10-17 re-analysis wave2，逐语句判等基线）：
//! - `evidence/r2-1422cd7d0-add_autoclutter_on_windows.disasm.txt`（radare2 RNE oracle
//!   全函数 430 指令反汇编）+ 本批次 raw bytes（0x1422cd8b0..0x1422cd8d0 hexdump）+
//!   跳表 0x143042370 解码（interaction/ecstate 两份 RE-ANALYSIS 交叉一致）。
//! - 输入候选 Vec（stride 0x50，`[r9]` 段 {data@+8, len@+0x10}，入口 `mov [r9+0x10],0`
//!   清零 len=drain）；每候选：
//!   +0x00 kind byte（==6 直接跳过，`CMP ECX,6; JZ`）、+0x01 flag byte、+0x04 dword、
//!   +0x08 host_index（仅 kind 1 用）、+0x10 16B 位置（XMM8）、+0x20 qword、+0x28 dword、
//!   +0x40 qword、+0x48 qword。
//! - 跳表 0x143042370（6 项 disp32，`target = table + signed32`）按 kind 分派，6 分支共享
//!   同一向量数学（仅宿主源与标志不同）：
//!   | kind | 宿主源（rax） | 标志 |
//!   |---|---|---|
//!   | 0 | base+0xf0 | r12b=2, r14d=0 |
//!   | 1 | base Vec（data@+8,len@+0x10，以候选+0x08 为下标；`jae → panic_bounds_check`） | r12b=2, r14d=0 |
//!   | 2 | base+0xf8 | `NOT r12b; AND r12b,1`（翻转候选+0x01）、r14d=0 |
//!   | 3 | base+0x1a8 | r12b=2, r14d=0 |
//!   | 4 | base+0x1b0 | r12b=2, r14d=0 |
//!   | 5 | base+0x1b8 | r12b=2, r14d=0 |
//! - 向量数学（case 0 @0x1422cd9b7 逐条）：常量 `[0x1429cd0c0]=q=(0,0,1,0)`、
//!   `[0x142925904]=1.0`；`reflect = 2(p·q)p + (w²-|p|²)q`（w=16B 第 4 分量=0）
//!   → `(2·p.x·p.z, 2·p.y·p.z, 2·p.z·p.z-|p|²)`。
//! - 写块 0x1422cd8c0（输出候选 Vec r15+0x48/0x50/0x58，stride 0x68；满则
//!   `call fcn.14078c880` grow + panic 串 "crates/systems/clutter/src/autoclutter/
//!   add_autoclutter_on_windo…" @0x1430422f8）：
//!   | +0x00 | 5（type，与 lantern/tree 同） |
//!   | +0x08 | 16B 位置（`movups [rbp-0x60]`——**原二进制从未写该栈槽**，raw bytes 证实
//!     0x1422cd8ba..0x1422cd8c0 为 6 字节 NOP 对齐垫，非隐藏 store；未初始化读=原 B 实况） |
//!   | +0x18 | 1（dword） |
//!   | +0x20 | 候选+0x48 |
//!   | +0x30 | r12b（2 或 !flag&1） |
//!   | +0x31 | u16 `[rbp+0x55]`（原从未写，未初始化） |
//!   | +0x33 | byte `[rbp+0x57]`（原从未写，未初始化） |
//!   | +0x34 | r14d（0） |
//!   | +0x38 | r13（宿主子结构首 qword） |
//!   | +0x40 | 候选+0x40 |
//!   | +0x48 | 候选+0x20 |
//!   | +0x50 | 候选+0x28（dword） |
//!   | +0x54 | reflect.x（movlps XMM8 低 qword=x,z） |
//!   | +0x58 | reflect.z |
//!   | +0x5c | reflect.y（unpckhpd XMM8 高→y） |
//!   | +0x60 | 1.0（0x3f800000） |
//! - 循环尾 `R9+=0x50; RSI+=0x50; cmp RSI,end; je epilogue`。
//!
//! U 登记（不伪造）：+0x08/+0x31/+0x33 三个输出字段原二进制为**未初始化栈读**（编译器
//! 死槽/游戏对这三个字段从不读取）；本重建以语义值填充并登记，不仿造未初始化读写。

use bevy_ecs::prelude::*;
use glam::Vec3;

use super::{AutoClutterSpawn, AutoClutterState, SpawnShared};

/// on_windows 输入候选（stride 0x50；radare2 输入读取块 0x1422cd967..0x1422cd9b5）。
#[derive(Clone, Debug)]
pub struct OnWindowCandidate {
    /// +0x00：种类字节（跳表索引 0–5；==6 直接跳过）。
    pub kind: u8,
    /// +0x01：标志字节（kind 2 分支 `NOT r12b; AND r12b,1` 翻转）。
    pub flag: u8,
    /// +0x04：dword（→ r14d；kind 2 分支先读取、写输出 +0x34 前被 `xor r14d,r14d` 清零）。
    pub f04: u32,
    /// +0x08：宿主 Vec 下标（仅 kind 1 分支用；`jae → panic_bounds_check`）。
    pub host_index: u64,
    /// +0x10：16B 候选位置（XMM8；第 4 分量 w 参与反射公式但原 w=0）。
    pub position: Vec3,
    /// +0x20：qword（→ 输出 +0x48）。
    pub f20: u64,
    /// +0x28：dword（→ 输出 +0x50）。
    pub f28: u64,
    /// +0x40：qword（→ 输出 +0x40）。
    pub f40: u64,
    /// +0x48：qword（→ 输出 +0x20）。
    pub f48: u64,
}

/// on_windows 宿主（`*in_R8`；各窗数据子结构首 qword 与 kind 1 的 Vec）。
/// 装配层按 B 宿主资源填充（base+0xf0..0x1b8 为各窗类型/百叶窗/开口子结构段）。
#[derive(Clone, Debug, Default)]
pub struct OnWindowHost {
    /// base+0xf0（kind 0 宿主源）首 qword。
    pub window_data_a: u64,
    /// base+0xf8（kind 2 宿主源）首 qword。
    pub window_data_b: u64,
    /// base+0x1a8（kind 3 宿主源）首 qword。
    pub window_data_c: u64,
    /// base+0x1b0（kind 4 宿主源）首 qword。
    pub window_data_d: u64,
    /// base+0x1b8（kind 5 宿主源）首 qword。
    pub window_data_e: u64,
    /// base+0x08/+0x10 Vec（kind 1 宿主源；以候选+0x08 为下标，越界 panic）。
    pub host_vec: Vec<u64>,
}

/// on_windows 输入（资源；宿主装配）。
#[derive(Resource, Clone, Debug, Default)]
pub struct OnWindowInput {
    pub candidates: Vec<OnWindowCandidate>,
    pub host: OnWindowHost,
}

/// add_autoclutter_on_windows（0x1422cd7d0）：drain 候选 Vec，按候选 kind（0–5，6 跳过）
/// 经跳表选窗宿主子结构，共享同一反射公式（`reflect = 2(p·q)p + (w²-|p|²)q`，
/// q=(0,0,1,0), w=0）计算落点，直接写 type=5 输出候选（stride 0x68）到
/// AutoClutterState 输出缓冲。
pub fn add_autoclutter_on_windows(mut state: ResMut<AutoClutterState>, input: Res<OnWindowInput>) {
    for candidate in &input.candidates {
        let kind = candidate.kind;
        if kind == 6 {
            continue; // O：`CMP ECX,6; JZ` 直接跳过。
        }
        // 跳表分派：6 分支仅宿主源与标志不同（O：0x143042370 解码 + case 体反汇编）。
        let host_first_qword = match kind {
            0 => input.host.window_data_a,
            1 => {
                // O：`cmp rax,[rcx+0x10]; jae → panic_bounds_check`（0x1422cdbe0/0x1422cdd58）。
                let idx = candidate.host_index as usize;
                assert!(
                    idx < input.host.host_vec.len(),
                    "add_autoclutter_on_windows: host Vec index out of bounds"
                );
                input.host.host_vec[idx]
            }
            2 => input.host.window_data_b,
            3 => input.host.window_data_c,
            4 => input.host.window_data_d,
            5 => input.host.window_data_e,
            // kind>6 在原二进制会读跳表外（rustc 串区）——未定义；本重建按 kind 0–5 合法
            // 域跳过，登记 U（原产物只产生 0–5）。
            _ => continue,
        };
        // 共享向量数学（O：case 体逐条）：reflect = 2(p·q)p + (w²-s)q，q=(0,0,1,0), w=0。
        let p = candidate.position;
        let s = p.x * p.x + p.y * p.y + p.z * p.z;
        let reflect = Vec3::new(2.0 * p.x * p.z, 2.0 * p.y * p.z, 2.0 * p.z * p.z - s);
        // 标志（O：kind 2 `NOT r12b; AND r12b,1`；其余 `mov r12b,2`）。
        let marker = if kind == 2 { !candidate.flag & 1 } else { 2 };
        // 写块 0x1422cd8c0（输出 stride 0x68）。
        state.spawn_candidates.push(AutoClutterSpawn {
            kind: 5,
            // +0x08：原二进制为未初始化栈读（raw bytes 证实无 store）——U，本重建以
            // prefab 名承载（lantern/tree 同槽位语义）。
            shared: SpawnShared::Prefab("window".to_string()),
            f18: 1,             // +0x18 dword=1。
            key: candidate.f48, // +0x20 = 候选+0x48。
            marker,
            // +0x54..+0x60 = (x,z,y) swizzle 的反射结果（movlps 低 qword=x,z + unpckhpd=y）；
            // 本重建写入 position 语义字段（xzy swizzle），供 manage 消费。
            position: Vec3::new(reflect.x, reflect.z, reflect.y),
            host_id: host_first_qword, // +0x38 = 宿主子结构首 qword（墙/窗 id）。
            f48: candidate.f20,        // +0x48 = 候选+0x20。
            f50: candidate.f28,        // +0x50 = 候选+0x28。
            f54: reflect.x.to_bits(),
            f58: reflect.z.to_bits(),
            f5c: reflect.y.to_bits(),
            scale: 1.0, // +0x60 = 0x3f800000。
        });
    }
}
