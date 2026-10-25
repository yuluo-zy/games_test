# 同OnEnter mesh-only批次独立审计

本报告基于当前真实tiny-glade/src/run_game.rs的OnEnter tuple；不是说所有模块源码都空，而是设备App仅调用`*_meshes`，没有挂其完整startup。grep实际App不存在generic startup_critters/startup_roof/startup_sheep/spawn_brush_previews调用。shader库、mesh库或暖机成功不等于原drawables/animation副作用完成。

|原startup|PDB作者函数范围（完整code_size，含原异常清理）|当前实际入口|缺失/候选|
|---|---|---|---|
|glade_startup_walls|140a92670..140a92f80，2320B|startup_walls_meshes|已交candidates-startup，18index half_cube+2shader+7真实drawable|
|startup_sky_and_sun|140864430..1408646ac，636B|startup_sky_and_sun_meshes|已交candidates-sky，1shader+1真实drawable，translation(-0,-2,-0)|
|startup_critters|1409d49a0..1409d5e91，5361B|startup_critters_meshes|5shader及4真实spawn、实际component/branch/audio；当前generic配对标I，不能整挂视为恢复|
|startup_roof|1414547d0..141454e60，1680B|startup_roof_meshes|2shader+2纯DrawableMeshBundle；原配对顺序已独立LLVM证实，交roof owner协调|
|startup_sheep|140a093e0..140a0b33f，8031B|startup_sheep_meshes|完整动画Mesh序列/属性拼接、shader、音频Pitch等宿主副作用；非仅five mesh+two shader|
|spawn_brush_previews|140924b40..140926be9，8361B|spawn_brush_previews_meshes|17command完整marker/transform/mesh映射；只有11mesh表，且文档51KB不是原作者实际8361B|

PDB范围来自evidence/country-core-audit/data/procs.tsv，不能用相邻public地址当边界。这里只列当前显式mesh-only批次；clearing/theme/water/terrain/grass等其它设备专项各自owner负责，不归入上述表推测已完成。

## sky原证

`sky-startup-original.asm`与`sky-startup-native-policy.json`。原PDB commandexecutor1412dd460只含DrawableMeshBundle；旧文档所谓其它未知bundle字段实际是Bevy command/errorhandler metadata，不是需要捏造的Sky组件。原只一实体，无额外sun实体、无OC/DontCastShadow/KeepOnLevelUnload/指定pass。Transformdefault调用140c9d180保原Quat与scale1，再覆盖translation(-0,-2,-0)。第一个nativefixture预期Y负零被实际原输出-2纠正，修正后完整函数fixture通过。

候选 `candidates-sky` manifest固定1file，rustfmt解析通过，未Cargo/typecheck，CPU单测准备实际build_draw_lists Forward1+Shadow1、sort_key10000000；物理meshVAO、纹理c2074e99b3c51eee、真正drawcallback执行另需设备验收。Env60当前已恢复callback，set_cull8→bind_disk(c2074e99b3c51eee,ce8d8d6a107ec5a6)→draw，无额外World Sky资源读取。

## roof已经独立证实的窄边界

`roof-startup-original.asm`：loadspire→registerspire→registerspireShader→spawnidentity pureDrawable；loadbeam→registerbeam→registerbeamShader→spawnidentity pureDrawable；最后load/registerbutton、load/registerscribble。旧generic接口先四mesh再两shader再spawn，调用/失败副作用顺序与原不一致。两条executor均1412dd460，不带NaniInstanceDesc/OC等；SSBO实际由drawcallbacks读取，不能造每屋顶全局代理替代。该startup资源文件原整体由roof owner协调，未在此候选覆盖其其它资源/geometry。
