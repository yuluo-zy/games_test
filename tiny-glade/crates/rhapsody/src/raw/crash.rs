//! GPU 崩溃诊断：debug 缓冲解析、断言检测与错误信息需求判定。
//! （原先的宿主侧 `CrashDiagnostics` 标记环在整条管线里无调用点，且不等价于原作的 `leekspin::RingBuffer`，已删除。）
use parking_lot::Mutex;
use serde::Serialize;

use super::{
    buffer::{Buffer, BufferDesc},
    command_buffer::VulkanCommandBuffer,
    device::Device,
    error::{Error, Result},
};
use ash::vk;
use std::{collections::BTreeMap, sync::Arc};

/// 原 debug 记录固定 4 个 u32：类型/源码行号、源码 ID、两个原始操作数。
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct GpuDebugRecord {
    pub kind: u8,
    pub source_line: u32,
    pub source_id: u32,
    pub operands: [u32; 2],
    pub source_name: Option<String>,
    pub message: String,
}
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct GpuDebugLog {
    /// 头部第 1 个 word 是计数；其他三个 word 的私有语义未恢复。
    pub header: [u32; 4],
    pub records: Vec<GpuDebugRecord>,
    pub truncated: bool,
}
/// 原 has_any 入口：无 buffer 为 false，存在时检查 word[1] 是否非零。
/// 原短 buffer 会 panic；公开候选返回错误以便故障报告仍可包含其他上下文。
pub fn has_any_gpu_assertion_failed(words: Option<&[u32]>) -> Result<bool> {
    match words {
        None => Ok(false),
        Some(words) => Ok(*words
            .get(1)
            .ok_or(Error::invalid_usage("GPU 调试头不足两个 word"))?
            != 0),
    }
}
pub fn parse_gpu_debug_info(
    words: &[u32],
    source_names: &BTreeMap<u32, String>,
) -> Result<GpuDebugLog> {
    let header: [u32; 4] = words
        .get(..4)
        .ok_or(Error::invalid_usage("GPU 调试头不足四个 word"))?
        .try_into()
        .unwrap();
    let available = (words.len() - 4) / 4;
    let count = (header[1] as usize).min(available);
    let records = words[4..4 + count * 4]
        .as_chunks::<4>()
        .0
        .iter()
        .map(|words| {
            let kind = (words[0] >> 24) as u8;
            let left = words[2];
            let right = words[3];
            let message = match kind {
                1 => format!("abort ({left}, {right})"),
                2 => "assertion failed".to_owned(),
                3 => format!("assertion failed ({left} > {right})"),
                4 => format!("assertion failed ({left} < {right})"),
                5 => format!("assertion failed ({left} >= {right})"),
                6 => format!("assertion failed ({left} <= {right})"),
                _ => "???".to_owned(),
            };
            GpuDebugRecord {
                kind,
                source_line: words[0] & 0x00ff_ffff,
                source_id: words[1],
                operands: [left, right],
                source_name: source_names.get(&words[1]).cloned(),
                message,
            }
        })
        .collect();
    Ok(GpuDebugLog {
        header,
        records,
        truncated: header[1] as usize > available,
    })
}

/// 原 `log_gpu_debug_info`（0x1418c46c0）里源名文件：`tmp/shader-cache/0x{src_id:x}.srcname`
/// （片段 0x142d06d10 + `{:x}` + `.srcname`@0x142d06d23），读失败时显示 `???`。
pub fn srcname_path(source_id: u32) -> String {
    format!("tmp/shader-cache/0x{source_id:x}.srcname")
}
/// 原逐记录日志行（pieces @0x142d06d60：`"🔥 "`,`": "`,`" in "`,`":"`；参数序 idx/消息/源名/行号）。
/// 原版以 info 级别输出（level 字段 3，门控 `max_level > 2`）。
pub fn format_debug_record_line(index: usize, record: &GpuDebugRecord) -> String {
    format!(
        "\u{1f525} {index}: {} in {}:{}",
        record.message,
        record.source_name.as_deref().unwrap_or("???"),
        record.source_line
    )
}
/// 逐记录输出（info 级别）。
pub fn log_debug_records(log: &GpuDebugLog) {
    for (index, record) in log.records.iter().enumerate() {
        log::info!("{}", format_debug_record_line(index, record));
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct GpuMarkerLabel {
    pub id: u32,
    pub slot: u32,
    pub label: String,
}
struct MarkerHistory {
    next: u32,
    labels: BTreeMap<u32, GpuMarkerLabel>,
}
#[derive(Clone, Debug, Serialize)]
pub struct GpuCrashSnapshot {
    /// 这是实际 buffer 读回值；宿主记录 marker 不会修改该值。
    pub observed_marker: u32,
    pub marker_label: Option<GpuMarkerLabel>,
    pub initial_sentinel_observed: bool,
    pub assertions_failed: bool,
    pub debug: GpuDebugLog,
}

/// GPU 调试源码名映射；`insert_name` 分配稳定 id，`get_name` 回查。
/// 与 `GpuCrashDiagnostics` 内部的 MarkerHistory 独立；只保留名称↔id 表。
#[derive(Clone, Debug, Default)]
pub struct CrashMarkerNames {
    next: u32,
    names: BTreeMap<u32, String>,
    reverse: std::collections::HashMap<String, u32>,
}
impl CrashMarkerNames {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn insert_name(&mut self, name: impl Into<String>) -> u32 {
        let name = name.into();
        if let Some(id) = self.reverse.get(&name) {
            return *id;
        }
        let id = self.next;
        self.next += 1;
        self.names.insert(id, name.clone());
        self.reverse.insert(name, id);
        id
    }
    pub fn get_name(&self, id: u32) -> Option<&str> {
        self.names.get(&id).map(String::as_str)
    }
    pub fn len(&self) -> usize {
        self.names.len()
    }
    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }
}

/// 原入口名；解析并输出本帧 GPU 调试日志（与 `log_gpu_crash_info` 的 debug 段相同来源）。
impl GpuCrashDiagnostics {
    pub fn log_gpu_debug_info(
        &self,
        source_names: &BTreeMap<u32, String>,
    ) -> Result<serde_json::Value> {
        self.device.wait_idle()?;
        let mut bytes = vec![0; self.debug_words * 4];
        self.debug_buffer.read_bytes(0, &mut bytes)?;
        let words: Vec<_> = bytes
            .as_chunks::<4>()
            .0
            .iter()
            .map(|bytes| u32::from_le_bytes(*bytes))
            .collect();
        let debug = parse_gpu_debug_info(&words, source_names)?;
        log_debug_records(&debug);
        Ok(serde_json::to_value(debug).unwrap_or(serde_json::Value::Null))
    }
}

/// GPU 诊断生产链；core fill marker 与 storage debug buffer，不假装支持厂商崩溃转储。
pub struct GpuCrashDiagnostics {
    pub marker_buffer: Arc<Buffer>,
    pub debug_buffer: Arc<Buffer>,
    device: Arc<Device>,
    history: Mutex<MarkerHistory>,
    debug_words: usize,
}
impl GpuCrashDiagnostics {
    pub fn new(device: Arc<Device>, record_capacity: u32) -> Result<Self> {
        let debug_words = (record_capacity as usize)
            .checked_mul(4)
            .and_then(|n| n.checked_add(4))
            .ok_or(Error::invalid_usage("GPU 调试容量溢出"))?;
        let marker_buffer = Arc::new(Buffer::new(
            device.clone(),
            &BufferDesc::new_gpu_to_cpu(4, vk::BufferUsageFlags::TRANSFER_DST),
        )?);
        let debug_buffer = Arc::new(Buffer::new(
            device.clone(),
            &BufferDesc::new_gpu_to_cpu(
                debug_words as u64 * 4,
                vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::TRANSFER_DST,
            ),
        )?);
        marker_buffer.write_bytes(0, &u32::MAX.to_le_bytes())?;
        debug_buffer.write_bytes(0, &vec![0; debug_words * 4])?;
        Ok(Self {
            marker_buffer,
            debug_buffer,
            device,
            history: Mutex::new(MarkerHistory {
                next: 0,
                labels: BTreeMap::new(),
            }),
            debug_words,
        })
    }
    /// 命令登记与实际 GPU 写入同时生产；原计数为 u32，历史槽为 id & 4095。
    pub fn record_crash_marker(
        &self,
        command: &mut VulkanCommandBuffer,
        label: impl Into<String>,
    ) -> Result<u32> {
        if !Arc::ptr_eq(&command.pool.device, &self.device)
            || command.pool.family != self.device.queue_family
        {
            return Err(Error::invalid_usage("诊断命令设备或队列族不匹配"));
        }
        if !command.is_recording() {
            return Err(Error::invalid_usage("未处于录制状态的命令不能写入 marker"));
        }
        let mut history = self.history.lock();
        let id = history.next;
        history.next = id.wrapping_add(1);
        history.labels.insert(
            id & 4095,
            GpuMarkerLabel {
                id,
                slot: id & 4095,
                label: label.into(),
            },
        );
        command.retain_opaque(self.marker_buffer.clone())?;
        unsafe {
            self.device
                .device
                .cmd_fill_buffer(command.handle, self.marker_buffer.handle, 0, 4, id);
        }
        Ok(id)
    }
    /// 让调试 buffer 存活至命令释放；shader 使用该 buffer 产生实际诊断数据。
    pub fn retain_debug_buffer(&self, command: &mut VulkanCommandBuffer) -> Result<()> {
        command.retain_opaque(self.debug_buffer.clone())
    }
    /// 调用者的 shader/transfer 写入后，建立 host 可读的 Vulkan 内存依赖。
    pub fn make_host_readable(&self, command: &mut VulkanCommandBuffer) -> Result<()> {
        if !Arc::ptr_eq(&command.pool.device, &self.device)
            || !command.is_recording()
            || command.pool.family != self.device.queue_family
        {
            return Err(Error::invalid_usage("诊断屏障的命令设备或录制状态不匹配"));
        }
        command.retain_opaque(self.marker_buffer.clone())?;
        command.retain_opaque(self.debug_buffer.clone())?;
        let mut scope = super::barrier::SynchronizationScope::new(
            vk::PipelineStageFlags::ALL_COMMANDS,
            vk::PipelineStageFlags::HOST,
        );
        for buffer in [&self.marker_buffer, &self.debug_buffer] {
            scope.push(
                super::barrier::BarrierTarget::Buffer(buffer.handle),
                &super::barrier::RangeBarrier::local(
                    crate::resource::OwnershipRange::Buffer {
                        offset: 0,
                        size: vk::WHOLE_SIZE,
                    },
                    vk::AccessFlags::MEMORY_WRITE,
                    vk::AccessFlags::HOST_READ,
                    vk::ImageLayout::UNDEFINED,
                    vk::ImageLayout::UNDEFINED,
                ),
            )?;
        }
        scope.issue_into(command)?;
        Ok(())
    }
    /// 完成边界之后才读取；recorded marker、宿主标签及设备错误都不冒充 GPU 已执行。
    pub fn log_gpu_crash_info(
        &self,
        source_names: &BTreeMap<u32, String>,
    ) -> Result<GpuCrashSnapshot> {
        // Buffer 映射入口自身持有队列锁并等待完成，不能在这里重复加非重入锁。
        // 整份 marker/debug 快照的跨 buffer 一致性由诊断 consumer 暂停新帧生产保护。
        self.device.wait_idle()?;
        let mut marker = [0; 4];
        self.marker_buffer.read_bytes(0, &mut marker)?;
        let observed_marker = u32::from_le_bytes(marker);
        let history = self.history.lock();
        let marker_label = history
            .labels
            .get(&(observed_marker & 4095))
            .filter(|label| label.id == observed_marker)
            .cloned();
        let mut bytes = vec![0; self.debug_words * 4];
        self.debug_buffer.read_bytes(0, &mut bytes)?;
        let words: Vec<_> = bytes
            .as_chunks::<4>()
            .0
            .iter()
            .map(|bytes| u32::from_le_bytes(*bytes))
            .collect();
        Ok(GpuCrashSnapshot {
            observed_marker,
            marker_label,
            initial_sentinel_observed: observed_marker == u32::MAX,
            assertions_failed: has_any_gpu_assertion_failed(Some(&words))?,
            debug: parse_gpu_debug_info(&words, source_names)?,
        })
    }
    /// 原入口只为 TIMEOUT(2) 与 DEVICE_LOST(-4) 请求进一步诊断。
    pub fn needs_vulkan_error_info(result: vk::Result) -> bool {
        matches!(result, vk::Result::TIMEOUT | vk::Result::ERROR_DEVICE_LOST)
    }
    pub fn log_vulkan_error_info(
        &self,
        error: vk::Result,
        source_names: &BTreeMap<u32, String>,
    ) -> serde_json::Value {
        let diagnostic = if Self::needs_vulkan_error_info(error) {
            match self.log_gpu_crash_info(source_names) {
                Ok(value) => serde_json::json!({"snapshot":value}),
                Err(cause) => serde_json::json!({"unavailable":cause.to_string()}),
            }
        } else {
            serde_json::Value::Null
        };
        serde_json::json!({"vulkan_result":error.as_raw(),"diagnostic":diagnostic,"vendor_crash_dump":false})
    }
    /// 候选错误包装保留操作与错误原因，读取失败时仍输出可用的宿主上下文。
    /// 原 leek 错误内部 ABI/来源链尚未映射到这个公开类型。
    pub fn log_error_info(
        &self,
        operation: &str,
        error: &Error,
        source_names: &BTreeMap<u32, String>,
    ) -> serde_json::Value {
        let snapshot = match self.log_gpu_crash_info(source_names) {
            Ok(value) => serde_json::json!({"snapshot":value}),
            Err(cause) => serde_json::json!({"unavailable":cause.to_string()}),
        };
        serde_json::json!({"operation":operation,"error":error.to_string(),"gpu":snapshot,"vendor_crash_dump":false})
    }
}

#[cfg(test)]
mod debug_line_tests {
    use super::*;

    #[test]
    fn srcname_path_and_log_line_match_the_original_pieces() {
        assert_eq!(srcname_path(0xabc), "tmp/shader-cache/0xabc.srcname");
        let words = [0, 1, 0, 0, (3 << 24) | 42, 0xabc, 5, 7];
        let names = BTreeMap::from([(0xabc, "foo.slang".to_owned())]);
        let log = parse_gpu_debug_info(&words, &names).unwrap();
        assert_eq!(
            format_debug_record_line(0, &log.records[0]),
            "\u{1f525} 0: assertion failed (5 > 7) in foo.slang:42"
        );
        let unknown =
            parse_gpu_debug_info(&[0, 1, 0, 0, 9 << 24, 1, 0, 0], &BTreeMap::new()).unwrap();
        assert_eq!(
            format_debug_record_line(2, &unknown.records[0]),
            "\u{1f525} 2: ??? in ???:0"
        );
    }
}
