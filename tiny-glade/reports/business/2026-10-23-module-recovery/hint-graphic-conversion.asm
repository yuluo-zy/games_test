
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141c4e8b0:     	pushq	%rbp
141c4e8b1:     	pushq	%r14
141c4e8b3:     	pushq	%rsi
141c4e8b4:     	pushq	%rdi
141c4e8b5:     	pushq	%rbx
141c4e8b6:     	subq	$0x90, %rsp
141c4e8bd:     	leaq	0x80(%rsp), %rbp
141c4e8c5:     	movq	$-0x2, 0x8(%rbp)
141c4e8cd:     	movq	%r8, %rbx
141c4e8d0:     	movq	%rdx, %r14
141c4e8d3:     	movq	%rcx, %rsi
141c4e8d6:     	movl	(%rdx), %edi
141c4e8d8:     	cmpl	$0xca, %edi
141c4e8de:     	jne	0x141c4e972 <.text+0x1c4d972>
141c4e8e4:     	movl	%edi, 0x4(%rbp)
141c4e8e7:     	movq	0x18(%r14), %rdi
141c4e8eb:     	movq	%rdi, -0x20(%rbp)
141c4e8ef:     	movq	%r14, -0x8(%rbp)
141c4e8f3:     	movups	0x8(%r14), %xmm0
141c4e8f8:     	movaps	%xmm0, -0x30(%rbp)
141c4e8fc:     	movq	0x20(%rbx), %rax
141c4e900:     	movq	%rax, -0x38(%rbp)
141c4e904:     	movups	(%rbx), %xmm0
141c4e907:     	movq	%rbx, -0x10(%rbp)
141c4e90b:     	movups	0x10(%rbx), %xmm1
141c4e90f:     	movups	%xmm1, -0x48(%rbp)
141c4e913:     	movups	%xmm0, -0x58(%rbp)
141c4e917:     	movl	$0xc5, -0x60(%rbp)
141c4e91e:     	cmpq	-0x30(%rbp), %rdi
141c4e922:     	jne	0x141c4e934 <.text+0x1c4d934>
141c4e924:     	leaq	0x12c596d(%rip), %rdx   # 0x142f14298
141c4e92b:     	leaq	-0x30(%rbp), %rcx
141c4e92f:     	callq	0x1405629f0 <.text+0x5619f0>
141c4e934:     	movq	-0x28(%rbp), %rax
141c4e938:     	leaq	(%rdi,%rdi,2), %rcx
141c4e93c:     	shlq	$0x4, %rcx
141c4e940:     	movups	-0x60(%rbp), %xmm0
141c4e944:     	movups	-0x50(%rbp), %xmm1
141c4e948:     	movups	-0x40(%rbp), %xmm2
141c4e94c:     	movups	%xmm2, 0x20(%rax,%rcx)
141c4e951:     	movups	%xmm1, 0x10(%rax,%rcx)
141c4e956:     	movups	%xmm0, (%rax,%rcx)
141c4e95a:     	incq	%rdi
141c4e95d:     	movq	%rdi, -0x20(%rbp)
141c4e961:     	movq	%rdi, 0x18(%rsi)
141c4e965:     	movaps	-0x30(%rbp), %xmm0
141c4e969:     	movups	%xmm0, 0x8(%rsi)
141c4e96d:     	jmp	0x141c4e9fc <.text+0x1c4d9fc>
141c4e972:     	movzbl	0x19e6290(%rip), %eax   # 0x143634c09
141c4e979:     	movl	$0x60, %ecx
141c4e97e:     	movl	$0x8, %edx
141c4e983:     	callq	0x140613c10 <.text+0x612c10>
141c4e988:     	testq	%rax, %rax
141c4e98b:     	je	0x141c4ea13 <.text+0x1c4da13>
141c4e991:     	movups	(%r14), %xmm0
141c4e995:     	movups	0x10(%r14), %xmm1
141c4e99a:     	movups	0x20(%r14), %xmm2
141c4e99f:     	movups	%xmm2, 0x20(%rax)
141c4e9a3:     	movups	%xmm1, 0x10(%rax)
141c4e9a7:     	movups	%xmm0, (%rax)
141c4e9aa:     	movq	0x20(%rbx), %rcx
141c4e9ae:     	movq	%rcx, -0x3c(%rbp)
141c4e9b2:     	movups	(%rbx), %xmm0
141c4e9b5:     	movups	0x10(%rbx), %xmm1
141c4e9b9:     	movups	%xmm1, -0x4c(%rbp)
141c4e9bd:     	movups	%xmm0, -0x5c(%rbp)
141c4e9c1:     	movl	$0xc5, 0x30(%rax)
141c4e9c8:     	movups	-0x60(%rbp), %xmm0
141c4e9cc:     	movups	-0x50(%rbp), %xmm1
141c4e9d0:     	movq	-0x44(%rbp), %rcx
141c4e9d4:     	movq	-0x3c(%rbp), %rdx
141c4e9d8:     	movups	%xmm0, 0x34(%rax)
141c4e9dc:     	movups	%xmm1, 0x44(%rax)
141c4e9e0:     	movq	%rcx, 0x50(%rax)
141c4e9e4:     	movq	%rdx, 0x58(%rax)
141c4e9e8:     	movq	$0x2, 0x8(%rsi)
141c4e9f0:     	movq	%rax, 0x10(%rsi)
141c4e9f4:     	movq	$0x2, 0x18(%rsi)
141c4e9fc:     	movl	$0xca, (%rsi)
141c4ea02:     	movq	%rsi, %rax
141c4ea05:     	addq	$0x90, %rsp
141c4ea0c:     	popq	%rbx
141c4ea0d:     	popq	%rdi
141c4ea0e:     	popq	%rsi
141c4ea0f:     	popq	%r14
141c4ea11:     	popq	%rbp
141c4ea12:     	retq
141c4ea13:     	movl	%edi, 0x4(%rbp)
141c4ea16:     	movq	%r14, -0x8(%rbp)
141c4ea1a:     	movq	%rbx, -0x10(%rbp)
141c4ea1e:     	movl	$0x8, %ecx
141c4ea23:     	movl	$0x60, %edx
141c4ea28:     	callq	0x1428d9000 <.text+0x28d8000>
141c4ea2d:     	ud2
141c4ea2f:     	nop
141c4ea30:     	movq	%rdx, 0x10(%rsp)
141c4ea35:     	pushq	%rbp
141c4ea36:     	pushq	%r14
141c4ea38:     	pushq	%rsi
141c4ea39:     	pushq	%rdi
141c4ea3a:     	pushq	%rbx
141c4ea3b:     	subq	$0x20, %rsp
141c4ea3f:     	leaq	0x80(%rdx), %rbp
141c4ea46:     	leaq	-0x30(%rbp), %rcx
141c4ea4a:     	callq	0x140ea8740 <.text+0xea7740>
141c4ea4f:     	nop
141c4ea50:     	addq	$0x20, %rsp
141c4ea54:     	popq	%rbx
141c4ea55:     	popq	%rdi
141c4ea56:     	popq	%rsi
141c4ea57:     	popq	%r14
141c4ea59:     	popq	%rbp
141c4ea5a:     	retq
141c4ea5b:     	nopl	(%rax,%rax)
141c4ea60:     	movq	%rdx, 0x10(%rsp)
141c4ea65:     	pushq	%rbp
141c4ea66:     	pushq	%r14
141c4ea68:     	pushq	%rsi
141c4ea69:     	pushq	%rdi
141c4ea6a:     	pushq	%rbx
141c4ea6b:     	subq	$0x20, %rsp
141c4ea6f:     	leaq	0x80(%rdx), %rbp
141c4ea76:     	leaq	-0x60(%rbp), %rcx
141c4ea7a:     	callq	0x141c4b700 <.text+0x1c4a700>
141c4ea7f:     	nop
141c4ea80:     	addq	$0x20, %rsp
141c4ea84:     	popq	%rbx
141c4ea85:     	popq	%rdi
141c4ea86:     	popq	%rsi
141c4ea87:     	popq	%r14
141c4ea89:     	popq	%rbp
141c4ea8a:     	retq
141c4ea8b:     	nopl	(%rax,%rax)
141c4ea90:     	movq	%rdx, 0x10(%rsp)
141c4ea95:     	pushq	%rbp
141c4ea96:     	pushq	%r14
141c4ea98:     	pushq	%rsi
141c4ea99:     	pushq	%rdi
141c4ea9a:     	pushq	%rbx
141c4ea9b:     	subq	$0x20, %rsp
141c4ea9f:     	leaq	0x80(%rdx), %rbp
141c4eaa6:     	cmpl	$0xca, 0x4(%rbp)
141c4eaad:     	jne	0x141c4eaba <.text+0x1c4daba>
141c4eaaf:     	addq	$0x20, %rsp
141c4eab3:     	popq	%rbx
141c4eab4:     	popq	%rdi
141c4eab5:     	popq	%rsi
141c4eab6:     	popq	%r14
141c4eab8:     	popq	%rbp
141c4eab9:     	retq
141c4eaba:     	movq	-0x10(%rbp), %rcx
141c4eabe:     	callq	0x140eaa8f0 <.text+0xea98f0>
141c4eac3:     	movq	-0x8(%rbp), %rcx
141c4eac7:     	callq	0x141c4b700 <.text+0x1c4a700>
141c4eacc:     	jmp	0x141c4eaaf <.text+0x1c4daaf>
141c4eace:     	int3
141c4eacf:     	int3
