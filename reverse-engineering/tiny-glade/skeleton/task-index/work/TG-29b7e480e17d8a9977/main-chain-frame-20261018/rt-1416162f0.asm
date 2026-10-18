1416162f0: push rbp
1416162f1: push r15
1416162f3: push r14
1416162f5: push r12
1416162f7: push rsi
1416162f8: push rdi
1416162f9: push rbx
1416162fa: sub rsp, 0x100
141616301: lea rbp, [rsp + 0x80]
141616309: mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
141616311: mov rsi, rcx
141616314: test r8, r8
141616317: sete al
14161631a: cmp r9, 0x40001
141616321: setb cl
141616324: or cl, al
141616326: je 0x1416163c7
14161632c: mov r14, r9
14161632f: mov rbx, r8
141616332: mov rdi, rdx
141616335: lea r15, [rbp + 0x10]
141616339: mov edx, 0x600000
14161633e: mov rcx, r15
141616341: mov r8d, 0x22
141616347: mov r9b, 1
14161634a: call 0x140bd4de0
14161634f: lea r12, [rbp + 0x68]
141616353: mov r8d, 4
141616359: mov rcx, r12
14161635c: mov rdx, r15
14161635f: call 0x140bd4f20
141616364: shl r14, 3
141616368: lea rax, [r14 + r14*2]
14161636c: mov qword ptr [rsp + 0x30], rax
141616371: mov qword ptr [rsp + 0x28], rbx
141616376: mov qword ptr [rsp + 0x20], r12
14161637b: lea r8, [rip + 0x16695ae]
141616382: lea rcx, [rbp - 0x40]
141616386: mov r9d, 0xb
14161638c: mov rdx, rdi
14161638f: call 0x1416a2dc0
141616394: cmp dword ptr [rbp - 0x40], 0xb
141616398: jne 0x141616428
14161639e: mov rax, qword ptr [rbp - 0x18]
1416163a2: mov qword ptr [rsi + 0x20], rax
1416163a6: movups xmm0, xmmword ptr [rbp - 0x38]
1416163aa: movups xmm1, xmmword ptr [rbp - 0x28]
1416163ae: movups xmmword ptr [rsi + 0x10], xmm1
1416163b2: movups xmmword ptr [rsi], xmm0
1416163b5: add rsp, 0x100
1416163bc: pop rbx
1416163bd: pop rdi
1416163be: pop rsi
1416163bf: pop r12
1416163c1: pop r14
1416163c3: pop r15
1416163c5: pop rbp
1416163c6: ret 
1416163c7: lea rax, [rip + 0x1487862]
1416163ce: mov qword ptr [rbp - 0x40], rax
1416163d2: mov qword ptr [rbp - 0x38], 0x2c
1416163da: lea rax, [rbp - 0x40]
1416163de: mov qword ptr [rbp + 0x68], rax
1416163e2: lea rax, [rip - 0x150c559]
1416163e9: mov qword ptr [rbp + 0x70], rax
1416163ed: lea rax, [rip + 0x16693d4]
1416163f4: mov qword ptr [rbp + 0x10], rax
1416163f8: mov qword ptr [rbp + 0x18], 1
141616400: mov qword ptr [rbp + 0x30], 0
141616408: lea rax, [rbp + 0x68]
14161640c: mov qword ptr [rbp + 0x20], rax
141616410: mov qword ptr [rbp + 0x28], 1
141616418: lea rdx, [rip + 0x16693b9]
14161641f: lea rcx, [rbp + 0x10]
141616423: call 0x1428d9390
141616428: movups xmm0, xmmword ptr [rbp]
14161642c: movaps xmmword ptr [rbp + 0x50], xmm0
141616430: movups xmm0, xmmword ptr [rbp - 0x40]
141616434: movups xmm1, xmmword ptr [rbp - 0x30]
141616438: movups xmm2, xmmword ptr [rbp - 0x20]
14161643c: movups xmm3, xmmword ptr [rbp - 0x10]
141616440: movaps xmmword ptr [rbp + 0x40], xmm3
141616444: movaps xmmword ptr [rbp + 0x30], xmm2
141616448: movaps xmmword ptr [rbp + 0x20], xmm1
14161644c: movaps xmmword ptr [rbp + 0x10], xmm0
141616450: lea rax, [rip + 0x1669d51]
141616457: mov qword ptr [rsp + 0x20], rax
14161645c: lea rcx, [rip + 0x1668685]
141616463: lea r9, [rip + 0x166865e]
14161646a: lea r8, [rbp + 0x10]
14161646e: mov edx, 0x2b
141616473: call 0x1428d9760
141616478: ud2 
14161647a: nop word ptr [rax + rax]
141616480: mov qword ptr [rsp + 0x10], rdx
141616485: push rbp
141616486: push r15
141616488: push r14
14161648a: push r12
14161648c: push rsi
14161648d: push rdi
14161648e: push rbx
14161648f: sub rsp, 0x40
141616493: lea rbp, [rdx + 0x80]
14161649a: lea rcx, [rbp + 0x10]
14161649e: call 0x1407cea70
1416164a3: nop 
1416164a4: add rsp, 0x40
1416164a8: pop rbx
1416164a9: pop rdi
1416164aa: pop rsi
1416164ab: pop r12
1416164ad: pop r14
1416164af: pop r15
1416164b1: pop rbp
1416164b2: ret 
