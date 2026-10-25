
D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14093fbc0: 48 8b 11                    	movq	(%rcx), %rdx
14093fbc3: 48 83 fa 0a                 	cmpq	$0xa, %rdx
14093fbc7: 74 0a                       	je	0x14093fbd3 <.text+0x93ebd3>
14093fbc9: b0 01                       	movb	$0x1, %al
14093fbcb: 83 fa 02                    	cmpl	$0x2, %edx
14093fbce: 75 07                       	jne	0x14093fbd7 <.text+0x93ebd7>
14093fbd0: 31 c0                       	xorl	%eax, %eax
14093fbd2: c3                          	retq
14093fbd3: 0f b6 41 08                 	movzbl	0x8(%rcx), %eax
14093fbd7: c3                          	retq
14093fbd8: cc                          	int3
14093fbd9: cc                          	int3
14093fbda: cc                          	int3
14093fbdb: cc                          	int3
14093fbdc: cc                          	int3
14093fbdd: cc                          	int3
14093fbde: cc                          	int3
14093fbdf: cc                          	int3
14093fbe0: 48 8b 01                    	movq	(%rcx), %rax
14093fbe3: 48 83 f8 0a                 	cmpq	$0xa, %rax
14093fbe7: 74 08                       	je	0x14093fbf1 <.text+0x93ebf1>
14093fbe9: 83 f8 02                    	cmpl	$0x2, %eax
14093fbec: 75 0a                       	jne	0x14093fbf8 <.text+0x93ebf8>
14093fbee: b0 01                       	movb	$0x1, %al
14093fbf0: c3                          	retq
14093fbf1: 0f b6 41 08                 	movzbl	0x8(%rcx), %eax
14093fbf5: 34 01                       	xorb	$0x1, %al
14093fbf7: c3                          	retq
14093fbf8: 31 c0                       	xorl	%eax, %eax
14093fbfa: c3                          	retq
14093fbfb: cc                          	int3
14093fbfc: cc                          	int3
14093fbfd: cc                          	int3
14093fbfe: cc                          	int3
14093fbff: cc                          	int3
14093fc00: 48 8b 01                    	movq	(%rcx), %rax
14093fc03: 48 8d 50 fd                 	leaq	-0x3(%rax), %rdx
14093fc07: 48 83 fa 02                 	cmpq	$0x2, %rdx
14093fc0b: 73 03                       	jae	0x14093fc10 <.text+0x93ec10>
14093fc0d: b0 01                       	movb	$0x1, %al
14093fc0f: c3                          	retq
14093fc10: 83 f8 07                    	cmpl	$0x7, %eax
14093fc13: 75 08                       	jne	0x14093fc1d <.text+0x93ec1d>
14093fc15: 80 79 08 01                 	cmpb	$0x1, 0x8(%rcx)
14093fc19: 0f 94 c0                    	sete	%al
14093fc1c: c3                          	retq
14093fc1d: 31 c0                       	xorl	%eax, %eax
14093fc1f: c3                          	retq
14093fc20: 48 8b 01                    	movq	(%rcx), %rax
14093fc23: 48 83 c0 fe                 	addq	$-0x2, %rax
14093fc27: 48 83 f8 09                 	cmpq	$0x9, %rax
14093fc2b: ba 04 00 00 00              	movl	$0x4, %edx
14093fc30: 48 0f 42 d0                 	cmovbq	%rax, %rdx
14093fc34: b0 01                       	movb	$0x1, %al
14093fc36: 41 b8 af 00 00 00           	movl	$0xaf, %r8d
14093fc3c: 49 0f a3 d0                 	btq	%rdx, %r8
