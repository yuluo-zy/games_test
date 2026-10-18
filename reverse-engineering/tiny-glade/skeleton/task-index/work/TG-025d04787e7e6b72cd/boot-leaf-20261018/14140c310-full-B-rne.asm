14140c310: 56 push rsi
14140c311: 57 push rdi
14140c312: 53 push rbx
14140c313: 4881ec80000000 sub rsp, 0x80
14140c31a: 4889d0 mov rax, rdx
14140c31d: 4889ce mov rsi, rcx
14140c320: 488b12 mov rdx, qword ptr [rdx]
14140c323: 4c8b5220 mov r10, qword ptr [rdx + 0x20]
14140c327: 488b4a28 mov rcx, qword ptr [rdx + 0x28]
14140c32b: 4c39d1 cmp rcx, r10
14140c32e: 734a jae 0x14140c37a
14140c330: 4c8b4218 mov r8, qword ptr [rdx + 0x18]
14140c334: 48ffc1 inc rcx
14140c337: 4c89d7 mov rdi, r10
14140c33a: 48f7df neg rdi
14140c33d: 49b90026000001000000 movabs r9, 0x100002600
14140c347: 660f1f840000000000 nop word ptr [rax + rax]
14140c350: 450fb65c08ff movzx r11d, byte ptr [r8 + rcx - 1]
14140c356: 4983fb2c cmp r11, 0x2c
14140c35a: 0f8791000000 ja 0x14140c3f1
14140c360: 4d0fa3d9 bt r9, r11
14140c364: 731f jae 0x14140c385
14140c366: 48894a28 mov qword ptr [rdx + 0x28], rcx
14140c36a: 4c8d1c0f lea r11, [rdi + rcx]
14140c36e: 49ffc3 inc r11
14140c371: 48ffc1 inc rcx
14140c374: 4983fb01 cmp r11, 1
14140c378: 75d6 jne 0x14140c350
14140c37a: 48c744243002000000 mov qword ptr [rsp + 0x30], 2
14140c383: eb5a jmp 0x14140c3df
14140c385: 4983fb2c cmp r11, 0x2c
14140c389: 7566 jne 0x14140c3f1
14140c38b: 0fb65808 movzx ebx, byte ptr [rax + 8]
14140c38f: 84db test bl, bl
14140c391: 7574 jne 0x14140c407
14140c393: 48894a28 mov qword ptr [rdx + 0x28], rcx
14140c397: 4c39d1 cmp rcx, r10
14140c39a: 733a jae 0x14140c3d6
14140c39c: 48ffc1 inc rcx
14140c39f: b801000000 mov eax, 1
14140c3a4: 4c29d0 sub rax, r10
14140c3a7: 660f1f840000000000 nop word ptr [rax + rax]
14140c3b0: 450fb65c08ff movzx r11d, byte ptr [r8 + rcx - 1]
14140c3b6: 4983fb20 cmp r11, 0x20
14140c3ba: 7754 ja 0x14140c410
14140c3bc: 4d0fa3d9 bt r9, r11
14140c3c0: 734e jae 0x14140c410
14140c3c2: 48894a28 mov qword ptr [rdx + 0x28], rcx
14140c3c6: 4c8d1408 lea r10, [rax + rcx]
14140c3ca: 49ffc2 inc r10
14140c3cd: 48ffc1 inc rcx
14140c3d0: 4983fa02 cmp r10, 2
14140c3d4: 75da jne 0x14140c3b0
14140c3d6: 48c744243005000000 mov qword ptr [rsp + 0x30], 5
14140c3df: 488d442430 lea rax, [rsp + 0x30]
14140c3e4: 4889d1 mov rcx, rdx
14140c3e7: 4889c2 mov rdx, rax
14140c3ea: e831454901 call 0x1428a0920 ; serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead>
14140c3ef: eb3d jmp 0x14140c42e
14140c3f1: 4183fb5d cmp r11d, 0x5d
14140c3f5: 750c jne 0x14140c403
14140c3f7: 48b80000000000000080 movabs rax, 0x8000000000000000
14140c401: eb39 jmp 0x14140c43c
14140c403: 0fb65808 movzx ebx, byte ptr [rax + 8]
14140c407: f6c301 test bl, 1
14140c40a: 747d je 0x14140c489
14140c40c: c6400800 mov byte ptr [rax + 8], 0
14140c410: 4180fb5d cmp r11b, 0x5d
14140c414: 7465 je 0x14140c47b
14140c416: 488d4c2430 lea rcx, [rsp + 0x30]
14140c41b: e800a10000 call 0x141416520 ; serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::startup::clearing::_::impl$0::deserialize::__Visitor>
14140c420: 31c0 xor eax, eax
14140c422: 483b442430 cmp rax, qword ptr [rsp + 0x30]
14140c427: 7118 jno 0x14140c441
14140c429: 488b442438 mov rax, qword ptr [rsp + 0x38]
14140c42e: 48894608 mov qword ptr [rsi + 8], rax
14140c432: 48b80100000000000080 movabs rax, 0x8000000000000001
14140c43c: 488906 mov qword ptr [rsi], rax
14140c43f: eb2c jmp 0x14140c46d
14140c441: 0f10442470 movups xmm0, xmmword ptr [rsp + 0x70]
14140c446: 0f114640 movups xmmword ptr [rsi + 0x40], xmm0
14140c44a: 0f10442430 movups xmm0, xmmword ptr [rsp + 0x30]
14140c44f: 0f104c2440 movups xmm1, xmmword ptr [rsp + 0x40]
14140c454: 0f10542450 movups xmm2, xmmword ptr [rsp + 0x50]
14140c459: 0f105c2460 movups xmm3, xmmword ptr [rsp + 0x60]
14140c45e: 0f115e30 movups xmmword ptr [rsi + 0x30], xmm3
14140c462: 0f115620 movups xmmword ptr [rsi + 0x20], xmm2
14140c466: 0f114e10 movups xmmword ptr [rsi + 0x10], xmm1
14140c46a: 0f1106 movups xmmword ptr [rsi], xmm0
14140c46d: 4889f0 mov rax, rsi
14140c470: 4881c480000000 add rsp, 0x80
14140c477: 5b pop rbx
14140c478: 5f pop rdi
14140c479: 5e pop rsi
14140c47a: c3 ret 
14140c47b: 48c744243012000000 mov qword ptr [rsp + 0x30], 0x12
14140c484: e956ffffff jmp 0x14140c3df
14140c489: 48c744243007000000 mov qword ptr [rsp + 0x30], 7
14140c492: e948ffffff jmp 0x14140c3df
