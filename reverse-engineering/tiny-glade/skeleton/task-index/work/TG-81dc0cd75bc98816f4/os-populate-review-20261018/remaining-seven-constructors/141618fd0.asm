141618fd0: push rbp
141618fd1: push r15
141618fd3: push r14
141618fd5: push r12
141618fd7: push rsi
141618fd8: push rdi
141618fd9: push rbx
141618fda: sub rsp, 0x100
141618fe1: lea rbp, [rsp + 0x80]
141618fe9: mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
141618ff1: mov rsi, rcx
141618ff4: test r8, r8
141618ff7: sete al
141618ffa: cmp r9, 0x27101
141619001: setb cl
141619004: or cl, al
141619006: je 0x1416190a3
14161900c: mov r14, r9
14161900f: mov rbx, r8
141619012: mov rdi, rdx
141619015: lea r15, [rbp + 0x10]
141619019: mov edx, 0x271000
14161901e: mov rcx, r15
141619021: mov r8d, 0x22
141619027: mov r9b, 1
14161902a: call 0x140bd4de0
14161902f: lea r12, [rbp + 0x68]
141619033: mov r8d, 4
141619039: mov rcx, r12
14161903c: mov rdx, r15
14161903f: call 0x140bd4f20
141619044: shl r14, 4
141619048: mov qword ptr [rsp + 0x30], r14
14161904d: mov qword ptr [rsp + 0x28], rbx
141619052: mov qword ptr [rsp + 0x20], r12
141619057: lea r8, [rip + 0x1666df1]
14161905e: lea rcx, [rbp - 0x40]
141619062: mov r9d, 0x15
141619068: mov rdx, rdi
14161906b: call 0x1416a2dc0
141619070: cmp dword ptr [rbp - 0x40], 0xb
141619074: jne 0x141619104
14161907a: mov rax, qword ptr [rbp - 0x18]
14161907e: mov qword ptr [rsi + 0x20], rax
141619082: movups xmm0, xmmword ptr [rbp - 0x38]
141619086: movups xmm1, xmmword ptr [rbp - 0x28]
14161908a: movups xmmword ptr [rsi + 0x10], xmm1
14161908e: movups xmmword ptr [rsi], xmm0
141619091: add rsp, 0x100
141619098: pop rbx
141619099: pop rdi
14161909a: pop rsi
14161909b: pop r12
14161909d: pop r14
14161909f: pop r15
1416190a1: pop rbp
1416190a2: ret 
1416190a3: lea rax, [rip + 0x1484a62]
1416190aa: mov qword ptr [rbp - 0x40], rax
1416190ae: mov qword ptr [rbp - 0x38], 0x50
1416190b6: lea rax, [rbp - 0x40]
1416190ba: mov qword ptr [rbp + 0x68], rax
1416190be: lea rax, [rip - 0x150f235]
1416190c5: mov qword ptr [rbp + 0x70], rax
1416190c9: lea rax, [rip + 0x16666f8]
1416190d0: mov qword ptr [rbp + 0x10], rax
1416190d4: mov qword ptr [rbp + 0x18], 1
1416190dc: mov qword ptr [rbp + 0x30], 0
1416190e4: lea rax, [rbp + 0x68]
1416190e8: mov qword ptr [rbp + 0x20], rax
1416190ec: mov qword ptr [rbp + 0x28], 1
1416190f4: lea rdx, [rip + 0x16666dd]
1416190fb: lea rcx, [rbp + 0x10]
1416190ff: call 0x1428d9390
141619104: movups xmm0, xmmword ptr [rbp]
141619108: movaps xmmword ptr [rbp + 0x50], xmm0
14161910c: movups xmm0, xmmword ptr [rbp - 0x40]
141619110: movups xmm1, xmmword ptr [rbp - 0x30]
141619114: movups xmm2, xmmword ptr [rbp - 0x20]
141619118: movups xmm3, xmmword ptr [rbp - 0x10]
14161911c: movaps xmmword ptr [rbp + 0x40], xmm3
141619120: movaps xmmword ptr [rbp + 0x30], xmm2
141619124: movaps xmmword ptr [rbp + 0x20], xmm1
141619128: movaps xmmword ptr [rbp + 0x10], xmm0
14161912c: lea rax, [rip + 0x1667075]
141619133: mov qword ptr [rsp + 0x20], rax
141619138: lea rcx, [rip + 0x16659a9]
14161913f: lea r9, [rip + 0x1665982]
141619146: lea r8, [rbp + 0x10]
14161914a: mov edx, 0x2b
14161914f: call 0x1428d9760
141619154: ud2 
141619156: nop word ptr cs:[rax + rax]
141619160: mov qword ptr [rsp + 0x10], rdx
141619165: push rbp
141619166: push r15
141619168: push r14
14161916a: push r12
14161916c: push rsi
14161916d: push rdi
14161916e: push rbx
14161916f: sub rsp, 0x40
141619173: lea rbp, [rdx + 0x80]
14161917a: lea rcx, [rbp + 0x10]
14161917e: call 0x1407cea70
141619183: nop 
141619184: add rsp, 0x40
141619188: pop rbx
141619189: pop rdi
14161918a: pop rsi
14161918b: pop r12
14161918d: pop r14
14161918f: pop r15
141619191: pop rbp
141619192: ret 
