# canonical数据API候选独立源码复核

审查者 tree_commands；2026-10-21。root经lead明确允许在原合法work写本review，未改claim/候选/生产。

源码 `tiny-glade/crates/systems/decorator/src/merge_proposal_roof_decorators.rs`
before SHA256 `50eeaacff4a25c54bdc4f184980f70948a4236b079de214cff04146e415ab3d8`
after SHA256 `2d7917ecaf5783164719e29f2b7c94dcddd275c3ec94d6ef3adde6eaf8072515`
patch SHA256 `59fa28eec026a966d1a2b2c4a8ef5c249b70f21c99f60deee1267bd078f3f43c`

独立复核通过，仅批准typed subtype与碰撞逻辑宽度读取适配。现canonical get返回DecoratorSubtype Copy，derived.0正确，旧derived.subtype()已无该owner契约；key仍(existing.rank)，required ID→required exact collision key→bounds.footprint.size.x的顺序未变。原roof-merge-select-full.asm.txt 1420013e1先ID类型读取、13ee key构造、1403查collision、1408..140b缺失失败，1411明确读取同值+30逻辑宽度；绝不能从旋转AABB x跨度替代。

此patch没有测试增删、无Vec/Arc克隆、无width默认值，原剩余delta符号/rank/family/selector等算法差异保持现状；该批准不表示完整roof merge恢复。无需为了本窄类型调用变更触及wall selector或新建替代数据owner。

已核before/candidate/patch实际SHA与delivery一致；依赖TG609最终aa33480b... canonical四表与TG-b0d最终452094d4...真实box入口。未执行Cargo/rustc/GPU或原程序；生产整合与真实测试由root。此审查不能被计为整包业务恢复或C运行同等性。
