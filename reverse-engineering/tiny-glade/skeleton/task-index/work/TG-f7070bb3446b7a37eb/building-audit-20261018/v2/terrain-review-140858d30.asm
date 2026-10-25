140858d30: 55                       push rbp
140858d31: 4157                     push r15
140858d33: 4156                     push r14
140858d35: 4154                     push r12
140858d37: 56                       push rsi
140858d38: 57                       push rdi
140858d39: 53                       push rbx
140858d3a: 4881ecc0000000           sub rsp, 0xc0
140858d41: 488dac2480000000         lea rbp, [rsp + 0x80]
140858d49: 0f297d30                 movaps xmmword ptr [rbp + 0x30], xmm7
140858d4d: 0f297520                 movaps xmmword ptr [rbp + 0x20], xmm6
140858d51: 48c74518feffffff         mov qword ptr [rbp + 0x18], 0xfffffffffffffffe
140858d59: 4889cf                   mov rdi, rcx
140858d5c: f30f108da8000000         movss xmm1, dword ptr [rbp + 0xa8]
140858d64: 48b90000000000000080     movabs rcx, 0x8000000000000000
140858d6e: 48330f                   xor rcx, qword ptr [rdi]
140858d71: 4883f902                 cmp rcx, 2
140858d75: b802000000               mov eax, 2
140858d7a: 480f42c1                 cmovb rax, rcx
140858d7e: 4885c0                   test rax, rax
140858d81: 0f84d0000000             je 0x140858e57
140858d87: 4883f801                 cmp rax, 1
140858d8b: 0f853f010000             jne 0x140858ed0
140858d91: 8b7708                   mov esi, dword ptr [rdi + 8]
140858d94: f30f107718               movss xmm6, dword ptr [rdi + 0x18]
140858d99: f30f104714               movss xmm0, dword ptr [rdi + 0x14]
140858d9e: 0f28d6                   movaps xmm2, xmm6
140858da1: f30fc2d004               cmpneqss xmm2, xmm0
140858da6: 660f7ed3                 movd ebx, xmm2
140858daa: 83e301                   and ebx, 1
140858dad: f30f59c1                 mulss xmm0, xmm1
140858db1: f30f10154bcb0c02         movss xmm2, dword ptr [rip + 0x20ccb4b]
140858db9: f30f5cd1                 subss xmm2, xmm1
140858dbd: f30f59f2                 mulss xmm6, xmm2
140858dc1: f30f58f0                 addss xmm6, xmm0
140858dc5: f30f104710               movss xmm0, dword ptr [rdi + 0x10]
140858dca: f30f107f0c               movss xmm7, dword ptr [rdi + 0xc]
140858dcf: f30f59d0                 mulss xmm2, xmm0
140858dd3: f30fc2c704               cmpneqss xmm0, xmm7
140858dd8: 660f7ec7                 movd edi, xmm0
140858ddc: 83e701                   and edi, 1
140858ddf: f30f59f9                 mulss xmm7, xmm1
140858de3: f30f58fa                 addss xmm7, xmm2
140858de7: 418b401c                 mov eax, dword ptr [r8 + 0x1c]
140858deb: 498b4810                 mov rcx, qword ptr [r8 + 0x10]
140858def: 8901                     mov dword ptr [rcx], eax
140858df1: 4d8b30                   mov r14, qword ptr [r8]
140858df4: 4d8b7e30                 mov r15, qword ptr [r14 + 0x30]
140858df8: 4d8b6640                 mov r12, qword ptr [r14 + 0x40]
140858dfc: 4d3b7e20                 cmp r15, qword ptr [r14 + 0x20]
140858e00: 7510                     jne 0x140858e12
140858e02: 498d4e20                 lea rcx, [r14 + 0x20]
140858e06: 488d153b133e02           lea rdx, [rip + 0x23e133b]
140858e0d: e89e9cd0ff               call 0x140562ab0
140858e12: 498b4628                 mov rax, qword ptr [r14 + 0x28]
140858e16: 4b8d0cbf                 lea rcx, [r15 + r15*4]
140858e1a: 891cc8                   mov dword ptr [rax + rcx*8], ebx
140858e1d: f30f1174c804             movss dword ptr [rax + rcx*8 + 4], xmm6
140858e23: 897cc808                 mov dword ptr [rax + rcx*8 + 8], edi
140858e27: f30f117cc80c             movss dword ptr [rax + rcx*8 + 0xc], xmm7
140858e2d: 48c744c81000000000       mov qword ptr [rax + rcx*8 + 0x10], 0
140858e36: c744c81800000000         mov dword ptr [rax + rcx*8 + 0x18], 0
140858e3e: 8974c81c                 mov dword ptr [rax + rcx*8 + 0x1c], esi
140858e42: 4c8964c820               mov qword ptr [rax + rcx*8 + 0x20], r12
140858e47: 49ffc7                   inc r15
140858e4a: 4d897e30                 mov qword ptr [r14 + 0x30], r15
140858e4e: 49ff4640                 inc qword ptr [r14 + 0x40]
140858e52: e92c030000               jmp 0x140859183
140858e57: f30f100579ba2302         movss xmm0, dword ptr [rip + 0x223ba79]
140858e5f: 0f2ec1                   ucomiss xmm0, xmm1
140858e62: 0f8673010000             jbe 0x140858fdb
140858e68: 4d89c4                   mov r12, r8
140858e6b: 4d89cf                   mov r15, r9
140858e6e: 8b7720                   mov esi, dword ptr [rdi + 0x20]
140858e71: 488b5f10                 mov rbx, qword ptr [rdi + 0x10]
140858e75: 4c8b7718                 mov r14, qword ptr [rdi + 0x18]
140858e79: f30f101d83ca0c02         movss xmm3, dword ptr [rip + 0x20cca83]
140858e81: f30f5cd9                 subss xmm3, xmm1
140858e85: 4889542420               mov qword ptr [rsp + 0x20], rdx
140858e8a: 89f1                     mov ecx, esi
140858e8c: 4889da                   mov rdx, rbx
140858e8f: 4d89f0                   mov r8, r14
140858e92: e8a9080000               call 0x140859740
140858e97: 0f28f0                   movaps xmm6, xmm0
140858e9a: f30f597724               mulss xmm6, dword ptr [rdi + 0x24]
140858e9f: 4d85f6                   test r14, r14
140858ea2: 0f95c1                   setne cl
140858ea5: 4b8d0476                 lea rax, [r14 + r14*2]
140858ea9: 488d0483                 lea rax, [rbx + rax*4]
140858ead: 4883f80c                 cmp rax, 0xc
140858eb1: 0f95c2                   setne dl
140858eb4: 84d1                     test cl, dl
140858eb6: 0f85ec010000             jne 0x1408590a8
140858ebc: c7450800000000           mov dword ptr [rbp + 8], 0
140858ec3: 48c7450000000000         mov qword ptr [rbp], 0
140858ecb: e9fb010000               jmp 0x1408590cb
140858ed0: 488d7718                 lea rsi, [rdi + 0x18]
140858ed4: 488d0dd5bd2302           lea rcx, [rip + 0x223bdd5]
140858edb: e810d04300               call 0x140c95ef0
140858ee0: f30f11450c               movss dword ptr [rbp + 0xc], xmm0
140858ee5: 488b4708                 mov rax, qword ptr [rdi + 8]
140858ee9: 488b4f10                 mov rcx, qword ptr [rdi + 0x10]
140858eed: 488d0c49                 lea rcx, [rcx + rcx*2]
140858ef1: 488d0c88                 lea rcx, [rax + rcx*4]
140858ef5: 488945d0                 mov qword ptr [rbp - 0x30], rax
140858ef9: 48894dd8                 mov qword ptr [rbp - 0x28], rcx
140858efd: 488975e0                 mov qword ptr [rbp - 0x20], rsi
140858f01: 488d450c                 lea rax, [rbp + 0xc]
140858f05: 488945e8                 mov qword ptr [rbp - 0x18], rax
140858f09: 4c8d05a8ba2302           lea r8, [rip + 0x223baa8]
140858f10: 488d4da8                 lea rcx, [rbp - 0x58]
140858f14: 488d55d0                 lea rdx, [rbp - 0x30]
140858f18: e8b35eab00               call 0x14130edd0
140858f1d: 488b45b8                 mov rax, qword ptr [rbp - 0x48]
140858f21: 4889c1                   mov rcx, rax
140858f24: 48d1e9                   shr rcx, 1
140858f27: 488945c0                 mov qword ptr [rbp - 0x40], rax
140858f2b: 4885c0                   test rax, rax
140858f2e: 0f84b3020000             je 0x1408591e7
140858f34: 488b85a0000000           mov rax, qword ptr [rbp + 0xa0]
140858f3b: 488b55a8                 mov rdx, qword ptr [rbp - 0x58]
140858f3f: 48895510                 mov qword ptr [rbp + 0x10], rdx
140858f43: 4c8b45b0                 mov r8, qword ptr [rbp - 0x50]
140858f47: 488d0c49                 lea rcx, [rcx + rcx*2]
140858f4b: 498b1488                 mov rdx, qword ptr [r8 + rcx*4]
140858f4f: 488955d0                 mov qword ptr [rbp - 0x30], rdx
140858f53: 4c8945c8                 mov qword ptr [rbp - 0x38], r8
140858f57: 418b4c8808               mov ecx, dword ptr [r8 + rcx*4 + 8]
140858f5c: 894dd8                   mov dword ptr [rbp - 0x28], ecx
140858f5f: 8b7f20                   mov edi, dword ptr [rdi + 0x20]
140858f62: 8b481c                   mov ecx, dword ptr [rax + 0x1c]
140858f65: 488b5010                 mov rdx, qword ptr [rax + 0x10]
140858f69: 890a                     mov dword ptr [rdx], ecx
140858f6b: 488b30                   mov rsi, qword ptr [rax]
140858f6e: 488b5e30                 mov rbx, qword ptr [rsi + 0x30]
140858f72: 4c8b7640                 mov r14, qword ptr [rsi + 0x40]
140858f76: 483b5e20                 cmp rbx, qword ptr [rsi + 0x20]
140858f7a: 7510                     jne 0x140858f8c
140858f7c: 488d4e20                 lea rcx, [rsi + 0x20]
140858f80: 488d15c1113e02           lea rdx, [rip + 0x23e11c1]
140858f87: e8649ad0ff               call 0x1405629f0
140858f8c: 488b4628                 mov rax, qword ptr [rsi + 0x28]
140858f90: 488d0c5b                 lea rcx, [rbx + rbx*2]
140858f94: 48c1e104                 shl rcx, 4
140858f98: 488b5510                 mov rdx, qword ptr [rbp + 0x10]
140858f9c: 48891408                 mov qword ptr [rax + rcx], rdx
140858fa0: 488b55c8                 mov rdx, qword ptr [rbp - 0x38]
140858fa4: 4889540808               mov qword ptr [rax + rcx + 8], rdx
140858fa9: 488b55c0                 mov rdx, qword ptr [rbp - 0x40]
140858fad: 4889540810               mov qword ptr [rax + rcx + 0x10], rdx
140858fb2: 488b55d0                 mov rdx, qword ptr [rbp - 0x30]
140858fb6: 4889540818               mov qword ptr [rax + rcx + 0x18], rdx
140858fbb: 8b55d8                   mov edx, dword ptr [rbp - 0x28]
140858fbe: 89540820                 mov dword ptr [rax + rcx + 0x20], edx
140858fc2: 897c0824                 mov dword ptr [rax + rcx + 0x24], edi
140858fc6: 4c89740828               mov qword ptr [rax + rcx + 0x28], r14
140858fcb: 48ffc3                   inc rbx
140858fce: 48895e30                 mov qword ptr [rsi + 0x30], rbx
140858fd2: 48ff4640                 inc qword ptr [rsi + 0x40]
140858fd6: e9a8010000               jmp 0x140859183
140858fdb: 4883c720                 add rdi, 0x20
140858fdf: 8b37                     mov esi, dword ptr [rdi]
140858fe1: 8975d0                   mov dword ptr [rbp - 0x30], esi
140858fe4: 48837a2000               cmp qword ptr [rdx + 0x20], 0
140858fe9: 0f84ae010000             je 0x14085919d
140858fef: 4c89cb                   mov rbx, r9
140858ff2: 488d4a28                 lea rcx, [rdx + 0x28]
140858ff6: 488d45d0                 lea rax, [rbp - 0x30]
140858ffa: 4989d6                   mov r14, rdx
140858ffd: 4889c2                   mov rdx, rax
140859000: e8cb7ed5ff               call 0x1405b0ed0
140859005: 498b4e08                 mov rcx, qword ptr [r14 + 8]
140859009: 498b5610                 mov rdx, qword ptr [r14 + 0x10]
14085900d: 4989d0                   mov r8, rdx
140859010: 4921c0                   and r8, rax
140859013: 48c1e839                 shr rax, 0x39
140859017: 660f6ec0                 movd xmm0, eax
14085901b: 660f60c0                 punpcklbw xmm0, xmm0
14085901f: f20f70c000               pshuflw xmm0, xmm0, 0
140859024: 660f70c000               pshufd xmm0, xmm0, 0
140859029: 488d41d0                 lea rax, [rcx - 0x30]
14085902d: 4531c9                   xor r9d, r9d
140859030: 660f76c9                 pcmpeqd xmm1, xmm1
140859034: f3420f6f1401             movdqu xmm2, xmmword ptr [rcx + r8]
14085903a: 660f6fda                 movdqa xmm3, xmm2
14085903e: 660f74d8                 pcmpeqb xmm3, xmm0
140859042: 66440fd7d3               pmovmskb r10d, xmm3
140859047: 4585d2                   test r10d, r10d
14085904a: 742c                     je 0x140859078
14085904c: 4989df                   mov r15, rbx
14085904f: f3450fbcda               tzcnt r11d, r10d
140859054: 4d01c3                   add r11, r8
140859057: 4921d3                   and r11, rdx
14085905a: 49f7db                   neg r11
14085905d: 4f8d1c5b                 lea r11, [r11 + r11*2]
140859061: 49c1e304                 shl r11, 4
140859065: 42393418                 cmp dword ptr [rax + r11], esi
140859069: 742f                     je 0x14085909a
14085906b: 458d5aff                 lea r11d, [r10 - 1]
14085906f: 664521d3                 and r11w, r10w
140859073: 4589da                   mov r10d, r11d
140859076: 75d7                     jne 0x14085904f
140859078: 660f74d1                 pcmpeqb xmm2, xmm1
14085907c: 66440fd7d2               pmovmskb r10d, xmm2
140859081: 4585d2                   test r10d, r10d
140859084: 0f8513010000             jne 0x14085919d
14085908a: 4d01c8                   add r8, r9
14085908d: 4983c010                 add r8, 0x10
140859091: 4983c110                 add r9, 0x10
140859095: 4921d0                   and r8, rdx
140859098: eb9a                     jmp 0x140859034
14085909a: 42c74419f800000000       mov dword ptr [rcx + r11 - 8], 0
1408590a3: e992000000               jmp 0x14085913a
1408590a8: f30f1040f4               movss xmm0, dword ptr [rax - 0xc]
1408590ad: f30f1048fc               movss xmm1, dword ptr [rax - 4]
1408590b2: f30f1145d0               movss dword ptr [rbp - 0x30], xmm0
1408590b7: f30f114dd4               movss dword ptr [rbp - 0x2c], xmm1
1408590bc: 4889e9                   mov rcx, rbp
1408590bf: 488d55d0                 lea rdx, [rbp - 0x30]
1408590c3: 0f28d6                   movaps xmm2, xmm6
1408590c6: e8153d4400               call 0x140c9cde0
1408590cb: 418b44241c               mov eax, dword ptr [r12 + 0x1c]
1408590d0: 498b4c2410               mov rcx, qword ptr [r12 + 0x10]
1408590d5: 8901                     mov dword ptr [rcx], eax
1408590d7: 498b3c24                 mov rdi, qword ptr [r12]
1408590db: 488b5f30                 mov rbx, qword ptr [rdi + 0x30]
1408590df: 4c8b7740                 mov r14, qword ptr [rdi + 0x40]
1408590e3: 483b5f20                 cmp rbx, qword ptr [rdi + 0x20]
1408590e7: 7510                     jne 0x1408590f9
1408590e9: 488d4f20                 lea rcx, [rdi + 0x20]
1408590ed: 488d1554103e02           lea rdx, [rip + 0x23e1054]
1408590f4: e8b799d0ff               call 0x140562ab0
1408590f9: 488b4728                 mov rax, qword ptr [rdi + 0x28]
1408590fd: 488d0c9b                 lea rcx, [rbx + rbx*4]
140859101: c704c800000000           mov dword ptr [rax + rcx*8], 0
140859108: c744c80801000000         mov dword ptr [rax + rcx*8 + 8], 1
140859110: f30f1174c80c             movss dword ptr [rax + rcx*8 + 0xc], xmm6
140859116: 488b5500                 mov rdx, qword ptr [rbp]
14085911a: 488954c810               mov qword ptr [rax + rcx*8 + 0x10], rdx
14085911f: 8b5508                   mov edx, dword ptr [rbp + 8]
140859122: 8954c818                 mov dword ptr [rax + rcx*8 + 0x18], edx
140859126: 8974c81c                 mov dword ptr [rax + rcx*8 + 0x1c], esi
14085912a: 4c8974c820               mov qword ptr [rax + rcx*8 + 0x20], r14
14085912f: 48ffc3                   inc rbx
140859132: 48895f30                 mov qword ptr [rdi + 0x30], rbx
140859136: 48ff4740                 inc qword ptr [rdi + 0x40]
14085913a: 418b471c                 mov eax, dword ptr [r15 + 0x1c]
14085913e: 498b4f10                 mov rcx, qword ptr [r15 + 0x10]
140859142: 8901                     mov dword ptr [rcx], eax
140859144: 498b3f                   mov rdi, qword ptr [r15]
140859147: 488b5f30                 mov rbx, qword ptr [rdi + 0x30]
14085914b: 4c8b7740                 mov r14, qword ptr [rdi + 0x40]
14085914f: 483b5f20                 cmp rbx, qword ptr [rdi + 0x20]
140859153: 7510                     jne 0x140859165
140859155: 488d4f20                 lea rcx, [rdi + 0x20]
140859159: 488d15e80f3e02           lea rdx, [rip + 0x23e0fe8]
140859160: e8cb94d0ff               call 0x140562630
140859165: 488b4728                 mov rax, qword ptr [rdi + 0x28]
140859169: 4889d9                   mov rcx, rbx
14085916c: 48c1e104                 shl rcx, 4
140859170: 4c893408                 mov qword ptr [rax + rcx], r14
140859174: 89740808                 mov dword ptr [rax + rcx + 8], esi
140859178: 48ffc3                   inc rbx
14085917b: 48895f30                 mov qword ptr [rdi + 0x30], rbx
14085917f: 48ff4740                 inc qword ptr [rdi + 0x40]
140859183: 0f287520                 movaps xmm6, xmmword ptr [rbp + 0x20]
140859187: 0f287d30                 movaps xmm7, xmmword ptr [rbp + 0x30]
14085918b: 4881c4c0000000           add rsp, 0xc0
140859192: 5b                       pop rbx
140859193: 5f                       pop rdi
140859194: 5e                       pop rsi
140859195: 415c                     pop r12
140859197: 415e                     pop r14
140859199: 415f                     pop r15
14085919b: 5d                       pop rbp
14085919c: c3                       ret 
14085919d: 48897da8                 mov qword ptr [rbp - 0x58], rdi
1408591a1: 488d05d8460000           lea rax, [rip + 0x46d8]
1408591a8: 488945b0                 mov qword ptr [rbp - 0x50], rax
1408591ac: 488d0545bb2302           lea rax, [rip + 0x223bb45]
1408591b3: 488945d0                 mov qword ptr [rbp - 0x30], rax
1408591b7: 48c745d802000000         mov qword ptr [rbp - 0x28], 2
1408591bf: 48c745f000000000         mov qword ptr [rbp - 0x10], 0
1408591c7: 488d45a8                 lea rax, [rbp - 0x58]
1408591cb: 488945e0                 mov qword ptr [rbp - 0x20], rax
1408591cf: 48c745e801000000         mov qword ptr [rbp - 0x18], 1
1408591d7: 488d153abb2302           lea rdx, [rip + 0x223bb3a]
1408591de: 488d4dd0                 lea rcx, [rbp - 0x30]
1408591e2: e8a9010802               call 0x1428d9390
1408591e7: 4c8d05caba2302           lea r8, [rip + 0x223baca]
1408591ee: 31d2                     xor edx, edx
1408591f0: e823030802               call 0x1428d9518
1408591f5: 0f0b                     ud2 
1408591f7: 660f1f840000000000       nop word ptr [rax + rax]
140859200: 4889542410               mov qword ptr [rsp + 0x10], rdx
140859205: 55                       push rbp
140859206: 4157                     push r15
140859208: 4156                     push r14
14085920a: 4154                     push r12
14085920c: 56                       push rsi
14085920d: 57                       push rdi
14085920e: 53                       push rbx
14085920f: 4883ec50                 sub rsp, 0x50
140859213: 488daa80000000           lea rbp, [rdx + 0x80]
14085921a: 0f297c2420               movaps xmmword ptr [rsp + 0x20], xmm7
14085921f: 0f29742430               movaps xmmword ptr [rsp + 0x30], xmm6
140859224: 48837d1000               cmp qword ptr [rbp + 0x10], 0
140859229: 741b                     je 0x140859246
14085922b: 488b4510                 mov rax, qword ptr [rbp + 0x10]
14085922f: 48c1e002                 shl rax, 2
140859233: 488d1440                 lea rdx, [rax + rax*2]
140859237: 41b804000000             mov r8d, 4
14085923d: 488b4dc8                 mov rcx, qword ptr [rbp - 0x38]
140859241: e8daa9dbff               call 0x140613c20
140859246: 0f28742430               movaps xmm6, xmmword ptr [rsp + 0x30]
14085924b: 0f287c2420               movaps xmm7, xmmword ptr [rsp + 0x20]
140859250: 4883c450                 add rsp, 0x50
140859254: 5b                       pop rbx
140859255: 5f                       pop rdi
140859256: 5e                       pop rsi
140859257: 415c                     pop r12
140859259: 415e                     pop r14
14085925b: 415f                     pop r15
14085925d: 5d                       pop rbp
14085925e: c3                       ret 
14085925f: 90                       nop 
140859260: 4889542410               mov qword ptr [rsp + 0x10], rdx
140859265: 55                       push rbp
140859266: 4157                     push r15
140859268: 4156                     push r14
14085926a: 4154                     push r12
14085926c: 56                       push rsi
14085926d: 57                       push rdi
14085926e: 53                       push rbx
14085926f: 4883ec50                 sub rsp, 0x50
140859273: 488daa80000000           lea rbp, [rdx + 0x80]
14085927a: 0f297c2420               movaps xmmword ptr [rsp + 0x20], xmm7
14085927f: 0f29742430               movaps xmmword ptr [rsp + 0x30], xmm6
140859284: 48837dc000               cmp qword ptr [rbp - 0x40], 0
140859289: 7520                     jne 0x1408592ab
14085928b: 488b45a8                 mov rax, qword ptr [rbp - 0x58]
14085928f: 4885c0                   test rax, rax
140859292: 7417                     je 0x1408592ab
140859294: 488b4db0                 mov rcx, qword ptr [rbp - 0x50]
140859298: 48c1e002                 shl rax, 2
14085929c: 488d1440                 lea rdx, [rax + rax*2]
1408592a0: 41b804000000             mov r8d, 4
1408592a6: e875a9dbff               call 0x140613c20
1408592ab: 0f28742430               movaps xmm6, xmmword ptr [rsp + 0x30]
1408592b0: 0f287c2420               movaps xmm7, xmmword ptr [rsp + 0x20]
1408592b5: 4883c450                 add rsp, 0x50
1408592b9: 5b                       pop rbx
1408592ba: 5f                       pop rdi
1408592bb: 5e                       pop rsi
1408592bc: 415c                     pop r12
1408592be: 415e                     pop r14
1408592c0: 415f                     pop r15
1408592c2: 5d                       pop rbp
1408592c3: c3                       ret 
1408592c4: cc                       int3 
1408592c5: cc                       int3 
1408592c6: cc                       int3 
1408592c7: cc                       int3 
1408592c8: cc                       int3 
1408592c9: cc                       int3 
1408592ca: cc                       int3 
1408592cb: cc                       int3 
1408592cc: cc                       int3 
1408592cd: cc                       int3 
1408592ce: cc                       int3 
1408592cf: cc                       int3 
