
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141567950: 55                          	push	rbp
141567951: 41 57                       	push	r15
141567953: 41 56                       	push	r14
141567955: 41 55                       	push	r13
141567957: 41 54                       	push	r12
141567959: 56                          	push	rsi
14156795a: 57                          	push	rdi
14156795b: 53                          	push	rbx
14156795c: 48 81 ec 28 01 00 00        	sub	rsp, 0x128
141567963: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
14156796b: 48 c7 85 a0 00 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0xa0], -0x2
141567976: 4c 89 85 90 00 00 00        	mov	qword ptr [rbp + 0x90], r8
14156797d: 4d 89 cf                    	mov	r15, r9
141567980: 49 c1 e7 04                 	shl	r15, 0x4
141567984: 45 31 ed                    	xor	r13d, r13d
141567987: 4c 89 c8                    	mov	rax, r9
14156798a: 48 c1 e8 3c                 	shr	rax, 0x3c
14156798e: 0f 85 d9 03 00 00           	jne	0x141567d6d <.text+0x1566d6d>
141567994: 48 bf fc ff ff ff ff ff ff 7f       	movabs	rdi, 0x7ffffffffffffffc
14156799e: 49 39 ff                    	cmp	r15, rdi
1415679a1: 0f 87 c6 03 00 00           	ja	0x141567d6d <.text+0x1566d6d>
1415679a7: 4d 89 ce                    	mov	r14, r9
1415679aa: 48 89 d3                    	mov	rbx, rdx
1415679ad: 48 89 ce                    	mov	rsi, rcx
1415679b0: 4d 85 ff                    	test	r15, r15
1415679b3: 74 47                       	je	0x1415679fc <.text+0x15669fc>
1415679b5: 0f b6 05 4d d2 0c 02        	movzx	eax, byte ptr [rip + 0x20cd24d] # 0x143634c09
1415679bc: 41 bd 04 00 00 00           	mov	r13d, 0x4
1415679c2: ba 04 00 00 00              	mov	edx, 0x4
1415679c7: 4c 89 f9                    	mov	rcx, r15
1415679ca: e8 41 c2 0a ff              	call	0x140613c10 <.text+0x612c10>
1415679cf: 48 85 c0                    	test	rax, rax
1415679d2: 0f 84 95 03 00 00           	je	0x141567d6d <.text+0x1566d6d>
1415679d8: 49 89 c4                    	mov	r12, rax
1415679db: 4d 89 f5                    	mov	r13, r14
1415679de: 49 83 fe 02                 	cmp	r14, 0x2
1415679e2: 72 27                       	jb	0x141567a0b <.text+0x1566a0b>
1415679e4: 4d 8d 47 f0                 	lea	r8, [r15 - 0x10]
1415679e8: 4c 89 e1                    	mov	rcx, r12
1415679eb: 31 d2                       	xor	edx, edx
1415679ed: e8 2e c2 3b 01              	call	0x142923c20 <.text+0x2922c20>
1415679f2: 4b 8d 04 3c                 	lea	rax, [r12 + r15]
1415679f6: 48 83 c0 f0                 	add	rax, -0x10
1415679fa: eb 17                       	jmp	0x141567a13 <.text+0x1566a13>
1415679fc: 41 bc 04 00 00 00           	mov	r12d, 0x4
141567a02: 45 31 ed                    	xor	r13d, r13d
141567a05: 49 83 fe 02                 	cmp	r14, 0x2
141567a09: 73 d9                       	jae	0x1415679e4 <.text+0x15669e4>
141567a0b: 4c 89 e0                    	mov	rax, r12
141567a0e: 4d 85 f6                    	test	r14, r14
141567a11: 74 06                       	je	0x141567a19 <.text+0x1566a19>
141567a13: 0f 57 c0                    	xorps	xmm0, xmm0
141567a16: 0f 11 00                    	movups	xmmword ptr [rax], xmm0
141567a19: 4c 89 6d 30                 	mov	qword ptr [rbp + 0x30], r13
141567a1d: 4c 89 65 38                 	mov	qword ptr [rbp + 0x38], r12
141567a21: 4c 89 75 40                 	mov	qword ptr [rbp + 0x40], r14
141567a25: 48 8d 4d e8                 	lea	rcx, [rbp - 0x18]
141567a29: 4c 8d 45 30                 	lea	r8, [rbp + 0x30]
141567a2d: 48 89 da                    	mov	rdx, rbx
141567a30: e8 db 9d 12 00              	call	0x141691810 <.text+0x1690810>
141567a35: 4d 89 f7                    	mov	r15, r14
141567a38: 49 c1 e7 06                 	shl	r15, 0x6
141567a3c: 45 31 e4                    	xor	r12d, r12d
141567a3f: 4c 89 f0                    	mov	rax, r14
141567a42: 48 c1 e8 3a                 	shr	rax, 0x3a
141567a46: 0f 85 33 03 00 00           	jne	0x141567d7f <.text+0x1566d7f>
141567a4c: 49 39 ff                    	cmp	r15, rdi
141567a4f: 0f 87 2a 03 00 00           	ja	0x141567d7f <.text+0x1566d7f>
141567a55: 4d 85 ff                    	test	r15, r15
141567a58: 74 48                       	je	0x141567aa2 <.text+0x1566aa2>
141567a5a: 0f b6 05 a8 d1 0c 02        	movzx	eax, byte ptr [rip + 0x20cd1a8] # 0x143634c09
141567a61: 41 bc 04 00 00 00           	mov	r12d, 0x4
141567a67: ba 04 00 00 00              	mov	edx, 0x4
141567a6c: 4c 89 f9                    	mov	rcx, r15
141567a6f: e8 9c c1 0a ff              	call	0x140613c10 <.text+0x612c10>
141567a74: 48 85 c0                    	test	rax, rax
141567a77: 0f 84 02 03 00 00           	je	0x141567d7f <.text+0x1566d7f>
141567a7d: 4c 89 f1                    	mov	rcx, r14
141567a80: 49 83 fe 02                 	cmp	r14, 0x2
141567a84: 72 29                       	jb	0x141567aaf <.text+0x1566aaf>
141567a86: 4d 8d 4e ff                 	lea	r9, [r14 - 0x1]
141567a8a: 4d 8d 46 fe                 	lea	r8, [r14 - 0x2]
141567a8e: 44 89 ca                    	mov	edx, r9d
141567a91: 83 e2 07                    	and	edx, 0x7
141567a94: 49 83 f8 07                 	cmp	r8, 0x7
141567a98: 73 26                       	jae	0x141567ac0 <.text+0x1566ac0>
141567a9a: 49 89 c0                    	mov	r8, rax
141567a9d: e9 26 01 00 00              	jmp	0x141567bc8 <.text+0x1566bc8>
141567aa2: b8 04 00 00 00              	mov	eax, 0x4
141567aa7: 31 c9                       	xor	ecx, ecx
141567aa9: 49 83 fe 02                 	cmp	r14, 0x2
141567aad: 73 d7                       	jae	0x141567a86 <.text+0x1566a86>
141567aaf: 49 89 c0                    	mov	r8, rax
141567ab2: 4d 85 f6                    	test	r14, r14
141567ab5: 0f 85 31 01 00 00           	jne	0x141567bec <.text+0x1566bec>
141567abb: e9 42 01 00 00              	jmp	0x141567c02 <.text+0x1566c02>
141567ac0: 49 83 e1 f8                 	and	r9, -0x8
141567ac4: 0f 57 c0                    	xorps	xmm0, xmm0
141567ac7: 49 89 c0                    	mov	r8, rax
141567aca: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
141567ad0: 41 0f 11 40 30              	movups	xmmword ptr [r8 + 0x30], xmm0
141567ad5: 41 0f 11 40 20              	movups	xmmword ptr [r8 + 0x20], xmm0
141567ada: 41 0f 11 40 10              	movups	xmmword ptr [r8 + 0x10], xmm0
141567adf: 41 0f 11 00                 	movups	xmmword ptr [r8], xmm0
141567ae3: 41 0f 11 40 40              	movups	xmmword ptr [r8 + 0x40], xmm0
141567ae8: 41 0f 11 40 50              	movups	xmmword ptr [r8 + 0x50], xmm0
141567aed: 41 0f 11 40 60              	movups	xmmword ptr [r8 + 0x60], xmm0
141567af2: 41 0f 11 40 70              	movups	xmmword ptr [r8 + 0x70], xmm0
141567af7: 41 0f 11 80 80 00 00 00     	movups	xmmword ptr [r8 + 0x80], xmm0
141567aff: 41 0f 11 80 90 00 00 00     	movups	xmmword ptr [r8 + 0x90], xmm0
141567b07: 41 0f 11 80 a0 00 00 00     	movups	xmmword ptr [r8 + 0xa0], xmm0
141567b0f: 41 0f 11 80 b0 00 00 00     	movups	xmmword ptr [r8 + 0xb0], xmm0
141567b17: 41 0f 11 80 c0 00 00 00     	movups	xmmword ptr [r8 + 0xc0], xmm0
141567b1f: 41 0f 11 80 d0 00 00 00     	movups	xmmword ptr [r8 + 0xd0], xmm0
141567b27: 41 0f 11 80 e0 00 00 00     	movups	xmmword ptr [r8 + 0xe0], xmm0
141567b2f: 41 0f 11 80 f0 00 00 00     	movups	xmmword ptr [r8 + 0xf0], xmm0
141567b37: 41 0f 11 80 30 01 00 00     	movups	xmmword ptr [r8 + 0x130], xmm0
141567b3f: 41 0f 11 80 20 01 00 00     	movups	xmmword ptr [r8 + 0x120], xmm0
141567b47: 41 0f 11 80 10 01 00 00     	movups	xmmword ptr [r8 + 0x110], xmm0
141567b4f: 41 0f 11 80 00 01 00 00     	movups	xmmword ptr [r8 + 0x100], xmm0
141567b57: 41 0f 11 80 70 01 00 00     	movups	xmmword ptr [r8 + 0x170], xmm0
141567b5f: 41 0f 11 80 60 01 00 00     	movups	xmmword ptr [r8 + 0x160], xmm0
141567b67: 41 0f 11 80 50 01 00 00     	movups	xmmword ptr [r8 + 0x150], xmm0
141567b6f: 41 0f 11 80 40 01 00 00     	movups	xmmword ptr [r8 + 0x140], xmm0
141567b77: 41 0f 11 80 80 01 00 00     	movups	xmmword ptr [r8 + 0x180], xmm0
141567b7f: 41 0f 11 80 90 01 00 00     	movups	xmmword ptr [r8 + 0x190], xmm0
141567b87: 41 0f 11 80 a0 01 00 00     	movups	xmmword ptr [r8 + 0x1a0], xmm0
141567b8f: 41 0f 11 80 b0 01 00 00     	movups	xmmword ptr [r8 + 0x1b0], xmm0
141567b97: 41 0f 11 80 c0 01 00 00     	movups	xmmword ptr [r8 + 0x1c0], xmm0
141567b9f: 41 0f 11 80 d0 01 00 00     	movups	xmmword ptr [r8 + 0x1d0], xmm0
141567ba7: 41 0f 11 80 e0 01 00 00     	movups	xmmword ptr [r8 + 0x1e0], xmm0
141567baf: 41 0f 11 80 f0 01 00 00     	movups	xmmword ptr [r8 + 0x1f0], xmm0
141567bb7: 49 81 c0 00 02 00 00        	add	r8, 0x200
141567bbe: 49 83 c1 f8                 	add	r9, -0x8
141567bc2: 0f 85 08 ff ff ff           	jne	0x141567ad0 <.text+0x1566ad0>
141567bc8: 48 85 d2                    	test	rdx, rdx
141567bcb: 74 1f                       	je	0x141567bec <.text+0x1566bec>
141567bcd: 0f 57 c0                    	xorps	xmm0, xmm0
141567bd0: 41 0f 11 40 30              	movups	xmmword ptr [r8 + 0x30], xmm0
141567bd5: 41 0f 11 40 20              	movups	xmmword ptr [r8 + 0x20], xmm0
141567bda: 41 0f 11 40 10              	movups	xmmword ptr [r8 + 0x10], xmm0
141567bdf: 41 0f 11 00                 	movups	xmmword ptr [r8], xmm0
141567be3: 49 83 c0 40                 	add	r8, 0x40
141567be7: 48 ff ca                    	dec	rdx
141567bea: 75 e4                       	jne	0x141567bd0 <.text+0x1566bd0>
141567bec: 0f 57 c0                    	xorps	xmm0, xmm0
141567bef: 41 0f 11 40 30              	movups	xmmword ptr [r8 + 0x30], xmm0
141567bf4: 41 0f 11 40 20              	movups	xmmword ptr [r8 + 0x20], xmm0
141567bf9: 41 0f 11 40 10              	movups	xmmword ptr [r8 + 0x10], xmm0
141567bfe: 41 0f 11 00                 	movups	xmmword ptr [r8], xmm0
141567c02: 48 89 4d 30                 	mov	qword ptr [rbp + 0x30], rcx
141567c06: 48 89 45 38                 	mov	qword ptr [rbp + 0x38], rax
141567c0a: 4c 89 75 40                 	mov	qword ptr [rbp + 0x40], r14
141567c0e: 48 8d 4d a0                 	lea	rcx, [rbp - 0x60]
141567c12: 4c 8d 45 30                 	lea	r8, [rbp + 0x30]
141567c16: 48 89 da                    	mov	rdx, rbx
141567c19: e8 82 8a 12 00              	call	0x1416906a0 <.text+0x168f6a0>
141567c1e: 0f 10 45 a0                 	movups	xmm0, xmmword ptr [rbp - 0x60]
141567c22: 0f 10 4d b0                 	movups	xmm1, xmmword ptr [rbp - 0x50]
141567c26: 0f 10 55 c0                 	movups	xmm2, xmmword ptr [rbp - 0x40]
141567c2a: 0f 10 5d d0                 	movups	xmm3, xmmword ptr [rbp - 0x30]
141567c2e: 0f 11 45 48                 	movups	xmmword ptr [rbp + 0x48], xmm0
141567c32: 0f 11 4d 58                 	movups	xmmword ptr [rbp + 0x58], xmm1
141567c36: 0f 11 55 68                 	movups	xmmword ptr [rbp + 0x68], xmm2
141567c3a: 0f 11 5d 78                 	movups	xmmword ptr [rbp + 0x78], xmm3
141567c3e: 48 8b 45 e0                 	mov	rax, qword ptr [rbp - 0x20]
141567c42: 48 89 85 88 00 00 00        	mov	qword ptr [rbp + 0x88], rax
141567c49: c6 45 3e 05                 	mov	byte ptr [rbp + 0x3e], 0x5
141567c4d: 0f b6 05 b5 cf 0c 02        	movzx	eax, byte ptr [rip + 0x20ccfb5] # 0x143634c09
141567c54: b9 60 00 00 00              	mov	ecx, 0x60
141567c59: ba 08 00 00 00              	mov	edx, 0x8
141567c5e: e8 ad bf 0a ff              	call	0x140613c10 <.text+0x612c10>
141567c63: 48 85 c0                    	test	rax, rax
141567c66: 0f 84 27 01 00 00           	je	0x141567d93 <.text+0x1566d93>
141567c6c: 0f 10 85 80 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0x80]
141567c73: 0f 11 40 50                 	movups	xmmword ptr [rax + 0x50], xmm0
141567c77: 0f 10 45 70                 	movups	xmm0, xmmword ptr [rbp + 0x70]
141567c7b: 0f 11 40 40                 	movups	xmmword ptr [rax + 0x40], xmm0
141567c7f: 0f 10 45 30                 	movups	xmm0, xmmword ptr [rbp + 0x30]
141567c83: 0f 10 4d 40                 	movups	xmm1, xmmword ptr [rbp + 0x40]
141567c87: 0f 10 55 50                 	movups	xmm2, xmmword ptr [rbp + 0x50]
141567c8b: 0f 10 5d 60                 	movups	xmm3, xmmword ptr [rbp + 0x60]
141567c8f: 0f 11 58 30                 	movups	xmmword ptr [rax + 0x30], xmm3
141567c93: 0f 11 50 20                 	movups	xmmword ptr [rax + 0x20], xmm2
141567c97: 0f 11 48 10                 	movups	xmmword ptr [rax + 0x10], xmm1
141567c9b: 48 89 85 98 00 00 00        	mov	qword ptr [rbp + 0x98], rax
141567ca2: 0f 11 00                    	movups	xmmword ptr [rax], xmm0
141567ca5: 31 c9                       	xor	ecx, ecx
141567ca7: e8 74 1e b7 fe              	call	0x1400d9b20 <.text+0xd8b20>
141567cac: 48 85 c0                    	test	rax, rax
141567caf: 0f 84 ef 00 00 00           	je	0x141567da4 <.text+0x1566da4>
141567cb5: 0f 10 00                    	movups	xmm0, xmmword ptr [rax]
141567cb8: 48 ff 00                    	inc	qword ptr [rax]
141567cbb: 48 8b 45 28                 	mov	rax, qword ptr [rbp + 0x28]
141567cbf: 48 89 86 88 00 00 00        	mov	qword ptr [rsi + 0x88], rax
141567cc6: 0f 10 4d e8                 	movups	xmm1, xmmword ptr [rbp - 0x18]
141567cca: 0f 10 55 f8                 	movups	xmm2, xmmword ptr [rbp - 0x8]
141567cce: 0f 10 5d 08                 	movups	xmm3, xmmword ptr [rbp + 0x8]
141567cd2: 0f 10 65 18                 	movups	xmm4, xmmword ptr [rbp + 0x18]
141567cd6: 0f 11 66 78                 	movups	xmmword ptr [rsi + 0x78], xmm4
141567cda: 0f 11 5e 68                 	movups	xmmword ptr [rsi + 0x68], xmm3
141567cde: 0f 11 56 58                 	movups	xmmword ptr [rsi + 0x58], xmm2
141567ce2: 0f 11 4e 48                 	movups	xmmword ptr [rsi + 0x48], xmm1
141567ce6: 48 c7 06 00 00 00 00        	mov	qword ptr [rsi], 0x0
141567ced: 48 c7 46 08 04 00 00 00     	mov	qword ptr [rsi + 0x8], 0x4
141567cf5: 48 c7 46 10 00 00 00 00     	mov	qword ptr [rsi + 0x10], 0x0
141567cfd: 0f 10 0d 0c 0d 56 01        	movups	xmm1, xmmword ptr [rip + 0x1560d0c] # 0x142ac8a10
141567d04: 0f 11 4e 18                 	movups	xmmword ptr [rsi + 0x18], xmm1
141567d08: 0f 10 0d 11 0d 56 01        	movups	xmm1, xmmword ptr [rip + 0x1560d11] # 0x142ac8a20
141567d0f: 0f 11 4e 28                 	movups	xmmword ptr [rsi + 0x28], xmm1
141567d13: 0f 11 46 38                 	movups	xmmword ptr [rsi + 0x38], xmm0
141567d17: 48 8b 85 98 00 00 00        	mov	rax, qword ptr [rbp + 0x98]
141567d1e: 48 89 86 90 00 00 00        	mov	qword ptr [rsi + 0x90], rax
141567d25: 48 8d 05 1c e7 70 01        	lea	rax, [rip + 0x170e71c]  # 0x142c76448
141567d2c: 48 89 86 98 00 00 00        	mov	qword ptr [rsi + 0x98], rax
141567d33: 48 c7 86 a0 00 00 00 00 00 00 00    	mov	qword ptr [rsi + 0xa0], 0x0
141567d3e: 48 8b 85 90 00 00 00        	mov	rax, qword ptr [rbp + 0x90]
141567d45: 48 89 86 a8 00 00 00        	mov	qword ptr [rsi + 0xa8], rax
141567d4c: c7 86 b0 00 00 00 00 00 00 00       	mov	dword ptr [rsi + 0xb0], 0x0
141567d56: 48 89 f0                    	mov	rax, rsi
141567d59: 48 81 c4 28 01 00 00        	add	rsp, 0x128
141567d60: 5b                          	pop	rbx
141567d61: 5f                          	pop	rdi
141567d62: 5e                          	pop	rsi
141567d63: 41 5c                       	pop	r12
141567d65: 41 5d                       	pop	r13
141567d67: 41 5e                       	pop	r14
141567d69: 41 5f                       	pop	r15
141567d6b: 5d                          	pop	rbp
141567d6c: c3                          	ret
141567d6d: 4c 8d 05 34 e3 70 01        	lea	r8, [rip + 0x170e334]   # 0x142c760a8
141567d74: 4c 89 e9                    	mov	rcx, r13
141567d77: 4c 89 fa                    	mov	rdx, r15
141567d7a: e8 64 12 37 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
141567d7f: 4c 8d 05 3a e3 70 01        	lea	r8, [rip + 0x170e33a]   # 0x142c760c0
141567d86: 4c 89 e1                    	mov	rcx, r12
141567d89: 4c 89 fa                    	mov	rdx, r15
141567d8c: e8 52 12 37 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
141567d91: eb 1d                       	jmp	0x141567db0 <.text+0x1566db0>
141567d93: b9 08 00 00 00              	mov	ecx, 0x8
141567d98: ba 60 00 00 00              	mov	edx, 0x60
141567d9d: e8 5e 12 37 01              	call	0x1428d9000 <.text+0x28d8000>
141567da2: eb 0c                       	jmp	0x141567db0 <.text+0x1566db0>
141567da4: 48 8d 0d f5 2c 6d 01        	lea	rcx, [rip + 0x16d2cf5]  # 0x142c3aaa0
141567dab: e8 20 03 37 01              	call	0x1428d80d0 <.text+0x28d70d0>
141567db0: 0f 0b                       	ud2
141567db2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nop	word ptr cs:[rax + rax]
141567dc0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141567dc5: 55                          	push	rbp
141567dc6: 41 57                       	push	r15
141567dc8: 41 56                       	push	r14
141567dca: 41 55                       	push	r13
141567dcc: 41 54                       	push	r12
141567dce: 56                          	push	rsi
141567dcf: 57                          	push	rdi
141567dd0: 53                          	push	rbx
141567dd1: 48 83 ec 28                 	sub	rsp, 0x28
141567dd5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
141567ddc: 48 8d 4d e8                 	lea	rcx, [rbp - 0x18]
141567de0: e8 7b 55 31 ff              	call	0x14087d360 <.text+0x87c360>
141567de5: 90                          	nop
141567de6: 48 83 c4 28                 	add	rsp, 0x28
141567dea: 5b                          	pop	rbx
141567deb: 5f                          	pop	rdi
141567dec: 5e                          	pop	rsi
141567ded: 41 5c                       	pop	r12
141567def: 41 5d                       	pop	r13
141567df1: 41 5e                       	pop	r14
141567df3: 41 5f                       	pop	r15
141567df5: 5d                          	pop	rbp
141567df6: c3                          	ret
141567df7: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
141567e00: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141567e05: 55                          	push	rbp
141567e06: 41 57                       	push	r15
141567e08: 41 56                       	push	r14
141567e0a: 41 55                       	push	r13
141567e0c: 41 54                       	push	r12
141567e0e: 56                          	push	rsi
141567e0f: 57                          	push	rdi
141567e10: 53                          	push	rbx
141567e11: 48 83 ec 28                 	sub	rsp, 0x28
141567e15: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
141567e1c: 48 8d 4d 30                 	lea	rcx, [rbp + 0x30]
141567e20: e8 eb 6c 31 ff              	call	0x14087eb10 <.text+0x87db10>
141567e25: 90                          	nop
141567e26: 48 83 c4 28                 	add	rsp, 0x28
141567e2a: 5b                          	pop	rbx
141567e2b: 5f                          	pop	rdi
141567e2c: 5e                          	pop	rsi
141567e2d: 41 5c                       	pop	r12
141567e2f: 41 5d                       	pop	r13
141567e31: 41 5e                       	pop	r14
141567e33: 41 5f                       	pop	r15
141567e35: 5d                          	pop	rbp
141567e36: c3                          	ret
141567e37: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
141567e40: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141567e45: 55                          	push	rbp
141567e46: 41 57                       	push	r15
141567e48: 41 56                       	push	r14
141567e4a: 41 55                       	push	r13
141567e4c: 41 54                       	push	r12
141567e4e: 56                          	push	rsi
141567e4f: 57                          	push	rdi
141567e50: 53                          	push	rbx
141567e51: 48 83 ec 28                 	sub	rsp, 0x28
141567e55: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
141567e5c: 48 8d 15 e5 e5 70 01        	lea	rdx, [rip + 0x170e5e5]  # 0x142c76448
141567e63: 48 8b 8d 98 00 00 00        	mov	rcx, qword ptr [rbp + 0x98]
141567e6a: e8 91 6a ac fe              	call	0x14002e900 <.text+0x2d900>
141567e6f: 90                          	nop
141567e70: 48 83 c4 28                 	add	rsp, 0x28
141567e74: 5b                          	pop	rbx
141567e75: 5f                          	pop	rdi
141567e76: 5e                          	pop	rsi
141567e77: 41 5c                       	pop	r12
141567e79: 41 5d                       	pop	r13
141567e7b: 41 5e                       	pop	r14
141567e7d: 41 5f                       	pop	r15
141567e7f: 5d                          	pop	rbp
141567e80: c3                          	ret
