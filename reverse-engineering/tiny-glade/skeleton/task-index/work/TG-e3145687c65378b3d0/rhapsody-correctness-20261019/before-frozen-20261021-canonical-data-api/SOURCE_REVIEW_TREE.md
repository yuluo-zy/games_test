# canonical数据API候选独立源码复核

审查者 tree_commands；2026-10-21。root经lead明确允许在原合法work写本review，未改claim/候选/生产。

源码 `tiny-glade/crates/systems/decorator/src/collision_detection.rs`
before SHA256 `229d084f590d73a70df70936e4241e3d0a410d41164a1ea183b84728ca223841`
after SHA256 `061d3d4bc7ff97e4b1c84dc2ae39b944e76ac45c8eb4c74fe298fbfb5396d394`
patch SHA256 `bae9baeed3c5fa1e8da50e3ec30fbe7d252e95f343626290817b45705dd52efb`

独立复核通过，仅批准本次canonical数据/API适配。ID先由唯一DerivedDecoratorSubtypes.get(id) required取得Copy subtype，接着以(subtype.0,info.rank)构造exact key并由collision_record借用真实记录；缺ID先于缺key失败，无try_get/默认值/几何复制。原parent-wall-linear-predicate-full.asm.txt 141fe57af取得subtype，57bc建key，57d4查表，57d9..57dc空值进入原失败；57e2/57e7是top+3c减bottom+38，57f2读+30逻辑宽度，与canonical Rectangle3d field-order erratum一致。新中文注释准确区分from_bbx的显式legacy辅助。

回归夹具拥有真正Arc<RaycastMeshData::new_cuboid_at>、原box生成的TriangleVec、typed显式None outline，借同库测rank2宽2/高4、rank1缺key和ID42缺失。旋转footprint是专门区分逻辑宽与AABB投影的有限record度量域夹具；并不证明真实旋转物理shape一致性或端到端碰撞。Arc返回后计数仅辅助，零额外复制由实际生产函数直接借用/读取独立确认。未删除旧case或测试。

已核before/candidate/patch实际SHA与delivery一致；依赖TG609最终aa33480b... canonical四表与TG-b0d最终452094d4...真实box入口。未执行Cargo/rustc/GPU或原程序；生产整合与真实测试由root。此审查不能被计为整包业务恢复或C运行同等性。
