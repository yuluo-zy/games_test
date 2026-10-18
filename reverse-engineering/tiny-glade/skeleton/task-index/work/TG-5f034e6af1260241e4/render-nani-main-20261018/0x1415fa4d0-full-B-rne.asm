
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1415fa4d0: 41 57                       	push	r15
1415fa4d2: 41 56                       	push	r14
1415fa4d4: 41 55                       	push	r13
1415fa4d6: 41 54                       	push	r12
1415fa4d8: 56                          	push	rsi
1415fa4d9: 57                          	push	rdi
1415fa4da: 55                          	push	rbp
1415fa4db: 53                          	push	rbx
1415fa4dc: 48 83 ec 48                 	sub	rsp, 0x48
1415fa4e0: 0f b6 29                    	movzx	ebp, byte ptr [rcx]
1415fa4e3: 0f b6 41 01                 	movzx	eax, byte ptr [rcx + 0x1]
1415fa4e7: 48 89 44 24 40              	mov	qword ptr [rsp + 0x40], rax
1415fa4ec: 40 38 c5                    	cmp	bpl, al
1415fa4ef: 0f 83 fa 02 00 00           	jae	0x1415fa7ef <.text+0x15f97ef>
1415fa4f5: 48 8b 32                    	mov	rsi, qword ptr [rdx]
1415fa4f8: 48 8b 7a 08                 	mov	rdi, qword ptr [rdx + 0x8]
1415fa4fc: 8b 5a 10                    	mov	ebx, dword ptr [rdx + 0x10]
1415fa4ff: 89 e8                       	mov	eax, ebp
1415fa501: c1 e0 04                    	shl	eax, 0x4
1415fa504: 48 05 c0 06 00 00           	add	rax, 0x6c0
1415fa50a: 48 89 44 24 38              	mov	qword ptr [rsp + 0x38], rax
1415fa50f: 31 c9                       	xor	ecx, ecx
1415fa511: 48 8d 15 20 3f 68 01        	lea	rdx, [rip + 0x1683f20]  # 0x142c7e438
1415fa518: 48 89 6c 24 30              	mov	qword ptr [rsp + 0x30], rbp
1415fa51d: 0f 1f 00                    	nop	dword ptr [rax]
1415fa520: 89 c8                       	mov	eax, ecx
1415fa522: 44 0f b6 f1                 	movzx	r14d, cl
1415fa526: 4a 63 0c b2                 	movsxd	rcx, dword ptr [rdx + 4*r14]
1415fa52a: 48 01 d1                    	add	rcx, rdx
1415fa52d: ff e1                       	jmp	rcx
1415fa52f: b1 01                       	mov	cl, 0x1
1415fa531: 80 3d 64 a8 03 02 00        	cmp	byte ptr [rip + 0x203a864], 0x0 # 0x143634d9c
1415fa538: 75 e6                       	jne	0x1415fa520 <.text+0x15f9520>
1415fa53a: e9 e1 01 00 00              	jmp	0x1415fa720 <.text+0x15f9720>
1415fa53f: b1 11                       	mov	cl, 0x11
1415fa541: 80 3d 54 a8 03 02 00        	cmp	byte ptr [rip + 0x203a854], 0x0 # 0x143634d9c
1415fa548: 75 d6                       	jne	0x1415fa520 <.text+0x15f9520>
1415fa54a: e9 d1 01 00 00              	jmp	0x1415fa720 <.text+0x15f9720>
1415fa54f: b1 0f                       	mov	cl, 0xf
1415fa551: 80 3d 44 a8 03 02 00        	cmp	byte ptr [rip + 0x203a844], 0x0 # 0x143634d9c
1415fa558: 75 c6                       	jne	0x1415fa520 <.text+0x15f9520>
1415fa55a: e9 c1 01 00 00              	jmp	0x1415fa720 <.text+0x15f9720>
1415fa55f: b1 0c                       	mov	cl, 0xc
1415fa561: 80 3d 34 a8 03 02 00        	cmp	byte ptr [rip + 0x203a834], 0x0 # 0x143634d9c
1415fa568: 75 b6                       	jne	0x1415fa520 <.text+0x15f9520>
1415fa56a: e9 b1 01 00 00              	jmp	0x1415fa720 <.text+0x15f9720>
1415fa56f: b1 05                       	mov	cl, 0x5
1415fa571: 80 3d 24 a8 03 02 00        	cmp	byte ptr [rip + 0x203a824], 0x0 # 0x143634d9c
1415fa578: 75 a6                       	jne	0x1415fa520 <.text+0x15f9520>
1415fa57a: e9 a1 01 00 00              	jmp	0x1415fa720 <.text+0x15f9720>
1415fa57f: b1 0d                       	mov	cl, 0xd
1415fa581: 80 3d 14 a8 03 02 00        	cmp	byte ptr [rip + 0x203a814], 0x0 # 0x143634d9c
1415fa588: 75 96                       	jne	0x1415fa520 <.text+0x15f9520>
1415fa58a: e9 91 01 00 00              	jmp	0x1415fa720 <.text+0x15f9720>
1415fa58f: b1 0a                       	mov	cl, 0xa
1415fa591: 80 3d 04 a8 03 02 00        	cmp	byte ptr [rip + 0x203a804], 0x0 # 0x143634d9c
1415fa598: 75 86                       	jne	0x1415fa520 <.text+0x15f9520>
1415fa59a: e9 81 01 00 00              	jmp	0x1415fa720 <.text+0x15f9720>
1415fa59f: b1 1a                       	mov	cl, 0x1a
1415fa5a1: 80 3d f4 a7 03 02 00        	cmp	byte ptr [rip + 0x203a7f4], 0x0 # 0x143634d9c
1415fa5a8: 0f 85 72 ff ff ff           	jne	0x1415fa520 <.text+0x15f9520>
1415fa5ae: e9 6d 01 00 00              	jmp	0x1415fa720 <.text+0x15f9720>
1415fa5b3: b1 19                       	mov	cl, 0x19
1415fa5b5: 80 3d e0 a7 03 02 00        	cmp	byte ptr [rip + 0x203a7e0], 0x0 # 0x143634d9c
1415fa5bc: 0f 85 5e ff ff ff           	jne	0x1415fa520 <.text+0x15f9520>
1415fa5c2: e9 59 01 00 00              	jmp	0x1415fa720 <.text+0x15f9720>
1415fa5c7: b1 03                       	mov	cl, 0x3
1415fa5c9: 80 3d cc a7 03 02 00        	cmp	byte ptr [rip + 0x203a7cc], 0x0 # 0x143634d9c
1415fa5d0: 0f 85 4a ff ff ff           	jne	0x1415fa520 <.text+0x15f9520>
1415fa5d6: e9 45 01 00 00              	jmp	0x1415fa720 <.text+0x15f9720>
1415fa5db: b1 10                       	mov	cl, 0x10
1415fa5dd: 80 3d b8 a7 03 02 00        	cmp	byte ptr [rip + 0x203a7b8], 0x0 # 0x143634d9c
1415fa5e4: 0f 85 36 ff ff ff           	jne	0x1415fa520 <.text+0x15f9520>
1415fa5ea: e9 31 01 00 00              	jmp	0x1415fa720 <.text+0x15f9720>
1415fa5ef: b1 04                       	mov	cl, 0x4
1415fa5f1: 80 3d a4 a7 03 02 00        	cmp	byte ptr [rip + 0x203a7a4], 0x0 # 0x143634d9c
1415fa5f8: 0f 85 22 ff ff ff           	jne	0x1415fa520 <.text+0x15f9520>
1415fa5fe: e9 1d 01 00 00              	jmp	0x1415fa720 <.text+0x15f9720>
1415fa603: b1 08                       	mov	cl, 0x8
1415fa605: 80 3d 90 a7 03 02 00        	cmp	byte ptr [rip + 0x203a790], 0x0 # 0x143634d9c
1415fa60c: 0f 85 0e ff ff ff           	jne	0x1415fa520 <.text+0x15f9520>
1415fa612: e9 09 01 00 00              	jmp	0x1415fa720 <.text+0x15f9720>
1415fa617: b1 02                       	mov	cl, 0x2
1415fa619: 80 3d 7c a7 03 02 00        	cmp	byte ptr [rip + 0x203a77c], 0x0 # 0x143634d9c
1415fa620: 0f 85 fa fe ff ff           	jne	0x1415fa520 <.text+0x15f9520>
1415fa626: e9 f5 00 00 00              	jmp	0x1415fa720 <.text+0x15f9720>
1415fa62b: b1 12                       	mov	cl, 0x12
1415fa62d: 80 3d 68 a7 03 02 00        	cmp	byte ptr [rip + 0x203a768], 0x0 # 0x143634d9c
1415fa634: 0f 85 e6 fe ff ff           	jne	0x1415fa520 <.text+0x15f9520>
1415fa63a: e9 e1 00 00 00              	jmp	0x1415fa720 <.text+0x15f9720>
1415fa63f: b1 15                       	mov	cl, 0x15
1415fa641: 80 3d 54 a7 03 02 00        	cmp	byte ptr [rip + 0x203a754], 0x0 # 0x143634d9c
1415fa648: 0f 85 d2 fe ff ff           	jne	0x1415fa520 <.text+0x15f9520>
1415fa64e: e9 cd 00 00 00              	jmp	0x1415fa720 <.text+0x15f9720>
1415fa653: b1 06                       	mov	cl, 0x6
1415fa655: 80 3d 40 a7 03 02 00        	cmp	byte ptr [rip + 0x203a740], 0x0 # 0x143634d9c
1415fa65c: 0f 85 be fe ff ff           	jne	0x1415fa520 <.text+0x15f9520>
1415fa662: e9 b9 00 00 00              	jmp	0x1415fa720 <.text+0x15f9720>
1415fa667: b1 16                       	mov	cl, 0x16
1415fa669: 80 3d 2c a7 03 02 00        	cmp	byte ptr [rip + 0x203a72c], 0x0 # 0x143634d9c
1415fa670: 0f 85 aa fe ff ff           	jne	0x1415fa520 <.text+0x15f9520>
1415fa676: e9 a5 00 00 00              	jmp	0x1415fa720 <.text+0x15f9720>
1415fa67b: b1 0e                       	mov	cl, 0xe
1415fa67d: 80 3d 18 a7 03 02 00        	cmp	byte ptr [rip + 0x203a718], 0x0 # 0x143634d9c
1415fa684: 0f 85 96 fe ff ff           	jne	0x1415fa520 <.text+0x15f9520>
1415fa68a: e9 91 00 00 00              	jmp	0x1415fa720 <.text+0x15f9720>
1415fa68f: b1 07                       	mov	cl, 0x7
1415fa691: 80 3d 04 a7 03 02 00        	cmp	byte ptr [rip + 0x203a704], 0x0 # 0x143634d9c
1415fa698: 0f 85 82 fe ff ff           	jne	0x1415fa520 <.text+0x15f9520>
1415fa69e: e9 7d 00 00 00              	jmp	0x1415fa720 <.text+0x15f9720>
1415fa6a3: b1 13                       	mov	cl, 0x13
1415fa6a5: 80 3d f0 a6 03 02 00        	cmp	byte ptr [rip + 0x203a6f0], 0x0 # 0x143634d9c
1415fa6ac: 0f 85 6e fe ff ff           	jne	0x1415fa520 <.text+0x15f9520>
1415fa6b2: eb 6c                       	jmp	0x1415fa720 <.text+0x15f9720>
1415fa6b4: b1 0b                       	mov	cl, 0xb
1415fa6b6: 80 3d df a6 03 02 00        	cmp	byte ptr [rip + 0x203a6df], 0x0 # 0x143634d9c
1415fa6bd: 0f 85 5d fe ff ff           	jne	0x1415fa520 <.text+0x15f9520>
1415fa6c3: eb 5b                       	jmp	0x1415fa720 <.text+0x15f9720>
1415fa6c5: b1 09                       	mov	cl, 0x9
1415fa6c7: 80 3d ce a6 03 02 00        	cmp	byte ptr [rip + 0x203a6ce], 0x0 # 0x143634d9c
1415fa6ce: 0f 85 4c fe ff ff           	jne	0x1415fa520 <.text+0x15f9520>
1415fa6d4: eb 4a                       	jmp	0x1415fa720 <.text+0x15f9720>
1415fa6d6: b1 17                       	mov	cl, 0x17
1415fa6d8: 80 3d bd a6 03 02 00        	cmp	byte ptr [rip + 0x203a6bd], 0x0 # 0x143634d9c
1415fa6df: 0f 85 3b fe ff ff           	jne	0x1415fa520 <.text+0x15f9520>
1415fa6e5: eb 39                       	jmp	0x1415fa720 <.text+0x15f9720>
1415fa6e7: b1 14                       	mov	cl, 0x14
1415fa6e9: 80 3d ac a6 03 02 00        	cmp	byte ptr [rip + 0x203a6ac], 0x0 # 0x143634d9c
1415fa6f0: 0f 85 2a fe ff ff           	jne	0x1415fa520 <.text+0x15f9520>
1415fa6f6: eb 28                       	jmp	0x1415fa720 <.text+0x15f9720>
1415fa6f8: b1 18                       	mov	cl, 0x18
1415fa6fa: 80 3d 9b a6 03 02 00        	cmp	byte ptr [rip + 0x203a69b], 0x0 # 0x143634d9c
1415fa701: 0f 85 19 fe ff ff           	jne	0x1415fa520 <.text+0x15f9520>
1415fa707: eb 17                       	jmp	0x1415fa720 <.text+0x15f9720>
1415fa709: b1 1b                       	mov	cl, 0x1b
1415fa70b: 80 3d 8a a6 03 02 00        	cmp	byte ptr [rip + 0x203a68a], 0x0 # 0x143634d9c
1415fa712: 0f 85 08 fe ff ff           	jne	0x1415fa520 <.text+0x15f9520>
1415fa718: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
1415fa720: 3c 01                       	cmp	al, 0x1
1415fa722: 77 0d                       	ja	0x1415fa731 <.text+0x15f9731>
1415fa724: 80 3d 72 a6 03 02 00        	cmp	byte ptr [rip + 0x203a672], 0x0 # 0x143634d9d
1415fa72b: 0f 84 ef fd ff ff           	je	0x1415fa520 <.text+0x15f9520>
1415fa731: 89 4c 24 28                 	mov	dword ptr [rsp + 0x28], ecx
1415fa735: c0 e0 02                    	shl	al, 0x2
1415fa738: 44 0f b6 e8                 	movzx	r13d, al
1415fa73c: 41 c1 e6 06                 	shl	r14d, 0x6
1415fa740: 4c 03 74 24 38              	add	r14, qword ptr [rsp + 0x38]
1415fa745: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
1415fa750: 41 8d 44 2d 00              	lea	eax, [r13 + rbp]
1415fa755: 89 44 24 2c                 	mov	dword ptr [rsp + 0x2c], eax
1415fa759: 4c 8d 44 24 2c              	lea	r8, [rsp + 0x2c]
1415fa75e: 41 b9 04 00 00 00           	mov	r9d, 0x4
1415fa764: 48 89 f1                    	mov	rcx, rsi
1415fa767: 48 89 fa                    	mov	rdx, rdi
1415fa76a: e8 e1 d2 0f 00              	call	0x1416f7a50 <.text+0x16f6a50>
1415fa76f: 4c 8b 26                    	mov	r12, qword ptr [rsi]
1415fa772: 4c 8b 7e 08                 	mov	r15, qword ptr [rsi + 0x8]
1415fa776: 41 80 bf ba 0a 00 00 01     	cmp	byte ptr [r15 + 0xaba], 0x1
1415fa77e: 75 0d                       	jne	0x1415fa78d <.text+0x15f978d>
1415fa780: 49 8b 54 24 48              	mov	rdx, qword ptr [r12 + 0x48]
1415fa785: 4c 89 f9                    	mov	rcx, r15
1415fa788: e8 03 39 5f ff              	call	0x140bee090 <.text+0xbed090>
1415fa78d: f0                          	lock
1415fa78e: 49 ff 87 a0 0a 00 00        	inc	qword ptr [r15 + 0xaa0]
1415fa795: 4d 8b 64 24 48              	mov	r12, qword ptr [r12 + 0x48]
1415fa79a: 48 8b 4e 10                 	mov	rcx, qword ptr [rsi + 0x10]
1415fa79e: 89 da                       	mov	edx, ebx
1415fa7a0: e8 2b d2 5e ff              	call	0x140be79d0 <.text+0xbe69d0>
1415fa7a5: 4c 89 f9                    	mov	rcx, r15
1415fa7a8: 48 89 c2                    	mov	rdx, rax
1415fa7ab: 45 31 c0                    	xor	r8d, r8d
1415fa7ae: e8 bd 4c 5f ff              	call	0x140bef470 <.text+0xbee470>
1415fa7b3: a8 01                       	test	al, 0x1
1415fa7b5: 0f 84 e8 01 00 00           	je	0x1415fa9a3 <.text+0x15f99a3>
1415fa7bb: 48 ff c5                    	inc	rbp
1415fa7be: 4c 89 e1                    	mov	rcx, r12
1415fa7c1: 4d 89 f0                    	mov	r8, r14
1415fa7c4: 41 ff 97 68 03 00 00        	call	qword ptr [r15 + 0x368]
1415fa7cb: 49 83 c6 10                 	add	r14, 0x10
1415fa7cf: 48 3b 6c 24 40              	cmp	rbp, qword ptr [rsp + 0x40]
1415fa7d4: 0f 82 76 ff ff ff           	jb	0x1415fa750 <.text+0x15f9750>
1415fa7da: 48 8b 6c 24 30              	mov	rbp, qword ptr [rsp + 0x30]
1415fa7df: 48 8d 15 52 3c 68 01        	lea	rdx, [rip + 0x1683c52]  # 0x142c7e438
1415fa7e6: 8b 4c 24 28                 	mov	ecx, dword ptr [rsp + 0x28]
1415fa7ea: e9 31 fd ff ff              	jmp	0x1415fa520 <.text+0x15f9520>
1415fa7ef: f6 05 a6 a5 03 02 01        	test	byte ptr [rip + 0x203a5a6], 0x1 # 0x143634d9c
1415fa7f6: 0f 84 86 00 00 00           	je	0x1415fa882 <.text+0x15f9882>
1415fa7fc: 31 c9                       	xor	ecx, ecx
1415fa7fe: 48 8d 05 c3 3b 68 01        	lea	rax, [rip + 0x1683bc3]  # 0x142c7e3c8
1415fa805: eb 09                       	jmp	0x1415fa810 <.text+0x15f9810>
1415fa807: b1 18                       	mov	cl, 0x18
1415fa809: 0f 1f 80 00 00 00 00        	nop	dword ptr [rax]
1415fa810: 0f b6 c9                    	movzx	ecx, cl
1415fa813: 48 63 14 88                 	movsxd	rdx, dword ptr [rax + 4*rcx]
1415fa817: 48 01 c2                    	add	rdx, rax
1415fa81a: b1 02                       	mov	cl, 0x2
1415fa81c: ff e2                       	jmp	rdx
1415fa81e: b1 01                       	mov	cl, 0x1
1415fa820: eb ee                       	jmp	0x1415fa810 <.text+0x15f9810>
1415fa822: b1 14                       	mov	cl, 0x14
1415fa824: eb ea                       	jmp	0x1415fa810 <.text+0x15f9810>
1415fa826: b1 0f                       	mov	cl, 0xf
1415fa828: eb e6                       	jmp	0x1415fa810 <.text+0x15f9810>
1415fa82a: b1 1a                       	mov	cl, 0x1a
1415fa82c: eb e2                       	jmp	0x1415fa810 <.text+0x15f9810>
1415fa82e: b1 16                       	mov	cl, 0x16
1415fa830: eb de                       	jmp	0x1415fa810 <.text+0x15f9810>
1415fa832: b1 09                       	mov	cl, 0x9
1415fa834: eb da                       	jmp	0x1415fa810 <.text+0x15f9810>
1415fa836: b1 07                       	mov	cl, 0x7
1415fa838: eb d6                       	jmp	0x1415fa810 <.text+0x15f9810>
1415fa83a: b1 03                       	mov	cl, 0x3
1415fa83c: eb d2                       	jmp	0x1415fa810 <.text+0x15f9810>
1415fa83e: b1 08                       	mov	cl, 0x8
1415fa840: eb ce                       	jmp	0x1415fa810 <.text+0x15f9810>
1415fa842: b1 19                       	mov	cl, 0x19
1415fa844: eb ca                       	jmp	0x1415fa810 <.text+0x15f9810>
1415fa846: b1 0a                       	mov	cl, 0xa
1415fa848: eb c6                       	jmp	0x1415fa810 <.text+0x15f9810>
1415fa84a: b1 13                       	mov	cl, 0x13
1415fa84c: eb c2                       	jmp	0x1415fa810 <.text+0x15f9810>
1415fa84e: b1 06                       	mov	cl, 0x6
1415fa850: eb be                       	jmp	0x1415fa810 <.text+0x15f9810>
1415fa852: b1 05                       	mov	cl, 0x5
1415fa854: eb ba                       	jmp	0x1415fa810 <.text+0x15f9810>
1415fa856: b1 04                       	mov	cl, 0x4
1415fa858: eb b6                       	jmp	0x1415fa810 <.text+0x15f9810>
1415fa85a: b1 1b                       	mov	cl, 0x1b
1415fa85c: eb b2                       	jmp	0x1415fa810 <.text+0x15f9810>
1415fa85e: b1 0e                       	mov	cl, 0xe
1415fa860: eb ae                       	jmp	0x1415fa810 <.text+0x15f9810>
1415fa862: b1 15                       	mov	cl, 0x15
1415fa864: eb aa                       	jmp	0x1415fa810 <.text+0x15f9810>
1415fa866: b1 0d                       	mov	cl, 0xd
1415fa868: eb a6                       	jmp	0x1415fa810 <.text+0x15f9810>
1415fa86a: b1 11                       	mov	cl, 0x11
1415fa86c: eb a2                       	jmp	0x1415fa810 <.text+0x15f9810>
1415fa86e: b1 10                       	mov	cl, 0x10
1415fa870: eb 9e                       	jmp	0x1415fa810 <.text+0x15f9810>
1415fa872: b1 12                       	mov	cl, 0x12
1415fa874: eb 9a                       	jmp	0x1415fa810 <.text+0x15f9810>
1415fa876: b1 0b                       	mov	cl, 0xb
1415fa878: eb 96                       	jmp	0x1415fa810 <.text+0x15f9810>
1415fa87a: b1 17                       	mov	cl, 0x17
1415fa87c: eb 92                       	jmp	0x1415fa810 <.text+0x15f9810>
1415fa87e: b1 0c                       	mov	cl, 0xc
1415fa880: eb 8e                       	jmp	0x1415fa810 <.text+0x15f9810>
1415fa882: f6 05 14 a5 03 02 01        	test	byte ptr [rip + 0x203a514], 0x1 # 0x143634d9d
1415fa889: 0f 84 83 00 00 00           	je	0x1415fa912 <.text+0x15f9912>
1415fa88f: 31 c9                       	xor	ecx, ecx
1415fa891: 48 8d 05 c0 3a 68 01        	lea	rax, [rip + 0x1683ac0]  # 0x142c7e358
1415fa898: eb 06                       	jmp	0x1415fa8a0 <.text+0x15f98a0>
1415fa89a: b1 18                       	mov	cl, 0x18
1415fa89c: 0f 1f 40 00                 	nop	dword ptr [rax]
1415fa8a0: 0f b6 c9                    	movzx	ecx, cl
1415fa8a3: 48 63 14 88                 	movsxd	rdx, dword ptr [rax + 4*rcx]
1415fa8a7: 48 01 c2                    	add	rdx, rax
1415fa8aa: b1 02                       	mov	cl, 0x2
1415fa8ac: ff e2                       	jmp	rdx
1415fa8ae: b1 01                       	mov	cl, 0x1
1415fa8b0: eb ee                       	jmp	0x1415fa8a0 <.text+0x15f98a0>
1415fa8b2: b1 14                       	mov	cl, 0x14
1415fa8b4: eb ea                       	jmp	0x1415fa8a0 <.text+0x15f98a0>
1415fa8b6: b1 0f                       	mov	cl, 0xf
1415fa8b8: eb e6                       	jmp	0x1415fa8a0 <.text+0x15f98a0>
1415fa8ba: b1 1a                       	mov	cl, 0x1a
1415fa8bc: eb e2                       	jmp	0x1415fa8a0 <.text+0x15f98a0>
1415fa8be: b1 16                       	mov	cl, 0x16
1415fa8c0: eb de                       	jmp	0x1415fa8a0 <.text+0x15f98a0>
1415fa8c2: b1 09                       	mov	cl, 0x9
1415fa8c4: eb da                       	jmp	0x1415fa8a0 <.text+0x15f98a0>
1415fa8c6: b1 07                       	mov	cl, 0x7
1415fa8c8: eb d6                       	jmp	0x1415fa8a0 <.text+0x15f98a0>
1415fa8ca: b1 03                       	mov	cl, 0x3
1415fa8cc: eb d2                       	jmp	0x1415fa8a0 <.text+0x15f98a0>
1415fa8ce: b1 08                       	mov	cl, 0x8
1415fa8d0: eb ce                       	jmp	0x1415fa8a0 <.text+0x15f98a0>
1415fa8d2: b1 19                       	mov	cl, 0x19
1415fa8d4: eb ca                       	jmp	0x1415fa8a0 <.text+0x15f98a0>
1415fa8d6: b1 0a                       	mov	cl, 0xa
1415fa8d8: eb c6                       	jmp	0x1415fa8a0 <.text+0x15f98a0>
1415fa8da: b1 13                       	mov	cl, 0x13
1415fa8dc: eb c2                       	jmp	0x1415fa8a0 <.text+0x15f98a0>
1415fa8de: b1 06                       	mov	cl, 0x6
1415fa8e0: eb be                       	jmp	0x1415fa8a0 <.text+0x15f98a0>
1415fa8e2: b1 05                       	mov	cl, 0x5
1415fa8e4: eb ba                       	jmp	0x1415fa8a0 <.text+0x15f98a0>
1415fa8e6: b1 04                       	mov	cl, 0x4
1415fa8e8: eb b6                       	jmp	0x1415fa8a0 <.text+0x15f98a0>
1415fa8ea: b1 1b                       	mov	cl, 0x1b
1415fa8ec: eb b2                       	jmp	0x1415fa8a0 <.text+0x15f98a0>
1415fa8ee: b1 0e                       	mov	cl, 0xe
1415fa8f0: eb ae                       	jmp	0x1415fa8a0 <.text+0x15f98a0>
1415fa8f2: b1 15                       	mov	cl, 0x15
1415fa8f4: eb aa                       	jmp	0x1415fa8a0 <.text+0x15f98a0>
1415fa8f6: b1 0d                       	mov	cl, 0xd
1415fa8f8: eb a6                       	jmp	0x1415fa8a0 <.text+0x15f98a0>
1415fa8fa: b1 11                       	mov	cl, 0x11
1415fa8fc: eb a2                       	jmp	0x1415fa8a0 <.text+0x15f98a0>
1415fa8fe: b1 10                       	mov	cl, 0x10
1415fa900: eb 9e                       	jmp	0x1415fa8a0 <.text+0x15f98a0>
1415fa902: b1 12                       	mov	cl, 0x12
1415fa904: eb 9a                       	jmp	0x1415fa8a0 <.text+0x15f98a0>
1415fa906: b1 0b                       	mov	cl, 0xb
1415fa908: eb 96                       	jmp	0x1415fa8a0 <.text+0x15f98a0>
1415fa90a: b1 17                       	mov	cl, 0x17
1415fa90c: eb 92                       	jmp	0x1415fa8a0 <.text+0x15f98a0>
1415fa90e: b1 0c                       	mov	cl, 0xc
1415fa910: eb 8e                       	jmp	0x1415fa8a0 <.text+0x15f98a0>
1415fa912: 31 c9                       	xor	ecx, ecx
1415fa914: 48 8d 05 cd 39 68 01        	lea	rax, [rip + 0x16839cd]  # 0x142c7e2e8
1415fa91b: eb 03                       	jmp	0x1415fa920 <.text+0x15f9920>
1415fa91d: b1 18                       	mov	cl, 0x18
1415fa91f: 90                          	nop
1415fa920: 0f b6 c9                    	movzx	ecx, cl
1415fa923: 48 63 14 88                 	movsxd	rdx, dword ptr [rax + 4*rcx]
1415fa927: 48 01 c2                    	add	rdx, rax
1415fa92a: b1 02                       	mov	cl, 0x2
1415fa92c: ff e2                       	jmp	rdx
1415fa92e: b1 01                       	mov	cl, 0x1
1415fa930: eb ee                       	jmp	0x1415fa920 <.text+0x15f9920>
1415fa932: b1 14                       	mov	cl, 0x14
1415fa934: eb ea                       	jmp	0x1415fa920 <.text+0x15f9920>
1415fa936: b1 0f                       	mov	cl, 0xf
1415fa938: eb e6                       	jmp	0x1415fa920 <.text+0x15f9920>
1415fa93a: b1 1a                       	mov	cl, 0x1a
1415fa93c: eb e2                       	jmp	0x1415fa920 <.text+0x15f9920>
1415fa93e: b1 16                       	mov	cl, 0x16
1415fa940: eb de                       	jmp	0x1415fa920 <.text+0x15f9920>
1415fa942: b1 09                       	mov	cl, 0x9
1415fa944: eb da                       	jmp	0x1415fa920 <.text+0x15f9920>
1415fa946: b1 07                       	mov	cl, 0x7
1415fa948: eb d6                       	jmp	0x1415fa920 <.text+0x15f9920>
1415fa94a: b1 03                       	mov	cl, 0x3
1415fa94c: eb d2                       	jmp	0x1415fa920 <.text+0x15f9920>
1415fa94e: b1 08                       	mov	cl, 0x8
1415fa950: eb ce                       	jmp	0x1415fa920 <.text+0x15f9920>
1415fa952: b1 19                       	mov	cl, 0x19
1415fa954: eb ca                       	jmp	0x1415fa920 <.text+0x15f9920>
1415fa956: b1 0a                       	mov	cl, 0xa
1415fa958: eb c6                       	jmp	0x1415fa920 <.text+0x15f9920>
1415fa95a: b1 13                       	mov	cl, 0x13
1415fa95c: eb c2                       	jmp	0x1415fa920 <.text+0x15f9920>
1415fa95e: b1 06                       	mov	cl, 0x6
1415fa960: eb be                       	jmp	0x1415fa920 <.text+0x15f9920>
1415fa962: b1 05                       	mov	cl, 0x5
1415fa964: eb ba                       	jmp	0x1415fa920 <.text+0x15f9920>
1415fa966: b1 04                       	mov	cl, 0x4
1415fa968: eb b6                       	jmp	0x1415fa920 <.text+0x15f9920>
1415fa96a: b1 1b                       	mov	cl, 0x1b
1415fa96c: eb b2                       	jmp	0x1415fa920 <.text+0x15f9920>
1415fa96e: b1 0e                       	mov	cl, 0xe
1415fa970: eb ae                       	jmp	0x1415fa920 <.text+0x15f9920>
1415fa972: b1 15                       	mov	cl, 0x15
1415fa974: eb aa                       	jmp	0x1415fa920 <.text+0x15f9920>
1415fa976: b1 0d                       	mov	cl, 0xd
1415fa978: eb a6                       	jmp	0x1415fa920 <.text+0x15f9920>
1415fa97a: b1 11                       	mov	cl, 0x11
1415fa97c: eb a2                       	jmp	0x1415fa920 <.text+0x15f9920>
1415fa97e: b1 10                       	mov	cl, 0x10
1415fa980: eb 9e                       	jmp	0x1415fa920 <.text+0x15f9920>
1415fa982: b1 12                       	mov	cl, 0x12
1415fa984: eb 9a                       	jmp	0x1415fa920 <.text+0x15f9920>
1415fa986: b1 0b                       	mov	cl, 0xb
1415fa988: eb 96                       	jmp	0x1415fa920 <.text+0x15f9920>
1415fa98a: b1 17                       	mov	cl, 0x17
1415fa98c: eb 92                       	jmp	0x1415fa920 <.text+0x15f9920>
1415fa98e: b1 0c                       	mov	cl, 0xc
1415fa990: eb 8e                       	jmp	0x1415fa920 <.text+0x15f9920>
1415fa992: 48 83 c4 48                 	add	rsp, 0x48
1415fa996: 5b                          	pop	rbx
1415fa997: 5d                          	pop	rbp
1415fa998: 5f                          	pop	rdi
1415fa999: 5e                          	pop	rsi
1415fa99a: 41 5c                       	pop	r12
1415fa99c: 41 5d                       	pop	r13
1415fa99e: 41 5e                       	pop	r14
1415fa9a0: 41 5f                       	pop	r15
1415fa9a2: c3                          	ret
1415fa9a3: 48 8d 0d 06 b8 4b 01        	lea	rcx, [rip + 0x14bb806]  # 0x142ab61b0
1415fa9aa: e8 61 e9 2d 01              	call	0x1428d9310 <.text+0x28d8310>
1415fa9af: cc                          	int3
