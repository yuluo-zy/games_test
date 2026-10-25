# 组C HIGHLIGHTS（re-analyst-C，2026-10-16）— 供中央汇总

1. **125/125 真实业务 VA 证据闭合**（sourcemap 126 行 = wall-constructor 93 + roof 33；1 项 rhapsody create_buffer 0x141fdca60 按 DEPENDENCY_FILTERING 筛除）。groupC decomp：wall-constructor 93 + roof 32 文件（67 fresh RE-MCP + 55 既有 .c 拷贝 + 3 手解）；reviewer REVIEW.md 缺失清单 6 项已全部闭环（5 补反编译/拷贝 + 1 筛除）。
2. **arch_walker 全族算法契约（restorer 硬依赖）**：砖长 = 总长/整数砖数（floor(total/brick_len+0.5) 后重除）；尾砖阈值 = brick_len*0.1（ArchWalker+0xc 常量 0x3dcccccd）；插值 t 越界/NaN 必 panic。4 个 walk 实例同构，closure 每砖调用。
3. **共享双缓冲队列资源**（progress@0 + VecA@0x08{cap,ptr,len} + VecB@0x20 + total@0x40）贯穿 create_roof/update_roof_spatial/generate_roof_stone_floor/place_pillars/maintain_plaster/assemble_roof_tiles —— 统一实现可覆盖 6 系统。
4. **`~id*0x30` 反向索引** = 槽位 u64::MAX-id（None=0 哨兵），delete_roofs/update_roof_spatial 共用。
5. **circle_categorize 机器码差分完成（reviewer 要求）**：1.75/-1.75/1.25/3.0/0.3/0.6 全部 asm rip 目标 ↔ .rdata 字节核对 ✓；算法 = 2r×2r 网格形状内点占比 vs smoothstep 阈值 → 圆形判定。
6. **half_timbering 0x141218e30（26.7KB）Ghidra 建桩失败 → 全 asm 落盘**：11 参签名（1st=wall→try_get_wall_shape，7th 栈参=Roof，XMM0 16B 查表键），construct_walls→half_timbering→(is_gable)→roof_outline 调用链。
7. **roof 谓词机器码契约（reviewer 优先项，14 个 Gap 可落码）**：ty=height_01>0.1(setb)、is_flat=height_01<=0.1(setae)、is_flat_w_no_rim=flat&&[+0x3d]!=0、is_gable=height>0.1&&ridge==1.0、gable_sides={1,rim@+0x3c,rim|2}、shape_dims 公式（circle 基 8.0/rect 基 9.0、s*12+(1-s)*base）、rotation_yaw=atan2(-sin,cos)/-π/2-atan、ridge_length_01=1.5*smoothstep+r、roof_height_ws=lerp(4,10)/12 与 lerp(3,6)/12。
8. **纠错（reviewer-C 登记确认）**：roofs-rt/ALGORITHMS.md `_UNK_142934604=16.0` 错；字节 = 8.0f（0x41000000），实现侧正确。
9. **place_pillars/assemble_stone_pillar 常量表**：0.2/1.2/0.01/0.5/±0.2/2.1/±0.25/0.56/1.26（.rdata 直读）；SSBO 注册表访问模式（FUN_141209c40 + 虚调 + Arc/condvar 握手）跨 place_pillars/assemble_roof_tiles/half_timbering。
10. **待澄清项**：0x1408e3250/3380 族 movss 常量目标（0x142a85984/85f84）在 RNE+B 双文件均为字符串区，语义阈值 0.1 由 gable_sides 旧 DB decomp + roof_state.rs 链证实 —— PDB 地址 vs .rdata 布局疑错位，建议符号级 DB 复核；construct_arches shape2 分支与 C wooden_arch_frame 登记未覆盖。
