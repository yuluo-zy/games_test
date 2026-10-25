# 树光标 UI 原闭包合同（待前置接口协调与独立复核）

## 身份与完整证据

原B RNE/PDB绑定同native/report.json，ui_place_tree1441 B、其closure584 B全部范围已字节校验。closure@142269720通过vtable143035160唤醒；生产 SignifierFactory::from_closure / UiSignifierStream::Factory 已提供安全 Arc 所有权，复用现API即可，禁止Box::leak或保存原地址vtable指针。

## 真实数据流

- ui_place_tree@142269355捕获 CursorPositionSS Vec2；@9364取WindowSize::as_vec2返回y；@936f取GameConfig.ui.icon_size_mult。
- 名称保留 `format!("decorator_{ss}")`，原格式指针143035150；坐标变化时名字变化，由既有signifier_create去重和mark_for_delete回收旧实体。
- 活跃界内时写工厂；点击与界外反馈的既有控制流不改变。
- 闭包先创建 SignifierEntityBuilder(Empty).animation_duration(0).build(meshes,shaders,commands)，然后给所得Entity插入(Ui2dExtent,Ui2dImage)；返回单Entity Vec。无UiDrawOrder、无鼠标可交互标记、无额外树预览状态。
- 图像为原16字节 `seed_cursor_icon`，layer0。
- 尺寸：k=height/1000；scaled=Vec2(28.4,26.4)*k*icon_mult；size每lane用max(scaled,Vec2(35.5,33))并在scaled NaN时选最小值。原@142269819..855使用单精度逐乘，禁止合并乘法为先算icon_mult*k后再乘，避免舍入不同。
- 位置：Vec2(ss.x,ss.y)+Vec2(size.x,0*size.y)*(-.5)，保留原0*Inf生成NaN的异常行为；普通输入即x居中、y原样。@142269858..87f。

## 现有consumer真实状态与前置缺口

- country-core::signifier_create 已唤醒Factory closure、为返回Entity赋SignifierId；活动集合供mark_for_delete/despawn。
- tiny-glade::world_postprocess::collect_ui_2d 已读取Ui2dExtent/Ui2dImage（无需Transform假数据），主UI pass按图像名解析texture。
- 当前 country-core/signifier_factory.rs 的 build 是只spawn+SignifierAnimation的显式GAP壳。原Empty分支仍须注册Empty Mesh、真实DrawableMeshBundle、vertex_color_shader严格查找、DontCastShadow/UiPass及TransientMesh回收，不可绕过它只造Ui2d组件声称闭环。
- 原source task TG-f99e87e1f1b232d48a (signifiers/mod.rs，PDB486) live NULL；shared core扩展需root协调。已有 round12/signifier-builder-full/builder.rs 是完整5分支候选，仍应核当前before与原字节再接入，不能直接照搬覆写现factory重构。

## 最窄树候选计划

前置安全builder接口就绪后，仅在当前ui_place_tree.rs把raw32B payload和绝对marker换为owned from_closure。用现Ui2dExtent/Ui2dImage、原name及size算法，恢复seed_cursor_icon私有helper而非保留当前空函数。复用producer候选before，提供第二阶段配对patch与SHA。

验收：工厂确实has_closure，wake后完整drawable和Ui2d图像组件存在；尺寸覆盖最小/高分辨率/icon scale/NaN；同名字请求仅唤醒一次，鼠标变化旧图标被回收、TransientMesh receipt注销；drop全部stream和factory后捕获Arc无泄漏。root执行编译/必要GPU，child不运行共享编译。
