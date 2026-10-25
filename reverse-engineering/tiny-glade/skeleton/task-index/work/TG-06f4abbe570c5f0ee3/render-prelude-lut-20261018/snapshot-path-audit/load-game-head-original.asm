
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
