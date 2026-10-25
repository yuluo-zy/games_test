# TG69 两项必要叶独审

结论：两窄候选通过源码/独立F0审查，允许与完整canonical依赖成组由root验收；不代表完整Dispatcher、validator宿主或音频业务已挂载完成。审者不是作者，live show确认原claim/合法work保持。

TG2a：实际原539B内联140827285..2d6与绑定142a91c00六表对应+69/+5c/+60/+5c/+5c/None，128bit判别只有(6,0)取None。重构结构不能按原偏移读，现源码对真实owned Move(state2,0)/Rotate(state3,0)读request.ty是正确canonical adapter；其余安全ty字段的有效性是各variant producer必须满足的前提，明确尚未闭合，不把默认0当恢复完成。当前set_move/set_rotate完整保该前提；其它place/stair/height作者未恢复typed producer，不可因此mount整体UI。两个新测试覆盖exact clear、非零第二word与owned request不受旧ty覆盖；不构造native union假ABI。

TG10dd：只在真实run_game:405到达的build_startup_resources注册一个canonical MutePreviewDecoratorOpSounds单位Event，沿Bevy App的Events update保留期，未手动clear/复制队列/挂新系统。该type必须同TGc4 mute-guard候选一起集成（当前production没有该type）；OutlineWallCmd已由app_systems:211注册，未重复。其余文件体未改。不能单独只合注册而缺canonical type。原writer/两个reader及完整TG69由render_reviewer另审，本审不外推它们完整。

已重核两份before/candidate/patch/合同SHA及实际生产before，分别在隔离树tiny-glade目录patch -p1 --fuzz=0无offset/fuzz精确得到dd84ef98/ea1ed1fc，未改生产，未Cargo/rustc/GPU。B原包证明仅本叶；target C与parse均不是行为验收。
