
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14088bf70: 55                          	pushq	%rbp
14088bf71: 48 83 ec 30                 	subq	$0x30, %rsp
14088bf75: 48 8d 6c 24 30              	leaq	0x30(%rsp), %rbp
14088bf7a: 48 c7 45 f8 fe ff ff ff     	movq	$-0x2, -0x8(%rbp)
14088bf82: 48 89 4d f0                 	movq	%rcx, -0x10(%rbp)
14088bf86: e8 f5 e3 00 00              	callq	0x14089a380 <.text+0x899380>
14088bf8b: 48 8b 4d f0                 	movq	-0x10(%rbp), %rcx
14088bf8f: 48 8b 41 08                 	movq	0x8(%rcx), %rax
14088bf93: 48 85 c0                    	testq	%rax, %rax
14088bf96: 74 1b                       	je	0x14088bfb3 <.text+0x88afb3>
14088bf98: 48 8b 09                    	movq	(%rcx), %rcx
14088bf9b: 48 c1 e0 03                 	shlq	$0x3, %rax
14088bf9f: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
14088bfa3: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14088bfa9: 48 83 c4 30                 	addq	$0x30, %rsp
14088bfad: 5d                          	popq	%rbp
14088bfae: e9 6d 7c d8 ff              	jmp	0x140613c20 <.text+0x612c20>
14088bfb3: 90                          	nop
14088bfb4: 48 83 c4 30                 	addq	$0x30, %rsp
14088bfb8: 5d                          	popq	%rbp
14088bfb9: c3                          	retq
14088bfba: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
14088bfc0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14088bfc5: 55                          	pushq	%rbp
14088bfc6: 48 83 ec 20                 	subq	$0x20, %rsp
14088bfca: 48 8d 6a 30                 	leaq	0x30(%rdx), %rbp
14088bfce: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
14088bfd2: 48 8b 40 08                 	movq	0x8(%rax), %rax
14088bfd6: 48 85 c0                    	testq	%rax, %rax
14088bfd9: 74 1a                       	je	0x14088bff5 <.text+0x88aff5>
14088bfdb: 48 8b 4d f0                 	movq	-0x10(%rbp), %rcx
14088bfdf: 48 8b 09                    	movq	(%rcx), %rcx
14088bfe2: 48 c1 e0 03                 	shlq	$0x3, %rax
14088bfe6: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
14088bfea: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14088bff0: e8 2b 7c d8 ff              	callq	0x140613c20 <.text+0x612c20>
14088bff5: 90                          	nop
14088bff6: 48 83 c4 20                 	addq	$0x20, %rsp
14088bffa: 5d                          	popq	%rbp
14088bffb: c3                          	retq
