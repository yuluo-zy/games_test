
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1405051a0:     	pushq	%rbp
1405051a1:     	pushq	%r14
1405051a3:     	pushq	%rsi
1405051a4:     	pushq	%rdi
1405051a5:     	pushq	%rbx
1405051a6:     	subq	$0xd0, %rsp
1405051ad:     	leaq	0x80(%rsp), %rbp
1405051b5:     	movq	$-0x2, 0x48(%rbp)
1405051bd:     	movq	%r8, %rdi
1405051c0:     	movq	%rdx, %rsi
1405051c3:     	movq	%rcx, %rbx
1405051c6:     	leaq	0x24e8d4e(%rip), %rcx   # 0x1429edf1b
1405051cd:     	movl	$0x9, %edx
1405051d2:     	callq	0x140b9d480 <.text+0xb9c480>
1405051d7:     	movq	%rdx, %r8
1405051da:     	leaq	-0x38(%rbp), %rcx
1405051de:     	movq	%rax, %rdx
1405051e1:     	callq	0x140d7ad40 <.text+0xd79d40>
1405051e6:     	movq	-0x30(%rbp), %r8
1405051ea:     	movq	-0x28(%rbp), %r9
1405051ee:     	leaq	0x28(%rbp), %rcx
1405051f2:     	movq	%rdi, %rdx
1405051f5:     	movq	%r8, 0x40(%rbp)
1405051f9:     	callq	0x14068bfc0 <.text+0x68afc0>
1405051fe:     	leaq	-0x50(%rbp), %rcx
140505202:     	leaq	0x28(%rbp), %rdx
140505206:     	movb	$0x9, %r8b
140505209:     	callq	0x14067e5b0 <.text+0x67d5b0>
14050520e:     	leaq	0x28(%rbp), %rcx
140505212:     	leaq	-0x50(%rbp), %rdx
140505216:     	movb	$0x12, %r8b
140505219:     	callq	0x14067e6d0 <.text+0x67d6d0>
14050521e:     	leaq	0x28(%rbp), %rcx
140505222:     	callq	0x140681e50 <.text+0x680e50>
140505227:     	movq	%rax, %rcx
14050522a:     	callq	0x140683c50 <.text+0x682c50>
14050522f:     	movq	%rax, %rcx
140505232:     	callq	0x140683c80 <.text+0x682c80>
140505237:     	movl	%eax, %r14d
14050523a:     	movq	-0x38(%rbp), %rdx
14050523e:     	testq	%rdx, %rdx
140505241:     	je	0x140505252 <.text+0x504252>
140505243:     	movl	$0x1, %r8d
140505249:     	movq	0x40(%rbp), %rcx
14050524d:     	callq	0x140613c20 <.text+0x612c20>
140505252:     	testb	%r14b, %r14b
140505255:     	je	0x140505274 <.text+0x504274>
140505257:     	movl	0x1c(%rbx), %eax
14050525a:     	movq	0x10(%rbx), %rcx
14050525e:     	movl	%eax, (%rcx)
140505260:     	movq	(%rbx), %rcx
140505263:     	movq	$0x3, -0x38(%rbp)
14050526b:     	leaq	-0x38(%rbp), %rdx
14050526f:     	callq	0x14093fad0 <.text+0x93ead0>
140505274:     	leaq	0x24e8ca9(%rip), %rcx   # 0x1429edf24
14050527b:     	movl	$0x9, %edx
140505280:     	callq	0x140b9d480 <.text+0xb9c480>
140505285:     	movq	%rdx, %r8
140505288:     	leaq	-0x38(%rbp), %rcx
14050528c:     	movq	%rax, %rdx
14050528f:     	callq	0x140d7ad40 <.text+0xd79d40>
140505294:     	movq	-0x30(%rbp), %r8
140505298:     	movq	-0x28(%rbp), %r9
14050529c:     	leaq	0x28(%rbp), %rcx
1405052a0:     	movq	%rdi, %rdx
1405052a3:     	movq	%r8, 0x40(%rbp)
1405052a7:     	callq	0x14068bfc0 <.text+0x68afc0>
1405052ac:     	leaq	0x28(%rbp), %rcx
1405052b0:     	callq	0x140681d50 <.text+0x680d50>
1405052b5:     	movq	%rax, %rcx
1405052b8:     	callq	0x140683c80 <.text+0x682c80>
1405052bd:     	movl	%eax, %ebx
1405052bf:     	movq	-0x38(%rbp), %rdx
1405052c3:     	testq	%rdx, %rdx
1405052c6:     	je	0x1405052d7 <.text+0x5042d7>
1405052c8:     	movl	$0x1, %r8d
1405052ce:     	movq	0x40(%rbp), %rcx
1405052d2:     	callq	0x140613c20 <.text+0x612c20>
1405052d7:     	testb	%bl, %bl
1405052d9:     	je	0x14050530f <.text+0x50430f>
1405052db:     	movq	0x8(%rsi), %rdi
1405052df:     	testq	%rdi, %rdi
1405052e2:     	jne	0x1405052e8 <.text+0x5042e8>
1405052e4:     	movq	0x10(%rsi), %rdi
1405052e8:     	movq	(%rdi), %rax
1405052eb:     	movq	0x10(%rdi), %rsi
1405052ef:     	subq	%rsi, %rax
1405052f2:     	cmpq	$0x7, %rax
1405052f6:     	jbe	0x14050531d <.text+0x50431d>
1405052f8:     	movq	0x8(%rdi), %rax
1405052fc:     	leaq	-0x3b7bf3(%rip), %rcx   # 0x14014d710 <.text+0x14c710>
140505303:     	movq	%rcx, (%rax,%rsi)
140505307:     	addq	$0x8, %rsi
14050530b:     	movq	%rsi, 0x10(%rdi)
14050530f:     	addq	$0xd0, %rsp
140505316:     	popq	%rbx
140505317:     	popq	%rdi
140505318:     	popq	%rsi
140505319:     	popq	%r14
14050531b:     	popq	%rbp
14050531c:     	retq
14050531d:     	movq	$0x1, 0x20(%rsp)
140505326:     	movl	$0x8, %r8d
14050532c:     	movl	$0x1, %r9d
140505332:     	movq	%rdi, %rcx
140505335:     	movq	%rsi, %rdx
140505338:     	callq	0x1428a29e0 <.text+0x28a19e0>
14050533d:     	jmp	0x1405052f8 <.text+0x5042f8>
14050533f:     	nop
140505340:     	movq	%rdx, 0x10(%rsp)
140505345:     	pushq	%rbp
140505346:     	pushq	%r14
140505348:     	pushq	%rsi
140505349:     	pushq	%rdi
14050534a:     	pushq	%rbx
14050534b:     	subq	$0x30, %rsp
14050534f:     	leaq	0x80(%rdx), %rbp
140505356:     	movq	-0x38(%rbp), %rdx
14050535a:     	testq	%rdx, %rdx
14050535d:     	je	0x14050536e <.text+0x50436e>
14050535f:     	movl	$0x1, %r8d
140505365:     	movq	0x40(%rbp), %rcx
140505369:     	callq	0x140613c20 <.text+0x612c20>
14050536e:     	nop
14050536f:     	addq	$0x30, %rsp
140505373:     	popq	%rbx
140505374:     	popq	%rdi
140505375:     	popq	%rsi
140505376:     	popq	%r14
140505378:     	popq	%rbp
140505379:     	retq
14050537a:     	nopw	(%rax,%rax)
140505380:     	movq	%rdx, 0x10(%rsp)
140505385:     	pushq	%rbp
140505386:     	pushq	%r14
140505388:     	pushq	%rsi
140505389:     	pushq	%rdi
14050538a:     	pushq	%rbx
14050538b:     	subq	$0x30, %rsp
14050538f:     	leaq	0x80(%rdx), %rbp
140505396:     	movq	-0x38(%rbp), %rdx
14050539a:     	testq	%rdx, %rdx
14050539d:     	je	0x1405053ae <.text+0x5043ae>
14050539f:     	movl	$0x1, %r8d
1405053a5:     	movq	0x40(%rbp), %rcx
1405053a9:     	callq	0x140613c20 <.text+0x612c20>
1405053ae:     	nop
1405053af:     	addq	$0x30, %rsp
1405053b3:     	popq	%rbx
1405053b4:     	popq	%rdi
1405053b5:     	popq	%rsi
1405053b6:     	popq	%r14
1405053b8:     	popq	%rbp
1405053b9:     	retq
