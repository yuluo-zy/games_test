# validator feedback：原 DebugLine / DebugSphere 完整四范围恢复

保留 TG67 原 claim/owner，在合法旧 work 出唯一必要增量。四范围 676/1585/1584/1472 B 全指令原 RNE 字节核对，另恢复三个名字 formatter 271/171/119 B；完整输入和字面量见同 work validator-feedback-root-contract-v1 两份 proof。

## 输入与身份

Sphere 原字段 color/radius/position/line_width/filled_in，最后两字段是现手工 Rust 的 alpha/solid；保留 public 字段避免外部调用断链，新增中文原义说明，Debug 使用原字段名。原名字为整个 DebugSphere 的 Debug 字符串。六种真实网格名保原表。alpha 实际作为非填充球的 DrawTopology::Alt 线宽，绝非透明度。

Line 原名字四段无分隔：点 Vec Debug、colors.is_some bool、DebugColor Debug、line_width f32 Debug。ui_pass/dashed/逐点颜色内容不入名字；不沿用当前重构 hash 改变原去重。保持标准库/glam Debug 作为依赖，不保证跨 Rust 版本任意 NaN 格式字节相同。颜色 None 则复制原枚举 RGB 到每个点，Some 则保所有原颜色，原 show 没有尺寸/有限性门。

## 工厂与副作用

共享 SignifierEntityBuilder 是唯一实体/mesh/animation/pass/lifetime owner，不手写第二 Drawable/SignifierId/TransientMesh。两 factory 都 animation_duration(0)，flag8 仍为 0，故初始 scale ZERO，下一动画帧按原 duration0 到完整大小；不禁用原动画或直接强制 scale ONE。Sphere Named/position/scale(radius) 后 build；filled_in=false 才插 Alt(line_width)，true 不插拓扑额外组件。Named 不领动态 mesh 回收票据。

Line 使用 PointsWithColors owned Vec，ui_pass 按原 pass ^1 接口；shader dashed_line 或 vertex_color_shader。build 完成后先 dashed 时插 DashedLineLength(length*4)，再总插 Alt(line_width)。长度对相邻点做原 f32 dy²+dx²、再 dz²+该和、sqrt，并按边顺序累加；不使用平台 dot3 改变顺序。保 Inf/NaN，不 clamp。

空 Line show 必须排一个工厂；真正 wake 走共享 mesh_from_pts 的原非空断言。空/单点/NaN 不在提交阶段 warn 吞；缺 Named mesh/shader 也不得返回空 Vec 冒充成功。显式 try_show 是现有额外 API，保其校验但成功只调原 show，避免第二工厂业务。

## 接线与验证边界

CollisionOutline::display 476B -> DebugLine/DebugSphere::show -> UiSignifierStream factory -> 真实 signifier_create 去重/唤醒 -> shared Builder -> Drawable/TransientMesh/DrawTopology -> 原 draw callbacks。必要 display 候选尚待独审，本包也不挂装饰宿主。保 Rectangle 两邻函数不扩算法。

12 个源码测试，root 实际执行；本 worker 只 rustfmt parse/格式化，未 Cargo/rustc/GPU。真实空点列失败、缺资源失败、动态 mesh 票据/删除、两球拓扑与动画、格式身份、不匹配颜色提交、非有限值传播都保明确验证断言。
