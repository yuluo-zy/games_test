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
1412abf3c: 488d0d2d61fdff lea rcx, [rip - 0x29ed3]
1412abf43: 4a894c3008 mov qword ptr [rax + r14 + 8], rcx
1412abf48: 4e897c3010 mov qword ptr [rax + r14 + 0x10], r15
1412abf4d: 42896c3018 mov dword ptr [rax + r14 + 0x18], ebp
1412abf52: 42c644301c00 mov byte ptr [rax + r14 + 0x1c], 0
1412abf58: 4983c620 add r14, 0x20
1412abf5c: 4c897310 mov qword ptr [rbx + 0x10], r14
1412abf60: 837f4001 cmp dword ptr [rdi + 0x40], 1
1412abf64: 0f85c2feffff jne 0x1412abe2c
1412abf6a: 488b5c2448 mov rbx, qword ptr [rsp + 0x48]
1412abf6f: 4885db test rbx, rbx
1412abf72: 7505 jne 0x1412abf79
1412abf74: 488b5c2450 mov rbx, qword ptr [rsp + 0x50]
1412abf79: 4c8d7f44 lea r15, [rdi + 0x44]
1412abf7d: 4c8b642460 mov r12, qword ptr [rsp + 0x60]
1412abf82: 488b03 mov rax, qword ptr [rbx]
1412abf85: 4c8b7310 mov r14, qword ptr [rbx + 0x10]
1412abf89: 4c29f0 sub rax, r14
1412abf8c: 4883f857 cmp rax, 0x57
1412abf90: 0f8659070000 jbe 0x1412ac6ef
1412abf96: 488b4308 mov rax, qword ptr [rbx + 8]
1412abf9a: 488d0dcf04fcff lea rcx, [rip - 0x3fb31]
1412abfa1: 4a890c30 mov qword ptr [rax + r14], rcx
1412abfa5: 488d0dc460fdff lea rcx, [rip - 0x29f3c]
1412abfac: 4a894c3008 mov qword ptr [rax + r14 + 8], rcx
1412abfb1: 4e89643010 mov qword ptr [rax + r14 + 0x10], r12
1412abfb6: 410f1007 movups xmm0, xmmword ptr [r15]
1412abfba: 410f104f10 movups xmm1, xmmword ptr [r15 + 0x10]
1412abfbf: 410f105720 movups xmm2, xmmword ptr [r15 + 0x20]
1412abfc4: 410f105f2c movups xmm3, xmmword ptr [r15 + 0x2c]
1412abfc9: 420f11443018 movups xmmword ptr [rax + r14 + 0x18], xmm0
1412abfcf: 420f114c3028 movups xmmword ptr [rax + r14 + 0x28], xmm1
1412abfd5: 420f11543038 movups xmmword ptr [rax + r14 + 0x38], xmm2
1412abfdb: 420f115c3044 movups xmmword ptr [rax + r14 + 0x44], xmm3
1412abfe1: 42c644305400 mov byte ptr [rax + r14 + 0x54], 0
1412abfe7: 4983c658 add r14, 0x58
1412abfeb: 4c897310 mov qword ptr [rbx + 0x10], r14
1412abfef: 80bf1401000002 cmp byte ptr [rdi + 0x114], 2
1412abff6: 0f843dfeffff je 0x1412abe39
1412abffc: 488b5c2448 mov rbx, qword ptr [rsp + 0x48]
1412ac001: 4885db test rbx, rbx
1412ac004: 7505 jne 0x1412ac00b
1412ac006: 488b5c2450 mov rbx, qword ptr [rsp + 0x50]
1412ac00b: 4c8dbf00010000 lea r15, [rdi + 0x100]
1412ac012: 4c8b642460 mov r12, qword ptr [rsp + 0x60]
1412ac017: 488b03 mov rax, qword ptr [rbx]
1412ac01a: 4c8b7310 mov r14, qword ptr [rbx + 0x10]
1412ac01e: 4c29f0 sub rax, r14
1412ac021: 4883f837 cmp rax, 0x37
1412ac025: 0f86e9060000 jbe 0x1412ac714
1412ac02b: 488b4308 mov rax, qword ptr [rbx + 8]
1412ac02f: 488d0daaf1fbff lea rcx, [rip - 0x40e56]
1412ac036: 4a890c30 mov qword ptr [rax + r14], rcx
1412ac03a: 488d0d2f60fdff lea rcx, [rip - 0x29fd1]
1412ac041: 4a894c3008 mov qword ptr [rax + r14 + 8], rcx
1412ac046: 4e89643010 mov qword ptr [rax + r14 + 0x10], r12
1412ac04b: 410f1007 movups xmm0, xmmword ptr [r15]
1412ac04f: 420f11443018 movups xmmword ptr [rax + r14 + 0x18], xmm0
1412ac055: 498b4f10 mov rcx, qword ptr [r15 + 0x10]
1412ac059: 4a894c3028 mov qword ptr [rax + r14 + 0x28], rcx
1412ac05e: 42c644303000 mov byte ptr [rax + r14 + 0x30], 0
1412ac064: 4983c638 add r14, 0x38
1412ac068: 4c897310 mov qword ptr [rbx + 0x10], r14
1412ac06c: 83bf8000000001 cmp dword ptr [rdi + 0x80], 1
1412ac073: 0f85cdfdffff jne 0x1412abe46
1412ac079: 488b5c2448 mov rbx, qword ptr [rsp + 0x48]
1412ac07e: 4885db test rbx, rbx
1412ac081: 7505 jne 0x1412ac088
1412ac083: 488b5c2450 mov rbx, qword ptr [rsp + 0x50]
1412ac088: 4c8dbf84000000 lea r15, [rdi + 0x84]
1412ac08f: 4c8b642460 mov r12, qword ptr [rsp + 0x60]
1412ac094: 488b03 mov rax, qword ptr [rbx]
1412ac097: 4c8b7310 mov r14, qword ptr [rbx + 0x10]
1412ac09b: 4c29f0 sub rax, r14
1412ac09e: 4883f827 cmp rax, 0x27
1412ac0a2: 0f8691060000 jbe 0x1412ac739
1412ac0a8: 488b4308 mov rax, qword ptr [rbx + 8]
1412ac0ac: 488d0d8df8fbff lea rcx, [rip - 0x40773]
1412ac0b3: 4a890c30 mov qword ptr [rax + r14], rcx
1412ac0b7: 488d0db25ffdff lea rcx, [rip - 0x2a04e]
1412ac0be: 4a894c3008 mov qword ptr [rax + r14 + 8], rcx
1412ac0c3: 4e89643010 mov qword ptr [rax + r14 + 0x10], r12
1412ac0c8: 498b0f mov rcx, qword ptr [r15]
1412ac0cb: 4a894c3018 mov qword ptr [rax + r14 + 0x18], rcx
1412ac0d0: 418b4f08 mov ecx, dword ptr [r15 + 8]
1412ac0d4: 42894c3020 mov dword ptr [rax + r14 + 0x20], ecx
1412ac0d9: 42c644302400 mov byte ptr [rax + r14 + 0x24], 0
1412ac0df: 4983c628 add r14, 0x28
1412ac0e3: 4c897310 mov qword ptr [rbx + 0x10], r14
1412ac0e7: 80bf4e01000000 cmp byte ptr [rdi + 0x14e], 0
1412ac0ee: 0f845ffdffff je 0x1412abe53
1412ac0f4: 488b5c2448 mov rbx, qword ptr [rsp + 0x48]
1412ac0f9: 4885db test rbx, rbx
1412ac0fc: 7505 jne 0x1412ac103
1412ac0fe: 488b5c2450 mov rbx, qword ptr [rsp + 0x50]
1412ac103: 4c8b7c2460 mov r15, qword ptr [rsp + 0x60]
1412ac108: 488b03 mov rax, qword ptr [rbx]
1412ac10b: 4c8b7310 mov r14, qword ptr [rbx + 0x10]
1412ac10f: 4c29f0 sub rax, r14
1412ac112: 4883f81f cmp rax, 0x1f
1412ac116: 0f8642060000 jbe 0x1412ac75e
1412ac11c: 488b4308 mov rax, qword ptr [rbx + 8]
1412ac120: 488d0db911fcff lea rcx, [rip - 0x3ee47]
1412ac127: 4a890c30 mov qword ptr [rax + r14], rcx
1412ac12b: 488d0d3e5ffdff lea rcx, [rip - 0x2a0c2]
1412ac132: 4a894c3008 mov qword ptr [rax + r14 + 8], rcx
1412ac137: 4e897c3010 mov qword ptr [rax + r14 + 0x10], r15
1412ac13c: 42c644301800 mov byte ptr [rax + r14 + 0x18], 0
1412ac142: 4983c620 add r14, 0x20
1412ac146: 4c897310 mov qword ptr [rbx + 0x10], r14
1412ac14a: f6471001 test byte ptr [rdi + 0x10], 1
1412ac14e: 0f8409fdffff je 0x1412abe5d
1412ac154: 488b5c2448 mov rbx, qword ptr [rsp + 0x48]
1412ac159: 4885db test rbx, rbx
1412ac15c: 7505 jne 0x1412ac163
1412ac15e: 488b5c2450 mov rbx, qword ptr [rsp + 0x50]
1412ac163: 4c8b7f18 mov r15, qword ptr [rdi + 0x18]
1412ac167: 4c8b642460 mov r12, qword ptr [rsp + 0x60]
1412ac16c: 488b03 mov rax, qword ptr [rbx]
1412ac16f: 4c8b7310 mov r14, qword ptr [rbx + 0x10]
1412ac173: 4c29f0 sub rax, r14
1412ac176: 4883f82f cmp rax, 0x2f
1412ac17a: 0f8603060000 jbe 0x1412ac783
1412ac180: 488b4308 mov rax, qword ptr [rbx + 8]
1412ac184: 488d0d05f4fbff lea rcx, [rip - 0x40bfb]
1412ac18b: 4a890c30 mov qword ptr [rax + r14], rcx
1412ac18f: 488d0dda5efdff lea rcx, [rip - 0x2a126]
1412ac196: 4a894c3008 mov qword ptr [rax + r14 + 8], rcx
1412ac19b: 4e89643010 mov qword ptr [rax + r14 + 0x10], r12
1412ac1a0: 4e897c3018 mov qword ptr [rax + r14 + 0x18], r15
1412ac1a5: 4289743020 mov dword ptr [rax + r14 + 0x20], esi
1412ac1aa: 42c644302800 mov byte ptr [rax + r14 + 0x28], 0
1412ac1b0: 4983c630 add r14, 0x30
1412ac1b4: 4c897310 mov qword ptr [rbx + 0x10], r14
1412ac1b8: 80bf4f01000000 cmp byte ptr [rdi + 0x14f], 0
1412ac1bf: 0f84a5fcffff je 0x1412abe6a
1412ac1c5: 410f28c8 movaps xmm1, xmm8
1412ac1c9: 410f59c8 mulps xmm1, xmm8
1412ac1cd: f30f16c1 movshdup xmm0, xmm1
1412ac1d1: f30f58c1 addss xmm0, xmm1
1412ac1d5: 0f28d1 movaps xmm2, xmm1
1412ac1d8: 660f15d1 unpckhpd xmm2, xmm1
1412ac1dc: f30f58d0 addss xmm2, xmm0
1412ac1e0: 0fc6d200 shufps xmm2, xmm2, 0
1412ac1e4: 0fc6c9ff shufps xmm1, xmm1, 0xff
1412ac1e8: 0f5cca subps xmm1, xmm2
1412ac1eb: 0f2805ce0e7201 movaps xmm0, xmmword ptr [rip + 0x1720ece]
1412ac1f2: 0f59c8 mulps xmm1, xmm0
1412ac1f5: 410f59c0 mulps xmm0, xmm8
1412ac1f9: f30f16d0 movshdup xmm2, xmm0
1412ac1fd: f30f58d0 addss xmm2, xmm0
1412ac201: 0f12c0 movhlps xmm0, xmm0
1412ac204: f30f58c2 addss xmm0, xmm2
1412ac208: f30f58c0 addss xmm0, xmm0
1412ac20c: 0fc6c000 shufps xmm0, xmm0, 0
1412ac210: 410f59c0 mulps xmm0, xmm8
1412ac214: 0f58c1 addps xmm0, xmm1
1412ac217: f30f100de5966701 movss xmm1, dword ptr [rip + 0x16796e5]
1412ac21f: 410f28d0 movaps xmm2, xmm8
1412ac223: 0f59d1 mulps xmm2, xmm1
1412ac226: 410f28d8 movaps xmm3, xmm8
1412ac22a: 410fc6d8c9 shufps xmm3, xmm8, 0xc9
1412ac22f: 0f59d9 mulps xmm3, xmm1
1412ac232: 0fc6d2d2 shufps xmm2, xmm2, 0xd2
1412ac236: 0f5cda subps xmm3, xmm2
1412ac239: 450f28c8 movaps xmm9, xmm8
1412ac23d: 450f58c8 addps xmm9, xmm8
1412ac241: 450fc6c9ff shufps xmm9, xmm9, 0xff
1412ac246: 440f59cb mulps xmm9, xmm3
1412ac24a: 488b5c2448 mov rbx, qword ptr [rsp + 0x48]
1412ac24f: 4885db test rbx, rbx
1412ac252: 7505 jne 0x1412ac259
1412ac254: 488b5c2450 mov rbx, qword ptr [rsp + 0x50]
1412ac259: 440f58c8 addps xmm9, xmm0
1412ac25d: 4c8b7c2460 mov r15, qword ptr [rsp + 0x60]
1412ac262: 488b03 mov rax, qword ptr [rbx]
1412ac265: 4c8b7310 mov r14, qword ptr [rbx + 0x10]
1412ac269: 4c29f0 sub rax, r14
1412ac26c: 4883f837 cmp rax, 0x37
1412ac270: 0f8632050000 jbe 0x1412ac7a8
1412ac276: 488b4308 mov rax, qword ptr [rbx + 8]
1412ac27a: 488d0d6ff0fbff lea rcx, [rip - 0x40f91]
1412ac281: 4a890c30 mov qword ptr [rax + r14], rcx
1412ac285: 488d0de45dfdff lea rcx, [rip - 0x2a21c]
1412ac28c: 4a894c3008 mov qword ptr [rax + r14 + 8], rcx
1412ac291: 4e897c3010 mov qword ptr [rax + r14 + 0x10], r15
1412ac296: 420f137c3018 movlps qword ptr [rax + r14 + 0x18], xmm7
1412ac29c: f3420f11743020 movss dword ptr [rax + r14 + 0x20], xmm6
1412ac2a3: 460f134c3024 movlps qword ptr [rax + r14 + 0x24], xmm9
1412ac2a9: 450f12c9 movhlps xmm9, xmm9
1412ac2ad: f3460f114c302c movss dword ptr [rax + r14 + 0x2c], xmm9
1412ac2b4: 42c644303000 mov byte ptr [rax + r14 + 0x30], 0
1412ac2ba: 4983c638 add r14, 0x38
1412ac2be: 4c897310 mov qword ptr [rbx + 0x10], r14
1412ac2c2: 0fb6af4c010000 movzx ebp, byte ptr [rdi + 0x14c]
1412ac2c9: 4080fd02 cmp bpl, 2
1412ac2cd: 0f84a8fbffff je 0x1412abe7b
1412ac2d3: 4189f7 mov r15d, esi
1412ac2d6: 488b5c2448 mov rbx, qword ptr [rsp + 0x48]
1412ac2db: 4885db test rbx, rbx
1412ac2de: 7505 jne 0x1412ac2e5
1412ac2e0: 488b5c2450 mov rbx, qword ptr [rsp + 0x50]
1412ac2e5: 440fb6a74d010000 movzx r12d, byte ptr [rdi + 0x14d]
1412ac2ed: 4c8b6c2460 mov r13, qword ptr [rsp + 0x60]
1412ac2f2: 488b03 mov rax, qword ptr [rbx]
1412ac2f5: 4c8b7310 mov r14, qword ptr [rbx + 0x10]
1412ac2f9: 4c29f0 sub rax, r14
1412ac2fc: 4883f82f cmp rax, 0x2f
1412ac300: 0f86c7040000 jbe 0x1412ac7cd
1412ac306: 488b4308 mov rax, qword ptr [rbx + 8]
1412ac30a: 488d0d3f08fcff lea rcx, [rip - 0x3f7c1]
1412ac311: 4a890c30 mov qword ptr [rax + r14], rcx
1412ac315: 488d0d545dfdff lea rcx, [rip - 0x2a2ac]
1412ac31c: 4a894c3008 mov qword ptr [rax + r14 + 8], rcx
1412ac321: 4e896c3010 mov qword ptr [rax + r14 + 0x10], r13
1412ac326: 4e897c3018 mov qword ptr [rax + r14 + 0x18], r15
1412ac32b: 42c744302000000000 mov dword ptr [rax + r14 + 0x20], 0
1412ac334: 42886c3024 mov byte ptr [rax + r14 + 0x24], bpl
1412ac339: 4688643025 mov byte ptr [rax + r14 + 0x25], r12b
1412ac33e: 42c644302800 mov byte ptr [rax + r14 + 0x28], 0
1412ac344: 4983c630 add r14, 0x30
1412ac348: 4c897310 mov qword ptr [rbx + 0x10], r14
1412ac34c: 837f2001 cmp dword ptr [rdi + 0x20], 1
1412ac350: 0f852ffbffff jne 0x1412abe85
1412ac356: 488b5c2448 mov rbx, qword ptr [rsp + 0x48]
1412ac35b: 4885db test rbx, rbx
1412ac35e: 7505 jne 0x1412ac365
1412ac360: 488b5c2450 mov rbx, qword ptr [rsp + 0x50]
1412ac365: 4c8b7f28 mov r15, qword ptr [rdi + 0x28]
1412ac369: 4c8b642460 mov r12, qword ptr [rsp + 0x60]
1412ac36e: 488b03 mov rax, qword ptr [rbx]
1412ac371: 4c8b7310 mov r14, qword ptr [rbx + 0x10]
1412ac375: 4c29f0 sub rax, r14
1412ac378: 4883f827 cmp rax, 0x27
1412ac37c: 0f8670040000 jbe 0x1412ac7f2
1412ac382: 488b4308 mov rax, qword ptr [rbx + 8]
1412ac386: 488d0d3312fcff lea rcx, [rip - 0x3edcd]
1412ac38d: 4a890c30 mov qword ptr [rax + r14], rcx
1412ac391: 488d0dd85cfdff lea rcx, [rip - 0x2a328]
1412ac398: 4a894c3008 mov qword ptr [rax + r14 + 8], rcx
1412ac39d: 4e89643010 mov qword ptr [rax + r14 + 0x10], r12
1412ac3a2: 4e897c3018 mov qword ptr [rax + r14 + 0x18], r15
1412ac3a7: 42c644302000 mov byte ptr [rax + r14 + 0x20], 0
1412ac3ad: 4983c628 add r14, 0x28
1412ac3b1: 4c897310 mov qword ptr [rbx + 0x10], r14
1412ac3b5: f60701 test byte ptr [rdi], 1
1412ac3b8: 0f85d0faffff jne 0x1412abe8e
1412ac3be: 488b5c2460 mov rbx, qword ptr [rsp + 0x60]
1412ac3c3: 83bff800000001 cmp dword ptr [rdi + 0xf8], 1
1412ac3ca: 755c jne 0x1412ac428
1412ac3cc: 4c8b742448 mov r14, qword ptr [rsp + 0x48]
1412ac3d1: 4d85f6 test r14, r14
1412ac3d4: 7505 jne 0x1412ac3db
1412ac3d6: 4c8b742450 mov r14, qword ptr [rsp + 0x50]
1412ac3db: 8baffc000000 mov ebp, dword ptr [rdi + 0xfc]
1412ac3e1: 498b06 mov rax, qword ptr [r14]
1412ac3e4: 4d8b7e10 mov r15, qword ptr [r14 + 0x10]
1412ac3e8: 4c29f8 sub rax, r15
1412ac3eb: 4883f81f cmp rax, 0x1f
1412ac3ef: 0f8622040000 jbe 0x1412ac817
1412ac3f5: 498b4608 mov rax, qword ptr [r14 + 8]
1412ac3f9: 488d0dc003fcff lea rcx, [rip - 0x3fc40]
1412ac400: 4a890c38 mov qword ptr [rax + r15], rcx
1412ac404: 488d0d655cfdff lea rcx, [rip - 0x2a39b]
1412ac40b: 4a894c3808 mov qword ptr [rax + r15 + 8], rcx
1412ac410: 4a895c3810 mov qword ptr [rax + r15 + 0x10], rbx
1412ac415: 42896c3818 mov dword ptr [rax + r15 + 0x18], ebp
1412ac41a: 42c644381c00 mov byte ptr [rax + r15 + 0x1c], 0
1412ac420: 4983c720 add r15, 0x20
1412ac424: 4d897e10 mov qword ptr [r14 + 0x10], r15
1412ac428: 80bf3c01000002 cmp byte ptr [rdi + 0x13c], 2
1412ac42f: 7527 jne 0x1412ac458
1412ac431: 83bf1801000001 cmp dword ptr [rdi + 0x118], 1
1412ac438: 0f8494000000 je 0x1412ac4d2
1412ac43e: 488b8c2480010000 mov rcx, qword ptr [rsp + 0x180]
1412ac446: 83bf9000000001 cmp dword ptr [rdi + 0x90], 1
1412ac44d: 0f84a2010000 je 0x1412ac5f5
1412ac453: e90a020000 jmp 0x1412ac662 ; unknown
1412ac458: 4c8b742448 mov r14, qword ptr [rsp + 0x48]
1412ac45d: 4d85f6 test r14, r14
1412ac460: 7505 jne 0x1412ac467
1412ac462: 4c8b742450 mov r14, qword ptr [rsp + 0x50]
1412ac467: 4c8da72c010000 lea r12, [rdi + 0x12c]
1412ac46e: 498b06 mov rax, qword ptr [r14]
1412ac471: 4d8b7e10 mov r15, qword ptr [r14 + 0x10]
1412ac475: 4c29f8 sub rax, r15
1412ac478: 4883f82f cmp rax, 0x2f
1412ac47c: 0f86ba030000 jbe 0x1412ac83c
1412ac482: 498b4608 mov rax, qword ptr [r14 + 8]
1412ac486: 488d0db305fcff lea rcx, [rip - 0x3fa4d]
1412ac48d: 4a890c38 mov qword ptr [rax + r15], rcx
1412ac491: 488d0dd85bfdff lea rcx, [rip - 0x2a428]
1412ac498: 4a894c3808 mov qword ptr [rax + r15 + 8], rcx
1412ac49d: 4a895c3810 mov qword ptr [rax + r15 + 0x10], rbx
1412ac4a2: 410f100424 movups xmm0, xmmword ptr [r12]
1412ac4a7: 420f11443818 movups xmmword ptr [rax + r15 + 0x18], xmm0
1412ac4ad: 418b4c2410 mov ecx, dword ptr [r12 + 0x10]
1412ac4b2: 42894c3828 mov dword ptr [rax + r15 + 0x28], ecx
1412ac4b7: 42c644382c00 mov byte ptr [rax + r15 + 0x2c], 0
1412ac4bd: 4983c730 add r15, 0x30
1412ac4c1: 4d897e10 mov qword ptr [r14 + 0x10], r15
1412ac4c5: 83bf1801000001 cmp dword ptr [rdi + 0x118], 1
1412ac4cc: 0f856cffffff jne 0x1412ac43e
1412ac4d2: 440f108f1c010000 movups xmm9, xmmword ptr [rdi + 0x11c]
1412ac4da: 410f28c1 movaps xmm0, xmm9
1412ac4de: 410fc6c1a4 shufps xmm0, xmm9, 0xa4
1412ac4e3: 410f28c8 movaps xmm1, xmm8
1412ac4e7: 410f59c8 mulps xmm1, xmm8
1412ac4eb: f30f16d1 movshdup xmm2, xmm1
1412ac4ef: f30f58d1 addss xmm2, xmm1
1412ac4f3: 0f28d9 movaps xmm3, xmm1
1412ac4f6: 660f15d9 unpckhpd xmm3, xmm1
1412ac4fa: f30f58da addss xmm3, xmm2
1412ac4fe: 0fc6db00 shufps xmm3, xmm3, 0
1412ac502: 0fc6c9ff shufps xmm1, xmm1, 0xff
1412ac506: 0f5ccb subps xmm1, xmm3
1412ac509: 0f59c8 mulps xmm1, xmm0
1412ac50c: 410f28c0 movaps xmm0, xmm8
1412ac510: 410f59c1 mulps xmm0, xmm9
1412ac514: f30f16d0 movshdup xmm2, xmm0
1412ac518: f30f58d0 addss xmm2, xmm0
1412ac51c: 0f12c0 movhlps xmm0, xmm0
1412ac51f: f30f58c2 addss xmm0, xmm2
1412ac523: f30f58c0 addss xmm0, xmm0
1412ac527: 0fc6c000 shufps xmm0, xmm0, 0
1412ac52b: 410f59c0 mulps xmm0, xmm8
1412ac52f: 0f58c1 addps xmm0, xmm1
1412ac532: 410f28c8 movaps xmm1, xmm8
1412ac536: 410fc6c8d2 shufps xmm1, xmm8, 0xd2
1412ac53b: 410f28d1 movaps xmm2, xmm9
1412ac53f: 410fc6d1d2 shufps xmm2, xmm9, 0xd2
1412ac544: 410f59c9 mulps xmm1, xmm9
1412ac548: 410f59d0 mulps xmm2, xmm8
1412ac54c: 0f5cca subps xmm1, xmm2
1412ac54f: 0fc6c9d2 shufps xmm1, xmm1, 0xd2
1412ac553: 450f58c0 addps xmm8, xmm8
1412ac557: 450fc6c0ff shufps xmm8, xmm8, 0xff
1412ac55c: 440f59c1 mulps xmm8, xmm1
1412ac560: 440f58c0 addps xmm8, xmm0
1412ac564: 410f28c0 movaps xmm0, xmm8
1412ac568: 66410f15c0 unpckhpd xmm0, xmm8
1412ac56d: 4c8b742448 mov r14, qword ptr [rsp + 0x48]
1412ac572: 4d85f6 test r14, r14
1412ac575: 7505 jne 0x1412ac57c
1412ac577: 4c8b742450 mov r14, qword ptr [rsp + 0x50]
1412ac57c: 410f58f8 addps xmm7, xmm8
1412ac580: f30f58f0 addss xmm6, xmm0
1412ac584: 498b06 mov rax, qword ptr [r14]
1412ac587: 4d8b7e10 mov r15, qword ptr [r14 + 0x10]
1412ac58b: 4c29f8 sub rax, r15
1412ac58e: 4883f83f cmp rax, 0x3f
1412ac592: 0f86c9020000 jbe 0x1412ac861
1412ac598: 498b4608 mov rax, qword ptr [r14 + 8]
1412ac59c: 488d0d4d08fcff lea rcx, [rip - 0x3f7b3]
1412ac5a3: 4a890c38 mov qword ptr [rax + r15], rcx
1412ac5a7: 488d0dc25afdff lea rcx, [rip - 0x2a53e]
1412ac5ae: 4a894c3808 mov qword ptr [rax + r15 + 8], rcx
1412ac5b3: 4a895c3810 mov qword ptr [rax + r15 + 0x10], rbx
1412ac5b8: 420f137c3818 movlps qword ptr [rax + r15 + 0x18], xmm7
1412ac5be: f3420f11743820 movss dword ptr [rax + r15 + 0x20], xmm6
1412ac5c5: 460f114c3824 movups xmmword ptr [rax + r15 + 0x24], xmm9
1412ac5cb: 4289743834 mov dword ptr [rax + r15 + 0x34], esi
1412ac5d0: 42c644383800 mov byte ptr [rax + r15 + 0x38], 0
1412ac5d6: 42c644383c00 mov byte ptr [rax + r15 + 0x3c], 0
1412ac5dc: 4983c740 add r15, 0x40
1412ac5e0: 4d897e10 mov qword ptr [r14 + 0x10], r15
1412ac5e4: 488b8c2480010000 mov rcx, qword ptr [rsp + 0x180]
1412ac5ec: 83bf9000000001 cmp dword ptr [rdi + 0x90], 1
1412ac5f3: 756d jne 0x1412ac662
1412ac5f5: 4c8b742448 mov r14, qword ptr [rsp + 0x48]
1412ac5fa: 4d85f6 test r14, r14
1412ac5fd: 7505 jne 0x1412ac604
1412ac5ff: 4c8b742450 mov r14, qword ptr [rsp + 0x50]
1412ac604: 4881c794000000 add rdi, 0x94
1412ac60b: 498b06 mov rax, qword ptr [r14]
1412ac60e: 4d8b7e10 mov r15, qword ptr [r14 + 0x10]
1412ac612: 4c29f8 sub rax, r15
1412ac615: 4883f82f cmp rax, 0x2f
1412ac619: 0f8667020000 jbe 0x1412ac886
1412ac61f: 498b4608 mov rax, qword ptr [r14 + 8]
1412ac623: 488d15960efcff lea rdx, [rip - 0x3f16a]
1412ac62a: 4a891438 mov qword ptr [rax + r15], rdx
1412ac62e: 488d153b5afdff lea rdx, [rip - 0x2a5c5]
1412ac635: 4a89543808 mov qword ptr [rax + r15 + 8], rdx
1412ac63a: 4a895c3810 mov qword ptr [rax + r15 + 0x10], rbx
1412ac63f: 4289743818 mov dword ptr [rax + r15 + 0x18], esi
1412ac644: 488b17 mov rdx, qword ptr [rdi]
1412ac647: 4a8954381c mov qword ptr [rax + r15 + 0x1c], rdx
1412ac64c: 8b5708 mov edx, dword ptr [rdi + 8]
1412ac64f: 4289543824 mov dword ptr [rax + r15 + 0x24], edx
1412ac654: 42c644382800 mov byte ptr [rax + r15 + 0x28], 0
1412ac65a: 4983c730 add r15, 0x30
1412ac65e: 4d897e10 mov qword ptr [r14 + 0x10], r15
1412ac662: 89f2 mov edx, esi
1412ac664: 4989d8 mov r8, rbx
1412ac667: e8c45556ff call 0x140811c30 ; system_decorator::decorator_visual_state::impl$5::add_decorator_visual_entity
1412ac66c: 4889d8 mov rax, rbx
1412ac66f: 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
1412ac677: 0f28bc24d0000000 movaps xmm7, xmmword ptr [rsp + 0xd0]
1412ac67f: 440f288424e0000000 movaps xmm8, xmmword ptr [rsp + 0xe0]
1412ac688: 440f288c24f0000000 movaps xmm9, xmmword ptr [rsp + 0xf0]
1412ac691: 4881c408010000 add rsp, 0x108
1412ac698: 5b pop rbx
1412ac699: 5d pop rbp
1412ac69a: 5f pop rdi
1412ac69b: 5e pop rsi
1412ac69c: 415c pop r12
1412ac69e: 415d pop r13
1412ac6a0: 415e pop r14
1412ac6a2: 415f pop r15
1412ac6a4: c3 ret 
1412ac6a5: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1412ac6ae: 41b828000000 mov r8d, 0x28
1412ac6b4: 41b901000000 mov r9d, 1
1412ac6ba: 4c89f1 mov rcx, r14
1412ac6bd: 4c89fa mov rdx, r15
1412ac6c0: e85bb85f01 call 0x1428a7f20 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1412ac6c5: e9f0f7ffff jmp 0x1412abeba ; unknown
1412ac6ca: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1412ac6d3: 41b820000000 mov r8d, 0x20
1412ac6d9: 41b901000000 mov r9d, 1
1412ac6df: 4889d9 mov rcx, rbx
1412ac6e2: 4c89f2 mov rdx, r14
1412ac6e5: e836b85f01 call 0x1428a7f20 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1412ac6ea: e93ef8ffff jmp 0x1412abf2d ; unknown
1412ac6ef: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1412ac6f8: 41b858000000 mov r8d, 0x58
1412ac6fe: 41b901000000 mov r9d, 1
1412ac704: 4889d9 mov rcx, rbx
1412ac707: 4c89f2 mov rdx, r14
1412ac70a: e811b85f01 call 0x1428a7f20 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1412ac70f: e982f8ffff jmp 0x1412abf96 ; unknown
1412ac714: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1412ac71d: 41b838000000 mov r8d, 0x38
1412ac723: 41b901000000 mov r9d, 1
1412ac729: 4889d9 mov rcx, rbx
1412ac72c: 4c89f2 mov rdx, r14
1412ac72f: e8ecb75f01 call 0x1428a7f20 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1412ac734: e9f2f8ffff jmp 0x1412ac02b ; unknown
1412ac739: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1412ac742: 41b828000000 mov r8d, 0x28
1412ac748: 41b901000000 mov r9d, 1
1412ac74e: 4889d9 mov rcx, rbx
1412ac751: 4c89f2 mov rdx, r14
1412ac754: e8c7b75f01 call 0x1428a7f20 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1412ac759: e94af9ffff jmp 0x1412ac0a8 ; unknown
1412ac75e: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1412ac767: 41b820000000 mov r8d, 0x20
1412ac76d: 41b901000000 mov r9d, 1
1412ac773: 4889d9 mov rcx, rbx
1412ac776: 4c89f2 mov rdx, r14
1412ac779: e8a2b75f01 call 0x1428a7f20 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1412ac77e: e999f9ffff jmp 0x1412ac11c ; unknown
1412ac783: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1412ac78c: 41b830000000 mov r8d, 0x30
1412ac792: 41b901000000 mov r9d, 1
1412ac798: 4889d9 mov rcx, rbx
1412ac79b: 4c89f2 mov rdx, r14
1412ac79e: e87db75f01 call 0x1428a7f20 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1412ac7a3: e9d8f9ffff jmp 0x1412ac180 ; unknown
1412ac7a8: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1412ac7b1: 41b838000000 mov r8d, 0x38
1412ac7b7: 41b901000000 mov r9d, 1
1412ac7bd: 4889d9 mov rcx, rbx
1412ac7c0: 4c89f2 mov rdx, r14
1412ac7c3: e858b75f01 call 0x1428a7f20 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1412ac7c8: e9a9faffff jmp 0x1412ac276 ; unknown
1412ac7cd: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1412ac7d6: 41b830000000 mov r8d, 0x30
1412ac7dc: 41b901000000 mov r9d, 1
1412ac7e2: 4889d9 mov rcx, rbx
1412ac7e5: 4c89f2 mov rdx, r14
1412ac7e8: e833b75f01 call 0x1428a7f20 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1412ac7ed: e914fbffff jmp 0x1412ac306 ; unknown
1412ac7f2: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1412ac7fb: 41b828000000 mov r8d, 0x28
1412ac801: 41b901000000 mov r9d, 1
1412ac807: 4889d9 mov rcx, rbx
1412ac80a: 4c89f2 mov rdx, r14
1412ac80d: e80eb75f01 call 0x1428a7f20 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1412ac812: e96bfbffff jmp 0x1412ac382 ; unknown
1412ac817: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1412ac820: 41b820000000 mov r8d, 0x20
1412ac826: 41b901000000 mov r9d, 1
1412ac82c: 4c89f1 mov rcx, r14
1412ac82f: 4c89fa mov rdx, r15
1412ac832: e8e9b65f01 call 0x1428a7f20 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1412ac837: e9b9fbffff jmp 0x1412ac3f5 ; unknown
1412ac83c: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1412ac845: 41b830000000 mov r8d, 0x30
1412ac84b: 41b901000000 mov r9d, 1
1412ac851: 4c89f1 mov rcx, r14
1412ac854: 4c89fa mov rdx, r15
1412ac857: e8c4b65f01 call 0x1428a7f20 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1412ac85c: e921fcffff jmp 0x1412ac482 ; unknown
1412ac861: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1412ac86a: 41b840000000 mov r8d, 0x40
1412ac870: 41b901000000 mov r9d, 1
1412ac876: 4c89f1 mov rcx, r14
1412ac879: 4c89fa mov rdx, r15
1412ac87c: e89fb65f01 call 0x1428a7f20 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1412ac881: e912fdffff jmp 0x1412ac598 ; unknown
1412ac886: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1412ac88f: 41b830000000 mov r8d, 0x30
1412ac895: 41b901000000 mov r9d, 1
1412ac89b: 4989cc mov r12, rcx
1412ac89e: 4c89f1 mov rcx, r14
1412ac8a1: 4c89fa mov rdx, r15
1412ac8a4: e877b65f01 call 0x1428a7f20 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1412ac8a9: 4c89e1 mov rcx, r12
1412ac8ac: e96efdffff jmp 0x1412ac61f ; unknown
1412ac8b1: 488d0db8e69701 lea rcx, [rip + 0x197e6b8]
1412ac8b8: 4c8d05e1e69701 lea r8, [rip + 0x197e6e1]
1412ac8bf: ba2b000000 mov edx, 0x2b
1412ac8c4: e867cb6201 call 0x1428d9430 ; core::panicking::panic
1412ac8c9: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1412ac8d2: 41b888000000 mov r8d, 0x88
1412ac8d8: 41b901000000 mov r9d, 1
1412ac8de: 4889d9 mov rcx, rbx
1412ac8e1: 4c89fa mov rdx, r15
1412ac8e4: e837b65f01 call 0x1428a7f20 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1412ac8e9: e9ebf0ffff jmp 0x1412ab9d9 ; unknown
1412ac8ee: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1412ac8f7: 41b888000000 mov r8d, 0x88
1412ac8fd: 41b901000000 mov r9d, 1
1412ac903: 4889d9 mov rcx, rbx
1412ac906: 4c89fa mov rdx, r15
1412ac909: e812b65f01 call 0x1428a7f20 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1412ac90e: e994f1ffff jmp 0x1412abaa7 ; unknown
1412ac913: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1412ac91c: 41b898000000 mov r8d, 0x98
1412ac922: 41b901000000 mov r9d, 1
1412ac928: 4889d9 mov rcx, rbx
1412ac92b: 4c89fa mov rdx, r15
1412ac92e: e8edb55f01 call 0x1428a7f20 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1412ac933: e92eeeffff jmp 0x1412ab766 ; unknown
1412ac938: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1412ac941: 41b898000000 mov r8d, 0x98
1412ac947: 41b901000000 mov r9d, 1
1412ac94d: 4889d9 mov rcx, rbx
1412ac950: 4c89fa mov rdx, r15
1412ac953: e8c8b55f01 call 0x1428a7f20 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1412ac958: e91cf4ffff jmp 0x1412abd79 ; unknown
1412ac95d: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1412ac966: 41b888000000 mov r8d, 0x88
1412ac96c: 41b901000000 mov r9d, 1
1412ac972: 4889d9 mov rcx, rbx
1412ac975: 4c89fa mov rdx, r15
1412ac978: e8a3b55f01 call 0x1428a7f20 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1412ac97d: e91cf3ffff jmp 0x1412abc9e ; unknown
1412ac982: 4c8d050fe59701 lea r8, [rip + 0x197e50f]
1412ac989: 4c89f9 mov rcx, r15
1412ac98c: 4c89e2 mov rdx, r12
1412ac98f: e89cc96201 call 0x1428d9330 ; core::option::expect_failed
1412ac994: 488d0d1de69701 lea rcx, [rip + 0x197e61d]
1412ac99b: e870c96201 call 0x1428d9310 ; core::option::unwrap_failed
1412ac9a0: 488d0d29e69701 lea rcx, [rip + 0x197e629]
1412ac9a7: e864c96201 call 0x1428d9310 ; core::option::unwrap_failed
1412ac9ac: cc int3 
