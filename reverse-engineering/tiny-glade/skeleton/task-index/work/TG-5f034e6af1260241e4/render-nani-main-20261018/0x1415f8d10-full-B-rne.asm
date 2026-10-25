
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1415f8d10: 41 57                       	push	r15
1415f8d12: 41 56                       	push	r14
1415f8d14: 41 55                       	push	r13
1415f8d16: 41 54                       	push	r12
1415f8d18: 56                          	push	rsi
1415f8d19: 57                          	push	rdi
1415f8d1a: 55                          	push	rbp
1415f8d1b: 53                          	push	rbx
1415f8d1c: 48 83 ec 58                 	sub	rsp, 0x58
1415f8d20: 0f b6 79 08                 	movzx	edi, byte ptr [rcx + 0x8]
1415f8d24: 0f b6 41 09                 	movzx	eax, byte ptr [rcx + 0x9]
1415f8d28: 48 89 44 24 50              	mov	qword ptr [rsp + 0x50], rax
1415f8d2d: 40 38 c7                    	cmp	dil, al
1415f8d30: 0f 83 0e 03 00 00           	jae	0x1415f9044 <.text+0x15f8044>
1415f8d36: 48 8b 32                    	mov	rsi, qword ptr [rdx]
1415f8d39: 48 8b 42 08                 	mov	rax, qword ptr [rdx + 0x8]
1415f8d3d: 48 89 44 24 48              	mov	qword ptr [rsp + 0x48], rax
1415f8d42: 8b 5a 10                    	mov	ebx, dword ptr [rdx + 0x10]
1415f8d45: 48 8b 29                    	mov	rbp, qword ptr [rcx]
1415f8d48: 89 f8                       	mov	eax, edi
1415f8d4a: c1 e0 04                    	shl	eax, 0x4
1415f8d4d: 48 05 c0 06 00 00           	add	rax, 0x6c0
1415f8d53: 48 89 44 24 40              	mov	qword ptr [rsp + 0x40], rax
1415f8d58: 31 c0                       	xor	eax, eax
1415f8d5a: 48 89 7c 24 38              	mov	qword ptr [rsp + 0x38], rdi
1415f8d5f: 90                          	nop
1415f8d60: 44 0f b6 f0                 	movzx	r14d, al
1415f8d64: 48 8d 15 8d 51 68 01        	lea	rdx, [rip + 0x168518d]  # 0x142c7def8
1415f8d6b: 4a 63 0c b2                 	movsxd	rcx, dword ptr [rdx + 4*r14]
1415f8d6f: 48 01 d1                    	add	rcx, rdx
1415f8d72: ff e1                       	jmp	rcx
1415f8d74: b1 01                       	mov	cl, 0x1
1415f8d76: 80 3d 1f c0 03 02 00        	cmp	byte ptr [rip + 0x203c01f], 0x0 # 0x143634d9c
1415f8d7d: 0f 84 dd 01 00 00           	je	0x1415f8f60 <.text+0x15f7f60>
1415f8d83: e9 f1 01 00 00              	jmp	0x1415f8f79 <.text+0x15f7f79>
1415f8d88: b1 11                       	mov	cl, 0x11
1415f8d8a: 80 3d 0b c0 03 02 00        	cmp	byte ptr [rip + 0x203c00b], 0x0 # 0x143634d9c
1415f8d91: 0f 84 c9 01 00 00           	je	0x1415f8f60 <.text+0x15f7f60>
1415f8d97: e9 dd 01 00 00              	jmp	0x1415f8f79 <.text+0x15f7f79>
1415f8d9c: b1 0f                       	mov	cl, 0xf
1415f8d9e: 80 3d f7 bf 03 02 00        	cmp	byte ptr [rip + 0x203bff7], 0x0 # 0x143634d9c
1415f8da5: 0f 84 b5 01 00 00           	je	0x1415f8f60 <.text+0x15f7f60>
1415f8dab: e9 c9 01 00 00              	jmp	0x1415f8f79 <.text+0x15f7f79>
1415f8db0: b1 0c                       	mov	cl, 0xc
1415f8db2: 80 3d e3 bf 03 02 00        	cmp	byte ptr [rip + 0x203bfe3], 0x0 # 0x143634d9c
1415f8db9: 0f 84 a1 01 00 00           	je	0x1415f8f60 <.text+0x15f7f60>
1415f8dbf: e9 b5 01 00 00              	jmp	0x1415f8f79 <.text+0x15f7f79>
1415f8dc4: b1 05                       	mov	cl, 0x5
1415f8dc6: 80 3d cf bf 03 02 00        	cmp	byte ptr [rip + 0x203bfcf], 0x0 # 0x143634d9c
1415f8dcd: 0f 84 8d 01 00 00           	je	0x1415f8f60 <.text+0x15f7f60>
1415f8dd3: e9 a1 01 00 00              	jmp	0x1415f8f79 <.text+0x15f7f79>
1415f8dd8: b1 0d                       	mov	cl, 0xd
1415f8dda: 80 3d bb bf 03 02 00        	cmp	byte ptr [rip + 0x203bfbb], 0x0 # 0x143634d9c
1415f8de1: 0f 84 79 01 00 00           	je	0x1415f8f60 <.text+0x15f7f60>
1415f8de7: e9 8d 01 00 00              	jmp	0x1415f8f79 <.text+0x15f7f79>
1415f8dec: b1 0a                       	mov	cl, 0xa
1415f8dee: 80 3d a7 bf 03 02 00        	cmp	byte ptr [rip + 0x203bfa7], 0x0 # 0x143634d9c
1415f8df5: 0f 84 65 01 00 00           	je	0x1415f8f60 <.text+0x15f7f60>
1415f8dfb: e9 79 01 00 00              	jmp	0x1415f8f79 <.text+0x15f7f79>
1415f8e00: b1 1a                       	mov	cl, 0x1a
1415f8e02: 80 3d 93 bf 03 02 00        	cmp	byte ptr [rip + 0x203bf93], 0x0 # 0x143634d9c
1415f8e09: 0f 84 51 01 00 00           	je	0x1415f8f60 <.text+0x15f7f60>
1415f8e0f: e9 65 01 00 00              	jmp	0x1415f8f79 <.text+0x15f7f79>
1415f8e14: b1 19                       	mov	cl, 0x19
1415f8e16: 80 3d 7f bf 03 02 00        	cmp	byte ptr [rip + 0x203bf7f], 0x0 # 0x143634d9c
1415f8e1d: 0f 84 3d 01 00 00           	je	0x1415f8f60 <.text+0x15f7f60>
1415f8e23: e9 51 01 00 00              	jmp	0x1415f8f79 <.text+0x15f7f79>
1415f8e28: b1 03                       	mov	cl, 0x3
1415f8e2a: 80 3d 6b bf 03 02 00        	cmp	byte ptr [rip + 0x203bf6b], 0x0 # 0x143634d9c
1415f8e31: 0f 84 29 01 00 00           	je	0x1415f8f60 <.text+0x15f7f60>
1415f8e37: e9 3d 01 00 00              	jmp	0x1415f8f79 <.text+0x15f7f79>
1415f8e3c: b1 10                       	mov	cl, 0x10
1415f8e3e: 80 3d 57 bf 03 02 00        	cmp	byte ptr [rip + 0x203bf57], 0x0 # 0x143634d9c
1415f8e45: 0f 84 15 01 00 00           	je	0x1415f8f60 <.text+0x15f7f60>
1415f8e4b: e9 29 01 00 00              	jmp	0x1415f8f79 <.text+0x15f7f79>
1415f8e50: b1 04                       	mov	cl, 0x4
1415f8e52: 80 3d 43 bf 03 02 00        	cmp	byte ptr [rip + 0x203bf43], 0x0 # 0x143634d9c
1415f8e59: 0f 84 01 01 00 00           	je	0x1415f8f60 <.text+0x15f7f60>
1415f8e5f: e9 15 01 00 00              	jmp	0x1415f8f79 <.text+0x15f7f79>
1415f8e64: b1 08                       	mov	cl, 0x8
1415f8e66: 80 3d 2f bf 03 02 00        	cmp	byte ptr [rip + 0x203bf2f], 0x0 # 0x143634d9c
1415f8e6d: 0f 84 ed 00 00 00           	je	0x1415f8f60 <.text+0x15f7f60>
1415f8e73: e9 01 01 00 00              	jmp	0x1415f8f79 <.text+0x15f7f79>
1415f8e78: b1 02                       	mov	cl, 0x2
1415f8e7a: 80 3d 1b bf 03 02 00        	cmp	byte ptr [rip + 0x203bf1b], 0x0 # 0x143634d9c
1415f8e81: 0f 84 d9 00 00 00           	je	0x1415f8f60 <.text+0x15f7f60>
1415f8e87: e9 ed 00 00 00              	jmp	0x1415f8f79 <.text+0x15f7f79>
1415f8e8c: b1 12                       	mov	cl, 0x12
1415f8e8e: 80 3d 07 bf 03 02 00        	cmp	byte ptr [rip + 0x203bf07], 0x0 # 0x143634d9c
1415f8e95: 0f 84 c5 00 00 00           	je	0x1415f8f60 <.text+0x15f7f60>
1415f8e9b: e9 d9 00 00 00              	jmp	0x1415f8f79 <.text+0x15f7f79>
1415f8ea0: b1 15                       	mov	cl, 0x15
1415f8ea2: 80 3d f3 be 03 02 00        	cmp	byte ptr [rip + 0x203bef3], 0x0 # 0x143634d9c
1415f8ea9: 0f 84 b1 00 00 00           	je	0x1415f8f60 <.text+0x15f7f60>
1415f8eaf: e9 c5 00 00 00              	jmp	0x1415f8f79 <.text+0x15f7f79>
1415f8eb4: b1 06                       	mov	cl, 0x6
1415f8eb6: 80 3d df be 03 02 00        	cmp	byte ptr [rip + 0x203bedf], 0x0 # 0x143634d9c
1415f8ebd: 0f 84 9d 00 00 00           	je	0x1415f8f60 <.text+0x15f7f60>
1415f8ec3: e9 b1 00 00 00              	jmp	0x1415f8f79 <.text+0x15f7f79>
1415f8ec8: b1 16                       	mov	cl, 0x16
1415f8eca: 80 3d cb be 03 02 00        	cmp	byte ptr [rip + 0x203becb], 0x0 # 0x143634d9c
1415f8ed1: 0f 84 89 00 00 00           	je	0x1415f8f60 <.text+0x15f7f60>
1415f8ed7: e9 9d 00 00 00              	jmp	0x1415f8f79 <.text+0x15f7f79>
1415f8edc: b1 0e                       	mov	cl, 0xe
1415f8ede: 80 3d b7 be 03 02 00        	cmp	byte ptr [rip + 0x203beb7], 0x0 # 0x143634d9c
1415f8ee5: 74 79                       	je	0x1415f8f60 <.text+0x15f7f60>
1415f8ee7: e9 8d 00 00 00              	jmp	0x1415f8f79 <.text+0x15f7f79>
1415f8eec: b1 07                       	mov	cl, 0x7
1415f8eee: 80 3d a7 be 03 02 00        	cmp	byte ptr [rip + 0x203bea7], 0x0 # 0x143634d9c
1415f8ef5: 74 69                       	je	0x1415f8f60 <.text+0x15f7f60>
1415f8ef7: e9 7d 00 00 00              	jmp	0x1415f8f79 <.text+0x15f7f79>
1415f8efc: b1 13                       	mov	cl, 0x13
1415f8efe: 80 3d 97 be 03 02 00        	cmp	byte ptr [rip + 0x203be97], 0x0 # 0x143634d9c
1415f8f05: 74 59                       	je	0x1415f8f60 <.text+0x15f7f60>
1415f8f07: eb 70                       	jmp	0x1415f8f79 <.text+0x15f7f79>
1415f8f09: b1 0b                       	mov	cl, 0xb
1415f8f0b: 80 3d 8a be 03 02 00        	cmp	byte ptr [rip + 0x203be8a], 0x0 # 0x143634d9c
1415f8f12: 74 4c                       	je	0x1415f8f60 <.text+0x15f7f60>
1415f8f14: eb 63                       	jmp	0x1415f8f79 <.text+0x15f7f79>
1415f8f16: b1 09                       	mov	cl, 0x9
1415f8f18: 80 3d 7d be 03 02 00        	cmp	byte ptr [rip + 0x203be7d], 0x0 # 0x143634d9c
1415f8f1f: 74 3f                       	je	0x1415f8f60 <.text+0x15f7f60>
1415f8f21: eb 56                       	jmp	0x1415f8f79 <.text+0x15f7f79>
1415f8f23: b1 17                       	mov	cl, 0x17
1415f8f25: 80 3d 70 be 03 02 00        	cmp	byte ptr [rip + 0x203be70], 0x0 # 0x143634d9c
1415f8f2c: 74 32                       	je	0x1415f8f60 <.text+0x15f7f60>
1415f8f2e: eb 49                       	jmp	0x1415f8f79 <.text+0x15f7f79>
1415f8f30: b1 14                       	mov	cl, 0x14
1415f8f32: 80 3d 63 be 03 02 00        	cmp	byte ptr [rip + 0x203be63], 0x0 # 0x143634d9c
1415f8f39: 74 25                       	je	0x1415f8f60 <.text+0x15f7f60>
1415f8f3b: eb 3c                       	jmp	0x1415f8f79 <.text+0x15f7f79>
1415f8f3d: b1 18                       	mov	cl, 0x18
1415f8f3f: 80 3d 56 be 03 02 00        	cmp	byte ptr [rip + 0x203be56], 0x0 # 0x143634d9c
1415f8f46: 74 18                       	je	0x1415f8f60 <.text+0x15f7f60>
1415f8f48: eb 2f                       	jmp	0x1415f8f79 <.text+0x15f7f79>
1415f8f4a: b1 1b                       	mov	cl, 0x1b
1415f8f4c: 80 3d 49 be 03 02 00        	cmp	byte ptr [rip + 0x203be49], 0x0 # 0x143634d9c
1415f8f53: 75 24                       	jne	0x1415f8f79 <.text+0x15f7f79>
1415f8f55: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
1415f8f60: 3c 01                       	cmp	al, 0x1
1415f8f62: 89 c8                       	mov	eax, ecx
1415f8f64: 0f 87 f6 fd ff ff           	ja	0x1415f8d60 <.text+0x15f7d60>
1415f8f6a: 80 3d 2c be 03 02 00        	cmp	byte ptr [rip + 0x203be2c], 0x0 # 0x143634d9d
1415f8f71: 89 c8                       	mov	eax, ecx
1415f8f73: 0f 85 e7 fd ff ff           	jne	0x1415f8d60 <.text+0x15f7d60>
1415f8f79: 88 4c 24 2f                 	mov	byte ptr [rsp + 0x2f], cl
1415f8f7d: 4e 8d 3c b7                 	lea	r15, [rdi + 4*r14]
1415f8f81: 41 c1 e6 06                 	shl	r14d, 0x6
1415f8f85: 4c 03 74 24 40              	add	r14, qword ptr [rsp + 0x40]
1415f8f8a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1415f8f90: 48 8b 55 40                 	mov	rdx, qword ptr [rbp + 0x40]
1415f8f94: 4c 39 fa                    	cmp	rdx, r15
1415f8f97: 0f 86 56 02 00 00           	jbe	0x1415f91f3 <.text+0x15f81f3>
1415f8f9d: 48 8b 45 38                 	mov	rax, qword ptr [rbp + 0x38]
1415f8fa1: 42 8b 04 b8                 	mov	eax, dword ptr [rax + 4*r15]
1415f8fa5: 44 89 7c 24 30              	mov	dword ptr [rsp + 0x30], r15d
1415f8faa: 89 44 24 34                 	mov	dword ptr [rsp + 0x34], eax
1415f8fae: 4c 8d 44 24 30              	lea	r8, [rsp + 0x30]
1415f8fb3: 41 b9 08 00 00 00           	mov	r9d, 0x8
1415f8fb9: 48 89 f1                    	mov	rcx, rsi
1415f8fbc: 48 8b 54 24 48              	mov	rdx, qword ptr [rsp + 0x48]
1415f8fc1: e8 8a ea 0f 00              	call	0x1416f7a50 <.text+0x16f6a50>
1415f8fc6: 4c 8b 2e                    	mov	r13, qword ptr [rsi]
1415f8fc9: 4c 8b 66 08                 	mov	r12, qword ptr [rsi + 0x8]
1415f8fcd: 41 80 bc 24 ba 0a 00 00 01  	cmp	byte ptr [r12 + 0xaba], 0x1
1415f8fd6: 75 0c                       	jne	0x1415f8fe4 <.text+0x15f7fe4>
1415f8fd8: 49 8b 55 48                 	mov	rdx, qword ptr [r13 + 0x48]
1415f8fdc: 4c 89 e1                    	mov	rcx, r12
1415f8fdf: e8 ac 50 5f ff              	call	0x140bee090 <.text+0xbed090>
1415f8fe4: f0                          	lock
1415f8fe5: 49 ff 84 24 a0 0a 00 00     	inc	qword ptr [r12 + 0xaa0]
1415f8fed: 4d 8b 6d 48                 	mov	r13, qword ptr [r13 + 0x48]
1415f8ff1: 48 8b 4e 10                 	mov	rcx, qword ptr [rsi + 0x10]
1415f8ff5: 89 da                       	mov	edx, ebx
1415f8ff7: e8 d4 e9 5e ff              	call	0x140be79d0 <.text+0xbe69d0>
1415f8ffc: 4c 89 e1                    	mov	rcx, r12
1415f8fff: 48 89 c2                    	mov	rdx, rax
1415f9002: 45 31 c0                    	xor	r8d, r8d
1415f9005: e8 66 64 5f ff              	call	0x140bef470 <.text+0xbee470>
1415f900a: a8 01                       	test	al, 0x1
1415f900c: 0f 84 f0 01 00 00           	je	0x1415f9202 <.text+0x15f8202>
1415f9012: 48 ff c7                    	inc	rdi
1415f9015: 4c 89 e9                    	mov	rcx, r13
1415f9018: 4d 89 f0                    	mov	r8, r14
1415f901b: 41 ff 94 24 68 03 00 00     	call	qword ptr [r12 + 0x368]
1415f9023: 49 ff c7                    	inc	r15
1415f9026: 49 83 c6 10                 	add	r14, 0x10
1415f902a: 48 3b 7c 24 50              	cmp	rdi, qword ptr [rsp + 0x50]
1415f902f: 0f 82 5b ff ff ff           	jb	0x1415f8f90 <.text+0x15f7f90>
1415f9035: 0f b6 44 24 2f              	movzx	eax, byte ptr [rsp + 0x2f]
1415f903a: 48 8b 7c 24 38              	mov	rdi, qword ptr [rsp + 0x38]
1415f903f: e9 1c fd ff ff              	jmp	0x1415f8d60 <.text+0x15f7d60>
1415f9044: f6 05 51 bd 03 02 01        	test	byte ptr [rip + 0x203bd51], 0x1 # 0x143634d9c
1415f904b: 0f 84 81 00 00 00           	je	0x1415f90d2 <.text+0x15f80d2>
1415f9051: 31 c9                       	xor	ecx, ecx
1415f9053: 48 8d 05 2e 4e 68 01        	lea	rax, [rip + 0x1684e2e]  # 0x142c7de88
1415f905a: eb 04                       	jmp	0x1415f9060 <.text+0x15f8060>
1415f905c: b1 18                       	mov	cl, 0x18
1415f905e: 66 90                       	nop
1415f9060: 0f b6 c9                    	movzx	ecx, cl
1415f9063: 48 63 14 88                 	movsxd	rdx, dword ptr [rax + 4*rcx]
1415f9067: 48 01 c2                    	add	rdx, rax
1415f906a: b1 02                       	mov	cl, 0x2
1415f906c: ff e2                       	jmp	rdx
1415f906e: b1 01                       	mov	cl, 0x1
1415f9070: eb ee                       	jmp	0x1415f9060 <.text+0x15f8060>
1415f9072: b1 14                       	mov	cl, 0x14
1415f9074: eb ea                       	jmp	0x1415f9060 <.text+0x15f8060>
1415f9076: b1 0f                       	mov	cl, 0xf
1415f9078: eb e6                       	jmp	0x1415f9060 <.text+0x15f8060>
1415f907a: b1 1a                       	mov	cl, 0x1a
1415f907c: eb e2                       	jmp	0x1415f9060 <.text+0x15f8060>
1415f907e: b1 16                       	mov	cl, 0x16
1415f9080: eb de                       	jmp	0x1415f9060 <.text+0x15f8060>
1415f9082: b1 09                       	mov	cl, 0x9
1415f9084: eb da                       	jmp	0x1415f9060 <.text+0x15f8060>
1415f9086: b1 07                       	mov	cl, 0x7
1415f9088: eb d6                       	jmp	0x1415f9060 <.text+0x15f8060>
1415f908a: b1 03                       	mov	cl, 0x3
1415f908c: eb d2                       	jmp	0x1415f9060 <.text+0x15f8060>
1415f908e: b1 08                       	mov	cl, 0x8
1415f9090: eb ce                       	jmp	0x1415f9060 <.text+0x15f8060>
1415f9092: b1 19                       	mov	cl, 0x19
1415f9094: eb ca                       	jmp	0x1415f9060 <.text+0x15f8060>
1415f9096: b1 0a                       	mov	cl, 0xa
1415f9098: eb c6                       	jmp	0x1415f9060 <.text+0x15f8060>
1415f909a: b1 13                       	mov	cl, 0x13
1415f909c: eb c2                       	jmp	0x1415f9060 <.text+0x15f8060>
1415f909e: b1 06                       	mov	cl, 0x6
1415f90a0: eb be                       	jmp	0x1415f9060 <.text+0x15f8060>
1415f90a2: b1 05                       	mov	cl, 0x5
1415f90a4: eb ba                       	jmp	0x1415f9060 <.text+0x15f8060>
1415f90a6: b1 04                       	mov	cl, 0x4
1415f90a8: eb b6                       	jmp	0x1415f9060 <.text+0x15f8060>
1415f90aa: b1 1b                       	mov	cl, 0x1b
1415f90ac: eb b2                       	jmp	0x1415f9060 <.text+0x15f8060>
1415f90ae: b1 0e                       	mov	cl, 0xe
1415f90b0: eb ae                       	jmp	0x1415f9060 <.text+0x15f8060>
1415f90b2: b1 15                       	mov	cl, 0x15
1415f90b4: eb aa                       	jmp	0x1415f9060 <.text+0x15f8060>
1415f90b6: b1 0d                       	mov	cl, 0xd
1415f90b8: eb a6                       	jmp	0x1415f9060 <.text+0x15f8060>
1415f90ba: b1 11                       	mov	cl, 0x11
1415f90bc: eb a2                       	jmp	0x1415f9060 <.text+0x15f8060>
1415f90be: b1 10                       	mov	cl, 0x10
1415f90c0: eb 9e                       	jmp	0x1415f9060 <.text+0x15f8060>
1415f90c2: b1 12                       	mov	cl, 0x12
1415f90c4: eb 9a                       	jmp	0x1415f9060 <.text+0x15f8060>
1415f90c6: b1 0b                       	mov	cl, 0xb
1415f90c8: eb 96                       	jmp	0x1415f9060 <.text+0x15f8060>
1415f90ca: b1 17                       	mov	cl, 0x17
1415f90cc: eb 92                       	jmp	0x1415f9060 <.text+0x15f8060>
1415f90ce: b1 0c                       	mov	cl, 0xc
1415f90d0: eb 8e                       	jmp	0x1415f9060 <.text+0x15f8060>
1415f90d2: f6 05 c4 bc 03 02 01        	test	byte ptr [rip + 0x203bcc4], 0x1 # 0x143634d9d
1415f90d9: 0f 84 83 00 00 00           	je	0x1415f9162 <.text+0x15f8162>
1415f90df: 31 c9                       	xor	ecx, ecx
1415f90e1: 48 8d 05 30 4d 68 01        	lea	rax, [rip + 0x1684d30]  # 0x142c7de18
1415f90e8: eb 06                       	jmp	0x1415f90f0 <.text+0x15f80f0>
1415f90ea: b1 18                       	mov	cl, 0x18
1415f90ec: 0f 1f 40 00                 	nop	dword ptr [rax]
1415f90f0: 0f b6 c9                    	movzx	ecx, cl
1415f90f3: 48 63 14 88                 	movsxd	rdx, dword ptr [rax + 4*rcx]
1415f90f7: 48 01 c2                    	add	rdx, rax
1415f90fa: b1 02                       	mov	cl, 0x2
1415f90fc: ff e2                       	jmp	rdx
1415f90fe: b1 01                       	mov	cl, 0x1
1415f9100: eb ee                       	jmp	0x1415f90f0 <.text+0x15f80f0>
1415f9102: b1 14                       	mov	cl, 0x14
1415f9104: eb ea                       	jmp	0x1415f90f0 <.text+0x15f80f0>
1415f9106: b1 0f                       	mov	cl, 0xf
1415f9108: eb e6                       	jmp	0x1415f90f0 <.text+0x15f80f0>
1415f910a: b1 1a                       	mov	cl, 0x1a
1415f910c: eb e2                       	jmp	0x1415f90f0 <.text+0x15f80f0>
1415f910e: b1 16                       	mov	cl, 0x16
1415f9110: eb de                       	jmp	0x1415f90f0 <.text+0x15f80f0>
1415f9112: b1 09                       	mov	cl, 0x9
1415f9114: eb da                       	jmp	0x1415f90f0 <.text+0x15f80f0>
1415f9116: b1 07                       	mov	cl, 0x7
1415f9118: eb d6                       	jmp	0x1415f90f0 <.text+0x15f80f0>
1415f911a: b1 03                       	mov	cl, 0x3
1415f911c: eb d2                       	jmp	0x1415f90f0 <.text+0x15f80f0>
1415f911e: b1 08                       	mov	cl, 0x8
1415f9120: eb ce                       	jmp	0x1415f90f0 <.text+0x15f80f0>
1415f9122: b1 19                       	mov	cl, 0x19
1415f9124: eb ca                       	jmp	0x1415f90f0 <.text+0x15f80f0>
1415f9126: b1 0a                       	mov	cl, 0xa
1415f9128: eb c6                       	jmp	0x1415f90f0 <.text+0x15f80f0>
1415f912a: b1 13                       	mov	cl, 0x13
1415f912c: eb c2                       	jmp	0x1415f90f0 <.text+0x15f80f0>
1415f912e: b1 06                       	mov	cl, 0x6
1415f9130: eb be                       	jmp	0x1415f90f0 <.text+0x15f80f0>
1415f9132: b1 05                       	mov	cl, 0x5
1415f9134: eb ba                       	jmp	0x1415f90f0 <.text+0x15f80f0>
1415f9136: b1 04                       	mov	cl, 0x4
1415f9138: eb b6                       	jmp	0x1415f90f0 <.text+0x15f80f0>
1415f913a: b1 1b                       	mov	cl, 0x1b
1415f913c: eb b2                       	jmp	0x1415f90f0 <.text+0x15f80f0>
1415f913e: b1 0e                       	mov	cl, 0xe
1415f9140: eb ae                       	jmp	0x1415f90f0 <.text+0x15f80f0>
1415f9142: b1 15                       	mov	cl, 0x15
1415f9144: eb aa                       	jmp	0x1415f90f0 <.text+0x15f80f0>
1415f9146: b1 0d                       	mov	cl, 0xd
1415f9148: eb a6                       	jmp	0x1415f90f0 <.text+0x15f80f0>
1415f914a: b1 11                       	mov	cl, 0x11
1415f914c: eb a2                       	jmp	0x1415f90f0 <.text+0x15f80f0>
1415f914e: b1 10                       	mov	cl, 0x10
1415f9150: eb 9e                       	jmp	0x1415f90f0 <.text+0x15f80f0>
1415f9152: b1 12                       	mov	cl, 0x12
1415f9154: eb 9a                       	jmp	0x1415f90f0 <.text+0x15f80f0>
1415f9156: b1 0b                       	mov	cl, 0xb
1415f9158: eb 96                       	jmp	0x1415f90f0 <.text+0x15f80f0>
1415f915a: b1 17                       	mov	cl, 0x17
1415f915c: eb 92                       	jmp	0x1415f90f0 <.text+0x15f80f0>
1415f915e: b1 0c                       	mov	cl, 0xc
1415f9160: eb 8e                       	jmp	0x1415f90f0 <.text+0x15f80f0>
1415f9162: 31 c9                       	xor	ecx, ecx
1415f9164: 48 8d 05 3d 4c 68 01        	lea	rax, [rip + 0x1684c3d]  # 0x142c7dda8
1415f916b: eb 03                       	jmp	0x1415f9170 <.text+0x15f8170>
1415f916d: b1 18                       	mov	cl, 0x18
1415f916f: 90                          	nop
1415f9170: 0f b6 c9                    	movzx	ecx, cl
1415f9173: 48 63 14 88                 	movsxd	rdx, dword ptr [rax + 4*rcx]
1415f9177: 48 01 c2                    	add	rdx, rax
1415f917a: b1 02                       	mov	cl, 0x2
1415f917c: ff e2                       	jmp	rdx
1415f917e: b1 01                       	mov	cl, 0x1
1415f9180: eb ee                       	jmp	0x1415f9170 <.text+0x15f8170>
1415f9182: b1 14                       	mov	cl, 0x14
1415f9184: eb ea                       	jmp	0x1415f9170 <.text+0x15f8170>
1415f9186: b1 0f                       	mov	cl, 0xf
1415f9188: eb e6                       	jmp	0x1415f9170 <.text+0x15f8170>
1415f918a: b1 1a                       	mov	cl, 0x1a
1415f918c: eb e2                       	jmp	0x1415f9170 <.text+0x15f8170>
1415f918e: b1 16                       	mov	cl, 0x16
1415f9190: eb de                       	jmp	0x1415f9170 <.text+0x15f8170>
1415f9192: b1 09                       	mov	cl, 0x9
1415f9194: eb da                       	jmp	0x1415f9170 <.text+0x15f8170>
1415f9196: b1 07                       	mov	cl, 0x7
1415f9198: eb d6                       	jmp	0x1415f9170 <.text+0x15f8170>
1415f919a: b1 03                       	mov	cl, 0x3
1415f919c: eb d2                       	jmp	0x1415f9170 <.text+0x15f8170>
1415f919e: b1 08                       	mov	cl, 0x8
1415f91a0: eb ce                       	jmp	0x1415f9170 <.text+0x15f8170>
1415f91a2: b1 19                       	mov	cl, 0x19
1415f91a4: eb ca                       	jmp	0x1415f9170 <.text+0x15f8170>
1415f91a6: b1 0a                       	mov	cl, 0xa
1415f91a8: eb c6                       	jmp	0x1415f9170 <.text+0x15f8170>
1415f91aa: b1 13                       	mov	cl, 0x13
1415f91ac: eb c2                       	jmp	0x1415f9170 <.text+0x15f8170>
1415f91ae: b1 06                       	mov	cl, 0x6
1415f91b0: eb be                       	jmp	0x1415f9170 <.text+0x15f8170>
1415f91b2: b1 05                       	mov	cl, 0x5
1415f91b4: eb ba                       	jmp	0x1415f9170 <.text+0x15f8170>
1415f91b6: b1 04                       	mov	cl, 0x4
1415f91b8: eb b6                       	jmp	0x1415f9170 <.text+0x15f8170>
1415f91ba: b1 1b                       	mov	cl, 0x1b
1415f91bc: eb b2                       	jmp	0x1415f9170 <.text+0x15f8170>
1415f91be: b1 0e                       	mov	cl, 0xe
1415f91c0: eb ae                       	jmp	0x1415f9170 <.text+0x15f8170>
1415f91c2: b1 15                       	mov	cl, 0x15
1415f91c4: eb aa                       	jmp	0x1415f9170 <.text+0x15f8170>
1415f91c6: b1 0d                       	mov	cl, 0xd
1415f91c8: eb a6                       	jmp	0x1415f9170 <.text+0x15f8170>
1415f91ca: b1 11                       	mov	cl, 0x11
1415f91cc: eb a2                       	jmp	0x1415f9170 <.text+0x15f8170>
1415f91ce: b1 10                       	mov	cl, 0x10
1415f91d0: eb 9e                       	jmp	0x1415f9170 <.text+0x15f8170>
1415f91d2: b1 12                       	mov	cl, 0x12
1415f91d4: eb 9a                       	jmp	0x1415f9170 <.text+0x15f8170>
1415f91d6: b1 0b                       	mov	cl, 0xb
1415f91d8: eb 96                       	jmp	0x1415f9170 <.text+0x15f8170>
1415f91da: b1 17                       	mov	cl, 0x17
1415f91dc: eb 92                       	jmp	0x1415f9170 <.text+0x15f8170>
1415f91de: b1 0c                       	mov	cl, 0xc
1415f91e0: eb 8e                       	jmp	0x1415f9170 <.text+0x15f8170>
1415f91e2: 48 83 c4 58                 	add	rsp, 0x58
1415f91e6: 5b                          	pop	rbx
1415f91e7: 5d                          	pop	rbp
1415f91e8: 5f                          	pop	rdi
1415f91e9: 5e                          	pop	rsi
1415f91ea: 41 5c                       	pop	r12
1415f91ec: 41 5d                       	pop	r13
1415f91ee: 41 5e                       	pop	r14
1415f91f0: 41 5f                       	pop	r15
1415f91f2: c3                          	ret
1415f91f3: 4c 8d 05 36 4b 68 01        	lea	r8, [rip + 0x1684b36]   # 0x142c7dd30
1415f91fa: 4c 89 f9                    	mov	rcx, r15
1415f91fd: e8 16 03 2e 01              	call	0x1428d9518 <.text+0x28d8518>
1415f9202: 48 8d 0d a7 cf 4b 01        	lea	rcx, [rip + 0x14bcfa7]  # 0x142ab61b0
1415f9209: e8 02 01 2e 01              	call	0x1428d9310 <.text+0x28d8310>
1415f920e: cc                          	int3
