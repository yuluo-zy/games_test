
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14165cd40: 55                          	pushq	%rbp
14165cd41: 41 57                       	pushq	%r15
14165cd43: 41 56                       	pushq	%r14
14165cd45: 41 55                       	pushq	%r13
14165cd47: 41 54                       	pushq	%r12
14165cd49: 56                          	pushq	%rsi
14165cd4a: 57                          	pushq	%rdi
14165cd4b: 53                          	pushq	%rbx
14165cd4c: 48 81 ec a8 00 00 00        	subq	$0xa8, %rsp
14165cd53: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14165cd5b: 48 c7 45 20 fe ff ff ff     	movq	$-0x2, 0x20(%rbp)
14165cd63: 49 89 d6                    	movq	%rdx, %r14
14165cd66: 48 89 ce                    	movq	%rcx, %rsi
14165cd69: 44 88 45 1f                 	movb	%r8b, 0x1f(%rbp)
14165cd6d: 48 89 55 a0                 	movq	%rdx, -0x60(%rbp)
14165cd71: 48 8d 05 c8 ef ff ff        	leaq	-0x1038(%rip), %rax     # 0x14165bd40 <.text+0x165ad40>
14165cd78: 48 89 45 a8                 	movq	%rax, -0x58(%rbp)
14165cd7c: 48 8d 45 1f                 	leaq	0x1f(%rbp), %rax
14165cd80: 48 89 45 b0                 	movq	%rax, -0x50(%rbp)
14165cd84: 48 8d 05 85 99 af fe        	leaq	-0x150667b(%rip), %rax  # 0x140156710 <.text+0x155710>
14165cd8b: 48 89 45 b8                 	movq	%rax, -0x48(%rbp)
14165cd8f: 48 8d 05 8a 54 62 01        	leaq	0x162548a(%rip), %rax   # 0x142c82220
14165cd96: 48 89 45 c0                 	movq	%rax, -0x40(%rbp)
14165cd9a: 48 c7 45 c8 02 00 00 00     	movq	$0x2, -0x38(%rbp)
14165cda2: 48 8d 45 a0                 	leaq	-0x60(%rbp), %rax
14165cda6: 48 89 45 d0                 	movq	%rax, -0x30(%rbp)
14165cdaa: 48 c7 45 d8 02 00 00 00     	movq	$0x2, -0x28(%rbp)
14165cdb2: 48 c7 45 e0 00 00 00 00     	movq	$0x0, -0x20(%rbp)
14165cdba: 48 8d 4d f8                 	leaq	-0x8(%rbp), %rcx
14165cdbe: 48 8d 55 c0                 	leaq	-0x40(%rbp), %rdx
14165cdc2: e8 c9 d1 71 ff              	callq	0x140d79f90 <.text+0xd78f90>
14165cdc7: 48 8b 7d 00                 	movq	(%rbp), %rdi
14165cdcb: 48 8b 5d 08                 	movq	0x8(%rbp), %rbx
14165cdcf: 41 0f 10 06                 	movups	(%r14), %xmm0
14165cdd3: 0f 29 45 c0                 	movaps	%xmm0, -0x40(%rbp)
14165cdd7: 44 0f b6 75 1f              	movzbl	0x1f(%rbp), %r14d
14165cddc: 0f b6 05 26 7e fd 01        	movzbl	0x1fd7e26(%rip), %eax   # 0x143634c09
14165cde3: b9 28 00 00 00              	movl	$0x28, %ecx
14165cde8: ba 08 00 00 00              	movl	$0x8, %edx
14165cded: e8 1e 6e fb fe              	callq	0x140613c10 <.text+0x612c10>
14165cdf2: 48 85 c0                    	testq	%rax, %rax
14165cdf5: 0f 84 af 00 00 00           	je	0x14165ceaa <.text+0x165beaa>
14165cdfb: 49 89 c5                    	movq	%rax, %r13
14165cdfe: 48 c7 00 01 00 00 00        	movq	$0x1, (%rax)
14165ce05: 48 c7 40 08 01 00 00 00     	movq	$0x1, 0x8(%rax)
14165ce0d: 0f 28 45 c0                 	movaps	-0x40(%rbp), %xmm0
14165ce11: 0f 11 40 10                 	movups	%xmm0, 0x10(%rax)
14165ce15: 44 88 70 20                 	movb	%r14b, 0x20(%rax)
14165ce19: 48 89 45 c0                 	movq	%rax, -0x40(%rbp)
14165ce1d: 4c 8d 25 1c 54 62 01        	leaq	0x162541c(%rip), %r12   # 0x142c82240
14165ce24: 4c 89 65 c8                 	movq	%r12, -0x38(%rbp)
14165ce28: 48 85 db                    	testq	%rbx, %rbx
14165ce2b: 0f 88 8a 00 00 00           	js	0x14165cebb <.text+0x165bebb>
14165ce31: 74 24                       	je	0x14165ce57 <.text+0x165be57>
14165ce33: 0f b6 05 cf 7d fd 01        	movzbl	0x1fd7dcf(%rip), %eax   # 0x143634c09
14165ce3a: 41 bf 01 00 00 00           	movl	$0x1, %r15d
14165ce40: ba 01 00 00 00              	movl	$0x1, %edx
14165ce45: 48 89 d9                    	movq	%rbx, %rcx
14165ce48: e8 c3 6d fb fe              	callq	0x140613c10 <.text+0x612c10>
14165ce4d: 48 85 c0                    	testq	%rax, %rax
14165ce50: 74 6c                       	je	0x14165cebe <.text+0x165bebe>
14165ce52: 49 89 c6                    	movq	%rax, %r14
14165ce55: eb 06                       	jmp	0x14165ce5d <.text+0x165be5d>
14165ce57: 41 be 01 00 00 00           	movl	$0x1, %r14d
14165ce5d: 4c 89 f1                    	movq	%r14, %rcx
14165ce60: 48 89 fa                    	movq	%rdi, %rdx
14165ce63: 49 89 d8                    	movq	%rbx, %r8
14165ce66: e8 95 6d 2c 01              	callq	0x142923c00 <.text+0x2922c00>
14165ce6b: 48 89 1e                    	movq	%rbx, (%rsi)
14165ce6e: 4c 89 76 08                 	movq	%r14, 0x8(%rsi)
14165ce72: 48 89 5e 10                 	movq	%rbx, 0x10(%rsi)
14165ce76: 4c 89 6e 18                 	movq	%r13, 0x18(%rsi)
14165ce7a: 4c 89 66 20                 	movq	%r12, 0x20(%rsi)
14165ce7e: 48 8b 55 f8                 	movq	-0x8(%rbp), %rdx
14165ce82: 48 85 d2                    	testq	%rdx, %rdx
14165ce85: 74 0e                       	je	0x14165ce95 <.text+0x165be95>
14165ce87: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14165ce8d: 48 89 f9                    	movq	%rdi, %rcx
14165ce90: e8 8b 6d fb fe              	callq	0x140613c20 <.text+0x612c20>
14165ce95: 90                          	nop
14165ce96: 48 81 c4 a8 00 00 00        	addq	$0xa8, %rsp
14165ce9d: 5b                          	popq	%rbx
14165ce9e: 5f                          	popq	%rdi
14165ce9f: 5e                          	popq	%rsi
14165cea0: 41 5c                       	popq	%r12
14165cea2: 41 5d                       	popq	%r13
14165cea4: 41 5e                       	popq	%r14
14165cea6: 41 5f                       	popq	%r15
14165cea8: 5d                          	popq	%rbp
14165cea9: c3                          	retq
14165ceaa: b9 08 00 00 00              	movl	$0x8, %ecx
14165ceaf: ba 28 00 00 00              	movl	$0x28, %edx
14165ceb4: e8 47 c1 27 01              	callq	0x1428d9000 <.text+0x28d8000>
14165ceb9: eb 19                       	jmp	0x14165ced4 <.text+0x165bed4>
14165cebb: 45 31 ff                    	xorl	%r15d, %r15d
14165cebe: 4c 89 6d 10                 	movq	%r13, 0x10(%rbp)
14165cec2: 4c 8d 05 57 c0 43 01        	leaq	0x143c057(%rip), %r8    # 0x142a98f20
14165cec9: 4c 89 f9                    	movq	%r15, %rcx
14165cecc: 48 89 da                    	movq	%rbx, %rdx
14165cecf: e8 0f c1 27 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14165ced4: 0f 0b                       	ud2
14165ced6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
14165cee0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14165cee5: 55                          	pushq	%rbp
14165cee6: 41 57                       	pushq	%r15
14165cee8: 41 56                       	pushq	%r14
14165ceea: 41 55                       	pushq	%r13
14165ceec: 41 54                       	pushq	%r12
14165ceee: 56                          	pushq	%rsi
14165ceef: 57                          	pushq	%rdi
14165cef0: 53                          	pushq	%rbx
14165cef1: 48 83 ec 28                 	subq	$0x28, %rsp
14165cef5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14165cefc: 48 8b 55 f8                 	movq	-0x8(%rbp), %rdx
14165cf00: 48 85 d2                    	testq	%rdx, %rdx
14165cf03: 74 0f                       	je	0x14165cf14 <.text+0x165bf14>
14165cf05: 48 8b 4d 00                 	movq	(%rbp), %rcx
14165cf09: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14165cf0f: e8 0c 6d fb fe              	callq	0x140613c20 <.text+0x612c20>
14165cf14: 90                          	nop
14165cf15: 48 83 c4 28                 	addq	$0x28, %rsp
14165cf19: 5b                          	popq	%rbx
14165cf1a: 5f                          	popq	%rdi
14165cf1b: 5e                          	popq	%rsi
14165cf1c: 41 5c                       	popq	%r12
14165cf1e: 41 5d                       	popq	%r13
14165cf20: 41 5e                       	popq	%r14
14165cf22: 41 5f                       	popq	%r15
14165cf24: 5d                          	popq	%rbp
14165cf25: c3                          	retq
14165cf26: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
14165cf30: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14165cf35: 55                          	pushq	%rbp
14165cf36: 41 57                       	pushq	%r15
14165cf38: 41 56                       	pushq	%r14
14165cf3a: 41 55                       	pushq	%r13
14165cf3c: 41 54                       	pushq	%r12
14165cf3e: 56                          	pushq	%rsi
14165cf3f: 57                          	pushq	%rdi
14165cf40: 53                          	pushq	%rbx
14165cf41: 48 83 ec 28                 	subq	$0x28, %rsp
14165cf45: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14165cf4c: 48 8b 45 10                 	movq	0x10(%rbp), %rax
14165cf50: f0                          	lock
14165cf51: 48 ff 08                    	decq	(%rax)
14165cf54: 75 09                       	jne	0x14165cf5f <.text+0x165bf5f>
14165cf56: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
14165cf5a: e8 71 2a ff fe              	callq	0x14064f9d0 <.text+0x64e9d0>
14165cf5f: 90                          	nop
14165cf60: 48 83 c4 28                 	addq	$0x28, %rsp
14165cf64: 5b                          	popq	%rbx
14165cf65: 5f                          	popq	%rdi
14165cf66: 5e                          	popq	%rsi
14165cf67: 41 5c                       	popq	%r12
14165cf69: 41 5d                       	popq	%r13
14165cf6b: 41 5e                       	popq	%r14
14165cf6d: 41 5f                       	popq	%r15
14165cf6f: 5d                          	popq	%rbp
14165cf70: c3                          	retq
