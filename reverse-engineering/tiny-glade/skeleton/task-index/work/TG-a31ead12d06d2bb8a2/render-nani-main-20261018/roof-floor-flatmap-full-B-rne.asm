
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141302d60: 55                          	push	rbp
141302d61: 41 57                       	push	r15
141302d63: 41 56                       	push	r14
141302d65: 41 55                       	push	r13
141302d67: 41 54                       	push	r12
141302d69: 56                          	push	rsi
141302d6a: 57                          	push	rdi
141302d6b: 53                          	push	rbx
141302d6c: 48 81 ec 68 01 00 00        	sub	rsp, 0x168
141302d73: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
141302d7b: 44 0f 29 bd d0 00 00 00     	movaps	xmmword ptr [rbp + 0xd0], xmm15
141302d83: 44 0f 29 b5 c0 00 00 00     	movaps	xmmword ptr [rbp + 0xc0], xmm14
141302d8b: 66 44 0f 7f ad b0 00 00 00  	movdqa	xmmword ptr [rbp + 0xb0], xmm13
141302d94: 44 0f 29 a5 a0 00 00 00     	movaps	xmmword ptr [rbp + 0xa0], xmm12
141302d9c: 44 0f 29 9d 90 00 00 00     	movaps	xmmword ptr [rbp + 0x90], xmm11
141302da4: 44 0f 29 95 80 00 00 00     	movaps	xmmword ptr [rbp + 0x80], xmm10
141302dac: 44 0f 29 4d 70              	movaps	xmmword ptr [rbp + 0x70], xmm9
141302db1: 44 0f 29 45 60              	movaps	xmmword ptr [rbp + 0x60], xmm8
141302db6: 0f 29 7d 50                 	movaps	xmmword ptr [rbp + 0x50], xmm7
141302dba: 0f 29 75 40                 	movaps	xmmword ptr [rbp + 0x40], xmm6
141302dbe: 48 c7 45 38 fe ff ff ff     	mov	qword ptr [rbp + 0x38], -0x2
141302dc6: 4c 89 45 28                 	mov	qword ptr [rbp + 0x28], r8
141302dca: 44 0f b6 02                 	movzx	r8d, byte ptr [rdx]
141302dce: 48 8b 7a 08                 	mov	rdi, qword ptr [rdx + 0x8]
141302dd2: 48 8b 5a 10                 	mov	rbx, qword ptr [rdx + 0x10]
141302dd6: 48 89 d8                    	mov	rax, rbx
141302dd9: 48 29 f8                    	sub	rax, rdi
141302ddc: 45 31 ff                    	xor	r15d, r15d
141302ddf: 44 88 45 37                 	mov	byte ptr [rbp + 0x37], r8b
141302de3: 45 84 c0                    	test	r8b, r8b
141302de6: 41 be 00 00 00 00           	mov	r14d, 0x0
141302dec: 48 89 45 c8                 	mov	qword ptr [rbp - 0x38], rax
141302df0: 4c 0f 45 f0                 	cmovne	r14, rax
141302df4: 48 89 4d b0                 	mov	qword ptr [rbp - 0x50], rcx
141302df8: 0f b6 42 30                 	movzx	eax, byte ptr [rdx + 0x30]
141302dfc: 48 8b 72 38                 	mov	rsi, qword ptr [rdx + 0x38]
141302e00: 4c 8b 42 40                 	mov	r8, qword ptr [rdx + 0x40]
141302e04: 4c 89 c1                    	mov	rcx, r8
141302e07: 48 29 f1                    	sub	rcx, rsi
141302e0a: 88 45 36                    	mov	byte ptr [rbp + 0x36], al
141302e0d: 84 c0                       	test	al, al
141302e0f: b8 00 00 00 00              	mov	eax, 0x0
141302e14: 48 89 4d c0                 	mov	qword ptr [rbp - 0x40], rcx
141302e18: 48 0f 45 c1                 	cmovne	rax, rcx
141302e1c: 48 8b 4a 60                 	mov	rcx, qword ptr [rdx + 0x60]
141302e20: 48 89 4d 18                 	mov	qword ptr [rbp + 0x18], rcx
141302e24: 48 85 c9                    	test	rcx, rcx
141302e27: 74 10                       	je	0x141302e39 <.text+0x1301e39>
141302e29: 8b 4a 6c                    	mov	ecx, dword ptr [rdx + 0x6c]
141302e2c: 2b 4a 68                    	sub	ecx, dword ptr [rdx + 0x68]
141302e2f: 41 0f 42 cf                 	cmovb	ecx, r15d
141302e33: 48 01 c9                    	add	rcx, rcx
141302e36: 49 89 cf                    	mov	r15, rcx
141302e39: 48 89 55 20                 	mov	qword ptr [rbp + 0x20], rdx
141302e3d: 4c 89 45 b8                 	mov	qword ptr [rbp - 0x48], r8
141302e41: 49 01 c6                    	add	r14, rax
141302e44: 0f 82 8c 05 00 00           	jb	0x1413033d6 <.text+0x13023d6>
141302e4a: 4d 01 f7                    	add	r15, r14
141302e4d: 0f 82 83 05 00 00           	jb	0x1413033d6 <.text+0x13023d6>
141302e53: 45 31 ed                    	xor	r13d, r13d
141302e56: b9 0c 00 00 00              	mov	ecx, 0xc
141302e5b: 4c 89 f8                    	mov	rax, r15
141302e5e: 48 f7 e1                    	mul	rcx
141302e61: 49 89 c4                    	mov	r12, rax
141302e64: 0f 80 9b 05 00 00           	jo	0x141303405 <.text+0x1302405>
141302e6a: 48 b8 fc ff ff ff ff ff ff 7f       	movabs	rax, 0x7ffffffffffffffc
141302e74: 49 39 c4                    	cmp	r12, rax
141302e77: 0f 87 88 05 00 00           	ja	0x141303405 <.text+0x1302405>
141302e7d: 31 d2                       	xor	edx, edx
141302e7f: 4d 85 e4                    	test	r12, r12
141302e82: 74 27                       	je	0x141302eab <.text+0x1301eab>
141302e84: 0f b6 05 7e 1d 33 02        	movzx	eax, byte ptr [rip + 0x2331d7e] # 0x143634c09
141302e8b: 41 bd 04 00 00 00           	mov	r13d, 0x4
141302e91: ba 04 00 00 00              	mov	edx, 0x4
141302e96: 4c 89 e1                    	mov	rcx, r12
141302e99: e8 72 0d 31 ff              	call	0x140613c10 <.text+0x612c10>
141302e9e: 48 85 c0                    	test	rax, rax
141302ea1: 0f 84 5e 05 00 00           	je	0x141303405 <.text+0x1302405>
141302ea7: 31 d2                       	xor	edx, edx
141302ea9: eb 08                       	jmp	0x141302eb3 <.text+0x1301eb3>
141302eab: b8 04 00 00 00              	mov	eax, 0x4
141302eb0: 45 31 ff                    	xor	r15d, r15d
141302eb3: 4c 89 7d 00                 	mov	qword ptr [rbp], r15
141302eb7: 48 89 45 08                 	mov	qword ptr [rbp + 0x8], rax
141302ebb: 48 c7 45 10 00 00 00 00     	mov	qword ptr [rbp + 0x10], 0x0
141302ec3: 48 8b 4d 20                 	mov	rcx, qword ptr [rbp + 0x20]
141302ec7: 44 8b 69 68                 	mov	r13d, dword ptr [rcx + 0x68]
141302ecb: 44 8b 61 6c                 	mov	r12d, dword ptr [rcx + 0x6c]
141302ecf: 44 89 e1                    	mov	ecx, r12d
141302ed2: 44 29 e9                    	sub	ecx, r13d
141302ed5: 0f 42 ca                    	cmovb	ecx, edx
141302ed8: 48 01 c9                    	add	rcx, rcx
141302edb: 48 8b 55 18                 	mov	rdx, qword ptr [rbp + 0x18]
141302edf: 48 85 d2                    	test	rdx, rdx
141302ee2: 48 0f 44 ca                 	cmove	rcx, rdx
141302ee6: 49 01 ce                    	add	r14, rcx
141302ee9: 0f 82 25 05 00 00           	jb	0x141303414 <.text+0x1302414>
141302eef: 4d 39 f7                    	cmp	r15, r14
141302ef2: 0f 82 a7 04 00 00           	jb	0x14130339f <.text+0x130239f>
141302ef8: 31 c9                       	xor	ecx, ecx
141302efa: 48 8d 55 e0                 	lea	rdx, [rbp - 0x20]
141302efe: f6 45 37 01                 	test	byte ptr [rbp + 0x37], 0x1
141302f02: 0f 84 8e 00 00 00           	je	0x141302f96 <.text+0x1301f96>
141302f08: 4c 8b 45 20                 	mov	r8, qword ptr [rbp + 0x20]
141302f0c: 49 83 c0 18                 	add	r8, 0x18
141302f10: 4d 8b 48 10                 	mov	r9, qword ptr [r8 + 0x10]
141302f14: 4c 89 4d f0                 	mov	qword ptr [rbp - 0x10], r9
141302f18: 41 0f 10 00                 	movups	xmm0, xmmword ptr [r8]
141302f1c: 0f 11 45 e0                 	movups	xmmword ptr [rbp - 0x20], xmm0
141302f20: 48 39 fb                    	cmp	rbx, rdi
141302f23: 74 71                       	je	0x141302f96 <.text+0x1301f96>
141302f25: 4c 8d 47 01                 	lea	r8, [rdi + 0x1]
141302f29: f6 45 c8 01                 	test	byte ptr [rbp - 0x38], 0x1
141302f2d: 74 20                       	je	0x141302f4f <.text+0x1301f4f>
141302f2f: 4c 8d 0c 7f                 	lea	r9, [rdi + 2*rdi]
141302f33: 4c 8d 14 49                 	lea	r10, [rcx + 2*rcx]
141302f37: 46 8b 5c 8a 08              	mov	r11d, dword ptr [rdx + 4*r9 + 0x8]
141302f3c: 46 89 5c 90 08              	mov	dword ptr [rax + 4*r10 + 0x8], r11d
141302f41: 4e 8b 0c 8a                 	mov	r9, qword ptr [rdx + 4*r9]
141302f45: 4e 89 0c 90                 	mov	qword ptr [rax + 4*r10], r9
141302f49: 48 ff c1                    	inc	rcx
141302f4c: 4c 89 c7                    	mov	rdi, r8
141302f4f: 4c 39 c3                    	cmp	rbx, r8
141302f52: 74 42                       	je	0x141302f96 <.text+0x1301f96>
141302f54: 4c 8d 45 ec                 	lea	r8, [rbp - 0x14]
141302f58: 4c 8d 0c 49                 	lea	r9, [rcx + 2*rcx]
141302f5c: 4e 8d 0c 88                 	lea	r9, [rax + 4*r9]
141302f60: 48 83 c1 02                 	add	rcx, 0x2
141302f64: 4c 8d 14 7f                 	lea	r10, [rdi + 2*rdi]
141302f68: 46 8b 5c 92 08              	mov	r11d, dword ptr [rdx + 4*r10 + 0x8]
141302f6d: 45 89 59 08                 	mov	dword ptr [r9 + 0x8], r11d
141302f71: 4e 8b 14 92                 	mov	r10, qword ptr [rdx + 4*r10]
141302f75: 4d 89 11                    	mov	qword ptr [r9], r10
141302f78: 4d 8b 10                    	mov	r10, qword ptr [r8]
141302f7b: 4d 89 51 0c                 	mov	qword ptr [r9 + 0xc], r10
141302f7f: 45 8b 50 08                 	mov	r10d, dword ptr [r8 + 0x8]
141302f83: 45 89 51 14                 	mov	dword ptr [r9 + 0x14], r10d
141302f87: 49 83 c1 18                 	add	r9, 0x18
141302f8b: bf 02 00 00 00              	mov	edi, 0x2
141302f90: 48 83 fb 02                 	cmp	rbx, 0x2
141302f94: 75 ca                       	jne	0x141302f60 <.text+0x1301f60>
141302f96: 48 83 7d 18 00              	cmp	qword ptr [rbp + 0x18], 0x0
141302f9b: 0f 84 e6 02 00 00           	je	0x141303287 <.text+0x1302287>
141302fa1: 45 39 e5                    	cmp	r13d, r12d
141302fa4: 0f 83 dd 02 00 00           	jae	0x141303287 <.text+0x1302287>
141302faa: 4c 8b 45 18                 	mov	r8, qword ptr [rbp + 0x18]
141302fae: 45 8b 00                    	mov	r8d, dword ptr [r8]
141302fb1: 0f 57 c0                    	xorps	xmm0, xmm0
141302fb4: f3 49 0f 2a c0              	cvtsi2ss	xmm0, r8
141302fb9: 45 89 e8                    	mov	r8d, r13d
141302fbc: 41 f7 d0                    	not	r8d
141302fbf: 45 01 e0                    	add	r8d, r12d
141302fc2: 41 83 f8 0b                 	cmp	r8d, 0xb
141302fc6: 0f 83 fc 00 00 00           	jae	0x1413030c8 <.text+0x13020c8>
141302fcc: 45 89 e1                    	mov	r9d, r12d
141302fcf: 45 29 e9                    	sub	r9d, r13d
141302fd2: 45 8d 45 01                 	lea	r8d, [r13 + 0x1]
141302fd6: 41 f6 c1 01                 	test	r9b, 0x1
141302fda: 74 4b                       	je	0x141303027 <.text+0x1302027>
141302fdc: 45 89 e9                    	mov	r9d, r13d
141302fdf: 0f 57 c9                    	xorps	xmm1, xmm1
141302fe2: f3 49 0f 2a c9              	cvtsi2ss	xmm1, r9
141302fe7: f3 0f 5e c8                 	divss	xmm1, xmm0
141302feb: f3 0f 58 0d f5 70 65 01     	addss	xmm1, dword ptr [rip + 0x16570f5] # 0x14295a0e8
141302ff3: 4c 8d 0c 49                 	lea	r9, [rcx + 2*rcx]
141302ff7: f3 0f 10 15 e9 70 65 01     	movss	xmm2, dword ptr [rip + 0x16570e9] # 0x14295a0e8
141302fff: 42 0f 13 14 88              	movlps	qword ptr [rax + 4*r9], xmm2
141303004: f3 42 0f 11 4c 88 08        	movss	dword ptr [rax + 4*r9 + 0x8], xmm1
14130300b: f3 0f 10 15 5d 28 62 01     	movss	xmm2, dword ptr [rip + 0x162285d] # 0x142925870
141303013: 42 0f 13 54 88 0c           	movlps	qword ptr [rax + 4*r9 + 0xc], xmm2
141303019: f3 42 0f 11 4c 88 14        	movss	dword ptr [rax + 4*r9 + 0x14], xmm1
141303020: 48 83 c1 02                 	add	rcx, 0x2
141303024: 45 89 c5                    	mov	r13d, r8d
141303027: 45 39 c4                    	cmp	r12d, r8d
14130302a: 0f 84 57 02 00 00           	je	0x141303287 <.text+0x1302287>
141303030: 4c 8d 04 49                 	lea	r8, [rcx + 2*rcx]
141303034: 4e 8d 04 80                 	lea	r8, [rax + 4*r8]
141303038: 49 83 c0 2c                 	add	r8, 0x2c
14130303c: f3 0f 10 0d a4 70 65 01     	movss	xmm1, dword ptr [rip + 0x16570a4] # 0x14295a0e8
141303044: f3 0f 10 15 9c 70 65 01     	movss	xmm2, dword ptr [rip + 0x165709c] # 0x14295a0e8
14130304c: f3 0f 10 1d 1c 28 62 01     	movss	xmm3, dword ptr [rip + 0x162281c] # 0x142925870
141303054: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
141303060: 45 89 e9                    	mov	r9d, r13d
141303063: 0f 57 e4                    	xorps	xmm4, xmm4
141303066: f3 49 0f 2a e1              	cvtsi2ss	xmm4, r9
14130306b: f3 0f 5e e0                 	divss	xmm4, xmm0
14130306f: f3 0f 58 e1                 	addss	xmm4, xmm1
141303073: 41 0f 13 50 d4              	movlps	qword ptr [r8 - 0x2c], xmm2
141303078: f3 41 0f 11 60 dc           	movss	dword ptr [r8 - 0x24], xmm4
14130307e: 45 8d 4d 01                 	lea	r9d, [r13 + 0x1]
141303082: 0f 57 ed                    	xorps	xmm5, xmm5
141303085: f3 49 0f 2a e9              	cvtsi2ss	xmm5, r9
14130308a: 41 0f 13 58 e0              	movlps	qword ptr [r8 - 0x20], xmm3
14130308f: f3 41 0f 11 60 e8           	movss	dword ptr [r8 - 0x18], xmm4
141303095: f3 0f 5e e8                 	divss	xmm5, xmm0
141303099: f3 0f 58 e9                 	addss	xmm5, xmm1
14130309d: 41 0f 13 50 ec              	movlps	qword ptr [r8 - 0x14], xmm2
1413030a2: f3 41 0f 11 68 f4           	movss	dword ptr [r8 - 0xc], xmm5
1413030a8: 41 0f 13 58 f8              	movlps	qword ptr [r8 - 0x8], xmm3
1413030ad: f3 41 0f 11 28              	movss	dword ptr [r8], xmm5
1413030b2: 48 83 c1 04                 	add	rcx, 0x4
1413030b6: 49 83 c0 30                 	add	r8, 0x30
1413030ba: 41 83 c5 02                 	add	r13d, 0x2
1413030be: 45 39 e5                    	cmp	r13d, r12d
1413030c1: 75 9d                       	jne	0x141303060 <.text+0x1302060>
1413030c3: e9 bf 01 00 00              	jmp	0x141303287 <.text+0x1302287>
1413030c8: 4c 8d 0c 49                 	lea	r9, [rcx + 2*rcx]
1413030cc: 4e 8d 1c 88                 	lea	r11, [rax + 4*r9]
1413030d0: 49 83 c3 10                 	add	r11, 0x10
1413030d4: 4e 8d 14 c5 00 00 00 00     	lea	r10, [8*r8]
1413030dc: 4f 8d 14 52                 	lea	r10, [r10 + 2*r10]
1413030e0: 4b 8d 3c 13                 	lea	rdi, [r11 + r10]
1413030e4: 4c 39 df                    	cmp	rdi, r11
1413030e7: 0f 82 df fe ff ff           	jb	0x141302fcc <.text+0x1301fcc>
1413030ed: 4e 8d 1c 88                 	lea	r11, [rax + 4*r9]
1413030f1: 49 83 c3 14                 	add	r11, 0x14
1413030f5: 4b 8d 3c 13                 	lea	rdi, [r11 + r10]
1413030f9: 4c 39 df                    	cmp	rdi, r11
1413030fc: 0f 82 ca fe ff ff           	jb	0x141302fcc <.text+0x1301fcc>
141303102: 4e 8d 1c 88                 	lea	r11, [rax + 4*r9]
141303106: 49 83 c3 0c                 	add	r11, 0xc
14130310a: 4b 8d 3c 13                 	lea	rdi, [r11 + r10]
14130310e: 4c 39 df                    	cmp	rdi, r11
141303111: 0f 82 b5 fe ff ff           	jb	0x141302fcc <.text+0x1301fcc>
141303117: 4e 8d 1c 88                 	lea	r11, [rax + 4*r9]
14130311b: 49 83 c3 08                 	add	r11, 0x8
14130311f: 4b 8d 3c 13                 	lea	rdi, [r11 + r10]
141303123: 4c 39 df                    	cmp	rdi, r11
141303126: 0f 82 a0 fe ff ff           	jb	0x141302fcc <.text+0x1301fcc>
14130312c: 4e 8d 1c 88                 	lea	r11, [rax + 4*r9]
141303130: 49 83 c3 04                 	add	r11, 0x4
141303134: 4b 8d 3c 13                 	lea	rdi, [r11 + r10]
141303138: 4c 39 df                    	cmp	rdi, r11
14130313b: 0f 82 8b fe ff ff           	jb	0x141302fcc <.text+0x1301fcc>
141303141: 4e 8d 0c 88                 	lea	r9, [rax + 4*r9]
141303145: 4d 01 ca                    	add	r10, r9
141303148: 4d 39 ca                    	cmp	r10, r9
14130314b: 0f 82 7b fe ff ff           	jb	0x141302fcc <.text+0x1301fcc>
141303151: 49 ff c0                    	inc	r8
141303154: 4d 89 c2                    	mov	r10, r8
141303157: 49 83 e2 fc                 	and	r10, -0x4
14130315b: 4a 8d 0c 51                 	lea	rcx, [rcx + 2*r10]
14130315f: 66 41 0f 6e cd              	movd	xmm1, r13d
141303164: 45 01 d5                    	add	r13d, r10d
141303167: 66 0f 70 c9 00              	pshufd	xmm1, xmm1, 0x0         # xmm1 = xmm1[0,0,0,0]
14130316c: 66 0f fe 0d ec 17 7d 01     	paddd	xmm1, xmmword ptr [rip + 0x17d17ec] # 0x142ad4960
141303174: 0f 28 d0                    	movaps	xmm2, xmm0
141303177: 0f c6 d0 00                 	shufps	xmm2, xmm0, 0x0         # xmm2 = xmm2[0,0],xmm0[0,0]
14130317b: 0f 28 35 9e a2 91 01        	movaps	xmm6, xmmword ptr [rip + 0x191a29e] # 0x142c1d420
141303182: 0f 28 3d c7 b3 82 01        	movaps	xmm7, xmmword ptr [rip + 0x182b3c7] # 0x142b2e550
141303189: 0f 28 25 d0 7a 63 01        	movaps	xmm4, xmmword ptr [rip + 0x1637ad0] # 0x14293ac60
141303190: f3 0f 10 2d 50 6f 65 01     	movss	xmm5, dword ptr [rip + 0x1656f50] # 0x14295a0e8
141303198: 44 0f 28 15 f0 57 93 01     	movaps	xmm10, xmmword ptr [rip + 0x19357f0] # 0x142c38990
1413031a0: f2 44 0f 10 1d 37 db 77 01  	movsd	xmm11, qword ptr [rip + 0x177db37] # 0x142a80ce0
1413031a9: 45 0f 57 e4                 	xorps	xmm12, xmm12
1413031ad: 66 44 0f 6f 2d ba 17 7d 01  	movdqa	xmm13, xmmword ptr [rip + 0x17d17ba] # 0x142ad4970
1413031b6: 4d 89 d3                    	mov	r11, r10
1413031b9: 0f 1f 80 00 00 00 00        	nop	dword ptr [rax]
1413031c0: 66 44 0f 6f f9              	movdqa	xmm15, xmm1
1413031c5: 66 44 0f db 3d a2 79 81 01  	pand	xmm15, xmmword ptr [rip + 0x18179a2] # 0x142b1ab70
1413031ce: 66 44 0f eb 3d 59 16 88 01  	por	xmm15, xmmword ptr [rip + 0x1881659] # 0x142b84830
1413031d7: 66 44 0f 6f f1              	movdqa	xmm14, xmm1
1413031dc: 66 41 0f 72 d6 10           	psrld	xmm14, 0x10
1413031e2: 66 44 0f eb 35 25 a2 91 01  	por	xmm14, xmmword ptr [rip + 0x191a225] # 0x142c1d410
1413031eb: 44 0f 5c f6                 	subps	xmm14, xmm6
1413031ef: 45 0f 58 f7                 	addps	xmm14, xmm15
1413031f3: 44 0f 5e f2                 	divps	xmm14, xmm2
1413031f7: 44 0f 58 f7                 	addps	xmm14, xmm7
1413031fb: 44 0f 28 fc                 	movaps	xmm15, xmm4
1413031ff: 45 0f c6 fe 03              	shufps	xmm15, xmm14, 0x3       # xmm15 = xmm15[3,0],xmm14[0,0]
141303204: 44 0f 28 cd                 	movaps	xmm9, xmm5
141303208: 45 0f c6 cf 24              	shufps	xmm9, xmm15, 0x24       # xmm9 = xmm9[0,1],xmm15[2,0]
14130320d: f3 45 0f 7e fe              	movq	xmm15, xmm14            # xmm15 = xmm14[0],zero
141303212: 44 0f 28 c4                 	movaps	xmm8, xmm4
141303216: 45 0f c6 c6 23              	shufps	xmm8, xmm14, 0x23       # xmm8 = xmm8[3,0],xmm14[2,0]
14130321b: 0f 28 dd                    	movaps	xmm3, xmm5
14130321e: 41 0f c6 d8 24              	shufps	xmm3, xmm8, 0x24        # xmm3 = xmm3[0,1],xmm8[2,0]
141303223: 45 0f 28 c3                 	movaps	xmm8, xmm11
141303227: 45 0f c6 c6 f9              	shufps	xmm8, xmm14, 0xf9       # xmm8 = xmm8[1,2],xmm14[3,3]
14130322c: 45 0f c6 c0 d2              	shufps	xmm8, xmm8, 0xd2        # xmm8 = xmm8[2,0,1,3]
141303231: 45 0f 11 41 50              	movups	xmmword ptr [r9 + 0x50], xmm8
141303236: 45 0f 28 c3                 	movaps	xmm8, xmm11
14130323a: 45 0f c6 c6 59              	shufps	xmm8, xmm14, 0x59       # xmm8 = xmm8[1,2],xmm14[1,1]
14130323f: 66 45 0f 15 f4              	unpckhpd	xmm14, xmm12            # xmm14 = xmm14[1],xmm12[1]
141303244: 45 0f c6 f2 e2              	shufps	xmm14, xmm10, 0xe2      # xmm14 = xmm14[2,0],xmm10[2,3]
141303249: 45 0f 11 71 40              	movups	xmmword ptr [r9 + 0x40], xmm14
14130324e: 41 0f 11 59 30              	movups	xmmword ptr [r9 + 0x30], xmm3
141303253: 45 0f c6 c0 d2              	shufps	xmm8, xmm8, 0xd2        # xmm8 = xmm8[2,0,1,3]
141303258: 45 0f 11 41 20              	movups	xmmword ptr [r9 + 0x20], xmm8
14130325d: 45 0f c6 fa e2              	shufps	xmm15, xmm10, 0xe2      # xmm15 = xmm15[2,0],xmm10[2,3]
141303262: 45 0f 11 79 10              	movups	xmmword ptr [r9 + 0x10], xmm15
141303267: 45 0f 11 09                 	movups	xmmword ptr [r9], xmm9
14130326b: 66 41 0f fe cd              	paddd	xmm1, xmm13
141303270: 49 83 c1 60                 	add	r9, 0x60
141303274: 49 83 c3 fc                 	add	r11, -0x4
141303278: 0f 85 42 ff ff ff           	jne	0x1413031c0 <.text+0x13021c0>
14130327e: 4d 39 d0                    	cmp	r8, r10
141303281: 0f 85 45 fd ff ff           	jne	0x141302fcc <.text+0x1301fcc>
141303287: f6 45 36 01                 	test	byte ptr [rbp + 0x36], 0x1
14130328b: 0f 84 a5 00 00 00           	je	0x141303336 <.text+0x1302336>
141303291: 4c 8b 4d 20                 	mov	r9, qword ptr [rbp + 0x20]
141303295: 49 83 c1 48                 	add	r9, 0x48
141303299: 4d 8b 41 10                 	mov	r8, qword ptr [r9 + 0x10]
14130329d: 4c 89 45 f0                 	mov	qword ptr [rbp - 0x10], r8
1413032a1: 41 0f 10 01                 	movups	xmm0, xmmword ptr [r9]
1413032a5: 0f 11 45 e0                 	movups	xmmword ptr [rbp - 0x20], xmm0
1413032a9: 48 8b 7d b8                 	mov	rdi, qword ptr [rbp - 0x48]
1413032ad: 48 39 f7                    	cmp	rdi, rsi
1413032b0: 0f 84 80 00 00 00           	je	0x141303336 <.text+0x1302336>
1413032b6: 4c 8d 46 01                 	lea	r8, [rsi + 0x1]
1413032ba: f6 45 c0 01                 	test	byte ptr [rbp - 0x40], 0x1
1413032be: 74 20                       	je	0x1413032e0 <.text+0x13022e0>
1413032c0: 4c 8d 0c 76                 	lea	r9, [rsi + 2*rsi]
1413032c4: 4c 8d 14 49                 	lea	r10, [rcx + 2*rcx]
1413032c8: 46 8b 5c 8a 08              	mov	r11d, dword ptr [rdx + 4*r9 + 0x8]
1413032cd: 46 89 5c 90 08              	mov	dword ptr [rax + 4*r10 + 0x8], r11d
1413032d2: 4e 8b 0c 8a                 	mov	r9, qword ptr [rdx + 4*r9]
1413032d6: 4e 89 0c 90                 	mov	qword ptr [rax + 4*r10], r9
1413032da: 48 ff c1                    	inc	rcx
1413032dd: 4c 89 c6                    	mov	rsi, r8
1413032e0: 4c 39 c7                    	cmp	rdi, r8
1413032e3: 74 51                       	je	0x141303336 <.text+0x1302336>
1413032e5: 4c 8d 45 ec                 	lea	r8, [rbp - 0x14]
1413032e9: 4c 8d 0c 49                 	lea	r9, [rcx + 2*rcx]
1413032ed: 4a 8d 04 88                 	lea	rax, [rax + 4*r9]
1413032f1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
141303300: 48 83 c1 02                 	add	rcx, 0x2
141303304: 4c 8d 0c 76                 	lea	r9, [rsi + 2*rsi]
141303308: 46 8b 54 8a 08              	mov	r10d, dword ptr [rdx + 4*r9 + 0x8]
14130330d: 44 89 50 08                 	mov	dword ptr [rax + 0x8], r10d
141303311: 4e 8b 0c 8a                 	mov	r9, qword ptr [rdx + 4*r9]
141303315: 4c 89 08                    	mov	qword ptr [rax], r9
141303318: 4d 8b 08                    	mov	r9, qword ptr [r8]
14130331b: 4c 89 48 0c                 	mov	qword ptr [rax + 0xc], r9
14130331f: 45 8b 48 08                 	mov	r9d, dword ptr [r8 + 0x8]
141303323: 44 89 48 14                 	mov	dword ptr [rax + 0x14], r9d
141303327: 48 83 c0 18                 	add	rax, 0x18
14130332b: be 02 00 00 00              	mov	esi, 0x2
141303330: 48 83 ff 02                 	cmp	rdi, 0x2
141303334: 75 ca                       	jne	0x141303300 <.text+0x1302300>
141303336: 48 89 4d 10                 	mov	qword ptr [rbp + 0x10], rcx
14130333a: 48 8b 45 b0                 	mov	rax, qword ptr [rbp - 0x50]
14130333e: 48 89 48 10                 	mov	qword ptr [rax + 0x10], rcx
141303342: 0f 10 45 00                 	movups	xmm0, xmmword ptr [rbp]
141303346: 0f 11 00                    	movups	xmmword ptr [rax], xmm0
141303349: 0f 28 75 40                 	movaps	xmm6, xmmword ptr [rbp + 0x40]
14130334d: 0f 28 7d 50                 	movaps	xmm7, xmmword ptr [rbp + 0x50]
141303351: 44 0f 28 45 60              	movaps	xmm8, xmmword ptr [rbp + 0x60]
141303356: 44 0f 28 4d 70              	movaps	xmm9, xmmword ptr [rbp + 0x70]
14130335b: 44 0f 28 95 80 00 00 00     	movaps	xmm10, xmmword ptr [rbp + 0x80]
141303363: 44 0f 28 9d 90 00 00 00     	movaps	xmm11, xmmword ptr [rbp + 0x90]
14130336b: 44 0f 28 a5 a0 00 00 00     	movaps	xmm12, xmmword ptr [rbp + 0xa0]
141303373: 44 0f 28 ad b0 00 00 00     	movaps	xmm13, xmmword ptr [rbp + 0xb0]
14130337b: 44 0f 28 b5 c0 00 00 00     	movaps	xmm14, xmmword ptr [rbp + 0xc0]
141303383: 44 0f 28 bd d0 00 00 00     	movaps	xmm15, xmmword ptr [rbp + 0xd0]
14130338b: 48 81 c4 68 01 00 00        	add	rsp, 0x168
141303392: 5b                          	pop	rbx
141303393: 5f                          	pop	rdi
141303394: 5e                          	pop	rsi
141303395: 41 5c                       	pop	r12
141303397: 41 5d                       	pop	r13
141303399: 41 5e                       	pop	r14
14130339b: 41 5f                       	pop	r15
14130339d: 5d                          	pop	rbp
14130339e: c3                          	ret
14130339f: 48 c7 44 24 20 0c 00 00 00  	mov	qword ptr [rsp + 0x20], 0xc
1413033a8: 48 89 e9                    	mov	rcx, rbp
1413033ab: 41 b9 04 00 00 00           	mov	r9d, 0x4
1413033b1: 31 d2                       	xor	edx, edx
1413033b3: 4d 89 f0                    	mov	r8, r14
1413033b6: e8 a5 66 5a 01              	call	0x1428a9a60 <.text+0x28a8a60>
1413033bb: 48 8b 45 08                 	mov	rax, qword ptr [rbp + 0x8]
1413033bf: 48 8b 4d 10                 	mov	rcx, qword ptr [rbp + 0x10]
1413033c3: 48 8d 55 e0                 	lea	rdx, [rbp - 0x20]
1413033c7: f6 45 37 01                 	test	byte ptr [rbp + 0x37], 0x1
1413033cb: 0f 85 37 fb ff ff           	jne	0x141302f08 <.text+0x1301f08>
1413033d1: e9 c0 fb ff ff              	jmp	0x141302f96 <.text+0x1301f96>
1413033d6: 48 8d 05 cb 56 93 01        	lea	rax, [rip + 0x19356cb]  # 0x142c38aa8
1413033dd: 48 89 45 d0                 	mov	qword ptr [rbp - 0x30], rax
1413033e1: 48 c7 45 d8 01 00 00 00     	mov	qword ptr [rbp - 0x28], 0x1
1413033e9: 48 c7 45 e0 08 00 00 00     	mov	qword ptr [rbp - 0x20], 0x8
1413033f1: 0f 57 c0                    	xorps	xmm0, xmm0
1413033f4: 0f 11 45 e8                 	movups	xmmword ptr [rbp - 0x18], xmm0
1413033f8: 48 8d 4d d0                 	lea	rcx, [rbp - 0x30]
1413033fc: 48 8b 55 28                 	mov	rdx, qword ptr [rbp + 0x28]
141303400: e8 8b 5f 5d 01              	call	0x1428d9390 <.text+0x28d8390>
141303405: 4c 89 e9                    	mov	rcx, r13
141303408: 4c 89 e2                    	mov	rdx, r12
14130340b: 4c 8b 45 28                 	mov	r8, qword ptr [rbp + 0x28]
14130340f: e8 cf 5b 5d 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
141303414: 48 8d 05 8d 56 93 01        	lea	rax, [rip + 0x193568d]  # 0x142c38aa8
14130341b: 48 89 45 d0                 	mov	qword ptr [rbp - 0x30], rax
14130341f: 48 c7 45 d8 01 00 00 00     	mov	qword ptr [rbp - 0x28], 0x1
141303427: 48 c7 45 e0 08 00 00 00     	mov	qword ptr [rbp - 0x20], 0x8
14130342f: 0f 57 c0                    	xorps	xmm0, xmm0
141303432: 0f 11 45 e8                 	movups	xmmword ptr [rbp - 0x18], xmm0
141303436: 48 8d 4d d0                 	lea	rcx, [rbp - 0x30]
14130343a: 48 8b 55 28                 	mov	rdx, qword ptr [rbp + 0x28]
14130343e: e8 4d 5f 5d 01              	call	0x1428d9390 <.text+0x28d8390>
141303443: 0f 0b                       	ud2
141303445: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
141303450: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141303455: 55                          	push	rbp
141303456: 41 57                       	push	r15
141303458: 41 56                       	push	r14
14130345a: 41 55                       	push	r13
14130345c: 41 54                       	push	r12
14130345e: 56                          	push	rsi
14130345f: 57                          	push	rdi
141303460: 53                          	push	rbx
141303461: 48 81 ec c8 00 00 00        	sub	rsp, 0xc8
141303468: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14130346f: 44 0f 29 7c 24 20           	movaps	xmmword ptr [rsp + 0x20], xmm15
141303475: 44 0f 29 74 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm14
14130347b: 66 44 0f 7f 6c 24 40        	movdqa	xmmword ptr [rsp + 0x40], xmm13
141303482: 44 0f 29 64 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm12
141303488: 44 0f 29 5c 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm11
14130348e: 44 0f 29 54 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm10
141303494: 44 0f 29 8c 24 80 00 00 00  	movaps	xmmword ptr [rsp + 0x80], xmm9
14130349d: 44 0f 29 84 24 90 00 00 00  	movaps	xmmword ptr [rsp + 0x90], xmm8
1413034a6: 0f 29 bc 24 a0 00 00 00     	movaps	xmmword ptr [rsp + 0xa0], xmm7
1413034ae: 0f 29 b4 24 b0 00 00 00     	movaps	xmmword ptr [rsp + 0xb0], xmm6
1413034b6: 48 8b 45 00                 	mov	rax, qword ptr [rbp]
1413034ba: 48 85 c0                    	test	rax, rax
1413034bd: 74 17                       	je	0x1413034d6 <.text+0x13024d6>
1413034bf: 48 8b 4d 08                 	mov	rcx, qword ptr [rbp + 0x8]
1413034c3: 48 c1 e0 02                 	shl	rax, 0x2
1413034c7: 48 8d 14 40                 	lea	rdx, [rax + 2*rax]
1413034cb: 41 b8 04 00 00 00           	mov	r8d, 0x4
1413034d1: e8 4a 07 31 ff              	call	0x140613c20 <.text+0x612c20>
1413034d6: 0f 28 b4 24 b0 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0xb0]
1413034de: 0f 28 bc 24 a0 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0xa0]
1413034e6: 44 0f 28 84 24 90 00 00 00  	movaps	xmm8, xmmword ptr [rsp + 0x90]
1413034ef: 44 0f 28 8c 24 80 00 00 00  	movaps	xmm9, xmmword ptr [rsp + 0x80]
1413034f8: 44 0f 28 54 24 70           	movaps	xmm10, xmmword ptr [rsp + 0x70]
1413034fe: 44 0f 28 5c 24 60           	movaps	xmm11, xmmword ptr [rsp + 0x60]
141303504: 44 0f 28 64 24 50           	movaps	xmm12, xmmword ptr [rsp + 0x50]
14130350a: 44 0f 28 6c 24 40           	movaps	xmm13, xmmword ptr [rsp + 0x40]
141303510: 44 0f 28 74 24 30           	movaps	xmm14, xmmword ptr [rsp + 0x30]
141303516: 44 0f 28 7c 24 20           	movaps	xmm15, xmmword ptr [rsp + 0x20]
14130351c: 48 81 c4 c8 00 00 00        	add	rsp, 0xc8
141303523: 5b                          	pop	rbx
141303524: 5f                          	pop	rdi
141303525: 5e                          	pop	rsi
141303526: 41 5c                       	pop	r12
141303528: 41 5d                       	pop	r13
14130352a: 41 5e                       	pop	r14
14130352c: 41 5f                       	pop	r15
14130352e: 5d                          	pop	rbp
14130352f: c3                          	ret
