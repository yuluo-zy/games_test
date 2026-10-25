
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14033def0: 55                          	pushq	%rbp
14033def1: 56                          	pushq	%rsi
14033def2: 57                          	pushq	%rdi
14033def3: 53                          	pushq	%rbx
14033def4: 48 81 ec 88 05 00 00        	subq	$0x588, %rsp            # imm = 0x588
14033defb: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14033df03: 48 c7 85 00 05 00 00 fe ff ff ff    	movq	$-0x2, 0x500(%rbp)
14033df0e: 48 89 ce                    	movq	%rcx, %rsi
14033df11: 0f b6 05 f1 6c 2f 03        	movzbl	0x32f6cf1(%rip), %eax   # 0x143634c09
14033df18: b9 f0 03 00 00              	movl	$0x3f0, %ecx            # imm = 0x3F0
14033df1d: ba 08 00 00 00              	movl	$0x8, %edx
14033df22: e8 e9 5c 2d 00              	callq	0x140613c10 <.text+0x612c10>
14033df27: 48 85 c0                    	testq	%rax, %rax
14033df2a: 0f 84 56 07 00 00           	je	0x14033e686 <.text+0x33d686>
14033df30: 48 89 85 f8 04 00 00        	movq	%rax, 0x4f8(%rbp)
14033df37: 0f 57 c0                    	xorps	%xmm0, %xmm0
14033df3a: 0f 11 85 40 03 00 00        	movups	%xmm0, 0x340(%rbp)
14033df41: 0f 11 85 58 03 00 00        	movups	%xmm0, 0x358(%rbp)
14033df48: 0f 11 85 70 03 00 00        	movups	%xmm0, 0x370(%rbp)
14033df4f: 0f 11 85 88 03 00 00        	movups	%xmm0, 0x388(%rbp)
14033df56: 0f 11 85 a0 03 00 00        	movups	%xmm0, 0x3a0(%rbp)
14033df5d: c7 85 b0 03 00 00 00 00 00 00       	movl	$0x0, 0x3b0(%rbp)
14033df67: 0f 11 85 d8 03 00 00        	movups	%xmm0, 0x3d8(%rbp)
14033df6e: 0f 11 85 f0 03 00 00        	movups	%xmm0, 0x3f0(%rbp)
14033df75: 0f 11 85 08 04 00 00        	movups	%xmm0, 0x408(%rbp)
14033df7c: 0f 11 85 20 04 00 00        	movups	%xmm0, 0x420(%rbp)
14033df83: 0f 11 85 38 04 00 00        	movups	%xmm0, 0x438(%rbp)
14033df8a: c7 85 48 04 00 00 00 00 00 00       	movl	$0x0, 0x448(%rbp)
14033df94: 48 c7 85 20 03 00 00 00 00 00 00    	movq	$0x0, 0x320(%rbp)
14033df9f: 48 c7 85 28 03 00 00 08 00 00 00    	movq	$0x8, 0x328(%rbp)
14033dfaa: 48 c7 85 30 03 00 00 00 00 00 00    	movq	$0x0, 0x330(%rbp)
14033dfb5: 48 c7 85 38 03 00 00 10 00 00 00    	movq	$0x10, 0x338(%rbp)
14033dfc0: 48 c7 85 50 03 00 00 10 00 00 00    	movq	$0x10, 0x350(%rbp)
14033dfcb: 48 c7 85 68 03 00 00 10 00 00 00    	movq	$0x10, 0x368(%rbp)
14033dfd6: 48 c7 85 80 03 00 00 10 00 00 00    	movq	$0x10, 0x380(%rbp)
14033dfe1: 48 c7 85 98 03 00 00 10 00 00 00    	movq	$0x10, 0x398(%rbp)
14033dfec: 48 bb 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rbx # imm = 0x8000000000000000
14033dff6: 48 89 9d b8 03 00 00        	movq	%rbx, 0x3b8(%rbp)
14033dffd: 48 8d 05 b3 3c 68 02        	leaq	0x2683cb3(%rip), %rax   # 0x1429c1cb7
14033e004: 48 89 85 c0 03 00 00        	movq	%rax, 0x3c0(%rbp)
14033e00b: 48 c7 85 c8 03 00 00 47 00 00 00    	movq	$0x47, 0x3c8(%rbp)
14033e016: 48 c7 85 d0 03 00 00 10 00 00 00    	movq	$0x10, 0x3d0(%rbp)
14033e021: 48 c7 85 e8 03 00 00 10 00 00 00    	movq	$0x10, 0x3e8(%rbp)
14033e02c: 48 c7 85 00 04 00 00 10 00 00 00    	movq	$0x10, 0x400(%rbp)
14033e037: 48 c7 85 18 04 00 00 10 00 00 00    	movq	$0x10, 0x418(%rbp)
14033e042: 48 c7 85 30 04 00 00 10 00 00 00    	movq	$0x10, 0x430(%rbp)
14033e04d: c7 85 50 04 00 00 00 00 00 00       	movl	$0x0, 0x450(%rbp)
14033e057: 66 c7 85 54 04 00 00 01 00  	movw	$0x1, 0x454(%rbp)
14033e060: 48 89 9d b8 04 00 00        	movq	%rbx, 0x4b8(%rbp)
14033e067: c7 85 f0 04 00 00 00 00 00 00       	movl	$0x0, 0x4f0(%rbp)
14033e071: 0f b6 05 91 6b 2f 03        	movzbl	0x32f6b91(%rip), %eax   # 0x143634c09
14033e078: b9 d8 01 00 00              	movl	$0x1d8, %ecx            # imm = 0x1D8
14033e07d: ba 08 00 00 00              	movl	$0x8, %edx
14033e082: e8 89 5b 2d 00              	callq	0x140613c10 <.text+0x612c10>
14033e087: 48 85 c0                    	testq	%rax, %rax
14033e08a: 0f 84 05 06 00 00           	je	0x14033e695 <.text+0x33d695>
14033e090: 48 89 c7                    	movq	%rax, %rdi
14033e093: 48 8d 95 20 03 00 00        	leaq	0x320(%rbp), %rdx
14033e09a: 41 b8 d8 01 00 00           	movl	$0x1d8, %r8d            # imm = 0x1D8
14033e0a0: 48 89 c1                    	movq	%rax, %rcx
14033e0a3: e8 58 5b 5e 02              	callq	0x142923c00 <.text+0x2922c00>
14033e0a8: 4c 8d 05 f1 3a 68 02        	leaq	0x2683af1(%rip), %r8    # 0x1429c1ba0
14033e0af: 48 8d 8d 40 02 00 00        	leaq	0x240(%rbp), %rcx
14033e0b6: 48 89 fa                    	movq	%rdi, %rdx
14033e0b9: e8 52 6a 9a 00              	callq	0x140ce4b10 <.text+0xce3b10>
14033e0be: 0f 57 c0                    	xorps	%xmm0, %xmm0
14033e0c1: 0f 11 85 40 03 00 00        	movups	%xmm0, 0x340(%rbp)
14033e0c8: 0f 11 85 58 03 00 00        	movups	%xmm0, 0x358(%rbp)
14033e0cf: 0f 11 85 70 03 00 00        	movups	%xmm0, 0x370(%rbp)
14033e0d6: 0f 11 85 88 03 00 00        	movups	%xmm0, 0x388(%rbp)
14033e0dd: 0f 11 85 a0 03 00 00        	movups	%xmm0, 0x3a0(%rbp)
14033e0e4: c7 85 b0 03 00 00 00 00 00 00       	movl	$0x0, 0x3b0(%rbp)
14033e0ee: 0f 11 85 d8 03 00 00        	movups	%xmm0, 0x3d8(%rbp)
14033e0f5: 0f 11 85 f0 03 00 00        	movups	%xmm0, 0x3f0(%rbp)
14033e0fc: 0f 11 85 08 04 00 00        	movups	%xmm0, 0x408(%rbp)
14033e103: 0f 11 85 20 04 00 00        	movups	%xmm0, 0x420(%rbp)
14033e10a: 0f 11 85 38 04 00 00        	movups	%xmm0, 0x438(%rbp)
14033e111: c7 85 48 04 00 00 00 00 00 00       	movl	$0x0, 0x448(%rbp)
14033e11b: 48 c7 85 20 03 00 00 00 00 00 00    	movq	$0x0, 0x320(%rbp)
14033e126: 48 c7 85 28 03 00 00 08 00 00 00    	movq	$0x8, 0x328(%rbp)
14033e131: 48 c7 85 30 03 00 00 00 00 00 00    	movq	$0x0, 0x330(%rbp)
14033e13c: 48 c7 85 38 03 00 00 10 00 00 00    	movq	$0x10, 0x338(%rbp)
14033e147: 48 c7 85 50 03 00 00 10 00 00 00    	movq	$0x10, 0x350(%rbp)
14033e152: 48 c7 85 68 03 00 00 10 00 00 00    	movq	$0x10, 0x368(%rbp)
14033e15d: 48 c7 85 80 03 00 00 10 00 00 00    	movq	$0x10, 0x380(%rbp)
14033e168: 48 c7 85 98 03 00 00 10 00 00 00    	movq	$0x10, 0x398(%rbp)
14033e173: 48 89 9d b8 03 00 00        	movq	%rbx, 0x3b8(%rbp)
14033e17a: 48 8d 05 f4 3a 68 02        	leaq	0x2683af4(%rip), %rax   # 0x1429c1c75
14033e181: 48 89 85 c0 03 00 00        	movq	%rax, 0x3c0(%rbp)
14033e188: 48 c7 85 c8 03 00 00 42 00 00 00    	movq	$0x42, 0x3c8(%rbp)
14033e193: 48 c7 85 d0 03 00 00 10 00 00 00    	movq	$0x10, 0x3d0(%rbp)
14033e19e: 48 c7 85 e8 03 00 00 10 00 00 00    	movq	$0x10, 0x3e8(%rbp)
14033e1a9: 48 c7 85 00 04 00 00 10 00 00 00    	movq	$0x10, 0x400(%rbp)
14033e1b4: 48 c7 85 18 04 00 00 10 00 00 00    	movq	$0x10, 0x418(%rbp)
14033e1bf: 48 c7 85 30 04 00 00 10 00 00 00    	movq	$0x10, 0x430(%rbp)
14033e1ca: c7 85 50 04 00 00 00 00 00 00       	movl	$0x0, 0x450(%rbp)
14033e1d4: 66 c7 85 54 04 00 00 01 00  	movw	$0x1, 0x454(%rbp)
14033e1dd: 48 89 9d 58 04 00 00        	movq	%rbx, 0x458(%rbp)
14033e1e4: c7 85 e0 04 00 00 00 00 00 00       	movl	$0x0, 0x4e0(%rbp)
14033e1ee: 0f b6 05 14 6a 2f 03        	movzbl	0x32f6a14(%rip), %eax   # 0x143634c09
14033e1f5: b9 c8 01 00 00              	movl	$0x1c8, %ecx            # imm = 0x1C8
14033e1fa: ba 08 00 00 00              	movl	$0x8, %edx
14033e1ff: e8 0c 5a 2d 00              	callq	0x140613c10 <.text+0x612c10>
14033e204: 48 85 c0                    	testq	%rax, %rax
14033e207: 0f 84 99 04 00 00           	je	0x14033e6a6 <.text+0x33d6a6>
14033e20d: 48 89 c7                    	movq	%rax, %rdi
14033e210: 48 8d 95 20 03 00 00        	leaq	0x320(%rbp), %rdx
14033e217: 41 b8 c8 01 00 00           	movl	$0x1c8, %r8d            # imm = 0x1C8
14033e21d: 48 89 c1                    	movq	%rax, %rcx
14033e220: e8 db 59 5e 02              	callq	0x142923c00 <.text+0x2922c00>
14033e225: 4c 8d 05 bc 38 68 02        	leaq	0x26838bc(%rip), %r8    # 0x1429c1ae8
14033e22c: 48 8d 8d d0 01 00 00        	leaq	0x1d0(%rbp), %rcx
14033e233: 48 89 fa                    	movq	%rdi, %rdx
14033e236: e8 d5 68 9a 00              	callq	0x140ce4b10 <.text+0xce3b10>
14033e23b: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
14033e242: e8 e9 c1 ff ff              	callq	0x14033a430 <.text+0x339430>
14033e247: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
14033e24e: e8 3d 45 00 00              	callq	0x140342790 <.text+0x341790>
14033e253: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
14033e25a: e8 01 c0 ff ff              	callq	0x14033a260 <.text+0x339260>
14033e25f: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
14033e263: e8 28 4a cc ff              	callq	0x140002c90 <.text+0x1c90>
14033e268: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
14033e26c: e8 2f c6 fd ff              	callq	0x14031a8a0 <.text+0x3198a0>
14033e271: 48 8d 8d b0 02 00 00        	leaq	0x2b0(%rbp), %rcx
14033e278: e8 83 f2 f6 ff              	callq	0x1402ad500 <.text+0x2ac500>
14033e27d: 48 8d 8d 20 03 00 00        	leaq	0x320(%rbp), %rcx
14033e284: e8 f7 11 1d 00              	callq	0x14050f480 <.text+0x50e480>
14033e289: 0f 10 85 a0 02 00 00        	movups	0x2a0(%rbp), %xmm0
14033e290: 48 8b 85 f8 04 00 00        	movq	0x4f8(%rbp), %rax
14033e297: 0f 11 40 60                 	movups	%xmm0, 0x60(%rax)
14033e29b: 0f 10 85 90 02 00 00        	movups	0x290(%rbp), %xmm0
14033e2a2: 0f 11 40 50                 	movups	%xmm0, 0x50(%rax)
14033e2a6: 0f 10 85 80 02 00 00        	movups	0x280(%rbp), %xmm0
14033e2ad: 0f 11 40 40                 	movups	%xmm0, 0x40(%rax)
14033e2b1: 0f 10 85 40 02 00 00        	movups	0x240(%rbp), %xmm0
14033e2b8: 0f 10 8d 50 02 00 00        	movups	0x250(%rbp), %xmm1
14033e2bf: 0f 10 95 60 02 00 00        	movups	0x260(%rbp), %xmm2
14033e2c6: 0f 10 9d 70 02 00 00        	movups	0x270(%rbp), %xmm3
14033e2cd: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
14033e2d1: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
14033e2d5: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
14033e2d9: 0f 11 00                    	movups	%xmm0, (%rax)
14033e2dc: 0f 10 85 d0 01 00 00        	movups	0x1d0(%rbp), %xmm0
14033e2e3: 0f 10 8d e0 01 00 00        	movups	0x1e0(%rbp), %xmm1
14033e2ea: 0f 10 95 f0 01 00 00        	movups	0x1f0(%rbp), %xmm2
14033e2f1: 0f 10 9d 00 02 00 00        	movups	0x200(%rbp), %xmm3
14033e2f8: 0f 11 40 70                 	movups	%xmm0, 0x70(%rax)
14033e2fc: 0f 11 88 80 00 00 00        	movups	%xmm1, 0x80(%rax)
14033e303: 0f 11 90 90 00 00 00        	movups	%xmm2, 0x90(%rax)
14033e30a: 0f 11 98 a0 00 00 00        	movups	%xmm3, 0xa0(%rax)
14033e311: 0f 10 85 10 02 00 00        	movups	0x210(%rbp), %xmm0
14033e318: 0f 11 80 b0 00 00 00        	movups	%xmm0, 0xb0(%rax)
14033e31f: 0f 10 85 20 02 00 00        	movups	0x220(%rbp), %xmm0
14033e326: 0f 11 80 c0 00 00 00        	movups	%xmm0, 0xc0(%rax)
14033e32d: 0f 10 85 30 02 00 00        	movups	0x230(%rbp), %xmm0
14033e334: 0f 11 80 d0 00 00 00        	movups	%xmm0, 0xd0(%rax)
14033e33b: 0f 10 85 c0 01 00 00        	movups	0x1c0(%rbp), %xmm0
14033e342: 0f 11 80 40 01 00 00        	movups	%xmm0, 0x140(%rax)
14033e349: 0f 10 85 b0 01 00 00        	movups	0x1b0(%rbp), %xmm0
14033e350: 0f 11 80 30 01 00 00        	movups	%xmm0, 0x130(%rax)
14033e357: 0f 10 85 a0 01 00 00        	movups	0x1a0(%rbp), %xmm0
14033e35e: 0f 11 80 20 01 00 00        	movups	%xmm0, 0x120(%rax)
14033e365: 0f 10 85 60 01 00 00        	movups	0x160(%rbp), %xmm0
14033e36c: 0f 10 8d 70 01 00 00        	movups	0x170(%rbp), %xmm1
14033e373: 0f 10 95 80 01 00 00        	movups	0x180(%rbp), %xmm2
14033e37a: 0f 10 9d 90 01 00 00        	movups	0x190(%rbp), %xmm3
14033e381: 0f 11 98 10 01 00 00        	movups	%xmm3, 0x110(%rax)
14033e388: 0f 11 90 00 01 00 00        	movups	%xmm2, 0x100(%rax)
14033e38f: 0f 11 88 f0 00 00 00        	movups	%xmm1, 0xf0(%rax)
14033e396: 0f 11 80 e0 00 00 00        	movups	%xmm0, 0xe0(%rax)
14033e39d: 0f 10 85 f0 00 00 00        	movups	0xf0(%rbp), %xmm0
14033e3a4: 0f 10 8d 00 01 00 00        	movups	0x100(%rbp), %xmm1
14033e3ab: 0f 10 95 10 01 00 00        	movups	0x110(%rbp), %xmm2
14033e3b2: 0f 10 9d 20 01 00 00        	movups	0x120(%rbp), %xmm3
14033e3b9: 0f 11 80 50 01 00 00        	movups	%xmm0, 0x150(%rax)
14033e3c0: 0f 11 88 60 01 00 00        	movups	%xmm1, 0x160(%rax)
14033e3c7: 0f 11 90 70 01 00 00        	movups	%xmm2, 0x170(%rax)
14033e3ce: 0f 11 98 80 01 00 00        	movups	%xmm3, 0x180(%rax)
14033e3d5: 0f 10 85 30 01 00 00        	movups	0x130(%rbp), %xmm0
14033e3dc: 0f 11 80 90 01 00 00        	movups	%xmm0, 0x190(%rax)
14033e3e3: 0f 10 85 40 01 00 00        	movups	0x140(%rbp), %xmm0
14033e3ea: 0f 11 80 a0 01 00 00        	movups	%xmm0, 0x1a0(%rax)
14033e3f1: 0f 10 85 50 01 00 00        	movups	0x150(%rbp), %xmm0
14033e3f8: 0f 11 80 b0 01 00 00        	movups	%xmm0, 0x1b0(%rax)
14033e3ff: 0f 10 85 80 00 00 00        	movups	0x80(%rbp), %xmm0
14033e406: 0f 10 8d 90 00 00 00        	movups	0x90(%rbp), %xmm1
14033e40d: 0f 10 95 a0 00 00 00        	movups	0xa0(%rbp), %xmm2
14033e414: 0f 10 9d b0 00 00 00        	movups	0xb0(%rbp), %xmm3
14033e41b: 0f 11 80 c0 01 00 00        	movups	%xmm0, 0x1c0(%rax)
14033e422: 0f 11 88 d0 01 00 00        	movups	%xmm1, 0x1d0(%rax)
14033e429: 0f 11 90 e0 01 00 00        	movups	%xmm2, 0x1e0(%rax)
14033e430: 0f 11 98 f0 01 00 00        	movups	%xmm3, 0x1f0(%rax)
14033e437: 0f 10 85 c0 00 00 00        	movups	0xc0(%rbp), %xmm0
14033e43e: 0f 11 80 00 02 00 00        	movups	%xmm0, 0x200(%rax)
14033e445: 0f 10 85 e0 00 00 00        	movups	0xe0(%rbp), %xmm0
14033e44c: 0f 11 80 20 02 00 00        	movups	%xmm0, 0x220(%rax)
14033e453: 0f 10 85 d0 00 00 00        	movups	0xd0(%rbp), %xmm0
14033e45a: 0f 11 80 10 02 00 00        	movups	%xmm0, 0x210(%rax)
14033e461: 0f 10 45 70                 	movups	0x70(%rbp), %xmm0
14033e465: 0f 11 80 90 02 00 00        	movups	%xmm0, 0x290(%rax)
14033e46c: 0f 10 45 60                 	movups	0x60(%rbp), %xmm0
14033e470: 0f 11 80 80 02 00 00        	movups	%xmm0, 0x280(%rax)
14033e477: 0f 10 45 50                 	movups	0x50(%rbp), %xmm0
14033e47b: 0f 11 80 70 02 00 00        	movups	%xmm0, 0x270(%rax)
14033e482: 0f 10 45 10                 	movups	0x10(%rbp), %xmm0
14033e486: 0f 10 4d 20                 	movups	0x20(%rbp), %xmm1
14033e48a: 0f 10 55 30                 	movups	0x30(%rbp), %xmm2
14033e48e: 0f 10 5d 40                 	movups	0x40(%rbp), %xmm3
14033e492: 0f 11 98 60 02 00 00        	movups	%xmm3, 0x260(%rax)
14033e499: 0f 11 90 50 02 00 00        	movups	%xmm2, 0x250(%rax)
14033e4a0: 0f 11 88 40 02 00 00        	movups	%xmm1, 0x240(%rax)
14033e4a7: 0f 11 80 30 02 00 00        	movups	%xmm0, 0x230(%rax)
14033e4ae: 0f 10 45 a0                 	movups	-0x60(%rbp), %xmm0
14033e4b2: 0f 10 4d b0                 	movups	-0x50(%rbp), %xmm1
14033e4b6: 0f 10 55 c0                 	movups	-0x40(%rbp), %xmm2
14033e4ba: 0f 10 5d d0                 	movups	-0x30(%rbp), %xmm3
14033e4be: 0f 11 80 a0 02 00 00        	movups	%xmm0, 0x2a0(%rax)
14033e4c5: 0f 11 88 b0 02 00 00        	movups	%xmm1, 0x2b0(%rax)
14033e4cc: 0f 11 90 c0 02 00 00        	movups	%xmm2, 0x2c0(%rax)
14033e4d3: 0f 11 98 d0 02 00 00        	movups	%xmm3, 0x2d0(%rax)
14033e4da: 0f 10 45 e0                 	movups	-0x20(%rbp), %xmm0
14033e4de: 0f 11 80 e0 02 00 00        	movups	%xmm0, 0x2e0(%rax)
14033e4e5: 0f 10 45 f0                 	movups	-0x10(%rbp), %xmm0
14033e4e9: 0f 11 80 f0 02 00 00        	movups	%xmm0, 0x2f0(%rax)
14033e4f0: 0f 10 45 00                 	movups	(%rbp), %xmm0
14033e4f4: 0f 11 80 00 03 00 00        	movups	%xmm0, 0x300(%rax)
14033e4fb: 0f 10 85 b0 02 00 00        	movups	0x2b0(%rbp), %xmm0
14033e502: 0f 10 8d c0 02 00 00        	movups	0x2c0(%rbp), %xmm1
14033e509: 0f 10 95 d0 02 00 00        	movups	0x2d0(%rbp), %xmm2
14033e510: 0f 10 9d e0 02 00 00        	movups	0x2e0(%rbp), %xmm3
14033e517: 0f 11 80 10 03 00 00        	movups	%xmm0, 0x310(%rax)
14033e51e: 0f 11 88 20 03 00 00        	movups	%xmm1, 0x320(%rax)
14033e525: 0f 11 90 30 03 00 00        	movups	%xmm2, 0x330(%rax)
14033e52c: 0f 11 98 40 03 00 00        	movups	%xmm3, 0x340(%rax)
14033e533: 0f 10 85 f0 02 00 00        	movups	0x2f0(%rbp), %xmm0
14033e53a: 0f 11 80 50 03 00 00        	movups	%xmm0, 0x350(%rax)
14033e541: 0f 10 85 00 03 00 00        	movups	0x300(%rbp), %xmm0
14033e548: 0f 11 80 60 03 00 00        	movups	%xmm0, 0x360(%rax)
14033e54f: 0f 10 85 10 03 00 00        	movups	0x310(%rbp), %xmm0
14033e556: 0f 11 80 70 03 00 00        	movups	%xmm0, 0x370(%rax)
14033e55d: 0f 10 85 20 03 00 00        	movups	0x320(%rbp), %xmm0
14033e564: 0f 10 8d 30 03 00 00        	movups	0x330(%rbp), %xmm1
14033e56b: 0f 10 95 40 03 00 00        	movups	0x340(%rbp), %xmm2
14033e572: 0f 10 9d 50 03 00 00        	movups	0x350(%rbp), %xmm3
14033e579: 0f 11 80 80 03 00 00        	movups	%xmm0, 0x380(%rax)
14033e580: 0f 11 88 90 03 00 00        	movups	%xmm1, 0x390(%rax)
14033e587: 0f 11 90 a0 03 00 00        	movups	%xmm2, 0x3a0(%rax)
14033e58e: 0f 11 98 b0 03 00 00        	movups	%xmm3, 0x3b0(%rax)
14033e595: 0f 10 85 60 03 00 00        	movups	0x360(%rbp), %xmm0
14033e59c: 0f 11 80 c0 03 00 00        	movups	%xmm0, 0x3c0(%rax)
14033e5a3: 0f 10 85 70 03 00 00        	movups	0x370(%rbp), %xmm0
14033e5aa: 0f 11 80 d0 03 00 00        	movups	%xmm0, 0x3d0(%rax)
14033e5b1: 0f 10 85 80 03 00 00        	movups	0x380(%rbp), %xmm0
14033e5b8: 0f 11 80 e0 03 00 00        	movups	%xmm0, 0x3e0(%rax)
14033e5bf: 48 c7 85 28 03 00 00 09 00 00 00    	movq	$0x9, 0x328(%rbp)
14033e5ca: 48 89 85 30 03 00 00        	movq	%rax, 0x330(%rbp)
14033e5d1: 48 c7 85 38 03 00 00 09 00 00 00    	movq	$0x9, 0x338(%rbp)
14033e5dc: 48 c7 85 40 03 00 00 00 00 00 00    	movq	$0x0, 0x340(%rbp)
14033e5e7: 48 c7 85 48 03 00 00 08 00 00 00    	movq	$0x8, 0x348(%rbp)
14033e5f2: 0f 57 c0                    	xorps	%xmm0, %xmm0
14033e5f5: 0f 11 85 50 03 00 00        	movups	%xmm0, 0x350(%rbp)
14033e5fc: 48 8b 85 c0 02 00 00        	movq	0x2c0(%rbp), %rax
14033e603: 48 89 85 70 03 00 00        	movq	%rax, 0x370(%rbp)
14033e60a: 0f 10 85 b0 02 00 00        	movups	0x2b0(%rbp), %xmm0
14033e611: 0f 11 85 60 03 00 00        	movups	%xmm0, 0x360(%rbp)
14033e618: 48 89 9d 20 03 00 00        	movq	%rbx, 0x320(%rbp)
14033e61f: 48 8d 8d 58 03 00 00        	leaq	0x358(%rbp), %rcx
14033e626: e8 a5 1f e3 ff              	callq	0x1401705d0 <.text+0x16f5d0>
14033e62b: 0f 10 85 80 03 00 00        	movups	0x380(%rbp), %xmm0
14033e632: 0f 11 46 60                 	movups	%xmm0, 0x60(%rsi)
14033e636: 0f 10 85 70 03 00 00        	movups	0x370(%rbp), %xmm0
14033e63d: 0f 11 46 50                 	movups	%xmm0, 0x50(%rsi)
14033e641: 0f 10 85 60 03 00 00        	movups	0x360(%rbp), %xmm0
14033e648: 0f 11 46 40                 	movups	%xmm0, 0x40(%rsi)
14033e64c: 0f 10 85 20 03 00 00        	movups	0x320(%rbp), %xmm0
14033e653: 0f 10 8d 30 03 00 00        	movups	0x330(%rbp), %xmm1
14033e65a: 0f 10 95 40 03 00 00        	movups	0x340(%rbp), %xmm2
14033e661: 0f 10 9d 50 03 00 00        	movups	0x350(%rbp), %xmm3
14033e668: 0f 11 5e 30                 	movups	%xmm3, 0x30(%rsi)
14033e66c: 0f 11 56 20                 	movups	%xmm2, 0x20(%rsi)
14033e670: 0f 11 4e 10                 	movups	%xmm1, 0x10(%rsi)
14033e674: 0f 11 06                    	movups	%xmm0, (%rsi)
14033e677: 48 89 f0                    	movq	%rsi, %rax
14033e67a: 48 81 c4 88 05 00 00        	addq	$0x588, %rsp            # imm = 0x588
14033e681: 5b                          	popq	%rbx
14033e682: 5f                          	popq	%rdi
14033e683: 5e                          	popq	%rsi
14033e684: 5d                          	popq	%rbp
14033e685: c3                          	retq
14033e686: b9 08 00 00 00              	movl	$0x8, %ecx
14033e68b: ba f0 03 00 00              	movl	$0x3f0, %edx            # imm = 0x3F0
14033e690: e8 6b a9 59 02              	callq	0x1428d9000 <.text+0x28d8000>
14033e695: b9 08 00 00 00              	movl	$0x8, %ecx
14033e69a: ba d8 01 00 00              	movl	$0x1d8, %edx            # imm = 0x1D8
14033e69f: e8 5c a9 59 02              	callq	0x1428d9000 <.text+0x28d8000>
14033e6a4: eb 0f                       	jmp	0x14033e6b5 <.text+0x33d6b5>
14033e6a6: b9 08 00 00 00              	movl	$0x8, %ecx
14033e6ab: ba c8 01 00 00              	movl	$0x1c8, %edx            # imm = 0x1C8
14033e6b0: e8 4b a9 59 02              	callq	0x1428d9000 <.text+0x28d8000>
14033e6b5: 0f 0b                       	ud2
14033e6b7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
14033e6c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14033e6c5: 55                          	pushq	%rbp
14033e6c6: 56                          	pushq	%rsi
14033e6c7: 57                          	pushq	%rdi
14033e6c8: 53                          	pushq	%rbx
14033e6c9: 48 83 ec 28                 	subq	$0x28, %rsp
14033e6cd: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14033e6d4: 48 8d 8d 20 03 00 00        	leaq	0x320(%rbp), %rcx
14033e6db: e8 60 bf 2c 00              	callq	0x14060a640 <.text+0x609640>
14033e6e0: 90                          	nop
14033e6e1: 48 83 c4 28                 	addq	$0x28, %rsp
14033e6e5: 5b                          	popq	%rbx
14033e6e6: 5f                          	popq	%rdi
14033e6e7: 5e                          	popq	%rsi
14033e6e8: 5d                          	popq	%rbp
14033e6e9: c3                          	retq
14033e6ea: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
14033e6f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14033e6f5: 55                          	pushq	%rbp
14033e6f6: 56                          	pushq	%rsi
14033e6f7: 57                          	pushq	%rdi
14033e6f8: 53                          	pushq	%rbx
14033e6f9: 48 83 ec 28                 	subq	$0x28, %rsp
14033e6fd: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14033e704: ba f0 03 00 00              	movl	$0x3f0, %edx            # imm = 0x3F0
14033e709: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14033e70f: 48 8b 8d f8 04 00 00        	movq	0x4f8(%rbp), %rcx
14033e716: e8 05 55 2d 00              	callq	0x140613c20 <.text+0x612c20>
14033e71b: 90                          	nop
14033e71c: 48 83 c4 28                 	addq	$0x28, %rsp
14033e720: 5b                          	popq	%rbx
14033e721: 5f                          	popq	%rdi
14033e722: 5e                          	popq	%rsi
14033e723: 5d                          	popq	%rbp
14033e724: c3                          	retq
14033e725: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
14033e730: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14033e735: 55                          	pushq	%rbp
14033e736: 56                          	pushq	%rsi
14033e737: 57                          	pushq	%rdi
14033e738: 53                          	pushq	%rbx
14033e739: 48 83 ec 28                 	subq	$0x28, %rsp
14033e73d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14033e744: 48 8d 8d 20 03 00 00        	leaq	0x320(%rbp), %rcx
14033e74b: e8 90 6c 29 00              	callq	0x1405d53e0 <.text+0x5d43e0>
14033e750: 90                          	nop
14033e751: 48 83 c4 28                 	addq	$0x28, %rsp
14033e755: 5b                          	popq	%rbx
14033e756: 5f                          	popq	%rdi
14033e757: 5e                          	popq	%rsi
14033e758: 5d                          	popq	%rbp
14033e759: c3                          	retq
14033e75a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
14033e760: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14033e765: 55                          	pushq	%rbp
14033e766: 56                          	pushq	%rsi
14033e767: 57                          	pushq	%rdi
14033e768: 53                          	pushq	%rbx
14033e769: 48 83 ec 28                 	subq	$0x28, %rsp
14033e76d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14033e774: 48 8d 8d 40 02 00 00        	leaq	0x240(%rbp), %rcx
14033e77b: e8 00 b1 ce ff              	callq	0x140029880 <.text+0x28880>
14033e780: 90                          	nop
14033e781: 48 83 c4 28                 	addq	$0x28, %rsp
14033e785: 5b                          	popq	%rbx
14033e786: 5f                          	popq	%rdi
14033e787: 5e                          	popq	%rsi
14033e788: 5d                          	popq	%rbp
14033e789: c3                          	retq
14033e78a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
14033e790: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14033e795: 55                          	pushq	%rbp
14033e796: 56                          	pushq	%rsi
14033e797: 57                          	pushq	%rdi
14033e798: 53                          	pushq	%rbx
14033e799: 48 83 ec 28                 	subq	$0x28, %rsp
14033e79d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14033e7a4: 48 8d 8d d0 01 00 00        	leaq	0x1d0(%rbp), %rcx
14033e7ab: e8 d0 b0 ce ff              	callq	0x140029880 <.text+0x28880>
14033e7b0: 90                          	nop
14033e7b1: 48 83 c4 28                 	addq	$0x28, %rsp
14033e7b5: 5b                          	popq	%rbx
14033e7b6: 5f                          	popq	%rdi
14033e7b7: 5e                          	popq	%rsi
14033e7b8: 5d                          	popq	%rbp
14033e7b9: c3                          	retq
14033e7ba: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
14033e7c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14033e7c5: 55                          	pushq	%rbp
14033e7c6: 56                          	pushq	%rsi
14033e7c7: 57                          	pushq	%rdi
14033e7c8: 53                          	pushq	%rbx
14033e7c9: 48 83 ec 28                 	subq	$0x28, %rsp
14033e7cd: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14033e7d4: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
14033e7db: e8 a0 b0 ce ff              	callq	0x140029880 <.text+0x28880>
14033e7e0: 90                          	nop
14033e7e1: 48 83 c4 28                 	addq	$0x28, %rsp
14033e7e5: 5b                          	popq	%rbx
14033e7e6: 5f                          	popq	%rdi
14033e7e7: 5e                          	popq	%rsi
14033e7e8: 5d                          	popq	%rbp
14033e7e9: c3                          	retq
14033e7ea: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
14033e7f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14033e7f5: 55                          	pushq	%rbp
14033e7f6: 56                          	pushq	%rsi
14033e7f7: 57                          	pushq	%rdi
14033e7f8: 53                          	pushq	%rbx
14033e7f9: 48 83 ec 28                 	subq	$0x28, %rsp
14033e7fd: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14033e804: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
14033e80b: e8 70 b0 ce ff              	callq	0x140029880 <.text+0x28880>
14033e810: 90                          	nop
14033e811: 48 83 c4 28                 	addq	$0x28, %rsp
14033e815: 5b                          	popq	%rbx
14033e816: 5f                          	popq	%rdi
14033e817: 5e                          	popq	%rsi
14033e818: 5d                          	popq	%rbp
14033e819: c3                          	retq
14033e81a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
14033e820: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14033e825: 55                          	pushq	%rbp
14033e826: 56                          	pushq	%rsi
14033e827: 57                          	pushq	%rdi
14033e828: 53                          	pushq	%rbx
14033e829: 48 83 ec 28                 	subq	$0x28, %rsp
14033e82d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14033e834: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
14033e83b: e8 40 b0 ce ff              	callq	0x140029880 <.text+0x28880>
14033e840: 90                          	nop
14033e841: 48 83 c4 28                 	addq	$0x28, %rsp
14033e845: 5b                          	popq	%rbx
14033e846: 5f                          	popq	%rdi
14033e847: 5e                          	popq	%rsi
14033e848: 5d                          	popq	%rbp
14033e849: c3                          	retq
14033e84a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
14033e850: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14033e855: 55                          	pushq	%rbp
14033e856: 56                          	pushq	%rsi
14033e857: 57                          	pushq	%rdi
14033e858: 53                          	pushq	%rbx
14033e859: 48 83 ec 28                 	subq	$0x28, %rsp
14033e85d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14033e864: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
14033e868: e8 13 b0 ce ff              	callq	0x140029880 <.text+0x28880>
14033e86d: 90                          	nop
14033e86e: 48 83 c4 28                 	addq	$0x28, %rsp
14033e872: 5b                          	popq	%rbx
14033e873: 5f                          	popq	%rdi
14033e874: 5e                          	popq	%rsi
14033e875: 5d                          	popq	%rbp
14033e876: c3                          	retq
14033e877: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
14033e880: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14033e885: 55                          	pushq	%rbp
14033e886: 56                          	pushq	%rsi
14033e887: 57                          	pushq	%rdi
14033e888: 53                          	pushq	%rbx
14033e889: 48 83 ec 28                 	subq	$0x28, %rsp
14033e88d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14033e894: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
14033e898: e8 e3 af ce ff              	callq	0x140029880 <.text+0x28880>
14033e89d: 90                          	nop
14033e89e: 48 83 c4 28                 	addq	$0x28, %rsp
14033e8a2: 5b                          	popq	%rbx
14033e8a3: 5f                          	popq	%rdi
14033e8a4: 5e                          	popq	%rsi
14033e8a5: 5d                          	popq	%rbp
14033e8a6: c3                          	retq
14033e8a7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
14033e8b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14033e8b5: 55                          	pushq	%rbp
14033e8b6: 56                          	pushq	%rsi
14033e8b7: 57                          	pushq	%rdi
14033e8b8: 53                          	pushq	%rbx
14033e8b9: 48 83 ec 28                 	subq	$0x28, %rsp
14033e8bd: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14033e8c4: 48 8d 8d b0 02 00 00        	leaq	0x2b0(%rbp), %rcx
14033e8cb: e8 b0 af ce ff              	callq	0x140029880 <.text+0x28880>
14033e8d0: 90                          	nop
14033e8d1: 48 83 c4 28                 	addq	$0x28, %rsp
14033e8d5: 5b                          	popq	%rbx
14033e8d6: 5f                          	popq	%rdi
14033e8d7: 5e                          	popq	%rsi
14033e8d8: 5d                          	popq	%rbp
14033e8d9: c3                          	retq
14033e8da: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
14033e8e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14033e8e5: 55                          	pushq	%rbp
14033e8e6: 56                          	pushq	%rsi
14033e8e7: 57                          	pushq	%rdi
14033e8e8: 53                          	pushq	%rbx
14033e8e9: 48 83 ec 28                 	subq	$0x28, %rsp
14033e8ed: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14033e8f4: 48 8d 8d 20 03 00 00        	leaq	0x320(%rbp), %rcx
14033e8fb: e8 80 af ce ff              	callq	0x140029880 <.text+0x28880>
14033e900: 90                          	nop
14033e901: 48 83 c4 28                 	addq	$0x28, %rsp
14033e905: 5b                          	popq	%rbx
14033e906: 5f                          	popq	%rdi
14033e907: 5e                          	popq	%rsi
14033e908: 5d                          	popq	%rbp
14033e909: c3                          	retq
