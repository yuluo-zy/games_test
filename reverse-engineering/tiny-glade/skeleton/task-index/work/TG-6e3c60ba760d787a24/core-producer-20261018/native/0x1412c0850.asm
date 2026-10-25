1412c0850: 55 push rbp
1412c0851: 56 push rsi
1412c0852: 4881ece8000000 sub rsp, 0xe8
1412c0859: 488dac2480000000 lea rbp, [rsp + 0x80]
1412c0861: 48c74560feffffff mov qword ptr [rbp + 0x60], 0xfffffffffffffffe
1412c0869: 4889ce mov rsi, rcx
1412c086c: 488d4da8 lea rcx, [rbp - 0x58]
1412c0870: 4531c0 xor r8d, r8d
1412c0873: e838d53500 call 0x14161ddb0
1412c0878: 0fb6058a433702 movzx eax, byte ptr [rip + 0x237438a]
1412c087f: b938000000 mov ecx, 0x38
1412c0884: ba08000000 mov edx, 8
1412c0889: e8823335ff call 0x140613c10
1412c088e: 4885c0 test rax, rax
1412c0891: 0f84b8000000 je 0x1412c094f
1412c0897: 488b4dd8 mov rcx, qword ptr [rbp - 0x28]
1412c089b: 48894830 mov qword ptr [rax + 0x30], rcx
1412c089f: 0f1045a8 movups xmm0, xmmword ptr [rbp - 0x58]
1412c08a3: 0f104db8 movups xmm1, xmmword ptr [rbp - 0x48]
1412c08a7: 0f1055c8 movups xmm2, xmmword ptr [rbp - 0x38]
1412c08ab: 0f115020 movups xmmword ptr [rax + 0x20], xmm2
1412c08af: 0f114810 movups xmmword ptr [rax + 0x10], xmm1
1412c08b3: 0f1100 movups xmmword ptr [rax], xmm0
1412c08b6: 488d0d0ad77d01 lea rcx, [rip + 0x17dd70a]
1412c08bd: 48894d10 mov qword ptr [rbp + 0x10], rcx
1412c08c1: 48c7451845000000 mov qword ptr [rbp + 0x18], 0x45
1412c08c9: 48894520 mov qword ptr [rbp + 0x20], rax
1412c08cd: 488d05ccc29601 lea rax, [rip + 0x196c2cc]
1412c08d4: 48894528 mov qword ptr [rbp + 0x28], rax
1412c08d8: 48c7454000000000 mov qword ptr [rbp + 0x40], 0
1412c08e0: c6455605 mov byte ptr [rbp + 0x56], 5
1412c08e4: 48894d30 mov qword ptr [rbp + 0x30], rcx
1412c08e8: 48c7453845000000 mov qword ptr [rbp + 0x38], 0x45
1412c08f0: 0fb60512433702 movzx eax, byte ptr [rip + 0x2374312]
1412c08f7: b940000000 mov ecx, 0x40
1412c08fc: ba08000000 mov edx, 8
1412c0901: e80a3335ff call 0x140613c10
1412c0906: 4885c0 test rax, rax
1412c0909: 7455 je 0x1412c0960
1412c090b: 0f104520 movups xmm0, xmmword ptr [rbp + 0x20]
1412c090f: 0f104d30 movups xmm1, xmmword ptr [rbp + 0x30]
1412c0913: 0f105540 movups xmm2, xmmword ptr [rbp + 0x40]
1412c0917: 0f105d50 movups xmm3, xmmword ptr [rbp + 0x50]
1412c091b: 0f115830 movups xmmword ptr [rax + 0x30], xmm3
1412c091f: 0f115020 movups xmmword ptr [rax + 0x20], xmm2
1412c0923: 0f114810 movups xmmword ptr [rax + 0x10], xmm1
1412c0927: 0f1100 movups xmmword ptr [rax], xmm0
1412c092a: 48ba063b71721d78130e movabs rdx, 0xe13781d72713b06
1412c0934: 4889f1 mov rcx, rsi
1412c0937: 4989c0 mov r8, rax
1412c093a: e8b1926aff call 0x140969bf0
1412c093f: 4883f801 cmp rax, 1
1412c0943: 742c je 0x1412c0971
1412c0945: 4881c4e8000000 add rsp, 0xe8
1412c094c: 5e pop rsi
1412c094d: 5d pop rbp
1412c094e: c3 ret 
1412c094f: b908000000 mov ecx, 8
1412c0954: ba38000000 mov edx, 0x38
1412c0959: e8a2866101 call 0x1428d9000
1412c095e: eb0f jmp 0x1412c096f
1412c0960: b908000000 mov ecx, 8
1412c0965: ba40000000 mov edx, 0x40
1412c096a: e891866101 call 0x1428d9000
1412c096f: 0f0b ud2 
1412c0971: 4889d0 mov rax, rdx
1412c0974: 4883c210 add rdx, 0x10
1412c0978: 4889d6 mov rsi, rdx
1412c097b: 4883781845 cmp qword ptr [rax + 0x18], 0x45
1412c0980: 757d jne 0x1412c09ff
1412c0982: 488b16 mov rdx, qword ptr [rsi]
1412c0985: 488d0d3bd67d01 lea rcx, [rip + 0x17dd63b]
1412c098c: 41b845000000 mov r8d, 0x45
1412c0992: e859326601 call 0x142923bf0
1412c0997: 85c0 test eax, eax
1412c0999: 7564 jne 0x1412c09ff
1412c099b: 488d0566d49601 lea rax, [rip + 0x196d466]
1412c09a2: 488945e0 mov qword ptr [rbp - 0x20], rax
1412c09a6: 488d05f32e27ff lea rax, [rip - 0xd8d10d]
1412c09ad: 488945e8 mov qword ptr [rbp - 0x18], rax
1412c09b1: 488d4510 lea rax, [rbp + 0x10]
1412c09b5: 488945f0 mov qword ptr [rbp - 0x10], rax
1412c09b9: 488d05d094e4fe lea rax, [rip - 0x11b6b30]
1412c09c0: 488945f8 mov qword ptr [rbp - 8], rax
1412c09c4: 488d055dcc9601 lea rax, [rip + 0x196cc5d]
1412c09cb: 48894520 mov qword ptr [rbp + 0x20], rax
1412c09cf: 48c7452803000000 mov qword ptr [rbp + 0x28], 3
1412c09d7: 48c7454000000000 mov qword ptr [rbp + 0x40], 0
1412c09df: 488d45e0 lea rax, [rbp - 0x20]
1412c09e3: 48894530 mov qword ptr [rbp + 0x30], rax
1412c09e7: 48c7453802000000 mov qword ptr [rbp + 0x38], 2
1412c09ef: 488d1562cc9601 lea rdx, [rip + 0x196cc62]
1412c09f6: 488d4d20 lea rcx, [rbp + 0x20]
1412c09fa: e891896101 call 0x1428d9390
1412c09ff: 488d0502d49601 lea rax, [rip + 0x196d402]
1412c0a06: 48894520 mov qword ptr [rbp + 0x20], rax
1412c0a0a: 488d058f2e27ff lea rax, [rip - 0xd8d171]
1412c0a11: 48894528 mov qword ptr [rbp + 0x28], rax
1412c0a15: 48897530 mov qword ptr [rbp + 0x30], rsi
1412c0a19: 488d057094e4fe lea rax, [rip - 0x11b6b90]
1412c0a20: 48894538 mov qword ptr [rbp + 0x38], rax
1412c0a24: 488d4d10 lea rcx, [rbp + 0x10]
1412c0a28: 48894d40 mov qword ptr [rbp + 0x40], rcx
1412c0a2c: 48894548 mov qword ptr [rbp + 0x48], rax
1412c0a30: 488d0531cb9601 lea rax, [rip + 0x196cb31]
1412c0a37: 488945e0 mov qword ptr [rbp - 0x20], rax
1412c0a3b: 48c745e803000000 mov qword ptr [rbp - 0x18], 3
1412c0a43: 48c7450000000000 mov qword ptr [rbp], 0
1412c0a4b: 488d4520 lea rax, [rbp + 0x20]
1412c0a4f: 488945f0 mov qword ptr [rbp - 0x10], rax
1412c0a53: 48c745f803000000 mov qword ptr [rbp - 8], 3
1412c0a5b: 488d156ecb9601 lea rdx, [rip + 0x196cb6e]
1412c0a62: 488d4de0 lea rcx, [rbp - 0x20]
1412c0a66: e825896101 call 0x1428d9390
1412c0a6b: cc int3 
1412c0a6c: 0f1f4000 nop dword ptr [rax]
1412c0a70: 4889542410 mov qword ptr [rsp + 0x10], rdx
1412c0a75: 55 push rbp
1412c0a76: 56 push rsi
1412c0a77: 4883ec28 sub rsp, 0x28
1412c0a7b: 488daa80000000 lea rbp, [rdx + 0x80]
1412c0a82: 488d4db8 lea rcx, [rbp - 0x48]
1412c0a86: e8251a92ff call 0x140be24b0
1412c0a8b: 90 nop 
1412c0a8c: 4883c428 add rsp, 0x28
1412c0a90: 5e pop rsi
1412c0a91: 5d pop rbp
1412c0a92: c3 ret 
1412c0a93: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1412c0aa0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1412c0aa5: 55 push rbp
1412c0aa6: 56 push rsi
1412c0aa7: 4883ec28 sub rsp, 0x28
1412c0aab: 488daa80000000 lea rbp, [rdx + 0x80]
1412c0ab2: 488d4d20 lea rcx, [rbp + 0x20]
1412c0ab6: e8f5f9dffe call 0x1400c04b0
1412c0abb: 90 nop 
1412c0abc: 4883c428 add rsp, 0x28
1412c0ac0: 5e pop rsi
1412c0ac1: 5d pop rbp
1412c0ac2: c3 ret 
