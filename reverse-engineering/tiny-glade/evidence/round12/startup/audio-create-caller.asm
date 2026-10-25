
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1400d81c0: 00 48 8b                    	addb	%cl, -0x75(%rax)
1400d81c3: 4d 78 e8                    	js	0x1400d81ae <.text+0xd71ae>
1400d81c6: 76 1d                       	jbe	0x1400d81e5 <.text+0xd71e5>
1400d81c8: 00 00                       	addb	%al, (%rax)
1400d81ca: 48 8b 85 90 00 00 00        	movq	0x90(%rbp), %rax
1400d81d1: 48 8b 50 18                 	movq	0x18(%rax), %rdx
1400d81d5: 48 85 d2                    	testq	%rdx, %rdx
1400d81d8: 74 16                       	je	0x1400d81f0 <.text+0xd71f0>
1400d81da: 48 8b 85 90 00 00 00        	movq	0x90(%rbp), %rax
1400d81e1: 48 8b 48 20                 	movq	0x20(%rax), %rcx
1400d81e5: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1400d81eb: e8 30 ba 53 00              	callq	0x140613c20 <.text+0x612c20>
1400d81f0: 90                          	nop
1400d81f1: 48 83 c4 30                 	addq	$0x30, %rsp
1400d81f5: 5d                          	popq	%rbp
1400d81f6: c3                          	retq
1400d81f7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1400d8200: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1400d8205: 55                          	pushq	%rbp
1400d8206: 48 83 ec 30                 	subq	$0x30, %rsp
1400d820a: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1400d8211: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
1400d8218: e8 13 4a c7 00              	callq	0x140d4cc30 <.text+0xd4bc30>
1400d821d: 90                          	nop
1400d821e: 48 83 c4 30                 	addq	$0x30, %rsp
1400d8222: 5d                          	popq	%rbp
1400d8223: c3                          	retq
1400d8224: cc                          	int3
1400d8225: cc                          	int3
1400d8226: cc                          	int3
1400d8227: cc                          	int3
1400d8228: cc                          	int3
1400d8229: cc                          	int3
1400d822a: cc                          	int3
1400d822b: cc                          	int3
1400d822c: cc                          	int3
1400d822d: cc                          	int3
1400d822e: cc                          	int3
1400d822f: cc                          	int3
1400d8230: 55                          	pushq	%rbp
1400d8231: 48 81 ec d0 00 00 00        	subq	$0xd0, %rsp
1400d8238: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1400d8240: 48 c7 45 48 fe ff ff ff     	movq	$-0x2, 0x48(%rbp)
1400d8248: 48 89 4d 28                 	movq	%rcx, 0x28(%rbp)
1400d824c: 0f b6 51 10                 	movzbl	0x10(%rcx), %edx
1400d8250: 48 8d 4d f0                 	leaq	-0x10(%rbp), %rcx
1400d8254: e8 e7 c7 97 00              	callq	0x140a54a40 <.text+0xa53a40>
1400d8259: 48 8b 55 00                 	movq	(%rbp), %rdx
1400d825d: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
1400d8261: 48 89 45 30                 	movq	%rax, 0x30(%rbp)
1400d8265: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
1400d8269: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
1400d826d: 0f 10 45 08                 	movups	0x8(%rbp), %xmm0
1400d8271: 0f 29 45 d0                 	movaps	%xmm0, -0x30(%rbp)
1400d8275: 0f 10 45 18                 	movups	0x18(%rbp), %xmm0
1400d8279: 0f 29 45 e0                 	movaps	%xmm0, -0x20(%rbp)
1400d827d: c6 45 47 01                 	movb	$0x1, 0x47(%rbp)
1400d8281: 48 8d 4d f0                 	leaq	-0x10(%rbp), %rcx
1400d8285: 4c 8d 45 d0                 	leaq	-0x30(%rbp), %r8
1400d8289: e8 22 29 9a 00              	callq	0x140a7abb0 <.text+0xa79bb0>
1400d828e: c6 45 47 01                 	movb	$0x1, 0x47(%rbp)
