# W4/W5 最近阻断：canonical RockyTerrain producer

本文件是给当前 foreign core resources/systems owner 的精确增量合同。本任务没有改其生产源码，也没有注册第二份同名 SSBO。

## 身份 / 启动调用

- 原 W4 TypeId `fa44538b95ae76c6 / 12146c4ea814069b` 是 **RockyTerrainMeshVertCount**，由 `Commands::insert_resource<RockyTerrainMeshVertCount>`@14205e980 的原立即数映射证实。不是当前虚构 RockyTerrainMeshHead 或当前含CPU Vec的 RockyTerrainMesh 对象。
- 原 `RockyTerrainMesh::load@1408bbf50` 完整1292B，成功 `eax=0`、`rdx=rsi`@1408bc2d0/2d2；rsi一直为原解析 vertices.len。调用者 `startup_terrain@140926bf0/5382B` 在 `14092706c` 用固定路径 `assets/data/rocky_terrain.json` 调用，成功在 `1409270a9` 将返回rdx放入真实8B Commands payload；`1412d7aa0`的PDB完整类型就是该VertCount。
- W4 `displace_rocky_terrain@140a3ee80` 读同owner首qword，除3，再截u32作为push/dispatch线程数；W5图中的 `rocky_terrain_shader@1409cb4e0` 读同owner低u32作为draw_no_index_buffer的vertex_count。两者是同一个顶点数，不能再给W4传triangle_count/3。
- 上述 native 与 producer 原call/RNE：本目录 `1408bbf50-full-rne.asm`、`140926bf0-full-rne.asm`、`rocky-loader-caller-audit.json`、`resource-id-producer-mapping.json`。本次查到真实startup producer，不宣称当前只含grid helper的startup_terrain.rs已实现完整5382B。

## 实际发行 JSON

文件 `ljxsj_92385/Tiny Glade/assets/data/rocky_terrain.json` SHA256 `0f63a7250df5a3382017dbdc3652748f33899750e3a65c5aef3727415b9183b7`。

```rust
#[derive(Deserialize)]
struct LoadedRockyTerrainVertex {
    position: [f32; 3], cell_bby: f32, cell_id: u32,
    dir_to_centroid: [f32; 3], is_corner: u32,
}
#[derive(Deserialize)]
struct LoadedRockyTerrainTriangle { neighbours: [i32; 3], is_top: u32 }
```

148794个vertices，49598个triangles。原source DTO恰36B/16B。现extra:[f32;6]、indices:[u32;4]无法反序列化该实际资产，不能用测试自己生成的假JSON证明loader完整。

## 原GPU转换（不是字段顺序透传）

`1408bc090..0d4` 每36B源转48B目标：

```text
GPU words0..2  = position (f32 bits)
GPU word3     = cell_bby (f32 bits)
GPU words4..6 = position AGAIN (f32 bits)
GPU word7     = cell_id (u32)
GPU words8..10= dir_to_centroid (f32 bits)
GPU word11    = is_corner (u32)
```

原先position+extra顺序透传少了第二份position且误读整数。`1408bc1b0..238` 每16B源转32B目标：

```text
GPU words0..2 = neighbours i32 as raw u32 (preserve negative bits)
GPU word3    = is_top
GPU words4..7= ZERO (explicit xorps+movups in original, this zero is proven)
```

两类真实TypedStorageBuffer，Name分别为`Rocky Terrain Vertices` / `Rocky Terrain Triangles`。

## 原buffer ctor合同

完整原PDB范围 `141611ba0-full-rne.asm`579B 与 `1416144a0-full-rne.asm`531B：

| type | record bytes | fixed elements | buffer bytes | usage | memory | align |
|---|---:|---:|---:|---:|---:|---:|
|VertexSSBO|48|148794|0x6cfae0 = 7142112|0x22|1|4|
|TriangleSSBO|32|49598|0x1837c0 = 1587136|0x22|1|4|

Some(bytes)校验整record、4B对齐、len < (bufferbytes + recordbytes)，即可以恰容量。原实际producer总传两类完整数据，不应MAPPED_MEMORY_READBACK/BUFFER_LEN0。现通用TypedStorageBuffer若加了额外用途，需按已重构库当前API为这两类恢复精确专用ctor；不可按原ABI重建底层库。

## 最窄typed消费者接口

当前架构需真实 `RockyTerrainMeshVertCount(pub usize)` ECS owner（原8B payload，不投影CPU Vec布局）。`RockyTerrainMesh::load` 成功返回真实vertices.len；startup只将该值Commands插入。W4直接借该typed count并传给共享恢复body，W5用其低u32。`pass_kit`中RockyTerrainVertexBuf/TriangleBuf当前foreign_carrier是第二身份，会与真实库type_assert冲突，应统一为canonical resources::rocky_terrain两类的reexport并为其实现现graph trait；draw_shaders ssbo_tags同样取该真实NAME/库对象，不能登记第二个ZST假buffer。

尚待对应owner整合：真实JSON DTO+packer+非零容量ctor+VertCount资源、完整startup_terrain producer。没有这些W4/W5不能算连续生产链闭合。
