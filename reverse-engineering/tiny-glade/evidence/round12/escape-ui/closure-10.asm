
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140505080:     	pushq	%rbp
140505081:     	pushq	%rsi
140505082:     	pushq	%rdi
140505083:     	pushq	%rbx
140505084:     	subq	$0xc8, %rsp
14050508b:     	leaq	0x80(%rsp), %rbp
140505093:     	movq	$-0x2, 0x40(%rbp)
14050509b:     	movq	%rdx, %rdi
14050509e:     	movq	%rcx, %rsi
1405050a1:     	leaq	0x24e8e67(%rip), %rcx   # 0x1429edf0f
1405050a8:     	movl	$0xc, %edx
1405050ad:     	callq	0x140b9d480 <.text+0xb9c480>
1405050b2:     	movq	%rdx, %r8
1405050b5:     	leaq	-0x60(%rbp), %rcx
1405050b9:     	movq	%rax, %rdx
1405050bc:     	callq	0x140d7ad40 <.text+0xd79d40>
1405050c1:     	movq	-0x58(%rbp), %r8
1405050c5:     	movq	-0x50(%rbp), %r9
1405050c9:     	leaq	0x20(%rbp), %rcx
1405050cd:     	movq	%rdi, %rdx
1405050d0:     	movq	%r8, 0x38(%rbp)
1405050d4:     	callq	0x14068bfc0 <.text+0x68afc0>
1405050d9:     	leaq	0x20(%rbp), %rcx
1405050dd:     	callq	0x140681d40 <.text+0x680d40>
1405050e2:     	movss	0x24e8aae(%rip), %xmm1  # 0x1429edb98
1405050ea:     	movq	%rax, %rcx
1405050ed:     	callq	0x140106350 <.text+0x105350>
1405050f2:     	movq	0x30(%rbp), %rax
1405050f6:     	movq	%rax, 0x10(%rbp)
1405050fa:     	movups	0x20(%rbp), %xmm0
1405050fe:     	movaps	%xmm0, (%rbp)
140505102:     	movq	%rbp, %rcx
140505105:     	callq	0x140681d50 <.text+0x680d50>
14050510a:     	movq	%rax, %rcx
14050510d:     	callq	0x140683c80 <.text+0x682c80>
140505112:     	movl	%eax, %ebx
140505114:     	movq	-0x60(%rbp), %rdx
140505118:     	testq	%rdx, %rdx
14050511b:     	je	0x14050512c <.text+0x50412c>
14050511d:     	movl	$0x1, %r8d
140505123:     	movq	0x38(%rbp), %rcx
140505127:     	callq	0x140613c20 <.text+0x612c20>
14050512c:     	testb	%bl, %bl
14050512e:     	je	0x14050514d <.text+0x50414d>
140505130:     	movl	0x1c(%rsi), %eax
140505133:     	movq	0x10(%rsi), %rcx
140505137:     	movl	%eax, (%rcx)
140505139:     	movq	(%rsi), %rcx
14050513c:     	movq	$0x9, -0x60(%rbp)
140505144:     	leaq	-0x60(%rbp), %rdx
140505148:     	callq	0x14093fad0 <.text+0x93ead0>
14050514d:     	nop
14050514e:     	addq	$0xc8, %rsp
140505155:     	popq	%rbx
140505156:     	popq	%rdi
140505157:     	popq	%rsi
140505158:     	popq	%rbp
140505159:     	retq
14050515a:     	nopw	(%rax,%rax)
140505160:     	movq	%rdx, 0x10(%rsp)
140505165:     	pushq	%rbp
140505166:     	pushq	%rsi
140505167:     	pushq	%rdi
140505168:     	pushq	%rbx
140505169:     	subq	$0x28, %rsp
14050516d:     	leaq	0x80(%rdx), %rbp
140505174:     	movq	-0x60(%rbp), %rdx
140505178:     	testq	%rdx, %rdx
14050517b:     	je	0x14050518c <.text+0x50418c>
14050517d:     	movl	$0x1, %r8d
140505183:     	movq	0x38(%rbp), %rcx
140505187:     	callq	0x140613c20 <.text+0x612c20>
14050518c:     	nop
14050518d:     	addq	$0x28, %rsp
140505191:     	popq	%rbx
140505192:     	popq	%rdi
140505193:     	popq	%rsi
140505194:     	popq	%rbp
140505195:     	retq
