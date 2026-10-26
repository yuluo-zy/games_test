
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14227bb70: 55                          	pushq	%rbp
14227bb71: 56                          	pushq	%rsi
14227bb72: 57                          	pushq	%rdi
14227bb73: 48 81 ec e0 00 00 00        	subq	$0xe0, %rsp
14227bb7a: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14227bb82: 48 c7 45 58 fe ff ff ff     	movq	$-0x2, 0x58(%rbp)
14227bb8a: 4c 89 c7                    	movq	%r8, %rdi
14227bb8d: 48 89 d6                    	movq	%rdx, %rsi
14227bb90: 48 8b 01                    	movq	(%rcx), %rax
14227bb93: 48 8b 49 08                 	movq	0x8(%rcx), %rcx
14227bb97: 48 8b 10                    	movq	(%rax), %rdx
14227bb9a: 45 31 c0                    	xorl	%r8d, %r8d
14227bb9d: 49 89 d1                    	movq	%rdx, %r9
14227bba0: 4c 2b 49 18                 	subq	0x18(%rcx), %r9
14227bba4: 4d 0f 42 c8                 	cmovbq	%r8, %r9
14227bba8: 48 2b 51 38                 	subq	0x38(%rcx), %rdx
14227bbac: 49 0f 42 d0                 	cmovbq	%r8, %rdx
14227bbb0: 4c 8b 51 30                 	movq	0x30(%rcx), %r10
14227bbb4: 49 29 d2                    	subq	%rdx, %r10
14227bbb7: 4d 0f 42 d0                 	cmovbq	%r8, %r10
14227bbbb: 0f 97 c2                    	seta	%dl
14227bbbe: 4c 8b 59 10                 	movq	0x10(%rcx), %r11
14227bbc2: 4d 29 cb                    	subq	%r9, %r11
14227bbc5: 4d 0f 42 d8                 	cmovbq	%r8, %r11
14227bbc9: 48 8b 49 40                 	movq	0x40(%rcx), %rcx
14227bbcd: 41 0f 97 c0                 	seta	%r8b
14227bbd1: 4d 01 d3                    	addq	%r10, %r11
14227bbd4: 49 89 c9                    	movq	%rcx, %r9
14227bbd7: 4d 29 d9                    	subq	%r11, %r9
14227bbda: 4c 89 08                    	movq	%r9, (%rax)
14227bbdd: 41 08 d0                    	orb	%dl, %r8b
14227bbe0: 0f 84 0a 01 00 00           	je	0x14227bcf0 <.text+0x227acf0>
14227bbe6: 48 89 08                    	movq	%rcx, (%rax)
14227bbe9: b1 14                       	movb	$0x14, %cl
14227bbeb: e8 a0 d1 76 fe              	callq	0x1409e8d90 <.text+0x9e7d90>
14227bbf0: 66 89 45 f4                 	movw	%ax, -0xc(%rbp)
14227bbf4: c7 45 f0 c9 00 00 00        	movl	$0xc9, -0x10(%rbp)
14227bbfb: 48 8b 07                    	movq	(%rdi), %rax
14227bbfe: 8b 88 64 01 00 00           	movl	0x164(%rax), %ecx
14227bc04: 89 4d 50                    	movl	%ecx, 0x50(%rbp)
14227bc07: 48 8b 80 5c 01 00 00        	movq	0x15c(%rax), %rax
14227bc0e: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
14227bc12: 48 8d 4d b8                 	leaq	-0x48(%rbp), %rcx
14227bc16: 48 8d 55 f0                 	leaq	-0x10(%rbp), %rdx
14227bc1a: 4c 8d 45 48                 	leaq	0x48(%rbp), %r8
14227bc1e: e8 cd 28 9d ff              	callq	0x141c4e4f0 <.text+0x1c4d4f0>
14227bc23: 66 c7 45 e8 00 12           	movw	$0x1200, -0x18(%rbp)    # imm = 0x1200
14227bc29: c7 45 a0 0a 00 00 00        	movl	$0xa, -0x60(%rbp)
14227bc30: 8b 46 1c                    	movl	0x1c(%rsi), %eax
14227bc33: 48 8b 4e 10                 	movq	0x10(%rsi), %rcx
14227bc37: 89 01                       	movl	%eax, (%rcx)
14227bc39: 48 8b 36                    	movq	(%rsi), %rsi
14227bc3c: 48 8b 46 40                 	movq	0x40(%rsi), %rax
