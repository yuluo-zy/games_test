
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14227e140:     	pushq	%rbp
14227e141:     	pushq	%r15
14227e143:     	pushq	%r14
14227e145:     	pushq	%r13
14227e147:     	pushq	%r12
14227e149:     	pushq	%rsi
14227e14a:     	pushq	%rdi
14227e14b:     	pushq	%rbx
14227e14c:     	subq	$0x108, %rsp            # imm = 0x108
14227e153:     	leaq	0x80(%rsp), %rbp
14227e15b:     	movq	$-0x2, 0x80(%rbp)
14227e166:     	movq	%rdx, 0x70(%rbp)
14227e16a:     	movq	%rcx, 0x78(%rbp)
14227e16e:     	movq	(%r9), %rax
14227e171:     	movq	0x8(%r9), %r15
14227e175:     	movq	(%rax), %r9
14227e178:     	xorl	%ebx, %ebx
14227e17a:     	movq	%r9, %r10
14227e17d:     	subq	0x18(%r15), %r10
14227e181:     	cmovbq	%rbx, %r10
14227e185:     	subq	0x38(%r15), %r9
14227e189:     	cmovbq	%rbx, %r9
14227e18d:     	movq	0x10(%r15), %rdi
14227e191:     	movq	0x30(%r15), %rdx
14227e195:     	movq	%rdx, %r11
14227e198:     	subq	%r9, %r11
14227e19b:     	cmovbq	%rbx, %r11
14227e19f:     	setbe	%r13b
14227e1a3:     	movq	%rdi, %r14
14227e1a6:     	subq	%r10, %r14
14227e1a9:     	cmovbq	%rbx, %r14
14227e1ad:     	setbe	%sil
14227e1b1:     	movq	0x40(%r15), %rcx
14227e1b5:     	movq	%rcx, %rbx
14227e1b8:     	subq	%r11, %rbx
14227e1bb:     	subq	%r14, %rbx
14227e1be:     	movq	0x8(%r15), %r12
14227e1c2:     	movq	0x28(%r15), %r15
14227e1c6:     	movq	%rbx, (%rax)
14227e1c9:     	testb	%r13b, %sil
14227e1cc:     	jne	0x14227e3c1 <.text+0x227d3c1>
14227e1d2:     	movq	%r9, %rsi
14227e1d5:     	shlq	$0x4, %rsi
14227e1d9:     	addq	%rsi, %r15
14227e1dc:     	shlq	$0x4, %r11
14227e1e0:     	movq	%r10, %rsi
14227e1e3:     	shlq	$0x4, %rsi
14227e1e7:     	addq	%rsi, %r12
14227e1ea:     	shlq	$0x4, %r14
14227e1ee:     	xorl	%esi, %esi
14227e1f0:     	cmpq	%r10, %rdi
14227e1f3:     	movl	$0x8, %r10d
14227e1f9:     	cmovbq	%r10, %r12
14227e1fd:     	leaq	-0x10(%r12,%r14), %rdi
14227e202:     	cmovaq	%rdi, %rsi
14227e206:     	cmpq	%r9, %rdx
14227e209:     	cmovbq	%r10, %r15
14227e20d:     	leaq	-0x10(%r15,%r11), %rdx
14227e212:     	cmovbeq	%rsi, %rdx
14227e216:     	movq	%rcx, (%rax)
14227e219:     	cmpb	$0x0, 0x8(%rdx)
14227e21d:     	je	0x14227e28f <.text+0x227d28f>
14227e21f:     	movq	(%r8), %rsi
14227e222:     	cmpb	$0x0, (%rsi)
14227e225:     	jne	0x14227e27d <.text+0x227d27d>
14227e227:     	movq	0xf0(%rbp), %rax
14227e22e:     	movl	0x1c(%rax), %ecx
14227e231:     	movq	0x10(%rax), %rdx
14227e235:     	movl	%ecx, (%rdx)
14227e237:     	movq	(%rax), %rdi
14227e23a:     	movq	0x30(%rdi), %rbx
14227e23e:     	movq	0x40(%rdi), %r14
14227e242:     	cmpq	0x20(%rdi), %rbx
14227e246:     	jne	0x14227e25e <.text+0x227d25e>
14227e248:     	leaq	0x20(%rdi), %rcx
14227e24c:     	leaq	0xdba27d(%rip), %rdx    # 0x1430384d0
14227e253:     	movq	%r8, %r15
14227e256:     	callq	0x140562630 <.text+0x561630>
14227e25b:     	movq	%r15, %r8
14227e25e:     	movq	0x28(%rdi), %rax
14227e262:     	movq	%rbx, %rcx
14227e265:     	shlq	$0x4, %rcx
14227e269:     	movq	%r14, (%rax,%rcx)
14227e26d:     	movb	$0x3, 0x8(%rax,%rcx)
14227e272:     	incq	%rbx
14227e275:     	movq	%rbx, 0x30(%rdi)
14227e279:     	incq	0x40(%rdi)
14227e27d:     	movl	0x1c(%r8), %eax
14227e281:     	movq	0x10(%r8), %rcx
14227e285:     	movl	%eax, (%rcx)
14227e287:     	movb	$0x1, (%rsi)
14227e28a:     	jmp	0x14227e3c1 <.text+0x227d3c1>
14227e28f:     	movq	0x70(%rbp), %rax
14227e293:     	movq	(%rax), %rbx
14227e296:     	cmpb	$0x0, 0x1fc(%rbx)
14227e29d:     	jne	0x14227e3c1 <.text+0x227d3c1>
14227e2a3:     	movq	(%r8), %rax
14227e2a6:     	cmpb	$0x0, (%rax)
14227e2a9:     	jne	0x14227e3c1 <.text+0x227d3c1>
14227e2af:     	movq	0x78(%rbp), %rax
14227e2b3:     	movq	(%rax), %rax
14227e2b6:     	cmpl	$0x6, (%rax)
14227e2b9:     	jne	0x14227e3c1 <.text+0x227d3c1>
14227e2bf:     	movq	0xf8(%rbp), %rsi
14227e2c6:     	movzbl	0x4(%rax), %edi
14227e2ca:     	movb	$0xf, %cl
14227e2cc:     	callq	0x1409e8d90 <.text+0x9e7d90>
14227e2d1:     	movw	%ax, 0x1c(%rbp)
14227e2d5:     	movl	$0xc9, 0x18(%rbp)
14227e2dc:     	movl	0x128(%rbx), %eax
14227e2e2:     	movl	%eax, -0x28(%rbp)
14227e2e5:     	movq	0x120(%rbx), %rax
14227e2ec:     	movq	%rax, -0x30(%rbp)
14227e2f0:     	leaq	-0x60(%rbp), %rcx
14227e2f4:     	leaq	0x18(%rbp), %rdx
14227e2f8:     	leaq	-0x30(%rbp), %r8
14227e2fc:     	callq	0x141c4e4f0 <.text+0x1c4d4f0>
14227e301:     	addl	$0x6, %edi
14227e304:     	movups	-0x60(%rbp), %xmm0
14227e308:     	movups	-0x50(%rbp), %xmm1
14227e30c:     	movups	-0x40(%rbp), %xmm2
14227e310:     	movups	%xmm0, -0x1c(%rbp)
14227e314:     	movups	%xmm1, -0xc(%rbp)
14227e318:     	movups	%xmm2, 0x4(%rbp)
14227e31c:     	movl	0x1c(%rsi), %eax
14227e31f:     	movq	0x10(%rsi), %rcx
14227e323:     	movl	%eax, (%rcx)
14227e325:     	movq	(%rsi), %rsi
14227e328:     	movq	0x40(%rsi), %rax
14227e32c:     	movq	%rax, 0x68(%rbp)
14227e330:     	movl	%edi, 0x18(%rbp)
14227e333:     	movl	0x10(%rbp), %eax
14227e336:     	movl	%eax, 0x5c(%rbp)
14227e339:     	movups	-0x30(%rbp), %xmm0
14227e33d:     	movups	-0x20(%rbp), %xmm1
14227e341:     	movups	-0x10(%rbp), %xmm2
14227e345:     	movups	(%rbp), %xmm3
14227e349:     	movups	%xmm3, 0x4c(%rbp)
14227e34d:     	movups	%xmm2, 0x3c(%rbp)
14227e351:     	movups	%xmm1, 0x2c(%rbp)
14227e355:     	movups	%xmm0, 0x1c(%rbp)
14227e359:     	movw	$0x300, 0x60(%rbp)      # imm = 0x300
14227e35f:     	movq	0x30(%rsi), %rdi
14227e363:     	cmpq	0x20(%rsi), %rdi
14227e367:     	jne	0x14227e379 <.text+0x227d379>
14227e369:     	leaq	0x20(%rsi), %rcx
14227e36d:     	leaq	0xdba15c(%rip), %rdx    # 0x1430384d0
14227e374:     	callq	0x140562f30 <.text+0x561f30>
14227e379:     	movq	0x28(%rsi), %rax
14227e37d:     	imulq	$0x58, %rdi, %rcx
14227e381:     	movups	0x18(%rbp), %xmm0
14227e385:     	movups	0x28(%rbp), %xmm1
14227e389:     	movups	0x38(%rbp), %xmm2
14227e38d:     	movups	0x48(%rbp), %xmm3
14227e391:     	movups	0x58(%rbp), %xmm4
14227e395:     	movq	0x68(%rbp), %rdx
14227e399:     	movq	%rdx, 0x50(%rax,%rcx)
14227e39e:     	movups	%xmm4, 0x40(%rax,%rcx)
14227e3a3:     	movups	%xmm3, 0x30(%rax,%rcx)
14227e3a8:     	movups	%xmm2, 0x20(%rax,%rcx)
14227e3ad:     	movups	%xmm1, 0x10(%rax,%rcx)
14227e3b2:     	movups	%xmm0, (%rax,%rcx)
14227e3b6:     	incq	%rdi
14227e3b9:     	movq	%rdi, 0x30(%rsi)
14227e3bd:     	incq	0x40(%rsi)
14227e3c1:     	addq	$0x108, %rsp            # imm = 0x108
14227e3c8:     	popq	%rbx
14227e3c9:     	popq	%rdi
14227e3ca:     	popq	%rsi
14227e3cb:     	popq	%r12
14227e3cd:     	popq	%r13
14227e3cf:     	popq	%r14
14227e3d1:     	popq	%r15
14227e3d3:     	popq	%rbp
14227e3d4:     	retq
14227e3d5:     	nopw	%cs:(%rax,%rax)
