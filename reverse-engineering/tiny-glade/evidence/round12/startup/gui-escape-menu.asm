
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1403cf9b0: 41 57                       	pushq	%r15
1403cf9b2: 41 56                       	pushq	%r14
1403cf9b4: 41 55                       	pushq	%r13
1403cf9b6: 41 54                       	pushq	%r12
1403cf9b8: 56                          	pushq	%rsi
1403cf9b9: 57                          	pushq	%rdi
1403cf9ba: 55                          	pushq	%rbp
1403cf9bb: 53                          	pushq	%rbx
1403cf9bc: 48 81 ec 48 01 00 00        	subq	$0x148, %rsp            # imm = 0x148
1403cf9c3: 4c 89 ce                    	movq	%r9, %rsi
1403cf9c6: 49 89 ce                    	movq	%rcx, %r14
1403cf9c9: 4c 8b 8c 24 f0 01 00 00     	movq	0x1f0(%rsp), %r9
1403cf9d1: 48 8b 84 24 b0 01 00 00     	movq	0x1b0(%rsp), %rax
1403cf9d9: 48 8b 0a                    	movq	(%rdx), %rcx
1403cf9dc: 48 89 8c 24 e8 00 00 00     	movq	%rcx, 0xe8(%rsp)
1403cf9e4: 48 8b 6a 20                 	movq	0x20(%rdx), %rbp
1403cf9e8: 48 8b 5a 40                 	movq	0x40(%rdx), %rbx
1403cf9ec: 4c 8b 62 60                 	movq	0x60(%rdx), %r12
1403cf9f0: 41 0f 10 00                 	movups	(%r8), %xmm0
1403cf9f4: 41 0f 10 48 10              	movups	0x10(%r8), %xmm1
1403cf9f9: 0f 29 8c 24 30 01 00 00     	movaps	%xmm1, 0x130(%rsp)
1403cfa01: 0f 29 84 24 20 01 00 00     	movaps	%xmm0, 0x120(%rsp)
1403cfa09: 49 8b 48 20                 	movq	0x20(%r8), %rcx
1403cfa0d: 48 89 8c 24 d0 00 00 00     	movq	%rcx, 0xd0(%rsp)
1403cfa15: 49 8b 49 20                 	movq	0x20(%r9), %rcx
1403cfa19: 48 89 8c 24 e0 00 00 00     	movq	%rcx, 0xe0(%rsp)
1403cfa21: 49 8b 49 30                 	movq	0x30(%r9), %rcx
1403cfa25: 48 89 8c 24 d8 00 00 00     	movq	%rcx, 0xd8(%rsp)
1403cfa2d: 41 8b 49 3c                 	movl	0x3c(%r9), %ecx
1403cfa31: 89 8c 24 c8 00 00 00        	movl	%ecx, 0xc8(%rsp)
1403cfa38: c6 44 24 32 00              	movb	$0x0, 0x32(%rsp)
1403cfa3d: c6 44 24 33 00              	movb	$0x0, 0x33(%rsp)
1403cfa42: 8b 48 1c                    	movl	0x1c(%rax), %ecx
1403cfa45: 48 8b 50 10                 	movq	0x10(%rax), %rdx
1403cfa49: 89 4c 24 34                 	movl	%ecx, 0x34(%rsp)
1403cfa4d: 48 89 54 24 58              	movq	%rdx, 0x58(%rsp)
1403cfa52: 89 0a                       	movl	%ecx, (%rdx)
1403cfa54: 48 8b 38                    	movq	(%rax), %rdi
1403cfa57: 48 8d 05 eb 20 60 02        	leaq	0x26020eb(%rip), %rax   # 0x1429d1b49
1403cfa5e: 48 89 44 24 68              	movq	%rax, 0x68(%rsp)
1403cfa63: 48 c7 44 24 70 0a 00 00 00  	movq	$0xa, 0x70(%rsp)
1403cfa6c: 48 8d 4c 24 68              	leaq	0x68(%rsp), %rcx
1403cfa71: e8 9a 41 2b 00              	callq	0x140683c10 <.text+0x682c10>
1403cfa76: 49 89 c7                    	movq	%rax, %r15
1403cfa79: 48 b8 43 b4 4f 00 20 53 69 b1       	movabsq	$-0x4e96acdfffb04bbd, %rax # imm = 0xB1695320004FB443
1403cfa83: 49 31 c7                    	xorq	%rax, %r15
1403cfa86: 48 89 f9                    	movq	%rdi, %rcx
1403cfa89: 4c 89 fa                    	movq	%r15, %rdx
1403cfa8c: e8 2f 44 2b 00              	callq	0x140683ec0 <.text+0x682ec0>
1403cfa91: c6 40 5d 00                 	movb	$0x0, 0x5d(%rax)
1403cfa95: f6 c2 01                    	testb	$0x1, %dl
1403cfa98: 75 33                       	jne	0x1403cfacd <.text+0x3ceacd>
1403cfa9a: 4c 8b 6f 10                 	movq	0x10(%rdi), %r13
1403cfa9e: 4c 3b 2f                    	cmpq	(%rdi), %r13
1403cfaa1: 75 0f                       	jne	0x1403cfab2 <.text+0x3ceab2>
1403cfaa3: 48 8d 15 96 f7 5e 02        	leaq	0x25ef796(%rip), %rdx   # 0x1429bf240
1403cfaaa: 48 89 f9                    	movq	%rdi, %rcx
1403cfaad: e8 1e 1f 26 00              	callq	0x1406319d0 <.text+0x6309d0>
1403cfab2: 48 8b 47 08                 	movq	0x8(%rdi), %rax
1403cfab6: 49 6b cd 38                 	imulq	$0x38, %r13, %rcx
1403cfaba: c7 04 08 02 00 00 00        	movl	$0x2, (%rax,%rcx)
1403cfac1: 4c 89 7c 08 30              	movq	%r15, 0x30(%rax,%rcx)
1403cfac6: 49 ff c5                    	incq	%r13
1403cfac9: 4c 89 6f 10                 	movq	%r13, 0x10(%rdi)
1403cfacd: 48 89 7c 24 38              	movq	%rdi, 0x38(%rsp)
1403cfad2: 4c 89 7c 24 40              	movq	%r15, 0x40(%rsp)
1403cfad7: 48 c7 44 24 48 00 00 00 00  	movq	$0x0, 0x48(%rsp)
1403cfae0: 4c 8d 05 6c 20 60 02        	leaq	0x260206c(%rip), %r8    # 0x1429d1b53
1403cfae7: 48 8d 4c 24 68              	leaq	0x68(%rsp), %rcx
1403cfaec: 48 8d 54 24 38              	leaq	0x38(%rsp), %rdx
1403cfaf1: 41 b9 09 00 00 00           	movl	$0x9, %r9d
1403cfaf7: e8 e4 c3 2b 00              	callq	0x14068bee0 <.text+0x68aee0>
1403cfafc: 48 8d 05 59 20 60 02        	leaq	0x2602059(%rip), %rax   # 0x1429d1b5c
1403cfb03: 48 89 84 24 f0 00 00 00     	movq	%rax, 0xf0(%rsp)
1403cfb0b: 48 c7 84 24 f8 00 00 00 06 00 00 00 	movq	$0x6, 0xf8(%rsp)
1403cfb17: 48 c7 84 24 00 01 00 00 01 00 00 00 	movq	$0x1, 0x100(%rsp)
1403cfb23: 48 c7 84 24 08 01 00 00 00 00 00 00 	movq	$0x0, 0x108(%rsp)
1403cfb2f: 48 c7 84 24 10 01 00 00 01 00 00 00 	movq	$0x1, 0x110(%rsp)
1403cfb3b: 48 c7 84 24 18 01 00 00 00 00 00 00 	movq	$0x0, 0x118(%rsp)
1403cfb47: c7 84 24 cc 00 00 00 00 00 a0 41    	movl	$0x41a00000, 0xcc(%rsp) # imm = 0x41A00000
1403cfb52: 41 80 7c 24 01 00           	cmpb	$0x0, 0x1(%r12)
1403cfb58: 75 50                       	jne	0x1403cfbaa <.text+0x3cebaa>
1403cfb5a: 48 8d 84 24 f0 00 00 00     	leaq	0xf0(%rsp), %rax
1403cfb62: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
1403cfb67: 48 8d 84 24 00 01 00 00     	leaq	0x100(%rsp), %rax
1403cfb6f: 48 89 44 24 40              	movq	%rax, 0x40(%rsp)
1403cfb74: 48 8d 84 24 cc 00 00 00     	leaq	0xcc(%rsp), %rax
1403cfb7c: 48 89 44 24 48              	movq	%rax, 0x48(%rsp)
1403cfb81: 48 8d 84 24 10 01 00 00     	leaq	0x110(%rsp), %rax
1403cfb89: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
1403cfb8e: 48 8d 15 cd 1f 60 02        	leaq	0x2601fcd(%rip), %rdx   # 0x1429d1b62
1403cfb95: 48 8d 4c 24 68              	leaq	0x68(%rsp), %rcx
1403cfb9a: 4c 8d 4c 24 38              	leaq	0x38(%rsp), %r9
1403cfb9f: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1403cfba5: e8 96 36 13 00              	callq	0x140503240 <.text+0x502240>
1403cfbaa: 4c 8b bc 24 b8 01 00 00     	movq	0x1b8(%rsp), %r15
1403cfbb2: 80 7d 00 01                 	cmpb	$0x1, (%rbp)
1403cfbb6: 0f 85 9a 00 00 00           	jne	0x1403cfc56 <.text+0x3cec56>
1403cfbbc: 4d 8b 36                    	movq	(%r14), %r14
1403cfbbf: 41 8b 8e d8 00 00 00        	movl	0xd8(%r14), %ecx
1403cfbc6: f3 41 0f 10 8e dc 00 00 00  	movss	0xdc(%r14), %xmm1
1403cfbcf: e8 bc fe 56 00              	callq	0x14093fa90 <.text+0x93ea90>
1403cfbd4: 84 c0                       	testb	%al, %al
1403cfbd6: 0f 84 9c 00 00 00           	je	0x1403cfc78 <.text+0x3cec78>
1403cfbdc: 48 8b 84 24 f8 01 00 00     	movq	0x1f8(%rsp), %rax
1403cfbe4: 48 8b 08                    	movq	(%rax), %rcx
1403cfbe7: e8 c4 ea 55 00              	callq	0x14092e6b0 <.text+0x92d6b0>
1403cfbec: 84 c0                       	testb	%al, %al
1403cfbee: 74 6d                       	je	0x1403cfc5d <.text+0x3cec5d>
1403cfbf0: 48 89 74 24 60              	movq	%rsi, 0x60(%rsp)
1403cfbf5: e8 26 5f 72 00              	callq	0x140af5b20 <.text+0xaf4b20>
1403cfbfa: 89 c5                       	movl	%eax, %ebp
1403cfbfc: 41 8b 47 1c                 	movl	0x1c(%r15), %eax
1403cfc00: 49 8b 4f 10                 	movq	0x10(%r15), %rcx
1403cfc04: 89 01                       	movl	%eax, (%rcx)
1403cfc06: 4d 8b 3f                    	movq	(%r15), %r15
1403cfc09: 4d 8b 6f 30                 	movq	0x30(%r15), %r13
1403cfc0d: 49 8b 77 40                 	movq	0x40(%r15), %rsi
1403cfc11: 4d 3b 6f 20                 	cmpq	0x20(%r15), %r13
1403cfc15: 75 10                       	jne	0x1403cfc27 <.text+0x3cec27>
1403cfc17: 49 8d 4f 20                 	leaq	0x20(%r15), %rcx
1403cfc1b: 48 8d 15 ae d8 58 02        	leaq	0x258d8ae(%rip), %rdx   # 0x14295d4d0
1403cfc22: e8 09 2a 19 00              	callq	0x140562630 <.text+0x561630>
1403cfc27: 49 8b 47 28                 	movq	0x28(%r15), %rax
1403cfc2b: 4c 89 e9                    	movq	%r13, %rcx
1403cfc2e: 48 c1 e1 04                 	shlq	$0x4, %rcx
1403cfc32: 48 89 34 08                 	movq	%rsi, (%rax,%rcx)
1403cfc36: 89 6c 08 08                 	movl	%ebp, 0x8(%rax,%rcx)
1403cfc3a: 49 ff c5                    	incq	%r13
1403cfc3d: 4d 89 6f 30                 	movq	%r13, 0x30(%r15)
1403cfc41: 49 ff 47 40                 	incq	0x40(%r15)
1403cfc45: 31 ed                       	xorl	%ebp, %ebp
1403cfc47: 48 8b 74 24 60              	movq	0x60(%rsp), %rsi
1403cfc4c: 4c 8b bc 24 b8 01 00 00     	movq	0x1b8(%rsp), %r15
1403cfc54: eb 24                       	jmp	0x1403cfc7a <.text+0x3cec7a>
1403cfc56: 31 ed                       	xorl	%ebp, %ebp
1403cfc58: e9 b4 00 00 00              	jmp	0x1403cfd11 <.text+0x3ced11>
1403cfc5d: 48 89 d9                    	movq	%rbx, %rcx
1403cfc60: e8 7b ff 56 00              	callq	0x14093fbe0 <.text+0x93ebe0>
1403cfc65: 84 c0                       	testb	%al, %al
1403cfc67: 74 0a                       	je	0x1403cfc73 <.text+0x3cec73>
1403cfc69: c6 44 24 32 01              	movb	$0x1, 0x32(%rsp)
1403cfc6e: 40 b5 01                    	movb	$0x1, %bpl
1403cfc71: eb 07                       	jmp	0x1403cfc7a <.text+0x3cec7a>
1403cfc73: c6 44 24 33 01              	movb	$0x1, 0x33(%rsp)
1403cfc78: 31 ed                       	xorl	%ebp, %ebp
1403cfc7a: 41 8b 8e f8 00 00 00        	movl	0xf8(%r14), %ecx
1403cfc81: f3 41 0f 10 8e fc 00 00 00  	movss	0xfc(%r14), %xmm1
1403cfc8a: e8 01 fe 56 00              	callq	0x14093fa90 <.text+0x93ea90>
1403cfc8f: 84 c0                       	testb	%al, %al
1403cfc91: 74 7e                       	je	0x1403cfd11 <.text+0x3ced11>
1403cfc93: 83 3b 08                    	cmpl	$0x8, (%rbx)
1403cfc96: 74 79                       	je	0x1403cfd11 <.text+0x3ced11>
1403cfc98: 49 89 f5                    	movq	%rsi, %r13
1403cfc9b: 48 8b 94 24 c8 01 00 00     	movq	0x1c8(%rsp), %rdx
1403cfca3: 8b 42 1c                    	movl	0x1c(%rdx), %eax
1403cfca6: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
1403cfcaa: 89 01                       	movl	%eax, (%rcx)
1403cfcac: 4c 8b 32                    	movq	(%rdx), %r14
1403cfcaf: 4d 8b 7e 30                 	movq	0x30(%r14), %r15
1403cfcb3: 49 8b 76 40                 	movq	0x40(%r14), %rsi
1403cfcb7: 4d 3b 7e 20                 	cmpq	0x20(%r14), %r15
1403cfcbb: 75 10                       	jne	0x1403cfccd <.text+0x3ceccd>
1403cfcbd: 49 8d 4e 20                 	leaq	0x20(%r14), %rcx
1403cfcc1: 48 8d 15 60 34 62 02        	leaq	0x2623460(%rip), %rdx   # 0x1429f3128
1403cfcc8: e8 a3 2b 19 00              	callq	0x140562870 <.text+0x561870>
1403cfccd: 49 8b 46 28                 	movq	0x28(%r14), %rax
1403cfcd1: 4a 89 34 f8                 	movq	%rsi, (%rax,%r15,8)
1403cfcd5: 49 ff c7                    	incq	%r15
1403cfcd8: 4d 89 7e 30                 	movq	%r15, 0x30(%r14)
1403cfcdc: 49 ff 46 40                 	incq	0x40(%r14)
1403cfce0: 4c 89 ee                    	movq	%r13, %rsi
1403cfce3: 41 8b 45 1c                 	movl	0x1c(%r13), %eax
1403cfce7: 49 8b 4d 10                 	movq	0x10(%r13), %rcx
1403cfceb: 89 01                       	movl	%eax, (%rcx)
1403cfced: 49 8b 4d 00                 	movq	(%r13), %rcx
1403cfcf1: c6 44 24 70 01              	movb	$0x1, 0x70(%rsp)
1403cfcf6: 48 c7 44 24 68 08 00 00 00  	movq	$0x8, 0x68(%rsp)
1403cfcff: 48 8d 54 24 68              	leaq	0x68(%rsp), %rdx
1403cfd04: e8 c7 fd 56 00              	callq	0x14093fad0 <.text+0x93ead0>
1403cfd09: 4c 8b bc 24 b8 01 00 00     	movq	0x1b8(%rsp), %r15
1403cfd11: 48 89 d9                    	movq	%rbx, %rcx
1403cfd14: e8 c7 fe 56 00              	callq	0x14093fbe0 <.text+0x93ebe0>
1403cfd19: 84 c0                       	testb	%al, %al
1403cfd1b: 74 0c                       	je	0x1403cfd29 <.text+0x3ced29>
1403cfd1d: 41 80 7c 24 01 00           	cmpb	$0x0, 0x1(%r12)
1403cfd23: 0f 84 11 01 00 00           	je	0x1403cfe3a <.text+0x3cee3a>
1403cfd29: 83 3b 03                    	cmpl	$0x3, (%rbx)
1403cfd2c: 74 22                       	je	0x1403cfd50 <.text+0x3ced50>
1403cfd2e: 4c 8b ac 24 d0 01 00 00     	movq	0x1d0(%rsp), %r13
1403cfd36: 40 f6 c5 01                 	testb	$0x1, %bpl
1403cfd3a: 0f 85 0d 02 00 00           	jne	0x1403cff4d <.text+0x3cef4d>
1403cfd40: 80 7c 24 33 00              	cmpb	$0x0, 0x33(%rsp)
1403cfd45: 0f 85 66 04 00 00           	jne	0x1403d01b1 <.text+0x3cf1b1>
1403cfd4b: e9 d2 04 00 00              	jmp	0x1403d0222 <.text+0x3cf222>
1403cfd50: 8b 44 24 34                 	movl	0x34(%rsp), %eax
1403cfd54: 48 8b 4c 24 58              	movq	0x58(%rsp), %rcx
1403cfd59: 89 01                       	movl	%eax, (%rcx)
1403cfd5b: 48 8d 05 08 1e 60 02        	leaq	0x2601e08(%rip), %rax   # 0x1429d1b6a
1403cfd62: 48 89 44 24 68              	movq	%rax, 0x68(%rsp)
1403cfd67: 48 c7 44 24 70 04 00 00 00  	movq	$0x4, 0x70(%rsp)
1403cfd70: 48 8d 4c 24 68              	leaq	0x68(%rsp), %rcx
1403cfd75: e8 96 3e 2b 00              	callq	0x140683c10 <.text+0x682c10>
1403cfd7a: 49 89 c6                    	movq	%rax, %r14
1403cfd7d: 48 b8 43 b4 4f 00 20 53 69 b1       	movabsq	$-0x4e96acdfffb04bbd, %rax # imm = 0xB1695320004FB443
1403cfd87: 49 31 c6                    	xorq	%rax, %r14
1403cfd8a: 48 89 f9                    	movq	%rdi, %rcx
1403cfd8d: 4c 89 f2                    	movq	%r14, %rdx
1403cfd90: e8 2b 41 2b 00              	callq	0x140683ec0 <.text+0x682ec0>
1403cfd95: c6 40 5d 00                 	movb	$0x0, 0x5d(%rax)
1403cfd99: f6 c2 01                    	testb	$0x1, %dl
1403cfd9c: 75 39                       	jne	0x1403cfdd7 <.text+0x3cedd7>
1403cfd9e: 4d 89 fc                    	movq	%r15, %r12
1403cfda1: 4c 8b 7f 10                 	movq	0x10(%rdi), %r15
1403cfda5: 4c 3b 3f                    	cmpq	(%rdi), %r15
1403cfda8: 75 0f                       	jne	0x1403cfdb9 <.text+0x3cedb9>
1403cfdaa: 48 8d 15 8f f4 5e 02        	leaq	0x25ef48f(%rip), %rdx   # 0x1429bf240
1403cfdb1: 48 89 f9                    	movq	%rdi, %rcx
1403cfdb4: e8 17 1c 26 00              	callq	0x1406319d0 <.text+0x6309d0>
1403cfdb9: 48 8b 47 08                 	movq	0x8(%rdi), %rax
1403cfdbd: 49 6b cf 38                 	imulq	$0x38, %r15, %rcx
1403cfdc1: c7 04 08 02 00 00 00        	movl	$0x2, (%rax,%rcx)
1403cfdc8: 4c 89 74 08 30              	movq	%r14, 0x30(%rax,%rcx)
1403cfdcd: 49 ff c7                    	incq	%r15
1403cfdd0: 4c 89 7f 10                 	movq	%r15, 0x10(%rdi)
1403cfdd4: 4d 89 e7                    	movq	%r12, %r15
1403cfdd7: 48 89 7c 24 38              	movq	%rdi, 0x38(%rsp)
1403cfddc: 4c 89 74 24 40              	movq	%r14, 0x40(%rsp)
1403cfde1: 48 c7 44 24 48 00 00 00 00  	movq	$0x0, 0x48(%rsp)
1403cfdea: 4c 8d 05 62 1d 60 02        	leaq	0x2601d62(%rip), %r8    # 0x1429d1b53
1403cfdf1: 4c 8d 74 24 68              	leaq	0x68(%rsp), %r14
1403cfdf6: 48 8d 54 24 38              	leaq	0x38(%rsp), %rdx
1403cfdfb: 41 b9 09 00 00 00           	movl	$0x9, %r9d
1403cfe01: 4c 89 f1                    	movq	%r14, %rcx
1403cfe04: e8 d7 c0 2b 00              	callq	0x14068bee0 <.text+0x68aee0>
1403cfe09: 48 8d 15 7b 1d 60 02        	leaq	0x2601d7b(%rip), %rdx   # 0x1429d1b8b
1403cfe10: 4c 8d 4c 24 33              	leaq	0x33(%rsp), %r9
1403cfe15: 41 b8 06 00 00 00           	movl	$0x6, %r8d
1403cfe1b: 4c 89 f1                    	movq	%r14, %rcx
1403cfe1e: e8 8d 40 13 00              	callq	0x140503eb0 <.text+0x502eb0>
1403cfe23: 4c 8b ac 24 d0 01 00 00     	movq	0x1d0(%rsp), %r13
1403cfe2b: 40 f6 c5 01                 	testb	$0x1, %bpl
1403cfe2f: 0f 84 0b ff ff ff           	je	0x1403cfd40 <.text+0x3ced40>
1403cfe35: e9 13 01 00 00              	jmp	0x1403cff4d <.text+0x3cef4d>
1403cfe3a: 8b 44 24 34                 	movl	0x34(%rsp), %eax
1403cfe3e: 48 8b 4c 24 58              	movq	0x58(%rsp), %rcx
1403cfe43: 89 01                       	movl	%eax, (%rcx)
1403cfe45: 48 8d 05 1e 1d 60 02        	leaq	0x2601d1e(%rip), %rax   # 0x1429d1b6a
1403cfe4c: 48 89 44 24 68              	movq	%rax, 0x68(%rsp)
1403cfe51: 48 c7 44 24 70 04 00 00 00  	movq	$0x4, 0x70(%rsp)
1403cfe5a: 48 8d 4c 24 68              	leaq	0x68(%rsp), %rcx
1403cfe5f: e8 ac 3d 2b 00              	callq	0x140683c10 <.text+0x682c10>
1403cfe64: 49 89 c6                    	movq	%rax, %r14
1403cfe67: 48 b8 43 b4 4f 00 20 53 69 b1       	movabsq	$-0x4e96acdfffb04bbd, %rax # imm = 0xB1695320004FB443
1403cfe71: 49 31 c6                    	xorq	%rax, %r14
1403cfe74: 48 89 f9                    	movq	%rdi, %rcx
1403cfe77: 4c 89 f2                    	movq	%r14, %rdx
1403cfe7a: e8 41 40 2b 00              	callq	0x140683ec0 <.text+0x682ec0>
1403cfe7f: c6 40 5d 00                 	movb	$0x0, 0x5d(%rax)
1403cfe83: f6 c2 01                    	testb	$0x1, %dl
1403cfe86: 75 39                       	jne	0x1403cfec1 <.text+0x3ceec1>
1403cfe88: 4d 89 fc                    	movq	%r15, %r12
1403cfe8b: 4c 8b 7f 10                 	movq	0x10(%rdi), %r15
1403cfe8f: 4c 3b 3f                    	cmpq	(%rdi), %r15
1403cfe92: 75 0f                       	jne	0x1403cfea3 <.text+0x3ceea3>
1403cfe94: 48 8d 15 a5 f3 5e 02        	leaq	0x25ef3a5(%rip), %rdx   # 0x1429bf240
1403cfe9b: 48 89 f9                    	movq	%rdi, %rcx
1403cfe9e: e8 2d 1b 26 00              	callq	0x1406319d0 <.text+0x6309d0>
1403cfea3: 48 8b 47 08                 	movq	0x8(%rdi), %rax
1403cfea7: 49 6b cf 38                 	imulq	$0x38, %r15, %rcx
1403cfeab: c7 04 08 02 00 00 00        	movl	$0x2, (%rax,%rcx)
1403cfeb2: 4c 89 74 08 30              	movq	%r14, 0x30(%rax,%rcx)
1403cfeb7: 49 ff c7                    	incq	%r15
1403cfeba: 4c 89 7f 10                 	movq	%r15, 0x10(%rdi)
1403cfebe: 4d 89 e7                    	movq	%r12, %r15
1403cfec1: 48 89 7c 24 38              	movq	%rdi, 0x38(%rsp)
1403cfec6: 4c 89 74 24 40              	movq	%r14, 0x40(%rsp)
1403cfecb: 48 c7 44 24 48 00 00 00 00  	movq	$0x0, 0x48(%rsp)
1403cfed4: 4c 8d 05 93 1c 60 02        	leaq	0x2601c93(%rip), %r8    # 0x1429d1b6e
1403cfedb: 4c 8d 74 24 68              	leaq	0x68(%rsp), %r14
1403cfee0: 48 8d 54 24 38              	leaq	0x38(%rsp), %rdx
1403cfee5: 41 b9 16 00 00 00           	movl	$0x16, %r9d
1403cfeeb: 4c 89 f1                    	movq	%r14, %rcx
1403cfeee: e8 ed bf 2b 00              	callq	0x14068bee0 <.text+0x68aee0>
1403cfef3: 48 8d 84 24 20 01 00 00     	leaq	0x120(%rsp), %rax
1403cfefb: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
1403cff00: 48 8b 84 24 c8 01 00 00     	movq	0x1c8(%rsp), %rax
1403cff08: 48 89 44 24 40              	movq	%rax, 0x40(%rsp)
1403cff0d: 48 89 74 24 48              	movq	%rsi, 0x48(%rsp)
1403cff12: 48 8d 44 24 32              	leaq	0x32(%rsp), %rax
1403cff17: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
1403cff1c: 48 8d 15 61 1c 60 02        	leaq	0x2601c61(%rip), %rdx   # 0x1429d1b84
1403cff23: 4c 8d 4c 24 38              	leaq	0x38(%rsp), %r9
1403cff28: 41 b8 07 00 00 00           	movl	$0x7, %r8d
1403cff2e: 4c 89 f1                    	movq	%r14, %rcx
1403cff31: e8 aa 06 13 00              	callq	0x1405005e0 <.text+0x4ff5e0>
1403cff36: 0f b6 6c 24 32              	movzbl	0x32(%rsp), %ebp
1403cff3b: 4c 8b ac 24 d0 01 00 00     	movq	0x1d0(%rsp), %r13
1403cff43: 40 f6 c5 01                 	testb	$0x1, %bpl
1403cff47: 0f 84 f3 fd ff ff           	je	0x1403cfd40 <.text+0x3ced40>
1403cff4d: 48 89 5c 24 60              	movq	%rbx, 0x60(%rsp)
1403cff52: 4c 8b a4 24 c0 01 00 00     	movq	0x1c0(%rsp), %r12
1403cff5a: 8b 46 1c                    	movl	0x1c(%rsi), %eax
1403cff5d: 48 8b 4e 10                 	movq	0x10(%rsi), %rcx
1403cff61: 89 01                       	movl	%eax, (%rcx)
1403cff63: 48 8b 0e                    	movq	(%rsi), %rcx
1403cff66: 48 c7 44 24 68 03 00 00 00  	movq	$0x3, 0x68(%rsp)
1403cff6f: 48 8d 54 24 68              	leaq	0x68(%rsp), %rdx
1403cff74: e8 57 fb 56 00              	callq	0x14093fad0 <.text+0x93ead0>
1403cff79: 48 8b 84 24 e8 00 00 00     	movq	0xe8(%rsp), %rax
1403cff81: 8b 00                       	movl	(%rax), %eax
1403cff83: 83 f8 13                    	cmpl	$0x13, %eax
1403cff86: 0f 84 f3 00 00 00           	je	0x1403d007f <.text+0x3cf07f>
1403cff8c: 4c 89 eb                    	movq	%r13, %rbx
1403cff8f: 83 f8 0d                    	cmpl	$0xd, %eax
1403cff92: 0f 85 80 00 00 00           	jne	0x1403d0018 <.text+0x3cf018>
1403cff98: 49 89 f6                    	movq	%rsi, %r14
1403cff9b: 41 8b 47 1c                 	movl	0x1c(%r15), %eax
1403cff9f: 49 8b 4f 10                 	movq	0x10(%r15), %rcx
1403cffa3: 89 01                       	movl	%eax, (%rcx)
1403cffa5: 4d 8b 3f                    	movq	(%r15), %r15
1403cffa8: 4d 8b 6f 30                 	movq	0x30(%r15), %r13
1403cffac: 49 8b 77 40                 	movq	0x40(%r15), %rsi
1403cffb0: 4d 3b 6f 20                 	cmpq	0x20(%r15), %r13
1403cffb4: 75 10                       	jne	0x1403cffc6 <.text+0x3cefc6>
1403cffb6: 49 8d 4f 20                 	leaq	0x20(%r15), %rcx
1403cffba: 48 8d 15 0f d5 58 02        	leaq	0x258d50f(%rip), %rdx   # 0x14295d4d0
1403cffc1: e8 6a 26 19 00              	callq	0x140562630 <.text+0x561630>
1403cffc6: 49 8b 47 28                 	movq	0x28(%r15), %rax
1403cffca: 4c 89 e9                    	movq	%r13, %rcx
1403cffcd: 48 c1 e1 04                 	shlq	$0x4, %rcx
1403cffd1: 48 89 34 08                 	movq	%rsi, (%rax,%rcx)
1403cffd5: c7 44 08 08 00 03 00 03     	movl	$0x3000300, 0x8(%rax,%rcx) # imm = 0x3000300
1403cffdd: 49 ff c5                    	incq	%r13
1403cffe0: 4d 89 6f 30                 	movq	%r13, 0x30(%r15)
1403cffe4: 49 ff 47 40                 	incq	0x40(%r15)
1403cffe8: 48 8d 4c 24 68              	leaq	0x68(%rsp), %rcx
1403cffed: e8 3e 6b 6b 00              	callq	0x140a86b30 <.text+0xa85b30>
1403cfff2: 48 8b 84 24 d8 00 00 00     	movq	0xd8(%rsp), %rax
1403cfffa: 8b 8c 24 c8 00 00 00        	movl	0xc8(%rsp), %ecx
1403d0001: 89 08                       	movl	%ecx, (%rax)
1403d0003: 0f 10 44 24 68              	movups	0x68(%rsp), %xmm0
1403d0008: 48 8b 84 24 e0 00 00 00     	movq	0xe0(%rsp), %rax
1403d0010: 0f 11 00                    	movups	%xmm0, (%rax)
1403d0013: 4c 89 f6                    	movq	%r14, %rsi
1403d0016: eb 64                       	jmp	0x1403d007c <.text+0x3cf07c>
1403d0018: 48 8d 4c 24 68              	leaq	0x68(%rsp), %rcx
1403d001d: e8 0e 6b 6b 00              	callq	0x140a86b30 <.text+0xa85b30>
1403d0022: 48 8b 94 24 f0 01 00 00     	movq	0x1f0(%rsp), %rdx
1403d002a: 8b 42 1c                    	movl	0x1c(%rdx), %eax
1403d002d: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
1403d0031: 89 01                       	movl	%eax, (%rcx)
1403d0033: 4c 8b 3a                    	movq	(%rdx), %r15
1403d0036: 4d 8b 6f 30                 	movq	0x30(%r15), %r13
1403d003a: 49 8b 6f 40                 	movq	0x40(%r15), %rbp
1403d003e: 4d 3b 6f 20                 	cmpq	0x20(%r15), %r13
1403d0042: 75 10                       	jne	0x1403d0054 <.text+0x3cf054>
1403d0044: 49 8d 4f 20                 	leaq	0x20(%r15), %rcx
1403d0048: 48 8d 15 81 d4 58 02        	leaq	0x258d481(%rip), %rdx   # 0x14295d4d0
1403d004f: e8 5c 2d 19 00              	callq	0x140562db0 <.text+0x561db0>
1403d0054: 49 8b 47 28                 	movq	0x28(%r15), %rax
1403d0058: 4a 8d 0c 6d 00 00 00 00     	leaq	(,%r13,2), %rcx
1403d0060: 4c 01 e9                    	addq	%r13, %rcx
1403d0063: 0f 10 44 24 68              	movups	0x68(%rsp), %xmm0
1403d0068: 0f 11 04 c8                 	movups	%xmm0, (%rax,%rcx,8)
1403d006c: 48 89 6c c8 10              	movq	%rbp, 0x10(%rax,%rcx,8)
1403d0071: 49 ff c5                    	incq	%r13
1403d0074: 4d 89 6f 30                 	movq	%r13, 0x30(%r15)
1403d0078: 49 ff 47 40                 	incq	0x40(%r15)
1403d007c: 49 89 dd                    	movq	%rbx, %r13
1403d007f: 49 89 f6                    	movq	%rsi, %r14
1403d0082: 41 8b 44 24 1c              	movl	0x1c(%r12), %eax
1403d0087: 49 8b 4c 24 10              	movq	0x10(%r12), %rcx
1403d008c: 89 01                       	movl	%eax, (%rcx)
1403d008e: 4d 8b 3c 24                 	movq	(%r12), %r15
1403d0092: 4d 8b 67 30                 	movq	0x30(%r15), %r12
1403d0096: 49 8b 77 40                 	movq	0x40(%r15), %rsi
1403d009a: 4d 3b 67 20                 	cmpq	0x20(%r15), %r12
1403d009e: 75 10                       	jne	0x1403d00b0 <.text+0x3cf0b0>
1403d00a0: 49 8d 4f 20                 	leaq	0x20(%r15), %rcx
1403d00a4: 48 8d 15 25 d4 58 02        	leaq	0x258d425(%rip), %rdx   # 0x14295d4d0
1403d00ab: e8 c0 27 19 00              	callq	0x140562870 <.text+0x561870>
1403d00b0: 49 8b 47 28                 	movq	0x28(%r15), %rax
1403d00b4: 4a 89 34 e0                 	movq	%rsi, (%rax,%r12,8)
1403d00b8: 49 ff c4                    	incq	%r12
1403d00bb: 4d 89 67 30                 	movq	%r12, 0x30(%r15)
1403d00bf: 49 ff 47 40                 	incq	0x40(%r15)
1403d00c3: 41 8b 45 1c                 	movl	0x1c(%r13), %eax
1403d00c7: 49 8b 4d 10                 	movq	0x10(%r13), %rcx
1403d00cb: 89 01                       	movl	%eax, (%rcx)
1403d00cd: 4c 89 eb                    	movq	%r13, %rbx
1403d00d0: 4d 8b 6d 00                 	movq	(%r13), %r13
1403d00d4: 4d 8b 7d 30                 	movq	0x30(%r13), %r15
1403d00d8: 49 8b 6d 40                 	movq	0x40(%r13), %rbp
1403d00dc: 4d 3b 7d 20                 	cmpq	0x20(%r13), %r15
1403d00e0: 75 10                       	jne	0x1403d00f2 <.text+0x3cf0f2>
1403d00e2: 49 8d 4d 20                 	leaq	0x20(%r13), %rcx
1403d00e6: 48 8d 15 e3 d3 58 02        	leaq	0x258d3e3(%rip), %rdx   # 0x14295d4d0
1403d00ed: e8 3e 25 19 00              	callq	0x140562630 <.text+0x561630>
1403d00f2: 4c 8b a4 24 e0 01 00 00     	movq	0x1e0(%rsp), %r12
1403d00fa: 49 8b 45 28                 	movq	0x28(%r13), %rax
1403d00fe: 4c 89 f9                    	movq	%r15, %rcx
1403d0101: 48 c1 e1 04                 	shlq	$0x4, %rcx
1403d0105: 48 89 2c 08                 	movq	%rbp, (%rax,%rcx)
1403d0109: c6 44 08 08 0a              	movb	$0xa, 0x8(%rax,%rcx)
1403d010e: 49 ff c7                    	incq	%r15
1403d0111: 4d 89 7d 30                 	movq	%r15, 0x30(%r13)
1403d0115: 49 ff 45 40                 	incq	0x40(%r13)
1403d0119: 48 8b 94 24 c8 01 00 00     	movq	0x1c8(%rsp), %rdx
1403d0121: 8b 42 1c                    	movl	0x1c(%rdx), %eax
1403d0124: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
1403d0128: 89 01                       	movl	%eax, (%rcx)
1403d012a: 4c 8b 3a                    	movq	(%rdx), %r15
1403d012d: 4d 8b 6f 30                 	movq	0x30(%r15), %r13
1403d0131: 49 8b 77 40                 	movq	0x40(%r15), %rsi
1403d0135: 4d 3b 6f 20                 	cmpq	0x20(%r15), %r13
1403d0139: 75 10                       	jne	0x1403d014b <.text+0x3cf14b>
1403d013b: 49 8d 4f 20                 	leaq	0x20(%r15), %rcx
1403d013f: 48 8d 15 e2 2f 62 02        	leaq	0x2622fe2(%rip), %rdx   # 0x1429f3128
1403d0146: e8 25 27 19 00              	callq	0x140562870 <.text+0x561870>
1403d014b: 49 8b 47 28                 	movq	0x28(%r15), %rax
1403d014f: 4a 89 34 e8                 	movq	%rsi, (%rax,%r13,8)
1403d0153: 49 ff c5                    	incq	%r13
1403d0156: 4d 89 6f 30                 	movq	%r13, 0x30(%r15)
1403d015a: 49 ff 47 40                 	incq	0x40(%r15)
1403d015e: 41 8b 44 24 1c              	movl	0x1c(%r12), %eax
1403d0163: 49 8b 4c 24 10              	movq	0x10(%r12), %rcx
1403d0168: 89 01                       	movl	%eax, (%rcx)
1403d016a: 4d 8b 3c 24                 	movq	(%r12), %r15
1403d016e: 4d 8b 67 30                 	movq	0x30(%r15), %r12
1403d0172: 49 8b 77 40                 	movq	0x40(%r15), %rsi
1403d0176: 4d 3b 67 20                 	cmpq	0x20(%r15), %r12
1403d017a: 75 10                       	jne	0x1403d018c <.text+0x3cf18c>
1403d017c: 49 8d 4f 20                 	leaq	0x20(%r15), %rcx
1403d0180: 48 8d 15 49 d3 58 02        	leaq	0x258d349(%rip), %rdx   # 0x14295d4d0
1403d0187: e8 e4 26 19 00              	callq	0x140562870 <.text+0x561870>
1403d018c: 49 8b 47 28                 	movq	0x28(%r15), %rax
1403d0190: 4a 89 34 e0                 	movq	%rsi, (%rax,%r12,8)
1403d0194: 49 ff c4                    	incq	%r12
1403d0197: 4d 89 67 30                 	movq	%r12, 0x30(%r15)
1403d019b: 49 ff 47 40                 	incq	0x40(%r15)
1403d019f: 4c 89 f6                    	movq	%r14, %rsi
1403d01a2: 49 89 dd                    	movq	%rbx, %r13
1403d01a5: 48 8b 5c 24 60              	movq	0x60(%rsp), %rbx
1403d01aa: 80 7c 24 33 00              	cmpb	$0x0, 0x33(%rsp)
1403d01af: 74 71                       	je	0x1403d0222 <.text+0x3cf222>
1403d01b1: 49 89 f6                    	movq	%rsi, %r14
1403d01b4: 41 8b 45 1c                 	movl	0x1c(%r13), %eax
1403d01b8: 49 8b 4d 10                 	movq	0x10(%r13), %rcx
1403d01bc: 89 01                       	movl	%eax, (%rcx)
1403d01be: 4d 8b 7d 00                 	movq	(%r13), %r15
1403d01c2: 4d 8b 67 30                 	movq	0x30(%r15), %r12
1403d01c6: 49 8b 77 40                 	movq	0x40(%r15), %rsi
1403d01ca: 4d 3b 67 20                 	cmpq	0x20(%r15), %r12
1403d01ce: 75 10                       	jne	0x1403d01e0 <.text+0x3cf1e0>
1403d01d0: 49 8d 4f 20                 	leaq	0x20(%r15), %rcx
1403d01d4: 48 8d 15 f5 d2 58 02        	leaq	0x258d2f5(%rip), %rdx   # 0x14295d4d0
1403d01db: e8 50 24 19 00              	callq	0x140562630 <.text+0x561630>
1403d01e0: 49 8b 47 28                 	movq	0x28(%r15), %rax
1403d01e4: 4c 89 e1                    	movq	%r12, %rcx
1403d01e7: 48 c1 e1 04                 	shlq	$0x4, %rcx
1403d01eb: 48 89 34 08                 	movq	%rsi, (%rax,%rcx)
1403d01ef: c6 44 08 08 0b              	movb	$0xb, 0x8(%rax,%rcx)
1403d01f4: 49 ff c4                    	incq	%r12
1403d01f7: 4d 89 67 30                 	movq	%r12, 0x30(%r15)
1403d01fb: 49 ff 47 40                 	incq	0x40(%r15)
1403d01ff: 4c 89 f6                    	movq	%r14, %rsi
1403d0202: 41 8b 46 1c                 	movl	0x1c(%r14), %eax
1403d0206: 49 8b 4e 10                 	movq	0x10(%r14), %rcx
1403d020a: 89 01                       	movl	%eax, (%rcx)
1403d020c: 49 8b 0e                    	movq	(%r14), %rcx
1403d020f: 48 c7 44 24 68 02 00 00 00  	movq	$0x2, 0x68(%rsp)
1403d0218: 48 8d 54 24 68              	leaq	0x68(%rsp), %rdx
1403d021d: e8 ae f8 56 00              	callq	0x14093fad0 <.text+0x93ead0>
1403d0222: 4c 8b a4 24 00 02 00 00     	movq	0x200(%rsp), %r12
1403d022a: 4c 8b bc 24 d8 01 00 00     	movq	0x1d8(%rsp), %r15
1403d0232: 48 8b 03                    	movq	(%rbx), %rax
1403d0235: 48 83 f8 03                 	cmpq	$0x3, %rax
1403d0239: 0f 84 6d 01 00 00           	je	0x1403d03ac <.text+0x3cf3ac>
1403d023f: 83 f8 04                    	cmpl	$0x4, %eax
1403d0242: 0f 85 85 05 00 00           	jne	0x1403d07cd <.text+0x3cf7cd>
1403d0248: 4c 8b b4 24 e8 01 00 00     	movq	0x1e8(%rsp), %r14
1403d0250: 48 8b 05 41 be 25 03        	movq	0x325be41(%rip), %rax   # 0x14362c098
1403d0257: 80 38 00                    	cmpb	$0x0, (%rax)
1403d025a: 0f 84 76 02 00 00           	je	0x1403d04d6 <.text+0x3cf4d6>
1403d0260: 8b 44 24 34                 	movl	0x34(%rsp), %eax
1403d0264: 48 8b 4c 24 58              	movq	0x58(%rsp), %rcx
1403d0269: 89 01                       	movl	%eax, (%rcx)
1403d026b: 48 8d 05 5e 19 60 02        	leaq	0x260195e(%rip), %rax   # 0x1429d1bd0
1403d0272: 48 89 44 24 68              	movq	%rax, 0x68(%rsp)
1403d0277: 48 c7 44 24 70 0a 00 00 00  	movq	$0xa, 0x70(%rsp)
1403d0280: 48 8d 4c 24 68              	leaq	0x68(%rsp), %rcx
1403d0285: e8 86 39 2b 00              	callq	0x140683c10 <.text+0x682c10>
1403d028a: 48 89 c3                    	movq	%rax, %rbx
1403d028d: 48 b8 43 b4 4f 00 20 53 69 b1       	movabsq	$-0x4e96acdfffb04bbd, %rax # imm = 0xB1695320004FB443
1403d0297: 48 31 c3                    	xorq	%rax, %rbx
1403d029a: 48 89 f9                    	movq	%rdi, %rcx
1403d029d: 48 89 da                    	movq	%rbx, %rdx
1403d02a0: e8 1b 3c 2b 00              	callq	0x140683ec0 <.text+0x682ec0>
1403d02a5: c6 40 5d 00                 	movb	$0x0, 0x5d(%rax)
1403d02a9: c6 80 b7 00 00 00 01        	movb	$0x1, 0xb7(%rax)
1403d02b0: f6 c2 01                    	testb	$0x1, %dl
1403d02b3: 75 33                       	jne	0x1403d02e8 <.text+0x3cf2e8>
1403d02b5: 4c 8b 6f 10                 	movq	0x10(%rdi), %r13
1403d02b9: 4c 3b 2f                    	cmpq	(%rdi), %r13
1403d02bc: 75 0f                       	jne	0x1403d02cd <.text+0x3cf2cd>
1403d02be: 48 8d 15 7b ef 5e 02        	leaq	0x25eef7b(%rip), %rdx   # 0x1429bf240
1403d02c5: 48 89 f9                    	movq	%rdi, %rcx
1403d02c8: e8 03 17 26 00              	callq	0x1406319d0 <.text+0x6309d0>
1403d02cd: 48 8b 47 08                 	movq	0x8(%rdi), %rax
1403d02d1: 49 6b cd 38                 	imulq	$0x38, %r13, %rcx
1403d02d5: c7 04 08 01 00 00 00        	movl	$0x1, (%rax,%rcx)
1403d02dc: 48 89 5c 08 30              	movq	%rbx, 0x30(%rax,%rcx)
1403d02e1: 49 ff c5                    	incq	%r13
1403d02e4: 4c 89 6f 10                 	movq	%r13, 0x10(%rdi)
1403d02e8: 48 89 7c 24 38              	movq	%rdi, 0x38(%rsp)
1403d02ed: 48 89 5c 24 40              	movq	%rbx, 0x40(%rsp)
1403d02f2: 48 c7 44 24 48 00 00 00 00  	movq	$0x0, 0x48(%rsp)
1403d02fb: 4c 8d 05 fc 18 60 02        	leaq	0x26018fc(%rip), %r8    # 0x1429d1bfe
1403d0302: 48 8d 7c 24 68              	leaq	0x68(%rsp), %rdi
1403d0307: 48 8d 54 24 38              	leaq	0x38(%rsp), %rdx
1403d030c: 41 b9 0e 00 00 00           	movl	$0xe, %r9d
1403d0312: 48 89 f9                    	movq	%rdi, %rcx
1403d0315: e8 c6 bb 2b 00              	callq	0x14068bee0 <.text+0x68aee0>
1403d031a: 48 8d 15 c1 18 60 02        	leaq	0x26018c1(%rip), %rdx   # 0x1429d1be2
1403d0321: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1403d0327: 48 89 f9                    	movq	%rdi, %rcx
1403d032a: e8 81 fb 12 00              	callq	0x1404ffeb0 <.text+0x4feeb0>
1403d032f: 48 8b 84 24 d0 00 00 00     	movq	0xd0(%rsp), %rax
1403d0337: f3 0f 10 80 f0 01 00 00     	movss	0x1f0(%rax), %xmm0
1403d033f: 0f 2e 05 82 15 60 02        	ucomiss	0x2601582(%rip), %xmm0  # 0x1429d18c8
1403d0346: 77 0b                       	ja	0x1403d0353 <.text+0x3cf353>
1403d0348: 49 8b 04 24                 	movq	(%r12), %rax
1403d034c: 48 83 78 10 02              	cmpq	$0x2, 0x10(%rax)
1403d0351: 76 1a                       	jbe	0x1403d036d <.text+0x3cf36d>
1403d0353: 48 8d 15 5a 18 60 02        	leaq	0x260185a(%rip), %rdx   # 0x1429d1bb4
1403d035a: 48 8d 4c 24 68              	leaq	0x68(%rsp), %rcx
1403d035f: 41 b8 07 00 00 00           	movl	$0x7, %r8d
1403d0365: 4d 89 f9                    	movq	%r15, %r9
1403d0368: e8 e3 35 13 00              	callq	0x140503950 <.text+0x502950>
1403d036d: 48 8d 15 47 18 60 02        	leaq	0x2601847(%rip), %rdx   # 0x1429d1bbb
1403d0374: 48 8d 7c 24 68              	leaq	0x68(%rsp), %rdi
1403d0379: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1403d037f: 48 89 f9                    	movq	%rdi, %rcx
1403d0382: 4d 89 f9                    	movq	%r15, %r9
1403d0385: e8 96 06 13 00              	callq	0x140500a20 <.text+0x4ffa20>
1403d038a: 4c 89 74 24 20              	movq	%r14, 0x20(%rsp)
1403d038f: 48 8d 15 ee 17 60 02        	leaq	0x26017ee(%rip), %rdx   # 0x1429d1b84
1403d0396: 41 b8 07 00 00 00           	movl	$0x7, %r8d
1403d039c: 48 89 f9                    	movq	%rdi, %rcx
1403d039f: 49 89 f1                    	movq	%rsi, %r9
1403d03a2: e8 f9 24 13 00              	callq	0x1405028a0 <.text+0x5018a0>
1403d03a7: e9 21 04 00 00              	jmp	0x1403d07cd <.text+0x3cf7cd>
1403d03ac: 48 8b 05 e5 bc 25 03        	movq	0x325bce5(%rip), %rax   # 0x14362c098
1403d03b3: 80 38 00                    	cmpb	$0x0, (%rax)
1403d03b6: 0f 84 0b 02 00 00           	je	0x1403d05c7 <.text+0x3cf5c7>
1403d03bc: 8b 44 24 34                 	movl	0x34(%rsp), %eax
1403d03c0: 48 8b 4c 24 58              	movq	0x58(%rsp), %rcx
1403d03c5: 89 01                       	movl	%eax, (%rcx)
1403d03c7: 48 8d 05 ed 17 60 02        	leaq	0x26017ed(%rip), %rax   # 0x1429d1bbb
1403d03ce: 48 89 44 24 68              	movq	%rax, 0x68(%rsp)
1403d03d3: 48 c7 44 24 70 08 00 00 00  	movq	$0x8, 0x70(%rsp)
1403d03dc: 48 8d 4c 24 68              	leaq	0x68(%rsp), %rcx
1403d03e1: e8 2a 38 2b 00              	callq	0x140683c10 <.text+0x682c10>
1403d03e6: 48 89 c3                    	movq	%rax, %rbx
1403d03e9: 48 b8 43 b4 4f 00 20 53 69 b1       	movabsq	$-0x4e96acdfffb04bbd, %rax # imm = 0xB1695320004FB443
1403d03f3: 48 31 c3                    	xorq	%rax, %rbx
1403d03f6: 48 89 f9                    	movq	%rdi, %rcx
1403d03f9: 48 89 da                    	movq	%rbx, %rdx
1403d03fc: e8 bf 3a 2b 00              	callq	0x140683ec0 <.text+0x682ec0>
1403d0401: c6 40 5d 00                 	movb	$0x0, 0x5d(%rax)
1403d0405: c6 80 b7 00 00 00 01        	movb	$0x1, 0xb7(%rax)
1403d040c: f6 c2 01                    	testb	$0x1, %dl
1403d040f: 75 39                       	jne	0x1403d044a <.text+0x3cf44a>
1403d0411: 4d 89 ee                    	movq	%r13, %r14
1403d0414: 4c 8b 6f 10                 	movq	0x10(%rdi), %r13
1403d0418: 4c 3b 2f                    	cmpq	(%rdi), %r13
1403d041b: 75 0f                       	jne	0x1403d042c <.text+0x3cf42c>
1403d041d: 48 8d 15 1c ee 5e 02        	leaq	0x25eee1c(%rip), %rdx   # 0x1429bf240
1403d0424: 48 89 f9                    	movq	%rdi, %rcx
1403d0427: e8 a4 15 26 00              	callq	0x1406319d0 <.text+0x6309d0>
1403d042c: 48 8b 47 08                 	movq	0x8(%rdi), %rax
1403d0430: 49 6b cd 38                 	imulq	$0x38, %r13, %rcx
1403d0434: c7 04 08 02 00 00 00        	movl	$0x2, (%rax,%rcx)
1403d043b: 48 89 5c 08 30              	movq	%rbx, 0x30(%rax,%rcx)
1403d0440: 49 ff c5                    	incq	%r13
1403d0443: 4c 89 6f 10                 	movq	%r13, 0x10(%rdi)
1403d0447: 4d 89 f5                    	movq	%r14, %r13
1403d044a: 48 89 7c 24 38              	movq	%rdi, 0x38(%rsp)
1403d044f: 48 89 5c 24 40              	movq	%rbx, 0x40(%rsp)
1403d0454: 48 c7 44 24 48 00 00 00 00  	movq	$0x0, 0x48(%rsp)
1403d045d: 4c 8d 05 5f 17 60 02        	leaq	0x260175f(%rip), %r8    # 0x1429d1bc3
1403d0464: 48 8d 4c 24 68              	leaq	0x68(%rsp), %rcx
1403d0469: 48 8d 54 24 38              	leaq	0x38(%rsp), %rdx
1403d046e: 41 b9 0d 00 00 00           	movl	$0xd, %r9d
1403d0474: e8 67 ba 2b 00              	callq	0x14068bee0 <.text+0x68aee0>
1403d0479: 48 8b 84 24 d0 00 00 00     	movq	0xd0(%rsp), %rax
1403d0481: f3 0f 10 80 f0 01 00 00     	movss	0x1f0(%rax), %xmm0
1403d0489: 0f 2e 05 38 14 60 02        	ucomiss	0x2601438(%rip), %xmm0  # 0x1429d18c8
1403d0490: 77 0b                       	ja	0x1403d049d <.text+0x3cf49d>
1403d0492: 49 8b 04 24                 	movq	(%r12), %rax
1403d0496: 48 83 78 10 02              	cmpq	$0x2, 0x10(%rax)
1403d049b: 76 1a                       	jbe	0x1403d04b7 <.text+0x3cf4b7>
1403d049d: 48 8d 15 10 17 60 02        	leaq	0x2601710(%rip), %rdx   # 0x1429d1bb4
1403d04a4: 48 8d 4c 24 68              	leaq	0x68(%rsp), %rcx
1403d04a9: 41 b8 07 00 00 00           	movl	$0x7, %r8d
1403d04af: 4d 89 f9                    	movq	%r15, %r9
1403d04b2: e8 a9 2f 13 00              	callq	0x140503460 <.text+0x502460>
1403d04b7: 48 8d 15 fd 16 60 02        	leaq	0x26016fd(%rip), %rdx   # 0x1429d1bbb
1403d04be: 48 8d 4c 24 68              	leaq	0x68(%rsp), %rcx
1403d04c3: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1403d04c9: 4d 89 f9                    	movq	%r15, %r9
1403d04cc: e8 6f 11 13 00              	callq	0x140501640 <.text+0x500640>
1403d04d1: e9 db 01 00 00              	jmp	0x1403d06b1 <.text+0x3cf6b1>
1403d04d6: 8b 44 24 34                 	movl	0x34(%rsp), %eax
1403d04da: 48 8b 4c 24 58              	movq	0x58(%rsp), %rcx
1403d04df: 89 01                       	movl	%eax, (%rcx)
1403d04e1: 48 8d 05 e8 16 60 02        	leaq	0x26016e8(%rip), %rax   # 0x1429d1bd0
1403d04e8: 48 89 44 24 68              	movq	%rax, 0x68(%rsp)
1403d04ed: 48 c7 44 24 70 0a 00 00 00  	movq	$0xa, 0x70(%rsp)
1403d04f6: 48 8d 4c 24 68              	leaq	0x68(%rsp), %rcx
1403d04fb: e8 10 37 2b 00              	callq	0x140683c10 <.text+0x682c10>
1403d0500: 48 89 c3                    	movq	%rax, %rbx
1403d0503: 48 b8 43 b4 4f 00 20 53 69 b1       	movabsq	$-0x4e96acdfffb04bbd, %rax # imm = 0xB1695320004FB443
1403d050d: 48 31 c3                    	xorq	%rax, %rbx
1403d0510: 48 89 f9                    	movq	%rdi, %rcx
1403d0513: 48 89 da                    	movq	%rbx, %rdx
1403d0516: e8 a5 39 2b 00              	callq	0x140683ec0 <.text+0x682ec0>
1403d051b: c6 40 5d 00                 	movb	$0x0, 0x5d(%rax)
1403d051f: c6 80 b7 00 00 00 01        	movb	$0x1, 0xb7(%rax)
1403d0526: f6 c2 01                    	testb	$0x1, %dl
1403d0529: 75 33                       	jne	0x1403d055e <.text+0x3cf55e>
1403d052b: 4c 8b 7f 10                 	movq	0x10(%rdi), %r15
1403d052f: 4c 3b 3f                    	cmpq	(%rdi), %r15
1403d0532: 75 0f                       	jne	0x1403d0543 <.text+0x3cf543>
1403d0534: 48 8d 15 05 ed 5e 02        	leaq	0x25eed05(%rip), %rdx   # 0x1429bf240
1403d053b: 48 89 f9                    	movq	%rdi, %rcx
1403d053e: e8 8d 14 26 00              	callq	0x1406319d0 <.text+0x6309d0>
1403d0543: 48 8b 47 08                 	movq	0x8(%rdi), %rax
1403d0547: 49 6b cf 38                 	imulq	$0x38, %r15, %rcx
1403d054b: c7 04 08 01 00 00 00        	movl	$0x1, (%rax,%rcx)
1403d0552: 48 89 5c 08 30              	movq	%rbx, 0x30(%rax,%rcx)
1403d0557: 49 ff c7                    	incq	%r15
1403d055a: 4c 89 7f 10                 	movq	%r15, 0x10(%rdi)
1403d055e: 48 89 7c 24 38              	movq	%rdi, 0x38(%rsp)
1403d0563: 48 89 5c 24 40              	movq	%rbx, 0x40(%rsp)
1403d0568: 48 c7 44 24 48 00 00 00 00  	movq	$0x0, 0x48(%rsp)
1403d0571: 4c 8d 05 7d 16 60 02        	leaq	0x260167d(%rip), %r8    # 0x1429d1bf5
1403d0578: 48 8d 7c 24 68              	leaq	0x68(%rsp), %rdi
1403d057d: 48 8d 54 24 38              	leaq	0x38(%rsp), %rdx
1403d0582: 41 b9 09 00 00 00           	movl	$0x9, %r9d
1403d0588: 48 89 f9                    	movq	%rdi, %rcx
1403d058b: e8 50 b9 2b 00              	callq	0x14068bee0 <.text+0x68aee0>
1403d0590: 48 8d 15 4b 16 60 02        	leaq	0x260164b(%rip), %rdx   # 0x1429d1be2
1403d0597: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1403d059d: 48 89 f9                    	movq	%rdi, %rcx
1403d05a0: e8 3b f2 12 00              	callq	0x1404ff7e0 <.text+0x4fe7e0>
1403d05a5: 4c 89 74 24 20              	movq	%r14, 0x20(%rsp)
1403d05aa: 48 8d 15 d3 15 60 02        	leaq	0x26015d3(%rip), %rdx   # 0x1429d1b84
1403d05b1: 41 b8 07 00 00 00           	movl	$0x7, %r8d
1403d05b7: 48 89 f9                    	movq	%rdi, %rcx
1403d05ba: 49 89 f1                    	movq	%rsi, %r9
1403d05bd: e8 de 22 13 00              	callq	0x1405028a0 <.text+0x5018a0>
1403d05c2: e9 06 02 00 00              	jmp	0x1403d07cd <.text+0x3cf7cd>
1403d05c7: 8b 44 24 34                 	movl	0x34(%rsp), %eax
1403d05cb: 48 8b 4c 24 58              	movq	0x58(%rsp), %rcx
1403d05d0: 89 01                       	movl	%eax, (%rcx)
1403d05d2: 48 8d 05 b8 15 60 02        	leaq	0x26015b8(%rip), %rax   # 0x1429d1b91
1403d05d9: 48 89 44 24 68              	movq	%rax, 0x68(%rsp)
1403d05de: 48 c7 44 24 70 0d 00 00 00  	movq	$0xd, 0x70(%rsp)
1403d05e7: 48 8d 4c 24 68              	leaq	0x68(%rsp), %rcx
1403d05ec: e8 1f 36 2b 00              	callq	0x140683c10 <.text+0x682c10>
1403d05f1: 48 89 c3                    	movq	%rax, %rbx
1403d05f4: 48 b8 43 b4 4f 00 20 53 69 b1       	movabsq	$-0x4e96acdfffb04bbd, %rax # imm = 0xB1695320004FB443
1403d05fe: 48 31 c3                    	xorq	%rax, %rbx
1403d0601: 48 89 f9                    	movq	%rdi, %rcx
1403d0604: 48 89 da                    	movq	%rbx, %rdx
1403d0607: e8 b4 38 2b 00              	callq	0x140683ec0 <.text+0x682ec0>
1403d060c: c6 40 5d 00                 	movb	$0x0, 0x5d(%rax)
1403d0610: c6 80 b7 00 00 00 01        	movb	$0x1, 0xb7(%rax)
1403d0617: f6 c2 01                    	testb	$0x1, %dl
1403d061a: 75 33                       	jne	0x1403d064f <.text+0x3cf64f>
1403d061c: 4c 8b 67 10                 	movq	0x10(%rdi), %r12
1403d0620: 4c 3b 27                    	cmpq	(%rdi), %r12
1403d0623: 75 0f                       	jne	0x1403d0634 <.text+0x3cf634>
1403d0625: 48 8d 15 14 ec 5e 02        	leaq	0x25eec14(%rip), %rdx   # 0x1429bf240
1403d062c: 48 89 f9                    	movq	%rdi, %rcx
1403d062f: e8 9c 13 26 00              	callq	0x1406319d0 <.text+0x6309d0>
1403d0634: 48 8b 47 08                 	movq	0x8(%rdi), %rax
1403d0638: 49 6b cc 38                 	imulq	$0x38, %r12, %rcx
1403d063c: c7 04 08 02 00 00 00        	movl	$0x2, (%rax,%rcx)
1403d0643: 48 89 5c 08 30              	movq	%rbx, 0x30(%rax,%rcx)
1403d0648: 49 ff c4                    	incq	%r12
1403d064b: 4c 89 67 10                 	movq	%r12, 0x10(%rdi)
1403d064f: 48 89 7c 24 38              	movq	%rdi, 0x38(%rsp)
1403d0654: 48 89 5c 24 40              	movq	%rbx, 0x40(%rsp)
1403d0659: 48 c7 44 24 48 00 00 00 00  	movq	$0x0, 0x48(%rsp)
1403d0662: 4c 8d 05 35 15 60 02        	leaq	0x2601535(%rip), %r8    # 0x1429d1b9e
1403d0669: 48 8d 5c 24 68              	leaq	0x68(%rsp), %rbx
1403d066e: 48 8d 54 24 38              	leaq	0x38(%rsp), %rdx
1403d0673: 41 b9 0c 00 00 00           	movl	$0xc, %r9d
1403d0679: 48 89 d9                    	movq	%rbx, %rcx
1403d067c: e8 5f b8 2b 00              	callq	0x14068bee0 <.text+0x68aee0>
1403d0681: 48 8d 15 22 15 60 02        	leaq	0x2601522(%rip), %rdx   # 0x1429d1baa
1403d0688: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
1403d068e: 48 89 d9                    	movq	%rbx, %rcx
1403d0691: 4d 89 f9                    	movq	%r15, %r9
1403d0694: e8 67 35 13 00              	callq	0x140503c00 <.text+0x502c00>
1403d0699: 48 8d 15 14 15 60 02        	leaq	0x2601514(%rip), %rdx   # 0x1429d1bb4
1403d06a0: 41 b8 07 00 00 00           	movl	$0x7, %r8d
1403d06a6: 48 89 d9                    	movq	%rbx, %rcx
1403d06a9: 4d 89 f9                    	movq	%r15, %r9
1403d06ac: e8 8f 17 13 00              	callq	0x140501e40 <.text+0x500e40>
1403d06b1: 8b 44 24 34                 	movl	0x34(%rsp), %eax
1403d06b5: 48 8b 4c 24 58              	movq	0x58(%rsp), %rcx
1403d06ba: 89 01                       	movl	%eax, (%rcx)
1403d06bc: 48 8d 05 0d 15 60 02        	leaq	0x260150d(%rip), %rax   # 0x1429d1bd0
1403d06c3: 48 89 44 24 68              	movq	%rax, 0x68(%rsp)
1403d06c8: 48 c7 44 24 70 0a 00 00 00  	movq	$0xa, 0x70(%rsp)
1403d06d1: 48 8d 4c 24 68              	leaq	0x68(%rsp), %rcx
1403d06d6: e8 35 35 2b 00              	callq	0x140683c10 <.text+0x682c10>
1403d06db: 48 89 c3                    	movq	%rax, %rbx
1403d06de: 48 b8 43 b4 4f 00 20 53 69 b1       	movabsq	$-0x4e96acdfffb04bbd, %rax # imm = 0xB1695320004FB443
1403d06e8: 48 31 c3                    	xorq	%rax, %rbx
1403d06eb: 48 89 f9                    	movq	%rdi, %rcx
1403d06ee: 48 89 da                    	movq	%rbx, %rdx
1403d06f1: e8 ca 37 2b 00              	callq	0x140683ec0 <.text+0x682ec0>
1403d06f6: c6 40 5d 00                 	movb	$0x0, 0x5d(%rax)
1403d06fa: c6 80 b7 00 00 00 01        	movb	$0x1, 0xb7(%rax)
1403d0701: f6 c2 01                    	testb	$0x1, %dl
1403d0704: 75 33                       	jne	0x1403d0739 <.text+0x3cf739>
1403d0706: 4c 8b 7f 10                 	movq	0x10(%rdi), %r15
1403d070a: 4c 3b 3f                    	cmpq	(%rdi), %r15
1403d070d: 75 0f                       	jne	0x1403d071e <.text+0x3cf71e>
1403d070f: 48 8d 15 2a eb 5e 02        	leaq	0x25eeb2a(%rip), %rdx   # 0x1429bf240
1403d0716: 48 89 f9                    	movq	%rdi, %rcx
1403d0719: e8 b2 12 26 00              	callq	0x1406319d0 <.text+0x6309d0>
1403d071e: 48 8b 47 08                 	movq	0x8(%rdi), %rax
1403d0722: 49 6b cf 38                 	imulq	$0x38, %r15, %rcx
1403d0726: c7 04 08 01 00 00 00        	movl	$0x1, (%rax,%rcx)
1403d072d: 48 89 5c 08 30              	movq	%rbx, 0x30(%rax,%rcx)
1403d0732: 49 ff c7                    	incq	%r15
1403d0735: 4c 89 7f 10                 	movq	%r15, 0x10(%rdi)
1403d0739: 48 89 7c 24 38              	movq	%rdi, 0x38(%rsp)
1403d073e: 48 89 5c 24 40              	movq	%rbx, 0x40(%rsp)
1403d0743: 48 c7 44 24 48 00 00 00 00  	movq	$0x0, 0x48(%rsp)
1403d074c: 4c 8d 05 87 14 60 02        	leaq	0x2601487(%rip), %r8    # 0x1429d1bda
1403d0753: 48 8d 7c 24 68              	leaq	0x68(%rsp), %rdi
1403d0758: 48 8d 54 24 38              	leaq	0x38(%rsp), %rdx
1403d075d: 41 b9 08 00 00 00           	movl	$0x8, %r9d
1403d0763: 48 89 f9                    	movq	%rdi, %rcx
1403d0766: e8 75 b7 2b 00              	callq	0x14068bee0 <.text+0x68aee0>
1403d076b: 48 8d 15 70 14 60 02        	leaq	0x2601470(%rip), %rdx   # 0x1429d1be2
1403d0772: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1403d0778: 48 89 f9                    	movq	%rdi, %rcx
1403d077b: e8 f0 fb 12 00              	callq	0x140500370 <.text+0x4ff370>
1403d0780: 48 89 74 24 20              	movq	%rsi, 0x20(%rsp)
1403d0785: 48 8d 15 5a 14 60 02        	leaq	0x260145a(%rip), %rdx   # 0x1429d1be6
1403d078c: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1403d0792: 48 89 f9                    	movq	%rdi, %rcx
1403d0795: 4d 89 e9                    	movq	%r13, %r9
1403d0798: e8 f3 df 12 00              	callq	0x1404fe790 <.text+0x4fd790>
1403d079d: 48 8d 15 e0 13 60 02        	leaq	0x26013e0(%rip), %rdx   # 0x1429d1b84
1403d07a4: 41 b8 07 00 00 00           	movl	$0x7, %r8d
1403d07aa: 48 89 f9                    	movq	%rdi, %rcx
1403d07ad: 49 89 f1                    	movq	%rsi, %r9
1403d07b0: e8 bb 06 13 00              	callq	0x140500e70 <.text+0x4ffe70>
1403d07b5: 48 8d 15 32 14 60 02        	leaq	0x2601432(%rip), %rdx   # 0x1429d1bee
1403d07bc: 41 b8 07 00 00 00           	movl	$0x7, %r8d
1403d07c2: 48 89 f9                    	movq	%rdi, %rcx
1403d07c5: 49 89 f1                    	movq	%rsi, %r9
1403d07c8: e8 83 04 13 00              	callq	0x140500c50 <.text+0x4ffc50>
1403d07cd: 90                          	nop
1403d07ce: 48 81 c4 48 01 00 00        	addq	$0x148, %rsp            # imm = 0x148
1403d07d5: 5b                          	popq	%rbx
1403d07d6: 5d                          	popq	%rbp
1403d07d7: 5f                          	popq	%rdi
1403d07d8: 5e                          	popq	%rsi
1403d07d9: 41 5c                       	popq	%r12
1403d07db: 41 5d                       	popq	%r13
1403d07dd: 41 5e                       	popq	%r14
1403d07df: 41 5f                       	popq	%r15
1403d07e1: c3                          	retq
