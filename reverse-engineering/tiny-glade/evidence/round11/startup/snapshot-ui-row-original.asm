
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140143040:     	pushq	%rbp
140143041:     	pushq	%r15
140143043:     	pushq	%r14
140143045:     	pushq	%r13
140143047:     	pushq	%r12
140143049:     	pushq	%rsi
14014304a:     	pushq	%rdi
14014304b:     	pushq	%rbx
14014304c:     	subq	$0x398, %rsp            # imm = 0x398
140143053:     	leaq	0x80(%rsp), %rbp
14014305b:     	movq	$-0x2, 0x310(%rbp)
140143066:     	movq	%rdx, %rdi
140143069:     	movq	(%rcx), %rsi
14014306c:     	movq	0x8(%rcx), %r12
140143070:     	movq	$0x4, -0x50(%rbp)
140143078:     	movb	$0x0, 0x2ff(%rbp)
14014307f:     	leaq	0x280e14e(%rip), %rdx   # 0x1429511d4
140143086:     	leaq	0x40(%rbp), %rcx
14014308a:     	movl	$0xc, %r8d
140143090:     	callq	0x1400568d0 <.text+0x558d0>
140143095:     	movups	0xa0(%rbp), %xmm0
14014309c:     	movaps	%xmm0, 0x270(%rbp)
1401430a3:     	movups	0x90(%rbp), %xmm0
1401430aa:     	movaps	%xmm0, 0x260(%rbp)
1401430b1:     	movups	0x80(%rbp), %xmm0
1401430b8:     	movaps	%xmm0, 0x250(%rbp)
1401430bf:     	movups	0x40(%rbp), %xmm0
1401430c3:     	movups	0x50(%rbp), %xmm1
1401430c7:     	movups	0x60(%rbp), %xmm2
1401430cb:     	movups	0x70(%rbp), %xmm3
1401430cf:     	movaps	%xmm3, 0x240(%rbp)
1401430d6:     	movaps	%xmm2, 0x230(%rbp)
1401430dd:     	movaps	%xmm1, 0x220(%rbp)
1401430e4:     	movaps	%xmm0, 0x210(%rbp)
1401430eb:     	movb	$0x1, 0x2ff(%rbp)
1401430f2:     	leaq	0x40(%rbp), %rcx
1401430f6:     	leaq	-0x50(%rbp), %rdx
1401430fa:     	leaq	0x210(%rbp), %r8
140143101:     	callq	0x140c46bd0 <.text+0xc45bd0>
140143106:     	leaq	0x280(%rbp), %rcx
14014310d:     	leaq	0x40(%rbp), %rdx
140143111:     	movq	%rdi, %r8
140143114:     	callq	0x140c46d20 <.text+0xc45d20>
140143119:     	cmpb	$0x0, 0x2d4(%rbp)
140143120:     	movabsq	$-0x8000000000000000, %r13 # imm = 0x8000000000000000
14014312a:     	je	0x14014348e <.text+0x14248e>
140143130:     	movq	0x290(%rbp), %rax
140143137:     	lock
140143138:     	decq	(%rax)
14014313b:     	jne	0x140143149 <.text+0x142149>
14014313d:     	leaq	0x290(%rbp), %rcx
140143144:     	callq	0x140c41170 <.text+0xc40170>
140143149:     	leaq	0x280e178(%rip), %rdx   # 0x1429512c8
140143150:     	leaq	0x40(%rbp), %rbx
140143154:     	movl	$0xa, %r8d
14014315a:     	movq	%rbx, %rcx
14014315d:     	callq	0x140d61b00 <.text+0xd60b00>
140143162:     	movb	$0x1, -0x50(%rbp)
140143166:     	movq	%rbx, %rcx
140143169:     	callq	0x140d76ed0 <.text+0xd75ed0>
14014316e:     	movq	%rdx, %r8
140143171:     	leaq	-0x50(%rbp), %rcx
140143175:     	movq	%rax, %rdx
140143178:     	callq	0x140d5a150 <.text+0xd59150>
14014317d:     	movq	%rax, %rbx
140143180:     	movq	0x40(%rbp), %rdx
140143184:     	testq	%rdx, %rdx
140143187:     	je	0x140143198 <.text+0x142198>
140143189:     	movq	0x48(%rbp), %rcx
14014318d:     	movl	$0x1, %r8d
140143193:     	callq	0x140613c20 <.text+0x612c20>
140143198:     	movl	%ebx, %eax
14014319a:     	andl	$0x3, %eax
14014319d:     	cmpl	$0x1, %eax
1401431a0:     	jne	0x14014320d <.text+0x14220d>
1401431a2:     	movq	%rbx, %rax
1401431a5:     	decq	%rax
1401431a8:     	movq	%rax, 0x2e8(%rbp)
1401431af:     	movq	-0x1(%rbx), %rax
1401431b3:     	movq	%rax, 0x300(%rbp)
1401431ba:     	movq	0x7(%rbx), %rax
1401431be:     	movq	%rax, 0x2f0(%rbp)
1401431c5:     	movq	(%rax), %rax
1401431c8:     	testq	%rax, %rax
1401431cb:     	je	0x1401431d6 <.text+0x1421d6>
1401431cd:     	movq	0x300(%rbp), %rcx
1401431d4:     	callq	*%rax
1401431d6:     	movq	0x300(%rbp), %rcx
1401431dd:     	movq	0x2f0(%rbp), %rax
1401431e4:     	movq	0x8(%rax), %rdx
1401431e8:     	testq	%rdx, %rdx
1401431eb:     	je	0x1401431f6 <.text+0x1421f6>
1401431ed:     	movq	0x10(%rax), %r8
1401431f1:     	callq	0x140613c20 <.text+0x612c20>
1401431f6:     	movl	$0x18, %edx
1401431fb:     	movl	$0x8, %r8d
140143201:     	movq	0x2e8(%rbp), %rcx
140143208:     	callq	0x140613c20 <.text+0x612c20>
14014320d:     	movq	$0x0, 0x40(%rbp)
140143215:     	movq	$0x8, 0x48(%rbp)
14014321d:     	movq	$0x0, 0x50(%rbp)
140143225:     	movq	%r13, 0x58(%rbp)
140143229:     	movq	%r13, 0x78(%rbp)
14014322d:     	movq	%r13, 0x90(%rbp)
140143234:     	movl	$0xd, 0xa8(%rbp)
14014323e:     	leaq	0x280e073(%rip), %rax   # 0x1429512b8
140143245:     	movq	%rax, 0x20(%rsp)
14014324a:     	movq	$0x1, 0x28(%rsp)
140143253:     	leaq	0x280e056(%rip), %r8    # 0x1429512b0
14014325a:     	leaq	-0x50(%rbp), %rcx
14014325e:     	leaq	0x40(%rbp), %rdx
140143262:     	movl	$0x4, %r9d
140143268:     	callq	0x140b7fad0 <.text+0xb7ead0>
14014326d:     	leaq	0x280e054(%rip), %rdx   # 0x1429512c8
140143274:     	leaq	0x210(%rbp), %rcx
14014327b:     	movl	$0xa, %r8d
140143281:     	callq	0x140d61b00 <.text+0xd60b00>
140143286:     	movb	$0x1, 0x30f(%rbp)
14014328d:     	leaq	0x210(%rbp), %rcx
140143294:     	callq	0x140d76ed0 <.text+0xd75ed0>
140143299:     	movq	%rdx, %r8
14014329c:     	movb	$0x1, 0x30f(%rbp)
1401432a3:     	leaq	0x40(%rbp), %rcx
1401432a7:     	movq	%rax, %rdx
1401432aa:     	callq	0x140d6af30 <.text+0xd69f30>
1401432af:     	xorl	%eax, %eax
1401432b1:     	cmpq	0x40(%rbp), %rax
1401432b5:     	jo	0x140143adb <.text+0x142adb>
1401432bb:     	movups	0x40(%rbp), %xmm0
1401432bf:     	movups	0x50(%rbp), %xmm1
1401432c3:     	movaps	%xmm1, 0x290(%rbp)
1401432ca:     	movaps	%xmm0, 0x280(%rbp)
1401432d1:     	movb	$0x0, 0x30f(%rbp)
1401432d8:     	leaq	0x40(%rbp), %rcx
1401432dc:     	leaq	-0x50(%rbp), %rdx
1401432e0:     	leaq	0x280(%rbp), %rbx
1401432e7:     	movq	%rbx, %r8
1401432ea:     	callq	0x1400d6c50 <.text+0xd5c50>
1401432ef:     	leaq	0x1f0(%rbp), %rcx
1401432f6:     	leaq	0x40(%rbp), %rdx
1401432fa:     	callq	0x140b7fdb0 <.text+0xb7edb0>
1401432ff:     	xorl	%eax, %eax
140143301:     	cmpq	0x1f0(%rbp), %rax
140143308:     	jo	0x1401435bc <.text+0x1425bc>
14014330e:     	movups	0x1f0(%rbp), %xmm0
140143315:     	movups	0x200(%rbp), %xmm1
14014331c:     	movaps	%xmm1, -0x40(%rbp)
140143320:     	movaps	%xmm0, -0x50(%rbp)
140143324:     	movzbl	0x34f18de(%rip), %eax   # 0x143634c09
14014332b:     	movl	$0x80, %ecx
140143330:     	movl	$0x1, %edx
140143335:     	callq	0x140613c10 <.text+0x612c10>
14014333a:     	testq	%rax, %rax
14014333d:     	je	0x140143b4e <.text+0x142b4e>
140143343:     	movq	$0x80, 0x280(%rbp)
14014334e:     	movq	%rax, 0x288(%rbp)
140143355:     	movq	$0x0, 0x290(%rbp)
140143360:     	movq	%rbx, 0x40(%rbp)
140143364:     	leaq	0x280dad5(%rip), %rax   # 0x142950e40
14014336b:     	movq	%rax, 0x48(%rbp)
14014336f:     	movq	$0x2, 0x50(%rbp)
140143377:     	movq	$0x0, 0x58(%rbp)
14014337f:     	movb	$0x0, 0x60(%rbp)
140143383:     	leaq	0x40(%rbp), %rcx
140143387:     	movq	%rsi, %rdx
14014338a:     	callq	0x1404b6de0 <.text+0x4b5de0>
14014338f:     	testq	%rax, %rax
140143392:     	jne	0x140143b66 <.text+0x142b66>
140143398:     	movq	0x280(%rbp), %rcx
14014339f:     	movq	0x288(%rbp), %rax
1401433a6:     	movq	%rax, 0x300(%rbp)
1401433ad:     	movq	%rcx, 0x2f0(%rbp)
1401433b4:     	movq	%rcx, %rax
1401433b7:     	negq	%rax
1401433ba:     	jo	0x140143b8b <.text+0x142b8b>
1401433c0:     	movq	0x290(%rbp), %r15
1401433c7:     	leaq	-0x50(%rbp), %r14
1401433cb:     	movq	%r14, %rcx
1401433ce:     	callq	0x140d76ed0 <.text+0xd75ed0>
1401433d3:     	movq	%rax, %rcx
1401433d6:     	movq	0x300(%rbp), %r8
1401433dd:     	movq	%r15, %r9
1401433e0:     	callq	0x140d59430 <.text+0xd58430>
1401433e5:     	movq	%rax, %r15
1401433e8:     	movq	0x2f0(%rbp), %rdx
1401433ef:     	testq	%rdx, %rdx
1401433f2:     	je	0x140143406 <.text+0x142406>
1401433f4:     	movl	$0x1, %r8d
1401433fa:     	movq	0x300(%rbp), %rcx
140143401:     	callq	0x140613c20 <.text+0x612c20>
140143406:     	testq	%r15, %r15
140143409:     	je	0x1401434d5 <.text+0x1424d5>
14014340f:     	movl	%r15d, %eax
140143412:     	andl	$0x3, %eax
140143415:     	cmpl	$0x1, %eax
140143418:     	jne	0x1401435a4 <.text+0x1425a4>
14014341e:     	movq	%r15, %rax
140143421:     	decq	%rax
140143424:     	movq	%rax, 0x2e8(%rbp)
14014342b:     	movq	-0x1(%r15), %rax
14014342f:     	movq	%rax, 0x300(%rbp)
140143436:     	movq	0x7(%r15), %rax
14014343a:     	movq	%rax, 0x2f0(%rbp)
140143441:     	movq	(%rax), %rax
140143444:     	testq	%rax, %rax
140143447:     	je	0x140143452 <.text+0x142452>
140143449:     	movq	0x300(%rbp), %rcx
140143450:     	callq	*%rax
140143452:     	movq	0x300(%rbp), %rcx
140143459:     	movq	0x2f0(%rbp), %rax
140143460:     	movq	0x8(%rax), %rdx
140143464:     	testq	%rdx, %rdx
140143467:     	je	0x140143472 <.text+0x142472>
140143469:     	movq	0x10(%rax), %r8
14014346d:     	callq	0x140613c20 <.text+0x612c20>
140143472:     	movl	$0x18, %edx
140143477:     	movl	$0x8, %r8d
14014347d:     	movq	0x2e8(%rbp), %rcx
140143484:     	callq	0x140613c20 <.text+0x612c20>
140143489:     	jmp	0x1401435a4 <.text+0x1425a4>
14014348e:     	movb	$0x0, 0x40(%rbp)
140143492:     	cmpb	$0x0, 0x2d3(%rbp)
140143499:     	je	0x1401434b3 <.text+0x1424b3>
14014349b:     	leaq	0x290(%rbp), %rcx
1401434a2:     	leaq	0x40(%rbp), %rdx
1401434a6:     	callq	0x140042870 <.text+0x41870>
1401434ab:     	testb	%al, %al
1401434ad:     	jne	0x140143130 <.text+0x142130>
1401434b3:     	movq	0x290(%rbp), %rax
1401434ba:     	lock
1401434bb:     	decq	(%rax)
1401434be:     	jne	0x1401435da <.text+0x1425da>
1401434c4:     	leaq	0x290(%rbp), %rcx
1401434cb:     	callq	0x140c41170 <.text+0xc40170>
1401434d0:     	jmp	0x1401435da <.text+0x1425da>
1401434d5:     	movq	0x34e8ecc(%rip), %rax   # 0x14362c3a8
1401434dc:     	movq	(%rax), %rax
1401434df:     	cmpq	$0x3, %rax
1401434e3:     	jb	0x1401435a4 <.text+0x1425a4>
1401434e9:     	movq	%r14, 0x280(%rbp)
1401434f0:     	leaq	0xc1e5e9(%rip), %rax    # 0x140d61ae0 <.text+0xd60ae0>
1401434f7:     	movq	%rax, 0x288(%rbp)
1401434fe:     	leaq	0x280dfeb(%rip), %rcx   # 0x1429514f0
140143505:     	callq	0x14067dd80 <.text+0x67cd80>
14014350a:     	movups	(%rax), %xmm0
14014350d:     	movl	0x10(%rax), %eax
140143510:     	movq	$0x3, 0x70(%rbp)
140143518:     	leaq	0x280dd69(%rip), %rcx   # 0x142951288
14014351f:     	movq	%rcx, 0x78(%rbp)
140143523:     	movq	$0x1a, 0x80(%rbp)
14014352e:     	leaq	0x280dd2b(%rip), %rdx   # 0x142951260
140143535:     	movq	%rdx, 0x90(%rbp)
14014353c:     	movq	$0x1, 0x98(%rbp)
140143547:     	movq	%rbx, 0xa0(%rbp)
14014354e:     	movq	$0x1, 0xa8(%rbp)
140143559:     	movq	$0x0, 0xb0(%rbp)
140143564:     	movq	$0x0, 0x40(%rbp)
14014356c:     	movq	%rcx, 0x48(%rbp)
140143570:     	movq	$0x1a, 0x50(%rbp)
140143578:     	movq	$0x0, 0x58(%rbp)
140143580:     	movups	%xmm0, 0x60(%rbp)
140143584:     	movl	$0x1, 0x88(%rbp)
14014358e:     	movl	%eax, 0x8c(%rbp)
140143594:     	leaq	0x2e7(%rbp), %rcx
14014359b:     	leaq	0x40(%rbp), %rdx
14014359f:     	callq	0x140d4a330 <.text+0xd49330>
1401435a4:     	movq	-0x50(%rbp), %rdx
1401435a8:     	testq	%rdx, %rdx
1401435ab:     	je	0x1401435bc <.text+0x1425bc>
1401435ad:     	movq	-0x48(%rbp), %rcx
1401435b1:     	movl	$0x1, %r8d
1401435b7:     	callq	0x140613c20 <.text+0x612c20>
1401435bc:     	movq	0x210(%rbp), %rdx
1401435c3:     	testq	%rdx, %rdx
1401435c6:     	je	0x1401435da <.text+0x1425da>
1401435c8:     	movq	0x218(%rbp), %rcx
1401435cf:     	movl	$0x1, %r8d
1401435d5:     	callq	0x140613c20 <.text+0x612c20>
1401435da:     	movq	$0x4, -0x50(%rbp)
1401435e2:     	movb	$0x0, 0x2fe(%rbp)
1401435e9:     	leaq	0x280dcb2(%rip), %rdx   # 0x1429512a2
1401435f0:     	leaq	0x40(%rbp), %rcx
1401435f4:     	movl	$0xe, %r8d
1401435fa:     	callq	0x1400568d0 <.text+0x558d0>
1401435ff:     	movups	0xa0(%rbp), %xmm0
140143606:     	movaps	%xmm0, 0x270(%rbp)
14014360d:     	movups	0x90(%rbp), %xmm0
140143614:     	movaps	%xmm0, 0x260(%rbp)
14014361b:     	movups	0x80(%rbp), %xmm0
140143622:     	movaps	%xmm0, 0x250(%rbp)
140143629:     	movups	0x40(%rbp), %xmm0
14014362d:     	movups	0x50(%rbp), %xmm1
140143631:     	movups	0x60(%rbp), %xmm2
140143635:     	movups	0x70(%rbp), %xmm3
140143639:     	movaps	%xmm3, 0x240(%rbp)
140143640:     	movaps	%xmm2, 0x230(%rbp)
140143647:     	movaps	%xmm1, 0x220(%rbp)
14014364e:     	movaps	%xmm0, 0x210(%rbp)
140143655:     	movb	$0x1, 0x2fe(%rbp)
14014365c:     	leaq	0x40(%rbp), %rcx
140143660:     	leaq	-0x50(%rbp), %rdx
140143664:     	leaq	0x210(%rbp), %r8
14014366b:     	callq	0x140c46bd0 <.text+0xc45bd0>
140143670:     	leaq	0x280(%rbp), %rcx
140143677:     	leaq	0x40(%rbp), %rdx
14014367b:     	movq	%rdi, %r8
14014367e:     	callq	0x140c46d20 <.text+0xc45d20>
140143683:     	cmpb	$0x0, 0x2d4(%rbp)
14014368a:     	je	0x140143945 <.text+0x142945>
140143690:     	movq	0x290(%rbp), %rax
140143697:     	lock
140143698:     	decq	(%rax)
14014369b:     	jne	0x1401436a9 <.text+0x1426a9>
14014369d:     	leaq	0x290(%rbp), %rcx
1401436a4:     	callq	0x140c41170 <.text+0xc40170>
1401436a9:     	leaq	0x280dc18(%rip), %rdx   # 0x1429512c8
1401436b0:     	leaq	0x40(%rbp), %rdi
1401436b4:     	movl	$0xa, %r8d
1401436ba:     	movq	%rdi, %rcx
1401436bd:     	callq	0x140d61b00 <.text+0xd60b00>
1401436c2:     	movb	$0x1, -0x50(%rbp)
1401436c6:     	movq	%rdi, %rcx
1401436c9:     	callq	0x140d76ed0 <.text+0xd75ed0>
1401436ce:     	movq	%rdx, %r8
1401436d1:     	leaq	-0x50(%rbp), %rcx
1401436d5:     	movq	%rax, %rdx
1401436d8:     	callq	0x140d5a150 <.text+0xd59150>
1401436dd:     	movq	%rax, %rdi
1401436e0:     	movq	0x40(%rbp), %rdx
1401436e4:     	testq	%rdx, %rdx
1401436e7:     	je	0x1401436f8 <.text+0x1426f8>
1401436e9:     	movq	0x48(%rbp), %rcx
1401436ed:     	movl	$0x1, %r8d
1401436f3:     	callq	0x140613c20 <.text+0x612c20>
1401436f8:     	movl	%edi, %eax
1401436fa:     	andl	$0x3, %eax
1401436fd:     	cmpl	$0x1, %eax
140143700:     	jne	0x14014376d <.text+0x14276d>
140143702:     	movq	%rdi, %rax
140143705:     	decq	%rax
140143708:     	movq	%rax, 0x2e8(%rbp)
14014370f:     	movq	-0x1(%rdi), %rax
140143713:     	movq	%rax, 0x300(%rbp)
14014371a:     	movq	0x7(%rdi), %rax
14014371e:     	movq	%rax, 0x2f0(%rbp)
140143725:     	movq	(%rax), %rax
140143728:     	testq	%rax, %rax
14014372b:     	je	0x140143736 <.text+0x142736>
14014372d:     	movq	0x300(%rbp), %rcx
140143734:     	callq	*%rax
140143736:     	movq	0x300(%rbp), %rcx
14014373d:     	movq	0x2f0(%rbp), %rax
140143744:     	movq	0x8(%rax), %rdx
140143748:     	testq	%rdx, %rdx
14014374b:     	je	0x140143756 <.text+0x142756>
14014374d:     	movq	0x10(%rax), %r8
140143751:     	callq	0x140613c20 <.text+0x612c20>
140143756:     	movl	$0x18, %edx
14014375b:     	movl	$0x8, %r8d
140143761:     	movq	0x2e8(%rbp), %rcx
140143768:     	callq	0x140613c20 <.text+0x612c20>
14014376d:     	movq	$0x0, 0x40(%rbp)
140143775:     	movq	$0x8, 0x48(%rbp)
14014377d:     	movq	$0x0, 0x50(%rbp)
140143785:     	movq	%r13, 0x58(%rbp)
140143789:     	movq	%r13, 0x78(%rbp)
14014378d:     	movq	%r13, 0x90(%rbp)
140143794:     	movl	$0xd, 0xa8(%rbp)
14014379e:     	leaq	0x280db13(%rip), %rax   # 0x1429512b8
1401437a5:     	movq	%rax, 0x20(%rsp)
1401437aa:     	movq	$0x1, 0x28(%rsp)
1401437b3:     	leaq	0x280daf6(%rip), %r8    # 0x1429512b0
1401437ba:     	leaq	-0x50(%rbp), %rcx
1401437be:     	leaq	0x40(%rbp), %rdx
1401437c2:     	movl	$0x4, %r9d
1401437c8:     	callq	0x140b7fad0 <.text+0xb7ead0>
1401437cd:     	leaq	0x280daf4(%rip), %rdx   # 0x1429512c8
1401437d4:     	leaq	0x210(%rbp), %rcx
1401437db:     	movl	$0xa, %r8d
1401437e1:     	callq	0x140d61b00 <.text+0xd60b00>
1401437e6:     	movb	$0x1, 0x30e(%rbp)
1401437ed:     	leaq	0x210(%rbp), %rcx
1401437f4:     	callq	0x140d76ed0 <.text+0xd75ed0>
1401437f9:     	movq	%rdx, %r8
1401437fc:     	movb	$0x1, 0x30e(%rbp)
140143803:     	leaq	0x40(%rbp), %rcx
140143807:     	movq	%rax, %rdx
14014380a:     	callq	0x140d6af30 <.text+0xd69f30>
14014380f:     	xorl	%eax, %eax
140143811:     	cmpq	0x40(%rbp), %rax
140143815:     	jo	0x140143b16 <.text+0x142b16>
14014381b:     	movups	0x40(%rbp), %xmm0
14014381f:     	movups	0x50(%rbp), %xmm1
140143823:     	movaps	%xmm1, 0x290(%rbp)
14014382a:     	movaps	%xmm0, 0x280(%rbp)
140143831:     	movb	$0x0, 0x30e(%rbp)
140143838:     	leaq	0x40(%rbp), %rcx
14014383c:     	leaq	-0x50(%rbp), %rdx
140143840:     	leaq	0x280(%rbp), %r8
140143847:     	callq	0x1400d6c50 <.text+0xd5c50>
14014384c:     	leaq	0x1f0(%rbp), %rcx
140143853:     	leaq	0x40(%rbp), %rdx
140143857:     	callq	0x140b7fcd0 <.text+0xb7ecd0>
14014385c:     	xorl	%eax, %eax
14014385e:     	cmpq	0x1f0(%rbp), %rax
140143865:     	jo	0x140143aa8 <.text+0x142aa8>
14014386b:     	movups	0x1f0(%rbp), %xmm0
140143872:     	movups	0x200(%rbp), %xmm1
140143879:     	movaps	%xmm1, 0x50(%rbp)
14014387d:     	movaps	%xmm0, 0x40(%rbp)
140143881:     	leaq	0x40(%rbp), %rcx
140143885:     	callq	0x140d76ed0 <.text+0xd75ed0>
14014388a:     	movq	%rdx, %r8
14014388d:     	leaq	0x280(%rbp), %rcx
140143894:     	movq	%rax, %rdx
140143897:     	callq	0x140d59140 <.text+0xd58140>
14014389c:     	movq	0x40(%rbp), %rdx
1401438a0:     	testq	%rdx, %rdx
1401438a3:     	je	0x1401438b4 <.text+0x1428b4>
1401438a5:     	movq	0x48(%rbp), %rcx
1401438a9:     	movl	$0x1, %r8d
1401438af:     	callq	0x140613c20 <.text+0x612c20>
1401438b4:     	movq	0x280(%rbp), %rcx
1401438bb:     	movq	%rcx, %rax
1401438be:     	negq	%rax
1401438c1:     	jno	0x14014398c <.text+0x14298c>
1401438c7:     	movq	0x288(%rbp), %rax
1401438ce:     	movl	%eax, %ecx
1401438d0:     	andl	$0x3, %ecx
1401438d3:     	cmpl	$0x1, %ecx
1401438d6:     	jne	0x140143aa8 <.text+0x142aa8>
1401438dc:     	leaq	-0x1(%rax), %rcx
1401438e0:     	movq	%rcx, 0x2e8(%rbp)
1401438e7:     	movq	-0x1(%rax), %rcx
1401438eb:     	movq	%rcx, 0x300(%rbp)
1401438f2:     	movq	0x7(%rax), %rax
1401438f6:     	movq	%rax, 0x2f0(%rbp)
1401438fd:     	movq	(%rax), %rax
140143900:     	testq	%rax, %rax
140143903:     	je	0x14014390e <.text+0x14290e>
140143905:     	movq	0x300(%rbp), %rcx
14014390c:     	callq	*%rax
14014390e:     	movq	0x300(%rbp), %rcx
140143915:     	movq	0x2f0(%rbp), %rax
14014391c:     	movq	0x8(%rax), %rdx
140143920:     	testq	%rdx, %rdx
140143923:     	je	0x14014392e <.text+0x14292e>
140143925:     	movq	0x10(%rax), %r8
140143929:     	callq	0x140613c20 <.text+0x612c20>
14014392e:     	movl	$0x18, %edx
140143933:     	movl	$0x8, %r8d
140143939:     	movq	0x2e8(%rbp), %rcx
140143940:     	jmp	0x140143aa3 <.text+0x142aa3>
140143945:     	movb	$0x0, 0x40(%rbp)
140143949:     	cmpb	$0x0, 0x2d3(%rbp)
140143950:     	je	0x14014396a <.text+0x14296a>
140143952:     	leaq	0x290(%rbp), %rcx
140143959:     	leaq	0x40(%rbp), %rdx
14014395d:     	callq	0x140042870 <.text+0x41870>
140143962:     	testb	%al, %al
140143964:     	jne	0x140143690 <.text+0x142690>
14014396a:     	movq	0x290(%rbp), %rax
140143971:     	lock
140143972:     	decq	(%rax)
140143975:     	jne	0x140143ac6 <.text+0x142ac6>
14014397b:     	leaq	0x290(%rbp), %rcx
140143982:     	callq	0x140c41170 <.text+0xc40170>
140143987:     	jmp	0x140143ac6 <.text+0x142ac6>
14014398c:     	movq	%rcx, 0x300(%rbp)
140143993:     	movq	0x288(%rbp), %rdx
14014399a:     	movq	0x290(%rbp), %r8
1401439a1:     	leaq	-0x50(%rbp), %rcx
1401439a5:     	movq	%rdx, 0x2f0(%rbp)
1401439ac:     	callq	0x140058a80 <.text+0x57a80>
1401439b1:     	movq	-0x50(%rbp), %rdi
1401439b5:     	movq	%rdi, %rax
1401439b8:     	negq	%rax
1401439bb:     	jno	0x140143a0a <.text+0x142a0a>
1401439bd:     	movq	-0x48(%rbp), %rcx
1401439c1:     	movq	(%rcx), %rax
1401439c4:     	cmpq	$0x1, %rax
1401439c8:     	movq	%rcx, 0x2e8(%rbp)
1401439cf:     	je	0x140143a6a <.text+0x142a6a>
1401439d5:     	testq	%rax, %rax
1401439d8:     	jne	0x140143a73 <.text+0x142a73>
1401439de:     	movq	0x2e8(%rbp), %rax
1401439e5:     	movq	0x10(%rax), %rdx
1401439e9:     	testq	%rdx, %rdx
1401439ec:     	je	0x140143a73 <.text+0x142a73>
1401439f2:     	movq	0x2e8(%rbp), %rax
1401439f9:     	movq	0x8(%rax), %rcx
1401439fd:     	movl	$0x1, %r8d
140143a03:     	callq	0x140613c20 <.text+0x612c20>
140143a08:     	jmp	0x140143a73 <.text+0x142a73>
140143a0a:     	movq	-0x48(%rbp), %rbx
140143a0e:     	movq	-0x40(%rbp), %r14
140143a12:     	movq	(%rsi), %rax
140143a15:     	testq	%rax, %rax
140143a18:     	je	0x140143a31 <.text+0x142a31>
140143a1a:     	movq	0x8(%rsi), %rcx
140143a1e:     	shlq	$0x2, %rax
140143a22:     	leaq	(%rax,%rax,8), %rdx
140143a26:     	movl	$0x4, %r8d
140143a2c:     	callq	0x140613c20 <.text+0x612c20>
140143a31:     	movl	$0x0, 0x18(%rsi)
140143a38:     	movq	$0x0, 0x20(%rsi)
140143a40:     	movq	%rdi, (%rsi)
140143a43:     	movq	%rbx, 0x8(%rsi)
140143a47:     	movq	%r14, 0x10(%rsi)
140143a4b:     	movq	$0x4, (%r12)
140143a53:     	movq	$0x0, 0x8(%r12)
140143a5c:     	movq	0x300(%rbp), %rdx
140143a63:     	testq	%rdx, %rdx
140143a66:     	jne	0x140143a96 <.text+0x142a96>
140143a68:     	jmp	0x140143aa8 <.text+0x142aa8>
140143a6a:     	addq	$0x8, %rcx
140143a6e:     	callq	0x1405e7a00 <.text+0x5e6a00>
140143a73:     	movl	$0x28, %edx
140143a78:     	movl	$0x8, %r8d
140143a7e:     	movq	0x2e8(%rbp), %rcx
140143a85:     	callq	0x140613c20 <.text+0x612c20>
140143a8a:     	movq	0x300(%rbp), %rdx
140143a91:     	testq	%rdx, %rdx
140143a94:     	je	0x140143aa8 <.text+0x142aa8>
140143a96:     	movl	$0x1, %r8d
140143a9c:     	movq	0x2f0(%rbp), %rcx
140143aa3:     	callq	0x140613c20 <.text+0x612c20>
140143aa8:     	movq	0x210(%rbp), %rdx
140143aaf:     	testq	%rdx, %rdx
140143ab2:     	je	0x140143ac6 <.text+0x142ac6>
140143ab4:     	movq	0x218(%rbp), %rcx
140143abb:     	movl	$0x1, %r8d
140143ac1:     	callq	0x140613c20 <.text+0x612c20>
140143ac6:     	nop
140143ac7:     	addq	$0x398, %rsp            # imm = 0x398
140143ace:     	popq	%rbx
140143acf:     	popq	%rdi
140143ad0:     	popq	%rsi
140143ad1:     	popq	%r12
140143ad3:     	popq	%r13
140143ad5:     	popq	%r14
140143ad7:     	popq	%r15
140143ad9:     	popq	%rbp
140143ada:     	retq
140143adb:     	movq	0x48(%rbp), %rax
140143adf:     	movq	%rax, 0x280(%rbp)
140143ae6:     	leaq	0x280d9d3(%rip), %rax   # 0x1429514c0
140143aed:     	movq	%rax, 0x20(%rsp)
140143af2:     	leaq	0x280d36f(%rip), %rcx   # 0x142950e68
140143af9:     	leaq	0x280d398(%rip), %r9    # 0x142950e98
140143b00:     	leaq	0x280(%rbp), %r8
140143b07:     	movl	$0x2b, %edx
140143b0c:     	callq	0x1428d9760 <.text+0x28d8760>
140143b11:     	jmp	0x140143bbe <.text+0x142bbe>
140143b16:     	movq	0x48(%rbp), %rax
140143b1a:     	movq	%rax, 0x280(%rbp)
140143b21:     	leaq	0x280d9e0(%rip), %rax   # 0x142951508
140143b28:     	movq	%rax, 0x20(%rsp)
140143b2d:     	leaq	0x280d334(%rip), %rcx   # 0x142950e68
140143b34:     	leaq	0x280d35d(%rip), %r9    # 0x142950e98
140143b3b:     	leaq	0x280(%rbp), %r8
140143b42:     	movl	$0x2b, %edx
140143b47:     	callq	0x1428d9760 <.text+0x28d8760>
140143b4c:     	jmp	0x140143bbe <.text+0x142bbe>
140143b4e:     	leaq	0x280d2d3(%rip), %r8    # 0x142950e28
140143b55:     	movl	$0x1, %ecx
140143b5a:     	movl	$0x80, %edx
140143b5f:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140143b64:     	jmp	0x140143bbe <.text+0x142bbe>
140143b66:     	movq	%rax, 0x300(%rbp)
140143b6d:     	movq	0x280(%rbp), %rdx
140143b74:     	testq	%rdx, %rdx
140143b77:     	je	0x140143b8b <.text+0x142b8b>
140143b79:     	movq	0x288(%rbp), %rcx
140143b80:     	movl	$0x1, %r8d
140143b86:     	callq	0x140613c20 <.text+0x612c20>
140143b8b:     	movq	0x300(%rbp), %rax
140143b92:     	movq	%rax, 0x40(%rbp)
140143b96:     	leaq	0x280d93b(%rip), %rax   # 0x1429514d8
140143b9d:     	movq	%rax, 0x20(%rsp)
140143ba2:     	leaq	0x280d2bf(%rip), %rcx   # 0x142950e68
140143ba9:     	leaq	0x280d298(%rip), %r9    # 0x142950e48
140143bb0:     	leaq	0x40(%rbp), %r8
140143bb4:     	movl	$0x2b, %edx
140143bb9:     	callq	0x1428d9760 <.text+0x28d8760>
140143bbe:     	ud2
140143bc0:     	movq	%rdx, 0x10(%rsp)
140143bc5:     	pushq	%rbp
140143bc6:     	pushq	%r15
140143bc8:     	pushq	%r14
140143bca:     	pushq	%r13
140143bcc:     	pushq	%r12
140143bce:     	pushq	%rsi
140143bcf:     	pushq	%rdi
140143bd0:     	pushq	%rbx
140143bd1:     	subq	$0x38, %rsp
140143bd5:     	leaq	0x80(%rdx), %rbp
140143bdc:     	cmpb	$0x0, 0x2ff(%rbp)
140143be3:     	jne	0x140143bf5 <.text+0x142bf5>
140143be5:     	cmpq	$0x4, -0x50(%rbp)
140143bea:     	je	0x140143bf5 <.text+0x142bf5>
140143bec:     	leaq	-0x50(%rbp), %rcx
140143bf0:     	callq	0x1405ea2f0 <.text+0x5e92f0>
140143bf5:     	nop
140143bf6:     	addq	$0x38, %rsp
140143bfa:     	popq	%rbx
140143bfb:     	popq	%rdi
140143bfc:     	popq	%rsi
140143bfd:     	popq	%r12
140143bff:     	popq	%r13
140143c01:     	popq	%r14
140143c03:     	popq	%r15
140143c05:     	popq	%rbp
140143c06:     	retq
140143c07:     	nopw	(%rax,%rax)
140143c10:     	movq	%rdx, 0x10(%rsp)
140143c15:     	pushq	%rbp
140143c16:     	pushq	%r15
140143c18:     	pushq	%r14
140143c1a:     	pushq	%r13
140143c1c:     	pushq	%r12
140143c1e:     	pushq	%rsi
140143c1f:     	pushq	%rdi
140143c20:     	pushq	%rbx
140143c21:     	subq	$0x38, %rsp
140143c25:     	leaq	0x80(%rdx), %rbp
140143c2c:     	leaq	0x290(%rbp), %rcx
140143c33:     	movq	0x290(%rbp), %rax
140143c3a:     	lock
140143c3b:     	decq	(%rax)
140143c3e:     	jne	0x140143c45 <.text+0x142c45>
140143c40:     	callq	0x140c41170 <.text+0xc40170>
140143c45:     	nop
140143c46:     	addq	$0x38, %rsp
140143c4a:     	popq	%rbx
140143c4b:     	popq	%rdi
140143c4c:     	popq	%rsi
140143c4d:     	popq	%r12
140143c4f:     	popq	%r13
140143c51:     	popq	%r14
140143c53:     	popq	%r15
140143c55:     	popq	%rbp
140143c56:     	retq
140143c57:     	nopw	(%rax,%rax)
140143c60:     	movq	%rdx, 0x10(%rsp)
140143c65:     	pushq	%rbp
140143c66:     	pushq	%r15
140143c68:     	pushq	%r14
140143c6a:     	pushq	%r13
140143c6c:     	pushq	%r12
140143c6e:     	pushq	%rsi
140143c6f:     	pushq	%rdi
140143c70:     	pushq	%rbx
140143c71:     	subq	$0x38, %rsp
140143c75:     	leaq	0x80(%rdx), %rbp
140143c7c:     	movq	0x40(%rbp), %rdx
140143c80:     	testq	%rdx, %rdx
140143c83:     	je	0x140143c94 <.text+0x142c94>
140143c85:     	movq	0x48(%rbp), %rcx
140143c89:     	movl	$0x1, %r8d
140143c8f:     	callq	0x140613c20 <.text+0x612c20>
140143c94:     	nop
140143c95:     	addq	$0x38, %rsp
140143c99:     	popq	%rbx
140143c9a:     	popq	%rdi
140143c9b:     	popq	%rsi
140143c9c:     	popq	%r12
140143c9e:     	popq	%r13
140143ca0:     	popq	%r14
140143ca2:     	popq	%r15
140143ca4:     	popq	%rbp
140143ca5:     	retq
140143ca6:     	nopw	%cs:(%rax,%rax)
140143cb0:     	movq	%rdx, 0x10(%rsp)
140143cb5:     	pushq	%rbp
140143cb6:     	pushq	%r15
140143cb8:     	pushq	%r14
140143cba:     	pushq	%r13
140143cbc:     	pushq	%r12
140143cbe:     	pushq	%rsi
140143cbf:     	pushq	%rdi
140143cc0:     	pushq	%rbx
140143cc1:     	subq	$0x38, %rsp
140143cc5:     	leaq	0x80(%rdx), %rbp
140143ccc:     	movq	0x300(%rbp), %rcx
140143cd3:     	movq	0x2f0(%rbp), %rax
140143cda:     	movq	0x8(%rax), %rdx
140143cde:     	testq	%rdx, %rdx
140143ce1:     	je	0x140143cf3 <.text+0x142cf3>
140143ce3:     	movq	0x2f0(%rbp), %rax
140143cea:     	movq	0x10(%rax), %r8
140143cee:     	callq	0x140613c20 <.text+0x612c20>
140143cf3:     	movl	$0x18, %edx
140143cf8:     	movl	$0x8, %r8d
140143cfe:     	movq	0x2e8(%rbp), %rcx
140143d05:     	callq	0x140613c20 <.text+0x612c20>
140143d0a:     	nop
140143d0b:     	addq	$0x38, %rsp
140143d0f:     	popq	%rbx
140143d10:     	popq	%rdi
140143d11:     	popq	%rsi
140143d12:     	popq	%r12
140143d14:     	popq	%r13
140143d16:     	popq	%r14
140143d18:     	popq	%r15
140143d1a:     	popq	%rbp
140143d1b:     	retq
140143d1c:     	nopl	(%rax)
140143d20:     	movq	%rdx, 0x10(%rsp)
140143d25:     	pushq	%rbp
140143d26:     	pushq	%r15
140143d28:     	pushq	%r14
140143d2a:     	pushq	%r13
140143d2c:     	pushq	%r12
140143d2e:     	pushq	%rsi
140143d2f:     	pushq	%rdi
140143d30:     	pushq	%rbx
140143d31:     	subq	$0x38, %rsp
140143d35:     	leaq	0x80(%rdx), %rbp
140143d3c:     	cmpb	$0x0, 0x2fe(%rbp)
140143d43:     	jne	0x140143d55 <.text+0x142d55>
140143d45:     	cmpq	$0x4, -0x50(%rbp)
140143d4a:     	je	0x140143d55 <.text+0x142d55>
140143d4c:     	leaq	-0x50(%rbp), %rcx
140143d50:     	callq	0x1405ea2f0 <.text+0x5e92f0>
140143d55:     	nop
140143d56:     	addq	$0x38, %rsp
140143d5a:     	popq	%rbx
140143d5b:     	popq	%rdi
140143d5c:     	popq	%rsi
140143d5d:     	popq	%r12
140143d5f:     	popq	%r13
140143d61:     	popq	%r14
140143d63:     	popq	%r15
140143d65:     	popq	%rbp
140143d66:     	retq
140143d67:     	nopw	(%rax,%rax)
140143d70:     	movq	%rdx, 0x10(%rsp)
140143d75:     	pushq	%rbp
140143d76:     	pushq	%r15
140143d78:     	pushq	%r14
140143d7a:     	pushq	%r13
140143d7c:     	pushq	%r12
140143d7e:     	pushq	%rsi
140143d7f:     	pushq	%rdi
140143d80:     	pushq	%rbx
140143d81:     	subq	$0x38, %rsp
140143d85:     	leaq	0x80(%rdx), %rbp
140143d8c:     	leaq	-0x50(%rbp), %rcx
140143d90:     	callq	0x1405eb380 <.text+0x5ea380>
140143d95:     	nop
140143d96:     	addq	$0x38, %rsp
140143d9a:     	popq	%rbx
140143d9b:     	popq	%rdi
140143d9c:     	popq	%rsi
140143d9d:     	popq	%r12
140143d9f:     	popq	%r13
140143da1:     	popq	%r14
140143da3:     	popq	%r15
140143da5:     	popq	%rbp
140143da6:     	retq
140143da7:     	nopw	(%rax,%rax)
140143db0:     	movq	%rdx, 0x10(%rsp)
140143db5:     	pushq	%rbp
140143db6:     	pushq	%r15
140143db8:     	pushq	%r14
140143dba:     	pushq	%r13
140143dbc:     	pushq	%r12
140143dbe:     	pushq	%rsi
140143dbf:     	pushq	%rdi
140143dc0:     	pushq	%rbx
140143dc1:     	subq	$0x38, %rsp
140143dc5:     	leaq	0x80(%rdx), %rbp
140143dcc:     	movb	$0x1, 0x30f(%rbp)
140143dd3:     	leaq	0x280(%rbp), %rcx
140143dda:     	callq	0x1405e7a00 <.text+0x5e6a00>
140143ddf:     	movb	$0x1, 0x30f(%rbp)
140143de6:     	addq	$0x38, %rsp
140143dea:     	popq	%rbx
140143deb:     	popq	%rdi
140143dec:     	popq	%rsi
140143ded:     	popq	%r12
140143def:     	popq	%r13
140143df1:     	popq	%r14
140143df3:     	popq	%r15
140143df5:     	popq	%rbp
140143df6:     	retq
140143df7:     	nopw	(%rax,%rax)
140143e00:     	movq	%rdx, 0x10(%rsp)
140143e05:     	pushq	%rbp
140143e06:     	pushq	%r15
140143e08:     	pushq	%r14
140143e0a:     	pushq	%r13
140143e0c:     	pushq	%r12
140143e0e:     	pushq	%rsi
140143e0f:     	pushq	%rdi
140143e10:     	pushq	%rbx
140143e11:     	subq	$0x38, %rsp
140143e15:     	leaq	0x80(%rdx), %rbp
140143e1c:     	movq	0x210(%rbp), %rdx
140143e23:     	testq	%rdx, %rdx
140143e26:     	je	0x140143e3a <.text+0x142e3a>
140143e28:     	movq	0x218(%rbp), %rcx
140143e2f:     	movl	$0x1, %r8d
140143e35:     	callq	0x140613c20 <.text+0x612c20>
140143e3a:     	nop
140143e3b:     	addq	$0x38, %rsp
140143e3f:     	popq	%rbx
140143e40:     	popq	%rdi
140143e41:     	popq	%rsi
140143e42:     	popq	%r12
140143e44:     	popq	%r13
140143e46:     	popq	%r14
140143e48:     	popq	%r15
140143e4a:     	popq	%rbp
140143e4b:     	retq
140143e4c:     	nopl	(%rax)
140143e50:     	movq	%rdx, 0x10(%rsp)
140143e55:     	pushq	%rbp
140143e56:     	pushq	%r15
140143e58:     	pushq	%r14
140143e5a:     	pushq	%r13
140143e5c:     	pushq	%r12
140143e5e:     	pushq	%rsi
140143e5f:     	pushq	%rdi
140143e60:     	pushq	%rbx
140143e61:     	subq	$0x38, %rsp
140143e65:     	leaq	0x80(%rdx), %rbp
140143e6c:     	movq	0x280(%rbp), %rdx
140143e73:     	testq	%rdx, %rdx
140143e76:     	je	0x140143e8a <.text+0x142e8a>
140143e78:     	movq	0x288(%rbp), %rcx
140143e7f:     	movl	$0x1, %r8d
140143e85:     	callq	0x140613c20 <.text+0x612c20>
140143e8a:     	nop
140143e8b:     	addq	$0x38, %rsp
140143e8f:     	popq	%rbx
140143e90:     	popq	%rdi
140143e91:     	popq	%rsi
140143e92:     	popq	%r12
140143e94:     	popq	%r13
140143e96:     	popq	%r14
140143e98:     	popq	%r15
140143e9a:     	popq	%rbp
140143e9b:     	retq
140143e9c:     	nopl	(%rax)
140143ea0:     	movq	%rdx, 0x10(%rsp)
140143ea5:     	pushq	%rbp
140143ea6:     	pushq	%r15
140143ea8:     	pushq	%r14
140143eaa:     	pushq	%r13
140143eac:     	pushq	%r12
140143eae:     	pushq	%rsi
140143eaf:     	pushq	%rdi
140143eb0:     	pushq	%rbx
140143eb1:     	subq	$0x38, %rsp
140143eb5:     	leaq	0x80(%rdx), %rbp
140143ebc:     	movq	-0x50(%rbp), %rdx
140143ec0:     	testq	%rdx, %rdx
140143ec3:     	je	0x140143ed4 <.text+0x142ed4>
140143ec5:     	movq	-0x48(%rbp), %rcx
140143ec9:     	movl	$0x1, %r8d
140143ecf:     	callq	0x140613c20 <.text+0x612c20>
140143ed4:     	nop
140143ed5:     	addq	$0x38, %rsp
140143ed9:     	popq	%rbx
140143eda:     	popq	%rdi
140143edb:     	popq	%rsi
140143edc:     	popq	%r12
140143ede:     	popq	%r13
140143ee0:     	popq	%r14
140143ee2:     	popq	%r15
140143ee4:     	popq	%rbp
140143ee5:     	retq
140143ee6:     	nopw	%cs:(%rax,%rax)
140143ef0:     	movq	%rdx, 0x10(%rsp)
140143ef5:     	pushq	%rbp
140143ef6:     	pushq	%r15
140143ef8:     	pushq	%r14
140143efa:     	pushq	%r13
140143efc:     	pushq	%r12
140143efe:     	pushq	%rsi
140143eff:     	pushq	%rdi
140143f00:     	pushq	%rbx
140143f01:     	subq	$0x38, %rsp
140143f05:     	leaq	0x80(%rdx), %rbp
140143f0c:     	leaq	0x40(%rbp), %rcx
140143f10:     	callq	0x1405e88a0 <.text+0x5e78a0>
140143f15:     	nop
140143f16:     	addq	$0x38, %rsp
140143f1a:     	popq	%rbx
140143f1b:     	popq	%rdi
140143f1c:     	popq	%rsi
140143f1d:     	popq	%r12
140143f1f:     	popq	%r13
140143f21:     	popq	%r14
140143f23:     	popq	%r15
140143f25:     	popq	%rbp
140143f26:     	retq
140143f27:     	nopw	(%rax,%rax)
140143f30:     	movq	%rdx, 0x10(%rsp)
140143f35:     	pushq	%rbp
140143f36:     	pushq	%r15
140143f38:     	pushq	%r14
140143f3a:     	pushq	%r13
140143f3c:     	pushq	%r12
140143f3e:     	pushq	%rsi
140143f3f:     	pushq	%rdi
140143f40:     	pushq	%rbx
140143f41:     	subq	$0x38, %rsp
140143f45:     	leaq	0x80(%rdx), %rbp
140143f4c:     	cmpq	$0x0, 0x2f0(%rbp)
140143f54:     	je	0x140143f6f <.text+0x142f6f>
140143f56:     	movl	$0x1, %r8d
140143f5c:     	movq	0x300(%rbp), %rcx
140143f63:     	movq	0x2f0(%rbp), %rdx
140143f6a:     	callq	0x140613c20 <.text+0x612c20>
140143f6f:     	nop
140143f70:     	addq	$0x38, %rsp
140143f74:     	popq	%rbx
140143f75:     	popq	%rdi
140143f76:     	popq	%rsi
140143f77:     	popq	%r12
140143f79:     	popq	%r13
140143f7b:     	popq	%r14
140143f7d:     	popq	%r15
140143f7f:     	popq	%rbp
140143f80:     	retq
140143f81:     	nopw	%cs:(%rax,%rax)
140143f90:     	movq	%rdx, 0x10(%rsp)
140143f95:     	pushq	%rbp
140143f96:     	pushq	%r15
140143f98:     	pushq	%r14
140143f9a:     	pushq	%r13
140143f9c:     	pushq	%r12
140143f9e:     	pushq	%rsi
140143f9f:     	pushq	%rdi
140143fa0:     	pushq	%rbx
140143fa1:     	subq	$0x38, %rsp
140143fa5:     	leaq	0x80(%rdx), %rbp
140143fac:     	movq	0x300(%rbp), %rcx
140143fb3:     	movq	0x2f0(%rbp), %rax
140143fba:     	movq	0x8(%rax), %rdx
140143fbe:     	testq	%rdx, %rdx
140143fc1:     	je	0x140143fd3 <.text+0x142fd3>
140143fc3:     	movq	0x2f0(%rbp), %rax
140143fca:     	movq	0x10(%rax), %r8
140143fce:     	callq	0x140613c20 <.text+0x612c20>
140143fd3:     	movl	$0x18, %edx
140143fd8:     	movl	$0x8, %r8d
140143fde:     	movq	0x2e8(%rbp), %rcx
140143fe5:     	callq	0x140613c20 <.text+0x612c20>
140143fea:     	nop
140143feb:     	addq	$0x38, %rsp
140143fef:     	popq	%rbx
140143ff0:     	popq	%rdi
140143ff1:     	popq	%rsi
140143ff2:     	popq	%r12
140143ff4:     	popq	%r13
140143ff6:     	popq	%r14
140143ff8:     	popq	%r15
140143ffa:     	popq	%rbp
140143ffb:     	retq
140143ffc:     	nopl	(%rax)
140144000:     	movq	%rdx, 0x10(%rsp)
140144005:     	pushq	%rbp
140144006:     	pushq	%r15
140144008:     	pushq	%r14
14014400a:     	pushq	%r13
14014400c:     	pushq	%r12
14014400e:     	pushq	%rsi
14014400f:     	pushq	%rdi
140144010:     	pushq	%rbx
140144011:     	subq	$0x38, %rsp
140144015:     	leaq	0x80(%rdx), %rbp
14014401c:     	leaq	0x290(%rbp), %rcx
140144023:     	movq	0x290(%rbp), %rax
14014402a:     	lock
14014402b:     	decq	(%rax)
14014402e:     	jne	0x140144035 <.text+0x143035>
140144030:     	callq	0x140c41170 <.text+0xc40170>
140144035:     	nop
140144036:     	addq	$0x38, %rsp
14014403a:     	popq	%rbx
14014403b:     	popq	%rdi
14014403c:     	popq	%rsi
14014403d:     	popq	%r12
14014403f:     	popq	%r13
140144041:     	popq	%r14
140144043:     	popq	%r15
140144045:     	popq	%rbp
140144046:     	retq
140144047:     	nopw	(%rax,%rax)
140144050:     	movq	%rdx, 0x10(%rsp)
140144055:     	pushq	%rbp
140144056:     	pushq	%r15
140144058:     	pushq	%r14
14014405a:     	pushq	%r13
14014405c:     	pushq	%r12
14014405e:     	pushq	%rsi
14014405f:     	pushq	%rdi
140144060:     	pushq	%rbx
140144061:     	subq	$0x38, %rsp
140144065:     	leaq	0x80(%rdx), %rbp
14014406c:     	movq	0x40(%rbp), %rdx
140144070:     	testq	%rdx, %rdx
140144073:     	je	0x140144084 <.text+0x143084>
140144075:     	movq	0x48(%rbp), %rcx
140144079:     	movl	$0x1, %r8d
14014407f:     	callq	0x140613c20 <.text+0x612c20>
140144084:     	nop
140144085:     	addq	$0x38, %rsp
140144089:     	popq	%rbx
14014408a:     	popq	%rdi
14014408b:     	popq	%rsi
14014408c:     	popq	%r12
14014408e:     	popq	%r13
140144090:     	popq	%r14
140144092:     	popq	%r15
140144094:     	popq	%rbp
140144095:     	retq
140144096:     	nopw	%cs:(%rax,%rax)
1401440a0:     	movq	%rdx, 0x10(%rsp)
1401440a5:     	pushq	%rbp
1401440a6:     	pushq	%r15
1401440a8:     	pushq	%r14
1401440aa:     	pushq	%r13
1401440ac:     	pushq	%r12
1401440ae:     	pushq	%rsi
1401440af:     	pushq	%rdi
1401440b0:     	pushq	%rbx
1401440b1:     	subq	$0x38, %rsp
1401440b5:     	leaq	0x80(%rdx), %rbp
1401440bc:     	movq	0x300(%rbp), %rcx
1401440c3:     	movq	0x2f0(%rbp), %rax
1401440ca:     	movq	0x8(%rax), %rdx
1401440ce:     	testq	%rdx, %rdx
1401440d1:     	je	0x1401440e3 <.text+0x1430e3>
1401440d3:     	movq	0x2f0(%rbp), %rax
1401440da:     	movq	0x10(%rax), %r8
1401440de:     	callq	0x140613c20 <.text+0x612c20>
1401440e3:     	movl	$0x18, %edx
1401440e8:     	movl	$0x8, %r8d
1401440ee:     	movq	0x2e8(%rbp), %rcx
1401440f5:     	callq	0x140613c20 <.text+0x612c20>
1401440fa:     	nop
1401440fb:     	addq	$0x38, %rsp
1401440ff:     	popq	%rbx
140144100:     	popq	%rdi
140144101:     	popq	%rsi
140144102:     	popq	%r12
140144104:     	popq	%r13
140144106:     	popq	%r14
140144108:     	popq	%r15
14014410a:     	popq	%rbp
14014410b:     	retq
14014410c:     	nopl	(%rax)
140144110:     	movq	%rdx, 0x10(%rsp)
140144115:     	pushq	%rbp
140144116:     	pushq	%r15
140144118:     	pushq	%r14
14014411a:     	pushq	%r13
14014411c:     	pushq	%r12
14014411e:     	pushq	%rsi
14014411f:     	pushq	%rdi
140144120:     	pushq	%rbx
140144121:     	subq	$0x38, %rsp
140144125:     	leaq	0x80(%rdx), %rbp
14014412c:     	leaq	-0x50(%rbp), %rcx
140144130:     	callq	0x1405eb380 <.text+0x5ea380>
140144135:     	nop
140144136:     	addq	$0x38, %rsp
14014413a:     	popq	%rbx
14014413b:     	popq	%rdi
14014413c:     	popq	%rsi
14014413d:     	popq	%r12
14014413f:     	popq	%r13
140144141:     	popq	%r14
140144143:     	popq	%r15
140144145:     	popq	%rbp
140144146:     	retq
140144147:     	nopw	(%rax,%rax)
140144150:     	movq	%rdx, 0x10(%rsp)
140144155:     	pushq	%rbp
140144156:     	pushq	%r15
140144158:     	pushq	%r14
14014415a:     	pushq	%r13
14014415c:     	pushq	%r12
14014415e:     	pushq	%rsi
14014415f:     	pushq	%rdi
140144160:     	pushq	%rbx
140144161:     	subq	$0x38, %rsp
140144165:     	leaq	0x80(%rdx), %rbp
14014416c:     	movb	$0x1, 0x30e(%rbp)
140144173:     	leaq	0x280(%rbp), %rcx
14014417a:     	callq	0x1405e7a00 <.text+0x5e6a00>
14014417f:     	movb	$0x1, 0x30e(%rbp)
140144186:     	addq	$0x38, %rsp
14014418a:     	popq	%rbx
14014418b:     	popq	%rdi
14014418c:     	popq	%rsi
14014418d:     	popq	%r12
14014418f:     	popq	%r13
140144191:     	popq	%r14
140144193:     	popq	%r15
140144195:     	popq	%rbp
140144196:     	retq
140144197:     	nopw	(%rax,%rax)
1401441a0:     	movq	%rdx, 0x10(%rsp)
1401441a5:     	pushq	%rbp
1401441a6:     	pushq	%r15
1401441a8:     	pushq	%r14
1401441aa:     	pushq	%r13
1401441ac:     	pushq	%r12
1401441ae:     	pushq	%rsi
1401441af:     	pushq	%rdi
1401441b0:     	pushq	%rbx
1401441b1:     	subq	$0x38, %rsp
1401441b5:     	leaq	0x80(%rdx), %rbp
1401441bc:     	movq	0x210(%rbp), %rdx
1401441c3:     	testq	%rdx, %rdx
1401441c6:     	je	0x1401441da <.text+0x1431da>
1401441c8:     	movq	0x218(%rbp), %rcx
1401441cf:     	movl	$0x1, %r8d
1401441d5:     	callq	0x140613c20 <.text+0x612c20>
1401441da:     	nop
1401441db:     	addq	$0x38, %rsp
1401441df:     	popq	%rbx
1401441e0:     	popq	%rdi
1401441e1:     	popq	%rsi
1401441e2:     	popq	%r12
1401441e4:     	popq	%r13
1401441e6:     	popq	%r14
1401441e8:     	popq	%r15
1401441ea:     	popq	%rbp
1401441eb:     	retq
1401441ec:     	nopl	(%rax)
1401441f0:     	movq	%rdx, 0x10(%rsp)
1401441f5:     	pushq	%rbp
1401441f6:     	pushq	%r15
1401441f8:     	pushq	%r14
1401441fa:     	pushq	%r13
1401441fc:     	pushq	%r12
1401441fe:     	pushq	%rsi
1401441ff:     	pushq	%rdi
140144200:     	pushq	%rbx
140144201:     	subq	$0x38, %rsp
140144205:     	leaq	0x80(%rdx), %rbp
14014420c:     	movq	0x40(%rbp), %rdx
140144210:     	testq	%rdx, %rdx
140144213:     	je	0x140144224 <.text+0x143224>
140144215:     	movq	0x48(%rbp), %rcx
140144219:     	movl	$0x1, %r8d
14014421f:     	callq	0x140613c20 <.text+0x612c20>
140144224:     	nop
140144225:     	addq	$0x38, %rsp
140144229:     	popq	%rbx
14014422a:     	popq	%rdi
14014422b:     	popq	%rsi
14014422c:     	popq	%r12
14014422e:     	popq	%r13
140144230:     	popq	%r14
140144232:     	popq	%r15
140144234:     	popq	%rbp
140144235:     	retq
140144236:     	nopw	%cs:(%rax,%rax)
140144240:     	movq	%rdx, 0x10(%rsp)
140144245:     	pushq	%rbp
140144246:     	pushq	%r15
140144248:     	pushq	%r14
14014424a:     	pushq	%r13
14014424c:     	pushq	%r12
14014424e:     	pushq	%rsi
14014424f:     	pushq	%rdi
140144250:     	pushq	%rbx
140144251:     	subq	$0x38, %rsp
140144255:     	leaq	0x80(%rdx), %rbp
14014425c:     	movq	0x300(%rbp), %rcx
140144263:     	movq	0x2f0(%rbp), %rax
14014426a:     	movq	0x8(%rax), %rdx
14014426e:     	testq	%rdx, %rdx
140144271:     	je	0x140144283 <.text+0x143283>
140144273:     	movq	0x2f0(%rbp), %rax
14014427a:     	movq	0x10(%rax), %r8
14014427e:     	callq	0x140613c20 <.text+0x612c20>
140144283:     	movl	$0x18, %edx
140144288:     	movl	$0x8, %r8d
14014428e:     	movq	0x2e8(%rbp), %rcx
140144295:     	callq	0x140613c20 <.text+0x612c20>
14014429a:     	nop
14014429b:     	addq	$0x38, %rsp
14014429f:     	popq	%rbx
1401442a0:     	popq	%rdi
1401442a1:     	popq	%rsi
1401442a2:     	popq	%r12
1401442a4:     	popq	%r13
1401442a6:     	popq	%r14
1401442a8:     	popq	%r15
1401442aa:     	popq	%rbp
1401442ab:     	retq
1401442ac:     	nopl	(%rax)
1401442b0:     	movq	%rdx, 0x10(%rsp)
1401442b5:     	pushq	%rbp
1401442b6:     	pushq	%r15
1401442b8:     	pushq	%r14
1401442ba:     	pushq	%r13
1401442bc:     	pushq	%r12
1401442be:     	pushq	%rsi
1401442bf:     	pushq	%rdi
1401442c0:     	pushq	%rbx
1401442c1:     	subq	$0x38, %rsp
1401442c5:     	leaq	0x80(%rdx), %rbp
1401442cc:     	cmpq	$0x0, 0x300(%rbp)
1401442d4:     	je	0x1401442ef <.text+0x1432ef>
1401442d6:     	movl	$0x1, %r8d
1401442dc:     	movq	0x2f0(%rbp), %rcx
1401442e3:     	movq	0x300(%rbp), %rdx
1401442ea:     	callq	0x140613c20 <.text+0x612c20>
1401442ef:     	nop
1401442f0:     	addq	$0x38, %rsp
1401442f4:     	popq	%rbx
1401442f5:     	popq	%rdi
1401442f6:     	popq	%rsi
1401442f7:     	popq	%r12
1401442f9:     	popq	%r13
1401442fb:     	popq	%r14
1401442fd:     	popq	%r15
1401442ff:     	popq	%rbp
140144300:     	retq
140144301:     	nopw	%cs:(%rax,%rax)
140144310:     	movq	%rdx, 0x10(%rsp)
140144315:     	pushq	%rbp
140144316:     	pushq	%r15
140144318:     	pushq	%r14
14014431a:     	pushq	%r13
14014431c:     	pushq	%r12
14014431e:     	pushq	%rsi
14014431f:     	pushq	%rdi
140144320:     	pushq	%rbx
140144321:     	subq	$0x38, %rsp
140144325:     	leaq	0x80(%rdx), %rbp
14014432c:     	movl	$0x28, %edx
140144331:     	movl	$0x8, %r8d
140144337:     	movq	0x2e8(%rbp), %rcx
14014433e:     	callq	0x140613c20 <.text+0x612c20>
140144343:     	nop
140144344:     	addq	$0x38, %rsp
140144348:     	popq	%rbx
140144349:     	popq	%rdi
14014434a:     	popq	%rsi
14014434b:     	popq	%r12
14014434d:     	popq	%r13
14014434f:     	popq	%r14
140144351:     	popq	%r15
140144353:     	popq	%rbp
140144354:     	retq
140144355:     	nopw	%cs:(%rax,%rax)
140144360:     	movq	%rdx, 0x10(%rsp)
140144365:     	pushq	%rbp
140144366:     	pushq	%r15
140144368:     	pushq	%r14
14014436a:     	pushq	%r13
14014436c:     	pushq	%r12
14014436e:     	pushq	%rsi
14014436f:     	pushq	%rdi
140144370:     	pushq	%rbx
140144371:     	subq	$0x38, %rsp
140144375:     	leaq	0x80(%rdx), %rbp
14014437c:     	cmpb	$0x0, 0x30e(%rbp)
140144383:     	je	0x1401443a0 <.text+0x1433a0>
140144385:     	leaq	-0x50(%rbp), %rcx
140144389:     	callq	0x1405eb380 <.text+0x5ea380>
14014438e:     	nop
14014438f:     	addq	$0x38, %rsp
140144393:     	popq	%rbx
140144394:     	popq	%rdi
140144395:     	popq	%rsi
140144396:     	popq	%r12
140144398:     	popq	%r13
14014439a:     	popq	%r14
14014439c:     	popq	%r15
14014439e:     	popq	%rbp
14014439f:     	retq
1401443a0:     	addq	$0x38, %rsp
1401443a4:     	popq	%rbx
1401443a5:     	popq	%rdi
1401443a6:     	popq	%rsi
1401443a7:     	popq	%r12
1401443a9:     	popq	%r13
1401443ab:     	popq	%r14
1401443ad:     	popq	%r15
1401443af:     	popq	%rbp
1401443b0:     	retq
1401443b1:     	nopw	%cs:(%rax,%rax)
1401443c0:     	movq	%rdx, 0x10(%rsp)
1401443c5:     	pushq	%rbp
1401443c6:     	pushq	%r15
1401443c8:     	pushq	%r14
1401443ca:     	pushq	%r13
1401443cc:     	pushq	%r12
1401443ce:     	pushq	%rsi
1401443cf:     	pushq	%rdi
1401443d0:     	pushq	%rbx
1401443d1:     	subq	$0x38, %rsp
1401443d5:     	leaq	0x80(%rdx), %rbp
1401443dc:     	cmpb	$0x0, 0x30f(%rbp)
1401443e3:     	je	0x140144400 <.text+0x143400>
1401443e5:     	leaq	-0x50(%rbp), %rcx
1401443e9:     	callq	0x1405eb380 <.text+0x5ea380>
1401443ee:     	nop
1401443ef:     	addq	$0x38, %rsp
1401443f3:     	popq	%rbx
1401443f4:     	popq	%rdi
1401443f5:     	popq	%rsi
1401443f6:     	popq	%r12
1401443f8:     	popq	%r13
1401443fa:     	popq	%r14
1401443fc:     	popq	%r15
1401443fe:     	popq	%rbp
1401443ff:     	retq
140144400:     	addq	$0x38, %rsp
140144404:     	popq	%rbx
140144405:     	popq	%rdi
140144406:     	popq	%rsi
140144407:     	popq	%r12
140144409:     	popq	%r13
14014440b:     	popq	%r14
14014440d:     	popq	%r15
14014440f:     	popq	%rbp
140144410:     	retq
