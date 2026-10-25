1417d11d0: push r15
1417d11d2: push r14
1417d11d4: push rsi
1417d11d5: push rdi
1417d11d6: push rbx
1417d11d7: sub rsp, 0xb0
1417d11de: movaps xmmword ptr [rsp + 0xa0], xmm13
1417d11e7: movaps xmmword ptr [rsp + 0x90], xmm12
1417d11f0: movaps xmmword ptr [rsp + 0x80], xmm11
1417d11f9: movaps xmmword ptr [rsp + 0x70], xmm10
1417d11ff: movaps xmmword ptr [rsp + 0x60], xmm9
1417d1205: movaps xmmword ptr [rsp + 0x50], xmm8
1417d120b: movaps xmmword ptr [rsp + 0x40], xmm7
1417d1210: movaps xmmword ptr [rsp + 0x30], xmm6
1417d1215: mov rsi, rcx
1417d1218: mov rax, qword ptr [rdx]
1417d121b: mov rdi, qword ptr [rax]
1417d121e: mov rbx, qword ptr [rax + 8]
1417d1222: mov r14, qword ptr [r8]
1417d1225: mov eax, dword ptr [r14 + 0x54]
1417d1229: mov qword ptr [rsp + 0x28], rax
1417d122e: lea rcx, [rsp + 0x28]
1417d1233: call 0x140caf380
1417d1238: movss xmm1, dword ptr [rdi]
1417d123c: xor eax, eax
1417d123e: ucomiss xmm1, xmm0
1417d1241: jbe 0x1417d13d4
1417d1247: movsd xmm7, qword ptr [r14 + 0x20]
1417d124d: movss xmm8, dword ptr [r14 + 0x28]
1417d1253: movss xmm9, dword ptr [r14 + 0x44]
1417d1259: mulss xmm9, dword ptr [r14 + 0x48]
1417d125f: movd xmm0, dword ptr [r14 + 0x4c]
1417d1265: movd xmm1, dword ptr [rip + 0x115466b]
1417d126d: call 0x142923f70
1417d1272: movdqa xmm6, xmm0
1417d1276: movsd xmm10, qword ptr [r14 + 0x2c]
1417d127c: movss xmm11, dword ptr [r14 + 0x34]
1417d1282: mov edi, dword ptr [r14 + 0x54]
1417d1286: movsd xmm12, qword ptr [r14 + 0x38]
1417d128c: movss xmm13, dword ptr [r14 + 0x40]
1417d1292: mov r15, qword ptr [rbx]
1417d1295: mov rbx, qword ptr [r14 + 0x18]
1417d1299: mov qword ptr [rsp + 0x28], rbx
1417d129e: cmp qword ptr [r15 + 0x18], 0
1417d12a3: je 0x1417d1346
1417d12a9: lea rcx, [r15 + 0x20]
1417d12ad: lea rdx, [rsp + 0x28]
1417d12b2: call 0x1405b0d70
1417d12b7: mov rcx, qword ptr [r15]
1417d12ba: mov rdx, qword ptr [r15 + 8]
1417d12be: mov r8, rdx
1417d12c1: and r8, rax
1417d12c4: shr rax, 0x39
1417d12c8: movd xmm0, eax
1417d12cc: punpcklbw xmm0, xmm0
1417d12d0: pshuflw xmm0, xmm0, 0
1417d12d5: pshufd xmm0, xmm0, 0
1417d12da: lea rax, [rcx - 0x2a8]
1417d12e1: xor r9d, r9d
1417d12e4: pcmpeqd xmm1, xmm1
1417d12e8: movdqu xmm2, xmmword ptr [rcx + r8]
1417d12ee: movdqa xmm3, xmm2
1417d12f2: pcmpeqb xmm3, xmm0
1417d12f6: pmovmskb r10d, xmm3
1417d12fb: test r10d, r10d
1417d12fe: je 0x1417d1328
1417d1300: tzcnt r11d, r10d
1417d1305: add r11, r8
1417d1308: and r11, rdx
1417d130b: neg r11
1417d130e: imul r11, r11, 0x2a8
1417d1315: cmp qword ptr [rax + r11], rbx
1417d1319: je 0x1417d1350
1417d131b: lea r11d, [r10 - 1]
1417d131f: and r11w, r10w
1417d1323: mov r10d, r11d
1417d1326: jne 0x1417d1300
1417d1328: pcmpeqb xmm2, xmm1
1417d132c: pmovmskb r10d, xmm2
1417d1331: test r10d, r10d
1417d1334: jne 0x1417d1346
1417d1336: add r8, r9
1417d1339: add r8, 0x10
1417d133d: add r9, 0x10
1417d1341: and r8, rdx
1417d1344: jmp 0x1417d12e8
1417d1346: movd xmm0, dword ptr [rip + 0x12b00c2]
1417d134e: jmp 0x1417d139c
1417d1350: cmp qword ptr [rcx + r11 - 0x290], 2
1417d1359: jb 0x1417d141f
1417d135f: movss xmm0, dword ptr [rcx + r11 - 0x270]
1417d1369: pxor xmm1, xmm1
1417d136d: ucomiss xmm0, xmm1
1417d1370: jbe 0x1417d141f
1417d1376: add rcx, r11
1417d1379: cmp byte ptr [rcx - 0xc], 0
1417d137d: je 0x1417d1397
1417d137f: movss xmm0, dword ptr [rcx - 0x28]
1417d1384: subss xmm0, dword ptr [rcx - 0x2c]
1417d1389: andps xmm0, xmmword ptr [rip + 0x115d5f0]
1417d1390: addss xmm0, dword ptr [rcx - 0x18]
1417d1395: jmp 0x1417d139c
1417d1397: movss xmm0, dword ptr [rcx - 0x18]
1417d139c: mulss xmm9, xmm6
1417d13a1: movlps qword ptr [rsi + 4], xmm7
1417d13a5: movss dword ptr [rsi + 0xc], xmm8
1417d13ab: movss dword ptr [rsi + 0x10], xmm9
1417d13b1: movlps qword ptr [rsi + 0x14], xmm10
1417d13b6: movss dword ptr [rsi + 0x1c], xmm11
1417d13bc: mov dword ptr [rsi + 0x20], edi
1417d13bf: movlps qword ptr [rsi + 0x24], xmm12
1417d13c4: movss dword ptr [rsi + 0x2c], xmm13
1417d13ca: movd dword ptr [rsi + 0x30], xmm0
1417d13cf: mov eax, 1
1417d13d4: mov dword ptr [rsi], eax
1417d13d6: mov rax, rsi
1417d13d9: movaps xmm6, xmmword ptr [rsp + 0x30]
1417d13de: movaps xmm7, xmmword ptr [rsp + 0x40]
1417d13e3: movaps xmm8, xmmword ptr [rsp + 0x50]
1417d13e9: movaps xmm9, xmmword ptr [rsp + 0x60]
1417d13ef: movaps xmm10, xmmword ptr [rsp + 0x70]
1417d13f5: movaps xmm11, xmmword ptr [rsp + 0x80]
1417d13fe: movaps xmm12, xmmword ptr [rsp + 0x90]
1417d1407: movaps xmm13, xmmword ptr [rsp + 0xa0]
1417d1410: add rsp, 0xb0
1417d1417: pop rbx
1417d1418: pop rdi
1417d1419: pop rsi
1417d141a: pop r14
1417d141c: pop r15
1417d141e: ret 
1417d141f: lea rcx, [rip + 0x1308d6d]
1417d1426: lea r8, [rip + 0x151372b]
1417d142d: mov edx, 0x32
1417d1432: call 0x1428d9430
1417d1437: int3 
