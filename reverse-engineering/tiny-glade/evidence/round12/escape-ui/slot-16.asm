
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1405028a0:     	pushq	%r15
1405028a2:     	pushq	%r14
1405028a4:     	pushq	%r13
1405028a6:     	pushq	%r12
1405028a8:     	pushq	%rsi
1405028a9:     	pushq	%rdi
1405028aa:     	pushq	%rbp
1405028ab:     	pushq	%rbx
1405028ac:     	subq	$0xd8, %rsp
1405028b3:     	movq	%r9, %rsi
1405028b6:     	movq	%rcx, %r14
1405028b9:     	movq	0x140(%rsp), %rbp
1405028c1:     	movq	%rdx, 0x28(%rsp)
1405028c6:     	movq	%r8, 0x30(%rsp)
1405028cb:     	leaq	0x10(%rcx), %rbx
1405028cf:     	movq	%rbx, %rcx
1405028d2:     	callq	0x1406868b0 <.text+0x6858b0>
1405028d7:     	movq	%rax, %r13
1405028da:     	leaq	0x28(%rsp), %r12
1405028df:     	movq	%r12, %rcx
1405028e2:     	callq	0x140683c10 <.text+0x682c10>
1405028e7:     	movq	%rax, 0x38(%rsp)
1405028ec:     	leaq	0x38(%rsp), %r15
1405028f1:     	movq	%r15, %rcx
1405028f4:     	callq	0x1400885d0 <.text+0x875d0>
1405028f9:     	movq	%rax, 0x58(%rsp)
1405028fe:     	movabsq	$0x41223169ff28813b, %rax # imm = 0x41223169FF28813B
140502908:     	movq	%rax, 0x60(%rsp)
14050290d:     	movabsq	$-0x58648d975d569727, %rdi # imm = 0xA79B7268A2A968D9
140502917:     	movq	%rdi, 0x68(%rsp)
14050291c:     	leaq	0x58(%rsp), %rcx
140502921:     	callq	0x1405bdf20 <.text+0x5bcf20>
140502926:     	movq	%r13, 0x58(%rsp)
14050292b:     	movq	%rax, 0x60(%rsp)
140502930:     	leaq	0x58(%rsp), %rcx
140502935:     	callq	0x1405bd910 <.text+0x5bc910>
14050293a:     	movq	%rax, %r13
14050293d:     	movq	%rbx, %rcx
140502940:     	movq	%rax, %rdx
140502943:     	callq	0x140686450 <.text+0x685450>
140502948:     	testb	%al, %al
14050294a:     	je	0x140502964 <.text+0x501964>
14050294c:     	leaq	0x58(%rsp), %r14
140502951:     	movq	%r14, %rcx
140502954:     	movq	%rbx, %rdx
140502957:     	movq	%r13, %r8
14050295a:     	callq	0x140686530 <.text+0x685530>
14050295f:     	jmp	0x140502aae <.text+0x501aae>
140502964:     	movq	0x3129a3d(%rip), %rax   # 0x14362c3a8
14050296b:     	movq	(%rax), %rax
14050296e:     	testq	%rax, %rax
140502971:     	je	0x140502a4c <.text+0x501a4c>
140502977:     	movq	%r14, 0x38(%rsp)
14050297c:     	leaq	0x30f1d(%rip), %rax     # 0x1405338a0 <.text+0x5328a0>
140502983:     	movq	%rax, 0x40(%rsp)
140502988:     	movq	%r12, 0x48(%rsp)
14050298d:     	movq	%rax, 0x50(%rsp)
140502992:     	leaq	0x24eb3bf(%rip), %rcx   # 0x1429edd58
140502999:     	callq	0x14067dd80 <.text+0x67cd80>
14050299e:     	movups	(%rax), %xmm0
1405029a1:     	movl	0x10(%rax), %eax
1405029a4:     	movq	$0x1, 0x88(%rsp)
1405029b0:     	leaq	0x24eb371(%rip), %rcx   # 0x1429edd28
1405029b7:     	movq	%rcx, 0x90(%rsp)
1405029bf:     	movq	$0x14, 0x98(%rsp)
1405029cb:     	leaq	0x24eb2c6(%rip), %rdx   # 0x1429edc98
1405029d2:     	movq	%rdx, 0xa8(%rsp)
1405029da:     	movq	$0x2, 0xb0(%rsp)
1405029e6:     	movq	%r15, 0xb8(%rsp)
1405029ee:     	movq	$0x2, 0xc0(%rsp)
1405029fa:     	movq	$0x0, 0xc8(%rsp)
140502a06:     	movq	$0x0, 0x58(%rsp)
140502a0f:     	movq	%rcx, 0x60(%rsp)
140502a14:     	movq	$0x14, 0x68(%rsp)
140502a1d:     	movq	$0x0, 0x70(%rsp)
140502a26:     	movups	%xmm0, 0x78(%rsp)
140502a2b:     	movl	$0x1, 0xa0(%rsp)
140502a36:     	movl	%eax, 0xa4(%rsp)
140502a3d:     	leaq	0x27(%rsp), %rcx
140502a42:     	leaq	0x58(%rsp), %rdx
140502a47:     	callq	0x140d4a330 <.text+0xd49330>
140502a4c:     	movups	0x28(%rsp), %xmm0
140502a51:     	movq	0x18(%r14), %r14
140502a55:     	movups	%xmm0, 0x38(%rsp)
140502a5a:     	leaq	0x38(%rsp), %rcx
140502a5f:     	callq	0x140683c10 <.text+0x682c10>
140502a64:     	movq	%rax, 0x58(%rsp)
140502a69:     	movabsq	$0x41223169ff28813b, %rax # imm = 0x41223169FF28813B
140502a73:     	movq	%rax, 0x60(%rsp)
140502a78:     	movq	%rdi, 0x68(%rsp)
140502a7d:     	leaq	0x58(%rsp), %rcx
140502a82:     	callq	0x1405bdf20 <.text+0x5bcf20>
140502a87:     	movq	%r14, 0x58(%rsp)
140502a8c:     	movq	%rax, 0x60(%rsp)
140502a91:     	leaq	0x58(%rsp), %rcx
140502a96:     	callq	0x1405bd910 <.text+0x5bc910>
140502a9b:     	leaq	0x58(%rsp), %r14
140502aa0:     	movq	%r14, %rcx
140502aa3:     	movq	%rbx, %rdx
140502aa6:     	movq	%rax, %r8
140502aa9:     	callq	0x1406861b0 <.text+0x6851b0>
140502aae:     	movq	%rsi, %rcx
140502ab1:     	movq	%rbp, %rdx
140502ab4:     	movq	%r14, %r8
140502ab7:     	callq	0x1405051a0 <.text+0x5041a0>
140502abc:     	nop
140502abd:     	addq	$0xd8, %rsp
140502ac4:     	popq	%rbx
140502ac5:     	popq	%rbp
140502ac6:     	popq	%rdi
140502ac7:     	popq	%rsi
140502ac8:     	popq	%r12
140502aca:     	popq	%r13
140502acc:     	popq	%r14
140502ace:     	popq	%r15
140502ad0:     	retq
