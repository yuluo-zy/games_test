# W2 / W12 原版独立核验合同

本次只读审核 W2 `0x140a12680`、W12 `0x140a22140`、调用方及两项 staging upload 的完整 PDB 范围，以及发行包内 `gauss_blur_mask` 的完整 SPIR-V。没有修改 production、运行 Cargo 或 GPU，也不把候选接线解释为原版运行结果。身份、范围长度、原字节 SHA 和 SPIR-V 指令索引见 `original-manifest.json`；`extract_original.py` 可重复提取。

## 资源条件与消费时点

两项原函数都取真实命名 `WorldRaster<512>` 资源；缺资源进入原 unwrap panic 路径。条件是 `+0x29b`（当前 source 的 `reupload_pending()`），不是 `staging_dirty`、`published` 或 dirty bitmap 是否非空。

| 动作 | Water W2 | Garden W12 |
| --- | --- | --- |
| 读取当前世界 tick `world+0x5b0` | `140a1271b` | `140a221db` |
| 检查资源 `+0x29b == 0` | `140a12721` / `12728` 跳过 | `140a221e1` / `221e8` 跳过 |
| 写资源 changed tick `row+0x5c` | `140a1272e` | `140a221ee` |
| 写 `+0x29b = 0` | `140a12731` | `140a221f1` |
| 准备 `resource+0x98, 0, 0x200` | `140a12738..12745` | `140a221f8..22205` |
| 调用 memset 清 dirty bitmap | `140a12747` | `140a22207` |
| 调用 `TextureDesc::new_2d(512,512,0x61)` | `140a12763` | `140a22223` |

所以真正的顺序为 **条件成立后 changed tick → reupload=false → 清 512 字节 dirty tile bitmap → 新建临时图 → 声明 pass**。条件不成立时，changed tick、标志和 bitmap 均不变。该函数并没有再次执行 GPU upload；`reupload=false` 是标志消费，不能被误读成 upload 的布尔入参。

`+0x98..+0x298` 是 4096 个 tile 的位图；手写旧 `WaterRasterStaging.staging` 注释把它称为“暂存字节”不准确。实际像素缓冲在 `WorldRaster` 的 Vec 里，此处不会擦除像素、tile hashes、selector、staging_dirty 或 published。

原 `render_world` 在 `1404ec2dd` 以 preset `0x13` 调用 Water upload `1403d9610`，在 `1404ec2eb` 以 preset `0x14` 调用 Garden upload `1403d9b30`，再于 `1404ec2f6` 调 W2，随后其他世界 pass，于 `1404ec364` 调 W12。两个 upload 检查 `+0x29a`，通过格式验证后写 changed tick，并以 word `0x0100` 写 `+0x29a`（Water `1403d9723`、Garden `1403d9c43`），也就是 `published=false, reupload=true`。W2/W12 因而消费同一帧 W1 设置的真实资源标志；不能用另一个 staging 副本替代。

## 原 shader 的确切通道行为

发行文件 SHA `3474c437ce66699d60a2aabcbea699c58a48408879e359b3b38b63b44a2d8258`；offset 23 起的完整 1478 words SHA `6bf38b9a8f8aad703574c82eefb16673cbb2e4798c8e4f52c38a53e97a70a07d`，与已有 water owner 的 retained SPV 逐字节相同。完整走完 243 条指令，没有截断。结论以这些原指令为准，手写 HLSL 仅作对照。

- `$Globals` / input / output 分别为 shader binding 13 / 14 / 15；output 的原类型是 `Rgba16f` storage image。
- 水平分支 `OpImageFetch %92` 后 `OpCompositeExtract ... 0`：固定取 input.r；垂直分支 `OpImageFetch %97` 后 `OpCompositeExtract ... 1`：固定取 input.g。取源通道不依赖 output_channel。
- `output_channel == 1`：word 1368 的 `OpImageRead %112` 读取旧目标，`OpCompositeInsert %113 ... 1` 只替换 g，word 1384 写回整个向量。
- `output_channel == 2`：word 1435 的 `OpImageRead %120`，`OpCompositeInsert %121 ... 2` 只替换 b，word 1451 写回整个向量。
- 两个目标读取分支都不受 horizontal 条件屏蔽；其他 output_channel 不写目标。
- 原半径为 `ceil(2*sigma)`，包含两端；越界 tap 被跳过，权重为 `exp(-offset²/(2*sigma²))`，结果除以实际累积权重。该范围内没有算法差异需要修正。

完整原 native 也确认 Water 的 uniform 顺序为 `(true,10,1)`、`(false,10,1)`、`(true,30,2)`、`(false,30,2)`；Garden 为 `(true,5,1)`、`(false,5,1)`。原常量字节分别是 `00002041`、`0000f041`、`0000a040`。

**Water 第三 pass 的输出是 temp.b，第四 pass 仍读取 temp.g。** 第三 pass 必须保留第一 pass 写出的 temp.g；不能改 shader 为“vertical 按 output_channel 读取”，也不能把两个 sigma 各自解释成独立完整的二维高斯。按原代码，第四 pass 对第一 pass 的水平 sigma=10 结果做垂直 sigma=30；第三 pass 的 temp.b 不被第四 pass 读取。这是原行为，不是本次应该纠正的业务缺陷。

## 精确 preserving 内容意图

| pass | 输入 → 输出 | 更新通道 | 写绑定 preserve |
| --- | --- | --- | --- |
| Water 1 | preset13 → temp | g | 否；首写没有活的旧目标通道 |
| Water 2 | temp → preset13 | g | 是；保留已上传的 r 及其他通道 |
| Water 3 | preset13 → temp | b | 是；尤其保留 Water 1 的 temp.g 给 Water 4 |
| Water 4 | temp → preset13 | b | 是；保留 r 与 Water 2 的 g |
| Garden 1 | preset14 → temp | g | 否；首写没有活的旧目标通道 |
| Garden 2 | temp → preset14 | g | 是；保留已上传的 r 及其他通道 |

首写在 SPV 层面仍然是 image RMW，但旧 r/b/a 没有被该链观察到；无需凭空添加零清除或把首写标成需要历史初始化。原 `0x19` 输入、`0x32` 输出的访问码、group、生命周期和 barrier 均应保留。preserve 仅表达当前恢复宿主必须保留活的前序内容，不应改变原 uniform 或修补原 shader 算法。

## live-mask 候选审查

审核候选：`TG-0ef1874c29a03d0dec/main-chain-frame-20261018/live-mask-candidate`。对应文件 SHA 见 `reviewed-candidate.json`。本闭包未发现阻止集成的语义错误；Garden 地址注释应由 `140a22201..22d` 改为 `140a221ee..22207`。

- 两个 live adapter 使用真实 `Mut<world_raster_defs::{WaterRaster,GardenRaster}>`。bypass 检查之后才 `set_changed()`，再 bypass 调 `consume_reupload()`，最后声明 pass；不使用原编译器 `ResourceView` 布局 cast。
- 现 `consume_reupload()` 恰好先 false 再 clear dirty bitmap。既有 legacy entry 和 live entry 共用一个 `declare_*_blur`，没有复制或改变原序列。
- Bevy 0.16.0 的 `resource_scope` 将当前 `World::change_tick()` 作为 `Mut` 的 `this_run`；`set_changed()` 写 `this_run`，wrapper 读取的 `read_change_tick()` 是同一值，条件不成立时不会被 bypass 检查误标 changed。
- 当前两图 pass 每项都先声明 input 再 output，因此 `bindings` 中 read/write 的索引是 **0/1**，不是 shader binding **14/15**。`preserve_storage_texture_contents` 通过目标名字 resolve 后查 `AccessRange.access_index`，收入的是精确 output index **1**。Water 2/3/4 和 Garden 2 的标记与上表一致。
- host 按同 resource、同 read/write 性质逐 occurrence 匹配实际 ImageView，之后通过该 occurrence 的 index 查询 preserving 标记，传给 `add_preserving_resource_access_range`。没有按 resource owner 扩散到源读绑定或其他 view。
- 两个 scene wrapper 先检查真实资源，再用 `resource_scope` 借出其所有权；声明函数只依赖 preset/temp，不回读被 scope 暂时移出的 raster，因此这一调用没有资源别名 cast 或第二份 raster。

还需由 root 对最终 production SHA 运行定向 CPU/GPU 验证；本审查没有执行该验证。W13/W16、完整 frame consumer 以及其他资源缺口不在本合同的结论范围内。
