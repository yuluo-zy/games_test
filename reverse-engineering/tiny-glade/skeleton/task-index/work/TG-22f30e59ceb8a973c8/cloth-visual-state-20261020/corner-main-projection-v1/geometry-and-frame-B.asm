1407f6c1a: 488b4d30 mov rcx, qword ptr [rbp + 0x30]
1407f6c1e: 488b95c0000000 mov rdx, qword ptr [rbp + 0xc0]
1407f6c25: 4c8d050c542902 lea r8, [rip + 0x229540c]
1407f6c2c: e8bfb22900 call 0x140a91ef0 ; country_core::resources::walls::public_walls::PublicWalls::get
1407f6c31: 488985b8010000 mov qword ptr [rbp + 0x1b8], rax
1407f6c38: 4883bdb801000000 cmp qword ptr [rbp + 0x1b8], 0
1407f6c40: 0f8441190000 je 0x1407f8587
1407f6c46: 498d1c3f lea rbx, [r15 + rdi]
1407f6c4a: 0fb643ff movzx eax, byte ptr [rbx - 1]
1407f6c4e: 888538010000 mov byte ptr [rbp + 0x138], al
1407f6c54: 8b43e0 mov eax, dword ptr [rbx - 0x20]
1407f6c57: 8985c8000000 mov dword ptr [rbp + 0xc8], eax
1407f6c5d: 488d4de0 lea rcx, [rbp - 0x20]
1407f6c61: 488b5538 mov rdx, qword ptr [rbp + 0x38]
1407f6c65: 4c8b8568010000 mov r8, qword ptr [rbp + 0x168]
1407f6c6c: 4c8d0df5532902 lea r9, [rip + 0x22953f5]
1407f6c73: e8a83d2200 call 0x140a1aa20 ; country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::transform
1407f6c78: 4c89b588000000 mov qword ptr [rbp + 0x88], r14
1407f6c7f: 48899d08020000 mov qword ptr [rbp + 0x208], rbx
1407f6c86: 440fb673fc movzx r14d, byte ptr [rbx - 4]
1407f6c8b: 4d85f6 test r14, r14
1407f6c8e: 7428 je 0x1407f6cb8
1407f6c90: 4489f0 mov eax, r14d
1407f6c93: c1e004 shl eax, 4
1407f6c96: 488d1c40 lea rbx, [rax + rax*2]
1407f6c9a: 0fb60568dfe302 movzx eax, byte ptr [rip + 0x2e3df68]
1407f6ca1: ba10000000 mov edx, 0x10
1407f6ca6: 4889d9 mov rcx, rbx
1407f6ca9: e862cfe1ff call 0x140613c10 ; unknown
1407f6cae: 4885c0 test rax, rax
1407f6cb1: 750a jne 0x1407f6cbd
1407f6cb3: e9e0180000 jmp 0x1407f8598 ; unknown
1407f6cb8: b810000000 mov eax, 0x10
1407f6cbd: 4c89b540010000 mov qword ptr [rbp + 0x140], r14
1407f6cc4: 48898548010000 mov qword ptr [rbp + 0x148], rax
1407f6ccb: 48c7855001000000000000 mov qword ptr [rbp + 0x150], 0
1407f6cd6: 488d8dc0010000 lea rcx, [rbp + 0x1c0]
1407f6cdd: 488b95b8010000 mov rdx, qword ptr [rbp + 0x1b8]
1407f6ce4: e8a73a3300 call 0x140b2a790 ; country_core::resources::walls::public_wall_state::PublicWallState::is_rectangle
1407f6ce9: 4c8bb588000000 mov r14, qword ptr [rbp + 0x88]
1407f6cf0: 498d143f lea rdx, [r15 + rdi]
1407f6cf4: 4883c2b8 add rdx, -0x48
1407f6cf8: 83bdc001000001 cmp dword ptr [rbp + 0x1c0], 1
1407f6cff: 488995d0000000 mov qword ptr [rbp + 0xd0], rdx
1407f6d06: 0f85bb080000 jne 0x1407f75c7
1407f6d0c: 488d8dc4010000 lea rcx, [rbp + 0x1c4]
1407f6d13: 488b4110 mov rax, qword ptr [rcx + 0x10]
1407f6d17: 488985a0010000 mov qword ptr [rbp + 0x1a0], rax
1407f6d1e: 0f1001 movups xmm0, xmmword ptr [rcx]
1407f6d21: 0f298590010000 movaps xmmword ptr [rbp + 0x190], xmm0
1407f6d28: 488b8508020000 mov rax, qword ptr [rbp + 0x208]
1407f6d2f: 8078da02 cmp byte ptr [rax - 0x26], 2
1407f6d33: 0f8331180000 jae 0x1407f856a
1407f6d39: 0fb648d8 movzx ecx, byte ptr [rax - 0x28]
1407f6d3d: e8aee93200 call 0x140b256f0 ; country_core::resources::walls::wall_attachment::WallCornerAttachment::to_rectangle_corner_id
1407f6d42: a801 test al, 1
1407f6d44: 0f84f5180000 je 0x1407f863f
1407f6d4a: 4889d3 mov rbx, rdx
1407f6d4d: 488d8d90010000 lea rcx, [rbp + 0x190]
1407f6d54: e8e7334a00 call 0x140c9a140 ; utils::geometry::rectangle::Rectangle2d::get_corner_normal
1407f6d59: 0f28f8 movaps xmm7, xmm0
1407f6d5c: 660f6ff1 movdqa xmm6, xmm1
1407f6d60: 488d8dec000000 lea rcx, [rbp + 0xec]
1407f6d67: 488d9590010000 lea rdx, [rbp + 0x190]
1407f6d6e: 4989d8 mov r8, rbx
1407f6d71: e89a344a00 call 0x140c9a210 ; utils::geometry::rectangle::Rectangle2d::get_edge_dirs_from_corner_id
1407f6d76: f2440f10bdec000000 movsd xmm15, qword ptr [rbp + 0xec]
1407f6d7f: f2440f109df4000000 movsd xmm11, qword ptr [rbp + 0xf4]
1407f6d88: f30f11bd80000000 movss dword ptr [rbp + 0x80], xmm7
1407f6d90: f30f11b584000000 movss dword ptr [rbp + 0x84], xmm6
1407f6d98: 488b8508020000 mov rax, qword ptr [rbp + 0x208]
1407f6d9f: 448a78fc mov r15b, byte ptr [rax - 4]
1407f6da3: 488b9520010000 mov rdx, qword ptr [rbp + 0x120]
1407f6daa: b103 mov cl, 3
1407f6dac: e89fc40300 call 0x140833250 ; country_core::resources::walls::decorator::DecoratorRank::new | country_core::systems::collision::world::impl$61::from | country_core::render::nani::mesh_atlas::MeshAtlasPart::new | country_core::LazyTile::from_index | c
1407f6db1: 488b9d28010000 mov rbx, qword ptr [rbp + 0x128]
1407f6db8: 4489f9 mov ecx, r15d
1407f6dbb: 89c2 mov edx, eax
1407f6dbd: e8ceee3200 call 0x140b25c90 ; country_core::resources::walls::decorator_subtype::DecoratorLibKey::new | rhapsody::rg::recorder::PassResourceAccessType::new
1407f6dc2: 888590010000 mov byte ptr [rbp + 0x190], al
1407f6dc8: 889591010000 mov byte ptr [rbp + 0x191], dl
1407f6dce: 4889d9 mov rcx, rbx
1407f6dd1: 488d9590010000 lea rdx, [rbp + 0x190]
1407f6dd8: e8b361a600 call 0x14125cf90 ; std::collections::hash::map::HashMap::get<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,country_core::resources::walls::decorator_subtype::DecoratorCollision,std::hash::random::RandomState,en
1407f6ddd: 4885c0 test rax, rax
1407f6de0: 0f84cb170000 je 0x1407f85b1
1407f6de6: f30f104030 movss xmm0, dword ptr [rax + 0x30]
1407f6deb: f30f5e0501791302 divss xmm0, dword ptr [rip + 0x2137901]
1407f6df3: 0f284df0 movaps xmm1, xmmword ptr [rbp - 0x10]
1407f6df7: 660f6e75f4 movd xmm6, dword ptr [rbp - 0xc]
1407f6dfc: f3440f12e0 movsldup xmm12, xmm0
1407f6e01: 450f28c7 movaps xmm8, xmm15
1407f6e05: 450f59c4 mulps xmm8, xmm12
1407f6e09: f3440f106df8 movss xmm13, dword ptr [rbp - 8]
1407f6e0f: 440f16e9 movlhps xmm13, xmm1
1407f6e13: 440fc6e9e2 shufps xmm13, xmm1, 0xe2
1407f6e18: 450f58c5 addps xmm8, xmm13
1407f6e1c: 450f59e3 mulps xmm12, xmm11
1407f6e20: 450f58e5 addps xmm12, xmm13
1407f6e24: 410f28c0 movaps xmm0, xmm8
1407f6e28: 0f2815b1ea1202 movaps xmm2, xmmword ptr [rip + 0x212eab1]
1407f6e2f: 0f59c2 mulps xmm0, xmm2
1407f6e32: 410f28cc movaps xmm1, xmm12
1407f6e36: 0f59ca mulps xmm1, xmm2
1407f6e39: 0f58c8 addps xmm1, xmm0
1407f6e3c: 0f138d78010000 movlps qword ptr [rbp + 0x178], xmm1
1407f6e43: 488d8d90010000 lea rcx, [rbp + 0x190]
1407f6e4a: 488d9578010000 lea rdx, [rbp + 0x178]
1407f6e51: 660f6fd6 movdqa xmm2, xmm6
1407f6e55: e8865f4a00 call 0x140c9cde0 ; utils::swizzlers::impl$1::xvy
1407f6e5a: 488d8de0000000 lea rcx, [rbp + 0xe0]
1407f6e61: 488d9580000000 lea rdx, [rbp + 0x80]
1407f6e68: e8535f4a00 call 0x140c9cdc0 ; utils::swizzlers::impl$0::x0y
1407f6e6d: 488d8dc0010000 lea rcx, [rbp + 0x1c0]
1407f6e74: 488d95e0000000 lea rdx, [rbp + 0xe0]
1407f6e7b: e850a34900 call 0x140c911d0 ; utils::xz_projective_rotation_bad
1407f6e80: 488b8508020000 mov rax, qword ptr [rbp + 0x208]
1407f6e87: 0fb640fc movzx eax, byte ptr [rax - 4]
1407f6e8b: 48ffc8 dec rax
1407f6e8e: 4883f803 cmp rax, 3
1407f6e92: 0f8344160000 jae 0x1407f84dc
1407f6e98: 440f2895c0010000 movaps xmm10, xmmword ptr [rbp + 0x1c0]
1407f6ea0: f20f10bd90010000 movsd xmm7, qword ptr [rbp + 0x190]
1407f6ea8: f3440f108d98010000 movss xmm9, dword ptr [rbp + 0x198]
1407f6eb1: 440f592d874d2902 mulps xmm13, xmmword ptr [rip + 0x2294d87]
1407f6eb9: 440f59058f4d2902 mulps xmm8, xmmword ptr [rip + 0x2294d8f]
1407f6ec1: 450f58c5 addps xmm8, xmm13
1407f6ec5: 440f138510010000 movlps qword ptr [rbp + 0x110], xmm8
1407f6ecd: 488d8d90010000 lea rcx, [rbp + 0x190]
1407f6ed4: 488d9510010000 lea rdx, [rbp + 0x110]
1407f6edb: 660f6fd6 movdqa xmm2, xmm6
1407f6edf: e8fc5e4a00 call 0x140c9cde0 ; utils::swizzlers::impl$1::xvy
1407f6ee4: f3410f16c7 movshdup xmm0, xmm15
1407f6ee9: 0f570500ea1202 xorps xmm0, xmmword ptr [rip + 0x212ea00]
1407f6ef0: f30f118578010000 movss dword ptr [rbp + 0x178], xmm0
1407f6ef8: f3440f11bd7c010000 movss dword ptr [rbp + 0x17c], xmm15
1407f6f01: 488d8de0000000 lea rcx, [rbp + 0xe0]
1407f6f08: 488d9578010000 lea rdx, [rbp + 0x178]
1407f6f0f: e8ac5e4a00 call 0x140c9cdc0 ; utils::swizzlers::impl$0::x0y
1407f6f14: 488d8dc0010000 lea rcx, [rbp + 0x1c0]
1407f6f1b: 488d95e0000000 lea rdx, [rbp + 0xe0]
1407f6f22: e8a9a24900 call 0x140c911d0 ; utils::xz_projective_rotation_bad
1407f6f27: 440f2885c0010000 movaps xmm8, xmmword ptr [rbp + 0x1c0]
1407f6f2f: 488b8590010000 mov rax, qword ptr [rbp + 0x190]
1407f6f36: 488985c0010000 mov qword ptr [rbp + 0x1c0], rax
1407f6f3d: 8b8598010000 mov eax, dword ptr [rbp + 0x198]
1407f6f43: 8985c8010000 mov dword ptr [rbp + 0x1c8], eax
1407f6f49: 488bbd50010000 mov rdi, qword ptr [rbp + 0x150]
1407f6f50: 483bbd40010000 cmp rdi, qword ptr [rbp + 0x140]
1407f6f57: 7513 jne 0x1407f6f6c
1407f6f59: 488d8d40010000 lea rcx, [rbp + 0x140]
1407f6f60: 488d1571512902 lea rdx, [rip + 0x2295171]
1407f6f67: e804450300 call 0x14082b470 ; alloc::raw_vec::RawVec::grow_one<utils::components::transform::Transform,alloc::alloc::Global>
1407f6f6c: 488b8548010000 mov rax, qword ptr [rbp + 0x148]
1407f6f73: 488d0c7f lea rcx, [rdi + rdi*2]
1407f6f77: 48c1e104 shl rcx, 4
1407f6f7b: 440f290408 movaps xmmword ptr [rax + rcx], xmm8
1407f6f80: 488b95c0010000 mov rdx, qword ptr [rbp + 0x1c0]
1407f6f87: 4889540810 mov qword ptr [rax + rcx + 0x10], rdx
1407f6f8c: 8b95c8010000 mov edx, dword ptr [rbp + 0x1c8]
1407f6f92: 89540818 mov dword ptr [rax + rcx + 0x18], edx
1407f6f96: 48ba0000803f0000803f movabs rdx, 0x3f8000003f800000
1407f6fa0: 488954081c mov qword ptr [rax + rcx + 0x1c], rdx
1407f6fa5: c74408240000803f mov dword ptr [rax + rcx + 0x24], 0x3f800000
1407f6fad: 48ffc7 inc rdi
1407f6fb0: 4889bd50010000 mov qword ptr [rbp + 0x150], rdi
1407f6fb7: 440f5925914c2902 mulps xmm12, xmmword ptr [rip + 0x2294c91]
1407f6fbf: 450f58ec addps xmm13, xmm12
1407f6fc3: 440f13ad10010000 movlps qword ptr [rbp + 0x110], xmm13
1407f6fcb: 488d8d90010000 lea rcx, [rbp + 0x190]
1407f6fd2: 488d9510010000 lea rdx, [rbp + 0x110]
1407f6fd9: 660f6fd6 movdqa xmm2, xmm6
1407f6fdd: e8fe5d4a00 call 0x140c9cde0 ; utils::swizzlers::impl$1::xvy
1407f6fe2: f3410f16c3 movshdup xmm0, xmm11
1407f6fe7: 0f570502e91202 xorps xmm0, xmmword ptr [rip + 0x212e902]
1407f6fee: f30f118578010000 movss dword ptr [rbp + 0x178], xmm0
1407f6ff6: f3440f119d7c010000 movss dword ptr [rbp + 0x17c], xmm11
1407f6fff: 488d8de0000000 lea rcx, [rbp + 0xe0]
1407f7006: 488d9578010000 lea rdx, [rbp + 0x178]
1407f700d: e8ae5d4a00 call 0x140c9cdc0 ; utils::swizzlers::impl$0::x0y
1407f7012: 488d8dc0010000 lea rcx, [rbp + 0x1c0]
1407f7019: 488d95e0000000 lea rdx, [rbp + 0xe0]
1407f7020: e8aba14900 call 0x140c911d0 ; utils::xz_projective_rotation_bad
1407f7025: 0f28b5c0010000 movaps xmm6, xmmword ptr [rbp + 0x1c0]
1407f702c: 488b8590010000 mov rax, qword ptr [rbp + 0x190]
1407f7033: 488985c0010000 mov qword ptr [rbp + 0x1c0], rax
1407f703a: 8b8598010000 mov eax, dword ptr [rbp + 0x198]
1407f7040: 8985c8010000 mov dword ptr [rbp + 0x1c8], eax
1407f7046: 488bbd50010000 mov rdi, qword ptr [rbp + 0x150]
1407f704d: 483bbd40010000 cmp rdi, qword ptr [rbp + 0x140]
1407f7054: 7513 jne 0x1407f7069
1407f7056: 488d8d40010000 lea rcx, [rbp + 0x140]
1407f705d: 488d158c502902 lea rdx, [rip + 0x229508c]
1407f7064: e807440300 call 0x14082b470 ; alloc::raw_vec::RawVec::grow_one<utils::components::transform::Transform,alloc::alloc::Global>
1407f7069: 488b8548010000 mov rax, qword ptr [rbp + 0x148]
1407f7070: 488d0c7f lea rcx, [rdi + rdi*2]
1407f7074: 48c1e104 shl rcx, 4
1407f7078: 0f293408 movaps xmmword ptr [rax + rcx], xmm6
1407f707c: 488b95c0010000 mov rdx, qword ptr [rbp + 0x1c0]
1407f7083: 4889540810 mov qword ptr [rax + rcx + 0x10], rdx
1407f7088: 8b95c8010000 mov edx, dword ptr [rbp + 0x1c8]
1407f708e: 89540818 mov dword ptr [rax + rcx + 0x18], edx
1407f7092: 48ba0000803f0000803f movabs rdx, 0x3f8000003f800000
1407f709c: 488954081c mov qword ptr [rax + rcx + 0x1c], rdx
1407f70a1: c74408240000803f mov dword ptr [rax + rcx + 0x24], 0x3f800000
1407f70a9: 48ffc7 inc rdi
1407f70ac: 4889bd50010000 mov qword ptr [rbp + 0x150], rdi
1407f70b3: 8b85a4000000 mov eax, dword ptr [rbp + 0xa4]
1407f70b9: 488b4d40 mov rcx, qword ptr [rbp + 0x40]
1407f70bd: 8901 mov dword ptr [rcx], eax
1407f70bf: 488b8dc0000000 mov rcx, qword ptr [rbp + 0xc0]
1407f70c6: e855540c00 call 0x1408bc520 ; country_core::resources::rt::impl$5::from | country_core::resources::walls::decorator_storage::impl$15::from
1407f70cb: 4989d1 mov r9, rdx
1407f70ce: 488b8d08020000 mov rcx, qword ptr [rbp + 0x208]
1407f70d5: 0fb649ff movzx ecx, byte ptr [rcx - 1]
1407f70d9: 884c2420 mov byte ptr [rsp + 0x20], cl
1407f70dd: 488b4d48 mov rcx, qword ptr [rbp + 0x48]
1407f70e1: 488b9568010000 mov rdx, qword ptr [rbp + 0x168]
1407f70e8: 4989c0 mov r8, rax
1407f70eb: e8c0e70200 call 0x1408258b0 ; system_decorator::blueprint::DecoratorBlueprints::insert
1407f70f0: 48898500020000 mov qword ptr [rbp + 0x200], rax
1407f70f7: 488b8508020000 mov rax, qword ptr [rbp + 0x208]
1407f70fe: 0fb640fc movzx eax, byte ptr [rax - 4]
1407f7102: fec8 dec al
1407f7104: 3c03 cmp al, 3
1407f7106: 0f8316150000 jae 0x1407f8622
1407f710c: 0fb6c0 movzx eax, al
1407f710f: 488d0d32522902 lea rcx, [rip + 0x2295232]
1407f7116: 4c633c81 movsxd r15, dword ptr [rcx + rax*4]
1407f711a: 450f28c2 movaps xmm8, xmm10
1407f711e: 440f59054a4b2902 mulps xmm8, xmmword ptr [rip + 0x2294b4a]
1407f7126: 488b9500020000 mov rdx, qword ptr [rbp + 0x200]
1407f712d: 488d82b0000000 lea rax, [rdx + 0xb0]
1407f7134: 48898598000000 mov qword ptr [rbp + 0x98], rax
1407f713b: 488d85c4010000 lea rax, [rbp + 0x1c4]
1407f7142: c7400700000000 mov dword ptr [rax + 7], 0
1407f7149: 48c740ff00000000 mov qword ptr [rax - 1], 0
1407f7151: 488b85c0010000 mov rax, qword ptr [rbp + 0x1c0]
1407f7158: 488b8dc7010000 mov rcx, qword ptr [rbp + 0x1c7]
1407f715f: 48898d7f010000 mov qword ptr [rbp + 0x17f], rcx
1407f7166: 48898578010000 mov qword ptr [rbp + 0x178], rax
1407f716d: 488b9ac0000000 mov rbx, qword ptr [rdx + 0xc0]
1407f7174: 483b9ab0000000 cmp rbx, qword ptr [rdx + 0xb0]
1407f717b: 7513 jne 0x1407f7190
1407f717d: 488b8d98000000 mov rcx, qword ptr [rbp + 0x98]
1407f7184: 488d150d502902 lea rdx, [rip + 0x229500d]
1407f718b: e820420300 call 0x14082b3b0 ; alloc::raw_vec::RawVec::grow_one<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder,alloc::alloc::Global>
1407f7190: 488d05b1512902 lea rax, [rip + 0x22951b1]
1407f7197: 4901c7 add r15, rax
1407f719a: 410f28f2 movaps xmm6, xmm10
1407f719e: 410f59f2 mulps xmm6, xmm10
1407f71a2: f30f16c6 movshdup xmm0, xmm6
1407f71a6: f30f58c6 addss xmm0, xmm6
1407f71aa: 0f28ce movaps xmm1, xmm6
1407f71ad: 660f15ce unpckhpd xmm1, xmm6
1407f71b1: f30f58c8 addss xmm1, xmm0
1407f71b5: 0fc6c900 shufps xmm1, xmm1, 0
1407f71b9: 0fc6f6ff shufps xmm6, xmm6, 0xff
1407f71bd: 0f5cf1 subps xmm6, xmm1
1407f71c0: 0f28c6 movaps xmm0, xmm6
1407f71c3: 0f281d964a2902 movaps xmm3, xmmword ptr [rip + 0x2294a96]
1407f71ca: 0f59c3 mulps xmm0, xmm3
1407f71cd: 410f28ca movaps xmm1, xmm10
1407f71d1: 0f59cb mulps xmm1, xmm3
1407f71d4: f30f16d1 movshdup xmm2, xmm1
1407f71d8: f30f58d1 addss xmm2, xmm1
1407f71dc: 0f12c9 movhlps xmm1, xmm1
1407f71df: f30f58ca addss xmm1, xmm2
1407f71e3: f30f58c9 addss xmm1, xmm1
1407f71e7: 0fc6c900 shufps xmm1, xmm1, 0
1407f71eb: 410f59ca mulps xmm1, xmm10
1407f71ef: 0f58c8 addps xmm1, xmm0
1407f71f2: 450f28da movaps xmm11, xmm10
1407f71f6: 450fc6dad2 shufps xmm11, xmm10, 0xd2
1407f71fb: 410f28c3 movaps xmm0, xmm11
1407f71ff: 0f59c3 mulps xmm0, xmm3
1407f7202: 410f58c0 addps xmm0, xmm8
1407f7206: 0fc6c0d2 shufps xmm0, xmm0, 0xd2
1407f720a: 450f28fa movaps xmm15, xmm10
1407f720e: 450f58fa addps xmm15, xmm10
1407f7212: 450fc6ffff shufps xmm15, xmm15, 0xff
1407f7217: 410f59c7 mulps xmm0, xmm15
1407f721b: 0f58c1 addps xmm0, xmm1
1407f721e: 0f58f8 addps xmm7, xmm0
1407f7221: 0f12c0 movhlps xmm0, xmm0
1407f7224: f3440f58c8 addss xmm9, xmm0
1407f7229: 4c8b9500020000 mov r10, qword ptr [rbp + 0x200]
1407f7230: 498b82b8000000 mov rax, qword ptr [r10 + 0xb8]
1407f7237: 4869cb60010000 imul rcx, rbx, 0x160
1407f723e: 48c7040800000000 mov qword ptr [rax + rcx], 0
1407f7246: 48c744081000000000 mov qword ptr [rax + rcx + 0x10], 0
1407f724f: 48c744082000000000 mov qword ptr [rax + rcx + 0x20], 0
1407f7258: c744083000000000 mov dword ptr [rax + rcx + 0x30], 0
1407f7260: 488b555c mov rdx, qword ptr [rbp + 0x5c]
1407f7264: 4889540834 mov qword ptr [rax + rcx + 0x34], rdx
1407f7269: 8b5564 mov edx, dword ptr [rbp + 0x64]
1407f726c: 8954083c mov dword ptr [rax + rcx + 0x3c], edx
1407f7270: c744084000000000 mov dword ptr [rax + rcx + 0x40], 0
1407f7278: 0f1085c0010000 movups xmm0, xmmword ptr [rbp + 0x1c0]
1407f727f: 0f108dd0010000 movups xmm1, xmmword ptr [rbp + 0x1d0]
1407f7286: f30f6f95e0010000 movdqu xmm2, xmmword ptr [rbp + 0x1e0]
1407f728e: 0f109dec010000 movups xmm3, xmmword ptr [rbp + 0x1ec]
1407f7295: 0f11440844 movups xmmword ptr [rax + rcx + 0x44], xmm0
1407f729a: 0f114c0854 movups xmmword ptr [rax + rcx + 0x54], xmm1
1407f729f: f30f7f540864 movdqu xmmword ptr [rax + rcx + 0x64], xmm2
1407f72a5: 0f115c0870 movups xmmword ptr [rax + rcx + 0x70], xmm3
1407f72aa: c784088000000000000000 mov dword ptr [rax + rcx + 0x80], 0
1407f72b5: 488b5568 mov rdx, qword ptr [rbp + 0x68]
1407f72b9: 4889940884000000 mov qword ptr [rax + rcx + 0x84], rdx
1407f72c1: 8b5570 mov edx, dword ptr [rbp + 0x70]
1407f72c4: 8994088c000000 mov dword ptr [rax + rcx + 0x8c], edx
1407f72cb: c784089000000000000000 mov dword ptr [rax + rcx + 0x90], 0
1407f72d6: 488b5574 mov rdx, qword ptr [rbp + 0x74]
1407f72da: 4889940894000000 mov qword ptr [rax + rcx + 0x94], rdx
1407f72e2: 8b557c mov edx, dword ptr [rbp + 0x7c]
1407f72e5: 8994089c000000 mov dword ptr [rax + rcx + 0x9c], edx
1407f72ec: 4c89bc08a0000000 mov qword ptr [rax + rcx + 0xa0], r15
1407f72f4: 48c78408a800000024000000 mov qword ptr [rax + rcx + 0xa8], 0x24
1407f7300: 488d156d4e2902 lea rdx, [rip + 0x2294e6d]
1407f7307: 48899408b0000000 mov qword ptr [rax + rcx + 0xb0], rdx
1407f730f: 48c78408b80000001e000000 mov qword ptr [rax + rcx + 0xb8], 0x1e
1407f731b: 440f299408c0000000 movaps xmmword ptr [rax + rcx + 0xc0], xmm10
1407f7324: 0f13bc08d0000000 movlps qword ptr [rax + rcx + 0xd0], xmm7
1407f732c: f3440f118c08d8000000 movss dword ptr [rax + rcx + 0xd8], xmm9
1407f7336: 49b90000803f0000803f movabs r9, 0x3f8000003f800000
1407f7340: 4c898c08dc000000 mov qword ptr [rax + rcx + 0xdc], r9
1407f7348: c78408e40000000000803f mov dword ptr [rax + rcx + 0xe4], 0x3f800000
1407f7353: c78408f000000001000000 mov dword ptr [rax + rcx + 0xf0], 1
1407f735e: 8b95c8000000 mov edx, dword ptr [rbp + 0xc8]
1407f7364: 899408f4000000 mov dword ptr [rax + rcx + 0xf4], edx
1407f736b: c78408f800000000000000 mov dword ptr [rax + rcx + 0xf8], 0
1407f7376: 0f108590010000 movups xmm0, xmmword ptr [rbp + 0x190]
1407f737d: 0f118408fc000000 movups xmmword ptr [rax + rcx + 0xfc], xmm0
1407f7385: 488b95a0010000 mov rdx, qword ptr [rbp + 0x1a0]
1407f738c: 488994080c010000 mov qword ptr [rax + rcx + 0x10c], rdx
1407f7394: c684081401000002 mov byte ptr [rax + rcx + 0x114], 2
1407f739c: 0fb695b7010000 movzx edx, byte ptr [rbp + 0x1b7]
1407f73a3: 88940817010000 mov byte ptr [rax + rcx + 0x117], dl
1407f73aa: 0fb795b5010000 movzx edx, word ptr [rbp + 0x1b5]
1407f73b1: 6689940815010000 mov word ptr [rax + rcx + 0x115], dx
1407f73b9: c784081801000000000000 mov dword ptr [rax + rcx + 0x118], 0
1407f73c4: f30f6f85e0000000 movdqu xmm0, xmmword ptr [rbp + 0xe0]
1407f73cc: f30f6f8df0000000 movdqu xmm1, xmmword ptr [rbp + 0xf0]
1407f73d4: f30f7f8c082c010000 movdqu xmmword ptr [rax + rcx + 0x12c], xmm1
1407f73dd: f30f7f84081c010000 movdqu xmmword ptr [rax + rcx + 0x11c], xmm0
1407f73e6: c684083c01000002 mov byte ptr [rax + rcx + 0x13c], 2
1407f73ee: 488b9578010000 mov rdx, qword ptr [rbp + 0x178]
1407f73f5: 4c8b857f010000 mov r8, qword ptr [rbp + 0x17f]
1407f73fc: 4c89840844010000 mov qword ptr [rax + rcx + 0x144], r8
1407f7404: 488994083d010000 mov qword ptr [rax + rcx + 0x13d], rdx
1407f740c: c684084c01000002 mov byte ptr [rax + rcx + 0x14c], 2
1407f7414: 0fb69538010000 movzx edx, byte ptr [rbp + 0x138]
1407f741b: 8894084e010000 mov byte ptr [rax + rcx + 0x14e], dl
1407f7422: 66c784084f0100000009 mov word ptr [rax + rcx + 0x14f], 0x900
1407f742c: 488b9510010000 mov rdx, qword ptr [rbp + 0x110]
1407f7433: 4c8b8517010000 mov r8, qword ptr [rbp + 0x117]
1407f743a: 4889940851010000 mov qword ptr [rax + rcx + 0x151], rdx
1407f7442: 4c89840858010000 mov qword ptr [rax + rcx + 0x158], r8
1407f744a: 48ffc3 inc rbx
1407f744d: 49899ac0000000 mov qword ptr [r10 + 0xc0], rbx
1407f7454: 498b02 mov rax, qword ptr [r10]
1407f7457: 490b4208 or rax, qword ptr [r10 + 8]
1407f745b: 0f85a4100000 jne 0x1407f8505
1407f7461: 49c7420800000000 mov qword ptr [r10 + 8], 0
1407f7469: 49c70201000000 mov qword ptr [r10], 1
1407f7470: 450f295210 movaps xmmword ptr [r10 + 0x10], xmm10
1407f7475: 0f29bdb0000000 movaps xmmword ptr [rbp + 0xb0], xmm7
1407f747c: 410f137a20 movlps qword ptr [r10 + 0x20], xmm7
1407f7481: f3450f114a28 movss dword ptr [r10 + 0x28], xmm9
1407f7487: 4d894a2c mov qword ptr [r10 + 0x2c], r9
1407f748b: 41c742340000803f mov dword ptr [r10 + 0x34], 0x3f800000
1407f7493: 488b8508020000 mov rax, qword ptr [rbp + 0x208]
1407f749a: 8a58fc mov bl, byte ptr [rax - 4]
1407f749d: 488b9520010000 mov rdx, qword ptr [rbp + 0x120]
1407f74a4: b103 mov cl, 3
1407f74a6: e8a5bd0300 call 0x140833250 ; country_core::resources::walls::decorator::DecoratorRank::new | country_core::systems::collision::world::impl$61::from | country_core::render::nani::mesh_atlas::MeshAtlasPart::new | country_core::LazyTile::from_index | c
1407f74ab: 488bbd28010000 mov rdi, qword ptr [rbp + 0x128]
1407f74b2: 89d9 mov ecx, ebx
1407f74b4: 89c2 mov edx, eax
1407f74b6: e8d5e73200 call 0x140b25c90 ; country_core::resources::walls::decorator_subtype::DecoratorLibKey::new | rhapsody::rg::recorder::PassResourceAccessType::new
1407f74bb: 888590010000 mov byte ptr [rbp + 0x190], al
