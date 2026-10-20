
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14068dc70: 55                          	pushq	%rbp
14068dc71: 56                          	pushq	%rsi
14068dc72: 57                          	pushq	%rdi
14068dc73: 48 81 ec 20 01 00 00        	subq	$0x120, %rsp            # imm = 0x120
14068dc7a: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14068dc82: 48 c7 85 98 00 00 00 fe ff ff ff    	movq	$-0x2, 0x98(%rbp)
14068dc8d: 48 89 d6                    	movq	%rdx, %rsi
14068dc90: 48 8d 4a 38                 	leaq	0x38(%rdx), %rcx
14068dc94: 48 ba 84 a7 2b 96 51 d2 1b 8e       	movabsq	$-0x71e42dae69d4587c, %rdx # imm = 0x8E1BD251962BA784
14068dc9e: 49 b8 9b 52 e6 c0 74 23 78 6a       	movabsq	$0x6a782374c0e6529b, %r8 # imm = 0x6A782374C0E6529B
14068dca8: e8 43 fa a4 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
14068dcad: 48 85 c0                    	testq	%rax, %rax
14068dcb0: 0f 84 f1 00 00 00           	je	0x14068dda7 <.text+0x68cda7>
14068dcb6: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
14068dcbd: 0f 86 e4 00 00 00           	jbe	0x14068dda7 <.text+0x68cda7>
14068dcc3: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
14068dcca: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
14068dcce: 48 85 c9                    	testq	%rcx, %rcx
14068dcd1: 0f 84 d0 00 00 00           	je	0x14068dda7 <.text+0x68cda7>
14068dcd7: 48 8b 86 e0 01 00 00        	movq	0x1e0(%rsi), %rax
14068dcde: 48 f7 d1                    	notq	%rcx
14068dce1: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
14068dce5: 48 c1 e1 05                 	shlq	$0x5, %rcx
14068dce9: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
14068dcef: 0f 84 b2 00 00 00           	je	0x14068dda7 <.text+0x68cda7>
14068dcf5: 48 01 c8                    	addq	%rcx, %rax
14068dcf8: 48 8b 48 10                 	movq	0x10(%rax), %rcx
14068dcfc: 8b 96 b0 05 00 00           	movl	0x5b0(%rsi), %edx
14068dd02: 89 50 5c                    	movl	%edx, 0x5c(%rax)
14068dd05: b2 01                       	movb	$0x1, %dl
14068dd07: e8 64 6c 94 00              	callq	0x140fd4970 <.text+0xfd3970>
14068dd0c: 48 8d 4d 18                 	leaq	0x18(%rbp), %rcx
14068dd10: e8 cb 2e 96 00              	callq	0x140ff0be0 <.text+0xfefbe0>
14068dd15: c6 85 88 00 00 00 08        	movb	$0x8, 0x88(%rbp)
14068dd1c: 48 8b 0d 4d e6 f9 02        	movq	0x2f9e64d(%rip), %rcx   # 0x14362c370
14068dd23: 4c 8d 05 4e 3e 53 02        	leaq	0x2533e4e(%rip), %r8    # 0x142bc1b78
14068dd2a: 48 8d 95 88 00 00 00        	leaq	0x88(%rbp), %rdx
14068dd31: e8 6a e5 94 00              	callq	0x140fdc2a0 <.text+0xfdb2a0>
14068dd36: 49 89 d0                    	movq	%rdx, %r8
14068dd39: 48 8d 4d 18                 	leaq	0x18(%rbp), %rcx
14068dd3d: 48 89 c2                    	movq	%rax, %rdx
14068dd40: e8 3b 02 95 00              	callq	0x140fddf80 <.text+0xfdcf80>
14068dd45: 0f 10 45 78                 	movups	0x78(%rbp), %xmm0
14068dd49: 0f 29 45 00                 	movaps	%xmm0, (%rbp)
14068dd4d: 0f 10 45 68                 	movups	0x68(%rbp), %xmm0
14068dd51: 0f 29 45 f0                 	movaps	%xmm0, -0x10(%rbp)
14068dd55: 0f 10 45 58                 	movups	0x58(%rbp), %xmm0
14068dd59: 0f 29 45 e0                 	movaps	%xmm0, -0x20(%rbp)
14068dd5d: 0f 10 45 18                 	movups	0x18(%rbp), %xmm0
14068dd61: 0f 10 4d 28                 	movups	0x28(%rbp), %xmm1
14068dd65: 0f 10 55 38                 	movups	0x38(%rbp), %xmm2
14068dd69: 0f 10 5d 48                 	movups	0x48(%rbp), %xmm3
14068dd6d: 0f 29 5d d0                 	movaps	%xmm3, -0x30(%rbp)
14068dd71: 0f 29 55 c0                 	movaps	%xmm2, -0x40(%rbp)
14068dd75: 0f 29 4d b0                 	movaps	%xmm1, -0x50(%rbp)
14068dd79: 0f 29 45 a0                 	movaps	%xmm0, -0x60(%rbp)
14068dd7d: 48 8d 7d 18                 	leaq	0x18(%rbp), %rdi
14068dd81: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
14068dd85: 48 89 f9                    	movq	%rdi, %rcx
14068dd88: 41 b0 01                    	movb	$0x1, %r8b
14068dd8b: e8 10 f4 94 00              	callq	0x140fdd1a0 <.text+0xfdc1a0>
14068dd90: 48 89 f1                    	movq	%rsi, %rcx
14068dd93: 48 89 fa                    	movq	%rdi, %rdx
14068dd96: e8 35 50 94 00              	callq	0x140fd2dd0 <.text+0xfd1dd0>
14068dd9b: 90                          	nop
14068dd9c: 48 81 c4 20 01 00 00        	addq	$0x120, %rsp            # imm = 0x120
14068dda3: 5f                          	popq	%rdi
14068dda4: 5e                          	popq	%rsi
14068dda5: 5d                          	popq	%rbp
14068dda6: c3                          	retq
14068dda7: 48 8d 05 a2 76 52 02        	leaq	0x25276a2(%rip), %rax   # 0x142bb5450
14068ddae: 48 89 45 a0                 	movq	%rax, -0x60(%rbp)
14068ddb2: 48 c7 45 a8 27 00 00 00     	movq	$0x27, -0x58(%rbp)
14068ddba: 48 8d 45 a0                 	leaq	-0x60(%rbp), %rax
14068ddbe: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
14068ddc5: 48 8d 05 c4 c0 a7 ff        	leaq	-0x583f3c(%rip), %rax   # 0x140109e90 <.text+0x108e90>
14068ddcc: 48 89 85 90 00 00 00        	movq	%rax, 0x90(%rbp)
14068ddd3: 48 8d 05 a6 77 52 02        	leaq	0x25277a6(%rip), %rax   # 0x142bb5580
14068ddda: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
14068ddde: 48 c7 45 20 02 00 00 00     	movq	$0x2, 0x20(%rbp)
14068dde6: 48 c7 45 38 00 00 00 00     	movq	$0x0, 0x38(%rbp)
14068ddee: 48 8d 85 88 00 00 00        	leaq	0x88(%rbp), %rax
14068ddf5: 48 89 45 28                 	movq	%rax, 0x28(%rbp)
14068ddf9: 48 c7 45 30 01 00 00 00     	movq	$0x1, 0x30(%rbp)
14068de01: 48 8d 15 30 76 52 02        	leaq	0x2527630(%rip), %rdx   # 0x142bb5438
14068de08: 48 8d 4d 18                 	leaq	0x18(%rbp), %rcx
14068de0c: e8 7f b5 24 02              	callq	0x1428d9390 <.text+0x28d8390>
14068de11: cc                          	int3
14068de12: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
14068de20: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14068de25: 55                          	pushq	%rbp
14068de26: 56                          	pushq	%rsi
14068de27: 57                          	pushq	%rdi
14068de28: 48 83 ec 20                 	subq	$0x20, %rsp
14068de2c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14068de33: 48 8d 4d 18                 	leaq	0x18(%rbp), %rcx
14068de37: e8 44 ba 99 ff              	callq	0x140029880 <.text+0x28880>
14068de3c: 90                          	nop
14068de3d: 48 83 c4 20                 	addq	$0x20, %rsp
14068de41: 5f                          	popq	%rdi
14068de42: 5e                          	popq	%rsi
14068de43: 5d                          	popq	%rbp
14068de44: c3                          	retq
