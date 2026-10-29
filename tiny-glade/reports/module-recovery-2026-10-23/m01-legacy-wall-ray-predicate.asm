
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408cab80: 56                          	pushq	%rsi
1408cab81: 57                          	pushq	%rdi
1408cab82: 48 83 ec 28                 	subq	$0x28, %rsp
1408cab86: 48 89 ce                    	movq	%rcx, %rsi
1408cab89: 48 8b 39                    	movq	(%rcx), %rdi
1408cab8c: 48 8b 49 08                 	movq	0x8(%rcx), %rcx
1408cab90: e8 ab 8d 2c 00              	callq	0x140b93940 <.text+0xb92940>
1408cab95: 48 85 c0                    	testq	%rax, %rax
1408cab98: 74 72                       	je	0x1408cac0c <.text+0x8c9c0c>
1408cab9a: 48 8b 00                    	movq	(%rax), %rax
1408cab9d: 89 c1                       	movl	%eax, %ecx
1408cab9f: 48 39 4f 10                 	cmpq	%rcx, 0x10(%rdi)
1408caba3: 76 4f                       	jbe	0x1408cabf4 <.text+0x8c9bf4>
1408caba5: 48 8b 57 08                 	movq	0x8(%rdi), %rdx
1408caba9: 48 85 d2                    	testq	%rdx, %rdx
1408cabac: 74 46                       	je	0x1408cabf4 <.text+0x8c9bf4>
1408cabae: 48 c1 e8 20                 	shrq	$0x20, %rax
1408cabb2: 83 c8 01                    	orl	$0x1, %eax
1408cabb5: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
1408cabb9: 39 44 ca 20                 	cmpl	%eax, 0x20(%rdx,%rcx,8)
1408cabbd: 75 35                       	jne	0x1408cabf4 <.text+0x8c9bf4>
1408cabbf: 48 8d 0c ca                 	leaq	(%rdx,%rcx,8), %rcx
1408cabc3: 0f b6 11                    	movzbl	(%rcx), %edx
1408cabc6: b0 01                       	movb	$0x1, %al
1408cabc8: 83 fa 08                    	cmpl	$0x8, %edx
1408cabcb: 77 20                       	ja	0x1408cabed <.text+0x8c9bed>
1408cabcd: 41 b8 05 01 00 00           	movl	$0x105, %r8d            # imm = 0x105
1408cabd3: 41 0f a3 d0                 	btl	%edx, %r8d
1408cabd7: 73 14                       	jae	0x1408cabed <.text+0x8c9bed>
1408cabd9: 48 8b 56 10                 	movq	0x10(%rsi), %rdx
1408cabdd: 83 3a 01                    	cmpl	$0x1, (%rdx)
1408cabe0: 75 0b                       	jne	0x1408cabed <.text+0x8c9bed>
1408cabe2: 48 8b 41 08                 	movq	0x8(%rcx), %rax
1408cabe6: 48 39 42 08                 	cmpq	%rax, 0x8(%rdx)
1408cabea: 0f 95 c0                    	setne	%al
1408cabed: 48 83 c4 28                 	addq	$0x28, %rsp
1408cabf1: 5f                          	popq	%rdi
1408cabf2: 5e                          	popq	%rsi
1408cabf3: c3                          	retq
1408cabf4: 48 8d 0d e5 2c 3a 02        	leaq	0x23a2ce5(%rip), %rcx   # 0x142c6d8e0
1408cabfb: 4c 8d 05 76 90 1d 02        	leaq	0x21d9076(%rip), %r8    # 0x142aa3c78
1408cac02: ba 18 00 00 00              	movl	$0x18, %edx
1408cac07: e8 19 db 00 02              	callq	0x1428d8725 <.text+0x28d7725>
1408cac0c: 48 8d 0d 4d 90 1d 02        	leaq	0x21d904d(%rip), %rcx   # 0x142aa3c60
1408cac13: e8 f8 e6 00 02              	callq	0x1428d9310 <.text+0x28d8310>
1408cac18: cc                          	int3
1408cac19: cc                          	int3
1408cac1a: cc                          	int3
1408cac1b: cc                          	int3
1408cac1c: cc                          	int3
1408cac1d: cc                          	int3
1408cac1e: cc                          	int3
1408cac1f: cc                          	int3
