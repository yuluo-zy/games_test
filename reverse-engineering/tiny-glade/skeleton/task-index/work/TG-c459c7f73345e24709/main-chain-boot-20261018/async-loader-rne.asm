
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1400d7440: 55                          	push	rbp
1400d7441: 41 57                       	push	r15
1400d7443: 41 56                       	push	r14
1400d7445: 41 54                       	push	r12
1400d7447: 56                          	push	rsi
1400d7448: 57                          	push	rdi
1400d7449: 53                          	push	rbx
1400d744a: b8 60 10 00 00              	mov	eax, 0x1060
1400d744f: e8 ec 35 7c 02              	call	0x14289aa40 <.text+0x2899a40>
1400d7454: 48 29 c4                    	sub	rsp, rax
1400d7457: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
1400d745f: 66 0f 7f bd d0 0f 00 00     	movdqa	xmmword ptr [rbp + 0xfd0], xmm7
1400d7467: 66 0f 7f b5 c0 0f 00 00     	movdqa	xmmword ptr [rbp + 0xfc0], xmm6
1400d746f: 48 c7 85 b8 0f 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0xfb8], -0x2
1400d747a: 48 8d 81 38 06 00 00        	lea	rax, [rcx + 0x638]
1400d7481: 48 89 85 a0 0f 00 00        	mov	qword ptr [rbp + 0xfa0], rax
1400d7488: 48 89 8d a8 0f 00 00        	mov	qword ptr [rbp + 0xfa8], rcx
1400d748f: 48 8b 91 38 06 00 00        	mov	rdx, qword ptr [rcx + 0x638]
1400d7496: 48 83 c2 10                 	add	rdx, 0x10
1400d749a: c6 85 b5 0f 00 00 01        	mov	byte ptr [rbp + 0xfb5], 0x1
1400d74a1: 48 8d 8d d0 0e 00 00        	lea	rcx, [rbp + 0xed0]
1400d74a8: e8 23 2e b1 00              	call	0x140bea2d0 <.text+0xbe92d0>
1400d74ad: 48 8d 8d 70 02 00 00        	lea	rcx, [rbp + 0x270]
1400d74b4: e8 17 e3 b0 00              	call	0x140be57d0 <.text+0xbe47d0>
1400d74b9: 48 8d 8d a0 08 00 00        	lea	rcx, [rbp + 0x8a0]
1400d74c0: e8 0b e3 b0 00              	call	0x140be57d0 <.text+0xbe47d0>
1400d74c5: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
1400d74c9: 48 8d b5 d0 0e 00 00        	lea	rsi, [rbp + 0xed0]
1400d74d0: 41 b8 d0 00 00 00           	mov	r8d, 0xd0
1400d74d6: 48 89 f2                    	mov	rdx, rsi
1400d74d9: e8 22 c7 84 02              	call	0x142923c00 <.text+0x2922c00>
1400d74de: 48 8d 8d 90 00 00 00        	lea	rcx, [rbp + 0x90]
1400d74e5: 48 8d 95 70 02 00 00        	lea	rdx, [rbp + 0x270]
1400d74ec: 41 b8 f0 00 00 00           	mov	r8d, 0xf0
1400d74f2: e8 09 c7 84 02              	call	0x142923c00 <.text+0x2922c00>
1400d74f7: 48 8d 8d 80 01 00 00        	lea	rcx, [rbp + 0x180]
1400d74fe: 48 8d 95 a0 08 00 00        	lea	rdx, [rbp + 0x8a0]
1400d7505: 41 b8 f0 00 00 00           	mov	r8d, 0xf0
1400d750b: e8 f0 c6 84 02              	call	0x142923c00 <.text+0x2922c00>
1400d7510: e8 2b 62 c7 00              	call	0x140d4d740 <.text+0xd4c740>
1400d7515: 48 89 c7                    	mov	rdi, rax
1400d7518: f6 00 01                    	test	byte ptr [rax], 0x1
1400d751b: 74 11                       	je	0x1400d752e <.text+0xd652e>
1400d751d: 48 89 f9                    	mov	rcx, rdi
1400d7520: 48 83 c1 08                 	add	rcx, 0x8
1400d7524: 48 8b 47 08                 	mov	rax, qword ptr [rdi + 0x8]
1400d7528: 48 8b 57 10                 	mov	rdx, qword ptr [rdi + 0x10]
1400d752c: eb 22                       	jmp	0x1400d7550 <.text+0xd6550>
1400d752e: c6 85 b6 0f 00 00 01        	mov	byte ptr [rbp + 0xfb6], 0x1
1400d7535: e8 c6 d8 c8 00              	call	0x140d64e00 <.text+0xd63e00>
1400d753a: 48 89 f9                    	mov	rcx, rdi
1400d753d: 48 83 c1 08                 	add	rcx, 0x8
1400d7541: 48 c7 07 01 00 00 00        	mov	qword ptr [rdi], 0x1
1400d7548: 48 89 47 08                 	mov	qword ptr [rdi + 0x8], rax
1400d754c: 48 89 57 10                 	mov	qword ptr [rdi + 0x10], rdx
1400d7550: 4c 8d 40 01                 	lea	r8, [rax + 0x1]
1400d7554: 4c 89 01                    	mov	qword ptr [rcx], r8
1400d7557: 0f 10 35 f2 2c 86 02        	movups	xmm6, xmmword ptr [rip + 0x2862cf2] # 0x14293a250
1400d755e: 0f 29 b5 80 02 00 00        	movaps	xmmword ptr [rbp + 0x280], xmm6
1400d7565: f3 0f 6f 3d d3 2c 86 02     	movdqu	xmm7, xmmword ptr [rip + 0x2862cd3] # 0x14293a240
1400d756d: 66 0f 7f bd 70 02 00 00     	movdqa	xmmword ptr [rbp + 0x270], xmm7
1400d7575: 48 89 85 90 02 00 00        	mov	qword ptr [rbp + 0x290], rax
1400d757c: 48 89 95 98 02 00 00        	mov	qword ptr [rbp + 0x298], rdx
1400d7583: e8 b8 61 c7 00              	call	0x140d4d740 <.text+0xd4c740>
1400d7588: 48 89 c7                    	mov	rdi, rax
1400d758b: f6 00 01                    	test	byte ptr [rax], 0x1
1400d758e: 74 11                       	je	0x1400d75a1 <.text+0xd65a1>
1400d7590: 48 89 f9                    	mov	rcx, rdi
1400d7593: 48 83 c1 08                 	add	rcx, 0x8
1400d7597: 48 8b 47 08                 	mov	rax, qword ptr [rdi + 0x8]
1400d759b: 48 8b 57 10                 	mov	rdx, qword ptr [rdi + 0x10]
1400d759f: eb 1b                       	jmp	0x1400d75bc <.text+0xd65bc>
1400d75a1: e8 5a d8 c8 00              	call	0x140d64e00 <.text+0xd63e00>
1400d75a6: 48 89 f9                    	mov	rcx, rdi
1400d75a9: 48 83 c1 08                 	add	rcx, 0x8
1400d75ad: 48 c7 07 01 00 00 00        	mov	qword ptr [rdi], 0x1
1400d75b4: 48 89 47 08                 	mov	qword ptr [rdi + 0x8], rax
1400d75b8: 48 89 57 10                 	mov	qword ptr [rdi + 0x10], rdx
1400d75bc: 4c 8d 40 01                 	lea	r8, [rax + 0x1]
1400d75c0: 4c 89 01                    	mov	qword ptr [rcx], r8
1400d75c3: 66 0f 6f 85 70 02 00 00     	movdqa	xmm0, xmmword ptr [rbp + 0x270]
1400d75cb: 66 0f 6f 8d 80 02 00 00     	movdqa	xmm1, xmmword ptr [rbp + 0x280]
1400d75d3: 0f 28 95 90 02 00 00        	movaps	xmm2, xmmword ptr [rbp + 0x290]
1400d75da: 0f 29 95 c0 08 00 00        	movaps	xmmword ptr [rbp + 0x8c0], xmm2
1400d75e1: 66 0f 7f 8d b0 08 00 00     	movdqa	xmmword ptr [rbp + 0x8b0], xmm1
1400d75e9: 66 0f 7f 85 a0 08 00 00     	movdqa	xmmword ptr [rbp + 0x8a0], xmm0
1400d75f1: 66 0f 7f bd d0 08 00 00     	movdqa	xmmword ptr [rbp + 0x8d0], xmm7
1400d75f9: 0f 29 b5 e0 08 00 00        	movaps	xmmword ptr [rbp + 0x8e0], xmm6
1400d7600: 48 89 85 f0 08 00 00        	mov	qword ptr [rbp + 0x8f0], rax
1400d7607: 48 89 95 f8 08 00 00        	mov	qword ptr [rbp + 0x8f8], rdx
1400d760e: e8 2d 61 c7 00              	call	0x140d4d740 <.text+0xd4c740>
1400d7613: 48 89 c7                    	mov	rdi, rax
1400d7616: f6 00 01                    	test	byte ptr [rax], 0x1
1400d7619: 74 11                       	je	0x1400d762c <.text+0xd662c>
1400d761b: 48 89 f9                    	mov	rcx, rdi
1400d761e: 48 83 c1 08                 	add	rcx, 0x8
1400d7622: 48 8b 47 08                 	mov	rax, qword ptr [rdi + 0x8]
1400d7626: 48 8b 57 10                 	mov	rdx, qword ptr [rdi + 0x10]
1400d762a: eb 1b                       	jmp	0x1400d7647 <.text+0xd6647>
1400d762c: e8 cf d7 c8 00              	call	0x140d64e00 <.text+0xd63e00>
1400d7631: 48 89 f9                    	mov	rcx, rdi
1400d7634: 48 83 c1 08                 	add	rcx, 0x8
1400d7638: 48 c7 07 01 00 00 00        	mov	qword ptr [rdi], 0x1
1400d763f: 48 89 47 08                 	mov	qword ptr [rdi + 0x8], rax
1400d7643: 48 89 57 10                 	mov	qword ptr [rdi + 0x10], rdx
1400d7647: 4c 8d 40 01                 	lea	r8, [rax + 0x1]
1400d764b: 4c 89 01                    	mov	qword ptr [rcx], r8
1400d764e: 0f 28 85 f0 08 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x8f0]
1400d7655: 0f 11 85 38 0f 00 00        	movups	xmmword ptr [rbp + 0xf38], xmm0
1400d765c: 0f 28 85 e0 08 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x8e0]
1400d7663: 0f 11 85 28 0f 00 00        	movups	xmmword ptr [rbp + 0xf28], xmm0
1400d766a: 66 0f 6f 85 a0 08 00 00     	movdqa	xmm0, xmmword ptr [rbp + 0x8a0]
1400d7672: 66 0f 6f 8d b0 08 00 00     	movdqa	xmm1, xmmword ptr [rbp + 0x8b0]
1400d767a: 0f 28 95 c0 08 00 00        	movaps	xmm2, xmmword ptr [rbp + 0x8c0]
1400d7681: 0f 28 9d d0 08 00 00        	movaps	xmm3, xmmword ptr [rbp + 0x8d0]
1400d7688: 0f 11 9d 18 0f 00 00        	movups	xmmword ptr [rbp + 0xf18], xmm3
1400d768f: 0f 11 95 08 0f 00 00        	movups	xmmword ptr [rbp + 0xf08], xmm2
1400d7696: f3 0f 7f 8d f8 0e 00 00     	movdqu	xmmword ptr [rbp + 0xef8], xmm1
1400d769e: f3 0f 7f 85 e8 0e 00 00     	movdqu	xmmword ptr [rbp + 0xee8], xmm0
1400d76a6: f3 0f 7f bd 48 0f 00 00     	movdqu	xmmword ptr [rbp + 0xf48], xmm7
1400d76ae: 0f 11 b5 58 0f 00 00        	movups	xmmword ptr [rbp + 0xf58], xmm6
1400d76b5: 48 89 85 68 0f 00 00        	mov	qword ptr [rbp + 0xf68], rax
1400d76bc: 48 89 95 70 0f 00 00        	mov	qword ptr [rbp + 0xf70], rdx
1400d76c3: 48 c7 85 d0 0e 00 00 00 00 00 00    	mov	qword ptr [rbp + 0xed0], 0x0
1400d76ce: 48 c7 85 d8 0e 00 00 08 00 00 00    	mov	qword ptr [rbp + 0xed8], 0x8
1400d76d9: 48 c7 85 e0 0e 00 00 00 00 00 00    	mov	qword ptr [rbp + 0xee0], 0x0
1400d76e4: 48 8b 85 a8 0f 00 00        	mov	rax, qword ptr [rbp + 0xfa8]
1400d76eb: 48 8d 78 10                 	lea	rdi, [rax + 0x10]
1400d76ef: 48 8d 98 a0 00 00 00        	lea	rbx, [rax + 0xa0]
1400d76f6: 49 be 25 45 03 5a d2 f8 46 58       	movabs	r14, 0x5846f8d25a034525
1400d7700: 66 0f 6f 35 78 2a 86 02     	movdqa	xmm6, xmmword ptr [rip + 0x2862a78] # 0x14293a180
1400d7708: 66 0f 76 ff                 	pcmpeqd	xmm7, xmm7
1400d770c: 49 bf a1 a8 17 b8 4f 32 00 4d       	movabs	r15, 0x4d00324fb817a8a1
1400d7716: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
1400d7720: c6 85 b7 0f 00 00 01        	mov	byte ptr [rbp + 0xfb7], 0x1
1400d7727: 48 89 f9                    	mov	rcx, rdi
1400d772a: e8 71 f0 bc 00              	call	0x140ca67a0 <.text+0xca57a0>
1400d772f: 48 8b 85 a8 0f 00 00        	mov	rax, qword ptr [rbp + 0xfa8]
1400d7736: 48 83 b8 98 00 00 00 00     	cmp	qword ptr [rax + 0x98], 0x0
1400d773e: 0f 84 9c 00 00 00           	je	0x1400d77e0 <.text+0xd67e0>
1400d7744: 48 8b 8d a8 0f 00 00        	mov	rcx, qword ptr [rbp + 0xfa8]
1400d774b: 48 8b 81 80 00 00 00        	mov	rax, qword ptr [rcx + 0x80]
1400d7752: 48 8b 89 88 00 00 00        	mov	rcx, qword ptr [rcx + 0x88]
1400d7759: 48 89 ca                    	mov	rdx, rcx
1400d775c: 4c 21 f2                    	and	rdx, r14
1400d775f: 45 31 c0                    	xor	r8d, r8d
1400d7762: f3 0f 6f 04 10              	movdqu	xmm0, xmmword ptr [rax + rdx]
1400d7767: 66 0f 6f c8                 	movdqa	xmm1, xmm0
1400d776b: 66 0f 74 ce                 	pcmpeqb	xmm1, xmm6
1400d776f: 66 44 0f d7 c9              	pmovmskb	r9d, xmm1
1400d7774: 45 85 c9                    	test	r9d, r9d
1400d7777: 75 34                       	jne	0x1400d77ad <.text+0xd67ad>
1400d7779: 66 0f 74 c7                 	pcmpeqb	xmm0, xmm7
1400d777d: 66 44 0f d7 c8              	pmovmskb	r9d, xmm0
1400d7782: 45 85 c9                    	test	r9d, r9d
1400d7785: 75 59                       	jne	0x1400d77e0 <.text+0xd67e0>
1400d7787: 4c 01 c2                    	add	rdx, r8
1400d778a: 48 83 c2 10                 	add	rdx, 0x10
1400d778e: 49 83 c0 10                 	add	r8, 0x10
1400d7792: 48 21 ca                    	and	rdx, rcx
1400d7795: eb cb                       	jmp	0x1400d7762 <.text+0xd6762>
1400d7797: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1400d77a0: 45 8d 51 ff                 	lea	r10d, [r9 - 0x1]
1400d77a4: 66 45 21 ca                 	and	r10w, r9w
1400d77a8: 45 89 d1                    	mov	r9d, r10d
1400d77ab: 74 cc                       	je	0x1400d7779 <.text+0xd6779>
1400d77ad: f3 45 0f bc d1              	tzcnt	r10d, r9d
1400d77b2: 49 01 d2                    	add	r10, rdx
1400d77b5: 49 21 ca                    	and	r10, rcx
1400d77b8: 49 f7 da                    	neg	r10
1400d77bb: 4f 8d 14 52                 	lea	r10, [r10 + 2*r10]
1400d77bf: 4e 39 7c d0 e8              	cmp	qword ptr [rax + 8*r10 - 0x18], r15
1400d77c4: 75 da                       	jne	0x1400d77a0 <.text+0xd67a0>
1400d77c6: 4e 39 74 d0 f0              	cmp	qword ptr [rax + 8*r10 - 0x10], r14
1400d77cb: 75 d3                       	jne	0x1400d77a0 <.text+0xd67a0>
1400d77cd: 4a 8d 04 d0                 	lea	rax, [rax + 8*r10]
1400d77d1: 4c 8b 60 f8                 	mov	r12, qword ptr [rax - 0x8]
1400d77d5: e9 ed 00 00 00              	jmp	0x1400d78c7 <.text+0xd68c7>
1400d77da: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1400d77e0: 8b 03                       	mov	eax, dword ptr [rbx]
1400d77e2: 3d fd ff ff 3f              	cmp	eax, 0x3ffffffd
1400d77e7: 0f 87 45 01 00 00           	ja	0x1400d7932 <.text+0xd6932>
1400d77ed: 8d 48 01                    	lea	ecx, [rax + 0x1]
1400d77f0: f0                          	lock
1400d77f1: 0f b1 0b                    	cmpxchg	dword ptr [rbx], ecx
1400d77f4: 0f 85 38 01 00 00           	jne	0x1400d7932 <.text+0xd6932>
1400d77fa: 48 8b 8d a8 0f 00 00        	mov	rcx, qword ptr [rbp + 0xfa8]
1400d7801: 0f b6 81 a8 00 00 00        	movzx	eax, byte ptr [rcx + 0xa8]
1400d7808: 48 83 b9 00 01 00 00 00     	cmp	qword ptr [rcx + 0x100], 0x0
1400d7810: 0f 84 e0 03 00 00           	je	0x1400d7bf6 <.text+0xd6bf6>
1400d7816: 48 8b 8d a8 0f 00 00        	mov	rcx, qword ptr [rbp + 0xfa8]
1400d781d: 48 8b 81 e8 00 00 00        	mov	rax, qword ptr [rcx + 0xe8]
1400d7824: 48 8b 89 f0 00 00 00        	mov	rcx, qword ptr [rcx + 0xf0]
1400d782b: 48 89 ca                    	mov	rdx, rcx
1400d782e: 4c 21 f2                    	and	rdx, r14
1400d7831: 45 31 c0                    	xor	r8d, r8d
1400d7834: f3 0f 6f 04 10              	movdqu	xmm0, xmmword ptr [rax + rdx]
1400d7839: 66 0f 6f c8                 	movdqa	xmm1, xmm0
1400d783d: 66 0f 74 ce                 	pcmpeqb	xmm1, xmm6
1400d7841: 66 44 0f d7 c9              	pmovmskb	r9d, xmm1
1400d7846: 45 85 c9                    	test	r9d, r9d
1400d7849: 75 32                       	jne	0x1400d787d <.text+0xd687d>
1400d784b: 66 0f 74 c7                 	pcmpeqb	xmm0, xmm7
1400d784f: 66 44 0f d7 c8              	pmovmskb	r9d, xmm0
1400d7854: 45 85 c9                    	test	r9d, r9d
1400d7857: 0f 85 99 03 00 00           	jne	0x1400d7bf6 <.text+0xd6bf6>
1400d785d: 4c 01 c2                    	add	rdx, r8
1400d7860: 48 83 c2 10                 	add	rdx, 0x10
1400d7864: 49 83 c0 10                 	add	r8, 0x10
1400d7868: 48 21 ca                    	and	rdx, rcx
1400d786b: eb c7                       	jmp	0x1400d7834 <.text+0xd6834>
1400d786d: 0f 1f 00                    	nop	dword ptr [rax]
1400d7870: 45 8d 51 ff                 	lea	r10d, [r9 - 0x1]
1400d7874: 66 45 21 ca                 	and	r10w, r9w
1400d7878: 45 89 d1                    	mov	r9d, r10d
1400d787b: 74 ce                       	je	0x1400d784b <.text+0xd684b>
1400d787d: f3 45 0f bc d9              	tzcnt	r11d, r9d
1400d7882: 49 01 d3                    	add	r11, rdx
1400d7885: 49 21 cb                    	and	r11, rcx
1400d7888: 4d 89 da                    	mov	r10, r11
1400d788b: 49 f7 da                    	neg	r10
1400d788e: 49 c1 e3 03                 	shl	r11, 0x3
1400d7892: 49 c1 e2 07                 	shl	r10, 0x7
1400d7896: 4d 29 da                    	sub	r10, r11
1400d7899: 4e 39 bc 10 78 ff ff ff     	cmp	qword ptr [rax + r10 - 0x88], r15
1400d78a1: 75 cd                       	jne	0x1400d7870 <.text+0xd6870>
1400d78a3: 4e 39 74 10 80              	cmp	qword ptr [rax + r10 - 0x80], r14
1400d78a8: 75 c6                       	jne	0x1400d7870 <.text+0xd6870>
1400d78aa: 4c 01 d0                    	add	rax, r10
1400d78ad: 4c 8b 60 f8                 	mov	r12, qword ptr [rax - 0x8]
1400d78b1: ba ff ff ff ff              	mov	edx, 0xffffffff
1400d78b6: f0                          	lock
1400d78b7: 0f c1 13                    	xadd	dword ptr [rbx], edx
1400d78ba: ff ca                       	dec	edx
1400d78bc: 89 d0                       	mov	eax, edx
1400d78be: 25 ff ff ff bf              	and	eax, 0xbfffffff
1400d78c3: f7 d8                       	neg	eax
1400d78c5: 70 7f                       	jo	0x1400d7946 <.text+0xd6946>
1400d78c7: 48 8b 85 a8 0f 00 00        	mov	rax, qword ptr [rbp + 0xfa8]
1400d78ce: 4c 39 a0 28 02 00 00        	cmp	qword ptr [rax + 0x228], r12
1400d78d5: 0f 86 31 03 00 00           	jbe	0x1400d7c0c <.text+0xd6c0c>
1400d78db: 48 8b 85 a8 0f 00 00        	mov	rax, qword ptr [rbp + 0xfa8]
1400d78e2: 48 8b 80 20 02 00 00        	mov	rax, qword ptr [rax + 0x220]
1400d78e9: 4a 8b 0c e0                 	mov	rcx, qword ptr [rax + 8*r12]
1400d78ed: 48 85 c9                    	test	rcx, rcx
1400d78f0: 0f 84 16 03 00 00           	je	0x1400d7c0c <.text+0xd6c0c>
1400d78f6: 48 8b 85 a8 0f 00 00        	mov	rax, qword ptr [rbp + 0xfa8]
1400d78fd: 48 8b 80 f0 01 00 00        	mov	rax, qword ptr [rax + 0x1f0]
1400d7904: 48 f7 d1                    	not	rcx
1400d7907: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
1400d790b: 48 c1 e1 05                 	shl	rcx, 0x5
1400d790f: 48 83 7c 08 20 00           	cmp	qword ptr [rax + rcx + 0x20], 0x0
1400d7915: 0f 84 f1 02 00 00           	je	0x1400d7c0c <.text+0xd6c0c>
1400d791b: 48 01 c8                    	add	rax, rcx
1400d791e: 48 8b 40 10                 	mov	rax, qword ptr [rax + 0x10]
1400d7922: 48 8b 48 30                 	mov	rcx, qword ptr [rax + 0x30]
1400d7926: 48 0b 48 10                 	or	rcx, qword ptr [rax + 0x10]
1400d792a: 0f 84 f0 fd ff ff           	je	0x1400d7720 <.text+0xd6720>
1400d7930: eb 28                       	jmp	0x1400d795a <.text+0xd695a>
1400d7932: c6 85 b7 0f 00 00 01        	mov	byte ptr [rbp + 0xfb7], 0x1
1400d7939: 48 89 d9                    	mov	rcx, rbx
1400d793c: e8 ef 10 80 02              	call	0x1428d8a30 <.text+0x28d7a30>
1400d7941: e9 b4 fe ff ff              	jmp	0x1400d77fa <.text+0xd67fa>
1400d7946: c6 85 b7 0f 00 00 01        	mov	byte ptr [rbp + 0xfb7], 0x1
1400d794d: 48 89 d9                    	mov	rcx, rbx
1400d7950: e8 eb 13 80 02              	call	0x1428d8d40 <.text+0x28d7d40>
1400d7955: e9 6d ff ff ff              	jmp	0x1400d78c7 <.text+0xd68c7>
1400d795a: 48 8b 1d 47 4a 55 03        	mov	rbx, qword ptr [rip + 0x3554a47] # 0x14362c3a8
1400d7961: 48 8b 03                    	mov	rax, qword ptr [rbx]
1400d7964: 48 83 f8 03                 	cmp	rax, 0x3
1400d7968: 0f 82 c8 00 00 00           	jb	0x1400d7a36 <.text+0xd6a36>
1400d796e: c6 85 b7 0f 00 00 01        	mov	byte ptr [rbp + 0xfb7], 0x1
1400d7975: 48 8d 0d 8c 2c 86 02        	lea	rcx, [rip + 0x2862c8c]  # 0x14293a608
1400d797c: e8 ff 63 5a 00              	call	0x14067dd80 <.text+0x67cd80>
1400d7981: f3 0f 6f 00                 	movdqu	xmm0, xmmword ptr [rax]
1400d7985: 8b 40 10                    	mov	eax, dword ptr [rax + 0x10]
1400d7988: 48 c7 85 d0 08 00 00 03 00 00 00    	mov	qword ptr [rbp + 0x8d0], 0x3
1400d7993: 48 8d 0d 86 2c 86 02        	lea	rcx, [rip + 0x2862c86]  # 0x14293a620
1400d799a: 48 89 8d d8 08 00 00        	mov	qword ptr [rbp + 0x8d8], rcx
1400d79a1: 48 c7 85 e0 08 00 00 14 00 00 00    	mov	qword ptr [rbp + 0x8e0], 0x14
1400d79ac: 48 8d 15 45 2c 86 02        	lea	rdx, [rip + 0x2862c45]  # 0x14293a5f8
1400d79b3: 48 89 95 f0 08 00 00        	mov	qword ptr [rbp + 0x8f0], rdx
1400d79ba: 48 c7 85 f8 08 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x8f8], 0x1
1400d79c5: 48 c7 85 00 09 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x900], 0x8
1400d79d0: 48 c7 85 a0 08 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x8a0], 0x0
1400d79db: 66 0f ef c9                 	pxor	xmm1, xmm1
1400d79df: f3 0f 7f 8d 08 09 00 00     	movdqu	xmmword ptr [rbp + 0x908], xmm1
1400d79e7: 48 89 8d a8 08 00 00        	mov	qword ptr [rbp + 0x8a8], rcx
1400d79ee: 48 c7 85 b0 08 00 00 14 00 00 00    	mov	qword ptr [rbp + 0x8b0], 0x14
1400d79f9: 48 c7 85 b8 08 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x8b8], 0x0
1400d7a04: f3 0f 7f 85 c0 08 00 00     	movdqu	xmmword ptr [rbp + 0x8c0], xmm0
1400d7a0c: c7 85 e8 08 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x8e8], 0x1
1400d7a16: 89 85 ec 08 00 00           	mov	dword ptr [rbp + 0x8ec], eax
1400d7a1c: c6 85 b7 0f 00 00 01        	mov	byte ptr [rbp + 0xfb7], 0x1
1400d7a23: 48 8d 8d 70 02 00 00        	lea	rcx, [rbp + 0x270]
1400d7a2a: 48 8d 95 a0 08 00 00        	lea	rdx, [rbp + 0x8a0]
1400d7a31: e8 fa 28 c7 00              	call	0x140d4a330 <.text+0xd49330>
1400d7a36: c6 85 b7 0f 00 00 01        	mov	byte ptr [rbp + 0xfb7], 0x1
1400d7a3d: 66 0f ef c0                 	pxor	xmm0, xmm0
1400d7a41: f3 0f 7f 44 24 28           	movdqu	xmmword ptr [rsp + 0x28], xmm0
1400d7a47: 48 89 74 24 20              	mov	qword ptr [rsp + 0x20], rsi
1400d7a4c: c6 44 24 38 01              	mov	byte ptr [rsp + 0x38], 0x1
1400d7a51: 48 b9 20 00 00 00 20 00 00 00       	movabs	rcx, 0x2000000020
1400d7a5b: 4c 8d 4d c0                 	lea	r9, [rbp - 0x40]
1400d7a5f: 48 89 fa                    	mov	rdx, rdi
1400d7a62: 45 31 c0                    	xor	r8d, r8d
1400d7a65: e8 a6 05 41 00              	call	0x1404e8010 <.text+0x4e7010>
1400d7a6a: 48 8b 03                    	mov	rax, qword ptr [rbx]
1400d7a6d: 48 83 f8 03                 	cmp	rax, 0x3
1400d7a71: 0f 82 c8 00 00 00           	jb	0x1400d7b3f <.text+0xd6b3f>
1400d7a77: c6 85 b7 0f 00 00 01        	mov	byte ptr [rbp + 0xfb7], 0x1
1400d7a7e: 48 8d 0d f3 2b 86 02        	lea	rcx, [rip + 0x2862bf3]  # 0x14293a678
1400d7a85: e8 f6 62 5a 00              	call	0x14067dd80 <.text+0x67cd80>
1400d7a8a: f3 0f 6f 00                 	movdqu	xmm0, xmmword ptr [rax]
1400d7a8e: 8b 40 10                    	mov	eax, dword ptr [rax + 0x10]
1400d7a91: 48 c7 85 d0 08 00 00 03 00 00 00    	mov	qword ptr [rbp + 0x8d0], 0x3
1400d7a9c: 48 8d 0d 7d 2b 86 02        	lea	rcx, [rip + 0x2862b7d]  # 0x14293a620
1400d7aa3: 48 89 8d d8 08 00 00        	mov	qword ptr [rbp + 0x8d8], rcx
1400d7aaa: 48 c7 85 e0 08 00 00 14 00 00 00    	mov	qword ptr [rbp + 0x8e0], 0x14
1400d7ab5: 48 8d 15 ac 2b 86 02        	lea	rdx, [rip + 0x2862bac]  # 0x14293a668
1400d7abc: 48 89 95 f0 08 00 00        	mov	qword ptr [rbp + 0x8f0], rdx
1400d7ac3: 48 c7 85 f8 08 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x8f8], 0x1
1400d7ace: 48 c7 85 00 09 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x900], 0x8
1400d7ad9: 48 c7 85 a0 08 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x8a0], 0x0
1400d7ae4: 66 0f ef c9                 	pxor	xmm1, xmm1
1400d7ae8: f3 0f 7f 8d 08 09 00 00     	movdqu	xmmword ptr [rbp + 0x908], xmm1
1400d7af0: 48 89 8d a8 08 00 00        	mov	qword ptr [rbp + 0x8a8], rcx
1400d7af7: 48 c7 85 b0 08 00 00 14 00 00 00    	mov	qword ptr [rbp + 0x8b0], 0x14
1400d7b02: 48 c7 85 b8 08 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x8b8], 0x0
1400d7b0d: f3 0f 7f 85 c0 08 00 00     	movdqu	xmmword ptr [rbp + 0x8c0], xmm0
1400d7b15: c7 85 e8 08 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x8e8], 0x1
1400d7b1f: 89 85 ec 08 00 00           	mov	dword ptr [rbp + 0x8ec], eax
1400d7b25: c6 85 b7 0f 00 00 01        	mov	byte ptr [rbp + 0xfb7], 0x1
1400d7b2c: 48 8d 8d 70 02 00 00        	lea	rcx, [rbp + 0x270]
1400d7b33: 48 8d 95 a0 08 00 00        	lea	rdx, [rbp + 0x8a0]
1400d7b3a: e8 f1 27 c7 00              	call	0x140d4a330 <.text+0xd49330>
1400d7b3f: 48 8d b5 a0 08 00 00        	lea	rsi, [rbp + 0x8a0]
1400d7b46: 41 b8 28 06 00 00           	mov	r8d, 0x628
1400d7b4c: 48 89 f1                    	mov	rcx, rsi
1400d7b4f: 48 89 fa                    	mov	rdx, rdi
1400d7b52: e8 a9 c0 84 02              	call	0x142923c00 <.text+0x2922c00>
1400d7b57: c6 85 b7 0f 00 00 00        	mov	byte ptr [rbp + 0xfb7], 0x0
1400d7b5e: 48 8d 8d 70 02 00 00        	lea	rcx, [rbp + 0x270]
1400d7b65: 48 8b 95 a8 0f 00 00        	mov	rdx, qword ptr [rbp + 0xfa8]
1400d7b6c: 49 89 f0                    	mov	r8, rsi
1400d7b6f: e8 8c e2 2e 00              	call	0x1403c5e00 <.text+0x3c4e00>
1400d7b74: 31 c0                       	xor	eax, eax
1400d7b76: 48 3b 85 70 02 00 00        	cmp	rax, qword ptr [rbp + 0x270]
1400d7b7d: 70 13                       	jo	0x1400d7b92 <.text+0xd6b92>
1400d7b7f: c6 85 b7 0f 00 00 00        	mov	byte ptr [rbp + 0xfb7], 0x0
1400d7b86: 48 8d 8d 70 02 00 00        	lea	rcx, [rbp + 0x270]
1400d7b8d: e8 ce 0f 51 00              	call	0x1405e8b60 <.text+0x5e7b60>
1400d7b92: c6 85 b6 0f 00 00 00        	mov	byte ptr [rbp + 0xfb6], 0x0
1400d7b99: 48 8d 8d d0 0e 00 00        	lea	rcx, [rbp + 0xed0]
1400d7ba0: e8 8b fa 51 00              	call	0x1405f7630 <.text+0x5f6630>
1400d7ba5: c6 85 b5 0f 00 00 00        	mov	byte ptr [rbp + 0xfb5], 0x0
1400d7bac: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
1400d7bb0: e8 fb 96 51 00              	call	0x1405f12b0 <.text+0x5f02b0>
1400d7bb5: 48 8b 8d a0 0f 00 00        	mov	rcx, qword ptr [rbp + 0xfa0]
1400d7bbc: 48 8b 01                    	mov	rax, qword ptr [rcx]
1400d7bbf: f0                          	lock
1400d7bc0: 48 ff 08                    	dec	qword ptr [rax]
1400d7bc3: 75 05                       	jne	0x1400d7bca <.text+0xd6bca>
1400d7bc5: e8 c6 42 a8 00              	call	0x140b5be90 <.text+0xb5ae90>
1400d7bca: 48 8b 8d a8 0f 00 00        	mov	rcx, qword ptr [rbp + 0xfa8]
1400d7bd1: e8 8a a1 52 00              	call	0x140601d60 <.text+0x600d60>
1400d7bd6: 0f 28 b5 c0 0f 00 00        	movaps	xmm6, xmmword ptr [rbp + 0xfc0]
1400d7bdd: 0f 28 bd d0 0f 00 00        	movaps	xmm7, xmmword ptr [rbp + 0xfd0]
1400d7be4: 48 81 c4 60 10 00 00        	add	rsp, 0x1060
1400d7beb: 5b                          	pop	rbx
1400d7bec: 5f                          	pop	rdi
1400d7bed: 5e                          	pop	rsi
1400d7bee: 41 5c                       	pop	r12
1400d7bf0: 41 5e                       	pop	r14
1400d7bf2: 41 5f                       	pop	r15
1400d7bf4: 5d                          	pop	rbp
1400d7bf5: c3                          	ret
1400d7bf6: ba ff ff ff ff              	mov	edx, 0xffffffff
1400d7bfb: f0                          	lock
1400d7bfc: 0f c1 13                    	xadd	dword ptr [rbx], edx
1400d7bff: ff ca                       	dec	edx
1400d7c01: 89 d0                       	mov	eax, edx
1400d7c03: 25 ff ff ff bf              	and	eax, 0xbfffffff
1400d7c08: f7 d8                       	neg	eax
1400d7c0a: 70 15                       	jo	0x1400d7c21 <.text+0xd6c21>
1400d7c0c: c6 85 b7 0f 00 00 01        	mov	byte ptr [rbp + 0xfb7], 0x1
1400d7c13: 48 8d 0d 8e 29 86 02        	lea	rcx, [rip + 0x286298e]  # 0x14293a5a8
1400d7c1a: e8 f1 16 80 02              	call	0x1428d9310 <.text+0x28d8310>
