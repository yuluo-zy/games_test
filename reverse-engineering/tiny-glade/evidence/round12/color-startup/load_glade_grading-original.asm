
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408bf9c0: 55                          	pushq	%rbp
1408bf9c1: 41 57                       	pushq	%r15
1408bf9c3: 41 56                       	pushq	%r14
1408bf9c5: 41 55                       	pushq	%r13
1408bf9c7: 41 54                       	pushq	%r12
1408bf9c9: 56                          	pushq	%rsi
1408bf9ca: 57                          	pushq	%rdi
1408bf9cb: 53                          	pushq	%rbx
1408bf9cc: 48 81 ec 18 01 00 00        	subq	$0x118, %rsp            # imm = 0x118
1408bf9d3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1408bf9db: 0f 29 bd 80 00 00 00        	movaps	%xmm7, 0x80(%rbp)
1408bf9e2: 0f 29 75 70                 	movaps	%xmm6, 0x70(%rbp)
1408bf9e6: 48 c7 45 68 fe ff ff ff     	movq	$-0x2, 0x68(%rbp)
1408bf9ee: 48 89 ce                    	movq	%rcx, %rsi
1408bf9f1: 48 8b 0a                    	movq	(%rdx), %rcx
1408bf9f4: 48 8b 41 08                 	movq	0x8(%rcx), %rax
1408bf9f8: 48 89 4d 20                 	movq	%rcx, 0x20(%rbp)
1408bf9fc: 48 8b 49 10                 	movq	0x10(%rcx), %rcx
1408bfa00: 31 d2                       	xorl	%edx, %edx
1408bfa02: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1408bfa10: 48 39 d1                    	cmpq	%rdx, %rcx
1408bfa13: 0f 84 97 01 00 00           	je	0x1408bfbb0 <.text+0x8bebb0>
1408bfa19: 80 3c 10 00                 	cmpb	$0x0, (%rax,%rdx)
1408bfa1d: 48 8d 52 01                 	leaq	0x1(%rdx), %rdx
1408bfa21: 75 ed                       	jne	0x1408bfa10 <.text+0x8bea10>
1408bfa23: 8b 7e 1c                    	movl	0x1c(%rsi), %edi
1408bfa26: 4c 8b 7e 10                 	movq	0x10(%rsi), %r15
1408bfa2a: 41 89 3f                    	movl	%edi, (%r15)
1408bfa2d: 48 8b 1e                    	movq	(%rsi), %rbx
1408bfa30: 0f b6 05 d2 51 d7 02        	movzbl	0x2d751d2(%rip), %eax   # 0x143634c09
1408bfa37: b9 16 00 00 00              	movl	$0x16, %ecx
1408bfa3c: ba 01 00 00 00              	movl	$0x1, %edx
1408bfa41: e8 ca 41 d5 ff              	callq	0x140613c10 <.text+0x612c10>
1408bfa46: 48 85 c0                    	testq	%rax, %rax
1408bfa49: 0f 84 ea 05 00 00           	je	0x1408c0039 <.text+0x8bf039>
1408bfa4f: 48 89 c6                    	movq	%rax, %rsi
1408bfa52: 0f 10 35 a7 13 1e 02        	movups	0x21e13a7(%rip), %xmm6  # 0x142aa0e00
1408bfa59: 0f 11 30                    	movups	%xmm6, (%rax)
1408bfa5c: 49 bc 6d 61 5f 37 30 39 5f 31       	movabsq	$0x315f3930375f616d, %r12 # imm = 0x315F3930375F616D
1408bfa66: 4c 89 60 0e                 	movq	%r12, 0xe(%rax)
1408bfa6a: 48 8b 13                    	movq	(%rbx), %rdx
1408bfa6d: 48 85 d2                    	testq	%rdx, %rdx
1408bfa70: 74 0f                       	je	0x1408bfa81 <.text+0x8bea81>
1408bfa72: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
1408bfa76: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1408bfa7c: e8 9f 41 d5 ff              	callq	0x140613c20 <.text+0x612c20>
1408bfa81: 48 c7 03 16 00 00 00        	movq	$0x16, (%rbx)
1408bfa88: 48 89 73 08                 	movq	%rsi, 0x8(%rbx)
1408bfa8c: 48 c7 43 10 16 00 00 00     	movq	$0x16, 0x10(%rbx)
1408bfa94: 0f 28 3d f5 06 1e 02        	movaps	0x21e06f5(%rip), %xmm7  # 0x142aa0190
1408bfa9b: 0f 11 7b 18                 	movups	%xmm7, 0x18(%rbx)
1408bfa9f: c6 43 28 00                 	movb	$0x0, 0x28(%rbx)
1408bfaa3: 48 c7 43 30 00 00 00 00     	movq	$0x0, 0x30(%rbx)
1408bfaab: 48 c7 43 36 00 00 00 00     	movq	$0x0, 0x36(%rbx)
1408bfab3: 0f b6 05 4f 51 d7 02        	movzbl	0x2d7514f(%rip), %eax   # 0x143634c09
1408bfaba: b9 16 00 00 00              	movl	$0x16, %ecx
1408bfabf: ba 01 00 00 00              	movl	$0x1, %edx
1408bfac4: e8 47 41 d5 ff              	callq	0x140613c10 <.text+0x612c10>
1408bfac9: 48 85 c0                    	testq	%rax, %rax
1408bfacc: 0f 84 67 05 00 00           	je	0x1408c0039 <.text+0x8bf039>
1408bfad2: 48 89 c6                    	movq	%rax, %rsi
1408bfad5: 0f 11 30                    	movups	%xmm6, (%rax)
1408bfad8: 4c 89 60 0e                 	movq	%r12, 0xe(%rax)
1408bfadc: 48 8b 53 40                 	movq	0x40(%rbx), %rdx
1408bfae0: 48 85 d2                    	testq	%rdx, %rdx
1408bfae3: 74 0f                       	je	0x1408bfaf4 <.text+0x8beaf4>
1408bfae5: 48 8b 4b 48                 	movq	0x48(%rbx), %rcx
1408bfae9: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1408bfaef: e8 2c 41 d5 ff              	callq	0x140613c20 <.text+0x612c20>
1408bfaf4: 48 c7 43 40 16 00 00 00     	movq	$0x16, 0x40(%rbx)
1408bfafc: 48 89 73 48                 	movq	%rsi, 0x48(%rbx)
1408bfb00: 48 c7 43 50 16 00 00 00     	movq	$0x16, 0x50(%rbx)
1408bfb08: 0f 11 7b 58                 	movups	%xmm7, 0x58(%rbx)
1408bfb0c: c6 43 68 00                 	movb	$0x0, 0x68(%rbx)
1408bfb10: 48 c7 43 70 00 00 00 00     	movq	$0x0, 0x70(%rbx)
1408bfb18: 48 c7 43 76 00 00 00 00     	movq	$0x0, 0x76(%rbx)
1408bfb20: 0f b6 05 e2 50 d7 02        	movzbl	0x2d750e2(%rip), %eax   # 0x143634c09
1408bfb27: b9 16 00 00 00              	movl	$0x16, %ecx
1408bfb2c: ba 01 00 00 00              	movl	$0x1, %edx
1408bfb31: e8 da 40 d5 ff              	callq	0x140613c10 <.text+0x612c10>
1408bfb36: 48 85 c0                    	testq	%rax, %rax
1408bfb39: 0f 84 fa 04 00 00           	je	0x1408c0039 <.text+0x8bf039>
1408bfb3f: 48 89 c6                    	movq	%rax, %rsi
1408bfb42: 0f 11 30                    	movups	%xmm6, (%rax)
1408bfb45: 4c 89 60 0e                 	movq	%r12, 0xe(%rax)
1408bfb49: 48 8b 93 80 00 00 00        	movq	0x80(%rbx), %rdx
1408bfb50: 48 85 d2                    	testq	%rdx, %rdx
1408bfb53: 74 12                       	je	0x1408bfb67 <.text+0x8beb67>
1408bfb55: 48 8b 8b 88 00 00 00        	movq	0x88(%rbx), %rcx
1408bfb5c: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1408bfb62: e8 b9 40 d5 ff              	callq	0x140613c20 <.text+0x612c20>
1408bfb67: 48 c7 83 80 00 00 00 16 00 00 00    	movq	$0x16, 0x80(%rbx)
1408bfb72: 48 89 b3 88 00 00 00        	movq	%rsi, 0x88(%rbx)
1408bfb79: 48 c7 83 90 00 00 00 16 00 00 00    	movq	$0x16, 0x90(%rbx)
1408bfb84: 0f 11 bb 98 00 00 00        	movups	%xmm7, 0x98(%rbx)
1408bfb8b: c6 83 a8 00 00 00 00        	movb	$0x0, 0xa8(%rbx)
1408bfb92: 48 c7 83 b0 00 00 00 00 00 00 00    	movq	$0x0, 0xb0(%rbx)
1408bfb9d: 48 c7 83 b6 00 00 00 00 00 00 00    	movq	$0x0, 0xb6(%rbx)
1408bfba8: 48 89 de                    	movq	%rbx, %rsi
1408bfbab: e9 6d 03 00 00              	jmp	0x1408bff1d <.text+0x8bef1d>
1408bfbb0: 48 8b 45 20                 	movq	0x20(%rbp), %rax
1408bfbb4: 0f b6 40 58                 	movzbl	0x58(%rax), %eax
1408bfbb8: c1 e0 03                    	shll	$0x3, %eax
1408bfbbb: 48 8d 0d 7e 19 1e 02        	leaq	0x21e197e(%rip), %rcx   # 0x142aa1540
1408bfbc2: 4c 8b 0c 08                 	movq	(%rax,%rcx), %r9
1408bfbc6: 48 8d 0d 9b 19 1e 02        	leaq	0x21e199b(%rip), %rcx   # 0x142aa1568
1408bfbcd: 48 8b 04 08                 	movq	(%rax,%rcx), %rax
1408bfbd1: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1408bfbd6: 48 8d 15 3d f9 22 02        	leaq	0x222f93d(%rip), %rdx   # 0x142aef51a
1408bfbdd: 48 8d 7d c8                 	leaq	-0x38(%rbp), %rdi
1408bfbe1: 41 b8 0c 00 00 00           	movl	$0xc, %r8d
1408bfbe7: 48 89 f9                    	movq	%rdi, %rcx
1408bfbea: e8 21 2c 4a 00              	callq	0x140d62810 <.text+0xd61810>
1408bfbef: 48 89 f9                    	movq	%rdi, %rcx
1408bfbf2: e8 d9 72 4b 00              	callq	0x140d76ed0 <.text+0xd75ed0>
1408bfbf7: 49 89 d0                    	movq	%rdx, %r8
1408bfbfa: 48 c7 44 24 20 0c 00 00 00  	movq	$0xc, 0x20(%rsp)
1408bfc03: 4c 8d 0d 9d 11 1e 02        	leaq	0x21e119d(%rip), %r9    # 0x142aa0da7
1408bfc0a: 48 8d 4d 28                 	leaq	0x28(%rbp), %rcx
1408bfc0e: 48 89 c2                    	movq	%rax, %rdx
1408bfc11: e8 fa 2b 4a 00              	callq	0x140d62810 <.text+0xd61810>
1408bfc16: 48 8d 4d 28                 	leaq	0x28(%rbp), %rcx
1408bfc1a: e8 b1 72 4b 00              	callq	0x140d76ed0 <.text+0xd75ed0>
1408bfc1f: 49 89 d0                    	movq	%rdx, %r8
1408bfc22: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
1408bfc26: 48 89 c2                    	movq	%rax, %rdx
1408bfc29: e8 12 95 49 00              	callq	0x140d59140 <.text+0xd58140>
1408bfc2e: 48 8b 55 28                 	movq	0x28(%rbp), %rdx
1408bfc32: 48 85 d2                    	testq	%rdx, %rdx
1408bfc35: 74 0f                       	je	0x1408bfc46 <.text+0x8bec46>
1408bfc37: 48 8b 4d 30                 	movq	0x30(%rbp), %rcx
1408bfc3b: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1408bfc41: e8 da 3f d5 ff              	callq	0x140613c20 <.text+0x612c20>
1408bfc46: 48 8b 4d b0                 	movq	-0x50(%rbp), %rcx
1408bfc4a: 48 89 c8                    	movq	%rcx, %rax
1408bfc4d: 48 f7 d8                    	negq	%rax
1408bfc50: 0f 81 d2 00 00 00           	jno	0x1408bfd28 <.text+0x8bed28>
1408bfc56: 48 8b 45 b8                 	movq	-0x48(%rbp), %rax
1408bfc5a: 89 c1                       	movl	%eax, %ecx
1408bfc5c: 83 e1 03                    	andl	$0x3, %ecx
1408bfc5f: 83 f9 01                    	cmpl	$0x1, %ecx
1408bfc62: 75 54                       	jne	0x1408bfcb8 <.text+0x8becb8>
1408bfc64: 48 8d 48 ff                 	leaq	-0x1(%rax), %rcx
1408bfc68: 48 89 4d e8                 	movq	%rcx, -0x18(%rbp)
1408bfc6c: 48 8b 48 ff                 	movq	-0x1(%rax), %rcx
1408bfc70: 48 89 4d 60                 	movq	%rcx, 0x60(%rbp)
1408bfc74: 48 8b 40 07                 	movq	0x7(%rax), %rax
1408bfc78: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
1408bfc7c: 48 8b 00                    	movq	(%rax), %rax
1408bfc7f: 48 85 c0                    	testq	%rax, %rax
1408bfc82: 74 06                       	je	0x1408bfc8a <.text+0x8bec8a>
1408bfc84: 48 8b 4d 60                 	movq	0x60(%rbp), %rcx
1408bfc88: ff d0                       	callq	*%rax
1408bfc8a: 48 8b 4d 60                 	movq	0x60(%rbp), %rcx
1408bfc8e: 48 8b 45 58                 	movq	0x58(%rbp), %rax
1408bfc92: 48 8b 50 08                 	movq	0x8(%rax), %rdx
1408bfc96: 48 85 d2                    	testq	%rdx, %rdx
1408bfc99: 74 09                       	je	0x1408bfca4 <.text+0x8beca4>
1408bfc9b: 4c 8b 40 10                 	movq	0x10(%rax), %r8
1408bfc9f: e8 7c 3f d5 ff              	callq	0x140613c20 <.text+0x612c20>
1408bfca4: ba 18 00 00 00              	movl	$0x18, %edx
1408bfca9: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1408bfcaf: 48 8b 4d e8                 	movq	-0x18(%rbp), %rcx
1408bfcb3: e8 68 3f d5 ff              	callq	0x140613c20 <.text+0x612c20>
1408bfcb8: 48 8b 55 c8                 	movq	-0x38(%rbp), %rdx
1408bfcbc: 48 85 d2                    	testq	%rdx, %rdx
1408bfcbf: 74 0f                       	je	0x1408bfcd0 <.text+0x8becd0>
1408bfcc1: 48 8b 4d d0                 	movq	-0x30(%rbp), %rcx
1408bfcc5: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1408bfccb: e8 50 3f d5 ff              	callq	0x140613c20 <.text+0x612c20>
1408bfcd0: 0f b6 05 32 4f d7 02        	movzbl	0x2d74f32(%rip), %eax   # 0x143634c09
1408bfcd7: b9 07 00 00 00              	movl	$0x7, %ecx
1408bfcdc: ba 01 00 00 00              	movl	$0x1, %edx
1408bfce1: e8 2a 3f d5 ff              	callq	0x140613c10 <.text+0x612c10>
1408bfce6: 48 85 c0                    	testq	%rax, %rax
1408bfce9: 0f 84 60 03 00 00           	je	0x1408c004f <.text+0x8bf04f>
1408bfcef: 8b 0d fd 6e 22 02           	movl	0x2226efd(%rip), %ecx   # 0x142ae6bf2
1408bfcf5: 89 48 03                    	movl	%ecx, 0x3(%rax)
1408bfcf8: 8b 0d f1 6e 22 02           	movl	0x2226ef1(%rip), %ecx   # 0x142ae6bef
1408bfcfe: 89 08                       	movl	%ecx, (%rax)
1408bfd00: c6 45 18 00                 	movb	$0x0, 0x18(%rbp)
1408bfd04: 48 c7 45 f0 07 00 00 00     	movq	$0x7, -0x10(%rbp)
1408bfd0c: 48 89 45 f8                 	movq	%rax, -0x8(%rbp)
1408bfd10: 48 c7 45 00 07 00 00 00     	movq	$0x7, (%rbp)
1408bfd18: 0f 28 05 11 c1 1d 02        	movaps	0x21dc111(%rip), %xmm0  # 0x142a9be30
1408bfd1f: 0f 11 45 08                 	movups	%xmm0, 0x8(%rbp)
1408bfd23: 45 31 e4                    	xorl	%r12d, %r12d
1408bfd26: eb 72                       	jmp	0x1408bfd9a <.text+0x8bed9a>
1408bfd28: 48 89 4d 60                 	movq	%rcx, 0x60(%rbp)
1408bfd2c: 48 8b 55 b8                 	movq	-0x48(%rbp), %rdx
1408bfd30: 4c 8b 45 c0                 	movq	-0x40(%rbp), %r8
1408bfd34: 48 8d 4d 28                 	leaq	0x28(%rbp), %rcx
1408bfd38: 48 89 55 58                 	movq	%rdx, 0x58(%rbp)
1408bfd3c: e8 7f 61 b4 00              	callq	0x141405ec0 <.text+0x1404ec0>
1408bfd41: 31 c0                       	xorl	%eax, %eax
1408bfd43: 48 3b 45 28                 	cmpq	0x28(%rbp), %rax
1408bfd47: 0f 80 18 03 00 00           	jo	0x1408c0065 <.text+0x8bf065>
1408bfd4d: 0f 10 45 28                 	movups	0x28(%rbp), %xmm0
1408bfd51: 0f 10 4d 38                 	movups	0x38(%rbp), %xmm1
1408bfd55: 0f 10 55 48                 	movups	0x48(%rbp), %xmm2
1408bfd59: 0f 29 55 10                 	movaps	%xmm2, 0x10(%rbp)
1408bfd5d: 0f 29 4d 00                 	movaps	%xmm1, (%rbp)
1408bfd61: 0f 29 45 f0                 	movaps	%xmm0, -0x10(%rbp)
1408bfd65: 48 8b 55 60                 	movq	0x60(%rbp), %rdx
1408bfd69: 48 85 d2                    	testq	%rdx, %rdx
1408bfd6c: 74 0f                       	je	0x1408bfd7d <.text+0x8bed7d>
1408bfd6e: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1408bfd74: 48 8b 4d 58                 	movq	0x58(%rbp), %rcx
1408bfd78: e8 a3 3e d5 ff              	callq	0x140613c20 <.text+0x612c20>
1408bfd7d: 48 8b 55 c8                 	movq	-0x38(%rbp), %rdx
1408bfd81: 48 85 d2                    	testq	%rdx, %rdx
1408bfd84: 74 0f                       	je	0x1408bfd95 <.text+0x8bed95>
1408bfd86: 48 8b 4d d0                 	movq	-0x30(%rbp), %rcx
1408bfd8a: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1408bfd90: e8 8b 3e d5 ff              	callq	0x140613c20 <.text+0x612c20>
1408bfd95: 44 0f b6 65 18              	movzbl	0x18(%rbp), %r12d
1408bfd9a: 8b 7e 1c                    	movl	0x1c(%rsi), %edi
1408bfd9d: 4c 8b 7e 10                 	movq	0x10(%rsi), %r15
1408bfda1: 41 89 3f                    	movl	%edi, (%r15)
1408bfda4: 48 8b 36                    	movq	(%rsi), %rsi
1408bfda7: 48 8d 4d 28                 	leaq	0x28(%rbp), %rcx
1408bfdab: 48 8d 55 f0                 	leaq	-0x10(%rbp), %rdx
1408bfdaf: e8 dc bf 4b 00              	callq	0x140d7bd90 <.text+0xd7ad90>
1408bfdb4: 44 88 65 50                 	movb	%r12b, 0x50(%rbp)
1408bfdb8: 0f 10 45 08                 	movups	0x8(%rbp), %xmm0
1408bfdbc: 0f 11 45 40                 	movups	%xmm0, 0x40(%rbp)
1408bfdc0: 48 8b 16                    	movq	(%rsi), %rdx
1408bfdc3: 48 85 d2                    	testq	%rdx, %rdx
1408bfdc6: 74 14                       	je	0x1408bfddc <.text+0x8beddc>
1408bfdc8: 48 8b 4e 08                 	movq	0x8(%rsi), %rcx
1408bfdcc: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1408bfdd2: e8 49 3e d5 ff              	callq	0x140613c20 <.text+0x612c20>
1408bfdd7: 44 0f b6 65 18              	movzbl	0x18(%rbp), %r12d
1408bfddc: 0f 10 45 28                 	movups	0x28(%rbp), %xmm0
1408bfde0: 0f 10 4d 38                 	movups	0x38(%rbp), %xmm1
1408bfde4: 0f 10 55 48                 	movups	0x48(%rbp), %xmm2
1408bfde8: 0f 11 56 20                 	movups	%xmm2, 0x20(%rsi)
1408bfdec: 0f 11 4e 10                 	movups	%xmm1, 0x10(%rsi)
1408bfdf0: 0f 11 06                    	movups	%xmm0, (%rsi)
1408bfdf3: 48 c7 46 36 00 00 00 00     	movq	$0x0, 0x36(%rsi)
1408bfdfb: 48 c7 46 30 00 00 00 00     	movq	$0x0, 0x30(%rsi)
1408bfe03: 48 8d 4d 28                 	leaq	0x28(%rbp), %rcx
1408bfe07: 48 8d 55 f0                 	leaq	-0x10(%rbp), %rdx
1408bfe0b: e8 80 bf 4b 00              	callq	0x140d7bd90 <.text+0xd7ad90>
1408bfe10: 49 89 f6                    	movq	%rsi, %r14
1408bfe13: 48 83 c6 40                 	addq	$0x40, %rsi
1408bfe17: 44 88 65 50                 	movb	%r12b, 0x50(%rbp)
1408bfe1b: 0f 10 45 08                 	movups	0x8(%rbp), %xmm0
1408bfe1f: 0f 11 45 40                 	movups	%xmm0, 0x40(%rbp)
1408bfe23: 48 8b 16                    	movq	(%rsi), %rdx
1408bfe26: 48 85 d2                    	testq	%rdx, %rdx
1408bfe29: 74 0f                       	je	0x1408bfe3a <.text+0x8bee3a>
1408bfe2b: 49 8b 4e 48                 	movq	0x48(%r14), %rcx
1408bfe2f: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1408bfe35: e8 e6 3d d5 ff              	callq	0x140613c20 <.text+0x612c20>
1408bfe3a: 0f 10 45 28                 	movups	0x28(%rbp), %xmm0
1408bfe3e: 0f 10 4d 38                 	movups	0x38(%rbp), %xmm1
1408bfe42: 48 8b 45 48                 	movq	0x48(%rbp), %rax
1408bfe46: 48 89 46 20                 	movq	%rax, 0x20(%rsi)
1408bfe4a: 0f b6 45 50                 	movzbl	0x50(%rbp), %eax
1408bfe4e: 88 46 28                    	movb	%al, 0x28(%rsi)
1408bfe51: 8b 45 51                    	movl	0x51(%rbp), %eax
1408bfe54: 89 46 29                    	movl	%eax, 0x29(%rsi)
1408bfe57: 0f b7 45 55                 	movzwl	0x55(%rbp), %eax
1408bfe5b: 66 89 46 2d                 	movw	%ax, 0x2d(%rsi)
1408bfe5f: 0f b6 45 57                 	movzbl	0x57(%rbp), %eax
1408bfe63: 88 46 2f                    	movb	%al, 0x2f(%rsi)
1408bfe66: 0f 11 4e 10                 	movups	%xmm1, 0x10(%rsi)
1408bfe6a: 0f 11 06                    	movups	%xmm0, (%rsi)
1408bfe6d: 49 c7 46 76 00 00 00 00     	movq	$0x0, 0x76(%r14)
1408bfe75: 49 c7 46 70 00 00 00 00     	movq	$0x0, 0x70(%r14)
1408bfe7d: 8a 5d 18                    	movb	0x18(%rbp), %bl
1408bfe80: 48 8d 4d 28                 	leaq	0x28(%rbp), %rcx
1408bfe84: 48 8d 55 f0                 	leaq	-0x10(%rbp), %rdx
1408bfe88: e8 03 bf 4b 00              	callq	0x140d7bd90 <.text+0xd7ad90>
1408bfe8d: 49 8d b6 80 00 00 00        	leaq	0x80(%r14), %rsi
1408bfe94: 88 5d 50                    	movb	%bl, 0x50(%rbp)
1408bfe97: 0f 10 45 08                 	movups	0x8(%rbp), %xmm0
1408bfe9b: 0f 11 45 40                 	movups	%xmm0, 0x40(%rbp)
1408bfe9f: 48 8b 16                    	movq	(%rsi), %rdx
1408bfea2: 48 85 d2                    	testq	%rdx, %rdx
1408bfea5: 74 12                       	je	0x1408bfeb9 <.text+0x8beeb9>
1408bfea7: 49 8b 8e 88 00 00 00        	movq	0x88(%r14), %rcx
1408bfeae: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1408bfeb4: e8 67 3d d5 ff              	callq	0x140613c20 <.text+0x612c20>
1408bfeb9: 0f 10 45 28                 	movups	0x28(%rbp), %xmm0
1408bfebd: 0f 10 4d 38                 	movups	0x38(%rbp), %xmm1
1408bfec1: 48 8b 45 48                 	movq	0x48(%rbp), %rax
1408bfec5: 48 89 46 20                 	movq	%rax, 0x20(%rsi)
1408bfec9: 0f b6 45 50                 	movzbl	0x50(%rbp), %eax
1408bfecd: 88 46 28                    	movb	%al, 0x28(%rsi)
1408bfed0: 8b 45 51                    	movl	0x51(%rbp), %eax
1408bfed3: 89 46 29                    	movl	%eax, 0x29(%rsi)
1408bfed6: 0f b7 45 55                 	movzwl	0x55(%rbp), %eax
1408bfeda: 66 89 46 2d                 	movw	%ax, 0x2d(%rsi)
1408bfede: 0f b6 45 57                 	movzbl	0x57(%rbp), %eax
1408bfee2: 88 46 2f                    	movb	%al, 0x2f(%rsi)
1408bfee5: 0f 11 4e 10                 	movups	%xmm1, 0x10(%rsi)
1408bfee9: 0f 11 06                    	movups	%xmm0, (%rsi)
1408bfeec: 49 c7 86 b6 00 00 00 00 00 00 00    	movq	$0x0, 0xb6(%r14)
1408bfef7: 49 c7 86 b0 00 00 00 00 00 00 00    	movq	$0x0, 0xb0(%r14)
1408bff02: 48 8b 55 f0                 	movq	-0x10(%rbp), %rdx
1408bff06: 48 85 d2                    	testq	%rdx, %rdx
1408bff09: 4c 89 f6                    	movq	%r14, %rsi
1408bff0c: 74 0f                       	je	0x1408bff1d <.text+0x8bef1d>
1408bff0e: 48 8b 4d f8                 	movq	-0x8(%rbp), %rcx
1408bff12: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1408bff18: e8 03 3d d5 ff              	callq	0x140613c20 <.text+0x612c20>
1408bff1d: 31 c0                       	xorl	%eax, %eax
1408bff1f: 48 8b 4d 20                 	movq	0x20(%rbp), %rcx
1408bff23: 48 3b 41 18                 	cmpq	0x18(%rcx), %rax
1408bff27: 71 1f                       	jno	0x1408bff48 <.text+0x8bef48>
1408bff29: 0f 28 75 70                 	movaps	0x70(%rbp), %xmm6
1408bff2d: 0f 28 bd 80 00 00 00        	movaps	0x80(%rbp), %xmm7
1408bff34: 48 81 c4 18 01 00 00        	addq	$0x118, %rsp            # imm = 0x118
1408bff3b: 5b                          	popq	%rbx
1408bff3c: 5f                          	popq	%rdi
1408bff3d: 5e                          	popq	%rsi
1408bff3e: 41 5c                       	popq	%r12
1408bff40: 41 5d                       	popq	%r13
1408bff42: 41 5e                       	popq	%r14
1408bff44: 41 5f                       	popq	%r15
1408bff46: 5d                          	popq	%rbp
1408bff47: c3                          	retq
1408bff48: 48 8b 45 20                 	movq	0x20(%rbp), %rax
1408bff4c: 48 83 c0 18                 	addq	$0x18, %rax
1408bff50: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
1408bff54: 41 89 3f                    	movl	%edi, (%r15)
1408bff57: 31 c0                       	xorl	%eax, %eax
1408bff59: 45 31 ff                    	xorl	%r15d, %r15d
1408bff5c: eb 43                       	jmp	0x1408bffa1 <.text+0x8befa1>
1408bff5e: 66 90                       	nop
1408bff60: 49 ff c7                    	incq	%r15
1408bff63: 0f 10 45 28                 	movups	0x28(%rbp), %xmm0
1408bff67: 0f 10 4d 38                 	movups	0x38(%rbp), %xmm1
1408bff6b: 0f 10 55 48                 	movups	0x48(%rbp), %xmm2
1408bff6f: 41 0f 11 55 20              	movups	%xmm2, 0x20(%r13)
1408bff74: 41 0f 11 4d 10              	movups	%xmm1, 0x10(%r13)
1408bff79: 41 0f 11 45 00              	movups	%xmm0, (%r13)
1408bff7e: 41 0f 13 75 30              	movlps	%xmm6, 0x30(%r13)
1408bff83: f3 41 0f 11 7d 38           	movss	%xmm7, 0x38(%r13)
1408bff89: 45 88 75 3c                 	movb	%r14b, 0x3c(%r13)
1408bff8d: 41 88 5d 3d                 	movb	%bl, 0x3d(%r13)
1408bff91: 48 8b 4d 60                 	movq	0x60(%rbp), %rcx
1408bff95: 48 89 c8                    	movq	%rcx, %rax
1408bff98: 48 81 f9 c0 00 00 00        	cmpq	$0xc0, %rcx
1408bff9f: 74 88                       	je	0x1408bff29 <.text+0x8bef29>
1408bffa1: 4c 8d 60 40                 	leaq	0x40(%rax), %r12
1408bffa5: 4d 85 ff                    	testq	%r15, %r15
1408bffa8: 74 16                       	je	0x1408bffc0 <.text+0x8befc0>
1408bffaa: 4c 89 65 60                 	movq	%r12, 0x60(%rbp)
1408bffae: 49 89 c4                    	movq	%rax, %r12
1408bffb1: eb 28                       	jmp	0x1408bffdb <.text+0x8befdb>
1408bffb3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1408bffc0: 49 81 fc c0 00 00 00        	cmpq	$0xc0, %r12
1408bffc7: 0f 84 5c ff ff ff           	je	0x1408bff29 <.text+0x8bef29>
1408bffcd: 48 83 e8 80                 	subq	$-0x80, %rax
1408bffd1: 41 bf 01 00 00 00           	movl	$0x1, %r15d
1408bffd7: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
1408bffdb: 4e 8d 2c 26                 	leaq	(%rsi,%r12), %r13
1408bffdf: 48 89 f7                    	movq	%rsi, %rdi
1408bffe2: 48 8b 75 20                 	movq	0x20(%rbp), %rsi
1408bffe6: 0f b6 5e 40                 	movzbl	0x40(%rsi), %ebx
1408bffea: 48 8d 4d 28                 	leaq	0x28(%rbp), %rcx
1408bffee: 48 8b 55 58                 	movq	0x58(%rbp), %rdx
1408bfff2: e8 99 bd 4b 00              	callq	0x140d7bd90 <.text+0xd7ad90>
1408bfff7: 88 5d 50                    	movb	%bl, 0x50(%rbp)
1408bfffa: 0f 10 46 30                 	movups	0x30(%rsi), %xmm0
1408bfffe: 0f 11 45 40                 	movups	%xmm0, 0x40(%rbp)
1408c0002: f2 0f 10 76 48              	movsd	0x48(%rsi), %xmm6
1408c0007: f3 0f 10 7e 50              	movss	0x50(%rsi), %xmm7
1408c000c: 44 0f b6 76 54              	movzbl	0x54(%rsi), %r14d
1408c0011: 0f b6 5e 55                 	movzbl	0x55(%rsi), %ebx
1408c0015: 48 89 fe                    	movq	%rdi, %rsi
1408c0018: 4a 8b 14 27                 	movq	(%rdi,%r12), %rdx
1408c001c: 48 85 d2                    	testq	%rdx, %rdx
1408c001f: 0f 84 3b ff ff ff           	je	0x1408bff60 <.text+0x8bef60>
1408c0025: 49 8b 4d 08                 	movq	0x8(%r13), %rcx
1408c0029: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1408c002f: e8 ec 3b d5 ff              	callq	0x140613c20 <.text+0x612c20>
1408c0034: e9 27 ff ff ff              	jmp	0x1408bff60 <.text+0x8bef60>
1408c0039: 4c 8d 05 e0 04 1e 02        	leaq	0x21e04e0(%rip), %r8    # 0x142aa0520
1408c0040: b9 01 00 00 00              	movl	$0x1, %ecx
1408c0045: ba 16 00 00 00              	movl	$0x16, %edx
1408c004a: e8 94 8f 01 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1408c004f: 4c 8d 05 8a 67 22 02        	leaq	0x222678a(%rip), %r8    # 0x142ae67e0
1408c0056: b9 01 00 00 00              	movl	$0x1, %ecx
1408c005b: ba 07 00 00 00              	movl	$0x7, %edx
1408c0060: e8 7e 8f 01 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1408c0065: 48 8b 45 30                 	movq	0x30(%rbp), %rax
1408c0069: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
1408c006d: 48 8d 05 74 0d 1e 02        	leaq	0x21e0d74(%rip), %rax   # 0x142aa0de8
1408c0074: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1408c0079: 48 8d 0d 38 03 1e 02        	leaq	0x21e0338(%rip), %rcx   # 0x142aa03b8
1408c0080: 4c 8d 0d e1 03 1e 02        	leaq	0x21e03e1(%rip), %r9    # 0x142aa0468
1408c0087: 4c 8d 45 f0                 	leaq	-0x10(%rbp), %r8
1408c008b: ba 2b 00 00 00              	movl	$0x2b, %edx
1408c0090: e8 cb 96 01 02              	callq	0x1428d9760 <.text+0x28d8760>
1408c0095: 0f 0b                       	ud2
1408c0097: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1408c00a0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1408c00a5: 55                          	pushq	%rbp
1408c00a6: 41 57                       	pushq	%r15
1408c00a8: 41 56                       	pushq	%r14
1408c00aa: 41 55                       	pushq	%r13
1408c00ac: 41 54                       	pushq	%r12
1408c00ae: 56                          	pushq	%rsi
1408c00af: 57                          	pushq	%rdi
1408c00b0: 53                          	pushq	%rbx
1408c00b1: 48 83 ec 48                 	subq	$0x48, %rsp
1408c00b5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1408c00bc: 0f 29 7c 24 20              	movaps	%xmm7, 0x20(%rsp)
1408c00c1: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1408c00c6: 48 8b 55 c8                 	movq	-0x38(%rbp), %rdx
1408c00ca: 48 85 d2                    	testq	%rdx, %rdx
1408c00cd: 74 0f                       	je	0x1408c00de <.text+0x8bf0de>
1408c00cf: 48 8b 4d d0                 	movq	-0x30(%rbp), %rcx
1408c00d3: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1408c00d9: e8 42 3b d5 ff              	callq	0x140613c20 <.text+0x612c20>
1408c00de: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1408c00e3: 0f 28 7c 24 20              	movaps	0x20(%rsp), %xmm7
1408c00e8: 48 83 c4 48                 	addq	$0x48, %rsp
1408c00ec: 5b                          	popq	%rbx
1408c00ed: 5f                          	popq	%rdi
1408c00ee: 5e                          	popq	%rsi
1408c00ef: 41 5c                       	popq	%r12
1408c00f1: 41 5d                       	popq	%r13
1408c00f3: 41 5e                       	popq	%r14
1408c00f5: 41 5f                       	popq	%r15
1408c00f7: 5d                          	popq	%rbp
1408c00f8: c3                          	retq
1408c00f9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1408c0100: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1408c0105: 55                          	pushq	%rbp
1408c0106: 41 57                       	pushq	%r15
1408c0108: 41 56                       	pushq	%r14
1408c010a: 41 55                       	pushq	%r13
1408c010c: 41 54                       	pushq	%r12
1408c010e: 56                          	pushq	%rsi
1408c010f: 57                          	pushq	%rdi
1408c0110: 53                          	pushq	%rbx
1408c0111: 48 83 ec 48                 	subq	$0x48, %rsp
1408c0115: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1408c011c: 0f 29 7c 24 20              	movaps	%xmm7, 0x20(%rsp)
1408c0121: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1408c0126: 48 8b 55 28                 	movq	0x28(%rbp), %rdx
1408c012a: 48 85 d2                    	testq	%rdx, %rdx
1408c012d: 74 0f                       	je	0x1408c013e <.text+0x8bf13e>
1408c012f: 48 8b 4d 30                 	movq	0x30(%rbp), %rcx
1408c0133: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1408c0139: e8 e2 3a d5 ff              	callq	0x140613c20 <.text+0x612c20>
1408c013e: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1408c0143: 0f 28 7c 24 20              	movaps	0x20(%rsp), %xmm7
1408c0148: 48 83 c4 48                 	addq	$0x48, %rsp
1408c014c: 5b                          	popq	%rbx
1408c014d: 5f                          	popq	%rdi
1408c014e: 5e                          	popq	%rsi
1408c014f: 41 5c                       	popq	%r12
1408c0151: 41 5d                       	popq	%r13
1408c0153: 41 5e                       	popq	%r14
1408c0155: 41 5f                       	popq	%r15
1408c0157: 5d                          	popq	%rbp
1408c0158: c3                          	retq
1408c0159: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1408c0160: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1408c0165: 55                          	pushq	%rbp
1408c0166: 41 57                       	pushq	%r15
1408c0168: 41 56                       	pushq	%r14
1408c016a: 41 55                       	pushq	%r13
1408c016c: 41 54                       	pushq	%r12
1408c016e: 56                          	pushq	%rsi
1408c016f: 57                          	pushq	%rdi
1408c0170: 53                          	pushq	%rbx
1408c0171: 48 83 ec 48                 	subq	$0x48, %rsp
1408c0175: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1408c017c: 0f 29 7c 24 20              	movaps	%xmm7, 0x20(%rsp)
1408c0181: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1408c0186: 48 8b 4d 60                 	movq	0x60(%rbp), %rcx
1408c018a: 48 8b 45 58                 	movq	0x58(%rbp), %rax
1408c018e: 48 8b 50 08                 	movq	0x8(%rax), %rdx
1408c0192: 48 85 d2                    	testq	%rdx, %rdx
1408c0195: 74 0d                       	je	0x1408c01a4 <.text+0x8bf1a4>
1408c0197: 48 8b 45 58                 	movq	0x58(%rbp), %rax
1408c019b: 4c 8b 40 10                 	movq	0x10(%rax), %r8
1408c019f: e8 7c 3a d5 ff              	callq	0x140613c20 <.text+0x612c20>
1408c01a4: ba 18 00 00 00              	movl	$0x18, %edx
1408c01a9: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1408c01af: 48 8b 4d e8                 	movq	-0x18(%rbp), %rcx
1408c01b3: e8 68 3a d5 ff              	callq	0x140613c20 <.text+0x612c20>
1408c01b8: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1408c01bd: 0f 28 7c 24 20              	movaps	0x20(%rsp), %xmm7
1408c01c2: 48 83 c4 48                 	addq	$0x48, %rsp
1408c01c6: 5b                          	popq	%rbx
1408c01c7: 5f                          	popq	%rdi
1408c01c8: 5e                          	popq	%rsi
1408c01c9: 41 5c                       	popq	%r12
1408c01cb: 41 5d                       	popq	%r13
1408c01cd: 41 5e                       	popq	%r14
1408c01cf: 41 5f                       	popq	%r15
1408c01d1: 5d                          	popq	%rbp
1408c01d2: c3                          	retq
1408c01d3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1408c01e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1408c01e5: 55                          	pushq	%rbp
1408c01e6: 41 57                       	pushq	%r15
1408c01e8: 41 56                       	pushq	%r14
1408c01ea: 41 55                       	pushq	%r13
1408c01ec: 41 54                       	pushq	%r12
1408c01ee: 56                          	pushq	%rsi
1408c01ef: 57                          	pushq	%rdi
1408c01f0: 53                          	pushq	%rbx
1408c01f1: 48 83 ec 48                 	subq	$0x48, %rsp
1408c01f5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1408c01fc: 0f 29 7c 24 20              	movaps	%xmm7, 0x20(%rsp)
1408c0201: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1408c0206: 48 83 7d 60 00              	cmpq	$0x0, 0x60(%rbp)
1408c020b: 74 13                       	je	0x1408c0220 <.text+0x8bf220>
1408c020d: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1408c0213: 48 8b 4d 58                 	movq	0x58(%rbp), %rcx
1408c0217: 48 8b 55 60                 	movq	0x60(%rbp), %rdx
1408c021b: e8 00 3a d5 ff              	callq	0x140613c20 <.text+0x612c20>
1408c0220: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1408c0225: 0f 28 7c 24 20              	movaps	0x20(%rsp), %xmm7
1408c022a: 48 83 c4 48                 	addq	$0x48, %rsp
1408c022e: 5b                          	popq	%rbx
1408c022f: 5f                          	popq	%rdi
1408c0230: 5e                          	popq	%rsi
1408c0231: 41 5c                       	popq	%r12
1408c0233: 41 5d                       	popq	%r13
1408c0235: 41 5e                       	popq	%r14
1408c0237: 41 5f                       	popq	%r15
1408c0239: 5d                          	popq	%rbp
1408c023a: c3                          	retq
1408c023b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
1408c0240: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1408c0245: 55                          	pushq	%rbp
1408c0246: 41 57                       	pushq	%r15
1408c0248: 41 56                       	pushq	%r14
1408c024a: 41 55                       	pushq	%r13
1408c024c: 41 54                       	pushq	%r12
1408c024e: 56                          	pushq	%rsi
1408c024f: 57                          	pushq	%rdi
1408c0250: 53                          	pushq	%rbx
1408c0251: 48 83 ec 48                 	subq	$0x48, %rsp
1408c0255: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1408c025c: 0f 29 7c 24 20              	movaps	%xmm7, 0x20(%rsp)
1408c0261: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1408c0266: 48 8b 55 f0                 	movq	-0x10(%rbp), %rdx
1408c026a: 48 85 d2                    	testq	%rdx, %rdx
1408c026d: 74 0f                       	je	0x1408c027e <.text+0x8bf27e>
1408c026f: 48 8b 4d f8                 	movq	-0x8(%rbp), %rcx
1408c0273: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1408c0279: e8 a2 39 d5 ff              	callq	0x140613c20 <.text+0x612c20>
1408c027e: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1408c0283: 0f 28 7c 24 20              	movaps	0x20(%rsp), %xmm7
1408c0288: 48 83 c4 48                 	addq	$0x48, %rsp
1408c028c: 5b                          	popq	%rbx
1408c028d: 5f                          	popq	%rdi
1408c028e: 5e                          	popq	%rsi
1408c028f: 41 5c                       	popq	%r12
1408c0291: 41 5d                       	popq	%r13
1408c0293: 41 5e                       	popq	%r14
1408c0295: 41 5f                       	popq	%r15
1408c0297: 5d                          	popq	%rbp
1408c0298: c3                          	retq
1408c0299: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1408c02a0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1408c02a5: 55                          	pushq	%rbp
1408c02a6: 41 57                       	pushq	%r15
1408c02a8: 41 56                       	pushq	%r14
1408c02aa: 41 55                       	pushq	%r13
1408c02ac: 41 54                       	pushq	%r12
1408c02ae: 56                          	pushq	%rsi
1408c02af: 57                          	pushq	%rdi
1408c02b0: 53                          	pushq	%rbx
1408c02b1: 48 83 ec 48                 	subq	$0x48, %rsp
1408c02b5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1408c02bc: 0f 29 7c 24 20              	movaps	%xmm7, 0x20(%rsp)
1408c02c1: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1408c02c6: 48 8d 4d f0                 	leaq	-0x10(%rbp), %rcx
1408c02ca: e8 d1 85 d2 ff              	callq	0x1405e88a0 <.text+0x5e78a0>
1408c02cf: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1408c02d4: 0f 28 7c 24 20              	movaps	0x20(%rsp), %xmm7
1408c02d9: 48 83 c4 48                 	addq	$0x48, %rsp
1408c02dd: 5b                          	popq	%rbx
1408c02de: 5f                          	popq	%rdi
1408c02df: 5e                          	popq	%rsi
1408c02e0: 41 5c                       	popq	%r12
1408c02e2: 41 5d                       	popq	%r13
1408c02e4: 41 5e                       	popq	%r14
1408c02e6: 41 5f                       	popq	%r15
1408c02e8: 5d                          	popq	%rbp
1408c02e9: c3                          	retq
