# Shared random_splits consumer interface（B，只读）

本文件仅为本claim的Chimney/Door消费者补共享依赖合同；未取得utils源claim，不写共享实现。复用旧shared-foundation原证导航，再以原RNE/PDB重新采集完整 `140c90350/332B` 和 `140c90050/760B`，见 `random-splits-readonly-B/`。

## 已证接口与输出

高层接口的业务参数形状为 `random_splits(count: usize, jitter: f32, rng: &mut Rng) -> Vec<f32>`。Win64 retVec hidden pointer在rcx；count在rdx；jitter在xmm2；rng在r9。额外stack参数原本只是Rust track-caller Location，不能误算为第四业务参数：`142fce870` 原数据解析为文件 `crates/systems/decorator/src/decorator_visual/chimney_assemble_bricks.rs` line264 col9。

wrapper count<=1会panic；按count预留f32 Vec后转发into。`random_splits_into(count,jitter,rng,&mut Vec)`不清原Vec，而是追加：count<3分支追加[0,1]；count>=3追加0、count-2个内部点、1。into自身没有wrapper的count>=2守卫，不能把wrapper和into的小count语义混淆。

返回的是归一化边界位置，不是总长为total的各段长度。Chimney helper把返回值仿射映射到两端高度，再按相邻点构造砖；Door helper也复用此分界点业务。

## 数值合同（原常量已核）

count>=3时：

- `step = 1.0f32 / (count as f32 - 1.0f32)`（计算顺序按native保留）；
- `max_jitter = (1.0f32 / ((count-1) as f32)) * f32::from_bits(0x3efd70a4)`；原系数是0.4950000047683716，不能猜0.5；
- 原min/NaN选择令 `jitter=NaN` 时采用max_jitter，普通值取较小值；负输入不被额外clamp到0；
- 对i=1..count-2，点为 `(rng.f32() - 0.5f32) * effective_jitter + (i as f32) * step`，每内部点一次rng.f32；两端0、1不消耗rng；
- 不添加排序、归一化或累计尾差。Vec公开实现直接复用，不恢复其reserve/grow内部。

完整浮点极值/有符号零的Rust源等价仍应由共享owner实现时独审；此处提供原执行顺序，不用宽泛数学等价替代。

## 当前不兼容处

`system-decorator/src/utils/mod.rs`现3参猜测接口是 `(rng,total,n)` 并返回随机段长、n0返回空、末段并尾差。这与原normalized边界点/count守卫不等价。utils crate源码当前未找到真实同名实现。两个assembly恢复不能直接调用该本地同名函数，也不应各自再复制一份猜测版本。应由root分配共享owner后恢复/暴露一次真实helper，再由消费者复用。

本依赖不影响已冻结tracker/Params协议候选及其8项同源测试；完整geometry host接线需要此合同。C未核、未运行native目标、没有吞吐数据。
