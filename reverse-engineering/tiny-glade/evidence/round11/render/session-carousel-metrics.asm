
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408f6df0: 00 00                       	addb	%al, (%rax)
1408f6df2: 48 8b 85 08 10 00 00        	movq	0x1008(%rbp), %rax
1408f6df9: 4c 8b 70 30                 	movq	0x30(%rax), %r14
1408f6dfd: 49 8d 7e 10                 	leaq	0x10(%r14), %rdi
1408f6e01: b1 01                       	movb	$0x1, %cl
1408f6e03: 31 c0                       	xorl	%eax, %eax
1408f6e05: f0                          	lock
1408f6e06: 41 0f b0 4e 10              	cmpxchgb	%cl, 0x10(%r14)
1408f6e0b: 0f 85 e3 27 00 00           	jne	0x1408f95f4 <.text+0x8f85f4>
1408f6e11: 48 89 f9                    	movq	%rdi, %rcx
1408f6e14: e8 57 ae 38 00              	callq	0x140c81c70 <.text+0xc80c70>
1408f6e19: 49 8b 5e 14                 	movq	0x14(%r14), %rbx
1408f6e1d: 41 0f 10 76 1c              	movups	0x1c(%r14), %xmm6
1408f6e22: f3 45 0f 10 46 2c           	movss	0x2c(%r14), %xmm8
1408f6e28: 49 8b 46 40                 	movq	0x40(%r14), %rax
1408f6e2c: 48 89 85 b0 07 00 00        	movq	%rax, 0x7b0(%rbp)
1408f6e33: 41 0f 10 46 30              	movups	0x30(%r14), %xmm0
1408f6e38: 0f 29 85 a0 07 00 00        	movaps	%xmm0, 0x7a0(%rbp)
1408f6e3f: 48 89 f9                    	movq	%rdi, %rcx
1408f6e42: e8 c9 af 38 00              	callq	0x140c81e10 <.text+0xc80e10>
