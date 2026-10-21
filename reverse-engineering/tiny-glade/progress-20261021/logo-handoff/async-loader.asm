0x1400d7440: push rbp
0x1400d7441: push r15
0x1400d7443: push r14
0x1400d7445: push r12
0x1400d7447: push rsi
0x1400d7448: push rdi
0x1400d7449: push rbx
0x1400d744a: mov eax, 0x1060
0x1400d744f: call 0x14289aa40
0x1400d7454: sub rsp, rax
0x1400d7457: lea rbp, [rsp + 0x80]
0x1400d745f: movdqa xmmword ptr [rbp + 0xfd0], xmm7
0x1400d7467: movdqa xmmword ptr [rbp + 0xfc0], xmm6
0x1400d746f: mov qword ptr [rbp + 0xfb8], 0xfffffffffffffffe
0x1400d747a: lea rax, [rcx + 0x638]
0x1400d7481: mov qword ptr [rbp + 0xfa0], rax
0x1400d7488: mov qword ptr [rbp + 0xfa8], rcx
0x1400d748f: mov rdx, qword ptr [rcx + 0x638]
0x1400d7496: add rdx, 0x10
0x1400d749a: mov byte ptr [rbp + 0xfb5], 1
0x1400d74a1: lea rcx, [rbp + 0xed0]
0x1400d74a8: call 0x140bea2d0
0x1400d74ad: lea rcx, [rbp + 0x270]
0x1400d74b4: call 0x140be57d0
0x1400d74b9: lea rcx, [rbp + 0x8a0]
0x1400d74c0: call 0x140be57d0
0x1400d74c5: lea rcx, [rbp - 0x40]
0x1400d74c9: lea rsi, [rbp + 0xed0]
0x1400d74d0: mov r8d, 0xd0
0x1400d74d6: mov rdx, rsi
0x1400d74d9: call 0x142923c00
0x1400d74de: lea rcx, [rbp + 0x90]
0x1400d74e5: lea rdx, [rbp + 0x270]
0x1400d74ec: mov r8d, 0xf0
0x1400d74f2: call 0x142923c00
0x1400d74f7: lea rcx, [rbp + 0x180]
0x1400d74fe: lea rdx, [rbp + 0x8a0]
0x1400d7505: mov r8d, 0xf0
0x1400d750b: call 0x142923c00
0x1400d7510: call 0x140d4d740
0x1400d7515: mov rdi, rax
0x1400d7518: test byte ptr [rax], 1
0x1400d751b: je 0x1400d752e
0x1400d751d: mov rcx, rdi
0x1400d7520: add rcx, 8
0x1400d7524: mov rax, qword ptr [rdi + 8]
0x1400d7528: mov rdx, qword ptr [rdi + 0x10]
0x1400d752c: jmp 0x1400d7550
0x1400d752e: mov byte ptr [rbp + 0xfb6], 1
0x1400d7535: call 0x140d64e00
0x1400d753a: mov rcx, rdi
0x1400d753d: add rcx, 8
0x1400d7541: mov qword ptr [rdi], 1
0x1400d7548: mov qword ptr [rdi + 8], rax
0x1400d754c: mov qword ptr [rdi + 0x10], rdx
0x1400d7550: lea r8, [rax + 1]
0x1400d7554: mov qword ptr [rcx], r8
0x1400d7557: movups xmm6, xmmword ptr [rip + 0x2862cf2]
0x1400d755e: movaps xmmword ptr [rbp + 0x280], xmm6
0x1400d7565: movdqu xmm7, xmmword ptr [rip + 0x2862cd3]
0x1400d756d: movdqa xmmword ptr [rbp + 0x270], xmm7
0x1400d7575: mov qword ptr [rbp + 0x290], rax
0x1400d757c: mov qword ptr [rbp + 0x298], rdx
0x1400d7583: call 0x140d4d740
0x1400d7588: mov rdi, rax
0x1400d758b: test byte ptr [rax], 1
0x1400d758e: je 0x1400d75a1
0x1400d7590: mov rcx, rdi
0x1400d7593: add rcx, 8
0x1400d7597: mov rax, qword ptr [rdi + 8]
0x1400d759b: mov rdx, qword ptr [rdi + 0x10]
0x1400d759f: jmp 0x1400d75bc
0x1400d75a1: call 0x140d64e00
0x1400d75a6: mov rcx, rdi
0x1400d75a9: add rcx, 8
0x1400d75ad: mov qword ptr [rdi], 1
0x1400d75b4: mov qword ptr [rdi + 8], rax
0x1400d75b8: mov qword ptr [rdi + 0x10], rdx
0x1400d75bc: lea r8, [rax + 1]
0x1400d75c0: mov qword ptr [rcx], r8
0x1400d75c3: movdqa xmm0, xmmword ptr [rbp + 0x270]
0x1400d75cb: movdqa xmm1, xmmword ptr [rbp + 0x280]
0x1400d75d3: movaps xmm2, xmmword ptr [rbp + 0x290]
0x1400d75da: movaps xmmword ptr [rbp + 0x8c0], xmm2
0x1400d75e1: movdqa xmmword ptr [rbp + 0x8b0], xmm1
0x1400d75e9: movdqa xmmword ptr [rbp + 0x8a0], xmm0
0x1400d75f1: movdqa xmmword ptr [rbp + 0x8d0], xmm7
0x1400d75f9: movaps xmmword ptr [rbp + 0x8e0], xmm6
0x1400d7600: mov qword ptr [rbp + 0x8f0], rax
0x1400d7607: mov qword ptr [rbp + 0x8f8], rdx
0x1400d760e: call 0x140d4d740
0x1400d7613: mov rdi, rax
0x1400d7616: test byte ptr [rax], 1
0x1400d7619: je 0x1400d762c
0x1400d761b: mov rcx, rdi
0x1400d761e: add rcx, 8
0x1400d7622: mov rax, qword ptr [rdi + 8]
0x1400d7626: mov rdx, qword ptr [rdi + 0x10]
0x1400d762a: jmp 0x1400d7647
0x1400d762c: call 0x140d64e00
0x1400d7631: mov rcx, rdi
0x1400d7634: add rcx, 8
0x1400d7638: mov qword ptr [rdi], 1
0x1400d763f: mov qword ptr [rdi + 8], rax
0x1400d7643: mov qword ptr [rdi + 0x10], rdx
0x1400d7647: lea r8, [rax + 1]
0x1400d764b: mov qword ptr [rcx], r8
0x1400d764e: movaps xmm0, xmmword ptr [rbp + 0x8f0]
0x1400d7655: movups xmmword ptr [rbp + 0xf38], xmm0
0x1400d765c: movaps xmm0, xmmword ptr [rbp + 0x8e0]
0x1400d7663: movups xmmword ptr [rbp + 0xf28], xmm0
0x1400d766a: movdqa xmm0, xmmword ptr [rbp + 0x8a0]
0x1400d7672: movdqa xmm1, xmmword ptr [rbp + 0x8b0]
0x1400d767a: movaps xmm2, xmmword ptr [rbp + 0x8c0]
0x1400d7681: movaps xmm3, xmmword ptr [rbp + 0x8d0]
0x1400d7688: movups xmmword ptr [rbp + 0xf18], xmm3
0x1400d768f: movups xmmword ptr [rbp + 0xf08], xmm2
0x1400d7696: movdqu xmmword ptr [rbp + 0xef8], xmm1
0x1400d769e: movdqu xmmword ptr [rbp + 0xee8], xmm0
0x1400d76a6: movdqu xmmword ptr [rbp + 0xf48], xmm7
0x1400d76ae: movups xmmword ptr [rbp + 0xf58], xmm6
0x1400d76b5: mov qword ptr [rbp + 0xf68], rax
0x1400d76bc: mov qword ptr [rbp + 0xf70], rdx
0x1400d76c3: mov qword ptr [rbp + 0xed0], 0
0x1400d76ce: mov qword ptr [rbp + 0xed8], 8
0x1400d76d9: mov qword ptr [rbp + 0xee0], 0
0x1400d76e4: mov rax, qword ptr [rbp + 0xfa8]
0x1400d76eb: lea rdi, [rax + 0x10]
0x1400d76ef: lea rbx, [rax + 0xa0]
0x1400d76f6: movabs r14, 0x5846f8d25a034525
0x1400d7700: movdqa xmm6, xmmword ptr [rip + 0x2862a78]
0x1400d7708: pcmpeqd xmm7, xmm7
0x1400d770c: movabs r15, 0x4d00324fb817a8a1
0x1400d7716: nop word ptr cs:[rax + rax]
0x1400d7720: mov byte ptr [rbp + 0xfb7], 1
0x1400d7727: mov rcx, rdi
0x1400d772a: call 0x140ca67a0
0x1400d772f: mov rax, qword ptr [rbp + 0xfa8]
0x1400d7736: cmp qword ptr [rax + 0x98], 0
0x1400d773e: je 0x1400d77e0
0x1400d7744: mov rcx, qword ptr [rbp + 0xfa8]
0x1400d774b: mov rax, qword ptr [rcx + 0x80]
0x1400d7752: mov rcx, qword ptr [rcx + 0x88]
0x1400d7759: mov rdx, rcx
0x1400d775c: and rdx, r14
0x1400d775f: xor r8d, r8d
0x1400d7762: movdqu xmm0, xmmword ptr [rax + rdx]
0x1400d7767: movdqa xmm1, xmm0
0x1400d776b: pcmpeqb xmm1, xmm6
0x1400d776f: pmovmskb r9d, xmm1
0x1400d7774: test r9d, r9d
0x1400d7777: jne 0x1400d77ad
0x1400d7779: pcmpeqb xmm0, xmm7
0x1400d777d: pmovmskb r9d, xmm0
0x1400d7782: test r9d, r9d
0x1400d7785: jne 0x1400d77e0
0x1400d7787: add rdx, r8
0x1400d778a: add rdx, 0x10
0x1400d778e: add r8, 0x10
0x1400d7792: and rdx, rcx
0x1400d7795: jmp 0x1400d7762
0x1400d7797: nop word ptr [rax + rax]
0x1400d77a0: lea r10d, [r9 - 1]
0x1400d77a4: and r10w, r9w
0x1400d77a8: mov r9d, r10d
0x1400d77ab: je 0x1400d7779
0x1400d77ad: tzcnt r10d, r9d
0x1400d77b2: add r10, rdx
0x1400d77b5: and r10, rcx
0x1400d77b8: neg r10
0x1400d77bb: lea r10, [r10 + r10*2]
0x1400d77bf: cmp qword ptr [rax + r10*8 - 0x18], r15
0x1400d77c4: jne 0x1400d77a0
0x1400d77c6: cmp qword ptr [rax + r10*8 - 0x10], r14
0x1400d77cb: jne 0x1400d77a0
0x1400d77cd: lea rax, [rax + r10*8]
0x1400d77d1: mov r12, qword ptr [rax - 8]
0x1400d77d5: jmp 0x1400d78c7
0x1400d77da: nop word ptr [rax + rax]
0x1400d77e0: mov eax, dword ptr [rbx]
0x1400d77e2: cmp eax, 0x3ffffffd
0x1400d77e7: ja 0x1400d7932
0x1400d77ed: lea ecx, [rax + 1]
0x1400d77f0: lock cmpxchg dword ptr [rbx], ecx
0x1400d77f4: jne 0x1400d7932
0x1400d77fa: mov rcx, qword ptr [rbp + 0xfa8]
0x1400d7801: movzx eax, byte ptr [rcx + 0xa8]
0x1400d7808: cmp qword ptr [rcx + 0x100], 0
0x1400d7810: je 0x1400d7bf6
0x1400d7816: mov rcx, qword ptr [rbp + 0xfa8]
0x1400d781d: mov rax, qword ptr [rcx + 0xe8]
0x1400d7824: mov rcx, qword ptr [rcx + 0xf0]
0x1400d782b: mov rdx, rcx
0x1400d782e: and rdx, r14
0x1400d7831: xor r8d, r8d
0x1400d7834: movdqu xmm0, xmmword ptr [rax + rdx]
0x1400d7839: movdqa xmm1, xmm0
0x1400d783d: pcmpeqb xmm1, xmm6
0x1400d7841: pmovmskb r9d, xmm1
0x1400d7846: test r9d, r9d
0x1400d7849: jne 0x1400d787d
0x1400d784b: pcmpeqb xmm0, xmm7
0x1400d784f: pmovmskb r9d, xmm0
0x1400d7854: test r9d, r9d
0x1400d7857: jne 0x1400d7bf6
0x1400d785d: add rdx, r8
0x1400d7860: add rdx, 0x10
0x1400d7864: add r8, 0x10
0x1400d7868: and rdx, rcx
0x1400d786b: jmp 0x1400d7834
0x1400d786d: nop dword ptr [rax]
0x1400d7870: lea r10d, [r9 - 1]
0x1400d7874: and r10w, r9w
0x1400d7878: mov r9d, r10d
0x1400d787b: je 0x1400d784b
0x1400d787d: tzcnt r11d, r9d
0x1400d7882: add r11, rdx
0x1400d7885: and r11, rcx
0x1400d7888: mov r10, r11
0x1400d788b: neg r10
0x1400d788e: shl r11, 3
0x1400d7892: shl r10, 7
0x1400d7896: sub r10, r11
0x1400d7899: cmp qword ptr [rax + r10 - 0x88], r15
0x1400d78a1: jne 0x1400d7870
0x1400d78a3: cmp qword ptr [rax + r10 - 0x80], r14
0x1400d78a8: jne 0x1400d7870
0x1400d78aa: add rax, r10
0x1400d78ad: mov r12, qword ptr [rax - 8]
0x1400d78b1: mov edx, 0xffffffff
0x1400d78b6: lock xadd dword ptr [rbx], edx
0x1400d78ba: dec edx
0x1400d78bc: mov eax, edx
0x1400d78be: and eax, 0xbfffffff
0x1400d78c3: neg eax
0x1400d78c5: jo 0x1400d7946
0x1400d78c7: mov rax, qword ptr [rbp + 0xfa8]
0x1400d78ce: cmp qword ptr [rax + 0x228], r12
0x1400d78d5: jbe 0x1400d7c0c
0x1400d78db: mov rax, qword ptr [rbp + 0xfa8]
0x1400d78e2: mov rax, qword ptr [rax + 0x220]
0x1400d78e9: mov rcx, qword ptr [rax + r12*8]
0x1400d78ed: test rcx, rcx
0x1400d78f0: je 0x1400d7c0c
0x1400d78f6: mov rax, qword ptr [rbp + 0xfa8]
0x1400d78fd: mov rax, qword ptr [rax + 0x1f0]
0x1400d7904: not rcx
0x1400d7907: lea rcx, [rcx + rcx*2]
0x1400d790b: shl rcx, 5
0x1400d790f: cmp qword ptr [rax + rcx + 0x20], 0
0x1400d7915: je 0x1400d7c0c
0x1400d791b: add rax, rcx
0x1400d791e: mov rax, qword ptr [rax + 0x10]
0x1400d7922: mov rcx, qword ptr [rax + 0x30]
0x1400d7926: or rcx, qword ptr [rax + 0x10]
0x1400d792a: je 0x1400d7720
0x1400d7930: jmp 0x1400d795a
0x1400d7932: mov byte ptr [rbp + 0xfb7], 1
0x1400d7939: mov rcx, rbx
0x1400d793c: call 0x1428d8a30
0x1400d7941: jmp 0x1400d77fa
0x1400d7946: mov byte ptr [rbp + 0xfb7], 1
0x1400d794d: mov rcx, rbx
0x1400d7950: call 0x1428d8d40
0x1400d7955: jmp 0x1400d78c7
0x1400d795a: mov rbx, qword ptr [rip + 0x3554a47]
0x1400d7961: mov rax, qword ptr [rbx]
0x1400d7964: cmp rax, 3
0x1400d7968: jb 0x1400d7a36
0x1400d796e: mov byte ptr [rbp + 0xfb7], 1
0x1400d7975: lea rcx, [rip + 0x2862c8c]
0x1400d797c: call 0x14067dd80
0x1400d7981: movdqu xmm0, xmmword ptr [rax]
0x1400d7985: mov eax, dword ptr [rax + 0x10]
0x1400d7988: mov qword ptr [rbp + 0x8d0], 3
0x1400d7993: lea rcx, [rip + 0x2862c86]
0x1400d799a: mov qword ptr [rbp + 0x8d8], rcx
0x1400d79a1: mov qword ptr [rbp + 0x8e0], 0x14
0x1400d79ac: lea rdx, [rip + 0x2862c45]
0x1400d79b3: mov qword ptr [rbp + 0x8f0], rdx
0x1400d79ba: mov qword ptr [rbp + 0x8f8], 1
0x1400d79c5: mov qword ptr [rbp + 0x900], 8
0x1400d79d0: mov qword ptr [rbp + 0x8a0], 0
0x1400d79db: pxor xmm1, xmm1
0x1400d79df: movdqu xmmword ptr [rbp + 0x908], xmm1
0x1400d79e7: mov qword ptr [rbp + 0x8a8], rcx
0x1400d79ee: mov qword ptr [rbp + 0x8b0], 0x14
0x1400d79f9: mov qword ptr [rbp + 0x8b8], 0
0x1400d7a04: movdqu xmmword ptr [rbp + 0x8c0], xmm0
0x1400d7a0c: mov dword ptr [rbp + 0x8e8], 1
0x1400d7a16: mov dword ptr [rbp + 0x8ec], eax
0x1400d7a1c: mov byte ptr [rbp + 0xfb7], 1
0x1400d7a23: lea rcx, [rbp + 0x270]
0x1400d7a2a: lea rdx, [rbp + 0x8a0]
0x1400d7a31: call 0x140d4a330
0x1400d7a36: mov byte ptr [rbp + 0xfb7], 1
0x1400d7a3d: pxor xmm0, xmm0
0x1400d7a41: movdqu xmmword ptr [rsp + 0x28], xmm0
0x1400d7a47: mov qword ptr [rsp + 0x20], rsi
0x1400d7a4c: mov byte ptr [rsp + 0x38], 1
0x1400d7a51: movabs rcx, 0x2000000020
0x1400d7a5b: lea r9, [rbp - 0x40]
0x1400d7a5f: mov rdx, rdi
0x1400d7a62: xor r8d, r8d
0x1400d7a65: call 0x1404e8010
0x1400d7a6a: mov rax, qword ptr [rbx]
0x1400d7a6d: cmp rax, 3
0x1400d7a71: jb 0x1400d7b3f
0x1400d7a77: mov byte ptr [rbp + 0xfb7], 1
0x1400d7a7e: lea rcx, [rip + 0x2862bf3]
0x1400d7a85: call 0x14067dd80
0x1400d7a8a: movdqu xmm0, xmmword ptr [rax]
0x1400d7a8e: mov eax, dword ptr [rax + 0x10]
0x1400d7a91: mov qword ptr [rbp + 0x8d0], 3
0x1400d7a9c: lea rcx, [rip + 0x2862b7d]
0x1400d7aa3: mov qword ptr [rbp + 0x8d8], rcx
0x1400d7aaa: mov qword ptr [rbp + 0x8e0], 0x14
0x1400d7ab5: lea rdx, [rip + 0x2862bac]
0x1400d7abc: mov qword ptr [rbp + 0x8f0], rdx
0x1400d7ac3: mov qword ptr [rbp + 0x8f8], 1
0x1400d7ace: mov qword ptr [rbp + 0x900], 8
0x1400d7ad9: mov qword ptr [rbp + 0x8a0], 0
0x1400d7ae4: pxor xmm1, xmm1
0x1400d7ae8: movdqu xmmword ptr [rbp + 0x908], xmm1
0x1400d7af0: mov qword ptr [rbp + 0x8a8], rcx
0x1400d7af7: mov qword ptr [rbp + 0x8b0], 0x14
0x1400d7b02: mov qword ptr [rbp + 0x8b8], 0
0x1400d7b0d: movdqu xmmword ptr [rbp + 0x8c0], xmm0
0x1400d7b15: mov dword ptr [rbp + 0x8e8], 1
0x1400d7b1f: mov dword ptr [rbp + 0x8ec], eax
0x1400d7b25: mov byte ptr [rbp + 0xfb7], 1
0x1400d7b2c: lea rcx, [rbp + 0x270]
0x1400d7b33: lea rdx, [rbp + 0x8a0]
0x1400d7b3a: call 0x140d4a330
0x1400d7b3f: lea rsi, [rbp + 0x8a0]
0x1400d7b46: mov r8d, 0x628
0x1400d7b4c: mov rcx, rsi
0x1400d7b4f: mov rdx, rdi
0x1400d7b52: call 0x142923c00
0x1400d7b57: mov byte ptr [rbp + 0xfb7], 0
0x1400d7b5e: lea rcx, [rbp + 0x270]
0x1400d7b65: mov rdx, qword ptr [rbp + 0xfa8]
0x1400d7b6c: mov r8, rsi
0x1400d7b6f: call 0x1403c5e00
0x1400d7b74: xor eax, eax
0x1400d7b76: cmp rax, qword ptr [rbp + 0x270]
0x1400d7b7d: jo 0x1400d7b92
0x1400d7b7f: mov byte ptr [rbp + 0xfb7], 0
0x1400d7b86: lea rcx, [rbp + 0x270]
0x1400d7b8d: call 0x1405e8b60
0x1400d7b92: mov byte ptr [rbp + 0xfb6], 0
0x1400d7b99: lea rcx, [rbp + 0xed0]
0x1400d7ba0: call 0x1405f7630
0x1400d7ba5: mov byte ptr [rbp + 0xfb5], 0
0x1400d7bac: lea rcx, [rbp - 0x40]
0x1400d7bb0: call 0x1405f12b0
0x1400d7bb5: mov rcx, qword ptr [rbp + 0xfa0]
0x1400d7bbc: mov rax, qword ptr [rcx]
0x1400d7bbf: lock dec qword ptr [rax]
0x1400d7bc3: jne 0x1400d7bca
0x1400d7bc5: call 0x140b5be90
0x1400d7bca: mov rcx, qword ptr [rbp + 0xfa8]
0x1400d7bd1: call 0x140601d60
0x1400d7bd6: movaps xmm6, xmmword ptr [rbp + 0xfc0]
0x1400d7bdd: movaps xmm7, xmmword ptr [rbp + 0xfd0]
0x1400d7be4: add rsp, 0x1060
0x1400d7beb: pop rbx
0x1400d7bec: pop rdi
0x1400d7bed: pop rsi
0x1400d7bee: pop r12
0x1400d7bf0: pop r14
0x1400d7bf2: pop r15
0x1400d7bf4: pop rbp
0x1400d7bf5: ret 
0x1400d7bf6: mov edx, 0xffffffff
0x1400d7bfb: lock xadd dword ptr [rbx], edx
0x1400d7bff: dec edx
0x1400d7c01: mov eax, edx
0x1400d7c03: and eax, 0xbfffffff
0x1400d7c08: neg eax
0x1400d7c0a: jo 0x1400d7c21
0x1400d7c0c: mov byte ptr [rbp + 0xfb7], 1
0x1400d7c13: lea rcx, [rip + 0x286298e]
0x1400d7c1a: call 0x1428d9310
0x1400d7c1f: ud2 
0x1400d7c21: mov byte ptr [rbp + 0xfb7], 1
0x1400d7c28: mov rcx, rbx
0x1400d7c2b: call 0x1428d8d40
0x1400d7c30: jmp 0x1400d7c0c
0x1400d7c32: nop word ptr cs:[rax + rax]
0x1400d7c40: mov qword ptr [rsp + 0x10], rdx
0x1400d7c45: push rbp
0x1400d7c46: push r15
0x1400d7c48: push r14
0x1400d7c4a: push r12
0x1400d7c4c: push rsi
0x1400d7c4d: push rdi
0x1400d7c4e: push rbx
0x1400d7c4f: sub rsp, 0x60
0x1400d7c53: lea rbp, [rdx + 0x80]
0x1400d7c5a: movdqa xmmword ptr [rsp + 0x40], xmm7
0x1400d7c60: movdqa xmmword ptr [rsp + 0x50], xmm6
0x1400d7c66: lea rcx, [rbp + 0x270]
0x1400d7c6d: call 0x1405f3270
0x1400d7c72: movdqa xmm6, xmmword ptr [rsp + 0x50]
0x1400d7c78: movdqa xmm7, xmmword ptr [rsp + 0x40]
0x1400d7c7e: add rsp, 0x60
0x1400d7c82: pop rbx
0x1400d7c83: pop rdi
0x1400d7c84: pop rsi
0x1400d7c85: pop r12
0x1400d7c87: pop r14
0x1400d7c89: pop r15
0x1400d7c8b: pop rbp
0x1400d7c8c: ret 
0x1400d7c8d: nop dword ptr [rax]
0x1400d7c90: mov qword ptr [rsp + 0x10], rdx
0x1400d7c95: push rbp
0x1400d7c96: push r15
0x1400d7c98: push r14
0x1400d7c9a: push r12
0x1400d7c9c: push rsi
0x1400d7c9d: push rdi
0x1400d7c9e: push rbx
0x1400d7c9f: sub rsp, 0x60
0x1400d7ca3: lea rbp, [rdx + 0x80]
0x1400d7caa: movdqa xmmword ptr [rsp + 0x40], xmm7
0x1400d7cb0: movdqa xmmword ptr [rsp + 0x50], xmm6
0x1400d7cb6: movzx ebx, byte ptr [rbp + 0xfb5]
0x1400d7cbd: mov rcx, qword ptr [rbp + 0xfa0]
0x1400d7cc4: mov rax, qword ptr [rcx]
0x1400d7cc7: lock dec qword ptr [rax]
0x1400d7ccb: jne 0x1400d7cd2
0x1400d7ccd: call 0x140b5be90
0x1400d7cd2: test bl, 1
0x1400d7cd5: je 0x1400d7ce7
0x1400d7cd7: mov rax, qword ptr [rbp + 0xfa8]
0x1400d7cde: lea rcx, [rax + 0x10]
0x1400d7ce2: call 0x1405e8b60
0x1400d7ce7: mov rcx, qword ptr [rbp + 0xfa8]
0x1400d7cee: call 0x140601d60
0x1400d7cf3: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1400d7cf8: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1400d7cfd: add rsp, 0x60
0x1400d7d01: pop rbx
0x1400d7d02: pop rdi
0x1400d7d03: pop rsi
0x1400d7d04: pop r12
0x1400d7d06: pop r14
0x1400d7d08: pop r15
0x1400d7d0a: pop rbp
0x1400d7d0b: ret 
0x1400d7d0c: nop dword ptr [rax]
0x1400d7d10: mov qword ptr [rsp + 0x10], rdx
0x1400d7d15: push rbp
0x1400d7d16: push r15
0x1400d7d18: push r14
0x1400d7d1a: push r12
0x1400d7d1c: push rsi
0x1400d7d1d: push rdi
0x1400d7d1e: push rbx
0x1400d7d1f: sub rsp, 0x60
0x1400d7d23: lea rbp, [rdx + 0x80]
0x1400d7d2a: movdqa xmmword ptr [rsp + 0x40], xmm7
0x1400d7d30: movaps xmmword ptr [rsp + 0x50], xmm6
0x1400d7d35: mov byte ptr [rbp + 0xfb6], 1
0x1400d7d3c: lea rcx, [rbp + 0x270]
0x1400d7d43: mov r8d, 0x38
0x1400d7d49: mov r9d, 0x10
0x1400d7d4f: lea rdx, [rbp + 0x290]
0x1400d7d56: call 0x140053060
0x1400d7d5b: mov byte ptr [rbp + 0xfb6], 1
0x1400d7d62: movdqa xmm6, xmmword ptr [rsp + 0x50]
0x1400d7d68: movdqa xmm7, xmmword ptr [rsp + 0x40]
0x1400d7d6e: add rsp, 0x60
0x1400d7d72: pop rbx
0x1400d7d73: pop rdi
0x1400d7d74: pop rsi
0x1400d7d75: pop r12
0x1400d7d77: pop r14
0x1400d7d79: pop r15
0x1400d7d7b: pop rbp
0x1400d7d7c: ret 
0x1400d7d7d: nop dword ptr [rax]
0x1400d7d80: mov qword ptr [rsp + 0x10], rdx
0x1400d7d85: push rbp
0x1400d7d86: push r15
0x1400d7d88: push r14
0x1400d7d8a: push r12
0x1400d7d8c: push rsi
0x1400d7d8d: push rdi
0x1400d7d8e: push rbx
0x1400d7d8f: sub rsp, 0x60
0x1400d7d93: lea rbp, [rdx + 0x80]
0x1400d7d9a: movdqa xmmword ptr [rsp + 0x40], xmm7
0x1400d7da0: movdqa xmmword ptr [rsp + 0x50], xmm6
0x1400d7da6: movzx ebx, byte ptr [rbp + 0xfb6]
0x1400d7dad: lea rcx, [rbp - 0x40]
0x1400d7db1: call 0x1405f12b0
0x1400d7db6: mov byte ptr [rbp + 0xfb5], bl
0x1400d7dbc: movdqa xmm6, xmmword ptr [rsp + 0x50]
0x1400d7dc2: movdqa xmm7, xmmword ptr [rsp + 0x40]
0x1400d7dc8: add rsp, 0x60
0x1400d7dcc: pop rbx
0x1400d7dcd: pop rdi
0x1400d7dce: pop rsi
0x1400d7dcf: pop r12
0x1400d7dd1: pop r14
0x1400d7dd3: pop r15
0x1400d7dd5: pop rbp
0x1400d7dd6: ret 
0x1400d7dd7: nop word ptr [rax + rax]
0x1400d7de0: mov qword ptr [rsp + 0x10], rdx
0x1400d7de5: push rbp
0x1400d7de6: push r15
0x1400d7de8: push r14
0x1400d7dea: push r12
0x1400d7dec: push rsi
0x1400d7ded: push rdi
0x1400d7dee: push rbx
0x1400d7def: sub rsp, 0x60
0x1400d7df3: lea rbp, [rdx + 0x80]
0x1400d7dfa: movdqa xmmword ptr [rsp + 0x40], xmm7
0x1400d7e00: movaps xmmword ptr [rsp + 0x50], xmm6
0x1400d7e05: lea rcx, [rbp + 0x8a0]
0x1400d7e0c: call 0x1405f6770
0x1400d7e11: mov byte ptr [rbp + 0xfb6], 1
0x1400d7e18: movdqa xmm6, xmmword ptr [rsp + 0x50]
0x1400d7e1e: movdqa xmm7, xmmword ptr [rsp + 0x40]
0x1400d7e24: add rsp, 0x60
0x1400d7e28: pop rbx
0x1400d7e29: pop rdi
0x1400d7e2a: pop rsi
0x1400d7e2b: pop r12
0x1400d7e2d: pop r14
0x1400d7e2f: pop r15
0x1400d7e31: pop rbp
0x1400d7e32: ret 
0x1400d7e33: nop word ptr cs:[rax + rax]
0x1400d7e40: mov qword ptr [rsp + 0x10], rdx
0x1400d7e45: push rbp
0x1400d7e46: push r15
0x1400d7e48: push r14
0x1400d7e4a: push r12
0x1400d7e4c: push rsi
0x1400d7e4d: push rdi
0x1400d7e4e: push rbx
0x1400d7e4f: sub rsp, 0x60
0x1400d7e53: lea rbp, [rdx + 0x80]
0x1400d7e5a: movdqa xmmword ptr [rsp + 0x40], xmm7
0x1400d7e60: movdqa xmmword ptr [rsp + 0x50], xmm6
0x1400d7e66: mov rcx, qword ptr [rbp + 0xfa8]
0x1400d7e6d: call 0x140601d60
0x1400d7e72: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1400d7e77: movaps xmm7, xmmword ptr [rsp + 0x40]
0x1400d7e7c: add rsp, 0x60
0x1400d7e80: pop rbx
0x1400d7e81: pop rdi
0x1400d7e82: pop rsi
0x1400d7e83: pop r12
0x1400d7e85: pop r14
0x1400d7e87: pop r15
0x1400d7e89: pop rbp
0x1400d7e8a: ret 
0x1400d7e8b: nop dword ptr [rax + rax]
0x1400d7e90: mov qword ptr [rsp + 0x10], rdx
0x1400d7e95: push rbp
0x1400d7e96: push r15
0x1400d7e98: push r14
0x1400d7e9a: push r12
0x1400d7e9c: push rsi
0x1400d7e9d: push rdi
0x1400d7e9e: push rbx
0x1400d7e9f: sub rsp, 0x60
0x1400d7ea3: lea rbp, [rdx + 0x80]
0x1400d7eaa: movdqa xmmword ptr [rsp + 0x40], xmm7
0x1400d7eb0: movdqa xmmword ptr [rsp + 0x50], xmm6
0x1400d7eb6: mov byte ptr [rbp + 0xfb5], 1
0x1400d7ebd: lea rcx, [rbp + 0xed0]
0x1400d7ec4: call 0x1405f2ea0
0x1400d7ec9: mov byte ptr [rbp + 0xfb5], 1
0x1400d7ed0: movdqa xmm6, xmmword ptr [rsp + 0x50]
0x1400d7ed6: movdqa xmm7, xmmword ptr [rsp + 0x40]
0x1400d7edc: add rsp, 0x60
0x1400d7ee0: pop rbx
0x1400d7ee1: pop rdi
0x1400d7ee2: pop rsi
0x1400d7ee3: pop r12
0x1400d7ee5: pop r14
0x1400d7ee7: pop r15
0x1400d7ee9: pop rbp
0x1400d7eea: ret 
0x1400d7eeb: nop dword ptr [rax + rax]
0x1400d7ef0: mov qword ptr [rsp + 0x10], rdx
0x1400d7ef5: push rbp
0x1400d7ef6: push r15
0x1400d7ef8: push r14
0x1400d7efa: push r12
0x1400d7efc: push rsi
0x1400d7efd: push rdi
0x1400d7efe: push rbx
0x1400d7eff: sub rsp, 0x60
0x1400d7f03: lea rbp, [rdx + 0x80]
0x1400d7f0a: movdqa xmmword ptr [rsp + 0x40], xmm7
0x1400d7f10: movdqa xmmword ptr [rsp + 0x50], xmm6
0x1400d7f16: movzx ebx, byte ptr [rbp + 0xfb7]
0x1400d7f1d: lea rcx, [rbp + 0xed0]
0x1400d7f24: call 0x1405f7630
0x1400d7f29: mov byte ptr [rbp + 0xfb6], bl
0x1400d7f2f: movdqa xmm6, xmmword ptr [rsp + 0x50]
0x1400d7f35: movdqa xmm7, xmmword ptr [rsp + 0x40]
0x1400d7f3b: add rsp, 0x60
0x1400d7f3f: pop rbx
0x1400d7f40: pop rdi
0x1400d7f41: pop rsi
0x1400d7f42: pop r12
0x1400d7f44: pop r14
0x1400d7f46: pop r15
0x1400d7f48: pop rbp
0x1400d7f49: ret 
