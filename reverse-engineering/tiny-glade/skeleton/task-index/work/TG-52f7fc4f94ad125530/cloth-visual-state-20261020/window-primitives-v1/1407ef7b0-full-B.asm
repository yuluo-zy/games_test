1407ef7b0: 4157 push r15
1407ef7b2: 4156 push r14
1407ef7b4: 56 push rsi
1407ef7b5: 57 push rdi
1407ef7b6: 53 push rbx
1407ef7b7: 4883ec70 sub rsp, 0x70
1407ef7bb: 4c89cf mov rdi, r9
1407ef7be: 4889ce mov rsi, rcx
1407ef7c1: 4180f802 cmp r8b, 2
1407ef7c5: 7321 jae 0x1407ef7e8
1407ef7c7: feca dec dl
1407ef7c9: 80fa03 cmp dl, 3
1407ef7cc: 0f8311010000 jae 0x1407ef8e3
1407ef7d2: 0fb6c2 movzx eax, dl
1407ef7d5: 488d0d8cb62902 lea rcx, [rip + 0x229b68c]
1407ef7dc: 488d04c1 lea rax, [rcx + rax*8]
1407ef7e0: 41b831000000 mov r8d, 0x31
1407ef7e6: eb2c jmp 0x1407ef814
1407ef7e8: 410fb6c0 movzx eax, r8b
1407ef7ec: 83f803 cmp eax, 3
1407ef7ef: 0f8506010000 jne 0x1407ef8fb
1407ef7f5: feca dec dl
1407ef7f7: 80fa03 cmp dl, 3
1407ef7fa: 0f8338010000 jae 0x1407ef938
1407ef800: 0fb6c2 movzx eax, dl
1407ef803: 488d0d76b62902 lea rcx, [rip + 0x229b676]
1407ef80a: 488d04c1 lea rax, [rcx + rax*8]
1407ef80e: 41b838000000 mov r8d, 0x38
1407ef814: 488b8c24c0000000 mov rcx, qword ptr [rsp + 0xc0]
1407ef81c: 488b10 mov rdx, qword ptr [rax]
1407ef81f: e82cdc1300 call 0x14092d450
1407ef824: 4885c0 test rax, rax
1407ef827: 0f849e000000 je 0x1407ef8cb
1407ef82d: 4989c7 mov r15, rax
1407ef830: 4889c1 mov rcx, rax
1407ef833: e868842100 call 0x140a07ca0
1407ef838: 4889c3 mov rbx, rax
1407ef83b: 4989d6 mov r14, rdx
1407ef83e: 4c89f9 mov rcx, r15
1407ef841: e8ba852100 call 0x140a07e00
1407ef846: 4885c0 test rax, rax
1407ef849: 0f8488000000 je 0x1407ef8d7
1407ef84f: 4c89f1 mov rcx, r14
1407ef852: 4829d9 sub rcx, rbx
1407ef855: 48c1e902 shr rcx, 2
1407ef859: 49b8abaaaaaaaaaaaaaa movabs r8, 0xaaaaaaaaaaaaaaab
1407ef863: 490fafc8 imul rcx, r8
1407ef867: 4989d1 mov r9, rdx
1407ef86a: 4929c1 sub r9, rax
1407ef86d: 49c1e902 shr r9, 2
1407ef871: 4d0fafc8 imul r9, r8
1407ef875: 4939c9 cmp r9, rcx
1407ef878: 4c0f43c9 cmovae r9, rcx
1407ef87c: 48895c2430 mov qword ptr [rsp + 0x30], rbx
1407ef881: 4c89742438 mov qword ptr [rsp + 0x38], r14
1407ef886: 4889442440 mov qword ptr [rsp + 0x40], rax
1407ef88b: 4889542448 mov qword ptr [rsp + 0x48], rdx
1407ef890: 48c744245000000000 mov qword ptr [rsp + 0x50], 0
1407ef899: 4c894c2458 mov qword ptr [rsp + 0x58], r9
1407ef89e: 48894c2460 mov qword ptr [rsp + 0x60], rcx
1407ef8a3: 48897c2468 mov qword ptr [rsp + 0x68], rdi
1407ef8a8: 4c8d0581aa2902 lea r8, [rip + 0x229aa81]
1407ef8af: 488d542430 lea rdx, [rsp + 0x30]
1407ef8b4: 4889f1 mov rcx, rsi
1407ef8b7: e88412a700 call 0x141260b40
1407ef8bc: 4889f0 mov rax, rsi
1407ef8bf: 4883c470 add rsp, 0x70
1407ef8c3: 5b pop rbx
1407ef8c4: 5f pop rdi
1407ef8c5: 5e pop rsi
1407ef8c6: 415e pop r14
1407ef8c8: 415f pop r15
1407ef8ca: c3 ret 
1407ef8cb: 488d0df6b22902 lea rcx, [rip + 0x229b2f6]
1407ef8d2: e8399a0e02 call 0x1428d9310
1407ef8d7: 488d0d02b32902 lea rcx, [rip + 0x229b302]
1407ef8de: e82d9a0e02 call 0x1428d9310
1407ef8e3: 488d0deead2902 lea rcx, [rip + 0x229adee]
1407ef8ea: 4c8d0587b12902 lea r8, [rip + 0x229b187]
1407ef8f1: ba28000000 mov edx, 0x28
1407ef8f6: e8359b0e02 call 0x1428d9430
1407ef8fb: 83f802 cmp eax, 2
1407ef8fe: 7550 jne 0x1407ef950
1407ef900: 488d05f1b12902 lea rax, [rip + 0x229b1f1]
1407ef907: 4889442430 mov qword ptr [rsp + 0x30], rax
1407ef90c: 48c744243801000000 mov qword ptr [rsp + 0x38], 1
1407ef915: 488d442428 lea rax, [rsp + 0x28]
1407ef91a: 4889442440 mov qword ptr [rsp + 0x40], rax
1407ef91f: 0f57c0 xorps xmm0, xmm0
1407ef922: 0f11442448 movups xmmword ptr [rsp + 0x48], xmm0
1407ef927: 488d15dab12902 lea rdx, [rip + 0x229b1da]
1407ef92e: 488d4c2430 lea rcx, [rsp + 0x30]
1407ef933: e8589a0e02 call 0x1428d9390
1407ef938: 488d0d99ad2902 lea rcx, [rip + 0x229ad99]
1407ef93f: 4c8d05b2b22902 lea r8, [rip + 0x229b2b2]
1407ef946: ba28000000 mov edx, 0x28
1407ef94b: e8e09a0e02 call 0x1428d9430
1407ef950: 488d0511b32902 lea rax, [rip + 0x229b311]
1407ef957: 4889442430 mov qword ptr [rsp + 0x30], rax
1407ef95c: 48c744243801000000 mov qword ptr [rsp + 0x38], 1
1407ef965: 488d442428 lea rax, [rsp + 0x28]
1407ef96a: 4889442440 mov qword ptr [rsp + 0x40], rax
1407ef96f: 0f57c0 xorps xmm0, xmm0
1407ef972: 0f11442448 movups xmmword ptr [rsp + 0x48], xmm0
1407ef977: 488d15fab22902 lea rdx, [rip + 0x229b2fa]
1407ef97e: 488d4c2430 lea rcx, [rsp + 0x30]
1407ef983: e8089a0e02 call 0x1428d9390
1407ef988: cc int3 
