
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141d900d0: 41 57                       	push	r15
141d900d2: 41 56                       	push	r14
141d900d4: 41 55                       	push	r13
141d900d6: 41 54                       	push	r12
141d900d8: 56                          	push	rsi
141d900d9: 57                          	push	rdi
141d900da: 55                          	push	rbp
141d900db: 53                          	push	rbx
141d900dc: 48 83 ec 68                 	sub	rsp, 0x68
141d900e0: 44 0f 29 44 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm8
141d900e6: 0f 29 7c 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm7
141d900eb: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
141d900f0: 48 89 ce                    	mov	rsi, rcx
141d900f3: 49 8b 00                    	mov	rax, qword ptr [r8]
141d900f6: 4d 8b 78 08                 	mov	r15, qword ptr [r8 + 0x8]
141d900fa: 49 8b 48 10                 	mov	rcx, qword ptr [r8 + 0x10]
141d900fe: 4c 8b 32                    	mov	r14, qword ptr [rdx]
141d90101: 0f b6 09                    	movzx	ecx, byte ptr [rcx]
141d90104: 4c 8d 41 ff                 	lea	r8, [rcx - 0x1]
141d90108: 31 d2                       	xor	edx, edx
141d9010a: 48 83 f9 02                 	cmp	rcx, 0x2
141d9010e: 49 0f 43 d0                 	cmovae	rdx, r8
141d90112: 48 85 d2                    	test	rdx, rdx
141d90115: 74 11                       	je	0x141d90128 <.text+0x1d8f128>
141d90117: 48 83 fa 01                 	cmp	rdx, 0x1
141d9011b: 75 1c                       	jne	0x141d90139 <.text+0x1d8f139>
141d9011d: f3 44 0f 10 05 de 57 b9 00  	movss	xmm8, dword ptr [rip + 0xb957de] # 0x142925904
141d90126: eb 15                       	jmp	0x141d9013d <.text+0x1d8f13d>
141d90128: 80 e1 01                    	and	cl, 0x1
141d9012b: 0f b6 c9                    	movzx	ecx, cl
141d9012e: 45 0f 57 c0                 	xorps	xmm8, xmm8
141d90132: f3 44 0f 2a c1              	cvtsi2ss	xmm8, ecx
141d90137: eb 04                       	jmp	0x141d9013d <.text+0x1d8f13d>
141d90139: 45 0f 57 c0                 	xorps	xmm8, xmm8
141d9013d: f3 0f 10 70 10              	movss	xmm6, dword ptr [rax + 0x10]
141d90142: f3 0f 10 78 18              	movss	xmm7, dword ptr [rax + 0x18]
141d90147: 49 8b 06                    	mov	rax, qword ptr [r14]
141d9014a: 48 8b 38                    	mov	rdi, qword ptr [rax]
141d9014d: 4c 89 7c 24 28              	mov	qword ptr [rsp + 0x28], r15
141d90152: 48 8d 5f 20                 	lea	rbx, [rdi + 0x20]
141d90156: 48 8d 54 24 28              	lea	rdx, [rsp + 0x28]
141d9015b: 48 89 d9                    	mov	rcx, rbx
141d9015e: e8 0d 0c 82 fe              	call	0x1405b0d70 <.text+0x5afd70>
141d90163: 49 89 c4                    	mov	r12, rax
141d90166: 49 c1 ec 39                 	shr	r12, 0x39
141d9016a: 48 8b 0f                    	mov	rcx, qword ptr [rdi]
141d9016d: 48 8b 57 08                 	mov	rdx, qword ptr [rdi + 0x8]
141d90171: 49 89 d0                    	mov	r8, rdx
141d90174: 49 21 c0                    	and	r8, rax
141d90177: 66 41 0f 6e c4              	movd	xmm0, r12d
141d9017c: 66 0f 60 c0                 	punpcklbw	xmm0, xmm0      # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
141d90180: f2 0f 70 c0 00              	pshuflw	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0,4,5,6,7]
141d90185: 66 0f 70 c0 00              	pshufd	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0]
141d9018a: 4c 8d 49 e8                 	lea	r9, [rcx - 0x18]
141d9018e: 45 31 d2                    	xor	r10d, r10d
141d90191: 66 0f 76 c9                 	pcmpeqd	xmm1, xmm1
141d90195: 4d 89 c3                    	mov	r11, r8
141d90198: f3 42 0f 6f 14 19           	movdqu	xmm2, xmmword ptr [rcx + r11]
141d9019e: 66 0f 6f da                 	movdqa	xmm3, xmm2
141d901a2: 66 0f 74 d8                 	pcmpeqb	xmm3, xmm0
141d901a6: 66 44 0f d7 eb              	pmovmskb	r13d, xmm3
141d901ab: 45 85 ed                    	test	r13d, r13d
141d901ae: 74 30                       	je	0x141d901e0 <.text+0x1d8f1e0>
141d901b0: f3 41 0f bc ed              	tzcnt	ebp, r13d
141d901b5: 4c 01 dd                    	add	rbp, r11
141d901b8: 48 21 d5                    	and	rbp, rdx
141d901bb: 48 f7 dd                    	neg	rbp
141d901be: 48 8d 6c 6d 00              	lea	rbp, [rbp + 2*rbp]
141d901c3: 4d 39 3c e9                 	cmp	qword ptr [r9 + 8*rbp], r15
141d901c7: 74 33                       	je	0x141d901fc <.text+0x1d8f1fc>
141d901c9: 41 8d 6d ff                 	lea	ebp, [r13 - 0x1]
141d901cd: 66 44 21 ed                 	and	bp, r13w
141d901d1: 41 89 ed                    	mov	r13d, ebp
141d901d4: 75 da                       	jne	0x141d901b0 <.text+0x1d8f1b0>
141d901d6: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
141d901e0: 66 0f 74 d1                 	pcmpeqb	xmm2, xmm1
141d901e4: 66 0f d7 ea                 	pmovmskb	ebp, xmm2
141d901e8: 85 ed                       	test	ebp, ebp
141d901ea: 75 16                       	jne	0x141d90202 <.text+0x1d8f202>
141d901ec: 4d 01 d3                    	add	r11, r10
141d901ef: 49 83 c3 10                 	add	r11, 0x10
141d901f3: 49 83 c2 10                 	add	r10, 0x10
141d901f7: 49 21 d3                    	and	r11, rdx
141d901fa: eb 9c                       	jmp	0x141d90198 <.text+0x1d8f198>
141d901fc: 48 8d 1c e9                 	lea	rbx, [rcx + 8*rbp]
141d90200: eb 79                       	jmp	0x141d9027b <.text+0x1d8f27b>
141d90202: 48 83 7f 10 00              	cmp	qword ptr [rdi + 0x10], 0x0
141d90207: 0f 84 30 01 00 00           	je	0x141d9033d <.text+0x1d8f33d>
141d9020d: f3 42 0f 6f 04 01           	movdqu	xmm0, xmmword ptr [rcx + r8]
141d90213: 66 0f d7 c0                 	pmovmskb	eax, xmm0
141d90217: 85 c0                       	test	eax, eax
141d90219: 0f 84 58 01 00 00           	je	0x141d90377 <.text+0x1d8f377>
141d9021f: f3 0f bc c0                 	tzcnt	eax, eax
141d90223: 4c 01 c0                    	add	rax, r8
141d90226: 48 21 d0                    	and	rax, rdx
141d90229: 44 0f b6 04 01              	movzx	r8d, byte ptr [rcx + rax]
141d9022e: 45 84 c0                    	test	r8b, r8b
141d90231: 0f 89 64 01 00 00           	jns	0x141d9039b <.text+0x1d8f39b>
141d90237: 4c 8d 48 f0                 	lea	r9, [rax - 0x10]
141d9023b: 49 21 d1                    	and	r9, rdx
141d9023e: 44 88 24 01                 	mov	byte ptr [rcx + rax], r12b
141d90242: 46 88 64 09 10              	mov	byte ptr [rcx + r9 + 0x10], r12b
141d90247: 48 f7 d8                    	neg	rax
141d9024a: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
141d9024e: 41 80 e0 01                 	and	r8b, 0x1
141d90252: 41 0f b6 d0                 	movzx	edx, r8b
141d90256: 48 29 57 10                 	sub	qword ptr [rdi + 0x10], rdx
141d9025a: 48 8d 1c c1                 	lea	rbx, [rcx + 8*rax]
141d9025e: 4c 89 7c c1 e8              	mov	qword ptr [rcx + 8*rax - 0x18], r15
141d90263: f3 0f 11 74 c1 f0           	movss	dword ptr [rcx + 8*rax - 0x10], xmm6
141d90269: f3 0f 11 7c c1 f4           	movss	dword ptr [rcx + 8*rax - 0xc], xmm7
141d9026f: c7 44 c1 f8 00 00 00 00     	mov	dword ptr [rcx + 8*rax - 0x8], 0x0
141d90277: 48 ff 47 18                 	inc	qword ptr [rdi + 0x18]
141d9027b: 49 8b 46 08                 	mov	rax, qword ptr [r14 + 0x8]
141d9027f: f3 0f 10 00                 	movss	xmm0, dword ptr [rax]
141d90283: f3 0f 10 0d 79 56 b9 00     	movss	xmm1, dword ptr [rip + 0xb95679] # 0x142925904
141d9028b: 0f 28 d1                    	movaps	xmm2, xmm1
141d9028e: f3 0f 5c d0                 	subss	xmm2, xmm0
141d90292: f3 0f 59 53 f8              	mulss	xmm2, dword ptr [rbx - 0x8]
141d90297: f3 44 0f 59 c0              	mulss	xmm8, xmm0
141d9029c: f3 44 0f 58 c2              	addss	xmm8, xmm2
141d902a1: f3 44 0f 11 43 f8           	movss	dword ptr [rbx - 0x8], xmm8
141d902a7: 49 8b 46 10                 	mov	rax, qword ptr [r14 + 0x10]
141d902ab: f3 0f 10 00                 	movss	xmm0, dword ptr [rax]
141d902af: f3 0f 5c c8                 	subss	xmm1, xmm0
141d902b3: f2 0f 10 53 f0              	movsd	xmm2, qword ptr [rbx - 0x10]
141d902b8: f3 0f 12 c9                 	movsldup	xmm1, xmm1              # xmm1 = xmm1[0,0,2,2]
141d902bc: 0f 59 ca                    	mulps	xmm1, xmm2
141d902bf: 0f 28 d6                    	movaps	xmm2, xmm6
141d902c2: 0f 14 d7                    	unpcklps	xmm2, xmm7              # xmm2 = xmm2[0],xmm7[0],xmm2[1],xmm7[1]
141d902c5: f3 0f 12 c0                 	movsldup	xmm0, xmm0              # xmm0 = xmm0[0,0,2,2]
141d902c9: 0f 59 c2                    	mulps	xmm0, xmm2
141d902cc: 0f 58 c1                    	addps	xmm0, xmm1
141d902cf: 0f 13 43 f0                 	movlps	qword ptr [rbx - 0x10], xmm0
141d902d3: 31 c0                       	xor	eax, eax
141d902d5: f3 0f 10 05 83 cd c3 00     	movss	xmm0, dword ptr [rip + 0xc3cd83] # 0x1429cd060
141d902dd: 41 0f 2e c0                 	ucomiss	xmm0, xmm8
141d902e1: 73 34                       	jae	0x141d90317 <.text+0x1d8f317>
141d902e3: 0f 28 c6                    	movaps	xmm0, xmm6
141d902e6: 0f 28 cf                    	movaps	xmm1, xmm7
141d902e9: e8 02 0b f0 fe              	call	0x140c90df0 <.text+0xc8fdf0>
141d902ee: 89 c7                       	mov	edi, eax
141d902f0: f3 0f 10 43 f0              	movss	xmm0, dword ptr [rbx - 0x10]
141d902f5: f3 0f 10 4b f4              	movss	xmm1, dword ptr [rbx - 0xc]
141d902fa: e8 f1 0a f0 fe              	call	0x140c90df0 <.text+0xc8fdf0>
141d902ff: 8b 4b f8                    	mov	ecx, dword ptr [rbx - 0x8]
141d90302: 89 7e 04                    	mov	dword ptr [rsi + 0x4], edi
141d90305: 89 46 08                    	mov	dword ptr [rsi + 0x8], eax
141d90308: 89 4e 0c                    	mov	dword ptr [rsi + 0xc], ecx
141d9030b: c7 46 10 00 00 00 00        	mov	dword ptr [rsi + 0x10], 0x0
141d90312: b8 01 00 00 00              	mov	eax, 0x1
141d90317: 89 06                       	mov	dword ptr [rsi], eax
141d90319: 48 89 f0                    	mov	rax, rsi
141d9031c: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
141d90321: 0f 28 7c 24 40              	movaps	xmm7, xmmword ptr [rsp + 0x40]
141d90326: 44 0f 28 44 24 50           	movaps	xmm8, xmmword ptr [rsp + 0x50]
141d9032c: 48 83 c4 68                 	add	rsp, 0x68
141d90330: 5b                          	pop	rbx
141d90331: 5d                          	pop	rbp
141d90332: 5f                          	pop	rdi
141d90333: 5e                          	pop	rsi
141d90334: 41 5c                       	pop	r12
141d90336: 41 5d                       	pop	r13
141d90338: 41 5e                       	pop	r14
141d9033a: 41 5f                       	pop	r15
141d9033c: c3                          	ret
141d9033d: ba 01 00 00 00              	mov	edx, 0x1
141d90342: 48 89 f9                    	mov	rcx, rdi
141d90345: 49 89 d8                    	mov	r8, rbx
141d90348: 41 b1 01                    	mov	r9b, 0x1
141d9034b: 48 89 c3                    	mov	rbx, rax
141d9034e: e8 7d 19 b3 00              	call	0x1428c1cd0 <.text+0x28c0cd0>
141d90353: 4c 8b 7c 24 28              	mov	r15, qword ptr [rsp + 0x28]
141d90358: 48 8b 0f                    	mov	rcx, qword ptr [rdi]
141d9035b: 48 8b 57 08                 	mov	rdx, qword ptr [rdi + 0x8]
141d9035f: 48 21 d3                    	and	rbx, rdx
141d90362: 49 89 d8                    	mov	r8, rbx
141d90365: f3 42 0f 6f 04 01           	movdqu	xmm0, xmmword ptr [rcx + r8]
141d9036b: 66 0f d7 c0                 	pmovmskb	eax, xmm0
141d9036f: 85 c0                       	test	eax, eax
141d90371: 0f 85 a8 fe ff ff           	jne	0x141d9021f <.text+0x1d8f21f>
141d90377: 41 b9 10 00 00 00           	mov	r9d, 0x10
141d9037d: 4d 01 c8                    	add	r8, r9
141d90380: 49 21 d0                    	and	r8, rdx
141d90383: f3 42 0f 6f 04 01           	movdqu	xmm0, xmmword ptr [rcx + r8]
141d90389: 66 0f d7 c0                 	pmovmskb	eax, xmm0
141d9038d: 49 83 c1 10                 	add	r9, 0x10
141d90391: 85 c0                       	test	eax, eax
141d90393: 0f 85 86 fe ff ff           	jne	0x141d9021f <.text+0x1d8f21f>
141d90399: eb e2                       	jmp	0x141d9037d <.text+0x1d8f37d>
141d9039b: 66 0f 6f 01                 	movdqa	xmm0, xmmword ptr [rcx]
141d9039f: 66 0f d7 c0                 	pmovmskb	eax, xmm0
141d903a3: f3 0f bc c0                 	tzcnt	eax, eax
141d903a7: 44 0f b6 04 01              	movzx	r8d, byte ptr [rcx + rax]
141d903ac: e9 86 fe ff ff              	jmp	0x141d90237 <.text+0x1d8f237>
