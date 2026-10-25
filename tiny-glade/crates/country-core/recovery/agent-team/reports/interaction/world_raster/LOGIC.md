# LOGIC — world_raster / paths / raster_stroke / active_raster_stroke 逐函数逻辑判等

审计员: logic-verifier-worldraster | 日期: 2026-10-17
协议: planning/2026-10-16-logic-verify-protocol.md
方法: ghidra `--worker shared` fresh 反编译(fields=decompiled_code,callees),逐函数重建控制流并与 Rust body 逐语句对照。
证据落盘: planning/logic-worldraster/decomp/*.txt(本批 40+ 函数 fresh decomp + 机器码 read_memory)。

## 判定汇总

| 模块 | 判等 | 未实现-需重做 | 胶水-非业务(未审计) | 未实现(不在范围) |
|---|---|---|---|---|
| resources/world_raster.rs | 10 | 0 | publisher shared/secondary、访问器、extend_dirty、write_composite(=write 内联逻辑) | — |
| resources/world_raster_defs.rs | 5 | 0 | SystemParam/derive 胶水、deref ICF | — |
| resources/raster_stroke.rs | 2 | 0 | fmt(Debug 胶水)、clone | add_point/smooth/length(原 inline,src 缺失) |
| resources/active_raster_stroke.rs | 5 | 0 | index_mut、default | — |
| systems/world_raster.rs | 3 | 0 | init_state 等 SystemParam 胶水 | — |
| systems/paths.rs | 1 | 1 | — | draw_path/record_path_history/path_brush_preview_size/mod::build(4 原函数无 src) |

**合计: 26 业务函数判等, 1 未实现-需重做, 无空壳/假逻辑。** 原函数内部逻辑全部逐分支对上 fresh 反编译。

---

## resources/world_raster.rs(10 判等)

| VA | 函数 | verdict | 分歧点 |
|---|---|---|---|
| 0x1409393e0 | build_tile_hasher | 判等 | 依次写 +0/+8/+0x10/+0x18 四个 u64 = TILE_HASH_KEYS,逐字节一致 |
| 0x1409393b0 | DirtyTilePublisher::unique | 判等 | 写 7 qword 初值;src 用 tabloid Publisher::new()(共享依赖) |
| 0x140a15530 | WorldRaster::new<512> | 判等 | alloc_zeroed composite/scratch 0x40000;alloc+逐 u32 填 tile_hashes(4096×0x07954dbb);alloc_zeroed tile_buffer 0x40;publisher 搬入 +0x60..0x97;memset +0x98..+0x29b(0x204B)清零 |
| 0x140a158e0 | WorldRaster::subscribe<512> | 判等 | 判别式: 首 qword==i64::MIN→Shared(锁+订阅)、==0x8000000000000001→Secondary(panic)、其它→Unique(直订)。src 三路 match 语义一致 |
| 0x140a15a10 | clear_buffer<512> | 判等 | selector==0→memset composite(ptr+8,len+0x10);否则 scratch(ptr+0x20,len+0x28);len==0 跳过 |
| 0x140a15da0 | clear_all<512> | 判等 | composite+scratch 清零、dirty memset 0xff(0x200B,+0x98)、staging_dirty(+0x299)=1、tile_hashes 逐 u32 填 0x07954dbb |
| 0x140a161a0 | copy_composite_to_scratch<512> | 判等 | len 不等→len_mismatch_fail panic;memcpy(scratch←composite) |
| 0x140a16580 | commit_scratch_to_composite<512> | 判等 | 同上反向 |
| 0x140a16960 | rasterize_stroke<512,…> | 判等(结构有偏差,行为等价,见下) | 点列<2 跳过;MoveTo=0/LineTo=1 判别;坐标 x/130+0.5×512;blend 选择 flags&1/draw/env0=Max/env1=Erase/env2=SatAdd;!draw&&flags&1→panic("not implemented");宽度=2×radius/130×512 |
| 0x140a172d0 | copy_to_staging<512> | 判等 | staging_dirty→out.fill(0)+清标志;selector 记 +0x298;retain(source/tile_buffer/tile_hashes/out);publish(Unique 直发/Shared+Secondary 锁发);published=1。哈希=build_tile_hasher 状态对 64B 调 Hasher::write 取 finish 低 32 位(src hash_tile 派生种子经 3 组转录向量验证与原生一致) |

## resources/world_raster_defs.rs(5 判等)

| VA | 函数 | verdict | 分歧点 |
|---|---|---|---|
| 0x14083eaf0 | PathRaster::new | 判等 | 直通 WorldRaster::new<512> |
| 0x14083eb10 | AllWorldRasterWrite::ordered_names | 判等 | 6 名长度 10/0x12/0x12/0x12/0xb/0xc(即 Path/C1/C2/C3/Water/Garden) |
| 0x14083ebe0 | AllWorldRasterWrite::write | 判等 | 逐槽(前 data.len 项):None→清 composite;Some→len 不等→format "Length mismatch; expected {}, but got {}"+unwrap panic,否则 memcpy;每槽后 dirty set_all+staging_dirty=1;data.len<6→尾部 unwrap panic。注:原数据形状是 (String,Option<Vec<u8>>)40B(名字被忽略),src 用 &[Option<Vec<u8>>]——Vec 处理行为一致 |
| 0x14083f410 | AllWorldRasterWrite::would_write_succeed | 判等 | 逐槽顺序校验:缺失→"End of stream while expecting raster {}";名字→"Expected {}, but got {}";Some→当前缓冲 len(按 selector 取 composite/scratch)与 data len 比对→"Length mismatch…"。None 跳过长度校验 |
| 0x14083fa60 | AllWorldRasterWrite::upload_to_staging | 判等 | 逐槽: FrozenMap get(unwrap if None)→名字/kind memcmp 断言→copy_to_staging(0, mapped)。src 走 SsboId 泛型 get_mut,等价 |

## resources/raster_stroke.rs(2 判等)

| VA | 函数 | verdict | 分歧点 |
|---|---|---|---|
| (inline) | RasterStroke::radius | 判等 | 字段读 |
| 0x140a97860 | RasterStroke::iter_points_xz | 判等 | fresh 机器码 0x140a97860 = `mov rax,[rcx+8]; mov rcx,[rcx+0x10]; lea rcx,[rcx+rcx*2]; lea rdx,[rax+rcx*4]; ret` = 计算 points 切片边界(ptr,ptr+len×12);src 的 points.iter().map(x/z) 等价。反编译为空(小 thunk),以机器码为准 |

## resources/active_raster_stroke.rs(5 判等)

| VA | 函数 | verdict | 分歧点 |
|---|---|---|---|
| 0x140868f70 | InProgressRasterStroke::last | 判等 | last().unwrap(): len==0 或 (len==1 且 ptr==0)→unwrap_failed。胶水 |
| 0x140868fb0 | InProgressRasterStroke::add_point | 判等(浮点微差,建议修) | 空→入队返回;distance=(dz²+dy²)+dx² 后 +=;len>1.0(NaN 不触发)→(fluid_started&&cursor≥2? 固化 input[..cursor]+drain(..cursor-1)+cursor=len : 重复入队+cursor=len+length=0)。**分歧**: 距离平方加序原生=(dz²+dy²)+dx²,src=(dy²+dx²)+dz²,仅最后一位舍入可不同 |
| 0x1408691b0 | InProgressRasterStroke::solidify_fluid_into | 判等 | 相邻去重(闭包:保存上点,!=才留,NaN 恒不等);<2 点原样;否则 try_new_from_points(closed=false).unwrap()+crashy_resample(0.5);out 空→全量追加,否则 skip(1) |
| 0x140868d00 | ActiveRasterStroke::start_new | 判等 | 清 solid/input/resampled、length=0、fluid_started=0(fluid_cursor 与两份位图不动);写 brush_size+0x60/mode+0x64;add_point(point);resampled.push(point);写 param_a+0x68/param_b+0x6c。顺序逐一对应 |
| 0x140868e40 | ActiveRasterStroke::resample_and_smooth | 判等 | resampled=clear+extend(solid);solidify_fluid_into(input,resampled);smooth_in_place(10,false) |

## systems/world_raster.rs(3 判等)

| VA | 函数 | verdict | 分歧点 |
|---|---|---|---|
| 0x140957990 | record_raster_history | 判等 | take_history_stroke 语义:读+清 resampled/brush_size/mode,Arc<RasterStroke>{flags=param_a,strength=param_b,points=Arc<[Vec3]>,radius=旧 brush_size,draw=mode!=0};construct_edit→History::add。slot×0x470 寻址一致 |
| 0x140957bf0 | upload_world_raster_buffers_to_staging | 判等 | 业务门=存在待读 UploadWorldRasterBuffersToStagingCmd 事件→AllWorldRasterWrite::upload_to_staging。Bevy tick 算术(SystemParam 变更检测)未逐位解码,但属库内胶水,门语义一致 |
| 0x140aba770/4750/142260ae0/…(11 实例) | draw_raster<T,C> | 判等(浮点微差,建议修) | 判别 0/1=开始、2=续画、≥3=提交。**开始**: decorator_cull_allow_any(archivist+0x1e0=1)+start_new(cursor.position,draw,radius,disc,strength)。**续画**: last().unwrap→distance=((Δz²+Δy²)+Δx²).sqrt,`!(distance>0.001)`(NaN 同原生 jbe 语义跳过)→add_point→resample_and_smooth→clear dirty_current(+0x270)→rasterize_world_space_stroke→extend(current→raster.dirty)→extend(previous)→swap_dirty(+0x270↔+0x70)→copy_composite_to_scratch→rasterize_stroke(1,scratch)→stage(copy_to_staging)→mode!=0?on_draw:on_erase(point,brush_size,false)。**提交**: resampled.len<2→返回;3 个事件(debug_history/on_record/tool_used);extend(dirty_previous)+clear;commit_scratch_to_composite;stage。逐一分支对上 |

## systems/paths.rs(1 判等 + 1 需重做)

| VA | 函数 | verdict | 分歧点 |
|---|---|---|---|
| 0x140ae4bd0 | DrawState::update | 判等 | r=min(mode_id-2(u8 回绕),3);r0→0;r∈{1,2}→(mode-1 回绕≥2?raw:2);r==3→(mode-1 回绕<2?3:0)。三分支全部对上 |
| 0x140ae4c20 | PathRastersResources::as_float_grid | **未实现-需重做** | **原函数无 index 参数,一次返回全部四个路径栅格引用**(64B 结构=4×{ptr,len},按各栅格 +0x298 selector 选 composite/scratch)。src 实现为 `as_float_grid(index:u8)->RasterBufferRef`,按 index 只取一个——**签名与逻辑均不符(自造近似)**。当前无调用者(draw_path 未实现),无运行影响,但按协议标未实现-需重做 |

---

## 未实现(不在本次逐函数范围,单独登记)

- `systems/paths/draw_path.rs::draw_path`(0x140ae4d00)— src 无 draw_path.rs,业务未实现。`ERASER_BRUSH_SCALE=0.5525`/`BRUSH_RECT_SCALE=0.8` 常量已在 src/paths.rs 声明但无消费者(死常量)。
- `systems/paths/record_path_history.rs::record_path_history`(0x140b1c270)— 未实现。
- `systems/paths/path_brush_preview_size.rs::path_brush_preview_size`(0x140b1c010)— 未实现。
- `systems/paths/mod.rs::impl$0::build`(0x140b68e40)— 未实现。
- `RasterStroke::add_point/smooth/length`(原 inline,line 46/63/70)— src raster_stroke.rs 未含,由 active_raster_stroke 的 add_point/resample_and_smooth 承接等价语义。
- `world_raster::_::impl$0::deserialize<Value>`(0x140a197c0)与 `serialize<…>`(0x14003c3b0)— serde 胶水(AGENTS.md 计合同不计数业务),src world_raster.rs 未含;当前保存流走 AllWorldRasterWrite::write_named,非该 serde 路径。

## 严重分歧(需处理/登记)

1. **as_float_grid 签名与逻辑不符(未实现-需重做)**:原返回 4 栅格元组,src 按 index 取一。draw_path 实现时必须按原签名重写。
2. **浮点加法序微差(建议修正,非阻断)**:add_point 与 draw_raster 续画距离 `((dz²+dy²)+dx²)` vs src `((dy²+dx²)+dz²)`,只影响最后一位舍入;src 注释"Native y*y+x*x, then z*z"与 fresh 反编译不符。
3. **rasterize_stroke 结构偏差(行为等价)**:原版单步 `blend_into<closure>` 直混目标缓冲;src 先 `render_into(None)` 覆盖度进零缓冲再逐像素混合。二者依赖 zeno 同一覆盖度计算,像素结果一致(判等成立),但调用链不同——若后续换 zeno 版本需复核。
4. **Vec 布局 ABI 注记(非业务分歧)**:二进制 Vec 字段序={cap,ptr,len}(`new` 0x140a15530 与 all 读取点互证),src 用 std {ptr,cap,len}。行为等价(空 Vec 永不解引用),仅库内 ABI 记录。
5. **write/would_write_succeed 的 None 检测**:原用 Vec cap 字段判 None(src 用 ptr 判空);对空 Some Vec(实际保存流恒为 0x40000 字节)有边界差异,无实际影响。

## 证据

- planning/logic-worldraster/decomp/: b01-core(10)、b02-stroke-paths(7)、b03-defs-sys(7)、b04-draw-raster(8)、b05-verify(retain/iter_points_xz 机器码)、b06-last(last/index_mut/fmt)。
- 只读 --worker shared;未修改 src。
