
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1404d9580: 55                          	push	rbp
1404d9581: 41 57                       	push	r15
1404d9583: 41 56                       	push	r14
1404d9585: 41 55                       	push	r13
1404d9587: 41 54                       	push	r12
1404d9589: 56                          	push	rsi
1404d958a: 57                          	push	rdi
1404d958b: 53                          	push	rbx
1404d958c: b8 d8 15 00 00              	mov	eax, 0x15d8
1404d9591: e8 aa 14 3c 02              	call	0x14289aa40 <.text+0x2899a40>
1404d9596: 48 29 c4                    	sub	rsp, rax
1404d9599: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
1404d95a1: 66 44 0f 7f bd 40 15 00 00  	movdqa	xmmword ptr [rbp + 0x1540], xmm15
1404d95aa: 44 0f 29 b5 30 15 00 00     	movaps	xmmword ptr [rbp + 0x1530], xmm14
1404d95b2: 66 44 0f 7f ad 20 15 00 00  	movdqa	xmmword ptr [rbp + 0x1520], xmm13
1404d95bb: 44 0f 29 a5 10 15 00 00     	movaps	xmmword ptr [rbp + 0x1510], xmm12
1404d95c3: 44 0f 29 9d 00 15 00 00     	movaps	xmmword ptr [rbp + 0x1500], xmm11
1404d95cb: 66 44 0f 29 95 f0 14 00 00  	movapd	xmmword ptr [rbp + 0x14f0], xmm10
1404d95d4: 66 44 0f 7f 8d e0 14 00 00  	movdqa	xmmword ptr [rbp + 0x14e0], xmm9
1404d95dd: 44 0f 29 85 d0 14 00 00     	movaps	xmmword ptr [rbp + 0x14d0], xmm8
1404d95e5: 66 0f 7f bd c0 14 00 00     	movdqa	xmmword ptr [rbp + 0x14c0], xmm7
1404d95ed: 66 0f 29 b5 b0 14 00 00     	movapd	xmmword ptr [rbp + 0x14b0], xmm6
1404d95f5: 48 c7 85 a8 14 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x14a8], -0x2
1404d9600: 48 c7 85 b0 13 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x13b0], 0x0
1404d960b: 48 c7 85 b8 13 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x13b8], 0x8
1404d9616: 48 c7 85 c0 13 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x13c0], 0x0
1404d9621: 48 89 8d 78 14 00 00        	mov	qword ptr [rbp + 0x1478], rcx
1404d9628: e8 43 8e 00 00              	call	0x1404e2470 <.text+0x4e1470>
1404d962d: 48 8b 85 78 14 00 00        	mov	rax, qword ptr [rbp + 0x1478]
1404d9634: 48 8d 48 30                 	lea	rcx, [rax + 0x30]
1404d9638: 48 89 8d 50 14 00 00        	mov	qword ptr [rbp + 0x1450], rcx
1404d963f: 48 8d 48 68                 	lea	rcx, [rax + 0x68]
1404d9643: 48 89 8d 88 14 00 00        	mov	qword ptr [rbp + 0x1488], rcx
1404d964a: 48 8d 88 88 0a 00 00        	lea	rcx, [rax + 0xa88]
1404d9651: 48 89 8d 10 14 00 00        	mov	qword ptr [rbp + 0x1410], rcx
1404d9658: 48 8d 48 20                 	lea	rcx, [rax + 0x20]
1404d965c: 48 89 8d 18 14 00 00        	mov	qword ptr [rbp + 0x1418], rcx
1404d9663: 48 8d 88 15 11 00 00        	lea	rcx, [rax + 0x1115]
1404d966a: 48 89 8d a8 13 00 00        	mov	qword ptr [rbp + 0x13a8], rcx
1404d9671: 66 44 0f 28 15 36 ff 50 02  	movapd	xmm10, xmmword ptr [rip + 0x250ff36] # 0x1429e95b0
1404d967a: 45 0f 57 db                 	xorps	xmm11, xmm11
1404d967e: f3 44 0f 10 25 49 71 4c 02  	movss	xmm12, dword ptr [rip + 0x24c7149] # 0x1429a07d0
1404d9687: 66 44 0f 6f 2d 60 c2 44 02  	movdqa	xmm13, xmmword ptr [rip + 0x244c260] # 0x1429258f0
1404d9690: 66 44 0f 6f 0d d7 7e 48 02  	movdqa	xmm9, xmmword ptr [rip + 0x2487ed7] # 0x142961570
1404d9699: 48 8d 88 c8 10 00 00        	lea	rcx, [rax + 0x10c8]
1404d96a0: 48 89 8d 20 14 00 00        	mov	qword ptr [rbp + 0x1420], rcx
1404d96a7: 48 8d 88 c0 00 00 00        	lea	rcx, [rax + 0xc0]
1404d96ae: 48 89 8d d8 13 00 00        	mov	qword ptr [rbp + 0x13d8], rcx
1404d96b5: 48 8d 88 a0 06 00 00        	lea	rcx, [rax + 0x6a0]
1404d96bc: 48 89 8d 50 13 00 00        	mov	qword ptr [rbp + 0x1350], rcx
1404d96c3: 48 8d 88 90 08 00 00        	lea	rcx, [rax + 0x890]
1404d96ca: 48 89 8d c8 13 00 00        	mov	qword ptr [rbp + 0x13c8], rcx
1404d96d1: 48 8d 88 e0 08 00 00        	lea	rcx, [rax + 0x8e0]
1404d96d8: 48 89 8d 70 13 00 00        	mov	qword ptr [rbp + 0x1370], rcx
1404d96df: 48 8d 88 80 09 00 00        	lea	rcx, [rax + 0x980]
1404d96e6: 48 89 8d d0 13 00 00        	mov	qword ptr [rbp + 0x13d0], rcx
1404d96ed: 48 8d 88 d0 09 00 00        	lea	rcx, [rax + 0x9d0]
1404d96f4: 48 89 8d 78 13 00 00        	mov	qword ptr [rbp + 0x1378], rcx
1404d96fb: 48 8d 88 e0 09 00 00        	lea	rcx, [rax + 0x9e0]
1404d9702: 48 89 8d 90 13 00 00        	mov	qword ptr [rbp + 0x1390], rcx
1404d9709: 48 8d 88 f0 10 00 00        	lea	rcx, [rax + 0x10f0]
1404d9710: 48 89 8d 88 13 00 00        	mov	qword ptr [rbp + 0x1388], rcx
1404d9717: 48 8d 88 30 07 00 00        	lea	rcx, [rax + 0x730]
1404d971e: 48 89 8d 30 13 00 00        	mov	qword ptr [rbp + 0x1330], rcx
1404d9725: 48 8d 88 88 06 00 00        	lea	rcx, [rax + 0x688]
1404d972c: 48 89 8d 38 13 00 00        	mov	qword ptr [rbp + 0x1338], rcx
1404d9733: 48 8d 88 b0 10 00 00        	lea	rcx, [rax + 0x10b0]
1404d973a: 48 89 8d 40 13 00 00        	mov	qword ptr [rbp + 0x1340], rcx
1404d9741: 48 8d 88 90 0a 00 00        	lea	rcx, [rax + 0xa90]
1404d9748: 48 89 8d 48 13 00 00        	mov	qword ptr [rbp + 0x1348], rcx
1404d974f: 48 8d 88 b0 08 00 00        	lea	rcx, [rax + 0x8b0]
1404d9756: 48 89 8d 58 13 00 00        	mov	qword ptr [rbp + 0x1358], rcx
1404d975d: 48 8d 88 c0 08 00 00        	lea	rcx, [rax + 0x8c0]
1404d9764: 48 89 8d 98 13 00 00        	mov	qword ptr [rbp + 0x1398], rcx
1404d976b: 48 8d 88 a0 09 00 00        	lea	rcx, [rax + 0x9a0]
1404d9772: 48 89 8d 60 13 00 00        	mov	qword ptr [rbp + 0x1360], rcx
1404d9779: 48 8d 88 b0 09 00 00        	lea	rcx, [rax + 0x9b0]
1404d9780: 48 89 8d a0 13 00 00        	mov	qword ptr [rbp + 0x13a0], rcx
1404d9787: 48 05 40 0a 00 00           	add	rax, 0xa40
1404d978d: 48 89 85 68 13 00 00        	mov	qword ptr [rbp + 0x1368], rax
1404d9794: 48 c7 85 80 13 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x1380], 0x0
1404d979f: 66 0f 76 ff                 	pcmpeqd	xmm7, xmm7
1404d97a3: eb 27                       	jmp	0x1404d97cc <.text+0x4d87cc>
1404d97a5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
1404d97b0: 41 b8 c8 00 00 00           	mov	r8d, 0xc8
1404d97b6: 48 8b 8d 98 14 00 00        	mov	rcx, qword ptr [rbp + 0x1498]
1404d97bd: 48 89 f2                    	mov	rdx, rsi
1404d97c0: e8 3b a4 44 02              	call	0x142923c00 <.text+0x2922c00>
1404d97c5: 48 ff 85 80 13 00 00        	inc	qword ptr [rbp + 0x1380]
1404d97cc: 48 8b 8d 88 14 00 00        	mov	rcx, qword ptr [rbp + 0x1488]
1404d97d3: 48 ba ba a5 7e fa dc 08 df 98       	movabs	rdx, -0x6720f72305815a46
1404d97dd: 49 b8 c3 2b 63 0e a6 17 b1 67       	movabs	r8, 0x67b117a60e632bc3
1404d97e7: e8 04 3f c0 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1404d97ec: 48 85 c0                    	test	rax, rax
1404d97ef: 4c 8b 85 78 14 00 00        	mov	r8, qword ptr [rbp + 0x1478]
1404d97f6: 74 58                       	je	0x1404d9850 <.text+0x4d8850>
1404d97f8: 49 39 90 48 02 00 00        	cmp	qword ptr [r8 + 0x248], rdx
1404d97ff: 76 4f                       	jbe	0x1404d9850 <.text+0x4d8850>
1404d9801: 49 8b 80 40 02 00 00        	mov	rax, qword ptr [r8 + 0x240]
1404d9808: 48 8b 0c d0                 	mov	rcx, qword ptr [rax + 8*rdx]
1404d980c: 48 85 c9                    	test	rcx, rcx
1404d980f: 74 3f                       	je	0x1404d9850 <.text+0x4d8850>
1404d9811: 49 8b 80 10 02 00 00        	mov	rax, qword ptr [r8 + 0x210]
1404d9818: 48 f7 d1                    	not	rcx
1404d981b: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
1404d981f: 48 c1 e1 05                 	shl	rcx, 0x5
1404d9823: 48 83 7c 08 20 00           	cmp	qword ptr [rax + rcx + 0x20], 0x0
1404d9829: 74 25                       	je	0x1404d9850 <.text+0x4d8850>
1404d982b: 48 01 c8                    	add	rax, rcx
1404d982e: 48 8b 48 10                 	mov	rcx, qword ptr [rax + 0x10]
1404d9832: e8 d9 04 64 00              	call	0x140b19d10 <.text+0xb18d10>
1404d9837: 84 c0                       	test	al, al
1404d9839: 4c 8b 85 78 14 00 00        	mov	r8, qword ptr [rbp + 0x1478]
1404d9840: 0f 84 1d 02 00 00           	je	0x1404d9a63 <.text+0x4d8a63>
1404d9846: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
1404d9850: 49 8b 00                    	mov	rax, qword ptr [r8]
1404d9853: 49 8b 50 08                 	mov	rdx, qword ptr [r8 + 0x8]
1404d9857: 48 85 c0                    	test	rax, rax
1404d985a: 74 14                       	je	0x1404d9870 <.text+0x4d8870>
1404d985c: 83 f8 01                    	cmp	eax, 0x1
1404d985f: 75 1f                       	jne	0x1404d9880 <.text+0x4d8880>
1404d9861: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
1404d9865: e8 f6 dd d8 ff              	call	0x140267660 <.text+0x266660>
1404d986a: eb 1d                       	jmp	0x1404d9889 <.text+0x4d8889>
1404d986c: 0f 1f 40 00                 	nop	dword ptr [rax]
1404d9870: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
1404d9874: e8 17 88 01 00              	call	0x1404f2090 <.text+0x4f1090>
1404d9879: eb 0e                       	jmp	0x1404d9889 <.text+0x4d8889>
1404d987b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
1404d9880: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
1404d9884: e8 27 6d 04 00              	call	0x1405205b0 <.text+0x51f5b0>
1404d9889: 31 c0                       	xor	eax, eax
1404d988b: 48 3b 45 c0                 	cmp	rax, qword ptr [rbp - 0x40]
1404d988f: 0f 80 ce 01 00 00           	jo	0x1404d9a63 <.text+0x4d8a63>
1404d9895: 41 b8 28 06 00 00           	mov	r8d, 0x628
1404d989b: 48 8d 8d 00 06 00 00        	lea	rcx, [rbp + 0x600]
1404d98a2: 48 8d 55 c0                 	lea	rdx, [rbp - 0x40]
1404d98a6: e8 55 a3 44 02              	call	0x142923c00 <.text+0x2922c00>
1404d98ab: 48 8b 05 f6 2a 15 03        	mov	rax, qword ptr [rip + 0x3152af6] # 0x14362c3a8
1404d98b2: 48 8b 00                    	mov	rax, qword ptr [rax]
1404d98b5: 48 83 f8 03                 	cmp	rax, 0x3
1404d98b9: 48 8b b5 78 14 00 00        	mov	rsi, qword ptr [rbp + 0x1478]
1404d98c0: 48 8b bd 10 14 00 00        	mov	rdi, qword ptr [rbp + 0x1410]
1404d98c7: 48 8d 9d 30 0c 00 00        	lea	rbx, [rbp + 0xc30]
1404d98ce: 0f 82 c7 00 00 00           	jb	0x1404d999b <.text+0x4d899b>
1404d98d4: c6 85 97 14 00 00 01        	mov	byte ptr [rbp + 0x1497], 0x1
1404d98db: 48 8d 0d 5e 04 51 02        	lea	rcx, [rip + 0x251045e]  # 0x1429e9d40
1404d98e2: e8 99 44 1a 00              	call	0x14067dd80 <.text+0x67cd80>
1404d98e7: 66 0f 10 00                 	movupd	xmm0, xmmword ptr [rax]
1404d98eb: 8b 40 10                    	mov	eax, dword ptr [rax + 0x10]
1404d98ee: 48 c7 85 60 0c 00 00 03 00 00 00    	mov	qword ptr [rbp + 0xc60], 0x3
1404d98f9: 48 8d 15 e2 03 51 02        	lea	rdx, [rip + 0x25103e2]  # 0x1429e9ce2
1404d9900: 48 89 95 68 0c 00 00        	mov	qword ptr [rbp + 0xc68], rdx
1404d9907: 48 c7 85 70 0c 00 00 15 00 00 00    	mov	qword ptr [rbp + 0xc70], 0x15
1404d9912: 48 8d 0d 17 04 51 02        	lea	rcx, [rip + 0x2510417]  # 0x1429e9d30
1404d9919: 48 89 8d 80 0c 00 00        	mov	qword ptr [rbp + 0xc80], rcx
1404d9920: 48 c7 85 88 0c 00 00 01 00 00 00    	mov	qword ptr [rbp + 0xc88], 0x1
1404d992b: 48 c7 85 90 0c 00 00 08 00 00 00    	mov	qword ptr [rbp + 0xc90], 0x8
1404d9936: 48 c7 85 30 0c 00 00 00 00 00 00    	mov	qword ptr [rbp + 0xc30], 0x0
1404d9941: 48 8d 8d 98 0c 00 00        	lea	rcx, [rbp + 0xc98]
1404d9948: 66 0f ef c9                 	pxor	xmm1, xmm1
1404d994c: f3 0f 7f 09                 	movdqu	xmmword ptr [rcx], xmm1
1404d9950: 48 89 95 38 0c 00 00        	mov	qword ptr [rbp + 0xc38], rdx
1404d9957: 48 c7 85 40 0c 00 00 15 00 00 00    	mov	qword ptr [rbp + 0xc40], 0x15
1404d9962: 48 c7 85 48 0c 00 00 00 00 00 00    	mov	qword ptr [rbp + 0xc48], 0x0
1404d996d: 66 0f 11 85 50 0c 00 00     	movupd	xmmword ptr [rbp + 0xc50], xmm0
1404d9975: c7 85 78 0c 00 00 01 00 00 00       	mov	dword ptr [rbp + 0xc78], 0x1
1404d997f: 89 85 7c 0c 00 00           	mov	dword ptr [rbp + 0xc7c], eax
1404d9985: c6 85 97 14 00 00 01        	mov	byte ptr [rbp + 0x1497], 0x1
1404d998c: 48 8d 8d 77 14 00 00        	lea	rcx, [rbp + 0x1477]
1404d9993: 48 89 da                    	mov	rdx, rbx
1404d9996: e8 95 09 87 00              	call	0x140d4a330 <.text+0xd49330>
1404d999b: 41 b8 28 06 00 00           	mov	r8d, 0x628
1404d99a1: 48 89 d9                    	mov	rcx, rbx
1404d99a4: 48 8d 95 00 06 00 00        	lea	rdx, [rbp + 0x600]
1404d99ab: e8 50 a2 44 02              	call	0x142923c00 <.text+0x2922c00>
1404d99b0: 31 c0                       	xor	eax, eax
1404d99b2: 48 3b 07                    	cmp	rax, qword ptr [rdi]
1404d99b5: 70 0c                       	jo	0x1404d99c3 <.text+0x4d89c3>
1404d99b7: 48 8b 8d 10 14 00 00        	mov	rcx, qword ptr [rbp + 0x1410]
1404d99be: e8 9d f1 10 00              	call	0x1405e8b60 <.text+0x5e7b60>
1404d99c3: 41 b8 28 06 00 00           	mov	r8d, 0x628
1404d99c9: 48 8b 8d 10 14 00 00        	mov	rcx, qword ptr [rbp + 0x1410]
1404d99d0: 48 89 da                    	mov	rdx, rbx
1404d99d3: e8 28 a2 44 02              	call	0x142923c00 <.text+0x2922c00>
1404d99d8: c6 85 97 14 00 00 00        	mov	byte ptr [rbp + 0x1497], 0x0
1404d99df: 48 8b 8d 88 14 00 00        	mov	rcx, qword ptr [rbp + 0x1488]
1404d99e6: 48 ba ba a5 7e fa dc 08 df 98       	movabs	rdx, -0x6720f72305815a46
1404d99f0: 49 b8 c3 2b 63 0e a6 17 b1 67       	movabs	r8, 0x67b117a60e632bc3
1404d99fa: e8 f1 3c c0 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1404d99ff: 48 85 c0                    	test	rax, rax
1404d9a02: 0f 84 b8 59 00 00           	je	0x1404df3c0 <.text+0x4de3c0>
1404d9a08: 48 39 96 48 02 00 00        	cmp	qword ptr [rsi + 0x248], rdx
1404d9a0f: 0f 86 ab 59 00 00           	jbe	0x1404df3c0 <.text+0x4de3c0>
1404d9a15: 48 8b 86 40 02 00 00        	mov	rax, qword ptr [rsi + 0x240]
1404d9a1c: 48 8b 0c d0                 	mov	rcx, qword ptr [rax + 8*rdx]
1404d9a20: 48 85 c9                    	test	rcx, rcx
1404d9a23: 0f 84 97 59 00 00           	je	0x1404df3c0 <.text+0x4de3c0>
1404d9a29: 48 8b 86 10 02 00 00        	mov	rax, qword ptr [rsi + 0x210]
1404d9a30: 48 f7 d1                    	not	rcx
1404d9a33: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
1404d9a37: 48 c1 e1 05                 	shl	rcx, 0x5
1404d9a3b: 48 83 7c 08 20 00           	cmp	qword ptr [rax + rcx + 0x20], 0x0
1404d9a41: 0f 84 79 59 00 00           	je	0x1404df3c0 <.text+0x4de3c0>
1404d9a47: 48 01 c8                    	add	rax, rcx
1404d9a4a: 48 8b 48 10                 	mov	rcx, qword ptr [rax + 0x10]
1404d9a4e: 8b 96 e0 05 00 00           	mov	edx, dword ptr [rsi + 0x5e0]
1404d9a54: 89 50 5c                    	mov	dword ptr [rax + 0x5c], edx
1404d9a57: c6 85 97 14 00 00 00        	mov	byte ptr [rbp + 0x1497], 0x0
1404d9a5e: e8 cd 02 64 00              	call	0x140b19d30 <.text+0xb18d30>
1404d9a63: 48 8b 8d 88 14 00 00        	mov	rcx, qword ptr [rbp + 0x1488]
1404d9a6a: 48 ba ba a5 7e fa dc 08 df 98       	movabs	rdx, -0x6720f72305815a46
1404d9a74: 49 b8 c3 2b 63 0e a6 17 b1 67       	movabs	r8, 0x67b117a60e632bc3
1404d9a7e: e8 6d 3c c0 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1404d9a83: 48 85 c0                    	test	rax, rax
1404d9a86: 4c 8b b5 78 14 00 00        	mov	r14, qword ptr [rbp + 0x1478]
1404d9a8d: 4c 8b 85 10 14 00 00        	mov	r8, qword ptr [rbp + 0x1410]
1404d9a94: 4c 8d a5 30 0c 00 00        	lea	r12, [rbp + 0xc30]
1404d9a9b: 74 63                       	je	0x1404d9b00 <.text+0x4d8b00>
1404d9a9d: 49 39 96 48 02 00 00        	cmp	qword ptr [r14 + 0x248], rdx
1404d9aa4: 76 5a                       	jbe	0x1404d9b00 <.text+0x4d8b00>
1404d9aa6: 49 8b 86 40 02 00 00        	mov	rax, qword ptr [r14 + 0x240]
1404d9aad: 48 8b 0c d0                 	mov	rcx, qword ptr [rax + 8*rdx]
1404d9ab1: 48 85 c9                    	test	rcx, rcx
1404d9ab4: 74 4a                       	je	0x1404d9b00 <.text+0x4d8b00>
1404d9ab6: 49 8b 86 10 02 00 00        	mov	rax, qword ptr [r14 + 0x210]
1404d9abd: 48 f7 d1                    	not	rcx
1404d9ac0: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
1404d9ac4: 48 c1 e1 05                 	shl	rcx, 0x5
1404d9ac8: 48 83 7c 08 20 00           	cmp	qword ptr [rax + rcx + 0x20], 0x0
1404d9ace: 74 30                       	je	0x1404d9b00 <.text+0x4d8b00>
1404d9ad0: 48 01 c8                    	add	rax, rcx
1404d9ad3: 48 8b 48 10                 	mov	rcx, qword ptr [rax + 0x10]
1404d9ad7: e8 44 02 64 00              	call	0x140b19d20 <.text+0xb18d20>
1404d9adc: 84 c0                       	test	al, al
1404d9ade: 4c 8b b5 78 14 00 00        	mov	r14, qword ptr [rbp + 0x1478]
1404d9ae5: 4c 8b 85 10 14 00 00        	mov	r8, qword ptr [rbp + 0x1410]
1404d9aec: 0f 84 8f 08 00 00           	je	0x1404da381 <.text+0x4d9381>
1404d9af2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nop	word ptr cs:[rax + rax]
1404d9b00: 49 8b 00                    	mov	rax, qword ptr [r8]
1404d9b03: 48 b9 00 00 00 00 00 00 00 80       	movabs	rcx, -0x8000000000000000
1404d9b0d: 49 89 08                    	mov	qword ptr [r8], rcx
1404d9b10: 48 89 c1                    	mov	rcx, rax
1404d9b13: 48 f7 d9                    	neg	rcx
1404d9b16: 0f 80 65 08 00 00           	jo	0x1404da381 <.text+0x4d9381>
1404d9b1c: 48 89 85 00 06 00 00        	mov	qword ptr [rbp + 0x600], rax
1404d9b23: 41 b8 20 06 00 00           	mov	r8d, 0x620
1404d9b29: 48 8d 8d 08 06 00 00        	lea	rcx, [rbp + 0x608]
1404d9b30: 48 8b 95 48 13 00 00        	mov	rdx, qword ptr [rbp + 0x1348]
1404d9b37: e8 c4 a0 44 02              	call	0x142923c00 <.text+0x2922c00>
1404d9b3c: 48 8b 05 65 28 15 03        	mov	rax, qword ptr [rip + 0x3152865] # 0x14362c3a8
1404d9b43: 48 8b 00                    	mov	rax, qword ptr [rax]
1404d9b46: 48 83 f8 03                 	cmp	rax, 0x3
1404d9b4a: 4c 8b bd 50 14 00 00        	mov	r15, qword ptr [rbp + 0x1450]
1404d9b51: 0f 82 c7 00 00 00           	jb	0x1404d9c1e <.text+0x4d8c1e>
1404d9b57: c6 85 a4 14 00 00 01        	mov	byte ptr [rbp + 0x14a4], 0x1
1404d9b5e: 48 8d 0d 4b 02 51 02        	lea	rcx, [rip + 0x251024b]  # 0x1429e9db0
1404d9b65: e8 16 42 1a 00              	call	0x14067dd80 <.text+0x67cd80>
1404d9b6a: 66 0f 10 00                 	movupd	xmm0, xmmword ptr [rax]
1404d9b6e: 8b 40 10                    	mov	eax, dword ptr [rax + 0x10]
1404d9b71: 48 c7 85 60 0c 00 00 03 00 00 00    	mov	qword ptr [rbp + 0xc60], 0x3
1404d9b7c: 48 8d 15 5f 01 51 02        	lea	rdx, [rip + 0x251015f]  # 0x1429e9ce2
1404d9b83: 48 89 95 68 0c 00 00        	mov	qword ptr [rbp + 0xc68], rdx
1404d9b8a: 48 c7 85 70 0c 00 00 15 00 00 00    	mov	qword ptr [rbp + 0xc70], 0x15
1404d9b95: 48 8d 0d 04 02 51 02        	lea	rcx, [rip + 0x2510204]  # 0x1429e9da0
1404d9b9c: 48 89 8d 80 0c 00 00        	mov	qword ptr [rbp + 0xc80], rcx
1404d9ba3: 48 c7 85 88 0c 00 00 01 00 00 00    	mov	qword ptr [rbp + 0xc88], 0x1
1404d9bae: 48 c7 85 90 0c 00 00 08 00 00 00    	mov	qword ptr [rbp + 0xc90], 0x8
1404d9bb9: 48 c7 85 30 0c 00 00 00 00 00 00    	mov	qword ptr [rbp + 0xc30], 0x0
1404d9bc4: 48 8d 8d 98 0c 00 00        	lea	rcx, [rbp + 0xc98]
1404d9bcb: 66 0f ef c9                 	pxor	xmm1, xmm1
1404d9bcf: f3 0f 7f 09                 	movdqu	xmmword ptr [rcx], xmm1
1404d9bd3: 48 89 95 38 0c 00 00        	mov	qword ptr [rbp + 0xc38], rdx
1404d9bda: 48 c7 85 40 0c 00 00 15 00 00 00    	mov	qword ptr [rbp + 0xc40], 0x15
1404d9be5: 48 c7 85 48 0c 00 00 00 00 00 00    	mov	qword ptr [rbp + 0xc48], 0x0
1404d9bf0: 66 0f 11 85 50 0c 00 00     	movupd	xmmword ptr [rbp + 0xc50], xmm0
1404d9bf8: c7 85 78 0c 00 00 01 00 00 00       	mov	dword ptr [rbp + 0xc78], 0x1
1404d9c02: 89 85 7c 0c 00 00           	mov	dword ptr [rbp + 0xc7c], eax
1404d9c08: c6 85 a4 14 00 00 01        	mov	byte ptr [rbp + 0x14a4], 0x1
1404d9c0f: 48 8d 8d 77 14 00 00        	lea	rcx, [rbp + 0x1477]
1404d9c16: 4c 89 e2                    	mov	rdx, r12
1404d9c19: e8 12 07 87 00              	call	0x140d4a330 <.text+0xd49330>
1404d9c1e: c6 85 a4 14 00 00 01        	mov	byte ptr [rbp + 0x14a4], 0x1
1404d9c25: 48 8b 8d 88 14 00 00        	mov	rcx, qword ptr [rbp + 0x1488]
1404d9c2c: 48 ba 40 7c d4 87 e7 56 7b 3d       	movabs	rdx, 0x3d7b56e787d47c40
1404d9c36: 49 b8 c7 bf 43 8d 0a b1 b0 67       	movabs	r8, 0x67b0b10a8d43bfc7
1404d9c40: e8 ab 3a c0 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1404d9c45: 48 85 c0                    	test	rax, rax
1404d9c48: 0f 84 b8 55 00 00           	je	0x1404df206 <.text+0x4de206>
1404d9c4e: 49 39 96 48 02 00 00        	cmp	qword ptr [r14 + 0x248], rdx
1404d9c55: 0f 86 ab 55 00 00           	jbe	0x1404df206 <.text+0x4de206>
1404d9c5b: 49 8b 86 40 02 00 00        	mov	rax, qword ptr [r14 + 0x240]
1404d9c62: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1404d9c66: 48 85 c0                    	test	rax, rax
1404d9c69: 0f 84 97 55 00 00           	je	0x1404df206 <.text+0x4de206>
1404d9c6f: 49 8b 8e 10 02 00 00        	mov	rcx, qword ptr [r14 + 0x210]
1404d9c76: 48 f7 d0                    	not	rax
1404d9c79: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1404d9c7d: 48 c1 e0 05                 	shl	rax, 0x5
1404d9c81: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
1404d9c87: 0f 84 79 55 00 00           	je	0x1404df206 <.text+0x4de206>
1404d9c8d: 48 01 c1                    	add	rcx, rax
1404d9c90: c6 85 a4 14 00 00 01        	mov	byte ptr [rbp + 0x14a4], 0x1
1404d9c97: 31 d2                       	xor	edx, edx
1404d9c99: e8 72 5a 82 00              	call	0x140cff710 <.text+0xcfe710>
1404d9c9e: 48 8b 08                    	mov	rcx, qword ptr [rax]
1404d9ca1: 48 89 ca                    	mov	rdx, rcx
1404d9ca4: 48 f7 da                    	neg	rdx
1404d9ca7: 0f 80 59 55 00 00           	jo	0x1404df206 <.text+0x4de206>
1404d9cad: 48 83 c0 08                 	add	rax, 0x8
1404d9cb1: 48 89 8d 30 0c 00 00        	mov	qword ptr [rbp + 0xc30], rcx
1404d9cb8: 41 b8 20 01 00 00           	mov	r8d, 0x120
1404d9cbe: 48 8d 8d 38 0c 00 00        	lea	rcx, [rbp + 0xc38]
1404d9cc5: 48 89 c2                    	mov	rdx, rax
1404d9cc8: e8 33 9f 44 02              	call	0x142923c00 <.text+0x2922c00>
1404d9ccd: c6 85 a4 14 00 00 01        	mov	byte ptr [rbp + 0x14a4], 0x1
1404d9cd4: 48 8d 8d 00 06 00 00        	lea	rcx, [rbp + 0x600]
1404d9cdb: 4c 89 e2                    	mov	rdx, r12
1404d9cde: e8 9d f0 e4 ff              	call	0x140328d80 <.text+0x327d80>
1404d9ce3: c6 85 a4 14 00 00 01        	mov	byte ptr [rbp + 0x14a4], 0x1
1404d9cea: 48 8b 8d 88 14 00 00        	mov	rcx, qword ptr [rbp + 0x1488]
1404d9cf1: 48 ba 8a bf e6 7d 0e 98 39 2b       	movabs	rdx, 0x2b39980e7de6bf8a
1404d9cfb: 49 b8 c5 56 0d d2 bf aa d9 52       	movabs	r8, 0x52d9aabfd20d56c5
1404d9d05: e8 e6 39 c0 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1404d9d0a: 48 85 c0                    	test	rax, rax
1404d9d0d: 0f 84 db 54 00 00           	je	0x1404df1ee <.text+0x4de1ee>
1404d9d13: 49 39 96 48 02 00 00        	cmp	qword ptr [r14 + 0x248], rdx
1404d9d1a: 0f 86 ce 54 00 00           	jbe	0x1404df1ee <.text+0x4de1ee>
1404d9d20: 49 8b 86 40 02 00 00        	mov	rax, qword ptr [r14 + 0x240]
1404d9d27: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1404d9d2b: 48 85 c0                    	test	rax, rax
1404d9d2e: 0f 84 ba 54 00 00           	je	0x1404df1ee <.text+0x4de1ee>
1404d9d34: 49 8b 8e 10 02 00 00        	mov	rcx, qword ptr [r14 + 0x210]
1404d9d3b: 48 f7 d0                    	not	rax
1404d9d3e: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1404d9d42: 48 c1 e0 05                 	shl	rax, 0x5
1404d9d46: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
1404d9d4c: 0f 84 9c 54 00 00           	je	0x1404df1ee <.text+0x4de1ee>
1404d9d52: 48 01 c1                    	add	rcx, rax
1404d9d55: c6 85 a4 14 00 00 01        	mov	byte ptr [rbp + 0x14a4], 0x1
1404d9d5c: 31 d2                       	xor	edx, edx
1404d9d5e: e8 ad 59 82 00              	call	0x140cff710 <.text+0xcfe710>
1404d9d63: 8b 48 18                    	mov	ecx, dword ptr [rax + 0x18]
1404d9d66: 81 f9 00 ca 9a 3b           	cmp	ecx, 0x3b9aca00
1404d9d6c: 0f 84 7c 54 00 00           	je	0x1404df1ee <.text+0x4de1ee>
1404d9d72: 8b 50 1c                    	mov	edx, dword ptr [rax + 0x1c]
1404d9d75: 4c 8b 40 10                 	mov	r8, qword ptr [rax + 0x10]
1404d9d79: 4c 89 85 40 0c 00 00        	mov	qword ptr [rbp + 0xc40], r8
1404d9d80: 66 0f 10 00                 	movupd	xmm0, xmmword ptr [rax]
1404d9d84: 66 0f 29 85 30 0c 00 00     	movapd	xmmword ptr [rbp + 0xc30], xmm0
1404d9d8c: 89 8d 48 0c 00 00           	mov	dword ptr [rbp + 0xc48], ecx
1404d9d92: 89 95 4c 0c 00 00           	mov	dword ptr [rbp + 0xc4c], edx
1404d9d98: c6 85 a4 14 00 00 01        	mov	byte ptr [rbp + 0x14a4], 0x1
1404d9d9f: 48 8d 8d 00 06 00 00        	lea	rcx, [rbp + 0x600]
1404d9da6: 4c 89 e2                    	mov	rdx, r12
1404d9da9: e8 a2 f5 e4 ff              	call	0x140329350 <.text+0x328350>
1404d9dae: c6 85 a4 14 00 00 01        	mov	byte ptr [rbp + 0x14a4], 0x1
1404d9db5: 48 8b 8d 88 14 00 00        	mov	rcx, qword ptr [rbp + 0x1488]
1404d9dbc: 48 ba a6 12 23 8c a9 dc 4d 86       	movabs	rdx, -0x79b2235673dced5a
1404d9dc6: 49 b8 4e 48 9a 9e 36 e9 b0 27       	movabs	r8, 0x27b0e9369e9a484e
1404d9dd0: e8 1b 39 c0 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1404d9dd5: 48 85 c0                    	test	rax, rax
1404d9dd8: 0f 84 73 56 00 00           	je	0x1404df451 <.text+0x4de451>
1404d9dde: 49 39 96 48 02 00 00        	cmp	qword ptr [r14 + 0x248], rdx
1404d9de5: 0f 86 66 56 00 00           	jbe	0x1404df451 <.text+0x4de451>
1404d9deb: 49 8b 86 40 02 00 00        	mov	rax, qword ptr [r14 + 0x240]
1404d9df2: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1404d9df6: 48 85 c0                    	test	rax, rax
1404d9df9: 0f 84 52 56 00 00           	je	0x1404df451 <.text+0x4de451>
1404d9dff: 49 8b 8e 10 02 00 00        	mov	rcx, qword ptr [r14 + 0x210]
1404d9e06: 48 f7 d0                    	not	rax
1404d9e09: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1404d9e0d: 48 c1 e0 05                 	shl	rax, 0x5
1404d9e11: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
1404d9e17: 0f 84 34 56 00 00           	je	0x1404df451 <.text+0x4de451>
1404d9e1d: 48 01 c1                    	add	rcx, rax
1404d9e20: c6 85 a4 14 00 00 01        	mov	byte ptr [rbp + 0x14a4], 0x1
1404d9e27: 31 d2                       	xor	edx, edx
1404d9e29: e8 e2 58 82 00              	call	0x140cff710 <.text+0xcfe710>
1404d9e2e: 8b 10                       	mov	edx, dword ptr [rax]
1404d9e30: 44 8b 40 04                 	mov	r8d, dword ptr [rax + 0x4]
1404d9e34: c6 85 a4 14 00 00 01        	mov	byte ptr [rbp + 0x14a4], 0x1
1404d9e3b: 48 8d 8d 00 06 00 00        	lea	rcx, [rbp + 0x600]
1404d9e42: e8 29 00 e5 ff              	call	0x140329e70 <.text+0x328e70>
1404d9e47: c6 85 a4 14 00 00 01        	mov	byte ptr [rbp + 0x14a4], 0x1
1404d9e4e: 48 8b 8d 88 14 00 00        	mov	rcx, qword ptr [rbp + 0x1488]
1404d9e55: 48 ba d7 98 6d 9d cb 47 e0 e5       	movabs	rdx, -0x1a1fb83462926729
1404d9e5f: 49 b8 5a 8a 78 0a 5c d3 5f 9c       	movabs	r8, -0x63a02ca3f58775a6
1404d9e69: e8 82 38 c0 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1404d9e6e: 48 85 c0                    	test	rax, rax
1404d9e71: 0f 84 a7 53 00 00           	je	0x1404df21e <.text+0x4de21e>
1404d9e77: 49 39 96 48 02 00 00        	cmp	qword ptr [r14 + 0x248], rdx
1404d9e7e: 0f 86 9a 53 00 00           	jbe	0x1404df21e <.text+0x4de21e>
1404d9e84: 49 8b 86 40 02 00 00        	mov	rax, qword ptr [r14 + 0x240]
1404d9e8b: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1404d9e8f: 48 85 c0                    	test	rax, rax
1404d9e92: 0f 84 86 53 00 00           	je	0x1404df21e <.text+0x4de21e>
1404d9e98: 49 8b 8e 10 02 00 00        	mov	rcx, qword ptr [r14 + 0x210]
1404d9e9f: 48 f7 d0                    	not	rax
1404d9ea2: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1404d9ea6: 48 c1 e0 05                 	shl	rax, 0x5
1404d9eaa: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
1404d9eb0: 0f 84 68 53 00 00           	je	0x1404df21e <.text+0x4de21e>
1404d9eb6: 48 01 c1                    	add	rcx, rax
1404d9eb9: c6 85 a4 14 00 00 01        	mov	byte ptr [rbp + 0x14a4], 0x1
1404d9ec0: 31 d2                       	xor	edx, edx
1404d9ec2: e8 49 58 82 00              	call	0x140cff710 <.text+0xcfe710>
1404d9ec7: 48 8b 08                    	mov	rcx, qword ptr [rax]
1404d9eca: 48 89 ca                    	mov	rdx, rcx
1404d9ecd: 48 f7 da                    	neg	rdx
1404d9ed0: 0f 80 48 53 00 00           	jo	0x1404df21e <.text+0x4de21e>
1404d9ed6: 48 89 8d 30 0c 00 00        	mov	qword ptr [rbp + 0xc30], rcx
1404d9edd: 66 0f 10 40 08              	movupd	xmm0, xmmword ptr [rax + 0x8]
1404d9ee2: 48 8d 85 38 0c 00 00        	lea	rax, [rbp + 0xc38]
1404d9ee9: 66 0f 11 00                 	movupd	xmmword ptr [rax], xmm0
1404d9eed: c6 85 a4 14 00 00 01        	mov	byte ptr [rbp + 0x14a4], 0x1
1404d9ef4: 48 8d 8d 00 06 00 00        	lea	rcx, [rbp + 0x600]
1404d9efb: 4c 89 e2                    	mov	rdx, r12
1404d9efe: e8 ed 0a e5 ff              	call	0x14032a9f0 <.text+0x3299f0>
1404d9f03: c6 85 a4 14 00 00 01        	mov	byte ptr [rbp + 0x14a4], 0x1
1404d9f0a: 48 8b 8d 88 14 00 00        	mov	rcx, qword ptr [rbp + 0x1488]
1404d9f11: 48 ba 8b 04 7b e6 61 26 c0 a4       	movabs	rdx, -0x5b3fd99e1984fb75
1404d9f1b: 49 b8 ec 67 2a aa e6 61 e7 ea       	movabs	r8, -0x15189e1955d59814
1404d9f25: e8 c6 37 c0 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1404d9f2a: 48 85 c0                    	test	rax, rax
1404d9f2d: 0f 84 03 53 00 00           	je	0x1404df236 <.text+0x4de236>
1404d9f33: 49 39 96 48 02 00 00        	cmp	qword ptr [r14 + 0x248], rdx
1404d9f3a: 0f 86 f6 52 00 00           	jbe	0x1404df236 <.text+0x4de236>
1404d9f40: 49 8b 86 40 02 00 00        	mov	rax, qword ptr [r14 + 0x240]
1404d9f47: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1404d9f4b: 48 85 c0                    	test	rax, rax
1404d9f4e: 0f 84 e2 52 00 00           	je	0x1404df236 <.text+0x4de236>
1404d9f54: 49 8b 8e 10 02 00 00        	mov	rcx, qword ptr [r14 + 0x210]
1404d9f5b: 48 f7 d0                    	not	rax
1404d9f5e: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1404d9f62: 48 c1 e0 05                 	shl	rax, 0x5
1404d9f66: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
1404d9f6c: 0f 84 c4 52 00 00           	je	0x1404df236 <.text+0x4de236>
1404d9f72: 48 01 c1                    	add	rcx, rax
1404d9f75: c6 85 a4 14 00 00 01        	mov	byte ptr [rbp + 0x14a4], 0x1
1404d9f7c: 31 d2                       	xor	edx, edx
1404d9f7e: e8 8d 57 82 00              	call	0x140cff710 <.text+0xcfe710>
1404d9f83: 0f b6 48 20                 	movzx	ecx, byte ptr [rax + 0x20]
1404d9f87: 80 f9 02                    	cmp	cl, 0x2
1404d9f8a: 0f 84 a6 52 00 00           	je	0x1404df236 <.text+0x4de236>
1404d9f90: 66 0f 10 00                 	movupd	xmm0, xmmword ptr [rax]
1404d9f94: f3 0f 6f 48 10              	movdqu	xmm1, xmmword ptr [rax + 0x10]
1404d9f99: 66 0f 7f 8d 40 0c 00 00     	movdqa	xmmword ptr [rbp + 0xc40], xmm1
1404d9fa1: 66 0f 29 85 30 0c 00 00     	movapd	xmmword ptr [rbp + 0xc30], xmm0
1404d9fa9: 88 8d 50 0c 00 00           	mov	byte ptr [rbp + 0xc50], cl
1404d9faf: 0f b6 48 23                 	movzx	ecx, byte ptr [rax + 0x23]
1404d9fb3: 48 8d 95 38 0c 00 00        	lea	rdx, [rbp + 0xc38]
1404d9fba: 88 4a 1b                    	mov	byte ptr [rdx + 0x1b], cl
1404d9fbd: 0f b7 40 21                 	movzx	eax, word ptr [rax + 0x21]
1404d9fc1: 66 89 42 19                 	mov	word ptr [rdx + 0x19], ax
1404d9fc5: c6 85 a4 14 00 00 01        	mov	byte ptr [rbp + 0x14a4], 0x1
1404d9fcc: 48 8d 8d 00 06 00 00        	lea	rcx, [rbp + 0x600]
1404d9fd3: 4c 89 e2                    	mov	rdx, r12
1404d9fd6: e8 65 e8 e4 ff              	call	0x140328840 <.text+0x327840>
1404d9fdb: c6 85 a4 14 00 00 01        	mov	byte ptr [rbp + 0x14a4], 0x1
1404d9fe2: 48 8b 8d 88 14 00 00        	mov	rcx, qword ptr [rbp + 0x1488]
1404d9fe9: 48 ba 80 ee bc 5f 6a 38 35 33       	movabs	rdx, 0x3335386a5fbcee80
1404d9ff3: 49 b8 60 ec 65 1a f3 59 2a 8f       	movabs	r8, -0x70d5a60ce59a13a0
1404d9ffd: e8 ee 36 c0 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1404da002: 48 85 c0                    	test	rax, rax
1404da005: 0f 84 43 52 00 00           	je	0x1404df24e <.text+0x4de24e>
1404da00b: 49 39 96 48 02 00 00        	cmp	qword ptr [r14 + 0x248], rdx
1404da012: 0f 86 36 52 00 00           	jbe	0x1404df24e <.text+0x4de24e>
1404da018: 49 8b 86 40 02 00 00        	mov	rax, qword ptr [r14 + 0x240]
1404da01f: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1404da023: 48 85 c0                    	test	rax, rax
1404da026: 0f 84 22 52 00 00           	je	0x1404df24e <.text+0x4de24e>
1404da02c: 49 8b 8e 10 02 00 00        	mov	rcx, qword ptr [r14 + 0x210]
1404da033: 48 f7 d0                    	not	rax
1404da036: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1404da03a: 48 c1 e0 05                 	shl	rax, 0x5
1404da03e: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
1404da044: 0f 84 04 52 00 00           	je	0x1404df24e <.text+0x4de24e>
1404da04a: 48 01 c1                    	add	rcx, rax
1404da04d: c6 85 a4 14 00 00 01        	mov	byte ptr [rbp + 0x14a4], 0x1
1404da054: 31 d2                       	xor	edx, edx
1404da056: e8 b5 56 82 00              	call	0x140cff710 <.text+0xcfe710>
1404da05b: 48 8b 08                    	mov	rcx, qword ptr [rax]
1404da05e: 48 83 f9 04                 	cmp	rcx, 0x4
1404da062: 0f 84 e6 51 00 00           	je	0x1404df24e <.text+0x4de24e>
1404da068: 48 89 8d 30 0c 00 00        	mov	qword ptr [rbp + 0xc30], rcx
1404da06f: 48 8b 48 28                 	mov	rcx, qword ptr [rax + 0x28]
1404da073: 48 8d 95 38 0c 00 00        	lea	rdx, [rbp + 0xc38]
1404da07a: 48 89 4a 20                 	mov	qword ptr [rdx + 0x20], rcx
1404da07e: 66 0f 10 40 08              	movupd	xmm0, xmmword ptr [rax + 0x8]
1404da083: f3 0f 6f 48 18              	movdqu	xmm1, xmmword ptr [rax + 0x18]
1404da088: f3 0f 7f 4a 10              	movdqu	xmmword ptr [rdx + 0x10], xmm1
1404da08d: 66 0f 11 02                 	movupd	xmmword ptr [rdx], xmm0
1404da091: c6 85 a4 14 00 00 01        	mov	byte ptr [rbp + 0x14a4], 0x1
1404da098: 48 8d 8d 00 06 00 00        	lea	rcx, [rbp + 0x600]
1404da09f: 4c 89 e2                    	mov	rdx, r12
1404da0a2: e8 49 e2 e4 ff              	call	0x1403282f0 <.text+0x3272f0>
1404da0a7: c6 85 a4 14 00 00 01        	mov	byte ptr [rbp + 0x14a4], 0x1
1404da0ae: 48 8d 8d 38 06 00 00        	lea	rcx, [rbp + 0x638]
1404da0b5: 48 ba 97 e7 5e ca 10 f7 ee 76       	movabs	rdx, 0x76eef710ca5ee797
1404da0bf: 49 b8 13 96 f3 b0 69 51 4a 80       	movabs	r8, -0x7fb5ae964f0c69ed
1404da0c9: e8 22 36 c0 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1404da0ce: 48 85 c0                    	test	rax, rax
1404da0d1: 0f 84 92 53 00 00           	je	0x1404df469 <.text+0x4de469>
1404da0d7: 48 39 95 18 08 00 00        	cmp	qword ptr [rbp + 0x818], rdx
1404da0de: 0f 86 85 53 00 00           	jbe	0x1404df469 <.text+0x4de469>
1404da0e4: 48 8b 85 10 08 00 00        	mov	rax, qword ptr [rbp + 0x810]
1404da0eb: 48 8b 0c d0                 	mov	rcx, qword ptr [rax + 8*rdx]
1404da0ef: 48 85 c9                    	test	rcx, rcx
1404da0f2: 0f 84 71 53 00 00           	je	0x1404df469 <.text+0x4de469>
1404da0f8: 48 8b 85 e0 07 00 00        	mov	rax, qword ptr [rbp + 0x7e0]
1404da0ff: 48 f7 d1                    	not	rcx
1404da102: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
1404da106: 48 c1 e1 05                 	shl	rcx, 0x5
1404da10a: 48 83 7c 08 20 00           	cmp	qword ptr [rax + rcx + 0x20], 0x0
1404da110: 0f 84 53 53 00 00           	je	0x1404df469 <.text+0x4de469>
1404da116: 48 01 c8                    	add	rax, rcx
1404da119: 48 8b 70 10                 	mov	rsi, qword ptr [rax + 0x10]
1404da11d: 8b 8d b0 0b 00 00           	mov	ecx, dword ptr [rbp + 0xbb0]
1404da123: 89 48 5c                    	mov	dword ptr [rax + 0x5c], ecx
1404da126: 48 8b 7e 30                 	mov	rdi, qword ptr [rsi + 0x30]
1404da12a: 48 8b 5e 40                 	mov	rbx, qword ptr [rsi + 0x40]
1404da12e: 48 3b 7e 20                 	cmp	rdi, qword ptr [rsi + 0x20]
1404da132: 75 17                       	jne	0x1404da14b <.text+0x4d914b>
1404da134: 48 8d 4e 20                 	lea	rcx, [rsi + 0x20]
1404da138: c6 85 a4 14 00 00 01        	mov	byte ptr [rbp + 0x14a4], 0x1
1404da13f: 48 8d 15 e2 8f 51 02        	lea	rdx, [rip + 0x2518fe2]  # 0x1429f3128
1404da146: e8 25 87 08 00              	call	0x140562870 <.text+0x561870>
1404da14b: 48 8b 46 28                 	mov	rax, qword ptr [rsi + 0x28]
1404da14f: 48 89 1c f8                 	mov	qword ptr [rax + 8*rdi], rbx
1404da153: 48 ff c7                    	inc	rdi
1404da156: 48 89 7e 30                 	mov	qword ptr [rsi + 0x30], rdi
1404da15a: 48 ff 46 40                 	inc	qword ptr [rsi + 0x40]
1404da15e: c6 85 a4 14 00 00 01        	mov	byte ptr [rbp + 0x14a4], 0x1
1404da165: 48 8b 8d 88 14 00 00        	mov	rcx, qword ptr [rbp + 0x1488]
1404da16c: 48 ba 3f eb bf 68 5e 15 84 a2       	movabs	rdx, -0x5d7beaa1974014c1
1404da176: 49 b8 3a 4a c6 1b 44 b3 a2 84       	movabs	r8, -0x7b5d4cbbe439b5c6
1404da180: e8 6b 35 c0 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1404da185: 48 85 c0                    	test	rax, rax
1404da188: 0f 84 d8 50 00 00           	je	0x1404df266 <.text+0x4de266>
1404da18e: 49 39 96 48 02 00 00        	cmp	qword ptr [r14 + 0x248], rdx
1404da195: 0f 86 cb 50 00 00           	jbe	0x1404df266 <.text+0x4de266>
1404da19b: 49 8b 86 40 02 00 00        	mov	rax, qword ptr [r14 + 0x240]
1404da1a2: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1404da1a6: 48 85 c0                    	test	rax, rax
1404da1a9: 0f 84 b7 50 00 00           	je	0x1404df266 <.text+0x4de266>
1404da1af: 49 8b 8e 10 02 00 00        	mov	rcx, qword ptr [r14 + 0x210]
1404da1b6: 48 f7 d0                    	not	rax
1404da1b9: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1404da1bd: 48 c1 e0 05                 	shl	rax, 0x5
1404da1c1: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
1404da1c7: 0f 84 99 50 00 00           	je	0x1404df266 <.text+0x4de266>
1404da1cd: 48 01 c1                    	add	rcx, rax
1404da1d0: c6 85 a4 14 00 00 01        	mov	byte ptr [rbp + 0x14a4], 0x1
1404da1d7: 31 d2                       	xor	edx, edx
1404da1d9: e8 32 55 82 00              	call	0x140cff710 <.text+0xcfe710>
1404da1de: 8b 48 48                    	mov	ecx, dword ptr [rax + 0x48]
1404da1e1: 83 f9 03                    	cmp	ecx, 0x3
1404da1e4: 0f 84 7c 50 00 00           	je	0x1404df266 <.text+0x4de266>
1404da1ea: 48 8b 50 40                 	mov	rdx, qword ptr [rax + 0x40]
1404da1ee: 48 89 95 70 0c 00 00        	mov	qword ptr [rbp + 0xc70], rdx
1404da1f5: 66 0f 10 00                 	movupd	xmm0, xmmword ptr [rax]
1404da1f9: f3 0f 6f 48 10              	movdqu	xmm1, xmmword ptr [rax + 0x10]
1404da1fe: f3 0f 6f 50 20              	movdqu	xmm2, xmmword ptr [rax + 0x20]
1404da203: f3 0f 6f 58 30              	movdqu	xmm3, xmmword ptr [rax + 0x30]
1404da208: 66 0f 7f 9d 60 0c 00 00     	movdqa	xmmword ptr [rbp + 0xc60], xmm3
1404da210: 66 0f 7f 95 50 0c 00 00     	movdqa	xmmword ptr [rbp + 0xc50], xmm2
1404da218: 66 0f 7f 8d 40 0c 00 00     	movdqa	xmmword ptr [rbp + 0xc40], xmm1
1404da220: 66 0f 29 85 30 0c 00 00     	movapd	xmmword ptr [rbp + 0xc30], xmm0
1404da228: 89 8d 78 0c 00 00           	mov	dword ptr [rbp + 0xc78], ecx
1404da22e: 8b 48 54                    	mov	ecx, dword ptr [rax + 0x54]
1404da231: 48 8d 95 38 0c 00 00        	lea	rdx, [rbp + 0xc38]
1404da238: 89 4a 4c                    	mov	dword ptr [rdx + 0x4c], ecx
1404da23b: 48 8b 40 4c                 	mov	rax, qword ptr [rax + 0x4c]
1404da23f: 48 89 42 44                 	mov	qword ptr [rdx + 0x44], rax
1404da243: c6 85 a4 14 00 00 01        	mov	byte ptr [rbp + 0x14a4], 0x1
1404da24a: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
1404da24e: 4c 89 e2                    	mov	rdx, r12
1404da251: e8 8a df 3c 00              	call	0x1408a81e0 <.text+0x8a71e0>
1404da256: c6 85 a4 14 00 00 01        	mov	byte ptr [rbp + 0x14a4], 0x1
1404da25d: 48 8d 8d 00 06 00 00        	lea	rcx, [rbp + 0x600]
1404da264: 48 8d 55 c0                 	lea	rdx, [rbp - 0x40]
1404da268: e8 53 01 e5 ff              	call	0x14032a3c0 <.text+0x3293c0>
1404da26d: c6 85 a4 14 00 00 01        	mov	byte ptr [rbp + 0x14a4], 0x1
1404da274: 48 8d 8d 38 06 00 00        	lea	rcx, [rbp + 0x638]
1404da27b: 48 ba a3 8b f6 7e 77 b5 d7 ae       	movabs	rdx, -0x51284a888109745d
1404da285: 49 b8 72 2b 69 b0 99 a1 34 58       	movabs	r8, 0x5834a199b0692b72
1404da28f: e8 5c 34 c0 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1404da294: 48 85 c0                    	test	rax, rax
1404da297: 0f 84 e4 51 00 00           	je	0x1404df481 <.text+0x4de481>
1404da29d: 48 39 95 18 08 00 00        	cmp	qword ptr [rbp + 0x818], rdx
1404da2a4: 0f 86 d7 51 00 00           	jbe	0x1404df481 <.text+0x4de481>
1404da2aa: 48 8b 85 10 08 00 00        	mov	rax, qword ptr [rbp + 0x810]
1404da2b1: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1404da2b5: 48 85 c0                    	test	rax, rax
1404da2b8: 0f 84 c3 51 00 00           	je	0x1404df481 <.text+0x4de481>
1404da2be: 48 8b 8d e0 07 00 00        	mov	rcx, qword ptr [rbp + 0x7e0]
1404da2c5: 48 f7 d0                    	not	rax
1404da2c8: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1404da2cc: 48 c1 e0 05                 	shl	rax, 0x5
1404da2d0: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
1404da2d6: 0f 84 a5 51 00 00           	je	0x1404df481 <.text+0x4de481>
1404da2dc: 48 01 c1                    	add	rcx, rax
1404da2df: c6 85 a4 14 00 00 01        	mov	byte ptr [rbp + 0x14a4], 0x1
1404da2e6: 31 d2                       	xor	edx, edx
1404da2e8: e8 23 54 82 00              	call	0x140cff710 <.text+0xcfe710>
1404da2ed: 48 8b 08                    	mov	rcx, qword ptr [rax]
1404da2f0: c6 85 a4 14 00 00 01        	mov	byte ptr [rbp + 0x14a4], 0x1
1404da2f7: e8 84 3a 1a 00              	call	0x14067dd80 <.text+0x67cd80>
1404da2fc: c6 85 a4 14 00 00 01        	mov	byte ptr [rbp + 0x14a4], 0x1
1404da303: 48 8d 8d 00 06 00 00        	lea	rcx, [rbp + 0x600]
1404da30a: 48 89 c2                    	mov	rdx, rax
1404da30d: e8 7e f5 e4 ff              	call	0x140329890 <.text+0x328890>
1404da312: 41 b8 28 06 00 00           	mov	r8d, 0x628
1404da318: 4c 89 e1                    	mov	rcx, r12
1404da31b: 48 8d 95 00 06 00 00        	lea	rdx, [rbp + 0x600]
1404da322: e8 d9 98 44 02              	call	0x142923c00 <.text+0x2922c00>
1404da327: 4c 89 f9                    	mov	rcx, r15
1404da32a: e8 31 e8 10 00              	call	0x1405e8b60 <.text+0x5e7b60>
1404da32f: 41 b8 28 06 00 00           	mov	r8d, 0x628
1404da335: 48 8b 8d 50 14 00 00        	mov	rcx, qword ptr [rbp + 0x1450]
1404da33c: 4c 89 e2                    	mov	rdx, r12
1404da33f: e8 bc 98 44 02              	call	0x142923c00 <.text+0x2922c00>
1404da344: eb 3b                       	jmp	0x1404da381 <.text+0x4d9381>
1404da346: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
1404da350: 48 89 f0                    	mov	rax, rsi
1404da353: 48 c1 e0 07                 	shl	rax, 0x7
1404da357: 48 8d 0c f0                 	lea	rcx, [rax + 8*rsi]
1404da35b: 48 03 8d b8 13 00 00        	add	rcx, qword ptr [rbp + 0x13b8]
1404da362: 41 b8 88 00 00 00           	mov	r8d, 0x88
1404da368: 48 89 fa                    	mov	rdx, rdi
1404da36b: e8 a0 98 44 02              	call	0x142923c10 <.text+0x2922c10>
1404da370: 48 ff c6                    	inc	rsi
1404da373: 48 89 b5 c0 13 00 00        	mov	qword ptr [rbp + 0x13c0], rsi
1404da37a: 4c 8b b5 78 14 00 00        	mov	r14, qword ptr [rbp + 0x1478]
1404da381: 49 8b 46 10                 	mov	rax, qword ptr [r14 + 0x10]
1404da385: 49 8b 56 18                 	mov	rdx, qword ptr [r14 + 0x18]
1404da389: 48 83 f8 02                 	cmp	rax, 0x2
1404da38d: 74 21                       	je	0x1404da3b0 <.text+0x4d93b0>
1404da38f: 83 f8 01                    	cmp	eax, 0x1
1404da392: 75 2c                       	jne	0x1404da3c0 <.text+0x4d93c0>
1404da394: 48 8d 8d 00 06 00 00        	lea	rcx, [rbp + 0x600]
1404da39b: e8 20 d7 d8 ff              	call	0x140267ac0 <.text+0x266ac0>
1404da3a0: eb 2a                       	jmp	0x1404da3cc <.text+0x4d93cc>
1404da3a2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nop	word ptr cs:[rax + rax]
1404da3b0: 48 8d 8d 00 06 00 00        	lea	rcx, [rbp + 0x600]
1404da3b7: e8 c4 50 04 00              	call	0x14051f480 <.text+0x51e480>
1404da3bc: eb 0e                       	jmp	0x1404da3cc <.text+0x4d93cc>
1404da3be: 66 90                       	nop
1404da3c0: 48 8d 8d 00 06 00 00        	lea	rcx, [rbp + 0x600]
1404da3c7: e8 d4 85 01 00              	call	0x1404f29a0 <.text+0x4f19a0>
1404da3cc: 66 83 bd 00 06 00 00 12     	cmp	word ptr [rbp + 0x600], 0x12
1404da3d4: 74 4a                       	je	0x1404da420 <.text+0x4d9420>
1404da3d6: 41 b8 88 00 00 00           	mov	r8d, 0x88
1404da3dc: 48 8d bd 30 0c 00 00        	lea	rdi, [rbp + 0xc30]
1404da3e3: 48 89 f9                    	mov	rcx, rdi
1404da3e6: 48 8d 95 00 06 00 00        	lea	rdx, [rbp + 0x600]
1404da3ed: e8 0e 98 44 02              	call	0x142923c00 <.text+0x2922c00>
1404da3f2: 48 8b b5 c0 13 00 00        	mov	rsi, qword ptr [rbp + 0x13c0]
1404da3f9: 48 3b b5 b0 13 00 00        	cmp	rsi, qword ptr [rbp + 0x13b0]
1404da400: 0f 85 4a ff ff ff           	jne	0x1404da350 <.text+0x4d9350>
1404da406: 48 8d 8d b0 13 00 00        	lea	rcx, [rbp + 0x13b0]
1404da40d: 48 8d 15 e4 f8 50 02        	lea	rdx, [rip + 0x250f8e4]  # 0x1429e9cf8
1404da414: e8 57 8d 08 00              	call	0x140563170 <.text+0x562170>
1404da419: e9 32 ff ff ff              	jmp	0x1404da350 <.text+0x4d9350>
1404da41e: 66 90                       	nop
1404da420: 80 bd 02 06 00 00 00        	cmp	byte ptr [rbp + 0x602], 0x0
1404da427: 0f 85 81 49 00 00           	jne	0x1404dedae <.text+0x4dddae>
1404da42d: 48 8b 8d 88 14 00 00        	mov	rcx, qword ptr [rbp + 0x1488]
1404da434: 48 ba 51 3b 7b bf c4 68 38 fb       	movabs	rdx, -0x4c7973b4084c4af
1404da43e: 49 b8 39 f6 dc 90 20 3d 9e e4       	movabs	r8, -0x1b61c2df6f2309c7
1404da448: e8 a3 32 c0 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1404da44d: 48 85 c0                    	test	rax, rax
1404da450: 48 8b 8d 78 14 00 00        	mov	rcx, qword ptr [rbp + 0x1478]
1404da457: 74 37                       	je	0x1404da490 <.text+0x4d9490>
1404da459: 48 39 91 48 02 00 00        	cmp	qword ptr [rcx + 0x248], rdx
1404da460: 76 2e                       	jbe	0x1404da490 <.text+0x4d9490>
1404da462: 48 8b 81 40 02 00 00        	mov	rax, qword ptr [rcx + 0x240]
1404da469: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1404da46d: 48 85 c0                    	test	rax, rax
1404da470: 74 1e                       	je	0x1404da490 <.text+0x4d9490>
1404da472: 48 8b 89 10 02 00 00        	mov	rcx, qword ptr [rcx + 0x210]
1404da479: 48 f7 d0                    	not	rax
1404da47c: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
1404da480: 48 c1 e0 05                 	shl	rax, 0x5
1404da484: 48 83 7c 01 20 00           	cmp	qword ptr [rcx + rax + 0x20], 0x0
1404da48a: 0f 85 94 49 00 00           	jne	0x1404dee24 <.text+0x4dde24>
1404da490: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
1404da494: 48 8b 95 50 14 00 00        	mov	rdx, qword ptr [rbp + 0x1450]
1404da49b: e8 00 a6 dd ff              	call	0x1402b4aa0 <.text+0x2b3aa0>
1404da4a0: 8b 75 c0                    	mov	esi, dword ptr [rbp - 0x40]
1404da4a3: 83 fe 03                    	cmp	esi, 0x3
1404da4a6: 0f 84 a4 01 00 00           	je	0x1404da650 <.text+0x4d9650>
1404da4ac: f2 0f 10 75 c4              	movsd	xmm6, qword ptr [rbp - 0x3c]
1404da4b1: 83 fe 02                    	cmp	esi, 0x2
1404da4b4: 0f 82 c3 00 00 00           	jb	0x1404da57d <.text+0x4d957d>
1404da4ba: 48 8b 8d 88 14 00 00        	mov	rcx, qword ptr [rbp + 0x1488]
1404da4c1: 48 ba 36 26 d5 31 c3 1a 3f 92       	movabs	rdx, -0x6dc0e53cce2ad9ca
1404da4cb: 49 b8 f6 2c 93 ff ea c9 14 41       	movabs	r8, 0x4114c9eaff932cf6
1404da4d5: e8 16 32 c0 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1404da4da: 48 85 c0                    	test	rax, rax
1404da4dd: 4c 8b 85 78 14 00 00        	mov	r8, qword ptr [rbp + 0x1478]
1404da4e4: 0f 84 15 50 00 00           	je	0x1404df4ff <.text+0x4de4ff>
1404da4ea: 49 39 90 48 02 00 00        	cmp	qword ptr [r8 + 0x248], rdx
1404da4f1: 0f 86 08 50 00 00           	jbe	0x1404df4ff <.text+0x4de4ff>
1404da4f7: 49 8b 80 40 02 00 00        	mov	rax, qword ptr [r8 + 0x240]
1404da4fe: 48 8b 0c d0                 	mov	rcx, qword ptr [rax + 8*rdx]
1404da502: 48 85 c9                    	test	rcx, rcx
1404da505: 0f 84 f4 4f 00 00           	je	0x1404df4ff <.text+0x4de4ff>
1404da50b: 49 8b 80 10 02 00 00        	mov	rax, qword ptr [r8 + 0x210]
1404da512: 48 f7 d1                    	not	rcx
1404da515: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
1404da519: 48 c1 e1 05                 	shl	rcx, 0x5
1404da51d: 48 83 7c 08 20 00           	cmp	qword ptr [rax + rcx + 0x20], 0x0
1404da523: 0f 84 d6 4f 00 00           	je	0x1404df4ff <.text+0x4de4ff>
1404da529: 48 01 c8                    	add	rax, rcx
1404da52c: 48 8b 78 10                 	mov	rdi, qword ptr [rax + 0x10]
1404da530: 41 8b 88 e0 05 00 00        	mov	ecx, dword ptr [r8 + 0x5e0]
1404da537: 89 48 5c                    	mov	dword ptr [rax + 0x5c], ecx
1404da53a: 48 8b 5f 30                 	mov	rbx, qword ptr [rdi + 0x30]
1404da53e: 4c 8b 77 40                 	mov	r14, qword ptr [rdi + 0x40]
1404da542: 48 3b 5f 20                 	cmp	rbx, qword ptr [rdi + 0x20]
1404da546: 75 10                       	jne	0x1404da558 <.text+0x4d9558>
1404da548: 48 8d 4f 20                 	lea	rcx, [rdi + 0x20]
1404da54c: 48 8d 15 d5 8b 51 02        	lea	rdx, [rip + 0x2518bd5]  # 0x1429f3128
1404da553: e8 58 88 08 00              	call	0x140562db0 <.text+0x561db0>
1404da558: 48 8b 47 28                 	mov	rax, qword ptr [rdi + 0x28]
1404da55c: 48 8d 0c 5b                 	lea	rcx, [rbx + 2*rbx]
1404da560: 4c 89 34 c8                 	mov	qword ptr [rax + 8*rcx], r14
1404da564: c7 44 c8 08 01 00 00 00     	mov	dword ptr [rax + 8*rcx + 0x8], 0x1
1404da56c: 66 0f 13 74 c8 0c           	movlpd	qword ptr [rax + 8*rcx + 0xc], xmm6
1404da572: 48 ff c3                    	inc	rbx
1404da575: 48 89 5f 30                 	mov	qword ptr [rdi + 0x30], rbx
1404da579: 48 ff 47 40                 	inc	qword ptr [rdi + 0x40]
1404da57d: 48 8b 8d 78 14 00 00        	mov	rcx, qword ptr [rbp + 0x1478]
1404da584: 48 8b 81 f8 10 00 00        	mov	rax, qword ptr [rcx + 0x10f8]
1404da58b: 48 ff c0                    	inc	rax
1404da58e: 48 89 81 f8 10 00 00        	mov	qword ptr [rcx + 0x10f8], rax
1404da595: 89 b5 40 0c 00 00           	mov	dword ptr [rbp + 0xc40], esi
1404da59b: 66 0f 13 b5 44 0c 00 00     	movlpd	qword ptr [rbp + 0xc44], xmm6
1404da5a3: 48 89 85 38 0c 00 00        	mov	qword ptr [rbp + 0xc38], rax
1404da5aa: 48 b8 01 00 00 00 00 00 00 80       	movabs	rax, -0x7fffffffffffffff
1404da5b4: 48 89 85 30 0c 00 00        	mov	qword ptr [rbp + 0xc30], rax
1404da5bb: 48 8d 8d 00 06 00 00        	lea	rcx, [rbp + 0x600]
1404da5c2: 48 8b 95 18 14 00 00        	mov	rdx, qword ptr [rbp + 0x1418]
1404da5c9: 4c 8d 85 30 0c 00 00        	lea	r8, [rbp + 0xc30]
1404da5d0: e8 3b b4 ee ff              	call	0x1403c5a10 <.text+0x3c4a10>
1404da5d5: 48 8b 85 00 06 00 00        	mov	rax, qword ptr [rbp + 0x600]
1404da5dc: 48 b9 07 00 00 00 00 00 00 80       	movabs	rcx, -0x7ffffffffffffff9
1404da5e6: 48 39 c8                    	cmp	rax, rcx
1404da5e9: 74 65                       	je	0x1404da650 <.text+0x4d9650>
1404da5eb: 48 b9 00 00 00 00 00 00 00 80       	movabs	rcx, -0x8000000000000000
1404da5f5: 48 31 c8                    	xor	rax, rcx
1404da5f8: 48 83 f8 07                 	cmp	rax, 0x7
1404da5fc: b9 03 00 00 00              	mov	ecx, 0x3
1404da601: 48 0f 43 c1                 	cmovae	rax, rcx
1404da605: 48 83 f8 04                 	cmp	rax, 0x4
1404da609: 74 14                       	je	0x1404da61f <.text+0x4d961f>
1404da60b: 48 83 f8 03                 	cmp	rax, 0x3
1404da60f: 75 3f                       	jne	0x1404da650 <.text+0x4d9650>
1404da611: 48 8d 8d 00 06 00 00        	lea	rcx, [rbp + 0x600]
1404da618: e8 f3 66 11 00              	call	0x1405f0d10 <.text+0x5efd10>
1404da61d: eb 31                       	jmp	0x1404da650 <.text+0x4d9650>
1404da61f: 48 8b 95 08 06 00 00        	mov	rdx, qword ptr [rbp + 0x608]
1404da626: 48 b8 02 00 00 00 00 00 00 80       	movabs	rax, -0x7ffffffffffffffe
1404da630: 48 39 c2                    	cmp	rdx, rax
1404da633: 7c 1b                       	jl	0x1404da650 <.text+0x4d9650>
1404da635: 48 85 d2                    	test	rdx, rdx
1404da638: 74 16                       	je	0x1404da650 <.text+0x4d9650>
1404da63a: 48 8b 8d 10 06 00 00        	mov	rcx, qword ptr [rbp + 0x610]
1404da641: 41 b8 01 00 00 00           	mov	r8d, 0x1
1404da647: e8 d4 95 13 00              	call	0x140613c20 <.text+0x612c20>
1404da64c: 0f 1f 40 00                 	nop	dword ptr [rax]
1404da650: 48 8d 8d 30 0c 00 00        	lea	rcx, [rbp + 0xc30]
1404da657: 48 8b 95 50 14 00 00        	mov	rdx, qword ptr [rbp + 0x1450]
1404da65e: e8 8d a1 dd ff              	call	0x1402b47f0 <.text+0x2b37f0>
1404da663: 4c 8b ad 40 0c 00 00        	mov	r13, qword ptr [rbp + 0xc40]
1404da66a: 4d 85 ed                    	test	r13, r13
1404da66d: 0f 84 ed 02 00 00           	je	0x1404da960 <.text+0x4d9960>
1404da673: 48 8b 9d 38 0c 00 00        	mov	rbx, qword ptr [rbp + 0xc38]
1404da67a: 49 01 dd                    	add	r13, rbx
1404da67d: eb 1d                       	jmp	0x1404da69c <.text+0x4d969c>
1404da67f: e8 9c 95 13 00              	call	0x140613c20 <.text+0x612c20>
1404da684: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
1404da690: 49 ff cd                    	dec	r13
1404da693: 4c 39 eb                    	cmp	rbx, r13
1404da696: 0f 84 c4 02 00 00           	je	0x1404da960 <.text+0x4d9960>
1404da69c: 48 8b 8d 78 14 00 00        	mov	rcx, qword ptr [rbp + 0x1478]
1404da6a3: 48 8b 41 20                 	mov	rax, qword ptr [rcx + 0x20]
1404da6a7: 48 8b 51 28                 	mov	rdx, qword ptr [rcx + 0x28]
1404da6ab: 48 83 f8 02                 	cmp	rax, 0x2
1404da6af: 74 3f                       	je	0x1404da6f0 <.text+0x4d96f0>
1404da6b1: 83 f8 01                    	cmp	eax, 0x1
1404da6b4: 75 6a                       	jne	0x1404da720 <.text+0x4d9720>
1404da6b6: 48 b8 02 00 00 00 00 00 00 80       	movabs	rax, -0x7ffffffffffffffe
1404da6c0: 48 89 85 30 0c 00 00        	mov	qword ptr [rbp + 0xc30], rax
1404da6c7: c7 44 24 20 00 ca 9a 3b     	mov	dword ptr [rsp + 0x20], 0x3b9aca00
1404da6cf: 48 8d 8d 00 06 00 00        	lea	rcx, [rbp + 0x600]
1404da6d6: 4c 8d 85 30 0c 00 00        	lea	r8, [rbp + 0xc30]
1404da6dd: e8 ee bb d8 ff              	call	0x1402662d0 <.text+0x2652d0>
1404da6e2: eb 68                       	jmp	0x1404da74c <.text+0x4d974c>
1404da6e4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
1404da6f0: 48 b8 02 00 00 00 00 00 00 80       	movabs	rax, -0x7ffffffffffffffe
1404da6fa: 48 89 85 30 0c 00 00        	mov	qword ptr [rbp + 0xc30], rax
1404da701: c7 44 24 20 00 ca 9a 3b     	mov	dword ptr [rsp + 0x20], 0x3b9aca00
1404da709: 48 8d 8d 00 06 00 00        	lea	rcx, [rbp + 0x600]
1404da710: 4c 8d 85 30 0c 00 00        	lea	r8, [rbp + 0xc30]
1404da717: e8 a4 d5 03 00              	call	0x140517cc0 <.text+0x516cc0>
1404da71c: eb 2e                       	jmp	0x1404da74c <.text+0x4d974c>
1404da71e: 66 90                       	nop
1404da720: 48 b8 02 00 00 00 00 00 00 80       	movabs	rax, -0x7ffffffffffffffe
1404da72a: 48 89 85 30 0c 00 00        	mov	qword ptr [rbp + 0xc30], rax
1404da731: c7 44 24 20 00 ca 9a 3b     	mov	dword ptr [rsp + 0x20], 0x3b9aca00
1404da739: 48 8d 8d 00 06 00 00        	lea	rcx, [rbp + 0x600]
1404da740: 4c 8d 85 30 0c 00 00        	lea	r8, [rbp + 0xc30]
1404da747: e8 c4 5e 01 00              	call	0x1404f0610 <.text+0x4ef610>
1404da74c: 83 bd 00 06 00 00 02        	cmp	dword ptr [rbp + 0x600], 0x2
1404da753: 0f 84 37 ff ff ff           	je	0x1404da690 <.text+0x4d9690>
1404da759: 0f 10 85 00 06 00 00        	movups	xmm0, xmmword ptr [rbp + 0x600]
1404da760: f3 0f 6f 8d 10 06 00 00     	movdqu	xmm1, xmmword ptr [rbp + 0x610]
1404da768: f3 0f 6f 95 20 06 00 00     	movdqu	xmm2, xmmword ptr [rbp + 0x620]
1404da770: f3 0f 6f 9d 30 06 00 00     	movdqu	xmm3, xmmword ptr [rbp + 0x630]
1404da778: 0f 29 85 30 0c 00 00        	movaps	xmmword ptr [rbp + 0xc30], xmm0
1404da77f: 0f 10 85 70 06 00 00        	movups	xmm0, xmmword ptr [rbp + 0x670]
1404da786: 0f 29 85 a0 0c 00 00        	movaps	xmmword ptr [rbp + 0xca0], xmm0
1404da78d: 0f 10 85 60 06 00 00        	movups	xmm0, xmmword ptr [rbp + 0x660]
1404da794: 0f 29 85 90 0c 00 00        	movaps	xmmword ptr [rbp + 0xc90], xmm0
1404da79b: 0f 10 85 50 06 00 00        	movups	xmm0, xmmword ptr [rbp + 0x650]
1404da7a2: 0f 29 85 80 0c 00 00        	movaps	xmmword ptr [rbp + 0xc80], xmm0
1404da7a9: 66 0f 10 85 40 06 00 00     	movupd	xmm0, xmmword ptr [rbp + 0x640]
1404da7b1: 66 0f 29 85 70 0c 00 00     	movapd	xmmword ptr [rbp + 0xc70], xmm0
1404da7b9: 66 0f 7f 9d 60 0c 00 00     	movdqa	xmmword ptr [rbp + 0xc60], xmm3
1404da7c1: 66 0f 7f 95 50 0c 00 00     	movdqa	xmmword ptr [rbp + 0xc50], xmm2
1404da7c9: 66 0f 7f 8d 40 0c 00 00     	movdqa	xmmword ptr [rbp + 0xc40], xmm1
1404da7d1: 48 8b 85 30 0c 00 00        	mov	rax, qword ptr [rbp + 0xc30]
1404da7d8: 48 85 c0                    	test	rax, rax
1404da7db: 0f 84 7d 47 00 00           	je	0x1404def5e <.text+0x4ddf5e>
1404da7e1: 4c 8b a5 08 06 00 00        	mov	r12, qword ptr [rbp + 0x608]
1404da7e8: 48 b8 07 00 00 00 00 00 00 80       	movabs	rax, -0x7ffffffffffffff9
1404da7f2: 49 39 c4                    	cmp	r12, rax
1404da7f5: 0f 84 95 fe ff ff           	je	0x1404da690 <.text+0x4d9690>
1404da7fb: 4c 8b b5 10 06 00 00        	mov	r14, qword ptr [rbp + 0x610]
1404da802: 4c 89 e0                    	mov	rax, r12
1404da805: 48 b9 00 00 00 00 00 00 00 80       	movabs	rcx, -0x8000000000000000
1404da80f: 48 31 c8                    	xor	rax, rcx
1404da812: 48 83 f8 07                 	cmp	rax, 0x7
1404da816: b9 03 00 00 00              	mov	ecx, 0x3
1404da81b: 48 0f 43 c1                 	cmovae	rax, rcx
1404da81f: 48 83 f8 04                 	cmp	rax, 0x4
1404da823: 0f 84 e6 00 00 00           	je	0x1404da90f <.text+0x4d990f>
1404da829: 48 83 f8 03                 	cmp	rax, 0x3
1404da82d: 0f 85 5d fe ff ff           	jne	0x1404da690 <.text+0x4d9690>
1404da833: 48 8b b5 20 06 00 00        	mov	rsi, qword ptr [rbp + 0x620]
1404da83a: 48 8b bd 28 06 00 00        	mov	rdi, qword ptr [rbp + 0x628]
1404da841: 4c 8b bd 30 06 00 00        	mov	r15, qword ptr [rbp + 0x630]
1404da848: 48 8b 95 38 06 00 00        	mov	rdx, qword ptr [rbp + 0x638]
1404da84f: 48 8d 04 55 00 00 00 00     	lea	rax, [2*rdx]
1404da857: 48 85 c0                    	test	rax, rax
1404da85a: 0f 85 e0 00 00 00           	jne	0x1404da940 <.text+0x4d9940>
1404da860: 4d 85 e4                    	test	r12, r12
1404da863: 74 11                       	je	0x1404da876 <.text+0x4d9876>
1404da865: 41 b8 01 00 00 00           	mov	r8d, 0x1
1404da86b: 4c 89 f1                    	mov	rcx, r14
1404da86e: 4c 89 e2                    	mov	rdx, r12
1404da871: e8 aa 93 13 00              	call	0x140613c20 <.text+0x612c20>
1404da876: 4d 85 ff                    	test	r15, r15
1404da879: 74 76                       	je	0x1404da8f1 <.text+0x4d98f1>
1404da87b: 4c 8d 77 50                 	lea	r14, [rdi + 0x50]
1404da87f: eb 18                       	jmp	0x1404da899 <.text+0x4d9899>
1404da881: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
1404da890: 49 83 ee 80                 	sub	r14, -0x80
1404da894: 49 ff cf                    	dec	r15
1404da897: 74 58                       	je	0x1404da8f1 <.text+0x4d98f1>
1404da899: 49 8b 56 c8                 	mov	rdx, qword ptr [r14 - 0x38]
1404da89d: 48 89 d0                    	mov	rax, rdx
1404da8a0: 48 f7 d8                    	neg	rax
1404da8a3: 70 11                       	jo	0x1404da8b6 <.text+0x4d98b6>
1404da8a5: 73 0f                       	jae	0x1404da8b6 <.text+0x4d98b6>
1404da8a7: 49 8b 4e d0                 	mov	rcx, qword ptr [r14 - 0x30]
1404da8ab: 41 b8 01 00 00 00           	mov	r8d, 0x1
1404da8b1: e8 6a 93 13 00              	call	0x140613c20 <.text+0x612c20>
1404da8b6: 49 8b 56 e0                 	mov	rdx, qword ptr [r14 - 0x20]
1404da8ba: 48 89 d0                    	mov	rax, rdx
1404da8bd: 48 f7 d8                    	neg	rax
1404da8c0: 70 11                       	jo	0x1404da8d3 <.text+0x4d98d3>
1404da8c2: 73 0f                       	jae	0x1404da8d3 <.text+0x4d98d3>
1404da8c4: 49 8b 4e e8                 	mov	rcx, qword ptr [r14 - 0x18]
1404da8c8: 41 b8 01 00 00 00           	mov	r8d, 0x1
1404da8ce: e8 4d 93 13 00              	call	0x140613c20 <.text+0x612c20>
1404da8d3: 49 8b 56 f8                 	mov	rdx, qword ptr [r14 - 0x8]
1404da8d7: 48 89 d0                    	mov	rax, rdx
1404da8da: 48 f7 d8                    	neg	rax
1404da8dd: 70 b1                       	jo	0x1404da890 <.text+0x4d9890>
1404da8df: 73 af                       	jae	0x1404da890 <.text+0x4d9890>
1404da8e1: 49 8b 0e                    	mov	rcx, qword ptr [r14]
1404da8e4: 41 b8 01 00 00 00           	mov	r8d, 0x1
1404da8ea: e8 31 93 13 00              	call	0x140613c20 <.text+0x612c20>
1404da8ef: eb 9f                       	jmp	0x1404da890 <.text+0x4d9890>
1404da8f1: 48 85 f6                    	test	rsi, rsi
1404da8f4: 0f 84 96 fd ff ff           	je	0x1404da690 <.text+0x4d9690>
1404da8fa: 48 c1 e6 07                 	shl	rsi, 0x7
1404da8fe: 41 b8 08 00 00 00           	mov	r8d, 0x8
1404da904: 48 89 f9                    	mov	rcx, rdi
1404da907: 48 89 f2                    	mov	rdx, rsi
1404da90a: e9 70 fd ff ff              	jmp	0x1404da67f <.text+0x4d967f>
1404da90f: 48 b8 02 00 00 00 00 00 00 80       	movabs	rax, -0x7ffffffffffffffe
1404da919: 49 39 c6                    	cmp	r14, rax
1404da91c: 0f 8c 6e fd ff ff           	jl	0x1404da690 <.text+0x4d9690>
1404da922: 4d 85 f6                    	test	r14, r14
1404da925: 0f 84 65 fd ff ff           	je	0x1404da690 <.text+0x4d9690>
1404da92b: 48 8b 8d 18 06 00 00        	mov	rcx, qword ptr [rbp + 0x618]
1404da932: 41 b8 01 00 00 00           	mov	r8d, 0x1
1404da938: 4c 89 f2                    	mov	rdx, r14
1404da93b: e9 3f fd ff ff              	jmp	0x1404da67f <.text+0x4d967f>
1404da940: 48 8b 8d 40 06 00 00        	mov	rcx, qword ptr [rbp + 0x640]
1404da947: 41 b8 01 00 00 00           	mov	r8d, 0x1
1404da94d: e8 ce 92 13 00              	call	0x140613c20 <.text+0x612c20>
1404da952: 4d 85 e4                    	test	r12, r12
1404da955: 0f 85 0a ff ff ff           	jne	0x1404da865 <.text+0x4d9865>
1404da95b: e9 16 ff ff ff              	jmp	0x1404da876 <.text+0x4d9876>
1404da960: 48 8b 8d 50 14 00 00        	mov	rcx, qword ptr [rbp + 0x1450]
1404da967: e8 d4 a2 dd ff              	call	0x1402b4c40 <.text+0x2b3c40>
1404da96c: 83 f8 04                    	cmp	eax, 0x4
1404da96f: 0f 84 bb 00 00 00           	je	0x1404daa30 <.text+0x4d9a30>
1404da975: 48 89 c1                    	mov	rcx, rax
1404da978: 48 c1 e9 20                 	shr	rcx, 0x20
1404da97c: 89 85 38 0c 00 00           	mov	dword ptr [rbp + 0xc38], eax
1404da982: 89 8d 3c 0c 00 00           	mov	dword ptr [rbp + 0xc3c], ecx
1404da988: 48 b8 00 00 00 00 00 00 00 80       	movabs	rax, -0x8000000000000000
1404da992: 48 89 85 30 0c 00 00        	mov	qword ptr [rbp + 0xc30], rax
1404da999: 48 8d 8d 00 06 00 00        	lea	rcx, [rbp + 0x600]
1404da9a0: 48 8b 95 18 14 00 00        	mov	rdx, qword ptr [rbp + 0x1418]
1404da9a7: 4c 8d 85 30 0c 00 00        	lea	r8, [rbp + 0xc30]
1404da9ae: e8 5d b0 ee ff              	call	0x1403c5a10 <.text+0x3c4a10>
1404da9b3: 48 8b 85 00 06 00 00        	mov	rax, qword ptr [rbp + 0x600]
1404da9ba: 48 b9 07 00 00 00 00 00 00 80       	movabs	rcx, -0x7ffffffffffffff9
1404da9c4: 48 39 c8                    	cmp	rax, rcx
1404da9c7: 74 67                       	je	0x1404daa30 <.text+0x4d9a30>
1404da9c9: 48 b9 00 00 00 00 00 00 00 80       	movabs	rcx, -0x8000000000000000
1404da9d3: 48 31 c8                    	xor	rax, rcx
1404da9d6: 48 83 f8 07                 	cmp	rax, 0x7
1404da9da: b9 03 00 00 00              	mov	ecx, 0x3
1404da9df: 48 0f 43 c1                 	cmovae	rax, rcx
