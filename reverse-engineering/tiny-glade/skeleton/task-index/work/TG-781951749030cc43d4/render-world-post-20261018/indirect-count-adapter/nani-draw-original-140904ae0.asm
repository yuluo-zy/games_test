
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140904ae0: 55                          	push	rbp
140904ae1: 41 57                       	push	r15
140904ae3: 41 56                       	push	r14
140904ae5: 41 55                       	push	r13
140904ae7: 41 54                       	push	r12
140904ae9: 56                          	push	rsi
140904aea: 57                          	push	rdi
140904aeb: 53                          	push	rbx
140904aec: 48 81 ec 88 03 00 00        	sub	rsp, 0x388
140904af3: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
140904afb: 48 c7 85 00 03 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x300], -0x2
140904b06: 89 d6                       	mov	esi, edx
140904b08: 48 89 8d f0 02 00 00        	mov	qword ptr [rbp + 0x2f0], rcx
140904b0f: 48 8b 81 e0 00 00 00        	mov	rax, qword ptr [rcx + 0xe0]
140904b16: 4c 8b a0 f0 01 00 00        	mov	r12, qword ptr [rax + 0x1f0]
140904b1d: 4d 8d 74 24 38              	lea	r14, [r12 + 0x38]
140904b22: c6 85 ff 02 00 00 01        	mov	byte ptr [rbp + 0x2ff], 0x1
140904b29: 48 ba ef a5 99 86 45 78 29 66       	movabs	rdx, 0x662978458699a5ef
140904b33: 49 b8 46 9b 41 62 c1 57 c0 3e       	movabs	r8, 0x3ec057c162419b46
140904b3d: 4c 89 f1                    	mov	rcx, r14
140904b40: e8 ab 8b 7d ff              	call	0x1400dd6f0 <.text+0xdc6f0>
140904b45: 48 85 c0                    	test	rax, rax
140904b48: 0f 84 eb 03 00 00           	je	0x140904f39 <.text+0x903f39>
140904b4e: 49 39 94 24 18 02 00 00     	cmp	qword ptr [r12 + 0x218], rdx
140904b56: 0f 86 dd 03 00 00           	jbe	0x140904f39 <.text+0x903f39>
140904b5c: 49 8b 84 24 10 02 00 00     	mov	rax, qword ptr [r12 + 0x210]
140904b64: 48 8b 0c d0                 	mov	rcx, qword ptr [rax + 8*rdx]
140904b68: 48 85 c9                    	test	rcx, rcx
140904b6b: 0f 84 c8 03 00 00           	je	0x140904f39 <.text+0x903f39>
140904b71: 49 8b 84 24 e0 01 00 00     	mov	rax, qword ptr [r12 + 0x1e0]
140904b79: 48 f7 d1                    	not	rcx
140904b7c: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
140904b80: 48 c1 e1 05                 	shl	rcx, 0x5
140904b84: 48 83 7c 08 20 00           	cmp	qword ptr [rax + rcx + 0x20], 0x0
140904b8a: 0f 84 a9 03 00 00           	je	0x140904f39 <.text+0x903f39>
140904b90: 48 01 c8                    	add	rax, rcx
140904b93: 48 8b 8d f0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2f0]
140904b9a: 0f b6 89 f0 00 00 00        	movzx	ecx, byte ptr [rcx + 0xf0]
140904ba1: c0 e1 03                    	shl	cl, 0x3
140904ba4: 48 bb 00 00 01 02 03 00 00 00       	movabs	rbx, 0x302010000
140904bae: 48 d3 eb                    	shr	rbx, cl
140904bb1: 4c 8b 68 10                 	mov	r13, qword ptr [rax + 0x10]
140904bb5: 40 0f b6 c6                 	movzx	eax, sil
140904bb9: 48 89 85 d8 02 00 00        	mov	qword ptr [rbp + 0x2d8], rax
140904bc0: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
140904bc4: c1 e0 06                    	shl	eax, 0x6
140904bc7: 4e 8d 3c 28                 	lea	r15, [rax + r13]
140904bcb: 49 8b 44 05 00              	mov	rax, qword ptr [r13 + rax]
140904bd0: 89 c1                       	mov	ecx, eax
140904bd2: 83 e1 08                    	and	ecx, 0x8
140904bd5: c1 e9 03                    	shr	ecx, 0x3
140904bd8: 48 83 f8 f0                 	cmp	rax, -0x10
140904bdc: 0f 93 c2                    	setae	dl
140904bdf: 08 ca                       	or	dl, cl
140904be1: 74 19                       	je	0x140904bfc <.text+0x903bfc>
140904be3: c6 85 ff 02 00 00 01        	mov	byte ptr [rbp + 0x2ff], 0x1
140904bea: 4c 89 f9                    	mov	rcx, r15
140904bed: 31 d2                       	xor	edx, edx
140904bef: 41 b9 00 ca 9a 3b           	mov	r9d, 0x3b9aca00
140904bf5: e8 b6 78 fc 01              	call	0x1428cc4b0 <.text+0x28cb4b0>
140904bfa: eb 0b                       	jmp	0x140904c07 <.text+0x903c07>
140904bfc: 48 8d 48 10                 	lea	rcx, [rax + 0x10]
140904c00: f0                          	lock
140904c01: 49 0f b1 0f                 	cmpxchg	qword ptr [r15], rcx
140904c05: 75 dc                       	jne	0x140904be3 <.text+0x903be3>
140904c07: c6 85 ff 02 00 00 01        	mov	byte ptr [rbp + 0x2ff], 0x1
140904c0e: 4c 89 f9                    	mov	rcx, r15
140904c11: e8 5a d0 37 00              	call	0x140c81c70 <.text+0xc80c70>
140904c16: 4c 89 ff                    	mov	rdi, r15
140904c19: 48 83 cf 01                 	or	rdi, 0x1
140904c1d: c6 85 ff 02 00 00 01        	mov	byte ptr [rbp + 0x2ff], 0x1
140904c24: 48 89 f9                    	mov	rcx, rdi
140904c27: e8 44 d0 37 00              	call	0x140c81c70 <.text+0xc80c70>
140904c2c: 49 8b 47 08                 	mov	rax, qword ptr [r15 + 0x8]
140904c30: 48 b9 01 00 00 00 00 00 00 80       	movabs	rcx, -0x7fffffffffffffff
140904c3a: 48 39 c8                    	cmp	rax, rcx
140904c3d: 4c 89 bd e8 02 00 00        	mov	qword ptr [rbp + 0x2e8], r15
140904c44: 0f 84 19 03 00 00           	je	0x140904f63 <.text+0x903f63>
140904c4a: 49 8d 8f a8 00 00 00        	lea	rcx, [r15 + 0xa8]
140904c51: 49 8d 57 08                 	lea	rdx, [r15 + 0x8]
140904c55: 48 f7 d8                    	neg	rax
140904c58: 48 0f 40 ca                 	cmovo	rcx, rdx
140904c5c: 48 8b 41 08                 	mov	rax, qword ptr [rcx + 0x8]
140904c60: 48 89 85 e0 02 00 00        	mov	qword ptr [rbp + 0x2e0], rax
140904c67: 71 05                       	jno	0x140904c6e <.text+0x903c6e>
140904c69: 45 31 ff                    	xor	r15d, r15d
140904c6c: eb 1e                       	jmp	0x140904c8c <.text+0x903c8c>
140904c6e: 49 8b 8f 98 00 00 00        	mov	rcx, qword ptr [r15 + 0x98]
140904c75: 49 8b 87 a0 00 00 00        	mov	rax, qword ptr [r15 + 0xa0]
140904c7c: 48 8b 40 38                 	mov	rax, qword ptr [rax + 0x38]
140904c80: c6 85 fe 02 00 00 01        	mov	byte ptr [rbp + 0x2fe], 0x1
140904c87: ff d0                       	call	rax
140904c89: 49 89 c7                    	mov	r15, rax
140904c8c: 8d 04 b5 00 00 00 00        	lea	eax, [4*rsi]
140904c93: 08 d8                       	or	al, bl
140904c95: 0f b6 d8                    	movzx	ebx, al
140904c98: 49 8b 85 50 14 00 00        	mov	rax, qword ptr [r13 + 0x1450]
140904c9f: 48 8b 50 28                 	mov	rdx, qword ptr [rax + 0x28]
140904ca3: 48 39 da                    	cmp	rdx, rbx
140904ca6: 0f 86 c5 02 00 00           	jbe	0x140904f71 <.text+0x903f71>
140904cac: 48 8b 40 20                 	mov	rax, qword ptr [rax + 0x20]
140904cb0: c1 e3 02                    	shl	ebx, 0x2
140904cb3: 44 8b 2c 18                 	mov	r13d, dword ptr [rax + rbx]
140904cb7: c6 85 fe 02 00 00 01        	mov	byte ptr [rbp + 0x2fe], 0x1
140904cbe: 48 ba 55 93 67 6e 78 cc 26 82       	movabs	rdx, -0x7dd9338791986cab
140904cc8: 49 b8 ac 41 a8 55 6a da cf 65       	movabs	r8, 0x65cfda6a55a841ac
140904cd2: 4c 89 f1                    	mov	rcx, r14
140904cd5: e8 16 8a 7d ff              	call	0x1400dd6f0 <.text+0xdc6f0>
140904cda: 48 85 c0                    	test	rax, rax
140904cdd: 0f 84 6b 02 00 00           	je	0x140904f4e <.text+0x903f4e>
140904ce3: 49 39 94 24 18 02 00 00     	cmp	qword ptr [r12 + 0x218], rdx
140904ceb: 0f 86 5d 02 00 00           	jbe	0x140904f4e <.text+0x903f4e>
140904cf1: 49 8b 84 24 10 02 00 00     	mov	rax, qword ptr [r12 + 0x210]
140904cf9: 48 8b 0c d0                 	mov	rcx, qword ptr [rax + 8*rdx]
140904cfd: 48 85 c9                    	test	rcx, rcx
140904d00: 0f 84 48 02 00 00           	je	0x140904f4e <.text+0x903f4e>
140904d06: 49 8b 84 24 e0 01 00 00     	mov	rax, qword ptr [r12 + 0x1e0]
140904d0e: 48 f7 d1                    	not	rcx
140904d11: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
140904d15: 48 c1 e1 05                 	shl	rcx, 0x5
140904d19: 48 83 7c 08 20 00           	cmp	qword ptr [rax + rcx + 0x20], 0x0
140904d1f: 0f 84 29 02 00 00           	je	0x140904f4e <.text+0x903f4e>
140904d25: 48 01 c8                    	add	rax, rcx
140904d28: 4c 8b 60 10                 	mov	r12, qword ptr [rax + 0x10]
140904d2c: 48 8b 85 f0 02 00 00        	mov	rax, qword ptr [rbp + 0x2f0]
140904d33: 4c 8b b0 e0 00 00 00        	mov	r14, qword ptr [rax + 0xe0]
140904d3a: 4d 8b 86 d0 00 00 00        	mov	r8, qword ptr [r14 + 0xd0]
140904d41: c6 85 fe 02 00 00 01        	mov	byte ptr [rbp + 0x2fe], 0x1
140904d48: 48 8d 8d a8 02 00 00        	lea	rcx, [rbp + 0x2a8]
140904d4f: 4c 89 e2                    	mov	rdx, r12
140904d52: e8 c9 bd 9c 00              	call	0x1412d0b20 <.text+0x12cfb20>
140904d57: 4d 8b 86 d0 00 00 00        	mov	r8, qword ptr [r14 + 0xd0]
140904d5e: c6 85 fe 02 00 00 01        	mov	byte ptr [rbp + 0x2fe], 0x1
140904d65: 48 8d 8d c0 02 00 00        	lea	rcx, [rbp + 0x2c0]
140904d6c: 4c 89 e2                    	mov	rdx, r12
140904d6f: e8 2c a4 9c 00              	call	0x1412cf1a0 <.text+0x12ce1a0>
140904d74: 4c 8d 65 c0                 	lea	r12, [rbp - 0x40]
140904d78: 41 b8 f8 00 00 00           	mov	r8d, 0xf8
140904d7e: 4c 89 e1                    	mov	rcx, r12
140904d81: 48 8b 95 f0 02 00 00        	mov	rdx, qword ptr [rbp + 0x2f0]
140904d88: e8 73 ee 01 02              	call	0x142923c00 <.text+0x2922c00>
140904d8d: c6 85 fe 02 00 00 00        	mov	byte ptr [rbp + 0x2fe], 0x0
140904d94: 48 8d 8d b8 00 00 00        	lea	rcx, [rbp + 0xb8]
140904d9b: 49 b8 80 8c 9c 6a 6a c3 f9 cd       	movabs	r8, -0x32063c9595637380
140904da5: 4c 89 e2                    	mov	rdx, r12
140904da8: e8 e3 98 d2 00              	call	0x14162e690 <.text+0x162d690>
140904dad: 4d 85 ff                    	test	r15, r15
140904db0: 74 41                       	je	0x140904df3 <.text+0x903df3>
140904db2: c6 85 fe 02 00 00 00        	mov	byte ptr [rbp + 0x2fe], 0x0
140904db9: 48 8d 8d b0 01 00 00        	lea	rcx, [rbp + 0x1b0]
140904dc0: 48 8d 95 b8 00 00 00        	lea	rdx, [rbp + 0xb8]
140904dc7: 49 b8 c6 90 17 63 2d 41 06 21       	movabs	r8, 0x2106412d631790c6
140904dd1: 4d 89 f9                    	mov	r9, r15
140904dd4: e8 77 13 00 00              	call	0x140906150 <.text+0x905150>
140904dd9: 49 c1 e5 02                 	shl	r13, 0x2
140904ddd: 4e 8d 3c ad 00 00 00 00     	lea	r15, [4*r13]
140904de5: 4d 01 ef                    	add	r15, r13
140904de8: 80 3d ad ff d2 02 00        	cmp	byte ptr [rip + 0x2d2ffad], 0x0 # 0x143634d9c
140904def: 74 33                       	je	0x140904e24 <.text+0x903e24>
140904df1: eb 40                       	jmp	0x140904e33 <.text+0x903e33>
140904df3: 48 8d 8d b0 01 00 00        	lea	rcx, [rbp + 0x1b0]
140904dfa: 48 8d 95 b8 00 00 00        	lea	rdx, [rbp + 0xb8]
140904e01: 41 b8 f8 00 00 00           	mov	r8d, 0xf8
140904e07: e8 f4 ed 01 02              	call	0x142923c00 <.text+0x2922c00>
140904e0c: 49 c1 e5 02                 	shl	r13, 0x2
140904e10: 4e 8d 3c ad 00 00 00 00     	lea	r15, [4*r13]
140904e18: 4d 01 ef                    	add	r15, r13
140904e1b: 80 3d 7a ff d2 02 00        	cmp	byte ptr [rip + 0x2d2ff7a], 0x0 # 0x143634d9c
140904e22: 75 0f                       	jne	0x140904e33 <.text+0x903e33>
140904e24: 40 80 fe 01                 	cmp	sil, 0x1
140904e28: 77 6b                       	ja	0x140904e95 <.text+0x903e95>
140904e2a: 80 3d 6c ff d2 02 00        	cmp	byte ptr [rip + 0x2d2ff6c], 0x0 # 0x143634d9d
140904e31: 75 62                       	jne	0x140904e95 <.text+0x903e95>
140904e33: 48 8d 75 c0                 	lea	rsi, [rbp - 0x40]
140904e37: 48 8d 95 b0 01 00 00        	lea	rdx, [rbp + 0x1b0]
140904e3e: 41 b8 f8 00 00 00           	mov	r8d, 0xf8
140904e44: 48 89 f1                    	mov	rcx, rsi
140904e47: e8 b4 ed 01 02              	call	0x142923c00 <.text+0x2922c00>
140904e4c: 48 8d 05 35 64 1a 02        	lea	rax, [rip + 0x21a6435]  # 0x142aab288
140904e53: 48 8b 8d d8 02 00 00        	mov	rcx, qword ptr [rbp + 0x2d8]
140904e5a: 48 8b 04 c8                 	mov	rax, qword ptr [rax + 8*rcx]
140904e5e: c6 85 fe 02 00 00 00        	mov	byte ptr [rbp + 0x2fe], 0x0
140904e65: 48 89 44 24 30              	mov	qword ptr [rsp + 0x30], rax
140904e6a: 48 89 5c 24 28              	mov	qword ptr [rsp + 0x28], rbx
140904e6f: 48 8d 85 a8 02 00 00        	lea	rax, [rbp + 0x2a8]
140904e76: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
140904e7b: 48 8d 95 c0 02 00 00        	lea	rdx, [rbp + 0x2c0]
140904e82: 41 b9 14 00 00 00           	mov	r9d, 0x14
140904e88: 48 89 f1                    	mov	rcx, rsi
140904e8b: 4d 89 f8                    	mov	r8, r15
140904e8e: e8 fd 30 00 00              	call	0x140907f90 <.text+0x906f90>
140904e93: eb 44                       	jmp	0x140904ed9 <.text+0x903ed9>
140904e95: 48 8d 75 c0                 	lea	rsi, [rbp - 0x40]
140904e99: 48 8d 95 b0 01 00 00        	lea	rdx, [rbp + 0x1b0]
140904ea0: 41 b8 f8 00 00 00           	mov	r8d, 0xf8
140904ea6: 48 89 f1                    	mov	rcx, rsi
140904ea9: e8 52 ed 01 02              	call	0x142923c00 <.text+0x2922c00>
140904eae: c6 85 fe 02 00 00 00        	mov	byte ptr [rbp + 0x2fe], 0x0
140904eb5: 48 8b 85 e0 02 00 00        	mov	rax, qword ptr [rbp + 0x2e0]
140904ebc: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
140904ec1: 48 8d 95 c0 02 00 00        	lea	rdx, [rbp + 0x2c0]
140904ec8: 41 b9 14 00 00 00           	mov	r9d, 0x14
140904ece: 48 89 f1                    	mov	rcx, rsi
140904ed1: 4d 89 f8                    	mov	r8, r15
140904ed4: e8 e7 2d 00 00              	call	0x140907cc0 <.text+0x906cc0>
140904ed9: c6 85 ff 02 00 00 00        	mov	byte ptr [rbp + 0x2ff], 0x0
140904ee0: 48 8b b5 e8 02 00 00        	mov	rsi, qword ptr [rbp + 0x2e8]
140904ee7: 48 89 f1                    	mov	rcx, rsi
140904eea: e8 21 cf 37 00              	call	0x140c81e10 <.text+0xc80e10>
140904eef: c6 85 ff 02 00 00 00        	mov	byte ptr [rbp + 0x2ff], 0x0
140904ef6: 48 89 f9                    	mov	rcx, rdi
140904ef9: e8 12 cf 37 00              	call	0x140c81e10 <.text+0xc80e10>
140904efe: 48 c7 c0 f0 ff ff ff        	mov	rax, -0x10
140904f05: f0                          	lock
140904f06: 48 0f c1 06                 	xadd	qword ptr [rsi], rax
140904f0a: 48 83 e0 f2                 	and	rax, -0xe
140904f0e: 48 83 f8 12                 	cmp	rax, 0x12
140904f12: 74 14                       	je	0x140904f28 <.text+0x903f28>
140904f14: 48 81 c4 88 03 00 00        	add	rsp, 0x388
140904f1b: 5b                          	pop	rbx
140904f1c: 5f                          	pop	rdi
140904f1d: 5e                          	pop	rsi
140904f1e: 41 5c                       	pop	r12
140904f20: 41 5d                       	pop	r13
140904f22: 41 5e                       	pop	r14
140904f24: 41 5f                       	pop	r15
140904f26: 5d                          	pop	rbp
140904f27: c3                          	ret
140904f28: c6 85 ff 02 00 00 00        	mov	byte ptr [rbp + 0x2ff], 0x0
140904f2f: 48 89 f1                    	mov	rcx, rsi
140904f32: e8 19 80 fc 01              	call	0x1428ccf50 <.text+0x28cbf50>
140904f37: eb db                       	jmp	0x140904f14 <.text+0x903f14>
140904f39: c6 85 ff 02 00 00 01        	mov	byte ptr [rbp + 0x2ff], 0x1
140904f40: 48 8d 0d 41 56 1a 02        	lea	rcx, [rip + 0x21a5641]  # 0x142aaa588
140904f47: e8 c4 43 fd 01              	call	0x1428d9310 <.text+0x28d8310>
140904f4c: eb 39                       	jmp	0x140904f87 <.text+0x903f87>
140904f4e: c6 85 fe 02 00 00 01        	mov	byte ptr [rbp + 0x2fe], 0x1
140904f55: 48 8d 0d 44 56 1a 02        	lea	rcx, [rip + 0x21a5644]  # 0x142aaa5a0
140904f5c: e8 af 43 fd 01              	call	0x1428d9310 <.text+0x28d8310>
140904f61: eb 24                       	jmp	0x140904f87 <.text+0x903f87>
140904f63: 48 8d 0d a6 54 1b 02        	lea	rcx, [rip + 0x21b54a6]  # 0x142aba410
140904f6a: e8 a1 43 fd 01              	call	0x1428d9310 <.text+0x28d8310>
140904f6f: eb 16                       	jmp	0x140904f87 <.text+0x903f87>
140904f71: c6 85 fe 02 00 00 01        	mov	byte ptr [rbp + 0x2fe], 0x1
140904f78: 4c 8d 05 71 55 1b 02        	lea	r8, [rip + 0x21b5571]   # 0x142aba4f0
140904f7f: 48 89 d9                    	mov	rcx, rbx
140904f82: e8 91 45 fd 01              	call	0x1428d9518 <.text+0x28d8518>
140904f87: 0f 0b                       	ud2
140904f89: 0f 1f 80 00 00 00 00        	nop	dword ptr [rax]
140904f90: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140904f95: 55                          	push	rbp
140904f96: 41 57                       	push	r15
140904f98: 41 56                       	push	r14
140904f9a: 41 55                       	push	r13
140904f9c: 41 54                       	push	r12
140904f9e: 56                          	push	rsi
140904f9f: 57                          	push	rdi
140904fa0: 53                          	push	rbx
140904fa1: 48 83 ec 38                 	sub	rsp, 0x38
140904fa5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140904fac: c6 85 ff 02 00 00 01        	mov	byte ptr [rbp + 0x2ff], 0x1
140904fb3: 48 8b 8d e8 02 00 00        	mov	rcx, qword ptr [rbp + 0x2e8]
140904fba: e8 c1 c2 73 ff              	call	0x140041280 <.text+0x40280>
140904fbf: c6 85 ff 02 00 00 01        	mov	byte ptr [rbp + 0x2ff], 0x1
140904fc6: 48 83 c4 38                 	add	rsp, 0x38
140904fca: 5b                          	pop	rbx
140904fcb: 5f                          	pop	rdi
140904fcc: 5e                          	pop	rsi
140904fcd: 41 5c                       	pop	r12
140904fcf: 41 5d                       	pop	r13
140904fd1: 41 5e                       	pop	r14
140904fd3: 41 5f                       	pop	r15
140904fd5: 5d                          	pop	rbp
140904fd6: c3                          	ret
140904fd7: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
140904fe0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140904fe5: 55                          	push	rbp
140904fe6: 41 57                       	push	r15
140904fe8: 41 56                       	push	r14
140904fea: 41 55                       	push	r13
140904fec: 41 54                       	push	r12
140904fee: 56                          	push	rsi
140904fef: 57                          	push	rdi
140904ff0: 53                          	push	rbx
140904ff1: 48 83 ec 38                 	sub	rsp, 0x38
140904ff5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140904ffc: 0f b6 9d fe 02 00 00        	movzx	ebx, byte ptr [rbp + 0x2fe]
140905003: 48 8b 8d e8 02 00 00        	mov	rcx, qword ptr [rbp + 0x2e8]
14090500a: e8 71 c2 73 ff              	call	0x140041280 <.text+0x40280>
14090500f: 88 9d ff 02 00 00           	mov	byte ptr [rbp + 0x2ff], bl
140905015: 48 83 c4 38                 	add	rsp, 0x38
140905019: 5b                          	pop	rbx
14090501a: 5f                          	pop	rdi
14090501b: 5e                          	pop	rsi
14090501c: 41 5c                       	pop	r12
14090501e: 41 5d                       	pop	r13
140905020: 41 5e                       	pop	r14
140905022: 41 5f                       	pop	r15
140905024: 5d                          	pop	rbp
140905025: c3                          	ret
140905026: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
140905030: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140905035: 55                          	push	rbp
140905036: 41 57                       	push	r15
140905038: 41 56                       	push	r14
14090503a: 41 55                       	push	r13
14090503c: 41 54                       	push	r12
14090503e: 56                          	push	rsi
14090503f: 57                          	push	rdi
140905040: 53                          	push	rbx
140905041: 48 83 ec 38                 	sub	rsp, 0x38
140905045: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14090504c: 80 bd ff 02 00 00 00        	cmp	byte ptr [rbp + 0x2ff], 0x0
140905053: 75 11                       	jne	0x140905066 <.text+0x904066>
140905055: 48 83 c4 38                 	add	rsp, 0x38
140905059: 5b                          	pop	rbx
14090505a: 5f                          	pop	rdi
14090505b: 5e                          	pop	rsi
14090505c: 41 5c                       	pop	r12
14090505e: 41 5d                       	pop	r13
140905060: 41 5e                       	pop	r14
140905062: 41 5f                       	pop	r15
140905064: 5d                          	pop	rbp
140905065: c3                          	ret
140905066: 48 8b 8d f0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2f0]
14090506d: e8 4e e5 f8 ff              	call	0x1408935c0 <.text+0x8925c0>
140905072: eb e1                       	jmp	0x140905055 <.text+0x904055>
