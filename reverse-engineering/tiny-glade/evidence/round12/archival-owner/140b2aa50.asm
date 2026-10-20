
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b2aa50: 55                          	pushq	%rbp
140b2aa51: 41 57                       	pushq	%r15
140b2aa53: 41 56                       	pushq	%r14
140b2aa55: 41 55                       	pushq	%r13
140b2aa57: 41 54                       	pushq	%r12
140b2aa59: 56                          	pushq	%rsi
140b2aa5a: 57                          	pushq	%rdi
140b2aa5b: 53                          	pushq	%rbx
140b2aa5c: 48 81 ec d8 01 00 00        	subq	$0x1d8, %rsp            # imm = 0x1D8
140b2aa63: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140b2aa6b: 48 c7 85 50 01 00 00 fe ff ff ff    	movq	$-0x2, 0x150(%rbp)
140b2aa76: 48 89 cb                    	movq	%rcx, %rbx
140b2aa79: 4c 8b 2a                    	movq	(%rdx), %r13
140b2aa7c: 48 8b 72 08                 	movq	0x8(%rdx), %rsi
140b2aa80: 48 83 c2 10                 	addq	$0x10, %rdx
140b2aa84: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
140b2aa88: 41 b8 90 00 00 00           	movl	$0x90, %r8d
140b2aa8e: e8 6d 91 df 01              	callq	0x142923c00 <.text+0x2922c00>
140b2aa93: 48 8b 03                    	movq	(%rbx), %rax
140b2aa96: 4c 8b 20                    	movq	(%rax), %r12
140b2aa99: 48 89 85 48 01 00 00        	movq	%rax, 0x148(%rbp)
140b2aaa0: 48 8b 78 08                 	movq	0x8(%rax), %rdi
140b2aaa4: 4c 8b 7d f0                 	movq	-0x10(%rbp), %r15
140b2aaa8: 4c 8b 75 f8                 	movq	-0x8(%rbp), %r14
140b2aaac: 4b 8d 44 37 01              	leaq	0x1(%r15,%r14), %rax
140b2aab1: 66 41 0f 6f 07              	movdqa	(%r15), %xmm0
140b2aab6: 66 0f d7 c8                 	pmovmskb	%xmm0, %ecx
140b2aaba: f7 d1                       	notl	%ecx
140b2aabc: 49 8d 57 10                 	leaq	0x10(%r15), %rdx
140b2aac0: 66 89 8d 28 01 00 00        	movw	%cx, 0x128(%rbp)
140b2aac7: 4c 89 bd 10 01 00 00        	movq	%r15, 0x110(%rbp)
140b2aace: 48 89 95 18 01 00 00        	movq	%rdx, 0x118(%rbp)
140b2aad5: 48 89 85 20 01 00 00        	movq	%rax, 0x120(%rbp)
140b2aadc: 48 8b 45 08                 	movq	0x8(%rbp), %rax
140b2aae0: 48 89 85 30 01 00 00        	movq	%rax, 0x130(%rbp)
140b2aae7: 48 8d 8d b0 00 00 00        	leaq	0xb0(%rbp), %rcx
140b2aaee: 48 8d 9d 10 01 00 00        	leaq	0x110(%rbp), %rbx
140b2aaf5: 48 89 da                    	movq	%rbx, %rdx
140b2aaf8: e8 73 eb 91 00              	callq	0x141449670 <.text+0x1448670>
140b2aafd: f3 0f 6f 85 b0 00 00 00     	movdqu	0xb0(%rbp), %xmm0
140b2ab05: 0f 10 8d c0 00 00 00        	movups	0xc0(%rbp), %xmm1
140b2ab0c: 0f 10 95 d0 00 00 00        	movups	0xd0(%rbp), %xmm2
140b2ab13: 66 0f 7f 85 10 01 00 00     	movdqa	%xmm0, 0x110(%rbp)
140b2ab1b: 0f 29 8d 20 01 00 00        	movaps	%xmm1, 0x120(%rbp)
140b2ab22: 0f 29 95 30 01 00 00        	movaps	%xmm2, 0x130(%rbp)
140b2ab29: 0f b6 07                    	movzbl	(%rdi), %eax
140b2ab2c: 88 44 24 30                 	movb	%al, 0x30(%rsp)
140b2ab30: 48 89 5c 24 20              	movq	%rbx, 0x20(%rsp)
140b2ab35: c6 44 24 28 00              	movb	$0x0, 0x28(%rsp)
140b2ab3a: 48 8d 5d 50                 	leaq	0x50(%rbp), %rbx
140b2ab3e: 48 89 d9                    	movq	%rbx, %rcx
140b2ab41: 4c 89 e2                    	movq	%r12, %rdx
140b2ab44: 4d 89 e8                    	movq	%r13, %r8
140b2ab47: 49 89 f1                    	movq	%rsi, %r9
140b2ab4a: e8 21 13 00 00              	callq	0x140b2be70 <.text+0xb2ae70>
140b2ab4f: 4c 8b 65 50                 	movq	0x50(%rbp), %r12
140b2ab53: 4d 85 e4                    	testq	%r12, %r12
140b2ab56: 0f 84 dc 01 00 00           	je	0x140b2ad38 <.text+0xb29d38>
140b2ab5c: 48 8b 85 a8 00 00 00        	movq	0xa8(%rbp), %rax
140b2ab63: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
140b2ab6a: 0f 10 85 98 00 00 00        	movups	0x98(%rbp), %xmm0
140b2ab71: 0f 29 85 f0 00 00 00        	movaps	%xmm0, 0xf0(%rbp)
140b2ab78: f3 0f 6f 45 58              	movdqu	0x58(%rbp), %xmm0
140b2ab7d: 0f 10 4d 68                 	movups	0x68(%rbp), %xmm1
140b2ab81: 0f 10 55 78                 	movups	0x78(%rbp), %xmm2
140b2ab85: 0f 10 9d 88 00 00 00        	movups	0x88(%rbp), %xmm3
140b2ab8c: 0f 29 9d e0 00 00 00        	movaps	%xmm3, 0xe0(%rbp)
140b2ab93: 0f 29 95 d0 00 00 00        	movaps	%xmm2, 0xd0(%rbp)
140b2ab9a: 0f 29 8d c0 00 00 00        	movaps	%xmm1, 0xc0(%rbp)
140b2aba1: 66 0f 7f 85 b0 00 00 00     	movdqa	%xmm0, 0xb0(%rbp)
140b2aba9: 4d 85 f6                    	testq	%r14, %r14
140b2abac: 74 29                       	je	0x140b2abd7 <.text+0xb29bd7>
140b2abae: 49 8d 46 01                 	leaq	0x1(%r14), %rax
140b2abb2: b9 50 00 00 00              	movl	$0x50, %ecx
140b2abb7: 48 f7 e1                    	mulq	%rcx
140b2abba: 49 01 c6                    	addq	%rax, %r14
140b2abbd: 49 83 c6 11                 	addq	$0x11, %r14
140b2abc1: 74 14                       	je	0x140b2abd7 <.text+0xb29bd7>
140b2abc3: 49 29 c7                    	subq	%rax, %r15
140b2abc6: 41 b8 10 00 00 00           	movl	$0x10, %r8d
140b2abcc: 4c 89 f9                    	movq	%r15, %rcx
140b2abcf: 4c 89 f2                    	movq	%r14, %rdx
140b2abd2: e8 49 90 ae ff              	callq	0x140613c20 <.text+0x612c20>
140b2abd7: 48 8b 55 c8                 	movq	-0x38(%rbp), %rdx
140b2abdb: 48 85 d2                    	testq	%rdx, %rdx
140b2abde: 74 27                       	je	0x140b2ac07 <.text+0xb29c07>
140b2abe0: 48 8d 04 95 13 00 00 00     	leaq	0x13(,%rdx,4), %rax
140b2abe8: 48 83 e0 f0                 	andq	$-0x10, %rax
140b2abec: 48 01 c2                    	addq	%rax, %rdx
140b2abef: 48 83 c2 11                 	addq	$0x11, %rdx
140b2abf3: 74 12                       	je	0x140b2ac07 <.text+0xb29c07>
140b2abf5: 48 8b 4d c0                 	movq	-0x40(%rbp), %rcx
140b2abf9: 48 29 c1                    	subq	%rax, %rcx
140b2abfc: 41 b8 10 00 00 00           	movl	$0x10, %r8d
140b2ac02: e8 19 90 ae ff              	callq	0x140613c20 <.text+0x612c20>
140b2ac07: 48 8b 55 28                 	movq	0x28(%rbp), %rdx
140b2ac0b: 48 85 d2                    	testq	%rdx, %rdx
140b2ac0e: 74 27                       	je	0x140b2ac37 <.text+0xb29c37>
140b2ac10: 48 8d 04 95 13 00 00 00     	leaq	0x13(,%rdx,4), %rax
140b2ac18: 48 83 e0 f0                 	andq	$-0x10, %rax
140b2ac1c: 48 01 c2                    	addq	%rax, %rdx
140b2ac1f: 48 83 c2 11                 	addq	$0x11, %rdx
140b2ac23: 74 12                       	je	0x140b2ac37 <.text+0xb29c37>
140b2ac25: 48 8b 4d 20                 	movq	0x20(%rbp), %rcx
140b2ac29: 48 29 c1                    	subq	%rax, %rcx
140b2ac2c: 41 b8 10 00 00 00           	movl	$0x10, %r8d
140b2ac32: e8 e9 8f ae ff              	callq	0x140613c20 <.text+0x612c20>
140b2ac37: 49 83 fd 03                 	cmpq	$0x3, %r13
140b2ac3b: 0f 84 85 01 00 00           	je	0x140b2adc6 <.text+0xb29dc6>
140b2ac41: 48 8b 85 00 01 00 00        	movq	0x100(%rbp), %rax
140b2ac48: 48 89 85 a8 00 00 00        	movq	%rax, 0xa8(%rbp)
140b2ac4f: 0f 28 85 f0 00 00 00        	movaps	0xf0(%rbp), %xmm0
140b2ac56: 0f 11 85 98 00 00 00        	movups	%xmm0, 0x98(%rbp)
140b2ac5d: 66 0f 6f 85 b0 00 00 00     	movdqa	0xb0(%rbp), %xmm0
140b2ac65: 0f 28 8d c0 00 00 00        	movaps	0xc0(%rbp), %xmm1
140b2ac6c: 0f 28 95 d0 00 00 00        	movaps	0xd0(%rbp), %xmm2
140b2ac73: 0f 28 9d e0 00 00 00        	movaps	0xe0(%rbp), %xmm3
140b2ac7a: 0f 11 9d 88 00 00 00        	movups	%xmm3, 0x88(%rbp)
140b2ac81: 0f 11 55 78                 	movups	%xmm2, 0x78(%rbp)
140b2ac85: 0f 11 4d 68                 	movups	%xmm1, 0x68(%rbp)
140b2ac89: f3 0f 7f 45 58              	movdqu	%xmm0, 0x58(%rbp)
140b2ac8e: 48 8b 85 48 01 00 00        	movq	0x148(%rbp), %rax
140b2ac95: 48 8b 50 10                 	movq	0x10(%rax), %rdx
140b2ac99: 4c 89 65 50                 	movq	%r12, 0x50(%rbp)
140b2ac9d: 48 89 5c 24 20              	movq	%rbx, 0x20(%rsp)
140b2aca2: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
140b2aca6: 4d 89 e8                    	movq	%r13, %r8
140b2aca9: 49 89 f1                    	movq	%rsi, %r9
140b2acac: e8 8f eb 9f 00              	callq	0x141529840 <.text+0x1528840>
140b2acb1: 48 8b 4d c0                 	movq	-0x40(%rbp), %rcx
140b2acb5: 48 85 c9                    	testq	%rcx, %rcx
140b2acb8: 0f 84 08 01 00 00           	je	0x140b2adc6 <.text+0xb29dc6>
140b2acbe: 4c 8b 4d c8                 	movq	-0x38(%rbp), %r9
140b2acc2: 4d 85 c9                    	testq	%r9, %r9
140b2acc5: 74 2e                       	je	0x140b2acf5 <.text+0xb29cf5>
140b2acc7: 49 8d 41 01                 	leaq	0x1(%r9), %rax
140b2accb: ba 58 00 00 00              	movl	$0x58, %edx
140b2acd0: 48 f7 e2                    	mulq	%rdx
140b2acd3: 48 83 c0 0f                 	addq	$0xf, %rax
140b2acd7: 48 83 e0 f0                 	andq	$-0x10, %rax
140b2acdb: 49 01 c1                    	addq	%rax, %r9
140b2acde: 49 83 c1 11                 	addq	$0x11, %r9
140b2ace2: 74 11                       	je	0x140b2acf5 <.text+0xb29cf5>
140b2ace4: 48 29 c1                    	subq	%rax, %rcx
140b2ace7: 41 b8 10 00 00 00           	movl	$0x10, %r8d
140b2aced: 4c 89 ca                    	movq	%r9, %rdx
140b2acf0: e8 2b 8f ae ff              	callq	0x140613c20 <.text+0x612c20>
140b2acf5: 4c 8b 4d f8                 	movq	-0x8(%rbp), %r9
140b2acf9: 4d 85 c9                    	testq	%r9, %r9
140b2acfc: 0f 84 c4 00 00 00           	je	0x140b2adc6 <.text+0xb29dc6>
140b2ad02: 49 8d 41 01                 	leaq	0x1(%r9), %rax
140b2ad06: b9 58 00 00 00              	movl	$0x58, %ecx
140b2ad0b: 48 f7 e1                    	mulq	%rcx
140b2ad0e: 48 83 c0 0f                 	addq	$0xf, %rax
140b2ad12: 48 83 e0 f0                 	andq	$-0x10, %rax
140b2ad16: 49 01 c1                    	addq	%rax, %r9
140b2ad19: 49 83 c1 11                 	addq	$0x11, %r9
140b2ad1d: 0f 84 a3 00 00 00           	je	0x140b2adc6 <.text+0xb29dc6>
140b2ad23: 48 8b 4d f0                 	movq	-0x10(%rbp), %rcx
140b2ad27: 48 29 c1                    	subq	%rax, %rcx
140b2ad2a: 41 b8 10 00 00 00           	movl	$0x10, %r8d
140b2ad30: 4c 89 ca                    	movq	%r9, %rdx
140b2ad33: e9 89 00 00 00              	jmp	0x140b2adc1 <.text+0xb29dc1>
140b2ad38: 4d 85 f6                    	testq	%r14, %r14
140b2ad3b: 74 29                       	je	0x140b2ad66 <.text+0xb29d66>
140b2ad3d: 49 8d 46 01                 	leaq	0x1(%r14), %rax
140b2ad41: b9 50 00 00 00              	movl	$0x50, %ecx
140b2ad46: 48 f7 e1                    	mulq	%rcx
140b2ad49: 49 01 c6                    	addq	%rax, %r14
140b2ad4c: 49 83 c6 11                 	addq	$0x11, %r14
140b2ad50: 74 14                       	je	0x140b2ad66 <.text+0xb29d66>
140b2ad52: 49 29 c7                    	subq	%rax, %r15
140b2ad55: 41 b8 10 00 00 00           	movl	$0x10, %r8d
140b2ad5b: 4c 89 f9                    	movq	%r15, %rcx
140b2ad5e: 4c 89 f2                    	movq	%r14, %rdx
140b2ad61: e8 ba 8e ae ff              	callq	0x140613c20 <.text+0x612c20>
140b2ad66: 48 8b 55 c8                 	movq	-0x38(%rbp), %rdx
140b2ad6a: 48 85 d2                    	testq	%rdx, %rdx
140b2ad6d: 74 27                       	je	0x140b2ad96 <.text+0xb29d96>
140b2ad6f: 48 8d 04 95 13 00 00 00     	leaq	0x13(,%rdx,4), %rax
140b2ad77: 48 83 e0 f0                 	andq	$-0x10, %rax
140b2ad7b: 48 01 c2                    	addq	%rax, %rdx
140b2ad7e: 48 83 c2 11                 	addq	$0x11, %rdx
140b2ad82: 74 12                       	je	0x140b2ad96 <.text+0xb29d96>
140b2ad84: 48 8b 4d c0                 	movq	-0x40(%rbp), %rcx
140b2ad88: 48 29 c1                    	subq	%rax, %rcx
140b2ad8b: 41 b8 10 00 00 00           	movl	$0x10, %r8d
140b2ad91: e8 8a 8e ae ff              	callq	0x140613c20 <.text+0x612c20>
140b2ad96: 48 8b 55 28                 	movq	0x28(%rbp), %rdx
140b2ad9a: 48 85 d2                    	testq	%rdx, %rdx
140b2ad9d: 74 27                       	je	0x140b2adc6 <.text+0xb29dc6>
140b2ad9f: 48 8d 04 95 13 00 00 00     	leaq	0x13(,%rdx,4), %rax
140b2ada7: 48 83 e0 f0                 	andq	$-0x10, %rax
140b2adab: 48 01 c2                    	addq	%rax, %rdx
140b2adae: 48 83 c2 11                 	addq	$0x11, %rdx
140b2adb2: 74 12                       	je	0x140b2adc6 <.text+0xb29dc6>
140b2adb4: 48 8b 4d 20                 	movq	0x20(%rbp), %rcx
140b2adb8: 48 29 c1                    	subq	%rax, %rcx
140b2adbb: 41 b8 10 00 00 00           	movl	$0x10, %r8d
140b2adc1: e8 5a 8e ae ff              	callq	0x140613c20 <.text+0x612c20>
140b2adc6: 90                          	nop
140b2adc7: 48 81 c4 d8 01 00 00        	addq	$0x1d8, %rsp            # imm = 0x1D8
140b2adce: 5b                          	popq	%rbx
140b2adcf: 5f                          	popq	%rdi
140b2add0: 5e                          	popq	%rsi
140b2add1: 41 5c                       	popq	%r12
140b2add3: 41 5d                       	popq	%r13
140b2add5: 41 5e                       	popq	%r14
140b2add7: 41 5f                       	popq	%r15
140b2add9: 5d                          	popq	%rbp
140b2adda: c3                          	retq
140b2addb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140b2ade0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b2ade5: 55                          	pushq	%rbp
140b2ade6: 41 57                       	pushq	%r15
140b2ade8: 41 56                       	pushq	%r14
140b2adea: 41 55                       	pushq	%r13
140b2adec: 41 54                       	pushq	%r12
140b2adee: 56                          	pushq	%rsi
140b2adef: 57                          	pushq	%rdi
140b2adf0: 53                          	pushq	%rbx
140b2adf1: 48 83 ec 38                 	subq	$0x38, %rsp
140b2adf5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b2adfc: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
140b2ae00: e8 8b 9c ad ff              	callq	0x140604a90 <.text+0x603a90>
140b2ae05: 90                          	nop
140b2ae06: 48 83 c4 38                 	addq	$0x38, %rsp
140b2ae0a: 5b                          	popq	%rbx
140b2ae0b: 5f                          	popq	%rdi
140b2ae0c: 5e                          	popq	%rsi
140b2ae0d: 41 5c                       	popq	%r12
140b2ae0f: 41 5d                       	popq	%r13
140b2ae11: 41 5e                       	popq	%r14
140b2ae13: 41 5f                       	popq	%r15
140b2ae15: 5d                          	popq	%rbp
140b2ae16: c3                          	retq
