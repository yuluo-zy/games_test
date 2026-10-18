1412c0850: push rbp
1412c0851: push rsi
1412c0852: sub rsp, 0xe8
1412c0859: lea rbp, [rsp + 0x80]
1412c0861: mov qword ptr [rbp + 0x60], 0xfffffffffffffffe
1412c0869: mov rsi, rcx
1412c086c: lea rcx, [rbp - 0x58]
1412c0870: xor r8d, r8d
1412c0873: call 0x14161ddb0
1412c0878: movzx eax, byte ptr [rip + 0x237438a]
1412c087f: mov ecx, 0x38
1412c0884: mov edx, 8
1412c0889: call 0x140613c10
1412c088e: test rax, rax
1412c0891: je 0x1412c094f
1412c0897: mov rcx, qword ptr [rbp - 0x28]
1412c089b: mov qword ptr [rax + 0x30], rcx
1412c089f: movups xmm0, xmmword ptr [rbp - 0x58]
1412c08a3: movups xmm1, xmmword ptr [rbp - 0x48]
1412c08a7: movups xmm2, xmmword ptr [rbp - 0x38]
1412c08ab: movups xmmword ptr [rax + 0x20], xmm2
1412c08af: movups xmmword ptr [rax + 0x10], xmm1
1412c08b3: movups xmmword ptr [rax], xmm0
1412c08b6: lea rcx, [rip + 0x17dd70a]
1412c08bd: mov qword ptr [rbp + 0x10], rcx
1412c08c1: mov qword ptr [rbp + 0x18], 0x45
1412c08c9: mov qword ptr [rbp + 0x20], rax
1412c08cd: lea rax, [rip + 0x196c2cc]
1412c08d4: mov qword ptr [rbp + 0x28], rax
1412c08d8: mov qword ptr [rbp + 0x40], 0
1412c08e0: mov byte ptr [rbp + 0x56], 5
1412c08e4: mov qword ptr [rbp + 0x30], rcx
1412c08e8: mov qword ptr [rbp + 0x38], 0x45
1412c08f0: movzx eax, byte ptr [rip + 0x2374312]
1412c08f7: mov ecx, 0x40
1412c08fc: mov edx, 8
1412c0901: call 0x140613c10
1412c0906: test rax, rax
1412c0909: je 0x1412c0960
1412c090b: movups xmm0, xmmword ptr [rbp + 0x20]
1412c090f: movups xmm1, xmmword ptr [rbp + 0x30]
1412c0913: movups xmm2, xmmword ptr [rbp + 0x40]
1412c0917: movups xmm3, xmmword ptr [rbp + 0x50]
1412c091b: movups xmmword ptr [rax + 0x30], xmm3
1412c091f: movups xmmword ptr [rax + 0x20], xmm2
1412c0923: movups xmmword ptr [rax + 0x10], xmm1
1412c0927: movups xmmword ptr [rax], xmm0
1412c092a: movabs rdx, 0xe13781d72713b06
1412c0934: mov rcx, rsi
1412c0937: mov r8, rax
1412c093a: call 0x140969bf0
1412c093f: cmp rax, 1
1412c0943: je 0x1412c0971
1412c0945: add rsp, 0xe8
1412c094c: pop rsi
1412c094d: pop rbp
1412c094e: ret 
1412c094f: mov ecx, 8
1412c0954: mov edx, 0x38
1412c0959: call 0x1428d9000
1412c095e: jmp 0x1412c096f
1412c0960: mov ecx, 8
1412c0965: mov edx, 0x40
1412c096a: call 0x1428d9000
1412c096f: ud2 
1412c0971: mov rax, rdx
1412c0974: add rdx, 0x10
1412c0978: mov rsi, rdx
1412c097b: cmp qword ptr [rax + 0x18], 0x45
1412c0980: jne 0x1412c09ff
1412c0982: mov rdx, qword ptr [rsi]
1412c0985: lea rcx, [rip + 0x17dd63b]
1412c098c: mov r8d, 0x45
1412c0992: call 0x142923bf0
1412c0997: test eax, eax
1412c0999: jne 0x1412c09ff
1412c099b: lea rax, [rip + 0x196d466]
1412c09a2: mov qword ptr [rbp - 0x20], rax
1412c09a6: lea rax, [rip - 0xd8d10d]
1412c09ad: mov qword ptr [rbp - 0x18], rax
1412c09b1: lea rax, [rbp + 0x10]
1412c09b5: mov qword ptr [rbp - 0x10], rax
1412c09b9: lea rax, [rip - 0x11b6b30]
1412c09c0: mov qword ptr [rbp - 8], rax
1412c09c4: lea rax, [rip + 0x196cc5d]
1412c09cb: mov qword ptr [rbp + 0x20], rax
1412c09cf: mov qword ptr [rbp + 0x28], 3
1412c09d7: mov qword ptr [rbp + 0x40], 0
1412c09df: lea rax, [rbp - 0x20]
1412c09e3: mov qword ptr [rbp + 0x30], rax
1412c09e7: mov qword ptr [rbp + 0x38], 2
1412c09ef: lea rdx, [rip + 0x196cc62]
1412c09f6: lea rcx, [rbp + 0x20]
1412c09fa: call 0x1428d9390
1412c09ff: lea rax, [rip + 0x196d402]
1412c0a06: mov qword ptr [rbp + 0x20], rax
1412c0a0a: lea rax, [rip - 0xd8d171]
1412c0a11: mov qword ptr [rbp + 0x28], rax
1412c0a15: mov qword ptr [rbp + 0x30], rsi
1412c0a19: lea rax, [rip - 0x11b6b90]
1412c0a20: mov qword ptr [rbp + 0x38], rax
1412c0a24: lea rcx, [rbp + 0x10]
1412c0a28: mov qword ptr [rbp + 0x40], rcx
1412c0a2c: mov qword ptr [rbp + 0x48], rax
1412c0a30: lea rax, [rip + 0x196cb31]
1412c0a37: mov qword ptr [rbp - 0x20], rax
1412c0a3b: mov qword ptr [rbp - 0x18], 3
1412c0a43: mov qword ptr [rbp], 0
1412c0a4b: lea rax, [rbp + 0x20]
1412c0a4f: mov qword ptr [rbp - 0x10], rax
1412c0a53: mov qword ptr [rbp - 8], 3
1412c0a5b: lea rdx, [rip + 0x196cb6e]
1412c0a62: lea rcx, [rbp - 0x20]
1412c0a66: call 0x1428d9390
1412c0a6b: int3 
1412c0a6c: nop dword ptr [rax]
1412c0a70: mov qword ptr [rsp + 0x10], rdx
1412c0a75: push rbp
1412c0a76: push rsi
1412c0a77: sub rsp, 0x28
1412c0a7b: lea rbp, [rdx + 0x80]
1412c0a82: lea rcx, [rbp - 0x48]
1412c0a86: call 0x140be24b0
1412c0a8b: nop 
1412c0a8c: add rsp, 0x28
1412c0a90: pop rsi
1412c0a91: pop rbp
1412c0a92: ret 
1412c0a93: nop word ptr cs:[rax + rax]
1412c0aa0: mov qword ptr [rsp + 0x10], rdx
1412c0aa5: push rbp
1412c0aa6: push rsi
1412c0aa7: sub rsp, 0x28
1412c0aab: lea rbp, [rdx + 0x80]
1412c0ab2: lea rcx, [rbp + 0x20]
1412c0ab6: call 0x1400c04b0
1412c0abb: nop 
1412c0abc: add rsp, 0x28
1412c0ac0: pop rsi
1412c0ac1: pop rbp
1412c0ac2: ret 
