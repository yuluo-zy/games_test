
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a38610: 55                          	push	rbp
140a38611: 41 57                       	push	r15
140a38613: 41 56                       	push	r14
140a38615: 41 55                       	push	r13
140a38617: 41 54                       	push	r12
140a38619: 56                          	push	rsi
140a3861a: 57                          	push	rdi
140a3861b: 53                          	push	rbx
140a3861c: 48 81 ec 28 05 00 00        	sub	rsp, 0x528
140a38623: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
140a3862b: 48 c7 85 a0 04 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x4a0], -0x2
140a38636: 4c 89 8d 60 04 00 00        	mov	qword ptr [rbp + 0x460], r9
140a3863d: 48 8b b5 18 05 00 00        	mov	rsi, qword ptr [rbp + 0x518]
140a38644: 48 8b bd 10 05 00 00        	mov	rdi, qword ptr [rbp + 0x510]
140a3864b: 45 31 c9                    	xor	r9d, r9d
140a3864e: 48 8d 05 43 53 09 02        	lea	rax, [rip + 0x2095343]  # 0x142acd998
140a38655: eb 09                       	jmp	0x140a38660 <.text+0xa37660>
140a38657: 41 b1 12                    	mov	r9b, 0x12
140a3865a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
140a38660: 45 0f b6 c9                 	movzx	r9d, r9b
140a38664: 4e 63 14 88                 	movsxd	r10, dword ptr [rax + 4*r9]
140a38668: 49 01 c2                    	add	r10, rax
140a3866b: 41 b1 02                    	mov	r9b, 0x2
140a3866e: 41 ff e2                    	jmp	r10
140a38671: 41 b1 01                    	mov	r9b, 0x1
140a38674: eb ea                       	jmp	0x140a38660 <.text+0xa37660>
140a38676: 41 b1 10                    	mov	r9b, 0x10
140a38679: eb e5                       	jmp	0x140a38660 <.text+0xa37660>
140a3867b: 41 b1 0d                    	mov	r9b, 0xd
140a3867e: eb e0                       	jmp	0x140a38660 <.text+0xa37660>
140a38680: 41 b1 06                    	mov	r9b, 0x6
140a38683: eb db                       	jmp	0x140a38660 <.text+0xa37660>
140a38685: 41 b1 0e                    	mov	r9b, 0xe
140a38688: eb d6                       	jmp	0x140a38660 <.text+0xa37660>
140a3868a: 41 b1 0b                    	mov	r9b, 0xb
140a3868d: eb d1                       	jmp	0x140a38660 <.text+0xa37660>
140a3868f: 41 b1 1b                    	mov	r9b, 0x1b
140a38692: eb cc                       	jmp	0x140a38660 <.text+0xa37660>
140a38694: 41 b1 1a                    	mov	r9b, 0x1a
140a38697: eb c7                       	jmp	0x140a38660 <.text+0xa37660>
140a38699: 41 b1 04                    	mov	r9b, 0x4
140a3869c: eb c2                       	jmp	0x140a38660 <.text+0xa37660>
140a3869e: 41 b1 11                    	mov	r9b, 0x11
140a386a1: eb bd                       	jmp	0x140a38660 <.text+0xa37660>
140a386a3: 41 b1 05                    	mov	r9b, 0x5
140a386a6: eb b8                       	jmp	0x140a38660 <.text+0xa37660>
140a386a8: 41 b1 09                    	mov	r9b, 0x9
140a386ab: eb b3                       	jmp	0x140a38660 <.text+0xa37660>
140a386ad: 41 b1 03                    	mov	r9b, 0x3
140a386b0: eb ae                       	jmp	0x140a38660 <.text+0xa37660>
140a386b2: 41 b1 13                    	mov	r9b, 0x13
140a386b5: eb a9                       	jmp	0x140a38660 <.text+0xa37660>
140a386b7: 41 b1 16                    	mov	r9b, 0x16
140a386ba: eb a4                       	jmp	0x140a38660 <.text+0xa37660>
140a386bc: 41 b1 07                    	mov	r9b, 0x7
140a386bf: eb 9f                       	jmp	0x140a38660 <.text+0xa37660>
140a386c1: 41 b1 17                    	mov	r9b, 0x17
140a386c4: eb 9a                       	jmp	0x140a38660 <.text+0xa37660>
140a386c6: 41 b1 0f                    	mov	r9b, 0xf
140a386c9: eb 95                       	jmp	0x140a38660 <.text+0xa37660>
140a386cb: 41 b1 08                    	mov	r9b, 0x8
140a386ce: eb 90                       	jmp	0x140a38660 <.text+0xa37660>
140a386d0: 41 b1 14                    	mov	r9b, 0x14
140a386d3: eb 8b                       	jmp	0x140a38660 <.text+0xa37660>
140a386d5: 41 b1 0c                    	mov	r9b, 0xc
140a386d8: eb 86                       	jmp	0x140a38660 <.text+0xa37660>
140a386da: 41 b1 0a                    	mov	r9b, 0xa
140a386dd: eb 81                       	jmp	0x140a38660 <.text+0xa37660>
140a386df: 41 b1 18                    	mov	r9b, 0x18
140a386e2: e9 79 ff ff ff              	jmp	0x140a38660 <.text+0xa37660>
140a386e7: 41 b1 15                    	mov	r9b, 0x15
140a386ea: e9 71 ff ff ff              	jmp	0x140a38660 <.text+0xa37660>
140a386ef: 41 b1 19                    	mov	r9b, 0x19
140a386f2: e9 69 ff ff ff              	jmp	0x140a38660 <.text+0xa37660>
140a386f7: 48 8b 01                    	mov	rax, qword ptr [rcx]
140a386fa: 48 8b 00                    	mov	rax, qword ptr [rax]
140a386fd: 48 83 c0 10                 	add	rax, 0x10
140a38701: 48 89 85 88 04 00 00        	mov	qword ptr [rbp + 0x488], rax
140a38708: 48 8d 05 01 45 09 02        	lea	rax, [rip + 0x2094501]  # 0x142accc10
140a3870f: 48 89 85 d0 00 00 00        	mov	qword ptr [rbp + 0xd0], rax
140a38716: 48 c7 85 d8 00 00 00 20 00 00 00    	mov	qword ptr [rbp + 0xd8], 0x20
140a38721: 48 8d 05 08 45 09 02        	lea	rax, [rip + 0x2094508]  # 0x142accc30
140a38728: 48 89 85 e0 00 00 00        	mov	qword ptr [rbp + 0xe0], rax
140a3872f: 48 8d 9d e8 00 00 00        	lea	rbx, [rbp + 0xe8]
140a38736: 48 c7 85 e8 00 00 00 2f 00 00 00    	mov	qword ptr [rbp + 0xe8], 0x2f
140a38741: 48 8d 05 17 45 09 02        	lea	rax, [rip + 0x2094517]  # 0x142accc5f
140a38748: 48 89 85 f0 00 00 00        	mov	qword ptr [rbp + 0xf0], rax
140a3874f: 48 c7 85 f8 00 00 00 0a 00 00 00    	mov	qword ptr [rbp + 0xf8], 0xa
140a3875a: 48 8d 05 08 45 09 02        	lea	rax, [rip + 0x2094508]  # 0x142accc69
140a38761: 48 89 85 00 01 00 00        	mov	qword ptr [rbp + 0x100], rax
140a38768: 48 c7 85 08 01 00 00 20 00 00 00    	mov	qword ptr [rbp + 0x108], 0x20
140a38773: 48 8d 05 0f 45 09 02        	lea	rax, [rip + 0x209450f]  # 0x142accc89
140a3877a: 48 89 85 10 01 00 00        	mov	qword ptr [rbp + 0x110], rax
140a38781: 48 c7 85 18 01 00 00 0b 00 00 00    	mov	qword ptr [rbp + 0x118], 0xb
140a3878c: 48 8d 05 01 45 09 02        	lea	rax, [rip + 0x2094501]  # 0x142accc94
140a38793: 48 89 85 20 01 00 00        	mov	qword ptr [rbp + 0x120], rax
140a3879a: 48 c7 85 28 01 00 00 21 00 00 00    	mov	qword ptr [rbp + 0x128], 0x21
140a387a5: 48 8d 05 09 45 09 02        	lea	rax, [rip + 0x2094509]  # 0x142acccb5
140a387ac: 48 89 85 30 01 00 00        	mov	qword ptr [rbp + 0x130], rax
140a387b3: 48 c7 85 38 01 00 00 17 00 00 00    	mov	qword ptr [rbp + 0x138], 0x17
140a387be: 48 8d 05 07 45 09 02        	lea	rax, [rip + 0x2094507]  # 0x142accccc
140a387c5: 48 89 85 40 01 00 00        	mov	qword ptr [rbp + 0x140], rax
140a387cc: 48 c7 85 48 01 00 00 24 00 00 00    	mov	qword ptr [rbp + 0x148], 0x24
140a387d7: 48 8d 05 12 45 09 02        	lea	rax, [rip + 0x2094512]  # 0x142acccf0
140a387de: 48 89 85 50 01 00 00        	mov	qword ptr [rbp + 0x150], rax
140a387e5: 48 c7 85 58 01 00 00 1e 00 00 00    	mov	qword ptr [rbp + 0x158], 0x1e
140a387f0: 48 8d 05 17 45 09 02        	lea	rax, [rip + 0x2094517]  # 0x142accd0e
140a387f7: 48 89 85 60 01 00 00        	mov	qword ptr [rbp + 0x160], rax
140a387fe: 48 c7 85 68 01 00 00 2b 00 00 00    	mov	qword ptr [rbp + 0x168], 0x2b
140a38809: 48 8d 05 29 45 09 02        	lea	rax, [rip + 0x2094529]  # 0x142accd39
140a38810: 48 89 85 70 01 00 00        	mov	qword ptr [rbp + 0x170], rax
140a38817: 48 c7 85 78 01 00 00 24 00 00 00    	mov	qword ptr [rbp + 0x178], 0x24
140a38822: 48 8d 05 34 45 09 02        	lea	rax, [rip + 0x2094534]  # 0x142accd5d
140a38829: 48 89 85 80 01 00 00        	mov	qword ptr [rbp + 0x180], rax
140a38830: 48 c7 85 88 01 00 00 31 00 00 00    	mov	qword ptr [rbp + 0x188], 0x31
140a3883b: 48 8d 05 4c 45 09 02        	lea	rax, [rip + 0x209454c]  # 0x142accd8e
140a38842: 48 89 85 90 01 00 00        	mov	qword ptr [rbp + 0x190], rax
140a38849: 48 c7 85 98 01 00 00 1e 00 00 00    	mov	qword ptr [rbp + 0x198], 0x1e
140a38854: 48 8d 05 51 45 09 02        	lea	rax, [rip + 0x2094551]  # 0x142accdac
140a3885b: 48 89 85 a0 01 00 00        	mov	qword ptr [rbp + 0x1a0], rax
140a38862: 48 c7 85 a8 01 00 00 2b 00 00 00    	mov	qword ptr [rbp + 0x1a8], 0x2b
140a3886d: 48 8d 05 63 45 09 02        	lea	rax, [rip + 0x2094563]  # 0x142accdd7
140a38874: 48 89 85 b0 01 00 00        	mov	qword ptr [rbp + 0x1b0], rax
140a3887b: 48 c7 85 b8 01 00 00 19 00 00 00    	mov	qword ptr [rbp + 0x1b8], 0x19
140a38886: 48 8d 05 63 45 09 02        	lea	rax, [rip + 0x2094563]  # 0x142accdf0
140a3888d: 48 89 85 c0 01 00 00        	mov	qword ptr [rbp + 0x1c0], rax
140a38894: 48 c7 85 c8 01 00 00 26 00 00 00    	mov	qword ptr [rbp + 0x1c8], 0x26
140a3889f: 48 8d 05 70 45 09 02        	lea	rax, [rip + 0x2094570]  # 0x142acce16
140a388a6: 48 89 85 d0 01 00 00        	mov	qword ptr [rbp + 0x1d0], rax
140a388ad: 48 c7 85 d8 01 00 00 13 00 00 00    	mov	qword ptr [rbp + 0x1d8], 0x13
140a388b8: 48 8d 05 6a 45 09 02        	lea	rax, [rip + 0x209456a]  # 0x142acce29
140a388bf: 48 89 85 e0 01 00 00        	mov	qword ptr [rbp + 0x1e0], rax
140a388c6: 48 c7 85 e8 01 00 00 20 00 00 00    	mov	qword ptr [rbp + 0x1e8], 0x20
140a388d1: 48 8d 05 71 45 09 02        	lea	rax, [rip + 0x2094571]  # 0x142acce49
140a388d8: 48 89 85 f0 01 00 00        	mov	qword ptr [rbp + 0x1f0], rax
140a388df: 48 c7 85 f8 01 00 00 0f 00 00 00    	mov	qword ptr [rbp + 0x1f8], 0xf
140a388ea: 48 8d 05 67 45 09 02        	lea	rax, [rip + 0x2094567]  # 0x142acce58
140a388f1: 48 89 85 00 02 00 00        	mov	qword ptr [rbp + 0x200], rax
140a388f8: 48 c7 85 08 02 00 00 1c 00 00 00    	mov	qword ptr [rbp + 0x208], 0x1c
140a38903: 48 8d 05 6a 45 09 02        	lea	rax, [rip + 0x209456a]  # 0x142acce74
140a3890a: 48 89 85 10 02 00 00        	mov	qword ptr [rbp + 0x210], rax
140a38911: 48 c7 85 18 02 00 00 16 00 00 00    	mov	qword ptr [rbp + 0x218], 0x16
140a3891c: 48 8d 05 67 45 09 02        	lea	rax, [rip + 0x2094567]  # 0x142acce8a
140a38923: 48 89 85 20 02 00 00        	mov	qword ptr [rbp + 0x220], rax
140a3892a: 48 c7 85 28 02 00 00 23 00 00 00    	mov	qword ptr [rbp + 0x228], 0x23
140a38935: 48 8d 05 71 45 09 02        	lea	rax, [rip + 0x2094571]  # 0x142accead
140a3893c: 48 89 85 30 02 00 00        	mov	qword ptr [rbp + 0x230], rax
140a38943: 48 c7 85 38 02 00 00 0a 00 00 00    	mov	qword ptr [rbp + 0x238], 0xa
140a3894e: 48 8d 05 62 45 09 02        	lea	rax, [rip + 0x2094562]  # 0x142acceb7
140a38955: 48 89 85 40 02 00 00        	mov	qword ptr [rbp + 0x240], rax
140a3895c: 48 c7 85 48 02 00 00 17 00 00 00    	mov	qword ptr [rbp + 0x248], 0x17
140a38967: 48 8d 05 60 45 09 02        	lea	rax, [rip + 0x2094560]  # 0x142accece
140a3896e: 48 89 85 50 02 00 00        	mov	qword ptr [rbp + 0x250], rax
140a38975: 48 c7 85 58 02 00 00 0b 00 00 00    	mov	qword ptr [rbp + 0x258], 0xb
140a38980: 48 8d 05 52 45 09 02        	lea	rax, [rip + 0x2094552]  # 0x142acced9
140a38987: 48 89 85 60 02 00 00        	mov	qword ptr [rbp + 0x260], rax
140a3898e: 48 c7 85 68 02 00 00 18 00 00 00    	mov	qword ptr [rbp + 0x268], 0x18
140a38999: 48 8d 05 51 45 09 02        	lea	rax, [rip + 0x2094551]  # 0x142accef1
140a389a0: 48 89 85 70 02 00 00        	mov	qword ptr [rbp + 0x270], rax
140a389a7: 48 c7 85 78 02 00 00 09 00 00 00    	mov	qword ptr [rbp + 0x278], 0x9
140a389b2: 48 8d 05 41 45 09 02        	lea	rax, [rip + 0x2094541]  # 0x142accefa
140a389b9: 48 89 85 80 02 00 00        	mov	qword ptr [rbp + 0x280], rax
140a389c0: 48 c7 85 88 02 00 00 16 00 00 00    	mov	qword ptr [rbp + 0x288], 0x16
140a389cb: 48 8d 05 3e 45 09 02        	lea	rax, [rip + 0x209453e]  # 0x142accf10
140a389d2: 48 89 85 90 02 00 00        	mov	qword ptr [rbp + 0x290], rax
140a389d9: 48 c7 85 98 02 00 00 11 00 00 00    	mov	qword ptr [rbp + 0x298], 0x11
140a389e4: 48 8d 05 36 45 09 02        	lea	rax, [rip + 0x2094536]  # 0x142accf21
140a389eb: 48 89 85 a0 02 00 00        	mov	qword ptr [rbp + 0x2a0], rax
140a389f2: 48 c7 85 a8 02 00 00 1e 00 00 00    	mov	qword ptr [rbp + 0x2a8], 0x1e
140a389fd: 48 8d 05 3b 45 09 02        	lea	rax, [rip + 0x209453b]  # 0x142accf3f
140a38a04: 48 89 85 b0 02 00 00        	mov	qword ptr [rbp + 0x2b0], rax
140a38a0b: 48 c7 85 b8 02 00 00 15 00 00 00    	mov	qword ptr [rbp + 0x2b8], 0x15
140a38a16: 48 8d 05 37 45 09 02        	lea	rax, [rip + 0x2094537]  # 0x142accf54
140a38a1d: 48 89 85 c0 02 00 00        	mov	qword ptr [rbp + 0x2c0], rax
140a38a24: 48 c7 85 c8 02 00 00 22 00 00 00    	mov	qword ptr [rbp + 0x2c8], 0x22
140a38a2f: 48 8d 05 40 45 09 02        	lea	rax, [rip + 0x2094540]  # 0x142accf76
140a38a36: 48 89 85 d0 02 00 00        	mov	qword ptr [rbp + 0x2d0], rax
140a38a3d: 48 c7 85 d8 02 00 00 15 00 00 00    	mov	qword ptr [rbp + 0x2d8], 0x15
140a38a48: 48 8d 05 3c 45 09 02        	lea	rax, [rip + 0x209453c]  # 0x142accf8b
140a38a4f: 48 89 85 e0 02 00 00        	mov	qword ptr [rbp + 0x2e0], rax
140a38a56: 48 c7 85 e8 02 00 00 22 00 00 00    	mov	qword ptr [rbp + 0x2e8], 0x22
140a38a61: 48 8d 05 45 45 09 02        	lea	rax, [rip + 0x2094545]  # 0x142accfad
140a38a68: 48 89 85 f0 02 00 00        	mov	qword ptr [rbp + 0x2f0], rax
140a38a6f: 48 c7 85 f8 02 00 00 1e 00 00 00    	mov	qword ptr [rbp + 0x2f8], 0x1e
140a38a7a: 48 8d 05 4a 45 09 02        	lea	rax, [rip + 0x209454a]  # 0x142accfcb
140a38a81: 48 89 85 00 03 00 00        	mov	qword ptr [rbp + 0x300], rax
140a38a88: 48 c7 85 08 03 00 00 2b 00 00 00    	mov	qword ptr [rbp + 0x308], 0x2b
140a38a93: 48 8d 05 5c 45 09 02        	lea	rax, [rip + 0x209455c]  # 0x142accff6
140a38a9a: 48 89 85 10 03 00 00        	mov	qword ptr [rbp + 0x310], rax
140a38aa1: 48 c7 85 18 03 00 00 09 00 00 00    	mov	qword ptr [rbp + 0x318], 0x9
140a38aac: 48 8d 05 4c 45 09 02        	lea	rax, [rip + 0x209454c]  # 0x142accfff
140a38ab3: 48 89 85 20 03 00 00        	mov	qword ptr [rbp + 0x320], rax
140a38aba: 48 c7 85 28 03 00 00 1f 00 00 00    	mov	qword ptr [rbp + 0x328], 0x1f
140a38ac5: 48 8d 05 52 45 09 02        	lea	rax, [rip + 0x2094552]  # 0x142acd01e
140a38acc: 48 89 85 30 03 00 00        	mov	qword ptr [rbp + 0x330], rax
140a38ad3: 48 c7 85 38 03 00 00 15 00 00 00    	mov	qword ptr [rbp + 0x338], 0x15
140a38ade: 48 8d 05 4e 45 09 02        	lea	rax, [rip + 0x209454e]  # 0x142acd033
140a38ae5: 48 89 85 40 03 00 00        	mov	qword ptr [rbp + 0x340], rax
140a38aec: 48 c7 85 48 03 00 00 22 00 00 00    	mov	qword ptr [rbp + 0x348], 0x22
140a38af7: 48 8d 05 57 45 09 02        	lea	rax, [rip + 0x2094557]  # 0x142acd055
140a38afe: 48 89 85 50 03 00 00        	mov	qword ptr [rbp + 0x350], rax
140a38b05: 48 c7 85 58 03 00 00 0c 00 00 00    	mov	qword ptr [rbp + 0x358], 0xc
140a38b10: 48 8d 05 4a 45 09 02        	lea	rax, [rip + 0x209454a]  # 0x142acd061
140a38b17: 48 89 85 60 03 00 00        	mov	qword ptr [rbp + 0x360], rax
140a38b1e: 48 c7 85 68 03 00 00 19 00 00 00    	mov	qword ptr [rbp + 0x368], 0x19
140a38b29: 48 8d 05 4a 45 09 02        	lea	rax, [rip + 0x209454a]  # 0x142acd07a
140a38b30: 48 89 85 70 03 00 00        	mov	qword ptr [rbp + 0x370], rax
140a38b37: 48 c7 85 78 03 00 00 0a 00 00 00    	mov	qword ptr [rbp + 0x378], 0xa
140a38b42: 48 8d 05 3b 45 09 02        	lea	rax, [rip + 0x209453b]  # 0x142acd084
140a38b49: 48 89 85 80 03 00 00        	mov	qword ptr [rbp + 0x380], rax
140a38b50: 48 c7 85 88 03 00 00 17 00 00 00    	mov	qword ptr [rbp + 0x388], 0x17
140a38b5b: 48 8d 05 39 45 09 02        	lea	rax, [rip + 0x2094539]  # 0x142acd09b
140a38b62: 48 89 85 90 03 00 00        	mov	qword ptr [rbp + 0x390], rax
140a38b69: 48 c7 85 98 03 00 00 0a 00 00 00    	mov	qword ptr [rbp + 0x398], 0xa
140a38b74: 48 8d 05 2a 45 09 02        	lea	rax, [rip + 0x209452a]  # 0x142acd0a5
140a38b7b: 48 89 85 a0 03 00 00        	mov	qword ptr [rbp + 0x3a0], rax
140a38b82: 48 c7 85 a8 03 00 00 17 00 00 00    	mov	qword ptr [rbp + 0x3a8], 0x17
140a38b8d: 48 8d 05 28 45 09 02        	lea	rax, [rip + 0x2094528]  # 0x142acd0bc
140a38b94: 48 89 85 b0 03 00 00        	mov	qword ptr [rbp + 0x3b0], rax
140a38b9b: 48 c7 85 b8 03 00 00 11 00 00 00    	mov	qword ptr [rbp + 0x3b8], 0x11
140a38ba6: 48 8d 05 20 45 09 02        	lea	rax, [rip + 0x2094520]  # 0x142acd0cd
140a38bad: 48 89 85 c0 03 00 00        	mov	qword ptr [rbp + 0x3c0], rax
140a38bb4: 48 c7 85 c8 03 00 00 1e 00 00 00    	mov	qword ptr [rbp + 0x3c8], 0x1e
140a38bbf: 48 8d 05 25 45 09 02        	lea	rax, [rip + 0x2094525]  # 0x142acd0eb
140a38bc6: 48 89 85 d0 03 00 00        	mov	qword ptr [rbp + 0x3d0], rax
140a38bcd: 48 c7 85 d8 03 00 00 09 00 00 00    	mov	qword ptr [rbp + 0x3d8], 0x9
140a38bd8: 48 8d 05 15 45 09 02        	lea	rax, [rip + 0x2094515]  # 0x142acd0f4
140a38bdf: 48 89 85 e0 03 00 00        	mov	qword ptr [rbp + 0x3e0], rax
140a38be6: 48 c7 85 e8 03 00 00 16 00 00 00    	mov	qword ptr [rbp + 0x3e8], 0x16
140a38bf1: 48 8d 05 12 45 09 02        	lea	rax, [rip + 0x2094512]  # 0x142acd10a
140a38bf8: 48 89 85 f0 03 00 00        	mov	qword ptr [rbp + 0x3f0], rax
140a38bff: 48 c7 85 f8 03 00 00 12 00 00 00    	mov	qword ptr [rbp + 0x3f8], 0x12
140a38c0a: 48 8d 05 0b 45 09 02        	lea	rax, [rip + 0x209450b]  # 0x142acd11c
140a38c11: 48 89 85 00 04 00 00        	mov	qword ptr [rbp + 0x400], rax
140a38c18: 48 c7 85 08 04 00 00 1f 00 00 00    	mov	qword ptr [rbp + 0x408], 0x1f
140a38c23: 48 8d 05 11 45 09 02        	lea	rax, [rip + 0x2094511]  # 0x142acd13b
140a38c2a: 48 89 85 10 04 00 00        	mov	qword ptr [rbp + 0x410], rax
140a38c31: 48 c7 85 18 04 00 00 11 00 00 00    	mov	qword ptr [rbp + 0x418], 0x11
140a38c3c: 48 8d 05 09 45 09 02        	lea	rax, [rip + 0x2094509]  # 0x142acd14c
140a38c43: 48 89 85 20 04 00 00        	mov	qword ptr [rbp + 0x420], rax
140a38c4a: 48 c7 85 28 04 00 00 1e 00 00 00    	mov	qword ptr [rbp + 0x428], 0x1e
140a38c55: 48 c7 85 38 04 00 00 1b 00 00 00    	mov	qword ptr [rbp + 0x438], 0x1b
140a38c60: 4d 8b 30                    	mov	r14, qword ptr [r8]
140a38c63: 48 8b 02                    	mov	rax, qword ptr [rdx]
140a38c66: 48 89 85 58 04 00 00        	mov	qword ptr [rbp + 0x458], rax
140a38c6d: 41 bf 01 00 00 00           	mov	r15d, 0x1
140a38c73: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140a38c80: 4c 89 bd 30 04 00 00        	mov	qword ptr [rbp + 0x430], r15
140a38c87: 4c 8b 4b e8                 	mov	r9, qword ptr [rbx - 0x18]
140a38c8b: 4d 85 c9                    	test	r9, r9
140a38c8e: 74 3a                       	je	0x140a38cca <.text+0xa37cca>
140a38c90: 4c 8b 03                    	mov	r8, qword ptr [rbx]
140a38c93: 48 8b 43 f0                 	mov	rax, qword ptr [rbx - 0x10]
140a38c97: 48 8b 53 f8                 	mov	rdx, qword ptr [rbx - 0x8]
140a38c9b: 48 8b 8d 58 04 00 00        	mov	rcx, qword ptr [rbp + 0x458]
140a38ca2: 48 89 4c 24 30              	mov	qword ptr [rsp + 0x30], rcx
140a38ca7: 4c 89 74 24 28              	mov	qword ptr [rsp + 0x28], r14
140a38cac: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
140a38cb1: 48 8b 8d 88 04 00 00        	mov	rcx, qword ptr [rbp + 0x488]
140a38cb8: e8 83 71 10 00              	call	0x140b3fe40 <.text+0xb3ee40>
140a38cbd: 48 83 c3 20                 	add	rbx, 0x20
140a38cc1: 49 ff c7                    	inc	r15
140a38cc4: 49 83 ff 1c                 	cmp	r15, 0x1c
140a38cc8: 75 b6                       	jne	0x140a38c80 <.text+0xa37c80>
140a38cca: 4c 8b 37                    	mov	r14, qword ptr [rdi]
140a38ccd: 8b 46 1c                    	mov	eax, dword ptr [rsi + 0x1c]
140a38cd0: 89 85 98 04 00 00           	mov	dword ptr [rbp + 0x498], eax
140a38cd6: 4c 8b 3e                    	mov	r15, qword ptr [rsi]
140a38cd9: 48 8b 46 10                 	mov	rax, qword ptr [rsi + 0x10]
140a38cdd: 48 89 85 70 04 00 00        	mov	qword ptr [rbp + 0x470], rax
140a38ce4: 49 8d 87 58 14 00 00        	lea	rax, [r15 + 0x1458]
140a38ceb: 48 89 85 50 04 00 00        	mov	qword ptr [rbp + 0x450], rax
140a38cf2: 31 c0                       	xor	eax, eax
140a38cf4: 44 0f b6 e8                 	movzx	r13d, al
140a38cf8: 48 8d 0d 09 4d 09 02        	lea	rcx, [rip + 0x2094d09]  # 0x142acda08
140a38cff: 4a 63 04 a9                 	movsxd	rax, dword ptr [rcx + 4*r13]
140a38d03: 48 01 c8                    	add	rax, rcx
140a38d06: 48 8b 9d 88 04 00 00        	mov	rbx, qword ptr [rbp + 0x488]
140a38d0d: 4c 89 b5 78 04 00 00        	mov	qword ptr [rbp + 0x478], r14
140a38d14: 4c 89 bd 80 04 00 00        	mov	qword ptr [rbp + 0x480], r15
140a38d1b: ff e0                       	jmp	rax
140a38d1d: 44 88 ad 9e 04 00 00        	mov	byte ptr [rbp + 0x49e], r13b
140a38d24: 48 8d 3d c5 14 08 02        	lea	rdi, [rip + 0x20814c5]  # 0x142aba1f0
140a38d2b: 41 bc 0b 00 00 00           	mov	r12d, 0xb
140a38d31: c6 85 9f 04 00 00 01        	mov	byte ptr [rbp + 0x49f], 0x1
140a38d38: e9 2f 03 00 00              	jmp	0x140a3906c <.text+0xa3806c>
140a38d3d: 44 88 ad 9e 04 00 00        	mov	byte ptr [rbp + 0x49e], r13b
140a38d44: 48 8d 3d b0 14 08 02        	lea	rdi, [rip + 0x20814b0]  # 0x142aba1fb
140a38d4b: 41 bc 0a 00 00 00           	mov	r12d, 0xa
140a38d51: c6 85 9f 04 00 00 02        	mov	byte ptr [rbp + 0x49f], 0x2
140a38d58: e9 0f 03 00 00              	jmp	0x140a3906c <.text+0xa3806c>
140a38d5d: 44 88 ad 9e 04 00 00        	mov	byte ptr [rbp + 0x49e], r13b
140a38d64: 48 8d 3d 9a 14 08 02        	lea	rdi, [rip + 0x208149a]  # 0x142aba205
140a38d6b: 41 bc 20 00 00 00           	mov	r12d, 0x20
140a38d71: c6 85 9f 04 00 00 03        	mov	byte ptr [rbp + 0x49f], 0x3
140a38d78: e9 ef 02 00 00              	jmp	0x140a3906c <.text+0xa3806c>
140a38d7d: 44 88 ad 9e 04 00 00        	mov	byte ptr [rbp + 0x49e], r13b
140a38d84: 48 8d 3d 9a 14 08 02        	lea	rdi, [rip + 0x208149a]  # 0x142aba225
140a38d8b: 41 bc 17 00 00 00           	mov	r12d, 0x17
140a38d91: c6 85 9f 04 00 00 04        	mov	byte ptr [rbp + 0x49f], 0x4
140a38d98: e9 cf 02 00 00              	jmp	0x140a3906c <.text+0xa3806c>
140a38d9d: 44 88 ad 9e 04 00 00        	mov	byte ptr [rbp + 0x49e], r13b
140a38da4: 48 8d 3d 91 14 08 02        	lea	rdi, [rip + 0x2081491]  # 0x142aba23c
140a38dab: 41 bc 1e 00 00 00           	mov	r12d, 0x1e
140a38db1: c6 85 9f 04 00 00 05        	mov	byte ptr [rbp + 0x49f], 0x5
140a38db8: e9 af 02 00 00              	jmp	0x140a3906c <.text+0xa3806c>
140a38dbd: 44 88 ad 9e 04 00 00        	mov	byte ptr [rbp + 0x49e], r13b
140a38dc4: 48 8d 3d 8f 14 08 02        	lea	rdi, [rip + 0x208148f]  # 0x142aba25a
140a38dcb: 41 bc 24 00 00 00           	mov	r12d, 0x24
140a38dd1: c6 85 9f 04 00 00 06        	mov	byte ptr [rbp + 0x49f], 0x6
140a38dd8: e9 8f 02 00 00              	jmp	0x140a3906c <.text+0xa3806c>
140a38ddd: 44 88 ad 9e 04 00 00        	mov	byte ptr [rbp + 0x49e], r13b
140a38de4: 48 8d 3d 93 14 08 02        	lea	rdi, [rip + 0x2081493]  # 0x142aba27e
140a38deb: 41 bc 1e 00 00 00           	mov	r12d, 0x1e
140a38df1: c6 85 9f 04 00 00 07        	mov	byte ptr [rbp + 0x49f], 0x7
140a38df8: e9 6f 02 00 00              	jmp	0x140a3906c <.text+0xa3806c>
140a38dfd: 44 88 ad 9e 04 00 00        	mov	byte ptr [rbp + 0x49e], r13b
140a38e04: 48 8d 3d 91 14 08 02        	lea	rdi, [rip + 0x2081491]  # 0x142aba29c
140a38e0b: 41 bc 19 00 00 00           	mov	r12d, 0x19
140a38e11: c6 85 9f 04 00 00 08        	mov	byte ptr [rbp + 0x49f], 0x8
140a38e18: e9 4f 02 00 00              	jmp	0x140a3906c <.text+0xa3806c>
140a38e1d: 44 88 ad 9e 04 00 00        	mov	byte ptr [rbp + 0x49e], r13b
140a38e24: 48 8d 3d 8a 14 08 02        	lea	rdi, [rip + 0x208148a]  # 0x142aba2b5
140a38e2b: 41 bc 13 00 00 00           	mov	r12d, 0x13
140a38e31: c6 85 9f 04 00 00 09        	mov	byte ptr [rbp + 0x49f], 0x9
140a38e38: e9 2f 02 00 00              	jmp	0x140a3906c <.text+0xa3806c>
140a38e3d: 44 88 ad 9e 04 00 00        	mov	byte ptr [rbp + 0x49e], r13b
140a38e44: 48 8d 3d 7d 14 08 02        	lea	rdi, [rip + 0x208147d]  # 0x142aba2c8
140a38e4b: 41 bc 0f 00 00 00           	mov	r12d, 0xf
140a38e51: c6 85 9f 04 00 00 0a        	mov	byte ptr [rbp + 0x49f], 0xa
140a38e58: e9 0f 02 00 00              	jmp	0x140a3906c <.text+0xa3806c>
140a38e5d: 44 88 ad 9e 04 00 00        	mov	byte ptr [rbp + 0x49e], r13b
140a38e64: 48 8d 3d 6c 14 08 02        	lea	rdi, [rip + 0x208146c]  # 0x142aba2d7
140a38e6b: 41 bc 0a 00 00 00           	mov	r12d, 0xa
140a38e71: c6 85 9f 04 00 00 0b        	mov	byte ptr [rbp + 0x49f], 0xb
140a38e78: e9 ef 01 00 00              	jmp	0x140a3906c <.text+0xa3806c>
140a38e7d: 44 88 ad 9e 04 00 00        	mov	byte ptr [rbp + 0x49e], r13b
140a38e84: 48 8d 3d 56 14 08 02        	lea	rdi, [rip + 0x2081456]  # 0x142aba2e1
140a38e8b: 41 bc 0b 00 00 00           	mov	r12d, 0xb
140a38e91: c6 85 9f 04 00 00 0c        	mov	byte ptr [rbp + 0x49f], 0xc
140a38e98: e9 cf 01 00 00              	jmp	0x140a3906c <.text+0xa3806c>
140a38e9d: 44 88 ad 9e 04 00 00        	mov	byte ptr [rbp + 0x49e], r13b
140a38ea4: 48 8d 3d 41 14 08 02        	lea	rdi, [rip + 0x2081441]  # 0x142aba2ec
140a38eab: 41 bc 09 00 00 00           	mov	r12d, 0x9
140a38eb1: c6 85 9f 04 00 00 0d        	mov	byte ptr [rbp + 0x49f], 0xd
140a38eb8: e9 af 01 00 00              	jmp	0x140a3906c <.text+0xa3806c>
140a38ebd: 44 88 ad 9e 04 00 00        	mov	byte ptr [rbp + 0x49e], r13b
140a38ec4: 48 8d 3d 2a 14 08 02        	lea	rdi, [rip + 0x208142a]  # 0x142aba2f5
140a38ecb: 41 bc 11 00 00 00           	mov	r12d, 0x11
140a38ed1: c6 85 9f 04 00 00 0e        	mov	byte ptr [rbp + 0x49f], 0xe
140a38ed8: e9 8f 01 00 00              	jmp	0x140a3906c <.text+0xa3806c>
140a38edd: 44 88 ad 9e 04 00 00        	mov	byte ptr [rbp + 0x49e], r13b
140a38ee4: 48 8d 3d 1b 14 08 02        	lea	rdi, [rip + 0x208141b]  # 0x142aba306
140a38eeb: 41 bc 15 00 00 00           	mov	r12d, 0x15
140a38ef1: c6 85 9f 04 00 00 0f        	mov	byte ptr [rbp + 0x49f], 0xf
140a38ef8: e9 6f 01 00 00              	jmp	0x140a3906c <.text+0xa3806c>
140a38efd: 44 88 ad 9e 04 00 00        	mov	byte ptr [rbp + 0x49e], r13b
140a38f04: 48 8d 3d 10 14 08 02        	lea	rdi, [rip + 0x2081410]  # 0x142aba31b
140a38f0b: 41 bc 15 00 00 00           	mov	r12d, 0x15
140a38f11: c6 85 9f 04 00 00 10        	mov	byte ptr [rbp + 0x49f], 0x10
140a38f18: e9 4f 01 00 00              	jmp	0x140a3906c <.text+0xa3806c>
140a38f1d: 44 88 ad 9e 04 00 00        	mov	byte ptr [rbp + 0x49e], r13b
140a38f24: 48 8d 3d 05 14 08 02        	lea	rdi, [rip + 0x2081405]  # 0x142aba330
140a38f2b: 41 bc 1e 00 00 00           	mov	r12d, 0x1e
140a38f31: c6 85 9f 04 00 00 11        	mov	byte ptr [rbp + 0x49f], 0x11
140a38f38: e9 2f 01 00 00              	jmp	0x140a3906c <.text+0xa3806c>
140a38f3d: 44 88 ad 9e 04 00 00        	mov	byte ptr [rbp + 0x49e], r13b
140a38f44: 48 8d 3d 03 14 08 02        	lea	rdi, [rip + 0x2081403]  # 0x142aba34e
140a38f4b: 41 bc 09 00 00 00           	mov	r12d, 0x9
140a38f51: c6 85 9f 04 00 00 12        	mov	byte ptr [rbp + 0x49f], 0x12
140a38f58: e9 0f 01 00 00              	jmp	0x140a3906c <.text+0xa3806c>
140a38f5d: 44 88 ad 9e 04 00 00        	mov	byte ptr [rbp + 0x49e], r13b
140a38f64: 48 8d 3d ec 13 08 02        	lea	rdi, [rip + 0x20813ec]  # 0x142aba357
140a38f6b: 41 bc 15 00 00 00           	mov	r12d, 0x15
140a38f71: c6 85 9f 04 00 00 13        	mov	byte ptr [rbp + 0x49f], 0x13
140a38f78: e9 ef 00 00 00              	jmp	0x140a3906c <.text+0xa3806c>
140a38f7d: 44 88 ad 9e 04 00 00        	mov	byte ptr [rbp + 0x49e], r13b
140a38f84: 48 8d 3d e1 13 08 02        	lea	rdi, [rip + 0x20813e1]  # 0x142aba36c
140a38f8b: 41 bc 0c 00 00 00           	mov	r12d, 0xc
140a38f91: c6 85 9f 04 00 00 14        	mov	byte ptr [rbp + 0x49f], 0x14
140a38f98: e9 cf 00 00 00              	jmp	0x140a3906c <.text+0xa3806c>
140a38f9d: 44 88 ad 9e 04 00 00        	mov	byte ptr [rbp + 0x49e], r13b
140a38fa4: 48 8d 3d cd 13 08 02        	lea	rdi, [rip + 0x20813cd]  # 0x142aba378
140a38fab: 41 bc 0a 00 00 00           	mov	r12d, 0xa
140a38fb1: c6 85 9f 04 00 00 15        	mov	byte ptr [rbp + 0x49f], 0x15
140a38fb8: e9 af 00 00 00              	jmp	0x140a3906c <.text+0xa3806c>
140a38fbd: 44 88 ad 9e 04 00 00        	mov	byte ptr [rbp + 0x49e], r13b
140a38fc4: 48 8d 3d b7 13 08 02        	lea	rdi, [rip + 0x20813b7]  # 0x142aba382
140a38fcb: 41 bc 0a 00 00 00           	mov	r12d, 0xa
140a38fd1: c6 85 9f 04 00 00 16        	mov	byte ptr [rbp + 0x49f], 0x16
140a38fd8: e9 8f 00 00 00              	jmp	0x140a3906c <.text+0xa3806c>
140a38fdd: 44 88 ad 9e 04 00 00        	mov	byte ptr [rbp + 0x49e], r13b
140a38fe4: 48 8d 3d a1 13 08 02        	lea	rdi, [rip + 0x20813a1]  # 0x142aba38c
140a38feb: 41 bc 11 00 00 00           	mov	r12d, 0x11
140a38ff1: c6 85 9f 04 00 00 17        	mov	byte ptr [rbp + 0x49f], 0x17
140a38ff8: eb 72                       	jmp	0x140a3906c <.text+0xa3806c>
140a38ffa: 44 88 ad 9e 04 00 00        	mov	byte ptr [rbp + 0x49e], r13b
140a39001: 48 8d 3d 95 13 08 02        	lea	rdi, [rip + 0x2081395]  # 0x142aba39d
140a39008: 41 bc 09 00 00 00           	mov	r12d, 0x9
140a3900e: c6 85 9f 04 00 00 18        	mov	byte ptr [rbp + 0x49f], 0x18
140a39015: eb 55                       	jmp	0x140a3906c <.text+0xa3806c>
140a39017: 44 88 ad 9e 04 00 00        	mov	byte ptr [rbp + 0x49e], r13b
140a3901e: 48 8d 3d 81 13 08 02        	lea	rdi, [rip + 0x2081381]  # 0x142aba3a6
140a39025: 41 bc 16 00 00 00           	mov	r12d, 0x16
140a3902b: c6 85 9f 04 00 00 19        	mov	byte ptr [rbp + 0x49f], 0x19
140a39032: eb 38                       	jmp	0x140a3906c <.text+0xa3806c>
140a39034: 44 88 ad 9e 04 00 00        	mov	byte ptr [rbp + 0x49e], r13b
140a3903b: 48 8d 3d 7a 13 08 02        	lea	rdi, [rip + 0x208137a]  # 0x142aba3bc
140a39042: 41 bc 12 00 00 00           	mov	r12d, 0x12
140a39048: c6 85 9f 04 00 00 1a        	mov	byte ptr [rbp + 0x49f], 0x1a
140a3904f: eb 1b                       	jmp	0x140a3906c <.text+0xa3806c>
140a39051: 44 88 ad 9e 04 00 00        	mov	byte ptr [rbp + 0x49e], r13b
140a39058: 48 8d 3d 6f 13 08 02        	lea	rdi, [rip + 0x208136f]  # 0x142aba3ce
140a3905f: 41 bc 11 00 00 00           	mov	r12d, 0x11
140a39065: c6 85 9f 04 00 00 1b        	mov	byte ptr [rbp + 0x49f], 0x1b
140a3906c: 4c 89 f1                    	mov	rcx, r14
140a3906f: 44 89 ea                    	mov	edx, r13d
140a39072: e8 d9 d6 0a 00              	call	0x140ae6750 <.text+0xae5750>
140a39077: 48 89 c6                    	mov	rsi, rax
140a3907a: 48 8b 8d 58 04 00 00        	mov	rcx, qword ptr [rbp + 0x458]
140a39081: 48 89 fa                    	mov	rdx, rdi
140a39084: 4d 89 e0                    	mov	r8, r12
140a39087: e8 14 ea ef ff              	call	0x140937aa0 <.text+0x936aa0>
140a3908c: a8 01                       	test	al, 0x1
140a3908e: 0f 84 8c 05 00 00           	je	0x140a39620 <.text+0xa38620>
140a39094: 83 3e 01                    	cmp	dword ptr [rsi], 0x1
140a39097: 0f 85 13 01 00 00           	jne	0x140a391b0 <.text+0xa381b0>
140a3909d: 48 89 d7                    	mov	rdi, rdx
140a390a0: 4c 8b 7e 08                 	mov	r15, qword ptr [rsi + 0x8]
140a390a4: 48 8d 8d d0 00 00 00        	lea	rcx, [rbp + 0xd0]
140a390ab: 48 8b 95 20 05 00 00        	mov	rdx, qword ptr [rbp + 0x520]
140a390b2: e8 e9 b5 2d 00              	call	0x140d146a0 <.text+0xd136a0>
140a390b7: 4c 8b a5 d8 00 00 00        	mov	r12, qword ptr [rbp + 0xd8]
140a390be: 4c 8b b5 f0 00 00 00        	mov	r14, qword ptr [rbp + 0xf0]
140a390c5: 4d 85 e4                    	test	r12, r12
140a390c8: 75 07                       	jne	0x140a390d1 <.text+0xa380d1>
140a390ca: 4c 8b a5 e0 00 00 00        	mov	r12, qword ptr [rbp + 0xe0]
140a390d1: 49 8b 04 24                 	mov	rax, qword ptr [r12]
140a390d5: 49 8b 5c 24 10              	mov	rbx, qword ptr [r12 + 0x10]
140a390da: 48 29 d8                    	sub	rax, rbx
140a390dd: 48 83 f8 77                 	cmp	rax, 0x77
140a390e1: 0f 86 14 05 00 00           	jbe	0x140a395fb <.text+0xa385fb>
140a390e7: 49 8b 44 24 08              	mov	rax, qword ptr [r12 + 0x8]
140a390ec: 48 8d 0d bd d8 89 00        	lea	rcx, [rip + 0x89d8bd]   # 0x1412d69b0 <.text+0x12d59b0>
140a390f3: 48 89 0c 18                 	mov	qword ptr [rax + rbx], rcx
140a390f7: 0f 28 05 c2 4f f0 01        	movaps	xmm0, xmmword ptr [rip + 0x1f04fc2] # 0x14293e0c0
140a390fe: 0f 11 44 18 08              	movups	xmmword ptr [rax + rbx + 0x8], xmm0
140a39103: 48 c7 44 18 18 00 00 00 00  	mov	qword ptr [rax + rbx + 0x18], 0x0
140a3910c: c7 44 18 20 00 00 00 00     	mov	dword ptr [rax + rbx + 0x20], 0x0
140a39114: 48 b9 00 00 80 3f 00 00 80 3f       	movabs	rcx, 0x3f8000003f800000
140a3911e: 48 89 4c 18 24              	mov	qword ptr [rax + rbx + 0x24], rcx
140a39123: c7 44 18 2c 00 00 80 3f     	mov	dword ptr [rax + rbx + 0x2c], 0x3f800000
140a3912b: 4c 89 7c 18 38              	mov	qword ptr [rax + rbx + 0x38], r15
140a39130: 48 89 7c 18 40              	mov	qword ptr [rax + rbx + 0x40], rdi
140a39135: c6 44 18 48 00              	mov	byte ptr [rax + rbx + 0x48], 0x0
140a3913a: 4c 89 74 18 58              	mov	qword ptr [rax + rbx + 0x58], r14
140a3913f: 48 8d 0d ea bb b2 00        	lea	rcx, [rip + 0xb2bbea]   # 0x141564d30 <.text+0x1563d30>
140a39146: 48 89 4c 18 68              	mov	qword ptr [rax + rbx + 0x68], rcx
140a3914b: 48 83 c3 78                 	add	rbx, 0x78
140a3914f: 49 89 5c 24 10              	mov	qword ptr [r12 + 0x10], rbx
140a39154: 48 8d 05 c5 f7 b2 00        	lea	rax, [rip + 0xb2f7c5]   # 0x141568920 <.text+0x1567920>
140a3915b: 41 0f b6 cd                 	movzx	ecx, r13b
140a3915f: 48 8d 15 12 49 09 02        	lea	rdx, [rip + 0x2094912]  # 0x142acda78
140a39166: 4c 63 04 8a                 	movsxd	r8, dword ptr [rdx + 4*rcx]
140a3916a: 49 01 d0                    	add	r8, rdx
140a3916d: 41 ff e0                    	jmp	r8
140a39170: 48 8d 05 29 81 f5 ff        	lea	rax, [rip - 0xa7ed7]    # 0x1409912a0 <.text+0x9902a0>
140a39177: 4c 8b 86 98 00 00 00        	mov	r8, qword ptr [rsi + 0x98]
140a3917e: 41 b9 00 20 00 00           	mov	r9d, 0x2000
140a39184: 48 8d 15 59 49 09 02        	lea	rdx, [rip + 0x2094959]  # 0x142acdae4
140a3918b: 48 63 0c 8a                 	movsxd	rcx, dword ptr [rdx + 4*rcx]
140a3918f: 48 01 d1                    	add	rcx, rdx
140a39192: 48 8b 9d 88 04 00 00        	mov	rbx, qword ptr [rbp + 0x488]
140a39199: 4c 8b b5 78 04 00 00        	mov	r14, qword ptr [rbp + 0x478]
140a391a0: 4c 8b bd 80 04 00 00        	mov	r15, qword ptr [rbp + 0x480]
140a391a7: 44 8b a5 98 04 00 00        	mov	r12d, dword ptr [rbp + 0x498]
140a391ae: ff e1                       	jmp	rcx
140a391b0: 48 b8 01 00 00 00 00 00 00 80       	movabs	rax, -0x7fffffffffffffff
140a391ba: 48 89 45 c8                 	mov	qword ptr [rbp - 0x38], rax
140a391be: 44 8b a5 98 04 00 00        	mov	r12d, dword ptr [rbp + 0x498]
140a391c5: e9 42 01 00 00              	jmp	0x140a3930c <.text+0xa3830c>
140a391ca: 48 8d 05 df e1 b2 00        	lea	rax, [rip + 0xb2e1df]   # 0x1415673b0 <.text+0x15663b0>
140a391d1: 4c 8b 86 98 00 00 00        	mov	r8, qword ptr [rsi + 0x98]
140a391d8: 41 b9 00 20 00 00           	mov	r9d, 0x2000
140a391de: 48 8d 15 ff 48 09 02        	lea	rdx, [rip + 0x20948ff]  # 0x142acdae4
140a391e5: 48 63 0c 8a                 	movsxd	rcx, dword ptr [rdx + 4*rcx]
140a391e9: 48 01 d1                    	add	rcx, rdx
140a391ec: 48 8b 9d 88 04 00 00        	mov	rbx, qword ptr [rbp + 0x488]
140a391f3: 4c 8b b5 78 04 00 00        	mov	r14, qword ptr [rbp + 0x478]
140a391fa: 4c 8b bd 80 04 00 00        	mov	r15, qword ptr [rbp + 0x480]
140a39201: 44 8b a5 98 04 00 00        	mov	r12d, dword ptr [rbp + 0x498]
140a39208: ff e1                       	jmp	rcx
140a3920a: 41 b9 00 10 00 00           	mov	r9d, 0x1000
140a39210: e9 ee 00 00 00              	jmp	0x140a39303 <.text+0xa38303>
140a39215: 4c 8b 86 98 00 00 00        	mov	r8, qword ptr [rsi + 0x98]
140a3921c: 41 b9 00 01 00 00           	mov	r9d, 0x100
140a39222: 48 8d 05 c7 0c b3 00        	lea	rax, [rip + 0xb30cc7]   # 0x141569ef0 <.text+0x1568ef0>
140a39229: e9 b9 00 00 00              	jmp	0x140a392e7 <.text+0xa382e7>
140a3922e: 48 8d 05 7b 17 b3 00        	lea	rax, [rip + 0xb3177b]   # 0x14156a9b0 <.text+0x15699b0>
140a39235: e9 a0 00 00 00              	jmp	0x140a392da <.text+0xa382da>
140a3923a: 4c 8b 86 98 00 00 00        	mov	r8, qword ptr [rsi + 0x98]
140a39241: 41 b9 00 40 01 00           	mov	r9d, 0x14000
140a39247: 48 8d 05 a2 d1 b2 00        	lea	rax, [rip + 0xb2d1a2]   # 0x1415663f0 <.text+0x15653f0>
140a3924e: e9 94 00 00 00              	jmp	0x140a392e7 <.text+0xa382e7>
140a39253: 4c 8b 86 98 00 00 00        	mov	r8, qword ptr [rsi + 0x98]
140a3925a: 41 b9 00 04 00 00           	mov	r9d, 0x400
140a39260: 48 8d 05 59 12 b3 00        	lea	rax, [rip + 0xb31259]   # 0x14156a4c0 <.text+0x15694c0>
140a39267: eb 7e                       	jmp	0x140a392e7 <.text+0xa382e7>
140a39269: 48 8d 05 70 06 b3 00        	lea	rax, [rip + 0xb30670]   # 0x1415698e0 <.text+0x15688e0>
140a39270: eb 68                       	jmp	0x140a392da <.text+0xa382da>
140a39272: 4c 8b 86 98 00 00 00        	mov	r8, qword ptr [rsi + 0x98]
140a39279: 41 b9 00 04 00 00           	mov	r9d, 0x400
140a3927f: 48 8d 05 ca 31 b3 00        	lea	rax, [rip + 0xb331ca]   # 0x14156c450 <.text+0x156b450>
140a39286: eb 5f                       	jmp	0x140a392e7 <.text+0xa382e7>
140a39288: 4c 8b 86 98 00 00 00        	mov	r8, qword ptr [rsi + 0x98]
140a3928f: 41 b9 00 08 00 00           	mov	r9d, 0x800
140a39295: 48 8d 05 14 fc b2 00        	lea	rax, [rip + 0xb2fc14]   # 0x141568eb0 <.text+0x1567eb0>
140a3929c: eb 49                       	jmp	0x140a392e7 <.text+0xa382e7>
140a3929e: 4c 8b 86 98 00 00 00        	mov	r8, qword ptr [rsi + 0x98]
140a392a5: 41 b9 00 04 00 00           	mov	r9d, 0x400
140a392ab: 48 8d 05 be db b2 00        	lea	rax, [rip + 0xb2dbbe]   # 0x141566e70 <.text+0x1565e70>
140a392b2: eb 33                       	jmp	0x140a392e7 <.text+0xa382e7>
140a392b4: 48 8d 05 d5 eb b2 00        	lea	rax, [rip + 0xb2ebd5]   # 0x141567e90 <.text+0x1566e90>
140a392bb: eb 1d                       	jmp	0x140a392da <.text+0xa382da>
140a392bd: 4c 8b 86 98 00 00 00        	mov	r8, qword ptr [rsi + 0x98]
140a392c4: 41 b9 00 80 00 00           	mov	r9d, 0x8000
140a392ca: 48 8d 05 2f 2c b3 00        	lea	rax, [rip + 0xb32c2f]   # 0x14156bf00 <.text+0x156af00>
140a392d1: eb 14                       	jmp	0x140a392e7 <.text+0xa382e7>
140a392d3: 48 8d 05 e6 26 b3 00        	lea	rax, [rip + 0xb326e6]   # 0x14156b9c0 <.text+0x156a9c0>
140a392da: 4c 8b 86 98 00 00 00        	mov	r8, qword ptr [rsi + 0x98]
140a392e1: 41 b9 00 20 00 00           	mov	r9d, 0x2000
140a392e7: 48 8b 9d 88 04 00 00        	mov	rbx, qword ptr [rbp + 0x488]
140a392ee: 4c 8b b5 78 04 00 00        	mov	r14, qword ptr [rbp + 0x478]
140a392f5: 4c 8b bd 80 04 00 00        	mov	r15, qword ptr [rbp + 0x480]
140a392fc: 44 8b a5 98 04 00 00        	mov	r12d, dword ptr [rbp + 0x498]
140a39303: 48 8d 4d c8                 	lea	rcx, [rbp - 0x38]
140a39307: 48 89 da                    	mov	rdx, rbx
140a3930a: ff d0                       	call	rax
140a3930c: 48 8b 85 70 04 00 00        	mov	rax, qword ptr [rbp + 0x470]
140a39313: 44 89 20                    	mov	dword ptr [rax], r12d
140a39316: 48 8b 86 98 00 00 00        	mov	rax, qword ptr [rsi + 0x98]
140a3931d: 41 0f b6 fd                 	movzx	edi, r13b
140a39321: 48 85 c0                    	test	rax, rax
140a39324: 0f 84 80 00 00 00           	je	0x140a393aa <.text+0xa383aa>
140a3932a: 89 f9                       	mov	ecx, edi
140a3932c: c1 e1 0a                    	shl	ecx, 0xa
140a3932f: 48 c1 e0 04                 	shl	rax, 0x4
140a39333: 48 8d 14 08                 	lea	rdx, [rax + rcx]
140a39337: 49 3b 97 70 14 00 00        	cmp	rdx, qword ptr [r15 + 0x1470]
140a3933e: 0f 87 3f 03 00 00           	ja	0x140a39683 <.text+0xa38683>
140a39344: 4c 8b 8e 90 00 00 00        	mov	r9, qword ptr [rsi + 0x90]
140a3934b: 48 c7 85 40 04 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x440], 0x0
140a39356: 48 89 85 48 04 00 00        	mov	qword ptr [rbp + 0x448], rax
140a3935d: c6 85 97 04 00 00 01        	mov	byte ptr [rbp + 0x497], 0x1
140a39364: 48 89 4c 24 38              	mov	qword ptr [rsp + 0x38], rcx
140a39369: 48 8d 8d 40 04 00 00        	lea	rcx, [rbp + 0x440]
140a39370: 48 89 4c 24 28              	mov	qword ptr [rsp + 0x28], rcx
140a39375: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
140a3937a: 48 c7 44 24 30 01 00 00 00  	mov	qword ptr [rsp + 0x30], 0x1
140a39383: 48 8d 8d d0 00 00 00        	lea	rcx, [rbp + 0xd0]
140a3938a: 48 89 da                    	mov	rdx, rbx
140a3938d: 4c 8b 85 50 04 00 00        	mov	r8, qword ptr [rbp + 0x450]
140a39394: e8 07 5c 1b 00              	call	0x140beefa0 <.text+0xbedfa0>
140a39399: 48 8b 85 d0 00 00 00        	mov	rax, qword ptr [rbp + 0xd0]
140a393a0: 48 83 f8 0b                 	cmp	rax, 0xb
140a393a4: 0f 85 e0 02 00 00           	jne	0x140a3968a <.text+0xa3868a>
140a393aa: 48 8b 85 70 04 00 00        	mov	rax, qword ptr [rbp + 0x470]
140a393b1: 44 89 20                    	mov	dword ptr [rax], r12d
140a393b4: 48 8d 8d d0 00 00 00        	lea	rcx, [rbp + 0xd0]
140a393bb: 48 8d 55 c8                 	lea	rdx, [rbp - 0x38]
140a393bf: 41 b8 b8 00 00 00           	mov	r8d, 0xb8
140a393c5: e8 36 a8 ee 01              	call	0x142923c00 <.text+0x2922c00>
140a393ca: 48 8d 04 7f                 	lea	rax, [rdi + 2*rdi]
140a393ce: c1 e0 06                    	shl	eax, 0x6
140a393d1: 49 8d 0c 07                 	lea	rcx, [r15 + rax]
140a393d5: 48 83 c1 08                 	add	rcx, 0x8
140a393d9: 48 89 8d 68 04 00 00        	mov	qword ptr [rbp + 0x468], rcx
140a393e0: 48 b9 02 00 00 00 00 00 00 80       	movabs	rcx, -0x7ffffffffffffffe
140a393ea: 49 39 4c 07 08              	cmp	qword ptr [r15 + rax + 0x8], rcx
140a393ef: 7c 0c                       	jl	0x140a393fd <.text+0xa383fd>
140a393f1: 48 8b 8d 68 04 00 00        	mov	rcx, qword ptr [rbp + 0x468]
140a393f8: e8 13 84 e5 ff              	call	0x140891810 <.text+0x890810>
140a393fd: 48 8d 95 d0 00 00 00        	lea	rdx, [rbp + 0xd0]
140a39404: 41 b8 b8 00 00 00           	mov	r8d, 0xb8
140a3940a: 48 8b 8d 68 04 00 00        	mov	rcx, qword ptr [rbp + 0x468]
140a39411: e8 ea a7 ee 01              	call	0x142923c00 <.text+0x2922c00>
140a39416: 44 0f b6 ad 9f 04 00 00     	movzx	r13d, byte ptr [rbp + 0x49f]
140a3941e: 48 8d 0d e3 45 09 02        	lea	rcx, [rip + 0x20945e3]  # 0x142acda08
140a39425: 4a 63 04 a9                 	movsxd	rax, dword ptr [rcx + 4*r13]
140a39429: 48 01 c8                    	add	rax, rcx
140a3942c: ff e0                       	jmp	rax
140a3942e: 49 8b 87 50 14 00 00        	mov	rax, qword ptr [r15 + 0x1450]
140a39435: 8b 40 60                    	mov	eax, dword ptr [rax + 0x60]
140a39438: 89 c6                       	mov	esi, eax
140a3943a: c1 ee 05                    	shr	esi, 0x5
140a3943d: 83 f8 20                    	cmp	eax, 0x20
140a39440: 0f 83 1f 01 00 00           	jae	0x140a39565 <.text+0xa38565>
140a39446: b8 04 00 00 00              	mov	eax, 0x4
140a3944b: 48 c7 85 70 04 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x470], 0x0
140a39456: e9 33 01 00 00              	jmp	0x140a3958e <.text+0xa3858e>
140a3945b: 4c 8b 86 98 00 00 00        	mov	r8, qword ptr [rsi + 0x98]
140a39462: 41 b9 00 08 00 00           	mov	r9d, 0x800
140a39468: 48 8d 05 f1 c4 b2 00        	lea	rax, [rip + 0xb2c4f1]   # 0x141565960 <.text+0x1564960>
140a3946f: e9 73 fe ff ff              	jmp	0x140a392e7 <.text+0xa382e7>
140a39474: 4c 8b 86 98 00 00 00        	mov	r8, qword ptr [rsi + 0x98]
140a3947b: 41 b9 00 02 00 00           	mov	r9d, 0x200
140a39481: 48 8d 05 68 ff b2 00        	lea	rax, [rip + 0xb2ff68]   # 0x1415693f0 <.text+0x15683f0>
140a39488: e9 5a fe ff ff              	jmp	0x140a392e7 <.text+0xa382e7>
140a3948d: 4c 8b 86 98 00 00 00        	mov	r8, qword ptr [rsi + 0x98]
140a39494: 41 b9 00 80 00 00           	mov	r9d, 0x8000
140a3949a: 48 8d 05 af e4 b2 00        	lea	rax, [rip + 0xb2e4af]   # 0x141567950 <.text+0x1566950>
140a394a1: e9 41 fe ff ff              	jmp	0x140a392e7 <.text+0xa382e7>
140a394a6: 4c 8b 86 98 00 00 00        	mov	r8, qword ptr [rsi + 0x98]
140a394ad: 41 b9 00 00 01 00           	mov	r9d, 0x10000
140a394b3: 48 8d 05 c6 1f b3 00        	lea	rax, [rip + 0xb31fc6]   # 0x14156b480 <.text+0x156a480>
140a394ba: e9 28 fe ff ff              	jmp	0x140a392e7 <.text+0xa382e7>
140a394bf: 4c 8b 86 98 00 00 00        	mov	r8, qword ptr [rsi + 0x98]
140a394c6: 41 b9 00 10 00 00           	mov	r9d, 0x1000
140a394cc: 48 8d 05 6d 1a b3 00        	lea	rax, [rip + 0xb31a6d]   # 0x14156af40 <.text+0x1569f40>
140a394d3: e9 0f fe ff ff              	jmp	0x140a392e7 <.text+0xa382e7>
140a394d8: 4c 8b 86 98 00 00 00        	mov	r8, qword ptr [rsi + 0x98]
140a394df: 41 b9 00 00 02 00           	mov	r9d, 0x20000
140a394e5: 48 8d 05 b4 c9 b2 00        	lea	rax, [rip + 0xb2c9b4]   # 0x141565ea0 <.text+0x1564ea0>
140a394ec: e9 f6 fd ff ff              	jmp	0x140a392e7 <.text+0xa382e7>
140a394f1: 4c 8b 86 98 00 00 00        	mov	r8, qword ptr [rsi + 0x98]
140a394f8: 41 b9 00 a0 00 00           	mov	r9d, 0xa000
140a394fe: 48 8d 05 2b d4 b2 00        	lea	rax, [rip + 0xb2d42b]   # 0x141566930 <.text+0x1565930>
140a39505: e9 dd fd ff ff              	jmp	0x140a392e7 <.text+0xa382e7>
140a3950a: 4c 8b 86 98 00 00 00        	mov	r8, qword ptr [rsi + 0x98]
140a39511: 41 b9 00 80 00 00           	mov	r9d, 0x8000
140a39517: 48 8d 05 c2 ee b2 00        	lea	rax, [rip + 0xb2eec2]   # 0x1415683e0 <.text+0x15673e0>
140a3951e: e9 c4 fd ff ff              	jmp	0x140a392e7 <.text+0xa382e7>
140a39523: 41 b9 80 25 00 00           	mov	r9d, 0x2580
140a39529: e9 d5 fd ff ff              	jmp	0x140a39303 <.text+0xa38303>
140a3952e: 41 b9 00 04 00 00           	mov	r9d, 0x400
140a39534: e9 ca fd ff ff              	jmp	0x140a39303 <.text+0xa38303>
140a39539: 41 b9 80 cf 00 00           	mov	r9d, 0xcf80
140a3953f: e9 bf fd ff ff              	jmp	0x140a39303 <.text+0xa38303>
140a39544: 41 b9 00 00 02 00           	mov	r9d, 0x20000
140a3954a: e9 b4 fd ff ff              	jmp	0x140a39303 <.text+0xa38303>
140a3954f: 41 b9 00 a0 00 00           	mov	r9d, 0xa000
140a39555: e9 a9 fd ff ff              	jmp	0x140a39303 <.text+0xa38303>
140a3955a: 41 b9 00 00 04 00           	mov	r9d, 0x40000
140a39560: e9 9e fd ff ff              	jmp	0x140a39303 <.text+0xa38303>
140a39565: 89 f7                       	mov	edi, esi
140a39567: c1 e7 02                    	shl	edi, 0x2
140a3956a: 0f b6 05 98 b6 bf 02        	movzx	eax, byte ptr [rip + 0x2bfb698] # 0x143634c09
140a39571: ba 04 00 00 00              	mov	edx, 0x4
140a39576: 48 89 f9                    	mov	rcx, rdi
140a39579: e8 c2 a6 bd ff              	call	0x140613c40 <.text+0x612c40>
140a3957e: 48 85 c0                    	test	rax, rax
140a39581: 0f 84 c3 01 00 00           	je	0x140a3974a <.text+0xa3874a>
140a39587: 48 89 b5 70 04 00 00        	mov	qword ptr [rbp + 0x470], rsi
140a3958e: 48 8b 8d 60 04 00 00        	mov	rcx, qword ptr [rbp + 0x460]
140a39595: 48 8b 39                    	mov	rdi, qword ptr [rcx]
140a39598: 48 89 f9                    	mov	rcx, rdi
140a3959b: 48 89 da                    	mov	rdx, rbx
140a3959e: 49 89 c0                    	mov	r8, rax
140a395a1: 48 89 85 68 04 00 00        	mov	qword ptr [rbp + 0x468], rax
140a395a8: 49 89 f1                    	mov	r9, rsi
140a395ab: e8 c0 1f 88 00              	call	0x1412bb570 <.text+0x12ba570>
140a395b0: 48 89 f9                    	mov	rcx, rdi
140a395b3: 48 89 da                    	mov	rdx, rbx
140a395b6: 4c 8b 85 68 04 00 00        	mov	r8, qword ptr [rbp + 0x468]
140a395bd: 49 89 f1                    	mov	r9, rsi
140a395c0: e8 2b 18 88 00              	call	0x1412badf0 <.text+0x12b9df0>
140a395c5: 48 8b 8d 68 04 00 00        	mov	rcx, qword ptr [rbp + 0x468]
140a395cc: 48 8b 95 70 04 00 00        	mov	rdx, qword ptr [rbp + 0x470]
140a395d3: 48 85 d2                    	test	rdx, rdx
140a395d6: 74 0e                       	je	0x140a395e6 <.text+0xa385e6>
140a395d8: c1 e2 02                    	shl	edx, 0x2
140a395db: 41 b8 04 00 00 00           	mov	r8d, 0x4
140a395e1: e8 3a a6 bd ff              	call	0x140613c20 <.text+0x612c20>
140a395e6: 90                          	nop
140a395e7: 48 81 c4 28 05 00 00        	add	rsp, 0x528
140a395ee: 5b                          	pop	rbx
140a395ef: 5f                          	pop	rdi
140a395f0: 5e                          	pop	rsi
140a395f1: 41 5c                       	pop	r12
140a395f3: 41 5d                       	pop	r13
140a395f5: 41 5e                       	pop	r14
140a395f7: 41 5f                       	pop	r15
140a395f9: 5d                          	pop	rbp
140a395fa: c3                          	ret
140a395fb: 48 c7 44 24 20 01 00 00 00  	mov	qword ptr [rsp + 0x20], 0x1
140a39604: 41 b8 78 00 00 00           	mov	r8d, 0x78
140a3960a: 41 b9 01 00 00 00           	mov	r9d, 0x1
140a39610: 4c 89 e1                    	mov	rcx, r12
140a39613: 48 89 da                    	mov	rdx, rbx
140a39616: e8 45 04 e7 01              	call	0x1428a9a60 <.text+0x28a8a60>
140a3961b: e9 c7 fa ff ff              	jmp	0x140a390e7 <.text+0xa380e7>
140a39620: 48 8d 85 9e 04 00 00        	lea	rax, [rbp + 0x49e]
140a39627: 48 89 45 c8                 	mov	qword ptr [rbp - 0x38], rax
140a3962b: 48 8d 05 7e 3c 00 00        	lea	rax, [rip + 0x3c7e]     # 0x140a3d2b0 <.text+0xa3c2b0>
140a39632: 48 89 45 d0                 	mov	qword ptr [rbp - 0x30], rax
140a39636: 48 8d 05 a3 3b 09 02        	lea	rax, [rip + 0x2093ba3]  # 0x142acd1e0
140a3963d: 48 89 85 d0 00 00 00        	mov	qword ptr [rbp + 0xd0], rax
140a39644: 48 c7 85 d8 00 00 00 01 00 00 00    	mov	qword ptr [rbp + 0xd8], 0x1
140a3964f: 48 c7 85 f0 00 00 00 00 00 00 00    	mov	qword ptr [rbp + 0xf0], 0x0
140a3965a: 48 8d 45 c8                 	lea	rax, [rbp - 0x38]
140a3965e: 48 89 85 e0 00 00 00        	mov	qword ptr [rbp + 0xe0], rax
140a39665: 48 c7 85 e8 00 00 00 01 00 00 00    	mov	qword ptr [rbp + 0xe8], 0x1
140a39670: 48 8d 15 79 3b 09 02        	lea	rdx, [rip + 0x2093b79]  # 0x142acd1f0
140a39677: 48 8d 8d d0 00 00 00        	lea	rcx, [rbp + 0xd0]
140a3967e: e8 0d fd e9 01              	call	0x1428d9390 <.text+0x28d8390>
140a39683: b8 0b 00 00 00              	mov	eax, 0xb
140a39688: eb 46                       	jmp	0x140a396d0 <.text+0xa386d0>
140a3968a: 48 8b 8d 18 01 00 00        	mov	rcx, qword ptr [rbp + 0x118]
140a39691: 48 89 8d c0 00 00 00        	mov	qword ptr [rbp + 0xc0], rcx
140a39698: 0f 10 85 d8 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xd8]
140a3969f: 0f 10 8d e8 00 00 00        	movups	xmm1, xmmword ptr [rbp + 0xe8]
140a396a6: 0f 10 95 f8 00 00 00        	movups	xmm2, xmmword ptr [rbp + 0xf8]
140a396ad: 0f 10 9d 08 01 00 00        	movups	xmm3, xmmword ptr [rbp + 0x108]
140a396b4: 0f 29 9d b0 00 00 00        	movaps	xmmword ptr [rbp + 0xb0], xmm3
140a396bb: 0f 29 95 a0 00 00 00        	movaps	xmmword ptr [rbp + 0xa0], xmm2
140a396c2: 0f 29 8d 90 00 00 00        	movaps	xmmword ptr [rbp + 0x90], xmm1
140a396c9: 0f 29 85 80 00 00 00        	movaps	xmmword ptr [rbp + 0x80], xmm0
140a396d0: 48 89 85 d0 00 00 00        	mov	qword ptr [rbp + 0xd0], rax
140a396d7: 0f 28 85 80 00 00 00        	movaps	xmm0, xmmword ptr [rbp + 0x80]
140a396de: 0f 28 8d 90 00 00 00        	movaps	xmm1, xmmword ptr [rbp + 0x90]
140a396e5: 0f 28 95 a0 00 00 00        	movaps	xmm2, xmmword ptr [rbp + 0xa0]
140a396ec: 0f 28 9d b0 00 00 00        	movaps	xmm3, xmmword ptr [rbp + 0xb0]
140a396f3: 0f 11 85 d8 00 00 00        	movups	xmmword ptr [rbp + 0xd8], xmm0
140a396fa: 0f 11 8d e8 00 00 00        	movups	xmmword ptr [rbp + 0xe8], xmm1
140a39701: 0f 11 95 f8 00 00 00        	movups	xmmword ptr [rbp + 0xf8], xmm2
140a39708: 0f 11 9d 08 01 00 00        	movups	xmmword ptr [rbp + 0x108], xmm3
140a3970f: 48 8b 85 c0 00 00 00        	mov	rax, qword ptr [rbp + 0xc0]
140a39716: 48 89 85 18 01 00 00        	mov	qword ptr [rbp + 0x118], rax
140a3971d: 48 8d 05 b4 0d 08 02        	lea	rax, [rip + 0x2080db4]  # 0x142aba4d8
140a39724: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
140a39729: 48 8d 0d 30 0a 08 02        	lea	rcx, [rip + 0x2080a30]  # 0x142aba160
140a39730: 4c 8d 0d 09 0a 08 02        	lea	r9, [rip + 0x2080a09]   # 0x142aba140
140a39737: 4c 8d 85 d0 00 00 00        	lea	r8, [rbp + 0xd0]
140a3973e: ba 2b 00 00 00              	mov	edx, 0x2b
140a39743: e8 18 00 ea 01              	call	0x1428d9760 <.text+0x28d8760>
140a39748: 0f 0b                       	ud2
140a3974a: 4c 8d 05 4f 3a 09 02        	lea	r8, [rip + 0x2093a4f]   # 0x142acd1a0
140a39751: b9 04 00 00 00              	mov	ecx, 0x4
140a39756: 48 89 fa                    	mov	rdx, rdi
140a39759: e8 85 f8 e9 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
140a3975e: cc                          	int3
140a3975f: 90                          	nop
140a39760: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a39765: 55                          	push	rbp
140a39766: 41 57                       	push	r15
140a39768: 41 56                       	push	r14
140a3976a: 41 55                       	push	r13
140a3976c: 41 54                       	push	r12
140a3976e: 56                          	push	rsi
140a3976f: 57                          	push	rdi
140a39770: 53                          	push	rbx
140a39771: 48 83 ec 48                 	sub	rsp, 0x48
140a39775: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a3977c: 48 8b 8d 68 04 00 00        	mov	rcx, qword ptr [rbp + 0x468]
140a39783: 48 83 bd 70 04 00 00 00     	cmp	qword ptr [rbp + 0x470], 0x0
140a3978b: 74 15                       	je	0x140a397a2 <.text+0xa387a2>
140a3978d: 48 8b 95 70 04 00 00        	mov	rdx, qword ptr [rbp + 0x470]
140a39794: c1 e2 02                    	shl	edx, 0x2
140a39797: 41 b8 04 00 00 00           	mov	r8d, 0x4
140a3979d: e8 7e a4 bd ff              	call	0x140613c20 <.text+0x612c20>
140a397a2: 90                          	nop
140a397a3: 48 83 c4 48                 	add	rsp, 0x48
140a397a7: 5b                          	pop	rbx
140a397a8: 5f                          	pop	rdi
140a397a9: 5e                          	pop	rsi
140a397aa: 41 5c                       	pop	r12
140a397ac: 41 5d                       	pop	r13
140a397ae: 41 5e                       	pop	r14
140a397b0: 41 5f                       	pop	r15
140a397b2: 5d                          	pop	rbp
140a397b3: c3                          	ret
140a397b4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
140a397c0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a397c5: 55                          	push	rbp
140a397c6: 41 57                       	push	r15
140a397c8: 41 56                       	push	r14
140a397ca: 41 55                       	push	r13
140a397cc: 41 54                       	push	r12
140a397ce: 56                          	push	rsi
140a397cf: 57                          	push	rdi
140a397d0: 53                          	push	rbx
140a397d1: 48 83 ec 48                 	sub	rsp, 0x48
140a397d5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a397dc: 80 bd 97 04 00 00 00        	cmp	byte ptr [rbp + 0x497], 0x0
140a397e3: 74 19                       	je	0x140a397fe <.text+0xa387fe>
140a397e5: 48 b8 02 00 00 00 00 00 00 80       	movabs	rax, -0x7ffffffffffffffe
140a397ef: 48 39 45 c8                 	cmp	qword ptr [rbp - 0x38], rax
140a397f3: 7c 09                       	jl	0x140a397fe <.text+0xa387fe>
140a397f5: 48 8d 4d c8                 	lea	rcx, [rbp - 0x38]
140a397f9: e8 12 80 e5 ff              	call	0x140891810 <.text+0x890810>
140a397fe: 90                          	nop
140a397ff: 48 83 c4 48                 	add	rsp, 0x48
140a39803: 5b                          	pop	rbx
140a39804: 5f                          	pop	rdi
140a39805: 5e                          	pop	rsi
140a39806: 41 5c                       	pop	r12
140a39808: 41 5d                       	pop	r13
140a3980a: 41 5e                       	pop	r14
140a3980c: 41 5f                       	pop	r15
140a3980e: 5d                          	pop	rbp
140a3980f: c3                          	ret
140a39810: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a39815: 55                          	push	rbp
140a39816: 41 57                       	push	r15
140a39818: 41 56                       	push	r14
140a3981a: 41 55                       	push	r13
140a3981c: 41 54                       	push	r12
140a3981e: 56                          	push	rsi
140a3981f: 57                          	push	rdi
140a39820: 53                          	push	rbx
140a39821: 48 83 ec 48                 	sub	rsp, 0x48
140a39825: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a3982c: c6 85 97 04 00 00 01        	mov	byte ptr [rbp + 0x497], 0x1
140a39833: 48 8d 8d d0 00 00 00        	lea	rcx, [rbp + 0xd0]
140a3983a: e8 d1 7f d7 ff              	call	0x1407b1810 <.text+0x7b0810>
140a3983f: c6 85 97 04 00 00 01        	mov	byte ptr [rbp + 0x497], 0x1
140a39846: 48 83 c4 48                 	add	rsp, 0x48
140a3984a: 5b                          	pop	rbx
140a3984b: 5f                          	pop	rdi
140a3984c: 5e                          	pop	rsi
140a3984d: 41 5c                       	pop	r12
140a3984f: 41 5d                       	pop	r13
140a39851: 41 5e                       	pop	r14
140a39853: 41 5f                       	pop	r15
140a39855: 5d                          	pop	rbp
140a39856: c3                          	ret
140a39857: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
140a39860: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a39865: 55                          	push	rbp
140a39866: 41 57                       	push	r15
140a39868: 41 56                       	push	r14
140a3986a: 41 55                       	push	r13
140a3986c: 41 54                       	push	r12
140a3986e: 56                          	push	rsi
140a3986f: 57                          	push	rdi
140a39870: 53                          	push	rbx
140a39871: 48 83 ec 48                 	sub	rsp, 0x48
140a39875: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a3987c: 48 8d 95 d0 00 00 00        	lea	rdx, [rbp + 0xd0]
140a39883: 41 b8 b8 00 00 00           	mov	r8d, 0xb8
140a39889: 48 8b 8d 68 04 00 00        	mov	rcx, qword ptr [rbp + 0x468]
140a39890: e8 6b a3 ee 01              	call	0x142923c00 <.text+0x2922c00>
140a39895: c6 85 97 04 00 00 00        	mov	byte ptr [rbp + 0x497], 0x0
140a3989c: 48 83 c4 48                 	add	rsp, 0x48
140a398a0: 5b                          	pop	rbx
140a398a1: 5f                          	pop	rdi
140a398a2: 5e                          	pop	rsi
140a398a3: 41 5c                       	pop	r12
140a398a5: 41 5d                       	pop	r13
140a398a7: 41 5e                       	pop	r14
140a398a9: 41 5f                       	pop	r15
140a398ab: 5d                          	pop	rbp
140a398ac: c3                          	ret
