# Member14 six-source B candidate V1

Root明确增量委派TGa738/frameinputs、TG1a355/main-chain-frame、TGc459/main-chain-boot及runtime companion，claim不变；两个assembly TG917/TGe108仍deco-trackers owner。本包按07设计91deb098、e9dc/35d+强制e6cb写出实际候选，生产未写，无Cargo/rustc/test/App/GPU/benchmark。六源生产before在冻结时全匹配；各候选/patch放对应合法work，并由本manifest聚合。

## 实际源码

- InstancedWall私有base状态区分AwaitingProduction/PendingUpload/Published（后者仅CPU shadow，不代GPU ready）。同owner内private pair同库同时建立两deco handles；base update不动这两者。真实19producer只借PublicWalls.last_delta.changed，验证已有全部handles/id/generation，显式升级legacy base-only owner；保base allocation/Vec/state。未变完整owner不DerefMut写tick。新owner有真实empty三alloc、AwaitingProduction，没有Trigger/RT/成功事件。
- runtime只有同一actual typed library、同一WallEntities、只读ownerQuery的SystemParam和临时borrowedBatch。每dirty for_batch一次typedentry lookup/type检查；之后Public gate，真Some才entity/lane lookup，missing/foreign必须失败。无dirty不probeentry，无consumer补lane、第二registry/BufferHost/runtime、延长unsafe lifetime。
- 原28 build/finalize/RT helper函数逐字复用A。Query允许Awaiting无Trigger，只有真实build和真实recompute成功才updatebase/insertTrigger/RT/event；普通缺Trigger/id错仍stale失败。失败不清deco或pending。旧CPU-only无owner创建分支仍保留；它不是证明早25初始化的路径。
- 两owned模块实际7/4业务入口替换旧generic callback入口。geometryV2六个函数逐字复用f223审过的版本；real Sparse writer clear后逐砖push，失败累积仍继续。原音效kind13/14（hex）与Removed15按真实diff/7506 View/旧位置，真正写canonicalPlayDecoratorSoundCmd。Door空removal仅已证Door/PublicNone明确分支。writer guard到同wall with_removed后才drop，不跨wall/帧。

## 实际App可合范围

run_game只新增真实两tracker资源、canonicalPlayDecoratorSoundCmd App事件通道，且在现19 publication/delete之后插真实ownerproducer。保现普通GameSystemSet边和chain_ignore_deferred；不在member14造flush。**不安装**部分full→middle4→copy、两host残缺替代组、整个legacyconfigure；root待fullvisual/全部middle/clear/prestartup assets闭合后统一挂原member14。两host源码存在/资源存在不代表Blueprints真实producer、完整anchor、loader、音频backend/OnExit或GPU已闭环。

## 验证与性能边界

六源仅rustfmt解析0；这不证明Bevy SystemParam/Query借用、HRTB token返回或Rust类型通过。9项新增测试源码未执行，旧5 geometry tests亦未在新candidate执行；原8 tracker binary绑定旧三源，不能自动覆盖新host。TEST_INVENTORY记录准确边界。核心测试用现真实Sparse/Params/tracker与已有CPU TestDevice，不造writer/计数mock；partial fixture真实push填满原canonical容量，不改max/free-list/counter，设备flush只验证CPU adapter的dirty/draw inputs，不能称真实GPU验证。完整core/App缺源时应保未验证，不抽掉impl或手写替身逼过harness。

每次publication只走已有changed集合；每dirty一次typedentry查找，无全表clone/Vec3转码/CPU GPUcount回读/固定max。现clear立即还本lane chunks，Drop入waste延至当前PostUpdate回收；同帧满容量删除重建可partial，normalize不制造retry。现raster status与compute gate不同，不能宣整帧failstop。没有测量吞吐或倍数。

必要下一门：07完整source审；root唯一真实Rust编译/限定测试（尤其SystemParam/HRTB生命周期与真实set可见），然后按实际安装范围整合。完整OnExit二session、9paramanchor、originalasset/collisionloader、全member14、真实audio/backend/GPU性能仍独立门。
