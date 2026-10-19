1407f7982: 488d04db lea rax, [rbx + rbx*8]
1407f7986: 4c8b44c1d8 mov r8, qword ptr [rcx + rax*8 - 0x28]
1407f798b: f049ff00 lock inc qword ptr [r8]
1407f798f: 0f8e6c0d0000 jle 0x1407f8701
1407f7995: 0f13bdc0010000 movlps qword ptr [rbp + 0x1c0], xmm7
1407f799c: f3440f118dc8010000 movss dword ptr [rbp + 0x1c8], xmm9
1407f79a5: 440f295510 movaps xmmword ptr [rbp + 0x10], xmm10
1407f79aa: c6851702000000 mov byte ptr [rbp + 0x217], 0
1407f79b1: 488d4510 lea rax, [rbp + 0x10]
1407f79b5: 4889442420 mov qword ptr [rsp + 0x20], rax
1407f79ba: 488b8d00020000 mov rcx, qword ptr [rbp + 0x200]
1407f79c1: 488b95d8000000 mov rdx, qword ptr [rbp + 0xd8]
1407f79c8: 4c8d8dc0010000 lea r9, [rbp + 0x1c0]
1407f79cf: e8dcd80200 call 0x1408252b0 ; system_decorator::blueprint::DecoratorBlueprint::set_colliders
1407f79d4: 4c8bb588000000 mov r14, qword ptr [rbp + 0x88]
1407f79db: 488b9508020000 mov rdx, qword ptr [rbp + 0x208]
1407f79e2: 488b85b8010000 mov rax, qword ptr [rbp + 0x1b8]
1407f79e9: 83b85802000002 cmp dword ptr [rax + 0x258], 2
1407f79f0: 0f83e9020000 jae 0x1407f7cdf
1407f79f6: 440fb67afc movzx r15d, byte ptr [rdx - 4]
1407f79fb: 418d47ff lea eax, [r15 - 1]
1407f79ff: 3c03 cmp al, 3
1407f7a01: 0f83b70c0000 jae 0x1407f86be
1407f7a07: 0fb6c0 movzx eax, al
1407f7a0a: 488d0d43492902 lea rcx, [rip + 0x2294943]
1407f7a11: 48631c81 movsxd rbx, dword ptr [rcx + rax*4]
1407f7a15: 488d85c4010000 lea rax, [rbp + 0x1c4]
1407f7a1c: c7400700000000 mov dword ptr [rax + 7], 0
1407f7a23: 48c740ff00000000 mov qword ptr [rax - 1], 0
1407f7a2b: 488d4ae8 lea rcx, [rdx - 0x18]
1407f7a2f: 488d15fa472902 lea rdx, [rip + 0x22947fa]
1407f7a36: e875190400 call 0x1408393b0 ; country_core::resources::walls::decorator::DecoratorExtraInfo::as_window | country_core::resources::walls::decorator::DecoratorExtraInfo::as_window_mut
1407f7a3b: 8b00 mov eax, dword ptr [rax]
1407f7a3d: 8985d8000000 mov dword ptr [rbp + 0xd8], eax
1407f7a43: 488b85c0010000 mov rax, qword ptr [rbp + 0x1c0]
1407f7a4a: 488b8dc7010000 mov rcx, qword ptr [rbp + 0x1c7]
1407f7a51: 48898d7f010000 mov qword ptr [rbp + 0x17f], rcx
1407f7a58: 48898578010000 mov qword ptr [rbp + 0x178], rax
1407f7a5f: 488b8500020000 mov rax, qword ptr [rbp + 0x200]
1407f7a66: 488bb8c0000000 mov rdi, qword ptr [rax + 0xc0]
1407f7a6d: 483bb8b0000000 cmp rdi, qword ptr [rax + 0xb0]
1407f7a74: 7513 jne 0x1407f7a89
1407f7a76: 488b8d98000000 mov rcx, qword ptr [rbp + 0x98]
1407f7a7d: 488d15c4472902 lea rdx, [rip + 0x22947c4]
1407f7a84: e827390300 call 0x14082b3b0 ; alloc::raw_vec::RawVec::grow_one<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder,alloc::alloc::Global>
1407f7a89: 488d05c4482902 lea rax, [rip + 0x22948c4]
1407f7a90: 4801c3 add rbx, rax
1407f7a93: 4969d7e8030000 imul rdx, r15, 0x3e8
1407f7a9a: 48039568010000 add rdx, qword ptr [rbp + 0x168]
1407f7aa1: 4c8b8d00020000 mov r9, qword ptr [rbp + 0x200]
1407f7aa8: 498b81b8000000 mov rax, qword ptr [r9 + 0xb8]
1407f7aaf: 4869cf60010000 imul rcx, rdi, 0x160
1407f7ab6: 48c7040800000000 mov qword ptr [rax + rcx], 0
1407f7abe: 48c744081001000000 mov qword ptr [rax + rcx + 0x10], 1
1407f7ac7: 4889540818 mov qword ptr [rax + rcx + 0x18], rdx
1407f7acc: 48c744082000000000 mov qword ptr [rax + rcx + 0x20], 0
1407f7ad5: c744083000000000 mov dword ptr [rax + rcx + 0x30], 0
1407f7add: 488b555c mov rdx, qword ptr [rbp + 0x5c]
1407f7ae1: 4889540834 mov qword ptr [rax + rcx + 0x34], rdx
1407f7ae6: 8b5564 mov edx, dword ptr [rbp + 0x64]
1407f7ae9: 8954083c mov dword ptr [rax + rcx + 0x3c], edx
1407f7aed: c744084000000000 mov dword ptr [rax + rcx + 0x40], 0
1407f7af5: 0f1085c0010000 movups xmm0, xmmword ptr [rbp + 0x1c0]
1407f7afc: 0f108dd0010000 movups xmm1, xmmword ptr [rbp + 0x1d0]
1407f7b03: f30f6f95e0010000 movdqu xmm2, xmmword ptr [rbp + 0x1e0]
1407f7b0b: 0f109dec010000 movups xmm3, xmmword ptr [rbp + 0x1ec]
1407f7b12: 0f11440844 movups xmmword ptr [rax + rcx + 0x44], xmm0
1407f7b17: 0f114c0854 movups xmmword ptr [rax + rcx + 0x54], xmm1
1407f7b1c: f30f7f540864 movdqu xmmword ptr [rax + rcx + 0x64], xmm2
1407f7b22: 0f115c0870 movups xmmword ptr [rax + rcx + 0x70], xmm3
1407f7b27: c784088000000000000000 mov dword ptr [rax + rcx + 0x80], 0
1407f7b32: 488b5568 mov rdx, qword ptr [rbp + 0x68]
1407f7b36: 4889940884000000 mov qword ptr [rax + rcx + 0x84], rdx
1407f7b3e: 8b5570 mov edx, dword ptr [rbp + 0x70]
1407f7b41: 8994088c000000 mov dword ptr [rax + rcx + 0x8c], edx
1407f7b48: c784089000000000000000 mov dword ptr [rax + rcx + 0x90], 0
1407f7b53: 488b5574 mov rdx, qword ptr [rbp + 0x74]
1407f7b57: 4889940894000000 mov qword ptr [rax + rcx + 0x94], rdx
1407f7b5f: 8b557c mov edx, dword ptr [rbp + 0x7c]
1407f7b62: 8994089c000000 mov dword ptr [rax + rcx + 0x9c], edx
1407f7b69: 48899c08a0000000 mov qword ptr [rax + rcx + 0xa0], rbx
1407f7b71: 48c78408a80000002a000000 mov qword ptr [rax + rcx + 0xa8], 0x2a
1407f7b7d: 488d15c8964202 lea rdx, [rip + 0x24296c8]
1407f7b84: 48899408b0000000 mov qword ptr [rax + rcx + 0xb0], rdx
1407f7b8c: 48c78408b800000024000000 mov qword ptr [rax + rcx + 0xb8], 0x24
1407f7b98: 440f299408c0000000 movaps xmmword ptr [rax + rcx + 0xc0], xmm10
1407f7ba1: 0f2885b0000000 movaps xmm0, xmmword ptr [rbp + 0xb0]
1407f7ba8: 0f138408d0000000 movlps qword ptr [rax + rcx + 0xd0], xmm0
1407f7bb0: f3440f118c08d8000000 movss dword ptr [rax + rcx + 0xd8], xmm9
1407f7bba: 48ba0000803f0000803f movabs rdx, 0x3f8000003f800000
1407f7bc4: 48899408dc000000 mov qword ptr [rax + rcx + 0xdc], rdx
1407f7bcc: c78408e40000000000803f mov dword ptr [rax + rcx + 0xe4], 0x3f800000
1407f7bd7: c78408f000000001000000 mov dword ptr [rax + rcx + 0xf0], 1
1407f7be2: 8b95d8000000 mov edx, dword ptr [rbp + 0xd8]
1407f7be8: 899408f4000000 mov dword ptr [rax + rcx + 0xf4], edx
1407f7bef: c78408f800000000000000 mov dword ptr [rax + rcx + 0xf8], 0
1407f7bfa: 0f108590010000 movups xmm0, xmmword ptr [rbp + 0x190]
1407f7c01: 0f118408fc000000 movups xmmword ptr [rax + rcx + 0xfc], xmm0
1407f7c09: 488b95a0010000 mov rdx, qword ptr [rbp + 0x1a0]
1407f7c10: 488994080c010000 mov qword ptr [rax + rcx + 0x10c], rdx
1407f7c18: c684081401000002 mov byte ptr [rax + rcx + 0x114], 2
1407f7c20: 0fb695b7010000 movzx edx, byte ptr [rbp + 0x1b7]
1407f7c27: 88940817010000 mov byte ptr [rax + rcx + 0x117], dl
1407f7c2e: 0fb795b5010000 movzx edx, word ptr [rbp + 0x1b5]
1407f7c35: 6689940815010000 mov word ptr [rax + rcx + 0x115], dx
1407f7c3d: c784081801000000000000 mov dword ptr [rax + rcx + 0x118], 0
1407f7c48: f30f6f85e0000000 movdqu xmm0, xmmword ptr [rbp + 0xe0]
1407f7c50: f30f6f8df0000000 movdqu xmm1, xmmword ptr [rbp + 0xf0]
1407f7c58: f30f7f8c082c010000 movdqu xmmword ptr [rax + rcx + 0x12c], xmm1
1407f7c61: f30f7f84081c010000 movdqu xmmword ptr [rax + rcx + 0x11c], xmm0
1407f7c6a: c684083c01000002 mov byte ptr [rax + rcx + 0x13c], 2
1407f7c72: 488b9578010000 mov rdx, qword ptr [rbp + 0x178]
1407f7c79: 4c8b857f010000 mov r8, qword ptr [rbp + 0x17f]
1407f7c80: 4c89840844010000 mov qword ptr [rax + rcx + 0x144], r8
1407f7c88: 488994083d010000 mov qword ptr [rax + rcx + 0x13d], rdx
1407f7c90: c684084c01000002 mov byte ptr [rax + rcx + 0x14c], 2
1407f7c98: 0fb69538010000 movzx edx, byte ptr [rbp + 0x138]
1407f7c9f: 8894084e010000 mov byte ptr [rax + rcx + 0x14e], dl
1407f7ca6: 66c784084f0100000009 mov word ptr [rax + rcx + 0x14f], 0x900
1407f7cb0: 488b9510010000 mov rdx, qword ptr [rbp + 0x110]
1407f7cb7: 4c8b8517010000 mov r8, qword ptr [rbp + 0x117]
1407f7cbe: 4889940851010000 mov qword ptr [rax + rcx + 0x151], rdx
1407f7cc6: 4c89840858010000 mov qword ptr [rax + rcx + 0x158], r8
1407f7cce: 48ffc7 inc rdi
1407f7cd1: 4989b9c0000000 mov qword ptr [r9 + 0xc0], rdi
1407f7cd8: 488b9508020000 mov rdx, qword ptr [rbp + 0x208]
