00000001415b23b0: push rbp
00000001415b23b1: push r15
00000001415b23b3: push r14
00000001415b23b5: push r13
00000001415b23b7: push r12
00000001415b23b9: push rsi
00000001415b23ba: push rdi
00000001415b23bb: push rbx
00000001415b23bc: sub rsp, 0x1b8
00000001415b23c3: lea rbp, [rsp + 0x80]
00000001415b23cb: movaps xmmword ptr [rbp + 0x120], xmm9
00000001415b23d3: movaps xmmword ptr [rbp + 0x110], xmm8
00000001415b23db: movaps xmmword ptr [rbp + 0x100], xmm7
00000001415b23e2: movaps xmmword ptr [rbp + 0xf0], xmm6
00000001415b23e9: mov qword ptr [rbp + 0xe8], 0xfffffffffffffffe
00000001415b23f4: mov rsi, rcx
00000001415b23f7: mov r14, qword ptr [rdx + 0x10]
00000001415b23fb: lea rcx, [rbp + 0x90]
00000001415b2402: call 0x140b88080 ; serde_json::value::de::SeqDeserializer::new
00000001415b2407: mov rax, qword ptr [rbp + 0x98]
00000001415b240e: cmp rax, qword ptr [rbp + 0xa8]
00000001415b2415: je 0x1415b242e
00000001415b2417: lea rcx, [rax + 0x20]
00000001415b241b: mov qword ptr [rbp + 0x98], rcx
00000001415b2422: movzx ecx, byte ptr [rax]
00000001415b2425: cmp cl, 6
00000001415b2428: jne 0x1415b24c8
00000001415b242e: lea rdx, [rip + 0x16c8cd3] ; 142c7b108 e5b0c742010000002100000000000000637572766568656967687469735f656e636c6f736564617263685f686569676874617263685f74797374796c657374727563747572655f7479636c7574746572 b'\xe5\xb0\xc7B\x01\x00\x00\x00!\x00\x00\x00\x00\x00\x00\x00curveheightis_enclosedarch_heightarch_tystylestructure_tyclutter'
00000001415b2435: lea r8, [rip + 0x16c7c24] ; 142c7a060 00000000000000001000000000000000080000000000000070a96a4001000000666577657220656c656d656e747320696e206d617000000080a0c7420100000015000000000000000000000000000000 b'\x00\x00\x00\x00\x00\x00\x00\x00\x10\x00\x00\x00\x00\x00\x00\x00\x08\x00\x00\x00\x00\x00\x00\x00p\xa9j@\x01\x00\x00\x00fewer elements in map\x00\x00\x00\x80\xa0\xc7B\x01\x00\x00\x00\x15\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00'
00000001415b243c: xor ecx, ecx
00000001415b243e: call 0x1428f9d20 ; serde::de::Error::invalid_length<serde_json::error::Error>
00000001415b2443: mov rdi, rax
00000001415b2446: mov qword ptr [rsi + 8], rdi
00000001415b244a: movabs rax, 0x8000000000000000
00000001415b2454: mov qword ptr [rsi], rax
00000001415b2457: mov rcx, qword ptr [rbp + 0x98]
00000001415b245e: mov rdx, qword ptr [rbp + 0xa8]
00000001415b2465: sub rdx, rcx
00000001415b2468: shr rdx, 5
00000001415b246c: call 0x140888d70 ; core::ptr::drop_in_place<slice2$<enum2$<serde_json::value::Value> > >
00000001415b2471: mov rdx, qword ptr [rbp + 0xa0]
00000001415b2478: test rdx, rdx
00000001415b247b: je 0x1415b2493
00000001415b247d: mov rcx, qword ptr [rbp + 0x90]
00000001415b2484: shl rdx, 5
00000001415b2488: mov r8d, 8
00000001415b248e: call 0x140613c20 ; 140613c20
00000001415b2493: mov rax, rsi
00000001415b2496: movaps xmm6, xmmword ptr [rbp + 0xf0]
00000001415b249d: movaps xmm7, xmmword ptr [rbp + 0x100]
00000001415b24a4: movaps xmm8, xmmword ptr [rbp + 0x110]
00000001415b24ac: movaps xmm9, xmmword ptr [rbp + 0x120]
00000001415b24b4: add rsp, 0x1b8
00000001415b24bb: pop rbx
00000001415b24bc: pop rdi
00000001415b24bd: pop rsi
00000001415b24be: pop r12
00000001415b24c0: pop r13
00000001415b24c2: pop r14
00000001415b24c4: pop r15
00000001415b24c6: pop rbp
00000001415b24c7: ret 
00000001415b24c8: mov byte ptr [rbp + 0x70], cl
00000001415b24cb: movups xmm0, xmmword ptr [rax + 1]
00000001415b24cf: movups xmm1, xmmword ptr [rax + 0x10]
00000001415b24d3: movups xmmword ptr [rbp + 0x71], xmm0
00000001415b24d7: movups xmmword ptr [rbp + 0x80], xmm1
00000001415b24de: lea rcx, [rbp + 0xb0]
00000001415b24e5: lea rdx, [rbp + 0x70]
00000001415b24e9: call 0x141285770 ; utils::curve::curve2_serialized_as_curve3::deserialize<enum2$<serde_json::value::Value> >
00000001415b24ee: mov rbx, qword ptr [rbp + 0xb0]
00000001415b24f5: mov rdi, qword ptr [rbp + 0xb8]
00000001415b24fc: mov rax, rbx
00000001415b24ff: neg rax
00000001415b2502: jo 0x1415b2446
00000001415b2508: mov rax, qword ptr [rbp + 0xe0]
00000001415b250f: mov qword ptr [rbp - 0x18], rax
00000001415b2513: movups xmm0, xmmword ptr [rbp + 0xc0]
00000001415b251a: movups xmm1, xmmword ptr [rbp + 0xd0]
00000001415b2521: movups xmmword ptr [rbp - 0x28], xmm1
00000001415b2525: movups xmmword ptr [rbp - 0x38], xmm0
00000001415b2529: mov qword ptr [rbp - 0x48], rbx
00000001415b252d: mov qword ptr [rbp - 0x40], rdi
00000001415b2531: mov rax, qword ptr [rbp + 0x98]
00000001415b2538: cmp rax, qword ptr [rbp + 0xa8]
00000001415b253f: je 0x1415b2554
00000001415b2541: lea rcx, [rax + 0x20]
00000001415b2545: mov qword ptr [rbp + 0x98], rcx
00000001415b254c: movzx ecx, byte ptr [rax]
00000001415b254f: cmp cl, 6
00000001415b2552: jne 0x1415b25ad
00000001415b2554: lea rdx, [rip + 0x16c8bad] ; 142c7b108 e5b0c742010000002100000000000000637572766568656967687469735f656e636c6f736564617263685f686569676874617263685f74797374796c657374727563747572655f7479636c7574746572 b'\xe5\xb0\xc7B\x01\x00\x00\x00!\x00\x00\x00\x00\x00\x00\x00curveheightis_enclosedarch_heightarch_tystylestructure_tyclutter'
00000001415b255b: lea r8, [rip + 0x16c7afe] ; 142c7a060 00000000000000001000000000000000080000000000000070a96a4001000000666577657220656c656d656e747320696e206d617000000080a0c7420100000015000000000000000000000000000000 b'\x00\x00\x00\x00\x00\x00\x00\x00\x10\x00\x00\x00\x00\x00\x00\x00\x08\x00\x00\x00\x00\x00\x00\x00p\xa9j@\x01\x00\x00\x00fewer elements in map\x00\x00\x00\x80\xa0\xc7B\x01\x00\x00\x00\x15\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00'
00000001415b2562: mov ecx, 1
00000001415b2567: call 0x1428f9d20 ; serde::de::Error::invalid_length<serde_json::error::Error>
00000001415b256c: mov r14, rax
00000001415b256f: test rbx, rbx
00000001415b2572: je 0x1415b2589
00000001415b2574: shl rbx, 3
00000001415b2578: mov r8d, 4
00000001415b257e: mov rcx, rdi
00000001415b2581: mov rdx, rbx
00000001415b2584: call 0x140613c20 ; 140613c20
00000001415b2589: mov rdx, qword ptr [rbp - 0x30]
00000001415b258d: test rdx, rdx
00000001415b2590: je 0x1415b25a5
00000001415b2592: mov rcx, qword ptr [rbp - 0x28]
00000001415b2596: shl rdx, 2
00000001415b259a: mov r8d, 4
00000001415b25a0: call 0x140613c20 ; 140613c20
00000001415b25a5: mov rdi, r14
00000001415b25a8: jmp 0x1415b2446
00000001415b25ad: mov byte ptr [rbp + 0xb0], cl
00000001415b25b3: movups xmm0, xmmword ptr [rax + 1]
00000001415b25b7: movups xmm1, xmmword ptr [rax + 0x10]
00000001415b25bb: movups xmmword ptr [rbp + 0xb1], xmm0
00000001415b25c2: movups xmmword ptr [rbp + 0xc0], xmm1
00000001415b25c9: lea rcx, [rbp + 0x70]
00000001415b25cd: lea rdx, [rbp + 0xb0]
00000001415b25d4: call 0x14159a370 ; serde_json::value::de::impl$2::deserialize_f32<serde::de::impls::impl$76::deserialize::PrimitiveVisitor>
00000001415b25d9: test byte ptr [rbp + 0x70], 1
00000001415b25dd: je 0x1415b25e5
00000001415b25df: mov r14, qword ptr [rbp + 0x78]
00000001415b25e3: jmp 0x1415b256f
00000001415b25e5: mov rax, qword ptr [rbp + 0x98]
00000001415b25ec: cmp rax, qword ptr [rbp + 0xa8]
00000001415b25f3: je 0x1415b260d
00000001415b25f5: movss xmm6, dword ptr [rbp + 0x74]
00000001415b25fa: lea rcx, [rax + 0x20]
00000001415b25fe: mov qword ptr [rbp + 0x98], rcx
00000001415b2605: movzx ecx, byte ptr [rax]
00000001415b2608: cmp cl, 6
00000001415b260b: jne 0x1415b262a
00000001415b260d: lea rdx, [rip + 0x16c8af4] ; 142c7b108 e5b0c742010000002100000000000000637572766568656967687469735f656e636c6f736564617263685f686569676874617263685f74797374796c657374727563747572655f7479636c7574746572 b'\xe5\xb0\xc7B\x01\x00\x00\x00!\x00\x00\x00\x00\x00\x00\x00curveheightis_enclosedarch_heightarch_tystylestructure_tyclutter'
00000001415b2614: lea r8, [rip + 0x16c7a45] ; 142c7a060 00000000000000001000000000000000080000000000000070a96a4001000000666577657220656c656d656e747320696e206d617000000080a0c7420100000015000000000000000000000000000000 b'\x00\x00\x00\x00\x00\x00\x00\x00\x10\x00\x00\x00\x00\x00\x00\x00\x08\x00\x00\x00\x00\x00\x00\x00p\xa9j@\x01\x00\x00\x00fewer elements in map\x00\x00\x00\x80\xa0\xc7B\x01\x00\x00\x00\x15\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00'
00000001415b261b: mov ecx, 2
00000001415b2620: call 0x1428f9d20 ; serde::de::Error::invalid_length<serde_json::error::Error>
00000001415b2625: jmp 0x1415b256c
00000001415b262a: mov byte ptr [rbp + 0x70], cl
00000001415b262d: movups xmm0, xmmword ptr [rax + 1]
00000001415b2631: movups xmm1, xmmword ptr [rax + 0x10]
00000001415b2635: movups xmmword ptr [rbp + 0x71], xmm0
00000001415b2639: movups xmmword ptr [rbp + 0x80], xmm1
00000001415b2640: lea rcx, [rbp + 0xb0]
00000001415b2647: lea rdx, [rbp + 0x70]
00000001415b264b: call 0x1416ebaf0 ; country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
00000001415b2650: test byte ptr [rbp + 0xb0], 1
00000001415b2657: je 0x1415b2665
00000001415b2659: mov r14, qword ptr [rbp + 0xb8]
00000001415b2660: jmp 0x1415b256f
00000001415b2665: mov r15d, dword ptr [rbp + 0xb4]
00000001415b266c: mov r12, qword ptr [rbp + 0xb8]
00000001415b2673: movups xmm0, xmmword ptr [rbp + 0xc0]
00000001415b267a: movaps xmmword ptr [rbp - 0x60], xmm0
00000001415b267e: cmp r15d, 4
00000001415b2682: je 0x1415b260d
00000001415b2684: mov rax, qword ptr [rbp + 0x98]
00000001415b268b: cmp rax, qword ptr [rbp + 0xa8]
00000001415b2692: je 0x1415b26a7
00000001415b2694: lea rcx, [rax + 0x20]
00000001415b2698: mov qword ptr [rbp + 0x98], rcx
00000001415b269f: movzx ecx, byte ptr [rax]
00000001415b26a2: cmp cl, 6
00000001415b26a5: jne 0x1415b26da
00000001415b26a7: xorps xmm7, xmm7
00000001415b26aa: lea rcx, [rbp + 0xb0]
00000001415b26b1: lea rdx, [rbp + 0x90]
00000001415b26b8: call 0x1415d0e30 ; serde::de::impl$6::next_element<serde_json::value::de::SeqDeserializer,f32>
00000001415b26bd: test byte ptr [rbp + 0xb0], 1
00000001415b26c4: jne 0x1415b2659
00000001415b26c6: cmp byte ptr [rbp + 0xb4], 0
00000001415b26cd: jne 0x1415b2717
00000001415b26cf: movss xmm8, dword ptr [rip + 0x14da588] ; 142a8cc60 333333409a9999bf000080c0000000000000803ecdcc4c3d00000000000000000000003f000000bf000000000000000000000000000000000100000000000000010000000000000070f67f4001000000 b'333@\x9a\x99\x99\xbf\x00\x00\x80\xc0\x00\x00\x00\x00\x00\x00\x80>\xcd\xccL=\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00?\x00\x00\x00\xbf\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00p\xf6\x7f@\x01\x00\x00\x00'
00000001415b26d8: jmp 0x1415b2720
00000001415b26da: mov byte ptr [rbp + 0xb0], cl
00000001415b26e0: movups xmm0, xmmword ptr [rax + 1]
00000001415b26e4: movups xmm1, xmmword ptr [rax + 0x10]
00000001415b26e8: movups xmmword ptr [rbp + 0xb1], xmm0
00000001415b26ef: movups xmmword ptr [rbp + 0xc0], xmm1
00000001415b26f6: lea rcx, [rbp + 0x70]
00000001415b26fa: lea rdx, [rbp + 0xb0]
00000001415b2701: call 0x14159a370 ; serde_json::value::de::impl$2::deserialize_f32<serde::de::impls::impl$76::deserialize::PrimitiveVisitor>
00000001415b2706: test byte ptr [rbp + 0x70], 1
00000001415b270a: jne 0x1415b25df
00000001415b2710: movss xmm7, dword ptr [rbp + 0x74]
00000001415b2715: jmp 0x1415b26aa
00000001415b2717: movss xmm8, dword ptr [rbp + 0xb8]
00000001415b2720: lea rcx, [rbp + 0xb0]
00000001415b2727: lea rdx, [rbp + 0x90]
00000001415b272e: call 0x1415d0a80 ; serde::de::impl$6::next_element<serde_json::value::de::SeqDeserializer,country_core::resources::walls::inner_wall_state::ArchType>
00000001415b2733: cmp byte ptr [rbp + 0xb0], 0
00000001415b273a: jne 0x1415b2659
00000001415b2740: movzx eax, byte ptr [rbp + 0xb1]
00000001415b2747: xor r13d, r13d
00000001415b274a: cmp al, 2
00000001415b274c: cmovne r13d, eax
00000001415b2750: lea rcx, [rbp + 0xb0]
00000001415b2757: lea rdx, [rbp + 0x90]
00000001415b275e: call 0x1415d0f40 ; serde::de::impl$6::next_element<serde_json::value::de::SeqDeserializer,country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$2::visit_seq::__DeserializeWith>
00000001415b2763: cmp byte ptr [rbp + 0xb0], 0
00000001415b276a: jne 0x1415b2659
00000001415b2770: movzx eax, byte ptr [rbp + 0xb1]
00000001415b2777: xor ecx, ecx
00000001415b2779: cmp al, 3
00000001415b277b: cmovne ecx, eax
00000001415b277e: mov dword ptr [rbp + 0x60], ecx
00000001415b2781: lea rcx, [rbp + 0xb0]
00000001415b2788: lea rdx, [rbp + 0x90]
00000001415b278f: call 0x1415d0ca0 ; serde::de::impl$6::next_element<serde_json::value::de::SeqDeserializer,country_core::resources::walls::inner_wall_state::StructureType>
00000001415b2794: cmp byte ptr [rbp + 0xb0], 0
00000001415b279b: jne 0x1415b2659
00000001415b27a1: movzx eax, byte ptr [rbp + 0xb1]
00000001415b27a8: xor ecx, ecx
00000001415b27aa: cmp al, 2
00000001415b27ac: cmovne ecx, eax
00000001415b27af: mov dword ptr [rbp + 0x64], ecx
00000001415b27b2: lea rcx, [rbp + 0xb0]
00000001415b27b9: lea rdx, [rbp + 0x90]
00000001415b27c0: call 0x1415d0d20 ; serde::de::impl$6::next_element<serde_json::value::de::SeqDeserializer,country_core::resources::walls::decorator::ClutterVersion>
00000001415b27c5: cmp byte ptr [rbp + 0xb0], 0
00000001415b27cc: jne 0x1415b2659
00000001415b27d2: movzx eax, byte ptr [rbp + 0xb2]
00000001415b27d9: xor ecx, ecx
00000001415b27db: cmp byte ptr [rbp + 0xb1], 0
00000001415b27e2: cmovne ecx, eax
00000001415b27e5: mov dword ptr [rbp + 0x68], ecx
00000001415b27e8: lea rcx, [rbp + 0xb0]
00000001415b27ef: lea rdx, [rbp + 0x90]
00000001415b27f6: call 0x1415d0a80 ; serde::de::impl$6::next_element<serde_json::value::de::SeqDeserializer,country_core::resources::walls::inner_wall_state::ArchType>
00000001415b27fb: cmp byte ptr [rbp + 0xb0], 0
00000001415b2802: jne 0x1415b2659
00000001415b2808: movzx eax, byte ptr [rbp + 0xb1]
00000001415b280f: xor ecx, ecx
00000001415b2811: cmp al, 2
00000001415b2813: cmovne ecx, eax
00000001415b2816: mov dword ptr [rbp + 0x6c], ecx
00000001415b2819: lea rcx, [rbp + 0xb0]
00000001415b2820: lea rdx, [rbp + 0x90]
00000001415b2827: call 0x1415d10c0 ; serde::de::impl$6::next_element<serde_json::value::de::SeqDeserializer,country_core::resources::walls::inner_wall_state::PlasterPeelAmount>
00000001415b282c: test byte ptr [rbp + 0xb0], 1
00000001415b2833: jne 0x1415b2659
00000001415b2839: cmp byte ptr [rbp + 0xb4], 0
00000001415b2840: jne 0x1415b2848
00000001415b2842: xorps xmm9, xmm9
00000001415b2846: jmp 0x1415b2851
00000001415b2848: movss xmm9, dword ptr [rbp + 0xb8]
00000001415b2851: lea rcx, [rbp + 0xb0]
00000001415b2858: lea rdx, [rbp + 0x90]
00000001415b285f: call 0x1415d0c20 ; serde::de::impl$6::next_element<serde_json::value::de::SeqDeserializer,bool>
00000001415b2864: cmp byte ptr [rbp + 0xb0], 1
00000001415b286b: je 0x1415b2659
00000001415b2871: lea rax, [rbp - 0x38]
00000001415b2875: movzx ecx, byte ptr [rbp + 0xb1]
00000001415b287c: movups xmm0, xmmword ptr [rax]
00000001415b287f: movups xmm1, xmmword ptr [rax + 0x10]
00000001415b2883: movups xmmword ptr [rbp], xmm0
00000001415b2887: movups xmmword ptr [rbp + 0x10], xmm1
00000001415b288b: mov rax, qword ptr [rax + 0x20]
00000001415b288f: mov qword ptr [rbp + 0x20], rax
00000001415b2893: movaps xmm0, xmmword ptr [rbp - 0x60]
00000001415b2897: movups xmmword ptr [rbp + 0x34], xmm0
00000001415b289b: xor eax, eax
00000001415b289d: cmp cl, 2
00000001415b28a0: cmovne eax, ecx
00000001415b28a3: mov qword ptr [rbp - 0x10], rbx
00000001415b28a7: mov qword ptr [rbp - 8], rdi
00000001415b28ab: mov dword ptr [rbp + 0x28], r15d
00000001415b28af: mov qword ptr [rbp + 0x2c], r12
00000001415b28b3: movss dword ptr [rbp + 0x44], xmm6
00000001415b28b8: movss dword ptr [rbp + 0x48], xmm7
00000001415b28bd: movss dword ptr [rbp + 0x4c], xmm8
00000001415b28c3: movss dword ptr [rbp + 0x50], xmm9
00000001415b28c9: mov byte ptr [rbp + 0x54], r13b
00000001415b28cd: mov ecx, dword ptr [rbp + 0x64]
00000001415b28d0: mov byte ptr [rbp + 0x55], cl
00000001415b28d3: mov ecx, dword ptr [rbp + 0x6c]
00000001415b28d6: mov byte ptr [rbp + 0x56], cl
00000001415b28d9: mov byte ptr [rbp + 0x57], al
00000001415b28dc: mov eax, dword ptr [rbp + 0x60]
00000001415b28df: mov byte ptr [rbp + 0x58], al
00000001415b28e2: mov eax, dword ptr [rbp + 0x68]
00000001415b28e5: mov byte ptr [rbp + 0x59], al
00000001415b28e8: mov rax, qword ptr [rbp + 0xa8]
00000001415b28ef: cmp rax, qword ptr [rbp + 0x98]
00000001415b28f6: jne 0x1415b2976
00000001415b28f8: mov eax, dword ptr [rbp + 0x50]
00000001415b28fb: mov dword ptr [rsi + 0x60], eax
00000001415b28fe: movzx eax, byte ptr [rbp + 0x54]
00000001415b2902: mov byte ptr [rsi + 0x64], al
00000001415b2905: movzx eax, byte ptr [rbp + 0x55]
00000001415b2909: mov byte ptr [rsi + 0x65], al
00000001415b290c: movzx eax, byte ptr [rbp + 0x56]
00000001415b2910: mov byte ptr [rsi + 0x66], al
00000001415b2913: movzx eax, byte ptr [rbp + 0x57]
00000001415b2917: mov byte ptr [rsi + 0x67], al
00000001415b291a: movzx eax, byte ptr [rbp + 0x58]
00000001415b291e: mov byte ptr [rsi + 0x68], al
00000001415b2921: movzx eax, byte ptr [rbp + 0x59]
00000001415b2925: mov byte ptr [rsi + 0x69], al
00000001415b2928: mov eax, dword ptr [rbp + 0x5a]
00000001415b292b: mov dword ptr [rsi + 0x6a], eax
00000001415b292e: movzx eax, word ptr [rbp + 0x5e]
00000001415b2932: mov word ptr [rsi + 0x6e], ax
00000001415b2936: movups xmm0, xmmword ptr [rbp + 0x40]
00000001415b293a: movups xmmword ptr [rsi + 0x50], xmm0
00000001415b293e: movups xmm0, xmmword ptr [rbp + 0x30]
00000001415b2942: movups xmmword ptr [rsi + 0x40], xmm0
00000001415b2946: movups xmm0, xmmword ptr [rbp - 0x10]
00000001415b294a: movups xmm1, xmmword ptr [rbp]
00000001415b294e: movups xmm2, xmmword ptr [rbp + 0x10]
00000001415b2952: movups xmm3, xmmword ptr [rbp + 0x20]
00000001415b2956: movups xmmword ptr [rsi + 0x30], xmm3
00000001415b295a: movups xmmword ptr [rsi + 0x20], xmm2
00000001415b295e: movups xmmword ptr [rsi + 0x10], xmm1
00000001415b2962: movups xmmword ptr [rsi], xmm0
00000001415b2965: lea rcx, [rbp + 0x90]
00000001415b296c: call 0x14088a560 ; core::ptr::drop_in_place<serde_json::value::de::SeqDeserializer>
00000001415b2971: jmp 0x1415b2493
00000001415b2976: lea rdx, [rip + 0x16c76d3] ; 142c7a050 38a0c74201000000170000000000000000000000000000001000000000000000080000000000000070a96a4001000000666577657220656c656d656e747320696e206d617000000080a0c74201000000 b'8\xa0\xc7B\x01\x00\x00\x00\x17\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x10\x00\x00\x00\x00\x00\x00\x00\x08\x00\x00\x00\x00\x00\x00\x00p\xa9j@\x01\x00\x00\x00fewer elements in map\x00\x00\x00\x80\xa0\xc7B\x01\x00\x00\x00'
00000001415b297d: lea r8, [rip + 0x16c76dc] ; 142c7a060 00000000000000001000000000000000080000000000000070a96a4001000000666577657220656c656d656e747320696e206d617000000080a0c7420100000015000000000000000000000000000000 b'\x00\x00\x00\x00\x00\x00\x00\x00\x10\x00\x00\x00\x00\x00\x00\x00\x08\x00\x00\x00\x00\x00\x00\x00p\xa9j@\x01\x00\x00\x00fewer elements in map\x00\x00\x00\x80\xa0\xc7B\x01\x00\x00\x00\x15\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00'
00000001415b2984: mov rcx, r14
00000001415b2987: call 0x1428f9d20 ; serde::de::Error::invalid_length<serde_json::error::Error>
00000001415b298c: mov qword ptr [rsi + 8], rax
00000001415b2990: movabs rax, 0x8000000000000000
00000001415b299a: mov qword ptr [rsi], rax
00000001415b299d: lea rcx, [rbp - 0x10]
00000001415b29a1: call 0x1405d24f0 ; core::ptr::drop_in_place<utils::curve::Curve<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D> >
00000001415b29a6: jmp 0x1415b2965
00000001415b29a8: nop dword ptr [rax + rax]
00000001415b29b0: mov qword ptr [rsp + 0x10], rdx
00000001415b29b5: push rbp
00000001415b29b6: push r15
00000001415b29b8: push r14
00000001415b29ba: push r13
00000001415b29bc: push r12
00000001415b29be: push rsi
00000001415b29bf: push rdi
00000001415b29c0: push rbx
00000001415b29c1: sub rsp, 0x68
00000001415b29c5: lea rbp, [rdx + 0x80]
00000001415b29cc: movaps xmmword ptr [rsp + 0x20], xmm9
00000001415b29d2: movaps xmmword ptr [rsp + 0x30], xmm8
00000001415b29d8: movaps xmmword ptr [rsp + 0x40], xmm7
00000001415b29dd: movaps xmmword ptr [rsp + 0x50], xmm6
00000001415b29e2: lea rcx, [rbp - 0x10]
00000001415b29e6: call 0x1405d24f0 ; core::ptr::drop_in_place<utils::curve::Curve<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D> >
00000001415b29eb: movaps xmm6, xmmword ptr [rsp + 0x50]
00000001415b29f0: movaps xmm7, xmmword ptr [rsp + 0x40]
00000001415b29f5: movaps xmm8, xmmword ptr [rsp + 0x30]
00000001415b29fb: movaps xmm9, xmmword ptr [rsp + 0x20]
00000001415b2a01: add rsp, 0x68
00000001415b2a05: pop rbx
00000001415b2a06: pop rdi
00000001415b2a07: pop rsi
00000001415b2a08: pop r12
00000001415b2a0a: pop r13
00000001415b2a0c: pop r14
00000001415b2a0e: pop r15
00000001415b2a10: pop rbp
00000001415b2a11: ret 
00000001415b2a12: nop word ptr cs:[rax + rax]
00000001415b2a20: mov qword ptr [rsp + 0x10], rdx
00000001415b2a25: push rbp
00000001415b2a26: push r15
00000001415b2a28: push r14
00000001415b2a2a: push r13
00000001415b2a2c: push r12
00000001415b2a2e: push rsi
00000001415b2a2f: push rdi
00000001415b2a30: push rbx
00000001415b2a31: sub rsp, 0x68
00000001415b2a35: lea rbp, [rdx + 0x80]
00000001415b2a3c: movaps xmmword ptr [rsp + 0x20], xmm9
00000001415b2a42: movaps xmmword ptr [rsp + 0x30], xmm8
00000001415b2a48: movaps xmmword ptr [rsp + 0x40], xmm7
00000001415b2a4d: movaps xmmword ptr [rsp + 0x50], xmm6
00000001415b2a52: lea rcx, [rbp - 0x48]
00000001415b2a56: call 0x1405d24f0 ; core::ptr::drop_in_place<utils::curve::Curve<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D> >
00000001415b2a5b: movaps xmm6, xmmword ptr [rsp + 0x50]
00000001415b2a60: movaps xmm7, xmmword ptr [rsp + 0x40]
00000001415b2a65: movaps xmm8, xmmword ptr [rsp + 0x30]
00000001415b2a6b: movaps xmm9, xmmword ptr [rsp + 0x20]
00000001415b2a71: add rsp, 0x68
00000001415b2a75: pop rbx
00000001415b2a76: pop rdi
00000001415b2a77: pop rsi
00000001415b2a78: pop r12
00000001415b2a7a: pop r13
00000001415b2a7c: pop r14
00000001415b2a7e: pop r15
00000001415b2a80: pop rbp
00000001415b2a81: ret 
00000001415b2a82: nop word ptr cs:[rax + rax]
00000001415b2a90: mov qword ptr [rsp + 0x10], rdx
00000001415b2a95: push rbp
00000001415b2a96: push r15
00000001415b2a98: push r14
00000001415b2a9a: push r13
00000001415b2a9c: push r12
00000001415b2a9e: push rsi
00000001415b2a9f: push rdi
00000001415b2aa0: push rbx
00000001415b2aa1: sub rsp, 0x68
00000001415b2aa5: lea rbp, [rdx + 0x80]
00000001415b2aac: movaps xmmword ptr [rsp + 0x20], xmm9
00000001415b2ab2: movaps xmmword ptr [rsp + 0x30], xmm8
00000001415b2ab8: movaps xmmword ptr [rsp + 0x40], xmm7
00000001415b2abd: movaps xmmword ptr [rsp + 0x50], xmm6
00000001415b2ac2: mov rdx, qword ptr [rbp + 0xa0]
00000001415b2ac9: test rdx, rdx
00000001415b2acc: je 0x1415b2ae4
00000001415b2ace: mov rcx, qword ptr [rbp + 0x90]
00000001415b2ad5: shl rdx, 5
00000001415b2ad9: mov r8d, 8
00000001415b2adf: call 0x140613c20 ; 140613c20
00000001415b2ae4: movaps xmm6, xmmword ptr [rsp + 0x50]
00000001415b2ae9: movaps xmm7, xmmword ptr [rsp + 0x40]
00000001415b2aee: movaps xmm8, xmmword ptr [rsp + 0x30]
00000001415b2af4: movaps xmm9, xmmword ptr [rsp + 0x20]
00000001415b2afa: add rsp, 0x68
00000001415b2afe: pop rbx
00000001415b2aff: pop rdi
00000001415b2b00: pop rsi
00000001415b2b01: pop r12
00000001415b2b03: pop r13
00000001415b2b05: pop r14
00000001415b2b07: pop r15
00000001415b2b09: pop rbp
00000001415b2b0a: ret 
00000001415b2b0b: nop dword ptr [rax + rax]
00000001415b2b10: mov qword ptr [rsp + 0x10], rdx
00000001415b2b15: push rbp
00000001415b2b16: push r15
00000001415b2b18: push r14
00000001415b2b1a: push r13
00000001415b2b1c: push r12
00000001415b2b1e: push rsi
00000001415b2b1f: push rdi
00000001415b2b20: push rbx
00000001415b2b21: sub rsp, 0x68
00000001415b2b25: lea rbp, [rdx + 0x80]
00000001415b2b2c: movaps xmmword ptr [rsp + 0x20], xmm9
00000001415b2b32: movaps xmmword ptr [rsp + 0x30], xmm8
00000001415b2b38: movaps xmmword ptr [rsp + 0x40], xmm7
00000001415b2b3d: movaps xmmword ptr [rsp + 0x50], xmm6
00000001415b2b42: lea rcx, [rbp + 0x90]
00000001415b2b49: call 0x14088a560 ; core::ptr::drop_in_place<serde_json::value::de::SeqDeserializer>
00000001415b2b4e: movaps xmm6, xmmword ptr [rsp + 0x50]
00000001415b2b53: movaps xmm7, xmmword ptr [rsp + 0x40]
00000001415b2b58: movaps xmm8, xmmword ptr [rsp + 0x30]
00000001415b2b5e: movaps xmm9, xmmword ptr [rsp + 0x20]
00000001415b2b64: add rsp, 0x68
00000001415b2b68: pop rbx
00000001415b2b69: pop rdi
00000001415b2b6a: pop rsi
00000001415b2b6b: pop r12
00000001415b2b6d: pop r13
00000001415b2b6f: pop r14
00000001415b2b71: pop r15
00000001415b2b73: pop rbp
00000001415b2b74: ret 
