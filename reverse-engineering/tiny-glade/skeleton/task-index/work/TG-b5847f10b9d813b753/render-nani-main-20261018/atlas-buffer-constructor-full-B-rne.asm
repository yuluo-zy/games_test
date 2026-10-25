
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141616c00: 55                          	push	rbp
141616c01: 41 57                       	push	r15
141616c03: 41 56                       	push	r14
141616c05: 41 54                       	push	r12
141616c07: 56                          	push	rsi
141616c08: 57                          	push	rdi
141616c09: 53                          	push	rbx
141616c0a: 48 81 ec 20 01 00 00        	sub	rsp, 0x120
141616c11: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
141616c19: 48 c7 85 98 00 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x98], -0x2
141616c24: 48 89 d7                    	mov	rdi, rdx
141616c27: 48 89 ce                    	mov	rsi, rcx
141616c2a: 4d 85 c0                    	test	r8, r8
141616c2d: 74 18                       	je	0x141616c47 <.text+0x1615c47>
141616c2f: 4c 89 cb                    	mov	rbx, r9
141616c32: 49 81 f9 c1 06 00 00        	cmp	r9, 0x6c1
141616c39: 0f 83 b1 01 00 00           	jae	0x141616df0 <.text+0x1615df0>
141616c3f: 4d 89 c7                    	mov	r15, r8
141616c42: 45 31 e4                    	xor	r12d, r12d
141616c45: eb 7c                       	jmp	0x141616cc3 <.text+0x1615cc3>
141616c47: 0f b6 05 bb df 01 02        	movzx	eax, byte ptr [rip + 0x201dfbb] # 0x143634c09
141616c4e: b9 00 6c 00 00              	mov	ecx, 0x6c00
141616c53: ba 04 00 00 00              	mov	edx, 0x4
141616c58: e8 b3 cf ff fe              	call	0x140613c10 <.text+0x612c10>
141616c5d: 48 85 c0                    	test	rax, rax
141616c60: 0f 84 f4 01 00 00           	je	0x141616e5a <.text+0x1615e5a>
141616c66: 49 89 c7                    	mov	r15, rax
141616c69: b8 70 00 00 00              	mov	eax, 0x70
141616c6e: 0f 57 c0                    	xorps	xmm0, xmm0
141616c71: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
141616c80: 41 0f 11 44 07 90           	movups	xmmword ptr [r15 + rax - 0x70], xmm0
141616c86: 41 0f 11 44 07 a0           	movups	xmmword ptr [r15 + rax - 0x60], xmm0
141616c8c: 41 0f 11 44 07 b0           	movups	xmmword ptr [r15 + rax - 0x50], xmm0
141616c92: 41 0f 11 44 07 c0           	movups	xmmword ptr [r15 + rax - 0x40], xmm0
141616c98: 41 0f 11 44 07 d0           	movups	xmmword ptr [r15 + rax - 0x30], xmm0
141616c9e: 41 0f 11 44 07 e0           	movups	xmmword ptr [r15 + rax - 0x20], xmm0
141616ca4: 41 0f 11 44 07 f0           	movups	xmmword ptr [r15 + rax - 0x10], xmm0
141616caa: 41 0f 11 04 07              	movups	xmmword ptr [r15 + rax], xmm0
141616caf: 48 83 e8 80                 	sub	rax, -0x80
141616cb3: 48 3d 70 6c 00 00           	cmp	rax, 0x6c70
141616cb9: 75 c5                       	jne	0x141616c80 <.text+0x1615c80>
141616cbb: 41 b4 01                    	mov	r12b, 0x1
141616cbe: bb c0 06 00 00              	mov	ebx, 0x6c0
141616cc3: 4c 89 bd 88 00 00 00        	mov	qword ptr [rbp + 0x88], r15
141616cca: 44 88 a5 97 00 00 00        	mov	byte ptr [rbp + 0x97], r12b
141616cd1: 48 8d 4d 10                 	lea	rcx, [rbp + 0x10]
141616cd5: ba 00 6c 00 00              	mov	edx, 0x6c00
141616cda: 41 b8 22 00 00 00           	mov	r8d, 0x22
141616ce0: 41 b1 01                    	mov	r9b, 0x1
141616ce3: e8 f8 e0 5b ff              	call	0x140bd4de0 <.text+0xbd3de0>
141616ce8: 4c 89 bd 88 00 00 00        	mov	qword ptr [rbp + 0x88], r15
141616cef: 44 88 a5 97 00 00 00        	mov	byte ptr [rbp + 0x97], r12b
141616cf6: 4c 8d 75 68                 	lea	r14, [rbp + 0x68]
141616cfa: 48 8d 55 10                 	lea	rdx, [rbp + 0x10]
141616cfe: 41 b8 04 00 00 00           	mov	r8d, 0x4
141616d04: 4c 89 f1                    	mov	rcx, r14
141616d07: e8 14 e2 5b ff              	call	0x140bd4f20 <.text+0xbd3f20>
141616d0c: 48 c1 e3 04                 	shl	rbx, 0x4
141616d10: 4c 89 bd 88 00 00 00        	mov	qword ptr [rbp + 0x88], r15
141616d17: 44 88 a5 97 00 00 00        	mov	byte ptr [rbp + 0x97], r12b
141616d1e: 48 89 5c 24 30              	mov	qword ptr [rsp + 0x30], rbx
141616d23: 4c 89 7c 24 28              	mov	qword ptr [rsp + 0x28], r15
141616d28: 4c 89 74 24 20              	mov	qword ptr [rsp + 0x20], r14
141616d2d: 4c 8d 05 c4 8c 66 01        	lea	r8, [rip + 0x1668cc4]   # 0x142c7f9f8
141616d34: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
141616d38: 41 b9 11 00 00 00           	mov	r9d, 0x11
141616d3e: 48 89 fa                    	mov	rdx, rdi
141616d41: e8 7a c0 08 00              	call	0x1416a2dc0 <.text+0x16a1dc0>
141616d46: 83 7d c0 0b                 	cmp	dword ptr [rbp - 0x40], 0xb
141616d4a: 75 44                       	jne	0x141616d90 <.text+0x1615d90>
141616d4c: 48 8b 45 e8                 	mov	rax, qword ptr [rbp - 0x18]
141616d50: 48 89 46 20                 	mov	qword ptr [rsi + 0x20], rax
141616d54: 0f 10 45 c8                 	movups	xmm0, xmmword ptr [rbp - 0x38]
141616d58: 0f 10 4d d8                 	movups	xmm1, xmmword ptr [rbp - 0x28]
141616d5c: 0f 11 4e 10                 	movups	xmmword ptr [rsi + 0x10], xmm1
141616d60: 0f 11 06                    	movups	xmmword ptr [rsi], xmm0
141616d63: 45 84 e4                    	test	r12b, r12b
141616d66: 74 13                       	je	0x141616d7b <.text+0x1615d7b>
141616d68: ba 00 6c 00 00              	mov	edx, 0x6c00
141616d6d: 41 b8 04 00 00 00           	mov	r8d, 0x4
141616d73: 4c 89 f9                    	mov	rcx, r15
141616d76: e8 a5 ce ff fe              	call	0x140613c20 <.text+0x612c20>
141616d7b: 48 89 f0                    	mov	rax, rsi
141616d7e: 48 81 c4 20 01 00 00        	add	rsp, 0x120
141616d85: 5b                          	pop	rbx
141616d86: 5f                          	pop	rdi
141616d87: 5e                          	pop	rsi
141616d88: 41 5c                       	pop	r12
141616d8a: 41 5e                       	pop	r14
141616d8c: 41 5f                       	pop	r15
141616d8e: 5d                          	pop	rbp
141616d8f: c3                          	ret
141616d90: 44 89 a5 84 00 00 00        	mov	dword ptr [rbp + 0x84], r12d
141616d97: 4c 89 7d 78                 	mov	qword ptr [rbp + 0x78], r15
141616d9b: 0f 10 45 00                 	movups	xmm0, xmmword ptr [rbp]
141616d9f: 0f 29 45 50                 	movaps	xmmword ptr [rbp + 0x50], xmm0
141616da3: 0f 10 45 c0                 	movups	xmm0, xmmword ptr [rbp - 0x40]
141616da7: 0f 10 4d d0                 	movups	xmm1, xmmword ptr [rbp - 0x30]
141616dab: 0f 10 55 e0                 	movups	xmm2, xmmword ptr [rbp - 0x20]
141616daf: 0f 10 5d f0                 	movups	xmm3, xmmword ptr [rbp - 0x10]
141616db3: 0f 29 5d 40                 	movaps	xmmword ptr [rbp + 0x40], xmm3
141616db7: 0f 29 55 30                 	movaps	xmmword ptr [rbp + 0x30], xmm2
141616dbb: 0f 29 4d 20                 	movaps	xmmword ptr [rbp + 0x20], xmm1
141616dbf: 0f 29 45 10                 	movaps	xmmword ptr [rbp + 0x10], xmm0
141616dc3: 48 8d 05 de 93 66 01        	lea	rax, [rip + 0x16693de]  # 0x142c801a8
141616dca: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
141616dcf: 48 8d 0d 12 7d 66 01        	lea	rcx, [rip + 0x1667d12]  # 0x142c7eae8
141616dd6: 4c 8d 0d eb 7c 66 01        	lea	r9, [rip + 0x1667ceb]   # 0x142c7eac8
141616ddd: 4c 8d 45 10                 	lea	r8, [rbp + 0x10]
141616de1: ba 2b 00 00 00              	mov	edx, 0x2b
141616de6: e8 75 29 2c 01              	call	0x1428d9760 <.text+0x28d8760>
141616deb: e9 87 00 00 00              	jmp	0x141616e77 <.text+0x1615e77>
141616df0: 48 8d 05 9b 79 48 01        	lea	rax, [rip + 0x148799b]  # 0x142a9e792
141616df7: 48 89 45 c0                 	mov	qword ptr [rbp - 0x40], rax
141616dfb: 48 c7 45 c8 3d 00 00 00     	mov	qword ptr [rbp - 0x38], 0x3d
141616e03: 48 8d 45 c0                 	lea	rax, [rbp - 0x40]
141616e07: 48 89 45 68                 	mov	qword ptr [rbp + 0x68], rax
141616e0b: 48 8d 05 7e 30 af fe        	lea	rax, [rip - 0x150cf82]  # 0x140109e90 <.text+0x108e90>
141616e12: 48 89 45 70                 	mov	qword ptr [rbp + 0x70], rax
141616e16: 48 8d 05 ab 89 66 01        	lea	rax, [rip + 0x16689ab]  # 0x142c7f7c8
141616e1d: 48 89 45 10                 	mov	qword ptr [rbp + 0x10], rax
141616e21: 48 c7 45 18 01 00 00 00     	mov	qword ptr [rbp + 0x18], 0x1
141616e29: 48 c7 45 30 00 00 00 00     	mov	qword ptr [rbp + 0x30], 0x0
141616e31: 48 8d 45 68                 	lea	rax, [rbp + 0x68]
141616e35: 48 89 45 20                 	mov	qword ptr [rbp + 0x20], rax
141616e39: 48 c7 45 28 01 00 00 00     	mov	qword ptr [rbp + 0x28], 0x1
141616e41: c6 85 97 00 00 00 00        	mov	byte ptr [rbp + 0x97], 0x0
141616e48: 48 8d 15 89 89 66 01        	lea	rdx, [rip + 0x1668989]  # 0x142c7f7d8
141616e4f: 48 8d 4d 10                 	lea	rcx, [rbp + 0x10]
141616e53: e8 38 25 2c 01              	call	0x1428d9390 <.text+0x28d8390>
141616e58: eb 1d                       	jmp	0x141616e77 <.text+0x1615e77>
141616e5a: c6 85 97 00 00 00 00        	mov	byte ptr [rbp + 0x97], 0x0
141616e61: 4c 8d 05 48 7c 66 01        	lea	r8, [rip + 0x1667c48]   # 0x142c7eab0
141616e68: b9 04 00 00 00              	mov	ecx, 0x4
141616e6d: ba 00 6c 00 00              	mov	edx, 0x6c00
141616e72: e8 6c 21 2c 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
141616e77: 0f 0b                       	ud2
141616e79: 0f 1f 80 00 00 00 00        	nop	dword ptr [rax]
141616e80: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141616e85: 55                          	push	rbp
141616e86: 41 57                       	push	r15
141616e88: 41 56                       	push	r14
141616e8a: 41 54                       	push	r12
141616e8c: 56                          	push	rsi
141616e8d: 57                          	push	rdi
141616e8e: 53                          	push	rbx
141616e8f: 48 83 ec 40                 	sub	rsp, 0x40
141616e93: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
141616e9a: 48 8b 75 78                 	mov	rsi, qword ptr [rbp + 0x78]
141616e9e: 48 89 b5 88 00 00 00        	mov	qword ptr [rbp + 0x88], rsi
141616ea5: 8b bd 84 00 00 00           	mov	edi, dword ptr [rbp + 0x84]
141616eab: 40 88 bd 97 00 00 00        	mov	byte ptr [rbp + 0x97], dil
141616eb2: 48 8d 4d 10                 	lea	rcx, [rbp + 0x10]
141616eb6: e8 b5 7b 1b ff              	call	0x1407cea70 <.text+0x7cda70>
141616ebb: 48 89 b5 88 00 00 00        	mov	qword ptr [rbp + 0x88], rsi
141616ec2: 40 88 bd 97 00 00 00        	mov	byte ptr [rbp + 0x97], dil
141616ec9: 48 83 c4 40                 	add	rsp, 0x40
141616ecd: 5b                          	pop	rbx
141616ece: 5f                          	pop	rdi
141616ecf: 5e                          	pop	rsi
141616ed0: 41 5c                       	pop	r12
141616ed2: 41 5e                       	pop	r14
141616ed4: 41 5f                       	pop	r15
141616ed6: 5d                          	pop	rbp
141616ed7: c3                          	ret
141616ed8: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
141616ee0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141616ee5: 55                          	push	rbp
141616ee6: 41 57                       	push	r15
141616ee8: 41 56                       	push	r14
141616eea: 41 54                       	push	r12
141616eec: 56                          	push	rsi
141616eed: 57                          	push	rdi
141616eee: 53                          	push	rbx
141616eef: 48 83 ec 40                 	sub	rsp, 0x40
141616ef3: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
141616efa: f6 85 97 00 00 00 01        	test	byte ptr [rbp + 0x97], 0x1
141616f01: 74 17                       	je	0x141616f1a <.text+0x1615f1a>
141616f03: 48 8b 8d 88 00 00 00        	mov	rcx, qword ptr [rbp + 0x88]
141616f0a: ba 00 6c 00 00              	mov	edx, 0x6c00
141616f0f: 41 b8 04 00 00 00           	mov	r8d, 0x4
141616f15: e8 06 cd ff fe              	call	0x140613c20 <.text+0x612c20>
141616f1a: 90                          	nop
141616f1b: 48 83 c4 40                 	add	rsp, 0x40
141616f1f: 5b                          	pop	rbx
141616f20: 5f                          	pop	rdi
141616f21: 5e                          	pop	rsi
141616f22: 41 5c                       	pop	r12
141616f24: 41 5e                       	pop	r14
141616f26: 41 5f                       	pop	r15
141616f28: 5d                          	pop	rbp
141616f29: c3                          	ret
