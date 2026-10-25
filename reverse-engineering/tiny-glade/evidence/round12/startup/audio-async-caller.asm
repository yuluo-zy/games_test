
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140593b70: 55                          	pushq	%rbp
140593b71: 41 57                       	pushq	%r15
140593b73: 41 56                       	pushq	%r14
140593b75: 41 55                       	pushq	%r13
140593b77: 41 54                       	pushq	%r12
140593b79: 56                          	pushq	%rsi
140593b7a: 57                          	pushq	%rdi
140593b7b: 53                          	pushq	%rbx
140593b7c: b8 38 57 00 00              	movl	$0x5738, %eax           # imm = 0x5738
140593b81: e8 ba 6e 30 02              	callq	0x14289aa40 <.text+0x2899a40>
140593b86: 48 29 c4                    	subq	%rax, %rsp
140593b89: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140593b91: 44 0f 29 85 a0 56 00 00     	movaps	%xmm8, 0x56a0(%rbp)
140593b99: 0f 29 bd 90 56 00 00        	movaps	%xmm7, 0x5690(%rbp)
140593ba0: 0f 29 b5 80 56 00 00        	movaps	%xmm6, 0x5680(%rbp)
140593ba7: 48 83 e4 80                 	andq	$-0x80, %rsp
140593bab: 48 89 e3                    	movq	%rsp, %rbx
140593bae: 48 89 ab f0 56 00 00        	movq	%rbp, 0x56f0(%rbx)
140593bb5: 48 c7 85 78 56 00 00 fe ff ff ff    	movq	$-0x2, 0x5678(%rbp)
140593bc0: 4c 89 c6                    	movq	%r8, %rsi
140593bc3: 48 89 8b d0 00 00 00        	movq	%rcx, 0xd0(%rbx)
140593bca: 48 8b 85 20 57 00 00        	movq	0x5720(%rbp), %rax
140593bd1: 4c 89 83 b0 02 00 00        	movq	%r8, 0x2b0(%rbx)
140593bd8: 4c 89 8b 38 01 00 00        	movq	%r9, 0x138(%rbx)
140593bdf: 48 89 83 40 01 00 00        	movq	%rax, 0x140(%rbx)
140593be6: 48 89 93 60 01 00 00        	movq	%rdx, 0x160(%rbx)
140593bed: 40 8a ba 49 01 00 00        	movb	0x149(%rdx), %dil
140593bf4: 48 8d 05 fd ba 46 02        	leaq	0x246bafd(%rip), %rax   # 0x1429ff6f8
140593bfb: 48 89 83 c0 3a 00 00        	movq	%rax, 0x3ac0(%rbx)
140593c02: 48 c7 83 c8 3a 00 00 0a 00 00 00    	movq	$0xa, 0x3ac8(%rbx)
140593c0d: c6 43 7a 01                 	movb	$0x1, 0x7a(%rbx)
140593c11: c6 43 7b 01                 	movb	$0x1, 0x7b(%rbx)
140593c15: c6 43 7c 01                 	movb	$0x1, 0x7c(%rbx)
140593c19: 48 8d 8b 80 05 00 00        	leaq	0x580(%rbx), %rcx
140593c20: ba 01 00 00 00              	movl	$0x1, %edx
140593c25: e8 16 13 e3 ff              	callq	0x1403c4f40 <.text+0x3c3f40>
140593c2a: 4c 8b b3 80 05 00 00        	movq	0x580(%rbx), %r14
140593c31: 4c 8b a3 88 05 00 00        	movq	0x588(%rbx), %r12
140593c38: 4c 8b bb 90 05 00 00        	movq	0x590(%rbx), %r15
140593c3f: 4c 8b ab 98 05 00 00        	movq	0x598(%rbx), %r13
140593c46: 4c 89 b3 e0 2f 00 00        	movq	%r14, 0x2fe0(%rbx)
140593c4d: 4c 89 a3 e8 2f 00 00        	movq	%r12, 0x2fe8(%rbx)
140593c54: 4c 89 bb 00 25 00 00        	movq	%r15, 0x2500(%rbx)
140593c5b: 4c 89 ab 08 25 00 00        	movq	%r13, 0x2508(%rbx)
140593c62: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140593c6c: 48 89 83 90 05 00 00        	movq	%rax, 0x590(%rbx)
140593c73: 48 c7 83 80 05 00 00 00 00 00 00    	movq	$0x0, 0x580(%rbx)
140593c7e: c6 83 a8 05 00 00 00        	movb	$0x0, 0x5a8(%rbx)
140593c85: 0f b6 05 7d 0f 0a 03        	movzbl	0x30a0f7d(%rip), %eax   # 0x143634c09
140593c8c: b9 0a 00 00 00              	movl	$0xa, %ecx
140593c91: ba 01 00 00 00              	movl	$0x1, %edx
140593c96: e8 75 ff 07 00              	callq	0x140613c10 <.text+0x612c10>
140593c9b: 48 89 83 80 00 00 00        	movq	%rax, 0x80(%rbx)
140593ca2: 48 85 c0                    	testq	%rax, %rax
140593ca5: 0f 84 24 59 00 00           	je	0x1405995cf <.text+0x5985cf>
140593cab: 48 b8 61 75 64 69 6f 5f 69 6e       	movabsq	$0x6e695f6f69647561, %rax # imm = 0x6E695F6F69647561
140593cb5: 48 8b 8b 80 00 00 00        	movq	0x80(%rbx), %rcx
140593cbc: 48 89 01                    	movq	%rax, (%rcx)
140593cbf: 66 c7 41 08 69 74           	movw	$0x7469, 0x8(%rcx)      # imm = 0x7469
140593cc5: 48 c7 83 a0 45 00 00 0a 00 00 00    	movq	$0xa, 0x45a0(%rbx)
140593cd0: 48 89 8b a8 45 00 00        	movq	%rcx, 0x45a8(%rbx)
140593cd7: 48 c7 83 b0 45 00 00 0a 00 00 00    	movq	$0xa, 0x45b0(%rbx)
140593ce2: 48 8d 8b e0 19 00 00        	leaq	0x19e0(%rbx), %rcx
140593ce9: 48 8d 93 80 05 00 00        	leaq	0x580(%rbx), %rdx
140593cf0: 4c 8d 83 a0 45 00 00        	leaq	0x45a0(%rbx), %r8
140593cf7: e8 f4 1b 7c 00              	callq	0x140d558f0 <.text+0xd548f0>
140593cfc: 40 88 bb 90 05 00 00        	movb	%dil, 0x590(%rbx)
140593d03: 4c 89 b3 80 05 00 00        	movq	%r14, 0x580(%rbx)
140593d0a: 4c 89 a3 88 05 00 00        	movq	%r12, 0x588(%rbx)
140593d11: c6 43 6e 00                 	movb	$0x0, 0x6e(%rbx)
140593d15: 48 8d bb a0 45 00 00        	leaq	0x45a0(%rbx), %rdi
140593d1c: 4c 8d b3 e0 19 00 00        	leaq	0x19e0(%rbx), %r14
140593d23: 4c 8d 83 80 05 00 00        	leaq	0x580(%rbx), %r8
140593d2a: 48 89 f9                    	movq	%rdi, %rcx
140593d2d: 4c 89 f2                    	movq	%r14, %rdx
140593d30: e8 7b 80 cf ff              	callq	0x14028bdb0 <.text+0x28adb0>
140593d35: 48 83 bb a0 45 00 00 00     	cmpq	$0x0, 0x45a0(%rbx)
140593d3d: 0f 84 a7 58 00 00           	je	0x1405995ea <.text+0x5985ea>
140593d43: 48 8b 8b b0 45 00 00        	movq	0x45b0(%rbx), %rcx
140593d4a: 48 89 8b 90 05 00 00        	movq	%rcx, 0x590(%rbx)
140593d51: f3 0f 6f 83 a0 45 00 00     	movdqu	0x45a0(%rbx), %xmm0
140593d59: 66 0f 7f 83 80 05 00 00     	movdqa	%xmm0, 0x580(%rbx)
140593d61: ff 15 89 e6 bf 02           	callq	*0x2bfe689(%rip)        # 0x1431923f0
