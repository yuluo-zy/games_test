
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140501e40:     	pushq	%r15
140501e42:     	pushq	%r14
140501e44:     	pushq	%r13
140501e46:     	pushq	%r12
140501e48:     	pushq	%rsi
140501e49:     	pushq	%rdi
140501e4a:     	pushq	%rbp
140501e4b:     	pushq	%rbx
140501e4c:     	subq	$0xf8, %rsp
140501e53:     	movq	%r9, %rsi
140501e56:     	movq	%rcx, %rbx
140501e59:     	movq	%rdx, 0x68(%rsp)
140501e5e:     	movq	%r8, 0x70(%rsp)
140501e63:     	leaq	0x10(%rcx), %rdi
140501e67:     	movq	%rdi, %rcx
140501e6a:     	callq	0x1406868b0 <.text+0x6858b0>
140501e6f:     	movq	%rax, %r12
140501e72:     	leaq	0x68(%rsp), %r15
140501e77:     	movq	%r15, %rcx
140501e7a:     	callq	0x140683c10 <.text+0x682c10>
140501e7f:     	movq	%rax, 0x48(%rsp)
140501e84:     	leaq	0x48(%rsp), %r14
140501e89:     	movq	%r14, %rcx
140501e8c:     	callq	0x1400885d0 <.text+0x875d0>
140501e91:     	movq	%rax, 0x78(%rsp)
140501e96:     	movabsq	$0x41223169ff28813b, %r13 # imm = 0x41223169FF28813B
140501ea0:     	movq	%r13, 0x80(%rsp)
140501ea8:     	movabsq	$-0x58648d975d569727, %rbp # imm = 0xA79B7268A2A968D9
140501eb2:     	movq	%rbp, 0x88(%rsp)
140501eba:     	leaq	0x78(%rsp), %rcx
140501ebf:     	callq	0x1405bdf20 <.text+0x5bcf20>
140501ec4:     	movq	%r12, 0x78(%rsp)
140501ec9:     	movq	%rax, 0x80(%rsp)
140501ed1:     	leaq	0x78(%rsp), %rcx
140501ed6:     	callq	0x1405bd910 <.text+0x5bc910>
140501edb:     	movq	%rax, %r12
140501ede:     	movq	%rdi, %rcx
140501ee1:     	movq	%rax, %rdx
140501ee4:     	callq	0x140686450 <.text+0x685450>
140501ee9:     	testb	%al, %al
140501eeb:     	je	0x140501f05 <.text+0x500f05>
140501eed:     	leaq	0x48(%rsp), %rbx
140501ef2:     	movq	%rbx, %rcx
140501ef5:     	movq	%rdi, %rdx
140501ef8:     	movq	%r12, %r8
140501efb:     	callq	0x140686530 <.text+0x685530>
140501f00:     	jmp	0x14050205a <.text+0x50105a>
140501f05:     	movq	0x312a49c(%rip), %rax   # 0x14362c3a8
140501f0c:     	movq	(%rax), %rax
140501f0f:     	testq	%rax, %rax
140501f12:     	je	0x140501ff9 <.text+0x500ff9>
140501f18:     	movq	%rbx, 0x48(%rsp)
140501f1d:     	leaq	0x3197c(%rip), %rax     # 0x1405338a0 <.text+0x5328a0>
140501f24:     	movq	%rax, 0x50(%rsp)
140501f29:     	movq	%r15, 0x58(%rsp)
140501f2e:     	movq	%rax, 0x60(%rsp)
140501f33:     	leaq	0x24ebe1e(%rip), %rcx   # 0x1429edd58
140501f3a:     	callq	0x14067dd80 <.text+0x67cd80>
140501f3f:     	movups	(%rax), %xmm0
140501f42:     	movl	0x10(%rax), %eax
140501f45:     	movq	$0x1, 0xa8(%rsp)
140501f51:     	leaq	0x24ebdd0(%rip), %rcx   # 0x1429edd28
140501f58:     	movq	%rcx, 0xb0(%rsp)
140501f60:     	movq	$0x14, 0xb8(%rsp)
140501f6c:     	leaq	0x24ebd25(%rip), %rdx   # 0x1429edc98
140501f73:     	movq	%rdx, 0xc8(%rsp)
140501f7b:     	movq	$0x2, 0xd0(%rsp)
140501f87:     	movq	%r14, 0xd8(%rsp)
140501f8f:     	movq	$0x2, 0xe0(%rsp)
140501f9b:     	movq	$0x0, 0xe8(%rsp)
140501fa7:     	movq	$0x0, 0x78(%rsp)
140501fb0:     	movq	%rcx, 0x80(%rsp)
140501fb8:     	movq	$0x14, 0x88(%rsp)
140501fc4:     	movq	$0x0, 0x90(%rsp)
140501fd0:     	movups	%xmm0, 0x98(%rsp)
140501fd8:     	movl	$0x1, 0xc0(%rsp)
140501fe3:     	movl	%eax, 0xc4(%rsp)
140501fea:     	leaq	0x47(%rsp), %rcx
140501fef:     	leaq	0x78(%rsp), %rdx
140501ff4:     	callq	0x140d4a330 <.text+0xd49330>
140501ff9:     	movups	0x68(%rsp), %xmm0
140501ffe:     	movq	0x18(%rbx), %rbx
140502002:     	movups	%xmm0, 0x48(%rsp)
140502007:     	leaq	0x48(%rsp), %rcx
14050200c:     	callq	0x140683c10 <.text+0x682c10>
140502011:     	movq	%rax, 0x78(%rsp)
140502016:     	movq	%r13, 0x80(%rsp)
14050201e:     	movq	%rbp, 0x88(%rsp)
140502026:     	leaq	0x78(%rsp), %rcx
14050202b:     	callq	0x1405bdf20 <.text+0x5bcf20>
140502030:     	movq	%rbx, 0x78(%rsp)
140502035:     	movq	%rax, 0x80(%rsp)
14050203d:     	leaq	0x78(%rsp), %rcx
140502042:     	callq	0x1405bd910 <.text+0x5bc910>
140502047:     	leaq	0x48(%rsp), %rbx
14050204c:     	movq	%rbx, %rcx
14050204f:     	movq	%rdi, %rdx
140502052:     	movq	%rax, %r8
140502055:     	callq	0x1406861b0 <.text+0x6851b0>
14050205a:     	movq	%rbx, %rcx
14050205d:     	callq	0x140683ce0 <.text+0x682ce0>
140502062:     	movl	%eax, %ecx
140502064:     	xorb	$0x1, %cl
140502067:     	movzbl	%cl, %ecx
14050206a:     	leaq	0x11(,%rcx,2), %rcx
140502072:     	leaq	0x24ebddc(%rip), %rdx   # 0x1429ede55
140502079:     	leaq	0x24ebdc2(%rip), %r8    # 0x1429ede42
140502080:     	testb	%al, %al
140502082:     	cmovneq	%rdx, %r8
140502086:     	movq	%r8, 0x20(%rsp)
14050208b:     	movq	%rcx, 0x28(%rsp)
140502090:     	movl	$0x42a80000, 0x30(%rsp) # imm = 0x42A80000
140502098:     	leaq	0x24ebdc7(%rip), %r8    # 0x1429ede66
14050209f:     	leaq	0x78(%rsp), %rdi
1405020a4:     	movl	$0xb, %r9d
1405020aa:     	movq	%rdi, %rcx
1405020ad:     	movq	%rbx, %rdx
1405020b0:     	callq	0x14032e0b0 <.text+0x32d0b0>
1405020b5:     	movq	%rdi, %rcx
1405020b8:     	callq	0x140681d50 <.text+0x680d50>
1405020bd:     	movq	%rax, %rcx
1405020c0:     	callq	0x140683c80 <.text+0x682c80>
1405020c5:     	testb	%al, %al
1405020c7:     	je	0x1405020d1 <.text+0x5010d1>
1405020c9:     	movq	%rsi, %rcx
1405020cc:     	callq	0x1403d09d0 <.text+0x3cf9d0>
1405020d1:     	nop
1405020d2:     	addq	$0xf8, %rsp
1405020d9:     	popq	%rbx
1405020da:     	popq	%rbp
1405020db:     	popq	%rdi
1405020dc:     	popq	%rsi
1405020dd:     	popq	%r12
1405020df:     	popq	%r13
1405020e1:     	popq	%r14
1405020e3:     	popq	%r15
1405020e5:     	retq
