
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141691810: 55                          	push	rbp
141691811: 41 56                       	push	r14
141691813: 41 54                       	push	r12
141691815: 56                          	push	rsi
141691816: 57                          	push	rdi
141691817: 53                          	push	rbx
141691818: 48 83 ec 78                 	sub	rsp, 0x78
14169181c: 48 8d 6c 24 70              	lea	rbp, [rsp + 0x70]
141691821: 48 c7 45 00 fe ff ff ff     	mov	qword ptr [rbp], -0x2
141691829: 4c 89 c0                    	mov	rax, r8
14169182c: 48 89 ce                    	mov	rsi, rcx
14169182f: 4d 8b 40 08                 	mov	r8, qword ptr [r8 + 0x8]
141691833: 48 89 45 e8                 	mov	qword ptr [rbp - 0x18], rax
141691837: 48 8b 78 10                 	mov	rdi, qword ptr [rax + 0x10]
14169183b: c6 45 ff 01                 	mov	byte ptr [rbp - 0x1], 0x1
14169183f: 48 8d 4d b8                 	lea	rcx, [rbp - 0x48]
141691843: 4c 89 45 f0                 	mov	qword ptr [rbp - 0x10], r8
141691847: 49 89 f9                    	mov	r9, rdi
14169184a: e8 e1 56 f8 ff              	call	0x141616f30 <.text+0x1615f30>
14169184f: 48 8b 45 e8                 	mov	rax, qword ptr [rbp - 0x18]
141691853: 4c 8b 20                    	mov	r12, qword ptr [rax]
141691856: 49 39 fc                    	cmp	r12, rdi
141691859: 76 47                       	jbe	0x1416918a2 <.text+0x16908a2>
14169185b: 49 c1 e4 04                 	shl	r12, 0x4
14169185f: 48 85 ff                    	test	rdi, rdi
141691862: 48 8b 4d f0                 	mov	rcx, qword ptr [rbp - 0x10]
141691866: 74 40                       	je	0x1416918a8 <.text+0x16908a8>
141691868: 49 89 fe                    	mov	r14, rdi
14169186b: 49 c1 e6 04                 	shl	r14, 0x4
14169186f: 41 b8 04 00 00 00           	mov	r8d, 0x4
141691875: 4c 89 e2                    	mov	rdx, r12
141691878: 4d 89 f1                    	mov	r9, r14
14169187b: e8 b0 23 f8 fe              	call	0x140613c30 <.text+0x612c30>
141691880: 48 89 c3                    	mov	rbx, rax
141691883: 48 85 c0                    	test	rax, rax
141691886: 75 33                       	jne	0x1416918bb <.text+0x16908bb>
141691888: 4c 89 65 e0                 	mov	qword ptr [rbp - 0x20], r12
14169188c: 4c 8d 05 5d 4f 5f 01        	lea	r8, [rip + 0x15f4f5d]   # 0x142c867f0
141691893: b9 04 00 00 00              	mov	ecx, 0x4
141691898: 4c 89 f2                    	mov	rdx, r14
14169189b: e8 43 77 24 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
1416918a0: 0f 0b                       	ud2
1416918a2: 48 8b 5d f0                 	mov	rbx, qword ptr [rbp - 0x10]
1416918a6: eb 13                       	jmp	0x1416918bb <.text+0x16908bb>
1416918a8: bb 04 00 00 00              	mov	ebx, 0x4
1416918ad: 41 b8 04 00 00 00           	mov	r8d, 0x4
1416918b3: 4c 89 e2                    	mov	rdx, r12
1416918b6: e8 65 23 f8 fe              	call	0x140613c20 <.text+0x612c20>
1416918bb: 48 8b 45 d8                 	mov	rax, qword ptr [rbp - 0x28]
1416918bf: 48 89 46 40                 	mov	qword ptr [rsi + 0x40], rax
1416918c3: 0f 10 45 b8                 	movups	xmm0, xmmword ptr [rbp - 0x48]
1416918c7: 0f 10 4d c8                 	movups	xmm1, xmmword ptr [rbp - 0x38]
1416918cb: 0f 11 4e 30                 	movups	xmmword ptr [rsi + 0x30], xmm1
1416918cf: 0f 11 46 20                 	movups	xmmword ptr [rsi + 0x20], xmm0
1416918d3: 48 89 5e 10                 	mov	qword ptr [rsi + 0x10], rbx
1416918d7: 48 89 7e 18                 	mov	qword ptr [rsi + 0x18], rdi
1416918db: 48 c7 06 ff ff ff ff        	mov	qword ptr [rsi], -0x1
1416918e2: 48 c7 46 08 00 00 00 00     	mov	qword ptr [rsi + 0x8], 0x0
1416918ea: 48 89 f0                    	mov	rax, rsi
1416918ed: 48 83 c4 78                 	add	rsp, 0x78
1416918f1: 5b                          	pop	rbx
1416918f2: 5f                          	pop	rdi
1416918f3: 5e                          	pop	rsi
1416918f4: 41 5c                       	pop	r12
1416918f6: 41 5e                       	pop	r14
1416918f8: 5d                          	pop	rbp
1416918f9: c3                          	ret
1416918fa: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
141691900: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141691905: 55                          	push	rbp
141691906: 41 56                       	push	r14
141691908: 41 54                       	push	r12
14169190a: 56                          	push	rsi
14169190b: 57                          	push	rdi
14169190c: 53                          	push	rbx
14169190d: 48 83 ec 28                 	sub	rsp, 0x28
141691911: 48 8d 6a 70                 	lea	rbp, [rdx + 0x70]
141691915: 41 b8 04 00 00 00           	mov	r8d, 0x4
14169191b: 48 8b 4d f0                 	mov	rcx, qword ptr [rbp - 0x10]
14169191f: 48 8b 55 e0                 	mov	rdx, qword ptr [rbp - 0x20]
141691923: e8 f8 22 f8 fe              	call	0x140613c20 <.text+0x612c20>
141691928: 48 8d 4d b8                 	lea	rcx, [rbp - 0x48]
14169192c: e8 7f 0b 55 ff              	call	0x140be24b0 <.text+0xbe14b0>
141691931: c6 45 ff 00                 	mov	byte ptr [rbp - 0x1], 0x0
141691935: 48 83 c4 28                 	add	rsp, 0x28
141691939: 5b                          	pop	rbx
14169193a: 5f                          	pop	rdi
14169193b: 5e                          	pop	rsi
14169193c: 41 5c                       	pop	r12
14169193e: 41 5e                       	pop	r14
141691940: 5d                          	pop	rbp
141691941: c3                          	ret
141691942: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nop	word ptr cs:[rax + rax]
141691950: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141691955: 55                          	push	rbp
141691956: 41 56                       	push	r14
141691958: 41 54                       	push	r12
14169195a: 56                          	push	rsi
14169195b: 57                          	push	rdi
14169195c: 53                          	push	rbx
14169195d: 48 83 ec 28                 	sub	rsp, 0x28
141691961: 48 8d 6a 70                 	lea	rbp, [rdx + 0x70]
141691965: 48 8b 45 e8                 	mov	rax, qword ptr [rbp - 0x18]
141691969: 80 7d ff 00                 	cmp	byte ptr [rbp - 0x1], 0x0
14169196d: 74 1b                       	je	0x14169198a <.text+0x169098a>
14169196f: 48 8b 10                    	mov	rdx, qword ptr [rax]
141691972: 48 85 d2                    	test	rdx, rdx
141691975: 74 13                       	je	0x14169198a <.text+0x169098a>
141691977: 48 c1 e2 04                 	shl	rdx, 0x4
14169197b: 41 b8 04 00 00 00           	mov	r8d, 0x4
141691981: 48 8b 4d f0                 	mov	rcx, qword ptr [rbp - 0x10]
141691985: e8 96 22 f8 fe              	call	0x140613c20 <.text+0x612c20>
14169198a: 90                          	nop
14169198b: 48 83 c4 28                 	add	rsp, 0x28
14169198f: 5b                          	pop	rbx
141691990: 5f                          	pop	rdi
141691991: 5e                          	pop	rsi
141691992: 41 5c                       	pop	r12
141691994: 41 5e                       	pop	r14
141691996: 5d                          	pop	rbp
141691997: c3                          	ret
