
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a3b2f0: 55                          	push	rbp
140a3b2f1: 41 57                       	push	r15
140a3b2f3: 41 56                       	push	r14
140a3b2f5: 41 54                       	push	r12
140a3b2f7: 56                          	push	rsi
140a3b2f8: 57                          	push	rdi
140a3b2f9: 53                          	push	rbx
140a3b2fa: 48 81 ec c0 03 00 00        	sub	rsp, 0x3c0
140a3b301: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
140a3b309: 48 c7 85 38 03 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x338], -0x2
140a3b314: 4c 89 cb                    	mov	rbx, r9
140a3b317: 4d 89 c6                    	mov	r14, r8
140a3b31a: 49 89 d7                    	mov	r15, rdx
140a3b31d: 48 89 ce                    	mov	rsi, rcx
140a3b320: 49 8d 48 38                 	lea	rcx, [r8 + 0x38]
140a3b324: 48 ba 8b 04 7b e6 61 26 c0 a4       	movabs	rdx, -0x5b3fd99e1984fb75
140a3b32e: 49 b8 ec 67 2a aa e6 61 e7 ea       	movabs	r8, -0x15189e1955d59814
140a3b338: e8 b3 23 6a ff              	call	0x1400dd6f0 <.text+0xdc6f0>
140a3b33d: 48 85 c0                    	test	rax, rax
140a3b340: 0f 84 7b 04 00 00           	je	0x140a3b7c1 <.text+0xa3a7c1>
140a3b346: 49 39 96 18 02 00 00        	cmp	qword ptr [r14 + 0x218], rdx
140a3b34d: 0f 86 6e 04 00 00           	jbe	0x140a3b7c1 <.text+0xa3a7c1>
140a3b353: 49 8b 86 10 02 00 00        	mov	rax, qword ptr [r14 + 0x210]
140a3b35a: 48 8b 0c d0                 	mov	rcx, qword ptr [rax + 8*rdx]
140a3b35e: 48 85 c9                    	test	rcx, rcx
140a3b361: 0f 84 5a 04 00 00           	je	0x140a3b7c1 <.text+0xa3a7c1>
140a3b367: 49 8b 86 e0 01 00 00        	mov	rax, qword ptr [r14 + 0x1e0]
140a3b36e: 48 f7 d1                    	not	rcx
140a3b371: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
140a3b375: 48 c1 e1 05                 	shl	rcx, 0x5
140a3b379: 48 83 7c 08 20 00           	cmp	qword ptr [rax + rcx + 0x20], 0x0
140a3b37f: 0f 84 3c 04 00 00           	je	0x140a3b7c1 <.text+0xa3a7c1>
140a3b385: 4c 8b a5 b8 03 00 00        	mov	r12, qword ptr [rbp + 0x3b8]
140a3b38c: 48 01 c8                    	add	rax, rcx
140a3b38f: 48 8b 40 10                 	mov	rax, qword ptr [rax + 0x10]
140a3b393: 0f b6 78 20                 	movzx	edi, byte ptr [rax + 0x20]
140a3b397: 4c 89 7c 24 30              	mov	qword ptr [rsp + 0x30], r15
140a3b39c: 4c 89 74 24 28              	mov	qword ptr [rsp + 0x28], r14
140a3b3a1: 48 c7 44 24 20 0c 00 00 00  	mov	qword ptr [rsp + 0x20], 0xc
140a3b3aa: 48 8d 15 df 20 09 02        	lea	rdx, [rip + 0x20920df]  # 0x142acd490
140a3b3b1: 4c 8d 0d db 20 09 02        	lea	r9, [rip + 0x20920db]   # 0x142acd493
140a3b3b8: 48 8d 8d 50 01 00 00        	lea	rcx, [rbp + 0x150]
140a3b3bf: 41 b8 03 00 00 00           	mov	r8d, 0x3
140a3b3c5: e8 d6 7e bc 00              	call	0x1416032a0 <.text+0x16022a0>
140a3b3ca: 80 bd 86 02 00 00 08        	cmp	byte ptr [rbp + 0x286], 0x8
140a3b3d1: 74 39                       	je	0x140a3b40c <.text+0xa3a40c>
140a3b3d3: 0f b6 05 76 99 bf 02        	movzx	eax, byte ptr [rip + 0x2bf9976] # 0x143634d50
140a3b3da: 83 f0 01                    	xor	eax, 0x1
140a3b3dd: 45 31 c9                    	xor	r9d, r9d
140a3b3e0: 41 80 7c 24 24 00           	cmp	byte ptr [r12 + 0x24], 0x0
140a3b3e6: 44 0f 45 c8                 	cmovne	r9d, eax
140a3b3ea: 48 8d 8d 60 02 00 00        	lea	rcx, [rbp + 0x260]
140a3b3f1: ba e8 00 00 00              	mov	edx, 0xe8
140a3b3f6: 48 03 95 88 02 00 00        	add	rdx, qword ptr [rbp + 0x288]
140a3b3fd: 49 b8 02 d9 ee 60 b7 09 a1 a9       	movabs	r8, -0x565ef6489f1126fe
140a3b407: e8 64 01 f3 ff              	call	0x14096b570 <.text+0x96a570>
140a3b40c: 4c 8d 75 c0                 	lea	r14, [rbp - 0x40]
140a3b410: 4c 8d bd 50 01 00 00        	lea	r15, [rbp + 0x150]
140a3b417: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a3b41d: 4c 89 f1                    	mov	rcx, r14
140a3b420: 4c 89 fa                    	mov	rdx, r15
140a3b423: e8 d8 87 ee 01              	call	0x142923c00 <.text+0x2922c00>
140a3b428: 48 89 9d e0 02 00 00        	mov	qword ptr [rbp + 0x2e0], rbx
140a3b42f: c7 85 50 01 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x150], 0x2
140a3b439: 4c 89 7c 24 20              	mov	qword ptr [rsp + 0x20], r15
140a3b43e: 48 8d 8d e0 02 00 00        	lea	rcx, [rbp + 0x2e0]
140a3b445: 49 b8 5e a9 e9 1c 54 8e 97 d9       	movabs	r8, -0x266871abe31656a2
140a3b44f: 4c 89 f2                    	mov	rdx, r14
140a3b452: 41 b1 19                    	mov	r9b, 0x19
140a3b455: e8 e6 2d eb ff              	call	0x1408ee240 <.text+0x8ed240>
140a3b45a: 48 8d 9d 50 01 00 00        	lea	rbx, [rbp + 0x150]
140a3b461: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a3b467: 48 89 d9                    	mov	rcx, rbx
140a3b46a: 4c 89 f2                    	mov	rdx, r14
140a3b46d: e8 8e 87 ee 01              	call	0x142923c00 <.text+0x2922c00>
140a3b472: 4c 89 a5 e0 02 00 00        	mov	qword ptr [rbp + 0x2e0], r12
140a3b479: c7 45 c0 02 00 00 00        	mov	dword ptr [rbp - 0x40], 0x2
140a3b480: 4c 89 74 24 20              	mov	qword ptr [rsp + 0x20], r14
140a3b485: 48 8d 8d e0 02 00 00        	lea	rcx, [rbp + 0x2e0]
140a3b48c: 49 b8 19 ad 7d 07 3c 60 5a 18       	movabs	r8, 0x185a603c077dad19
140a3b496: 48 89 da                    	mov	rdx, rbx
140a3b499: 41 b1 19                    	mov	r9b, 0x19
140a3b49c: e8 ef 3b f3 ff              	call	0x14096f090 <.text+0x96e090>
140a3b4a1: 4c 8b bd a8 03 00 00        	mov	r15, qword ptr [rbp + 0x3a8]
140a3b4a8: 48 8d 5d c0                 	lea	rbx, [rbp - 0x40]
140a3b4ac: 4c 8d b5 50 01 00 00        	lea	r14, [rbp + 0x150]
140a3b4b3: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a3b4b9: 48 89 d9                    	mov	rcx, rbx
140a3b4bc: 4c 89 f2                    	mov	rdx, r14
140a3b4bf: e8 3c 87 ee 01              	call	0x142923c00 <.text+0x2922c00>
140a3b4c4: 4c 89 bd e0 02 00 00        	mov	qword ptr [rbp + 0x2e0], r15
140a3b4cb: c7 85 50 01 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x150], 0x2
140a3b4d5: 4c 89 74 24 20              	mov	qword ptr [rsp + 0x20], r14
140a3b4da: 48 8d 8d e0 02 00 00        	lea	rcx, [rbp + 0x2e0]
140a3b4e1: 49 b8 e3 6b 5a 83 cd 4c a7 b8       	movabs	r8, -0x4758b3327ca5941d
140a3b4eb: 48 89 da                    	mov	rdx, rbx
140a3b4ee: 41 b1 19                    	mov	r9b, 0x19
140a3b4f1: e8 4a 2d eb ff              	call	0x1408ee240 <.text+0x8ed240>
140a3b4f6: 48 8b 9d b0 03 00 00        	mov	rbx, qword ptr [rbp + 0x3b0]
140a3b4fd: 4c 8d 7d c0                 	lea	r15, [rbp - 0x40]
140a3b501: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a3b507: 4c 89 f1                    	mov	rcx, r14
140a3b50a: 4c 89 fa                    	mov	rdx, r15
140a3b50d: e8 ee 86 ee 01              	call	0x142923c00 <.text+0x2922c00>
140a3b512: 48 89 9d e0 02 00 00        	mov	qword ptr [rbp + 0x2e0], rbx
140a3b519: c7 45 c0 02 00 00 00        	mov	dword ptr [rbp - 0x40], 0x2
140a3b520: 4c 89 7c 24 20              	mov	qword ptr [rsp + 0x20], r15
140a3b525: 48 8d 8d e0 02 00 00        	lea	rcx, [rbp + 0x2e0]
140a3b52c: 49 b8 13 19 d6 6a 5a 01 75 aa       	movabs	r8, -0x558afea59529e6ed
140a3b536: 4c 89 f2                    	mov	rdx, r14
140a3b539: 41 b1 32                    	mov	r9b, 0x32
140a3b53c: e8 bf 27 eb ff              	call	0x1408edd00 <.text+0x8ecd00>
140a3b541: 4c 8b a5 a0 03 00 00        	mov	r12, qword ptr [rbp + 0x3a0]
140a3b548: 4c 8d 75 c0                 	lea	r14, [rbp - 0x40]
140a3b54c: 4c 8d bd 50 01 00 00        	lea	r15, [rbp + 0x150]
140a3b553: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a3b559: 4c 89 f1                    	mov	rcx, r14
140a3b55c: 4c 89 fa                    	mov	rdx, r15
140a3b55f: e8 9c 86 ee 01              	call	0x142923c00 <.text+0x2922c00>
140a3b564: 4c 89 a5 e0 02 00 00        	mov	qword ptr [rbp + 0x2e0], r12
140a3b56b: c7 85 50 01 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x150], 0x2
140a3b575: 4c 89 7c 24 20              	mov	qword ptr [rsp + 0x20], r15
140a3b57a: 48 8d 8d e0 02 00 00        	lea	rcx, [rbp + 0x2e0]
140a3b581: 49 b8 b2 92 98 e6 3a 9a 21 88       	movabs	r8, -0x77de65c519676d4e
140a3b58b: 4c 89 f2                    	mov	rdx, r14
140a3b58e: 41 b1 32                    	mov	r9b, 0x32
140a3b591: e8 6a 27 eb ff              	call	0x1408edd00 <.text+0x8ecd00>
140a3b596: 48 8d 8d 50 01 00 00        	lea	rcx, [rbp + 0x150]
140a3b59d: 48 8d 55 c0                 	lea	rdx, [rbp - 0x40]
140a3b5a1: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a3b5a7: e8 54 86 ee 01              	call	0x142923c00 <.text+0x2922c00>
140a3b5ac: 80 bd 86 02 00 00 08        	cmp	byte ptr [rbp + 0x286], 0x8
140a3b5b3: 74 25                       	je	0x140a3b5da <.text+0xa3a5da>
140a3b5b5: 48 8d 8d 60 02 00 00        	lea	rcx, [rbp + 0x260]
140a3b5bc: ba e8 00 00 00              	mov	edx, 0xe8
140a3b5c1: 48 03 95 88 02 00 00        	add	rdx, qword ptr [rbp + 0x288]
140a3b5c8: 49 b8 92 5f a5 f4 f2 81 cf c4       	movabs	r8, -0x3b307e0d0b5aa06e
140a3b5d2: 41 89 f9                    	mov	r9d, edi
140a3b5d5: e8 96 ff f2 ff              	call	0x14096b570 <.text+0x96a570>
140a3b5da: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
140a3b5de: 48 8d 95 50 01 00 00        	lea	rdx, [rbp + 0x150]
140a3b5e5: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a3b5eb: e8 10 86 ee 01              	call	0x142923c00 <.text+0x2922c00>
140a3b5f0: 80 bd f6 00 00 00 08        	cmp	byte ptr [rbp + 0xf6], 0x8
140a3b5f7: 74 2a                       	je	0x140a3b623 <.text+0xa3a623>
140a3b5f9: f3 0f 10 9d c0 03 00 00     	movss	xmm3, dword ptr [rbp + 0x3c0]
140a3b601: 48 8d 8d d0 00 00 00        	lea	rcx, [rbp + 0xd0]
140a3b608: ba e8 00 00 00              	mov	edx, 0xe8
140a3b60d: 48 03 95 f8 00 00 00        	add	rdx, qword ptr [rbp + 0xf8]
140a3b614: 49 b8 49 1d 52 67 0b d5 02 a9       	movabs	r8, -0x56fd2af498ade2b7
140a3b61e: e8 0d 04 f3 ff              	call	0x14096ba30 <.text+0x96aa30>
140a3b623: 48 8d 8d 50 01 00 00        	lea	rcx, [rbp + 0x150]
140a3b62a: 48 8d 55 c0                 	lea	rdx, [rbp - 0x40]
140a3b62e: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a3b634: e8 c7 85 ee 01              	call	0x142923c00 <.text+0x2922c00>
140a3b639: 48 83 c3 04                 	add	rbx, 0x4
140a3b63d: c6 85 37 03 00 00 01        	mov	byte ptr [rbp + 0x337], 0x1
140a3b644: 48 89 d9                    	mov	rcx, rbx
140a3b647: e8 44 a1 19 00              	call	0x140bd5790 <.text+0xbd4790>
140a3b64c: 48 89 c1                    	mov	rcx, rax
140a3b64f: 48 c1 e9 20                 	shr	rcx, 0x20
140a3b653: 0f 10 85 f0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1f0]
140a3b65a: 0f 29 85 20 03 00 00        	movaps	xmmword ptr [rbp + 0x320], xmm0
140a3b661: 0f 10 85 e0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1e0]
140a3b668: 0f 29 85 10 03 00 00        	movaps	xmmword ptr [rbp + 0x310], xmm0
140a3b66f: 0f 10 85 d0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1d0]
140a3b676: 0f 29 85 00 03 00 00        	movaps	xmmword ptr [rbp + 0x300], xmm0
140a3b67d: 0f 10 85 c0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1c0]
140a3b684: 0f 29 85 f0 02 00 00        	movaps	xmmword ptr [rbp + 0x2f0], xmm0
140a3b68b: 0f 10 85 b0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1b0]
140a3b692: 0f 29 85 e0 02 00 00        	movaps	xmmword ptr [rbp + 0x2e0], xmm0
140a3b699: 48 8b 95 88 02 00 00        	mov	rdx, qword ptr [rbp + 0x288]
140a3b6a0: 4c 8b 85 d0 02 00 00        	mov	r8, qword ptr [rbp + 0x2d0]
140a3b6a7: 4c 8b 8d 60 01 00 00        	mov	r9, qword ptr [rbp + 0x160]
140a3b6ae: 4c 89 4d d0                 	mov	qword ptr [rbp - 0x30], r9
140a3b6b2: 0f 10 85 50 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x150]
140a3b6b9: 0f 29 45 c0                 	movaps	xmmword ptr [rbp - 0x40], xmm0
140a3b6bd: 0f 10 85 68 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x168]
140a3b6c4: 0f 29 45 f0                 	movaps	xmmword ptr [rbp - 0x10], xmm0
140a3b6c8: 4c 8b 8d 78 01 00 00        	mov	r9, qword ptr [rbp + 0x178]
140a3b6cf: 4c 89 4d 00                 	mov	qword ptr [rbp], r9
140a3b6d3: 0f 10 85 80 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x180]
140a3b6da: 0f 11 45 d8                 	movups	xmmword ptr [rbp - 0x28], xmm0
140a3b6de: 4c 8b 8d 90 01 00 00        	mov	r9, qword ptr [rbp + 0x190]
140a3b6e5: 4c 89 4d e8                 	mov	qword ptr [rbp - 0x18], r9
140a3b6e9: 0f 10 85 00 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x200]
140a3b6f0: 0f 10 8d 10 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x210]
140a3b6f7: 0f 10 95 20 02 00 00        	movups	xmm2, xmmword ptr [rbp + 0x220]
140a3b6fe: 0f 10 9d 30 02 00 00        	movups	xmm3, xmmword ptr [rbp + 0x230]
140a3b705: 0f 11 45 38                 	movups	xmmword ptr [rbp + 0x38], xmm0
140a3b709: 0f 11 4d 48                 	movups	xmmword ptr [rbp + 0x48], xmm1
140a3b70d: 0f 11 55 58                 	movups	xmmword ptr [rbp + 0x58], xmm2
140a3b711: 0f 11 5d 68                 	movups	xmmword ptr [rbp + 0x68], xmm3
140a3b715: 0f 10 85 40 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x240]
140a3b71c: 0f 11 45 78                 	movups	xmmword ptr [rbp + 0x78], xmm0
140a3b720: 0f 10 85 50 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x250]
140a3b727: 0f 11 85 88 00 00 00        	movups	xmmword ptr [rbp + 0x88], xmm0
140a3b72e: 4c 8b 8d a8 01 00 00        	mov	r9, qword ptr [rbp + 0x1a8]
140a3b735: 4c 89 4d 18                 	mov	qword ptr [rbp + 0x18], r9
140a3b739: 0f 10 85 98 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x198]
140a3b740: 0f 11 45 08                 	movups	xmmword ptr [rbp + 0x8], xmm0
140a3b744: 4c 89 85 a0 00 00 00        	mov	qword ptr [rbp + 0xa0], r8
140a3b74b: 48 89 95 98 00 00 00        	mov	qword ptr [rbp + 0x98], rdx
140a3b752: c7 45 20 01 00 00 00        	mov	dword ptr [rbp + 0x20], 0x1
140a3b759: 89 45 24                    	mov	dword ptr [rbp + 0x24], eax
140a3b75c: 89 4d 28                    	mov	dword ptr [rbp + 0x28], ecx
140a3b75f: c7 45 2c 01 00 00 00        	mov	dword ptr [rbp + 0x2c], 0x1
140a3b766: 48 8d 8d e0 02 00 00        	lea	rcx, [rbp + 0x2e0]
140a3b76d: 48 8d 55 c0                 	lea	rdx, [rbp - 0x40]
140a3b771: e8 5a 1d fa ff              	call	0x1409dd4d0 <.text+0x9dc4d0>
140a3b776: 80 bd 86 02 00 00 08        	cmp	byte ptr [rbp + 0x286], 0x8
140a3b77d: 74 13                       	je	0x140a3b792 <.text+0xa3a792>
140a3b77f: 48 8d 8d 70 02 00 00        	lea	rcx, [rbp + 0x270]
140a3b786: c6 85 37 03 00 00 00        	mov	byte ptr [rbp + 0x337], 0x0
140a3b78d: e8 1e 6d 1a 00              	call	0x140be24b0 <.text+0xbe14b0>
140a3b792: 41 8b 44 24 20              	mov	eax, dword ptr [r12 + 0x20]
140a3b797: 89 46 20                    	mov	dword ptr [rsi + 0x20], eax
140a3b79a: 41 0f 10 04 24              	movups	xmm0, xmmword ptr [r12]
140a3b79f: 41 0f 10 4c 24 10           	movups	xmm1, xmmword ptr [r12 + 0x10]
140a3b7a5: 0f 11 4e 10                 	movups	xmmword ptr [rsi + 0x10], xmm1
140a3b7a9: 0f 11 06                    	movups	xmmword ptr [rsi], xmm0
140a3b7ac: 48 89 f0                    	mov	rax, rsi
140a3b7af: 48 81 c4 c0 03 00 00        	add	rsp, 0x3c0
140a3b7b6: 5b                          	pop	rbx
140a3b7b7: 5f                          	pop	rdi
140a3b7b8: 5e                          	pop	rsi
140a3b7b9: 41 5c                       	pop	r12
140a3b7bb: 41 5e                       	pop	r14
140a3b7bd: 41 5f                       	pop	r15
140a3b7bf: 5d                          	pop	rbp
140a3b7c0: c3                          	ret
140a3b7c1: 48 8d 0d b0 1c 09 02        	lea	rcx, [rip + 0x2091cb0]  # 0x142acd478
140a3b7c8: e8 43 db e9 01              	call	0x1428d9310 <.text+0x28d8310>
140a3b7cd: cc                          	int3
140a3b7ce: 66 90                       	nop
140a3b7d0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a3b7d5: 55                          	push	rbp
140a3b7d6: 41 57                       	push	r15
140a3b7d8: 41 56                       	push	r14
140a3b7da: 41 54                       	push	r12
140a3b7dc: 56                          	push	rsi
140a3b7dd: 57                          	push	rdi
140a3b7de: 53                          	push	rbx
140a3b7df: 48 83 ec 40                 	sub	rsp, 0x40
140a3b7e3: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a3b7ea: 80 bd 86 02 00 00 08        	cmp	byte ptr [rbp + 0x286], 0x8
140a3b7f1: 74 13                       	je	0x140a3b806 <.text+0xa3a806>
140a3b7f3: 48 8d 8d 70 02 00 00        	lea	rcx, [rbp + 0x270]
140a3b7fa: c6 85 37 03 00 00 00        	mov	byte ptr [rbp + 0x337], 0x0
140a3b801: e8 aa 6c 1a 00              	call	0x140be24b0 <.text+0xbe14b0>
140a3b806: c6 85 37 03 00 00 00        	mov	byte ptr [rbp + 0x337], 0x0
140a3b80d: 48 83 c4 40                 	add	rsp, 0x40
140a3b811: 5b                          	pop	rbx
140a3b812: 5f                          	pop	rdi
140a3b813: 5e                          	pop	rsi
140a3b814: 41 5c                       	pop	r12
140a3b816: 41 5e                       	pop	r14
140a3b818: 41 5f                       	pop	r15
140a3b81a: 5d                          	pop	rbp
140a3b81b: c3                          	ret
140a3b81c: 0f 1f 40 00                 	nop	dword ptr [rax]
140a3b820: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a3b825: 55                          	push	rbp
140a3b826: 41 57                       	push	r15
140a3b828: 41 56                       	push	r14
140a3b82a: 41 54                       	push	r12
140a3b82c: 56                          	push	rsi
140a3b82d: 57                          	push	rdi
140a3b82e: 53                          	push	rbx
140a3b82f: 48 83 ec 40                 	sub	rsp, 0x40
140a3b833: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a3b83a: 80 bd 37 03 00 00 00        	cmp	byte ptr [rbp + 0x337], 0x0
140a3b841: 75 0f                       	jne	0x140a3b852 <.text+0xa3a852>
140a3b843: 48 83 c4 40                 	add	rsp, 0x40
140a3b847: 5b                          	pop	rbx
140a3b848: 5f                          	pop	rdi
140a3b849: 5e                          	pop	rsi
140a3b84a: 41 5c                       	pop	r12
140a3b84c: 41 5e                       	pop	r14
140a3b84e: 41 5f                       	pop	r15
140a3b850: 5d                          	pop	rbp
140a3b851: c3                          	ret
140a3b852: 48 8d 8d 50 01 00 00        	lea	rcx, [rbp + 0x150]
140a3b859: e8 52 b7 e5 ff              	call	0x140896fb0 <.text+0x895fb0>
140a3b85e: eb e3                       	jmp	0x140a3b843 <.text+0xa3a843>
140a3b860: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a3b865: 55                          	push	rbp
140a3b866: 41 57                       	push	r15
140a3b868: 41 56                       	push	r14
140a3b86a: 41 54                       	push	r12
140a3b86c: 56                          	push	rsi
140a3b86d: 57                          	push	rdi
140a3b86e: 53                          	push	rbx
140a3b86f: 48 83 ec 40                 	sub	rsp, 0x40
140a3b873: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a3b87a: 48 8d 8d 50 01 00 00        	lea	rcx, [rbp + 0x150]
140a3b881: e8 2a b7 e5 ff              	call	0x140896fb0 <.text+0x895fb0>
140a3b886: 90                          	nop
140a3b887: 48 83 c4 40                 	add	rsp, 0x40
140a3b88b: 5b                          	pop	rbx
140a3b88c: 5f                          	pop	rdi
140a3b88d: 5e                          	pop	rsi
140a3b88e: 41 5c                       	pop	r12
140a3b890: 41 5e                       	pop	r14
140a3b892: 41 5f                       	pop	r15
140a3b894: 5d                          	pop	rbp
140a3b895: c3                          	ret
140a3b896: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
140a3b8a0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a3b8a5: 55                          	push	rbp
140a3b8a6: 41 57                       	push	r15
140a3b8a8: 41 56                       	push	r14
140a3b8aa: 41 54                       	push	r12
140a3b8ac: 56                          	push	rsi
140a3b8ad: 57                          	push	rdi
140a3b8ae: 53                          	push	rbx
140a3b8af: 48 83 ec 40                 	sub	rsp, 0x40
140a3b8b3: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a3b8ba: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
140a3b8be: e8 ed b6 e5 ff              	call	0x140896fb0 <.text+0x895fb0>
140a3b8c3: 90                          	nop
140a3b8c4: 48 83 c4 40                 	add	rsp, 0x40
140a3b8c8: 5b                          	pop	rbx
140a3b8c9: 5f                          	pop	rdi
140a3b8ca: 5e                          	pop	rsi
140a3b8cb: 41 5c                       	pop	r12
140a3b8cd: 41 5e                       	pop	r14
140a3b8cf: 41 5f                       	pop	r15
140a3b8d1: 5d                          	pop	rbp
140a3b8d2: c3                          	ret
140a3b8d3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a3b8e0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a3b8e5: 55                          	push	rbp
140a3b8e6: 41 57                       	push	r15
140a3b8e8: 41 56                       	push	r14
140a3b8ea: 41 54                       	push	r12
140a3b8ec: 56                          	push	rsi
140a3b8ed: 57                          	push	rdi
140a3b8ee: 53                          	push	rbx
140a3b8ef: 48 83 ec 40                 	sub	rsp, 0x40
140a3b8f3: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a3b8fa: 48 8d 8d 50 01 00 00        	lea	rcx, [rbp + 0x150]
140a3b901: e8 aa b6 e5 ff              	call	0x140896fb0 <.text+0x895fb0>
140a3b906: 90                          	nop
140a3b907: 48 83 c4 40                 	add	rsp, 0x40
140a3b90b: 5b                          	pop	rbx
140a3b90c: 5f                          	pop	rdi
140a3b90d: 5e                          	pop	rsi
140a3b90e: 41 5c                       	pop	r12
140a3b910: 41 5e                       	pop	r14
140a3b912: 41 5f                       	pop	r15
140a3b914: 5d                          	pop	rbp
140a3b915: c3                          	ret
140a3b916: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
140a3b920: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a3b925: 55                          	push	rbp
140a3b926: 41 57                       	push	r15
140a3b928: 41 56                       	push	r14
140a3b92a: 41 54                       	push	r12
140a3b92c: 56                          	push	rsi
140a3b92d: 57                          	push	rdi
140a3b92e: 53                          	push	rbx
140a3b92f: 48 83 ec 40                 	sub	rsp, 0x40
140a3b933: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a3b93a: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
140a3b93e: e8 6d b6 e5 ff              	call	0x140896fb0 <.text+0x895fb0>
140a3b943: 90                          	nop
140a3b944: 48 83 c4 40                 	add	rsp, 0x40
140a3b948: 5b                          	pop	rbx
140a3b949: 5f                          	pop	rdi
140a3b94a: 5e                          	pop	rsi
140a3b94b: 41 5c                       	pop	r12
140a3b94d: 41 5e                       	pop	r14
140a3b94f: 41 5f                       	pop	r15
140a3b951: 5d                          	pop	rbp
140a3b952: c3                          	ret
140a3b953: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a3b960: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a3b965: 55                          	push	rbp
140a3b966: 41 57                       	push	r15
140a3b968: 41 56                       	push	r14
140a3b96a: 41 54                       	push	r12
140a3b96c: 56                          	push	rsi
140a3b96d: 57                          	push	rdi
140a3b96e: 53                          	push	rbx
140a3b96f: 48 83 ec 40                 	sub	rsp, 0x40
140a3b973: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a3b97a: 48 8d 8d 50 01 00 00        	lea	rcx, [rbp + 0x150]
140a3b981: e8 2a b6 e5 ff              	call	0x140896fb0 <.text+0x895fb0>
140a3b986: 90                          	nop
140a3b987: 48 83 c4 40                 	add	rsp, 0x40
140a3b98b: 5b                          	pop	rbx
140a3b98c: 5f                          	pop	rdi
140a3b98d: 5e                          	pop	rsi
140a3b98e: 41 5c                       	pop	r12
140a3b990: 41 5e                       	pop	r14
140a3b992: 41 5f                       	pop	r15
140a3b994: 5d                          	pop	rbp
140a3b995: c3                          	ret
140a3b996: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
140a3b9a0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a3b9a5: 55                          	push	rbp
140a3b9a6: 41 57                       	push	r15
140a3b9a8: 41 56                       	push	r14
140a3b9aa: 41 54                       	push	r12
140a3b9ac: 56                          	push	rsi
140a3b9ad: 57                          	push	rdi
140a3b9ae: 53                          	push	rbx
140a3b9af: 48 83 ec 40                 	sub	rsp, 0x40
140a3b9b3: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a3b9ba: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
140a3b9be: e8 ed b5 e5 ff              	call	0x140896fb0 <.text+0x895fb0>
140a3b9c3: 90                          	nop
140a3b9c4: 48 83 c4 40                 	add	rsp, 0x40
140a3b9c8: 5b                          	pop	rbx
140a3b9c9: 5f                          	pop	rdi
140a3b9ca: 5e                          	pop	rsi
140a3b9cb: 41 5c                       	pop	r12
140a3b9cd: 41 5e                       	pop	r14
140a3b9cf: 41 5f                       	pop	r15
140a3b9d1: 5d                          	pop	rbp
140a3b9d2: c3                          	ret
140a3b9d3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a3b9e0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a3b9e5: 55                          	push	rbp
140a3b9e6: 41 57                       	push	r15
140a3b9e8: 41 56                       	push	r14
140a3b9ea: 41 54                       	push	r12
140a3b9ec: 56                          	push	rsi
140a3b9ed: 57                          	push	rdi
140a3b9ee: 53                          	push	rbx
140a3b9ef: 48 83 ec 40                 	sub	rsp, 0x40
140a3b9f3: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a3b9fa: 48 8d 8d 50 01 00 00        	lea	rcx, [rbp + 0x150]
140a3ba01: e8 aa b5 e5 ff              	call	0x140896fb0 <.text+0x895fb0>
140a3ba06: 90                          	nop
140a3ba07: 48 83 c4 40                 	add	rsp, 0x40
140a3ba0b: 5b                          	pop	rbx
140a3ba0c: 5f                          	pop	rdi
140a3ba0d: 5e                          	pop	rsi
140a3ba0e: 41 5c                       	pop	r12
140a3ba10: 41 5e                       	pop	r14
140a3ba12: 41 5f                       	pop	r15
140a3ba14: 5d                          	pop	rbp
140a3ba15: c3                          	ret
140a3ba16: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
140a3ba20: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a3ba25: 55                          	push	rbp
140a3ba26: 41 57                       	push	r15
140a3ba28: 41 56                       	push	r14
140a3ba2a: 41 54                       	push	r12
140a3ba2c: 56                          	push	rsi
140a3ba2d: 57                          	push	rdi
140a3ba2e: 53                          	push	rbx
140a3ba2f: 48 83 ec 40                 	sub	rsp, 0x40
140a3ba33: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a3ba3a: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
140a3ba3e: e8 6d b5 e5 ff              	call	0x140896fb0 <.text+0x895fb0>
140a3ba43: 90                          	nop
140a3ba44: 48 83 c4 40                 	add	rsp, 0x40
140a3ba48: 5b                          	pop	rbx
140a3ba49: 5f                          	pop	rdi
140a3ba4a: 5e                          	pop	rsi
140a3ba4b: 41 5c                       	pop	r12
140a3ba4d: 41 5e                       	pop	r14
140a3ba4f: 41 5f                       	pop	r15
140a3ba51: 5d                          	pop	rbp
140a3ba52: c3                          	ret
