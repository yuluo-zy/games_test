14261f470: sub rsp, 0x18
14261f474: movaps xmmword ptr [rsp], xmm6
14261f478: mov rax, rcx
14261f47b: movsd xmm0, qword ptr [rdx]
14261f47f: movsd xmm1, qword ptr [rdx + 8]
14261f484: subps xmm1, xmm0
14261f487: movaps xmm3, xmm1
14261f48a: shufps xmm3, xmm1, 0xe1
14261f48e: movsd xmm2, qword ptr [r8]
14261f493: movsd xmm4, qword ptr [r8 + 8]
14261f499: subps xmm4, xmm2
14261f49c: movaps xmm6, xmm3
14261f49f: mulps xmm6, xmm4
14261f4a2: movshdup xmm5, xmm6
14261f4a6: subps xmm5, xmm6
14261f4a9: subps xmm2, xmm0
14261f4ac: xorps xmm6, xmm6
14261f4af: ucomiss xmm5, xmm6
14261f4b2: jne 0x14261f4cf
14261f4b4: jp 0x14261f4cf
14261f4b6: mulps xmm2, xmm3
14261f4b9: movshdup xmm0, xmm2
14261f4bd: subss xmm2, xmm0
14261f4c1: ucomiss xmm2, xmm6
14261f4c4: jne 0x14261f52e
14261f4c6: jp 0x14261f52e
14261f4c8: mov ecx, 2
14261f4cd: jmp 0x14261f533
14261f4cf: movsldup xmm5, xmm5
14261f4d3: divps xmm4, xmm5
14261f4d6: movaps xmm6, xmm2
14261f4d9: shufps xmm6, xmm2, 0xe1
14261f4dd: mulps xmm6, xmm4
14261f4e0: movshdup xmm4, xmm6
14261f4e4: subps xmm4, xmm6
14261f4e7: mov ecx, 1
14261f4ec: ucomiss xmm4, dword ptr [rdx + 0x10]
14261f4f0: jb 0x14261f533
14261f4f2: divps xmm3, xmm5
14261f4f5: mulps xmm2, xmm3
14261f4f8: movshdup xmm3, xmm2
14261f4fc: subss xmm2, xmm3
14261f500: ucomiss xmm2, dword ptr [r8 + 0x10]
14261f505: jb 0x14261f533
14261f507: movss xmm3, dword ptr [rdx + 0x14]
14261f50c: ucomiss xmm3, xmm4
14261f50f: jb 0x14261f533
14261f511: movss xmm3, dword ptr [r8 + 0x14]
14261f517: ucomiss xmm3, xmm2
14261f51a: jb 0x14261f533
14261f51c: movsldup xmm2, xmm4
14261f520: mulps xmm1, xmm2
14261f523: addps xmm0, xmm1
14261f526: movlps qword ptr [rax + 4], xmm0
14261f52a: xor ecx, ecx
14261f52c: jmp 0x14261f533
14261f52e: mov ecx, 3
14261f533: mov dword ptr [rax], ecx
14261f535: movaps xmm6, xmmword ptr [rsp]
14261f539: add rsp, 0x18
14261f53d: ret 
