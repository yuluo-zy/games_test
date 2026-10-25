
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14205eff0: 55                          	push	rbp
14205eff1: 41 57                       	push	r15
14205eff3: 41 56                       	push	r14
14205eff5: 41 55                       	push	r13
14205eff7: 41 54                       	push	r12
14205eff9: 56                          	push	rsi
14205effa: 57                          	push	rdi
14205effb: 53                          	push	rbx
14205effc: 48 81 ec 18 01 00 00        	sub	rsp, 0x118
14205f003: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
14205f00b: 48 c7 85 90 00 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x90], -0x2
14205f016: 41 8b 01                    	mov	eax, dword ptr [r9]
14205f019: 44 8d 50 fd                 	lea	r10d, [rax - 0x3]
14205f01d: 49 89 c3                    	mov	r11, rax
14205f020: 49 83 c3 fe                 	add	r11, -0x2
14205f024: 31 f6                       	xor	esi, esi
14205f026: 41 83 fa 03                 	cmp	r10d, 0x3
14205f02a: 49 0f 42 f3                 	cmovb	rsi, r11
14205f02e: 4c 8d 15 27 07 f7 00        	lea	r10, [rip + 0xf70727]   # 0x142fcf75c
14205f035: 4d 63 1c b2                 	movsxd	r11, dword ptr [r10 + 4*rsi]
14205f039: 4d 01 d3                    	add	r11, r10
14205f03c: 41 ff e3                    	jmp	r11
14205f03f: 83 f8 02                    	cmp	eax, 0x2
14205f042: 0f 85 50 01 00 00           	jne	0x14205f198 <.text+0x205e198>
14205f048: 45 31 ff                    	xor	r15d, r15d
14205f04b: e9 8e 02 00 00              	jmp	0x14205f2de <.text+0x205e2de>
14205f050: 4c 8b 42 10                 	mov	r8, qword ptr [rdx + 0x10]
14205f054: 41 8b 41 04                 	mov	eax, dword ptr [r9 + 0x4]
14205f058: 4d 8b 48 10                 	mov	r9, qword ptr [r8 + 0x10]
14205f05c: 49 39 c1                    	cmp	r9, rax
14205f05f: 0f 86 37 04 00 00           	jbe	0x14205f49c <.text+0x205e49c>
14205f065: 4d 8b 48 08                 	mov	r9, qword ptr [r8 + 0x8]
14205f069: 4d 8b 50 40                 	mov	r10, qword ptr [r8 + 0x40]
14205f06d: 45 0f b7 0c 81              	movzx	r9d, word ptr [r9 + 4*rax]
14205f072: 4d 39 ca                    	cmp	r10, r9
14205f075: 0f 86 33 04 00 00           	jbe	0x14205f4ae <.text+0x205e4ae>
14205f07b: 49 8b 40 38                 	mov	rax, qword ptr [r8 + 0x38]
14205f07f: 41 c1 e1 06                 	shl	r9d, 0x6
14205f083: 4e 8b 04 08                 	mov	r8, qword ptr [rax + r9]
14205f087: 4d 8d 50 fe                 	lea	r10, [r8 - 0x2]
14205f08b: 49 83 fa 02                 	cmp	r10, 0x2
14205f08f: 0f 83 f6 03 00 00           	jae	0x14205f48b <.text+0x205e48b>
14205f095: 48 8b 52 08                 	mov	rdx, qword ptr [rdx + 0x8]
14205f099: 4c 8b 82 48 06 00 00        	mov	r8, qword ptr [rdx + 0x648]
14205f0a0: 4c 01 c8                    	add	rax, r9
14205f0a3: 0f b7 50 1c                 	movzx	edx, word ptr [rax + 0x1c]
14205f0a7: 66 41 3b 90 50 0a 00 00     	cmp	dx, word ptr [r8 + 0xa50]
14205f0af: 0f 85 b7 02 00 00           	jne	0x14205f36c <.text+0x205e36c>
14205f0b5: 80 78 1e 01                 	cmp	byte ptr [rax + 0x1e], 0x1
14205f0b9: 0f 85 da 02 00 00           	jne	0x14205f399 <.text+0x205e399>
14205f0bf: 48 8b 50 10                 	mov	rdx, qword ptr [rax + 0x10]
14205f0c3: 44 8b 42 0c                 	mov	r8d, dword ptr [rdx + 0xc]
14205f0c7: 44 3b 40 18                 	cmp	r8d, dword ptr [rax + 0x18]
14205f0cb: 0f 85 1f 04 00 00           	jne	0x14205f4f0 <.text+0x205e4f0>
14205f0d1: 48 8b 42 30                 	mov	rax, qword ptr [rdx + 0x30]
14205f0d5: 48 89 41 08                 	mov	qword ptr [rcx + 0x8], rax
14205f0d9: 48 c7 41 10 00 00 00 00     	mov	qword ptr [rcx + 0x10], 0x0
14205f0e1: 48 c7 41 18 ff ff ff ff     	mov	qword ptr [rcx + 0x18], -0x1
14205f0e9: c7 01 05 00 00 00           	mov	dword ptr [rcx], 0x5
14205f0ef: e9 61 02 00 00              	jmp	0x14205f355 <.text+0x205e355>
14205f0f4: 4c 8b 42 10                 	mov	r8, qword ptr [rdx + 0x10]
14205f0f8: 41 8b 41 04                 	mov	eax, dword ptr [r9 + 0x4]
14205f0fc: 4d 8b 48 10                 	mov	r9, qword ptr [r8 + 0x10]
14205f100: 49 39 c1                    	cmp	r9, rax
14205f103: 0f 86 93 03 00 00           	jbe	0x14205f49c <.text+0x205e49c>
14205f109: 4d 8b 48 08                 	mov	r9, qword ptr [r8 + 0x8]
14205f10d: 4d 8b 50 40                 	mov	r10, qword ptr [r8 + 0x40]
14205f111: 45 0f b7 0c 81              	movzx	r9d, word ptr [r9 + 4*rax]
14205f116: 4d 39 ca                    	cmp	r10, r9
14205f119: 0f 86 8f 03 00 00           	jbe	0x14205f4ae <.text+0x205e4ae>
14205f11f: 49 8b 40 38                 	mov	rax, qword ptr [r8 + 0x38]
14205f123: 41 c1 e1 06                 	shl	r9d, 0x6
14205f127: 4e 8b 04 08                 	mov	r8, qword ptr [rax + r9]
14205f12b: 4d 8d 50 fe                 	lea	r10, [r8 - 0x2]
14205f12f: 49 83 fa 02                 	cmp	r10, 0x2
14205f133: 0f 83 52 03 00 00           	jae	0x14205f48b <.text+0x205e48b>
14205f139: 48 8b 52 08                 	mov	rdx, qword ptr [rdx + 0x8]
14205f13d: 4c 8b 82 48 06 00 00        	mov	r8, qword ptr [rdx + 0x648]
14205f144: 4c 01 c8                    	add	rax, r9
14205f147: 0f b7 50 1c                 	movzx	edx, word ptr [rax + 0x1c]
14205f14b: 66 41 3b 90 50 0a 00 00     	cmp	dx, word ptr [r8 + 0xa50]
14205f153: 0f 85 13 02 00 00           	jne	0x14205f36c <.text+0x205e36c>
14205f159: 80 78 1e 01                 	cmp	byte ptr [rax + 0x1e], 0x1
14205f15d: 0f 85 36 02 00 00           	jne	0x14205f399 <.text+0x205e399>
14205f163: 48 8b 50 10                 	mov	rdx, qword ptr [rax + 0x10]
14205f167: 44 8b 42 0c                 	mov	r8d, dword ptr [rdx + 0xc]
14205f16b: 44 3b 40 18                 	cmp	r8d, dword ptr [rax + 0x18]
14205f16f: 0f 85 87 03 00 00           	jne	0x14205f4fc <.text+0x205e4fc>
14205f175: 48 8b 42 30                 	mov	rax, qword ptr [rdx + 0x30]
14205f179: 48 89 41 08                 	mov	qword ptr [rcx + 0x8], rax
14205f17d: 48 c7 41 10 00 00 00 00     	mov	qword ptr [rcx + 0x10], 0x0
14205f185: 48 c7 41 18 ff ff ff ff     	mov	qword ptr [rcx + 0x18], -0x1
14205f18d: c7 01 06 00 00 00           	mov	dword ptr [rcx], 0x6
14205f193: e9 bd 01 00 00              	jmp	0x14205f355 <.text+0x205e355>
14205f198: 45 0f b6 51 38              	movzx	r10d, byte ptr [r9 + 0x38]
14205f19d: 4d 89 cf                    	mov	r15, r9
14205f1a0: 41 f6 c2 01                 	test	r10b, 0x1
14205f1a4: 0f 84 34 01 00 00           	je	0x14205f2de <.text+0x205e2de>
14205f1aa: 4d 8d 59 04                 	lea	r11, [r9 + 0x4]
14205f1ae: 49 8d 71 39                 	lea	rsi, [r9 + 0x39]
14205f1b2: 0f b6 5e 02                 	movzx	ebx, byte ptr [rsi + 0x2]
14205f1b6: 88 5d 1b                    	mov	byte ptr [rbp + 0x1b], bl
14205f1b9: 0f b7 36                    	movzx	esi, word ptr [rsi]
14205f1bc: 66 89 75 19                 	mov	word ptr [rbp + 0x19], si
14205f1c0: 41 0f 10 03                 	movups	xmm0, xmmword ptr [r11]
14205f1c4: 41 0f 10 4b 10              	movups	xmm1, xmmword ptr [r11 + 0x10]
14205f1c9: 41 0f 10 53 20              	movups	xmm2, xmmword ptr [r11 + 0x20]
14205f1ce: 0f 11 45 e4                 	movups	xmmword ptr [rbp - 0x1c], xmm0
14205f1d2: 0f 11 4d f4                 	movups	xmmword ptr [rbp - 0xc], xmm1
14205f1d6: 0f 11 55 04                 	movups	xmmword ptr [rbp + 0x4], xmm2
14205f1da: 45 8b 5b 30                 	mov	r11d, dword ptr [r11 + 0x30]
14205f1de: 44 89 5d 14                 	mov	dword ptr [rbp + 0x14], r11d
14205f1e2: 89 45 e0                    	mov	dword ptr [rbp - 0x20], eax
14205f1e5: 44 88 55 18                 	mov	byte ptr [rbp + 0x18], r10b
14205f1e9: 48 8b 72 10                 	mov	rsi, qword ptr [rdx + 0x10]
14205f1ed: 41 8b 41 3c                 	mov	eax, dword ptr [r9 + 0x3c]
14205f1f1: 4c 8b 56 10                 	mov	r10, qword ptr [rsi + 0x10]
14205f1f5: 49 39 c2                    	cmp	r10, rax
14205f1f8: 0f 86 6f 03 00 00           	jbe	0x14205f56d <.text+0x205e56d>
14205f1fe: 4c 8b 56 08                 	mov	r10, qword ptr [rsi + 0x8]
14205f202: 4c 8b 5e 40                 	mov	r11, qword ptr [rsi + 0x40]
14205f206: 45 0f b7 14 82              	movzx	r10d, word ptr [r10 + 4*rax]
14205f20b: 4d 39 d3                    	cmp	r11, r10
14205f20e: 0f 86 6b 03 00 00           	jbe	0x14205f57f <.text+0x205e57f>
14205f214: 48 8b 46 38                 	mov	rax, qword ptr [rsi + 0x38]
14205f218: 41 c1 e2 06                 	shl	r10d, 0x6
14205f21c: 4e 8b 1c 10                 	mov	r11, qword ptr [rax + r10]
14205f220: 49 83 fb 02                 	cmp	r11, 0x2
14205f224: 0f 83 30 03 00 00           	jae	0x14205f55a <.text+0x205e55a>
14205f22a: 4c 01 d0                    	add	rax, r10
14205f22d: 4d 8b 90 48 06 00 00        	mov	r10, qword ptr [r8 + 0x648]
14205f234: 44 0f b7 58 1c              	movzx	r11d, word ptr [rax + 0x1c]
14205f239: 66 45 3b 9a 50 0a 00 00     	cmp	r11w, word ptr [r10 + 0xa50]
14205f241: 0f 85 7f 01 00 00           	jne	0x14205f3c6 <.text+0x205e3c6>
14205f247: 80 78 1e 00                 	cmp	byte ptr [rax + 0x1e], 0x0
14205f24b: 0f 85 a5 01 00 00           	jne	0x14205f3f6 <.text+0x205e3f6>
14205f251: 4c 8b 50 10                 	mov	r10, qword ptr [rax + 0x10]
14205f255: 45 8b 5a 0c                 	mov	r11d, dword ptr [r10 + 0xc]
14205f259: 44 3b 58 18                 	cmp	r11d, dword ptr [rax + 0x18]
14205f25d: 0f 85 2e 03 00 00           	jne	0x14205f591 <.text+0x205e591>
14205f263: 41 0f b6 5a 2c              	movzx	ebx, byte ptr [r10 + 0x2c]
14205f268: 49 83 c2 2c                 	add	r10, 0x2c
14205f26c: 4c 8d 5d 10                 	lea	r11, [rbp + 0x10]
14205f270: 8b 45 10                    	mov	eax, dword ptr [rbp + 0x10]
14205f273: 28 c3                       	sub	bl, al
14205f275: 0f 82 22 03 00 00           	jb	0x14205f59d <.text+0x205e59d>
14205f27b: 0f b6 f3                    	movzx	esi, bl
14205f27e: f6 45 00 01                 	test	byte ptr [rbp], 0x1
14205f282: 74 13                       	je	0x14205f297 <.text+0x205e297>
14205f284: 8b 7d 04                    	mov	edi, dword ptr [rbp + 0x4]
14205f287: 89 bd 8c 00 00 00           	mov	dword ptr [rbp + 0x8c], edi
14205f28d: 39 f7                       	cmp	edi, esi
14205f28f: 89 fe                       	mov	esi, edi
14205f291: 0f 83 8c 01 00 00           	jae	0x14205f423 <.text+0x205e423>
14205f297: 01 c6                       	add	esi, eax
14205f299: 4c 8d 55 e0                 	lea	r10, [rbp - 0x20]
14205f29d: 4c 89 55 38                 	mov	qword ptr [rbp + 0x38], r10
14205f2a1: 48 89 55 40                 	mov	qword ptr [rbp + 0x40], rdx
14205f2a5: 4c 89 4d 48                 	mov	qword ptr [rbp + 0x48], r9
14205f2a9: 4c 89 45 50                 	mov	qword ptr [rbp + 0x50], r8
14205f2ad: 89 45 58                    	mov	dword ptr [rbp + 0x58], eax
14205f2b0: 89 75 5c                    	mov	dword ptr [rbp + 0x5c], esi
14205f2b3: 48 8d 41 08                 	lea	rax, [rcx + 0x8]
14205f2b7: 4c 8d 05 72 00 f7 00        	lea	r8, [rip + 0xf70072]    # 0x142fcf330
14205f2be: 48 8d 55 38                 	lea	rdx, [rbp + 0x38]
14205f2c2: 48 89 ce                    	mov	rsi, rcx
14205f2c5: 48 89 c1                    	mov	rcx, rax
14205f2c8: 4c 89 cf                    	mov	rdi, r9
14205f2cb: e8 e0 77 85 ff              	call	0x1418b6ab0 <.text+0x18b5ab0>
14205f2d0: 48 89 f1                    	mov	rcx, rsi
14205f2d3: 0f b6 47 40                 	movzx	eax, byte ptr [rdi + 0x40]
14205f2d7: 83 c8 02                    	or	eax, 0x2
14205f2da: 89 06                       	mov	dword ptr [rsi], eax
14205f2dc: eb 77                       	jmp	0x14205f355 <.text+0x205e355>
14205f2de: 48 89 cb                    	mov	rbx, rcx
14205f2e1: 4d 89 ce                    	mov	r14, r9
14205f2e4: 41 8b 71 3c                 	mov	esi, dword ptr [r9 + 0x3c]
14205f2e8: 48 89 d1                    	mov	rcx, rdx
14205f2eb: 49 89 d5                    	mov	r13, rdx
14205f2ee: 89 f2                       	mov	edx, esi
14205f2f0: 4c 89 c7                    	mov	rdi, r8
14205f2f3: e8 98 7a 69 ff              	call	0x1416f6d90 <.text+0x16f5d90>
14205f2f8: 48 8d 4d b0                 	lea	rcx, [rbp - 0x50]
14205f2fc: 89 c2                       	mov	edx, eax
14205f2fe: e8 cd 1a bb fe              	call	0x140c10dd0 <.text+0xc0fdd0>
14205f303: 44 8b 65 b8                 	mov	r12d, dword ptr [rbp - 0x48]
14205f307: 49 8b 55 10                 	mov	rdx, qword ptr [r13 + 0x10]
14205f30b: 4c 89 7c 24 20              	mov	qword ptr [rsp + 0x20], r15
14205f310: 48 8d 4d 38                 	lea	rcx, [rbp + 0x38]
14205f314: 49 89 f8                    	mov	r8, rdi
14205f317: 41 89 f1                    	mov	r9d, esi
14205f31a: e8 41 87 b8 fe              	call	0x140be7a60 <.text+0xbe6a60>
14205f31f: 83 7d 38 0b                 	cmp	dword ptr [rbp + 0x38], 0xb
14205f323: 0f 85 df 01 00 00           	jne	0x14205f508 <.text+0x205e508>
14205f329: 41 80 7e 40 00              	cmp	byte ptr [r14 + 0x40], 0x0
14205f32e: 48 8b 45 40                 	mov	rax, qword ptr [rbp + 0x40]
14205f332: 48 89 d9                    	mov	rcx, rbx
14205f335: 48 c7 43 08 00 00 00 00     	mov	qword ptr [rbx + 0x8], 0x0
14205f33d: 48 89 43 10                 	mov	qword ptr [rbx + 0x10], rax
14205f341: 4c 89 63 18                 	mov	qword ptr [rbx + 0x18], r12
14205f345: 74 08                       	je	0x14205f34f <.text+0x205e34f>
14205f347: c7 01 01 00 00 00           	mov	dword ptr [rcx], 0x1
14205f34d: eb 06                       	jmp	0x14205f355 <.text+0x205e355>
14205f34f: c7 01 00 00 00 00           	mov	dword ptr [rcx], 0x0
14205f355: 48 89 c8                    	mov	rax, rcx
14205f358: 48 81 c4 18 01 00 00        	add	rsp, 0x118
14205f35f: 5b                          	pop	rbx
14205f360: 5f                          	pop	rdi
14205f361: 5e                          	pop	rsi
14205f362: 41 5c                       	pop	r12
14205f364: 41 5d                       	pop	r13
14205f366: 41 5e                       	pop	r14
14205f368: 41 5f                       	pop	r15
14205f36a: 5d                          	pop	rbp
14205f36b: c3                          	ret
14205f36c: 48 c7 45 e0 00 00 00 00     	mov	qword ptr [rbp - 0x20], 0x0
14205f374: 48 83 c0 1c                 	add	rax, 0x1c
14205f378: 49 81 c0 50 0a 00 00        	add	r8, 0xa50
14205f37f: 48 8d 0d 1a 2f ca 00        	lea	rcx, [rip + 0xca2f1a]   # 0x142d022a0
14205f386: 48 89 4c 24 20              	mov	qword ptr [rsp + 0x20], rcx
14205f38b: 4c 8d 4d e0                 	lea	r9, [rbp - 0x20]
14205f38f: 31 c9                       	xor	ecx, ecx
14205f391: 48 89 c2                    	mov	rdx, rax
14205f394: e8 7d b4 86 00              	call	0x1428ca816 <.text+0x28c9816>
14205f399: 48 c7 45 e0 00 00 00 00     	mov	qword ptr [rbp - 0x20], 0x0
14205f3a1: 48 83 c0 1e                 	add	rax, 0x1e
14205f3a5: 48 8d 0d 14 2f ca 00        	lea	rcx, [rip + 0xca2f14]   # 0x142d022c0
14205f3ac: 48 89 4c 24 20              	mov	qword ptr [rsp + 0x20], rcx
14205f3b1: 4c 8d 05 21 2f ca 00        	lea	r8, [rip + 0xca2f21]    # 0x142d022d9
14205f3b8: 4c 8d 4d e0                 	lea	r9, [rbp - 0x20]
14205f3bc: 31 c9                       	xor	ecx, ecx
14205f3be: 48 89 c2                    	mov	rdx, rax
14205f3c1: e8 3c b5 86 00              	call	0x1428ca902 <.text+0x28c9902>
14205f3c6: 48 c7 45 38 00 00 00 00     	mov	qword ptr [rbp + 0x38], 0x0
14205f3ce: 48 83 c0 1c                 	add	rax, 0x1c
14205f3d2: 49 81 c2 50 0a 00 00        	add	r10, 0xa50
14205f3d9: 48 8d 0d c0 2e ca 00        	lea	rcx, [rip + 0xca2ec0]   # 0x142d022a0
14205f3e0: 48 89 4c 24 20              	mov	qword ptr [rsp + 0x20], rcx
14205f3e5: 4c 8d 4d 38                 	lea	r9, [rbp + 0x38]
14205f3e9: 31 c9                       	xor	ecx, ecx
14205f3eb: 48 89 c2                    	mov	rdx, rax
14205f3ee: 4d 89 d0                    	mov	r8, r10
14205f3f1: e8 20 b4 86 00              	call	0x1428ca816 <.text+0x28c9816>
14205f3f6: 48 c7 45 38 00 00 00 00     	mov	qword ptr [rbp + 0x38], 0x0
14205f3fe: 48 83 c0 1e                 	add	rax, 0x1e
14205f402: 48 8d 0d b7 2e ca 00        	lea	rcx, [rip + 0xca2eb7]   # 0x142d022c0
14205f409: 48 89 4c 24 20              	mov	qword ptr [rsp + 0x20], rcx
14205f40e: 4c 8d 05 a3 2e ca 00        	lea	r8, [rip + 0xca2ea3]    # 0x142d022b8
14205f415: 4c 8d 4d 38                 	lea	r9, [rbp + 0x38]
14205f419: 31 c9                       	xor	ecx, ecx
14205f41b: 48 89 c2                    	mov	rdx, rax
14205f41e: e8 df b4 86 00              	call	0x1428ca902 <.text+0x28c9902>
14205f423: 48 8d 85 8c 00 00 00        	lea	rax, [rbp + 0x8c]
14205f42a: 48 89 45 38                 	mov	qword ptr [rbp + 0x38], rax
14205f42e: 48 8d 05 5b 1e fd fd        	lea	rax, [rip - 0x202e1a5]  # 0x140031290 <.text+0x30290>
14205f435: 48 89 45 40                 	mov	qword ptr [rbp + 0x40], rax
14205f439: 4c 89 55 48                 	mov	qword ptr [rbp + 0x48], r10
14205f43d: 48 8d 0d 9c 7a 0f fe        	lea	rcx, [rip - 0x1f08564]  # 0x140156ee0 <.text+0x155ee0>
14205f444: 48 89 4d 50                 	mov	qword ptr [rbp + 0x50], rcx
14205f448: 4c 89 5d 58                 	mov	qword ptr [rbp + 0x58], r11
14205f44c: 48 89 45 60                 	mov	qword ptr [rbp + 0x60], rax
14205f450: 48 8d 05 c9 00 f7 00        	lea	rax, [rip + 0xf700c9]   # 0x142fcf520
14205f457: 48 89 45 b0                 	mov	qword ptr [rbp - 0x50], rax
14205f45b: 48 c7 45 b8 04 00 00 00     	mov	qword ptr [rbp - 0x48], 0x4
14205f463: 48 c7 45 d0 00 00 00 00     	mov	qword ptr [rbp - 0x30], 0x0
14205f46b: 48 8d 45 38                 	lea	rax, [rbp + 0x38]
14205f46f: 48 89 45 c0                 	mov	qword ptr [rbp - 0x40], rax
14205f473: 48 c7 45 c8 03 00 00 00     	mov	qword ptr [rbp - 0x38], 0x3
14205f47b: 48 8d 15 de 00 f7 00        	lea	rdx, [rip + 0xf700de]   # 0x142fcf560
14205f482: 48 8d 4d b0                 	lea	rcx, [rbp - 0x50]
14205f486: e8 05 9f 87 00              	call	0x1428d9390 <.text+0x28d8390>
14205f48b: 41 83 f8 02                 	cmp	r8d, 0x2
14205f48f: 72 06                       	jb	0x14205f497 <.text+0x205e497>
14205f491: 41 83 f8 04                 	cmp	r8d, 0x4
14205f495: 75 41                       	jne	0x14205f4d8 <.text+0x205e4d8>
14205f497: e8 c4 b5 86 00              	call	0x1428caa60 <.text+0x28c9a60>
14205f49c: 4c 8d 05 65 48 ab 00        	lea	r8, [rip + 0xab4865]    # 0x142b13d08
14205f4a3: 48 89 c1                    	mov	rcx, rax
14205f4a6: 4c 89 ca                    	mov	rdx, r9
14205f4a9: e8 6a a0 87 00              	call	0x1428d9518 <.text+0x28d8518>
14205f4ae: 4c 8d 05 6b 48 ab 00        	lea	r8, [rip + 0xab486b]    # 0x142b13d20
14205f4b5: 4c 89 c9                    	mov	rcx, r9
14205f4b8: 4c 89 d2                    	mov	rdx, r10
14205f4bb: e8 58 a0 87 00              	call	0x1428d9518 <.text+0x28d8518>
14205f4c0: 48 8d 0d f9 00 f7 00        	lea	rcx, [rip + 0xf700f9]   # 0x142fcf5c0
14205f4c7: 4c 8d 05 0a 01 f7 00        	lea	r8, [rip + 0xf7010a]    # 0x142fcf5d8
14205f4ce: ba 13 00 00 00              	mov	edx, 0x13
14205f4d3: e8 58 9f 87 00              	call	0x1428d9430 <.text+0x28d8430>
14205f4d8: 48 8d 0d e9 0c ab 00        	lea	rcx, [rip + 0xab0ce9]   # 0x142b101c8
14205f4df: 4c 8d 05 52 48 ab 00        	lea	r8, [rip + 0xab4852]    # 0x142b13d38
14205f4e6: ba 28 00 00 00              	mov	edx, 0x28
14205f4eb: e8 40 9f 87 00              	call	0x1428d9430 <.text+0x28d8430>
14205f4f0: 48 8d 0d b1 00 f7 00        	lea	rcx, [rip + 0xf700b1]   # 0x142fcf5a8
14205f4f7: e8 14 9e 87 00              	call	0x1428d9310 <.text+0x28d8310>
14205f4fc: 48 8d 0d 8d 00 f7 00        	lea	rcx, [rip + 0xf7008d]   # 0x142fcf590
14205f503: e8 08 9e 87 00              	call	0x1428d9310 <.text+0x28d8310>
14205f508: 0f 10 45 78                 	movups	xmm0, xmmword ptr [rbp + 0x78]
14205f50c: 0f 29 45 20                 	movaps	xmmword ptr [rbp + 0x20], xmm0
14205f510: 0f 10 45 38                 	movups	xmm0, xmmword ptr [rbp + 0x38]
14205f514: 0f 10 4d 48                 	movups	xmm1, xmmword ptr [rbp + 0x48]
14205f518: 0f 10 55 58                 	movups	xmm2, xmmword ptr [rbp + 0x58]
14205f51c: 0f 10 5d 68                 	movups	xmm3, xmmword ptr [rbp + 0x68]
14205f520: 0f 29 5d 10                 	movaps	xmmword ptr [rbp + 0x10], xmm3
14205f524: 0f 29 55 00                 	movaps	xmmword ptr [rbp], xmm2
14205f528: 0f 29 4d f0                 	movaps	xmmword ptr [rbp - 0x10], xmm1
14205f52c: 0f 29 45 e0                 	movaps	xmmword ptr [rbp - 0x20], xmm0
14205f530: 48 8d 05 41 00 f7 00        	lea	rax, [rip + 0xf70041]   # 0x142fcf578
14205f537: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
14205f53c: 48 8d 0d 25 fe f6 00        	lea	rcx, [rip + 0xf6fe25]   # 0x142fcf368
14205f543: 4c 8d 0d fe fd f6 00        	lea	r9, [rip + 0xf6fdfe]    # 0x142fcf348
14205f54a: 4c 8d 45 e0                 	lea	r8, [rbp - 0x20]
14205f54e: ba 2b 00 00 00              	mov	edx, 0x2b
14205f553: e8 08 a2 87 00              	call	0x1428d9760 <.text+0x28d8760>
14205f558: 0f 0b                       	ud2
14205f55a: 49 83 c3 fe                 	add	r11, -0x2
14205f55e: 49 83 fb 03                 	cmp	r11, 0x3
14205f562: 0f 83 8e 00 00 00           	jae	0x14205f5f6 <.text+0x205e5f6>
14205f568: e8 d3 b4 86 00              	call	0x1428caa40 <.text+0x28c9a40>
14205f56d: 4c 8d 05 34 47 ab 00        	lea	r8, [rip + 0xab4734]    # 0x142b13ca8
14205f574: 48 89 c1                    	mov	rcx, rax
14205f577: 4c 89 d2                    	mov	rdx, r10
14205f57a: e8 99 9f 87 00              	call	0x1428d9518 <.text+0x28d8518>
14205f57f: 4c 8d 05 3a 47 ab 00        	lea	r8, [rip + 0xab473a]    # 0x142b13cc0
14205f586: 4c 89 d1                    	mov	rcx, r10
14205f589: 4c 89 da                    	mov	rdx, r11
14205f58c: e8 87 9f 87 00              	call	0x1428d9518 <.text+0x28d8518>
14205f591: 48 8d 0d 18 ff f6 00        	lea	rcx, [rip + 0xf6ff18]   # 0x142fcf4b0
14205f598: e8 73 9d 87 00              	call	0x1428d9310 <.text+0x28d8310>
14205f59d: 4c 89 5d b0                 	mov	qword ptr [rbp - 0x50], r11
14205f5a1: 48 8d 05 e8 1c fd fd        	lea	rax, [rip - 0x202e318]  # 0x140031290 <.text+0x30290>
14205f5a8: 48 89 45 b8                 	mov	qword ptr [rbp - 0x48], rax
14205f5ac: 4c 89 55 c0                 	mov	qword ptr [rbp - 0x40], r10
14205f5b0: 48 8d 05 29 79 0f fe        	lea	rax, [rip - 0x1f086d7]  # 0x140156ee0 <.text+0x155ee0>
14205f5b7: 48 89 45 c8                 	mov	qword ptr [rbp - 0x38], rax
14205f5bb: 48 8d 05 4e 00 f7 00        	lea	rax, [rip + 0xf7004e]   # 0x142fcf610
14205f5c2: 48 89 45 38                 	mov	qword ptr [rbp + 0x38], rax
14205f5c6: 48 c7 45 40 03 00 00 00     	mov	qword ptr [rbp + 0x40], 0x3
14205f5ce: 48 c7 45 58 00 00 00 00     	mov	qword ptr [rbp + 0x58], 0x0
14205f5d6: 48 8d 45 b0                 	lea	rax, [rbp - 0x50]
14205f5da: 48 89 45 48                 	mov	qword ptr [rbp + 0x48], rax
14205f5de: 48 c7 45 50 02 00 00 00     	mov	qword ptr [rbp + 0x50], 0x2
14205f5e6: 48 8d 15 53 00 f7 00        	lea	rdx, [rip + 0xf70053]   # 0x142fcf640
14205f5ed: 48 8d 4d 38                 	lea	rcx, [rbp + 0x38]
14205f5f1: e8 9a 9d 87 00              	call	0x1428d9390 <.text+0x28d8390>
14205f5f6: 48 8d 0d cb 0b ab 00        	lea	rcx, [rip + 0xab0bcb]   # 0x142b101c8
14205f5fd: 4c 8d 05 d4 46 ab 00        	lea	r8, [rip + 0xab46d4]    # 0x142b13cd8
14205f604: ba 28 00 00 00              	mov	edx, 0x28
14205f609: e8 22 9e 87 00              	call	0x1428d9430 <.text+0x28d8430>
14205f60e: cc                          	int3
14205f60f: 90                          	nop
14205f610: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14205f615: 55                          	push	rbp
14205f616: 41 57                       	push	r15
14205f618: 41 56                       	push	r14
14205f61a: 41 55                       	push	r13
14205f61c: 41 54                       	push	r12
14205f61e: 56                          	push	rsi
14205f61f: 57                          	push	rdi
14205f620: 53                          	push	rbx
14205f621: 48 83 ec 28                 	sub	rsp, 0x28
14205f625: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14205f62c: 48 8d 4d e0                 	lea	rcx, [rbp - 0x20]
14205f630: e8 3b f4 76 fe              	call	0x1407cea70 <.text+0x7cda70>
14205f635: 90                          	nop
14205f636: 48 83 c4 28                 	add	rsp, 0x28
14205f63a: 5b                          	pop	rbx
14205f63b: 5f                          	pop	rdi
14205f63c: 5e                          	pop	rsi
14205f63d: 41 5c                       	pop	r12
14205f63f: 41 5d                       	pop	r13
14205f641: 41 5e                       	pop	r14
14205f643: 41 5f                       	pop	r15
14205f645: 5d                          	pop	rbp
14205f646: c3                          	ret
