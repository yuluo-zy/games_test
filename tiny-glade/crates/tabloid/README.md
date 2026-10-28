# tabloid

`tabloid` 是累积增量的发布订阅模块。它把连续变化合并为批次，供碰撞、地形等系统按自己的更新节奏消费。模块不保存世界的完整状态，也不提供持久化、事件重放或异步唤醒。

## 数据模型与公开接口

| 接口 | 职责 |
| --- | --- |
| `State: Clone` | 定义空批次 `empty()`，并提供与原批次相互独立的复制 |
| `StateDelta<T>` | 通过 `apply_to(&self, &mut T)` 把一次变化合并进批次 |
| `Publisher<T>` | 登记订阅、发布增量、维护全局发布序号 |
| `Subscriber<T>` | 通过 `take(&mut self)` 取走当前累积批次 |
| `PublicationReader<T>` | 只读拥有一个批次；通过 `Deref<Target = T>` 访问状态，通过 `sequence()` 访问发布序号 |
| `Publication<T>` | 状态与序号的公开数据结构；实际投递由模块管理其 `Arc` 所有权 |
| `PublishError` | 发布失败原因；当前只有序号耗尽 `SequenceExhausted` |

`Publisher::new()` 与 `Default` 创建空发布者，初始序号为 `0`。`subscribe(&mut self)` 创建独立消费边界；新订阅者只接收此后的发布。`publish(delta)` 成功后序号加一，即使没有订阅者。`take()` 在没有待消费变化时返回 `None`。

克隆 `Subscriber` 会共享同一个消费边界，其中一个克隆取走批次后，其他克隆的 `take()` 返回 `None`。需要独立消费时，应再次调用 `subscribe()`。

## 合并流程

一次发布按以下流程执行：

1. 检查序号能否递增，清理失效订阅。
2. 按登记顺序锁住存活的消费边界。
3. 对每个不同的待消费批次复制状态并应用增量；空边界从 `State::empty()` 开始。同一批次的订阅者共享这次计算得到的新快照。
4. 提交全部替换，更新发布序号，释放全部锁，再销毁被替换的旧快照。

取走快照会重置该消费边界。之后的发布从空批次继续累积；已取走的 reader 内容不会随之改变。

| 操作 | 快消费者待消费 | 慢消费者待消费 |
| --- | --- | --- |
| 发布 `1` | `[1]` | `[1]` |
| 快消费者 `take()` | 空 | `[1]` |
| 发布 `2` | `[2]` | `[1, 2]` |
| 两者分别 `take()` | 得到 `[2]` | 得到 `[1, 2]` |

## 使用示例

```rust
use tabloid::{Publisher, State, StateDelta};

#[derive(Clone, Debug, PartialEq)]
struct DirtyBits(u64);

impl State for DirtyBits {
    fn empty() -> Self {
        Self(0)
    }
}

impl StateDelta<DirtyBits> for u64 {
    fn apply_to(&self, state: &mut DirtyBits) {
        state.0 |= *self;
    }
}

let mut publisher = Publisher::<DirtyBits>::new();
let mut fast = publisher.subscribe();
let mut slow = publisher.subscribe();

publisher.publish(0b01_u64)?;
let first = fast.take().unwrap();
publisher.publish(0b10_u64)?;

assert_eq!(first.0, 0b01); // 持有旧快照不阻止新发布。
assert_eq!(first.sequence(), 1);
assert_eq!(fast.take().unwrap().0, 0b10);
assert_eq!(slow.take().unwrap().0, 0b11);
# Ok::<(), tabloid::PublishError>(())
```

## 并发、生命周期与错误约定

发布和新增订阅需要发布者的可变借用。订阅者可在其他线程取走批次，具体 `Send` / `Sync` 能力由状态类型和 Rust 自动 trait 推导决定；模块不提供额外线程池。发布准备期间，`take()` 会短暂等待；返回的 reader 不持锁，保留它不会阻止后续发布。

发布者仅保存订阅的弱引用。最后一个订阅者销毁后，其待消费快照可立即释放；后续发布或新增订阅会清理登记项。reader 独立拥有快照，即使发布者、订阅者已经销毁仍可读取。旧快照的状态析构发生在订阅锁释放后，因此状态的析构函数可以访问订阅者。

`State::empty()`、`Clone::clone()` 和 `StateDelta::apply_to()` 在订阅锁内调用，必须保持简短，不能重入同一个发布者或订阅者。`Clone` 必须生成可独立修改的状态；共享可变内部数据会破坏快照语义。增量只保证每个不同待消费批次应用一次；实现不得假设一次发布只调用一次 `apply_to()`。

序号达到 `u64::MAX` 时，发布返回 `SequenceExhausted`，序号与待消费批次均保持不变。状态构造、复制或增量应用发生 panic 时，模块不吞掉 panic，也不提交候选批次或递增序号。模块拥有的候选与已准备快照在全部订阅锁释放后析构；回调内部尚未交出的局部对象仍由回调负责清理。外部副作用或共享可变状态不能回滚，调用方应避免让回调 panic。

## 性能约束

设存活消费边界数为 `S`，不同待消费批次数为 `B`，其中 `B <= S`。每次发布需要 `O(S)` 的登记、锁定与替换工作，以及 `B` 次空状态构造或状态复制、增量应用。各消费节奏一致时可共享批次；消费节奏分化时会分别计算。

reader 的创建与读取无需复制状态。发布过程会分配临时容器和新快照；这不是无分配通道。批次包含无限增长的容器时，连续发布的状态复制也会持续变贵，应由业务选择紧凑、可合并的表示，例如固定大小的脏位图。模块没有容量上限或背压。

## 验证与维护

在工作区根目录执行：

```sh
cargo test -p tabloid --all-targets
cargo test -p tabloid --doc
cargo clippy -p tabloid --all-targets -- -D warnings
cargo run -p tabloid --release --example review_publication_cost
```

单元测试覆盖独立与共享消费边界、快照稳定性、批次复用、订阅回收、序号耗尽、并发消费、正常提交及失败准备的锁外析构；`tests/dirty_tiles.rs` 使用 512 字节位图验证 OR 合并。成本示例统计固定与增长状态在不同订阅数下的复制、应用、分配次数和耗时，耗时结果取决于本机负载。

生产者与消费者的游戏系统接线由使用方负责。历史恢复来源见 [foundation-audit.json](../../provenance/foundation-audit.json) 和 [原始调查报告](../../../reverse-engineering/tiny-glade/evidence/foundation-plt-2026-10-13/SUMMARY.md)；它们不代替当前源码的接口约定。

本轮源码复审、工程化改造及验证边界见 [复审报告](../../reports/building/2026-10-18-building-foundation-review/README.md)。
