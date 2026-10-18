
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1400cd450: 55                          	push	rbp
1400cd451: 41 56                       	push	r14
1400cd453: 56                          	push	rsi
1400cd454: 57                          	push	rdi
1400cd455: 53                          	push	rbx
1400cd456: 48 81 ec a0 00 00 00        	sub	rsp, 0xa0
1400cd45d: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
1400cd465: 48 c7 45 18 fe ff ff ff     	mov	qword ptr [rbp + 0x18], -0x2
1400cd46d: 4c 89 ce                    	mov	rsi, r9
1400cd470: 4c 89 c7                    	mov	rdi, r8
1400cd473: 89 d3                       	mov	ebx, edx
1400cd475: 41 89 ce                    	mov	r14d, ecx
1400cd478: 4c 8d 0d b9 a2 86 02        	lea	r9, [rip + 0x286a2b9]   # 0x142937738
1400cd47f: 48 8d 4d d8                 	lea	rcx, [rbp - 0x28]
1400cd483: 41 b8 01 00 00 00           	mov	r8d, 0x1
1400cd489: 48 89 fa                    	mov	rdx, rdi
1400cd48c: e8 2f 0b be 00              	call	0x140cadfc0 <.text+0xcacfc0>
1400cd491: 31 c0                       	xor	eax, eax
1400cd493: 48 3b 45 d8                 	cmp	rax, qword ptr [rbp - 0x28]
1400cd497: 0f 81 d7 00 00 00           	jno	0x1400cd574 <.text+0xcc574>
1400cd49d: 4c 8d 0d c4 12 89 02        	lea	r9, [rip + 0x28912c4]   # 0x14295e768
1400cd4a4: 48 8d 4d d8                 	lea	rcx, [rbp - 0x28]
1400cd4a8: 41 b8 01 00 00 00           	mov	r8d, 0x1
1400cd4ae: 48 89 fa                    	mov	rdx, rdi
1400cd4b1: e8 0a 0b be 00              	call	0x140cadfc0 <.text+0xcacfc0>
1400cd4b6: 31 c0                       	xor	eax, eax
1400cd4b8: 48 3b 45 d8                 	cmp	rax, qword ptr [rbp - 0x28]
1400cd4bc: 0f 81 10 01 00 00           	jno	0x1400cd5d2 <.text+0xcc5d2>
1400cd4c2: 4c 8d 0d ef 67 8d 02        	lea	r9, [rip + 0x28d67ef]   # 0x1429a3cb8
1400cd4c9: 48 8d 4d d8                 	lea	rcx, [rbp - 0x28]
1400cd4cd: 41 b8 01 00 00 00           	mov	r8d, 0x1
1400cd4d3: 48 89 fa                    	mov	rdx, rdi
1400cd4d6: e8 e5 0a be 00              	call	0x140cadfc0 <.text+0xcacfc0>
1400cd4db: 31 c0                       	xor	eax, eax
1400cd4dd: 48 3b 45 d8                 	cmp	rax, qword ptr [rbp - 0x28]
1400cd4e1: 0f 81 49 01 00 00           	jno	0x1400cd630 <.text+0xcc630>
1400cd4e7: 4c 8d 0d 42 98 93 02        	lea	r9, [rip + 0x2939842]   # 0x142a06d30
1400cd4ee: 48 8d 4d d8                 	lea	rcx, [rbp - 0x28]
1400cd4f2: 41 b8 01 00 00 00           	mov	r8d, 0x1
1400cd4f8: 48 89 fa                    	mov	rdx, rdi
1400cd4fb: e8 c0 0a be 00              	call	0x140cadfc0 <.text+0xcacfc0>
1400cd500: 31 c0                       	xor	eax, eax
1400cd502: 48 3b 45 d8                 	cmp	rax, qword ptr [rbp - 0x28]
1400cd506: 0f 81 82 01 00 00           	jno	0x1400cd68e <.text+0xcc68e>
1400cd50c: 44 89 f1                    	mov	ecx, r14d
1400cd50f: 89 da                       	mov	edx, ebx
1400cd511: 49 89 f8                    	mov	r8, rdi
1400cd514: 49 89 f1                    	mov	r9, rsi
1400cd517: e8 44 ec 2a 00              	call	0x14037c160 <.text+0x37b160>
1400cd51c: 4c 8d 0d b5 1a 8f 02        	lea	r9, [rip + 0x28f1ab5]   # 0x1429befd8
1400cd523: 48 8d 4d d8                 	lea	rcx, [rbp - 0x28]
1400cd527: 41 b8 01 00 00 00           	mov	r8d, 0x1
1400cd52d: 48 89 fa                    	mov	rdx, rdi
1400cd530: e8 8b 0a be 00              	call	0x140cadfc0 <.text+0xcacfc0>
1400cd535: 31 c0                       	xor	eax, eax
1400cd537: 48 3b 45 d8                 	cmp	rax, qword ptr [rbp - 0x28]
1400cd53b: 0f 81 ab 01 00 00           	jno	0x1400cd6ec <.text+0xcc6ec>
1400cd541: 4c 8d 0d 28 f2 85 02        	lea	r9, [rip + 0x285f228]   # 0x14292c770
1400cd548: 48 8d 4d d8                 	lea	rcx, [rbp - 0x28]
1400cd54c: 41 b8 01 00 00 00           	mov	r8d, 0x1
1400cd552: 48 89 fa                    	mov	rdx, rdi
1400cd555: e8 66 0a be 00              	call	0x140cadfc0 <.text+0xcacfc0>
1400cd55a: 31 c0                       	xor	eax, eax
1400cd55c: 48 3b 45 d8                 	cmp	rax, qword ptr [rbp - 0x28]
1400cd560: 0f 81 e1 01 00 00           	jno	0x1400cd747 <.text+0xcc747>
1400cd566: 48 81 c4 a0 00 00 00        	add	rsp, 0xa0
1400cd56d: 5b                          	pop	rbx
1400cd56e: 5f                          	pop	rdi
1400cd56f: 5e                          	pop	rsi
1400cd570: 41 5e                       	pop	r14
1400cd572: 5d                          	pop	rbp
1400cd573: c3                          	ret
1400cd574: 48 8b 45 e8                 	mov	rax, qword ptr [rbp - 0x18]
1400cd578: 48 89 45 10                 	mov	qword ptr [rbp + 0x10], rax
1400cd57c: 0f 10 45 d8                 	movups	xmm0, xmmword ptr [rbp - 0x28]
1400cd580: 0f 29 45 00                 	movaps	xmmword ptr [rbp], xmm0
1400cd584: 48 89 e8                    	mov	rax, rbp
1400cd587: 48 89 45 f0                 	mov	qword ptr [rbp - 0x10], rax
1400cd58b: 48 8d 05 3e e5 f6 ff        	lea	rax, [rip - 0x91ac2]    # 0x14003bad0 <.text+0x3aad0>
1400cd592: 48 89 45 f8                 	mov	qword ptr [rbp - 0x8], rax
1400cd596: 48 8d 05 4b a2 86 02        	lea	rax, [rip + 0x286a24b]  # 0x1429377e8
1400cd59d: 48 89 45 a8                 	mov	qword ptr [rbp - 0x58], rax
1400cd5a1: 48 c7 45 b0 02 00 00 00     	mov	qword ptr [rbp - 0x50], 0x2
1400cd5a9: 48 c7 45 c8 00 00 00 00     	mov	qword ptr [rbp - 0x38], 0x0
1400cd5b1: 48 8d 45 f0                 	lea	rax, [rbp - 0x10]
1400cd5b5: 48 89 45 b8                 	mov	qword ptr [rbp - 0x48], rax
1400cd5b9: 48 c7 45 c0 01 00 00 00     	mov	qword ptr [rbp - 0x40], 0x1
1400cd5c1: 48 8d 4d a8                 	lea	rcx, [rbp - 0x58]
1400cd5c5: 48 89 f2                    	mov	rdx, rsi
1400cd5c8: e8 c3 bd 80 02              	call	0x1428d9390 <.text+0x28d8390>
1400cd5cd: e9 ce 01 00 00              	jmp	0x1400cd7a0 <.text+0xcc7a0>
1400cd5d2: 48 8b 45 e8                 	mov	rax, qword ptr [rbp - 0x18]
1400cd5d6: 48 89 45 10                 	mov	qword ptr [rbp + 0x10], rax
1400cd5da: 0f 10 45 d8                 	movups	xmm0, xmmword ptr [rbp - 0x28]
1400cd5de: 0f 29 45 00                 	movaps	xmmword ptr [rbp], xmm0
1400cd5e2: 48 89 e8                    	mov	rax, rbp
1400cd5e5: 48 89 45 f0                 	mov	qword ptr [rbp - 0x10], rax
1400cd5e9: 48 8d 05 e0 e4 f6 ff        	lea	rax, [rip - 0x91b20]    # 0x14003bad0 <.text+0x3aad0>
1400cd5f0: 48 89 45 f8                 	mov	qword ptr [rbp - 0x8], rax
1400cd5f4: 48 8d 05 1d 12 89 02        	lea	rax, [rip + 0x289121d]  # 0x14295e818
1400cd5fb: 48 89 45 a8                 	mov	qword ptr [rbp - 0x58], rax
1400cd5ff: 48 c7 45 b0 02 00 00 00     	mov	qword ptr [rbp - 0x50], 0x2
1400cd607: 48 c7 45 c8 00 00 00 00     	mov	qword ptr [rbp - 0x38], 0x0
1400cd60f: 48 8d 45 f0                 	lea	rax, [rbp - 0x10]
1400cd613: 48 89 45 b8                 	mov	qword ptr [rbp - 0x48], rax
1400cd617: 48 c7 45 c0 01 00 00 00     	mov	qword ptr [rbp - 0x40], 0x1
1400cd61f: 48 8d 4d a8                 	lea	rcx, [rbp - 0x58]
1400cd623: 48 89 f2                    	mov	rdx, rsi
1400cd626: e8 65 bd 80 02              	call	0x1428d9390 <.text+0x28d8390>
1400cd62b: e9 70 01 00 00              	jmp	0x1400cd7a0 <.text+0xcc7a0>
1400cd630: 48 8b 45 e8                 	mov	rax, qword ptr [rbp - 0x18]
1400cd634: 48 89 45 10                 	mov	qword ptr [rbp + 0x10], rax
1400cd638: 0f 10 45 d8                 	movups	xmm0, xmmword ptr [rbp - 0x28]
1400cd63c: 0f 29 45 00                 	movaps	xmmword ptr [rbp], xmm0
1400cd640: 48 89 e8                    	mov	rax, rbp
1400cd643: 48 89 45 f0                 	mov	qword ptr [rbp - 0x10], rax
1400cd647: 48 8d 05 82 e4 f6 ff        	lea	rax, [rip - 0x91b7e]    # 0x14003bad0 <.text+0x3aad0>
1400cd64e: 48 89 45 f8                 	mov	qword ptr [rbp - 0x8], rax
1400cd652: 48 8d 05 0f 67 8d 02        	lea	rax, [rip + 0x28d670f]  # 0x1429a3d68
1400cd659: 48 89 45 a8                 	mov	qword ptr [rbp - 0x58], rax
1400cd65d: 48 c7 45 b0 02 00 00 00     	mov	qword ptr [rbp - 0x50], 0x2
1400cd665: 48 c7 45 c8 00 00 00 00     	mov	qword ptr [rbp - 0x38], 0x0
1400cd66d: 48 8d 45 f0                 	lea	rax, [rbp - 0x10]
1400cd671: 48 89 45 b8                 	mov	qword ptr [rbp - 0x48], rax
1400cd675: 48 c7 45 c0 01 00 00 00     	mov	qword ptr [rbp - 0x40], 0x1
1400cd67d: 48 8d 4d a8                 	lea	rcx, [rbp - 0x58]
1400cd681: 48 89 f2                    	mov	rdx, rsi
1400cd684: e8 07 bd 80 02              	call	0x1428d9390 <.text+0x28d8390>
1400cd689: e9 12 01 00 00              	jmp	0x1400cd7a0 <.text+0xcc7a0>
1400cd68e: 48 8b 45 e8                 	mov	rax, qword ptr [rbp - 0x18]
1400cd692: 48 89 45 10                 	mov	qword ptr [rbp + 0x10], rax
1400cd696: 0f 10 45 d8                 	movups	xmm0, xmmword ptr [rbp - 0x28]
1400cd69a: 0f 29 45 00                 	movaps	xmmword ptr [rbp], xmm0
1400cd69e: 48 89 e8                    	mov	rax, rbp
1400cd6a1: 48 89 45 f0                 	mov	qword ptr [rbp - 0x10], rax
1400cd6a5: 48 8d 05 24 e4 f6 ff        	lea	rax, [rip - 0x91bdc]    # 0x14003bad0 <.text+0x3aad0>
1400cd6ac: 48 89 45 f8                 	mov	qword ptr [rbp - 0x8], rax
1400cd6b0: 48 8d 05 29 97 93 02        	lea	rax, [rip + 0x2939729]  # 0x142a06de0
1400cd6b7: 48 89 45 a8                 	mov	qword ptr [rbp - 0x58], rax
1400cd6bb: 48 c7 45 b0 02 00 00 00     	mov	qword ptr [rbp - 0x50], 0x2
1400cd6c3: 48 c7 45 c8 00 00 00 00     	mov	qword ptr [rbp - 0x38], 0x0
1400cd6cb: 48 8d 45 f0                 	lea	rax, [rbp - 0x10]
1400cd6cf: 48 89 45 b8                 	mov	qword ptr [rbp - 0x48], rax
1400cd6d3: 48 c7 45 c0 01 00 00 00     	mov	qword ptr [rbp - 0x40], 0x1
1400cd6db: 48 8d 4d a8                 	lea	rcx, [rbp - 0x58]
1400cd6df: 48 89 f2                    	mov	rdx, rsi
1400cd6e2: e8 a9 bc 80 02              	call	0x1428d9390 <.text+0x28d8390>
1400cd6e7: e9 b4 00 00 00              	jmp	0x1400cd7a0 <.text+0xcc7a0>
1400cd6ec: 48 8b 45 e8                 	mov	rax, qword ptr [rbp - 0x18]
1400cd6f0: 48 89 45 10                 	mov	qword ptr [rbp + 0x10], rax
1400cd6f4: 0f 10 45 d8                 	movups	xmm0, xmmword ptr [rbp - 0x28]
1400cd6f8: 0f 29 45 00                 	movaps	xmmword ptr [rbp], xmm0
1400cd6fc: 48 89 e8                    	mov	rax, rbp
1400cd6ff: 48 89 45 f0                 	mov	qword ptr [rbp - 0x10], rax
1400cd703: 48 8d 05 c6 e3 f6 ff        	lea	rax, [rip - 0x91c3a]    # 0x14003bad0 <.text+0x3aad0>
1400cd70a: 48 89 45 f8                 	mov	qword ptr [rbp - 0x8], rax
1400cd70e: 48 8d 05 73 19 8f 02        	lea	rax, [rip + 0x28f1973]  # 0x1429bf088
1400cd715: 48 89 45 a8                 	mov	qword ptr [rbp - 0x58], rax
1400cd719: 48 c7 45 b0 02 00 00 00     	mov	qword ptr [rbp - 0x50], 0x2
1400cd721: 48 c7 45 c8 00 00 00 00     	mov	qword ptr [rbp - 0x38], 0x0
1400cd729: 48 8d 45 f0                 	lea	rax, [rbp - 0x10]
1400cd72d: 48 89 45 b8                 	mov	qword ptr [rbp - 0x48], rax
1400cd731: 48 c7 45 c0 01 00 00 00     	mov	qword ptr [rbp - 0x40], 0x1
1400cd739: 48 8d 4d a8                 	lea	rcx, [rbp - 0x58]
1400cd73d: 48 89 f2                    	mov	rdx, rsi
1400cd740: e8 4b bc 80 02              	call	0x1428d9390 <.text+0x28d8390>
1400cd745: eb 59                       	jmp	0x1400cd7a0 <.text+0xcc7a0>
1400cd747: 48 8b 45 e8                 	mov	rax, qword ptr [rbp - 0x18]
1400cd74b: 48 89 45 10                 	mov	qword ptr [rbp + 0x10], rax
1400cd74f: 0f 10 45 d8                 	movups	xmm0, xmmword ptr [rbp - 0x28]
1400cd753: 0f 29 45 00                 	movaps	xmmword ptr [rbp], xmm0
1400cd757: 48 89 e8                    	mov	rax, rbp
1400cd75a: 48 89 45 f0                 	mov	qword ptr [rbp - 0x10], rax
1400cd75e: 48 8d 05 6b e3 f6 ff        	lea	rax, [rip - 0x91c95]    # 0x14003bad0 <.text+0x3aad0>
1400cd765: 48 89 45 f8                 	mov	qword ptr [rbp - 0x8], rax
1400cd769: 48 8d 05 b0 f0 85 02        	lea	rax, [rip + 0x285f0b0]  # 0x14292c820
1400cd770: 48 89 45 a8                 	mov	qword ptr [rbp - 0x58], rax
1400cd774: 48 c7 45 b0 02 00 00 00     	mov	qword ptr [rbp - 0x50], 0x2
