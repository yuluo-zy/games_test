
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1409bf320: 55                          	push	rbp
1409bf321: 41 57                       	push	r15
1409bf323: 41 56                       	push	r14
1409bf325: 41 55                       	push	r13
1409bf327: 41 54                       	push	r12
1409bf329: 56                          	push	rsi
1409bf32a: 57                          	push	rdi
1409bf32b: 53                          	push	rbx
1409bf32c: 48 81 ec f8 09 00 00        	sub	rsp, 0x9f8
1409bf333: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
1409bf33b: 48 c7 85 70 09 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x970], -0x2
1409bf346: 4c 89 8d 50 09 00 00        	mov	qword ptr [rbp + 0x950], r9
1409bf34d: 4c 89 c6                    	mov	rsi, r8
1409bf350: 49 89 d6                    	mov	r14, rdx
1409bf353: 48 89 cf                    	mov	rdi, rcx
1409bf356: 48 8b 9d e0 09 00 00        	mov	rbx, qword ptr [rbp + 0x9e0]
1409bf35d: 0f 57 c0                    	xorps	xmm0, xmm0
1409bf360: 0f 11 85 b4 03 00 00        	movups	xmmword ptr [rbp + 0x3b4], xmm0
1409bf367: 48 8d 4b 04                 	lea	rcx, [rbx + 0x4]
1409bf36b: e8 20 64 21 00              	call	0x140bd5790 <.text+0xbd4790>
1409bf370: c7 85 b0 03 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x3b0], 0x1
1409bf37a: 48 89 b5 c8 03 00 00        	mov	qword ptr [rbp + 0x3c8], rsi
1409bf381: 48 c7 85 98 07 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x798], 0x1
1409bf38c: 48 89 9d a0 07 00 00        	mov	qword ptr [rbp + 0x7a0], rbx
1409bf393: 48 c7 85 a8 07 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x7a8], 0x0
1409bf39e: 48 89 7c 24 40              	mov	qword ptr [rsp + 0x40], rdi
1409bf3a3: 4c 89 74 24 38              	mov	qword ptr [rsp + 0x38], r14
1409bf3a8: 48 89 44 24 30              	mov	qword ptr [rsp + 0x30], rax
1409bf3ad: 48 8d 85 98 07 00 00        	lea	rax, [rbp + 0x798]
1409bf3b4: 48 89 44 24 28              	mov	qword ptr [rsp + 0x28], rax
1409bf3b9: 48 c7 44 24 20 01 00 00 00  	mov	qword ptr [rsp + 0x20], 0x1
1409bf3c2: 48 8d 15 12 08 10 02        	lea	rdx, [rip + 0x2100812]  # 0x142abfbdb
1409bf3c9: 48 8d 4d c8                 	lea	rcx, [rbp - 0x38]
1409bf3cd: 48 8d 9d b0 03 00 00        	lea	rbx, [rbp + 0x3b0]
1409bf3d4: 41 b8 16 00 00 00           	mov	r8d, 0x16
1409bf3da: 49 89 d9                    	mov	r9, rbx
1409bf3dd: e8 8e 85 ea ff              	call	0x140867970 <.text+0x866970>
1409bf3e2: 4c 89 b5 60 09 00 00        	mov	qword ptr [rbp + 0x960], r14
1409bf3e9: 49 8d 4e 38                 	lea	rcx, [r14 + 0x38]
1409bf3ed: c6 85 6f 09 00 00 01        	mov	byte ptr [rbp + 0x96f], 0x1
1409bf3f4: 48 ba 29 f7 80 5b 19 62 c3 4b       	movabs	rdx, 0x4bc362195b80f729
1409bf3fe: 49 b8 fb ce cb 80 ce 98 27 98       	movabs	r8, -0x67d867317f343105
1409bf408: e8 e3 e2 71 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1409bf40d: 48 85 c0                    	test	rax, rax
1409bf410: 48 8b 8d 60 09 00 00        	mov	rcx, qword ptr [rbp + 0x960]
1409bf417: 0f 84 29 03 00 00           	je	0x1409bf746 <.text+0x9be746>
1409bf41d: 48 39 91 18 02 00 00        	cmp	qword ptr [rcx + 0x218], rdx
1409bf424: 0f 86 1c 03 00 00           	jbe	0x1409bf746 <.text+0x9be746>
1409bf42a: 48 8b 81 10 02 00 00        	mov	rax, qword ptr [rcx + 0x210]
1409bf431: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1409bf435: 48 85 c0                    	test	rax, rax
1409bf438: 0f 84 08 03 00 00           	je	0x1409bf746 <.text+0x9be746>
1409bf43e: 48 8b 89 e0 01 00 00        	mov	rcx, qword ptr [rcx + 0x1e0]
1409bf445: 48 f7 d0                    	not	rax
1409bf448: 48 8d 14 40                 	lea	rdx, [rax + 2*rax]
1409bf44c: 48 c1 e2 05                 	shl	rdx, 0x5
1409bf450: 48 83 7c 11 20 00           	cmp	qword ptr [rcx + rdx + 0x20], 0x0
1409bf456: 0f 84 ea 02 00 00           	je	0x1409bf746 <.text+0x9be746>
1409bf45c: 48 8b 85 e8 09 00 00        	mov	rax, qword ptr [rbp + 0x9e8]
1409bf463: 48 01 d1                    	add	rcx, rdx
1409bf466: 48 8b 49 10                 	mov	rcx, qword ptr [rcx + 0x10]
1409bf46a: 48 8b 11                    	mov	rdx, qword ptr [rcx]
1409bf46d: 48 85 d2                    	test	rdx, rdx
1409bf470: 78 0a                       	js	0x1409bf47c <.text+0x9be47c>
1409bf472: 0f 57 c0                    	xorps	xmm0, xmm0
1409bf475: f3 48 0f 2a c2              	cvtsi2ss	xmm0, rdx
1409bf47a: eb 18                       	jmp	0x1409bf494 <.text+0x9be494>
1409bf47c: 49 89 d0                    	mov	r8, rdx
1409bf47f: 49 d1 e8                    	shr	r8
1409bf482: 83 e2 01                    	and	edx, 0x1
1409bf485: 4c 09 c2                    	or	rdx, r8
1409bf488: 0f 57 c0                    	xorps	xmm0, xmm0
1409bf48b: f3 48 0f 2a c2              	cvtsi2ss	xmm0, rdx
1409bf490: f3 0f 58 c0                 	addss	xmm0, xmm0
1409bf494: f3 0f 2a 49 08              	cvtsi2ss	xmm1, dword ptr [rcx + 0x8]
1409bf499: f3 0f 5e 0d 9f 81 fe 01     	divss	xmm1, dword ptr [rip + 0x1fe819f] # 0x1429a7640
1409bf4a1: f3 0f 58 c8                 	addss	xmm1, xmm0
1409bf4a5: f3 0f 11 8d 58 09 00 00     	movss	dword ptr [rbp + 0x958], xmm1
1409bf4ad: c6 85 5c 09 00 00 00        	mov	byte ptr [rbp + 0x95c], 0x0
1409bf4b4: 4c 8b b0 58 01 00 00        	mov	r14, qword ptr [rax + 0x158]
1409bf4bb: 48 8b 80 60 01 00 00        	mov	rax, qword ptr [rax + 0x160]
1409bf4c2: 48 c1 e0 07                 	shl	rax, 0x7
1409bf4c6: 4c 01 f0                    	add	rax, r14
1409bf4c9: 48 89 85 48 09 00 00        	mov	qword ptr [rbp + 0x948], rax
1409bf4d0: 4c 8d bd a0 07 00 00        	lea	r15, [rbp + 0x7a0]
1409bf4d7: 4c 8d a5 b8 03 00 00        	lea	r12, [rbp + 0x3b8]
1409bf4de: 4c 3b b5 48 09 00 00        	cmp	r14, qword ptr [rbp + 0x948]
1409bf4e5: 0f 84 09 02 00 00           	je	0x1409bf6f4 <.text+0x9be6f4>
1409bf4eb: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
1409bf4f0: 4d 8b 6e 50                 	mov	r13, qword ptr [r14 + 0x50]
1409bf4f4: 49 8b 76 58                 	mov	rsi, qword ptr [r14 + 0x58]
1409bf4f8: c6 85 6f 09 00 00 01        	mov	byte ptr [rbp + 0x96f], 0x1
1409bf4ff: e8 4c 72 22 00              	call	0x140be6750 <.text+0xbe5750>
1409bf504: 89 c7                       	mov	edi, eax
1409bf506: c6 85 6f 09 00 00 01        	mov	byte ptr [rbp + 0x96f], 0x1
1409bf50d: 31 c9                       	xor	ecx, ecx
1409bf50f: e8 0c a6 71 ff              	call	0x1400d9b20 <.text+0xd8b20>
1409bf514: 48 85 c0                    	test	rax, rax
1409bf517: 0f 84 14 02 00 00           	je	0x1409bf731 <.text+0x9be731>
1409bf51d: 89 bd 38 09 00 00           	mov	dword ptr [rbp + 0x938], edi
1409bf523: 66 c7 85 40 09 00 00 00 00  	mov	word ptr [rbp + 0x940], 0x0
1409bf52c: c7 85 3c 09 00 00 00 00 80 3f       	mov	dword ptr [rbp + 0x93c], 0x3f800000
1409bf536: 48 c7 85 90 08 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x890], 0x1
1409bf541: 4c 89 ad 98 08 00 00        	mov	qword ptr [rbp + 0x898], r13
1409bf548: 48 89 b5 30 09 00 00        	mov	qword ptr [rbp + 0x930], rsi
1409bf54f: 48 c7 85 a0 08 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x8a0], 0x0
1409bf55a: 48 c7 85 a8 08 00 00 04 00 00 00    	mov	qword ptr [rbp + 0x8a8], 0x4
1409bf565: 48 c7 85 b0 08 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x8b0], 0x0
1409bf570: 0f 10 00                    	movups	xmm0, xmmword ptr [rax]
1409bf573: 48 ff 00                    	inc	qword ptr [rax]
1409bf576: 0f 10 0d b3 71 0d 02        	movups	xmm1, xmmword ptr [rip + 0x20d71b3] # 0x142a96730
1409bf57d: 48 8d 85 00 09 00 00        	lea	rax, [rbp + 0x900]
1409bf584: 0f 11 48 10                 	movups	xmmword ptr [rax + 0x10], xmm1
1409bf588: 0f 10 0d 91 71 0d 02        	movups	xmm1, xmmword ptr [rip + 0x20d7191] # 0x142a96720
1409bf58f: 0f 11 08                    	movups	xmmword ptr [rax], xmm1
1409bf592: 0f 11 85 20 09 00 00        	movups	xmmword ptr [rbp + 0x920], xmm0
1409bf599: 48 b8 03 00 00 00 01 00 00 00       	movabs	rax, 0x100000003
1409bf5a3: 48 89 85 d0 08 00 00        	mov	qword ptr [rbp + 0x8d0], rax
1409bf5aa: 48 c7 85 b8 08 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x8b8], 0x0
1409bf5b5: 48 c7 85 c0 08 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x8c0], 0x1
1409bf5c0: 48 c7 85 c8 08 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x8c8], 0x0
1409bf5cb: c6 85 6f 09 00 00 01        	mov	byte ptr [rbp + 0x96f], 0x1
1409bf5d2: 48 89 d9                    	mov	rcx, rbx
1409bf5d5: 48 8d 95 90 08 00 00        	lea	rdx, [rbp + 0x890]
1409bf5dc: 4c 8d 45 c8                 	lea	r8, [rbp - 0x38]
1409bf5e0: 45 31 c9                    	xor	r9d, r9d
1409bf5e3: e8 f8 65 f4 ff              	call	0x140905be0 <.text+0x904be0>
1409bf5e8: c6 85 6f 09 00 00 01        	mov	byte ptr [rbp + 0x96f], 0x1
1409bf5ef: 48 8d 8d 98 07 00 00        	lea	rcx, [rbp + 0x798]
1409bf5f6: 48 89 da                    	mov	rdx, rbx
1409bf5f9: 4c 8d 85 58 09 00 00        	lea	r8, [rbp + 0x958]
1409bf600: 4d 89 f1                    	mov	r9, r14
1409bf603: e8 58 be f4 ff              	call	0x14090b460 <.text+0x90a460>
1409bf608: 48 8b 85 50 09 00 00        	mov	rax, qword ptr [rbp + 0x950]
1409bf60f: 48 89 85 90 08 00 00        	mov	qword ptr [rbp + 0x890], rax
1409bf616: c7 85 b0 03 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x3b0], 0x2
1409bf620: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
1409bf625: 48 8d 8d 90 08 00 00        	lea	rcx, [rbp + 0x890]
1409bf62c: 48 8d 95 98 07 00 00        	lea	rdx, [rbp + 0x798]
1409bf633: 49 b8 dc 1d 1c 95 13 ea 8b d4       	movabs	r8, -0x2b7415ec6ae3e224
1409bf63d: 41 b1 1d                    	mov	r9b, 0x1d
1409bf640: e8 bb f1 f2 ff              	call	0x1408ee800 <.text+0x8ed800>
1409bf645: 48 8b b5 98 07 00 00        	mov	rsi, qword ptr [rbp + 0x798]
1409bf64c: 41 b8 a0 00 00 00           	mov	r8d, 0xa0
1409bf652: 4c 89 e1                    	mov	rcx, r12
1409bf655: 4c 89 fa                    	mov	rdx, r15
1409bf658: e8 a3 45 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409bf65d: 0f b6 85 40 08 00 00        	movzx	eax, byte ptr [rbp + 0x840]
1409bf664: 41 0f 10 87 a1 00 00 00     	movups	xmm0, xmmword ptr [r15 + 0xa1]
1409bf66c: 41 0f 10 8f b1 00 00 00     	movups	xmm1, xmmword ptr [r15 + 0xb1]
1409bf674: 41 0f 10 97 c1 00 00 00     	movups	xmm2, xmmword ptr [r15 + 0xc1]
1409bf67c: 41 0f 10 9f d1 00 00 00     	movups	xmm3, xmmword ptr [r15 + 0xd1]
1409bf684: 41 0f 11 84 24 a1 00 00 00  	movups	xmmword ptr [r12 + 0xa1], xmm0
1409bf68d: 41 0f 11 8c 24 b1 00 00 00  	movups	xmmword ptr [r12 + 0xb1], xmm1
1409bf696: 41 0f 11 94 24 c1 00 00 00  	movups	xmmword ptr [r12 + 0xc1], xmm2
1409bf69f: 41 0f 11 9c 24 d1 00 00 00  	movups	xmmword ptr [r12 + 0xd1], xmm3
1409bf6a8: 41 0f 10 87 e0 00 00 00     	movups	xmm0, xmmword ptr [r15 + 0xe0]
1409bf6b0: 41 0f 11 84 24 e0 00 00 00  	movups	xmmword ptr [r12 + 0xe0], xmm0
1409bf6b9: 24 bf                       	and	al, -0x41
1409bf6bb: 4d 8b 4e 70                 	mov	r9, qword ptr [r14 + 0x70]
1409bf6bf: 48 89 b5 b0 03 00 00        	mov	qword ptr [rbp + 0x3b0], rsi
1409bf6c6: 88 85 58 04 00 00           	mov	byte ptr [rbp + 0x458], al
1409bf6cc: c6 85 6f 09 00 00 01        	mov	byte ptr [rbp + 0x96f], 0x1
1409bf6d3: 48 89 d9                    	mov	rcx, rbx
1409bf6d6: 4c 89 f2                    	mov	rdx, r14
1409bf6d9: 4c 8b 85 60 09 00 00        	mov	r8, qword ptr [rbp + 0x960]
1409bf6e0: 41 ff d1                    	call	r9
1409bf6e3: 49 83 ee 80                 	sub	r14, -0x80
1409bf6e7: 4c 3b b5 48 09 00 00        	cmp	r14, qword ptr [rbp + 0x948]
1409bf6ee: 0f 85 fc fd ff ff           	jne	0x1409bf4f0 <.text+0x9be4f0>
1409bf6f4: 48 8d b5 b0 03 00 00        	lea	rsi, [rbp + 0x3b0]
1409bf6fb: 48 8d 55 c8                 	lea	rdx, [rbp - 0x38]
1409bf6ff: 41 b8 e8 03 00 00           	mov	r8d, 0x3e8
1409bf705: 48 89 f1                    	mov	rcx, rsi
1409bf708: e8 f3 44 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409bf70d: c6 85 6f 09 00 00 00        	mov	byte ptr [rbp + 0x96f], 0x0
1409bf714: 48 89 f1                    	mov	rcx, rsi
1409bf717: e8 54 8c ea ff              	call	0x140868370 <.text+0x867370>
1409bf71c: 90                          	nop
1409bf71d: 48 81 c4 f8 09 00 00        	add	rsp, 0x9f8
1409bf724: 5b                          	pop	rbx
1409bf725: 5f                          	pop	rdi
1409bf726: 5e                          	pop	rsi
1409bf727: 41 5c                       	pop	r12
1409bf729: 41 5d                       	pop	r13
1409bf72b: 41 5e                       	pop	r14
1409bf72d: 41 5f                       	pop	r15
1409bf72f: 5d                          	pop	rbp
1409bf730: c3                          	ret
1409bf731: c6 85 6f 09 00 00 01        	mov	byte ptr [rbp + 0x96f], 0x1
1409bf738: 48 8d 0d 61 b3 27 02        	lea	rcx, [rip + 0x227b361]  # 0x142c3aaa0
1409bf73f: e8 8c 89 f1 01              	call	0x1428d80d0 <.text+0x28d70d0>
1409bf744: eb 13                       	jmp	0x1409bf759 <.text+0x9be759>
1409bf746: c6 85 6f 09 00 00 01        	mov	byte ptr [rbp + 0x96f], 0x1
1409bf74d: 48 8d 0d 84 64 0e 02        	lea	rcx, [rip + 0x20e6484]  # 0x142aa5bd8
1409bf754: e8 b7 9b f1 01              	call	0x1428d9310 <.text+0x28d8310>
1409bf759: 0f 0b                       	ud2
1409bf75b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
1409bf760: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409bf765: 55                          	push	rbp
1409bf766: 41 57                       	push	r15
1409bf768: 41 56                       	push	r14
1409bf76a: 41 55                       	push	r13
1409bf76c: 41 54                       	push	r12
1409bf76e: 56                          	push	rsi
1409bf76f: 57                          	push	rdi
1409bf770: 53                          	push	rbx
1409bf771: 48 83 ec 48                 	sub	rsp, 0x48
1409bf775: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409bf77c: 80 bd 6f 09 00 00 00        	cmp	byte ptr [rbp + 0x96f], 0x0
1409bf783: 75 11                       	jne	0x1409bf796 <.text+0x9be796>
1409bf785: 48 83 c4 48                 	add	rsp, 0x48
1409bf789: 5b                          	pop	rbx
1409bf78a: 5f                          	pop	rdi
1409bf78b: 5e                          	pop	rsi
1409bf78c: 41 5c                       	pop	r12
1409bf78e: 41 5d                       	pop	r13
1409bf790: 41 5e                       	pop	r14
1409bf792: 41 5f                       	pop	r15
1409bf794: 5d                          	pop	rbp
1409bf795: c3                          	ret
1409bf796: 48 8d 4d c8                 	lea	rcx, [rbp - 0x38]
1409bf79a: e8 21 2c ed ff              	call	0x1408923c0 <.text+0x8913c0>
1409bf79f: eb e4                       	jmp	0x1409bf785 <.text+0x9be785>
1409bf7a1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
1409bf7b0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409bf7b5: 55                          	push	rbp
1409bf7b6: 41 57                       	push	r15
1409bf7b8: 41 56                       	push	r14
1409bf7ba: 41 55                       	push	r13
1409bf7bc: 41 54                       	push	r12
1409bf7be: 56                          	push	rsi
1409bf7bf: 57                          	push	rdi
1409bf7c0: 53                          	push	rbx
1409bf7c1: 48 83 ec 48                 	sub	rsp, 0x48
1409bf7c5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409bf7cc: c6 85 6f 09 00 00 01        	mov	byte ptr [rbp + 0x96f], 0x1
1409bf7d3: 48 8d 8d 98 07 00 00        	lea	rcx, [rbp + 0x798]
1409bf7da: e8 e1 3d ed ff              	call	0x1408935c0 <.text+0x8925c0>
1409bf7df: c6 85 6f 09 00 00 01        	mov	byte ptr [rbp + 0x96f], 0x1
1409bf7e6: 48 83 c4 48                 	add	rsp, 0x48
1409bf7ea: 5b                          	pop	rbx
1409bf7eb: 5f                          	pop	rdi
1409bf7ec: 5e                          	pop	rsi
1409bf7ed: 41 5c                       	pop	r12
1409bf7ef: 41 5d                       	pop	r13
1409bf7f1: 41 5e                       	pop	r14
1409bf7f3: 41 5f                       	pop	r15
1409bf7f5: 5d                          	pop	rbp
1409bf7f6: c3                          	ret
