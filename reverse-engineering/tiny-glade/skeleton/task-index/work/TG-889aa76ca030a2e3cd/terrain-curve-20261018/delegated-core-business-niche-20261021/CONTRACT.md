# 存储旧Option/owned迭代的最窄恢复合同

root明确委派TG889旧terrain-curve owner，claim保持；本work候选唯一作者lead，不生产/Cargo/GPU。当前storage before8b3dbb7a、restore helper beforef42dd7bc（TG2f6a live NULL后正常claim core-storage-niche-20261021）。restore不是同source，必须对应合法work配对候选，不能伪称两个方法同文件。

## 原证与独立复核

完整冻结STORAGE-NICHE-AUDIT+必须合读STORAGE-MAP-VERSUS-RETURN-ERRATA位于TG609/clutter-startup-20261020/storage-niche-audit-20261021。本作者独立读14155afe0完整已存汇编：R9输入Info保存RBX，RCX返回区保存RDI；removed erase→modified insert→HashMap insert返回旧Option至rsp+a0，检查的是rsp+e7。None走d2s insert并仅给返回区+47写2；Some复制旧值并update。外层140a20c70丢弃旧Option，是unit，不把inner Option强推成外层返回API；现现代接口返回新Info可兼容保留，但必须字节原样，不造state2。

归档原完整3967B的891指令已逐字节验证。关键818249 insert回rsp50，24e看rsp97；不是输入rsp137。复用实际bucket.insert返回旧Option即可，不按新Info.state分支。

三loop原PDB明确IntoIter/Drain/IntoIter::next Option（完整5原包9844B及inline identities已存）：merge14155b170、tracked141256e40/1412573b0、split140a1f590。原确实读bucket内Info+47比较2，但这是next返回None niche；当前Rust for已完成Option匹配，不再把2复制成合法live状态业务门。原字段命名/flags含义不在本scope，**所有输入原字节保持，不mask/强制0/1、不新增flags registry**。人为raw2不被提升成原合法输入域，不写新测试当native live2。

## 必要源码闭包与生命周期

- Decorators::insert按原removed.remove→modified.insert→map.insert顺序，返回真实旧Option。当前map在modified之前错误次序一并纠正，保分配失败前已发生的原副作用。
- apply_decorator_place保immutable/get_or_insert门、原modern返回新Info签名；保存old Option，old None→d2s.insert（冲突panic），Some→d2s.update。新输入所有字段与返回副本原样，绝不写2。
- overwrite_decorator_state_from_archivist只改相同old Option分支；缺archive返回false无副作用。上层restore事件/collider/历史其它缺口不因此完成。
- merge源是原owned map IntoIter。当前全crates只有on_walls_merged一个具体调用；它已从per_wall移除两个owned桶，却先clone两份再借用merge。改同文件merge_with接受owned Decorators，on_walls_merged直接match两个Option并move源/目标，保remove(a)再remove(b)、单源reindex、双源target逐项重索引、最后per_wall.insert的原顺序；取消无意义桶clone。stage1借source.removed_ids，stage3 consume source.decorators.for迭代，删除额外state2 break。没有跨文件caller/API适配。
- tracked慢路径原Drain，不只是清空map后的for。原先先标旧keys removed，然后drain原map逐项d2s.remove。drain保持容量，遇d2s panic时Drain析构仍清理剩余owned entries，不能borrow循环后clear造成原失败时留下旧map。快路径和新项标记/tail d2s逻辑不动。
- split从per_wall移除的owned bucket，先传播removed_ids后consume owned decorators，不再借用+raw2 break；各dst侧原区间搜索/required d2s/removed注册顺序保持。此包不改旧几何分裂公式或侧分派算法。
- **顶层DecoratorStorage::clear_modification_tracking已有原791B正确保live map，完整保留。** 桶同名clear是另一个destructive clear adapter，不按名字把它与顶层混同、不在此无证重写。decorator_chain validator只读顶层正式方法。

## 验收边界

有效flags0/1 fresh insert与replace原字节保持、原removed/modified追踪；fresh bucket但d2s冲突对0/1均原panic且保bucket/modified已写部分副作用；replace按old Some走update而非新字节。多0/1装饰merge、tracked慢/快、split全部迁移并索引一致；空集自然结束；tracked d2s缺失时drain所有旧条目析构且旧map空、容量保留。旧state2 live测试换为多合法状态迁移，不用无效域验证错误业务。

source作者不运行Cargo；独立复核后root统一实际测试。此包不宣称完整storage/tracking/history/restore所有业务原侧等价。
