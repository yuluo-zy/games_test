
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14068d610: 55                          	pushq	%rbp
14068d611: 56                          	pushq	%rsi
14068d612: 57                          	pushq	%rdi
14068d613: 48 81 ec f0 01 00 00        	subq	$0x1f0, %rsp            # imm = 0x1F0
14068d61a: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14068d622: 48 c7 85 68 01 00 00 fe ff ff ff    	movq	$-0x2, 0x168(%rbp)
14068d62d: 48 89 d6                    	movq	%rdx, %rsi
14068d630: 48 8d 7d 10                 	leaq	0x10(%rbp), %rdi
14068d634: 48 89 f9                    	movq	%rdi, %rcx
14068d637: e8 d4 9d 95 00              	callq	0x140fe7410 <.text+0xfe6410>
14068d63c: 48 c7 85 a8 00 00 00 00 00 00 00    	movq	$0x0, 0xa8(%rbp)
14068d647: 48 c7 85 b0 00 00 00 08 00 00 00    	movq	$0x8, 0xb0(%rbp)
14068d652: 48 c7 85 b8 00 00 00 00 00 00 00    	movq	$0x0, 0xb8(%rbp)
14068d65d: 48 89 f1                    	movq	%rsi, %rcx
14068d660: 48 89 fa                    	movq	%rdi, %rdx
14068d663: e8 58 2c 94 00              	callq	0x140fd02c0 <.text+0xfcf2c0>
14068d668: 0f 57 c9                    	xorps	%xmm1, %xmm1
14068d66b: 48 89 c1                    	movq	%rax, %rcx
14068d66e: e8 3d 4f 94 00              	callq	0x140fd25b0 <.text+0xfd15b0>
14068d673: 48 8d 7e 38                 	leaq	0x38(%rsi), %rdi
14068d677: 48 ba 57 cb aa e3 f9 48 e7 b0       	movabsq	$-0x4f18b7061c5534a9, %rdx # imm = 0xB0E748F9E3AACB57
14068d681: 49 b8 6e e9 99 82 fa 17 71 75       	movabsq	$0x757117fa8299e96e, %r8 # imm = 0x757117FA8299E96E
14068d68b: 48 89 f9                    	movq	%rdi, %rcx
14068d68e: e8 5d 00 a5 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
14068d693: 48 85 c0                    	testq	%rax, %rax
14068d696: 74 33                       	je	0x14068d6cb <.text+0x68c6cb>
14068d698: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
14068d69f: 76 2a                       	jbe	0x14068d6cb <.text+0x68c6cb>
14068d6a1: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
14068d6a8: 48 8b 04 d0                 	movq	(%rax,%rdx,8), %rax
14068d6ac: 48 85 c0                    	testq	%rax, %rax
14068d6af: 74 1a                       	je	0x14068d6cb <.text+0x68c6cb>
14068d6b1: 48 8b 8e e0 01 00 00        	movq	0x1e0(%rsi), %rcx
14068d6b8: 48 f7 d0                    	notq	%rax
14068d6bb: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14068d6bf: 48 c1 e0 05                 	shlq	$0x5, %rax
14068d6c3: 48 83 7c 01 20 00           	cmpq	$0x0, 0x20(%rcx,%rax)
14068d6c9: 75 08                       	jne	0x14068d6d3 <.text+0x68c6d3>
14068d6cb: 48 89 f1                    	movq	%rsi, %rcx
14068d6ce: e8 9d cc 95 00              	callq	0x140fea370 <.text+0xfe9370>
14068d6d3: 48 ba d1 27 90 5f e5 42 26 4d       	movabsq	$0x4d2642e55f9027d1, %rdx # imm = 0x4D2642E55F9027D1
14068d6dd: 49 b8 fc 80 48 0d 30 7e 29 4d       	movabsq	$0x4d297e300d4880fc, %r8 # imm = 0x4D297E300D4880FC
14068d6e7: 48 89 f9                    	movq	%rdi, %rcx
14068d6ea: e8 01 00 a5 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
14068d6ef: 48 85 c0                    	testq	%rax, %rax
14068d6f2: 74 33                       	je	0x14068d727 <.text+0x68c727>
14068d6f4: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
14068d6fb: 76 2a                       	jbe	0x14068d727 <.text+0x68c727>
14068d6fd: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
14068d704: 48 8b 04 d0                 	movq	(%rax,%rdx,8), %rax
14068d708: 48 85 c0                    	testq	%rax, %rax
14068d70b: 74 1a                       	je	0x14068d727 <.text+0x68c727>
14068d70d: 48 8b 8e e0 01 00 00        	movq	0x1e0(%rsi), %rcx
14068d714: 48 f7 d0                    	notq	%rax
14068d717: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14068d71b: 48 c1 e0 05                 	shlq	$0x5, %rax
14068d71f: 48 83 7c 01 20 00           	cmpq	$0x0, 0x20(%rcx,%rax)
14068d725: 75 08                       	jne	0x14068d72f <.text+0x68c72f>
14068d727: 48 89 f1                    	movq	%rsi, %rcx
14068d72a: e8 d1 de 95 00              	callq	0x140feb600 <.text+0xfea600>
14068d72f: 48 ba fb 98 df be b5 38 9e 18       	movabsq	$0x189e38b5bedf98fb, %rdx # imm = 0x189E38B5BEDF98FB
14068d739: 49 b8 93 29 5c 82 12 69 3f d3       	movabsq	$-0x2cc096ed7da3d66d, %r8 # imm = 0xD33F6912825C2993
14068d743: 48 89 f9                    	movq	%rdi, %rcx
14068d746: e8 a5 ff a4 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
14068d74b: 48 85 c0                    	testq	%rax, %rax
14068d74e: 74 33                       	je	0x14068d783 <.text+0x68c783>
14068d750: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
14068d757: 76 2a                       	jbe	0x14068d783 <.text+0x68c783>
14068d759: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
14068d760: 48 8b 04 d0                 	movq	(%rax,%rdx,8), %rax
14068d764: 48 85 c0                    	testq	%rax, %rax
14068d767: 74 1a                       	je	0x14068d783 <.text+0x68c783>
14068d769: 48 8b 8e e0 01 00 00        	movq	0x1e0(%rsi), %rcx
14068d770: 48 f7 d0                    	notq	%rax
14068d773: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14068d777: 48 c1 e0 05                 	shlq	$0x5, %rax
14068d77b: 48 83 7c 01 20 00           	cmpq	$0x0, 0x20(%rcx,%rax)
14068d781: 75 08                       	jne	0x14068d78b <.text+0x68c78b>
14068d783: 48 89 f1                    	movq	%rsi, %rcx
14068d786: e8 05 ad 95 00              	callq	0x140fe8490 <.text+0xfe7490>
14068d78b: 48 ba 01 84 ad 92 78 5a e8 1c       	movabsq	$0x1ce85a7892ad8401, %rdx # imm = 0x1CE85A7892AD8401
14068d795: 49 b8 a2 73 6a 46 01 4e 6a d5       	movabsq	$-0x2a95b1feb9958c5e, %r8 # imm = 0xD56A4E01466A73A2
14068d79f: 48 89 f9                    	movq	%rdi, %rcx
14068d7a2: e8 49 ff a4 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
14068d7a7: 48 85 c0                    	testq	%rax, %rax
14068d7aa: 74 33                       	je	0x14068d7df <.text+0x68c7df>
14068d7ac: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
14068d7b3: 76 2a                       	jbe	0x14068d7df <.text+0x68c7df>
14068d7b5: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
14068d7bc: 48 8b 04 d0                 	movq	(%rax,%rdx,8), %rax
14068d7c0: 48 85 c0                    	testq	%rax, %rax
14068d7c3: 74 1a                       	je	0x14068d7df <.text+0x68c7df>
14068d7c5: 48 8b 8e e0 01 00 00        	movq	0x1e0(%rsi), %rcx
14068d7cc: 48 f7 d0                    	notq	%rax
14068d7cf: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14068d7d3: 48 c1 e0 05                 	shlq	$0x5, %rax
14068d7d7: 48 83 7c 01 20 00           	cmpq	$0x0, 0x20(%rcx,%rax)
14068d7dd: 75 08                       	jne	0x14068d7e7 <.text+0x68c7e7>
14068d7df: 48 89 f1                    	movq	%rsi, %rcx
14068d7e2: e8 b9 d1 95 00              	callq	0x140fea9a0 <.text+0xfe99a0>
14068d7e7: 48 ba 98 c5 ef d4 ca b6 2b 06       	movabsq	$0x62bb6cad4efc598, %rdx # imm = 0x62BB6CAD4EFC598
14068d7f1: 49 b8 6e 85 b1 16 3c d4 ac 22       	movabsq	$0x22acd43c16b1856e, %r8 # imm = 0x22ACD43C16B1856E
14068d7fb: 48 89 f9                    	movq	%rdi, %rcx
14068d7fe: e8 ed fe a4 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
14068d803: 48 85 c0                    	testq	%rax, %rax
14068d806: 74 33                       	je	0x14068d83b <.text+0x68c83b>
14068d808: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
14068d80f: 76 2a                       	jbe	0x14068d83b <.text+0x68c83b>
14068d811: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
14068d818: 48 8b 04 d0                 	movq	(%rax,%rdx,8), %rax
14068d81c: 48 85 c0                    	testq	%rax, %rax
14068d81f: 74 1a                       	je	0x14068d83b <.text+0x68c83b>
14068d821: 48 8b 8e e0 01 00 00        	movq	0x1e0(%rsi), %rcx
14068d828: 48 f7 d0                    	notq	%rax
14068d82b: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14068d82f: 48 c1 e0 05                 	shlq	$0x5, %rax
14068d833: 48 83 7c 01 20 00           	cmpq	$0x0, 0x20(%rcx,%rax)
14068d839: 75 08                       	jne	0x14068d843 <.text+0x68c843>
14068d83b: 48 89 f1                    	movq	%rsi, %rcx
14068d83e: e8 7d b2 95 00              	callq	0x140fe8ac0 <.text+0xfe7ac0>
14068d843: 48 ba bf 58 a6 51 d1 0e 15 0d       	movabsq	$0xd150ed151a658bf, %rdx # imm = 0xD150ED151A658BF
14068d84d: 49 b8 02 50 ea a5 4a 61 dd 01       	movabsq	$0x1dd614aa5ea5002, %r8 # imm = 0x1DD614AA5EA5002
14068d857: 48 89 f9                    	movq	%rdi, %rcx
14068d85a: e8 91 fe a4 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
14068d85f: 48 85 c0                    	testq	%rax, %rax
14068d862: 74 33                       	je	0x14068d897 <.text+0x68c897>
14068d864: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
14068d86b: 76 2a                       	jbe	0x14068d897 <.text+0x68c897>
14068d86d: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
14068d874: 48 8b 04 d0                 	movq	(%rax,%rdx,8), %rax
14068d878: 48 85 c0                    	testq	%rax, %rax
14068d87b: 74 1a                       	je	0x14068d897 <.text+0x68c897>
14068d87d: 48 8b 8e e0 01 00 00        	movq	0x1e0(%rsi), %rcx
14068d884: 48 f7 d0                    	notq	%rax
14068d887: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14068d88b: 48 c1 e0 05                 	shlq	$0x5, %rax
14068d88f: 48 83 7c 01 20 00           	cmpq	$0x0, 0x20(%rcx,%rax)
14068d895: 75 08                       	jne	0x14068d89f <.text+0x68c89f>
14068d897: 48 89 f1                    	movq	%rsi, %rcx
14068d89a: e8 51 b8 95 00              	callq	0x140fe90f0 <.text+0xfe80f0>
14068d89f: 48 ba 74 01 8e 86 67 ca aa 5e       	movabsq	$0x5eaaca67868e0174, %rdx # imm = 0x5EAACA67868E0174
14068d8a9: 49 b8 76 32 76 a5 60 5b 67 67       	movabsq	$0x67675b60a5763276, %r8 # imm = 0x67675B60A5763276
14068d8b3: 48 89 f9                    	movq	%rdi, %rcx
14068d8b6: e8 35 fe a4 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
14068d8bb: 48 85 c0                    	testq	%rax, %rax
14068d8be: 74 33                       	je	0x14068d8f3 <.text+0x68c8f3>
14068d8c0: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
14068d8c7: 76 2a                       	jbe	0x14068d8f3 <.text+0x68c8f3>
14068d8c9: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
14068d8d0: 48 8b 04 d0                 	movq	(%rax,%rdx,8), %rax
14068d8d4: 48 85 c0                    	testq	%rax, %rax
14068d8d7: 74 1a                       	je	0x14068d8f3 <.text+0x68c8f3>
14068d8d9: 48 8b 8e e0 01 00 00        	movq	0x1e0(%rsi), %rcx
14068d8e0: 48 f7 d0                    	notq	%rax
14068d8e3: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14068d8e7: 48 c1 e0 05                 	shlq	$0x5, %rax
14068d8eb: 48 83 7c 01 20 00           	cmpq	$0x0, 0x20(%rcx,%rax)
14068d8f1: 75 08                       	jne	0x14068d8fb <.text+0x68c8fb>
14068d8f3: 48 89 f1                    	movq	%rsi, %rcx
14068d8f6: e8 d5 d6 95 00              	callq	0x140feafd0 <.text+0xfe9fd0>
14068d8fb: 48 ba 19 82 77 26 05 9f 23 7e       	movabsq	$0x7e239f0526778219, %rdx # imm = 0x7E239F0526778219
14068d905: 49 b8 22 70 43 6f c4 14 d2 52       	movabsq	$0x52d214c46f437022, %r8 # imm = 0x52D214C46F437022
14068d90f: 48 89 f9                    	movq	%rdi, %rcx
14068d912: e8 d9 fd a4 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
14068d917: 48 85 c0                    	testq	%rax, %rax
14068d91a: 74 33                       	je	0x14068d94f <.text+0x68c94f>
14068d91c: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
14068d923: 76 2a                       	jbe	0x14068d94f <.text+0x68c94f>
14068d925: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
14068d92c: 48 8b 04 d0                 	movq	(%rax,%rdx,8), %rax
14068d930: 48 85 c0                    	testq	%rax, %rax
14068d933: 74 1a                       	je	0x14068d94f <.text+0x68c94f>
14068d935: 48 8b 8e e0 01 00 00        	movq	0x1e0(%rsi), %rcx
14068d93c: 48 f7 d0                    	notq	%rax
14068d93f: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14068d943: 48 c1 e0 05                 	shlq	$0x5, %rax
14068d947: 48 83 7c 01 20 00           	cmpq	$0x0, 0x20(%rcx,%rax)
14068d94d: 75 08                       	jne	0x14068d957 <.text+0x68c957>
14068d94f: 48 89 f1                    	movq	%rsi, %rcx
14068d952: e8 b9 bd 95 00              	callq	0x140fe9710 <.text+0xfe8710>
14068d957: 48 ba dc 8a b7 e7 a4 03 f3 73       	movabsq	$0x73f303a4e7b78adc, %rdx # imm = 0x73F303A4E7B78ADC
14068d961: 49 b8 6e 13 ea b4 2b 37 4f ed       	movabsq	$-0x12b0c8d44b15ec92, %r8 # imm = 0xED4F372BB4EA136E
14068d96b: 48 89 f9                    	movq	%rdi, %rcx
14068d96e: e8 7d fd a4 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
14068d973: 48 85 c0                    	testq	%rax, %rax
14068d976: 74 33                       	je	0x14068d9ab <.text+0x68c9ab>
14068d978: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
14068d97f: 76 2a                       	jbe	0x14068d9ab <.text+0x68c9ab>
14068d981: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
14068d988: 48 8b 04 d0                 	movq	(%rax,%rdx,8), %rax
14068d98c: 48 85 c0                    	testq	%rax, %rax
14068d98f: 74 1a                       	je	0x14068d9ab <.text+0x68c9ab>
14068d991: 48 8b 8e e0 01 00 00        	movq	0x1e0(%rsi), %rcx
14068d998: 48 f7 d0                    	notq	%rax
14068d99b: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14068d99f: 48 c1 e0 05                 	shlq	$0x5, %rax
14068d9a3: 48 83 7c 01 20 00           	cmpq	$0x0, 0x20(%rcx,%rax)
14068d9a9: 75 08                       	jne	0x14068d9b3 <.text+0x68c9b3>
14068d9ab: 48 89 f1                    	movq	%rsi, %rcx
14068d9ae: e8 ad a4 95 00              	callq	0x140fe7e60 <.text+0xfe6e60>
14068d9b3: 48 89 f1                    	movq	%rsi, %rcx
14068d9b6: e8 45 51 94 00              	callq	0x140fd2b00 <.text+0xfd1b00>
14068d9bb: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
14068d9bf: e8 1c 07 96 00              	callq	0x140fee0e0 <.text+0xfed0e0>
14068d9c4: c6 85 f0 00 00 00 07        	movb	$0x7, 0xf0(%rbp)
14068d9cb: 48 8b 0d 9e e9 f9 02        	movq	0x2f9e99e(%rip), %rcx   # 0x14362c370
14068d9d2: 4c 8d 05 9f 41 53 02        	leaq	0x253419f(%rip), %r8    # 0x142bc1b78
14068d9d9: 48 8d 95 f0 00 00 00        	leaq	0xf0(%rbp), %rdx
14068d9e0: e8 bb e8 94 00              	callq	0x140fdc2a0 <.text+0xfdb2a0>
14068d9e5: 49 89 d0                    	movq	%rdx, %r8
14068d9e8: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
14068d9ec: 48 89 c2                    	movq	%rax, %rdx
14068d9ef: e8 8c 05 95 00              	callq	0x140fddf80 <.text+0xfdcf80>
14068d9f4: 0f 10 45 70                 	movups	0x70(%rbp), %xmm0
14068d9f8: 0f 29 45 00                 	movaps	%xmm0, (%rbp)
14068d9fc: 0f 10 45 60                 	movups	0x60(%rbp), %xmm0
14068da00: 0f 29 45 f0                 	movaps	%xmm0, -0x10(%rbp)
14068da04: 0f 10 45 50                 	movups	0x50(%rbp), %xmm0
14068da08: 0f 29 45 e0                 	movaps	%xmm0, -0x20(%rbp)
14068da0c: 0f 10 45 10                 	movups	0x10(%rbp), %xmm0
14068da10: 0f 10 4d 20                 	movups	0x20(%rbp), %xmm1
14068da14: 0f 10 55 30                 	movups	0x30(%rbp), %xmm2
14068da18: 0f 10 5d 40                 	movups	0x40(%rbp), %xmm3
14068da1c: 0f 29 5d d0                 	movaps	%xmm3, -0x30(%rbp)
14068da20: 0f 29 55 c0                 	movaps	%xmm2, -0x40(%rbp)
14068da24: 0f 29 4d b0                 	movaps	%xmm1, -0x50(%rbp)
14068da28: 0f 29 45 a0                 	movaps	%xmm0, -0x60(%rbp)
14068da2c: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
14068da33: e8 88 1d 96 00              	callq	0x140fef7c0 <.text+0xfee7c0>
14068da38: c6 85 67 01 00 00 24        	movb	$0x24, 0x167(%rbp)
14068da3f: 48 8b 0d 2a e9 f9 02        	movq	0x2f9e92a(%rip), %rcx   # 0x14362c370
14068da46: 4c 8d 05 2b 41 53 02        	leaq	0x253412b(%rip), %r8    # 0x142bc1b78
14068da4d: 48 8d 95 67 01 00 00        	leaq	0x167(%rbp), %rdx
14068da54: e8 47 e8 94 00              	callq	0x140fdc2a0 <.text+0xfdb2a0>
14068da59: 49 89 d0                    	movq	%rdx, %r8
14068da5c: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
14068da63: 48 89 c2                    	movq	%rax, %rdx
14068da66: e8 15 05 95 00              	callq	0x140fddf80 <.text+0xfdcf80>
14068da6b: 0f 10 85 50 01 00 00        	movups	0x150(%rbp), %xmm0
14068da72: 0f 29 85 e0 00 00 00        	movaps	%xmm0, 0xe0(%rbp)
14068da79: 0f 10 85 40 01 00 00        	movups	0x140(%rbp), %xmm0
14068da80: 0f 29 85 d0 00 00 00        	movaps	%xmm0, 0xd0(%rbp)
14068da87: 0f 10 85 30 01 00 00        	movups	0x130(%rbp), %xmm0
14068da8e: 0f 29 85 c0 00 00 00        	movaps	%xmm0, 0xc0(%rbp)
14068da95: 0f 10 85 f0 00 00 00        	movups	0xf0(%rbp), %xmm0
14068da9c: 0f 10 8d 00 01 00 00        	movups	0x100(%rbp), %xmm1
14068daa3: 0f 10 95 10 01 00 00        	movups	0x110(%rbp), %xmm2
14068daaa: 0f 10 9d 20 01 00 00        	movups	0x120(%rbp), %xmm3
14068dab1: 0f 29 9d b0 00 00 00        	movaps	%xmm3, 0xb0(%rbp)
14068dab8: 0f 29 95 a0 00 00 00        	movaps	%xmm2, 0xa0(%rbp)
14068dabf: 0f 29 8d 90 00 00 00        	movaps	%xmm1, 0x90(%rbp)
14068dac6: 0f 29 85 80 00 00 00        	movaps	%xmm0, 0x80(%rbp)
14068dacd: 0f 28 45 a0                 	movaps	-0x60(%rbp), %xmm0
14068dad1: 0f 28 4d b0                 	movaps	-0x50(%rbp), %xmm1
14068dad5: 0f 28 55 c0                 	movaps	-0x40(%rbp), %xmm2
14068dad9: 0f 28 5d d0                 	movaps	-0x30(%rbp), %xmm3
14068dadd: 0f 29 45 10                 	movaps	%xmm0, 0x10(%rbp)
14068dae1: 0f 29 4d 20                 	movaps	%xmm1, 0x20(%rbp)
14068dae5: 0f 29 55 30                 	movaps	%xmm2, 0x30(%rbp)
14068dae9: 0f 29 5d 40                 	movaps	%xmm3, 0x40(%rbp)
14068daed: 0f 28 45 e0                 	movaps	-0x20(%rbp), %xmm0
14068daf1: 0f 29 45 50                 	movaps	%xmm0, 0x50(%rbp)
14068daf5: 0f 28 45 f0                 	movaps	-0x10(%rbp), %xmm0
14068daf9: 0f 29 45 60                 	movaps	%xmm0, 0x60(%rbp)
14068dafd: 0f 28 45 00                 	movaps	(%rbp), %xmm0
14068db01: 0f 29 45 70                 	movaps	%xmm0, 0x70(%rbp)
14068db05: 48 8d 55 10                 	leaq	0x10(%rbp), %rdx
14068db09: 48 89 f1                    	movq	%rsi, %rcx
14068db0c: e8 2f 51 94 00              	callq	0x140fd2c40 <.text+0xfd1c40>
14068db11: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
14068db15: e8 86 fd 95 00              	callq	0x140fed8a0 <.text+0xfec8a0>
14068db1a: c6 45 a0 2c                 	movb	$0x2c, -0x60(%rbp)
14068db1e: 48 8b 0d 4b e8 f9 02        	movq	0x2f9e84b(%rip), %rcx   # 0x14362c370
14068db25: 4c 8d 05 4c 40 53 02        	leaq	0x253404c(%rip), %r8    # 0x142bc1b78
14068db2c: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
14068db30: e8 6b e7 94 00              	callq	0x140fdc2a0 <.text+0xfdb2a0>
14068db35: 49 89 d0                    	movq	%rdx, %r8
14068db38: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
14068db3c: 48 89 c2                    	movq	%rax, %rdx
14068db3f: e8 3c 04 95 00              	callq	0x140fddf80 <.text+0xfdcf80>
14068db44: 0f 10 45 70                 	movups	0x70(%rbp), %xmm0
14068db48: 0f 29 85 50 01 00 00        	movaps	%xmm0, 0x150(%rbp)
14068db4f: 0f 10 45 60                 	movups	0x60(%rbp), %xmm0
14068db53: 0f 29 85 40 01 00 00        	movaps	%xmm0, 0x140(%rbp)
14068db5a: 0f 10 45 50                 	movups	0x50(%rbp), %xmm0
14068db5e: 0f 29 85 30 01 00 00        	movaps	%xmm0, 0x130(%rbp)
14068db65: 0f 10 45 10                 	movups	0x10(%rbp), %xmm0
14068db69: 0f 10 4d 20                 	movups	0x20(%rbp), %xmm1
14068db6d: 0f 10 55 30                 	movups	0x30(%rbp), %xmm2
14068db71: 0f 10 5d 40                 	movups	0x40(%rbp), %xmm3
14068db75: 0f 29 9d 20 01 00 00        	movaps	%xmm3, 0x120(%rbp)
14068db7c: 0f 29 95 10 01 00 00        	movaps	%xmm2, 0x110(%rbp)
14068db83: 0f 29 8d 00 01 00 00        	movaps	%xmm1, 0x100(%rbp)
14068db8a: 0f 29 85 f0 00 00 00        	movaps	%xmm0, 0xf0(%rbp)
14068db91: 48 8d 95 f0 00 00 00        	leaq	0xf0(%rbp), %rdx
14068db98: 48 89 f1                    	movq	%rsi, %rcx
14068db9b: e8 30 52 94 00              	callq	0x140fd2dd0 <.text+0xfd1dd0>
14068dba0: 90                          	nop
14068dba1: 48 81 c4 f0 01 00 00        	addq	$0x1f0, %rsp            # imm = 0x1F0
14068dba8: 5f                          	popq	%rdi
14068dba9: 5e                          	popq	%rsi
14068dbaa: 5d                          	popq	%rbp
14068dbab: c3                          	retq
14068dbac: 0f 1f 40 00                 	nopl	(%rax)
14068dbb0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14068dbb5: 55                          	pushq	%rbp
14068dbb6: 56                          	pushq	%rsi
14068dbb7: 57                          	pushq	%rdi
14068dbb8: 48 83 ec 20                 	subq	$0x20, %rsp
14068dbbc: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14068dbc3: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
14068dbc7: e8 b4 bc 99 ff              	callq	0x140029880 <.text+0x28880>
14068dbcc: 90                          	nop
14068dbcd: 48 83 c4 20                 	addq	$0x20, %rsp
14068dbd1: 5f                          	popq	%rdi
14068dbd2: 5e                          	popq	%rsi
14068dbd3: 5d                          	popq	%rbp
14068dbd4: c3                          	retq
14068dbd5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
14068dbe0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14068dbe5: 55                          	pushq	%rbp
14068dbe6: 56                          	pushq	%rsi
14068dbe7: 57                          	pushq	%rdi
14068dbe8: 48 83 ec 20                 	subq	$0x20, %rsp
14068dbec: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14068dbf3: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
14068dbf7: e8 84 bc 99 ff              	callq	0x140029880 <.text+0x28880>
14068dbfc: 90                          	nop
14068dbfd: 48 83 c4 20                 	addq	$0x20, %rsp
14068dc01: 5f                          	popq	%rdi
14068dc02: 5e                          	popq	%rsi
14068dc03: 5d                          	popq	%rbp
14068dc04: c3                          	retq
14068dc05: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
14068dc10: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14068dc15: 55                          	pushq	%rbp
14068dc16: 56                          	pushq	%rsi
14068dc17: 57                          	pushq	%rdi
14068dc18: 48 83 ec 20                 	subq	$0x20, %rsp
14068dc1c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14068dc23: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
14068dc2a: e8 51 bc 99 ff              	callq	0x140029880 <.text+0x28880>
14068dc2f: 90                          	nop
14068dc30: 48 83 c4 20                 	addq	$0x20, %rsp
14068dc34: 5f                          	popq	%rdi
14068dc35: 5e                          	popq	%rsi
14068dc36: 5d                          	popq	%rbp
14068dc37: c3                          	retq
14068dc38: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
14068dc40: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14068dc45: 55                          	pushq	%rbp
14068dc46: 56                          	pushq	%rsi
14068dc47: 57                          	pushq	%rdi
14068dc48: 48 83 ec 20                 	subq	$0x20, %rsp
14068dc4c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14068dc53: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
14068dc57: e8 24 bc 99 ff              	callq	0x140029880 <.text+0x28880>
14068dc5c: 90                          	nop
14068dc5d: 48 83 c4 20                 	addq	$0x20, %rsp
14068dc61: 5f                          	popq	%rdi
14068dc62: 5e                          	popq	%rsi
14068dc63: 5d                          	popq	%rbp
14068dc64: c3                          	retq
