141611000: push rbp
141611001: push r15
141611003: push r14
141611005: push r12
141611007: push rsi
141611008: push rdi
141611009: push rbx
14161100a: sub rsp, 0x120
141611011: lea rbp, [rsp + 0x80]
141611019: mov qword ptr [rbp + 0x98], 0xfffffffffffffffe
141611024: mov rdi, rdx
141611027: mov rsi, rcx
14161102a: test r8, r8
14161102d: je 0x141611069
14161102f: mov rbx, r9
141611032: mov rax, r9
141611035: and rax, 3
141611039: mov qword ptr [rbp - 0x40], rax
14161103d: jne 0x1416111ac
141611043: mov r15, r8
141611046: mov rax, r8
141611049: and rax, 3
14161104d: mov qword ptr [rbp - 0x40], rax
141611051: jne 0x1416111bd
141611057: cmp rbx, 0x40004
14161105e: jae 0x141611247
141611064: xor r12d, r12d
141611067: jmp 0x1416110e3
141611069: movzx eax, byte ptr [rip + 0x2023b99]
141611070: mov ecx, 0x40000
141611075: mov edx, 4
14161107a: call 0x140613c10
14161107f: test rax, rax
141611082: je 0x1416112b1
141611088: mov r15, rax
14161108b: mov eax, 0x1c
141611090: xorps xmm0, xmm0
141611093: nop word ptr cs:[rax + rax]
1416110a0: movups xmmword ptr [r15 + rax*4 - 0x70], xmm0
1416110a6: movups xmmword ptr [r15 + rax*4 - 0x60], xmm0
1416110ac: movups xmmword ptr [r15 + rax*4 - 0x50], xmm0
1416110b2: movups xmmword ptr [r15 + rax*4 - 0x40], xmm0
1416110b8: movups xmmword ptr [r15 + rax*4 - 0x30], xmm0
1416110be: movups xmmword ptr [r15 + rax*4 - 0x20], xmm0
1416110c4: movups xmmword ptr [r15 + rax*4 - 0x10], xmm0
1416110ca: movups xmmword ptr [r15 + rax*4], xmm0
1416110cf: add rax, 0x20
1416110d3: cmp rax, 0x1001c
1416110d9: jne 0x1416110a0
1416110db: mov r12b, 1
1416110de: mov ebx, 0x40000
1416110e3: mov qword ptr [rbp + 0x88], r15
1416110ea: mov byte ptr [rbp + 0x97], r12b
1416110f1: lea rcx, [rbp + 0x10]
1416110f5: mov edx, 0x40000
1416110fa: mov r8d, 0x22
141611100: mov r9b, 1
141611103: call 0x140bd4de0
141611108: mov qword ptr [rbp + 0x88], r15
14161110f: mov byte ptr [rbp + 0x97], r12b
141611116: lea r14, [rbp + 0x68]
14161111a: lea rdx, [rbp + 0x10]
14161111e: mov r8d, 4
141611124: mov rcx, r14
141611127: call 0x140bd4f20
14161112c: mov qword ptr [rbp + 0x88], r15
141611133: mov byte ptr [rbp + 0x97], r12b
14161113a: mov qword ptr [rsp + 0x30], rbx
14161113f: mov qword ptr [rsp + 0x28], r15
141611144: mov qword ptr [rsp + 0x20], r14
141611149: lea r8, [rip + 0x166ef90]
141611150: lea rcx, [rbp - 0x40]
141611154: mov r9d, 0x1e
14161115a: mov rdx, rdi
14161115d: call 0x1416a2dc0
141611162: cmp dword ptr [rbp - 0x40], 0xb
141611166: jne 0x1416111e7
141611168: mov rax, qword ptr [rbp - 0x18]
14161116c: mov qword ptr [rsi + 0x20], rax
141611170: movups xmm0, xmmword ptr [rbp - 0x38]
141611174: movups xmm1, xmmword ptr [rbp - 0x28]
141611178: movups xmmword ptr [rsi + 0x10], xmm1
14161117c: movups xmmword ptr [rsi], xmm0
14161117f: test r12b, r12b
141611182: je 0x141611197
141611184: mov edx, 0x40000
141611189: mov r8d, 4
14161118f: mov rcx, r15
141611192: call 0x140613c20
141611197: mov rax, rsi
14161119a: add rsp, 0x120
1416111a1: pop rbx
1416111a2: pop rdi
1416111a3: pop rsi
1416111a4: pop r12
1416111a6: pop r14
1416111a8: pop r15
1416111aa: pop rbp
1416111ab: ret 
1416111ac: mov qword ptr [rbp + 0x10], 0
1416111b4: lea rax, [rip + 0x16b8d8d]
1416111bb: jmp 0x1416111cc
1416111bd: mov qword ptr [rbp + 0x10], 0
1416111c5: lea rax, [rip + 0x16b8d94]
1416111cc: mov qword ptr [rsp + 0x20], rax
1416111d1: lea r8, [rip + 0x16b8d68]
1416111d8: lea rdx, [rbp - 0x40]
1416111dc: lea r9, [rbp + 0x10]
1416111e0: xor ecx, ecx
1416111e2: call 0x1428d9596
1416111e7: mov dword ptr [rbp + 0x84], r12d
1416111ee: mov qword ptr [rbp + 0x78], r15
1416111f2: movups xmm0, xmmword ptr [rbp]
1416111f6: movaps xmmword ptr [rbp + 0x50], xmm0
1416111fa: movups xmm0, xmmword ptr [rbp - 0x40]
1416111fe: movups xmm1, xmmword ptr [rbp - 0x30]
141611202: movups xmm2, xmmword ptr [rbp - 0x20]
141611206: movups xmm3, xmmword ptr [rbp - 0x10]
14161120a: movaps xmmword ptr [rbp + 0x40], xmm3
14161120e: movaps xmmword ptr [rbp + 0x30], xmm2
141611212: movaps xmmword ptr [rbp + 0x20], xmm1
141611216: movaps xmmword ptr [rbp + 0x10], xmm0
14161121a: lea rax, [rip + 0x166ef87]
141611221: mov qword ptr [rsp + 0x20], rax
141611226: lea rcx, [rip + 0x166d8bb]
14161122d: lea r9, [rip + 0x166d894]
141611234: lea r8, [rbp + 0x10]
141611238: mov edx, 0x2b
14161123d: call 0x1428d9760
141611242: jmp 0x1416112ce
141611247: lea rax, [rip + 0x148bd56]
14161124e: mov qword ptr [rbp - 0x40], rax
141611252: mov qword ptr [rbp - 0x38], 0x46
14161125a: lea rax, [rbp - 0x40]
14161125e: mov qword ptr [rbp + 0x68], rax
141611262: lea rax, [rip - 0x15073d9]
141611269: mov qword ptr [rbp + 0x70], rax
14161126d: lea rax, [rip + 0x166e554]
141611274: mov qword ptr [rbp + 0x10], rax
141611278: mov qword ptr [rbp + 0x18], 1
141611280: mov qword ptr [rbp + 0x30], 0
141611288: lea rax, [rbp + 0x68]
14161128c: mov qword ptr [rbp + 0x20], rax
141611290: mov qword ptr [rbp + 0x28], 1
141611298: mov byte ptr [rbp + 0x97], 0
14161129f: lea rdx, [rip + 0x166e532]
1416112a6: lea rcx, [rbp + 0x10]
1416112aa: call 0x1428d9390
1416112af: jmp 0x1416112ce
1416112b1: mov byte ptr [rbp + 0x97], 0
1416112b8: lea r8, [rip + 0x166d7f1]
1416112bf: mov ecx, 4
1416112c4: mov edx, 0x40000
1416112c9: call 0x1428d8fe3
1416112ce: ud2 
1416112d0: mov qword ptr [rsp + 0x10], rdx
1416112d5: push rbp
1416112d6: push r15
1416112d8: push r14
1416112da: push r12
1416112dc: push rsi
1416112dd: push rdi
1416112de: push rbx
1416112df: sub rsp, 0x40
1416112e3: lea rbp, [rdx + 0x80]
1416112ea: mov rsi, qword ptr [rbp + 0x78]
1416112ee: mov qword ptr [rbp + 0x88], rsi
1416112f5: mov edi, dword ptr [rbp + 0x84]
1416112fb: mov byte ptr [rbp + 0x97], dil
141611302: lea rcx, [rbp + 0x10]
141611306: call 0x1407cea70
14161130b: mov qword ptr [rbp + 0x88], rsi
141611312: mov byte ptr [rbp + 0x97], dil
141611319: add rsp, 0x40
14161131d: pop rbx
14161131e: pop rdi
14161131f: pop rsi
141611320: pop r12
141611322: pop r14
141611324: pop r15
141611326: pop rbp
141611327: ret 
141611328: nop dword ptr [rax + rax]
141611330: mov qword ptr [rsp + 0x10], rdx
141611335: push rbp
141611336: push r15
141611338: push r14
14161133a: push r12
14161133c: push rsi
14161133d: push rdi
14161133e: push rbx
14161133f: sub rsp, 0x40
141611343: lea rbp, [rdx + 0x80]
14161134a: test byte ptr [rbp + 0x97], 1
141611351: je 0x14161136a
141611353: mov rcx, qword ptr [rbp + 0x88]
14161135a: mov edx, 0x40000
14161135f: mov r8d, 4
141611365: call 0x140613c20
14161136a: nop 
14161136b: add rsp, 0x40
14161136f: pop rbx
141611370: pop rdi
141611371: pop rsi
141611372: pop r12
141611374: pop r14
141611376: pop r15
141611378: pop rbp
141611379: ret 
