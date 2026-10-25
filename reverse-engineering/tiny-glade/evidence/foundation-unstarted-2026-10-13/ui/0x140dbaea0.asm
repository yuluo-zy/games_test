
D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140dbaea0: 48 89 c8                    	movq	%rcx, %rax
140dbaea3: 48 8b 4a 40                 	movq	0x40(%rdx), %rcx
140dbaea7: 48 89 48 40                 	movq	%rcx, 0x40(%rax)
140dbaeab: 0f 10 02                    	movups	(%rdx), %xmm0
140dbaeae: 0f 10 4a 10                 	movups	0x10(%rdx), %xmm1
140dbaeb2: 0f 10 52 20                 	movups	0x20(%rdx), %xmm2
140dbaeb6: 0f 10 5a 30                 	movups	0x30(%rdx), %xmm3
140dbaeba: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
140dbaebe: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
140dbaec2: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
140dbaec6: 0f 11 00                    	movups	%xmm0, (%rax)
140dbaec9: c6 40 48 00                 	movb	$0x0, 0x48(%rax)
140dbaecd: c3                          	retq
140dbaece: cc                          	int3
140dbaecf: cc                          	int3
140dbaed0: 55                          	pushq	%rbp
140dbaed1: 56                          	pushq	%rsi
140dbaed2: 57                          	pushq	%rdi
140dbaed3: 48 83 ec 50                 	subq	$0x50, %rsp
140dbaed7: 48 8d 6c 24 50              	leaq	0x50(%rsp), %rbp
140dbaedc: 48 c7 45 f8 fe ff ff ff     	movq	$-0x2, -0x8(%rbp)
140dbaee4: 48 89 ce                    	movq	%rcx, %rsi
140dbaee7: 48 89 55 d0                 	movq	%rdx, -0x30(%rbp)
140dbaeeb: 4c 89 45 d8                 	movq	%r8, -0x28(%rbp)
140dbaeef: 48 89 d1                    	movq	%rdx, %rcx
