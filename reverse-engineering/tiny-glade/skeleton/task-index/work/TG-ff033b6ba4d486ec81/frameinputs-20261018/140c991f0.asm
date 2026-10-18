140c991f0: push rbp
140c991f1: push rsi
140c991f2: sub rsp, 0x108
140c991f9: lea rbp, [rsp + 0x80]
140c99201: movaps xmmword ptr [rbp + 0x70], xmm11
140c99206: movaps xmmword ptr [rbp + 0x60], xmm10
140c9920b: movaps xmmword ptr [rbp + 0x50], xmm9
140c99210: movaps xmmword ptr [rbp + 0x40], xmm8
140c99215: movaps xmmword ptr [rbp + 0x30], xmm7
140c99219: movaps xmmword ptr [rbp + 0x20], xmm6
140c9921d: mov qword ptr [rbp + 0x18], 0xfffffffffffffffe
140c99225: movss xmm8, dword ptr [rdx + 0x10]
140c9922b: movss xmm10, dword ptr [rdx + 0x14]
140c99231: movaps xmm0, xmm8
140c99235: mulss xmm0, xmm10
140c9923a: xorps xmm1, xmm1
140c9923d: ucomiss xmm0, xmm1
140c99240: jbe 0x140c99433
140c99246: movaps xmm6, xmm2
140c99249: mov rsi, rcx
140c9924c: movsd xmm9, qword ptr [rdx]
140c99251: movsd xmm7, qword ptr [rdx + 8]
140c99256: movzx eax, byte ptr [rip + 0x299b9ac]
140c9925d: mov ecx, 0x30
140c99262: mov edx, 4
140c99267: call 0x140613c10
140c9926c: test rax, rax
140c9926f: je 0x140c9944b
140c99275: movss xmm2, dword ptr [rip + 0x1c8c5f3]
140c9927d: movaps xmm5, xmm8
140c99281: mulss xmm5, xmm2
140c99285: movss xmm4, dword ptr [rip + 0x1cc0e5b]
140c9928d: movaps xmm3, xmm10
140c99291: mulss xmm3, xmm4
140c99295: movaps xmm1, xmm5
140c99298: movshdup xmm0, xmm9
140c9929d: xorps xmm0, xmmword ptr [rip + 0x1c8c64c]
140c992a4: shufps xmm1, xmm3, 0
140c992a8: movaps xmm11, xmm9
140c992ac: movlhps xmm11, xmm0
140c992b0: shufps xmm9, xmm11, 0x24
140c992b5: mulps xmm1, xmm9
140c992b9: movaps xmm0, xmm1
140c992bc: unpckhpd xmm0, xmm1
140c992c0: addps xmm0, xmm1
140c992c3: movshdup xmm11, xmm0
140c992c8: movshdup xmm1, xmm7
140c992cc: addss xmm1, xmm11
140c992d1: mulss xmm10, xmm2
140c992d6: shufps xmm5, xmm10, 0
140c992db: mulps xmm5, xmm9
140c992df: movaps xmm2, xmm5
140c992e2: unpckhpd xmm2, xmm5
140c992e6: addps xmm2, xmm5
140c992e9: mulss xmm8, xmm4
140c992ee: movaps xmm5, xmm8
140c992f2: shufps xmm5, xmm10, 0
140c992f7: mulps xmm5, xmm9
140c992fb: movaps xmm4, xmm5
140c992fe: unpckhpd xmm4, xmm5
140c99302: addps xmm4, xmm5
140c99305: shufps xmm8, xmm3, 0
140c9930a: mulps xmm8, xmm9
140c9930e: movaps xmm3, xmm8
140c99312: unpckhpd xmm3, xmm8
140c99317: addps xmm3, xmm8
140c9931b: movaps xmm5, xmm7
140c9931e: addss xmm5, xmm3
140c99322: movss dword ptr [rax], xmm5
140c99326: movss dword ptr [rax + 4], xmm6
140c9932b: shufps xmm0, xmm2, 0x10
140c9932f: shufps xmm0, xmm2, 0xe2
140c99333: shufps xmm2, xmm4, 0x10
140c99337: shufps xmm2, xmm4, 0xe2
140c9933b: shufps xmm4, xmm3, 0x10
140c9933f: shufps xmm4, xmm3, 0xe2
140c99343: shufps xmm7, xmm7, 0xe1
140c99347: addps xmm4, xmm7
140c9934a: movlps qword ptr [rax + 8], xmm4
140c9934e: movss dword ptr [rax + 0x10], xmm6
140c99353: addps xmm2, xmm7
140c99356: movlps qword ptr [rax + 0x14], xmm2
140c9935a: movss dword ptr [rax + 0x1c], xmm6
140c9935f: addps xmm0, xmm7
140c99362: movlps qword ptr [rax + 0x20], xmm0
140c99366: movss dword ptr [rax + 0x28], xmm6
140c9936b: movss dword ptr [rax + 0x2c], xmm1
140c99370: mov qword ptr [rbp - 8], 4
140c99378: mov qword ptr [rbp], rax
140c9937c: mov qword ptr [rbp + 8], 4
140c99384: mov ecx, dword ptr [rax + 8]
140c99387: mov dword ptr [rbp - 0x50], ecx
140c9938a: mov rax, qword ptr [rax]
140c9938d: mov qword ptr [rbp - 0x58], rax
140c99391: mov byte ptr [rbp + 0x17], 1
140c99395: lea rdx, [rip + 0x1e953f4]
140c9939c: lea rcx, [rbp - 8]
140c993a0: call 0x140562cf0
140c993a5: mov rax, qword ptr [rbp]
140c993a9: mov ecx, dword ptr [rbp - 0x50]
140c993ac: mov dword ptr [rax + 0x38], ecx
140c993af: mov rcx, qword ptr [rbp - 0x58]
140c993b3: mov qword ptr [rax + 0x30], rcx
140c993b7: mov qword ptr [rbp + 8], 5
140c993bf: movups xmm0, xmmword ptr [rbp - 8]
140c993c3: movaps xmmword ptr [rbp - 0x20], xmm0
140c993c7: mov qword ptr [rbp - 0x10], 5
140c993cf: mov byte ptr [rbp + 0x17], 0
140c993d3: lea rcx, [rbp - 0x58]
140c993d7: lea rdx, [rbp - 0x20]
140c993db: xor r8d, r8d
140c993de: call 0x141940470
140c993e3: xor eax, eax
140c993e5: cmp rax, qword ptr [rbp - 0x58]
140c993e9: jo 0x140c9945a
140c993eb: mov rax, qword ptr [rbp - 0x28]
140c993ef: mov qword ptr [rsi + 0x30], rax
140c993f3: movups xmm0, xmmword ptr [rbp - 0x58]
140c993f7: movups xmm1, xmmword ptr [rbp - 0x48]
140c993fb: movups xmm2, xmmword ptr [rbp - 0x38]
140c993ff: movups xmmword ptr [rsi + 0x20], xmm2
140c99403: movups xmmword ptr [rsi + 0x10], xmm1
140c99407: movups xmmword ptr [rsi], xmm0
140c9940a: mov rax, rsi
140c9940d: movaps xmm6, xmmword ptr [rbp + 0x20]
140c99411: movaps xmm7, xmmword ptr [rbp + 0x30]
140c99415: movaps xmm8, xmmword ptr [rbp + 0x40]
140c9941a: movaps xmm9, xmmword ptr [rbp + 0x50]
140c9941f: movaps xmm10, xmmword ptr [rbp + 0x60]
140c99424: movaps xmm11, xmmword ptr [rbp + 0x70]
140c99429: add rsp, 0x108
140c99430: pop rsi
140c99431: pop rbp
140c99432: ret 
140c99433: lea rcx, [rip + 0x1e95232]
140c9943a: lea r8, [rip + 0x1e95337]
140c99441: mov edx, 0x23
140c99446: call 0x1428d9430
140c9944b: mov ecx, 4
140c99450: mov edx, 0x30
140c99455: call 0x1428d9000
140c9945a: movups xmm0, xmmword ptr [rbp - 0x50]
140c9945e: movaps xmmword ptr [rbp - 0x20], xmm0
140c99462: mov byte ptr [rbp + 0x17], 0
140c99466: lea rax, [rip + 0x1e9533b]
140c9946d: mov qword ptr [rsp + 0x20], rax
140c99472: lea rcx, [rip + 0x21d8037]
140c99479: lea r9, [rip + 0x21d8010]
140c99480: lea r8, [rbp - 0x20]
140c99484: mov edx, 0x2b
140c99489: call 0x1428d9760
140c9948e: ud2 
140c99490: mov qword ptr [rsp + 0x10], rdx
140c99495: push rbp
140c99496: push rsi
140c99497: sub rsp, 0x88
140c9949e: lea rbp, [rdx + 0x80]
140c994a5: movaps xmmword ptr [rsp + 0x20], xmm11
140c994ab: movaps xmmword ptr [rsp + 0x30], xmm10
140c994b1: movaps xmmword ptr [rsp + 0x40], xmm9
140c994b7: movaps xmmword ptr [rsp + 0x50], xmm8
140c994bd: movaps xmmword ptr [rsp + 0x60], xmm7
140c994c2: movaps xmmword ptr [rsp + 0x70], xmm6
140c994c7: cmp byte ptr [rbp + 0x17], 0
140c994cb: je 0x140c994ed
140c994cd: mov rax, qword ptr [rbp - 8]
140c994d1: test rax, rax
140c994d4: je 0x140c994ed
140c994d6: mov rcx, qword ptr [rbp]
140c994da: shl rax, 2
140c994de: lea rdx, [rax + rax*2]
140c994e2: mov r8d, 4
140c994e8: call 0x140613c20
140c994ed: movaps xmm6, xmmword ptr [rsp + 0x70]
140c994f2: movaps xmm7, xmmword ptr [rsp + 0x60]
140c994f7: movaps xmm8, xmmword ptr [rsp + 0x50]
140c994fd: movaps xmm9, xmmword ptr [rsp + 0x40]
140c99503: movaps xmm10, xmmword ptr [rsp + 0x30]
140c99509: movaps xmm11, xmmword ptr [rsp + 0x20]
140c9950f: add rsp, 0x88
140c99516: pop rsi
140c99517: pop rbp
140c99518: ret 
