
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14142f860: 55                          	push	rbp
14142f861: 41 57                       	push	r15
14142f863: 41 56                       	push	r14
14142f865: 41 55                       	push	r13
14142f867: 41 54                       	push	r12
14142f869: 56                          	push	rsi
14142f86a: 57                          	push	rdi
14142f86b: 53                          	push	rbx
14142f86c: 48 81 ec 38 01 00 00        	sub	rsp, 0x138
14142f873: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
14142f87b: 0f 29 bd a0 00 00 00        	movaps	xmmword ptr [rbp + 0xa0], xmm7
14142f882: 0f 29 b5 90 00 00 00        	movaps	xmmword ptr [rbp + 0x90], xmm6
14142f889: 48 c7 85 88 00 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x88], -0x2
14142f894: 48 89 d7                    	mov	rdi, rdx
14142f897: 48 89 ce                    	mov	rsi, rcx
14142f89a: 48 8b 4a 20                 	mov	rcx, qword ptr [rdx + 0x20]
14142f89e: 48 8b 42 28                 	mov	rax, qword ptr [rdx + 0x28]
14142f8a2: 48 39 c8                    	cmp	rax, rcx
14142f8a5: 73 4f                       	jae	0x14142f8f6 <.text+0x142e8f6>
14142f8a7: 4c 8d 77 18                 	lea	r14, [rdi + 0x18]
14142f8ab: 48 8b 57 18                 	mov	rdx, qword ptr [rdi + 0x18]
14142f8af: 48 ff c0                    	inc	rax
14142f8b2: 49 89 c8                    	mov	r8, rcx
14142f8b5: 49 f7 d8                    	neg	r8
14142f8b8: 49 bf 00 26 00 00 01 00 00 00       	movabs	r15, 0x100002600
14142f8c2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nop	word ptr cs:[rax + rax]
14142f8d0: 44 0f b6 4c 02 ff           	movzx	r9d, byte ptr [rdx + rax - 0x1]
14142f8d6: 49 83 f9 20                 	cmp	r9, 0x20
14142f8da: 77 3d                       	ja	0x14142f919 <.text+0x142e919>
14142f8dc: 4d 0f a3 cf                 	bt	r15, r9
14142f8e0: 73 37                       	jae	0x14142f919 <.text+0x142e919>
14142f8e2: 48 89 47 28                 	mov	qword ptr [rdi + 0x28], rax
14142f8e6: 4d 8d 0c 00                 	lea	r9, [r8 + rax]
14142f8ea: 49 ff c1                    	inc	r9
14142f8ed: 48 ff c0                    	inc	rax
14142f8f0: 49 83 f9 01                 	cmp	r9, 0x1
14142f8f4: 75 da                       	jne	0x14142f8d0 <.text+0x142e8d0>
14142f8f6: 48 c7 45 48 05 00 00 00     	mov	qword ptr [rbp + 0x48], 0x5
14142f8fe: 48 8d 55 48                 	lea	rdx, [rbp + 0x48]
14142f902: 48 89 f9                    	mov	rcx, rdi
14142f905: e8 16 10 47 01              	call	0x1428a0920 <.text+0x289f920>
14142f90a: 48 89 46 08                 	mov	qword ptr [rsi + 0x8], rax
14142f90e: c7 06 01 00 00 00           	mov	dword ptr [rsi], 0x1
14142f914: e9 a7 0a 00 00              	jmp	0x1414303c0 <.text+0x142f3c0>
14142f919: 41 83 f9 5b                 	cmp	r9d, 0x5b
14142f91d: 0f 84 86 06 00 00           	je	0x14142ffa9 <.text+0x142efa9>
14142f923: 41 83 f9 7b                 	cmp	r9d, 0x7b
14142f927: 0f 85 b8 0a 00 00           	jne	0x1414303e5 <.text+0x142f3e5>
14142f92d: fe 4f 30                    	dec	byte ptr [rdi + 0x30]
14142f930: 0f 84 41 0c 00 00           	je	0x141430577 <.text+0x142f577>
14142f936: 48 89 47 28                 	mov	qword ptr [rdi + 0x28], rax
14142f93a: 48 39 c8                    	cmp	rax, rcx
14142f93d: 0f 83 59 06 00 00           	jae	0x14142ff9c <.text+0x142ef9c>
14142f943: 45 31 e4                    	xor	r12d, r12d
14142f946: b3 01                       	mov	bl, 0x1
14142f948: 4c 8d 6d 48                 	lea	r13, [rbp + 0x48]
14142f94c: c7 45 40 00 00 00 00        	mov	dword ptr [rbp + 0x40], 0x0
14142f953: c7 85 80 00 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x80], 0x0
14142f95d: c7 45 f8 00 00 00 00        	mov	dword ptr [rbp - 0x8], 0x0
14142f964: c7 45 08 00 00 00 00        	mov	dword ptr [rbp + 0x8], 0x0
14142f96b: c7 45 04 00 00 00 00        	mov	dword ptr [rbp + 0x4], 0x0
14142f972: c7 45 00 00 00 00 00        	mov	dword ptr [rbp], 0x0
14142f979: c7 45 fc 00 00 00 00        	mov	dword ptr [rbp - 0x4], 0x0
14142f980: 48 89 55 d8                 	mov	qword ptr [rbp - 0x28], rdx
14142f984: 49 8b 16                    	mov	rdx, qword ptr [r14]
14142f987: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
14142f990: 44 0f b6 04 02              	movzx	r8d, byte ptr [rdx + rax]
14142f995: 49 83 f8 2c                 	cmp	r8, 0x2c
14142f999: 77 75                       	ja	0x14142fa10 <.text+0x142ea10>
14142f99b: 4d 0f a3 c7                 	bt	r15, r8
14142f99f: 73 1f                       	jae	0x14142f9c0 <.text+0x142e9c0>
14142f9a1: 48 ff c0                    	inc	rax
14142f9a4: 48 89 47 28                 	mov	qword ptr [rdi + 0x28], rax
14142f9a8: 48 39 c1                    	cmp	rcx, rax
14142f9ab: 75 e3                       	jne	0x14142f990 <.text+0x142e990>
14142f9ad: e9 ea 05 00 00              	jmp	0x14142ff9c <.text+0x142ef9c>
14142f9b2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nop	word ptr cs:[rax + rax]
14142f9c0: 49 83 f8 2c                 	cmp	r8, 0x2c
14142f9c4: 75 4a                       	jne	0x14142fa10 <.text+0x142ea10>
14142f9c6: f6 c3 01                    	test	bl, 0x1
14142f9c9: 0f 85 44 0b 00 00           	jne	0x141430513 <.text+0x142f513>
14142f9cf: 48 ff c0                    	inc	rax
14142f9d2: 48 89 47 28                 	mov	qword ptr [rdi + 0x28], rax
14142f9d6: 48 39 c8                    	cmp	rax, rcx
14142f9d9: 0f 83 95 08 00 00           	jae	0x141430274 <.text+0x142f274>
14142f9df: 90                          	nop
14142f9e0: 44 0f b6 04 02              	movzx	r8d, byte ptr [rdx + rax]
14142f9e5: 49 83 f8 20                 	cmp	r8, 0x20
14142f9e9: 77 3a                       	ja	0x14142fa25 <.text+0x142ea25>
14142f9eb: 4d 0f a3 c7                 	bt	r15, r8
14142f9ef: 73 34                       	jae	0x14142fa25 <.text+0x142ea25>
14142f9f1: 48 ff c0                    	inc	rax
14142f9f4: 48 89 47 28                 	mov	qword ptr [rdi + 0x28], rax
14142f9f8: 48 39 c1                    	cmp	rcx, rax
14142f9fb: 75 e3                       	jne	0x14142f9e0 <.text+0x142e9e0>
14142f9fd: e9 72 08 00 00              	jmp	0x141430274 <.text+0x142f274>
14142fa02: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nop	word ptr cs:[rax + rax]
14142fa10: 41 83 f8 7d                 	cmp	r8d, 0x7d
14142fa14: 0f 84 95 07 00 00           	je	0x1414301af <.text+0x142f1af>
14142fa1a: f6 c3 01                    	test	bl, 0x1
14142fa1d: 0f 84 61 0b 00 00           	je	0x141430584 <.text+0x142f584>
14142fa23: 31 db                       	xor	ebx, ebx
14142fa25: 41 80 f8 22                 	cmp	r8b, 0x22
14142fa29: 0f 85 b6 0a 00 00           	jne	0x1414304e5 <.text+0x142f4e5>
14142fa2f: 48 ff c0                    	inc	rax
14142fa32: 48 89 47 28                 	mov	qword ptr [rdi + 0x28], rax
14142fa36: 48 c7 47 10 00 00 00 00     	mov	qword ptr [rdi + 0x10], 0x0
14142fa3e: 4c 89 e9                    	mov	rcx, r13
14142fa41: 4c 89 f2                    	mov	rdx, r14
14142fa44: 49 89 f8                    	mov	r8, rdi
14142fa47: e8 c4 ae 75 ff              	call	0x140b8a910 <.text+0xb89910>
14142fa4c: 83 7d 48 02                 	cmp	dword ptr [rbp + 0x48], 0x2
14142fa50: 48 8b 55 50                 	mov	rdx, qword ptr [rbp + 0x50]
14142fa54: 0f 84 e0 09 00 00           	je	0x14143043a <.text+0x142f43a>
14142fa5a: 4c 8b 45 58                 	mov	r8, qword ptr [rbp + 0x58]
14142fa5e: 48 8d 4d 10                 	lea	rcx, [rbp + 0x10]
14142fa62: e8 f9 be 70 ff              	call	0x140b3b960 <.text+0xb3a960>
14142fa67: f6 45 10 01                 	test	byte ptr [rbp + 0x10], 0x1
14142fa6b: 0f 85 c5 09 00 00           	jne	0x141430436 <.text+0x142f436>
14142fa71: 0f b6 45 11                 	movzx	eax, byte ptr [rbp + 0x11]
14142fa75: 48 8d 0d b8 e1 82 01        	lea	rcx, [rip + 0x182e1b8]  # 0x142c5dc34
14142fa7c: 48 63 04 81                 	movsxd	rax, dword ptr [rcx + 4*rax]
14142fa80: 48 01 c8                    	add	rax, rcx
14142fa83: ff e0                       	jmp	rax
14142fa85: 45 85 e4                    	test	r12d, r12d
14142fa88: 0f 85 1e 0b 00 00           	jne	0x1414305ac <.text+0x142f5ac>
14142fa8e: 48 8b 4f 20                 	mov	rcx, qword ptr [rdi + 0x20]
14142fa92: 48 8b 47 28                 	mov	rax, qword ptr [rdi + 0x28]
14142fa96: 48 39 c8                    	cmp	rax, rcx
14142fa99: 0f 83 fd 04 00 00           	jae	0x14142ff9c <.text+0x142ef9c>
14142fa9f: 49 8b 16                    	mov	rdx, qword ptr [r14]
14142faa2: 48 ff c0                    	inc	rax
14142faa5: 48 f7 d9                    	neg	rcx
14142faa8: 44 0f b6 44 02 ff           	movzx	r8d, byte ptr [rdx + rax - 0x1]
14142faae: 49 83 f8 3a                 	cmp	r8, 0x3a
14142fab2: 0f 87 d9 0a 00 00           	ja	0x141430591 <.text+0x142f591>
14142fab8: 4d 0f a3 c7                 	bt	r15, r8
14142fabc: 0f 83 b1 03 00 00           	jae	0x14142fe73 <.text+0x142ee73>
14142fac2: 48 89 47 28                 	mov	qword ptr [rdi + 0x28], rax
14142fac6: 4c 8d 04 01                 	lea	r8, [rcx + rax]
14142faca: 49 ff c0                    	inc	r8
14142facd: 48 ff c0                    	inc	rax
14142fad0: 49 83 f8 01                 	cmp	r8, 0x1
14142fad4: 75 d2                       	jne	0x14142faa8 <.text+0x142eaa8>
14142fad6: e9 c1 04 00 00              	jmp	0x14142ff9c <.text+0x142ef9c>
14142fadb: 83 7d 08 00                 	cmp	dword ptr [rbp + 0x8], 0x0
14142fadf: 0f 85 16 0b 00 00           	jne	0x1414305fb <.text+0x142f5fb>
14142fae5: 48 8b 4f 20                 	mov	rcx, qword ptr [rdi + 0x20]
14142fae9: 48 8b 47 28                 	mov	rax, qword ptr [rdi + 0x28]
14142faed: 48 39 c8                    	cmp	rax, rcx
14142faf0: 0f 83 a6 04 00 00           	jae	0x14142ff9c <.text+0x142ef9c>
14142faf6: 49 8b 16                    	mov	rdx, qword ptr [r14]
14142faf9: 48 ff c0                    	inc	rax
14142fafc: 48 f7 d9                    	neg	rcx
14142faff: 44 0f b6 44 02 ff           	movzx	r8d, byte ptr [rdx + rax - 0x1]
14142fb05: 49 83 f8 3a                 	cmp	r8, 0x3a
14142fb09: 0f 87 82 0a 00 00           	ja	0x141430591 <.text+0x142f591>
14142fb0f: 4d 0f a3 c7                 	bt	r15, r8
14142fb13: 0f 83 3c 02 00 00           	jae	0x14142fd55 <.text+0x142ed55>
14142fb19: 48 89 47 28                 	mov	qword ptr [rdi + 0x28], rax
14142fb1d: 4c 8d 04 01                 	lea	r8, [rcx + rax]
14142fb21: 49 ff c0                    	inc	r8
14142fb24: 48 ff c0                    	inc	rax
14142fb27: 49 83 f8 01                 	cmp	r8, 0x1
14142fb2b: 75 d2                       	jne	0x14142faff <.text+0x142eaff>
14142fb2d: e9 6a 04 00 00              	jmp	0x14142ff9c <.text+0x142ef9c>
14142fb32: 83 bd 80 00 00 00 00        	cmp	dword ptr [rbp + 0x80], 0x0
14142fb39: 0f 85 97 0a 00 00           	jne	0x1414305d6 <.text+0x142f5d6>
14142fb3f: 48 8b 4f 20                 	mov	rcx, qword ptr [rdi + 0x20]
14142fb43: 48 8b 47 28                 	mov	rax, qword ptr [rdi + 0x28]
14142fb47: 48 39 c8                    	cmp	rax, rcx
14142fb4a: 0f 83 4c 04 00 00           	jae	0x14142ff9c <.text+0x142ef9c>
14142fb50: 49 8b 16                    	mov	rdx, qword ptr [r14]
14142fb53: 48 ff c0                    	inc	rax
14142fb56: 48 f7 d9                    	neg	rcx
14142fb59: 44 0f b6 44 02 ff           	movzx	r8d, byte ptr [rdx + rax - 0x1]
14142fb5f: 49 83 f8 3a                 	cmp	r8, 0x3a
14142fb63: 0f 87 28 0a 00 00           	ja	0x141430591 <.text+0x142f591>
14142fb69: 4d 0f a3 c7                 	bt	r15, r8
14142fb6d: 0f 83 29 02 00 00           	jae	0x14142fd9c <.text+0x142ed9c>
14142fb73: 48 89 47 28                 	mov	qword ptr [rdi + 0x28], rax
14142fb77: 4c 8d 04 01                 	lea	r8, [rcx + rax]
14142fb7b: 49 ff c0                    	inc	r8
14142fb7e: 48 ff c0                    	inc	rax
14142fb81: 49 83 f8 01                 	cmp	r8, 0x1
14142fb85: 75 d2                       	jne	0x14142fb59 <.text+0x142eb59>
14142fb87: e9 10 04 00 00              	jmp	0x14142ff9c <.text+0x142ef9c>
14142fb8c: 83 7d f8 00                 	cmp	dword ptr [rbp - 0x8], 0x0
14142fb90: 0f 85 24 0a 00 00           	jne	0x1414305ba <.text+0x142f5ba>
14142fb96: 48 8b 4f 20                 	mov	rcx, qword ptr [rdi + 0x20]
14142fb9a: 48 8b 47 28                 	mov	rax, qword ptr [rdi + 0x28]
14142fb9e: 48 39 c8                    	cmp	rax, rcx
14142fba1: 0f 83 f5 03 00 00           	jae	0x14142ff9c <.text+0x142ef9c>
14142fba7: 49 8b 16                    	mov	rdx, qword ptr [r14]
14142fbaa: 48 ff c0                    	inc	rax
14142fbad: 48 f7 d9                    	neg	rcx
14142fbb0: 44 0f b6 44 02 ff           	movzx	r8d, byte ptr [rdx + rax - 0x1]
14142fbb6: 49 83 f8 3a                 	cmp	r8, 0x3a
14142fbba: 0f 87 d1 09 00 00           	ja	0x141430591 <.text+0x142f591>
14142fbc0: 4d 0f a3 c7                 	bt	r15, r8
14142fbc4: 0f 83 1b 02 00 00           	jae	0x14142fde5 <.text+0x142ede5>
14142fbca: 48 89 47 28                 	mov	qword ptr [rdi + 0x28], rax
14142fbce: 4c 8d 04 01                 	lea	r8, [rcx + rax]
14142fbd2: 49 ff c0                    	inc	r8
14142fbd5: 48 ff c0                    	inc	rax
14142fbd8: 49 83 f8 01                 	cmp	r8, 0x1
14142fbdc: 75 d2                       	jne	0x14142fbb0 <.text+0x142ebb0>
14142fbde: e9 b9 03 00 00              	jmp	0x14142ff9c <.text+0x142ef9c>
14142fbe3: 83 7d fc 00                 	cmp	dword ptr [rbp - 0x4], 0x0
14142fbe7: 0f 85 b1 09 00 00           	jne	0x14143059e <.text+0x142f59e>
14142fbed: 48 8b 4f 20                 	mov	rcx, qword ptr [rdi + 0x20]
14142fbf1: 48 8b 47 28                 	mov	rax, qword ptr [rdi + 0x28]
14142fbf5: 48 39 c8                    	cmp	rax, rcx
14142fbf8: 0f 83 9e 03 00 00           	jae	0x14142ff9c <.text+0x142ef9c>
14142fbfe: 49 8b 16                    	mov	rdx, qword ptr [r14]
14142fc01: 48 ff c0                    	inc	rax
14142fc04: 48 f7 d9                    	neg	rcx
14142fc07: 44 0f b6 44 02 ff           	movzx	r8d, byte ptr [rdx + rax - 0x1]
14142fc0d: 49 83 f8 3a                 	cmp	r8, 0x3a
14142fc11: 0f 87 7a 09 00 00           	ja	0x141430591 <.text+0x142f591>
14142fc17: 4d 0f a3 c7                 	bt	r15, r8
14142fc1b: 0f 83 0b 02 00 00           	jae	0x14142fe2c <.text+0x142ee2c>
14142fc21: 48 89 47 28                 	mov	qword ptr [rdi + 0x28], rax
14142fc25: 4c 8d 04 01                 	lea	r8, [rcx + rax]
14142fc29: 49 ff c0                    	inc	r8
14142fc2c: 48 ff c0                    	inc	rax
14142fc2f: 49 83 f8 01                 	cmp	r8, 0x1
14142fc33: 75 d2                       	jne	0x14142fc07 <.text+0x142ec07>
14142fc35: e9 62 03 00 00              	jmp	0x14142ff9c <.text+0x142ef9c>
14142fc3a: 83 7d 40 00                 	cmp	dword ptr [rbp + 0x40], 0x0
14142fc3e: 0f 85 a9 09 00 00           	jne	0x1414305ed <.text+0x142f5ed>
14142fc44: 48 8b 4f 20                 	mov	rcx, qword ptr [rdi + 0x20]
14142fc48: 48 8b 47 28                 	mov	rax, qword ptr [rdi + 0x28]
14142fc4c: 48 39 c8                    	cmp	rax, rcx
14142fc4f: 0f 83 47 03 00 00           	jae	0x14142ff9c <.text+0x142ef9c>
14142fc55: 49 8b 16                    	mov	rdx, qword ptr [r14]
14142fc58: 48 ff c0                    	inc	rax
14142fc5b: 48 f7 d9                    	neg	rcx
14142fc5e: 44 0f b6 44 02 ff           	movzx	r8d, byte ptr [rdx + rax - 0x1]
14142fc64: 49 83 f8 3a                 	cmp	r8, 0x3a
14142fc68: 0f 87 23 09 00 00           	ja	0x141430591 <.text+0x142f591>
14142fc6e: 4d 0f a3 c7                 	bt	r15, r8
14142fc72: 0f 83 49 02 00 00           	jae	0x14142fec1 <.text+0x142eec1>
14142fc78: 48 89 47 28                 	mov	qword ptr [rdi + 0x28], rax
14142fc7c: 4c 8d 04 01                 	lea	r8, [rcx + rax]
14142fc80: 49 ff c0                    	inc	r8
14142fc83: 48 ff c0                    	inc	rax
14142fc86: 49 83 f8 01                 	cmp	r8, 0x1
14142fc8a: 75 d2                       	jne	0x14142fc5e <.text+0x142ec5e>
14142fc8c: e9 0b 03 00 00              	jmp	0x14142ff9c <.text+0x142ef9c>
14142fc91: 83 7d 04 00                 	cmp	dword ptr [rbp + 0x4], 0x0
14142fc95: 0f 85 49 09 00 00           	jne	0x1414305e4 <.text+0x142f5e4>
14142fc9b: 48 8b 4f 20                 	mov	rcx, qword ptr [rdi + 0x20]
14142fc9f: 48 8b 47 28                 	mov	rax, qword ptr [rdi + 0x28]
14142fca3: 48 39 c8                    	cmp	rax, rcx
14142fca6: 0f 83 f0 02 00 00           	jae	0x14142ff9c <.text+0x142ef9c>
14142fcac: 49 8b 16                    	mov	rdx, qword ptr [r14]
14142fcaf: 48 ff c0                    	inc	rax
14142fcb2: 48 f7 d9                    	neg	rcx
14142fcb5: 44 0f b6 44 02 ff           	movzx	r8d, byte ptr [rdx + rax - 0x1]
14142fcbb: 49 83 f8 3a                 	cmp	r8, 0x3a
14142fcbf: 0f 87 cc 08 00 00           	ja	0x141430591 <.text+0x142f591>
14142fcc5: 4d 0f a3 c7                 	bt	r15, r8
14142fcc9: 0f 83 38 02 00 00           	jae	0x14142ff07 <.text+0x142ef07>
14142fccf: 48 89 47 28                 	mov	qword ptr [rdi + 0x28], rax
14142fcd3: 4c 8d 04 01                 	lea	r8, [rcx + rax]
14142fcd7: 49 ff c0                    	inc	r8
14142fcda: 48 ff c0                    	inc	rax
14142fcdd: 49 83 f8 01                 	cmp	r8, 0x1
14142fce1: 75 d2                       	jne	0x14142fcb5 <.text+0x142ecb5>
14142fce3: e9 b4 02 00 00              	jmp	0x14142ff9c <.text+0x142ef9c>
14142fce8: 83 7d 00 00                 	cmp	dword ptr [rbp], 0x0
14142fcec: 0f 85 d6 08 00 00           	jne	0x1414305c8 <.text+0x142f5c8>
14142fcf2: 48 8b 4f 20                 	mov	rcx, qword ptr [rdi + 0x20]
14142fcf6: 48 8b 47 28                 	mov	rax, qword ptr [rdi + 0x28]
14142fcfa: 48 39 c8                    	cmp	rax, rcx
14142fcfd: 0f 83 99 02 00 00           	jae	0x14142ff9c <.text+0x142ef9c>
14142fd03: 49 8b 16                    	mov	rdx, qword ptr [r14]
14142fd06: 48 ff c0                    	inc	rax
14142fd09: 48 f7 d9                    	neg	rcx
14142fd0c: 44 0f b6 44 02 ff           	movzx	r8d, byte ptr [rdx + rax - 0x1]
14142fd12: 49 83 f8 3a                 	cmp	r8, 0x3a
14142fd16: 0f 87 75 08 00 00           	ja	0x141430591 <.text+0x142f591>
14142fd1c: 4d 0f a3 c7                 	bt	r15, r8
14142fd20: 0f 83 2d 02 00 00           	jae	0x14142ff53 <.text+0x142ef53>
14142fd26: 48 89 47 28                 	mov	qword ptr [rdi + 0x28], rax
14142fd2a: 4c 8d 04 01                 	lea	r8, [rcx + rax]
14142fd2e: 49 ff c0                    	inc	r8
14142fd31: 48 ff c0                    	inc	rax
14142fd34: 49 83 f8 01                 	cmp	r8, 0x1
14142fd38: 75 d2                       	jne	0x14142fd0c <.text+0x142ed0c>
14142fd3a: e9 5d 02 00 00              	jmp	0x14142ff9c <.text+0x142ef9c>
14142fd3f: 48 89 f9                    	mov	rcx, rdi
14142fd42: e8 b9 93 fd ff              	call	0x141409100 <.text+0x1408100>
14142fd47: 48 85 c0                    	test	rax, rax
14142fd4a: 0f 84 f0 01 00 00           	je	0x14142ff40 <.text+0x142ef40>
14142fd50: e9 a6 07 00 00              	jmp	0x1414304fb <.text+0x142f4fb>
14142fd55: 49 83 f8 3a                 	cmp	r8, 0x3a
14142fd59: 0f 85 32 08 00 00           	jne	0x141430591 <.text+0x142f591>
14142fd5f: 48 89 47 28                 	mov	qword ptr [rdi + 0x28], rax
14142fd63: 48 8d 4d 10                 	lea	rcx, [rbp + 0x10]
14142fd67: 48 89 fa                    	mov	rdx, rdi
14142fd6a: e8 91 2e fe ff              	call	0x141412c00 <.text+0x1411c00>
14142fd6f: f6 45 10 01                 	test	byte ptr [rbp + 0x10], 0x1
14142fd73: 0f 85 33 07 00 00           	jne	0x1414304ac <.text+0x142f4ac>
14142fd79: 8b 45 14                    	mov	eax, dword ptr [rbp + 0x14]
14142fd7c: 89 45 e8                    	mov	dword ptr [rbp - 0x18], eax
14142fd7f: c7 45 08 01 00 00 00        	mov	dword ptr [rbp + 0x8], 0x1
14142fd86: 48 8b 4f 20                 	mov	rcx, qword ptr [rdi + 0x20]
14142fd8a: 48 8b 47 28                 	mov	rax, qword ptr [rdi + 0x28]
14142fd8e: 48 39 c8                    	cmp	rax, rcx
14142fd91: 0f 82 ed fb ff ff           	jb	0x14142f984 <.text+0x142e984>
14142fd97: e9 00 02 00 00              	jmp	0x14142ff9c <.text+0x142ef9c>
14142fd9c: 49 83 f8 3a                 	cmp	r8, 0x3a
14142fda0: 0f 85 eb 07 00 00           	jne	0x141430591 <.text+0x142f591>
14142fda6: 48 89 47 28                 	mov	qword ptr [rdi + 0x28], rax
14142fdaa: 48 8d 4d 10                 	lea	rcx, [rbp + 0x10]
14142fdae: 48 89 fa                    	mov	rdx, rdi
14142fdb1: e8 1a d2 fd ff              	call	0x14140cfd0 <.text+0x140bfd0>
14142fdb6: f6 45 10 01                 	test	byte ptr [rbp + 0x10], 0x1
14142fdba: 0f 85 ec 06 00 00           	jne	0x1414304ac <.text+0x142f4ac>
14142fdc0: f3 0f 10 7d 14              	movss	xmm7, dword ptr [rbp + 0x14]
14142fdc5: c7 85 80 00 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x80], 0x1
14142fdcf: 48 8b 4f 20                 	mov	rcx, qword ptr [rdi + 0x20]
14142fdd3: 48 8b 47 28                 	mov	rax, qword ptr [rdi + 0x28]
14142fdd7: 48 39 c8                    	cmp	rax, rcx
14142fdda: 0f 82 a4 fb ff ff           	jb	0x14142f984 <.text+0x142e984>
14142fde0: e9 b7 01 00 00              	jmp	0x14142ff9c <.text+0x142ef9c>
14142fde5: 49 83 f8 3a                 	cmp	r8, 0x3a
14142fde9: 0f 85 a2 07 00 00           	jne	0x141430591 <.text+0x142f591>
14142fdef: 48 89 47 28                 	mov	qword ptr [rdi + 0x28], rax
14142fdf3: 48 8d 4d 10                 	lea	rcx, [rbp + 0x10]
14142fdf7: 48 89 fa                    	mov	rdx, rdi
14142fdfa: e8 71 d4 fd ff              	call	0x14140d270 <.text+0x140c270>
14142fdff: f6 45 10 01                 	test	byte ptr [rbp + 0x10], 0x1
14142fe03: 0f 85 a3 06 00 00           	jne	0x1414304ac <.text+0x142f4ac>
14142fe09: 8b 45 14                    	mov	eax, dword ptr [rbp + 0x14]
14142fe0c: 89 45 e4                    	mov	dword ptr [rbp - 0x1c], eax
14142fe0f: c7 45 f8 01 00 00 00        	mov	dword ptr [rbp - 0x8], 0x1
14142fe16: 48 8b 4f 20                 	mov	rcx, qword ptr [rdi + 0x20]
14142fe1a: 48 8b 47 28                 	mov	rax, qword ptr [rdi + 0x28]
14142fe1e: 48 39 c8                    	cmp	rax, rcx
14142fe21: 0f 82 5d fb ff ff           	jb	0x14142f984 <.text+0x142e984>
14142fe27: e9 70 01 00 00              	jmp	0x14142ff9c <.text+0x142ef9c>
14142fe2c: 49 83 f8 3a                 	cmp	r8, 0x3a
14142fe30: 0f 85 5b 07 00 00           	jne	0x141430591 <.text+0x142f591>
14142fe36: 48 89 47 28                 	mov	qword ptr [rdi + 0x28], rax
14142fe3a: 48 8d 4d 10                 	lea	rcx, [rbp + 0x10]
14142fe3e: 48 89 fa                    	mov	rdx, rdi
14142fe41: e8 ba 2d fe ff              	call	0x141412c00 <.text+0x1411c00>
14142fe46: f6 45 10 01                 	test	byte ptr [rbp + 0x10], 0x1
14142fe4a: 0f 85 5c 06 00 00           	jne	0x1414304ac <.text+0x142f4ac>
14142fe50: 8b 45 14                    	mov	eax, dword ptr [rbp + 0x14]
14142fe53: 89 45 f0                    	mov	dword ptr [rbp - 0x10], eax
14142fe56: c7 45 fc 01 00 00 00        	mov	dword ptr [rbp - 0x4], 0x1
14142fe5d: 48 8b 4f 20                 	mov	rcx, qword ptr [rdi + 0x20]
14142fe61: 48 8b 47 28                 	mov	rax, qword ptr [rdi + 0x28]
14142fe65: 48 39 c8                    	cmp	rax, rcx
14142fe68: 0f 82 16 fb ff ff           	jb	0x14142f984 <.text+0x142e984>
14142fe6e: e9 29 01 00 00              	jmp	0x14142ff9c <.text+0x142ef9c>
14142fe73: 49 83 f8 3a                 	cmp	r8, 0x3a
14142fe77: 0f 85 14 07 00 00           	jne	0x141430591 <.text+0x142f591>
14142fe7d: 48 89 47 28                 	mov	qword ptr [rdi + 0x28], rax
14142fe81: 48 8d 4d 10                 	lea	rcx, [rbp + 0x10]
14142fe85: 48 89 fa                    	mov	rdx, rdi
14142fe88: e8 93 39 fe ff              	call	0x141413820 <.text+0x1412820>
14142fe8d: f6 45 10 01                 	test	byte ptr [rbp + 0x10], 0x1
14142fe91: 0f 85 15 06 00 00           	jne	0x1414304ac <.text+0x142f4ac>
14142fe97: 8b 45 14                    	mov	eax, dword ptr [rbp + 0x14]
14142fe9a: 89 45 0c                    	mov	dword ptr [rbp + 0xc], eax
14142fe9d: 48 8b 45 18                 	mov	rax, qword ptr [rbp + 0x18]
14142fea1: 48 89 45 d0                 	mov	qword ptr [rbp - 0x30], rax
14142fea5: 41 bc 01 00 00 00           	mov	r12d, 0x1
14142feab: 48 8b 4f 20                 	mov	rcx, qword ptr [rdi + 0x20]
14142feaf: 48 8b 47 28                 	mov	rax, qword ptr [rdi + 0x28]
14142feb3: 48 39 c8                    	cmp	rax, rcx
14142feb6: 0f 82 c8 fa ff ff           	jb	0x14142f984 <.text+0x142e984>
14142febc: e9 db 00 00 00              	jmp	0x14142ff9c <.text+0x142ef9c>
14142fec1: 49 83 f8 3a                 	cmp	r8, 0x3a
14142fec5: 0f 85 c6 06 00 00           	jne	0x141430591 <.text+0x142f591>
14142fecb: 48 89 47 28                 	mov	qword ptr [rdi + 0x28], rax
14142fecf: 48 8d 4d 10                 	lea	rcx, [rbp + 0x10]
14142fed3: 48 89 fa                    	mov	rdx, rdi
14142fed6: e8 f5 d0 fd ff              	call	0x14140cfd0 <.text+0x140bfd0>
14142fedb: f6 45 10 01                 	test	byte ptr [rbp + 0x10], 0x1
14142fedf: 0f 85 c7 05 00 00           	jne	0x1414304ac <.text+0x142f4ac>
14142fee5: f3 0f 10 75 14              	movss	xmm6, dword ptr [rbp + 0x14]
14142feea: c7 45 40 01 00 00 00        	mov	dword ptr [rbp + 0x40], 0x1
14142fef1: 48 8b 4f 20                 	mov	rcx, qword ptr [rdi + 0x20]
14142fef5: 48 8b 47 28                 	mov	rax, qword ptr [rdi + 0x28]
14142fef9: 48 39 c8                    	cmp	rax, rcx
14142fefc: 0f 82 82 fa ff ff           	jb	0x14142f984 <.text+0x142e984>
14142ff02: e9 95 00 00 00              	jmp	0x14142ff9c <.text+0x142ef9c>
14142ff07: 49 83 f8 3a                 	cmp	r8, 0x3a
14142ff0b: 0f 85 80 06 00 00           	jne	0x141430591 <.text+0x142f591>
14142ff11: 48 89 47 28                 	mov	qword ptr [rdi + 0x28], rax
14142ff15: 48 8d 4d 10                 	lea	rcx, [rbp + 0x10]
14142ff19: 48 89 fa                    	mov	rdx, rdi
14142ff1c: e8 df 2c fe ff              	call	0x141412c00 <.text+0x1411c00>
14142ff21: f6 45 10 01                 	test	byte ptr [rbp + 0x10], 0x1
14142ff25: 0f 85 81 05 00 00           	jne	0x1414304ac <.text+0x142f4ac>
14142ff2b: 8b 45 14                    	mov	eax, dword ptr [rbp + 0x14]
14142ff2e: 89 45 ec                    	mov	dword ptr [rbp - 0x14], eax
14142ff31: c7 45 04 01 00 00 00        	mov	dword ptr [rbp + 0x4], 0x1
14142ff38: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
14142ff40: 48 8b 4f 20                 	mov	rcx, qword ptr [rdi + 0x20]
14142ff44: 48 8b 47 28                 	mov	rax, qword ptr [rdi + 0x28]
14142ff48: 48 39 c8                    	cmp	rax, rcx
14142ff4b: 0f 82 33 fa ff ff           	jb	0x14142f984 <.text+0x142e984>
14142ff51: eb 49                       	jmp	0x14142ff9c <.text+0x142ef9c>
14142ff53: 49 83 f8 3a                 	cmp	r8, 0x3a
14142ff57: 0f 85 34 06 00 00           	jne	0x141430591 <.text+0x142f591>
14142ff5d: 48 89 47 28                 	mov	qword ptr [rdi + 0x28], rax
14142ff61: 48 8d 4d 10                 	lea	rcx, [rbp + 0x10]
14142ff65: 48 89 fa                    	mov	rdx, rdi
14142ff68: e8 43 33 fe ff              	call	0x1414132b0 <.text+0x14122b0>
14142ff6d: f6 45 10 01                 	test	byte ptr [rbp + 0x10], 0x1
14142ff71: 0f 85 35 05 00 00           	jne	0x1414304ac <.text+0x142f4ac>
14142ff77: 8b 45 14                    	mov	eax, dword ptr [rbp + 0x14]
14142ff7a: 89 45 e0                    	mov	dword ptr [rbp - 0x20], eax
14142ff7d: 8b 45 18                    	mov	eax, dword ptr [rbp + 0x18]
14142ff80: 48 89 45 d8                 	mov	qword ptr [rbp - 0x28], rax
14142ff84: c7 45 00 01 00 00 00        	mov	dword ptr [rbp], 0x1
14142ff8b: 48 8b 4f 20                 	mov	rcx, qword ptr [rdi + 0x20]
14142ff8f: 48 8b 47 28                 	mov	rax, qword ptr [rdi + 0x28]
14142ff93: 48 39 c8                    	cmp	rax, rcx
14142ff96: 0f 82 e8 f9 ff ff           	jb	0x14142f984 <.text+0x142e984>
14142ff9c: 48 c7 45 48 03 00 00 00     	mov	qword ptr [rbp + 0x48], 0x3
14142ffa4: e9 d3 02 00 00              	jmp	0x14143027c <.text+0x142f27c>
14142ffa9: fe 4f 30                    	dec	byte ptr [rdi + 0x30]
14142ffac: 0f 84 c5 05 00 00           	je	0x141430577 <.text+0x142f577>
14142ffb2: 48 89 47 28                 	mov	qword ptr [rdi + 0x28], rax
14142ffb6: 48 89 7d 10                 	mov	qword ptr [rbp + 0x10], rdi
14142ffba: c6 45 18 01                 	mov	byte ptr [rbp + 0x18], 0x1
14142ffbe: 48 8d 4d 48                 	lea	rcx, [rbp + 0x48]
14142ffc2: 48 8d 55 10                 	lea	rdx, [rbp + 0x10]
14142ffc6: e8 45 a5 fd ff              	call	0x14140a510 <.text+0x1409510>
14142ffcb: 8b 45 48                    	mov	eax, dword ptr [rbp + 0x48]
14142ffce: 83 f8 02                    	cmp	eax, 0x2
14142ffd1: 0f 84 13 01 00 00           	je	0x1414300ea <.text+0x142f0ea>
14142ffd7: 85 c0                       	test	eax, eax
14142ffd9: 0f 84 89 04 00 00           	je	0x141430468 <.text+0x142f468>
14142ffdf: 44 8b 75 4c                 	mov	r14d, dword ptr [rbp + 0x4c]
14142ffe3: 48 8b 5d 50                 	mov	rbx, qword ptr [rbp + 0x50]
14142ffe7: 48 8d 4d 48                 	lea	rcx, [rbp + 0x48]
14142ffeb: 48 8d 55 10                 	lea	rdx, [rbp + 0x10]
14142ffef: e8 5c c0 fd ff              	call	0x14140c050 <.text+0x140b050>
14142fff4: f6 45 48 01                 	test	byte ptr [rbp + 0x48], 0x1
14142fff8: 0f 85 ec 00 00 00           	jne	0x1414300ea <.text+0x142f0ea>
14142fffe: f6 45 4c 01                 	test	byte ptr [rbp + 0x4c], 0x1
141430002: 0f 84 ad 04 00 00           	je	0x1414304b5 <.text+0x142f4b5>
141430008: f3 0f 10 75 50              	movss	xmm6, dword ptr [rbp + 0x50]
14143000d: 48 8d 4d 48                 	lea	rcx, [rbp + 0x48]
141430011: 48 8d 55 10                 	lea	rdx, [rbp + 0x10]
141430015: e8 36 c0 fd ff              	call	0x14140c050 <.text+0x140b050>
14143001a: f6 45 48 01                 	test	byte ptr [rbp + 0x48], 0x1
14143001e: 0f 85 c6 00 00 00           	jne	0x1414300ea <.text+0x142f0ea>
141430024: f6 45 4c 01                 	test	byte ptr [rbp + 0x4c], 0x1
141430028: 0f 84 9f 04 00 00           	je	0x1414304cd <.text+0x142f4cd>
14143002e: f3 0f 10 7d 50              	movss	xmm7, dword ptr [rbp + 0x50]
141430033: 48 8d 4d 48                 	lea	rcx, [rbp + 0x48]
141430037: 48 8d 55 10                 	lea	rdx, [rbp + 0x10]
14143003b: e8 00 a2 fd ff              	call	0x14140a240 <.text+0x1409240>
141430040: f6 45 48 01                 	test	byte ptr [rbp + 0x48], 0x1
141430044: 0f 85 a0 00 00 00           	jne	0x1414300ea <.text+0x142f0ea>
14143004a: f6 45 4c 01                 	test	byte ptr [rbp + 0x4c], 0x1
14143004e: 0f 84 03 05 00 00           	je	0x141430557 <.text+0x142f557>
141430054: 44 8b 7d 50                 	mov	r15d, dword ptr [rbp + 0x50]
141430058: 48 8d 4d 48                 	lea	rcx, [rbp + 0x48]
14143005c: 48 8d 55 10                 	lea	rdx, [rbp + 0x10]
141430060: e8 3b aa fd ff              	call	0x14140aaa0 <.text+0x1409aa0>
141430065: f6 45 48 01                 	test	byte ptr [rbp + 0x48], 0x1
141430069: 75 7f                       	jne	0x1414300ea <.text+0x142f0ea>
14143006b: 45 31 ed                    	xor	r13d, r13d
14143006e: 80 7d 4c 00                 	cmp	byte ptr [rbp + 0x4c], 0x0
141430072: 74 04                       	je	0x141430078 <.text+0x142f078>
141430074: 44 8b 6d 50                 	mov	r13d, dword ptr [rbp + 0x50]
141430078: 48 8d 4d 48                 	lea	rcx, [rbp + 0x48]
14143007c: 48 8d 55 10                 	lea	rdx, [rbp + 0x10]
141430080: e8 1b aa fd ff              	call	0x14140aaa0 <.text+0x1409aa0>
141430085: f6 45 48 01                 	test	byte ptr [rbp + 0x48], 0x1
141430089: 75 5f                       	jne	0x1414300ea <.text+0x142f0ea>
14143008b: 44 89 75 0c                 	mov	dword ptr [rbp + 0xc], r14d
14143008f: 45 31 e4                    	xor	r12d, r12d
141430092: 80 7d 4c 00                 	cmp	byte ptr [rbp + 0x4c], 0x0
141430096: 74 04                       	je	0x14143009c <.text+0x142f09c>
141430098: 44 8b 65 50                 	mov	r12d, dword ptr [rbp + 0x50]
14143009c: f3 0f 10 05 60 58 4f 01     	movss	xmm0, dword ptr [rip + 0x14f5860] # 0x142925904
1414300a4: 0f 28 c8                    	movaps	xmm1, xmm0
1414300a7: e8 44 0d 86 ff              	call	0x140c90df0 <.text+0xc8fdf0>
1414300ac: 41 89 c6                    	mov	r14d, eax
1414300af: 48 8d 4d 48                 	lea	rcx, [rbp + 0x48]
1414300b3: 48 8d 55 10                 	lea	rdx, [rbp + 0x10]
1414300b7: e8 24 a7 fd ff              	call	0x14140a7e0 <.text+0x14097e0>
1414300bc: f6 45 48 01                 	test	byte ptr [rbp + 0x48], 0x1
1414300c0: 75 28                       	jne	0x1414300ea <.text+0x142f0ea>
1414300c2: 8b 45 4c                    	mov	eax, dword ptr [rbp + 0x4c]
1414300c5: 89 85 80 00 00 00           	mov	dword ptr [rbp + 0x80], eax
1414300cb: 48 8b 45 50                 	mov	rax, qword ptr [rbp + 0x50]
1414300cf: 48 89 45 40                 	mov	qword ptr [rbp + 0x40], rax
1414300d3: 48 8d 4d 48                 	lea	rcx, [rbp + 0x48]
1414300d7: 48 8d 55 10                 	lea	rdx, [rbp + 0x10]
1414300db: e8 c0 a9 fd ff              	call	0x14140aaa0 <.text+0x1409aa0>
1414300e0: f6 45 48 01                 	test	byte ptr [rbp + 0x48], 0x1
1414300e4: 0f 84 36 04 00 00           	je	0x141430520 <.text+0x142f520>
1414300ea: 48 8b 5d 50                 	mov	rbx, qword ptr [rbp + 0x50]
1414300ee: 41 bf 01 00 00 00           	mov	r15d, 0x1
1414300f4: 48 c7 85 80 00 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x80], 0x0
1414300ff: fe 47 30                    	inc	byte ptr [rdi + 0x30]
141430102: 44 89 7d 48                 	mov	dword ptr [rbp + 0x48], r15d
141430106: 41 89 cd                    	mov	r13d, ecx
141430109: 89 4d 4c                    	mov	dword ptr [rbp + 0x4c], ecx
14143010c: 48 89 5d 50                 	mov	qword ptr [rbp + 0x50], rbx
141430110: f3 0f 11 75 58              	movss	dword ptr [rbp + 0x58], xmm6
141430115: f3 0f 11 7d 5c              	movss	dword ptr [rbp + 0x5c], xmm7
14143011a: 89 55 60                    	mov	dword ptr [rbp + 0x60], edx
14143011d: 44 89 45 64                 	mov	dword ptr [rbp + 0x64], r8d
141430121: 44 89 65 68                 	mov	dword ptr [rbp + 0x68], r12d
141430125: 44 89 75 6c                 	mov	dword ptr [rbp + 0x6c], r14d
141430129: 4c 89 4d 70                 	mov	qword ptr [rbp + 0x70], r9
14143012d: 89 45 78                    	mov	dword ptr [rbp + 0x78], eax
141430130: 48 89 f9                    	mov	rcx, rdi
141430133: e8 b8 83 c2 fe              	call	0x1400584f0 <.text+0x574f0>
141430138: 48 8d 55 58                 	lea	rdx, [rbp + 0x58]
14143013c: 8b 4a 20                    	mov	ecx, dword ptr [rdx + 0x20]
14143013f: 89 4d 30                    	mov	dword ptr [rbp + 0x30], ecx
141430142: 0f 10 02                    	movups	xmm0, xmmword ptr [rdx]
141430145: 0f 10 4a 10                 	movups	xmm1, xmmword ptr [rdx + 0x10]
141430149: 0f 29 4d 20                 	movaps	xmmword ptr [rbp + 0x20], xmm1
14143014d: 0f 29 45 10                 	movaps	xmmword ptr [rbp + 0x10], xmm0
141430151: 45 85 ff                    	test	r15d, r15d
141430154: 45 89 ec                    	mov	r12d, r13d
141430157: 41 be 01 00 00 00           	mov	r14d, 0x1
14143015d: 0f 84 ea 00 00 00           	je	0x14143024d <.text+0x142f24d>
141430163: 48 85 c0                    	test	rax, rax
141430166: 0f 84 2a 02 00 00           	je	0x141430396 <.text+0x142f396>
14143016c: 48 8b 08                    	mov	rcx, qword ptr [rax]
14143016f: 48 83 f9 01                 	cmp	rcx, 0x1
141430173: 48 89 45 40                 	mov	qword ptr [rbp + 0x40], rax
141430177: 0f 84 f9 01 00 00           	je	0x141430376 <.text+0x142f376>
14143017d: 48 85 c9                    	test	rcx, rcx
141430180: 0f 85 fc 01 00 00           	jne	0x141430382 <.text+0x142f382>
141430186: 48 8b 45 40                 	mov	rax, qword ptr [rbp + 0x40]
14143018a: 48 8b 50 10                 	mov	rdx, qword ptr [rax + 0x10]
14143018e: 48 85 d2                    	test	rdx, rdx
141430191: 0f 84 eb 01 00 00           	je	0x141430382 <.text+0x142f382>
141430197: 48 8b 45 40                 	mov	rax, qword ptr [rbp + 0x40]
14143019b: 48 8b 48 08                 	mov	rcx, qword ptr [rax + 0x8]
14143019f: 41 b8 01 00 00 00           	mov	r8d, 0x1
1414301a5: e8 76 3a 1e ff              	call	0x140613c20 <.text+0x612c20>
1414301aa: e9 d3 01 00 00              	jmp	0x141430382 <.text+0x142f382>
1414301af: 41 f6 c4 01                 	test	r12b, 0x1
1414301b3: 48 8b 45 d8                 	mov	rax, qword ptr [rbp - 0x28]
1414301b7: 8b 4d e0                    	mov	ecx, dword ptr [rbp - 0x20]
1414301ba: 0f 84 4a 02 00 00           	je	0x14143040a <.text+0x142f40a>
1414301c0: f6 45 40 01                 	test	byte ptr [rbp + 0x40], 0x1
1414301c4: 0f 84 56 02 00 00           	je	0x141430420 <.text+0x142f420>
1414301ca: f6 85 80 00 00 00 01        	test	byte ptr [rbp + 0x80], 0x1
1414301d1: 0f 84 7b 02 00 00           	je	0x141430452 <.text+0x142f452>
1414301d7: f6 45 f8 01                 	test	byte ptr [rbp - 0x8], 0x1
1414301db: 0f 84 b5 02 00 00           	je	0x141430496 <.text+0x142f496>
1414301e1: 45 31 ed                    	xor	r13d, r13d
1414301e4: f6 45 08 01                 	test	byte ptr [rbp + 0x8], 0x1
1414301e8: 44 8b 7d e8                 	mov	r15d, dword ptr [rbp - 0x18]
1414301ec: 45 0f 44 fd                 	cmove	r15d, r13d
1414301f0: f6 45 04 01                 	test	byte ptr [rbp + 0x4], 0x1
1414301f4: 44 8b 65 ec                 	mov	r12d, dword ptr [rbp - 0x14]
1414301f8: 45 0f 44 e5                 	cmove	r12d, r13d
1414301fc: 48 c1 e0 20                 	shl	rax, 0x20
141430200: 41 89 ce                    	mov	r14d, ecx
141430203: 49 09 c6                    	or	r14, rax
141430206: f6 45 00 01                 	test	byte ptr [rbp], 0x1
14143020a: 4d 0f 44 f5                 	cmove	r14, r13
14143020e: f6 45 fc 01                 	test	byte ptr [rbp - 0x4], 0x1
141430212: 48 89 f3                    	mov	rbx, rsi
141430215: 8b 75 f0                    	mov	esi, dword ptr [rbp - 0x10]
141430218: 41 0f 44 f5                 	cmove	esi, r13d
14143021c: f3 0f 10 05 e0 56 4f 01     	movss	xmm0, dword ptr [rip + 0x14f56e0] # 0x142925904
141430224: 0f 28 c8                    	movaps	xmm1, xmm0
141430227: e8 c4 0b 86 ff              	call	0x140c90df0 <.text+0xc8fdf0>
14143022c: 45 89 e0                    	mov	r8d, r12d
14143022f: 41 89 f1                    	mov	r9d, esi
141430232: 48 89 de                    	mov	rsi, rbx
141430235: 44 89 fa                    	mov	edx, r15d
141430238: b1 01                       	mov	cl, 0x1
14143023a: 89 8d 80 00 00 00           	mov	dword ptr [rbp + 0x80], ecx
141430240: 4c 8b 7d d0                 	mov	r15, qword ptr [rbp - 0x30]
141430244: 8b 4d e4                    	mov	ecx, dword ptr [rbp - 0x1c]
141430247: 44 8b 65 0c                 	mov	r12d, dword ptr [rbp + 0xc]
14143024b: eb 4e                       	jmp	0x14143029b <.text+0x142f29b>
14143024d: 48 85 c0                    	test	rax, rax
141430250: 0f 85 43 01 00 00           	jne	0x141430399 <.text+0x142f399>
141430256: 8b 45 30                    	mov	eax, dword ptr [rbp + 0x30]
141430259: 89 45 c0                    	mov	dword ptr [rbp - 0x40], eax
14143025c: 0f 28 45 10                 	movaps	xmm0, xmmword ptr [rbp + 0x10]
141430260: 0f 28 4d 20                 	movaps	xmm1, xmmword ptr [rbp + 0x20]
141430264: 0f 29 4d b0                 	movaps	xmmword ptr [rbp - 0x50], xmm1
141430268: 0f 29 45 a0                 	movaps	xmmword ptr [rbp - 0x60], xmm0
14143026c: 45 31 f6                    	xor	r14d, r14d
14143026f: e9 22 01 00 00              	jmp	0x141430396 <.text+0x142f396>
141430274: 48 c7 45 48 05 00 00 00     	mov	qword ptr [rbp + 0x48], 0x5
14143027c: 48 8d 55 48                 	lea	rdx, [rbp + 0x48]
141430280: 48 89 f9                    	mov	rcx, rdi
141430283: e8 98 06 47 01              	call	0x1428a0920 <.text+0x289f920>
141430288: 49 89 c7                    	mov	r15, rax
14143028b: 41 bd 01 00 00 00           	mov	r13d, 0x1
141430291: c7 85 80 00 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x80], 0x0
14143029b: fe 47 30                    	inc	byte ptr [rdi + 0x30]
14143029e: 44 89 6d 48                 	mov	dword ptr [rbp + 0x48], r13d
1414302a2: 44 89 65 4c                 	mov	dword ptr [rbp + 0x4c], r12d
1414302a6: 4c 89 7d 50                 	mov	qword ptr [rbp + 0x50], r15
1414302aa: f3 0f 11 75 58              	movss	dword ptr [rbp + 0x58], xmm6
1414302af: f3 0f 11 7d 5c              	movss	dword ptr [rbp + 0x5c], xmm7
1414302b4: 89 4d 60                    	mov	dword ptr [rbp + 0x60], ecx
1414302b7: 89 55 64                    	mov	dword ptr [rbp + 0x64], edx
1414302ba: 44 89 45 68                 	mov	dword ptr [rbp + 0x68], r8d
1414302be: 89 45 6c                    	mov	dword ptr [rbp + 0x6c], eax
1414302c1: 4c 89 75 70                 	mov	qword ptr [rbp + 0x70], r14
1414302c5: 44 89 4d 78                 	mov	dword ptr [rbp + 0x78], r9d
1414302c9: 48 89 f9                    	mov	rcx, rdi
1414302cc: e8 7f 81 c2 fe              	call	0x140058450 <.text+0x57450>
1414302d1: 48 8d 55 58                 	lea	rdx, [rbp + 0x58]
1414302d5: 8b 4a 20                    	mov	ecx, dword ptr [rdx + 0x20]
1414302d8: 89 4d 30                    	mov	dword ptr [rbp + 0x30], ecx
1414302db: 0f 10 02                    	movups	xmm0, xmmword ptr [rdx]
1414302de: 0f 10 4a 10                 	movups	xmm1, xmmword ptr [rdx + 0x10]
1414302e2: 0f 29 4d 20                 	movaps	xmmword ptr [rbp + 0x20], xmm1
1414302e6: 0f 29 45 10                 	movaps	xmmword ptr [rbp + 0x10], xmm0
1414302ea: 41 be 01 00 00 00           	mov	r14d, 0x1
1414302f0: 45 85 ed                    	test	r13d, r13d
1414302f3: 74 39                       	je	0x14143032e <.text+0x142f32e>
1414302f5: 48 85 c0                    	test	rax, rax
1414302f8: 74 52                       	je	0x14143034c <.text+0x142f34c>
1414302fa: 48 8b 08                    	mov	rcx, qword ptr [rax]
1414302fd: 48 83 f9 01                 	cmp	rcx, 0x1
141430301: 48 89 45 40                 	mov	qword ptr [rbp + 0x40], rax
141430305: 74 4a                       	je	0x141430351 <.text+0x142f351>
141430307: 48 85 c9                    	test	rcx, rcx
14143030a: 75 51                       	jne	0x14143035d <.text+0x142f35d>
14143030c: 48 8b 45 40                 	mov	rax, qword ptr [rbp + 0x40]
141430310: 48 8b 50 10                 	mov	rdx, qword ptr [rax + 0x10]
141430314: 48 85 d2                    	test	rdx, rdx
141430317: 74 44                       	je	0x14143035d <.text+0x142f35d>
141430319: 48 8b 45 40                 	mov	rax, qword ptr [rbp + 0x40]
14143031d: 48 8b 48 08                 	mov	rcx, qword ptr [rax + 0x8]
141430321: 41 b8 01 00 00 00           	mov	r8d, 0x1
141430327: e8 f4 38 1e ff              	call	0x140613c20 <.text+0x612c20>
14143032c: eb 2f                       	jmp	0x14143035d <.text+0x142f35d>
14143032e: 48 85 c0                    	test	rax, rax
141430331: 75 66                       	jne	0x141430399 <.text+0x142f399>
141430333: 8b 45 30                    	mov	eax, dword ptr [rbp + 0x30]
141430336: 89 45 c0                    	mov	dword ptr [rbp - 0x40], eax
141430339: 0f 28 45 10                 	movaps	xmm0, xmmword ptr [rbp + 0x10]
14143033d: 0f 28 4d 20                 	movaps	xmm1, xmmword ptr [rbp + 0x20]
141430341: 0f 29 4d b0                 	movaps	xmmword ptr [rbp - 0x50], xmm1
141430345: 0f 29 45 a0                 	movaps	xmmword ptr [rbp - 0x60], xmm0
141430349: 45 31 f6                    	xor	r14d, r14d
14143034c: 4c 89 f8                    	mov	rax, r15
14143034f: eb 48                       	jmp	0x141430399 <.text+0x142f399>
141430351: 48 89 c1                    	mov	rcx, rax
141430354: 48 83 c1 08                 	add	rcx, 0x8
141430358: e8 a3 76 1b ff              	call	0x1405e7a00 <.text+0x5e6a00>
14143035d: ba 28 00 00 00              	mov	edx, 0x28
141430362: 41 b8 08 00 00 00           	mov	r8d, 0x8
141430368: 48 8b 4d 40                 	mov	rcx, qword ptr [rbp + 0x40]
14143036c: e8 af 38 1e ff              	call	0x140613c20 <.text+0x612c20>
141430371: 4c 89 f8                    	mov	rax, r15
141430374: eb 23                       	jmp	0x141430399 <.text+0x142f399>
141430376: 48 89 c1                    	mov	rcx, rax
141430379: 48 83 c1 08                 	add	rcx, 0x8
14143037d: e8 7e 76 1b ff              	call	0x1405e7a00 <.text+0x5e6a00>
141430382: ba 28 00 00 00              	mov	edx, 0x28
141430387: 41 b8 08 00 00 00           	mov	r8d, 0x8
14143038d: 48 8b 4d 40                 	mov	rcx, qword ptr [rbp + 0x40]
141430391: e8 8a 38 1e ff              	call	0x140613c20 <.text+0x612c20>
141430396: 48 89 d8                    	mov	rax, rbx
141430399: 45 85 f6                    	test	r14d, r14d
14143039c: 75 5a                       	jne	0x1414303f8 <.text+0x142f3f8>
14143039e: 8b 4d c0                    	mov	ecx, dword ptr [rbp - 0x40]
1414303a1: 89 4e 30                    	mov	dword ptr [rsi + 0x30], ecx
1414303a4: 0f 28 45 a0                 	movaps	xmm0, xmmword ptr [rbp - 0x60]
1414303a8: 0f 28 4d b0                 	movaps	xmm1, xmmword ptr [rbp - 0x50]
1414303ac: 0f 11 4e 20                 	movups	xmmword ptr [rsi + 0x20], xmm1
1414303b0: 0f 11 46 10                 	movups	xmmword ptr [rsi + 0x10], xmm0
1414303b4: 44 89 66 04                 	mov	dword ptr [rsi + 0x4], r12d
1414303b8: 31 c9                       	xor	ecx, ecx
1414303ba: 48 89 46 08                 	mov	qword ptr [rsi + 0x8], rax
1414303be: 89 0e                       	mov	dword ptr [rsi], ecx
1414303c0: 48 89 f0                    	mov	rax, rsi
1414303c3: 0f 28 b5 90 00 00 00        	movaps	xmm6, xmmword ptr [rbp + 0x90]
1414303ca: 0f 28 bd a0 00 00 00        	movaps	xmm7, xmmword ptr [rbp + 0xa0]
1414303d1: 48 81 c4 38 01 00 00        	add	rsp, 0x138
1414303d8: 5b                          	pop	rbx
1414303d9: 5f                          	pop	rdi
1414303da: 5e                          	pop	rsi
1414303db: 41 5c                       	pop	r12
1414303dd: 41 5d                       	pop	r13
1414303df: 41 5e                       	pop	r14
1414303e1: 41 5f                       	pop	r15
1414303e3: 5d                          	pop	rbp
1414303e4: c3                          	ret
1414303e5: 4c 8d 05 24 c3 82 01        	lea	r8, [rip + 0x182c324]   # 0x142c5c710
1414303ec: 48 8d 55 f7                 	lea	rdx, [rbp - 0x9]
1414303f0: 48 89 f9                    	mov	rcx, rdi
1414303f3: e8 08 88 4c 01              	call	0x1428f8c00 <.text+0x28f7c00>
1414303f8: 48 89 c1                    	mov	rcx, rax
1414303fb: 48 89 fa                    	mov	rdx, rdi
1414303fe: e8 fd 1d 47 01              	call	0x1428a2200 <.text+0x28a1200>
141430403: b9 01 00 00 00              	mov	ecx, 0x1
141430408: eb b0                       	jmp	0x1414303ba <.text+0x142f3ba>
14143040a: 48 8d 0d 37 c9 82 01        	lea	rcx, [rip + 0x182c937]  # 0x142c5cd48
141430411: ba 08 00 00 00              	mov	edx, 0x8
141430416: e8 a5 97 4c 01              	call	0x1428f9bc0 <.text+0x28f8bc0>
14143041b: e9 68 fe ff ff              	jmp	0x141430288 <.text+0x142f288>
141430420: 48 8d 0d 01 cc 82 01        	lea	rcx, [rip + 0x182cc01]  # 0x142c5d028
141430427: ba 05 00 00 00              	mov	edx, 0x5
14143042c: e8 8f 97 4c 01              	call	0x1428f9bc0 <.text+0x28f8bc0>
141430431: e9 52 fe ff ff              	jmp	0x141430288 <.text+0x142f288>
141430436: 48 8b 55 18                 	mov	rdx, qword ptr [rbp + 0x18]
14143043a: 41 bd 01 00 00 00           	mov	r13d, 0x1
141430440: c7 85 80 00 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x80], 0x0
14143044a: 49 89 d7                    	mov	r15, rdx
14143044d: e9 49 fe ff ff              	jmp	0x14143029b <.text+0x142f29b>
141430452: 48 8d 0d d4 cb 82 01        	lea	rcx, [rip + 0x182cbd4]  # 0x142c5d02d
141430459: ba 10 00 00 00              	mov	edx, 0x10
14143045e: e8 5d 97 4c 01              	call	0x1428f9bc0 <.text+0x28f8bc0>
141430463: e9 20 fe ff ff              	jmp	0x141430288 <.text+0x142f288>
141430468: 48 8d 15 a9 cb 82 01        	lea	rdx, [rip + 0x182cba9]  # 0x142c5d018
14143046f: 4c 8d 05 7a ba 82 01        	lea	r8, [rip + 0x182ba7a]   # 0x142c5bef0
141430476: 48 c7 85 80 00 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x80], 0x0
141430481: 31 c9                       	xor	ecx, ecx
141430483: e8 98 98 4c 01              	call	0x1428f9d20 <.text+0x28f8d20>
141430488: 48 89 c3                    	mov	rbx, rax
14143048b: 41 bf 01 00 00 00           	mov	r15d, 0x1
141430491: e9 69 fc ff ff              	jmp	0x1414300ff <.text+0x142f0ff>
141430496: 48 8d 0d a0 cb 82 01        	lea	rcx, [rip + 0x182cba0]  # 0x142c5d03d
14143049d: ba 02 00 00 00              	mov	edx, 0x2
1414304a2: e8 19 97 4c 01              	call	0x1428f9bc0 <.text+0x28f8bc0>
1414304a7: e9 dc fd ff ff              	jmp	0x141430288 <.text+0x142f288>
1414304ac: 4c 8b 7d 18                 	mov	r15, qword ptr [rbp + 0x18]
1414304b0: e9 d6 fd ff ff              	jmp	0x14143028b <.text+0x142f28b>
1414304b5: 48 8d 15 5c cb 82 01        	lea	rdx, [rip + 0x182cb5c]  # 0x142c5d018
1414304bc: 4c 8d 05 2d ba 82 01        	lea	r8, [rip + 0x182ba2d]   # 0x142c5bef0
1414304c3: b9 01 00 00 00              	mov	ecx, 0x1
1414304c8: e9 9d 00 00 00              	jmp	0x14143056a <.text+0x142f56a>
1414304cd: 48 8d 15 44 cb 82 01        	lea	rdx, [rip + 0x182cb44]  # 0x142c5d018
1414304d4: 4c 8d 05 15 ba 82 01        	lea	r8, [rip + 0x182ba15]   # 0x142c5bef0
1414304db: b9 02 00 00 00              	mov	ecx, 0x2
1414304e0: e9 85 00 00 00              	jmp	0x14143056a <.text+0x142f56a>
1414304e5: 41 0f b6 c0                 	movzx	eax, r8b
1414304e9: 83 f8 7d                    	cmp	eax, 0x7d
1414304ec: 75 25                       	jne	0x141430513 <.text+0x142f513>
1414304ee: 48 c7 45 48 12 00 00 00     	mov	qword ptr [rbp + 0x48], 0x12
1414304f6: e9 81 fd ff ff              	jmp	0x14143027c <.text+0x142f27c>
1414304fb: 41 bd 01 00 00 00           	mov	r13d, 0x1
141430501: c7 85 80 00 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x80], 0x0
14143050b: 49 89 c7                    	mov	r15, rax
14143050e: e9 88 fd ff ff              	jmp	0x14143029b <.text+0x142f29b>
141430513: 48 c7 45 48 10 00 00 00     	mov	qword ptr [rbp + 0x48], 0x10
14143051b: e9 5c fd ff ff              	jmp	0x14143027c <.text+0x142f27c>
141430520: 44 89 fa                    	mov	edx, r15d
141430523: 45 31 ff                    	xor	r15d, r15d
141430526: 83 bd 80 00 00 00 00        	cmp	dword ptr [rbp + 0x80], 0x0
14143052d: 4c 8b 4d 40                 	mov	r9, qword ptr [rbp + 0x40]
141430531: 4d 0f 44 cf                 	cmove	r9, r15
141430535: 80 7d 4c 00                 	cmp	byte ptr [rbp + 0x4c], 0x0
141430539: b8 00 00 00 00              	mov	eax, 0x0
14143053e: 74 03                       	je	0x141430543 <.text+0x142f543>
141430540: 8b 45 50                    	mov	eax, dword ptr [rbp + 0x50]
141430543: b1 01                       	mov	cl, 0x1
141430545: 48 89 8d 80 00 00 00        	mov	qword ptr [rbp + 0x80], rcx
14143054c: 8b 4d 0c                    	mov	ecx, dword ptr [rbp + 0xc]
14143054f: 45 89 e8                    	mov	r8d, r13d
141430552: e9 a8 fb ff ff              	jmp	0x1414300ff <.text+0x142f0ff>
141430557: 48 8d 15 ba ca 82 01        	lea	rdx, [rip + 0x182caba]  # 0x142c5d018
14143055e: 4c 8d 05 8b b9 82 01        	lea	r8, [rip + 0x182b98b]   # 0x142c5bef0
141430565: b9 03 00 00 00              	mov	ecx, 0x3
14143056a: e8 b1 97 4c 01              	call	0x1428f9d20 <.text+0x28f8d20>
14143056f: 48 89 c3                    	mov	rbx, rax
141430572: e9 77 fb ff ff              	jmp	0x1414300ee <.text+0x142f0ee>
141430577: 48 c7 45 48 15 00 00 00     	mov	qword ptr [rbp + 0x48], 0x15
14143057f: e9 7a f3 ff ff              	jmp	0x14142f8fe <.text+0x142e8fe>
141430584: 48 c7 45 48 08 00 00 00     	mov	qword ptr [rbp + 0x48], 0x8
14143058c: e9 eb fc ff ff              	jmp	0x14143027c <.text+0x142f27c>
141430591: 48 c7 45 48 06 00 00 00     	mov	qword ptr [rbp + 0x48], 0x6
141430599: e9 de fc ff ff              	jmp	0x14143027c <.text+0x142f27c>
14143059e: 48 8d 0d b3 ca 82 01        	lea	rcx, [rip + 0x182cab3]  # 0x142c5d058
1414305a5: ba 04 00 00 00              	mov	edx, 0x4
1414305aa: eb 5b                       	jmp	0x141430607 <.text+0x142f607>
1414305ac: 48 8d 0d 95 c7 82 01        	lea	rcx, [rip + 0x182c795]  # 0x142c5cd48
1414305b3: ba 08 00 00 00              	mov	edx, 0x8
1414305b8: eb 4d                       	jmp	0x141430607 <.text+0x142f607>
1414305ba: 48 8d 0d 7c ca 82 01        	lea	rcx, [rip + 0x182ca7c]  # 0x142c5d03d
1414305c1: ba 02 00 00 00              	mov	edx, 0x2
1414305c6: eb 3f                       	jmp	0x141430607 <.text+0x142f607>
1414305c8: 48 8d 0d 7e ca 82 01        	lea	rcx, [rip + 0x182ca7e]  # 0x142c5d04d
1414305cf: ba 0b 00 00 00              	mov	edx, 0xb
1414305d4: eb 31                       	jmp	0x141430607 <.text+0x142f607>
1414305d6: 48 8d 0d 50 ca 82 01        	lea	rcx, [rip + 0x182ca50]  # 0x142c5d02d
1414305dd: ba 10 00 00 00              	mov	edx, 0x10
1414305e2: eb 23                       	jmp	0x141430607 <.text+0x142f607>
1414305e4: 48 8d 0d 5b ca 82 01        	lea	rcx, [rip + 0x182ca5b]  # 0x142c5d046
1414305eb: eb 15                       	jmp	0x141430602 <.text+0x142f602>
1414305ed: 48 8d 0d 34 ca 82 01        	lea	rcx, [rip + 0x182ca34]  # 0x142c5d028
1414305f4: ba 05 00 00 00              	mov	edx, 0x5
1414305f9: eb 0c                       	jmp	0x141430607 <.text+0x142f607>
1414305fb: 48 8d 0d 3d ca 82 01        	lea	rcx, [rip + 0x182ca3d]  # 0x142c5d03f
141430602: ba 07 00 00 00              	mov	edx, 0x7
141430607: e8 a4 97 4c 01              	call	0x1428f9db0 <.text+0x28f8db0>
14143060c: e9 77 fc ff ff              	jmp	0x141430288 <.text+0x142f288>
141430611: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
141430620: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141430625: 55                          	push	rbp
141430626: 41 57                       	push	r15
141430628: 41 56                       	push	r14
14143062a: 41 55                       	push	r13
14143062c: 41 54                       	push	r12
14143062e: 56                          	push	rsi
14143062f: 57                          	push	rdi
141430630: 53                          	push	rbx
141430631: 48 83 ec 48                 	sub	rsp, 0x48
141430635: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14143063c: 0f 29 7c 24 20              	movaps	xmmword ptr [rsp + 0x20], xmm7
141430641: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
141430646: ba 28 00 00 00              	mov	edx, 0x28
14143064b: 41 b8 08 00 00 00           	mov	r8d, 0x8
141430651: 48 8b 4d 40                 	mov	rcx, qword ptr [rbp + 0x40]
141430655: e8 c6 35 1e ff              	call	0x140613c20 <.text+0x612c20>
14143065a: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
14143065f: 0f 28 7c 24 20              	movaps	xmm7, xmmword ptr [rsp + 0x20]
141430664: 48 83 c4 48                 	add	rsp, 0x48
141430668: 5b                          	pop	rbx
141430669: 5f                          	pop	rdi
14143066a: 5e                          	pop	rsi
14143066b: 41 5c                       	pop	r12
14143066d: 41 5d                       	pop	r13
14143066f: 41 5e                       	pop	r14
141430671: 41 5f                       	pop	r15
141430673: 5d                          	pop	rbp
141430674: c3                          	ret
141430675: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
141430680: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141430685: 55                          	push	rbp
141430686: 41 57                       	push	r15
141430688: 41 56                       	push	r14
14143068a: 41 55                       	push	r13
14143068c: 41 54                       	push	r12
14143068e: 56                          	push	rsi
14143068f: 57                          	push	rdi
141430690: 53                          	push	rbx
141430691: 48 83 ec 48                 	sub	rsp, 0x48
141430695: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14143069c: 0f 29 7c 24 20              	movaps	xmmword ptr [rsp + 0x20], xmm7
1414306a1: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
1414306a6: ba 28 00 00 00              	mov	edx, 0x28
1414306ab: 41 b8 08 00 00 00           	mov	r8d, 0x8
1414306b1: 48 8b 4d 40                 	mov	rcx, qword ptr [rbp + 0x40]
1414306b5: e8 66 35 1e ff              	call	0x140613c20 <.text+0x612c20>
1414306ba: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
1414306bf: 0f 28 7c 24 20              	movaps	xmm7, xmmword ptr [rsp + 0x20]
1414306c4: 48 83 c4 48                 	add	rsp, 0x48
1414306c8: 5b                          	pop	rbx
1414306c9: 5f                          	pop	rdi
1414306ca: 5e                          	pop	rsi
1414306cb: 41 5c                       	pop	r12
1414306cd: 41 5d                       	pop	r13
1414306cf: 41 5e                       	pop	r14
1414306d1: 41 5f                       	pop	r15
1414306d3: 5d                          	pop	rbp
1414306d4: c3                          	ret
1414306d5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
1414306e0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1414306e5: 55                          	push	rbp
1414306e6: 41 57                       	push	r15
1414306e8: 41 56                       	push	r14
1414306ea: 41 55                       	push	r13
1414306ec: 41 54                       	push	r12
1414306ee: 56                          	push	rsi
1414306ef: 57                          	push	rdi
1414306f0: 53                          	push	rbx
1414306f1: 48 83 ec 48                 	sub	rsp, 0x48
1414306f5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1414306fc: 0f 29 7c 24 20              	movaps	xmmword ptr [rsp + 0x20], xmm7
141430701: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
141430706: 48 8d 4d 50                 	lea	rcx, [rbp + 0x50]
14143070a: 80 bd 80 00 00 00 00        	cmp	byte ptr [rbp + 0x80], 0x0
141430711: 75 05                       	jne	0x141430718 <.text+0x142f718>
141430713: e8 88 81 1b ff              	call	0x1405e88a0 <.text+0x5e78a0>
141430718: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
14143071d: 0f 28 7c 24 20              	movaps	xmm7, xmmword ptr [rsp + 0x20]
141430722: 48 83 c4 48                 	add	rsp, 0x48
141430726: 5b                          	pop	rbx
141430727: 5f                          	pop	rdi
141430728: 5e                          	pop	rsi
141430729: 41 5c                       	pop	r12
14143072b: 41 5d                       	pop	r13
14143072d: 41 5e                       	pop	r14
14143072f: 41 5f                       	pop	r15
141430731: 5d                          	pop	rbp
141430732: c3                          	ret
141430733: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
141430740: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141430745: 55                          	push	rbp
141430746: 41 57                       	push	r15
141430748: 41 56                       	push	r14
14143074a: 41 55                       	push	r13
14143074c: 41 54                       	push	r12
14143074e: 56                          	push	rsi
14143074f: 57                          	push	rdi
141430750: 53                          	push	rbx
141430751: 48 83 ec 48                 	sub	rsp, 0x48
141430755: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14143075c: 0f 29 7c 24 20              	movaps	xmmword ptr [rsp + 0x20], xmm7
141430761: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
141430766: 48 8d 4d 50                 	lea	rcx, [rbp + 0x50]
14143076a: 80 bd 80 00 00 00 00        	cmp	byte ptr [rbp + 0x80], 0x0
141430771: 75 05                       	jne	0x141430778 <.text+0x142f778>
141430773: e8 28 81 1b ff              	call	0x1405e88a0 <.text+0x5e78a0>
141430778: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
14143077d: 0f 28 7c 24 20              	movaps	xmm7, xmmword ptr [rsp + 0x20]
141430782: 48 83 c4 48                 	add	rsp, 0x48
141430786: 5b                          	pop	rbx
141430787: 5f                          	pop	rdi
141430788: 5e                          	pop	rsi
141430789: 41 5c                       	pop	r12
14143078b: 41 5d                       	pop	r13
14143078d: 41 5e                       	pop	r14
14143078f: 41 5f                       	pop	r15
141430791: 5d                          	pop	rbp
141430792: c3                          	ret
