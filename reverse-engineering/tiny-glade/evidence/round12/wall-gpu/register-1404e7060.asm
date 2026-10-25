
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1404e7060:     	pushq	%rbp
1404e7061:     	pushq	%rsi
1404e7062:     	pushq	%rdi
1404e7063:     	subq	$0x2c0, %rsp            # imm = 0x2C0
1404e706a:     	leaq	0x80(%rsp), %rbp
1404e7072:     	movq	$-0x2, 0x238(%rbp)
1404e707d:     	movq	%rcx, %rsi
1404e7080:     	xorps	%xmm0, %xmm0
1404e7083:     	movups	%xmm0, -0x40(%rbp)
1404e7087:     	movups	%xmm0, -0x28(%rbp)
1404e708b:     	movups	%xmm0, -0x10(%rbp)
1404e708f:     	movups	%xmm0, 0x8(%rbp)
1404e7093:     	movups	%xmm0, 0x20(%rbp)
1404e7097:     	movl	$0x0, 0x30(%rbp)
1404e709e:     	movups	%xmm0, 0x58(%rbp)
1404e70a2:     	movups	%xmm0, 0x70(%rbp)
1404e70a6:     	movups	%xmm0, 0x88(%rbp)
1404e70ad:     	movups	%xmm0, 0xa0(%rbp)
1404e70b4:     	movups	%xmm0, 0xb8(%rbp)
1404e70bb:     	movl	$0x0, 0xc8(%rbp)
1404e70c5:     	movq	$0x0, -0x60(%rbp)
1404e70cd:     	movq	$0x8, -0x58(%rbp)
1404e70d5:     	movq	$0x0, -0x50(%rbp)
1404e70dd:     	movq	$0x10, -0x48(%rbp)
1404e70e5:     	movq	$0x10, -0x30(%rbp)
1404e70ed:     	movq	$0x10, -0x18(%rbp)
1404e70f5:     	movq	$0x10, (%rbp)
1404e70fd:     	movq	$0x10, 0x18(%rbp)
1404e7105:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1404e710f:     	movq	%rax, 0x38(%rbp)
1404e7113:     	leaq	0x250453b(%rip), %rcx   # 0x1429eb655
1404e711a:     	movq	%rcx, 0x40(%rbp)
1404e711e:     	movq	$0x3c, 0x48(%rbp)
1404e7126:     	movq	$0x10, 0x50(%rbp)
1404e712e:     	movq	$0x10, 0x68(%rbp)
1404e7136:     	movq	$0x10, 0x80(%rbp)
1404e7141:     	movq	$0x10, 0x98(%rbp)
1404e714c:     	movq	$0x10, 0xb0(%rbp)
1404e7157:     	movl	$0x0, 0xd0(%rbp)
1404e7161:     	movw	$0x1, 0xd4(%rbp)
1404e716a:     	movq	%rax, 0x1d0(%rbp)
1404e7171:     	movl	$0x0, 0x230(%rbp)
1404e717b:     	movzbl	0x314da87(%rip), %eax   # 0x143634c09
1404e7182:     	movl	$0x298, %ecx            # imm = 0x298
1404e7187:     	movl	$0x8, %edx
1404e718c:     	callq	0x140613c10 <.text+0x612c10>
1404e7191:     	testq	%rax, %rax
1404e7194:     	je	0x1404e71cb <.text+0x4e61cb>
1404e7196:     	movq	%rax, %rdi
1404e7199:     	leaq	-0x60(%rbp), %rdx
1404e719d:     	movl	$0x298, %r8d            # imm = 0x298
1404e71a3:     	movq	%rax, %rcx
1404e71a6:     	callq	0x142923c00 <.text+0x2922c00>
1404e71ab:     	leaq	0x2504266(%rip), %r8    # 0x1429eb418
1404e71b2:     	movq	%rsi, %rcx
1404e71b5:     	movq	%rdi, %rdx
1404e71b8:     	callq	0x140ce4b10 <.text+0xce3b10>
1404e71bd:     	movq	%rsi, %rax
1404e71c0:     	addq	$0x2c0, %rsp            # imm = 0x2C0
1404e71c7:     	popq	%rdi
1404e71c8:     	popq	%rsi
1404e71c9:     	popq	%rbp
1404e71ca:     	retq
1404e71cb:     	movl	$0x8, %ecx
1404e71d0:     	movl	$0x298, %edx            # imm = 0x298
1404e71d5:     	callq	0x1428d9000 <.text+0x28d8000>
1404e71da:     	ud2
