# Saved loading background：owner与跨App图引用修复

本轮只修改 `loading_graph.rs`、`run_game.rs::transfer_loading_resources` 和 `world_postprocess.rs::run_vermeer` 的直接闭包；没有改rhapsody。

原问题：loading将本帧导入的图索引写入VermeerState.background；handoff只移动这个TexRef，gameplay EffectsHost没有注册同一背景。索引既跨帧又跨App资源命名空间，可能失效或碰巧绑定其它图像。没有存档背景的测试覆盖不到此问题。

修复关系：

1. 原TextureHost上传完成后，立即以get(texture).shared()取得真实原生Arc<Texture>。
2. 一次性core slot没有进入AssetTextureLibrary，不再使用raw host handle；在保留原生Arc后释放slot。新SavedLoadingBackgroundTexture资源持有真实owner。
3. 每个loading图、每个活跃gameplay Vermeer图均从此owner调用既有import_native_texture_owned(access0x22)，取得当前graph实际索引并在EffectsHost注册，刷新本帧state.background。
4. handoff在六资源移动之前验证：Some背景必须有真实owner，from/to GraphicsDeviceResource与texture.device()完全同Arc，目标App不能已有另一个背景owner。失败不提前破坏原App。
5. 成功时同owner与VermeerState一起移入gameplay。原core reveal完成或reset清state.background时，删除额外owner资源；已提交graph保留其自己的Arc直至真实GPU完成。未完成关闭也按World/renderer原有销毁顺序释放。

原None不造背景；Some缺owner为named error，绝不通过删除state.background绕过。图导入底层会再次校验Device身份与资源依赖。RGBA/alpha/格式0x2b/extent/Vermeer颜色与progress算法不变。

新增CPU测试源码：连续两graph索引重新绑定；Some缺owner拒绝且保持原引用；原None不变；handoff缺owner在六资源移动与StartupRevealStarted之前拒绝。它们验证控制/引用契约，不冒充真实GPU owner或像素验收。

中央已报告无背景loading GPU回归通过，新owner实现已参与该次构建；本子agent未运行Cargo或GPU。包含真实saved loading_background.jpg的生产启动/handoff分支仍需中央实跑验收。
