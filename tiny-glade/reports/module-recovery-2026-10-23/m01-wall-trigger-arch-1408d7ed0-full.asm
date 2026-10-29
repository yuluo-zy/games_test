
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408d7ed0:     	pushq	%r15
1408d7ed2:     	pushq	%r14
1408d7ed4:     	pushq	%r13
1408d7ed6:     	pushq	%r12
1408d7ed8:     	pushq	%rsi
1408d7ed9:     	pushq	%rdi
1408d7eda:     	pushq	%rbp
1408d7edb:     	pushq	%rbx
1408d7edc:     	subq	$0x168, %rsp            # imm = 0x168
1408d7ee3:     	movaps	%xmm6, 0x150(%rsp)
1408d7eeb:     	movq	%r8, %rsi
1408d7eee:     	movq	%rdx, %rdi
1408d7ef1:     	movq	0x8(%rcx), %rax
1408d7ef5:     	movq	0x10(%rcx), %rdx
1408d7ef9:     	leaq	(%rax,%rdx,4), %r8
1408d7efd:     	movq	0x20(%rcx), %r9
1408d7f01:     	movq	0x28(%rcx), %rcx
1408d7f05:     	leaq	(%rcx,%rcx,2), %r10
1408d7f09:     	leaq	(%r9,%r10,4), %r10
1408d7f0d:     	cmpq	%rdx, %rcx
1408d7f10:     	cmovaeq	%rdx, %rcx
1408d7f14:     	movq	%rax, 0xa0(%rsp)
1408d7f1c:     	movq	%r8, 0xa8(%rsp)
1408d7f24:     	movq	%r9, 0xb0(%rsp)
1408d7f2c:     	movq	%r10, 0xb8(%rsp)
1408d7f34:     	movq	%rcx, 0xc8(%rsp)
1408d7f3c:     	movq	%rdx, 0xd0(%rsp)
1408d7f44:     	movq	$0x0, 0xd8(%rsp)
1408d7f50:     	testq	%rcx, %rcx
1408d7f53:     	je	0x1408d8341 <.text+0x8d7341>
1408d7f59:     	leaq	0x2c(%rsp), %r12
1408d7f5e:     	xorl	%eax, %eax
1408d7f60:     	leaq	0x20(%rsp), %rbx
1408d7f65:     	leaq	0x50(%rsp), %r14
1408d7f6a:     	xorps	%xmm6, %xmm6
1408d7f6d:     	leaq	0x108(%rsp), %r15
1408d7f75:     	leaq	0x12c(%rsp), %r13
1408d7f7d:     	jmp	0x1408d7fc7 <.text+0x8d6fc7>
1408d7f7f:     	nop
1408d7f80:     	movq	0x8(%rsi), %rax
1408d7f84:     	leaq	(,%rbp,8), %rcx
1408d7f8c:     	addq	%rbp, %rcx
1408d7f8f:     	movl	0x40(%rsp), %edx
1408d7f93:     	movl	%edx, 0x20(%rax,%rcx,4)
1408d7f97:     	movaps	0x20(%rsp), %xmm0
1408d7f9c:     	movaps	0x30(%rsp), %xmm1
1408d7fa1:     	movups	%xmm1, 0x10(%rax,%rcx,4)
1408d7fa6:     	movups	%xmm0, (%rax,%rcx,4)
1408d7faa:     	incq	%rbp
1408d7fad:     	movq	%rbp, 0x10(%rsi)
1408d7fb1:     	movq	0xc0(%rsp), %rax
1408d7fb9:     	cmpq	0xc8(%rsp), %rax
1408d7fc1:     	jae	0x1408d8341 <.text+0x8d7341>
1408d7fc7:     	leaq	0x1(%rax), %rcx
1408d7fcb:     	movq	%rcx, 0xc0(%rsp)
1408d7fd3:     	leaq	(,%rax,4), %rcx
1408d7fdb:     	addq	0xa0(%rsp), %rcx
1408d7fe3:     	leaq	(%rax,%rax,2), %rax
1408d7fe7:     	shlq	$0x2, %rax
1408d7feb:     	addq	0xb0(%rsp), %rax
1408d7ff3:     	cmpq	$0x0, 0xd8(%rsp)
1408d7ffc:     	je	0x1408d8030 <.text+0x8d7030>
1408d7ffe:     	movq	0xe8(%rsp), %rdx
1408d8006:     	movq	0xf0(%rsp), %r8
1408d800e:     	movq	%rcx, 0xe8(%rsp)
1408d8016:     	movq	%rax, 0xf0(%rsp)
1408d801e:     	movq	%rdx, 0xd8(%rsp)
1408d8026:     	movq	%r8, 0xe0(%rsp)
1408d802e:     	jmp	0x1408d809d <.text+0x8d709d>
1408d8030:     	movq	$0x1, 0x50(%rsp)
1408d8039:     	movq	%rcx, 0x58(%rsp)
1408d803e:     	movq	%rax, 0x60(%rsp)
1408d8043:     	leaq	0xa0(%rsp), %rax
1408d804b:     	movq	%rax, 0x68(%rsp)
1408d8050:     	movq	%rbx, %rcx
1408d8053:     	movq	%r14, %rdx
1408d8056:     	callq	0x14085e430 <.text+0x85d430>
1408d805b:     	movups	0x20(%rsp), %xmm0
1408d8060:     	movups	0x30(%rsp), %xmm1
1408d8065:     	leaq	0xd8(%rsp), %rax
1408d806d:     	movups	%xmm1, 0x10(%rax)
1408d8071:     	movups	%xmm0, (%rax)
1408d8074:     	movq	0xd8(%rsp), %rdx
1408d807c:     	testq	%rdx, %rdx
1408d807f:     	je	0x1408d8341 <.text+0x8d7341>
1408d8085:     	movq	0xe0(%rsp), %r8
1408d808d:     	movq	0xe8(%rsp), %rcx
1408d8095:     	movq	0xf0(%rsp), %rax
1408d809d:     	movl	0x8(%r8), %r9d
1408d80a1:     	movl	%r9d, 0x78(%rsp)
1408d80a6:     	movq	(%r8), %r8
1408d80a9:     	movq	%r8, 0x70(%rsp)
1408d80ae:     	movl	0x8(%rax), %r8d
1408d80b2:     	movl	%r8d, 0x88(%rsp)
1408d80ba:     	movq	(%rax), %r8
1408d80bd:     	movq	%r8, 0x80(%rsp)
1408d80c5:     	movss	(%rdx), %xmm0
1408d80c9:     	movaps	%xmm0, %xmm1
1408d80cc:     	mulss	%xmm6, %xmm1
1408d80d0:     	movss	0x78(%rsp), %xmm2
1408d80d6:     	addss	%xmm1, %xmm2
1408d80da:     	movsd	0x70(%rsp), %xmm3
1408d80e0:     	unpcklps	%xmm0, %xmm1            # xmm1 = xmm1[0],xmm0[0],xmm1[1],xmm0[1]
1408d80e3:     	addps	%xmm3, %xmm1
1408d80e6:     	movlps	%xmm1, 0x90(%rsp)
1408d80ee:     	movss	%xmm2, 0x98(%rsp)
1408d80f7:     	movss	(%rcx), %xmm0
1408d80fb:     	movaps	%xmm0, %xmm1
1408d80fe:     	mulss	%xmm6, %xmm1
1408d8102:     	movss	0x88(%rsp), %xmm2
1408d810b:     	addss	%xmm1, %xmm2
1408d810f:     	movsd	0x80(%rsp), %xmm3
1408d8118:     	unpcklps	%xmm0, %xmm1            # xmm1 = xmm1[0],xmm0[0],xmm1[1],xmm0[1]
1408d811b:     	addps	%xmm3, %xmm1
1408d811e:     	movlps	%xmm1, 0xf8(%rsp)
1408d8126:     	movss	%xmm2, 0x100(%rsp)
1408d812f:     	movl	0x8(%rax), %ecx
1408d8132:     	movl	%ecx, 0x58(%rsp)
1408d8136:     	movq	(%rax), %rax
1408d8139:     	movq	%rax, 0x50(%rsp)
1408d813e:     	movl	0x98(%rsp), %eax
1408d8145:     	movl	%eax, 0x28(%rsp)
1408d8149:     	movq	0x90(%rsp), %rax
1408d8151:     	movq	%rax, 0x20(%rsp)
1408d8156:     	movq	%r15, %rcx
1408d8159:     	leaq	0x70(%rsp), %rdx
1408d815e:     	movq	%r14, %r8
1408d8161:     	movq	%rbx, %r9
1408d8164:     	callq	0x14123ad70 <.text+0x1239d70>
1408d8169:     	movq	%r14, %rcx
1408d816c:     	movq	%r15, %rdx
1408d816f:     	callq	0x14123ada0 <.text+0x1239da0>
1408d8174:     	movl	0x58(%rsp), %eax
1408d8178:     	movl	%eax, 0x28(%rsp)
1408d817c:     	movq	0x50(%rsp), %rcx
1408d8181:     	movq	%rcx, 0x20(%rsp)
1408d8186:     	movl	%eax, 0x8(%r12)
1408d818b:     	movq	%rcx, (%r12)
1408d818f:     	movl	0x58(%rsp), %eax
1408d8193:     	movl	%eax, 0x14(%r12)
1408d8198:     	movq	0x50(%rsp), %rax
1408d819d:     	movq	%rax, 0xc(%r12)
1408d81a2:     	movq	0x10(%rdi), %rbp
1408d81a6:     	cmpq	(%rdi), %rbp
1408d81a9:     	jne	0x1408d81ba <.text+0x8d71ba>
1408d81ab:     	movq	%rdi, %rcx
1408d81ae:     	leaq	0x21cc03b(%rip), %rdx   # 0x142aa41f0
1408d81b5:     	callq	0x140b5fc30 <.text+0xb5ec30>
1408d81ba:     	movq	0x8(%rdi), %rax
1408d81be:     	leaq	(,%rbp,8), %rcx
1408d81c6:     	addq	%rbp, %rcx
1408d81c9:     	movl	0x40(%rsp), %edx
1408d81cd:     	movl	%edx, 0x20(%rax,%rcx,4)
1408d81d1:     	movups	0x20(%rsp), %xmm0
1408d81d6:     	movups	0x30(%rsp), %xmm1
1408d81db:     	movups	%xmm1, 0x10(%rax,%rcx,4)
1408d81e0:     	movups	%xmm0, (%rax,%rcx,4)
1408d81e4:     	incq	%rbp
1408d81e7:     	movq	%rbp, 0x10(%rdi)
1408d81eb:     	movl	0x128(%rsp), %eax
1408d81f2:     	movl	%eax, 0x40(%rsp)
1408d81f6:     	movups	0x108(%rsp), %xmm0
1408d81fe:     	movups	0x118(%rsp), %xmm1
1408d8206:     	movaps	%xmm1, 0x30(%rsp)
1408d820b:     	movaps	%xmm0, 0x20(%rsp)
1408d8210:     	movq	0x10(%rsi), %rbp
1408d8214:     	cmpq	(%rsi), %rbp
1408d8217:     	jne	0x1408d8228 <.text+0x8d7228>
1408d8219:     	movq	%rsi, %rcx
1408d821c:     	leaq	0x21cbfe5(%rip), %rdx   # 0x142aa4208
1408d8223:     	callq	0x140b5fc30 <.text+0xb5ec30>
1408d8228:     	movq	0x8(%rsi), %rax
1408d822c:     	leaq	(,%rbp,8), %rcx
1408d8234:     	addq	%rbp, %rcx
1408d8237:     	movl	0x40(%rsp), %edx
1408d823b:     	movl	%edx, 0x20(%rax,%rcx,4)
1408d823f:     	movaps	0x20(%rsp), %xmm0
1408d8244:     	movaps	0x30(%rsp), %xmm1
1408d8249:     	movups	%xmm1, 0x10(%rax,%rcx,4)
1408d824e:     	movups	%xmm0, (%rax,%rcx,4)
1408d8252:     	incq	%rbp
1408d8255:     	movq	%rbp, 0x10(%rsi)
1408d8259:     	movq	%r13, %rcx
1408d825c:     	leaq	0x80(%rsp), %rdx
1408d8264:     	leaq	0xf8(%rsp), %r8
1408d826c:     	leaq	0x90(%rsp), %r9
1408d8274:     	callq	0x14123ad70 <.text+0x1239d70>
1408d8279:     	movq	%r14, %rcx
1408d827c:     	movq	%r13, %rdx
1408d827f:     	callq	0x14123ada0 <.text+0x1239da0>
1408d8284:     	movl	0x58(%rsp), %eax
1408d8288:     	movl	%eax, 0x28(%rsp)
1408d828c:     	movq	0x50(%rsp), %rcx
1408d8291:     	movq	%rcx, 0x20(%rsp)
1408d8296:     	movl	%eax, 0x8(%r12)
1408d829b:     	movq	%rcx, (%r12)
1408d829f:     	movl	0x58(%rsp), %eax
1408d82a3:     	movl	%eax, 0x14(%r12)
1408d82a8:     	movq	0x50(%rsp), %rax
1408d82ad:     	movq	%rax, 0xc(%r12)
1408d82b2:     	movq	0x10(%rdi), %rbp
1408d82b6:     	cmpq	(%rdi), %rbp
1408d82b9:     	jne	0x1408d82ca <.text+0x8d72ca>
1408d82bb:     	movq	%rdi, %rcx
1408d82be:     	leaq	0x21cbf5b(%rip), %rdx   # 0x142aa4220
1408d82c5:     	callq	0x140b5fc30 <.text+0xb5ec30>
1408d82ca:     	movq	0x8(%rdi), %rax
1408d82ce:     	leaq	(,%rbp,8), %rcx
1408d82d6:     	addq	%rbp, %rcx
1408d82d9:     	movl	0x40(%rsp), %edx
1408d82dd:     	movl	%edx, 0x20(%rax,%rcx,4)
1408d82e1:     	movups	0x20(%rsp), %xmm0
1408d82e6:     	movups	0x30(%rsp), %xmm1
1408d82eb:     	movups	%xmm1, 0x10(%rax,%rcx,4)
1408d82f0:     	movups	%xmm0, (%rax,%rcx,4)
1408d82f4:     	incq	%rbp
1408d82f7:     	movq	%rbp, 0x10(%rdi)
1408d82fb:     	movl	0x14c(%rsp), %eax
1408d8302:     	movl	%eax, 0x40(%rsp)
1408d8306:     	movups	0x12c(%rsp), %xmm0
1408d830e:     	movups	0x13c(%rsp), %xmm1
1408d8316:     	movaps	%xmm1, 0x30(%rsp)
1408d831b:     	movaps	%xmm0, 0x20(%rsp)
1408d8320:     	movq	0x10(%rsi), %rbp
1408d8324:     	cmpq	(%rsi), %rbp
1408d8327:     	jne	0x1408d7f80 <.text+0x8d6f80>
1408d832d:     	movq	%rsi, %rcx
1408d8330:     	leaq	0x21cbf01(%rip), %rdx   # 0x142aa4238
1408d8337:     	callq	0x140b5fc30 <.text+0xb5ec30>
1408d833c:     	jmp	0x1408d7f80 <.text+0x8d6f80>
1408d8341:     	movaps	0x150(%rsp), %xmm6
1408d8349:     	addq	$0x168, %rsp            # imm = 0x168
1408d8350:     	popq	%rbx
1408d8351:     	popq	%rbp
1408d8352:     	popq	%rdi
1408d8353:     	popq	%rsi
1408d8354:     	popq	%r12
1408d8356:     	popq	%r13
1408d8358:     	popq	%r14
1408d835a:     	popq	%r15
1408d835c:     	retq
