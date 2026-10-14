# startup-components 域 HIGHLIGHTS（2026-10-14，owner tg_sc1）

1. **138/138 业务函数全部 O**：components+startup 38 文件无一处空白；MODULE_GAP_ANALYSIS §1 的两大缺口
   （components 101KiB + startup 115KiB）已全部取证（复用 expansion-b1/b2 既有 decomp + rne-read 补证）。
2. **组件域 = Bevy 注册胶水为主**：~58 个 `register_required_components`（18KB 单态模板，bevy_ecs 内部）+ 9 个
   `clone_behavior`；唯一真实业务系统 = **`cache_prev_object_transforms` 0x14082bd80**（逐帧缓存实例变换到 PrevTransform）。
3. **clone_behavior 双实现族（纠错）**：默认 0x140002370 = `xor eax,eax;ret`（返回 0=无克隆）；每组件变体
   = `lea rdx,[类型表]; mov eax,2;ret`（返回 2）。PDB 只给方法名，机器码区分，勿混用。
4. **transient_mesh 注册表布局与 reconstruction 不符**：二进制 = `{Mutex@+0x0, Vec@+0x8}`（Mutex 与 Vec 分离），
   reconstruction 写 `Mutex<Vec>`；`new_with_registry` 内部 Arc clone。建议对齐。
5. **`load_prefabs_impl` 0x1412a1df0（154KB）= 网格资产生产者源头**：读 assets/prefabs，属性
   appear_pos/idappear_pos/prim_center/prim_normals/Vertex_Normal/Vertex_Position，即 68 个
   `load_json_as_mesh` 调用方的上游；panic 串 `Unknown mesh code processor`/`Sholud not exist`。
6. **PrefabWatcher = notify 热重载**：new（100ms debounce + channel）+ watch_atlas + 插件 build；错误串
   `Failed to watch file {path:?}!`。
7. **load_mesh_atlases 生成程序化图集**：stone_floor/plaster/plaster_strip/stairs_floor/smoke + `assets/nani_meshes.ron`
   RON 解析 + 地形 grid_gen_cw/ccw；`load_nani_meshes` 两个 closure 变体（初始/reload）。
8. **startup_session 会话生命周期**：起始 build（full/demo/intro）→ 建目录 → meta.json（theme:
   Summer/Olden/Flowery/Autumn/Winter）读写 → history.json/snapshots/screenshot.jpg。
9. **startup 资产清单完备**：shader/网格/纹理名逐函数提取（shaders、brush、sheep、critters、water、terrain、
   ivy/grass/meadow、themes flowery/olden），可用作 reconstruction 资产路径/资源契约的权威输入。
10. **跨域接口**：cache_prev_object_transforms ↔ render/nani sparse instance buffer（0xa0/0x48/0x30 stride）；
    FMOD 事件（Music/Halloween、Sheep/Baah、Butterfly/Shoosh、snapshot:/Loading）→ audio-misc；
    session meta → session-save。GPU 后置规则遵守：只恢复 CPU 侧载入/注册/生成逻辑。
