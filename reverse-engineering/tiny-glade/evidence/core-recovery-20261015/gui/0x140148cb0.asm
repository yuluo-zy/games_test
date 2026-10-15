0x140148cb0 push     rbp
0x140148cb1 push     r15
0x140148cb3 push     r14
0x140148cb5 push     r13
0x140148cb7 push     r12
0x140148cb9 push     rsi
0x140148cba push     rdi
0x140148cbb push     rbx
0x140148cbc sub      rsp, 0x408
0x140148cc3 lea      rbp, [rsp + 0x80]
0x140148ccb mov      qword ptr [rbp + 0x380], 0xfffffffffffffffe
0x140148cd6 mov      rbx, r8
0x140148cd9 mov      rdi, rdx
0x140148cdc mov      qword ptr [rbp + 0x370], rcx
0x140148ce3 movzx    eax, byte ptr [rip + 0x34ebf1f]
0x140148cea mov      ecx, 8
0x140148cef mov      edx, 8
0x140148cf4 call     0x140613c10 ; __rust_alloc
0x140148cf9 test     rax, rax
0x140148cfc je       0x140149645
0x140148d02 lea      rcx, [rdi + 0xec]
0x140148d09 mov      qword ptr [rbp + 0x368], rax
0x140148d10 mov      qword ptr [rax], rcx
0x140148d13 lea      rcx, [rbp + 0xf0]
0x140148d1a call     0x140c34750 ; _$LT$egui..widget_text..WidgetText$u20$as$u20$core..default..Default$GT$::default::h1fb81403bbfc71e5
0x140148d1f mov      qword ptr [rbp + 0x260], 0
0x140148d2a mov      qword ptr [rbp + 0x268], 1
0x140148d35 xorps    xmm0, xmm0
0x140148d38 movups   xmmword ptr [rbp + 0x270], xmm0
0x140148d3f mov      qword ptr [rbp + 0x280], 1
0x140148d4a mov      qword ptr [rbp + 0x288], 0
0x140148d55 movups   xmm1, xmmword ptr [rbp + 0xf0]
0x140148d5c movups   xmm2, xmmword ptr [rbp + 0x100]
0x140148d63 movups   xmm3, xmmword ptr [rbp + 0x110]
0x140148d6a movups   xmm4, xmmword ptr [rbp + 0x120]
0x140148d71 movups   xmmword ptr [rbp - 0x50], xmm1
0x140148d75 movups   xmmword ptr [rbp - 0x40], xmm2
0x140148d79 movups   xmmword ptr [rbp - 0x30], xmm3
0x140148d7d movups   xmmword ptr [rbp - 0x20], xmm4
0x140148d81 movups   xmm1, xmmword ptr [rbp + 0x130]
0x140148d88 movups   xmmword ptr [rbp - 0x10], xmm1
0x140148d8c movups   xmm1, xmmword ptr [rbp + 0x140]
0x140148d93 movups   xmmword ptr [rbp], xmm1
0x140148d97 movups   xmm1, xmmword ptr [rbp + 0x150]
0x140148d9e movups   xmmword ptr [rbp + 0x10], xmm1
0x140148da2 movups   xmmword ptr [rbp + 0x2d8], xmm0
0x140148da9 mov      qword ptr [rbp + 0x1c0], 0
0x140148db4 mov      qword ptr [rbp + 0x1d0], 0
0x140148dbf mov      qword ptr [rbp + 0x1e0], 0
0x140148dca movups   xmm1, xmmword ptr [rbp - 0x58]
0x140148dce movups   xmm2, xmmword ptr [rbp - 0x48]
0x140148dd2 movups   xmm3, xmmword ptr [rbp - 0x38]
0x140148dd6 movups   xmm4, xmmword ptr [rbp - 0x28]
0x140148dda movups   xmmword ptr [rbp + 0x1e8], xmm1
0x140148de1 movups   xmmword ptr [rbp + 0x1f8], xmm2
0x140148de8 movups   xmmword ptr [rbp + 0x208], xmm3
0x140148def movups   xmmword ptr [rbp + 0x218], xmm4
0x140148df6 movups   xmm1, xmmword ptr [rbp - 0x18]
0x140148dfa movups   xmmword ptr [rbp + 0x228], xmm1
0x140148e01 movups   xmm1, xmmword ptr [rbp - 8]
0x140148e05 movups   xmmword ptr [rbp + 0x238], xmm1
0x140148e0c movups   xmm1, xmmword ptr [rbp + 8]
0x140148e10 movups   xmmword ptr [rbp + 0x248], xmm1
0x140148e17 mov      rax, qword ptr [rbp + 0x18]
0x140148e1b mov      qword ptr [rbp + 0x258], rax
0x140148e22 mov      dword ptr [rbp + 0x290], 2
0x140148e2c movhps   xmm0, qword ptr [rip + 0x2807f8d]
0x140148e33 movups   xmmword ptr [rbp + 0x298], xmm0
0x140148e3a mov      byte ptr [rbp + 0x2a8], 0
0x140148e41 movaps   xmm0, xmmword ptr [rip + 0x27eb788]
0x140148e48 movups   xmmword ptr [rbp + 0x2b0], xmm0
0x140148e4f mov      byte ptr [rbp + 0x2c0], 0
0x140148e56 mov      rax, qword ptr [rbp + 0x368]
0x140148e5d mov      qword ptr [rbp + 0x2c8], rax
0x140148e64 lea      rax, [rip + 0x28b922d]
0x140148e6b mov      qword ptr [rbp + 0x2d0], rax
0x140148e72 mov      qword ptr [rbp + 0x2f0], 0
0x140148e7d mov      dword ptr [rbp + 0x300], 0x10101
0x140148e87 mov      byte ptr [rbp + 0x304], 2
0x140148e8e movzx    eax, byte ptr [rip + 0x34ebd74]
0x140148e95 mov      ecx, 0x13
0x140148e9a mov      edx, 1
0x140148e9f call     0x140613c10 ; __rust_alloc
0x140148ea4 test     rax, rax
0x140148ea7 je       0x140149654
0x140148ead movups   xmm0, xmmword ptr [rip + 0x280855c] ; @0x142951410 'Interpolation SpeedSave Camera PositionCamera snapshot saved.Press F8 to resore it.Press F8 to switc'
0x140148eb4 movups   xmmword ptr [rax], xmm0
0x140148eb7 mov      dword ptr [rax + 0xf], 0x64656570
0x140148ebe mov      dword ptr [rbp + 0x164], 0
0x140148ec8 mov      dword ptr [rbp + 0x167], 0
0x140148ed2 mov      rsi, qword ptr [rbp + 0x1f0]
0x140148ed9 mov      ecx, esi
0x140148edb and      ecx, 0xfffffffe
0x140148ede lea      rdx, [rsi - 3]
0x140148ee2 xor      r8d, r8d
0x140148ee5 cmp      ecx, 4
0x140148ee8 cmove    r8, rdx
0x140148eec test     r8, r8
0x140148eef mov      qword ptr [rbp + 0x368], rax
0x140148ef6 je       0x140148f31
0x140148ef8 cmp      r8, 1
0x140148efc jne      0x140148f9f
0x140148f02 mov      rdx, qword ptr [rbp + 0x1f8]
0x140148f09 test     rdx, rdx
0x140148f0c je       0x140148f20
0x140148f0e mov      rcx, qword ptr [rbp + 0x200]
0x140148f15 mov      r8d, 1
0x140148f1b call     0x140613c20 ; __rust_dealloc
0x140148f20 lea      rcx, [rbp + 0x210]
0x140148f27 call     0x140608f80 ; core::ptr::drop_in_place$LT$alloc..vec..Vec$LT$epaint..text..text_layout_types..LayoutSection$GT$$GT$::hb247fd76ce4de2e9 (.llvm.12095432060108768980) / core::ptr::drop_in_place$LT$alloc..vec..Vec$LT$epaint..text..text_layout_types..LayoutSection$GT$$GT$::h284850123c6b7642 (.llvm.10346396820423203432)
0x140148f2c jmp      0x140148fb8 ; 
0x140148f31 mov      rdx, qword ptr [rbp + 0x220]
0x140148f38 test     rdx, rdx
0x140148f3b je       0x140148f4f
0x140148f3d mov      rcx, qword ptr [rbp + 0x228]
0x140148f44 mov      r8d, 1
0x140148f4a call     0x140613c20 ; __rust_dealloc
0x140148f4f cmp      esi, 3
0x140148f52 je       0x140148f72
0x140148f54 cmp      esi, 2
0x140148f57 jb       0x140148f72
0x140148f59 mov      rax, qword ptr [rbp + 0x1f8]
0x140148f60 lock dec qword ptr [rax]
0x140148f64 jne      0x140148f72
0x140148f66 lea      rcx, [rbp + 0x1f8]
0x140148f6d call     0x14064eb30 ; alloc::sync::Arc$LT$T$C$A$GT$::drop_slow::h5672070ce44dd4cf / alloc::sync::Arc$LT$T$C$A$GT$::drop_slow::h8e637e57b8f96748
0x140148f72 mov      rax, qword ptr [rbp + 0x208]
0x140148f79 cmp      rax, 6
0x140148f7d je       0x140148fb8
0x140148f7f cmp      eax, 5
0x140148f82 jb       0x140148fb8
0x140148f84 mov      rax, qword ptr [rbp + 0x210]
0x140148f8b lock dec qword ptr [rax]
0x140148f8f jne      0x140148fb8
0x140148f91 lea      rcx, [rbp + 0x210]
0x140148f98 call     0x14064eb30 ; alloc::sync::Arc$LT$T$C$A$GT$::drop_slow::h5672070ce44dd4cf / alloc::sync::Arc$LT$T$C$A$GT$::drop_slow::h8e637e57b8f96748
0x140148f9d jmp      0x140148fb8 ; 
0x140148f9f mov      rax, qword ptr [rbp + 0x1f8]
0x140148fa6 lock dec qword ptr [rax]
0x140148faa jne      0x140148fb8
0x140148fac lea      rcx, [rbp + 0x1f8]
0x140148fb3 call     0x140c7de90 ; alloc::sync::Arc$LT$T$C$A$GT$::drop_slow::h88bf8f93b401f9b7
0x140148fb8 mov      qword ptr [rbp + 0x1f0], 3
0x140148fc3 movups   xmm0, xmmword ptr [rbp - 0x58]
0x140148fc7 movups   xmmword ptr [rbp + 0x1f8], xmm0
0x140148fce mov      qword ptr [rbp + 0x208], 6
0x140148fd9 movups   xmm0, xmmword ptr [rbp + 0xf0]
0x140148fe0 movups   xmmword ptr [rbp + 0x210], xmm0
0x140148fe7 mov      qword ptr [rbp + 0x220], 0x13
0x140148ff2 mov      rax, qword ptr [rbp + 0x368]
0x140148ff9 mov      qword ptr [rbp + 0x228], rax
0x140149000 mov      qword ptr [rbp + 0x230], 0x13
0x14014900b mov      dword ptr [rbp + 0x238], 0
0x140149015 mov      dword ptr [rbp + 0x240], 0
0x14014901f mov      qword ptr [rbp + 0x248], 0
0x14014902a mov      byte ptr [rbp + 0x250], 0
0x140149031 mov      rax, qword ptr [rbp + 0x160]
0x140149038 mov      qword ptr [rbp + 0x251], rax
0x14014903f mov      eax, dword ptr [rbp + 0x167]
0x140149045 mov      dword ptr [rbp + 0x258], eax
0x14014904b lea      r14, [rbp - 0x58]
0x14014904f lea      rdx, [rbp + 0x1c0]
0x140149056 mov      r8d, 0x148
0x14014905c mov      rcx, r14
0x14014905f call     0x142923c00 ; memcpy
0x140149064 lea      rcx, [rbp + 0x1c0]
0x14014906b mov      rdx, r14
0x14014906e mov      r8, qword ptr [rbp + 0x370]
0x140149075 call     0x140c77160 ; _$LT$egui..widgets..slider..Slider$u20$as$u20$egui..widgets..Widget$GT$::ui::h1e78a398b6556716
0x14014907a mov      rax, qword ptr [rbp + 0x1d0]
0x140149081 lock dec qword ptr [rax]
0x140149085 jne      0x140149093
0x140149087 lea      rcx, [rbp + 0x1d0]
0x14014908e call     0x140c41170 ; alloc::sync::Arc$LT$T$C$A$GT$::drop_slow::he9d75ed3b56676ff
0x140149093 mov      qword ptr [rbp - 0x58], 4
0x14014909b mov      byte ptr [rbp + 0x37f], 0
0x1401490a2 lea      rdx, [rip + 0x280837a] ; @0x142951423 'Save Camera PositionCamera snapshot saved.Press F8 to resore it.Press F8 to switch between snapshots'
0x1401490a9 lea      rcx, [rbp + 0x1c0]
0x1401490b0 mov      r8d, 0x14
0x1401490b6 call     0x1400568d0 ; _$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$::into::h83d03329215035d5 (.llvm.12463876434711410378)
0x1401490bb movups   xmm0, xmmword ptr [rbp + 0x220]
0x1401490c2 movaps   xmmword ptr [rbp + 0x150], xmm0
0x1401490c9 movups   xmm0, xmmword ptr [rbp + 0x210]
0x1401490d0 movaps   xmmword ptr [rbp + 0x140], xmm0
0x1401490d7 movups   xmm0, xmmword ptr [rbp + 0x200]
0x1401490de movaps   xmmword ptr [rbp + 0x130], xmm0
0x1401490e5 movups   xmm0, xmmword ptr [rbp + 0x1c0]
0x1401490ec movups   xmm1, xmmword ptr [rbp + 0x1d0]
0x1401490f3 movups   xmm2, xmmword ptr [rbp + 0x1e0]
0x1401490fa movups   xmm3, xmmword ptr [rbp + 0x1f0]
0x140149101 movaps   xmmword ptr [rbp + 0x120], xmm3
0x140149108 movaps   xmmword ptr [rbp + 0x110], xmm2
0x14014910f movaps   xmmword ptr [rbp + 0x100], xmm1
0x140149116 movaps   xmmword ptr [rbp + 0xf0], xmm0
0x14014911d mov      byte ptr [rbp + 0x37f], 1
0x140149124 lea      rcx, [rbp + 0x1c0]
0x14014912b lea      r8, [rbp + 0xf0]
0x140149132 mov      rdx, r14
0x140149135 call     0x140c46bd0 ; egui::widgets::button::Button::opt_image_and_text::hdd7a18fc15f55700
0x14014913a lea      rcx, [rbp + 0x160]
0x140149141 lea      rdx, [rbp + 0x1c0]
0x140149148 mov      r8, qword ptr [rbp + 0x370]
0x14014914f call     0x140c46d20 ; _$LT$egui..widgets..button..Button$u20$as$u20$egui..widgets..Widget$GT$::ui::ha728e61814580cc6
0x140149154 cmp      byte ptr [rbp + 0x1b4], 0
0x14014915b je       0x14014917f
0x14014915d mov      rax, qword ptr [rbp + 0x170]
0x140149164 lock dec qword ptr [rax]
0x140149168 jne      0x140149176
0x14014916a lea      rcx, [rbp + 0x170]
0x140149171 call     0x140c41170 ; alloc::sync::Arc$LT$T$C$A$GT$::drop_slow::he9d75ed3b56676ff
0x140149176 mov      qword ptr [rdi], 1
0x14014917d jmp      0x1401491bf ; 
0x14014917f mov      byte ptr [rbp + 0x1c0], 0
0x140149186 cmp      byte ptr [rbp + 0x1b3], 0
0x14014918d je       0x1401491a6
0x14014918f lea      rcx, [rbp + 0x170]
0x140149196 lea      rdx, [rbp + 0x1c0]
0x14014919d call     0x140042870 ; egui::context::Context::write::h9406d0def1a52fff / egui::context::Context::write::hd1da45529f868497
0x1401491a2 test     al, al
0x1401491a4 jne      0x14014915d
0x1401491a6 mov      rax, qword ptr [rbp + 0x170]
0x1401491ad lock dec qword ptr [rax]
0x1401491b1 jne      0x1401491bf
0x1401491b3 lea      rcx, [rbp + 0x170]
0x1401491ba call     0x140c41170 ; alloc::sync::Arc$LT$T$C$A$GT$::drop_slow::he9d75ed3b56676ff
0x1401491bf mov      r13, qword ptr [rbx + 0x10]
0x1401491c3 test     r13, r13
0x1401491c6 je       0x1401495bd
0x1401491cc mov      qword ptr [rbp + 0x368], rbx
0x1401491d3 add      rbx, 0x20
0x1401491d7 lea      r15, [rip + 0x2807d1a]
0x1401491de lea      r12, [rbp + 0x1c0]
0x1401491e5 xor      esi, esi
0x1401491e7 jmp      0x1401491f8 ; 
0x1401491e9 nop      dword ptr [rax]
0x1401491f0 inc      rsi
0x1401491f3 cmp      r13, rsi
0x1401491f6 je       0x14014925b
0x1401491f8 mov      qword ptr [rbp - 0x58], rsi
0x1401491fc movzx    eax, byte ptr [rip + 0x34eba06]
0x140149203 mov      ecx, 0x18
0x140149208 mov      edx, 8
0x14014920d call     0x140613c10 ; __rust_alloc
0x140149212 test     rax, rax
0x140149215 je       0x140149636
0x14014921b mov      qword ptr [rax], r14
0x14014921e mov      qword ptr [rax + 8], rbx
0x140149222 mov      qword ptr [rax + 0x10], rdi
0x140149226 mov      qword ptr [rsp + 0x20], r15
0x14014922b mov      rcx, r12
0x14014922e mov      rdx, qword ptr [rbp + 0x370]
0x140149235 xor      r8d, r8d
0x140149238 mov      r9, rax
0x14014923b call     0x1400c4230 ; egui::ui::Ui::horizontal_with_main_wrap_dyn::h07a52f58023f48eb
0x140149240 mov      rax, qword ptr [rbp + 0x1d0]
0x140149247 lock dec qword ptr [rax]
0x14014924b jne      0x1401491f0
0x14014924d lea      rcx, [rbp + 0x1d0]
0x140149254 call     0x140c41170 ; alloc::sync::Arc$LT$T$C$A$GT$::drop_slow::he9d75ed3b56676ff
0x140149259 jmp      0x1401491f0 ; 
0x14014925b mov      rbx, qword ptr [rbp + 0x368]
0x140149262 mov      rax, qword ptr [rbx + 0x10]
0x140149266 cmp      rax, 1
0x14014926a jbe      0x1401493ea
0x140149270 movzx    eax, byte ptr [rip + 0x34eb992]
0x140149277 mov      ecx, 0x16
0x14014927c mov      edx, 1
0x140149281 call     0x140613c10 ; __rust_alloc
0x140149286 test     rax, rax
0x140149289 je       0x14014967b
0x14014928f movups   xmm0, xmmword ptr [rip + 0x28081a1] ; @0x142951437 'Camera snapshot saved.Press F8 to resore it.Press F8 to switch between snapshots. '
0x140149296 movups   xmmword ptr [rax], xmm0
0x140149299 movabs   rcx, 0x2e64657661732074
0x1401492a3 mov      qword ptr [rax + 0xe], rcx
0x1401492a7 mov      byte ptr [rbp + 0x220], 0
0x1401492ae mov      dword ptr [rbp + 0x225], 0
0x1401492b8 mov      dword ptr [rbp + 0x228], 0
0x1401492c2 mov      qword ptr [rbp + 0x1c0], 3
0x1401492cd mov      qword ptr [rbp + 0x1d8], 6
0x1401492d8 mov      qword ptr [rbp + 0x1f0], 0x16
0x1401492e3 mov      qword ptr [rbp + 0x1f8], rax
0x1401492ea mov      qword ptr [rbp + 0x200], 0x16
0x1401492f5 mov      dword ptr [rbp + 0x208], 0
0x1401492ff mov      dword ptr [rbp + 0x210], 0
0x140149309 mov      qword ptr [rbp + 0x218], 0
0x140149314 mov      word ptr [rbp + 0x230], 0x200
0x14014931d mov      byte ptr [rbp + 0x232], 2
0x140149324 mov      byte ptr [rbp + 0x235], 2
0x14014932b lea      rcx, [rbp - 0x58]
0x14014932f lea      rdx, [rbp + 0x1c0]
0x140149336 mov      r8, qword ptr [rbp + 0x370]
0x14014933d call     0x140c28d40 ; _$LT$egui..widgets..label..Label$u20$as$u20$egui..widgets..Widget$GT$::ui::hbba1bb39743e9d2d
0x140149342 mov      rax, qword ptr [rbp - 0x48]
0x140149346 lock dec qword ptr [rax]
0x14014934a jne      0x140149355
0x14014934c lea      rcx, [rbp - 0x48]
0x140149350 call     0x140c41170 ; alloc::sync::Arc$LT$T$C$A$GT$::drop_slow::he9d75ed3b56676ff
0x140149355 movzx    eax, byte ptr [rip + 0x34eb8ad]
0x14014935c mov      ecx, 0x25
0x140149361 mov      edx, 1
0x140149366 call     0x140613c10 ; __rust_alloc
0x14014936b test     rax, rax
0x14014936e je       0x140149691
0x140149374 movups   xmm0, xmmword ptr [rip + 0x28080f8] ; @0x142951473 'ch between snapshots. '
0x14014937b movups   xmmword ptr [rax + 0x10], xmm0
0x14014937f movups   xmm0, xmmword ptr [rip + 0x28080dd] ; @0x142951463 'Press F8 to switch between snapshots. '
0x140149386 movups   xmmword ptr [rax], xmm0
0x140149389 movabs   rcx, 0x2e73746f68737061
0x140149393 mov      qword ptr [rax + 0x1d], rcx
0x140149397 mov      byte ptr [rbp + 0x220], 0
0x14014939e mov      dword ptr [rbp + 0x225], 0
0x1401493a8 mov      dword ptr [rbp + 0x228], 0
0x1401493b2 mov      qword ptr [rbp + 0x1c0], 3
0x1401493bd mov      qword ptr [rbp + 0x1d8], 6
0x1401493c8 mov      qword ptr [rbp + 0x1f0], 0x25
0x1401493d3 mov      qword ptr [rbp + 0x1f8], rax
0x1401493da mov      qword ptr [rbp + 0x200], 0x25
0x1401493e5 jmp      0x14014955d ; 
0x1401493ea test     rax, rax
0x1401493ed je       0x1401495bd
0x1401493f3 movzx    eax, byte ptr [rip + 0x34eb80f]
0x1401493fa mov      ecx, 0x16
0x1401493ff mov      edx, 1
0x140149404 call     0x140613c10 ; __rust_alloc
0x140149409 test     rax, rax
0x14014940c je       0x14014967b
0x140149412 movups   xmm0, xmmword ptr [rip + 0x280801e] ; @0x142951437 'Camera snapshot saved.Press F8 to resore it.Press F8 to switch between snapshots. '
0x140149419 movups   xmmword ptr [rax], xmm0
0x14014941c movabs   rcx, 0x2e64657661732074
0x140149426 mov      qword ptr [rax + 0xe], rcx
0x14014942a mov      byte ptr [rbp + 0x220], 0
0x140149431 mov      dword ptr [rbp + 0x225], 0
0x14014943b mov      dword ptr [rbp + 0x228], 0
0x140149445 mov      qword ptr [rbp + 0x1c0], 3
0x140149450 mov      qword ptr [rbp + 0x1d8], 6
0x14014945b mov      qword ptr [rbp + 0x1f0], 0x16
0x140149466 mov      qword ptr [rbp + 0x1f8], rax
0x14014946d mov      qword ptr [rbp + 0x200], 0x16
0x140149478 mov      dword ptr [rbp + 0x208], 0
0x140149482 mov      dword ptr [rbp + 0x210], 0
0x14014948c mov      qword ptr [rbp + 0x218], 0
0x140149497 mov      word ptr [rbp + 0x230], 0x200
0x1401494a0 mov      byte ptr [rbp + 0x232], 2
0x1401494a7 mov      byte ptr [rbp + 0x235], 2
0x1401494ae lea      rcx, [rbp - 0x58]
0x1401494b2 lea      rdx, [rbp + 0x1c0]
0x1401494b9 mov      r8, qword ptr [rbp + 0x370]
0x1401494c0 call     0x140c28d40 ; _$LT$egui..widgets..label..Label$u20$as$u20$egui..widgets..Widget$GT$::ui::hbba1bb39743e9d2d
0x1401494c5 mov      rax, qword ptr [rbp - 0x48]
0x1401494c9 lock dec qword ptr [rax]
0x1401494cd jne      0x1401494d8
0x1401494cf lea      rcx, [rbp - 0x48]
0x1401494d3 call     0x140c41170 ; alloc::sync::Arc$LT$T$C$A$GT$::drop_slow::he9d75ed3b56676ff
0x1401494d8 movzx    eax, byte ptr [rip + 0x34eb72a]
0x1401494df mov      ecx, 0x16
0x1401494e4 mov      edx, 1
0x1401494e9 call     0x140613c10 ; __rust_alloc
0x1401494ee test     rax, rax
0x1401494f1 je       0x14014967b
0x1401494f7 movups   xmm0, xmmword ptr [rip + 0x2807f4f] ; @0x14295144d 'Press F8 to resore it.Press F8 to switch between snapshots. '
0x1401494fe movups   xmmword ptr [rax], xmm0
0x140149501 movabs   rcx, 0x2e74692065726f73
0x14014950b mov      qword ptr [rax + 0xe], rcx
0x14014950f mov      byte ptr [rbp + 0x220], 0
0x140149516 mov      dword ptr [rbp + 0x225], 0
0x140149520 mov      dword ptr [rbp + 0x228], 0
0x14014952a mov      qword ptr [rbp + 0x1c0], 3
0x140149535 mov      qword ptr [rbp + 0x1d8], 6
0x140149540 mov      qword ptr [rbp + 0x1f0], 0x16
0x14014954b mov      qword ptr [rbp + 0x1f8], rax
0x140149552 mov      qword ptr [rbp + 0x200], 0x16
0x14014955d mov      dword ptr [rbp + 0x208], 0
0x140149567 mov      dword ptr [rbp + 0x210], 0
0x140149571 mov      qword ptr [rbp + 0x218], 0
0x14014957c mov      word ptr [rbp + 0x230], 0x200
0x140149585 mov      byte ptr [rbp + 0x232], 2
0x14014958c mov      byte ptr [rbp + 0x235], 2
0x140149593 lea      rcx, [rbp - 0x58]
0x140149597 lea      rdx, [rbp + 0x1c0]
0x14014959e mov      r8, qword ptr [rbp + 0x370]
0x1401495a5 call     0x140c28d40 ; _$LT$egui..widgets..label..Label$u20$as$u20$egui..widgets..Widget$GT$::ui::hbba1bb39743e9d2d
0x1401495aa mov      rax, qword ptr [rbp - 0x48]
0x1401495ae lock dec qword ptr [rax]
0x1401495b2 jne      0x1401495bd
0x1401495b4 lea      rcx, [rbp - 0x48]
0x1401495b8 call     0x140c41170 ; alloc::sync::Arc$LT$T$C$A$GT$::drop_slow::he9d75ed3b56676ff
0x1401495bd movzx    eax, byte ptr [rip + 0x34eb645]
0x1401495c4 mov      ecx, 0x10
0x1401495c9 mov      edx, 8
0x1401495ce call     0x140613c10 ; __rust_alloc
0x1401495d3 test     rax, rax
0x1401495d6 je       0x14014966c
0x1401495dc mov      qword ptr [rax], rbx
0x1401495df mov      qword ptr [rax + 8], rdi
0x1401495e3 lea      rcx, [rip + 0x2807a8e]
0x1401495ea mov      qword ptr [rsp + 0x20], rcx
0x1401495ef lea      rcx, [rbp + 0x1c0]
0x1401495f6 mov      rdx, qword ptr [rbp + 0x370]
0x1401495fd xor      r8d, r8d
0x140149600 mov      r9, rax
0x140149603 call     0x1400c4230 ; egui::ui::Ui::horizontal_with_main_wrap_dyn::h07a52f58023f48eb
0x140149608 mov      rax, qword ptr [rbp + 0x1d0]
0x14014960f lock dec qword ptr [rax]
0x140149613 jne      0x140149621
0x140149615 lea      rcx, [rbp + 0x1d0]
0x14014961c call     0x140c41170 ; alloc::sync::Arc$LT$T$C$A$GT$::drop_slow::he9d75ed3b56676ff
0x140149621 nop      
0x140149622 add      rsp, 0x408
0x140149629 pop      rbx
0x14014962a pop      rdi
0x14014962b pop      rsi
0x14014962c pop      r12
0x14014962e pop      r13
0x140149630 pop      r14
0x140149632 pop      r15
0x140149634 pop      rbp
0x140149635 ret      
0x140149636 mov      ecx, 8
0x14014963b mov      edx, 0x18
0x140149640 call     0x1428d9000 ; alloc::alloc::handle_alloc_error::h3e7daf9bcd04547a
0x140149645 mov      ecx, 8
0x14014964a mov      edx, 8
0x14014964f call     0x1428d9000 ; alloc::alloc::handle_alloc_error::h3e7daf9bcd04547a
0x140149654 lea      r8, [rip + 0x2807acd]
0x14014965b mov      ecx, 1
0x140149660 mov      edx, 0x13
0x140149665 call     0x1428d8fe3 ; alloc::raw_vec::handle_error::hb99beac7784c4464
0x14014966a ud2      
0x14014966c mov      ecx, 8
0x140149671 mov      edx, 0x10
0x140149676 call     0x1428d9000 ; alloc::alloc::handle_alloc_error::h3e7daf9bcd04547a
0x14014967b lea      r8, [rip + 0x2807aa6]
0x140149682 mov      ecx, 1
0x140149687 mov      edx, 0x16
0x14014968c call     0x1428d8fe3 ; alloc::raw_vec::handle_error::hb99beac7784c4464
0x140149691 lea      r8, [rip + 0x2807a90]
0x140149698 mov      ecx, 1
0x14014969d mov      edx, 0x25
0x1401496a2 call     0x1428d8fe3 ; alloc::raw_vec::handle_error::hb99beac7784c4464
0x1401496a7 int3     
0x1401496a8 nop      dword ptr [rax + rax]
0x1401496b0 mov      qword ptr [rsp + 0x10], rdx
0x1401496b5 push     rbp
0x1401496b6 push     r15
0x1401496b8 push     r14
0x1401496ba push     r13
0x1401496bc push     r12
0x1401496be push     rsi
0x1401496bf push     rdi
0x1401496c0 push     rbx
0x1401496c1 sub      rsp, 0x28
0x1401496c5 lea      rbp, [rdx + 0x80]
0x1401496cc lea      rcx, [rbp + 0x1c0]
0x1401496d3 call     0x1405ec430 ; core::ptr::drop_in_place$LT$egui..widgets..slider..Slider$GT$::hce4bcb5ac07e40a9
0x1401496d8 nop      
0x1401496d9 add      rsp, 0x28
0x1401496dd pop      rbx
0x1401496de pop      rdi
0x1401496df pop      rsi
0x1401496e0 pop      r12
0x1401496e2 pop      r13
0x1401496e4 pop      r14
0x1401496e6 pop      r15
0x1401496e8 pop      rbp
0x1401496e9 ret      
0x1401496ea nop      word ptr [rax + rax]
0x1401496f0 mov      qword ptr [rsp + 0x10], rdx
0x1401496f5 push     rbp
0x1401496f6 push     r15
0x1401496f8 push     r14
0x1401496fa push     r13
0x1401496fc push     r12
0x1401496fe push     rsi
0x1401496ff push     rdi
0x140149700 push     rbx
0x140149701 sub      rsp, 0x28
0x140149705 lea      rbp, [rdx + 0x80]
0x14014970c cmp      byte ptr [rbp + 0x37f], 0
0x140149713 jne      0x140149725
0x140149715 cmp      qword ptr [rbp - 0x58], 4
0x14014971a je       0x140149725
0x14014971c lea      rcx, [rbp - 0x58]
0x140149720 call     0x1405ea2f0 ; core::ptr::drop_in_place$LT$egui..widgets..image..Image$GT$::h14e0db96575526f3 / core::ptr::drop_in_place$LT$egui..widgets..image..Image$GT$::he79b48ffe16f7f22
0x140149725 nop      
0x140149726 add      rsp, 0x28
0x14014972a pop      rbx
0x14014972b pop      rdi
0x14014972c pop      rsi
0x14014972d pop      r12
0x14014972f pop      r13
0x140149731 pop      r14
0x140149733 pop      r15
0x140149735 pop      rbp
0x140149736 ret      
0x140149737 nop      word ptr [rax + rax]
0x140149740 mov      qword ptr [rsp + 0x10], rdx
0x140149745 push     rbp
0x140149746 push     r15
0x140149748 push     r14
0x14014974a push     r13
0x14014974c push     r12
0x14014974e push     rsi
0x14014974f push     rdi
0x140149750 push     rbx
0x140149751 sub      rsp, 0x28
0x140149755 lea      rbp, [rdx + 0x80]
0x14014975c lea      rcx, [rbp + 0x170]
0x140149763 mov      rax, qword ptr [rbp + 0x170]
0x14014976a lock dec qword ptr [rax]
0x14014976e jne      0x140149775
0x140149770 call     0x140c41170 ; alloc::sync::Arc$LT$T$C$A$GT$::drop_slow::he9d75ed3b56676ff
0x140149775 nop      
0x140149776 add      rsp, 0x28
0x14014977a pop      rbx
0x14014977b pop      rdi
0x14014977c pop      rsi
0x14014977d pop      r12
0x14014977f pop      r13
0x140149781 pop      r14
0x140149783 pop      r15
0x140149785 pop      rbp
0x140149786 ret      
0x140149787 nop      word ptr [rax + rax]
0x140149790 mov      qword ptr [rsp + 0x10], rdx
0x140149795 push     rbp
0x140149796 push     r15
0x140149798 push     r14
0x14014979a push     r13
0x14014979c push     r12
0x14014979e push     rsi
0x14014979f push     rdi
0x1401497a0 push     rbx
0x1401497a1 sub      rsp, 0x28
0x1401497a5 lea      rbp, [rdx + 0x80]
0x1401497ac lea      rdx, [rip + 0x28b88e5]
0x1401497b3 mov      rcx, qword ptr [rbp + 0x368]
0x1401497ba call     0x14002e900 ; core::ptr::drop_in_place$LT$alloc..boxed..Box$LT$dyn$u20$image..image..ImageDecoder$GT$$GT$::h9ba56180edc32c8c (.llvm.1100611113890887402) / core::ptr::drop_in_place$LT$alloc..boxed..Box$LT$dyn$u20$image..image..ImageDecoder$GT$$GT$::h4ca9b6c0ff719077 (.llvm.5988595327333108083)
0x1401497bf nop      
0x1401497c0 add      rsp, 0x28
0x1401497c4 pop      rbx
0x1401497c5 pop      rdi
0x1401497c6 pop      rsi
0x1401497c7 pop      r12
0x1401497c9 pop      r13
0x1401497cb pop      r14
0x1401497cd pop      r15
0x1401497cf pop      rbp
0x1401497d0 ret      
0x1401497d1 nop      word ptr cs:[rax + rax]
0x1401497e0 mov      qword ptr [rsp + 0x10], rdx
0x1401497e5 push     rbp
0x1401497e6 push     r15
0x1401497e8 push     r14
0x1401497ea push     r13
0x1401497ec push     r12
0x1401497ee push     rsi
0x1401497ef push     rdi
0x1401497f0 push     rbx
0x1401497f1 sub      rsp, 0x28
0x1401497f5 lea      rbp, [rdx + 0x80]
0x1401497fc lea      rcx, [rbp + 0x208]
0x140149803 call     0x1405fdb50 ; core::ptr::drop_in_place$LT$core..option..Option$LT$egui..style..TextStyle$GT$$GT$::h520bce03b9e9c449 (.llvm.7124148353756482180) / core::ptr::drop_in_place$LT$core..option..Option$LT$egui..style..TextStyle$GT$$GT$::h520bce03b9e9c449 (.llvm.15948950018123943476)
0x140149808 nop      
0x140149809 add      rsp, 0x28
0x14014980d pop      rbx
0x14014980e pop      rdi
0x14014980f pop      rsi
0x140149810 pop      r12
0x140149812 pop      r13
0x140149814 pop      r14
0x140149816 pop      r15
0x140149818 pop      rbp
0x140149819 ret      
0x14014981a nop      word ptr [rax + rax]
0x140149820 mov      qword ptr [rsp + 0x10], rdx
0x140149825 push     rbp
0x140149826 push     r15
0x140149828 push     r14
0x14014982a push     r13
0x14014982c push     r12
0x14014982e push     rsi
0x14014982f push     rdi
0x140149830 push     rbx
0x140149831 sub      rsp, 0x28
0x140149835 lea      rbp, [rdx + 0x80]
0x14014983c mov      qword ptr [rbp + 0x1f0], 3
0x140149847 movups   xmm0, xmmword ptr [rbp - 0x58]
0x14014984b movups   xmmword ptr [rbp + 0x1f8], xmm0
0x140149852 mov      qword ptr [rbp + 0x208], 6
0x14014985d movups   xmm0, xmmword ptr [rbp + 0xf0]
0x140149864 movups   xmmword ptr [rbp + 0x210], xmm0
0x14014986b mov      qword ptr [rbp + 0x220], 0x13
0x140149876 mov      rax, qword ptr [rbp + 0x368]
0x14014987d mov      qword ptr [rbp + 0x228], rax
0x140149884 mov      qword ptr [rbp + 0x230], 0x13
0x14014988f mov      dword ptr [rbp + 0x238], 0
0x140149899 mov      dword ptr [rbp + 0x240], 0
0x1401498a3 mov      qword ptr [rbp + 0x248], 0
0x1401498ae mov      byte ptr [rbp + 0x250], 0
0x1401498b5 mov      rax, qword ptr [rbp + 0x160]
0x1401498bc mov      qword ptr [rbp + 0x251], rax
0x1401498c3 mov      eax, dword ptr [rbp + 0x167]
0x1401498c9 mov      dword ptr [rbp + 0x258], eax
0x1401498cf add      rsp, 0x28
0x1401498d3 pop      rbx
0x1401498d4 pop      rdi
0x1401498d5 pop      rsi
0x1401498d6 pop      r12
0x1401498d8 pop      r13
0x1401498da pop      r14
0x1401498dc pop      r15
0x1401498de pop      rbp
0x1401498df ret      