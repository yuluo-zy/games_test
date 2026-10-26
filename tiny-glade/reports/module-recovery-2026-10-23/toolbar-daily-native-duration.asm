
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140d63bd0: 55                          	pushq	%rbp
140d63bd1: 48 89 e5                    	movq	%rsp, %rbp
140d63bd4: 4c 8b 12                    	movq	(%rdx), %r10
140d63bd7: 44 89 c0                    	movl	%r8d, %eax
140d63bda: 49 c1 e1 20                 	shlq	$0x20, %r9
140d63bde: 49 09 c1                    	orq	%rax, %r9
140d63be1: 4c 89 d0                    	movq	%r10, %rax
140d63be4: 4c 29 c8                    	subq	%r9, %rax
140d63be7: 48 f7 d8                    	negq	%rax
140d63bea: 45 31 c0                    	xorl	%r8d, %r8d
140d63bed: 4d 29 ca                    	subq	%r9, %r10
140d63bf0: 41 0f 9c c0                 	setl	%r8b
140d63bf4: 4c 0f 4c d0                 	cmovlq	%rax, %r10
140d63bf8: 48 ba bd 42 7a e5 d5 94 bf d6       	movabsq	$-0x29406b2a1a85bd43, %rdx # imm = 0xD6BF94D5E57A42BD
140d63c02: 4c 89 d0                    	movq	%r10, %rax
140d63c05: 48 f7 e2                    	mulq	%rdx
140d63c08: 48 c1 ea 17                 	shrq	$0x17, %rdx
140d63c0c: 69 c2 80 96 98 00           	imull	$0x989680, %edx, %eax   # imm = 0x989680
140d63c12: 41 29 c2                    	subl	%eax, %r10d
140d63c15: 41 6b c2 64                 	imull	$0x64, %r10d, %eax
140d63c19: 48 89 51 08                 	movq	%rdx, 0x8(%rcx)
140d63c1d: 89 41 10                    	movl	%eax, 0x10(%rcx)
140d63c20: 4c 89 01                    	movq	%r8, (%rcx)
140d63c23: 48 89 c8                    	movq	%rcx, %rax
140d63c26: 5d                          	popq	%rbp
140d63c27: c3                          	retq
