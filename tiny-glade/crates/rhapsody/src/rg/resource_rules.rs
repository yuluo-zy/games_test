// 候选实现迁移来源：reconstruction/render/resource_info.rs.
// 这是行为重建，不是原作者源码。
//! 由原机器码恢复的游戏私有资源图规则。类型与映射证据见原 render-skeleton；Vec、排序和访问分类依赖直接复用。
//!
//! 原模块为 rhapsody::rg::{resource_info,recorder}；映射保存在原工程 render-skeleton 证据中。
//! 本文件是行为重建。向量、排序与依赖访问分类直接调用原依赖接口，不重新还原通用库。

pub const NO_ACCESS: u32 = u32::MAX;

/// ResourceInfo::calculate (0x140530430) 写入且 link_up_next_access_chains (0x140bd47d0) 消费的 20 字节记录，不代表完整 ResourceInfo ABI。
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResourceAccessRecord {
    pub resource_index: u32,
    pub previous_access_index: u32,
    pub next_access_index: u32,
    pub synchronization_group: u32,
    pub access_code: u8,
    /// 原函数未读取这些字节，保留其原值。
    pub unobserved_padding: [u8; 3],
}

/// 原 first/last pass 端点的数字投影；生命周期生成以整个标签字是否等于 1 判断。
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PassEndpoint {
    pub tag: u32,
    pub pass_index: u32,
}

/// 已观察到的每资源 24 字节记录，来源为 ResourceInfo +0x38 / +0x40 向量。
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResourceLifetimeRecord {
    pub first_pass: PassEndpoint,
    pub last_pass: PassEndpoint,
    pub first_access_index: u32,
    pub last_access_index: u32,
}

/// 原 0x140bd47d0 将前访问链接至当前访问，不清空无后继 next；重复前驱按遍历顺序覆盖，越界保留此前写入。
pub fn link_up_next_access_chains(accesses: &mut [ResourceAccessRecord]) {
    for current in 0..accesses.len() {
        let previous = accesses[current].previous_access_index;
        if previous != NO_ACCESS {
            let current = u32::try_from(current).expect("resource access index exceeds u32");
            accesses[previous as usize].next_access_index = current;
        }
    }
}

/// 原 0x140bd4860 追加并排序生命周期事件。每资源两个事件，第二个置 bit16，pass 左移17，资源索引直接 OR；不清空向量或额外验证打包域。
pub fn calculate_lifetime_events(resources: &[ResourceLifetimeRecord], events: &mut Vec<u32>) {
    for (resource_index, resource) in resources.iter().enumerate() {
        if resource.first_pass.tag == 1 && resource.last_pass.tag == 1 {
            let resource_index = resource_index as u32;
            events.push(resource.first_pass.pass_index.wrapping_shl(17) | resource_index);
            events.push(resource.last_pass.pass_index.wrapping_shl(17) | resource_index | 0x1_0000);
        }
    }
    // 输出排序由游戏消费；排序直接调用标准库，不重新还原通用排序实现。
    events.sort_unstable();
}
