
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140500e70:     	pushq	%r15
140500e72:     	pushq	%r14
140500e74:     	pushq	%r13
140500e76:     	pushq	%r12
140500e78:     	pushq	%rsi
140500e79:     	pushq	%rdi
140500e7a:     	pushq	%rbp
140500e7b:     	pushq	%rbx
140500e7c:     	subq	$0xd8, %rsp
140500e83:     	movq	%r9, %rsi
140500e86:     	movq	%rcx, %rbx
140500e89:     	movq	%rdx, 0x28(%rsp)
140500e8e:     	movq	%r8, 0x30(%rsp)
140500e93:     	leaq	0x10(%rcx), %rdi
140500e97:     	movq	%rdi, %rcx
140500e9a:     	callq	0x1406868b0 <.text+0x6858b0>
140500e9f:     	movq	%rax, %r12
140500ea2:     	leaq	0x28(%rsp), %r15
140500ea7:     	movq	%r15, %rcx
140500eaa:     	callq	0x140683c10 <.text+0x682c10>
140500eaf:     	movq	%rax, 0x38(%rsp)
140500eb4:     	leaq	0x38(%rsp), %r14
140500eb9:     	movq	%r14, %rcx
140500ebc:     	callq	0x1400885d0 <.text+0x875d0>
140500ec1:     	movq	%rax, 0x58(%rsp)
140500ec6:     	movabsq	$0x41223169ff28813b, %r13 # imm = 0x41223169FF28813B
140500ed0:     	movq	%r13, 0x60(%rsp)
140500ed5:     	movabsq	$-0x58648d975d569727, %rbp # imm = 0xA79B7268A2A968D9
140500edf:     	movq	%rbp, 0x68(%rsp)
140500ee4:     	leaq	0x58(%rsp), %rcx
140500ee9:     	callq	0x1405bdf20 <.text+0x5bcf20>
140500eee:     	movq	%r12, 0x58(%rsp)
140500ef3:     	movq	%rax, 0x60(%rsp)
140500ef8:     	leaq	0x58(%rsp), %rcx
140500efd:     	callq	0x1405bd910 <.text+0x5bc910>
140500f02:     	movq	%rax, %r12
140500f05:     	movq	%rdi, %rcx
140500f08:     	movq	%rax, %rdx
140500f0b:     	callq	0x140686450 <.text+0x685450>
140500f10:     	testb	%al, %al
140500f12:     	je	0x140500f2c <.text+0x4fff2c>
140500f14:     	leaq	0x58(%rsp), %rbx
140500f19:     	movq	%rbx, %rcx
140500f1c:     	movq	%rdi, %rdx
140500f1f:     	movq	%r12, %r8
140500f22:     	callq	0x140686530 <.text+0x685530>
140500f27:     	jmp	0x14050106c <.text+0x50006c>
140500f2c:     	movq	0x312b475(%rip), %rax   # 0x14362c3a8
140500f33:     	movq	(%rax), %rax
140500f36:     	testq	%rax, %rax
140500f39:     	je	0x140501014 <.text+0x500014>
140500f3f:     	movq	%rbx, 0x38(%rsp)
140500f44:     	leaq	0x32955(%rip), %rax     # 0x1405338a0 <.text+0x5328a0>
140500f4b:     	movq	%rax, 0x40(%rsp)
140500f50:     	movq	%r15, 0x48(%rsp)
140500f55:     	movq	%rax, 0x50(%rsp)
140500f5a:     	leaq	0x24ecdf7(%rip), %rcx   # 0x1429edd58
140500f61:     	callq	0x14067dd80 <.text+0x67cd80>
140500f66:     	movups	(%rax), %xmm0
140500f69:     	movl	0x10(%rax), %eax
140500f6c:     	movq	$0x1, 0x88(%rsp)
140500f78:     	leaq	0x24ecda9(%rip), %rcx   # 0x1429edd28
140500f7f:     	movq	%rcx, 0x90(%rsp)
140500f87:     	movq	$0x14, 0x98(%rsp)
140500f93:     	leaq	0x24eccfe(%rip), %rdx   # 0x1429edc98
140500f9a:     	movq	%rdx, 0xa8(%rsp)
140500fa2:     	movq	$0x2, 0xb0(%rsp)
140500fae:     	movq	%r14, 0xb8(%rsp)
140500fb6:     	movq	$0x2, 0xc0(%rsp)
140500fc2:     	movq	$0x0, 0xc8(%rsp)
140500fce:     	movq	$0x0, 0x58(%rsp)
140500fd7:     	movq	%rcx, 0x60(%rsp)
140500fdc:     	movq	$0x14, 0x68(%rsp)
140500fe5:     	movq	$0x0, 0x70(%rsp)
140500fee:     	movups	%xmm0, 0x78(%rsp)
140500ff3:     	movl	$0x1, 0xa0(%rsp)
140500ffe:     	movl	%eax, 0xa4(%rsp)
140501005:     	leaq	0x27(%rsp), %rcx
14050100a:     	leaq	0x58(%rsp), %rdx
14050100f:     	callq	0x140d4a330 <.text+0xd49330>
140501014:     	movups	0x28(%rsp), %xmm0
140501019:     	movq	0x18(%rbx), %rbx
14050101d:     	movups	%xmm0, 0x38(%rsp)
140501022:     	leaq	0x38(%rsp), %rcx
140501027:     	callq	0x140683c10 <.text+0x682c10>
14050102c:     	movq	%rax, 0x58(%rsp)
140501031:     	movq	%r13, 0x60(%rsp)
140501036:     	movq	%rbp, 0x68(%rsp)
14050103b:     	leaq	0x58(%rsp), %rcx
140501040:     	callq	0x1405bdf20 <.text+0x5bcf20>
140501045:     	movq	%rbx, 0x58(%rsp)
14050104a:     	movq	%rax, 0x60(%rsp)
14050104f:     	leaq	0x58(%rsp), %rcx
140501054:     	callq	0x1405bd910 <.text+0x5bc910>
140501059:     	leaq	0x58(%rsp), %rbx
14050105e:     	movq	%rbx, %rcx
140501061:     	movq	%rdi, %rdx
140501064:     	movq	%rax, %r8
140501067:     	callq	0x1406861b0 <.text+0x6851b0>
14050106c:     	movq	%rsi, %rcx
14050106f:     	movq	%rbx, %rdx
140501072:     	callq	0x140504f10 <.text+0x503f10>
140501077:     	nop
140501078:     	addq	$0xd8, %rsp
14050107f:     	popq	%rbx
140501080:     	popq	%rbp
140501081:     	popq	%rdi
140501082:     	popq	%rsi
140501083:     	popq	%r12
140501085:     	popq	%r13
140501087:     	popq	%r14
140501089:     	popq	%r15
14050108b:     	retq
