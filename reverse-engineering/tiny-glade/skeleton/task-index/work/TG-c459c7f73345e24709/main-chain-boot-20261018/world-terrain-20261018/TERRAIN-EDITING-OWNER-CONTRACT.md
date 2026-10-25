# TerrainEditingData：W3 同 owner 字段交接

这是供当前 canonical resources/system owner 增量整合的接口合同。本任务未修改生产 core，不能用另一份 ActiveStrokes Resource 或按原Bevy布局 cast 代替。

## 原身份与实际字段

原 W3 `rasterize_terrain_stroke@1408c7120`：

- `1408c71c2/71cc` 两个 TypeId 字 `1e38c16726d0fb8d / ec8924b33809588b`。同一原立即数在 `App::insert_resource<country_core::resources::terrain_editing::TerrainEditingData>@141532740` 的 `141532796` 出现，完整 raw scan+PDB mapping 在本目录 `resource-id-producer-mapping.json`。
- `1408c7229` 获得该真实资源指针，`722d` 保存；`7234/723a` 无条件把 World.change_tick 写入该 row 的 changed tick。发生在第三项事件资源查找、任何早退判断之前。
- `1408c723d/7247` 的 `cda6af2f6afb130e / 036e88a927bd70aa` 是 `Events<RecalculateHeightMapCmd>`，由原 `EventRegistry::register_event<...>@1415158a0` 和两个 SystemParam init_state 同立即数证实。原 `+10/+30` 是双缓冲事件长度，不能叫独立 PathStrokeQueues/StrokeEditCounts。
- `1408c72a8..72bd`：若事件两个长度都0并且 TerrainEditingData byte+40 ==1，直接早退。事件非空时即使 byte+40 已经1也运行。原不通过EventReader、不drain，不造“只消费一次”。
- `1408c895b/8962`：整个栅格化、两次模糊、地形导数 **四个 pass 声明完成后** 写同一 byte+40=1。这是声明时状态，不是GPU完成标志。

## 原 producer 初值

本目录 `140940190-full-rne.asm` 是完整 PDB extent3054B的发行RNE。

- `140940393` 写 `[rbp+d60] u32=0`。
- `14094039d` 写 `[rbp+da0] byte=0`。
- `1409403b9` 把 base=`rbp+d60` 交给 `insert_resource<TerrainEditingData>@1409403c3`。

因此该同 owner byte+40 初值确为 false。不要将 bool 放在全局/另一个 GPU resource，也不要每帧重置为 false。当前 wrapper `TerrainEditingData(pub tg_terrain::stroke::TerrainEditingData)` 仅公开 strokes+next_index，漏掉该实际渲染状态。

## 最窄现架构接口

建议 owner 保留一个 canonical TerrainEditingData，并提供等价的安全字段借用：

```rust
pub fn rasterization_parts(
    &mut self,
) -> (
    &std::collections::HashMap<StrokeId, TerrainStroke>,
    &mut bool, // 原 byte+40，描述名 rasterized
);
```

若 owner 已将 wrapper 改成具名字段，可直接提供真实 `rasterized` 字段及 strokes 借用，语义相同。该接口不新增状态，不宣称当前 Rust 布局等于原 ABI。现有 tuple literal/构造入口的迁移由 canonical owner统一做，初值false。反序列化替换/clear是否额外重置此字段仍须核对应原consumer，不因方便猜测。

宿主 W3：取得真实 TerrainEditingData 的 `Mut` 后，**显式 `set_changed()`**，再取 Events<RecalculateHeightMapCmd> 并用真实 `!events.is_empty()`，然后借用上述 map/bool交同一恢复业务体。缺任一实际资源返回错误，不 init Default补缺。

## 实际 GPU 数据与寿命

W3 原 map key 低32位用于 `(id<<10)|(count&0x3ff)`；真实 value的 height_ws/falloff打f16对。每个世界 XYZ 点原 `1408c76dd/76e3` 取+0和+8，是 **XZ**，旧手写 p[0]/p[1]错误。

原点分配是 `min((len+1)&!1,1024)*8`；原shader存在uint4/16B物理读取，含 count1 场景，所以必须保留完整round-even读范围，不能只声明n*8。只写有意义XZ bytes，原padding值未证；本合同不要求主动填0。Current host若实际安全检查必须初始化padding，必须另列有界宿主适配并保持point_count不变。

infos原 `1408c7bd5..7bfa` 精确分配count*16，包括0；root已核当前DynamicConstants将支持原合法空chunk，保留真实page/address。原发行SPV `8112..8118` 的 `ULessThan(0,stroke_count)` 控制进入body327；infos address/load仅在body327内，count0不读取infos/points。因此空chunk不声明零长度GPU读取，但不得替换成假的address0/max16。

真正非空 DynamicConstantsChunk 将经 RecordedComputePass typed metadata → 同一GraphPasses session import actual Buffer+range → compute read0x1a/group1声明 → 原现executor `managed.retain_pass_resources` → command ResourceOwner → 同sequence完成退役。没有u64生命周期假token，也不调用旧空 `track_gpu_lifetime()`。
