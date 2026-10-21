# 两窗主链实际宿主合同B

root明确窄委派，TG10dd原NULL正常唯一领取decorator-chain-root-20261021；run_game仍原TGc459/main-chain-boot owner，仅其合法work新增caller窄patch。当前仅候选，未生产写、Cargo、GPU。保留窗口/角窗冻结包，验证补丁另走TG69唯一writer。

## 原绑定

NATIVE_HOST_PROOF_V1.json 对13个完整PDB边界函数逐条原RNE字节核对，覆盖根Plugin17521B、外chain4650B、gen15 tuple2935B、两窗run_unsafe1401/1604B、原cache9331B、corner ctor453B、两次flush共用ctor437B与wrapper328B、墙洞in_set1118B及wrapper495B/真实leaf31B。vtables+50直接原PE值，不凭signature猜系统身份。另一个7参wrapper绑定1407c21b0而非墙窗，明确不挂错。

原Plugin3451af调用19成员chain，3451b4 rawset0x19=25，3451e2 in_set，345253 Update。member2 cache，4 derive，5 clear/visual transient无内部顺序tuple，6十五生成器无内部chain，7 collider-only，8 flush，9 validate，10..13归档/剔除/历史，14含fullvisual/copy链，15unleash，16同flush，17lantern，18UI dispatch。现留存成员保此顺序，不假称未挂的原成员已恢复。源旧helper.after(unleash).after(UI)与原序反向，必须纠正。outer原是chain()，只有内部full/copy原chain_ignore_deferred；不可把outer也ignore而丢Commands可见性。

墙窗绑定：gen529ddc vtable1429f2508，+50=5281b0，5285db直接调用7ebfd0。角窗：gen529df9调用040c40，040da1装vtable14292ce90，+50=03ff20，0402aa直接调用7f66b0。

外chain0d55a8/0d57ec两次调用254d90，254ee2装vtable1429a34a8，+50=252850，2528e0直接调用8d8360。必须保collider-only后一次、unleash后一次；不等待下一帧代替同帧可见性。

墙洞清帧原根Plugin344e0f raw0x0a，344e12调用2957c0，344e1d添加Update；2959b8装vtable1429ab780，+50=2943b0，2944c5直接调7f98c0，7f98da尾调f867c0。set10与set25的先后复用同App run_game add_stages原0..44相邻边（当前345..351行），不得将墙洞清帧并入clear_out_visual_entities。PrevWallHoles已由startup_construction_runtime::install真实init，无第二owner。

## 实际sourceflow与候选

production目前build_recovered_visual_systems仅定义无caller；run_game创建gameplay App只调build_startup_resources，因而即使两窗本体恢复也不进场景。caller候选在同create App已注册global sets/正式owner后，紧跟build_startup_resources调用同一个build_recovered_visual_systems；GameLoop::from_app现接收这个同App，renderer.install_asset_libraries→install_gameplay_device_runtime→PreStartup→首Update继续使用同World。不给device runtime另装第二份。

host候选set10挂真实clear_out_wall_holes；set25保cache→typed derive→clear tuple→{墙窗,角窗,真实generic clutter}无内部顺序group→collider-only→flush→{fullvisual→copy ignore_deferred}，在已注册unleash之前；第二flush在unleash之后、UI之前。未恢复的其它12 generator/validation/编辑/归档/lantern不照旧lib聚合一并挂入。移除旧resource_exists门控：缺真实catalog等正式资源必须暴露失败，不能无限静默skip假装游戏正常。

## 整合硬门

1. TG609四表+真实26loader+required ID subtype producer（corelead唯一writer）成组整合，不能BBox/空catalog/同名第二资源。
2. 墙窗v2/角窗v1+rectangle edge leaf+subdivision v3独审、实际CPU验证；两窗源函数名保持原，改参数由Bevy自动借同正式资源。
3. TG6613 cache及TG280 requests/stairs/stairs_assemble的原真实producer必须闭合。当前roof63/terrain63/dst4返回None且每帧swap_clear，generic接收dst4会required cache失败；不能带假缓存把host挂上。duck已有唯一sourceclaim正恢复，不由此候选擅改。
4. TG69同blueprint.rs缺validate_blueprints：原539B82715c无条件调用顶层storage140a1e720，再在DeferredHistory非空时进入8161Btry_fold原业务。当前方法虽存在，正式生产无调用。validator/8KB callback及UI473B子闭包未恢复前，不把这个host称编辑/长稳闭环。root已转委派我下一有界增量在原TG69work，claim保持。

host可读/可审，caller补丁不能提前越过cache与validation硬门并宣称完整；原GPU30帧/交互/保存重启仍root独占验证。只读工具成功、rustfmt parse与候选文件数不计作场景运行通过。
