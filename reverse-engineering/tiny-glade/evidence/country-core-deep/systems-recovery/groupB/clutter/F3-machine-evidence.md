# F3 机器码证据摘录（capstone 直解 B/RNE，2026-10-16，re-analyst tg_xb1）

## kind 跳表（0x143043818，5 项 + default）
```
0x1422db580: mov rax,[rbp+0x6c0]        ; 记录指针（=区域表项+0x48）
0x1422db587: movzx eax, byte ptr [rax-7] ; kind（表项+0x41）
0x1422db596: cmp rax, 4 ; ja 0x1422dd7ca ; default
0x1422db5a0: lea rcx,[rip+0xd68271]      ; 表基 0x143043818
0x1422db5a7: movsxd rax,[rcx+rax*4] ; add rax,rcx ; jmp rax
kind[0]=0x1422db5b0 kind[1]=0x1422dd1db kind[2]=0x1422dc378 kind[3]=0x1422dcaab kind[4]=0x1422db834
```
## mesh-type 跳表（0x14304382c，28 项）
```
0x1422d7dd7: movzx eax, byte ptr [rax-8] ; mesh-type（表项+0x40）
0x1422d7ddb: lea rdx,[rip+0xd6ba4a]      ; 表基 0x14304382c
0x1422d7de2: movsxd rcx,[rdx+rax*4] ; add rcx,rdx ; jmp rcx
[0]=0x1422d7df6 [1]=0x1422d7dfa [2]=0x1422d7e34 [3]=0x1422d7e34 [4]=0x1422d7e32
[5]=0x1422d7da2 [6]=0x1422d7da6 [7]=0x1422d839d [8]=0x1422d7dae
[9]=0x1422de849 [10]=0x1422de84d [11..18]=0x1422d7dba..dd6 [19]=0x1422d89df
[20]=0x1422de875 [21]=0x1422d7de2 [22]=0x1422d7de6 [23]=0x1422de881
[24]=0x1422de885 [25]=0x1422d8961 [26]=0x1422d8ee2 [27]=0x1422d8ea6
```
## 加权随机挑网格（type 0/1，0x1422d7e3b–0x1422d7f2e）
```
sum = Σ 权重（0x40 步 4 项 + 尾部 0x10 步）；init_rng_u64(键哈希)（0x140c96270）
→ fastrand::f32（0x140caf380）→ 累和 权重/总 直到 ≥ r → 选中下标（越界 panic 0x1422de821）
→ 网格记录 = 表[选中].ptr（+0x30 权重，0x10 槽）
```
## 过滤器判别（0x1422d8107–0x1422d8468）
```
0x1422d810e: mov r12b, byte ptr [rax-8]   ; r12b = mesh-type
0x1422d8180: call 0x1408cc870            ; legacy_raycast（[rsp+0x20]=0x30fb 掩码）
0x1422d8185: add r12b, 0xfe              ; type-2
0x1422d8189: cmp [rbp+0x1d4], 2 ; jne 0x1422d8320  ; hit!=2 分支
0x1422d8234/58/7d/a2: call 0x140a14df0 ×4  ; 4 角水面采样（xmm3=130.0@0x142a7b658）
0x1422d82b5-2c4: maxss(0,sum); minss(1.0)   ; clamp[0,1]（1.0@0x142925904）
0x1422d82c8: ucomiss xmm1, [rip+0x6883c1]  ; 阈值 0x142960690 = 0.0
0x1422d82d3: jbe 0x1422d83e0              ; dry
  wet: 0x1422d82fe: [rbp+0x6d8]=1; 0x1422d830a: cmp r12b,3; jb 0x1422d7da2; jmp 0x1422d8468
  dry: 0x1422d840c: cmp r12b,3; jae 0x1422d845d→0x1422d8468
       0x1422d8412: pos=( [rax-0x24]^0x80000000, [rax-0x28] ) → 0x1422d8510
0x1422d8468: init_rng_u64([rbp+0x6e8]=键哈希) → f32 r → θ = (1-r)*0.0 + r*2π（2π@0x142934674）
             → cos@0x142923e80 / sin@0x142923fd0 → (cosθ,sinθ)
```
## 常量
```
0x142960690 = 0.0f        （水位判别阈值）
0x142960674 = 0.01f       （单采样 skip 门；packed 0x1429cd300 = {0.01,0.01} = type0/1 落点最小钳）
0x142934674 = 6.283185f   （2π，圆周落点角上限）
0x142a7b658 = 130.0f      （bilinear 采样半径）
0x142925904 = 1.0f        （clamp 上限）
掩码 0x30fb               （legacy_raycast）
预设记录哨兵：[rec+0x30] == -1 → 权重 0（Option None）
```
