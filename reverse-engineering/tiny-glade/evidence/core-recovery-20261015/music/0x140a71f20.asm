0x140a71f20 push     rbp
0x140a71f21 push     r15
0x140a71f23 push     r14
0x140a71f25 push     r12
0x140a71f27 push     rsi
0x140a71f28 push     rdi
0x140a71f29 push     rbx
0x140a71f2a sub      rsp, 0x400
0x140a71f31 lea      rbp, [rsp + 0x80]
0x140a71f39 mov      qword ptr [rbp + 0x378], 0xfffffffffffffffe
0x140a71f44 mov      rsi, rdx
0x140a71f47 mov      rdi, qword ptr [rcx]
0x140a71f4a lea      rcx, [rbp + 0xe0]
0x140a71f51 call     0x1408bf2c0 ; _$LT$country_core..startup..load_glade_audio..GladeAudio$u20$as$u20$core..default..Default$GT$::default::h756d245c8c6be322
0x140a71f56 mov      rbx, qword ptr [rbp + 0x100]
0x140a71f5d mov      r14, qword ptr [rbp + 0x108]
0x140a71f64 mov      rdx, qword ptr [rdi]
0x140a71f67 test     rdx, rdx
0x140a71f6a je       0x140a71fc5
0x140a71f6c add      rdx, 0x10
0x140a71f70 lea      rcx, [rbp - 0x50]
0x140a71f74 mov      r8, rbx
0x140a71f77 mov      r9, r14
0x140a71f7a call     0x140a54440 ; country_core::resources::audio_resources::AudioCoreInner::instantiate_audio::h8d884187476fbacc
0x140a71f7f cmp      qword ptr [rbp - 0x50], 2
0x140a71f84 jne      0x140a72024
0x140a71f8a mov      rax, qword ptr [rbp - 0x48]
0x140a71f8e mov      qword ptr [rbp + 0x340], rax
0x140a71f95 lea      rax, [rip + 0x2064364]
0x140a71f9c mov      qword ptr [rsp + 0x20], rax
0x140a71fa1 lea      rcx, [rip + 0x2063f40] ; @0x142ad5ee8 'called `Result::unwrap()` on an `Err` value'
0x140a71fa8 lea      r9, [rip + 0x2063f19]
0x140a71faf lea      r8, [rbp + 0x340]
0x140a71fb6 mov      edx, 0x2b
0x140a71fbb call     0x1428d9760 ; core::result::unwrap_failed::h96d74ae09566f4b3
0x140a71fc0 jmp      0x140a725bd ; 
0x140a71fc5 test     r14, r14
0x140a71fc8 js       0x140a72578
0x140a71fce je       0x140a71ff8
0x140a71fd0 movzx    eax, byte ptr [rip + 0x2bc2c32]
0x140a71fd7 mov      r12d, 1
0x140a71fdd mov      edx, 1
0x140a71fe2 mov      rcx, r14
0x140a71fe5 call     0x140613c10 ; __rust_alloc
0x140a71fea test     rax, rax
0x140a71fed je       0x140a7257b
0x140a71ff3 mov      r15, rax
0x140a71ff6 jmp      0x140a71ffe ; 
0x140a71ff8 mov      r15d, 1
0x140a71ffe mov      rcx, r15
0x140a72001 mov      rdx, rbx
0x140a72004 mov      r8, r14
0x140a72007 call     0x142923c00 ; memcpy
0x140a7200c mov      qword ptr [rbp - 0x50], 0
0x140a72014 mov      qword ptr [rbp - 0x40], r14
0x140a72018 mov      qword ptr [rbp - 0x38], r15
0x140a7201c mov      qword ptr [rbp - 0x30], r14
0x140a72020 mov      byte ptr [rbp - 0x28], 2
0x140a72024 mov      rax, qword ptr [rbp - 0x50]
0x140a72028 mov      rcx, qword ptr [rbp - 0x48]
0x140a7202c mov      rdx, qword ptr [rbp - 0x40]
0x140a72030 mov      r8, qword ptr [rbp - 0x38]
0x140a72034 mov      r9, qword ptr [rbp - 0x30]
0x140a72038 mov      qword ptr [rbp + 0x330], r9
0x140a7203f movzx    r9d, byte ptr [rbp - 0x28]
0x140a72044 mov      byte ptr [rbp + 0x338], r9b
0x140a7204b mov      r9d, dword ptr [rbp - 0x27]
0x140a7204f mov      dword ptr [rbp + 0x339], r9d
0x140a72056 movzx    r9d, word ptr [rbp - 0x23]
0x140a7205b mov      word ptr [rbp + 0x33d], r9w
0x140a72063 movzx    r9d, byte ptr [rbp - 0x21]
0x140a72068 mov      byte ptr [rbp + 0x33f], r9b
0x140a7206f mov      qword ptr [rbp + 0x320], rdx
0x140a72076 mov      qword ptr [rbp + 0x328], r8
0x140a7207d mov      qword ptr [rbp + 0x310], rax
0x140a72084 mov      qword ptr [rbp + 0x318], rcx
0x140a7208b mov      r14, qword ptr [rbp + 0xe8]
0x140a72092 mov      r15, qword ptr [rbp + 0xf0]
0x140a72099 test     r15, r15
0x140a7209c je       0x140a720d4
0x140a7209e lea      r12, [r14 + 8]
0x140a720a2 jmp      0x140a720b9 ; 
0x140a720a4 nop      word ptr cs:[rax + rax]
0x140a720b0 add      r12, 0x18
0x140a720b4 dec      r15
0x140a720b7 je       0x140a720d4
0x140a720b9 mov      rdx, qword ptr [r12 - 8]
0x140a720be test     rdx, rdx
0x140a720c1 je       0x140a720b0
0x140a720c3 mov      rcx, qword ptr [r12]
0x140a720c7 mov      r8d, 1
0x140a720cd call     0x140613c20 ; __rust_dealloc
0x140a720d2 jmp      0x140a720b0 ; 
0x140a720d4 mov      rax, qword ptr [rbp + 0xe0]
0x140a720db test     rax, rax
0x140a720de je       0x140a720f6
0x140a720e0 shl      rax, 3
0x140a720e4 lea      rdx, [rax + rax*2]
0x140a720e8 mov      r8d, 8
0x140a720ee mov      rcx, r14
0x140a720f1 call     0x140613c20 ; __rust_dealloc
0x140a720f6 mov      rdx, qword ptr [rbp + 0xf8]
0x140a720fd test     rdx, rdx
0x140a72100 je       0x140a72110
0x140a72102 mov      r8d, 1
0x140a72108 mov      rcx, rbx
0x140a7210b call     0x140613c20 ; __rust_dealloc
0x140a72110 mov      rdx, qword ptr [rbp + 0x110]
0x140a72117 mov      rax, rdx
0x140a7211a neg      rax
0x140a7211d jo       0x140a72133
0x140a7211f jae      0x140a72133
0x140a72121 mov      rcx, qword ptr [rbp + 0x118]
0x140a72128 mov      r8d, 1
0x140a7212e call     0x140613c20 ; __rust_dealloc
0x140a72133 cmp      dword ptr [rbp + 0x310], 1
0x140a7213a jne      0x140a72167
0x140a7213c lea      rdx, [rbp + 0x318]
0x140a72143 mov      byte ptr [rbp + 0x377], 1
0x140a7214a lea      rcx, [rbp - 0x50]
0x140a7214e call     0x1414372d0 ; libfmod::EventInstance::start::hb8dc190a69906390
0x140a72153 movabs   rax, 0x8000000000000004
0x140a7215d cmp      qword ptr [rbp - 0x50], rax
0x140a72161 jne      0x140a7251f
0x140a72167 movaps   xmm0, xmmword ptr [rbp + 0x310]
0x140a7216e movaps   xmm1, xmmword ptr [rbp + 0x320]
0x140a72175 movaps   xmm2, xmmword ptr [rbp + 0x330]
0x140a7217c movaps   xmmword ptr [rbp + 0x2d0], xmm2
0x140a72183 movaps   xmmword ptr [rbp + 0x2c0], xmm1
0x140a7218a movaps   xmmword ptr [rbp + 0x2b0], xmm0
0x140a72191 mov      rdx, qword ptr [rdi]
0x140a72194 test     rdx, rdx
0x140a72197 je       0x140a721fc
0x140a72199 add      rdx, 0x10
0x140a7219d lea      r8, [rip + 0x2064174] ; @0x142ad6318 'event:/Music/mus_GladePreview'
0x140a721a4 lea      rcx, [rbp + 0xe0]
0x140a721ab mov      r9d, 0x1d
0x140a721b1 call     0x140a54440 ; country_core::resources::audio_resources::AudioCoreInner::instantiate_audio::h8d884187476fbacc
0x140a721b6 cmp      qword ptr [rbp + 0xe0], 2
0x140a721be jne      0x140a7225f
0x140a721c4 mov      rax, qword ptr [rbp + 0xe8]
0x140a721cb mov      qword ptr [rbp - 0x50], rax
0x140a721cf lea      rax, [rip + 0x2064162]
0x140a721d6 mov      qword ptr [rsp + 0x20], rax
0x140a721db lea      rcx, [rip + 0x2063d06] ; @0x142ad5ee8 'called `Result::unwrap()` on an `Err` value'
0x140a721e2 lea      r9, [rip + 0x2063cdf]
0x140a721e9 lea      r8, [rbp - 0x50]
0x140a721ed mov      edx, 0x2b
0x140a721f2 call     0x1428d9760 ; core::result::unwrap_failed::h96d74ae09566f4b3
0x140a721f7 jmp      0x140a725bd ; 
0x140a721fc movzx    eax, byte ptr [rip + 0x2bc2a06]
0x140a72203 mov      ecx, 0x1d
0x140a72208 mov      edx, 1
0x140a7220d call     0x140613c10 ; __rust_alloc
0x140a72212 test     rax, rax
0x140a72215 je       0x140a7258f
0x140a7221b movups   xmm0, xmmword ptr [rip + 0x2064103] ; @0x142ad6325 'mus_GladePreview'
0x140a72222 movups   xmmword ptr [rax + 0xd], xmm0
0x140a72226 movups   xmm0, xmmword ptr [rip + 0x20640eb] ; @0x142ad6318 'event:/Music/mus_GladePreview'
0x140a7222d movups   xmmword ptr [rax], xmm0
0x140a72230 mov      qword ptr [rbp + 0xe0], 0
0x140a7223b mov      qword ptr [rbp + 0xf0], 0x1d
0x140a72246 mov      qword ptr [rbp + 0xf8], rax
0x140a7224d mov      qword ptr [rbp + 0x100], 0x1d
0x140a72258 mov      byte ptr [rbp + 0x108], 2
0x140a7225f mov      rax, qword ptr [rbp + 0xe0]
0x140a72266 mov      rcx, qword ptr [rbp + 0xe8]
0x140a7226d mov      rdx, qword ptr [rbp + 0xf0]
0x140a72274 mov      r8, qword ptr [rbp + 0xf8]
0x140a7227b mov      r9, qword ptr [rbp + 0x100]
0x140a72282 mov      qword ptr [rbp + 0x300], r9
0x140a72289 movzx    r9d, byte ptr [rbp + 0x108]
0x140a72291 mov      byte ptr [rbp + 0x308], r9b
0x140a72298 mov      r9d, dword ptr [rbp + 0x109]
0x140a7229f mov      dword ptr [rbp + 0x309], r9d
0x140a722a6 movzx    r9d, word ptr [rbp + 0x10d]
0x140a722ae mov      word ptr [rbp + 0x30d], r9w
0x140a722b6 movzx    r9d, byte ptr [rbp + 0x10f]
0x140a722be mov      byte ptr [rbp + 0x30f], r9b
0x140a722c5 mov      qword ptr [rbp + 0x2f0], rdx
0x140a722cc mov      qword ptr [rbp + 0x2f8], r8
0x140a722d3 mov      qword ptr [rbp + 0x2e0], rax
0x140a722da mov      qword ptr [rbp + 0x2e8], rcx
0x140a722e1 mov      rdx, qword ptr [rdi]
0x140a722e4 test     rdx, rdx
0x140a722e7 je       0x140a7234c
0x140a722e9 add      rdx, 0x10
0x140a722ed lea      r8, [rip + 0x206405c] ; @0x142ad6350 'event:/Amb/GladePreview'
0x140a722f4 lea      rcx, [rbp + 0xe0]
0x140a722fb mov      r9d, 0x17
0x140a72301 call     0x140a54440 ; country_core::resources::audio_resources::AudioCoreInner::instantiate_audio::h8d884187476fbacc
0x140a72306 cmp      qword ptr [rbp + 0xe0], 2
0x140a7230e jne      0x140a723b2
0x140a72314 mov      rax, qword ptr [rbp + 0xe8]
0x140a7231b mov      qword ptr [rbp - 0x50], rax
0x140a7231f lea      rax, [rip + 0x2064042]
0x140a72326 mov      qword ptr [rsp + 0x20], rax
0x140a7232b lea      rcx, [rip + 0x2063bb6] ; @0x142ad5ee8 'called `Result::unwrap()` on an `Err` value'
0x140a72332 lea      r9, [rip + 0x2063b8f]
0x140a72339 lea      r8, [rbp - 0x50]
0x140a7233d mov      edx, 0x2b
0x140a72342 call     0x1428d9760 ; core::result::unwrap_failed::h96d74ae09566f4b3
0x140a72347 jmp      0x140a725bd ; 
0x140a7234c movzx    eax, byte ptr [rip + 0x2bc28b6]
0x140a72353 mov      ecx, 0x17
0x140a72358 mov      edx, 1
0x140a7235d call     0x140613c10 ; __rust_alloc
0x140a72362 test     rax, rax
0x140a72365 je       0x140a725a7
0x140a7236b movups   xmm0, xmmword ptr [rip + 0x2063fde] ; @0x142ad6350 'event:/Amb/GladePreview'
0x140a72372 movups   xmmword ptr [rax], xmm0
0x140a72375 movabs   rcx, 0x7765697665725065
0x140a7237f mov      qword ptr [rax + 0xf], rcx
0x140a72383 mov      qword ptr [rbp + 0xe0], 0
0x140a7238e mov      qword ptr [rbp + 0xf0], 0x17
0x140a72399 mov      qword ptr [rbp + 0xf8], rax
0x140a723a0 mov      qword ptr [rbp + 0x100], 0x17
0x140a723ab mov      byte ptr [rbp + 0x108], 2
0x140a723b2 mov      rax, qword ptr [rbp + 0xe0]
0x140a723b9 mov      rcx, qword ptr [rbp + 0xe8]
0x140a723c0 mov      rdx, qword ptr [rbp + 0xf0]
0x140a723c7 mov      r8, qword ptr [rbp + 0xf8]
0x140a723ce mov      r9, qword ptr [rbp + 0x100]
0x140a723d5 mov      qword ptr [rbp + 0x360], r9
0x140a723dc movzx    r9d, byte ptr [rbp + 0x108]
0x140a723e4 mov      byte ptr [rbp + 0x368], r9b
0x140a723eb mov      r9d, dword ptr [rbp + 0x109]
0x140a723f2 mov      dword ptr [rbp + 0x369], r9d
0x140a723f9 movzx    r9d, word ptr [rbp + 0x10d]
0x140a72401 mov      word ptr [rbp + 0x36d], r9w
0x140a72409 movzx    r9d, byte ptr [rbp + 0x10f]
0x140a72411 mov      byte ptr [rbp + 0x36f], r9b
0x140a72418 mov      qword ptr [rbp + 0x350], rdx
0x140a7241f mov      qword ptr [rbp + 0x358], r8
0x140a72426 mov      qword ptr [rbp + 0x340], rax
0x140a7242d mov      qword ptr [rbp + 0x348], rcx
0x140a72434 lea      rcx, [rbp - 0x50]
0x140a72438 mov      rdx, rdi
0x140a7243b call     0x1408b63c0 ; country_core::systems::audio::music::music_player::MusicPlayer::init::hf6903dea92c88a31
0x140a72440 lea      rdi, [rbp + 0xe0]
0x140a72447 lea      rdx, [rbp - 0x50]
0x140a7244b mov      r8d, 0x130
0x140a72451 mov      rcx, rdi
0x140a72454 call     0x142923c00 ; memcpy
0x140a72459 movaps   xmm0, xmmword ptr [rbp + 0x2b0]
0x140a72460 movaps   xmm1, xmmword ptr [rbp + 0x2c0]
0x140a72467 movaps   xmm2, xmmword ptr [rbp + 0x2d0]
0x140a7246e movups   xmmword ptr [rbp + 0x210], xmm0
0x140a72475 movups   xmmword ptr [rbp + 0x220], xmm1
0x140a7247c movups   xmmword ptr [rbp + 0x230], xmm2
0x140a72483 mov      byte ptr [rbp + 0x2ac], 5
0x140a7248a movaps   xmm0, xmmword ptr [rbp + 0x2e0]
0x140a72491 movaps   xmm1, xmmword ptr [rbp + 0x2f0]
0x140a72498 movaps   xmm2, xmmword ptr [rbp + 0x300]
0x140a7249f movups   xmmword ptr [rbp + 0x240], xmm0
0x140a724a6 movups   xmmword ptr [rbp + 0x250], xmm1
0x140a724ad movups   xmmword ptr [rbp + 0x260], xmm2
0x140a724b4 movaps   xmm0, xmmword ptr [rbp + 0x340]
0x140a724bb movaps   xmm1, xmmword ptr [rbp + 0x350]
0x140a724c2 movaps   xmm2, xmmword ptr [rbp + 0x360]
0x140a724c9 movups   xmmword ptr [rbp + 0x270], xmm0
0x140a724d0 movups   xmmword ptr [rbp + 0x280], xmm1
0x140a724d7 movups   xmmword ptr [rbp + 0x290], xmm2
0x140a724de mov      qword ptr [rbp + 0x2a0], 0
0x140a724e9 mov      dword ptr [rbp + 0x2a8], 0
0x140a724f3 mov      byte ptr [rbp + 0x377], 0
0x140a724fa lea      r8, [rip + 0x2063e7f]
0x140a72501 mov      rcx, rsi
0x140a72504 mov      rdx, rdi
0x140a72507 call     0x1412e00c0 ; bevy_ecs::system::commands::Commands::insert_resource::h7d82c99877f2c640
0x140a7250c nop      
0x140a7250d add      rsp, 0x400
0x140a72514 pop      rbx
0x140a72515 pop      rdi
0x140a72516 pop      rsi
0x140a72517 pop      r12
0x140a72519 pop      r14
0x140a7251b pop      r15
0x140a7251d pop      rbp
0x140a7251e ret      
0x140a7251f mov      rax, qword ptr [rbp - 0x20]
0x140a72523 mov      qword ptr [rbp + 0x110], rax
0x140a7252a movups   xmm0, xmmword ptr [rbp - 0x50]
0x140a7252e movups   xmm1, xmmword ptr [rbp - 0x40]
0x140a72532 movups   xmm2, xmmword ptr [rbp - 0x30]
0x140a72536 movaps   xmmword ptr [rbp + 0x100], xmm2
0x140a7253d movaps   xmmword ptr [rbp + 0xf0], xmm1
0x140a72544 movaps   xmmword ptr [rbp + 0xe0], xmm0
0x140a7254b lea      rax, [rip + 0x206066e]
0x140a72552 mov      qword ptr [rsp + 0x20], rax
0x140a72557 lea      rcx, [rip + 0x205e0a2] ; @0x142ad0600 'called `Result::unwrap()` on an `Err` value'
0x140a7255e lea      r9, [rip + 0x205e05b]
0x140a72565 lea      r8, [rbp + 0xe0]
0x140a7256c mov      edx, 0x2b
0x140a72571 call     0x1428d9760 ; core::result::unwrap_failed::h96d74ae09566f4b3
0x140a72576 jmp      0x140a725bd ; 
0x140a72578 xor      r12d, r12d
0x140a7257b lea      r8, [rip + 0x205e22e]
0x140a72582 mov      rcx, r12
0x140a72585 mov      rdx, r14
0x140a72588 call     0x1428d8fe3 ; alloc::raw_vec::handle_error::hb99beac7784c4464
0x140a7258d jmp      0x140a725bd ; 
0x140a7258f lea      r8, [rip + 0x205e21a]
0x140a72596 mov      ecx, 1
0x140a7259b mov      edx, 0x1d
0x140a725a0 call     0x1428d8fe3 ; alloc::raw_vec::handle_error::hb99beac7784c4464
0x140a725a5 jmp      0x140a725bd ; 
0x140a725a7 lea      r8, [rip + 0x205e202]
0x140a725ae mov      ecx, 1
0x140a725b3 mov      edx, 0x17
0x140a725b8 call     0x1428d8fe3 ; alloc::raw_vec::handle_error::hb99beac7784c4464
0x140a725bd ud2      
0x140a725bf nop      
0x140a725c0 mov      qword ptr [rsp + 0x10], rdx
0x140a725c5 push     rbp
0x140a725c6 push     r15
0x140a725c8 push     r14
0x140a725ca push     r12
0x140a725cc push     rsi
0x140a725cd push     rdi
0x140a725ce push     rbx
0x140a725cf sub      rsp, 0x30
0x140a725d3 lea      rbp, [rdx + 0x80]
0x140a725da lea      rcx, [rbp + 0xe0]
0x140a725e1 call     0x14088e240 ; core::ptr::drop_in_place$LT$country_core..startup..load_glade_audio..GladeAudio$GT$::hfdf3aca559188713
0x140a725e6 nop      
0x140a725e7 add      rsp, 0x30
0x140a725eb pop      rbx
0x140a725ec pop      rdi
0x140a725ed pop      rsi
0x140a725ee pop      r12
0x140a725f0 pop      r14
0x140a725f2 pop      r15
0x140a725f4 pop      rbp
0x140a725f5 ret      
0x140a725f6 nop      word ptr cs:[rax + rax]
0x140a72600 mov      qword ptr [rsp + 0x10], rdx
0x140a72605 push     rbp
0x140a72606 push     r15
0x140a72608 push     r14
0x140a7260a push     r12
0x140a7260c push     rsi
0x140a7260d push     rdi
0x140a7260e push     rbx
0x140a7260f sub      rsp, 0x30
0x140a72613 lea      rbp, [rdx + 0x80]
0x140a7261a lea      rcx, [rbp + 0x340]
0x140a72621 call     0x140d4cc30 ; anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop::h2a52e76aa8e9c217
0x140a72626 nop      
0x140a72627 add      rsp, 0x30
0x140a7262b pop      rbx
0x140a7262c pop      rdi
0x140a7262d pop      rsi
0x140a7262e pop      r12
0x140a72630 pop      r14
0x140a72632 pop      r15
0x140a72634 pop      rbp
0x140a72635 ret      
0x140a72636 nop      word ptr cs:[rax + rax]
0x140a72640 mov      qword ptr [rsp + 0x10], rdx
0x140a72645 push     rbp
0x140a72646 push     r15
0x140a72648 push     r14
0x140a7264a push     r12
0x140a7264c push     rsi
0x140a7264d push     rdi
0x140a7264e push     rbx
0x140a7264f sub      rsp, 0x30
0x140a72653 lea      rbp, [rdx + 0x80]
0x140a7265a mov      byte ptr [rbp + 0x377], 1
0x140a72661 lea      rcx, [rbp + 0xe0]
0x140a72668 call     0x140a526b0 ; core::ptr::drop_in_place$LT$libfmod..Error$GT$::h60b34b2c39a0f0f1 (.llvm.4456468861159116502)
0x140a7266d mov      byte ptr [rbp + 0x377], 1
0x140a72674 add      rsp, 0x30
0x140a72678 pop      rbx
0x140a72679 pop      rdi
0x140a7267a pop      rsi
0x140a7267b pop      r12
0x140a7267d pop      r14
0x140a7267f pop      r15
0x140a72681 pop      rbp
0x140a72682 ret      
0x140a72683 nop      word ptr cs:[rax + rax]
0x140a72690 mov      qword ptr [rsp + 0x10], rdx
0x140a72695 push     rbp
0x140a72696 push     r15
0x140a72698 push     r14
0x140a7269a push     r12
0x140a7269c push     rsi
0x140a7269d push     rdi
0x140a7269e push     rbx
0x140a7269f sub      rsp, 0x30
0x140a726a3 lea      rbp, [rdx + 0x80]
0x140a726aa lea      rcx, [rbp + 0x2b0]
0x140a726b1 call     0x14014a680 ; core::ptr::drop_in_place$LT$country_core..resources..critters..sheep..SheepAudio$GT$::hfb03c8fee6406828 / bevy_ecs::component::ComponentDescriptor::drop_ptr::h9feafd2efa3d66f6 (.llvm.8002602149119224474)
0x140a726b6 mov      byte ptr [rbp + 0x377], 0
0x140a726bd add      rsp, 0x30
0x140a726c1 pop      rbx
0x140a726c2 pop      rdi
0x140a726c3 pop      rsi
0x140a726c4 pop      r12
0x140a726c6 pop      r14
0x140a726c8 pop      r15
0x140a726ca pop      rbp
0x140a726cb ret      
0x140a726cc nop      dword ptr [rax]
0x140a726d0 mov      qword ptr [rsp + 0x10], rdx
0x140a726d5 push     rbp
0x140a726d6 push     r15
0x140a726d8 push     r14
0x140a726da push     r12
0x140a726dc push     rsi
0x140a726dd push     rdi
0x140a726de push     rbx
0x140a726df sub      rsp, 0x30
0x140a726e3 lea      rbp, [rdx + 0x80]
0x140a726ea lea      rcx, [rbp - 0x50]
0x140a726ee call     0x140d4cc30 ; anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop::h2a52e76aa8e9c217
0x140a726f3 nop      
0x140a726f4 add      rsp, 0x30
0x140a726f8 pop      rbx
0x140a726f9 pop      rdi
0x140a726fa pop      rsi
0x140a726fb pop      r12
0x140a726fd pop      r14
0x140a726ff pop      r15
0x140a72701 pop      rbp
0x140a72702 ret      
0x140a72703 nop      word ptr cs:[rax + rax]
0x140a72710 mov      qword ptr [rsp + 0x10], rdx
0x140a72715 push     rbp
0x140a72716 push     r15
0x140a72718 push     r14
0x140a7271a push     r12
0x140a7271c push     rsi
0x140a7271d push     rdi
0x140a7271e push     rbx
0x140a7271f sub      rsp, 0x30
0x140a72723 lea      rbp, [rdx + 0x80]
0x140a7272a lea      rcx, [rbp + 0x2e0]
0x140a72731 call     0x14014a680 ; core::ptr::drop_in_place$LT$country_core..resources..critters..sheep..SheepAudio$GT$::hfb03c8fee6406828 / bevy_ecs::component::ComponentDescriptor::drop_ptr::h9feafd2efa3d66f6 (.llvm.8002602149119224474)
0x140a72736 nop      
0x140a72737 add      rsp, 0x30
0x140a7273b pop      rbx
0x140a7273c pop      rdi
0x140a7273d pop      rsi
0x140a7273e pop      r12
0x140a72740 pop      r14
0x140a72742 pop      r15
0x140a72744 pop      rbp
0x140a72745 ret      
0x140a72746 nop      word ptr cs:[rax + rax]
0x140a72750 mov      qword ptr [rsp + 0x10], rdx
0x140a72755 push     rbp
0x140a72756 push     r15
0x140a72758 push     r14
0x140a7275a push     r12
0x140a7275c push     rsi
0x140a7275d push     rdi
0x140a7275e push     rbx
0x140a7275f sub      rsp, 0x30
0x140a72763 lea      rbp, [rdx + 0x80]
0x140a7276a lea      rcx, [rbp - 0x50]
0x140a7276e call     0x140d4cc30 ; anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop::h2a52e76aa8e9c217
0x140a72773 nop      
0x140a72774 add      rsp, 0x30
0x140a72778 pop      rbx
0x140a72779 pop      rdi
0x140a7277a pop      rsi
0x140a7277b pop      r12
0x140a7277d pop      r14
0x140a7277f pop      r15
0x140a72781 pop      rbp
0x140a72782 ret      
0x140a72783 nop      word ptr cs:[rax + rax]
0x140a72790 mov      qword ptr [rsp + 0x10], rdx
0x140a72795 push     rbp
0x140a72796 push     r15
0x140a72798 push     r14
0x140a7279a push     r12
0x140a7279c push     rsi
0x140a7279d push     rdi
0x140a7279e push     rbx
0x140a7279f sub      rsp, 0x30
0x140a727a3 lea      rbp, [rdx + 0x80]
0x140a727aa lea      rcx, [rbp + 0x340]
0x140a727b1 call     0x14014a680 ; core::ptr::drop_in_place$LT$country_core..resources..critters..sheep..SheepAudio$GT$::hfb03c8fee6406828 / bevy_ecs::component::ComponentDescriptor::drop_ptr::h9feafd2efa3d66f6 (.llvm.8002602149119224474)
0x140a727b6 nop      
0x140a727b7 add      rsp, 0x30
0x140a727bb pop      rbx
0x140a727bc pop      rdi
0x140a727bd pop      rsi
0x140a727be pop      r12
0x140a727c0 pop      r14
0x140a727c2 pop      r15
0x140a727c4 pop      rbp
0x140a727c5 ret      
0x140a727c6 nop      word ptr cs:[rax + rax]
0x140a727d0 mov      qword ptr [rsp + 0x10], rdx
0x140a727d5 push     rbp
0x140a727d6 push     r15
0x140a727d8 push     r14
0x140a727da push     r12
0x140a727dc push     rsi
0x140a727dd push     rdi
0x140a727de push     rbx
0x140a727df sub      rsp, 0x30
0x140a727e3 lea      rbp, [rdx + 0x80]
0x140a727ea cmp      byte ptr [rbp + 0x377], 0
0x140a727f1 jne      0x140a72802
0x140a727f3 add      rsp, 0x30
0x140a727f7 pop      rbx
0x140a727f8 pop      rdi
0x140a727f9 pop      rsi
0x140a727fa pop      r12
0x140a727fc pop      r14
0x140a727fe pop      r15
0x140a72800 pop      rbp
0x140a72801 ret      
0x140a72802 lea      rcx, [rbp + 0x310]
0x140a72809 call     0x14014a680 ; core::ptr::drop_in_place$LT$country_core..resources..critters..sheep..SheepAudio$GT$::hfb03c8fee6406828 / bevy_ecs::component::ComponentDescriptor::drop_ptr::h9feafd2efa3d66f6 (.llvm.8002602149119224474)
0x140a7280e jmp      0x140a727f3 ; 