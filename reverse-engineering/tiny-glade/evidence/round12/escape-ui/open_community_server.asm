
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1403d09d0:     	pushq	%r15
1403d09d2:     	pushq	%r14
1403d09d4:     	pushq	%r12
1403d09d6:     	pushq	%rsi
1403d09d7:     	pushq	%rdi
1403d09d8:     	pushq	%rbx
1403d09d9:     	subq	$0x48, %rsp
1403d09dd:     	movq	%rcx, %rsi
1403d09e0:     	leaq	0x25fae90(%rip), %rax   # 0x1429cb877
1403d09e7:     	movq	%rax, 0x28(%rsp)
1403d09ec:     	leaq	0x28(%rsp), %rax
1403d09f1:     	movq	0x28(%rsp), %rcx
1403d09f6:     	movabsq	$-0x1ab361b39a4b5295, %rax # imm = 0xE54C9E4C65B4AD6B
1403d0a00:     	movq	%rax, 0x28(%rsp)
1403d0a05:     	leaq	0x28(%rsp), %rax
1403d0a0a:     	movq	0x28(%rsp), %rdx
1403d0a0f:     	callq	0x1403cf2e0 <.text+0x3ce2e0>
1403d0a14:     	movq	(%rax), %r12
1403d0a17:     	movq	0x8(%rax), %r15
1403d0a1b:     	movq	0x10(%rax), %r14
1403d0a1f:     	movl	0x18(%rax), %edi
1403d0a22:     	movzbl	0x1c(%rax), %ebx
1403d0a26:     	movzbl	0x32641dc(%rip), %eax   # 0x143634c09
1403d0a2d:     	movl	$0x1d, %ecx
1403d0a32:     	movl	$0x1, %edx
1403d0a37:     	callq	0x140613c10 <.text+0x612c10>
1403d0a3c:     	testq	%rax, %rax
1403d0a3f:     	je	0x1403d0aca <.text+0x3cfaca>
1403d0a45:     	xorb	$-0x6f, %bl
1403d0a48:     	xorl	$0xa76feff9, %edi       # imm = 0xA76FEFF9
1403d0a4e:     	movl	%edi, %ecx
1403d0a50:     	shrl	$0x8, %ecx
1403d0a53:     	movabsq	$-0x7ed18c861fe5f727, %rdx # imm = 0x812E7379E01A08D9
1403d0a5d:     	xorq	%rdx, %r14
1403d0a60:     	movabsq	$-0x728c1044d6d4004b, %rdx # imm = 0x8D73EFBB292BFFB5
1403d0a6a:     	xorq	%rdx, %r15
1403d0a6d:     	movabsq	$-0x65af447a4af3be48, %rdx # imm = 0x9A50BB85B50C41B8
1403d0a77:     	xorq	%rdx, %r12
1403d0a7a:     	movq	%r12, (%rax)
1403d0a7d:     	movq	%r15, 0x8(%rax)
1403d0a81:     	movq	%r14, 0x10(%rax)
1403d0a85:     	movb	%dil, 0x18(%rax)
1403d0a89:     	shrl	$0x18, %edi
1403d0a8c:     	movb	%dil, 0x1b(%rax)
1403d0a90:     	movw	%cx, 0x19(%rax)
1403d0a94:     	movb	%bl, 0x1c(%rax)
1403d0a97:     	movq	$0x1d, 0x30(%rsp)
1403d0aa0:     	movq	%rax, 0x38(%rsp)
1403d0aa5:     	movq	$0x1d, 0x40(%rsp)
1403d0aae:     	leaq	0x30(%rsp), %rdx
1403d0ab3:     	movq	%rsi, %rcx
1403d0ab6:     	callq	0x14026d580 <.text+0x26c580>
1403d0abb:     	nop
1403d0abc:     	addq	$0x48, %rsp
1403d0ac0:     	popq	%rbx
1403d0ac1:     	popq	%rdi
1403d0ac2:     	popq	%rsi
1403d0ac3:     	popq	%r12
1403d0ac5:     	popq	%r14
1403d0ac7:     	popq	%r15
1403d0ac9:     	retq
1403d0aca:     	leaq	0x2600f67(%rip), %r8    # 0x1429d1a38
1403d0ad1:     	movl	$0x1, %ecx
1403d0ad6:     	movl	$0x1d, %edx
1403d0adb:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1403d0ae0:     	int3
