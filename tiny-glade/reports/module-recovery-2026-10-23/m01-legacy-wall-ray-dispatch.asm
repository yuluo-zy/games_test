
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408cdc60: 41 57                       	pushq	%r15
1408cdc62: 41 56                       	pushq	%r14
1408cdc64: 41 55                       	pushq	%r13
1408cdc66: 41 54                       	pushq	%r12
1408cdc68: 56                          	pushq	%rsi
1408cdc69: 57                          	pushq	%rdi
1408cdc6a: 55                          	pushq	%rbp
1408cdc6b: 53                          	pushq	%rbx
1408cdc6c: 48 81 ec 58 01 00 00        	subq	$0x158, %rsp            # imm = 0x158
1408cdc73: 44 0f 29 84 24 40 01 00 00  	movaps	%xmm8, 0x140(%rsp)
1408cdc7c: 0f 29 bc 24 30 01 00 00     	movaps	%xmm7, 0x130(%rsp)
1408cdc84: 0f 29 b4 24 20 01 00 00     	movaps	%xmm6, 0x120(%rsp)
1408cdc8c: 48 89 d3                    	movq	%rdx, %rbx
1408cdc8f: 49 89 cd                    	movq	%rcx, %r13
1408cdc92: 8b bc 24 d0 01 00 00        	movl	0x1d0(%rsp), %edi
1408cdc99: f3 0f 10 b4 24 c8 01 00 00  	movss	0x1c8(%rsp), %xmm6
1408cdca2: 48 8b 84 24 c0 01 00 00     	movq	0x1c0(%rsp), %rax
1408cdcaa: 4c 89 4c 24 70              	movq	%r9, 0x70(%rsp)
1408cdcaf: 48 89 44 24 78              	movq	%rax, 0x78(%rsp)
1408cdcb4: 4c 8d 8a 80 3d 00 00        	leaq	0x3d80(%rdx), %r9
1408cdcbb: 41 0f 10 38                 	movups	(%r8), %xmm7
1408cdcbf: 4c 89 84 24 c0 00 00 00     	movq	%r8, 0xc0(%rsp)
1408cdcc7: f2 45 0f 10 40 10           	movsd	0x10(%r8), %xmm8
1408cdccd: 41 be 04 00 00 00           	movl	$0x4, %r14d
1408cdcd3: bd 20 00 00 00              	movl	$0x20, %ebp
1408cdcd8: 45 31 ff                    	xorl	%r15d, %r15d
1408cdcdb: 4c 8d 54 24 70              	leaq	0x70(%rsp), %r10
1408cdce0: 48 8d 8c 24 c8 00 00 00     	leaq	0xc8(%rsp), %rcx
1408cdce8: 4c 8d 25 69 5b 1d 02        	leaq	0x21d5b69(%rip), %r12   # 0x142aa3858
1408cdcef: 4c 8d 9c 24 80 00 00 00     	leaq	0x80(%rsp), %r11
1408cdcf7: 48 89 44 24 58              	movq	%rax, 0x58(%rsp)
1408cdcfc: 4c 89 f8                    	movq	%r15, %rax
1408cdcff: 90                          	nop
1408cdd00: 48 81 fd b0 3c 00 00        	cmpq	$0x3cb0, %rbp           # imm = 0x3CB0
1408cdd07: 0f 84 52 01 00 00           	je	0x1408cde5f <.text+0x8cce5f>
1408cdd0d: 48 81 c5 90 03 00 00        	addq	$0x390, %rbp            # imm = 0x390
1408cdd14: 4c 8d 78 01                 	leaq	0x1(%rax), %r15
1408cdd18: 0f a3 c7                    	btl	%eax, %edi
1408cdd1b: 4c 89 f8                    	movq	%r15, %rax
1408cdd1e: 73 e0                       	jae	0x1408cdd00 <.text+0x8ccd00>
1408cdd20: 44 89 44 24 48              	movl	%r8d, 0x48(%rsp)
1408cdd25: 89 54 24 4c                 	movl	%edx, 0x4c(%rsp)
1408cdd29: 4c 89 6c 24 50              	movq	%r13, 0x50(%rsp)
1408cdd2e: 4c 8d 2c 2b                 	leaq	(%rbx,%rbp), %r13
1408cdd32: 49 81 c5 70 fc ff ff        	addq	$-0x390, %r13           # imm = 0xFC70
1408cdd39: 48 89 9c 24 c8 00 00 00     	movq	%rbx, 0xc8(%rsp)
1408cdd41: 4c 89 ac 24 d0 00 00 00     	movq	%r13, 0xd0(%rsp)
1408cdd49: 4c 89 94 24 d8 00 00 00     	movq	%r10, 0xd8(%rsp)
1408cdd51: 48 8d 94 2b a0 fc ff ff     	leaq	-0x360(%rbx,%rbp), %rdx
1408cdd59: 0f 29 bc 24 00 01 00 00     	movaps	%xmm7, 0x100(%rsp)
1408cdd61: 44 0f 13 84 24 10 01 00 00  	movlps	%xmm8, 0x110(%rsp)
1408cdd6a: 48 89 8c 24 80 00 00 00     	movq	%rcx, 0x80(%rsp)
1408cdd72: 4c 89 a4 24 88 00 00 00     	movq	%r12, 0x88(%rsp)
1408cdd7a: 48 c7 84 24 90 00 00 00 00 00 00 00 	movq	$0x0, 0x90(%rsp)
1408cdd86: c7 84 24 a0 00 00 00 00 00 00 00    	movl	$0x0, 0xa0(%rsp)
1408cdd91: c7 84 24 ac 00 00 00 00 00 00 00    	movl	$0x0, 0xac(%rsp)
1408cdd9c: 4c 89 de                    	movq	%r11, %rsi
1408cdd9f: 4c 89 5c 24 38              	movq	%r11, 0x38(%rsp)
1408cdda4: f3 0f 11 74 24 28           	movss	%xmm6, 0x28(%rsp)
1408cddaa: 48 8d 84 24 00 01 00 00     	leaq	0x100(%rsp), %rax
1408cddb2: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1408cddb7: c6 44 24 30 01              	movb	$0x1, 0x30(%rsp)
1408cddbc: 48 8d 8c 24 e0 00 00 00     	leaq	0xe0(%rsp), %rcx
1408cddc4: 4d 89 cc                    	movq	%r9, %r12
1408cddc7: 4d 89 c8                    	movq	%r9, %r8
1408cddca: 4d 89 e9                    	movq	%r13, %r9
1408cddcd: e8 fe 67 2c 00              	callq	0x140b945d0 <.text+0xb935d0>
1408cddd2: 83 bc 24 e8 00 00 00 04     	cmpl	$0x4, 0xe8(%rsp)
1408cddda: 75 14                       	jne	0x1408cddf0 <.text+0x8ccdf0>
1408cdddc: 4c 8b 6c 24 50              	movq	0x50(%rsp), %r13
1408cdde1: 4d 89 e1                    	movq	%r12, %r9
1408cdde4: 8b 54 24 4c                 	movl	0x4c(%rsp), %edx
1408cdde8: 44 8b 44 24 48              	movl	0x48(%rsp), %r8d
1408cdded: eb 54                       	jmp	0x1408cde43 <.text+0x8cce43>
1408cddef: 90                          	nop
1408cddf0: 8b 94 24 e0 00 00 00        	movl	0xe0(%rsp), %edx
1408cddf7: 44 8b 84 24 e4 00 00 00     	movl	0xe4(%rsp), %r8d
1408cddff: 4c 8b b4 24 e8 00 00 00     	movq	0xe8(%rsp), %r14
1408cde07: f3 0f 10 b4 24 f0 00 00 00  	movss	0xf0(%rsp), %xmm6
1408cde10: 48 8d 8c 24 f4 00 00 00     	leaq	0xf4(%rsp), %rcx
1408cde18: 48 8b 01                    	movq	(%rcx), %rax
1408cde1b: 48 89 44 24 60              	movq	%rax, 0x60(%rsp)
1408cde20: 8b 41 08                    	movl	0x8(%rcx), %eax
1408cde23: 89 44 24 68                 	movl	%eax, 0x68(%rsp)
1408cde27: 4c 89 f0                    	movq	%r14, %rax
1408cde2a: 48 c1 e8 20                 	shrq	$0x20, %rax
1408cde2e: 48 89 44 24 58              	movq	%rax, 0x58(%rsp)
1408cde33: 4c 89 ac 24 b8 00 00 00     	movq	%r13, 0xb8(%rsp)
1408cde3b: 4c 8b 6c 24 50              	movq	0x50(%rsp), %r13
1408cde40: 4d 89 e1                    	movq	%r12, %r9
1408cde43: 4c 8d 54 24 70              	leaq	0x70(%rsp), %r10
1408cde48: 48 8d 8c 24 c8 00 00 00     	leaq	0xc8(%rsp), %rcx
1408cde50: 4c 8d 25 01 5a 1d 02        	leaq	0x21d5a01(%rip), %r12   # 0x142aa3858
1408cde57: 49 89 f3                    	movq	%rsi, %r11
1408cde5a: e9 9d fe ff ff              	jmp	0x1408cdcfc <.text+0x8cccfc>
1408cde5f: 48 8b 8c 24 b8 00 00 00     	movq	0xb8(%rsp), %rcx
1408cde67: 48 89 8c 24 80 00 00 00     	movq	%rcx, 0x80(%rsp)
1408cde6f: 89 94 24 88 00 00 00        	movl	%edx, 0x88(%rsp)
1408cde76: 44 89 84 24 8c 00 00 00     	movl	%r8d, 0x8c(%rsp)
1408cde7e: 44 89 b4 24 90 00 00 00     	movl	%r14d, 0x90(%rsp)
1408cde86: 48 8b 44 24 58              	movq	0x58(%rsp), %rax
1408cde8b: 89 84 24 94 00 00 00        	movl	%eax, 0x94(%rsp)
1408cde92: f3 0f 11 b4 24 98 00 00 00  	movss	%xmm6, 0x98(%rsp)
1408cde9b: 48 8b 44 24 60              	movq	0x60(%rsp), %rax
1408cdea0: 48 89 84 24 9c 00 00 00     	movq	%rax, 0x9c(%rsp)
1408cdea8: 8b 44 24 68                 	movl	0x68(%rsp), %eax
1408cdeac: 89 84 24 a4 00 00 00        	movl	%eax, 0xa4(%rsp)
1408cdeb3: 41 83 fe 04                 	cmpl	$0x4, %r14d
1408cdeb7: 75 0a                       	jne	0x1408cdec3 <.text+0x8ccec3>
1408cdeb9: 41 c7 45 34 02 00 00 00     	movl	$0x2, 0x34(%r13)
1408cdec1: eb 5d                       	jmp	0x1408cdf20 <.text+0x8ccf20>
1408cdec3: e8 78 5a 2c 00              	callq	0x140b93940 <.text+0xb92940>
1408cdec8: 48 85 c0                    	testq	%rax, %rax
1408cdecb: 0f 84 94 00 00 00           	je	0x1408cdf65 <.text+0x8ccf65>
1408cded1: 48 8b 08                    	movq	(%rax), %rcx
1408cded4: 41 89 c8                    	movl	%ecx, %r8d
1408cded7: 4c 39 43 10                 	cmpq	%r8, 0x10(%rbx)
1408cdedb: 76 70                       	jbe	0x1408cdf4d <.text+0x8ccf4d>
1408cdedd: 48 8b 53 08                 	movq	0x8(%rbx), %rdx
1408cdee1: 48 85 d2                    	testq	%rdx, %rdx
1408cdee4: 74 67                       	je	0x1408cdf4d <.text+0x8ccf4d>
1408cdee6: 48 c1 e9 20                 	shrq	$0x20, %rcx
1408cdeea: 83 c9 01                    	orl	$0x1, %ecx
1408cdeed: 4f 8d 0c 80                 	leaq	(%r8,%r8,4), %r9
1408cdef1: 42 39 4c ca 20              	cmpl	%ecx, 0x20(%rdx,%r9,8)
1408cdef6: 75 55                       	jne	0x1408cdf4d <.text+0x8ccf4d>
1408cdef8: 4c 8d 84 24 90 00 00 00     	leaq	0x90(%rsp), %r8
1408cdf00: 4a 8d 0c ca                 	leaq	(%rdx,%r9,8), %rcx
1408cdf04: 48 83 c0 34                 	addq	$0x34, %rax
1408cdf08: 48 89 4c 24 20              	movq	%rcx, 0x20(%rsp)
1408cdf0d: 4c 89 e9                    	movq	%r13, %rcx
1408cdf10: 48 8b 94 24 c0 00 00 00     	movq	0xc0(%rsp), %rdx
1408cdf18: 49 89 c1                    	movq	%rax, %r9
1408cdf1b: e8 80 05 00 00              	callq	0x1408ce4a0 <.text+0x8cd4a0>
1408cdf20: 0f 28 b4 24 20 01 00 00     	movaps	0x120(%rsp), %xmm6
1408cdf28: 0f 28 bc 24 30 01 00 00     	movaps	0x130(%rsp), %xmm7
1408cdf30: 44 0f 28 84 24 40 01 00 00  	movaps	0x140(%rsp), %xmm8
1408cdf39: 48 81 c4 58 01 00 00        	addq	$0x158, %rsp            # imm = 0x158
1408cdf40: 5b                          	popq	%rbx
1408cdf41: 5d                          	popq	%rbp
1408cdf42: 5f                          	popq	%rdi
1408cdf43: 5e                          	popq	%rsi
1408cdf44: 41 5c                       	popq	%r12
1408cdf46: 41 5d                       	popq	%r13
1408cdf48: 41 5e                       	popq	%r14
1408cdf4a: 41 5f                       	popq	%r15
1408cdf4c: c3                          	retq
1408cdf4d: 48 8d 0d 8c f9 39 02        	leaq	0x239f98c(%rip), %rcx   # 0x142c6d8e0
1408cdf54: 4c 8d 05 4d 5d 1d 02        	leaq	0x21d5d4d(%rip), %r8    # 0x142aa3ca8
1408cdf5b: ba 18 00 00 00              	movl	$0x18, %edx
1408cdf60: e8 c0 a7 00 02              	callq	0x1428d8725 <.text+0x28d7725>
1408cdf65: 48 8d 0d 24 5d 1d 02        	leaq	0x21d5d24(%rip), %rcx   # 0x142aa3c90
1408cdf6c: e8 9f b3 00 02              	callq	0x1428d9310 <.text+0x28d8310>
1408cdf71: cc                          	int3
1408cdf72: cc                          	int3
1408cdf73: cc                          	int3
1408cdf74: cc                          	int3
1408cdf75: cc                          	int3
1408cdf76: cc                          	int3
1408cdf77: cc                          	int3
1408cdf78: cc                          	int3
1408cdf79: cc                          	int3
1408cdf7a: cc                          	int3
1408cdf7b: cc                          	int3
1408cdf7c: cc                          	int3
1408cdf7d: cc                          	int3
1408cdf7e: cc                          	int3
1408cdf7f: cc                          	int3
