
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1413036c0:     	pushq	%rbp
1413036c1:     	pushq	%r15
1413036c3:     	pushq	%r14
1413036c5:     	pushq	%r13
1413036c7:     	pushq	%r12
1413036c9:     	pushq	%rsi
1413036ca:     	pushq	%rdi
1413036cb:     	pushq	%rbx
1413036cc:     	subq	$0xf8, %rsp
1413036d3:     	leaq	0x80(%rsp), %rbp
1413036db:     	movq	$-0x2, 0x70(%rbp)
1413036e3:     	movq	%r8, %rbx
1413036e6:     	movq	%rdx, %r15
1413036e9:     	movq	%rcx, %rdi
1413036ec:     	leaq	0x58(%rbp), %rcx
1413036f0:     	callq	0x1412f2310 <.text+0x12f1310>
1413036f5:     	testb	$0x1, 0x58(%rbp)
1413036f9:     	je	0x141303709 <.text+0x1302709>
1413036fb:     	movq	0x18(%r15), %rsi
1413036ff:     	cmpq	$0x2, %rsi
141303703:     	jne	0x141303725 <.text+0x1302725>
141303705:     	xorl	%eax, %eax
141303707:     	jmp	0x14130376a <.text+0x130276a>
141303709:     	movq	$0x0, (%rdi)
141303710:     	movq	$0x2, 0x8(%rdi)
141303718:     	movq	$0x0, 0x10(%rdi)
141303720:     	jmp	0x141303d34 <.text+0x1302d34>
141303725:     	cmpl	$0x1, 0x38(%r15)
14130372a:     	jne	0x141303745 <.text+0x1302745>
14130372c:     	movq	0x48(%r15), %rax
141303730:     	xorl	%ecx, %ecx
141303732:     	subq	0x40(%r15), %rax
141303736:     	cmovaeq	%rax, %rcx
14130373a:     	cmpl	$0x1, 0x58(%r15)
14130373f:     	je	0x14130374e <.text+0x130274e>
141303741:     	xorl	%eax, %eax
141303743:     	jmp	0x14130375c <.text+0x130275c>
141303745:     	xorl	%ecx, %ecx
141303747:     	cmpl	$0x1, 0x58(%r15)
14130374c:     	jne	0x141303741 <.text+0x1302741>
14130374e:     	movq	0x68(%r15), %rdx
141303752:     	xorl	%eax, %eax
141303754:     	subq	0x60(%r15), %rdx
141303758:     	cmovaeq	%rdx, %rax
14130375c:     	addq	%rax, %rcx
14130375f:     	movq	$-0x1, %rax
141303766:     	cmovaeq	%rcx, %rax
14130376a:     	movq	0x78(%r15), %rdx
14130376e:     	movq	%rdx, 0x8(%rbp)
141303772:     	cmpq	$0x2, %rdx
141303776:     	movq	%rdi, -0x40(%rbp)
14130377a:     	jne	0x141303780 <.text+0x1302780>
14130377c:     	xorl	%edx, %edx
14130377e:     	jmp	0x1413037da <.text+0x13027da>
141303780:     	cmpl	$0x1, 0x98(%r15)
141303788:     	jne	0x1413037ac <.text+0x13027ac>
14130378a:     	movq	0xa8(%r15), %rdx
141303791:     	xorl	%ecx, %ecx
141303793:     	subq	0xa0(%r15), %rdx
14130379a:     	cmovaeq	%rdx, %rcx
14130379e:     	cmpl	$0x1, 0xb8(%r15)
1413037a6:     	je	0x1413037b8 <.text+0x13027b8>
1413037a8:     	xorl	%edx, %edx
1413037aa:     	jmp	0x1413037cc <.text+0x13027cc>
1413037ac:     	xorl	%ecx, %ecx
1413037ae:     	cmpl	$0x1, 0xb8(%r15)
1413037b6:     	jne	0x1413037a8 <.text+0x13027a8>
1413037b8:     	movq	0xc8(%r15), %r8
1413037bf:     	xorl	%edx, %edx
1413037c1:     	subq	0xc0(%r15), %r8
1413037c8:     	cmovaeq	%r8, %rdx
1413037cc:     	addq	%rdx, %rcx
1413037cf:     	movq	$-0x1, %rdx
1413037d6:     	cmovaeq	%rcx, %rdx
1413037da:     	addq	%rdx, %rax
1413037dd:     	movq	$-0x1, %rcx
1413037e4:     	cmovbq	%rcx, %rax
1413037e8:     	incq	%rax
1413037eb:     	cmoveq	%rcx, %rax
1413037ef:     	cmpq	$0x5, %rax
1413037f3:     	movl	$0x4, %edi
1413037f8:     	cmovaeq	%rax, %rdi
1413037fc:     	leaq	(,%rdi,8), %r14
141303804:     	xorl	%r12d, %r12d
141303807:     	shrq	$0x3d, %rax
14130380b:     	jne	0x141303d4b <.text+0x1302d4b>
141303811:     	movabsq	$0x7ffffffffffffffe, %rax # imm = 0x7FFFFFFFFFFFFFFE
14130381b:     	cmpq	%rax, %r14
14130381e:     	ja	0x141303d4b <.text+0x1302d4b>
141303824:     	movq	0x5a(%rbp), %r13
141303828:     	movzbl	0x23313da(%rip), %eax   # 0x143634c09
14130382f:     	movl	$0x2, %r12d
141303835:     	movl	$0x2, %edx
14130383a:     	movq	%r14, %rcx
14130383d:     	callq	0x140613c10 <.text+0x612c10>
141303842:     	testq	%rax, %rax
141303845:     	je	0x141303d4b <.text+0x1302d4b>
14130384b:     	movq	%rax, %rcx
14130384e:     	movq	%r13, (%rax)
141303851:     	movq	%rdi, -0x20(%rbp)
141303855:     	movq	%rax, -0x18(%rbp)
141303859:     	movq	$0x1, -0x10(%rbp)
141303861:     	movzbl	(%r15), %eax
141303865:     	movb	%al, 0x6e(%rbp)
141303868:     	movq	0x8(%r15), %r12
14130386c:     	movq	0x10(%r15), %rax
141303870:     	movq	%rax, -0x58(%rbp)
141303874:     	movq	0x20(%r15), %rax
141303878:     	movq	%rax, 0x28(%rbp)
14130387c:     	movq	0x28(%r15), %rax
141303880:     	movq	%rax, -0x38(%rbp)
141303884:     	movzwl	0x30(%r15), %ebx
141303889:     	movq	0x38(%r15), %rdi
14130388d:     	movq	0x40(%r15), %rax
141303891:     	movq	%rax, 0x58(%rbp)
141303895:     	movq	0x48(%r15), %rax
141303899:     	movq	%rax, 0x20(%rbp)
14130389d:     	movl	0x50(%r15), %eax
1413038a1:     	movl	%eax, 0x68(%rbp)
1413038a4:     	movq	0x58(%r15), %rax
1413038a8:     	movq	%rax, 0x40(%rbp)
1413038ac:     	movq	0x60(%r15), %rax
1413038b0:     	movq	%rax, -0x8(%rbp)
1413038b4:     	movq	0x68(%r15), %rax
1413038b8:     	movq	%rax, -0x28(%rbp)
1413038bc:     	movl	0x70(%r15), %eax
1413038c0:     	movl	%eax, (%rbp)
1413038c3:     	movl	0xb0(%r15), %r14d
1413038ca:     	movl	%r14d, %eax
1413038cd:     	shrl	$0x10, %eax
1413038d0:     	movl	%eax, 0x54(%rbp)
1413038d3:     	movzbl	0xb8(%r15), %eax
1413038db:     	movb	%al, 0x6f(%rbp)
1413038de:     	movq	0x80(%r15), %rax
1413038e5:     	movq	%rax, 0x18(%rbp)
1413038e9:     	movq	0x88(%r15), %rax
1413038f0:     	movq	%rax, -0x50(%rbp)
1413038f4:     	movzwl	0x90(%r15), %eax
1413038fc:     	movw	%ax, 0x66(%rbp)
141303900:     	movq	0x98(%r15), %rax
141303907:     	movq	%rax, 0x38(%rbp)
14130390b:     	movq	0xa0(%r15), %rax
141303912:     	movq	%rax, 0x30(%rbp)
141303916:     	movq	0xa8(%r15), %rax
14130391d:     	movq	%rax, 0x10(%rbp)
141303921:     	movq	0xc0(%r15), %rax
141303928:     	movq	%rax, 0x48(%rbp)
14130392c:     	movq	0xc8(%r15), %rax
141303933:     	movq	%rax, -0x30(%rbp)
141303937:     	movl	0xd0(%r15), %eax
14130393e:     	shlq	$0x10, %rax
141303942:     	movq	%rax, -0x48(%rbp)
141303946:     	movl	$0x1, %r15d
14130394c:     	jmp	0x141303968 <.text+0x1302968>
14130394e:     	nop
141303950:     	movabsq	$0xff000000000000, %rax # imm = 0xFF000000000000
14130395a:     	orq	%rax, %r13
14130395d:     	movq	%r13, (%rcx,%r15,8)
141303961:     	incq	%r15
141303964:     	movq	%r15, -0x10(%rbp)
141303968:     	movabsq	$0x842108421084211, %r13 # imm = 0x842108421084211
141303972:     	cmpq	$0x2, %rsi
141303976:     	jne	0x1413039f0 <.text+0x13029f0>
141303978:     	movq	%rdi, %r9
14130397b:     	cmpq	-0x58(%rbp), %r12
14130397f:     	setb	%al
141303982:     	andb	0x6e(%rbp), %al
141303985:     	cmpb	$0x1, %al
141303987:     	jne	0x141303a20 <.text+0x1302a20>
14130398d:     	movq	%r12, %rbx
141303990:     	shlq	$0x10, %rbx
141303994:     	subq	%r12, %rbx
141303997:     	incq	%r12
14130399a:     	movq	%rbx, %rax
14130399d:     	mulq	%r13
1413039a0:     	subl	%edx, %ebx
1413039a2:     	shrl	%ebx
1413039a4:     	addl	%edx, %ebx
1413039a6:     	shrl	$0x4, %ebx
1413039a9:     	movw	$0x0, 0x68(%rbp)
1413039af:     	movw	%bx, 0x6a(%rbp)
1413039b3:     	movl	$0x1, %edi
1413039b8:     	movl	$0x20, %eax
1413039bd:     	movq	%rax, 0x20(%rbp)
1413039c1:     	movq	$0x0, 0x40(%rbp)
1413039c9:     	movq	%rax, -0x38(%rbp)
1413039cd:     	movl	$0x1, %eax
1413039d2:     	movq	%rax, 0x28(%rbp)
1413039d6:     	movl	$0x1, %esi
1413039db:     	xorl	%eax, %eax
1413039dd:     	jmp	0x141303b10 <.text+0x1302b10>
1413039e2:     	nopw	%cs:(%rax,%rax)
1413039f0:     	testb	$0x1, %dil
1413039f4:     	je	0x141303aa0 <.text+0x1302aa0>
1413039fa:     	movq	0x58(%rbp), %rax
1413039fe:     	movq	0x20(%rbp), %rdx
141303a02:     	movl	$0x0, %r9d
141303a08:     	movq	%rdx, 0x20(%rbp)
141303a0c:     	cmpq	%rdx, %rax
141303a0f:     	jae	0x141303aa3 <.text+0x1302aa3>
141303a15:     	jmp	0x141303b10 <.text+0x1302b10>
141303a1a:     	nopw	(%rax,%rax)
141303a20:     	cmpl	$0x2, 0x8(%rbp)
141303a24:     	movq	0x18(%rbp), %r8
141303a28:     	movq	0x30(%rbp), %rdx
141303a2c:     	je	0x141303d21 <.text+0x1302d21>
141303a32:     	testb	$0x1, 0x38(%rbp)
141303a36:     	je	0x141303b95 <.text+0x1302b95>
141303a3c:     	cmpq	0x10(%rbp), %rdx
141303a40:     	jae	0x141303b8d <.text+0x1302b8d>
141303a46:     	movq	%r8, 0x18(%rbp)
141303a4a:     	movq	%rdx, %r8
141303a4d:     	shlq	$0x10, %r8
141303a51:     	subq	%rdx, %r8
141303a54:     	leaq	0x1(%rdx), %r10
141303a58:     	movzwl	0x54(%rbp), %eax
141303a5c:     	shlq	$0x20, %rax
141303a60:     	movl	%r14d, %r11d
141303a63:     	shll	$0x10, %r11d
141303a67:     	orq	%rax, %r11
141303a6a:     	movq	%r8, %rax
141303a6d:     	mulq	%r13
141303a70:     	subl	%edx, %r8d
141303a73:     	shrl	%r8d
141303a76:     	addl	%edx, %r8d
141303a79:     	shrl	$0x4, %r8d
141303a7d:     	movzwl	%r8w, %r13d
141303a81:     	orq	%r11, %r13
141303a84:     	movl	$0x2, %esi
141303a89:     	movq	%r10, 0x30(%rbp)
141303a8d:     	jmp	0x141303c3a <.text+0x1302c3a>
141303a92:     	nopw	%cs:(%rax,%rax)
141303aa0:     	movq	%rdi, %r9
141303aa3:     	testb	$0x1, %sil
141303aa7:     	je	0x141303b60 <.text+0x1302b60>
141303aad:     	movq	0x28(%rbp), %rax
141303ab1:     	cmpq	-0x38(%rbp), %rax
141303ab5:     	jae	0x141303b60 <.text+0x1302b60>
141303abb:     	movq	0x28(%rbp), %rax
141303abf:     	movq	%rax, %r8
141303ac2:     	shlq	$0x10, %r8
141303ac6:     	subq	%rax, %r8
141303ac9:     	leaq	0x1(%rax), %r9
141303acd:     	movq	%r8, %rax
141303ad0:     	mulq	%r13
141303ad3:     	subl	%edx, %r8d
141303ad6:     	shrl	%r8d
141303ad9:     	addl	%edx, %r8d
141303adc:     	shrl	$0x4, %r8d
141303ae0:     	movq	$0x0, 0x58(%rbp)
141303ae8:     	movw	%r8w, 0x68(%rbp)
141303aed:     	movw	%bx, 0x6a(%rbp)
141303af1:     	movl	$0x1, %edi
141303af6:     	movl	$0x20, %edx
141303afb:     	movq	%r9, 0x28(%rbp)
141303aff:     	xorl	%eax, %eax
141303b01:     	movl	$0x0, %r9d
141303b07:     	movq	%rdx, 0x20(%rbp)
141303b0b:     	cmpq	%rdx, %rax
141303b0e:     	jae	0x141303aa3 <.text+0x1302aa3>
141303b10:     	leaq	0x58(%rbp), %rdx
141303b14:     	leaq	0x68(%rbp), %r8
141303b18:     	leaq	0x1(%rax), %r9
141303b1c:     	movq	%r9, (%rdx)
141303b1f:     	movl	(%r8), %r9d
141303b22:     	movq	%rax, %r8
141303b25:     	shlq	$0x10, %r8
141303b29:     	subq	%rax, %r8
141303b2c:     	movq	%r8, %rax
141303b2f:     	mulq	%r13
141303b32:     	shlq	$0x10, %r9
141303b36:     	subl	%edx, %r8d
141303b39:     	shrl	%r8d
141303b3c:     	addl	%edx, %r8d
141303b3f:     	shrl	$0x4, %r8d
141303b43:     	movzwl	%r8w, %r13d
141303b47:     	orq	%r9, %r13
141303b4a:     	cmpq	-0x20(%rbp), %r15
141303b4e:     	jne	0x141303950 <.text+0x1302950>
141303b54:     	jmp	0x141303c47 <.text+0x1302c47>
141303b59:     	nopl	(%rax)
141303b60:     	testb	$0x1, 0x40(%rbp)
141303b64:     	je	0x14130397b <.text+0x130297b>
141303b6a:     	movq	-0x8(%rbp), %rax
141303b6e:     	cmpq	-0x28(%rbp), %rax
141303b72:     	jae	0x141303b80 <.text+0x1302b80>
141303b74:     	movq	%r9, %rdi
141303b77:     	leaq	-0x8(%rbp), %rdx
141303b7b:     	movq	%rbp, %r8
141303b7e:     	jmp	0x141303b18 <.text+0x1302b18>
141303b80:     	movq	$0x0, 0x40(%rbp)
141303b88:     	jmp	0x14130397b <.text+0x130297b>
141303b8d:     	movq	$0x0, 0x38(%rbp)
141303b95:     	cmpq	-0x50(%rbp), %r8
141303b99:     	setb	%al
141303b9c:     	testb	%al, 0x8(%rbp)
141303b9f:     	je	0x141303be8 <.text+0x1302be8>
141303ba1:     	movq	%r8, %r14
141303ba4:     	shlq	$0x10, %r14
141303ba8:     	subq	%r8, %r14
141303bab:     	incq	%r8
141303bae:     	movq	%r14, %rax
141303bb1:     	mulq	%r13
141303bb4:     	subl	%edx, %r14d
141303bb7:     	shrl	%r14d
141303bba:     	addl	%edx, %r14d
141303bbd:     	shrl	$0x4, %r14d
141303bc1:     	movl	$0x1, %eax
141303bc6:     	movq	%rax, 0x38(%rbp)
141303bca:     	movl	$0x20, %eax
141303bcf:     	movq	%rax, 0x10(%rbp)
141303bd3:     	xorl	%edx, %edx
141303bd5:     	movzwl	0x66(%rbp), %eax
141303bd9:     	movl	%eax, 0x54(%rbp)
141303bdc:     	cmpq	0x10(%rbp), %rdx
141303be0:     	jb	0x141303a46 <.text+0x1302a46>
141303be6:     	jmp	0x141303b8d <.text+0x1302b8d>
141303be8:     	movq	0x48(%rbp), %rax
141303bec:     	cmpq	-0x30(%rbp), %rax
141303bf0:     	setb	%al
141303bf3:     	testb	%al, 0x6f(%rbp)
141303bf6:     	je	0x141303d21 <.text+0x1302d21>
141303bfc:     	movq	%rdx, 0x30(%rbp)
141303c00:     	movq	%r8, 0x18(%rbp)
141303c04:     	movq	0x48(%rbp), %rax
141303c08:     	movq	%rax, %r8
141303c0b:     	shlq	$0x10, %r8
141303c0f:     	subq	%rax, %r8
141303c12:     	leaq	0x1(%rax), %r10
141303c16:     	movq	%r8, %rax
141303c19:     	mulq	%r13
141303c1c:     	subl	%edx, %r8d
141303c1f:     	shrl	%r8d
141303c22:     	addl	%edx, %r8d
141303c25:     	shrl	$0x4, %r8d
141303c29:     	movzwl	%r8w, %r13d
141303c2d:     	orq	-0x48(%rbp), %r13
141303c31:     	movl	$0x2, %esi
141303c36:     	movq	%r10, 0x48(%rbp)
141303c3a:     	movq	%r9, %rdi
141303c3d:     	cmpq	-0x20(%rbp), %r15
141303c41:     	jne	0x141303950 <.text+0x1302950>
141303c47:     	cmpq	$0x2, %rsi
141303c4b:     	jne	0x141303ca0 <.text+0x1302ca0>
141303c4d:     	xorl	%eax, %eax
141303c4f:     	cmpl	$0x2, 0x8(%rbp)
141303c53:     	je	0x141303ce5 <.text+0x1302ce5>
141303c59:     	movq	0x10(%rbp), %rdx
141303c5d:     	subq	0x30(%rbp), %rdx
141303c61:     	movl	$0x0, %r8d
141303c67:     	cmovbq	%r8, %rdx
141303c6b:     	testb	$0x1, 0x38(%rbp)
141303c6f:     	cmoveq	%r8, %rdx
141303c73:     	movq	-0x30(%rbp), %rcx
141303c77:     	subq	0x48(%rbp), %rcx
141303c7b:     	cmovbq	%r8, %rcx
141303c7f:     	testb	$0x1, 0x6f(%rbp)
141303c83:     	cmoveq	%r8, %rcx
141303c87:     	addq	%rdx, %rcx
141303c8a:     	movq	$-0x1, %rdx
141303c91:     	cmovbq	%rdx, %rcx
141303c95:     	jmp	0x141303ce7 <.text+0x1302ce7>
141303c97:     	nopw	(%rax,%rax)
141303ca0:     	movq	0x20(%rbp), %rcx
141303ca4:     	subq	0x58(%rbp), %rcx
141303ca8:     	movl	$0x0, %edx
141303cad:     	cmovbq	%rdx, %rcx
141303cb1:     	testb	$0x1, %dil
141303cb5:     	cmoveq	%rdx, %rcx
141303cb9:     	movq	-0x28(%rbp), %rax
141303cbd:     	subq	-0x8(%rbp), %rax
141303cc1:     	cmovbq	%rdx, %rax
141303cc5:     	testb	$0x1, 0x40(%rbp)
141303cc9:     	cmoveq	%rdx, %rax
141303ccd:     	addq	%rcx, %rax
141303cd0:     	movq	$-0x1, %rcx
141303cd7:     	cmovbq	%rcx, %rax
141303cdb:     	cmpl	$0x2, 0x8(%rbp)
141303cdf:     	jne	0x141303c59 <.text+0x1302c59>
141303ce5:     	xorl	%ecx, %ecx
141303ce7:     	addq	%rcx, %rax
141303cea:     	movq	$-0x1, %r8
141303cf1:     	cmovbq	%r8, %rax
141303cf5:     	incq	%rax
141303cf8:     	je	0x141303cfd <.text+0x1302cfd>
141303cfa:     	movq	%rax, %r8
141303cfd:     	movq	$0x8, 0x20(%rsp)
141303d06:     	movl	$0x2, %r9d
141303d0c:     	leaq	-0x20(%rbp), %rcx
141303d10:     	movq	%r15, %rdx
141303d13:     	callq	0x1428a9a60 <.text+0x28a8a60>
141303d18:     	movq	-0x18(%rbp), %rcx
141303d1c:     	jmp	0x141303950 <.text+0x1302950>
141303d21:     	movq	-0x10(%rbp), %rax
141303d25:     	movq	-0x40(%rbp), %rdi
141303d29:     	movq	%rax, 0x10(%rdi)
141303d2d:     	movups	-0x20(%rbp), %xmm0
141303d31:     	movups	%xmm0, (%rdi)
141303d34:     	movq	%rdi, %rax
141303d37:     	addq	$0xf8, %rsp
141303d3e:     	popq	%rbx
141303d3f:     	popq	%rdi
141303d40:     	popq	%rsi
141303d41:     	popq	%r12
141303d43:     	popq	%r13
141303d45:     	popq	%r14
141303d47:     	popq	%r15
141303d49:     	popq	%rbp
141303d4a:     	retq
141303d4b:     	movq	%r12, %rcx
141303d4e:     	movq	%r14, %rdx
141303d51:     	movq	%rbx, %r8
141303d54:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
141303d59:     	int3
141303d5a:     	nopw	(%rax,%rax)
141303d60:     	movq	%rdx, 0x10(%rsp)
141303d65:     	pushq	%rbp
141303d66:     	pushq	%r15
141303d68:     	pushq	%r14
141303d6a:     	pushq	%r13
141303d6c:     	pushq	%r12
141303d6e:     	pushq	%rsi
141303d6f:     	pushq	%rdi
