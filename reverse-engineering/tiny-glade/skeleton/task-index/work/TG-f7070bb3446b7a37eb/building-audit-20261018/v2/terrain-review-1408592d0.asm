1408592d0: 55                       push rbp
1408592d1: 4157                     push r15
1408592d3: 4156                     push r14
1408592d5: 4154                     push r12
1408592d7: 56                       push rsi
1408592d8: 57                       push rdi
1408592d9: 53                       push rbx
1408592da: 4881ecb0000000           sub rsp, 0xb0
1408592e1: 488dac2480000000         lea rbp, [rsp + 0x80]
1408592e9: 0f297d20                 movaps xmmword ptr [rbp + 0x20], xmm7
1408592ed: 0f297510                 movaps xmmword ptr [rbp + 0x10], xmm6
1408592f1: 48c74508feffffff         mov qword ptr [rbp + 8], 0xfffffffffffffffe
1408592f9: 4889cf                   mov rdi, rcx
1408592fc: f30f108d98000000         movss xmm1, dword ptr [rbp + 0x98]
140859304: 48b90000000000000080     movabs rcx, 0x8000000000000000
14085930e: 48330f                   xor rcx, qword ptr [rdi]
140859311: 4883f902                 cmp rcx, 2
140859315: b802000000               mov eax, 2
14085931a: 480f42c1                 cmovb rax, rcx
14085931e: 4885c0                   test rax, rax
140859321: 0f84d3000000             je 0x1408593fa
140859327: 4883f801                 cmp rax, 1
14085932b: 0f8528010000             jne 0x140859459
140859331: 8b7708                   mov esi, dword ptr [rdi + 8]
140859334: f30f107718               movss xmm6, dword ptr [rdi + 0x18]
140859339: f30f104714               movss xmm0, dword ptr [rdi + 0x14]
14085933e: 0f28d6                   movaps xmm2, xmm6
140859341: f30fc2d004               cmpneqss xmm2, xmm0
140859346: 660f7ed3                 movd ebx, xmm2
14085934a: 83e301                   and ebx, 1
14085934d: f30f1015afc50c02         movss xmm2, dword ptr [rip + 0x20cc5af]
140859355: f30f5cd1                 subss xmm2, xmm1
140859359: f30f59c2                 mulss xmm0, xmm2
14085935d: f30f59f1                 mulss xmm6, xmm1
140859361: f30f58f0                 addss xmm6, xmm0
140859365: f30f107f10               movss xmm7, dword ptr [rdi + 0x10]
14085936a: f30f10470c               movss xmm0, dword ptr [rdi + 0xc]
14085936f: 0f28df                   movaps xmm3, xmm7
140859372: f30fc2d804               cmpneqss xmm3, xmm0
140859377: 660f7edf                 movd edi, xmm3
14085937b: 83e701                   and edi, 1
14085937e: f30f59d0                 mulss xmm2, xmm0
140859382: f30f59f9                 mulss xmm7, xmm1
140859386: f30f58fa                 addss xmm7, xmm2
14085938a: 418b401c                 mov eax, dword ptr [r8 + 0x1c]
14085938e: 498b4810                 mov rcx, qword ptr [r8 + 0x10]
140859392: 8901                     mov dword ptr [rcx], eax
140859394: 4d8b30                   mov r14, qword ptr [r8]
140859397: 4d8b7e30                 mov r15, qword ptr [r14 + 0x30]
14085939b: 4d8b6640                 mov r12, qword ptr [r14 + 0x40]
14085939f: 4d3b7e20                 cmp r15, qword ptr [r14 + 0x20]
1408593a3: 7510                     jne 0x1408593b5
1408593a5: 498d4e20                 lea rcx, [r14 + 0x20]
1408593a9: 488d15980d3e02           lea rdx, [rip + 0x23e0d98]
1408593b0: e8fb96d0ff               call 0x140562ab0
1408593b5: 498b4628                 mov rax, qword ptr [r14 + 0x28]
1408593b9: 4b8d0cbf                 lea rcx, [r15 + r15*4]
1408593bd: 891cc8                   mov dword ptr [rax + rcx*8], ebx
1408593c0: f30f1174c804             movss dword ptr [rax + rcx*8 + 4], xmm6
1408593c6: 897cc808                 mov dword ptr [rax + rcx*8 + 8], edi
1408593ca: f30f117cc80c             movss dword ptr [rax + rcx*8 + 0xc], xmm7
1408593d0: 48c744c81000000000       mov qword ptr [rax + rcx*8 + 0x10], 0
1408593d9: c744c81800000000         mov dword ptr [rax + rcx*8 + 0x18], 0
1408593e1: 8974c81c                 mov dword ptr [rax + rcx*8 + 0x1c], esi
1408593e5: 4c8964c820               mov qword ptr [rax + rcx*8 + 0x20], r12
1408593ea: 49ffc7                   inc r15
1408593ed: 4d897e30                 mov qword ptr [r14 + 0x30], r15
1408593f1: 49ff4640                 inc qword ptr [r14 + 0x40]
1408593f5: e946020000               jmp 0x140859640
1408593fa: 4d89cf                   mov r15, r9
1408593fd: 4d89c4                   mov r12, r8
140859400: 8b7720                   mov esi, dword ptr [rdi + 0x20]
140859403: 488b5f10                 mov rbx, qword ptr [rdi + 0x10]
140859407: 4c8b7718                 mov r14, qword ptr [rdi + 0x18]
14085940b: 4889542420               mov qword ptr [rsp + 0x20], rdx
140859410: 89f1                     mov ecx, esi
140859412: 4889da                   mov rdx, rbx
140859415: 4d89f0                   mov r8, r14
140859418: 0f28d9                   movaps xmm3, xmm1
14085941b: e820030000               call 0x140859740
140859420: 0f28f0                   movaps xmm6, xmm0
140859423: f30f597724               mulss xmm6, dword ptr [rdi + 0x24]
140859428: 4d85f6                   test r14, r14
14085942b: 0f95c1                   setne cl
14085942e: 4b8d0476                 lea rax, [r14 + r14*2]
140859432: 488d0483                 lea rax, [rbx + rax*4]
140859436: 4883f80c                 cmp rax, 0xc
14085943a: 0f95c2                   setne dl
14085943d: 84d1                     test cl, dl
14085943f: 0f851f010000             jne 0x140859564
140859445: c745f800000000           mov dword ptr [rbp - 8], 0
14085944c: 48c745f000000000         mov qword ptr [rbp - 0x10], 0
140859454: e92f010000               jmp 0x140859588
140859459: 488d7718                 lea rsi, [rdi + 0x18]
14085945d: 488d0d4cb82302           lea rcx, [rip + 0x223b84c]
140859464: e887ca4300               call 0x140c95ef0
140859469: f30f1145fc               movss dword ptr [rbp - 4], xmm0
14085946e: 488b4708                 mov rax, qword ptr [rdi + 8]
140859472: 488b4f10                 mov rcx, qword ptr [rdi + 0x10]
140859476: 488d0c49                 lea rcx, [rcx + rcx*2]
14085947a: 488d0c88                 lea rcx, [rax + rcx*4]
14085947e: 488945c0                 mov qword ptr [rbp - 0x40], rax
140859482: 48894dc8                 mov qword ptr [rbp - 0x38], rcx
140859486: 488975d0                 mov qword ptr [rbp - 0x30], rsi
14085948a: 488d45fc                 lea rax, [rbp - 4]
14085948e: 488945d8                 mov qword ptr [rbp - 0x28], rax
140859492: 4c8d051fb52302           lea r8, [rip + 0x223b51f]
140859499: 488d4da8                 lea rcx, [rbp - 0x58]
14085949d: 488d55c0                 lea rdx, [rbp - 0x40]
1408594a1: e88a92aa00               call 0x141302730
1408594a6: 488b45b8                 mov rax, qword ptr [rbp - 0x48]
1408594aa: 4889c1                   mov rcx, rax
1408594ad: 48d1e9                   shr rcx, 1
1408594b0: 488945e0                 mov qword ptr [rbp - 0x20], rax
1408594b4: 4885c0                   test rax, rax
1408594b7: 0f849d010000             je 0x14085965a
1408594bd: 488b8590000000           mov rax, qword ptr [rbp + 0x90]
1408594c4: 488b55a8                 mov rdx, qword ptr [rbp - 0x58]
1408594c8: 48895500                 mov qword ptr [rbp], rdx
1408594cc: 4c8b45b0                 mov r8, qword ptr [rbp - 0x50]
1408594d0: 488d0c49                 lea rcx, [rcx + rcx*2]
1408594d4: 498b1488                 mov rdx, qword ptr [r8 + rcx*4]
1408594d8: 488955c0                 mov qword ptr [rbp - 0x40], rdx
1408594dc: 4c8945e8                 mov qword ptr [rbp - 0x18], r8
1408594e0: 418b4c8808               mov ecx, dword ptr [r8 + rcx*4 + 8]
1408594e5: 894dc8                   mov dword ptr [rbp - 0x38], ecx
1408594e8: 8b7f20                   mov edi, dword ptr [rdi + 0x20]
1408594eb: 8b481c                   mov ecx, dword ptr [rax + 0x1c]
1408594ee: 488b5010                 mov rdx, qword ptr [rax + 0x10]
1408594f2: 890a                     mov dword ptr [rdx], ecx
1408594f4: 488b30                   mov rsi, qword ptr [rax]
1408594f7: 488b5e30                 mov rbx, qword ptr [rsi + 0x30]
1408594fb: 4c8b7640                 mov r14, qword ptr [rsi + 0x40]
1408594ff: 483b5e20                 cmp rbx, qword ptr [rsi + 0x20]
140859503: 7510                     jne 0x140859515
140859505: 488d4e20                 lea rcx, [rsi + 0x20]
140859509: 488d15380c3e02           lea rdx, [rip + 0x23e0c38]
140859510: e8db94d0ff               call 0x1405629f0
140859515: 488b4628                 mov rax, qword ptr [rsi + 0x28]
140859519: 488d0c5b                 lea rcx, [rbx + rbx*2]
14085951d: 48c1e104                 shl rcx, 4
140859521: 488b5500                 mov rdx, qword ptr [rbp]
140859525: 48891408                 mov qword ptr [rax + rcx], rdx
140859529: 488b55e8                 mov rdx, qword ptr [rbp - 0x18]
14085952d: 4889540808               mov qword ptr [rax + rcx + 8], rdx
140859532: 488b55e0                 mov rdx, qword ptr [rbp - 0x20]
140859536: 4889540810               mov qword ptr [rax + rcx + 0x10], rdx
14085953b: 488b55c0                 mov rdx, qword ptr [rbp - 0x40]
14085953f: 4889540818               mov qword ptr [rax + rcx + 0x18], rdx
140859544: 8b55c8                   mov edx, dword ptr [rbp - 0x38]
140859547: 89540820                 mov dword ptr [rax + rcx + 0x20], edx
14085954b: 897c0824                 mov dword ptr [rax + rcx + 0x24], edi
14085954f: 4c89740828               mov qword ptr [rax + rcx + 0x28], r14
140859554: 48ffc3                   inc rbx
140859557: 48895e30                 mov qword ptr [rsi + 0x30], rbx
14085955b: 48ff4640                 inc qword ptr [rsi + 0x40]
14085955f: e9dc000000               jmp 0x140859640
140859564: f30f1040f4               movss xmm0, dword ptr [rax - 0xc]
140859569: f30f1048fc               movss xmm1, dword ptr [rax - 4]
14085956e: f30f1145c0               movss dword ptr [rbp - 0x40], xmm0
140859573: f30f114dc4               movss dword ptr [rbp - 0x3c], xmm1
140859578: 488d4df0                 lea rcx, [rbp - 0x10]
14085957c: 488d55c0                 lea rdx, [rbp - 0x40]
140859580: 0f28d6                   movaps xmm2, xmm6
140859583: e858384400               call 0x140c9cde0
140859588: 418b44241c               mov eax, dword ptr [r12 + 0x1c]
14085958d: 498b4c2410               mov rcx, qword ptr [r12 + 0x10]
140859592: 8901                     mov dword ptr [rcx], eax
140859594: 498b3c24                 mov rdi, qword ptr [r12]
140859598: 488b5f30                 mov rbx, qword ptr [rdi + 0x30]
14085959c: 4c8b7740                 mov r14, qword ptr [rdi + 0x40]
1408595a0: 483b5f20                 cmp rbx, qword ptr [rdi + 0x20]
1408595a4: 7510                     jne 0x1408595b6
1408595a6: 488d4f20                 lea rcx, [rdi + 0x20]
1408595aa: 488d15970b3e02           lea rdx, [rip + 0x23e0b97]
1408595b1: e8fa94d0ff               call 0x140562ab0
1408595b6: 488b4728                 mov rax, qword ptr [rdi + 0x28]
1408595ba: 488d0c9b                 lea rcx, [rbx + rbx*4]
1408595be: c704c800000000           mov dword ptr [rax + rcx*8], 0
1408595c5: c744c80801000000         mov dword ptr [rax + rcx*8 + 8], 1
1408595cd: f30f1174c80c             movss dword ptr [rax + rcx*8 + 0xc], xmm6
1408595d3: 488b55f0                 mov rdx, qword ptr [rbp - 0x10]
1408595d7: 488954c810               mov qword ptr [rax + rcx*8 + 0x10], rdx
1408595dc: 8b55f8                   mov edx, dword ptr [rbp - 8]
1408595df: 8954c818                 mov dword ptr [rax + rcx*8 + 0x18], edx
1408595e3: 8974c81c                 mov dword ptr [rax + rcx*8 + 0x1c], esi
1408595e7: 4c8974c820               mov qword ptr [rax + rcx*8 + 0x20], r14
1408595ec: 48ffc3                   inc rbx
1408595ef: 48895f30                 mov qword ptr [rdi + 0x30], rbx
1408595f3: 48ff4740                 inc qword ptr [rdi + 0x40]
1408595f7: 418b471c                 mov eax, dword ptr [r15 + 0x1c]
1408595fb: 498b4f10                 mov rcx, qword ptr [r15 + 0x10]
1408595ff: 8901                     mov dword ptr [rcx], eax
140859601: 498b3f                   mov rdi, qword ptr [r15]
140859604: 488b5f30                 mov rbx, qword ptr [rdi + 0x30]
140859608: 4c8b7740                 mov r14, qword ptr [rdi + 0x40]
14085960c: 483b5f20                 cmp rbx, qword ptr [rdi + 0x20]
140859610: 7510                     jne 0x140859622
140859612: 488d4f20                 lea rcx, [rdi + 0x20]
140859616: 488d152b0b3e02           lea rdx, [rip + 0x23e0b2b]
14085961d: e80e90d0ff               call 0x140562630
140859622: 488b4728                 mov rax, qword ptr [rdi + 0x28]
140859626: 4889d9                   mov rcx, rbx
140859629: 48c1e104                 shl rcx, 4
14085962d: 4c893408                 mov qword ptr [rax + rcx], r14
140859631: 89740808                 mov dword ptr [rax + rcx + 8], esi
140859635: 48ffc3                   inc rbx
140859638: 48895f30                 mov qword ptr [rdi + 0x30], rbx
14085963c: 48ff4740                 inc qword ptr [rdi + 0x40]
140859640: 0f287510                 movaps xmm6, xmmword ptr [rbp + 0x10]
140859644: 0f287d20                 movaps xmm7, xmmword ptr [rbp + 0x20]
140859648: 4881c4b0000000           add rsp, 0xb0
14085964f: 5b                       pop rbx
140859650: 5f                       pop rdi
140859651: 5e                       pop rsi
140859652: 415c                     pop r12
140859654: 415e                     pop r14
140859656: 415f                     pop r15
140859658: 5d                       pop rbp
140859659: c3                       ret 
14085965a: 4c8d05cfb62302           lea r8, [rip + 0x223b6cf]
140859661: 31d2                     xor edx, edx
140859663: e8b0fe0702               call 0x1428d9518
140859668: 0f0b                     ud2 
14085966a: 660f1f440000             nop word ptr [rax + rax]
140859670: 4889542410               mov qword ptr [rsp + 0x10], rdx
140859675: 55                       push rbp
140859676: 4157                     push r15
140859678: 4156                     push r14
14085967a: 4154                     push r12
14085967c: 56                       push rsi
14085967d: 57                       push rdi
14085967e: 53                       push rbx
14085967f: 4883ec50                 sub rsp, 0x50
140859683: 488daa80000000           lea rbp, [rdx + 0x80]
14085968a: 0f297c2420               movaps xmmword ptr [rsp + 0x20], xmm7
14085968f: 0f29742430               movaps xmmword ptr [rsp + 0x30], xmm6
140859694: 48837d0000               cmp qword ptr [rbp], 0
140859699: 741b                     je 0x1408596b6
14085969b: 488b4500                 mov rax, qword ptr [rbp]
14085969f: 48c1e002                 shl rax, 2
1408596a3: 488d1440                 lea rdx, [rax + rax*2]
1408596a7: 41b804000000             mov r8d, 4
1408596ad: 488b4de8                 mov rcx, qword ptr [rbp - 0x18]
1408596b1: e86aa5dbff               call 0x140613c20
1408596b6: 0f28742430               movaps xmm6, xmmword ptr [rsp + 0x30]
1408596bb: 0f287c2420               movaps xmm7, xmmword ptr [rsp + 0x20]
1408596c0: 4883c450                 add rsp, 0x50
1408596c4: 5b                       pop rbx
1408596c5: 5f                       pop rdi
1408596c6: 5e                       pop rsi
1408596c7: 415c                     pop r12
1408596c9: 415e                     pop r14
1408596cb: 415f                     pop r15
1408596cd: 5d                       pop rbp
1408596ce: c3                       ret 
1408596cf: 90                       nop 
1408596d0: 4889542410               mov qword ptr [rsp + 0x10], rdx
1408596d5: 55                       push rbp
1408596d6: 4157                     push r15
1408596d8: 4156                     push r14
1408596da: 4154                     push r12
1408596dc: 56                       push rsi
1408596dd: 57                       push rdi
1408596de: 53                       push rbx
1408596df: 4883ec50                 sub rsp, 0x50
1408596e3: 488daa80000000           lea rbp, [rdx + 0x80]
1408596ea: 0f297c2420               movaps xmmword ptr [rsp + 0x20], xmm7
1408596ef: 0f29742430               movaps xmmword ptr [rsp + 0x30], xmm6
1408596f4: 48837de000               cmp qword ptr [rbp - 0x20], 0
1408596f9: 7520                     jne 0x14085971b
1408596fb: 488b45a8                 mov rax, qword ptr [rbp - 0x58]
1408596ff: 4885c0                   test rax, rax
140859702: 7417                     je 0x14085971b
140859704: 488b4db0                 mov rcx, qword ptr [rbp - 0x50]
140859708: 48c1e002                 shl rax, 2
14085970c: 488d1440                 lea rdx, [rax + rax*2]
140859710: 41b804000000             mov r8d, 4
140859716: e805a5dbff               call 0x140613c20
14085971b: 0f28742430               movaps xmm6, xmmword ptr [rsp + 0x30]
140859720: 0f287c2420               movaps xmm7, xmmword ptr [rsp + 0x20]
140859725: 4883c450                 add rsp, 0x50
140859729: 5b                       pop rbx
14085972a: 5f                       pop rdi
14085972b: 5e                       pop rsi
14085972c: 415c                     pop r12
14085972e: 415e                     pop r14
140859730: 415f                     pop r15
140859732: 5d                       pop rbp
140859733: c3                       ret 
140859734: cc                       int3 
140859735: cc                       int3 
140859736: cc                       int3 
140859737: cc                       int3 
140859738: cc                       int3 
140859739: cc                       int3 
14085973a: cc                       int3 
14085973b: cc                       int3 
14085973c: cc                       int3 
14085973d: cc                       int3 
14085973e: cc                       int3 
14085973f: cc                       int3 
