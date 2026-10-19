1412ab270: 4157 push r15
1412ab272: 4156 push r14
1412ab274: 4155 push r13
1412ab276: 4154 push r12
1412ab278: 56 push rsi
1412ab279: 57 push rdi
1412ab27a: 55 push rbp
1412ab27b: 53 push rbx
1412ab27c: 4881ec08010000 sub rsp, 0x108
1412ab283: 440f298c24f0000000 movaps xmmword ptr [rsp + 0xf0], xmm9
1412ab28c: 440f298424e0000000 movaps xmmword ptr [rsp + 0xe0], xmm8
1412ab295: 0f29bc24d0000000 movaps xmmword ptr [rsp + 0xd0], xmm7
1412ab29d: 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
1412ab2a5: 4d89ce mov r14, r9
1412ab2a8: 89d6 mov esi, edx
1412ab2aa: 4889cf mov rdi, rcx
1412ab2ad: 440f2881c0000000 movaps xmm8, xmmword ptr [rcx + 0xc0]
1412ab2b5: f20f10b9d0000000 movsd xmm7, qword ptr [rcx + 0xd0]
1412ab2bd: f30f10b1d8000000 movss xmm6, dword ptr [rcx + 0xd8]
1412ab2c5: 8b81e4000000 mov eax, dword ptr [rcx + 0xe4]
1412ab2cb: 89442438 mov dword ptr [rsp + 0x38], eax
1412ab2cf: 488b81dc000000 mov rax, qword ptr [rcx + 0xdc]
1412ab2d6: 4889442430 mov qword ptr [rsp + 0x30], rax
1412ab2db: f30f108940010000 movss xmm1, dword ptr [rcx + 0x140]
1412ab2e3: f20f109144010000 movsd xmm2, qword ptr [rcx + 0x144]
1412ab2eb: 0f28c1 movaps xmm0, xmm1
1412ab2ee: f30f59c1 mulss xmm0, xmm1
1412ab2f2: 0f28da movaps xmm3, xmm2
1412ab2f5: 0f59da mulps xmm3, xmm2
1412ab2f8: f30f58c3 addss xmm0, xmm3
1412ab2fc: f30f16db movshdup xmm3, xmm3
1412ab300: f30f58d8 addss xmm3, xmm0
1412ab304: 0f57c0 xorps xmm0, xmm0
1412ab307: f30f51c3 sqrtss xmm0, xmm3
1412ab30b: 0f57db xorps xmm3, xmm3
1412ab30e: 0f2ec3 ucomiss xmm0, xmm3
1412ab311: 0f8693000000 jbe 0x1412ab3aa
1412ab317: 0f16ca movlhps xmm1, xmm2
1412ab31a: 0f28c1 movaps xmm0, xmm1
1412ab31d: 0fc6c2d8 shufps xmm0, xmm2, 0xd8
1412ab321: 0fc6ca58 shufps xmm1, xmm2, 0x58
1412ab325: 410f28d0 movaps xmm2, xmm8
1412ab329: 410f59d0 mulps xmm2, xmm8
1412ab32d: f30f16da movshdup xmm3, xmm2
1412ab331: f30f58da addss xmm3, xmm2
1412ab335: 0f28e2 movaps xmm4, xmm2
1412ab338: 660f15e2 unpckhpd xmm4, xmm2
1412ab33c: f30f58e3 addss xmm4, xmm3
1412ab340: 0fc6e400 shufps xmm4, xmm4, 0
1412ab344: 0fc6d2ff shufps xmm2, xmm2, 0xff
1412ab348: 0f5cd4 subps xmm2, xmm4
1412ab34b: 0f59d1 mulps xmm2, xmm1
1412ab34e: 410f28c8 movaps xmm1, xmm8
1412ab352: 0f59c8 mulps xmm1, xmm0
1412ab355: f30f16d9 movshdup xmm3, xmm1
1412ab359: f30f58d9 addss xmm3, xmm1
1412ab35d: 0f12c9 movhlps xmm1, xmm1
1412ab360: f30f58cb addss xmm1, xmm3
1412ab364: f30f58c9 addss xmm1, xmm1
1412ab368: 0fc6c900 shufps xmm1, xmm1, 0
1412ab36c: 410f59c8 mulps xmm1, xmm8
1412ab370: 0f58ca addps xmm1, xmm2
1412ab373: 410f28d0 movaps xmm2, xmm8
1412ab377: 410fc6d0d2 shufps xmm2, xmm8, 0xd2
1412ab37c: 0f59d0 mulps xmm2, xmm0
1412ab37f: 0fc6c0d2 shufps xmm0, xmm0, 0xd2
1412ab383: 410f59c0 mulps xmm0, xmm8
1412ab387: 0f5cd0 subps xmm2, xmm0
1412ab38a: 0fc6d2d2 shufps xmm2, xmm2, 0xd2
1412ab38e: 410f28c0 movaps xmm0, xmm8
1412ab392: 410f58c0 addps xmm0, xmm8
1412ab396: 0fc6c0ff shufps xmm0, xmm0, 0xff
1412ab39a: 0f59c2 mulps xmm0, xmm2
1412ab39d: 0f58c1 addps xmm0, xmm1
1412ab3a0: 0f58f8 addps xmm7, xmm0
1412ab3a3: 0f12c0 movhlps xmm0, xmm0
1412ab3a6: f30f58f0 addss xmm6, xmm0
1412ab3aa: 837f3001 cmp dword ptr [rdi + 0x30], 1
1412ab3ae: 7510 jne 0x1412ab3c0
1412ab3b0: 8b473c mov eax, dword ptr [rdi + 0x3c]
1412ab3b3: 89442438 mov dword ptr [rsp + 0x38], eax
1412ab3b7: 488b4734 mov rax, qword ptr [rdi + 0x34]
1412ab3bb: 4889442430 mov qword ptr [rsp + 0x30], rax
1412ab3c0: 488b9c2478010000 mov rbx, qword ptr [rsp + 0x178]
1412ab3c8: 488bafe8000000 mov rbp, qword ptr [rdi + 0xe8]
1412ab3cf: 4c8bbfb0000000 mov r15, qword ptr [rdi + 0xb0]
1412ab3d6: 4c8ba7b8000000 mov r12, qword ptr [rdi + 0xb8]
1412ab3dd: 498d4424f7 lea rax, [r12 - 9]
1412ab3e2: 4883f81b cmp rax, 0x1b
1412ab3e6: 0f8731070000 ja 0x1412abb1d
1412ab3ec: 488d0d65fc9701 lea rcx, [rip + 0x197fc65]
1412ab3f3: 48630481 movsxd rax, dword ptr [rcx + rax*4]
1412ab3f7: 4801c8 add rax, rcx
1412ab3fa: ffe0 jmp rax
1412ab3fc: 48b86e616e695f756265 movabs rax, 0x6562755f696e616e
1412ab406: 493307 xor rax, qword ptr [r15]
1412ab409: 410fb64f08 movzx ecx, byte ptr [r15 + 8]
1412ab40e: 4883f172 xor rcx, 0x72
1412ab412: 4809c1 or rcx, rax
1412ab415: 0f8502070000 jne 0x1412abb1d
1412ab41b: 488b8fa0000000 mov rcx, qword ptr [rdi + 0xa0]
1412ab422: 488b97a8000000 mov rdx, qword ptr [rdi + 0xa8]
1412ab429: e852199fff call 0x140c9cd80 ; utils::name_hash::NameHash::compute
1412ab42e: 48898424a8000000 mov qword ptr [rsp + 0xa8], rax
1412ab436: c78424a000000000000000 mov dword ptr [rsp + 0xa0], 0
1412ab441: 488b442430 mov rax, qword ptr [rsp + 0x30]
1412ab446: 488984248c000000 mov qword ptr [rsp + 0x8c], rax
1412ab44e: 8b442438 mov eax, dword ptr [rsp + 0x38]
1412ab452: 89842494000000 mov dword ptr [rsp + 0x94], eax
1412ab459: e82207d9fe call 0x14003bb80 ; bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>
1412ab45e: c68424b000000017 mov byte ptr [rsp + 0xb0], 0x17
1412ab466: 440f29442470 movaps xmmword ptr [rsp + 0x70], xmm8
1412ab46c: 0f13bc2480000000 movlps qword ptr [rsp + 0x80], xmm7
1412ab474: f30f11b42488000000 movss dword ptr [rsp + 0x88], xmm6
1412ab47d: 4889ac2498000000 mov qword ptr [rsp + 0x98], rbp
1412ab485: 4c8d0d3cfa9701 lea r9, [rip + 0x197fa3c]
1412ab48c: 488d4c2440 lea rcx, [rsp + 0x40]
1412ab491: 4c8d442470 lea r8, [rsp + 0x70]
1412ab496: 4889da mov rdx, rbx
1412ab499: e8422bfcff call 0x14126dfe0 ; bevy_ecs::system::commands::Commands::spawn<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::UberInstanceData> >
1412ab49e: e972090000 jmp 0x1412abe15 ; unknown
1412ab4a3: f3410f6f07 movdqu xmm0, xmmword ptr [r15]
1412ab4a8: 410fb64710 movzx eax, byte ptr [r15 + 0x10]
1412ab4ad: 660f6ec8 movd xmm1, eax
1412ab4b1: 660f7405f7f89701 pcmpeqb xmm0, xmmword ptr [rip + 0x197f8f7]
1412ab4b9: 660f740d3f0c6801 pcmpeqb xmm1, xmmword ptr [rip + 0x1680c3f]
1412ab4c1: 660fdbc8 pand xmm1, xmm0
1412ab4c5: 660fd7c1 pmovmskb eax, xmm1
1412ab4c9: 3dffff0000 cmp eax, 0xffff
1412ab4ce: 0f8549060000 jne 0x1412abb1d
1412ab4d4: 488b8fa0000000 mov rcx, qword ptr [rdi + 0xa0]
1412ab4db: 488b97a8000000 mov rdx, qword ptr [rdi + 0xa8]
1412ab4e2: e899189fff call 0x140c9cd80 ; utils::name_hash::NameHash::compute
1412ab4e7: 48898424a8000000 mov qword ptr [rsp + 0xa8], rax
1412ab4ef: c78424a000000000000000 mov dword ptr [rsp + 0xa0], 0
1412ab4fa: 488b442430 mov rax, qword ptr [rsp + 0x30]
1412ab4ff: 488984248c000000 mov qword ptr [rsp + 0x8c], rax
1412ab507: 8b442438 mov eax, dword ptr [rsp + 0x38]
1412ab50b: 89842494000000 mov dword ptr [rsp + 0x94], eax
1412ab512: e86906d9fe call 0x14003bb80 ; bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>
1412ab517: c68424b000000016 mov byte ptr [rsp + 0xb0], 0x16
1412ab51f: 440f29442470 movaps xmmword ptr [rsp + 0x70], xmm8
1412ab525: 0f13bc2480000000 movlps qword ptr [rsp + 0x80], xmm7
1412ab52d: f30f11b42488000000 movss dword ptr [rsp + 0x88], xmm6
1412ab536: 4889ac2498000000 mov qword ptr [rsp + 0x98], rbp
1412ab53e: 4c8d0d9bf99701 lea r9, [rip + 0x197f99b]
1412ab545: 488d4c2440 lea rcx, [rsp + 0x40]
1412ab54a: 4c8d442470 lea r8, [rsp + 0x70]
1412ab54f: 4889da mov rdx, rbx
1412ab552: e88929fcff call 0x14126dee0 ; bevy_ecs::system::commands::Commands::spawn<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::BalconyDoorInstanceData> >
1412ab557: e9b9080000 jmp 0x1412abe15 ; unknown
1412ab55c: f3410f6f07 movdqu xmm0, xmmword ptr [r15]
1412ab561: f3410f6f4f09 movdqu xmm1, xmmword ptr [r15 + 9]
1412ab567: 660f740d712c8101 pcmpeqb xmm1, xmmword ptr [rip + 0x1812c71]
1412ab56f: 660f7405792c8101 pcmpeqb xmm0, xmmword ptr [rip + 0x1812c79]
1412ab577: 660fdbc1 pand xmm0, xmm1
1412ab57b: 660fd7c0 pmovmskb eax, xmm0
1412ab57f: 3dffff0000 cmp eax, 0xffff
1412ab584: 0f8593050000 jne 0x1412abb1d
1412ab58a: 488b8fa0000000 mov rcx, qword ptr [rdi + 0xa0]
1412ab591: 488b97a8000000 mov rdx, qword ptr [rdi + 0xa8]
1412ab598: e8e3179fff call 0x140c9cd80 ; utils::name_hash::NameHash::compute
1412ab59d: 48898424a8000000 mov qword ptr [rsp + 0xa8], rax
1412ab5a5: c78424a000000000000000 mov dword ptr [rsp + 0xa0], 0
1412ab5b0: 488b442430 mov rax, qword ptr [rsp + 0x30]
1412ab5b5: 488984248c000000 mov qword ptr [rsp + 0x8c], rax
1412ab5bd: 8b442438 mov eax, dword ptr [rsp + 0x38]
1412ab5c1: 89842494000000 mov dword ptr [rsp + 0x94], eax
1412ab5c8: e8b305d9fe call 0x14003bb80 ; bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>
1412ab5cd: c68424b000000007 mov byte ptr [rsp + 0xb0], 7
1412ab5d5: 440f29442470 movaps xmmword ptr [rsp + 0x70], xmm8
1412ab5db: 0f13bc2480000000 movlps qword ptr [rsp + 0x80], xmm7
1412ab5e3: f30f11b42488000000 movss dword ptr [rsp + 0x88], xmm6
1412ab5ec: 4889ac2498000000 mov qword ptr [rsp + 0x98], rbp
1412ab5f4: 4c8d0d45f99701 lea r9, [rip + 0x197f945]
1412ab5fb: 488d4c2440 lea rcx, [rsp + 0x40]
1412ab600: 4c8d442470 lea r8, [rsp + 0x70]
1412ab605: 4889da mov rdx, rbx
1412ab608: e8e330fcff call 0x14126e6f0 ; bevy_ecs::system::commands::Commands::spawn<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::GothicWindowBricksInstanceData> >
1412ab60d: e903080000 jmp 0x1412abe15 ; unknown
1412ab612: 48b86e616e695f636c6f movabs rax, 0x6f6c635f696e616e
1412ab61c: 493307 xor rax, qword ptr [r15]
1412ab61f: 410fb74f08 movzx ecx, word ptr [r15 + 8]
1412ab624: 4881f174680000 xor rcx, 0x6874
1412ab62b: 4809c1 or rcx, rax
1412ab62e: 0f8480050000 je 0x1412abbb4
1412ab634: 48b86e616e695f736d6f movabs rax, 0x6f6d735f696e616e
1412ab63e: 493307 xor rax, qword ptr [r15]
1412ab641: 410fb74f08 movzx ecx, word ptr [r15 + 8]
1412ab646: 4881f16b650000 xor rcx, 0x656b
1412ab64d: 4809c1 or rcx, rax
1412ab650: 0f85c7040000 jne 0x1412abb1d
1412ab656: 31c9 xor ecx, ecx
1412ab658: e8f37b58ff call 0x140833250 ; country_core::resources::walls::decorator::DecoratorRank::new
1412ab65d: 488d8c24a0000000 lea rcx, [rsp + 0xa0]
1412ab665: 89c2 mov edx, eax
1412ab667: e8f40658ff call 0x14082bd60 ; country_core::components::drawable::impl$1::from
1412ab66c: 8b442438 mov eax, dword ptr [rsp + 0x38]
1412ab670: 89842494000000 mov dword ptr [rsp + 0x94], eax
1412ab677: 488b442430 mov rax, qword ptr [rsp + 0x30]
1412ab67c: 488984248c000000 mov qword ptr [rsp + 0x8c], rax
1412ab684: e8f704d9fe call 0x14003bb80 ; bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>
1412ab689: c68424b000000015 mov byte ptr [rsp + 0xb0], 0x15
1412ab691: 440f29442470 movaps xmmword ptr [rsp + 0x70], xmm8
1412ab697: 0f13bc2480000000 movlps qword ptr [rsp + 0x80], xmm7
1412ab69f: f30f11b42488000000 movss dword ptr [rsp + 0x88], xmm6
1412ab6a8: 4889ac2498000000 mov qword ptr [rsp + 0x98], rbp
1412ab6b0: 4c8d0d41f89701 lea r9, [rip + 0x197f841]
1412ab6b7: 488d4c2440 lea rcx, [rsp + 0x40]
1412ab6bc: 4c8d442470 lea r8, [rsp + 0x70]
1412ab6c1: 4889da mov rdx, rbx
1412ab6c4: e8172afcff call 0x14126e0e0 ; bevy_ecs::system::commands::Commands::spawn<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::SmokeInstanceData> >
1412ab6c9: e947070000 jmp 0x1412abe15 ; unknown
1412ab6ce: f3410f6f07 movdqu xmm0, xmmword ptr [r15]
1412ab6d3: f3410f6f4f0b movdqu xmm1, xmmword ptr [r15 + 0xb]
1412ab6d9: 660f740d0ff79701 pcmpeqb xmm1, xmmword ptr [rip + 0x197f70f]
1412ab6e1: 660f740517f79701 pcmpeqb xmm0, xmmword ptr [rip + 0x197f717]
1412ab6e9: 660fdbc1 pand xmm0, xmm1
1412ab6ed: 660fd7c0 pmovmskb eax, xmm0
1412ab6f1: 3dffff0000 cmp eax, 0xffff
1412ab6f6: 0f8521040000 jne 0x1412abb1d
1412ab6fc: 488b8fa0000000 mov rcx, qword ptr [rdi + 0xa0]
1412ab703: 488b97a8000000 mov rdx, qword ptr [rdi + 0xa8]
1412ab70a: e871169fff call 0x140c9cd80 ; utils::name_hash::NameHash::compute
1412ab70f: 4989c6 mov r14, rax
1412ab712: e86904d9fe call 0x14003bb80 ; bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>
1412ab717: 440fb6a750010000 movzx r12d, byte ptr [rdi + 0x150]
1412ab71f: 4180fc09 cmp r12b, 9
1412ab723: 0f846b120000 je 0x1412ac994
1412ab729: 488d4c2470 lea rcx, [rsp + 0x70]
1412ab72e: 4889da mov rdx, rbx
1412ab731: e86a8fa6ff call 0x140d146a0 ; bevy_ecs::system::commands::Commands::spawn_empty
1412ab736: 488b5c2478 mov rbx, qword ptr [rsp + 0x78]
1412ab73b: 4885db test rbx, rbx
1412ab73e: 7508 jne 0x1412ab748
1412ab740: 488b9c2480000000 mov rbx, qword ptr [rsp + 0x80]
1412ab748: 4c8bac2490000000 mov r13, qword ptr [rsp + 0x90]
1412ab750: 488b03 mov rax, qword ptr [rbx]
1412ab753: 4c8b7b10 mov r15, qword ptr [rbx + 0x10]
1412ab757: 4c29f8 sub rax, r15
1412ab75a: 483d97000000 cmp rax, 0x97
1412ab760: 0f86ad110000 jbe 0x1412ac913
1412ab766: 488b4308 mov rax, qword ptr [rbx + 8]
1412ab76a: 488d0d2ffffbff lea rcx, [rip - 0x400d1]
1412ab771: 4a890c38 mov qword ptr [rax + r15], rcx
1412ab775: 460f11443808 movups xmmword ptr [rax + r15 + 8], xmm8
1412ab77b: 420f137c3818 movlps qword ptr [rax + r15 + 0x18], xmm7
1412ab781: f3420f11743820 movss dword ptr [rax + r15 + 0x20], xmm6
1412ab788: 488b4c2430 mov rcx, qword ptr [rsp + 0x30]
1412ab78d: 4a894c3824 mov qword ptr [rax + r15 + 0x24], rcx
1412ab792: 8b4c2438 mov ecx, dword ptr [rsp + 0x38]
1412ab796: 42894c382c mov dword ptr [rax + r15 + 0x2c], ecx
1412ab79b: 4a896c3830 mov qword ptr [rax + r15 + 0x30], rbp
1412ab7a0: 42c744383800000000 mov dword ptr [rax + r15 + 0x38], 0
1412ab7a9: 4e89743840 mov qword ptr [rax + r15 + 0x40], r14
1412ab7ae: 42c64438480f mov byte ptr [rax + r15 + 0x48], 0xf
1412ab7b4: e90e060000 jmp 0x1412abdc7 ; unknown
1412ab7b9: f3410f6f07 movdqu xmm0, xmmword ptr [r15]
1412ab7be: f3410f6f4f0e movdqu xmm1, xmmword ptr [r15 + 0xe]
1412ab7c4: 660f740db42a8101 pcmpeqb xmm1, xmmword ptr [rip + 0x1812ab4]
1412ab7cc: 660f74053c2a8101 pcmpeqb xmm0, xmmword ptr [rip + 0x1812a3c]
1412ab7d4: 660fdbc1 pand xmm0, xmm1
1412ab7d8: 660fd7c0 pmovmskb eax, xmm0
1412ab7dc: 3dffff0000 cmp eax, 0xffff
1412ab7e1: 0f8455040000 je 0x1412abc3c
1412ab7e7: f3410f6f07 movdqu xmm0, xmmword ptr [r15]
1412ab7ec: f3410f6f4f0e movdqu xmm1, xmmword ptr [r15 + 0xe]
1412ab7f2: 660f740d062a8101 pcmpeqb xmm1, xmmword ptr [rip + 0x1812a06]
1412ab7fa: 660f74050e2a8101 pcmpeqb xmm0, xmmword ptr [rip + 0x1812a0e]
1412ab802: 660fdbc1 pand xmm0, xmm1
1412ab806: 660fd7c0 pmovmskb eax, xmm0
1412ab80a: 3dffff0000 cmp eax, 0xffff
1412ab80f: 0f8508030000 jne 0x1412abb1d
1412ab815: 488b8fa0000000 mov rcx, qword ptr [rdi + 0xa0]
1412ab81c: 488b97a8000000 mov rdx, qword ptr [rdi + 0xa8]
1412ab823: e858159fff call 0x140c9cd80 ; utils::name_hash::NameHash::compute
1412ab828: 48898424a8000000 mov qword ptr [rsp + 0xa8], rax
1412ab830: c78424a000000000000000 mov dword ptr [rsp + 0xa0], 0
1412ab83b: 488b442430 mov rax, qword ptr [rsp + 0x30]
1412ab840: 488984248c000000 mov qword ptr [rsp + 0x8c], rax
1412ab848: 8b442438 mov eax, dword ptr [rsp + 0x38]
1412ab84c: 89842494000000 mov dword ptr [rsp + 0x94], eax
1412ab853: e82803d9fe call 0x14003bb80 ; bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>
1412ab858: c68424b000000006 mov byte ptr [rsp + 0xb0], 6
1412ab860: 440f29442470 movaps xmmword ptr [rsp + 0x70], xmm8
1412ab866: 0f13bc2480000000 movlps qword ptr [rsp + 0x80], xmm7
1412ab86e: f30f11b42488000000 movss dword ptr [rsp + 0x88], xmm6
1412ab877: 4889ac2498000000 mov qword ptr [rsp + 0x98], rbp
1412ab87f: 4c8d0dd2f69701 lea r9, [rip + 0x197f6d2]
1412ab886: 488d4c2440 lea rcx, [rsp + 0x40]
1412ab88b: 4c8d442470 lea r8, [rsp + 0x70]
1412ab890: 4889da mov rdx, rbx
1412ab893: e8582ffcff call 0x14126e7f0 ; bevy_ecs::system::commands::Commands::spawn<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::SolidVertexColorBricksInstanceData> >
1412ab898: e978050000 jmp 0x1412abe15 ; unknown
1412ab89d: f3410f6f07 movdqu xmm0, xmmword ptr [r15]
1412ab8a2: f3410f6f4f05 movdqu xmm1, xmmword ptr [r15 + 5]
1412ab8a8: 660f740d10f59701 pcmpeqb xmm1, xmmword ptr [rip + 0x197f510]
1412ab8b0: 660f740518f59701 pcmpeqb xmm0, xmmword ptr [rip + 0x197f518]
1412ab8b8: 660fdbc1 pand xmm0, xmm1
1412ab8bc: 660fd7c0 pmovmskb eax, xmm0
1412ab8c0: 3dffff0000 cmp eax, 0xffff
1412ab8c5: 0f8552020000 jne 0x1412abb1d
1412ab8cb: 488b8fa0000000 mov rcx, qword ptr [rdi + 0xa0]
1412ab8d2: 488b97a8000000 mov rdx, qword ptr [rdi + 0xa8]
1412ab8d9: e8a2149fff call 0x140c9cd80 ; utils::name_hash::NameHash::compute
1412ab8de: 48898424a8000000 mov qword ptr [rsp + 0xa8], rax
1412ab8e6: c78424a000000000000000 mov dword ptr [rsp + 0xa0], 0
1412ab8f1: 488b442430 mov rax, qword ptr [rsp + 0x30]
1412ab8f6: 488984248c000000 mov qword ptr [rsp + 0x8c], rax
1412ab8fe: 8b442438 mov eax, dword ptr [rsp + 0x38]
1412ab902: 89842494000000 mov dword ptr [rsp + 0x94], eax
1412ab909: e87202d9fe call 0x14003bb80 ; bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>
1412ab90e: c68424b000000012 mov byte ptr [rsp + 0xb0], 0x12
1412ab916: 440f29442470 movaps xmmword ptr [rsp + 0x70], xmm8
1412ab91c: 0f13bc2480000000 movlps qword ptr [rsp + 0x80], xmm7
1412ab924: f30f11b42488000000 movss dword ptr [rsp + 0x88], xmm6
1412ab92d: 4889ac2498000000 mov qword ptr [rsp + 0x98], rbp
1412ab935: 4c8d0decf59701 lea r9, [rip + 0x197f5ec]
1412ab93c: 488d4c2440 lea rcx, [rsp + 0x40]
1412ab941: 4c8d442470 lea r8, [rsp + 0x70]
1412ab946: 4889da mov rdx, rbx
1412ab949: e8a22cfcff call 0x14126e5f0 ; bevy_ecs::system::commands::Commands::spawn<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::LanternEmissiveInstanceData> >
1412ab94e: e9c2040000 jmp 0x1412abe15 ; unknown
1412ab953: f3410f6f07 movdqu xmm0, xmmword ptr [r15]
1412ab958: f3410f6f4f07 movdqu xmm1, xmmword ptr [r15 + 7]
1412ab95e: 660f740d7af49701 pcmpeqb xmm1, xmmword ptr [rip + 0x197f47a]
1412ab966: 660f7405a2288101 pcmpeqb xmm0, xmmword ptr [rip + 0x18128a2]
1412ab96e: 660fdbc1 pand xmm0, xmm1
1412ab972: 660fd7c0 pmovmskb eax, xmm0
1412ab976: 3dffff0000 cmp eax, 0xffff
1412ab97b: 0f859c010000 jne 0x1412abb1d
1412ab981: 488b8fa0000000 mov rcx, qword ptr [rdi + 0xa0]
1412ab988: 488b97a8000000 mov rdx, qword ptr [rdi + 0xa8]
1412ab98f: e8ec139fff call 0x140c9cd80 ; utils::name_hash::NameHash::compute
1412ab994: 4989c6 mov r14, rax
1412ab997: e8e401d9fe call 0x14003bb80 ; bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>
1412ab99c: 488d4c2470 lea rcx, [rsp + 0x70]
1412ab9a1: 4889da mov rdx, rbx
1412ab9a4: e8f78ca6ff call 0x140d146a0 ; bevy_ecs::system::commands::Commands::spawn_empty
1412ab9a9: 488b5c2478 mov rbx, qword ptr [rsp + 0x78]
1412ab9ae: 4885db test rbx, rbx
1412ab9b1: 7508 jne 0x1412ab9bb
1412ab9b3: 488b9c2480000000 mov rbx, qword ptr [rsp + 0x80]
1412ab9bb: 4c8ba42490000000 mov r12, qword ptr [rsp + 0x90]
1412ab9c3: 488b03 mov rax, qword ptr [rbx]
1412ab9c6: 4c8b7b10 mov r15, qword ptr [rbx + 0x10]
1412ab9ca: 4c29f8 sub rax, r15
1412ab9cd: 483d87000000 cmp rax, 0x87
1412ab9d3: 0f86f00e0000 jbe 0x1412ac8c9
1412ab9d9: 488b4308 mov rax, qword ptr [rbx + 8]
1412ab9dd: 488d0d7c12fcff lea rcx, [rip - 0x3ed84]
1412ab9e4: 4a890c38 mov qword ptr [rax + r15], rcx
1412ab9e8: 460f11443808 movups xmmword ptr [rax + r15 + 8], xmm8
1412ab9ee: 420f137c3818 movlps qword ptr [rax + r15 + 0x18], xmm7
1412ab9f4: f3420f11743820 movss dword ptr [rax + r15 + 0x20], xmm6
1412ab9fb: 488b4c2430 mov rcx, qword ptr [rsp + 0x30]
1412aba00: 4a894c3824 mov qword ptr [rax + r15 + 0x24], rcx
1412aba05: 8b4c2438 mov ecx, dword ptr [rsp + 0x38]
1412aba09: 42894c382c mov dword ptr [rax + r15 + 0x2c], ecx
1412aba0e: 4a896c3830 mov qword ptr [rax + r15 + 0x30], rbp
1412aba13: 42c744383800000000 mov dword ptr [rax + r15 + 0x38], 0
1412aba1c: 4e89743840 mov qword ptr [rax + r15 + 0x40], r14
1412aba21: 42c644384803 mov byte ptr [rax + r15 + 0x48], 3
1412aba27: e9c0020000 jmp 0x1412abcec ; unknown
1412aba2c: 488d1500f49701 lea rdx, [rip + 0x197f400]
1412aba33: 4d89c5 mov r13, r8
1412aba36: 41b824000000 mov r8d, 0x24
1412aba3c: 4c89f9 mov rcx, r15
1412aba3f: e8ac816701 call 0x142923bf0 ; unknown
1412aba44: 4d89e8 mov r8, r13
1412aba47: 85c0 test eax, eax
1412aba49: 0f85ce000000 jne 0x1412abb1d
1412aba4f: 488b8fa0000000 mov rcx, qword ptr [rdi + 0xa0]
1412aba56: 488b97a8000000 mov rdx, qword ptr [rdi + 0xa8]
1412aba5d: e81e139fff call 0x140c9cd80 ; utils::name_hash::NameHash::compute
1412aba62: 4989c6 mov r14, rax
1412aba65: e81601d9fe call 0x14003bb80 ; bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>
1412aba6a: 488d4c2470 lea rcx, [rsp + 0x70]
1412aba6f: 4889da mov rdx, rbx
1412aba72: e8298ca6ff call 0x140d146a0 ; bevy_ecs::system::commands::Commands::spawn_empty
1412aba77: 488b5c2478 mov rbx, qword ptr [rsp + 0x78]
1412aba7c: 4885db test rbx, rbx
1412aba7f: 7508 jne 0x1412aba89
1412aba81: 488b9c2480000000 mov rbx, qword ptr [rsp + 0x80]
1412aba89: 4c8ba42490000000 mov r12, qword ptr [rsp + 0x90]
1412aba91: 488b03 mov rax, qword ptr [rbx]
1412aba94: 4c8b7b10 mov r15, qword ptr [rbx + 0x10]
1412aba98: 4c29f8 sub rax, r15
1412aba9b: 483d87000000 cmp rax, 0x87
1412abaa1: 0f86470e0000 jbe 0x1412ac8ee
1412abaa7: 488b4308 mov rax, qword ptr [rbx + 8]
1412abaab: 488d0d9e16fcff lea rcx, [rip - 0x3e962]
1412abab2: 4a890c38 mov qword ptr [rax + r15], rcx
1412abab6: 460f11443808 movups xmmword ptr [rax + r15 + 8], xmm8
1412ababc: 420f137c3818 movlps qword ptr [rax + r15 + 0x18], xmm7
1412abac2: f3420f11743820 movss dword ptr [rax + r15 + 0x20], xmm6
1412abac9: 488b4c2430 mov rcx, qword ptr [rsp + 0x30]
1412abace: 4a894c3824 mov qword ptr [rax + r15 + 0x24], rcx
1412abad3: 8b4c2438 mov ecx, dword ptr [rsp + 0x38]
1412abad7: 42894c382c mov dword ptr [rax + r15 + 0x2c], ecx
1412abadc: 4a896c3830 mov qword ptr [rax + r15 + 0x30], rbp
1412abae1: 42c744383800000000 mov dword ptr [rax + r15 + 0x38], 0
1412abaea: 4e89743840 mov qword ptr [rax + r15 + 0x40], r14
1412abaef: 42c644384805 mov byte ptr [rax + r15 + 0x48], 5
1412abaf5: e9f2010000 jmp 0x1412abcec ; unknown
1412abafa: 488d150ff39701 lea rdx, [rip + 0x197f30f]
1412abb01: 4d89c5 mov r13, r8
1412abb04: 41b823000000 mov r8d, 0x23
1412abb0a: 4c89f9 mov rcx, r15
1412abb0d: e8de806701 call 0x142923bf0 ; unknown
1412abb12: 4d89e8 mov r8, r13
1412abb15: 85c0 test eax, eax
1412abb17: 0f84f2010000 je 0x1412abd0f
1412abb1d: 488b97a0000000 mov rdx, qword ptr [rdi + 0xa0]
1412abb24: 488b87a8000000 mov rax, qword ptr [rdi + 0xa8]
1412abb2b: 4c89c1 mov rcx, r8
1412abb2e: 4989c0 mov r8, rax
1412abb31: e8ba1968ff call 0x14092d4f0 ; country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_handle_by_name
1412abb36: 4989c5 mov r13, rax
1412abb39: 4c89f1 mov rcx, r14
1412abb3c: 4c89fa mov rdx, r15
1412abb3f: 4d89e0 mov r8, r12
1412abb42: e859bf68ff call 0x140937aa0 ; country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
1412abb47: a801 test al, 1
1412abb49: 0f84330e0000 je 0x1412ac982
1412abb4f: 8b442438 mov eax, dword ptr [rsp + 0x38]
1412abb53: 89842494000000 mov dword ptr [rsp + 0x94], eax
1412abb5a: 488b442430 mov rax, qword ptr [rsp + 0x30]
1412abb5f: 488984248c000000 mov qword ptr [rsp + 0x8c], rax
1412abb67: 4c89ac24a0000000 mov qword ptr [rsp + 0xa0], r13
1412abb6f: 48899424a8000000 mov qword ptr [rsp + 0xa8], rdx
1412abb77: 440f29442470 movaps xmmword ptr [rsp + 0x70], xmm8
1412abb7d: 0f13bc2480000000 movlps qword ptr [rsp + 0x80], xmm7
1412abb85: f30f11b42488000000 movss dword ptr [rsp + 0x88], xmm6
1412abb8e: 4889ac2498000000 mov qword ptr [rsp + 0x98], rbp
1412abb96: 4c8d0d13f39701 lea r9, [rip + 0x197f313]
1412abb9d: 488d4c2440 lea rcx, [rsp + 0x40]
1412abba2: 4c8d442470 lea r8, [rsp + 0x70]
1412abba7: 4889da mov rdx, rbx
1412abbaa: e83126fcff call 0x14126e1e0 ; bevy_ecs::system::commands::Commands::spawn<country_core::components::drawable::DrawableMeshBundle>
1412abbaf: e961020000 jmp 0x1412abe15 ; unknown
1412abbb4: 488b8fa0000000 mov rcx, qword ptr [rdi + 0xa0]
1412abbbb: 488b97a8000000 mov rdx, qword ptr [rdi + 0xa8]
1412abbc2: e8b9119fff call 0x140c9cd80 ; utils::name_hash::NameHash::compute
1412abbc7: 48898424a8000000 mov qword ptr [rsp + 0xa8], rax
1412abbcf: c78424a000000000000000 mov dword ptr [rsp + 0xa0], 0
1412abbda: 488b442430 mov rax, qword ptr [rsp + 0x30]
1412abbdf: 488984248c000000 mov qword ptr [rsp + 0x8c], rax
1412abbe7: 8b442438 mov eax, dword ptr [rsp + 0x38]
1412abbeb: 89842494000000 mov dword ptr [rsp + 0x94], eax
1412abbf2: e889ffd8fe call 0x14003bb80 ; bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>
1412abbf7: c68424b000000014 mov byte ptr [rsp + 0xb0], 0x14
1412abbff: 440f29442470 movaps xmmword ptr [rsp + 0x70], xmm8
1412abc05: 0f13bc2480000000 movlps qword ptr [rsp + 0x80], xmm7
1412abc0d: f30f11b42488000000 movss dword ptr [rsp + 0x88], xmm6
1412abc16: 4889ac2498000000 mov qword ptr [rsp + 0x98], rbp
1412abc1e: 4c8d0debf29701 lea r9, [rip + 0x197f2eb]
1412abc25: 488d4c2440 lea rcx, [rsp + 0x40]
1412abc2a: 4c8d442470 lea r8, [rsp + 0x70]
1412abc2f: 4889da mov rdx, rbx
1412abc32: e8a921fcff call 0x14126dde0 ; bevy_ecs::system::commands::Commands::spawn<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::ClothInstanceData> >
1412abc37: e9d9010000 jmp 0x1412abe15 ; unknown
1412abc3c: 48833f00 cmp qword ptr [rdi], 0
1412abc40: 0f856b0c0000 jne 0x1412ac8b1
1412abc46: 488b8fa0000000 mov rcx, qword ptr [rdi + 0xa0]
1412abc4d: 488b97a8000000 mov rdx, qword ptr [rdi + 0xa8]
1412abc54: e827119fff call 0x140c9cd80 ; utils::name_hash::NameHash::compute
1412abc59: 4989c6 mov r14, rax
1412abc5c: e81fffd8fe call 0x14003bb80 ; bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>
1412abc61: 488d4c2470 lea rcx, [rsp + 0x70]
1412abc66: 4889da mov rdx, rbx
1412abc69: e8328aa6ff call 0x140d146a0 ; bevy_ecs::system::commands::Commands::spawn_empty
1412abc6e: 488b5c2478 mov rbx, qword ptr [rsp + 0x78]
1412abc73: 4885db test rbx, rbx
1412abc76: 7508 jne 0x1412abc80
1412abc78: 488b9c2480000000 mov rbx, qword ptr [rsp + 0x80]
1412abc80: 4c8ba42490000000 mov r12, qword ptr [rsp + 0x90]
1412abc88: 488b03 mov rax, qword ptr [rbx]
1412abc8b: 4c8b7b10 mov r15, qword ptr [rbx + 0x10]
1412abc8f: 4c29f8 sub rax, r15
1412abc92: 483d87000000 cmp rax, 0x87
1412abc98: 0f86bf0c0000 jbe 0x1412ac95d
1412abc9e: 488b4308 mov rax, qword ptr [rbx + 8]
1412abca2: 488d0d0700fcff lea rcx, [rip - 0x3fff9]
1412abca9: 4a890c38 mov qword ptr [rax + r15], rcx
1412abcad: 460f11443808 movups xmmword ptr [rax + r15 + 8], xmm8
1412abcb3: 420f137c3818 movlps qword ptr [rax + r15 + 0x18], xmm7
1412abcb9: f3420f11743820 movss dword ptr [rax + r15 + 0x20], xmm6
1412abcc0: 488b4c2430 mov rcx, qword ptr [rsp + 0x30]
1412abcc5: 4a894c3824 mov qword ptr [rax + r15 + 0x24], rcx
1412abcca: 8b4c2438 mov ecx, dword ptr [rsp + 0x38]
1412abcce: 42894c382c mov dword ptr [rax + r15 + 0x2c], ecx
1412abcd3: 4a896c3830 mov qword ptr [rax + r15 + 0x30], rbp
1412abcd8: 42c744383800000000 mov dword ptr [rax + r15 + 0x38], 0
1412abce1: 4e89743840 mov qword ptr [rax + r15 + 0x40], r14
1412abce6: 42c644384804 mov byte ptr [rax + r15 + 0x48], 4
1412abcec: 42c644385800 mov byte ptr [rax + r15 + 0x58], 0
1412abcf2: 4e89643868 mov qword ptr [rax + r15 + 0x68], r12
1412abcf7: 488d0d7263fdff lea rcx, [rip - 0x29c8e]
1412abcfe: 4a894c3878 mov qword ptr [rax + r15 + 0x78], rcx
1412abd03: 4981c788000000 add r15, 0x88
1412abd0a: e9de000000 jmp 0x1412abded ; unknown
1412abd0f: 488b8fa0000000 mov rcx, qword ptr [rdi + 0xa0]
1412abd16: 488b97a8000000 mov rdx, qword ptr [rdi + 0xa8]
1412abd1d: e85e109fff call 0x140c9cd80 ; utils::name_hash::NameHash::compute
1412abd22: 4989c6 mov r14, rax
1412abd25: e856fed8fe call 0x14003bb80 ; bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>
1412abd2a: 440fb6a750010000 movzx r12d, byte ptr [rdi + 0x150]
1412abd32: 4180fc09 cmp r12b, 9
1412abd36: 0f84640c0000 je 0x1412ac9a0
1412abd3c: 488d4c2470 lea rcx, [rsp + 0x70]
1412abd41: 4889da mov rdx, rbx
1412abd44: e85789a6ff call 0x140d146a0 ; bevy_ecs::system::commands::Commands::spawn_empty
1412abd49: 488b5c2478 mov rbx, qword ptr [rsp + 0x78]
1412abd4e: 4885db test rbx, rbx
1412abd51: 7508 jne 0x1412abd5b
1412abd53: 488b9c2480000000 mov rbx, qword ptr [rsp + 0x80]
1412abd5b: 4c8bac2490000000 mov r13, qword ptr [rsp + 0x90]
1412abd63: 488b03 mov rax, qword ptr [rbx]
1412abd66: 4c8b7b10 mov r15, qword ptr [rbx + 0x10]
1412abd6a: 4c29f8 sub rax, r15
1412abd6d: 483d97000000 cmp rax, 0x97
1412abd73: 0f86bf0b0000 jbe 0x1412ac938
1412abd79: 488b4308 mov rax, qword ptr [rbx + 8]
1412abd7d: 488d0dacfcfbff lea rcx, [rip - 0x40354]
1412abd84: 4a890c38 mov qword ptr [rax + r15], rcx
1412abd88: 460f11443808 movups xmmword ptr [rax + r15 + 8], xmm8
1412abd8e: 420f137c3818 movlps qword ptr [rax + r15 + 0x18], xmm7
1412abd94: f3420f11743820 movss dword ptr [rax + r15 + 0x20], xmm6
1412abd9b: 488b4c2430 mov rcx, qword ptr [rsp + 0x30]
1412abda0: 4a894c3824 mov qword ptr [rax + r15 + 0x24], rcx
1412abda5: 8b4c2438 mov ecx, dword ptr [rsp + 0x38]
1412abda9: 42894c382c mov dword ptr [rax + r15 + 0x2c], ecx
1412abdae: 4a896c3830 mov qword ptr [rax + r15 + 0x30], rbp
1412abdb3: 42c744383800000000 mov dword ptr [rax + r15 + 0x38], 0
1412abdbc: 4e89743840 mov qword ptr [rax + r15 + 0x40], r14
1412abdc1: 42c644384810 mov byte ptr [rax + r15 + 0x48], 0x10
1412abdc7: 4688643858 mov byte ptr [rax + r15 + 0x58], r12b
1412abdcc: 42c644386800 mov byte ptr [rax + r15 + 0x68], 0
1412abdd2: 4e896c3878 mov qword ptr [rax + r15 + 0x78], r13
1412abdd7: 488d0d9262fdff lea rcx, [rip - 0x29d6e]
1412abdde: 4a898c3888000000 mov qword ptr [rax + r15 + 0x88], rcx
1412abde6: 4981c798000000 add r15, 0x98
1412abded: 4c897b10 mov qword ptr [rbx + 0x10], r15
1412abdf1: 488b842490000000 mov rax, qword ptr [rsp + 0x90]
1412abdf9: 4889442460 mov qword ptr [rsp + 0x60], rax
1412abdfe: 0f10442470 movups xmm0, xmmword ptr [rsp + 0x70]
1412abe03: 0f108c2480000000 movups xmm1, xmmword ptr [rsp + 0x80]
1412abe0b: 0f294c2450 movaps xmmword ptr [rsp + 0x50], xmm1
1412abe10: 0f29442440 movaps xmmword ptr [rsp + 0x40], xmm0
1412abe15: f687f000000001 test byte ptr [rdi + 0xf0], 1
1412abe1c: 0f85dd000000 jne 0x1412abeff
1412abe22: 837f4001 cmp dword ptr [rdi + 0x40], 1
1412abe26: 0f843e010000 je 0x1412abf6a
1412abe2c: 80bf1401000002 cmp byte ptr [rdi + 0x114], 2
1412abe33: 0f85c3010000 jne 0x1412abffc
1412abe39: 83bf8000000001 cmp dword ptr [rdi + 0x80], 1
1412abe40: 0f8433020000 je 0x1412ac079
1412abe46: 80bf4e01000000 cmp byte ptr [rdi + 0x14e], 0
1412abe4d: 0f85a1020000 jne 0x1412ac0f4
1412abe53: f6471001 test byte ptr [rdi + 0x10], 1
1412abe57: 0f85f7020000 jne 0x1412ac154
1412abe5d: 80bf4f01000000 cmp byte ptr [rdi + 0x14f], 0
1412abe64: 0f855b030000 jne 0x1412ac1c5
1412abe6a: 0fb6af4c010000 movzx ebp, byte ptr [rdi + 0x14c]
1412abe71: 4080fd02 cmp bpl, 2
1412abe75: 0f8558040000 jne 0x1412ac2d3
1412abe7b: 837f2001 cmp dword ptr [rdi + 0x20], 1
1412abe7f: 0f84d1040000 je 0x1412ac356
1412abe85: f60701 test byte ptr [rdi], 1
1412abe88: 0f8430050000 je 0x1412ac3be
1412abe8e: 4c8b742448 mov r14, qword ptr [rsp + 0x48]
1412abe93: 4d85f6 test r14, r14
1412abe96: 7505 jne 0x1412abe9d
1412abe98: 4c8b742450 mov r14, qword ptr [rsp + 0x50]
1412abe9d: 4c8b6708 mov r12, qword ptr [rdi + 8]
1412abea1: 488b5c2460 mov rbx, qword ptr [rsp + 0x60]
1412abea6: 498b06 mov rax, qword ptr [r14]
1412abea9: 4d8b7e10 mov r15, qword ptr [r14 + 0x10]
1412abead: 4c29f8 sub rax, r15
1412abeb0: 4883f827 cmp rax, 0x27
1412abeb4: 0f86eb070000 jbe 0x1412ac6a5
1412abeba: 498b4608 mov rax, qword ptr [r14 + 8]
1412abebe: 488d0d8bf9fbff lea rcx, [rip - 0x40675]
1412abec5: 4a890c38 mov qword ptr [rax + r15], rcx
1412abec9: 488d0da061fdff lea rcx, [rip - 0x29e60]
1412abed0: 4a894c3808 mov qword ptr [rax + r15 + 8], rcx
1412abed5: 4a895c3810 mov qword ptr [rax + r15 + 0x10], rbx
1412abeda: 4e89643818 mov qword ptr [rax + r15 + 0x18], r12
1412abedf: 42c644382000 mov byte ptr [rax + r15 + 0x20], 0
1412abee5: 4983c728 add r15, 0x28
1412abee9: 4d897e10 mov qword ptr [r14 + 0x10], r15
1412abeed: 83bff800000001 cmp dword ptr [rdi + 0xf8], 1
1412abef4: 0f84d2040000 je 0x1412ac3cc
1412abefa: e929050000 jmp 0x1412ac428 ; unknown
1412abeff: 488b5c2448 mov rbx, qword ptr [rsp + 0x48]
1412abf04: 4885db test rbx, rbx
1412abf07: 7505 jne 0x1412abf0e
1412abf09: 488b5c2450 mov rbx, qword ptr [rsp + 0x50]
1412abf0e: 8baff4000000 mov ebp, dword ptr [rdi + 0xf4]
1412abf14: 4c8b7c2460 mov r15, qword ptr [rsp + 0x60]
1412abf19: 488b03 mov rax, qword ptr [rbx]
1412abf1c: 4c8b7310 mov r14, qword ptr [rbx + 0x10]
1412abf20: 4c29f0 sub rax, r14
1412abf23: 4883f81f cmp rax, 0x1f
1412abf27: 0f869d070000 jbe 0x1412ac6ca
1412abf2d: 488b4308 mov rax, qword ptr [rbx + 8]
1412abf31: 488d0d9814fcff lea rcx, [rip - 0x3eb68]
1412abf38: 4a890c30 mov qword ptr [rax + r14], rcx
