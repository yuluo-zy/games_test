
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a5a570: 55                          	push	rbp
140a5a571: 41 57                       	push	r15
140a5a573: 41 56                       	push	r14
140a5a575: 41 55                       	push	r13
140a5a577: 41 54                       	push	r12
140a5a579: 56                          	push	rsi
140a5a57a: 57                          	push	rdi
140a5a57b: 53                          	push	rbx
140a5a57c: 48 81 ec 48 07 00 00        	sub	rsp, 0x748
140a5a583: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
140a5a58b: 0f 29 b5 b0 06 00 00        	movaps	xmmword ptr [rbp + 0x6b0], xmm6
140a5a592: 48 c7 85 a8 06 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x6a8], -0x2
140a5a59d: 4d 89 cf                    	mov	r15, r9
140a5a5a0: 4c 89 c3                    	mov	rbx, r8
140a5a5a3: 49 89 d4                    	mov	r12, rdx
140a5a5a6: 48 89 8d 58 06 00 00        	mov	qword ptr [rbp + 0x658], rcx
140a5a5ad: 48 8b bd 38 07 00 00        	mov	rdi, qword ptr [rbp + 0x738]
140a5a5b4: 80 3d c5 1a bd 02 00        	cmp	byte ptr [rip + 0x2bd1ac5], 0x0 # 0x14362c080
140a5a5bb: 0f 84 4b 01 00 00           	je	0x140a5a70c <.text+0xa5970c>
140a5a5c1: 48 89 9d 90 06 00 00        	mov	qword ptr [rbp + 0x690], rbx
140a5a5c8: 49 8d 4f 04                 	lea	rcx, [r15 + 0x4]
140a5a5cc: e8 bf b1 17 00              	call	0x140bd5790 <.text+0xbd4790>
140a5a5d1: 48 89 45 38                 	mov	qword ptr [rbp + 0x38], rax
140a5a5d5: 41 0f 10 47 04              	movups	xmm0, xmmword ptr [r15 + 0x4]
140a5a5da: 41 0f 10 4f 14              	movups	xmm1, xmmword ptr [r15 + 0x14]
140a5a5df: 0f 29 8d d0 04 00 00        	movaps	xmmword ptr [rbp + 0x4d0], xmm1
140a5a5e6: 0f 29 85 c0 04 00 00        	movaps	xmmword ptr [rbp + 0x4c0], xmm0
140a5a5ed: 48 8d b5 00 03 00 00        	lea	rsi, [rbp + 0x300]
140a5a5f4: 48 8d 95 c0 04 00 00        	lea	rdx, [rbp + 0x4c0]
140a5a5fb: 48 89 f1                    	mov	rcx, rsi
140a5a5fe: e8 2d b1 17 00              	call	0x140bd5730 <.text+0xbd4730>
140a5a603: 48 89 f1                    	mov	rcx, rsi
140a5a606: e8 85 b1 17 00              	call	0x140bd5790 <.text+0xbd4790>
140a5a60b: 48 89 45 30                 	mov	qword ptr [rbp + 0x30], rax
140a5a60f: 48 8d 77 28                 	lea	rsi, [rdi + 0x28]
140a5a613: 0f 10 47 28                 	movups	xmm0, xmmword ptr [rdi + 0x28]
140a5a617: 0f 10 4f 38                 	movups	xmm1, xmmword ptr [rdi + 0x38]
140a5a61b: 0f 29 8d d0 04 00 00        	movaps	xmmword ptr [rbp + 0x4d0], xmm1
140a5a622: 0f 29 85 c0 04 00 00        	movaps	xmmword ptr [rbp + 0x4c0], xmm0
140a5a629: 48 8d 4d 40                 	lea	rcx, [rbp + 0x40]
140a5a62d: 48 8d 95 c0 04 00 00        	lea	rdx, [rbp + 0x4c0]
140a5a634: 41 b8 4c 00 00 00           	mov	r8d, 0x4c
140a5a63a: e8 f1 ad 17 00              	call	0x140bd5430 <.text+0xbd4430>
140a5a63f: 0f 10 45 40                 	movups	xmm0, xmmword ptr [rbp + 0x40]
140a5a643: 0f 10 4d 50                 	movups	xmm1, xmmword ptr [rbp + 0x50]
140a5a647: 0f 29 8d 10 03 00 00        	movaps	xmmword ptr [rbp + 0x310], xmm1
140a5a64e: 0f 29 85 00 03 00 00        	movaps	xmmword ptr [rbp + 0x300], xmm0
140a5a655: 48 8d 8d d0 04 00 00        	lea	rcx, [rbp + 0x4d0]
140a5a65c: 48 8d 95 00 03 00 00        	lea	rdx, [rbp + 0x300]
140a5a663: e8 78 c6 17 00              	call	0x140bd6ce0 <.text+0xbd5ce0>
140a5a668: 48 8d 05 a9 8b 07 02        	lea	rax, [rip + 0x2078ba9]  # 0x142ad3218
140a5a66f: 48 89 85 c0 04 00 00        	mov	qword ptr [rbp + 0x4c0], rax
140a5a676: 48 c7 85 c8 04 00 00 0b 00 00 00    	mov	qword ptr [rbp + 0x4c8], 0xb
140a5a681: 48 8d 95 c0 04 00 00        	lea	rdx, [rbp + 0x4c0]
140a5a688: 4c 89 a5 98 06 00 00        	mov	qword ptr [rbp + 0x698], r12
140a5a68f: 4c 89 e1                    	mov	rcx, r12
140a5a692: e8 39 ee 18 00              	call	0x140be94d0 <.text+0xbe84d0>
140a5a697: 0f 10 45 40                 	movups	xmm0, xmmword ptr [rbp + 0x40]
140a5a69b: 0f 10 4d 50                 	movups	xmm1, xmmword ptr [rbp + 0x50]
140a5a69f: 0f 11 45 c0                 	movups	xmmword ptr [rbp - 0x40], xmm0
140a5a6a3: 0f 11 4d d0                 	movups	xmmword ptr [rbp - 0x30], xmm1
140a5a6a7: 89 45 bc                    	mov	dword ptr [rbp - 0x44], eax
140a5a6aa: 48 89 f1                    	mov	rcx, rsi
140a5a6ad: e8 de b0 17 00              	call	0x140bd5790 <.text+0xbd4790>
140a5a6b2: 49 89 c1                    	mov	r9, rax
140a5a6b5: 49 c1 e9 20                 	shr	r9, 0x20
140a5a6b9: 89 c2                       	mov	edx, eax
140a5a6bb: d1 ea                       	shr	edx
140a5a6bd: 89 c1                       	mov	ecx, eax
140a5a6bf: 29 d1                       	sub	ecx, edx
140a5a6c1: 49 89 c0                    	mov	r8, rax
140a5a6c4: 49 c1 e8 21                 	shr	r8, 0x21
140a5a6c8: 44 89 ca                    	mov	edx, r9d
140a5a6cb: 44 29 c2                    	sub	edx, r8d
140a5a6ce: 0f af d1                    	imul	edx, ecx
140a5a6d1: 4c 89 c9                    	mov	rcx, r9
140a5a6d4: 4c 89 4d 28                 	mov	qword ptr [rbp + 0x28], r9
140a5a6d8: 48 89 45 20                 	mov	qword ptr [rbp + 0x20], rax
140a5a6dc: 0f af c8                    	imul	ecx, eax
140a5a6df: 89 8d ac 04 00 00           	mov	dword ptr [rbp + 0x4ac], ecx
140a5a6e5: 89 95 a8 04 00 00           	mov	dword ptr [rbp + 0x4a8], edx
140a5a6eb: be 0c 00 00 00              	mov	esi, 0xc
140a5a6f0: 45 31 f6                    	xor	r14d, r14d
140a5a6f3: b1 02                       	mov	cl, 0x2
140a5a6f5: 41 0f b6 d6                 	movzx	edx, r14b
140a5a6f9: 48 8d 05 14 8f 07 02        	lea	rax, [rip + 0x2078f14]  # 0x142ad3614
140a5a700: 48 63 14 90                 	movsxd	rdx, dword ptr [rax + 4*rdx]
140a5a704: 48 01 c2                    	add	rdx, rax
140a5a707: 45 31 e4                    	xor	r12d, r12d
140a5a70a: ff e2                       	jmp	rdx
140a5a70c: 49 89 fe                    	mov	r14, rdi
140a5a70f: 49 8d 7f 04                 	lea	rdi, [r15 + 0x4]
140a5a713: 48 89 f9                    	mov	rcx, rdi
140a5a716: e8 75 b0 17 00              	call	0x140bd5790 <.text+0xbd4790>
140a5a71b: 48 89 c6                    	mov	rsi, rax
140a5a71e: 48 89 f9                    	mov	rcx, rdi
140a5a721: e8 6a b0 17 00              	call	0x140bd5790 <.text+0xbd4790>
140a5a726: 48 8d 4d 40                 	lea	rcx, [rbp + 0x40]
140a5a72a: 48 89 c2                    	mov	rdx, rax
140a5a72d: 41 b8 7a 00 00 00           	mov	r8d, 0x7a
140a5a733: e8 68 ab 17 00              	call	0x140bd52a0 <.text+0xbd42a0>
140a5a738: 0f 10 45 40                 	movups	xmm0, xmmword ptr [rbp + 0x40]
140a5a73c: 0f 10 4d 50                 	movups	xmm1, xmmword ptr [rbp + 0x50]
140a5a740: 0f 29 8d 10 03 00 00        	movaps	xmmword ptr [rbp + 0x310], xmm1
140a5a747: 0f 29 85 00 03 00 00        	movaps	xmmword ptr [rbp + 0x300], xmm0
140a5a74e: 48 8d 8d d0 04 00 00        	lea	rcx, [rbp + 0x4d0]
140a5a755: 48 8d bd 00 03 00 00        	lea	rdi, [rbp + 0x300]
140a5a75c: 48 89 fa                    	mov	rdx, rdi
140a5a75f: e8 7c c5 17 00              	call	0x140bd6ce0 <.text+0xbd5ce0>
140a5a764: 48 8d 05 28 8a 07 02        	lea	rax, [rip + 0x2078a28]  # 0x142ad3193
140a5a76b: 48 89 85 c0 04 00 00        	mov	qword ptr [rbp + 0x4c0], rax
140a5a772: 48 c7 85 c8 04 00 00 0a 00 00 00    	mov	qword ptr [rbp + 0x4c8], 0xa
140a5a77d: 48 8d 95 c0 04 00 00        	lea	rdx, [rbp + 0x4c0]
140a5a784: 4c 89 e1                    	mov	rcx, r12
140a5a787: e8 44 ed 18 00              	call	0x140be94d0 <.text+0xbe84d0>
140a5a78c: 0f 10 45 40                 	movups	xmm0, xmmword ptr [rbp + 0x40]
140a5a790: 0f 10 4d 50                 	movups	xmm1, xmmword ptr [rbp + 0x50]
140a5a794: 0f 11 85 24 02 00 00        	movups	xmmword ptr [rbp + 0x224], xmm0
140a5a79b: 0f 11 8d 34 02 00 00        	movups	xmmword ptr [rbp + 0x234], xmm1
140a5a7a2: 89 85 20 02 00 00           	mov	dword ptr [rbp + 0x220], eax
140a5a7a8: 48 8d 4b 38                 	lea	rcx, [rbx + 0x38]
140a5a7ac: 48 ba b9 75 23 64 09 2e 14 4a       	movabs	rdx, 0x4a142e09642375b9
140a5a7b6: 49 b8 7f 84 8c fa 4c 15 f6 43       	movabs	r8, 0x43f6154cfa8c847f
140a5a7c0: e8 2b 2f 68 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
140a5a7c5: 48 85 c0                    	test	rax, rax
140a5a7c8: 0f 84 de 03 00 00           	je	0x140a5abac <.text+0xa59bac>
140a5a7ce: 48 39 93 18 02 00 00        	cmp	qword ptr [rbx + 0x218], rdx
140a5a7d5: 0f 86 d1 03 00 00           	jbe	0x140a5abac <.text+0xa59bac>
140a5a7db: 48 8b 83 10 02 00 00        	mov	rax, qword ptr [rbx + 0x210]
140a5a7e2: 48 8b 0c d0                 	mov	rcx, qword ptr [rax + 8*rdx]
140a5a7e6: 48 85 c9                    	test	rcx, rcx
140a5a7e9: 0f 84 bd 03 00 00           	je	0x140a5abac <.text+0xa59bac>
140a5a7ef: 48 8b 83 e0 01 00 00        	mov	rax, qword ptr [rbx + 0x1e0]
140a5a7f6: 48 f7 d1                    	not	rcx
140a5a7f9: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
140a5a7fd: 48 c1 e1 05                 	shl	rcx, 0x5
140a5a801: 48 83 7c 08 20 00           	cmp	qword ptr [rax + rcx + 0x20], 0x0
140a5a807: 0f 84 9f 03 00 00           	je	0x140a5abac <.text+0xa59bac>
140a5a80d: 48 01 c8                    	add	rax, rcx
140a5a810: 48 8b 40 10                 	mov	rax, qword ptr [rax + 0x10]
140a5a814: 0f b6 88 c0 00 00 00        	movzx	ecx, byte ptr [rax + 0xc0]
140a5a81b: c1 e1 06                    	shl	ecx, 0x6
140a5a81e: f3 0f 10 74 08 30           	movss	xmm6, dword ptr [rax + rcx + 0x30]
140a5a824: 4c 89 64 24 30              	mov	qword ptr [rsp + 0x30], r12
140a5a829: 48 89 5c 24 28              	mov	qword ptr [rsp + 0x28], rbx
140a5a82e: 48 c7 44 24 20 12 00 00 00  	mov	qword ptr [rsp + 0x20], 0x12
140a5a837: 48 8d 15 ba 89 07 02        	lea	rdx, [rip + 0x20789ba]  # 0x142ad31f8
140a5a83e: 4c 8d 0d c1 89 07 02        	lea	r9, [rip + 0x20789c1]   # 0x142ad3206
140a5a845: 48 8d 9d c0 04 00 00        	lea	rbx, [rbp + 0x4c0]
140a5a84c: 41 b8 0e 00 00 00           	mov	r8d, 0xe
140a5a852: 48 89 d9                    	mov	rcx, rbx
140a5a855: e8 46 8a ba 00              	call	0x1416032a0 <.text+0x16022a0>
140a5a85a: 4c 89 7d 40                 	mov	qword ptr [rbp + 0x40], r15
140a5a85e: c7 85 00 03 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x300], 0x2
140a5a868: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
140a5a86d: 48 8d 4d 40                 	lea	rcx, [rbp + 0x40]
140a5a871: 49 b8 5e a9 e9 1c 54 8e 97 d9       	movabs	r8, -0x266871abe31656a2
140a5a87b: 48 89 da                    	mov	rdx, rbx
140a5a87e: 41 b1 19                    	mov	r9b, 0x19
140a5a881: e8 ba 39 e9 ff              	call	0x1408ee240 <.text+0x8ed240>
140a5a886: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a5a88c: 48 89 f9                    	mov	rcx, rdi
140a5a88f: 48 89 da                    	mov	rdx, rbx
140a5a892: e8 69 93 ec 01              	call	0x142923c00 <.text+0x2922c00>
140a5a897: 4c 89 75 40                 	mov	qword ptr [rbp + 0x40], r14
140a5a89b: c7 85 c0 04 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x4c0], 0x2
140a5a8a5: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
140a5a8aa: 48 8d 4d 40                 	lea	rcx, [rbp + 0x40]
140a5a8ae: 49 b8 5a 0b 50 93 7f 1f 0c b9       	movabs	r8, -0x46f3e0806caff4a6
140a5a8b8: 48 89 fa                    	mov	rdx, rdi
140a5a8bb: 41 b1 19                    	mov	r9b, 0x19
140a5a8be: e8 7d 39 e9 ff              	call	0x1408ee240 <.text+0x8ed240>
140a5a8c3: 48 8d bd c0 04 00 00        	lea	rdi, [rbp + 0x4c0]
140a5a8ca: 48 8d 9d 00 03 00 00        	lea	rbx, [rbp + 0x300]
140a5a8d1: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a5a8d7: 48 89 f9                    	mov	rcx, rdi
140a5a8da: 48 89 da                    	mov	rdx, rbx
140a5a8dd: e8 1e 93 ec 01              	call	0x142923c00 <.text+0x2922c00>
140a5a8e2: 48 8b 85 30 07 00 00        	mov	rax, qword ptr [rbp + 0x730]
140a5a8e9: 48 89 45 40                 	mov	qword ptr [rbp + 0x40], rax
140a5a8ed: c7 85 00 03 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x300], 0x2
140a5a8f7: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
140a5a8fc: 48 8d 4d 40                 	lea	rcx, [rbp + 0x40]
140a5a900: 49 b8 fc fb 49 a0 9d 57 61 48       	movabs	r8, 0x4861579da049fbfc
140a5a90a: 48 89 fa                    	mov	rdx, rdi
140a5a90d: 41 b1 19                    	mov	r9b, 0x19
140a5a910: e8 2b 39 e9 ff              	call	0x1408ee240 <.text+0x8ed240>
140a5a915: 48 8d bd 00 03 00 00        	lea	rdi, [rbp + 0x300]
140a5a91c: 48 8d 9d c0 04 00 00        	lea	rbx, [rbp + 0x4c0]
140a5a923: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a5a929: 48 89 f9                    	mov	rcx, rdi
140a5a92c: 48 89 da                    	mov	rdx, rbx
140a5a92f: e8 cc 92 ec 01              	call	0x142923c00 <.text+0x2922c00>
140a5a934: 48 b8 65 18 a0 91 ab 39 9f 57       	movabs	rax, 0x579f39ab91a01865
140a5a93e: 48 89 45 40                 	mov	qword ptr [rbp + 0x40], rax
140a5a942: c7 85 c0 04 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x4c0], 0x2
140a5a94c: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
140a5a951: 48 8d 4d 40                 	lea	rcx, [rbp + 0x40]
140a5a955: 49 b8 8c 97 66 69 03 f5 5b 81       	movabs	r8, -0x7ea40afc96996874
140a5a95f: 48 89 fa                    	mov	rdx, rdi
140a5a962: 41 b1 19                    	mov	r9b, 0x19
140a5a965: e8 66 ad ea ff              	call	0x1409056d0 <.text+0x9046d0>
140a5a96a: 48 8d bd c0 04 00 00        	lea	rdi, [rbp + 0x4c0]
140a5a971: 48 8d 9d 00 03 00 00        	lea	rbx, [rbp + 0x300]
140a5a978: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a5a97e: 48 89 f9                    	mov	rcx, rdi
140a5a981: 48 89 da                    	mov	rdx, rbx
140a5a984: e8 77 92 ec 01              	call	0x142923c00 <.text+0x2922c00>
140a5a989: 48 8d 85 20 02 00 00        	lea	rax, [rbp + 0x220]
140a5a990: 48 89 45 40                 	mov	qword ptr [rbp + 0x40], rax
140a5a994: c7 85 00 03 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x300], 0x2
140a5a99e: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
140a5a9a3: 48 8d 4d 40                 	lea	rcx, [rbp + 0x40]
140a5a9a7: 49 b8 c8 8b 41 02 02 79 81 b9       	movabs	r8, -0x467e86fdfdbe7438
140a5a9b1: 48 89 fa                    	mov	rdx, rdi
140a5a9b4: 41 b1 32                    	mov	r9b, 0x32
140a5a9b7: e8 44 33 e9 ff              	call	0x1408edd00 <.text+0x8ecd00>
140a5a9bc: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a5a9c3: 48 8d 95 c0 04 00 00        	lea	rdx, [rbp + 0x4c0]
140a5a9ca: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a5a9d0: e8 2b 92 ec 01              	call	0x142923c00 <.text+0x2922c00>
140a5a9d5: 80 bd 36 04 00 00 08        	cmp	byte ptr [rbp + 0x436], 0x8
140a5a9dc: 74 25                       	je	0x140a5aa03 <.text+0xa59a03>
140a5a9de: 48 8d 8d 10 04 00 00        	lea	rcx, [rbp + 0x410]
140a5a9e5: ba e8 00 00 00              	mov	edx, 0xe8
140a5a9ea: 48 03 95 38 04 00 00        	add	rdx, qword ptr [rbp + 0x438]
140a5a9f1: 49 b8 ab 08 87 47 b4 e0 4e b3       	movabs	r8, -0x4cb11f4bb878f755
140a5a9fb: 0f 28 de                    	movaps	xmm3, xmm6
140a5a9fe: e8 2d 10 f1 ff              	call	0x14096ba30 <.text+0x96aa30>
140a5aa03: 48 89 f7                    	mov	rdi, rsi
140a5aa06: 48 c1 ef 20                 	shr	rdi, 0x20
140a5aa0a: 48 8d 8d c0 04 00 00        	lea	rcx, [rbp + 0x4c0]
140a5aa11: 48 8d 95 00 03 00 00        	lea	rdx, [rbp + 0x300]
140a5aa18: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a5aa1e: e8 dd 91 ec 01              	call	0x142923c00 <.text+0x2922c00>
140a5aa23: 0f 10 85 60 05 00 00        	movups	xmm0, xmmword ptr [rbp + 0x560]
140a5aa2a: 0f 29 85 80 00 00 00        	movaps	xmmword ptr [rbp + 0x80], xmm0
140a5aa31: 0f 10 85 50 05 00 00        	movups	xmm0, xmmword ptr [rbp + 0x550]
140a5aa38: 0f 29 45 70                 	movaps	xmmword ptr [rbp + 0x70], xmm0
140a5aa3c: 0f 10 85 40 05 00 00        	movups	xmm0, xmmword ptr [rbp + 0x540]
140a5aa43: 0f 29 45 60                 	movaps	xmmword ptr [rbp + 0x60], xmm0
140a5aa47: 0f 10 85 30 05 00 00        	movups	xmm0, xmmword ptr [rbp + 0x530]
140a5aa4e: 0f 29 45 50                 	movaps	xmmword ptr [rbp + 0x50], xmm0
140a5aa52: 0f 10 85 20 05 00 00        	movups	xmm0, xmmword ptr [rbp + 0x520]
140a5aa59: 0f 29 45 40                 	movaps	xmmword ptr [rbp + 0x40], xmm0
140a5aa5d: 48 8b 85 f8 05 00 00        	mov	rax, qword ptr [rbp + 0x5f8]
140a5aa64: 48 8b 8d 40 06 00 00        	mov	rcx, qword ptr [rbp + 0x640]
140a5aa6b: 48 8b 95 d0 04 00 00        	mov	rdx, qword ptr [rbp + 0x4d0]
140a5aa72: 48 89 95 10 03 00 00        	mov	qword ptr [rbp + 0x310], rdx
140a5aa79: 0f 10 85 c0 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x4c0]
140a5aa80: 0f 29 85 00 03 00 00        	movaps	xmmword ptr [rbp + 0x300], xmm0
140a5aa87: 0f 10 85 d8 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x4d8]
140a5aa8e: 0f 29 85 30 03 00 00        	movaps	xmmword ptr [rbp + 0x330], xmm0
140a5aa95: 48 8b 95 e8 04 00 00        	mov	rdx, qword ptr [rbp + 0x4e8]
140a5aa9c: 48 89 95 40 03 00 00        	mov	qword ptr [rbp + 0x340], rdx
140a5aaa3: 0f 10 85 f0 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x4f0]
140a5aaaa: 0f 11 85 18 03 00 00        	movups	xmmword ptr [rbp + 0x318], xmm0
140a5aab1: 48 8b 95 00 05 00 00        	mov	rdx, qword ptr [rbp + 0x500]
140a5aab8: 48 89 95 28 03 00 00        	mov	qword ptr [rbp + 0x328], rdx
140a5aabf: 0f 10 85 70 05 00 00        	movups	xmm0, xmmword ptr [rbp + 0x570]
140a5aac6: 0f 10 8d 80 05 00 00        	movups	xmm1, xmmword ptr [rbp + 0x580]
140a5aacd: 0f 10 95 90 05 00 00        	movups	xmm2, xmmword ptr [rbp + 0x590]
140a5aad4: 0f 10 9d a0 05 00 00        	movups	xmm3, xmmword ptr [rbp + 0x5a0]
140a5aadb: 0f 11 85 78 03 00 00        	movups	xmmword ptr [rbp + 0x378], xmm0
140a5aae2: 0f 11 8d 88 03 00 00        	movups	xmmword ptr [rbp + 0x388], xmm1
140a5aae9: 0f 11 95 98 03 00 00        	movups	xmmword ptr [rbp + 0x398], xmm2
140a5aaf0: 0f 11 9d a8 03 00 00        	movups	xmmword ptr [rbp + 0x3a8], xmm3
140a5aaf7: 0f 10 85 b0 05 00 00        	movups	xmm0, xmmword ptr [rbp + 0x5b0]
140a5aafe: 0f 11 85 b8 03 00 00        	movups	xmmword ptr [rbp + 0x3b8], xmm0
140a5ab05: 0f 10 85 c0 05 00 00        	movups	xmm0, xmmword ptr [rbp + 0x5c0]
140a5ab0c: 0f 11 85 c8 03 00 00        	movups	xmmword ptr [rbp + 0x3c8], xmm0
140a5ab13: 48 8b 95 18 05 00 00        	mov	rdx, qword ptr [rbp + 0x518]
140a5ab1a: 48 89 95 58 03 00 00        	mov	qword ptr [rbp + 0x358], rdx
140a5ab21: 0f 10 85 08 05 00 00        	movups	xmm0, xmmword ptr [rbp + 0x508]
140a5ab28: 0f 11 85 48 03 00 00        	movups	xmmword ptr [rbp + 0x348], xmm0
140a5ab2f: 48 89 8d e0 03 00 00        	mov	qword ptr [rbp + 0x3e0], rcx
140a5ab36: 48 89 85 d8 03 00 00        	mov	qword ptr [rbp + 0x3d8], rax
140a5ab3d: c7 85 60 03 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x360], 0x1
140a5ab47: 89 b5 64 03 00 00           	mov	dword ptr [rbp + 0x364], esi
140a5ab4d: 89 bd 68 03 00 00           	mov	dword ptr [rbp + 0x368], edi
140a5ab53: c7 85 6c 03 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x36c], 0x1
140a5ab5d: 48 8d 4d 40                 	lea	rcx, [rbp + 0x40]
140a5ab61: 48 8d 95 00 03 00 00        	lea	rdx, [rbp + 0x300]
140a5ab68: e8 63 29 f8 ff              	call	0x1409dd4d0 <.text+0x9dc4d0>
140a5ab6d: 80 bd f6 05 00 00 08        	cmp	byte ptr [rbp + 0x5f6], 0x8
140a5ab74: 74 0c                       	je	0x140a5ab82 <.text+0xa59b82>
140a5ab76: 48 8d 8d e0 05 00 00        	lea	rcx, [rbp + 0x5e0]
140a5ab7d: e8 2e 79 18 00              	call	0x140be24b0 <.text+0xbe14b0>
140a5ab82: 8b 85 40 02 00 00           	mov	eax, dword ptr [rbp + 0x240]
140a5ab88: 48 8b 8d 58 06 00 00        	mov	rcx, qword ptr [rbp + 0x658]
140a5ab8f: 89 41 20                    	mov	dword ptr [rcx + 0x20], eax
140a5ab92: 48 89 c8                    	mov	rax, rcx
140a5ab95: 0f 10 85 20 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x220]
140a5ab9c: 0f 10 8d 30 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x230]
140a5aba3: 0f 11 49 10                 	movups	xmmword ptr [rcx + 0x10], xmm1
140a5aba7: 0f 11 01                    	movups	xmmword ptr [rcx], xmm0
140a5abaa: eb 0c                       	jmp	0x140a5abb8 <.text+0xa59bb8>
140a5abac: 48 8d 0d 2d 86 07 02        	lea	rcx, [rip + 0x207862d]  # 0x142ad31e0
140a5abb3: e8 58 e7 e7 01              	call	0x1428d9310 <.text+0x28d8310>
140a5abb8: 0f 28 b5 b0 06 00 00        	movaps	xmm6, xmmword ptr [rbp + 0x6b0]
140a5abbf: 48 81 c4 48 07 00 00        	add	rsp, 0x748
140a5abc6: 5b                          	pop	rbx
140a5abc7: 5f                          	pop	rdi
140a5abc8: 5e                          	pop	rsi
140a5abc9: 41 5c                       	pop	r12
140a5abcb: 41 5d                       	pop	r13
140a5abcd: 41 5e                       	pop	r14
140a5abcf: 41 5f                       	pop	r15
140a5abd1: 5d                          	pop	rbp
140a5abd2: c3                          	ret
140a5abd3: b1 03                       	mov	cl, 0x3
140a5abd5: eb 19                       	jmp	0x140a5abf0 <.text+0xa59bf0>
140a5abd7: b1 04                       	mov	cl, 0x4
140a5abd9: eb 15                       	jmp	0x140a5abf0 <.text+0xa59bf0>
140a5abdb: b1 05                       	mov	cl, 0x5
140a5abdd: eb 11                       	jmp	0x140a5abf0 <.text+0xa59bf0>
140a5abdf: b1 06                       	mov	cl, 0x6
140a5abe1: eb 0d                       	jmp	0x140a5abf0 <.text+0xa59bf0>
140a5abe3: b1 07                       	mov	cl, 0x7
140a5abe5: eb 09                       	jmp	0x140a5abf0 <.text+0xa59bf0>
140a5abe7: b1 01                       	mov	cl, 0x1
140a5abe9: 0f 1f 80 00 00 00 00        	nop	dword ptr [rax]
140a5abf0: 48 83 c6 04                 	add	rsi, 0x4
140a5abf4: 41 ff c6                    	inc	r14d
140a5abf7: 49 83 c4 10                 	add	r12, 0x10
140a5abfb: 0f b6 c9                    	movzx	ecx, cl
140a5abfe: 48 63 14 88                 	movsxd	rdx, dword ptr [rax + 4*rcx]
140a5ac02: 48 01 c2                    	add	rdx, rax
140a5ac05: b1 02                       	mov	cl, 0x2
140a5ac07: ff e2                       	jmp	rdx
140a5ac09: 4c 89 bd b0 04 00 00        	mov	qword ptr [rbp + 0x4b0], r15
140a5ac10: 48 8d 85 ac 04 00 00        	lea	rax, [rbp + 0x4ac]
140a5ac17: 48 89 85 c0 04 00 00        	mov	qword ptr [rbp + 0x4c0], rax
140a5ac1e: 48 8d 85 a8 04 00 00        	lea	rax, [rbp + 0x4a8]
140a5ac25: 48 89 85 c8 04 00 00        	mov	qword ptr [rbp + 0x4c8], rax
140a5ac2c: c6 85 d0 04 00 00 00        	mov	byte ptr [rbp + 0x4d0], 0x0
140a5ac33: 4c 8d 05 ae 84 07 02        	lea	r8, [rip + 0x20784ae]   # 0x142ad30e8
140a5ac3a: 48 8d 8d d8 01 00 00        	lea	rcx, [rbp + 0x1d8]
140a5ac41: 48 8d 95 c0 04 00 00        	lea	rdx, [rbp + 0x4c0]
140a5ac48: e8 83 2e 8a 00              	call	0x1412fdad0 <.text+0x12fcad0>
140a5ac4d: 4c 8b ad e8 01 00 00        	mov	r13, qword ptr [rbp + 0x1e8]
140a5ac54: 4e 8d 04 ad 00 00 00 00     	lea	r8, [4*r13]
140a5ac5c: 31 db                       	xor	ebx, ebx
140a5ac5e: 4c 89 e8                    	mov	rax, r13
140a5ac61: 48 c1 e8 3e                 	shr	rax, 0x3e
140a5ac65: 0f 85 06 19 00 00           	jne	0x140a5c571 <.text+0xa5b571>
140a5ac6b: 48 b8 fc ff ff ff ff ff ff 7f       	movabs	rax, 0x7ffffffffffffffc
140a5ac75: 49 39 c0                    	cmp	r8, rax
140a5ac78: 0f 87 f3 18 00 00           	ja	0x140a5c571 <.text+0xa5b571>
140a5ac7e: 48 8b bd e0 01 00 00        	mov	rdi, qword ptr [rbp + 0x1e0]
140a5ac85: 45 31 ff                    	xor	r15d, r15d
140a5ac88: 4d 85 c0                    	test	r8, r8
140a5ac8b: 4c 89 85 78 06 00 00        	mov	qword ptr [rbp + 0x678], r8
140a5ac92: 74 32                       	je	0x140a5acc6 <.text+0xa59cc6>
140a5ac94: 0f b6 05 6e 9f bd 02        	movzx	eax, byte ptr [rip + 0x2bd9f6e] # 0x143634c09
140a5ac9b: bb 04 00 00 00              	mov	ebx, 0x4
140a5aca0: ba 04 00 00 00              	mov	edx, 0x4
140a5aca5: 4c 89 c1                    	mov	rcx, r8
140a5aca8: e8 63 8f bb ff              	call	0x140613c10 <.text+0x612c10>
140a5acad: 4c 8b 85 78 06 00 00        	mov	r8, qword ptr [rbp + 0x678]
140a5acb4: 48 85 c0                    	test	rax, rax
140a5acb7: 0f 84 b4 18 00 00           	je	0x140a5c571 <.text+0xa5b571>
140a5acbd: 4c 89 ad b8 04 00 00        	mov	qword ptr [rbp + 0x4b8], r13
140a5acc4: eb 10                       	jmp	0x140a5acd6 <.text+0xa59cd6>
140a5acc6: b8 04 00 00 00              	mov	eax, 0x4
140a5accb: 48 c7 85 b8 04 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x4b8], 0x0
140a5acd6: 48 89 85 80 06 00 00        	mov	qword ptr [rbp + 0x680], rax
140a5acdd: 48 89 c1                    	mov	rcx, rax
140a5ace0: 48 89 fa                    	mov	rdx, rdi
140a5ace3: e8 18 8f ec 01              	call	0x142923c00 <.text+0x2922c00>
140a5ace8: 4d 85 ed                    	test	r13, r13
140a5aceb: 0f 84 c5 00 00 00           	je	0x140a5adb6 <.text+0xa59db6>
140a5acf1: 48 8b 85 78 06 00 00        	mov	rax, qword ptr [rbp + 0x678]
140a5acf8: 48 8d 50 fc                 	lea	rdx, [rax - 0x4]
140a5acfc: 48 89 d1                    	mov	rcx, rdx
140a5acff: 48 c1 e9 02                 	shr	rcx, 0x2
140a5ad03: 48 ff c1                    	inc	rcx
140a5ad06: 89 c8                       	mov	eax, ecx
140a5ad08: 83 e0 07                    	and	eax, 0x7
140a5ad0b: 48 83 fa 1c                 	cmp	rdx, 0x1c
140a5ad0f: 73 0b                       	jae	0x140a5ad1c <.text+0xa59d1c>
140a5ad11: 31 d2                       	xor	edx, edx
140a5ad13: 4c 8b 85 80 06 00 00        	mov	r8, qword ptr [rbp + 0x680]
140a5ad1a: eb 74                       	jmp	0x140a5ad90 <.text+0xa59d90>
140a5ad1c: 48 83 e1 f8                 	and	rcx, -0x8
140a5ad20: 31 d2                       	xor	edx, edx
140a5ad22: 4c 8b 85 80 06 00 00        	mov	r8, qword ptr [rbp + 0x680]
140a5ad29: 0f 1f 80 00 00 00 00        	nop	dword ptr [rax]
140a5ad30: 45 8b 08                    	mov	r9d, dword ptr [r8]
140a5ad33: 41 01 d1                    	add	r9d, edx
140a5ad36: 41 89 10                    	mov	dword ptr [r8], edx
140a5ad39: 41 8b 50 04                 	mov	edx, dword ptr [r8 + 0x4]
140a5ad3d: 44 01 ca                    	add	edx, r9d
140a5ad40: 45 89 48 04                 	mov	dword ptr [r8 + 0x4], r9d
140a5ad44: 45 8b 48 08                 	mov	r9d, dword ptr [r8 + 0x8]
140a5ad48: 41 01 d1                    	add	r9d, edx
140a5ad4b: 41 89 50 08                 	mov	dword ptr [r8 + 0x8], edx
140a5ad4f: 41 8b 50 0c                 	mov	edx, dword ptr [r8 + 0xc]
140a5ad53: 44 01 ca                    	add	edx, r9d
140a5ad56: 45 89 48 0c                 	mov	dword ptr [r8 + 0xc], r9d
140a5ad5a: 45 8b 48 10                 	mov	r9d, dword ptr [r8 + 0x10]
140a5ad5e: 41 01 d1                    	add	r9d, edx
140a5ad61: 41 89 50 10                 	mov	dword ptr [r8 + 0x10], edx
140a5ad65: 41 8b 50 14                 	mov	edx, dword ptr [r8 + 0x14]
140a5ad69: 44 01 ca                    	add	edx, r9d
140a5ad6c: 45 89 48 14                 	mov	dword ptr [r8 + 0x14], r9d
140a5ad70: 45 8b 48 18                 	mov	r9d, dword ptr [r8 + 0x18]
140a5ad74: 41 01 d1                    	add	r9d, edx
140a5ad77: 41 89 50 18                 	mov	dword ptr [r8 + 0x18], edx
140a5ad7b: 41 8b 50 1c                 	mov	edx, dword ptr [r8 + 0x1c]
140a5ad7f: 45 89 48 1c                 	mov	dword ptr [r8 + 0x1c], r9d
140a5ad83: 49 83 c0 20                 	add	r8, 0x20
140a5ad87: 44 01 ca                    	add	edx, r9d
140a5ad8a: 48 83 c1 f8                 	add	rcx, -0x8
140a5ad8e: 75 a0                       	jne	0x140a5ad30 <.text+0xa59d30>
140a5ad90: 41 89 d7                    	mov	r15d, edx
140a5ad93: 48 85 c0                    	test	rax, rax
140a5ad96: 74 1e                       	je	0x140a5adb6 <.text+0xa59db6>
140a5ad98: 31 c9                       	xor	ecx, ecx
140a5ad9a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
140a5ada0: 45 8b 3c 88                 	mov	r15d, dword ptr [r8 + 4*rcx]
140a5ada4: 41 01 d7                    	add	r15d, edx
140a5ada7: 41 89 14 88                 	mov	dword ptr [r8 + 4*rcx], edx
140a5adab: 48 ff c1                    	inc	rcx
140a5adae: 44 89 fa                    	mov	edx, r15d
140a5adb1: 48 39 c8                    	cmp	rax, rcx
140a5adb4: 75 ea                       	jne	0x140a5ada0 <.text+0xa59da0>
140a5adb6: 48 83 e6 f0                 	and	rsi, -0x10
140a5adba: 48 8d 4d 40                 	lea	rcx, [rbp + 0x40]
140a5adbe: 48 89 f2                    	mov	rdx, rsi
140a5adc1: 45 31 c0                    	xor	r8d, r8d
140a5adc4: e8 57 a0 17 00              	call	0x140bd4e20 <.text+0xbd3e20>
140a5adc9: 0f 10 45 40                 	movups	xmm0, xmmword ptr [rbp + 0x40]
140a5adcd: 0f 29 85 00 03 00 00        	movaps	xmmword ptr [rbp + 0x300], xmm0
140a5add4: 48 8d 8d d0 04 00 00        	lea	rcx, [rbp + 0x4d0]
140a5addb: 48 8d 95 00 03 00 00        	lea	rdx, [rbp + 0x300]
140a5ade2: e8 19 bf 17 00              	call	0x140bd6d00 <.text+0xbd5d00>
140a5ade7: 48 8d 05 35 84 07 02        	lea	rax, [rip + 0x2078435]  # 0x142ad3223
140a5adee: 48 89 85 c0 04 00 00        	mov	qword ptr [rbp + 0x4c0], rax
140a5adf5: 48 c7 85 c8 04 00 00 10 00 00 00    	mov	qword ptr [rbp + 0x4c8], 0x10
140a5ae00: 48 8d 95 c0 04 00 00        	lea	rdx, [rbp + 0x4c0]
140a5ae07: 48 8b 8d 98 06 00 00        	mov	rcx, qword ptr [rbp + 0x698]
140a5ae0e: e8 bd e6 18 00              	call	0x140be94d0 <.text+0xbe84d0>
140a5ae13: 0f 10 45 40                 	movups	xmm0, xmmword ptr [rbp + 0x40]
140a5ae17: 0f 29 85 e0 02 00 00        	movaps	xmmword ptr [rbp + 0x2e0], xmm0
140a5ae1e: 89 85 f0 02 00 00           	mov	dword ptr [rbp + 0x2f0], eax
140a5ae24: 44 89 fa                    	mov	edx, r15d
140a5ae27: 48 c1 e2 03                 	shl	rdx, 0x3
140a5ae2b: 48 8d 4d 40                 	lea	rcx, [rbp + 0x40]
140a5ae2f: 45 31 c0                    	xor	r8d, r8d
140a5ae32: e8 e9 9f 17 00              	call	0x140bd4e20 <.text+0xbd3e20>
140a5ae37: 0f 10 45 40                 	movups	xmm0, xmmword ptr [rbp + 0x40]
140a5ae3b: 0f 29 85 00 03 00 00        	movaps	xmmword ptr [rbp + 0x300], xmm0
140a5ae42: 48 8d 8d d0 04 00 00        	lea	rcx, [rbp + 0x4d0]
140a5ae49: 48 8d 95 00 03 00 00        	lea	rdx, [rbp + 0x300]
140a5ae50: e8 ab be 17 00              	call	0x140bd6d00 <.text+0xbd5d00>
140a5ae55: 48 8d 05 d7 83 07 02        	lea	rax, [rip + 0x20783d7]  # 0x142ad3233
140a5ae5c: 48 89 85 c0 04 00 00        	mov	qword ptr [rbp + 0x4c0], rax
140a5ae63: 48 c7 85 c8 04 00 00 0f 00 00 00    	mov	qword ptr [rbp + 0x4c8], 0xf
140a5ae6e: 48 8d 95 c0 04 00 00        	lea	rdx, [rbp + 0x4c0]
140a5ae75: 48 8b 8d 98 06 00 00        	mov	rcx, qword ptr [rbp + 0x698]
140a5ae7c: e8 4f e6 18 00              	call	0x140be94d0 <.text+0xbe84d0>
140a5ae81: 0f 10 45 40                 	movups	xmm0, xmmword ptr [rbp + 0x40]
140a5ae85: 0f 29 85 a0 02 00 00        	movaps	xmmword ptr [rbp + 0x2a0], xmm0
140a5ae8c: 89 85 b0 02 00 00           	mov	dword ptr [rbp + 0x2b0], eax
140a5ae92: 48 8d 4d 40                 	lea	rcx, [rbp + 0x40]
140a5ae96: 4c 89 e2                    	mov	rdx, r12
140a5ae99: 45 31 c0                    	xor	r8d, r8d
140a5ae9c: e8 7f 9f 17 00              	call	0x140bd4e20 <.text+0xbd3e20>
140a5aea1: 0f 10 45 40                 	movups	xmm0, xmmword ptr [rbp + 0x40]
140a5aea5: 0f 29 85 00 03 00 00        	movaps	xmmword ptr [rbp + 0x300], xmm0
140a5aeac: 48 8d 8d d0 04 00 00        	lea	rcx, [rbp + 0x4d0]
140a5aeb3: 48 8d 95 00 03 00 00        	lea	rdx, [rbp + 0x300]
140a5aeba: e8 41 be 17 00              	call	0x140bd6d00 <.text+0xbd5d00>
140a5aebf: 48 8d 05 7c 83 07 02        	lea	rax, [rip + 0x207837c]  # 0x142ad3242
140a5aec6: 48 89 85 c0 04 00 00        	mov	qword ptr [rbp + 0x4c0], rax
140a5aecd: 48 c7 85 c8 04 00 00 11 00 00 00    	mov	qword ptr [rbp + 0x4c8], 0x11
140a5aed8: 48 8d 95 c0 04 00 00        	lea	rdx, [rbp + 0x4c0]
140a5aedf: 48 8b 8d 98 06 00 00        	mov	rcx, qword ptr [rbp + 0x698]
140a5aee6: e8 e5 e5 18 00              	call	0x140be94d0 <.text+0xbe84d0>
140a5aeeb: 0f 10 45 40                 	movups	xmm0, xmmword ptr [rbp + 0x40]
140a5aeef: 0f 29 85 c0 02 00 00        	movaps	xmmword ptr [rbp + 0x2c0], xmm0
140a5aef6: 89 85 d0 02 00 00           	mov	dword ptr [rbp + 0x2d0], eax
140a5aefc: 48 8b 85 90 06 00 00        	mov	rax, qword ptr [rbp + 0x690]
140a5af03: 4c 8d 78 38                 	lea	r15, [rax + 0x38]
140a5af07: 48 ba 2e a7 83 82 b7 4c c9 47       	movabs	rdx, 0x47c94cb78283a72e
140a5af11: 49 b8 9d 3e 32 97 ed 3d 17 8c       	movabs	r8, -0x73e8c21268cdc163
140a5af1b: 4c 89 f9                    	mov	rcx, r15
140a5af1e: e8 cd 27 68 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
140a5af23: 48 85 c0                    	test	rax, rax
140a5af26: 4c 8b 85 90 06 00 00        	mov	r8, qword ptr [rbp + 0x690]
140a5af2d: 0f 84 22 16 00 00           	je	0x140a5c555 <.text+0xa5b555>
140a5af33: 49 39 90 18 02 00 00        	cmp	qword ptr [r8 + 0x218], rdx
140a5af3a: 0f 86 15 16 00 00           	jbe	0x140a5c555 <.text+0xa5b555>
140a5af40: 49 8b 80 10 02 00 00        	mov	rax, qword ptr [r8 + 0x210]
140a5af47: 48 8b 0c d0                 	mov	rcx, qword ptr [rax + 8*rdx]
140a5af4b: 48 85 c9                    	test	rcx, rcx
140a5af4e: 0f 84 01 16 00 00           	je	0x140a5c555 <.text+0xa5b555>
140a5af54: 49 8b 80 e0 01 00 00        	mov	rax, qword ptr [r8 + 0x1e0]
140a5af5b: 48 f7 d1                    	not	rcx
140a5af5e: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
140a5af62: 48 c1 e1 05                 	shl	rcx, 0x5
140a5af66: 48 83 7c 08 20 00           	cmp	qword ptr [rax + rcx + 0x20], 0x0
140a5af6c: 0f 84 e3 15 00 00           	je	0x140a5c555 <.text+0xa5b555>
140a5af72: 48 01 c8                    	add	rax, rcx
140a5af75: 48 8b 78 10                 	mov	rdi, qword ptr [rax + 0x10]
140a5af79: 4c 8b 85 78 06 00 00        	mov	r8, qword ptr [rbp + 0x678]
140a5af80: 4d 85 c0                    	test	r8, r8
140a5af83: 48 89 bd 88 06 00 00        	mov	qword ptr [rbp + 0x688], rdi
140a5af8a: 74 2c                       	je	0x140a5afb8 <.text+0xa59fb8>
140a5af8c: 0f b6 05 76 9c bd 02        	movzx	eax, byte ptr [rip + 0x2bd9c76] # 0x143634c09
140a5af93: ba 04 00 00 00              	mov	edx, 0x4
140a5af98: 4c 89 c1                    	mov	rcx, r8
140a5af9b: e8 70 8c bb ff              	call	0x140613c10 <.text+0x612c10>
140a5afa0: 48 85 c0                    	test	rax, rax
140a5afa3: 0f 84 f4 15 00 00           	je	0x140a5c59d <.text+0xa5b59d>
140a5afa9: 48 89 c6                    	mov	rsi, rax
140a5afac: 4c 89 ef                    	mov	rdi, r13
140a5afaf: 4c 8b 85 78 06 00 00        	mov	r8, qword ptr [rbp + 0x678]
140a5afb6: eb 07                       	jmp	0x140a5afbf <.text+0xa59fbf>
140a5afb8: 31 ff                       	xor	edi, edi
140a5afba: be 04 00 00 00              	mov	esi, 0x4
140a5afbf: 44 0f b6 a5 40 07 00 00     	movzx	r12d, byte ptr [rbp + 0x740]
140a5afc7: 48 8b 85 38 07 00 00        	mov	rax, qword ptr [rbp + 0x738]
140a5afce: 48 8d 58 24                 	lea	rbx, [rax + 0x24]
140a5afd2: 48 89 f1                    	mov	rcx, rsi
140a5afd5: 48 8b 95 80 06 00 00        	mov	rdx, qword ptr [rbp + 0x680]
140a5afdc: e8 1f 8c ec 01              	call	0x142923c00 <.text+0x2922c00>
140a5afe1: 48 89 bd 60 06 00 00        	mov	qword ptr [rbp + 0x660], rdi
140a5afe8: 48 89 b5 68 06 00 00        	mov	qword ptr [rbp + 0x668], rsi
140a5afef: 48 8d 35 0a 84 07 02        	lea	rsi, [rip + 0x207840a]  # 0x142ad3400
140a5aff6: 48 8d bd 60 06 00 00        	lea	rdi, [rbp + 0x660]
140a5affd: eb 13                       	jmp	0x140a5b012 <.text+0xa5a012>
140a5afff: 90                          	nop
140a5b000: 48 8b 85 68 06 00 00        	mov	rax, qword ptr [rbp + 0x668]
140a5b007: 42 c7 04 a8 00 00 00 00     	mov	dword ptr [rax + 4*r13], 0x0
140a5b00f: 49 ff c5                    	inc	r13
140a5b012: 4c 89 ad 70 06 00 00        	mov	qword ptr [rbp + 0x670], r13
140a5b019: 41 f6 c5 03                 	test	r13b, 0x3
140a5b01d: 74 16                       	je	0x140a5b035 <.text+0xa5a035>
140a5b01f: 4c 3b ad 60 06 00 00        	cmp	r13, qword ptr [rbp + 0x660]
140a5b026: 75 d8                       	jne	0x140a5b000 <.text+0xa5a000>
140a5b028: 48 89 f9                    	mov	rcx, rdi
140a5b02b: 48 89 f2                    	mov	rdx, rsi
140a5b02e: e8 6d d3 c4 00              	call	0x1416a83a0 <.text+0x16a73a0>
140a5b033: eb cb                       	jmp	0x140a5b000 <.text+0xa5a000>
140a5b035: b1 01                       	mov	cl, 0x1
140a5b037: 31 c0                       	xor	eax, eax
140a5b039: 48 8b 95 88 06 00 00        	mov	rdx, qword ptr [rbp + 0x688]
140a5b040: f0                          	lock
140a5b041: 0f b0 0a                    	cmpxchg	byte ptr [rdx], cl
140a5b044: 0f 85 cd 14 00 00           	jne	0x140a5c517 <.text+0xa5b517>
140a5b04a: 48 8b 8d 88 06 00 00        	mov	rcx, qword ptr [rbp + 0x688]
140a5b051: e8 1a 6c 22 00              	call	0x140c81c70 <.text+0xc80c70>
140a5b056: 48 8b 85 88 06 00 00        	mov	rax, qword ptr [rbp + 0x688]
140a5b05d: 48 8d 50 08                 	lea	rdx, [rax + 0x8]
140a5b061: 48 8b 85 98 06 00 00        	mov	rax, qword ptr [rbp + 0x698]
140a5b068: 4c 8b 80 90 01 00 00        	mov	r8, qword ptr [rax + 0x190]
140a5b06f: 48 8b bd 68 06 00 00        	mov	rdi, qword ptr [rbp + 0x668]
140a5b076: 48 8b b5 70 06 00 00        	mov	rsi, qword ptr [rbp + 0x670]
140a5b07d: 48 c1 e6 02                 	shl	rsi, 0x2
140a5b081: 48 c7 44 24 20 00 00 00 00  	mov	qword ptr [rsp + 0x20], 0x0
140a5b08a: 48 8d 8d c0 04 00 00        	lea	rcx, [rbp + 0x4c0]
140a5b091: 49 89 f1                    	mov	r9, rsi
140a5b094: e8 a7 76 1a 00              	call	0x140c02740 <.text+0xc01740>
140a5b099: 48 8b 8d c8 04 00 00        	mov	rcx, qword ptr [rbp + 0x4c8]
140a5b0a0: 48 39 f1                    	cmp	rcx, rsi
140a5b0a3: 0f 85 85 14 00 00           	jne	0x140a5c52e <.text+0xa5b52e>
140a5b0a9: 48 8b 8d c0 04 00 00        	mov	rcx, qword ptr [rbp + 0x4c0]
140a5b0b0: 48 89 fa                    	mov	rdx, rdi
140a5b0b3: 49 89 f0                    	mov	r8, rsi
140a5b0b6: e8 45 8b ec 01              	call	0x142923c00 <.text+0x2922c00>
140a5b0bb: 0f 10 85 c0 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x4c0]
140a5b0c2: 0f 10 8d d0 04 00 00        	movups	xmm1, xmmword ptr [rbp + 0x4d0]
140a5b0c9: 0f 10 95 e0 04 00 00        	movups	xmm2, xmmword ptr [rbp + 0x4e0]
140a5b0d0: 0f 10 9d f0 04 00 00        	movups	xmm3, xmmword ptr [rbp + 0x4f0]
140a5b0d7: 0f 29 85 20 02 00 00        	movaps	xmmword ptr [rbp + 0x220], xmm0
140a5b0de: 0f 29 8d 30 02 00 00        	movaps	xmmword ptr [rbp + 0x230], xmm1
140a5b0e5: 0f 29 95 40 02 00 00        	movaps	xmmword ptr [rbp + 0x240], xmm2
140a5b0ec: 0f 29 9d 50 02 00 00        	movaps	xmmword ptr [rbp + 0x250], xmm3
140a5b0f3: 48 8b 85 00 05 00 00        	mov	rax, qword ptr [rbp + 0x500]
140a5b0fa: 48 89 85 60 02 00 00        	mov	qword ptr [rbp + 0x260], rax
140a5b101: c6 85 a6 06 00 00 01        	mov	byte ptr [rbp + 0x6a6], 0x1
140a5b108: 48 8b b5 88 06 00 00        	mov	rsi, qword ptr [rbp + 0x688]
140a5b10f: 48 89 f1                    	mov	rcx, rsi
140a5b112: e8 f9 6c 22 00              	call	0x140c81e10 <.text+0xc80e10>
140a5b117: 31 c9                       	xor	ecx, ecx
140a5b119: b0 01                       	mov	al, 0x1
140a5b11b: f0                          	lock
140a5b11c: 0f b0 0e                    	cmpxchg	byte ptr [rsi], cl
140a5b11f: 0f 85 1a 14 00 00           	jne	0x140a5c53f <.text+0xa5b53f>
140a5b125: 48 8b 85 60 02 00 00        	mov	rax, qword ptr [rbp + 0x260]
140a5b12c: 48 89 85 00 05 00 00        	mov	qword ptr [rbp + 0x500], rax
140a5b133: 0f 28 85 20 02 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x220]
140a5b13a: 0f 28 8d 30 02 00 00        	movaps	xmm1, xmmword ptr [rbp + 0x230]
140a5b141: 0f 28 95 40 02 00 00        	movaps	xmm2, xmmword ptr [rbp + 0x240]
140a5b148: 0f 28 9d 50 02 00 00        	movaps	xmm3, xmmword ptr [rbp + 0x250]
140a5b14f: 0f 29 9d f0 04 00 00        	movaps	xmmword ptr [rbp + 0x4f0], xmm3
140a5b156: 0f 29 95 e0 04 00 00        	movaps	xmmword ptr [rbp + 0x4e0], xmm2
140a5b15d: 0f 29 8d d0 04 00 00        	movaps	xmmword ptr [rbp + 0x4d0], xmm1
140a5b164: 0f 29 85 c0 04 00 00        	movaps	xmmword ptr [rbp + 0x4c0], xmm0
140a5b16b: 48 8b 85 98 06 00 00        	mov	rax, qword ptr [rbp + 0x698]
140a5b172: 4c 8b 80 90 01 00 00        	mov	r8, qword ptr [rax + 0x190]
140a5b179: c6 85 a6 06 00 00 00        	mov	byte ptr [rbp + 0x6a6], 0x0
140a5b180: 48 8d 4d e0                 	lea	rcx, [rbp - 0x20]
140a5b184: 48 8d 95 c0 04 00 00        	lea	rdx, [rbp + 0x4c0]
140a5b18b: e8 20 71 1a 00              	call	0x140c022b0 <.text+0xc012b0>
140a5b190: c6 85 a7 06 00 00 01        	mov	byte ptr [rbp + 0x6a7], 0x1
140a5b197: 48 8b 85 98 06 00 00        	mov	rax, qword ptr [rbp + 0x698]
140a5b19e: 48 89 44 24 30              	mov	qword ptr [rsp + 0x30], rax
140a5b1a3: 48 8b 85 90 06 00 00        	mov	rax, qword ptr [rbp + 0x690]
140a5b1aa: 48 89 44 24 28              	mov	qword ptr [rsp + 0x28], rax
140a5b1af: 48 c7 44 24 20 19 00 00 00  	mov	qword ptr [rsp + 0x20], 0x19
140a5b1b8: 48 8d 15 b1 80 07 02        	lea	rdx, [rip + 0x20780b1]  # 0x142ad3270
140a5b1bf: 4c 8d 0d ba 80 07 02        	lea	r9, [rip + 0x20780ba]   # 0x142ad3280
140a5b1c6: 48 8d 8d c0 04 00 00        	lea	rcx, [rbp + 0x4c0]
140a5b1cd: 41 b8 10 00 00 00           	mov	r8d, 0x10
140a5b1d3: e8 c8 80 ba 00              	call	0x1416032a0 <.text+0x16022a0>
140a5b1d8: c6 85 a7 06 00 00 01        	mov	byte ptr [rbp + 0x6a7], 0x1
140a5b1df: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a5b1e6: 48 8d 95 c0 04 00 00        	lea	rdx, [rbp + 0x4c0]
140a5b1ed: 49 b8 7d 9e 00 1d 50 10 67 ce       	movabs	r8, -0x3198efafe2ff6183
140a5b1f7: 4c 8d 8d e0 02 00 00        	lea	r9, [rbp + 0x2e0]
140a5b1fe: e8 5d 2b f1 ff              	call	0x14096dd60 <.text+0x96cd60>
140a5b203: c6 85 a7 06 00 00 01        	mov	byte ptr [rbp + 0x6a7], 0x1
140a5b20a: 48 8d 8d c0 04 00 00        	lea	rcx, [rbp + 0x4c0]
140a5b211: 48 8d 95 00 03 00 00        	lea	rdx, [rbp + 0x300]
140a5b218: 49 b8 b4 e4 61 20 b6 4c 0a 55       	movabs	r8, 0x550a4cb62061e4b4
140a5b222: 4c 8d 8d c0 02 00 00        	lea	r9, [rbp + 0x2c0]
140a5b229: e8 32 2b f1 ff              	call	0x14096dd60 <.text+0x96cd60>
140a5b22e: 0f 10 45 e0                 	movups	xmm0, xmmword ptr [rbp - 0x20]
140a5b232: 0f 10 4d f0                 	movups	xmm1, xmmword ptr [rbp - 0x10]
140a5b236: 0f 10 55 00                 	movups	xmm2, xmmword ptr [rbp]
140a5b23a: 0f 10 5d 10                 	movups	xmm3, xmmword ptr [rbp + 0x10]
140a5b23e: 0f 29 5d 70                 	movaps	xmmword ptr [rbp + 0x70], xmm3
140a5b242: 0f 29 55 60                 	movaps	xmmword ptr [rbp + 0x60], xmm2
140a5b246: 0f 29 4d 50                 	movaps	xmmword ptr [rbp + 0x50], xmm1
140a5b24a: 0f 29 45 40                 	movaps	xmmword ptr [rbp + 0x40], xmm0
140a5b24e: c6 85 a7 06 00 00 00        	mov	byte ptr [rbp + 0x6a7], 0x0
140a5b255: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a5b25c: 48 8d 95 c0 04 00 00        	lea	rdx, [rbp + 0x4c0]
140a5b263: 49 b8 f4 dd c0 4a a7 29 62 f7       	movabs	r8, -0x89dd658b53f220c
140a5b26d: 4c 8d 4d 40                 	lea	r9, [rbp + 0x40]
140a5b271: e8 ca 32 f1 ff              	call	0x14096e540 <.text+0x96d540>
140a5b276: 0f b6 05 8c 99 bd 02        	movzx	eax, byte ptr [rip + 0x2bd998c] # 0x143634c09
140a5b27d: b9 04 00 00 00              	mov	ecx, 0x4
140a5b282: ba 01 00 00 00              	mov	edx, 0x1
140a5b287: e8 84 89 bb ff              	call	0x140613c10 <.text+0x612c10>
140a5b28c: 48 85 c0                    	test	rax, rax
140a5b28f: 0f 84 f0 12 00 00           	je	0x140a5c585 <.text+0xa5b585>
140a5b295: 48 89 c6                    	mov	rsi, rax
140a5b298: 44 89 30                    	mov	dword ptr [rax], r14d
140a5b29b: 48 8b 95 48 03 00 00        	mov	rdx, qword ptr [rbp + 0x348]
140a5b2a2: 48 85 d2                    	test	rdx, rdx
140a5b2a5: 74 12                       	je	0x140a5b2b9 <.text+0xa5a2b9>
140a5b2a7: 48 8b 8d 50 03 00 00        	mov	rcx, qword ptr [rbp + 0x350]
140a5b2ae: 41 b8 01 00 00 00           	mov	r8d, 0x1
140a5b2b4: e8 67 89 bb ff              	call	0x140613c20 <.text+0x612c20>
140a5b2b9: 48 c7 85 48 03 00 00 04 00 00 00    	mov	qword ptr [rbp + 0x348], 0x4
140a5b2c4: 48 89 b5 50 03 00 00        	mov	qword ptr [rbp + 0x350], rsi
140a5b2cb: 48 c7 85 58 03 00 00 04 00 00 00    	mov	qword ptr [rbp + 0x358], 0x4
140a5b2d6: 48 8d 8d c0 04 00 00        	lea	rcx, [rbp + 0x4c0]
140a5b2dd: 48 8d 95 00 03 00 00        	lea	rdx, [rbp + 0x300]
140a5b2e4: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a5b2ea: e8 11 89 ec 01              	call	0x142923c00 <.text+0x2922c00>
140a5b2ef: 0f 10 85 60 05 00 00        	movups	xmm0, xmmword ptr [rbp + 0x560]
140a5b2f6: 0f 29 85 80 00 00 00        	movaps	xmmword ptr [rbp + 0x80], xmm0
140a5b2fd: 0f 10 85 50 05 00 00        	movups	xmm0, xmmword ptr [rbp + 0x550]
140a5b304: 0f 29 45 70                 	movaps	xmmword ptr [rbp + 0x70], xmm0
140a5b308: 0f 10 85 40 05 00 00        	movups	xmm0, xmmword ptr [rbp + 0x540]
140a5b30f: 0f 29 45 60                 	movaps	xmmword ptr [rbp + 0x60], xmm0
140a5b313: 0f 10 85 30 05 00 00        	movups	xmm0, xmmword ptr [rbp + 0x530]
140a5b31a: 0f 29 45 50                 	movaps	xmmword ptr [rbp + 0x50], xmm0
140a5b31e: 0f 10 85 20 05 00 00        	movups	xmm0, xmmword ptr [rbp + 0x520]
140a5b325: 0f 29 45 40                 	movaps	xmmword ptr [rbp + 0x40], xmm0
140a5b329: 48 8b 85 f8 05 00 00        	mov	rax, qword ptr [rbp + 0x5f8]
140a5b330: 48 8b 8d 40 06 00 00        	mov	rcx, qword ptr [rbp + 0x640]
140a5b337: 48 8b 95 d0 04 00 00        	mov	rdx, qword ptr [rbp + 0x4d0]
140a5b33e: 48 89 95 10 03 00 00        	mov	qword ptr [rbp + 0x310], rdx
140a5b345: 0f 10 85 c0 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x4c0]
140a5b34c: 0f 29 85 00 03 00 00        	movaps	xmmword ptr [rbp + 0x300], xmm0
140a5b353: 0f 10 85 d8 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x4d8]
140a5b35a: 0f 29 85 30 03 00 00        	movaps	xmmword ptr [rbp + 0x330], xmm0
140a5b361: 48 8b 95 e8 04 00 00        	mov	rdx, qword ptr [rbp + 0x4e8]
140a5b368: 48 89 95 40 03 00 00        	mov	qword ptr [rbp + 0x340], rdx
140a5b36f: 0f 10 85 f0 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x4f0]
140a5b376: 0f 11 85 18 03 00 00        	movups	xmmword ptr [rbp + 0x318], xmm0
140a5b37d: 48 8b 95 00 05 00 00        	mov	rdx, qword ptr [rbp + 0x500]
140a5b384: 48 89 95 28 03 00 00        	mov	qword ptr [rbp + 0x328], rdx
140a5b38b: 0f 10 85 70 05 00 00        	movups	xmm0, xmmword ptr [rbp + 0x570]
140a5b392: 0f 10 8d 80 05 00 00        	movups	xmm1, xmmword ptr [rbp + 0x580]
140a5b399: 0f 10 95 90 05 00 00        	movups	xmm2, xmmword ptr [rbp + 0x590]
140a5b3a0: 0f 10 9d a0 05 00 00        	movups	xmm3, xmmword ptr [rbp + 0x5a0]
140a5b3a7: 0f 11 85 78 03 00 00        	movups	xmmword ptr [rbp + 0x378], xmm0
140a5b3ae: 0f 11 8d 88 03 00 00        	movups	xmmword ptr [rbp + 0x388], xmm1
140a5b3b5: 0f 11 95 98 03 00 00        	movups	xmmword ptr [rbp + 0x398], xmm2
140a5b3bc: 0f 11 9d a8 03 00 00        	movups	xmmword ptr [rbp + 0x3a8], xmm3
140a5b3c3: 0f 10 85 b0 05 00 00        	movups	xmm0, xmmword ptr [rbp + 0x5b0]
140a5b3ca: 0f 11 85 b8 03 00 00        	movups	xmmword ptr [rbp + 0x3b8], xmm0
140a5b3d1: 0f 10 85 c0 05 00 00        	movups	xmm0, xmmword ptr [rbp + 0x5c0]
140a5b3d8: 0f 11 85 c8 03 00 00        	movups	xmmword ptr [rbp + 0x3c8], xmm0
140a5b3df: 48 8b 95 18 05 00 00        	mov	rdx, qword ptr [rbp + 0x518]
140a5b3e6: 48 89 95 58 03 00 00        	mov	qword ptr [rbp + 0x358], rdx
140a5b3ed: 0f 10 85 08 05 00 00        	movups	xmm0, xmmword ptr [rbp + 0x508]
140a5b3f4: 0f 11 85 48 03 00 00        	movups	xmmword ptr [rbp + 0x348], xmm0
140a5b3fb: 48 89 8d e0 03 00 00        	mov	qword ptr [rbp + 0x3e0], rcx
140a5b402: 48 89 85 d8 03 00 00        	mov	qword ptr [rbp + 0x3d8], rax
140a5b409: c7 85 60 03 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x360], 0x1
140a5b413: 44 89 b5 64 03 00 00        	mov	dword ptr [rbp + 0x364], r14d
140a5b41a: 48 b8 01 00 00 00 01 00 00 00       	movabs	rax, 0x100000001
140a5b424: 48 89 85 68 03 00 00        	mov	qword ptr [rbp + 0x368], rax
140a5b42b: 48 8d 4d 40                 	lea	rcx, [rbp + 0x40]
140a5b42f: 48 8d b5 00 03 00 00        	lea	rsi, [rbp + 0x300]
140a5b436: 48 89 f2                    	mov	rdx, rsi
140a5b439: e8 92 20 f8 ff              	call	0x1409dd4d0 <.text+0x9dc4d0>
140a5b43e: 80 bd f6 05 00 00 08        	cmp	byte ptr [rbp + 0x5f6], 0x8
140a5b445: 74 13                       	je	0x140a5b45a <.text+0xa5a45a>
140a5b447: 48 8d 8d e0 05 00 00        	lea	rcx, [rbp + 0x5e0]
140a5b44e: c6 85 a7 06 00 00 00        	mov	byte ptr [rbp + 0x6a7], 0x0
140a5b455: e8 56 70 18 00              	call	0x140be24b0 <.text+0xbe14b0>
140a5b45a: 48 8b 95 60 06 00 00        	mov	rdx, qword ptr [rbp + 0x660]
140a5b461: 48 85 d2                    	test	rdx, rdx
140a5b464: 74 16                       	je	0x140a5b47c <.text+0xa5a47c>
140a5b466: 48 8b 8d 68 06 00 00        	mov	rcx, qword ptr [rbp + 0x668]
140a5b46d: 48 c1 e2 02                 	shl	rdx, 0x2
140a5b471: 41 b8 04 00 00 00           	mov	r8d, 0x4
140a5b477: e8 a4 87 bb ff              	call	0x140613c20 <.text+0x612c20>
140a5b47c: 48 8b 85 98 06 00 00        	mov	rax, qword ptr [rbp + 0x698]
140a5b483: 48 89 44 24 30              	mov	qword ptr [rsp + 0x30], rax
140a5b488: 48 8b 85 90 06 00 00        	mov	rax, qword ptr [rbp + 0x690]
140a5b48f: 48 89 44 24 28              	mov	qword ptr [rsp + 0x28], rax
140a5b494: 48 c7 44 24 20 14 00 00 00  	mov	qword ptr [rsp + 0x20], 0x14
140a5b49d: 48 8d 15 f5 7d 07 02        	lea	rdx, [rip + 0x2077df5]  # 0x142ad3299
140a5b4a4: 4c 8d 0d f9 7d 07 02        	lea	r9, [rip + 0x2077df9]   # 0x142ad32a4
140a5b4ab: 48 8d 8d c0 04 00 00        	lea	rcx, [rbp + 0x4c0]
140a5b4b2: 41 b8 0b 00 00 00           	mov	r8d, 0xb
140a5b4b8: e8 e3 7d ba 00              	call	0x1416032a0 <.text+0x16022a0>
140a5b4bd: 48 89 5d 40                 	mov	qword ptr [rbp + 0x40], rbx
140a5b4c1: c7 85 00 03 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x300], 0x2
140a5b4cb: 48 89 74 24 20              	mov	qword ptr [rsp + 0x20], rsi
140a5b4d0: 48 8d 4d 40                 	lea	rcx, [rbp + 0x40]
140a5b4d4: 48 8d 95 c0 04 00 00        	lea	rdx, [rbp + 0x4c0]
140a5b4db: 49 b8 c4 08 4d 06 c2 bd c9 a0       	movabs	r8, -0x5f36423df9b2f73c
140a5b4e5: 41 b1 19                    	mov	r9b, 0x19
140a5b4e8: e8 53 2d e9 ff              	call	0x1408ee240 <.text+0x8ed240>
140a5b4ed: 48 8d bd c0 04 00 00        	lea	rdi, [rbp + 0x4c0]
140a5b4f4: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a5b4fa: 48 89 f1                    	mov	rcx, rsi
140a5b4fd: 48 89 fa                    	mov	rdx, rdi
140a5b500: e8 fb 86 ec 01              	call	0x142923c00 <.text+0x2922c00>
140a5b505: 48 8d 45 bc                 	lea	rax, [rbp - 0x44]
140a5b509: 48 89 45 40                 	mov	qword ptr [rbp + 0x40], rax
140a5b50d: c7 85 c0 04 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x4c0], 0x2
140a5b517: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
140a5b51c: 48 8d 4d 40                 	lea	rcx, [rbp + 0x40]
140a5b520: 49 b8 ae d2 41 c8 fd d6 de 84       	movabs	r8, -0x7b21290237be2d52
140a5b52a: 48 89 f2                    	mov	rdx, rsi
140a5b52d: 41 b1 32                    	mov	r9b, 0x32
140a5b530: e8 cb 27 e9 ff              	call	0x1408edd00 <.text+0x8ecd00>
140a5b535: 48 8d b5 c0 04 00 00        	lea	rsi, [rbp + 0x4c0]
140a5b53c: 48 8d 95 00 03 00 00        	lea	rdx, [rbp + 0x300]
140a5b543: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a5b549: 48 89 f1                    	mov	rcx, rsi
140a5b54c: e8 af 86 ec 01              	call	0x142923c00 <.text+0x2922c00>
140a5b551: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a5b558: 49 b8 7d 9e 00 1d 50 10 67 ce       	movabs	r8, -0x3198efafe2ff6183
140a5b562: 4c 8d 8d e0 02 00 00        	lea	r9, [rbp + 0x2e0]
140a5b569: 48 89 f2                    	mov	rdx, rsi
140a5b56c: e8 4f 25 f1 ff              	call	0x14096dac0 <.text+0x96cac0>
140a5b571: 48 8d 8d c0 04 00 00        	lea	rcx, [rbp + 0x4c0]
140a5b578: 48 8d 95 00 03 00 00        	lea	rdx, [rbp + 0x300]
140a5b57f: 49 b8 b4 e4 61 20 b6 4c 0a 55       	movabs	r8, 0x550a4cb62061e4b4
140a5b589: 4c 8d 8d c0 02 00 00        	lea	r9, [rbp + 0x2c0]
140a5b590: e8 cb 27 f1 ff              	call	0x14096dd60 <.text+0x96cd60>
140a5b595: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a5b59c: 48 8d 95 c0 04 00 00        	lea	rdx, [rbp + 0x4c0]
140a5b5a3: 49 b8 ff aa 4a 65 54 24 7d b9       	movabs	r8, -0x4682dbab9ab55501
140a5b5ad: 4c 8d 8d a0 02 00 00        	lea	r9, [rbp + 0x2a0]
140a5b5b4: e8 a7 27 f1 ff              	call	0x14096dd60 <.text+0x96cd60>
140a5b5b9: 0f 10 85 a0 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x3a0]
140a5b5c0: 0f 29 85 80 00 00 00        	movaps	xmmword ptr [rbp + 0x80], xmm0
140a5b5c7: 0f 10 85 90 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x390]
140a5b5ce: 0f 29 45 70                 	movaps	xmmword ptr [rbp + 0x70], xmm0
140a5b5d2: 0f 10 85 80 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x380]
140a5b5d9: 0f 29 45 60                 	movaps	xmmword ptr [rbp + 0x60], xmm0
140a5b5dd: 0f 10 85 70 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x370]
140a5b5e4: 0f 29 45 50                 	movaps	xmmword ptr [rbp + 0x50], xmm0
140a5b5e8: 0f 10 85 60 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x360]
140a5b5ef: 0f 29 45 40                 	movaps	xmmword ptr [rbp + 0x40], xmm0
140a5b5f3: 48 8b 85 38 04 00 00        	mov	rax, qword ptr [rbp + 0x438]
140a5b5fa: 48 8b 8d 80 04 00 00        	mov	rcx, qword ptr [rbp + 0x480]
140a5b601: 48 8b 95 10 03 00 00        	mov	rdx, qword ptr [rbp + 0x310]
140a5b608: 48 89 95 d0 04 00 00        	mov	qword ptr [rbp + 0x4d0], rdx
140a5b60f: 0f 10 85 00 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x300]
140a5b616: 0f 29 85 c0 04 00 00        	movaps	xmmword ptr [rbp + 0x4c0], xmm0
140a5b61d: 0f 10 85 18 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x318]
140a5b624: 0f 29 85 f0 04 00 00        	movaps	xmmword ptr [rbp + 0x4f0], xmm0
140a5b62b: 48 8b 95 28 03 00 00        	mov	rdx, qword ptr [rbp + 0x328]
140a5b632: 48 89 95 00 05 00 00        	mov	qword ptr [rbp + 0x500], rdx
140a5b639: 0f 10 85 30 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x330]
140a5b640: 0f 11 85 d8 04 00 00        	movups	xmmword ptr [rbp + 0x4d8], xmm0
140a5b647: 48 8b 95 40 03 00 00        	mov	rdx, qword ptr [rbp + 0x340]
140a5b64e: 48 89 95 e8 04 00 00        	mov	qword ptr [rbp + 0x4e8], rdx
140a5b655: 0f 10 85 b0 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x3b0]
140a5b65c: 0f 10 8d c0 03 00 00        	movups	xmm1, xmmword ptr [rbp + 0x3c0]
140a5b663: 0f 10 95 d0 03 00 00        	movups	xmm2, xmmword ptr [rbp + 0x3d0]
140a5b66a: 0f 10 9d e0 03 00 00        	movups	xmm3, xmmword ptr [rbp + 0x3e0]
140a5b671: 0f 11 85 38 05 00 00        	movups	xmmword ptr [rbp + 0x538], xmm0
140a5b678: 0f 11 8d 48 05 00 00        	movups	xmmword ptr [rbp + 0x548], xmm1
140a5b67f: 0f 11 95 58 05 00 00        	movups	xmmword ptr [rbp + 0x558], xmm2
140a5b686: 0f 11 9d 68 05 00 00        	movups	xmmword ptr [rbp + 0x568], xmm3
140a5b68d: 0f 10 85 f0 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x3f0]
140a5b694: 0f 11 85 78 05 00 00        	movups	xmmword ptr [rbp + 0x578], xmm0
140a5b69b: 0f 10 85 00 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x400]
140a5b6a2: 0f 11 85 88 05 00 00        	movups	xmmword ptr [rbp + 0x588], xmm0
140a5b6a9: 48 8b 95 58 03 00 00        	mov	rdx, qword ptr [rbp + 0x358]
140a5b6b0: 48 89 95 18 05 00 00        	mov	qword ptr [rbp + 0x518], rdx
140a5b6b7: 0f 10 85 48 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x348]
140a5b6be: 0f 11 85 08 05 00 00        	movups	xmmword ptr [rbp + 0x508], xmm0
140a5b6c5: 48 89 8d a0 05 00 00        	mov	qword ptr [rbp + 0x5a0], rcx
140a5b6cc: 48 89 85 98 05 00 00        	mov	qword ptr [rbp + 0x598], rax
140a5b6d3: c7 85 20 05 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x520], 0x1
140a5b6dd: 48 8b 45 20                 	mov	rax, qword ptr [rbp + 0x20]
140a5b6e1: 89 85 24 05 00 00           	mov	dword ptr [rbp + 0x524], eax
140a5b6e7: 48 8b 45 28                 	mov	rax, qword ptr [rbp + 0x28]
140a5b6eb: 89 85 28 05 00 00           	mov	dword ptr [rbp + 0x528], eax
140a5b6f1: c7 85 2c 05 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x52c], 0x1
140a5b6fb: 48 8d 4d 40                 	lea	rcx, [rbp + 0x40]
140a5b6ff: 48 8d 95 c0 04 00 00        	lea	rdx, [rbp + 0x4c0]
140a5b706: e8 c5 1d f8 ff              	call	0x1409dd4d0 <.text+0x9dc4d0>
140a5b70b: 80 bd 36 04 00 00 08        	cmp	byte ptr [rbp + 0x436], 0x8
140a5b712: 74 0c                       	je	0x140a5b720 <.text+0xa5a720>
140a5b714: 48 8d 8d 20 04 00 00        	lea	rcx, [rbp + 0x420]
140a5b71b: e8 90 6d 18 00              	call	0x140be24b0 <.text+0xbe14b0>
140a5b720: 48 8d 4d 40                 	lea	rcx, [rbp + 0x40]
140a5b724: 48 8b 55 30                 	mov	rdx, qword ptr [rbp + 0x30]
140a5b728: 41 b8 61 00 00 00           	mov	r8d, 0x61
140a5b72e: e8 6d 9b 17 00              	call	0x140bd52a0 <.text+0xbd42a0>
140a5b733: 0f 10 45 40                 	movups	xmm0, xmmword ptr [rbp + 0x40]
140a5b737: 0f 10 4d 50                 	movups	xmm1, xmmword ptr [rbp + 0x50]
140a5b73b: 0f 29 8d 10 03 00 00        	movaps	xmmword ptr [rbp + 0x310], xmm1
140a5b742: 0f 29 85 00 03 00 00        	movaps	xmmword ptr [rbp + 0x300], xmm0
140a5b749: 48 8d 8d d0 04 00 00        	lea	rcx, [rbp + 0x4d0]
140a5b750: 48 8d 95 00 03 00 00        	lea	rdx, [rbp + 0x300]
140a5b757: e8 84 b5 17 00              	call	0x140bd6ce0 <.text+0xbd5ce0>
140a5b75c: 48 8d 05 55 7b 07 02        	lea	rax, [rip + 0x2077b55]  # 0x142ad32b8
140a5b763: 48 89 85 c0 04 00 00        	mov	qword ptr [rbp + 0x4c0], rax
140a5b76a: 48 c7 85 c8 04 00 00 0b 00 00 00    	mov	qword ptr [rbp + 0x4c8], 0xb
140a5b775: 48 8d 95 c0 04 00 00        	lea	rdx, [rbp + 0x4c0]
140a5b77c: 48 8b 8d 98 06 00 00        	mov	rcx, qword ptr [rbp + 0x698]
140a5b783: e8 48 dd 18 00              	call	0x140be94d0 <.text+0xbe84d0>
140a5b788: 0f 10 45 40                 	movups	xmm0, xmmword ptr [rbp + 0x40]
140a5b78c: 0f 10 4d 50                 	movups	xmm1, xmmword ptr [rbp + 0x50]
140a5b790: 0f 11 85 f4 01 00 00        	movups	xmmword ptr [rbp + 0x1f4], xmm0
140a5b797: 0f 11 8d 04 02 00 00        	movups	xmmword ptr [rbp + 0x204], xmm1
140a5b79e: 89 85 f0 01 00 00           	mov	dword ptr [rbp + 0x1f0], eax
140a5b7a4: 48 8d 4d 40                 	lea	rcx, [rbp + 0x40]
140a5b7a8: 48 8b 55 38                 	mov	rdx, qword ptr [rbp + 0x38]
140a5b7ac: 41 b8 61 00 00 00           	mov	r8d, 0x61
140a5b7b2: e8 e9 9a 17 00              	call	0x140bd52a0 <.text+0xbd42a0>
140a5b7b7: 0f 10 45 40                 	movups	xmm0, xmmword ptr [rbp + 0x40]
140a5b7bb: 0f 10 4d 50                 	movups	xmm1, xmmword ptr [rbp + 0x50]
140a5b7bf: 0f 29 8d 10 03 00 00        	movaps	xmmword ptr [rbp + 0x310], xmm1
140a5b7c6: 0f 29 85 00 03 00 00        	movaps	xmmword ptr [rbp + 0x300], xmm0
140a5b7cd: 48 8d 8d d0 04 00 00        	lea	rcx, [rbp + 0x4d0]
140a5b7d4: 48 8d 95 00 03 00 00        	lea	rdx, [rbp + 0x300]
140a5b7db: e8 00 b5 17 00              	call	0x140bd6ce0 <.text+0xbd5ce0>
140a5b7e0: 48 8d 05 dc 7a 07 02        	lea	rax, [rip + 0x2077adc]  # 0x142ad32c3
140a5b7e7: 48 89 85 c0 04 00 00        	mov	qword ptr [rbp + 0x4c0], rax
140a5b7ee: 48 c7 85 c8 04 00 00 0a 00 00 00    	mov	qword ptr [rbp + 0x4c8], 0xa
140a5b7f9: 48 8d 95 c0 04 00 00        	lea	rdx, [rbp + 0x4c0]
140a5b800: 48 8b 8d 98 06 00 00        	mov	rcx, qword ptr [rbp + 0x698]
140a5b807: e8 c4 dc 18 00              	call	0x140be94d0 <.text+0xbe84d0>
140a5b80c: 0f 10 45 40                 	movups	xmm0, xmmword ptr [rbp + 0x40]
140a5b810: 0f 10 4d 50                 	movups	xmm1, xmmword ptr [rbp + 0x50]
140a5b814: 0f 11 45 e4                 	movups	xmmword ptr [rbp - 0x1c], xmm0
140a5b818: 0f 11 4d f4                 	movups	xmmword ptr [rbp - 0xc], xmm1
140a5b81c: 89 45 e0                    	mov	dword ptr [rbp - 0x20], eax
140a5b81f: 48 8b 85 98 06 00 00        	mov	rax, qword ptr [rbp + 0x698]
140a5b826: 48 89 44 24 30              	mov	qword ptr [rsp + 0x30], rax
140a5b82b: 48 8b 85 90 06 00 00        	mov	rax, qword ptr [rbp + 0x690]
140a5b832: 48 89 44 24 28              	mov	qword ptr [rsp + 0x28], rax
140a5b837: 48 c7 44 24 20 1a 00 00 00  	mov	qword ptr [rsp + 0x20], 0x1a
140a5b840: 48 8d 15 86 7a 07 02        	lea	rdx, [rip + 0x2077a86]  # 0x142ad32cd
140a5b847: 4c 8d 0d 90 7a 07 02        	lea	r9, [rip + 0x2077a90]   # 0x142ad32de
140a5b84e: 48 8d 8d c0 04 00 00        	lea	rcx, [rbp + 0x4c0]
140a5b855: 41 b8 11 00 00 00           	mov	r8d, 0x11
140a5b85b: e8 40 7a ba 00              	call	0x1416032a0 <.text+0x16022a0>
140a5b860: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a5b867: 48 8d 95 c0 04 00 00        	lea	rdx, [rbp + 0x4c0]
140a5b86e: 49 b8 7d 9e 00 1d 50 10 67 ce       	movabs	r8, -0x3198efafe2ff6183
140a5b878: 4c 8d 8d e0 02 00 00        	lea	r9, [rbp + 0x2e0]
140a5b87f: e8 3c 22 f1 ff              	call	0x14096dac0 <.text+0x96cac0>
140a5b884: 48 8d 8d c0 04 00 00        	lea	rcx, [rbp + 0x4c0]
140a5b88b: 48 8d b5 00 03 00 00        	lea	rsi, [rbp + 0x300]
140a5b892: 49 b8 ff aa 4a 65 54 24 7d b9       	movabs	r8, -0x4682dbab9ab55501
140a5b89c: 4c 8d 8d a0 02 00 00        	lea	r9, [rbp + 0x2a0]
140a5b8a3: 48 89 f2                    	mov	rdx, rsi
140a5b8a6: e8 15 22 f1 ff              	call	0x14096dac0 <.text+0x96cac0>
140a5b8ab: 48 8d 85 f0 01 00 00        	lea	rax, [rbp + 0x1f0]
140a5b8b2: 48 89 45 40                 	mov	qword ptr [rbp + 0x40], rax
140a5b8b6: c7 85 00 03 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x300], 0x2
140a5b8c0: 48 89 74 24 20              	mov	qword ptr [rsp + 0x20], rsi
140a5b8c5: 48 8d 4d 40                 	lea	rcx, [rbp + 0x40]
140a5b8c9: 48 8d 95 c0 04 00 00        	lea	rdx, [rbp + 0x4c0]
140a5b8d0: 49 b8 c8 8b 41 02 02 79 81 b9       	movabs	r8, -0x467e86fdfdbe7438
140a5b8da: 41 b1 32                    	mov	r9b, 0x32
140a5b8dd: e8 1e 24 e9 ff              	call	0x1408edd00 <.text+0x8ecd00>
140a5b8e2: 48 8d 95 c0 04 00 00        	lea	rdx, [rbp + 0x4c0]
140a5b8e9: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a5b8ef: 48 89 f1                    	mov	rcx, rsi
140a5b8f2: e8 09 83 ec 01              	call	0x142923c00 <.text+0x2922c00>
140a5b8f7: 48 8d 95 c0 02 00 00        	lea	rdx, [rbp + 0x2c0]
140a5b8fe: 41 b8 60 00 00 00           	mov	r8d, 0x60
140a5b904: 48 89 f1                    	mov	rcx, rsi
140a5b907: e8 74 2f f1 ff              	call	0x14096e880 <.text+0x96d880>
140a5b90c: 48 ba b9 75 23 64 09 2e 14 4a       	movabs	rdx, 0x4a142e09642375b9
140a5b916: 49 b8 7f 84 8c fa 4c 15 f6 43       	movabs	r8, 0x43f6154cfa8c847f
140a5b920: 4c 89 f9                    	mov	rcx, r15
140a5b923: e8 c8 1d 68 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
140a5b928: 48 85 c0                    	test	rax, rax
140a5b92b: 4c 8b 85 90 06 00 00        	mov	r8, qword ptr [rbp + 0x690]
140a5b932: 0f 84 2b 0c 00 00           	je	0x140a5c563 <.text+0xa5b563>
140a5b938: 49 39 90 18 02 00 00        	cmp	qword ptr [r8 + 0x218], rdx
140a5b93f: 0f 86 1e 0c 00 00           	jbe	0x140a5c563 <.text+0xa5b563>
140a5b945: 49 8b 80 10 02 00 00        	mov	rax, qword ptr [r8 + 0x210]
140a5b94c: 48 8b 0c d0                 	mov	rcx, qword ptr [rax + 8*rdx]
140a5b950: 48 85 c9                    	test	rcx, rcx
140a5b953: 0f 84 0a 0c 00 00           	je	0x140a5c563 <.text+0xa5b563>
140a5b959: 49 8b 80 e0 01 00 00        	mov	rax, qword ptr [r8 + 0x1e0]
140a5b960: 48 f7 d1                    	not	rcx
140a5b963: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
140a5b967: 48 c1 e1 05                 	shl	rcx, 0x5
140a5b96b: 48 83 7c 08 20 00           	cmp	qword ptr [rax + rcx + 0x20], 0x0
140a5b971: 0f 84 ec 0b 00 00           	je	0x140a5c563 <.text+0xa5b563>
140a5b977: 48 01 c8                    	add	rax, rcx
140a5b97a: 48 8b 40 10                 	mov	rax, qword ptr [rax + 0x10]
140a5b97e: 0f b6 88 c0 00 00 00        	movzx	ecx, byte ptr [rax + 0xc0]
140a5b985: c1 e1 06                    	shl	ecx, 0x6
140a5b988: f3 0f 10 74 08 30           	movss	xmm6, dword ptr [rax + rcx + 0x30]
140a5b98e: 48 8b 85 98 06 00 00        	mov	rax, qword ptr [rbp + 0x698]
140a5b995: 48 89 44 24 28              	mov	qword ptr [rsp + 0x28], rax
140a5b99a: 4c 89 44 24 20              	mov	qword ptr [rsp + 0x20], r8
140a5b99f: 48 8d 15 12 79 07 02        	lea	rdx, [rip + 0x2077912]  # 0x142ad32b8
140a5b9a6: 48 8d 8d c0 04 00 00        	lea	rcx, [rbp + 0x4c0]
140a5b9ad: 41 b8 0b 00 00 00           	mov	r8d, 0xb
140a5b9b3: 45 89 e1                    	mov	r9d, r12d
140a5b9b6: e8 a5 42 ba 00              	call	0x1415ffc60 <.text+0x15fec60>
140a5b9bb: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a5b9c2: 48 8d 95 c0 04 00 00        	lea	rdx, [rbp + 0x4c0]
140a5b9c9: 49 b8 7d 9e 00 1d 50 10 67 ce       	movabs	r8, -0x3198efafe2ff6183
140a5b9d3: 4c 8d 8d e0 02 00 00        	lea	r9, [rbp + 0x2e0]
140a5b9da: e8 e1 20 f1 ff              	call	0x14096dac0 <.text+0x96cac0>
140a5b9df: 48 8d 8d c0 04 00 00        	lea	rcx, [rbp + 0x4c0]
140a5b9e6: 49 b8 ff aa 4a 65 54 24 7d b9       	movabs	r8, -0x4682dbab9ab55501
140a5b9f0: 4c 8d 8d a0 02 00 00        	lea	r9, [rbp + 0x2a0]
140a5b9f7: 48 89 f2                    	mov	rdx, rsi
140a5b9fa: e8 c1 20 f1 ff              	call	0x14096dac0 <.text+0x96cac0>
140a5b9ff: 48 8b 85 b0 04 00 00        	mov	rax, qword ptr [rbp + 0x4b0]
140a5ba06: 48 89 45 40                 	mov	qword ptr [rbp + 0x40], rax
140a5ba0a: c7 85 00 03 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x300], 0x2
140a5ba14: 48 89 74 24 20              	mov	qword ptr [rsp + 0x20], rsi
140a5ba19: 48 8d 4d 40                 	lea	rcx, [rbp + 0x40]
140a5ba1d: 48 8d 95 c0 04 00 00        	lea	rdx, [rbp + 0x4c0]
140a5ba24: 49 b8 5e a9 e9 1c 54 8e 97 d9       	movabs	r8, -0x266871abe31656a2
140a5ba2e: 41 b1 19                    	mov	r9b, 0x19
140a5ba31: e8 0a 28 e9 ff              	call	0x1408ee240 <.text+0x8ed240>
140a5ba36: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a5ba3c: 48 89 f1                    	mov	rcx, rsi
140a5ba3f: 48 89 fa                    	mov	rdx, rdi
140a5ba42: e8 b9 81 ec 01              	call	0x142923c00 <.text+0x2922c00>
140a5ba47: 48 8b 85 38 07 00 00        	mov	rax, qword ptr [rbp + 0x738]
140a5ba4e: 48 89 45 40                 	mov	qword ptr [rbp + 0x40], rax
140a5ba52: c7 85 c0 04 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x4c0], 0x2
140a5ba5c: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
140a5ba61: 48 8d 4d 40                 	lea	rcx, [rbp + 0x40]
140a5ba65: 49 b8 5a 0b 50 93 7f 1f 0c b9       	movabs	r8, -0x46f3e0806caff4a6
140a5ba6f: 48 89 f2                    	mov	rdx, rsi
140a5ba72: 41 b1 19                    	mov	r9b, 0x19
140a5ba75: e8 c6 27 e9 ff              	call	0x1408ee240 <.text+0x8ed240>
140a5ba7a: 48 8d b5 c0 04 00 00        	lea	rsi, [rbp + 0x4c0]
140a5ba81: 48 8d bd 00 03 00 00        	lea	rdi, [rbp + 0x300]
140a5ba88: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a5ba8e: 48 89 f1                    	mov	rcx, rsi
140a5ba91: 48 89 fa                    	mov	rdx, rdi
140a5ba94: e8 67 81 ec 01              	call	0x142923c00 <.text+0x2922c00>
140a5ba99: 48 8d 45 bc                 	lea	rax, [rbp - 0x44]
140a5ba9d: 48 89 45 40                 	mov	qword ptr [rbp + 0x40], rax
140a5baa1: c7 85 00 03 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x300], 0x2
140a5baab: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
140a5bab0: 48 8d 4d 40                 	lea	rcx, [rbp + 0x40]
140a5bab4: 49 b8 ae d2 41 c8 fd d6 de 84       	movabs	r8, -0x7b21290237be2d52
140a5babe: 48 89 f2                    	mov	rdx, rsi
140a5bac1: 41 b1 19                    	mov	r9b, 0x19
140a5bac4: e8 77 27 e9 ff              	call	0x1408ee240 <.text+0x8ed240>
140a5bac9: 48 8d b5 00 03 00 00        	lea	rsi, [rbp + 0x300]
140a5bad0: 48 8d bd c0 04 00 00        	lea	rdi, [rbp + 0x4c0]
140a5bad7: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a5badd: 48 89 f1                    	mov	rcx, rsi
140a5bae0: 48 89 fa                    	mov	rdx, rdi
140a5bae3: e8 18 81 ec 01              	call	0x142923c00 <.text+0x2922c00>
140a5bae8: 48 8b 85 30 07 00 00        	mov	rax, qword ptr [rbp + 0x730]
140a5baef: 48 89 45 40                 	mov	qword ptr [rbp + 0x40], rax
140a5baf3: c7 85 c0 04 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x4c0], 0x2
140a5bafd: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
140a5bb02: 48 8d 4d 40                 	lea	rcx, [rbp + 0x40]
140a5bb06: 49 b8 fc fb 49 a0 9d 57 61 48       	movabs	r8, 0x4861579da049fbfc
140a5bb10: 48 89 f2                    	mov	rdx, rsi
140a5bb13: 41 b1 19                    	mov	r9b, 0x19
140a5bb16: e8 25 27 e9 ff              	call	0x1408ee240 <.text+0x8ed240>
140a5bb1b: 48 8d b5 c0 04 00 00        	lea	rsi, [rbp + 0x4c0]
140a5bb22: 48 8d bd 00 03 00 00        	lea	rdi, [rbp + 0x300]
140a5bb29: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a5bb2f: 48 89 f1                    	mov	rcx, rsi
140a5bb32: 48 89 fa                    	mov	rdx, rdi
140a5bb35: e8 c6 80 ec 01              	call	0x142923c00 <.text+0x2922c00>
140a5bb3a: 48 b8 65 18 a0 91 ab 39 9f 57       	movabs	rax, 0x579f39ab91a01865
140a5bb44: 48 89 45 40                 	mov	qword ptr [rbp + 0x40], rax
140a5bb48: c7 85 00 03 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x300], 0x2
140a5bb52: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
140a5bb57: 48 8d 4d 40                 	lea	rcx, [rbp + 0x40]
140a5bb5b: 49 b8 8c 97 66 69 03 f5 5b 81       	movabs	r8, -0x7ea40afc96996874
140a5bb65: 48 89 f2                    	mov	rdx, rsi
140a5bb68: 41 b1 19                    	mov	r9b, 0x19
140a5bb6b: e8 60 9b ea ff              	call	0x1409056d0 <.text+0x9046d0>
140a5bb70: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a5bb77: 48 8d 95 c0 04 00 00        	lea	rdx, [rbp + 0x4c0]
140a5bb7e: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a5bb84: e8 77 80 ec 01              	call	0x142923c00 <.text+0x2922c00>
140a5bb89: 80 bd 36 04 00 00 08        	cmp	byte ptr [rbp + 0x436], 0x8
140a5bb90: 74 25                       	je	0x140a5bbb7 <.text+0xa5abb7>
140a5bb92: 48 8d 8d 10 04 00 00        	lea	rcx, [rbp + 0x410]
140a5bb99: ba e8 00 00 00              	mov	edx, 0xe8
140a5bb9e: 48 03 95 38 04 00 00        	add	rdx, qword ptr [rbp + 0x438]
140a5bba5: 49 b8 ab 08 87 47 b4 e0 4e b3       	movabs	r8, -0x4cb11f4bb878f755
140a5bbaf: 0f 28 de                    	movaps	xmm3, xmm6
140a5bbb2: e8 79 fe f0 ff              	call	0x14096ba30 <.text+0x96aa30>
140a5bbb7: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a5bbbd: 48 89 f1                    	mov	rcx, rsi
140a5bbc0: 48 89 fa                    	mov	rdx, rdi
140a5bbc3: e8 38 80 ec 01              	call	0x142923c00 <.text+0x2922c00>
140a5bbc8: 48 8d 85 f0 01 00 00        	lea	rax, [rbp + 0x1f0]
140a5bbcf: 48 89 45 40                 	mov	qword ptr [rbp + 0x40], rax
140a5bbd3: c7 85 00 03 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x300], 0x2
140a5bbdd: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
140a5bbe2: 48 8d 4d 40                 	lea	rcx, [rbp + 0x40]
140a5bbe6: 49 b8 c8 8b 41 02 02 79 81 b9       	movabs	r8, -0x467e86fdfdbe7438
140a5bbf0: 48 89 f2                    	mov	rdx, rsi
140a5bbf3: 41 b1 32                    	mov	r9b, 0x32
140a5bbf6: e8 05 21 e9 ff              	call	0x1408edd00 <.text+0x8ecd00>
140a5bbfb: 48 8d b5 00 03 00 00        	lea	rsi, [rbp + 0x300]
140a5bc02: 48 8d 95 c0 04 00 00        	lea	rdx, [rbp + 0x4c0]
140a5bc09: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a5bc0f: 48 89 f1                    	mov	rcx, rsi
140a5bc12: e8 e9 7f ec 01              	call	0x142923c00 <.text+0x2922c00>
140a5bc17: 48 8d 95 c0 02 00 00        	lea	rdx, [rbp + 0x2c0]
140a5bc1e: 41 b8 40 00 00 00           	mov	r8d, 0x40
140a5bc24: 48 89 f1                    	mov	rcx, rsi
140a5bc27: e8 54 2c f1 ff              	call	0x14096e880 <.text+0x96d880>
140a5bc2c: 48 8b 85 98 06 00 00        	mov	rax, qword ptr [rbp + 0x698]
140a5bc33: 48 89 44 24 28              	mov	qword ptr [rsp + 0x28], rax
140a5bc38: 48 8b 85 90 06 00 00        	mov	rax, qword ptr [rbp + 0x690]
140a5bc3f: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
140a5bc44: 48 8d 15 c5 76 07 02        	lea	rdx, [rip + 0x20776c5]  # 0x142ad3310
140a5bc4b: 48 8d 8d c0 04 00 00        	lea	rcx, [rbp + 0x4c0]
140a5bc52: 41 b8 0f 00 00 00           	mov	r8d, 0xf
140a5bc58: 45 89 e1                    	mov	r9d, r12d
140a5bc5b: e8 c0 1b ba 00              	call	0x1415fd820 <.text+0x15fc820>
140a5bc60: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a5bc67: 48 8d 95 c0 04 00 00        	lea	rdx, [rbp + 0x4c0]
140a5bc6e: 49 b8 7d 9e 00 1d 50 10 67 ce       	movabs	r8, -0x3198efafe2ff6183
140a5bc78: 4c 8d 8d e0 02 00 00        	lea	r9, [rbp + 0x2e0]
140a5bc7f: e8 3c 1e f1 ff              	call	0x14096dac0 <.text+0x96cac0>
140a5bc84: 48 8d 8d c0 04 00 00        	lea	rcx, [rbp + 0x4c0]
140a5bc8b: 49 b8 ff aa 4a 65 54 24 7d b9       	movabs	r8, -0x4682dbab9ab55501
140a5bc95: 4c 8d 8d a0 02 00 00        	lea	r9, [rbp + 0x2a0]
140a5bc9c: 48 89 f2                    	mov	rdx, rsi
140a5bc9f: e8 1c 1e f1 ff              	call	0x14096dac0 <.text+0x96cac0>
140a5bca4: 48 8b 85 b0 04 00 00        	mov	rax, qword ptr [rbp + 0x4b0]
140a5bcab: 48 89 45 40                 	mov	qword ptr [rbp + 0x40], rax
140a5bcaf: c7 85 00 03 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x300], 0x2
140a5bcb9: 48 89 74 24 20              	mov	qword ptr [rsp + 0x20], rsi
140a5bcbe: 48 8d 4d 40                 	lea	rcx, [rbp + 0x40]
140a5bcc2: 48 8d 95 c0 04 00 00        	lea	rdx, [rbp + 0x4c0]
140a5bcc9: 49 b8 5e a9 e9 1c 54 8e 97 d9       	movabs	r8, -0x266871abe31656a2
140a5bcd3: 41 b1 19                    	mov	r9b, 0x19
140a5bcd6: e8 65 25 e9 ff              	call	0x1408ee240 <.text+0x8ed240>
140a5bcdb: 48 8d bd c0 04 00 00        	lea	rdi, [rbp + 0x4c0]
140a5bce2: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a5bce8: 48 89 f1                    	mov	rcx, rsi
140a5bceb: 48 89 fa                    	mov	rdx, rdi
140a5bcee: e8 0d 7f ec 01              	call	0x142923c00 <.text+0x2922c00>
140a5bcf3: 48 8b 85 38 07 00 00        	mov	rax, qword ptr [rbp + 0x738]
140a5bcfa: 48 89 45 40                 	mov	qword ptr [rbp + 0x40], rax
140a5bcfe: c7 85 c0 04 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x4c0], 0x2
140a5bd08: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
140a5bd0d: 48 8d 4d 40                 	lea	rcx, [rbp + 0x40]
140a5bd11: 49 b8 5a 0b 50 93 7f 1f 0c b9       	movabs	r8, -0x46f3e0806caff4a6
140a5bd1b: 48 89 f2                    	mov	rdx, rsi
140a5bd1e: 41 b1 19                    	mov	r9b, 0x19
140a5bd21: e8 1a 25 e9 ff              	call	0x1408ee240 <.text+0x8ed240>
140a5bd26: 48 8d b5 c0 04 00 00        	lea	rsi, [rbp + 0x4c0]
140a5bd2d: 48 8d bd 00 03 00 00        	lea	rdi, [rbp + 0x300]
140a5bd34: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a5bd3a: 48 89 f1                    	mov	rcx, rsi
140a5bd3d: 48 89 fa                    	mov	rdx, rdi
140a5bd40: e8 bb 7e ec 01              	call	0x142923c00 <.text+0x2922c00>
140a5bd45: 48 8d 45 bc                 	lea	rax, [rbp - 0x44]
140a5bd49: 48 89 45 40                 	mov	qword ptr [rbp + 0x40], rax
140a5bd4d: c7 85 00 03 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x300], 0x2
140a5bd57: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
140a5bd5c: 48 8d 4d 40                 	lea	rcx, [rbp + 0x40]
140a5bd60: 49 b8 ae d2 41 c8 fd d6 de 84       	movabs	r8, -0x7b21290237be2d52
140a5bd6a: 48 89 f2                    	mov	rdx, rsi
140a5bd6d: 41 b1 19                    	mov	r9b, 0x19
140a5bd70: e8 cb 24 e9 ff              	call	0x1408ee240 <.text+0x8ed240>
140a5bd75: 48 8d b5 00 03 00 00        	lea	rsi, [rbp + 0x300]
140a5bd7c: 48 8d bd c0 04 00 00        	lea	rdi, [rbp + 0x4c0]
140a5bd83: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a5bd89: 48 89 f1                    	mov	rcx, rsi
140a5bd8c: 48 89 fa                    	mov	rdx, rdi
140a5bd8f: e8 6c 7e ec 01              	call	0x142923c00 <.text+0x2922c00>
140a5bd94: 48 8b 85 30 07 00 00        	mov	rax, qword ptr [rbp + 0x730]
140a5bd9b: 48 89 45 40                 	mov	qword ptr [rbp + 0x40], rax
140a5bd9f: c7 85 c0 04 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x4c0], 0x2
140a5bda9: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
140a5bdae: 48 8d 4d 40                 	lea	rcx, [rbp + 0x40]
140a5bdb2: 49 b8 fc fb 49 a0 9d 57 61 48       	movabs	r8, 0x4861579da049fbfc
140a5bdbc: 48 89 f2                    	mov	rdx, rsi
140a5bdbf: 41 b1 19                    	mov	r9b, 0x19
140a5bdc2: e8 79 24 e9 ff              	call	0x1408ee240 <.text+0x8ed240>
140a5bdc7: 48 8d b5 c0 04 00 00        	lea	rsi, [rbp + 0x4c0]
140a5bdce: 48 8d bd 00 03 00 00        	lea	rdi, [rbp + 0x300]
140a5bdd5: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a5bddb: 48 89 f1                    	mov	rcx, rsi
140a5bdde: 48 89 fa                    	mov	rdx, rdi
140a5bde1: e8 1a 7e ec 01              	call	0x142923c00 <.text+0x2922c00>
140a5bde6: 48 b8 65 18 a0 91 ab 39 9f 57       	movabs	rax, 0x579f39ab91a01865
140a5bdf0: 48 89 45 40                 	mov	qword ptr [rbp + 0x40], rax
140a5bdf4: c7 85 00 03 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x300], 0x2
140a5bdfe: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
140a5be03: 48 8d 4d 40                 	lea	rcx, [rbp + 0x40]
140a5be07: 49 b8 8c 97 66 69 03 f5 5b 81       	movabs	r8, -0x7ea40afc96996874
140a5be11: 48 89 f2                    	mov	rdx, rsi
140a5be14: 41 b1 19                    	mov	r9b, 0x19
140a5be17: e8 b4 98 ea ff              	call	0x1409056d0 <.text+0x9046d0>
140a5be1c: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a5be23: 48 8d 95 c0 04 00 00        	lea	rdx, [rbp + 0x4c0]
140a5be2a: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a5be30: e8 cb 7d ec 01              	call	0x142923c00 <.text+0x2922c00>
140a5be35: 80 bd 36 04 00 00 08        	cmp	byte ptr [rbp + 0x436], 0x8
140a5be3c: 74 25                       	je	0x140a5be63 <.text+0xa5ae63>
140a5be3e: 48 8d 8d 10 04 00 00        	lea	rcx, [rbp + 0x410]
140a5be45: ba e8 00 00 00              	mov	edx, 0xe8
140a5be4a: 48 03 95 38 04 00 00        	add	rdx, qword ptr [rbp + 0x438]
140a5be51: 49 b8 ab 08 87 47 b4 e0 4e b3       	movabs	r8, -0x4cb11f4bb878f755
140a5be5b: 0f 28 de                    	movaps	xmm3, xmm6
140a5be5e: e8 cd fb f0 ff              	call	0x14096ba30 <.text+0x96aa30>
140a5be63: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a5be69: 48 89 f1                    	mov	rcx, rsi
140a5be6c: 48 89 fa                    	mov	rdx, rdi
140a5be6f: e8 8c 7d ec 01              	call	0x142923c00 <.text+0x2922c00>
140a5be74: 48 8d 85 f0 01 00 00        	lea	rax, [rbp + 0x1f0]
140a5be7b: 48 89 45 40                 	mov	qword ptr [rbp + 0x40], rax
140a5be7f: c7 85 00 03 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x300], 0x2
140a5be89: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
140a5be8e: 48 8d 4d 40                 	lea	rcx, [rbp + 0x40]
140a5be92: 49 b8 c8 8b 41 02 02 79 81 b9       	movabs	r8, -0x467e86fdfdbe7438
140a5be9c: 48 89 f2                    	mov	rdx, rsi
140a5be9f: 41 b1 32                    	mov	r9b, 0x32
140a5bea2: e8 59 1e e9 ff              	call	0x1408edd00 <.text+0x8ecd00>
140a5bea7: 4c 8d ad 00 03 00 00        	lea	r13, [rbp + 0x300]
140a5beae: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a5beb4: 4c 89 e9                    	mov	rcx, r13
140a5beb7: 48 89 f2                    	mov	rdx, rsi
140a5beba: e8 41 7d ec 01              	call	0x142923c00 <.text+0x2922c00>
140a5bebf: 48 8d 95 c0 02 00 00        	lea	rdx, [rbp + 0x2c0]
140a5bec6: 41 b8 50 00 00 00           	mov	r8d, 0x50
140a5becc: 4c 89 e9                    	mov	rcx, r13
140a5becf: e8 ac 29 f1 ff              	call	0x14096e880 <.text+0x96d880>
140a5bed4: 44 89 a5 a4 04 00 00        	mov	dword ptr [rbp + 0x4a4], r12d
140a5bedb: 44 89 a5 a0 04 00 00        	mov	dword ptr [rbp + 0x4a0], r12d
140a5bee2: 44 89 a5 9c 04 00 00        	mov	dword ptr [rbp + 0x49c], r12d
140a5bee9: 48 8d 05 58 74 07 02        	lea	rax, [rip + 0x2077458]  # 0x142ad3348
140a5bef0: 48 89 85 20 02 00 00        	mov	qword ptr [rbp + 0x220], rax
140a5bef7: 48 c7 85 28 02 00 00 14 00 00 00    	mov	qword ptr [rbp + 0x228], 0x14
140a5bf02: 48 8d 85 a4 04 00 00        	lea	rax, [rbp + 0x4a4]
140a5bf09: 48 89 85 30 02 00 00        	mov	qword ptr [rbp + 0x230], rax
140a5bf10: 48 8d 05 09 74 07 02        	lea	rax, [rip + 0x2077409]  # 0x142ad3320
140a5bf17: 48 89 85 38 02 00 00        	mov	qword ptr [rbp + 0x238], rax
140a5bf1e: c6 85 40 02 00 00 01        	mov	byte ptr [rbp + 0x240], 0x1
140a5bf25: 48 8d 05 5c 74 07 02        	lea	rax, [rip + 0x207745c]  # 0x142ad3388
140a5bf2c: 48 89 85 48 02 00 00        	mov	qword ptr [rbp + 0x248], rax
140a5bf33: 48 c7 85 50 02 00 00 14 00 00 00    	mov	qword ptr [rbp + 0x250], 0x14
140a5bf3e: 48 8d 85 a0 04 00 00        	lea	rax, [rbp + 0x4a0]
140a5bf45: 48 89 85 58 02 00 00        	mov	qword ptr [rbp + 0x258], rax
140a5bf4c: 48 8d 05 0d 74 07 02        	lea	rax, [rip + 0x207740d]  # 0x142ad3360
140a5bf53: 48 89 85 60 02 00 00        	mov	qword ptr [rbp + 0x260], rax
140a5bf5a: c6 85 68 02 00 00 02        	mov	byte ptr [rbp + 0x268], 0x2
140a5bf61: 48 8d 05 60 74 07 02        	lea	rax, [rip + 0x2077460]  # 0x142ad33c8
140a5bf68: 48 89 85 70 02 00 00        	mov	qword ptr [rbp + 0x270], rax
140a5bf6f: 48 c7 85 78 02 00 00 0b 00 00 00    	mov	qword ptr [rbp + 0x278], 0xb
140a5bf7a: 48 8d 85 9c 04 00 00        	lea	rax, [rbp + 0x49c]
140a5bf81: 48 89 85 80 02 00 00        	mov	qword ptr [rbp + 0x280], rax
140a5bf88: 48 8d 05 11 74 07 02        	lea	rax, [rip + 0x2077411]  # 0x142ad33a0
140a5bf8f: 48 89 85 88 02 00 00        	mov	qword ptr [rbp + 0x288], rax
140a5bf96: c6 85 90 02 00 00 00        	mov	byte ptr [rbp + 0x290], 0x0
140a5bf9d: 45 31 ff                    	xor	r15d, r15d
140a5bfa0: 4c 8d a5 60 06 00 00        	lea	r12, [rbp + 0x660]
140a5bfa7: 48 8d 7d 40                 	lea	rdi, [rbp + 0x40]
140a5bfab: 4c 8d b5 c0 02 00 00        	lea	r14, [rbp + 0x2c0]
140a5bfb2: 49 83 ff 78                 	cmp	r15, 0x78
140a5bfb6: 0f 84 e1 02 00 00           	je	0x140a5c29d <.text+0xa5b29d>
140a5bfbc: 0f 1f 40 00                 	nop	dword ptr [rax]
140a5bfc0: 4a 8b 94 3d 20 02 00 00     	mov	rdx, qword ptr [rbp + r15 + 0x220]
140a5bfc8: 4e 8b 84 3d 28 02 00 00     	mov	r8, qword ptr [rbp + r15 + 0x228]
140a5bfd0: 4e 8b 8c 3d 30 02 00 00     	mov	r9, qword ptr [rbp + r15 + 0x230]
140a5bfd8: 4a 8b 84 3d 38 02 00 00     	mov	rax, qword ptr [rbp + r15 + 0x238]
140a5bfe0: 42 0f b6 9c 3d 40 02 00 00  	movzx	ebx, byte ptr [rbp + r15 + 0x240]
140a5bfe9: 48 8b 8d 98 06 00 00        	mov	rcx, qword ptr [rbp + 0x698]
140a5bff0: 48 89 4c 24 30              	mov	qword ptr [rsp + 0x30], rcx
140a5bff5: 48 8b 8d 90 06 00 00        	mov	rcx, qword ptr [rbp + 0x690]
140a5bffc: 48 89 4c 24 28              	mov	qword ptr [rsp + 0x28], rcx
140a5c001: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
140a5c006: 48 89 f1                    	mov	rcx, rsi
140a5c009: e8 62 45 ba 00              	call	0x141600570 <.text+0x15ff570>
140a5c00e: 4c 89 e9                    	mov	rcx, r13
140a5c011: 48 89 f2                    	mov	rdx, rsi
140a5c014: 49 b8 7d 9e 00 1d 50 10 67 ce       	movabs	r8, -0x3198efafe2ff6183
140a5c01e: 4c 8d 8d e0 02 00 00        	lea	r9, [rbp + 0x2e0]
140a5c025: e8 96 1a f1 ff              	call	0x14096dac0 <.text+0x96cac0>
140a5c02a: 48 89 f1                    	mov	rcx, rsi
140a5c02d: 4c 89 ea                    	mov	rdx, r13
140a5c030: 49 b8 ff aa 4a 65 54 24 7d b9       	movabs	r8, -0x4682dbab9ab55501
140a5c03a: 4c 8d 8d a0 02 00 00        	lea	r9, [rbp + 0x2a0]
140a5c041: e8 7a 1a f1 ff              	call	0x14096dac0 <.text+0x96cac0>
140a5c046: 48 8b 85 b0 04 00 00        	mov	rax, qword ptr [rbp + 0x4b0]
140a5c04d: 48 89 85 60 06 00 00        	mov	qword ptr [rbp + 0x660], rax
140a5c054: c7 85 00 03 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x300], 0x2
140a5c05e: 4c 89 6c 24 20              	mov	qword ptr [rsp + 0x20], r13
140a5c063: 4c 89 e1                    	mov	rcx, r12
140a5c066: 48 89 f2                    	mov	rdx, rsi
140a5c069: 49 b8 5e a9 e9 1c 54 8e 97 d9       	movabs	r8, -0x266871abe31656a2
140a5c073: 41 b1 19                    	mov	r9b, 0x19
140a5c076: e8 c5 21 e9 ff              	call	0x1408ee240 <.text+0x8ed240>
140a5c07b: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a5c081: 4c 89 e9                    	mov	rcx, r13
140a5c084: 48 89 f2                    	mov	rdx, rsi
140a5c087: e8 74 7b ec 01              	call	0x142923c00 <.text+0x2922c00>
140a5c08c: 48 8b 85 38 07 00 00        	mov	rax, qword ptr [rbp + 0x738]
140a5c093: 48 89 85 60 06 00 00        	mov	qword ptr [rbp + 0x660], rax
140a5c09a: c7 85 c0 04 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x4c0], 0x2
140a5c0a4: 48 89 74 24 20              	mov	qword ptr [rsp + 0x20], rsi
140a5c0a9: 4c 89 e1                    	mov	rcx, r12
140a5c0ac: 4c 89 ea                    	mov	rdx, r13
140a5c0af: 49 b8 5a 0b 50 93 7f 1f 0c b9       	movabs	r8, -0x46f3e0806caff4a6
140a5c0b9: 41 b1 19                    	mov	r9b, 0x19
140a5c0bc: e8 7f 21 e9 ff              	call	0x1408ee240 <.text+0x8ed240>
140a5c0c1: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a5c0c7: 48 89 f1                    	mov	rcx, rsi
140a5c0ca: 4c 89 ea                    	mov	rdx, r13
140a5c0cd: e8 2e 7b ec 01              	call	0x142923c00 <.text+0x2922c00>
140a5c0d2: 48 8d 45 bc                 	lea	rax, [rbp - 0x44]
140a5c0d6: 48 89 85 60 06 00 00        	mov	qword ptr [rbp + 0x660], rax
140a5c0dd: c7 85 00 03 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x300], 0x2
140a5c0e7: 4c 89 6c 24 20              	mov	qword ptr [rsp + 0x20], r13
140a5c0ec: 4c 89 e1                    	mov	rcx, r12
140a5c0ef: 48 89 f2                    	mov	rdx, rsi
140a5c0f2: 49 b8 ae d2 41 c8 fd d6 de 84       	movabs	r8, -0x7b21290237be2d52
140a5c0fc: 41 b1 19                    	mov	r9b, 0x19
140a5c0ff: e8 3c 21 e9 ff              	call	0x1408ee240 <.text+0x8ed240>
140a5c104: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a5c10a: 4c 89 e9                    	mov	rcx, r13
140a5c10d: 48 89 f2                    	mov	rdx, rsi
140a5c110: e8 eb 7a ec 01              	call	0x142923c00 <.text+0x2922c00>
140a5c115: 48 8b 85 30 07 00 00        	mov	rax, qword ptr [rbp + 0x730]
140a5c11c: 48 89 85 60 06 00 00        	mov	qword ptr [rbp + 0x660], rax
140a5c123: c7 85 c0 04 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x4c0], 0x2
140a5c12d: 48 89 74 24 20              	mov	qword ptr [rsp + 0x20], rsi
140a5c132: 4c 89 e1                    	mov	rcx, r12
140a5c135: 4c 89 ea                    	mov	rdx, r13
140a5c138: 49 b8 fc fb 49 a0 9d 57 61 48       	movabs	r8, 0x4861579da049fbfc
140a5c142: 41 b1 19                    	mov	r9b, 0x19
140a5c145: e8 f6 20 e9 ff              	call	0x1408ee240 <.text+0x8ed240>
140a5c14a: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a5c150: 48 89 f1                    	mov	rcx, rsi
140a5c153: 4c 89 ea                    	mov	rdx, r13
140a5c156: e8 a5 7a ec 01              	call	0x142923c00 <.text+0x2922c00>
140a5c15b: 48 b8 65 18 a0 91 ab 39 9f 57       	movabs	rax, 0x579f39ab91a01865
140a5c165: 48 89 85 60 06 00 00        	mov	qword ptr [rbp + 0x660], rax
140a5c16c: c7 85 00 03 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x300], 0x2
140a5c176: 4c 89 6c 24 20              	mov	qword ptr [rsp + 0x20], r13
140a5c17b: 4c 89 e1                    	mov	rcx, r12
140a5c17e: 48 89 f2                    	mov	rdx, rsi
140a5c181: 49 b8 8c 97 66 69 03 f5 5b 81       	movabs	r8, -0x7ea40afc96996874
140a5c18b: 41 b1 19                    	mov	r9b, 0x19
140a5c18e: e8 3d 95 ea ff              	call	0x1409056d0 <.text+0x9046d0>
140a5c193: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a5c199: 48 89 f9                    	mov	rcx, rdi
140a5c19c: 48 89 f2                    	mov	rdx, rsi
140a5c19f: e8 5c 7a ec 01              	call	0x142923c00 <.text+0x2922c00>
140a5c1a4: 80 bd 76 01 00 00 08        	cmp	byte ptr [rbp + 0x176], 0x8
140a5c1ab: 74 28                       	je	0x140a5c1d5 <.text+0xa5b1d5>
140a5c1ad: 48 8b 95 78 01 00 00        	mov	rdx, qword ptr [rbp + 0x178]
140a5c1b4: b8 e8 00 00 00              	mov	eax, 0xe8
140a5c1b9: 48 01 c2                    	add	rdx, rax
140a5c1bc: 48 8d 8d 50 01 00 00        	lea	rcx, [rbp + 0x150]
140a5c1c3: 49 b8 ab 08 87 47 b4 e0 4e b3       	movabs	r8, -0x4cb11f4bb878f755
140a5c1cd: 0f 28 de                    	movaps	xmm3, xmm6
140a5c1d0: e8 5b f8 f0 ff              	call	0x14096ba30 <.text+0x96aa30>
140a5c1d5: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a5c1db: 48 89 f1                    	mov	rcx, rsi
140a5c1de: 48 89 fa                    	mov	rdx, rdi
140a5c1e1: e8 1a 7a ec 01              	call	0x142923c00 <.text+0x2922c00>
140a5c1e6: 4c 89 e9                    	mov	rcx, r13
140a5c1e9: 48 89 f2                    	mov	rdx, rsi
140a5c1ec: 49 b8 c3 15 a9 0b 0c d8 5a bc       	movabs	r8, -0x43a527f3f456ea3d
140a5c1f6: e8 55 16 bb 00              	call	0x14160d850 <.text+0x160c850>
140a5c1fb: 48 8d 85 f0 01 00 00        	lea	rax, [rbp + 0x1f0]
140a5c202: 48 89 45 40                 	mov	qword ptr [rbp + 0x40], rax
140a5c206: c7 85 c0 04 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x4c0], 0x2
140a5c210: 48 89 74 24 20              	mov	qword ptr [rsp + 0x20], rsi
140a5c215: 48 89 f9                    	mov	rcx, rdi
140a5c218: 4c 89 ea                    	mov	rdx, r13
140a5c21b: 49 b8 f4 71 b6 80 88 ea a1 b0       	movabs	r8, -0x4f5e15777f498e0c
140a5c225: 41 b1 19                    	mov	r9b, 0x19
140a5c228: e8 13 20 e9 ff              	call	0x1408ee240 <.text+0x8ed240>
140a5c22d: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a5c233: 48 89 f1                    	mov	rcx, rsi
140a5c236: 4c 89 ea                    	mov	rdx, r13
140a5c239: e8 c2 79 ec 01              	call	0x142923c00 <.text+0x2922c00>
140a5c23e: 48 8d 45 e0                 	lea	rax, [rbp - 0x20]
140a5c242: 48 89 45 40                 	mov	qword ptr [rbp + 0x40], rax
140a5c246: c7 85 00 03 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x300], 0x2
140a5c250: 4c 89 6c 24 20              	mov	qword ptr [rsp + 0x20], r13
140a5c255: 48 89 f9                    	mov	rcx, rdi
140a5c258: 48 89 f2                    	mov	rdx, rsi
140a5c25b: 49 b8 c8 8b 41 02 02 79 81 b9       	movabs	r8, -0x467e86fdfdbe7438
140a5c265: 41 b1 32                    	mov	r9b, 0x32
140a5c268: e8 93 1a e9 ff              	call	0x1408edd00 <.text+0x8ecd00>
140a5c26d: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a5c273: 4c 89 e9                    	mov	rcx, r13
140a5c276: 48 89 f2                    	mov	rdx, rsi
140a5c279: e8 82 79 ec 01              	call	0x142923c00 <.text+0x2922c00>
140a5c27e: c1 e3 04                    	shl	ebx, 0x4
140a5c281: 4c 89 e9                    	mov	rcx, r13
140a5c284: 4c 89 f2                    	mov	rdx, r14
140a5c287: 49 89 d8                    	mov	r8, rbx
140a5c28a: e8 f1 25 f1 ff              	call	0x14096e880 <.text+0x96d880>
140a5c28f: 49 83 c7 28                 	add	r15, 0x28
140a5c293: 49 83 ff 78                 	cmp	r15, 0x78
140a5c297: 0f 85 23 fd ff ff           	jne	0x140a5bfc0 <.text+0xa5afc0>
140a5c29d: 48 8b 85 98 06 00 00        	mov	rax, qword ptr [rbp + 0x698]
140a5c2a4: 48 89 44 24 30              	mov	qword ptr [rsp + 0x30], rax
140a5c2a9: 48 8b 85 90 06 00 00        	mov	rax, qword ptr [rbp + 0x690]
140a5c2b0: 48 89 44 24 28              	mov	qword ptr [rsp + 0x28], rax
140a5c2b5: 48 c7 44 24 20 1d 00 00 00  	mov	qword ptr [rsp + 0x20], 0x1d
140a5c2be: 48 8d 15 0e 71 07 02        	lea	rdx, [rip + 0x207710e]  # 0x142ad33d3
140a5c2c5: 4c 8d 0d 13 71 07 02        	lea	r9, [rip + 0x2077113]   # 0x142ad33df
140a5c2cc: 48 8d 8d c0 04 00 00        	lea	rcx, [rbp + 0x4c0]
140a5c2d3: 41 b8 0c 00 00 00           	mov	r8d, 0xc
140a5c2d9: e8 c2 6f ba 00              	call	0x1416032a0 <.text+0x16022a0>
140a5c2de: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a5c2e5: 48 8d 95 c0 04 00 00        	lea	rdx, [rbp + 0x4c0]
140a5c2ec: 49 b8 7d 9e 00 1d 50 10 67 ce       	movabs	r8, -0x3198efafe2ff6183
140a5c2f6: 4c 8d 8d e0 02 00 00        	lea	r9, [rbp + 0x2e0]
140a5c2fd: e8 be 17 f1 ff              	call	0x14096dac0 <.text+0x96cac0>
140a5c302: 48 8d 8d c0 04 00 00        	lea	rcx, [rbp + 0x4c0]
140a5c309: 48 8d 95 00 03 00 00        	lea	rdx, [rbp + 0x300]
140a5c310: 49 b8 ff aa 4a 65 54 24 7d b9       	movabs	r8, -0x4682dbab9ab55501
140a5c31a: 4c 8d 8d a0 02 00 00        	lea	r9, [rbp + 0x2a0]
140a5c321: e8 9a 17 f1 ff              	call	0x14096dac0 <.text+0x96cac0>
140a5c326: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a5c32d: 48 8d b5 c0 04 00 00        	lea	rsi, [rbp + 0x4c0]
140a5c334: 49 b8 c3 15 a9 0b 0c d8 5a bc       	movabs	r8, -0x43a527f3f456ea3d
140a5c33e: 48 89 f2                    	mov	rdx, rsi
140a5c341: e8 0a 15 bb 00              	call	0x14160d850 <.text+0x160c850>
140a5c346: 48 b8 65 18 a0 91 ab 39 9f 57       	movabs	rax, 0x579f39ab91a01865
140a5c350: 48 89 45 40                 	mov	qword ptr [rbp + 0x40], rax
140a5c354: c7 85 c0 04 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x4c0], 0x2
140a5c35e: 48 89 74 24 20              	mov	qword ptr [rsp + 0x20], rsi
140a5c363: 48 8d 4d 40                 	lea	rcx, [rbp + 0x40]
140a5c367: 48 8d 95 00 03 00 00        	lea	rdx, [rbp + 0x300]
140a5c36e: 49 b8 8c 97 66 69 03 f5 5b 81       	movabs	r8, -0x7ea40afc96996874
140a5c378: 41 b1 19                    	mov	r9b, 0x19
140a5c37b: e8 50 93 ea ff              	call	0x1409056d0 <.text+0x9046d0>
140a5c380: 48 8d bd 00 03 00 00        	lea	rdi, [rbp + 0x300]
140a5c387: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a5c38d: 48 89 f1                    	mov	rcx, rsi
140a5c390: 48 89 fa                    	mov	rdx, rdi
140a5c393: e8 68 78 ec 01              	call	0x142923c00 <.text+0x2922c00>
140a5c398: 48 8d 85 f0 01 00 00        	lea	rax, [rbp + 0x1f0]
140a5c39f: 48 89 45 40                 	mov	qword ptr [rbp + 0x40], rax
140a5c3a3: c7 85 00 03 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x300], 0x2
140a5c3ad: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
140a5c3b2: 48 8d 4d 40                 	lea	rcx, [rbp + 0x40]
140a5c3b6: 49 b8 f4 71 b6 80 88 ea a1 b0       	movabs	r8, -0x4f5e15777f498e0c
140a5c3c0: 48 89 f2                    	mov	rdx, rsi
140a5c3c3: 41 b1 19                    	mov	r9b, 0x19
140a5c3c6: e8 75 1e e9 ff              	call	0x1408ee240 <.text+0x8ed240>
140a5c3cb: 48 8d b5 00 03 00 00        	lea	rsi, [rbp + 0x300]
140a5c3d2: 48 8d bd c0 04 00 00        	lea	rdi, [rbp + 0x4c0]
140a5c3d9: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a5c3df: 48 89 f1                    	mov	rcx, rsi
140a5c3e2: 48 89 fa                    	mov	rdx, rdi
140a5c3e5: e8 16 78 ec 01              	call	0x142923c00 <.text+0x2922c00>
140a5c3ea: 48 8b 85 38 07 00 00        	mov	rax, qword ptr [rbp + 0x738]
140a5c3f1: 48 89 45 40                 	mov	qword ptr [rbp + 0x40], rax
140a5c3f5: c7 85 c0 04 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x4c0], 0x2
140a5c3ff: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
140a5c404: 48 8d 4d 40                 	lea	rcx, [rbp + 0x40]
140a5c408: 49 b8 5a 0b 50 93 7f 1f 0c b9       	movabs	r8, -0x46f3e0806caff4a6
140a5c412: 48 89 f2                    	mov	rdx, rsi
140a5c415: 41 b1 19                    	mov	r9b, 0x19
140a5c418: e8 23 1e e9 ff              	call	0x1408ee240 <.text+0x8ed240>
140a5c41d: 48 8d b5 c0 04 00 00        	lea	rsi, [rbp + 0x4c0]
140a5c424: 48 8d bd 00 03 00 00        	lea	rdi, [rbp + 0x300]
140a5c42b: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a5c431: 48 89 f1                    	mov	rcx, rsi
140a5c434: 48 89 fa                    	mov	rdx, rdi
140a5c437: e8 c4 77 ec 01              	call	0x142923c00 <.text+0x2922c00>
140a5c43c: 48 8d 45 e0                 	lea	rax, [rbp - 0x20]
140a5c440: 48 89 45 40                 	mov	qword ptr [rbp + 0x40], rax
140a5c444: c7 85 00 03 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x300], 0x2
140a5c44e: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
140a5c453: 48 8d 4d 40                 	lea	rcx, [rbp + 0x40]
140a5c457: 49 b8 c8 8b 41 02 02 79 81 b9       	movabs	r8, -0x467e86fdfdbe7438
140a5c461: 48 89 f2                    	mov	rdx, rsi
140a5c464: 41 b1 32                    	mov	r9b, 0x32
140a5c467: e8 94 18 e9 ff              	call	0x1408edd00 <.text+0x8ecd00>
140a5c46c: 48 8d b5 00 03 00 00        	lea	rsi, [rbp + 0x300]
140a5c473: 48 8d 95 c0 04 00 00        	lea	rdx, [rbp + 0x4c0]
140a5c47a: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a5c480: 48 89 f1                    	mov	rcx, rsi
140a5c483: e8 78 77 ec 01              	call	0x142923c00 <.text+0x2922c00>
140a5c488: 48 8d 95 c0 02 00 00        	lea	rdx, [rbp + 0x2c0]
140a5c48f: 41 b8 30 00 00 00           	mov	r8d, 0x30
140a5c495: 48 89 f1                    	mov	rcx, rsi
140a5c498: e8 e3 23 f1 ff              	call	0x14096e880 <.text+0x96d880>
140a5c49d: 8b 45 00                    	mov	eax, dword ptr [rbp]
140a5c4a0: 48 8b 8d 58 06 00 00        	mov	rcx, qword ptr [rbp + 0x658]
140a5c4a7: 89 41 20                    	mov	dword ptr [rcx + 0x20], eax
140a5c4aa: 48 89 c8                    	mov	rax, rcx
140a5c4ad: 0f 10 45 e0                 	movups	xmm0, xmmword ptr [rbp - 0x20]
140a5c4b1: 0f 10 4d f0                 	movups	xmm1, xmmword ptr [rbp - 0x10]
140a5c4b5: 0f 11 49 10                 	movups	xmmword ptr [rcx + 0x10], xmm1
140a5c4b9: 0f 11 01                    	movups	xmmword ptr [rcx], xmm0
140a5c4bc: 48 8b 95 b8 04 00 00        	mov	rdx, qword ptr [rbp + 0x4b8]
140a5c4c3: 48 85 d2                    	test	rdx, rdx
140a5c4c6: 74 1d                       	je	0x140a5c4e5 <.text+0xa5b4e5>
140a5c4c8: 48 c1 e2 02                 	shl	rdx, 0x2
140a5c4cc: 41 b8 04 00 00 00           	mov	r8d, 0x4
140a5c4d2: 48 8b 8d 80 06 00 00        	mov	rcx, qword ptr [rbp + 0x680]
140a5c4d9: e8 42 77 bb ff              	call	0x140613c20 <.text+0x612c20>
140a5c4de: 48 8b 85 58 06 00 00        	mov	rax, qword ptr [rbp + 0x658]
140a5c4e5: 48 8b 95 d8 01 00 00        	mov	rdx, qword ptr [rbp + 0x1d8]
140a5c4ec: 48 85 d2                    	test	rdx, rdx
140a5c4ef: 0f 84 c3 e6 ff ff           	je	0x140a5abb8 <.text+0xa59bb8>
140a5c4f5: 48 8b 8d e0 01 00 00        	mov	rcx, qword ptr [rbp + 0x1e0]
140a5c4fc: 48 c1 e2 02                 	shl	rdx, 0x2
140a5c500: 41 b8 04 00 00 00           	mov	r8d, 0x4
140a5c506: e8 15 77 bb ff              	call	0x140613c20 <.text+0x612c20>
140a5c50b: 48 8b 85 58 06 00 00        	mov	rax, qword ptr [rbp + 0x658]
140a5c512: e9 a1 e6 ff ff              	jmp	0x140a5abb8 <.text+0xa59bb8>
140a5c517: 48 8b 8d 88 06 00 00        	mov	rcx, qword ptr [rbp + 0x688]
140a5c51e: 41 b8 00 ca 9a 3b           	mov	r8d, 0x3b9aca00
140a5c524: e8 e7 0c e7 01              	call	0x1428cd210 <.text+0x28cc210>
140a5c529: e9 1c eb ff ff              	jmp	0x140a5b04a <.text+0xa5a04a>
140a5c52e: 4c 8d 05 8b 91 03 02        	lea	r8, [rip + 0x203918b]   # 0x142a956c0
140a5c535: 48 89 f2                    	mov	rdx, rsi
140a5c538: e8 a3 d3 e7 01              	call	0x1428d98e0 <.text+0x28d88e0>
140a5c53d: eb 76                       	jmp	0x140a5c5b5 <.text+0xa5b5b5>
140a5c53f: c6 85 a6 06 00 00 01        	mov	byte ptr [rbp + 0x6a6], 0x1
140a5c546: 48 89 f1                    	mov	rcx, rsi
140a5c549: 31 d2                       	xor	edx, edx
140a5c54b: e8 40 16 e7 01              	call	0x1428cdb90 <.text+0x28ccb90>
140a5c550: e9 d0 eb ff ff              	jmp	0x140a5b125 <.text+0xa5a125>
140a5c555: 48 8d 0d fc 6c 07 02        	lea	rcx, [rip + 0x2076cfc]  # 0x142ad3258
140a5c55c: e8 af cd e7 01              	call	0x1428d9310 <.text+0x28d8310>
140a5c561: eb 52                       	jmp	0x140a5c5b5 <.text+0xa5b5b5>
140a5c563: 48 8d 0d 8e 6d 07 02        	lea	rcx, [rip + 0x2076d8e]  # 0x142ad32f8
140a5c56a: e8 a1 cd e7 01              	call	0x1428d9310 <.text+0x28d8310>
140a5c56f: eb 44                       	jmp	0x140a5c5b5 <.text+0xa5b5b5>
140a5c571: 4c 89 c2                    	mov	rdx, r8
140a5c574: 4c 8d 05 fd c5 1d 02        	lea	r8, [rip + 0x21dc5fd]   # 0x142c38b78
140a5c57b: 48 89 d9                    	mov	rcx, rbx
140a5c57e: e8 60 ca e7 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
140a5c583: eb 30                       	jmp	0x140a5c5b5 <.text+0xa5b5b5>
140a5c585: 4c 8d 05 44 13 22 02        	lea	r8, [rip + 0x2221344]   # 0x142c7d8d0
140a5c58c: b9 01 00 00 00              	mov	ecx, 0x1
140a5c591: ba 04 00 00 00              	mov	edx, 0x4
140a5c596: e8 48 ca e7 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
140a5c59b: eb 18                       	jmp	0x140a5c5b5 <.text+0xa5b5b5>
140a5c59d: 4c 8d 05 d4 c5 1d 02        	lea	r8, [rip + 0x21dc5d4]   # 0x142c38b78
140a5c5a4: b9 04 00 00 00              	mov	ecx, 0x4
140a5c5a9: 48 8b 95 78 06 00 00        	mov	rdx, qword ptr [rbp + 0x678]
140a5c5b0: e8 2e ca e7 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
140a5c5b5: 0f 0b                       	ud2
140a5c5b7: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
140a5c5c0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5c5c5: 55                          	push	rbp
140a5c5c6: 41 57                       	push	r15
140a5c5c8: 41 56                       	push	r14
140a5c5ca: 41 55                       	push	r13
140a5c5cc: 41 54                       	push	r12
140a5c5ce: 56                          	push	rsi
140a5c5cf: 57                          	push	rdi
140a5c5d0: 53                          	push	rbx
140a5c5d1: 48 83 ec 48                 	sub	rsp, 0x48
140a5c5d5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5c5dc: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5c5e1: 80 bd f6 05 00 00 08        	cmp	byte ptr [rbp + 0x5f6], 0x8
140a5c5e8: 74 0c                       	je	0x140a5c5f6 <.text+0xa5b5f6>
140a5c5ea: 48 8d 8d e0 05 00 00        	lea	rcx, [rbp + 0x5e0]
140a5c5f1: e8 ba 5e 18 00              	call	0x140be24b0 <.text+0xbe14b0>
140a5c5f6: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5c5fb: 48 83 c4 48                 	add	rsp, 0x48
140a5c5ff: 5b                          	pop	rbx
140a5c600: 5f                          	pop	rdi
140a5c601: 5e                          	pop	rsi
140a5c602: 41 5c                       	pop	r12
140a5c604: 41 5d                       	pop	r13
140a5c606: 41 5e                       	pop	r14
140a5c608: 41 5f                       	pop	r15
140a5c60a: 5d                          	pop	rbp
140a5c60b: c3                          	ret
140a5c60c: 0f 1f 40 00                 	nop	dword ptr [rax]
140a5c610: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5c615: 55                          	push	rbp
140a5c616: 41 57                       	push	r15
140a5c618: 41 56                       	push	r14
140a5c61a: 41 55                       	push	r13
140a5c61c: 41 54                       	push	r12
140a5c61e: 56                          	push	rsi
140a5c61f: 57                          	push	rdi
140a5c620: 53                          	push	rbx
140a5c621: 48 83 ec 48                 	sub	rsp, 0x48
140a5c625: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5c62c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5c631: 48 8b 95 d8 01 00 00        	mov	rdx, qword ptr [rbp + 0x1d8]
140a5c638: 48 85 d2                    	test	rdx, rdx
140a5c63b: 74 16                       	je	0x140a5c653 <.text+0xa5b653>
140a5c63d: 48 8b 8d e0 01 00 00        	mov	rcx, qword ptr [rbp + 0x1e0]
140a5c644: 48 c1 e2 02                 	shl	rdx, 0x2
140a5c648: 41 b8 04 00 00 00           	mov	r8d, 0x4
140a5c64e: e8 cd 75 bb ff              	call	0x140613c20 <.text+0x612c20>
140a5c653: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5c658: 48 83 c4 48                 	add	rsp, 0x48
140a5c65c: 5b                          	pop	rbx
140a5c65d: 5f                          	pop	rdi
140a5c65e: 5e                          	pop	rsi
140a5c65f: 41 5c                       	pop	r12
140a5c661: 41 5d                       	pop	r13
140a5c663: 41 5e                       	pop	r14
140a5c665: 41 5f                       	pop	r15
140a5c667: 5d                          	pop	rbp
140a5c668: c3                          	ret
140a5c669: 0f 1f 80 00 00 00 00        	nop	dword ptr [rax]
140a5c670: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5c675: 55                          	push	rbp
140a5c676: 41 57                       	push	r15
140a5c678: 41 56                       	push	r14
140a5c67a: 41 55                       	push	r13
140a5c67c: 41 54                       	push	r12
140a5c67e: 56                          	push	rsi
140a5c67f: 57                          	push	rdi
140a5c680: 53                          	push	rbx
140a5c681: 48 83 ec 48                 	sub	rsp, 0x48
140a5c685: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5c68c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5c691: 48 8b 95 b8 04 00 00        	mov	rdx, qword ptr [rbp + 0x4b8]
140a5c698: 48 85 d2                    	test	rdx, rdx
140a5c69b: 74 16                       	je	0x140a5c6b3 <.text+0xa5b6b3>
140a5c69d: 48 c1 e2 02                 	shl	rdx, 0x2
140a5c6a1: 41 b8 04 00 00 00           	mov	r8d, 0x4
140a5c6a7: 48 8b 8d 80 06 00 00        	mov	rcx, qword ptr [rbp + 0x680]
140a5c6ae: e8 6d 75 bb ff              	call	0x140613c20 <.text+0x612c20>
140a5c6b3: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5c6b8: 48 83 c4 48                 	add	rsp, 0x48
140a5c6bc: 5b                          	pop	rbx
140a5c6bd: 5f                          	pop	rdi
140a5c6be: 5e                          	pop	rsi
140a5c6bf: 41 5c                       	pop	r12
140a5c6c1: 41 5d                       	pop	r13
140a5c6c3: 41 5e                       	pop	r14
140a5c6c5: 41 5f                       	pop	r15
140a5c6c7: 5d                          	pop	rbp
140a5c6c8: c3                          	ret
140a5c6c9: 0f 1f 80 00 00 00 00        	nop	dword ptr [rax]
140a5c6d0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5c6d5: 55                          	push	rbp
140a5c6d6: 41 57                       	push	r15
140a5c6d8: 41 56                       	push	r14
140a5c6da: 41 55                       	push	r13
140a5c6dc: 41 54                       	push	r12
140a5c6de: 56                          	push	rsi
140a5c6df: 57                          	push	rdi
140a5c6e0: 53                          	push	rbx
140a5c6e1: 48 83 ec 48                 	sub	rsp, 0x48
140a5c6e5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5c6ec: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5c6f1: 48 8b 95 60 06 00 00        	mov	rdx, qword ptr [rbp + 0x660]
140a5c6f8: 48 85 d2                    	test	rdx, rdx
140a5c6fb: 74 16                       	je	0x140a5c713 <.text+0xa5b713>
140a5c6fd: 48 8b 8d 68 06 00 00        	mov	rcx, qword ptr [rbp + 0x668]
140a5c704: 48 c1 e2 02                 	shl	rdx, 0x2
140a5c708: 41 b8 04 00 00 00           	mov	r8d, 0x4
140a5c70e: e8 0d 75 bb ff              	call	0x140613c20 <.text+0x612c20>
140a5c713: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5c718: 48 83 c4 48                 	add	rsp, 0x48
140a5c71c: 5b                          	pop	rbx
140a5c71d: 5f                          	pop	rdi
140a5c71e: 5e                          	pop	rsi
140a5c71f: 41 5c                       	pop	r12
140a5c721: 41 5d                       	pop	r13
140a5c723: 41 5e                       	pop	r14
140a5c725: 41 5f                       	pop	r15
140a5c727: 5d                          	pop	rbp
140a5c728: c3                          	ret
140a5c729: 0f 1f 80 00 00 00 00        	nop	dword ptr [rax]
140a5c730: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5c735: 55                          	push	rbp
140a5c736: 41 57                       	push	r15
140a5c738: 41 56                       	push	r14
140a5c73a: 41 55                       	push	r13
140a5c73c: 41 54                       	push	r12
140a5c73e: 56                          	push	rsi
140a5c73f: 57                          	push	rdi
140a5c740: 53                          	push	rbx
140a5c741: 48 83 ec 48                 	sub	rsp, 0x48
140a5c745: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5c74c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5c751: 48 8b 8d 88 06 00 00        	mov	rcx, qword ptr [rbp + 0x688]
140a5c758: e8 d3 54 70 ff              	call	0x140161c30 <.text+0x160c30>
140a5c75d: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5c762: 48 83 c4 48                 	add	rsp, 0x48
140a5c766: 5b                          	pop	rbx
140a5c767: 5f                          	pop	rdi
140a5c768: 5e                          	pop	rsi
140a5c769: 41 5c                       	pop	r12
140a5c76b: 41 5d                       	pop	r13
140a5c76d: 41 5e                       	pop	r14
140a5c76f: 41 5f                       	pop	r15
140a5c771: 5d                          	pop	rbp
140a5c772: c3                          	ret
140a5c773: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a5c780: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5c785: 55                          	push	rbp
140a5c786: 41 57                       	push	r15
140a5c788: 41 56                       	push	r14
140a5c78a: 41 55                       	push	r13
140a5c78c: 41 54                       	push	r12
140a5c78e: 56                          	push	rsi
140a5c78f: 57                          	push	rdi
140a5c790: 53                          	push	rbx
140a5c791: 48 83 ec 48                 	sub	rsp, 0x48
140a5c795: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5c79c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5c7a1: 0f 10 85 e0 04 00 00        	movups	xmm0, xmmword ptr [rbp + 0x4e0]
140a5c7a8: 0f 10 8d f0 04 00 00        	movups	xmm1, xmmword ptr [rbp + 0x4f0]
140a5c7af: 0f 29 8d 10 03 00 00        	movaps	xmmword ptr [rbp + 0x310], xmm1
140a5c7b6: 0f 29 85 00 03 00 00        	movaps	xmmword ptr [rbp + 0x300], xmm0
140a5c7bd: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a5c7c4: e8 e7 5c 18 00              	call	0x140be24b0 <.text+0xbe14b0>
140a5c7c9: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5c7ce: 48 83 c4 48                 	add	rsp, 0x48
140a5c7d2: 5b                          	pop	rbx
140a5c7d3: 5f                          	pop	rdi
140a5c7d4: 5e                          	pop	rsi
140a5c7d5: 41 5c                       	pop	r12
140a5c7d7: 41 5d                       	pop	r13
140a5c7d9: 41 5e                       	pop	r14
140a5c7db: 41 5f                       	pop	r15
140a5c7dd: 5d                          	pop	rbp
140a5c7de: c3                          	ret
140a5c7df: 90                          	nop
140a5c7e0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5c7e5: 55                          	push	rbp
140a5c7e6: 41 57                       	push	r15
140a5c7e8: 41 56                       	push	r14
140a5c7ea: 41 55                       	push	r13
140a5c7ec: 41 54                       	push	r12
140a5c7ee: 56                          	push	rsi
140a5c7ef: 57                          	push	rdi
140a5c7f0: 53                          	push	rbx
140a5c7f1: 48 83 ec 48                 	sub	rsp, 0x48
140a5c7f5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5c7fc: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5c801: 80 bd f6 05 00 00 08        	cmp	byte ptr [rbp + 0x5f6], 0x8
140a5c808: 74 13                       	je	0x140a5c81d <.text+0xa5b81d>
140a5c80a: 48 8d 8d e0 05 00 00        	lea	rcx, [rbp + 0x5e0]
140a5c811: c6 85 a7 06 00 00 00        	mov	byte ptr [rbp + 0x6a7], 0x0
140a5c818: e8 93 5c 18 00              	call	0x140be24b0 <.text+0xbe14b0>
140a5c81d: c6 85 a7 06 00 00 00        	mov	byte ptr [rbp + 0x6a7], 0x0
140a5c824: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5c829: 48 83 c4 48                 	add	rsp, 0x48
140a5c82d: 5b                          	pop	rbx
140a5c82e: 5f                          	pop	rdi
140a5c82f: 5e                          	pop	rsi
140a5c830: 41 5c                       	pop	r12
140a5c832: 41 5d                       	pop	r13
140a5c834: 41 5e                       	pop	r14
140a5c836: 41 5f                       	pop	r15
140a5c838: 5d                          	pop	rbp
140a5c839: c3                          	ret
140a5c83a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
140a5c840: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5c845: 55                          	push	rbp
140a5c846: 41 57                       	push	r15
140a5c848: 41 56                       	push	r14
140a5c84a: 41 55                       	push	r13
140a5c84c: 41 54                       	push	r12
140a5c84e: 56                          	push	rsi
140a5c84f: 57                          	push	rdi
140a5c850: 53                          	push	rbx
140a5c851: 48 83 ec 48                 	sub	rsp, 0x48
140a5c855: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5c85c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5c861: 48 8d 8d c0 04 00 00        	lea	rcx, [rbp + 0x4c0]
140a5c868: e8 43 a7 e3 ff              	call	0x140896fb0 <.text+0x895fb0>
140a5c86d: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5c872: 48 83 c4 48                 	add	rsp, 0x48
140a5c876: 5b                          	pop	rbx
140a5c877: 5f                          	pop	rdi
140a5c878: 5e                          	pop	rsi
140a5c879: 41 5c                       	pop	r12
140a5c87b: 41 5d                       	pop	r13
140a5c87d: 41 5e                       	pop	r14
140a5c87f: 41 5f                       	pop	r15
140a5c881: 5d                          	pop	rbp
140a5c882: c3                          	ret
140a5c883: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a5c890: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5c895: 55                          	push	rbp
140a5c896: 41 57                       	push	r15
140a5c898: 41 56                       	push	r14
140a5c89a: 41 55                       	push	r13
140a5c89c: 41 54                       	push	r12
140a5c89e: 56                          	push	rsi
140a5c89f: 57                          	push	rdi
140a5c8a0: 53                          	push	rbx
140a5c8a1: 48 83 ec 48                 	sub	rsp, 0x48
140a5c8a5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5c8ac: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5c8b1: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a5c8b8: e8 f3 a6 e3 ff              	call	0x140896fb0 <.text+0x895fb0>
140a5c8bd: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5c8c2: 48 83 c4 48                 	add	rsp, 0x48
140a5c8c6: 5b                          	pop	rbx
140a5c8c7: 5f                          	pop	rdi
140a5c8c8: 5e                          	pop	rsi
140a5c8c9: 41 5c                       	pop	r12
140a5c8cb: 41 5d                       	pop	r13
140a5c8cd: 41 5e                       	pop	r14
140a5c8cf: 41 5f                       	pop	r15
140a5c8d1: 5d                          	pop	rbp
140a5c8d2: c3                          	ret
140a5c8d3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a5c8e0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5c8e5: 55                          	push	rbp
140a5c8e6: 41 57                       	push	r15
140a5c8e8: 41 56                       	push	r14
140a5c8ea: 41 55                       	push	r13
140a5c8ec: 41 54                       	push	r12
140a5c8ee: 56                          	push	rsi
140a5c8ef: 57                          	push	rdi
140a5c8f0: 53                          	push	rbx
140a5c8f1: 48 83 ec 48                 	sub	rsp, 0x48
140a5c8f5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5c8fc: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5c901: 80 bd 36 04 00 00 08        	cmp	byte ptr [rbp + 0x436], 0x8
140a5c908: 74 0c                       	je	0x140a5c916 <.text+0xa5b916>
140a5c90a: 48 8d 8d 20 04 00 00        	lea	rcx, [rbp + 0x420]
140a5c911: e8 9a 5b 18 00              	call	0x140be24b0 <.text+0xbe14b0>
140a5c916: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5c91b: 48 83 c4 48                 	add	rsp, 0x48
140a5c91f: 5b                          	pop	rbx
140a5c920: 5f                          	pop	rdi
140a5c921: 5e                          	pop	rsi
140a5c922: 41 5c                       	pop	r12
140a5c924: 41 5d                       	pop	r13
140a5c926: 41 5e                       	pop	r14
140a5c928: 41 5f                       	pop	r15
140a5c92a: 5d                          	pop	rbp
140a5c92b: c3                          	ret
140a5c92c: 0f 1f 40 00                 	nop	dword ptr [rax]
140a5c930: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5c935: 55                          	push	rbp
140a5c936: 41 57                       	push	r15
140a5c938: 41 56                       	push	r14
140a5c93a: 41 55                       	push	r13
140a5c93c: 41 54                       	push	r12
140a5c93e: 56                          	push	rsi
140a5c93f: 57                          	push	rdi
140a5c940: 53                          	push	rbx
140a5c941: 48 83 ec 48                 	sub	rsp, 0x48
140a5c945: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5c94c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5c951: 48 8d 8d c0 04 00 00        	lea	rcx, [rbp + 0x4c0]
140a5c958: e8 53 a6 e3 ff              	call	0x140896fb0 <.text+0x895fb0>
140a5c95d: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5c962: 48 83 c4 48                 	add	rsp, 0x48
140a5c966: 5b                          	pop	rbx
140a5c967: 5f                          	pop	rdi
140a5c968: 5e                          	pop	rsi
140a5c969: 41 5c                       	pop	r12
140a5c96b: 41 5d                       	pop	r13
140a5c96d: 41 5e                       	pop	r14
140a5c96f: 41 5f                       	pop	r15
140a5c971: 5d                          	pop	rbp
140a5c972: c3                          	ret
140a5c973: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a5c980: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5c985: 55                          	push	rbp
140a5c986: 41 57                       	push	r15
140a5c988: 41 56                       	push	r14
140a5c98a: 41 55                       	push	r13
140a5c98c: 41 54                       	push	r12
140a5c98e: 56                          	push	rsi
140a5c98f: 57                          	push	rdi
140a5c990: 53                          	push	rbx
140a5c991: 48 83 ec 48                 	sub	rsp, 0x48
140a5c995: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5c99c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5c9a1: 80 bd a7 06 00 00 00        	cmp	byte ptr [rbp + 0x6a7], 0x0
140a5c9a8: 74 26                       	je	0x140a5c9d0 <.text+0xa5b9d0>
140a5c9aa: 48 8d 4d e0                 	lea	rcx, [rbp - 0x20]
140a5c9ae: e8 2d 0e f1 ff              	call	0x14096d7e0 <.text+0x96c7e0>
140a5c9b3: c6 85 a6 06 00 00 00        	mov	byte ptr [rbp + 0x6a6], 0x0
140a5c9ba: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5c9bf: 48 83 c4 48                 	add	rsp, 0x48
140a5c9c3: 5b                          	pop	rbx
140a5c9c4: 5f                          	pop	rdi
140a5c9c5: 5e                          	pop	rsi
140a5c9c6: 41 5c                       	pop	r12
140a5c9c8: 41 5d                       	pop	r13
140a5c9ca: 41 5e                       	pop	r14
140a5c9cc: 41 5f                       	pop	r15
140a5c9ce: 5d                          	pop	rbp
140a5c9cf: c3                          	ret
140a5c9d0: c6 85 a6 06 00 00 00        	mov	byte ptr [rbp + 0x6a6], 0x0
140a5c9d7: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5c9dc: 48 83 c4 48                 	add	rsp, 0x48
140a5c9e0: 5b                          	pop	rbx
140a5c9e1: 5f                          	pop	rdi
140a5c9e2: 5e                          	pop	rsi
140a5c9e3: 41 5c                       	pop	r12
140a5c9e5: 41 5d                       	pop	r13
140a5c9e7: 41 5e                       	pop	r14
140a5c9e9: 41 5f                       	pop	r15
140a5c9eb: 5d                          	pop	rbp
140a5c9ec: c3                          	ret
140a5c9ed: 0f 1f 00                    	nop	dword ptr [rax]
140a5c9f0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5c9f5: 55                          	push	rbp
140a5c9f6: 41 57                       	push	r15
140a5c9f8: 41 56                       	push	r14
140a5c9fa: 41 55                       	push	r13
140a5c9fc: 41 54                       	push	r12
140a5c9fe: 56                          	push	rsi
140a5c9ff: 57                          	push	rdi
140a5ca00: 53                          	push	rbx
140a5ca01: 48 83 ec 48                 	sub	rsp, 0x48
140a5ca05: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5ca0c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5ca11: 80 bd a6 06 00 00 00        	cmp	byte ptr [rbp + 0x6a6], 0x0
140a5ca18: 74 3e                       	je	0x140a5ca58 <.text+0xa5ba58>
140a5ca1a: 0f 28 85 40 02 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x240]
140a5ca21: 0f 28 8d 50 02 00 00        	movaps	xmm1, xmmword ptr [rbp + 0x250]
140a5ca28: 0f 29 8d 00 02 00 00        	movaps	xmmword ptr [rbp + 0x200], xmm1
140a5ca2f: 0f 29 85 f0 01 00 00        	movaps	xmmword ptr [rbp + 0x1f0], xmm0
140a5ca36: 48 8d 8d f0 01 00 00        	lea	rcx, [rbp + 0x1f0]
140a5ca3d: e8 6e 5a 18 00              	call	0x140be24b0 <.text+0xbe14b0>
140a5ca42: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5ca47: 48 83 c4 48                 	add	rsp, 0x48
140a5ca4b: 5b                          	pop	rbx
140a5ca4c: 5f                          	pop	rdi
140a5ca4d: 5e                          	pop	rsi
140a5ca4e: 41 5c                       	pop	r12
140a5ca50: 41 5d                       	pop	r13
140a5ca52: 41 5e                       	pop	r14
140a5ca54: 41 5f                       	pop	r15
140a5ca56: 5d                          	pop	rbp
140a5ca57: c3                          	ret
140a5ca58: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5ca5d: 48 83 c4 48                 	add	rsp, 0x48
140a5ca61: 5b                          	pop	rbx
140a5ca62: 5f                          	pop	rdi
140a5ca63: 5e                          	pop	rsi
140a5ca64: 41 5c                       	pop	r12
140a5ca66: 41 5d                       	pop	r13
140a5ca68: 41 5e                       	pop	r14
140a5ca6a: 41 5f                       	pop	r15
140a5ca6c: 5d                          	pop	rbp
140a5ca6d: c3                          	ret
140a5ca6e: 66 90                       	nop
140a5ca70: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5ca75: 55                          	push	rbp
140a5ca76: 41 57                       	push	r15
140a5ca78: 41 56                       	push	r14
140a5ca7a: 41 55                       	push	r13
140a5ca7c: 41 54                       	push	r12
140a5ca7e: 56                          	push	rsi
140a5ca7f: 57                          	push	rdi
140a5ca80: 53                          	push	rbx
140a5ca81: 48 83 ec 48                 	sub	rsp, 0x48
140a5ca85: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5ca8c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5ca91: 48 8d 8d c0 04 00 00        	lea	rcx, [rbp + 0x4c0]
140a5ca98: e8 13 a5 e3 ff              	call	0x140896fb0 <.text+0x895fb0>
140a5ca9d: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5caa2: 48 83 c4 48                 	add	rsp, 0x48
140a5caa6: 5b                          	pop	rbx
140a5caa7: 5f                          	pop	rdi
140a5caa8: 5e                          	pop	rsi
140a5caa9: 41 5c                       	pop	r12
140a5caab: 41 5d                       	pop	r13
140a5caad: 41 5e                       	pop	r14
140a5caaf: 41 5f                       	pop	r15
140a5cab1: 5d                          	pop	rbp
140a5cab2: c3                          	ret
140a5cab3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a5cac0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5cac5: 55                          	push	rbp
140a5cac6: 41 57                       	push	r15
140a5cac8: 41 56                       	push	r14
140a5caca: 41 55                       	push	r13
140a5cacc: 41 54                       	push	r12
140a5cace: 56                          	push	rsi
140a5cacf: 57                          	push	rdi
140a5cad0: 53                          	push	rbx
140a5cad1: 48 83 ec 48                 	sub	rsp, 0x48
140a5cad5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5cadc: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5cae1: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a5cae8: e8 c3 a4 e3 ff              	call	0x140896fb0 <.text+0x895fb0>
140a5caed: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5caf2: 48 83 c4 48                 	add	rsp, 0x48
140a5caf6: 5b                          	pop	rbx
140a5caf7: 5f                          	pop	rdi
140a5caf8: 5e                          	pop	rsi
140a5caf9: 41 5c                       	pop	r12
140a5cafb: 41 5d                       	pop	r13
140a5cafd: 41 5e                       	pop	r14
140a5caff: 41 5f                       	pop	r15
140a5cb01: 5d                          	pop	rbp
140a5cb02: c3                          	ret
140a5cb03: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a5cb10: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5cb15: 55                          	push	rbp
140a5cb16: 41 57                       	push	r15
140a5cb18: 41 56                       	push	r14
140a5cb1a: 41 55                       	push	r13
140a5cb1c: 41 54                       	push	r12
140a5cb1e: 56                          	push	rsi
140a5cb1f: 57                          	push	rdi
140a5cb20: 53                          	push	rbx
140a5cb21: 48 83 ec 48                 	sub	rsp, 0x48
140a5cb25: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5cb2c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5cb31: 48 8d 8d c0 04 00 00        	lea	rcx, [rbp + 0x4c0]
140a5cb38: e8 73 a4 e3 ff              	call	0x140896fb0 <.text+0x895fb0>
140a5cb3d: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5cb42: 48 83 c4 48                 	add	rsp, 0x48
140a5cb46: 5b                          	pop	rbx
140a5cb47: 5f                          	pop	rdi
140a5cb48: 5e                          	pop	rsi
140a5cb49: 41 5c                       	pop	r12
140a5cb4b: 41 5d                       	pop	r13
140a5cb4d: 41 5e                       	pop	r14
140a5cb4f: 41 5f                       	pop	r15
140a5cb51: 5d                          	pop	rbp
140a5cb52: c3                          	ret
140a5cb53: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a5cb60: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5cb65: 55                          	push	rbp
140a5cb66: 41 57                       	push	r15
140a5cb68: 41 56                       	push	r14
140a5cb6a: 41 55                       	push	r13
140a5cb6c: 41 54                       	push	r12
140a5cb6e: 56                          	push	rsi
140a5cb6f: 57                          	push	rdi
140a5cb70: 53                          	push	rbx
140a5cb71: 48 83 ec 48                 	sub	rsp, 0x48
140a5cb75: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5cb7c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5cb81: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a5cb88: e8 23 a4 e3 ff              	call	0x140896fb0 <.text+0x895fb0>
140a5cb8d: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5cb92: 48 83 c4 48                 	add	rsp, 0x48
140a5cb96: 5b                          	pop	rbx
140a5cb97: 5f                          	pop	rdi
140a5cb98: 5e                          	pop	rsi
140a5cb99: 41 5c                       	pop	r12
140a5cb9b: 41 5d                       	pop	r13
140a5cb9d: 41 5e                       	pop	r14
140a5cb9f: 41 5f                       	pop	r15
140a5cba1: 5d                          	pop	rbp
140a5cba2: c3                          	ret
140a5cba3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a5cbb0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5cbb5: 55                          	push	rbp
140a5cbb6: 41 57                       	push	r15
140a5cbb8: 41 56                       	push	r14
140a5cbba: 41 55                       	push	r13
140a5cbbc: 41 54                       	push	r12
140a5cbbe: 56                          	push	rsi
140a5cbbf: 57                          	push	rdi
140a5cbc0: 53                          	push	rbx
140a5cbc1: 48 83 ec 48                 	sub	rsp, 0x48
140a5cbc5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5cbcc: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5cbd1: 48 8d 8d c0 04 00 00        	lea	rcx, [rbp + 0x4c0]
140a5cbd8: e8 d3 a3 e3 ff              	call	0x140896fb0 <.text+0x895fb0>
140a5cbdd: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5cbe2: 48 83 c4 48                 	add	rsp, 0x48
140a5cbe6: 5b                          	pop	rbx
140a5cbe7: 5f                          	pop	rdi
140a5cbe8: 5e                          	pop	rsi
140a5cbe9: 41 5c                       	pop	r12
140a5cbeb: 41 5d                       	pop	r13
140a5cbed: 41 5e                       	pop	r14
140a5cbef: 41 5f                       	pop	r15
140a5cbf1: 5d                          	pop	rbp
140a5cbf2: c3                          	ret
140a5cbf3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a5cc00: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5cc05: 55                          	push	rbp
140a5cc06: 41 57                       	push	r15
140a5cc08: 41 56                       	push	r14
140a5cc0a: 41 55                       	push	r13
140a5cc0c: 41 54                       	push	r12
140a5cc0e: 56                          	push	rsi
140a5cc0f: 57                          	push	rdi
140a5cc10: 53                          	push	rbx
140a5cc11: 48 83 ec 48                 	sub	rsp, 0x48
140a5cc15: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5cc1c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5cc21: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a5cc28: e8 83 a3 e3 ff              	call	0x140896fb0 <.text+0x895fb0>
140a5cc2d: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5cc32: 48 83 c4 48                 	add	rsp, 0x48
140a5cc36: 5b                          	pop	rbx
140a5cc37: 5f                          	pop	rdi
140a5cc38: 5e                          	pop	rsi
140a5cc39: 41 5c                       	pop	r12
140a5cc3b: 41 5d                       	pop	r13
140a5cc3d: 41 5e                       	pop	r14
140a5cc3f: 41 5f                       	pop	r15
140a5cc41: 5d                          	pop	rbp
140a5cc42: c3                          	ret
140a5cc43: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a5cc50: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5cc55: 55                          	push	rbp
140a5cc56: 41 57                       	push	r15
140a5cc58: 41 56                       	push	r14
140a5cc5a: 41 55                       	push	r13
140a5cc5c: 41 54                       	push	r12
140a5cc5e: 56                          	push	rsi
140a5cc5f: 57                          	push	rdi
140a5cc60: 53                          	push	rbx
140a5cc61: 48 83 ec 48                 	sub	rsp, 0x48
140a5cc65: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5cc6c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5cc71: c6 85 a7 06 00 00 00        	mov	byte ptr [rbp + 0x6a7], 0x0
140a5cc78: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a5cc7f: e8 2c a3 e3 ff              	call	0x140896fb0 <.text+0x895fb0>
140a5cc84: c6 85 a7 06 00 00 00        	mov	byte ptr [rbp + 0x6a7], 0x0
140a5cc8b: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5cc90: 48 83 c4 48                 	add	rsp, 0x48
140a5cc94: 5b                          	pop	rbx
140a5cc95: 5f                          	pop	rdi
140a5cc96: 5e                          	pop	rsi
140a5cc97: 41 5c                       	pop	r12
140a5cc99: 41 5d                       	pop	r13
140a5cc9b: 41 5e                       	pop	r14
140a5cc9d: 41 5f                       	pop	r15
140a5cc9f: 5d                          	pop	rbp
140a5cca0: c3                          	ret
140a5cca1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
140a5ccb0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5ccb5: 55                          	push	rbp
140a5ccb6: 41 57                       	push	r15
140a5ccb8: 41 56                       	push	r14
140a5ccba: 41 55                       	push	r13
140a5ccbc: 41 54                       	push	r12
140a5ccbe: 56                          	push	rsi
140a5ccbf: 57                          	push	rdi
140a5ccc0: 53                          	push	rbx
140a5ccc1: 48 83 ec 48                 	sub	rsp, 0x48
140a5ccc5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5cccc: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5ccd1: 48 8d 8d c0 04 00 00        	lea	rcx, [rbp + 0x4c0]
140a5ccd8: e8 d3 a2 e3 ff              	call	0x140896fb0 <.text+0x895fb0>
140a5ccdd: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5cce2: 48 83 c4 48                 	add	rsp, 0x48
140a5cce6: 5b                          	pop	rbx
140a5cce7: 5f                          	pop	rdi
140a5cce8: 5e                          	pop	rsi
140a5cce9: 41 5c                       	pop	r12
140a5cceb: 41 5d                       	pop	r13
140a5cced: 41 5e                       	pop	r14
140a5ccef: 41 5f                       	pop	r15
140a5ccf1: 5d                          	pop	rbp
140a5ccf2: c3                          	ret
140a5ccf3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a5cd00: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5cd05: 55                          	push	rbp
140a5cd06: 41 57                       	push	r15
140a5cd08: 41 56                       	push	r14
140a5cd0a: 41 55                       	push	r13
140a5cd0c: 41 54                       	push	r12
140a5cd0e: 56                          	push	rsi
140a5cd0f: 57                          	push	rdi
140a5cd10: 53                          	push	rbx
140a5cd11: 48 83 ec 48                 	sub	rsp, 0x48
140a5cd15: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5cd1c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5cd21: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a5cd28: e8 83 a2 e3 ff              	call	0x140896fb0 <.text+0x895fb0>
140a5cd2d: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5cd32: 48 83 c4 48                 	add	rsp, 0x48
140a5cd36: 5b                          	pop	rbx
140a5cd37: 5f                          	pop	rdi
140a5cd38: 5e                          	pop	rsi
140a5cd39: 41 5c                       	pop	r12
140a5cd3b: 41 5d                       	pop	r13
140a5cd3d: 41 5e                       	pop	r14
140a5cd3f: 41 5f                       	pop	r15
140a5cd41: 5d                          	pop	rbp
140a5cd42: c3                          	ret
140a5cd43: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a5cd50: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5cd55: 55                          	push	rbp
140a5cd56: 41 57                       	push	r15
140a5cd58: 41 56                       	push	r14
140a5cd5a: 41 55                       	push	r13
140a5cd5c: 41 54                       	push	r12
140a5cd5e: 56                          	push	rsi
140a5cd5f: 57                          	push	rdi
140a5cd60: 53                          	push	rbx
140a5cd61: 48 83 ec 48                 	sub	rsp, 0x48
140a5cd65: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5cd6c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5cd71: 48 8d 8d c0 04 00 00        	lea	rcx, [rbp + 0x4c0]
140a5cd78: e8 33 a2 e3 ff              	call	0x140896fb0 <.text+0x895fb0>
140a5cd7d: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5cd82: 48 83 c4 48                 	add	rsp, 0x48
140a5cd86: 5b                          	pop	rbx
140a5cd87: 5f                          	pop	rdi
140a5cd88: 5e                          	pop	rsi
140a5cd89: 41 5c                       	pop	r12
140a5cd8b: 41 5d                       	pop	r13
140a5cd8d: 41 5e                       	pop	r14
140a5cd8f: 41 5f                       	pop	r15
140a5cd91: 5d                          	pop	rbp
140a5cd92: c3                          	ret
140a5cd93: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a5cda0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5cda5: 55                          	push	rbp
140a5cda6: 41 57                       	push	r15
140a5cda8: 41 56                       	push	r14
140a5cdaa: 41 55                       	push	r13
140a5cdac: 41 54                       	push	r12
140a5cdae: 56                          	push	rsi
140a5cdaf: 57                          	push	rdi
140a5cdb0: 53                          	push	rbx
140a5cdb1: 48 83 ec 48                 	sub	rsp, 0x48
140a5cdb5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5cdbc: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5cdc1: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a5cdc8: e8 e3 a1 e3 ff              	call	0x140896fb0 <.text+0x895fb0>
140a5cdcd: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5cdd2: 48 83 c4 48                 	add	rsp, 0x48
140a5cdd6: 5b                          	pop	rbx
140a5cdd7: 5f                          	pop	rdi
140a5cdd8: 5e                          	pop	rsi
140a5cdd9: 41 5c                       	pop	r12
140a5cddb: 41 5d                       	pop	r13
140a5cddd: 41 5e                       	pop	r14
140a5cddf: 41 5f                       	pop	r15
140a5cde1: 5d                          	pop	rbp
140a5cde2: c3                          	ret
140a5cde3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a5cdf0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5cdf5: 55                          	push	rbp
140a5cdf6: 41 57                       	push	r15
140a5cdf8: 41 56                       	push	r14
140a5cdfa: 41 55                       	push	r13
140a5cdfc: 41 54                       	push	r12
140a5cdfe: 56                          	push	rsi
140a5cdff: 57                          	push	rdi
140a5ce00: 53                          	push	rbx
140a5ce01: 48 83 ec 48                 	sub	rsp, 0x48
140a5ce05: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5ce0c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5ce11: 48 8d 8d c0 04 00 00        	lea	rcx, [rbp + 0x4c0]
140a5ce18: e8 93 a1 e3 ff              	call	0x140896fb0 <.text+0x895fb0>
140a5ce1d: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5ce22: 48 83 c4 48                 	add	rsp, 0x48
140a5ce26: 5b                          	pop	rbx
140a5ce27: 5f                          	pop	rdi
140a5ce28: 5e                          	pop	rsi
140a5ce29: 41 5c                       	pop	r12
140a5ce2b: 41 5d                       	pop	r13
140a5ce2d: 41 5e                       	pop	r14
140a5ce2f: 41 5f                       	pop	r15
140a5ce31: 5d                          	pop	rbp
140a5ce32: c3                          	ret
140a5ce33: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a5ce40: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5ce45: 55                          	push	rbp
140a5ce46: 41 57                       	push	r15
140a5ce48: 41 56                       	push	r14
140a5ce4a: 41 55                       	push	r13
140a5ce4c: 41 54                       	push	r12
140a5ce4e: 56                          	push	rsi
140a5ce4f: 57                          	push	rdi
140a5ce50: 53                          	push	rbx
140a5ce51: 48 83 ec 48                 	sub	rsp, 0x48
140a5ce55: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5ce5c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5ce61: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a5ce68: e8 43 a1 e3 ff              	call	0x140896fb0 <.text+0x895fb0>
140a5ce6d: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5ce72: 48 83 c4 48                 	add	rsp, 0x48
140a5ce76: 5b                          	pop	rbx
140a5ce77: 5f                          	pop	rdi
140a5ce78: 5e                          	pop	rsi
140a5ce79: 41 5c                       	pop	r12
140a5ce7b: 41 5d                       	pop	r13
140a5ce7d: 41 5e                       	pop	r14
140a5ce7f: 41 5f                       	pop	r15
140a5ce81: 5d                          	pop	rbp
140a5ce82: c3                          	ret
140a5ce83: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a5ce90: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5ce95: 55                          	push	rbp
140a5ce96: 41 57                       	push	r15
140a5ce98: 41 56                       	push	r14
140a5ce9a: 41 55                       	push	r13
140a5ce9c: 41 54                       	push	r12
140a5ce9e: 56                          	push	rsi
140a5ce9f: 57                          	push	rdi
140a5cea0: 53                          	push	rbx
140a5cea1: 48 83 ec 48                 	sub	rsp, 0x48
140a5cea5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5ceac: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5ceb1: 48 8d 8d c0 04 00 00        	lea	rcx, [rbp + 0x4c0]
140a5ceb8: e8 f3 a0 e3 ff              	call	0x140896fb0 <.text+0x895fb0>
140a5cebd: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5cec2: 48 83 c4 48                 	add	rsp, 0x48
140a5cec6: 5b                          	pop	rbx
140a5cec7: 5f                          	pop	rdi
140a5cec8: 5e                          	pop	rsi
140a5cec9: 41 5c                       	pop	r12
140a5cecb: 41 5d                       	pop	r13
140a5cecd: 41 5e                       	pop	r14
140a5cecf: 41 5f                       	pop	r15
140a5ced1: 5d                          	pop	rbp
140a5ced2: c3                          	ret
140a5ced3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a5cee0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5cee5: 55                          	push	rbp
140a5cee6: 41 57                       	push	r15
140a5cee8: 41 56                       	push	r14
140a5ceea: 41 55                       	push	r13
140a5ceec: 41 54                       	push	r12
140a5ceee: 56                          	push	rsi
140a5ceef: 57                          	push	rdi
140a5cef0: 53                          	push	rbx
140a5cef1: 48 83 ec 48                 	sub	rsp, 0x48
140a5cef5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5cefc: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5cf01: 48 8d 8d c0 04 00 00        	lea	rcx, [rbp + 0x4c0]
140a5cf08: e8 a3 a0 e3 ff              	call	0x140896fb0 <.text+0x895fb0>
140a5cf0d: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5cf12: 48 83 c4 48                 	add	rsp, 0x48
140a5cf16: 5b                          	pop	rbx
140a5cf17: 5f                          	pop	rdi
140a5cf18: 5e                          	pop	rsi
140a5cf19: 41 5c                       	pop	r12
140a5cf1b: 41 5d                       	pop	r13
140a5cf1d: 41 5e                       	pop	r14
140a5cf1f: 41 5f                       	pop	r15
140a5cf21: 5d                          	pop	rbp
140a5cf22: c3                          	ret
140a5cf23: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a5cf30: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5cf35: 55                          	push	rbp
140a5cf36: 41 57                       	push	r15
140a5cf38: 41 56                       	push	r14
140a5cf3a: 41 55                       	push	r13
140a5cf3c: 41 54                       	push	r12
140a5cf3e: 56                          	push	rsi
140a5cf3f: 57                          	push	rdi
140a5cf40: 53                          	push	rbx
140a5cf41: 48 83 ec 48                 	sub	rsp, 0x48
140a5cf45: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5cf4c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5cf51: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a5cf58: e8 53 a0 e3 ff              	call	0x140896fb0 <.text+0x895fb0>
140a5cf5d: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5cf62: 48 83 c4 48                 	add	rsp, 0x48
140a5cf66: 5b                          	pop	rbx
140a5cf67: 5f                          	pop	rdi
140a5cf68: 5e                          	pop	rsi
140a5cf69: 41 5c                       	pop	r12
140a5cf6b: 41 5d                       	pop	r13
140a5cf6d: 41 5e                       	pop	r14
140a5cf6f: 41 5f                       	pop	r15
140a5cf71: 5d                          	pop	rbp
140a5cf72: c3                          	ret
140a5cf73: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a5cf80: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5cf85: 55                          	push	rbp
140a5cf86: 41 57                       	push	r15
140a5cf88: 41 56                       	push	r14
140a5cf8a: 41 55                       	push	r13
140a5cf8c: 41 54                       	push	r12
140a5cf8e: 56                          	push	rsi
140a5cf8f: 57                          	push	rdi
140a5cf90: 53                          	push	rbx
140a5cf91: 48 83 ec 48                 	sub	rsp, 0x48
140a5cf95: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5cf9c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5cfa1: 48 8d 8d c0 04 00 00        	lea	rcx, [rbp + 0x4c0]
140a5cfa8: e8 03 a0 e3 ff              	call	0x140896fb0 <.text+0x895fb0>
140a5cfad: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5cfb2: 48 83 c4 48                 	add	rsp, 0x48
140a5cfb6: 5b                          	pop	rbx
140a5cfb7: 5f                          	pop	rdi
140a5cfb8: 5e                          	pop	rsi
140a5cfb9: 41 5c                       	pop	r12
140a5cfbb: 41 5d                       	pop	r13
140a5cfbd: 41 5e                       	pop	r14
140a5cfbf: 41 5f                       	pop	r15
140a5cfc1: 5d                          	pop	rbp
140a5cfc2: c3                          	ret
140a5cfc3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a5cfd0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5cfd5: 55                          	push	rbp
140a5cfd6: 41 57                       	push	r15
140a5cfd8: 41 56                       	push	r14
140a5cfda: 41 55                       	push	r13
140a5cfdc: 41 54                       	push	r12
140a5cfde: 56                          	push	rsi
140a5cfdf: 57                          	push	rdi
140a5cfe0: 53                          	push	rbx
140a5cfe1: 48 83 ec 48                 	sub	rsp, 0x48
140a5cfe5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5cfec: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5cff1: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a5cff8: e8 b3 9f e3 ff              	call	0x140896fb0 <.text+0x895fb0>
140a5cffd: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5d002: 48 83 c4 48                 	add	rsp, 0x48
140a5d006: 5b                          	pop	rbx
140a5d007: 5f                          	pop	rdi
140a5d008: 5e                          	pop	rsi
140a5d009: 41 5c                       	pop	r12
140a5d00b: 41 5d                       	pop	r13
140a5d00d: 41 5e                       	pop	r14
140a5d00f: 41 5f                       	pop	r15
140a5d011: 5d                          	pop	rbp
140a5d012: c3                          	ret
140a5d013: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a5d020: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5d025: 55                          	push	rbp
140a5d026: 41 57                       	push	r15
140a5d028: 41 56                       	push	r14
140a5d02a: 41 55                       	push	r13
140a5d02c: 41 54                       	push	r12
140a5d02e: 56                          	push	rsi
140a5d02f: 57                          	push	rdi
140a5d030: 53                          	push	rbx
140a5d031: 48 83 ec 48                 	sub	rsp, 0x48
140a5d035: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5d03c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5d041: 48 8d 8d c0 04 00 00        	lea	rcx, [rbp + 0x4c0]
140a5d048: e8 63 9f e3 ff              	call	0x140896fb0 <.text+0x895fb0>
140a5d04d: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5d052: 48 83 c4 48                 	add	rsp, 0x48
140a5d056: 5b                          	pop	rbx
140a5d057: 5f                          	pop	rdi
140a5d058: 5e                          	pop	rsi
140a5d059: 41 5c                       	pop	r12
140a5d05b: 41 5d                       	pop	r13
140a5d05d: 41 5e                       	pop	r14
140a5d05f: 41 5f                       	pop	r15
140a5d061: 5d                          	pop	rbp
140a5d062: c3                          	ret
140a5d063: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a5d070: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5d075: 55                          	push	rbp
140a5d076: 41 57                       	push	r15
140a5d078: 41 56                       	push	r14
140a5d07a: 41 55                       	push	r13
140a5d07c: 41 54                       	push	r12
140a5d07e: 56                          	push	rsi
140a5d07f: 57                          	push	rdi
140a5d080: 53                          	push	rbx
140a5d081: 48 83 ec 48                 	sub	rsp, 0x48
140a5d085: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5d08c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5d091: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a5d098: e8 13 9f e3 ff              	call	0x140896fb0 <.text+0x895fb0>
140a5d09d: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5d0a2: 48 83 c4 48                 	add	rsp, 0x48
140a5d0a6: 5b                          	pop	rbx
140a5d0a7: 5f                          	pop	rdi
140a5d0a8: 5e                          	pop	rsi
140a5d0a9: 41 5c                       	pop	r12
140a5d0ab: 41 5d                       	pop	r13
140a5d0ad: 41 5e                       	pop	r14
140a5d0af: 41 5f                       	pop	r15
140a5d0b1: 5d                          	pop	rbp
140a5d0b2: c3                          	ret
140a5d0b3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a5d0c0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5d0c5: 55                          	push	rbp
140a5d0c6: 41 57                       	push	r15
140a5d0c8: 41 56                       	push	r14
140a5d0ca: 41 55                       	push	r13
140a5d0cc: 41 54                       	push	r12
140a5d0ce: 56                          	push	rsi
140a5d0cf: 57                          	push	rdi
140a5d0d0: 53                          	push	rbx
140a5d0d1: 48 83 ec 48                 	sub	rsp, 0x48
140a5d0d5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5d0dc: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5d0e1: 48 8d 8d c0 04 00 00        	lea	rcx, [rbp + 0x4c0]
140a5d0e8: e8 c3 9e e3 ff              	call	0x140896fb0 <.text+0x895fb0>
140a5d0ed: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5d0f2: 48 83 c4 48                 	add	rsp, 0x48
140a5d0f6: 5b                          	pop	rbx
140a5d0f7: 5f                          	pop	rdi
140a5d0f8: 5e                          	pop	rsi
140a5d0f9: 41 5c                       	pop	r12
140a5d0fb: 41 5d                       	pop	r13
140a5d0fd: 41 5e                       	pop	r14
140a5d0ff: 41 5f                       	pop	r15
140a5d101: 5d                          	pop	rbp
140a5d102: c3                          	ret
140a5d103: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a5d110: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5d115: 55                          	push	rbp
140a5d116: 41 57                       	push	r15
140a5d118: 41 56                       	push	r14
140a5d11a: 41 55                       	push	r13
140a5d11c: 41 54                       	push	r12
140a5d11e: 56                          	push	rsi
140a5d11f: 57                          	push	rdi
140a5d120: 53                          	push	rbx
140a5d121: 48 83 ec 48                 	sub	rsp, 0x48
140a5d125: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5d12c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5d131: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a5d138: e8 73 9e e3 ff              	call	0x140896fb0 <.text+0x895fb0>
140a5d13d: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5d142: 48 83 c4 48                 	add	rsp, 0x48
140a5d146: 5b                          	pop	rbx
140a5d147: 5f                          	pop	rdi
140a5d148: 5e                          	pop	rsi
140a5d149: 41 5c                       	pop	r12
140a5d14b: 41 5d                       	pop	r13
140a5d14d: 41 5e                       	pop	r14
140a5d14f: 41 5f                       	pop	r15
140a5d151: 5d                          	pop	rbp
140a5d152: c3                          	ret
140a5d153: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a5d160: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5d165: 55                          	push	rbp
140a5d166: 41 57                       	push	r15
140a5d168: 41 56                       	push	r14
140a5d16a: 41 55                       	push	r13
140a5d16c: 41 54                       	push	r12
140a5d16e: 56                          	push	rsi
140a5d16f: 57                          	push	rdi
140a5d170: 53                          	push	rbx
140a5d171: 48 83 ec 48                 	sub	rsp, 0x48
140a5d175: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5d17c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5d181: 48 8d 8d c0 04 00 00        	lea	rcx, [rbp + 0x4c0]
140a5d188: e8 23 9e e3 ff              	call	0x140896fb0 <.text+0x895fb0>
140a5d18d: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5d192: 48 83 c4 48                 	add	rsp, 0x48
140a5d196: 5b                          	pop	rbx
140a5d197: 5f                          	pop	rdi
140a5d198: 5e                          	pop	rsi
140a5d199: 41 5c                       	pop	r12
140a5d19b: 41 5d                       	pop	r13
140a5d19d: 41 5e                       	pop	r14
140a5d19f: 41 5f                       	pop	r15
140a5d1a1: 5d                          	pop	rbp
140a5d1a2: c3                          	ret
140a5d1a3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a5d1b0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5d1b5: 55                          	push	rbp
140a5d1b6: 41 57                       	push	r15
140a5d1b8: 41 56                       	push	r14
140a5d1ba: 41 55                       	push	r13
140a5d1bc: 41 54                       	push	r12
140a5d1be: 56                          	push	rsi
140a5d1bf: 57                          	push	rdi
140a5d1c0: 53                          	push	rbx
140a5d1c1: 48 83 ec 48                 	sub	rsp, 0x48
140a5d1c5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5d1cc: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5d1d1: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a5d1d8: e8 d3 9d e3 ff              	call	0x140896fb0 <.text+0x895fb0>
140a5d1dd: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5d1e2: 48 83 c4 48                 	add	rsp, 0x48
140a5d1e6: 5b                          	pop	rbx
140a5d1e7: 5f                          	pop	rdi
140a5d1e8: 5e                          	pop	rsi
140a5d1e9: 41 5c                       	pop	r12
140a5d1eb: 41 5d                       	pop	r13
140a5d1ed: 41 5e                       	pop	r14
140a5d1ef: 41 5f                       	pop	r15
140a5d1f1: 5d                          	pop	rbp
140a5d1f2: c3                          	ret
140a5d1f3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a5d200: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5d205: 55                          	push	rbp
140a5d206: 41 57                       	push	r15
140a5d208: 41 56                       	push	r14
140a5d20a: 41 55                       	push	r13
140a5d20c: 41 54                       	push	r12
140a5d20e: 56                          	push	rsi
140a5d20f: 57                          	push	rdi
140a5d210: 53                          	push	rbx
140a5d211: 48 83 ec 48                 	sub	rsp, 0x48
140a5d215: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5d21c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5d221: 48 8d 8d c0 04 00 00        	lea	rcx, [rbp + 0x4c0]
140a5d228: e8 83 9d e3 ff              	call	0x140896fb0 <.text+0x895fb0>
140a5d22d: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5d232: 48 83 c4 48                 	add	rsp, 0x48
140a5d236: 5b                          	pop	rbx
140a5d237: 5f                          	pop	rdi
140a5d238: 5e                          	pop	rsi
140a5d239: 41 5c                       	pop	r12
140a5d23b: 41 5d                       	pop	r13
140a5d23d: 41 5e                       	pop	r14
140a5d23f: 41 5f                       	pop	r15
140a5d241: 5d                          	pop	rbp
140a5d242: c3                          	ret
140a5d243: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a5d250: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5d255: 55                          	push	rbp
140a5d256: 41 57                       	push	r15
140a5d258: 41 56                       	push	r14
140a5d25a: 41 55                       	push	r13
140a5d25c: 41 54                       	push	r12
140a5d25e: 56                          	push	rsi
140a5d25f: 57                          	push	rdi
140a5d260: 53                          	push	rbx
140a5d261: 48 83 ec 48                 	sub	rsp, 0x48
140a5d265: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5d26c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5d271: 48 8d 8d c0 04 00 00        	lea	rcx, [rbp + 0x4c0]
140a5d278: e8 33 9d e3 ff              	call	0x140896fb0 <.text+0x895fb0>
140a5d27d: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5d282: 48 83 c4 48                 	add	rsp, 0x48
140a5d286: 5b                          	pop	rbx
140a5d287: 5f                          	pop	rdi
140a5d288: 5e                          	pop	rsi
140a5d289: 41 5c                       	pop	r12
140a5d28b: 41 5d                       	pop	r13
140a5d28d: 41 5e                       	pop	r14
140a5d28f: 41 5f                       	pop	r15
140a5d291: 5d                          	pop	rbp
140a5d292: c3                          	ret
140a5d293: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a5d2a0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5d2a5: 55                          	push	rbp
140a5d2a6: 41 57                       	push	r15
140a5d2a8: 41 56                       	push	r14
140a5d2aa: 41 55                       	push	r13
140a5d2ac: 41 54                       	push	r12
140a5d2ae: 56                          	push	rsi
140a5d2af: 57                          	push	rdi
140a5d2b0: 53                          	push	rbx
140a5d2b1: 48 83 ec 48                 	sub	rsp, 0x48
140a5d2b5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5d2bc: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5d2c1: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a5d2c8: e8 e3 9c e3 ff              	call	0x140896fb0 <.text+0x895fb0>
140a5d2cd: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5d2d2: 48 83 c4 48                 	add	rsp, 0x48
140a5d2d6: 5b                          	pop	rbx
140a5d2d7: 5f                          	pop	rdi
140a5d2d8: 5e                          	pop	rsi
140a5d2d9: 41 5c                       	pop	r12
140a5d2db: 41 5d                       	pop	r13
140a5d2dd: 41 5e                       	pop	r14
140a5d2df: 41 5f                       	pop	r15
140a5d2e1: 5d                          	pop	rbp
140a5d2e2: c3                          	ret
140a5d2e3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a5d2f0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5d2f5: 55                          	push	rbp
140a5d2f6: 41 57                       	push	r15
140a5d2f8: 41 56                       	push	r14
140a5d2fa: 41 55                       	push	r13
140a5d2fc: 41 54                       	push	r12
140a5d2fe: 56                          	push	rsi
140a5d2ff: 57                          	push	rdi
140a5d300: 53                          	push	rbx
140a5d301: 48 83 ec 48                 	sub	rsp, 0x48
140a5d305: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5d30c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5d311: 48 8d 8d c0 04 00 00        	lea	rcx, [rbp + 0x4c0]
140a5d318: e8 93 9c e3 ff              	call	0x140896fb0 <.text+0x895fb0>
140a5d31d: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5d322: 48 83 c4 48                 	add	rsp, 0x48
140a5d326: 5b                          	pop	rbx
140a5d327: 5f                          	pop	rdi
140a5d328: 5e                          	pop	rsi
140a5d329: 41 5c                       	pop	r12
140a5d32b: 41 5d                       	pop	r13
140a5d32d: 41 5e                       	pop	r14
140a5d32f: 41 5f                       	pop	r15
140a5d331: 5d                          	pop	rbp
140a5d332: c3                          	ret
140a5d333: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a5d340: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5d345: 55                          	push	rbp
140a5d346: 41 57                       	push	r15
140a5d348: 41 56                       	push	r14
140a5d34a: 41 55                       	push	r13
140a5d34c: 41 54                       	push	r12
140a5d34e: 56                          	push	rsi
140a5d34f: 57                          	push	rdi
140a5d350: 53                          	push	rbx
140a5d351: 48 83 ec 48                 	sub	rsp, 0x48
140a5d355: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5d35c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5d361: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a5d368: e8 43 9c e3 ff              	call	0x140896fb0 <.text+0x895fb0>
140a5d36d: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5d372: 48 83 c4 48                 	add	rsp, 0x48
140a5d376: 5b                          	pop	rbx
140a5d377: 5f                          	pop	rdi
140a5d378: 5e                          	pop	rsi
140a5d379: 41 5c                       	pop	r12
140a5d37b: 41 5d                       	pop	r13
140a5d37d: 41 5e                       	pop	r14
140a5d37f: 41 5f                       	pop	r15
140a5d381: 5d                          	pop	rbp
140a5d382: c3                          	ret
140a5d383: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a5d390: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5d395: 55                          	push	rbp
140a5d396: 41 57                       	push	r15
140a5d398: 41 56                       	push	r14
140a5d39a: 41 55                       	push	r13
140a5d39c: 41 54                       	push	r12
140a5d39e: 56                          	push	rsi
140a5d39f: 57                          	push	rdi
140a5d3a0: 53                          	push	rbx
140a5d3a1: 48 83 ec 48                 	sub	rsp, 0x48
140a5d3a5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5d3ac: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5d3b1: 48 8d 8d c0 04 00 00        	lea	rcx, [rbp + 0x4c0]
140a5d3b8: e8 f3 9b e3 ff              	call	0x140896fb0 <.text+0x895fb0>
140a5d3bd: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5d3c2: 48 83 c4 48                 	add	rsp, 0x48
140a5d3c6: 5b                          	pop	rbx
140a5d3c7: 5f                          	pop	rdi
140a5d3c8: 5e                          	pop	rsi
140a5d3c9: 41 5c                       	pop	r12
140a5d3cb: 41 5d                       	pop	r13
140a5d3cd: 41 5e                       	pop	r14
140a5d3cf: 41 5f                       	pop	r15
140a5d3d1: 5d                          	pop	rbp
140a5d3d2: c3                          	ret
140a5d3d3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a5d3e0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5d3e5: 55                          	push	rbp
140a5d3e6: 41 57                       	push	r15
140a5d3e8: 41 56                       	push	r14
140a5d3ea: 41 55                       	push	r13
140a5d3ec: 41 54                       	push	r12
140a5d3ee: 56                          	push	rsi
140a5d3ef: 57                          	push	rdi
140a5d3f0: 53                          	push	rbx
140a5d3f1: 48 83 ec 48                 	sub	rsp, 0x48
140a5d3f5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5d3fc: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5d401: 48 8d 4d 40                 	lea	rcx, [rbp + 0x40]
140a5d405: e8 a6 9b e3 ff              	call	0x140896fb0 <.text+0x895fb0>
140a5d40a: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5d40f: 48 83 c4 48                 	add	rsp, 0x48
140a5d413: 5b                          	pop	rbx
140a5d414: 5f                          	pop	rdi
140a5d415: 5e                          	pop	rsi
140a5d416: 41 5c                       	pop	r12
140a5d418: 41 5d                       	pop	r13
140a5d41a: 41 5e                       	pop	r14
140a5d41c: 41 5f                       	pop	r15
140a5d41e: 5d                          	pop	rbp
140a5d41f: c3                          	ret
140a5d420: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5d425: 55                          	push	rbp
140a5d426: 41 57                       	push	r15
140a5d428: 41 56                       	push	r14
140a5d42a: 41 55                       	push	r13
140a5d42c: 41 54                       	push	r12
140a5d42e: 56                          	push	rsi
140a5d42f: 57                          	push	rdi
140a5d430: 53                          	push	rbx
140a5d431: 48 83 ec 48                 	sub	rsp, 0x48
140a5d435: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5d43c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5d441: 48 8d 8d 00 03 00 00        	lea	rcx, [rbp + 0x300]
140a5d448: e8 63 9b e3 ff              	call	0x140896fb0 <.text+0x895fb0>
140a5d44d: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5d452: 48 83 c4 48                 	add	rsp, 0x48
140a5d456: 5b                          	pop	rbx
140a5d457: 5f                          	pop	rdi
140a5d458: 5e                          	pop	rsi
140a5d459: 41 5c                       	pop	r12
140a5d45b: 41 5d                       	pop	r13
140a5d45d: 41 5e                       	pop	r14
140a5d45f: 41 5f                       	pop	r15
140a5d461: 5d                          	pop	rbp
140a5d462: c3                          	ret
140a5d463: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a5d470: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a5d475: 55                          	push	rbp
140a5d476: 41 57                       	push	r15
140a5d478: 41 56                       	push	r14
140a5d47a: 41 55                       	push	r13
140a5d47c: 41 54                       	push	r12
140a5d47e: 56                          	push	rsi
140a5d47f: 57                          	push	rdi
140a5d480: 53                          	push	rbx
140a5d481: 48 83 ec 48                 	sub	rsp, 0x48
140a5d485: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a5d48c: 0f 29 74 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm6
140a5d491: 48 8d 8d c0 04 00 00        	lea	rcx, [rbp + 0x4c0]
140a5d498: e8 13 9b e3 ff              	call	0x140896fb0 <.text+0x895fb0>
140a5d49d: 0f 28 74 24 30              	movaps	xmm6, xmmword ptr [rsp + 0x30]
140a5d4a2: 48 83 c4 48                 	add	rsp, 0x48
140a5d4a6: 5b                          	pop	rbx
140a5d4a7: 5f                          	pop	rdi
140a5d4a8: 5e                          	pop	rsi
140a5d4a9: 41 5c                       	pop	r12
140a5d4ab: 41 5d                       	pop	r13
140a5d4ad: 41 5e                       	pop	r14
140a5d4af: 41 5f                       	pop	r15
140a5d4b1: 5d                          	pop	rbp
140a5d4b2: c3                          	ret
