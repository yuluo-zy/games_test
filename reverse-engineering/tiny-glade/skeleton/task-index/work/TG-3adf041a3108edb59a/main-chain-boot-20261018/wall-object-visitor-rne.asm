00000001415bb1c0: push rbp
00000001415bb1c1: push r15
00000001415bb1c3: push r14
00000001415bb1c5: push r13
00000001415bb1c7: push r12
00000001415bb1c9: push rsi
00000001415bb1ca: push rdi
00000001415bb1cb: push rbx
00000001415bb1cc: sub rsp, 0x298
00000001415bb1d3: lea rbp, [rsp + 0x80]
00000001415bb1db: movaps xmmword ptr [rbp + 0x200], xmm9
00000001415bb1e3: movaps xmmword ptr [rbp + 0x1f0], xmm8
00000001415bb1eb: movaps xmmword ptr [rbp + 0x1e0], xmm7
00000001415bb1f2: movaps xmmword ptr [rbp + 0x1d0], xmm6
00000001415bb1f9: mov qword ptr [rbp + 0x1c8], 0xfffffffffffffffe
00000001415bb204: mov qword ptr [rbp + 0x120], rcx
00000001415bb20b: mov rax, qword ptr [rdx + 0x10]
00000001415bb20f: mov qword ptr [rbp + 0x28], rax
00000001415bb213: lea r14, [rbp + 0xb8]
00000001415bb21a: mov rcx, r14
00000001415bb21d: call 0x140b880d0 ; serde_json::value::de::MapDeserializer::new
00000001415bb222: movabs rax, 0x8000000000000000
00000001415bb22c: mov qword ptr [rbp + 0x1b8], rax
00000001415bb233: mov qword ptr [rbp - 0x10], rax
00000001415bb237: lea rbx, [rbp + 0x131]
00000001415bb23e: mov dword ptr [rbp + 0x17c], 4
00000001415bb248: mov byte ptr [rbp + 0x1b7], 3
00000001415bb24f: mov dword ptr [rbp + 0x180], 0
00000001415bb259: mov byte ptr [rbp + 0x1c4], 2
00000001415bb260: lea r12, [rbp + 0x190]
00000001415bb267: lea r15, [rip + 0x16c01ca] ; 142c7b438 85ff93fe970394fe380294febb0294fe100194fe080494fe730494fe2c0394fe4e0594fecd0194fede0494fea50094fe800194feef1294feef1294feef1294fefe1394fe1f1494fe551494feef1294fe b'\x85\xff\x93\xfe\x97\x03\x94\xfe8\x02\x94\xfe\xbb\x02\x94\xfe\x10\x01\x94\xfe\x08\x04\x94\xfes\x04\x94\xfe,\x03\x94\xfeN\x05\x94\xfe\xcd\x01\x94\xfe\xde\x04\x94\xfe\xa5\x00\x94\xfe\x80\x01\x94\xfe\xef\x12\x94\xfe\xef\x12\x94\xfe\xef\x12\x94\xfe\xfe\x13\x94\xfe\x1f\x14\x94\xfeU\x14\x94\xfe\xef\x12\x94\xfe'
00000001415bb26e: mov byte ptr [rbp + 0x1b6], 2
00000001415bb275: mov byte ptr [rbp + 0x1c5], 2
00000001415bb27c: mov byte ptr [rbp + 0x1c6], 2
00000001415bb283: lea r13, [rbp + 0x130]
00000001415bb28a: lea rsi, [rip + 0x16c018b] ; 142c7b41c 38ff93fe38ff93fe38ff93fe440094fe6b0094feb00094fe38ff93fe85ff93fe970394fe380294febb0294fe100194fe080494fe730494fe2c0394fe4e0594fecd0194fede0494fea50094fe800194fe b'8\xff\x93\xfe8\xff\x93\xfe8\xff\x93\xfeD\x00\x94\xfek\x00\x94\xfe\xb0\x00\x94\xfe8\xff\x93\xfe\x85\xff\x93\xfe\x97\x03\x94\xfe8\x02\x94\xfe\xbb\x02\x94\xfe\x10\x01\x94\xfe\x08\x04\x94\xfes\x04\x94\xfe,\x03\x94\xfeN\x05\x94\xfe\xcd\x01\x94\xfe\xde\x04\x94\xfe\xa5\x00\x94\xfe\x80\x01\x94\xfe'
00000001415bb291: mov dword ptr [rbp + 0x18c], 0
00000001415bb29b: mov dword ptr [rbp + 0x188], 0
00000001415bb2a5: mov dword ptr [rbp + 0x184], 0
00000001415bb2af: mov dword ptr [rbp + 0x178], 0
00000001415bb2b9: nop dword ptr [rax]
00000001415bb2c0: mov byte ptr [rbp + 0x1c7], 1
00000001415bb2c7: mov rcx, r12
00000001415bb2ca: mov rdx, r14
00000001415bb2cd: call 0x14143c450 ; alloc::collections::btree::map::IntoIter::dying_next<alloc::string::String,enum2$<serde_json::value::Value>,alloc::alloc::Global>
00000001415bb2d2: mov rax, qword ptr [rbp + 0x190]
00000001415bb2d9: test rax, rax
00000001415bb2dc: je 0x1415bb9f9
00000001415bb2e2: mov rcx, qword ptr [rbp + 0x1a0]
00000001415bb2e9: lea rdx, [rcx + rcx*2]
00000001415bb2ed: mov r8, qword ptr [rax + rdx*8 + 0x168]
00000001415bb2f5: mov r9, qword ptr [rax + rdx*8 + 0x170]
00000001415bb2fd: mov qword ptr [rbp + 0x128], r9
00000001415bb304: mov rdi, qword ptr [rax + rdx*8 + 0x178]
00000001415bb30c: shl rcx, 5
00000001415bb310: movups xmm0, xmmword ptr [rax + rcx]
00000001415bb314: movups xmm1, xmmword ptr [rax + rcx + 0x10]
00000001415bb319: movaps xmmword ptr [rbp + 0x130], xmm0
00000001415bb320: movaps xmmword ptr [rbp + 0x140], xmm1
00000001415bb327: mov qword ptr [rbp + 0x170], r8
00000001415bb32e: mov rax, r8
00000001415bb331: neg rax
00000001415bb334: jo 0x1415bb9f9
00000001415bb33a: movzx eax, byte ptr [rbp + 0x100]
00000001415bb341: cmp rax, 6
00000001415bb345: ja 0x1415bb4cc
00000001415bb34b: movsxd rax, dword ptr [rsi + rax*4]
00000001415bb34f: add rax, rsi
00000001415bb352: jmp rax
00000001415bb354: movaps xmm0, xmmword ptr [rbp + 0x130]
00000001415bb35b: movaps xmm1, xmmword ptr [rbp + 0x140]
00000001415bb362: lea rax, [rbp + 0x100]
00000001415bb369: movups xmmword ptr [rax + 0x10], xmm1
00000001415bb36d: movups xmmword ptr [rax], xmm0
00000001415bb370: mov rcx, r12
00000001415bb373: mov rdx, qword ptr [rbp + 0x128]
00000001415bb37a: mov r8, rdi
00000001415bb37d: call 0x140861520 ; country_core::resources::walls::inner_wall_state::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
00000001415bb382: mov rdx, qword ptr [rbp + 0x170]
00000001415bb389: test rdx, rdx
00000001415bb38c: je 0x1415bb3a0
00000001415bb38e: mov r8d, 1
00000001415bb394: mov rcx, qword ptr [rbp + 0x128]
00000001415bb39b: call 0x140613c20 ; 140613c20
00000001415bb3a0: cmp byte ptr [rbp + 0x190], 0
00000001415bb3a7: jne 0x1415bba9c
00000001415bb3ad: movzx eax, byte ptr [rbp + 0x191]
00000001415bb3b4: movsxd rax, dword ptr [r15 + rax*4]
00000001415bb3b8: add rax, r15
00000001415bb3bb: jmp rax
00000001415bb3bd: mov rax, qword ptr [rbp + 0x1b8]
00000001415bb3c4: neg rax
00000001415bb3c7: jno 0x1415bc02f
00000001415bb3cd: movzx eax, byte ptr [rbp + 0x100]
00000001415bb3d4: mov byte ptr [rbp + 0x100], 6
00000001415bb3db: cmp al, 6
00000001415bb3dd: je 0x1415bbd7a
00000001415bb3e3: mov byte ptr [rbp + 0x190], al
00000001415bb3e9: movups xmm0, xmmword ptr [r14 + 0x49]
00000001415bb3ee: movups xmm1, xmmword ptr [r14 + 0x58]
00000001415bb3f3: lea rax, [rbp + 0x1a0]
00000001415bb3fa: movups xmmword ptr [rax], xmm1
00000001415bb3fd: movups xmmword ptr [rax - 0xf], xmm0
00000001415bb401: mov byte ptr [rbp + 0x1c7], 1
00000001415bb408: mov rcx, r13
00000001415bb40b: mov rdx, r12
00000001415bb40e: call 0x141285770 ; utils::curve::curve2_serialized_as_curve3::deserialize<enum2$<serde_json::value::Value> >
00000001415bb413: mov rcx, qword ptr [rbp + 0x130]
00000001415bb41a: mov rdi, qword ptr [rbp + 0x138]
00000001415bb421: mov rax, rcx
00000001415bb424: neg rax
00000001415bb427: jo 0x1415bbeff
00000001415bb42d: mov rax, qword ptr [rbx + 0x2f]
00000001415bb431: lea rdx, [rbp]
00000001415bb435: mov qword ptr [rdx + 0x20], rax
00000001415bb439: movups xmm0, xmmword ptr [rbx + 0xf]
00000001415bb43d: movups xmm1, xmmword ptr [rbx + 0x1f]
00000001415bb441: movups xmmword ptr [rdx + 0x10], xmm1
00000001415bb445: movups xmmword ptr [rdx], xmm0
00000001415bb448: mov qword ptr [rbp + 0x1b8], rcx
00000001415bb44f: mov qword ptr [rbp - 0x10], rcx
00000001415bb453: mov qword ptr [rbp + 0x30], rdi
00000001415bb457: mov qword ptr [rbp - 8], rdi
00000001415bb45b: jmp 0x1415bb2c0
00000001415bb460: mov rdx, qword ptr [rbp + 0x108]
00000001415bb467: test rdx, rdx
00000001415bb46a: je 0x1415bb354
00000001415bb470: mov rcx, qword ptr [rbp + 0x110]
00000001415bb477: mov r8d, 1
00000001415bb47d: call 0x140613c20 ; 140613c20
00000001415bb482: jmp 0x1415bb354
00000001415bb487: mov rcx, qword ptr [rbp + 0x110]
00000001415bb48e: mov rdx, qword ptr [rbp + 0x118]
00000001415bb495: mov qword ptr [rbp + 0xb0], rcx
00000001415bb49c: call 0x140888d70 ; core::ptr::drop_in_place<slice2$<enum2$<serde_json::value::Value> > >
00000001415bb4a1: mov rdx, qword ptr [rbp + 0x108]
00000001415bb4a8: test rdx, rdx
00000001415bb4ab: je 0x1415bb354
00000001415bb4b1: shl rdx, 5
00000001415bb4b5: mov r8d, 8
00000001415bb4bb: mov rcx, qword ptr [rbp + 0xb0]
00000001415bb4c2: call 0x140613c20 ; 140613c20
00000001415bb4c7: jmp 0x1415bb354
00000001415bb4cc: lea rcx, [rbp + 0x108]
00000001415bb4d3: call 0x140898f40 ; core::ptr::drop_in_place<serde_json::map::Map<alloc::string::String,enum2$<serde_json::value::Value> > >
00000001415bb4d8: jmp 0x1415bb354
00000001415bb4dd: cmp byte ptr [rbp + 0x1c4], 2
00000001415bb4e4: jne 0x1415bbf64
00000001415bb4ea: movzx eax, byte ptr [rbp + 0x100]
00000001415bb4f1: mov byte ptr [rbp + 0x100], 6
00000001415bb4f8: cmp al, 6
00000001415bb4fa: je 0x1415bbe45
00000001415bb500: mov byte ptr [rbp + 0x130], al
00000001415bb506: movups xmm0, xmmword ptr [r14 + 0x49]
00000001415bb50b: movups xmm1, xmmword ptr [r14 + 0x58]
00000001415bb510: movups xmmword ptr [rbx + 0xf], xmm1
00000001415bb514: movups xmmword ptr [rbx], xmm0
00000001415bb517: mov byte ptr [rbp + 0x1c7], 1
00000001415bb51e: mov rcx, r12
00000001415bb521: mov rdx, r13
00000001415bb524: call 0x14159d4f0 ; serde_json::value::de::impl$2::deserialize_bool<serde::de::impls::BoolVisitor>
00000001415bb529: test byte ptr [rbp + 0x190], 1
00000001415bb530: jne 0x1415bba9c
00000001415bb536: movzx eax, byte ptr [rbp + 0x191]
00000001415bb53d: mov byte ptr [rbp + 0x1c4], al
00000001415bb543: jmp 0x1415bb2c0
00000001415bb548: cmp dword ptr [rbp + 0x188], 0
00000001415bb54f: jne 0x1415bbff5
00000001415bb555: movzx eax, byte ptr [rbp + 0x100]
00000001415bb55c: mov byte ptr [rbp + 0x100], 6
00000001415bb563: cmp al, 6
00000001415bb565: je 0x1415bbdb7
00000001415bb56b: mov byte ptr [rbp + 0x130], al
00000001415bb571: movups xmm0, xmmword ptr [r14 + 0x49]
00000001415bb576: movups xmm1, xmmword ptr [r14 + 0x58]
00000001415bb57b: movups xmmword ptr [rbx + 0xf], xmm1
00000001415bb57f: movups xmmword ptr [rbx], xmm0
00000001415bb582: mov byte ptr [rbp + 0x1c7], 1
00000001415bb589: mov rcx, r12
00000001415bb58c: mov rdx, r13
00000001415bb58f: call 0x14159a370 ; serde_json::value::de::impl$2::deserialize_f32<serde::de::impls::impl$76::deserialize::PrimitiveVisitor>
00000001415bb594: test byte ptr [rbp + 0x190], 1
00000001415bb59b: jne 0x1415bba9c
00000001415bb5a1: movss xmm7, dword ptr [rbp + 0x194]
00000001415bb5a9: mov dword ptr [rbp + 0x188], 1
00000001415bb5b3: jmp 0x1415bb2c0
00000001415bb5b8: movzx eax, byte ptr [rbp + 0x100]
00000001415bb5bf: mov byte ptr [rbp + 0x100], 6
00000001415bb5c6: cmp al, 6
00000001415bb5c8: je 0x1415bbd40
00000001415bb5ce: mov byte ptr [rbp + 0x130], al
00000001415bb5d4: movups xmm0, xmmword ptr [r14 + 0x49]
00000001415bb5d9: movups xmm1, xmmword ptr [r14 + 0x58]
00000001415bb5de: movups xmmword ptr [rbx + 0xf], xmm1
00000001415bb5e2: movups xmmword ptr [rbx], xmm0
00000001415bb5e5: mov byte ptr [rbp + 0x1c7], 1
00000001415bb5ec: mov rcx, r13
00000001415bb5ef: call 0x1415a3940 ; serde_json::value::de::impl$2::deserialize_ignored_any<serde::de::ignored_any::IgnoredAny>
00000001415bb5f4: mov rdi, rax
00000001415bb5f7: test rax, rax
00000001415bb5fa: je 0x1415bb2c0
00000001415bb600: jmp 0x1415bbeae
00000001415bb605: cmp byte ptr [rbp + 0x1b6], 2
00000001415bb60c: jne 0x1415bc04c
00000001415bb612: movzx eax, byte ptr [rbp + 0x100]
00000001415bb619: mov byte ptr [rbp + 0x100], 6
00000001415bb620: cmp al, 6
00000001415bb622: je 0x1415bbe93
00000001415bb628: mov byte ptr [rbp + 0x130], al
00000001415bb62e: movups xmm0, xmmword ptr [r14 + 0x49]
00000001415bb633: movups xmm1, xmmword ptr [r14 + 0x58]
00000001415bb638: movups xmmword ptr [rbx + 0xf], xmm1
00000001415bb63c: movups xmmword ptr [rbx], xmm0
00000001415bb63f: mov byte ptr [rbp + 0x1c7], 1
00000001415bb646: mov rcx, r12
00000001415bb649: mov rdx, r13
00000001415bb64c: call 0x14085ff00 ; country_core::resources::walls::inner_wall_state::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
00000001415bb651: test byte ptr [rbp + 0x190], 1
00000001415bb658: jne 0x1415bba9c
00000001415bb65e: movzx eax, byte ptr [rbp + 0x191]
00000001415bb665: mov byte ptr [rbp + 0x1b6], al
00000001415bb66b: jmp 0x1415bb2c0
00000001415bb670: cmp dword ptr [rbp + 0x17c], 4
00000001415bb677: jne 0x1415bbf81
00000001415bb67d: movzx eax, byte ptr [rbp + 0x100]
00000001415bb684: mov byte ptr [rbp + 0x100], 6
00000001415bb68b: cmp al, 6
00000001415bb68d: je 0x1415bbe0e
00000001415bb693: mov byte ptr [rbp + 0x130], al
00000001415bb699: movups xmm0, xmmword ptr [r14 + 0x49]
00000001415bb69e: movups xmm1, xmmword ptr [r14 + 0x58]
00000001415bb6a3: movups xmmword ptr [rbx + 0xf], xmm1
00000001415bb6a7: movups xmmword ptr [rbx], xmm0
00000001415bb6aa: mov byte ptr [rbp + 0x1c7], 1
00000001415bb6b1: mov rcx, r12
00000001415bb6b4: mov rdx, r13
00000001415bb6b7: call 0x1416ebaf0 ; country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
00000001415bb6bc: test byte ptr [rbp + 0x190], 1
00000001415bb6c3: jne 0x1415bba9c
00000001415bb6c9: mov eax, dword ptr [rbp + 0x194]
00000001415bb6cf: mov dword ptr [rbp + 0x17c], eax
00000001415bb6d5: mov rax, qword ptr [rbp + 0x198]
00000001415bb6dc: mov qword ptr [rbp + 0x38], rax
00000001415bb6e0: lea rax, [rbp + 0x1a0]
00000001415bb6e7: movups xmm0, xmmword ptr [rax]
00000001415bb6ea: movaps xmmword ptr [rbp - 0x60], xmm0
00000001415bb6ee: jmp 0x1415bb2c0
00000001415bb6f3: cmp dword ptr [rbp + 0x184], 0
00000001415bb6fa: jne 0x1415bbfd8
00000001415bb700: movzx eax, byte ptr [rbp + 0x100]
00000001415bb707: mov byte ptr [rbp + 0x100], 6
00000001415bb70e: cmp al, 6
00000001415bb710: je 0x1415bbdd4
00000001415bb716: mov byte ptr [rbp + 0x130], al
00000001415bb71c: movups xmm0, xmmword ptr [r14 + 0x49]
00000001415bb721: movups xmm1, xmmword ptr [r14 + 0x58]
00000001415bb726: movups xmmword ptr [rbx + 0xf], xmm1
00000001415bb72a: movups xmmword ptr [rbx], xmm0
00000001415bb72d: mov byte ptr [rbp + 0x1c7], 1
00000001415bb734: mov rcx, r12
00000001415bb737: mov rdx, r13
00000001415bb73a: call 0x14159a370 ; serde_json::value::de::impl$2::deserialize_f32<serde::de::impls::impl$76::deserialize::PrimitiveVisitor>
00000001415bb73f: test byte ptr [rbp + 0x190], 1
00000001415bb746: jne 0x1415bba9c
00000001415bb74c: movss xmm8, dword ptr [rbp + 0x194]
00000001415bb755: mov dword ptr [rbp + 0x184], 1
00000001415bb75f: jmp 0x1415bb2c0
00000001415bb764: cmp byte ptr [rbp + 0x1c5], 2
00000001415bb76b: jne 0x1415bc086
00000001415bb771: movzx eax, byte ptr [rbp + 0x100]
00000001415bb778: mov byte ptr [rbp + 0x100], 6
00000001415bb77f: cmp al, 6
00000001415bb781: je 0x1415bbd5d
00000001415bb787: mov byte ptr [rbp + 0x130], al
00000001415bb78d: movups xmm0, xmmword ptr [r14 + 0x49]
00000001415bb792: movups xmm1, xmmword ptr [r14 + 0x58]
00000001415bb797: movups xmmword ptr [rbx + 0xf], xmm1
00000001415bb79b: movups xmmword ptr [rbx], xmm0
00000001415bb79e: mov byte ptr [rbp + 0x1c7], 1
00000001415bb7a5: mov rcx, r12
00000001415bb7a8: mov rdx, r13
00000001415bb7ab: call 0x140861780 ; country_core::resources::walls::inner_wall_state::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
00000001415bb7b0: test byte ptr [rbp + 0x190], 1
00000001415bb7b7: jne 0x1415bba9c
00000001415bb7bd: movzx eax, byte ptr [rbp + 0x191]
00000001415bb7c4: mov byte ptr [rbp + 0x1c5], al
00000001415bb7ca: jmp 0x1415bb2c0
00000001415bb7cf: cmp dword ptr [rbp + 0x178], 0
00000001415bb7d6: jne 0x1415bc069
00000001415bb7dc: movzx eax, byte ptr [rbp + 0x100]
00000001415bb7e3: mov byte ptr [rbp + 0x100], 6
00000001415bb7ea: cmp al, 6
00000001415bb7ec: je 0x1415bbe79
00000001415bb7f2: mov byte ptr [rbp + 0x130], al
00000001415bb7f8: movups xmm0, xmmword ptr [r14 + 0x49]
00000001415bb7fd: movups xmm1, xmmword ptr [r14 + 0x58]
00000001415bb802: movups xmmword ptr [rbx + 0xf], xmm1
00000001415bb806: movups xmmword ptr [rbx], xmm0
00000001415bb809: mov byte ptr [rbp + 0x1c7], 1
00000001415bb810: mov rcx, r12
00000001415bb813: mov rdx, r13
00000001415bb816: call 0x14159a370 ; serde_json::value::de::impl$2::deserialize_f32<serde::de::impls::impl$76::deserialize::PrimitiveVisitor>
00000001415bb81b: test byte ptr [rbp + 0x190], 1
00000001415bb822: jne 0x1415bba9c
00000001415bb828: movss xmm9, dword ptr [rbp + 0x194]
00000001415bb831: mov dword ptr [rbp + 0x178], 1
00000001415bb83b: jmp 0x1415bb2c0
00000001415bb840: cmp byte ptr [rbp + 0x1c6], 2
00000001415bb847: jne 0x1415bbf9e
00000001415bb84d: movzx eax, byte ptr [rbp + 0x100]
00000001415bb854: mov byte ptr [rbp + 0x100], 6
00000001415bb85b: cmp al, 6
00000001415bb85d: je 0x1415bbe5f
00000001415bb863: mov byte ptr [rbp + 0x130], al
00000001415bb869: movups xmm0, xmmword ptr [r14 + 0x49]
00000001415bb86e: movups xmm1, xmmword ptr [r14 + 0x58]
00000001415bb873: movups xmmword ptr [rbx + 0xf], xmm1
00000001415bb877: movups xmmword ptr [rbx], xmm0
00000001415bb87a: mov byte ptr [rbp + 0x1c7], 1
00000001415bb881: mov rcx, r12
00000001415bb884: mov rdx, r13
00000001415bb887: call 0x14085ff00 ; country_core::resources::walls::inner_wall_state::_::impl$0::deserialize<enum2$<serde_json::value::Value> >
00000001415bb88c: test byte ptr [rbp + 0x190], 1
00000001415bb893: jne 0x1415bba9c
00000001415bb899: movzx eax, byte ptr [rbp + 0x191]
00000001415bb8a0: mov byte ptr [rbp + 0x1c6], al
00000001415bb8a6: jmp 0x1415bb2c0
00000001415bb8ab: cmp byte ptr [rbp + 0x1b7], 3
00000001415bb8b2: jne 0x1415bc0a3
00000001415bb8b8: movzx eax, byte ptr [rbp + 0x100]
00000001415bb8bf: mov byte ptr [rbp + 0x100], 6
00000001415bb8c6: cmp al, 6
00000001415bb8c8: je 0x1415bbdf1
00000001415bb8ce: mov byte ptr [rbp + 0x130], al
00000001415bb8d4: movups xmm0, xmmword ptr [r14 + 0x49]
00000001415bb8d9: movups xmm1, xmmword ptr [r14 + 0x58]
00000001415bb8de: movups xmmword ptr [rbx + 0xf], xmm1
00000001415bb8e2: movups xmmword ptr [rbx], xmm0
00000001415bb8e5: mov byte ptr [rbp + 0x1c7], 1
00000001415bb8ec: mov rcx, r12
00000001415bb8ef: mov rdx, r13
00000001415bb8f2: call 0x14085ebc0 ; country_core::resources::walls::inner_wall_state::deserialize_wall_style<enum2$<serde_json::value::Value> >
00000001415bb8f7: cmp byte ptr [rbp + 0x190], 0
00000001415bb8fe: jne 0x1415bba9c
00000001415bb904: movzx eax, byte ptr [rbp + 0x191]
00000001415bb90b: mov byte ptr [rbp + 0x1b7], al
00000001415bb911: jmp 0x1415bb2c0
00000001415bb916: cmp dword ptr [rbp + 0x180], 0
00000001415bb91d: jne 0x1415bbfbb
00000001415bb923: movzx eax, byte ptr [rbp + 0x100]
00000001415bb92a: mov byte ptr [rbp + 0x100], 6
00000001415bb931: cmp al, 6
00000001415bb933: je 0x1415bbd9a
00000001415bb939: mov byte ptr [rbp + 0x130], al
00000001415bb93f: movups xmm0, xmmword ptr [r14 + 0x49]
00000001415bb944: movups xmm1, xmmword ptr [r14 + 0x58]
00000001415bb949: movups xmmword ptr [rbx + 0xf], xmm1
00000001415bb94d: movups xmmword ptr [rbx], xmm0
00000001415bb950: mov byte ptr [rbp + 0x1c7], 1
00000001415bb957: mov rcx, r12
00000001415bb95a: mov rdx, r13
00000001415bb95d: call 0x14159a370 ; serde_json::value::de::impl$2::deserialize_f32<serde::de::impls::impl$76::deserialize::PrimitiveVisitor>
00000001415bb962: test byte ptr [rbp + 0x190], 1
00000001415bb969: jne 0x1415bba9c
00000001415bb96f: movss xmm6, dword ptr [rbp + 0x194]
00000001415bb977: mov dword ptr [rbp + 0x180], 1
00000001415bb981: jmp 0x1415bb2c0
00000001415bb986: test byte ptr [rbp + 0x18c], 1
00000001415bb98d: jne 0x1415bc012
00000001415bb993: movzx eax, byte ptr [rbp + 0x100]
00000001415bb99a: mov byte ptr [rbp + 0x100], 6
00000001415bb9a1: cmp al, 6
00000001415bb9a3: je 0x1415bbe2b
00000001415bb9a9: mov byte ptr [rbp + 0x130], al
00000001415bb9af: movups xmm0, xmmword ptr [r14 + 0x49]
00000001415bb9b4: movups xmm1, xmmword ptr [r14 + 0x58]
00000001415bb9b9: movups xmmword ptr [rbx + 0xf], xmm1
00000001415bb9bd: movups xmmword ptr [rbx], xmm0
00000001415bb9c0: mov byte ptr [rbp + 0x1c7], 1
00000001415bb9c7: mov rcx, r12
00000001415bb9ca: mov rdx, r13
00000001415bb9cd: call 0x14159a180 ; serde_json::value::de::impl$2::deserialize_u8<serde::de::impls::impl$63::deserialize::PrimitiveVisitor>
00000001415bb9d2: cmp byte ptr [rbp + 0x190], 0
00000001415bb9d9: jne 0x1415bba9c
00000001415bb9df: movzx eax, byte ptr [rbp + 0x191]
00000001415bb9e6: mov byte ptr [rbp + 0x1b4], al
00000001415bb9ec: mov al, 1
00000001415bb9ee: mov dword ptr [rbp + 0x18c], eax
00000001415bb9f4: jmp 0x1415bb2c0
00000001415bb9f9: mov rax, qword ptr [rbp + 0x1b8]
00000001415bba00: neg rax
00000001415bba03: setno sil
00000001415bba07: jo 0x1415bbaa8
00000001415bba0d: mov rax, qword ptr [rbp + 0x20]
00000001415bba11: mov qword ptr [rbp + 0x160], rax
00000001415bba18: movups xmm0, xmmword ptr [rbp - 0x10]
00000001415bba1c: movups xmm1, xmmword ptr [rbp]
00000001415bba20: movups xmm2, xmmword ptr [rbp + 0x10]
00000001415bba24: movaps xmmword ptr [rbp + 0x150], xmm2
00000001415bba2b: movaps xmmword ptr [rbp + 0x140], xmm1
00000001415bba32: movaps xmmword ptr [rbp + 0x130], xmm0
00000001415bba39: test byte ptr [rbp + 0x178], 1
00000001415bba40: je 0x1415bbac8
00000001415bba46: mov r14d, dword ptr [rbp + 0x17c]
00000001415bba4d: cmp r14d, 4
00000001415bba51: jne 0x1415bbb30
00000001415bba57: lea rdx, [rip + 0x16bf6c5] ; 142c7b123 69735f656e636c6f736564617263685f686569676874617263685f74797374796c657374727563747572655f7479636c75747465725f76657273696f6e706c61737465725f7065656c72656d6f766564 b'is_enclosedarch_heightarch_tystylestructure_tyclutter_versionplaster_peelremoved'
00000001415bba5e: lea rcx, [rbp + 0x190]
00000001415bba65: mov r8d, 0xb
00000001415bba6b: call 0x1416ebe70 ; country_core::systems::wall::valid_enclosure::_::impl$0::deserialize<serde::__private::de::missing_field::MissingFieldDeserializer<serde_json::error::Error> >
00000001415bba70: movzx r9d, byte ptr [rbp + 0x1c4]
00000001415bba78: movzx r11d, byte ptr [rbp + 0x1c5]
00000001415bba80: mov bl, byte ptr [rbp + 0x1c6]
00000001415bba86: test byte ptr [rbp + 0x190], 1
00000001415bba8d: je 0x1415bbb55
00000001415bba93: mov rdi, qword ptr [rbp + 0x198]
00000001415bba9a: jmp 0x1415bbadc
00000001415bba9c: mov rdi, qword ptr [rbp + 0x198]
00000001415bbaa3: jmp 0x1415bbeae
00000001415bbaa8: mov byte ptr [rbp + 0x1c7], 1
00000001415bbaaf: lea rcx, [rip + 0x16bf662] ; 142c7b118 637572766568656967687469735f656e636c6f736564617263685f686569676874617263685f74797374796c657374727563747572655f7479636c75747465725f76657273696f6e706c61737465725f b'curveheightis_enclosedarch_heightarch_tystylestructure_tyclutter_versionplaster_'
00000001415bbab6: mov edx, 5
00000001415bbabb: call 0x1428f9bc0 ; serde::de::Error::missing_field<serde_json::error::Error>
00000001415bbac0: mov rdi, rax
00000001415bbac3: jmp 0x1415bbeb0
00000001415bbac8: lea rcx, [rip + 0x16bf64e] ; 142c7b11d 68656967687469735f656e636c6f736564617263685f686569676874617263685f74797374796c657374727563747572655f7479636c75747465725f76657273696f6e706c61737465725f7065656c72 b'heightis_enclosedarch_heightarch_tystylestructure_tyclutter_versionplaster_peelr'
00000001415bbacf: mov edx, 6
00000001415bbad4: call 0x1428f9bc0 ; serde::de::Error::missing_field<serde_json::error::Error>
00000001415bbad9: mov rdi, rax
00000001415bbadc: mov rdx, qword ptr [rbp + 0x130]
00000001415bbae3: test rdx, rdx
00000001415bbae6: je 0x1415bbafe
00000001415bbae8: mov rcx, qword ptr [rbp + 0x138]
00000001415bbaef: shl rdx, 3
00000001415bbaf3: mov r8d, 4
00000001415bbaf9: call 0x140613c20 ; 140613c20
00000001415bbafe: mov rdx, qword ptr [rbp + 0x148]
00000001415bbb05: test rdx, rdx
00000001415bbb08: mov rbx, qword ptr [rbp + 0x1b8]
00000001415bbb0f: je 0x1415bbeb7
00000001415bbb15: mov rcx, qword ptr [rbp + 0x150]
00000001415bbb1c: shl rdx, 2
00000001415bbb20: mov r8d, 4
00000001415bbb26: call 0x140613c20 ; 140613c20
00000001415bbb2b: jmp 0x1415bbeb7
00000001415bbb30: movaps xmm0, xmmword ptr [rbp - 0x60]
00000001415bbb34: movaps xmmword ptr [rbp - 0x20], xmm0
00000001415bbb38: movzx r9d, byte ptr [rbp + 0x1c4]
00000001415bbb40: movzx r11d, byte ptr [rbp + 0x1c5]
00000001415bbb48: movzx ebx, byte ptr [rbp + 0x1c6]
00000001415bbb4f: mov rsi, qword ptr [rbp + 0x38]
00000001415bbb53: jmp 0x1415bbb6e
00000001415bbb55: mov r14d, dword ptr [rbp + 0x194]
00000001415bbb5c: mov rsi, qword ptr [rbp + 0x198]
00000001415bbb63: movups xmm0, xmmword ptr [rbp + 0x1a0]
00000001415bbb6a: movaps xmmword ptr [rbp - 0x20], xmm0
00000001415bbb6e: mov rcx, qword ptr [rbp + 0x130]
00000001415bbb75: mov rdi, qword ptr [rbp + 0x138]
00000001415bbb7c: movaps xmm0, xmmword ptr [rbp + 0x140]
00000001415bbb83: movaps xmmword ptr [rbp - 0x50], xmm0
00000001415bbb87: movaps xmm0, xmmword ptr [rbp + 0x150]
00000001415bbb8e: movaps xmmword ptr [rbp - 0x40], xmm0
00000001415bbb92: mov rax, qword ptr [rbp + 0x160]
00000001415bbb99: mov qword ptr [rbp - 0x30], rax
00000001415bbb9d: mov rax, rcx
00000001415bbba0: neg rax
00000001415bbba3: jo 0x1415bbeff
00000001415bbba9: movzx eax, byte ptr [rbp + 0x1b4]
00000001415bbbb0: xor r8d, r8d
00000001415bbbb3: test byte ptr [rbp + 0x18c], 1
00000001415bbbba: cmove eax, r8d
00000001415bbbbe: movzx edx, byte ptr [rbp + 0x1b7]
00000001415bbbc5: cmp dl, 3
00000001415bbbc8: movzx edx, dl
00000001415bbbcb: cmove edx, r8d
00000001415bbbcf: test byte ptr [rbp + 0x188], 1
00000001415bbbd6: jne 0x1415bbbe0
00000001415bbbd8: movss xmm7, dword ptr [rip + 0x14d1080] ; 142a8cc60 333333409a9999bf000080c0000000000000803ecdcc4c3d00000000000000000000003f000000bf000000000000000000000000000000000100000000000000010000000000000070f67f4001000000 b'333@\x9a\x99\x99\xbf\x00\x00\x80\xc0\x00\x00\x00\x00\x00\x00\x80>\xcd\xccL=\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00?\x00\x00\x00\xbf\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00p\xf6\x7f@\x01\x00\x00\x00'
00000001415bbbe0: test byte ptr [rbp + 0x184], 1
00000001415bbbe7: jne 0x1415bbbed
00000001415bbbe9: xorps xmm8, xmm8
00000001415bbbed: and r11b, 1
00000001415bbbf1: and bl, 1
00000001415bbbf4: and r9b, 1
00000001415bbbf8: test byte ptr [rbp + 0x180], 1
00000001415bbbff: jne 0x1415bbc04
00000001415bbc01: xorps xmm6, xmm6
00000001415bbc04: movzx r10d, byte ptr [rbp + 0x1b6]
00000001415bbc0c: and r10b, 1
00000001415bbc10: mov r8, qword ptr [rbp - 0x30]
00000001415bbc14: mov qword ptr [rbp + 0x70], r8
00000001415bbc18: movaps xmm0, xmmword ptr [rbp - 0x50]
00000001415bbc1c: movaps xmm1, xmmword ptr [rbp - 0x40]
00000001415bbc20: movups xmmword ptr [rbp + 0x60], xmm1
00000001415bbc24: movups xmmword ptr [rbp + 0x50], xmm0
00000001415bbc28: movaps xmm0, xmmword ptr [rbp - 0x20]
00000001415bbc2c: movups xmmword ptr [rbp + 0x84], xmm0
00000001415bbc33: mov qword ptr [rbp + 0x40], rcx
00000001415bbc37: mov qword ptr [rbp + 0x48], rdi
00000001415bbc3b: mov dword ptr [rbp + 0x78], r14d
00000001415bbc3f: mov qword ptr [rbp + 0x7c], rsi
00000001415bbc43: movss dword ptr [rbp + 0x94], xmm9
00000001415bbc4c: movss dword ptr [rbp + 0x98], xmm8
00000001415bbc55: movss dword ptr [rbp + 0x9c], xmm7
00000001415bbc5d: movss dword ptr [rbp + 0xa0], xmm6
00000001415bbc65: mov byte ptr [rbp + 0xa4], bl
00000001415bbc6b: mov byte ptr [rbp + 0xa5], r11b
00000001415bbc72: mov byte ptr [rbp + 0xa6], r10b
00000001415bbc79: mov byte ptr [rbp + 0xa7], r9b
00000001415bbc80: mov byte ptr [rbp + 0xa8], dl
00000001415bbc86: mov byte ptr [rbp + 0xa9], al
00000001415bbc8c: cmp qword ptr [rbp + 0xf8], 0
00000001415bbc94: jne 0x1415bc0c0
00000001415bbc9a: mov eax, dword ptr [rbp + 0xa0]
00000001415bbca0: mov rsi, qword ptr [rbp + 0x120]
00000001415bbca7: mov dword ptr [rsi + 0x60], eax
00000001415bbcaa: movzx eax, byte ptr [rbp + 0xa4]
00000001415bbcb1: mov byte ptr [rsi + 0x64], al
00000001415bbcb4: movzx eax, byte ptr [rbp + 0xa5]
00000001415bbcbb: mov byte ptr [rsi + 0x65], al
00000001415bbcbe: movzx eax, byte ptr [rbp + 0xa6]
00000001415bbcc5: mov byte ptr [rsi + 0x66], al
00000001415bbcc8: movzx eax, byte ptr [rbp + 0xa7]
00000001415bbccf: mov byte ptr [rsi + 0x67], al
00000001415bbcd2: movzx eax, byte ptr [rbp + 0xa8]
00000001415bbcd9: mov byte ptr [rsi + 0x68], al
00000001415bbcdc: movzx eax, byte ptr [rbp + 0xa9]
00000001415bbce3: mov byte ptr [rsi + 0x69], al
00000001415bbce6: mov eax, dword ptr [rbp + 0xaa]
00000001415bbcec: mov dword ptr [rsi + 0x6a], eax
00000001415bbcef: movzx eax, word ptr [rbp + 0xae]
00000001415bbcf6: mov word ptr [rsi + 0x6e], ax
00000001415bbcfa: movups xmm0, xmmword ptr [rbp + 0x90]
00000001415bbd01: movups xmmword ptr [rsi + 0x50], xmm0
00000001415bbd05: movups xmm0, xmmword ptr [rbp + 0x80]
00000001415bbd0c: movups xmmword ptr [rsi + 0x40], xmm0
00000001415bbd10: movups xmm0, xmmword ptr [rbp + 0x40]
00000001415bbd14: movups xmm1, xmmword ptr [rbp + 0x50]
00000001415bbd18: movups xmm2, xmmword ptr [rbp + 0x60]
00000001415bbd1c: movups xmm3, xmmword ptr [rbp + 0x70]
00000001415bbd20: movups xmmword ptr [rsi + 0x30], xmm3
00000001415bbd24: movups xmmword ptr [rsi + 0x20], xmm2
00000001415bbd28: movups xmmword ptr [rsi + 0x10], xmm1
00000001415bbd2c: movups xmmword ptr [rsi], xmm0
00000001415bbd2f: lea rcx, [rbp + 0xb8]
00000001415bbd36: call 0x140887df0 ; core::ptr::drop_in_place<serde_json::map::IntoIter>
00000001415bbd3b: jmp 0x1415bbf23
00000001415bbd40: mov byte ptr [rbp + 0x1c7], 1
00000001415bbd47: lea rcx, [rip + 0x16be51a] ; 142c7a268 76616c7565206973206d697373696e677374727563742076617269616e74000078a2c742010000000e000000000000006e6577747970652076617269616e740098a2c742010000000f00000000000000 b'value is missingstruct variant\x00\x00x\xa2\xc7B\x01\x00\x00\x00\x0e\x00\x00\x00\x00\x00\x00\x00newtype variant\x00\x98\xa2\xc7B\x01\x00\x00\x00\x0f\x00\x00\x00\x00\x00\x00\x00'
00000001415bbd4e: mov edx, 0x10
00000001415bbd53: call 0x1428fa000 ; serde_json::error::impl$8::custom<ref$<str$> >
00000001415bbd58: jmp 0x1415bbeab
00000001415bbd5d: mov byte ptr [rbp + 0x1c7], 1
00000001415bbd64: lea rcx, [rip + 0x16be4fd] ; 142c7a268 76616c7565206973206d697373696e677374727563742076617269616e74000078a2c742010000000e000000000000006e6577747970652076617269616e740098a2c742010000000f00000000000000 b'value is missingstruct variant\x00\x00x\xa2\xc7B\x01\x00\x00\x00\x0e\x00\x00\x00\x00\x00\x00\x00newtype variant\x00\x98\xa2\xc7B\x01\x00\x00\x00\x0f\x00\x00\x00\x00\x00\x00\x00'
00000001415bbd6b: mov edx, 0x10
00000001415bbd70: call 0x1428fa000 ; serde_json::error::impl$8::custom<ref$<str$> >
00000001415bbd75: jmp 0x1415bbeab
00000001415bbd7a: mov byte ptr [rbp + 0x1c7], 1
00000001415bbd81: lea rcx, [rip + 0x16be4e0] ; 142c7a268 76616c7565206973206d697373696e677374727563742076617269616e74000078a2c742010000000e000000000000006e6577747970652076617269616e740098a2c742010000000f00000000000000 b'value is missingstruct variant\x00\x00x\xa2\xc7B\x01\x00\x00\x00\x0e\x00\x00\x00\x00\x00\x00\x00newtype variant\x00\x98\xa2\xc7B\x01\x00\x00\x00\x0f\x00\x00\x00\x00\x00\x00\x00'
00000001415bbd88: mov edx, 0x10
00000001415bbd8d: call 0x1428fa000 ; serde_json::error::impl$8::custom<ref$<str$> >
00000001415bbd92: mov rdi, rax
00000001415bbd95: jmp 0x1415bbeff
00000001415bbd9a: mov byte ptr [rbp + 0x1c7], 1
00000001415bbda1: lea rcx, [rip + 0x16be4c0] ; 142c7a268 76616c7565206973206d697373696e677374727563742076617269616e74000078a2c742010000000e000000000000006e6577747970652076617269616e740098a2c742010000000f00000000000000 b'value is missingstruct variant\x00\x00x\xa2\xc7B\x01\x00\x00\x00\x0e\x00\x00\x00\x00\x00\x00\x00newtype variant\x00\x98\xa2\xc7B\x01\x00\x00\x00\x0f\x00\x00\x00\x00\x00\x00\x00'
00000001415bbda8: mov edx, 0x10
00000001415bbdad: call 0x1428fa000 ; serde_json::error::impl$8::custom<ref$<str$> >
00000001415bbdb2: jmp 0x1415bbeab
00000001415bbdb7: mov byte ptr [rbp + 0x1c7], 1
00000001415bbdbe: lea rcx, [rip + 0x16be4a3] ; 142c7a268 76616c7565206973206d697373696e677374727563742076617269616e74000078a2c742010000000e000000000000006e6577747970652076617269616e740098a2c742010000000f00000000000000 b'value is missingstruct variant\x00\x00x\xa2\xc7B\x01\x00\x00\x00\x0e\x00\x00\x00\x00\x00\x00\x00newtype variant\x00\x98\xa2\xc7B\x01\x00\x00\x00\x0f\x00\x00\x00\x00\x00\x00\x00'
00000001415bbdc5: mov edx, 0x10
00000001415bbdca: call 0x1428fa000 ; serde_json::error::impl$8::custom<ref$<str$> >
00000001415bbdcf: jmp 0x1415bbeab
00000001415bbdd4: mov byte ptr [rbp + 0x1c7], 1
00000001415bbddb: lea rcx, [rip + 0x16be486] ; 142c7a268 76616c7565206973206d697373696e677374727563742076617269616e74000078a2c742010000000e000000000000006e6577747970652076617269616e740098a2c742010000000f00000000000000 b'value is missingstruct variant\x00\x00x\xa2\xc7B\x01\x00\x00\x00\x0e\x00\x00\x00\x00\x00\x00\x00newtype variant\x00\x98\xa2\xc7B\x01\x00\x00\x00\x0f\x00\x00\x00\x00\x00\x00\x00'
00000001415bbde2: mov edx, 0x10
00000001415bbde7: call 0x1428fa000 ; serde_json::error::impl$8::custom<ref$<str$> >
00000001415bbdec: jmp 0x1415bbeab
00000001415bbdf1: mov byte ptr [rbp + 0x1c7], 1
00000001415bbdf8: lea rcx, [rip + 0x16be469] ; 142c7a268 76616c7565206973206d697373696e677374727563742076617269616e74000078a2c742010000000e000000000000006e6577747970652076617269616e740098a2c742010000000f00000000000000 b'value is missingstruct variant\x00\x00x\xa2\xc7B\x01\x00\x00\x00\x0e\x00\x00\x00\x00\x00\x00\x00newtype variant\x00\x98\xa2\xc7B\x01\x00\x00\x00\x0f\x00\x00\x00\x00\x00\x00\x00'
00000001415bbdff: mov edx, 0x10
00000001415bbe04: call 0x1428fa000 ; serde_json::error::impl$8::custom<ref$<str$> >
00000001415bbe09: jmp 0x1415bbeab
00000001415bbe0e: mov byte ptr [rbp + 0x1c7], 1
00000001415bbe15: lea rcx, [rip + 0x16be44c] ; 142c7a268 76616c7565206973206d697373696e677374727563742076617269616e74000078a2c742010000000e000000000000006e6577747970652076617269616e740098a2c742010000000f00000000000000 b'value is missingstruct variant\x00\x00x\xa2\xc7B\x01\x00\x00\x00\x0e\x00\x00\x00\x00\x00\x00\x00newtype variant\x00\x98\xa2\xc7B\x01\x00\x00\x00\x0f\x00\x00\x00\x00\x00\x00\x00'
00000001415bbe1c: mov edx, 0x10
00000001415bbe21: call 0x1428fa000 ; serde_json::error::impl$8::custom<ref$<str$> >
00000001415bbe26: jmp 0x1415bbeab
00000001415bbe2b: mov byte ptr [rbp + 0x1c7], 1
00000001415bbe32: lea rcx, [rip + 0x16be42f] ; 142c7a268 76616c7565206973206d697373696e677374727563742076617269616e74000078a2c742010000000e000000000000006e6577747970652076617269616e740098a2c742010000000f00000000000000 b'value is missingstruct variant\x00\x00x\xa2\xc7B\x01\x00\x00\x00\x0e\x00\x00\x00\x00\x00\x00\x00newtype variant\x00\x98\xa2\xc7B\x01\x00\x00\x00\x0f\x00\x00\x00\x00\x00\x00\x00'
00000001415bbe39: mov edx, 0x10
00000001415bbe3e: call 0x1428fa000 ; serde_json::error::impl$8::custom<ref$<str$> >
00000001415bbe43: jmp 0x1415bbeab
00000001415bbe45: mov byte ptr [rbp + 0x1c7], 1
00000001415bbe4c: lea rcx, [rip + 0x16be415] ; 142c7a268 76616c7565206973206d697373696e677374727563742076617269616e74000078a2c742010000000e000000000000006e6577747970652076617269616e740098a2c742010000000f00000000000000 b'value is missingstruct variant\x00\x00x\xa2\xc7B\x01\x00\x00\x00\x0e\x00\x00\x00\x00\x00\x00\x00newtype variant\x00\x98\xa2\xc7B\x01\x00\x00\x00\x0f\x00\x00\x00\x00\x00\x00\x00'
00000001415bbe53: mov edx, 0x10
00000001415bbe58: call 0x1428fa000 ; serde_json::error::impl$8::custom<ref$<str$> >
00000001415bbe5d: jmp 0x1415bbeab
00000001415bbe5f: mov byte ptr [rbp + 0x1c7], 1
00000001415bbe66: lea rcx, [rip + 0x16be3fb] ; 142c7a268 76616c7565206973206d697373696e677374727563742076617269616e74000078a2c742010000000e000000000000006e6577747970652076617269616e740098a2c742010000000f00000000000000 b'value is missingstruct variant\x00\x00x\xa2\xc7B\x01\x00\x00\x00\x0e\x00\x00\x00\x00\x00\x00\x00newtype variant\x00\x98\xa2\xc7B\x01\x00\x00\x00\x0f\x00\x00\x00\x00\x00\x00\x00'
00000001415bbe6d: mov edx, 0x10
00000001415bbe72: call 0x1428fa000 ; serde_json::error::impl$8::custom<ref$<str$> >
00000001415bbe77: jmp 0x1415bbeab
00000001415bbe79: mov byte ptr [rbp + 0x1c7], 1
00000001415bbe80: lea rcx, [rip + 0x16be3e1] ; 142c7a268 76616c7565206973206d697373696e677374727563742076617269616e74000078a2c742010000000e000000000000006e6577747970652076617269616e740098a2c742010000000f00000000000000 b'value is missingstruct variant\x00\x00x\xa2\xc7B\x01\x00\x00\x00\x0e\x00\x00\x00\x00\x00\x00\x00newtype variant\x00\x98\xa2\xc7B\x01\x00\x00\x00\x0f\x00\x00\x00\x00\x00\x00\x00'
00000001415bbe87: mov edx, 0x10
00000001415bbe8c: call 0x1428fa000 ; serde_json::error::impl$8::custom<ref$<str$> >
00000001415bbe91: jmp 0x1415bbeab
00000001415bbe93: mov byte ptr [rbp + 0x1c7], 1
00000001415bbe9a: lea rcx, [rip + 0x16be3c7] ; 142c7a268 76616c7565206973206d697373696e677374727563742076617269616e74000078a2c742010000000e000000000000006e6577747970652076617269616e740098a2c742010000000f00000000000000 b'value is missingstruct variant\x00\x00x\xa2\xc7B\x01\x00\x00\x00\x0e\x00\x00\x00\x00\x00\x00\x00newtype variant\x00\x98\xa2\xc7B\x01\x00\x00\x00\x0f\x00\x00\x00\x00\x00\x00\x00'
00000001415bbea1: mov edx, 0x10
00000001415bbea6: call 0x1428fa000 ; serde_json::error::impl$8::custom<ref$<str$> >
00000001415bbeab: mov rdi, rax
00000001415bbeae: xor esi, esi
00000001415bbeb0: mov rbx, qword ptr [rbp + 0x1b8]
00000001415bbeb7: mov rax, rbx
00000001415bbeba: neg rax
00000001415bbebd: setno al
00000001415bbec0: not sil
00000001415bbec3: test sil, al
00000001415bbec6: je 0x1415bbeff
00000001415bbec8: test rbx, rbx
00000001415bbecb: je 0x1415bbee3
00000001415bbecd: shl rbx, 3
00000001415bbed1: mov r8d, 4
00000001415bbed7: mov rcx, qword ptr [rbp + 0x30]
00000001415bbedb: mov rdx, rbx
00000001415bbede: call 0x140613c20 ; 140613c20
00000001415bbee3: mov rdx, qword ptr [rbp + 8]
00000001415bbee7: test rdx, rdx
00000001415bbeea: je 0x1415bbeff
00000001415bbeec: mov rcx, qword ptr [rbp + 0x10]
00000001415bbef0: shl rdx, 2
00000001415bbef4: mov r8d, 4
00000001415bbefa: call 0x140613c20 ; 140613c20
00000001415bbeff: mov rsi, qword ptr [rbp + 0x120]
00000001415bbf06: mov qword ptr [rsi + 8], rdi
00000001415bbf0a: movabs rax, 0x8000000000000000
00000001415bbf14: mov qword ptr [rsi], rax
00000001415bbf17: lea rcx, [rbp + 0xb8]
00000001415bbf1e: call 0x140887df0 ; core::ptr::drop_in_place<serde_json::map::IntoIter>
00000001415bbf23: lea rcx, [rbp + 0x100]
00000001415bbf2a: call 0x14088e430 ; core::ptr::drop_in_place<enum2$<core::option::Option<enum2$<serde_json::value::Value> > > >
00000001415bbf2f: mov rax, rsi
00000001415bbf32: movaps xmm6, xmmword ptr [rbp + 0x1d0]
00000001415bbf39: movaps xmm7, xmmword ptr [rbp + 0x1e0]
00000001415bbf40: movaps xmm8, xmmword ptr [rbp + 0x1f0]
00000001415bbf48: movaps xmm9, xmmword ptr [rbp + 0x200]
00000001415bbf50: add rsp, 0x298
00000001415bbf57: pop rbx
00000001415bbf58: pop rdi
00000001415bbf59: pop rsi
00000001415bbf5a: pop r12
00000001415bbf5c: pop r13
00000001415bbf5e: pop r14
00000001415bbf60: pop r15
00000001415bbf62: pop rbp
00000001415bbf63: ret 
00000001415bbf64: mov byte ptr [rbp + 0x1c7], 1
00000001415bbf6b: lea rcx, [rip + 0x16bf1fa] ; 142c7b16c 72656d6f7665645f737570706f7274737374727563742057616c6c4174746163686d656e74416e63686f722077697468203620656c656d656e7473007cb1c742010000002b0000000000000077616c6c b'removed_supportsstruct WallAttachmentAnchor with 6 elements\x00|\xb1\xc7B\x01\x00\x00\x00+\x00\x00\x00\x00\x00\x00\x00wall'
00000001415bbf72: mov edx, 0x10
00000001415bbf77: call 0x1428f9db0 ; serde::de::Error::duplicate_field<serde_json::error::Error>
00000001415bbf7c: jmp 0x1415bbeab
00000001415bbf81: mov byte ptr [rbp + 0x1c7], 1
00000001415bbf88: lea rcx, [rip + 0x16bf194] ; 142c7b123 69735f656e636c6f736564617263685f686569676874617263685f74797374796c657374727563747572655f7479636c75747465725f76657273696f6e706c61737465725f7065656c72656d6f766564 b'is_enclosedarch_heightarch_tystylestructure_tyclutter_versionplaster_peelremoved'
00000001415bbf8f: mov edx, 0xb
00000001415bbf94: call 0x1428f9db0 ; serde::de::Error::duplicate_field<serde_json::error::Error>
00000001415bbf99: jmp 0x1415bbeab
00000001415bbf9e: mov byte ptr [rbp + 0x1c7], 1
00000001415bbfa5: lea rcx, [rip + 0x16bf18d] ; 142c7b139 617263685f74797374796c657374727563747572655f7479636c75747465725f76657273696f6e706c61737465725f7065656c72656d6f7665645f737570706f7274737374727563742057616c6c4174 b'arch_tystylestructure_tyclutter_versionplaster_peelremoved_supportsstruct WallAt'
00000001415bbfac: mov edx, 7
00000001415bbfb1: call 0x1428f9db0 ; serde::de::Error::duplicate_field<serde_json::error::Error>
00000001415bbfb6: jmp 0x1415bbeab
00000001415bbfbb: mov byte ptr [rbp + 0x1c7], 1
00000001415bbfc2: lea rcx, [rip + 0x16bf197] ; 142c7b160 706c61737465725f7065656c72656d6f7665645f737570706f7274737374727563742057616c6c4174746163686d656e74416e63686f722077697468203620656c656d656e7473007cb1c74201000000 b'plaster_peelremoved_supportsstruct WallAttachmentAnchor with 6 elements\x00|\xb1\xc7B\x01\x00\x00\x00'
00000001415bbfc9: mov edx, 0xc
00000001415bbfce: call 0x1428f9db0 ; serde::de::Error::duplicate_field<serde_json::error::Error>
00000001415bbfd3: jmp 0x1415bbeab
00000001415bbfd8: mov byte ptr [rbp + 0x1c7], 1
00000001415bbfdf: lea rcx, [rip + 0x16be9c9] ; 142c7a9af 656c65766174696f6e6265666f72655f72656d6f7665645f737570706f72747361667465725f72656d6f7665645f737570706f7274736f726967696e616c746172676574656e636c6f737572655f7061 b'elevationbefore_removed_supportsafter_removed_supportsoriginaltargetenclosure_pa'
00000001415bbfe6: mov edx, 9
00000001415bbfeb: call 0x1428f9db0 ; serde::de::Error::duplicate_field<serde_json::error::Error>
00000001415bbff0: jmp 0x1415bbeab
00000001415bbff5: mov byte ptr [rbp + 0x1c7], 1
00000001415bbffc: lea rcx, [rip + 0x16bf12b] ; 142c7b12e 617263685f686569676874617263685f74797374796c657374727563747572655f7479636c75747465725f76657273696f6e706c61737465725f7065656c72656d6f7665645f737570706f7274737374 b'arch_heightarch_tystylestructure_tyclutter_versionplaster_peelremoved_supportsst'
00000001415bc003: mov edx, 0xb
00000001415bc008: call 0x1428f9db0 ; serde::de::Error::duplicate_field<serde_json::error::Error>
00000001415bc00d: jmp 0x1415bbeab
00000001415bc012: mov byte ptr [rbp + 0x1c7], 1
00000001415bc019: lea rcx, [rip + 0x16bf131] ; 142c7b151 636c75747465725f76657273696f6e706c61737465725f7065656c72656d6f7665645f737570706f7274737374727563742057616c6c4174746163686d656e74416e63686f722077697468203620656c b'clutter_versionplaster_peelremoved_supportsstruct WallAttachmentAnchor with 6 el'
00000001415bc020: mov edx, 0xf
00000001415bc025: call 0x1428f9db0 ; serde::de::Error::duplicate_field<serde_json::error::Error>
00000001415bc02a: jmp 0x1415bbeab
00000001415bc02f: mov byte ptr [rbp + 0x1c7], 1
00000001415bc036: lea rcx, [rip + 0x16bf0db] ; 142c7b118 637572766568656967687469735f656e636c6f736564617263685f686569676874617263685f74797374796c657374727563747572655f7479636c75747465725f76657273696f6e706c61737465725f b'curveheightis_enclosedarch_heightarch_tystylestructure_tyclutter_versionplaster_'
00000001415bc03d: mov edx, 5
00000001415bc042: call 0x1428f9db0 ; serde::de::Error::duplicate_field<serde_json::error::Error>
00000001415bc047: jmp 0x1415bbeab
00000001415bc04c: mov byte ptr [rbp + 0x1c7], 1
00000001415bc053: lea rcx, [rip + 0x16be3c2] ; 142c7a41c 6672656568616e645f747968616c6674696d6265726c616e7465726e5f6c6967687466656e63657377696e646f775f6c69676874666c616777696e646f77736761746573666c61745f72696d5f737479 b'freehand_tyhalftimberlantern_lightfenceswindow_lightflagwindowsgatesflat_rim_sty'
00000001415bc05a: mov edx, 0xb
00000001415bc05f: call 0x1428f9db0 ; serde::de::Error::duplicate_field<serde_json::error::Error>
00000001415bc064: jmp 0x1415bbeab
00000001415bc069: mov byte ptr [rbp + 0x1c7], 1
00000001415bc070: lea rcx, [rip + 0x16bf0a6] ; 142c7b11d 68656967687469735f656e636c6f736564617263685f686569676874617263685f74797374796c657374727563747572655f7479636c75747465725f76657273696f6e706c61737465725f7065656c72 b'heightis_enclosedarch_heightarch_tystylestructure_tyclutter_versionplaster_peelr'
00000001415bc077: mov edx, 6
00000001415bc07c: call 0x1428f9db0 ; serde::de::Error::duplicate_field<serde_json::error::Error>
00000001415bc081: jmp 0x1415bbeab
00000001415bc086: mov byte ptr [rbp + 0x1c7], 1
00000001415bc08d: lea rcx, [rip + 0x16bf0b1] ; 142c7b145 7374727563747572655f7479636c75747465725f76657273696f6e706c61737465725f7065656c72656d6f7665645f737570706f7274737374727563742057616c6c4174746163686d656e74416e6368 b'structure_tyclutter_versionplaster_peelremoved_supportsstruct WallAttachmentAnch'
00000001415bc094: mov edx, 0xc
00000001415bc099: call 0x1428f9db0 ; serde::de::Error::duplicate_field<serde_json::error::Error>
00000001415bc09e: jmp 0x1415bbeab
00000001415bc0a3: mov byte ptr [rbp + 0x1c7], 1
00000001415bc0aa: lea rcx, [rip + 0x16bf08f] ; 142c7b140 7374796c657374727563747572655f7479636c75747465725f76657273696f6e706c61737465725f7065656c72656d6f7665645f737570706f7274737374727563742057616c6c4174746163686d656e b'stylestructure_tyclutter_versionplaster_peelremoved_supportsstruct WallAttachmen'
00000001415bc0b1: mov edx, 5
00000001415bc0b6: call 0x1428f9db0 ; serde::de::Error::duplicate_field<serde_json::error::Error>
00000001415bc0bb: jmp 0x1415bbeab
00000001415bc0c0: lea rdx, [rip + 0x16bdfd1] ; 142c7a098 80a0c742010000001500000000000000000000000000000000000000000000000100000000000000e084964001000000000000000000000000000000000000000100000000000000e084964001000000 b'\x80\xa0\xc7B\x01\x00\x00\x00\x15\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\xe0\x84\x96@\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\xe0\x84\x96@\x01\x00\x00\x00'
00000001415bc0c7: lea r8, [rip + 0x16bdf92] ; 142c7a060 00000000000000001000000000000000080000000000000070a96a4001000000666577657220656c656d656e747320696e206d617000000080a0c7420100000015000000000000000000000000000000 b'\x00\x00\x00\x00\x00\x00\x00\x00\x10\x00\x00\x00\x00\x00\x00\x00\x08\x00\x00\x00\x00\x00\x00\x00p\xa9j@\x01\x00\x00\x00fewer elements in map\x00\x00\x00\x80\xa0\xc7B\x01\x00\x00\x00\x15\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00'
00000001415bc0ce: mov rcx, qword ptr [rbp + 0x28]
00000001415bc0d2: call 0x1428f9d20 ; serde::de::Error::invalid_length<serde_json::error::Error>
00000001415bc0d7: mov rsi, qword ptr [rbp + 0x120]
00000001415bc0de: mov qword ptr [rsi + 8], rax
00000001415bc0e2: movabs rax, 0x8000000000000000
00000001415bc0ec: mov qword ptr [rsi], rax
00000001415bc0ef: lea rcx, [rbp + 0x40]
00000001415bc0f3: call 0x1405d24f0 ; core::ptr::drop_in_place<utils::curve::Curve<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D> >
00000001415bc0f8: jmp 0x1415bbd2f
00000001415bc0fd: nop dword ptr [rax]
00000001415bc100: mov qword ptr [rsp + 0x10], rdx
00000001415bc105: push rbp
00000001415bc106: push r15
00000001415bc108: push r14
00000001415bc10a: push r13
00000001415bc10c: push r12
00000001415bc10e: push rsi
00000001415bc10f: push rdi
00000001415bc110: push rbx
00000001415bc111: sub rsp, 0x68
00000001415bc115: lea rbp, [rdx + 0x80]
00000001415bc11c: movaps xmmword ptr [rsp + 0x20], xmm9
00000001415bc122: movaps xmmword ptr [rsp + 0x30], xmm8
00000001415bc128: movaps xmmword ptr [rsp + 0x40], xmm7
00000001415bc12d: movaps xmmword ptr [rsp + 0x50], xmm6
00000001415bc132: lea rcx, [rbp + 0x40]
00000001415bc136: call 0x1405d24f0 ; core::ptr::drop_in_place<utils::curve::Curve<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D> >
00000001415bc13b: movaps xmm6, xmmword ptr [rsp + 0x50]
00000001415bc140: movaps xmm7, xmmword ptr [rsp + 0x40]
00000001415bc145: movaps xmm8, xmmword ptr [rsp + 0x30]
00000001415bc14b: movaps xmm9, xmmword ptr [rsp + 0x20]
00000001415bc151: add rsp, 0x68
00000001415bc155: pop rbx
00000001415bc156: pop rdi
00000001415bc157: pop rsi
00000001415bc158: pop r12
00000001415bc15a: pop r13
00000001415bc15c: pop r14
00000001415bc15e: pop r15
00000001415bc160: pop rbp
00000001415bc161: ret 
00000001415bc162: nop word ptr cs:[rax + rax]
00000001415bc170: mov qword ptr [rsp + 0x10], rdx
00000001415bc175: push rbp
00000001415bc176: push r15
00000001415bc178: push r14
00000001415bc17a: push r13
00000001415bc17c: push r12
00000001415bc17e: push rsi
00000001415bc17f: push rdi
00000001415bc180: push rbx
00000001415bc181: sub rsp, 0x68
00000001415bc185: lea rbp, [rdx + 0x80]
00000001415bc18c: movaps xmmword ptr [rsp + 0x20], xmm9
00000001415bc192: movaps xmmword ptr [rsp + 0x30], xmm8
00000001415bc198: movaps xmmword ptr [rsp + 0x40], xmm7
00000001415bc19d: movaps xmmword ptr [rsp + 0x50], xmm6
00000001415bc1a2: lea rcx, [rbp + 0x100]
00000001415bc1a9: call 0x14088e430 ; core::ptr::drop_in_place<enum2$<core::option::Option<enum2$<serde_json::value::Value> > > >
00000001415bc1ae: movaps xmm6, xmmword ptr [rsp + 0x50]
00000001415bc1b3: movaps xmm7, xmmword ptr [rsp + 0x40]
00000001415bc1b8: movaps xmm8, xmmword ptr [rsp + 0x30]
00000001415bc1be: movaps xmm9, xmmword ptr [rsp + 0x20]
00000001415bc1c4: add rsp, 0x68
00000001415bc1c8: pop rbx
00000001415bc1c9: pop rdi
00000001415bc1ca: pop rsi
00000001415bc1cb: pop r12
00000001415bc1cd: pop r13
00000001415bc1cf: pop r14
00000001415bc1d1: pop r15
00000001415bc1d3: pop rbp
00000001415bc1d4: ret 
00000001415bc1d5: nop word ptr cs:[rax + rax]
00000001415bc1e0: mov qword ptr [rsp + 0x10], rdx
00000001415bc1e5: push rbp
00000001415bc1e6: push r15
00000001415bc1e8: push r14
00000001415bc1ea: push r13
00000001415bc1ec: push r12
00000001415bc1ee: push rsi
00000001415bc1ef: push rdi
00000001415bc1f0: push rbx
00000001415bc1f1: sub rsp, 0x68
00000001415bc1f5: lea rbp, [rdx + 0x80]
00000001415bc1fc: movaps xmmword ptr [rsp + 0x20], xmm9
00000001415bc202: movaps xmmword ptr [rsp + 0x30], xmm8
00000001415bc208: movaps xmmword ptr [rsp + 0x40], xmm7
00000001415bc20d: movaps xmmword ptr [rsp + 0x50], xmm6
00000001415bc212: lea rcx, [rbp + 0x130]
00000001415bc219: call 0x1405d24f0 ; core::ptr::drop_in_place<utils::curve::Curve<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D> >
00000001415bc21e: mov byte ptr [rbp + 0x1c7], 0
00000001415bc225: movaps xmm6, xmmword ptr [rsp + 0x50]
00000001415bc22a: movaps xmm7, xmmword ptr [rsp + 0x40]
00000001415bc22f: movaps xmm8, xmmword ptr [rsp + 0x30]
00000001415bc235: movaps xmm9, xmmword ptr [rsp + 0x20]
00000001415bc23b: add rsp, 0x68
00000001415bc23f: pop rbx
00000001415bc240: pop rdi
00000001415bc241: pop rsi
00000001415bc242: pop r12
00000001415bc244: pop r13
00000001415bc246: pop r14
00000001415bc248: pop r15
00000001415bc24a: pop rbp
00000001415bc24b: ret 
00000001415bc24c: nop dword ptr [rax]
00000001415bc250: mov qword ptr [rsp + 0x10], rdx
00000001415bc255: push rbp
00000001415bc256: push r15
00000001415bc258: push r14
00000001415bc25a: push r13
00000001415bc25c: push r12
00000001415bc25e: push rsi
00000001415bc25f: push rdi
00000001415bc260: push rbx
00000001415bc261: sub rsp, 0x68
00000001415bc265: lea rbp, [rdx + 0x80]
00000001415bc26c: movaps xmmword ptr [rsp + 0x20], xmm9
00000001415bc272: movaps xmmword ptr [rsp + 0x30], xmm8
00000001415bc278: movaps xmmword ptr [rsp + 0x40], xmm7
00000001415bc27d: movaps xmmword ptr [rsp + 0x50], xmm6
00000001415bc282: mov rdx, qword ptr [rbp + 0x108]
00000001415bc289: test rdx, rdx
00000001415bc28c: je 0x1415bc2a4
00000001415bc28e: shl rdx, 5
00000001415bc292: mov r8d, 8
00000001415bc298: mov rcx, qword ptr [rbp + 0xb0]
00000001415bc29f: call 0x140613c20 ; 140613c20
00000001415bc2a4: movaps xmm6, xmmword ptr [rsp + 0x50]
00000001415bc2a9: movaps xmm7, xmmword ptr [rsp + 0x40]
00000001415bc2ae: movaps xmm8, xmmword ptr [rsp + 0x30]
00000001415bc2b4: movaps xmm9, xmmword ptr [rsp + 0x20]
00000001415bc2ba: add rsp, 0x68
00000001415bc2be: pop rbx
00000001415bc2bf: pop rdi
00000001415bc2c0: pop rsi
00000001415bc2c1: pop r12
00000001415bc2c3: pop r13
00000001415bc2c5: pop r14
00000001415bc2c7: pop r15
00000001415bc2c9: pop rbp
00000001415bc2ca: ret 
00000001415bc2cb: nop dword ptr [rax + rax]
00000001415bc2d0: mov qword ptr [rsp + 0x10], rdx
00000001415bc2d5: push rbp
00000001415bc2d6: push r15
00000001415bc2d8: push r14
00000001415bc2da: push r13
00000001415bc2dc: push r12
00000001415bc2de: push rsi
00000001415bc2df: push rdi
00000001415bc2e0: push rbx
00000001415bc2e1: sub rsp, 0x68
00000001415bc2e5: lea rbp, [rdx + 0x80]
00000001415bc2ec: movaps xmmword ptr [rsp + 0x20], xmm9
00000001415bc2f2: movaps xmmword ptr [rsp + 0x30], xmm8
00000001415bc2f8: movaps xmmword ptr [rsp + 0x40], xmm7
00000001415bc2fd: movaps xmmword ptr [rsp + 0x50], xmm6
00000001415bc302: lea rcx, [rbp + 0x100]
00000001415bc309: call 0x14088e430 ; core::ptr::drop_in_place<enum2$<core::option::Option<enum2$<serde_json::value::Value> > > >
00000001415bc30e: movaps xmm6, xmmword ptr [rsp + 0x50]
00000001415bc313: movaps xmm7, xmmword ptr [rsp + 0x40]
00000001415bc318: movaps xmm8, xmmword ptr [rsp + 0x30]
00000001415bc31e: movaps xmm9, xmmword ptr [rsp + 0x20]
00000001415bc324: add rsp, 0x68
00000001415bc328: pop rbx
00000001415bc329: pop rdi
00000001415bc32a: pop rsi
00000001415bc32b: pop r12
00000001415bc32d: pop r13
00000001415bc32f: pop r14
00000001415bc331: pop r15
00000001415bc333: pop rbp
00000001415bc334: ret 
00000001415bc335: nop word ptr cs:[rax + rax]
00000001415bc340: mov qword ptr [rsp + 0x10], rdx
00000001415bc345: push rbp
00000001415bc346: push r15
00000001415bc348: push r14
00000001415bc34a: push r13
00000001415bc34c: push r12
00000001415bc34e: push rsi
00000001415bc34f: push rdi
00000001415bc350: push rbx
00000001415bc351: sub rsp, 0x68
00000001415bc355: lea rbp, [rdx + 0x80]
00000001415bc35c: movaps xmmword ptr [rsp + 0x20], xmm9
00000001415bc362: movaps xmmword ptr [rsp + 0x30], xmm8
00000001415bc368: movaps xmmword ptr [rsp + 0x40], xmm7
00000001415bc36d: movaps xmmword ptr [rsp + 0x50], xmm6
00000001415bc372: movaps xmm0, xmmword ptr [rbp + 0x130]
00000001415bc379: movaps xmm1, xmmword ptr [rbp + 0x140]
00000001415bc380: lea rax, [rbp + 0x100]
00000001415bc387: movups xmmword ptr [rax + 0x10], xmm1
00000001415bc38b: movups xmmword ptr [rax], xmm0
00000001415bc38e: mov byte ptr [rbp + 0x1b5], 1
00000001415bc395: movaps xmm6, xmmword ptr [rsp + 0x50]
00000001415bc39a: movaps xmm7, xmmword ptr [rsp + 0x40]
00000001415bc39f: movaps xmm8, xmmword ptr [rsp + 0x30]
00000001415bc3a5: movaps xmm9, xmmword ptr [rsp + 0x20]
00000001415bc3ab: add rsp, 0x68
00000001415bc3af: pop rbx
00000001415bc3b0: pop rdi
00000001415bc3b1: pop rsi
00000001415bc3b2: pop r12
00000001415bc3b4: pop r13
00000001415bc3b6: pop r14
00000001415bc3b8: pop r15
00000001415bc3ba: pop rbp
00000001415bc3bb: ret 
00000001415bc3bc: nop dword ptr [rax]
00000001415bc3c0: mov qword ptr [rsp + 0x10], rdx
00000001415bc3c5: push rbp
00000001415bc3c6: push r15
00000001415bc3c8: push r14
00000001415bc3ca: push r13
00000001415bc3cc: push r12
00000001415bc3ce: push rsi
00000001415bc3cf: push rdi
00000001415bc3d0: push rbx
00000001415bc3d1: sub rsp, 0x68
00000001415bc3d5: lea rbp, [rdx + 0x80]
00000001415bc3dc: movaps xmmword ptr [rsp + 0x20], xmm9
00000001415bc3e2: movaps xmmword ptr [rsp + 0x30], xmm8
00000001415bc3e8: movaps xmmword ptr [rsp + 0x40], xmm7
00000001415bc3ed: movaps xmmword ptr [rsp + 0x50], xmm6
00000001415bc3f2: cmp qword ptr [rbp + 0x170], 0
00000001415bc3fa: je 0x1415bc415
00000001415bc3fc: mov r8d, 1
00000001415bc402: mov rcx, qword ptr [rbp + 0x128]
00000001415bc409: mov rdx, qword ptr [rbp + 0x170]
00000001415bc410: call 0x140613c20 ; 140613c20
00000001415bc415: mov byte ptr [rbp + 0x1b5], 0
00000001415bc41c: movaps xmm6, xmmword ptr [rsp + 0x50]
00000001415bc421: movaps xmm7, xmmword ptr [rsp + 0x40]
00000001415bc426: movaps xmm8, xmmword ptr [rsp + 0x30]
00000001415bc42c: movaps xmm9, xmmword ptr [rsp + 0x20]
00000001415bc432: add rsp, 0x68
00000001415bc436: pop rbx
00000001415bc437: pop rdi
00000001415bc438: pop rsi
00000001415bc439: pop r12
00000001415bc43b: pop r13
00000001415bc43d: pop r14
00000001415bc43f: pop r15
00000001415bc441: pop rbp
00000001415bc442: ret 
00000001415bc443: nop word ptr cs:[rax + rax]
00000001415bc450: mov qword ptr [rsp + 0x10], rdx
00000001415bc455: push rbp
00000001415bc456: push r15
00000001415bc458: push r14
00000001415bc45a: push r13
00000001415bc45c: push r12
00000001415bc45e: push rsi
00000001415bc45f: push rdi
00000001415bc460: push rbx
00000001415bc461: sub rsp, 0x68
00000001415bc465: lea rbp, [rdx + 0x80]
00000001415bc46c: movaps xmmword ptr [rsp + 0x20], xmm9
00000001415bc472: movaps xmmword ptr [rsp + 0x30], xmm8
00000001415bc478: movaps xmmword ptr [rsp + 0x40], xmm7
00000001415bc47d: movaps xmmword ptr [rsp + 0x50], xmm6
00000001415bc482: cmp byte ptr [rbp + 0x1b5], 0
00000001415bc489: je 0x1415bc4dc
00000001415bc48b: cmp qword ptr [rbp + 0x170], 0
00000001415bc493: je 0x1415bc4ae
00000001415bc495: mov r8d, 1
00000001415bc49b: mov rcx, qword ptr [rbp + 0x128]
00000001415bc4a2: mov rdx, qword ptr [rbp + 0x170]
00000001415bc4a9: call 0x140613c20 ; 140613c20
00000001415bc4ae: mov byte ptr [rbp + 0x1c7], 1
00000001415bc4b5: movaps xmm6, xmmword ptr [rsp + 0x50]
00000001415bc4ba: movaps xmm7, xmmword ptr [rsp + 0x40]
00000001415bc4bf: movaps xmm8, xmmword ptr [rsp + 0x30]
00000001415bc4c5: movaps xmm9, xmmword ptr [rsp + 0x20]
00000001415bc4cb: add rsp, 0x68
00000001415bc4cf: pop rbx
00000001415bc4d0: pop rdi
00000001415bc4d1: pop rsi
00000001415bc4d2: pop r12
00000001415bc4d4: pop r13
00000001415bc4d6: pop r14
00000001415bc4d8: pop r15
00000001415bc4da: pop rbp
00000001415bc4db: ret 
00000001415bc4dc: mov byte ptr [rbp + 0x1c7], 1
00000001415bc4e3: movaps xmm6, xmmword ptr [rsp + 0x50]
00000001415bc4e8: movaps xmm7, xmmword ptr [rsp + 0x40]
00000001415bc4ed: movaps xmm8, xmmword ptr [rsp + 0x30]
00000001415bc4f3: movaps xmm9, xmmword ptr [rsp + 0x20]
00000001415bc4f9: add rsp, 0x68
00000001415bc4fd: pop rbx
00000001415bc4fe: pop rdi
00000001415bc4ff: pop rsi
00000001415bc500: pop r12
00000001415bc502: pop r13
00000001415bc504: pop r14
00000001415bc506: pop r15
00000001415bc508: pop rbp
00000001415bc509: ret 
00000001415bc50a: nop word ptr [rax + rax]
00000001415bc510: mov qword ptr [rsp + 0x10], rdx
00000001415bc515: push rbp
00000001415bc516: push r15
00000001415bc518: push r14
00000001415bc51a: push r13
00000001415bc51c: push r12
00000001415bc51e: push rsi
00000001415bc51f: push rdi
00000001415bc520: push rbx
00000001415bc521: sub rsp, 0x68
00000001415bc525: lea rbp, [rdx + 0x80]
00000001415bc52c: movaps xmmword ptr [rsp + 0x20], xmm9
00000001415bc532: movaps xmmword ptr [rsp + 0x30], xmm8
00000001415bc538: movaps xmmword ptr [rsp + 0x40], xmm7
00000001415bc53d: movaps xmmword ptr [rsp + 0x50], xmm6
00000001415bc542: neg qword ptr [rbp + 0x1b8]
00000001415bc549: jo 0x1415bc584
00000001415bc54b: test byte ptr [rbp + 0x1c7], 1
00000001415bc552: je 0x1415bc5ab
00000001415bc554: lea rcx, [rbp - 0x10]
00000001415bc558: call 0x1405d24f0 ; core::ptr::drop_in_place<utils::curve::Curve<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D> >
00000001415bc55d: movaps xmm6, xmmword ptr [rsp + 0x50]
00000001415bc562: movaps xmm7, xmmword ptr [rsp + 0x40]
00000001415bc567: movaps xmm8, xmmword ptr [rsp + 0x30]
00000001415bc56d: movaps xmm9, xmmword ptr [rsp + 0x20]
00000001415bc573: add rsp, 0x68
00000001415bc577: pop rbx
00000001415bc578: pop rdi
00000001415bc579: pop rsi
00000001415bc57a: pop r12
00000001415bc57c: pop r13
00000001415bc57e: pop r14
00000001415bc580: pop r15
00000001415bc582: pop rbp
00000001415bc583: ret 
00000001415bc584: movaps xmm6, xmmword ptr [rsp + 0x50]
00000001415bc589: movaps xmm7, xmmword ptr [rsp + 0x40]
00000001415bc58e: movaps xmm8, xmmword ptr [rsp + 0x30]
00000001415bc594: movaps xmm9, xmmword ptr [rsp + 0x20]
00000001415bc59a: add rsp, 0x68
00000001415bc59e: pop rbx
00000001415bc59f: pop rdi
00000001415bc5a0: pop rsi
00000001415bc5a1: pop r12
00000001415bc5a3: pop r13
00000001415bc5a5: pop r14
00000001415bc5a7: pop r15
00000001415bc5a9: pop rbp
00000001415bc5aa: ret 
00000001415bc5ab: movaps xmm6, xmmword ptr [rsp + 0x50]
00000001415bc5b0: movaps xmm7, xmmword ptr [rsp + 0x40]
00000001415bc5b5: movaps xmm8, xmmword ptr [rsp + 0x30]
00000001415bc5bb: movaps xmm9, xmmword ptr [rsp + 0x20]
00000001415bc5c1: add rsp, 0x68
00000001415bc5c5: pop rbx
00000001415bc5c6: pop rdi
00000001415bc5c7: pop rsi
00000001415bc5c8: pop r12
00000001415bc5ca: pop r13
00000001415bc5cc: pop r14
00000001415bc5ce: pop r15
00000001415bc5d0: pop rbp
00000001415bc5d1: ret 
00000001415bc5d2: nop word ptr cs:[rax + rax]
00000001415bc5e0: mov qword ptr [rsp + 0x10], rdx
00000001415bc5e5: push rbp
00000001415bc5e6: push r15
00000001415bc5e8: push r14
00000001415bc5ea: push r13
00000001415bc5ec: push r12
00000001415bc5ee: push rsi
00000001415bc5ef: push rdi
00000001415bc5f0: push rbx
00000001415bc5f1: sub rsp, 0x68
00000001415bc5f5: lea rbp, [rdx + 0x80]
00000001415bc5fc: movaps xmmword ptr [rsp + 0x20], xmm9
00000001415bc602: movaps xmmword ptr [rsp + 0x30], xmm8
00000001415bc608: movaps xmmword ptr [rsp + 0x40], xmm7
00000001415bc60d: movaps xmmword ptr [rsp + 0x50], xmm6
00000001415bc612: lea rcx, [rbp + 0xb8]
00000001415bc619: call 0x14088a500 ; core::ptr::drop_in_place<serde_json::value::de::MapDeserializer>
00000001415bc61e: movaps xmm6, xmmword ptr [rsp + 0x50]
00000001415bc623: movaps xmm7, xmmword ptr [rsp + 0x40]
00000001415bc628: movaps xmm8, xmmword ptr [rsp + 0x30]
00000001415bc62e: movaps xmm9, xmmword ptr [rsp + 0x20]
00000001415bc634: add rsp, 0x68
00000001415bc638: pop rbx
00000001415bc639: pop rdi
00000001415bc63a: pop rsi
00000001415bc63b: pop r12
00000001415bc63d: pop r13
00000001415bc63f: pop r14
00000001415bc641: pop r15
00000001415bc643: pop rbp
00000001415bc644: ret 
