# 原 WallLazySubscriber 实际 owner 与调度闭环

目标为B发行RNE SHA256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03；不依赖统计完成度。

`once_startup_walls` 已在原FunctionSystem wrapper内完全内联。PDB `1408c61d0` 613B 三参数按序为 ResMut<TerrainHeightsData<TerrainHeightSsbo>>, ResMut<PathRaster>, Commands。原138/140 component-id槽按此顺序访问并保留Terrain ptr为RDI、Path ptr为RCX；`1408c630a` 调`WorldRaster::subscribe<512>@140a158e0`，结果保存为第1入参RDX；`1408c631b`对Terrain ptr加0x18，`1408c6322`调`Publisher::subscribe<DirtyTiles>@141454290`，结果为第2入参R8；`1408c6339`调`Commands::insert_resource<WallLazySubscriber>@1412e0d00`。因此两路是Path、Terrain，不能反过来，不能另建发布者。

原`publish_public_wall_state@140a8b530`于`140a8bce8`取该ResMut；`140a8bd2b`对第1字段调`iter_dirty_tiles<closure0>@141458b20`；`140a8bdaa`对+8字段调`iter_dirty_tiles<closure1>@141458a50`。两helper分别调用同一`tabloid::Subscriber::take@141454590`，再分别走`DirtyTiles::iter_tiles<closure0>@140adeea0`与`<closure1>@140adf4b0`。现代码的Primary/Secondary次序按此绑定，而不是按名字推测。

生产在现publication模块中合并恢复该原startup业务，canonical资源名为WallLazySubscriber，旧WallPublicationSubscriptions是同身份reexport。PreStartup注册在现真实load_game_profile之后，Commands在首次Update前应用；随后set19在adjust之后、publish之前收取两路batch。Terrain订阅来自生产同一TerrainHeightsData，WallPublicationTerrain仅由该owner派生采样视图，未创造第二个publisher。

原`add_game_startup_systems@140941110`的10项链为PreStartup：LoadAllGameTextures、load_ui_shaders、load_shaders、init_populate_ssbo_library、load_prefabs、init_color_library、load_game_profile、init_glade_border、once_startup_walls、load_audio；该完整10链尚未在当前App完整装配，不能把此单项恢复称作整链完成。链的原helper140a97110、tuple140a2bf40、add_systems<PreStartup>140b67620在boot work有独立原native证据。

新增限定回归由真实WorldRaster.copy_to_staging、真实TerrainHeightsData.copy_from_gpu各自产生dirty，验证Path只入Primary、Terrain只入Secondary。它没有GPU，不伪造readback资源或Created指针；尚待parent Cargo。boot未运行Cargo。
