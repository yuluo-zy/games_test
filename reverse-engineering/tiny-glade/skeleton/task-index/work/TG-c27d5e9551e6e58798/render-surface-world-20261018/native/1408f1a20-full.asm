
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408f1a20: 55                          	push	rbp
1408f1a21: 41 57                       	push	r15
1408f1a23: 41 56                       	push	r14
1408f1a25: 41 55                       	push	r13
1408f1a27: 41 54                       	push	r12
1408f1a29: 56                          	push	rsi
1408f1a2a: 57                          	push	rdi
1408f1a2b: 53                          	push	rbx
1408f1a2c: 48 81 ec 78 01 00 00        	sub	rsp, 0x178
1408f1a33: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
1408f1a3b: 0f 29 b5 e0 00 00 00        	movaps	xmmword ptr [rbp + 0xe0], xmm6
1408f1a42: 48 c7 85 d8 00 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0xd8], -0x2
1408f1a4d: 80 3d 3c 33 d4 02 00        	cmp	byte ptr [rip + 0x2d4333c], 0x0 # 0x143634d90
1408f1a54: 0f 85 4e 0a 00 00           	jne	0x1408f24a8 <.text+0x8f14a8>
1408f1a5a: 4d 89 cf                    	mov	r15, r9
1408f1a5d: 4c 89 c7                    	mov	rdi, r8
1408f1a60: 49 89 d6                    	mov	r14, rdx
1408f1a63: 48 8b 01                    	mov	rax, qword ptr [rcx]
1408f1a66: 48 8b 18                    	mov	rbx, qword ptr [rax]
1408f1a69: 48 8d 15 b8 5f 1b 02        	lea	rdx, [rip + 0x21b5fb8]  # 0x142aa7a28
1408f1a70: 48 8d 4d 48                 	lea	rcx, [rbp + 0x48]
1408f1a74: 41 b8 1e 00 00 00           	mov	r8d, 0x1e
1408f1a7a: e8 31 d4 21 00              	call	0x140b0eeb0 <.text+0xb0deb0>
1408f1a7f: 31 c0                       	xor	eax, eax
1408f1a81: 48 3b 45 48                 	cmp	rax, qword ptr [rbp + 0x48]
1408f1a85: 0f 80 41 0b 00 00           	jo	0x1408f25cc <.text+0x8f15cc>
1408f1a8b: 4c 8b ad 70 01 00 00        	mov	r13, qword ptr [rbp + 0x170]
1408f1a92: 48 8b b5 68 01 00 00        	mov	rsi, qword ptr [rbp + 0x168]
1408f1a99: 48 83 c3 10                 	add	rbx, 0x10
1408f1a9d: 48 8b 85 b8 00 00 00        	mov	rax, qword ptr [rbp + 0xb8]
1408f1aa4: 48 89 45 30                 	mov	qword ptr [rbp + 0x30], rax
1408f1aa8: 0f 10 85 a8 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xa8]
1408f1aaf: 0f 29 45 20                 	movaps	xmmword ptr [rbp + 0x20], xmm0
1408f1ab3: 0f 10 85 98 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0x98]
1408f1aba: 0f 29 45 10                 	movaps	xmmword ptr [rbp + 0x10], xmm0
1408f1abe: 0f 10 85 88 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0x88]
1408f1ac5: 0f 29 45 00                 	movaps	xmmword ptr [rbp], xmm0
1408f1ac9: 0f 10 45 48                 	movups	xmm0, xmmword ptr [rbp + 0x48]
1408f1acd: 0f 10 4d 58                 	movups	xmm1, xmmword ptr [rbp + 0x58]
1408f1ad1: 0f 10 55 68                 	movups	xmm2, xmmword ptr [rbp + 0x68]
1408f1ad5: 0f 10 5d 78                 	movups	xmm3, xmmword ptr [rbp + 0x78]
1408f1ad9: 0f 29 5d f0                 	movaps	xmmword ptr [rbp - 0x10], xmm3
1408f1add: 0f 29 55 e0                 	movaps	xmmword ptr [rbp - 0x20], xmm2
1408f1ae1: 0f 29 4d d0                 	movaps	xmmword ptr [rbp - 0x30], xmm1
1408f1ae5: 0f 29 45 c0                 	movaps	xmmword ptr [rbp - 0x40], xmm0
1408f1ae9: 48 8b 3f                    	mov	rdi, qword ptr [rdi]
1408f1aec: 49 8b 06                    	mov	rax, qword ptr [r14]
1408f1aef: 48 89 45 b8                 	mov	qword ptr [rbp - 0x48], rax
1408f1af3: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1408f1af8: 4c 8d 25 91 5f 1b 02        	lea	r12, [rip + 0x21b5f91]  # 0x142aa7a90
1408f1aff: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
1408f1b03: 41 b8 0b 00 00 00           	mov	r8d, 0xb
1408f1b09: 4c 89 e2                    	mov	rdx, r12
1408f1b0c: 49 89 f9                    	mov	r9, rdi
1408f1b0f: e8 8c 91 22 00              	call	0x140b1aca0 <.text+0xb19ca0>
1408f1b14: 49 89 c6                    	mov	r14, rax
1408f1b17: 49 8b 4d 00                 	mov	rcx, qword ptr [r13]
1408f1b1b: 31 c0                       	xor	eax, eax
1408f1b1d: 48 89 4d 40                 	mov	qword ptr [rbp + 0x40], rcx
1408f1b21: 80 79 58 04                 	cmp	byte ptr [rcx + 0x58], 0x4
1408f1b25: 0f 95 c0                    	setne	al
1408f1b28: 48 8d 0d 84 5f 1b 02        	lea	rcx, [rip + 0x21b5f84]  # 0x142aa7ab3
1408f1b2f: 48 8d 15 65 5f 1b 02        	lea	rdx, [rip + 0x21b5f65]  # 0x142aa7a9b
1408f1b36: 48 0f 44 d1                 	cmove	rdx, rcx
1408f1b3a: 4c 8d 0d 88 5f 1b 02        	lea	r9, [rip + 0x21b5f88]   # 0x142aa7ac9
1408f1b41: 4d 0f 45 cc                 	cmovne	r9, r12
1408f1b45: 01 c0                       	add	eax, eax
1408f1b47: 4c 8d 40 16                 	lea	r8, [rax + 0x16]
1408f1b4b: 48 83 c8 09                 	or	rax, 0x9
1408f1b4f: 48 8b 8d 60 01 00 00        	mov	rcx, qword ptr [rbp + 0x160]
1408f1b56: 48 8b 09                    	mov	rcx, qword ptr [rcx]
1408f1b59: 4d 8b 17                    	mov	r10, qword ptr [r15]
1408f1b5c: 4c 89 95 d0 00 00 00        	mov	qword ptr [rbp + 0xd0], r10
1408f1b63: 4c 89 54 24 30              	mov	qword ptr [rsp + 0x30], r10
1408f1b68: 48 89 8d c8 00 00 00        	mov	qword ptr [rbp + 0xc8], rcx
1408f1b6f: 48 89 4c 24 28              	mov	qword ptr [rsp + 0x28], rcx
1408f1b74: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1408f1b79: 48 89 9d c0 00 00 00        	mov	qword ptr [rbp + 0xc0], rbx
1408f1b80: 48 89 d9                    	mov	rcx, rbx
1408f1b83: e8 b8 e2 24 00              	call	0x140b3fe40 <.text+0xb3ee40>
1408f1b88: 49 89 c7                    	mov	r15, rax
1408f1b8b: 48 8d 4d 48                 	lea	rcx, [rbp + 0x48]
1408f1b8f: 48 89 f2                    	mov	rdx, rsi
1408f1b92: e8 09 2b 42 00              	call	0x140d146a0 <.text+0xd136a0>
1408f1b97: 4c 8b 65 50                 	mov	r12, qword ptr [rbp + 0x50]
1408f1b9b: 4d 85 e4                    	test	r12, r12
1408f1b9e: 75 04                       	jne	0x1408f1ba4 <.text+0x8f0ba4>
1408f1ba0: 4c 8b 65 58                 	mov	r12, qword ptr [rbp + 0x58]
1408f1ba4: 48 89 fb                    	mov	rbx, rdi
1408f1ba7: 48 8b 75 68                 	mov	rsi, qword ptr [rbp + 0x68]
1408f1bab: 49 8b 04 24                 	mov	rax, qword ptr [r12]
1408f1baf: 4d 8b 6c 24 10              	mov	r13, qword ptr [r12 + 0x10]
1408f1bb4: 4c 29 e8                    	sub	rax, r13
1408f1bb7: 48 3d 87 00 00 00           	cmp	rax, 0x87
1408f1bbd: 0f 86 00 09 00 00           	jbe	0x1408f24c3 <.text+0x8f14c3>
1408f1bc3: 49 8b 44 24 08              	mov	rax, qword ptr [r12 + 0x8]
1408f1bc8: 48 8d 3d a1 a8 9e 00        	lea	rdi, [rip + 0x9ea8a1]   # 0x1412dc470 <.text+0x12db470>
1408f1bcf: 4a 89 3c 28                 	mov	qword ptr [rax + r13], rdi
1408f1bd3: 0f 28 35 e6 c4 04 02        	movaps	xmm6, xmmword ptr [rip + 0x204c4e6] # 0x14293e0c0
1408f1bda: 42 0f 11 74 28 08           	movups	xmmword ptr [rax + r13 + 0x8], xmm6
1408f1be0: 4a c7 44 28 18 00 00 00 00  	mov	qword ptr [rax + r13 + 0x18], 0x0
1408f1be9: 42 c7 44 28 20 00 00 00 00  	mov	dword ptr [rax + r13 + 0x20], 0x0
1408f1bf2: 48 b9 00 00 80 3f 00 00 80 3f       	movabs	rcx, 0x3f8000003f800000
1408f1bfc: 4a 89 4c 28 24              	mov	qword ptr [rax + r13 + 0x24], rcx
1408f1c01: 42 c7 44 28 2c 00 00 80 3f  	mov	dword ptr [rax + r13 + 0x2c], 0x3f800000
1408f1c0a: 4e 89 74 28 38              	mov	qword ptr [rax + r13 + 0x38], r14
1408f1c0f: 4e 89 7c 28 40              	mov	qword ptr [rax + r13 + 0x40], r15
1408f1c14: 42 c7 44 28 48 0f 00 00 00  	mov	dword ptr [rax + r13 + 0x48], 0xf
1408f1c1d: 42 c6 44 28 58 00           	mov	byte ptr [rax + r13 + 0x58], 0x0
1408f1c23: 4a 89 74 28 68              	mov	qword ptr [rax + r13 + 0x68], rsi
1408f1c28: 48 8d 0d 01 31 c7 00        	lea	rcx, [rip + 0xc73101]   # 0x141564d30 <.text+0x1563d30>
1408f1c2f: 4a 89 4c 28 78              	mov	qword ptr [rax + r13 + 0x78], rcx
1408f1c34: 49 81 c5 88 00 00 00        	add	r13, 0x88
1408f1c3b: 4d 89 6c 24 10              	mov	qword ptr [r12 + 0x10], r13
1408f1c40: 48 8b 85 d0 00 00 00        	mov	rax, qword ptr [rbp + 0xd0]
1408f1c47: 48 89 44 24 30              	mov	qword ptr [rsp + 0x30], rax
1408f1c4c: 48 8b 85 c8 00 00 00        	mov	rax, qword ptr [rbp + 0xc8]
1408f1c53: 48 89 44 24 28              	mov	qword ptr [rsp + 0x28], rax
1408f1c58: 48 c7 44 24 20 1b 00 00 00  	mov	qword ptr [rsp + 0x20], 0x1b
1408f1c61: 48 8d 15 6a 5e 1b 02        	lea	rdx, [rip + 0x21b5e6a]  # 0x142aa7ad2
1408f1c68: 4c 8d 0d 8b 5e 1b 02        	lea	r9, [rip + 0x21b5e8b]   # 0x142aa7afa
1408f1c6f: 41 b8 28 00 00 00           	mov	r8d, 0x28
1408f1c75: 48 8b 8d c0 00 00 00        	mov	rcx, qword ptr [rbp + 0xc0]
1408f1c7c: e8 bf e1 24 00              	call	0x140b3fe40 <.text+0xb3ee40>
1408f1c81: 49 89 c7                    	mov	r15, rax
1408f1c84: 48 8d 4d 48                 	lea	rcx, [rbp + 0x48]
1408f1c88: 48 8b 95 68 01 00 00        	mov	rdx, qword ptr [rbp + 0x168]
1408f1c8f: e8 0c 2a 42 00              	call	0x140d146a0 <.text+0xd136a0>
1408f1c94: 4c 8b 65 50                 	mov	r12, qword ptr [rbp + 0x50]
1408f1c98: 4d 85 e4                    	test	r12, r12
1408f1c9b: 75 04                       	jne	0x1408f1ca1 <.text+0x8f0ca1>
1408f1c9d: 4c 8b 65 58                 	mov	r12, qword ptr [rbp + 0x58]
1408f1ca1: 48 8b 75 68                 	mov	rsi, qword ptr [rbp + 0x68]
1408f1ca5: 49 8b 04 24                 	mov	rax, qword ptr [r12]
1408f1ca9: 4d 8b 6c 24 10              	mov	r13, qword ptr [r12 + 0x10]
1408f1cae: 4c 29 e8                    	sub	rax, r13
1408f1cb1: 48 3d 87 00 00 00           	cmp	rax, 0x87
1408f1cb7: 0f 86 2b 08 00 00           	jbe	0x1408f24e8 <.text+0x8f14e8>
1408f1cbd: 49 8b 44 24 08              	mov	rax, qword ptr [r12 + 0x8]
1408f1cc2: 4a 89 3c 28                 	mov	qword ptr [rax + r13], rdi
1408f1cc6: 42 0f 11 74 28 08           	movups	xmmword ptr [rax + r13 + 0x8], xmm6
1408f1ccc: 4a c7 44 28 18 00 00 00 00  	mov	qword ptr [rax + r13 + 0x18], 0x0
1408f1cd5: 42 c7 44 28 20 00 00 00 00  	mov	dword ptr [rax + r13 + 0x20], 0x0
1408f1cde: 48 b9 00 00 80 3f 00 00 80 3f       	movabs	rcx, 0x3f8000003f800000
1408f1ce8: 4a 89 4c 28 24              	mov	qword ptr [rax + r13 + 0x24], rcx
1408f1ced: 42 c7 44 28 2c 00 00 80 3f  	mov	dword ptr [rax + r13 + 0x2c], 0x3f800000
1408f1cf6: 4e 89 74 28 38              	mov	qword ptr [rax + r13 + 0x38], r14
1408f1cfb: 4e 89 7c 28 40              	mov	qword ptr [rax + r13 + 0x40], r15
1408f1d00: 42 c7 44 28 48 0e 00 00 00  	mov	dword ptr [rax + r13 + 0x48], 0xe
1408f1d09: 42 c6 44 28 58 00           	mov	byte ptr [rax + r13 + 0x58], 0x0
1408f1d0f: 4a 89 74 28 68              	mov	qword ptr [rax + r13 + 0x68], rsi
1408f1d14: 48 8d 0d 15 30 c7 00        	lea	rcx, [rip + 0xc73015]   # 0x141564d30 <.text+0x1563d30>
1408f1d1b: 4a 89 4c 28 78              	mov	qword ptr [rax + r13 + 0x78], rcx
1408f1d20: 49 81 c5 88 00 00 00        	add	r13, 0x88
1408f1d27: 4d 89 6c 24 10              	mov	qword ptr [r12 + 0x10], r13
1408f1d2c: 48 8b 75 40                 	mov	rsi, qword ptr [rbp + 0x40]
1408f1d30: 0f b6 46 58                 	movzx	eax, byte ptr [rsi + 0x58]
1408f1d34: 3c 04                       	cmp	al, 0x4
1408f1d36: 48 89 df                    	mov	rdi, rbx
1408f1d39: 48 8b 5d b8                 	mov	rbx, qword ptr [rbp - 0x48]
1408f1d3d: 0f 85 ed 00 00 00           	jne	0x1408f1e30 <.text+0x8f0e30>
1408f1d43: 48 8b 85 d0 00 00 00        	mov	rax, qword ptr [rbp + 0xd0]
1408f1d4a: 48 89 44 24 30              	mov	qword ptr [rsp + 0x30], rax
1408f1d4f: 48 8b 85 c8 00 00 00        	mov	rax, qword ptr [rbp + 0xc8]
1408f1d56: 48 89 44 24 28              	mov	qword ptr [rsp + 0x28], rax
1408f1d5b: 48 c7 44 24 20 10 00 00 00  	mov	qword ptr [rsp + 0x20], 0x10
1408f1d64: 48 8d 15 aa 5d 1b 02        	lea	rdx, [rip + 0x21b5daa]  # 0x142aa7b15
1408f1d6b: 4c 8d 0d c0 5d 1b 02        	lea	r9, [rip + 0x21b5dc0]   # 0x142aa7b32
1408f1d72: 41 b8 1d 00 00 00           	mov	r8d, 0x1d
1408f1d78: 48 8b 8d c0 00 00 00        	mov	rcx, qword ptr [rbp + 0xc0]
1408f1d7f: e8 bc e0 24 00              	call	0x140b3fe40 <.text+0xb3ee40>
1408f1d84: 49 89 c7                    	mov	r15, rax
1408f1d87: 48 8d 4d 48                 	lea	rcx, [rbp + 0x48]
1408f1d8b: 48 8b 95 68 01 00 00        	mov	rdx, qword ptr [rbp + 0x168]
1408f1d92: e8 09 29 42 00              	call	0x140d146a0 <.text+0xd136a0>
1408f1d97: 4c 8b 65 50                 	mov	r12, qword ptr [rbp + 0x50]
1408f1d9b: 4d 85 e4                    	test	r12, r12
1408f1d9e: 75 04                       	jne	0x1408f1da4 <.text+0x8f0da4>
1408f1da0: 4c 8b 65 58                 	mov	r12, qword ptr [rbp + 0x58]
1408f1da4: 48 8b 75 68                 	mov	rsi, qword ptr [rbp + 0x68]
1408f1da8: 49 8b 04 24                 	mov	rax, qword ptr [r12]
1408f1dac: 4d 8b 6c 24 10              	mov	r13, qword ptr [r12 + 0x10]
1408f1db1: 4c 29 e8                    	sub	rax, r13
1408f1db4: 48 83 f8 77                 	cmp	rax, 0x77
1408f1db8: 0f 86 e9 07 00 00           	jbe	0x1408f25a7 <.text+0x8f15a7>
1408f1dbe: 49 8b 44 24 08              	mov	rax, qword ptr [r12 + 0x8]
1408f1dc3: 48 8d 0d 16 80 9e 00        	lea	rcx, [rip + 0x9e8016]   # 0x1412d9de0 <.text+0x12d8de0>
1408f1dca: 4a 89 0c 28                 	mov	qword ptr [rax + r13], rcx
1408f1dce: 42 0f 11 74 28 08           	movups	xmmword ptr [rax + r13 + 0x8], xmm6
1408f1dd4: 4a c7 44 28 18 00 00 00 00  	mov	qword ptr [rax + r13 + 0x18], 0x0
1408f1ddd: 42 c7 44 28 20 00 00 00 00  	mov	dword ptr [rax + r13 + 0x20], 0x0
1408f1de6: 48 b9 00 00 80 3f 00 00 80 3f       	movabs	rcx, 0x3f8000003f800000
1408f1df0: 4a 89 4c 28 24              	mov	qword ptr [rax + r13 + 0x24], rcx
1408f1df5: 42 c7 44 28 2c 00 00 80 3f  	mov	dword ptr [rax + r13 + 0x2c], 0x3f800000
1408f1dfe: 4e 89 74 28 38              	mov	qword ptr [rax + r13 + 0x38], r14
1408f1e03: 4e 89 7c 28 40              	mov	qword ptr [rax + r13 + 0x40], r15
1408f1e08: 42 c6 44 28 48 00           	mov	byte ptr [rax + r13 + 0x48], 0x0
1408f1e0e: 4a 89 74 28 58              	mov	qword ptr [rax + r13 + 0x58], rsi
1408f1e13: 48 8d 0d 16 2f c7 00        	lea	rcx, [rip + 0xc72f16]   # 0x141564d30 <.text+0x1563d30>
1408f1e1a: 4a 89 4c 28 68              	mov	qword ptr [rax + r13 + 0x68], rcx
1408f1e1f: 49 83 c5 78                 	add	r13, 0x78
1408f1e23: 4d 89 6c 24 10              	mov	qword ptr [r12 + 0x10], r13
1408f1e28: 48 8b 75 40                 	mov	rsi, qword ptr [rbp + 0x40]
1408f1e2c: 0f b6 46 58                 	movzx	eax, byte ptr [rsi + 0x58]
1408f1e30: 04 fd                       	add	al, -0x3
1408f1e32: 31 c9                       	xor	ecx, ecx
1408f1e34: 3c 02                       	cmp	al, 0x2
1408f1e36: 0f 93 c1                    	setae	cl
1408f1e39: 4c 8d 44 49 11              	lea	r8, [rcx + 2*rcx + 0x11]
1408f1e3e: 48 8d 05 11 5d 1b 02        	lea	rax, [rip + 0x21b5d11]  # 0x142aa7b56
1408f1e45: 48 8d 15 f6 5c 1b 02        	lea	rdx, [rip + 0x21b5cf6]  # 0x142aa7b42
1408f1e4c: 48 0f 42 d0                 	cmovb	rdx, rax
1408f1e50: 48 8b 85 d0 00 00 00        	mov	rax, qword ptr [rbp + 0xd0]
1408f1e57: 48 89 44 24 30              	mov	qword ptr [rsp + 0x30], rax
1408f1e5c: 48 8b 85 c8 00 00 00        	mov	rax, qword ptr [rbp + 0xc8]
1408f1e63: 48 89 44 24 28              	mov	qword ptr [rsp + 0x28], rax
1408f1e68: 48 c7 44 24 20 0e 00 00 00  	mov	qword ptr [rsp + 0x20], 0xe
1408f1e71: 4c 8d 0d ef 5c 1b 02        	lea	r9, [rip + 0x21b5cef]   # 0x142aa7b67
1408f1e78: 48 8b 8d c0 00 00 00        	mov	rcx, qword ptr [rbp + 0xc0]
1408f1e7f: e8 5c e7 24 00              	call	0x140b405e0 <.text+0xb3f5e0>
1408f1e84: 49 89 c6                    	mov	r14, rax
1408f1e87: 0f b6 46 58                 	movzx	eax, byte ptr [rsi + 0x58]
1408f1e8b: 04 fd                       	add	al, -0x3
1408f1e8d: 3c 02                       	cmp	al, 0x2
1408f1e8f: b8 20 00 00 00              	mov	eax, 0x20
1408f1e94: 41 b8 1a 00 00 00           	mov	r8d, 0x1a
1408f1e9a: 4c 0f 42 c0                 	cmovb	r8, rax
1408f1e9e: 48 8d 05 ea 5c 1b 02        	lea	rax, [rip + 0x21b5cea]  # 0x142aa7b8f
1408f1ea5: 48 8d 15 c9 5c 1b 02        	lea	rdx, [rip + 0x21b5cc9]  # 0x142aa7b75
1408f1eac: 48 0f 42 d0                 	cmovb	rdx, rax
1408f1eb0: 48 8d 4d 48                 	lea	rcx, [rbp + 0x48]
1408f1eb4: e8 f7 cf 21 00              	call	0x140b0eeb0 <.text+0xb0deb0>
1408f1eb9: 31 c0                       	xor	eax, eax
1408f1ebb: 48 3b 45 48                 	cmp	rax, qword ptr [rbp + 0x48]
1408f1ebf: 0f 80 3c 07 00 00           	jo	0x1408f2601 <.text+0x8f1601>
1408f1ec5: 48 8b 85 b8 00 00 00        	mov	rax, qword ptr [rbp + 0xb8]
1408f1ecc: 48 89 45 30                 	mov	qword ptr [rbp + 0x30], rax
1408f1ed0: 0f 10 85 a8 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xa8]
1408f1ed7: 0f 29 45 20                 	movaps	xmmword ptr [rbp + 0x20], xmm0
1408f1edb: 0f 10 85 98 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0x98]
1408f1ee2: 0f 29 45 10                 	movaps	xmmword ptr [rbp + 0x10], xmm0
1408f1ee6: 0f 10 85 88 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0x88]
1408f1eed: 0f 29 45 00                 	movaps	xmmword ptr [rbp], xmm0
1408f1ef1: 0f 10 45 48                 	movups	xmm0, xmmword ptr [rbp + 0x48]
1408f1ef5: 0f 10 4d 58                 	movups	xmm1, xmmword ptr [rbp + 0x58]
1408f1ef9: 0f 10 55 68                 	movups	xmm2, xmmword ptr [rbp + 0x68]
1408f1efd: 0f 10 5d 78                 	movups	xmm3, xmmword ptr [rbp + 0x78]
1408f1f01: 0f 29 5d f0                 	movaps	xmmword ptr [rbp - 0x10], xmm3
1408f1f05: 0f 29 55 e0                 	movaps	xmmword ptr [rbp - 0x20], xmm2
1408f1f09: 0f 29 4d d0                 	movaps	xmmword ptr [rbp - 0x30], xmm1
1408f1f0d: 0f 29 45 c0                 	movaps	xmmword ptr [rbp - 0x40], xmm0
1408f1f11: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
1408f1f16: 48 8d 15 ab 5c 1b 02        	lea	rdx, [rip + 0x21b5cab]  # 0x142aa7bc8
1408f1f1d: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
1408f1f21: 41 b8 18 00 00 00           	mov	r8d, 0x18
1408f1f27: 49 89 f9                    	mov	r9, rdi
1408f1f2a: e8 71 8d 22 00              	call	0x140b1aca0 <.text+0xb19ca0>
1408f1f2f: 49 89 c7                    	mov	r15, rax
1408f1f32: 48 8d 4d 48                 	lea	rcx, [rbp + 0x48]
1408f1f36: 48 8b 95 68 01 00 00        	mov	rdx, qword ptr [rbp + 0x168]
1408f1f3d: e8 5e 27 42 00              	call	0x140d146a0 <.text+0xd136a0>
1408f1f42: 4c 8b 65 50                 	mov	r12, qword ptr [rbp + 0x50]
1408f1f46: 4d 85 e4                    	test	r12, r12
1408f1f49: 75 04                       	jne	0x1408f1f4f <.text+0x8f0f4f>
1408f1f4b: 4c 8b 65 58                 	mov	r12, qword ptr [rbp + 0x58]
1408f1f4f: 48 8b 75 68                 	mov	rsi, qword ptr [rbp + 0x68]
1408f1f53: 49 8b 04 24                 	mov	rax, qword ptr [r12]
1408f1f57: 4d 8b 6c 24 10              	mov	r13, qword ptr [r12 + 0x10]
1408f1f5c: 4c 29 e8                    	sub	rax, r13
1408f1f5f: 48 83 f8 77                 	cmp	rax, 0x77
1408f1f63: 0f 86 a4 05 00 00           	jbe	0x1408f250d <.text+0x8f150d>
1408f1f69: 49 8b 44 24 08              	mov	rax, qword ptr [r12 + 0x8]
1408f1f6e: 48 8d 0d eb b4 9e 00        	lea	rcx, [rip + 0x9eb4eb]   # 0x1412dd460 <.text+0x12dc460>
1408f1f75: 4a 89 0c 28                 	mov	qword ptr [rax + r13], rcx
1408f1f79: 42 0f 11 74 28 08           	movups	xmmword ptr [rax + r13 + 0x8], xmm6
1408f1f7f: 4a c7 44 28 18 00 00 00 00  	mov	qword ptr [rax + r13 + 0x18], 0x0
1408f1f88: 42 c7 44 28 20 00 00 00 00  	mov	dword ptr [rax + r13 + 0x20], 0x0
1408f1f91: 48 b9 00 00 80 3f 00 00 80 3f       	movabs	rcx, 0x3f8000003f800000
1408f1f9b: 4a 89 4c 28 24              	mov	qword ptr [rax + r13 + 0x24], rcx
1408f1fa0: 42 c7 44 28 2c 00 00 80 3f  	mov	dword ptr [rax + r13 + 0x2c], 0x3f800000
1408f1fa9: 4e 89 7c 28 38              	mov	qword ptr [rax + r13 + 0x38], r15
1408f1fae: 4e 89 74 28 40              	mov	qword ptr [rax + r13 + 0x40], r14
1408f1fb3: 42 c6 44 28 48 00           	mov	byte ptr [rax + r13 + 0x48], 0x0
1408f1fb9: 4a 89 74 28 58              	mov	qword ptr [rax + r13 + 0x58], rsi
1408f1fbe: 48 8d 0d 6b 2d c7 00        	lea	rcx, [rip + 0xc72d6b]   # 0x141564d30 <.text+0x1563d30>
1408f1fc5: 4a 89 4c 28 68              	mov	qword ptr [rax + r13 + 0x68], rcx
1408f1fca: 49 83 c5 78                 	add	r13, 0x78
1408f1fce: 4d 89 6c 24 10              	mov	qword ptr [r12 + 0x10], r13
1408f1fd3: 48 8b 85 d0 00 00 00        	mov	rax, qword ptr [rbp + 0xd0]
1408f1fda: 48 89 44 24 30              	mov	qword ptr [rsp + 0x30], rax
1408f1fdf: 48 8b 85 c8 00 00 00        	mov	rax, qword ptr [rbp + 0xc8]
1408f1fe6: 48 89 44 24 28              	mov	qword ptr [rsp + 0x28], rax
1408f1feb: 48 c7 44 24 20 15 00 00 00  	mov	qword ptr [rsp + 0x20], 0x15
1408f1ff4: 48 8d 15 e5 5b 1b 02        	lea	rdx, [rip + 0x21b5be5]  # 0x142aa7be0
1408f1ffb: 4c 8d 0d f9 5b 1b 02        	lea	r9, [rip + 0x21b5bf9]   # 0x142aa7bfb
1408f2002: 41 b8 1b 00 00 00           	mov	r8d, 0x1b
1408f2008: 48 8b 8d c0 00 00 00        	mov	rcx, qword ptr [rbp + 0xc0]
1408f200f: e8 2c de 24 00              	call	0x140b3fe40 <.text+0xb3ee40>
1408f2014: 49 89 c6                    	mov	r14, rax
1408f2017: 48 8b 45 40                 	mov	rax, qword ptr [rbp + 0x40]
1408f201b: 80 78 58 04                 	cmp	byte ptr [rax + 0x58], 0x4
1408f201f: 75 53                       	jne	0x1408f2074 <.text+0x8f1074>
1408f2021: 48 8d 15 28 5c 1b 02        	lea	rdx, [rip + 0x21b5c28]  # 0x142aa7c50
1408f2028: 48 8d 4d 48                 	lea	rcx, [rbp + 0x48]
1408f202c: 41 b8 25 00 00 00           	mov	r8d, 0x25
1408f2032: e8 79 ce 21 00              	call	0x140b0eeb0 <.text+0xb0deb0>
1408f2037: 31 c0                       	xor	eax, eax
1408f2039: 48 3b 45 48                 	cmp	rax, qword ptr [rbp + 0x48]
1408f203d: 71 57                       	jno	0x1408f2096 <.text+0x8f1096>
1408f203f: 48 8b 45 50                 	mov	rax, qword ptr [rbp + 0x50]
1408f2043: 48 89 45 c0                 	mov	qword ptr [rbp - 0x40], rax
1408f2047: 48 8d 05 2a 5c 1b 02        	lea	rax, [rip + 0x21b5c2a]  # 0x142aa7c78
1408f204e: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1408f2053: 48 8d 0d d6 49 1b 02        	lea	rcx, [rip + 0x21b49d6]  # 0x142aa6a30
1408f205a: 4c 8d 0d af 49 1b 02        	lea	r9, [rip + 0x21b49af]   # 0x142aa6a10
1408f2061: 4c 8d 45 c0                 	lea	r8, [rbp - 0x40]
1408f2065: ba 2b 00 00 00              	mov	edx, 0x2b
1408f206a: e8 f1 76 fe 01              	call	0x1428d9760 <.text+0x28d8760>
1408f206f: e9 56 06 00 00              	jmp	0x1408f26ca <.text+0x8f16ca>
1408f2074: 48 8d 15 95 5b 1b 02        	lea	rdx, [rip + 0x21b5b95]  # 0x142aa7c10
1408f207b: 48 8d 4d 48                 	lea	rcx, [rbp + 0x48]
1408f207f: 41 b8 21 00 00 00           	mov	r8d, 0x21
1408f2085: e8 26 ce 21 00              	call	0x140b0eeb0 <.text+0xb0deb0>
1408f208a: 31 c0                       	xor	eax, eax
1408f208c: 48 3b 45 48                 	cmp	rax, qword ptr [rbp + 0x48]
1408f2090: 0f 80 04 06 00 00           	jo	0x1408f269a <.text+0x8f169a>
1408f2096: 48 8b 85 b8 00 00 00        	mov	rax, qword ptr [rbp + 0xb8]
1408f209d: 48 89 45 30                 	mov	qword ptr [rbp + 0x30], rax
1408f20a1: 0f 10 85 a8 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xa8]
1408f20a8: 0f 29 45 20                 	movaps	xmmword ptr [rbp + 0x20], xmm0
1408f20ac: 0f 10 85 98 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0x98]
1408f20b3: 0f 29 45 10                 	movaps	xmmword ptr [rbp + 0x10], xmm0
1408f20b7: 0f 10 85 88 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0x88]
1408f20be: 0f 29 45 00                 	movaps	xmmword ptr [rbp], xmm0
1408f20c2: 0f 10 45 48                 	movups	xmm0, xmmword ptr [rbp + 0x48]
1408f20c6: 0f 10 4d 58                 	movups	xmm1, xmmword ptr [rbp + 0x58]
1408f20ca: 0f 10 55 68                 	movups	xmm2, xmmword ptr [rbp + 0x68]
1408f20ce: 0f 10 5d 78                 	movups	xmm3, xmmword ptr [rbp + 0x78]
1408f20d2: 0f 29 5d f0                 	movaps	xmmword ptr [rbp - 0x10], xmm3
1408f20d6: 0f 29 55 e0                 	movaps	xmmword ptr [rbp - 0x20], xmm2
1408f20da: 0f 29 4d d0                 	movaps	xmmword ptr [rbp - 0x30], xmm1
1408f20de: 0f 29 45 c0                 	movaps	xmmword ptr [rbp - 0x40], xmm0
1408f20e2: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
1408f20e7: 48 8d 15 a2 5b 1b 02        	lea	rdx, [rip + 0x21b5ba2]  # 0x142aa7c90
1408f20ee: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
1408f20f2: 41 b8 0e 00 00 00           	mov	r8d, 0xe
1408f20f8: 49 89 f9                    	mov	r9, rdi
1408f20fb: e8 a0 8b 22 00              	call	0x140b1aca0 <.text+0xb19ca0>
1408f2100: 49 89 c7                    	mov	r15, rax
1408f2103: 48 8d 4d 48                 	lea	rcx, [rbp + 0x48]
1408f2107: 48 8b 95 68 01 00 00        	mov	rdx, qword ptr [rbp + 0x168]
1408f210e: e8 8d 25 42 00              	call	0x140d146a0 <.text+0xd136a0>
1408f2113: 4c 8b 65 50                 	mov	r12, qword ptr [rbp + 0x50]
1408f2117: 4d 85 e4                    	test	r12, r12
1408f211a: 75 04                       	jne	0x1408f2120 <.text+0x8f1120>
1408f211c: 4c 8b 65 58                 	mov	r12, qword ptr [rbp + 0x58]
1408f2120: 48 8b 75 68                 	mov	rsi, qword ptr [rbp + 0x68]
1408f2124: 49 8b 04 24                 	mov	rax, qword ptr [r12]
1408f2128: 4d 8b 6c 24 10              	mov	r13, qword ptr [r12 + 0x10]
1408f212d: 4c 29 e8                    	sub	rax, r13
1408f2130: 48 83 f8 77                 	cmp	rax, 0x77
1408f2134: 0f 86 f8 03 00 00           	jbe	0x1408f2532 <.text+0x8f1532>
1408f213a: 49 8b 44 24 08              	mov	rax, qword ptr [r12 + 0x8]
1408f213f: 48 8d 0d 1a b3 9e 00        	lea	rcx, [rip + 0x9eb31a]   # 0x1412dd460 <.text+0x12dc460>
1408f2146: 4a 89 0c 28                 	mov	qword ptr [rax + r13], rcx
1408f214a: 42 0f 11 74 28 08           	movups	xmmword ptr [rax + r13 + 0x8], xmm6
1408f2150: 4a c7 44 28 18 00 00 00 00  	mov	qword ptr [rax + r13 + 0x18], 0x0
1408f2159: 42 c7 44 28 20 00 00 00 00  	mov	dword ptr [rax + r13 + 0x20], 0x0
1408f2162: 48 b9 00 00 80 3f 00 00 80 3f       	movabs	rcx, 0x3f8000003f800000
1408f216c: 4a 89 4c 28 24              	mov	qword ptr [rax + r13 + 0x24], rcx
1408f2171: 42 c7 44 28 2c 00 00 80 3f  	mov	dword ptr [rax + r13 + 0x2c], 0x3f800000
1408f217a: 4e 89 7c 28 38              	mov	qword ptr [rax + r13 + 0x38], r15
1408f217f: 4e 89 74 28 40              	mov	qword ptr [rax + r13 + 0x40], r14
1408f2184: 42 c6 44 28 48 00           	mov	byte ptr [rax + r13 + 0x48], 0x0
1408f218a: 4a 89 74 28 58              	mov	qword ptr [rax + r13 + 0x58], rsi
1408f218f: 48 8d 0d 9a 2b c7 00        	lea	rcx, [rip + 0xc72b9a]   # 0x141564d30 <.text+0x1563d30>
1408f2196: 4a 89 4c 28 68              	mov	qword ptr [rax + r13 + 0x68], rcx
1408f219b: 49 83 c5 78                 	add	r13, 0x78
1408f219f: 4d 89 6c 24 10              	mov	qword ptr [r12 + 0x10], r13
1408f21a4: 48 8b 85 d0 00 00 00        	mov	rax, qword ptr [rbp + 0xd0]
1408f21ab: 48 89 44 24 30              	mov	qword ptr [rsp + 0x30], rax
1408f21b0: 48 8b 85 c8 00 00 00        	mov	rax, qword ptr [rbp + 0xc8]
1408f21b7: 48 89 44 24 28              	mov	qword ptr [rsp + 0x28], rax
1408f21bc: 48 c7 44 24 20 0b 00 00 00  	mov	qword ptr [rsp + 0x20], 0xb
1408f21c5: 48 8d 15 8a 59 1b 02        	lea	rdx, [rip + 0x21b598a]  # 0x142aa7b56
1408f21cc: 4c 8d 0d cb 5a 1b 02        	lea	r9, [rip + 0x21b5acb]   # 0x142aa7c9e
1408f21d3: 41 b8 11 00 00 00           	mov	r8d, 0x11
1408f21d9: 48 8b 8d c0 00 00 00        	mov	rcx, qword ptr [rbp + 0xc0]
1408f21e0: e8 5b dc 24 00              	call	0x140b3fe40 <.text+0xb3ee40>
1408f21e5: 49 89 c6                    	mov	r14, rax
1408f21e8: 48 8b 45 40                 	mov	rax, qword ptr [rbp + 0x40]
1408f21ec: 80 78 58 03                 	cmp	byte ptr [rax + 0x58], 0x3
1408f21f0: b8 1e 00 00 00              	mov	eax, 0x1e
1408f21f5: 41 b8 17 00 00 00           	mov	r8d, 0x17
1408f21fb: 4c 0f 44 c0                 	cmove	r8, rax
1408f21ff: 48 8d 05 ba 5a 1b 02        	lea	rax, [rip + 0x21b5aba]  # 0x142aa7cc0
1408f2206: 48 8d 15 9c 5a 1b 02        	lea	rdx, [rip + 0x21b5a9c]  # 0x142aa7ca9
1408f220d: 48 0f 44 d0                 	cmove	rdx, rax
1408f2211: 48 8d 4d 48                 	lea	rcx, [rbp + 0x48]
1408f2215: e8 96 cc 21 00              	call	0x140b0eeb0 <.text+0xb0deb0>
1408f221a: 31 c0                       	xor	eax, eax
1408f221c: 48 3b 45 48                 	cmp	rax, qword ptr [rbp + 0x48]
1408f2220: 0f 80 10 04 00 00           	jo	0x1408f2636 <.text+0x8f1636>
1408f2226: 48 8b 85 b8 00 00 00        	mov	rax, qword ptr [rbp + 0xb8]
1408f222d: 48 89 45 30                 	mov	qword ptr [rbp + 0x30], rax
1408f2231: 0f 10 85 a8 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xa8]
1408f2238: 0f 29 45 20                 	movaps	xmmword ptr [rbp + 0x20], xmm0
1408f223c: 0f 10 85 98 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0x98]
1408f2243: 0f 29 45 10                 	movaps	xmmword ptr [rbp + 0x10], xmm0
1408f2247: 0f 10 85 88 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0x88]
1408f224e: 0f 29 45 00                 	movaps	xmmword ptr [rbp], xmm0
1408f2252: 0f 10 45 48                 	movups	xmm0, xmmword ptr [rbp + 0x48]
1408f2256: 0f 10 4d 58                 	movups	xmm1, xmmword ptr [rbp + 0x58]
1408f225a: 0f 10 55 68                 	movups	xmm2, xmmword ptr [rbp + 0x68]
1408f225e: 0f 10 5d 78                 	movups	xmm3, xmmword ptr [rbp + 0x78]
1408f2262: 0f 29 5d f0                 	movaps	xmmword ptr [rbp - 0x10], xmm3
1408f2266: 0f 29 55 e0                 	movaps	xmmword ptr [rbp - 0x20], xmm2
1408f226a: 0f 29 4d d0                 	movaps	xmmword ptr [rbp - 0x30], xmm1
1408f226e: 0f 29 45 c0                 	movaps	xmmword ptr [rbp - 0x40], xmm0
1408f2272: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
1408f2277: 48 8d 15 7a 5a 1b 02        	lea	rdx, [rip + 0x21b5a7a]  # 0x142aa7cf8
1408f227e: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
1408f2282: 41 b8 04 00 00 00           	mov	r8d, 0x4
1408f2288: 49 89 f9                    	mov	r9, rdi
1408f228b: e8 10 8a 22 00              	call	0x140b1aca0 <.text+0xb19ca0>
1408f2290: 49 89 c7                    	mov	r15, rax
1408f2293: 48 8d 4d 48                 	lea	rcx, [rbp + 0x48]
1408f2297: 48 8b 95 68 01 00 00        	mov	rdx, qword ptr [rbp + 0x168]
1408f229e: e8 fd 23 42 00              	call	0x140d146a0 <.text+0xd136a0>
1408f22a3: 4c 8b 65 50                 	mov	r12, qword ptr [rbp + 0x50]
1408f22a7: 4d 85 e4                    	test	r12, r12
1408f22aa: 75 04                       	jne	0x1408f22b0 <.text+0x8f12b0>
1408f22ac: 4c 8b 65 58                 	mov	r12, qword ptr [rbp + 0x58]
1408f22b0: 48 8b 75 68                 	mov	rsi, qword ptr [rbp + 0x68]
1408f22b4: 49 8b 04 24                 	mov	rax, qword ptr [r12]
1408f22b8: 4d 8b 6c 24 10              	mov	r13, qword ptr [r12 + 0x10]
1408f22bd: 4c 29 e8                    	sub	rax, r13
1408f22c0: 48 83 f8 77                 	cmp	rax, 0x77
1408f22c4: 48 8b 8d c0 00 00 00        	mov	rcx, qword ptr [rbp + 0xc0]
1408f22cb: 0f 86 86 02 00 00           	jbe	0x1408f2557 <.text+0x8f1557>
1408f22d1: 49 8b 44 24 08              	mov	rax, qword ptr [r12 + 0x8]
1408f22d6: 48 8d 15 83 b1 9e 00        	lea	rdx, [rip + 0x9eb183]   # 0x1412dd460 <.text+0x12dc460>
1408f22dd: 4a 89 14 28                 	mov	qword ptr [rax + r13], rdx
1408f22e1: 42 0f 11 74 28 08           	movups	xmmword ptr [rax + r13 + 0x8], xmm6
1408f22e7: 4a c7 44 28 18 00 00 00 00  	mov	qword ptr [rax + r13 + 0x18], 0x0
1408f22f0: 42 c7 44 28 20 00 00 00 00  	mov	dword ptr [rax + r13 + 0x20], 0x0
1408f22f9: 48 ba 00 00 80 3f 00 00 80 3f       	movabs	rdx, 0x3f8000003f800000
1408f2303: 4a 89 54 28 24              	mov	qword ptr [rax + r13 + 0x24], rdx
1408f2308: 42 c7 44 28 2c 00 00 80 3f  	mov	dword ptr [rax + r13 + 0x2c], 0x3f800000
1408f2311: 4e 89 7c 28 38              	mov	qword ptr [rax + r13 + 0x38], r15
1408f2316: 4e 89 74 28 40              	mov	qword ptr [rax + r13 + 0x40], r14
1408f231b: 42 c6 44 28 48 00           	mov	byte ptr [rax + r13 + 0x48], 0x0
1408f2321: 4a 89 74 28 58              	mov	qword ptr [rax + r13 + 0x58], rsi
1408f2326: 48 8d 15 03 2a c7 00        	lea	rdx, [rip + 0xc72a03]   # 0x141564d30 <.text+0x1563d30>
1408f232d: 4a 89 54 28 68              	mov	qword ptr [rax + r13 + 0x68], rdx
1408f2332: 49 83 c5 78                 	add	r13, 0x78
1408f2336: 4d 89 6c 24 10              	mov	qword ptr [r12 + 0x10], r13
1408f233b: 48 8b 85 d0 00 00 00        	mov	rax, qword ptr [rbp + 0xd0]
1408f2342: 48 89 44 24 30              	mov	qword ptr [rsp + 0x30], rax
1408f2347: 48 8b 85 c8 00 00 00        	mov	rax, qword ptr [rbp + 0xc8]
1408f234e: 48 89 44 24 28              	mov	qword ptr [rsp + 0x28], rax
1408f2353: 48 c7 44 24 20 0d 00 00 00  	mov	qword ptr [rsp + 0x20], 0xd
1408f235c: 48 8d 15 99 59 1b 02        	lea	rdx, [rip + 0x21b5999]  # 0x142aa7cfc
1408f2363: 4c 8d 0d ac 59 1b 02        	lea	r9, [rip + 0x21b59ac]   # 0x142aa7d16
1408f236a: 41 b8 1a 00 00 00           	mov	r8d, 0x1a
1408f2370: e8 cb da 24 00              	call	0x140b3fe40 <.text+0xb3ee40>
1408f2375: 48 89 c3                    	mov	rbx, rax
1408f2378: 48 8d 15 a4 59 1b 02        	lea	rdx, [rip + 0x21b59a4]  # 0x142aa7d23
1408f237f: 48 8d 4d 48                 	lea	rcx, [rbp + 0x48]
1408f2383: 41 b8 1f 00 00 00           	mov	r8d, 0x1f
1408f2389: e8 22 cb 21 00              	call	0x140b0eeb0 <.text+0xb0deb0>
1408f238e: 31 c0                       	xor	eax, eax
1408f2390: 48 3b 45 48                 	cmp	rax, qword ptr [rbp + 0x48]
1408f2394: 0f 80 ce 02 00 00           	jo	0x1408f2668 <.text+0x8f1668>
1408f239a: 48 8b 85 b8 00 00 00        	mov	rax, qword ptr [rbp + 0xb8]
1408f23a1: 48 89 45 30                 	mov	qword ptr [rbp + 0x30], rax
1408f23a5: 0f 10 85 a8 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xa8]
1408f23ac: 0f 29 45 20                 	movaps	xmmword ptr [rbp + 0x20], xmm0
1408f23b0: 0f 10 85 98 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0x98]
1408f23b7: 0f 29 45 10                 	movaps	xmmword ptr [rbp + 0x10], xmm0
1408f23bb: 0f 10 85 88 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0x88]
1408f23c2: 0f 29 45 00                 	movaps	xmmword ptr [rbp], xmm0
1408f23c6: 0f 10 45 48                 	movups	xmm0, xmmword ptr [rbp + 0x48]
1408f23ca: 0f 10 4d 58                 	movups	xmm1, xmmword ptr [rbp + 0x58]
1408f23ce: 0f 10 55 68                 	movups	xmm2, xmmword ptr [rbp + 0x68]
1408f23d2: 0f 10 5d 78                 	movups	xmm3, xmmword ptr [rbp + 0x78]
1408f23d6: 0f 29 5d f0                 	movaps	xmmword ptr [rbp - 0x10], xmm3
1408f23da: 0f 29 55 e0                 	movaps	xmmword ptr [rbp - 0x20], xmm2
1408f23de: 0f 29 4d d0                 	movaps	xmmword ptr [rbp - 0x30], xmm1
1408f23e2: 0f 29 45 c0                 	movaps	xmmword ptr [rbp - 0x40], xmm0
1408f23e6: 48 8b 45 b8                 	mov	rax, qword ptr [rbp - 0x48]
1408f23ea: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1408f23ef: 48 8d 15 6a 59 1b 02        	lea	rdx, [rip + 0x21b596a]  # 0x142aa7d60
1408f23f6: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
1408f23fa: 41 b8 0c 00 00 00           	mov	r8d, 0xc
1408f2400: 49 89 f9                    	mov	r9, rdi
1408f2403: e8 98 88 22 00              	call	0x140b1aca0 <.text+0xb19ca0>
1408f2408: 48 89 c7                    	mov	rdi, rax
1408f240b: 48 8d 4d 48                 	lea	rcx, [rbp + 0x48]
1408f240f: 48 8b 95 68 01 00 00        	mov	rdx, qword ptr [rbp + 0x168]
1408f2416: e8 85 22 42 00              	call	0x140d146a0 <.text+0xd136a0>
1408f241b: 48 8b 75 50                 	mov	rsi, qword ptr [rbp + 0x50]
1408f241f: 48 85 f6                    	test	rsi, rsi
1408f2422: 75 04                       	jne	0x1408f2428 <.text+0x8f1428>
1408f2424: 48 8b 75 58                 	mov	rsi, qword ptr [rbp + 0x58]
1408f2428: 4c 8b 7d 68                 	mov	r15, qword ptr [rbp + 0x68]
1408f242c: 48 8b 06                    	mov	rax, qword ptr [rsi]
1408f242f: 4c 8b 76 10                 	mov	r14, qword ptr [rsi + 0x10]
1408f2433: 4c 29 f0                    	sub	rax, r14
1408f2436: 48 83 f8 77                 	cmp	rax, 0x77
1408f243a: 0f 86 42 01 00 00           	jbe	0x1408f2582 <.text+0x8f1582>
1408f2440: 48 8b 46 08                 	mov	rax, qword ptr [rsi + 0x8]
1408f2444: 48 8d 0d 15 b0 9e 00        	lea	rcx, [rip + 0x9eb015]   # 0x1412dd460 <.text+0x12dc460>
1408f244b: 4a 89 0c 30                 	mov	qword ptr [rax + r14], rcx
1408f244f: 42 0f 11 74 30 08           	movups	xmmword ptr [rax + r14 + 0x8], xmm6
1408f2455: 4a c7 44 30 18 00 00 00 00  	mov	qword ptr [rax + r14 + 0x18], 0x0
1408f245e: 42 c7 44 30 20 00 00 00 00  	mov	dword ptr [rax + r14 + 0x20], 0x0
1408f2467: 48 b9 00 00 80 3f 00 00 80 3f       	movabs	rcx, 0x3f8000003f800000
1408f2471: 4a 89 4c 30 24              	mov	qword ptr [rax + r14 + 0x24], rcx
1408f2476: 42 c7 44 30 2c 00 00 80 3f  	mov	dword ptr [rax + r14 + 0x2c], 0x3f800000
1408f247f: 4a 89 7c 30 38              	mov	qword ptr [rax + r14 + 0x38], rdi
1408f2484: 4a 89 5c 30 40              	mov	qword ptr [rax + r14 + 0x40], rbx
1408f2489: 42 c6 44 30 48 00           	mov	byte ptr [rax + r14 + 0x48], 0x0
1408f248f: 4e 89 7c 30 58              	mov	qword ptr [rax + r14 + 0x58], r15
1408f2494: 48 8d 0d 95 28 c7 00        	lea	rcx, [rip + 0xc72895]   # 0x141564d30 <.text+0x1563d30>
1408f249b: 4a 89 4c 30 68              	mov	qword ptr [rax + r14 + 0x68], rcx
1408f24a0: 49 83 c6 78                 	add	r14, 0x78
1408f24a4: 4c 89 76 10                 	mov	qword ptr [rsi + 0x10], r14
1408f24a8: 0f 28 b5 e0 00 00 00        	movaps	xmm6, xmmword ptr [rbp + 0xe0]
1408f24af: 48 81 c4 78 01 00 00        	add	rsp, 0x178
1408f24b6: 5b                          	pop	rbx
1408f24b7: 5f                          	pop	rdi
1408f24b8: 5e                          	pop	rsi
1408f24b9: 41 5c                       	pop	r12
1408f24bb: 41 5d                       	pop	r13
1408f24bd: 41 5e                       	pop	r14
1408f24bf: 41 5f                       	pop	r15
1408f24c1: 5d                          	pop	rbp
1408f24c2: c3                          	ret
1408f24c3: 48 c7 44 24 20 01 00 00 00  	mov	qword ptr [rsp + 0x20], 0x1
1408f24cc: 41 b8 88 00 00 00           	mov	r8d, 0x88
1408f24d2: 41 b9 01 00 00 00           	mov	r9d, 0x1
1408f24d8: 4c 89 e1                    	mov	rcx, r12
1408f24db: 4c 89 ea                    	mov	rdx, r13
1408f24de: e8 7d 75 fb 01              	call	0x1428a9a60 <.text+0x28a8a60>
1408f24e3: e9 db f6 ff ff              	jmp	0x1408f1bc3 <.text+0x8f0bc3>
1408f24e8: 48 c7 44 24 20 01 00 00 00  	mov	qword ptr [rsp + 0x20], 0x1
1408f24f1: 41 b8 88 00 00 00           	mov	r8d, 0x88
1408f24f7: 41 b9 01 00 00 00           	mov	r9d, 0x1
1408f24fd: 4c 89 e1                    	mov	rcx, r12
1408f2500: 4c 89 ea                    	mov	rdx, r13
1408f2503: e8 58 75 fb 01              	call	0x1428a9a60 <.text+0x28a8a60>
1408f2508: e9 b0 f7 ff ff              	jmp	0x1408f1cbd <.text+0x8f0cbd>
1408f250d: 48 c7 44 24 20 01 00 00 00  	mov	qword ptr [rsp + 0x20], 0x1
1408f2516: 41 b8 78 00 00 00           	mov	r8d, 0x78
1408f251c: 41 b9 01 00 00 00           	mov	r9d, 0x1
1408f2522: 4c 89 e1                    	mov	rcx, r12
1408f2525: 4c 89 ea                    	mov	rdx, r13
1408f2528: e8 33 75 fb 01              	call	0x1428a9a60 <.text+0x28a8a60>
1408f252d: e9 37 fa ff ff              	jmp	0x1408f1f69 <.text+0x8f0f69>
1408f2532: 48 c7 44 24 20 01 00 00 00  	mov	qword ptr [rsp + 0x20], 0x1
1408f253b: 41 b8 78 00 00 00           	mov	r8d, 0x78
1408f2541: 41 b9 01 00 00 00           	mov	r9d, 0x1
1408f2547: 4c 89 e1                    	mov	rcx, r12
1408f254a: 4c 89 ea                    	mov	rdx, r13
1408f254d: e8 0e 75 fb 01              	call	0x1428a9a60 <.text+0x28a8a60>
1408f2552: e9 e3 fb ff ff              	jmp	0x1408f213a <.text+0x8f113a>
1408f2557: 48 c7 44 24 20 01 00 00 00  	mov	qword ptr [rsp + 0x20], 0x1
1408f2560: 41 b8 78 00 00 00           	mov	r8d, 0x78
1408f2566: 41 b9 01 00 00 00           	mov	r9d, 0x1
1408f256c: 48 89 cb                    	mov	rbx, rcx
1408f256f: 4c 89 e1                    	mov	rcx, r12
1408f2572: 4c 89 ea                    	mov	rdx, r13
1408f2575: e8 e6 74 fb 01              	call	0x1428a9a60 <.text+0x28a8a60>
1408f257a: 48 89 d9                    	mov	rcx, rbx
1408f257d: e9 4f fd ff ff              	jmp	0x1408f22d1 <.text+0x8f12d1>
1408f2582: 48 c7 44 24 20 01 00 00 00  	mov	qword ptr [rsp + 0x20], 0x1
1408f258b: 41 b8 78 00 00 00           	mov	r8d, 0x78
1408f2591: 41 b9 01 00 00 00           	mov	r9d, 0x1
1408f2597: 48 89 f1                    	mov	rcx, rsi
1408f259a: 4c 89 f2                    	mov	rdx, r14
1408f259d: e8 be 74 fb 01              	call	0x1428a9a60 <.text+0x28a8a60>
1408f25a2: e9 99 fe ff ff              	jmp	0x1408f2440 <.text+0x8f1440>
1408f25a7: 48 c7 44 24 20 01 00 00 00  	mov	qword ptr [rsp + 0x20], 0x1
1408f25b0: 41 b8 78 00 00 00           	mov	r8d, 0x78
1408f25b6: 41 b9 01 00 00 00           	mov	r9d, 0x1
1408f25bc: 4c 89 e1                    	mov	rcx, r12
1408f25bf: 4c 89 ea                    	mov	rdx, r13
1408f25c2: e8 99 74 fb 01              	call	0x1428a9a60 <.text+0x28a8a60>
1408f25c7: e9 f2 f7 ff ff              	jmp	0x1408f1dbe <.text+0x8f0dbe>
1408f25cc: 48 8b 45 50                 	mov	rax, qword ptr [rbp + 0x50]
1408f25d0: 48 89 45 c0                 	mov	qword ptr [rbp - 0x40], rax
1408f25d4: 48 8d 05 9d 54 1b 02        	lea	rax, [rip + 0x21b549d]  # 0x142aa7a78
1408f25db: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1408f25e0: 48 8d 0d 49 44 1b 02        	lea	rcx, [rip + 0x21b4449]  # 0x142aa6a30
1408f25e7: 4c 8d 0d 22 44 1b 02        	lea	r9, [rip + 0x21b4422]   # 0x142aa6a10
1408f25ee: 4c 8d 45 c0                 	lea	r8, [rbp - 0x40]
1408f25f2: ba 2b 00 00 00              	mov	edx, 0x2b
1408f25f7: e8 64 71 fe 01              	call	0x1428d9760 <.text+0x28d8760>
1408f25fc: e9 c9 00 00 00              	jmp	0x1408f26ca <.text+0x8f16ca>
1408f2601: 48 8b 45 50                 	mov	rax, qword ptr [rbp + 0x50]
1408f2605: 48 89 45 c0                 	mov	qword ptr [rbp - 0x40], rax
1408f2609: 48 8d 05 a0 55 1b 02        	lea	rax, [rip + 0x21b55a0]  # 0x142aa7bb0
1408f2610: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1408f2615: 48 8d 0d 14 44 1b 02        	lea	rcx, [rip + 0x21b4414]  # 0x142aa6a30
1408f261c: 4c 8d 0d ed 43 1b 02        	lea	r9, [rip + 0x21b43ed]   # 0x142aa6a10
1408f2623: 4c 8d 45 c0                 	lea	r8, [rbp - 0x40]
1408f2627: ba 2b 00 00 00              	mov	edx, 0x2b
1408f262c: e8 2f 71 fe 01              	call	0x1428d9760 <.text+0x28d8760>
1408f2631: e9 94 00 00 00              	jmp	0x1408f26ca <.text+0x8f16ca>
1408f2636: 48 8b 45 50                 	mov	rax, qword ptr [rbp + 0x50]
1408f263a: 48 89 45 c0                 	mov	qword ptr [rbp - 0x40], rax
1408f263e: 48 8d 05 9b 56 1b 02        	lea	rax, [rip + 0x21b569b]  # 0x142aa7ce0
1408f2645: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1408f264a: 48 8d 0d df 43 1b 02        	lea	rcx, [rip + 0x21b43df]  # 0x142aa6a30
1408f2651: 4c 8d 0d b8 43 1b 02        	lea	r9, [rip + 0x21b43b8]   # 0x142aa6a10
1408f2658: 4c 8d 45 c0                 	lea	r8, [rbp - 0x40]
1408f265c: ba 2b 00 00 00              	mov	edx, 0x2b
1408f2661: e8 fa 70 fe 01              	call	0x1428d9760 <.text+0x28d8760>
1408f2666: eb 62                       	jmp	0x1408f26ca <.text+0x8f16ca>
1408f2668: 48 8b 45 50                 	mov	rax, qword ptr [rbp + 0x50]
1408f266c: 48 89 45 c0                 	mov	qword ptr [rbp - 0x40], rax
1408f2670: 48 8d 05 d1 56 1b 02        	lea	rax, [rip + 0x21b56d1]  # 0x142aa7d48
1408f2677: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1408f267c: 48 8d 0d ad 43 1b 02        	lea	rcx, [rip + 0x21b43ad]  # 0x142aa6a30
1408f2683: 4c 8d 0d 86 43 1b 02        	lea	r9, [rip + 0x21b4386]   # 0x142aa6a10
1408f268a: 4c 8d 45 c0                 	lea	r8, [rbp - 0x40]
1408f268e: ba 2b 00 00 00              	mov	edx, 0x2b
1408f2693: e8 c8 70 fe 01              	call	0x1428d9760 <.text+0x28d8760>
1408f2698: eb 30                       	jmp	0x1408f26ca <.text+0x8f16ca>
1408f269a: 48 8b 45 50                 	mov	rax, qword ptr [rbp + 0x50]
1408f269e: 48 89 45 c0                 	mov	qword ptr [rbp - 0x40], rax
1408f26a2: 48 8d 05 8f 55 1b 02        	lea	rax, [rip + 0x21b558f]  # 0x142aa7c38
1408f26a9: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1408f26ae: 48 8d 0d 7b 43 1b 02        	lea	rcx, [rip + 0x21b437b]  # 0x142aa6a30
1408f26b5: 4c 8d 0d 54 43 1b 02        	lea	r9, [rip + 0x21b4354]   # 0x142aa6a10
1408f26bc: 4c 8d 45 c0                 	lea	r8, [rbp - 0x40]
1408f26c0: ba 2b 00 00 00              	mov	edx, 0x2b
1408f26c5: e8 96 70 fe 01              	call	0x1428d9760 <.text+0x28d8760>
1408f26ca: 0f 0b                       	ud2
1408f26cc: 0f 1f 40 00                 	nop	dword ptr [rax]
1408f26d0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1408f26d5: 55                          	push	rbp
1408f26d6: 41 57                       	push	r15
1408f26d8: 41 56                       	push	r14
1408f26da: 41 55                       	push	r13
1408f26dc: 41 54                       	push	r12
1408f26de: 56                          	push	rsi
1408f26df: 57                          	push	rdi
1408f26e0: 53                          	push	rbx
1408f26e1: 48 83 ec 48                 	sub	rsp, 0x48
1408f26e5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1408f26ec: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
1408f26f1: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
1408f26f5: e8 36 a5 45 00              	call	0x140d4cc30 <.text+0xd4bc30>
1408f26fa: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
1408f26ff: 48 83 c4 48                 	add	rsp, 0x48
1408f2703: 5b                          	pop	rbx
1408f2704: 5f                          	pop	rdi
1408f2705: 5e                          	pop	rsi
1408f2706: 41 5c                       	pop	r12
1408f2708: 41 5d                       	pop	r13
1408f270a: 41 5e                       	pop	r14
1408f270c: 41 5f                       	pop	r15
1408f270e: 5d                          	pop	rbp
1408f270f: c3                          	ret
1408f2710: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1408f2715: 55                          	push	rbp
1408f2716: 41 57                       	push	r15
1408f2718: 41 56                       	push	r14
1408f271a: 41 55                       	push	r13
1408f271c: 41 54                       	push	r12
1408f271e: 56                          	push	rsi
1408f271f: 57                          	push	rdi
1408f2720: 53                          	push	rbx
1408f2721: 48 83 ec 48                 	sub	rsp, 0x48
1408f2725: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1408f272c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
1408f2731: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
1408f2735: e8 f6 a4 45 00              	call	0x140d4cc30 <.text+0xd4bc30>
1408f273a: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
1408f273f: 48 83 c4 48                 	add	rsp, 0x48
1408f2743: 5b                          	pop	rbx
1408f2744: 5f                          	pop	rdi
1408f2745: 5e                          	pop	rsi
1408f2746: 41 5c                       	pop	r12
1408f2748: 41 5d                       	pop	r13
1408f274a: 41 5e                       	pop	r14
1408f274c: 41 5f                       	pop	r15
1408f274e: 5d                          	pop	rbp
1408f274f: c3                          	ret
1408f2750: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1408f2755: 55                          	push	rbp
1408f2756: 41 57                       	push	r15
1408f2758: 41 56                       	push	r14
1408f275a: 41 55                       	push	r13
1408f275c: 41 54                       	push	r12
1408f275e: 56                          	push	rsi
1408f275f: 57                          	push	rdi
1408f2760: 53                          	push	rbx
1408f2761: 48 83 ec 48                 	sub	rsp, 0x48
1408f2765: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1408f276c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
1408f2771: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
1408f2775: e8 b6 a4 45 00              	call	0x140d4cc30 <.text+0xd4bc30>
1408f277a: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
1408f277f: 48 83 c4 48                 	add	rsp, 0x48
1408f2783: 5b                          	pop	rbx
1408f2784: 5f                          	pop	rdi
1408f2785: 5e                          	pop	rsi
1408f2786: 41 5c                       	pop	r12
1408f2788: 41 5d                       	pop	r13
1408f278a: 41 5e                       	pop	r14
1408f278c: 41 5f                       	pop	r15
1408f278e: 5d                          	pop	rbp
1408f278f: c3                          	ret
1408f2790: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1408f2795: 55                          	push	rbp
1408f2796: 41 57                       	push	r15
1408f2798: 41 56                       	push	r14
1408f279a: 41 55                       	push	r13
1408f279c: 41 54                       	push	r12
1408f279e: 56                          	push	rsi
1408f279f: 57                          	push	rdi
1408f27a0: 53                          	push	rbx
1408f27a1: 48 83 ec 48                 	sub	rsp, 0x48
1408f27a5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1408f27ac: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
1408f27b1: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
1408f27b5: e8 76 a4 45 00              	call	0x140d4cc30 <.text+0xd4bc30>
1408f27ba: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
1408f27bf: 48 83 c4 48                 	add	rsp, 0x48
1408f27c3: 5b                          	pop	rbx
1408f27c4: 5f                          	pop	rdi
1408f27c5: 5e                          	pop	rsi
1408f27c6: 41 5c                       	pop	r12
1408f27c8: 41 5d                       	pop	r13
1408f27ca: 41 5e                       	pop	r14
1408f27cc: 41 5f                       	pop	r15
1408f27ce: 5d                          	pop	rbp
1408f27cf: c3                          	ret
1408f27d0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1408f27d5: 55                          	push	rbp
1408f27d6: 41 57                       	push	r15
1408f27d8: 41 56                       	push	r14
1408f27da: 41 55                       	push	r13
1408f27dc: 41 54                       	push	r12
1408f27de: 56                          	push	rsi
1408f27df: 57                          	push	rdi
1408f27e0: 53                          	push	rbx
1408f27e1: 48 83 ec 48                 	sub	rsp, 0x48
1408f27e5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1408f27ec: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
1408f27f1: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
1408f27f5: e8 36 a4 45 00              	call	0x140d4cc30 <.text+0xd4bc30>
1408f27fa: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
1408f27ff: 48 83 c4 48                 	add	rsp, 0x48
1408f2803: 5b                          	pop	rbx
1408f2804: 5f                          	pop	rdi
1408f2805: 5e                          	pop	rsi
1408f2806: 41 5c                       	pop	r12
1408f2808: 41 5d                       	pop	r13
1408f280a: 41 5e                       	pop	r14
1408f280c: 41 5f                       	pop	r15
1408f280e: 5d                          	pop	rbp
1408f280f: c3                          	ret
1408f2810: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1408f2815: 55                          	push	rbp
1408f2816: 41 57                       	push	r15
1408f2818: 41 56                       	push	r14
1408f281a: 41 55                       	push	r13
1408f281c: 41 54                       	push	r12
1408f281e: 56                          	push	rsi
1408f281f: 57                          	push	rdi
1408f2820: 53                          	push	rbx
1408f2821: 48 83 ec 48                 	sub	rsp, 0x48
1408f2825: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1408f282c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
1408f2831: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
1408f2835: e8 f6 a3 45 00              	call	0x140d4cc30 <.text+0xd4bc30>
1408f283a: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
1408f283f: 48 83 c4 48                 	add	rsp, 0x48
1408f2843: 5b                          	pop	rbx
1408f2844: 5f                          	pop	rdi
1408f2845: 5e                          	pop	rsi
1408f2846: 41 5c                       	pop	r12
1408f2848: 41 5d                       	pop	r13
1408f284a: 41 5e                       	pop	r14
1408f284c: 41 5f                       	pop	r15
1408f284e: 5d                          	pop	rbp
1408f284f: c3                          	ret
