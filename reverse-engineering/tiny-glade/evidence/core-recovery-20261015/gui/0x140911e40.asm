0x140911e40 push     r15
0x140911e42 push     r14
0x140911e44 push     r13
0x140911e46 push     r12
0x140911e48 push     rsi
0x140911e49 push     rdi
0x140911e4a push     rbx
0x140911e4b sub      rsp, 0x290
0x140911e52 movaps   xmmword ptr [rsp + 0x280], xmm7
0x140911e5a movaps   xmmword ptr [rsp + 0x270], xmm6
0x140911e62 mov      rsi, r8
0x140911e65 mov      rax, qword ptr [rsp + 0x2f0]
0x140911e6d mov      r8d, dword ptr [r9 + 0x1c]
0x140911e71 mov      r10, qword ptr [r9 + 0x10]
0x140911e75 mov      dword ptr [r10], r8d
0x140911e78 mov      rdi, qword ptr [r9]
0x140911e7b mov      r8d, dword ptr [rdi + 0x18]
0x140911e7f mov      r12, qword ptr [rdx]
0x140911e82 movss    xmm1, dword ptr [r12 + 0x28]
0x140911e89 mov      r13, qword ptr [rax]
0x140911e8c mulss    xmm1, dword ptr [r13 + 0xec]
0x140911e95 movss    xmm0, dword ptr [rdi + 0x1c]
0x140911e9a lea      rdx, [rip + 0x2199de7]
0x140911ea1 movsxd   r9, dword ptr [rdx + r8*4]
0x140911ea5 add      r9, rdx
0x140911ea8 jmp      r9
0x140911eab jmp      0x140911f4a ; 
0x140911eb0 addss    xmm0, xmm1
0x140911eb4 movss    xmm1, dword ptr [rip + 0x201c834]
0x140911ebc xor      edx, edx
0x140911ebe ucomiss  xmm1, xmm0
0x140911ec1 seta     dl
0x140911ec4 ja       0x140911ec9
0x140911ec6 xorps    xmm0, xmm0
0x140911ec9 xor      edx, 3
0x140911ecc mov      dword ptr [rdi + 0x18], edx
0x140911ecf movss    dword ptr [rdi + 0x1c], xmm0
0x140911ed4 mov      rcx, qword ptr [rcx]
0x140911ed7 mov      edx, dword ptr [rcx + 0x128]
0x140911edd lea      r8, [rip + 0x2199db4]
0x140911ee4 movsxd   rdx, dword ptr [r8 + rdx*4]
0x140911ee8 add      rdx, r8
0x140911eeb jmp      rdx
0x140911eed addss    xmm0, xmm1
0x140911ef1 movss    xmm1, dword ptr [rip + 0x202279b]
0x140911ef9 xor      edx, edx
0x140911efb ucomiss  xmm1, xmm0
0x140911efe seta     dl
0x140911f01 lea      edx, [rdx + rdx*2]
0x140911f04 mov      dword ptr [rdi + 0x18], edx
0x140911f07 movss    dword ptr [rdi + 0x1c], xmm0
0x140911f0c mov      rcx, qword ptr [rcx]
0x140911f0f mov      edx, dword ptr [rcx + 0x128]
0x140911f15 lea      r8, [rip + 0x2199d7c]
0x140911f1c movsxd   rdx, dword ptr [r8 + rdx*4]
0x140911f20 add      rdx, r8
0x140911f23 jmp      rdx
0x140911f25 addss    xmm0, xmm1
0x140911f29 mov      r8d, 1
0x140911f2f movss    xmm1, dword ptr [rip + 0x2022759]
0x140911f37 ucomiss  xmm1, xmm0
0x140911f3a ja       0x140911f4a
0x140911f3c addss    xmm0, dword ptr [rip + 0x2199590]
0x140911f44 mov      r8d, 2
0x140911f4a mov      dword ptr [rdi + 0x18], r8d
0x140911f4e movss    dword ptr [rdi + 0x1c], xmm0
0x140911f53 mov      rcx, qword ptr [rcx]
0x140911f56 mov      edx, dword ptr [rcx + 0x128]
0x140911f5c lea      r8, [rip + 0x2199d35]
0x140911f63 movsxd   rdx, dword ptr [r8 + rdx*4]
0x140911f67 add      rdx, r8
0x140911f6a jmp      rdx
0x140911f6c xorps    xmm0, xmm0
0x140911f6f ucomiss  xmm0, dword ptr [rcx + 0x12c]
0x140911f76 jae      0x140911fb7
0x140911f78 mov      edx, dword ptr [rcx + 0x130]
0x140911f7e lea      r8, [rip + 0x2199d27]
0x140911f85 movsxd   rdx, dword ptr [r8 + rdx*4]
0x140911f89 add      rdx, r8
0x140911f8c jmp      rdx
0x140911f8e mov      ecx, dword ptr [rax + 0x1c]
0x140911f91 mov      rax, qword ptr [rax + 0x10]
0x140911f95 mov      dword ptr [rax], ecx
0x140911f97 mov      rax, qword ptr [r13]
0x140911f9b mov      rbx, qword ptr [r13 + 8]
0x140911f9f mov      qword ptr [r13], 0
0x140911fa7 lea      rcx, [rip + 0x2199d12]
0x140911fae movsxd   rax, dword ptr [rcx + rax*4]
0x140911fb2 add      rax, rcx
0x140911fb5 jmp      rax
0x140911fb7 mov      rax, qword ptr [rdi + 0x10]
0x140911fbb mov      rbx, qword ptr [rdi + 0x20]
0x140911fbf inc      rbx
0x140911fc2 cmp      rax, rbx
0x140911fc5 cmovb    rbx, rax
0x140911fc9 mov      rdx, qword ptr [rsi]
0x140911fcc lea      rcx, [rsp + 0x70]
0x140911fd1 call     0x140911710 ; country_core::systems::camera_rig::MainCamera::store_parameters::hb8702e3b7c5c86bf
0x140911fd6 mov      r14, qword ptr [rdi + 0x10]
0x140911fda cmp      r14, rbx
0x140911fdd jb       0x1409124c7
0x140911fe3 cmp      r14, qword ptr [rdi]
0x140911fe6 jne      0x140911ff7
0x140911fe8 lea      rdx, [rip + 0x2199951]
0x140911fef mov      rcx, rdi
0x140911ff2 call     0x140b5fc30 ; alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::heefd067d23e5a206 / alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::hadbebbf8c9ec68bc
0x140911ff7 lea      r15, [rbx + rbx*8]
0x140911ffb shl      r15, 2
0x140911fff add      r15, qword ptr [rdi + 8]
0x140912003 mov      rax, r14
0x140912006 sub      rax, rbx
0x140912009 jbe      0x14091201f
0x14091200b lea      rcx, [r15 + 0x24]
0x14091200f shl      rax, 2
0x140912013 lea      r8, [rax + rax*8]
0x140912017 mov      rdx, r15
0x14091201a call     0x142923c10 ; memmove
0x14091201f mov      eax, dword ptr [rsp + 0x90]
0x140912026 mov      dword ptr [r15 + 0x20], eax
0x14091202a movups   xmm0, xmmword ptr [rsp + 0x70]
0x14091202f movups   xmm1, xmmword ptr [rsp + 0x80]
0x140912037 movups   xmmword ptr [r15 + 0x10], xmm1
0x14091203c movups   xmmword ptr [r15], xmm0
0x140912040 inc      r14
0x140912043 mov      qword ptr [rdi + 0x10], r14
0x140912047 mov      qword ptr [rdi + 0x20], rbx
0x14091204b jmp      0x14091218f ; 
0x140912050 xorps    xmm0, xmm0
0x140912053 ucomiss  xmm0, dword ptr [rcx + 0x134]
0x14091205a jb       0x140911f8e
0x140912060 mov      rcx, qword ptr [rdi + 0x10]
0x140912064 test     rcx, rcx
0x140912067 je       0x140911f8e
0x14091206d mov      rax, qword ptr [rdi + 0x20]
0x140912071 inc      rax
0x140912074 mov      rdx, rax
0x140912077 or       rdx, rcx
0x14091207a shr      rdx, 0x20
0x14091207e je       0x14091215d
0x140912084 xor      edx, edx
0x140912086 div      rcx
0x140912089 mov      rbx, rdx
0x14091208c mov      qword ptr [rdi + 0x20], rdx
0x140912090 jmp      0x140912167 ; 
0x140912095 mov      rdx, qword ptr [rdi + 0x10]
0x140912099 mov      qword ptr [rdi + 0x20], rbx
0x14091209d test     rdx, rdx
0x1409120a0 je       0x14091218f
0x1409120a6 cmp      rdx, rbx
0x1409120a9 ja       0x140912167
0x1409120af lea      r8, [rip + 0x21998d2]
0x1409120b6 mov      rcx, rbx
0x1409120b9 call     0x1428d9518 ; core::panicking::panic_bounds_check::h213dd6812833a70b
0x1409120be mov      rdx, qword ptr [rsi]
0x1409120c1 lea      rcx, [rsp + 0x70]
0x1409120c6 call     0x140911710 ; country_core::systems::camera_rig::MainCamera::store_parameters::hb8702e3b7c5c86bf
0x1409120cb mov      rdx, qword ptr [rdi + 0x10]
0x1409120cf cmp      rdx, rbx
0x1409120d2 jbe      0x1409124eb
0x1409120d8 mov      rax, qword ptr [rdi + 8]
0x1409120dc lea      rcx, [rbx + rbx*8]
0x1409120e0 mov      edx, dword ptr [rsp + 0x90]
0x1409120e7 mov      dword ptr [rax + rcx*4 + 0x20], edx
0x1409120eb movups   xmm0, xmmword ptr [rsp + 0x70]
0x1409120f0 movups   xmm1, xmmword ptr [rsp + 0x80]
0x1409120f8 movups   xmmword ptr [rax + rcx*4 + 0x10], xmm1
0x1409120fd movups   xmmword ptr [rax + rcx*4], xmm0
0x140912101 jmp      0x14091218f ; 
0x140912106 mov      r14, qword ptr [rdi + 0x10]
0x14091210a cmp      r14, rbx
0x14091210d jbe      0x1409124d9
0x140912113 mov      rax, qword ptr [rdi + 8]
0x140912117 lea      rdx, [rbx + rbx*8]
0x14091211b lea      rcx, [rax + rdx*4]
0x14091211f lea      rdx, [rax + rdx*4]
0x140912123 add      rdx, 0x24
0x140912127 mov      rax, rbx
0x14091212a not      rax
0x14091212d add      rax, r14
0x140912130 shl      rax, 2
0x140912134 lea      r8, [rax + rax*8]
0x140912138 call     0x142923c10 ; memmove
0x14091213d dec      r14
0x140912140 mov      qword ptr [rdi + 0x10], r14
0x140912144 mov      rax, qword ptr [rdi + 0x20]
0x140912148 cmp      rax, rbx
0x14091214b jb       0x14091218f
0x14091214d xor      ecx, ecx
0x14091214f sub      rax, 1
0x140912153 cmovae   rcx, rax
0x140912157 mov      qword ptr [rdi + 0x20], rcx
0x14091215b jmp      0x14091218f ; 
0x14091215d xor      edx, edx
0x14091215f div      ecx
0x140912161 mov      ebx, edx
0x140912163 mov      qword ptr [rdi + 0x20], rbx
0x140912167 lea      rdx, [rbx + rbx*8]
0x14091216b shl      rdx, 2
0x14091216f add      rdx, qword ptr [rdi + 8]
0x140912173 mov      qword ptr [rdi + 0x18], 1
0x14091217b mov      eax, dword ptr [rsi + 0x1c]
0x14091217e mov      rcx, qword ptr [rsi + 0x10]
0x140912182 mov      dword ptr [rcx], eax
0x140912184 mov      rcx, qword ptr [rsi]
0x140912187 xor      r8d, r8d
0x14091218a call     0x140911520 ; country_core::systems::camera_rig::MainCamera::load_parameters::h09e6c823e4f65705
0x14091218f mov      eax, dword ptr [rdi + 0x18]
0x140912192 test     eax, eax
0x140912194 je       0x1409124a4
0x14091219a movss    xmm6, dword ptr [r12 + 0x28]
0x1409121a1 movss    xmm0, dword ptr [r13 + 0xec]
0x1409121aa mulss    xmm0, dword ptr [rip + 0x20137d2]
0x1409121b2 cmp      eax, 3
0x1409121b5 je       0x14091220a
0x1409121b7 cmp      eax, 2
0x1409121ba je       0x14091222c
0x1409121bc movss    xmm1, dword ptr [rdi + 0x1c]
0x1409121c1 divss    xmm1, dword ptr [rip + 0x20224c7]
0x1409121c9 mulss    xmm0, xmm1
0x1409121cd mulss    xmm6, xmm0
0x1409121d1 mov      eax, dword ptr [rsi + 0x1c]
0x1409121d4 mov      rcx, qword ptr [rsi + 0x10]
0x1409121d8 mov      dword ptr [rcx], eax
0x1409121da mov      rsi, qword ptr [rsi]
0x1409121dd lea      rdi, [rsi + 0x100]
0x1409121e4 movzx    ecx, byte ptr [rsi + 0x4d0]
0x1409121eb lea      rax, [rip + 0x237faa6]
0x1409121f2 mov      ebx, 0x8c
0x1409121f7 lea      rdx, [rip + 0x2199ad6]
0x1409121fe movsxd   rcx, dword ptr [rdx + rcx*4]
0x140912202 add      rcx, rdx
0x140912205 mov      rdx, rdi
0x140912208 jmp      rcx
0x14091220a movss    xmm2, dword ptr [rdi + 0x1c]
0x14091220f mulss    xmm2, dword ptr [rip + 0x2013739]
0x140912217 mulss    xmm2, xmm2
0x14091221b mulss    xmm2, xmm2
0x14091221f movss    xmm1, dword ptr [rip + 0x20136dd]
0x140912227 call     0x140c904d0 ; utils::lerp::hf3285295543ed03f
0x14091222c mulss    xmm6, xmm0
0x140912230 mov      eax, dword ptr [rsi + 0x1c]
0x140912233 mov      rcx, qword ptr [rsi + 0x10]
0x140912237 mov      dword ptr [rcx], eax
0x140912239 mov      rsi, qword ptr [rsi]
0x14091223c lea      rdi, [rsi + 0x100]
0x140912243 movzx    ecx, byte ptr [rsi + 0x4d0]
0x14091224a lea      rax, [rip + 0x237fa47]
0x140912251 mov      ebx, 0x8c
0x140912256 lea      rdx, [rip + 0x2199a77]
0x14091225d movsxd   rcx, dword ptr [rdx + rcx*4]
0x140912261 add      rcx, rdx
0x140912264 mov      rdx, rdi
0x140912267 jmp      rcx
0x140912269 lea      rdx, [rsi + 0x1b8]
0x140912270 lea      rax, [rip + 0x237fa21]
0x140912277 mov      ebx, 0x144
0x14091227c jmp      0x1409122d0 ; 
0x14091227e lea      rdx, [rsi + 0x390]
0x140912285 lea      rax, [rip + 0x237fa0c]
0x14091228c mov      ebx, 0x31c
0x140912291 jmp      0x1409122d0 ; 
0x140912293 lea      rdx, [rsi + 0x270]
0x14091229a lea      rax, [rip + 0x237f987]
0x1409122a1 mov      ebx, 0x1c4
0x1409122a6 jmp      0x1409122d0 ; 
0x1409122a8 lea      rdx, [rsi + 0x2f0]
0x1409122af lea      rax, [rip + 0x237f93a]
0x1409122b6 mov      ebx, 0x264
0x1409122bb jmp      0x1409122d0 ; 
0x1409122bd lea      rdx, [rsi + 0x448]
0x1409122c4 lea      rax, [rip + 0x237f995]
0x1409122cb mov      ebx, 0x3a4
0x1409122d0 lea      rcx, [rsp + 0x70]
0x1409122d5 movaps   xmm2, xmm6
0x1409122d8 call     qword ptr [rax + 0x30]
0x1409122db mov      eax, dword ptr [rsp + 0x78]
0x1409122df mov      dword ptr [rsp + 0x28], eax
0x1409122e3 mov      rax, qword ptr [rsp + 0x70]
0x1409122e8 mov      qword ptr [rsp + 0x20], rax
0x1409122ed movaps   xmm7, xmmword ptr [rsp + 0x80]
0x1409122f5 divss    xmm6, dword ptr [rip + 0x20cce6f]
0x1409122fd movaps   xmm0, xmm6
0x140912300 call     0x142923ec0 ; expf
0x140912305 movss    xmm1, dword ptr [rip + 0x20135f7]
0x14091230d movaps   xmm2, xmm1
0x140912310 subss    xmm2, xmm0
0x140912314 movsd    xmm0, qword ptr [rsi + rbx + 0x104]
0x14091231d subss    xmm1, xmm2
0x140912321 unpcklps xmm1, xmm2
0x140912324 mulps    xmm1, xmm0
0x140912327 movshdup xmm0, xmm1
0x14091232b addss    xmm0, xmm1
0x14091232f movss    dword ptr [rsi + rbx + 0x104], xmm0
0x140912338 movaps   xmmword ptr [rsp + 0x30], xmm7
0x14091233d movss    xmm0, dword ptr [rsi + 0x4d8]
0x140912345 movups   xmm1, xmmword ptr [rdi + rbx]
0x140912349 movups   xmm2, xmmword ptr [rdi + rbx + 0x10]
0x14091234e mov      rax, qword ptr [rdi + rbx + 0x20]
0x140912353 mov      ecx, dword ptr [rdi + rbx + 0x28]
0x140912357 mov      dword ptr [rsp + 0x6c], ecx
0x14091235b mov      qword ptr [rsp + 0x64], rax
0x140912360 movups   xmmword ptr [rsp + 0x54], xmm2
0x140912365 movups   xmmword ptr [rsp + 0x44], xmm1
0x14091236a movss    dword ptr [rsp + 0x40], xmm0
0x140912370 lea      rcx, [rsp + 0x170]
0x140912378 lea      rdx, [rsp + 0x20]
0x14091237d call     0x140c8ff70 ; _$LT$$LP$glam..f32..vec3..Vec3$C$glam..f32..sse2..quat..Quat$RP$$u20$as$u20$utils..camera..IntoCameraBodyMatrices$GT$::into_camera_body_matrices::h7c5971583caead3a
0x140912382 lea      rcx, [rsp + 0x1f0]
0x14091238a lea      rdx, [rsp + 0x40]
0x14091238f call     0x140c97390 ; utils::camera::CameraProjectionParams::calc_matrices::ha4168fdbac703f7b
0x140912394 movaps   xmm0, xmmword ptr [rsp + 0x1f0]
0x14091239c movaps   xmm1, xmmword ptr [rsp + 0x200]
0x1409123a4 movaps   xmm2, xmmword ptr [rsp + 0x210]
0x1409123ac movaps   xmm3, xmmword ptr [rsp + 0x220]
0x1409123b4 movaps   xmmword ptr [rsp + 0xa0], xmm3
0x1409123bc movaps   xmmword ptr [rsp + 0x90], xmm2
0x1409123c4 movaps   xmmword ptr [rsp + 0x80], xmm1
0x1409123cc movaps   xmmword ptr [rsp + 0x70], xmm0
0x1409123d1 movaps   xmm0, xmmword ptr [rsp + 0x230]
0x1409123d9 movaps   xmmword ptr [rsp + 0xb0], xmm0
0x1409123e1 movaps   xmm0, xmmword ptr [rsp + 0x240]
0x1409123e9 movaps   xmmword ptr [rsp + 0xc0], xmm0
0x1409123f1 movaps   xmm0, xmmword ptr [rsp + 0x250]
0x1409123f9 movaps   xmmword ptr [rsp + 0xd0], xmm0
0x140912401 movaps   xmm0, xmmword ptr [rsp + 0x260]
0x140912409 movaps   xmmword ptr [rsp + 0xe0], xmm0
0x140912411 movaps   xmm0, xmmword ptr [rsp + 0x170]
0x140912419 movaps   xmm1, xmmword ptr [rsp + 0x180]
0x140912421 movaps   xmm2, xmmword ptr [rsp + 0x190]
0x140912429 movaps   xmm3, xmmword ptr [rsp + 0x1a0]
0x140912431 movaps   xmmword ptr [rsp + 0xf0], xmm0
0x140912439 movaps   xmmword ptr [rsp + 0x100], xmm1
0x140912441 movaps   xmmword ptr [rsp + 0x110], xmm2
0x140912449 movaps   xmmword ptr [rsp + 0x120], xmm3
0x140912451 movaps   xmm0, xmmword ptr [rsp + 0x1b0]
0x140912459 movaps   xmm1, xmmword ptr [rsp + 0x1c0]
0x140912461 movaps   xmm2, xmmword ptr [rsp + 0x1d0]
0x140912469 movaps   xmm3, xmmword ptr [rsp + 0x1e0]
0x140912471 movaps   xmmword ptr [rsp + 0x160], xmm3
0x140912479 movaps   xmmword ptr [rsp + 0x150], xmm2
0x140912481 movaps   xmmword ptr [rsp + 0x140], xmm1
0x140912489 movaps   xmmword ptr [rsp + 0x130], xmm0
0x140912491 lea      rdx, [rsp + 0x70]
0x140912496 mov      r8d, 0x100
0x14091249c mov      rcx, rsi
0x14091249f call     0x142923c00 ; memcpy
0x1409124a4 movaps   xmm6, xmmword ptr [rsp + 0x270]
0x1409124ac movaps   xmm7, xmmword ptr [rsp + 0x280]
0x1409124b4 add      rsp, 0x290
0x1409124bb pop      rbx
0x1409124bc pop      rdi
0x1409124bd pop      rsi
0x1409124be pop      r12
0x1409124c0 pop      r13
0x1409124c2 pop      r14
0x1409124c4 pop      r15
0x1409124c6 ret      
0x1409124c7 lea      r8, [rip + 0x2199472]
0x1409124ce mov      rcx, rbx
0x1409124d1 mov      rdx, r14
0x1409124d4 call     0x1428d9081 ; alloc::vec::Vec$LT$T$C$A$GT$::insert::assert_failed::hb4d0af46048842f2
0x1409124d9 lea      r8, [rip + 0x2199490]
0x1409124e0 mov      rcx, rbx
0x1409124e3 mov      rdx, r14
0x1409124e6 call     0x1428d90e9 ; alloc::vec::Vec$LT$T$C$A$GT$::remove::assert_failed::h1bd6cfb1d4be152f
0x1409124eb lea      r8, [rip + 0x2199466]
0x1409124f2 mov      rcx, rbx
0x1409124f5 call     0x1428d9518 ; core::panicking::panic_bounds_check::h213dd6812833a70b
0x1409124fa int3     