
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14234bfb2: 48 8b 8d 58 02 00 00        	movq	0x258(%rbp), %rcx
14234bfb9: e8 72 52 5c fe              	callq	0x140911230 <.text+0x910230>
14234bfbe: 3c 03                       	cmpb	$0x3, %al
14234bfc0: 0f 85 d8 00 00 00           	jne	0x14234c09e <.text+0x234b09e>
14234bfc6: 48 8b 8d c8 01 00 00        	movq	0x1c8(%rbp), %rcx
14234bfcd: e8 ae 96 57 fe              	callq	0x1408c5680 <.text+0x8c4680>
14234bfd2: 84 c0                       	testb	%al, %al
14234bfd4: 0f 84 c4 00 00 00           	je	0x14234c09e <.text+0x234b09e>
14234bfda: 48 8b 95 30 03 00 00        	movq	0x330(%rbp), %rdx
14234bfe1: 8b 42 1c                    	movl	0x1c(%rdx), %eax
14234bfe4: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
14234bfe8: 89 01                       	movl	%eax, (%rcx)
14234bfea: 48 8b 3a                    	movq	(%rdx), %rdi
14234bfed: 48 8d 05 3b 32 d0 00        	leaq	0xd0323b(%rip), %rax    # 0x14304f22f
14234bff4: 48 89 85 c0 00 00 00        	movq	%rax, 0xc0(%rbp)
14234bffb: 48 c7 85 c8 00 00 00 0c 00 00 00    	movq	$0xc, 0xc8(%rbp)
14234c006: 48 8d 8d c0 00 00 00        	leaq	0xc0(%rbp), %rcx
14234c00d: e8 8e 1d 27 fe              	callq	0x1405bdda0 <.text+0x5bcda0>
14234c012: 48 bb 43 b4 4f 00 20 53 69 b1       	movabsq	$-0x4e96acdfffb04bbd, %rbx # imm = 0xB1695320004FB443
14234c01c: 48 31 c3                    	xorq	%rax, %rbx
14234c01f: 48 89 f9                    	movq	%rdi, %rcx
14234c022: 48 89 da                    	movq	%rbx, %rdx
14234c025: e8 96 7e 33 fe              	callq	0x140683ec0 <.text+0x682ec0>
14234c02a: c6 40 5d 00                 	movb	$0x0, 0x5d(%rax)
14234c02e: f6 c2 01                    	testb	$0x1, %dl
14234c031: 75 33                       	jne	0x14234c066 <.text+0x234b066>
14234c033: 48 8b 77 10                 	movq	0x10(%rdi), %rsi
14234c037: 48 3b 37                    	cmpq	(%rdi), %rsi
14234c03a: 75 0f                       	jne	0x14234c04b <.text+0x234b04b>
14234c03c: 48 8d 15 85 2f 86 00        	leaq	0x862f85(%rip), %rdx    # 0x142baefc8
14234c043: 48 89 f9                    	movq	%rdi, %rcx
14234c046: e8 85 59 2e fe              	callq	0x1406319d0 <.text+0x6309d0>
14234c04b: 48 8b 47 08                 	movq	0x8(%rdi), %rax
14234c04f: 48 6b ce 38                 	imulq	$0x38, %rsi, %rcx
14234c053: c7 04 08 01 00 00 00        	movl	$0x1, (%rax,%rcx)
14234c05a: 48 89 5c 08 30              	movq	%rbx, 0x30(%rax,%rcx)
14234c05f: 48 ff c6                    	incq	%rsi
14234c062: 48 89 77 10                 	movq	%rsi, 0x10(%rdi)
14234c066: 48 89 7d 10                 	movq	%rdi, 0x10(%rbp)
14234c06a: 48 89 5d 18                 	movq	%rbx, 0x18(%rbp)
14234c06e: 48 c7 45 20 00 00 00 00     	movq	$0x0, 0x20(%rbp)
14234c076: 48 c7 44 24 20 00 00 00 00  	movq	$0x0, 0x20(%rsp)
14234c07f: 4c 8d 05 b5 31 d0 00        	leaq	0xd031b5(%rip), %r8     # 0x14304f23b
14234c086: 48 8d 8d c0 00 00 00        	leaq	0xc0(%rbp), %rcx
14234c08d: 48 8d 55 10                 	leaq	0x10(%rbp), %rdx
14234c091: 41 b9 12 00 00 00           	movl	$0x12, %r9d
14234c097: e8 d4 28 c5 fe              	callq	0x140f9e970 <.text+0xf9d970>
14234c09c: eb 60                       	jmp	0x14234c0fe <.text+0x234b0fe>
14234c09e: 48 8b 8d 58 02 00 00        	movq	0x258(%rbp), %rcx
14234c0a5: e8 86 51 5c fe              	callq	0x140911230 <.text+0x910230>
14234c0aa: 3c 03                       	cmpb	$0x3, %al
14234c0ac: 75 50                       	jne	0x14234c0fe <.text+0x234b0fe>
14234c0ae: 48 8b 05 73 ff 2d 01        	movq	0x12dff73(%rip), %rax   # 0x14362c028
14234c0b5: 80 38 01                    	cmpb	$0x1, (%rax)
14234c0b8: 75 44                       	jne	0x14234c0fe <.text+0x234b0fe>
14234c0ba: 48 8b 95 68 03 00 00        	movq	0x368(%rbp), %rdx
14234c0c1: 8b 42 1c                    	movl	0x1c(%rdx), %eax
14234c0c4: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
14234c0c8: 89 01                       	movl	%eax, (%rcx)
14234c0ca: 48 8b 32                    	movq	(%rdx), %rsi
14234c0cd: 48 8b 7e 30                 	movq	0x30(%rsi), %rdi
14234c0d1: 48 8b 5e 40                 	movq	0x40(%rsi), %rbx
14234c0d5: 48 3b 7e 20                 	cmpq	0x20(%rsi), %rdi
14234c0d9: 75 10                       	jne	0x14234c0eb <.text+0x234b0eb>
14234c0db: 48 8d 4e 20                 	leaq	0x20(%rsi), %rcx
14234c0df: 48 8d 15 8a 91 86 00        	leaq	0x86918a(%rip), %rdx    # 0x142bb5270
14234c0e6: e8 85 67 21 fe              	callq	0x140562870 <.text+0x561870>
14234c0eb: 48 8b 46 28                 	movq	0x28(%rsi), %rax
14234c0ef: 48 89 1c f8                 	movq	%rbx, (%rax,%rdi,8)
14234c0f3: 48 ff c7                    	incq	%rdi
14234c0f6: 48 89 7e 30                 	movq	%rdi, 0x30(%rsi)
14234c0fa: 48 ff 46 40                 	incq	0x40(%rsi)
14234c0fe: 41 0f b6 44 24 01           	movzbl	0x1(%r12), %eax
