//! 全局 u64 命令桶保存 raw 销毁拥有权。sequence 与队列 timeline 分域。
use crate::raw::managed::tracking::Failure;
use std::collections::BTreeMap;
pub type DestroyAction = Box<dyn FnOnce() + Send>;
#[derive(Default)]
pub struct CommandBufferDisposalBuckets {
    buckets: BTreeMap<u64, Vec<DestroyAction>>,
}
impl CommandBufferDisposalBuckets {
    /// 原 insert 先替换，后断言；覆盖的桶不提前执行 vkDestroy。
    pub fn begin_command_bucket(&mut self, sequence: u64) -> Result<(), Failure> {
        if let Some(previous) = self.buckets.insert(sequence, Vec::new()) {
            for action in previous {
                std::mem::forget(action);
            }
            return Err(Failure::DuplicateBucket);
        }
        Ok(())
    }
    pub fn contains(&self, sequence: u64) -> bool {
        self.buckets.contains_key(&sequence)
    }
    pub fn append_existing(
        &mut self,
        sequence: u64,
        action: DestroyAction,
    ) -> Result<(), DestroyAction> {
        match self.buckets.get_mut(&sequence) {
            Some(bucket) => {
                bucket.push(action);
                Ok(())
            }
            None => Err(action),
        }
    }
    /// 旧候选接口。completed 参数必须是已证实的命令水位，不能传 timeline。
    pub fn insert(&mut self, sequence: u64, dispose: impl FnOnce() + Send + 'static) {
        self.buckets
            .entry(sequence)
            .or_default()
            .push(Box::new(dispose));
    }
    pub fn remove(&mut self, sequence: u64) -> Option<Vec<DestroyAction>> {
        self.buckets.remove(&sequence)
    }
    pub fn dispose_cb_resource<T: Send + 'static>(&mut self, sequence: u64, resource: T) {
        self.insert(sequence, move || drop(resource));
    }
    /// consumer 必须先完成 SequentialDisposal 的 gap 检查；此处没有第二个序列调度器。
    pub fn drain_completed_sequence(&mut self, sequence: u64) -> Result<usize, Failure> {
        let actions = self.remove(sequence).ok_or(Failure::MissingBucket)?;
        let count = actions.len();
        for action in actions {
            action();
        }
        Ok(count)
    }
    pub fn dispose_in_order(&mut self, completed_sequence: u64) -> usize {
        let keys = self
            .buckets
            .range(..=completed_sequence)
            .map(|(k, _)| *k)
            .collect::<Vec<_>>();
        keys.into_iter()
            .map(|key| self.drain_completed_sequence(key).unwrap())
            .sum()
    }
    pub fn pending(&self) -> usize {
        self.buckets.values().map(Vec::len).sum()
    }
    pub fn drain_after_idle(&mut self) -> Vec<DestroyAction> {
        std::mem::take(&mut self.buckets)
            .into_values()
            .flatten()
            .collect()
    }
}
impl Drop for CommandBufferDisposalBuckets {
    fn drop(&mut self) {
        // 未取得完成证据时，Rust 容器析构不能提前销毁仍在 GPU 使用的对象。
        for action in self.drain_after_idle() {
            std::mem::forget(action);
        }
    }
}
pub type SequentialDisposal = CommandBufferDisposalBuckets;
