
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140ff0be0: 55                          	pushq	%rbp
140ff0be1: 56                          	pushq	%rsi
140ff0be2: 57                          	pushq	%rdi
140ff0be3: 53                          	pushq	%rbx
140ff0be4: 48 81 ec 48 06 00 00        	subq	$0x648, %rsp            # imm = 0x648
140ff0beb: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140ff0bf3: 48 c7 85 c0 05 00 00 fe ff ff ff    	movq	$-0x2, 0x5c0(%rbp)
140ff0bfe: 48 89 ce                    	movq	%rcx, %rsi
140ff0c01: 0f b6 05 01 40 64 02        	movzbl	0x2644001(%rip), %eax   # 0x143634c09
140ff0c08: b9 f0 03 00 00              	movl	$0x3f0, %ecx            # imm = 0x3F0
140ff0c0d: ba 08 00 00 00              	movl	$0x8, %edx
140ff0c12: e8 f9 2f 62 ff              	callq	0x140613c10 <.text+0x612c10>
140ff0c17: 48 85 c0                    	testq	%rax, %rax
140ff0c1a: 0f 84 5a 07 00 00           	je	0x140ff137a <.text+0xff037a>
140ff0c20: 48 89 85 b8 05 00 00        	movq	%rax, 0x5b8(%rbp)
140ff0c27: 0f 57 c0                    	xorps	%xmm0, %xmm0
140ff0c2a: 0f 11 85 68 03 00 00        	movups	%xmm0, 0x368(%rbp)
140ff0c31: 0f 11 85 80 03 00 00        	movups	%xmm0, 0x380(%rbp)
140ff0c38: 0f 11 85 98 03 00 00        	movups	%xmm0, 0x398(%rbp)
140ff0c3f: 0f 11 85 b0 03 00 00        	movups	%xmm0, 0x3b0(%rbp)
140ff0c46: 0f 11 85 c8 03 00 00        	movups	%xmm0, 0x3c8(%rbp)
140ff0c4d: c7 85 d8 03 00 00 00 00 00 00       	movl	$0x0, 0x3d8(%rbp)
140ff0c57: 0f 11 85 00 04 00 00        	movups	%xmm0, 0x400(%rbp)
140ff0c5e: 0f 11 85 18 04 00 00        	movups	%xmm0, 0x418(%rbp)
140ff0c65: 0f 11 85 30 04 00 00        	movups	%xmm0, 0x430(%rbp)
140ff0c6c: 0f 11 85 48 04 00 00        	movups	%xmm0, 0x448(%rbp)
140ff0c73: 0f 11 85 60 04 00 00        	movups	%xmm0, 0x460(%rbp)
140ff0c7a: c7 85 70 04 00 00 00 00 00 00       	movl	$0x0, 0x470(%rbp)
140ff0c84: 48 c7 85 20 03 00 00 00 00 00 00    	movq	$0x0, 0x320(%rbp)
140ff0c8f: 48 c7 85 48 03 00 00 00 00 00 00    	movq	$0x0, 0x348(%rbp)
140ff0c9a: 48 c7 85 50 03 00 00 08 00 00 00    	movq	$0x8, 0x350(%rbp)
140ff0ca5: 48 c7 85 58 03 00 00 00 00 00 00    	movq	$0x0, 0x358(%rbp)
140ff0cb0: 48 c7 85 60 03 00 00 10 00 00 00    	movq	$0x10, 0x360(%rbp)
140ff0cbb: 48 c7 85 78 03 00 00 10 00 00 00    	movq	$0x10, 0x378(%rbp)
140ff0cc6: 48 c7 85 90 03 00 00 10 00 00 00    	movq	$0x10, 0x390(%rbp)
140ff0cd1: 48 c7 85 a8 03 00 00 10 00 00 00    	movq	$0x10, 0x3a8(%rbp)
140ff0cdc: 48 c7 85 c0 03 00 00 10 00 00 00    	movq	$0x10, 0x3c0(%rbp)
140ff0ce7: 48 bb 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rbx # imm = 0x8000000000000000
140ff0cf1: 48 89 9d e0 03 00 00        	movq	%rbx, 0x3e0(%rbp)
140ff0cf8: 48 8d 05 b1 31 bd 01        	leaq	0x1bd31b1(%rip), %rax   # 0x142bc3eb0
140ff0cff: 48 89 85 e8 03 00 00        	movq	%rax, 0x3e8(%rbp)
140ff0d06: 48 c7 85 f0 03 00 00 31 00 00 00    	movq	$0x31, 0x3f0(%rbp)
140ff0d11: 48 c7 85 f8 03 00 00 10 00 00 00    	movq	$0x10, 0x3f8(%rbp)
140ff0d1c: 48 c7 85 10 04 00 00 10 00 00 00    	movq	$0x10, 0x410(%rbp)
140ff0d27: 48 c7 85 28 04 00 00 10 00 00 00    	movq	$0x10, 0x428(%rbp)
140ff0d32: 48 c7 85 40 04 00 00 10 00 00 00    	movq	$0x10, 0x440(%rbp)
140ff0d3d: 48 c7 85 58 04 00 00 10 00 00 00    	movq	$0x10, 0x458(%rbp)
140ff0d48: c7 85 78 04 00 00 00 00 00 00       	movl	$0x0, 0x478(%rbp)
140ff0d52: 66 c7 85 7c 04 00 00 01 00  	movw	$0x1, 0x47c(%rbp)
140ff0d5b: c7 85 80 04 00 00 00 00 00 00       	movl	$0x0, 0x480(%rbp)
140ff0d65: 0f b6 05 9d 3e 64 02        	movzbl	0x2643e9d(%rip), %eax   # 0x143634c09
140ff0d6c: b9 68 01 00 00              	movl	$0x168, %ecx            # imm = 0x168
140ff0d71: ba 08 00 00 00              	movl	$0x8, %edx
140ff0d76: e8 95 2e 62 ff              	callq	0x140613c10 <.text+0x612c10>
140ff0d7b: 48 85 c0                    	testq	%rax, %rax
140ff0d7e: 0f 84 05 06 00 00           	je	0x140ff1389 <.text+0xff0389>
140ff0d84: 48 89 c7                    	movq	%rax, %rdi
140ff0d87: 48 8d 95 20 03 00 00        	leaq	0x320(%rbp), %rdx
140ff0d8e: 41 b8 68 01 00 00           	movl	$0x168, %r8d            # imm = 0x168
140ff0d94: 48 89 c1                    	movq	%rax, %rcx
140ff0d97: e8 64 2e 93 01              	callq	0x142923c00 <.text+0x2922c00>
140ff0d9c: 4c 8d 05 55 30 bd 01        	leaq	0x1bd3055(%rip), %r8    # 0x142bc3df8
140ff0da3: 48 8d 8d 40 02 00 00        	leaq	0x240(%rbp), %rcx
140ff0daa: 48 89 fa                    	movq	%rdi, %rdx
140ff0dad: e8 5e 3d cf ff              	callq	0x140ce4b10 <.text+0xce3b10>
140ff0db2: 48 8d 8d d0 01 00 00        	leaq	0x1d0(%rbp), %rcx
140ff0db9: e8 62 2f ec 00              	callq	0x141eb3d20 <.text+0x1eb2d20>
140ff0dbe: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
140ff0dc5: e8 f6 a8 eb 00              	callq	0x141eab6c0 <.text+0x1eaa6c0>
140ff0dca: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
140ff0dd1: e8 6a 57 ec 00              	callq	0x141eb6540 <.text+0x1eb5540>
140ff0dd6: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
140ff0ddd: e8 3e 25 ec 00              	callq	0x141eb3320 <.text+0x1eb2320>
140ff0de2: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140ff0de6: e8 15 2a ec 00              	callq	0x141eb3800 <.text+0x1eb2800>
140ff0deb: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140ff0def: e8 2c 74 eb 00              	callq	0x141ea8220 <.text+0x1ea7220>
140ff0df4: 0f 57 c0                    	xorps	%xmm0, %xmm0
140ff0df7: 0f 11 85 40 03 00 00        	movups	%xmm0, 0x340(%rbp)
140ff0dfe: 0f 11 85 58 03 00 00        	movups	%xmm0, 0x358(%rbp)
140ff0e05: 0f 11 85 70 03 00 00        	movups	%xmm0, 0x370(%rbp)
140ff0e0c: 0f 11 85 88 03 00 00        	movups	%xmm0, 0x388(%rbp)
140ff0e13: 0f 11 85 a0 03 00 00        	movups	%xmm0, 0x3a0(%rbp)
140ff0e1a: c7 85 b0 03 00 00 00 00 00 00       	movl	$0x0, 0x3b0(%rbp)
140ff0e24: 0f 11 85 d8 03 00 00        	movups	%xmm0, 0x3d8(%rbp)
140ff0e2b: 0f 11 85 f0 03 00 00        	movups	%xmm0, 0x3f0(%rbp)
140ff0e32: 0f 11 85 08 04 00 00        	movups	%xmm0, 0x408(%rbp)
140ff0e39: 0f 11 85 20 04 00 00        	movups	%xmm0, 0x420(%rbp)
140ff0e40: 0f 11 85 38 04 00 00        	movups	%xmm0, 0x438(%rbp)
140ff0e47: c7 85 48 04 00 00 00 00 00 00       	movl	$0x0, 0x448(%rbp)
140ff0e51: 48 c7 85 20 03 00 00 00 00 00 00    	movq	$0x0, 0x320(%rbp)
140ff0e5c: 48 c7 85 28 03 00 00 08 00 00 00    	movq	$0x8, 0x328(%rbp)
140ff0e67: 48 c7 85 30 03 00 00 00 00 00 00    	movq	$0x0, 0x330(%rbp)
140ff0e72: 48 c7 85 38 03 00 00 10 00 00 00    	movq	$0x10, 0x338(%rbp)
140ff0e7d: 48 c7 85 50 03 00 00 10 00 00 00    	movq	$0x10, 0x350(%rbp)
140ff0e88: 48 c7 85 68 03 00 00 10 00 00 00    	movq	$0x10, 0x368(%rbp)
140ff0e93: 48 c7 85 80 03 00 00 10 00 00 00    	movq	$0x10, 0x380(%rbp)
140ff0e9e: 48 c7 85 98 03 00 00 10 00 00 00    	movq	$0x10, 0x398(%rbp)
140ff0ea9: 48 89 9d b8 03 00 00        	movq	%rbx, 0x3b8(%rbp)
140ff0eb0: 48 8d 05 2a 30 bd 01        	leaq	0x1bd302a(%rip), %rax   # 0x142bc3ee1
140ff0eb7: 48 89 85 c0 03 00 00        	movq	%rax, 0x3c0(%rbp)
140ff0ebe: 48 c7 85 c8 03 00 00 3f 00 00 00    	movq	$0x3f, 0x3c8(%rbp)
140ff0ec9: 48 c7 85 d0 03 00 00 10 00 00 00    	movq	$0x10, 0x3d0(%rbp)
140ff0ed4: 48 c7 85 e8 03 00 00 10 00 00 00    	movq	$0x10, 0x3e8(%rbp)
140ff0edf: 48 c7 85 00 04 00 00 10 00 00 00    	movq	$0x10, 0x400(%rbp)
140ff0eea: 48 c7 85 18 04 00 00 10 00 00 00    	movq	$0x10, 0x418(%rbp)
140ff0ef5: 48 c7 85 30 04 00 00 10 00 00 00    	movq	$0x10, 0x430(%rbp)
140ff0f00: c7 85 50 04 00 00 00 00 00 00       	movl	$0x0, 0x450(%rbp)
140ff0f0a: 66 c7 85 54 04 00 00 01 00  	movw	$0x1, 0x454(%rbp)
140ff0f13: 48 89 9d 50 05 00 00        	movq	%rbx, 0x550(%rbp)
140ff0f1a: c7 85 b0 05 00 00 00 00 00 00       	movl	$0x0, 0x5b0(%rbp)
140ff0f24: 0f b6 05 de 3c 64 02        	movzbl	0x2643cde(%rip), %eax   # 0x143634c09
140ff0f2b: b9 98 02 00 00              	movl	$0x298, %ecx            # imm = 0x298
140ff0f30: ba 08 00 00 00              	movl	$0x8, %edx
140ff0f35: e8 d6 2c 62 ff              	callq	0x140613c10 <.text+0x612c10>
140ff0f3a: 48 85 c0                    	testq	%rax, %rax
140ff0f3d: 0f 84 57 04 00 00           	je	0x140ff139a <.text+0xff039a>
140ff0f43: 48 89 c7                    	movq	%rax, %rdi
140ff0f46: 48 8d 95 20 03 00 00        	leaq	0x320(%rbp), %rdx
140ff0f4d: 41 b8 98 02 00 00           	movl	$0x298, %r8d            # imm = 0x298
140ff0f53: 48 89 c1                    	movq	%rax, %rcx
140ff0f56: e8 a5 2c 93 01              	callq	0x142923c00 <.text+0x2922c00>
140ff0f5b: 4c 8d 05 de 2d bd 01        	leaq	0x1bd2dde(%rip), %r8    # 0x142bc3d40
140ff0f62: 48 8d 8d b0 02 00 00        	leaq	0x2b0(%rbp), %rcx
140ff0f69: 48 89 fa                    	movq	%rdi, %rdx
140ff0f6c: e8 9f 3b cf ff              	callq	0x140ce4b10 <.text+0xce3b10>
140ff0f71: 48 8d 8d 20 03 00 00        	leaq	0x320(%rbp), %rcx
140ff0f78: e8 53 7f ec 00              	callq	0x141eb8ed0 <.text+0x1eb7ed0>
140ff0f7d: 0f 10 85 a0 02 00 00        	movups	0x2a0(%rbp), %xmm0
140ff0f84: 48 8b 85 b8 05 00 00        	movq	0x5b8(%rbp), %rax
140ff0f8b: 0f 11 40 60                 	movups	%xmm0, 0x60(%rax)
140ff0f8f: 0f 10 85 90 02 00 00        	movups	0x290(%rbp), %xmm0
140ff0f96: 0f 11 40 50                 	movups	%xmm0, 0x50(%rax)
140ff0f9a: 0f 10 85 80 02 00 00        	movups	0x280(%rbp), %xmm0
140ff0fa1: 0f 11 40 40                 	movups	%xmm0, 0x40(%rax)
140ff0fa5: 0f 10 85 40 02 00 00        	movups	0x240(%rbp), %xmm0
140ff0fac: 0f 10 8d 50 02 00 00        	movups	0x250(%rbp), %xmm1
140ff0fb3: 0f 10 95 60 02 00 00        	movups	0x260(%rbp), %xmm2
140ff0fba: 0f 10 9d 70 02 00 00        	movups	0x270(%rbp), %xmm3
140ff0fc1: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
140ff0fc5: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
140ff0fc9: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
140ff0fcd: 0f 11 00                    	movups	%xmm0, (%rax)
140ff0fd0: 0f 10 85 d0 01 00 00        	movups	0x1d0(%rbp), %xmm0
140ff0fd7: 0f 10 8d e0 01 00 00        	movups	0x1e0(%rbp), %xmm1
140ff0fde: 0f 10 95 f0 01 00 00        	movups	0x1f0(%rbp), %xmm2
140ff0fe5: 0f 10 9d 00 02 00 00        	movups	0x200(%rbp), %xmm3
140ff0fec: 0f 11 40 70                 	movups	%xmm0, 0x70(%rax)
140ff0ff0: 0f 11 88 80 00 00 00        	movups	%xmm1, 0x80(%rax)
140ff0ff7: 0f 11 90 90 00 00 00        	movups	%xmm2, 0x90(%rax)
140ff0ffe: 0f 11 98 a0 00 00 00        	movups	%xmm3, 0xa0(%rax)
140ff1005: 0f 10 85 10 02 00 00        	movups	0x210(%rbp), %xmm0
140ff100c: 0f 11 80 b0 00 00 00        	movups	%xmm0, 0xb0(%rax)
140ff1013: 0f 10 85 20 02 00 00        	movups	0x220(%rbp), %xmm0
140ff101a: 0f 11 80 c0 00 00 00        	movups	%xmm0, 0xc0(%rax)
140ff1021: 0f 10 85 30 02 00 00        	movups	0x230(%rbp), %xmm0
140ff1028: 0f 11 80 d0 00 00 00        	movups	%xmm0, 0xd0(%rax)
140ff102f: 0f 10 85 c0 01 00 00        	movups	0x1c0(%rbp), %xmm0
140ff1036: 0f 11 80 40 01 00 00        	movups	%xmm0, 0x140(%rax)
140ff103d: 0f 10 85 b0 01 00 00        	movups	0x1b0(%rbp), %xmm0
140ff1044: 0f 11 80 30 01 00 00        	movups	%xmm0, 0x130(%rax)
140ff104b: 0f 10 85 a0 01 00 00        	movups	0x1a0(%rbp), %xmm0
140ff1052: 0f 11 80 20 01 00 00        	movups	%xmm0, 0x120(%rax)
140ff1059: 0f 10 85 60 01 00 00        	movups	0x160(%rbp), %xmm0
140ff1060: 0f 10 8d 70 01 00 00        	movups	0x170(%rbp), %xmm1
140ff1067: 0f 10 95 80 01 00 00        	movups	0x180(%rbp), %xmm2
140ff106e: 0f 10 9d 90 01 00 00        	movups	0x190(%rbp), %xmm3
140ff1075: 0f 11 98 10 01 00 00        	movups	%xmm3, 0x110(%rax)
140ff107c: 0f 11 90 00 01 00 00        	movups	%xmm2, 0x100(%rax)
140ff1083: 0f 11 88 f0 00 00 00        	movups	%xmm1, 0xf0(%rax)
140ff108a: 0f 11 80 e0 00 00 00        	movups	%xmm0, 0xe0(%rax)
140ff1091: 0f 10 85 f0 00 00 00        	movups	0xf0(%rbp), %xmm0
140ff1098: 0f 10 8d 00 01 00 00        	movups	0x100(%rbp), %xmm1
140ff109f: 0f 10 95 10 01 00 00        	movups	0x110(%rbp), %xmm2
140ff10a6: 0f 10 9d 20 01 00 00        	movups	0x120(%rbp), %xmm3
140ff10ad: 0f 11 80 50 01 00 00        	movups	%xmm0, 0x150(%rax)
140ff10b4: 0f 11 88 60 01 00 00        	movups	%xmm1, 0x160(%rax)
140ff10bb: 0f 11 90 70 01 00 00        	movups	%xmm2, 0x170(%rax)
140ff10c2: 0f 11 98 80 01 00 00        	movups	%xmm3, 0x180(%rax)
140ff10c9: 0f 10 85 30 01 00 00        	movups	0x130(%rbp), %xmm0
140ff10d0: 0f 11 80 90 01 00 00        	movups	%xmm0, 0x190(%rax)
140ff10d7: 0f 10 85 40 01 00 00        	movups	0x140(%rbp), %xmm0
140ff10de: 0f 11 80 a0 01 00 00        	movups	%xmm0, 0x1a0(%rax)
140ff10e5: 0f 10 85 50 01 00 00        	movups	0x150(%rbp), %xmm0
140ff10ec: 0f 11 80 b0 01 00 00        	movups	%xmm0, 0x1b0(%rax)
140ff10f3: 0f 10 85 80 00 00 00        	movups	0x80(%rbp), %xmm0
140ff10fa: 0f 10 8d 90 00 00 00        	movups	0x90(%rbp), %xmm1
140ff1101: 0f 10 95 a0 00 00 00        	movups	0xa0(%rbp), %xmm2
140ff1108: 0f 10 9d b0 00 00 00        	movups	0xb0(%rbp), %xmm3
140ff110f: 0f 11 80 c0 01 00 00        	movups	%xmm0, 0x1c0(%rax)
140ff1116: 0f 11 88 d0 01 00 00        	movups	%xmm1, 0x1d0(%rax)
140ff111d: 0f 11 90 e0 01 00 00        	movups	%xmm2, 0x1e0(%rax)
140ff1124: 0f 11 98 f0 01 00 00        	movups	%xmm3, 0x1f0(%rax)
140ff112b: 0f 10 85 c0 00 00 00        	movups	0xc0(%rbp), %xmm0
140ff1132: 0f 11 80 00 02 00 00        	movups	%xmm0, 0x200(%rax)
140ff1139: 0f 10 85 e0 00 00 00        	movups	0xe0(%rbp), %xmm0
140ff1140: 0f 11 80 20 02 00 00        	movups	%xmm0, 0x220(%rax)
140ff1147: 0f 10 85 d0 00 00 00        	movups	0xd0(%rbp), %xmm0
140ff114e: 0f 11 80 10 02 00 00        	movups	%xmm0, 0x210(%rax)
140ff1155: 0f 10 45 70                 	movups	0x70(%rbp), %xmm0
140ff1159: 0f 11 80 90 02 00 00        	movups	%xmm0, 0x290(%rax)
140ff1160: 0f 10 45 60                 	movups	0x60(%rbp), %xmm0
140ff1164: 0f 11 80 80 02 00 00        	movups	%xmm0, 0x280(%rax)
140ff116b: 0f 10 45 50                 	movups	0x50(%rbp), %xmm0
140ff116f: 0f 11 80 70 02 00 00        	movups	%xmm0, 0x270(%rax)
140ff1176: 0f 10 45 10                 	movups	0x10(%rbp), %xmm0
140ff117a: 0f 10 4d 20                 	movups	0x20(%rbp), %xmm1
140ff117e: 0f 10 55 30                 	movups	0x30(%rbp), %xmm2
140ff1182: 0f 10 5d 40                 	movups	0x40(%rbp), %xmm3
140ff1186: 0f 11 98 60 02 00 00        	movups	%xmm3, 0x260(%rax)
140ff118d: 0f 11 90 50 02 00 00        	movups	%xmm2, 0x250(%rax)
140ff1194: 0f 11 88 40 02 00 00        	movups	%xmm1, 0x240(%rax)
140ff119b: 0f 11 80 30 02 00 00        	movups	%xmm0, 0x230(%rax)
140ff11a2: 0f 10 45 a0                 	movups	-0x60(%rbp), %xmm0
140ff11a6: 0f 10 4d b0                 	movups	-0x50(%rbp), %xmm1
140ff11aa: 0f 10 55 c0                 	movups	-0x40(%rbp), %xmm2
140ff11ae: 0f 10 5d d0                 	movups	-0x30(%rbp), %xmm3
140ff11b2: 0f 11 80 a0 02 00 00        	movups	%xmm0, 0x2a0(%rax)
140ff11b9: 0f 11 88 b0 02 00 00        	movups	%xmm1, 0x2b0(%rax)
140ff11c0: 0f 11 90 c0 02 00 00        	movups	%xmm2, 0x2c0(%rax)
140ff11c7: 0f 11 98 d0 02 00 00        	movups	%xmm3, 0x2d0(%rax)
140ff11ce: 0f 10 45 e0                 	movups	-0x20(%rbp), %xmm0
140ff11d2: 0f 11 80 e0 02 00 00        	movups	%xmm0, 0x2e0(%rax)
140ff11d9: 0f 10 45 f0                 	movups	-0x10(%rbp), %xmm0
140ff11dd: 0f 11 80 f0 02 00 00        	movups	%xmm0, 0x2f0(%rax)
140ff11e4: 0f 10 45 00                 	movups	(%rbp), %xmm0
140ff11e8: 0f 11 80 00 03 00 00        	movups	%xmm0, 0x300(%rax)
140ff11ef: 0f 10 85 b0 02 00 00        	movups	0x2b0(%rbp), %xmm0
140ff11f6: 0f 10 8d c0 02 00 00        	movups	0x2c0(%rbp), %xmm1
140ff11fd: 0f 10 95 d0 02 00 00        	movups	0x2d0(%rbp), %xmm2
140ff1204: 0f 10 9d e0 02 00 00        	movups	0x2e0(%rbp), %xmm3
140ff120b: 0f 11 80 10 03 00 00        	movups	%xmm0, 0x310(%rax)
140ff1212: 0f 11 88 20 03 00 00        	movups	%xmm1, 0x320(%rax)
140ff1219: 0f 11 90 30 03 00 00        	movups	%xmm2, 0x330(%rax)
140ff1220: 0f 11 98 40 03 00 00        	movups	%xmm3, 0x340(%rax)
140ff1227: 0f 10 85 f0 02 00 00        	movups	0x2f0(%rbp), %xmm0
140ff122e: 0f 11 80 50 03 00 00        	movups	%xmm0, 0x350(%rax)
140ff1235: 0f 10 85 00 03 00 00        	movups	0x300(%rbp), %xmm0
140ff123c: 0f 11 80 60 03 00 00        	movups	%xmm0, 0x360(%rax)
140ff1243: 0f 10 85 10 03 00 00        	movups	0x310(%rbp), %xmm0
140ff124a: 0f 11 80 70 03 00 00        	movups	%xmm0, 0x370(%rax)
140ff1251: 0f 10 85 20 03 00 00        	movups	0x320(%rbp), %xmm0
140ff1258: 0f 10 8d 30 03 00 00        	movups	0x330(%rbp), %xmm1
140ff125f: 0f 10 95 40 03 00 00        	movups	0x340(%rbp), %xmm2
140ff1266: 0f 10 9d 50 03 00 00        	movups	0x350(%rbp), %xmm3
140ff126d: 0f 11 80 80 03 00 00        	movups	%xmm0, 0x380(%rax)
140ff1274: 0f 11 88 90 03 00 00        	movups	%xmm1, 0x390(%rax)
140ff127b: 0f 11 90 a0 03 00 00        	movups	%xmm2, 0x3a0(%rax)
140ff1282: 0f 11 98 b0 03 00 00        	movups	%xmm3, 0x3b0(%rax)
140ff1289: 0f 10 85 60 03 00 00        	movups	0x360(%rbp), %xmm0
140ff1290: 0f 11 80 c0 03 00 00        	movups	%xmm0, 0x3c0(%rax)
140ff1297: 0f 10 85 70 03 00 00        	movups	0x370(%rbp), %xmm0
140ff129e: 0f 11 80 d0 03 00 00        	movups	%xmm0, 0x3d0(%rax)
140ff12a5: 0f 10 85 80 03 00 00        	movups	0x380(%rbp), %xmm0
140ff12ac: 0f 11 80 e0 03 00 00        	movups	%xmm0, 0x3e0(%rax)
140ff12b3: 48 c7 85 28 03 00 00 09 00 00 00    	movq	$0x9, 0x328(%rbp)
140ff12be: 48 89 85 30 03 00 00        	movq	%rax, 0x330(%rbp)
140ff12c5: 48 c7 85 38 03 00 00 09 00 00 00    	movq	$0x9, 0x338(%rbp)
140ff12d0: 48 c7 85 40 03 00 00 00 00 00 00    	movq	$0x0, 0x340(%rbp)
140ff12db: 48 c7 85 48 03 00 00 08 00 00 00    	movq	$0x8, 0x348(%rbp)
140ff12e6: 0f 57 c0                    	xorps	%xmm0, %xmm0
140ff12e9: 0f 11 85 50 03 00 00        	movups	%xmm0, 0x350(%rbp)
140ff12f0: 48 8b 85 c0 02 00 00        	movq	0x2c0(%rbp), %rax
140ff12f7: 48 89 85 70 03 00 00        	movq	%rax, 0x370(%rbp)
140ff12fe: 0f 10 85 b0 02 00 00        	movups	0x2b0(%rbp), %xmm0
140ff1305: 0f 11 85 60 03 00 00        	movups	%xmm0, 0x360(%rbp)
140ff130c: 48 89 9d 20 03 00 00        	movq	%rbx, 0x320(%rbp)
140ff1313: 48 8d 8d 58 03 00 00        	leaq	0x358(%rbp), %rcx
140ff131a: e8 01 34 fe ff              	callq	0x140fd4720 <.text+0xfd3720>
140ff131f: 0f 10 85 80 03 00 00        	movups	0x380(%rbp), %xmm0
140ff1326: 0f 11 46 60                 	movups	%xmm0, 0x60(%rsi)
140ff132a: 0f 10 85 70 03 00 00        	movups	0x370(%rbp), %xmm0
140ff1331: 0f 11 46 50                 	movups	%xmm0, 0x50(%rsi)
140ff1335: 0f 10 85 60 03 00 00        	movups	0x360(%rbp), %xmm0
140ff133c: 0f 11 46 40                 	movups	%xmm0, 0x40(%rsi)
140ff1340: 0f 10 85 20 03 00 00        	movups	0x320(%rbp), %xmm0
140ff1347: 0f 10 8d 30 03 00 00        	movups	0x330(%rbp), %xmm1
140ff134e: 0f 10 95 40 03 00 00        	movups	0x340(%rbp), %xmm2
140ff1355: 0f 10 9d 50 03 00 00        	movups	0x350(%rbp), %xmm3
140ff135c: 0f 11 5e 30                 	movups	%xmm3, 0x30(%rsi)
140ff1360: 0f 11 56 20                 	movups	%xmm2, 0x20(%rsi)
140ff1364: 0f 11 4e 10                 	movups	%xmm1, 0x10(%rsi)
140ff1368: 0f 11 06                    	movups	%xmm0, (%rsi)
140ff136b: 48 89 f0                    	movq	%rsi, %rax
140ff136e: 48 81 c4 48 06 00 00        	addq	$0x648, %rsp            # imm = 0x648
140ff1375: 5b                          	popq	%rbx
140ff1376: 5f                          	popq	%rdi
140ff1377: 5e                          	popq	%rsi
140ff1378: 5d                          	popq	%rbp
140ff1379: c3                          	retq
140ff137a: b9 08 00 00 00              	movl	$0x8, %ecx
140ff137f: ba f0 03 00 00              	movl	$0x3f0, %edx            # imm = 0x3F0
140ff1384: e8 77 7c 8e 01              	callq	0x1428d9000 <.text+0x28d8000>
140ff1389: b9 08 00 00 00              	movl	$0x8, %ecx
140ff138e: ba 68 01 00 00              	movl	$0x168, %edx            # imm = 0x168
140ff1393: e8 68 7c 8e 01              	callq	0x1428d9000 <.text+0x28d8000>
140ff1398: eb 0f                       	jmp	0x140ff13a9 <.text+0xff03a9>
140ff139a: b9 08 00 00 00              	movl	$0x8, %ecx
140ff139f: ba 98 02 00 00              	movl	$0x298, %edx            # imm = 0x298
140ff13a4: e8 57 7c 8e 01              	callq	0x1428d9000 <.text+0x28d8000>
140ff13a9: 0f 0b                       	ud2
140ff13ab: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140ff13b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140ff13b5: 55                          	pushq	%rbp
140ff13b6: 56                          	pushq	%rsi
140ff13b7: 57                          	pushq	%rdi
140ff13b8: 53                          	pushq	%rbx
140ff13b9: 48 83 ec 28                 	subq	$0x28, %rsp
140ff13bd: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140ff13c4: 48 8d 8d 20 03 00 00        	leaq	0x320(%rbp), %rcx
140ff13cb: e8 40 5b 5f ff              	callq	0x1405e6f10 <.text+0x5e5f10>
140ff13d0: 90                          	nop
140ff13d1: 48 83 c4 28                 	addq	$0x28, %rsp
140ff13d5: 5b                          	popq	%rbx
140ff13d6: 5f                          	popq	%rdi
140ff13d7: 5e                          	popq	%rsi
140ff13d8: 5d                          	popq	%rbp
140ff13d9: c3                          	retq
140ff13da: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140ff13e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140ff13e5: 55                          	pushq	%rbp
140ff13e6: 56                          	pushq	%rsi
140ff13e7: 57                          	pushq	%rdi
140ff13e8: 53                          	pushq	%rbx
140ff13e9: 48 83 ec 28                 	subq	$0x28, %rsp
140ff13ed: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140ff13f4: ba f0 03 00 00              	movl	$0x3f0, %edx            # imm = 0x3F0
140ff13f9: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140ff13ff: 48 8b 8d b8 05 00 00        	movq	0x5b8(%rbp), %rcx
140ff1406: e8 15 28 62 ff              	callq	0x140613c20 <.text+0x612c20>
140ff140b: 90                          	nop
140ff140c: 48 83 c4 28                 	addq	$0x28, %rsp
140ff1410: 5b                          	popq	%rbx
140ff1411: 5f                          	popq	%rdi
140ff1412: 5e                          	popq	%rsi
140ff1413: 5d                          	popq	%rbp
140ff1414: c3                          	retq
140ff1415: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140ff1420: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140ff1425: 55                          	pushq	%rbp
140ff1426: 56                          	pushq	%rsi
140ff1427: 57                          	pushq	%rdi
140ff1428: 53                          	pushq	%rbx
140ff1429: 48 83 ec 28                 	subq	$0x28, %rsp
140ff142d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140ff1434: 48 8d 8d 20 03 00 00        	leaq	0x320(%rbp), %rcx
140ff143b: e8 70 df 5d ff              	callq	0x1405cf3b0 <.text+0x5ce3b0>
140ff1440: 90                          	nop
140ff1441: 48 83 c4 28                 	addq	$0x28, %rsp
140ff1445: 5b                          	popq	%rbx
140ff1446: 5f                          	popq	%rdi
140ff1447: 5e                          	popq	%rsi
140ff1448: 5d                          	popq	%rbp
140ff1449: c3                          	retq
140ff144a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140ff1450: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140ff1455: 55                          	pushq	%rbp
140ff1456: 56                          	pushq	%rsi
140ff1457: 57                          	pushq	%rdi
140ff1458: 53                          	pushq	%rbx
140ff1459: 48 83 ec 28                 	subq	$0x28, %rsp
140ff145d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140ff1464: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140ff1468: e8 13 84 03 ff              	callq	0x140029880 <.text+0x28880>
140ff146d: 90                          	nop
140ff146e: 48 83 c4 28                 	addq	$0x28, %rsp
140ff1472: 5b                          	popq	%rbx
140ff1473: 5f                          	popq	%rdi
140ff1474: 5e                          	popq	%rsi
140ff1475: 5d                          	popq	%rbp
140ff1476: c3                          	retq
140ff1477: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140ff1480: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140ff1485: 55                          	pushq	%rbp
140ff1486: 56                          	pushq	%rsi
140ff1487: 57                          	pushq	%rdi
140ff1488: 53                          	pushq	%rbx
140ff1489: 48 83 ec 28                 	subq	$0x28, %rsp
140ff148d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140ff1494: 48 8d 8d 40 02 00 00        	leaq	0x240(%rbp), %rcx
140ff149b: e8 e0 83 03 ff              	callq	0x140029880 <.text+0x28880>
140ff14a0: 90                          	nop
140ff14a1: 48 83 c4 28                 	addq	$0x28, %rsp
140ff14a5: 5b                          	popq	%rbx
140ff14a6: 5f                          	popq	%rdi
140ff14a7: 5e                          	popq	%rsi
140ff14a8: 5d                          	popq	%rbp
140ff14a9: c3                          	retq
140ff14aa: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140ff14b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140ff14b5: 55                          	pushq	%rbp
140ff14b6: 56                          	pushq	%rsi
140ff14b7: 57                          	pushq	%rdi
140ff14b8: 53                          	pushq	%rbx
140ff14b9: 48 83 ec 28                 	subq	$0x28, %rsp
140ff14bd: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140ff14c4: 48 8d 8d d0 01 00 00        	leaq	0x1d0(%rbp), %rcx
140ff14cb: e8 b0 83 03 ff              	callq	0x140029880 <.text+0x28880>
140ff14d0: 90                          	nop
140ff14d1: 48 83 c4 28                 	addq	$0x28, %rsp
140ff14d5: 5b                          	popq	%rbx
140ff14d6: 5f                          	popq	%rdi
140ff14d7: 5e                          	popq	%rsi
140ff14d8: 5d                          	popq	%rbp
140ff14d9: c3                          	retq
140ff14da: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140ff14e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140ff14e5: 55                          	pushq	%rbp
140ff14e6: 56                          	pushq	%rsi
140ff14e7: 57                          	pushq	%rdi
140ff14e8: 53                          	pushq	%rbx
140ff14e9: 48 83 ec 28                 	subq	$0x28, %rsp
140ff14ed: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140ff14f4: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
140ff14fb: e8 80 83 03 ff              	callq	0x140029880 <.text+0x28880>
140ff1500: 90                          	nop
140ff1501: 48 83 c4 28                 	addq	$0x28, %rsp
140ff1505: 5b                          	popq	%rbx
140ff1506: 5f                          	popq	%rdi
140ff1507: 5e                          	popq	%rsi
140ff1508: 5d                          	popq	%rbp
140ff1509: c3                          	retq
140ff150a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140ff1510: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140ff1515: 55                          	pushq	%rbp
140ff1516: 56                          	pushq	%rsi
140ff1517: 57                          	pushq	%rdi
140ff1518: 53                          	pushq	%rbx
140ff1519: 48 83 ec 28                 	subq	$0x28, %rsp
140ff151d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140ff1524: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
140ff152b: e8 50 83 03 ff              	callq	0x140029880 <.text+0x28880>
140ff1530: 90                          	nop
140ff1531: 48 83 c4 28                 	addq	$0x28, %rsp
140ff1535: 5b                          	popq	%rbx
140ff1536: 5f                          	popq	%rdi
140ff1537: 5e                          	popq	%rsi
140ff1538: 5d                          	popq	%rbp
140ff1539: c3                          	retq
140ff153a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140ff1540: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140ff1545: 55                          	pushq	%rbp
140ff1546: 56                          	pushq	%rsi
140ff1547: 57                          	pushq	%rdi
140ff1548: 53                          	pushq	%rbx
140ff1549: 48 83 ec 28                 	subq	$0x28, %rsp
140ff154d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140ff1554: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
140ff155b: e8 20 83 03 ff              	callq	0x140029880 <.text+0x28880>
140ff1560: 90                          	nop
140ff1561: 48 83 c4 28                 	addq	$0x28, %rsp
140ff1565: 5b                          	popq	%rbx
140ff1566: 5f                          	popq	%rdi
140ff1567: 5e                          	popq	%rsi
140ff1568: 5d                          	popq	%rbp
140ff1569: c3                          	retq
140ff156a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140ff1570: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140ff1575: 55                          	pushq	%rbp
140ff1576: 56                          	pushq	%rsi
140ff1577: 57                          	pushq	%rdi
140ff1578: 53                          	pushq	%rbx
140ff1579: 48 83 ec 28                 	subq	$0x28, %rsp
140ff157d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140ff1584: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140ff1588: e8 f3 82 03 ff              	callq	0x140029880 <.text+0x28880>
140ff158d: 90                          	nop
140ff158e: 48 83 c4 28                 	addq	$0x28, %rsp
140ff1592: 5b                          	popq	%rbx
140ff1593: 5f                          	popq	%rdi
140ff1594: 5e                          	popq	%rsi
140ff1595: 5d                          	popq	%rbp
140ff1596: c3                          	retq
140ff1597: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140ff15a0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140ff15a5: 55                          	pushq	%rbp
140ff15a6: 56                          	pushq	%rsi
140ff15a7: 57                          	pushq	%rdi
140ff15a8: 53                          	pushq	%rbx
140ff15a9: 48 83 ec 28                 	subq	$0x28, %rsp
140ff15ad: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140ff15b4: 48 8d 8d b0 02 00 00        	leaq	0x2b0(%rbp), %rcx
140ff15bb: e8 c0 82 03 ff              	callq	0x140029880 <.text+0x28880>
140ff15c0: 90                          	nop
140ff15c1: 48 83 c4 28                 	addq	$0x28, %rsp
140ff15c5: 5b                          	popq	%rbx
140ff15c6: 5f                          	popq	%rdi
140ff15c7: 5e                          	popq	%rsi
140ff15c8: 5d                          	popq	%rbp
140ff15c9: c3                          	retq
140ff15ca: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140ff15d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140ff15d5: 55                          	pushq	%rbp
140ff15d6: 56                          	pushq	%rsi
140ff15d7: 57                          	pushq	%rdi
140ff15d8: 53                          	pushq	%rbx
140ff15d9: 48 83 ec 28                 	subq	$0x28, %rsp
140ff15dd: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140ff15e4: 48 8d 8d 20 03 00 00        	leaq	0x320(%rbp), %rcx
140ff15eb: e8 90 82 03 ff              	callq	0x140029880 <.text+0x28880>
140ff15f0: 90                          	nop
140ff15f1: 48 83 c4 28                 	addq	$0x28, %rsp
140ff15f5: 5b                          	popq	%rbx
140ff15f6: 5f                          	popq	%rdi
140ff15f7: 5e                          	popq	%rsi
140ff15f8: 5d                          	popq	%rbp
140ff15f9: c3                          	retq
140ff15fa: cc                          	int3
140ff15fb: cc                          	int3
140ff15fc: cc                          	int3
140ff15fd: cc                          	int3
140ff15fe: cc                          	int3
140ff15ff: cc                          	int3
140ff1600: 55                          	pushq	%rbp
140ff1601: 56                          	pushq	%rsi
140ff1602: 57                          	pushq	%rdi
140ff1603: 48 81 ec 20 01 00 00        	subq	$0x120, %rsp            # imm = 0x120
140ff160a: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140ff1612: 48 c7 85 98 00 00 00 fe ff ff ff    	movq	$-0x2, 0x98(%rbp)
140ff161d: 48 89 d6                    	movq	%rdx, %rsi
140ff1620: 48 89 cf                    	movq	%rcx, %rdi
140ff1623: 31 c0                       	xorl	%eax, %eax
140ff1625: 48 3b 81 38 01 00 00        	cmpq	0x138(%rcx), %rax
140ff162c: 0f 81 f2 01 00 00           	jno	0x140ff1824 <.text+0xff0824>
140ff1632: 48 89 f1                    	movq	%rsi, %rcx
140ff1635: 48 89 fa                    	movq	%rdi, %rdx
140ff1638: e8 93 ce dd 00              	callq	0x141dce4d0 <.text+0x1dcd4d0>
140ff163d: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
140ff1641: 48 89 f1                    	movq	%rsi, %rcx
140ff1644: 48 89 fa                    	movq	%rdi, %rdx
140ff1647: e8 d4 ad dd 00              	callq	0x141dcc420 <.text+0x1dcb420>
140ff164c: 48 89 45 28                 	movq	%rax, 0x28(%rbp)
140ff1650: 48 89 f1                    	movq	%rsi, %rcx
140ff1653: 48 89 fa                    	movq	%rdi, %rdx
140ff1656: e8 a5 dd dd 00              	callq	0x141dcf400 <.text+0x1dce400>
140ff165b: 48 89 45 30                 	movq	%rax, 0x30(%rbp)
140ff165f: 48 89 f1                    	movq	%rsi, %rcx
140ff1662: 48 89 fa                    	movq	%rdi, %rdx
140ff1665: e8 06 6a dd 00              	callq	0x141dc8070 <.text+0x1dc7070>
140ff166a: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
140ff166e: 48 89 f1                    	movq	%rsi, %rcx
140ff1671: 48 89 fa                    	movq	%rdi, %rdx
140ff1674: e8 07 75 dd 00              	callq	0x141dc8b80 <.text+0x1dc7b80>
140ff1679: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
140ff167d: 48 89 f1                    	movq	%rsi, %rcx
140ff1680: 48 89 fa                    	movq	%rdi, %rdx
140ff1683: e8 58 24 df 00              	callq	0x141de3ae0 <.text+0x1de2ae0>
140ff1688: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
140ff168c: 48 89 f1                    	movq	%rsi, %rcx
140ff168f: 48 89 fa                    	movq	%rdi, %rdx
140ff1692: e8 39 4c df 00              	callq	0x141de62d0 <.text+0x1de52d0>
140ff1697: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
140ff169b: 48 89 f1                    	movq	%rsi, %rcx
140ff169e: 48 89 fa                    	movq	%rdi, %rdx
140ff16a1: e8 0a 00 df 00              	callq	0x141de16b0 <.text+0x1de06b0>
140ff16a6: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
140ff16aa: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
140ff16ae: 48 89 f2                    	movq	%rsi, %rdx
140ff16b1: 49 89 f8                    	movq	%rdi, %r8
140ff16b4: e8 77 2e d2 ff              	callq	0x140d14530 <.text+0xd13530>
140ff16b9: 48 89 f1                    	movq	%rsi, %rcx
140ff16bc: 48 89 fa                    	movq	%rdi, %rdx
140ff16bf: e8 fc 67 df 00              	callq	0x141de7ec0 <.text+0x1de6ec0>
140ff16c4: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
140ff16c8: 48 89 f1                    	movq	%rsi, %rcx
140ff16cb: 48 89 fa                    	movq	%rdi, %rdx
140ff16ce: e8 fd 57 df 00              	callq	0x141de6ed0 <.text+0x1de5ed0>
140ff16d3: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
140ff16d7: 48 89 f1                    	movq	%rsi, %rcx
140ff16da: 48 89 fa                    	movq	%rdi, %rdx
140ff16dd: e8 6e 9f de 00              	callq	0x141ddb650 <.text+0x1dda650>
140ff16e2: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
140ff16e6: 48 89 f1                    	movq	%rsi, %rcx
140ff16e9: 48 89 fa                    	movq	%rdi, %rdx
140ff16ec: e8 cf fb de 00              	callq	0x141de12c0 <.text+0x1de02c0>
140ff16f1: 48 89 45 78                 	movq	%rax, 0x78(%rbp)
140ff16f5: 48 89 bd 80 00 00 00        	movq	%rdi, 0x80(%rbp)
140ff16fc: 48 8d 8f 38 01 00 00        	leaq	0x138(%rdi), %rcx
140ff1703: 48 8b 45 d8                 	movq	-0x28(%rbp), %rax
140ff1707: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
140ff170b: 0f 10 45 a8                 	movups	-0x58(%rbp), %xmm0
140ff170f: 0f 10 4d b8                 	movups	-0x48(%rbp), %xmm1
140ff1713: 0f 10 55 c8                 	movups	-0x38(%rbp), %xmm2
140ff1717: 0f 29 55 00                 	movaps	%xmm2, (%rbp)
140ff171b: 0f 29 4d f0                 	movaps	%xmm1, -0x10(%rbp)
140ff171f: 0f 29 45 e0                 	movaps	%xmm0, -0x20(%rbp)
140ff1723: 48 8b 86 a0 05 00 00        	movq	0x5a0(%rsi), %rax
140ff172a: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
140ff1731: 31 c0                       	xorl	%eax, %eax
140ff1733: 48 89 8d 90 00 00 00        	movq	%rcx, 0x90(%rbp)
140ff173a: 48 3b 01                    	cmpq	(%rcx), %rax
140ff173d: 70 0c                       	jo	0x140ff174b <.text+0xff074b>
140ff173f: 48 8b 8d 90 00 00 00        	movq	0x90(%rbp), %rcx
140ff1746: e8 d5 f3 5d ff              	callq	0x1405d0b20 <.text+0x5cfb20>
140ff174b: 48 8b 45 10                 	movq	0x10(%rbp), %rax
140ff174f: 48 8b 8d 90 00 00 00        	movq	0x90(%rbp), %rcx
140ff1756: 48 89 41 30                 	movq	%rax, 0x30(%rcx)
140ff175a: 0f 28 45 e0                 	movaps	-0x20(%rbp), %xmm0
140ff175e: 0f 28 4d f0                 	movaps	-0x10(%rbp), %xmm1
140ff1762: 0f 28 55 00                 	movaps	(%rbp), %xmm2
140ff1766: 0f 11 51 20                 	movups	%xmm2, 0x20(%rcx)
140ff176a: 0f 11 49 10                 	movups	%xmm1, 0x10(%rcx)
140ff176e: 0f 11 01                    	movups	%xmm0, (%rcx)
140ff1771: 48 8b bd 80 00 00 00        	movq	0x80(%rbp), %rdi
140ff1778: c7 87 70 01 00 00 00 00 00 00       	movl	$0x0, 0x170(%rdi)
140ff1782: c6 87 74 01 00 00 06        	movb	$0x6, 0x174(%rdi)
140ff1789: 48 8b 45 20                 	movq	0x20(%rbp), %rax
140ff178d: 48 89 87 78 01 00 00        	movq	%rax, 0x178(%rdi)
140ff1794: 48 8b 45 28                 	movq	0x28(%rbp), %rax
140ff1798: 48 89 87 80 01 00 00        	movq	%rax, 0x180(%rdi)
140ff179f: 48 8b 45 30                 	movq	0x30(%rbp), %rax
140ff17a3: 48 89 87 88 01 00 00        	movq	%rax, 0x188(%rdi)
140ff17aa: 48 8b 45 38                 	movq	0x38(%rbp), %rax
140ff17ae: 48 89 87 90 01 00 00        	movq	%rax, 0x190(%rdi)
140ff17b5: 48 8b 45 40                 	movq	0x40(%rbp), %rax
140ff17b9: 48 89 87 98 01 00 00        	movq	%rax, 0x198(%rdi)
140ff17c0: 48 8b 45 48                 	movq	0x48(%rbp), %rax
140ff17c4: 48 89 87 a0 01 00 00        	movq	%rax, 0x1a0(%rdi)
140ff17cb: 48 8b 45 50                 	movq	0x50(%rbp), %rax
140ff17cf: 48 89 87 a8 01 00 00        	movq	%rax, 0x1a8(%rdi)
140ff17d6: 48 8b 45 58                 	movq	0x58(%rbp), %rax
140ff17da: 48 89 87 b0 01 00 00        	movq	%rax, 0x1b0(%rdi)
140ff17e1: 48 8b 45 60                 	movq	0x60(%rbp), %rax
140ff17e5: 48 89 87 b8 01 00 00        	movq	%rax, 0x1b8(%rdi)
140ff17ec: 48 8b 45 68                 	movq	0x68(%rbp), %rax
140ff17f0: 48 89 87 c0 01 00 00        	movq	%rax, 0x1c0(%rdi)
140ff17f7: 48 8b 45 70                 	movq	0x70(%rbp), %rax
140ff17fb: 48 89 87 c8 01 00 00        	movq	%rax, 0x1c8(%rdi)
140ff1802: c6 87 d0 01 00 00 00        	movb	$0x0, 0x1d0(%rdi)
140ff1809: 48 8b 45 78                 	movq	0x78(%rbp), %rax
140ff180d: 48 89 87 d8 01 00 00        	movq	%rax, 0x1d8(%rdi)
140ff1814: 48 8b 85 88 00 00 00        	movq	0x88(%rbp), %rax
140ff181b: 48 89 87 e0 01 00 00        	movq	%rax, 0x1e0(%rdi)
140ff1822: eb 14                       	jmp	0x140ff1838 <.text+0xff0838>
140ff1824: 48 8b 86 a0 05 00 00        	movq	0x5a0(%rsi), %rax
140ff182b: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
140ff182f: 48 39 87 e0 01 00 00        	cmpq	%rax, 0x1e0(%rdi)
140ff1836: 75 1c                       	jne	0x140ff1854 <.text+0xff0854>
140ff1838: b8 00 50 cc 3d              	movl	$0x3dcc5000, %eax       # imm = 0x3DCC5000
140ff183d: 03 86 b0 05 00 00           	addl	0x5b0(%rsi), %eax
140ff1843: 89 87 30 01 00 00           	movl	%eax, 0x130(%rdi)
140ff1849: 48 81 c4 20 01 00 00        	addq	$0x120, %rsp            # imm = 0x120
140ff1850: 5f                          	popq	%rdi
140ff1851: 5e                          	popq	%rsi
140ff1852: 5d                          	popq	%rbp
140ff1853: c3                          	retq
140ff1854: 48 8d 05 8d 2a bd 01        	leaq	0x1bd2a8d(%rip), %rax   # 0x142bc42e8
140ff185b: 48 89 45 a8                 	movq	%rax, -0x58(%rbp)
140ff185f: 48 c7 45 b0 01 00 00 00     	movq	$0x1, -0x50(%rbp)
140ff1867: 48 c7 45 b8 08 00 00 00     	movq	$0x8, -0x48(%rbp)
140ff186f: 0f 57 c0                    	xorps	%xmm0, %xmm0
140ff1872: 0f 11 45 c0                 	movups	%xmm0, -0x40(%rbp)
140ff1876: 48 81 c7 e0 01 00 00        	addq	$0x1e0, %rdi            # imm = 0x1E0
140ff187d: 48 8d 05 e4 2a bd 01        	leaq	0x1bd2ae4(%rip), %rax   # 0x142bc4368
140ff1884: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140ff1889: 4c 8d 45 e0                 	leaq	-0x20(%rbp), %r8
140ff188d: 4c 8d 4d a8                 	leaq	-0x58(%rbp), %r9
140ff1891: 31 c9                       	xorl	%ecx, %ecx
140ff1893: 48 89 fa                    	movq	%rdi, %rdx
140ff1896: e8 46 c8 8d 01              	callq	0x1428ce0e1 <.text+0x28cd0e1>
140ff189b: cc                          	int3
140ff189c: 0f 1f 40 00                 	nopl	(%rax)
140ff18a0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140ff18a5: 55                          	pushq	%rbp
140ff18a6: 56                          	pushq	%rsi
140ff18a7: 57                          	pushq	%rdi
140ff18a8: 48 83 ec 30                 	subq	$0x30, %rsp
140ff18ac: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140ff18b3: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
140ff18b7: e8 64 f2 5d ff              	callq	0x1405d0b20 <.text+0x5cfb20>
140ff18bc: 90                          	nop
140ff18bd: 48 83 c4 30                 	addq	$0x30, %rsp
140ff18c1: 5f                          	popq	%rdi
140ff18c2: 5e                          	popq	%rsi
140ff18c3: 5d                          	popq	%rbp
140ff18c4: c3                          	retq
140ff18c5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140ff18d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140ff18d5: 55                          	pushq	%rbp
140ff18d6: 56                          	pushq	%rsi
140ff18d7: 57                          	pushq	%rdi
140ff18d8: 48 83 ec 30                 	subq	$0x30, %rsp
140ff18dc: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140ff18e3: 48 8b 45 10                 	movq	0x10(%rbp), %rax
140ff18e7: 48 8b 8d 90 00 00 00        	movq	0x90(%rbp), %rcx
140ff18ee: 48 89 41 30                 	movq	%rax, 0x30(%rcx)
140ff18f2: 0f 28 45 e0                 	movaps	-0x20(%rbp), %xmm0
140ff18f6: 0f 28 4d f0                 	movaps	-0x10(%rbp), %xmm1
140ff18fa: 0f 28 55 00                 	movaps	(%rbp), %xmm2
140ff18fe: 0f 11 51 20                 	movups	%xmm2, 0x20(%rcx)
140ff1902: 0f 11 49 10                 	movups	%xmm1, 0x10(%rcx)
140ff1906: 0f 11 01                    	movups	%xmm0, (%rcx)
140ff1909: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
140ff1910: c7 80 70 01 00 00 00 00 00 00       	movl	$0x0, 0x170(%rax)
140ff191a: c6 80 74 01 00 00 06        	movb	$0x6, 0x174(%rax)
140ff1921: 48 8b 4d 20                 	movq	0x20(%rbp), %rcx
140ff1925: 48 89 88 78 01 00 00        	movq	%rcx, 0x178(%rax)
140ff192c: 48 8b 4d 28                 	movq	0x28(%rbp), %rcx
140ff1930: 48 89 88 80 01 00 00        	movq	%rcx, 0x180(%rax)
140ff1937: 48 8b 4d 30                 	movq	0x30(%rbp), %rcx
140ff193b: 48 89 88 88 01 00 00        	movq	%rcx, 0x188(%rax)
140ff1942: 48 8b 4d 38                 	movq	0x38(%rbp), %rcx
140ff1946: 48 89 88 90 01 00 00        	movq	%rcx, 0x190(%rax)
140ff194d: 48 8b 4d 40                 	movq	0x40(%rbp), %rcx
140ff1951: 48 89 88 98 01 00 00        	movq	%rcx, 0x198(%rax)
140ff1958: 48 8b 4d 48                 	movq	0x48(%rbp), %rcx
140ff195c: 48 89 88 a0 01 00 00        	movq	%rcx, 0x1a0(%rax)
140ff1963: 48 8b 4d 50                 	movq	0x50(%rbp), %rcx
140ff1967: 48 89 88 a8 01 00 00        	movq	%rcx, 0x1a8(%rax)
140ff196e: 48 8b 4d 58                 	movq	0x58(%rbp), %rcx
140ff1972: 48 89 88 b0 01 00 00        	movq	%rcx, 0x1b0(%rax)
140ff1979: 48 8b 4d 60                 	movq	0x60(%rbp), %rcx
140ff197d: 48 89 88 b8 01 00 00        	movq	%rcx, 0x1b8(%rax)
140ff1984: 48 8b 4d 68                 	movq	0x68(%rbp), %rcx
140ff1988: 48 89 88 c0 01 00 00        	movq	%rcx, 0x1c0(%rax)
140ff198f: 48 8b 4d 70                 	movq	0x70(%rbp), %rcx
140ff1993: 48 89 88 c8 01 00 00        	movq	%rcx, 0x1c8(%rax)
140ff199a: c6 80 d0 01 00 00 00        	movb	$0x0, 0x1d0(%rax)
140ff19a1: 48 8b 4d 78                 	movq	0x78(%rbp), %rcx
140ff19a5: 48 89 88 d8 01 00 00        	movq	%rcx, 0x1d8(%rax)
140ff19ac: 48 8b 8d 88 00 00 00        	movq	0x88(%rbp), %rcx
140ff19b3: 48 89 88 e0 01 00 00        	movq	%rcx, 0x1e0(%rax)
140ff19ba: 48 83 c4 30                 	addq	$0x30, %rsp
140ff19be: 5f                          	popq	%rdi
140ff19bf: 5e                          	popq	%rsi
140ff19c0: 5d                          	popq	%rbp
140ff19c1: c3                          	retq
140ff19c2: cc                          	int3
140ff19c3: cc                          	int3
140ff19c4: cc                          	int3
140ff19c5: cc                          	int3
140ff19c6: cc                          	int3
140ff19c7: cc                          	int3
140ff19c8: cc                          	int3
140ff19c9: cc                          	int3
140ff19ca: cc                          	int3
140ff19cb: cc                          	int3
140ff19cc: cc                          	int3
140ff19cd: cc                          	int3
140ff19ce: cc                          	int3
140ff19cf: cc                          	int3
140ff19d0: 41 57                       	pushq	%r15
140ff19d2: 41 56                       	pushq	%r14
140ff19d4: 41 55                       	pushq	%r13
140ff19d6: 41 54                       	pushq	%r12
140ff19d8: 56                          	pushq	%rsi
140ff19d9: 57                          	pushq	%rdi
140ff19da: 55                          	pushq	%rbp
140ff19db: 53                          	pushq	%rbx
140ff19dc: 48 81 ec c8 02 00 00        	subq	$0x2c8, %rsp            # imm = 0x2C8
140ff19e3: bd 01 00 00 00              	movl	$0x1, %ebp
140ff19e8: f0                          	lock
140ff19e9: 0f c1 aa b0 05 00 00        	xaddl	%ebp, 0x5b0(%rdx)
140ff19f0: 31 c0                       	xorl	%eax, %eax
140ff19f2: 48 3b 81 38 01 00 00        	cmpq	0x138(%rcx), %rax
140ff19f9: 0f 80 71 0a 00 00           	jo	0x140ff2470 <.text+0xff1470>
140ff19ff: 49 89 cd                    	movq	%rcx, %r13
140ff1a02: 48 8b 89 78 01 00 00        	movq	0x178(%rcx), %rcx
140ff1a09: 48 8b 82 18 02 00 00        	movq	0x218(%rdx), %rax
140ff1a10: 48 39 c8                    	cmpq	%rcx, %rax
140ff1a13: 0f 86 7f 07 00 00           	jbe	0x140ff2198 <.text+0xff1198>
140ff1a19: 48 8b 9a 10 02 00 00        	movq	0x210(%rdx), %rbx
140ff1a20: 48 8b 0c cb                 	movq	(%rbx,%rcx,8), %rcx
140ff1a24: 48 85 c9                    	testq	%rcx, %rcx
140ff1a27: 0f 84 6b 07 00 00           	je	0x140ff2198 <.text+0xff1198>
140ff1a2d: 48 8b ba e0 01 00 00        	movq	0x1e0(%rdx), %rdi
140ff1a34: 48 f7 d1                    	notq	%rcx
140ff1a37: 48 8d 34 49                 	leaq	(%rcx,%rcx,2), %rsi
140ff1a3b: 48 c1 e6 05                 	shlq	$0x5, %rsi
140ff1a3f: 48 83 7c 37 20 00           	cmpq	$0x0, 0x20(%rdi,%rsi)
140ff1a45: 0f 84 4d 07 00 00           	je	0x140ff2198 <.text+0xff1198>
140ff1a4b: 49 8b 8d 80 01 00 00        	movq	0x180(%r13), %rcx
140ff1a52: 48 39 c8                    	cmpq	%rcx, %rax
140ff1a55: 0f 86 61 07 00 00           	jbe	0x140ff21bc <.text+0xff11bc>
140ff1a5b: 48 8b 0c cb                 	movq	(%rbx,%rcx,8), %rcx
140ff1a5f: 48 85 c9                    	testq	%rcx, %rcx
140ff1a62: 0f 84 54 07 00 00           	je	0x140ff21bc <.text+0xff11bc>
140ff1a68: 48 f7 d1                    	notq	%rcx
140ff1a6b: 4c 8d 34 49                 	leaq	(%rcx,%rcx,2), %r14
140ff1a6f: 49 c1 e6 05                 	shlq	$0x5, %r14
140ff1a73: 4a 83 7c 37 20 00           	cmpq	$0x0, 0x20(%rdi,%r14)
140ff1a79: 0f 84 3d 07 00 00           	je	0x140ff21bc <.text+0xff11bc>
140ff1a7f: 49 8b 8d 88 01 00 00        	movq	0x188(%r13), %rcx
140ff1a86: 48 39 c8                    	cmpq	%rcx, %rax
140ff1a89: 0f 86 4e 07 00 00           	jbe	0x140ff21dd <.text+0xff11dd>
140ff1a8f: 48 8b 0c cb                 	movq	(%rbx,%rcx,8), %rcx
140ff1a93: 48 85 c9                    	testq	%rcx, %rcx
140ff1a96: 0f 84 41 07 00 00           	je	0x140ff21dd <.text+0xff11dd>
140ff1a9c: 48 f7 d1                    	notq	%rcx
140ff1a9f: 4c 8d 04 49                 	leaq	(%rcx,%rcx,2), %r8
140ff1aa3: 49 c1 e0 05                 	shlq	$0x5, %r8
140ff1aa7: 4a 83 7c 07 20 00           	cmpq	$0x0, 0x20(%rdi,%r8)
140ff1aad: 0f 84 2a 07 00 00           	je	0x140ff21dd <.text+0xff11dd>
140ff1ab3: 49 8b 8d 90 01 00 00        	movq	0x190(%r13), %rcx
140ff1aba: 48 39 c8                    	cmpq	%rcx, %rax
140ff1abd: 0f 86 3b 07 00 00           	jbe	0x140ff21fe <.text+0xff11fe>
140ff1ac3: 48 8b 0c cb                 	movq	(%rbx,%rcx,8), %rcx
140ff1ac7: 48 85 c9                    	testq	%rcx, %rcx
140ff1aca: 0f 84 2e 07 00 00           	je	0x140ff21fe <.text+0xff11fe>
140ff1ad0: 48 f7 d1                    	notq	%rcx
140ff1ad3: 4c 8d 14 49                 	leaq	(%rcx,%rcx,2), %r10
140ff1ad7: 49 c1 e2 05                 	shlq	$0x5, %r10
140ff1adb: 4a 83 7c 17 20 00           	cmpq	$0x0, 0x20(%rdi,%r10)
140ff1ae1: 0f 84 17 07 00 00           	je	0x140ff21fe <.text+0xff11fe>
140ff1ae7: 49 8b 8d 98 01 00 00        	movq	0x198(%r13), %rcx
140ff1aee: 48 39 c8                    	cmpq	%rcx, %rax
140ff1af1: 0f 86 28 07 00 00           	jbe	0x140ff221f <.text+0xff121f>
140ff1af7: 48 8b 0c cb                 	movq	(%rbx,%rcx,8), %rcx
140ff1afb: 48 85 c9                    	testq	%rcx, %rcx
140ff1afe: 0f 84 1b 07 00 00           	je	0x140ff221f <.text+0xff121f>
140ff1b04: 48 f7 d1                    	notq	%rcx
140ff1b07: 4c 8d 1c 49                 	leaq	(%rcx,%rcx,2), %r11
140ff1b0b: 49 c1 e3 05                 	shlq	$0x5, %r11
140ff1b0f: 4a 83 7c 1f 20 00           	cmpq	$0x0, 0x20(%rdi,%r11)
140ff1b15: 0f 84 04 07 00 00           	je	0x140ff221f <.text+0xff121f>
140ff1b1b: 49 8b 8d a0 01 00 00        	movq	0x1a0(%r13), %rcx
140ff1b22: 48 39 c8                    	cmpq	%rcx, %rax
140ff1b25: 0f 86 9d 07 00 00           	jbe	0x140ff22c8 <.text+0xff12c8>
140ff1b2b: 48 8b 04 cb                 	movq	(%rbx,%rcx,8), %rax
140ff1b2f: 48 85 c0                    	testq	%rax, %rax
140ff1b32: 0f 84 90 07 00 00           	je	0x140ff22c8 <.text+0xff12c8>
140ff1b38: 48 f7 d0                    	notq	%rax
140ff1b3b: 4c 8d 0c 40                 	leaq	(%rax,%rax,2), %r9
140ff1b3f: 49 c1 e1 05                 	shlq	$0x5, %r9
140ff1b43: 4a 83 7c 0f 20 00           	cmpq	$0x0, 0x20(%rdi,%r9)
140ff1b49: 0f 84 79 07 00 00           	je	0x140ff22c8 <.text+0xff12c8>
140ff1b4f: 48 01 fe                    	addq	%rdi, %rsi
140ff1b52: 48 8b 46 10                 	movq	0x10(%rsi), %rax
140ff1b56: 48 89 84 24 a0 02 00 00     	movq	%rax, 0x2a0(%rsp)
140ff1b5e: 41 8b 85 30 01 00 00        	movl	0x130(%r13), %eax
140ff1b65: 49 01 fe                    	addq	%rdi, %r14
140ff1b68: 49 8b 4e 10                 	movq	0x10(%r14), %rcx
140ff1b6c: 48 89 8c 24 a8 02 00 00     	movq	%rcx, 0x2a8(%rsp)
140ff1b74: 49 01 f8                    	addq	%rdi, %r8
140ff1b77: 49 8b 48 10                 	movq	0x10(%r8), %rcx
140ff1b7b: 48 89 8c 24 b0 02 00 00     	movq	%rcx, 0x2b0(%rsp)
140ff1b83: 49 01 fa                    	addq	%rdi, %r10
140ff1b86: 49 8b 4a 10                 	movq	0x10(%r10), %rcx
140ff1b8a: 48 89 8c 24 b8 02 00 00     	movq	%rcx, 0x2b8(%rsp)
140ff1b92: 49 01 fb                    	addq	%rdi, %r11
140ff1b95: 49 8b 4b 10                 	movq	0x10(%r11), %rcx
140ff1b99: 48 89 8c 24 c0 02 00 00     	movq	%rcx, 0x2c0(%rsp)
140ff1ba1: 49 01 f9                    	addq	%rdi, %r9
140ff1ba4: 49 8b 59 10                 	movq	0x10(%r9), %rbx
140ff1ba8: 8b 8a b0 05 00 00           	movl	0x5b0(%rdx), %ecx
140ff1bae: 49 8b 8d a8 01 00 00        	movq	0x1a8(%r13), %rcx
140ff1bb5: 48 39 8a 18 02 00 00        	cmpq	%rcx, 0x218(%rdx)
140ff1bbc: 0f 86 27 07 00 00           	jbe	0x140ff22e9 <.text+0xff12e9>
140ff1bc2: 48 8b ba 10 02 00 00        	movq	0x210(%rdx), %rdi
140ff1bc9: 48 8b 0c cf                 	movq	(%rdi,%rcx,8), %rcx
140ff1bcd: 48 85 c9                    	testq	%rcx, %rcx
140ff1bd0: 0f 84 13 07 00 00           	je	0x140ff22e9 <.text+0xff12e9>
140ff1bd6: 4c 8b ba e0 01 00 00        	movq	0x1e0(%rdx), %r15
140ff1bdd: 48 f7 d1                    	notq	%rcx
140ff1be0: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140ff1be4: 48 c1 e1 05                 	shlq	$0x5, %rcx
140ff1be8: 49 83 7c 0f 20 00           	cmpq	$0x0, 0x20(%r15,%rcx)
140ff1bee: 0f 84 f5 06 00 00           	je	0x140ff22e9 <.text+0xff12e9>
140ff1bf4: 48 89 9c 24 90 02 00 00     	movq	%rbx, 0x290(%rsp)
140ff1bfc: 4c 89 b4 24 98 02 00 00     	movq	%r14, 0x298(%rsp)
140ff1c04: 49 01 cf                    	addq	%rcx, %r15
140ff1c07: 49 8b 4f 10                 	movq	0x10(%r15), %rcx
140ff1c0b: 48 89 8c 24 88 02 00 00     	movq	%rcx, 0x288(%rsp)
140ff1c13: 8b 8a b0 05 00 00           	movl	0x5b0(%rdx), %ecx
140ff1c19: 49 8b 8d b0 01 00 00        	movq	0x1b0(%r13), %rcx
140ff1c20: 48 39 8a 18 02 00 00        	cmpq	%rcx, 0x218(%rdx)
140ff1c27: 0f 86 dd 06 00 00           	jbe	0x140ff230a <.text+0xff130a>
140ff1c2d: 48 8b ba 10 02 00 00        	movq	0x210(%rdx), %rdi
140ff1c34: 48 8b 0c cf                 	movq	(%rdi,%rcx,8), %rcx
140ff1c38: 48 85 c9                    	testq	%rcx, %rcx
140ff1c3b: 0f 84 c9 06 00 00           	je	0x140ff230a <.text+0xff130a>
140ff1c41: 4c 8b a2 e0 01 00 00        	movq	0x1e0(%rdx), %r12
140ff1c48: 48 f7 d1                    	notq	%rcx
140ff1c4b: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140ff1c4f: 48 c1 e1 05                 	shlq	$0x5, %rcx
140ff1c53: 49 83 7c 0c 20 00           	cmpq	$0x0, 0x20(%r12,%rcx)
140ff1c59: 0f 84 ab 06 00 00           	je	0x140ff230a <.text+0xff130a>
140ff1c5f: 49 01 cc                    	addq	%rcx, %r12
140ff1c62: 49 8b 4c 24 10              	movq	0x10(%r12), %rcx
140ff1c67: 48 89 8c 24 80 02 00 00     	movq	%rcx, 0x280(%rsp)
140ff1c6f: 8b 8a b0 05 00 00           	movl	0x5b0(%rdx), %ecx
140ff1c75: 49 8b 8d b8 01 00 00        	movq	0x1b8(%r13), %rcx
140ff1c7c: 48 39 8a 18 02 00 00        	cmpq	%rcx, 0x218(%rdx)
140ff1c83: 0f 86 a2 06 00 00           	jbe	0x140ff232b <.text+0xff132b>
140ff1c89: 48 8b ba 10 02 00 00        	movq	0x210(%rdx), %rdi
140ff1c90: 48 8b 0c cf                 	movq	(%rdi,%rcx,8), %rcx
140ff1c94: 48 85 c9                    	testq	%rcx, %rcx
140ff1c97: 0f 84 8e 06 00 00           	je	0x140ff232b <.text+0xff132b>
140ff1c9d: 4c 8b b2 e0 01 00 00        	movq	0x1e0(%rdx), %r14
140ff1ca4: 48 f7 d1                    	notq	%rcx
140ff1ca7: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140ff1cab: 48 c1 e1 05                 	shlq	$0x5, %rcx
140ff1caf: 49 83 7c 0e 20 00           	cmpq	$0x0, 0x20(%r14,%rcx)
140ff1cb5: 0f 84 70 06 00 00           	je	0x140ff232b <.text+0xff132b>
140ff1cbb: 49 01 ce                    	addq	%rcx, %r14
140ff1cbe: 49 8b 4e 10                 	movq	0x10(%r14), %rcx
140ff1cc2: 48 89 8c 24 78 02 00 00     	movq	%rcx, 0x278(%rsp)
140ff1cca: 8b 8a b0 05 00 00           	movl	0x5b0(%rdx), %ecx
140ff1cd0: 4c 89 ac 24 a8 00 00 00     	movq	%r13, 0xa8(%rsp)
140ff1cd8: 49 8b 8d c0 01 00 00        	movq	0x1c0(%r13), %rcx
140ff1cdf: 48 39 8a 18 02 00 00        	cmpq	%rcx, 0x218(%rdx)
140ff1ce6: 0f 86 e8 06 00 00           	jbe	0x140ff23d4 <.text+0xff13d4>
140ff1cec: 48 8b ba 10 02 00 00        	movq	0x210(%rdx), %rdi
140ff1cf3: 48 8b 0c cf                 	movq	(%rdi,%rcx,8), %rcx
140ff1cf7: 48 85 c9                    	testq	%rcx, %rcx
140ff1cfa: 0f 84 d4 06 00 00           	je	0x140ff23d4 <.text+0xff13d4>
140ff1d00: 4c 8b aa e0 01 00 00        	movq	0x1e0(%rdx), %r13
140ff1d07: 48 f7 d1                    	notq	%rcx
140ff1d0a: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140ff1d0e: 48 c1 e1 05                 	shlq	$0x5, %rcx
140ff1d12: 49 83 7c 0d 20 00           	cmpq	$0x0, 0x20(%r13,%rcx)
140ff1d18: 0f 84 b6 06 00 00           	je	0x140ff23d4 <.text+0xff13d4>
140ff1d1e: 49 01 cd                    	addq	%rcx, %r13
140ff1d21: 49 8b 4d 10                 	movq	0x10(%r13), %rcx
140ff1d25: 48 89 8c 24 70 02 00 00     	movq	%rcx, 0x270(%rsp)
140ff1d2d: 8b 8a b0 05 00 00           	movl	0x5b0(%rdx), %ecx
140ff1d33: 48 8b 8c 24 a8 00 00 00     	movq	0xa8(%rsp), %rcx
140ff1d3b: 48 8b 89 c8 01 00 00        	movq	0x1c8(%rcx), %rcx
140ff1d42: 48 39 8a 18 02 00 00        	cmpq	%rcx, 0x218(%rdx)
140ff1d49: 0f 86 b9 06 00 00           	jbe	0x140ff2408 <.text+0xff1408>
140ff1d4f: 48 8b ba 10 02 00 00        	movq	0x210(%rdx), %rdi
140ff1d56: 48 8b 0c cf                 	movq	(%rdi,%rcx,8), %rcx
140ff1d5a: 48 85 c9                    	testq	%rcx, %rcx
140ff1d5d: 0f 84 a5 06 00 00           	je	0x140ff2408 <.text+0xff1408>
140ff1d63: 48 89 f3                    	movq	%rsi, %rbx
140ff1d66: 89 ee                       	movl	%ebp, %esi
140ff1d68: 48 8b aa e0 01 00 00        	movq	0x1e0(%rdx), %rbp
140ff1d6f: 48 f7 d1                    	notq	%rcx
140ff1d72: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140ff1d76: 48 c1 e1 05                 	shlq	$0x5, %rcx
140ff1d7a: 48 83 7c 0d 20 00           	cmpq	$0x0, 0x20(%rbp,%rcx)
140ff1d80: 0f 84 82 06 00 00           	je	0x140ff2408 <.text+0xff1408>
140ff1d86: 48 01 cd                    	addq	%rcx, %rbp
140ff1d89: 48 8b 4d 10                 	movq	0x10(%rbp), %rcx
140ff1d8d: 48 89 8c 24 68 02 00 00     	movq	%rcx, 0x268(%rsp)
140ff1d95: 8b 8a b0 05 00 00           	movl	0x5b0(%rdx), %ecx
140ff1d9b: 48 8b 8c 24 a8 00 00 00     	movq	0xa8(%rsp), %rcx
140ff1da3: 48 8b 89 d8 01 00 00        	movq	0x1d8(%rcx), %rcx
140ff1daa: 48 39 8a 18 02 00 00        	cmpq	%rcx, 0x218(%rdx)
140ff1db1: 0f 86 85 06 00 00           	jbe	0x140ff243c <.text+0xff143c>
140ff1db7: 48 8b ba 10 02 00 00        	movq	0x210(%rdx), %rdi
140ff1dbe: 48 8b 0c cf                 	movq	(%rdi,%rcx,8), %rcx
140ff1dc2: 48 85 c9                    	testq	%rcx, %rcx
140ff1dc5: 0f 84 71 06 00 00           	je	0x140ff243c <.text+0xff143c>
140ff1dcb: 48 8b ba e0 01 00 00        	movq	0x1e0(%rdx), %rdi
140ff1dd2: 48 f7 d1                    	notq	%rcx
140ff1dd5: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140ff1dd9: 48 c1 e1 05                 	shlq	$0x5, %rcx
140ff1ddd: 48 83 7c 0f 20 00           	cmpq	$0x0, 0x20(%rdi,%rcx)
140ff1de3: 0f 84 53 06 00 00           	je	0x140ff243c <.text+0xff143c>
140ff1de9: 48 01 cf                    	addq	%rcx, %rdi
140ff1dec: 48 8b 4f 10                 	movq	0x10(%rdi), %rcx
140ff1df0: 48 89 8c 24 60 02 00 00     	movq	%rcx, 0x260(%rsp)
140ff1df8: 8b 8a b0 05 00 00           	movl	0x5b0(%rdx), %ecx
140ff1dfe: 48 8b 8c 24 a0 02 00 00     	movq	0x2a0(%rsp), %rcx
140ff1e06: 48 89 8c 24 b0 00 00 00     	movq	%rcx, 0xb0(%rsp)
140ff1e0e: 48 8d 4b 5c                 	leaq	0x5c(%rbx), %rcx
140ff1e12: 48 83 c3 58                 	addq	$0x58, %rbx
140ff1e16: 48 89 9c 24 b8 00 00 00     	movq	%rbx, 0xb8(%rsp)
140ff1e1e: 48 89 8c 24 c0 00 00 00     	movq	%rcx, 0xc0(%rsp)
140ff1e26: 89 84 24 c8 00 00 00        	movl	%eax, 0xc8(%rsp)
140ff1e2d: 89 b4 24 cc 00 00 00        	movl	%esi, 0xcc(%rsp)
140ff1e34: 48 8b 8c 24 a8 02 00 00     	movq	0x2a8(%rsp), %rcx
140ff1e3c: 48 89 8c 24 d0 00 00 00     	movq	%rcx, 0xd0(%rsp)
140ff1e44: 48 8b 9c 24 98 02 00 00     	movq	0x298(%rsp), %rbx
140ff1e4c: 48 8d 4b 5c                 	leaq	0x5c(%rbx), %rcx
140ff1e50: 48 83 c3 58                 	addq	$0x58, %rbx
140ff1e54: 48 89 9c 24 d8 00 00 00     	movq	%rbx, 0xd8(%rsp)
140ff1e5c: 48 89 8c 24 e0 00 00 00     	movq	%rcx, 0xe0(%rsp)
140ff1e64: 89 84 24 e8 00 00 00        	movl	%eax, 0xe8(%rsp)
140ff1e6b: 89 b4 24 ec 00 00 00        	movl	%esi, 0xec(%rsp)
140ff1e72: 48 8b 8c 24 b0 02 00 00     	movq	0x2b0(%rsp), %rcx
140ff1e7a: 48 89 8c 24 f0 00 00 00     	movq	%rcx, 0xf0(%rsp)
140ff1e82: 49 8d 48 5c                 	leaq	0x5c(%r8), %rcx
140ff1e86: 49 83 c0 58                 	addq	$0x58, %r8
140ff1e8a: 4c 89 84 24 f8 00 00 00     	movq	%r8, 0xf8(%rsp)
140ff1e92: 48 89 8c 24 00 01 00 00     	movq	%rcx, 0x100(%rsp)
140ff1e9a: 89 84 24 08 01 00 00        	movl	%eax, 0x108(%rsp)
140ff1ea1: 89 b4 24 0c 01 00 00        	movl	%esi, 0x10c(%rsp)
140ff1ea8: 48 8b 8c 24 b8 02 00 00     	movq	0x2b8(%rsp), %rcx
140ff1eb0: 48 89 8c 24 10 01 00 00     	movq	%rcx, 0x110(%rsp)
140ff1eb8: 49 8d 4a 5c                 	leaq	0x5c(%r10), %rcx
140ff1ebc: 49 83 c2 58                 	addq	$0x58, %r10
140ff1ec0: 4c 89 94 24 18 01 00 00     	movq	%r10, 0x118(%rsp)
140ff1ec8: 48 89 8c 24 20 01 00 00     	movq	%rcx, 0x120(%rsp)
140ff1ed0: 89 84 24 28 01 00 00        	movl	%eax, 0x128(%rsp)
140ff1ed7: 89 b4 24 2c 01 00 00        	movl	%esi, 0x12c(%rsp)
140ff1ede: 48 8b 8c 24 c0 02 00 00     	movq	0x2c0(%rsp), %rcx
140ff1ee6: 48 89 8c 24 30 01 00 00     	movq	%rcx, 0x130(%rsp)
140ff1eee: 49 8d 4b 5c                 	leaq	0x5c(%r11), %rcx
140ff1ef2: 49 83 c3 58                 	addq	$0x58, %r11
140ff1ef6: 4c 89 9c 24 38 01 00 00     	movq	%r11, 0x138(%rsp)
140ff1efe: 48 89 8c 24 40 01 00 00     	movq	%rcx, 0x140(%rsp)
140ff1f06: 89 84 24 48 01 00 00        	movl	%eax, 0x148(%rsp)
140ff1f0d: 89 b4 24 4c 01 00 00        	movl	%esi, 0x14c(%rsp)
140ff1f14: 48 8b 9c 24 a8 00 00 00     	movq	0xa8(%rsp), %rbx
140ff1f1c: 48 8d 8b 70 01 00 00        	leaq	0x170(%rbx), %rcx
140ff1f23: 48 89 8c 24 50 01 00 00     	movq	%rcx, 0x150(%rsp)
140ff1f2b: 4c 8b 84 24 90 02 00 00     	movq	0x290(%rsp), %r8
140ff1f33: 4c 89 84 24 58 01 00 00     	movq	%r8, 0x158(%rsp)
140ff1f3b: 4d 8d 41 5c                 	leaq	0x5c(%r9), %r8
140ff1f3f: 49 83 c1 58                 	addq	$0x58, %r9
140ff1f43: 4c 89 8c 24 60 01 00 00     	movq	%r9, 0x160(%rsp)
140ff1f4b: 4c 89 84 24 68 01 00 00     	movq	%r8, 0x168(%rsp)
140ff1f53: 89 84 24 70 01 00 00        	movl	%eax, 0x170(%rsp)
140ff1f5a: 89 b4 24 74 01 00 00        	movl	%esi, 0x174(%rsp)
140ff1f61: 4c 8b 84 24 88 02 00 00     	movq	0x288(%rsp), %r8
140ff1f69: 4c 89 84 24 78 01 00 00     	movq	%r8, 0x178(%rsp)
140ff1f71: 4d 8d 47 5c                 	leaq	0x5c(%r15), %r8
140ff1f75: 49 83 c7 58                 	addq	$0x58, %r15
140ff1f79: 4c 89 bc 24 80 01 00 00     	movq	%r15, 0x180(%rsp)
140ff1f81: 4c 89 84 24 88 01 00 00     	movq	%r8, 0x188(%rsp)
140ff1f89: 4c 8d 83 38 01 00 00        	leaq	0x138(%rbx), %r8
140ff1f90: 89 84 24 90 01 00 00        	movl	%eax, 0x190(%rsp)
140ff1f97: 89 b4 24 94 01 00 00        	movl	%esi, 0x194(%rsp)
140ff1f9e: 4c 8b 8c 24 80 02 00 00     	movq	0x280(%rsp), %r9
140ff1fa6: 4c 89 8c 24 98 01 00 00     	movq	%r9, 0x198(%rsp)
140ff1fae: 4d 8d 4c 24 5c              	leaq	0x5c(%r12), %r9
140ff1fb3: 49 83 c4 58                 	addq	$0x58, %r12
140ff1fb7: 4c 89 a4 24 a0 01 00 00     	movq	%r12, 0x1a0(%rsp)
140ff1fbf: 4c 8d 5d 5c                 	leaq	0x5c(%rbp), %r11
140ff1fc3: 48 83 c5 58                 	addq	$0x58, %rbp
140ff1fc7: 4c 89 8c 24 a8 01 00 00     	movq	%r9, 0x1a8(%rsp)
140ff1fcf: 4d 8d 7d 5c                 	leaq	0x5c(%r13), %r15
140ff1fd3: 49 83 c5 58                 	addq	$0x58, %r13
140ff1fd7: 89 84 24 b0 01 00 00        	movl	%eax, 0x1b0(%rsp)
140ff1fde: 89 b4 24 b4 01 00 00        	movl	%esi, 0x1b4(%rsp)
140ff1fe5: 48 89 94 24 b8 01 00 00     	movq	%rdx, 0x1b8(%rsp)
140ff1fed: 4d 8d 4e 5c                 	leaq	0x5c(%r14), %r9
140ff1ff1: 49 83 c6 58                 	addq	$0x58, %r14
140ff1ff5: 48 c7 84 24 c0 01 00 00 00 00 00 00 	movq	$0x0, 0x1c0(%rsp)
140ff2001: 4c 89 84 24 c8 01 00 00     	movq	%r8, 0x1c8(%rsp)
140ff2009: 4c 8d 93 d0 01 00 00        	leaq	0x1d0(%rbx), %r10
140ff2010: 4c 89 94 24 d8 01 00 00     	movq	%r10, 0x1d8(%rsp)
140ff2018: 48 8b 94 24 78 02 00 00     	movq	0x278(%rsp), %rdx
140ff2020: 48 89 94 24 e0 01 00 00     	movq	%rdx, 0x1e0(%rsp)
140ff2028: 4c 8d 67 58                 	leaq	0x58(%rdi), %r12
140ff202c: 48 83 c7 5c                 	addq	$0x5c, %rdi
140ff2030: 4c 89 b4 24 e8 01 00 00     	movq	%r14, 0x1e8(%rsp)
140ff2038: 4c 89 8c 24 f0 01 00 00     	movq	%r9, 0x1f0(%rsp)
140ff2040: 4c 8d 84 24 f0 00 00 00     	leaq	0xf0(%rsp), %r8
140ff2048: 89 84 24 f8 01 00 00        	movl	%eax, 0x1f8(%rsp)
140ff204f: 89 b4 24 fc 01 00 00        	movl	%esi, 0x1fc(%rsp)
140ff2056: 48 8b 94 24 70 02 00 00     	movq	0x270(%rsp), %rdx
140ff205e: 48 89 94 24 00 02 00 00     	movq	%rdx, 0x200(%rsp)
140ff2066: 4c 8d 8c 24 10 01 00 00     	leaq	0x110(%rsp), %r9
140ff206e: 4c 89 ac 24 08 02 00 00     	movq	%r13, 0x208(%rsp)
140ff2076: 48 8d 94 24 30 01 00 00     	leaq	0x130(%rsp), %rdx
140ff207e: 4c 89 bc 24 10 02 00 00     	movq	%r15, 0x210(%rsp)
140ff2086: 4c 8d b4 24 58 01 00 00     	leaq	0x158(%rsp), %r14
140ff208e: 89 84 24 18 02 00 00        	movl	%eax, 0x218(%rsp)
140ff2095: 89 b4 24 1c 02 00 00        	movl	%esi, 0x21c(%rsp)
140ff209c: 4c 8b bc 24 68 02 00 00     	movq	0x268(%rsp), %r15
140ff20a4: 4c 89 bc 24 20 02 00 00     	movq	%r15, 0x220(%rsp)
140ff20ac: 4c 8d bc 24 78 01 00 00     	leaq	0x178(%rsp), %r15
140ff20b4: 48 89 ac 24 28 02 00 00     	movq	%rbp, 0x228(%rsp)
140ff20bc: 4c 8d ac 24 98 01 00 00     	leaq	0x198(%rsp), %r13
140ff20c4: 4c 89 9c 24 30 02 00 00     	movq	%r11, 0x230(%rsp)
140ff20cc: 4c 8d 9c 24 b8 01 00 00     	leaq	0x1b8(%rsp), %r11
140ff20d4: 89 84 24 38 02 00 00        	movl	%eax, 0x238(%rsp)
140ff20db: 89 b4 24 3c 02 00 00        	movl	%esi, 0x23c(%rsp)
