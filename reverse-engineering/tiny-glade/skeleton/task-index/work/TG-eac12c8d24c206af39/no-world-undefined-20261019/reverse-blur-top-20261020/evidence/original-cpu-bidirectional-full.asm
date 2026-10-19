
/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1409ed860: 55                          	pushq	%rbp
1409ed861: 41 57                       	pushq	%r15
1409ed863: 41 56                       	pushq	%r14
1409ed865: 41 55                       	pushq	%r13
1409ed867: 41 54                       	pushq	%r12
1409ed869: 56                          	pushq	%rsi
1409ed86a: 57                          	pushq	%rdi
1409ed86b: 53                          	pushq	%rbx
1409ed86c: 48 81 ec d8 08 00 00        	subq	$0x8d8, %rsp            # imm = 0x8D8
1409ed873: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1409ed87b: 48 c7 85 50 08 00 00 fe ff ff ff    	movq	$-0x2, 0x850(%rbp)
1409ed886: 48 89 d7                    	movq	%rdx, %rdi
1409ed889: 48 89 8d c8 07 00 00        	movq	%rcx, 0x7c8(%rbp)
1409ed890: 48 8d 8d 94 07 00 00        	leaq	0x794(%rbp), %rcx
1409ed897: 4c 89 85 b8 07 00 00        	movq	%r8, 0x7b8(%rbp)
1409ed89e: e8 3d 09 00 00              	callq	0x1409ee1e0 <.text+0x9ed1e0>
1409ed8a3: 0f 10 85 98 07 00 00        	movups	0x798(%rbp), %xmm0
1409ed8aa: 0f 10 8d a8 07 00 00        	movups	0x7a8(%rbp), %xmm1
1409ed8b1: 0f 29 8d 10 06 00 00        	movaps	%xmm1, 0x610(%rbp)
1409ed8b8: 0f 29 85 00 06 00 00        	movaps	%xmm0, 0x600(%rbp)
1409ed8bf: 48 8d b5 70 04 00 00        	leaq	0x470(%rbp), %rsi
1409ed8c6: 48 8d 95 00 06 00 00        	leaq	0x600(%rbp), %rdx
1409ed8cd: 48 89 f1                    	movq	%rsi, %rcx
1409ed8d0: e8 9b 7a 1e 00              	callq	0x140bd5370 <.text+0xbd4370>
1409ed8d5: 48 8d 8d e0 02 00 00        	leaq	0x2e0(%rbp), %rcx
1409ed8dc: 48 89 f2                    	movq	%rsi, %rdx
1409ed8df: e8 ac 7a 1e 00              	callq	0x140bd5390 <.text+0xbd4390>
1409ed8e4: 0f 10 85 e0 02 00 00        	movups	0x2e0(%rbp), %xmm0
1409ed8eb: 0f 10 8d f0 02 00 00        	movups	0x2f0(%rbp), %xmm1
1409ed8f2: 0f 29 8d 80 04 00 00        	movaps	%xmm1, 0x480(%rbp)
1409ed8f9: 0f 29 85 70 04 00 00        	movaps	%xmm0, 0x470(%rbp)
1409ed900: 48 8d 8d 10 06 00 00        	leaq	0x610(%rbp), %rcx
1409ed907: 4c 8d bd 70 04 00 00        	leaq	0x470(%rbp), %r15
1409ed90e: 4c 89 fa                    	movq	%r15, %rdx
1409ed911: e8 ca 93 1e 00              	callq	0x140bd6ce0 <.text+0xbd5ce0>
1409ed916: 48 8d 05 13 7a 0d 02        	leaq	0x20d7a13(%rip), %rax   # 0x142ac5330
1409ed91d: 48 89 85 00 06 00 00        	movq	%rax, 0x600(%rbp)
1409ed924: 48 c7 85 08 06 00 00 11 00 00 00    	movq	$0x11, 0x608(%rbp)
1409ed92f: 48 8d 95 00 06 00 00        	leaq	0x600(%rbp), %rdx
1409ed936: 48 89 bd c0 07 00 00        	movq	%rdi, 0x7c0(%rbp)
1409ed93d: 48 89 f9                    	movq	%rdi, %rcx
1409ed940: e8 8b bb 1f 00              	callq	0x140be94d0 <.text+0xbe84d0>
1409ed945: 0f 10 85 e0 02 00 00        	movups	0x2e0(%rbp), %xmm0
1409ed94c: 0f 10 8d f0 02 00 00        	movups	0x2f0(%rbp), %xmm1
1409ed953: 0f 11 8d 20 08 00 00        	movups	%xmm1, 0x820(%rbp)
1409ed95a: 0f 11 85 10 08 00 00        	movups	%xmm0, 0x810(%rbp)
1409ed961: 89 85 0c 08 00 00           	movl	%eax, 0x80c(%rbp)
1409ed967: 0f b6 85 2c 08 00 00        	movzbl	0x82c(%rbp), %eax
1409ed96e: 48 89 85 30 08 00 00        	movq	%rax, 0x830(%rbp)
1409ed975: 85 c0                       	testl	%eax, %eax
1409ed977: 0f 84 5c 05 00 00           	je	0x1409eded9 <.text+0x9eced9>
1409ed97d: 48 8b b5 30 08 00 00        	movq	0x830(%rbp), %rsi
1409ed984: 8d 46 ff                    	leal	-0x1(%rsi), %eax
1409ed987: 89 85 3c 08 00 00           	movl	%eax, 0x83c(%rbp)
1409ed98d: 48 8d bd d0 07 00 00        	leaq	0x7d0(%rbp), %rdi
1409ed994: eb 14                       	jmp	0x1409ed9aa <.text+0x9ec9aa>
1409ed996: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1409ed9a0: 89 de                       	movl	%ebx, %esi
1409ed9a2: 85 db                       	testl	%ebx, %ebx
1409ed9a4: 0f 84 2f 05 00 00           	je	0x1409eded9 <.text+0x9eced9>
1409ed9aa: 8d 5e ff                    	leal	-0x1(%rsi), %ebx
1409ed9ad: b8 01 00 00 00              	movl	$0x1, %eax
1409ed9b2: 89 d9                       	movl	%ebx, %ecx
1409ed9b4: d3 e0                       	shll	%cl, %eax
1409ed9b6: 48 8d 8d 10 08 00 00        	leaq	0x810(%rbp), %rcx
1409ed9bd: 0f 10 01                    	movups	(%rcx), %xmm0
1409ed9c0: 0f 10 49 10                 	movups	0x10(%rcx), %xmm1
1409ed9c4: 0f 29 8d 10 06 00 00        	movaps	%xmm1, 0x610(%rbp)
1409ed9cb: 0f 29 85 00 06 00 00        	movaps	%xmm0, 0x600(%rbp)
1409ed9d2: 89 85 e0 02 00 00           	movl	%eax, 0x2e0(%rbp)
1409ed9d8: 89 85 e4 02 00 00           	movl	%eax, 0x2e4(%rbp)
1409ed9de: c7 85 e8 02 00 00 01 00 00 00       	movl	$0x1, 0x2e8(%rbp)
1409ed9e8: 4c 89 f9                    	movq	%r15, %rcx
1409ed9eb: 4c 8d b5 00 06 00 00        	leaq	0x600(%rbp), %r14
1409ed9f2: 4c 89 f2                    	movq	%r14, %rdx
1409ed9f5: 4c 8d 85 e0 02 00 00        	leaq	0x2e0(%rbp), %r8
1409ed9fc: e8 9f 7c 1e 00              	callq	0x140bd56a0 <.text+0xbd46a0>
1409eda01: 4c 8b ad 7c 04 00 00        	movq	0x47c(%rbp), %r13
1409eda08: 8b 85 84 04 00 00           	movl	0x484(%rbp), %eax
1409eda0e: 89 85 38 08 00 00           	movl	%eax, 0x838(%rbp)
1409eda14: 48 8b 85 c0 07 00 00        	movq	0x7c0(%rbp), %rax
1409eda1b: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
1409eda20: 48 8b 85 b8 07 00 00        	movq	0x7b8(%rbp), %rax
1409eda27: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
1409eda2c: 48 c7 44 24 20 1e 00 00 00  	movq	$0x1e, 0x20(%rsp)
1409eda35: 41 b8 09 00 00 00           	movl	$0x9, %r8d
1409eda3b: 4c 89 f1                    	movq	%r14, %rcx
1409eda3e: 48 8d 15 fc 78 0d 02        	leaq	0x20d78fc(%rip), %rdx   # 0x142ac5341
1409eda45: 4c 8d 0d fe 78 0d 02        	leaq	0x20d78fe(%rip), %r9    # 0x142ac534a
1409eda4c: e8 4f 58 c1 00              	callq	0x1416032a0 <.text+0x16022a0>
1409eda51: c6 85 4f 08 00 00 01        	movb	$0x1, 0x84f(%rbp)
1409eda58: 4c 89 f9                    	movq	%r15, %rcx
1409eda5b: e8 40 7d 1e 00              	callq	0x140bd57a0 <.text+0xbd47a0>
1409eda60: c6 85 4f 08 00 00 01        	movb	$0x1, 0x84f(%rbp)
1409eda67: 48 89 f9                    	movq	%rdi, %rcx
1409eda6a: 4c 89 fa                    	movq	%r15, %rdx
1409eda6d: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1409eda73: e8 a8 7d 1e 00              	callq	0x140bd5820 <.text+0xbd4820>
1409eda78: c6 85 4f 08 00 00 01        	movb	$0x1, 0x84f(%rbp)
1409eda7f: 4c 89 f9                    	movq	%r15, %rcx
1409eda82: 48 89 fa                    	movq	%rdi, %rdx
1409eda85: 41 89 d8                    	movl	%ebx, %r8d
1409eda88: e8 53 7e 1e 00              	callq	0x140bd58e0 <.text+0xbd48e0>
1409eda8d: c6 85 4f 08 00 00 01        	movb	$0x1, 0x84f(%rbp)
1409eda94: 48 89 f9                    	movq	%rdi, %rcx
1409eda97: 4c 89 fa                    	movq	%r15, %rdx
1409eda9a: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1409edaa0: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1409edaa6: e8 75 7e 1e 00              	callq	0x140bd5920 <.text+0xbd4920>
1409edaab: 48 8d 85 94 07 00 00        	leaq	0x794(%rbp), %rax
1409edab2: 48 89 85 40 08 00 00        	movq	%rax, 0x840(%rbp)
1409edab9: 0f 10 85 d0 07 00 00        	movups	0x7d0(%rbp), %xmm0
1409edac0: 0f 10 8d e0 07 00 00        	movups	0x7e0(%rbp), %xmm1
1409edac7: 0f 10 95 f0 07 00 00        	movups	0x7f0(%rbp), %xmm2
1409edace: 0f 10 9d fc 07 00 00        	movups	0x7fc(%rbp), %xmm3
1409edad5: 0f 11 9d 9c 04 00 00        	movups	%xmm3, 0x49c(%rbp)
1409edadc: 0f 29 95 90 04 00 00        	movaps	%xmm2, 0x490(%rbp)
1409edae3: 0f 29 8d 80 04 00 00        	movaps	%xmm1, 0x480(%rbp)
1409edaea: 0f 29 85 70 04 00 00        	movaps	%xmm0, 0x470(%rbp)
1409edaf1: 4c 89 7c 24 20              	movq	%r15, 0x20(%rsp)
1409edaf6: 48 8d 8d 40 08 00 00        	leaq	0x840(%rbp), %rcx
1409edafd: 4c 89 f2                    	movq	%r14, %rdx
1409edb00: 49 b8 68 2e b0 9e 28 cf df dc       	movabsq	$-0x232030d7614fd198, %r8 # imm = 0xDCDFCF289EB02E68
1409edb0a: 41 b1 19                    	movb	$0x19, %r9b
1409edb0d: e8 2e 07 f0 ff              	callq	0x1408ee240 <.text+0x8ed240>
1409edb12: 41 b8 90 01 00 00           	movl	$0x190, %r8d            # imm = 0x190
1409edb18: 4d 89 fc                    	movq	%r15, %r12
1409edb1b: 4c 89 f9                    	movq	%r15, %rcx
1409edb1e: 4c 89 f2                    	movq	%r14, %rdx
1409edb21: e8 da 60 f3 01              	callq	0x142923c00 <.text+0x2922c00>
1409edb26: c6 85 4e 08 00 00 01        	movb	$0x1, 0x84e(%rbp)
1409edb2d: 4c 89 f1                    	movq	%r14, %rcx
1409edb30: e8 6b 7c 1e 00              	callq	0x140bd57a0 <.text+0xbd47a0>
1409edb35: 48 8d bd d0 07 00 00        	leaq	0x7d0(%rbp), %rdi
1409edb3c: c6 85 4e 08 00 00 01        	movb	$0x1, 0x84e(%rbp)
1409edb43: 48 89 f9                    	movq	%rdi, %rcx
1409edb46: 4c 89 f2                    	movq	%r14, %rdx
1409edb49: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1409edb4f: e8 cc 7c 1e 00              	callq	0x140bd5820 <.text+0xbd4820>
1409edb54: 8b 85 3c 08 00 00           	movl	0x83c(%rbp), %eax
1409edb5a: 39 f0                       	cmpl	%esi, %eax
1409edb5c: 41 89 f0                    	movl	%esi, %r8d
1409edb5f: 44 0f 42 c0                 	cmovbl	%eax, %r8d
1409edb63: c6 85 4e 08 00 00 01        	movb	$0x1, 0x84e(%rbp)
1409edb6a: 4c 89 f1                    	movq	%r14, %rcx
1409edb6d: 48 89 fa                    	movq	%rdi, %rdx
1409edb70: e8 6b 7d 1e 00              	callq	0x140bd58e0 <.text+0xbd48e0>
1409edb75: c6 85 4e 08 00 00 01        	movb	$0x1, 0x84e(%rbp)
1409edb7c: 48 89 f9                    	movq	%rdi, %rcx
1409edb7f: 4c 89 f2                    	movq	%r14, %rdx
1409edb82: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1409edb88: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1409edb8e: e8 8d 7d 1e 00              	callq	0x140bd5920 <.text+0xbd4920>
1409edb93: 48 8d 85 0c 08 00 00        	leaq	0x80c(%rbp), %rax
1409edb9a: 48 89 85 40 08 00 00        	movq	%rax, 0x840(%rbp)
1409edba1: 0f 10 85 d0 07 00 00        	movups	0x7d0(%rbp), %xmm0
1409edba8: 0f 10 8d e0 07 00 00        	movups	0x7e0(%rbp), %xmm1
1409edbaf: 0f 10 95 f0 07 00 00        	movups	0x7f0(%rbp), %xmm2
1409edbb6: 0f 10 9d fc 07 00 00        	movups	0x7fc(%rbp), %xmm3
1409edbbd: 0f 11 9d 2c 06 00 00        	movups	%xmm3, 0x62c(%rbp)
1409edbc4: 0f 29 95 20 06 00 00        	movaps	%xmm2, 0x620(%rbp)
1409edbcb: 0f 29 8d 10 06 00 00        	movaps	%xmm1, 0x610(%rbp)
1409edbd2: 0f 29 85 00 06 00 00        	movaps	%xmm0, 0x600(%rbp)
1409edbd9: 4c 89 74 24 20              	movq	%r14, 0x20(%rsp)
1409edbde: 48 8d 8d 40 08 00 00        	leaq	0x840(%rbp), %rcx
1409edbe5: 4c 89 e2                    	movq	%r12, %rdx
1409edbe8: 49 b8 dd b3 40 cc 97 56 ba 11       	movabsq	$0x11ba5697cc40b3dd, %r8 # imm = 0x11BA5697CC40B3DD
1409edbf2: 41 b1 19                    	movb	$0x19, %r9b
1409edbf5: e8 46 06 f0 ff              	callq	0x1408ee240 <.text+0x8ed240>
1409edbfa: 41 b8 90 01 00 00           	movl	$0x190, %r8d            # imm = 0x190
1409edc00: 4c 89 f1                    	movq	%r14, %rcx
1409edc03: 4c 89 e2                    	movq	%r12, %rdx
1409edc06: e8 f5 5f f3 01              	callq	0x142923c00 <.text+0x2922c00>
1409edc0b: c6 85 4d 08 00 00 01        	movb	$0x1, 0x84d(%rbp)
1409edc12: 4c 89 e1                    	movq	%r12, %rcx
1409edc15: e8 86 7b 1e 00              	callq	0x140bd57a0 <.text+0xbd47a0>
1409edc1a: 4c 8d 75 c0                 	leaq	-0x40(%rbp), %r14
1409edc1e: 4d 89 e7                    	movq	%r12, %r15
1409edc21: c6 85 4d 08 00 00 01        	movb	$0x1, 0x84d(%rbp)
1409edc28: 48 89 f9                    	movq	%rdi, %rcx
1409edc2b: 4c 89 e2                    	movq	%r12, %rdx
1409edc2e: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1409edc34: e8 e7 7b 1e 00              	callq	0x140bd5820 <.text+0xbd4820>
1409edc39: c6 85 4d 08 00 00 01        	movb	$0x1, 0x84d(%rbp)
1409edc40: 4c 89 f9                    	movq	%r15, %rcx
1409edc43: 48 89 fa                    	movq	%rdi, %rdx
1409edc46: 41 89 d8                    	movl	%ebx, %r8d
1409edc49: e8 92 7c 1e 00              	callq	0x140bd58e0 <.text+0xbd48e0>
1409edc4e: c6 85 4d 08 00 00 01        	movb	$0x1, 0x84d(%rbp)
1409edc55: 48 89 f9                    	movq	%rdi, %rcx
1409edc58: 4c 89 fa                    	movq	%r15, %rdx
1409edc5b: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1409edc61: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1409edc67: e8 b4 7c 1e 00              	callq	0x140bd5920 <.text+0xbd4920>
1409edc6c: 0f 10 85 d0 07 00 00        	movups	0x7d0(%rbp), %xmm0
1409edc73: 0f 10 8d e0 07 00 00        	movups	0x7e0(%rbp), %xmm1
1409edc7a: 0f 10 95 f0 07 00 00        	movups	0x7f0(%rbp), %xmm2
1409edc81: 0f 10 9d fc 07 00 00        	movups	0x7fc(%rbp), %xmm3
1409edc88: 0f 29 85 70 04 00 00        	movaps	%xmm0, 0x470(%rbp)
1409edc8f: 0f 29 8d 80 04 00 00        	movaps	%xmm1, 0x480(%rbp)
1409edc96: 0f 29 95 90 04 00 00        	movaps	%xmm2, 0x490(%rbp)
1409edc9d: 0f 11 9d 9c 04 00 00        	movups	%xmm3, 0x49c(%rbp)
1409edca4: 48 8d 85 0c 08 00 00        	leaq	0x80c(%rbp), %rax
1409edcab: 48 89 85 d0 07 00 00        	movq	%rax, 0x7d0(%rbp)
1409edcb2: 4c 89 7c 24 20              	movq	%r15, 0x20(%rsp)
1409edcb7: 48 89 f9                    	movq	%rdi, %rcx
1409edcba: 48 8d bd 00 06 00 00        	leaq	0x600(%rbp), %rdi
1409edcc1: 48 89 fa                    	movq	%rdi, %rdx
1409edcc4: 49 b8 b2 92 98 e6 3a 9a 21 88       	movabsq	$-0x77de65c519676d4e, %r8 # imm = 0x88219A3AE69892B2
1409edcce: 41 b1 37                    	movb	$0x37, %r9b
1409edcd1: e8 2a 00 f0 ff              	callq	0x1408edd00 <.text+0x8ecd00>
1409edcd6: 41 b8 90 01 00 00           	movl	$0x190, %r8d            # imm = 0x190
1409edcdc: 48 8d 8d e0 02 00 00        	leaq	0x2e0(%rbp), %rcx
1409edce3: 48 89 fa                    	movq	%rdi, %rdx
1409edce6: e8 15 5f f3 01              	callq	0x142923c00 <.text+0x2922c00>
1409edceb: 80 bd 16 04 00 00 08        	cmpb	$0x8, 0x416(%rbp)
1409edcf2: 74 28                       	je	0x1409edd1c <.text+0x9ecd1c>
1409edcf4: 48 8b 95 18 04 00 00        	movq	0x418(%rbp), %rdx
1409edcfb: b8 e8 00 00 00              	movl	$0xe8, %eax
1409edd00: 48 01 c2                    	addq	%rax, %rdx
1409edd03: 48 8d 8d f0 03 00 00        	leaq	0x3f0(%rbp), %rcx
1409edd0a: 49 b8 ad 42 65 d3 21 f1 fc 51       	movabsq	$0x51fcf121d36542ad, %r8 # imm = 0x51FCF121D36542AD
1409edd14: 4d 89 e9                    	movq	%r13, %r9
1409edd17: e8 44 da f7 ff              	callq	0x14096b760 <.text+0x96a760>
1409edd1c: 41 b8 90 01 00 00           	movl	$0x190, %r8d            # imm = 0x190
1409edd22: 4c 89 f1                    	movq	%r14, %rcx
1409edd25: 48 8d 95 e0 02 00 00        	leaq	0x2e0(%rbp), %rdx
1409edd2c: e8 cf 5e f3 01              	callq	0x142923c00 <.text+0x2922c00>
1409edd31: 80 bd f6 00 00 00 08        	cmpb	$0x8, 0xf6(%rbp)
1409edd38: 74 2f                       	je	0x1409edd69 <.text+0x9ecd69>
1409edd3a: 3b b5 30 08 00 00           	cmpl	0x830(%rbp), %esi
1409edd40: 41 0f 95 c1                 	setne	%r9b
1409edd44: 48 8b 95 f8 00 00 00        	movq	0xf8(%rbp), %rdx
1409edd4b: b8 e8 00 00 00              	movl	$0xe8, %eax
1409edd50: 48 01 c2                    	addq	%rax, %rdx
1409edd53: 48 8d 8d d0 00 00 00        	leaq	0xd0(%rbp), %rcx
1409edd5a: 49 b8 12 bd 46 4a 20 83 a6 be       	movabsq	$-0x41597cdfb5b942ee, %r8 # imm = 0xBEA683204A46BD12
1409edd64: e8 e7 da f7 ff              	callq	0x14096b850 <.text+0x96a850>
1409edd69: 4c 89 ee                    	movq	%r13, %rsi
1409edd6c: 48 c1 ee 20                 	shrq	$0x20, %rsi
1409edd70: 41 b8 90 01 00 00           	movl	$0x190, %r8d            # imm = 0x190
1409edd76: 48 8d 8d 50 01 00 00        	leaq	0x150(%rbp), %rcx
1409edd7d: 4c 89 f2                    	movq	%r14, %rdx
1409edd80: e8 7b 5e f3 01              	callq	0x142923c00 <.text+0x2922c00>
1409edd85: 4c 8d 8d b0 01 00 00        	leaq	0x1b0(%rbp), %r9
1409edd8c: 41 0f 10 41 40              	movups	0x40(%r9), %xmm0
1409edd91: 0f 29 85 b0 04 00 00        	movaps	%xmm0, 0x4b0(%rbp)
1409edd98: 41 0f 10 01                 	movups	(%r9), %xmm0
1409edd9c: 41 0f 10 49 10              	movups	0x10(%r9), %xmm1
1409edda1: 41 0f 10 51 20              	movups	0x20(%r9), %xmm2
1409edda6: 41 0f 10 59 30              	movups	0x30(%r9), %xmm3
1409eddab: 0f 29 9d a0 04 00 00        	movaps	%xmm3, 0x4a0(%rbp)
1409eddb2: 0f 29 95 90 04 00 00        	movaps	%xmm2, 0x490(%rbp)
1409eddb9: 0f 29 8d 80 04 00 00        	movaps	%xmm1, 0x480(%rbp)
1409eddc0: 0f 29 85 70 04 00 00        	movaps	%xmm0, 0x470(%rbp)
1409eddc7: 48 8b 85 88 02 00 00        	movq	0x288(%rbp), %rax
1409eddce: 48 8b 8d d0 02 00 00        	movq	0x2d0(%rbp), %rcx
1409eddd5: 48 8b 95 60 01 00 00        	movq	0x160(%rbp), %rdx
1409edddc: 48 89 95 10 06 00 00        	movq	%rdx, 0x610(%rbp)
1409edde3: 0f 10 85 50 01 00 00        	movups	0x150(%rbp), %xmm0
1409eddea: 0f 29 85 00 06 00 00        	movaps	%xmm0, 0x600(%rbp)
1409eddf1: 49 8b 51 c8                 	movq	-0x38(%r9), %rdx
1409eddf5: 4c 8d 85 30 06 00 00        	leaq	0x630(%rbp), %r8
1409eddfc: 49 89 50 10                 	movq	%rdx, 0x10(%r8)
1409ede00: 41 0f 10 41 b8              	movups	-0x48(%r9), %xmm0
1409ede05: 41 0f 11 00                 	movups	%xmm0, (%r8)
1409ede09: 49 8b 51 e0                 	movq	-0x20(%r9), %rdx
1409ede0d: 49 89 50 f8                 	movq	%rdx, -0x8(%r8)
1409ede11: 41 0f 10 41 d0              	movups	-0x30(%r9), %xmm0
1409ede16: 41 0f 11 40 e8              	movups	%xmm0, -0x18(%r8)
1409ede1b: 41 0f 10 81 a0 00 00 00     	movups	0xa0(%r9), %xmm0
1409ede23: 41 0f 11 80 98 00 00 00     	movups	%xmm0, 0x98(%r8)
1409ede2b: 41 0f 10 81 90 00 00 00     	movups	0x90(%r9), %xmm0
1409ede33: 41 0f 11 80 88 00 00 00     	movups	%xmm0, 0x88(%r8)
1409ede3b: 41 0f 10 41 50              	movups	0x50(%r9), %xmm0
1409ede40: 41 0f 10 49 60              	movups	0x60(%r9), %xmm1
1409ede45: 41 0f 10 51 70              	movups	0x70(%r9), %xmm2
1409ede4a: 41 0f 10 99 80 00 00 00     	movups	0x80(%r9), %xmm3
1409ede52: 41 0f 11 58 78              	movups	%xmm3, 0x78(%r8)
1409ede57: 41 0f 11 50 68              	movups	%xmm2, 0x68(%r8)
1409ede5c: 41 0f 11 48 58              	movups	%xmm1, 0x58(%r8)
1409ede61: 41 0f 11 40 48              	movups	%xmm0, 0x48(%r8)
1409ede66: 49 8b 51 f8                 	movq	-0x8(%r9), %rdx
1409ede6a: 49 89 50 28                 	movq	%rdx, 0x28(%r8)
1409ede6e: 41 0f 10 41 e8              	movups	-0x18(%r9), %xmm0
1409ede73: 41 0f 11 40 18              	movups	%xmm0, 0x18(%r8)
1409ede78: 48 89 8d e0 06 00 00        	movq	%rcx, 0x6e0(%rbp)
1409ede7f: 48 89 85 d8 06 00 00        	movq	%rax, 0x6d8(%rbp)
1409ede86: c7 85 60 06 00 00 01 00 00 00       	movl	$0x1, 0x660(%rbp)
1409ede90: 44 89 ad 64 06 00 00        	movl	%r13d, 0x664(%rbp)
1409ede97: 89 b5 68 06 00 00           	movl	%esi, 0x668(%rbp)
1409ede9d: 8b 85 38 08 00 00           	movl	0x838(%rbp), %eax
1409edea3: 89 85 6c 06 00 00           	movl	%eax, 0x66c(%rbp)
1409edea9: 4c 89 f9                    	movq	%r15, %rcx
1409edeac: 48 89 fa                    	movq	%rdi, %rdx
1409edeaf: e8 1c f6 fe ff              	callq	0x1409dd4d0 <.text+0x9dc4d0>
1409edeb4: 80 bd 86 02 00 00 08        	cmpb	$0x8, 0x286(%rbp)
1409edebb: 48 8d bd d0 07 00 00        	leaq	0x7d0(%rbp), %rdi
1409edec2: 0f 84 d8 fa ff ff           	je	0x1409ed9a0 <.text+0x9ec9a0>
1409edec8: 48 8d 8d 70 02 00 00        	leaq	0x270(%rbp), %rcx
1409edecf: e8 dc 45 1f 00              	callq	0x140be24b0 <.text+0xbe14b0>
1409eded4: e9 c7 fa ff ff              	jmp	0x1409ed9a0 <.text+0x9ec9a0>
1409eded9: 8b 8d 2c 08 00 00           	movl	0x82c(%rbp), %ecx
1409ededf: 48 8b 85 c8 07 00 00        	movq	0x7c8(%rbp), %rax
1409edee6: 89 48 20                    	movl	%ecx, 0x20(%rax)
1409edee9: 44 8b 85 0c 08 00 00        	movl	0x80c(%rbp), %r8d
1409edef0: 48 8b 8d 10 08 00 00        	movq	0x810(%rbp), %rcx
1409edef7: 8b 95 18 08 00 00           	movl	0x818(%rbp), %edx
1409edefd: 0f 10 85 1c 08 00 00        	movups	0x81c(%rbp), %xmm0
1409edf04: 0f 11 40 10                 	movups	%xmm0, 0x10(%rax)
1409edf08: 44 89 00                    	movl	%r8d, (%rax)
1409edf0b: 48 89 48 04                 	movq	%rcx, 0x4(%rax)
1409edf0f: 89 50 0c                    	movl	%edx, 0xc(%rax)
1409edf12: 48 81 c4 d8 08 00 00        	addq	$0x8d8, %rsp            # imm = 0x8D8
1409edf19: 5b                          	popq	%rbx
1409edf1a: 5f                          	popq	%rdi
1409edf1b: 5e                          	popq	%rsi
1409edf1c: 41 5c                       	popq	%r12
1409edf1e: 41 5d                       	popq	%r13
1409edf20: 41 5e                       	popq	%r14
1409edf22: 41 5f                       	popq	%r15
1409edf24: 5d                          	popq	%rbp
1409edf25: c3                          	retq
1409edf26: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1409edf30: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409edf35: 55                          	pushq	%rbp
1409edf36: 41 57                       	pushq	%r15
1409edf38: 41 56                       	pushq	%r14
1409edf3a: 41 55                       	pushq	%r13
1409edf3c: 41 54                       	pushq	%r12
1409edf3e: 56                          	pushq	%rsi
1409edf3f: 57                          	pushq	%rdi
1409edf40: 53                          	pushq	%rbx
1409edf41: 48 83 ec 38                 	subq	$0x38, %rsp
1409edf45: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409edf4c: 80 bd 86 02 00 00 08        	cmpb	$0x8, 0x286(%rbp)
1409edf53: 74 0c                       	je	0x1409edf61 <.text+0x9ecf61>
1409edf55: 48 8d 8d 70 02 00 00        	leaq	0x270(%rbp), %rcx
1409edf5c: e8 4f 45 1f 00              	callq	0x140be24b0 <.text+0xbe14b0>
1409edf61: 90                          	nop
1409edf62: 48 83 c4 38                 	addq	$0x38, %rsp
1409edf66: 5b                          	popq	%rbx
1409edf67: 5f                          	popq	%rdi
1409edf68: 5e                          	popq	%rsi
1409edf69: 41 5c                       	popq	%r12
1409edf6b: 41 5d                       	popq	%r13
1409edf6d: 41 5e                       	popq	%r14
1409edf6f: 41 5f                       	popq	%r15
1409edf71: 5d                          	popq	%rbp
1409edf72: c3                          	retq
1409edf73: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1409edf80: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409edf85: 55                          	pushq	%rbp
1409edf86: 41 57                       	pushq	%r15
1409edf88: 41 56                       	pushq	%r14
1409edf8a: 41 55                       	pushq	%r13
1409edf8c: 41 54                       	pushq	%r12
1409edf8e: 56                          	pushq	%rsi
1409edf8f: 57                          	pushq	%rdi
1409edf90: 53                          	pushq	%rbx
1409edf91: 48 83 ec 38                 	subq	$0x38, %rsp
1409edf95: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409edf9c: 80 bd 4d 08 00 00 00        	cmpb	$0x0, 0x84d(%rbp)
1409edfa3: 75 11                       	jne	0x1409edfb6 <.text+0x9ecfb6>
1409edfa5: 48 83 c4 38                 	addq	$0x38, %rsp
1409edfa9: 5b                          	popq	%rbx
1409edfaa: 5f                          	popq	%rdi
1409edfab: 5e                          	popq	%rsi
1409edfac: 41 5c                       	popq	%r12
1409edfae: 41 5d                       	popq	%r13
1409edfb0: 41 5e                       	popq	%r14
1409edfb2: 41 5f                       	popq	%r15
1409edfb4: 5d                          	popq	%rbp
1409edfb5: c3                          	retq
1409edfb6: 48 8d 8d 00 06 00 00        	leaq	0x600(%rbp), %rcx
1409edfbd: e8 ee 8f ea ff              	callq	0x140896fb0 <.text+0x895fb0>
1409edfc2: eb e1                       	jmp	0x1409edfa5 <.text+0x9ecfa5>
1409edfc4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
1409edfd0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409edfd5: 55                          	pushq	%rbp
1409edfd6: 41 57                       	pushq	%r15
1409edfd8: 41 56                       	pushq	%r14
1409edfda: 41 55                       	pushq	%r13
1409edfdc: 41 54                       	pushq	%r12
1409edfde: 56                          	pushq	%rsi
1409edfdf: 57                          	pushq	%rdi
1409edfe0: 53                          	pushq	%rbx
1409edfe1: 48 83 ec 38                 	subq	$0x38, %rsp
1409edfe5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409edfec: 80 bd 4e 08 00 00 00        	cmpb	$0x0, 0x84e(%rbp)
1409edff3: 75 11                       	jne	0x1409ee006 <.text+0x9ed006>
1409edff5: 48 83 c4 38                 	addq	$0x38, %rsp
1409edff9: 5b                          	popq	%rbx
1409edffa: 5f                          	popq	%rdi
1409edffb: 5e                          	popq	%rsi
1409edffc: 41 5c                       	popq	%r12
1409edffe: 41 5d                       	popq	%r13
1409ee000: 41 5e                       	popq	%r14
1409ee002: 41 5f                       	popq	%r15
1409ee004: 5d                          	popq	%rbp
1409ee005: c3                          	retq
1409ee006: 48 8d 8d 70 04 00 00        	leaq	0x470(%rbp), %rcx
1409ee00d: e8 9e 8f ea ff              	callq	0x140896fb0 <.text+0x895fb0>
1409ee012: eb e1                       	jmp	0x1409edff5 <.text+0x9ecff5>
1409ee014: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
1409ee020: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409ee025: 55                          	pushq	%rbp
1409ee026: 41 57                       	pushq	%r15
1409ee028: 41 56                       	pushq	%r14
1409ee02a: 41 55                       	pushq	%r13
1409ee02c: 41 54                       	pushq	%r12
1409ee02e: 56                          	pushq	%rsi
1409ee02f: 57                          	pushq	%rdi
1409ee030: 53                          	pushq	%rbx
1409ee031: 48 83 ec 38                 	subq	$0x38, %rsp
1409ee035: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409ee03c: 80 bd 4f 08 00 00 00        	cmpb	$0x0, 0x84f(%rbp)
1409ee043: 75 11                       	jne	0x1409ee056 <.text+0x9ed056>
1409ee045: 48 83 c4 38                 	addq	$0x38, %rsp
1409ee049: 5b                          	popq	%rbx
1409ee04a: 5f                          	popq	%rdi
1409ee04b: 5e                          	popq	%rsi
1409ee04c: 41 5c                       	popq	%r12
1409ee04e: 41 5d                       	popq	%r13
1409ee050: 41 5e                       	popq	%r14
1409ee052: 41 5f                       	popq	%r15
1409ee054: 5d                          	popq	%rbp
1409ee055: c3                          	retq
1409ee056: 48 8d 8d 00 06 00 00        	leaq	0x600(%rbp), %rcx
1409ee05d: e8 4e 8f ea ff              	callq	0x140896fb0 <.text+0x895fb0>
1409ee062: eb e1                       	jmp	0x1409ee045 <.text+0x9ed045>
1409ee064: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
1409ee070: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409ee075: 55                          	pushq	%rbp
1409ee076: 41 57                       	pushq	%r15
1409ee078: 41 56                       	pushq	%r14
1409ee07a: 41 55                       	pushq	%r13
1409ee07c: 41 54                       	pushq	%r12
1409ee07e: 56                          	pushq	%rsi
1409ee07f: 57                          	pushq	%rdi
1409ee080: 53                          	pushq	%rbx
1409ee081: 48 83 ec 38                 	subq	$0x38, %rsp
1409ee085: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409ee08c: c6 85 4f 08 00 00 00        	movb	$0x0, 0x84f(%rbp)
1409ee093: 48 8d 8d 00 06 00 00        	leaq	0x600(%rbp), %rcx
1409ee09a: e8 11 8f ea ff              	callq	0x140896fb0 <.text+0x895fb0>
1409ee09f: c6 85 4f 08 00 00 00        	movb	$0x0, 0x84f(%rbp)
1409ee0a6: 48 83 c4 38                 	addq	$0x38, %rsp
1409ee0aa: 5b                          	popq	%rbx
1409ee0ab: 5f                          	popq	%rdi
1409ee0ac: 5e                          	popq	%rsi
1409ee0ad: 41 5c                       	popq	%r12
1409ee0af: 41 5d                       	popq	%r13
1409ee0b1: 41 5e                       	popq	%r14
1409ee0b3: 41 5f                       	popq	%r15
1409ee0b5: 5d                          	popq	%rbp
1409ee0b6: c3                          	retq
1409ee0b7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1409ee0c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409ee0c5: 55                          	pushq	%rbp
1409ee0c6: 41 57                       	pushq	%r15
1409ee0c8: 41 56                       	pushq	%r14
1409ee0ca: 41 55                       	pushq	%r13
1409ee0cc: 41 54                       	pushq	%r12
1409ee0ce: 56                          	pushq	%rsi
1409ee0cf: 57                          	pushq	%rdi
1409ee0d0: 53                          	pushq	%rbx
1409ee0d1: 48 83 ec 38                 	subq	$0x38, %rsp
1409ee0d5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409ee0dc: c6 85 4e 08 00 00 00        	movb	$0x0, 0x84e(%rbp)
1409ee0e3: 48 8d 8d 70 04 00 00        	leaq	0x470(%rbp), %rcx
1409ee0ea: e8 c1 8e ea ff              	callq	0x140896fb0 <.text+0x895fb0>
1409ee0ef: c6 85 4e 08 00 00 00        	movb	$0x0, 0x84e(%rbp)
1409ee0f6: 48 83 c4 38                 	addq	$0x38, %rsp
1409ee0fa: 5b                          	popq	%rbx
1409ee0fb: 5f                          	popq	%rdi
1409ee0fc: 5e                          	popq	%rsi
1409ee0fd: 41 5c                       	popq	%r12
1409ee0ff: 41 5d                       	popq	%r13
1409ee101: 41 5e                       	popq	%r14
1409ee103: 41 5f                       	popq	%r15
1409ee105: 5d                          	popq	%rbp
1409ee106: c3                          	retq
1409ee107: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1409ee110: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409ee115: 55                          	pushq	%rbp
1409ee116: 41 57                       	pushq	%r15
1409ee118: 41 56                       	pushq	%r14
1409ee11a: 41 55                       	pushq	%r13
1409ee11c: 41 54                       	pushq	%r12
1409ee11e: 56                          	pushq	%rsi
1409ee11f: 57                          	pushq	%rdi
1409ee120: 53                          	pushq	%rbx
1409ee121: 48 83 ec 38                 	subq	$0x38, %rsp
1409ee125: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409ee12c: c6 85 4d 08 00 00 00        	movb	$0x0, 0x84d(%rbp)
1409ee133: 48 8d 8d 00 06 00 00        	leaq	0x600(%rbp), %rcx
1409ee13a: e8 71 8e ea ff              	callq	0x140896fb0 <.text+0x895fb0>
1409ee13f: c6 85 4d 08 00 00 00        	movb	$0x0, 0x84d(%rbp)
1409ee146: 48 83 c4 38                 	addq	$0x38, %rsp
1409ee14a: 5b                          	popq	%rbx
1409ee14b: 5f                          	popq	%rdi
1409ee14c: 5e                          	popq	%rsi
1409ee14d: 41 5c                       	popq	%r12
1409ee14f: 41 5d                       	popq	%r13
1409ee151: 41 5e                       	popq	%r14
1409ee153: 41 5f                       	popq	%r15
1409ee155: 5d                          	popq	%rbp
1409ee156: c3                          	retq
1409ee157: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1409ee160: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409ee165: 55                          	pushq	%rbp
1409ee166: 41 57                       	pushq	%r15
1409ee168: 41 56                       	pushq	%r14
1409ee16a: 41 55                       	pushq	%r13
1409ee16c: 41 54                       	pushq	%r12
1409ee16e: 56                          	pushq	%rsi
1409ee16f: 57                          	pushq	%rdi
1409ee170: 53                          	pushq	%rbx
1409ee171: 48 83 ec 38                 	subq	$0x38, %rsp
1409ee175: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409ee17c: 48 8d 8d e0 02 00 00        	leaq	0x2e0(%rbp), %rcx
1409ee183: e8 28 8e ea ff              	callq	0x140896fb0 <.text+0x895fb0>
1409ee188: 90                          	nop
1409ee189: 48 83 c4 38                 	addq	$0x38, %rsp
1409ee18d: 5b                          	popq	%rbx
1409ee18e: 5f                          	popq	%rdi
1409ee18f: 5e                          	popq	%rsi
1409ee190: 41 5c                       	popq	%r12
1409ee192: 41 5d                       	popq	%r13
1409ee194: 41 5e                       	popq	%r14
1409ee196: 41 5f                       	popq	%r15
1409ee198: 5d                          	popq	%rbp
1409ee199: c3                          	retq
1409ee19a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1409ee1a0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409ee1a5: 55                          	pushq	%rbp
1409ee1a6: 41 57                       	pushq	%r15
1409ee1a8: 41 56                       	pushq	%r14
1409ee1aa: 41 55                       	pushq	%r13
1409ee1ac: 41 54                       	pushq	%r12
1409ee1ae: 56                          	pushq	%rsi
1409ee1af: 57                          	pushq	%rdi
1409ee1b0: 53                          	pushq	%rbx
1409ee1b1: 48 83 ec 38                 	subq	$0x38, %rsp
1409ee1b5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409ee1bc: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
1409ee1c0: e8 eb 8d ea ff              	callq	0x140896fb0 <.text+0x895fb0>
1409ee1c5: 90                          	nop
1409ee1c6: 48 83 c4 38                 	addq	$0x38, %rsp
1409ee1ca: 5b                          	popq	%rbx
1409ee1cb: 5f                          	popq	%rdi
1409ee1cc: 5e                          	popq	%rsi
1409ee1cd: 41 5c                       	popq	%r12
1409ee1cf: 41 5d                       	popq	%r13
1409ee1d1: 41 5e                       	popq	%r14
1409ee1d3: 41 5f                       	popq	%r15
1409ee1d5: 5d                          	popq	%rbp
1409ee1d6: c3                          	retq
