
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b2cfa0:     	pushq	%rbp
140b2cfa1:     	pushq	%r15
140b2cfa3:     	pushq	%r14
140b2cfa5:     	pushq	%r13
140b2cfa7:     	pushq	%r12
140b2cfa9:     	pushq	%rsi
140b2cfaa:     	pushq	%rdi
140b2cfab:     	pushq	%rbx
140b2cfac:     	subq	$0x2f8, %rsp            # imm = 0x2F8
140b2cfb3:     	leaq	0x80(%rsp), %rbp
140b2cfbb:     	movdqa	%xmm6, 0x260(%rbp)
140b2cfc3:     	movq	$-0x2, 0x258(%rbp)
140b2cfce:     	movq	%r9, 0x248(%rbp)
140b2cfd5:     	movq	%r8, 0x228(%rbp)
140b2cfdc:     	movq	%rdx, 0x240(%rbp)
140b2cfe3:     	movq	%rcx, %rdi
140b2cfe6:     	callq	0x140c16a00 <.text+0xc15a00>
140b2cfeb:     	testb	%al, %al
140b2cfed:     	movq	%rdi, 0x238(%rbp)
140b2cff4:     	je	0x140b2d056 <.text+0xb2c056>
140b2cff6:     	leaq	0x1fbe28b(%rip), %r12   # 0x142aeb288
140b2cffd:     	movl	$0x49, %r15d
140b2d003:     	movl	$0x49, %edx
140b2d008:     	movq	%r12, %rcx
140b2d00b:     	callq	0x140b2ae70 <.text+0xb29e70>
140b2d010:     	cmpq	$0x1, %rax
140b2d014:     	jne	0x140b2d0cd <.text+0xb2c0cd>
140b2d01a:     	movq	%rdx, %r9
140b2d01d:     	testq	%rdx, %rdx
140b2d020:     	je	0x140b2d060 <.text+0xb2c060>
140b2d022:     	cmpq	$0x49, %r9
140b2d026:     	jae	0x140b2d05e <.text+0xb2c05e>
140b2d028:     	leaq	0x1fbe259(%rip), %rax   # 0x142aeb288
140b2d02f:     	cmpb	$-0x41, (%r9,%rax)
140b2d034:     	jg	0x140b2d060 <.text+0xb2c060>
140b2d036:     	leaq	0x1fbe3ab(%rip), %rax   # 0x142aeb3e8
140b2d03d:     	movq	%rax, 0x20(%rsp)
140b2d042:     	leaq	0x1fbe23f(%rip), %rcx   # 0x142aeb288
140b2d049:     	movl	$0x49, %edx
140b2d04e:     	xorl	%r8d, %r8d
140b2d051:     	callq	0x1428d9940 <.text+0x28d8940>
140b2d056:     	xorl	%r14d, %r14d
140b2d059:     	jmp	0x140b2d22e <.text+0xb2c22e>
140b2d05e:     	jne	0x140b2d036 <.text+0xb2c036>
140b2d060:     	leaq	0x1fbe221(%rip), %r12   # 0x142aeb288
140b2d067:     	movq	%r12, %rcx
140b2d06a:     	movq	%r9, %rdx
140b2d06d:     	callq	0x140b2ae70 <.text+0xb29e70>
140b2d072:     	cmpq	$0x1, %rax
140b2d076:     	jne	0x140b2d0cd <.text+0xb2c0cd>
140b2d078:     	movq	%rdx, %r8
140b2d07b:     	addq	$0x2, %r8
140b2d07f:     	je	0x140b2d0ba <.text+0xb2c0ba>
140b2d081:     	cmpq	$0x49, %r8
140b2d085:     	jae	0x140b2d0b8 <.text+0xb2c0b8>
140b2d087:     	leaq	0x1fbe1fa(%rip), %rax   # 0x142aeb288
140b2d08e:     	cmpb	$-0x41, (%r8,%rax)
140b2d093:     	jg	0x140b2d0ba <.text+0xb2c0ba>
140b2d095:     	leaq	0x1fbe364(%rip), %rax   # 0x142aeb400
140b2d09c:     	movq	%rax, 0x20(%rsp)
140b2d0a1:     	leaq	0x1fbe1e0(%rip), %rcx   # 0x142aeb288
140b2d0a8:     	movl	$0x49, %edx
140b2d0ad:     	movl	$0x49, %r9d
140b2d0b3:     	callq	0x1428d9940 <.text+0x28d8940>
140b2d0b8:     	jne	0x140b2d095 <.text+0xb2c095>
140b2d0ba:     	movl	$0x47, %r15d
140b2d0c0:     	subq	%rdx, %r15
140b2d0c3:     	leaq	0x1fbe1be(%rip), %r12   # 0x142aeb288
140b2d0ca:     	addq	%r8, %r12
140b2d0cd:     	leaq	0x1fbe785(%rip), %rax   # 0x142aeb859
140b2d0d4:     	leaq	0x1fbe73d(%rip), %r13   # 0x142aeb818
140b2d0db:     	nopl	(%rax,%rax)
140b2d0e0:     	movsbl	-0x1(%rax), %ecx
140b2d0e4:     	testl	%ecx, %ecx
140b2d0e6:     	js	0x140b2d100 <.text+0xb2c100>
140b2d0e8:     	decq	%rax
140b2d0eb:     	cmpl	$0x5c, %ecx
140b2d0ee:     	jne	0x140b2d159 <.text+0xb2c159>
140b2d0f0:     	jmp	0x140b2d16e <.text+0xb2c16e>
140b2d0f2:     	nopw	%cs:(%rax,%rax)
140b2d100:     	movzbl	-0x2(%rax), %edx
140b2d104:     	cmpb	$-0x40, %dl
140b2d107:     	jge	0x140b2d12e <.text+0xb2c12e>
140b2d109:     	movzbl	-0x3(%rax), %r8d
140b2d10e:     	cmpb	$-0x40, %r8b
140b2d112:     	jge	0x140b2d137 <.text+0xb2c137>
140b2d114:     	movzbl	-0x4(%rax), %r9d
140b2d119:     	addq	$-0x4, %rax
140b2d11d:     	andl	$0x7, %r9d
140b2d121:     	shll	$0x6, %r9d
140b2d125:     	andl	$0x3f, %r8d
140b2d129:     	orl	%r9d, %r8d
140b2d12c:     	jmp	0x140b2d13f <.text+0xb2c13f>
140b2d12e:     	addq	$-0x2, %rax
140b2d132:     	andl	$0x1f, %edx
140b2d135:     	jmp	0x140b2d149 <.text+0xb2c149>
140b2d137:     	addq	$-0x3, %rax
140b2d13b:     	andl	$0xf, %r8d
140b2d13f:     	shll	$0x6, %r8d
140b2d143:     	andl	$0x3f, %edx
140b2d146:     	orl	%r8d, %edx
140b2d149:     	shll	$0x6, %edx
140b2d14c:     	andb	$0x3f, %cl
140b2d14f:     	movzbl	%cl, %ecx
140b2d152:     	orl	%edx, %ecx
140b2d154:     	cmpl	$0x5c, %ecx
140b2d157:     	je	0x140b2d16e <.text+0xb2c16e>
140b2d159:     	cmpl	$0x2f, %ecx
140b2d15c:     	je	0x140b2d16e <.text+0xb2c16e>
140b2d15e:     	cmpq	%r13, %rax
140b2d161:     	jne	0x140b2d0e0 <.text+0xb2c0e0>
140b2d167:     	movl	$0x41, %esi
140b2d16c:     	jmp	0x140b2d1bd <.text+0xb2c1bd>
140b2d16e:     	leaq	0x1fbe6a3(%rip), %r13   # 0x142aeb818
140b2d175:     	subq	%r13, %rax
140b2d178:     	movq	%rax, %r8
140b2d17b:     	incq	%r8
140b2d17e:     	je	0x140b2d1b2 <.text+0xb2c1b2>
140b2d180:     	cmpq	$0x41, %r8
140b2d184:     	jae	0x140b2d1b0 <.text+0xb2c1b0>
140b2d186:     	cmpb	$-0x41, (%r8,%r13)
140b2d18b:     	jg	0x140b2d1b2 <.text+0xb2c1b2>
140b2d18d:     	leaq	0x1fbe234(%rip), %rax   # 0x142aeb3c8
140b2d194:     	movq	%rax, 0x20(%rsp)
140b2d199:     	leaq	0x1fbe678(%rip), %rcx   # 0x142aeb818
140b2d1a0:     	movl	$0x41, %edx
140b2d1a5:     	movl	$0x41, %r9d
140b2d1ab:     	callq	0x1428d9940 <.text+0x28d8940>
140b2d1b0:     	jne	0x140b2d18d <.text+0xb2c18d>
140b2d1b2:     	movl	$0x40, %esi
140b2d1b7:     	subq	%rax, %rsi
140b2d1ba:     	addq	%r8, %r13
140b2d1bd:     	callq	0x140c18130 <.text+0xc17130>
140b2d1c2:     	movq	%rax, %rcx
140b2d1c5:     	movq	(%rax), %rax
140b2d1c8:     	cmpq	$0x1, %rax
140b2d1cc:     	jne	0x140b2d90a <.text+0xb2c90a>
140b2d1d2:     	addq	$0x8, %rcx
140b2d1d6:     	cmpq	$0x0, (%rcx)
140b2d1da:     	jne	0x140b2d92e <.text+0xb2c92e>
140b2d1e0:     	movq	$-0x1, (%rcx)
140b2d1e7:     	movq	%rcx, 0x250(%rbp)
140b2d1ee:     	addq	$0x8, %rcx
140b2d1f2:     	movq	%rsi, 0x20(%rsp)
140b2d1f7:     	movq	$0x0, 0x30(%rsp)
140b2d200:     	movq	$0x1, 0x28(%rsp)
140b2d209:     	movq	%r12, %rdx
140b2d20c:     	movq	%r15, %r8
140b2d20f:     	movq	%r13, %r9
140b2d212:     	callq	0x140c16d40 <.text+0xc15d40>
140b2d217:     	movq	0x250(%rbp), %rcx
140b2d21e:     	incq	(%rcx)
140b2d221:     	movq	%rax, 0x1e8(%rbp)
140b2d228:     	movl	$0x1, %r14d
140b2d22e:     	movq	0x2e0(%rbp), %rax
140b2d235:     	movq	%r14, 0x1e0(%rbp)
140b2d23c:     	movq	(%rax), %rsi
140b2d23f:     	movq	0x8(%rax), %r12
140b2d243:     	movq	0xf8(%rsi), %rdi
140b2d24a:     	movq	0x100(%rsi), %r13
140b2d251:     	movzbl	0x108(%rsi), %ebx
140b2d258:     	callq	0x140d4d740 <.text+0xd4c740>
140b2d25d:     	movq	%rax, %r15
140b2d260:     	testb	$0x1, (%rax)
140b2d263:     	movq	%r14, 0x250(%rbp)
140b2d26a:     	je	0x140b2d27d <.text+0xb2c27d>
140b2d26c:     	movq	%r15, %rcx
140b2d26f:     	addq	$0x8, %rcx
140b2d273:     	movq	0x8(%r15), %rax
140b2d277:     	movq	0x10(%r15), %rdx
140b2d27b:     	jmp	0x140b2d29f <.text+0xb2c29f>
140b2d27d:     	movq	%r14, 0x230(%rbp)
140b2d284:     	callq	0x140d64e00 <.text+0xd63e00>
140b2d289:     	movq	%r15, %rcx
140b2d28c:     	addq	$0x8, %rcx
140b2d290:     	movq	$0x1, (%r15)
140b2d297:     	movq	%rax, 0x8(%r15)
140b2d29b:     	movq	%rdx, 0x10(%r15)
140b2d29f:     	leaq	(%rdi,%r13,4), %r8
140b2d2a3:     	leaq	0xf8(%r12), %r9
140b2d2ab:     	leaq	0x1a0(%r12), %r10
140b2d2b3:     	leaq	0x1(%rax), %r11
140b2d2b7:     	movq	%r11, (%rcx)
140b2d2ba:     	movups	0x1fbde3f(%rip), %xmm0  # 0x142aeb100
140b2d2c1:     	movaps	%xmm0, 0x200(%rbp)
140b2d2c8:     	movdqu	0x1fbde20(%rip), %xmm0  # 0x142aeb0f0
140b2d2d0:     	movdqa	%xmm0, 0x1f0(%rbp)
140b2d2d8:     	movq	%rax, 0x210(%rbp)
140b2d2df:     	movq	%rdx, 0x218(%rbp)
140b2d2e6:     	pxor	%xmm0, %xmm0
140b2d2ea:     	movdqu	%xmm0, 0x1c8(%rbp)
140b2d2f2:     	movdqu	%xmm0, 0x1b8(%rbp)
140b2d2fa:     	movq	%r12, 0x170(%rbp)
140b2d301:     	movq	%r10, 0x178(%rbp)
140b2d308:     	movq	%r9, 0x180(%rbp)
140b2d30f:     	movq	%rsi, 0x188(%rbp)
140b2d316:     	movq	%rdi, 0x190(%rbp)
140b2d31d:     	movq	%r8, 0x198(%rbp)
140b2d324:     	movq	$0x8, 0x1a0(%rbp)
140b2d32f:     	movq	$0x0, 0x1a8(%rbp)
140b2d33a:     	movq	$0x8, 0x1b0(%rbp)
140b2d345:     	movb	%bl, 0x1d8(%rbp)
140b2d34b:     	leaq	-0x40(%rbp), %rcx
140b2d34f:     	leaq	0x170(%rbp), %rdx
140b2d356:     	callq	0x141795a10 <.text+0x1794a10>
140b2d35b:     	movq	-0x40(%rbp), %rdx
140b2d35f:     	cmpq	%rdx, 0x200(%rbp)
140b2d366:     	jb	0x140b2d93a <.text+0xb2c93a>
140b2d36c:     	movups	0x1d0(%rbp), %xmm0
140b2d373:     	movaps	%xmm0, 0x20(%rbp)
140b2d377:     	movups	0x1c0(%rbp), %xmm0
140b2d37e:     	movaps	%xmm0, 0x10(%rbp)
140b2d382:     	movups	0x1b0(%rbp), %xmm0
140b2d389:     	movaps	%xmm0, (%rbp)
140b2d38d:     	movdqu	0x170(%rbp), %xmm0
140b2d395:     	movdqu	0x180(%rbp), %xmm1
140b2d39d:     	movdqu	0x190(%rbp), %xmm2
140b2d3a5:     	movups	0x1a0(%rbp), %xmm3
140b2d3ac:     	movaps	%xmm3, -0x10(%rbp)
140b2d3b0:     	movdqa	%xmm2, -0x20(%rbp)
140b2d3b5:     	movdqa	%xmm1, -0x30(%rbp)
140b2d3ba:     	movdqa	%xmm0, -0x40(%rbp)
140b2d3bf:     	leaq	-0x40(%rbp), %rcx
140b2d3c3:     	leaq	0x1f0(%rbp), %rdx
140b2d3ca:     	callq	0x141475530 <.text+0x1474530>
140b2d3cf:     	movdqa	0x1f0(%rbp), %xmm0
140b2d3d7:     	movdqa	0x200(%rbp), %xmm1
140b2d3df:     	movdqa	0x210(%rbp), %xmm2
140b2d3e7:     	movdqa	%xmm0, 0x170(%rbp)
140b2d3ef:     	movdqa	%xmm1, 0x180(%rbp)
140b2d3f7:     	movdqa	%xmm2, 0x190(%rbp)
140b2d3ff:     	movq	0x238(%rbp), %rdx
140b2d406:     	movl	0x1c(%rdx), %eax
140b2d409:     	movq	0x10(%rdx), %rcx
140b2d40d:     	movl	%eax, (%rcx)
140b2d40f:     	movq	(%rdx), %rax
140b2d412:     	movq	%rax, 0x250(%rbp)
140b2d419:     	cmpb	$0x0, 0x18(%rax)
140b2d41d:     	je	0x140b2d856 <.text+0xb2c856>
140b2d423:     	movq	0x2e8(%rbp), %rax
140b2d42a:     	movl	0x1c(%rax), %ecx
140b2d42d:     	movq	0x10(%rax), %rdx
140b2d431:     	movl	%ecx, (%rdx)
140b2d433:     	movq	(%rax), %r15
140b2d436:     	leaq	-0x40(%rbp), %rcx
140b2d43a:     	movl	$0x170, %r8d            # imm = 0x170
140b2d440:     	movq	%r15, %rdx
140b2d443:     	callq	0x142923c00 <.text+0x2922c00>
140b2d448:     	xorl	%r14d, %r14d
140b2d44b:     	movl	$0x170, %r8d            # imm = 0x170
140b2d451:     	movq	%r15, %rcx
140b2d454:     	xorl	%edx, %edx
140b2d456:     	callq	0x142923c20 <.text+0x2922c20>
140b2d45b:     	movq	0x120(%rbp), %rsi
140b2d462:     	movq	$-0x1, %rdi
140b2d469:     	leaq	0x130(%rbp), %r15
140b2d470:     	leaq	0x1fbe401(%rip), %r12   # 0x142aeb878
140b2d477:     	jmp	0x140b2d4c0 <.text+0xb2c4c0>
140b2d479:     	nopl	(%rax)
140b2d480:     	incq	%r14
140b2d483:     	movq	0x18(%rbx), %rax
140b2d487:     	leaq	(,%r13,4), %rcx
140b2d48f:     	addq	%r13, %rcx
140b2d492:     	movdqa	0x1f0(%rbp), %xmm0
140b2d49a:     	movdqa	0x200(%rbp), %xmm1
140b2d4a2:     	movq	0x210(%rbp), %rdx
140b2d4a9:     	movq	%rdx, 0x20(%rax,%rcx,8)
140b2d4ae:     	movdqu	%xmm1, 0x10(%rax,%rcx,8)
140b2d4b4:     	movdqu	%xmm0, (%rax,%rcx,8)
140b2d4b9:     	incq	%r13
140b2d4bc:     	movq	%r13, 0x20(%rbx)
140b2d4c0:     	cmpq	%rsi, %r14
140b2d4c3:     	jae	0x140b2d58d <.text+0xb2c58d>
140b2d4c9:     	movq	%r14, %rcx
140b2d4cc:     	callq	0x140c111b0 <.text+0xc101b0>
140b2d4d1:     	movl	%eax, %ecx
140b2d4d3:     	cmpl	$0x2c, %eax
140b2d4d6:     	jae	0x140b2d963 <.text+0xb2c963>
140b2d4dc:     	leaq	(%rdx,%rdx,4), %r13
140b2d4e0:     	shlq	$0x3, %r13
140b2d4e4:     	movq	0x250(%rbp), %rax
140b2d4eb:     	movq	0x20(%rax), %rdx
140b2d4ef:     	movl	$0x1, %eax
140b2d4f4:     	addq	-0x40(%rbp,%rcx,8), %r13
140b2d4f9:     	lock
140b2d4fa:     	cmpxchgq	%rdi, 0x8(%rdx)
140b2d4ff:     	jne	0x140b2d955 <.text+0xb2c955>
140b2d505:     	movq	(%rdx), %rax
140b2d508:     	movq	$0x1, 0x8(%rdx)
140b2d510:     	cmpq	$0x1, %rax
140b2d514:     	jne	0x140b2d955 <.text+0xb2c955>
140b2d51a:     	movq	0x250(%rbp), %rax
140b2d521:     	movq	0x20(%rax), %rbx
140b2d525:     	movq	%r15, %rcx
140b2d528:     	movq	%r13, %rdx
140b2d52b:     	callq	0x140d7bd90 <.text+0xd7ad90>
140b2d530:     	movq	0x18(%r13), %rax
140b2d534:     	movq	0x20(%r13), %rcx
140b2d538:     	lock
140b2d539:     	incq	(%rax)
140b2d53c:     	jle	0x140b2d974 <.text+0xb2c974>
140b2d542:     	movq	0x140(%rbp), %rdx
140b2d549:     	movq	%rdx, 0x200(%rbp)
140b2d550:     	movdqu	0x130(%rbp), %xmm0
140b2d558:     	movdqa	%xmm0, 0x1f0(%rbp)
140b2d560:     	movq	%rax, 0x208(%rbp)
140b2d567:     	movq	%rcx, 0x210(%rbp)
140b2d56e:     	movq	0x20(%rbx), %r13
140b2d572:     	cmpq	0x10(%rbx), %r13
140b2d576:     	jne	0x140b2d480 <.text+0xb2c480>
140b2d57c:     	leaq	0x10(%rbx), %rcx
140b2d580:     	movq	%r12, %rdx
140b2d583:     	callq	0x140562ab0 <.text+0x561ab0>
140b2d588:     	jmp	0x140b2d480 <.text+0xb2c480>
140b2d58d:     	leaq	-0x40(%rbp), %rcx
140b2d591:     	callq	0x14087d740 <.text+0x87c740>
140b2d596:     	movq	0x250(%rbp), %rax
140b2d59d:     	movq	0x20(%rax), %rax
140b2d5a1:     	movq	0x18(%rax), %rdi
140b2d5a5:     	movq	0x20(%rax), %rax
140b2d5a9:     	leaq	(%rax,%rax,4), %rax
140b2d5ad:     	leaq	(%rdi,%rax,8), %rax
140b2d5b1:     	movq	%rax, 0x158(%rbp)
140b2d5b8:     	movq	0x240(%rbp), %rax
140b2d5bf:     	movq	(%rax), %rax
140b2d5c2:     	movq	%rax, 0x160(%rbp)
140b2d5c9:     	movq	0x228(%rbp), %rax
140b2d5d0:     	movq	(%rax), %rax
140b2d5d3:     	movq	%rax, 0x168(%rbp)
140b2d5da:     	leaq	-0x40(%rbp), %rbx
140b2d5de:     	pcmpeqd	%xmm6, %xmm6
140b2d5e2:     	jmp	0x140b2d670 <.text+0xb2c670>
140b2d5e7:     	nopw	(%rax,%rax)
140b2d5f0:     	movdqu	(%rcx,%r8), %xmm1
140b2d5f6:     	movdqa	%xmm1, %xmm2
140b2d5fa:     	pcmpeqb	%xmm0, %xmm2
140b2d5fe:     	pmovmskb	%xmm2, %r9d
140b2d603:     	testl	%r9d, %r9d
140b2d606:     	je	0x140b2d640 <.text+0xb2c640>
140b2d608:     	tzcntl	%r9d, %r10d
140b2d60d:     	addq	%r8, %r10
140b2d610:     	andq	%rdx, %r10
140b2d613:     	shlq	$0x3, %r10
140b2d617:     	movq	%rcx, %r11
140b2d61a:     	subq	%r10, %r11
140b2d61d:     	cmpq	-0x8(%r11), %r13
140b2d621:     	je	0x140b2d84d <.text+0xb2c84d>
140b2d627:     	leal	-0x1(%r9), %r10d
140b2d62b:     	andw	%r9w, %r10w
140b2d62f:     	movl	%r10d, %r9d
140b2d632:     	jne	0x140b2d608 <.text+0xb2c608>
140b2d634:     	nopw	%cs:(%rax,%rax)
140b2d640:     	pcmpeqb	%xmm6, %xmm1
140b2d644:     	pmovmskb	%xmm1, %r9d
140b2d649:     	testl	%r9d, %r9d
140b2d64c:     	jne	0x140b2d700 <.text+0xb2c700>
140b2d652:     	addq	%rax, %r8
140b2d655:     	addq	$0x10, %r8
140b2d659:     	addq	$0x10, %rax
140b2d65d:     	andq	%rdx, %r8
140b2d660:     	jmp	0x140b2d5f0 <.text+0xb2c5f0>
140b2d662:     	nopw	%cs:(%rax,%rax)
140b2d670:     	cmpq	0x158(%rbp), %rdi
140b2d677:     	je	0x140b2d856 <.text+0xb2c856>
140b2d67d:     	movq	0x250(%rbp), %rax
140b2d684:     	movq	0x38(%rax), %rsi
140b2d688:     	movdqu	0x8(%rdi), %xmm0
140b2d68d:     	movq	%rsi, -0x40(%rbp)
140b2d691:     	movdqu	%xmm0, -0x38(%rbp)
140b2d696:     	movq	%rbx, %rcx
140b2d699:     	callq	0x14145a710 <.text+0x1459710>
140b2d69e:     	movq	%rax, %r13
140b2d6a1:     	movq	%rax, 0x1f0(%rbp)
140b2d6a8:     	cmpq	$0x0, 0x188(%rbp)
140b2d6b0:     	je	0x140b2d700 <.text+0xb2c700>
140b2d6b2:     	leaq	0x190(%rbp), %rcx
140b2d6b9:     	leaq	0x1f0(%rbp), %rdx
140b2d6c0:     	callq	0x1405b0d70 <.text+0x5afd70>
140b2d6c5:     	movq	0x170(%rbp), %rcx
140b2d6cc:     	movq	0x178(%rbp), %rdx
140b2d6d3:     	movq	%rdx, %r8
140b2d6d6:     	andq	%rax, %r8
140b2d6d9:     	shrq	$0x39, %rax
140b2d6dd:     	movd	%eax, %xmm0
140b2d6e1:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140b2d6e5:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140b2d6ea:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140b2d6ef:     	xorl	%eax, %eax
140b2d6f1:     	jmp	0x140b2d5f0 <.text+0xb2c5f0>
140b2d6f6:     	nopw	%cs:(%rax,%rax)
140b2d700:     	movq	0x18(%rdi), %rax
140b2d704:     	movq	0x20(%rdi), %rcx
140b2d708:     	movq	0x10(%rcx), %rdx
140b2d70c:     	movq	0x28(%rcx), %r10
140b2d710:     	decq	%rdx
140b2d713:     	andq	$-0x10, %rdx
140b2d717:     	addq	%rax, %rdx
140b2d71a:     	addq	$0x10, %rdx
140b2d71e:     	movq	0x248(%rbp), %rax
140b2d725:     	movq	%rax, 0x20(%rsp)
140b2d72a:     	movq	%rbx, %rcx
140b2d72d:     	movq	0x160(%rbp), %r8
140b2d734:     	movq	0x168(%rbp), %r9
140b2d73b:     	callq	*%r10
140b2d73e:     	movq	%rdi, 0x228(%rbp)
140b2d745:     	movq	-0x40(%rbp), %rax
140b2d749:     	movq	%rax, 0x240(%rbp)
140b2d750:     	movq	-0x38(%rbp), %rax
140b2d754:     	movq	%rax, 0x238(%rbp)
140b2d75b:     	movq	-0x30(%rbp), %r15
140b2d75f:     	testq	%r15, %r15
140b2d762:     	je	0x140b2d820 <.text+0xb2c820>
140b2d768:     	shlq	$0x3, %r15
140b2d76c:     	xorl	%ebx, %ebx
140b2d76e:     	nop
140b2d770:     	movq	0x238(%rbp), %rax
140b2d777:     	movq	(%rax,%rbx), %rdi
140b2d77b:     	movq	%rdi, %rax
140b2d77e:     	shrq	$0x20, %rax
140b2d782:     	je	0x140b2d820 <.text+0xb2c820>
140b2d788:     	movq	0x248(%rbp), %rax
140b2d78f:     	movq	0x8(%rax), %r14
140b2d793:     	testq	%r14, %r14
140b2d796:     	jne	0x140b2d7a3 <.text+0xb2c7a3>
140b2d798:     	movq	0x248(%rbp), %rax
140b2d79f:     	movq	0x10(%rax), %r14
140b2d7a3:     	movq	(%r14), %rax
140b2d7a6:     	movq	0x10(%r14), %r12
140b2d7aa:     	subq	%r12, %rax
140b2d7ad:     	cmpq	$0x2f, %rax
140b2d7b1:     	jbe	0x140b2d7fa <.text+0xb2c7fa>
140b2d7b3:     	movq	0x8(%r14), %rax
140b2d7b7:     	leaq	0x7ac252(%rip), %rcx    # 0x1412d9a10 <.text+0x12d8a10>
140b2d7be:     	movq	%rcx, (%rax,%r12)
140b2d7c2:     	leaq	0xa37567(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
140b2d7c9:     	movq	%rcx, 0x8(%rax,%r12)
140b2d7ce:     	movq	%rdi, 0x10(%rax,%r12)
140b2d7d3:     	movq	%r13, 0x18(%rax,%r12)
140b2d7d8:     	movq	%rsi, 0x20(%rax,%r12)
140b2d7dd:     	movb	$0x0, 0x28(%rax,%r12)
140b2d7e3:     	addq	$0x30, %r12
140b2d7e7:     	movq	%r12, 0x10(%r14)
140b2d7eb:     	addq	$0x8, %rbx
140b2d7ef:     	cmpq	%rbx, %r15
140b2d7f2:     	jne	0x140b2d770 <.text+0xb2c770>
140b2d7f8:     	jmp	0x140b2d820 <.text+0xb2c820>
140b2d7fa:     	movq	$0x1, 0x20(%rsp)
140b2d803:     	movl	$0x30, %r8d
140b2d809:     	movl	$0x1, %r9d
140b2d80f:     	movq	%r14, %rcx
140b2d812:     	movq	%r12, %rdx
140b2d815:     	callq	0x1428a9a60 <.text+0x28a8a60>
140b2d81a:     	jmp	0x140b2d7b3 <.text+0xb2c7b3>
140b2d81c:     	nopl	(%rax)
140b2d820:     	movq	0x240(%rbp), %rdx
140b2d827:     	testq	%rdx, %rdx
140b2d82a:     	movq	0x228(%rbp), %rdi
140b2d831:     	leaq	-0x40(%rbp), %rbx
140b2d835:     	je	0x140b2d84d <.text+0xb2c84d>
140b2d837:     	shlq	$0x3, %rdx
140b2d83b:     	movl	$0x8, %r8d
140b2d841:     	movq	0x238(%rbp), %rcx
140b2d848:     	callq	0x140613c20 <.text+0x612c20>
140b2d84d:     	addq	$0x28, %rdi
140b2d851:     	jmp	0x140b2d670 <.text+0xb2c670>
140b2d856:     	movq	0x178(%rbp), %rdx
140b2d85d:     	testq	%rdx, %rdx
140b2d860:     	je	0x140b2d88c <.text+0xb2c88c>
140b2d862:     	leaq	0x17(,%rdx,8), %rax
140b2d86a:     	andq	$-0x10, %rax
140b2d86e:     	addq	%rax, %rdx
140b2d871:     	addq	$0x11, %rdx
140b2d875:     	je	0x140b2d88c <.text+0xb2c88c>
140b2d877:     	movq	0x170(%rbp), %rcx
140b2d87e:     	subq	%rax, %rcx
140b2d881:     	movl	$0x10, %r8d
140b2d887:     	callq	0x140613c20 <.text+0x612c20>
140b2d88c:     	cmpq	$0x0, 0x1e0(%rbp)
140b2d894:     	je	0x140b2d8d9 <.text+0xb2c8d9>
140b2d896:     	callq	0x140c18130 <.text+0xc17130>
140b2d89b:     	movq	%rax, %rcx
140b2d89e:     	movq	(%rax), %rax
140b2d8a1:     	cmpq	$0x1, %rax
140b2d8a5:     	jne	0x140b2d8f4 <.text+0xb2c8f4>
140b2d8a7:     	addq	$0x8, %rcx
140b2d8ab:     	cmpq	$0x0, (%rcx)
140b2d8af:     	jne	0x140b2d92e <.text+0xb2c92e>
140b2d8b1:     	movq	$-0x1, (%rcx)
140b2d8b8:     	movq	%rcx, 0x248(%rbp)
140b2d8bf:     	addq	$0x8, %rcx
140b2d8c3:     	movq	0x1e8(%rbp), %rdx
140b2d8ca:     	callq	0x140c170f0 <.text+0xc160f0>
140b2d8cf:     	movq	0x248(%rbp), %rax
140b2d8d6:     	incq	(%rax)
140b2d8d9:     	movaps	0x260(%rbp), %xmm6
140b2d8e0:     	addq	$0x2f8, %rsp            # imm = 0x2F8
140b2d8e7:     	popq	%rbx
140b2d8e8:     	popq	%rdi
140b2d8e9:     	popq	%rsi
140b2d8ea:     	popq	%r12
140b2d8ec:     	popq	%r13
140b2d8ee:     	popq	%r14
140b2d8f0:     	popq	%r15
140b2d8f2:     	popq	%rbp
140b2d8f3:     	retq
140b2d8f4:     	testq	%rax, %rax
140b2d8f7:     	jne	0x140b2d922 <.text+0xb2c922>
140b2d8f9:     	xorl	%edx, %edx
140b2d8fb:     	callq	0x1428f87d0 <.text+0x28f77d0>
140b2d900:     	movq	%rax, %rcx
140b2d903:     	testq	%rax, %rax
140b2d906:     	jne	0x140b2d8ab <.text+0xb2c8ab>
140b2d908:     	jmp	0x140b2d922 <.text+0xb2c922>
140b2d90a:     	testq	%rax, %rax
140b2d90d:     	jne	0x140b2d922 <.text+0xb2c922>
140b2d90f:     	xorl	%edx, %edx
140b2d911:     	callq	0x1428f87d0 <.text+0x28f77d0>
140b2d916:     	movq	%rax, %rcx
140b2d919:     	testq	%rax, %rax
140b2d91c:     	jne	0x140b2d1d6 <.text+0xb2c1d6>
140b2d922:     	leaq	0x210d177(%rip), %rcx   # 0x142c3aaa0
140b2d929:     	callq	0x1428d80d0 <.text+0x28d70d0>
140b2d92e:     	leaq	0x210d23b(%rip), %rcx   # 0x142c3ab70
140b2d935:     	callq	0x1428d9250 <.text+0x28d8250>
140b2d93a:     	leaq	0x210(%rbp), %r8
140b2d941:     	leaq	0x1f0(%rbp), %rcx
140b2d948:     	movb	$0x1, %r9b
140b2d94b:     	callq	0x1428aa7a0 <.text+0x28a97a0>
140b2d950:     	jmp	0x140b2d36c <.text+0xb2c36c>
140b2d955:     	leaq	0x1fbdf04(%rip), %rcx   # 0x142aeb860
140b2d95c:     	callq	0x1428d9310 <.text+0x28d8310>
140b2d961:     	jmp	0x140b2d974 <.text+0xb2c974>
140b2d963:     	leaq	0x213640e(%rip), %r8    # 0x142c63d78
140b2d96a:     	movl	$0x2c, %edx
140b2d96f:     	callq	0x1428d9518 <.text+0x28d8518>
140b2d974:     	ud2
