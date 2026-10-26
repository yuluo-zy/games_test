
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14234b010: 55                          	pushq	%rbp
14234b011: 41 57                       	pushq	%r15
14234b013: 41 56                       	pushq	%r14
14234b015: 41 55                       	pushq	%r13
14234b017: 41 54                       	pushq	%r12
14234b019: 56                          	pushq	%rsi
14234b01a: 57                          	pushq	%rdi
14234b01b: 53                          	pushq	%rbx
14234b01c: 48 81 ec 48 03 00 00        	subq	$0x348, %rsp            # imm = 0x348
14234b023: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14234b02b: 44 0f 29 95 b0 02 00 00     	movaps	%xmm10, 0x2b0(%rbp)
14234b033: 44 0f 29 8d a0 02 00 00     	movaps	%xmm9, 0x2a0(%rbp)
14234b03b: 44 0f 29 85 90 02 00 00     	movaps	%xmm8, 0x290(%rbp)
14234b043: 0f 29 bd 80 02 00 00        	movaps	%xmm7, 0x280(%rbp)
14234b04a: 0f 29 b5 70 02 00 00        	movaps	%xmm6, 0x270(%rbp)
14234b051: 48 c7 85 68 02 00 00 fe ff ff ff    	movq	$-0x2, 0x268(%rbp)
14234b05c: 4c 89 8d 40 02 00 00        	movq	%r9, 0x240(%rbp)
14234b063: 4c 89 85 18 02 00 00        	movq	%r8, 0x218(%rbp)
14234b06a: 49 89 cf                    	movq	%rcx, %r15
14234b06d: 48 8b 85 60 03 00 00        	movq	0x360(%rbp), %rax
14234b074: 4c 8b 22                    	movq	(%rdx), %r12
14234b077: 4c 8b 42 20                 	movq	0x20(%rdx), %r8
14234b07b: 48 8b 4a 40                 	movq	0x40(%rdx), %rcx
14234b07f: 48 89 8d 90 01 00 00        	movq	%rcx, 0x190(%rbp)
14234b086: 48 8b 4a 60                 	movq	0x60(%rdx), %rcx
14234b08a: 48 89 8d a0 01 00 00        	movq	%rcx, 0x1a0(%rbp)
14234b091: 48 8b 9a 80 00 00 00        	movq	0x80(%rdx), %rbx
14234b098: 48 8b 8a a0 00 00 00        	movq	0xa0(%rdx), %rcx
14234b09f: 48 89 8d 10 02 00 00        	movq	%rcx, 0x210(%rbp)
14234b0a6: 48 8b 8a c0 00 00 00        	movq	0xc0(%rdx), %rcx
14234b0ad: 48 89 8d 20 02 00 00        	movq	%rcx, 0x220(%rbp)
14234b0b4: 48 8b 8a e0 00 00 00        	movq	0xe0(%rdx), %rcx
14234b0bb: 48 89 8d c8 01 00 00        	movq	%rcx, 0x1c8(%rbp)
14234b0c2: 48 8b 08                    	movq	(%rax), %rcx
14234b0c5: 48 89 8d 70 01 00 00        	movq	%rcx, 0x170(%rbp)
14234b0cc: 48 8b 40 20                 	movq	0x20(%rax), %rax
14234b0d0: 48 89 85 78 01 00 00        	movq	%rax, 0x178(%rbp)
14234b0d7: 48 8b 8d 58 03 00 00        	movq	0x358(%rbp), %rcx
14234b0de: 48 8b 85 38 03 00 00        	movq	0x338(%rbp), %rax
14234b0e5: 48 8b 51 20                 	movq	0x20(%rcx), %rdx
14234b0e9: 48 89 95 d0 01 00 00        	movq	%rdx, 0x1d0(%rbp)
14234b0f0: 48 8b 51 30                 	movq	0x30(%rcx), %rdx
14234b0f4: 48 89 95 d8 01 00 00        	movq	%rdx, 0x1d8(%rbp)
14234b0fb: 8b 51 3c                    	movl	0x3c(%rcx), %edx
14234b0fe: 89 95 f4 01 00 00           	movl	%edx, 0x1f4(%rbp)
14234b104: 48 8b 51 40                 	movq	0x40(%rcx), %rdx
14234b108: 48 89 95 08 02 00 00        	movq	%rdx, 0x208(%rbp)
14234b10f: 48 8b 51 48                 	movq	0x48(%rcx), %rdx
14234b113: 48 89 95 00 02 00 00        	movq	%rdx, 0x200(%rbp)
14234b11a: 48 8b 71 68                 	movq	0x68(%rcx), %rsi
14234b11e: 48 8b 79 70                 	movq	0x70(%rcx), %rdi
14234b122: 48 8b 91 90 00 00 00        	movq	0x90(%rcx), %rdx
14234b129: 48 89 95 b0 01 00 00        	movq	%rdx, 0x1b0(%rbp)
14234b130: 48 8b 91 a0 00 00 00        	movq	0xa0(%rcx), %rdx
14234b137: 48 89 95 b8 01 00 00        	movq	%rdx, 0x1b8(%rbp)
14234b13e: 8b 91 ac 00 00 00           	movl	0xac(%rcx), %edx
14234b144: 89 95 f0 01 00 00           	movl	%edx, 0x1f0(%rbp)
14234b14a: 4c 8b a9 b0 00 00 00        	movq	0xb0(%rcx), %r13
14234b151: 48 8b 91 c0 00 00 00        	movq	0xc0(%rcx), %rdx
14234b158: 48 89 95 e0 01 00 00        	movq	%rdx, 0x1e0(%rbp)
14234b15f: 8b 91 cc 00 00 00           	movl	0xcc(%rcx), %edx
14234b165: 89 95 3c 02 00 00           	movl	%edx, 0x23c(%rbp)
14234b16b: 48 8b 91 f0 00 00 00        	movq	0xf0(%rcx), %rdx
14234b172: 48 89 95 30 02 00 00        	movq	%rdx, 0x230(%rbp)
14234b179: 48 8b 89 f8 00 00 00        	movq	0xf8(%rcx), %rcx
14234b180: 48 89 8d 88 01 00 00        	movq	%rcx, 0x188(%rbp)
14234b187: 8b 48 1c                    	movl	0x1c(%rax), %ecx
14234b18a: 48 8b 50 10                 	movq	0x10(%rax), %rdx
14234b18e: 89 0a                       	movl	%ecx, (%rdx)
