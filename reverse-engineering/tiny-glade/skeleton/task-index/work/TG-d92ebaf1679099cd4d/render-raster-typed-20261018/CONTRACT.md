# W1 真实 SSBO 泛型身份修复

当前共享工程的 B 恢复接口；本变更不声称 C shaderface/rhapsody2 业务等价。C 文件确实存在且 source checksum 与 B 不同；六个 Raster 上传入口保留，但其原编译调用有 ToShaderPermutationKey，不能把 B URI 实现扩称 C 已验证。

## 原证据

B RNE SHA f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03，PDB SHA 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911。B PDB六个 read_storage_buffer 单态实例的完整泛型类型均是 resources::world_raster_defs::*RasterStagingSsbo，地址与完整名称在 B-native-staging-generics.tsv。它们支持真实类型身份，不自行证明所有字段布局。

## 变更

原恢复的 upload_world_raster 声明了独立零大小 *StagingBuf 并以 ssbo_carrier 假定 BUFFER_LEN=1；真实 producer 为 world_raster_defs 同名 u8，512*512 元素，MappedStorageBuffer。库按完整 type_name 校验，拥有相同 NAME 的另一 Rust 类型仍会失败。现在六个 *StagingBuf 直接 re-export producer 的既有类型，不新建GPU数据，不改变格式选择/标志变更/dispatch顺序。

## 未闭合

RasterFlags repr(C) 是原字段访问 view；当前 Bevy WorldRaster 的 source-build 布局不同，不得强转。已新增 upload_live_world_raster<R:LiveRasterKind> 接真实 Bevy Mut<R>，pending读取 bypass_change_detection，格式校验之后依次 set_changed 与 mark_staging_uploaded，再用同一 declare_upload 算法构建pass。六个真实命名资源各有唯一关联 View/Staging，不能选错carrier。与后续 reupload_pending()/consume_reupload() 共用真实资源。没有生产 FramePasses 使用它前不计世界W1链已闭合。

## 验证

rustfmt 解析通过；cargo 与真实GPU由根整合owner统一执行，未取得结果前为 implemented_unverified。现有 upload_world_raster 单元测试会使用同一真实mapped类型，不能为测试注册原假carrier。

## Fresh 原指令与六资源身份

本轮直接由 LLVM 对发行 RNE 两个有界范围反汇编，shasum再次核对为上述B SHA，未使用patched文本作指令oracle。upload-path-B-rne.asm：1403d8c7a读取+29a、c95..ca9格式校验、1403d8cd7写changed_tick、1403d8ce3写word+29a=0100。copy-to-staging-B-rne.asm：140a1767c写+29a=1；这证明原published与上传后置位reupload的关系。

| actual resources::world_raster_defs类型（同名View） | original TypeId lo / hi | original preset |
|---|---|---|
| PathRaster | 85c987d23d58d2de / 2ec6ab231f77203b | 1 |
| Cobblestone3Raster | 6dc90dcfd23786fa / ed952bd348f66c04 | 2 |
| WaterRaster | 1ad182936286a0ee / ac04e3b6f9127e6f | 3 |
| GardenRaster | 979ffa6288aa259a / 09791d5b46ed449d | 4 |
| Cobblestone1Raster | bb8761bcc7907109 / dc4ef498cea8673a | 0x13 |
| Cobblestone2Raster | 86486f3e82eb6bbe / 4d7091c2f9bfa189 | 0x14 |

主FramePasses adapter应在对应slot从ECS取得该真实named resource的Mut，然后调用live入口；六个mandatory asset库仍由FrameWorldAccess原身份提供。不能为通过检查插入默认RasterFlags。

新增 CPU业务回归 live_ecs_producer_and_upload_consumer_share_flags_and_precise_change_tick：真实copy_to_staging生产published；pending为空不改tick；错误format在tick和flag修改之前失败；合法上传才改tick/flags并登记pass。此回归尚待根统一cargo执行，不作为已通过结论。
