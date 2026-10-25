
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14062fcb0: 55                          	pushq	%rbp
14062fcb1: 56                          	pushq	%rsi
14062fcb2: 57                          	pushq	%rdi
14062fcb3: 48 81 ec 10 08 00 00        	subq	$0x810, %rsp            # imm = 0x810
14062fcba: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14062fcc2: 48 c7 85 88 07 00 00 fe ff ff ff    	movq	$-0x2, 0x788(%rbp)
14062fccd: 48 89 ce                    	movq	%rcx, %rsi
14062fcd0: 48 8d 79 38                 	leaq	0x38(%rcx), %rdi
14062fcd4: 48 ba 44 f3 5f e2 15 3d a4 44       	movabsq	$0x44a43d15e25ff344, %rdx # imm = 0x44A43D15E25FF344
14062fcde: 49 b8 a3 50 a9 c1 bf dc e6 77       	movabsq	$0x77e6dcbfc1a950a3, %r8 # imm = 0x77E6DCBFC1A950A3
14062fce8: 48 89 f9                    	movq	%rdi, %rcx
14062fceb: e8 00 da aa ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
14062fcf0: 48 85 c0                    	testq	%rax, %rax
14062fcf3: 74 33                       	je	0x14062fd28 <.text+0x62ed28>
14062fcf5: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
14062fcfc: 76 2a                       	jbe	0x14062fd28 <.text+0x62ed28>
14062fcfe: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
14062fd05: 48 8b 04 d0                 	movq	(%rax,%rdx,8), %rax
14062fd09: 48 85 c0                    	testq	%rax, %rax
14062fd0c: 74 1a                       	je	0x14062fd28 <.text+0x62ed28>
14062fd0e: 48 8b 8e e0 01 00 00        	movq	0x1e0(%rsi), %rcx
14062fd15: 48 f7 d0                    	notq	%rax
14062fd18: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14062fd1c: 48 c1 e0 05                 	shlq	$0x5, %rax
14062fd20: 48 83 7c 01 20 00           	cmpq	$0x0, 0x20(%rcx,%rax)
14062fd26: 75 08                       	jne	0x14062fd30 <.text+0x62ed30>
14062fd28: 48 89 f1                    	movq	%rsi, %rcx
14062fd2b: e8 c0 72 7a 00              	callq	0x140dd6ff0 <.text+0xdd5ff0>
14062fd30: 48 ba 7b e2 be 49 b9 b2 91 98       	movabsq	$-0x676e4d46b6411d85, %rdx # imm = 0x9891B2B949BEE27B
14062fd3a: 49 b8 7a 05 da b8 2a 5b 0a fa       	movabsq	$-0x5f5a4d54725fa86, %r8 # imm = 0xFA0A5B2AB8DA057A
14062fd44: 48 89 f9                    	movq	%rdi, %rcx
14062fd47: e8 a4 d9 aa ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
14062fd4c: 48 85 c0                    	testq	%rax, %rax
14062fd4f: 74 33                       	je	0x14062fd84 <.text+0x62ed84>
14062fd51: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
14062fd58: 76 2a                       	jbe	0x14062fd84 <.text+0x62ed84>
14062fd5a: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
14062fd61: 48 8b 04 d0                 	movq	(%rax,%rdx,8), %rax
14062fd65: 48 85 c0                    	testq	%rax, %rax
14062fd68: 74 1a                       	je	0x14062fd84 <.text+0x62ed84>
14062fd6a: 48 8b 8e e0 01 00 00        	movq	0x1e0(%rsi), %rcx
14062fd71: 48 f7 d0                    	notq	%rax
14062fd74: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14062fd78: 48 c1 e0 05                 	shlq	$0x5, %rax
14062fd7c: 48 83 7c 01 20 00           	cmpq	$0x0, 0x20(%rcx,%rax)
14062fd82: 75 08                       	jne	0x14062fd8c <.text+0x62ed8c>
14062fd84: 48 89 f1                    	movq	%rsi, %rcx
14062fd87: e8 a4 59 7a 00              	callq	0x140dd5730 <.text+0xdd4730>
14062fd8c: 48 ba a1 ba 99 4f 4b 45 79 55       	movabsq	$0x5579454b4f99baa1, %rdx # imm = 0x5579454B4F99BAA1
14062fd96: 49 b8 55 d9 35 a4 42 72 90 2c       	movabsq	$0x2c907242a435d955, %r8 # imm = 0x2C907242A435D955
14062fda0: 48 89 f9                    	movq	%rdi, %rcx
14062fda3: e8 48 d9 aa ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
14062fda8: 48 85 c0                    	testq	%rax, %rax
14062fdab: 74 33                       	je	0x14062fde0 <.text+0x62ede0>
14062fdad: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
14062fdb4: 76 2a                       	jbe	0x14062fde0 <.text+0x62ede0>
14062fdb6: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
14062fdbd: 48 8b 04 d0                 	movq	(%rax,%rdx,8), %rax
14062fdc1: 48 85 c0                    	testq	%rax, %rax
14062fdc4: 74 1a                       	je	0x14062fde0 <.text+0x62ede0>
14062fdc6: 48 8b 8e e0 01 00 00        	movq	0x1e0(%rsi), %rcx
14062fdcd: 48 f7 d0                    	notq	%rax
14062fdd0: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14062fdd4: 48 c1 e0 05                 	shlq	$0x5, %rax
14062fdd8: 48 83 7c 01 20 00           	cmpq	$0x0, 0x20(%rcx,%rax)
14062fdde: 75 08                       	jne	0x14062fde8 <.text+0x62ede8>
14062fde0: 48 89 f1                    	movq	%rsi, %rcx
14062fde3: e8 98 84 7a 00              	callq	0x140dd8280 <.text+0xdd7280>
14062fde8: 4c 8d 05 e9 c9 3d 02        	leaq	0x23dc9e9(%rip), %r8    # 0x142a0c7d8
14062fdef: 48 8d 8d d0 01 00 00        	leaq	0x1d0(%rbp), %rcx
14062fdf6: b2 11                       	movb	$0x11, %dl
14062fdf8: e8 93 c1 7a 00              	callq	0x140ddbf90 <.text+0xddaf90>
14062fdfd: 48 8d 8d 40 02 00 00        	leaq	0x240(%rbp), %rcx
14062fe04: e8 07 bc 7a 00              	callq	0x140ddba10 <.text+0xddaa10>
14062fe09: c6 85 10 07 00 00 12        	movb	$0x12, 0x710(%rbp)
14062fe10: 48 8b 0d 59 c5 ff 02        	movq	0x2ffc559(%rip), %rcx   # 0x14362c370
14062fe17: 4c 8d 05 22 4c 54 02        	leaq	0x2544c22(%rip), %r8    # 0x142b74a40
14062fe1e: 48 8d 95 10 07 00 00        	leaq	0x710(%rbp), %rdx
14062fe25: e8 66 cd 7a 00              	callq	0x140ddcb90 <.text+0xddbb90>
14062fe2a: 49 89 d0                    	movq	%rdx, %r8
14062fe2d: 48 8d 8d 40 02 00 00        	leaq	0x240(%rbp), %rcx
14062fe34: 48 89 c2                    	movq	%rax, %rdx
14062fe37: e8 44 e4 7a 00              	callq	0x140dde280 <.text+0xddd280>
14062fe3c: 0f 10 85 a0 02 00 00        	movups	0x2a0(%rbp), %xmm0
14062fe43: 0f 29 85 00 07 00 00        	movaps	%xmm0, 0x700(%rbp)
14062fe4a: 0f 10 85 90 02 00 00        	movups	0x290(%rbp), %xmm0
14062fe51: 0f 29 85 f0 06 00 00        	movaps	%xmm0, 0x6f0(%rbp)
14062fe58: 0f 10 85 80 02 00 00        	movups	0x280(%rbp), %xmm0
14062fe5f: 0f 29 85 e0 06 00 00        	movaps	%xmm0, 0x6e0(%rbp)
14062fe66: 0f 10 85 40 02 00 00        	movups	0x240(%rbp), %xmm0
14062fe6d: 0f 10 8d 50 02 00 00        	movups	0x250(%rbp), %xmm1
14062fe74: 0f 10 95 60 02 00 00        	movups	0x260(%rbp), %xmm2
14062fe7b: 0f 10 9d 70 02 00 00        	movups	0x270(%rbp), %xmm3
14062fe82: 0f 29 9d d0 06 00 00        	movaps	%xmm3, 0x6d0(%rbp)
14062fe89: 0f 29 95 c0 06 00 00        	movaps	%xmm2, 0x6c0(%rbp)
14062fe90: 0f 29 8d b0 06 00 00        	movaps	%xmm1, 0x6b0(%rbp)
14062fe97: 0f 29 85 a0 06 00 00        	movaps	%xmm0, 0x6a0(%rbp)
14062fe9e: 48 8d 8d 40 02 00 00        	leaq	0x240(%rbp), %rcx
14062fea5: e8 16 f0 7a 00              	callq	0x140ddeec0 <.text+0xdddec0>
14062feaa: 0f 10 85 a0 02 00 00        	movups	0x2a0(%rbp), %xmm0
14062feb1: 0f 29 85 70 07 00 00        	movaps	%xmm0, 0x770(%rbp)
14062feb8: 0f 10 85 90 02 00 00        	movups	0x290(%rbp), %xmm0
14062febf: 0f 29 85 60 07 00 00        	movaps	%xmm0, 0x760(%rbp)
14062fec6: 0f 10 85 80 02 00 00        	movups	0x280(%rbp), %xmm0
14062fecd: 0f 29 85 50 07 00 00        	movaps	%xmm0, 0x750(%rbp)
14062fed4: 0f 10 85 40 02 00 00        	movups	0x240(%rbp), %xmm0
14062fedb: 0f 10 8d 50 02 00 00        	movups	0x250(%rbp), %xmm1
14062fee2: 0f 10 95 60 02 00 00        	movups	0x260(%rbp), %xmm2
14062fee9: 0f 10 9d 70 02 00 00        	movups	0x270(%rbp), %xmm3
14062fef0: 0f 29 9d 40 07 00 00        	movaps	%xmm3, 0x740(%rbp)
14062fef7: 0f 29 95 30 07 00 00        	movaps	%xmm2, 0x730(%rbp)
14062fefe: 0f 29 8d 20 07 00 00        	movaps	%xmm1, 0x720(%rbp)
14062ff05: 0f 29 85 10 07 00 00        	movaps	%xmm0, 0x710(%rbp)
14062ff0c: 48 8d 8d 40 02 00 00        	leaq	0x240(%rbp), %rcx
14062ff13: 48 8d 95 10 07 00 00        	leaq	0x710(%rbp), %rdx
14062ff1a: e8 e1 df 7a 00              	callq	0x140dddf00 <.text+0xddcf00>
14062ff1f: 0f 10 85 40 02 00 00        	movups	0x240(%rbp), %xmm0
14062ff26: 0f 10 8d 50 02 00 00        	movups	0x250(%rbp), %xmm1
14062ff2d: 0f 10 95 60 02 00 00        	movups	0x260(%rbp), %xmm2
14062ff34: 0f 10 9d 70 02 00 00        	movups	0x270(%rbp), %xmm3
14062ff3b: 0f 29 85 10 07 00 00        	movaps	%xmm0, 0x710(%rbp)
14062ff42: 0f 29 8d 20 07 00 00        	movaps	%xmm1, 0x720(%rbp)
14062ff49: 0f 29 95 30 07 00 00        	movaps	%xmm2, 0x730(%rbp)
14062ff50: 0f 29 9d 40 07 00 00        	movaps	%xmm3, 0x740(%rbp)
14062ff57: 0f 10 85 80 02 00 00        	movups	0x280(%rbp), %xmm0
14062ff5e: 0f 29 85 50 07 00 00        	movaps	%xmm0, 0x750(%rbp)
14062ff65: 0f 10 85 90 02 00 00        	movups	0x290(%rbp), %xmm0
14062ff6c: 0f 29 85 60 07 00 00        	movaps	%xmm0, 0x760(%rbp)
14062ff73: 0f 10 85 a0 02 00 00        	movups	0x2a0(%rbp), %xmm0
14062ff7a: 0f 29 85 70 07 00 00        	movaps	%xmm0, 0x770(%rbp)
14062ff81: 48 8d 8d 40 02 00 00        	leaq	0x240(%rbp), %rcx
14062ff88: 48 8d 95 10 07 00 00        	leaq	0x710(%rbp), %rdx
14062ff8f: e8 ac da 7a 00              	callq	0x140ddda40 <.text+0xddca40>
14062ff94: c6 85 10 07 00 00 14        	movb	$0x14, 0x710(%rbp)
14062ff9b: 48 8b 0d ce c3 ff 02        	movq	0x2ffc3ce(%rip), %rcx   # 0x14362c370
14062ffa2: 4c 8d 05 97 4a 54 02        	leaq	0x2544a97(%rip), %r8    # 0x142b74a40
14062ffa9: 48 8d 95 10 07 00 00        	leaq	0x710(%rbp), %rdx
14062ffb0: e8 db cb 7a 00              	callq	0x140ddcb90 <.text+0xddbb90>
14062ffb5: 49 89 d0                    	movq	%rdx, %r8
14062ffb8: 48 8d 8d 40 02 00 00        	leaq	0x240(%rbp), %rcx
14062ffbf: 48 89 c2                    	movq	%rax, %rdx
14062ffc2: e8 b9 e2 7a 00              	callq	0x140dde280 <.text+0xddd280>
14062ffc7: 0f 10 85 a0 02 00 00        	movups	0x2a0(%rbp), %xmm0
14062ffce: 0f 29 85 90 06 00 00        	movaps	%xmm0, 0x690(%rbp)
14062ffd5: 0f 10 85 90 02 00 00        	movups	0x290(%rbp), %xmm0
14062ffdc: 0f 29 85 80 06 00 00        	movaps	%xmm0, 0x680(%rbp)
14062ffe3: 0f 10 85 80 02 00 00        	movups	0x280(%rbp), %xmm0
14062ffea: 0f 29 85 70 06 00 00        	movaps	%xmm0, 0x670(%rbp)
14062fff1: 0f 10 85 40 02 00 00        	movups	0x240(%rbp), %xmm0
14062fff8: 0f 10 8d 50 02 00 00        	movups	0x250(%rbp), %xmm1
14062ffff: 0f 10 95 60 02 00 00        	movups	0x260(%rbp), %xmm2
140630006: 0f 10 9d 70 02 00 00        	movups	0x270(%rbp), %xmm3
14063000d: 0f 29 9d 60 06 00 00        	movaps	%xmm3, 0x660(%rbp)
140630014: 0f 29 95 50 06 00 00        	movaps	%xmm2, 0x650(%rbp)
14063001b: 0f 29 8d 40 06 00 00        	movaps	%xmm1, 0x640(%rbp)
140630022: 0f 29 85 30 06 00 00        	movaps	%xmm0, 0x630(%rbp)
140630029: 4c 8d 05 c0 c7 3d 02        	leaq	0x23dc7c0(%rip), %r8    # 0x142a0c7f0
140630030: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
140630037: b2 1d                       	movb	$0x1d, %dl
140630039: e8 72 ff 7a 00              	callq	0x140ddffb0 <.text+0xddefb0>
14063003e: 4c 8d 05 c3 c7 3d 02        	leaq	0x23dc7c3(%rip), %r8    # 0x142a0c808
140630045: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
14063004c: b2 15                       	movb	$0x15, %dl
14063004e: e8 ad f3 7a 00              	callq	0x140ddf400 <.text+0xdde400>
140630053: 4c 8d 05 c6 c7 3d 02        	leaq	0x23dc7c6(%rip), %r8    # 0x142a0c820
14063005a: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
140630061: b2 28                       	movb	$0x28, %dl
140630063: e8 48 10 7b 00              	callq	0x140de10b0 <.text+0xde00b0>
140630068: 4c 8d 05 c9 c7 3d 02        	leaq	0x23dc7c9(%rip), %r8    # 0x142a0c838
14063006f: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140630073: b2 22                       	movb	$0x22, %dl
140630075: e8 36 19 7b 00              	callq	0x140de19b0 <.text+0xde09b0>
14063007a: 4c 8d 05 cf c7 3d 02        	leaq	0x23dc7cf(%rip), %r8    # 0x142a0c850
140630081: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140630085: b2 22                       	movb	$0x22, %dl
140630087: e8 d4 4c 7b 00              	callq	0x140de4d60 <.text+0xde3d60>
14063008c: 4c 8d 05 d5 c7 3d 02        	leaq	0x23dc7d5(%rip), %r8    # 0x142a0c868
140630093: 48 8d 8d 10 07 00 00        	leaq	0x710(%rbp), %rcx
14063009a: b2 2c                       	movb	$0x2c, %dl
14063009c: e8 2f 55 7b 00              	callq	0x140de55d0 <.text+0xde45d0>
1406300a1: 0f 10 85 30 02 00 00        	movups	0x230(%rbp), %xmm0
1406300a8: 0f 29 85 a0 02 00 00        	movaps	%xmm0, 0x2a0(%rbp)
1406300af: 0f 10 85 20 02 00 00        	movups	0x220(%rbp), %xmm0
1406300b6: 0f 29 85 90 02 00 00        	movaps	%xmm0, 0x290(%rbp)
1406300bd: 0f 10 85 10 02 00 00        	movups	0x210(%rbp), %xmm0
1406300c4: 0f 29 85 80 02 00 00        	movaps	%xmm0, 0x280(%rbp)
1406300cb: 0f 10 85 d0 01 00 00        	movups	0x1d0(%rbp), %xmm0
1406300d2: 0f 10 8d e0 01 00 00        	movups	0x1e0(%rbp), %xmm1
1406300d9: 0f 10 95 f0 01 00 00        	movups	0x1f0(%rbp), %xmm2
1406300e0: 0f 10 9d 00 02 00 00        	movups	0x200(%rbp), %xmm3
1406300e7: 0f 29 9d 70 02 00 00        	movaps	%xmm3, 0x270(%rbp)
1406300ee: 0f 29 95 60 02 00 00        	movaps	%xmm2, 0x260(%rbp)
1406300f5: 0f 29 8d 50 02 00 00        	movaps	%xmm1, 0x250(%rbp)
1406300fc: 0f 29 85 40 02 00 00        	movaps	%xmm0, 0x240(%rbp)
140630103: 0f 28 85 a0 06 00 00        	movaps	0x6a0(%rbp), %xmm0
14063010a: 0f 28 8d b0 06 00 00        	movaps	0x6b0(%rbp), %xmm1
140630111: 0f 28 95 c0 06 00 00        	movaps	0x6c0(%rbp), %xmm2
140630118: 0f 28 9d d0 06 00 00        	movaps	0x6d0(%rbp), %xmm3
14063011f: 0f 29 85 b0 02 00 00        	movaps	%xmm0, 0x2b0(%rbp)
140630126: 0f 29 8d c0 02 00 00        	movaps	%xmm1, 0x2c0(%rbp)
14063012d: 0f 29 95 d0 02 00 00        	movaps	%xmm2, 0x2d0(%rbp)
140630134: 0f 29 9d e0 02 00 00        	movaps	%xmm3, 0x2e0(%rbp)
14063013b: 0f 28 85 e0 06 00 00        	movaps	0x6e0(%rbp), %xmm0
140630142: 0f 29 85 f0 02 00 00        	movaps	%xmm0, 0x2f0(%rbp)
140630149: 0f 28 85 f0 06 00 00        	movaps	0x6f0(%rbp), %xmm0
140630150: 0f 29 85 00 03 00 00        	movaps	%xmm0, 0x300(%rbp)
140630157: 0f 28 85 00 07 00 00        	movaps	0x700(%rbp), %xmm0
14063015e: 0f 29 85 10 03 00 00        	movaps	%xmm0, 0x310(%rbp)
140630165: 0f 28 85 90 06 00 00        	movaps	0x690(%rbp), %xmm0
14063016c: 0f 29 85 80 03 00 00        	movaps	%xmm0, 0x380(%rbp)
140630173: 0f 28 85 80 06 00 00        	movaps	0x680(%rbp), %xmm0
14063017a: 0f 29 85 70 03 00 00        	movaps	%xmm0, 0x370(%rbp)
140630181: 0f 28 85 70 06 00 00        	movaps	0x670(%rbp), %xmm0
140630188: 0f 29 85 60 03 00 00        	movaps	%xmm0, 0x360(%rbp)
14063018f: 0f 28 85 30 06 00 00        	movaps	0x630(%rbp), %xmm0
140630196: 0f 28 8d 40 06 00 00        	movaps	0x640(%rbp), %xmm1
14063019d: 0f 28 95 50 06 00 00        	movaps	0x650(%rbp), %xmm2
1406301a4: 0f 28 9d 60 06 00 00        	movaps	0x660(%rbp), %xmm3
1406301ab: 0f 29 9d 50 03 00 00        	movaps	%xmm3, 0x350(%rbp)
1406301b2: 0f 29 95 40 03 00 00        	movaps	%xmm2, 0x340(%rbp)
1406301b9: 0f 29 8d 30 03 00 00        	movaps	%xmm1, 0x330(%rbp)
1406301c0: 0f 29 85 20 03 00 00        	movaps	%xmm0, 0x320(%rbp)
1406301c7: 0f 10 85 c0 01 00 00        	movups	0x1c0(%rbp), %xmm0
1406301ce: 0f 29 85 f0 03 00 00        	movaps	%xmm0, 0x3f0(%rbp)
1406301d5: 0f 10 85 b0 01 00 00        	movups	0x1b0(%rbp), %xmm0
1406301dc: 0f 29 85 e0 03 00 00        	movaps	%xmm0, 0x3e0(%rbp)
1406301e3: 0f 10 85 a0 01 00 00        	movups	0x1a0(%rbp), %xmm0
1406301ea: 0f 29 85 d0 03 00 00        	movaps	%xmm0, 0x3d0(%rbp)
1406301f1: 0f 10 85 60 01 00 00        	movups	0x160(%rbp), %xmm0
1406301f8: 0f 10 8d 70 01 00 00        	movups	0x170(%rbp), %xmm1
1406301ff: 0f 10 95 80 01 00 00        	movups	0x180(%rbp), %xmm2
140630206: 0f 10 9d 90 01 00 00        	movups	0x190(%rbp), %xmm3
14063020d: 0f 29 9d c0 03 00 00        	movaps	%xmm3, 0x3c0(%rbp)
140630214: 0f 29 95 b0 03 00 00        	movaps	%xmm2, 0x3b0(%rbp)
14063021b: 0f 29 8d a0 03 00 00        	movaps	%xmm1, 0x3a0(%rbp)
140630222: 0f 29 85 90 03 00 00        	movaps	%xmm0, 0x390(%rbp)
140630229: 0f 10 85 50 01 00 00        	movups	0x150(%rbp), %xmm0
140630230: 0f 29 85 60 04 00 00        	movaps	%xmm0, 0x460(%rbp)
140630237: 0f 10 85 40 01 00 00        	movups	0x140(%rbp), %xmm0
14063023e: 0f 29 85 50 04 00 00        	movaps	%xmm0, 0x450(%rbp)
140630245: 0f 10 85 30 01 00 00        	movups	0x130(%rbp), %xmm0
14063024c: 0f 29 85 40 04 00 00        	movaps	%xmm0, 0x440(%rbp)
140630253: 0f 10 85 f0 00 00 00        	movups	0xf0(%rbp), %xmm0
14063025a: 0f 10 8d 00 01 00 00        	movups	0x100(%rbp), %xmm1
140630261: 0f 10 95 10 01 00 00        	movups	0x110(%rbp), %xmm2
140630268: 0f 10 9d 20 01 00 00        	movups	0x120(%rbp), %xmm3
14063026f: 0f 29 9d 30 04 00 00        	movaps	%xmm3, 0x430(%rbp)
140630276: 0f 29 95 20 04 00 00        	movaps	%xmm2, 0x420(%rbp)
14063027d: 0f 29 8d 10 04 00 00        	movaps	%xmm1, 0x410(%rbp)
140630284: 0f 29 85 00 04 00 00        	movaps	%xmm0, 0x400(%rbp)
14063028b: 0f 10 85 e0 00 00 00        	movups	0xe0(%rbp), %xmm0
140630292: 0f 29 85 d0 04 00 00        	movaps	%xmm0, 0x4d0(%rbp)
140630299: 0f 10 85 d0 00 00 00        	movups	0xd0(%rbp), %xmm0
1406302a0: 0f 29 85 c0 04 00 00        	movaps	%xmm0, 0x4c0(%rbp)
1406302a7: 0f 10 85 c0 00 00 00        	movups	0xc0(%rbp), %xmm0
1406302ae: 0f 29 85 b0 04 00 00        	movaps	%xmm0, 0x4b0(%rbp)
1406302b5: 0f 10 85 80 00 00 00        	movups	0x80(%rbp), %xmm0
1406302bc: 0f 10 8d 90 00 00 00        	movups	0x90(%rbp), %xmm1
1406302c3: 0f 10 95 a0 00 00 00        	movups	0xa0(%rbp), %xmm2
1406302ca: 0f 10 9d b0 00 00 00        	movups	0xb0(%rbp), %xmm3
1406302d1: 0f 29 9d a0 04 00 00        	movaps	%xmm3, 0x4a0(%rbp)
1406302d8: 0f 29 95 90 04 00 00        	movaps	%xmm2, 0x490(%rbp)
1406302df: 0f 29 8d 80 04 00 00        	movaps	%xmm1, 0x480(%rbp)
1406302e6: 0f 29 85 70 04 00 00        	movaps	%xmm0, 0x470(%rbp)
1406302ed: 0f 10 45 70                 	movups	0x70(%rbp), %xmm0
1406302f1: 0f 29 85 40 05 00 00        	movaps	%xmm0, 0x540(%rbp)
1406302f8: 0f 10 45 60                 	movups	0x60(%rbp), %xmm0
1406302fc: 0f 29 85 30 05 00 00        	movaps	%xmm0, 0x530(%rbp)
140630303: 0f 10 45 50                 	movups	0x50(%rbp), %xmm0
140630307: 0f 29 85 20 05 00 00        	movaps	%xmm0, 0x520(%rbp)
14063030e: 0f 10 45 10                 	movups	0x10(%rbp), %xmm0
140630312: 0f 10 4d 20                 	movups	0x20(%rbp), %xmm1
140630316: 0f 10 55 30                 	movups	0x30(%rbp), %xmm2
14063031a: 0f 10 5d 40                 	movups	0x40(%rbp), %xmm3
14063031e: 0f 29 9d 10 05 00 00        	movaps	%xmm3, 0x510(%rbp)
140630325: 0f 29 95 00 05 00 00        	movaps	%xmm2, 0x500(%rbp)
14063032c: 0f 29 8d f0 04 00 00        	movaps	%xmm1, 0x4f0(%rbp)
140630333: 0f 29 85 e0 04 00 00        	movaps	%xmm0, 0x4e0(%rbp)
14063033a: 0f 10 45 00                 	movups	(%rbp), %xmm0
14063033e: 0f 29 85 b0 05 00 00        	movaps	%xmm0, 0x5b0(%rbp)
140630345: 0f 10 45 f0                 	movups	-0x10(%rbp), %xmm0
140630349: 0f 29 85 a0 05 00 00        	movaps	%xmm0, 0x5a0(%rbp)
140630350: 0f 10 45 e0                 	movups	-0x20(%rbp), %xmm0
140630354: 0f 29 85 90 05 00 00        	movaps	%xmm0, 0x590(%rbp)
14063035b: 0f 10 45 a0                 	movups	-0x60(%rbp), %xmm0
14063035f: 0f 10 4d b0                 	movups	-0x50(%rbp), %xmm1
140630363: 0f 10 55 c0                 	movups	-0x40(%rbp), %xmm2
140630367: 0f 10 5d d0                 	movups	-0x30(%rbp), %xmm3
14063036b: 0f 29 9d 80 05 00 00        	movaps	%xmm3, 0x580(%rbp)
140630372: 0f 29 95 70 05 00 00        	movaps	%xmm2, 0x570(%rbp)
140630379: 0f 29 8d 60 05 00 00        	movaps	%xmm1, 0x560(%rbp)
140630380: 0f 29 85 50 05 00 00        	movaps	%xmm0, 0x550(%rbp)
140630387: 0f 10 85 70 07 00 00        	movups	0x770(%rbp), %xmm0
14063038e: 0f 29 85 20 06 00 00        	movaps	%xmm0, 0x620(%rbp)
140630395: 0f 10 85 60 07 00 00        	movups	0x760(%rbp), %xmm0
14063039c: 0f 29 85 10 06 00 00        	movaps	%xmm0, 0x610(%rbp)
1406303a3: 0f 10 85 50 07 00 00        	movups	0x750(%rbp), %xmm0
1406303aa: 0f 29 85 00 06 00 00        	movaps	%xmm0, 0x600(%rbp)
1406303b1: 0f 10 85 10 07 00 00        	movups	0x710(%rbp), %xmm0
1406303b8: 0f 10 8d 20 07 00 00        	movups	0x720(%rbp), %xmm1
1406303bf: 0f 10 95 30 07 00 00        	movups	0x730(%rbp), %xmm2
1406303c6: 0f 10 9d 40 07 00 00        	movups	0x740(%rbp), %xmm3
1406303cd: 0f 29 9d f0 05 00 00        	movaps	%xmm3, 0x5f0(%rbp)
1406303d4: 0f 29 95 e0 05 00 00        	movaps	%xmm2, 0x5e0(%rbp)
1406303db: 0f 29 8d d0 05 00 00        	movaps	%xmm1, 0x5d0(%rbp)
1406303e2: 0f 29 85 c0 05 00 00        	movaps	%xmm0, 0x5c0(%rbp)
1406303e9: 48 8d 95 40 02 00 00        	leaq	0x240(%rbp), %rdx
1406303f0: 48 89 f1                    	movq	%rsi, %rcx
1406303f3: e8 68 38 7a 00              	callq	0x140dd3c60 <.text+0xdd2c60>
1406303f8: 48 89 f0                    	movq	%rsi, %rax
1406303fb: 48 81 c4 10 08 00 00        	addq	$0x810, %rsp            # imm = 0x810
140630402: 5f                          	popq	%rdi
140630403: 5e                          	popq	%rsi
140630404: 5d                          	popq	%rbp
140630405: c3                          	retq
140630406: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140630410: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140630415: 55                          	pushq	%rbp
140630416: 56                          	pushq	%rsi
140630417: 57                          	pushq	%rdi
140630418: 48 83 ec 20                 	subq	$0x20, %rsp
14063041c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140630423: 48 8d 8d d0 01 00 00        	leaq	0x1d0(%rbp), %rcx
14063042a: e8 51 94 9f ff              	callq	0x140029880 <.text+0x28880>
14063042f: 90                          	nop
140630430: 48 83 c4 20                 	addq	$0x20, %rsp
140630434: 5f                          	popq	%rdi
140630435: 5e                          	popq	%rsi
140630436: 5d                          	popq	%rbp
140630437: c3                          	retq
140630438: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
140630440: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140630445: 55                          	pushq	%rbp
140630446: 56                          	pushq	%rsi
140630447: 57                          	pushq	%rdi
140630448: 48 83 ec 20                 	subq	$0x20, %rsp
14063044c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140630453: 48 8d 8d 40 02 00 00        	leaq	0x240(%rbp), %rcx
14063045a: e8 21 94 9f ff              	callq	0x140029880 <.text+0x28880>
14063045f: 90                          	nop
140630460: 48 83 c4 20                 	addq	$0x20, %rsp
140630464: 5f                          	popq	%rdi
140630465: 5e                          	popq	%rsi
140630466: 5d                          	popq	%rbp
140630467: c3                          	retq
140630468: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
140630470: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140630475: 55                          	pushq	%rbp
140630476: 56                          	pushq	%rsi
140630477: 57                          	pushq	%rdi
140630478: 48 83 ec 20                 	subq	$0x20, %rsp
14063047c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140630483: 48 8d 8d a0 06 00 00        	leaq	0x6a0(%rbp), %rcx
14063048a: e8 f1 93 9f ff              	callq	0x140029880 <.text+0x28880>
14063048f: 90                          	nop
140630490: 48 83 c4 20                 	addq	$0x20, %rsp
140630494: 5f                          	popq	%rdi
140630495: 5e                          	popq	%rsi
140630496: 5d                          	popq	%rbp
140630497: c3                          	retq
140630498: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1406304a0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1406304a5: 55                          	pushq	%rbp
1406304a6: 56                          	pushq	%rsi
1406304a7: 57                          	pushq	%rdi
1406304a8: 48 83 ec 20                 	subq	$0x20, %rsp
1406304ac: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1406304b3: 48 8d 8d 40 02 00 00        	leaq	0x240(%rbp), %rcx
1406304ba: e8 c1 93 9f ff              	callq	0x140029880 <.text+0x28880>
1406304bf: 90                          	nop
1406304c0: 48 83 c4 20                 	addq	$0x20, %rsp
1406304c4: 5f                          	popq	%rdi
1406304c5: 5e                          	popq	%rsi
1406304c6: 5d                          	popq	%rbp
1406304c7: c3                          	retq
1406304c8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1406304d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1406304d5: 55                          	pushq	%rbp
1406304d6: 56                          	pushq	%rsi
1406304d7: 57                          	pushq	%rdi
1406304d8: 48 83 ec 20                 	subq	$0x20, %rsp
1406304dc: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1406304e3: 48 8d 8d 30 06 00 00        	leaq	0x630(%rbp), %rcx
1406304ea: e8 91 93 9f ff              	callq	0x140029880 <.text+0x28880>
1406304ef: 90                          	nop
1406304f0: 48 83 c4 20                 	addq	$0x20, %rsp
1406304f4: 5f                          	popq	%rdi
1406304f5: 5e                          	popq	%rsi
1406304f6: 5d                          	popq	%rbp
1406304f7: c3                          	retq
1406304f8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
140630500: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140630505: 55                          	pushq	%rbp
140630506: 56                          	pushq	%rsi
140630507: 57                          	pushq	%rdi
140630508: 48 83 ec 20                 	subq	$0x20, %rsp
14063050c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140630513: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
14063051a: e8 61 93 9f ff              	callq	0x140029880 <.text+0x28880>
14063051f: 90                          	nop
140630520: 48 83 c4 20                 	addq	$0x20, %rsp
140630524: 5f                          	popq	%rdi
140630525: 5e                          	popq	%rsi
140630526: 5d                          	popq	%rbp
140630527: c3                          	retq
140630528: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
140630530: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140630535: 55                          	pushq	%rbp
140630536: 56                          	pushq	%rsi
140630537: 57                          	pushq	%rdi
140630538: 48 83 ec 20                 	subq	$0x20, %rsp
14063053c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140630543: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
14063054a: e8 31 93 9f ff              	callq	0x140029880 <.text+0x28880>
14063054f: 90                          	nop
140630550: 48 83 c4 20                 	addq	$0x20, %rsp
140630554: 5f                          	popq	%rdi
140630555: 5e                          	popq	%rsi
140630556: 5d                          	popq	%rbp
140630557: c3                          	retq
140630558: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
140630560: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140630565: 55                          	pushq	%rbp
140630566: 56                          	pushq	%rsi
140630567: 57                          	pushq	%rdi
140630568: 48 83 ec 20                 	subq	$0x20, %rsp
14063056c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140630573: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
14063057a: e8 01 93 9f ff              	callq	0x140029880 <.text+0x28880>
14063057f: 90                          	nop
140630580: 48 83 c4 20                 	addq	$0x20, %rsp
140630584: 5f                          	popq	%rdi
140630585: 5e                          	popq	%rsi
140630586: 5d                          	popq	%rbp
140630587: c3                          	retq
140630588: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
140630590: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140630595: 55                          	pushq	%rbp
140630596: 56                          	pushq	%rsi
140630597: 57                          	pushq	%rdi
140630598: 48 83 ec 20                 	subq	$0x20, %rsp
14063059c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1406305a3: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
1406305a7: e8 d4 92 9f ff              	callq	0x140029880 <.text+0x28880>
1406305ac: 90                          	nop
1406305ad: 48 83 c4 20                 	addq	$0x20, %rsp
1406305b1: 5f                          	popq	%rdi
1406305b2: 5e                          	popq	%rsi
1406305b3: 5d                          	popq	%rbp
1406305b4: c3                          	retq
1406305b5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
1406305c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1406305c5: 55                          	pushq	%rbp
1406305c6: 56                          	pushq	%rsi
1406305c7: 57                          	pushq	%rdi
1406305c8: 48 83 ec 20                 	subq	$0x20, %rsp
1406305cc: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1406305d3: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
1406305d7: e8 a4 92 9f ff              	callq	0x140029880 <.text+0x28880>
1406305dc: 90                          	nop
1406305dd: 48 83 c4 20                 	addq	$0x20, %rsp
1406305e1: 5f                          	popq	%rdi
1406305e2: 5e                          	popq	%rsi
1406305e3: 5d                          	popq	%rbp
1406305e4: c3                          	retq
