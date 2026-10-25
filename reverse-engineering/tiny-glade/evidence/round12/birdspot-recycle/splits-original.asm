
D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140c90050: 55                          	push	rbp
140c90051: 41 57                       	push	r15
140c90053: 41 56                       	push	r14
140c90055: 41 55                       	push	r13
140c90057: 41 54                       	push	r12
140c90059: 56                          	push	rsi
140c9005a: 57                          	push	rdi
140c9005b: 53                          	push	rbx
140c9005c: 48 81 ec 98 00 00 00        	sub	rsp, 0x98
140c90063: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
140c9006b: 44 0f 29 4d 00              	movaps	xmmword ptr [rbp], xmm9
140c90070: 44 0f 29 45 f0              	movaps	xmmword ptr [rbp - 0x10], xmm8
140c90075: 0f 29 7d e0                 	movaps	xmmword ptr [rbp - 0x20], xmm7
140c90079: 0f 29 75 d0                 	movaps	xmmword ptr [rbp - 0x30], xmm6
140c9007d: 48 c7 45 c8 fe ff ff ff     	mov	qword ptr [rbp - 0x38], -0x2
140c90085: 4c 89 ce                    	mov	rsi, r9
140c90088: 48 83 f9 03                 	cmp	rcx, 0x3
140c9008c: 73 2f                       	jae	0x140c900bd <.text+0xc8f0bd>
140c9008e: 48 8b 06                    	mov	rax, qword ptr [rsi]
140c90091: 48 8b 56 10                 	mov	rdx, qword ptr [rsi + 0x10]
140c90095: 48 29 d0                    	sub	rax, rdx
140c90098: 48 83 f8 01                 	cmp	rax, 0x1
140c9009c: 0f 86 e8 01 00 00           	jbe	0x140c9028a <.text+0xc8f28a>
140c900a2: 48 8b 46 08                 	mov	rax, qword ptr [rsi + 0x8]
140c900a6: 48 83 c6 10                 	add	rsi, 0x10
140c900aa: 48 b9 00 00 00 00 00 00 80 3f       	movabs	rcx, 0x3f80000000000000
140c900b4: 48 89 0c 90                 	mov	qword ptr [rax + 4*rdx], rcx
140c900b8: e9 a0 01 00 00              	jmp	0x140c9025d <.text+0xc8f25d>
140c900bd: 4c 89 c7                    	mov	rdi, r8
140c900c0: 0f 28 f1                    	movaps	xmm6, xmm1
140c900c3: 48 89 cb                    	mov	rbx, rcx
140c900c6: 48 89 c8                    	mov	rax, rcx
140c900c9: 48 ff c8                    	dec	rax
140c900cc: 78 07                       	js	0x140c900d5 <.text+0xc8f0d5>
140c900ce: f3 48 0f 2a c0              	cvtsi2ss	xmm0, rax
140c900d3: eb 15                       	jmp	0x140c900ea <.text+0xc8f0ea>
140c900d5: 48 89 c1                    	mov	rcx, rax
140c900d8: 48 d1 e9                    	shr	rcx
140c900db: 83 e0 01                    	and	eax, 0x1
140c900de: 48 09 c8                    	or	rax, rcx
140c900e1: f3 48 0f 2a c0              	cvtsi2ss	xmm0, rax
140c900e6: f3 0f 58 c0                 	addss	xmm0, xmm0
140c900ea: 4c 8b b5 80 00 00 00        	mov	r14, qword ptr [rbp + 0x80]
140c900f1: f3 0f 10 3d 0b 58 c9 01     	movss	xmm7, dword ptr [rip + 0x1c9580b] # 0x142925904
140c900f9: 44 0f 28 c7                 	movaps	xmm8, xmm7
140c900fd: f3 44 0f 5e c0              	divss	xmm8, xmm0
140c90102: f3 44 0f 59 05 6d d7 e9 01  	mulss	xmm8, dword ptr [rip + 0x1e9d76d] # 0x142b2d878
140c9010b: 45 0f 28 c8                 	movaps	xmm9, xmm8
140c9010f: f3 44 0f 5d ce              	minss	xmm9, xmm6
140c90114: f3 0f c2 f6 03              	cmpunordss	xmm6, xmm6
140c90119: 4c 8b 7e 10                 	mov	r15, qword ptr [rsi + 0x10]
140c9011d: 4c 3b 3e                    	cmp	r15, qword ptr [rsi]
140c90120: 75 0b                       	jne	0x140c9012d <.text+0xc8f12d>
140c90122: 48 89 f1                    	mov	rcx, rsi
140c90125: 4c 89 f2                    	mov	rdx, r14
140c90128: e8 43 f7 ec ff              	call	0x140b5f870 <.text+0xb5e870>
140c9012d: 44 0f 54 c6                 	andps	xmm8, xmm6
140c90131: 41 0f 55 f1                 	andnps	xmm6, xmm9
140c90135: 48 8b 46 08                 	mov	rax, qword ptr [rsi + 0x8]
140c90139: 42 c7 04 b8 00 00 00 00     	mov	dword ptr [rax + 4*r15], 0x0
140c90141: 49 ff c7                    	inc	r15
140c90144: 4c 89 7e 10                 	mov	qword ptr [rsi + 0x10], r15
140c90148: 48 85 db                    	test	rbx, rbx
140c9014b: 78 0b                       	js	0x140c90158 <.text+0xc8f158>
140c9014d: 45 0f 57 c9                 	xorps	xmm9, xmm9
140c90151: f3 4c 0f 2a cb              	cvtsi2ss	xmm9, rbx
140c90156: eb 1c                       	jmp	0x140c90174 <.text+0xc8f174>
140c90158: 48 89 d8                    	mov	rax, rbx
140c9015b: 48 d1 e8                    	shr	rax
140c9015e: 89 d9                       	mov	ecx, ebx
140c90160: 83 e1 01                    	and	ecx, 0x1
140c90163: 48 09 c1                    	or	rcx, rax
140c90166: 45 0f 57 c9                 	xorps	xmm9, xmm9
140c9016a: f3 4c 0f 2a c9              	cvtsi2ss	xmm9, rcx
140c9016f: f3 45 0f 58 c9              	addss	xmm9, xmm9
140c90174: 41 0f 56 f0                 	orps	xmm6, xmm8
140c90178: f3 44 0f 58 0d 63 9f cc 01  	addss	xmm9, dword ptr [rip + 0x1cc9f63] # 0x14295a0e4
140c90181: 48 83 c3 fe                 	add	rbx, -0x2
140c90185: 48 8b 06                    	mov	rax, qword ptr [rsi]
140c90188: 4c 29 f8                    	sub	rax, r15
140c9018b: 48 39 d8                    	cmp	rax, rbx
140c9018e: 0f 82 1c 01 00 00           	jb	0x140c902b0 <.text+0xc8f2b0>
140c90194: 48 8d 46 10                 	lea	rax, [rsi + 0x10]
140c90198: 48 89 45 b8                 	mov	qword ptr [rbp - 0x48], rax
140c9019c: f3 41 0f 5e f9              	divss	xmm7, xmm9
140c901a1: 4c 89 7d b0                 	mov	qword ptr [rbp - 0x50], r15
140c901a5: 4e 8d 3c bd 00 00 00 00     	lea	r15, [4*r15]
140c901ad: 4c 8b 66 08                 	mov	r12, qword ptr [rsi + 0x8]
140c901b1: 4d 01 fc                    	add	r12, r15
140c901b4: 48 c7 45 c0 00 00 00 00     	mov	qword ptr [rbp - 0x40], 0x0
140c901bc: f3 44 0f 10 05 23 9f cc 01  	movss	xmm8, dword ptr [rip + 0x1cc9f23] # 0x14295a0e8
140c901c5: eb 35                       	jmp	0x140c901fc <.text+0xc8f1fc>
140c901c7: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
140c901d0: 0f 57 c9                    	xorps	xmm1, xmm1
140c901d3: f3 49 0f 2a cd              	cvtsi2ss	xmm1, r13
140c901d8: f3 0f 59 cf                 	mulss	xmm1, xmm7
140c901dc: f3 41 0f 58 c0              	addss	xmm0, xmm8
140c901e1: f3 0f 59 c6                 	mulss	xmm0, xmm6
140c901e5: f3 0f 58 c1                 	addss	xmm0, xmm1
140c901e9: 48 8b 45 c0                 	mov	rax, qword ptr [rbp - 0x40]
140c901ed: f3 41 0f 11 04 84           	movss	dword ptr [r12 + 4*rax], xmm0
140c901f3: 4c 89 6d c0                 	mov	qword ptr [rbp - 0x40], r13
140c901f7: 4c 39 eb                    	cmp	rbx, r13
140c901fa: 74 2e                       	je	0x140c9022a <.text+0xc8f22a>
140c901fc: 48 89 f9                    	mov	rcx, rdi
140c901ff: e8 7c f1 01 00              	call	0x140caf380 <.text+0xcae380>
140c90204: 4c 8b 6d c0                 	mov	r13, qword ptr [rbp - 0x40]
140c90208: 49 ff c5                    	inc	r13
140c9020b: 79 c3                       	jns	0x140c901d0 <.text+0xc8f1d0>
140c9020d: 4c 89 e8                    	mov	rax, r13
140c90210: 48 d1 e8                    	shr	rax
140c90213: 44 89 e9                    	mov	ecx, r13d
140c90216: 83 e1 01                    	and	ecx, 0x1
140c90219: 48 09 c1                    	or	rcx, rax
140c9021c: 0f 57 c9                    	xorps	xmm1, xmm1
140c9021f: f3 48 0f 2a c9              	cvtsi2ss	xmm1, rcx
140c90224: f3 0f 58 c9                 	addss	xmm1, xmm1
140c90228: eb ae                       	jmp	0x140c901d8 <.text+0xc8f1d8>
140c9022a: 48 8b 7d b0                 	mov	rdi, qword ptr [rbp - 0x50]
140c9022e: 4a 8d 04 2f                 	lea	rax, [rdi + r13]
140c90232: 48 89 46 10                 	mov	qword ptr [rsi + 0x10], rax
140c90236: 48 3b 06                    	cmp	rax, qword ptr [rsi]
140c90239: 75 0b                       	jne	0x140c90246 <.text+0xc8f246>
140c9023b: 48 89 f1                    	mov	rcx, rsi
140c9023e: 4c 89 f2                    	mov	rdx, r14
140c90241: e8 2a f6 ec ff              	call	0x140b5f870 <.text+0xb5e870>
140c90246: 4a 8d 14 2f                 	lea	rdx, [rdi + r13]
140c9024a: 48 ff ca                    	dec	rdx
140c9024d: 4c 03 7e 08                 	add	r15, qword ptr [rsi + 0x8]
140c90251: 43 c7 04 af 00 00 80 3f     	mov	dword ptr [r15 + 4*r13], 0x3f800000
140c90259: 48 8b 75 b8                 	mov	rsi, qword ptr [rbp - 0x48]
140c9025d: 48 83 c2 02                 	add	rdx, 0x2
140c90261: 48 89 16                    	mov	qword ptr [rsi], rdx
140c90264: 0f 28 75 d0                 	movaps	xmm6, xmmword ptr [rbp - 0x30]
140c90268: 0f 28 7d e0                 	movaps	xmm7, xmmword ptr [rbp - 0x20]
140c9026c: 44 0f 28 45 f0              	movaps	xmm8, xmmword ptr [rbp - 0x10]
140c90271: 44 0f 28 4d 00              	movaps	xmm9, xmmword ptr [rbp]
140c90276: 48 81 c4 98 00 00 00        	add	rsp, 0x98
140c9027d: 5b                          	pop	rbx
140c9027e: 5f                          	pop	rdi
140c9027f: 5e                          	pop	rsi
140c90280: 41 5c                       	pop	r12
140c90282: 41 5d                       	pop	r13
140c90284: 41 5e                       	pop	r14
140c90286: 41 5f                       	pop	r15
140c90288: 5d                          	pop	rbp
140c90289: c3                          	ret
140c9028a: 48 c7 44 24 20 04 00 00 00  	mov	qword ptr [rsp + 0x20], 0x4
140c90293: 41 b8 02 00 00 00           	mov	r8d, 0x2
140c90299: 41 b9 04 00 00 00           	mov	r9d, 0x4
140c9029f: 48 89 f1                    	mov	rcx, rsi
140c902a2: e8 f9 be c6 01              	call	0x1428fc1a0 <.text+0x28fb1a0>
140c902a7: 48 8b 56 10                 	mov	rdx, qword ptr [rsi + 0x10]
140c902ab: e9 f2 fd ff ff              	jmp	0x140c900a2 <.text+0xc8f0a2>
140c902b0: 48 c7 44 24 20 04 00 00 00  	mov	qword ptr [rsp + 0x20], 0x4
140c902b9: 41 b9 04 00 00 00           	mov	r9d, 0x4
140c902bf: 48 89 f1                    	mov	rcx, rsi
140c902c2: 4c 89 fa                    	mov	rdx, r15
140c902c5: 49 89 d8                    	mov	r8, rbx
140c902c8: e8 d3 be c6 01              	call	0x1428fc1a0 <.text+0x28fb1a0>
140c902cd: 4c 8b 7e 10                 	mov	r15, qword ptr [rsi + 0x10]
140c902d1: e9 be fe ff ff              	jmp	0x140c90194 <.text+0xc8f194>
140c902d6: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
140c902e0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140c902e5: 55                          	push	rbp
140c902e6: 41 57                       	push	r15
140c902e8: 41 56                       	push	r14
140c902ea: 41 55                       	push	r13
140c902ec: 41 54                       	push	r12
140c902ee: 56                          	push	rsi
140c902ef: 57                          	push	rdi
140c902f0: 53                          	push	rbx
140c902f1: 48 83 ec 68                 	sub	rsp, 0x68
140c902f5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140c902fc: 44 0f 29 4c 24 20           	movaps	xmmword ptr [rsp + 0x20], xmm9
140c90302: 44 0f 29 44 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm8
140c90308: 0f 29 7c 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm7
140c9030d: 0f 29 74 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm6
140c90312: 48 8b 45 b0                 	mov	rax, qword ptr [rbp - 0x50]
140c90316: 48 03 45 c0                 	add	rax, qword ptr [rbp - 0x40]
140c9031a: 48 8b 4d b8                 	mov	rcx, qword ptr [rbp - 0x48]
140c9031e: 48 89 01                    	mov	qword ptr [rcx], rax
140c90321: 0f 28 74 24 50              	movaps	xmm6, xmmword ptr [rsp + 0x50]
140c90326: 0f 28 7c 24 40              	movaps	xmm7, xmmword ptr [rsp + 0x40]
140c9032b: 44 0f 28 44 24 30           	movaps	xmm8, xmmword ptr [rsp + 0x30]
140c90331: 44 0f 28 4c 24 20           	movaps	xmm9, xmmword ptr [rsp + 0x20]
140c90337: 48 83 c4 68                 	add	rsp, 0x68
140c9033b: 5b                          	pop	rbx
140c9033c: 5f                          	pop	rdi
140c9033d: 5e                          	pop	rsi
140c9033e: 41 5c                       	pop	r12
140c90340: 41 5d                       	pop	r13
140c90342: 41 5e                       	pop	r14
140c90344: 41 5f                       	pop	r15
140c90346: 5d                          	pop	rbp
140c90347: c3                          	ret
140c90348: cc                          	int3
140c90349: cc                          	int3
140c9034a: cc                          	int3
140c9034b: cc                          	int3
140c9034c: cc                          	int3
140c9034d: cc                          	int3
140c9034e: cc                          	int3
140c9034f: cc                          	int3
140c90350: 55                          	push	rbp
140c90351: 41 57                       	push	r15
140c90353: 41 56                       	push	r14
140c90355: 41 54                       	push	r12
140c90357: 56                          	push	rsi
140c90358: 57                          	push	rdi
140c90359: 53                          	push	rbx
140c9035a: 48 83 ec 60                 	sub	rsp, 0x60
140c9035e: 48 8d 6c 24 60              	lea	rbp, [rsp + 0x60]
140c90363: 0f 29 75 f0                 	movaps	xmmword ptr [rbp - 0x10], xmm6
140c90367: 48 c7 45 e8 fe ff ff ff     	mov	qword ptr [rbp - 0x18], -0x2
140c9036f: 48 8b 7d 60                 	mov	rdi, qword ptr [rbp + 0x60]
140c90373: 48 83 fa 01                 	cmp	rdx, 0x1
140c90377: 0f 86 a2 00 00 00           	jbe	0x140c9041f <.text+0xc8f41f>
140c9037d: 49 89 d6                    	mov	r14, rdx
140c90380: 4c 8d 3c 95 00 00 00 00     	lea	r15, [4*rdx]
140c90388: 45 31 e4                    	xor	r12d, r12d
140c9038b: 48 89 d0                    	mov	rax, rdx
140c9038e: 48 c1 e8 3e                 	shr	rax, 0x3e
140c90392: 0f 85 9b 00 00 00           	jne	0x140c90433 <.text+0xc8f433>
140c90398: 48 b8 fc ff ff ff ff ff ff 7f       	movabs	rax, 0x7ffffffffffffffc
140c903a2: 49 39 c7                    	cmp	r15, rax
140c903a5: 0f 87 88 00 00 00           	ja	0x140c90433 <.text+0xc8f433>
140c903ab: 4c 89 cb                    	mov	rbx, r9
140c903ae: 0f 28 f2                    	movaps	xmm6, xmm2
140c903b1: 48 89 ce                    	mov	rsi, rcx
140c903b4: 0f b6 05 4e 48 9a 02        	movzx	eax, byte ptr [rip + 0x29a484e] # 0x143634c09
140c903bb: 41 bc 04 00 00 00           	mov	r12d, 0x4
140c903c1: ba 04 00 00 00              	mov	edx, 0x4
140c903c6: 4c 89 f9                    	mov	rcx, r15
140c903c9: e8 42 38 98 ff              	call	0x140613c10 <.text+0x612c10>
140c903ce: 48 85 c0                    	test	rax, rax
140c903d1: 74 60                       	je	0x140c90433 <.text+0xc8f433>
140c903d3: 4c 89 75 d0                 	mov	qword ptr [rbp - 0x30], r14
140c903d7: 48 89 45 d8                 	mov	qword ptr [rbp - 0x28], rax
140c903db: 48 c7 45 e0 00 00 00 00     	mov	qword ptr [rbp - 0x20], 0x0
140c903e3: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
140c903e8: 4c 8d 4d d0                 	lea	r9, [rbp - 0x30]
140c903ec: 4c 89 f1                    	mov	rcx, r14
140c903ef: 0f 28 ce                    	movaps	xmm1, xmm6
140c903f2: 49 89 d8                    	mov	r8, rbx
140c903f5: e8 56 fc ff ff              	call	0x140c90050 <.text+0xc8f050>
140c903fa: 48 8b 45 e0                 	mov	rax, qword ptr [rbp - 0x20]
140c903fe: 48 89 46 10                 	mov	qword ptr [rsi + 0x10], rax
140c90402: 0f 10 45 d0                 	movups	xmm0, xmmword ptr [rbp - 0x30]
140c90406: 0f 11 06                    	movups	xmmword ptr [rsi], xmm0
140c90409: 48 89 f0                    	mov	rax, rsi
140c9040c: 0f 28 75 f0                 	movaps	xmm6, xmmword ptr [rbp - 0x10]
140c90410: 48 83 c4 60                 	add	rsp, 0x60
140c90414: 5b                          	pop	rbx
140c90415: 5f                          	pop	rdi
140c90416: 5e                          	pop	rsi
140c90417: 41 5c                       	pop	r12
140c90419: 41 5e                       	pop	r14
140c9041b: 41 5f                       	pop	r15
140c9041d: 5d                          	pop	rbp
140c9041e: c3                          	ret
140c9041f: 48 8d 0d 5a d4 e9 01        	lea	rcx, [rip + 0x1e9d45a]  # 0x142b2d880
140c90426: ba 1d 00 00 00              	mov	edx, 0x1d
140c9042b: 49 89 f8                    	mov	r8, rdi
140c9042e: e8 fd 8f c4 01              	call	0x1428d9430 <.text+0x28d8430>
140c90433: 4c 89 e1                    	mov	rcx, r12
140c90436: 4c 89 fa                    	mov	rdx, r15
140c90439: 49 89 f8                    	mov	r8, rdi
140c9043c: e8 a2 8b c4 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
140c90441: cc                          	int3
140c90442: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nop	word ptr cs:[rax + rax]
140c90450: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140c90455: 55                          	push	rbp
140c90456: 41 57                       	push	r15
140c90458: 41 56                       	push	r14
140c9045a: 41 54                       	push	r12
140c9045c: 56                          	push	rsi
140c9045d: 57                          	push	rdi
140c9045e: 53                          	push	rbx
140c9045f: 48 83 ec 40                 	sub	rsp, 0x40
140c90463: 48 8d 6a 60                 	lea	rbp, [rdx + 0x60]
140c90467: 0f 29 74 24 20              	movaps	xmmword ptr [rsp + 0x20], xmm6
140c9046c: 48 8b 55 d0                 	mov	rdx, qword ptr [rbp - 0x30]
140c90470: 48 85 d2                    	test	rdx, rdx
140c90473: 74 13                       	je	0x140c90488 <.text+0xc8f488>
140c90475: 48 8b 4d d8                 	mov	rcx, qword ptr [rbp - 0x28]
140c90479: 48 c1 e2 02                 	shl	rdx, 0x2
140c9047d: 41 b8 04 00 00 00           	mov	r8d, 0x4
140c90483: e8 98 37 98 ff              	call	0x140613c20 <.text+0x612c20>
140c90488: 0f 28 74 24 20              	movaps	xmm6, xmmword ptr [rsp + 0x20]
140c9048d: 48 83 c4 40                 	add	rsp, 0x40
140c90491: 5b                          	pop	rbx
140c90492: 5f                          	pop	rdi
140c90493: 5e                          	pop	rsi
140c90494: 41 5c                       	pop	r12
140c90496: 41 5e                       	pop	r14
140c90498: 41 5f                       	pop	r15
140c9049a: 5d                          	pop	rbp
140c9049b: c3                          	ret
140c9049c: cc                          	int3
140c9049d: cc                          	int3
140c9049e: cc                          	int3
140c9049f: cc                          	int3
