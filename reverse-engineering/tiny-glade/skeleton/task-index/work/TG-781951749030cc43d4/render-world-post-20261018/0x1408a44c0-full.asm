
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408a44c0: 55                          	push	rbp
1408a44c1: 41 57                       	push	r15
1408a44c3: 41 56                       	push	r14
1408a44c5: 41 54                       	push	r12
1408a44c7: 56                          	push	rsi
1408a44c8: 57                          	push	rdi
1408a44c9: 53                          	push	rbx
1408a44ca: 48 81 ec 20 05 00 00        	sub	rsp, 0x520
1408a44d1: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
1408a44d9: 0f 29 bd 90 04 00 00        	movaps	xmmword ptr [rbp + 0x490], xmm7
1408a44e0: 0f 29 b5 80 04 00 00        	movaps	xmmword ptr [rbp + 0x480], xmm6
1408a44e7: 48 c7 85 78 04 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x478], -0x2
1408a44f2: 4c 89 cf                    	mov	rdi, r9
1408a44f5: 4c 89 c3                    	mov	rbx, r8
1408a44f8: 49 89 d6                    	mov	r14, rdx
1408a44fb: 49 89 cf                    	mov	r15, rcx
1408a44fe: 48 8d 4a 38                 	lea	rcx, [rdx + 0x38]
1408a4502: 48 ba ff a3 08 6a ee 51 a4 bb       	movabs	rdx, -0x445bae1195f75c01
1408a450c: 49 b8 9d f2 bc fe 15 8d 00 8d       	movabs	r8, -0x72ff72ea01430d63
1408a4516: e8 d5 91 83 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1408a451b: 48 85 c0                    	test	rax, rax
1408a451e: 0f 84 87 04 00 00           	je	0x1408a49ab <.text+0x8a39ab>
1408a4524: 49 39 96 18 02 00 00        	cmp	qword ptr [r14 + 0x218], rdx
1408a452b: 0f 86 7a 04 00 00           	jbe	0x1408a49ab <.text+0x8a39ab>
1408a4531: 49 8b 86 10 02 00 00        	mov	rax, qword ptr [r14 + 0x210]
1408a4538: 48 8b 0c d0                 	mov	rcx, qword ptr [rax + 8*rdx]
1408a453c: 48 85 c9                    	test	rcx, rcx
1408a453f: 0f 84 66 04 00 00           	je	0x1408a49ab <.text+0x8a39ab>
1408a4545: 49 8b 86 e0 01 00 00        	mov	rax, qword ptr [r14 + 0x1e0]
1408a454c: 48 f7 d1                    	not	rcx
1408a454f: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
1408a4553: 48 c1 e1 05                 	shl	rcx, 0x5
1408a4557: 48 83 7c 08 20 00           	cmp	qword ptr [rax + rcx + 0x20], 0x0
1408a455d: 0f 84 48 04 00 00           	je	0x1408a49ab <.text+0x8a39ab>
1408a4563: 48 01 c8                    	add	rax, rcx
1408a4566: 4c 8b 60 10                 	mov	r12, qword ptr [rax + 0x10]
1408a456a: 48 8d 4b 04                 	lea	rcx, [rbx + 0x4]
1408a456e: e8 1d 12 33 00              	call	0x140bd5790 <.text+0xbd4790>
1408a4573: 48 89 c6                    	mov	rsi, rax
1408a4576: 41 0f b6 84 24 d0 04 00 00  	movzx	eax, byte ptr [r12 + 0x4d0]
1408a457f: 83 f8 04                    	cmp	eax, 0x4
1408a4582: 74 05                       	je	0x1408a4589 <.text+0x8a3589>
1408a4584: 83 f8 01                    	cmp	eax, 0x1
1408a4587: 75 12                       	jne	0x1408a459b <.text+0x8a359b>
1408a4589: f3 0f 10 35 17 00 09 02     	movss	xmm6, dword ptr [rip + 0x2090017] # 0x1429345a8
1408a4591: f3 0f 10 3d 3b 66 1f 02     	movss	xmm7, dword ptr [rip + 0x21f663b] # 0x142a9abd4
1408a4599: eb 0b                       	jmp	0x1408a45a6 <.text+0x8a35a6>
1408a459b: f3 0f 10 3d 61 13 08 02     	movss	xmm7, dword ptr [rip + 0x2081361] # 0x142925904
1408a45a3: 0f 28 f7                    	movaps	xmm6, xmm7
1408a45a6: 4c 89 7c 24 30              	mov	qword ptr [rsp + 0x30], r15
1408a45ab: 4c 89 74 24 28              	mov	qword ptr [rsp + 0x28], r14
1408a45b0: 48 c7 44 24 20 1a 00 00 00  	mov	qword ptr [rsp + 0x20], 0x1a
1408a45b9: 48 8d 15 c8 6d 1f 02        	lea	rdx, [rip + 0x21f6dc8]  # 0x142a9b388
1408a45c0: 4c 8d 0d cc 6d 1f 02        	lea	r9, [rip + 0x21f6dcc]   # 0x142a9b393
1408a45c7: 48 8d 8d e0 02 00 00        	lea	rcx, [rbp + 0x2e0]
1408a45ce: 41 b8 0b 00 00 00           	mov	r8d, 0xb
1408a45d4: e8 c7 ec d5 00              	call	0x1416032a0 <.text+0x16022a0>
1408a45d9: 80 bd 16 04 00 00 08        	cmp	byte ptr [rbp + 0x416], 0x8
1408a45e0: 74 25                       	je	0x1408a4607 <.text+0x8a3607>
1408a45e2: 48 8d 8d f0 03 00 00        	lea	rcx, [rbp + 0x3f0]
1408a45e9: ba e8 00 00 00              	mov	edx, 0xe8
1408a45ee: 48 03 95 18 04 00 00        	add	rdx, qword ptr [rbp + 0x418]
1408a45f5: 49 b8 4c a5 59 1f 44 98 8a 41       	movabs	r8, 0x418a98441f59a54c
1408a45ff: 41 b1 01                    	mov	r9b, 0x1
1408a4602: e8 49 72 0c 00              	call	0x14096b850 <.text+0x96a850>
1408a4607: 4c 8d b5 50 01 00 00        	lea	r14, [rbp + 0x150]
1408a460e: 48 8d 95 e0 02 00 00        	lea	rdx, [rbp + 0x2e0]
1408a4615: 41 b8 90 01 00 00           	mov	r8d, 0x190
1408a461b: 4c 89 f1                    	mov	rcx, r14
1408a461e: e8 dd f5 07 02              	call	0x142923c00 <.text+0x2922c00>
1408a4623: c6 85 77 04 00 00 11        	mov	byte ptr [rbp + 0x477], 0x11
1408a462a: c7 45 c0 02 00 00 00        	mov	dword ptr [rbp - 0x40], 0x2
1408a4631: 48 8d 45 c0                 	lea	rax, [rbp - 0x40]
1408a4635: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1408a463a: 48 8d 8d 77 04 00 00        	lea	rcx, [rbp + 0x477]
1408a4641: 49 b8 a4 1a 84 c9 4d 1f 65 11       	movabs	r8, 0x11651f4dc9841aa4
1408a464b: 4c 89 f2                    	mov	rdx, r14
1408a464e: 41 b1 19                    	mov	r9b, 0x19
1408a4651: e8 0a 0c 06 00              	call	0x140905260 <.text+0x904260>
1408a4656: 4c 8d 7d c0                 	lea	r15, [rbp - 0x40]
1408a465a: 41 b8 90 01 00 00           	mov	r8d, 0x190
1408a4660: 4c 89 f9                    	mov	rcx, r15
1408a4663: 4c 89 f2                    	mov	rdx, r14
1408a4666: e8 95 f5 07 02              	call	0x142923c00 <.text+0x2922c00>
1408a466b: 48 b8 65 18 a0 91 ab 39 9f 57       	movabs	rax, 0x579f39ab91a01865
1408a4675: 48 89 85 e0 02 00 00        	mov	qword ptr [rbp + 0x2e0], rax
1408a467c: c7 85 50 01 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x150], 0x2
1408a4686: 4c 89 74 24 20              	mov	qword ptr [rsp + 0x20], r14
1408a468b: 4c 8d b5 e0 02 00 00        	lea	r14, [rbp + 0x2e0]
1408a4692: 49 b8 8c 97 66 69 03 f5 5b 81       	movabs	r8, -0x7ea40afc96996874
1408a469c: 4c 89 f1                    	mov	rcx, r14
1408a469f: 4c 89 fa                    	mov	rdx, r15
1408a46a2: 41 b1 19                    	mov	r9b, 0x19
1408a46a5: e8 26 10 06 00              	call	0x1409056d0 <.text+0x9046d0>
1408a46aa: 4c 8d bd 50 01 00 00        	lea	r15, [rbp + 0x150]
1408a46b1: 48 8d 55 c0                 	lea	rdx, [rbp - 0x40]
1408a46b5: 41 b8 90 01 00 00           	mov	r8d, 0x190
1408a46bb: 4c 89 f9                    	mov	rcx, r15
1408a46be: e8 3d f5 07 02              	call	0x142923c00 <.text+0x2922c00>
1408a46c3: 48 89 5d c0                 	mov	qword ptr [rbp - 0x40], rbx
1408a46c7: c7 85 e0 02 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x2e0], 0x2
1408a46d1: 4c 89 74 24 20              	mov	qword ptr [rsp + 0x20], r14
1408a46d6: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
1408a46da: 49 b8 5e a9 e9 1c 54 8e 97 d9       	movabs	r8, -0x266871abe31656a2
1408a46e4: 4c 89 fa                    	mov	rdx, r15
1408a46e7: 41 b1 32                    	mov	r9b, 0x32
1408a46ea: e8 11 96 04 00              	call	0x1408edd00 <.text+0x8ecd00>
1408a46ef: 48 8d 9d e0 02 00 00        	lea	rbx, [rbp + 0x2e0]
1408a46f6: 4c 8d b5 50 01 00 00        	lea	r14, [rbp + 0x150]
1408a46fd: 41 b8 90 01 00 00           	mov	r8d, 0x190
1408a4703: 48 89 d9                    	mov	rcx, rbx
1408a4706: 4c 89 f2                    	mov	rdx, r14
1408a4709: e8 f2 f4 07 02              	call	0x142923c00 <.text+0x2922c00>
1408a470e: 48 89 7d c0                 	mov	qword ptr [rbp - 0x40], rdi
1408a4712: c7 85 50 01 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x150], 0x2
1408a471c: 4c 89 74 24 20              	mov	qword ptr [rsp + 0x20], r14
1408a4721: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
1408a4725: 49 b8 fc fb 49 a0 9d 57 61 48       	movabs	r8, 0x4861579da049fbfc
1408a472f: 48 89 da                    	mov	rdx, rbx
1408a4732: 41 b1 19                    	mov	r9b, 0x19
1408a4735: e8 06 9b 04 00              	call	0x1408ee240 <.text+0x8ed240>
1408a473a: 48 8d 8d 50 01 00 00        	lea	rcx, [rbp + 0x150]
1408a4741: 48 8d 95 e0 02 00 00        	lea	rdx, [rbp + 0x2e0]
1408a4748: 41 b8 90 01 00 00           	mov	r8d, 0x190
1408a474e: e8 ad f4 07 02              	call	0x142923c00 <.text+0x2922c00>
1408a4753: 80 bd 86 02 00 00 08        	cmp	byte ptr [rbp + 0x286], 0x8
1408a475a: 74 25                       	je	0x1408a4781 <.text+0x8a3781>
1408a475c: 48 8d 8d 60 02 00 00        	lea	rcx, [rbp + 0x260]
1408a4763: ba e8 00 00 00              	mov	edx, 0xe8
1408a4768: 48 03 95 88 02 00 00        	add	rdx, qword ptr [rbp + 0x288]
1408a476f: 49 b8 f5 51 86 b0 3f 67 ab c5       	movabs	r8, -0x3a5498c04f79ae0b
1408a4779: 45 31 c9                    	xor	r9d, r9d
1408a477c: e8 ef 6d 0c 00              	call	0x14096b570 <.text+0x96a570>
1408a4781: 48 8d 8d e0 02 00 00        	lea	rcx, [rbp + 0x2e0]
1408a4788: 48 8d 95 50 01 00 00        	lea	rdx, [rbp + 0x150]
1408a478f: 41 b8 90 01 00 00           	mov	r8d, 0x190
1408a4795: e8 66 f4 07 02              	call	0x142923c00 <.text+0x2922c00>
1408a479a: 80 bd 16 04 00 00 08        	cmp	byte ptr [rbp + 0x416], 0x8
1408a47a1: 74 25                       	je	0x1408a47c8 <.text+0x8a37c8>
1408a47a3: 48 8d 8d f0 03 00 00        	lea	rcx, [rbp + 0x3f0]
1408a47aa: ba e8 00 00 00              	mov	edx, 0xe8
1408a47af: 48 03 95 18 04 00 00        	add	rdx, qword ptr [rbp + 0x418]
1408a47b6: 49 b8 b4 48 17 4f 84 99 e1 c6       	movabs	r8, -0x391e667bb0e8b74c
1408a47c0: 0f 28 df                    	movaps	xmm3, xmm7
1408a47c3: e8 68 72 0c 00              	call	0x14096ba30 <.text+0x96aa30>
1408a47c8: 48 8d 8d 50 01 00 00        	lea	rcx, [rbp + 0x150]
1408a47cf: 48 8d 95 e0 02 00 00        	lea	rdx, [rbp + 0x2e0]
1408a47d6: 41 b8 90 01 00 00           	mov	r8d, 0x190
1408a47dc: e8 1f f4 07 02              	call	0x142923c00 <.text+0x2922c00>
1408a47e1: 80 bd 86 02 00 00 08        	cmp	byte ptr [rbp + 0x286], 0x8
1408a47e8: 74 25                       	je	0x1408a480f <.text+0x8a380f>
1408a47ea: 48 8d 8d 60 02 00 00        	lea	rcx, [rbp + 0x260]
1408a47f1: ba e8 00 00 00              	mov	edx, 0xe8
1408a47f6: 48 03 95 88 02 00 00        	add	rdx, qword ptr [rbp + 0x288]
1408a47fd: 49 b8 e7 58 7a b2 36 1f 67 47       	movabs	r8, 0x47671f36b27a58e7
1408a4807: 0f 28 de                    	movaps	xmm3, xmm6
1408a480a: e8 21 72 0c 00              	call	0x14096ba30 <.text+0x96aa30>
1408a480f: 48 89 f7                    	mov	rdi, rsi
1408a4812: 48 c1 ef 20                 	shr	rdi, 0x20
1408a4816: 48 8d 8d e0 02 00 00        	lea	rcx, [rbp + 0x2e0]
1408a481d: 48 8d 95 50 01 00 00        	lea	rdx, [rbp + 0x150]
1408a4824: 41 b8 90 01 00 00           	mov	r8d, 0x190
1408a482a: e8 d1 f3 07 02              	call	0x142923c00 <.text+0x2922c00>
1408a482f: 0f 10 85 80 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x380]
1408a4836: 0f 29 45 00                 	movaps	xmmword ptr [rbp], xmm0
1408a483a: 0f 10 85 70 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x370]
1408a4841: 0f 29 45 f0                 	movaps	xmmword ptr [rbp - 0x10], xmm0
1408a4845: 0f 10 85 60 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x360]
1408a484c: 0f 29 45 e0                 	movaps	xmmword ptr [rbp - 0x20], xmm0
1408a4850: 0f 10 85 50 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x350]
1408a4857: 0f 29 45 d0                 	movaps	xmmword ptr [rbp - 0x30], xmm0
1408a485b: 0f 10 85 40 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x340]
1408a4862: 0f 29 45 c0                 	movaps	xmmword ptr [rbp - 0x40], xmm0
1408a4866: 48 8b 85 18 04 00 00        	mov	rax, qword ptr [rbp + 0x418]
1408a486d: 48 8b 8d 60 04 00 00        	mov	rcx, qword ptr [rbp + 0x460]
1408a4874: 48 8b 95 f0 02 00 00        	mov	rdx, qword ptr [rbp + 0x2f0]
1408a487b: 48 89 95 60 01 00 00        	mov	qword ptr [rbp + 0x160], rdx
1408a4882: 0f 10 85 e0 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2e0]
1408a4889: 0f 29 85 50 01 00 00        	movaps	xmmword ptr [rbp + 0x150], xmm0
1408a4890: 0f 10 85 f8 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2f8]
1408a4897: 0f 29 85 80 01 00 00        	movaps	xmmword ptr [rbp + 0x180], xmm0
1408a489e: 48 8b 95 08 03 00 00        	mov	rdx, qword ptr [rbp + 0x308]
1408a48a5: 48 89 95 90 01 00 00        	mov	qword ptr [rbp + 0x190], rdx
1408a48ac: 0f 10 85 10 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x310]
1408a48b3: 0f 11 85 68 01 00 00        	movups	xmmword ptr [rbp + 0x168], xmm0
1408a48ba: 48 8b 95 20 03 00 00        	mov	rdx, qword ptr [rbp + 0x320]
1408a48c1: 48 89 95 78 01 00 00        	mov	qword ptr [rbp + 0x178], rdx
1408a48c8: 0f 10 85 90 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x390]
1408a48cf: 0f 10 8d a0 03 00 00        	movups	xmm1, xmmword ptr [rbp + 0x3a0]
1408a48d6: 0f 10 95 b0 03 00 00        	movups	xmm2, xmmword ptr [rbp + 0x3b0]
1408a48dd: 0f 10 9d c0 03 00 00        	movups	xmm3, xmmword ptr [rbp + 0x3c0]
1408a48e4: 0f 11 85 c8 01 00 00        	movups	xmmword ptr [rbp + 0x1c8], xmm0
1408a48eb: 0f 11 8d d8 01 00 00        	movups	xmmword ptr [rbp + 0x1d8], xmm1
1408a48f2: 0f 11 95 e8 01 00 00        	movups	xmmword ptr [rbp + 0x1e8], xmm2
1408a48f9: 0f 11 9d f8 01 00 00        	movups	xmmword ptr [rbp + 0x1f8], xmm3
1408a4900: 0f 10 85 d0 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x3d0]
1408a4907: 0f 11 85 08 02 00 00        	movups	xmmword ptr [rbp + 0x208], xmm0
1408a490e: 0f 10 85 e0 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x3e0]
1408a4915: 0f 11 85 18 02 00 00        	movups	xmmword ptr [rbp + 0x218], xmm0
1408a491c: 48 8b 95 38 03 00 00        	mov	rdx, qword ptr [rbp + 0x338]
1408a4923: 48 89 95 a8 01 00 00        	mov	qword ptr [rbp + 0x1a8], rdx
1408a492a: 0f 10 85 28 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x328]
1408a4931: 0f 11 85 98 01 00 00        	movups	xmmword ptr [rbp + 0x198], xmm0
1408a4938: 48 89 8d 30 02 00 00        	mov	qword ptr [rbp + 0x230], rcx
1408a493f: 48 89 85 28 02 00 00        	mov	qword ptr [rbp + 0x228], rax
1408a4946: c7 85 b0 01 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x1b0], 0x1
1408a4950: 89 b5 b4 01 00 00           	mov	dword ptr [rbp + 0x1b4], esi
1408a4956: 89 bd b8 01 00 00           	mov	dword ptr [rbp + 0x1b8], edi
1408a495c: c7 85 bc 01 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x1bc], 0x1
1408a4966: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
1408a496a: 48 8d 95 50 01 00 00        	lea	rdx, [rbp + 0x150]
1408a4971: e8 5a 8b 13 00              	call	0x1409dd4d0 <.text+0x9dc4d0>
1408a4976: 80 bd 16 04 00 00 08        	cmp	byte ptr [rbp + 0x416], 0x8
1408a497d: 74 0c                       	je	0x1408a498b <.text+0x8a398b>
1408a497f: 48 8d 8d 00 04 00 00        	lea	rcx, [rbp + 0x400]
1408a4986: e8 25 db 33 00              	call	0x140be24b0 <.text+0xbe14b0>
1408a498b: 0f 28 b5 80 04 00 00        	movaps	xmm6, xmmword ptr [rbp + 0x480]
1408a4992: 0f 28 bd 90 04 00 00        	movaps	xmm7, xmmword ptr [rbp + 0x490]
1408a4999: 48 81 c4 20 05 00 00        	add	rsp, 0x520
1408a49a0: 5b                          	pop	rbx
1408a49a1: 5f                          	pop	rdi
1408a49a2: 5e                          	pop	rsi
1408a49a3: 41 5c                       	pop	r12
1408a49a5: 41 5e                       	pop	r14
1408a49a7: 41 5f                       	pop	r15
1408a49a9: 5d                          	pop	rbp
1408a49aa: c3                          	ret
1408a49ab: 48 8d 0d be 69 1f 02        	lea	rcx, [rip + 0x21f69be]  # 0x142a9b370
1408a49b2: e8 59 49 03 02              	call	0x1428d9310 <.text+0x28d8310>
1408a49b7: cc                          	int3
1408a49b8: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
1408a49c0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1408a49c5: 55                          	push	rbp
1408a49c6: 41 57                       	push	r15
1408a49c8: 41 56                       	push	r14
1408a49ca: 41 54                       	push	r12
1408a49cc: 56                          	push	rsi
1408a49cd: 57                          	push	rdi
1408a49ce: 53                          	push	rbx
1408a49cf: 48 83 ec 60                 	sub	rsp, 0x60
1408a49d3: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1408a49da: 0f 29 7c 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm7
1408a49df: 0f 29 74 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm6
1408a49e4: 80 bd 16 04 00 00 08        	cmp	byte ptr [rbp + 0x416], 0x8
1408a49eb: 74 0c                       	je	0x1408a49f9 <.text+0x8a39f9>
1408a49ed: 48 8d 8d 00 04 00 00        	lea	rcx, [rbp + 0x400]
1408a49f4: e8 b7 da 33 00              	call	0x140be24b0 <.text+0xbe14b0>
1408a49f9: 0f 28 74 24 40              	movaps	xmm6, xmmword ptr [rsp + 0x40]
1408a49fe: 0f 28 7c 24 30              	movaps	xmm7, xmmword ptr [rsp + 0x30]
1408a4a03: 48 83 c4 60                 	add	rsp, 0x60
1408a4a07: 5b                          	pop	rbx
1408a4a08: 5f                          	pop	rdi
1408a4a09: 5e                          	pop	rsi
1408a4a0a: 41 5c                       	pop	r12
1408a4a0c: 41 5e                       	pop	r14
1408a4a0e: 41 5f                       	pop	r15
1408a4a10: 5d                          	pop	rbp
1408a4a11: c3                          	ret
1408a4a12: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nop	word ptr cs:[rax + rax]
1408a4a20: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1408a4a25: 55                          	push	rbp
1408a4a26: 41 57                       	push	r15
1408a4a28: 41 56                       	push	r14
1408a4a2a: 41 54                       	push	r12
1408a4a2c: 56                          	push	rsi
1408a4a2d: 57                          	push	rdi
1408a4a2e: 53                          	push	rbx
1408a4a2f: 48 83 ec 60                 	sub	rsp, 0x60
1408a4a33: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1408a4a3a: 0f 29 7c 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm7
1408a4a3f: 0f 29 74 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm6
1408a4a44: 48 8d 8d e0 02 00 00        	lea	rcx, [rbp + 0x2e0]
1408a4a4b: e8 60 25 ff ff              	call	0x140896fb0 <.text+0x895fb0>
1408a4a50: 0f 28 74 24 40              	movaps	xmm6, xmmword ptr [rsp + 0x40]
1408a4a55: 0f 28 7c 24 30              	movaps	xmm7, xmmword ptr [rsp + 0x30]
1408a4a5a: 48 83 c4 60                 	add	rsp, 0x60
1408a4a5e: 5b                          	pop	rbx
1408a4a5f: 5f                          	pop	rdi
1408a4a60: 5e                          	pop	rsi
1408a4a61: 41 5c                       	pop	r12
1408a4a63: 41 5e                       	pop	r14
1408a4a65: 41 5f                       	pop	r15
1408a4a67: 5d                          	pop	rbp
1408a4a68: c3                          	ret
1408a4a69: 0f 1f 80 00 00 00 00        	nop	dword ptr [rax]
1408a4a70: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1408a4a75: 55                          	push	rbp
1408a4a76: 41 57                       	push	r15
1408a4a78: 41 56                       	push	r14
1408a4a7a: 41 54                       	push	r12
1408a4a7c: 56                          	push	rsi
1408a4a7d: 57                          	push	rdi
1408a4a7e: 53                          	push	rbx
1408a4a7f: 48 83 ec 60                 	sub	rsp, 0x60
1408a4a83: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1408a4a8a: 0f 29 7c 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm7
1408a4a8f: 0f 29 74 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm6
1408a4a94: 48 8d 8d 50 01 00 00        	lea	rcx, [rbp + 0x150]
1408a4a9b: e8 10 25 ff ff              	call	0x140896fb0 <.text+0x895fb0>
1408a4aa0: 0f 28 74 24 40              	movaps	xmm6, xmmword ptr [rsp + 0x40]
1408a4aa5: 0f 28 7c 24 30              	movaps	xmm7, xmmword ptr [rsp + 0x30]
1408a4aaa: 48 83 c4 60                 	add	rsp, 0x60
1408a4aae: 5b                          	pop	rbx
1408a4aaf: 5f                          	pop	rdi
1408a4ab0: 5e                          	pop	rsi
1408a4ab1: 41 5c                       	pop	r12
1408a4ab3: 41 5e                       	pop	r14
1408a4ab5: 41 5f                       	pop	r15
1408a4ab7: 5d                          	pop	rbp
1408a4ab8: c3                          	ret
1408a4ab9: 0f 1f 80 00 00 00 00        	nop	dword ptr [rax]
1408a4ac0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1408a4ac5: 55                          	push	rbp
1408a4ac6: 41 57                       	push	r15
1408a4ac8: 41 56                       	push	r14
1408a4aca: 41 54                       	push	r12
1408a4acc: 56                          	push	rsi
1408a4acd: 57                          	push	rdi
1408a4ace: 53                          	push	rbx
1408a4acf: 48 83 ec 60                 	sub	rsp, 0x60
1408a4ad3: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1408a4ada: 0f 29 7c 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm7
1408a4adf: 0f 29 74 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm6
1408a4ae4: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
1408a4ae8: e8 c3 24 ff ff              	call	0x140896fb0 <.text+0x895fb0>
1408a4aed: 0f 28 74 24 40              	movaps	xmm6, xmmword ptr [rsp + 0x40]
1408a4af2: 0f 28 7c 24 30              	movaps	xmm7, xmmword ptr [rsp + 0x30]
1408a4af7: 48 83 c4 60                 	add	rsp, 0x60
1408a4afb: 5b                          	pop	rbx
1408a4afc: 5f                          	pop	rdi
1408a4afd: 5e                          	pop	rsi
1408a4afe: 41 5c                       	pop	r12
1408a4b00: 41 5e                       	pop	r14
1408a4b02: 41 5f                       	pop	r15
1408a4b04: 5d                          	pop	rbp
1408a4b05: c3                          	ret
1408a4b06: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
1408a4b10: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1408a4b15: 55                          	push	rbp
1408a4b16: 41 57                       	push	r15
1408a4b18: 41 56                       	push	r14
1408a4b1a: 41 54                       	push	r12
1408a4b1c: 56                          	push	rsi
1408a4b1d: 57                          	push	rdi
1408a4b1e: 53                          	push	rbx
1408a4b1f: 48 83 ec 60                 	sub	rsp, 0x60
1408a4b23: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1408a4b2a: 0f 29 7c 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm7
1408a4b2f: 0f 29 74 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm6
1408a4b34: 48 8d 8d 50 01 00 00        	lea	rcx, [rbp + 0x150]
1408a4b3b: e8 70 24 ff ff              	call	0x140896fb0 <.text+0x895fb0>
1408a4b40: 0f 28 74 24 40              	movaps	xmm6, xmmword ptr [rsp + 0x40]
1408a4b45: 0f 28 7c 24 30              	movaps	xmm7, xmmword ptr [rsp + 0x30]
1408a4b4a: 48 83 c4 60                 	add	rsp, 0x60
1408a4b4e: 5b                          	pop	rbx
1408a4b4f: 5f                          	pop	rdi
1408a4b50: 5e                          	pop	rsi
1408a4b51: 41 5c                       	pop	r12
1408a4b53: 41 5e                       	pop	r14
1408a4b55: 41 5f                       	pop	r15
1408a4b57: 5d                          	pop	rbp
1408a4b58: c3                          	ret
1408a4b59: 0f 1f 80 00 00 00 00        	nop	dword ptr [rax]
1408a4b60: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1408a4b65: 55                          	push	rbp
1408a4b66: 41 57                       	push	r15
1408a4b68: 41 56                       	push	r14
1408a4b6a: 41 54                       	push	r12
1408a4b6c: 56                          	push	rsi
1408a4b6d: 57                          	push	rdi
1408a4b6e: 53                          	push	rbx
1408a4b6f: 48 83 ec 60                 	sub	rsp, 0x60
1408a4b73: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1408a4b7a: 0f 29 7c 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm7
1408a4b7f: 0f 29 74 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm6
1408a4b84: 48 8d 8d e0 02 00 00        	lea	rcx, [rbp + 0x2e0]
1408a4b8b: e8 20 24 ff ff              	call	0x140896fb0 <.text+0x895fb0>
1408a4b90: 0f 28 74 24 40              	movaps	xmm6, xmmword ptr [rsp + 0x40]
1408a4b95: 0f 28 7c 24 30              	movaps	xmm7, xmmword ptr [rsp + 0x30]
1408a4b9a: 48 83 c4 60                 	add	rsp, 0x60
1408a4b9e: 5b                          	pop	rbx
1408a4b9f: 5f                          	pop	rdi
1408a4ba0: 5e                          	pop	rsi
1408a4ba1: 41 5c                       	pop	r12
1408a4ba3: 41 5e                       	pop	r14
1408a4ba5: 41 5f                       	pop	r15
1408a4ba7: 5d                          	pop	rbp
1408a4ba8: c3                          	ret
1408a4ba9: 0f 1f 80 00 00 00 00        	nop	dword ptr [rax]
1408a4bb0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1408a4bb5: 55                          	push	rbp
1408a4bb6: 41 57                       	push	r15
1408a4bb8: 41 56                       	push	r14
1408a4bba: 41 54                       	push	r12
1408a4bbc: 56                          	push	rsi
1408a4bbd: 57                          	push	rdi
1408a4bbe: 53                          	push	rbx
1408a4bbf: 48 83 ec 60                 	sub	rsp, 0x60
1408a4bc3: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1408a4bca: 0f 29 7c 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm7
1408a4bcf: 0f 29 74 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm6
1408a4bd4: 48 8d 8d 50 01 00 00        	lea	rcx, [rbp + 0x150]
1408a4bdb: e8 d0 23 ff ff              	call	0x140896fb0 <.text+0x895fb0>
1408a4be0: 0f 28 74 24 40              	movaps	xmm6, xmmword ptr [rsp + 0x40]
1408a4be5: 0f 28 7c 24 30              	movaps	xmm7, xmmword ptr [rsp + 0x30]
1408a4bea: 48 83 c4 60                 	add	rsp, 0x60
1408a4bee: 5b                          	pop	rbx
1408a4bef: 5f                          	pop	rdi
1408a4bf0: 5e                          	pop	rsi
1408a4bf1: 41 5c                       	pop	r12
1408a4bf3: 41 5e                       	pop	r14
1408a4bf5: 41 5f                       	pop	r15
1408a4bf7: 5d                          	pop	rbp
1408a4bf8: c3                          	ret
1408a4bf9: 0f 1f 80 00 00 00 00        	nop	dword ptr [rax]
1408a4c00: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1408a4c05: 55                          	push	rbp
1408a4c06: 41 57                       	push	r15
1408a4c08: 41 56                       	push	r14
1408a4c0a: 41 54                       	push	r12
1408a4c0c: 56                          	push	rsi
1408a4c0d: 57                          	push	rdi
1408a4c0e: 53                          	push	rbx
1408a4c0f: 48 83 ec 60                 	sub	rsp, 0x60
1408a4c13: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1408a4c1a: 0f 29 7c 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm7
1408a4c1f: 0f 29 74 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm6
1408a4c24: 48 8d 8d e0 02 00 00        	lea	rcx, [rbp + 0x2e0]
1408a4c2b: e8 80 23 ff ff              	call	0x140896fb0 <.text+0x895fb0>
1408a4c30: 0f 28 74 24 40              	movaps	xmm6, xmmword ptr [rsp + 0x40]
1408a4c35: 0f 28 7c 24 30              	movaps	xmm7, xmmword ptr [rsp + 0x30]
1408a4c3a: 48 83 c4 60                 	add	rsp, 0x60
1408a4c3e: 5b                          	pop	rbx
1408a4c3f: 5f                          	pop	rdi
1408a4c40: 5e                          	pop	rsi
1408a4c41: 41 5c                       	pop	r12
1408a4c43: 41 5e                       	pop	r14
1408a4c45: 41 5f                       	pop	r15
1408a4c47: 5d                          	pop	rbp
1408a4c48: c3                          	ret
1408a4c49: 0f 1f 80 00 00 00 00        	nop	dword ptr [rax]
1408a4c50: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1408a4c55: 55                          	push	rbp
1408a4c56: 41 57                       	push	r15
1408a4c58: 41 56                       	push	r14
1408a4c5a: 41 54                       	push	r12
1408a4c5c: 56                          	push	rsi
1408a4c5d: 57                          	push	rdi
1408a4c5e: 53                          	push	rbx
1408a4c5f: 48 83 ec 60                 	sub	rsp, 0x60
1408a4c63: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1408a4c6a: 0f 29 7c 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm7
1408a4c6f: 0f 29 74 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm6
1408a4c74: 48 8d 8d 50 01 00 00        	lea	rcx, [rbp + 0x150]
1408a4c7b: e8 30 23 ff ff              	call	0x140896fb0 <.text+0x895fb0>
1408a4c80: 0f 28 74 24 40              	movaps	xmm6, xmmword ptr [rsp + 0x40]
1408a4c85: 0f 28 7c 24 30              	movaps	xmm7, xmmword ptr [rsp + 0x30]
1408a4c8a: 48 83 c4 60                 	add	rsp, 0x60
1408a4c8e: 5b                          	pop	rbx
1408a4c8f: 5f                          	pop	rdi
1408a4c90: 5e                          	pop	rsi
1408a4c91: 41 5c                       	pop	r12
1408a4c93: 41 5e                       	pop	r14
1408a4c95: 41 5f                       	pop	r15
1408a4c97: 5d                          	pop	rbp
1408a4c98: c3                          	ret
