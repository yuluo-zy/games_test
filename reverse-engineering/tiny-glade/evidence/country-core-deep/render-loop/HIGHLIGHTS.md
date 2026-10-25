# render_loop 域 HIGHLIGHTS（tg_rl1，供中央汇总）

1. **Nani GPU draw-list 全链恢复（O）**：gpu_generate_draw_lists（0x140987dd0）编排 round0/round≠0 双路径，pass 族 `/nani/*` 15 个 + 墙 `/wall/generate_draw_lists.cs.hlsl:cs_pass1|cs_pass2` + reset_pass2_inst_counts；round≠0 带 hzb（W27/W28）。
2. **PDB 名错位候选（需裁决）**：`DrawLists::new`（0x1409ba340）机器码是含 2 内联 Vec + 2 计数器=1 的小结构初始化，与 reconstruction 的 `DrawLists::new=default(21 空 Vec)` 不符；`grass::clone_behavior` 与 bevy is_exclusive 共享 0x140002370。
3. **Vermeer 状态机（O，手解）**：enum2$ 判别+0x48（0/1/2 = 待采集/已采集/未启用）；`is_active=cmp[+0x48],2;setne`；`should_freeze_camera=setne;and al,[+0x55]`；painterly `has_target=byte[+8]&1`。
4. **透明/水面链（O）**：water_surface/ice_surface 用软件 BVH（rt_bl_bvh/rt_triangles）+ `bind_ray_tracing`；折射拷贝 W44；pre/post-TAA 透明度 W46/W53。
5. **公共绑定 bind_common（O）**：0x14090b460 从 Transform 浮点构造 3×3 旋转矩阵（2x 项公式），绑定 shadowmap + matrix + uniform，全部 raster draw pass 复用。
6. **命令包格式（O）**：mod.rs 的 clear_rg_buffer/clear_rg_color_image/rg_buffer_copy_from_slice 用 0x40B 命令包 push 到 cap-first 命令队列（ClearBuffer/ClearImage/BufferMemcpy 标签）。
7. **材质名分派（O）**：build_draw_lists（0x1409ba540）按 shader 名串（nani_solid_albedo/vertex/velocity_only、gothic_window、window_bricks、duck/wren/butterfly/firefly、ivy_leaf/flower 等）trie 分派到 draw_shaders 各 draw 函数；ivy leaf/flower 为 name_hash 常量薄 thunk。
8. **41 个 compute_passes 全部按 wp5 §2-§23 复证 O**，每文件含 pass 名/shader URI/绑定/分派链证据。
9. **U 最大项**：DrawLists/DrawData 字段布局、各 *Ssbo 字段、handle hash→资源名映射、dispatch 枚举名映射、draw_shaders shader body（GPU 后置）。
10. **交付**：78 章全填充（占位符清零）、156 函数反编译（146 OK + 10 NotFound 手解）、DIGEST.md 完整。
