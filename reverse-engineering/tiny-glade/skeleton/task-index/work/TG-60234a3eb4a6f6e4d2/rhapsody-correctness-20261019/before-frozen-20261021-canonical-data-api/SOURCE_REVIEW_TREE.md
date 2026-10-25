# canonical数据API候选独立源码复核

审查者 tree_commands；2026-10-21。root经lead明确允许在原合法work写本review，未改claim/候选/生产。

源码 `tiny-glade/crates/systems/decorator/src/ui_systems/decorator_interaction_intent.rs`
before SHA256 `929c4ca757c027dc62eec2d140b453690558d15a9e5d86efe3669042209927cd`
after SHA256 `9a4bc189a21e08ddff3e1ecc0fea2d1afc0dcdd84628384d59e831571bd8d2ec`
patch SHA256 `1495cb41b5a726a2da68c94d04241c0716403675d2a80c010e2e133bf1d6fe68`

独立复核通过，仅批准snap高度的canonical required借用以及真实ECS夹具改型。生产snap_collision_height仅collision_record(exact key)后top-bottom，无可选fallback、无TriangleVec或Arc克隆。snap_balcony_door原先两次lookup仍在几何/query前，调用顺序与rank字段不变。原snap-balcony-door.asm.txt 14127f18a构造full key、f2c9/f2d0取72B bucket末端-4减-8（对应record+3c-+38）；f2ff构造baseline key，f446/f44d同样读取高度差。缺key分支仍required失败，非Unavailable或零高。

scene夹具原legacy DerivedDecoratorInfo bbox factory已替为真实box TriangleVec+Arc<new_cuboid_at>+Rectangle3d(中心size*.5、同尺寸)+typed显式None outline；同World真正插DecoratorMeshLibrary及RaycastWorld，保原Query<&Roof>、真实surface碰撞与rank/baseline缺key测试，未删/忽略旧case或放宽容差。此处None是显式不消费的outline值，shape为真实cuboid，不是None物理壳。既有完整calculate/merge未恢复分支不在本patch验收。

已核before/candidate/patch实际SHA与delivery一致；依赖TG609最终aa33480b... canonical四表与TG-b0d最终452094d4...真实box入口。未执行Cargo/rustc/GPU或原程序；生产整合与真实测试由root。此审查不能被计为整包业务恢复或C运行同等性。
