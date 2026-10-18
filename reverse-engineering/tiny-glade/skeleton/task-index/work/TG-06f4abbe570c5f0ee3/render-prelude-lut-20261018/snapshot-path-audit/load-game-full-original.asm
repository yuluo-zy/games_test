
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14146d030: 55                          	push	rbp
14146d031: 41 57                       	push	r15
14146d033: 41 56                       	push	r14
14146d035: 41 55                       	push	r13
14146d037: 41 54                       	push	r12
14146d039: 56                          	push	rsi
14146d03a: 57                          	push	rdi
14146d03b: 53                          	push	rbx
14146d03c: 48 81 ec 58 0a 00 00        	sub	rsp, 0xa58
14146d043: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
14146d04b: 44 0f 29 95 c0 09 00 00     	movaps	xmmword ptr [rbp + 0x9c0], xmm10
14146d053: 44 0f 29 8d b0 09 00 00     	movaps	xmmword ptr [rbp + 0x9b0], xmm9
14146d05b: 44 0f 29 85 a0 09 00 00     	movaps	xmmword ptr [rbp + 0x9a0], xmm8
14146d063: 0f 29 bd 90 09 00 00        	movaps	xmmword ptr [rbp + 0x990], xmm7
14146d06a: 0f 29 b5 80 09 00 00        	movaps	xmmword ptr [rbp + 0x980], xmm6
14146d071: 48 c7 85 78 09 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x978], -0x2
14146d07c: 4c 89 8d 60 09 00 00        	mov	qword ptr [rbp + 0x960], r9
14146d083: 4c 89 85 e0 08 00 00        	mov	qword ptr [rbp + 0x8e0], r8
14146d08a: 48 89 95 58 09 00 00        	mov	qword ptr [rbp + 0x958], rdx
14146d091: 48 89 8d 38 09 00 00        	mov	qword ptr [rbp + 0x938], rcx
14146d098: 48 8b bd 80 0a 00 00        	mov	rdi, qword ptr [rbp + 0xa80]
14146d09f: 48 8b 37                    	mov	rsi, qword ptr [rdi]
14146d0a2: 4c 8b 6f 08                 	mov	r13, qword ptr [rdi + 0x8]
14146d0a6: 48 8b 06                    	mov	rax, qword ptr [rsi]
14146d0a9: 31 c9                       	xor	ecx, ecx
14146d0ab: 49 89 c2                    	mov	r10, rax
14146d0ae: 4d 2b 55 18                 	sub	r10, qword ptr [r13 + 0x18]
14146d0b2: 4c 0f 42 d1                 	cmovb	r10, rcx
14146d0b6: 49 2b 45 38                 	sub	rax, qword ptr [r13 + 0x38]
14146d0ba: 48 0f 42 c1                 	cmovb	rax, rcx
14146d0be: 4d 8b 65 10                 	mov	r12, qword ptr [r13 + 0x10]
14146d0c2: 49 8b 55 30                 	mov	rdx, qword ptr [r13 + 0x30]
14146d0c6: 48 89 95 10 09 00 00        	mov	qword ptr [rbp + 0x910], rdx
14146d0cd: 48 29 c2                    	sub	rdx, rax
14146d0d0: 48 0f 42 d1                 	cmovb	rdx, rcx
14146d0d4: 41 0f 96 c0                 	setbe	r8b
14146d0d8: 4c 89 a5 50 09 00 00        	mov	qword ptr [rbp + 0x950], r12
14146d0df: 4d 29 d4                    	sub	r12, r10
14146d0e2: 4c 0f 42 e1                 	cmovb	r12, rcx
14146d0e6: 41 0f 96 c1                 	setbe	r9b
14146d0ea: 45 84 c1                    	test	r9b, r8b
14146d0ed: 4e 8d 1c 22                 	lea	r11, [rdx + r12]
14146d0f1: 4c 0f 44 d9                 	cmove	r11, rcx
14146d0f5: 49 8b 4d 40                 	mov	rcx, qword ptr [r13 + 0x40]
14146d0f9: 4c 29 d9                    	sub	rcx, r11
14146d0fc: 48 8b 9d 78 0a 00 00        	mov	rbx, qword ptr [rbp + 0xa78]
14146d103: f2 0f 10 b3 90 00 00 00     	movsd	xmm6, qword ptr [rbx + 0x90]
14146d10b: 4c 8b 1b                    	mov	r11, qword ptr [rbx]
14146d10e: 4c 89 9d a8 08 00 00        	mov	qword ptr [rbp + 0x8a8], r11
14146d115: 4c 8b 5b 10                 	mov	r11, qword ptr [rbx + 0x10]
14146d119: 4c 89 9d 60 05 00 00        	mov	qword ptr [rbp + 0x560], r11
14146d120: 44 8b 5b 1c                 	mov	r11d, dword ptr [rbx + 0x1c]
14146d124: 44 89 9d c0 08 00 00        	mov	dword ptr [rbp + 0x8c0], r11d
14146d12b: 4c 8b 5b 20                 	mov	r11, qword ptr [rbx + 0x20]
14146d12f: 4c 89 9d 58 05 00 00        	mov	qword ptr [rbp + 0x558], r11
14146d136: 4c 8b 5b 30                 	mov	r11, qword ptr [rbx + 0x30]
14146d13a: 4c 89 9d d8 08 00 00        	mov	qword ptr [rbp + 0x8d8], r11
14146d141: 44 8b 5b 3c                 	mov	r11d, dword ptr [rbx + 0x3c]
14146d145: 44 89 9d 0c 09 00 00        	mov	dword ptr [rbp + 0x90c], r11d
14146d14c: 4c 8b 5b 40                 	mov	r11, qword ptr [rbx + 0x40]
14146d150: 4c 89 9d 18 09 00 00        	mov	qword ptr [rbp + 0x918], r11
14146d157: 4c 8b 5b 50                 	mov	r11, qword ptr [rbx + 0x50]
14146d15b: 4c 89 9d 70 05 00 00        	mov	qword ptr [rbp + 0x570], r11
14146d162: 44 8b 5b 5c                 	mov	r11d, dword ptr [rbx + 0x5c]
14146d166: 44 89 9d c4 08 00 00        	mov	dword ptr [rbp + 0x8c4], r11d
14146d16d: 4c 8b 5b 60                 	mov	r11, qword ptr [rbx + 0x60]
14146d171: 4c 89 9d 80 05 00 00        	mov	qword ptr [rbp + 0x580], r11
14146d178: 4c 8b 5b 70                 	mov	r11, qword ptr [rbx + 0x70]
14146d17c: 4c 89 9d b0 08 00 00        	mov	qword ptr [rbp + 0x8b0], r11
14146d183: 44 8b 5b 7c                 	mov	r11d, dword ptr [rbx + 0x7c]
14146d187: 44 89 9d b8 08 00 00        	mov	dword ptr [rbp + 0x8b8], r11d
14146d18e: 4c 8b 9b 80 00 00 00        	mov	r11, qword ptr [rbx + 0x80]
14146d195: 4c 89 9d 70 08 00 00        	mov	qword ptr [rbp + 0x870], r11
14146d19c: 4c 8b 9b 88 00 00 00        	mov	r11, qword ptr [rbx + 0x88]
14146d1a3: 4c 89 9d f8 08 00 00        	mov	qword ptr [rbp + 0x8f8], r11
14146d1aa: 4c 8b 9b 98 00 00 00        	mov	r11, qword ptr [rbx + 0x98]
14146d1b1: 4c 89 9d 68 05 00 00        	mov	qword ptr [rbp + 0x568], r11
14146d1b8: 4c 8b 9b a8 00 00 00        	mov	r11, qword ptr [rbx + 0xa8]
14146d1bf: 4c 89 9d 28 05 00 00        	mov	qword ptr [rbp + 0x528], r11
14146d1c6: 44 8b 9b b4 00 00 00        	mov	r11d, dword ptr [rbx + 0xb4]
14146d1cd: 44 89 9d 5c 08 00 00        	mov	dword ptr [rbp + 0x85c], r11d
14146d1d4: 4c 8b 9b b8 00 00 00        	mov	r11, qword ptr [rbx + 0xb8]
14146d1db: 4c 89 9d 78 08 00 00        	mov	qword ptr [rbp + 0x878], r11
14146d1e2: 4c 8b 9b c8 00 00 00        	mov	r11, qword ptr [rbx + 0xc8]
14146d1e9: 4c 89 9d 30 05 00 00        	mov	qword ptr [rbp + 0x530], r11
14146d1f0: 44 8b 9b d4 00 00 00        	mov	r11d, dword ptr [rbx + 0xd4]
14146d1f7: 44 89 9d 60 08 00 00        	mov	dword ptr [rbp + 0x860], r11d
14146d1fe: 4c 8b 7f 28                 	mov	r15, qword ptr [rdi + 0x28]
14146d202: 4c 8b 77 38                 	mov	r14, qword ptr [rdi + 0x38]
14146d206: 8b 5f 44                    	mov	ebx, dword ptr [rdi + 0x44]
14146d209: 4c 8b 5f 48                 	mov	r11, qword ptr [rdi + 0x48]
14146d20d: 4c 89 9d 20 05 00 00        	mov	qword ptr [rbp + 0x520], r11
14146d214: 4c 8b 5f 58                 	mov	r11, qword ptr [rdi + 0x58]
14146d218: 4c 89 9d 78 05 00 00        	mov	qword ptr [rbp + 0x578], r11
14146d21f: 44 8b 5f 64                 	mov	r11d, dword ptr [rdi + 0x64]
14146d223: 44 89 9d c8 08 00 00        	mov	dword ptr [rbp + 0x8c8], r11d
14146d22a: 4c 8b 5f 68                 	mov	r11, qword ptr [rdi + 0x68]
14146d22e: 4c 89 9d 28 09 00 00        	mov	qword ptr [rbp + 0x928], r11
14146d235: 4c 8b 5f 78                 	mov	r11, qword ptr [rdi + 0x78]
14146d239: 4c 89 9d 00 09 00 00        	mov	qword ptr [rbp + 0x900], r11
14146d240: 44 8b 9f 84 00 00 00        	mov	r11d, dword ptr [rdi + 0x84]
14146d247: 44 89 9d 48 09 00 00        	mov	dword ptr [rbp + 0x948], r11d
14146d24e: 4c 8b 9f 88 00 00 00        	mov	r11, qword ptr [rdi + 0x88]
14146d255: 4c 89 9d 38 05 00 00        	mov	qword ptr [rbp + 0x538], r11
14146d25c: 4c 8b 9f 98 00 00 00        	mov	r11, qword ptr [rdi + 0x98]
14146d263: 4c 89 9d 40 05 00 00        	mov	qword ptr [rbp + 0x540], r11
14146d26a: 44 8b 9f a4 00 00 00        	mov	r11d, dword ptr [rdi + 0xa4]
14146d271: 44 89 9d 64 08 00 00        	mov	dword ptr [rbp + 0x864], r11d
14146d278: 4d 8b 5d 08                 	mov	r11, qword ptr [r13 + 0x8]
14146d27c: 49 8b 7d 28                 	mov	rdi, qword ptr [r13 + 0x28]
14146d280: 48 89 0e                    	mov	qword ptr [rsi], rcx
14146d283: 45 84 c1                    	test	r9b, r8b
14146d286: 0f 85 0a 01 00 00           	jne	0x14146d396 <.text+0x146c396>
14146d28c: 31 c9                       	xor	ecx, ecx
14146d28e: 4c 39 95 50 09 00 00        	cmp	qword ptr [rbp + 0x950], r10
14146d295: 4f 8d 04 92                 	lea	r8, [r10 + 4*r10]
14146d299: 4f 8d 04 c3                 	lea	r8, [r11 + 8*r8]
14146d29d: 41 b9 08 00 00 00           	mov	r9d, 0x8
14146d2a3: 4d 0f 42 c1                 	cmovb	r8, r9
14146d2a7: 4f 8d 14 a4                 	lea	r10, [r12 + 4*r12]
14146d2ab: 4f 8d 44 d0 d8              	lea	r8, [r8 + 8*r10 - 0x28]
14146d2b0: 49 0f 47 c8                 	cmova	rcx, r8
14146d2b4: 48 39 85 10 09 00 00        	cmp	qword ptr [rbp + 0x910], rax
14146d2bb: 48 8d 04 80                 	lea	rax, [rax + 4*rax]
14146d2bf: 48 8d 04 c7                 	lea	rax, [rdi + 8*rax]
14146d2c3: 49 0f 42 c1                 	cmovb	rax, r9
14146d2c7: 48 8d 14 92                 	lea	rdx, [rdx + 4*rdx]
14146d2cb: 48 8d 44 d0 d8              	lea	rax, [rax + 8*rdx - 0x28]
14146d2d0: 48 0f 46 c1                 	cmovbe	rax, rcx
14146d2d4: 83 38 0e                    	cmp	dword ptr [rax], 0xe
14146d2d7: 0f 85 b9 00 00 00           	jne	0x14146d396 <.text+0x146c396>
14146d2dd: 48 8b 85 58 09 00 00        	mov	rax, qword ptr [rbp + 0x958]
14146d2e4: 48 8b 30                    	mov	rsi, qword ptr [rax]
14146d2e7: 48 b8 01 00 00 00 00 00 00 80       	movabs	rax, -0x7fffffffffffffff
14146d2f1: 48 39 46 18                 	cmp	qword ptr [rsi + 0x18], rax
14146d2f5: 0f 85 91 00 00 00           	jne	0x14146d38c <.text+0x146c38c>
14146d2fb: 48 8d 4e 20                 	lea	rcx, [rsi + 0x20]
14146d2ff: 48 89 8d 18 05 00 00        	mov	qword ptr [rbp + 0x518], rcx
14146d306: e8 c5 9b 90 ff              	call	0x140d76ed0 <.text+0xd75ed0>
14146d30b: 49 89 d0                    	mov	r8, rdx
14146d30e: 48 8d 8d b0 05 00 00        	lea	rcx, [rbp + 0x5b0]
14146d315: 48 89 c2                    	mov	rdx, rax
14146d318: e8 f3 d3 8f ff              	call	0x140d6a710 <.text+0xd69710>
14146d31d: 83 bd b0 05 00 00 02        	cmp	dword ptr [rbp + 0x5b0], 0x2
14146d324: 0f 84 93 25 00 00           	je	0x14146f8bd <.text+0x146e8bd>
14146d32a: 0f 10 85 b0 05 00 00        	movups	xmm0, xmmword ptr [rbp + 0x5b0]
14146d331: 0f 10 8d c0 05 00 00        	movups	xmm1, xmmword ptr [rbp + 0x5c0]
14146d338: 0f 10 95 d0 05 00 00        	movups	xmm2, xmmword ptr [rbp + 0x5d0]
14146d33f: 0f 10 9d e0 05 00 00        	movups	xmm3, xmmword ptr [rbp + 0x5e0]
14146d346: 0f 29 85 60 03 00 00        	movaps	xmmword ptr [rbp + 0x360], xmm0
14146d34d: 48 8b 85 00 06 00 00        	mov	rax, qword ptr [rbp + 0x600]
14146d354: 48 89 85 b0 03 00 00        	mov	qword ptr [rbp + 0x3b0], rax
14146d35b: 0f 10 85 f0 05 00 00        	movups	xmm0, xmmword ptr [rbp + 0x5f0]
14146d362: 0f 29 85 a0 03 00 00        	movaps	xmmword ptr [rbp + 0x3a0], xmm0
14146d369: 0f 29 9d 90 03 00 00        	movaps	xmmword ptr [rbp + 0x390], xmm3
14146d370: 0f 29 95 80 03 00 00        	movaps	xmmword ptr [rbp + 0x380], xmm2
14146d377: 0f 29 8d 70 03 00 00        	movaps	xmmword ptr [rbp + 0x370], xmm1
14146d37e: 48 83 bd 60 03 00 00 02     	cmp	qword ptr [rbp + 0x360], 0x2
14146d386: 0f 84 4a 25 00 00           	je	0x14146f8d6 <.text+0x146e8d6>
14146d38c: 80 7e 48 00                 	cmp	byte ptr [rsi + 0x48], 0x0
14146d390: 0f 85 0f 25 00 00           	jne	0x14146f8a5 <.text+0x146e8a5>
14146d396: 48 8b 85 38 09 00 00        	mov	rax, qword ptr [rbp + 0x938]
14146d39d: 48 8b 00                    	mov	rax, qword ptr [rax]
14146d3a0: 83 38 0e                    	cmp	dword ptr [rax], 0xe
14146d3a3: 0f 85 7c 22 00 00           	jne	0x14146f625 <.text+0x146e625>
14146d3a9: 48 8b 85 58 09 00 00        	mov	rax, qword ptr [rbp + 0x958]
14146d3b0: 4c 8b 00                    	mov	r8, qword ptr [rax]
14146d3b3: 41 0f b6 40 48              	movzx	eax, byte ptr [r8 + 0x48]
14146d3b8: 85 c0                       	test	eax, eax
14146d3ba: 0f 84 04 02 00 00           	je	0x14146d5c4 <.text+0x146c5c4>
14146d3c0: 83 f8 02                    	cmp	eax, 0x2
14146d3c3: 0f 85 9f 04 00 00           	jne	0x14146d868 <.text+0x146c868>
14146d3c9: 48 8b 8d 58 09 00 00        	mov	rcx, qword ptr [rbp + 0x958]
14146d3d0: 8b 41 1c                    	mov	eax, dword ptr [rcx + 0x1c]
14146d3d3: 48 8b 49 10                 	mov	rcx, qword ptr [rcx + 0x10]
14146d3d7: 89 01                       	mov	dword ptr [rcx], eax
14146d3d9: 49 8b 08                    	mov	rcx, qword ptr [r8]
14146d3dc: 49 8b 40 08                 	mov	rax, qword ptr [r8 + 0x8]
14146d3e0: 49 8b 50 10                 	mov	rdx, qword ptr [r8 + 0x10]
14146d3e4: 49 c7 00 00 00 00 00        	mov	qword ptr [r8], 0x0
14146d3eb: 49 c7 40 08 08 00 00 00     	mov	qword ptr [r8 + 0x8], 0x8
14146d3f3: 49 c7 40 10 00 00 00 00     	mov	qword ptr [r8 + 0x10], 0x0
14146d3fb: 48 89 d3                    	mov	rbx, rdx
14146d3fe: 48 c1 e3 05                 	shl	rbx, 0x5
14146d402: 48 01 c3                    	add	rbx, rax
14146d405: 48 89 85 60 03 00 00        	mov	qword ptr [rbp + 0x360], rax
14146d40c: 48 89 85 68 03 00 00        	mov	qword ptr [rbp + 0x368], rax
14146d413: 48 89 8d 70 03 00 00        	mov	qword ptr [rbp + 0x370], rcx
14146d41a: 48 89 9d 78 03 00 00        	mov	qword ptr [rbp + 0x378], rbx
14146d421: 48 85 d2                    	test	rdx, rdx
14146d424: 0f 84 d2 04 00 00           	je	0x14146d8fc <.text+0x146c8fc>
14146d42a: 48 8b 8d e0 08 00 00        	mov	rcx, qword ptr [rbp + 0x8e0]
14146d431: 8b 51 1c                    	mov	edx, dword ptr [rcx + 0x1c]
14146d434: 89 95 58 09 00 00           	mov	dword ptr [rbp + 0x958], edx
14146d43a: 48 8b 11                    	mov	rdx, qword ptr [rcx]
14146d43d: 48 89 95 f8 08 00 00        	mov	qword ptr [rbp + 0x8f8], rdx
14146d444: 48 8b 49 10                 	mov	rcx, qword ptr [rcx + 0x10]
14146d448: 48 89 8d 38 09 00 00        	mov	qword ptr [rbp + 0x938], rcx
14146d44f: 48 8b 8d 28 09 00 00        	mov	rcx, qword ptr [rbp + 0x928]
14146d456: 48 83 c1 20                 	add	rcx, 0x20
14146d45a: 48 89 8d 18 09 00 00        	mov	qword ptr [rbp + 0x918], rcx
14146d461: 49 bf 03 00 00 00 00 00 00 80       	movabs	r15, -0x7ffffffffffffffd
14146d46b: 41 be 01 00 00 00           	mov	r14d, 0x1
14146d471: eb 48                       	jmp	0x14146d4bb <.text+0x146c4bb>
14146d473: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
14146d480: 48 8b 95 28 09 00 00        	mov	rdx, qword ptr [rbp + 0x928]
14146d487: 48 8b 42 28                 	mov	rax, qword ptr [rdx + 0x28]
14146d48b: 4a 8d 0c 6d 00 00 00 00     	lea	rcx, [2*r13]
14146d493: 4c 01 e9                    	add	rcx, r13
14146d496: 4c 89 24 c8                 	mov	qword ptr [rax + 8*rcx], r12
14146d49a: 0f 13 74 c8 08              	movlps	qword ptr [rax + 8*rcx + 0x8], xmm6
14146d49f: 48 89 74 c8 10              	mov	qword ptr [rax + 8*rcx + 0x10], rsi
14146d4a4: 49 ff c5                    	inc	r13
14146d4a7: 4c 89 6a 30                 	mov	qword ptr [rdx + 0x30], r13
14146d4ab: 48 ff 42 40                 	inc	qword ptr [rdx + 0x40]
14146d4af: 48 89 f8                    	mov	rax, rdi
14146d4b2: 48 39 df                    	cmp	rdi, rbx
14146d4b5: 0f 84 41 04 00 00           	je	0x14146d8fc <.text+0x146c8fc>
14146d4bb: 48 8d 78 20                 	lea	rdi, [rax + 0x20]
14146d4bf: 48 89 bd 68 03 00 00        	mov	qword ptr [rbp + 0x368], rdi
14146d4c6: 48 8b 08                    	mov	rcx, qword ptr [rax]
14146d4c9: 4c 39 f9                    	cmp	rcx, r15
14146d4cc: 0f 84 2a 04 00 00           	je	0x14146d8fc <.text+0x146c8fc>
14146d4d2: 4c 8b 60 08                 	mov	r12, qword ptr [rax + 0x8]
14146d4d6: 0f 10 70 10                 	movups	xmm6, xmmword ptr [rax + 0x10]
14146d4da: 48 89 c8                    	mov	rax, rcx
14146d4dd: 48 ba 00 00 00 00 00 00 00 80       	movabs	rdx, -0x8000000000000000
14146d4e7: 48 31 d0                    	xor	rax, rdx
14146d4ea: 48 83 f8 03                 	cmp	rax, 0x3
14146d4ee: 49 0f 43 c6                 	cmovae	rax, r14
14146d4f2: 48 85 c0                    	test	rax, rax
14146d4f5: 74 59                       	je	0x14146d550 <.text+0x146c550>
14146d4f7: 48 83 f8 01                 	cmp	rax, 0x1
14146d4fb: 0f 85 8f 00 00 00           	jne	0x14146d590 <.text+0x146c590>
14146d501: 8b 85 58 09 00 00           	mov	eax, dword ptr [rbp + 0x958]
14146d507: 48 8b 95 38 09 00 00        	mov	rdx, qword ptr [rbp + 0x938]
14146d50e: 89 02                       	mov	dword ptr [rdx], eax
14146d510: 48 89 8d b0 05 00 00        	mov	qword ptr [rbp + 0x5b0], rcx
14146d517: 4c 89 a5 b8 05 00 00        	mov	qword ptr [rbp + 0x5b8], r12
14146d51e: 0f 11 b5 c0 05 00 00        	movups	xmmword ptr [rbp + 0x5c0], xmm6
14146d525: 48 8b 8d f8 08 00 00        	mov	rcx, qword ptr [rbp + 0x8f8]
14146d52c: 48 8d 95 b0 05 00 00        	lea	rdx, [rbp + 0x5b0]
14146d533: 4c 8d 05 fe d8 7e 01        	lea	r8, [rip + 0x17ed8fe]   # 0x142c5ae38
14146d53a: e8 41 cc 6e ff              	call	0x140b5a180 <.text+0xb59180>
14146d53f: e9 6b ff ff ff              	jmp	0x14146d4af <.text+0x146c4af>
14146d544: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
14146d550: 48 8b 85 00 09 00 00        	mov	rax, qword ptr [rbp + 0x900]
14146d557: 8b 8d 48 09 00 00           	mov	ecx, dword ptr [rbp + 0x948]
14146d55d: 89 08                       	mov	dword ptr [rax], ecx
14146d55f: 48 8b 85 28 09 00 00        	mov	rax, qword ptr [rbp + 0x928]
14146d566: 4c 8b 68 30                 	mov	r13, qword ptr [rax + 0x30]
14146d56a: 48 8b 70 40                 	mov	rsi, qword ptr [rax + 0x40]
14146d56e: 4c 3b 68 20                 	cmp	r13, qword ptr [rax + 0x20]
14146d572: 0f 85 08 ff ff ff           	jne	0x14146d480 <.text+0x146c480>
14146d578: 48 8b 8d 18 09 00 00        	mov	rcx, qword ptr [rbp + 0x918]
14146d57f: 48 8d 15 c2 cb 7c 01        	lea	rdx, [rip + 0x17ccbc2]  # 0x142c3a148
14146d586: e8 25 58 0f ff              	call	0x140562db0 <.text+0x561db0>
14146d58b: e9 f0 fe ff ff              	jmp	0x14146d480 <.text+0x146c480>
14146d590: 4c 89 a5 b0 05 00 00        	mov	qword ptr [rbp + 0x5b0], r12
14146d597: 31 c0                       	xor	eax, eax
14146d599: 41 86 44 24 10              	xchg	byte ptr [r12 + 0x10], al
14146d59e: f0                          	lock
14146d59f: 49 ff 0c 24                 	dec	qword ptr [r12]
14146d5a3: 75 0c                       	jne	0x14146d5b1 <.text+0x146c5b1>
14146d5a5: 48 8d 8d b0 05 00 00        	lea	rcx, [rbp + 0x5b0]
14146d5ac: e8 cf 87 e7 fe              	call	0x1402e5d80 <.text+0x2e4d80>
14146d5b1: 48 8b bd 68 03 00 00        	mov	rdi, qword ptr [rbp + 0x368]
14146d5b8: 48 8b 9d 78 03 00 00        	mov	rbx, qword ptr [rbp + 0x378]
14146d5bf: e9 eb fe ff ff              	jmp	0x14146d4af <.text+0x146c4af>
14146d5c4: 49 8b 40 50                 	mov	rax, qword ptr [r8 + 0x50]
14146d5c8: 48 89 85 50 05 00 00        	mov	qword ptr [rbp + 0x550], rax
14146d5cf: 41 0f b6 40 49              	movzx	eax, byte ptr [r8 + 0x49]
14146d5d4: 88 85 28 09 00 00           	mov	byte ptr [rbp + 0x928], al
14146d5da: 41 0f b6 58 4a              	movzx	ebx, byte ptr [r8 + 0x4a]
14146d5df: 4c 89 85 00 09 00 00        	mov	qword ptr [rbp + 0x900], r8
14146d5e6: 45 0f b6 70 4b              	movzx	r14d, byte ptr [r8 + 0x4b]
14146d5eb: 48 8b 05 b6 ed 1b 02        	mov	rax, qword ptr [rip + 0x21bedb6] # 0x14362c3a8
14146d5f2: 48 8b 00                    	mov	rax, qword ptr [rax]
14146d5f5: 48 83 f8 04                 	cmp	rax, 0x4
14146d5f9: 4c 8b ad 20 05 00 00        	mov	r13, qword ptr [rbp + 0x520]
14146d600: 48 8b b5 60 09 00 00        	mov	rsi, qword ptr [rbp + 0x960]
14146d607: 0f 82 dc 00 00 00           	jb	0x14146d6e9 <.text+0x146c6e9>
14146d60d: 48 bf 00 00 00 00 00 00 00 80       	movabs	rdi, -0x8000000000000000
14146d617: 48 89 bd 30 09 00 00        	mov	qword ptr [rbp + 0x930], rdi
14146d61e: c6 85 76 09 00 00 01        	mov	byte ptr [rbp + 0x976], 0x1
14146d625: 48 8d 0d 04 6b 7f 01        	lea	rcx, [rip + 0x17f6b04]  # 0x142c64130
14146d62c: e8 4f 07 21 ff              	call	0x14067dd80 <.text+0x67cd80>
14146d631: 0f 10 00                    	movups	xmm0, xmmword ptr [rax]
14146d634: 8b 40 10                    	mov	eax, dword ptr [rax + 0x10]
14146d637: 48 c7 85 e0 05 00 00 04 00 00 00    	mov	qword ptr [rbp + 0x5e0], 0x4
14146d642: 48 8d 0d ff 6a 7f 01        	lea	rcx, [rip + 0x17f6aff]  # 0x142c64148
14146d649: 48 89 8d e8 05 00 00        	mov	qword ptr [rbp + 0x5e8], rcx
14146d650: 48 c7 85 f0 05 00 00 26 00 00 00    	mov	qword ptr [rbp + 0x5f0], 0x26
14146d65b: 48 8d 15 be 6a 7f 01        	lea	rdx, [rip + 0x17f6abe]  # 0x142c64120
14146d662: 48 89 95 00 06 00 00        	mov	qword ptr [rbp + 0x600], rdx
14146d669: 48 c7 85 08 06 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x608], 0x1
14146d674: 48 c7 85 10 06 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x610], 0x8
14146d67f: 48 c7 85 b0 05 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x5b0], 0x0
14146d68a: 0f 57 c9                    	xorps	xmm1, xmm1
14146d68d: 0f 11 8d 18 06 00 00        	movups	xmmword ptr [rbp + 0x618], xmm1
14146d694: 48 89 8d b8 05 00 00        	mov	qword ptr [rbp + 0x5b8], rcx
14146d69b: 48 c7 85 c0 05 00 00 26 00 00 00    	mov	qword ptr [rbp + 0x5c0], 0x26
14146d6a6: 48 c7 85 c8 05 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x5c8], 0x0
14146d6b1: 0f 11 85 d0 05 00 00        	movups	xmmword ptr [rbp + 0x5d0], xmm0
14146d6b8: c7 85 f8 05 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x5f8], 0x1
14146d6c2: 89 85 fc 05 00 00           	mov	dword ptr [rbp + 0x5fc], eax
14146d6c8: 48 89 bd 30 09 00 00        	mov	qword ptr [rbp + 0x930], rdi
14146d6cf: c6 85 76 09 00 00 01        	mov	byte ptr [rbp + 0x976], 0x1
14146d6d6: 48 8d 8d 72 09 00 00        	lea	rcx, [rbp + 0x972]
14146d6dd: 48 8d 95 b0 05 00 00        	lea	rdx, [rbp + 0x5b0]
14146d6e4: e8 47 cc 8d ff              	call	0x140d4a330 <.text+0xd49330>
14146d6e9: 48 8b 16                    	mov	rdx, qword ptr [rsi]
14146d6ec: 48 83 c2 60                 	add	rdx, 0x60
14146d6f0: 48 b9 00 00 00 00 00 00 00 80       	movabs	rcx, -0x8000000000000000
14146d6fa: 48 89 8d e8 08 00 00        	mov	qword ptr [rbp + 0x8e8], rcx
14146d701: 48 89 8d 30 09 00 00        	mov	qword ptr [rbp + 0x930], rcx
14146d708: c6 85 76 09 00 00 01        	mov	byte ptr [rbp + 0x976], 0x1
14146d70f: 48 8d 8d 60 03 00 00        	lea	rcx, [rbp + 0x360]
14146d716: 48 89 95 e0 08 00 00        	mov	qword ptr [rbp + 0x8e0], rdx
14146d71d: e8 de 88 6d ff              	call	0x140b46000 <.text+0xb45000>
14146d722: 48 8d 8d 60 03 00 00        	lea	rcx, [rbp + 0x360]
14146d729: e8 a2 97 90 ff              	call	0x140d76ed0 <.text+0xd75ed0>
14146d72e: 49 89 d0                    	mov	r8, rdx
14146d731: 48 c7 44 24 20 0b 00 00 00  	mov	qword ptr [rsp + 0x20], 0xb
14146d73a: 4c 8d 0d 2d 6a 7f 01        	lea	r9, [rip + 0x17f6a2d]   # 0x142c6416e
14146d741: 48 8d 8d b0 01 00 00        	lea	rcx, [rbp + 0x1b0]
14146d748: 48 89 c2                    	mov	rdx, rax
14146d74b: e8 c0 50 8f ff              	call	0x140d62810 <.text+0xd61810>
14146d750: c7 85 b0 05 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x5b0], 0x0
14146d75a: 0f 57 c0                    	xorps	xmm0, xmm0
14146d75d: 0f 11 85 b8 05 00 00        	movups	xmmword ptr [rbp + 0x5b8], xmm0
14146d764: c7 85 c8 05 00 00 07 00 00 00       	mov	dword ptr [rbp + 0x5c8], 0x7
14146d76e: 48 c7 85 cc 05 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x5cc], 0x0
14146d779: 66 c7 85 d4 05 00 00 00 00  	mov	word ptr [rbp + 0x5d4], 0x0
14146d782: c6 85 d1 05 00 00 01        	mov	byte ptr [rbp + 0x5d1], 0x1
14146d789: c6 85 d4 05 00 00 01        	mov	byte ptr [rbp + 0x5d4], 0x1
14146d790: c6 85 d3 05 00 00 01        	mov	byte ptr [rbp + 0x5d3], 0x1
14146d797: 48 8d 8d b0 01 00 00        	lea	rcx, [rbp + 0x1b0]
14146d79e: e8 2d 97 90 ff              	call	0x140d76ed0 <.text+0xd75ed0>
14146d7a3: 49 89 d0                    	mov	r8, rdx
14146d7a6: 48 8d 8d b0 05 00 00        	lea	rcx, [rbp + 0x5b0]
14146d7ad: 48 89 c2                    	mov	rdx, rax
14146d7b0: e8 0b c6 8e ff              	call	0x140d59dc0 <.text+0xd58dc0>
14146d7b5: 49 89 c4                    	mov	r12, rax
14146d7b8: 49 89 d7                    	mov	r15, rdx
14146d7bb: 48 8b 95 b0 01 00 00        	mov	rdx, qword ptr [rbp + 0x1b0]
14146d7c2: 48 85 d2                    	test	rdx, rdx
14146d7c5: 74 12                       	je	0x14146d7d9 <.text+0x146c7d9>
14146d7c7: 48 8b 8d b8 01 00 00        	mov	rcx, qword ptr [rbp + 0x1b8]
14146d7ce: 41 b8 01 00 00 00           	mov	r8d, 0x1
14146d7d4: e8 47 64 1a ff              	call	0x140613c20 <.text+0x612c20>
14146d7d9: 4d 85 e4                    	test	r12, r12
14146d7dc: 0f 84 e6 01 00 00           	je	0x14146d9c8 <.text+0x146c9c8>
14146d7e2: 44 89 f8                    	mov	eax, r15d
14146d7e5: 83 e0 03                    	and	eax, 0x3
14146d7e8: 83 f8 01                    	cmp	eax, 0x1
14146d7eb: 4c 8b a5 00 09 00 00        	mov	r12, qword ptr [rbp + 0x900]
14146d7f2: 0f 85 e0 01 00 00           	jne	0x14146d9d8 <.text+0x146c9d8>
14146d7f8: 4c 89 f8                    	mov	rax, r15
14146d7fb: 48 ff c8                    	dec	rax
14146d7fe: 48 89 85 10 09 00 00        	mov	qword ptr [rbp + 0x910], rax
14146d805: 49 8b 47 ff                 	mov	rax, qword ptr [r15 - 0x1]
14146d809: 48 89 85 38 09 00 00        	mov	qword ptr [rbp + 0x938], rax
14146d810: 49 8b 47 07                 	mov	rax, qword ptr [r15 + 0x7]
14146d814: 48 89 85 48 09 00 00        	mov	qword ptr [rbp + 0x948], rax
14146d81b: 48 8b 00                    	mov	rax, qword ptr [rax]
14146d81e: 48 85 c0                    	test	rax, rax
14146d821: 74 09                       	je	0x14146d82c <.text+0x146c82c>
14146d823: 48 8b 8d 38 09 00 00        	mov	rcx, qword ptr [rbp + 0x938]
14146d82a: ff d0                       	call	rax
14146d82c: 48 8b 8d 38 09 00 00        	mov	rcx, qword ptr [rbp + 0x938]
14146d833: 48 8b 85 48 09 00 00        	mov	rax, qword ptr [rbp + 0x948]
14146d83a: 48 8b 50 08                 	mov	rdx, qword ptr [rax + 0x8]
14146d83e: 48 85 d2                    	test	rdx, rdx
14146d841: 74 09                       	je	0x14146d84c <.text+0x146c84c>
14146d843: 4c 8b 40 10                 	mov	r8, qword ptr [rax + 0x10]
14146d847: e8 d4 63 1a ff              	call	0x140613c20 <.text+0x612c20>
14146d84c: ba 18 00 00 00              	mov	edx, 0x18
14146d851: 41 b8 08 00 00 00           	mov	r8d, 0x8
14146d857: 48 8b 8d 10 09 00 00        	mov	rcx, qword ptr [rbp + 0x910]
14146d85e: e8 bd 63 1a ff              	call	0x140613c20 <.text+0x612c20>
14146d863: e9 70 01 00 00              	jmp	0x14146d9d8 <.text+0x146c9d8>
14146d868: 4c 89 85 00 09 00 00        	mov	qword ptr [rbp + 0x900], r8
14146d86f: 45 8b 60 4c                 	mov	r12d, dword ptr [r8 + 0x4c]
14146d873: 45 85 e4                    	test	r12d, r12d
14146d876: 0f 88 a9 00 00 00           	js	0x14146d925 <.text+0x146c925>
14146d87c: 49 8d 47 20                 	lea	rax, [r15 + 0x20]
14146d880: 48 89 85 28 09 00 00        	mov	qword ptr [rbp + 0x928], rax
14146d887: 45 31 ed                    	xor	r13d, r13d
14146d88a: eb 29                       	jmp	0x14146d8b5 <.text+0x146c8b5>
14146d88c: 0f 1f 40 00                 	nop	dword ptr [rax]
14146d890: 41 ff c5                    	inc	r13d
14146d893: 49 8b 47 28                 	mov	rax, qword ptr [r15 + 0x28]
14146d897: 48 89 f9                    	mov	rcx, rdi
14146d89a: 48 c1 e1 04                 	shl	rcx, 0x4
14146d89e: 48 89 34 08                 	mov	qword ptr [rax + rcx], rsi
14146d8a2: c7 44 08 08 01 01 01 03     	mov	dword ptr [rax + rcx + 0x8], 0x3010101
14146d8aa: 48 ff c7                    	inc	rdi
14146d8ad: 49 89 7f 30                 	mov	qword ptr [r15 + 0x30], rdi
14146d8b1: 49 ff 47 40                 	inc	qword ptr [r15 + 0x40]
14146d8b5: 45 39 e5                    	cmp	r13d, r12d
14146d8b8: 0f 8d ea 00 00 00           	jge	0x14146d9a8 <.text+0x146c9a8>
14146d8be: 41 89 1e                    	mov	dword ptr [r14], ebx
14146d8c1: 49 8b 7f 30                 	mov	rdi, qword ptr [r15 + 0x30]
14146d8c5: 49 8b 77 40                 	mov	rsi, qword ptr [r15 + 0x40]
14146d8c9: 49 3b 7f 20                 	cmp	rdi, qword ptr [r15 + 0x20]
14146d8cd: 75 c1                       	jne	0x14146d890 <.text+0x146c890>
14146d8cf: 48 b8 00 00 00 00 00 00 00 80       	movabs	rax, -0x8000000000000000
14146d8d9: 48 89 85 30 09 00 00        	mov	qword ptr [rbp + 0x930], rax
14146d8e0: c6 85 76 09 00 00 01        	mov	byte ptr [rbp + 0x976], 0x1
14146d8e7: 48 8b 8d 28 09 00 00        	mov	rcx, qword ptr [rbp + 0x928]
14146d8ee: 48 8d 15 53 c8 7c 01        	lea	rdx, [rip + 0x17cc853]  # 0x142c3a148
14146d8f5: e8 36 4d 0f ff              	call	0x140562630 <.text+0x561630>
14146d8fa: eb 94                       	jmp	0x14146d890 <.text+0x146c890>
14146d8fc: 48 b8 00 00 00 00 00 00 00 80       	movabs	rax, -0x8000000000000000
14146d906: 48 89 85 30 09 00 00        	mov	qword ptr [rbp + 0x930], rax
14146d90d: c6 85 76 09 00 00 01        	mov	byte ptr [rbp + 0x976], 0x1
14146d914: 48 8d 8d 60 03 00 00        	lea	rcx, [rbp + 0x360]
14146d91b: e8 d0 d9 40 ff              	call	0x14087b2f0 <.text+0x87a2f0>
14146d920: e9 00 1d 00 00              	jmp	0x14146f625 <.text+0x146e625>
14146d925: 41 f7 dc                    	neg	r12d
14146d928: 49 8d 47 20                 	lea	rax, [r15 + 0x20]
14146d92c: 48 89 85 28 09 00 00        	mov	qword ptr [rbp + 0x928], rax
14146d933: 45 31 ed                    	xor	r13d, r13d
14146d936: eb 2d                       	jmp	0x14146d965 <.text+0x146c965>
14146d938: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
14146d940: 41 ff c5                    	inc	r13d
14146d943: 49 8b 47 28                 	mov	rax, qword ptr [r15 + 0x28]
14146d947: 48 89 f9                    	mov	rcx, rdi
14146d94a: 48 c1 e1 04                 	shl	rcx, 0x4
14146d94e: 48 89 34 08                 	mov	qword ptr [rax + rcx], rsi
14146d952: c7 44 08 08 01 00 01 03     	mov	dword ptr [rax + rcx + 0x8], 0x3010001
14146d95a: 48 ff c7                    	inc	rdi
14146d95d: 49 89 7f 30                 	mov	qword ptr [r15 + 0x30], rdi
14146d961: 49 ff 47 40                 	inc	qword ptr [r15 + 0x40]
14146d965: 45 39 e5                    	cmp	r13d, r12d
14146d968: 7d 3e                       	jge	0x14146d9a8 <.text+0x146c9a8>
14146d96a: 41 89 1e                    	mov	dword ptr [r14], ebx
14146d96d: 49 8b 7f 30                 	mov	rdi, qword ptr [r15 + 0x30]
14146d971: 49 8b 77 40                 	mov	rsi, qword ptr [r15 + 0x40]
14146d975: 49 3b 7f 20                 	cmp	rdi, qword ptr [r15 + 0x20]
14146d979: 75 c5                       	jne	0x14146d940 <.text+0x146c940>
14146d97b: 48 b8 00 00 00 00 00 00 00 80       	movabs	rax, -0x8000000000000000
14146d985: 48 89 85 30 09 00 00        	mov	qword ptr [rbp + 0x930], rax
14146d98c: c6 85 76 09 00 00 01        	mov	byte ptr [rbp + 0x976], 0x1
14146d993: 48 8b 8d 28 09 00 00        	mov	rcx, qword ptr [rbp + 0x928]
14146d99a: 48 8d 15 a7 c7 7c 01        	lea	rdx, [rip + 0x17cc7a7]  # 0x142c3a148
14146d9a1: e8 8a 4c 0f ff              	call	0x140562630 <.text+0x561630>
14146d9a6: eb 98                       	jmp	0x14146d940 <.text+0x146c940>
14146d9a8: 48 8b 8d 58 09 00 00        	mov	rcx, qword ptr [rbp + 0x958]
14146d9af: 8b 41 1c                    	mov	eax, dword ptr [rcx + 0x1c]
14146d9b2: 48 8b 49 10                 	mov	rcx, qword ptr [rcx + 0x10]
14146d9b6: 89 01                       	mov	dword ptr [rcx], eax
14146d9b8: 48 8b 85 00 09 00 00        	mov	rax, qword ptr [rbp + 0x900]
14146d9bf: c6 40 48 02                 	mov	byte ptr [rax + 0x48], 0x2
14146d9c3: e9 5d 1c 00 00              	jmp	0x14146f625 <.text+0x146e625>
14146d9c8: 4c 89 f9                    	mov	rcx, r15
14146d9cb: ff 15 1f 4a d2 01           	call	qword ptr [rip + 0x1d24a1f] # 0x1431923f0
14146d9d1: 4c 8b a5 00 09 00 00        	mov	r12, qword ptr [rbp + 0x900]
14146d9d8: 48 8b 95 60 03 00 00        	mov	rdx, qword ptr [rbp + 0x360]
14146d9df: 48 85 d2                    	test	rdx, rdx
14146d9e2: 74 12                       	je	0x14146d9f6 <.text+0x146c9f6>
14146d9e4: 48 8b 8d 68 03 00 00        	mov	rcx, qword ptr [rbp + 0x368]
14146d9eb: 41 b8 01 00 00 00           	mov	r8d, 0x1
14146d9f1: e8 2a 62 1a ff              	call	0x140613c20 <.text+0x612c20>
14146d9f6: 84 db                       	test	bl, bl
14146d9f8: 0f 84 4f 01 00 00           	je	0x14146db4d <.text+0x146cb4d>
14146d9fe: 48 8b 05 a3 e9 1b 02        	mov	rax, qword ptr [rip + 0x21be9a3] # 0x14362c3a8
14146da05: 48 8b 00                    	mov	rax, qword ptr [rax]
14146da08: 48 83 f8 03                 	cmp	rax, 0x3
14146da0c: 0f 82 dc 00 00 00           	jb	0x14146daee <.text+0x146caee>
14146da12: 48 be 00 00 00 00 00 00 00 80       	movabs	rsi, -0x8000000000000000
14146da1c: 48 89 b5 30 09 00 00        	mov	qword ptr [rbp + 0x930], rsi
14146da23: c6 85 76 09 00 00 01        	mov	byte ptr [rbp + 0x976], 0x1
14146da2a: 48 8d 0d 87 67 7f 01        	lea	rcx, [rip + 0x17f6787]  # 0x142c641b8
14146da31: e8 4a 03 21 ff              	call	0x14067dd80 <.text+0x67cd80>
14146da36: 0f 10 00                    	movups	xmm0, xmmword ptr [rax]
14146da39: 8b 40 10                    	mov	eax, dword ptr [rax + 0x10]
14146da3c: 48 c7 85 e0 05 00 00 03 00 00 00    	mov	qword ptr [rbp + 0x5e0], 0x3
14146da47: 48 8d 0d fa 66 7f 01        	lea	rcx, [rip + 0x17f66fa]  # 0x142c64148
14146da4e: 48 89 8d e8 05 00 00        	mov	qword ptr [rbp + 0x5e8], rcx
14146da55: 48 c7 85 f0 05 00 00 26 00 00 00    	mov	qword ptr [rbp + 0x5f0], 0x26
14146da60: 48 8d 15 41 67 7f 01        	lea	rdx, [rip + 0x17f6741]  # 0x142c641a8
14146da67: 48 89 95 00 06 00 00        	mov	qword ptr [rbp + 0x600], rdx
14146da6e: 48 c7 85 08 06 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x608], 0x1
14146da79: 48 c7 85 10 06 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x610], 0x8
14146da84: 48 c7 85 b0 05 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x5b0], 0x0
14146da8f: 0f 57 c9                    	xorps	xmm1, xmm1
14146da92: 0f 11 8d 18 06 00 00        	movups	xmmword ptr [rbp + 0x618], xmm1
14146da99: 48 89 8d b8 05 00 00        	mov	qword ptr [rbp + 0x5b8], rcx
14146daa0: 48 c7 85 c0 05 00 00 26 00 00 00    	mov	qword ptr [rbp + 0x5c0], 0x26
14146daab: 48 c7 85 c8 05 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x5c8], 0x0
14146dab6: 0f 11 85 d0 05 00 00        	movups	xmmword ptr [rbp + 0x5d0], xmm0
14146dabd: c7 85 f8 05 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x5f8], 0x1
14146dac7: 89 85 fc 05 00 00           	mov	dword ptr [rbp + 0x5fc], eax
14146dacd: 48 89 b5 30 09 00 00        	mov	qword ptr [rbp + 0x930], rsi
14146dad4: c6 85 76 09 00 00 01        	mov	byte ptr [rbp + 0x976], 0x1
14146dadb: 48 8d 8d 72 09 00 00        	lea	rcx, [rbp + 0x972]
14146dae2: 48 8d 95 b0 05 00 00        	lea	rdx, [rbp + 0x5b0]
14146dae9: e8 42 c8 8d ff              	call	0x140d4a330 <.text+0xd49330>
14146daee: 48 8b 85 78 05 00 00        	mov	rax, qword ptr [rbp + 0x578]
14146daf5: 8b 8d c8 08 00 00           	mov	ecx, dword ptr [rbp + 0x8c8]
14146dafb: 89 08                       	mov	dword ptr [rax], ecx
14146dafd: 49 8b 75 30                 	mov	rsi, qword ptr [r13 + 0x30]
14146db01: 49 8b 7d 40                 	mov	rdi, qword ptr [r13 + 0x40]
14146db05: 49 3b 75 20                 	cmp	rsi, qword ptr [r13 + 0x20]
14146db09: 75 28                       	jne	0x14146db33 <.text+0x146cb33>
14146db0b: 49 8d 4d 20                 	lea	rcx, [r13 + 0x20]
14146db0f: 48 b8 00 00 00 00 00 00 00 80       	movabs	rax, -0x8000000000000000
14146db19: 48 89 85 30 09 00 00        	mov	qword ptr [rbp + 0x930], rax
14146db20: c6 85 76 09 00 00 01        	mov	byte ptr [rbp + 0x976], 0x1
14146db27: 48 8d 15 1a c6 7c 01        	lea	rdx, [rip + 0x17cc61a]  # 0x142c3a148
14146db2e: e8 3d 4d 0f ff              	call	0x140562870 <.text+0x561870>
14146db33: 49 8b 45 28                 	mov	rax, qword ptr [r13 + 0x28]
14146db37: 48 89 3c f0                 	mov	qword ptr [rax + 8*rsi], rdi
14146db3b: 48 ff c6                    	inc	rsi
14146db3e: 49 89 75 30                 	mov	qword ptr [r13 + 0x30], rsi
14146db42: 49 ff 45 40                 	inc	qword ptr [r13 + 0x40]
14146db46: 4c 8b a5 00 09 00 00        	mov	r12, qword ptr [rbp + 0x900]
14146db4d: 48 8b 8d 58 09 00 00        	mov	rcx, qword ptr [rbp + 0x958]
14146db54: 8b 41 1c                    	mov	eax, dword ptr [rcx + 0x1c]
14146db57: 48 8b 49 10                 	mov	rcx, qword ptr [rcx + 0x10]
14146db5b: 89 85 cc 08 00 00           	mov	dword ptr [rbp + 0x8cc], eax
14146db61: 48 89 8d 88 05 00 00        	mov	qword ptr [rbp + 0x588], rcx
14146db68: 89 01                       	mov	dword ptr [rcx], eax
14146db6a: 49 8b 74 24 18              	mov	rsi, qword ptr [r12 + 0x18]
14146db6f: 49 8b 7c 24 38              	mov	rdi, qword ptr [r12 + 0x38]
14146db74: 49 8d 44 24 20              	lea	rax, [r12 + 0x20]
14146db79: 49 8b 4c 24 40              	mov	rcx, qword ptr [r12 + 0x40]
14146db7e: 48 89 8d 10 09 00 00        	mov	qword ptr [rbp + 0x910], rcx
14146db85: 48 bb 00 00 00 00 00 00 00 80       	movabs	rbx, -0x8000000000000000
14146db8f: 49 89 5c 24 18              	mov	qword ptr [r12 + 0x18], rbx
14146db94: 48 89 f2                    	mov	rdx, rsi
14146db97: 48 31 da                    	xor	rdx, rbx
14146db9a: 48 83 fa 02                 	cmp	rdx, 0x2
14146db9e: b9 02 00 00 00              	mov	ecx, 0x2
14146dba3: 48 0f 42 ca                 	cmovb	rcx, rdx
14146dba7: 48 83 f9 01                 	cmp	rcx, 0x1
14146dbab: 0f 84 85 05 00 00           	je	0x14146e136 <.text+0x146d136>
14146dbb1: 48 83 f9 02                 	cmp	rcx, 0x2
14146dbb5: 0f 85 26 1e 00 00           	jne	0x14146f9e1 <.text+0x146e9e1>
14146dbbb: 48 8b 48 10                 	mov	rcx, qword ptr [rax + 0x10]
14146dbbf: 48 89 8d a0 05 00 00        	mov	qword ptr [rbp + 0x5a0], rcx
14146dbc6: 0f 10 00                    	movups	xmm0, xmmword ptr [rax]
14146dbc9: 0f 29 85 90 05 00 00        	movaps	xmmword ptr [rbp + 0x590], xmm0
14146dbd0: 48 89 b5 30 08 00 00        	mov	qword ptr [rbp + 0x830], rsi
14146dbd7: 0f 28 85 90 05 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x590]
14146dbde: 0f 11 85 38 08 00 00        	movups	xmmword ptr [rbp + 0x838], xmm0
14146dbe5: 48 8b 85 a0 05 00 00        	mov	rax, qword ptr [rbp + 0x5a0]
14146dbec: 48 89 85 48 08 00 00        	mov	qword ptr [rbp + 0x848], rax
14146dbf3: 45 84 f6                    	test	r14b, r14b
14146dbf6: 74 0a                       	je	0x14146dc02 <.text+0x146cc02>
14146dbf8: c7 87 c0 00 00 00 1b 00 00 00       	mov	dword ptr [rdi + 0xc0], 0x1b
14146dc02: 48 89 bd 48 09 00 00        	mov	qword ptr [rbp + 0x948], rdi
14146dc09: 48 89 9d d0 08 00 00        	mov	qword ptr [rbp + 0x8d0], rbx
14146dc10: c6 85 71 09 00 00 01        	mov	byte ptr [rbp + 0x971], 0x1
14146dc17: c6 85 70 09 00 00 01        	mov	byte ptr [rbp + 0x970], 0x1
14146dc1e: 48 8d 8d 30 08 00 00        	lea	rcx, [rbp + 0x830]
14146dc25: e8 a6 92 90 ff              	call	0x140d76ed0 <.text+0xd75ed0>
14146dc2a: 49 89 c4                    	mov	r12, rax
14146dc2d: 49 89 d6                    	mov	r14, rdx
14146dc30: 48 8b 85 b0 08 00 00        	mov	rax, qword ptr [rbp + 0x8b0]
14146dc37: 8b 8d b8 08 00 00           	mov	ecx, dword ptr [rbp + 0x8b8]
14146dc3d: 89 08                       	mov	dword ptr [rax], ecx
14146dc3f: 48 8b 85 60 05 00 00        	mov	rax, qword ptr [rbp + 0x560]
14146dc46: 8b 8d c0 08 00 00           	mov	ecx, dword ptr [rbp + 0x8c0]
14146dc4c: 89 08                       	mov	dword ptr [rax], ecx
14146dc4e: 48 8b 85 d8 08 00 00        	mov	rax, qword ptr [rbp + 0x8d8]
14146dc55: 8b 8d 0c 09 00 00           	mov	ecx, dword ptr [rbp + 0x90c]
14146dc5b: 89 08                       	mov	dword ptr [rax], ecx
14146dc5d: 48 8b 85 70 05 00 00        	mov	rax, qword ptr [rbp + 0x570]
14146dc64: 8b 8d c4 08 00 00           	mov	ecx, dword ptr [rbp + 0x8c4]
14146dc6a: 89 08                       	mov	dword ptr [rax], ecx
14146dc6c: 48 8b 85 28 05 00 00        	mov	rax, qword ptr [rbp + 0x528]
14146dc73: 8b 8d 5c 08 00 00           	mov	ecx, dword ptr [rbp + 0x85c]
14146dc79: 89 08                       	mov	dword ptr [rax], ecx
14146dc7b: 48 8b 8d 48 09 00 00        	mov	rcx, qword ptr [rbp + 0x948]
14146dc82: 8b 81 c0 00 00 00           	mov	eax, dword ptr [rcx + 0xc0]
14146dc88: 83 f8 1a                    	cmp	eax, 0x1a
14146dc8b: 48 8b 05 76 e6 1b 02        	mov	rax, qword ptr [rip + 0x21be676] # 0x14362c308
14146dc92: 0f 9d 00                    	setge	byte ptr [rax]
14146dc95: 0f 9f 05 75 e3 1b 02        	setg	byte ptr [rip + 0x21be375] # 0x14362c011
14146dc9c: 48 8d 91 c4 00 00 00        	lea	rdx, [rcx + 0xc4]
14146dca3: 48 89 9d 40 09 00 00        	mov	qword ptr [rbp + 0x940], rbx
14146dcaa: c6 85 77 09 00 00 01        	mov	byte ptr [rbp + 0x977], 0x1
14146dcb1: 48 8b 8d a8 08 00 00        	mov	rcx, qword ptr [rbp + 0x8a8]
14146dcb8: 41 b0 01                    	mov	r8b, 0x1
14146dcbb: e8 60 38 4a ff              	call	0x140911520 <.text+0x910520>
14146dcc0: 48 b8 00 00 00 00 00 00 00 80       	movabs	rax, -0x8000000000000000
14146dcca: 48 89 85 40 09 00 00        	mov	qword ptr [rbp + 0x940], rax
14146dcd1: c6 85 77 09 00 00 01        	mov	byte ptr [rbp + 0x977], 0x1
14146dcd8: 48 8b 8d a8 08 00 00        	mov	rcx, qword ptr [rbp + 0x8a8]
14146dcdf: e8 6c 3e 4a ff              	call	0x140911b50 <.text+0x910b50>
14146dce4: 4c 8b 85 48 09 00 00        	mov	r8, qword ptr [rbp + 0x948]
14146dceb: 41 0f 10 80 94 00 00 00     	movups	xmm0, xmmword ptr [r8 + 0x94]
14146dcf3: 41 0f 10 88 a4 00 00 00     	movups	xmm1, xmmword ptr [r8 + 0xa4]
14146dcfb: 41 0f 10 90 b0 00 00 00     	movups	xmm2, xmmword ptr [r8 + 0xb0]
14146dd03: 48 8b 85 58 05 00 00        	mov	rax, qword ptr [rbp + 0x558]
14146dd0a: 0f 11 50 1c                 	movups	xmmword ptr [rax + 0x1c], xmm2
14146dd0e: 0f 11 48 10                 	movups	xmmword ptr [rax + 0x10], xmm1
14146dd12: 0f 11 00                    	movups	xmmword ptr [rax], xmm0
14146dd15: 49 8b 40 30                 	mov	rax, qword ptr [r8 + 0x30]
14146dd19: 48 89 85 50 09 00 00        	mov	qword ptr [rbp + 0x950], rax
14146dd20: 49 8b 40 38                 	mov	rax, qword ptr [r8 + 0x38]
14146dd24: 48 89 85 60 09 00 00        	mov	qword ptr [rbp + 0x960], rax
14146dd2b: 49 8b 70 40                 	mov	rsi, qword ptr [r8 + 0x40]
14146dd2f: 41 8b 80 90 00 00 00        	mov	eax, dword ptr [r8 + 0x90]
14146dd36: 48 8b 8d 68 05 00 00        	mov	rcx, qword ptr [rbp + 0x568]
14146dd3d: 89 41 40                    	mov	dword ptr [rcx + 0x40], eax
14146dd40: 41 0f 10 80 80 00 00 00     	movups	xmm0, xmmword ptr [r8 + 0x80]
14146dd48: 0f 11 41 30                 	movups	xmmword ptr [rcx + 0x30], xmm0
14146dd4c: 41 0f 10 40 70              	movups	xmm0, xmmword ptr [r8 + 0x70]
14146dd51: 0f 11 41 20                 	movups	xmmword ptr [rcx + 0x20], xmm0
14146dd55: 41 0f 10 40 60              	movups	xmm0, xmmword ptr [r8 + 0x60]
14146dd5a: 0f 11 41 10                 	movups	xmmword ptr [rcx + 0x10], xmm0
14146dd5e: 41 0f 10 40 50              	movups	xmm0, xmmword ptr [r8 + 0x50]
14146dd63: 0f 11 01                    	movups	xmmword ptr [rcx], xmm0
14146dd66: 41 8b 80 e8 00 00 00        	mov	eax, dword ptr [r8 + 0xe8]
14146dd6d: 89 85 38 09 00 00           	mov	dword ptr [rbp + 0x938], eax
14146dd73: 41 8b 80 ec 00 00 00        	mov	eax, dword ptr [r8 + 0xec]
14146dd7a: 89 85 58 09 00 00           	mov	dword ptr [rbp + 0x958], eax
14146dd80: 41 0f 10 00                 	movups	xmm0, xmmword ptr [r8]
14146dd84: 0f 29 45 b0                 	movaps	xmmword ptr [rbp - 0x50], xmm0
14146dd88: 4d 8b 48 10                 	mov	r9, qword ptr [r8 + 0x10]
14146dd8c: 49 8b 40 18                 	mov	rax, qword ptr [r8 + 0x18]
14146dd90: 49 8b 48 20                 	mov	rcx, qword ptr [r8 + 0x20]
14146dd94: 49 8b 50 28                 	mov	rdx, qword ptr [r8 + 0x28]
14146dd98: 4d 8b 50 48                 	mov	r10, qword ptr [r8 + 0x48]
14146dd9c: 4c 89 95 b8 08 00 00        	mov	qword ptr [rbp + 0x8b8], r10
14146dda3: 45 8b 80 c0 00 00 00        	mov	r8d, dword ptr [r8 + 0xc0]
14146ddaa: 44 89 85 0c 09 00 00        	mov	dword ptr [rbp + 0x90c], r8d
14146ddb1: 48 c1 e2 04                 	shl	rdx, 0x4
14146ddb5: 48 01 ca                    	add	rdx, rcx
14146ddb8: 48 89 8d b0 05 00 00        	mov	qword ptr [rbp + 0x5b0], rcx
14146ddbf: 48 89 8d b8 05 00 00        	mov	qword ptr [rbp + 0x5b8], rcx
14146ddc6: 48 89 85 c0 05 00 00        	mov	qword ptr [rbp + 0x5c0], rax
14146ddcd: 48 89 95 c8 05 00 00        	mov	qword ptr [rbp + 0x5c8], rdx
14146ddd4: 0f 29 85 b0 01 00 00        	movaps	xmmword ptr [rbp + 0x1b0], xmm0
14146dddb: 4c 89 8d b0 08 00 00        	mov	qword ptr [rbp + 0x8b0], r9
14146dde2: 4c 89 8d c0 01 00 00        	mov	qword ptr [rbp + 0x1c0], r9
14146dde9: 4c 8d 05 68 ae 7c 01        	lea	r8, [rip + 0x17cae68]   # 0x142c38c58
14146ddf0: 48 8d 8d 60 03 00 00        	lea	rcx, [rbp + 0x360]
14146ddf7: 48 8d 95 b0 05 00 00        	lea	rdx, [rbp + 0x5b0]
14146ddfe: e8 0d 97 2c 00              	call	0x141737510 <.text+0x1736510>
14146de03: 48 89 b5 d8 08 00 00        	mov	qword ptr [rbp + 0x8d8], rsi
14146de0a: 0f 10 85 60 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x360]
14146de11: 0f 11 85 c8 05 00 00        	movups	xmmword ptr [rbp + 0x5c8], xmm0
14146de18: 48 8b 85 70 03 00 00        	mov	rax, qword ptr [rbp + 0x370]
14146de1f: 48 89 85 d8 05 00 00        	mov	qword ptr [rbp + 0x5d8], rax
14146de26: 0f 28 85 b0 01 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x1b0]
14146de2d: 0f 29 85 b0 05 00 00        	movaps	xmmword ptr [rbp + 0x5b0], xmm0
14146de34: 48 8b 85 c0 01 00 00        	mov	rax, qword ptr [rbp + 0x1c0]
14146de3b: 48 89 85 c0 05 00 00        	mov	qword ptr [rbp + 0x5c0], rax
14146de42: 48 8b 8d 18 09 00 00        	mov	rcx, qword ptr [rbp + 0x918]
14146de49: e8 a2 ce 41 ff              	call	0x14088acf0 <.text+0x889cf0>
14146de4e: 0f 28 85 b0 05 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x5b0]
14146de55: 0f 28 8d c0 05 00 00        	movaps	xmm1, xmmword ptr [rbp + 0x5c0]
14146de5c: 0f 28 95 d0 05 00 00        	movaps	xmm2, xmmword ptr [rbp + 0x5d0]
14146de63: 48 8b 8d 18 09 00 00        	mov	rcx, qword ptr [rbp + 0x918]
14146de6a: 0f 11 51 20                 	movups	xmmword ptr [rcx + 0x20], xmm2
14146de6e: 0f 11 49 10                 	movups	xmmword ptr [rcx + 0x10], xmm1
14146de72: 0f 11 01                    	movups	xmmword ptr [rcx], xmm0
14146de75: 48 b8 00 00 00 00 00 00 00 80       	movabs	rax, -0x8000000000000000
14146de7f: 48 89 41 30                 	mov	qword ptr [rcx + 0x30], rax
14146de83: 48 8b 85 b0 08 00 00        	mov	rax, qword ptr [rbp + 0x8b0]
14146de8a: 48 89 41 58                 	mov	qword ptr [rcx + 0x58], rax
14146de8e: 48 8b 85 b8 08 00 00        	mov	rax, qword ptr [rbp + 0x8b8]
14146de95: 48 89 41 60                 	mov	qword ptr [rcx + 0x60], rax
14146de99: 8b 85 38 09 00 00           	mov	eax, dword ptr [rbp + 0x938]
14146de9f: 89 41 68                    	mov	dword ptr [rcx + 0x68], eax
14146dea2: 8b 85 58 09 00 00           	mov	eax, dword ptr [rbp + 0x958]
14146dea8: 89 41 6c                    	mov	dword ptr [rcx + 0x6c], eax
14146deab: 8b 85 0c 09 00 00           	mov	eax, dword ptr [rbp + 0x90c]
14146deb1: 89 41 70                    	mov	dword ptr [rcx + 0x70], eax
14146deb4: 48 0f ba a5 10 09 00 00 38  	bt	qword ptr [rbp + 0x910], 0x38
14146debd: 73 63                       	jae	0x14146df22 <.text+0x146cf22>
14146debf: 48 8b 85 78 05 00 00        	mov	rax, qword ptr [rbp + 0x578]
14146dec6: 8b 8d c8 08 00 00           	mov	ecx, dword ptr [rbp + 0x8c8]
14146decc: 89 08                       	mov	dword ptr [rax], ecx
14146dece: 49 8b 75 30                 	mov	rsi, qword ptr [r13 + 0x30]
14146ded2: 49 8b 7d 40                 	mov	rdi, qword ptr [r13 + 0x40]
14146ded6: 49 3b 75 20                 	cmp	rsi, qword ptr [r13 + 0x20]
14146deda: 48 8b 85 60 09 00 00        	mov	rax, qword ptr [rbp + 0x960]
14146dee1: 48 8b 95 50 09 00 00        	mov	rdx, qword ptr [rbp + 0x950]
14146dee8: 75 25                       	jne	0x14146df0f <.text+0x146cf0f>
14146deea: 49 8d 4d 20                 	lea	rcx, [r13 + 0x20]
14146deee: 48 89 95 40 09 00 00        	mov	qword ptr [rbp + 0x940], rdx
14146def5: 48 89 85 20 09 00 00        	mov	qword ptr [rbp + 0x920], rax
14146defc: c6 85 77 09 00 00 00        	mov	byte ptr [rbp + 0x977], 0x0
14146df03: 48 8d 15 3e c2 7c 01        	lea	rdx, [rip + 0x17cc23e]  # 0x142c3a148
14146df0a: e8 61 49 0f ff              	call	0x140562870 <.text+0x561870>
14146df0f: 49 8b 45 28                 	mov	rax, qword ptr [r13 + 0x28]
14146df13: 48 89 3c f0                 	mov	qword ptr [rax + 8*rsi], rdi
14146df17: 48 ff c6                    	inc	rsi
14146df1a: 49 89 75 30                 	mov	qword ptr [r13 + 0x30], rsi
14146df1e: 49 ff 45 40                 	inc	qword ptr [r13 + 0x40]
14146df22: 80 bd 28 09 00 00 00        	cmp	byte ptr [rbp + 0x928], 0x0
14146df29: 75 7b                       	jne	0x14146dfa6 <.text+0x146cfa6>
14146df2b: 4c 8b bd 60 0a 00 00        	mov	r15, qword ptr [rbp + 0xa60]
14146df32: 48 8b 85 50 09 00 00        	mov	rax, qword ptr [rbp + 0x950]
14146df39: 48 89 85 40 09 00 00        	mov	qword ptr [rbp + 0x940], rax
14146df40: 48 8b 85 60 09 00 00        	mov	rax, qword ptr [rbp + 0x960]
14146df47: 48 89 85 20 09 00 00        	mov	qword ptr [rbp + 0x920], rax
14146df4e: c6 85 77 09 00 00 00        	mov	byte ptr [rbp + 0x977], 0x0
14146df55: 4c 89 7c 24 20              	mov	qword ptr [rsp + 0x20], r15
14146df5a: 48 8d 8d b0 05 00 00        	lea	rcx, [rbp + 0x5b0]
14146df61: 48 8b 95 18 09 00 00        	mov	rdx, qword ptr [rbp + 0x918]
14146df68: 4d 89 e0                    	mov	r8, r12
14146df6b: 4d 89 f1                    	mov	r9, r14
14146df6e: e8 dd 8e 33 00              	call	0x1417a6e50 <.text+0x17a5e50>
14146df73: 48 8b b5 b8 05 00 00        	mov	rsi, qword ptr [rbp + 0x5b8]
14146df7a: 48 83 fe 03                 	cmp	rsi, 0x3
14146df7e: 74 26                       	je	0x14146dfa6 <.text+0x146cfa6>
14146df80: 8b bd b0 05 00 00           	mov	edi, dword ptr [rbp + 0x5b0]
14146df86: 48 8d 95 c0 05 00 00        	lea	rdx, [rbp + 0x5c0]
14146df8d: 48 8d 4d b0                 	lea	rcx, [rbp - 0x50]
14146df91: 41 b8 a8 01 00 00           	mov	r8d, 0x1a8
14146df97: e8 64 5c 4b 01              	call	0x142923c00 <.text+0x2922c00>
14146df9c: 48 83 fe 02                 	cmp	rsi, 0x2
14146dfa0: 0f 85 8b 0b 00 00           	jne	0x14146eb31 <.text+0x146db31>
14146dfa6: 48 8b bd 18 09 00 00        	mov	rdi, qword ptr [rbp + 0x918]
14146dfad: c7 47 68 ff ff ff ff        	mov	dword ptr [rdi + 0x68], 0xffffffff
14146dfb4: 48 8b 47 10                 	mov	rax, qword ptr [rdi + 0x10]
14146dfb8: 48 85 c0                    	test	rax, rax
14146dfbb: 0f 84 e5 07 00 00           	je	0x14146e7a6 <.text+0x146d7a6>
14146dfc1: 48 8b 4f 08                 	mov	rcx, qword ptr [rdi + 0x8]
14146dfc5: 48 8d 04 80                 	lea	rax, [rax + 4*rax]
14146dfc9: 48 8d 04 c1                 	lea	rax, [rcx + 8*rax]
14146dfcd: 48 89 85 38 09 00 00        	mov	qword ptr [rbp + 0x938], rax
14146dfd4: 45 31 c0                    	xor	r8d, r8d
14146dfd7: 49 ba 01 00 00 00 00 00 00 80       	movabs	r10, -0x7fffffffffffffff
14146dfe1: 49 bb 10 00 00 00 00 00 00 80       	movabs	r11, -0x7ffffffffffffff0
14146dfeb: 48 8d 3d fe 6b 7f 01        	lea	rdi, [rip + 0x17f6bfe]  # 0x142c64bf0
14146dff2: 45 31 c9                    	xor	r9d, r9d
14146dff5: eb 21                       	jmp	0x14146e018 <.text+0x146d018>
14146dff7: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
14146e000: 48 8b 8d 28 09 00 00        	mov	rcx, qword ptr [rbp + 0x928]
14146e007: 48 83 c1 28                 	add	rcx, 0x28
14146e00b: 48 3b 8d 38 09 00 00        	cmp	rcx, qword ptr [rbp + 0x938]
14146e012: 0f 84 91 07 00 00           	je	0x14146e7a9 <.text+0x146d7a9>
14146e018: 48 8b 41 10                 	mov	rax, qword ptr [rcx + 0x10]
14146e01c: 48 85 c0                    	test	rax, rax
14146e01f: 48 89 8d 28 09 00 00        	mov	qword ptr [rbp + 0x928], rcx
14146e026: 74 d8                       	je	0x14146e000 <.text+0x146d000>
14146e028: 4c 8b 61 08                 	mov	r12, qword ptr [rcx + 0x8]
14146e02c: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
14146e030: 49 8d 04 c4                 	lea	rax, [r12 + 8*rax]
14146e034: 48 89 85 58 09 00 00        	mov	qword ptr [rbp + 0x958], rax
14146e03b: eb 2e                       	jmp	0x14146e06b <.text+0x146d06b>
14146e03d: 0f 1f 00                    	nop	dword ptr [rax]
14146e040: 49 8b 44 24 08              	mov	rax, qword ptr [r12 + 0x8]
14146e045: 0f b6 50 10                 	movzx	edx, byte ptr [rax + 0x10]
14146e049: 48 8d 0d 78 6b 7f 01        	lea	rcx, [rip + 0x17f6b78]  # 0x142c64bc8
14146e050: 48 03 04 d1                 	add	rax, qword ptr [rcx + 8*rdx]
14146e054: 48 8b 00                    	mov	rax, qword ptr [rax]
14146e057: 4c 39 c8                    	cmp	rax, r9
14146e05a: 4c 0f 47 c8                 	cmova	r9, rax
14146e05e: 49 83 c4 18                 	add	r12, 0x18
14146e062: 4c 3b a5 58 09 00 00        	cmp	r12, qword ptr [rbp + 0x958]
14146e069: 74 95                       	je	0x14146e000 <.text+0x146d000>
14146e06b: 49 8b 04 24                 	mov	rax, qword ptr [r12]
14146e06f: 48 83 f8 07                 	cmp	rax, 0x7
14146e073: 74 4b                       	je	0x14146e0c0 <.text+0x146d0c0>
14146e075: 83 f8 08                    	cmp	eax, 0x8
14146e078: 74 c6                       	je	0x14146e040 <.text+0x146d040>
14146e07a: 83 f8 09                    	cmp	eax, 0x9
14146e07d: 75 df                       	jne	0x14146e05e <.text+0x146d05e>
14146e07f: 49 8b 44 24 08              	mov	rax, qword ptr [r12 + 0x8]
14146e084: 0f b6 50 18                 	movzx	edx, byte ptr [rax + 0x18]
14146e088: 80 c2 fd                    	add	dl, -0x3
14146e08b: 80 fa 0e                    	cmp	dl, 0xe
14146e08e: 0f b6 d2                    	movzx	edx, dl
14146e091: b9 03 00 00 00              	mov	ecx, 0x3
14146e096: 0f 43 d1                    	cmovae	edx, ecx
14146e099: 0f b6 d2                    	movzx	edx, dl
14146e09c: 48 8d 0d 4d 6c 7f 01        	lea	rcx, [rip + 0x17f6c4d]  # 0x142c64cf0
14146e0a3: 48 63 14 91                 	movsxd	rdx, dword ptr [rcx + 4*rdx]
14146e0a7: 48 01 ca                    	add	rdx, rcx
14146e0aa: ff e2                       	jmp	rdx
14146e0ac: 48 83 c0 20                 	add	rax, 0x20
14146e0b0: eb a2                       	jmp	0x14146e054 <.text+0x146d054>
14146e0b2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nop	word ptr cs:[rax + rax]
14146e0c0: 49 8b 74 24 10              	mov	rsi, qword ptr [r12 + 0x10]
14146e0c5: 48 85 f6                    	test	rsi, rsi
14146e0c8: 74 94                       	je	0x14146e05e <.text+0x146d05e>
14146e0ca: 49 8b 44 24 08              	mov	rax, qword ptr [r12 + 0x8]
14146e0cf: 48 c1 e6 04                 	shl	rsi, 0x4
14146e0d3: 48 01 c6                    	add	rsi, rax
14146e0d6: eb 15                       	jmp	0x14146e0ed <.text+0x146d0ed>
14146e0d8: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
14146e0e0: 48 83 c0 10                 	add	rax, 0x10
14146e0e4: 48 39 f0                    	cmp	rax, rsi
14146e0e7: 0f 84 71 ff ff ff           	je	0x14146e05e <.text+0x146d05e>
14146e0ed: 48 8b 50 08                 	mov	rdx, qword ptr [rax + 0x8]
14146e0f1: 48 85 d2                    	test	rdx, rdx
14146e0f4: 74 ea                       	je	0x14146e0e0 <.text+0x146d0e0>
14146e0f6: 48 8b 18                    	mov	rbx, qword ptr [rax]
14146e0f9: 48 c1 e2 03                 	shl	rdx, 0x3
14146e0fd: 45 31 f6                    	xor	r14d, r14d
14146e100: 4e 8b 3c 33                 	mov	r15, qword ptr [rbx + r14]
14146e104: 49 8b 4f 10                 	mov	rcx, qword ptr [r15 + 0x10]
14146e108: 4e 8d 2c 11                 	lea	r13, [rcx + r10]
14146e10c: 4c 39 d9                    	cmp	rcx, r11
14146e10f: 4d 0f 4d e8                 	cmovge	r13, r8
14146e113: 4a 8b 0c ef                 	mov	rcx, qword ptr [rdi + 8*r13]
14146e117: 49 8b 0c 0f                 	mov	rcx, qword ptr [r15 + rcx]
14146e11b: 4c 39 c9                    	cmp	rcx, r9
14146e11e: 4c 0f 47 c9                 	cmova	r9, rcx
14146e122: 49 83 c6 08                 	add	r14, 0x8
14146e126: 4c 39 f2                    	cmp	rdx, r14
14146e129: 75 d5                       	jne	0x14146e100 <.text+0x146d100>
14146e12b: eb b3                       	jmp	0x14146e0e0 <.text+0x146d0e0>
14146e12d: 48 83 c0 10                 	add	rax, 0x10
14146e131: e9 1e ff ff ff              	jmp	0x14146e054 <.text+0x146d054>
14146e136: 44 88 f6                    	mov	sil, r14b
14146e139: 48 8b 48 10                 	mov	rcx, qword ptr [rax + 0x10]
14146e13d: 48 89 4d c0                 	mov	qword ptr [rbp - 0x40], rcx
14146e141: 0f 10 00                    	movups	xmm0, xmmword ptr [rax]
14146e144: 0f 29 45 b0                 	movaps	xmmword ptr [rbp - 0x50], xmm0
14146e148: 48 89 7d c8                 	mov	qword ptr [rbp - 0x38], rdi
14146e14c: 4c 8d 7d b0                 	lea	r15, [rbp - 0x50]
14146e150: 4c 89 f9                    	mov	rcx, r15
14146e153: e8 78 8d 90 ff              	call	0x140d76ed0 <.text+0xd75ed0>
14146e158: 49 89 d0                    	mov	r8, rdx
14146e15b: 48 8b 8d 70 0a 00 00        	mov	rcx, qword ptr [rbp + 0xa70]
14146e162: 4c 8b 09                    	mov	r9, qword ptr [rcx]
14146e165: 4c 8d b5 b0 01 00 00        	lea	r14, [rbp + 0x1b0]
14146e16c: 4c 89 f1                    	mov	rcx, r14
14146e16f: 48 89 c2                    	mov	rdx, rax
14146e172: e8 89 2b 00 00              	call	0x141470d00 <.text+0x146fd00>
14146e177: 0f b6 9d bf 01 00 00        	movzx	ebx, byte ptr [rbp + 0x1bf]
14146e17e: 80 fb 02                    	cmp	bl, 0x2
14146e181: 0f 85 91 08 00 00           	jne	0x14146ea18 <.text+0x146da18>
14146e187: 48 8b bd b0 01 00 00        	mov	rdi, qword ptr [rbp + 0x1b0]
14146e18e: 48 89 bd 68 08 00 00        	mov	qword ptr [rbp + 0x868], rdi
14146e195: 48 8d b5 b0 05 00 00        	lea	rsi, [rbp + 0x5b0]
14146e19c: 48 89 f1                    	mov	rcx, rsi
14146e19f: e8 3c 21 7a ff              	call	0x140c102e0 <.text+0xc0f2e0>
14146e1a4: 48 8b 85 60 09 00 00        	mov	rax, qword ptr [rbp + 0x960]
14146e1ab: 8b 48 1c                    	mov	ecx, dword ptr [rax + 0x1c]
14146e1ae: 48 8b 40 10                 	mov	rax, qword ptr [rax + 0x10]
14146e1b2: 89 08                       	mov	dword ptr [rax], ecx
14146e1b4: 0f 10 85 b0 05 00 00        	movups	xmm0, xmmword ptr [rbp + 0x5b0]
14146e1bb: 48 8b 85 e0 08 00 00        	mov	rax, qword ptr [rbp + 0x8e0]
14146e1c2: 0f 11 00                    	movups	xmmword ptr [rax], xmm0
14146e1c5: 4c 89 bd b0 01 00 00        	mov	qword ptr [rbp + 0x1b0], r15
14146e1cc: 48 8d 05 0d 39 8f ff        	lea	rax, [rip - 0x70c6f3]   # 0x140d61ae0 <.text+0xd60ae0>
14146e1d3: 48 89 85 b8 01 00 00        	mov	qword ptr [rbp + 0x1b8], rax
14146e1da: 48 8d 85 68 08 00 00        	lea	rax, [rbp + 0x868]
14146e1e1: 48 89 85 c0 01 00 00        	mov	qword ptr [rbp + 0x1c0], rax
14146e1e8: 48 8d 05 21 ea 8d ff        	lea	rax, [rip - 0x7215df]   # 0x140d4cc10 <.text+0xd4bc10>
14146e1ef: 48 89 85 c8 01 00 00        	mov	qword ptr [rbp + 0x1c8], rax
14146e1f6: 48 c7 85 b0 05 00 00 02 00 00 00    	mov	qword ptr [rbp + 0x5b0], 0x2
14146e201: 48 c7 85 c0 05 00 00 02 00 00 00    	mov	qword ptr [rbp + 0x5c0], 0x2
14146e20c: 48 c7 85 d0 05 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x5d0], 0x0
14146e217: 48 c7 85 d8 05 00 00 20 00 00 00    	mov	qword ptr [rbp + 0x5d8], 0x20
14146e222: c6 85 e0 05 00 00 03        	mov	byte ptr [rbp + 0x5e0], 0x3
14146e229: 48 c7 85 e8 05 00 00 02 00 00 00    	mov	qword ptr [rbp + 0x5e8], 0x2
14146e234: 48 c7 85 f8 05 00 00 02 00 00 00    	mov	qword ptr [rbp + 0x5f8], 0x2
14146e23f: 48 c7 85 08 06 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x608], 0x1
14146e24a: 48 b8 20 00 00 00 04 00 00 00       	movabs	rax, 0x400000020
14146e254: 48 89 85 10 06 00 00        	mov	qword ptr [rbp + 0x610], rax
14146e25b: c6 85 18 06 00 00 03        	mov	byte ptr [rbp + 0x618], 0x3
14146e262: 48 8d 05 e7 5f 7f 01        	lea	rax, [rip + 0x17f5fe7]  # 0x142c64250
14146e269: 48 89 85 60 03 00 00        	mov	qword ptr [rbp + 0x360], rax
14146e270: 48 c7 85 68 03 00 00 02 00 00 00    	mov	qword ptr [rbp + 0x368], 0x2
14146e27b: 4c 89 b5 70 03 00 00        	mov	qword ptr [rbp + 0x370], r14
14146e282: 48 c7 85 78 03 00 00 02 00 00 00    	mov	qword ptr [rbp + 0x378], 0x2
14146e28d: 48 89 b5 80 03 00 00        	mov	qword ptr [rbp + 0x380], rsi
14146e294: 48 c7 85 88 03 00 00 02 00 00 00    	mov	qword ptr [rbp + 0x388], 0x2
14146e29f: 48 8d 8d 78 01 00 00        	lea	rcx, [rbp + 0x178]
14146e2a6: 48 8d b5 60 03 00 00        	lea	rsi, [rbp + 0x360]
14146e2ad: 48 89 f2                    	mov	rdx, rsi
14146e2b0: e8 db bc 90 ff              	call	0x140d79f90 <.text+0xd78f90>
14146e2b5: 0f 10 85 78 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x178]
14146e2bc: 0f 29 85 80 08 00 00        	movaps	xmmword ptr [rbp + 0x880], xmm0
14146e2c3: 48 8b 85 88 01 00 00        	mov	rax, qword ptr [rbp + 0x188]
14146e2ca: 48 89 85 90 08 00 00        	mov	qword ptr [rbp + 0x890], rax
14146e2d1: 48 8b 05 d0 e0 1b 02        	mov	rax, qword ptr [rip + 0x21be0d0] # 0x14362c3a8
14146e2d8: 48 8b 00                    	mov	rax, qword ptr [rax]
14146e2db: 48 85 c0                    	test	rax, rax
14146e2de: 0f 84 da 00 00 00           	je	0x14146e3be <.text+0x146d3be>
14146e2e4: 48 8d 85 80 08 00 00        	lea	rax, [rbp + 0x880]
14146e2eb: 48 89 85 60 03 00 00        	mov	qword ptr [rbp + 0x360], rax
14146e2f2: 48 8d 05 d7 d7 bc fe        	lea	rax, [rip - 0x1432829]  # 0x14003bad0 <.text+0x3aad0>
14146e2f9: 48 89 85 68 03 00 00        	mov	qword ptr [rbp + 0x368], rax
14146e300: 48 8d 0d 79 5f 7f 01        	lea	rcx, [rip + 0x17f5f79]  # 0x142c64280
14146e307: e8 74 fa 20 ff              	call	0x14067dd80 <.text+0x67cd80>
14146e30c: 0f 10 00                    	movups	xmm0, xmmword ptr [rax]
14146e30f: 8b 40 10                    	mov	eax, dword ptr [rax + 0x10]
14146e312: 48 c7 85 e0 05 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x5e0], 0x1
14146e31d: 48 8d 0d 24 5e 7f 01        	lea	rcx, [rip + 0x17f5e24]  # 0x142c64148
14146e324: 48 89 8d e8 05 00 00        	mov	qword ptr [rbp + 0x5e8], rcx
14146e32b: 48 c7 85 f0 05 00 00 26 00 00 00    	mov	qword ptr [rbp + 0x5f0], 0x26
14146e336: 48 8d 15 33 5f 7f 01        	lea	rdx, [rip + 0x17f5f33]  # 0x142c64270
14146e33d: 48 89 95 00 06 00 00        	mov	qword ptr [rbp + 0x600], rdx
14146e344: 48 c7 85 08 06 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x608], 0x1
14146e34f: 48 89 b5 10 06 00 00        	mov	qword ptr [rbp + 0x610], rsi
14146e356: 48 c7 85 18 06 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x618], 0x1
14146e361: 48 c7 85 20 06 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x620], 0x0
14146e36c: 48 c7 85 b0 05 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x5b0], 0x0
14146e377: 48 89 8d b8 05 00 00        	mov	qword ptr [rbp + 0x5b8], rcx
14146e37e: 48 c7 85 c0 05 00 00 26 00 00 00    	mov	qword ptr [rbp + 0x5c0], 0x26
14146e389: 48 c7 85 c8 05 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x5c8], 0x0
14146e394: 0f 11 85 d0 05 00 00        	movups	xmmword ptr [rbp + 0x5d0], xmm0
14146e39b: c7 85 f8 05 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x5f8], 0x1
14146e3a5: 89 85 fc 05 00 00           	mov	dword ptr [rbp + 0x5fc], eax
14146e3ab: 48 8d 8d 72 09 00 00        	lea	rcx, [rbp + 0x972]
14146e3b2: 48 8d 95 b0 05 00 00        	lea	rdx, [rbp + 0x5b0]
14146e3b9: e8 72 bf 8d ff              	call	0x140d4a330 <.text+0xd49330>
14146e3be: 48 8b 8d 88 08 00 00        	mov	rcx, qword ptr [rbp + 0x888]
14146e3c5: 48 8b 95 90 08 00 00        	mov	rdx, qword ptr [rbp + 0x890]
14146e3cc: e8 cf 2c 45 ff              	call	0x1408c10a0 <.text+0x8c00a0>
14146e3d1: 41 c6 44 24 48 02           	mov	byte ptr [r12 + 0x48], 0x2
14146e3d7: 8b 85 cc 08 00 00           	mov	eax, dword ptr [rbp + 0x8cc]
14146e3dd: 48 8b 8d 88 05 00 00        	mov	rcx, qword ptr [rbp + 0x588]
14146e3e4: 89 01                       	mov	dword ptr [rcx], eax
14146e3e6: 49 8b 44 24 10              	mov	rax, qword ptr [r12 + 0x10]
14146e3eb: 48 89 85 40 08 00 00        	mov	qword ptr [rbp + 0x840], rax
14146e3f2: 41 0f 10 04 24              	movups	xmm0, xmmword ptr [r12]
14146e3f7: 0f 29 85 30 08 00 00        	movaps	xmmword ptr [rbp + 0x830], xmm0
14146e3fe: 49 c7 04 24 00 00 00 00     	mov	qword ptr [r12], 0x0
14146e406: 49 c7 44 24 08 08 00 00 00  	mov	qword ptr [r12 + 0x8], 0x8
14146e40f: 49 c7 44 24 10 00 00 00 00  	mov	qword ptr [r12 + 0x10], 0x0
14146e418: 48 8b 85 40 05 00 00        	mov	rax, qword ptr [rbp + 0x540]
14146e41f: 8b 8d 64 08 00 00           	mov	ecx, dword ptr [rbp + 0x864]
14146e425: 89 08                       	mov	dword ptr [rax], ecx
14146e427: 48 8b 9d 38 05 00 00        	mov	rbx, qword ptr [rbp + 0x538]
14146e42e: 48 8b 73 30                 	mov	rsi, qword ptr [rbx + 0x30]
14146e432: 48 8b 7b 40                 	mov	rdi, qword ptr [rbx + 0x40]
14146e436: 48 3b 73 20                 	cmp	rsi, qword ptr [rbx + 0x20]
14146e43a: 75 17                       	jne	0x14146e453 <.text+0x146d453>
14146e43c: 48 8d 4b 20                 	lea	rcx, [rbx + 0x20]
14146e440: c6 85 6f 09 00 00 01        	mov	byte ptr [rbp + 0x96f], 0x1
14146e447: 48 8d 15 fa bc 7c 01        	lea	rdx, [rip + 0x17cbcfa]  # 0x142c3a148
14146e44e: e8 1d 44 0f ff              	call	0x140562870 <.text+0x561870>
14146e453: 48 8b 85 50 0a 00 00        	mov	rax, qword ptr [rbp + 0xa50]
14146e45a: 48 8b 8d 48 0a 00 00        	mov	rcx, qword ptr [rbp + 0xa48]
14146e461: 48 8b 53 28                 	mov	rdx, qword ptr [rbx + 0x28]
14146e465: 48 89 3c f2                 	mov	qword ptr [rdx + 8*rsi], rdi
14146e469: 48 ff c6                    	inc	rsi
14146e46c: 48 89 73 30                 	mov	qword ptr [rbx + 0x30], rsi
14146e470: 48 ff 43 40                 	inc	qword ptr [rbx + 0x40]
14146e474: 8b 50 1c                    	mov	edx, dword ptr [rax + 0x1c]
14146e477: 4c 8b 40 10                 	mov	r8, qword ptr [rax + 0x10]
14146e47b: 41 89 10                    	mov	dword ptr [r8], edx
14146e47e: 48 8b 00                    	mov	rax, qword ptr [rax]
14146e481: c6 00 02                    	mov	byte ptr [rax], 0x2
14146e484: 8b 41 1c                    	mov	eax, dword ptr [rcx + 0x1c]
14146e487: 48 8b 51 10                 	mov	rdx, qword ptr [rcx + 0x10]
14146e48b: 89 02                       	mov	dword ptr [rdx], eax
14146e48d: 48 8b 01                    	mov	rax, qword ptr [rcx]
14146e490: 48 ff 80 d8 01 00 00        	inc	qword ptr [rax + 0x1d8]
14146e497: 48 c7 85 80 03 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x380], 0x1
14146e4a2: 48 c7 85 88 03 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x388], 0x0
14146e4ad: 48 be 00 00 00 00 00 00 00 80       	movabs	rsi, -0x8000000000000000
14146e4b7: 48 89 b5 90 03 00 00        	mov	qword ptr [rbp + 0x390], rsi
14146e4be: c6 85 d0 03 00 00 00        	mov	byte ptr [rbp + 0x3d0], 0x0
14146e4c5: 48 89 b5 d8 03 00 00        	mov	qword ptr [rbp + 0x3d8], rsi
14146e4cc: 48 c7 85 60 03 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x360], 0x0
14146e4d7: 48 c7 85 68 03 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x368], 0x8
14146e4e2: 0f 57 f6                    	xorps	xmm6, xmm6
14146e4e5: 0f 11 b5 70 03 00 00        	movups	xmmword ptr [rbp + 0x370], xmm6
14146e4ec: c6 85 f8 03 00 00 00        	mov	byte ptr [rbp + 0x3f8], 0x0
14146e4f3: 48 8b 8d 30 08 00 00        	mov	rcx, qword ptr [rbp + 0x830]
14146e4fa: 48 8b 85 38 08 00 00        	mov	rax, qword ptr [rbp + 0x838]
14146e501: 48 8b bd 40 08 00 00        	mov	rdi, qword ptr [rbp + 0x840]
14146e508: 48 c1 e7 05                 	shl	rdi, 0x5
14146e50c: 48 01 c7                    	add	rdi, rax
14146e50f: 48 89 85 b0 01 00 00        	mov	qword ptr [rbp + 0x1b0], rax
14146e516: 48 89 85 b8 01 00 00        	mov	qword ptr [rbp + 0x1b8], rax
14146e51d: 48 89 8d c0 01 00 00        	mov	qword ptr [rbp + 0x1c0], rcx
14146e524: 48 89 bd c8 01 00 00        	mov	qword ptr [rbp + 0x1c8], rdi
14146e52b: 48 8d 9d c0 05 00 00        	lea	rbx, [rbp + 0x5c0]
14146e532: 49 be 03 00 00 00 00 00 00 80       	movabs	r14, -0x7ffffffffffffffd
14146e53c: 41 bf 01 00 00 00           	mov	r15d, 0x1
14146e542: eb 3e                       	jmp	0x14146e582 <.text+0x146d582>
14146e544: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
14146e550: 48 8b 85 68 03 00 00        	mov	rax, qword ptr [rbp + 0x368]
14146e557: 4c 89 e1                    	mov	rcx, r12
14146e55a: 48 c1 e1 05                 	shl	rcx, 0x5
14146e55e: 0f 10 85 b0 05 00 00        	movups	xmm0, xmmword ptr [rbp + 0x5b0]
14146e565: 0f 10 8d c0 05 00 00        	movups	xmm1, xmmword ptr [rbp + 0x5c0]
14146e56c: 0f 11 4c 08 10              	movups	xmmword ptr [rax + rcx + 0x10], xmm1
14146e571: 0f 11 04 08                 	movups	xmmword ptr [rax + rcx], xmm0
14146e575: 49 ff c4                    	inc	r12
14146e578: 4c 89 a5 70 03 00 00        	mov	qword ptr [rbp + 0x370], r12
14146e57f: 4c 89 e8                    	mov	rax, r13
14146e582: 48 39 f8                    	cmp	rax, rdi
14146e585: 0f 84 8f 01 00 00           	je	0x14146e71a <.text+0x146d71a>
14146e58b: 4c 8d 68 20                 	lea	r13, [rax + 0x20]
14146e58f: 4c 89 ad b8 01 00 00        	mov	qword ptr [rbp + 0x1b8], r13
14146e596: 48 8b 08                    	mov	rcx, qword ptr [rax]
14146e599: 4c 39 f1                    	cmp	rcx, r14
14146e59c: 0f 84 78 01 00 00           	je	0x14146e71a <.text+0x146d71a>
14146e5a2: 49 89 c8                    	mov	r8, rcx
14146e5a5: 49 31 f0                    	xor	r8, rsi
14146e5a8: 49 83 f8 03                 	cmp	r8, 0x3
14146e5ac: 4d 0f 43 c7                 	cmovae	r8, r15
14146e5b0: 4d 85 c0                    	test	r8, r8
14146e5b3: 74 5b                       	je	0x14146e610 <.text+0x146d610>
14146e5b5: 48 8b 50 08                 	mov	rdx, qword ptr [rax + 0x8]
14146e5b9: 49 83 f8 01                 	cmp	r8, 0x1
14146e5bd: 0f 85 1d 01 00 00           	jne	0x14146e6e0 <.text+0x146d6e0>
14146e5c3: 48 83 c0 10                 	add	rax, 0x10
14146e5c7: 0f 10 00                    	movups	xmm0, xmmword ptr [rax]
14146e5ca: 0f 29 85 60 01 00 00        	movaps	xmmword ptr [rbp + 0x160], xmm0
14146e5d1: 48 89 8d b0 05 00 00        	mov	qword ptr [rbp + 0x5b0], rcx
14146e5d8: 48 89 95 b8 05 00 00        	mov	qword ptr [rbp + 0x5b8], rdx
14146e5df: 0f 11 03                    	movups	xmmword ptr [rbx], xmm0
14146e5e2: 4c 8b a5 70 03 00 00        	mov	r12, qword ptr [rbp + 0x370]
14146e5e9: 4c 3b a5 60 03 00 00        	cmp	r12, qword ptr [rbp + 0x360]
14146e5f0: 0f 85 5a ff ff ff           	jne	0x14146e550 <.text+0x146d550>
14146e5f6: 48 8d 8d 60 03 00 00        	lea	rcx, [rbp + 0x360]
14146e5fd: 48 8d 15 04 5d 7f 01        	lea	rdx, [rip + 0x17f5d04]  # 0x142c64308
14146e604: e8 27 46 0f ff              	call	0x140562c30 <.text+0x561c30>
14146e609: e9 42 ff ff ff              	jmp	0x14146e550 <.text+0x146d550>
14146e60e: 66 90                       	nop
14146e610: 48 8b 05 91 dd 1b 02        	mov	rax, qword ptr [rip + 0x21bdd91] # 0x14362c3a8
14146e617: 48 8b 08                    	mov	rcx, qword ptr [rax]
14146e61a: 4c 89 e8                    	mov	rax, r13
14146e61d: 48 83 f9 02                 	cmp	rcx, 0x2
14146e621: 0f 82 5b ff ff ff           	jb	0x14146e582 <.text+0x146d582>
14146e627: 48 8d 0d c2 5c 7f 01        	lea	rcx, [rip + 0x17f5cc2]  # 0x142c642f0
14146e62e: e8 4d f7 20 ff              	call	0x14067dd80 <.text+0x67cd80>
14146e633: 0f 10 00                    	movups	xmm0, xmmword ptr [rax]
14146e636: 8b 40 10                    	mov	eax, dword ptr [rax + 0x10]
14146e639: 48 c7 85 e0 05 00 00 02 00 00 00    	mov	qword ptr [rbp + 0x5e0], 0x2
14146e644: 48 8d 0d fd 5a 7f 01        	lea	rcx, [rip + 0x17f5afd]  # 0x142c64148
14146e64b: 48 89 8d e8 05 00 00        	mov	qword ptr [rbp + 0x5e8], rcx
14146e652: 48 c7 85 f0 05 00 00 26 00 00 00    	mov	qword ptr [rbp + 0x5f0], 0x26
14146e65d: 48 8d 15 7c 5c 7f 01        	lea	rdx, [rip + 0x17f5c7c]  # 0x142c642e0
14146e664: 48 89 95 00 06 00 00        	mov	qword ptr [rbp + 0x600], rdx
14146e66b: 48 c7 85 08 06 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x608], 0x1
14146e676: 48 c7 85 10 06 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x610], 0x8
14146e681: 48 c7 85 b0 05 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x5b0], 0x0
14146e68c: 0f 11 73 58                 	movups	xmmword ptr [rbx + 0x58], xmm6
14146e690: 48 89 8d b8 05 00 00        	mov	qword ptr [rbp + 0x5b8], rcx
14146e697: 48 c7 85 c0 05 00 00 26 00 00 00    	mov	qword ptr [rbp + 0x5c0], 0x26
14146e6a2: 48 c7 85 c8 05 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x5c8], 0x0
14146e6ad: 0f 11 85 d0 05 00 00        	movups	xmmword ptr [rbp + 0x5d0], xmm0
14146e6b4: c7 85 f8 05 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x5f8], 0x1
14146e6be: 89 85 fc 05 00 00           	mov	dword ptr [rbp + 0x5fc], eax
14146e6c4: 48 8d 8d 72 09 00 00        	lea	rcx, [rbp + 0x972]
14146e6cb: 48 8d 95 b0 05 00 00        	lea	rdx, [rbp + 0x5b0]
14146e6d2: e8 59 bc 8d ff              	call	0x140d4a330 <.text+0xd49330>
14146e6d7: 4c 89 e8                    	mov	rax, r13
14146e6da: e9 a3 fe ff ff              	jmp	0x14146e582 <.text+0x146d582>
14146e6df: 90                          	nop
14146e6e0: 48 89 95 b8 05 00 00        	mov	qword ptr [rbp + 0x5b8], rdx
14146e6e7: 48 89 b5 b0 05 00 00        	mov	qword ptr [rbp + 0x5b0], rsi
14146e6ee: 4c 8b a5 70 03 00 00        	mov	r12, qword ptr [rbp + 0x370]
14146e6f5: 4c 3b a5 60 03 00 00        	cmp	r12, qword ptr [rbp + 0x360]
14146e6fc: 0f 85 4e fe ff ff           	jne	0x14146e550 <.text+0x146d550>
14146e702: 48 8d 8d 60 03 00 00        	lea	rcx, [rbp + 0x360]
14146e709: 48 8d 15 10 5c 7f 01        	lea	rdx, [rip + 0x17f5c10]  # 0x142c64320
14146e710: e8 1b 45 0f ff              	call	0x140562c30 <.text+0x561c30>
14146e715: e9 36 fe ff ff              	jmp	0x14146e550 <.text+0x146d550>
14146e71a: c6 85 6e 09 00 00 01        	mov	byte ptr [rbp + 0x96e], 0x1
14146e721: 48 8d 8d b0 01 00 00        	lea	rcx, [rbp + 0x1b0]
14146e728: e8 c3 cb 40 ff              	call	0x14087b2f0 <.text+0x87a2f0>
14146e72d: 48 8d bd b0 05 00 00        	lea	rdi, [rbp + 0x5b0]
14146e734: 48 8d 95 60 03 00 00        	lea	rdx, [rbp + 0x360]
14146e73b: 41 b8 a0 00 00 00           	mov	r8d, 0xa0
14146e741: 48 89 f9                    	mov	rcx, rdi
14146e744: e8 b7 54 4b 01              	call	0x142923c00 <.text+0x2922c00>
14146e749: c6 85 6e 09 00 00 00        	mov	byte ptr [rbp + 0x96e], 0x0
14146e750: 4c 8d 05 41 5b 7f 01        	lea	r8, [rip + 0x17f5b41]   # 0x142c64298
14146e757: 48 8b 8d 58 0a 00 00        	mov	rcx, qword ptr [rbp + 0xa58]
14146e75e: 48 89 fa                    	mov	rdx, rdi
14146e761: e8 2a 24 e7 ff              	call	0x1412e0b90 <.text+0x12dfb90>
14146e766: 48 8b 95 80 08 00 00        	mov	rdx, qword ptr [rbp + 0x880]
14146e76d: 48 85 d2                    	test	rdx, rdx
14146e770: 74 12                       	je	0x14146e784 <.text+0x146d784>
14146e772: 48 8b 8d 88 08 00 00        	mov	rcx, qword ptr [rbp + 0x888]
14146e779: 41 b8 01 00 00 00           	mov	r8d, 0x1
14146e77f: e8 9c 54 1a ff              	call	0x140613c20 <.text+0x612c20>
14146e784: 48 8d 8d 68 08 00 00        	lea	rcx, [rbp + 0x868]
14146e78b: e8 a0 e4 8d ff              	call	0x140d4cc30 <.text+0xd4bc30>
14146e790: 48 8b 55 b0                 	mov	rdx, qword ptr [rbp - 0x50]
14146e794: 48 85 d2                    	test	rdx, rdx
14146e797: 0f 84 88 0e 00 00           	je	0x14146f625 <.text+0x146e625>
14146e79d: 48 8b 4d b8                 	mov	rcx, qword ptr [rbp - 0x48]
14146e7a1: e9 74 0e 00 00              	jmp	0x14146f61a <.text+0x146e61a>
14146e7a6: 45 31 c9                    	xor	r9d, r9d
14146e7a9: 48 8b 8d 80 05 00 00        	mov	rcx, qword ptr [rbp + 0x580]
14146e7b0: 48 8b 81 c0 00 00 00        	mov	rax, qword ptr [rcx + 0xc0]
14146e7b7: 49 39 c1                    	cmp	r9, rax
14146e7ba: 49 0f 47 c1                 	cmova	rax, r9
14146e7be: 48 89 81 c0 00 00 00        	mov	qword ptr [rcx + 0xc0], rax
14146e7c5: 48 8b 8d 18 09 00 00        	mov	rcx, qword ptr [rbp + 0x918]
14146e7cc: 8b 41 6c                    	mov	eax, dword ptr [rcx + 0x6c]
14146e7cf: 85 c0                       	test	eax, eax
14146e7d1: ba 00 00 00 00              	mov	edx, 0x0
14146e7d6: 0f 4f d0                    	cmovg	edx, eax
14146e7d9: 89 51 6c                    	mov	dword ptr [rcx + 0x6c], edx
14146e7dc: 4c 8b a5 50 09 00 00        	mov	r12, qword ptr [rbp + 0x950]
14146e7e3: 4c 89 a5 40 09 00 00        	mov	qword ptr [rbp + 0x940], r12
14146e7ea: 48 8b b5 60 09 00 00        	mov	rsi, qword ptr [rbp + 0x960]
14146e7f1: 48 89 b5 20 09 00 00        	mov	qword ptr [rbp + 0x920], rsi
14146e7f8: c6 85 77 09 00 00 00        	mov	byte ptr [rbp + 0x977], 0x0
14146e7ff: e8 dc 4d 8f ff              	call	0x140d635e0 <.text+0xd625e0>
14146e804: 48 89 85 90 05 00 00        	mov	qword ptr [rbp + 0x590], rax
14146e80b: 89 95 98 05 00 00           	mov	dword ptr [rbp + 0x598], edx
14146e811: 48 8b 85 48 09 00 00        	mov	rax, qword ptr [rbp + 0x948]
14146e818: 48 8b 78 48                 	mov	rdi, qword ptr [rax + 0x48]
14146e81c: 8b 98 c0 00 00 00           	mov	ebx, dword ptr [rax + 0xc0]
14146e822: 4c 89 a5 40 09 00 00        	mov	qword ptr [rbp + 0x940], r12
14146e829: 48 89 b5 20 09 00 00        	mov	qword ptr [rbp + 0x920], rsi
14146e830: c6 85 77 09 00 00 00        	mov	byte ptr [rbp + 0x977], 0x0
14146e837: 4c 8d ad 60 03 00 00        	lea	r13, [rbp + 0x360]
14146e83e: 4c 89 e9                    	mov	rcx, r13
14146e841: 48 8b 95 a8 08 00 00        	mov	rdx, qword ptr [rbp + 0x8a8]
14146e848: e8 c3 2e 4a ff              	call	0x140911710 <.text+0x910710>
14146e84d: 48 8b 85 58 05 00 00        	mov	rax, qword ptr [rbp + 0x558]
14146e854: 66 44 8b 70 28              	mov	r14w, word ptr [rax + 0x28]
14146e859: 44 8b 38                    	mov	r15d, dword ptr [rax]
14146e85c: 44 0f 10 40 04              	movups	xmm8, xmmword ptr [rax + 0x4]
14146e861: 44 0f 10 48 14              	movups	xmm9, xmmword ptr [rax + 0x14]
14146e866: f3 44 0f 10 50 24           	movss	xmm10, dword ptr [rax + 0x24]
14146e86c: 48 8b 85 18 09 00 00        	mov	rax, qword ptr [rbp + 0x918]
14146e873: f2 0f 10 78 68              	movsd	xmm7, qword ptr [rax + 0x68]
14146e878: 48 8b 50 08                 	mov	rdx, qword ptr [rax + 0x8]
14146e87c: 48 8b 40 10                 	mov	rax, qword ptr [rax + 0x10]
14146e880: 48 8d 04 80                 	lea	rax, [rax + 4*rax]
14146e884: 4c 8d 04 c2                 	lea	r8, [rdx + 8*rax]
14146e888: 4c 89 a5 40 09 00 00        	mov	qword ptr [rbp + 0x940], r12
14146e88f: 48 89 b5 20 09 00 00        	mov	qword ptr [rbp + 0x920], rsi
14146e896: c6 85 77 09 00 00 00        	mov	byte ptr [rbp + 0x977], 0x0
14146e89d: 4c 8d 0d a4 55 7f 01        	lea	r9, [rip + 0x17f55a4]   # 0x142c63e48
14146e8a4: 48 8d 4d b0                 	lea	rcx, [rbp - 0x50]
14146e8a8: e8 13 f6 e9 ff              	call	0x14130dec0 <.text+0x130cec0>
14146e8ad: 48 8b 85 18 09 00 00        	mov	rax, qword ptr [rbp + 0x918]
14146e8b4: 48 8b 50 20                 	mov	rdx, qword ptr [rax + 0x20]
14146e8b8: 4c 8b 40 28                 	mov	r8, qword ptr [rax + 0x28]
14146e8bc: 49 c1 e0 04                 	shl	r8, 0x4
14146e8c0: 49 01 d0                    	add	r8, rdx
14146e8c3: 4c 8d 0d 7e 55 7f 01        	lea	r9, [rip + 0x17f557e]   # 0x142c63e48
14146e8ca: 4c 8d a5 b0 01 00 00        	lea	r12, [rbp + 0x1b0]
14146e8d1: 4c 89 e1                    	mov	rcx, r12
14146e8d4: e8 e7 6d e9 ff              	call	0x1413056c0 <.text+0x13046c0>
14146e8d9: 48 8b 85 50 09 00 00        	mov	rax, qword ptr [rbp + 0x950]
14146e8e0: 48 f7 d8                    	neg	rax
14146e8e3: 0f 80 35 05 00 00           	jo	0x14146ee1e <.text+0x146de1e>
14146e8e9: 45 31 ed                    	xor	r13d, r13d
14146e8ec: b9 28 00 00 00              	mov	ecx, 0x28
14146e8f1: 48 8b b5 d8 08 00 00        	mov	rsi, qword ptr [rbp + 0x8d8]
14146e8f8: 48 89 f0                    	mov	rax, rsi
14146e8fb: 48 f7 e1                    	mul	rcx
14146e8fe: 49 89 c4                    	mov	r12, rax
14146e901: 0f 80 c3 10 00 00           	jo	0x14146f9ca <.text+0x146e9ca>
14146e907: 48 b8 fc ff ff ff ff ff ff 7f       	movabs	rax, 0x7ffffffffffffffc
14146e911: 49 39 c4                    	cmp	r12, rax
14146e914: 0f 87 b0 10 00 00           	ja	0x14146f9ca <.text+0x146e9ca>
14146e91a: 4d 85 e4                    	test	r12, r12
14146e91d: 0f 84 f2 01 00 00           	je	0x14146eb15 <.text+0x146db15>
14146e923: 0f b6 05 df 62 1c 02        	movzx	eax, byte ptr [rip + 0x21c62df] # 0x143634c09
14146e92a: 41 bd 04 00 00 00           	mov	r13d, 0x4
14146e930: ba 04 00 00 00              	mov	edx, 0x4
14146e935: 4c 89 e1                    	mov	rcx, r12
14146e938: e8 d3 52 1a ff              	call	0x140613c10 <.text+0x612c10>
14146e93d: 48 85 c0                    	test	rax, rax
14146e940: 0f 84 84 10 00 00           	je	0x14146f9ca <.text+0x146e9ca>
14146e946: 48 85 f6                    	test	rsi, rsi
14146e949: 4c 8d ad 60 03 00 00        	lea	r13, [rbp + 0x360]
14146e950: 0f 84 b6 04 00 00           	je	0x14146ee0c <.text+0x146de0c>
14146e956: 4c 8b 8d 60 09 00 00        	mov	r9, qword ptr [rbp + 0x960]
14146e95d: 49 8d 49 04                 	lea	rcx, [r9 + 0x4]
14146e961: 31 d2                       	xor	edx, edx
14146e963: 49 89 f0                    	mov	r8, rsi
14146e966: eb 78                       	jmp	0x14146e9e0 <.text+0x146d9e0>
14146e968: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
14146e970: 49 ff c8                    	dec	r8
14146e973: 44 8b 9d b8 05 00 00        	mov	r11d, dword ptr [rbp + 0x5b8]
14146e97a: 44 89 9d 88 08 00 00        	mov	dword ptr [rbp + 0x888], r11d
14146e981: 4c 8b 9d b0 05 00 00        	mov	r11, qword ptr [rbp + 0x5b0]
14146e988: 4c 89 9d 80 08 00 00        	mov	qword ptr [rbp + 0x880], r11
14146e98f: 44 89 14 10                 	mov	dword ptr [rax + rdx], r10d
14146e993: 44 8b 95 88 08 00 00        	mov	r10d, dword ptr [rbp + 0x888]
14146e99a: 44 89 54 10 0c              	mov	dword ptr [rax + rdx + 0xc], r10d
14146e99f: 4c 8b 95 80 08 00 00        	mov	r10, qword ptr [rbp + 0x880]
14146e9a6: 4c 89 54 10 04              	mov	qword ptr [rax + rdx + 0x4], r10
14146e9ab: 4d 8b 51 10                 	mov	r10, qword ptr [r9 + 0x10]
14146e9af: 4c 89 54 10 10              	mov	qword ptr [rax + rdx + 0x10], r10
14146e9b4: 45 8b 51 18                 	mov	r10d, dword ptr [r9 + 0x18]
14146e9b8: 44 89 54 10 18              	mov	dword ptr [rax + rdx + 0x18], r10d
14146e9bd: 4d 8b 51 1c                 	mov	r10, qword ptr [r9 + 0x1c]
14146e9c1: 4c 89 54 10 1c              	mov	qword ptr [rax + rdx + 0x1c], r10
14146e9c6: 45 8b 51 24                 	mov	r10d, dword ptr [r9 + 0x24]
14146e9ca: 49 83 c1 28                 	add	r9, 0x28
14146e9ce: 44 89 54 10 24              	mov	dword ptr [rax + rdx + 0x24], r10d
14146e9d3: 48 83 c2 28                 	add	rdx, 0x28
14146e9d7: 4d 85 c0                    	test	r8, r8
14146e9da: 0f 84 29 01 00 00           	je	0x14146eb09 <.text+0x146db09>
14146e9e0: 49 39 d4                    	cmp	r12, rdx
14146e9e3: 0f 84 20 01 00 00           	je	0x14146eb09 <.text+0x146db09>
14146e9e9: 45 31 d2                    	xor	r10d, r10d
14146e9ec: 41 83 39 01                 	cmp	dword ptr [r9], 0x1
14146e9f0: 0f 85 7a ff ff ff           	jne	0x14146e970 <.text+0x146d970>
14146e9f6: 44 8b 54 11 08              	mov	r10d, dword ptr [rcx + rdx + 0x8]
14146e9fb: 44 89 95 b8 05 00 00        	mov	dword ptr [rbp + 0x5b8], r10d
14146ea02: 4c 8b 14 11                 	mov	r10, qword ptr [rcx + rdx]
14146ea06: 4c 89 95 b0 05 00 00        	mov	qword ptr [rbp + 0x5b0], r10
14146ea0d: 41 ba 01 00 00 00           	mov	r10d, 0x1
14146ea13: e9 58 ff ff ff              	jmp	0x14146e970 <.text+0x146d970>
14146ea18: f6 85 bc 01 00 00 01        	test	byte ptr [rbp + 0x1bc], 0x1
14146ea1f: 74 09                       	je	0x14146ea2a <.text+0x146da2a>
14146ea21: 40 84 f6                    	test	sil, sil
14146ea24: 0f 84 35 0c 00 00           	je	0x14146f65f <.text+0x146e65f>
14146ea2a: 48 8b bd b0 01 00 00        	mov	rdi, qword ptr [rbp + 0x1b0]
14146ea31: 8b 85 b8 01 00 00           	mov	eax, dword ptr [rbp + 0x1b8]
14146ea37: 89 85 f0 08 00 00           	mov	dword ptr [rbp + 0x8f0], eax
14146ea3d: 8b 85 bb 01 00 00           	mov	eax, dword ptr [rbp + 0x1bb]
14146ea43: 89 85 f3 08 00 00           	mov	dword ptr [rbp + 0x8f3], eax
14146ea49: 48 89 bd b0 05 00 00        	mov	qword ptr [rbp + 0x5b0], rdi
14146ea50: 8b 85 f0 08 00 00           	mov	eax, dword ptr [rbp + 0x8f0]
14146ea56: 8b 8d f3 08 00 00           	mov	ecx, dword ptr [rbp + 0x8f3]
14146ea5c: 89 85 b8 05 00 00           	mov	dword ptr [rbp + 0x5b8], eax
14146ea62: 89 8d bb 05 00 00           	mov	dword ptr [rbp + 0x5bb], ecx
14146ea68: 88 9d bf 05 00 00           	mov	byte ptr [rbp + 0x5bf], bl
14146ea6e: 48 8d 4d b0                 	lea	rcx, [rbp - 0x50]
14146ea72: e8 59 84 90 ff              	call	0x140d76ed0 <.text+0xd75ed0>
14146ea77: 48 bb 00 00 00 00 00 00 00 80       	movabs	rbx, -0x8000000000000000
14146ea81: 41 88 f6                    	mov	r14b, sil
14146ea84: 48 89 c1                    	mov	rcx, rax
14146ea87: e8 44 32 8f ff              	call	0x140d61cd0 <.text+0xd60cd0>
14146ea8c: 48 85 c0                    	test	rax, rax
14146ea8f: 0f 84 cf 0f 00 00           	je	0x14146fa64 <.text+0x146ea64>
14146ea95: 49 89 d0                    	mov	r8, rdx
14146ea98: 48 c7 44 24 20 09 00 00 00  	mov	qword ptr [rsp + 0x20], 0x9
14146eaa1: 4c 8d 0d 80 57 7f 01        	lea	r9, [rip + 0x17f5780]   # 0x142c64228
14146eaa8: 48 8d 8d 90 01 00 00        	lea	rcx, [rbp + 0x190]
14146eaaf: 48 89 c2                    	mov	rdx, rax
14146eab2: e8 59 3d 8f ff              	call	0x140d62810 <.text+0xd61810>
14146eab7: 48 8b 85 b8 05 00 00        	mov	rax, qword ptr [rbp + 0x5b8]
14146eabe: 48 89 85 10 09 00 00        	mov	qword ptr [rbp + 0x910], rax
14146eac5: 48 8b b5 90 01 00 00        	mov	rsi, qword ptr [rbp + 0x190]
14146eacc: 0f 10 85 98 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x198]
14146ead3: 0f 29 85 90 05 00 00        	movaps	xmmword ptr [rbp + 0x590], xmm0
14146eada: 48 8b 85 a8 01 00 00        	mov	rax, qword ptr [rbp + 0x1a8]
14146eae1: 48 89 85 a0 05 00 00        	mov	qword ptr [rbp + 0x5a0], rax
14146eae8: 48 8b 55 b0                 	mov	rdx, qword ptr [rbp - 0x50]
14146eaec: 48 85 d2                    	test	rdx, rdx
14146eaef: 0f 84 db f0 ff ff           	je	0x14146dbd0 <.text+0x146cbd0>
14146eaf5: 48 8b 4d b8                 	mov	rcx, qword ptr [rbp - 0x48]
14146eaf9: 41 b8 01 00 00 00           	mov	r8d, 0x1
14146eaff: e8 1c 51 1a ff              	call	0x140613c20 <.text+0x612c20>
14146eb04: e9 c7 f0 ff ff              	jmp	0x14146dbd0 <.text+0x146cbd0>
14146eb09: 48 89 b5 e8 08 00 00        	mov	qword ptr [rbp + 0x8e8], rsi
14146eb10: e9 02 03 00 00              	jmp	0x14146ee17 <.text+0x146de17>
14146eb15: b8 04 00 00 00              	mov	eax, 0x4
14146eb1a: 48 c7 85 e8 08 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x8e8], 0x0
14146eb25: 4c 8d ad 60 03 00 00        	lea	r13, [rbp + 0x360]
14146eb2c: e9 e6 02 00 00              	jmp	0x14146ee17 <.text+0x146de17>
14146eb31: 48 89 b5 b0 01 00 00        	mov	qword ptr [rbp + 0x1b0], rsi
14146eb38: 48 8d 8d b8 01 00 00        	lea	rcx, [rbp + 0x1b8]
14146eb3f: 48 8d 55 b0                 	lea	rdx, [rbp - 0x50]
14146eb43: 41 b8 a8 01 00 00           	mov	r8d, 0x1a8
14146eb49: e8 b2 50 4b 01              	call	0x142923c00 <.text+0x2922c00>
14146eb4e: 48 0f ba a5 10 09 00 00 30  	bt	qword ptr [rbp + 0x910], 0x30
14146eb57: 0f 83 3f 01 00 00           	jae	0x14146ec9c <.text+0x146dc9c>
14146eb5d: 48 8b 05 44 d8 1b 02        	mov	rax, qword ptr [rip + 0x21bd844] # 0x14362c3a8
14146eb64: 48 8b 00                    	mov	rax, qword ptr [rax]
14146eb67: 48 83 f8 03                 	cmp	rax, 0x3
14146eb6b: 0f 82 c4 00 00 00           	jb	0x14146ec35 <.text+0x146dc35>
14146eb71: c6 85 73 09 00 00 01        	mov	byte ptr [rbp + 0x973], 0x1
14146eb78: 48 8d 0d 21 5f 7f 01        	lea	rcx, [rip + 0x17f5f21]  # 0x142c64aa0
14146eb7f: e8 fc f1 20 ff              	call	0x14067dd80 <.text+0x67cd80>
14146eb84: 0f 10 00                    	movups	xmm0, xmmword ptr [rax]
14146eb87: 8b 40 10                    	mov	eax, dword ptr [rax + 0x10]
14146eb8a: 48 c7 85 e0 05 00 00 03 00 00 00    	mov	qword ptr [rbp + 0x5e0], 0x3
14146eb95: 48 8d 0d ac 55 7f 01        	lea	rcx, [rip + 0x17f55ac]  # 0x142c64148
14146eb9c: 48 89 8d e8 05 00 00        	mov	qword ptr [rbp + 0x5e8], rcx
14146eba3: 48 c7 85 f0 05 00 00 26 00 00 00    	mov	qword ptr [rbp + 0x5f0], 0x26
14146ebae: 48 8d 15 db 5e 7f 01        	lea	rdx, [rip + 0x17f5edb]  # 0x142c64a90
14146ebb5: 48 89 95 00 06 00 00        	mov	qword ptr [rbp + 0x600], rdx
14146ebbc: 48 c7 85 08 06 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x608], 0x1
14146ebc7: 48 c7 85 10 06 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x610], 0x8
14146ebd2: 48 c7 85 b0 05 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x5b0], 0x0
14146ebdd: 0f 57 c9                    	xorps	xmm1, xmm1
14146ebe0: 0f 11 8d 18 06 00 00        	movups	xmmword ptr [rbp + 0x618], xmm1
14146ebe7: 48 89 8d b8 05 00 00        	mov	qword ptr [rbp + 0x5b8], rcx
14146ebee: 48 c7 85 c0 05 00 00 26 00 00 00    	mov	qword ptr [rbp + 0x5c0], 0x26
14146ebf9: 48 c7 85 c8 05 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x5c8], 0x0
14146ec04: 0f 11 85 d0 05 00 00        	movups	xmmword ptr [rbp + 0x5d0], xmm0
14146ec0b: c7 85 f8 05 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x5f8], 0x1
14146ec15: 89 85 fc 05 00 00           	mov	dword ptr [rbp + 0x5fc], eax
14146ec1b: c6 85 73 09 00 00 01        	mov	byte ptr [rbp + 0x973], 0x1
14146ec22: 48 8d 8d 72 09 00 00        	lea	rcx, [rbp + 0x972]
14146ec29: 48 8d 95 b0 05 00 00        	lea	rdx, [rbp + 0x5b0]
14146ec30: e8 fb b6 8d ff              	call	0x140d4a330 <.text+0xd49330>
14146ec35: 48 8b 8d 18 02 00 00        	mov	rcx, qword ptr [rbp + 0x218]
14146ec3c: 48 85 c9                    	test	rcx, rcx
14146ec3f: 74 5b                       	je	0x14146ec9c <.text+0x146dc9c>
14146ec41: 48 8b 85 10 02 00 00        	mov	rax, qword ptr [rbp + 0x210]
14146ec48: 48 8d 0c 89                 	lea	rcx, [rcx + 4*rcx]
14146ec4c: 48 8d 0c c8                 	lea	rcx, [rax + 8*rcx]
14146ec50: eb 09                       	jmp	0x14146ec5b <.text+0x146dc5b>
14146ec52: 48 83 c0 28                 	add	rax, 0x28
14146ec56: 48 39 c8                    	cmp	rax, rcx
14146ec59: 74 41                       	je	0x14146ec9c <.text+0x146dc9c>
14146ec5b: 4c 8b 40 20                 	mov	r8, qword ptr [rax + 0x20]
14146ec5f: 4d 85 c0                    	test	r8, r8
14146ec62: 74 ee                       	je	0x14146ec52 <.text+0x146dc52>
14146ec64: 48 8b 50 18                 	mov	rdx, qword ptr [rax + 0x18]
14146ec68: 4d 6b c0 58                 	imul	r8, r8, 0x58
14146ec6c: 45 31 c9                    	xor	r9d, r9d
14146ec6f: eb 18                       	jmp	0x14146ec89 <.text+0x146dc89>
14146ec71: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
14146ec80: 49 83 c1 58                 	add	r9, 0x58
14146ec84: 4d 39 c8                    	cmp	r8, r9
14146ec87: 74 c9                       	je	0x14146ec52 <.text+0x146dc52>
14146ec89: 42 80 7c 0a 52 3a           	cmp	byte ptr [rdx + r9 + 0x52], 0x3a
14146ec8f: 75 ef                       	jne	0x14146ec80 <.text+0x146dc80>
14146ec91: 42 c7 44 0a 30 e7 03 00 00  	mov	dword ptr [rdx + r9 + 0x30], 0x3e7
14146ec9a: eb e4                       	jmp	0x14146ec80 <.text+0x146dc80>
14146ec9c: 48 0f ba a5 10 09 00 00 28  	bt	qword ptr [rbp + 0x910], 0x28
14146eca5: 0f 83 f3 0a 00 00           	jae	0x14146f79e <.text+0x146e79e>
14146ecab: 48 8b 05 f6 d6 1b 02        	mov	rax, qword ptr [rip + 0x21bd6f6] # 0x14362c3a8
14146ecb2: 48 8b 00                    	mov	rax, qword ptr [rax]
14146ecb5: 48 83 f8 03                 	cmp	rax, 0x3
14146ecb9: 0f 82 c4 00 00 00           	jb	0x14146ed83 <.text+0x146dd83>
14146ecbf: c6 85 73 09 00 00 01        	mov	byte ptr [rbp + 0x973], 0x1
14146ecc6: 48 8d 0d 33 5e 7f 01        	lea	rcx, [rip + 0x17f5e33]  # 0x142c64b00
14146eccd: e8 ae f0 20 ff              	call	0x14067dd80 <.text+0x67cd80>
14146ecd2: 0f 10 00                    	movups	xmm0, xmmword ptr [rax]
14146ecd5: 8b 40 10                    	mov	eax, dword ptr [rax + 0x10]
14146ecd8: 48 c7 85 e0 05 00 00 03 00 00 00    	mov	qword ptr [rbp + 0x5e0], 0x3
14146ece3: 48 8d 0d 5e 54 7f 01        	lea	rcx, [rip + 0x17f545e]  # 0x142c64148
14146ecea: 48 89 8d e8 05 00 00        	mov	qword ptr [rbp + 0x5e8], rcx
14146ecf1: 48 c7 85 f0 05 00 00 26 00 00 00    	mov	qword ptr [rbp + 0x5f0], 0x26
14146ecfc: 48 8d 15 ed 5d 7f 01        	lea	rdx, [rip + 0x17f5ded]  # 0x142c64af0
14146ed03: 48 89 95 00 06 00 00        	mov	qword ptr [rbp + 0x600], rdx
14146ed0a: 48 c7 85 08 06 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x608], 0x1
14146ed15: 48 c7 85 10 06 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x610], 0x8
14146ed20: 48 c7 85 b0 05 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x5b0], 0x0
14146ed2b: 0f 57 c9                    	xorps	xmm1, xmm1
14146ed2e: 0f 11 8d 18 06 00 00        	movups	xmmword ptr [rbp + 0x618], xmm1
14146ed35: 48 89 8d b8 05 00 00        	mov	qword ptr [rbp + 0x5b8], rcx
14146ed3c: 48 c7 85 c0 05 00 00 26 00 00 00    	mov	qword ptr [rbp + 0x5c0], 0x26
14146ed47: 48 c7 85 c8 05 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x5c8], 0x0
14146ed52: 0f 11 85 d0 05 00 00        	movups	xmmword ptr [rbp + 0x5d0], xmm0
14146ed59: c7 85 f8 05 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x5f8], 0x1
14146ed63: 89 85 fc 05 00 00           	mov	dword ptr [rbp + 0x5fc], eax
14146ed69: c6 85 73 09 00 00 01        	mov	byte ptr [rbp + 0x973], 0x1
14146ed70: 48 8d 8d 72 09 00 00        	lea	rcx, [rbp + 0x972]
14146ed77: 48 8d 95 b0 05 00 00        	lea	rdx, [rbp + 0x5b0]
14146ed7e: e8 ad b5 8d ff              	call	0x140d4a330 <.text+0xd49330>
14146ed83: 48 8b 85 48 02 00 00        	mov	rax, qword ptr [rbp + 0x248]
14146ed8a: 48 85 c0                    	test	rax, rax
14146ed8d: 0f 84 45 09 00 00           	je	0x14146f6d8 <.text+0x146e6d8>
14146ed93: 48 8b 8d 40 02 00 00        	mov	rcx, qword ptr [rbp + 0x240]
14146ed9a: 48 c1 e0 04                 	shl	rax, 0x4
14146ed9e: 31 d2                       	xor	edx, edx
14146eda0: 4c 8d 05 01 5f 7f 01        	lea	r8, [rip + 0x17f5f01]   # 0x142c64ca8
14146eda7: eb 15                       	jmp	0x14146edbe <.text+0x146ddbe>
14146eda9: 45 89 d1                    	mov	r9d, r10d
14146edac: 44 89 4c 11 08              	mov	dword ptr [rcx + rdx + 0x8], r9d
14146edb1: 48 83 c2 10                 	add	rdx, 0x10
14146edb5: 48 39 d0                    	cmp	rax, rdx
14146edb8: 0f 84 1a 09 00 00           	je	0x14146f6d8 <.text+0x146e6d8>
14146edbe: 44 8b 54 11 08              	mov	r10d, dword ptr [rcx + rdx + 0x8]
14146edc3: 49 83 fa 08                 	cmp	r10, 0x8
14146edc7: 77 12                       	ja	0x14146eddb <.text+0x146dddb>
14146edc9: 4f 63 0c 90                 	movsxd	r9, dword ptr [r8 + 4*r10]
14146edcd: 4d 01 c1                    	add	r9, r8
14146edd0: 41 ff e1                    	jmp	r9
14146edd3: 41 b9 02 00 00 00           	mov	r9d, 0x2
14146edd9: eb d1                       	jmp	0x14146edac <.text+0x146ddac>
14146eddb: 41 b9 01 00 00 00           	mov	r9d, 0x1
14146ede1: 41 81 fa e7 03 00 00        	cmp	r10d, 0x3e7
14146ede8: 74 bf                       	je	0x14146eda9 <.text+0x146dda9>
14146edea: eb c0                       	jmp	0x14146edac <.text+0x146ddac>
14146edec: 41 b9 03 00 00 00           	mov	r9d, 0x3
14146edf2: eb b8                       	jmp	0x14146edac <.text+0x146ddac>
14146edf4: 41 b9 05 00 00 00           	mov	r9d, 0x5
14146edfa: eb b0                       	jmp	0x14146edac <.text+0x146ddac>
14146edfc: 41 b9 08 00 00 00           	mov	r9d, 0x8
14146ee02: eb a8                       	jmp	0x14146edac <.text+0x146ddac>
14146ee04: 41 b9 04 00 00 00           	mov	r9d, 0x4
14146ee0a: eb a0                       	jmp	0x14146edac <.text+0x146ddac>
14146ee0c: 48 c7 85 e8 08 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x8e8], 0x0
14146ee17: 4c 8d a5 b0 01 00 00        	lea	r12, [rbp + 0x1b0]
14146ee1e: 48 8b b5 68 05 00 00        	mov	rsi, qword ptr [rbp + 0x568]
14146ee25: 0f b6 4e 42                 	movzx	ecx, byte ptr [rsi + 0x42]
14146ee29: 0f b7 56 40                 	movzx	edx, word ptr [rsi + 0x40]
14146ee2d: 44 8b 46 38                 	mov	r8d, dword ptr [rsi + 0x38]
14146ee31: f3 0f 10 46 3c              	movss	xmm0, dword ptr [rsi + 0x3c]
14146ee36: 44 0f b6 4e 14              	movzx	r9d, byte ptr [rsi + 0x14]
14146ee3b: 44 8b 56 10                 	mov	r10d, dword ptr [rsi + 0x10]
14146ee3f: 48 89 bd f8 05 00 00        	mov	qword ptr [rbp + 0x5f8], rdi
14146ee46: 89 9d 70 06 00 00           	mov	dword ptr [rbp + 0x670], ebx
14146ee4c: 0f 10 8d 60 03 00 00        	movups	xmm1, xmmword ptr [rbp + 0x360]
14146ee53: 0f 10 95 70 03 00 00        	movups	xmm2, xmmword ptr [rbp + 0x370]
14146ee5a: 0f 11 8d 74 06 00 00        	movups	xmmword ptr [rbp + 0x674], xmm1
14146ee61: 0f 11 95 84 06 00 00        	movups	xmmword ptr [rbp + 0x684], xmm2
14146ee68: 44 8b 9d 80 03 00 00        	mov	r11d, dword ptr [rbp + 0x380]
14146ee6f: 44 89 9d 94 06 00 00        	mov	dword ptr [rbp + 0x694], r11d
14146ee76: 44 89 bd 44 06 00 00        	mov	dword ptr [rbp + 0x644], r15d
14146ee7d: 44 0f 11 85 48 06 00 00     	movups	xmmword ptr [rbp + 0x648], xmm8
14146ee85: 44 0f 11 8d 58 06 00 00     	movups	xmmword ptr [rbp + 0x658], xmm9
14146ee8d: f3 44 0f 11 95 68 06 00 00  	movss	dword ptr [rbp + 0x668], xmm10
14146ee96: 66 44 89 b5 6c 06 00 00     	mov	word ptr [rbp + 0x66c], r14w
14146ee9e: 0f 13 bd 98 06 00 00        	movlps	qword ptr [rbp + 0x698], xmm7
14146eea5: 0f 10 4d b0                 	movups	xmm1, xmmword ptr [rbp - 0x50]
14146eea9: 0f 29 8d b0 05 00 00        	movaps	xmmword ptr [rbp + 0x5b0], xmm1
14146eeb0: 4c 8b 5d c0                 	mov	r11, qword ptr [rbp - 0x40]
14146eeb4: 4c 89 9d c0 05 00 00        	mov	qword ptr [rbp + 0x5c0], r11
14146eebb: 4c 8b 9d c0 01 00 00        	mov	r11, qword ptr [rbp + 0x1c0]
14146eec2: 4c 89 9d d8 05 00 00        	mov	qword ptr [rbp + 0x5d8], r11
14146eec9: 0f 10 8d b0 01 00 00        	movups	xmm1, xmmword ptr [rbp + 0x1b0]
14146eed0: 0f 11 8d c8 05 00 00        	movups	xmmword ptr [rbp + 0x5c8], xmm1
14146eed7: 4c 8b 9d e8 08 00 00        	mov	r11, qword ptr [rbp + 0x8e8]
14146eede: 4c 89 9d e0 05 00 00        	mov	qword ptr [rbp + 0x5e0], r11
14146eee5: 48 89 85 e8 05 00 00        	mov	qword ptr [rbp + 0x5e8], rax
14146eeec: 48 8b 85 d8 08 00 00        	mov	rax, qword ptr [rbp + 0x8d8]
14146eef3: 48 89 85 f0 05 00 00        	mov	qword ptr [rbp + 0x5f0], rax
14146eefa: 0f 10 0e                    	movups	xmm1, xmmword ptr [rsi]
14146eefd: 0f 29 8d 00 06 00 00        	movaps	xmmword ptr [rbp + 0x600], xmm1
14146ef04: 44 89 95 10 06 00 00        	mov	dword ptr [rbp + 0x610], r10d
14146ef0b: 44 88 8d 14 06 00 00        	mov	byte ptr [rbp + 0x614], r9b
14146ef12: 0f 10 4e 18                 	movups	xmm1, xmmword ptr [rsi + 0x18]
14146ef16: 0f 11 8d 18 06 00 00        	movups	xmmword ptr [rbp + 0x618], xmm1
14146ef1d: 0f 10 4e 28                 	movups	xmm1, xmmword ptr [rsi + 0x28]
14146ef21: 0f 11 8d 28 06 00 00        	movups	xmmword ptr [rbp + 0x628], xmm1
14146ef28: 44 89 85 38 06 00 00        	mov	dword ptr [rbp + 0x638], r8d
14146ef2f: f3 0f 11 85 3c 06 00 00     	movss	dword ptr [rbp + 0x63c], xmm0
14146ef37: 66 89 95 40 06 00 00        	mov	word ptr [rbp + 0x640], dx
14146ef3e: 88 8d 42 06 00 00           	mov	byte ptr [rbp + 0x642], cl
14146ef44: 0f b6 05 be 5c 1c 02        	movzx	eax, byte ptr [rip + 0x21c5cbe] # 0x143634c09
14146ef4b: b9 f0 00 00 00              	mov	ecx, 0xf0
14146ef50: ba 08 00 00 00              	mov	edx, 0x8
14146ef55: e8 b6 4c 1a ff              	call	0x140613c10 <.text+0x612c10>
14146ef5a: 48 85 c0                    	test	rax, rax
14146ef5d: 48 8b 9d 48 09 00 00        	mov	rbx, qword ptr [rbp + 0x948]
14146ef64: 4c 8b b5 00 09 00 00        	mov	r14, qword ptr [rbp + 0x900]
14146ef6b: 0f 84 dd 09 00 00           	je	0x14146f94e <.text+0x146e94e>
14146ef71: 48 8d 95 b0 05 00 00        	lea	rdx, [rbp + 0x5b0]
14146ef78: 41 b8 f0 00 00 00           	mov	r8d, 0xf0
14146ef7e: 48 89 85 58 09 00 00        	mov	qword ptr [rbp + 0x958], rax
14146ef85: 48 89 c1                    	mov	rcx, rax
14146ef88: e8 73 4c 4b 01              	call	0x142923c00 <.text+0x2922c00>
14146ef8d: 48 8b 05 14 d4 1b 02        	mov	rax, qword ptr [rip + 0x21bd414] # 0x14362c3a8
14146ef94: 48 8b 00                    	mov	rax, qword ptr [rax]
14146ef97: 48 83 f8 04                 	cmp	rax, 0x4
14146ef9b: 48 8b bd 18 09 00 00        	mov	rdi, qword ptr [rbp + 0x918]
14146efa2: 0f 82 f3 00 00 00           	jb	0x14146f09b <.text+0x146e09b>
14146efa8: 48 8d 8d 90 05 00 00        	lea	rcx, [rbp + 0x590]
14146efaf: e8 5c 49 8f ff              	call	0x140d63910 <.text+0xd62910>
14146efb4: 48 89 85 60 03 00 00        	mov	qword ptr [rbp + 0x360], rax
14146efbb: 89 95 68 03 00 00           	mov	dword ptr [rbp + 0x368], edx
14146efc1: 4c 89 ad b0 01 00 00        	mov	qword ptr [rbp + 0x1b0], r13
14146efc8: 48 8d 05 81 b5 91 ff        	lea	rax, [rip - 0x6e4a7f]   # 0x140d8a550 <.text+0xd89550>
14146efcf: 48 89 85 b8 01 00 00        	mov	qword ptr [rbp + 0x1b8], rax
14146efd6: 48 8d 0d 7b 5b 7f 01        	lea	rcx, [rip + 0x17f5b7b]  # 0x142c64b58
14146efdd: e8 9e ed 20 ff              	call	0x14067dd80 <.text+0x67cd80>
14146efe2: 0f 10 00                    	movups	xmm0, xmmword ptr [rax]
14146efe5: 8b 40 10                    	mov	eax, dword ptr [rax + 0x10]
14146efe8: 48 c7 85 e0 05 00 00 04 00 00 00    	mov	qword ptr [rbp + 0x5e0], 0x4
14146eff3: 48 8d 0d 4e 51 7f 01        	lea	rcx, [rip + 0x17f514e]  # 0x142c64148
14146effa: 48 89 8d e8 05 00 00        	mov	qword ptr [rbp + 0x5e8], rcx
14146f001: 48 c7 85 f0 05 00 00 26 00 00 00    	mov	qword ptr [rbp + 0x5f0], 0x26
14146f00c: 48 8d 15 35 5b 7f 01        	lea	rdx, [rip + 0x17f5b35]  # 0x142c64b48
14146f013: 48 89 95 00 06 00 00        	mov	qword ptr [rbp + 0x600], rdx
14146f01a: 48 c7 85 08 06 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x608], 0x1
14146f025: 4c 89 a5 10 06 00 00        	mov	qword ptr [rbp + 0x610], r12
14146f02c: 48 c7 85 18 06 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x618], 0x1
14146f037: 48 c7 85 20 06 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x620], 0x0
14146f042: 48 c7 85 b0 05 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x5b0], 0x0
14146f04d: 48 89 8d b8 05 00 00        	mov	qword ptr [rbp + 0x5b8], rcx
14146f054: 48 c7 85 c0 05 00 00 26 00 00 00    	mov	qword ptr [rbp + 0x5c0], 0x26
14146f05f: 48 c7 85 c8 05 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x5c8], 0x0
14146f06a: 0f 11 85 d0 05 00 00        	movups	xmmword ptr [rbp + 0x5d0], xmm0
14146f071: c7 85 f8 05 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x5f8], 0x1
14146f07b: 89 85 fc 05 00 00           	mov	dword ptr [rbp + 0x5fc], eax
14146f081: 48 8d 8d 72 09 00 00        	lea	rcx, [rbp + 0x972]
14146f088: 48 8d 95 b0 05 00 00        	lea	rdx, [rbp + 0x5b0]
14146f08f: e8 9c b2 8d ff              	call	0x140d4a330 <.text+0xd49330>
14146f094: 48 8b 9d 48 09 00 00        	mov	rbx, qword ptr [rbp + 0x948]
14146f09b: 48 8b b5 68 0a 00 00        	mov	rsi, qword ptr [rbp + 0xa68]
14146f0a2: ba f0 00 00 00              	mov	edx, 0xf0
14146f0a7: 41 b8 08 00 00 00           	mov	r8d, 0x8
14146f0ad: 48 89 d9                    	mov	rcx, rbx
14146f0b0: e8 6b 4b 1a ff              	call	0x140613c20 <.text+0x612c20>
14146f0b5: 48 8b 06                    	mov	rax, qword ptr [rsi]
14146f0b8: f3 0f 10 80 14 01 00 00     	movss	xmm0, dword ptr [rax + 0x114]
14146f0c0: 48 8b 85 60 05 00 00        	mov	rax, qword ptr [rbp + 0x560]
14146f0c7: 8b 8d c0 08 00 00           	mov	ecx, dword ptr [rbp + 0x8c0]
14146f0cd: 89 08                       	mov	dword ptr [rax], ecx
14146f0cf: 48 8b 85 a8 08 00 00        	mov	rax, qword ptr [rbp + 0x8a8]
14146f0d6: f3 0f 11 80 94 01 00 00     	movss	dword ptr [rax + 0x194], xmm0
14146f0de: 8b 7f 68                    	mov	edi, dword ptr [rdi + 0x68]
14146f0e1: 48 8b 85 10 09 00 00        	mov	rax, qword ptr [rbp + 0x910]
14146f0e8: 29 f8                       	sub	eax, edi
14146f0ea: 89 85 58 08 00 00           	mov	dword ptr [rbp + 0x858], eax
14146f0f0: 48 8b 8d 50 05 00 00        	mov	rcx, qword ptr [rbp + 0x550]
14146f0f7: 48 85 c9                    	test	rcx, rcx
14146f0fa: 0f 84 a5 00 00 00           	je	0x14146f1a5 <.text+0x146e1a5>
14146f100: 83 ff ff                    	cmp	edi, -0x1
14146f103: 0f 85 0a 09 00 00           	jne	0x14146fa13 <.text+0x146ea13>
14146f109: 39 c8                       	cmp	eax, ecx
14146f10b: 0f 8d 96 00 00 00           	jge	0x14146f1a7 <.text+0x146e1a7>
14146f111: 48 8d 85 50 05 00 00        	lea	rax, [rbp + 0x550]
14146f118: 48 89 85 60 03 00 00        	mov	qword ptr [rbp + 0x360], rax
14146f11f: 48 8d 05 9a e5 91 ff        	lea	rax, [rip - 0x6e1a66]   # 0x140d8d6c0 <.text+0xd8c6c0>
14146f126: 48 89 85 68 03 00 00        	mov	qword ptr [rbp + 0x368], rax
14146f12d: 48 8d 85 58 08 00 00        	lea	rax, [rbp + 0x858]
14146f134: 48 89 85 70 03 00 00        	mov	qword ptr [rbp + 0x370], rax
14146f13b: 48 8d 05 3e e4 91 ff        	lea	rax, [rip - 0x6e1bc2]   # 0x140d8d580 <.text+0xd8c580>
14146f142: 48 89 85 78 03 00 00        	mov	qword ptr [rbp + 0x378], rax
14146f149: 48 8d 05 88 52 7f 01        	lea	rax, [rip + 0x17f5288]  # 0x142c643d8
14146f150: 48 89 85 b0 05 00 00        	mov	qword ptr [rbp + 0x5b0], rax
14146f157: 48 c7 85 b8 05 00 00 03 00 00 00    	mov	qword ptr [rbp + 0x5b8], 0x3
14146f162: 48 c7 85 d0 05 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x5d0], 0x0
14146f16d: 4c 89 ad c0 05 00 00        	mov	qword ptr [rbp + 0x5c0], r13
14146f174: 48 c7 85 c8 05 00 00 02 00 00 00    	mov	qword ptr [rbp + 0x5c8], 0x2
14146f17f: c6 85 75 09 00 00 01        	mov	byte ptr [rbp + 0x975], 0x1
14146f186: c6 85 74 09 00 00 01        	mov	byte ptr [rbp + 0x974], 0x1
14146f18d: 48 8d 15 74 52 7f 01        	lea	rdx, [rip + 0x17f5274]  # 0x142c64408
14146f194: 48 8d 8d b0 05 00 00        	lea	rcx, [rbp + 0x5b0]
14146f19b: e8 f0 a1 46 01              	call	0x1428d9390 <.text+0x28d8390>
14146f1a0: e9 cb 08 00 00              	jmp	0x14146fa70 <.text+0x146ea70>
14146f1a5: 31 c9                       	xor	ecx, ecx
14146f1a7: 8b 95 cc 08 00 00           	mov	edx, dword ptr [rbp + 0x8cc]
14146f1ad: 4c 8b 85 88 05 00 00        	mov	r8, qword ptr [rbp + 0x588]
14146f1b4: 41 89 10                    	mov	dword ptr [r8], edx
14146f1b7: 29 c8                       	sub	eax, ecx
14146f1b9: 41 c6 46 48 01              	mov	byte ptr [r14 + 0x48], 0x1
14146f1be: 41 89 46 4c                 	mov	dword ptr [r14 + 0x4c], eax
14146f1c2: 48 8b 85 70 08 00 00        	mov	rax, qword ptr [rbp + 0x870]
14146f1c9: 48 8b 88 f8 00 00 00        	mov	rcx, qword ptr [rax + 0xf8]
14146f1d0: 48 8b 90 00 01 00 00        	mov	rdx, qword ptr [rax + 0x100]
14146f1d7: 44 0f b6 80 08 01 00 00     	movzx	r8d, byte ptr [rax + 0x108]
14146f1df: 45 84 c0                    	test	r8b, r8b
14146f1e2: 0f 84 a3 00 00 00           	je	0x14146f28b <.text+0x146e28b>
14146f1e8: 4c 8d 8d 70 03 00 00        	lea	r9, [rbp + 0x370]
14146f1ef: 4c 8d 95 78 03 00 00        	lea	r10, [rbp + 0x378]
14146f1f6: 4c 8d 9d 68 03 00 00        	lea	r11, [rbp + 0x368]
14146f1fd: 48 8d 34 95 00 00 00 00     	lea	rsi, [4*rdx]
14146f205: 45 31 ff                    	xor	r15d, r15d
14146f208: 48 89 c8                    	mov	rax, rcx
14146f20b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
14146f210: 48 85 f6                    	test	rsi, rsi
14146f213: 0f 84 58 01 00 00           	je	0x14146f371 <.text+0x146e371>
14146f219: 44 8b 30                    	mov	r14d, dword ptr [rax]
14146f21c: 48 83 c0 04                 	add	rax, 0x4
14146f220: 48 8b 9d f8 08 00 00        	mov	rbx, qword ptr [rbp + 0x8f8]
14146f227: 48 8b 9b a8 01 00 00        	mov	rbx, qword ptr [rbx + 0x1a8]
14146f22e: 4f 8d 24 f6                 	lea	r12, [r14 + 8*r14]
14146f232: 4e 8b 74 e3 10              	mov	r14, qword ptr [rbx + 8*r12 + 0x10]
14146f237: 48 83 c6 fc                 	add	rsi, -0x4
14146f23b: 4d 85 f6                    	test	r14, r14
14146f23e: 74 d0                       	je	0x14146f210 <.text+0x146e210>
14146f240: 4a 8d 34 e3                 	lea	rsi, [rbx + 8*r12]
14146f244: 48 8b 9d 70 08 00 00        	mov	rbx, qword ptr [rbp + 0x870]
14146f24b: 48 8b 9b 10 01 00 00        	mov	rbx, qword ptr [rbx + 0x110]
14146f252: 4c 8b 7e 18                 	mov	r15, qword ptr [rsi + 0x18]
14146f256: 48 8b 76 38                 	mov	rsi, qword ptr [rsi + 0x38]
14146f25a: 48 8b 34 de                 	mov	rsi, qword ptr [rsi + 8*rbx]
14146f25e: 48 f7 d6                    	not	rsi
14146f261: 4c 8d 24 76                 	lea	r12, [rsi + 2*rsi]
14146f265: 49 c1 e4 04                 	shl	r12, 0x4
14146f269: 4b 8b 74 27 28              	mov	rsi, qword ptr [r15 + r12 + 0x28]
14146f26e: 4b 8b 5c 27 10              	mov	rbx, qword ptr [r15 + r12 + 0x10]
14146f273: 4f 8b 7c 27 20              	mov	r15, qword ptr [r15 + r12 + 0x20]
14146f278: 48 89 9d 60 03 00 00        	mov	qword ptr [rbp + 0x360], rbx
14146f27f: 4c 89 bd 68 03 00 00        	mov	qword ptr [rbp + 0x368], r15
14146f286: e9 cf 00 00 00              	jmp	0x14146f35a <.text+0x146e35a>
14146f28b: 4c 8d 8d c0 05 00 00        	lea	r9, [rbp + 0x5c0]
14146f292: 4c 8d 95 c8 05 00 00        	lea	r10, [rbp + 0x5c8]
14146f299: 4c 8d 9d b8 05 00 00        	lea	r11, [rbp + 0x5b8]
14146f2a0: 48 8d 34 95 00 00 00 00     	lea	rsi, [4*rdx]
14146f2a8: 45 31 ff                    	xor	r15d, r15d
14146f2ab: 48 89 c8                    	mov	rax, rcx
14146f2ae: 66 90                       	nop
14146f2b0: 48 85 f6                    	test	rsi, rsi
14146f2b3: 0f 84 b8 00 00 00           	je	0x14146f371 <.text+0x146e371>
14146f2b9: 44 8b 30                    	mov	r14d, dword ptr [rax]
14146f2bc: 48 83 c0 04                 	add	rax, 0x4
14146f2c0: 48 8b 9d f8 08 00 00        	mov	rbx, qword ptr [rbp + 0x8f8]
14146f2c7: 48 8b 9b 00 01 00 00        	mov	rbx, qword ptr [rbx + 0x100]
14146f2ce: 4f 8d 24 b6                 	lea	r12, [r14 + 4*r14]
14146f2d2: 49 c1 e4 05                 	shl	r12, 0x5
14146f2d6: 4e 8b 74 23 58              	mov	r14, qword ptr [rbx + r12 + 0x58]
14146f2db: 48 83 c6 fc                 	add	rsi, -0x4
14146f2df: 4d 85 f6                    	test	r14, r14
14146f2e2: 74 cc                       	je	0x14146f2b0 <.text+0x146e2b0>
14146f2e4: 4c 01 e3                    	add	rbx, r12
14146f2e7: 8b b3 94 00 00 00           	mov	esi, dword ptr [rbx + 0x94]
14146f2ed: 4c 8b bd f8 08 00 00        	mov	r15, qword ptr [rbp + 0x8f8]
14146f2f4: 4d 8b bf a8 01 00 00        	mov	r15, qword ptr [r15 + 0x1a8]
14146f2fb: 48 8d 34 f6                 	lea	rsi, [rsi + 8*rsi]
14146f2ff: 4c 8b a5 70 08 00 00        	mov	r12, qword ptr [rbp + 0x870]
14146f306: 4d 8b a4 24 10 01 00 00     	mov	r12, qword ptr [r12 + 0x110]
14146f30e: 4d 8b 6c f7 18              	mov	r13, qword ptr [r15 + 8*rsi + 0x18]
14146f313: 49 8b 74 f7 38              	mov	rsi, qword ptr [r15 + 8*rsi + 0x38]
14146f318: 4a 8b 34 e6                 	mov	rsi, qword ptr [rsi + 8*r12]
14146f31c: 48 f7 d6                    	not	rsi
14146f31f: 48 8b 5b 50                 	mov	rbx, qword ptr [rbx + 0x50]
14146f323: 4c 8d 3c 76                 	lea	r15, [rsi + 2*rsi]
14146f327: 49 c1 e7 04                 	shl	r15, 0x4
14146f32b: 8b 73 08                    	mov	esi, dword ptr [rbx + 0x8]
14146f32e: 48 8d 1c b6                 	lea	rbx, [rsi + 4*rsi]
14146f332: 48 c1 e3 04                 	shl	rbx, 0x4
14146f336: 4b 03 5c 3d 10              	add	rbx, qword ptr [r13 + r15 + 0x10]
14146f33b: 48 c1 e6 02                 	shl	rsi, 0x2
14146f33f: 4f 8b 64 3d 20              	mov	r12, qword ptr [r13 + r15 + 0x20]
14146f344: 49 01 f4                    	add	r12, rsi
14146f347: 4b 03 74 3d 28              	add	rsi, qword ptr [r13 + r15 + 0x28]
14146f34c: 48 89 9d b0 05 00 00        	mov	qword ptr [rbp + 0x5b0], rbx
14146f353: 4c 89 a5 b8 05 00 00        	mov	qword ptr [rbp + 0x5b8], r12
14146f35a: 49 89 31                    	mov	qword ptr [r9], rsi
14146f35d: 41 0f 13 32                 	movlps	qword ptr [r10], xmm6
14146f361: 4d 8b 4b 08                 	mov	r9, qword ptr [r11 + 0x8]
14146f365: 4d 8b 53 10                 	mov	r10, qword ptr [r11 + 0x10]
14146f369: 41 bf 01 00 00 00           	mov	r15d, 0x1
14146f36f: eb 05                       	jmp	0x14146f376 <.text+0x146e376>
14146f371: 45 31 f6                    	xor	r14d, r14d
14146f374: 31 db                       	xor	ebx, ebx
14146f376: 48 8b b5 60 09 00 00        	mov	rsi, qword ptr [rbp + 0x960]
14146f37d: 4c 8b a5 50 09 00 00        	mov	r12, qword ptr [rbp + 0x950]
14146f384: 48 8d 0c 91                 	lea	rcx, [rcx + 4*rdx]
14146f388: 45 84 c0                    	test	r8b, r8b
14146f38b: 74 37                       	je	0x14146f3c4 <.text+0x146e3c4>
14146f38d: 4d 39 f7                    	cmp	r15, r14
14146f390: 4c 8b 9d f8 08 00 00        	mov	r11, qword ptr [rbp + 0x8f8]
14146f397: 75 5d                       	jne	0x14146f3f6 <.text+0x146e3f6>
14146f399: 0f 1f 80 00 00 00 00        	nop	dword ptr [rax]
14146f3a0: 48 39 c8                    	cmp	rax, rcx
14146f3a3: 0f 84 b6 00 00 00           	je	0x14146f45f <.text+0x146e45f>
14146f3a9: 8b 10                       	mov	edx, dword ptr [rax]
14146f3ab: 48 83 c0 04                 	add	rax, 0x4
14146f3af: 4d 8b 83 a8 01 00 00        	mov	r8, qword ptr [r11 + 0x1a8]
14146f3b6: 48 8d 14 d2                 	lea	rdx, [rdx + 8*rdx]
14146f3ba: 49 83 7c d0 10 00           	cmp	qword ptr [r8 + 8*rdx + 0x10], 0x0
14146f3c0: 74 de                       	je	0x14146f3a0 <.text+0x146e3a0>
14146f3c2: eb 32                       	jmp	0x14146f3f6 <.text+0x146e3f6>
14146f3c4: 4d 39 f7                    	cmp	r15, r14
14146f3c7: 4c 8b 9d f8 08 00 00        	mov	r11, qword ptr [rbp + 0x8f8]
14146f3ce: 75 26                       	jne	0x14146f3f6 <.text+0x146e3f6>
14146f3d0: 48 39 c8                    	cmp	rax, rcx
14146f3d3: 0f 84 86 00 00 00           	je	0x14146f45f <.text+0x146e45f>
14146f3d9: 8b 10                       	mov	edx, dword ptr [rax]
14146f3db: 48 83 c0 04                 	add	rax, 0x4
14146f3df: 4d 8b 83 00 01 00 00        	mov	r8, qword ptr [r11 + 0x100]
14146f3e6: 48 8d 14 92                 	lea	rdx, [rdx + 4*rdx]
14146f3ea: 48 c1 e2 05                 	shl	rdx, 0x5
14146f3ee: 49 83 7c 10 58 00           	cmp	qword ptr [r8 + rdx + 0x58], 0x0
14146f3f4: 74 da                       	je	0x14146f3d0 <.text+0x146e3d0>
14146f3f6: 48 85 db                    	test	rbx, rbx
14146f3f9: 0f 94 c0                    	sete	al
14146f3fc: 34 01                       	xor	al, 0x1
14146f3fe: 0f b6 c0                    	movzx	eax, al
14146f401: 48 89 85 b0 05 00 00        	mov	qword ptr [rbp + 0x5b0], rax
14146f408: 48 8d 05 a3 4b 7f 01        	lea	rax, [rip + 0x17f4ba3]  # 0x142c63fb2
14146f40f: 48 89 85 b8 05 00 00        	mov	qword ptr [rbp + 0x5b8], rax
14146f416: 48 c7 85 c0 05 00 00 5f 00 00 00    	mov	qword ptr [rbp + 0x5c0], 0x5f
14146f421: c6 85 75 09 00 00 01        	mov	byte ptr [rbp + 0x975], 0x1
14146f428: c6 85 74 09 00 00 01        	mov	byte ptr [rbp + 0x974], 0x1
14146f42f: 48 8d 05 ea 4f 7f 01        	lea	rax, [rip + 0x17f4fea]  # 0x142c64420
14146f436: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
14146f43b: 48 8d 0d 3e 4a 7f 01        	lea	rcx, [rip + 0x17f4a3e]  # 0x142c63e80
14146f442: 4c 8d 0d 67 4a 7f 01        	lea	r9, [rip + 0x17f4a67]   # 0x142c63eb0
14146f449: 4c 8d 85 b0 05 00 00        	lea	r8, [rbp + 0x5b0]
14146f450: ba 2b 00 00 00              	mov	edx, 0x2b
14146f455: e8 06 a3 46 01              	call	0x1428d9760 <.text+0x28d8760>
14146f45a: e9 11 06 00 00              	jmp	0x14146fa70 <.text+0x146ea70>
14146f45f: 48 85 db                    	test	rbx, rbx
14146f462: 0f 94 c0                    	sete	al
14146f465: 74 95                       	je	0x14146f3fc <.text+0x146e3fc>
14146f467: 49 c1 ea 20                 	shr	r10, 0x20
14146f46b: 45 89 11                    	mov	dword ptr [r9], r10d
14146f46e: 48 8b 03                    	mov	rax, qword ptr [rbx]
14146f471: 48 89 c1                    	mov	rcx, rax
14146f474: 48 f7 d9                    	neg	rcx
14146f477: 70 19                       	jo	0x14146f492 <.text+0x146e492>
14146f479: 73 17                       	jae	0x14146f492 <.text+0x146e492>
14146f47b: 48 8b 4b 08                 	mov	rcx, qword ptr [rbx + 0x8]
14146f47f: 48 c1 e0 03                 	shl	rax, 0x3
14146f483: 48 8d 14 80                 	lea	rdx, [rax + 4*rax]
14146f487: 41 b8 04 00 00 00           	mov	r8d, 0x4
14146f48d: e8 8e 47 1a ff              	call	0x140613c20 <.text+0x612c20>
14146f492: 4c 89 23                    	mov	qword ptr [rbx], r12
14146f495: 48 89 73 08                 	mov	qword ptr [rbx + 0x8], rsi
14146f499: 48 8b 85 d8 08 00 00        	mov	rax, qword ptr [rbp + 0x8d8]
14146f4a0: 48 89 43 10                 	mov	qword ptr [rbx + 0x10], rax
14146f4a4: 39 bd 10 09 00 00           	cmp	dword ptr [rbp + 0x910], edi
14146f4aa: 75 63                       	jne	0x14146f50f <.text+0x146e50f>
14146f4ac: 48 8b 85 30 05 00 00        	mov	rax, qword ptr [rbp + 0x530]
14146f4b3: 8b 8d 60 08 00 00           	mov	ecx, dword ptr [rbp + 0x860]
14146f4b9: 89 08                       	mov	dword ptr [rax], ecx
14146f4bb: 48 8b 85 78 08 00 00        	mov	rax, qword ptr [rbp + 0x878]
14146f4c2: 48 8b 70 30                 	mov	rsi, qword ptr [rax + 0x30]
14146f4c6: 48 8b 78 40                 	mov	rdi, qword ptr [rax + 0x40]
14146f4ca: 48 3b 70 20                 	cmp	rsi, qword ptr [rax + 0x20]
14146f4ce: 75 25                       	jne	0x14146f4f5 <.text+0x146e4f5>
14146f4d0: 48 8b 85 78 08 00 00        	mov	rax, qword ptr [rbp + 0x878]
14146f4d7: 48 8d 48 20                 	lea	rcx, [rax + 0x20]
14146f4db: c6 85 75 09 00 00 00        	mov	byte ptr [rbp + 0x975], 0x0
14146f4e2: c6 85 74 09 00 00 01        	mov	byte ptr [rbp + 0x974], 0x1
14146f4e9: 48 8d 15 88 9b 81 01        	lea	rdx, [rip + 0x1819b88]  # 0x142c89078
14146f4f0: e8 7b 33 0f ff              	call	0x140562870 <.text+0x561870>
14146f4f5: 48 8b 8d 78 08 00 00        	mov	rcx, qword ptr [rbp + 0x878]
14146f4fc: 48 8b 41 28                 	mov	rax, qword ptr [rcx + 0x28]
14146f500: 48 89 3c f0                 	mov	qword ptr [rax + 8*rsi], rdi
14146f504: 48 ff c6                    	inc	rsi
14146f507: 48 89 71 30                 	mov	qword ptr [rcx + 0x30], rsi
14146f50b: 48 ff 41 40                 	inc	qword ptr [rcx + 0x40]
14146f50f: 48 8b 85 e0 08 00 00        	mov	rax, qword ptr [rbp + 0x8e0]
14146f516: 0f 10 00                    	movups	xmm0, xmmword ptr [rax]
14146f519: 0f 29 85 b0 05 00 00        	movaps	xmmword ptr [rbp + 0x5b0], xmm0
14146f520: c6 85 75 09 00 00 00        	mov	byte ptr [rbp + 0x975], 0x0
14146f527: c6 85 74 09 00 00 01        	mov	byte ptr [rbp + 0x974], 0x1
14146f52e: 48 8d 8d b0 01 00 00        	lea	rcx, [rbp + 0x1b0]
14146f535: 48 8d 95 b0 05 00 00        	lea	rdx, [rbp + 0x5b0]
14146f53c: e8 2f 8f 67 ff              	call	0x140ae8470 <.text+0xae7470>
14146f541: 48 8b 85 40 0a 00 00        	mov	rax, qword ptr [rbp + 0xa40]
14146f548: 48 8b 95 18 09 00 00        	mov	rdx, qword ptr [rbp + 0x918]
14146f54f: 48 8b 4a 10                 	mov	rcx, qword ptr [rdx + 0x10]
14146f553: 4c 8b 85 70 05 00 00        	mov	r8, qword ptr [rbp + 0x570]
14146f55a: 44 8b 8d c4 08 00 00        	mov	r9d, dword ptr [rbp + 0x8c4]
14146f561: 45 89 08                    	mov	dword ptr [r8], r9d
14146f564: 48 89 4a 58                 	mov	qword ptr [rdx + 0x58], rcx
14146f568: 44 8b 40 1c                 	mov	r8d, dword ptr [rax + 0x1c]
14146f56c: 48 8b 10                    	mov	rdx, qword ptr [rax]
14146f56f: 48 8b 40 10                 	mov	rax, qword ptr [rax + 0x10]
14146f573: 48 89 8a 20 01 00 00        	mov	qword ptr [rdx + 0x120], rcx
14146f57a: 8b 8d c8 01 00 00           	mov	ecx, dword ptr [rbp + 0x1c8]
14146f580: 44 89 00                    	mov	dword ptr [rax], r8d
14146f583: 89 8a 40 01 00 00           	mov	dword ptr [rdx + 0x140], ecx
14146f589: c7 82 44 01 00 00 00 00 00 00       	mov	dword ptr [rdx + 0x144], 0x0
14146f593: 48 8b 85 e0 08 00 00        	mov	rax, qword ptr [rbp + 0x8e0]
14146f59a: 0f 10 00                    	movups	xmm0, xmmword ptr [rax]
14146f59d: 0f 11 85 e0 05 00 00        	movups	xmmword ptr [rbp + 0x5e0], xmm0
14146f5a4: 0f 10 85 b0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1b0]
14146f5ab: 0f 10 8d c0 01 00 00        	movups	xmm1, xmmword ptr [rbp + 0x1c0]
14146f5b2: 0f 11 85 b8 05 00 00        	movups	xmmword ptr [rbp + 0x5b8], xmm0
14146f5b9: 0f 11 8d c8 05 00 00        	movups	xmmword ptr [rbp + 0x5c8], xmm1
14146f5c0: 48 8b 85 58 09 00 00        	mov	rax, qword ptr [rbp + 0x958]
14146f5c7: 48 89 85 d8 05 00 00        	mov	qword ptr [rbp + 0x5d8], rax
14146f5ce: 48 c7 85 b0 05 00 00 03 00 00 00    	mov	qword ptr [rbp + 0x5b0], 0x3
14146f5d9: c6 85 75 09 00 00 00        	mov	byte ptr [rbp + 0x975], 0x0
14146f5e0: c6 85 74 09 00 00 00        	mov	byte ptr [rbp + 0x974], 0x0
14146f5e7: 48 8d 8d 60 03 00 00        	lea	rcx, [rbp + 0x360]
14146f5ee: 4c 8d 85 b0 05 00 00        	lea	r8, [rbp + 0x5b0]
14146f5f5: e8 86 ac 46 ff              	call	0x1408da280 <.text+0x8d9280>
14146f5fa: 83 bd 60 03 00 00 0b        	cmp	dword ptr [rbp + 0x360], 0xb
14146f601: 0f 85 5b 03 00 00           	jne	0x14146f962 <.text+0x146e962>
14146f607: 48 8b 95 30 08 00 00        	mov	rdx, qword ptr [rbp + 0x830]
14146f60e: 48 85 d2                    	test	rdx, rdx
14146f611: 74 12                       	je	0x14146f625 <.text+0x146e625>
14146f613: 48 8b 8d 38 08 00 00        	mov	rcx, qword ptr [rbp + 0x838]
14146f61a: 41 b8 01 00 00 00           	mov	r8d, 0x1
14146f620: e8 fb 45 1a ff              	call	0x140613c20 <.text+0x612c20>
14146f625: 0f 28 b5 80 09 00 00        	movaps	xmm6, xmmword ptr [rbp + 0x980]
14146f62c: 0f 28 bd 90 09 00 00        	movaps	xmm7, xmmword ptr [rbp + 0x990]
14146f633: 44 0f 28 85 a0 09 00 00     	movaps	xmm8, xmmword ptr [rbp + 0x9a0]
14146f63b: 44 0f 28 8d b0 09 00 00     	movaps	xmm9, xmmword ptr [rbp + 0x9b0]
14146f643: 44 0f 28 95 c0 09 00 00     	movaps	xmm10, xmmword ptr [rbp + 0x9c0]
14146f64b: 48 81 c4 58 0a 00 00        	add	rsp, 0xa58
14146f652: 5b                          	pop	rbx
14146f653: 5f                          	pop	rdi
14146f654: 5e                          	pop	rsi
14146f655: 41 5c                       	pop	r12
14146f657: 41 5d                       	pop	r13
14146f659: 41 5e                       	pop	r14
14146f65b: 41 5f                       	pop	r15
14146f65d: 5d                          	pop	rbp
14146f65e: c3                          	ret
14146f65f: 48 8b bd b0 01 00 00        	mov	rdi, qword ptr [rbp + 0x1b0]
14146f666: 8a 9d bf 01 00 00           	mov	bl, byte ptr [rbp + 0x1bf]
14146f66c: 48 8d 4d b0                 	lea	rcx, [rbp - 0x50]
14146f670: e8 5b 78 90 ff              	call	0x140d76ed0 <.text+0xd75ed0>
14146f675: 48 89 c1                    	mov	rcx, rax
14146f678: e8 03 74 82 ff              	call	0x140c96a80 <.text+0xc95a80>
14146f67d: 48 85 c0                    	test	rax, rax
14146f680: 0f 84 f9 01 00 00           	je	0x14146f87f <.text+0x146e87f>
14146f686: 48 8d 0d c3 4d 7f 01        	lea	rcx, [rip + 0x17f4dc3]  # 0x142c64450
14146f68d: 48 89 8d 60 03 00 00        	mov	qword ptr [rbp + 0x360], rcx
14146f694: 48 c7 85 68 03 00 00 23 00 00 00    	mov	qword ptr [rbp + 0x368], 0x23
14146f69f: 48 89 85 70 03 00 00        	mov	qword ptr [rbp + 0x370], rax
14146f6a6: 48 c7 85 b0 05 00 00 03 00 00 00    	mov	qword ptr [rbp + 0x5b0], 0x3
14146f6b1: 48 8d 8d 60 03 00 00        	lea	rcx, [rbp + 0x360]
14146f6b8: 48 8d 95 b0 05 00 00        	lea	rdx, [rbp + 0x5b0]
14146f6bf: e8 5c 93 43 01              	call	0x1428a8a20 <.text+0x28a7a20>
14146f6c4: 48 89 c7                    	mov	rdi, rax
14146f6c7: 48 8d 8d b0 01 00 00        	lea	rcx, [rbp + 0x1b0]
14146f6ce: e8 2d 03 42 ff              	call	0x14088fa00 <.text+0x88ea00>
14146f6d3: e9 b6 ea ff ff              	jmp	0x14146e18e <.text+0x146d18e>
14146f6d8: 48 8b 8d 18 02 00 00        	mov	rcx, qword ptr [rbp + 0x218]
14146f6df: 48 85 c9                    	test	rcx, rcx
14146f6e2: 0f 84 b6 00 00 00           	je	0x14146f79e <.text+0x146e79e>
14146f6e8: 48 8b 85 10 02 00 00        	mov	rax, qword ptr [rbp + 0x210]
14146f6ef: 48 8d 0c 89                 	lea	rcx, [rcx + 4*rcx]
14146f6f3: 48 8d 0c c8                 	lea	rcx, [rax + 8*rcx]
14146f6f7: 48 8d 15 ce 55 7f 01        	lea	rdx, [rip + 0x17f55ce]  # 0x142c64ccc
14146f6fe: eb 0d                       	jmp	0x14146f70d <.text+0x146e70d>
14146f700: 48 83 c0 28                 	add	rax, 0x28
14146f704: 48 39 c8                    	cmp	rax, rcx
14146f707: 0f 84 91 00 00 00           	je	0x14146f79e <.text+0x146e79e>
14146f70d: 4c 8b 48 20                 	mov	r9, qword ptr [rax + 0x20]
14146f711: 4d 85 c9                    	test	r9, r9
14146f714: 74 ea                       	je	0x14146f700 <.text+0x146e700>
14146f716: 4c 8b 40 18                 	mov	r8, qword ptr [rax + 0x18]
14146f71a: 4d 6b c9 58                 	imul	r9, r9, 0x58
14146f71e: 45 31 d2                    	xor	r10d, r10d
14146f721: eb 1e                       	jmp	0x14146f741 <.text+0x146e741>
14146f723: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
14146f730: 41 89 f3                    	mov	r11d, esi
14146f733: 47 89 5c 10 30              	mov	dword ptr [r8 + r10 + 0x30], r11d
14146f738: 49 83 c2 58                 	add	r10, 0x58
14146f73c: 4d 39 d1                    	cmp	r9, r10
14146f73f: 74 bf                       	je	0x14146f700 <.text+0x146e700>
14146f741: 47 0f b6 5c 10 52           	movzx	r11d, byte ptr [r8 + r10 + 0x52]
14146f747: 41 80 c3 c8                 	add	r11b, -0x38
14146f74b: 41 80 fb 02                 	cmp	r11b, 0x2
14146f74f: 77 e7                       	ja	0x14146f738 <.text+0x146e738>
14146f751: 43 8b 74 10 30              	mov	esi, dword ptr [r8 + r10 + 0x30]
14146f756: 48 83 fe 08                 	cmp	rsi, 0x8
14146f75a: 77 12                       	ja	0x14146f76e <.text+0x146e76e>
14146f75c: 4c 63 1c b2                 	movsxd	r11, dword ptr [rdx + 4*rsi]
14146f760: 49 01 d3                    	add	r11, rdx
14146f763: 41 ff e3                    	jmp	r11
14146f766: 41 bb 02 00 00 00           	mov	r11d, 0x2
14146f76c: eb c5                       	jmp	0x14146f733 <.text+0x146e733>
14146f76e: 41 bb 01 00 00 00           	mov	r11d, 0x1
14146f774: 81 fe e7 03 00 00           	cmp	esi, 0x3e7
14146f77a: 74 b4                       	je	0x14146f730 <.text+0x146e730>
14146f77c: eb b5                       	jmp	0x14146f733 <.text+0x146e733>
14146f77e: 41 bb 03 00 00 00           	mov	r11d, 0x3
14146f784: eb ad                       	jmp	0x14146f733 <.text+0x146e733>
14146f786: 41 bb 05 00 00 00           	mov	r11d, 0x5
14146f78c: eb a5                       	jmp	0x14146f733 <.text+0x146e733>
14146f78e: 41 bb 08 00 00 00           	mov	r11d, 0x8
14146f794: eb 9d                       	jmp	0x14146f733 <.text+0x146e733>
14146f796: 41 bb 04 00 00 00           	mov	r11d, 0x4
14146f79c: eb 95                       	jmp	0x14146f733 <.text+0x146e733>
14146f79e: 4c 8d a5 60 03 00 00        	lea	r12, [rbp + 0x360]
14146f7a5: 48 8d 95 b0 01 00 00        	lea	rdx, [rbp + 0x1b0]
14146f7ac: 41 b8 b0 01 00 00           	mov	r8d, 0x1b0
14146f7b2: 4c 89 e1                    	mov	rcx, r12
14146f7b5: e8 46 44 4b 01              	call	0x142923c00 <.text+0x2922c00>
14146f7ba: 4c 8d b5 b0 05 00 00        	lea	r14, [rbp + 0x5b0]
14146f7c1: 41 b8 80 02 00 00           	mov	r8d, 0x280
14146f7c7: 4c 89 f1                    	mov	rcx, r14
14146f7ca: 4c 89 fa                    	mov	rdx, r15
14146f7cd: e8 2e 44 4b 01              	call	0x142923c00 <.text+0x2922c00>
14146f7d2: c6 85 73 09 00 00 00        	mov	byte ptr [rbp + 0x973], 0x0
14146f7d9: 4c 89 e1                    	mov	rcx, r12
14146f7dc: 48 8b 95 80 05 00 00        	mov	rdx, qword ptr [rbp + 0x580]
14146f7e3: 4d 89 f0                    	mov	r8, r14
14146f7e6: e8 65 a4 33 00              	call	0x1417a9c50 <.text+0x17a8c50>
14146f7eb: 48 0f ba a5 10 09 00 00 28  	bt	qword ptr [rbp + 0x910], 0x28
14146f7f4: 48 8b 85 18 09 00 00        	mov	rax, qword ptr [rbp + 0x918]
14146f7fb: 89 78 68                    	mov	dword ptr [rax + 0x68], edi
14146f7fe: 48 89 c7                    	mov	rdi, rax
14146f801: 0f 83 ad e7 ff ff           	jae	0x14146dfb4 <.text+0x146cfb4>
14146f807: 48 8b 47 20                 	mov	rax, qword ptr [rdi + 0x20]
14146f80b: 48 89 85 28 09 00 00        	mov	qword ptr [rbp + 0x928], rax
14146f812: 48 8b 47 28                 	mov	rax, qword ptr [rdi + 0x28]
14146f816: 48 c7 47 28 00 00 00 00     	mov	qword ptr [rdi + 0x28], 0x0
14146f81e: 48 89 85 38 09 00 00        	mov	qword ptr [rbp + 0x938], rax
14146f825: 48 85 c0                    	test	rax, rax
14146f828: 0f 84 86 e7 ff ff           	je	0x14146dfb4 <.text+0x146cfb4>
14146f82e: 48 8b b5 38 09 00 00        	mov	rsi, qword ptr [rbp + 0x938]
14146f835: 48 f7 de                    	neg	rsi
14146f838: b9 01 00 00 00              	mov	ecx, 0x1
14146f83d: 4c 8b bd 28 09 00 00        	mov	r15, qword ptr [rbp + 0x928]
14146f844: eb 1f                       	jmp	0x14146f865 <.text+0x146e865>
14146f846: 48 8b 8d 58 09 00 00        	mov	rcx, qword ptr [rbp + 0x958]
14146f84d: 48 8d 04 0e                 	lea	rax, [rsi + rcx]
14146f851: 48 ff c0                    	inc	rax
14146f854: 48 ff c1                    	inc	rcx
14146f857: 49 83 c7 10                 	add	r15, 0x10
14146f85b: 48 83 f8 01                 	cmp	rax, 0x1
14146f85f: 0f 84 4f e7 ff ff           	je	0x14146dfb4 <.text+0x146cfb4>
14146f865: 48 89 8d 58 09 00 00        	mov	qword ptr [rbp + 0x958], rcx
14146f86c: 49 8b 07                    	mov	rax, qword ptr [r15]
14146f86f: f0                          	lock
14146f870: 48 ff 08                    	dec	qword ptr [rax]
14146f873: 75 d1                       	jne	0x14146f846 <.text+0x146e846>
14146f875: 4c 89 f9                    	mov	rcx, r15
14146f878: e8 03 65 e7 fe              	call	0x1402e5d80 <.text+0x2e4d80>
14146f87d: eb c7                       	jmp	0x14146f846 <.text+0x146e846>
14146f87f: 8b 85 b8 01 00 00           	mov	eax, dword ptr [rbp + 0x1b8]
14146f885: 8b 8d bb 01 00 00           	mov	ecx, dword ptr [rbp + 0x1bb]
14146f88b: 89 8d f3 08 00 00           	mov	dword ptr [rbp + 0x8f3], ecx
14146f891: 89 85 f0 08 00 00           	mov	dword ptr [rbp + 0x8f0], eax
14146f897: 80 fb 02                    	cmp	bl, 0x2
14146f89a: 0f 84 ee e8 ff ff           	je	0x14146e18e <.text+0x146d18e>
14146f8a0: e9 a4 f1 ff ff              	jmp	0x14146ea49 <.text+0x146da49>
14146f8a5: 48 8d 0d e4 47 7f 01        	lea	rcx, [rip + 0x17f47e4]  # 0x142c64090
14146f8ac: 4c 8d 05 35 48 7f 01        	lea	r8, [rip + 0x17f4835]   # 0x142c640e8
14146f8b3: ba 58 00 00 00              	mov	edx, 0x58
14146f8b8: e8 73 9b 46 01              	call	0x1428d9430 <.text+0x28d8430>
14146f8bd: 48 8b 85 b8 05 00 00        	mov	rax, qword ptr [rbp + 0x5b8]
14146f8c4: 48 89 85 68 03 00 00        	mov	qword ptr [rbp + 0x368], rax
14146f8cb: 48 c7 85 60 03 00 00 02 00 00 00    	mov	qword ptr [rbp + 0x360], 0x2
14146f8d6: 48 8d 8d 68 03 00 00        	lea	rcx, [rbp + 0x368]
14146f8dd: e8 1e 81 17 ff              	call	0x1405e7a00 <.text+0x5e6a00>
14146f8e2: 48 8d 85 18 05 00 00        	lea	rax, [rbp + 0x518]
14146f8e9: 48 89 85 60 03 00 00        	mov	qword ptr [rbp + 0x360], rax
14146f8f0: 48 8d 05 f9 ef bb fe        	lea	rax, [rip - 0x1441007]  # 0x14002e8f0 <.text+0x2d8f0>
14146f8f7: 48 89 85 68 03 00 00        	mov	qword ptr [rbp + 0x368], rax
14146f8fe: 48 8d 05 1b 47 7f 01        	lea	rax, [rip + 0x17f471b]  # 0x142c64020
14146f905: 48 89 85 b0 05 00 00        	mov	qword ptr [rbp + 0x5b0], rax
14146f90c: 48 c7 85 b8 05 00 00 02 00 00 00    	mov	qword ptr [rbp + 0x5b8], 0x2
14146f917: 48 c7 85 d0 05 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x5d0], 0x0
14146f922: 48 8d 85 60 03 00 00        	lea	rax, [rbp + 0x360]
14146f929: 48 89 85 c0 05 00 00        	mov	qword ptr [rbp + 0x5c0], rax
14146f930: 48 c7 85 c8 05 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x5c8], 0x1
14146f93b: 48 8d 15 36 47 7f 01        	lea	rdx, [rip + 0x17f4736]  # 0x142c64078
14146f942: 48 8d 8d b0 05 00 00        	lea	rcx, [rbp + 0x5b0]
14146f949: e8 42 9a 46 01              	call	0x1428d9390 <.text+0x28d8390>
14146f94e: b9 08 00 00 00              	mov	ecx, 0x8
14146f953: ba f0 00 00 00              	mov	edx, 0xf0
14146f958: e8 a3 96 46 01              	call	0x1428d9000 <.text+0x28d8000>
14146f95d: e9 0e 01 00 00              	jmp	0x14146fa70 <.text+0x146ea70>
14146f962: 0f 10 85 60 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x360]
14146f969: 0f 10 8d 70 03 00 00        	movups	xmm1, xmmword ptr [rbp + 0x370]
14146f970: 0f 10 95 80 03 00 00        	movups	xmm2, xmmword ptr [rbp + 0x380]
14146f977: 0f 10 9d 90 03 00 00        	movups	xmm3, xmmword ptr [rbp + 0x390]
14146f97e: 0f 29 9d e0 05 00 00        	movaps	xmmword ptr [rbp + 0x5e0], xmm3
14146f985: 0f 29 95 d0 05 00 00        	movaps	xmmword ptr [rbp + 0x5d0], xmm2
14146f98c: 0f 29 8d c0 05 00 00        	movaps	xmmword ptr [rbp + 0x5c0], xmm1
14146f993: 0f 29 85 b0 05 00 00        	movaps	xmmword ptr [rbp + 0x5b0], xmm0
14146f99a: 48 8d 05 97 4a 7f 01        	lea	rax, [rip + 0x17f4a97]  # 0x142c64438
14146f9a1: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
14146f9a6: 48 8d 0d d3 44 7f 01        	lea	rcx, [rip + 0x17f44d3]  # 0x142c63e80
14146f9ad: 4c 8d 0d ac 44 7f 01        	lea	r9, [rip + 0x17f44ac]   # 0x142c63e60
14146f9b4: 4c 8d 85 b0 05 00 00        	lea	r8, [rbp + 0x5b0]
14146f9bb: ba 2b 00 00 00              	mov	edx, 0x2b
14146f9c0: e8 9b 9d 46 01              	call	0x1428d9760 <.text+0x28d8760>
14146f9c5: e9 a6 00 00 00              	jmp	0x14146fa70 <.text+0x146ea70>
14146f9ca: 4c 8d 05 77 91 7c 01        	lea	r8, [rip + 0x17c9177]   # 0x142c38b48
14146f9d1: 4c 89 e9                    	mov	rcx, r13
14146f9d4: 4c 89 e2                    	mov	rdx, r12
14146f9d7: e8 07 96 46 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
14146f9dc: e9 8f 00 00 00              	jmp	0x14146fa70 <.text+0x146ea70>
14146f9e1: 48 b8 00 00 00 00 00 00 00 80       	movabs	rax, -0x8000000000000000
14146f9eb: 48 89 85 30 09 00 00        	mov	qword ptr [rbp + 0x930], rax
14146f9f2: c6 85 76 09 00 00 01        	mov	byte ptr [rbp + 0x976], 0x1
14146f9f9: 48 8d 0d d0 47 7f 01        	lea	rcx, [rip + 0x17f47d0]  # 0x142c641d0
14146fa00: 4c 8d 05 f1 47 7f 01        	lea	r8, [rip + 0x17f47f1]   # 0x142c641f8
14146fa07: ba 28 00 00 00              	mov	edx, 0x28
14146fa0c: e8 1f 9a 46 01              	call	0x1428d9430 <.text+0x28d8430>
14146fa11: eb 5d                       	jmp	0x14146fa70 <.text+0x146ea70>
14146fa13: 48 8d 05 5e 49 7f 01        	lea	rax, [rip + 0x17f495e]  # 0x142c64378
14146fa1a: 48 89 85 b0 05 00 00        	mov	qword ptr [rbp + 0x5b0], rax
14146fa21: 48 c7 85 b8 05 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x5b8], 0x1
14146fa2c: 48 c7 85 c0 05 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x5c0], 0x8
14146fa37: 0f 57 c0                    	xorps	xmm0, xmm0
14146fa3a: 0f 11 85 c8 05 00 00        	movups	xmmword ptr [rbp + 0x5c8], xmm0
14146fa41: c6 85 75 09 00 00 01        	mov	byte ptr [rbp + 0x975], 0x1
14146fa48: c6 85 74 09 00 00 01        	mov	byte ptr [rbp + 0x974], 0x1
14146fa4f: 48 8d 15 32 49 7f 01        	lea	rdx, [rip + 0x17f4932]  # 0x142c64388
14146fa56: 48 8d 8d b0 05 00 00        	lea	rcx, [rbp + 0x5b0]
14146fa5d: e8 2e 99 46 01              	call	0x1428d9390 <.text+0x28d8390>
14146fa62: eb 0c                       	jmp	0x14146fa70 <.text+0x146ea70>
14146fa64: 48 8d 0d a5 47 7f 01        	lea	rcx, [rip + 0x17f47a5]  # 0x142c64210
14146fa6b: e8 a0 98 46 01              	call	0x1428d9310 <.text+0x28d8310>
14146fa70: 0f 0b                       	ud2
14146fa72: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nop	word ptr cs:[rax + rax]
14146fa80: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14146fa85: 55                          	push	rbp
14146fa86: 41 57                       	push	r15
14146fa88: 41 56                       	push	r14
14146fa8a: 41 55                       	push	r13
14146fa8c: 41 54                       	push	r12
14146fa8e: 56                          	push	rsi
14146fa8f: 57                          	push	rdi
14146fa90: 53                          	push	rbx
14146fa91: 48 83 ec 78                 	sub	rsp, 0x78
14146fa95: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14146fa9c: 44 0f 29 54 24 20           	movaps	xmmword ptr [rsp + 0x20], xmm10
14146faa2: 44 0f 29 4c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm9
14146faa8: 44 0f 29 44 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm8
14146faae: 0f 29 7c 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm7
14146fab3: 0f 29 74 24 60              	movaps	xmmword ptr [rsp + 0x60], xmm6
14146fab8: 48 8b 95 60 03 00 00        	mov	rdx, qword ptr [rbp + 0x360]
14146fabf: 48 85 d2                    	test	rdx, rdx
14146fac2: 74 12                       	je	0x14146fad6 <.text+0x146ead6>
14146fac4: 48 8b 8d 68 03 00 00        	mov	rcx, qword ptr [rbp + 0x368]
14146facb: 41 b8 01 00 00 00           	mov	r8d, 0x1
14146fad1: e8 4a 41 1a ff              	call	0x140613c20 <.text+0x612c20>
14146fad6: 48 b8 00 00 00 00 00 00 00 80       	movabs	rax, -0x8000000000000000
14146fae0: 48 89 85 30 09 00 00        	mov	qword ptr [rbp + 0x930], rax
14146fae7: c6 85 76 09 00 00 01        	mov	byte ptr [rbp + 0x976], 0x1
14146faee: 0f 28 74 24 60              	movaps	xmm6, xmmword ptr [rsp + 0x60]
14146faf3: 0f 28 7c 24 50              	movaps	xmm7, xmmword ptr [rsp + 0x50]
14146faf8: 44 0f 28 44 24 40           	movaps	xmm8, xmmword ptr [rsp + 0x40]
14146fafe: 44 0f 28 4c 24 30           	movaps	xmm9, xmmword ptr [rsp + 0x30]
14146fb04: 44 0f 28 54 24 20           	movaps	xmm10, xmmword ptr [rsp + 0x20]
14146fb0a: 48 83 c4 78                 	add	rsp, 0x78
14146fb0e: 5b                          	pop	rbx
14146fb0f: 5f                          	pop	rdi
14146fb10: 5e                          	pop	rsi
14146fb11: 41 5c                       	pop	r12
14146fb13: 41 5d                       	pop	r13
14146fb15: 41 5e                       	pop	r14
14146fb17: 41 5f                       	pop	r15
14146fb19: 5d                          	pop	rbp
14146fb1a: c3                          	ret
14146fb1b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
14146fb20: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14146fb25: 55                          	push	rbp
14146fb26: 41 57                       	push	r15
14146fb28: 41 56                       	push	r14
14146fb2a: 41 55                       	push	r13
14146fb2c: 41 54                       	push	r12
14146fb2e: 56                          	push	rsi
14146fb2f: 57                          	push	rdi
14146fb30: 53                          	push	rbx
14146fb31: 48 83 ec 78                 	sub	rsp, 0x78
14146fb35: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14146fb3c: 44 0f 29 54 24 20           	movaps	xmmword ptr [rsp + 0x20], xmm10
14146fb42: 44 0f 29 4c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm9
14146fb48: 44 0f 29 44 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm8
14146fb4e: 0f 29 7c 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm7
14146fb53: 0f 29 74 24 60              	movaps	xmmword ptr [rsp + 0x60], xmm6
14146fb58: 48 8b 95 b0 01 00 00        	mov	rdx, qword ptr [rbp + 0x1b0]
14146fb5f: 48 85 d2                    	test	rdx, rdx
14146fb62: 74 12                       	je	0x14146fb76 <.text+0x146eb76>
14146fb64: 48 8b 8d b8 01 00 00        	mov	rcx, qword ptr [rbp + 0x1b8]
14146fb6b: 41 b8 01 00 00 00           	mov	r8d, 0x1
14146fb71: e8 aa 40 1a ff              	call	0x140613c20 <.text+0x612c20>
14146fb76: 0f 28 74 24 60              	movaps	xmm6, xmmword ptr [rsp + 0x60]
14146fb7b: 0f 28 7c 24 50              	movaps	xmm7, xmmword ptr [rsp + 0x50]
14146fb80: 44 0f 28 44 24 40           	movaps	xmm8, xmmword ptr [rsp + 0x40]
14146fb86: 44 0f 28 4c 24 30           	movaps	xmm9, xmmword ptr [rsp + 0x30]
14146fb8c: 44 0f 28 54 24 20           	movaps	xmm10, xmmword ptr [rsp + 0x20]
14146fb92: 48 83 c4 78                 	add	rsp, 0x78
14146fb96: 5b                          	pop	rbx
14146fb97: 5f                          	pop	rdi
14146fb98: 5e                          	pop	rsi
14146fb99: 41 5c                       	pop	r12
14146fb9b: 41 5d                       	pop	r13
14146fb9d: 41 5e                       	pop	r14
14146fb9f: 41 5f                       	pop	r15
14146fba1: 5d                          	pop	rbp
14146fba2: c3                          	ret
14146fba3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
14146fbb0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14146fbb5: 55                          	push	rbp
14146fbb6: 41 57                       	push	r15
14146fbb8: 41 56                       	push	r14
14146fbba: 41 55                       	push	r13
14146fbbc: 41 54                       	push	r12
14146fbbe: 56                          	push	rsi
14146fbbf: 57                          	push	rdi
14146fbc0: 53                          	push	rbx
14146fbc1: 48 83 ec 78                 	sub	rsp, 0x78
14146fbc5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14146fbcc: 44 0f 29 54 24 20           	movaps	xmmword ptr [rsp + 0x20], xmm10
14146fbd2: 44 0f 29 4c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm9
14146fbd8: 44 0f 29 44 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm8
14146fbde: 0f 29 7c 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm7
14146fbe3: 0f 29 74 24 60              	movaps	xmmword ptr [rsp + 0x60], xmm6
14146fbe8: 48 8b 8d 38 09 00 00        	mov	rcx, qword ptr [rbp + 0x938]
14146fbef: 48 8b 85 48 09 00 00        	mov	rax, qword ptr [rbp + 0x948]
14146fbf6: 48 8b 50 08                 	mov	rdx, qword ptr [rax + 0x8]
14146fbfa: 48 85 d2                    	test	rdx, rdx
14146fbfd: 74 10                       	je	0x14146fc0f <.text+0x146ec0f>
14146fbff: 48 8b 85 48 09 00 00        	mov	rax, qword ptr [rbp + 0x948]
14146fc06: 4c 8b 40 10                 	mov	r8, qword ptr [rax + 0x10]
14146fc0a: e8 11 40 1a ff              	call	0x140613c20 <.text+0x612c20>
14146fc0f: ba 18 00 00 00              	mov	edx, 0x18
14146fc14: 41 b8 08 00 00 00           	mov	r8d, 0x8
14146fc1a: 48 8b 8d 10 09 00 00        	mov	rcx, qword ptr [rbp + 0x910]
14146fc21: e8 fa 3f 1a ff              	call	0x140613c20 <.text+0x612c20>
14146fc26: 0f 28 74 24 60              	movaps	xmm6, xmmword ptr [rsp + 0x60]
14146fc2b: 0f 28 7c 24 50              	movaps	xmm7, xmmword ptr [rsp + 0x50]
14146fc30: 44 0f 28 44 24 40           	movaps	xmm8, xmmword ptr [rsp + 0x40]
14146fc36: 44 0f 28 4c 24 30           	movaps	xmm9, xmmword ptr [rsp + 0x30]
14146fc3c: 44 0f 28 54 24 20           	movaps	xmm10, xmmword ptr [rsp + 0x20]
14146fc42: 48 83 c4 78                 	add	rsp, 0x78
14146fc46: 5b                          	pop	rbx
14146fc47: 5f                          	pop	rdi
14146fc48: 5e                          	pop	rsi
14146fc49: 41 5c                       	pop	r12
14146fc4b: 41 5d                       	pop	r13
14146fc4d: 41 5e                       	pop	r14
14146fc4f: 41 5f                       	pop	r15
14146fc51: 5d                          	pop	rbp
14146fc52: c3                          	ret
14146fc53: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
14146fc60: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14146fc65: 55                          	push	rbp
14146fc66: 41 57                       	push	r15
14146fc68: 41 56                       	push	r14
14146fc6a: 41 55                       	push	r13
14146fc6c: 41 54                       	push	r12
14146fc6e: 56                          	push	rsi
14146fc6f: 57                          	push	rdi
14146fc70: 53                          	push	rbx
14146fc71: 48 83 ec 78                 	sub	rsp, 0x78
14146fc75: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14146fc7c: 44 0f 29 54 24 20           	movaps	xmmword ptr [rsp + 0x20], xmm10
14146fc82: 44 0f 29 4c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm9
14146fc88: 44 0f 29 44 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm8
14146fc8e: 0f 29 7c 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm7
14146fc93: 0f 29 74 24 60              	movaps	xmmword ptr [rsp + 0x60], xmm6
14146fc98: 48 8b 55 b0                 	mov	rdx, qword ptr [rbp - 0x50]
14146fc9c: 48 85 d2                    	test	rdx, rdx
14146fc9f: 74 0f                       	je	0x14146fcb0 <.text+0x146ecb0>
14146fca1: 48 8b 4d b8                 	mov	rcx, qword ptr [rbp - 0x48]
14146fca5: 41 b8 01 00 00 00           	mov	r8d, 0x1
14146fcab: e8 70 3f 1a ff              	call	0x140613c20 <.text+0x612c20>
14146fcb0: 48 b8 00 00 00 00 00 00 00 80       	movabs	rax, -0x8000000000000000
14146fcba: 48 89 85 30 09 00 00        	mov	qword ptr [rbp + 0x930], rax
14146fcc1: c6 85 76 09 00 00 01        	mov	byte ptr [rbp + 0x976], 0x1
14146fcc8: 0f 28 74 24 60              	movaps	xmm6, xmmword ptr [rsp + 0x60]
14146fccd: 0f 28 7c 24 50              	movaps	xmm7, xmmword ptr [rsp + 0x50]
14146fcd2: 44 0f 28 44 24 40           	movaps	xmm8, xmmword ptr [rsp + 0x40]
14146fcd8: 44 0f 28 4c 24 30           	movaps	xmm9, xmmword ptr [rsp + 0x30]
14146fcde: 44 0f 28 54 24 20           	movaps	xmm10, xmmword ptr [rsp + 0x20]
14146fce4: 48 83 c4 78                 	add	rsp, 0x78
14146fce8: 5b                          	pop	rbx
14146fce9: 5f                          	pop	rdi
14146fcea: 5e                          	pop	rsi
14146fceb: 41 5c                       	pop	r12
14146fced: 41 5d                       	pop	r13
14146fcef: 41 5e                       	pop	r14
14146fcf1: 41 5f                       	pop	r15
14146fcf3: 5d                          	pop	rbp
14146fcf4: c3                          	ret
14146fcf5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
14146fd00: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14146fd05: 55                          	push	rbp
14146fd06: 41 57                       	push	r15
14146fd08: 41 56                       	push	r14
14146fd0a: 41 55                       	push	r13
14146fd0c: 41 54                       	push	r12
14146fd0e: 56                          	push	rsi
14146fd0f: 57                          	push	rdi
14146fd10: 53                          	push	rbx
14146fd11: 48 83 ec 78                 	sub	rsp, 0x78
14146fd15: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14146fd1c: 44 0f 29 54 24 20           	movaps	xmmword ptr [rsp + 0x20], xmm10
14146fd22: 44 0f 29 4c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm9
14146fd28: 44 0f 29 44 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm8
14146fd2e: 0f 29 7c 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm7
14146fd33: 0f 29 74 24 60              	movaps	xmmword ptr [rsp + 0x60], xmm6
14146fd38: 48 8d 8d b0 05 00 00        	lea	rcx, [rbp + 0x5b0]
14146fd3f: e8 bc fc 41 ff              	call	0x14088fa00 <.text+0x88ea00>
14146fd44: 0f 28 74 24 60              	movaps	xmm6, xmmword ptr [rsp + 0x60]
14146fd49: 0f 28 7c 24 50              	movaps	xmm7, xmmword ptr [rsp + 0x50]
14146fd4e: 44 0f 28 44 24 40           	movaps	xmm8, xmmword ptr [rsp + 0x40]
14146fd54: 44 0f 28 4c 24 30           	movaps	xmm9, xmmword ptr [rsp + 0x30]
14146fd5a: 44 0f 28 54 24 20           	movaps	xmm10, xmmword ptr [rsp + 0x20]
14146fd60: 48 83 c4 78                 	add	rsp, 0x78
14146fd64: 5b                          	pop	rbx
14146fd65: 5f                          	pop	rdi
14146fd66: 5e                          	pop	rsi
14146fd67: 41 5c                       	pop	r12
14146fd69: 41 5d                       	pop	r13
14146fd6b: 41 5e                       	pop	r14
14146fd6d: 41 5f                       	pop	r15
14146fd6f: 5d                          	pop	rbp
14146fd70: c3                          	ret
14146fd71: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
14146fd80: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14146fd85: 55                          	push	rbp
14146fd86: 41 57                       	push	r15
14146fd88: 41 56                       	push	r14
14146fd8a: 41 55                       	push	r13
14146fd8c: 41 54                       	push	r12
14146fd8e: 56                          	push	rsi
14146fd8f: 57                          	push	rdi
14146fd90: 53                          	push	rbx
14146fd91: 48 83 ec 78                 	sub	rsp, 0x78
14146fd95: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14146fd9c: 44 0f 29 54 24 20           	movaps	xmmword ptr [rsp + 0x20], xmm10
14146fda2: 44 0f 29 4c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm9
14146fda8: 44 0f 29 44 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm8
14146fdae: 0f 29 7c 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm7
14146fdb3: 0f 29 74 24 60              	movaps	xmmword ptr [rsp + 0x60], xmm6
14146fdb8: 48 8d 8d 68 08 00 00        	lea	rcx, [rbp + 0x868]
14146fdbf: e8 6c ce 8d ff              	call	0x140d4cc30 <.text+0xd4bc30>
14146fdc4: 0f 28 74 24 60              	movaps	xmm6, xmmword ptr [rsp + 0x60]
14146fdc9: 0f 28 7c 24 50              	movaps	xmm7, xmmword ptr [rsp + 0x50]
14146fdce: 44 0f 28 44 24 40           	movaps	xmm8, xmmword ptr [rsp + 0x40]
14146fdd4: 44 0f 28 4c 24 30           	movaps	xmm9, xmmword ptr [rsp + 0x30]
14146fdda: 44 0f 28 54 24 20           	movaps	xmm10, xmmword ptr [rsp + 0x20]
14146fde0: 48 83 c4 78                 	add	rsp, 0x78
14146fde4: 5b                          	pop	rbx
14146fde5: 5f                          	pop	rdi
14146fde6: 5e                          	pop	rsi
14146fde7: 41 5c                       	pop	r12
14146fde9: 41 5d                       	pop	r13
14146fdeb: 41 5e                       	pop	r14
14146fded: 41 5f                       	pop	r15
14146fdef: 5d                          	pop	rbp
14146fdf0: c3                          	ret
14146fdf1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
14146fe00: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14146fe05: 55                          	push	rbp
14146fe06: 41 57                       	push	r15
14146fe08: 41 56                       	push	r14
14146fe0a: 41 55                       	push	r13
14146fe0c: 41 54                       	push	r12
14146fe0e: 56                          	push	rsi
14146fe0f: 57                          	push	rdi
14146fe10: 53                          	push	rbx
14146fe11: 48 83 ec 78                 	sub	rsp, 0x78
14146fe15: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14146fe1c: 44 0f 29 54 24 20           	movaps	xmmword ptr [rsp + 0x20], xmm10
14146fe22: 44 0f 29 4c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm9
14146fe28: 44 0f 29 44 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm8
14146fe2e: 0f 29 7c 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm7
14146fe33: 0f 29 74 24 60              	movaps	xmmword ptr [rsp + 0x60], xmm6
14146fe38: 48 8b 95 80 08 00 00        	mov	rdx, qword ptr [rbp + 0x880]
14146fe3f: 48 85 d2                    	test	rdx, rdx
14146fe42: 74 12                       	je	0x14146fe56 <.text+0x146ee56>
14146fe44: 48 8b 8d 88 08 00 00        	mov	rcx, qword ptr [rbp + 0x888]
14146fe4b: 41 b8 01 00 00 00           	mov	r8d, 0x1
14146fe51: e8 ca 3d 1a ff              	call	0x140613c20 <.text+0x612c20>
14146fe56: 0f 28 74 24 60              	movaps	xmm6, xmmword ptr [rsp + 0x60]
14146fe5b: 0f 28 7c 24 50              	movaps	xmm7, xmmword ptr [rsp + 0x50]
14146fe60: 44 0f 28 44 24 40           	movaps	xmm8, xmmword ptr [rsp + 0x40]
14146fe66: 44 0f 28 4c 24 30           	movaps	xmm9, xmmword ptr [rsp + 0x30]
14146fe6c: 44 0f 28 54 24 20           	movaps	xmm10, xmmword ptr [rsp + 0x20]
14146fe72: 48 83 c4 78                 	add	rsp, 0x78
14146fe76: 5b                          	pop	rbx
14146fe77: 5f                          	pop	rdi
14146fe78: 5e                          	pop	rsi
14146fe79: 41 5c                       	pop	r12
14146fe7b: 41 5d                       	pop	r13
14146fe7d: 41 5e                       	pop	r14
14146fe7f: 41 5f                       	pop	r15
14146fe81: 5d                          	pop	rbp
14146fe82: c3                          	ret
14146fe83: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
14146fe90: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14146fe95: 55                          	push	rbp
14146fe96: 41 57                       	push	r15
14146fe98: 41 56                       	push	r14
14146fe9a: 41 55                       	push	r13
14146fe9c: 41 54                       	push	r12
14146fe9e: 56                          	push	rsi
14146fe9f: 57                          	push	rdi
14146fea0: 53                          	push	rbx
14146fea1: 48 83 ec 78                 	sub	rsp, 0x78
14146fea5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14146feac: 44 0f 29 54 24 20           	movaps	xmmword ptr [rsp + 0x20], xmm10
14146feb2: 44 0f 29 4c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm9
14146feb8: 44 0f 29 44 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm8
14146febe: 0f 29 7c 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm7
14146fec3: 0f 29 74 24 60              	movaps	xmmword ptr [rsp + 0x60], xmm6
14146fec8: 48 8d 8d b0 01 00 00        	lea	rcx, [rbp + 0x1b0]
14146fecf: e8 1c b4 40 ff              	call	0x14087b2f0 <.text+0x87a2f0>
14146fed4: c6 85 6e 09 00 00 01        	mov	byte ptr [rbp + 0x96e], 0x1
14146fedb: 0f 28 74 24 60              	movaps	xmm6, xmmword ptr [rsp + 0x60]
14146fee0: 0f 28 7c 24 50              	movaps	xmm7, xmmword ptr [rsp + 0x50]
14146fee5: 44 0f 28 44 24 40           	movaps	xmm8, xmmword ptr [rsp + 0x40]
14146feeb: 44 0f 28 4c 24 30           	movaps	xmm9, xmmword ptr [rsp + 0x30]
14146fef1: 44 0f 28 54 24 20           	movaps	xmm10, xmmword ptr [rsp + 0x20]
14146fef7: 48 83 c4 78                 	add	rsp, 0x78
14146fefb: 5b                          	pop	rbx
14146fefc: 5f                          	pop	rdi
14146fefd: 5e                          	pop	rsi
14146fefe: 41 5c                       	pop	r12
14146ff00: 41 5d                       	pop	r13
14146ff02: 41 5e                       	pop	r14
14146ff04: 41 5f                       	pop	r15
14146ff06: 5d                          	pop	rbp
14146ff07: c3                          	ret
14146ff08: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
14146ff10: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14146ff15: 55                          	push	rbp
14146ff16: 41 57                       	push	r15
14146ff18: 41 56                       	push	r14
14146ff1a: 41 55                       	push	r13
14146ff1c: 41 54                       	push	r12
14146ff1e: 56                          	push	rsi
14146ff1f: 57                          	push	rdi
14146ff20: 53                          	push	rbx
14146ff21: 48 83 ec 78                 	sub	rsp, 0x78
14146ff25: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14146ff2c: 44 0f 29 54 24 20           	movaps	xmmword ptr [rsp + 0x20], xmm10
14146ff32: 44 0f 29 4c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm9
14146ff38: 44 0f 29 44 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm8
14146ff3e: 0f 29 7c 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm7
14146ff43: 0f 29 74 24 60              	movaps	xmmword ptr [rsp + 0x60], xmm6
14146ff48: 48 8d 8d b0 05 00 00        	lea	rcx, [rbp + 0x5b0]
14146ff4f: e8 3c 89 eb ff              	call	0x141328890 <.text+0x1327890>
14146ff54: 0f 28 74 24 60              	movaps	xmm6, xmmword ptr [rsp + 0x60]
14146ff59: 0f 28 7c 24 50              	movaps	xmm7, xmmword ptr [rsp + 0x50]
14146ff5e: 44 0f 28 44 24 40           	movaps	xmm8, xmmword ptr [rsp + 0x40]
14146ff64: 44 0f 28 4c 24 30           	movaps	xmm9, xmmword ptr [rsp + 0x30]
14146ff6a: 44 0f 28 54 24 20           	movaps	xmm10, xmmword ptr [rsp + 0x20]
14146ff70: 48 83 c4 78                 	add	rsp, 0x78
14146ff74: 5b                          	pop	rbx
14146ff75: 5f                          	pop	rdi
14146ff76: 5e                          	pop	rsi
14146ff77: 41 5c                       	pop	r12
14146ff79: 41 5d                       	pop	r13
14146ff7b: 41 5e                       	pop	r14
14146ff7d: 41 5f                       	pop	r15
14146ff7f: 5d                          	pop	rbp
14146ff80: c3                          	ret
14146ff81: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
14146ff90: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14146ff95: 55                          	push	rbp
14146ff96: 41 57                       	push	r15
14146ff98: 41 56                       	push	r14
14146ff9a: 41 55                       	push	r13
14146ff9c: 41 54                       	push	r12
14146ff9e: 56                          	push	rsi
14146ff9f: 57                          	push	rdi
14146ffa0: 53                          	push	rbx
14146ffa1: 48 83 ec 78                 	sub	rsp, 0x78
14146ffa5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14146ffac: 44 0f 29 54 24 20           	movaps	xmmword ptr [rsp + 0x20], xmm10
14146ffb2: 44 0f 29 4c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm9
14146ffb8: 44 0f 29 44 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm8
14146ffbe: 0f 29 7c 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm7
14146ffc3: 0f 29 74 24 60              	movaps	xmmword ptr [rsp + 0x60], xmm6
14146ffc8: 48 8d 8d b0 05 00 00        	lea	rcx, [rbp + 0x5b0]
14146ffcf: e8 bc 88 eb ff              	call	0x141328890 <.text+0x1327890>
14146ffd4: 0f 28 74 24 60              	movaps	xmm6, xmmword ptr [rsp + 0x60]
14146ffd9: 0f 28 7c 24 50              	movaps	xmm7, xmmword ptr [rsp + 0x50]
14146ffde: 44 0f 28 44 24 40           	movaps	xmm8, xmmword ptr [rsp + 0x40]
14146ffe4: 44 0f 28 4c 24 30           	movaps	xmm9, xmmword ptr [rsp + 0x30]
14146ffea: 44 0f 28 54 24 20           	movaps	xmm10, xmmword ptr [rsp + 0x20]
14146fff0: 48 83 c4 78                 	add	rsp, 0x78
14146fff4: 5b                          	pop	rbx
14146fff5: 5f                          	pop	rdi
14146fff6: 5e                          	pop	rsi
14146fff7: 41 5c                       	pop	r12
14146fff9: 41 5d                       	pop	r13
14146fffb: 41 5e                       	pop	r14
14146fffd: 41 5f                       	pop	r15
14146ffff: 5d                          	pop	rbp
141470000: c3                          	ret
141470001: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
141470010: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141470015: 55                          	push	rbp
141470016: 41 57                       	push	r15
141470018: 41 56                       	push	r14
14147001a: 41 55                       	push	r13
14147001c: 41 54                       	push	r12
14147001e: 56                          	push	rsi
14147001f: 57                          	push	rdi
141470020: 53                          	push	rbx
141470021: 48 83 ec 78                 	sub	rsp, 0x78
141470025: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14147002c: 44 0f 29 54 24 20           	movaps	xmmword ptr [rsp + 0x20], xmm10
141470032: 44 0f 29 4c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm9
141470038: 44 0f 29 44 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm8
14147003e: 0f 29 7c 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm7
141470043: 0f 29 74 24 60              	movaps	xmmword ptr [rsp + 0x60], xmm6
141470048: 80 bd 6e 09 00 00 00        	cmp	byte ptr [rbp + 0x96e], 0x0
14147004f: 74 40                       	je	0x141470091 <.text+0x146f091>
141470051: 48 8d 8d 60 03 00 00        	lea	rcx, [rbp + 0x360]
141470058: e8 a3 c3 41 ff              	call	0x14088c400 <.text+0x88b400>
14147005d: c6 85 6f 09 00 00 00        	mov	byte ptr [rbp + 0x96f], 0x0
141470064: 0f 28 74 24 60              	movaps	xmm6, xmmword ptr [rsp + 0x60]
141470069: 0f 28 7c 24 50              	movaps	xmm7, xmmword ptr [rsp + 0x50]
14147006e: 44 0f 28 44 24 40           	movaps	xmm8, xmmword ptr [rsp + 0x40]
141470074: 44 0f 28 4c 24 30           	movaps	xmm9, xmmword ptr [rsp + 0x30]
14147007a: 44 0f 28 54 24 20           	movaps	xmm10, xmmword ptr [rsp + 0x20]
141470080: 48 83 c4 78                 	add	rsp, 0x78
141470084: 5b                          	pop	rbx
141470085: 5f                          	pop	rdi
141470086: 5e                          	pop	rsi
141470087: 41 5c                       	pop	r12
141470089: 41 5d                       	pop	r13
14147008b: 41 5e                       	pop	r14
14147008d: 41 5f                       	pop	r15
14147008f: 5d                          	pop	rbp
141470090: c3                          	ret
141470091: c6 85 6f 09 00 00 00        	mov	byte ptr [rbp + 0x96f], 0x0
141470098: 0f 28 74 24 60              	movaps	xmm6, xmmword ptr [rsp + 0x60]
14147009d: 0f 28 7c 24 50              	movaps	xmm7, xmmword ptr [rsp + 0x50]
1414700a2: 44 0f 28 44 24 40           	movaps	xmm8, xmmword ptr [rsp + 0x40]
1414700a8: 44 0f 28 4c 24 30           	movaps	xmm9, xmmword ptr [rsp + 0x30]
1414700ae: 44 0f 28 54 24 20           	movaps	xmm10, xmmword ptr [rsp + 0x20]
1414700b4: 48 83 c4 78                 	add	rsp, 0x78
1414700b8: 5b                          	pop	rbx
1414700b9: 5f                          	pop	rdi
1414700ba: 5e                          	pop	rsi
1414700bb: 41 5c                       	pop	r12
1414700bd: 41 5d                       	pop	r13
1414700bf: 41 5e                       	pop	r14
1414700c1: 41 5f                       	pop	r15
1414700c3: 5d                          	pop	rbp
1414700c4: c3                          	ret
1414700c5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
1414700d0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1414700d5: 55                          	push	rbp
1414700d6: 41 57                       	push	r15
1414700d8: 41 56                       	push	r14
1414700da: 41 55                       	push	r13
1414700dc: 41 54                       	push	r12
1414700de: 56                          	push	rsi
1414700df: 57                          	push	rdi
1414700e0: 53                          	push	rbx
1414700e1: 48 83 ec 78                 	sub	rsp, 0x78
1414700e5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1414700ec: 44 0f 29 54 24 20           	movaps	xmmword ptr [rsp + 0x20], xmm10
1414700f2: 44 0f 29 4c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm9
1414700f8: 44 0f 29 44 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm8
1414700fe: 0f 29 7c 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm7
141470103: 0f 29 74 24 60              	movaps	xmmword ptr [rsp + 0x60], xmm6
141470108: 80 bd 6f 09 00 00 00        	cmp	byte ptr [rbp + 0x96f], 0x0
14147010f: 74 39                       	je	0x14147014a <.text+0x146f14a>
141470111: 48 8d 8d 30 08 00 00        	lea	rcx, [rbp + 0x830]
141470118: e8 13 69 40 ff              	call	0x140876a30 <.text+0x875a30>
14147011d: 0f 28 74 24 60              	movaps	xmm6, xmmword ptr [rsp + 0x60]
141470122: 0f 28 7c 24 50              	movaps	xmm7, xmmword ptr [rsp + 0x50]
141470127: 44 0f 28 44 24 40           	movaps	xmm8, xmmword ptr [rsp + 0x40]
14147012d: 44 0f 28 4c 24 30           	movaps	xmm9, xmmword ptr [rsp + 0x30]
141470133: 44 0f 28 54 24 20           	movaps	xmm10, xmmword ptr [rsp + 0x20]
141470139: 48 83 c4 78                 	add	rsp, 0x78
14147013d: 5b                          	pop	rbx
14147013e: 5f                          	pop	rdi
14147013f: 5e                          	pop	rsi
141470140: 41 5c                       	pop	r12
141470142: 41 5d                       	pop	r13
141470144: 41 5e                       	pop	r14
141470146: 41 5f                       	pop	r15
141470148: 5d                          	pop	rbp
141470149: c3                          	ret
14147014a: 0f 28 74 24 60              	movaps	xmm6, xmmword ptr [rsp + 0x60]
14147014f: 0f 28 7c 24 50              	movaps	xmm7, xmmword ptr [rsp + 0x50]
141470154: 44 0f 28 44 24 40           	movaps	xmm8, xmmword ptr [rsp + 0x40]
14147015a: 44 0f 28 4c 24 30           	movaps	xmm9, xmmword ptr [rsp + 0x30]
141470160: 44 0f 28 54 24 20           	movaps	xmm10, xmmword ptr [rsp + 0x20]
141470166: 48 83 c4 78                 	add	rsp, 0x78
14147016a: 5b                          	pop	rbx
14147016b: 5f                          	pop	rdi
14147016c: 5e                          	pop	rsi
14147016d: 41 5c                       	pop	r12
14147016f: 41 5d                       	pop	r13
141470171: 41 5e                       	pop	r14
141470173: 41 5f                       	pop	r15
141470175: 5d                          	pop	rbp
141470176: c3                          	ret
141470177: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
141470180: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141470185: 55                          	push	rbp
141470186: 41 57                       	push	r15
141470188: 41 56                       	push	r14
14147018a: 41 55                       	push	r13
14147018c: 41 54                       	push	r12
14147018e: 56                          	push	rsi
14147018f: 57                          	push	rdi
141470190: 53                          	push	rbx
141470191: 48 83 ec 78                 	sub	rsp, 0x78
141470195: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14147019c: 44 0f 29 54 24 20           	movaps	xmmword ptr [rsp + 0x20], xmm10
1414701a2: 44 0f 29 4c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm9
1414701a8: 44 0f 29 44 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm8
1414701ae: 0f 29 7c 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm7
1414701b3: 0f 29 74 24 60              	movaps	xmmword ptr [rsp + 0x60], xmm6
1414701b8: 44 0f b6 b5 70 09 00 00     	movzx	r14d, byte ptr [rbp + 0x970]
1414701c0: 0f b6 9d 71 09 00 00        	movzx	ebx, byte ptr [rbp + 0x971]
1414701c7: 48 8b b5 a0 08 00 00        	mov	rsi, qword ptr [rbp + 0x8a0]
1414701ce: 48 8b bd d0 08 00 00        	mov	rdi, qword ptr [rbp + 0x8d0]
1414701d5: 48 8b 95 30 08 00 00        	mov	rdx, qword ptr [rbp + 0x830]
1414701dc: 48 85 d2                    	test	rdx, rdx
1414701df: 74 12                       	je	0x1414701f3 <.text+0x146f1f3>
1414701e1: 48 8b 8d 38 08 00 00        	mov	rcx, qword ptr [rbp + 0x838]
1414701e8: 41 b8 01 00 00 00           	mov	r8d, 0x1
1414701ee: e8 2d 3a 1a ff              	call	0x140613c20 <.text+0x612c20>
1414701f3: 41 f6 c6 01                 	test	r14b, 0x1
1414701f7: 74 66                       	je	0x14147025f <.text+0x146f25f>
1414701f9: 4c 8b b5 48 09 00 00        	mov	r14, qword ptr [rbp + 0x948]
141470200: 4c 89 f1                    	mov	rcx, r14
141470203: e8 38 e9 41 ff              	call	0x14088eb40 <.text+0x88db40>
141470208: ba f0 00 00 00              	mov	edx, 0xf0
14147020d: 41 b8 08 00 00 00           	mov	r8d, 0x8
141470213: 4c 89 f1                    	mov	rcx, r14
141470216: e8 05 3a 1a ff              	call	0x140613c20 <.text+0x612c20>
14147021b: 48 89 bd 30 09 00 00        	mov	qword ptr [rbp + 0x930], rdi
141470222: 48 89 b5 48 05 00 00        	mov	qword ptr [rbp + 0x548], rsi
141470229: 80 e3 01                    	and	bl, 0x1
14147022c: 88 9d 76 09 00 00           	mov	byte ptr [rbp + 0x976], bl
141470232: 0f 28 74 24 60              	movaps	xmm6, xmmword ptr [rsp + 0x60]
141470237: 0f 28 7c 24 50              	movaps	xmm7, xmmword ptr [rsp + 0x50]
14147023c: 44 0f 28 44 24 40           	movaps	xmm8, xmmword ptr [rsp + 0x40]
141470242: 44 0f 28 4c 24 30           	movaps	xmm9, xmmword ptr [rsp + 0x30]
141470248: 44 0f 28 54 24 20           	movaps	xmm10, xmmword ptr [rsp + 0x20]
14147024e: 48 83 c4 78                 	add	rsp, 0x78
141470252: 5b                          	pop	rbx
141470253: 5f                          	pop	rdi
141470254: 5e                          	pop	rsi
141470255: 41 5c                       	pop	r12
141470257: 41 5d                       	pop	r13
141470259: 41 5e                       	pop	r14
14147025b: 41 5f                       	pop	r15
14147025d: 5d                          	pop	rbp
14147025e: c3                          	ret
14147025f: 48 89 bd 30 09 00 00        	mov	qword ptr [rbp + 0x930], rdi
141470266: 48 89 b5 48 05 00 00        	mov	qword ptr [rbp + 0x548], rsi
14147026d: 80 e3 01                    	and	bl, 0x1
141470270: 88 9d 76 09 00 00           	mov	byte ptr [rbp + 0x976], bl
141470276: 0f 28 74 24 60              	movaps	xmm6, xmmword ptr [rsp + 0x60]
14147027b: 0f 28 7c 24 50              	movaps	xmm7, xmmword ptr [rsp + 0x50]
141470280: 44 0f 28 44 24 40           	movaps	xmm8, xmmword ptr [rsp + 0x40]
141470286: 44 0f 28 4c 24 30           	movaps	xmm9, xmmword ptr [rsp + 0x30]
14147028c: 44 0f 28 54 24 20           	movaps	xmm10, xmmword ptr [rsp + 0x20]
141470292: 48 83 c4 78                 	add	rsp, 0x78
141470296: 5b                          	pop	rbx
141470297: 5f                          	pop	rdi
141470298: 5e                          	pop	rsi
141470299: 41 5c                       	pop	r12
14147029b: 41 5d                       	pop	r13
14147029d: 41 5e                       	pop	r14
14147029f: 41 5f                       	pop	r15
1414702a1: 5d                          	pop	rbp
1414702a2: c3                          	ret
1414702a3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
1414702b0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1414702b5: 55                          	push	rbp
1414702b6: 41 57                       	push	r15
1414702b8: 41 56                       	push	r14
1414702ba: 41 55                       	push	r13
1414702bc: 41 54                       	push	r12
1414702be: 56                          	push	rsi
1414702bf: 57                          	push	rdi
1414702c0: 53                          	push	rbx
1414702c1: 48 83 ec 78                 	sub	rsp, 0x78
1414702c5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1414702cc: 44 0f 29 54 24 20           	movaps	xmmword ptr [rsp + 0x20], xmm10
1414702d2: 44 0f 29 4c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm9
1414702d8: 44 0f 29 44 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm8
1414702de: 0f 29 7c 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm7
1414702e3: 0f 29 74 24 60              	movaps	xmmword ptr [rsp + 0x60], xmm6
1414702e8: 48 8b bd 50 09 00 00        	mov	rdi, qword ptr [rbp + 0x950]
1414702ef: 48 89 bd 40 09 00 00        	mov	qword ptr [rbp + 0x940], rdi
1414702f6: 48 8b b5 60 09 00 00        	mov	rsi, qword ptr [rbp + 0x960]
1414702fd: 48 89 b5 20 09 00 00        	mov	qword ptr [rbp + 0x920], rsi
141470304: c6 85 77 09 00 00 00        	mov	byte ptr [rbp + 0x977], 0x0
14147030b: 48 8d 8d b0 01 00 00        	lea	rcx, [rbp + 0x1b0]
141470312: e8 49 4a 42 ff              	call	0x140894d60 <.text+0x893d60>
141470317: 48 89 bd 40 09 00 00        	mov	qword ptr [rbp + 0x940], rdi
14147031e: 48 89 b5 20 09 00 00        	mov	qword ptr [rbp + 0x920], rsi
141470325: c6 85 77 09 00 00 00        	mov	byte ptr [rbp + 0x977], 0x0
14147032c: 0f 28 74 24 60              	movaps	xmm6, xmmword ptr [rsp + 0x60]
141470331: 0f 28 7c 24 50              	movaps	xmm7, xmmword ptr [rsp + 0x50]
141470336: 44 0f 28 44 24 40           	movaps	xmm8, xmmword ptr [rsp + 0x40]
14147033c: 44 0f 28 4c 24 30           	movaps	xmm9, xmmword ptr [rsp + 0x30]
141470342: 44 0f 28 54 24 20           	movaps	xmm10, xmmword ptr [rsp + 0x20]
141470348: 48 83 c4 78                 	add	rsp, 0x78
14147034c: 5b                          	pop	rbx
14147034d: 5f                          	pop	rdi
14147034e: 5e                          	pop	rsi
14147034f: 41 5c                       	pop	r12
141470351: 41 5d                       	pop	r13
141470353: 41 5e                       	pop	r14
141470355: 41 5f                       	pop	r15
141470357: 5d                          	pop	rbp
141470358: c3                          	ret
141470359: 0f 1f 80 00 00 00 00        	nop	dword ptr [rax]
141470360: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141470365: 55                          	push	rbp
141470366: 41 57                       	push	r15
141470368: 41 56                       	push	r14
14147036a: 41 55                       	push	r13
14147036c: 41 54                       	push	r12
14147036e: 56                          	push	rsi
14147036f: 57                          	push	rdi
141470370: 53                          	push	rbx
141470371: 48 83 ec 78                 	sub	rsp, 0x78
141470375: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14147037c: 44 0f 29 54 24 20           	movaps	xmmword ptr [rsp + 0x20], xmm10
141470382: 44 0f 29 4c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm9
141470388: 44 0f 29 44 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm8
14147038e: 0f 29 7c 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm7
141470393: 0f 29 74 24 60              	movaps	xmmword ptr [rsp + 0x60], xmm6
141470398: 0f 28 85 b0 05 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x5b0]
14147039f: 0f 28 8d c0 05 00 00        	movaps	xmm1, xmmword ptr [rbp + 0x5c0]
1414703a6: 0f 28 95 d0 05 00 00        	movaps	xmm2, xmmword ptr [rbp + 0x5d0]
1414703ad: 48 8b 8d 18 09 00 00        	mov	rcx, qword ptr [rbp + 0x918]
1414703b4: 0f 11 51 20                 	movups	xmmword ptr [rcx + 0x20], xmm2
1414703b8: 0f 11 49 10                 	movups	xmmword ptr [rcx + 0x10], xmm1
1414703bc: 0f 11 01                    	movups	xmmword ptr [rcx], xmm0
1414703bf: 48 b8 00 00 00 00 00 00 00 80       	movabs	rax, -0x8000000000000000
1414703c9: 48 89 41 30                 	mov	qword ptr [rcx + 0x30], rax
1414703cd: 48 8b 85 b0 08 00 00        	mov	rax, qword ptr [rbp + 0x8b0]
1414703d4: 48 89 41 58                 	mov	qword ptr [rcx + 0x58], rax
1414703d8: 48 8b 85 b8 08 00 00        	mov	rax, qword ptr [rbp + 0x8b8]
1414703df: 48 89 41 60                 	mov	qword ptr [rcx + 0x60], rax
1414703e3: 8b 85 38 09 00 00           	mov	eax, dword ptr [rbp + 0x938]
1414703e9: 89 41 68                    	mov	dword ptr [rcx + 0x68], eax
1414703ec: 8b 85 58 09 00 00           	mov	eax, dword ptr [rbp + 0x958]
1414703f2: 89 41 6c                    	mov	dword ptr [rcx + 0x6c], eax
1414703f5: 8b 85 0c 09 00 00           	mov	eax, dword ptr [rbp + 0x90c]
1414703fb: 89 41 70                    	mov	dword ptr [rcx + 0x70], eax
1414703fe: 48 8b 85 50 09 00 00        	mov	rax, qword ptr [rbp + 0x950]
141470405: 48 89 85 40 09 00 00        	mov	qword ptr [rbp + 0x940], rax
14147040c: 48 8b 85 60 09 00 00        	mov	rax, qword ptr [rbp + 0x960]
141470413: 48 89 85 20 09 00 00        	mov	qword ptr [rbp + 0x920], rax
14147041a: c6 85 77 09 00 00 00        	mov	byte ptr [rbp + 0x977], 0x0
141470421: 0f 28 74 24 60              	movaps	xmm6, xmmword ptr [rsp + 0x60]
141470426: 0f 28 7c 24 50              	movaps	xmm7, xmmword ptr [rsp + 0x50]
14147042b: 44 0f 28 44 24 40           	movaps	xmm8, xmmword ptr [rsp + 0x40]
141470431: 44 0f 28 4c 24 30           	movaps	xmm9, xmmword ptr [rsp + 0x30]
141470437: 44 0f 28 54 24 20           	movaps	xmm10, xmmword ptr [rsp + 0x20]
14147043d: 48 83 c4 78                 	add	rsp, 0x78
141470441: 5b                          	pop	rbx
141470442: 5f                          	pop	rdi
141470443: 5e                          	pop	rsi
141470444: 41 5c                       	pop	r12
141470446: 41 5d                       	pop	r13
141470448: 41 5e                       	pop	r14
14147044a: 41 5f                       	pop	r15
14147044c: 5d                          	pop	rbp
14147044d: c3                          	ret
14147044e: 66 90                       	nop
141470450: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141470455: 55                          	push	rbp
141470456: 41 57                       	push	r15
141470458: 41 56                       	push	r14
14147045a: 41 55                       	push	r13
14147045c: 41 54                       	push	r12
14147045e: 56                          	push	rsi
14147045f: 57                          	push	rdi
141470460: 53                          	push	rbx
141470461: 48 83 ec 78                 	sub	rsp, 0x78
141470465: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14147046c: 44 0f 29 54 24 20           	movaps	xmmword ptr [rsp + 0x20], xmm10
141470472: 44 0f 29 4c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm9
141470478: 44 0f 29 44 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm8
14147047e: 0f 29 7c 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm7
141470483: 0f 29 74 24 60              	movaps	xmmword ptr [rsp + 0x60], xmm6
141470488: 48 8b bd 38 09 00 00        	mov	rdi, qword ptr [rbp + 0x938]
14147048f: 48 8b 9d 58 09 00 00        	mov	rbx, qword ptr [rbp + 0x958]
141470496: 48 39 fb                    	cmp	rbx, rdi
141470499: 48 8b b5 28 09 00 00        	mov	rsi, qword ptr [rbp + 0x928]
1414704a0: 75 3c                       	jne	0x1414704de <.text+0x146f4de>
1414704a2: c6 85 73 09 00 00 00        	mov	byte ptr [rbp + 0x973], 0x0
1414704a9: 0f 28 74 24 60              	movaps	xmm6, xmmword ptr [rsp + 0x60]
1414704ae: 0f 28 7c 24 50              	movaps	xmm7, xmmword ptr [rsp + 0x50]
1414704b3: 44 0f 28 44 24 40           	movaps	xmm8, xmmword ptr [rsp + 0x40]
1414704b9: 44 0f 28 4c 24 30           	movaps	xmm9, xmmword ptr [rsp + 0x30]
1414704bf: 44 0f 28 54 24 20           	movaps	xmm10, xmmword ptr [rsp + 0x20]
1414704c5: 48 83 c4 78                 	add	rsp, 0x78
1414704c9: 5b                          	pop	rbx
1414704ca: 5f                          	pop	rdi
1414704cb: 5e                          	pop	rsi
1414704cc: 41 5c                       	pop	r12
1414704ce: 41 5d                       	pop	r13
1414704d0: 41 5e                       	pop	r14
1414704d2: 41 5f                       	pop	r15
1414704d4: 5d                          	pop	rbp
1414704d5: c3                          	ret
1414704d6: 48 ff c3                    	inc	rbx
1414704d9: 48 39 fb                    	cmp	rbx, rdi
1414704dc: 74 c4                       	je	0x1414704a2 <.text+0x146f4a2>
1414704de: 48 89 d9                    	mov	rcx, rbx
1414704e1: 48 c1 e1 04                 	shl	rcx, 0x4
1414704e5: 48 8b 04 0e                 	mov	rax, qword ptr [rsi + rcx]
1414704e9: f0                          	lock
1414704ea: 48 ff 08                    	dec	qword ptr [rax]
1414704ed: 75 e7                       	jne	0x1414704d6 <.text+0x146f4d6>
1414704ef: 48 01 f1                    	add	rcx, rsi
1414704f2: c6 85 73 09 00 00 00        	mov	byte ptr [rbp + 0x973], 0x0
1414704f9: e8 82 58 e7 fe              	call	0x1402e5d80 <.text+0x2e4d80>
1414704fe: eb d6                       	jmp	0x1414704d6 <.text+0x146f4d6>
141470500: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141470505: 55                          	push	rbp
141470506: 41 57                       	push	r15
141470508: 41 56                       	push	r14
14147050a: 41 55                       	push	r13
14147050c: 41 54                       	push	r12
14147050e: 56                          	push	rsi
14147050f: 57                          	push	rdi
141470510: 53                          	push	rbx
141470511: 48 83 ec 78                 	sub	rsp, 0x78
141470515: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14147051c: 44 0f 29 54 24 20           	movaps	xmmword ptr [rsp + 0x20], xmm10
141470522: 44 0f 29 4c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm9
141470528: 44 0f 29 44 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm8
14147052e: 0f 29 7c 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm7
141470533: 0f 29 74 24 60              	movaps	xmmword ptr [rsp + 0x60], xmm6
141470538: 48 8d 4d b0                 	lea	rcx, [rbp - 0x50]
14147053c: e8 1f 48 42 ff              	call	0x140894d60 <.text+0x893d60>
141470541: 48 8b 85 50 09 00 00        	mov	rax, qword ptr [rbp + 0x950]
141470548: 48 89 85 40 09 00 00        	mov	qword ptr [rbp + 0x940], rax
14147054f: 48 8b 85 60 09 00 00        	mov	rax, qword ptr [rbp + 0x960]
141470556: 48 89 85 20 09 00 00        	mov	qword ptr [rbp + 0x920], rax
14147055d: c6 85 77 09 00 00 00        	mov	byte ptr [rbp + 0x977], 0x0
141470564: 0f 28 74 24 60              	movaps	xmm6, xmmword ptr [rsp + 0x60]
141470569: 0f 28 7c 24 50              	movaps	xmm7, xmmword ptr [rsp + 0x50]
14147056e: 44 0f 28 44 24 40           	movaps	xmm8, xmmword ptr [rsp + 0x40]
141470574: 44 0f 28 4c 24 30           	movaps	xmm9, xmmword ptr [rsp + 0x30]
14147057a: 44 0f 28 54 24 20           	movaps	xmm10, xmmword ptr [rsp + 0x20]
141470580: 48 83 c4 78                 	add	rsp, 0x78
141470584: 5b                          	pop	rbx
141470585: 5f                          	pop	rdi
141470586: 5e                          	pop	rsi
141470587: 41 5c                       	pop	r12
141470589: 41 5d                       	pop	r13
14147058b: 41 5e                       	pop	r14
14147058d: 41 5f                       	pop	r15
14147058f: 5d                          	pop	rbp
141470590: c3                          	ret
141470591: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
1414705a0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1414705a5: 55                          	push	rbp
1414705a6: 41 57                       	push	r15
1414705a8: 41 56                       	push	r14
1414705aa: 41 55                       	push	r13
1414705ac: 41 54                       	push	r12
1414705ae: 56                          	push	rsi
1414705af: 57                          	push	rdi
1414705b0: 53                          	push	rbx
1414705b1: 48 83 ec 78                 	sub	rsp, 0x78
1414705b5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1414705bc: 44 0f 29 54 24 20           	movaps	xmmword ptr [rsp + 0x20], xmm10
1414705c2: 44 0f 29 4c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm9
1414705c8: 44 0f 29 44 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm8
1414705ce: 0f 29 7c 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm7
1414705d3: 0f 29 74 24 60              	movaps	xmmword ptr [rsp + 0x60], xmm6
1414705d8: 48 8b 95 b0 01 00 00        	mov	rdx, qword ptr [rbp + 0x1b0]
1414705df: 48 85 d2                    	test	rdx, rdx
1414705e2: 74 16                       	je	0x1414705fa <.text+0x146f5fa>
1414705e4: 48 8b 8d b8 01 00 00        	mov	rcx, qword ptr [rbp + 0x1b8]
1414705eb: 48 c1 e2 04                 	shl	rdx, 0x4
1414705ef: 41 b8 08 00 00 00           	mov	r8d, 0x8
1414705f5: e8 26 36 1a ff              	call	0x140613c20 <.text+0x612c20>
1414705fa: 0f 28 74 24 60              	movaps	xmm6, xmmword ptr [rsp + 0x60]
1414705ff: 0f 28 7c 24 50              	movaps	xmm7, xmmword ptr [rsp + 0x50]
141470604: 44 0f 28 44 24 40           	movaps	xmm8, xmmword ptr [rsp + 0x40]
14147060a: 44 0f 28 4c 24 30           	movaps	xmm9, xmmword ptr [rsp + 0x30]
141470610: 44 0f 28 54 24 20           	movaps	xmm10, xmmword ptr [rsp + 0x20]
141470616: 48 83 c4 78                 	add	rsp, 0x78
14147061a: 5b                          	pop	rbx
14147061b: 5f                          	pop	rdi
14147061c: 5e                          	pop	rsi
14147061d: 41 5c                       	pop	r12
14147061f: 41 5d                       	pop	r13
141470621: 41 5e                       	pop	r14
141470623: 41 5f                       	pop	r15
141470625: 5d                          	pop	rbp
141470626: c3                          	ret
141470627: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
141470630: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141470635: 55                          	push	rbp
141470636: 41 57                       	push	r15
141470638: 41 56                       	push	r14
14147063a: 41 55                       	push	r13
14147063c: 41 54                       	push	r12
14147063e: 56                          	push	rsi
14147063f: 57                          	push	rdi
141470640: 53                          	push	rbx
141470641: 48 83 ec 78                 	sub	rsp, 0x78
141470645: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14147064c: 44 0f 29 54 24 20           	movaps	xmmword ptr [rsp + 0x20], xmm10
141470652: 44 0f 29 4c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm9
141470658: 44 0f 29 44 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm8
14147065e: 0f 29 7c 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm7
141470663: 0f 29 74 24 60              	movaps	xmmword ptr [rsp + 0x60], xmm6
141470668: 48 8b bd 50 09 00 00        	mov	rdi, qword ptr [rbp + 0x950]
14147066f: 48 89 bd 40 09 00 00        	mov	qword ptr [rbp + 0x940], rdi
141470676: 48 8b b5 60 09 00 00        	mov	rsi, qword ptr [rbp + 0x960]
14147067d: 48 89 b5 20 09 00 00        	mov	qword ptr [rbp + 0x920], rsi
141470684: c6 85 77 09 00 00 00        	mov	byte ptr [rbp + 0x977], 0x0
14147068b: 48 8d 8d b0 05 00 00        	lea	rcx, [rbp + 0x5b0]
141470692: e8 a9 e4 41 ff              	call	0x14088eb40 <.text+0x88db40>
141470697: 48 89 bd 40 09 00 00        	mov	qword ptr [rbp + 0x940], rdi
14147069e: 48 89 b5 20 09 00 00        	mov	qword ptr [rbp + 0x920], rsi
1414706a5: c6 85 77 09 00 00 00        	mov	byte ptr [rbp + 0x977], 0x0
1414706ac: 0f 28 74 24 60              	movaps	xmm6, xmmword ptr [rsp + 0x60]
1414706b1: 0f 28 7c 24 50              	movaps	xmm7, xmmword ptr [rsp + 0x50]
1414706b6: 44 0f 28 44 24 40           	movaps	xmm8, xmmword ptr [rsp + 0x40]
1414706bc: 44 0f 28 4c 24 30           	movaps	xmm9, xmmword ptr [rsp + 0x30]
1414706c2: 44 0f 28 54 24 20           	movaps	xmm10, xmmword ptr [rsp + 0x20]
1414706c8: 48 83 c4 78                 	add	rsp, 0x78
1414706cc: 5b                          	pop	rbx
1414706cd: 5f                          	pop	rdi
1414706ce: 5e                          	pop	rsi
1414706cf: 41 5c                       	pop	r12
1414706d1: 41 5d                       	pop	r13
1414706d3: 41 5e                       	pop	r14
1414706d5: 41 5f                       	pop	r15
1414706d7: 5d                          	pop	rbp
1414706d8: c3                          	ret
1414706d9: 0f 1f 80 00 00 00 00        	nop	dword ptr [rax]
1414706e0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1414706e5: 55                          	push	rbp
1414706e6: 41 57                       	push	r15
1414706e8: 41 56                       	push	r14
1414706ea: 41 55                       	push	r13
1414706ec: 41 54                       	push	r12
1414706ee: 56                          	push	rsi
1414706ef: 57                          	push	rdi
1414706f0: 53                          	push	rbx
1414706f1: 48 83 ec 78                 	sub	rsp, 0x78
1414706f5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1414706fc: 44 0f 29 54 24 20           	movaps	xmmword ptr [rsp + 0x20], xmm10
141470702: 44 0f 29 4c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm9
141470708: 44 0f 29 44 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm8
14147070e: 0f 29 7c 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm7
141470713: 0f 29 74 24 60              	movaps	xmmword ptr [rsp + 0x60], xmm6
141470718: 48 8b 8d 58 09 00 00        	mov	rcx, qword ptr [rbp + 0x958]
14147071f: e8 1c c4 ff ff              	call	0x14146cb40 <.text+0x146bb40>
141470724: 48 8b 85 50 09 00 00        	mov	rax, qword ptr [rbp + 0x950]
14147072b: 48 89 85 40 09 00 00        	mov	qword ptr [rbp + 0x940], rax
141470732: 48 8b 85 60 09 00 00        	mov	rax, qword ptr [rbp + 0x960]
141470739: 48 89 85 20 09 00 00        	mov	qword ptr [rbp + 0x920], rax
141470740: c6 85 77 09 00 00 00        	mov	byte ptr [rbp + 0x977], 0x0
141470747: 0f 28 74 24 60              	movaps	xmm6, xmmword ptr [rsp + 0x60]
14147074c: 0f 28 7c 24 50              	movaps	xmm7, xmmword ptr [rsp + 0x50]
141470751: 44 0f 28 44 24 40           	movaps	xmm8, xmmword ptr [rsp + 0x40]
141470757: 44 0f 28 4c 24 30           	movaps	xmm9, xmmword ptr [rsp + 0x30]
14147075d: 44 0f 28 54 24 20           	movaps	xmm10, xmmword ptr [rsp + 0x20]
141470763: 48 83 c4 78                 	add	rsp, 0x78
141470767: 5b                          	pop	rbx
141470768: 5f                          	pop	rdi
141470769: 5e                          	pop	rsi
14147076a: 41 5c                       	pop	r12
14147076c: 41 5d                       	pop	r13
14147076e: 41 5e                       	pop	r14
141470770: 41 5f                       	pop	r15
141470772: 5d                          	pop	rbp
141470773: c3                          	ret
141470774: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
141470780: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141470785: 55                          	push	rbp
141470786: 41 57                       	push	r15
141470788: 41 56                       	push	r14
14147078a: 41 55                       	push	r13
14147078c: 41 54                       	push	r12
14147078e: 56                          	push	rsi
14147078f: 57                          	push	rdi
141470790: 53                          	push	rbx
141470791: 48 83 ec 78                 	sub	rsp, 0x78
141470795: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14147079c: 44 0f 29 54 24 20           	movaps	xmmword ptr [rsp + 0x20], xmm10
1414707a2: 44 0f 29 4c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm9
1414707a8: 44 0f 29 44 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm8
1414707ae: 0f 29 7c 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm7
1414707b3: 0f 29 74 24 60              	movaps	xmmword ptr [rsp + 0x60], xmm6
1414707b8: 80 bd 73 09 00 00 00        	cmp	byte ptr [rbp + 0x973], 0x0
1414707bf: 74 5c                       	je	0x14147081d <.text+0x146f81d>
1414707c1: 48 8d 8d b0 01 00 00        	lea	rcx, [rbp + 0x1b0]
1414707c8: e8 e3 0b 42 ff              	call	0x1408913b0 <.text+0x8903b0>
1414707cd: 48 8b 85 50 09 00 00        	mov	rax, qword ptr [rbp + 0x950]
1414707d4: 48 89 85 40 09 00 00        	mov	qword ptr [rbp + 0x940], rax
1414707db: 48 8b 85 60 09 00 00        	mov	rax, qword ptr [rbp + 0x960]
1414707e2: 48 89 85 20 09 00 00        	mov	qword ptr [rbp + 0x920], rax
1414707e9: c6 85 77 09 00 00 00        	mov	byte ptr [rbp + 0x977], 0x0
1414707f0: 0f 28 74 24 60              	movaps	xmm6, xmmword ptr [rsp + 0x60]
1414707f5: 0f 28 7c 24 50              	movaps	xmm7, xmmword ptr [rsp + 0x50]
1414707fa: 44 0f 28 44 24 40           	movaps	xmm8, xmmword ptr [rsp + 0x40]
141470800: 44 0f 28 4c 24 30           	movaps	xmm9, xmmword ptr [rsp + 0x30]
141470806: 44 0f 28 54 24 20           	movaps	xmm10, xmmword ptr [rsp + 0x20]
14147080c: 48 83 c4 78                 	add	rsp, 0x78
141470810: 5b                          	pop	rbx
141470811: 5f                          	pop	rdi
141470812: 5e                          	pop	rsi
141470813: 41 5c                       	pop	r12
141470815: 41 5d                       	pop	r13
141470817: 41 5e                       	pop	r14
141470819: 41 5f                       	pop	r15
14147081b: 5d                          	pop	rbp
14147081c: c3                          	ret
14147081d: 48 8b 85 50 09 00 00        	mov	rax, qword ptr [rbp + 0x950]
141470824: 48 89 85 40 09 00 00        	mov	qword ptr [rbp + 0x940], rax
14147082b: 48 8b 85 60 09 00 00        	mov	rax, qword ptr [rbp + 0x960]
141470832: 48 89 85 20 09 00 00        	mov	qword ptr [rbp + 0x920], rax
141470839: c6 85 77 09 00 00 00        	mov	byte ptr [rbp + 0x977], 0x0
141470840: 0f 28 74 24 60              	movaps	xmm6, xmmword ptr [rsp + 0x60]
141470845: 0f 28 7c 24 50              	movaps	xmm7, xmmword ptr [rsp + 0x50]
14147084a: 44 0f 28 44 24 40           	movaps	xmm8, xmmword ptr [rsp + 0x40]
141470850: 44 0f 28 4c 24 30           	movaps	xmm9, xmmword ptr [rsp + 0x30]
141470856: 44 0f 28 54 24 20           	movaps	xmm10, xmmword ptr [rsp + 0x20]
14147085c: 48 83 c4 78                 	add	rsp, 0x78
141470860: 5b                          	pop	rbx
141470861: 5f                          	pop	rdi
141470862: 5e                          	pop	rsi
141470863: 41 5c                       	pop	r12
141470865: 41 5d                       	pop	r13
141470867: 41 5e                       	pop	r14
141470869: 41 5f                       	pop	r15
14147086b: 5d                          	pop	rbp
14147086c: c3                          	ret
14147086d: 0f 1f 00                    	nop	dword ptr [rax]
141470870: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141470875: 55                          	push	rbp
141470876: 41 57                       	push	r15
141470878: 41 56                       	push	r14
14147087a: 41 55                       	push	r13
14147087c: 41 54                       	push	r12
14147087e: 56                          	push	rsi
14147087f: 57                          	push	rdi
141470880: 53                          	push	rbx
141470881: 48 83 ec 78                 	sub	rsp, 0x78
141470885: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14147088c: 44 0f 29 54 24 20           	movaps	xmmword ptr [rsp + 0x20], xmm10
141470892: 44 0f 29 4c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm9
141470898: 44 0f 29 44 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm8
14147089e: 0f 29 7c 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm7
1414708a3: 0f 29 74 24 60              	movaps	xmmword ptr [rsp + 0x60], xmm6
1414708a8: 48 8b b5 20 09 00 00        	mov	rsi, qword ptr [rbp + 0x920]
1414708af: 48 8b bd 40 09 00 00        	mov	rdi, qword ptr [rbp + 0x940]
1414708b6: 80 bd 77 09 00 00 00        	cmp	byte ptr [rbp + 0x977], 0x0
1414708bd: 0f 84 84 00 00 00           	je	0x141470947 <.text+0x146f947>
1414708c3: 48 89 bd d0 08 00 00        	mov	qword ptr [rbp + 0x8d0], rdi
1414708ca: 48 89 b5 a0 08 00 00        	mov	qword ptr [rbp + 0x8a0], rsi
1414708d1: c6 85 71 09 00 00 01        	mov	byte ptr [rbp + 0x971], 0x1
1414708d8: c6 85 70 09 00 00 00        	mov	byte ptr [rbp + 0x970], 0x0
1414708df: 48 8b 9d 48 09 00 00        	mov	rbx, qword ptr [rbp + 0x948]
1414708e6: 48 89 d9                    	mov	rcx, rbx
1414708e9: e8 72 44 42 ff              	call	0x140894d60 <.text+0x893d60>
1414708ee: 48 8b 53 18                 	mov	rdx, qword ptr [rbx + 0x18]
1414708f2: 48 85 d2                    	test	rdx, rdx
1414708f5: 74 1a                       	je	0x141470911 <.text+0x146f911>
1414708f7: 48 8b 85 48 09 00 00        	mov	rax, qword ptr [rbp + 0x948]
1414708fe: 48 8b 48 20                 	mov	rcx, qword ptr [rax + 0x20]
141470902: 48 c1 e2 04                 	shl	rdx, 0x4
141470906: 41 b8 08 00 00 00           	mov	r8d, 0x8
14147090c: e8 0f 33 1a ff              	call	0x140613c20 <.text+0x612c20>
141470911: 48 8b 85 48 09 00 00        	mov	rax, qword ptr [rbp + 0x948]
141470918: 48 8b 40 30                 	mov	rax, qword ptr [rax + 0x30]
14147091c: 48 8d 0c 45 00 00 00 00     	lea	rcx, [2*rax]
141470924: 48 85 c9                    	test	rcx, rcx
141470927: 74 1e                       	je	0x141470947 <.text+0x146f947>
141470929: 48 8b 8d 48 09 00 00        	mov	rcx, qword ptr [rbp + 0x948]
141470930: 48 8b 49 38                 	mov	rcx, qword ptr [rcx + 0x38]
141470934: 48 c1 e0 03                 	shl	rax, 0x3
141470938: 48 8d 14 80                 	lea	rdx, [rax + 4*rax]
14147093c: 41 b8 04 00 00 00           	mov	r8d, 0x4
141470942: e8 d9 32 1a ff              	call	0x140613c20 <.text+0x612c20>
141470947: ba f0 00 00 00              	mov	edx, 0xf0
14147094c: 41 b8 08 00 00 00           	mov	r8d, 0x8
141470952: 48 8b 8d 48 09 00 00        	mov	rcx, qword ptr [rbp + 0x948]
141470959: e8 c2 32 1a ff              	call	0x140613c20 <.text+0x612c20>
14147095e: 48 89 bd d0 08 00 00        	mov	qword ptr [rbp + 0x8d0], rdi
141470965: 48 89 b5 a0 08 00 00        	mov	qword ptr [rbp + 0x8a0], rsi
14147096c: c6 85 71 09 00 00 01        	mov	byte ptr [rbp + 0x971], 0x1
141470973: c6 85 70 09 00 00 00        	mov	byte ptr [rbp + 0x970], 0x0
14147097a: 0f 28 74 24 60              	movaps	xmm6, xmmword ptr [rsp + 0x60]
14147097f: 0f 28 7c 24 50              	movaps	xmm7, xmmword ptr [rsp + 0x50]
141470984: 44 0f 28 44 24 40           	movaps	xmm8, xmmword ptr [rsp + 0x40]
14147098a: 44 0f 28 4c 24 30           	movaps	xmm9, xmmword ptr [rsp + 0x30]
141470990: 44 0f 28 54 24 20           	movaps	xmm10, xmmword ptr [rsp + 0x20]
141470996: 48 83 c4 78                 	add	rsp, 0x78
14147099a: 5b                          	pop	rbx
14147099b: 5f                          	pop	rdi
14147099c: 5e                          	pop	rsi
14147099d: 41 5c                       	pop	r12
14147099f: 41 5d                       	pop	r13
1414709a1: 41 5e                       	pop	r14
1414709a3: 41 5f                       	pop	r15
1414709a5: 5d                          	pop	rbp
1414709a6: c3                          	ret
1414709a7: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1414709b0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1414709b5: 55                          	push	rbp
1414709b6: 41 57                       	push	r15
1414709b8: 41 56                       	push	r14
1414709ba: 41 55                       	push	r13
1414709bc: 41 54                       	push	r12
1414709be: 56                          	push	rsi
1414709bf: 57                          	push	rdi
1414709c0: 53                          	push	rbx
1414709c1: 48 83 ec 78                 	sub	rsp, 0x78
1414709c5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1414709cc: 44 0f 29 54 24 20           	movaps	xmmword ptr [rsp + 0x20], xmm10
1414709d2: 44 0f 29 4c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm9
1414709d8: 44 0f 29 44 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm8
1414709de: 0f 29 7c 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm7
1414709e3: 0f 29 74 24 60              	movaps	xmmword ptr [rsp + 0x60], xmm6
1414709e8: 0f b6 9d 75 09 00 00        	movzx	ebx, byte ptr [rbp + 0x975]
1414709ef: 80 bd 74 09 00 00 00        	cmp	byte ptr [rbp + 0x974], 0x0
1414709f6: 74 65                       	je	0x141470a5d <.text+0x146fa5d>
1414709f8: 48 8b 8d 58 09 00 00        	mov	rcx, qword ptr [rbp + 0x958]
1414709ff: e8 3c c1 ff ff              	call	0x14146cb40 <.text+0x146bb40>
141470a04: 48 8b 85 50 09 00 00        	mov	rax, qword ptr [rbp + 0x950]
141470a0b: 48 89 85 d0 08 00 00        	mov	qword ptr [rbp + 0x8d0], rax
141470a12: 48 8b 85 60 09 00 00        	mov	rax, qword ptr [rbp + 0x960]
141470a19: 48 89 85 a0 08 00 00        	mov	qword ptr [rbp + 0x8a0], rax
141470a20: 80 e3 01                    	and	bl, 0x1
141470a23: 88 9d 71 09 00 00           	mov	byte ptr [rbp + 0x971], bl
141470a29: c6 85 70 09 00 00 00        	mov	byte ptr [rbp + 0x970], 0x0
141470a30: 0f 28 74 24 60              	movaps	xmm6, xmmword ptr [rsp + 0x60]
141470a35: 0f 28 7c 24 50              	movaps	xmm7, xmmword ptr [rsp + 0x50]
141470a3a: 44 0f 28 44 24 40           	movaps	xmm8, xmmword ptr [rsp + 0x40]
141470a40: 44 0f 28 4c 24 30           	movaps	xmm9, xmmword ptr [rsp + 0x30]
141470a46: 44 0f 28 54 24 20           	movaps	xmm10, xmmword ptr [rsp + 0x20]
141470a4c: 48 83 c4 78                 	add	rsp, 0x78
141470a50: 5b                          	pop	rbx
141470a51: 5f                          	pop	rdi
141470a52: 5e                          	pop	rsi
141470a53: 41 5c                       	pop	r12
141470a55: 41 5d                       	pop	r13
141470a57: 41 5e                       	pop	r14
141470a59: 41 5f                       	pop	r15
141470a5b: 5d                          	pop	rbp
141470a5c: c3                          	ret
141470a5d: 48 8b 85 50 09 00 00        	mov	rax, qword ptr [rbp + 0x950]
141470a64: 48 89 85 d0 08 00 00        	mov	qword ptr [rbp + 0x8d0], rax
141470a6b: 48 8b 85 60 09 00 00        	mov	rax, qword ptr [rbp + 0x960]
141470a72: 48 89 85 a0 08 00 00        	mov	qword ptr [rbp + 0x8a0], rax
141470a79: 80 e3 01                    	and	bl, 0x1
141470a7c: 88 9d 71 09 00 00           	mov	byte ptr [rbp + 0x971], bl
141470a82: c6 85 70 09 00 00 00        	mov	byte ptr [rbp + 0x970], 0x0
141470a89: 0f 28 74 24 60              	movaps	xmm6, xmmword ptr [rsp + 0x60]
141470a8e: 0f 28 7c 24 50              	movaps	xmm7, xmmword ptr [rsp + 0x50]
141470a93: 44 0f 28 44 24 40           	movaps	xmm8, xmmword ptr [rsp + 0x40]
141470a99: 44 0f 28 4c 24 30           	movaps	xmm9, xmmword ptr [rsp + 0x30]
141470a9f: 44 0f 28 54 24 20           	movaps	xmm10, xmmword ptr [rsp + 0x20]
141470aa5: 48 83 c4 78                 	add	rsp, 0x78
141470aa9: 5b                          	pop	rbx
141470aaa: 5f                          	pop	rdi
141470aab: 5e                          	pop	rsi
141470aac: 41 5c                       	pop	r12
141470aae: 41 5d                       	pop	r13
141470ab0: 41 5e                       	pop	r14
141470ab2: 41 5f                       	pop	r15
141470ab4: 5d                          	pop	rbp
141470ab5: c3                          	ret
141470ab6: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
141470ac0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141470ac5: 55                          	push	rbp
141470ac6: 41 57                       	push	r15
141470ac8: 41 56                       	push	r14
141470aca: 41 55                       	push	r13
141470acc: 41 54                       	push	r12
141470ace: 56                          	push	rsi
141470acf: 57                          	push	rdi
141470ad0: 53                          	push	rbx
141470ad1: 48 83 ec 78                 	sub	rsp, 0x78
141470ad5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
141470adc: 44 0f 29 54 24 20           	movaps	xmmword ptr [rsp + 0x20], xmm10
141470ae2: 44 0f 29 4c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm9
141470ae8: 44 0f 29 44 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm8
141470aee: 0f 29 7c 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm7
141470af3: 0f 29 74 24 60              	movaps	xmmword ptr [rsp + 0x60], xmm6
141470af8: 48 8b 85 30 09 00 00        	mov	rax, qword ptr [rbp + 0x930]
141470aff: 48 8d 0c 45 00 00 00 00     	lea	rcx, [2*rax]
141470b07: 48 85 c9                    	test	rcx, rcx
141470b0a: 0f 95 c1                    	setne	cl
141470b0d: 84 8d 76 09 00 00           	test	byte ptr [rbp + 0x976], cl
141470b13: 74 1a                       	je	0x141470b2f <.text+0x146fb2f>
141470b15: 48 8b 8d 48 05 00 00        	mov	rcx, qword ptr [rbp + 0x548]
141470b1c: 48 c1 e0 03                 	shl	rax, 0x3
141470b20: 48 8d 14 80                 	lea	rdx, [rax + 4*rax]
141470b24: 41 b8 04 00 00 00           	mov	r8d, 0x4
141470b2a: e8 f1 30 1a ff              	call	0x140613c20 <.text+0x612c20>
141470b2f: 0f 28 74 24 60              	movaps	xmm6, xmmword ptr [rsp + 0x60]
141470b34: 0f 28 7c 24 50              	movaps	xmm7, xmmword ptr [rsp + 0x50]
141470b39: 44 0f 28 44 24 40           	movaps	xmm8, xmmword ptr [rsp + 0x40]
141470b3f: 44 0f 28 4c 24 30           	movaps	xmm9, xmmword ptr [rsp + 0x30]
141470b45: 44 0f 28 54 24 20           	movaps	xmm10, xmmword ptr [rsp + 0x20]
141470b4b: 48 83 c4 78                 	add	rsp, 0x78
141470b4f: 5b                          	pop	rbx
141470b50: 5f                          	pop	rdi
141470b51: 5e                          	pop	rsi
141470b52: 41 5c                       	pop	r12
141470b54: 41 5d                       	pop	r13
141470b56: 41 5e                       	pop	r14
141470b58: 41 5f                       	pop	r15
141470b5a: 5d                          	pop	rbp
141470b5b: c3                          	ret
141470b5c: 0f 1f 40 00                 	nop	dword ptr [rax]
141470b60: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141470b65: 55                          	push	rbp
141470b66: 41 57                       	push	r15
141470b68: 41 56                       	push	r14
141470b6a: 41 55                       	push	r13
141470b6c: 41 54                       	push	r12
141470b6e: 56                          	push	rsi
141470b6f: 57                          	push	rdi
141470b70: 53                          	push	rbx
141470b71: 48 83 ec 78                 	sub	rsp, 0x78
141470b75: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
141470b7c: 44 0f 29 54 24 20           	movaps	xmmword ptr [rsp + 0x20], xmm10
141470b82: 44 0f 29 4c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm9
141470b88: 44 0f 29 44 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm8
141470b8e: 0f 29 7c 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm7
141470b93: 0f 29 74 24 60              	movaps	xmmword ptr [rsp + 0x60], xmm6
141470b98: 48 8d 8d b0 01 00 00        	lea	rcx, [rbp + 0x1b0]
141470b9f: e8 5c ee 41 ff              	call	0x14088fa00 <.text+0x88ea00>
141470ba4: 0f 28 74 24 60              	movaps	xmm6, xmmword ptr [rsp + 0x60]
141470ba9: 0f 28 7c 24 50              	movaps	xmm7, xmmword ptr [rsp + 0x50]
141470bae: 44 0f 28 44 24 40           	movaps	xmm8, xmmword ptr [rsp + 0x40]
141470bb4: 44 0f 28 4c 24 30           	movaps	xmm9, xmmword ptr [rsp + 0x30]
141470bba: 44 0f 28 54 24 20           	movaps	xmm10, xmmword ptr [rsp + 0x20]
141470bc0: 48 83 c4 78                 	add	rsp, 0x78
141470bc4: 5b                          	pop	rbx
141470bc5: 5f                          	pop	rdi
141470bc6: 5e                          	pop	rsi
141470bc7: 41 5c                       	pop	r12
141470bc9: 41 5d                       	pop	r13
141470bcb: 41 5e                       	pop	r14
141470bcd: 41 5f                       	pop	r15
141470bcf: 5d                          	pop	rbp
141470bd0: c3                          	ret
141470bd1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
141470be0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141470be5: 55                          	push	rbp
141470be6: 41 57                       	push	r15
141470be8: 41 56                       	push	r14
141470bea: 41 55                       	push	r13
141470bec: 41 54                       	push	r12
141470bee: 56                          	push	rsi
141470bef: 57                          	push	rdi
141470bf0: 53                          	push	rbx
141470bf1: 48 83 ec 78                 	sub	rsp, 0x78
141470bf5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
141470bfc: 44 0f 29 54 24 20           	movaps	xmmword ptr [rsp + 0x20], xmm10
141470c02: 44 0f 29 4c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm9
141470c08: 44 0f 29 44 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm8
141470c0e: 0f 29 7c 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm7
141470c13: 0f 29 74 24 60              	movaps	xmmword ptr [rsp + 0x60], xmm6
141470c18: c6 85 75 09 00 00 00        	mov	byte ptr [rbp + 0x975], 0x0
141470c1f: c6 85 74 09 00 00 00        	mov	byte ptr [rbp + 0x974], 0x0
141470c26: 48 8d 8d b0 05 00 00        	lea	rcx, [rbp + 0x5b0]
141470c2d: e8 5e d9 41 ff              	call	0x14088e590 <.text+0x88d590>
141470c32: c6 85 75 09 00 00 00        	mov	byte ptr [rbp + 0x975], 0x0
141470c39: c6 85 74 09 00 00 00        	mov	byte ptr [rbp + 0x974], 0x0
141470c40: 0f 28 74 24 60              	movaps	xmm6, xmmword ptr [rsp + 0x60]
141470c45: 0f 28 7c 24 50              	movaps	xmm7, xmmword ptr [rsp + 0x50]
141470c4a: 44 0f 28 44 24 40           	movaps	xmm8, xmmword ptr [rsp + 0x40]
141470c50: 44 0f 28 4c 24 30           	movaps	xmm9, xmmword ptr [rsp + 0x30]
141470c56: 44 0f 28 54 24 20           	movaps	xmm10, xmmword ptr [rsp + 0x20]
141470c5c: 48 83 c4 78                 	add	rsp, 0x78
141470c60: 5b                          	pop	rbx
141470c61: 5f                          	pop	rdi
141470c62: 5e                          	pop	rsi
141470c63: 41 5c                       	pop	r12
141470c65: 41 5d                       	pop	r13
141470c67: 41 5e                       	pop	r14
141470c69: 41 5f                       	pop	r15
141470c6b: 5d                          	pop	rbp
141470c6c: c3                          	ret
141470c6d: 0f 1f 00                    	nop	dword ptr [rax]
141470c70: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141470c75: 55                          	push	rbp
141470c76: 41 57                       	push	r15
141470c78: 41 56                       	push	r14
141470c7a: 41 55                       	push	r13
141470c7c: 41 54                       	push	r12
141470c7e: 56                          	push	rsi
141470c7f: 57                          	push	rdi
141470c80: 53                          	push	rbx
141470c81: 48 83 ec 78                 	sub	rsp, 0x78
141470c85: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
141470c8c: 44 0f 29 54 24 20           	movaps	xmmword ptr [rsp + 0x20], xmm10
141470c92: 44 0f 29 4c 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm9
141470c98: 44 0f 29 44 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm8
141470c9e: 0f 29 7c 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm7
141470ca3: 0f 29 74 24 60              	movaps	xmmword ptr [rsp + 0x60], xmm6
141470ca8: 48 8d 8d 60 03 00 00        	lea	rcx, [rbp + 0x360]
141470caf: e8 3c a6 40 ff              	call	0x14087b2f0 <.text+0x87a2f0>
141470cb4: 48 b8 00 00 00 00 00 00 00 80       	movabs	rax, -0x8000000000000000
141470cbe: 48 89 85 30 09 00 00        	mov	qword ptr [rbp + 0x930], rax
141470cc5: c6 85 76 09 00 00 01        	mov	byte ptr [rbp + 0x976], 0x1
141470ccc: 0f 28 74 24 60              	movaps	xmm6, xmmword ptr [rsp + 0x60]
141470cd1: 0f 28 7c 24 50              	movaps	xmm7, xmmword ptr [rsp + 0x50]
141470cd6: 44 0f 28 44 24 40           	movaps	xmm8, xmmword ptr [rsp + 0x40]
141470cdc: 44 0f 28 4c 24 30           	movaps	xmm9, xmmword ptr [rsp + 0x30]
141470ce2: 44 0f 28 54 24 20           	movaps	xmm10, xmmword ptr [rsp + 0x20]
141470ce8: 48 83 c4 78                 	add	rsp, 0x78
141470cec: 5b                          	pop	rbx
141470ced: 5f                          	pop	rdi
141470cee: 5e                          	pop	rsi
141470cef: 41 5c                       	pop	r12
141470cf1: 41 5d                       	pop	r13
141470cf3: 41 5e                       	pop	r14
141470cf5: 41 5f                       	pop	r15
141470cf7: 5d                          	pop	rbp
141470cf8: c3                          	ret
