1407ed1aa: 488b9598020000 mov rdx, qword ptr [rbp + 0x298]
1407ed1b1: 0fb65afc movzx ebx, byte ptr [rdx - 4]
1407ed1b5: 8d43ff lea eax, [rbx - 1]
1407ed1b8: 3c03 cmp al, 3
1407ed1ba: 0f835a1c0000 jae 0x1407eee1a
1407ed1c0: 0fb6c0 movzx eax, al
1407ed1c3: 488d0d7adc2902 lea rcx, [rip + 0x229dc7a]
1407ed1ca: 48633c81 movsxd rdi, dword ptr [rcx + rax*4]
1407ed1ce: 488d8513020000 lea rax, [rbp + 0x213]
1407ed1d5: c7400800000000 mov dword ptr [rax + 8], 0
1407ed1dc: 48c70000000000 mov qword ptr [rax], 0
1407ed1e3: 488d4ae8 lea rcx, [rdx - 0x18]
1407ed1e7: 488d15bad42902 lea rdx, [rip + 0x229d4ba]
1407ed1ee: e8bdc10400 call 0x1408393b0 ; country_core::resources::walls::decorator::DecoratorExtraInfo::as_window | country_core::resources::walls::decorator::DecoratorExtraInfo::as_window_mut
1407ed1f3: 8b00 mov eax, dword ptr [rax]
1407ed1f5: 898590020000 mov dword ptr [rbp + 0x290], eax
