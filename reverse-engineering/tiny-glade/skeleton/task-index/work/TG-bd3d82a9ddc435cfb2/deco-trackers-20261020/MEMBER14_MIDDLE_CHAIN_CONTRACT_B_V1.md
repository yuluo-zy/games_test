# Member14 Brick assembly：B 原证、现 host 与接入边界 V1

目标是两真实 per-wall assembly；本 V1 是继续恢复的合同，不是完整几何/音频/系统注册已完成的声明。冻结 tracker 三源及8 tests结果保持原范围。无生产修改、Cargo、benchmark、GPU 或 Git 操作。B original RNE f507a6ce...、PDB 2d9686b8...；C 仅索引，不据此宣布等价。

## 1. 原 allocation 的创建与读取已闭合

原 `publish_public_wall_state@140a8b530/18515B` 全包复用 TG-3594e8acede6203727/main-chain-boot-20261018/original-publisher-rne.asm。3749 指令、18515 连续 bytes 对原 RNE 全核零不符、零 gap，见 MEMBER14_PUBLISHER_REUSE_B.json。不是从现重写字段反推原布局。

140a8ce7d 从真实 AssetSsboLibrary 取得并按全类型名核 BrickTransformSsbo；接着建四个 32B empty Allocation（Vec {0,4,0}、mutex0），各 Weak 指向该同 storage.waste。只第三/第四已绑定本消费链：

| 业务 | 分配 call | PublicWallState value 内 Weak/handle | withdiff hash bucket（含8B key）handle | 读取 |
|---|---|---|---|---|
| Chimney |140a8cff1|+158/+160|+168|1412aea80|
| DoorStairs |140a8d078|+168/+170|+178|1412af6e0|

第三复制到临时 record@rbp+500 的658/660（140a8da23/31），第四668/670（140a8da3f/4d）；140a8dd06..dd22复制2a0记录。前两个 Brick allocations 的业务标签本链未证，不命名、不映射、不删除。另三 Nani allocations 不属于这两个 Brick lanes。

**原两 lane 在 publication 中先创建，assembly 只取非空 handles。** 缺 handle 在两 withdiff 的1412af1a3/1412afa3f进入 panic，不是 PublicWalls miss。只有真实不存在 PublicWalls 的分支可按原闭包选择无current重建、明确消费removals；有墙但现Entity/InstancedWall/handle缺失不能归入该分支。

## 2. Params 到原构造入口

canonical Params仍为40/32B准确carrier，比较使用已合 native ordered-f32语义；不derive位Eq、不补Default。现共享 utils::random_splits/into/xz_rotation_bad 已 root 整合，lib.rs SHA feaaee386a98f4551009dca584c63b3e1b680db0bed21d1c76bb33a7bf5e5fd6；共享7 tests当时仅source审，不能写成已跑。初始化每 DecoId 的rng为 `Rng::with_seed(utils::rng::init_rng(id))` 恰好一次；fastrand1.8 with_seed不会额外advance/mix。

### Chimney 当前项（1412aebee..aef40）

跳过 Removed/poison，其余逐项使用新 current 参数。读取+8/+c为XZ中心，+10/+14为两垂直端，+18/+1c为XZ方向，+20为原 source id。原完整 constructor `14200bcd0/2836B` 及每次调用已存在 companion native-host-B。

每current先调一次主体：XMM0/1=+8/+c、XMM2=+14、XMM3=+10、stack+20/+28=0.5/0.5、+30=0.3。然后按参数+0 tag 决定额外构造：

- tag1：两端 +10 与(+10+1.6)，尺寸0.2/0.2，delta0.1；不能先把两端排序。
- tag0：真 TerrainHeightsData 对原两个XZ端点采样并以原 min/NaN次序确定另一端；尺寸1.3/1.0，delta−0.3。端点常量向量的精确表达继续在原寄存器链中核，不用包围盒/固定地面代替。
- 其他tag：只有+4严格大于0.1（unordered不进入）时追加；两端+10与(+10−+4)，尺寸0.8/0.8，delta0。

每次调用传同rng、同writer和同累计failed字节；同outer WallId供逐砖seed，参数+20供BrickSourceId::from_wall_id编码。该观察区分两id，不把低32位来源编码当全局资源身份。

### Door 当前项（1412af84f..8f7）

helper `14200c7f0/4668B`：XMM0/1 = 参数+0/+4，XMM2/3=+10/+14；stack+20=+1c、+28=+18、+30=0.56、+38同rng、+40/+48=+8/+c；同writer+failed捕获。全32B八个f32均实际进入此路径，不能仍使用40B门梯统一载荷。helper宽度/方向/深度的最终安全Rust参数命名须遵从下面原运算，不能由字段名称猜。

两模块 PDB3087/3088已只读定界dump（constructor-PDB-B）；S_GPROC32是placeholder `void()`，无原字段声明/参数类型，不据此声称恢复原Rust签名。

## 3. 原几何算法到 canonical96B payload

两 helper 都直接调用 `SparseInstanceBufferWriter::push<BrickTransformSsbo>@14128cdd0/784B`。共同clear过程的单个RoofTile PDB alias不能推翻实际push类型。使用现canonical BrickTransformSsbo，不造raw96B第二载荷、第二typedbuffer或新BufferHost。

Chimney 已核的运算：

1. `height = endpoint2 - endpoint3`；`n=max((ceil(height/0.4) as usize),3)`；random_splits(n,0.2/height,rng)。Rust float→usize原为饱和转换，不私加固定max。规范化t按原乘加顺序映射到两垂直端；原随后collect成第二Vec，内存失败保持Rust分配失败语义。
2. 相邻端点逐层；layer_index/(n−1)经 `s=clamp((t−0.85)/0.149999976,0,1)`、`s*s*(3−2*s)`，分别乘delta后加两个基础尺寸；Rectangle2d::new复用共享已恢复finite/单位轴guards。
3. 每层四条边，取point(i),point((i+1)&3)，单位边方向×0.3×0.5再乘交错符号（首层−1、次层+1）；两个端点同偏移。无随机替代几何。
4. 一次rng.f32生成厚度 `0.3 + (r*0.05 - (1-r)*0.05)`；get_edge_normal→共享xz_rotation_bad；scale是边长/层高/厚度，translation是边中点XZ/两层中点Y。原compute_affine→Affine3Packed::from构成前48B。
5. 其余由原BrickTransformSsbo::default提供，包括surface[-100;3]；seed为outer WallId低32位加layer+edge（wrapping）；material为参数source id编码；wall_distance为跨层连续累计的边起止距离；flags保原default。不能每层擅自把累计距离清零。

Door 已核的算法骨架：先 `max(ceil((end2−end3+extra)/0.2) as usize,2)` 的随机垂直分割，jitter=0.0400000028/(end2−end3+extra)；映射到由end2向(end3−extra)的原顺序。另层数上界来自 `(ceil((end2−end3)/0.2) as usize).clamp(2,5)`，这是原局部业务clamp，不能外推为全帧max。横向分割以ceil(width/0.45)为基础，每个层/step重取random_splits；宽度映射到[−width/2,+width/2]；两个push分支分别在14200d263、14200d5a1。完整point/scale/seed/side brick表达仍需逐寄存器核闭，**此V1不批准把骨架实现成完整Door几何**。

原所有float常量16B窗口与实际使用指令已冻 MEMBER14_CONSTRUCTOR_CONSTANTS_B.json；strings/pointers窗口不是float合同。Ghidra仅导航，精确行为以两原RNE ASM为准。

## 4. per-wall clear、容量失败及日志归一化

两host取各自allocation→锁chunk ids→拼同Brick writer→clear（Chimney1412aeb16、Door1412af77e）→遍历全部current逐砖push。不能把全部current复制到新的registry或每帧克隆整表；dirty wall才重建，writer/guard词法作用域不延长。

现 Sparse writer.push 返回Result<(),NoFreeChunks>；原返回AL非零是NoFreeChunks失败。每次失败设置本wall的累计flag，继续其余砖；已经写入的新砖保留、旧lane已清除。现base upload.replace_all失败保旧的合同不能套进两deco lanes。

额外精查：两完整主host仅初始化failed（+117/+c7）并向helper传其地址；当前证据未发现读取或根据flag发UiSignifierStream。原系统参数保有该资源并不证明有输出。禁止发明容量UI、清history或自动retry。随后原with_removed仍执行，normalize Current/Added/Changed并删除Removed；dirty集合已排空，所以容量partial不会自身重新入dirty。设备上传失败则走现共享try_gpu_flush保dirty重试，二者不是同一种失败。

Chimney current音频为原tag转换条件的kind19/20、position=+8/+10/+c；真实Removed callback1412681c0按DecoId查真实DerivedDecoratorInfo，只有原decorator tag0x14才发kind0x15事件，随后删除payload。Changed不是Removed callback，不能给Changed统一播放删除音。Door真with_removed闭包无audio。PublicWalls miss可明确选各自已证no-op removal分支，但不能把真实存在的Chimney Removed无声空化。

## 5. 现有同owner写入、上传、绘制路径

源码哈希/行范围在 MEMBER14_MIDDLE_CHAIN_MANIFEST_B_V1.json。现真实路径：

`PublicWalls CPU state + WallEntities → Entity的InstancedWall<BrickTransformSsbo> → 同AssetSsboLibrary.get_mut<BrickTransformSsbo> → Sparse writer CPU shadow/active/dirty → PostUpdate AssetMaintenance::Geometry sync_wall_bricks → take_out_waste + try_gpu_flush → world_graph W21/W27 world_walls → 真draw ranges/visibility/indirect counts → LOD wall draw callbacks`

`construction/bricks.rs`只重导出canonicalBrickTransform为WallBrickInstance，不是另一格式。现library.get为共享ref/get_mut为独占ref；安全adapter需要实际ResMut library与QueryMut owner。原PDB Res<lib>不授权给共享ref转mut，不添UnsafeCell/unsafeSync。所有lane同typedstorage，同dirtybits，同devicebuffer，同frameflush；不新建SSBO库/设备proxy。

提案仍是**现架构有界映射**：同唯一WallEntities→Entity→InstancedWall持有base/chimney/door三个owned lanes。只适配本链两个已证独立allocations；不声称重造原PublicWallState全部allocation布局。base更新只改base，deco rebuild只clear其lane。每个handle均验证belongs-to同storage；droppingowner回既有waste，不能单独在tracker持GPU所有权。

### 真实drawlist和highlight

新增两原RNE包 member14-draw-B：14098aaf0/1961、14098b2a0/3257。两者仅查询highlight entity的InstancedWall单+20allocation，to_ranges→hide→global normal ranges→show。Chimney/Door的PublicWalls独立alloc不参加此单handle排除。正常无highlight时从同storage activebitmap取全部ranges。

因此装饰lanes仍经同active全局normal可见；highlight仅base，不把deco无证合并到basehighlight。当前world_walls正是该关系。W21clear Next，读Prev/writeNext；W27reset原间接counts、clearPrev、读Next+HZB/writePrev。两轮都实际读canonicalBrickTransforms，Shader写真drawlist/counts，无CPU countreadback或固定max。

### 删除与设备失败

当前delete_wall移除同WallEntities关联并commands.despawn真实Entity；deferred命令真正应用后component的全部owned handles Drop进入同waste；PostUpdate take_out_waste回收active/count/freechunks再统一flush。tracker.on_wall_removed只丢log+dirty，不成为第二回收owner。删单个Deco只dirty该wall并重建其lane，不despawn墙。

现raster砖callback以WallBrickUploadStatus.gpu_ready拦设备失败；W21/W27 compute当前无同statusgate，不能声称整帧failstop。owner源码接入必须显式解决/限定这一现有边界，不能靠本合同外推GPU一致性。

## 6. 源授权、初始化与调度的必要边界

live官方task_queue.show原JSON见member14-live-show-v1；记录的是查询时锁，源候选前须再核：

- InstancedWall TG-a738caa535351a7991：frameinputs-20261018持锁；Library TG-3f1019bf28c1d470d9：render-ssbo-lease-20261018；drawlist TG-5f034e6af1260241e4：render-nani-main-20261018。
- 原construct_walls TG-1a3550f643b9278be7：main-chain-frame-20261018；publisher TG-3594e8acede6203727：main-chain-boot-20261018。
- Sparse TG-f4e8ea3c0995e850c0、delete TG-8e89b19e5e8416b140、PublicWalls TG-2f6aeeca0bb0e76a93、PublicWallState TG-58352a844a7f599600 live为null；null不表示本agent已取得。
- 两assembly TG-917d... / TG-e108...属于deco-trackers-20261020；utils TG-1d8...属于04 cloth-visual-state。
- 新geometry/runtime.rs、tiny-glade/world_walls.rs、core/construct_wall_bricks.rs为现glue无独立native任务；root需按实际所有权明确委派，不从旧work目录推当前claim。

**初始化待接入的实质问题**：current InstancedWall::new只建立pending baseVec，allocation=None；component通过construct_wall_bricks的Commands在成功真实构建后产生，baseallocation又延至PostUpdate sync。member14在Update消费，所以不能假定有墙→该handle已存在。原publication已建allocation并不等于现adapter已满足。需root协调同WallEntities真实producer与Commands apply、同library前置注册/初始化，令所有新/更新正常路径在assembly前具有真实3lane owner；墙存在但构建失败/Entity缺失必须保明确错误域，不能consumer默认spawn空主体或归入PublicWalls miss丢log。本V1只证明创建者与目标，未宣称这段现适配已实现。

必要顺序是 publication/真实wallproducer及deferred可见→同owner的lane初始化→member14实例化写tracker→两真实assembly clear/push/normalize→PostUpdate同storage flush→W21/W27。初次new、同帧baseupdate、同帧OnWallDeleted、重新加载/存储替换、真实terrain/derivedinfo/audio资源都要在注册前闭合。

## 7. 后续实现与验证边界

先闭Door完整表达/Chimney terrain+audio/current分支，再提交owned两assembly候选；外部owner源码仅root合法协调后候选。不得用protocoladapter或mock几何系统注册替代原7/4param闭包。

有意义的限定验证：同一真实Sparse实现+canonicalParams，覆盖所有Chimney tags/terrain端、Door两个push分支、Changed旧/新与Removed音频、dirtywall只有本laneclear且base不受影响、32边界/NoFreeChunks partial+日志normalize不自动retry；同Entity三handles Drop→take_out_waste；normal/highlight ranges验证decos仍normal。只有源正确定型后root独跑同源harness/core/App可用检查；没有本agent自行执行测试/benchmark/GPU，也没有吞吐或性能倍数声明。
