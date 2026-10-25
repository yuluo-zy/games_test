# resources/rt/bvh_builder.rs

> 文件作用：**RtBvhBuilderGlue 后台构建器**——独立线程 worker 消费 RtWorld 脏网格列表，把脏三角构建为 GPU BVH。
> 状态机在 RtBvhBuilderGlueResource 内：busy 引用计数（+0x10，LOCK/UNLOCK）、状态字节（+0x14）、处理标志（+0x15）、
> 退出标志（+3）；全局共享停止标志 0x14362c420。空闲时 sleep 1ms（FUN_140d55980）。

函数总计 6：业务 1 / 内联无独立符号 1 / 派生胶水 4

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `bvh_builder_worker` | 0x140b24d50 | 41 | 循环：抢 busy（LOCK/DEC refcount 语义）；state==2 时 sleep 再循环；否则从 +0x98 数据指针拷脏网格列表（+0xd..+0x12 域），置处理标志 +0x15=1，交给 build_gpu_bvh；按全局 0x14362c420 判定停止/世界活动 | O | roofs-rt/decomp/decomp-0x140b24d50.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `RtBvhBuilderGlue::is_idle`（行 24）— busy 计数==0 谓词（内联）

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 4 项（bvh_builder_worker 的嵌套闭包 + RtBvhBuilderGlue 默认）

## 布局/结构要点

- RtBvhBuilderGlueResource：`+0x10` busy/refcount（int）、`+0x14` state 字节（2=待机）、`+0x15` 处理标志、
  `+0x98` 数据指针；`+0x3` 退出标志（Bevy system param 判定）。
- 交叉域：update_rt_world（rt/mod.rs）触发 builder 创建；upload_rt_world 消费产出。

## 实现方式（算法级）

完整算法见 `roofs-rt/ALGORITHMS.md` §4。要点（**偏移更正：raw bytes 0x140b24d50 前导复核**）：

### bvh_builder_worker（0x140b24d50）状态机
```
loop:
  抢 busy: LOCK; cmpxchg [glue+0x10]: 0→0x3fffffff; 非零 → panic
  停止标志 0x14362c420（首 qword & 0x7fffffffffffffff 非零 → fastrand 随机停止）
  if glue+0x18 != 0: break（退出 → 日志 + Arc drop glue）
  state = [glue+0xa0]; [glue+0xa0] = 2
  if 先前 == 2（空闲）: 释放抢注; sleep(0, 1ms); continue
  # 处理路径：
  [glue+0xa8] = 1（处理中）
  读脏列表 {ptr@+0x68, len@+0x70, count@+0x98, 计数@+0x78..+0x94, 脏字节@+0xa1..0xa7}
  占用掩码 → FUN_14130d1b0 组装三角缓冲（0x28 步长）→ build_gpu_bvh → 新 BVH
  再抢 busy; 停止检查
  if glue+0x18 == 0: 交换新旧 BVH（+0x20..+0x60 十二组 u32 写回）→ 释放旧缓冲
                     （tri 0x50 / node 0x18 / meta u32）→ [glue+0xa8] = 0 → 释放抢注
  else: 释放新缓冲; 释放抢注
  释放临时三角缓冲; FUN_1400524e0(脏列表, 计数, 0x20, 0x10)  # 完成通知（与 update 对称）
```
**关键偏移（raw bytes）**：`+0x10 busy（cmpxchg 0→0x3fffffff）、+0x18 退出/停止字节、+0xa0 状态（2=空闲）、+0xa8 处理标志` —— 章内/DIGEST 的 "+0x14/+0x15" 为旧读，已更正。
