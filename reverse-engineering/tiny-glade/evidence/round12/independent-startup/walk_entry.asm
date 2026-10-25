14090e510: 56 push rsi
14090e511: 57 push rdi
14090e512: 53 push rbx
14090e513: 4881eca0000000 sub rsp, 0xa0
14090e51a: 0f29bc2490000000 movaps xmmword ptr [rsp + 0x90], xmm7
14090e522: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
14090e52a: 4889d7 mov rdi, rdx
14090e52d: 4889ce mov rsi, rcx
14090e530: f20f100558ce1902 movsd xmm0, qword ptr [rip + 0x219ce58]
14090e538: f20f11442438 movsd qword ptr [rsp + 0x38], xmm0
14090e53e: f30f100d5ace1902 movss xmm1, dword ptr [rip + 0x219ce5a]
14090e546: f30f101556ce1902 movss xmm2, dword ptr [rip + 0x219ce56]
14090e54e: 4c89c1 mov rcx, r8
14090e551: e8eab00d00 call 0x1409e9640
14090e556: 488d4c2450 lea rcx, [rsp + 0x50]
14090e55b: 488d542438 lea rdx, [rsp + 0x38]
14090e560: 0f28d0 movaps xmm2, xmm0
14090e563: e878e83800 call 0x140c9cde0
14090e568: 8b5c2450 mov ebx, dword ptr [rsp + 0x50]
14090e56c: f30f10742454 movss xmm6, dword ptr [rsp + 0x54]
14090e572: f30f107c2458 movss xmm7, dword ptr [rsp + 0x58]
14090e578: f20f10442450 movsd xmm0, qword ptr [rsp + 0x50]
14090e57e: 0f57c9 xorps xmm1, xmm1
14090e581: f30f58cf addss xmm1, xmm7
14090e585: 0f580524ce1902 addps xmm0, xmmword ptr [rip + 0x219ce24]
14090e58c: 0f13442438 movlps qword ptr [rsp + 0x38], xmm0
14090e591: f30f114c2440 movss dword ptr [rsp + 0x40], xmm1
14090e597: f20f100561f70c02 movsd xmm0, qword ptr [rip + 0x20cf761]
14090e59f: f20f11442444 movsd qword ptr [rsp + 0x44], xmm0
14090e5a5: c744244c00000080 mov dword ptr [rsp + 0x4c], 0x80000000
14090e5ad: c7442430ffff0100 mov dword ptr [rsp + 0x30], 0x1ffff
14090e5b5: c7442428ffff7f7f mov dword ptr [rsp + 0x28], 0x7f7fffff
14090e5bd: 488d4c2450 lea rcx, [rsp + 0x50]
14090e5c2: 4c8d442438 lea r8, [rsp + 0x38]
14090e5c7: 4889fa mov rdx, rdi
14090e5ca: 4531c9 xor r9d, r9d
14090e5cd: e8aefbfbff call 0x1408ce180
14090e5d2: 807c245011 cmp byte ptr [rsp + 0x50], 0x11
14090e5d7: 7410 je 0x14090e5e9
14090e5d9: 8b5c2468 mov ebx, dword ptr [rsp + 0x68]
14090e5dd: f30f1074246c movss xmm6, dword ptr [rsp + 0x6c]
14090e5e3: f30f107c2470 movss xmm7, dword ptr [rsp + 0x70]
14090e5e9: f30f583507ff1702 addss xmm6, dword ptr [rip + 0x217ff07]
14090e5f1: f30f5f3577730102 maxss xmm6, dword ptr [rip + 0x2017377]
14090e5f9: 895c2438 mov dword ptr [rsp + 0x38], ebx
14090e5fd: f30f1174243c movss dword ptr [rsp + 0x3c], xmm6
14090e603: f30f117c2440 movss dword ptr [rsp + 0x40], xmm7
14090e609: f20f1005afcd1902 movsd xmm0, qword ptr [rip + 0x219cdaf]
14090e611: f20f11442478 movsd qword ptr [rsp + 0x78], xmm0
14090e617: 488d4c2450 lea rcx, [rsp + 0x50]
14090e61c: 488d542478 lea rdx, [rsp + 0x78]
14090e621: e89ae73800 call 0x140c9cdc0
14090e626: f20f10442450 movsd xmm0, qword ptr [rsp + 0x50]
14090e62c: f30f104c2458 movss xmm1, dword ptr [rsp + 0x58]
14090e632: 0f28d0 movaps xmm2, xmm0
14090e635: 0f59d0 mulps xmm2, xmm0
14090e638: f30f16da movshdup xmm3, xmm2
14090e63c: f30f58da addss xmm3, xmm2
14090e640: 0f28d1 movaps xmm2, xmm1
14090e643: f30f59d1 mulss xmm2, xmm1
14090e647: f30f58d3 addss xmm2, xmm3
14090e64b: 0f57db xorps xmm3, xmm3
14090e64e: f30f51da sqrtss xmm3, xmm2
14090e652: f30f1015aa720102 movss xmm2, dword ptr [rip + 0x20172aa]
14090e65a: f30f5ed3 divss xmm2, xmm3
14090e65e: 660f7ed0 movd eax, xmm2
14090e662: 85c0 test eax, eax
14090e664: 0f99c1 setns cl
14090e667: 89c2 mov edx, eax
14090e669: 81e2ffffff7f and edx, 0x7fffffff
14090e66f: 81c2000080ff add edx, 0xff800000
14090e675: 81fa0000007f cmp edx, 0x7f000000
14090e67b: 0f92c2 setb dl
14090e67e: 20ca and dl, cl
14090e680: ffc8 dec eax
14090e682: 3dffff7f00 cmp eax, 0x7fffff
14090e687: 0f92c0 setb al
14090e68a: 08d0 or al, dl
14090e68c: 740d je 0x14090e69b
14090e68e: f30f12da movsldup xmm3, xmm2
14090e692: 0f59c3 mulps xmm0, xmm3
14090e695: f30f59ca mulss xmm1, xmm2
14090e699: eb0f jmp 0x14090e6aa
14090e69b: 0f28058eec0b02 movaps xmm0, xmmword ptr [rip + 0x20bec8e]
14090e6a2: f30f100d3aba0402 movss xmm1, dword ptr [rip + 0x204ba3a]
14090e6aa: 0f13442450 movlps qword ptr [rsp + 0x50], xmm0
14090e6af: f30f114c2458 movss dword ptr [rsp + 0x58], xmm1
14090e6b5: 4881c6f0020000 add rsi, 0x2f0
14090e6bc: 488d542438 lea rdx, [rsp + 0x38]
14090e6c1: 4c8d442450 lea r8, [rsp + 0x50]
14090e6c6: 4889f1 mov rcx, rsi
14090e6c9: e8625a0800 call 0x140994130
14090e6ce: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
14090e6d6: 0f28bc2490000000 movaps xmm7, xmmword ptr [rsp + 0x90]
14090e6de: 4881c4a0000000 add rsp, 0xa0
14090e6e5: 5b pop rbx
14090e6e6: 5f pop rdi
14090e6e7: 5e pop rsi
14090e6e8: c3 ret 
