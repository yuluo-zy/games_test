
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b2bc00: 56                          	pushq	%rsi
140b2bc01: 57                          	pushq	%rdi
140b2bc02: 53                          	pushq	%rbx
140b2bc03: 48 83 ec 50                 	subq	$0x50, %rsp
140b2bc07: 48 89 54 24 30              	movq	%rdx, 0x30(%rsp)
140b2bc0c: 49 83 78 18 00              	cmpq	$0x0, 0x18(%r8)
140b2bc11: 0f 84 04 01 00 00           	je	0x140b2bd1b <.text+0xb2ad1b>
140b2bc17: 48 89 d7                    	movq	%rdx, %rdi
140b2bc1a: 48 89 ce                    	movq	%rcx, %rsi
140b2bc1d: 49 8d 48 20                 	leaq	0x20(%r8), %rcx
140b2bc21: 48 8d 54 24 30              	leaq	0x30(%rsp), %rdx
140b2bc26: 4c 89 c3                    	movq	%r8, %rbx
140b2bc29: e8 42 51 a8 ff              	callq	0x1405b0d70 <.text+0x5afd70>
140b2bc2e: 48 8b 0b                    	movq	(%rbx), %rcx
140b2bc31: 48 8b 53 08                 	movq	0x8(%rbx), %rdx
140b2bc35: 49 89 d0                    	movq	%rdx, %r8
140b2bc38: 49 21 c0                    	andq	%rax, %r8
140b2bc3b: 48 c1 e8 39                 	shrq	$0x39, %rax
140b2bc3f: 66 0f 6e c0                 	movd	%eax, %xmm0
140b2bc43: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140b2bc47: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140b2bc4c: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140b2bc51: 48 8d 81 68 ff ff ff        	leaq	-0x98(%rcx), %rax
140b2bc58: 45 31 c9                    	xorl	%r9d, %r9d
140b2bc5b: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
140b2bc5f: f3 42 0f 6f 14 01           	movdqu	(%rcx,%r8), %xmm2
140b2bc65: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
140b2bc69: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
140b2bc6d: 66 44 0f d7 d3              	pmovmskb	%xmm3, %r10d
140b2bc72: 45 85 d2                    	testl	%r10d, %r10d
140b2bc75: 74 29                       	je	0x140b2bca0 <.text+0xb2aca0>
140b2bc77: f3 45 0f bc da              	tzcntl	%r10d, %r11d
140b2bc7c: 4d 01 c3                    	addq	%r8, %r11
140b2bc7f: 49 21 d3                    	andq	%rdx, %r11
140b2bc82: 49 f7 db                    	negq	%r11
140b2bc85: 4d 69 db 98 00 00 00        	imulq	$0x98, %r11, %r11
140b2bc8c: 4a 39 3c 18                 	cmpq	%rdi, (%rax,%r11)
140b2bc90: 74 2c                       	je	0x140b2bcbe <.text+0xb2acbe>
140b2bc92: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
140b2bc96: 66 45 21 d3                 	andw	%r10w, %r11w
140b2bc9a: 45 89 da                    	movl	%r11d, %r10d
140b2bc9d: 75 d8                       	jne	0x140b2bc77 <.text+0xb2ac77>
140b2bc9f: 90                          	nop
140b2bca0: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
140b2bca4: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
140b2bca9: 45 85 d2                    	testl	%r10d, %r10d
140b2bcac: 75 6d                       	jne	0x140b2bd1b <.text+0xb2ad1b>
140b2bcae: 4d 01 c8                    	addq	%r9, %r8
140b2bcb1: 49 83 c0 10                 	addq	$0x10, %r8
140b2bcb5: 49 83 c1 10                 	addq	$0x10, %r9
140b2bcb9: 49 21 d0                    	andq	%rdx, %r8
140b2bcbc: eb a1                       	jmp	0x140b2bc5f <.text+0xb2ac5f>
140b2bcbe: 48 83 c6 30                 	addq	$0x30, %rsi
140b2bcc2: 4a 8b 44 19 a0              	movq	-0x60(%rcx,%r11), %rax
140b2bcc7: 4a 8b 54 19 a8              	movq	-0x58(%rcx,%r11), %rdx
140b2bccc: 4c 8d 04 10                 	leaq	(%rax,%rdx), %r8
140b2bcd0: 49 ff c0                    	incq	%r8
140b2bcd3: 66 0f 6f 00                 	movdqa	(%rax), %xmm0
140b2bcd7: 66 44 0f d7 c8              	pmovmskb	%xmm0, %r9d
140b2bcdc: 41 f7 d1                    	notl	%r9d
140b2bcdf: 4a 8b 54 19 b8              	movq	-0x48(%rcx,%r11), %rdx
140b2bce4: 48 89 74 24 20              	movq	%rsi, 0x20(%rsp)
140b2bce9: 48 8d 4c 24 20              	leaq	0x20(%rsp), %rcx
140b2bcee: 48 89 4c 24 28              	movq	%rcx, 0x28(%rsp)
140b2bcf3: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
140b2bcf8: 48 83 c0 10                 	addq	$0x10, %rax
140b2bcfc: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
140b2bd01: 4c 89 44 24 40              	movq	%r8, 0x40(%rsp)
140b2bd06: 66 44 89 4c 24 48           	movw	%r9w, 0x48(%rsp)
140b2bd0c: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140b2bd11: 4c 8d 44 24 28              	leaq	0x28(%rsp), %r8
140b2bd16: e8 45 a5 04 00              	callq	0x140b76260 <.text+0xb75260>
140b2bd1b: 90                          	nop
140b2bd1c: 48 83 c4 50                 	addq	$0x50, %rsp
140b2bd20: 5b                          	popq	%rbx
140b2bd21: 5f                          	popq	%rdi
140b2bd22: 5e                          	popq	%rsi
140b2bd23: c3                          	retq
