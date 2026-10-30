
B版本内容/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408ca9a0: 56                          	pushq	%rsi
1408ca9a1: 57                          	pushq	%rdi
1408ca9a2: 48 83 ec 28                 	subq	$0x28, %rsp
1408ca9a6: 48 89 ce                    	movq	%rcx, %rsi
1408ca9a9: 48 8b 39                    	movq	(%rcx), %rdi
1408ca9ac: 48 8b 49 08                 	movq	0x8(%rcx), %rcx
1408ca9b0: e8 8b 8f 2c 00              	callq	0x140b93940 <.text+0xb92940>
1408ca9b5: 48 85 c0                    	testq	%rax, %rax
1408ca9b8: 74 72                       	je	0x1408caa2c <.text+0x8c9a2c>
1408ca9ba: 48 8b 00                    	movq	(%rax), %rax
1408ca9bd: 89 c1                       	movl	%eax, %ecx
1408ca9bf: 48 39 4f 10                 	cmpq	%rcx, 0x10(%rdi)
1408ca9c3: 76 4f                       	jbe	0x1408caa14 <.text+0x8c9a14>
1408ca9c5: 48 8b 57 08                 	movq	0x8(%rdi), %rdx
1408ca9c9: 48 85 d2                    	testq	%rdx, %rdx
1408ca9cc: 74 46                       	je	0x1408caa14 <.text+0x8c9a14>
1408ca9ce: 48 c1 e8 20                 	shrq	$0x20, %rax
1408ca9d2: 83 c8 01                    	orl	$0x1, %eax
1408ca9d5: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
1408ca9d9: 39 44 ca 20                 	cmpl	%eax, 0x20(%rdx,%rcx,8)
1408ca9dd: 75 35                       	jne	0x1408caa14 <.text+0x8c9a14>
1408ca9df: 48 8d 0c ca                 	leaq	(%rdx,%rcx,8), %rcx
1408ca9e3: 0f b6 11                    	movzbl	(%rcx), %edx
1408ca9e6: b0 01                       	movb	$0x1, %al
1408ca9e8: 83 fa 08                    	cmpl	$0x8, %edx
1408ca9eb: 77 20                       	ja	0x1408caa0d <.text+0x8c9a0d>
1408ca9ed: 41 b8 07 01 00 00           	movl	$0x107, %r8d            # imm = 0x107
1408ca9f3: 41 0f a3 d0                 	btl	%edx, %r8d
1408ca9f7: 73 14                       	jae	0x1408caa0d <.text+0x8c9a0d>
1408ca9f9: 48 8b 56 10                 	movq	0x10(%rsi), %rdx
1408ca9fd: 83 3a 01                    	cmpl	$0x1, (%rdx)
1408caa00: 75 0b                       	jne	0x1408caa0d <.text+0x8c9a0d>
1408caa02: 48 8b 41 08                 	movq	0x8(%rcx), %rax
1408caa06: 48 39 42 08                 	cmpq	%rax, 0x8(%rdx)
1408caa0a: 0f 95 c0                    	setne	%al
1408caa0d: 48 83 c4 28                 	addq	$0x28, %rsp
1408caa11: 5f                          	popq	%rdi
1408caa12: 5e                          	popq	%rsi
1408caa13: c3                          	retq
1408caa14: 48 8d 0d c5 2e 3a 02        	leaq	0x23a2ec5(%rip), %rcx   # 0x142c6d8e0
1408caa1b: 4c 8d 05 e6 92 1d 02        	leaq	0x21d92e6(%rip), %r8    # 0x142aa3d08
1408caa22: ba 18 00 00 00              	movl	$0x18, %edx
1408caa27: e8 f9 dc 00 02              	callq	0x1428d8725 <.text+0x28d7725>
1408caa2c: 48 8d 0d bd 92 1d 02        	leaq	0x21d92bd(%rip), %rcx   # 0x142aa3cf0
1408caa33: e8 d8 e8 00 02              	callq	0x1428d9310 <.text+0x28d8310>
1408caa38: cc                          	int3
