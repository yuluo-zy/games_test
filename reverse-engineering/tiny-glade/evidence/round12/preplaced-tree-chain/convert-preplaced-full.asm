
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14226be70:     	pushq	%rbp
14226be71:     	pushq	%r15
14226be73:     	pushq	%r14
14226be75:     	pushq	%r13
14226be77:     	pushq	%r12
14226be79:     	pushq	%rsi
14226be7a:     	pushq	%rdi
14226be7b:     	pushq	%rbx
14226be7c:     	subq	$0x2a8, %rsp            # imm = 0x2A8
14226be83:     	leaq	0x80(%rsp), %rbp
14226be8b:     	movaps	%xmm15, 0x210(%rbp)
14226be93:     	movaps	%xmm14, 0x200(%rbp)
14226be9b:     	movaps	%xmm13, 0x1f0(%rbp)
14226bea3:     	movaps	%xmm12, 0x1e0(%rbp)
14226beab:     	movaps	%xmm11, 0x1d0(%rbp)
14226beb3:     	movaps	%xmm10, 0x1c0(%rbp)
14226bebb:     	movaps	%xmm9, 0x1b0(%rbp)
14226bec3:     	movaps	%xmm8, 0x1a0(%rbp)
14226becb:     	movaps	%xmm7, 0x190(%rbp)
14226bed2:     	movaps	%xmm6, 0x180(%rbp)
14226bed9:     	movq	$-0x2, 0x178(%rbp)
14226bee4:     	movq	(%rcx), %rax
14226bee7:     	movq	0x8(%rcx), %rcx
14226beeb:     	movq	(%rax), %r10
14226beee:     	xorl	%r11d, %r11d
14226bef1:     	movq	%r10, %rsi
14226bef4:     	subq	0x18(%rcx), %rsi
14226bef8:     	cmovbq	%r11, %rsi
14226befc:     	subq	0x38(%rcx), %r10
14226bf00:     	cmovbq	%r11, %r10
14226bf04:     	movq	0x30(%rcx), %rdi
14226bf08:     	subq	%r10, %rdi
14226bf0b:     	cmovbq	%r11, %rdi
14226bf0f:     	setbe	%r10b
14226bf13:     	movq	0x10(%rcx), %rbx
14226bf17:     	subq	%rsi, %rbx
14226bf1a:     	cmovbq	%r11, %rbx
14226bf1e:     	movq	0x40(%rcx), %rcx
14226bf22:     	setbe	%r11b
14226bf26:     	addq	%rdi, %rbx
14226bf29:     	movq	%rcx, %rsi
14226bf2c:     	subq	%rbx, %rsi
14226bf2f:     	movq	%rsi, (%rax)
14226bf32:     	testb	%r10b, %r11b
14226bf35:     	je	0x14226c69a <.text+0x226b69a>
14226bf3b:     	testb	$0x1, 0x8(%r8)
14226bf40:     	je	0x14226c6a9 <.text+0x226b6a9>
14226bf46:     	movq	(%r8), %rax
14226bf49:     	incq	%rax
14226bf4c:     	movq	%rax, (%r8)
14226bf4f:     	cmpq	$0x2, %rax
14226bf53:     	jbe	0x14226c6a9 <.text+0x226b6a9>
14226bf59:     	movq	%r9, %rsi
14226bf5c:     	movq	%rdx, %r14
14226bf5f:     	movq	0x290(%rbp), %rdi
14226bf66:     	movb	$0x0, 0x8(%r8)
14226bf6b:     	movq	0x13c0436(%rip), %rax   # 0x14362c3a8
14226bf72:     	movq	(%rax), %rax
14226bf75:     	cmpq	$0x3, %rax
14226bf79:     	jb	0x14226c037 <.text+0x226b037>
14226bf7f:     	leaq	0xdc9712(%rip), %rcx    # 0x143035698
14226bf86:     	callq	0x14067dd80 <.text+0x67cd80>
14226bf8b:     	movdqu	(%rax), %xmm0
14226bf8f:     	movl	0x10(%rax), %eax
14226bf92:     	movq	$0x3, 0xb0(%rbp)
14226bf9d:     	leaq	0xdc970c(%rip), %rcx    # 0x1430356b0
14226bfa4:     	movq	%rcx, 0xb8(%rbp)
14226bfab:     	movq	$0x24, 0xc0(%rbp)
14226bfb6:     	leaq	0xdc9693(%rip), %rdx    # 0x143035650
14226bfbd:     	movq	%rdx, 0xd0(%rbp)
14226bfc4:     	movq	$0x1, 0xd8(%rbp)
14226bfcf:     	movq	$0x8, 0xe0(%rbp)
14226bfda:     	movq	$0x0, 0x80(%rbp)
14226bfe5:     	xorps	%xmm1, %xmm1
14226bfe8:     	movups	%xmm1, 0xe8(%rbp)
14226bfef:     	movq	%rcx, 0x88(%rbp)
14226bff6:     	movq	$0x24, 0x90(%rbp)
14226c001:     	movq	$0x0, 0x98(%rbp)
14226c00c:     	movdqu	%xmm0, 0xa0(%rbp)
14226c014:     	movl	$0x1, 0xc8(%rbp)
14226c01e:     	movl	%eax, 0xcc(%rbp)
14226c024:     	leaq	0x176(%rbp), %rcx
14226c02b:     	leaq	0x80(%rbp), %rdx
14226c032:     	callq	0x140d4a330 <.text+0xd49330>
14226c037:     	movq	$0x0, 0x140(%rbp)
14226c042:     	movq	$0x8, 0x148(%rbp)
14226c04d:     	movq	$0x0, 0x150(%rbp)
14226c058:     	movq	(%rdi), %rcx
14226c05b:     	callq	0x14067dd80 <.text+0x67cd80>
14226c060:     	leaq	0x8(%rax), %rcx
14226c064:     	leaq	0x20(%rax), %rdx
14226c068:     	leaq	0x10(%rax), %r8
14226c06c:     	movq	%rax, %r9
14226c06f:     	addq	$0x28, %r9
14226c073:     	cmpb	$0x0, 0x298(%rax)
14226c07a:     	cmovneq	%rdx, %rcx
14226c07e:     	cmoveq	%r8, %r9
14226c082:     	movq	(%r9), %rax
14226c085:     	movq	(%rcx), %rcx
14226c088:     	movq	%rcx, 0x10(%rbp)
14226c08c:     	movq	%rax, 0x18(%rbp)
14226c090:     	leaq	-0x50(%rbp), %rcx
14226c094:     	movq	%rsi, %rdx
14226c097:     	callq	0x140ae4c20 <.text+0xae3c20>
14226c09c:     	movq	0x298(%rbp), %rax
14226c0a3:     	movq	(%rax), %rcx
14226c0a6:     	callq	0x14067dd80 <.text+0x67cd80>
14226c0ab:     	movq	0x2a8(%rbp), %rcx
14226c0b2:     	leaq	0x8(%rax), %rdx
14226c0b6:     	leaq	0x20(%rax), %r8
14226c0ba:     	movq	%rax, %r9
14226c0bd:     	addq	$0x28, %r9
14226c0c1:     	cmpb	$0x0, 0x298(%rax)
14226c0c8:     	cmovneq	%r8, %rdx
14226c0cc:     	leaq	0x10(%rax), %rax
14226c0d0:     	cmoveq	%rax, %r9
14226c0d4:     	movq	(%r9), %rax
14226c0d7:     	movq	(%rdx), %rdx
14226c0da:     	movq	%rdx, (%rbp)
14226c0de:     	movq	%rax, 0x8(%rbp)
14226c0e2:     	movq	(%rcx), %rcx
14226c0e5:     	leaq	0xec8f94(%rip), %rdx    # 0x143135080
14226c0ec:     	callq	0x1425df9c0 <.text+0x25de9c0>
14226c0f1:     	movq	%rax, %rsi
14226c0f4:     	testq	%rax, %rax
14226c0f7:     	je	0x14226ca94 <.text+0x226ba94>
14226c0fd:     	leaq	0xec90b4(%rip), %rax    # 0x1431351b8
14226c104:     	movq	%rax, 0x100(%rbp)
14226c10b:     	movq	$0x3f, 0x108(%rbp)
14226c116:     	movq	%rsi, %rdi
14226c119:     	addq	$0x10, %rdi
14226c11d:     	cmpq	$0x3f, 0x18(%rsi)
14226c122:     	jne	0x14226ca63 <.text+0x226ba63>
14226c128:     	movq	(%rdi), %rdx
14226c12b:     	leaq	0xec9086(%rip), %rcx    # 0x1431351b8
14226c132:     	movl	$0x3f, %r8d
14226c138:     	callq	0x142923bf0 <.text+0x2922bf0>
14226c13d:     	testl	%eax, %eax
14226c13f:     	jne	0x14226ca63 <.text+0x226ba63>
14226c145:     	movq	(%rsi), %rcx
14226c148:     	movq	0x8(%rsi), %rax
14226c14c:     	callq	*0x18(%rax)
14226c14f:     	testq	%rax, %rax
14226c152:     	je	0x14226ca94 <.text+0x226ba94>
14226c158:     	movq	0x2b0(%rbp), %rcx
14226c15f:     	movq	(%rax), %rdx
14226c162:     	movq	%rdx, 0x38(%rbp)
14226c166:     	movq	0x8(%rax), %rax
14226c16a:     	movq	%rax, 0x40(%rbp)
14226c16e:     	movq	(%rcx), %rdx
14226c171:     	leaq	0x80(%rbp), %rcx
14226c178:     	callq	0x140a1b100 <.text+0xa1a100>
14226c17d:     	movq	0x2a0(%rbp), %rax
14226c184:     	movq	0x2b8(%rbp), %rcx
14226c18b:     	movq	0x80(%rbp), %rsi
14226c192:     	movq	0x88(%rbp), %rdi
14226c199:     	movzwl	0x98(%rbp), %r13d
14226c1a1:     	movq	0xa0(%rbp), %r12
14226c1a8:     	movq	(%rcx), %rbx
14226c1ab:     	movq	0x8(%rcx), %r15
14226c1af:     	movq	(%rax), %rax
14226c1b2:     	movq	%rax, 0x50(%rbp)
14226c1b6:     	movl	0x1c(%r14), %eax
14226c1ba:     	movl	%eax, 0x12c(%rbp)
14226c1c0:     	movq	(%r14), %rax
14226c1c3:     	movq	0x10(%r14), %r14
14226c1c7:     	movss	0x80f489(%rip), %xmm6   # 0x142a7b658
14226c1cf:     	xorps	%xmm15, %xmm15
14226c1d3:     	movss	0x810b6d(%rip), %xmm7   # 0x142a7cd48
14226c1db:     	movq	%rax, 0x158(%rbp)
14226c1e2:     	addq	$0x20, %rax
14226c1e6:     	movq	%rax, 0x48(%rbp)
14226c1ea:     	movl	%r13d, %edx
14226c1ed:     	nopl	(%rax)
14226c1f0:     	testq	%r12, %r12
14226c1f3:     	je	0x14226c70b <.text+0x226b70b>
14226c1f9:     	testw	%dx, %dx
14226c1fc:     	je	0x14226c210 <.text+0x226b210>
14226c1fe:     	testq	%rsi, %rsi
14226c201:     	je	0x14226c70b <.text+0x226b70b>
14226c207:     	leal	-0x1(%rdx), %r13d
14226c20b:     	andl	%edx, %r13d
14226c20e:     	jmp	0x14226c23a <.text+0x226b23a>
14226c210:     	movdqa	(%rdi), %xmm0
14226c214:     	pmovmskb	%xmm0, %edx
14226c218:     	addq	$-0x100, %rsi
14226c21f:     	addq	$0x10, %rdi
14226c223:     	cmpl	$0xffff, %edx           # imm = 0xFFFF
14226c229:     	je	0x14226c210 <.text+0x226b210>
14226c22b:     	movl	$0xfffffffe, %eax       # imm = 0xFFFFFFFE
14226c230:     	subl	%edx, %eax
14226c232:     	notl	%edx
14226c234:     	movl	%edx, %r13d
14226c237:     	andl	%eax, %r13d
14226c23a:     	decq	%r12
14226c23d:     	tzcntl	%edx, %ecx
14226c241:     	shll	$0x4, %ecx
14226c244:     	movq	%rsi, %rax
14226c247:     	subq	%rcx, %rax
14226c24a:     	movq	-0x8(%rax), %rcx
14226c24e:     	movl	%ecx, %r8d
14226c251:     	movl	%r13d, %edx
14226c254:     	cmpq	0x10(%r15), %r8
14226c258:     	jae	0x14226c1f0 <.text+0x226b1f0>
14226c25a:     	movq	0x8(%r15), %r9
14226c25e:     	shrq	$0x20, %rcx
14226c262:     	leaq	(%r8,%r8,4), %r8
14226c266:     	movl	%r13d, %edx
14226c269:     	cmpl	%ecx, (%r9,%r8,4)
14226c26d:     	jne	0x14226c1f0 <.text+0x226b1f0>
14226c26f:     	leaq	(%r9,%r8,4), %rcx
14226c273:     	movl	0x4(%rcx), %r8d
14226c277:     	movl	%r13d, %edx
14226c27a:     	movl	$0xffffffff, %r9d       # imm = 0xFFFFFFFF
14226c280:     	cmpq	%r9, %r8
14226c283:     	je	0x14226c1f0 <.text+0x226b1f0>
14226c289:     	movl	%r13d, %edx
14226c28c:     	cmpq	%r8, 0x38(%rbx)
14226c290:     	jbe	0x14226c1f0 <.text+0x226b1f0>
14226c296:     	movq	0x28(%rbx), %rdx
14226c29a:     	movl	%r8d, %r9d
14226c29d:     	shrl	$0x6, %r9d
14226c2a1:     	movq	(%rdx,%r9,8), %rdx
14226c2a5:     	btq	%r8, %rdx
14226c2a9:     	movl	%r13d, %edx
14226c2ac:     	jae	0x14226c1f0 <.text+0x226b1f0>
14226c2b2:     	movl	0xc(%rcx), %edx
14226c2b5:     	movq	0x1a8(%r15), %r8
14226c2bc:     	leaq	(%rdx,%rdx,8), %r10
14226c2c0:     	leaq	(%r8,%r10,8), %rdx
14226c2c4:     	movq	0x110(%rbx), %r9
14226c2cb:     	movq	0x40(%r8,%r10,8), %r8
14226c2d0:     	cmpq	%r9, %r8
14226c2d3:     	jbe	0x14226c2f8 <.text+0x226b2f8>
14226c2d5:     	movq	0x38(%rdx), %r10
14226c2d9:     	movq	(%r10,%r9,8), %r9
14226c2dd:     	testq	%r9, %r9
14226c2e0:     	je	0x14226c2f8 <.text+0x226b2f8>
14226c2e2:     	movq	0x18(%rdx), %r10
14226c2e6:     	notq	%r9
14226c2e9:     	leaq	(%r9,%r9,2), %r9
14226c2ed:     	shlq	$0x4, %r9
14226c2f1:     	movq	0x10(%r10,%r9), %r10
14226c2f6:     	jmp	0x14226c2fb <.text+0x226b2fb>
14226c2f8:     	xorl	%r10d, %r10d
14226c2fb:     	movq	0x118(%rbx), %r9
14226c302:     	cmpq	%r9, %r8
14226c305:     	movq	%r14, 0x130(%rbp)
14226c30c:     	jbe	0x14226c338 <.text+0x226b338>
14226c30e:     	movq	0x38(%rdx), %r8
14226c312:     	movq	(%r8,%r9,8), %r8
14226c316:     	testq	%r8, %r8
14226c319:     	je	0x14226c338 <.text+0x226b338>
14226c31b:     	movq	0x18(%rdx), %rdx
14226c31f:     	notq	%r8
14226c322:     	leaq	(%r8,%r8,2), %r8
14226c326:     	shlq	$0x4, %r8
14226c32a:     	movq	0x10(%rdx,%r8), %rdx
14226c32f:     	movq	%rdx, 0x168(%rbp)
14226c336:     	jmp	0x14226c343 <.text+0x226b343>
14226c338:     	movq	$0x0, 0x168(%rbp)
14226c343:     	movq	-0x10(%rax), %rax
14226c347:     	movq	%rax, 0x60(%rbp)
14226c34b:     	movl	0x10(%rcx), %eax
14226c34e:     	movq	%rax, 0x138(%rbp)
14226c355:     	leaq	(%rax,%rax,2), %rax
14226c359:     	shlq	$0x4, %rax
14226c35d:     	movq	%r10, %r14
14226c360:     	addq	%rax, %r14
14226c363:     	leaq	0x24(%rbp), %rcx
14226c367:     	movq	%r14, %rdx
14226c36a:     	callq	0x140ad51a0 <.text+0xad41a0>
14226c36f:     	movss	0x24(%rbp), %xmm8
14226c375:     	movss	0x2c(%rbp), %xmm9
14226c37b:     	movl	$0x43020000, 0x20(%rsp) # imm = 0x43020000
14226c383:     	leaq	-0x50(%rbp), %rcx
14226c387:     	movaps	%xmm8, %xmm1
14226c38b:     	movaps	%xmm9, %xmm2
14226c38f:     	movaps	%xmm6, %xmm3
14226c392:     	callq	0x140a14df0 <.text+0xa13df0>
14226c397:     	movdqa	%xmm0, %xmm10
14226c39c:     	movl	$0x43020000, 0x20(%rsp) # imm = 0x43020000
14226c3a4:     	leaq	-0x40(%rbp), %rcx
14226c3a8:     	movaps	%xmm8, %xmm1
14226c3ac:     	movaps	%xmm9, %xmm2
14226c3b0:     	movaps	%xmm6, %xmm3
14226c3b3:     	callq	0x140a14df0 <.text+0xa13df0>
14226c3b8:     	movdqa	%xmm0, %xmm11
14226c3bd:     	movl	$0x43020000, 0x20(%rsp) # imm = 0x43020000
14226c3c5:     	leaq	-0x30(%rbp), %rcx
14226c3c9:     	movaps	%xmm8, %xmm1
14226c3cd:     	movaps	%xmm9, %xmm2
14226c3d1:     	movaps	%xmm6, %xmm3
14226c3d4:     	callq	0x140a14df0 <.text+0xa13df0>
14226c3d9:     	movdqa	%xmm0, %xmm12
14226c3de:     	movl	$0x43020000, 0x20(%rsp) # imm = 0x43020000
14226c3e6:     	leaq	-0x20(%rbp), %rcx
14226c3ea:     	movaps	%xmm8, %xmm1
14226c3ee:     	movaps	%xmm9, %xmm2
14226c3f2:     	movaps	%xmm6, %xmm3
14226c3f5:     	callq	0x140a14df0 <.text+0xa13df0>
14226c3fa:     	movdqa	%xmm0, %xmm13
14226c3ff:     	movl	$0x43020000, 0x20(%rsp) # imm = 0x43020000
14226c407:     	leaq	0x10(%rbp), %rcx
14226c40b:     	movaps	%xmm8, %xmm1
14226c40f:     	movaps	%xmm9, %xmm2
14226c413:     	movaps	%xmm6, %xmm3
14226c416:     	callq	0x140a14df0 <.text+0xa13df0>
14226c41b:     	movdqa	%xmm0, %xmm14
14226c420:     	movl	$0x43020000, 0x20(%rsp) # imm = 0x43020000
14226c428:     	movq	%rbp, %rcx
14226c42b:     	movaps	%xmm8, %xmm1
14226c42f:     	movaps	%xmm9, %xmm2
14226c433:     	movaps	%xmm6, %xmm3
14226c436:     	callq	0x140a14df0 <.text+0xa13df0>
14226c43b:     	ucomiss	%xmm15, %xmm14
14226c43f:     	ja	0x14226c460 <.text+0x226b460>
14226c441:     	ucomiss	%xmm15, %xmm0
14226c445:     	ja	0x14226c460 <.text+0x226b460>
14226c447:     	movq	0x38(%rbp), %rcx
14226c44b:     	movq	0x40(%rbp), %rdx
14226c44f:     	movaps	%xmm8, %xmm2
14226c453:     	movaps	%xmm9, %xmm3
14226c457:     	callq	0x1409ea800 <.text+0x9e9800>
14226c45c:     	testb	%al, %al
14226c45e:     	je	0x14226c4dc <.text+0x226b4dc>
14226c460:     	movb	$0x0, 0x68(%rbp)
14226c464:     	movl	0x12c(%rbp), %eax
14226c46a:     	movq	0x130(%rbp), %rcx
14226c471:     	movl	%eax, (%rcx)
14226c473:     	movq	0x158(%rbp), %rax
14226c47a:     	movq	0x30(%rax), %r14
14226c47e:     	movq	0x40(%rax), %rcx
14226c482:     	movq	%rcx, 0x168(%rbp)
14226c489:     	cmpq	0x20(%rax), %r14
14226c48d:     	jne	0x14226c49f <.text+0x226b49f>
14226c48f:     	movq	0x48(%rbp), %rcx
14226c493:     	leaq	0xdc8876(%rip), %rdx    # 0x143034d10
14226c49a:     	callq	0x140562630 <.text+0x561630>
14226c49f:     	movq	0x158(%rbp), %rdx
14226c4a6:     	movq	0x28(%rdx), %rax
14226c4aa:     	movq	%r14, %rcx
14226c4ad:     	shlq	$0x4, %rcx
14226c4b1:     	movq	0x168(%rbp), %r8
14226c4b8:     	movq	%r8, (%rax,%rcx)
14226c4bc:     	movq	0x60(%rbp), %r8
14226c4c0:     	movq	%r8, 0x8(%rax,%rcx)
14226c4c5:     	incq	%r14
14226c4c8:     	movq	%r14, 0x30(%rdx)
14226c4cc:     	incq	0x40(%rdx)
14226c4d0:     	movq	0x130(%rbp), %r14
14226c4d7:     	jmp	0x14226c1ea <.text+0x226b1ea>
14226c4dc:     	addss	%xmm11, %xmm10
14226c4e1:     	addss	%xmm12, %xmm10
14226c4e6:     	addss	%xmm13, %xmm10
14226c4eb:     	xorps	%xmm0, %xmm0
14226c4ee:     	maxss	%xmm10, %xmm0
14226c4f3:     	movss	0x6b9409(%rip), %xmm1   # 0x142925904
14226c4fb:     	minss	%xmm0, %xmm1
14226c4ff:     	ucomiss	%xmm15, %xmm1
14226c503:     	setbe	0x68(%rbp)
14226c507:     	ja	0x14226c464 <.text+0x226b464>
14226c50d:     	leaq	-0x10(%rbp), %rcx
14226c511:     	movaps	%xmm8, %xmm1
14226c515:     	movaps	%xmm9, %xmm2
14226c519:     	movaps	%xmm7, %xmm3
14226c51c:     	callq	0x140c97540 <.text+0xc96540>
14226c521:     	movq	0x50(%rbp), %rcx
14226c525:     	leaq	-0x10(%rbp), %rdx
14226c529:     	leaq	0x68(%rbp), %r8
14226c52d:     	callq	0x1425defd0 <.text+0x25ddfd0>
14226c532:     	testb	$0x1, 0x68(%rbp)
14226c536:     	je	0x14226c464 <.text+0x226b464>
14226c53c:     	leaq	0x80(%rbp), %rcx
14226c543:     	movq	%r14, %rdx
14226c546:     	callq	0x140ad51a0 <.text+0xad41a0>
14226c54b:     	movq	0x138(%rbp), %rax
14226c552:     	shlq	$0x4, %rax
14226c556:     	movq	0x168(%rbp), %rcx
14226c55d:     	addq	%rax, %rcx
14226c560:     	movsd	0xc(%r14), %xmm8
14226c566:     	movl	0x14(%r14), %r14d
14226c56a:     	callq	0x141556130 <.text+0x1555130>
14226c56f:     	movq	%rdx, %r8
14226c572:     	movl	%r14d, 0x138(%rbp)
14226c579:     	testq	%rdx, %rdx
14226c57c:     	js	0x14226cad2 <.text+0x226bad2>
14226c582:     	movq	%rax, %r9
14226c585:     	testq	%r8, %r8
14226c588:     	movq	0x130(%rbp), %r14
14226c58f:     	movq	%r8, 0x168(%rbp)
14226c596:     	je	0x14226c5cc <.text+0x226b5cc>
14226c598:     	movq	%r9, 0x160(%rbp)
14226c59f:     	movzbl	0x13c8663(%rip), %eax   # 0x143634c09
14226c5a6:     	movl	$0x1, %edx
14226c5ab:     	movq	%r8, %rcx
14226c5ae:     	callq	0x140613c10 <.text+0x612c10>
14226c5b3:     	testq	%rax, %rax
14226c5b6:     	je	0x14226cad6 <.text+0x226bad6>
14226c5bc:     	movq	0x168(%rbp), %r8
14226c5c3:     	movq	0x160(%rbp), %r9
14226c5ca:     	jmp	0x14226c5d1 <.text+0x226b5d1>
14226c5cc:     	movl	$0x1, %eax
14226c5d1:     	movq	%rax, 0x160(%rbp)
14226c5d8:     	movq	%rax, %rcx
14226c5db:     	movq	%r9, %rdx
14226c5de:     	callq	0x142923c00 <.text+0x2922c00>
14226c5e3:     	movq	0x80(%rbp), %rax
14226c5ea:     	movq	%rax, 0x100(%rbp)
14226c5f1:     	movl	0x88(%rbp), %eax
14226c5f7:     	movl	%eax, 0x108(%rbp)
14226c5fd:     	movq	0x150(%rbp), %rax
14226c604:     	movq	%rax, 0x58(%rbp)
14226c608:     	cmpq	0x140(%rbp), %rax
14226c60f:     	jne	0x14226c624 <.text+0x226b624>
14226c611:     	leaq	0x140(%rbp), %rcx
14226c618:     	leaq	0xdc9139(%rip), %rdx    # 0x143035758
14226c61f:     	callq	0x140b5f9f0 <.text+0xb5e9f0>
14226c624:     	movq	0x148(%rbp), %rax
14226c62b:     	movq	0x58(%rbp), %r9
14226c62f:     	leaq	(%r9,%r9,8), %rcx
14226c633:     	movl	$0x0, (%rax,%rcx,8)
14226c63a:     	movq	0x60(%rbp), %rdx
14226c63e:     	movq	%rdx, 0x8(%rax,%rcx,8)
14226c643:     	movq	0x168(%rbp), %r8
14226c64a:     	movq	%r8, 0x10(%rax,%rcx,8)
14226c64f:     	movq	0x160(%rbp), %rdx
14226c656:     	movq	%rdx, 0x18(%rax,%rcx,8)
14226c65b:     	movq	%r8, 0x20(%rax,%rcx,8)
14226c660:     	movq	0x100(%rbp), %rdx
14226c667:     	movq	%rdx, 0x28(%rax,%rcx,8)
14226c66c:     	movl	0x108(%rbp), %edx
14226c672:     	movl	%edx, 0x30(%rax,%rcx,8)
14226c676:     	movlps	%xmm8, 0x34(%rax,%rcx,8)
14226c67c:     	movl	0x138(%rbp), %edx
14226c682:     	movl	%edx, 0x3c(%rax,%rcx,8)
14226c686:     	movb	$0x1, 0x40(%rax,%rcx,8)
14226c68b:     	incq	%r9
14226c68e:     	movq	%r9, 0x150(%rbp)
14226c695:     	jmp	0x14226c1ea <.text+0x226b1ea>
14226c69a:     	movq	%rcx, (%rax)
14226c69d:     	movb	$0x1, 0x8(%r8)
14226c6a2:     	movq	$0x1, (%r8)
14226c6a9:     	movaps	0x180(%rbp), %xmm6
14226c6b0:     	movaps	0x190(%rbp), %xmm7
14226c6b7:     	movaps	0x1a0(%rbp), %xmm8
14226c6bf:     	movaps	0x1b0(%rbp), %xmm9
14226c6c7:     	movaps	0x1c0(%rbp), %xmm10
14226c6cf:     	movaps	0x1d0(%rbp), %xmm11
14226c6d7:     	movaps	0x1e0(%rbp), %xmm12
14226c6df:     	movaps	0x1f0(%rbp), %xmm13
14226c6e7:     	movaps	0x200(%rbp), %xmm14
14226c6ef:     	movaps	0x210(%rbp), %xmm15
14226c6f7:     	addq	$0x2a8, %rsp            # imm = 0x2A8
14226c6fe:     	popq	%rbx
14226c6ff:     	popq	%rdi
14226c700:     	popq	%rsi
14226c701:     	popq	%r12
14226c703:     	popq	%r13
14226c705:     	popq	%r14
14226c707:     	popq	%r15
14226c709:     	popq	%rbp
14226c70a:     	retq
14226c70b:     	movq	0x148(%rbp), %rax
14226c712:     	movq	0x150(%rbp), %rcx
14226c719:     	movq	$0x0, 0x150(%rbp)
14226c724:     	leaq	(%rcx,%rcx,8), %rdx
14226c728:     	leaq	(%rax,%rdx,8), %rdx
14226c72c:     	movq	%rax, 0x80(%rbp)
14226c733:     	movq	%rdx, 0x88(%rbp)
14226c73a:     	leaq	0x140(%rbp), %rax
14226c741:     	movq	%rax, 0x90(%rbp)
14226c748:     	movq	%rcx, 0x98(%rbp)
14226c74f:     	movq	$0x0, 0xa0(%rbp)
14226c75a:     	leaq	0xdc8e3f(%rip), %r8     # 0x1430355a0
14226c761:     	leaq	0x68(%rbp), %rsi
14226c765:     	leaq	0x80(%rbp), %rdx
14226c76c:     	movq	%rsi, %rcx
14226c76f:     	callq	0x141bdb7e0 <.text+0x1bda7e0>
14226c774:     	movq	$0x0, 0x80(%rbp)
14226c77f:     	movq	$0x8, 0x88(%rbp)
14226c78a:     	movq	$0x0, 0x90(%rbp)
14226c795:     	movzbl	0x13c846d(%rip), %eax   # 0x143634c09
14226c79c:     	movl	$0x40, %ecx
14226c7a1:     	movl	$0x8, %edx
14226c7a6:     	callq	0x140613c10 <.text+0x612c10>
14226c7ab:     	movq	%rax, 0x158(%rbp)
14226c7b2:     	testq	%rax, %rax
14226c7b5:     	je	0x14226caa2 <.text+0x226baa2>
14226c7bb:     	xorl	%ecx, %ecx
14226c7bd:     	callq	0x1400d9b20 <.text+0xd8b20>
14226c7c2:     	testq	%rax, %rax
14226c7c5:     	je	0x14226cab3 <.text+0x226bab3>
14226c7cb:     	movdqu	(%rax), %xmm0
14226c7cf:     	incq	(%rax)
14226c7d2:     	movq	0x158(%rbp), %rdi
14226c7d9:     	movq	$0x1, (%rdi)
14226c7e0:     	movq	$0x1, 0x8(%rdi)
14226c7e8:     	movups	0xec8a21(%rip), %xmm1   # 0x143135210
14226c7ef:     	movups	%xmm1, 0x10(%rdi)
14226c7f3:     	movups	0xec8a26(%rip), %xmm1   # 0x143135220
14226c7fa:     	movups	%xmm1, 0x20(%rdi)
14226c7fe:     	movdqu	%xmm0, 0x30(%rdi)
14226c803:     	movq	%rdi, 0x100(%rbp)
14226c80a:     	movzbl	0x13c83f8(%rip), %eax   # 0x143634c09
14226c811:     	movl	$0x60, %ecx
14226c816:     	movl	$0x8, %edx
14226c81b:     	callq	0x140613c10 <.text+0x612c10>
14226c820:     	testq	%rax, %rax
14226c823:     	je	0x14226cac1 <.text+0x226bac1>
14226c829:     	movq	$0x1, (%rax)
14226c830:     	movq	$0x1, 0x8(%rax)
14226c838:     	movq	$0x0, 0x10(%rax)
14226c840:     	movq	$0x0, 0x20(%rax)
14226c848:     	movq	$0x0, 0x30(%rax)
14226c850:     	movq	$0x4, 0x38(%rax)
14226c858:     	pxor	%xmm0, %xmm0
14226c85c:     	movdqu	%xmm0, 0x40(%rax)
14226c861:     	movq	$0x4, 0x50(%rax)
14226c869:     	movq	$0x0, 0x58(%rax)
14226c871:     	movdqu	0x68(%rbp), %xmm0
14226c876:     	movdqa	%xmm0, 0x100(%rbp)
14226c87e:     	movq	0x78(%rbp), %rcx
14226c882:     	movq	%rcx, 0x110(%rbp)
14226c889:     	movq	%rdi, 0x118(%rbp)
14226c890:     	movq	%rax, 0x120(%rbp)
14226c897:     	movq	0x13bfb0a(%rip), %rax   # 0x14362c3a8
14226c89e:     	movq	(%rax), %rax
14226c8a1:     	cmpq	$0x3, %rax
14226c8a5:     	jb	0x14226c997 <.text+0x226b997>
14226c8ab:     	movq	0x110(%rbp), %rax
14226c8b2:     	movq	%rax, 0x30(%rbp)
14226c8b6:     	leaq	0x30(%rbp), %rax
14226c8ba:     	movq	%rax, 0x68(%rbp)
14226c8be:     	leaq	-0x14df205(%rip), %rax  # 0x140d8d6c0 <.text+0xd8c6c0>
14226c8c5:     	movq	%rax, 0x70(%rbp)
14226c8c9:     	movb	$0x1, 0x177(%rbp)
14226c8d0:     	leaq	0xdc8e69(%rip), %rcx    # 0x143035740
14226c8d7:     	callq	0x14067dd80 <.text+0x67cd80>
14226c8dc:     	movdqu	(%rax), %xmm0
14226c8e0:     	movl	0x10(%rax), %eax
14226c8e3:     	movq	$0x3, 0xb0(%rbp)
14226c8ee:     	leaq	0xdc8dbb(%rip), %rcx    # 0x1430356b0
14226c8f5:     	movq	%rcx, 0xb8(%rbp)
14226c8fc:     	movq	$0x24, 0xc0(%rbp)
14226c907:     	leaq	0xdc8e12(%rip), %rdx    # 0x143035720
14226c90e:     	movq	%rdx, 0xd0(%rbp)
14226c915:     	movq	$0x2, 0xd8(%rbp)
14226c920:     	movq	%rsi, 0xe0(%rbp)
14226c927:     	movq	$0x1, 0xe8(%rbp)
14226c932:     	movq	$0x0, 0xf0(%rbp)
14226c93d:     	movq	$0x0, 0x80(%rbp)
14226c948:     	movq	%rcx, 0x88(%rbp)
14226c94f:     	movq	$0x24, 0x90(%rbp)
14226c95a:     	movq	$0x0, 0x98(%rbp)
14226c965:     	movdqu	%xmm0, 0xa0(%rbp)
14226c96d:     	movl	$0x1, 0xc8(%rbp)
14226c977:     	movl	%eax, 0xcc(%rbp)
14226c97d:     	movb	$0x1, 0x177(%rbp)
14226c984:     	leaq	0x176(%rbp), %rcx
14226c98b:     	leaq	0x80(%rbp), %rdx
14226c992:     	callq	0x140d4a330 <.text+0xd49330>
14226c997:     	movq	0x2c0(%rbp), %rdx
14226c99e:     	movl	0x1c(%rdx), %eax
14226c9a1:     	movq	0x10(%rdx), %rcx
14226c9a5:     	movl	%eax, (%rcx)
14226c9a7:     	movq	(%rdx), %rcx
14226c9aa:     	movq	0x120(%rbp), %rax
14226c9b1:     	movq	%rax, 0xa0(%rbp)
14226c9b8:     	movdqa	0x100(%rbp), %xmm0
14226c9c0:     	movq	0x110(%rbp), %rax
14226c9c7:     	movq	%rax, 0x90(%rbp)
14226c9ce:     	movq	0x118(%rbp), %rax
14226c9d5:     	movq	%rax, 0x98(%rbp)
14226c9dc:     	movdqa	%xmm0, 0x80(%rbp)
14226c9e4:     	movb	$0x0, 0x177(%rbp)
14226c9eb:     	leaq	0x80(%rbp), %rdx
14226c9f2:     	callq	0x140a844f0 <.text+0xa834f0>
14226c9f7:     	movq	0x148(%rbp), %rsi
14226c9fe:     	movq	0x150(%rbp), %rdi
14226ca05:     	testq	%rdi, %rdi
14226ca08:     	je	0x14226ca38 <.text+0x226ba38>
14226ca0a:     	leaq	0x18(%rsi), %rbx
14226ca0e:     	jmp	0x14226ca19 <.text+0x226ba19>
14226ca10:     	addq	$0x48, %rbx
14226ca14:     	decq	%rdi
14226ca17:     	je	0x14226ca38 <.text+0x226ba38>
14226ca19:     	cmpl	$0x1, -0x18(%rbx)
14226ca1d:     	je	0x14226ca10 <.text+0x226ba10>
14226ca1f:     	movq	-0x8(%rbx), %rdx
14226ca23:     	testq	%rdx, %rdx
14226ca26:     	je	0x14226ca10 <.text+0x226ba10>
14226ca28:     	movq	(%rbx), %rcx
14226ca2b:     	movl	$0x1, %r8d
14226ca31:     	callq	0x140613c20 <.text+0x612c20>
14226ca36:     	jmp	0x14226ca10 <.text+0x226ba10>
14226ca38:     	movq	0x140(%rbp), %rax
14226ca3f:     	testq	%rax, %rax
14226ca42:     	je	0x14226c6a9 <.text+0x226b6a9>
14226ca48:     	shlq	$0x3, %rax
14226ca4c:     	leaq	(%rax,%rax,8), %rdx
14226ca50:     	movl	$0x8, %r8d
14226ca56:     	movq	%rsi, %rcx
14226ca59:     	callq	0x140613c20 <.text+0x612c20>
14226ca5e:     	jmp	0x14226c6a9 <.text+0x226b6a9>
14226ca63:     	movq	$0x0, 0x80(%rbp)
14226ca6e:     	leaq	0xec867b(%rip), %rax    # 0x1431350f0
14226ca75:     	movq	%rax, 0x20(%rsp)
14226ca7a:     	leaq	0x100(%rbp), %rdx
14226ca81:     	leaq	0x80(%rbp), %r9
14226ca88:     	xorl	%ecx, %ecx
14226ca8a:     	movq	%rdi, %r8
14226ca8d:     	callq	0x1428cab41 <.text+0x28c9b41>
14226ca92:     	jmp	0x14226cafc <.text+0x226bafc>
14226ca94:     	leaq	0xdc8c3d(%rip), %rcx    # 0x1430356d8
14226ca9b:     	callq	0x1428d9310 <.text+0x28d8310>
14226caa0:     	jmp	0x14226cafc <.text+0x226bafc>
14226caa2:     	movl	$0x8, %ecx
14226caa7:     	movl	$0x40, %edx
14226caac:     	callq	0x1428d9000 <.text+0x28d8000>
14226cab1:     	jmp	0x14226cafc <.text+0x226bafc>
14226cab3:     	leaq	0xdc7fc6(%rip), %rcx    # 0x143034a80
14226caba:     	callq	0x1428d80d0 <.text+0x28d70d0>
14226cabf:     	jmp	0x14226cafc <.text+0x226bafc>
14226cac1:     	movl	$0x8, %ecx
14226cac6:     	movl	$0x60, %edx
14226cacb:     	callq	0x1428d9000 <.text+0x28d8000>
14226cad0:     	jmp	0x14226cafc <.text+0x226bafc>
14226cad2:     	xorl	%ecx, %ecx
14226cad4:     	jmp	0x14226cae9 <.text+0x226bae9>
14226cad6:     	movl	$0x1, %ecx
14226cadb:     	movq	0x168(%rbp), %rax
14226cae2:     	movq	%rax, 0x160(%rbp)
14226cae9:     	leaq	0xdc8b18(%rip), %r8     # 0x143035608
14226caf0:     	movq	0x160(%rbp), %rdx
14226caf7:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
14226cafc:     	ud2
14226cafe:     	nop
14226cb00:     	movq	%rdx, 0x10(%rsp)
14226cb05:     	pushq	%rbp
14226cb06:     	pushq	%r15
14226cb08:     	pushq	%r14
14226cb0a:     	pushq	%r13
14226cb0c:     	pushq	%r12
14226cb0e:     	pushq	%rsi
14226cb0f:     	pushq	%rdi
14226cb10:     	pushq	%rbx
14226cb11:     	subq	$0xc8, %rsp
14226cb18:     	leaq	0x80(%rdx), %rbp
14226cb1f:     	movaps	%xmm15, 0x20(%rsp)
14226cb25:     	movaps	%xmm14, 0x30(%rsp)
14226cb2b:     	movaps	%xmm13, 0x40(%rsp)
14226cb31:     	movaps	%xmm12, 0x50(%rsp)
14226cb37:     	movaps	%xmm11, 0x60(%rsp)
14226cb3d:     	movaps	%xmm10, 0x70(%rsp)
14226cb43:     	movaps	%xmm9, 0x80(%rsp)
14226cb4c:     	movaps	%xmm8, 0x90(%rsp)
14226cb55:     	movaps	%xmm7, 0xa0(%rsp)
14226cb5d:     	movaps	%xmm6, 0xb0(%rsp)
14226cb65:     	cmpq	$0x0, 0x168(%rbp)
14226cb6d:     	je	0x14226cb88 <.text+0x226bb88>
14226cb6f:     	movl	$0x1, %r8d
14226cb75:     	movq	0x160(%rbp), %rcx
14226cb7c:     	movq	0x168(%rbp), %rdx
14226cb83:     	callq	0x140613c20 <.text+0x612c20>
14226cb88:     	movaps	0xb0(%rsp), %xmm6
14226cb90:     	movaps	0xa0(%rsp), %xmm7
14226cb98:     	movaps	0x90(%rsp), %xmm8
14226cba1:     	movaps	0x80(%rsp), %xmm9
14226cbaa:     	movaps	0x70(%rsp), %xmm10
14226cbb0:     	movaps	0x60(%rsp), %xmm11
14226cbb6:     	movaps	0x50(%rsp), %xmm12
14226cbbc:     	movaps	0x40(%rsp), %xmm13
14226cbc2:     	movaps	0x30(%rsp), %xmm14
14226cbc8:     	movaps	0x20(%rsp), %xmm15
14226cbce:     	addq	$0xc8, %rsp
14226cbd5:     	popq	%rbx
14226cbd6:     	popq	%rdi
14226cbd7:     	popq	%rsi
14226cbd8:     	popq	%r12
14226cbda:     	popq	%r13
14226cbdc:     	popq	%r14
14226cbde:     	popq	%r15
14226cbe0:     	popq	%rbp
14226cbe1:     	retq
14226cbe2:     	nopw	%cs:(%rax,%rax)
14226cbf0:     	movq	%rdx, 0x10(%rsp)
14226cbf5:     	pushq	%rbp
14226cbf6:     	pushq	%r15
14226cbf8:     	pushq	%r14
14226cbfa:     	pushq	%r13
14226cbfc:     	pushq	%r12
14226cbfe:     	pushq	%rsi
14226cbff:     	pushq	%rdi
14226cc00:     	pushq	%rbx
14226cc01:     	subq	$0xc8, %rsp
14226cc08:     	leaq	0x80(%rdx), %rbp
14226cc0f:     	movaps	%xmm15, 0x20(%rsp)
14226cc15:     	movaps	%xmm14, 0x30(%rsp)
14226cc1b:     	movaps	%xmm13, 0x40(%rsp)
14226cc21:     	movaps	%xmm12, 0x50(%rsp)
14226cc27:     	movaps	%xmm11, 0x60(%rsp)
14226cc2d:     	movaps	%xmm10, 0x70(%rsp)
14226cc33:     	movaps	%xmm9, 0x80(%rsp)
14226cc3c:     	movaps	%xmm8, 0x90(%rsp)
14226cc45:     	movaps	%xmm7, 0xa0(%rsp)
14226cc4d:     	movaps	%xmm6, 0xb0(%rsp)
14226cc55:     	movq	0x158(%rbp), %rax
14226cc5c:     	lock
14226cc5d:     	decq	(%rax)
14226cc60:     	jne	0x14226cc6e <.text+0x226bc6e>
14226cc62:     	leaq	0x100(%rbp), %rcx
14226cc69:     	callq	0x140b5cb60 <.text+0xb5bb60>
14226cc6e:     	movaps	0xb0(%rsp), %xmm6
14226cc76:     	movaps	0xa0(%rsp), %xmm7
14226cc7e:     	movaps	0x90(%rsp), %xmm8
14226cc87:     	movaps	0x80(%rsp), %xmm9
14226cc90:     	movaps	0x70(%rsp), %xmm10
14226cc96:     	movaps	0x60(%rsp), %xmm11
14226cc9c:     	movaps	0x50(%rsp), %xmm12
14226cca2:     	movaps	0x40(%rsp), %xmm13
14226cca8:     	movaps	0x30(%rsp), %xmm14
14226ccae:     	movaps	0x20(%rsp), %xmm15
14226ccb4:     	addq	$0xc8, %rsp
14226ccbb:     	popq	%rbx
14226ccbc:     	popq	%rdi
14226ccbd:     	popq	%rsi
14226ccbe:     	popq	%r12
14226ccc0:     	popq	%r13
14226ccc2:     	popq	%r14
14226ccc4:     	popq	%r15
14226ccc6:     	popq	%rbp
14226ccc7:     	retq
14226ccc8:     	nopl	(%rax,%rax)
14226ccd0:     	movq	%rdx, 0x10(%rsp)
14226ccd5:     	pushq	%rbp
14226ccd6:     	pushq	%r15
14226ccd8:     	pushq	%r14
14226ccda:     	pushq	%r13
14226ccdc:     	pushq	%r12
14226ccde:     	pushq	%rsi
14226ccdf:     	pushq	%rdi
14226cce0:     	pushq	%rbx
14226cce1:     	subq	$0xc8, %rsp
14226cce8:     	leaq	0x80(%rdx), %rbp
14226ccef:     	movaps	%xmm15, 0x20(%rsp)
14226ccf5:     	movaps	%xmm14, 0x30(%rsp)
14226ccfb:     	movaps	%xmm13, 0x40(%rsp)
14226cd01:     	movaps	%xmm12, 0x50(%rsp)
14226cd07:     	movaps	%xmm11, 0x60(%rsp)
14226cd0d:     	movaps	%xmm10, 0x70(%rsp)
14226cd13:     	movaps	%xmm9, 0x80(%rsp)
14226cd1c:     	movaps	%xmm8, 0x90(%rsp)
14226cd25:     	movaps	%xmm7, 0xa0(%rsp)
14226cd2d:     	movaps	%xmm6, 0xb0(%rsp)
14226cd35:     	cmpb	$0x0, 0x177(%rbp)
14226cd3c:     	je	0x14226cda4 <.text+0x226bda4>
14226cd3e:     	leaq	0x100(%rbp), %rcx
14226cd45:     	callq	0x140e985f0 <.text+0xe975f0>
14226cd4a:     	movaps	0xb0(%rsp), %xmm6
14226cd52:     	movaps	0xa0(%rsp), %xmm7
14226cd5a:     	movaps	0x90(%rsp), %xmm8
14226cd63:     	movaps	0x80(%rsp), %xmm9
14226cd6c:     	movaps	0x70(%rsp), %xmm10
14226cd72:     	movaps	0x60(%rsp), %xmm11
14226cd78:     	movaps	0x50(%rsp), %xmm12
14226cd7e:     	movaps	0x40(%rsp), %xmm13
14226cd84:     	movaps	0x30(%rsp), %xmm14
14226cd8a:     	movaps	0x20(%rsp), %xmm15
14226cd90:     	addq	$0xc8, %rsp
14226cd97:     	popq	%rbx
14226cd98:     	popq	%rdi
14226cd99:     	popq	%rsi
14226cd9a:     	popq	%r12
14226cd9c:     	popq	%r13
14226cd9e:     	popq	%r14
14226cda0:     	popq	%r15
14226cda2:     	popq	%rbp
14226cda3:     	retq
14226cda4:     	movaps	0xb0(%rsp), %xmm6
14226cdac:     	movaps	0xa0(%rsp), %xmm7
14226cdb4:     	movaps	0x90(%rsp), %xmm8
14226cdbd:     	movaps	0x80(%rsp), %xmm9
14226cdc6:     	movaps	0x70(%rsp), %xmm10
14226cdcc:     	movaps	0x60(%rsp), %xmm11
14226cdd2:     	movaps	0x50(%rsp), %xmm12
14226cdd8:     	movaps	0x40(%rsp), %xmm13
14226cdde:     	movaps	0x30(%rsp), %xmm14
14226cde4:     	movaps	0x20(%rsp), %xmm15
14226cdea:     	addq	$0xc8, %rsp
14226cdf1:     	popq	%rbx
14226cdf2:     	popq	%rdi
14226cdf3:     	popq	%rsi
14226cdf4:     	popq	%r12
14226cdf6:     	popq	%r13
14226cdf8:     	popq	%r14
14226cdfa:     	popq	%r15
14226cdfc:     	popq	%rbp
14226cdfd:     	retq
14226cdfe:     	nop
14226ce00:     	movq	%rdx, 0x10(%rsp)
14226ce05:     	pushq	%rbp
14226ce06:     	pushq	%r15
14226ce08:     	pushq	%r14
14226ce0a:     	pushq	%r13
14226ce0c:     	pushq	%r12
14226ce0e:     	pushq	%rsi
14226ce0f:     	pushq	%rdi
14226ce10:     	pushq	%rbx
14226ce11:     	subq	$0xc8, %rsp
14226ce18:     	leaq	0x80(%rdx), %rbp
14226ce1f:     	movaps	%xmm15, 0x20(%rsp)
14226ce25:     	movaps	%xmm14, 0x30(%rsp)
14226ce2b:     	movaps	%xmm13, 0x40(%rsp)
14226ce31:     	movaps	%xmm12, 0x50(%rsp)
14226ce37:     	movaps	%xmm11, 0x60(%rsp)
14226ce3d:     	movaps	%xmm10, 0x70(%rsp)
14226ce43:     	movaps	%xmm9, 0x80(%rsp)
14226ce4c:     	movaps	%xmm8, 0x90(%rsp)
14226ce55:     	movaps	%xmm7, 0xa0(%rsp)
14226ce5d:     	movaps	%xmm6, 0xb0(%rsp)
14226ce65:     	movl	$0x40, %edx
14226ce6a:     	movl	$0x8, %r8d
14226ce70:     	movq	0x158(%rbp), %rcx
14226ce77:     	callq	0x140613c20 <.text+0x612c20>
14226ce7c:     	movaps	0xb0(%rsp), %xmm6
14226ce84:     	movaps	0xa0(%rsp), %xmm7
14226ce8c:     	movaps	0x90(%rsp), %xmm8
14226ce95:     	movaps	0x80(%rsp), %xmm9
14226ce9e:     	movaps	0x70(%rsp), %xmm10
14226cea4:     	movaps	0x60(%rsp), %xmm11
14226ceaa:     	movaps	0x50(%rsp), %xmm12
14226ceb0:     	movaps	0x40(%rsp), %xmm13
14226ceb6:     	movaps	0x30(%rsp), %xmm14
14226cebc:     	movaps	0x20(%rsp), %xmm15
14226cec2:     	addq	$0xc8, %rsp
14226cec9:     	popq	%rbx
14226ceca:     	popq	%rdi
14226cecb:     	popq	%rsi
14226cecc:     	popq	%r12
14226cece:     	popq	%r13
14226ced0:     	popq	%r14
14226ced2:     	popq	%r15
14226ced4:     	popq	%rbp
14226ced5:     	retq
14226ced6:     	nopw	%cs:(%rax,%rax)
14226cee0:     	movq	%rdx, 0x10(%rsp)
14226cee5:     	pushq	%rbp
14226cee6:     	pushq	%r15
14226cee8:     	pushq	%r14
14226ceea:     	pushq	%r13
14226ceec:     	pushq	%r12
14226ceee:     	pushq	%rsi
14226ceef:     	pushq	%rdi
14226cef0:     	pushq	%rbx
14226cef1:     	subq	$0xc8, %rsp
14226cef8:     	leaq	0x80(%rdx), %rbp
14226ceff:     	movaps	%xmm15, 0x20(%rsp)
14226cf05:     	movaps	%xmm14, 0x30(%rsp)
14226cf0b:     	movaps	%xmm13, 0x40(%rsp)
14226cf11:     	movaps	%xmm12, 0x50(%rsp)
14226cf17:     	movaps	%xmm11, 0x60(%rsp)
14226cf1d:     	movaps	%xmm10, 0x70(%rsp)
14226cf23:     	movaps	%xmm9, 0x80(%rsp)
14226cf2c:     	movaps	%xmm8, 0x90(%rsp)
14226cf35:     	movaps	%xmm7, 0xa0(%rsp)
14226cf3d:     	movaps	%xmm6, 0xb0(%rsp)
14226cf45:     	leaq	0x80(%rbp), %rcx
14226cf4c:     	callq	0x140e98880 <.text+0xe97880>
14226cf51:     	movaps	0xb0(%rsp), %xmm6
14226cf59:     	movaps	0xa0(%rsp), %xmm7
14226cf61:     	movaps	0x90(%rsp), %xmm8
14226cf6a:     	movaps	0x80(%rsp), %xmm9
14226cf73:     	movaps	0x70(%rsp), %xmm10
14226cf79:     	movaps	0x60(%rsp), %xmm11
14226cf7f:     	movaps	0x50(%rsp), %xmm12
14226cf85:     	movaps	0x40(%rsp), %xmm13
14226cf8b:     	movaps	0x30(%rsp), %xmm14
14226cf91:     	movaps	0x20(%rsp), %xmm15
14226cf97:     	addq	$0xc8, %rsp
14226cf9e:     	popq	%rbx
14226cf9f:     	popq	%rdi
14226cfa0:     	popq	%rsi
14226cfa1:     	popq	%r12
14226cfa3:     	popq	%r13
14226cfa5:     	popq	%r14
14226cfa7:     	popq	%r15
14226cfa9:     	popq	%rbp
14226cfaa:     	retq
14226cfab:     	nopl	(%rax,%rax)
14226cfb0:     	movq	%rdx, 0x10(%rsp)
14226cfb5:     	pushq	%rbp
14226cfb6:     	pushq	%r15
14226cfb8:     	pushq	%r14
14226cfba:     	pushq	%r13
14226cfbc:     	pushq	%r12
14226cfbe:     	pushq	%rsi
14226cfbf:     	pushq	%rdi
14226cfc0:     	pushq	%rbx
14226cfc1:     	subq	$0xc8, %rsp
14226cfc8:     	leaq	0x80(%rdx), %rbp
14226cfcf:     	movaps	%xmm15, 0x20(%rsp)
14226cfd5:     	movaps	%xmm14, 0x30(%rsp)
14226cfdb:     	movaps	%xmm13, 0x40(%rsp)
14226cfe1:     	movaps	%xmm12, 0x50(%rsp)
14226cfe7:     	movaps	%xmm11, 0x60(%rsp)
14226cfed:     	movaps	%xmm10, 0x70(%rsp)
14226cff3:     	movaps	%xmm9, 0x80(%rsp)
14226cffc:     	movaps	%xmm8, 0x90(%rsp)
14226d005:     	movaps	%xmm7, 0xa0(%rsp)
14226d00d:     	movaps	%xmm6, 0xb0(%rsp)
14226d015:     	leaq	0x68(%rbp), %rcx
14226d019:     	callq	0x140e98880 <.text+0xe97880>
14226d01e:     	movaps	0xb0(%rsp), %xmm6
14226d026:     	movaps	0xa0(%rsp), %xmm7
14226d02e:     	movaps	0x90(%rsp), %xmm8
14226d037:     	movaps	0x80(%rsp), %xmm9
14226d040:     	movaps	0x70(%rsp), %xmm10
14226d046:     	movaps	0x60(%rsp), %xmm11
14226d04c:     	movaps	0x50(%rsp), %xmm12
14226d052:     	movaps	0x40(%rsp), %xmm13
14226d058:     	movaps	0x30(%rsp), %xmm14
14226d05e:     	movaps	0x20(%rsp), %xmm15
14226d064:     	addq	$0xc8, %rsp
14226d06b:     	popq	%rbx
14226d06c:     	popq	%rdi
14226d06d:     	popq	%rsi
14226d06e:     	popq	%r12
14226d070:     	popq	%r13
14226d072:     	popq	%r14
14226d074:     	popq	%r15
14226d076:     	popq	%rbp
14226d077:     	retq
14226d078:     	nopl	(%rax,%rax)
14226d080:     	movq	%rdx, 0x10(%rsp)
14226d085:     	pushq	%rbp
14226d086:     	pushq	%r15
14226d088:     	pushq	%r14
14226d08a:     	pushq	%r13
14226d08c:     	pushq	%r12
14226d08e:     	pushq	%rsi
14226d08f:     	pushq	%rdi
14226d090:     	pushq	%rbx
14226d091:     	subq	$0xc8, %rsp
14226d098:     	leaq	0x80(%rdx), %rbp
14226d09f:     	movaps	%xmm15, 0x20(%rsp)
14226d0a5:     	movaps	%xmm14, 0x30(%rsp)
14226d0ab:     	movaps	%xmm13, 0x40(%rsp)
14226d0b1:     	movaps	%xmm12, 0x50(%rsp)
14226d0b7:     	movaps	%xmm11, 0x60(%rsp)
14226d0bd:     	movaps	%xmm10, 0x70(%rsp)
14226d0c3:     	movaps	%xmm9, 0x80(%rsp)
14226d0cc:     	movaps	%xmm8, 0x90(%rsp)
14226d0d5:     	movaps	%xmm7, 0xa0(%rsp)
14226d0dd:     	movaps	%xmm6, 0xb0(%rsp)
14226d0e5:     	leaq	0x140(%rbp), %rcx
14226d0ec:     	callq	0x140e98ee0 <.text+0xe97ee0>
14226d0f1:     	movaps	0xb0(%rsp), %xmm6
14226d0f9:     	movaps	0xa0(%rsp), %xmm7
14226d101:     	movaps	0x90(%rsp), %xmm8
14226d10a:     	movaps	0x80(%rsp), %xmm9
14226d113:     	movaps	0x70(%rsp), %xmm10
14226d119:     	movaps	0x60(%rsp), %xmm11
14226d11f:     	movaps	0x50(%rsp), %xmm12
14226d125:     	movaps	0x40(%rsp), %xmm13
14226d12b:     	movaps	0x30(%rsp), %xmm14
14226d131:     	movaps	0x20(%rsp), %xmm15
14226d137:     	addq	$0xc8, %rsp
14226d13e:     	popq	%rbx
14226d13f:     	popq	%rdi
14226d140:     	popq	%rsi
14226d141:     	popq	%r12
14226d143:     	popq	%r13
14226d145:     	popq	%r14
14226d147:     	popq	%r15
14226d149:     	popq	%rbp
14226d14a:     	retq
