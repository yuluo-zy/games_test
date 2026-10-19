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
