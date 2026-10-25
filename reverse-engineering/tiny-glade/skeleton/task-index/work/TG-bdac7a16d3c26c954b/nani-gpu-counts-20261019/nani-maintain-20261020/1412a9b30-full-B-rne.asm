1412a9b30: 4157 push r15
1412a9b32: 4156 push r14
1412a9b34: 4155 push r13
1412a9b36: 4154 push r12
1412a9b38: 56 push rsi
1412a9b39: 57 push rdi
1412a9b3a: 55 push rbp
1412a9b3b: 53 push rbx
1412a9b3c: 4881ec08010000 sub rsp, 0x108
1412a9b43: 440f298c24f0000000 movaps xmmword ptr [rsp + 0xf0], xmm9
1412a9b4c: 440f298424e0000000 movaps xmmword ptr [rsp + 0xe0], xmm8
1412a9b55: 0f29bc24d0000000 movaps xmmword ptr [rsp + 0xd0], xmm7
1412a9b5d: 0f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm6
1412a9b65: 4d89ce mov r14, r9
1412a9b68: 89d6 mov esi, edx
1412a9b6a: 4889cf mov rdi, rcx
1412a9b6d: 440f2881c0000000 movaps xmm8, xmmword ptr [rcx + 0xc0]
1412a9b75: f20f10b9d0000000 movsd xmm7, qword ptr [rcx + 0xd0]
1412a9b7d: f30f10b1d8000000 movss xmm6, dword ptr [rcx + 0xd8]
1412a9b85: 8b81e4000000 mov eax, dword ptr [rcx + 0xe4]
1412a9b8b: 89442438 mov dword ptr [rsp + 0x38], eax
1412a9b8f: 488b81dc000000 mov rax, qword ptr [rcx + 0xdc]
1412a9b96: 4889442430 mov qword ptr [rsp + 0x30], rax
1412a9b9b: f30f108940010000 movss xmm1, dword ptr [rcx + 0x140]
1412a9ba3: f20f109144010000 movsd xmm2, qword ptr [rcx + 0x144]
1412a9bab: 0f28c1 movaps xmm0, xmm1
1412a9bae: f30f59c1 mulss xmm0, xmm1
1412a9bb2: 0f28da movaps xmm3, xmm2
1412a9bb5: 0f59da mulps xmm3, xmm2
1412a9bb8: f30f58c3 addss xmm0, xmm3
1412a9bbc: f30f16db movshdup xmm3, xmm3
1412a9bc0: f30f58d8 addss xmm3, xmm0
1412a9bc4: 0f57c0 xorps xmm0, xmm0
1412a9bc7: f30f51c3 sqrtss xmm0, xmm3
1412a9bcb: 0f57db xorps xmm3, xmm3
1412a9bce: 0f2ec3 ucomiss xmm0, xmm3
1412a9bd1: 0f8693000000 jbe 0x1412a9c6a
1412a9bd7: 0f16ca movlhps xmm1, xmm2
1412a9bda: 0f28c1 movaps xmm0, xmm1
1412a9bdd: 0fc6c2d8 shufps xmm0, xmm2, 0xd8
1412a9be1: 0fc6ca58 shufps xmm1, xmm2, 0x58
1412a9be5: 410f28d0 movaps xmm2, xmm8
1412a9be9: 410f59d0 mulps xmm2, xmm8
1412a9bed: f30f16da movshdup xmm3, xmm2
1412a9bf1: f30f58da addss xmm3, xmm2
1412a9bf5: 0f28e2 movaps xmm4, xmm2
1412a9bf8: 660f15e2 unpckhpd xmm4, xmm2
1412a9bfc: f30f58e3 addss xmm4, xmm3
1412a9c00: 0fc6e400 shufps xmm4, xmm4, 0
1412a9c04: 0fc6d2ff shufps xmm2, xmm2, 0xff
1412a9c08: 0f5cd4 subps xmm2, xmm4
1412a9c0b: 0f59d1 mulps xmm2, xmm1
1412a9c0e: 410f28c8 movaps xmm1, xmm8
1412a9c12: 0f59c8 mulps xmm1, xmm0
1412a9c15: f30f16d9 movshdup xmm3, xmm1
1412a9c19: f30f58d9 addss xmm3, xmm1
1412a9c1d: 0f12c9 movhlps xmm1, xmm1
1412a9c20: f30f58cb addss xmm1, xmm3
1412a9c24: f30f58c9 addss xmm1, xmm1
1412a9c28: 0fc6c900 shufps xmm1, xmm1, 0
1412a9c2c: 410f59c8 mulps xmm1, xmm8
1412a9c30: 0f58ca addps xmm1, xmm2
1412a9c33: 410f28d0 movaps xmm2, xmm8
1412a9c37: 410fc6d0d2 shufps xmm2, xmm8, 0xd2
1412a9c3c: 0f59d0 mulps xmm2, xmm0
1412a9c3f: 0fc6c0d2 shufps xmm0, xmm0, 0xd2
1412a9c43: 410f59c0 mulps xmm0, xmm8
1412a9c47: 0f5cd0 subps xmm2, xmm0
1412a9c4a: 0fc6d2d2 shufps xmm2, xmm2, 0xd2
1412a9c4e: 410f28c0 movaps xmm0, xmm8
1412a9c52: 410f58c0 addps xmm0, xmm8
1412a9c56: 0fc6c0ff shufps xmm0, xmm0, 0xff
1412a9c5a: 0f59c2 mulps xmm0, xmm2
1412a9c5d: 0f58c1 addps xmm0, xmm1
1412a9c60: 0f58f8 addps xmm7, xmm0
1412a9c63: 0f12c0 movhlps xmm0, xmm0
1412a9c66: f30f58f0 addss xmm6, xmm0
1412a9c6a: 837f3001 cmp dword ptr [rdi + 0x30], 1
1412a9c6e: 7510 jne 0x1412a9c80
1412a9c70: 8b473c mov eax, dword ptr [rdi + 0x3c]
1412a9c73: 89442438 mov dword ptr [rsp + 0x38], eax
1412a9c77: 488b4734 mov rax, qword ptr [rdi + 0x34]
1412a9c7b: 4889442430 mov qword ptr [rsp + 0x30], rax
1412a9c80: 488b9c2478010000 mov rbx, qword ptr [rsp + 0x178]
1412a9c88: 488bafe8000000 mov rbp, qword ptr [rdi + 0xe8]
1412a9c8f: 4c8bbfb0000000 mov r15, qword ptr [rdi + 0xb0]
1412a9c96: 4c8ba7b8000000 mov r12, qword ptr [rdi + 0xb8]
1412a9c9d: 498d4424f7 lea rax, [r12 - 9]
1412a9ca2: 4883f81b cmp rax, 0x1b
1412a9ca6: 0f8731070000 ja 0x1412aa3dd
1412a9cac: 488d0d35139801 lea rcx, [rip + 0x1981335]
1412a9cb3: 48630481 movsxd rax, dword ptr [rcx + rax*4]
1412a9cb7: 4801c8 add rax, rcx
1412a9cba: ffe0 jmp rax
1412a9cbc: 48b86e616e695f756265 movabs rax, 0x6562755f696e616e
1412a9cc6: 493307 xor rax, qword ptr [r15]
1412a9cc9: 410fb64f08 movzx ecx, byte ptr [r15 + 8]
1412a9cce: 4883f172 xor rcx, 0x72
1412a9cd2: 4809c1 or rcx, rax
1412a9cd5: 0f8502070000 jne 0x1412aa3dd
1412a9cdb: 488b8fa0000000 mov rcx, qword ptr [rdi + 0xa0]
1412a9ce2: 488b97a8000000 mov rdx, qword ptr [rdi + 0xa8]
1412a9ce9: e892309fff call 0x140c9cd80 ; utils::name_hash::NameHash::compute
1412a9cee: 48898424a8000000 mov qword ptr [rsp + 0xa8], rax
1412a9cf6: c78424a000000000000000 mov dword ptr [rsp + 0xa0], 0
1412a9d01: 488b442430 mov rax, qword ptr [rsp + 0x30]
1412a9d06: 488984248c000000 mov qword ptr [rsp + 0x8c], rax
1412a9d0e: 8b442438 mov eax, dword ptr [rsp + 0x38]
1412a9d12: 89842494000000 mov dword ptr [rsp + 0x94], eax
1412a9d19: e8621ed9fe call 0x14003bb80 ; bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>
1412a9d1e: c68424b000000017 mov byte ptr [rsp + 0xb0], 0x17
1412a9d26: 440f29442470 movaps xmmword ptr [rsp + 0x70], xmm8
1412a9d2c: 0f13bc2480000000 movlps qword ptr [rsp + 0x80], xmm7
1412a9d34: f30f11b42488000000 movss dword ptr [rsp + 0x88], xmm6
1412a9d3d: 4889ac2498000000 mov qword ptr [rsp + 0x98], rbp
1412a9d45: 4c8d0d7c119801 lea r9, [rip + 0x198117c]
1412a9d4c: 488d4c2440 lea rcx, [rsp + 0x40]
1412a9d51: 4c8d442470 lea r8, [rsp + 0x70]
1412a9d56: 4889da mov rdx, rbx
1412a9d59: e88242fcff call 0x14126dfe0 ; bevy_ecs::system::commands::Commands::spawn<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::UberInstanceData> >
1412a9d5e: e972090000 jmp 0x1412aa6d5 ; unknown
1412a9d63: f3410f6f07 movdqu xmm0, xmmword ptr [r15]
1412a9d68: 410fb64710 movzx eax, byte ptr [r15 + 0x10]
1412a9d6d: 660f6ec8 movd xmm1, eax
1412a9d71: 660f740537109801 pcmpeqb xmm0, xmmword ptr [rip + 0x1981037]
1412a9d79: 660f740d7f236801 pcmpeqb xmm1, xmmword ptr [rip + 0x168237f]
1412a9d81: 660fdbc8 pand xmm1, xmm0
1412a9d85: 660fd7c1 pmovmskb eax, xmm1
1412a9d89: 3dffff0000 cmp eax, 0xffff
1412a9d8e: 0f8549060000 jne 0x1412aa3dd
1412a9d94: 488b8fa0000000 mov rcx, qword ptr [rdi + 0xa0]
1412a9d9b: 488b97a8000000 mov rdx, qword ptr [rdi + 0xa8]
1412a9da2: e8d92f9fff call 0x140c9cd80 ; utils::name_hash::NameHash::compute
1412a9da7: 48898424a8000000 mov qword ptr [rsp + 0xa8], rax
1412a9daf: c78424a000000000000000 mov dword ptr [rsp + 0xa0], 0
1412a9dba: 488b442430 mov rax, qword ptr [rsp + 0x30]
1412a9dbf: 488984248c000000 mov qword ptr [rsp + 0x8c], rax
1412a9dc7: 8b442438 mov eax, dword ptr [rsp + 0x38]
1412a9dcb: 89842494000000 mov dword ptr [rsp + 0x94], eax
1412a9dd2: e8a91dd9fe call 0x14003bb80 ; bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>
1412a9dd7: c68424b000000016 mov byte ptr [rsp + 0xb0], 0x16
1412a9ddf: 440f29442470 movaps xmmword ptr [rsp + 0x70], xmm8
1412a9de5: 0f13bc2480000000 movlps qword ptr [rsp + 0x80], xmm7
1412a9ded: f30f11b42488000000 movss dword ptr [rsp + 0x88], xmm6
1412a9df6: 4889ac2498000000 mov qword ptr [rsp + 0x98], rbp
1412a9dfe: 4c8d0ddb109801 lea r9, [rip + 0x19810db]
1412a9e05: 488d4c2440 lea rcx, [rsp + 0x40]
1412a9e0a: 4c8d442470 lea r8, [rsp + 0x70]
1412a9e0f: 4889da mov rdx, rbx
1412a9e12: e8c940fcff call 0x14126dee0 ; bevy_ecs::system::commands::Commands::spawn<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::BalconyDoorInstanceData> >
1412a9e17: e9b9080000 jmp 0x1412aa6d5 ; unknown
1412a9e1c: f3410f6f07 movdqu xmm0, xmmword ptr [r15]
1412a9e21: f3410f6f4f09 movdqu xmm1, xmmword ptr [r15 + 9]
1412a9e27: 660f740db1438101 pcmpeqb xmm1, xmmword ptr [rip + 0x18143b1]
1412a9e2f: 660f7405b9438101 pcmpeqb xmm0, xmmword ptr [rip + 0x18143b9]
1412a9e37: 660fdbc1 pand xmm0, xmm1
1412a9e3b: 660fd7c0 pmovmskb eax, xmm0
1412a9e3f: 3dffff0000 cmp eax, 0xffff
1412a9e44: 0f8593050000 jne 0x1412aa3dd
1412a9e4a: 488b8fa0000000 mov rcx, qword ptr [rdi + 0xa0]
1412a9e51: 488b97a8000000 mov rdx, qword ptr [rdi + 0xa8]
1412a9e58: e8232f9fff call 0x140c9cd80 ; utils::name_hash::NameHash::compute
1412a9e5d: 48898424a8000000 mov qword ptr [rsp + 0xa8], rax
1412a9e65: c78424a000000000000000 mov dword ptr [rsp + 0xa0], 0
1412a9e70: 488b442430 mov rax, qword ptr [rsp + 0x30]
1412a9e75: 488984248c000000 mov qword ptr [rsp + 0x8c], rax
1412a9e7d: 8b442438 mov eax, dword ptr [rsp + 0x38]
1412a9e81: 89842494000000 mov dword ptr [rsp + 0x94], eax
1412a9e88: e8f31cd9fe call 0x14003bb80 ; bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>
1412a9e8d: c68424b000000007 mov byte ptr [rsp + 0xb0], 7
1412a9e95: 440f29442470 movaps xmmword ptr [rsp + 0x70], xmm8
1412a9e9b: 0f13bc2480000000 movlps qword ptr [rsp + 0x80], xmm7
1412a9ea3: f30f11b42488000000 movss dword ptr [rsp + 0x88], xmm6
1412a9eac: 4889ac2498000000 mov qword ptr [rsp + 0x98], rbp
1412a9eb4: 4c8d0d85109801 lea r9, [rip + 0x1981085]
1412a9ebb: 488d4c2440 lea rcx, [rsp + 0x40]
1412a9ec0: 4c8d442470 lea r8, [rsp + 0x70]
1412a9ec5: 4889da mov rdx, rbx
1412a9ec8: e82348fcff call 0x14126e6f0 ; bevy_ecs::system::commands::Commands::spawn<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::GothicWindowBricksInstanceData> >
1412a9ecd: e903080000 jmp 0x1412aa6d5 ; unknown
1412a9ed2: 48b86e616e695f636c6f movabs rax, 0x6f6c635f696e616e
1412a9edc: 493307 xor rax, qword ptr [r15]
1412a9edf: 410fb74f08 movzx ecx, word ptr [r15 + 8]
1412a9ee4: 4881f174680000 xor rcx, 0x6874
1412a9eeb: 4809c1 or rcx, rax
1412a9eee: 0f8480050000 je 0x1412aa474
1412a9ef4: 48b86e616e695f736d6f movabs rax, 0x6f6d735f696e616e
1412a9efe: 493307 xor rax, qword ptr [r15]
1412a9f01: 410fb74f08 movzx ecx, word ptr [r15 + 8]
1412a9f06: 4881f16b650000 xor rcx, 0x656b
1412a9f0d: 4809c1 or rcx, rax
1412a9f10: 0f85c7040000 jne 0x1412aa3dd
1412a9f16: 31c9 xor ecx, ecx
1412a9f18: e8339358ff call 0x140833250 ; country_core::resources::walls::decorator::DecoratorRank::new
1412a9f1d: 488d8c24a0000000 lea rcx, [rsp + 0xa0]
1412a9f25: 89c2 mov edx, eax
1412a9f27: e8341e58ff call 0x14082bd60 ; country_core::components::drawable::impl$1::from
1412a9f2c: 8b442438 mov eax, dword ptr [rsp + 0x38]
1412a9f30: 89842494000000 mov dword ptr [rsp + 0x94], eax
1412a9f37: 488b442430 mov rax, qword ptr [rsp + 0x30]
1412a9f3c: 488984248c000000 mov qword ptr [rsp + 0x8c], rax
1412a9f44: e8371cd9fe call 0x14003bb80 ; bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>
1412a9f49: c68424b000000015 mov byte ptr [rsp + 0xb0], 0x15
1412a9f51: 440f29442470 movaps xmmword ptr [rsp + 0x70], xmm8
1412a9f57: 0f13bc2480000000 movlps qword ptr [rsp + 0x80], xmm7
1412a9f5f: f30f11b42488000000 movss dword ptr [rsp + 0x88], xmm6
1412a9f68: 4889ac2498000000 mov qword ptr [rsp + 0x98], rbp
1412a9f70: 4c8d0d810f9801 lea r9, [rip + 0x1980f81]
1412a9f77: 488d4c2440 lea rcx, [rsp + 0x40]
1412a9f7c: 4c8d442470 lea r8, [rsp + 0x70]
1412a9f81: 4889da mov rdx, rbx
1412a9f84: e85741fcff call 0x14126e0e0 ; bevy_ecs::system::commands::Commands::spawn<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::SmokeInstanceData> >
1412a9f89: e947070000 jmp 0x1412aa6d5 ; unknown
1412a9f8e: f3410f6f07 movdqu xmm0, xmmword ptr [r15]
1412a9f93: f3410f6f4f0b movdqu xmm1, xmmword ptr [r15 + 0xb]
1412a9f99: 660f740d4f0e9801 pcmpeqb xmm1, xmmword ptr [rip + 0x1980e4f]
1412a9fa1: 660f7405570e9801 pcmpeqb xmm0, xmmword ptr [rip + 0x1980e57]
1412a9fa9: 660fdbc1 pand xmm0, xmm1
1412a9fad: 660fd7c0 pmovmskb eax, xmm0
1412a9fb1: 3dffff0000 cmp eax, 0xffff
1412a9fb6: 0f8521040000 jne 0x1412aa3dd
1412a9fbc: 488b8fa0000000 mov rcx, qword ptr [rdi + 0xa0]
1412a9fc3: 488b97a8000000 mov rdx, qword ptr [rdi + 0xa8]
1412a9fca: e8b12d9fff call 0x140c9cd80 ; utils::name_hash::NameHash::compute
1412a9fcf: 4989c6 mov r14, rax
1412a9fd2: e8a91bd9fe call 0x14003bb80 ; bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>
1412a9fd7: 440fb6a750010000 movzx r12d, byte ptr [rdi + 0x150]
1412a9fdf: 4180fc09 cmp r12b, 9
1412a9fe3: 0f846b120000 je 0x1412ab254
1412a9fe9: 488d4c2470 lea rcx, [rsp + 0x70]
1412a9fee: 4889da mov rdx, rbx
1412a9ff1: e8aaa6a6ff call 0x140d146a0 ; bevy_ecs::system::commands::Commands::spawn_empty
1412a9ff6: 488b5c2478 mov rbx, qword ptr [rsp + 0x78]
1412a9ffb: 4885db test rbx, rbx
1412a9ffe: 7508 jne 0x1412aa008
1412aa000: 488b9c2480000000 mov rbx, qword ptr [rsp + 0x80]
1412aa008: 4c8bac2490000000 mov r13, qword ptr [rsp + 0x90]
1412aa010: 488b03 mov rax, qword ptr [rbx]
1412aa013: 4c8b7b10 mov r15, qword ptr [rbx + 0x10]
1412aa017: 4c29f8 sub rax, r15
1412aa01a: 483d97000000 cmp rax, 0x97
1412aa020: 0f86ad110000 jbe 0x1412ab1d3
1412aa026: 488b4308 mov rax, qword ptr [rbx + 8]
1412aa02a: 488d0d6f16fcff lea rcx, [rip - 0x3e991]
1412aa031: 4a890c38 mov qword ptr [rax + r15], rcx
1412aa035: 460f11443808 movups xmmword ptr [rax + r15 + 8], xmm8
1412aa03b: 420f137c3818 movlps qword ptr [rax + r15 + 0x18], xmm7
1412aa041: f3420f11743820 movss dword ptr [rax + r15 + 0x20], xmm6
1412aa048: 488b4c2430 mov rcx, qword ptr [rsp + 0x30]
1412aa04d: 4a894c3824 mov qword ptr [rax + r15 + 0x24], rcx
1412aa052: 8b4c2438 mov ecx, dword ptr [rsp + 0x38]
1412aa056: 42894c382c mov dword ptr [rax + r15 + 0x2c], ecx
1412aa05b: 4a896c3830 mov qword ptr [rax + r15 + 0x30], rbp
1412aa060: 42c744383800000000 mov dword ptr [rax + r15 + 0x38], 0
1412aa069: 4e89743840 mov qword ptr [rax + r15 + 0x40], r14
1412aa06e: 42c64438480f mov byte ptr [rax + r15 + 0x48], 0xf
1412aa074: e90e060000 jmp 0x1412aa687 ; unknown
1412aa079: f3410f6f07 movdqu xmm0, xmmword ptr [r15]
1412aa07e: f3410f6f4f0e movdqu xmm1, xmmword ptr [r15 + 0xe]
1412aa084: 660f740df4418101 pcmpeqb xmm1, xmmword ptr [rip + 0x18141f4]
1412aa08c: 660f74057c418101 pcmpeqb xmm0, xmmword ptr [rip + 0x181417c]
1412aa094: 660fdbc1 pand xmm0, xmm1
1412aa098: 660fd7c0 pmovmskb eax, xmm0
1412aa09c: 3dffff0000 cmp eax, 0xffff
1412aa0a1: 0f8455040000 je 0x1412aa4fc
1412aa0a7: f3410f6f07 movdqu xmm0, xmmword ptr [r15]
1412aa0ac: f3410f6f4f0e movdqu xmm1, xmmword ptr [r15 + 0xe]
1412aa0b2: 660f740d46418101 pcmpeqb xmm1, xmmword ptr [rip + 0x1814146]
1412aa0ba: 660f74054e418101 pcmpeqb xmm0, xmmword ptr [rip + 0x181414e]
1412aa0c2: 660fdbc1 pand xmm0, xmm1
1412aa0c6: 660fd7c0 pmovmskb eax, xmm0
1412aa0ca: 3dffff0000 cmp eax, 0xffff
1412aa0cf: 0f8508030000 jne 0x1412aa3dd
1412aa0d5: 488b8fa0000000 mov rcx, qword ptr [rdi + 0xa0]
1412aa0dc: 488b97a8000000 mov rdx, qword ptr [rdi + 0xa8]
1412aa0e3: e8982c9fff call 0x140c9cd80 ; utils::name_hash::NameHash::compute
1412aa0e8: 48898424a8000000 mov qword ptr [rsp + 0xa8], rax
1412aa0f0: c78424a000000000000000 mov dword ptr [rsp + 0xa0], 0
1412aa0fb: 488b442430 mov rax, qword ptr [rsp + 0x30]
1412aa100: 488984248c000000 mov qword ptr [rsp + 0x8c], rax
1412aa108: 8b442438 mov eax, dword ptr [rsp + 0x38]
1412aa10c: 89842494000000 mov dword ptr [rsp + 0x94], eax
1412aa113: e8681ad9fe call 0x14003bb80 ; bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>
1412aa118: c68424b000000006 mov byte ptr [rsp + 0xb0], 6
1412aa120: 440f29442470 movaps xmmword ptr [rsp + 0x70], xmm8
1412aa126: 0f13bc2480000000 movlps qword ptr [rsp + 0x80], xmm7
1412aa12e: f30f11b42488000000 movss dword ptr [rsp + 0x88], xmm6
1412aa137: 4889ac2498000000 mov qword ptr [rsp + 0x98], rbp
1412aa13f: 4c8d0d120e9801 lea r9, [rip + 0x1980e12]
1412aa146: 488d4c2440 lea rcx, [rsp + 0x40]
1412aa14b: 4c8d442470 lea r8, [rsp + 0x70]
1412aa150: 4889da mov rdx, rbx
1412aa153: e89846fcff call 0x14126e7f0 ; bevy_ecs::system::commands::Commands::spawn<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::SolidVertexColorBricksInstanceData> >
1412aa158: e978050000 jmp 0x1412aa6d5 ; unknown
1412aa15d: f3410f6f07 movdqu xmm0, xmmword ptr [r15]
1412aa162: f3410f6f4f05 movdqu xmm1, xmmword ptr [r15 + 5]
1412aa168: 660f740d500c9801 pcmpeqb xmm1, xmmword ptr [rip + 0x1980c50]
1412aa170: 660f7405580c9801 pcmpeqb xmm0, xmmword ptr [rip + 0x1980c58]
1412aa178: 660fdbc1 pand xmm0, xmm1
1412aa17c: 660fd7c0 pmovmskb eax, xmm0
1412aa180: 3dffff0000 cmp eax, 0xffff
1412aa185: 0f8552020000 jne 0x1412aa3dd
1412aa18b: 488b8fa0000000 mov rcx, qword ptr [rdi + 0xa0]
1412aa192: 488b97a8000000 mov rdx, qword ptr [rdi + 0xa8]
1412aa199: e8e22b9fff call 0x140c9cd80 ; utils::name_hash::NameHash::compute
1412aa19e: 48898424a8000000 mov qword ptr [rsp + 0xa8], rax
1412aa1a6: c78424a000000000000000 mov dword ptr [rsp + 0xa0], 0
1412aa1b1: 488b442430 mov rax, qword ptr [rsp + 0x30]
1412aa1b6: 488984248c000000 mov qword ptr [rsp + 0x8c], rax
1412aa1be: 8b442438 mov eax, dword ptr [rsp + 0x38]
1412aa1c2: 89842494000000 mov dword ptr [rsp + 0x94], eax
1412aa1c9: e8b219d9fe call 0x14003bb80 ; bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>
1412aa1ce: c68424b000000012 mov byte ptr [rsp + 0xb0], 0x12
1412aa1d6: 440f29442470 movaps xmmword ptr [rsp + 0x70], xmm8
1412aa1dc: 0f13bc2480000000 movlps qword ptr [rsp + 0x80], xmm7
1412aa1e4: f30f11b42488000000 movss dword ptr [rsp + 0x88], xmm6
1412aa1ed: 4889ac2498000000 mov qword ptr [rsp + 0x98], rbp
1412aa1f5: 4c8d0d2c0d9801 lea r9, [rip + 0x1980d2c]
1412aa1fc: 488d4c2440 lea rcx, [rsp + 0x40]
1412aa201: 4c8d442470 lea r8, [rsp + 0x70]
1412aa206: 4889da mov rdx, rbx
1412aa209: e8e243fcff call 0x14126e5f0 ; bevy_ecs::system::commands::Commands::spawn<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::LanternEmissiveInstanceData> >
1412aa20e: e9c2040000 jmp 0x1412aa6d5 ; unknown
1412aa213: f3410f6f07 movdqu xmm0, xmmword ptr [r15]
1412aa218: f3410f6f4f07 movdqu xmm1, xmmword ptr [r15 + 7]
1412aa21e: 660f740dba0b9801 pcmpeqb xmm1, xmmword ptr [rip + 0x1980bba]
1412aa226: 660f7405e23f8101 pcmpeqb xmm0, xmmword ptr [rip + 0x1813fe2]
1412aa22e: 660fdbc1 pand xmm0, xmm1
1412aa232: 660fd7c0 pmovmskb eax, xmm0
1412aa236: 3dffff0000 cmp eax, 0xffff
1412aa23b: 0f859c010000 jne 0x1412aa3dd
1412aa241: 488b8fa0000000 mov rcx, qword ptr [rdi + 0xa0]
1412aa248: 488b97a8000000 mov rdx, qword ptr [rdi + 0xa8]
1412aa24f: e82c2b9fff call 0x140c9cd80 ; utils::name_hash::NameHash::compute
1412aa254: 4989c6 mov r14, rax
1412aa257: e82419d9fe call 0x14003bb80 ; bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>
1412aa25c: 488d4c2470 lea rcx, [rsp + 0x70]
1412aa261: 4889da mov rdx, rbx
1412aa264: e837a4a6ff call 0x140d146a0 ; bevy_ecs::system::commands::Commands::spawn_empty
1412aa269: 488b5c2478 mov rbx, qword ptr [rsp + 0x78]
1412aa26e: 4885db test rbx, rbx
1412aa271: 7508 jne 0x1412aa27b
1412aa273: 488b9c2480000000 mov rbx, qword ptr [rsp + 0x80]
1412aa27b: 4c8ba42490000000 mov r12, qword ptr [rsp + 0x90]
1412aa283: 488b03 mov rax, qword ptr [rbx]
1412aa286: 4c8b7b10 mov r15, qword ptr [rbx + 0x10]
1412aa28a: 4c29f8 sub rax, r15
1412aa28d: 483d87000000 cmp rax, 0x87
1412aa293: 0f86f00e0000 jbe 0x1412ab189
1412aa299: 488b4308 mov rax, qword ptr [rbx + 8]
1412aa29d: 488d0dbc29fcff lea rcx, [rip - 0x3d644]
1412aa2a4: 4a890c38 mov qword ptr [rax + r15], rcx
1412aa2a8: 460f11443808 movups xmmword ptr [rax + r15 + 8], xmm8
1412aa2ae: 420f137c3818 movlps qword ptr [rax + r15 + 0x18], xmm7
1412aa2b4: f3420f11743820 movss dword ptr [rax + r15 + 0x20], xmm6
1412aa2bb: 488b4c2430 mov rcx, qword ptr [rsp + 0x30]
1412aa2c0: 4a894c3824 mov qword ptr [rax + r15 + 0x24], rcx
1412aa2c5: 8b4c2438 mov ecx, dword ptr [rsp + 0x38]
1412aa2c9: 42894c382c mov dword ptr [rax + r15 + 0x2c], ecx
1412aa2ce: 4a896c3830 mov qword ptr [rax + r15 + 0x30], rbp
1412aa2d3: 42c744383800000000 mov dword ptr [rax + r15 + 0x38], 0
1412aa2dc: 4e89743840 mov qword ptr [rax + r15 + 0x40], r14
1412aa2e1: 42c644384803 mov byte ptr [rax + r15 + 0x48], 3
1412aa2e7: e9c0020000 jmp 0x1412aa5ac ; unknown
1412aa2ec: 488d15400b9801 lea rdx, [rip + 0x1980b40]
1412aa2f3: 4d89c5 mov r13, r8
1412aa2f6: 41b824000000 mov r8d, 0x24
1412aa2fc: 4c89f9 mov rcx, r15
1412aa2ff: e8ec986701 call 0x142923bf0 ; unknown
1412aa304: 4d89e8 mov r8, r13
1412aa307: 85c0 test eax, eax
1412aa309: 0f85ce000000 jne 0x1412aa3dd
1412aa30f: 488b8fa0000000 mov rcx, qword ptr [rdi + 0xa0]
1412aa316: 488b97a8000000 mov rdx, qword ptr [rdi + 0xa8]
1412aa31d: e85e2a9fff call 0x140c9cd80 ; utils::name_hash::NameHash::compute
1412aa322: 4989c6 mov r14, rax
1412aa325: e85618d9fe call 0x14003bb80 ; bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>
1412aa32a: 488d4c2470 lea rcx, [rsp + 0x70]
1412aa32f: 4889da mov rdx, rbx
1412aa332: e869a3a6ff call 0x140d146a0 ; bevy_ecs::system::commands::Commands::spawn_empty
1412aa337: 488b5c2478 mov rbx, qword ptr [rsp + 0x78]
1412aa33c: 4885db test rbx, rbx
1412aa33f: 7508 jne 0x1412aa349
1412aa341: 488b9c2480000000 mov rbx, qword ptr [rsp + 0x80]
1412aa349: 4c8ba42490000000 mov r12, qword ptr [rsp + 0x90]
1412aa351: 488b03 mov rax, qword ptr [rbx]
1412aa354: 4c8b7b10 mov r15, qword ptr [rbx + 0x10]
1412aa358: 4c29f8 sub rax, r15
1412aa35b: 483d87000000 cmp rax, 0x87
1412aa361: 0f86470e0000 jbe 0x1412ab1ae
1412aa367: 488b4308 mov rax, qword ptr [rbx + 8]
1412aa36b: 488d0dde2dfcff lea rcx, [rip - 0x3d222]
1412aa372: 4a890c38 mov qword ptr [rax + r15], rcx
1412aa376: 460f11443808 movups xmmword ptr [rax + r15 + 8], xmm8
1412aa37c: 420f137c3818 movlps qword ptr [rax + r15 + 0x18], xmm7
1412aa382: f3420f11743820 movss dword ptr [rax + r15 + 0x20], xmm6
1412aa389: 488b4c2430 mov rcx, qword ptr [rsp + 0x30]
1412aa38e: 4a894c3824 mov qword ptr [rax + r15 + 0x24], rcx
1412aa393: 8b4c2438 mov ecx, dword ptr [rsp + 0x38]
1412aa397: 42894c382c mov dword ptr [rax + r15 + 0x2c], ecx
1412aa39c: 4a896c3830 mov qword ptr [rax + r15 + 0x30], rbp
1412aa3a1: 42c744383800000000 mov dword ptr [rax + r15 + 0x38], 0
1412aa3aa: 4e89743840 mov qword ptr [rax + r15 + 0x40], r14
1412aa3af: 42c644384805 mov byte ptr [rax + r15 + 0x48], 5
1412aa3b5: e9f2010000 jmp 0x1412aa5ac ; unknown
1412aa3ba: 488d154f0a9801 lea rdx, [rip + 0x1980a4f]
1412aa3c1: 4d89c5 mov r13, r8
1412aa3c4: 41b823000000 mov r8d, 0x23
1412aa3ca: 4c89f9 mov rcx, r15
1412aa3cd: e81e986701 call 0x142923bf0 ; unknown
1412aa3d2: 4d89e8 mov r8, r13
1412aa3d5: 85c0 test eax, eax
1412aa3d7: 0f84f2010000 je 0x1412aa5cf
1412aa3dd: 488b97a0000000 mov rdx, qword ptr [rdi + 0xa0]
1412aa3e4: 488b87a8000000 mov rax, qword ptr [rdi + 0xa8]
1412aa3eb: 4c89c1 mov rcx, r8
1412aa3ee: 4989c0 mov r8, rax
1412aa3f1: e8fa3068ff call 0x14092d4f0 ; country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_handle_by_name
1412aa3f6: 4989c5 mov r13, rax
1412aa3f9: 4c89f1 mov rcx, r14
1412aa3fc: 4c89fa mov rdx, r15
1412aa3ff: 4d89e0 mov r8, r12
1412aa402: e899d668ff call 0x140937aa0 ; country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
1412aa407: a801 test al, 1
1412aa409: 0f84330e0000 je 0x1412ab242
1412aa40f: 8b442438 mov eax, dword ptr [rsp + 0x38]
1412aa413: 89842494000000 mov dword ptr [rsp + 0x94], eax
1412aa41a: 488b442430 mov rax, qword ptr [rsp + 0x30]
1412aa41f: 488984248c000000 mov qword ptr [rsp + 0x8c], rax
1412aa427: 4c89ac24a0000000 mov qword ptr [rsp + 0xa0], r13
1412aa42f: 48899424a8000000 mov qword ptr [rsp + 0xa8], rdx
1412aa437: 440f29442470 movaps xmmword ptr [rsp + 0x70], xmm8
1412aa43d: 0f13bc2480000000 movlps qword ptr [rsp + 0x80], xmm7
1412aa445: f30f11b42488000000 movss dword ptr [rsp + 0x88], xmm6
1412aa44e: 4889ac2498000000 mov qword ptr [rsp + 0x98], rbp
1412aa456: 4c8d0d530a9801 lea r9, [rip + 0x1980a53]
1412aa45d: 488d4c2440 lea rcx, [rsp + 0x40]
1412aa462: 4c8d442470 lea r8, [rsp + 0x70]
1412aa467: 4889da mov rdx, rbx
1412aa46a: e8713dfcff call 0x14126e1e0 ; bevy_ecs::system::commands::Commands::spawn<country_core::components::drawable::DrawableMeshBundle>
1412aa46f: e961020000 jmp 0x1412aa6d5 ; unknown
1412aa474: 488b8fa0000000 mov rcx, qword ptr [rdi + 0xa0]
1412aa47b: 488b97a8000000 mov rdx, qword ptr [rdi + 0xa8]
1412aa482: e8f9289fff call 0x140c9cd80 ; utils::name_hash::NameHash::compute
1412aa487: 48898424a8000000 mov qword ptr [rsp + 0xa8], rax
1412aa48f: c78424a000000000000000 mov dword ptr [rsp + 0xa0], 0
1412aa49a: 488b442430 mov rax, qword ptr [rsp + 0x30]
1412aa49f: 488984248c000000 mov qword ptr [rsp + 0x8c], rax
1412aa4a7: 8b442438 mov eax, dword ptr [rsp + 0x38]
1412aa4ab: 89842494000000 mov dword ptr [rsp + 0x94], eax
1412aa4b2: e8c916d9fe call 0x14003bb80 ; bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>
1412aa4b7: c68424b000000014 mov byte ptr [rsp + 0xb0], 0x14
1412aa4bf: 440f29442470 movaps xmmword ptr [rsp + 0x70], xmm8
1412aa4c5: 0f13bc2480000000 movlps qword ptr [rsp + 0x80], xmm7
1412aa4cd: f30f11b42488000000 movss dword ptr [rsp + 0x88], xmm6
1412aa4d6: 4889ac2498000000 mov qword ptr [rsp + 0x98], rbp
1412aa4de: 4c8d0d2b0a9801 lea r9, [rip + 0x1980a2b]
1412aa4e5: 488d4c2440 lea rcx, [rsp + 0x40]
1412aa4ea: 4c8d442470 lea r8, [rsp + 0x70]
1412aa4ef: 4889da mov rdx, rbx
1412aa4f2: e8e938fcff call 0x14126dde0 ; bevy_ecs::system::commands::Commands::spawn<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::ClothInstanceData> >
1412aa4f7: e9d9010000 jmp 0x1412aa6d5 ; unknown
1412aa4fc: 48833f00 cmp qword ptr [rdi], 0
1412aa500: 0f856b0c0000 jne 0x1412ab171
1412aa506: 488b8fa0000000 mov rcx, qword ptr [rdi + 0xa0]
1412aa50d: 488b97a8000000 mov rdx, qword ptr [rdi + 0xa8]
1412aa514: e867289fff call 0x140c9cd80 ; utils::name_hash::NameHash::compute
1412aa519: 4989c6 mov r14, rax
1412aa51c: e85f16d9fe call 0x14003bb80 ; bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>
1412aa521: 488d4c2470 lea rcx, [rsp + 0x70]
1412aa526: 4889da mov rdx, rbx
1412aa529: e872a1a6ff call 0x140d146a0 ; bevy_ecs::system::commands::Commands::spawn_empty
1412aa52e: 488b5c2478 mov rbx, qword ptr [rsp + 0x78]
1412aa533: 4885db test rbx, rbx
1412aa536: 7508 jne 0x1412aa540
1412aa538: 488b9c2480000000 mov rbx, qword ptr [rsp + 0x80]
1412aa540: 4c8ba42490000000 mov r12, qword ptr [rsp + 0x90]
1412aa548: 488b03 mov rax, qword ptr [rbx]
1412aa54b: 4c8b7b10 mov r15, qword ptr [rbx + 0x10]
1412aa54f: 4c29f8 sub rax, r15
1412aa552: 483d87000000 cmp rax, 0x87
1412aa558: 0f86bf0c0000 jbe 0x1412ab21d
1412aa55e: 488b4308 mov rax, qword ptr [rbx + 8]
1412aa562: 488d0d4717fcff lea rcx, [rip - 0x3e8b9]
1412aa569: 4a890c38 mov qword ptr [rax + r15], rcx
1412aa56d: 460f11443808 movups xmmword ptr [rax + r15 + 8], xmm8
1412aa573: 420f137c3818 movlps qword ptr [rax + r15 + 0x18], xmm7
1412aa579: f3420f11743820 movss dword ptr [rax + r15 + 0x20], xmm6
1412aa580: 488b4c2430 mov rcx, qword ptr [rsp + 0x30]
1412aa585: 4a894c3824 mov qword ptr [rax + r15 + 0x24], rcx
1412aa58a: 8b4c2438 mov ecx, dword ptr [rsp + 0x38]
1412aa58e: 42894c382c mov dword ptr [rax + r15 + 0x2c], ecx
1412aa593: 4a896c3830 mov qword ptr [rax + r15 + 0x30], rbp
1412aa598: 42c744383800000000 mov dword ptr [rax + r15 + 0x38], 0
1412aa5a1: 4e89743840 mov qword ptr [rax + r15 + 0x40], r14
1412aa5a6: 42c644384804 mov byte ptr [rax + r15 + 0x48], 4
1412aa5ac: 42c644385800 mov byte ptr [rax + r15 + 0x58], 0
1412aa5b2: 4e89643868 mov qword ptr [rax + r15 + 0x68], r12
1412aa5b7: 488d0db27afdff lea rcx, [rip - 0x2854e]
1412aa5be: 4a894c3878 mov qword ptr [rax + r15 + 0x78], rcx
1412aa5c3: 4981c788000000 add r15, 0x88
1412aa5ca: e9de000000 jmp 0x1412aa6ad ; unknown
1412aa5cf: 488b8fa0000000 mov rcx, qword ptr [rdi + 0xa0]
1412aa5d6: 488b97a8000000 mov rdx, qword ptr [rdi + 0xa8]
1412aa5dd: e89e279fff call 0x140c9cd80 ; utils::name_hash::NameHash::compute
1412aa5e2: 4989c6 mov r14, rax
1412aa5e5: e89615d9fe call 0x14003bb80 ; bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>
1412aa5ea: 440fb6a750010000 movzx r12d, byte ptr [rdi + 0x150]
1412aa5f2: 4180fc09 cmp r12b, 9
1412aa5f6: 0f84640c0000 je 0x1412ab260
1412aa5fc: 488d4c2470 lea rcx, [rsp + 0x70]
1412aa601: 4889da mov rdx, rbx
1412aa604: e897a0a6ff call 0x140d146a0 ; bevy_ecs::system::commands::Commands::spawn_empty
1412aa609: 488b5c2478 mov rbx, qword ptr [rsp + 0x78]
1412aa60e: 4885db test rbx, rbx
1412aa611: 7508 jne 0x1412aa61b
1412aa613: 488b9c2480000000 mov rbx, qword ptr [rsp + 0x80]
1412aa61b: 4c8bac2490000000 mov r13, qword ptr [rsp + 0x90]
1412aa623: 488b03 mov rax, qword ptr [rbx]
1412aa626: 4c8b7b10 mov r15, qword ptr [rbx + 0x10]
1412aa62a: 4c29f8 sub rax, r15
1412aa62d: 483d97000000 cmp rax, 0x97
1412aa633: 0f86bf0b0000 jbe 0x1412ab1f8
1412aa639: 488b4308 mov rax, qword ptr [rbx + 8]
1412aa63d: 488d0dec13fcff lea rcx, [rip - 0x3ec14]
1412aa644: 4a890c38 mov qword ptr [rax + r15], rcx
1412aa648: 460f11443808 movups xmmword ptr [rax + r15 + 8], xmm8
1412aa64e: 420f137c3818 movlps qword ptr [rax + r15 + 0x18], xmm7
1412aa654: f3420f11743820 movss dword ptr [rax + r15 + 0x20], xmm6
1412aa65b: 488b4c2430 mov rcx, qword ptr [rsp + 0x30]
1412aa660: 4a894c3824 mov qword ptr [rax + r15 + 0x24], rcx
1412aa665: 8b4c2438 mov ecx, dword ptr [rsp + 0x38]
1412aa669: 42894c382c mov dword ptr [rax + r15 + 0x2c], ecx
1412aa66e: 4a896c3830 mov qword ptr [rax + r15 + 0x30], rbp
1412aa673: 42c744383800000000 mov dword ptr [rax + r15 + 0x38], 0
1412aa67c: 4e89743840 mov qword ptr [rax + r15 + 0x40], r14
1412aa681: 42c644384810 mov byte ptr [rax + r15 + 0x48], 0x10
1412aa687: 4688643858 mov byte ptr [rax + r15 + 0x58], r12b
1412aa68c: 42c644386800 mov byte ptr [rax + r15 + 0x68], 0
1412aa692: 4e896c3878 mov qword ptr [rax + r15 + 0x78], r13
1412aa697: 488d0dd279fdff lea rcx, [rip - 0x2862e]
1412aa69e: 4a898c3888000000 mov qword ptr [rax + r15 + 0x88], rcx
1412aa6a6: 4981c798000000 add r15, 0x98
1412aa6ad: 4c897b10 mov qword ptr [rbx + 0x10], r15
1412aa6b1: 488b842490000000 mov rax, qword ptr [rsp + 0x90]
1412aa6b9: 4889442460 mov qword ptr [rsp + 0x60], rax
1412aa6be: 0f10442470 movups xmm0, xmmword ptr [rsp + 0x70]
1412aa6c3: 0f108c2480000000 movups xmm1, xmmword ptr [rsp + 0x80]
1412aa6cb: 0f294c2450 movaps xmmword ptr [rsp + 0x50], xmm1
1412aa6d0: 0f29442440 movaps xmmword ptr [rsp + 0x40], xmm0
1412aa6d5: f687f000000001 test byte ptr [rdi + 0xf0], 1
1412aa6dc: 0f85dd000000 jne 0x1412aa7bf
1412aa6e2: 837f4001 cmp dword ptr [rdi + 0x40], 1
1412aa6e6: 0f843e010000 je 0x1412aa82a
1412aa6ec: 80bf1401000002 cmp byte ptr [rdi + 0x114], 2
1412aa6f3: 0f85c3010000 jne 0x1412aa8bc
1412aa6f9: 83bf8000000001 cmp dword ptr [rdi + 0x80], 1
1412aa700: 0f8433020000 je 0x1412aa939
1412aa706: 80bf4e01000000 cmp byte ptr [rdi + 0x14e], 0
1412aa70d: 0f85a1020000 jne 0x1412aa9b4
1412aa713: f6471001 test byte ptr [rdi + 0x10], 1
1412aa717: 0f85f7020000 jne 0x1412aaa14
1412aa71d: 80bf4f01000000 cmp byte ptr [rdi + 0x14f], 0
1412aa724: 0f855b030000 jne 0x1412aaa85
1412aa72a: 0fb6af4c010000 movzx ebp, byte ptr [rdi + 0x14c]
1412aa731: 4080fd02 cmp bpl, 2
1412aa735: 0f8558040000 jne 0x1412aab93
1412aa73b: 837f2001 cmp dword ptr [rdi + 0x20], 1
1412aa73f: 0f84d1040000 je 0x1412aac16
1412aa745: f60701 test byte ptr [rdi], 1
1412aa748: 0f8430050000 je 0x1412aac7e
1412aa74e: 4c8b742448 mov r14, qword ptr [rsp + 0x48]
1412aa753: 4d85f6 test r14, r14
1412aa756: 7505 jne 0x1412aa75d
1412aa758: 4c8b742450 mov r14, qword ptr [rsp + 0x50]
1412aa75d: 4c8b6708 mov r12, qword ptr [rdi + 8]
1412aa761: 488b5c2460 mov rbx, qword ptr [rsp + 0x60]
1412aa766: 498b06 mov rax, qword ptr [r14]
1412aa769: 4d8b7e10 mov r15, qword ptr [r14 + 0x10]
1412aa76d: 4c29f8 sub rax, r15
1412aa770: 4883f827 cmp rax, 0x27
1412aa774: 0f86eb070000 jbe 0x1412aaf65
1412aa77a: 498b4608 mov rax, qword ptr [r14 + 8]
1412aa77e: 488d0dcb10fcff lea rcx, [rip - 0x3ef35]
1412aa785: 4a890c38 mov qword ptr [rax + r15], rcx
1412aa789: 488d0de078fdff lea rcx, [rip - 0x28720]
1412aa790: 4a894c3808 mov qword ptr [rax + r15 + 8], rcx
1412aa795: 4a895c3810 mov qword ptr [rax + r15 + 0x10], rbx
1412aa79a: 4e89643818 mov qword ptr [rax + r15 + 0x18], r12
1412aa79f: 42c644382000 mov byte ptr [rax + r15 + 0x20], 0
1412aa7a5: 4983c728 add r15, 0x28
1412aa7a9: 4d897e10 mov qword ptr [r14 + 0x10], r15
1412aa7ad: 83bff800000001 cmp dword ptr [rdi + 0xf8], 1
1412aa7b4: 0f84d2040000 je 0x1412aac8c
1412aa7ba: e929050000 jmp 0x1412aace8 ; unknown
1412aa7bf: 488b5c2448 mov rbx, qword ptr [rsp + 0x48]
1412aa7c4: 4885db test rbx, rbx
1412aa7c7: 7505 jne 0x1412aa7ce
1412aa7c9: 488b5c2450 mov rbx, qword ptr [rsp + 0x50]
1412aa7ce: 8baff4000000 mov ebp, dword ptr [rdi + 0xf4]
1412aa7d4: 4c8b7c2460 mov r15, qword ptr [rsp + 0x60]
1412aa7d9: 488b03 mov rax, qword ptr [rbx]
1412aa7dc: 4c8b7310 mov r14, qword ptr [rbx + 0x10]
1412aa7e0: 4c29f0 sub rax, r14
1412aa7e3: 4883f81f cmp rax, 0x1f
1412aa7e7: 0f869d070000 jbe 0x1412aaf8a
1412aa7ed: 488b4308 mov rax, qword ptr [rbx + 8]
1412aa7f1: 488d0dd82bfcff lea rcx, [rip - 0x3d428]
1412aa7f8: 4a890c30 mov qword ptr [rax + r14], rcx
1412aa7fc: 488d0d6d78fdff lea rcx, [rip - 0x28793]
1412aa803: 4a894c3008 mov qword ptr [rax + r14 + 8], rcx
1412aa808: 4e897c3010 mov qword ptr [rax + r14 + 0x10], r15
1412aa80d: 42896c3018 mov dword ptr [rax + r14 + 0x18], ebp
1412aa812: 42c644301c00 mov byte ptr [rax + r14 + 0x1c], 0
1412aa818: 4983c620 add r14, 0x20
1412aa81c: 4c897310 mov qword ptr [rbx + 0x10], r14
1412aa820: 837f4001 cmp dword ptr [rdi + 0x40], 1
1412aa824: 0f85c2feffff jne 0x1412aa6ec
1412aa82a: 488b5c2448 mov rbx, qword ptr [rsp + 0x48]
1412aa82f: 4885db test rbx, rbx
1412aa832: 7505 jne 0x1412aa839
1412aa834: 488b5c2450 mov rbx, qword ptr [rsp + 0x50]
1412aa839: 4c8d7f44 lea r15, [rdi + 0x44]
1412aa83d: 4c8b642460 mov r12, qword ptr [rsp + 0x60]
1412aa842: 488b03 mov rax, qword ptr [rbx]
1412aa845: 4c8b7310 mov r14, qword ptr [rbx + 0x10]
1412aa849: 4c29f0 sub rax, r14
1412aa84c: 4883f857 cmp rax, 0x57
1412aa850: 0f8659070000 jbe 0x1412aafaf
1412aa856: 488b4308 mov rax, qword ptr [rbx + 8]
1412aa85a: 488d0d0f1cfcff lea rcx, [rip - 0x3e3f1]
1412aa861: 4a890c30 mov qword ptr [rax + r14], rcx
1412aa865: 488d0d0478fdff lea rcx, [rip - 0x287fc]
1412aa86c: 4a894c3008 mov qword ptr [rax + r14 + 8], rcx
1412aa871: 4e89643010 mov qword ptr [rax + r14 + 0x10], r12
1412aa876: 410f1007 movups xmm0, xmmword ptr [r15]
1412aa87a: 410f104f10 movups xmm1, xmmword ptr [r15 + 0x10]
1412aa87f: 410f105720 movups xmm2, xmmword ptr [r15 + 0x20]
1412aa884: 410f105f2c movups xmm3, xmmword ptr [r15 + 0x2c]
1412aa889: 420f11443018 movups xmmword ptr [rax + r14 + 0x18], xmm0
1412aa88f: 420f114c3028 movups xmmword ptr [rax + r14 + 0x28], xmm1
1412aa895: 420f11543038 movups xmmword ptr [rax + r14 + 0x38], xmm2
1412aa89b: 420f115c3044 movups xmmword ptr [rax + r14 + 0x44], xmm3
1412aa8a1: 42c644305400 mov byte ptr [rax + r14 + 0x54], 0
1412aa8a7: 4983c658 add r14, 0x58
1412aa8ab: 4c897310 mov qword ptr [rbx + 0x10], r14
1412aa8af: 80bf1401000002 cmp byte ptr [rdi + 0x114], 2
1412aa8b6: 0f843dfeffff je 0x1412aa6f9
1412aa8bc: 488b5c2448 mov rbx, qword ptr [rsp + 0x48]
1412aa8c1: 4885db test rbx, rbx
1412aa8c4: 7505 jne 0x1412aa8cb
1412aa8c6: 488b5c2450 mov rbx, qword ptr [rsp + 0x50]
1412aa8cb: 4c8dbf00010000 lea r15, [rdi + 0x100]
1412aa8d2: 4c8b642460 mov r12, qword ptr [rsp + 0x60]
1412aa8d7: 488b03 mov rax, qword ptr [rbx]
1412aa8da: 4c8b7310 mov r14, qword ptr [rbx + 0x10]
1412aa8de: 4c29f0 sub rax, r14
1412aa8e1: 4883f837 cmp rax, 0x37
1412aa8e5: 0f86e9060000 jbe 0x1412aafd4
1412aa8eb: 488b4308 mov rax, qword ptr [rbx + 8]
1412aa8ef: 488d0dea08fcff lea rcx, [rip - 0x3f716]
1412aa8f6: 4a890c30 mov qword ptr [rax + r14], rcx
1412aa8fa: 488d0d6f77fdff lea rcx, [rip - 0x28891]
1412aa901: 4a894c3008 mov qword ptr [rax + r14 + 8], rcx
1412aa906: 4e89643010 mov qword ptr [rax + r14 + 0x10], r12
1412aa90b: 410f1007 movups xmm0, xmmword ptr [r15]
1412aa90f: 420f11443018 movups xmmword ptr [rax + r14 + 0x18], xmm0
1412aa915: 498b4f10 mov rcx, qword ptr [r15 + 0x10]
1412aa919: 4a894c3028 mov qword ptr [rax + r14 + 0x28], rcx
1412aa91e: 42c644303000 mov byte ptr [rax + r14 + 0x30], 0
1412aa924: 4983c638 add r14, 0x38
1412aa928: 4c897310 mov qword ptr [rbx + 0x10], r14
1412aa92c: 83bf8000000001 cmp dword ptr [rdi + 0x80], 1
1412aa933: 0f85cdfdffff jne 0x1412aa706
1412aa939: 488b5c2448 mov rbx, qword ptr [rsp + 0x48]
1412aa93e: 4885db test rbx, rbx
1412aa941: 7505 jne 0x1412aa948
1412aa943: 488b5c2450 mov rbx, qword ptr [rsp + 0x50]
1412aa948: 4c8dbf84000000 lea r15, [rdi + 0x84]
1412aa94f: 4c8b642460 mov r12, qword ptr [rsp + 0x60]
1412aa954: 488b03 mov rax, qword ptr [rbx]
1412aa957: 4c8b7310 mov r14, qword ptr [rbx + 0x10]
1412aa95b: 4c29f0 sub rax, r14
1412aa95e: 4883f827 cmp rax, 0x27
1412aa962: 0f8691060000 jbe 0x1412aaff9
1412aa968: 488b4308 mov rax, qword ptr [rbx + 8]
1412aa96c: 488d0dcd0ffcff lea rcx, [rip - 0x3f033]
1412aa973: 4a890c30 mov qword ptr [rax + r14], rcx
1412aa977: 488d0df276fdff lea rcx, [rip - 0x2890e]
1412aa97e: 4a894c3008 mov qword ptr [rax + r14 + 8], rcx
1412aa983: 4e89643010 mov qword ptr [rax + r14 + 0x10], r12
1412aa988: 498b0f mov rcx, qword ptr [r15]
1412aa98b: 4a894c3018 mov qword ptr [rax + r14 + 0x18], rcx
1412aa990: 418b4f08 mov ecx, dword ptr [r15 + 8]
1412aa994: 42894c3020 mov dword ptr [rax + r14 + 0x20], ecx
1412aa999: 42c644302400 mov byte ptr [rax + r14 + 0x24], 0
1412aa99f: 4983c628 add r14, 0x28
1412aa9a3: 4c897310 mov qword ptr [rbx + 0x10], r14
1412aa9a7: 80bf4e01000000 cmp byte ptr [rdi + 0x14e], 0
1412aa9ae: 0f845ffdffff je 0x1412aa713
1412aa9b4: 488b5c2448 mov rbx, qword ptr [rsp + 0x48]
1412aa9b9: 4885db test rbx, rbx
1412aa9bc: 7505 jne 0x1412aa9c3
1412aa9be: 488b5c2450 mov rbx, qword ptr [rsp + 0x50]
1412aa9c3: 4c8b7c2460 mov r15, qword ptr [rsp + 0x60]
1412aa9c8: 488b03 mov rax, qword ptr [rbx]
1412aa9cb: 4c8b7310 mov r14, qword ptr [rbx + 0x10]
1412aa9cf: 4c29f0 sub rax, r14
1412aa9d2: 4883f81f cmp rax, 0x1f
1412aa9d6: 0f8642060000 jbe 0x1412ab01e
1412aa9dc: 488b4308 mov rax, qword ptr [rbx + 8]
1412aa9e0: 488d0df928fcff lea rcx, [rip - 0x3d707]
1412aa9e7: 4a890c30 mov qword ptr [rax + r14], rcx
1412aa9eb: 488d0d7e76fdff lea rcx, [rip - 0x28982]
1412aa9f2: 4a894c3008 mov qword ptr [rax + r14 + 8], rcx
1412aa9f7: 4e897c3010 mov qword ptr [rax + r14 + 0x10], r15
1412aa9fc: 42c644301800 mov byte ptr [rax + r14 + 0x18], 0
1412aaa02: 4983c620 add r14, 0x20
1412aaa06: 4c897310 mov qword ptr [rbx + 0x10], r14
1412aaa0a: f6471001 test byte ptr [rdi + 0x10], 1
1412aaa0e: 0f8409fdffff je 0x1412aa71d
1412aaa14: 488b5c2448 mov rbx, qword ptr [rsp + 0x48]
1412aaa19: 4885db test rbx, rbx
1412aaa1c: 7505 jne 0x1412aaa23
1412aaa1e: 488b5c2450 mov rbx, qword ptr [rsp + 0x50]
1412aaa23: 4c8b7f18 mov r15, qword ptr [rdi + 0x18]
1412aaa27: 4c8b642460 mov r12, qword ptr [rsp + 0x60]
1412aaa2c: 488b03 mov rax, qword ptr [rbx]
1412aaa2f: 4c8b7310 mov r14, qword ptr [rbx + 0x10]
1412aaa33: 4c29f0 sub rax, r14
1412aaa36: 4883f82f cmp rax, 0x2f
1412aaa3a: 0f8603060000 jbe 0x1412ab043
1412aaa40: 488b4308 mov rax, qword ptr [rbx + 8]
1412aaa44: 488d0d450bfcff lea rcx, [rip - 0x3f4bb]
1412aaa4b: 4a890c30 mov qword ptr [rax + r14], rcx
1412aaa4f: 488d0d1a76fdff lea rcx, [rip - 0x289e6]
1412aaa56: 4a894c3008 mov qword ptr [rax + r14 + 8], rcx
1412aaa5b: 4e89643010 mov qword ptr [rax + r14 + 0x10], r12
1412aaa60: 4e897c3018 mov qword ptr [rax + r14 + 0x18], r15
1412aaa65: 4289743020 mov dword ptr [rax + r14 + 0x20], esi
1412aaa6a: 42c644302800 mov byte ptr [rax + r14 + 0x28], 0
1412aaa70: 4983c630 add r14, 0x30
1412aaa74: 4c897310 mov qword ptr [rbx + 0x10], r14
1412aaa78: 80bf4f01000000 cmp byte ptr [rdi + 0x14f], 0
1412aaa7f: 0f84a5fcffff je 0x1412aa72a
1412aaa85: 410f28c8 movaps xmm1, xmm8
1412aaa89: 410f59c8 mulps xmm1, xmm8
1412aaa8d: f30f16c1 movshdup xmm0, xmm1
1412aaa91: f30f58c1 addss xmm0, xmm1
1412aaa95: 0f28d1 movaps xmm2, xmm1
1412aaa98: 660f15d1 unpckhpd xmm2, xmm1
1412aaa9c: f30f58d0 addss xmm2, xmm0
1412aaaa0: 0fc6d200 shufps xmm2, xmm2, 0
1412aaaa4: 0fc6c9ff shufps xmm1, xmm1, 0xff
1412aaaa8: 0f5cca subps xmm1, xmm2
1412aaaab: 0f28050e267201 movaps xmm0, xmmword ptr [rip + 0x172260e]
1412aaab2: 0f59c8 mulps xmm1, xmm0
1412aaab5: 410f59c0 mulps xmm0, xmm8
1412aaab9: f30f16d0 movshdup xmm2, xmm0
1412aaabd: f30f58d0 addss xmm2, xmm0
1412aaac1: 0f12c0 movhlps xmm0, xmm0
1412aaac4: f30f58c2 addss xmm0, xmm2
1412aaac8: f30f58c0 addss xmm0, xmm0
1412aaacc: 0fc6c000 shufps xmm0, xmm0, 0
1412aaad0: 410f59c0 mulps xmm0, xmm8
1412aaad4: 0f58c1 addps xmm0, xmm1
1412aaad7: f30f100d25ae6701 movss xmm1, dword ptr [rip + 0x167ae25]
1412aaadf: 410f28d0 movaps xmm2, xmm8
1412aaae3: 0f59d1 mulps xmm2, xmm1
1412aaae6: 410f28d8 movaps xmm3, xmm8
1412aaaea: 410fc6d8c9 shufps xmm3, xmm8, 0xc9
1412aaaef: 0f59d9 mulps xmm3, xmm1
1412aaaf2: 0fc6d2d2 shufps xmm2, xmm2, 0xd2
1412aaaf6: 0f5cda subps xmm3, xmm2
1412aaaf9: 450f28c8 movaps xmm9, xmm8
1412aaafd: 450f58c8 addps xmm9, xmm8
1412aab01: 450fc6c9ff shufps xmm9, xmm9, 0xff
1412aab06: 440f59cb mulps xmm9, xmm3
1412aab0a: 488b5c2448 mov rbx, qword ptr [rsp + 0x48]
1412aab0f: 4885db test rbx, rbx
1412aab12: 7505 jne 0x1412aab19
1412aab14: 488b5c2450 mov rbx, qword ptr [rsp + 0x50]
1412aab19: 440f58c8 addps xmm9, xmm0
1412aab1d: 4c8b7c2460 mov r15, qword ptr [rsp + 0x60]
1412aab22: 488b03 mov rax, qword ptr [rbx]
1412aab25: 4c8b7310 mov r14, qword ptr [rbx + 0x10]
1412aab29: 4c29f0 sub rax, r14
1412aab2c: 4883f837 cmp rax, 0x37
1412aab30: 0f8632050000 jbe 0x1412ab068
1412aab36: 488b4308 mov rax, qword ptr [rbx + 8]
1412aab3a: 488d0daf07fcff lea rcx, [rip - 0x3f851]
1412aab41: 4a890c30 mov qword ptr [rax + r14], rcx
1412aab45: 488d0d2475fdff lea rcx, [rip - 0x28adc]
1412aab4c: 4a894c3008 mov qword ptr [rax + r14 + 8], rcx
1412aab51: 4e897c3010 mov qword ptr [rax + r14 + 0x10], r15
1412aab56: 420f137c3018 movlps qword ptr [rax + r14 + 0x18], xmm7
1412aab5c: f3420f11743020 movss dword ptr [rax + r14 + 0x20], xmm6
1412aab63: 460f134c3024 movlps qword ptr [rax + r14 + 0x24], xmm9
1412aab69: 450f12c9 movhlps xmm9, xmm9
1412aab6d: f3460f114c302c movss dword ptr [rax + r14 + 0x2c], xmm9
1412aab74: 42c644303000 mov byte ptr [rax + r14 + 0x30], 0
1412aab7a: 4983c638 add r14, 0x38
1412aab7e: 4c897310 mov qword ptr [rbx + 0x10], r14
1412aab82: 0fb6af4c010000 movzx ebp, byte ptr [rdi + 0x14c]
1412aab89: 4080fd02 cmp bpl, 2
1412aab8d: 0f84a8fbffff je 0x1412aa73b
1412aab93: 4189f7 mov r15d, esi
1412aab96: 488b5c2448 mov rbx, qword ptr [rsp + 0x48]
1412aab9b: 4885db test rbx, rbx
1412aab9e: 7505 jne 0x1412aaba5
1412aaba0: 488b5c2450 mov rbx, qword ptr [rsp + 0x50]
1412aaba5: 440fb6a74d010000 movzx r12d, byte ptr [rdi + 0x14d]
1412aabad: 4c8b6c2460 mov r13, qword ptr [rsp + 0x60]
1412aabb2: 488b03 mov rax, qword ptr [rbx]
1412aabb5: 4c8b7310 mov r14, qword ptr [rbx + 0x10]
1412aabb9: 4c29f0 sub rax, r14
1412aabbc: 4883f82f cmp rax, 0x2f
1412aabc0: 0f86c7040000 jbe 0x1412ab08d
1412aabc6: 488b4308 mov rax, qword ptr [rbx + 8]
1412aabca: 488d0d7f1ffcff lea rcx, [rip - 0x3e081]
1412aabd1: 4a890c30 mov qword ptr [rax + r14], rcx
1412aabd5: 488d0d9474fdff lea rcx, [rip - 0x28b6c]
1412aabdc: 4a894c3008 mov qword ptr [rax + r14 + 8], rcx
1412aabe1: 4e896c3010 mov qword ptr [rax + r14 + 0x10], r13
1412aabe6: 4e897c3018 mov qword ptr [rax + r14 + 0x18], r15
1412aabeb: 42c744302000000000 mov dword ptr [rax + r14 + 0x20], 0
1412aabf4: 42886c3024 mov byte ptr [rax + r14 + 0x24], bpl
1412aabf9: 4688643025 mov byte ptr [rax + r14 + 0x25], r12b
1412aabfe: 42c644302800 mov byte ptr [rax + r14 + 0x28], 0
1412aac04: 4983c630 add r14, 0x30
1412aac08: 4c897310 mov qword ptr [rbx + 0x10], r14
1412aac0c: 837f2001 cmp dword ptr [rdi + 0x20], 1
1412aac10: 0f852ffbffff jne 0x1412aa745
1412aac16: 488b5c2448 mov rbx, qword ptr [rsp + 0x48]
1412aac1b: 4885db test rbx, rbx
1412aac1e: 7505 jne 0x1412aac25
1412aac20: 488b5c2450 mov rbx, qword ptr [rsp + 0x50]
1412aac25: 4c8b7f28 mov r15, qword ptr [rdi + 0x28]
1412aac29: 4c8b642460 mov r12, qword ptr [rsp + 0x60]
1412aac2e: 488b03 mov rax, qword ptr [rbx]
1412aac31: 4c8b7310 mov r14, qword ptr [rbx + 0x10]
1412aac35: 4c29f0 sub rax, r14
1412aac38: 4883f827 cmp rax, 0x27
1412aac3c: 0f8670040000 jbe 0x1412ab0b2
1412aac42: 488b4308 mov rax, qword ptr [rbx + 8]
1412aac46: 488d0d7329fcff lea rcx, [rip - 0x3d68d]
1412aac4d: 4a890c30 mov qword ptr [rax + r14], rcx
1412aac51: 488d0d1874fdff lea rcx, [rip - 0x28be8]
1412aac58: 4a894c3008 mov qword ptr [rax + r14 + 8], rcx
1412aac5d: 4e89643010 mov qword ptr [rax + r14 + 0x10], r12
1412aac62: 4e897c3018 mov qword ptr [rax + r14 + 0x18], r15
1412aac67: 42c644302000 mov byte ptr [rax + r14 + 0x20], 0
1412aac6d: 4983c628 add r14, 0x28
1412aac71: 4c897310 mov qword ptr [rbx + 0x10], r14
1412aac75: f60701 test byte ptr [rdi], 1
1412aac78: 0f85d0faffff jne 0x1412aa74e
1412aac7e: 488b5c2460 mov rbx, qword ptr [rsp + 0x60]
1412aac83: 83bff800000001 cmp dword ptr [rdi + 0xf8], 1
1412aac8a: 755c jne 0x1412aace8
1412aac8c: 4c8b742448 mov r14, qword ptr [rsp + 0x48]
1412aac91: 4d85f6 test r14, r14
1412aac94: 7505 jne 0x1412aac9b
1412aac96: 4c8b742450 mov r14, qword ptr [rsp + 0x50]
1412aac9b: 8baffc000000 mov ebp, dword ptr [rdi + 0xfc]
1412aaca1: 498b06 mov rax, qword ptr [r14]
1412aaca4: 4d8b7e10 mov r15, qword ptr [r14 + 0x10]
1412aaca8: 4c29f8 sub rax, r15
1412aacab: 4883f81f cmp rax, 0x1f
1412aacaf: 0f8622040000 jbe 0x1412ab0d7
1412aacb5: 498b4608 mov rax, qword ptr [r14 + 8]
1412aacb9: 488d0d001bfcff lea rcx, [rip - 0x3e500]
1412aacc0: 4a890c38 mov qword ptr [rax + r15], rcx
1412aacc4: 488d0da573fdff lea rcx, [rip - 0x28c5b]
1412aaccb: 4a894c3808 mov qword ptr [rax + r15 + 8], rcx
1412aacd0: 4a895c3810 mov qword ptr [rax + r15 + 0x10], rbx
1412aacd5: 42896c3818 mov dword ptr [rax + r15 + 0x18], ebp
1412aacda: 42c644381c00 mov byte ptr [rax + r15 + 0x1c], 0
1412aace0: 4983c720 add r15, 0x20
1412aace4: 4d897e10 mov qword ptr [r14 + 0x10], r15
1412aace8: 80bf3c01000002 cmp byte ptr [rdi + 0x13c], 2
1412aacef: 7527 jne 0x1412aad18
1412aacf1: 83bf1801000001 cmp dword ptr [rdi + 0x118], 1
1412aacf8: 0f8494000000 je 0x1412aad92
1412aacfe: 488b8c2480010000 mov rcx, qword ptr [rsp + 0x180]
1412aad06: 83bf9000000001 cmp dword ptr [rdi + 0x90], 1
1412aad0d: 0f84a2010000 je 0x1412aaeb5
1412aad13: e90a020000 jmp 0x1412aaf22 ; unknown
1412aad18: 4c8b742448 mov r14, qword ptr [rsp + 0x48]
1412aad1d: 4d85f6 test r14, r14
1412aad20: 7505 jne 0x1412aad27
1412aad22: 4c8b742450 mov r14, qword ptr [rsp + 0x50]
1412aad27: 4c8da72c010000 lea r12, [rdi + 0x12c]
1412aad2e: 498b06 mov rax, qword ptr [r14]
1412aad31: 4d8b7e10 mov r15, qword ptr [r14 + 0x10]
1412aad35: 4c29f8 sub rax, r15
1412aad38: 4883f82f cmp rax, 0x2f
1412aad3c: 0f86ba030000 jbe 0x1412ab0fc
1412aad42: 498b4608 mov rax, qword ptr [r14 + 8]
1412aad46: 488d0df31cfcff lea rcx, [rip - 0x3e30d]
1412aad4d: 4a890c38 mov qword ptr [rax + r15], rcx
1412aad51: 488d0d1873fdff lea rcx, [rip - 0x28ce8]
1412aad58: 4a894c3808 mov qword ptr [rax + r15 + 8], rcx
1412aad5d: 4a895c3810 mov qword ptr [rax + r15 + 0x10], rbx
1412aad62: 410f100424 movups xmm0, xmmword ptr [r12]
1412aad67: 420f11443818 movups xmmword ptr [rax + r15 + 0x18], xmm0
1412aad6d: 418b4c2410 mov ecx, dword ptr [r12 + 0x10]
1412aad72: 42894c3828 mov dword ptr [rax + r15 + 0x28], ecx
1412aad77: 42c644382c00 mov byte ptr [rax + r15 + 0x2c], 0
1412aad7d: 4983c730 add r15, 0x30
1412aad81: 4d897e10 mov qword ptr [r14 + 0x10], r15
1412aad85: 83bf1801000001 cmp dword ptr [rdi + 0x118], 1
1412aad8c: 0f856cffffff jne 0x1412aacfe
1412aad92: 440f108f1c010000 movups xmm9, xmmword ptr [rdi + 0x11c]
1412aad9a: 410f28c1 movaps xmm0, xmm9
1412aad9e: 410fc6c1a4 shufps xmm0, xmm9, 0xa4
1412aada3: 410f28c8 movaps xmm1, xmm8
1412aada7: 410f59c8 mulps xmm1, xmm8
1412aadab: f30f16d1 movshdup xmm2, xmm1
1412aadaf: f30f58d1 addss xmm2, xmm1
1412aadb3: 0f28d9 movaps xmm3, xmm1
1412aadb6: 660f15d9 unpckhpd xmm3, xmm1
1412aadba: f30f58da addss xmm3, xmm2
1412aadbe: 0fc6db00 shufps xmm3, xmm3, 0
1412aadc2: 0fc6c9ff shufps xmm1, xmm1, 0xff
1412aadc6: 0f5ccb subps xmm1, xmm3
1412aadc9: 0f59c8 mulps xmm1, xmm0
1412aadcc: 410f28c0 movaps xmm0, xmm8
1412aadd0: 410f59c1 mulps xmm0, xmm9
1412aadd4: f30f16d0 movshdup xmm2, xmm0
1412aadd8: f30f58d0 addss xmm2, xmm0
1412aaddc: 0f12c0 movhlps xmm0, xmm0
1412aaddf: f30f58c2 addss xmm0, xmm2
1412aade3: f30f58c0 addss xmm0, xmm0
1412aade7: 0fc6c000 shufps xmm0, xmm0, 0
1412aadeb: 410f59c0 mulps xmm0, xmm8
1412aadef: 0f58c1 addps xmm0, xmm1
1412aadf2: 410f28c8 movaps xmm1, xmm8
1412aadf6: 410fc6c8d2 shufps xmm1, xmm8, 0xd2
1412aadfb: 410f28d1 movaps xmm2, xmm9
1412aadff: 410fc6d1d2 shufps xmm2, xmm9, 0xd2
1412aae04: 410f59c9 mulps xmm1, xmm9
1412aae08: 410f59d0 mulps xmm2, xmm8
1412aae0c: 0f5cca subps xmm1, xmm2
1412aae0f: 0fc6c9d2 shufps xmm1, xmm1, 0xd2
1412aae13: 450f58c0 addps xmm8, xmm8
1412aae17: 450fc6c0ff shufps xmm8, xmm8, 0xff
1412aae1c: 440f59c1 mulps xmm8, xmm1
1412aae20: 440f58c0 addps xmm8, xmm0
1412aae24: 410f28c0 movaps xmm0, xmm8
1412aae28: 66410f15c0 unpckhpd xmm0, xmm8
1412aae2d: 4c8b742448 mov r14, qword ptr [rsp + 0x48]
1412aae32: 4d85f6 test r14, r14
1412aae35: 7505 jne 0x1412aae3c
1412aae37: 4c8b742450 mov r14, qword ptr [rsp + 0x50]
1412aae3c: 410f58f8 addps xmm7, xmm8
1412aae40: f30f58f0 addss xmm6, xmm0
1412aae44: 498b06 mov rax, qword ptr [r14]
1412aae47: 4d8b7e10 mov r15, qword ptr [r14 + 0x10]
1412aae4b: 4c29f8 sub rax, r15
1412aae4e: 4883f83f cmp rax, 0x3f
1412aae52: 0f86c9020000 jbe 0x1412ab121
1412aae58: 498b4608 mov rax, qword ptr [r14 + 8]
1412aae5c: 488d0d8d1ffcff lea rcx, [rip - 0x3e073]
1412aae63: 4a890c38 mov qword ptr [rax + r15], rcx
1412aae67: 488d0d0272fdff lea rcx, [rip - 0x28dfe]
1412aae6e: 4a894c3808 mov qword ptr [rax + r15 + 8], rcx
1412aae73: 4a895c3810 mov qword ptr [rax + r15 + 0x10], rbx
1412aae78: 420f137c3818 movlps qword ptr [rax + r15 + 0x18], xmm7
1412aae7e: f3420f11743820 movss dword ptr [rax + r15 + 0x20], xmm6
1412aae85: 460f114c3824 movups xmmword ptr [rax + r15 + 0x24], xmm9
1412aae8b: 4289743834 mov dword ptr [rax + r15 + 0x34], esi
1412aae90: 42c644383800 mov byte ptr [rax + r15 + 0x38], 0
1412aae96: 42c644383c00 mov byte ptr [rax + r15 + 0x3c], 0
1412aae9c: 4983c740 add r15, 0x40
1412aaea0: 4d897e10 mov qword ptr [r14 + 0x10], r15
1412aaea4: 488b8c2480010000 mov rcx, qword ptr [rsp + 0x180]
1412aaeac: 83bf9000000001 cmp dword ptr [rdi + 0x90], 1
1412aaeb3: 756d jne 0x1412aaf22
1412aaeb5: 4c8b742448 mov r14, qword ptr [rsp + 0x48]
1412aaeba: 4d85f6 test r14, r14
1412aaebd: 7505 jne 0x1412aaec4
1412aaebf: 4c8b742450 mov r14, qword ptr [rsp + 0x50]
1412aaec4: 4881c794000000 add rdi, 0x94
1412aaecb: 498b06 mov rax, qword ptr [r14]
1412aaece: 4d8b7e10 mov r15, qword ptr [r14 + 0x10]
1412aaed2: 4c29f8 sub rax, r15
1412aaed5: 4883f82f cmp rax, 0x2f
1412aaed9: 0f8667020000 jbe 0x1412ab146
1412aaedf: 498b4608 mov rax, qword ptr [r14 + 8]
1412aaee3: 488d15d625fcff lea rdx, [rip - 0x3da2a]
1412aaeea: 4a891438 mov qword ptr [rax + r15], rdx
1412aaeee: 488d157b71fdff lea rdx, [rip - 0x28e85]
1412aaef5: 4a89543808 mov qword ptr [rax + r15 + 8], rdx
1412aaefa: 4a895c3810 mov qword ptr [rax + r15 + 0x10], rbx
1412aaeff: 4289743818 mov dword ptr [rax + r15 + 0x18], esi
1412aaf04: 488b17 mov rdx, qword ptr [rdi]
1412aaf07: 4a8954381c mov qword ptr [rax + r15 + 0x1c], rdx
1412aaf0c: 8b5708 mov edx, dword ptr [rdi + 8]
1412aaf0f: 4289543824 mov dword ptr [rax + r15 + 0x24], edx
1412aaf14: 42c644382800 mov byte ptr [rax + r15 + 0x28], 0
1412aaf1a: 4983c730 add r15, 0x30
1412aaf1e: 4d897e10 mov qword ptr [r14 + 0x10], r15
1412aaf22: 89f2 mov edx, esi
1412aaf24: 4989d8 mov r8, rbx
1412aaf27: e8b46a56ff call 0x1408119e0 ; system_decorator::decorator_visual_state::impl$4::add_decorator_visual_entity
1412aaf2c: 4889d8 mov rax, rbx
1412aaf2f: 0f28b424c0000000 movaps xmm6, xmmword ptr [rsp + 0xc0]
1412aaf37: 0f28bc24d0000000 movaps xmm7, xmmword ptr [rsp + 0xd0]
1412aaf3f: 440f288424e0000000 movaps xmm8, xmmword ptr [rsp + 0xe0]
1412aaf48: 440f288c24f0000000 movaps xmm9, xmmword ptr [rsp + 0xf0]
1412aaf51: 4881c408010000 add rsp, 0x108
1412aaf58: 5b pop rbx
1412aaf59: 5d pop rbp
1412aaf5a: 5f pop rdi
1412aaf5b: 5e pop rsi
1412aaf5c: 415c pop r12
1412aaf5e: 415d pop r13
1412aaf60: 415e pop r14
1412aaf62: 415f pop r15
1412aaf64: c3 ret 
1412aaf65: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1412aaf6e: 41b828000000 mov r8d, 0x28
1412aaf74: 41b901000000 mov r9d, 1
1412aaf7a: 4c89f1 mov rcx, r14
1412aaf7d: 4c89fa mov rdx, r15
1412aaf80: e89bcf5f01 call 0x1428a7f20 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1412aaf85: e9f0f7ffff jmp 0x1412aa77a ; unknown
1412aaf8a: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1412aaf93: 41b820000000 mov r8d, 0x20
1412aaf99: 41b901000000 mov r9d, 1
1412aaf9f: 4889d9 mov rcx, rbx
1412aafa2: 4c89f2 mov rdx, r14
1412aafa5: e876cf5f01 call 0x1428a7f20 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1412aafaa: e93ef8ffff jmp 0x1412aa7ed ; unknown
1412aafaf: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1412aafb8: 41b858000000 mov r8d, 0x58
1412aafbe: 41b901000000 mov r9d, 1
1412aafc4: 4889d9 mov rcx, rbx
1412aafc7: 4c89f2 mov rdx, r14
1412aafca: e851cf5f01 call 0x1428a7f20 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1412aafcf: e982f8ffff jmp 0x1412aa856 ; unknown
1412aafd4: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1412aafdd: 41b838000000 mov r8d, 0x38
1412aafe3: 41b901000000 mov r9d, 1
1412aafe9: 4889d9 mov rcx, rbx
1412aafec: 4c89f2 mov rdx, r14
1412aafef: e82ccf5f01 call 0x1428a7f20 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1412aaff4: e9f2f8ffff jmp 0x1412aa8eb ; unknown
1412aaff9: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1412ab002: 41b828000000 mov r8d, 0x28
1412ab008: 41b901000000 mov r9d, 1
1412ab00e: 4889d9 mov rcx, rbx
1412ab011: 4c89f2 mov rdx, r14
1412ab014: e807cf5f01 call 0x1428a7f20 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1412ab019: e94af9ffff jmp 0x1412aa968 ; unknown
1412ab01e: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1412ab027: 41b820000000 mov r8d, 0x20
1412ab02d: 41b901000000 mov r9d, 1
1412ab033: 4889d9 mov rcx, rbx
1412ab036: 4c89f2 mov rdx, r14
1412ab039: e8e2ce5f01 call 0x1428a7f20 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1412ab03e: e999f9ffff jmp 0x1412aa9dc ; unknown
1412ab043: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1412ab04c: 41b830000000 mov r8d, 0x30
1412ab052: 41b901000000 mov r9d, 1
1412ab058: 4889d9 mov rcx, rbx
1412ab05b: 4c89f2 mov rdx, r14
1412ab05e: e8bdce5f01 call 0x1428a7f20 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1412ab063: e9d8f9ffff jmp 0x1412aaa40 ; unknown
1412ab068: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1412ab071: 41b838000000 mov r8d, 0x38
1412ab077: 41b901000000 mov r9d, 1
1412ab07d: 4889d9 mov rcx, rbx
1412ab080: 4c89f2 mov rdx, r14
1412ab083: e898ce5f01 call 0x1428a7f20 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1412ab088: e9a9faffff jmp 0x1412aab36 ; unknown
1412ab08d: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1412ab096: 41b830000000 mov r8d, 0x30
1412ab09c: 41b901000000 mov r9d, 1
1412ab0a2: 4889d9 mov rcx, rbx
1412ab0a5: 4c89f2 mov rdx, r14
1412ab0a8: e873ce5f01 call 0x1428a7f20 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1412ab0ad: e914fbffff jmp 0x1412aabc6 ; unknown
1412ab0b2: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1412ab0bb: 41b828000000 mov r8d, 0x28
1412ab0c1: 41b901000000 mov r9d, 1
1412ab0c7: 4889d9 mov rcx, rbx
1412ab0ca: 4c89f2 mov rdx, r14
1412ab0cd: e84ece5f01 call 0x1428a7f20 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1412ab0d2: e96bfbffff jmp 0x1412aac42 ; unknown
1412ab0d7: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1412ab0e0: 41b820000000 mov r8d, 0x20
1412ab0e6: 41b901000000 mov r9d, 1
1412ab0ec: 4c89f1 mov rcx, r14
1412ab0ef: 4c89fa mov rdx, r15
1412ab0f2: e829ce5f01 call 0x1428a7f20 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1412ab0f7: e9b9fbffff jmp 0x1412aacb5 ; unknown
1412ab0fc: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1412ab105: 41b830000000 mov r8d, 0x30
1412ab10b: 41b901000000 mov r9d, 1
1412ab111: 4c89f1 mov rcx, r14
1412ab114: 4c89fa mov rdx, r15
1412ab117: e804ce5f01 call 0x1428a7f20 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1412ab11c: e921fcffff jmp 0x1412aad42 ; unknown
1412ab121: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1412ab12a: 41b840000000 mov r8d, 0x40
1412ab130: 41b901000000 mov r9d, 1
1412ab136: 4c89f1 mov rcx, r14
1412ab139: 4c89fa mov rdx, r15
1412ab13c: e8dfcd5f01 call 0x1428a7f20 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1412ab141: e912fdffff jmp 0x1412aae58 ; unknown
1412ab146: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1412ab14f: 41b830000000 mov r8d, 0x30
1412ab155: 41b901000000 mov r9d, 1
1412ab15b: 4989cc mov r12, rcx
1412ab15e: 4c89f1 mov rcx, r14
1412ab161: 4c89fa mov rdx, r15
1412ab164: e8b7cd5f01 call 0x1428a7f20 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1412ab169: 4c89e1 mov rcx, r12
1412ab16c: e96efdffff jmp 0x1412aaedf ; unknown
1412ab171: 488d0df8fd9701 lea rcx, [rip + 0x197fdf8]
1412ab178: 4c8d0521fe9701 lea r8, [rip + 0x197fe21]
1412ab17f: ba2b000000 mov edx, 0x2b
1412ab184: e8a7e26201 call 0x1428d9430 ; core::panicking::panic
1412ab189: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1412ab192: 41b888000000 mov r8d, 0x88
1412ab198: 41b901000000 mov r9d, 1
1412ab19e: 4889d9 mov rcx, rbx
1412ab1a1: 4c89fa mov rdx, r15
1412ab1a4: e877cd5f01 call 0x1428a7f20 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1412ab1a9: e9ebf0ffff jmp 0x1412aa299 ; unknown
1412ab1ae: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1412ab1b7: 41b888000000 mov r8d, 0x88
1412ab1bd: 41b901000000 mov r9d, 1
1412ab1c3: 4889d9 mov rcx, rbx
1412ab1c6: 4c89fa mov rdx, r15
1412ab1c9: e852cd5f01 call 0x1428a7f20 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1412ab1ce: e994f1ffff jmp 0x1412aa367 ; unknown
1412ab1d3: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1412ab1dc: 41b898000000 mov r8d, 0x98
1412ab1e2: 41b901000000 mov r9d, 1
1412ab1e8: 4889d9 mov rcx, rbx
1412ab1eb: 4c89fa mov rdx, r15
1412ab1ee: e82dcd5f01 call 0x1428a7f20 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1412ab1f3: e92eeeffff jmp 0x1412aa026 ; unknown
1412ab1f8: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1412ab201: 41b898000000 mov r8d, 0x98
1412ab207: 41b901000000 mov r9d, 1
1412ab20d: 4889d9 mov rcx, rbx
1412ab210: 4c89fa mov rdx, r15
1412ab213: e808cd5f01 call 0x1428a7f20 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1412ab218: e91cf4ffff jmp 0x1412aa639 ; unknown
1412ab21d: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1412ab226: 41b888000000 mov r8d, 0x88
1412ab22c: 41b901000000 mov r9d, 1
1412ab232: 4889d9 mov rcx, rbx
1412ab235: 4c89fa mov rdx, r15
1412ab238: e8e3cc5f01 call 0x1428a7f20 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1412ab23d: e91cf3ffff jmp 0x1412aa55e ; unknown
1412ab242: 4c8d054ffc9701 lea r8, [rip + 0x197fc4f]
1412ab249: 4c89f9 mov rcx, r15
1412ab24c: 4c89e2 mov rdx, r12
1412ab24f: e8dce06201 call 0x1428d9330 ; core::option::expect_failed
1412ab254: 488d0d5dfd9701 lea rcx, [rip + 0x197fd5d]
1412ab25b: e8b0e06201 call 0x1428d9310 ; core::option::unwrap_failed
1412ab260: 488d0d69fd9701 lea rcx, [rip + 0x197fd69]
1412ab267: e8a4e06201 call 0x1428d9310 ; core::option::unwrap_failed
1412ab26c: cc int3 
