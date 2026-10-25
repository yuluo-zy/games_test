1412bd0f0: push rbp
1412bd0f1: push rsi
1412bd0f2: sub rsp, 0xd8
1412bd0f9: lea rbp, [rsp + 0x80]
1412bd101: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412bd109: mov rsi, rcx
1412bd10c: lea rcx, [rbp - 0x58]
1412bd110: xor r8d, r8d
1412bd113: call 0x141612870
1412bd118: movzx eax, byte ptr [rip + 0x2377aea]
1412bd11f: mov ecx, 0x28
1412bd124: mov edx, 8
1412bd129: call 0x140613c10
1412bd12e: test rax, rax
1412bd131: je 0x1412bd1e7
1412bd137: mov rcx, qword ptr [rbp - 0x38]
1412bd13b: mov qword ptr [rax + 0x20], rcx
1412bd13f: movups xmm0, xmmword ptr [rbp - 0x58]
1412bd143: movups xmm1, xmmword ptr [rbp - 0x48]
1412bd147: movups xmmword ptr [rax + 0x10], xmm1
1412bd14b: movups xmmword ptr [rax], xmm0
1412bd14e: lea rcx, [rip + 0x17e016e]
1412bd155: mov qword ptr [rbp], rcx
1412bd159: mov qword ptr [rbp + 8], 0x3b
1412bd161: mov qword ptr [rbp + 0x10], rax
1412bd165: lea rax, [rip + 0x196f564]
1412bd16c: mov qword ptr [rbp + 0x18], rax
1412bd170: mov qword ptr [rbp + 0x30], 0
1412bd178: mov byte ptr [rbp + 0x46], 5
1412bd17c: mov qword ptr [rbp + 0x20], rcx
1412bd180: mov qword ptr [rbp + 0x28], 0x3b
1412bd188: movzx eax, byte ptr [rip + 0x2377a7a]
1412bd18f: mov ecx, 0x40
1412bd194: mov edx, 8
1412bd199: call 0x140613c10
1412bd19e: test rax, rax
1412bd1a1: je 0x1412bd1f8
1412bd1a3: movups xmm0, xmmword ptr [rbp + 0x10]
1412bd1a7: movups xmm1, xmmword ptr [rbp + 0x20]
1412bd1ab: movups xmm2, xmmword ptr [rbp + 0x30]
1412bd1af: movups xmm3, xmmword ptr [rbp + 0x40]
1412bd1b3: movups xmmword ptr [rax + 0x30], xmm3
1412bd1b7: movups xmmword ptr [rax + 0x20], xmm2
1412bd1bb: movups xmmword ptr [rax + 0x10], xmm1
1412bd1bf: movups xmmword ptr [rax], xmm0
1412bd1c2: movabs rdx, 0xc76c5cfaf80ca46a
1412bd1cc: mov rcx, rsi
1412bd1cf: mov r8, rax
1412bd1d2: call 0x140969bf0
1412bd1d7: cmp rax, 1
1412bd1db: je 0x1412bd209
1412bd1dd: add rsp, 0xd8
1412bd1e4: pop rsi
1412bd1e5: pop rbp
1412bd1e6: ret 
1412bd1e7: mov ecx, 8
1412bd1ec: mov edx, 0x28
1412bd1f1: call 0x1428d9000
1412bd1f6: jmp 0x1412bd207
1412bd1f8: mov ecx, 8
1412bd1fd: mov edx, 0x40
1412bd202: call 0x1428d9000
1412bd207: ud2 
1412bd209: mov rax, rdx
1412bd20c: add rdx, 0x10
1412bd210: mov rsi, rdx
1412bd213: cmp qword ptr [rax + 0x18], 0x3b
1412bd218: jne 0x1412bd296
1412bd21a: mov rdx, qword ptr [rsi]
1412bd21d: lea rcx, [rip + 0x17e009f]
1412bd224: mov r8d, 0x3b
1412bd22a: call 0x142923bf0
1412bd22f: test eax, eax
1412bd231: jne 0x1412bd296
1412bd233: lea rax, [rip + 0x19706b6]
1412bd23a: mov qword ptr [rbp - 0x30], rax
1412bd23e: lea rax, [rip - 0xd899a5]
1412bd245: mov qword ptr [rbp - 0x28], rax
1412bd249: mov rax, rbp
1412bd24c: mov qword ptr [rbp - 0x20], rax
1412bd250: lea rax, [rip - 0x11b33c7]
1412bd257: mov qword ptr [rbp - 0x18], rax
1412bd25b: lea rax, [rip + 0x19703c6]
1412bd262: mov qword ptr [rbp + 0x10], rax
1412bd266: mov qword ptr [rbp + 0x18], 3
1412bd26e: mov qword ptr [rbp + 0x30], 0
1412bd276: lea rax, [rbp - 0x30]
1412bd27a: mov qword ptr [rbp + 0x20], rax
1412bd27e: mov qword ptr [rbp + 0x28], 2
1412bd286: lea rdx, [rip + 0x19703cb]
1412bd28d: lea rcx, [rbp + 0x10]
1412bd291: call 0x1428d9390
1412bd296: lea rax, [rip + 0x1970653]
1412bd29d: mov qword ptr [rbp + 0x10], rax
1412bd2a1: lea rax, [rip - 0xd89a08]
1412bd2a8: mov qword ptr [rbp + 0x18], rax
1412bd2ac: mov qword ptr [rbp + 0x20], rsi
1412bd2b0: lea rax, [rip - 0x11b3427]
1412bd2b7: mov qword ptr [rbp + 0x28], rax
1412bd2bb: mov rcx, rbp
1412bd2be: mov qword ptr [rbp + 0x30], rcx
1412bd2c2: mov qword ptr [rbp + 0x38], rax
1412bd2c6: lea rax, [rip + 0x197029b]
1412bd2cd: mov qword ptr [rbp - 0x30], rax
1412bd2d1: mov qword ptr [rbp - 0x28], 3
1412bd2d9: mov qword ptr [rbp - 0x10], 0
1412bd2e1: lea rax, [rbp + 0x10]
1412bd2e5: mov qword ptr [rbp - 0x20], rax
1412bd2e9: mov qword ptr [rbp - 0x18], 3
1412bd2f1: lea rdx, [rip + 0x19702d8]
1412bd2f8: lea rcx, [rbp - 0x30]
1412bd2fc: call 0x1428d9390
1412bd301: int3 
1412bd302: nop word ptr cs:[rax + rax]
1412bd310: mov qword ptr [rsp + 0x10], rdx
1412bd315: push rbp
1412bd316: push rsi
1412bd317: sub rsp, 0x28
1412bd31b: lea rbp, [rdx + 0x80]
1412bd322: lea rcx, [rbp - 0x58]
1412bd326: call 0x140be24b0
1412bd32b: nop 
1412bd32c: add rsp, 0x28
1412bd330: pop rsi
1412bd331: pop rbp
1412bd332: ret 
1412bd333: nop word ptr cs:[rax + rax]
1412bd340: mov qword ptr [rsp + 0x10], rdx
1412bd345: push rbp
1412bd346: push rsi
1412bd347: sub rsp, 0x28
1412bd34b: lea rbp, [rdx + 0x80]
1412bd352: lea rcx, [rbp + 0x10]
1412bd356: call 0x1400c04b0
1412bd35b: nop 
1412bd35c: add rsp, 0x28
1412bd360: pop rsi
1412bd361: pop rbp
1412bd362: ret 
