
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a3ba60: 55                          	push	rbp
140a3ba61: 41 57                       	push	r15
140a3ba63: 41 56                       	push	r14
140a3ba65: 41 55                       	push	r13
140a3ba67: 41 54                       	push	r12
140a3ba69: 56                          	push	rsi
140a3ba6a: 57                          	push	rdi
140a3ba6b: 53                          	push	rbx
140a3ba6c: 48 81 ec 28 05 00 00        	sub	rsp, 0x528
140a3ba73: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
140a3ba7b: 0f 29 b5 90 04 00 00        	movaps	xmmword ptr [rbp + 0x490], xmm6
140a3ba82: 48 c7 85 88 04 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x488], -0x2
140a3ba8d: 4d 89 c7                    	mov	r15, r8
140a3ba90: 49 89 d4                    	mov	r12, rdx
140a3ba93: 49 89 ce                    	mov	r14, rcx
140a3ba96: 48 8b b5 18 05 00 00        	mov	rsi, qword ptr [rbp + 0x518]
140a3ba9d: 4c 89 8d 88 02 00 00        	mov	qword ptr [rbp + 0x288], r9
140a3baa4: 49 8d 49 04                 	lea	rcx, [r9 + 0x4]
140a3baa8: e8 e3 9c 19 00              	call	0x140bd5790 <.text+0xbd4790>
140a3baad: 49 89 c5                    	mov	r13, rax
140a3bab0: 48 8d 4e 04                 	lea	rcx, [rsi + 0x4]
140a3bab4: e8 d7 9c 19 00              	call	0x140bd5790 <.text+0xbd4790>
140a3bab9: 48 89 c7                    	mov	rdi, rax
140a3babc: 49 8d 4f 38                 	lea	rcx, [r15 + 0x38]
140a3bac0: 48 ba 8b 04 7b e6 61 26 c0 a4       	movabs	rdx, -0x5b3fd99e1984fb75
140a3baca: 49 b8 ec 67 2a aa e6 61 e7 ea       	movabs	r8, -0x15189e1955d59814
140a3bad4: e8 17 1c 6a ff              	call	0x1400dd6f0 <.text+0xdc6f0>
140a3bad9: 48 85 c0                    	test	rax, rax
140a3badc: 0f 84 a8 0f 00 00           	je	0x140a3ca8a <.text+0xa3ba8a>
140a3bae2: 49 39 97 18 02 00 00        	cmp	qword ptr [r15 + 0x218], rdx
140a3bae9: 0f 86 9b 0f 00 00           	jbe	0x140a3ca8a <.text+0xa3ba8a>
140a3baef: 49 8b 87 10 02 00 00        	mov	rax, qword ptr [r15 + 0x210]
140a3baf6: 48 8b 0c d0                 	mov	rcx, qword ptr [rax + 8*rdx]
140a3bafa: 48 85 c9                    	test	rcx, rcx
140a3bafd: 0f 84 87 0f 00 00           	je	0x140a3ca8a <.text+0xa3ba8a>
140a3bb03: 49 8b 87 e0 01 00 00        	mov	rax, qword ptr [r15 + 0x1e0]
140a3bb0a: 48 f7 d1                    	not	rcx
140a3bb0d: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
140a3bb11: 48 c1 e1 05                 	shl	rcx, 0x5
140a3bb15: 48 83 7c 08 20 00           	cmp	qword ptr [rax + rcx + 0x20], 0x0
140a3bb1b: 0f 84 69 0f 00 00           	je	0x140a3ca8a <.text+0xa3ba8a>
140a3bb21: 48 01 c8                    	add	rax, rcx
140a3bb24: 48 8b 40 10                 	mov	rax, qword ptr [rax + 0x10]
140a3bb28: 0f b6 40 20                 	movzx	eax, byte ptr [rax + 0x20]
140a3bb2c: 89 85 2c 04 00 00           	mov	dword ptr [rbp + 0x42c], eax
140a3bb32: 48 8d 9d 90 02 00 00        	lea	rbx, [rbp + 0x290]
140a3bb39: 48 89 d9                    	mov	rcx, rbx
140a3bb3c: 48 89 fa                    	mov	rdx, rdi
140a3bb3f: 41 b8 10 00 00 00           	mov	r8d, 0x10
140a3bb45: e8 56 97 19 00              	call	0x140bd52a0 <.text+0xbd42a0>
140a3bb4a: 48 8d b5 f0 00 00 00        	lea	rsi, [rbp + 0xf0]
140a3bb51: 48 89 f1                    	mov	rcx, rsi
140a3bb54: 48 89 da                    	mov	rdx, rbx
140a3bb57: e8 14 98 19 00              	call	0x140bd5370 <.text+0xbd4370>
140a3bb5c: 48 8d 9d 30 04 00 00        	lea	rbx, [rbp + 0x430]
140a3bb63: 48 89 d9                    	mov	rcx, rbx
140a3bb66: 48 89 f2                    	mov	rdx, rsi
140a3bb69: e8 22 98 19 00              	call	0x140bd5390 <.text+0xbd4390>
140a3bb6e: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
140a3bb73: 4c 8d 05 3e 19 09 02        	lea	r8, [rip + 0x209193e]   # 0x142acd4b8
140a3bb7a: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
140a3bb81: 41 b9 08 00 00 00           	mov	r9d, 0x8
140a3bb87: 4c 89 e2                    	mov	rdx, r12
140a3bb8a: e8 01 19 f3 ff              	call	0x14096d490 <.text+0x96c490>
140a3bb8f: 83 bd 90 02 00 00 01        	cmp	dword ptr [rbp + 0x290], 0x1
140a3bb96: 0f 84 fa 0e 00 00           	je	0x140a3ca96 <.text+0xa3ba96>
140a3bb9c: 48 8b 85 d4 02 00 00        	mov	rax, qword ptr [rbp + 0x2d4]
140a3bba3: 48 89 85 30 01 00 00        	mov	qword ptr [rbp + 0x130], rax
140a3bbaa: 0f 10 85 94 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x294]
140a3bbb1: 0f 10 8d a4 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x2a4]
140a3bbb8: 0f 10 95 b4 02 00 00        	movups	xmm2, xmmword ptr [rbp + 0x2b4]
140a3bbbf: 0f 10 9d c4 02 00 00        	movups	xmm3, xmmword ptr [rbp + 0x2c4]
140a3bbc6: 0f 29 9d 20 01 00 00        	movaps	xmmword ptr [rbp + 0x120], xmm3
140a3bbcd: 0f 29 95 10 01 00 00        	movaps	xmmword ptr [rbp + 0x110], xmm2
140a3bbd4: 0f 29 8d 00 01 00 00        	movaps	xmmword ptr [rbp + 0x100], xmm1
140a3bbdb: 0f 29 85 f0 00 00 00        	movaps	xmmword ptr [rbp + 0xf0], xmm0
140a3bbe2: 0f b6 8d dc 02 00 00        	movzx	ecx, byte ptr [rbp + 0x2dc]
140a3bbe9: 0f 10 85 14 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x114]
140a3bbf0: 0f 11 85 93 02 00 00        	movups	xmmword ptr [rbp + 0x293], xmm0
140a3bbf7: 0f 10 85 24 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x124]
140a3bbfe: 0f 11 85 a3 02 00 00        	movups	xmmword ptr [rbp + 0x2a3], xmm0
140a3bc05: 8b 85 34 01 00 00           	mov	eax, dword ptr [rbp + 0x134]
140a3bc0b: 89 85 b3 02 00 00           	mov	dword ptr [rbp + 0x2b3], eax
140a3bc11: 8b 85 10 01 00 00           	mov	eax, dword ptr [rbp + 0x110]
140a3bc17: 89 45 40                    	mov	dword ptr [rbp + 0x40], eax
140a3bc1a: 0f 28 85 f0 00 00 00        	movaps	xmm0, xmmword ptr [rbp + 0xf0]
140a3bc21: 0f 28 8d 00 01 00 00        	movaps	xmm1, xmmword ptr [rbp + 0x100]
140a3bc28: 0f 29 4d 30                 	movaps	xmmword ptr [rbp + 0x30], xmm1
140a3bc2c: 0f 29 45 20                 	movaps	xmmword ptr [rbp + 0x20], xmm0
140a3bc30: 88 8d 87 04 00 00           	mov	byte ptr [rbp + 0x487], cl
140a3bc36: 88 4d 44                    	mov	byte ptr [rbp + 0x44], cl
140a3bc39: 8b 85 b3 02 00 00           	mov	eax, dword ptr [rbp + 0x2b3]
140a3bc3f: 89 85 e0 00 00 00           	mov	dword ptr [rbp + 0xe0], eax
140a3bc45: 0f 10 85 93 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x293]
140a3bc4c: 0f 10 8d a3 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x2a3]
140a3bc53: 0f 29 8d d0 00 00 00        	movaps	xmmword ptr [rbp + 0xd0], xmm1
140a3bc5a: 0f 29 85 c0 00 00 00        	movaps	xmmword ptr [rbp + 0xc0], xmm0
140a3bc61: 48 8d 8d 30 04 00 00        	lea	rcx, [rbp + 0x430]
140a3bc68: 48 89 fa                    	mov	rdx, rdi
140a3bc6b: 41 b8 4c 00 00 00           	mov	r8d, 0x4c
140a3bc71: e8 2a 96 19 00              	call	0x140bd52a0 <.text+0xbd42a0>
140a3bc76: 0f 10 85 30 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x430]
140a3bc7d: 0f 10 8d 40 04 00 00        	movups	xmm1, xmmword ptr [rbp + 0x440]
140a3bc84: 0f 29 8d 00 01 00 00        	movaps	xmmword ptr [rbp + 0x100], xmm1
140a3bc8b: 0f 29 85 f0 00 00 00        	movaps	xmmword ptr [rbp + 0xf0], xmm0
140a3bc92: 48 8d 8d a0 02 00 00        	lea	rcx, [rbp + 0x2a0]
140a3bc99: 48 8d 95 f0 00 00 00        	lea	rdx, [rbp + 0xf0]
140a3bca0: e8 3b b0 19 00              	call	0x140bd6ce0 <.text+0xbd5ce0>
140a3bca5: 48 8d 05 2c 18 09 02        	lea	rax, [rip + 0x209182c]  # 0x142acd4d8
140a3bcac: 48 89 85 90 02 00 00        	mov	qword ptr [rbp + 0x290], rax
140a3bcb3: 48 c7 85 98 02 00 00 17 00 00 00    	mov	qword ptr [rbp + 0x298], 0x17
140a3bcbe: 48 8d 95 90 02 00 00        	lea	rdx, [rbp + 0x290]
140a3bcc5: 4c 89 e1                    	mov	rcx, r12
140a3bcc8: e8 03 d8 1a 00              	call	0x140be94d0 <.text+0xbe84d0>
140a3bccd: 0f 10 85 30 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x430]
140a3bcd4: 0f 10 8d 40 04 00 00        	movups	xmm1, xmmword ptr [rbp + 0x440]
140a3bcdb: 0f 11 85 a0 00 00 00        	movups	xmmword ptr [rbp + 0xa0], xmm0
140a3bce2: 0f 11 8d b0 00 00 00        	movups	xmmword ptr [rbp + 0xb0], xmm1
140a3bce9: 89 85 9c 00 00 00           	mov	dword ptr [rbp + 0x9c], eax
140a3bcef: 48 8d 8d 30 04 00 00        	lea	rcx, [rbp + 0x430]
140a3bcf6: 48 89 fa                    	mov	rdx, rdi
140a3bcf9: 41 b8 53 00 00 00           	mov	r8d, 0x53
140a3bcff: e8 9c 95 19 00              	call	0x140bd52a0 <.text+0xbd42a0>
140a3bd04: 0f 10 85 30 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x430]
140a3bd0b: 0f 10 8d 40 04 00 00        	movups	xmm1, xmmword ptr [rbp + 0x440]
140a3bd12: 0f 29 8d 00 01 00 00        	movaps	xmmword ptr [rbp + 0x100], xmm1
140a3bd19: 0f 29 85 f0 00 00 00        	movaps	xmmword ptr [rbp + 0xf0], xmm0
140a3bd20: 48 8d 8d a0 02 00 00        	lea	rcx, [rbp + 0x2a0]
140a3bd27: 48 8d 95 f0 00 00 00        	lea	rdx, [rbp + 0xf0]
140a3bd2e: e8 ad af 19 00              	call	0x140bd6ce0 <.text+0xbd5ce0>
140a3bd33: 48 8d 05 b5 17 09 02        	lea	rax, [rip + 0x20917b5]  # 0x142acd4ef
140a3bd3a: 48 89 85 90 02 00 00        	mov	qword ptr [rbp + 0x290], rax
140a3bd41: 48 c7 85 98 02 00 00 1a 00 00 00    	mov	qword ptr [rbp + 0x298], 0x1a
140a3bd4c: 48 8d 95 90 02 00 00        	lea	rdx, [rbp + 0x290]
140a3bd53: 4c 89 e1                    	mov	rcx, r12
140a3bd56: e8 75 d7 1a 00              	call	0x140be94d0 <.text+0xbe84d0>
140a3bd5b: 0f 10 85 30 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x430]
140a3bd62: 0f 10 8d 40 04 00 00        	movups	xmm1, xmmword ptr [rbp + 0x440]
140a3bd69: 0f 11 45 7c                 	movups	xmmword ptr [rbp + 0x7c], xmm0
140a3bd6d: 0f 11 8d 8c 00 00 00        	movups	xmmword ptr [rbp + 0x8c], xmm1
140a3bd74: 89 45 78                    	mov	dword ptr [rbp + 0x78], eax
140a3bd77: 48 8d 8d 30 04 00 00        	lea	rcx, [rbp + 0x430]
140a3bd7e: 4c 89 ea                    	mov	rdx, r13
140a3bd81: 41 b8 09 00 00 00           	mov	r8d, 0x9
140a3bd87: e8 14 95 19 00              	call	0x140bd52a0 <.text+0xbd42a0>
140a3bd8c: 0f 10 85 30 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x430]
140a3bd93: 0f 10 8d 40 04 00 00        	movups	xmm1, xmmword ptr [rbp + 0x440]
140a3bd9a: 0f 29 8d 00 01 00 00        	movaps	xmmword ptr [rbp + 0x100], xmm1
140a3bda1: 0f 29 85 f0 00 00 00        	movaps	xmmword ptr [rbp + 0xf0], xmm0
140a3bda8: 48 8d 8d a0 02 00 00        	lea	rcx, [rbp + 0x2a0]
140a3bdaf: 48 8d 9d f0 00 00 00        	lea	rbx, [rbp + 0xf0]
140a3bdb6: 48 89 da                    	mov	rdx, rbx
140a3bdb9: e8 22 af 19 00              	call	0x140bd6ce0 <.text+0xbd5ce0>
140a3bdbe: 48 8d 05 44 17 09 02        	lea	rax, [rip + 0x2091744]  # 0x142acd509
140a3bdc5: 48 89 85 90 02 00 00        	mov	qword ptr [rbp + 0x290], rax
140a3bdcc: 48 c7 85 98 02 00 00 11 00 00 00    	mov	qword ptr [rbp + 0x298], 0x11
140a3bdd7: 48 8d 95 90 02 00 00        	lea	rdx, [rbp + 0x290]
140a3bdde: 4c 89 e1                    	mov	rcx, r12
140a3bde1: e8 ea d6 1a 00              	call	0x140be94d0 <.text+0xbe84d0>
140a3bde6: 0f 10 85 30 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x430]
140a3bded: 0f 10 8d 40 04 00 00        	movups	xmm1, xmmword ptr [rbp + 0x440]
140a3bdf4: 0f 11 45 58                 	movups	xmmword ptr [rbp + 0x58], xmm0
140a3bdf8: 0f 11 4d 68                 	movups	xmmword ptr [rbp + 0x68], xmm1
140a3bdfc: 89 45 54                    	mov	dword ptr [rbp + 0x54], eax
140a3bdff: 4c 89 64 24 30              	mov	qword ptr [rsp + 0x30], r12
140a3be04: 4c 89 7c 24 28              	mov	qword ptr [rsp + 0x28], r15
140a3be09: 48 c7 44 24 20 16 00 00 00  	mov	qword ptr [rsp + 0x20], 0x16
140a3be12: 48 8d 15 01 17 09 02        	lea	rdx, [rip + 0x2091701]  # 0x142acd51a
140a3be19: 4c 8d 0d 07 17 09 02        	lea	r9, [rip + 0x2091707]   # 0x142acd527
140a3be20: 48 8d b5 90 02 00 00        	lea	rsi, [rbp + 0x290]
140a3be27: 41 b8 0d 00 00 00           	mov	r8d, 0xd
140a3be2d: 48 89 f1                    	mov	rcx, rsi
140a3be30: e8 6b 74 bc 00              	call	0x1416032a0 <.text+0x16022a0>
140a3be35: 48 8b 85 88 02 00 00        	mov	rax, qword ptr [rbp + 0x288]
140a3be3c: 48 89 85 30 04 00 00        	mov	qword ptr [rbp + 0x430], rax
140a3be43: c7 85 f0 00 00 00 02 00 00 00       	mov	dword ptr [rbp + 0xf0], 0x2
140a3be4d: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
140a3be52: 48 8d 8d 30 04 00 00        	lea	rcx, [rbp + 0x430]
140a3be59: 49 b8 5e a9 e9 1c 54 8e 97 d9       	movabs	r8, -0x266871abe31656a2
140a3be63: 48 89 f2                    	mov	rdx, rsi
140a3be66: 41 b1 19                    	mov	r9b, 0x19
140a3be69: e8 d2 23 eb ff              	call	0x1408ee240 <.text+0x8ed240>
140a3be6e: 48 8d b5 f0 00 00 00        	lea	rsi, [rbp + 0xf0]
140a3be75: 48 8d 9d 90 02 00 00        	lea	rbx, [rbp + 0x290]
140a3be7c: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a3be82: 48 89 f1                    	mov	rcx, rsi
140a3be85: 48 89 da                    	mov	rdx, rbx
140a3be88: e8 73 7d ee 01              	call	0x142923c00 <.text+0x2922c00>
140a3be8d: 48 8d 45 54                 	lea	rax, [rbp + 0x54]
140a3be91: 48 89 85 30 04 00 00        	mov	qword ptr [rbp + 0x430], rax
140a3be98: c7 85 90 02 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x290], 0x2
140a3bea2: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
140a3bea7: 48 8d 8d 30 04 00 00        	lea	rcx, [rbp + 0x430]
140a3beae: 49 b8 b2 92 98 e6 3a 9a 21 88       	movabs	r8, -0x77de65c519676d4e
140a3beb8: 48 89 f2                    	mov	rdx, rsi
140a3bebb: 41 b1 32                    	mov	r9b, 0x32
140a3bebe: e8 3d 1e eb ff              	call	0x1408edd00 <.text+0x8ecd00>
140a3bec3: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
140a3beca: 48 8d 95 f0 00 00 00        	lea	rdx, [rbp + 0xf0]
140a3bed1: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a3bed7: e8 24 7d ee 01              	call	0x142923c00 <.text+0x2922c00>
140a3bedc: 4c 89 e8                    	mov	rax, r13
140a3bedf: 48 c1 e8 20                 	shr	rax, 0x20
140a3bee3: 0f 10 85 30 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x330]
140a3beea: 0f 29 85 70 04 00 00        	movaps	xmmword ptr [rbp + 0x470], xmm0
140a3bef1: 0f 10 85 20 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x320]
140a3bef8: 0f 29 85 60 04 00 00        	movaps	xmmword ptr [rbp + 0x460], xmm0
140a3beff: 0f 10 85 10 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x310]
140a3bf06: 0f 29 85 50 04 00 00        	movaps	xmmword ptr [rbp + 0x450], xmm0
140a3bf0d: 0f 10 85 00 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x300]
140a3bf14: 0f 29 85 40 04 00 00        	movaps	xmmword ptr [rbp + 0x440], xmm0
140a3bf1b: 0f 10 85 f0 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2f0]
140a3bf22: 0f 29 85 30 04 00 00        	movaps	xmmword ptr [rbp + 0x430], xmm0
140a3bf29: 48 8b 8d c8 03 00 00        	mov	rcx, qword ptr [rbp + 0x3c8]
140a3bf30: 48 8b 95 10 04 00 00        	mov	rdx, qword ptr [rbp + 0x410]
140a3bf37: 4c 8b 85 a0 02 00 00        	mov	r8, qword ptr [rbp + 0x2a0]
140a3bf3e: 4c 89 85 00 01 00 00        	mov	qword ptr [rbp + 0x100], r8
140a3bf45: 0f 10 85 90 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x290]
140a3bf4c: 0f 29 85 f0 00 00 00        	movaps	xmmword ptr [rbp + 0xf0], xmm0
140a3bf53: 0f 10 85 a8 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2a8]
140a3bf5a: 0f 29 85 20 01 00 00        	movaps	xmmword ptr [rbp + 0x120], xmm0
140a3bf61: 4c 8b 85 b8 02 00 00        	mov	r8, qword ptr [rbp + 0x2b8]
140a3bf68: 4c 89 85 30 01 00 00        	mov	qword ptr [rbp + 0x130], r8
140a3bf6f: 0f 10 85 c0 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2c0]
140a3bf76: 0f 11 85 08 01 00 00        	movups	xmmword ptr [rbp + 0x108], xmm0
140a3bf7d: 4c 8b 85 d0 02 00 00        	mov	r8, qword ptr [rbp + 0x2d0]
140a3bf84: 4c 89 85 18 01 00 00        	mov	qword ptr [rbp + 0x118], r8
140a3bf8b: 0f 10 85 40 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x340]
140a3bf92: 0f 10 8d 50 03 00 00        	movups	xmm1, xmmword ptr [rbp + 0x350]
140a3bf99: 0f 10 95 60 03 00 00        	movups	xmm2, xmmword ptr [rbp + 0x360]
140a3bfa0: 0f 10 9d 70 03 00 00        	movups	xmm3, xmmword ptr [rbp + 0x370]
140a3bfa7: 0f 11 85 68 01 00 00        	movups	xmmword ptr [rbp + 0x168], xmm0
140a3bfae: 0f 11 8d 78 01 00 00        	movups	xmmword ptr [rbp + 0x178], xmm1
140a3bfb5: 0f 11 95 88 01 00 00        	movups	xmmword ptr [rbp + 0x188], xmm2
140a3bfbc: 0f 11 9d 98 01 00 00        	movups	xmmword ptr [rbp + 0x198], xmm3
140a3bfc3: 0f 10 85 80 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x380]
140a3bfca: 0f 11 85 a8 01 00 00        	movups	xmmword ptr [rbp + 0x1a8], xmm0
140a3bfd1: 0f 10 85 90 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x390]
140a3bfd8: 0f 11 85 b8 01 00 00        	movups	xmmword ptr [rbp + 0x1b8], xmm0
140a3bfdf: 4c 8b 85 e8 02 00 00        	mov	r8, qword ptr [rbp + 0x2e8]
140a3bfe6: 4c 89 85 48 01 00 00        	mov	qword ptr [rbp + 0x148], r8
140a3bfed: 0f 10 85 d8 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2d8]
140a3bff4: 0f 11 85 38 01 00 00        	movups	xmmword ptr [rbp + 0x138], xmm0
140a3bffb: 48 89 95 d0 01 00 00        	mov	qword ptr [rbp + 0x1d0], rdx
140a3c002: 48 89 8d c8 01 00 00        	mov	qword ptr [rbp + 0x1c8], rcx
140a3c009: c7 85 50 01 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x150], 0x1
140a3c013: 44 89 ad 54 01 00 00        	mov	dword ptr [rbp + 0x154], r13d
140a3c01a: 89 85 58 01 00 00           	mov	dword ptr [rbp + 0x158], eax
140a3c020: c7 85 5c 01 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x15c], 0x1
140a3c02a: 48 8d 8d 30 04 00 00        	lea	rcx, [rbp + 0x430]
140a3c031: 48 8d 95 f0 00 00 00        	lea	rdx, [rbp + 0xf0]
140a3c038: e8 93 14 fa ff              	call	0x1409dd4d0 <.text+0x9dc4d0>
140a3c03d: 4c 89 b5 e8 00 00 00        	mov	qword ptr [rbp + 0xe8], r14
140a3c044: 80 bd c6 03 00 00 08        	cmp	byte ptr [rbp + 0x3c6], 0x8
140a3c04b: 74 0c                       	je	0x140a3c059 <.text+0xa3b059>
140a3c04d: 48 8d 8d b0 03 00 00        	lea	rcx, [rbp + 0x3b0]
140a3c054: e8 57 64 1a 00              	call	0x140be24b0 <.text+0xbe14b0>
140a3c059: 4c 8b b5 30 05 00 00        	mov	r14, qword ptr [rbp + 0x530]
140a3c060: 4c 89 64 24 30              	mov	qword ptr [rsp + 0x30], r12
140a3c065: 4c 89 7c 24 28              	mov	qword ptr [rsp + 0x28], r15
140a3c06a: 48 c7 44 24 20 17 00 00 00  	mov	qword ptr [rsp + 0x20], 0x17
140a3c073: 48 8d 15 c3 14 09 02        	lea	rdx, [rip + 0x20914c3]  # 0x142acd53d
140a3c07a: 4c 8d 0d ca 14 09 02        	lea	r9, [rip + 0x20914ca]   # 0x142acd54b
140a3c081: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
140a3c088: 41 b8 0e 00 00 00           	mov	r8d, 0xe
140a3c08e: e8 0d 72 bc 00              	call	0x1416032a0 <.text+0x16022a0>
140a3c093: 80 bd c6 03 00 00 08        	cmp	byte ptr [rbp + 0x3c6], 0x8
140a3c09a: 74 38                       	je	0x140a3c0d4 <.text+0xa3b0d4>
140a3c09c: 0f b6 05 ad 8c bf 02        	movzx	eax, byte ptr [rip + 0x2bf8cad] # 0x143634d50
140a3c0a3: 83 f0 01                    	xor	eax, 0x1
140a3c0a6: 45 31 c9                    	xor	r9d, r9d
140a3c0a9: 41 80 7e 24 00              	cmp	byte ptr [r14 + 0x24], 0x0
140a3c0ae: 44 0f 45 c8                 	cmovne	r9d, eax
140a3c0b2: 48 8d 8d a0 03 00 00        	lea	rcx, [rbp + 0x3a0]
140a3c0b9: ba e8 00 00 00              	mov	edx, 0xe8
140a3c0be: 48 03 95 c8 03 00 00        	add	rdx, qword ptr [rbp + 0x3c8]
140a3c0c5: 49 b8 02 d9 ee 60 b7 09 a1 a9       	movabs	r8, -0x565ef6489f1126fe
140a3c0cf: e8 9c f4 f2 ff              	call	0x14096b570 <.text+0x96a570>
140a3c0d4: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a3c0da: 48 89 f1                    	mov	rcx, rsi
140a3c0dd: 48 89 da                    	mov	rdx, rbx
140a3c0e0: e8 1b 7b ee 01              	call	0x142923c00 <.text+0x2922c00>
140a3c0e5: 4c 89 b5 30 04 00 00        	mov	qword ptr [rbp + 0x430], r14
140a3c0ec: c7 85 90 02 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x290], 0x2
140a3c0f6: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
140a3c0fb: 48 8d 8d 30 04 00 00        	lea	rcx, [rbp + 0x430]
140a3c102: 49 b8 19 ad 7d 07 3c 60 5a 18       	movabs	r8, 0x185a603c077dad19
140a3c10c: 48 89 f2                    	mov	rdx, rsi
140a3c10f: 41 b1 19                    	mov	r9b, 0x19
140a3c112: e8 79 2f f3 ff              	call	0x14096f090 <.text+0x96e090>
140a3c117: 4c 8b ad 20 05 00 00        	mov	r13, qword ptr [rbp + 0x520]
140a3c11e: 48 8d b5 90 02 00 00        	lea	rsi, [rbp + 0x290]
140a3c125: 48 8d 9d f0 00 00 00        	lea	rbx, [rbp + 0xf0]
140a3c12c: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a3c132: 48 89 f1                    	mov	rcx, rsi
140a3c135: 48 89 da                    	mov	rdx, rbx
140a3c138: e8 c3 7a ee 01              	call	0x142923c00 <.text+0x2922c00>
140a3c13d: 4c 89 ad 30 04 00 00        	mov	qword ptr [rbp + 0x430], r13
140a3c144: c7 85 f0 00 00 00 02 00 00 00       	mov	dword ptr [rbp + 0xf0], 0x2
140a3c14e: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
140a3c153: 48 8d 8d 30 04 00 00        	lea	rcx, [rbp + 0x430]
140a3c15a: 49 b8 e3 6b 5a 83 cd 4c a7 b8       	movabs	r8, -0x4758b3327ca5941d
140a3c164: 48 89 f2                    	mov	rdx, rsi
140a3c167: 41 b1 19                    	mov	r9b, 0x19
140a3c16a: e8 d1 20 eb ff              	call	0x1408ee240 <.text+0x8ed240>
140a3c16f: 4c 8b ad 10 05 00 00        	mov	r13, qword ptr [rbp + 0x510]
140a3c176: 48 8d b5 f0 00 00 00        	lea	rsi, [rbp + 0xf0]
140a3c17d: 48 8d 9d 90 02 00 00        	lea	rbx, [rbp + 0x290]
140a3c184: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a3c18a: 48 89 f1                    	mov	rcx, rsi
140a3c18d: 48 89 da                    	mov	rdx, rbx
140a3c190: e8 6b 7a ee 01              	call	0x142923c00 <.text+0x2922c00>
140a3c195: 4c 89 ad 30 04 00 00        	mov	qword ptr [rbp + 0x430], r13
140a3c19c: c7 85 90 02 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x290], 0x2
140a3c1a6: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
140a3c1ab: 48 8d 8d 30 04 00 00        	lea	rcx, [rbp + 0x430]
140a3c1b2: 49 b8 fc fb 49 a0 9d 57 61 48       	movabs	r8, 0x4861579da049fbfc
140a3c1bc: 48 89 f2                    	mov	rdx, rsi
140a3c1bf: 41 b1 19                    	mov	r9b, 0x19
140a3c1c2: e8 79 20 eb ff              	call	0x1408ee240 <.text+0x8ed240>
140a3c1c7: 48 8d b5 90 02 00 00        	lea	rsi, [rbp + 0x290]
140a3c1ce: 48 8d 9d f0 00 00 00        	lea	rbx, [rbp + 0xf0]
140a3c1d5: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a3c1db: 48 89 f1                    	mov	rcx, rsi
140a3c1de: 48 89 da                    	mov	rdx, rbx
140a3c1e1: e8 1a 7a ee 01              	call	0x142923c00 <.text+0x2922c00>
140a3c1e6: 48 8d 85 9c 00 00 00        	lea	rax, [rbp + 0x9c]
140a3c1ed: 48 89 85 30 04 00 00        	mov	qword ptr [rbp + 0x430], rax
140a3c1f4: c7 85 f0 00 00 00 02 00 00 00       	mov	dword ptr [rbp + 0xf0], 0x2
140a3c1fe: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
140a3c203: 48 8d 8d 30 04 00 00        	lea	rcx, [rbp + 0x430]
140a3c20a: 49 b8 0f d3 a0 2c cb 1c 22 e1       	movabs	r8, -0x1edde334d35f2cf1
140a3c214: 48 89 f2                    	mov	rdx, rsi
140a3c217: 41 b1 32                    	mov	r9b, 0x32
140a3c21a: e8 e1 1a eb ff              	call	0x1408edd00 <.text+0x8ecd00>
140a3c21f: 48 8d b5 f0 00 00 00        	lea	rsi, [rbp + 0xf0]
140a3c226: 48 8d 9d 90 02 00 00        	lea	rbx, [rbp + 0x290]
140a3c22d: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a3c233: 48 89 f1                    	mov	rcx, rsi
140a3c236: 48 89 da                    	mov	rdx, rbx
140a3c239: e8 c2 79 ee 01              	call	0x142923c00 <.text+0x2922c00>
140a3c23e: 48 8d 45 78                 	lea	rax, [rbp + 0x78]
140a3c242: 48 89 85 30 04 00 00        	mov	qword ptr [rbp + 0x430], rax
140a3c249: c7 85 90 02 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x290], 0x2
140a3c253: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
140a3c258: 48 8d 8d 30 04 00 00        	lea	rcx, [rbp + 0x430]
140a3c25f: 49 b8 7e 57 47 bd c7 77 88 a8       	movabs	r8, -0x5777883842b8a882
140a3c269: 48 89 f2                    	mov	rdx, rsi
140a3c26c: 41 b1 32                    	mov	r9b, 0x32
140a3c26f: e8 8c 1a eb ff              	call	0x1408edd00 <.text+0x8ecd00>
140a3c274: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
140a3c27b: 48 8d 95 f0 00 00 00        	lea	rdx, [rbp + 0xf0]
140a3c282: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a3c288: e8 73 79 ee 01              	call	0x142923c00 <.text+0x2922c00>
140a3c28d: 49 89 fd                    	mov	r13, rdi
140a3c290: 49 c1 ed 20                 	shr	r13, 0x20
140a3c294: 0f 10 85 30 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x330]
140a3c29b: 0f 29 85 70 04 00 00        	movaps	xmmword ptr [rbp + 0x470], xmm0
140a3c2a2: 0f 10 85 20 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x320]
140a3c2a9: 0f 29 85 60 04 00 00        	movaps	xmmword ptr [rbp + 0x460], xmm0
140a3c2b0: 0f 10 85 10 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x310]
140a3c2b7: 0f 29 85 50 04 00 00        	movaps	xmmword ptr [rbp + 0x450], xmm0
140a3c2be: 0f 10 85 00 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x300]
140a3c2c5: 0f 29 85 40 04 00 00        	movaps	xmmword ptr [rbp + 0x440], xmm0
140a3c2cc: 0f 10 85 f0 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2f0]
140a3c2d3: 0f 29 85 30 04 00 00        	movaps	xmmword ptr [rbp + 0x430], xmm0
140a3c2da: 48 8b 85 c8 03 00 00        	mov	rax, qword ptr [rbp + 0x3c8]
140a3c2e1: 48 8b 8d 10 04 00 00        	mov	rcx, qword ptr [rbp + 0x410]
140a3c2e8: 48 8b 95 a0 02 00 00        	mov	rdx, qword ptr [rbp + 0x2a0]
140a3c2ef: 48 89 95 00 01 00 00        	mov	qword ptr [rbp + 0x100], rdx
140a3c2f6: 0f 10 85 90 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x290]
140a3c2fd: 0f 29 85 f0 00 00 00        	movaps	xmmword ptr [rbp + 0xf0], xmm0
140a3c304: 0f 10 85 a8 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2a8]
140a3c30b: 0f 29 85 20 01 00 00        	movaps	xmmword ptr [rbp + 0x120], xmm0
140a3c312: 48 8b 95 b8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2b8]
140a3c319: 48 89 95 30 01 00 00        	mov	qword ptr [rbp + 0x130], rdx
140a3c320: 0f 10 85 c0 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2c0]
140a3c327: 0f 11 85 08 01 00 00        	movups	xmmword ptr [rbp + 0x108], xmm0
140a3c32e: 48 8b 95 d0 02 00 00        	mov	rdx, qword ptr [rbp + 0x2d0]
140a3c335: 48 89 95 18 01 00 00        	mov	qword ptr [rbp + 0x118], rdx
140a3c33c: 0f 10 85 40 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x340]
140a3c343: 0f 10 8d 50 03 00 00        	movups	xmm1, xmmword ptr [rbp + 0x350]
140a3c34a: 0f 10 95 60 03 00 00        	movups	xmm2, xmmword ptr [rbp + 0x360]
140a3c351: 0f 10 9d 70 03 00 00        	movups	xmm3, xmmword ptr [rbp + 0x370]
140a3c358: 0f 11 85 68 01 00 00        	movups	xmmword ptr [rbp + 0x168], xmm0
140a3c35f: 0f 11 8d 78 01 00 00        	movups	xmmword ptr [rbp + 0x178], xmm1
140a3c366: 0f 11 95 88 01 00 00        	movups	xmmword ptr [rbp + 0x188], xmm2
140a3c36d: 0f 11 9d 98 01 00 00        	movups	xmmword ptr [rbp + 0x198], xmm3
140a3c374: 0f 10 85 80 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x380]
140a3c37b: 0f 11 85 a8 01 00 00        	movups	xmmword ptr [rbp + 0x1a8], xmm0
140a3c382: 0f 10 85 90 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x390]
140a3c389: 0f 11 85 b8 01 00 00        	movups	xmmword ptr [rbp + 0x1b8], xmm0
140a3c390: 48 8b 95 e8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2e8]
140a3c397: 48 89 95 48 01 00 00        	mov	qword ptr [rbp + 0x148], rdx
140a3c39e: 0f 10 85 d8 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2d8]
140a3c3a5: 0f 11 85 38 01 00 00        	movups	xmmword ptr [rbp + 0x138], xmm0
140a3c3ac: 48 89 8d d0 01 00 00        	mov	qword ptr [rbp + 0x1d0], rcx
140a3c3b3: 48 89 85 c8 01 00 00        	mov	qword ptr [rbp + 0x1c8], rax
140a3c3ba: c7 85 50 01 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x150], 0x1
140a3c3c4: 89 bd 54 01 00 00           	mov	dword ptr [rbp + 0x154], edi
140a3c3ca: 44 89 ad 58 01 00 00        	mov	dword ptr [rbp + 0x158], r13d
140a3c3d1: c7 85 5c 01 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x15c], 0x1
140a3c3db: 48 8d 8d 30 04 00 00        	lea	rcx, [rbp + 0x430]
140a3c3e2: 48 8d 95 f0 00 00 00        	lea	rdx, [rbp + 0xf0]
140a3c3e9: e8 e2 10 fa ff              	call	0x1409dd4d0 <.text+0x9dc4d0>
140a3c3ee: f3 0f 10 b5 38 05 00 00     	movss	xmm6, dword ptr [rbp + 0x538]
140a3c3f6: 80 bd c6 03 00 00 08        	cmp	byte ptr [rbp + 0x3c6], 0x8
140a3c3fd: 74 0c                       	je	0x140a3c40b <.text+0xa3b40b>
140a3c3ff: 48 8d 8d b0 03 00 00        	lea	rcx, [rbp + 0x3b0]
140a3c406: e8 a5 60 1a 00              	call	0x140be24b0 <.text+0xbe14b0>
140a3c40b: 41 8b 46 20                 	mov	eax, dword ptr [r14 + 0x20]
140a3c40f: 89 45 10                    	mov	dword ptr [rbp + 0x10], eax
140a3c412: 41 0f 10 06                 	movups	xmm0, xmmword ptr [r14]
140a3c416: 41 0f 10 4e 10              	movups	xmm1, xmmword ptr [r14 + 0x10]
140a3c41b: 0f 29 4d 00                 	movaps	xmmword ptr [rbp], xmm1
140a3c41f: 0f 29 45 f0                 	movaps	xmmword ptr [rbp - 0x10], xmm0
140a3c423: 41 0f b6 5e 24              	movzx	ebx, byte ptr [r14 + 0x24]
140a3c428: 8b 85 e0 00 00 00           	mov	eax, dword ptr [rbp + 0xe0]
140a3c42e: 89 45 e0                    	mov	dword ptr [rbp - 0x20], eax
140a3c431: 0f 28 85 c0 00 00 00        	movaps	xmm0, xmmword ptr [rbp + 0xc0]
140a3c438: 0f 28 8d d0 00 00 00        	movaps	xmm1, xmmword ptr [rbp + 0xd0]
140a3c43f: 0f 29 4d d0                 	movaps	xmmword ptr [rbp - 0x30], xmm1
140a3c443: 0f 29 45 c0                 	movaps	xmmword ptr [rbp - 0x40], xmm0
140a3c447: 49 8b 8c 24 90 01 00 00     	mov	rcx, qword ptr [r12 + 0x190]
140a3c44f: e8 4c cb 1b 00              	call	0x140bf8fa0 <.text+0xbf7fa0>
140a3c454: 83 fa 21                    	cmp	edx, 0x21
140a3c457: 0f 92 c1                    	setb	cl
140a3c45a: 20 c1                       	and	cl, al
140a3c45c: 44 0f b6 c9                 	movzx	r9d, cl
140a3c460: 0f 57 c0                    	xorps	xmm0, xmm0
140a3c463: 31 c0                       	xor	eax, eax
140a3c465: 0f 2e f0                    	ucomiss	xmm6, xmm0
140a3c468: 0f 97 c0                    	seta	al
140a3c46b: 4c 89 64 24 30              	mov	qword ptr [rsp + 0x30], r12
140a3c470: 4c 89 7c 24 28              	mov	qword ptr [rsp + 0x28], r15
140a3c475: 89 44 24 20                 	mov	dword ptr [rsp + 0x20], eax
140a3c479: 48 8d 15 e2 10 09 02        	lea	rdx, [rip + 0x20910e2]  # 0x142acd562
140a3c480: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
140a3c487: 41 b8 04 00 00 00           	mov	r8d, 0x4
140a3c48d: e8 ce 49 bc 00              	call	0x141600e60 <.text+0x15ffe60>
140a3c492: 80 bd c6 03 00 00 08        	cmp	byte ptr [rbp + 0x3c6], 0x8
140a3c499: 74 3c                       	je	0x140a3c4d7 <.text+0xa3b4d7>
140a3c49b: 0f b6 05 ae 88 bf 02        	movzx	eax, byte ptr [rip + 0x2bf88ae] # 0x143634d50
140a3c4a2: 22 9d 87 04 00 00           	and	bl, byte ptr [rbp + 0x487]
140a3c4a8: 83 f0 01                    	xor	eax, 0x1
140a3c4ab: 45 31 c9                    	xor	r9d, r9d
140a3c4ae: f6 c3 01                    	test	bl, 0x1
140a3c4b1: 44 0f 45 c8                 	cmovne	r9d, eax
140a3c4b5: 48 8d 8d a0 03 00 00        	lea	rcx, [rbp + 0x3a0]
140a3c4bc: ba e8 00 00 00              	mov	edx, 0xe8
140a3c4c1: 48 03 95 c8 03 00 00        	add	rdx, qword ptr [rbp + 0x3c8]
140a3c4c8: 49 b8 02 d9 ee 60 b7 09 a1 a9       	movabs	r8, -0x565ef6489f1126fe
140a3c4d2: e8 99 f0 f2 ff              	call	0x14096b570 <.text+0x96a570>
140a3c4d7: 48 8d 9d 90 02 00 00        	lea	rbx, [rbp + 0x290]
140a3c4de: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a3c4e4: 48 89 f1                    	mov	rcx, rsi
140a3c4e7: 48 89 da                    	mov	rdx, rbx
140a3c4ea: e8 11 77 ee 01              	call	0x142923c00 <.text+0x2922c00>
140a3c4ef: 48 8b 85 88 02 00 00        	mov	rax, qword ptr [rbp + 0x288]
140a3c4f6: 48 89 85 30 04 00 00        	mov	qword ptr [rbp + 0x430], rax
140a3c4fd: c7 85 90 02 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x290], 0x2
140a3c507: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
140a3c50c: 48 8d 8d 30 04 00 00        	lea	rcx, [rbp + 0x430]
140a3c513: 49 b8 5e a9 e9 1c 54 8e 97 d9       	movabs	r8, -0x266871abe31656a2
140a3c51d: 48 89 f2                    	mov	rdx, rsi
140a3c520: 41 b1 19                    	mov	r9b, 0x19
140a3c523: e8 18 1d eb ff              	call	0x1408ee240 <.text+0x8ed240>
140a3c528: 4c 8b bd 18 05 00 00        	mov	r15, qword ptr [rbp + 0x518]
140a3c52f: 48 8d b5 90 02 00 00        	lea	rsi, [rbp + 0x290]
140a3c536: 48 8d 9d f0 00 00 00        	lea	rbx, [rbp + 0xf0]
140a3c53d: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a3c543: 48 89 f1                    	mov	rcx, rsi
140a3c546: 48 89 da                    	mov	rdx, rbx
140a3c549: e8 b2 76 ee 01              	call	0x142923c00 <.text+0x2922c00>
140a3c54e: 48 8d 45 54                 	lea	rax, [rbp + 0x54]
140a3c552: 48 89 85 30 04 00 00        	mov	qword ptr [rbp + 0x430], rax
140a3c559: c7 85 f0 00 00 00 02 00 00 00       	mov	dword ptr [rbp + 0xf0], 0x2
140a3c563: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
140a3c568: 48 8d 8d 30 04 00 00        	lea	rcx, [rbp + 0x430]
140a3c56f: 49 b8 48 00 23 94 b2 f9 78 cb       	movabs	r8, -0x3487064d6bdcffb8
140a3c579: 48 89 f2                    	mov	rdx, rsi
140a3c57c: 41 b1 19                    	mov	r9b, 0x19
140a3c57f: e8 bc 1c eb ff              	call	0x1408ee240 <.text+0x8ed240>
140a3c584: 48 8d b5 f0 00 00 00        	lea	rsi, [rbp + 0xf0]
140a3c58b: 48 8d 9d 90 02 00 00        	lea	rbx, [rbp + 0x290]
140a3c592: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a3c598: 48 89 f1                    	mov	rcx, rsi
140a3c59b: 48 89 da                    	mov	rdx, rbx
140a3c59e: e8 5d 76 ee 01              	call	0x142923c00 <.text+0x2922c00>
140a3c5a3: 48 8d 45 f0                 	lea	rax, [rbp - 0x10]
140a3c5a7: 48 89 85 30 04 00 00        	mov	qword ptr [rbp + 0x430], rax
140a3c5ae: c7 85 90 02 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x290], 0x2
140a3c5b8: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
140a3c5bd: 48 8d 8d 30 04 00 00        	lea	rcx, [rbp + 0x430]
140a3c5c4: 49 b8 19 ad 7d 07 3c 60 5a 18       	movabs	r8, 0x185a603c077dad19
140a3c5ce: 48 89 f2                    	mov	rdx, rsi
140a3c5d1: 41 b1 19                    	mov	r9b, 0x19
140a3c5d4: e8 b7 2a f3 ff              	call	0x14096f090 <.text+0x96e090>
140a3c5d9: 48 8d b5 90 02 00 00        	lea	rsi, [rbp + 0x290]
140a3c5e0: 48 8d 9d f0 00 00 00        	lea	rbx, [rbp + 0xf0]
140a3c5e7: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a3c5ed: 48 89 f1                    	mov	rcx, rsi
140a3c5f0: 48 89 da                    	mov	rdx, rbx
140a3c5f3: e8 08 76 ee 01              	call	0x142923c00 <.text+0x2922c00>
140a3c5f8: 48 8d 85 9c 00 00 00        	lea	rax, [rbp + 0x9c]
140a3c5ff: 48 89 85 30 04 00 00        	mov	qword ptr [rbp + 0x430], rax
140a3c606: c7 85 f0 00 00 00 02 00 00 00       	mov	dword ptr [rbp + 0xf0], 0x2
140a3c610: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
140a3c615: 48 8d 8d 30 04 00 00        	lea	rcx, [rbp + 0x430]
140a3c61c: 49 b8 e4 75 57 72 1d bb a7 cf       	movabs	r8, -0x305844e28da88a1c
140a3c626: 48 89 f2                    	mov	rdx, rsi
140a3c629: 41 b1 19                    	mov	r9b, 0x19
140a3c62c: e8 0f 1c eb ff              	call	0x1408ee240 <.text+0x8ed240>
140a3c631: 48 8d b5 f0 00 00 00        	lea	rsi, [rbp + 0xf0]
140a3c638: 48 8d 9d 90 02 00 00        	lea	rbx, [rbp + 0x290]
140a3c63f: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a3c645: 48 89 f1                    	mov	rcx, rsi
140a3c648: 48 89 da                    	mov	rdx, rbx
140a3c64b: e8 b0 75 ee 01              	call	0x142923c00 <.text+0x2922c00>
140a3c650: 48 8d 45 78                 	lea	rax, [rbp + 0x78]
140a3c654: 48 89 85 30 04 00 00        	mov	qword ptr [rbp + 0x430], rax
140a3c65b: c7 85 90 02 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x290], 0x2
140a3c665: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
140a3c66a: 48 8d 8d 30 04 00 00        	lea	rcx, [rbp + 0x430]
140a3c671: 49 b8 c8 32 66 75 a2 6b 12 d8       	movabs	r8, -0x27ed945d8a99cd38
140a3c67b: 48 89 f2                    	mov	rdx, rsi
140a3c67e: 41 b1 19                    	mov	r9b, 0x19
140a3c681: e8 ba 1b eb ff              	call	0x1408ee240 <.text+0x8ed240>
140a3c686: 48 8d b5 90 02 00 00        	lea	rsi, [rbp + 0x290]
140a3c68d: 48 8d 9d f0 00 00 00        	lea	rbx, [rbp + 0xf0]
140a3c694: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a3c69a: 48 89 f1                    	mov	rcx, rsi
140a3c69d: 48 89 da                    	mov	rdx, rbx
140a3c6a0: e8 5b 75 ee 01              	call	0x142923c00 <.text+0x2922c00>
140a3c6a5: 48 8b 85 20 05 00 00        	mov	rax, qword ptr [rbp + 0x520]
140a3c6ac: 48 89 85 30 04 00 00        	mov	qword ptr [rbp + 0x430], rax
140a3c6b3: c7 85 f0 00 00 00 02 00 00 00       	mov	dword ptr [rbp + 0xf0], 0x2
140a3c6bd: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
140a3c6c2: 48 8d 8d 30 04 00 00        	lea	rcx, [rbp + 0x430]
140a3c6c9: 49 b8 e3 6b 5a 83 cd 4c a7 b8       	movabs	r8, -0x4758b3327ca5941d
140a3c6d3: 48 89 f2                    	mov	rdx, rsi
140a3c6d6: 41 b1 19                    	mov	r9b, 0x19
140a3c6d9: e8 62 1b eb ff              	call	0x1408ee240 <.text+0x8ed240>
140a3c6de: 48 8d b5 f0 00 00 00        	lea	rsi, [rbp + 0xf0]
140a3c6e5: 48 8d 9d 90 02 00 00        	lea	rbx, [rbp + 0x290]
140a3c6ec: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a3c6f2: 48 89 f1                    	mov	rcx, rsi
140a3c6f5: 48 89 da                    	mov	rdx, rbx
140a3c6f8: e8 03 75 ee 01              	call	0x142923c00 <.text+0x2922c00>
140a3c6fd: 48 8d 45 c0                 	lea	rax, [rbp - 0x40]
140a3c701: 48 89 85 30 04 00 00        	mov	qword ptr [rbp + 0x430], rax
140a3c708: c7 85 90 02 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x290], 0x2
140a3c712: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
140a3c717: 48 8d 8d 30 04 00 00        	lea	rcx, [rbp + 0x430]
140a3c71e: 49 b8 57 6d 2b 7e ed 0c 07 7d       	movabs	r8, 0x7d070ced7e2b6d57
140a3c728: 48 89 f2                    	mov	rdx, rsi
140a3c72b: 41 b1 19                    	mov	r9b, 0x19
140a3c72e: e8 5d 29 f3 ff              	call	0x14096f090 <.text+0x96e090>
140a3c733: 4c 8b b5 28 05 00 00        	mov	r14, qword ptr [rbp + 0x528]
140a3c73a: 48 8d b5 90 02 00 00        	lea	rsi, [rbp + 0x290]
140a3c741: 48 8d 9d f0 00 00 00        	lea	rbx, [rbp + 0xf0]
140a3c748: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a3c74e: 48 89 f1                    	mov	rcx, rsi
140a3c751: 48 89 da                    	mov	rdx, rbx
140a3c754: e8 a7 74 ee 01              	call	0x142923c00 <.text+0x2922c00>
140a3c759: 4c 89 b5 30 04 00 00        	mov	qword ptr [rbp + 0x430], r14
140a3c760: c7 85 f0 00 00 00 02 00 00 00       	mov	dword ptr [rbp + 0xf0], 0x2
140a3c76a: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
140a3c76f: 48 8d 8d 30 04 00 00        	lea	rcx, [rbp + 0x430]
140a3c776: 49 b8 13 19 d6 6a 5a 01 75 aa       	movabs	r8, -0x558afea59529e6ed
140a3c780: 48 89 f2                    	mov	rdx, rsi
140a3c783: 41 b1 32                    	mov	r9b, 0x32
140a3c786: e8 75 15 eb ff              	call	0x1408edd00 <.text+0x8ecd00>
140a3c78b: 48 8d b5 f0 00 00 00        	lea	rsi, [rbp + 0xf0]
140a3c792: 48 8d 9d 90 02 00 00        	lea	rbx, [rbp + 0x290]
140a3c799: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a3c79f: 48 89 f1                    	mov	rcx, rsi
140a3c7a2: 48 89 da                    	mov	rdx, rbx
140a3c7a5: e8 56 74 ee 01              	call	0x142923c00 <.text+0x2922c00>
140a3c7aa: 4c 89 bd 30 04 00 00        	mov	qword ptr [rbp + 0x430], r15
140a3c7b1: c7 85 90 02 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x290], 0x2
140a3c7bb: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
140a3c7c0: 48 8d 8d 30 04 00 00        	lea	rcx, [rbp + 0x430]
140a3c7c7: 49 b8 b2 92 98 e6 3a 9a 21 88       	movabs	r8, -0x77de65c519676d4e
140a3c7d1: 48 89 f2                    	mov	rdx, rsi
140a3c7d4: 41 b1 32                    	mov	r9b, 0x32
140a3c7d7: e8 24 15 eb ff              	call	0x1408edd00 <.text+0x8ecd00>
140a3c7dc: 4c 8b b5 e8 00 00 00        	mov	r14, qword ptr [rbp + 0xe8]
140a3c7e3: 48 8d b5 90 02 00 00        	lea	rsi, [rbp + 0x290]
140a3c7ea: 48 8d 9d f0 00 00 00        	lea	rbx, [rbp + 0xf0]
140a3c7f1: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a3c7f7: 48 89 f1                    	mov	rcx, rsi
140a3c7fa: 48 89 da                    	mov	rdx, rbx
140a3c7fd: e8 fe 73 ee 01              	call	0x142923c00 <.text+0x2922c00>
140a3c802: 48 8d 45 20                 	lea	rax, [rbp + 0x20]
140a3c806: 48 89 85 30 04 00 00        	mov	qword ptr [rbp + 0x430], rax
140a3c80d: c7 85 f0 00 00 00 02 00 00 00       	mov	dword ptr [rbp + 0xf0], 0x2
140a3c817: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
140a3c81c: 48 8d 8d 30 04 00 00        	lea	rcx, [rbp + 0x430]
140a3c823: 49 b8 b5 7a 86 3b 29 c5 80 b9       	movabs	r8, -0x467f3ad6c479854b
140a3c82d: 48 89 f2                    	mov	rdx, rsi
140a3c830: 41 b1 32                    	mov	r9b, 0x32
140a3c833: e8 c8 14 eb ff              	call	0x1408edd00 <.text+0x8ecd00>
140a3c838: 48 8d 8d f0 00 00 00        	lea	rcx, [rbp + 0xf0]
140a3c83f: 48 8d 95 90 02 00 00        	lea	rdx, [rbp + 0x290]
140a3c846: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a3c84c: e8 af 73 ee 01              	call	0x142923c00 <.text+0x2922c00>
140a3c851: 80 bd 26 02 00 00 08        	cmp	byte ptr [rbp + 0x226], 0x8
140a3c858: 74 29                       	je	0x140a3c883 <.text+0xa3b883>
140a3c85a: 48 8d 8d 00 02 00 00        	lea	rcx, [rbp + 0x200]
140a3c861: ba e8 00 00 00              	mov	edx, 0xe8
140a3c866: 48 03 95 28 02 00 00        	add	rdx, qword ptr [rbp + 0x228]
140a3c86d: 49 b8 92 5f a5 f4 f2 81 cf c4       	movabs	r8, -0x3b307e0d0b5aa06e
140a3c877: 44 8b 8d 2c 04 00 00        	mov	r9d, dword ptr [rbp + 0x42c]
140a3c87e: e8 ed ec f2 ff              	call	0x14096b570 <.text+0x96a570>
140a3c883: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
140a3c88a: 48 8d 95 f0 00 00 00        	lea	rdx, [rbp + 0xf0]
140a3c891: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a3c897: e8 64 73 ee 01              	call	0x142923c00 <.text+0x2922c00>
140a3c89c: 80 bd c6 03 00 00 08        	cmp	byte ptr [rbp + 0x3c6], 0x8
140a3c8a3: 74 25                       	je	0x140a3c8ca <.text+0xa3b8ca>
140a3c8a5: 48 8d 8d a0 03 00 00        	lea	rcx, [rbp + 0x3a0]
140a3c8ac: ba e8 00 00 00              	mov	edx, 0xe8
140a3c8b1: 48 03 95 c8 03 00 00        	add	rdx, qword ptr [rbp + 0x3c8]
140a3c8b8: 49 b8 49 1d 52 67 0b d5 02 a9       	movabs	r8, -0x56fd2af498ade2b7
140a3c8c2: 0f 28 de                    	movaps	xmm3, xmm6
140a3c8c5: e8 66 f1 f2 ff              	call	0x14096ba30 <.text+0x96aa30>
140a3c8ca: 48 8d 8d f0 00 00 00        	lea	rcx, [rbp + 0xf0]
140a3c8d1: 48 8d 95 90 02 00 00        	lea	rdx, [rbp + 0x290]
140a3c8d8: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a3c8de: e8 1d 73 ee 01              	call	0x142923c00 <.text+0x2922c00>
140a3c8e3: 0f 10 85 90 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x190]
140a3c8ea: 0f 29 85 70 04 00 00        	movaps	xmmword ptr [rbp + 0x470], xmm0
140a3c8f1: 0f 10 85 80 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x180]
140a3c8f8: 0f 29 85 60 04 00 00        	movaps	xmmword ptr [rbp + 0x460], xmm0
140a3c8ff: 0f 10 85 70 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x170]
140a3c906: 0f 29 85 50 04 00 00        	movaps	xmmword ptr [rbp + 0x450], xmm0
140a3c90d: 0f 10 85 60 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x160]
140a3c914: 0f 29 85 40 04 00 00        	movaps	xmmword ptr [rbp + 0x440], xmm0
140a3c91b: 0f 10 85 50 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x150]
140a3c922: 0f 29 85 30 04 00 00        	movaps	xmmword ptr [rbp + 0x430], xmm0
140a3c929: 48 8b 85 28 02 00 00        	mov	rax, qword ptr [rbp + 0x228]
140a3c930: 48 8b 8d 70 02 00 00        	mov	rcx, qword ptr [rbp + 0x270]
140a3c937: 48 8b 95 00 01 00 00        	mov	rdx, qword ptr [rbp + 0x100]
140a3c93e: 48 89 95 a0 02 00 00        	mov	qword ptr [rbp + 0x2a0], rdx
140a3c945: 0f 10 85 f0 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xf0]
140a3c94c: 0f 29 85 90 02 00 00        	movaps	xmmword ptr [rbp + 0x290], xmm0
140a3c953: 0f 10 85 08 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x108]
140a3c95a: 0f 29 85 c0 02 00 00        	movaps	xmmword ptr [rbp + 0x2c0], xmm0
140a3c961: 48 8b 95 18 01 00 00        	mov	rdx, qword ptr [rbp + 0x118]
140a3c968: 48 89 95 d0 02 00 00        	mov	qword ptr [rbp + 0x2d0], rdx
140a3c96f: 0f 10 85 20 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x120]
140a3c976: 0f 11 85 a8 02 00 00        	movups	xmmword ptr [rbp + 0x2a8], xmm0
140a3c97d: 48 8b 95 30 01 00 00        	mov	rdx, qword ptr [rbp + 0x130]
140a3c984: 48 89 95 b8 02 00 00        	mov	qword ptr [rbp + 0x2b8], rdx
140a3c98b: 0f 10 85 a0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1a0]
140a3c992: 0f 10 8d b0 01 00 00        	movups	xmm1, xmmword ptr [rbp + 0x1b0]
140a3c999: 0f 10 95 c0 01 00 00        	movups	xmm2, xmmword ptr [rbp + 0x1c0]
140a3c9a0: 0f 10 9d d0 01 00 00        	movups	xmm3, xmmword ptr [rbp + 0x1d0]
140a3c9a7: 0f 11 85 08 03 00 00        	movups	xmmword ptr [rbp + 0x308], xmm0
140a3c9ae: 0f 11 8d 18 03 00 00        	movups	xmmword ptr [rbp + 0x318], xmm1
140a3c9b5: 0f 11 95 28 03 00 00        	movups	xmmword ptr [rbp + 0x328], xmm2
140a3c9bc: 0f 11 9d 38 03 00 00        	movups	xmmword ptr [rbp + 0x338], xmm3
140a3c9c3: 0f 10 85 e0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1e0]
140a3c9ca: 0f 11 85 48 03 00 00        	movups	xmmword ptr [rbp + 0x348], xmm0
140a3c9d1: 0f 10 85 f0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1f0]
140a3c9d8: 0f 11 85 58 03 00 00        	movups	xmmword ptr [rbp + 0x358], xmm0
140a3c9df: 48 8b 95 48 01 00 00        	mov	rdx, qword ptr [rbp + 0x148]
140a3c9e6: 48 89 95 e8 02 00 00        	mov	qword ptr [rbp + 0x2e8], rdx
140a3c9ed: 0f 10 85 38 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x138]
140a3c9f4: 0f 11 85 d8 02 00 00        	movups	xmmword ptr [rbp + 0x2d8], xmm0
140a3c9fb: 48 89 8d 70 03 00 00        	mov	qword ptr [rbp + 0x370], rcx
140a3ca02: 48 89 85 68 03 00 00        	mov	qword ptr [rbp + 0x368], rax
140a3ca09: c7 85 f0 02 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x2f0], 0x1
140a3ca13: 89 bd f4 02 00 00           	mov	dword ptr [rbp + 0x2f4], edi
140a3ca19: 44 89 ad f8 02 00 00        	mov	dword ptr [rbp + 0x2f8], r13d
140a3ca20: c7 85 fc 02 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x2fc], 0x1
140a3ca2a: 48 8d 8d 30 04 00 00        	lea	rcx, [rbp + 0x430]
140a3ca31: 48 8d 95 90 02 00 00        	lea	rdx, [rbp + 0x290]
140a3ca38: e8 93 0a fa ff              	call	0x1409dd4d0 <.text+0x9dc4d0>
140a3ca3d: 80 bd 26 02 00 00 08        	cmp	byte ptr [rbp + 0x226], 0x8
140a3ca44: 74 0c                       	je	0x140a3ca52 <.text+0xa3ba52>
140a3ca46: 48 8d 8d 10 02 00 00        	lea	rcx, [rbp + 0x210]
140a3ca4d: e8 5e 5a 1a 00              	call	0x140be24b0 <.text+0xbe14b0>
140a3ca52: 41 8b 47 20                 	mov	eax, dword ptr [r15 + 0x20]
140a3ca56: 41 89 46 20                 	mov	dword ptr [r14 + 0x20], eax
140a3ca5a: 41 0f 10 07                 	movups	xmm0, xmmword ptr [r15]
140a3ca5e: 41 0f 10 4f 10              	movups	xmm1, xmmword ptr [r15 + 0x10]
140a3ca63: 41 0f 11 4e 10              	movups	xmmword ptr [r14 + 0x10], xmm1
140a3ca68: 41 0f 11 06                 	movups	xmmword ptr [r14], xmm0
140a3ca6c: 4c 89 f0                    	mov	rax, r14
140a3ca6f: 0f 28 b5 90 04 00 00        	movaps	xmm6, xmmword ptr [rbp + 0x490]
140a3ca76: 48 81 c4 28 05 00 00        	add	rsp, 0x528
140a3ca7d: 5b                          	pop	rbx
140a3ca7e: 5f                          	pop	rdi
140a3ca7f: 5e                          	pop	rsi
140a3ca80: 41 5c                       	pop	r12
140a3ca82: 41 5d                       	pop	r13
140a3ca84: 41 5e                       	pop	r14
140a3ca86: 41 5f                       	pop	r15
140a3ca88: 5d                          	pop	rbp
140a3ca89: c3                          	ret
140a3ca8a: 48 8d 0d 0f 0a 09 02        	lea	rcx, [rip + 0x2090a0f]  # 0x142acd4a0
140a3ca91: e8 7a c8 e9 01              	call	0x1428d9310 <.text+0x28d8310>
140a3ca96: 48 8b 85 98 02 00 00        	mov	rax, qword ptr [rbp + 0x298]
140a3ca9d: 48 89 85 f0 00 00 00        	mov	qword ptr [rbp + 0xf0], rax
140a3caa4: 48 8d 05 15 0a 09 02        	lea	rax, [rip + 0x2090a15]  # 0x142acd4c0
140a3caab: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
140a3cab0: 48 8d 0d 51 fc 08 02        	lea	rcx, [rip + 0x208fc51]  # 0x142acc708
140a3cab7: 4c 8d 0d 2a fc 08 02        	lea	r9, [rip + 0x208fc2a]   # 0x142acc6e8
140a3cabe: 4c 8d 85 f0 00 00 00        	lea	r8, [rbp + 0xf0]
140a3cac5: ba 2b 00 00 00              	mov	edx, 0x2b
140a3caca: e8 91 cc e9 01              	call	0x1428d9760 <.text+0x28d8760>
140a3cacf: 0f 0b                       	ud2
140a3cad1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
140a3cae0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a3cae5: 55                          	push	rbp
140a3cae6: 41 57                       	push	r15
140a3cae8: 41 56                       	push	r14
140a3caea: 41 55                       	push	r13
140a3caec: 41 54                       	push	r12
140a3caee: 56                          	push	rsi
140a3caef: 57                          	push	rdi
140a3caf0: 53                          	push	rbx
140a3caf1: 48 83 ec 48                 	sub	rsp, 0x48
140a3caf5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a3cafc: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a3cb01: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
140a3cb08: e8 a3 a4 e5 ff              	call	0x140896fb0 <.text+0x895fb0>
140a3cb0d: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a3cb12: 48 83 c4 48                 	add	rsp, 0x48
140a3cb16: 5b                          	pop	rbx
140a3cb17: 5f                          	pop	rdi
140a3cb18: 5e                          	pop	rsi
140a3cb19: 41 5c                       	pop	r12
140a3cb1b: 41 5d                       	pop	r13
140a3cb1d: 41 5e                       	pop	r14
140a3cb1f: 41 5f                       	pop	r15
140a3cb21: 5d                          	pop	rbp
140a3cb22: c3                          	ret
140a3cb23: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a3cb30: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a3cb35: 55                          	push	rbp
140a3cb36: 41 57                       	push	r15
140a3cb38: 41 56                       	push	r14
140a3cb3a: 41 55                       	push	r13
140a3cb3c: 41 54                       	push	r12
140a3cb3e: 56                          	push	rsi
140a3cb3f: 57                          	push	rdi
140a3cb40: 53                          	push	rbx
140a3cb41: 48 83 ec 48                 	sub	rsp, 0x48
140a3cb45: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a3cb4c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a3cb51: 80 bd c6 03 00 00 08        	cmp	byte ptr [rbp + 0x3c6], 0x8
140a3cb58: 74 0c                       	je	0x140a3cb66 <.text+0xa3bb66>
140a3cb5a: 48 8d 8d b0 03 00 00        	lea	rcx, [rbp + 0x3b0]
140a3cb61: e8 4a 59 1a 00              	call	0x140be24b0 <.text+0xbe14b0>
140a3cb66: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a3cb6b: 48 83 c4 48                 	add	rsp, 0x48
140a3cb6f: 5b                          	pop	rbx
140a3cb70: 5f                          	pop	rdi
140a3cb71: 5e                          	pop	rsi
140a3cb72: 41 5c                       	pop	r12
140a3cb74: 41 5d                       	pop	r13
140a3cb76: 41 5e                       	pop	r14
140a3cb78: 41 5f                       	pop	r15
140a3cb7a: 5d                          	pop	rbp
140a3cb7b: c3                          	ret
140a3cb7c: 0f 1f 40 00                 	nop	dword ptr [rax]
140a3cb80: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a3cb85: 55                          	push	rbp
140a3cb86: 41 57                       	push	r15
140a3cb88: 41 56                       	push	r14
140a3cb8a: 41 55                       	push	r13
140a3cb8c: 41 54                       	push	r12
140a3cb8e: 56                          	push	rsi
140a3cb8f: 57                          	push	rdi
140a3cb90: 53                          	push	rbx
140a3cb91: 48 83 ec 48                 	sub	rsp, 0x48
140a3cb95: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a3cb9c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a3cba1: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
140a3cba8: e8 03 a4 e5 ff              	call	0x140896fb0 <.text+0x895fb0>
140a3cbad: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a3cbb2: 48 83 c4 48                 	add	rsp, 0x48
140a3cbb6: 5b                          	pop	rbx
140a3cbb7: 5f                          	pop	rdi
140a3cbb8: 5e                          	pop	rsi
140a3cbb9: 41 5c                       	pop	r12
140a3cbbb: 41 5d                       	pop	r13
140a3cbbd: 41 5e                       	pop	r14
140a3cbbf: 41 5f                       	pop	r15
140a3cbc1: 5d                          	pop	rbp
140a3cbc2: c3                          	ret
140a3cbc3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a3cbd0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a3cbd5: 55                          	push	rbp
140a3cbd6: 41 57                       	push	r15
140a3cbd8: 41 56                       	push	r14
140a3cbda: 41 55                       	push	r13
140a3cbdc: 41 54                       	push	r12
140a3cbde: 56                          	push	rsi
140a3cbdf: 57                          	push	rdi
140a3cbe0: 53                          	push	rbx
140a3cbe1: 48 83 ec 48                 	sub	rsp, 0x48
140a3cbe5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a3cbec: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a3cbf1: 48 8d 8d f0 00 00 00        	lea	rcx, [rbp + 0xf0]
140a3cbf8: e8 b3 a3 e5 ff              	call	0x140896fb0 <.text+0x895fb0>
140a3cbfd: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a3cc02: 48 83 c4 48                 	add	rsp, 0x48
140a3cc06: 5b                          	pop	rbx
140a3cc07: 5f                          	pop	rdi
140a3cc08: 5e                          	pop	rsi
140a3cc09: 41 5c                       	pop	r12
140a3cc0b: 41 5d                       	pop	r13
140a3cc0d: 41 5e                       	pop	r14
140a3cc0f: 41 5f                       	pop	r15
140a3cc11: 5d                          	pop	rbp
140a3cc12: c3                          	ret
140a3cc13: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a3cc20: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a3cc25: 55                          	push	rbp
140a3cc26: 41 57                       	push	r15
140a3cc28: 41 56                       	push	r14
140a3cc2a: 41 55                       	push	r13
140a3cc2c: 41 54                       	push	r12
140a3cc2e: 56                          	push	rsi
140a3cc2f: 57                          	push	rdi
140a3cc30: 53                          	push	rbx
140a3cc31: 48 83 ec 48                 	sub	rsp, 0x48
140a3cc35: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a3cc3c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a3cc41: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
140a3cc48: e8 63 a3 e5 ff              	call	0x140896fb0 <.text+0x895fb0>
140a3cc4d: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a3cc52: 48 83 c4 48                 	add	rsp, 0x48
140a3cc56: 5b                          	pop	rbx
140a3cc57: 5f                          	pop	rdi
140a3cc58: 5e                          	pop	rsi
140a3cc59: 41 5c                       	pop	r12
140a3cc5b: 41 5d                       	pop	r13
140a3cc5d: 41 5e                       	pop	r14
140a3cc5f: 41 5f                       	pop	r15
140a3cc61: 5d                          	pop	rbp
140a3cc62: c3                          	ret
140a3cc63: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a3cc70: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a3cc75: 55                          	push	rbp
140a3cc76: 41 57                       	push	r15
140a3cc78: 41 56                       	push	r14
140a3cc7a: 41 55                       	push	r13
140a3cc7c: 41 54                       	push	r12
140a3cc7e: 56                          	push	rsi
140a3cc7f: 57                          	push	rdi
140a3cc80: 53                          	push	rbx
140a3cc81: 48 83 ec 48                 	sub	rsp, 0x48
140a3cc85: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a3cc8c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a3cc91: 48 8d 8d f0 00 00 00        	lea	rcx, [rbp + 0xf0]
140a3cc98: e8 13 a3 e5 ff              	call	0x140896fb0 <.text+0x895fb0>
140a3cc9d: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a3cca2: 48 83 c4 48                 	add	rsp, 0x48
140a3cca6: 5b                          	pop	rbx
140a3cca7: 5f                          	pop	rdi
140a3cca8: 5e                          	pop	rsi
140a3cca9: 41 5c                       	pop	r12
140a3ccab: 41 5d                       	pop	r13
140a3ccad: 41 5e                       	pop	r14
140a3ccaf: 41 5f                       	pop	r15
140a3ccb1: 5d                          	pop	rbp
140a3ccb2: c3                          	ret
140a3ccb3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a3ccc0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a3ccc5: 55                          	push	rbp
140a3ccc6: 41 57                       	push	r15
140a3ccc8: 41 56                       	push	r14
140a3ccca: 41 55                       	push	r13
140a3cccc: 41 54                       	push	r12
140a3ccce: 56                          	push	rsi
140a3cccf: 57                          	push	rdi
140a3ccd0: 53                          	push	rbx
140a3ccd1: 48 83 ec 48                 	sub	rsp, 0x48
140a3ccd5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a3ccdc: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a3cce1: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
140a3cce8: e8 c3 a2 e5 ff              	call	0x140896fb0 <.text+0x895fb0>
140a3cced: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a3ccf2: 48 83 c4 48                 	add	rsp, 0x48
140a3ccf6: 5b                          	pop	rbx
140a3ccf7: 5f                          	pop	rdi
140a3ccf8: 5e                          	pop	rsi
140a3ccf9: 41 5c                       	pop	r12
140a3ccfb: 41 5d                       	pop	r13
140a3ccfd: 41 5e                       	pop	r14
140a3ccff: 41 5f                       	pop	r15
140a3cd01: 5d                          	pop	rbp
140a3cd02: c3                          	ret
140a3cd03: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a3cd10: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a3cd15: 55                          	push	rbp
140a3cd16: 41 57                       	push	r15
140a3cd18: 41 56                       	push	r14
140a3cd1a: 41 55                       	push	r13
140a3cd1c: 41 54                       	push	r12
140a3cd1e: 56                          	push	rsi
140a3cd1f: 57                          	push	rdi
140a3cd20: 53                          	push	rbx
140a3cd21: 48 83 ec 48                 	sub	rsp, 0x48
140a3cd25: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a3cd2c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a3cd31: 48 8d 8d f0 00 00 00        	lea	rcx, [rbp + 0xf0]
140a3cd38: e8 73 a2 e5 ff              	call	0x140896fb0 <.text+0x895fb0>
140a3cd3d: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a3cd42: 48 83 c4 48                 	add	rsp, 0x48
140a3cd46: 5b                          	pop	rbx
140a3cd47: 5f                          	pop	rdi
140a3cd48: 5e                          	pop	rsi
140a3cd49: 41 5c                       	pop	r12
140a3cd4b: 41 5d                       	pop	r13
140a3cd4d: 41 5e                       	pop	r14
140a3cd4f: 41 5f                       	pop	r15
140a3cd51: 5d                          	pop	rbp
140a3cd52: c3                          	ret
140a3cd53: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a3cd60: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a3cd65: 55                          	push	rbp
140a3cd66: 41 57                       	push	r15
140a3cd68: 41 56                       	push	r14
140a3cd6a: 41 55                       	push	r13
140a3cd6c: 41 54                       	push	r12
140a3cd6e: 56                          	push	rsi
140a3cd6f: 57                          	push	rdi
140a3cd70: 53                          	push	rbx
140a3cd71: 48 83 ec 48                 	sub	rsp, 0x48
140a3cd75: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a3cd7c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a3cd81: 80 bd c6 03 00 00 08        	cmp	byte ptr [rbp + 0x3c6], 0x8
140a3cd88: 74 0c                       	je	0x140a3cd96 <.text+0xa3bd96>
140a3cd8a: 48 8d 8d b0 03 00 00        	lea	rcx, [rbp + 0x3b0]
140a3cd91: e8 1a 57 1a 00              	call	0x140be24b0 <.text+0xbe14b0>
140a3cd96: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a3cd9b: 48 83 c4 48                 	add	rsp, 0x48
140a3cd9f: 5b                          	pop	rbx
140a3cda0: 5f                          	pop	rdi
140a3cda1: 5e                          	pop	rsi
140a3cda2: 41 5c                       	pop	r12
140a3cda4: 41 5d                       	pop	r13
140a3cda6: 41 5e                       	pop	r14
140a3cda8: 41 5f                       	pop	r15
140a3cdaa: 5d                          	pop	rbp
140a3cdab: c3                          	ret
140a3cdac: 0f 1f 40 00                 	nop	dword ptr [rax]
140a3cdb0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a3cdb5: 55                          	push	rbp
140a3cdb6: 41 57                       	push	r15
140a3cdb8: 41 56                       	push	r14
140a3cdba: 41 55                       	push	r13
140a3cdbc: 41 54                       	push	r12
140a3cdbe: 56                          	push	rsi
140a3cdbf: 57                          	push	rdi
140a3cdc0: 53                          	push	rbx
140a3cdc1: 48 83 ec 48                 	sub	rsp, 0x48
140a3cdc5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a3cdcc: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a3cdd1: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
140a3cdd8: e8 d3 a1 e5 ff              	call	0x140896fb0 <.text+0x895fb0>
140a3cddd: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a3cde2: 48 83 c4 48                 	add	rsp, 0x48
140a3cde6: 5b                          	pop	rbx
140a3cde7: 5f                          	pop	rdi
140a3cde8: 5e                          	pop	rsi
140a3cde9: 41 5c                       	pop	r12
140a3cdeb: 41 5d                       	pop	r13
140a3cded: 41 5e                       	pop	r14
140a3cdef: 41 5f                       	pop	r15
140a3cdf1: 5d                          	pop	rbp
140a3cdf2: c3                          	ret
140a3cdf3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a3ce00: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a3ce05: 55                          	push	rbp
140a3ce06: 41 57                       	push	r15
140a3ce08: 41 56                       	push	r14
140a3ce0a: 41 55                       	push	r13
140a3ce0c: 41 54                       	push	r12
140a3ce0e: 56                          	push	rsi
140a3ce0f: 57                          	push	rdi
140a3ce10: 53                          	push	rbx
140a3ce11: 48 83 ec 48                 	sub	rsp, 0x48
140a3ce15: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a3ce1c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a3ce21: 48 8d 8d f0 00 00 00        	lea	rcx, [rbp + 0xf0]
140a3ce28: e8 83 a1 e5 ff              	call	0x140896fb0 <.text+0x895fb0>
140a3ce2d: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a3ce32: 48 83 c4 48                 	add	rsp, 0x48
140a3ce36: 5b                          	pop	rbx
140a3ce37: 5f                          	pop	rdi
140a3ce38: 5e                          	pop	rsi
140a3ce39: 41 5c                       	pop	r12
140a3ce3b: 41 5d                       	pop	r13
140a3ce3d: 41 5e                       	pop	r14
140a3ce3f: 41 5f                       	pop	r15
140a3ce41: 5d                          	pop	rbp
140a3ce42: c3                          	ret
140a3ce43: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a3ce50: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a3ce55: 55                          	push	rbp
140a3ce56: 41 57                       	push	r15
140a3ce58: 41 56                       	push	r14
140a3ce5a: 41 55                       	push	r13
140a3ce5c: 41 54                       	push	r12
140a3ce5e: 56                          	push	rsi
140a3ce5f: 57                          	push	rdi
140a3ce60: 53                          	push	rbx
140a3ce61: 48 83 ec 48                 	sub	rsp, 0x48
140a3ce65: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a3ce6c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a3ce71: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
140a3ce78: e8 33 a1 e5 ff              	call	0x140896fb0 <.text+0x895fb0>
140a3ce7d: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a3ce82: 48 83 c4 48                 	add	rsp, 0x48
140a3ce86: 5b                          	pop	rbx
140a3ce87: 5f                          	pop	rdi
140a3ce88: 5e                          	pop	rsi
140a3ce89: 41 5c                       	pop	r12
140a3ce8b: 41 5d                       	pop	r13
140a3ce8d: 41 5e                       	pop	r14
140a3ce8f: 41 5f                       	pop	r15
140a3ce91: 5d                          	pop	rbp
140a3ce92: c3                          	ret
140a3ce93: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a3cea0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a3cea5: 55                          	push	rbp
140a3cea6: 41 57                       	push	r15
140a3cea8: 41 56                       	push	r14
140a3ceaa: 41 55                       	push	r13
140a3ceac: 41 54                       	push	r12
140a3ceae: 56                          	push	rsi
140a3ceaf: 57                          	push	rdi
140a3ceb0: 53                          	push	rbx
140a3ceb1: 48 83 ec 48                 	sub	rsp, 0x48
140a3ceb5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a3cebc: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a3cec1: 48 8d 8d f0 00 00 00        	lea	rcx, [rbp + 0xf0]
140a3cec8: e8 e3 a0 e5 ff              	call	0x140896fb0 <.text+0x895fb0>
140a3cecd: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a3ced2: 48 83 c4 48                 	add	rsp, 0x48
140a3ced6: 5b                          	pop	rbx
140a3ced7: 5f                          	pop	rdi
140a3ced8: 5e                          	pop	rsi
140a3ced9: 41 5c                       	pop	r12
140a3cedb: 41 5d                       	pop	r13
140a3cedd: 41 5e                       	pop	r14
140a3cedf: 41 5f                       	pop	r15
140a3cee1: 5d                          	pop	rbp
140a3cee2: c3                          	ret
140a3cee3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a3cef0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a3cef5: 55                          	push	rbp
140a3cef6: 41 57                       	push	r15
140a3cef8: 41 56                       	push	r14
140a3cefa: 41 55                       	push	r13
140a3cefc: 41 54                       	push	r12
140a3cefe: 56                          	push	rsi
140a3ceff: 57                          	push	rdi
140a3cf00: 53                          	push	rbx
140a3cf01: 48 83 ec 48                 	sub	rsp, 0x48
140a3cf05: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a3cf0c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a3cf11: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
140a3cf18: e8 93 a0 e5 ff              	call	0x140896fb0 <.text+0x895fb0>
140a3cf1d: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a3cf22: 48 83 c4 48                 	add	rsp, 0x48
140a3cf26: 5b                          	pop	rbx
140a3cf27: 5f                          	pop	rdi
140a3cf28: 5e                          	pop	rsi
140a3cf29: 41 5c                       	pop	r12
140a3cf2b: 41 5d                       	pop	r13
140a3cf2d: 41 5e                       	pop	r14
140a3cf2f: 41 5f                       	pop	r15
140a3cf31: 5d                          	pop	rbp
140a3cf32: c3                          	ret
140a3cf33: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a3cf40: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a3cf45: 55                          	push	rbp
140a3cf46: 41 57                       	push	r15
140a3cf48: 41 56                       	push	r14
140a3cf4a: 41 55                       	push	r13
140a3cf4c: 41 54                       	push	r12
140a3cf4e: 56                          	push	rsi
140a3cf4f: 57                          	push	rdi
140a3cf50: 53                          	push	rbx
140a3cf51: 48 83 ec 48                 	sub	rsp, 0x48
140a3cf55: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a3cf5c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a3cf61: 48 8d 8d f0 00 00 00        	lea	rcx, [rbp + 0xf0]
140a3cf68: e8 43 a0 e5 ff              	call	0x140896fb0 <.text+0x895fb0>
140a3cf6d: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a3cf72: 48 83 c4 48                 	add	rsp, 0x48
140a3cf76: 5b                          	pop	rbx
140a3cf77: 5f                          	pop	rdi
140a3cf78: 5e                          	pop	rsi
140a3cf79: 41 5c                       	pop	r12
140a3cf7b: 41 5d                       	pop	r13
140a3cf7d: 41 5e                       	pop	r14
140a3cf7f: 41 5f                       	pop	r15
140a3cf81: 5d                          	pop	rbp
140a3cf82: c3                          	ret
140a3cf83: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a3cf90: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a3cf95: 55                          	push	rbp
140a3cf96: 41 57                       	push	r15
140a3cf98: 41 56                       	push	r14
140a3cf9a: 41 55                       	push	r13
140a3cf9c: 41 54                       	push	r12
140a3cf9e: 56                          	push	rsi
140a3cf9f: 57                          	push	rdi
140a3cfa0: 53                          	push	rbx
140a3cfa1: 48 83 ec 48                 	sub	rsp, 0x48
140a3cfa5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a3cfac: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a3cfb1: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
140a3cfb8: e8 f3 9f e5 ff              	call	0x140896fb0 <.text+0x895fb0>
140a3cfbd: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a3cfc2: 48 83 c4 48                 	add	rsp, 0x48
140a3cfc6: 5b                          	pop	rbx
140a3cfc7: 5f                          	pop	rdi
140a3cfc8: 5e                          	pop	rsi
140a3cfc9: 41 5c                       	pop	r12
140a3cfcb: 41 5d                       	pop	r13
140a3cfcd: 41 5e                       	pop	r14
140a3cfcf: 41 5f                       	pop	r15
140a3cfd1: 5d                          	pop	rbp
140a3cfd2: c3                          	ret
140a3cfd3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a3cfe0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a3cfe5: 55                          	push	rbp
140a3cfe6: 41 57                       	push	r15
140a3cfe8: 41 56                       	push	r14
140a3cfea: 41 55                       	push	r13
140a3cfec: 41 54                       	push	r12
140a3cfee: 56                          	push	rsi
140a3cfef: 57                          	push	rdi
140a3cff0: 53                          	push	rbx
140a3cff1: 48 83 ec 48                 	sub	rsp, 0x48
140a3cff5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a3cffc: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a3d001: 48 8d 8d f0 00 00 00        	lea	rcx, [rbp + 0xf0]
140a3d008: e8 a3 9f e5 ff              	call	0x140896fb0 <.text+0x895fb0>
140a3d00d: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a3d012: 48 83 c4 48                 	add	rsp, 0x48
140a3d016: 5b                          	pop	rbx
140a3d017: 5f                          	pop	rdi
140a3d018: 5e                          	pop	rsi
140a3d019: 41 5c                       	pop	r12
140a3d01b: 41 5d                       	pop	r13
140a3d01d: 41 5e                       	pop	r14
140a3d01f: 41 5f                       	pop	r15
140a3d021: 5d                          	pop	rbp
140a3d022: c3                          	ret
140a3d023: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a3d030: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a3d035: 55                          	push	rbp
140a3d036: 41 57                       	push	r15
140a3d038: 41 56                       	push	r14
140a3d03a: 41 55                       	push	r13
140a3d03c: 41 54                       	push	r12
140a3d03e: 56                          	push	rsi
140a3d03f: 57                          	push	rdi
140a3d040: 53                          	push	rbx
140a3d041: 48 83 ec 48                 	sub	rsp, 0x48
140a3d045: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a3d04c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a3d051: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
140a3d058: e8 53 9f e5 ff              	call	0x140896fb0 <.text+0x895fb0>
140a3d05d: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a3d062: 48 83 c4 48                 	add	rsp, 0x48
140a3d066: 5b                          	pop	rbx
140a3d067: 5f                          	pop	rdi
140a3d068: 5e                          	pop	rsi
140a3d069: 41 5c                       	pop	r12
140a3d06b: 41 5d                       	pop	r13
140a3d06d: 41 5e                       	pop	r14
140a3d06f: 41 5f                       	pop	r15
140a3d071: 5d                          	pop	rbp
140a3d072: c3                          	ret
140a3d073: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a3d080: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a3d085: 55                          	push	rbp
140a3d086: 41 57                       	push	r15
140a3d088: 41 56                       	push	r14
140a3d08a: 41 55                       	push	r13
140a3d08c: 41 54                       	push	r12
140a3d08e: 56                          	push	rsi
140a3d08f: 57                          	push	rdi
140a3d090: 53                          	push	rbx
140a3d091: 48 83 ec 48                 	sub	rsp, 0x48
140a3d095: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a3d09c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a3d0a1: 48 8d 8d f0 00 00 00        	lea	rcx, [rbp + 0xf0]
140a3d0a8: e8 03 9f e5 ff              	call	0x140896fb0 <.text+0x895fb0>
140a3d0ad: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a3d0b2: 48 83 c4 48                 	add	rsp, 0x48
140a3d0b6: 5b                          	pop	rbx
140a3d0b7: 5f                          	pop	rdi
140a3d0b8: 5e                          	pop	rsi
140a3d0b9: 41 5c                       	pop	r12
140a3d0bb: 41 5d                       	pop	r13
140a3d0bd: 41 5e                       	pop	r14
140a3d0bf: 41 5f                       	pop	r15
140a3d0c1: 5d                          	pop	rbp
140a3d0c2: c3                          	ret
140a3d0c3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a3d0d0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a3d0d5: 55                          	push	rbp
140a3d0d6: 41 57                       	push	r15
140a3d0d8: 41 56                       	push	r14
140a3d0da: 41 55                       	push	r13
140a3d0dc: 41 54                       	push	r12
140a3d0de: 56                          	push	rsi
140a3d0df: 57                          	push	rdi
140a3d0e0: 53                          	push	rbx
140a3d0e1: 48 83 ec 48                 	sub	rsp, 0x48
140a3d0e5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a3d0ec: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a3d0f1: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
140a3d0f8: e8 b3 9e e5 ff              	call	0x140896fb0 <.text+0x895fb0>
140a3d0fd: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a3d102: 48 83 c4 48                 	add	rsp, 0x48
140a3d106: 5b                          	pop	rbx
140a3d107: 5f                          	pop	rdi
140a3d108: 5e                          	pop	rsi
140a3d109: 41 5c                       	pop	r12
140a3d10b: 41 5d                       	pop	r13
140a3d10d: 41 5e                       	pop	r14
140a3d10f: 41 5f                       	pop	r15
140a3d111: 5d                          	pop	rbp
140a3d112: c3                          	ret
140a3d113: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a3d120: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a3d125: 55                          	push	rbp
140a3d126: 41 57                       	push	r15
140a3d128: 41 56                       	push	r14
140a3d12a: 41 55                       	push	r13
140a3d12c: 41 54                       	push	r12
140a3d12e: 56                          	push	rsi
140a3d12f: 57                          	push	rdi
140a3d130: 53                          	push	rbx
140a3d131: 48 83 ec 48                 	sub	rsp, 0x48
140a3d135: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a3d13c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a3d141: 48 8d 8d f0 00 00 00        	lea	rcx, [rbp + 0xf0]
140a3d148: e8 63 9e e5 ff              	call	0x140896fb0 <.text+0x895fb0>
140a3d14d: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a3d152: 48 83 c4 48                 	add	rsp, 0x48
140a3d156: 5b                          	pop	rbx
140a3d157: 5f                          	pop	rdi
140a3d158: 5e                          	pop	rsi
140a3d159: 41 5c                       	pop	r12
140a3d15b: 41 5d                       	pop	r13
140a3d15d: 41 5e                       	pop	r14
140a3d15f: 41 5f                       	pop	r15
140a3d161: 5d                          	pop	rbp
140a3d162: c3                          	ret
140a3d163: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a3d170: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a3d175: 55                          	push	rbp
140a3d176: 41 57                       	push	r15
140a3d178: 41 56                       	push	r14
140a3d17a: 41 55                       	push	r13
140a3d17c: 41 54                       	push	r12
140a3d17e: 56                          	push	rsi
140a3d17f: 57                          	push	rdi
140a3d180: 53                          	push	rbx
140a3d181: 48 83 ec 48                 	sub	rsp, 0x48
140a3d185: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a3d18c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a3d191: 48 8d 8d 90 02 00 00        	lea	rcx, [rbp + 0x290]
140a3d198: e8 13 9e e5 ff              	call	0x140896fb0 <.text+0x895fb0>
140a3d19d: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a3d1a2: 48 83 c4 48                 	add	rsp, 0x48
140a3d1a6: 5b                          	pop	rbx
140a3d1a7: 5f                          	pop	rdi
140a3d1a8: 5e                          	pop	rsi
140a3d1a9: 41 5c                       	pop	r12
140a3d1ab: 41 5d                       	pop	r13
140a3d1ad: 41 5e                       	pop	r14
140a3d1af: 41 5f                       	pop	r15
140a3d1b1: 5d                          	pop	rbp
140a3d1b2: c3                          	ret
140a3d1b3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a3d1c0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a3d1c5: 55                          	push	rbp
140a3d1c6: 41 57                       	push	r15
140a3d1c8: 41 56                       	push	r14
140a3d1ca: 41 55                       	push	r13
140a3d1cc: 41 54                       	push	r12
140a3d1ce: 56                          	push	rsi
140a3d1cf: 57                          	push	rdi
140a3d1d0: 53                          	push	rbx
140a3d1d1: 48 83 ec 48                 	sub	rsp, 0x48
140a3d1d5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a3d1dc: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a3d1e1: 80 bd 26 02 00 00 08        	cmp	byte ptr [rbp + 0x226], 0x8
140a3d1e8: 74 0c                       	je	0x140a3d1f6 <.text+0xa3c1f6>
140a3d1ea: 48 8d 8d 10 02 00 00        	lea	rcx, [rbp + 0x210]
140a3d1f1: e8 ba 52 1a 00              	call	0x140be24b0 <.text+0xbe14b0>
140a3d1f6: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a3d1fb: 48 83 c4 48                 	add	rsp, 0x48
140a3d1ff: 5b                          	pop	rbx
140a3d200: 5f                          	pop	rdi
140a3d201: 5e                          	pop	rsi
140a3d202: 41 5c                       	pop	r12
140a3d204: 41 5d                       	pop	r13
140a3d206: 41 5e                       	pop	r14
140a3d208: 41 5f                       	pop	r15
140a3d20a: 5d                          	pop	rbp
140a3d20b: c3                          	ret
140a3d20c: 0f 1f 40 00                 	nop	dword ptr [rax]
140a3d210: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a3d215: 55                          	push	rbp
140a3d216: 41 57                       	push	r15
140a3d218: 41 56                       	push	r14
140a3d21a: 41 55                       	push	r13
140a3d21c: 41 54                       	push	r12
140a3d21e: 56                          	push	rsi
140a3d21f: 57                          	push	rdi
140a3d220: 53                          	push	rbx
140a3d221: 48 83 ec 48                 	sub	rsp, 0x48
140a3d225: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a3d22c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a3d231: 48 8d 8d f0 00 00 00        	lea	rcx, [rbp + 0xf0]
140a3d238: e8 f3 f9 30 00              	call	0x140d4cc30 <.text+0xd4bc30>
140a3d23d: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a3d242: 48 83 c4 48                 	add	rsp, 0x48
140a3d246: 5b                          	pop	rbx
140a3d247: 5f                          	pop	rdi
140a3d248: 5e                          	pop	rsi
140a3d249: 41 5c                       	pop	r12
140a3d24b: 41 5d                       	pop	r13
140a3d24d: 41 5e                       	pop	r14
140a3d24f: 41 5f                       	pop	r15
140a3d251: 5d                          	pop	rbp
140a3d252: c3                          	ret
140a3d253: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a3d260: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a3d265: 55                          	push	rbp
140a3d266: 41 57                       	push	r15
140a3d268: 41 56                       	push	r14
140a3d26a: 41 55                       	push	r13
140a3d26c: 41 54                       	push	r12
140a3d26e: 56                          	push	rsi
140a3d26f: 57                          	push	rdi
140a3d270: 53                          	push	rbx
140a3d271: 48 83 ec 48                 	sub	rsp, 0x48
140a3d275: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a3d27c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a3d281: 48 8d 8d f0 00 00 00        	lea	rcx, [rbp + 0xf0]
140a3d288: e8 23 9d e5 ff              	call	0x140896fb0 <.text+0x895fb0>
140a3d28d: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a3d292: 48 83 c4 48                 	add	rsp, 0x48
140a3d296: 5b                          	pop	rbx
140a3d297: 5f                          	pop	rdi
140a3d298: 5e                          	pop	rsi
140a3d299: 41 5c                       	pop	r12
140a3d29b: 41 5d                       	pop	r13
140a3d29d: 41 5e                       	pop	r14
140a3d29f: 41 5f                       	pop	r15
140a3d2a1: 5d                          	pop	rbp
140a3d2a2: c3                          	ret
