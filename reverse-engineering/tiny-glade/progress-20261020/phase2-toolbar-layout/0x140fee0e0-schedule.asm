
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140fee0e0: 55                          	pushq	%rbp
140fee0e1: 56                          	pushq	%rsi
140fee0e2: 57                          	pushq	%rdi
140fee0e3: 53                          	pushq	%rbx
140fee0e4: 48 81 ec 18 05 00 00        	subq	$0x518, %rsp            # imm = 0x518
140fee0eb: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140fee0f3: 48 c7 85 90 04 00 00 fe ff ff ff    	movq	$-0x2, 0x490(%rbp)
140fee0fe: 48 89 ce                    	movq	%rcx, %rsi
140fee101: 0f b6 05 01 6b 64 02        	movzbl	0x2646b01(%rip), %eax   # 0x143634c09
140fee108: b9 f0 03 00 00              	movl	$0x3f0, %ecx            # imm = 0x3F0
140fee10d: ba 08 00 00 00              	movl	$0x8, %edx
140fee112: e8 f9 5a 62 ff              	callq	0x140613c10 <.text+0x612c10>
140fee117: 48 85 c0                    	testq	%rax, %rax
140fee11a: 0f 84 cd 05 00 00           	je	0x140fee6ed <.text+0xfed6ed>
140fee120: 48 89 85 88 04 00 00        	movq	%rax, 0x488(%rbp)
140fee127: 0f 57 c0                    	xorps	%xmm0, %xmm0
140fee12a: 0f 11 85 68 03 00 00        	movups	%xmm0, 0x368(%rbp)
140fee131: 0f 11 85 80 03 00 00        	movups	%xmm0, 0x380(%rbp)
140fee138: 0f 11 85 98 03 00 00        	movups	%xmm0, 0x398(%rbp)
140fee13f: 0f 11 85 b0 03 00 00        	movups	%xmm0, 0x3b0(%rbp)
140fee146: 0f 11 85 c8 03 00 00        	movups	%xmm0, 0x3c8(%rbp)
140fee14d: c7 85 d8 03 00 00 00 00 00 00       	movl	$0x0, 0x3d8(%rbp)
140fee157: 0f 11 85 00 04 00 00        	movups	%xmm0, 0x400(%rbp)
140fee15e: 0f 11 85 18 04 00 00        	movups	%xmm0, 0x418(%rbp)
140fee165: 0f 11 85 30 04 00 00        	movups	%xmm0, 0x430(%rbp)
140fee16c: 0f 11 85 48 04 00 00        	movups	%xmm0, 0x448(%rbp)
140fee173: 0f 11 85 60 04 00 00        	movups	%xmm0, 0x460(%rbp)
140fee17a: c7 85 70 04 00 00 00 00 00 00       	movl	$0x0, 0x470(%rbp)
140fee184: 48 c7 85 28 03 00 00 00 00 00 00    	movq	$0x0, 0x328(%rbp)
140fee18f: 48 c7 85 48 03 00 00 00 00 00 00    	movq	$0x0, 0x348(%rbp)
140fee19a: 48 c7 85 50 03 00 00 08 00 00 00    	movq	$0x8, 0x350(%rbp)
140fee1a5: 48 c7 85 58 03 00 00 00 00 00 00    	movq	$0x0, 0x358(%rbp)
140fee1b0: 48 c7 85 60 03 00 00 10 00 00 00    	movq	$0x10, 0x360(%rbp)
140fee1bb: 48 c7 85 78 03 00 00 10 00 00 00    	movq	$0x10, 0x378(%rbp)
140fee1c6: 48 c7 85 90 03 00 00 10 00 00 00    	movq	$0x10, 0x390(%rbp)
140fee1d1: 48 c7 85 a8 03 00 00 10 00 00 00    	movq	$0x10, 0x3a8(%rbp)
140fee1dc: 48 c7 85 c0 03 00 00 10 00 00 00    	movq	$0x10, 0x3c0(%rbp)
140fee1e7: 48 bb 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rbx # imm = 0x8000000000000000
140fee1f1: 48 89 9d e0 03 00 00        	movq	%rbx, 0x3e0(%rbp)
140fee1f8: 48 8d 05 67 51 bd 01        	leaq	0x1bd5167(%rip), %rax   # 0x142bc3366
140fee1ff: 48 89 85 e8 03 00 00        	movq	%rax, 0x3e8(%rbp)
140fee206: 48 c7 85 f0 03 00 00 29 00 00 00    	movq	$0x29, 0x3f0(%rbp)
140fee211: 48 c7 85 f8 03 00 00 10 00 00 00    	movq	$0x10, 0x3f8(%rbp)
140fee21c: 48 c7 85 10 04 00 00 10 00 00 00    	movq	$0x10, 0x410(%rbp)
140fee227: 48 c7 85 28 04 00 00 10 00 00 00    	movq	$0x10, 0x428(%rbp)
140fee232: 48 c7 85 40 04 00 00 10 00 00 00    	movq	$0x10, 0x440(%rbp)
140fee23d: 48 c7 85 58 04 00 00 10 00 00 00    	movq	$0x10, 0x458(%rbp)
140fee248: c7 85 78 04 00 00 00 00 00 00       	movl	$0x0, 0x478(%rbp)
140fee252: 66 c7 85 7c 04 00 00 01 00  	movw	$0x1, 0x47c(%rbp)
140fee25b: c7 85 80 04 00 00 00 00 00 00       	movl	$0x0, 0x480(%rbp)
140fee265: 0f b6 05 9d 69 64 02        	movzbl	0x264699d(%rip), %eax   # 0x143634c09
140fee26c: b9 60 01 00 00              	movl	$0x160, %ecx            # imm = 0x160
140fee271: ba 08 00 00 00              	movl	$0x8, %edx
140fee276: e8 95 59 62 ff              	callq	0x140613c10 <.text+0x612c10>
140fee27b: 48 85 c0                    	testq	%rax, %rax
140fee27e: 0f 84 78 04 00 00           	je	0x140fee6fc <.text+0xfed6fc>
140fee284: 48 89 c7                    	movq	%rax, %rdi
140fee287: 48 8d 95 28 03 00 00        	leaq	0x328(%rbp), %rdx
140fee28e: 41 b8 60 01 00 00           	movl	$0x160, %r8d            # imm = 0x160
140fee294: 48 89 c1                    	movq	%rax, %rcx
140fee297: e8 64 59 93 01              	callq	0x142923c00 <.text+0x2922c00>
140fee29c: 4c 8d 05 e5 4c bd 01        	leaq	0x1bd4ce5(%rip), %r8    # 0x142bc2f88
140fee2a3: 48 8d 8d b8 02 00 00        	leaq	0x2b8(%rbp), %rcx
140fee2aa: 48 89 fa                    	movq	%rdi, %rdx
140fee2ad: e8 5e 68 cf ff              	callq	0x140ce4b10 <.text+0xce3b10>
140fee2b2: 48 8d 8d 48 02 00 00        	leaq	0x248(%rbp), %rcx
140fee2b9: e8 72 a4 eb 00              	callq	0x141ea8730 <.text+0x1ea7730>
140fee2be: 48 8d 8d d8 01 00 00        	leaq	0x1d8(%rbp), %rcx
140fee2c5: e8 a6 9d eb 00              	callq	0x141ea8070 <.text+0x1ea7070>
140fee2ca: 48 8d 8d 68 01 00 00        	leaq	0x168(%rbp), %rcx
140fee2d1: e8 aa a2 eb 00              	callq	0x141ea8580 <.text+0x1ea7580>
140fee2d6: 48 8d 8d f8 00 00 00        	leaq	0xf8(%rbp), %rcx
140fee2dd: e8 5e db eb 00              	callq	0x141eabe40 <.text+0x1eaae40>
140fee2e2: 48 8d 8d 88 00 00 00        	leaq	0x88(%rbp), %rcx
140fee2e9: e8 22 9a eb 00              	callq	0x141ea7d10 <.text+0x1ea6d10>
140fee2ee: 48 8d 4d 18                 	leaq	0x18(%rbp), %rcx
140fee2f2: e8 c9 9b eb 00              	callq	0x141ea7ec0 <.text+0x1ea6ec0>
140fee2f7: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
140fee2fb: e8 10 87 eb 00              	callq	0x141ea6a10 <.text+0x1ea5a10>
140fee300: 48 8d 8d 28 03 00 00        	leaq	0x328(%rbp), %rcx
140fee307: e8 e4 8d eb 00              	callq	0x141ea70f0 <.text+0x1ea60f0>
140fee30c: 0f 10 85 18 03 00 00        	movups	0x318(%rbp), %xmm0
140fee313: 48 8b 85 88 04 00 00        	movq	0x488(%rbp), %rax
140fee31a: 0f 11 40 60                 	movups	%xmm0, 0x60(%rax)
140fee31e: 0f 10 85 08 03 00 00        	movups	0x308(%rbp), %xmm0
140fee325: 0f 11 40 50                 	movups	%xmm0, 0x50(%rax)
140fee329: 0f 10 85 f8 02 00 00        	movups	0x2f8(%rbp), %xmm0
140fee330: 0f 11 40 40                 	movups	%xmm0, 0x40(%rax)
140fee334: 0f 10 85 b8 02 00 00        	movups	0x2b8(%rbp), %xmm0
140fee33b: 0f 10 8d c8 02 00 00        	movups	0x2c8(%rbp), %xmm1
140fee342: 0f 10 95 d8 02 00 00        	movups	0x2d8(%rbp), %xmm2
140fee349: 0f 10 9d e8 02 00 00        	movups	0x2e8(%rbp), %xmm3
140fee350: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
140fee354: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
140fee358: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
140fee35c: 0f 11 00                    	movups	%xmm0, (%rax)
140fee35f: 0f 10 85 48 02 00 00        	movups	0x248(%rbp), %xmm0
140fee366: 0f 10 8d 58 02 00 00        	movups	0x258(%rbp), %xmm1
140fee36d: 0f 10 95 68 02 00 00        	movups	0x268(%rbp), %xmm2
140fee374: 0f 10 9d 78 02 00 00        	movups	0x278(%rbp), %xmm3
140fee37b: 0f 11 40 70                 	movups	%xmm0, 0x70(%rax)
140fee37f: 0f 11 88 80 00 00 00        	movups	%xmm1, 0x80(%rax)
140fee386: 0f 11 90 90 00 00 00        	movups	%xmm2, 0x90(%rax)
140fee38d: 0f 11 98 a0 00 00 00        	movups	%xmm3, 0xa0(%rax)
140fee394: 0f 10 85 88 02 00 00        	movups	0x288(%rbp), %xmm0
140fee39b: 0f 11 80 b0 00 00 00        	movups	%xmm0, 0xb0(%rax)
140fee3a2: 0f 10 85 98 02 00 00        	movups	0x298(%rbp), %xmm0
140fee3a9: 0f 11 80 c0 00 00 00        	movups	%xmm0, 0xc0(%rax)
140fee3b0: 0f 10 85 a8 02 00 00        	movups	0x2a8(%rbp), %xmm0
140fee3b7: 0f 11 80 d0 00 00 00        	movups	%xmm0, 0xd0(%rax)
140fee3be: 0f 10 85 38 02 00 00        	movups	0x238(%rbp), %xmm0
140fee3c5: 0f 11 80 40 01 00 00        	movups	%xmm0, 0x140(%rax)
140fee3cc: 0f 10 85 28 02 00 00        	movups	0x228(%rbp), %xmm0
140fee3d3: 0f 11 80 30 01 00 00        	movups	%xmm0, 0x130(%rax)
140fee3da: 0f 10 85 18 02 00 00        	movups	0x218(%rbp), %xmm0
140fee3e1: 0f 11 80 20 01 00 00        	movups	%xmm0, 0x120(%rax)
140fee3e8: 0f 10 85 d8 01 00 00        	movups	0x1d8(%rbp), %xmm0
140fee3ef: 0f 10 8d e8 01 00 00        	movups	0x1e8(%rbp), %xmm1
140fee3f6: 0f 10 95 f8 01 00 00        	movups	0x1f8(%rbp), %xmm2
140fee3fd: 0f 10 9d 08 02 00 00        	movups	0x208(%rbp), %xmm3
140fee404: 0f 11 98 10 01 00 00        	movups	%xmm3, 0x110(%rax)
140fee40b: 0f 11 90 00 01 00 00        	movups	%xmm2, 0x100(%rax)
140fee412: 0f 11 88 f0 00 00 00        	movups	%xmm1, 0xf0(%rax)
140fee419: 0f 11 80 e0 00 00 00        	movups	%xmm0, 0xe0(%rax)
140fee420: 0f 10 85 68 01 00 00        	movups	0x168(%rbp), %xmm0
140fee427: 0f 10 8d 78 01 00 00        	movups	0x178(%rbp), %xmm1
140fee42e: 0f 10 95 88 01 00 00        	movups	0x188(%rbp), %xmm2
140fee435: 0f 10 9d 98 01 00 00        	movups	0x198(%rbp), %xmm3
140fee43c: 0f 11 80 50 01 00 00        	movups	%xmm0, 0x150(%rax)
140fee443: 0f 11 88 60 01 00 00        	movups	%xmm1, 0x160(%rax)
140fee44a: 0f 11 90 70 01 00 00        	movups	%xmm2, 0x170(%rax)
140fee451: 0f 11 98 80 01 00 00        	movups	%xmm3, 0x180(%rax)
140fee458: 0f 10 85 a8 01 00 00        	movups	0x1a8(%rbp), %xmm0
140fee45f: 0f 11 80 90 01 00 00        	movups	%xmm0, 0x190(%rax)
140fee466: 0f 10 85 b8 01 00 00        	movups	0x1b8(%rbp), %xmm0
140fee46d: 0f 11 80 a0 01 00 00        	movups	%xmm0, 0x1a0(%rax)
140fee474: 0f 10 85 c8 01 00 00        	movups	0x1c8(%rbp), %xmm0
140fee47b: 0f 11 80 b0 01 00 00        	movups	%xmm0, 0x1b0(%rax)
140fee482: 0f 10 85 f8 00 00 00        	movups	0xf8(%rbp), %xmm0
140fee489: 0f 10 8d 08 01 00 00        	movups	0x108(%rbp), %xmm1
140fee490: 0f 10 95 18 01 00 00        	movups	0x118(%rbp), %xmm2
140fee497: 0f 10 9d 28 01 00 00        	movups	0x128(%rbp), %xmm3
140fee49e: 0f 11 80 c0 01 00 00        	movups	%xmm0, 0x1c0(%rax)
140fee4a5: 0f 11 88 d0 01 00 00        	movups	%xmm1, 0x1d0(%rax)
140fee4ac: 0f 11 90 e0 01 00 00        	movups	%xmm2, 0x1e0(%rax)
140fee4b3: 0f 11 98 f0 01 00 00        	movups	%xmm3, 0x1f0(%rax)
140fee4ba: 0f 10 85 38 01 00 00        	movups	0x138(%rbp), %xmm0
140fee4c1: 0f 11 80 00 02 00 00        	movups	%xmm0, 0x200(%rax)
140fee4c8: 0f 10 85 58 01 00 00        	movups	0x158(%rbp), %xmm0
140fee4cf: 0f 11 80 20 02 00 00        	movups	%xmm0, 0x220(%rax)
140fee4d6: 0f 10 85 48 01 00 00        	movups	0x148(%rbp), %xmm0
140fee4dd: 0f 11 80 10 02 00 00        	movups	%xmm0, 0x210(%rax)
140fee4e4: 0f 10 85 e8 00 00 00        	movups	0xe8(%rbp), %xmm0
140fee4eb: 0f 11 80 90 02 00 00        	movups	%xmm0, 0x290(%rax)
140fee4f2: 0f 10 85 d8 00 00 00        	movups	0xd8(%rbp), %xmm0
140fee4f9: 0f 11 80 80 02 00 00        	movups	%xmm0, 0x280(%rax)
140fee500: 0f 10 85 c8 00 00 00        	movups	0xc8(%rbp), %xmm0
140fee507: 0f 11 80 70 02 00 00        	movups	%xmm0, 0x270(%rax)
140fee50e: 0f 10 85 88 00 00 00        	movups	0x88(%rbp), %xmm0
140fee515: 0f 10 8d 98 00 00 00        	movups	0x98(%rbp), %xmm1
140fee51c: 0f 10 95 a8 00 00 00        	movups	0xa8(%rbp), %xmm2
140fee523: 0f 10 9d b8 00 00 00        	movups	0xb8(%rbp), %xmm3
140fee52a: 0f 11 98 60 02 00 00        	movups	%xmm3, 0x260(%rax)
140fee531: 0f 11 90 50 02 00 00        	movups	%xmm2, 0x250(%rax)
140fee538: 0f 11 88 40 02 00 00        	movups	%xmm1, 0x240(%rax)
140fee53f: 0f 11 80 30 02 00 00        	movups	%xmm0, 0x230(%rax)
140fee546: 0f 10 45 18                 	movups	0x18(%rbp), %xmm0
140fee54a: 0f 10 4d 28                 	movups	0x28(%rbp), %xmm1
140fee54e: 0f 10 55 38                 	movups	0x38(%rbp), %xmm2
140fee552: 0f 10 5d 48                 	movups	0x48(%rbp), %xmm3
140fee556: 0f 11 80 a0 02 00 00        	movups	%xmm0, 0x2a0(%rax)
140fee55d: 0f 11 88 b0 02 00 00        	movups	%xmm1, 0x2b0(%rax)
140fee564: 0f 11 90 c0 02 00 00        	movups	%xmm2, 0x2c0(%rax)
140fee56b: 0f 11 98 d0 02 00 00        	movups	%xmm3, 0x2d0(%rax)
140fee572: 0f 10 45 58                 	movups	0x58(%rbp), %xmm0
140fee576: 0f 11 80 e0 02 00 00        	movups	%xmm0, 0x2e0(%rax)
140fee57d: 0f 10 45 68                 	movups	0x68(%rbp), %xmm0
140fee581: 0f 11 80 f0 02 00 00        	movups	%xmm0, 0x2f0(%rax)
140fee588: 0f 10 45 78                 	movups	0x78(%rbp), %xmm0
140fee58c: 0f 11 80 00 03 00 00        	movups	%xmm0, 0x300(%rax)
140fee593: 0f 10 45 a8                 	movups	-0x58(%rbp), %xmm0
140fee597: 0f 10 4d b8                 	movups	-0x48(%rbp), %xmm1
140fee59b: 0f 10 55 c8                 	movups	-0x38(%rbp), %xmm2
140fee59f: 0f 10 5d d8                 	movups	-0x28(%rbp), %xmm3
140fee5a3: 0f 11 80 10 03 00 00        	movups	%xmm0, 0x310(%rax)
140fee5aa: 0f 11 88 20 03 00 00        	movups	%xmm1, 0x320(%rax)
140fee5b1: 0f 11 90 30 03 00 00        	movups	%xmm2, 0x330(%rax)
140fee5b8: 0f 11 98 40 03 00 00        	movups	%xmm3, 0x340(%rax)
140fee5bf: 0f 10 45 e8                 	movups	-0x18(%rbp), %xmm0
140fee5c3: 0f 11 80 50 03 00 00        	movups	%xmm0, 0x350(%rax)
140fee5ca: 0f 10 45 f8                 	movups	-0x8(%rbp), %xmm0
140fee5ce: 0f 11 80 60 03 00 00        	movups	%xmm0, 0x360(%rax)
140fee5d5: 0f 10 45 08                 	movups	0x8(%rbp), %xmm0
140fee5d9: 0f 11 80 70 03 00 00        	movups	%xmm0, 0x370(%rax)
140fee5e0: 0f 10 85 28 03 00 00        	movups	0x328(%rbp), %xmm0
140fee5e7: 0f 10 8d 38 03 00 00        	movups	0x338(%rbp), %xmm1
140fee5ee: 0f 10 95 48 03 00 00        	movups	0x348(%rbp), %xmm2
140fee5f5: 0f 10 9d 58 03 00 00        	movups	0x358(%rbp), %xmm3
140fee5fc: 0f 11 80 80 03 00 00        	movups	%xmm0, 0x380(%rax)
140fee603: 0f 11 88 90 03 00 00        	movups	%xmm1, 0x390(%rax)
140fee60a: 0f 11 90 a0 03 00 00        	movups	%xmm2, 0x3a0(%rax)
140fee611: 0f 11 98 b0 03 00 00        	movups	%xmm3, 0x3b0(%rax)
140fee618: 0f 10 85 68 03 00 00        	movups	0x368(%rbp), %xmm0
140fee61f: 0f 11 80 c0 03 00 00        	movups	%xmm0, 0x3c0(%rax)
140fee626: 0f 10 85 78 03 00 00        	movups	0x378(%rbp), %xmm0
140fee62d: 0f 11 80 d0 03 00 00        	movups	%xmm0, 0x3d0(%rax)
140fee634: 0f 10 85 88 03 00 00        	movups	0x388(%rbp), %xmm0
140fee63b: 0f 11 80 e0 03 00 00        	movups	%xmm0, 0x3e0(%rax)
140fee642: 48 c7 85 30 03 00 00 09 00 00 00    	movq	$0x9, 0x330(%rbp)
140fee64d: 48 89 85 38 03 00 00        	movq	%rax, 0x338(%rbp)
140fee654: 48 c7 85 40 03 00 00 09 00 00 00    	movq	$0x9, 0x340(%rbp)
140fee65f: 48 c7 85 48 03 00 00 00 00 00 00    	movq	$0x0, 0x348(%rbp)
140fee66a: 48 c7 85 50 03 00 00 08 00 00 00    	movq	$0x8, 0x350(%rbp)
140fee675: 0f 57 c0                    	xorps	%xmm0, %xmm0
140fee678: 0f 11 85 58 03 00 00        	movups	%xmm0, 0x358(%rbp)
140fee67f: 48 89 9d 28 03 00 00        	movq	%rbx, 0x328(%rbp)
140fee686: 48 8d 8d 60 03 00 00        	leaq	0x360(%rbp), %rcx
140fee68d: e8 8e 60 fe ff              	callq	0x140fd4720 <.text+0xfd3720>
140fee692: 0f 10 85 88 03 00 00        	movups	0x388(%rbp), %xmm0
140fee699: 0f 11 46 60                 	movups	%xmm0, 0x60(%rsi)
140fee69d: 0f 10 85 78 03 00 00        	movups	0x378(%rbp), %xmm0
140fee6a4: 0f 11 46 50                 	movups	%xmm0, 0x50(%rsi)
140fee6a8: 0f 10 85 68 03 00 00        	movups	0x368(%rbp), %xmm0
140fee6af: 0f 11 46 40                 	movups	%xmm0, 0x40(%rsi)
140fee6b3: 0f 10 85 28 03 00 00        	movups	0x328(%rbp), %xmm0
140fee6ba: 0f 10 8d 38 03 00 00        	movups	0x338(%rbp), %xmm1
140fee6c1: 0f 10 95 48 03 00 00        	movups	0x348(%rbp), %xmm2
140fee6c8: 0f 10 9d 58 03 00 00        	movups	0x358(%rbp), %xmm3
140fee6cf: 0f 11 5e 30                 	movups	%xmm3, 0x30(%rsi)
140fee6d3: 0f 11 56 20                 	movups	%xmm2, 0x20(%rsi)
140fee6d7: 0f 11 4e 10                 	movups	%xmm1, 0x10(%rsi)
140fee6db: 0f 11 06                    	movups	%xmm0, (%rsi)
140fee6de: 48 89 f0                    	movq	%rsi, %rax
140fee6e1: 48 81 c4 18 05 00 00        	addq	$0x518, %rsp            # imm = 0x518
140fee6e8: 5b                          	popq	%rbx
140fee6e9: 5f                          	popq	%rdi
140fee6ea: 5e                          	popq	%rsi
140fee6eb: 5d                          	popq	%rbp
140fee6ec: c3                          	retq
140fee6ed: b9 08 00 00 00              	movl	$0x8, %ecx
140fee6f2: ba f0 03 00 00              	movl	$0x3f0, %edx            # imm = 0x3F0
140fee6f7: e8 04 a9 8e 01              	callq	0x1428d9000 <.text+0x28d8000>
140fee6fc: b9 08 00 00 00              	movl	$0x8, %ecx
140fee701: ba 60 01 00 00              	movl	$0x160, %edx            # imm = 0x160
140fee706: e8 f5 a8 8e 01              	callq	0x1428d9000 <.text+0x28d8000>
140fee70b: 0f 0b                       	ud2
140fee70d: 0f 1f 00                    	nopl	(%rax)
140fee710: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140fee715: 55                          	pushq	%rbp
140fee716: 56                          	pushq	%rsi
140fee717: 57                          	pushq	%rdi
140fee718: 53                          	pushq	%rbx
140fee719: 48 83 ec 28                 	subq	$0x28, %rsp
140fee71d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140fee724: 48 8d 8d 28 03 00 00        	leaq	0x328(%rbp), %rcx
140fee72b: e8 40 77 5f ff              	callq	0x1405e5e70 <.text+0x5e4e70>
140fee730: 90                          	nop
140fee731: 48 83 c4 28                 	addq	$0x28, %rsp
140fee735: 5b                          	popq	%rbx
140fee736: 5f                          	popq	%rdi
140fee737: 5e                          	popq	%rsi
140fee738: 5d                          	popq	%rbp
140fee739: c3                          	retq
140fee73a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140fee740: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140fee745: 55                          	pushq	%rbp
140fee746: 56                          	pushq	%rsi
140fee747: 57                          	pushq	%rdi
140fee748: 53                          	pushq	%rbx
140fee749: 48 83 ec 28                 	subq	$0x28, %rsp
140fee74d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140fee754: ba f0 03 00 00              	movl	$0x3f0, %edx            # imm = 0x3F0
140fee759: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140fee75f: 48 8b 8d 88 04 00 00        	movq	0x488(%rbp), %rcx
140fee766: e8 b5 54 62 ff              	callq	0x140613c20 <.text+0x612c20>
140fee76b: 90                          	nop
140fee76c: 48 83 c4 28                 	addq	$0x28, %rsp
140fee770: 5b                          	popq	%rbx
140fee771: 5f                          	popq	%rdi
140fee772: 5e                          	popq	%rsi
140fee773: 5d                          	popq	%rbp
140fee774: c3                          	retq
140fee775: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140fee780: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140fee785: 55                          	pushq	%rbp
140fee786: 56                          	pushq	%rsi
140fee787: 57                          	pushq	%rdi
140fee788: 53                          	pushq	%rbx
140fee789: 48 83 ec 28                 	subq	$0x28, %rsp
140fee78d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140fee794: 48 8d 8d b8 02 00 00        	leaq	0x2b8(%rbp), %rcx
140fee79b: e8 e0 b0 03 ff              	callq	0x140029880 <.text+0x28880>
140fee7a0: 90                          	nop
140fee7a1: 48 83 c4 28                 	addq	$0x28, %rsp
140fee7a5: 5b                          	popq	%rbx
140fee7a6: 5f                          	popq	%rdi
140fee7a7: 5e                          	popq	%rsi
140fee7a8: 5d                          	popq	%rbp
140fee7a9: c3                          	retq
140fee7aa: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140fee7b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140fee7b5: 55                          	pushq	%rbp
140fee7b6: 56                          	pushq	%rsi
140fee7b7: 57                          	pushq	%rdi
140fee7b8: 53                          	pushq	%rbx
140fee7b9: 48 83 ec 28                 	subq	$0x28, %rsp
140fee7bd: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140fee7c4: 48 8d 8d 48 02 00 00        	leaq	0x248(%rbp), %rcx
140fee7cb: e8 b0 b0 03 ff              	callq	0x140029880 <.text+0x28880>
140fee7d0: 90                          	nop
140fee7d1: 48 83 c4 28                 	addq	$0x28, %rsp
140fee7d5: 5b                          	popq	%rbx
140fee7d6: 5f                          	popq	%rdi
140fee7d7: 5e                          	popq	%rsi
140fee7d8: 5d                          	popq	%rbp
140fee7d9: c3                          	retq
140fee7da: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140fee7e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140fee7e5: 55                          	pushq	%rbp
140fee7e6: 56                          	pushq	%rsi
140fee7e7: 57                          	pushq	%rdi
140fee7e8: 53                          	pushq	%rbx
140fee7e9: 48 83 ec 28                 	subq	$0x28, %rsp
140fee7ed: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140fee7f4: 48 8d 8d d8 01 00 00        	leaq	0x1d8(%rbp), %rcx
140fee7fb: e8 80 b0 03 ff              	callq	0x140029880 <.text+0x28880>
140fee800: 90                          	nop
140fee801: 48 83 c4 28                 	addq	$0x28, %rsp
140fee805: 5b                          	popq	%rbx
140fee806: 5f                          	popq	%rdi
140fee807: 5e                          	popq	%rsi
140fee808: 5d                          	popq	%rbp
140fee809: c3                          	retq
140fee80a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140fee810: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140fee815: 55                          	pushq	%rbp
140fee816: 56                          	pushq	%rsi
140fee817: 57                          	pushq	%rdi
140fee818: 53                          	pushq	%rbx
140fee819: 48 83 ec 28                 	subq	$0x28, %rsp
140fee81d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140fee824: 48 8d 8d 68 01 00 00        	leaq	0x168(%rbp), %rcx
140fee82b: e8 50 b0 03 ff              	callq	0x140029880 <.text+0x28880>
140fee830: 90                          	nop
140fee831: 48 83 c4 28                 	addq	$0x28, %rsp
140fee835: 5b                          	popq	%rbx
140fee836: 5f                          	popq	%rdi
140fee837: 5e                          	popq	%rsi
140fee838: 5d                          	popq	%rbp
140fee839: c3                          	retq
140fee83a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140fee840: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140fee845: 55                          	pushq	%rbp
140fee846: 56                          	pushq	%rsi
140fee847: 57                          	pushq	%rdi
140fee848: 53                          	pushq	%rbx
140fee849: 48 83 ec 28                 	subq	$0x28, %rsp
140fee84d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140fee854: 48 8d 8d f8 00 00 00        	leaq	0xf8(%rbp), %rcx
140fee85b: e8 20 b0 03 ff              	callq	0x140029880 <.text+0x28880>
140fee860: 90                          	nop
140fee861: 48 83 c4 28                 	addq	$0x28, %rsp
140fee865: 5b                          	popq	%rbx
140fee866: 5f                          	popq	%rdi
140fee867: 5e                          	popq	%rsi
140fee868: 5d                          	popq	%rbp
140fee869: c3                          	retq
140fee86a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140fee870: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140fee875: 55                          	pushq	%rbp
140fee876: 56                          	pushq	%rsi
140fee877: 57                          	pushq	%rdi
140fee878: 53                          	pushq	%rbx
140fee879: 48 83 ec 28                 	subq	$0x28, %rsp
140fee87d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140fee884: 48 8d 8d 88 00 00 00        	leaq	0x88(%rbp), %rcx
140fee88b: e8 f0 af 03 ff              	callq	0x140029880 <.text+0x28880>
140fee890: 90                          	nop
140fee891: 48 83 c4 28                 	addq	$0x28, %rsp
140fee895: 5b                          	popq	%rbx
140fee896: 5f                          	popq	%rdi
140fee897: 5e                          	popq	%rsi
140fee898: 5d                          	popq	%rbp
140fee899: c3                          	retq
140fee89a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140fee8a0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140fee8a5: 55                          	pushq	%rbp
140fee8a6: 56                          	pushq	%rsi
140fee8a7: 57                          	pushq	%rdi
140fee8a8: 53                          	pushq	%rbx
140fee8a9: 48 83 ec 28                 	subq	$0x28, %rsp
140fee8ad: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140fee8b4: 48 8d 4d 18                 	leaq	0x18(%rbp), %rcx
140fee8b8: e8 c3 af 03 ff              	callq	0x140029880 <.text+0x28880>
140fee8bd: 90                          	nop
140fee8be: 48 83 c4 28                 	addq	$0x28, %rsp
140fee8c2: 5b                          	popq	%rbx
140fee8c3: 5f                          	popq	%rdi
140fee8c4: 5e                          	popq	%rsi
140fee8c5: 5d                          	popq	%rbp
140fee8c6: c3                          	retq
140fee8c7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140fee8d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140fee8d5: 55                          	pushq	%rbp
140fee8d6: 56                          	pushq	%rsi
140fee8d7: 57                          	pushq	%rdi
140fee8d8: 53                          	pushq	%rbx
140fee8d9: 48 83 ec 28                 	subq	$0x28, %rsp
140fee8dd: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140fee8e4: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
140fee8e8: e8 93 af 03 ff              	callq	0x140029880 <.text+0x28880>
140fee8ed: 90                          	nop
140fee8ee: 48 83 c4 28                 	addq	$0x28, %rsp
140fee8f2: 5b                          	popq	%rbx
140fee8f3: 5f                          	popq	%rdi
140fee8f4: 5e                          	popq	%rsi
140fee8f5: 5d                          	popq	%rbp
140fee8f6: c3                          	retq
140fee8f7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140fee900: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140fee905: 55                          	pushq	%rbp
140fee906: 56                          	pushq	%rsi
140fee907: 57                          	pushq	%rdi
140fee908: 53                          	pushq	%rbx
140fee909: 48 83 ec 28                 	subq	$0x28, %rsp
140fee90d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140fee914: 48 8d 8d 28 03 00 00        	leaq	0x328(%rbp), %rcx
140fee91b: e8 60 af 03 ff              	callq	0x140029880 <.text+0x28880>
140fee920: 90                          	nop
140fee921: 48 83 c4 28                 	addq	$0x28, %rsp
140fee925: 5b                          	popq	%rbx
140fee926: 5f                          	popq	%rdi
140fee927: 5e                          	popq	%rsi
140fee928: 5d                          	popq	%rbp
140fee929: c3                          	retq
140fee92a: cc                          	int3
140fee92b: cc                          	int3
140fee92c: cc                          	int3
140fee92d: cc                          	int3
140fee92e: cc                          	int3
140fee92f: cc                          	int3
140fee930: 41 57                       	pushq	%r15
140fee932: 41 56                       	pushq	%r14
140fee934: 41 55                       	pushq	%r13
140fee936: 41 54                       	pushq	%r12
140fee938: 56                          	pushq	%rsi
140fee939: 57                          	pushq	%rdi
140fee93a: 55                          	pushq	%rbp
140fee93b: 53                          	pushq	%rbx
140fee93c: 48 83 ec 78                 	subq	$0x78, %rsp
140fee940: 48 89 d7                    	movq	%rdx, %rdi
140fee943: 48 89 ce                    	movq	%rcx, %rsi
140fee946: 83 b9 38 01 00 00 02        	cmpl	$0x2, 0x138(%rcx)
140fee94d: 0f 85 e2 00 00 00           	jne	0x140feea35 <.text+0xfeda35>
140fee953: 48 89 f9                    	movq	%rdi, %rcx
140fee956: 48 89 f2                    	movq	%rsi, %rdx
140fee959: e8 e2 58 de 00              	callq	0x141dd4240 <.text+0x1dd3240>
140fee95e: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
140fee963: 48 89 f9                    	movq	%rdi, %rcx
140fee966: 48 89 f2                    	movq	%rsi, %rdx
140fee969: e8 92 e7 dd 00              	callq	0x141dcd100 <.text+0x1dcc100>
140fee96e: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
140fee973: 48 89 f9                    	movq	%rdi, %rcx
140fee976: 48 89 f2                    	movq	%rsi, %rdx
140fee979: e8 02 2d de 00              	callq	0x141dd1680 <.text+0x1dd0680>
140fee97e: 49 89 c7                    	movq	%rax, %r15
140fee981: 48 89 f9                    	movq	%rdi, %rcx
140fee984: 48 89 f2                    	movq	%rsi, %rdx
140fee987: e8 24 5f de 00              	callq	0x141dd48b0 <.text+0x1dd38b0>
140fee98c: 49 89 c4                    	movq	%rax, %r12
140fee98f: 48 89 f9                    	movq	%rdi, %rcx
140fee992: 48 89 f2                    	movq	%rsi, %rdx
140fee995: e8 36 86 de 00              	callq	0x141dd6fd0 <.text+0x1dd5fd0>
140fee99a: 49 89 c5                    	movq	%rax, %r13
140fee99d: 48 89 f9                    	movq	%rdi, %rcx
140fee9a0: 48 89 f2                    	movq	%rsi, %rdx
140fee9a3: e8 48 cb dd 00              	callq	0x141dcb4f0 <.text+0x1dca4f0>
140fee9a8: 48 89 c5                    	movq	%rax, %rbp
140fee9ab: 48 89 f9                    	movq	%rdi, %rcx
140fee9ae: 48 89 f2                    	movq	%rsi, %rdx
140fee9b1: e8 1a 79 df 00              	callq	0x141de62d0 <.text+0x1de52d0>
140fee9b6: 48 89 c3                    	movq	%rax, %rbx
140fee9b9: 48 89 f9                    	movq	%rdi, %rcx
140fee9bc: 48 89 f2                    	movq	%rsi, %rdx
140fee9bf: e8 6c 4d de 00              	callq	0x141dd3730 <.text+0x1dd2730>
140fee9c4: 49 89 c6                    	movq	%rax, %r14
140fee9c7: 48 89 f9                    	movq	%rdi, %rcx
140fee9ca: 48 89 f2                    	movq	%rsi, %rdx
140fee9cd: e8 9e 96 dd 00              	callq	0x141dc8070 <.text+0x1dc7070>
140fee9d2: 48 8b 8f a0 05 00 00        	movq	0x5a0(%rdi), %rcx
140fee9d9: c7 86 38 01 00 00 00 00 00 00       	movl	$0x0, 0x138(%rsi)
140fee9e3: 48 8b 54 24 38              	movq	0x38(%rsp), %rdx
140fee9e8: 48 89 96 48 01 00 00        	movq	%rdx, 0x148(%rsi)
140fee9ef: 48 8b 54 24 30              	movq	0x30(%rsp), %rdx
140fee9f4: 48 89 96 50 01 00 00        	movq	%rdx, 0x150(%rsi)
140fee9fb: 4c 89 be 58 01 00 00        	movq	%r15, 0x158(%rsi)
140feea02: 4c 89 a6 60 01 00 00        	movq	%r12, 0x160(%rsi)
140feea09: 4c 89 ae 68 01 00 00        	movq	%r13, 0x168(%rsi)
140feea10: 48 89 ae 70 01 00 00        	movq	%rbp, 0x170(%rsi)
140feea17: 48 89 9e 78 01 00 00        	movq	%rbx, 0x178(%rsi)
140feea1e: 4c 89 b6 80 01 00 00        	movq	%r14, 0x180(%rsi)
140feea25: 48 89 86 88 01 00 00        	movq	%rax, 0x188(%rsi)
140feea2c: 48 89 8e 90 01 00 00        	movq	%rcx, 0x190(%rsi)
140feea33: eb 15                       	jmp	0x140feea4a <.text+0xfeda4a>
140feea35: 48 8b 87 a0 05 00 00        	movq	0x5a0(%rdi), %rax
140feea3c: 48 89 44 24 40              	movq	%rax, 0x40(%rsp)
140feea41: 48 39 86 90 01 00 00        	cmpq	%rax, 0x190(%rsi)
140feea48: 75 22                       	jne	0x140feea6c <.text+0xfeda6c>
140feea4a: b8 00 50 cc 3d              	movl	$0x3dcc5000, %eax       # imm = 0x3DCC5000
140feea4f: 03 87 b0 05 00 00           	addl	0x5b0(%rdi), %eax
140feea55: 89 86 30 01 00 00           	movl	%eax, 0x130(%rsi)
140feea5b: 48 83 c4 78                 	addq	$0x78, %rsp
140feea5f: 5b                          	popq	%rbx
140feea60: 5d                          	popq	%rbp
140feea61: 5f                          	popq	%rdi
140feea62: 5e                          	popq	%rsi
140feea63: 41 5c                       	popq	%r12
140feea65: 41 5d                       	popq	%r13
140feea67: 41 5e                       	popq	%r14
140feea69: 41 5f                       	popq	%r15
140feea6b: c3                          	retq
140feea6c: 48 8d 05 8d 4c bd 01        	leaq	0x1bd4c8d(%rip), %rax   # 0x142bc3700
140feea73: 48 89 44 24 48              	movq	%rax, 0x48(%rsp)
140feea78: 48 c7 44 24 50 01 00 00 00  	movq	$0x1, 0x50(%rsp)
140feea81: 48 c7 44 24 58 08 00 00 00  	movq	$0x8, 0x58(%rsp)
140feea8a: 0f 57 c0                    	xorps	%xmm0, %xmm0
140feea8d: 0f 11 44 24 60              	movups	%xmm0, 0x60(%rsp)
140feea92: 48 81 c6 90 01 00 00        	addq	$0x190, %rsi            # imm = 0x190
140feea99: 48 8d 05 e0 4c bd 01        	leaq	0x1bd4ce0(%rip), %rax   # 0x142bc3780
140feeaa0: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140feeaa5: 4c 8d 44 24 40              	leaq	0x40(%rsp), %r8
140feeaaa: 4c 8d 4c 24 48              	leaq	0x48(%rsp), %r9
140feeaaf: 31 c9                       	xorl	%ecx, %ecx
140feeab1: 48 89 f2                    	movq	%rsi, %rdx
140feeab4: e8 28 f6 8d 01              	callq	0x1428ce0e1 <.text+0x28cd0e1>
140feeab9: cc                          	int3
140feeaba: cc                          	int3
140feeabb: cc                          	int3
140feeabc: cc                          	int3
140feeabd: cc                          	int3
140feeabe: cc                          	int3
140feeabf: cc                          	int3
140feeac0: 41 57                       	pushq	%r15
140feeac2: 41 56                       	pushq	%r14
140feeac4: 41 55                       	pushq	%r13
140feeac6: 41 54                       	pushq	%r12
140feeac8: 56                          	pushq	%rsi
140feeac9: 57                          	pushq	%rdi
140feeaca: 55                          	pushq	%rbp
140feeacb: 53                          	pushq	%rbx
140feeacc: 48 81 ec e8 01 00 00        	subq	$0x1e8, %rsp            # imm = 0x1E8
140feead3: bf 01 00 00 00              	movl	$0x1, %edi
140feead8: f0                          	lock
140feead9: 0f c1 ba b0 05 00 00        	xaddl	%edi, 0x5b0(%rdx)
140feeae0: 83 b9 38 01 00 00 02        	cmpl	$0x2, 0x138(%rcx)
140feeae7: 0f 84 16 07 00 00           	je	0x140fef203 <.text+0xfee203>
140feeaed: 48 89 ce                    	movq	%rcx, %rsi
140feeaf0: 48 8b 81 48 01 00 00        	movq	0x148(%rcx), %rax
140feeaf7: 4c 8b 9a 18 02 00 00        	movq	0x218(%rdx), %r11
140feeafe: 49 39 c3                    	cmpq	%rax, %r11
140feeb01: 0f 86 e4 04 00 00           	jbe	0x140feefeb <.text+0xfedfeb>
140feeb07: 4c 8b b2 10 02 00 00        	movq	0x210(%rdx), %r14
140feeb0e: 49 8b 04 c6                 	movq	(%r14,%rax,8), %rax
140feeb12: 48 85 c0                    	testq	%rax, %rax
140feeb15: 0f 84 d0 04 00 00           	je	0x140feefeb <.text+0xfedfeb>
140feeb1b: 4c 8b ba e0 01 00 00        	movq	0x1e0(%rdx), %r15
140feeb22: 48 f7 d0                    	notq	%rax
140feeb25: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
140feeb29: 48 c1 e0 05                 	shlq	$0x5, %rax
140feeb2d: 49 83 7c 07 20 00           	cmpq	$0x0, 0x20(%r15,%rax)
140feeb33: 0f 84 b2 04 00 00           	je	0x140feefeb <.text+0xfedfeb>
140feeb39: 48 8b 8e 50 01 00 00        	movq	0x150(%rsi), %rcx
140feeb40: 49 39 cb                    	cmpq	%rcx, %r11
140feeb43: 0f 86 c3 04 00 00           	jbe	0x140fef00c <.text+0xfee00c>
140feeb49: 49 8b 0c ce                 	movq	(%r14,%rcx,8), %rcx
140feeb4d: 48 85 c9                    	testq	%rcx, %rcx
140feeb50: 0f 84 b6 04 00 00           	je	0x140fef00c <.text+0xfee00c>
140feeb56: 48 f7 d1                    	notq	%rcx
140feeb59: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140feeb5d: 48 c1 e1 05                 	shlq	$0x5, %rcx
140feeb61: 49 83 7c 0f 20 00           	cmpq	$0x0, 0x20(%r15,%rcx)
140feeb67: 0f 84 9f 04 00 00           	je	0x140fef00c <.text+0xfee00c>
140feeb6d: 4c 8b 86 58 01 00 00        	movq	0x158(%rsi), %r8
140feeb74: 4d 39 c3                    	cmpq	%r8, %r11
140feeb77: 0f 86 b0 04 00 00           	jbe	0x140fef02d <.text+0xfee02d>
140feeb7d: 4f 8b 04 c6                 	movq	(%r14,%r8,8), %r8
140feeb81: 4d 85 c0                    	testq	%r8, %r8
140feeb84: 0f 84 a3 04 00 00           	je	0x140fef02d <.text+0xfee02d>
140feeb8a: 49 f7 d0                    	notq	%r8
140feeb8d: 4f 8d 04 40                 	leaq	(%r8,%r8,2), %r8
140feeb91: 49 c1 e0 05                 	shlq	$0x5, %r8
140feeb95: 4b 83 7c 07 20 00           	cmpq	$0x0, 0x20(%r15,%r8)
140feeb9b: 0f 84 8c 04 00 00           	je	0x140fef02d <.text+0xfee02d>
140feeba1: 4c 8b 8e 60 01 00 00        	movq	0x160(%rsi), %r9
140feeba8: 4d 39 cb                    	cmpq	%r9, %r11
140feebab: 0f 86 9d 04 00 00           	jbe	0x140fef04e <.text+0xfee04e>
140feebb1: 4f 8b 0c ce                 	movq	(%r14,%r9,8), %r9
140feebb5: 4d 85 c9                    	testq	%r9, %r9
140feebb8: 0f 84 90 04 00 00           	je	0x140fef04e <.text+0xfee04e>
140feebbe: 49 f7 d1                    	notq	%r9
140feebc1: 4f 8d 0c 49                 	leaq	(%r9,%r9,2), %r9
140feebc5: 49 c1 e1 05                 	shlq	$0x5, %r9
140feebc9: 4b 83 7c 0f 20 00           	cmpq	$0x0, 0x20(%r15,%r9)
140feebcf: 0f 84 79 04 00 00           	je	0x140fef04e <.text+0xfee04e>
140feebd5: 4c 8b 96 68 01 00 00        	movq	0x168(%rsi), %r10
140feebdc: 4d 39 d3                    	cmpq	%r10, %r11
140feebdf: 0f 86 8a 04 00 00           	jbe	0x140fef06f <.text+0xfee06f>
140feebe5: 4f 8b 14 d6                 	movq	(%r14,%r10,8), %r10
140feebe9: 4d 85 d2                    	testq	%r10, %r10
140feebec: 0f 84 7d 04 00 00           	je	0x140fef06f <.text+0xfee06f>
140feebf2: 49 f7 d2                    	notq	%r10
140feebf5: 4f 8d 14 52                 	leaq	(%r10,%r10,2), %r10
140feebf9: 49 c1 e2 05                 	shlq	$0x5, %r10
140feebfd: 4b 83 7c 17 20 00           	cmpq	$0x0, 0x20(%r15,%r10)
140feec03: 0f 84 66 04 00 00           	je	0x140fef06f <.text+0xfee06f>
140feec09: 48 8b 9e 70 01 00 00        	movq	0x170(%rsi), %rbx
140feec10: 49 39 db                    	cmpq	%rbx, %r11
140feec13: 0f 86 77 04 00 00           	jbe	0x140fef090 <.text+0xfee090>
140feec19: 49 8b 1c de                 	movq	(%r14,%rbx,8), %rbx
140feec1d: 48 85 db                    	testq	%rbx, %rbx
140feec20: 0f 84 6a 04 00 00           	je	0x140fef090 <.text+0xfee090>
140feec26: 48 f7 d3                    	notq	%rbx
140feec29: 48 8d 1c 5b                 	leaq	(%rbx,%rbx,2), %rbx
140feec2d: 48 c1 e3 05                 	shlq	$0x5, %rbx
140feec31: 49 83 7c 1f 20 00           	cmpq	$0x0, 0x20(%r15,%rbx)
140feec37: 0f 84 53 04 00 00           	je	0x140fef090 <.text+0xfee090>
140feec3d: 4c 8b a6 78 01 00 00        	movq	0x178(%rsi), %r12
140feec44: 4d 39 e3                    	cmpq	%r12, %r11
140feec47: 0f 86 64 04 00 00           	jbe	0x140fef0b1 <.text+0xfee0b1>
140feec4d: 4f 8b 1c e6                 	movq	(%r14,%r12,8), %r11
140feec51: 4d 85 db                    	testq	%r11, %r11
140feec54: 0f 84 57 04 00 00           	je	0x140fef0b1 <.text+0xfee0b1>
140feec5a: 49 f7 d3                    	notq	%r11
140feec5d: 4f 8d 34 5b                 	leaq	(%r11,%r11,2), %r14
140feec61: 49 c1 e6 05                 	shlq	$0x5, %r14
140feec65: 4b 83 7c 37 20 00           	cmpq	$0x0, 0x20(%r15,%r14)
140feec6b: 0f 84 40 04 00 00           	je	0x140fef0b1 <.text+0xfee0b1>
140feec71: 4c 01 f8                    	addq	%r15, %rax
140feec74: 4c 8b 58 10                 	movq	0x10(%rax), %r11
140feec78: 4c 89 9c 24 b0 01 00 00     	movq	%r11, 0x1b0(%rsp)
140feec80: 44 8b 9e 30 01 00 00        	movl	0x130(%rsi), %r11d
140feec87: 4c 01 f9                    	addq	%r15, %rcx
140feec8a: 4c 8b 61 10                 	movq	0x10(%rcx), %r12
140feec8e: 4c 89 a4 24 b8 01 00 00     	movq	%r12, 0x1b8(%rsp)
140feec96: 4d 01 f8                    	addq	%r15, %r8
140feec99: 4d 8b 60 10                 	movq	0x10(%r8), %r12
140feec9d: 4c 89 a4 24 c0 01 00 00     	movq	%r12, 0x1c0(%rsp)
140feeca5: 4d 01 f9                    	addq	%r15, %r9
140feeca8: 4d 8b 61 10                 	movq	0x10(%r9), %r12
140feecac: 4c 89 a4 24 c8 01 00 00     	movq	%r12, 0x1c8(%rsp)
140feecb4: 4d 01 fa                    	addq	%r15, %r10
140feecb7: 4d 8b 62 10                 	movq	0x10(%r10), %r12
140feecbb: 4c 89 a4 24 d0 01 00 00     	movq	%r12, 0x1d0(%rsp)
140feecc3: 4c 01 fb                    	addq	%r15, %rbx
140feecc6: 4c 8b 63 10                 	movq	0x10(%rbx), %r12
140feecca: 4c 89 a4 24 d8 01 00 00     	movq	%r12, 0x1d8(%rsp)
140feecd2: 4d 01 fe                    	addq	%r15, %r14
140feecd5: 4d 8b 7e 10                 	movq	0x10(%r14), %r15
140feecd9: 4c 89 bc 24 e0 01 00 00     	movq	%r15, 0x1e0(%rsp)
140feece1: 8b aa b0 05 00 00           	movl	0x5b0(%rdx), %ebp
140feece7: 4c 8b a6 80 01 00 00        	movq	0x180(%rsi), %r12
140feecee: 4c 8b ba 18 02 00 00        	movq	0x218(%rdx), %r15
140feecf5: 4d 39 e7                    	cmpq	%r12, %r15
140feecf8: 0f 86 4d 04 00 00           	jbe	0x140fef14b <.text+0xfee14b>
140feecfe: 4c 8b aa 10 02 00 00        	movq	0x210(%rdx), %r13
140feed05: 4f 8b 64 e5 00              	movq	(%r13,%r12,8), %r12
140feed0a: 4d 85 e4                    	testq	%r12, %r12
140feed0d: 0f 84 38 04 00 00           	je	0x140fef14b <.text+0xfee14b>
140feed13: 48 8b 92 e0 01 00 00        	movq	0x1e0(%rdx), %rdx
140feed1a: 49 f7 d4                    	notq	%r12
140feed1d: 4f 8d 24 64                 	leaq	(%r12,%r12,2), %r12
140feed21: 49 c1 e4 05                 	shlq	$0x5, %r12
140feed25: 4a 83 7c 22 20 00           	cmpq	$0x0, 0x20(%rdx,%r12)
140feed2b: 0f 84 1a 04 00 00           	je	0x140fef14b <.text+0xfee14b>
140feed31: 48 8b ae 88 01 00 00        	movq	0x188(%rsi), %rbp
140feed38: 49 39 ef                    	cmpq	%rbp, %r15
140feed3b: 0f 86 28 04 00 00           	jbe	0x140fef169 <.text+0xfee169>
140feed41: 4d 8b 7c ed 00              	movq	(%r13,%rbp,8), %r15
140feed46: 4d 85 ff                    	testq	%r15, %r15
140feed49: 0f 84 1a 04 00 00           	je	0x140fef169 <.text+0xfee169>
140feed4f: 49 f7 d7                    	notq	%r15
140feed52: 4b 8d 2c 7f                 	leaq	(%r15,%r15,2), %rbp
140feed56: 48 c1 e5 05                 	shlq	$0x5, %rbp
140feed5a: 48 83 7c 2a 20 00           	cmpq	$0x0, 0x20(%rdx,%rbp)
140feed60: 0f 84 03 04 00 00           	je	0x140fef169 <.text+0xfee169>
140feed66: 49 01 d4                    	addq	%rdx, %r12
140feed69: 48 01 d5                    	addq	%rdx, %rbp
140feed6c: 4d 8b 7c 24 10              	movq	0x10(%r12), %r15
140feed71: 4c 8b 6d 10                 	movq	0x10(%rbp), %r13
140feed75: 48 8b 94 24 b0 01 00 00     	movq	0x1b0(%rsp), %rdx
140feed7d: 48 89 94 24 88 00 00 00     	movq	%rdx, 0x88(%rsp)
140feed85: 48 8d 50 5c                 	leaq	0x5c(%rax), %rdx
140feed89: 48 83 c0 58                 	addq	$0x58, %rax
140feed8d: 48 89 84 24 90 00 00 00     	movq	%rax, 0x90(%rsp)
140feed95: 48 89 94 24 98 00 00 00     	movq	%rdx, 0x98(%rsp)
140feed9d: 44 89 9c 24 a0 00 00 00     	movl	%r11d, 0xa0(%rsp)
140feeda5: 89 bc 24 a4 00 00 00        	movl	%edi, 0xa4(%rsp)
140feedac: 48 8b 84 24 b8 01 00 00     	movq	0x1b8(%rsp), %rax
140feedb4: 48 89 84 24 a8 00 00 00     	movq	%rax, 0xa8(%rsp)
140feedbc: 48 8d 41 5c                 	leaq	0x5c(%rcx), %rax
140feedc0: 48 83 c1 58                 	addq	$0x58, %rcx
140feedc4: 48 89 8c 24 b0 00 00 00     	movq	%rcx, 0xb0(%rsp)
140feedcc: 48 89 84 24 b8 00 00 00     	movq	%rax, 0xb8(%rsp)
140feedd4: 44 89 9c 24 c0 00 00 00     	movl	%r11d, 0xc0(%rsp)
140feeddc: 89 bc 24 c4 00 00 00        	movl	%edi, 0xc4(%rsp)
140feede3: 48 8b 84 24 c0 01 00 00     	movq	0x1c0(%rsp), %rax
140feedeb: 48 89 84 24 c8 00 00 00     	movq	%rax, 0xc8(%rsp)
140feedf3: 49 8d 40 5c                 	leaq	0x5c(%r8), %rax
140feedf7: 49 83 c0 58                 	addq	$0x58, %r8
140feedfb: 4c 89 84 24 d0 00 00 00     	movq	%r8, 0xd0(%rsp)
140feee03: 48 89 84 24 d8 00 00 00     	movq	%rax, 0xd8(%rsp)
140feee0b: 44 89 9c 24 e0 00 00 00     	movl	%r11d, 0xe0(%rsp)
140feee13: 89 bc 24 e4 00 00 00        	movl	%edi, 0xe4(%rsp)
140feee1a: 48 8b 84 24 c8 01 00 00     	movq	0x1c8(%rsp), %rax
140feee22: 48 89 84 24 e8 00 00 00     	movq	%rax, 0xe8(%rsp)
140feee2a: 49 8d 41 5c                 	leaq	0x5c(%r9), %rax
140feee2e: 49 83 c1 58                 	addq	$0x58, %r9
140feee32: 4c 89 8c 24 f0 00 00 00     	movq	%r9, 0xf0(%rsp)
140feee3a: 48 89 84 24 f8 00 00 00     	movq	%rax, 0xf8(%rsp)
140feee42: 44 89 9c 24 00 01 00 00     	movl	%r11d, 0x100(%rsp)
140feee4a: 89 bc 24 04 01 00 00        	movl	%edi, 0x104(%rsp)
140feee51: 48 8b 84 24 d0 01 00 00     	movq	0x1d0(%rsp), %rax
140feee59: 48 89 84 24 08 01 00 00     	movq	%rax, 0x108(%rsp)
140feee61: 49 8d 42 5c                 	leaq	0x5c(%r10), %rax
140feee65: 49 83 c2 58                 	addq	$0x58, %r10
140feee69: 4c 89 94 24 10 01 00 00     	movq	%r10, 0x110(%rsp)
140feee71: 48 89 84 24 18 01 00 00     	movq	%rax, 0x118(%rsp)
140feee79: 44 89 9c 24 20 01 00 00     	movl	%r11d, 0x120(%rsp)
140feee81: 89 bc 24 24 01 00 00        	movl	%edi, 0x124(%rsp)
140feee88: 48 8b 84 24 d8 01 00 00     	movq	0x1d8(%rsp), %rax
140feee90: 48 89 84 24 28 01 00 00     	movq	%rax, 0x128(%rsp)
140feee98: 48 8d 4b 5c                 	leaq	0x5c(%rbx), %rcx
140feee9c: 48 83 c3 58                 	addq	$0x58, %rbx
140feeea0: 48 89 9c 24 30 01 00 00     	movq	%rbx, 0x130(%rsp)
140feeea8: 48 8d 86 38 01 00 00        	leaq	0x138(%rsi), %rax
140feeeaf: 48 89 8c 24 38 01 00 00     	movq	%rcx, 0x138(%rsp)
140feeeb7: 49 8d 4c 24 5c              	leaq	0x5c(%r12), %rcx
140feeebc: 49 83 c4 58                 	addq	$0x58, %r12
140feeec0: 44 89 9c 24 40 01 00 00     	movl	%r11d, 0x140(%rsp)
140feeec8: 89 bc 24 44 01 00 00        	movl	%edi, 0x144(%rsp)
140feeecf: 48 8b 94 24 e0 01 00 00     	movq	0x1e0(%rsp), %rdx
140feeed7: 48 89 94 24 48 01 00 00     	movq	%rdx, 0x148(%rsp)
140feeedf: 49 8d 56 5c                 	leaq	0x5c(%r14), %rdx
140feeee3: 49 83 c6 58                 	addq	$0x58, %r14
140feeee7: 4c 89 b4 24 50 01 00 00     	movq	%r14, 0x150(%rsp)
140feeeef: 4c 8d 55 58                 	leaq	0x58(%rbp), %r10
140feeef3: 48 83 c5 5c                 	addq	$0x5c, %rbp
140feeef7: 48 89 94 24 58 01 00 00     	movq	%rdx, 0x158(%rsp)
140feeeff: 48 8d 94 24 a8 00 00 00     	leaq	0xa8(%rsp), %rdx
140feef07: 44 89 9c 24 60 01 00 00     	movl	%r11d, 0x160(%rsp)
140feef0f: 89 bc 24 64 01 00 00        	movl	%edi, 0x164(%rsp)
140feef16: 4c 89 bc 24 68 01 00 00     	movq	%r15, 0x168(%rsp)
140feef1e: 4c 8d 84 24 c8 00 00 00     	leaq	0xc8(%rsp), %r8
140feef26: 4c 89 a4 24 70 01 00 00     	movq	%r12, 0x170(%rsp)
140feef2e: 4c 8d 8c 24 e8 00 00 00     	leaq	0xe8(%rsp), %r9
140feef36: 48 89 8c 24 78 01 00 00     	movq	%rcx, 0x178(%rsp)
140feef3e: 48 8d 8c 24 08 01 00 00     	leaq	0x108(%rsp), %rcx
140feef46: 44 89 9c 24 80 01 00 00     	movl	%r11d, 0x180(%rsp)
140feef4e: 89 bc 24 84 01 00 00        	movl	%edi, 0x184(%rsp)
140feef55: 48 89 84 24 88 01 00 00     	movq	%rax, 0x188(%rsp)
140feef5d: 4c 89 ac 24 90 01 00 00     	movq	%r13, 0x190(%rsp)
140feef65: 48 8d 9c 24 28 01 00 00     	leaq	0x128(%rsp), %rbx
140feef6d: 4c 89 94 24 98 01 00 00     	movq	%r10, 0x198(%rsp)
140feef75: 4c 8d 94 24 48 01 00 00     	leaq	0x148(%rsp), %r10
140feef7d: 48 89 ac 24 a0 01 00 00     	movq	%rbp, 0x1a0(%rsp)
140feef85: 4c 8d b4 24 68 01 00 00     	leaq	0x168(%rsp), %r14
140feef8d: 44 89 9c 24 a8 01 00 00     	movl	%r11d, 0x1a8(%rsp)
140feef95: 4c 8d 9c 24 90 01 00 00     	leaq	0x190(%rsp), %r11
140feef9d: 89 bc 24 ac 01 00 00        	movl	%edi, 0x1ac(%rsp)
140feefa4: 4c 89 5c 24 48              	movq	%r11, 0x48(%rsp)
140feefa9: 48 89 44 24 40              	movq	%rax, 0x40(%rsp)
140feefae: 4c 89 74 24 38              	movq	%r14, 0x38(%rsp)
140feefb3: 4c 89 54 24 30              	movq	%r10, 0x30(%rsp)
140feefb8: 48 89 5c 24 28              	movq	%rbx, 0x28(%rsp)
140feefbd: 48 89 4c 24 20              	movq	%rcx, 0x20(%rsp)
140feefc2: 48 8d 8c 24 88 00 00 00     	leaq	0x88(%rsp), %rcx
140feefca: e8 71 d5 eb 00              	callq	0x141eac540 <.text+0x1eab540>
140feefcf: 89 be 30 01 00 00           	movl	%edi, 0x130(%rsi)
140feefd5: 31 c0                       	xorl	%eax, %eax
140feefd7: 48 81 c4 e8 01 00 00        	addq	$0x1e8, %rsp            # imm = 0x1E8
140feefde: 5b                          	popq	%rbx
140feefdf: 5d                          	popq	%rbp
140feefe0: 5f                          	popq	%rdi
140feefe1: 5e                          	popq	%rsi
140feefe2: 41 5c                       	popq	%r12
140feefe4: 41 5d                       	popq	%r13
140feefe6: 41 5e                       	popq	%r14
140feefe8: 41 5f                       	popq	%r15
140feefea: c3                          	retq
140feefeb: 48 81 c6 98 00 00 00        	addq	$0x98, %rsi
140feeff2: 48 8d 05 32 45 bd 01        	leaq	0x1bd4532(%rip), %rax   # 0x142bc352b
140feeff9: 48 89 44 24 58              	movq	%rax, 0x58(%rsp)
140feeffe: 48 c7 44 24 60 2c 00 00 00  	movq	$0x2c, 0x60(%rsp)
140fef007: e9 79 01 00 00              	jmp	0x140fef185 <.text+0xfee185>
140fef00c: 48 81 c6 98 00 00 00        	addq	$0x98, %rsi
140fef013: 48 8d 05 c0 45 bd 01        	leaq	0x1bd45c0(%rip), %rax   # 0x142bc35da
140fef01a: 48 89 44 24 58              	movq	%rax, 0x58(%rsp)
140fef01f: 48 c7 44 24 60 2d 00 00 00  	movq	$0x2d, 0x60(%rsp)
140fef028: e9 58 01 00 00              	jmp	0x140fef185 <.text+0xfee185>
140fef02d: 48 81 c6 98 00 00 00        	addq	$0x98, %rsi
140fef034: 48 8d 05 c5 44 bd 01        	leaq	0x1bd44c5(%rip), %rax   # 0x142bc3500
140fef03b: 48 89 44 24 58              	movq	%rax, 0x58(%rsp)
140fef040: 48 c7 44 24 60 2b 00 00 00  	movq	$0x2b, 0x60(%rsp)
140fef049: e9 37 01 00 00              	jmp	0x140fef185 <.text+0xfee185>
140fef04e: 48 81 c6 98 00 00 00        	addq	$0x98, %rsi
140fef055: 48 8d 05 fb 44 bd 01        	leaq	0x1bd44fb(%rip), %rax   # 0x142bc3557
140fef05c: 48 89 44 24 58              	movq	%rax, 0x58(%rsp)
140fef061: 48 c7 44 24 60 32 00 00 00  	movq	$0x32, 0x60(%rsp)
140fef06a: e9 16 01 00 00              	jmp	0x140fef185 <.text+0xfee185>
140fef06f: 48 81 c6 98 00 00 00        	addq	$0x98, %rsi
140fef076: 48 8d 05 6e 43 bd 01        	leaq	0x1bd436e(%rip), %rax   # 0x142bc33eb
140fef07d: 48 89 44 24 58              	movq	%rax, 0x58(%rsp)
140fef082: 48 c7 44 24 60 4a 00 00 00  	movq	$0x4a, 0x60(%rsp)
140fef08b: e9 f5 00 00 00              	jmp	0x140fef185 <.text+0xfee185>
140fef090: 48 81 c6 98 00 00 00        	addq	$0x98, %rsi
140fef097: 48 8d 05 eb 44 bd 01        	leaq	0x1bd44eb(%rip), %rax   # 0x142bc3589
140fef09e: 48 89 44 24 58              	movq	%rax, 0x58(%rsp)
140fef0a3: 48 c7 44 24 60 27 00 00 00  	movq	$0x27, 0x60(%rsp)
140fef0ac: e9 d4 00 00 00              	jmp	0x140fef185 <.text+0xfee185>
140fef0b1: 48 81 c6 98 00 00 00        	addq	$0x98, %rsi
140fef0b8: 48 8d 05 bf 45 bd 01        	leaq	0x1bd45bf(%rip), %rax   # 0x142bc367e
140fef0bf: 48 89 44 24 58              	movq	%rax, 0x58(%rsp)
140fef0c4: 48 c7 44 24 60 1e 00 00 00  	movq	$0x1e, 0x60(%rsp)
140fef0cd: 48 89 74 24 68              	movq	%rsi, 0x68(%rsp)
140fef0d2: 48 8d 05 f7 c9 04 ff        	leaq	-0xfb3609(%rip), %rax   # 0x14003bad0 <.text+0x3aad0>
140fef0d9: 48 89 44 24 70              	movq	%rax, 0x70(%rsp)
140fef0de: 48 8d 44 24 58              	leaq	0x58(%rsp), %rax
140fef0e3: 48 89 44 24 78              	movq	%rax, 0x78(%rsp)
140fef0e8: 48 8d 05 a1 ad 11 ff        	leaq	-0xee525f(%rip), %rax   # 0x140109e90 <.text+0x108e90>
140fef0ef: 48 89 84 24 80 00 00 00     	movq	%rax, 0x80(%rsp)
140fef0f7: 48 8d 05 62 43 bd 01        	leaq	0x1bd4362(%rip), %rax   # 0x142bc3460
140fef0fe: 48 89 84 24 88 00 00 00     	movq	%rax, 0x88(%rsp)
140fef106: 48 c7 84 24 90 00 00 00 02 00 00 00 	movq	$0x2, 0x90(%rsp)
140fef112: 48 c7 84 24 a8 00 00 00 00 00 00 00 	movq	$0x0, 0xa8(%rsp)
140fef11e: 48 8d 44 24 68              	leaq	0x68(%rsp), %rax
140fef123: 48 89 84 24 98 00 00 00     	movq	%rax, 0x98(%rsp)
140fef12b: 48 c7 84 24 a0 00 00 00 02 00 00 00 	movq	$0x2, 0xa0(%rsp)
140fef137: 48 8d 15 62 45 bd 01        	leaq	0x1bd4562(%rip), %rdx   # 0x142bc36a0
140fef13e: 48 8d 8c 24 88 00 00 00     	leaq	0x88(%rsp), %rcx
140fef146: e8 45 a2 8e 01              	callq	0x1428d9390 <.text+0x28d8390>
140fef14b: 48 81 c6 98 00 00 00        	addq	$0x98, %rsi
140fef152: 48 8d 05 ae 44 bd 01        	leaq	0x1bd44ae(%rip), %rax   # 0x142bc3607
140fef159: 48 89 44 24 58              	movq	%rax, 0x58(%rsp)
140fef15e: 48 c7 44 24 60 77 00 00 00  	movq	$0x77, 0x60(%rsp)
140fef167: eb 1c                       	jmp	0x140fef185 <.text+0xfee185>
140fef169: 48 81 c6 98 00 00 00        	addq	$0x98, %rsi
140fef170: 48 8d 05 39 44 bd 01        	leaq	0x1bd4439(%rip), %rax   # 0x142bc35b0
140fef177: 48 89 44 24 58              	movq	%rax, 0x58(%rsp)
140fef17c: 48 c7 44 24 60 2a 00 00 00  	movq	$0x2a, 0x60(%rsp)
140fef185: 48 89 74 24 68              	movq	%rsi, 0x68(%rsp)
140fef18a: 48 8d 05 3f c9 04 ff        	leaq	-0xfb36c1(%rip), %rax   # 0x14003bad0 <.text+0x3aad0>
140fef191: 48 89 44 24 70              	movq	%rax, 0x70(%rsp)
140fef196: 48 8d 44 24 58              	leaq	0x58(%rsp), %rax
140fef19b: 48 89 44 24 78              	movq	%rax, 0x78(%rsp)
140fef1a0: 48 8d 05 e9 ac 11 ff        	leaq	-0xee5317(%rip), %rax   # 0x140109e90 <.text+0x108e90>
140fef1a7: 48 89 84 24 80 00 00 00     	movq	%rax, 0x80(%rsp)
140fef1af: 48 8d 05 aa 42 bd 01        	leaq	0x1bd42aa(%rip), %rax   # 0x142bc3460
140fef1b6: 48 89 84 24 88 00 00 00     	movq	%rax, 0x88(%rsp)
140fef1be: 48 c7 84 24 90 00 00 00 02 00 00 00 	movq	$0x2, 0x90(%rsp)
140fef1ca: 48 c7 84 24 a8 00 00 00 00 00 00 00 	movq	$0x0, 0xa8(%rsp)
140fef1d6: 48 8d 44 24 68              	leaq	0x68(%rsp), %rax
140fef1db: 48 89 84 24 98 00 00 00     	movq	%rax, 0x98(%rsp)
140fef1e3: 48 c7 84 24 a0 00 00 00 02 00 00 00 	movq	$0x2, 0xa0(%rsp)
140fef1ef: 48 8d 15 f2 42 bd 01        	leaq	0x1bd42f2(%rip), %rdx   # 0x142bc34e8
140fef1f6: 48 8d 8c 24 88 00 00 00     	leaq	0x88(%rsp), %rcx
140fef1fe: e8 8d a1 8e 01              	callq	0x1428d9390 <.text+0x28d8390>
140fef203: 48 8d 0d 8e 45 bd 01        	leaq	0x1bd458e(%rip), %rcx   # 0x142bc3798
140fef20a: 4c 8d 05 e7 45 bd 01        	leaq	0x1bd45e7(%rip), %r8    # 0x142bc37f8
140fef211: ba 59 00 00 00              	movl	$0x59, %edx
140fef216: e8 15 a1 8e 01              	callq	0x1428d9330 <.text+0x28d8330>
140fef21b: cc                          	int3
140fef21c: cc                          	int3
140fef21d: cc                          	int3
140fef21e: cc                          	int3
140fef21f: cc                          	int3
140fef220: 48 83 ec 28                 	subq	$0x28, %rsp
140fef224: 83 b9 38 01 00 00 02        	cmpl	$0x2, 0x138(%rcx)
140fef22b: 74 05                       	je	0x140fef232 <.text+0xfee232>
140fef22d: 48 83 c4 28                 	addq	$0x28, %rsp
140fef231: c3                          	retq
140fef232: 48 8d 0d 5f 45 bd 01        	leaq	0x1bd455f(%rip), %rcx   # 0x142bc3798
140fef239: 4c 8d 05 d0 45 bd 01        	leaq	0x1bd45d0(%rip), %r8    # 0x142bc3810
140fef240: ba 59 00 00 00              	movl	$0x59, %edx
140fef245: e8 e6 a0 8e 01              	callq	0x1428d9330 <.text+0x28d8330>
140fef24a: cc                          	int3
140fef24b: cc                          	int3
140fef24c: cc                          	int3
140fef24d: cc                          	int3
140fef24e: cc                          	int3
140fef24f: cc                          	int3
140fef250: 48 83 ec 28                 	subq	$0x28, %rsp
140fef254: 83 b9 38 01 00 00 02        	cmpl	$0x2, 0x138(%rcx)
140fef25b: 74 05                       	je	0x140fef262 <.text+0xfee262>
140fef25d: 48 83 c4 28                 	addq	$0x28, %rsp
140fef261: c3                          	retq
140fef262: 48 8d 0d 2f 45 bd 01        	leaq	0x1bd452f(%rip), %rcx   # 0x142bc3798
140fef269: 4c 8d 05 b8 45 bd 01        	leaq	0x1bd45b8(%rip), %r8    # 0x142bc3828
140fef270: ba 59 00 00 00              	movl	$0x59, %edx
140fef275: e8 b6 a0 8e 01              	callq	0x1428d9330 <.text+0x28d8330>
140fef27a: cc                          	int3
140fef27b: cc                          	int3
140fef27c: cc                          	int3
140fef27d: cc                          	int3
140fef27e: cc                          	int3
140fef27f: cc                          	int3
140fef280: 48 83 ec 28                 	subq	$0x28, %rsp
140fef284: 83 ba 38 01 00 00 02        	cmpl	$0x2, 0x138(%rdx)
140fef28b: 0f 84 32 04 00 00           	je	0x140fef6c3 <.text+0xfee6c3>
140fef291: 48 8b 82 48 01 00 00        	movq	0x148(%rdx), %rax
140fef298: 4d 8b 88 18 02 00 00        	movq	0x218(%r8), %r9
140fef29f: 49 39 c1                    	cmpq	%rax, %r9
140fef2a2: 0f 86 e7 01 00 00           	jbe	0x140fef48f <.text+0xfee48f>
140fef2a8: 4d 8b 90 10 02 00 00        	movq	0x210(%r8), %r10
140fef2af: 4d 8b 1c c2                 	movq	(%r10,%rax,8), %r11
140fef2b3: 4d 85 db                    	testq	%r11, %r11
140fef2b6: 0f 84 d3 01 00 00           	je	0x140fef48f <.text+0xfee48f>
140fef2bc: 49 8b 80 e0 01 00 00        	movq	0x1e0(%r8), %rax
140fef2c3: 49 f7 d3                    	notq	%r11
140fef2c6: 4f 8d 04 5b                 	leaq	(%r11,%r11,2), %r8
140fef2ca: 49 c1 e0 05                 	shlq	$0x5, %r8
140fef2ce: 4a 83 7c 00 20 00           	cmpq	$0x0, 0x20(%rax,%r8)
140fef2d4: 0f 84 b5 01 00 00           	je	0x140fef48f <.text+0xfee48f>
140fef2da: 4c 8b 82 50 01 00 00        	movq	0x150(%rdx), %r8
140fef2e1: 4d 39 c1                    	cmpq	%r8, %r9
140fef2e4: 0f 86 fc 01 00 00           	jbe	0x140fef4e6 <.text+0xfee4e6>
140fef2ea: 4f 8b 04 c2                 	movq	(%r10,%r8,8), %r8
140fef2ee: 4d 85 c0                    	testq	%r8, %r8
140fef2f1: 0f 84 ef 01 00 00           	je	0x140fef4e6 <.text+0xfee4e6>
140fef2f7: 49 f7 d0                    	notq	%r8
140fef2fa: 4f 8d 04 40                 	leaq	(%r8,%r8,2), %r8
140fef2fe: 49 c1 e0 05                 	shlq	$0x5, %r8
140fef302: 4a 83 7c 00 20 00           	cmpq	$0x0, 0x20(%rax,%r8)
140fef308: 0f 84 d8 01 00 00           	je	0x140fef4e6 <.text+0xfee4e6>
140fef30e: 4c 8b 82 58 01 00 00        	movq	0x158(%rdx), %r8
140fef315: 4d 39 c1                    	cmpq	%r8, %r9
140fef318: 0f 86 01 02 00 00           	jbe	0x140fef51f <.text+0xfee51f>
140fef31e: 4f 8b 04 c2                 	movq	(%r10,%r8,8), %r8
140fef322: 4d 85 c0                    	testq	%r8, %r8
140fef325: 0f 84 f4 01 00 00           	je	0x140fef51f <.text+0xfee51f>
140fef32b: 49 f7 d0                    	notq	%r8
140fef32e: 4f 8d 04 40                 	leaq	(%r8,%r8,2), %r8
140fef332: 49 c1 e0 05                 	shlq	$0x5, %r8
140fef336: 4a 83 7c 00 20 00           	cmpq	$0x0, 0x20(%rax,%r8)
140fef33c: 0f 84 dd 01 00 00           	je	0x140fef51f <.text+0xfee51f>
140fef342: 4c 8b 82 60 01 00 00        	movq	0x160(%rdx), %r8
140fef349: 4d 39 c1                    	cmpq	%r8, %r9
140fef34c: 0f 86 09 02 00 00           	jbe	0x140fef55b <.text+0xfee55b>
140fef352: 4f 8b 04 c2                 	movq	(%r10,%r8,8), %r8
140fef356: 4d 85 c0                    	testq	%r8, %r8
140fef359: 0f 84 fc 01 00 00           	je	0x140fef55b <.text+0xfee55b>
140fef35f: 49 f7 d0                    	notq	%r8
140fef362: 4f 8d 04 40                 	leaq	(%r8,%r8,2), %r8
140fef366: 49 c1 e0 05                 	shlq	$0x5, %r8
140fef36a: 4a 83 7c 00 20 00           	cmpq	$0x0, 0x20(%rax,%r8)
140fef370: 0f 84 e5 01 00 00           	je	0x140fef55b <.text+0xfee55b>
140fef376: 4c 8b 82 68 01 00 00        	movq	0x168(%rdx), %r8
140fef37d: 4d 39 c1                    	cmpq	%r8, %r9
140fef380: 0f 86 11 02 00 00           	jbe	0x140fef597 <.text+0xfee597>
140fef386: 4f 8b 04 c2                 	movq	(%r10,%r8,8), %r8
140fef38a: 4d 85 c0                    	testq	%r8, %r8
140fef38d: 0f 84 04 02 00 00           	je	0x140fef597 <.text+0xfee597>
140fef393: 49 f7 d0                    	notq	%r8
140fef396: 4f 8d 04 40                 	leaq	(%r8,%r8,2), %r8
140fef39a: 49 c1 e0 05                 	shlq	$0x5, %r8
140fef39e: 4a 83 7c 00 20 00           	cmpq	$0x0, 0x20(%rax,%r8)
140fef3a4: 0f 84 ed 01 00 00           	je	0x140fef597 <.text+0xfee597>
140fef3aa: 4c 8b 82 70 01 00 00        	movq	0x170(%rdx), %r8
140fef3b1: 4d 39 c1                    	cmpq	%r8, %r9
140fef3b4: 0f 86 19 02 00 00           	jbe	0x140fef5d3 <.text+0xfee5d3>
140fef3ba: 4f 8b 04 c2                 	movq	(%r10,%r8,8), %r8
140fef3be: 4d 85 c0                    	testq	%r8, %r8
140fef3c1: 0f 84 0c 02 00 00           	je	0x140fef5d3 <.text+0xfee5d3>
140fef3c7: 49 f7 d0                    	notq	%r8
140fef3ca: 4f 8d 04 40                 	leaq	(%r8,%r8,2), %r8
140fef3ce: 49 c1 e0 05                 	shlq	$0x5, %r8
140fef3d2: 4a 83 7c 00 20 00           	cmpq	$0x0, 0x20(%rax,%r8)
140fef3d8: 0f 84 f5 01 00 00           	je	0x140fef5d3 <.text+0xfee5d3>
140fef3de: 4c 8b 82 78 01 00 00        	movq	0x178(%rdx), %r8
140fef3e5: 4d 39 c1                    	cmpq	%r8, %r9
140fef3e8: 0f 86 21 02 00 00           	jbe	0x140fef60f <.text+0xfee60f>
140fef3ee: 4f 8b 04 c2                 	movq	(%r10,%r8,8), %r8
140fef3f2: 4d 85 c0                    	testq	%r8, %r8
140fef3f5: 0f 84 14 02 00 00           	je	0x140fef60f <.text+0xfee60f>
140fef3fb: 49 f7 d0                    	notq	%r8
140fef3fe: 4f 8d 04 40                 	leaq	(%r8,%r8,2), %r8
140fef402: 49 c1 e0 05                 	shlq	$0x5, %r8
140fef406: 4a 83 7c 00 20 00           	cmpq	$0x0, 0x20(%rax,%r8)
140fef40c: 0f 84 fd 01 00 00           	je	0x140fef60f <.text+0xfee60f>
140fef412: 4c 8b 82 80 01 00 00        	movq	0x180(%rdx), %r8
140fef419: 4d 39 c1                    	cmpq	%r8, %r9
140fef41c: 0f 86 29 02 00 00           	jbe	0x140fef64b <.text+0xfee64b>
140fef422: 4f 8b 04 c2                 	movq	(%r10,%r8,8), %r8
140fef426: 4d 85 c0                    	testq	%r8, %r8
140fef429: 0f 84 1c 02 00 00           	je	0x140fef64b <.text+0xfee64b>
140fef42f: 49 f7 d0                    	notq	%r8
140fef432: 4f 8d 04 40                 	leaq	(%r8,%r8,2), %r8
140fef436: 49 c1 e0 05                 	shlq	$0x5, %r8
140fef43a: 4a 83 7c 00 20 00           	cmpq	$0x0, 0x20(%rax,%r8)
140fef440: 0f 84 05 02 00 00           	je	0x140fef64b <.text+0xfee64b>
140fef446: 48 8b 92 88 01 00 00        	movq	0x188(%rdx), %rdx
140fef44d: 49 39 d1                    	cmpq	%rdx, %r9
140fef450: 0f 86 31 02 00 00           	jbe	0x140fef687 <.text+0xfee687>
140fef456: 49 8b 14 d2                 	movq	(%r10,%rdx,8), %rdx
140fef45a: 48 85 d2                    	testq	%rdx, %rdx
140fef45d: 0f 84 24 02 00 00           	je	0x140fef687 <.text+0xfee687>
140fef463: 48 f7 d2                    	notq	%rdx
140fef466: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
140fef46a: 48 c1 e2 05                 	shlq	$0x5, %rdx
140fef46e: 48 83 7c 10 20 00           	cmpq	$0x0, 0x20(%rax,%rdx)
140fef474: 0f 84 0d 02 00 00           	je	0x140fef687 <.text+0xfee687>
140fef47a: 48 b8 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %rax # imm = 0x8000000000000001
140fef484: 48 89 01                    	movq	%rax, (%rcx)
140fef487: 48 89 c8                    	movq	%rcx, %rax
140fef48a: 48 83 c4 28                 	addq	$0x28, %rsp
140fef48e: c3                          	retq
140fef48f: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140fef499: 48 89 01                    	movq	%rax, (%rcx)
140fef49c: 48 8d 15 31 3f bd 01        	leaq	0x1bd3f31(%rip), %rdx   # 0x142bc33d4
140fef4a3: 48 89 51 08                 	movq	%rdx, 0x8(%rcx)
140fef4a7: 48 c7 41 10 17 00 00 00     	movq	$0x17, 0x10(%rcx)
140fef4af: 48 89 41 18                 	movq	%rax, 0x18(%rcx)
140fef4b3: 48 8d 15 e1 36 f9 01        	leaq	0x1f936e1(%rip), %rdx   # 0x142f82b9b
140fef4ba: 48 89 51 20                 	movq	%rdx, 0x20(%rcx)
140fef4be: 48 c7 41 28 4d 00 00 00     	movq	$0x4d, 0x28(%rcx)
140fef4c6: 48 89 41 30                 	movq	%rax, 0x30(%rcx)
140fef4ca: 48 c7 41 38 01 00 00 00     	movq	$0x1, 0x38(%rcx)
140fef4d2: 48 c7 41 40 00 00 00 00     	movq	$0x0, 0x40(%rcx)
140fef4da: c6 41 48 00                 	movb	$0x0, 0x48(%rcx)
140fef4de: 48 89 c8                    	movq	%rcx, %rax
140fef4e1: 48 83 c4 28                 	addq	$0x28, %rsp
140fef4e5: c3                          	retq
140fef4e6: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140fef4f0: 48 89 01                    	movq	%rax, (%rcx)
140fef4f3: 48 8d 15 da 3e bd 01        	leaq	0x1bd3eda(%rip), %rdx   # 0x142bc33d4
140fef4fa: 48 89 51 08                 	movq	%rdx, 0x8(%rcx)
140fef4fe: 48 c7 41 10 17 00 00 00     	movq	$0x17, 0x10(%rcx)
140fef506: 48 89 41 18                 	movq	%rax, 0x18(%rcx)
140fef50a: 48 8d 15 bc 27 f9 01        	leaq	0x1f927bc(%rip), %rdx   # 0x142f81ccd
140fef511: 48 89 51 20                 	movq	%rdx, 0x20(%rcx)
140fef515: 48 c7 41 28 4e 00 00 00     	movq	$0x4e, 0x28(%rcx)
140fef51d: eb a7                       	jmp	0x140fef4c6 <.text+0xfee4c6>
140fef51f: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140fef529: 48 89 01                    	movq	%rax, (%rcx)
140fef52c: 48 8d 15 a1 3e bd 01        	leaq	0x1bd3ea1(%rip), %rdx   # 0x142bc33d4
140fef533: 48 89 51 08                 	movq	%rdx, 0x8(%rcx)
140fef537: 48 c7 41 10 17 00 00 00     	movq	$0x17, 0x10(%rcx)
140fef53f: 48 89 41 18                 	movq	%rax, 0x18(%rcx)
140fef543: 48 8d 15 1a 2a f9 01        	leaq	0x1f92a1a(%rip), %rdx   # 0x142f81f64
140fef54a: 48 89 51 20                 	movq	%rdx, 0x20(%rcx)
140fef54e: 48 c7 41 28 4c 00 00 00     	movq	$0x4c, 0x28(%rcx)
140fef556: e9 6b ff ff ff              	jmp	0x140fef4c6 <.text+0xfee4c6>
140fef55b: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140fef565: 48 89 01                    	movq	%rax, (%rcx)
140fef568: 48 8d 15 65 3e bd 01        	leaq	0x1bd3e65(%rip), %rdx   # 0x142bc33d4
140fef56f: 48 89 51 08                 	movq	%rdx, 0x8(%rcx)
140fef573: 48 c7 41 10 17 00 00 00     	movq	$0x17, 0x10(%rcx)
140fef57b: 48 89 41 18                 	movq	%rax, 0x18(%rcx)
140fef57f: 48 8d 15 49 21 f9 01        	leaq	0x1f92149(%rip), %rdx   # 0x142f816cf
140fef586: 48 89 51 20                 	movq	%rdx, 0x20(%rcx)
140fef58a: 48 c7 41 28 53 00 00 00     	movq	$0x53, 0x28(%rcx)
140fef592: e9 2f ff ff ff              	jmp	0x140fef4c6 <.text+0xfee4c6>
140fef597: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140fef5a1: 48 89 01                    	movq	%rax, (%rcx)
140fef5a4: 48 8d 15 29 3e bd 01        	leaq	0x1bd3e29(%rip), %rdx   # 0x142bc33d4
140fef5ab: 48 89 51 08                 	movq	%rdx, 0x8(%rcx)
140fef5af: 48 c7 41 10 17 00 00 00     	movq	$0x17, 0x10(%rcx)
140fef5b7: 48 89 41 18                 	movq	%rax, 0x18(%rcx)
140fef5bb: 48 8d 15 22 1a f9 01        	leaq	0x1f91a22(%rip), %rdx   # 0x142f80fe4
140fef5c2: 48 89 51 20                 	movq	%rdx, 0x20(%rcx)
140fef5c6: 48 c7 41 28 6b 00 00 00     	movq	$0x6b, 0x28(%rcx)
140fef5ce: e9 f3 fe ff ff              	jmp	0x140fef4c6 <.text+0xfee4c6>
140fef5d3: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140fef5dd: 48 89 01                    	movq	%rax, (%rcx)
140fef5e0: 48 8d 15 ed 3d bd 01        	leaq	0x1bd3ded(%rip), %rdx   # 0x142bc33d4
140fef5e7: 48 89 51 08                 	movq	%rdx, 0x8(%rcx)
140fef5eb: 48 c7 41 10 17 00 00 00     	movq	$0x17, 0x10(%rcx)
140fef5f3: 48 89 41 18                 	movq	%rax, 0x18(%rcx)
140fef5f7: 48 8d 15 7a 1b f9 01        	leaq	0x1f91b7a(%rip), %rdx   # 0x142f81178
140fef5fe: 48 89 51 20                 	movq	%rdx, 0x20(%rcx)
140fef602: 48 c7 41 28 48 00 00 00     	movq	$0x48, 0x28(%rcx)
140fef60a: e9 b7 fe ff ff              	jmp	0x140fef4c6 <.text+0xfee4c6>
140fef60f: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140fef619: 48 89 01                    	movq	%rax, (%rcx)
140fef61c: 48 8d 15 b1 3d bd 01        	leaq	0x1bd3db1(%rip), %rdx   # 0x142bc33d4
140fef623: 48 89 51 08                 	movq	%rdx, 0x8(%rcx)
140fef627: 48 c7 41 10 17 00 00 00     	movq	$0x17, 0x10(%rcx)
140fef62f: 48 89 41 18                 	movq	%rax, 0x18(%rcx)
140fef633: 48 8d 15 af 30 f9 01        	leaq	0x1f930af(%rip), %rdx   # 0x142f826e9
140fef63a: 48 89 51 20                 	movq	%rdx, 0x20(%rcx)
140fef63e: 48 c7 41 28 42 00 00 00     	movq	$0x42, 0x28(%rcx)
140fef646: e9 7b fe ff ff              	jmp	0x140fef4c6 <.text+0xfee4c6>
140fef64b: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140fef655: 48 89 01                    	movq	%rax, (%rcx)
140fef658: 48 8d 15 75 3d bd 01        	leaq	0x1bd3d75(%rip), %rdx   # 0x142bc33d4
140fef65f: 48 89 51 08                 	movq	%rdx, 0x8(%rcx)
140fef663: 48 c7 41 10 17 00 00 00     	movq	$0x17, 0x10(%rcx)
140fef66b: 48 89 41 18                 	movq	%rax, 0x18(%rcx)
140fef66f: 48 8d 15 42 21 f9 01        	leaq	0x1f92142(%rip), %rdx   # 0x142f817b8
140fef676: 48 89 51 20                 	movq	%rdx, 0x20(%rcx)
140fef67a: 48 c7 41 28 98 00 00 00     	movq	$0x98, 0x28(%rcx)
140fef682: e9 3f fe ff ff              	jmp	0x140fef4c6 <.text+0xfee4c6>
140fef687: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140fef691: 48 89 01                    	movq	%rax, (%rcx)
140fef694: 48 8d 15 39 3d bd 01        	leaq	0x1bd3d39(%rip), %rdx   # 0x142bc33d4
140fef69b: 48 89 51 08                 	movq	%rdx, 0x8(%rcx)
140fef69f: 48 c7 41 10 17 00 00 00     	movq	$0x17, 0x10(%rcx)
140fef6a7: 48 89 41 18                 	movq	%rax, 0x18(%rcx)
140fef6ab: 48 8d 15 d2 1f f9 01        	leaq	0x1f91fd2(%rip), %rdx   # 0x142f81684
140fef6b2: 48 89 51 20                 	movq	%rdx, 0x20(%rcx)
140fef6b6: 48 c7 41 28 4b 00 00 00     	movq	$0x4b, 0x28(%rcx)
140fef6be: e9 03 fe ff ff              	jmp	0x140fef4c6 <.text+0xfee4c6>
140fef6c3: 48 8d 0d ce 40 bd 01        	leaq	0x1bd40ce(%rip), %rcx   # 0x142bc3798
140fef6ca: 4c 8d 05 6f 41 bd 01        	leaq	0x1bd416f(%rip), %r8    # 0x142bc3840
140fef6d1: ba 59 00 00 00              	movl	$0x59, %edx
140fef6d6: e8 55 9c 8e 01              	callq	0x1428d9330 <.text+0x28d8330>
140fef6db: cc                          	int3
140fef6dc: cc                          	int3
140fef6dd: cc                          	int3
140fef6de: cc                          	int3
140fef6df: cc                          	int3
140fef6e0: 48 83 ec 68                 	subq	$0x68, %rsp
140fef6e4: 83 b9 38 01 00 00 02        	cmpl	$0x2, 0x138(%rcx)
140fef6eb: 0f 84 8e 00 00 00           	je	0x140fef77f <.text+0xfee77f>
140fef6f1: 48 8b 82 a0 05 00 00        	movq	0x5a0(%rdx), %rax
140fef6f8: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
140fef6fd: 48 39 81 90 01 00 00        	cmpq	%rax, 0x190(%rcx)
140fef704: 75 1d                       	jne	0x140fef723 <.text+0xfee723>
140fef706: 48 8b 92 08 01 00 00        	movq	0x108(%rdx), %rdx
140fef70d: 8b 81 98 01 00 00           	movl	0x198(%rcx), %eax
140fef713: 89 91 98 01 00 00           	movl	%edx, 0x198(%rcx)
140fef719: 48 39 c2                    	cmpq	%rax, %rdx
140fef71c: 72 52                       	jb	0x140fef770 <.text+0xfee770>
140fef71e: 48 83 c4 68                 	addq	$0x68, %rsp
140fef722: c3                          	retq
140fef723: 48 8d 05 26 59 f9 01        	leaq	0x1f95926(%rip), %rax   # 0x142f85050
140fef72a: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
140fef72f: 48 c7 44 24 40 01 00 00 00  	movq	$0x1, 0x40(%rsp)
140fef738: 48 c7 44 24 48 08 00 00 00  	movq	$0x8, 0x48(%rsp)
140fef741: 0f 57 c0                    	xorps	%xmm0, %xmm0
140fef744: 0f 11 44 24 50              	movups	%xmm0, 0x50(%rsp)
140fef749: 48 81 c1 90 01 00 00        	addq	$0x190, %rcx            # imm = 0x190
140fef750: 48 8d 05 09 59 f9 01        	leaq	0x1f95909(%rip), %rax   # 0x142f85060
140fef757: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140fef75c: 4c 8d 44 24 30              	leaq	0x30(%rsp), %r8
140fef761: 4c 8d 4c 24 38              	leaq	0x38(%rsp), %r9
140fef766: 48 89 ca                    	movq	%rcx, %rdx
140fef769: 31 c9                       	xorl	%ecx, %ecx
140fef76b: e8 71 e9 8d 01              	callq	0x1428ce0e1 <.text+0x28cd0e1>
140fef770: 4c 8d 05 01 59 f9 01        	leaq	0x1f95901(%rip), %r8    # 0x142f85078
140fef777: 48 89 c1                    	movq	%rax, %rcx
140fef77a: e8 b1 a0 8e 01              	callq	0x1428d9830 <.text+0x28d8830>
140fef77f: 48 8d 0d 12 47 f9 01        	leaq	0x1f94712(%rip), %rcx   # 0x142f83e98
140fef786: 4c 8d 05 3b 58 f9 01        	leaq	0x1f9583b(%rip), %r8    # 0x142f84fc8
140fef78d: ba 59 00 00 00              	movl	$0x59, %edx
140fef792: e8 99 9b 8e 01              	callq	0x1428d9330 <.text+0x28d8330>
140fef797: cc                          	int3
140fef798: cc                          	int3
140fef799: cc                          	int3
140fef79a: cc                          	int3
140fef79b: cc                          	int3
140fef79c: cc                          	int3
140fef79d: cc                          	int3
140fef79e: cc                          	int3
140fef79f: cc                          	int3
140fef7a0: 48 b8 23 dc 61 8a c6 a5 5a 8a       	movabsq	$-0x75a55a39759e23dd, %rax # imm = 0x8A5AA5C68A61DC23
140fef7aa: 48 ba 34 cc 82 91 b0 8f cb 6d       	movabsq	$0x6dcb8fb09182cc34, %rdx # imm = 0x6DCB8FB09182CC34
140fef7b4: c3                          	retq
140fef7b5: cc                          	int3
140fef7b6: cc                          	int3
140fef7b7: cc                          	int3
140fef7b8: cc                          	int3
140fef7b9: cc                          	int3
140fef7ba: cc                          	int3
140fef7bb: cc                          	int3
140fef7bc: cc                          	int3
140fef7bd: cc                          	int3
140fef7be: cc                          	int3
140fef7bf: cc                          	int3
