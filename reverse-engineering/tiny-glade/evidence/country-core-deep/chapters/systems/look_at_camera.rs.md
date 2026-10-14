# systems/look_at_camera.rs

> 文件作用：**LookAtCamera 组件驱动**——每帧把带 `LookAtCamera` 组件的实体的 Transform
> 旋转到朝向记录目标（Query (LookAtCamera, Transform) + 共享工具 FUN_140c96d40 提取、
> FUN_140c9ce00 写回 + Mut tick）；组件注册为 Bevy derive 的 required-components 胶水。

函数总计 3：业务 3 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→已证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `impl$0::clone_behavior` | 0x140002370 | 6 | 返回 0（组件克隆行为标志：不克隆） | O：`xor eax,eax; ret` | camera-time\decomp\decomp-0x140002370.txt（hex 手解） |
| `impl$0::register_required_components` | 0x1408756e0 | 6 | **Bevy derive 注册胶水**（非运行时系统）：required-components 递归注册（bevy_ecs-0.16.0 component.rs panic 串 + 0xd0B 组件记录拷贝） | O：derive 胶水实证（骨架建模成系统是错的） | camera-time\decomp\decomp-0x1408756e0.txt |
| `look_at_camera` | 0x140873de0 | 10 | puffin scope；逐实体：FUN_140c96d40 提取目标变换→FUN_140c9ce00 写回实体 Transform（位置+旋转四元数），Mut tick 回写 | O：实证（朝向数学在 utils::camera 共享工具） | camera-time\decomp\decomp-0x140873de0.txt |
