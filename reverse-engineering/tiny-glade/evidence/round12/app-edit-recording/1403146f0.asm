
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1403146f0:     	pushq	%rbp
1403146f1:     	pushq	%rsi
1403146f2:     	pushq	%rdi
1403146f3:     	subq	$0x210, %rsp            # imm = 0x210
1403146fa:     	leaq	0x80(%rsp), %rbp
140314702:     	movq	$-0x2, 0x188(%rbp)
14031470d:     	movq	%rcx, %rsi
140314710:     	xorps	%xmm0, %xmm0
140314713:     	movups	%xmm0, 0x68(%rbp)
140314717:     	movups	%xmm0, 0x80(%rbp)
14031471e:     	movups	%xmm0, 0x98(%rbp)
140314725:     	movups	%xmm0, 0xb0(%rbp)
14031472c:     	movups	%xmm0, 0xc8(%rbp)
140314733:     	movl	$0x0, 0xd8(%rbp)
14031473d:     	movups	%xmm0, 0x100(%rbp)
140314744:     	movups	%xmm0, 0x118(%rbp)
14031474b:     	movups	%xmm0, 0x130(%rbp)
140314752:     	movups	%xmm0, 0x148(%rbp)
140314759:     	movups	%xmm0, 0x160(%rbp)
140314760:     	movl	$0x0, 0x170(%rbp)
14031476a:     	movq	$0x0, -0x60(%rbp)
140314772:     	movq	$0x0, 0x48(%rbp)
14031477a:     	movq	$0x8, 0x50(%rbp)
140314782:     	movq	$0x0, 0x58(%rbp)
14031478a:     	movq	$0x10, 0x60(%rbp)
140314792:     	movq	$0x10, 0x78(%rbp)
14031479a:     	movq	$0x10, 0x90(%rbp)
1403147a5:     	movq	$0x10, 0xa8(%rbp)
1403147b0:     	movq	$0x10, 0xc0(%rbp)
1403147bb:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1403147c5:     	movq	%rax, 0xe0(%rbp)
1403147cc:     	leaq	0x26a68a2(%rip), %rax   # 0x1429bb075
1403147d3:     	movq	%rax, 0xe8(%rbp)
1403147da:     	movq	$0x31, 0xf0(%rbp)
1403147e5:     	movq	$0x10, 0xf8(%rbp)
1403147f0:     	movq	$0x10, 0x110(%rbp)
1403147fb:     	movq	$0x10, 0x128(%rbp)
140314806:     	movq	$0x10, 0x140(%rbp)
140314811:     	movq	$0x10, 0x158(%rbp)
14031481c:     	movl	$0x0, 0x178(%rbp)
140314826:     	movw	$0x1, 0x17c(%rbp)
14031482f:     	movl	$0x0, 0x180(%rbp)
140314839:     	movzbl	0x33203c9(%rip), %eax   # 0x143634c09
140314840:     	movl	$0x1e8, %ecx            # imm = 0x1E8
140314845:     	movl	$0x8, %edx
14031484a:     	callq	0x140613c10 <.text+0x612c10>
14031484f:     	testq	%rax, %rax
140314852:     	je	0x140314889 <.text+0x313889>
140314854:     	movq	%rax, %rdi
140314857:     	leaq	-0x60(%rbp), %rdx
14031485b:     	movl	$0x1e8, %r8d            # imm = 0x1E8
140314861:     	movq	%rax, %rcx
140314864:     	callq	0x142923c00 <.text+0x2922c00>
140314869:     	leaq	0x26a6730(%rip), %r8    # 0x1429bafa0
140314870:     	movq	%rsi, %rcx
140314873:     	movq	%rdi, %rdx
140314876:     	callq	0x140ce4b10 <.text+0xce3b10>
14031487b:     	movq	%rsi, %rax
14031487e:     	addq	$0x210, %rsp            # imm = 0x210
140314885:     	popq	%rdi
140314886:     	popq	%rsi
140314887:     	popq	%rbp
140314888:     	retq
140314889:     	movl	$0x8, %ecx
14031488e:     	movl	$0x1e8, %edx            # imm = 0x1E8
140314893:     	callq	0x1428d9000 <.text+0x28d8000>
140314898:     	ud2
