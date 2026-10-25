# Ghidra NotFound 小函数手解记录（roofs-rt 域，rne-read --hex 原字节）

Ghidra 数据库对这些 <0x50 字节小函数未建函数（decompile_function 返回 NotFound），按 PLAYBOOK §1.3 用 `rne-read.py --hex` 读取原字节手解。机器码为准。

## 0x140002370 — clone_behavior（共享空桩，跨 6+ 域复用）
`31 c0 c3` = `xor eax,eax; ret` → 返回 None/空。roof_animation/roof_tiles/roof_trigger_volume 等 PDB 名指向同一地址（编译器去重）。

## 0x1400885d0 — RoofRidgeDims::get_inner（已由 ridge.rs 恢复，.c 证据在 evidence/roof-ridge/）
`tg_country_core_resources_roofs_roof_ridge_RoofRidgeDims_get_inner.c`（已拷本目录）。

## 0x14067dd80 — RtBvhBuilderGlueResource::new
`48 89 c8 c3` = `mov rax,rcx; ret` → 返回 self/空构造（共享去重胶水；真实逻辑内联进 update_rt_world 调用点）。

## 0x14089d4d0 — Affine3Packed::from_mat3_translation
```
mov rax,rcx; movups xmm0,[rdx]; movss xmm1,[r8]; movss xmm2,[rdx+0x20]
unpcklps xmm2,xmm1; movlhps xmm2,[r8+4]; movups [rdx],xmm0; ... ; movups [rcx],... 
```
Mat3+translation → 48B 3×vec4 打包。`add_y` 证实 translation.y@+0x28。

## 0x14089d500 — Affine3Packed::from_cols
Mat4 列重排（shufps 0xd4/0x22/0x42、unpcklpd/movlhps），逐行写 +0x00/+0x10/+0x20。

## 0x14089d550 — Affine3Packed::add_y
`f3 0f 58 49 28` `f3 0f 11 49 28` `c3` = `addss [rcx+0x28]; movss [rcx+0x28]` → translation.y += y。

## 0x14089d560 — render::impl$1::from（From<Mat4>）
3×4 打包：读 [rdx+4/+0x14/+0x34/+0x10/+0x30/+0x28]，shufps/unpcklpd/movlhps 重排，写 +0x0/0x10/0x20。

## 0x1408dbc20 — RoofAnimation::is_being_animated
`movss xmm0,[const]; cmpss xmm0,[rcx],4(NEQ); movd eax,xmm0; and eax,1; ret` → 首字段 != 常量。

## 0x1408dbc40 — RoofAnimation::new
`xorps xmm0,xmm0; xorps xmm1,xmm1; ret` → 16B 全零。

## 0x1408e2690 — RoofRidgeDims::from（已由 ridge.rs 恢复；.c 在 evidence/roof-ridge/）
`tg_...RoofRidgeDims_from.c`（已拷）。tag!=1→{0.1,0.1}；dir(0x3c)==0 → {small+ridge*shape_dims[0], 0.1}；else → {0.1, small+ridge*shape_dims[1]}。

## 0x1408e2700 — RoofRidgeDims::calc（ridge.rs ridge_dims_calc）
`.c` 已拷。`(1-ridge)*0.1 + ridge*shape_dims[轴]`。

## 0x1408e30b0 — calc_roof_tip_center_ls（ridge.rs calc_roof_tip_center_ls_observed）
```
cmp byte[rcx+8],0; jne L; movss xmm0,[rcx+0x14]; addss xmm0,xmm0; jmp M
L: movss xmm0,[rcx+0x1c]
M: mulss xmm0,[rcx+0x24]; mulss xmm0, xmm1(scale)
```
circle→2r，rect→dim_x；乘 tip_01×scale。

## 0x1408e31a0 — FlatRoofStyle::is_wooden
```
movzx eax,[rcx]; test eax,eax; jz A; cmp eax,1; jne B; xor eax,eax; ret   ; tag==1 → false
A: movzx eax,[rdx]; ret                                                     ; tag==0 → other
B: movzx eax,[rcx+1]; ret                                                   ; tag>=2 → field@+1
```

## 0x1408e31c0 — Roof::new（roof_state.rs new_observed）
28B 参数块拷 +0x24..+0x40；几何体 +8..+0x24；floor@+0x4c；动画@+0x50；tag@+0x54；帧体 +0x40..+0x4c；+0x55..0x57 不写。

## 0x1408e3220 — Roof::pivot_3d
`mov ecx,[rdx+8](tag); movss xmm0,[rdx+rcx*8+0xc]; movss xmm1,[rdx+rcx*8+0x10]; movss xmm2,[rdx+0x4c]; mov [rax]xmm0; [+4]xmm1; [+8]xmm2` → {frame.xz by tag, floor@+0x4c}。

## 0x1408e3250 — Roof::ty（roof_state.rs ty_observed）
`movss xmm0,[const 0.1]; ucomiss xmm0,[rcx+0x30]; seta al; ret` → height_01 > 0.1（NaN 无序置 CF→false）。

## 0x1408e3380 — Roof::is_flat
`movss xmm0,[const]; ucomiss xmm0,[rcx+0x30]; setae al` → height_01 <= 0.1。

## 0x1408e3390 — Roof::is_flat_w_no_rim
`... setae al; cmp byte[rcx+0x3d],0; setne cl; and al,cl? ; ret` → is_flat && rim(+0x3d)==0（rim 字节 AND 判定）。

## 0x1408e33b0 — Roof::is_gable（roof_state.rs is_gable_observed）
`movss xmm0,[rcx+0x30]; movss xmm1,[rcx+0x34]; cmpss xmm0,[const 0.1],1(LT); cmpss xmm1,[const 1.0],0(EQ); andps; movd eax; and eax,1` → ty && ridge==1。

## 0x1408e33e0 — Roof::gable_sides（.c 在 evidence/roof-tiles/，已拷）
`if ridge==1 && ty: out[1]=dir_byte; out[2]=dir_byte|2; found=1 else found=0`。

## 0x1408e3800 — Roof::shape_dims（.c 在 evidence/roof-surface/，已拷）
`cmp byte[rcx+8],0; jne R; movss xmm0,[rcx+0x14]; addss xmm0,xmm0; movaps xmm1,xmm0; ret` （circle→{2r,2r}）
`R: movss xmm0,[rcx+0x1c]; movss xmm1,[rcx+0x20]`（rect→{dim_x,dim_z}）。

## 0x1408e3c30 — RoofShapeParams::eave_length_ws（surface.rs eave_length_ws）
`.c` 已拷。`(sqrt(profile)+(1-sqrt(profile))*0.5)*(1.5*eave+(1-eave)*0)`，返回单一 f32（62B）。

## 0x1408e6af0 — roof_shape::impl$37::clone_behavior（Roof 组件真实现）
`lea rdx,[closure@0x14113144]; mov eax,2; ret` → 返回 `(Some(closure), 2)`。

## 0x140939390 — RoofTileSimulation::new
`mov [rcx],0(dword); movss [rcx+4],xmm1(timer); mov qword[rcx+8],0; mov qword[rcx+0xd],0; ret` → 状态+计时+余量零化。

## 0x140a38600 — TreeStorage::next_index
`mov rax,[rcx+0x30]; lea rdx,[rax+1]; mov [rcx+0x30],rdx; ret` → 自增 +0x30 并返回旧值。

## 0x140a880c0 — get_roof_expanded_top_and_bottom（.c 在 evidence/roof-tiles/，已拷）
## 0x140a88200 — RoofBoundsDims::as_circle（.c 已拷）
`test byte[rcx],1; jnz PANIC; movss xmm0,[rcx+4]; movss xmm1,[rcx+8]; ret`；PANIC：`lea rcx,msg; mov edx,0x28; lea r8,file; call panic`。
## 0x140a88240 — RoofBoundsDims::as_rectangle
`test byte[rcx],1; jz PANIC; movups xmm0,[rdx+4]; movups [rcx],xmm0; mov rax,rcx; ret`；PANIC 同上（line 0x28）。

## 0x1408bc460 — rt::impl$1::from
```
movss xmm0,[rdx]; sqrtss xmm3,xmm0; mulss xmm3,K; addss xmm3,B
cvttss2si r8,xmm3; xor eax,eax; ucomiss xmm3,0; cmovb r8d,eax
movss xmm4,[M]; ucomiss xmm3,xmm4; mov r9d,-1; cmovge r9d,eax; ...
```
→ `trunc(sqrt(x)*K+B)` 夹 [0,M)，越界 -1 哨兵（打包字段转换）。常量 K/B/M 未解析 → I。

## 0x1408bc510 — rt::impl$2::from
`mov rax,rcx; mov [rcx+8],rdx; mov dword[rcx],1; ret` → `{tag=1, value}`。

## 0x1408bc520 — rt::impl$5::from（From<WallId>）
`xor eax,eax; mov rdx,rcx; ret` → 空/零 key（RtWorld 键转换）。

## 0x1408e30f0 — inverse_ridge_01_param（已由 ridge.rs 恢复；.c 在 evidence/roof-ridge/，已拷）
`(ridge-0.1)/(dim-0.1)`，NaN→panic("invalid inverse")，退化(dim==0.1)→panic("degenerate")，夹紧 [0,1]。

## 0x1408e2750 — RoofRidgeDims::to_world_space（已由 ridge.rs 恢复；.c 已拷）
点/线段双分支；线段分支乘零保留符号零。

## 0x140833230 / 0x1408c5680 — GableRoofStyle 谓词（与 wall 域谓词同地址去重）
is_stepped=`cmp byte[rcx],0; sete`（tag==0）；is_overhang=`movzx byte[rcx]`（tag!=0）。
