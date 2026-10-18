
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1415f9cc0: 41 57                       	push	r15
1415f9cc2: 41 56                       	push	r14
1415f9cc4: 41 55                       	push	r13
1415f9cc6: 41 54                       	push	r12
1415f9cc8: 56                          	push	rsi
1415f9cc9: 57                          	push	rdi
1415f9cca: 55                          	push	rbp
1415f9ccb: 53                          	push	rbx
1415f9ccc: 48 83 ec 58                 	sub	rsp, 0x58
1415f9cd0: 4d 89 c7                    	mov	r15, r8
1415f9cd3: 48 89 d7                    	mov	rdi, rdx
1415f9cd6: 44 0f b6 41 08              	movzx	r8d, byte ptr [rcx + 0x8]
1415f9cdb: 0f b6 71 09                 	movzx	esi, byte ptr [rcx + 0x9]
1415f9cdf: 45 8b 4f 3c                 	mov	r9d, dword ptr [r15 + 0x3c]
1415f9ce3: 41 8b 47 40                 	mov	eax, dword ptr [r15 + 0x40]
1415f9ce7: 31 d2                       	xor	edx, edx
1415f9ce9: 31 ed                       	xor	ebp, ebp
1415f9ceb: 89 44 24 38                 	mov	dword ptr [rsp + 0x38], eax
1415f9cef: 83 f8 01                    	cmp	eax, 0x1
1415f9cf2: 0f 94 c3                    	sete	bl
1415f9cf5: 41 0f 95 c2                 	setne	r10b
1415f9cf9: 31 c0                       	xor	eax, eax
1415f9cfb: 44 89 4c 24 3c              	mov	dword ptr [rsp + 0x3c], r9d
1415f9d00: 41 83 f9 01                 	cmp	r9d, 0x1
1415f9d04: 41 0f 94 c3                 	sete	r11b
1415f9d08: 41 0f 95 c1                 	setne	r9b
1415f9d0c: 48 89 74 24 50              	mov	qword ptr [rsp + 0x50], rsi
1415f9d11: 41 38 f0                    	cmp	r8b, sil
1415f9d14: 0f 83 47 03 00 00           	jae	0x1415fa061 <.text+0x15f9061>
1415f9d1a: 45 0f b6 e8                 	movzx	r13d, r8b
1415f9d1e: 45 31 c0                    	xor	r8d, r8d
1415f9d21: 4c 8b 21                    	mov	r12, qword ptr [rcx]
1415f9d24: 41 8b 4f 38                 	mov	ecx, dword ptr [r15 + 0x38]
1415f9d28: 89 4c 24 34                 	mov	dword ptr [rsp + 0x34], ecx
1415f9d2c: 88 da                       	mov	dl, bl
1415f9d2e: 44 88 d5                    	mov	bpl, r10b
1415f9d31: 01 d5                       	add	ebp, edx
1415f9d33: 44 88 d8                    	mov	al, r11b
1415f9d36: 45 88 c8                    	mov	r8b, r9b
1415f9d39: 41 01 c0                    	add	r8d, eax
1415f9d3c: 44 89 44 24 44              	mov	dword ptr [rsp + 0x44], r8d
1415f9d41: 31 c9                       	xor	ecx, ecx
1415f9d43: 48 8d 15 2e 45 68 01        	lea	rdx, [rip + 0x168452e]  # 0x142c7e278
1415f9d4a: 89 ee                       	mov	esi, ebp
1415f9d4c: 4c 89 6c 24 48              	mov	qword ptr [rsp + 0x48], r13
1415f9d51: eb 1d                       	jmp	0x1415f9d70 <.text+0x15f8d70>
1415f9d53: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
1415f9d60: 4c 8b 6c 24 48              	mov	r13, qword ptr [rsp + 0x48]
1415f9d65: 48 8d 15 0c 45 68 01        	lea	rdx, [rip + 0x168450c]  # 0x142c7e278
1415f9d6c: 8b 4c 24 40                 	mov	ecx, dword ptr [rsp + 0x40]
1415f9d70: 89 c8                       	mov	eax, ecx
1415f9d72: 0f b6 c9                    	movzx	ecx, cl
1415f9d75: 48 63 0c 8a                 	movsxd	rcx, dword ptr [rdx + 4*rcx]
1415f9d79: 48 01 d1                    	add	rcx, rdx
1415f9d7c: ff e1                       	jmp	rcx
1415f9d7e: b1 01                       	mov	cl, 0x1
1415f9d80: 80 3d 15 b0 03 02 00        	cmp	byte ptr [rip + 0x203b015], 0x0 # 0x143634d9c
1415f9d87: 75 e7                       	jne	0x1415f9d70 <.text+0x15f8d70>
1415f9d89: e9 e2 01 00 00              	jmp	0x1415f9f70 <.text+0x15f8f70>
1415f9d8e: b1 11                       	mov	cl, 0x11
1415f9d90: 80 3d 05 b0 03 02 00        	cmp	byte ptr [rip + 0x203b005], 0x0 # 0x143634d9c
1415f9d97: 75 d7                       	jne	0x1415f9d70 <.text+0x15f8d70>
1415f9d99: e9 d2 01 00 00              	jmp	0x1415f9f70 <.text+0x15f8f70>
1415f9d9e: b1 0f                       	mov	cl, 0xf
1415f9da0: 80 3d f5 af 03 02 00        	cmp	byte ptr [rip + 0x203aff5], 0x0 # 0x143634d9c
1415f9da7: 75 c7                       	jne	0x1415f9d70 <.text+0x15f8d70>
1415f9da9: e9 c2 01 00 00              	jmp	0x1415f9f70 <.text+0x15f8f70>
1415f9dae: b1 0c                       	mov	cl, 0xc
1415f9db0: 80 3d e5 af 03 02 00        	cmp	byte ptr [rip + 0x203afe5], 0x0 # 0x143634d9c
1415f9db7: 75 b7                       	jne	0x1415f9d70 <.text+0x15f8d70>
1415f9db9: e9 b2 01 00 00              	jmp	0x1415f9f70 <.text+0x15f8f70>
1415f9dbe: b1 05                       	mov	cl, 0x5
1415f9dc0: 80 3d d5 af 03 02 00        	cmp	byte ptr [rip + 0x203afd5], 0x0 # 0x143634d9c
1415f9dc7: 75 a7                       	jne	0x1415f9d70 <.text+0x15f8d70>
1415f9dc9: e9 a2 01 00 00              	jmp	0x1415f9f70 <.text+0x15f8f70>
1415f9dce: b1 0d                       	mov	cl, 0xd
1415f9dd0: 80 3d c5 af 03 02 00        	cmp	byte ptr [rip + 0x203afc5], 0x0 # 0x143634d9c
1415f9dd7: 75 97                       	jne	0x1415f9d70 <.text+0x15f8d70>
1415f9dd9: e9 92 01 00 00              	jmp	0x1415f9f70 <.text+0x15f8f70>
1415f9dde: b1 0a                       	mov	cl, 0xa
1415f9de0: 80 3d b5 af 03 02 00        	cmp	byte ptr [rip + 0x203afb5], 0x0 # 0x143634d9c
1415f9de7: 75 87                       	jne	0x1415f9d70 <.text+0x15f8d70>
1415f9de9: e9 82 01 00 00              	jmp	0x1415f9f70 <.text+0x15f8f70>
1415f9dee: b1 1a                       	mov	cl, 0x1a
1415f9df0: 80 3d a5 af 03 02 00        	cmp	byte ptr [rip + 0x203afa5], 0x0 # 0x143634d9c
1415f9df7: 0f 85 73 ff ff ff           	jne	0x1415f9d70 <.text+0x15f8d70>
1415f9dfd: e9 6e 01 00 00              	jmp	0x1415f9f70 <.text+0x15f8f70>
1415f9e02: b1 19                       	mov	cl, 0x19
1415f9e04: 80 3d 91 af 03 02 00        	cmp	byte ptr [rip + 0x203af91], 0x0 # 0x143634d9c
1415f9e0b: 0f 85 5f ff ff ff           	jne	0x1415f9d70 <.text+0x15f8d70>
1415f9e11: e9 5a 01 00 00              	jmp	0x1415f9f70 <.text+0x15f8f70>
1415f9e16: b1 03                       	mov	cl, 0x3
1415f9e18: 80 3d 7d af 03 02 00        	cmp	byte ptr [rip + 0x203af7d], 0x0 # 0x143634d9c
1415f9e1f: 0f 85 4b ff ff ff           	jne	0x1415f9d70 <.text+0x15f8d70>
1415f9e25: e9 46 01 00 00              	jmp	0x1415f9f70 <.text+0x15f8f70>
1415f9e2a: b1 10                       	mov	cl, 0x10
1415f9e2c: 80 3d 69 af 03 02 00        	cmp	byte ptr [rip + 0x203af69], 0x0 # 0x143634d9c
1415f9e33: 0f 85 37 ff ff ff           	jne	0x1415f9d70 <.text+0x15f8d70>
1415f9e39: e9 32 01 00 00              	jmp	0x1415f9f70 <.text+0x15f8f70>
1415f9e3e: b1 04                       	mov	cl, 0x4
1415f9e40: 80 3d 55 af 03 02 00        	cmp	byte ptr [rip + 0x203af55], 0x0 # 0x143634d9c
1415f9e47: 0f 85 23 ff ff ff           	jne	0x1415f9d70 <.text+0x15f8d70>
1415f9e4d: e9 1e 01 00 00              	jmp	0x1415f9f70 <.text+0x15f8f70>
1415f9e52: b1 08                       	mov	cl, 0x8
1415f9e54: 80 3d 41 af 03 02 00        	cmp	byte ptr [rip + 0x203af41], 0x0 # 0x143634d9c
1415f9e5b: 0f 85 0f ff ff ff           	jne	0x1415f9d70 <.text+0x15f8d70>
1415f9e61: e9 0a 01 00 00              	jmp	0x1415f9f70 <.text+0x15f8f70>
1415f9e66: b1 02                       	mov	cl, 0x2
1415f9e68: 80 3d 2d af 03 02 00        	cmp	byte ptr [rip + 0x203af2d], 0x0 # 0x143634d9c
1415f9e6f: 0f 85 fb fe ff ff           	jne	0x1415f9d70 <.text+0x15f8d70>
1415f9e75: e9 f6 00 00 00              	jmp	0x1415f9f70 <.text+0x15f8f70>
1415f9e7a: b1 12                       	mov	cl, 0x12
1415f9e7c: 80 3d 19 af 03 02 00        	cmp	byte ptr [rip + 0x203af19], 0x0 # 0x143634d9c
1415f9e83: 0f 85 e7 fe ff ff           	jne	0x1415f9d70 <.text+0x15f8d70>
1415f9e89: e9 e2 00 00 00              	jmp	0x1415f9f70 <.text+0x15f8f70>
1415f9e8e: b1 15                       	mov	cl, 0x15
1415f9e90: 80 3d 05 af 03 02 00        	cmp	byte ptr [rip + 0x203af05], 0x0 # 0x143634d9c
1415f9e97: 0f 85 d3 fe ff ff           	jne	0x1415f9d70 <.text+0x15f8d70>
1415f9e9d: e9 ce 00 00 00              	jmp	0x1415f9f70 <.text+0x15f8f70>
1415f9ea2: b1 06                       	mov	cl, 0x6
1415f9ea4: 80 3d f1 ae 03 02 00        	cmp	byte ptr [rip + 0x203aef1], 0x0 # 0x143634d9c
1415f9eab: 0f 85 bf fe ff ff           	jne	0x1415f9d70 <.text+0x15f8d70>
1415f9eb1: e9 ba 00 00 00              	jmp	0x1415f9f70 <.text+0x15f8f70>
1415f9eb6: b1 16                       	mov	cl, 0x16
1415f9eb8: 80 3d dd ae 03 02 00        	cmp	byte ptr [rip + 0x203aedd], 0x0 # 0x143634d9c
1415f9ebf: 0f 85 ab fe ff ff           	jne	0x1415f9d70 <.text+0x15f8d70>
1415f9ec5: e9 a6 00 00 00              	jmp	0x1415f9f70 <.text+0x15f8f70>
1415f9eca: b1 0e                       	mov	cl, 0xe
1415f9ecc: 80 3d c9 ae 03 02 00        	cmp	byte ptr [rip + 0x203aec9], 0x0 # 0x143634d9c
1415f9ed3: 0f 85 97 fe ff ff           	jne	0x1415f9d70 <.text+0x15f8d70>
1415f9ed9: e9 92 00 00 00              	jmp	0x1415f9f70 <.text+0x15f8f70>
1415f9ede: b1 07                       	mov	cl, 0x7
1415f9ee0: 80 3d b5 ae 03 02 00        	cmp	byte ptr [rip + 0x203aeb5], 0x0 # 0x143634d9c
1415f9ee7: 0f 85 83 fe ff ff           	jne	0x1415f9d70 <.text+0x15f8d70>
1415f9eed: e9 7e 00 00 00              	jmp	0x1415f9f70 <.text+0x15f8f70>
1415f9ef2: b1 13                       	mov	cl, 0x13
1415f9ef4: 80 3d a1 ae 03 02 00        	cmp	byte ptr [rip + 0x203aea1], 0x0 # 0x143634d9c
1415f9efb: 0f 85 6f fe ff ff           	jne	0x1415f9d70 <.text+0x15f8d70>
1415f9f01: eb 6d                       	jmp	0x1415f9f70 <.text+0x15f8f70>
1415f9f03: b1 0b                       	mov	cl, 0xb
1415f9f05: 80 3d 90 ae 03 02 00        	cmp	byte ptr [rip + 0x203ae90], 0x0 # 0x143634d9c
1415f9f0c: 0f 85 5e fe ff ff           	jne	0x1415f9d70 <.text+0x15f8d70>
1415f9f12: eb 5c                       	jmp	0x1415f9f70 <.text+0x15f8f70>
1415f9f14: b1 09                       	mov	cl, 0x9
1415f9f16: 80 3d 7f ae 03 02 00        	cmp	byte ptr [rip + 0x203ae7f], 0x0 # 0x143634d9c
1415f9f1d: 0f 85 4d fe ff ff           	jne	0x1415f9d70 <.text+0x15f8d70>
1415f9f23: eb 4b                       	jmp	0x1415f9f70 <.text+0x15f8f70>
1415f9f25: b1 17                       	mov	cl, 0x17
1415f9f27: 80 3d 6e ae 03 02 00        	cmp	byte ptr [rip + 0x203ae6e], 0x0 # 0x143634d9c
1415f9f2e: 0f 85 3c fe ff ff           	jne	0x1415f9d70 <.text+0x15f8d70>
1415f9f34: eb 3a                       	jmp	0x1415f9f70 <.text+0x15f8f70>
1415f9f36: b1 14                       	mov	cl, 0x14
1415f9f38: 80 3d 5d ae 03 02 00        	cmp	byte ptr [rip + 0x203ae5d], 0x0 # 0x143634d9c
1415f9f3f: 0f 85 2b fe ff ff           	jne	0x1415f9d70 <.text+0x15f8d70>
1415f9f45: eb 29                       	jmp	0x1415f9f70 <.text+0x15f8f70>
1415f9f47: b1 18                       	mov	cl, 0x18
1415f9f49: 80 3d 4c ae 03 02 00        	cmp	byte ptr [rip + 0x203ae4c], 0x0 # 0x143634d9c
1415f9f50: 0f 85 1a fe ff ff           	jne	0x1415f9d70 <.text+0x15f8d70>
1415f9f56: eb 18                       	jmp	0x1415f9f70 <.text+0x15f8f70>
1415f9f58: b1 1b                       	mov	cl, 0x1b
1415f9f5a: 80 3d 3b ae 03 02 00        	cmp	byte ptr [rip + 0x203ae3b], 0x0 # 0x143634d9c
1415f9f61: 0f 85 09 fe ff ff           	jne	0x1415f9d70 <.text+0x15f8d70>
1415f9f67: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1415f9f70: 3c 01                       	cmp	al, 0x1
1415f9f72: 77 0d                       	ja	0x1415f9f81 <.text+0x15f8f81>
1415f9f74: 80 3d 22 ae 03 02 00        	cmp	byte ptr [rip + 0x203ae22], 0x0 # 0x143634d9d
1415f9f7b: 0f 84 ef fd ff ff           	je	0x1415f9d70 <.text+0x15f8d70>
1415f9f81: 89 4c 24 40                 	mov	dword ptr [rsp + 0x40], ecx
1415f9f85: c0 e0 02                    	shl	al, 0x2
1415f9f88: 83 7c 24 34 00              	cmp	dword ptr [rsp + 0x34], 0x0
1415f9f8d: 0f 84 80 02 00 00           	je	0x1415fa213 <.text+0x15f9213>
1415f9f93: 44 0f b6 f0                 	movzx	r14d, al
1415f9f97: 83 7c 24 3c 00              	cmp	dword ptr [rsp + 0x3c], 0x0
1415f9f9c: 0f 84 b9 02 00 00           	je	0x1415fa25b <.text+0x15f925b>
1415f9fa2: 83 7c 24 38 00              	cmp	dword ptr [rsp + 0x38], 0x0
1415f9fa7: 0f 84 ef 02 00 00           	je	0x1415fa29c <.text+0x15f929c>
1415f9fad: 4d 01 ee                    	add	r14, r13
1415f9fb0: eb 50                       	jmp	0x1415fa002 <.text+0x15f9002>
1415f9fb2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nop	word ptr cs:[rax + rax]
1415f9fc0: b8 40 00 00 00              	mov	eax, 0x40
1415f9fc5: 31 d2                       	xor	edx, edx
1415f9fc7: f7 74 24 34                 	div	dword ptr [rsp + 0x34]
1415f9fcb: 49 ff c5                    	inc	r13
1415f9fce: 83 fa 01                    	cmp	edx, 0x1
1415f9fd1: 83 d8 ff                    	sbb	eax, -0x1
1415f9fd4: f0                          	lock
1415f9fd5: 49 ff 87 a0 0a 00 00        	inc	qword ptr [r15 + 0xaa0]
1415f9fdc: 48 8b 4b 48                 	mov	rcx, qword ptr [rbx + 0x48]
1415f9fe0: 89 c2                       	mov	edx, eax
1415f9fe2: 44 8b 44 24 44              	mov	r8d, dword ptr [rsp + 0x44]
1415f9fe7: 41 89 f1                    	mov	r9d, esi
1415f9fea: 41 ff 97 60 03 00 00        	call	qword ptr [r15 + 0x360]
1415f9ff1: 49 ff c6                    	inc	r14
1415f9ff4: 4c 3b 6c 24 50              	cmp	r13, qword ptr [rsp + 0x50]
1415f9ff9: 49 89 ef                    	mov	r15, rbp
1415f9ffc: 0f 83 5e fd ff ff           	jae	0x1415f9d60 <.text+0x15f8d60>
1415fa002: 49 8b 54 24 28              	mov	rdx, qword ptr [r12 + 0x28]
1415fa007: 4c 39 f2                    	cmp	rdx, r14
1415fa00a: 0f 86 d0 02 00 00           	jbe	0x1415fa2e0 <.text+0x15f92e0>
1415fa010: 49 8b 44 24 20              	mov	rax, qword ptr [r12 + 0x20]
1415fa015: 42 8b 04 b0                 	mov	eax, dword ptr [rax + 4*r14]
1415fa019: 44 89 74 24 2c              	mov	dword ptr [rsp + 0x2c], r14d
1415fa01e: 89 44 24 30                 	mov	dword ptr [rsp + 0x30], eax
1415fa022: 4c 8d 44 24 2c              	lea	r8, [rsp + 0x2c]
1415fa027: 41 b9 08 00 00 00           	mov	r9d, 0x8
1415fa02d: 48 89 f9                    	mov	rcx, rdi
1415fa030: 4c 89 fd                    	mov	rbp, r15
1415fa033: 4c 89 fa                    	mov	rdx, r15
1415fa036: e8 15 da 0f 00              	call	0x1416f7a50 <.text+0x16f6a50>
1415fa03b: 48 8b 1f                    	mov	rbx, qword ptr [rdi]
1415fa03e: 4c 8b 7f 08                 	mov	r15, qword ptr [rdi + 0x8]
1415fa042: 41 80 bf ba 0a 00 00 01     	cmp	byte ptr [r15 + 0xaba], 0x1
1415fa04a: 0f 85 70 ff ff ff           	jne	0x1415f9fc0 <.text+0x15f8fc0>
1415fa050: 48 8b 53 48                 	mov	rdx, qword ptr [rbx + 0x48]
1415fa054: 4c 89 f9                    	mov	rcx, r15
1415fa057: e8 34 40 5f ff              	call	0x140bee090 <.text+0xbed090>
1415fa05c: e9 5f ff ff ff              	jmp	0x1415f9fc0 <.text+0x15f8fc0>
1415fa061: f6 05 34 ad 03 02 01        	test	byte ptr [rip + 0x203ad34], 0x1 # 0x143634d9c
1415fa068: 0f 84 84 00 00 00           	je	0x1415fa0f2 <.text+0x15f90f2>
1415fa06e: 31 c9                       	xor	ecx, ecx
1415fa070: 48 8d 05 91 41 68 01        	lea	rax, [rip + 0x1684191]  # 0x142c7e208
1415fa077: eb 07                       	jmp	0x1415fa080 <.text+0x15f9080>
1415fa079: b1 18                       	mov	cl, 0x18
1415fa07b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
1415fa080: 0f b6 c9                    	movzx	ecx, cl
1415fa083: 48 63 14 88                 	movsxd	rdx, dword ptr [rax + 4*rcx]
1415fa087: 48 01 c2                    	add	rdx, rax
1415fa08a: b1 02                       	mov	cl, 0x2
1415fa08c: ff e2                       	jmp	rdx
1415fa08e: b1 01                       	mov	cl, 0x1
1415fa090: eb ee                       	jmp	0x1415fa080 <.text+0x15f9080>
1415fa092: b1 14                       	mov	cl, 0x14
1415fa094: eb ea                       	jmp	0x1415fa080 <.text+0x15f9080>
1415fa096: b1 0f                       	mov	cl, 0xf
1415fa098: eb e6                       	jmp	0x1415fa080 <.text+0x15f9080>
1415fa09a: b1 1a                       	mov	cl, 0x1a
1415fa09c: eb e2                       	jmp	0x1415fa080 <.text+0x15f9080>
1415fa09e: b1 16                       	mov	cl, 0x16
1415fa0a0: eb de                       	jmp	0x1415fa080 <.text+0x15f9080>
1415fa0a2: b1 09                       	mov	cl, 0x9
1415fa0a4: eb da                       	jmp	0x1415fa080 <.text+0x15f9080>
1415fa0a6: b1 07                       	mov	cl, 0x7
1415fa0a8: eb d6                       	jmp	0x1415fa080 <.text+0x15f9080>
1415fa0aa: b1 03                       	mov	cl, 0x3
1415fa0ac: eb d2                       	jmp	0x1415fa080 <.text+0x15f9080>
1415fa0ae: b1 08                       	mov	cl, 0x8
1415fa0b0: eb ce                       	jmp	0x1415fa080 <.text+0x15f9080>
1415fa0b2: b1 19                       	mov	cl, 0x19
1415fa0b4: eb ca                       	jmp	0x1415fa080 <.text+0x15f9080>
1415fa0b6: b1 0a                       	mov	cl, 0xa
1415fa0b8: eb c6                       	jmp	0x1415fa080 <.text+0x15f9080>
1415fa0ba: b1 13                       	mov	cl, 0x13
1415fa0bc: eb c2                       	jmp	0x1415fa080 <.text+0x15f9080>
1415fa0be: b1 06                       	mov	cl, 0x6
1415fa0c0: eb be                       	jmp	0x1415fa080 <.text+0x15f9080>
1415fa0c2: b1 05                       	mov	cl, 0x5
1415fa0c4: eb ba                       	jmp	0x1415fa080 <.text+0x15f9080>
1415fa0c6: b1 04                       	mov	cl, 0x4
1415fa0c8: eb b6                       	jmp	0x1415fa080 <.text+0x15f9080>
1415fa0ca: b1 1b                       	mov	cl, 0x1b
1415fa0cc: eb b2                       	jmp	0x1415fa080 <.text+0x15f9080>
1415fa0ce: b1 0e                       	mov	cl, 0xe
1415fa0d0: eb ae                       	jmp	0x1415fa080 <.text+0x15f9080>
1415fa0d2: b1 15                       	mov	cl, 0x15
1415fa0d4: eb aa                       	jmp	0x1415fa080 <.text+0x15f9080>
1415fa0d6: b1 0d                       	mov	cl, 0xd
1415fa0d8: eb a6                       	jmp	0x1415fa080 <.text+0x15f9080>
1415fa0da: b1 11                       	mov	cl, 0x11
1415fa0dc: eb a2                       	jmp	0x1415fa080 <.text+0x15f9080>
1415fa0de: b1 10                       	mov	cl, 0x10
1415fa0e0: eb 9e                       	jmp	0x1415fa080 <.text+0x15f9080>
1415fa0e2: b1 12                       	mov	cl, 0x12
1415fa0e4: eb 9a                       	jmp	0x1415fa080 <.text+0x15f9080>
1415fa0e6: b1 0b                       	mov	cl, 0xb
1415fa0e8: eb 96                       	jmp	0x1415fa080 <.text+0x15f9080>
1415fa0ea: b1 17                       	mov	cl, 0x17
1415fa0ec: eb 92                       	jmp	0x1415fa080 <.text+0x15f9080>
1415fa0ee: b1 0c                       	mov	cl, 0xc
1415fa0f0: eb 8e                       	jmp	0x1415fa080 <.text+0x15f9080>
1415fa0f2: f6 05 a4 ac 03 02 01        	test	byte ptr [rip + 0x203aca4], 0x1 # 0x143634d9d
1415fa0f9: 0f 84 83 00 00 00           	je	0x1415fa182 <.text+0x15f9182>
1415fa0ff: 31 c9                       	xor	ecx, ecx
1415fa101: 48 8d 05 90 40 68 01        	lea	rax, [rip + 0x1684090]  # 0x142c7e198
1415fa108: eb 06                       	jmp	0x1415fa110 <.text+0x15f9110>
1415fa10a: b1 18                       	mov	cl, 0x18
1415fa10c: 0f 1f 40 00                 	nop	dword ptr [rax]
1415fa110: 0f b6 c9                    	movzx	ecx, cl
1415fa113: 48 63 14 88                 	movsxd	rdx, dword ptr [rax + 4*rcx]
1415fa117: 48 01 c2                    	add	rdx, rax
1415fa11a: b1 02                       	mov	cl, 0x2
1415fa11c: ff e2                       	jmp	rdx
1415fa11e: b1 01                       	mov	cl, 0x1
1415fa120: eb ee                       	jmp	0x1415fa110 <.text+0x15f9110>
1415fa122: b1 14                       	mov	cl, 0x14
1415fa124: eb ea                       	jmp	0x1415fa110 <.text+0x15f9110>
1415fa126: b1 0f                       	mov	cl, 0xf
1415fa128: eb e6                       	jmp	0x1415fa110 <.text+0x15f9110>
1415fa12a: b1 1a                       	mov	cl, 0x1a
1415fa12c: eb e2                       	jmp	0x1415fa110 <.text+0x15f9110>
1415fa12e: b1 16                       	mov	cl, 0x16
1415fa130: eb de                       	jmp	0x1415fa110 <.text+0x15f9110>
1415fa132: b1 09                       	mov	cl, 0x9
1415fa134: eb da                       	jmp	0x1415fa110 <.text+0x15f9110>
1415fa136: b1 07                       	mov	cl, 0x7
1415fa138: eb d6                       	jmp	0x1415fa110 <.text+0x15f9110>
1415fa13a: b1 03                       	mov	cl, 0x3
1415fa13c: eb d2                       	jmp	0x1415fa110 <.text+0x15f9110>
1415fa13e: b1 08                       	mov	cl, 0x8
1415fa140: eb ce                       	jmp	0x1415fa110 <.text+0x15f9110>
1415fa142: b1 19                       	mov	cl, 0x19
1415fa144: eb ca                       	jmp	0x1415fa110 <.text+0x15f9110>
1415fa146: b1 0a                       	mov	cl, 0xa
1415fa148: eb c6                       	jmp	0x1415fa110 <.text+0x15f9110>
1415fa14a: b1 13                       	mov	cl, 0x13
1415fa14c: eb c2                       	jmp	0x1415fa110 <.text+0x15f9110>
1415fa14e: b1 06                       	mov	cl, 0x6
1415fa150: eb be                       	jmp	0x1415fa110 <.text+0x15f9110>
1415fa152: b1 05                       	mov	cl, 0x5
1415fa154: eb ba                       	jmp	0x1415fa110 <.text+0x15f9110>
1415fa156: b1 04                       	mov	cl, 0x4
1415fa158: eb b6                       	jmp	0x1415fa110 <.text+0x15f9110>
1415fa15a: b1 1b                       	mov	cl, 0x1b
1415fa15c: eb b2                       	jmp	0x1415fa110 <.text+0x15f9110>
1415fa15e: b1 0e                       	mov	cl, 0xe
1415fa160: eb ae                       	jmp	0x1415fa110 <.text+0x15f9110>
1415fa162: b1 15                       	mov	cl, 0x15
1415fa164: eb aa                       	jmp	0x1415fa110 <.text+0x15f9110>
1415fa166: b1 0d                       	mov	cl, 0xd
1415fa168: eb a6                       	jmp	0x1415fa110 <.text+0x15f9110>
1415fa16a: b1 11                       	mov	cl, 0x11
1415fa16c: eb a2                       	jmp	0x1415fa110 <.text+0x15f9110>
1415fa16e: b1 10                       	mov	cl, 0x10
1415fa170: eb 9e                       	jmp	0x1415fa110 <.text+0x15f9110>
1415fa172: b1 12                       	mov	cl, 0x12
1415fa174: eb 9a                       	jmp	0x1415fa110 <.text+0x15f9110>
1415fa176: b1 0b                       	mov	cl, 0xb
1415fa178: eb 96                       	jmp	0x1415fa110 <.text+0x15f9110>
1415fa17a: b1 17                       	mov	cl, 0x17
1415fa17c: eb 92                       	jmp	0x1415fa110 <.text+0x15f9110>
1415fa17e: b1 0c                       	mov	cl, 0xc
1415fa180: eb 8e                       	jmp	0x1415fa110 <.text+0x15f9110>
1415fa182: 31 c9                       	xor	ecx, ecx
1415fa184: 48 8d 05 9d 3f 68 01        	lea	rax, [rip + 0x1683f9d]  # 0x142c7e128
1415fa18b: eb 03                       	jmp	0x1415fa190 <.text+0x15f9190>
1415fa18d: b1 18                       	mov	cl, 0x18
1415fa18f: 90                          	nop
1415fa190: 0f b6 c9                    	movzx	ecx, cl
1415fa193: 48 63 14 88                 	movsxd	rdx, dword ptr [rax + 4*rcx]
1415fa197: 48 01 c2                    	add	rdx, rax
1415fa19a: b1 02                       	mov	cl, 0x2
1415fa19c: ff e2                       	jmp	rdx
1415fa19e: b1 01                       	mov	cl, 0x1
1415fa1a0: eb ee                       	jmp	0x1415fa190 <.text+0x15f9190>
1415fa1a2: b1 14                       	mov	cl, 0x14
1415fa1a4: eb ea                       	jmp	0x1415fa190 <.text+0x15f9190>
1415fa1a6: b1 0f                       	mov	cl, 0xf
1415fa1a8: eb e6                       	jmp	0x1415fa190 <.text+0x15f9190>
1415fa1aa: b1 1a                       	mov	cl, 0x1a
1415fa1ac: eb e2                       	jmp	0x1415fa190 <.text+0x15f9190>
1415fa1ae: b1 16                       	mov	cl, 0x16
1415fa1b0: eb de                       	jmp	0x1415fa190 <.text+0x15f9190>
1415fa1b2: b1 09                       	mov	cl, 0x9
1415fa1b4: eb da                       	jmp	0x1415fa190 <.text+0x15f9190>
1415fa1b6: b1 07                       	mov	cl, 0x7
1415fa1b8: eb d6                       	jmp	0x1415fa190 <.text+0x15f9190>
1415fa1ba: b1 03                       	mov	cl, 0x3
1415fa1bc: eb d2                       	jmp	0x1415fa190 <.text+0x15f9190>
1415fa1be: b1 08                       	mov	cl, 0x8
1415fa1c0: eb ce                       	jmp	0x1415fa190 <.text+0x15f9190>
1415fa1c2: b1 19                       	mov	cl, 0x19
1415fa1c4: eb ca                       	jmp	0x1415fa190 <.text+0x15f9190>
1415fa1c6: b1 0a                       	mov	cl, 0xa
1415fa1c8: eb c6                       	jmp	0x1415fa190 <.text+0x15f9190>
1415fa1ca: b1 13                       	mov	cl, 0x13
1415fa1cc: eb c2                       	jmp	0x1415fa190 <.text+0x15f9190>
1415fa1ce: b1 06                       	mov	cl, 0x6
1415fa1d0: eb be                       	jmp	0x1415fa190 <.text+0x15f9190>
1415fa1d2: b1 05                       	mov	cl, 0x5
1415fa1d4: eb ba                       	jmp	0x1415fa190 <.text+0x15f9190>
1415fa1d6: b1 04                       	mov	cl, 0x4
1415fa1d8: eb b6                       	jmp	0x1415fa190 <.text+0x15f9190>
1415fa1da: b1 1b                       	mov	cl, 0x1b
1415fa1dc: eb b2                       	jmp	0x1415fa190 <.text+0x15f9190>
1415fa1de: b1 0e                       	mov	cl, 0xe
1415fa1e0: eb ae                       	jmp	0x1415fa190 <.text+0x15f9190>
1415fa1e2: b1 15                       	mov	cl, 0x15
1415fa1e4: eb aa                       	jmp	0x1415fa190 <.text+0x15f9190>
1415fa1e6: b1 0d                       	mov	cl, 0xd
1415fa1e8: eb a6                       	jmp	0x1415fa190 <.text+0x15f9190>
1415fa1ea: b1 11                       	mov	cl, 0x11
1415fa1ec: eb a2                       	jmp	0x1415fa190 <.text+0x15f9190>
1415fa1ee: b1 10                       	mov	cl, 0x10
1415fa1f0: eb 9e                       	jmp	0x1415fa190 <.text+0x15f9190>
1415fa1f2: b1 12                       	mov	cl, 0x12
1415fa1f4: eb 9a                       	jmp	0x1415fa190 <.text+0x15f9190>
1415fa1f6: b1 0b                       	mov	cl, 0xb
1415fa1f8: eb 96                       	jmp	0x1415fa190 <.text+0x15f9190>
1415fa1fa: b1 17                       	mov	cl, 0x17
1415fa1fc: eb 92                       	jmp	0x1415fa190 <.text+0x15f9190>
1415fa1fe: b1 0c                       	mov	cl, 0xc
1415fa200: eb 8e                       	jmp	0x1415fa190 <.text+0x15f9190>
1415fa202: 48 83 c4 58                 	add	rsp, 0x58
1415fa206: 5b                          	pop	rbx
1415fa207: 5d                          	pop	rbp
1415fa208: 5f                          	pop	rdi
1415fa209: 5e                          	pop	rsi
1415fa20a: 41 5c                       	pop	r12
1415fa20c: 41 5d                       	pop	r13
1415fa20e: 41 5e                       	pop	r14
1415fa210: 41 5f                       	pop	r15
1415fa212: c3                          	ret
1415fa213: 0f b6 c0                    	movzx	eax, al
1415fa216: 41 01 c5                    	add	r13d, eax
1415fa219: 49 8b 54 24 28              	mov	rdx, qword ptr [r12 + 0x28]
1415fa21e: 4c 39 ea                    	cmp	rdx, r13
1415fa221: 0f 86 b6 00 00 00           	jbe	0x1415fa2dd <.text+0x15f92dd>
1415fa227: 49 8b 44 24 20              	mov	rax, qword ptr [r12 + 0x20]
1415fa22c: 42 8b 04 a8                 	mov	eax, dword ptr [rax + 4*r13]
1415fa230: 44 89 6c 24 2c              	mov	dword ptr [rsp + 0x2c], r13d
1415fa235: 89 44 24 30                 	mov	dword ptr [rsp + 0x30], eax
1415fa239: 4c 8d 44 24 2c              	lea	r8, [rsp + 0x2c]
1415fa23e: 41 b9 08 00 00 00           	mov	r9d, 0x8
1415fa244: 48 89 f9                    	mov	rcx, rdi
1415fa247: 4c 89 fa                    	mov	rdx, r15
1415fa24a: e8 01 d8 0f 00              	call	0x1416f7a50 <.text+0x16f6a50>
1415fa24f: 48 8d 0d 72 bf 4b 01        	lea	rcx, [rip + 0x14bbf72]  # 0x142ab61c8
1415fa256: e8 85 f7 2d 01              	call	0x1428d99e0 <.text+0x28d89e0>
1415fa25b: 45 01 ee                    	add	r14d, r13d
1415fa25e: 49 8b 54 24 28              	mov	rdx, qword ptr [r12 + 0x28]
1415fa263: 4c 39 f2                    	cmp	rdx, r14
1415fa266: 76 78                       	jbe	0x1415fa2e0 <.text+0x15f92e0>
1415fa268: 49 8b 44 24 20              	mov	rax, qword ptr [r12 + 0x20]
1415fa26d: 42 8b 04 b0                 	mov	eax, dword ptr [rax + 4*r14]
1415fa271: 44 89 74 24 2c              	mov	dword ptr [rsp + 0x2c], r14d
1415fa276: 89 44 24 30                 	mov	dword ptr [rsp + 0x30], eax
1415fa27a: 4c 8d 44 24 2c              	lea	r8, [rsp + 0x2c]
1415fa27f: 41 b9 08 00 00 00           	mov	r9d, 0x8
1415fa285: 48 89 f9                    	mov	rcx, rdi
1415fa288: 4c 89 fa                    	mov	rdx, r15
1415fa28b: e8 c0 d7 0f 00              	call	0x1416f7a50 <.text+0x16f6a50>
1415fa290: 48 8d 0d 49 bf 4b 01        	lea	rcx, [rip + 0x14bbf49]  # 0x142ab61e0
1415fa297: e8 44 f7 2d 01              	call	0x1428d99e0 <.text+0x28d89e0>
1415fa29c: 45 01 ee                    	add	r14d, r13d
1415fa29f: 49 8b 54 24 28              	mov	rdx, qword ptr [r12 + 0x28]
1415fa2a4: 4c 39 f2                    	cmp	rdx, r14
1415fa2a7: 76 37                       	jbe	0x1415fa2e0 <.text+0x15f92e0>
1415fa2a9: 49 8b 44 24 20              	mov	rax, qword ptr [r12 + 0x20]
1415fa2ae: 42 8b 04 b0                 	mov	eax, dword ptr [rax + 4*r14]
1415fa2b2: 44 89 74 24 2c              	mov	dword ptr [rsp + 0x2c], r14d
1415fa2b7: 89 44 24 30                 	mov	dword ptr [rsp + 0x30], eax
1415fa2bb: 4c 8d 44 24 2c              	lea	r8, [rsp + 0x2c]
1415fa2c0: 41 b9 08 00 00 00           	mov	r9d, 0x8
1415fa2c6: 48 89 f9                    	mov	rcx, rdi
1415fa2c9: 4c 89 fa                    	mov	rdx, r15
1415fa2cc: e8 7f d7 0f 00              	call	0x1416f7a50 <.text+0x16f6a50>
1415fa2d1: 48 8d 0d 20 bf 4b 01        	lea	rcx, [rip + 0x14bbf20]  # 0x142ab61f8
1415fa2d8: e8 03 f7 2d 01              	call	0x1428d99e0 <.text+0x28d89e0>
1415fa2dd: 4d 89 ee                    	mov	r14, r13
1415fa2e0: 4c 8d 05 31 3a 68 01        	lea	r8, [rip + 0x1683a31]   # 0x142c7dd18
1415fa2e7: 4c 89 f1                    	mov	rcx, r14
1415fa2ea: e8 29 f2 2d 01              	call	0x1428d9518 <.text+0x28d8518>
1415fa2ef: cc                          	int3
