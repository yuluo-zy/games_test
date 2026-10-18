1409125e0: push rbp
1409125e1: push rsi
1409125e2: push rdi
1409125e3: sub rsp, 0x3e0
1409125ea: lea rbp, [rsp + 0x80]
1409125f2: mov qword ptr [rbp + 0x358], 0xfffffffffffffffe
1409125fd: mov rsi, rdx
140912600: lea r8, [rip + 0x21994c1]
140912607: lea rdi, [rbp + 0x80]
14091260e: mov rcx, rdi
140912611: mov dl, 9
140912613: call 0x1416fe600
140912618: lea rcx, [rbp + 0x270]
14091261f: mov rdx, rdi
140912622: call 0x14144dc80
140912627: mov rcx, qword ptr [rip + 0x2d19d42]
14091262e: lea r8, [rip + 0x238493b]
140912635: lea rdx, [rbp + 0x200]
14091263c: call 0x140b218a0
140912641: mov r8, rdx
140912644: lea rcx, [rbp + 0x270]
14091264b: mov rdx, rax
14091264e: call 0x141452ef0
140912653: movups xmm0, xmmword ptr [rbp + 0x2d0]
14091265a: movaps xmmword ptr [rbp + 0xe0], xmm0
140912661: movups xmm0, xmmword ptr [rbp + 0x2c0]
140912668: movaps xmmword ptr [rbp + 0xd0], xmm0
14091266f: movups xmm0, xmmword ptr [rbp + 0x2b0]
140912676: movaps xmmword ptr [rbp + 0xc0], xmm0
14091267d: movups xmm0, xmmword ptr [rbp + 0x270]
140912684: movups xmm1, xmmword ptr [rbp + 0x280]
14091268b: movups xmm2, xmmword ptr [rbp + 0x290]
140912692: movups xmm3, xmmword ptr [rbp + 0x2a0]
140912699: movaps xmmword ptr [rbp + 0xb0], xmm3
1409126a0: movaps xmmword ptr [rbp + 0xa0], xmm2
1409126a7: movaps xmmword ptr [rbp + 0x90], xmm1
1409126ae: movaps xmmword ptr [rbp + 0x80], xmm0
1409126b5: lea rdx, [rbp + 0x80]
1409126bc: mov rcx, rsi
1409126bf: call 0x140b67280
1409126c4: lea rcx, [rbp + 0x10]
1409126c8: call 0x1416fe870
1409126cd: lea rcx, [rbp - 0x60]
1409126d1: call 0x1416fea60
1409126d6: lea rcx, [rbp + 0x80]
1409126dd: call 0x141564e80
1409126e2: lea rcx, [rbp + 0x200]
1409126e9: lea rdx, [rbp + 0x80]
1409126f0: call 0x1416fec40
1409126f5: movups xmm0, xmmword ptr [rbp - 0x60]
1409126f9: movups xmm1, xmmword ptr [rbp - 0x50]
1409126fd: movups xmm2, xmmword ptr [rbp - 0x40]
140912701: movups xmm3, xmmword ptr [rbp - 0x30]
140912705: movaps xmmword ptr [rbp + 0x270], xmm0
14091270c: movaps xmmword ptr [rbp + 0x280], xmm1
140912713: movaps xmmword ptr [rbp + 0x290], xmm2
14091271a: movaps xmmword ptr [rbp + 0x2a0], xmm3
140912721: movups xmm0, xmmword ptr [rbp - 0x20]
140912725: movaps xmmword ptr [rbp + 0x2b0], xmm0
14091272c: movups xmm0, xmmword ptr [rbp - 0x10]
140912730: movaps xmmword ptr [rbp + 0x2c0], xmm0
140912737: movups xmm0, xmmword ptr [rbp]
14091273b: movaps xmmword ptr [rbp + 0x2d0], xmm0
140912742: movups xmm0, xmmword ptr [rbp + 0x200]
140912749: movups xmm1, xmmword ptr [rbp + 0x210]
140912750: movups xmm2, xmmword ptr [rbp + 0x220]
140912757: movups xmm3, xmmword ptr [rbp + 0x230]
14091275e: movaps xmmword ptr [rbp + 0x2e0], xmm0
140912765: movaps xmmword ptr [rbp + 0x2f0], xmm1
14091276c: movaps xmmword ptr [rbp + 0x300], xmm2
140912773: movaps xmmword ptr [rbp + 0x310], xmm3
14091277a: movups xmm0, xmmword ptr [rbp + 0x240]
140912781: movaps xmmword ptr [rbp + 0x320], xmm0
140912788: movups xmm0, xmmword ptr [rbp + 0x250]
14091278f: movaps xmmword ptr [rbp + 0x330], xmm0
140912796: movups xmm0, xmmword ptr [rbp + 0x260]
14091279d: movaps xmmword ptr [rbp + 0x340], xmm0
1409127a4: movups xmm0, xmmword ptr [rbp + 0x70]
1409127a8: movaps xmmword ptr [rbp + 0xe0], xmm0
1409127af: movups xmm0, xmmword ptr [rbp + 0x60]
1409127b3: movaps xmmword ptr [rbp + 0xd0], xmm0
1409127ba: movups xmm0, xmmword ptr [rbp + 0x50]
1409127be: movaps xmmword ptr [rbp + 0xc0], xmm0
1409127c5: movups xmm0, xmmword ptr [rbp + 0x10]
1409127c9: movups xmm1, xmmword ptr [rbp + 0x20]
1409127cd: movups xmm2, xmmword ptr [rbp + 0x30]
1409127d1: movups xmm3, xmmword ptr [rbp + 0x40]
1409127d5: movaps xmmword ptr [rbp + 0xb0], xmm3
1409127dc: movaps xmmword ptr [rbp + 0xa0], xmm2
1409127e3: movaps xmmword ptr [rbp + 0x90], xmm1
1409127ea: movaps xmmword ptr [rbp + 0x80], xmm0
1409127f1: lea rcx, [rbp + 0xf0]
1409127f8: lea rdx, [rbp + 0x270]
1409127ff: mov r8d, 0xe0
140912805: call 0x142923c00
14091280a: lea rcx, [rbp + 0x270]
140912811: lea rdx, [rbp + 0x80]
140912818: call 0x141450ae0
14091281d: mov byte ptr [rbp + 0x200], 9
140912824: mov rcx, qword ptr [rip + 0x2d19b45]
14091282b: lea r8, [rip + 0x234d52e]
140912832: lea rdx, [rbp + 0x200]
140912839: call 0x140b218a0
14091283e: mov r8, rdx
140912841: lea rcx, [rbp + 0x270]
140912848: mov rdx, rax
14091284b: call 0x141453000
140912850: movups xmm0, xmmword ptr [rbp + 0x2d0]
140912857: movaps xmmword ptr [rbp + 0xe0], xmm0
14091285e: movups xmm0, xmmword ptr [rbp + 0x2c0]
140912865: movaps xmmword ptr [rbp + 0xd0], xmm0
14091286c: movups xmm0, xmmword ptr [rbp + 0x2b0]
140912873: movaps xmmword ptr [rbp + 0xc0], xmm0
14091287a: movups xmm0, xmmword ptr [rbp + 0x270]
140912881: movups xmm1, xmmword ptr [rbp + 0x280]
140912888: movups xmm2, xmmword ptr [rbp + 0x290]
14091288f: movups xmm3, xmmword ptr [rbp + 0x2a0]
140912896: movaps xmmword ptr [rbp + 0xb0], xmm3
14091289d: movaps xmmword ptr [rbp + 0xa0], xmm2
1409128a4: movaps xmmword ptr [rbp + 0x90], xmm1
1409128ab: movaps xmmword ptr [rbp + 0x80], xmm0
1409128b2: lea rdx, [rbp + 0x80]
1409128b9: mov rcx, rsi
1409128bc: call 0x140b67280
1409128c1: mov qword ptr [rbp + 0x80], 0
1409128cc: mov qword ptr [rbp + 0x88], 4
1409128d7: mov qword ptr [rbp + 0x90], 0
1409128e2: mov qword ptr [rbp + 0xa0], 0
1409128ed: mov dword ptr [rbp + 0x98], 0
1409128f7: lea rdx, [rbp + 0x80]
1409128fe: mov rcx, rsi
140912901: call 0x141545d70
140912906: mov rsi, rax
140912909: lea rcx, [rax + 0x38]
14091290d: movabs rdx, 0x80c70da6101ec1a
140912917: movabs r8, 0x668ae4faf3605349
140912921: call 0x1400dd6f0
140912926: test rax, rax
140912929: je 0x140912969
14091292b: cmp qword ptr [rsi + 0x218], rdx
140912932: jbe 0x140912969
140912934: mov rax, qword ptr [rsi + 0x210]
14091293b: mov rax, qword ptr [rax + rdx*8]
14091293f: test rax, rax
140912942: je 0x140912969
140912944: mov rcx, qword ptr [rsi + 0x1e0]
14091294b: not rax
14091294e: lea rax, [rax + rax*2]
140912952: shl rax, 5
140912956: cmp qword ptr [rcx + rax + 0x20], 0
14091295c: je 0x140912969
14091295e: add rsp, 0x3e0
140912965: pop rdi
140912966: pop rsi
140912967: pop rbp
140912968: ret 
140912969: mov rcx, rsi
14091296c: add rsp, 0x3e0
140912973: pop rdi
140912974: pop rsi
140912975: pop rbp
140912976: jmp 0x14151f980
14091297b: nop dword ptr [rax + rax]
140912980: mov qword ptr [rsp + 0x10], rdx
140912985: push rbp
140912986: push rsi
140912987: push rdi
140912988: sub rsp, 0x20
14091298c: lea rbp, [rdx + 0x80]
140912993: lea rcx, [rbp + 0x270]
14091299a: call 0x140029880
14091299f: nop 
1409129a0: add rsp, 0x20
1409129a4: pop rdi
1409129a5: pop rsi
1409129a6: pop rbp
1409129a7: ret 
1409129a8: nop dword ptr [rax + rax]
1409129b0: mov qword ptr [rsp + 0x10], rdx
1409129b5: push rbp
1409129b6: push rsi
1409129b7: push rdi
1409129b8: sub rsp, 0x20
1409129bc: lea rbp, [rdx + 0x80]
1409129c3: lea rcx, [rbp + 0x10]
1409129c7: call 0x140029880
1409129cc: nop 
1409129cd: add rsp, 0x20
1409129d1: pop rdi
1409129d2: pop rsi
1409129d3: pop rbp
1409129d4: ret 
1409129d5: nop word ptr cs:[rax + rax]
1409129e0: mov qword ptr [rsp + 0x10], rdx
1409129e5: push rbp
1409129e6: push rsi
1409129e7: push rdi
1409129e8: sub rsp, 0x20
1409129ec: lea rbp, [rdx + 0x80]
1409129f3: lea rcx, [rbp - 0x60]
1409129f7: call 0x140029880
1409129fc: nop 
1409129fd: add rsp, 0x20
140912a01: pop rdi
140912a02: pop rsi
140912a03: pop rbp
140912a04: ret 
140912a05: nop word ptr cs:[rax + rax]
140912a10: mov qword ptr [rsp + 0x10], rdx
140912a15: push rbp
140912a16: push rsi
140912a17: push rdi
140912a18: sub rsp, 0x20
140912a1c: lea rbp, [rdx + 0x80]
140912a23: lea rcx, [rbp + 0x270]
140912a2a: call 0x140029880
140912a2f: nop 
140912a30: add rsp, 0x20
140912a34: pop rdi
140912a35: pop rsi
140912a36: pop rbp
140912a37: ret 
