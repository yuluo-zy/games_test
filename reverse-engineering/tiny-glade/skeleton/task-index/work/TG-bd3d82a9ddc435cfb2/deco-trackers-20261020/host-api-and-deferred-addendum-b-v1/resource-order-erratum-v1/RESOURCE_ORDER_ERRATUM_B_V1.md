# 强制 resource failure 次序 erratum B V1

旧 HOST_API_AND_DEFERRED_ADDENDUM_B_V1.md（f13501d5）与manifest35d667ff保持不变。本补充明确**替代**第四节「两host先public.get(wall)」的描述；其余owner/deferred/容量/注册门不外推。

原每dirty wall先完成真实Brick SSBO查找及完整type验证，再PublicWalls查找。Chimney ae937调用141258610，ae942缺entry跳af195 panic，ae96d/983核0x3a type name，ae990真实downcast，ae999缺值同panic；ae9ad之后才取PublicWalls，ae9bc/aea60是真wall miss分支。Door af5a9同查找、af5b4/60e缺值→afa17，af5e0/5f6核type，af61b之后PublicWalls；真miss af627/af91f。不能在Public miss时掩盖缺storage或错误storage type而normalize成功。

无dirty wall时原不做entry查找；不要为了简化在with_diffs外全局require_entry。当前SystemParam resource存在性本身仍由Bevy验证，和库内typedentry不同。

具体最窄API改为：`WallBrickAssemblyResources::for_batch(&mut self)` 在**每dirty callback开头**做一次 `library.get_mut::<BrickTransformSsbo>().expect(...)`，返回仅本scope的borrowed `WallBrickAssemblyBatch`，持同storage的&mut以及同WallEntities/Query的借用引用。此时尚不查wall entity/lane。随后caller实际PublicWalls.get：真None才显式noop-removal；真Some再batch.chimney_writer/door_stairs_writer查owner/id/lane并借用该已取得storage。batch不是第二registry/runtime/host；它不拥有数据，不跨wall/帧保存，不克隆map/Arc。

这样每dirty仍只有一次库probe，不加validate+writer两次查找，不假报cachehit。writer及其allocation mutex保留到同wall with_removed完成；原Chimney af077先with_removed，af087..09f后unlock已选指令绑定。不得延长到下一wall或跨帧。

新增限定验证项（未运行）：同时missing Public和typedstorage时须先报storage错误且不得returncompletion；typedstorage存在而Public miss允许已证noop removals；Public存在但owner/lane缺失必须失败；空dirty不probe库entry；所有正常分支只查一次库，不用统计数字冒充实测吞吐。
