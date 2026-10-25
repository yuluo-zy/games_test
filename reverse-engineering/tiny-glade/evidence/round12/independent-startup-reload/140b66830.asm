
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b66830: 55                          	pushq	%rbp
140b66831: 56                          	pushq	%rsi
140b66832: 53                          	pushq	%rbx
140b66833: 48 81 ec d0 00 00 00        	subq	$0xd0, %rsp
140b6683a: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140b66842: 48 c7 45 48 fe ff ff ff     	movq	$-0x2, 0x48(%rbp)
140b6684a: 4c 89 45 38                 	movq	%r8, 0x38(%rbp)
140b6684e: 88 d3                       	movb	%dl, %bl
140b66850: 48 89 ce                    	movq	%rcx, %rsi
140b66853: 48 83 c1 38                 	addq	$0x38, %rcx
140b66857: c6 45 47 01                 	movb	$0x1, 0x47(%rbp)
140b6685b: 48 ba 84 a7 2b 96 51 d2 1b 8e       	movabsq	$-0x71e42dae69d4587c, %rdx # imm = 0x8E1BD251962BA784
140b66865: 49 b8 9b 52 e6 c0 74 23 78 6a       	movabsq	$0x6a782374c0e6529b, %r8 # imm = 0x6A782374C0E6529B
140b6686f: e8 7c 6e 57 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
140b66874: 48 85 c0                    	testq	%rax, %rax
140b66877: 0f 84 a3 00 00 00           	je	0x140b66920 <.text+0xb65920>
140b6687d: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
140b66884: 0f 86 96 00 00 00           	jbe	0x140b66920 <.text+0xb65920>
140b6688a: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
140b66891: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
140b66895: 48 85 c9                    	testq	%rcx, %rcx
140b66898: 0f 84 82 00 00 00           	je	0x140b66920 <.text+0xb65920>
140b6689e: 48 8b 86 e0 01 00 00        	movq	0x1e0(%rsi), %rax
140b668a5: 48 f7 d1                    	notq	%rcx
140b668a8: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140b668ac: 48 c1 e1 05                 	shlq	$0x5, %rcx
140b668b0: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
140b668b6: 74 68                       	je	0x140b66920 <.text+0xb65920>
140b668b8: 48 01 c8                    	addq	%rcx, %rax
140b668bb: 48 8b 48 10                 	movq	0x10(%rax), %rcx
140b668bf: 8b 96 b0 05 00 00           	movl	0x5b0(%rsi), %edx
140b668c5: 89 50 5c                    	movl	%edx, 0x5c(%rax)
140b668c8: 48 8b 45 38                 	movq	0x38(%rbp), %rax
140b668cc: 0f 10 40 60                 	movups	0x60(%rax), %xmm0
140b668d0: 0f 29 45 00                 	movaps	%xmm0, (%rbp)
140b668d4: 0f 10 40 50                 	movups	0x50(%rax), %xmm0
140b668d8: 0f 29 45 f0                 	movaps	%xmm0, -0x10(%rbp)
140b668dc: 0f 10 40 40                 	movups	0x40(%rax), %xmm0
140b668e0: 0f 29 45 e0                 	movaps	%xmm0, -0x20(%rbp)
140b668e4: 0f 10 00                    	movups	(%rax), %xmm0
140b668e7: 0f 10 48 10                 	movups	0x10(%rax), %xmm1
140b668eb: 0f 10 50 20                 	movups	0x20(%rax), %xmm2
140b668ef: 0f 10 58 30                 	movups	0x30(%rax), %xmm3
140b668f3: 0f 29 5d d0                 	movaps	%xmm3, -0x30(%rbp)
140b668f7: 0f 29 55 c0                 	movaps	%xmm2, -0x40(%rbp)
140b668fb: 0f 29 4d b0                 	movaps	%xmm1, -0x50(%rbp)
140b668ff: 0f 29 45 a0                 	movaps	%xmm0, -0x60(%rbp)
140b66903: c6 45 47 00                 	movb	$0x0, 0x47(%rbp)
140b66907: 4c 8d 45 a0                 	leaq	-0x60(%rbp), %r8
140b6690b: 89 da                       	movl	%ebx, %edx
140b6690d: e8 de 99 b3 00              	callq	0x1416a02f0 <.text+0x169f2f0>
140b66912: 48 89 f0                    	movq	%rsi, %rax
140b66915: 48 81 c4 d0 00 00 00        	addq	$0xd0, %rsp
140b6691c: 5b                          	popq	%rbx
140b6691d: 5e                          	popq	%rsi
140b6691e: 5d                          	popq	%rbp
140b6691f: c3                          	retq
140b66920: 48 8d 05 99 c3 f8 01        	leaq	0x1f8c399(%rip), %rax   # 0x142af2cc0
140b66927: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
140b6692b: 48 c7 45 20 27 00 00 00     	movq	$0x27, 0x20(%rbp)
140b66933: 48 8d 45 18                 	leaq	0x18(%rbp), %rax
140b66937: 48 89 45 28                 	movq	%rax, 0x28(%rbp)
140b6693b: 48 8d 05 4e 35 5a ff        	leaq	-0xa5cab2(%rip), %rax   # 0x140109e90 <.text+0x108e90>
140b66942: 48 89 45 30                 	movq	%rax, 0x30(%rbp)
140b66946: 48 8d 05 a3 c4 f8 01        	leaq	0x1f8c4a3(%rip), %rax   # 0x142af2df0
140b6694d: 48 89 45 a0                 	movq	%rax, -0x60(%rbp)
140b66951: 48 c7 45 a8 02 00 00 00     	movq	$0x2, -0x58(%rbp)
140b66959: 48 c7 45 c0 00 00 00 00     	movq	$0x0, -0x40(%rbp)
140b66961: 48 8d 45 28                 	leaq	0x28(%rbp), %rax
140b66965: 48 89 45 b0                 	movq	%rax, -0x50(%rbp)
140b66969: 48 c7 45 b8 01 00 00 00     	movq	$0x1, -0x48(%rbp)
140b66971: c6 45 47 01                 	movb	$0x1, 0x47(%rbp)
140b66975: 48 8d 15 2c c3 f8 01        	leaq	0x1f8c32c(%rip), %rdx   # 0x142af2ca8
140b6697c: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140b66980: e8 0b 2a d7 01              	callq	0x1428d9390 <.text+0x28d8390>
140b66985: 0f 0b                       	ud2
140b66987: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140b66990: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b66995: 55                          	pushq	%rbp
140b66996: 56                          	pushq	%rsi
140b66997: 53                          	pushq	%rbx
140b66998: 48 83 ec 20                 	subq	$0x20, %rsp
140b6699c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b669a3: 80 7d 47 00                 	cmpb	$0x0, 0x47(%rbp)
140b669a7: 75 08                       	jne	0x140b669b1 <.text+0xb659b1>
140b669a9: 48 83 c4 20                 	addq	$0x20, %rsp
140b669ad: 5b                          	popq	%rbx
140b669ae: 5e                          	popq	%rsi
140b669af: 5d                          	popq	%rbp
140b669b0: c3                          	retq
140b669b1: 48 8b 4d 38                 	movq	0x38(%rbp), %rcx
140b669b5: e8 c6 2e 4c ff              	callq	0x140029880 <.text+0x28880>
140b669ba: eb ed                       	jmp	0x140b669a9 <.text+0xb659a9>
