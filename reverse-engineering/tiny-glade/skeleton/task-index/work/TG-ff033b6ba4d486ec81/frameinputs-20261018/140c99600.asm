140c99600: sub rsp, 0x48
140c99604: cmp rdx, 5
140c99608: jae 0x140c9969c
140c9960e: movsd xmm0, qword ptr [rcx + 0x10]
140c99613: movshdup xmm1, xmm0
140c99617: mulss xmm1, xmm0
140c9961b: xorps xmm2, xmm2
140c9961e: ucomiss xmm1, xmm2
140c99621: jbe 0x140c996b4
140c99627: movsd xmm1, qword ptr [rcx]
140c9962b: movshdup xmm2, xmm1
140c9962f: xorps xmm2, xmmword ptr [rip + 0x1c8c2ba]
140c99636: shufps xmm0, xmm0, 0x11
140c9963a: movaps xmm3, xmm1
140c9963d: movlhps xmm3, xmm2
140c99640: shufps xmm1, xmm3, 0x24
140c99644: xor eax, eax
140c99646: cmp rdx, 4
140c9964a: cmovne rax, rdx
140c9964e: mulps xmm0, xmmword ptr [rip + 0x1e0c70b]
140c99655: movshdup xmm2, xmm0
140c99659: movss dword ptr [rsp + 0x28], xmm2
140c9965f: movups xmmword ptr [rsp + 0x2c], xmm0
140c99664: movhps qword ptr [rsp + 0x3c], xmm0
140c99669: movss dword ptr [rsp + 0x44], xmm0
140c9966f: movsd xmm0, qword ptr [rsp + rax*8 + 0x28]
140c99675: unpcklps xmm0, xmm0
140c99678: mulps xmm0, xmm1
140c9967b: movaps xmm2, xmm0
140c9967e: unpckhpd xmm2, xmm0
140c99682: addps xmm2, xmm0
140c99685: movshdup xmm1, xmm2
140c99689: movss xmm0, dword ptr [rcx + 8]
140c9968e: addss xmm0, xmm2
140c99692: addss xmm1, dword ptr [rcx + 0xc]
140c99697: add rsp, 0x48
140c9969b: ret 
140c9969c: lea rcx, [rip + 0x1e9511d]
140c996a3: lea r8, [rip + 0x1e9512e]
140c996aa: mov edx, 0x18
140c996af: call 0x1428d9430
140c996b4: lea rcx, [rip + 0x1e94fb1]
140c996bb: lea r8, [rip + 0x1e94ff6]
140c996c2: mov edx, 0x23
140c996c7: call 0x1428d9430
140c996cc: int3 
