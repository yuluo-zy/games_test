
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a20890: 41 57                       	pushq	%r15
140a20892: 41 56                       	pushq	%r14
140a20894: 41 54                       	pushq	%r12
140a20896: 56                          	pushq	%rsi
140a20897: 57                          	pushq	%rdi
140a20898: 53                          	pushq	%rbx
140a20899: 48 81 ec e8 00 00 00        	subq	$0xe8, %rsp
140a208a0: 80 b9 b0 01 00 00 00        	cmpb	$0x0, 0x1b0(%rcx)
140a208a7: 0f 85 8d 02 00 00           	jne	0x140a20b3a <.text+0xa1fb3a>
140a208ad: 48 89 d6                    	movq	%rdx, %rsi
140a208b0: 48 89 cb                    	movq	%rcx, %rbx
140a208b3: 4c 8b 72 10                 	movq	0x10(%rdx), %r14
140a208b7: 49 83 fe 03                 	cmpq	$0x3, %r14
140a208bb: 75 0f                       	jne	0x140a208cc <.text+0xa1f8cc>
140a208bd: 4c 8b 36                    	movq	(%rsi), %r14
140a208c0: 4c 8b 7e 08                 	movq	0x8(%rsi), %r15
140a208c4: 8b 7e 68                    	movl	0x68(%rsi), %edi
140a208c7: e9 90 00 00 00              	jmp	0x140a2095c <.text+0xa1f95c>
140a208cc: 8b 7e 68                    	movl	0x68(%rsi), %edi
140a208cf: 89 7c 24 60                 	movl	%edi, 0x60(%rsp)
140a208d3: 0f 10 06                    	movups	(%rsi), %xmm0
140a208d6: 0f 29 44 24 50              	movaps	%xmm0, 0x50(%rsp)
140a208db: 4c 8d a4 24 a0 00 00 00     	leaq	0xa0(%rsp), %r12
140a208e3: 4c 8d 44 24 50              	leaq	0x50(%rsp), %r8
140a208e8: 4c 89 e1                    	movq	%r12, %rcx
140a208eb: 48 89 da                    	movq	%rbx, %rdx
140a208ee: 45 31 c9                    	xorl	%r9d, %r9d
140a208f1: e8 7a fd ff ff              	callq	0x140a20670 <.text+0xa1f670>
140a208f6: f3 0f 6f 46 20              	movdqu	0x20(%rsi), %xmm0
140a208fb: f3 0f 6f 4e 30              	movdqu	0x30(%rsi), %xmm1
140a20900: 66 0f 7f 84 24 a0 00 00 00  	movdqa	%xmm0, 0xa0(%rsp)
140a20909: 66 0f 7f 8c 24 b0 00 00 00  	movdqa	%xmm1, 0xb0(%rsp)
140a20912: 48 8b 46 40                 	movq	0x40(%rsi), %rax
140a20916: 48 89 84 24 c0 00 00 00     	movq	%rax, 0xc0(%rsp)
140a2091e: 4c 8b 7e 18                 	movq	0x18(%rsi), %r15
140a20922: 48 89 d9                    	movq	%rbx, %rcx
140a20925: 4c 89 f2                    	movq	%r14, %rdx
140a20928: 4d 89 f8                    	movq	%r15, %r8
140a2092b: e8 00 c6 ff ff              	callq	0x140a1cf30 <.text+0xa1bf30>
140a20930: 48 89 54 24 40              	movq	%rdx, 0x40(%rsp)
140a20935: 4c 89 74 24 30              	movq	%r14, 0x30(%rsp)
140a2093a: 4c 89 7c 24 38              	movq	%r15, 0x38(%rsp)
140a2093f: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140a20944: 48 89 4c 24 20              	movq	%rcx, 0x20(%rsp)
140a20949: 48 8d 4c 24 50              	leaq	0x50(%rsp), %rcx
140a2094e: 48 89 c2                    	movq	%rax, %rdx
140a20951: 41 89 f8                    	movl	%edi, %r8d
140a20954: 4d 89 e1                    	movq	%r12, %r9
140a20957: e8 84 a6 b3 00              	callq	0x14155afe0 <.text+0x1559fe0>
140a2095c: 48 89 d9                    	movq	%rbx, %rcx
140a2095f: 4c 89 f2                    	movq	%r14, %rdx
140a20962: 4d 89 f8                    	movq	%r15, %r8
140a20965: e8 76 c4 ff ff              	callq	0x140a1cde0 <.text+0xa1bde0>
140a2096a: 48 85 c0                    	testq	%rax, %rax
140a2096d: 0f 84 8f 02 00 00           	je	0x140a20c02 <.text+0xa1fc02>
140a20973: 48 89 c3                    	movq	%rax, %rbx
140a20976: 89 7c 24 50                 	movl	%edi, 0x50(%rsp)
140a2097a: 48 8d 48 60                 	leaq	0x60(%rax), %rcx
140a2097e: 89 fa                       	movl	%edi, %edx
140a20980: e8 7b 88 8a ff              	callq	0x1402c9200 <.text+0x2c8200>
140a20985: 48 83 7b 48 00              	cmpq	$0x0, 0x48(%rbx)
140a2098a: 0f 84 e6 00 00 00           	je	0x140a20a76 <.text+0xa1fa76>
140a20990: 48 89 d9                    	movq	%rbx, %rcx
140a20993: 48 83 c1 50                 	addq	$0x50, %rcx
140a20997: 48 8d 54 24 50              	leaq	0x50(%rsp), %rdx
140a2099c: e8 2f 05 b9 ff              	callq	0x1405b0ed0 <.text+0x5afed0>
140a209a1: 48 8b 4b 30                 	movq	0x30(%rbx), %rcx
140a209a5: 48 8b 53 38                 	movq	0x38(%rbx), %rdx
140a209a9: 49 89 d0                    	movq	%rdx, %r8
140a209ac: 49 21 c0                    	andq	%rax, %r8
140a209af: 48 c1 e8 39                 	shrq	$0x39, %rax
140a209b3: 66 0f 6e c0                 	movd	%eax, %xmm0
140a209b7: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140a209bb: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140a209c0: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140a209c5: 48 8d 41 b0                 	leaq	-0x50(%rcx), %rax
140a209c9: 45 31 c9                    	xorl	%r9d, %r9d
140a209cc: 44 8b 54 24 50              	movl	0x50(%rsp), %r10d
140a209d1: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
140a209d5: f3 42 0f 6f 14 01           	movdqu	(%rcx,%r8), %xmm2
140a209db: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
140a209df: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
140a209e3: 66 44 0f d7 db              	pmovmskb	%xmm3, %r11d
140a209e8: 45 85 db                    	testl	%r11d, %r11d
140a209eb: 74 33                       	je	0x140a20a20 <.text+0xa1fa20>
140a209ed: f3 41 0f bc fb              	tzcntl	%r11d, %edi
140a209f2: 4c 01 c7                    	addq	%r8, %rdi
140a209f5: 48 21 d7                    	andq	%rdx, %rdi
140a209f8: 48 f7 df                    	negq	%rdi
140a209fb: 48 8d 3c bf                 	leaq	(%rdi,%rdi,4), %rdi
140a209ff: 48 c1 e7 04                 	shlq	$0x4, %rdi
140a20a03: 44 3b 14 38                 	cmpl	(%rax,%rdi), %r10d
140a20a07: 74 35                       	je	0x140a20a3e <.text+0xa1fa3e>
140a20a09: 41 8d 7b ff                 	leal	-0x1(%r11), %edi
140a20a0d: 66 44 21 df                 	andw	%r11w, %di
140a20a11: 41 89 fb                    	movl	%edi, %r11d
140a20a14: 75 d7                       	jne	0x140a209ed <.text+0xa1f9ed>
140a20a16: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140a20a20: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
140a20a24: 66 44 0f d7 da              	pmovmskb	%xmm2, %r11d
140a20a29: 45 85 db                    	testl	%r11d, %r11d
140a20a2c: 75 48                       	jne	0x140a20a76 <.text+0xa1fa76>
140a20a2e: 4d 01 c8                    	addq	%r9, %r8
140a20a31: 49 83 c0 10                 	addq	$0x10, %r8
140a20a35: 49 83 c1 10                 	addq	$0x10, %r9
140a20a39: 49 21 d0                    	andq	%rdx, %r8
140a20a3c: eb 97                       	jmp	0x140a209d5 <.text+0xa1f9d5>
140a20a3e: 48 01 f9                    	addq	%rdi, %rcx
140a20a41: 80 7e 60 01                 	cmpb	$0x1, 0x60(%rsi)
140a20a45: 74 3b                       	je	0x140a20a82 <.text+0xa1fa82>
140a20a47: 0f b6 46 65                 	movzbl	0x65(%rsi), %eax
140a20a4b: 3c 40                       	cmpb	$0x40, %al
140a20a4d: 75 42                       	jne	0x140a20a91 <.text+0xa1fa91>
140a20a4f: 80 7e 62 01                 	cmpb	$0x1, 0x62(%rsi)
140a20a53: 74 45                       	je	0x140a20a9a <.text+0xa1fa9a>
140a20a55: 83 7e 50 01                 	cmpl	$0x1, 0x50(%rsi)
140a20a59: 74 5e                       	je	0x140a20ab9 <.text+0xa1fab9>
140a20a5b: f6 46 58 01                 	testb	$0x1, 0x58(%rsi)
140a20a5f: 0f 85 8a 00 00 00           	jne	0x140a20aef <.text+0xa1faef>
140a20a65: 0f b6 46 64                 	movzbl	0x64(%rsi), %eax
140a20a69: 3c 02                       	cmpb	$0x2, %al
140a20a6b: 0f 85 a3 00 00 00           	jne	0x140a20b14 <.text+0xa1fb14>
140a20a71: e9 b3 00 00 00              	jmp	0x140a20b29 <.text+0xa1fb29>
140a20a76: 48 8d 0d f3 89 0a 02        	leaq	0x20a89f3(%rip), %rcx   # 0x142ac9470
140a20a7d: e8 8e 88 eb 01              	callq	0x1428d9310 <.text+0x28d8310>
140a20a82: 0f b6 46 61                 	movzbl	0x61(%rsi), %eax
140a20a86: 88 41 fc                    	movb	%al, -0x4(%rcx)
140a20a89: 0f b6 46 65                 	movzbl	0x65(%rsi), %eax
140a20a8d: 3c 40                       	cmpb	$0x40, %al
140a20a8f: 74 be                       	je	0x140a20a4f <.text+0xa1fa4f>
140a20a91: 88 41 fe                    	movb	%al, -0x2(%rcx)
140a20a94: 80 7e 62 01                 	cmpb	$0x1, 0x62(%rsi)
140a20a98: 75 bb                       	jne	0x140a20a55 <.text+0xa1fa55>
140a20a9a: 0f be 46 63                 	movsbl	0x63(%rsi), %eax
140a20a9e: 01 41 e4                    	addl	%eax, -0x1c(%rcx)
140a20aa1: 31 c0                       	xorl	%eax, %eax
140a20aa3: 80 3d e6 42 c1 02 00        	cmpb	$0x0, 0x2c142e6(%rip)   # 0x143634d90
140a20aaa: 0f 94 c0                    	sete	%al
140a20aad: 8d 04 80                    	leal	(%rax,%rax,4), %eax
140a20ab0: 88 41 fd                    	movb	%al, -0x3(%rcx)
140a20ab3: 83 7e 50 01                 	cmpl	$0x1, 0x50(%rsi)
140a20ab7: 75 a2                       	jne	0x140a20a5b <.text+0xa1fa5b>
140a20ab9: 0f b6 41 f9                 	movzbl	-0x7(%rcx), %eax
140a20abd: 04 fe                       	addb	$-0x2, %al
140a20abf: 0f b6 c0                    	movzbl	%al, %eax
140a20ac2: 3c 06                       	cmpb	$0x6, %al
140a20ac4: ba 06 00 00 00              	movl	$0x6, %edx
140a20ac9: 0f 42 d0                    	cmovbl	%eax, %edx
140a20acc: 83 ca 04                    	orl	$0x4, %edx
140a20acf: 0f b6 c2                    	movzbl	%dl, %eax
140a20ad2: 83 f8 06                    	cmpl	$0x6, %eax
140a20ad5: 0f 85 33 01 00 00           	jne	0x140a20c0e <.text+0xa1fc0e>
140a20adb: 66 0f 6e 46 54              	movd	0x54(%rsi), %xmm0
140a20ae0: 66 0f 7e 41 e8              	movd	%xmm0, -0x18(%rcx)
140a20ae5: f6 46 58 01                 	testb	$0x1, 0x58(%rsi)
140a20ae9: 0f 84 76 ff ff ff           	je	0x140a20a65 <.text+0xa1fa65>
140a20aef: 66 0f 6e 46 5c              	movd	0x5c(%rsi), %xmm0
140a20af4: 48 8d 41 e8                 	leaq	-0x18(%rcx), %rax
140a20af8: 48 89 44 24 48              	movq	%rax, 0x48(%rsp)
140a20afd: 0f b6 41 f9                 	movzbl	-0x7(%rcx), %eax
140a20b01: 0c f8                       	orb	$-0x8, %al
140a20b03: 3c fa                       	cmpb	$-0x6, %al
140a20b05: 73 4b                       	jae	0x140a20b52 <.text+0xa1fb52>
140a20b07: 66 0f 7e 41 ec              	movd	%xmm0, -0x14(%rcx)
140a20b0c: 0f b6 46 64                 	movzbl	0x64(%rsi), %eax
140a20b10: 3c 02                       	cmpb	$0x2, %al
140a20b12: 74 15                       	je	0x140a20b29 <.text+0xa1fb29>
140a20b14: 48 8d 51 e8                 	leaq	-0x18(%rcx), %rdx
140a20b18: 48 89 54 24 48              	movq	%rdx, 0x48(%rsp)
140a20b1d: 80 79 f9 05                 	cmpb	$0x5, -0x7(%rcx)
140a20b21: 0f 85 83 00 00 00           	jne	0x140a20baa <.text+0xa1fbaa>
140a20b27: 88 02                       	movb	%al, (%rdx)
140a20b29: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
140a20b30: 5b                          	popq	%rbx
140a20b31: 5f                          	popq	%rdi
140a20b32: 5e                          	popq	%rsi
140a20b33: 41 5c                       	popq	%r12
140a20b35: 41 5e                       	popq	%r14
140a20b37: 41 5f                       	popq	%r15
140a20b39: c3                          	retq
140a20b3a: 48 8d 0d 57 85 0a 02        	leaq	0x20a8557(%rip), %rcx   # 0x142ac9098
140a20b41: 4c 8d 05 b8 89 0a 02        	leaq	0x20a89b8(%rip), %r8    # 0x142ac9500
140a20b48: ba 21 00 00 00              	movl	$0x21, %edx
140a20b4d: e8 de 88 eb 01              	callq	0x1428d9430 <.text+0x28d8430>
140a20b52: 48 8d 44 24 48              	leaq	0x48(%rsp), %rax
140a20b57: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
140a20b5c: 48 8d 05 ad 37 02 00        	leaq	0x237ad(%rip), %rax     # 0x140a44310 <.text+0xa43310>
140a20b63: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
140a20b68: 48 8d 05 01 22 07 02        	leaq	0x2072201(%rip), %rax   # 0x142a92d70
140a20b6f: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
140a20b74: 48 c7 44 24 58 01 00 00 00  	movq	$0x1, 0x58(%rsp)
140a20b7d: 48 c7 44 24 70 00 00 00 00  	movq	$0x0, 0x70(%rsp)
140a20b86: 48 8d 44 24 30              	leaq	0x30(%rsp), %rax
140a20b8b: 48 89 44 24 60              	movq	%rax, 0x60(%rsp)
140a20b90: 48 c7 44 24 68 01 00 00 00  	movq	$0x1, 0x68(%rsp)
140a20b99: 48 8d 15 30 89 0a 02        	leaq	0x20a8930(%rip), %rdx   # 0x142ac94d0
140a20ba0: 48 8d 4c 24 50              	leaq	0x50(%rsp), %rcx
140a20ba5: e8 e6 87 eb 01              	callq	0x1428d9390 <.text+0x28d8390>
140a20baa: 48 8d 44 24 48              	leaq	0x48(%rsp), %rax
140a20baf: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
140a20bb4: 48 8d 05 55 37 02 00        	leaq	0x23755(%rip), %rax     # 0x140a44310 <.text+0xa43310>
140a20bbb: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
140a20bc0: 48 8d 05 01 21 07 02        	leaq	0x2072101(%rip), %rax   # 0x142a92cc8
140a20bc7: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
140a20bcc: 48 c7 44 24 58 01 00 00 00  	movq	$0x1, 0x58(%rsp)
140a20bd5: 48 c7 44 24 70 00 00 00 00  	movq	$0x0, 0x70(%rsp)
140a20bde: 48 8d 44 24 30              	leaq	0x30(%rsp), %rax
140a20be3: 48 89 44 24 60              	movq	%rax, 0x60(%rsp)
140a20be8: 48 c7 44 24 68 01 00 00 00  	movq	$0x1, 0x68(%rsp)
140a20bf1: 48 8d 15 f0 88 0a 02        	leaq	0x20a88f0(%rip), %rdx   # 0x142ac94e8
140a20bf8: 48 8d 4c 24 50              	leaq	0x50(%rsp), %rcx
140a20bfd: e8 8e 87 eb 01              	callq	0x1428d9390 <.text+0x28d8390>
140a20c02: 48 8d 0d 4f 88 0a 02        	leaq	0x20a884f(%rip), %rcx   # 0x142ac9458
140a20c09: e8 02 87 eb 01              	callq	0x1428d9310 <.text+0x28d8310>
140a20c0e: 48 83 c1 fe                 	addq	$-0x2, %rcx
140a20c12: 48 89 4c 24 30              	movq	%rcx, 0x30(%rsp)
140a20c17: 48 8d 05 f2 35 00 00        	leaq	0x35f2(%rip), %rax      # 0x140a24210 <.text+0xa23210>
140a20c1e: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
140a20c23: 48 8d 05 7e 88 0a 02        	leaq	0x20a887e(%rip), %rax   # 0x142ac94a8
140a20c2a: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
140a20c2f: 48 c7 44 24 58 01 00 00 00  	movq	$0x1, 0x58(%rsp)
140a20c38: 48 c7 44 24 70 00 00 00 00  	movq	$0x0, 0x70(%rsp)
140a20c41: 48 8d 44 24 30              	leaq	0x30(%rsp), %rax
140a20c46: 48 89 44 24 60              	movq	%rax, 0x60(%rsp)
140a20c4b: 48 c7 44 24 68 01 00 00 00  	movq	$0x1, 0x68(%rsp)
140a20c54: 48 8d 15 5d 88 0a 02        	leaq	0x20a885d(%rip), %rdx   # 0x142ac94b8
140a20c5b: 48 8d 4c 24 50              	leaq	0x50(%rsp), %rcx
140a20c60: e8 2b 87 eb 01              	callq	0x1428d9390 <.text+0x28d8390>
140a20c65: cc                          	int3
