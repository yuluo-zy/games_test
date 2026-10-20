
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1409dd790:     	pushq	%rbp
1409dd791:     	pushq	%r15
1409dd793:     	pushq	%r14
1409dd795:     	pushq	%rsi
1409dd796:     	pushq	%rdi
1409dd797:     	pushq	%rbx
1409dd798:     	subq	$0x138, %rsp            # imm = 0x138
1409dd79f:     	leaq	0x80(%rsp), %rbp
1409dd7a7:     	movq	$-0x2, 0xb0(%rbp)
1409dd7b2:     	movq	(%r8), %rsi
1409dd7b5:     	xorl	%eax, %eax
1409dd7b7:     	cmpq	0x30(%rsi), %rax
1409dd7bb:     	jo	0x1409dd9ed <.text+0x9dc9ed>
1409dd7c1:     	movq	0x120(%rbp), %r10
1409dd7c8:     	movq	(%r10), %rax
1409dd7cb:     	movq	0x8(%r10), %r10
1409dd7cf:     	movq	(%rax), %r11
1409dd7d2:     	xorl	%edi, %edi
1409dd7d4:     	movq	%r11, %rbx
1409dd7d7:     	subq	0x18(%r10), %rbx
1409dd7db:     	cmovbq	%rdi, %rbx
1409dd7df:     	subq	0x38(%r10), %r11
1409dd7e3:     	cmovbq	%rdi, %r11
1409dd7e7:     	movq	0x30(%r10), %r14
1409dd7eb:     	subq	%r11, %r14
1409dd7ee:     	cmovbq	%rdi, %r14
1409dd7f2:     	setbe	%r11b
1409dd7f6:     	movq	0x10(%r10), %r15
1409dd7fa:     	subq	%rbx, %r15
1409dd7fd:     	cmovbq	%rdi, %r15
1409dd801:     	movq	0x40(%r10), %r10
1409dd805:     	setbe	%bl
1409dd808:     	addq	%r14, %r15
1409dd80b:     	testb	%r11b, %bl
1409dd80e:     	cmoveq	%rdi, %r15
1409dd812:     	subq	%r15, %r10
1409dd815:     	movq	%r10, (%rax)
1409dd818:     	movb	$0x1, %al
1409dd81a:     	testb	%r11b, %bl
1409dd81d:     	je	0x1409dd876 <.text+0x9dc876>
1409dd81f:     	movq	0x128(%rbp), %rax
1409dd826:     	movq	(%rax), %r10
1409dd829:     	movq	0x8(%rax), %rax
1409dd82d:     	movq	(%r10), %r11
1409dd830:     	xorl	%edi, %edi
1409dd832:     	movq	%r11, %rbx
1409dd835:     	subq	0x18(%rax), %rbx
1409dd839:     	cmovbq	%rdi, %rbx
1409dd83d:     	subq	0x38(%rax), %r11
1409dd841:     	cmovbq	%rdi, %r11
1409dd845:     	movq	0x10(%rax), %r14
1409dd849:     	movq	0x30(%rax), %r15
1409dd84d:     	subq	%r11, %r15
1409dd850:     	cmovbq	%rdi, %r15
1409dd854:     	seta	%r11b
1409dd858:     	subq	%rbx, %r14
1409dd85b:     	cmovbq	%rdi, %r14
1409dd85f:     	movq	0x40(%rax), %rbx
1409dd863:     	seta	%al
1409dd866:     	addq	%r15, %r14
1409dd869:     	orb	%r11b, %al
1409dd86c:     	cmovneq	%rdi, %r14
1409dd870:     	subq	%r14, %rbx
1409dd873:     	movq	%rbx, (%r10)
1409dd876:     	movq	0x130(%rbp), %r10
1409dd87d:     	movq	(%rcx), %rcx
1409dd880:     	cmpb	$0x1, 0x18(%rcx)
1409dd884:     	jbe	0x1409dd89a <.text+0x9dc89a>
1409dd886:     	movq	0x20(%rcx), %r11
1409dd88a:     	cmpb	$0x2, 0x19(%rcx)
1409dd88e:     	setb	%cl
1409dd891:     	andb	0xb9(%r11), %cl
1409dd898:     	orb	%cl, %al
1409dd89a:     	movq	(%r10), %rcx
1409dd89d:     	movq	0x8(%r10), %r10
1409dd8a1:     	movq	(%rcx), %r11
1409dd8a4:     	xorl	%edi, %edi
1409dd8a6:     	movq	%r11, %rbx
1409dd8a9:     	subq	0x18(%r10), %rbx
1409dd8ad:     	cmovbq	%rdi, %rbx
1409dd8b1:     	subq	0x38(%r10), %r11
1409dd8b5:     	cmovbq	%rdi, %r11
1409dd8b9:     	movq	0x30(%r10), %r14
1409dd8bd:     	subq	%r11, %r14
1409dd8c0:     	cmovbq	%rdi, %r14
1409dd8c4:     	setbe	%r11b
1409dd8c8:     	movq	0x10(%r10), %r15
1409dd8cc:     	subq	%rbx, %r15
1409dd8cf:     	cmovbq	%rdi, %r15
1409dd8d3:     	movq	0x40(%r10), %r10
1409dd8d7:     	setbe	%bl
1409dd8da:     	addq	%r14, %r15
1409dd8dd:     	movq	%r10, %rdi
1409dd8e0:     	subq	%r15, %rdi
1409dd8e3:     	movq	%rdi, (%rcx)
1409dd8e6:     	testb	%r11b, %bl
1409dd8e9:     	je	0x1409dd8f7 <.text+0x9dc8f7>
1409dd8eb:     	movq	(%rdx), %rdi
1409dd8ee:     	testb	%al, %al
1409dd8f0:     	jne	0x1409dd8fd <.text+0x9dc8fd>
1409dd8f2:     	jmp	0x1409dd9dd <.text+0x9dc9dd>
1409dd8f7:     	movq	%r10, (%rcx)
1409dd8fa:     	movq	(%rdx), %rdi
1409dd8fd:     	movq	%r8, %r14
1409dd900:     	movq	%rdx, %rbx
1409dd903:     	movq	0x110(%rbp), %r15
1409dd90a:     	movl	0x1c(%r9), %eax
1409dd90e:     	movq	0x10(%r9), %rcx
1409dd912:     	movl	%eax, (%rcx)
1409dd914:     	movq	(%r9), %rdx
1409dd917:     	leaq	0x78(%rbp), %rcx
1409dd91b:     	movq	%rdi, %r8
1409dd91e:     	xorl	%r9d, %r9d
1409dd921:     	callq	0x140b2cb30 <.text+0xb2bb30>
1409dd926:     	movl	0x1c(%r15), %eax
1409dd92a:     	movq	0x10(%r15), %rcx
1409dd92e:     	movl	%eax, (%rcx)
1409dd930:     	movq	(%r15), %rdx
1409dd933:     	movb	$0x1, 0xaf(%rbp)
1409dd93a:     	leaq	-0x60(%rbp), %rcx
1409dd93e:     	callq	0x140900010 <.text+0x8ff010>
1409dd943:     	movq	0x140(%rbp), %rax
1409dd94a:     	movq	%r14, %rcx
1409dd94d:     	movl	0x1c(%r14), %r14d
1409dd951:     	movq	0x10(%rcx), %r15
1409dd955:     	movl	%r14d, (%r15)
1409dd958:     	movups	0x78(%rbp), %xmm0
1409dd95c:     	movups	0x88(%rbp), %xmm1
1409dd963:     	movups	0x98(%rbp), %xmm2
1409dd96a:     	movaps	%xmm2, 0x60(%rbp)
1409dd96e:     	movaps	%xmm1, 0x50(%rbp)
1409dd972:     	movaps	%xmm0, 0x40(%rbp)
1409dd976:     	movups	-0x20(%rbp), %xmm0
1409dd97a:     	movaps	%xmm0, 0x30(%rbp)
1409dd97e:     	movups	-0x60(%rbp), %xmm0
1409dd982:     	movups	-0x50(%rbp), %xmm1
1409dd986:     	movups	-0x40(%rbp), %xmm2
1409dd98a:     	movups	-0x30(%rbp), %xmm3
1409dd98e:     	movaps	%xmm3, 0x20(%rbp)
1409dd992:     	movaps	%xmm2, 0x10(%rbp)
1409dd996:     	movaps	%xmm1, (%rbp)
1409dd99a:     	movaps	%xmm0, -0x10(%rbp)
1409dd99e:     	movl	0x1c(%rax), %ecx
1409dd9a1:     	movq	0x10(%rax), %rdx
1409dd9a5:     	movl	%ecx, (%rdx)
1409dd9a7:     	movq	(%rax), %r9
1409dd9aa:     	movb	$0x0, 0xaf(%rbp)
1409dd9b1:     	leaq	0x40(%rbp), %rdx
1409dd9b5:     	leaq	-0x10(%rbp), %r8
1409dd9b9:     	movq	%rsi, %rcx
1409dd9bc:     	callq	0x140a854a0 <.text+0xa844a0>
1409dd9c1:     	movq	0x118(%rbp), %rdx
1409dd9c8:     	movl	%r14d, (%r15)
1409dd9cb:     	movb	$0x0, 0xaf(%rbp)
1409dd9d2:     	movq	%rsi, %rcx
1409dd9d5:     	callq	0x140a86730 <.text+0xa85730>
1409dd9da:     	movq	%rbx, %rdx
1409dd9dd:     	movl	0x1c(%rdx), %eax
1409dd9e0:     	movq	0x10(%rdx), %rcx
1409dd9e4:     	movl	%eax, (%rcx)
1409dd9e6:     	movb	$0x1, 0x1b0(%rdi)
1409dd9ed:     	addq	$0x138, %rsp            # imm = 0x138
1409dd9f4:     	popq	%rbx
1409dd9f5:     	popq	%rdi
1409dd9f6:     	popq	%rsi
1409dd9f7:     	popq	%r14
1409dd9f9:     	popq	%r15
1409dd9fb:     	popq	%rbp
1409dd9fc:     	retq
