# Storage niche/owned iteration 合同独立复核

复核者water_activity_chain，不是本次候选作者。**批准进入最窄源码实现**，实现边界为本目录 `CONTRACT.md`（本次读入SHA `e83054e9492a87da1b716b26b8f95927200b821cb8783c12db981abdbad211b4`）与 `CONTRACT_REQUIRED_OLD_OPTION_ADDENDUM_WATER.md`（SHA `f313653c8e0e5c786524cfa69be5f2543f9eaf0d87f87af332d55b873d86789a`）共同组成。后者的merge/split旧Option分支已由lead明确接受，不能遗漏。没有执行构建、游戏或新增原证采集，尚无source/runtime通过结论。

## 独立核过的事实

1. 重新读完整14155afe0与归档inline实际输出门：R9指向新输入，RCX指向旧Option返回缓冲；b04e比较rsp+e7，81824e比较rsp+97，均不能读成新输入+47。removed.remove→modified.insert→map.insert发生在d2s选择前。fresh时d2s冲突仍须panic，且bucket/modified已有真实部分mutation，不能为了返回Result撤回或跳过写入。Some旧值走update，与new flags0/1无关。现代apply返回新Info API可以保留，但返回副本必须原样；原outerunit不因innerOption强改签名。
2. 在已有5个完整原包/inline范围中再次核IntoIter/Drain的实际next与niche：merge b53d/b59a/b5a0、tracked d2s257020/7077/7094、unit257570/75c8/75e2、split a1faac/fb08/fb0d。Rust for已经匹配Option，不能再复制额外state2业务break。该结论不升级raw2为有效live记录。
3. tracked d2s完整1381B重新通读：旧keys先标removed，256ffA/257006将原map临时置空，之后逐旧条目d2s.remove；正常尾2570d9..70e5归还同容量并len0。EH 257320..7394同样归还原allocation/len0。正确用drain的RAII可在d2s panic时清尽未消费项并保容量。borrow-for后clear不能提供这个失败行为。原Info纯值复制没有新增全局owner。
4. 本次读本地hashbrown0.15.2 RawDrain::drop（3935后）：剩余element Drop→clear_no_drop→把空table搬回orig_table，解释已由PDB锁定的原依赖生命周期；直接复用依赖，不把它计成游戏业务恢复或声称全vendor行为验证。
5. 全crates实际 `.merge_with(` 只有 on_walls_merged 同文件调用，现已先remove(a)、remove(b)得owned桶却clone后借传；改owned入参/consume源桶不破foreigncaller。单源仍reindex后move，双源target仍逐项reindex再consume源，最终insert merged；不增加按ID排序或default target。保原IntoIter在异常时消费/析构剩余源，不能用mem::take后把空壳当原桶。
6. split移除的源也是owned；先借传播removed_ids，再consume decorators，保d2s.remove失败点与子墙/注册表分派顺序。child insert oldOption具体门见本补充。没有因此批准重写未重新核的split几何公式。
7. 当前实际apply_decorator_place业务调用apply_decorator_op.rs:445丢弃返回值；测试中的placed副本不能绑架原state2错误规则。restore helper在restore_archived_decorators.rs是另文件和独立合法任务，原缺archive无副作用false可保；上层restore事件/清理/collider/历史并未在此完整验收。
8. 全部输入Info位保持，包括现state/flags字节。不mask、不强写0/1/2，不加平行registry。有效0/1 fixture用于原输入域检验；不把历史误造raw2 fixture作为native合法状态验收。必须合读旧 `STORAGE-MAP-VERSUS-RETURN-ERRATA`：旧实现只在返回副本写2，正常map先存原输入，不能说所有正常放置会存2/必然启动panic。
9. 顶层DecoratorStorage::clear_modification_tracking与桶Decorators同名方法不是同一个owner。当前顶层保live map且清追踪；本窄修保持它。restore/backup调用桶clear可能是另一个destructive adapter，不能依据名字顺便改生产，亦不能由本审查宣称其完整语义。

## 源码验收要求

源码后仍需独立审和root统一实际测试。核fresh/replace均按oldOption、Info所有位相同、removed/modified/map次序；merge双源含重复ID的真旧Option分支；合法多项完整迁移/空集自然结束；tracked fast和慢路径以及故意缺d2s导致panic后drain空map/容量保留；split直接入子墙与未匹配注册表。原失败不得默认成功。保本补充所列tracked新项独有插入次序，别机械复用通用insert。

## 原证和输入边界

全部复用 `/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-60901c6c8508b1af76/clutter-startup-20261020/storage-niche-audit-20261021` 的native/inline五包、iterator-abi-evidence.json与旧audit+erratum；14155afe0复用evidence/building-decorator-info/B-storage-consumer，restore复用evidence/round12/archival-owner/restore-original.asm。RNE/PDB身份与旧audit一致；没有重采全程序、改claim、源码或运行Cargo。当前contract是可执行恢复范围的通过，不能称整个storage/history/游戏主链通过。
