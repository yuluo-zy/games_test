
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1404f6850: 55                          	push	rbp
1404f6851: 41 57                       	push	r15
1404f6853: 41 56                       	push	r14
1404f6855: 56                          	push	rsi
1404f6856: 57                          	push	rdi
1404f6857: 53                          	push	rbx
1404f6858: 48 81 ec e8 01 00 00        	sub	rsp, 0x1e8
1404f685f: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
1404f6867: 48 c7 85 60 01 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x160], -0x2
1404f6872: 4c 89 cb                    	mov	rbx, r9
1404f6875: 48 89 d7                    	mov	rdi, rdx
1404f6878: 48 89 ce                    	mov	rsi, rcx
1404f687b: 4c 89 c1                    	mov	rcx, r8
1404f687e: e8 cd 8d 71 00              	call	0x140c0f650 <.text+0xc0e650>
1404f6883: 48 89 85 50 01 00 00        	mov	qword ptr [rbp + 0x150], rax
1404f688a: 48 83 bf c8 00 00 00 00     	cmp	qword ptr [rdi + 0xc8], 0x0
1404f6892: 0f 84 aa 02 00 00           	je	0x1404f6b42 <.text+0x4f5b42>
1404f6898: 48 8d 8f d0 00 00 00        	lea	rcx, [rdi + 0xd0]
1404f689f: 4c 8d bd 50 01 00 00        	lea	r15, [rbp + 0x150]
1404f68a6: 4c 89 fa                    	mov	rdx, r15
1404f68a9: e8 92 ad 0b 00              	call	0x1405b1640 <.text+0x5b0640>
1404f68ae: 4c 8b b7 b0 00 00 00        	mov	r14, qword ptr [rdi + 0xb0]
1404f68b5: 48 8b 8f b8 00 00 00        	mov	rcx, qword ptr [rdi + 0xb8]
1404f68bc: 48 89 ca                    	mov	rdx, rcx
1404f68bf: 48 21 c2                    	and	rdx, rax
1404f68c2: 48 c1 e8 39                 	shr	rax, 0x39
1404f68c6: 66 0f 6e c0                 	movd	xmm0, eax
1404f68ca: 66 0f 60 c0                 	punpcklbw	xmm0, xmm0      # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1404f68ce: f2 0f 70 c0 00              	pshuflw	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1404f68d3: 66 0f 70 c0 00              	pshufd	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0]
1404f68d8: 4d 8d 46 98                 	lea	r8, [r14 - 0x68]
1404f68dc: 45 31 c9                    	xor	r9d, r9d
1404f68df: 48 8b 85 50 01 00 00        	mov	rax, qword ptr [rbp + 0x150]
1404f68e6: 66 0f 76 c9                 	pcmpeqd	xmm1, xmm1
1404f68ea: f3 41 0f 6f 14 16           	movdqu	xmm2, xmmword ptr [r14 + rdx]
1404f68f0: 66 0f 6f da                 	movdqa	xmm3, xmm2
1404f68f4: 66 0f 74 d8                 	pcmpeqb	xmm3, xmm0
1404f68f8: 66 44 0f d7 d3              	pmovmskb	r10d, xmm3
1404f68fd: 45 85 d2                    	test	r10d, r10d
1404f6900: 74 2e                       	je	0x1404f6930 <.text+0x4f5930>
1404f6902: f3 45 0f bc da              	tzcnt	r11d, r10d
1404f6907: 49 01 d3                    	add	r11, rdx
1404f690a: 49 21 cb                    	and	r11, rcx
1404f690d: 49 f7 db                    	neg	r11
1404f6910: 4d 6b db 68                 	imul	r11, r11, 0x68
1404f6914: 4b 3b 04 18                 	cmp	rax, qword ptr [r8 + r11]
1404f6918: 74 38                       	je	0x1404f6952 <.text+0x4f5952>
1404f691a: 45 8d 5a ff                 	lea	r11d, [r10 - 0x1]
1404f691e: 66 45 21 d3                 	and	r11w, r10w
1404f6922: 45 89 da                    	mov	r10d, r11d
1404f6925: 75 db                       	jne	0x1404f6902 <.text+0x4f5902>
1404f6927: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1404f6930: 66 0f 74 d1                 	pcmpeqb	xmm2, xmm1
1404f6934: 66 44 0f d7 d2              	pmovmskb	r10d, xmm2
1404f6939: 45 85 d2                    	test	r10d, r10d
1404f693c: 0f 85 00 02 00 00           	jne	0x1404f6b42 <.text+0x4f5b42>
1404f6942: 4c 01 ca                    	add	rdx, r9
1404f6945: 48 83 c2 10                 	add	rdx, 0x10
1404f6949: 49 83 c1 10                 	add	r9, 0x10
1404f694d: 48 21 ca                    	and	rdx, rcx
1404f6950: eb 98                       	jmp	0x1404f68ea <.text+0x4f58ea>
1404f6952: 43 83 7c 1e a0 02           	cmp	dword ptr [r14 + r11 - 0x60], 0x2
1404f6958: 0f 85 8c 03 00 00           	jne	0x1404f6cea <.text+0x4f5cea>
1404f695e: 4d 01 de                    	add	r14, r11
1404f6961: 41 0f b6 4e fe              	movzx	ecx, byte ptr [r14 - 0x2]
1404f6966: 80 f9 07                    	cmp	cl, 0x7
1404f6969: 0f 84 ab 03 00 00           	je	0x1404f6d1a <.text+0x4f5d1a>
1404f696f: 41 8b 56 f4                 	mov	edx, dword ptr [r14 - 0xc]
1404f6973: 4d 8b 46 ec                 	mov	r8, qword ptr [r14 - 0x14]
1404f6977: 44 8b 4b 14                 	mov	r9d, dword ptr [rbx + 0x14]
1404f697b: 49 31 d1                    	xor	r9, rdx
1404f697e: 4c 33 43 0c                 	xor	r8, qword ptr [rbx + 0xc]
1404f6982: 4d 09 c8                    	or	r8, r9
1404f6985: 0f 85 ac 00 00 00           	jne	0x1404f6a37 <.text+0x4f5a37>
1404f698b: 3a 4b 1e                    	cmp	cl, byte ptr [rbx + 0x1e]
1404f698e: 0f 85 ff 03 00 00           	jne	0x1404f6d93 <.text+0x4f5d93>
1404f6994: 41 8b 46 e0                 	mov	eax, dword ptr [r14 - 0x20]
1404f6998: 3b 03                       	cmp	eax, dword ptr [rbx]
1404f699a: 0f 85 f3 03 00 00           	jne	0x1404f6d93 <.text+0x4f5d93>
1404f69a0: 41 8b 46 e4                 	mov	eax, dword ptr [r14 - 0x1c]
1404f69a4: 3b 43 04                    	cmp	eax, dword ptr [rbx + 0x4]
1404f69a7: 0f 85 e6 03 00 00           	jne	0x1404f6d93 <.text+0x4f5d93>
1404f69ad: 41 8b 46 e8                 	mov	eax, dword ptr [r14 - 0x18]
1404f69b1: 3b 43 08                    	cmp	eax, dword ptr [rbx + 0x8]
1404f69b4: 0f 85 d9 03 00 00           	jne	0x1404f6d93 <.text+0x4f5d93>
1404f69ba: 41 0f b6 46 fc              	movzx	eax, byte ptr [r14 - 0x4]
1404f69bf: 3a 43 1c                    	cmp	al, byte ptr [rbx + 0x1c]
1404f69c2: 0f 85 cb 03 00 00           	jne	0x1404f6d93 <.text+0x4f5d93>
1404f69c8: 41 8b 46 f8                 	mov	eax, dword ptr [r14 - 0x8]
1404f69cc: 3b 43 18                    	cmp	eax, dword ptr [rbx + 0x18]
1404f69cf: 0f 85 be 03 00 00           	jne	0x1404f6d93 <.text+0x4f5d93>
1404f69d5: 41 0f b6 46 fd              	movzx	eax, byte ptr [r14 - 0x3]
1404f69da: 3a 43 1d                    	cmp	al, byte ptr [rbx + 0x1d]
1404f69dd: 0f 85 b0 03 00 00           	jne	0x1404f6d93 <.text+0x4f5d93>
1404f69e3: 45 0f b6 4e a4              	movzx	r9d, byte ptr [r14 - 0x5c]
1404f69e8: 49 8d 56 c8                 	lea	rdx, [r14 - 0x38]
1404f69ec: 48 8d 4d 14                 	lea	rcx, [rbp + 0x14]
1404f69f0: 49 89 f8                    	mov	r8, rdi
1404f69f3: e8 a8 f6 6d 00              	call	0x140bd60a0 <.text+0xbd50a0>
1404f69f8: 8b 45 14                    	mov	eax, dword ptr [rbp + 0x14]
1404f69fb: 41 c7 46 a0 00 00 00 00     	mov	dword ptr [r14 - 0x60], 0x0
1404f6a03: 41 89 46 a4                 	mov	dword ptr [r14 - 0x5c], eax
1404f6a07: 0f 10 45 18                 	movups	xmm0, xmmword ptr [rbp + 0x18]
1404f6a0b: 0f 10 4d 28                 	movups	xmm1, xmmword ptr [rbp + 0x28]
1404f6a0f: 41 0f 11 4e b8              	movups	xmmword ptr [r14 - 0x48], xmm1
1404f6a14: 41 0f 11 46 a8              	movups	xmmword ptr [r14 - 0x58], xmm0
1404f6a19: 0f 10 45 14                 	movups	xmm0, xmmword ptr [rbp + 0x14]
1404f6a1d: 0f 10 4d 24                 	movups	xmm1, xmmword ptr [rbp + 0x24]
1404f6a21: 0f 11 06                    	movups	xmmword ptr [rsi], xmm0
1404f6a24: 0f 11 4e 10                 	movups	xmmword ptr [rsi + 0x10], xmm1
1404f6a28: 8b 45 34                    	mov	eax, dword ptr [rbp + 0x34]
1404f6a2b: 89 46 20                    	mov	dword ptr [rsi + 0x20], eax
1404f6a2e: c6 46 24 01                 	mov	byte ptr [rsi + 0x24], 0x1
1404f6a32: e9 a3 02 00 00              	jmp	0x1404f6cda <.text+0x4f5cda>
1404f6a37: 48 8b 0d 6a 59 13 03        	mov	rcx, qword ptr [rip + 0x313596a] # 0x14362c3a8
1404f6a3e: 48 8b 09                    	mov	rcx, qword ptr [rcx]
1404f6a41: 48 83 f9 05                 	cmp	rcx, 0x5
1404f6a45: 0f 82 f7 00 00 00           	jb	0x1404f6b42 <.text+0x4f5b42>
1404f6a4b: 49 83 c6 ec                 	add	r14, -0x14
1404f6a4f: 48 8d 43 0c                 	lea	rax, [rbx + 0xc]
1404f6a53: 4c 89 7d 38                 	mov	qword ptr [rbp + 0x38], r15
1404f6a57: 48 8d 0d 12 8d 71 00        	lea	rcx, [rip + 0x718d12]   # 0x140c0f770 <.text+0xc0e770>
1404f6a5e: 48 89 4d 40                 	mov	qword ptr [rbp + 0x40], rcx
1404f6a62: 4c 89 75 48                 	mov	qword ptr [rbp + 0x48], r14
1404f6a66: 48 8d 0d 93 03 0c 00        	lea	rcx, [rip + 0xc0393]    # 0x1405b6e00 <.text+0x5b5e00>
1404f6a6d: 48 89 4d 50                 	mov	qword ptr [rbp + 0x50], rcx
1404f6a71: 48 89 45 58                 	mov	qword ptr [rbp + 0x58], rax
1404f6a75: 48 89 4d 60                 	mov	qword ptr [rbp + 0x60], rcx
1404f6a79: 48 8d 0d 10 64 4f 02        	lea	rcx, [rip + 0x24f6410]  # 0x1429ece90
1404f6a80: e8 fb 72 18 00              	call	0x14067dd80 <.text+0x67cd80>
1404f6a85: 0f 10 00                    	movups	xmm0, xmmword ptr [rax]
1404f6a88: 8b 40 10                    	mov	eax, dword ptr [rax + 0x10]
1404f6a8b: 48 c7 85 c8 00 00 00 05 00 00 00    	mov	qword ptr [rbp + 0xc8], 0x5
1404f6a96: 48 8d 0d 0b 64 4f 02        	lea	rcx, [rip + 0x24f640b]  # 0x1429ecea8
1404f6a9d: 48 89 8d d0 00 00 00        	mov	qword ptr [rbp + 0xd0], rcx
1404f6aa4: 48 c7 85 d8 00 00 00 16 00 00 00    	mov	qword ptr [rbp + 0xd8], 0x16
1404f6aaf: 48 8d 15 aa 63 4f 02        	lea	rdx, [rip + 0x24f63aa]  # 0x1429ece60
1404f6ab6: 48 89 95 e8 00 00 00        	mov	qword ptr [rbp + 0xe8], rdx
1404f6abd: 48 c7 85 f0 00 00 00 03 00 00 00    	mov	qword ptr [rbp + 0xf0], 0x3
1404f6ac8: 48 8d 55 38                 	lea	rdx, [rbp + 0x38]
1404f6acc: 48 89 95 f8 00 00 00        	mov	qword ptr [rbp + 0xf8], rdx
1404f6ad3: 48 c7 85 00 01 00 00 03 00 00 00    	mov	qword ptr [rbp + 0x100], 0x3
1404f6ade: 48 c7 85 08 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x108], 0x0
1404f6ae9: 48 c7 85 98 00 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x98], 0x0
1404f6af4: 48 89 8d a0 00 00 00        	mov	qword ptr [rbp + 0xa0], rcx
1404f6afb: 48 c7 85 a8 00 00 00 16 00 00 00    	mov	qword ptr [rbp + 0xa8], 0x16
1404f6b06: 48 c7 85 b0 00 00 00 00 00 00 00    	mov	qword ptr [rbp + 0xb0], 0x0
1404f6b11: 0f 11 85 b8 00 00 00        	movups	xmmword ptr [rbp + 0xb8], xmm0
1404f6b18: c7 85 e0 00 00 00 01 00 00 00       	mov	dword ptr [rbp + 0xe0], 0x1
1404f6b22: 89 85 e4 00 00 00           	mov	dword ptr [rbp + 0xe4], eax
1404f6b28: 48 8d 8d 18 01 00 00        	lea	rcx, [rbp + 0x118]
1404f6b2f: 48 8d 95 98 00 00 00        	lea	rdx, [rbp + 0x98]
1404f6b36: e8 f5 37 85 00              	call	0x140d4a330 <.text+0xd49330>
1404f6b3b: 48 8b 85 50 01 00 00        	mov	rax, qword ptr [rbp + 0x150]
1404f6b42: 48 8b 97 a8 00 00 00        	mov	rdx, qword ptr [rdi + 0xa8]
1404f6b49: 4c 8b 48 f8                 	mov	r9, qword ptr [rax - 0x8]
1404f6b4d: 48 c7 45 b8 00 00 00 00     	mov	qword ptr [rbp - 0x48], 0x0
1404f6b55: f3 0f 6f 03                 	movdqu	xmm0, xmmword ptr [rbx]
1404f6b59: f3 0f 6f 4b 10              	movdqu	xmm1, xmmword ptr [rbx + 0x10]
1404f6b5e: 66 0f 7f 4d e0              	movdqa	xmmword ptr [rbp - 0x20], xmm1
1404f6b63: 66 0f 7f 45 d0              	movdqa	xmmword ptr [rbp - 0x30], xmm0
1404f6b68: 48 8d 4d b8                 	lea	rcx, [rbp - 0x48]
1404f6b6c: 48 89 4c 24 28              	mov	qword ptr [rsp + 0x28], rcx
1404f6b71: 48 8d 4d d0                 	lea	rcx, [rbp - 0x30]
1404f6b75: 48 89 4c 24 20              	mov	qword ptr [rsp + 0x20], rcx
1404f6b7a: 4c 8d b5 98 00 00 00        	lea	r14, [rbp + 0x98]
1404f6b81: 4c 89 f1                    	mov	rcx, r14
1404f6b84: 49 89 c0                    	mov	r8, rax
1404f6b87: e8 64 3a 0c 00              	call	0x1405ba5f0 <.text+0x5b95f0>
1404f6b8c: 48 8d 4d 38                 	lea	rcx, [rbp + 0x38]
1404f6b90: 4c 89 f2                    	mov	rdx, r14
1404f6b93: 49 89 d8                    	mov	r8, rbx
1404f6b96: e8 25 b1 e1 ff              	call	0x140311cc0 <.text+0x310cc0>
1404f6b9b: 0f b6 45 6e                 	movzx	eax, byte ptr [rbp + 0x6e]
1404f6b9f: 48 8b 4d 38                 	mov	rcx, qword ptr [rbp + 0x38]
1404f6ba3: 3c 07                       	cmp	al, 0x7
1404f6ba5: 75 0c                       	jne	0x1404f6bb3 <.text+0x4f5bb3>
1404f6ba7: 48 89 0e                    	mov	qword ptr [rsi], rcx
1404f6baa: c6 46 24 02                 	mov	byte ptr [rsi + 0x24], 0x2
1404f6bae: e9 27 01 00 00              	jmp	0x1404f6cda <.text+0x4f5cda>
1404f6bb3: 0f 10 45 40                 	movups	xmm0, xmmword ptr [rbp + 0x40]
1404f6bb7: 0f 10 4d 50                 	movups	xmm1, xmmword ptr [rbp + 0x50]
1404f6bbb: f3 0f 6f 55 5e              	movdqu	xmm2, xmmword ptr [rbp + 0x5e]
1404f6bc0: f3 0f 7f 95 3e 01 00 00     	movdqu	xmmword ptr [rbp + 0x13e], xmm2
1404f6bc8: 0f 11 8d 30 01 00 00        	movups	xmmword ptr [rbp + 0x130], xmm1
1404f6bcf: 0f 11 85 20 01 00 00        	movups	xmmword ptr [rbp + 0x120], xmm0
1404f6bd6: 0f b6 55 6f                 	movzx	edx, byte ptr [rbp + 0x6f]
1404f6bda: 48 89 8d 18 01 00 00        	mov	qword ptr [rbp + 0x118], rcx
1404f6be1: 88 85 4e 01 00 00           	mov	byte ptr [rbp + 0x14e], al
1404f6be7: 88 95 4f 01 00 00           	mov	byte ptr [rbp + 0x14f], dl
1404f6bed: c6 85 5f 01 00 00 01        	mov	byte ptr [rbp + 0x15f], 0x1
1404f6bf4: 48 8d 4d f0                 	lea	rcx, [rbp - 0x10]
1404f6bf8: 48 8d 95 18 01 00 00        	lea	rdx, [rbp + 0x118]
1404f6bff: 49 89 f8                    	mov	r8, rdi
1404f6c02: 45 31 c9                    	xor	r9d, r9d
1404f6c05: e8 96 f4 6d 00              	call	0x140bd60a0 <.text+0xbd50a0>
1404f6c0a: 48 81 c7 b0 00 00 00        	add	rdi, 0xb0
1404f6c11: 48 8b 85 48 01 00 00        	mov	rax, qword ptr [rbp + 0x148]
1404f6c18: 48 89 85 f0 00 00 00        	mov	qword ptr [rbp + 0xf0], rax
1404f6c1f: 0f 10 85 18 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x118]
1404f6c26: 0f 10 8d 28 01 00 00        	movups	xmm1, xmmword ptr [rbp + 0x128]
1404f6c2d: f3 0f 6f 95 38 01 00 00     	movdqu	xmm2, xmmword ptr [rbp + 0x138]
1404f6c35: f3 0f 7f 95 e0 00 00 00     	movdqu	xmmword ptr [rbp + 0xe0], xmm2
1404f6c3d: 0f 11 8d d0 00 00 00        	movups	xmmword ptr [rbp + 0xd0], xmm1
1404f6c44: 0f 11 85 c0 00 00 00        	movups	xmmword ptr [rbp + 0xc0], xmm0
1404f6c4b: 8b 45 f0                    	mov	eax, dword ptr [rbp - 0x10]
1404f6c4e: 0f 10 45 f4                 	movups	xmm0, xmmword ptr [rbp - 0xc]
1404f6c52: 0f 11 85 a0 00 00 00        	movups	xmmword ptr [rbp + 0xa0], xmm0
1404f6c59: 0f 10 45 04                 	movups	xmm0, xmmword ptr [rbp + 0x4]
1404f6c5d: 0f 11 85 b0 00 00 00        	movups	xmmword ptr [rbp + 0xb0], xmm0
1404f6c64: 89 85 9c 00 00 00           	mov	dword ptr [rbp + 0x9c], eax
1404f6c6a: c7 85 98 00 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x98], 0x0
1404f6c74: 4c 8b 85 50 01 00 00        	mov	r8, qword ptr [rbp + 0x150]
1404f6c7b: c6 85 5f 01 00 00 00        	mov	byte ptr [rbp + 0x15f], 0x0
1404f6c82: 48 8d 4d 38                 	lea	rcx, [rbp + 0x38]
1404f6c86: 4c 8d 8d 98 00 00 00        	lea	r9, [rbp + 0x98]
1404f6c8d: 48 89 fa                    	mov	rdx, rdi
1404f6c90: e8 bb 22 dd ff              	call	0x1402c8f50 <.text+0x2c7f50>
1404f6c95: 83 7d 38 03                 	cmp	dword ptr [rbp + 0x38], 0x3
1404f6c99: 74 22                       	je	0x1404f6cbd <.text+0x4f5cbd>
1404f6c9b: 80 bd 96 00 00 00 07        	cmp	byte ptr [rbp + 0x96], 0x7
1404f6ca2: 74 09                       	je	0x1404f6cad <.text+0x4f5cad>
1404f6ca4: 48 8d 4d 60                 	lea	rcx, [rbp + 0x60]
1404f6ca8: e8 d3 4e b4 ff              	call	0x14003bb80 <.text+0x3ab80>
1404f6cad: c6 85 5f 01 00 00 00        	mov	byte ptr [rbp + 0x15f], 0x0
1404f6cb4: 48 8d 4d 60                 	lea	rcx, [rbp + 0x60]
1404f6cb8: e8 f3 b7 6e 00              	call	0x140be24b0 <.text+0xbe14b0>
1404f6cbd: 8b 45 10                    	mov	eax, dword ptr [rbp + 0x10]
1404f6cc0: 89 46 20                    	mov	dword ptr [rsi + 0x20], eax
1404f6cc3: f3 0f 6f 45 f0              	movdqu	xmm0, xmmword ptr [rbp - 0x10]
1404f6cc8: f3 0f 6f 4d 00              	movdqu	xmm1, xmmword ptr [rbp]
1404f6ccd: f3 0f 7f 4e 10              	movdqu	xmmword ptr [rsi + 0x10], xmm1
1404f6cd2: f3 0f 7f 06                 	movdqu	xmmword ptr [rsi], xmm0
1404f6cd6: c6 46 24 00                 	mov	byte ptr [rsi + 0x24], 0x0
1404f6cda: 48 81 c4 e8 01 00 00        	add	rsp, 0x1e8
1404f6ce1: 5b                          	pop	rbx
1404f6ce2: 5f                          	pop	rdi
1404f6ce3: 5e                          	pop	rsi
1404f6ce4: 41 5e                       	pop	r14
1404f6ce6: 41 5f                       	pop	r15
1404f6ce8: 5d                          	pop	rbp
1404f6ce9: c3                          	ret
1404f6cea: 4c 89 bd 18 01 00 00        	mov	qword ptr [rbp + 0x118], r15
1404f6cf1: 48 8d 05 78 8a 71 00        	lea	rax, [rip + 0x718a78]   # 0x140c0f770 <.text+0xc0e770>
1404f6cf8: 48 89 85 20 01 00 00        	mov	qword ptr [rbp + 0x120], rax
1404f6cff: 48 8d 05 0a 61 4f 02        	lea	rax, [rip + 0x24f610a]  # 0x1429ece10
1404f6d06: 48 89 85 98 00 00 00        	mov	qword ptr [rbp + 0x98], rax
1404f6d0d: 48 c7 85 a0 00 00 00 01 00 00 00    	mov	qword ptr [rbp + 0xa0], 0x1
1404f6d18: eb 2e                       	jmp	0x1404f6d48 <.text+0x4f5d48>
1404f6d1a: 4c 89 bd 18 01 00 00        	mov	qword ptr [rbp + 0x118], r15
1404f6d21: 48 8d 05 48 8a 71 00        	lea	rax, [rip + 0x718a48]   # 0x140c0f770 <.text+0xc0e770>
1404f6d28: 48 89 85 20 01 00 00        	mov	qword ptr [rbp + 0x120], rax
1404f6d2f: 48 8d 05 ca 61 4f 02        	lea	rax, [rip + 0x24f61ca]  # 0x1429ecf00
1404f6d36: 48 89 85 98 00 00 00        	mov	qword ptr [rbp + 0x98], rax
1404f6d3d: 48 c7 85 a0 00 00 00 02 00 00 00    	mov	qword ptr [rbp + 0xa0], 0x2
1404f6d48: 48 8d 85 18 01 00 00        	lea	rax, [rbp + 0x118]
1404f6d4f: 48 89 85 a8 00 00 00        	mov	qword ptr [rbp + 0xa8], rax
1404f6d56: 48 c7 85 b0 00 00 00 01 00 00 00    	mov	qword ptr [rbp + 0xb0], 0x1
1404f6d61: 48 c7 85 b8 00 00 00 00 00 00 00    	mov	qword ptr [rbp + 0xb8], 0x0
1404f6d6c: 48 8d 7d 38                 	lea	rdi, [rbp + 0x38]
1404f6d70: 48 8d 95 98 00 00 00        	lea	rdx, [rbp + 0x98]
1404f6d77: 48 89 f9                    	mov	rcx, rdi
1404f6d7a: e8 11 32 88 00              	call	0x140d79f90 <.text+0xd78f90>
1404f6d7f: 48 89 f9                    	mov	rcx, rdi
1404f6d82: e8 99 bd 3a 02              	call	0x1428a2b20 <.text+0x28a1b20>
1404f6d87: 48 89 06                    	mov	qword ptr [rsi], rax
1404f6d8a: c6 46 24 02                 	mov	byte ptr [rsi + 0x24], 0x2
1404f6d8e: e9 47 ff ff ff              	jmp	0x1404f6cda <.text+0x4f5cda>
1404f6d93: 48 c7 85 98 00 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x98], 0x0
1404f6d9e: 49 83 c6 e0                 	add	r14, -0x20
1404f6da2: 48 8d 05 17 61 4f 02        	lea	rax, [rip + 0x24f6117]  # 0x1429ecec0
1404f6da9: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1404f6dae: 4c 8d 8d 98 00 00 00        	lea	r9, [rbp + 0x98]
1404f6db5: 31 c9                       	xor	ecx, ecx
1404f6db7: 4c 89 f2                    	mov	rdx, r14
1404f6dba: 49 89 d8                    	mov	r8, rbx
1404f6dbd: e8 c8 2b 3b 02              	call	0x1428a998a <.text+0x28a898a>
1404f6dc2: cc                          	int3
1404f6dc3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
1404f6dd0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1404f6dd5: 55                          	push	rbp
1404f6dd6: 41 57                       	push	r15
1404f6dd8: 41 56                       	push	r14
1404f6dda: 56                          	push	rsi
1404f6ddb: 57                          	push	rdi
1404f6ddc: 53                          	push	rbx
1404f6ddd: 48 83 ec 38                 	sub	rsp, 0x38
1404f6de1: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1404f6de8: c6 85 5f 01 00 00 00        	mov	byte ptr [rbp + 0x15f], 0x0
1404f6def: 48 8d 4d 60                 	lea	rcx, [rbp + 0x60]
1404f6df3: e8 b8 b6 6e 00              	call	0x140be24b0 <.text+0xbe14b0>
1404f6df8: c6 85 5f 01 00 00 00        	mov	byte ptr [rbp + 0x15f], 0x0
1404f6dff: 48 83 c4 38                 	add	rsp, 0x38
1404f6e03: 5b                          	pop	rbx
1404f6e04: 5f                          	pop	rdi
1404f6e05: 5e                          	pop	rsi
1404f6e06: 41 5e                       	pop	r14
1404f6e08: 41 5f                       	pop	r15
1404f6e0a: 5d                          	pop	rbp
1404f6e0b: c3                          	ret
1404f6e0c: 0f 1f 40 00                 	nop	dword ptr [rax]
1404f6e10: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1404f6e15: 55                          	push	rbp
1404f6e16: 41 57                       	push	r15
1404f6e18: 41 56                       	push	r14
1404f6e1a: 56                          	push	rsi
1404f6e1b: 57                          	push	rdi
1404f6e1c: 53                          	push	rbx
1404f6e1d: 48 83 ec 38                 	sub	rsp, 0x38
1404f6e21: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1404f6e28: 80 bd 5f 01 00 00 00        	cmp	byte ptr [rbp + 0x15f], 0x0
1404f6e2f: 75 0d                       	jne	0x1404f6e3e <.text+0x4f5e3e>
1404f6e31: 48 83 c4 38                 	add	rsp, 0x38
1404f6e35: 5b                          	pop	rbx
1404f6e36: 5f                          	pop	rdi
1404f6e37: 5e                          	pop	rsi
1404f6e38: 41 5e                       	pop	r14
1404f6e3a: 41 5f                       	pop	r15
1404f6e3c: 5d                          	pop	rbp
1404f6e3d: c3                          	ret
1404f6e3e: 48 8d 8d 18 01 00 00        	lea	rcx, [rbp + 0x118]
1404f6e45: e8 96 77 0f 00              	call	0x1405ee5e0 <.text+0x5ed5e0>
1404f6e4a: eb e5                       	jmp	0x1404f6e31 <.text+0x4f5e31>
