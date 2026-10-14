# resources/critters.rs

> 文件作用：**小动物组件/资源族：Critter/Firefly/Butterfly/Duck/Wren/BirdSpotManager 等（CPU 状态+FMOD+池）**

函数总计 26：业务 22 / 内联无独立符号 0 / 派生胶水 4

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::critters::impl$12::register_required_components` | 0x140f88c80 | 57 | Bevy 注册胶水 | O：Bevy 组件注册（复用 derive，不计业务） | collision-eco\decomp\decomp-0x140f88c80.txt |
| `country_core::resources::critters::impl$13::register_required_components` | 0x140f89370 | 74 | Bevy 注册胶水 | O：Bevy 组件注册（复用 derive，不计业务） | collision-eco\decomp\decomp-0x140f89370.txt |
| `country_core::resources::critters::impl$14::register_required_components` | 0x140f89a60 | 79 | Bevy 注册胶水 | O：Bevy 组件注册（复用 derive，不计业务） | collision-eco\decomp\decomp-0x140f89a60.txt |
| `country_core::resources::critters::impl$15::clone_behavior` | 0x140f8a840 | 87 | 折叠 stub | O：clone_behavior 折叠 stub（返回 {静态ptr,2}） | collision-eco\decomp\decomp-0x140f8a840.txt |
| `country_core::resources::critters::impl$15::register_required_components` | 0x140f8a150 | 87 | Bevy 注册胶水 | O：Bevy 组件注册（复用 derive，不计业务） | collision-eco\decomp\decomp-0x140f8a150.txt |
| `country_core::resources::critters::impl$20::register_required_components` | 0x140f8a850 | 102 | Bevy 注册胶水 | O：Bevy 组件注册（复用 derive，不计业务） |  |
| `country_core::resources::critters::impl$21::register_required_components` | 0x140f8af40 | 107 | Bevy 注册胶水 | O：Bevy 组件注册（复用 derive，不计业务） | collision-eco\decomp\decomp-0x140f8af40.txt |
| `country_core::resources::critters::impl$22::register_required_components` | 0x140f8b630 | 114 | Bevy 注册胶水 | O：Bevy 组件注册（复用 derive，不计业务） | collision-eco\decomp\decomp-0x140f8b630.txt |
| `country_core::resources::critters::Duck::being_petted` | 0x140002370 | 136 | 折叠 stub | O：ICF 折叠（Duck::being_petted 等多符号共享） | audio-misc\decomp\decomp-0x140002370.txt, camera-time\decomp\decomp-0x140002370.txt, collision-eco\decomp\decomp-0x140002370.txt |
| `country_core::resources::critters::impl$23::clone_behavior` | 0x140002370 | 136 | 折叠 stub | O：ICF 折叠（Duck::being_petted 等多符号共享） | audio-misc\decomp\decomp-0x140002370.txt, camera-time\decomp\decomp-0x140002370.txt, collision-eco\decomp\decomp-0x140002370.txt |
| `country_core::resources::critters::impl$23::register_required_components` | 0x140f8bd20 | 136 | Bevy 注册胶水 | O：Bevy 组件注册（复用 derive，不计业务） | collision-eco\decomp\decomp-0x140f8bd20.txt |
| `country_core::resources::critters::ButterflyAudio::set_position` | 0x140f85ba0 | 144 | 音频 | O：ButterflyAudio::set_position——5B thunk→0x140a57da0 | collision-eco\decomp\decomp-0x140f85ba0.txt |
| `country_core::resources::critters::impl$24::register_required_components` | 0x140f8c410 | 148 | Bevy 注册胶水 | O：Bevy 组件注册（复用 derive，不计业务） | collision-eco\decomp\decomp-0x140f8c410.txt |
| `country_core::resources::critters::impl$25::register_required_components` | 0x140f8cb00 | 169 | Bevy 注册胶水 | O：Bevy 组件注册（复用 derive，不计业务） | collision-eco\decomp\decomp-0x140f8cb00.txt |
| `country_core::resources::critters::impl$27::register_required_components` | 0x140f8d1f0 | 190 | Bevy 注册胶水 | O：Bevy 组件注册（复用 derive，不计业务） | collision-eco\decomp\decomp-0x140f8d1f0.txt |
| `country_core::resources::critters::Duck::new` | 0x140f85bb0 | 208 | 构造 | O：Duck::new——FMOD 事件 event:/SFX/Ducks/{Quack,TakeOff,Land}（wp10 合同） | collision-eco\decomp\decomp-0x140f85bb0.txt |
| `country_core::resources::critters::impl$31::register_required_components` | 0x140f8d8e0 | 259 | Bevy 注册胶水 | O：Bevy 组件注册（复用 derive，不计业务） | collision-eco\decomp\decomp-0x140f8d8e0.txt |
| `country_core::resources::critters::impl$32::register_required_components` | 0x140f8dfd0 | 279 | Bevy 注册胶水 | O：Bevy 组件注册（复用 derive，不计业务） | collision-eco\decomp\decomp-0x140f8dfd0.txt |
| `country_core::resources::critters::BirdSpotManager::gen_id` | 0x140f86330 | 316 | 查询/计算 | O：BirdSpotManager::gen_id——+8 自增返回旧值 | collision-eco\decomp\decomp-0x140f86330.txt |
| `country_core::resources::critters::BirdSpotManager::create_spots` | 0x140f86340 | 322 | 系统:写入 | O：BirdSpotManager::create_spots——池复用/分配（wp10 合同） | collision-eco\decomp\decomp-0x140f86340.txt |
| `country_core::resources::critters::BirdSpotManager::recycle<system_critters::wren::update_roof_bird_spots::closure_env$3>` | 0x1422f86e0 | 329 | 系统:写入 | O：BirdSpotManager::recycle——+0x10 原子借用守卫后池回收 | collision-eco\decomp\decomp-0x1422f86e0.txt |
| `country_core::resources::critters::impl$37::register_required_components` | 0x140f8e6c0 | 336 | Bevy 注册胶水 | O：Bevy 组件注册（复用 derive，不计业务） | collision-eco\decomp\decomp-0x140f8e6c0.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`default`、`drop`、`eq` 等 4 项
