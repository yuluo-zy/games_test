# 只读补丁应用目录勘误

V1/V2漏列patch应用cwd，这是交付元数据缺项；冻结源码、候选hash、source manifest和两个入口都不改。所有补丁均按-p1解释a/b前缀后，必须使用下表绝对cwd；不能把10个patch假设在同一cwd应用。root首次整批check因此被挡，未写source；其旧source测试log明确排除本轮候选验收。root随后逐cwd preflight/apply，且逐source after hash验证全部精确，记录为 `/private/tmp/tg-validator10-integration-20261021.json`。

|生产source|去除b/后的patch路径|应用cwd|
|---|---|---|
|`tiny-glade/crates/utils/src/geometry/circle.rs`|`tiny-glade/crates/utils/src/geometry/circle.rs`|`/Users/yuluo/Desktop/game/games_test`|
|`tiny-glade/crates/systems/decorator/src/cull_oob_decorators.rs`|`crates/systems/decorator/src/cull_oob_decorators.rs`|`/Users/yuluo/Desktop/game/games_test/tiny-glade`|
|`tiny-glade/crates/country-core/src/utils/debug_viz.rs`|`tiny-glade/crates/country-core/src/utils/debug_viz.rs`|`/Users/yuluo/Desktop/game/games_test`|
|`tiny-glade/crates/country-core/src/resources/walls/decorator_outline.rs`|`tiny-glade/crates/country-core/src/resources/walls/decorator_outline.rs`|`/Users/yuluo/Desktop/game/games_test`|
|`tiny-glade/crates/systems/decorator/src/audio/op_preview_audio.rs`|`tiny-glade/crates/systems/decorator/src/audio/op_preview_audio.rs`|`/Users/yuluo/Desktop/game/games_test`|
|`tiny-glade/crates/systems/decorator/src/audio/op_stairs_audio.rs`|`tiny-glade/crates/systems/decorator/src/audio/op_stairs_audio.rs`|`/Users/yuluo/Desktop/game/games_test`|
|`tiny-glade/crates/systems/decorator/src/ui_systems/ui_place_decorator.rs`|`tiny-glade/crates/systems/decorator/src/ui_systems/ui_place_decorator.rs`|`/Users/yuluo/Desktop/game/games_test`|
|`tiny-glade/crates/systems/decorator/src/ui_systems/ui_decorator_affordance_dispatch.rs`|`crates/systems/decorator/src/ui_systems/ui_decorator_affordance_dispatch.rs`|`/Users/yuluo/Desktop/game/games_test/tiny-glade`|
|`tiny-glade/crates/systems/decorator/src/blueprint.rs`|`crates/systems/decorator/src/blueprint.rs`|`/Users/yuluo/Desktop/game/games_test/tiny-glade`|
|`tiny-glade/crates/tiny-glade/src/decorator_plugin.rs`|`crates/tiny-glade/src/decorator_plugin.rs`|`/Users/yuluo/Desktop/game/games_test/tiny-glade`|

实际候选测试由root在全部10项after验证后启动，日志`/private/tmp/tg-validator-closure-tests-v2-20261021.log`；此处仅记录运行身份，不先声称编译/测试通过。host不挂载的前提不变。
