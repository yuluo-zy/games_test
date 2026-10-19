1407f7cdf: 448a7afc mov r15b, byte ptr [rdx - 4]
1407f7ce3: 488b9520010000 mov rdx, qword ptr [rbp + 0x120]
1407f7cea: b103 mov cl, 3
1407f7cec: e85fb50300 call 0x140833250 ; country_core::resources::walls::decorator::DecoratorRank::new | country_core::systems::collision::world::impl$61::from | country_core::render::nani::mesh_atlas::MeshAtlasPart::new | country_core::LazyTile::from_index | c
1407f7cf1: 488b9d28010000 mov rbx, qword ptr [rbp + 0x128]
1407f7cf8: 4489f9 mov ecx, r15d
1407f7cfb: 89c2 mov edx, eax
1407f7cfd: e88edf3200 call 0x140b25c90 ; country_core::resources::walls::decorator_subtype::DecoratorLibKey::new | rhapsody::rg::recorder::PassResourceAccessType::new
1407f7d02: 888590010000 mov byte ptr [rbp + 0x190], al
1407f7d08: 889591010000 mov byte ptr [rbp + 0x191], dl
1407f7d0e: 4889d9 mov rcx, rbx
1407f7d11: 488d9590010000 lea rdx, [rbp + 0x190]
1407f7d18: e87352a600 call 0x14125cf90 ; std::collections::hash::map::HashMap::get<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,country_core::resources::walls::decorator_subtype::DecoratorCollision,std::hash::random::RandomState,en
1407f7d1d: 4885c0 test rax, rax
1407f7d20: 0f842a090000 je 0x1407f8650
1407f7d26: f30f107830 movss xmm7, dword ptr [rax + 0x30]
1407f7d2b: f3440f10603c movss xmm12, dword ptr [rax + 0x3c]
1407f7d31: f3440f5c6038 subss xmm12, dword ptr [rax + 0x38]
1407f7d37: 440f28c7 movaps xmm8, xmm7
1407f7d3b: f3440f5e05b0691302 divss xmm8, dword ptr [rip + 0x21369b0]
1407f7d44: 488b8dd0000000 mov rcx, qword ptr [rbp + 0xd0]
1407f7d4b: 488d150e452902 lea rdx, [rip + 0x229450e]
1407f7d52: e809b50300 call 0x140833260 ; country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall
1407f7d57: f2440f1030 movsd xmm14, qword ptr [rax]
1407f7d5c: 450f28e8 movaps xmm13, xmm8
1407f7d60: f3450f58e8 addss xmm13, xmm8
1407f7d65: f3440f582d123f2902 addss xmm13, dword ptr [rip + 0x2293f12]
1407f7d6e: 488b8500020000 mov rax, qword ptr [rbp + 0x200]
1407f7d75: 488bb8d8000000 mov rdi, qword ptr [rax + 0xd8]
1407f7d7c: 483bb8c8000000 cmp rdi, qword ptr [rax + 0xc8]
1407f7d83: 751a jne 0x1407f7d9f
1407f7d85: 488b8500020000 mov rax, qword ptr [rbp + 0x200]
1407f7d8c: 488d88c8000000 lea rcx, [rax + 0xc8]
1407f7d93: 488d15de442902 lea rdx, [rip + 0x22944de]
1407f7d9a: e861b8d6ff call 0x140563600 ; alloc::raw_vec::RawVec::grow_one<rhapsody::rg::resource_info::ResourceAccessInfo,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<tuple$<glam::f32::vec2::Vec2,utils::curve::curve_u::CurveU<utils::curve::CurveCoor
1407f7d9f: 488b9500020000 mov rdx, qword ptr [rbp + 0x200]
1407f7da6: 488b82d0000000 mov rax, qword ptr [rdx + 0xd0]
1407f7dad: 488d0cbf lea rcx, [rdi + rdi*4]
1407f7db1: 440f133488 movlps qword ptr [rax + rcx*4], xmm14
1407f7db6: f3440f116c8808 movss dword ptr [rax + rcx*4 + 8], xmm13
1407f7dbd: f3440f1164880c movss dword ptr [rax + rcx*4 + 0xc], xmm12
1407f7dc4: c644881000 mov byte ptr [rax + rcx*4 + 0x10], 0
1407f7dc9: 48ffc7 inc rdi
1407f7dcc: 4889bad8000000 mov qword ptr [rdx + 0xd8], rdi
1407f7dd3: 4c8b8548010000 mov r8, qword ptr [rbp + 0x148]
1407f7dda: 4c8b8d50010000 mov r9, qword ptr [rbp + 0x150]
1407f7de1: f3440f11442458 movss dword ptr [rsp + 0x58], xmm8
1407f7de8: 488d8520010000 lea rax, [rbp + 0x120]
1407f7def: 4889442438 mov qword ptr [rsp + 0x38], rax
1407f7df4: 4889542430 mov qword ptr [rsp + 0x30], rdx
1407f7df9: 8b85c8000000 mov eax, dword ptr [rbp + 0xc8]
1407f7dff: 89442428 mov dword ptr [rsp + 0x28], eax
1407f7e03: 0fb68538010000 movzx eax, byte ptr [rbp + 0x138]
1407f7e0a: 88442420 mov byte ptr [rsp + 0x20], al
1407f7e0e: c744245001000000 mov dword ptr [rsp + 0x50], 1
1407f7e16: c744244800000000 mov dword ptr [rsp + 0x48], 0
1407f7e1e: c744244000000000 mov dword ptr [rsp + 0x40], 0
1407f7e26: 488b8d68010000 mov rcx, qword ptr [rbp + 0x168]
1407f7e2d: 488b95d0000000 mov rdx, qword ptr [rbp + 0xd0]
1407f7e34: e8577bffff call 0x1407ef990 ; system_decorator::decorator_visual::cottage_wall_window::add_lintels
1407f7e39: 488b85b8010000 mov rax, qword ptr [rbp + 0x1b8]
1407f7e40: 83b85802000002 cmp dword ptr [rax + 0x258], 2
1407f7e47: 0f8351040000 jae 0x1407f829e
1407f7e4d: 488b9508020000 mov rdx, qword ptr [rbp + 0x208]
1407f7e54: 8b4ae4 mov ecx, dword ptr [rdx - 0x1c]
1407f7e57: 69c1b6aaaaaa imul eax, ecx, 0xaaaaaab6
1407f7e5d: 054b885655 add eax, 0x5556884b
1407f7e62: 3d55555555 cmp eax, 0x55555555
1407f7e67: 0f871b010000 ja 0x1407f7f88
1407f7e6d: 0fb642fc movzx eax, byte ptr [rdx - 4]
1407f7e71: 83f803 cmp eax, 3
1407f7e74: 0f830e010000 jae 0x1407f7f88
1407f7e7a: 83f801 cmp eax, 1
1407f7e7d: 488b9d68010000 mov rbx, qword ptr [rbp + 0x168]
1407f7e84: 740d je 0x1407f7e93
1407f7e86: 83f802 cmp eax, 2
1407f7e89: 0f855a080000 jne 0x1407f86e9
1407f7e8f: 440f28c7 movaps xmm8, xmm7
1407f7e93: 0fb6056fcde302 movzx eax, byte ptr [rip + 0x2e3cd6f]
1407f7e9a: b950000000 mov ecx, 0x50
1407f7e9f: ba10000000 mov edx, 0x10
1407f7ea4: e867bde1ff call 0x140613c10 ; unknown
1407f7ea9: 4885c0 test rax, rax
1407f7eac: 0f8426080000 je 0x1407f86d8
1407f7eb2: 4889c7 mov rdi, rax
1407f7eb5: 0f280dd43d2902 movaps xmm1, xmmword ptr [rip + 0x2293dd4]
1407f7ebc: 440f59d9 mulps xmm11, xmm1
1407f7ec0: 410f28c2 movaps xmm0, xmm10
1407f7ec4: 0f5905d53d2902 mulps xmm0, xmmword ptr [rip + 0x2293dd5]
1407f7ecb: 410f58c3 addps xmm0, xmm11
1407f7ecf: 0fc6c0d2 shufps xmm0, xmm0, 0xd2
1407f7ed3: 440f59f8 mulps xmm15, xmm0
1407f7ed7: 0f59f1 mulps xmm6, xmm1
1407f7eda: 410f28c2 movaps xmm0, xmm10
1407f7ede: 0f59c1 mulps xmm0, xmm1
1407f7ee1: f30f16c8 movshdup xmm1, xmm0
1407f7ee5: f30f58c8 addss xmm1, xmm0
1407f7ee9: 0f12c0 movhlps xmm0, xmm0
1407f7eec: f30f58c1 addss xmm0, xmm1
1407f7ef0: f30f58c0 addss xmm0, xmm0
1407f7ef4: 0fc6c000 shufps xmm0, xmm0, 0
1407f7ef8: 410f59c2 mulps xmm0, xmm10
1407f7efc: 0f58c6 addps xmm0, xmm6
1407f7eff: 410f58c7 addps xmm0, xmm15
1407f7f03: 0f28c8 movaps xmm1, xmm0
1407f7f06: 660f15c8 unpckhpd xmm1, xmm0
1407f7f0a: f3410f58c9 addss xmm1, xmm9
1407f7f0f: 440f2910 movaps xmmword ptr [rax], xmm10
1407f7f13: 0f5885b0000000 addps xmm0, xmmword ptr [rbp + 0xb0]
1407f7f1a: 0f134010 movlps qword ptr [rax + 0x10], xmm0
1407f7f1e: f30f114818 movss dword ptr [rax + 0x18], xmm1
1407f7f23: 48b80000803f0000803f movabs rax, 0x3f8000003f800000
1407f7f2d: 4889471c mov qword ptr [rdi + 0x1c], rax
1407f7f31: c747240000803f mov dword ptr [rdi + 0x24], 0x3f800000
1407f7f38: 488b85c0000000 mov rax, qword ptr [rbp + 0xc0]
1407f7f3f: 48894730 mov qword ptr [rdi + 0x30], rax
1407f7f43: f3440f114738 movss dword ptr [rdi + 0x38], xmm8
1407f7f49: f3440f11673c movss dword ptr [rdi + 0x3c], xmm12
1407f7f4f: 895f40 mov dword ptr [rdi + 0x40], ebx
1407f7f52: 488b8500020000 mov rax, qword ptr [rbp + 0x200]
1407f7f59: 488b8810010000 mov rcx, qword ptr [rax + 0x110]
1407f7f60: 4885c9 test rcx, rcx
1407f7f63: 7410 je 0x1407f7f75
1407f7f65: ba50000000 mov edx, 0x50
1407f7f6a: 41b810000000 mov r8d, 0x10
1407f7f70: e8abbce1ff call 0x140613c20 ; unknown
1407f7f75: 488b8500020000 mov rax, qword ptr [rbp + 0x200]
1407f7f7c: 4889b810010000 mov qword ptr [rax + 0x110], rdi
1407f7f83: e916030000 jmp 0x1407f829e ; unknown
1407f7f88: e8b3e24900 call 0x140c96240 ; utils::rng::init_rng
1407f7f8d: 4889c7 mov rdi, rax
1407f7f90: 48b82f64bd78641d76a0 movabs rax, 0xa0761d6478bd642f
1407f7f9a: 4801c7 add rdi, rax
1407f7f9d: 4889f8 mov rax, rdi
1407f7fa0: 48b9db28b4a0d17e03e7 movabs rcx, 0xe7037ed1a0b428db
1407f7faa: 4831c8 xor rax, rcx
1407f7fad: 48f7e7 mul rdi
1407f7fb0: 31d0 xor eax, edx
1407f7fb2: c1e81e shr eax, 0x1e
1407f7fb5: 83f801 cmp eax, 1
1407f7fb8: 0f85e0020000 jne 0x1407f829e
1407f7fbe: 488b8508020000 mov rax, qword ptr [rbp + 0x208]
1407f7fc5: 0fb650fc movzx edx, byte ptr [rax - 4]
1407f7fc9: 440f2995c0010000 movaps xmmword ptr [rbp + 0x1c0], xmm10
1407f7fd1: 660f6f85b0000000 movdqa xmm0, xmmword ptr [rbp + 0xb0]
1407f7fd9: 660fd685d0010000 movq qword ptr [rbp + 0x1d0], xmm0
1407f7fe1: f3440f118dd8010000 movss dword ptr [rbp + 0x1d8], xmm9
1407f7fea: 488d85c4010000 lea rax, [rbp + 0x1c4]
1407f7ff1: 48b90000803f0000803f movabs rcx, 0x3f8000003f800000
1407f7ffb: 48894818 mov qword ptr [rax + 0x18], rcx
1407f7fff: c740200000803f mov dword ptr [rax + 0x20], 0x3f800000
1407f8006: 488b4550 mov rax, qword ptr [rbp + 0x50]
1407f800a: 4889442420 mov qword ptr [rsp + 0x20], rax
1407f800f: 488d8d78010000 lea rcx, [rbp + 0x178]
1407f8016: 41b003 mov r8b, 3
1407f8019: 4c8d8dc0010000 lea r9, [rbp + 0x1c0]
1407f8020: e88b77ffff call 0x1407ef7b0 ; system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms
1407f8025: 488b8580010000 mov rax, qword ptr [rbp + 0x180]
1407f802c: 488985b0000000 mov qword ptr [rbp + 0xb0], rax
1407f8033: 488b8588010000 mov rax, qword ptr [rbp + 0x188]
1407f803a: 4885c0 test rax, rax
1407f803d: 0f8435020000 je 0x1407f8278
1407f8043: 488d0440 lea rax, [rax + rax*2]
1407f8047: 48c1e004 shl rax, 4
1407f804b: 4c8bbdb0000000 mov r15, qword ptr [rbp + 0xb0]
1407f8052: 4c01f8 add rax, r15
1407f8055: 488985c8000000 mov qword ptr [rbp + 0xc8], rax
1407f805c: 488b8500020000 mov rax, qword ptr [rbp + 0x200]
1407f8063: 4805f8000000 add rax, 0xf8
1407f8069: 488985d0000000 mov qword ptr [rbp + 0xd0], rax
1407f8070: 4531c9 xor r9d, r9d
1407f8073: e995000000 jmp 0x1407f810d ; unknown
1407f8078: 0f1f840000000000 nop dword ptr [rax + rax]
1407f8080: 4983c730 add r15, 0x30
1407f8084: 4c8b8db8010000 mov r9, qword ptr [rbp + 0x1b8]
1407f808b: 49ffc1 inc r9
1407f808e: 4c8b8500020000 mov r8, qword ptr [rbp + 0x200]
1407f8095: 498b8000010000 mov rax, qword ptr [r8 + 0x100]
1407f809c: 488d0c9b lea rcx, [rbx + rbx*4]
1407f80a0: 48c1e104 shl rcx, 4
1407f80a4: c6040801 mov byte ptr [rax + rcx], 1
1407f80a8: 488b9538010000 mov rdx, qword ptr [rbp + 0x138]
1407f80af: 4889540808 mov qword ptr [rax + rcx + 8], rdx
1407f80b4: 660f6f85e0000000 movdqa xmm0, xmmword ptr [rbp + 0xe0]
1407f80bc: 660f6f8df0000000 movdqa xmm1, xmmword ptr [rbp + 0xf0]
1407f80c4: 660f6f9500010000 movdqa xmm2, xmmword ptr [rbp + 0x100]
1407f80cc: 660f7f440810 movdqa xmmword ptr [rax + rcx + 0x10], xmm0
1407f80d2: 660f7f4c0820 movdqa xmmword ptr [rax + rcx + 0x20], xmm1
1407f80d8: 660f7f540830 movdqa xmmword ptr [rax + rcx + 0x30], xmm2
1407f80de: 488b9508020000 mov rdx, qword ptr [rbp + 0x208]
1407f80e5: 4889540840 mov qword ptr [rax + rcx + 0x40], rdx
1407f80ea: 488b95c0000000 mov rdx, qword ptr [rbp + 0xc0]
1407f80f1: 4889540848 mov qword ptr [rax + rcx + 0x48], rdx
1407f80f6: 48ffc3 inc rbx
1407f80f9: 49899808010000 mov qword ptr [r8 + 0x108], rbx
1407f8100: 4c3bbdc8000000 cmp r15, qword ptr [rbp + 0xc8]
1407f8107: 0f846b010000 je 0x1407f8278
1407f810d: 66410f6f07 movdqa xmm0, xmmword ptr [r15]
1407f8112: 66410f6f4f10 movdqa xmm1, xmmword ptr [r15 + 0x10]
1407f8118: 66410f6f5720 movdqa xmm2, xmmword ptr [r15 + 0x20]
1407f811e: 660f7f95e0010000 movdqa xmmword ptr [rbp + 0x1e0], xmm2
1407f8126: 660f7f8dd0010000 movdqa xmmword ptr [rbp + 0x1d0], xmm1
1407f812e: 660f7f85c0010000 movdqa xmmword ptr [rbp + 0x1c0], xmm0
1407f8136: 488b8568010000 mov rax, qword ptr [rbp + 0x168]
1407f813d: 8985a0010000 mov dword ptr [rbp + 0x1a0], eax
1407f8143: 488d0546412902 lea rax, [rip + 0x2294146]
1407f814a: 48898590010000 mov qword ptr [rbp + 0x190], rax
1407f8151: 48c785980100000a000000 mov qword ptr [rbp + 0x198], 0xa
1407f815c: 4c898db8010000 mov qword ptr [rbp + 0x1b8], r9
1407f8163: 4c898da8010000 mov qword ptr [rbp + 0x1a8], r9
1407f816a: 488d8d90010000 lea rcx, [rbp + 0x190]
1407f8171: e8fa48a900 call 0x14128ca70 ; utils::calculate_hash<tuple$<country_core::resources::walls::decorator::DecoratorId,ref$<str$>,usize> >
1407f8176: 48898508020000 mov qword ptr [rbp + 0x208], rax
1407f817d: 48b82f64bd78641d76a0 movabs rax, 0xa0761d6478bd642f
1407f8187: 4c8d0407 lea r8, [rdi + rax]
1407f818b: 4c89c0 mov rax, r8
1407f818e: 48b9db28b4a0d17e03e7 movabs rcx, 0xe7037ed1a0b428db
1407f8198: 4831c8 xor rax, rcx
1407f819b: 49f7e0 mul r8
1407f819e: 4889d1 mov rcx, rdx
1407f81a1: 4831c1 xor rcx, rax
1407f81a4: 4889c8 mov rax, rcx
1407f81a7: ba06000000 mov edx, 6
1407f81ac: 48f7e2 mul rdx
1407f81af: 4801c9 add rcx, rcx
1407f81b2: 488d0449 lea rax, [rcx + rcx*2]
1407f81b6: 4883f803 cmp rax, 3
1407f81ba: 774f ja 0x1407f820b
1407f81bc: 48b85ec87af1c83aec40 movabs rax, 0x40ec3ac8f17ac85e
1407f81c6: 4801c7 add rdi, rax
1407f81c9: 48b92f64bd78641d76a0 movabs rcx, 0xa0761d6478bd642f
1407f81d3: 49b9db28b4a0d17e03e7 movabs r9, 0xe7037ed1a0b428db
1407f81dd: 0f1f00 nop dword ptr [rax]
1407f81e0: 4889f8 mov rax, rdi
1407f81e3: 4c31c8 xor rax, r9
1407f81e6: 48f7e7 mul rdi
1407f81e9: 4901c8 add r8, rcx
1407f81ec: 4831c2 xor rdx, rax
1407f81ef: 488d0412 lea rax, [rdx + rdx]
1407f81f3: 488d0440 lea rax, [rax + rax*2]
1407f81f7: 4801cf add rdi, rcx
1407f81fa: 4883f804 cmp rax, 4
1407f81fe: 72e0 jb 0x1407f81e0
1407f8200: 4889d0 mov rax, rdx
1407f8203: b906000000 mov ecx, 6
1407f8208: 48f7e1 mul rcx
1407f820b: 4c89c7 mov rdi, r8
1407f820e: 48899538010000 mov qword ptr [rbp + 0x138], rdx
1407f8215: 660f6f85c0010000 movdqa xmm0, xmmword ptr [rbp + 0x1c0]
1407f821d: 660f6f8dd0010000 movdqa xmm1, xmmword ptr [rbp + 0x1d0]
1407f8225: 660f6f95e0010000 movdqa xmm2, xmmword ptr [rbp + 0x1e0]
1407f822d: 660f7f9500010000 movdqa xmmword ptr [rbp + 0x100], xmm2
1407f8235: 660f7f8df0000000 movdqa xmmword ptr [rbp + 0xf0], xmm1
1407f823d: 660f7f85e0000000 movdqa xmmword ptr [rbp + 0xe0], xmm0
1407f8245: 488b8500020000 mov rax, qword ptr [rbp + 0x200]
1407f824c: 488b9808010000 mov rbx, qword ptr [rax + 0x108]
1407f8253: 483b98f8000000 cmp rbx, qword ptr [rax + 0xf8]
1407f825a: 0f8520feffff jne 0x1407f8080
1407f8260: 488b8dd0000000 mov rcx, qword ptr [rbp + 0xd0]
1407f8267: 488d1532402902 lea rdx, [rip + 0x2294032]
1407f826e: e88d47f9ff call 0x14078ca00 ; alloc::raw_vec::RawVec::grow_one<tuple$<glam::f32::affine3a::Affine3A,u32,array$<f32,2> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc
1407f8273: e908feffff jmp 0x1407f8080 ; unknown
1407f8278: 488b8578010000 mov rax, qword ptr [rbp + 0x178]
1407f827f: 4885c0 test rax, rax
1407f8282: 741a je 0x1407f829e
1407f8284: 48c1e004 shl rax, 4
1407f8288: 488d1440 lea rdx, [rax + rax*2]
1407f828c: 41b810000000 mov r8d, 0x10
1407f8292: 488b8db0000000 mov rcx, qword ptr [rbp + 0xb0]
1407f8299: e882b9e1ff call 0x140613c20 ; unknown
