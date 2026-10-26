
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141559a10: 55                          	pushq	%rbp
141559a11: 56                          	pushq	%rsi
141559a12: 57                          	pushq	%rdi
141559a13: 48 81 ec a0 00 00 00        	subq	$0xa0, %rsp
141559a1a: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
141559a22: 48 c7 45 18 fe ff ff ff     	movq	$-0x2, 0x18(%rbp)
141559a2a: 48 89 ce                    	movq	%rcx, %rsi
141559a2d: 48 8d 7d e0                 	leaq	-0x20(%rbp), %rdi
141559a31: 48 89 f9                    	movq	%rdi, %rcx
141559a34: e8 b7 f2 73 ff              	callq	0x140c98cf0 <.text+0xc97cf0>
141559a39: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
141559a3d: f3 0f 10 15 2b be 3c 01     	movss	0x13cbe2b(%rip), %xmm2  # 0x142925870
141559a45: 48 89 fa                    	movq	%rdi, %rdx
141559a48: e8 a3 1b f1 ff              	callq	0x14146b5f0 <.text+0x146a5f0>
141559a4d: 48 8b 45 b8                 	movq	-0x48(%rbp), %rax
141559a51: 48 89 46 10                 	movq	%rax, 0x10(%rsi)
141559a55: 0f 10 45 a8                 	movups	-0x58(%rbp), %xmm0
141559a59: 0f 11 06                    	movups	%xmm0, (%rsi)
141559a5c: 48 8b 55 c0                 	movq	-0x40(%rbp), %rdx
141559a60: 48 85 d2                    	testq	%rdx, %rdx
141559a63: 74 13                       	je	0x141559a78 <.text+0x1558a78>
141559a65: 48 8b 4d c8                 	movq	-0x38(%rbp), %rcx
141559a69: 48 c1 e2 02                 	shlq	$0x2, %rdx
141559a6d: 41 b8 04 00 00 00           	movl	$0x4, %r8d
141559a73: e8 a8 a1 0b ff              	callq	0x140613c20 <.text+0x612c20>
141559a78: 48 8b 55 e0                 	movq	-0x20(%rbp), %rdx
141559a7c: 48 85 d2                    	testq	%rdx, %rdx
141559a7f: 74 13                       	je	0x141559a94 <.text+0x1558a94>
141559a81: 48 8b 4d e8                 	movq	-0x18(%rbp), %rcx
141559a85: 48 c1 e2 03                 	shlq	$0x3, %rdx
141559a89: 41 b8 04 00 00 00           	movl	$0x4, %r8d
141559a8f: e8 8c a1 0b ff              	callq	0x140613c20 <.text+0x612c20>
141559a94: 48 8b 55 f8                 	movq	-0x8(%rbp), %rdx
141559a98: 48 85 d2                    	testq	%rdx, %rdx
141559a9b: 74 13                       	je	0x141559ab0 <.text+0x1558ab0>
141559a9d: 48 8b 4d 00                 	movq	(%rbp), %rcx
141559aa1: 48 c1 e2 02                 	shlq	$0x2, %rdx
141559aa5: 41 b8 04 00 00 00           	movl	$0x4, %r8d
141559aab: e8 70 a1 0b ff              	callq	0x140613c20 <.text+0x612c20>
141559ab0: 48 89 f0                    	movq	%rsi, %rax
141559ab3: 48 81 c4 a0 00 00 00        	addq	$0xa0, %rsp
141559aba: 5f                          	popq	%rdi
141559abb: 5e                          	popq	%rsi
141559abc: 5d                          	popq	%rbp
141559abd: c3                          	retq
141559abe: 66 90                       	nop
141559ac0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141559ac5: 55                          	pushq	%rbp
141559ac6: 56                          	pushq	%rsi
141559ac7: 57                          	pushq	%rdi
141559ac8: 48 83 ec 20                 	subq	$0x20, %rsp
141559acc: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141559ad3: 48 8d 4d e0                 	leaq	-0x20(%rbp), %rcx
141559ad7: e8 14 8a 07 ff              	callq	0x1405d24f0 <.text+0x5d14f0>
141559adc: 90                          	nop
141559add: 48 83 c4 20                 	addq	$0x20, %rsp
141559ae1: 5f                          	popq	%rdi
141559ae2: 5e                          	popq	%rsi
141559ae3: 5d                          	popq	%rbp
141559ae4: c3                          	retq
