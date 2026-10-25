14161fea0: push rbp
14161fea1: push r15
14161fea3: push r14
14161fea5: push r12
14161fea7: push rsi
14161fea8: push rdi
14161fea9: push rbx
14161feaa: sub rsp, 0x130
14161feb1: lea rbp, [rsp + 0x80]
14161feb9: mov qword ptr [rbp + 0xa8], 0xfffffffffffffffe
14161fec4: mov rdi, rdx
14161fec7: mov rsi, rcx
14161feca: test r8, r8
14161fecd: je 0x14161ff09
14161fecf: mov rbx, r9
14161fed2: mov rax, r9
14161fed5: and rax, 3
14161fed9: mov qword ptr [rbp - 0x40], rax
14161fedd: jne 0x1416200ee
14161fee3: mov r15, r8
14161fee6: mov rax, r8
14161fee9: and rax, 3
14161feed: mov qword ptr [rbp - 0x40], rax
14161fef1: jne 0x1416200ff
14161fef7: cmp rbx, 0x184
14161fefe: jae 0x14162019a
14161ff04: xor r12d, r12d
14161ff07: jmp 0x14161ff43
14161ff09: movzx eax, byte ptr [rip + 0x2014cf9]
14161ff10: mov ebx, 0x180
14161ff15: mov ecx, 0x180
14161ff1a: mov edx, 4
14161ff1f: call 0x140613c10
14161ff24: test rax, rax
14161ff27: je 0x141620204
14161ff2d: mov r15, rax
14161ff30: mov r8d, 0x180
14161ff36: mov rcx, rax
14161ff39: xor edx, edx
14161ff3b: call 0x142923c20
14161ff40: mov r12b, 1
14161ff43: mov qword ptr [rbp + 0x98], r15
14161ff4a: mov byte ptr [rbp + 0xa7], r12b
14161ff51: lea rcx, [rbp + 0x10]
14161ff55: mov edx, 0x180
14161ff5a: mov r8d, 0x22
14161ff60: mov r9b, 4
14161ff63: call 0x140bd4de0
14161ff68: mov qword ptr [rbp + 0x98], r15
14161ff6f: mov byte ptr [rbp + 0xa7], r12b
14161ff76: lea r14, [rbp + 0x60]
14161ff7a: lea rdx, [rbp + 0x10]
14161ff7e: mov r8d, 4
14161ff84: mov rcx, r14
14161ff87: call 0x140bd4f20
14161ff8c: mov qword ptr [rbp + 0x98], r15
14161ff93: mov byte ptr [rbp + 0xa7], r12b
14161ff9a: mov qword ptr [rsp + 0x30], rbx
14161ff9f: mov qword ptr [rsp + 0x28], r15
14161ffa4: mov qword ptr [rsp + 0x20], r14
14161ffa9: lea r8, [rip + 0x165f8a8]
14161ffb0: lea rcx, [rbp - 0x40]
14161ffb4: mov r9d, 0x12
14161ffba: mov rdx, rdi
14161ffbd: call 0x1416a2dc0
14161ffc2: cmp dword ptr [rbp - 0x40], 0xb
14161ffc6: jne 0x141620129
14161ffcc: mov rax, qword ptr [rbp - 0x18]
14161ffd0: mov qword ptr [rbp + 0x80], rax
14161ffd7: movups xmm0, xmmword ptr [rbp - 0x38]
14161ffdb: movups xmm1, xmmword ptr [rbp - 0x28]
14161ffdf: movaps xmmword ptr [rbp + 0x70], xmm1
14161ffe3: movaps xmmword ptr [rbp + 0x60], xmm0
14161ffe7: test r12b, r12b
14161ffea: je 0x14161ffff
14161ffec: mov edx, 0x180
14161fff1: mov r8d, 4
14161fff7: mov rcx, r15
14161fffa: call 0x140613c20
14161ffff: lea rcx, [rbp + 0x60]
141620003: mov rdx, rdi
141620006: call 0x140bd4cd0
14162000b: test rax, rax
14162000e: je 0x141620189
141620014: cmp rdx, qword ptr [rbp + 0x78]
141620018: jb 0x141620073
14162001a: mov rcx, rdx
14162001d: and rcx, 3
141620021: mov qword ptr [rbp - 0x40], rcx
141620025: jne 0x141620090
141620027: mov rcx, rax
14162002a: and rcx, 3
14162002e: mov qword ptr [rbp - 0x40], rcx
141620032: jne 0x1416200bf
141620038: mov rcx, qword ptr [rbp + 0x80]
14162003f: mov qword ptr [rsi + 0x30], rcx
141620043: movaps xmm0, xmmword ptr [rbp + 0x60]
141620047: movaps xmm1, xmmword ptr [rbp + 0x70]
14162004b: movups xmmword ptr [rsi + 0x20], xmm1
14162004f: movups xmmword ptr [rsi + 0x10], xmm0
141620053: shr rdx, 2
141620057: mov qword ptr [rsi], rax
14162005a: mov qword ptr [rsi + 8], rdx
14162005e: mov rax, rsi
141620061: add rsp, 0x130
141620068: pop rbx
141620069: pop rdi
14162006a: pop rsi
14162006b: pop r12
14162006d: pop r14
14162006f: pop r15
141620071: pop rbp
141620072: ret 
141620073: lea rcx, [rip + 0x1660176]
14162007a: lea r8, [rip + 0x16601a7]
141620081: mov edx, 0x37
141620086: call 0x1428d9430
14162008b: jmp 0x141620221
141620090: mov qword ptr [rbp + 0x10], 0
141620098: lea rax, [rip + 0x16601a9]
14162009f: mov qword ptr [rsp + 0x20], rax
1416200a4: lea r8, [rip + 0x1660195]
1416200ab: lea rdx, [rbp - 0x40]
1416200af: lea r9, [rbp + 0x10]
1416200b3: xor ecx, ecx
1416200b5: call 0x1428d9596
1416200ba: jmp 0x141620221
1416200bf: mov qword ptr [rbp + 0x10], 0
1416200c7: lea rax, [rip + 0x1660192]
1416200ce: mov qword ptr [rsp + 0x20], rax
1416200d3: lea r8, [rip + 0x1660166]
1416200da: lea rdx, [rbp - 0x40]
1416200de: lea r9, [rbp + 0x10]
1416200e2: xor ecx, ecx
1416200e4: call 0x1428d9596
1416200e9: jmp 0x141620221
1416200ee: mov qword ptr [rbp + 0x10], 0
1416200f6: lea rax, [rip + 0x16a9e4b]
1416200fd: jmp 0x14162010e
1416200ff: mov qword ptr [rbp + 0x10], 0
141620107: lea rax, [rip + 0x16a9e52]
14162010e: mov qword ptr [rsp + 0x20], rax
141620113: lea r8, [rip + 0x16a9e26]
14162011a: lea rdx, [rbp - 0x40]
14162011e: lea r9, [rbp + 0x10]
141620122: xor ecx, ecx
141620124: call 0x1428d9596
141620129: mov dword ptr [rbp + 0x94], r12d
141620130: mov qword ptr [rbp - 0x48], r15
141620134: movups xmm0, xmmword ptr [rbp]
141620138: movaps xmmword ptr [rbp + 0x50], xmm0
14162013c: movups xmm0, xmmword ptr [rbp - 0x40]
141620140: movups xmm1, xmmword ptr [rbp - 0x30]
141620144: movups xmm2, xmmword ptr [rbp - 0x20]
141620148: movups xmm3, xmmword ptr [rbp - 0x10]
14162014c: movaps xmmword ptr [rbp + 0x40], xmm3
141620150: movaps xmmword ptr [rbp + 0x30], xmm2
141620154: movaps xmmword ptr [rbp + 0x20], xmm1
141620158: movaps xmmword ptr [rbp + 0x10], xmm0
14162015c: lea rax, [rip + 0x1660045]
141620163: mov qword ptr [rsp + 0x20], rax
141620168: lea rcx, [rip + 0x165e979]
14162016f: lea r9, [rip + 0x165e952]
141620176: lea r8, [rbp + 0x10]
14162017a: mov edx, 0x2b
14162017f: call 0x1428d9760
141620184: jmp 0x141620221
141620189: lea rcx, [rip + 0x1660048]
141620190: call 0x1428d9310
141620195: jmp 0x141620221
14162019a: lea rax, [rip + 0x147e03a]
1416201a1: mov qword ptr [rbp - 0x40], rax
1416201a5: mov qword ptr [rbp - 0x38], 0x4e
1416201ad: lea rax, [rbp - 0x40]
1416201b1: mov qword ptr [rbp + 0x60], rax
1416201b5: lea rax, [rip - 0x151632c]
1416201bc: mov qword ptr [rbp + 0x68], rax
1416201c0: lea rax, [rip + 0x165f601]
1416201c7: mov qword ptr [rbp + 0x10], rax
1416201cb: mov qword ptr [rbp + 0x18], 1
1416201d3: mov qword ptr [rbp + 0x30], 0
1416201db: lea rax, [rbp + 0x60]
1416201df: mov qword ptr [rbp + 0x20], rax
1416201e3: mov qword ptr [rbp + 0x28], 1
1416201eb: mov byte ptr [rbp + 0xa7], 0
1416201f2: lea rdx, [rip + 0x165f5df]
1416201f9: lea rcx, [rbp + 0x10]
1416201fd: call 0x1428d9390
141620202: jmp 0x141620221
141620204: mov byte ptr [rbp + 0xa7], 0
14162020b: lea r8, [rip + 0x165e89e]
141620212: mov ecx, 4
141620217: mov edx, 0x180
14162021c: call 0x1428d8fe3
141620221: ud2 
141620223: nop word ptr cs:[rax + rax]
141620230: mov qword ptr [rsp + 0x10], rdx
141620235: push rbp
141620236: push r15
141620238: push r14
14162023a: push r12
14162023c: push rsi
14162023d: push rdi
14162023e: push rbx
14162023f: sub rsp, 0x40
141620243: lea rbp, [rdx + 0x80]
14162024a: test byte ptr [rbp + 0xa7], 1
141620251: je 0x14162026a
141620253: mov rcx, qword ptr [rbp + 0x98]
14162025a: mov edx, 0x180
14162025f: mov r8d, 4
141620265: call 0x140613c20
14162026a: nop 
14162026b: add rsp, 0x40
14162026f: pop rbx
141620270: pop rdi
141620271: pop rsi
141620272: pop r12
141620274: pop r14
141620276: pop r15
141620278: pop rbp
141620279: ret 
14162027a: nop word ptr [rax + rax]
141620280: mov qword ptr [rsp + 0x10], rdx
141620285: push rbp
141620286: push r15
141620288: push r14
14162028a: push r12
14162028c: push rsi
14162028d: push rdi
14162028e: push rbx
14162028f: sub rsp, 0x40
141620293: lea rbp, [rdx + 0x80]
14162029a: lea rcx, [rbp + 0x60]
14162029e: call 0x140be24b0
1416202a3: nop 
1416202a4: add rsp, 0x40
1416202a8: pop rbx
1416202a9: pop rdi
1416202aa: pop rsi
1416202ab: pop r12
1416202ad: pop r14
1416202af: pop r15
1416202b1: pop rbp
1416202b2: ret 
1416202b3: nop word ptr cs:[rax + rax]
1416202c0: mov qword ptr [rsp + 0x10], rdx
1416202c5: push rbp
1416202c6: push r15
1416202c8: push r14
1416202ca: push r12
1416202cc: push rsi
1416202cd: push rdi
1416202ce: push rbx
1416202cf: sub rsp, 0x40
1416202d3: lea rbp, [rdx + 0x80]
1416202da: mov rsi, qword ptr [rbp - 0x48]
1416202de: mov qword ptr [rbp + 0x98], rsi
1416202e5: mov edi, dword ptr [rbp + 0x94]
1416202eb: mov byte ptr [rbp + 0xa7], dil
1416202f2: lea rcx, [rbp + 0x10]
1416202f6: call 0x1407cea70
1416202fb: mov qword ptr [rbp + 0x98], rsi
141620302: mov byte ptr [rbp + 0xa7], dil
141620309: add rsp, 0x40
14162030d: pop rbx
14162030e: pop rdi
14162030f: pop rsi
141620310: pop r12
141620312: pop r14
141620314: pop r15
141620316: pop rbp
141620317: ret 
