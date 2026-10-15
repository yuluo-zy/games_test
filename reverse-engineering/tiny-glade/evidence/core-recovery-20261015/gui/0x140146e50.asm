0x140146e50 push     rbp
0x140146e51 push     r14
0x140146e53 push     rsi
0x140146e54 push     rdi
0x140146e55 push     rbx
0x140146e56 sub      rsp, 0x410
0x140146e5d lea      rbp, [rsp + 0x80]
0x140146e65 movaps   xmmword ptr [rbp + 0x380], xmm6
0x140146e6c mov      qword ptr [rbp + 0x378], 0xfffffffffffffffe
0x140146e77 mov      rsi, rdx
0x140146e7a mov      rbx, qword ptr [rcx]
0x140146e7d mov      r14, qword ptr [rcx + 8]
0x140146e81 movzx    eax, byte ptr [rip + 0x34edd81]
0x140146e88 mov      ecx, 8
0x140146e8d mov      edx, 8
0x140146e92 call     0x140613c10 ; __rust_alloc
0x140146e97 test     rax, rax
0x140146e9a je       0x14014738f
0x140146ea0 mov      qword ptr [rbp + 0x368], rax
0x140146ea7 mov      qword ptr [rax], rbx
0x140146eaa lea      rcx, [rbp + 0xf0]
0x140146eb1 call     0x140c34750 ; _$LT$egui..widget_text..WidgetText$u20$as$u20$core..default..Default$GT$::default::h1fb81403bbfc71e5
0x140146eb6 mov      qword ptr [rbp + 0x260], 0
0x140146ec1 mov      qword ptr [rbp + 0x268], 1
0x140146ecc xorps    xmm0, xmm0
0x140146ecf movups   xmmword ptr [rbp + 0x270], xmm0
0x140146ed6 mov      qword ptr [rbp + 0x280], 1
0x140146ee1 mov      qword ptr [rbp + 0x288], 0
0x140146eec movups   xmm1, xmmword ptr [rbp + 0xf0]
0x140146ef3 movups   xmm2, xmmword ptr [rbp + 0x100]
0x140146efa movups   xmm3, xmmword ptr [rbp + 0x110]
0x140146f01 movups   xmm4, xmmword ptr [rbp + 0x120]
0x140146f08 movups   xmmword ptr [rbp - 0x50], xmm1
0x140146f0c movups   xmmword ptr [rbp - 0x40], xmm2
0x140146f10 movups   xmmword ptr [rbp - 0x30], xmm3
0x140146f14 movups   xmmword ptr [rbp - 0x20], xmm4
0x140146f18 movups   xmm1, xmmword ptr [rbp + 0x130]
0x140146f1f movups   xmmword ptr [rbp - 0x10], xmm1
0x140146f23 movups   xmm1, xmmword ptr [rbp + 0x140]
0x140146f2a movups   xmmword ptr [rbp], xmm1
0x140146f2e movups   xmm1, xmmword ptr [rbp + 0x150]
0x140146f35 movups   xmmword ptr [rbp + 0x10], xmm1
0x140146f39 movups   xmmword ptr [rbp + 0x2d8], xmm0
0x140146f40 mov      qword ptr [rbp + 0x1c0], 0
0x140146f4b mov      qword ptr [rbp + 0x1d0], 0
0x140146f56 mov      qword ptr [rbp + 0x1e0], 0
0x140146f61 movups   xmm0, xmmword ptr [rbp - 0x58]
0x140146f65 movups   xmm1, xmmword ptr [rbp - 0x48]
0x140146f69 movups   xmm2, xmmword ptr [rbp - 0x38]
0x140146f6d movups   xmm3, xmmword ptr [rbp - 0x28]
0x140146f71 movups   xmmword ptr [rbp + 0x1e8], xmm0
0x140146f78 movups   xmmword ptr [rbp + 0x1f8], xmm1
0x140146f7f movups   xmmword ptr [rbp + 0x208], xmm2
0x140146f86 movups   xmmword ptr [rbp + 0x218], xmm3
0x140146f8d movups   xmm0, xmmword ptr [rbp - 0x18]
0x140146f91 movups   xmmword ptr [rbp + 0x228], xmm0
0x140146f98 movups   xmm0, xmmword ptr [rbp - 8]
0x140146f9c movups   xmmword ptr [rbp + 0x238], xmm0
0x140146fa3 movups   xmm0, xmmword ptr [rbp + 8]
0x140146fa7 movups   xmmword ptr [rbp + 0x248], xmm0
0x140146fae mov      rax, qword ptr [rbp + 0x18]
0x140146fb2 mov      qword ptr [rbp + 0x258], rax
0x140146fb9 mov      dword ptr [rbp + 0x290], 2
0x140146fc3 movaps   xmm0, xmmword ptr [rip + 0x27ed646]
0x140146fca movups   xmmword ptr [rbp + 0x298], xmm0
0x140146fd1 mov      byte ptr [rbp + 0x2a8], 0
0x140146fd8 movaps   xmm0, xmmword ptr [rip + 0x27ed5f1]
0x140146fdf movups   xmmword ptr [rbp + 0x2b0], xmm0
0x140146fe6 mov      byte ptr [rbp + 0x2c0], 0
0x140146fed mov      rax, qword ptr [rbp + 0x368]
0x140146ff4 mov      qword ptr [rbp + 0x2c8], rax
0x140146ffb lea      rax, [rip + 0x28bb096]
0x140147002 mov      qword ptr [rbp + 0x2d0], rax
0x140147009 mov      qword ptr [rbp + 0x2f0], 0
0x140147014 mov      dword ptr [rbp + 0x300], 0x10101
0x14014701e mov      byte ptr [rbp + 0x304], 2
0x140147025 movzx    eax, byte ptr [rip + 0x34edbdd]
0x14014702c mov      ecx, 3
0x140147031 mov      edx, 1
0x140147036 call     0x140613c10 ; __rust_alloc
0x14014703b test     rax, rax
0x14014703e je       0x14014739e
0x140147044 mov      byte ptr [rax + 2], 0x56
0x140147048 mov      word ptr [rax], 0x6f46
0x14014704d mov      dword ptr [rbp + 0x167], 0
0x140147057 mov      dword ptr [rbp + 0x164], 0
0x140147061 mov      rdi, qword ptr [rbp + 0x1f0]
0x140147068 mov      ecx, edi
0x14014706a and      ecx, 0xfffffffe
0x14014706d lea      rdx, [rdi - 3]
0x140147071 xor      r8d, r8d
0x140147074 cmp      ecx, 4
0x140147077 cmove    r8, rdx
0x14014707b test     r8, r8
0x14014707e mov      qword ptr [rbp + 0x368], rax
0x140147085 je       0x1401470c0
0x140147087 cmp      r8, 1
0x14014708b jne      0x14014712e
0x140147091 mov      rdx, qword ptr [rbp + 0x1f8]
0x140147098 test     rdx, rdx
0x14014709b je       0x1401470af
0x14014709d mov      rcx, qword ptr [rbp + 0x200]
0x1401470a4 mov      r8d, 1
0x1401470aa call     0x140613c20 ; __rust_dealloc
0x1401470af lea      rcx, [rbp + 0x210]
0x1401470b6 call     0x140608f80 ; core::ptr::drop_in_place$LT$alloc..vec..Vec$LT$epaint..text..text_layout_types..LayoutSection$GT$$GT$::hb247fd76ce4de2e9 (.llvm.12095432060108768980) / core::ptr::drop_in_place$LT$alloc..vec..Vec$LT$epaint..text..text_layout_types..LayoutSection$GT$$GT$::h284850123c6b7642 (.llvm.10346396820423203432)
0x1401470bb jmp      0x140147147 ; 
0x1401470c0 mov      rdx, qword ptr [rbp + 0x220]
0x1401470c7 test     rdx, rdx
0x1401470ca je       0x1401470de
0x1401470cc mov      rcx, qword ptr [rbp + 0x228]
0x1401470d3 mov      r8d, 1
0x1401470d9 call     0x140613c20 ; __rust_dealloc
0x1401470de cmp      edi, 3
0x1401470e1 je       0x140147101
0x1401470e3 cmp      edi, 2
0x1401470e6 jb       0x140147101
0x1401470e8 mov      rax, qword ptr [rbp + 0x1f8]
0x1401470ef lock dec qword ptr [rax]
0x1401470f3 jne      0x140147101
0x1401470f5 lea      rcx, [rbp + 0x1f8]
0x1401470fc call     0x14064eb30 ; alloc::sync::Arc$LT$T$C$A$GT$::drop_slow::h5672070ce44dd4cf / alloc::sync::Arc$LT$T$C$A$GT$::drop_slow::h8e637e57b8f96748
0x140147101 mov      rax, qword ptr [rbp + 0x208]
0x140147108 cmp      rax, 6
0x14014710c je       0x140147147
0x14014710e cmp      eax, 5
0x140147111 jb       0x140147147
0x140147113 mov      rax, qword ptr [rbp + 0x210]
0x14014711a lock dec qword ptr [rax]
0x14014711e jne      0x140147147
0x140147120 lea      rcx, [rbp + 0x210]
0x140147127 call     0x14064eb30 ; alloc::sync::Arc$LT$T$C$A$GT$::drop_slow::h5672070ce44dd4cf / alloc::sync::Arc$LT$T$C$A$GT$::drop_slow::h8e637e57b8f96748
0x14014712c jmp      0x140147147 ; 
0x14014712e mov      rax, qword ptr [rbp + 0x1f8]
0x140147135 lock dec qword ptr [rax]
0x140147139 jne      0x140147147
0x14014713b lea      rcx, [rbp + 0x1f8]
0x140147142 call     0x140c7de90 ; alloc::sync::Arc$LT$T$C$A$GT$::drop_slow::h88bf8f93b401f9b7
0x140147147 mov      qword ptr [rbp + 0x1f0], 3
0x140147152 movups   xmm0, xmmword ptr [rbp - 0x58]
0x140147156 movups   xmmword ptr [rbp + 0x1f8], xmm0
0x14014715d mov      qword ptr [rbp + 0x208], 6
0x140147168 movups   xmm0, xmmword ptr [rbp + 0xf0]
0x14014716f movups   xmmword ptr [rbp + 0x210], xmm0
0x140147176 mov      qword ptr [rbp + 0x220], 3
0x140147181 mov      rax, qword ptr [rbp + 0x368]
0x140147188 mov      qword ptr [rbp + 0x228], rax
0x14014718f mov      qword ptr [rbp + 0x230], 3
0x14014719a mov      dword ptr [rbp + 0x238], 0
0x1401471a4 mov      dword ptr [rbp + 0x240], 0
0x1401471ae mov      qword ptr [rbp + 0x248], 0
0x1401471b9 mov      byte ptr [rbp + 0x250], 0
0x1401471c0 mov      rax, qword ptr [rbp + 0x160]
0x1401471c7 mov      qword ptr [rbp + 0x251], rax
0x1401471ce mov      eax, dword ptr [rbp + 0x167]
0x1401471d4 mov      dword ptr [rbp + 0x258], eax
0x1401471da lea      rdi, [rbp - 0x58]
0x1401471de lea      rdx, [rbp + 0x1c0]
0x1401471e5 mov      r8d, 0x148
0x1401471eb mov      rcx, rdi
0x1401471ee call     0x142923c00 ; memcpy
0x1401471f3 lea      rcx, [rbp + 0x1c0]
0x1401471fa mov      rdx, rdi
0x1401471fd mov      r8, rsi
0x140147200 call     0x140c77160 ; _$LT$egui..widgets..slider..Slider$u20$as$u20$egui..widgets..Widget$GT$::ui::h1e78a398b6556716
0x140147205 mov      rax, qword ptr [rbp + 0x1d0]
0x14014720c lock dec qword ptr [rax]
0x140147210 jne      0x14014721e
0x140147212 lea      rcx, [rbp + 0x1d0]
0x140147219 call     0x140c41170 ; alloc::sync::Arc$LT$T$C$A$GT$::drop_slow::he9d75ed3b56676ff
0x14014721e mov      qword ptr [rbp - 0x58], 4
0x140147226 mov      byte ptr [rbp + 0x377], 0
0x14014722d lea      rdx, [rip + 0x2809f71] ; @0x1429511a5 'Reset'
0x140147234 lea      rcx, [rbp + 0x1c0]
0x14014723b mov      r8d, 5
0x140147241 call     0x1400568d0 ; _$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$::into::h83d03329215035d5 (.llvm.12463876434711410378)
0x140147246 movups   xmm0, xmmword ptr [rbp + 0x220]
0x14014724d movaps   xmmword ptr [rbp + 0x150], xmm0
0x140147254 movups   xmm0, xmmword ptr [rbp + 0x210]
0x14014725b movaps   xmmword ptr [rbp + 0x140], xmm0
0x140147262 movups   xmm0, xmmword ptr [rbp + 0x200]
0x140147269 movaps   xmmword ptr [rbp + 0x130], xmm0
0x140147270 movups   xmm0, xmmword ptr [rbp + 0x1c0]
0x140147277 movups   xmm1, xmmword ptr [rbp + 0x1d0]
0x14014727e movups   xmm2, xmmword ptr [rbp + 0x1e0]
0x140147285 movups   xmm3, xmmword ptr [rbp + 0x1f0]
0x14014728c movaps   xmmword ptr [rbp + 0x120], xmm3
0x140147293 movaps   xmmword ptr [rbp + 0x110], xmm2
0x14014729a movaps   xmmword ptr [rbp + 0x100], xmm1
0x1401472a1 movaps   xmmword ptr [rbp + 0xf0], xmm0
0x1401472a8 mov      byte ptr [rbp + 0x377], 1
0x1401472af lea      rcx, [rbp + 0x1c0]
0x1401472b6 lea      rdx, [rbp - 0x58]
0x1401472ba lea      r8, [rbp + 0xf0]
0x1401472c1 call     0x140c46bd0 ; egui::widgets::button::Button::opt_image_and_text::hdd7a18fc15f55700
0x1401472c6 lea      rcx, [rbp + 0x160]
0x1401472cd lea      rdx, [rbp + 0x1c0]
0x1401472d4 mov      r8, rsi
0x1401472d7 call     0x140c46d20 ; _$LT$egui..widgets..button..Button$u20$as$u20$egui..widgets..Widget$GT$::ui::ha728e61814580cc6
0x1401472dc cmp      byte ptr [rbp + 0x1b4], 0
0x1401472e3 je       0x14014730e
0x1401472e5 mov      rax, qword ptr [rbp + 0x170]
0x1401472ec lock dec qword ptr [rax]
0x1401472f0 jne      0x1401472fe
0x1401472f2 lea      rcx, [rbp + 0x170]
0x1401472f9 call     0x140c41170 ; alloc::sync::Arc$LT$T$C$A$GT$::drop_slow::he9d75ed3b56676ff
0x1401472fe mov      dword ptr [rbx], 0x42340000
0x140147304 movss    xmm6, dword ptr [rip + 0x27ed314]
0x14014730c jmp      0x140147352 ; 
0x14014730e mov      byte ptr [rbp + 0x1c0], 0
0x140147315 cmp      byte ptr [rbp + 0x1b3], 0
0x14014731c je       0x140147335
0x14014731e lea      rcx, [rbp + 0x170]
0x140147325 lea      rdx, [rbp + 0x1c0]
0x14014732c call     0x140042870 ; egui::context::Context::write::h9406d0def1a52fff / egui::context::Context::write::hd1da45529f868497
0x140147331 test     al, al
0x140147333 jne      0x1401472e5
0x140147335 mov      rax, qword ptr [rbp + 0x170]
0x14014733c lock dec qword ptr [rax]
0x140147340 jne      0x14014734e
0x140147342 lea      rcx, [rbp + 0x170]
0x140147349 call     0x140c41170 ; alloc::sync::Arc$LT$T$C$A$GT$::drop_slow::he9d75ed3b56676ff
0x14014734e movss    xmm6, dword ptr [rbx]
0x140147352 mov      rsi, qword ptr [r14]
0x140147355 mov      rcx, rsi
0x140147358 call     0x1409114d0 ; country_core::systems::camera_rig::MainCamera::target_vertical_fov_degrees::heb6c9d2412ef6093
0x14014735d ucomiss  xmm6, xmm0
0x140147360 jne      0x140147364
0x140147362 jnp      0x14014737a
0x140147364 mov      eax, dword ptr [r14 + 0x1c]
0x140147368 mov      rcx, qword ptr [r14 + 0x10]
0x14014736c mov      dword ptr [rcx], eax
0x14014736e movss    xmm1, dword ptr [rbx]
0x140147372 mov      rcx, rsi
0x140147375 call     0x1409114f0 ; country_core::systems::camera_rig::MainCamera::immediately_set_vertical_fov_degrees::h3332a5c9df90f461
0x14014737a movaps   xmm6, xmmword ptr [rbp + 0x380]
0x140147381 add      rsp, 0x410
0x140147388 pop      rbx
0x140147389 pop      rdi
0x14014738a pop      rsi
0x14014738b pop      r14
0x14014738d pop      rbp
0x14014738e ret      
0x14014738f mov      ecx, 8
0x140147394 mov      edx, 8
0x140147399 call     0x1428d9000 ; alloc::alloc::handle_alloc_error::h3e7daf9bcd04547a
0x14014739e lea      r8, [rip + 0x2809d83]
0x1401473a5 mov      ecx, 1
0x1401473aa mov      edx, 3
0x1401473af call     0x1428d8fe3 ; alloc::raw_vec::handle_error::hb99beac7784c4464
0x1401473b4 ud2      
0x1401473b6 nop      word ptr cs:[rax + rax]
0x1401473c0 mov      qword ptr [rsp + 0x10], rdx
0x1401473c5 push     rbp
0x1401473c6 push     r14
0x1401473c8 push     rsi
0x1401473c9 push     rdi
0x1401473ca push     rbx
0x1401473cb sub      rsp, 0x30
0x1401473cf lea      rbp, [rdx + 0x80]
0x1401473d6 movaps   xmmword ptr [rsp + 0x20], xmm6
0x1401473db lea      rcx, [rbp + 0x1c0]
0x1401473e2 call     0x1405ec430 ; core::ptr::drop_in_place$LT$egui..widgets..slider..Slider$GT$::hce4bcb5ac07e40a9
0x1401473e7 movaps   xmm6, xmmword ptr [rsp + 0x20]
0x1401473ec add      rsp, 0x30
0x1401473f0 pop      rbx
0x1401473f1 pop      rdi
0x1401473f2 pop      rsi
0x1401473f3 pop      r14
0x1401473f5 pop      rbp
0x1401473f6 ret      
0x1401473f7 nop      word ptr [rax + rax]
0x140147400 mov      qword ptr [rsp + 0x10], rdx
0x140147405 push     rbp
0x140147406 push     r14
0x140147408 push     rsi
0x140147409 push     rdi
0x14014740a push     rbx
0x14014740b sub      rsp, 0x30
0x14014740f lea      rbp, [rdx + 0x80]
0x140147416 movaps   xmmword ptr [rsp + 0x20], xmm6
0x14014741b cmp      byte ptr [rbp + 0x377], 0
0x140147422 jne      0x140147434
0x140147424 cmp      qword ptr [rbp - 0x58], 4
0x140147429 je       0x140147434
0x14014742b lea      rcx, [rbp - 0x58]
0x14014742f call     0x1405ea2f0 ; core::ptr::drop_in_place$LT$egui..widgets..image..Image$GT$::h14e0db96575526f3 / core::ptr::drop_in_place$LT$egui..widgets..image..Image$GT$::he79b48ffe16f7f22
0x140147434 movaps   xmm6, xmmword ptr [rsp + 0x20]
0x140147439 add      rsp, 0x30
0x14014743d pop      rbx
0x14014743e pop      rdi
0x14014743f pop      rsi
0x140147440 pop      r14
0x140147442 pop      rbp
0x140147443 ret      
0x140147444 nop      word ptr cs:[rax + rax]
0x140147450 mov      qword ptr [rsp + 0x10], rdx
0x140147455 push     rbp
0x140147456 push     r14
0x140147458 push     rsi
0x140147459 push     rdi
0x14014745a push     rbx
0x14014745b sub      rsp, 0x30
0x14014745f lea      rbp, [rdx + 0x80]
0x140147466 movaps   xmmword ptr [rsp + 0x20], xmm6
0x14014746b lea      rcx, [rbp + 0x170]
0x140147472 mov      rax, qword ptr [rbp + 0x170]
0x140147479 lock dec qword ptr [rax]
0x14014747d jne      0x140147484
0x14014747f call     0x140c41170 ; alloc::sync::Arc$LT$T$C$A$GT$::drop_slow::he9d75ed3b56676ff
0x140147484 movaps   xmm6, xmmword ptr [rsp + 0x20]
0x140147489 add      rsp, 0x30
0x14014748d pop      rbx
0x14014748e pop      rdi
0x14014748f pop      rsi
0x140147490 pop      r14
0x140147492 pop      rbp
0x140147493 ret      
0x140147494 nop      word ptr cs:[rax + rax]
0x1401474a0 mov      qword ptr [rsp + 0x10], rdx
0x1401474a5 push     rbp
0x1401474a6 push     r14
0x1401474a8 push     rsi
0x1401474a9 push     rdi
0x1401474aa push     rbx
0x1401474ab sub      rsp, 0x30
0x1401474af lea      rbp, [rdx + 0x80]
0x1401474b6 movaps   xmmword ptr [rsp + 0x20], xmm6
0x1401474bb lea      rdx, [rip + 0x28babd6]
0x1401474c2 mov      rcx, qword ptr [rbp + 0x368]
0x1401474c9 call     0x14002e900 ; core::ptr::drop_in_place$LT$alloc..boxed..Box$LT$dyn$u20$image..image..ImageDecoder$GT$$GT$::h9ba56180edc32c8c (.llvm.1100611113890887402) / core::ptr::drop_in_place$LT$alloc..boxed..Box$LT$dyn$u20$image..image..ImageDecoder$GT$$GT$::h4ca9b6c0ff719077 (.llvm.5988595327333108083)
0x1401474ce movaps   xmm6, xmmword ptr [rsp + 0x20]
0x1401474d3 add      rsp, 0x30
0x1401474d7 pop      rbx
0x1401474d8 pop      rdi
0x1401474d9 pop      rsi
0x1401474da pop      r14
0x1401474dc pop      rbp
0x1401474dd ret      
0x1401474de nop      
0x1401474e0 mov      qword ptr [rsp + 0x10], rdx
0x1401474e5 push     rbp
0x1401474e6 push     r14
0x1401474e8 push     rsi
0x1401474e9 push     rdi
0x1401474ea push     rbx
0x1401474eb sub      rsp, 0x30
0x1401474ef lea      rbp, [rdx + 0x80]
0x1401474f6 movaps   xmmword ptr [rsp + 0x20], xmm6
0x1401474fb lea      rcx, [rbp + 0x208]
0x140147502 call     0x1405fdb50 ; core::ptr::drop_in_place$LT$core..option..Option$LT$egui..style..TextStyle$GT$$GT$::h520bce03b9e9c449 (.llvm.7124148353756482180) / core::ptr::drop_in_place$LT$core..option..Option$LT$egui..style..TextStyle$GT$$GT$::h520bce03b9e9c449 (.llvm.15948950018123943476)
0x140147507 movaps   xmm6, xmmword ptr [rsp + 0x20]
0x14014750c add      rsp, 0x30
0x140147510 pop      rbx
0x140147511 pop      rdi
0x140147512 pop      rsi
0x140147513 pop      r14
0x140147515 pop      rbp
0x140147516 ret      
0x140147517 nop      word ptr [rax + rax]
0x140147520 mov      qword ptr [rsp + 0x10], rdx
0x140147525 push     rbp
0x140147526 push     r14
0x140147528 push     rsi
0x140147529 push     rdi
0x14014752a push     rbx
0x14014752b sub      rsp, 0x30
0x14014752f lea      rbp, [rdx + 0x80]
0x140147536 movaps   xmmword ptr [rsp + 0x20], xmm6
0x14014753b mov      qword ptr [rbp + 0x1f0], 3
0x140147546 movups   xmm0, xmmword ptr [rbp - 0x58]
0x14014754a movups   xmmword ptr [rbp + 0x1f8], xmm0
0x140147551 mov      qword ptr [rbp + 0x208], 6
0x14014755c movups   xmm0, xmmword ptr [rbp + 0xf0]
0x140147563 movups   xmmword ptr [rbp + 0x210], xmm0
0x14014756a mov      qword ptr [rbp + 0x220], 3
0x140147575 mov      rax, qword ptr [rbp + 0x368]
0x14014757c mov      qword ptr [rbp + 0x228], rax
0x140147583 mov      qword ptr [rbp + 0x230], 3
0x14014758e mov      dword ptr [rbp + 0x238], 0
0x140147598 mov      dword ptr [rbp + 0x240], 0
0x1401475a2 mov      qword ptr [rbp + 0x248], 0
0x1401475ad mov      byte ptr [rbp + 0x250], 0
0x1401475b4 mov      rax, qword ptr [rbp + 0x160]
0x1401475bb mov      qword ptr [rbp + 0x251], rax
0x1401475c2 mov      eax, dword ptr [rbp + 0x167]
0x1401475c8 mov      dword ptr [rbp + 0x258], eax
0x1401475ce movaps   xmm6, xmmword ptr [rsp + 0x20]
0x1401475d3 add      rsp, 0x30
0x1401475d7 pop      rbx
0x1401475d8 pop      rdi
0x1401475d9 pop      rsi
0x1401475da pop      r14
0x1401475dc pop      rbp
0x1401475dd ret      