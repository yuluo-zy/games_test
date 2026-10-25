
reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
142291230:     	pushq	%rbp
142291231:     	pushq	%r15
142291233:     	pushq	%r14
142291235:     	pushq	%r13
142291237:     	pushq	%r12
142291239:     	pushq	%rsi
14229123a:     	pushq	%rdi
14229123b:     	pushq	%rbx
14229123c:     	subq	$0x848, %rsp            # imm = 0x848
142291243:     	leaq	0x80(%rsp), %rbp
14229124b:     	movaps	%xmm15, 0x7b0(%rbp)
142291253:     	movaps	%xmm14, 0x7a0(%rbp)
14229125b:     	movaps	%xmm13, 0x790(%rbp)
142291263:     	movaps	%xmm12, 0x780(%rbp)
14229126b:     	movaps	%xmm11, 0x770(%rbp)
142291273:     	movaps	%xmm10, 0x760(%rbp)
14229127b:     	movaps	%xmm9, 0x750(%rbp)
142291283:     	movaps	%xmm8, 0x740(%rbp)
14229128b:     	movaps	%xmm7, 0x730(%rbp)
142291292:     	movaps	%xmm6, 0x720(%rbp)
142291299:     	movq	$-0x2, 0x718(%rbp)
1422912a4:     	movq	%r9, 0x200(%rbp)
1422912ab:     	movq	0x840(%rbp), %rax
1422912b2:     	movq	0x838(%rbp), %r10
1422912b9:     	movups	(%r10), %xmm0
1422912bd:     	movups	0x10(%r10), %xmm1
1422912c2:     	movaps	%xmm1, 0x20(%rbp)
1422912c6:     	movaps	%xmm0, 0x10(%rbp)
1422912ca:     	movq	0x20(%r10), %r11
1422912ce:     	movq	%r11, 0x320(%rbp)
1422912d5:     	movq	0x40(%r10), %r11
1422912d9:     	movq	%r11, 0xa0(%rbp)
1422912e0:     	movq	0x60(%r10), %r11
1422912e4:     	movq	%r11, 0x180(%rbp)
1422912eb:     	movq	0x80(%r10), %r11
1422912f2:     	movq	%r11, 0x1b0(%rbp)
1422912f9:     	movq	0xa0(%r10), %r11
142291300:     	movq	%r11, 0x208(%rbp)
142291307:     	movq	0xc0(%r10), %r11
14229130e:     	movq	%r11, 0xc0(%rbp)
142291315:     	movq	0xc8(%r10), %r11
14229131c:     	movq	%r11, 0xc8(%rbp)
142291323:     	movq	0xd8(%r10), %r10
14229132a:     	movq	%r10, 0x660(%rbp)
142291331:     	movq	(%rax), %r10
142291334:     	movq	%r10, 0x670(%rbp)
14229133b:     	movq	0x10(%rax), %r10
14229133f:     	movq	%r10, 0x668(%rbp)
142291346:     	movl	0x1c(%rax), %r10d
14229134a:     	movl	%r10d, 0x6d8(%rbp)
142291351:     	movq	0x20(%rax), %rax
142291355:     	movq	%rax, 0x650(%rbp)
14229135c:     	movq	(%rdx), %r10
14229135f:     	movq	0x8(%rdx), %rax
142291363:     	movq	%rax, 0x500(%rbp)
14229136a:     	movl	0x14(%rdx), %eax
14229136d:     	movl	%eax, 0x5ec(%rbp)
142291373:     	movq	0xf8(%r10), %rdx
14229137a:     	movq	0x100(%r10), %rax
142291381:     	movq	%rdx, 0x310(%rbp)
142291388:     	leaq	(%rdx,%rax,4), %rax
14229138c:     	movq	%rax, 0x1d0(%rbp)
142291393:     	movq	(%rcx), %rax
142291396:     	movq	%rax, 0x608(%rbp)
14229139d:     	movq	0x8(%rcx), %rax
1422913a1:     	movq	%rax, 0x88(%rbp)
1422913a8:     	movl	0x14(%rcx), %eax
1422913ab:     	movl	%eax, 0x6f8(%rbp)
1422913b1:     	movl	0x1c(%r8), %eax
1422913b5:     	movl	%eax, 0x6f0(%rbp)
1422913bb:     	movq	(%r8), %rax
1422913be:     	movq	%rax, 0x688(%rbp)
1422913c5:     	movq	0x10(%r8), %rax
1422913c9:     	movq	%rax, 0x690(%rbp)
1422913d0:     	movq	0x850(%rbp), %rax
1422913d7:     	movl	0x1c(%rax), %ecx
1422913da:     	movl	%ecx, 0x2cc(%rbp)
1422913e0:     	movq	(%rax), %r14
1422913e3:     	movq	0x10(%rax), %rax
1422913e7:     	movq	%rax, 0x120(%rbp)
1422913ee:     	movq	0x860(%rbp), %rax
1422913f5:     	movl	0x1c(%rax), %edx
1422913f8:     	movl	%edx, 0x4b0(%rbp)
1422913fe:     	movq	(%rax), %rcx
142291401:     	movq	%rcx, 0x510(%rbp)
142291408:     	movq	0x10(%rax), %rax
14229140c:     	movq	%rax, 0x220(%rbp)
142291413:     	movq	0x878(%rbp), %rax
14229141a:     	movl	0x1c(%rax), %r8d
14229141e:     	movl	%r8d, 0x4b4(%rbp)
142291425:     	movq	(%rax), %rcx
142291428:     	movq	%rcx, 0x518(%rbp)
14229142f:     	movq	0x10(%rax), %rax
142291433:     	movq	%rax, 0x228(%rbp)
14229143a:     	movq	0x880(%rbp), %rax
142291441:     	movl	0x1c(%rax), %r11d
142291445:     	movl	%r11d, 0x4cc(%rbp)
14229144c:     	movq	(%rax), %rcx
14229144f:     	movq	%rcx, 0x520(%rbp)
142291456:     	movq	0x10(%rax), %rax
14229145a:     	movq	%rax, 0x288(%rbp)
142291461:     	movq	0x868(%rbp), %rax
142291468:     	movl	0x1c(%rax), %esi
14229146b:     	movl	%esi, 0x4bc(%rbp)
142291471:     	movq	(%rax), %rcx
142291474:     	movq	%rcx, 0x3d0(%rbp)
14229147b:     	movq	0x10(%rax), %rax
14229147f:     	movq	%rax, 0x240(%rbp)
142291486:     	movq	0x870(%rbp), %rax
14229148d:     	movl	0x1c(%rax), %edi
142291490:     	movl	%edi, 0x2d4(%rbp)
142291496:     	movq	(%rax), %rcx
142291499:     	movq	%rcx, 0x380(%rbp)
1422914a0:     	movq	0x10(%rax), %rax
1422914a4:     	movq	%rax, 0x150(%rbp)
1422914ab:     	movq	0x848(%rbp), %rax
1422914b2:     	movq	0x20(%rax), %rcx
1422914b6:     	movq	%rcx, 0x388(%rbp)
1422914bd:     	movq	0x30(%rax), %r15
1422914c1:     	movq	%r15, 0x158(%rbp)
1422914c8:     	movl	0x3c(%rax), %r15d
1422914cc:     	movl	%r15d, 0x2d8(%rbp)
1422914d3:     	movq	0x40(%rax), %rcx
1422914d7:     	movq	%rcx, 0x358(%rbp)
1422914de:     	movq	0x50(%rax), %r12
1422914e2:     	movq	%r12, 0xe0(%rbp)
1422914e9:     	movl	0x5c(%rax), %r12d
1422914ed:     	movl	%r12d, 0x2c0(%rbp)
1422914f4:     	movq	0x60(%rax), %rcx
1422914f8:     	movq	%rcx, 0x390(%rbp)
1422914ff:     	movq	0x70(%rax), %r13
142291503:     	movq	%r13, 0x160(%rbp)
14229150a:     	movl	0x7c(%rax), %r13d
14229150e:     	movl	%r13d, 0x2dc(%rbp)
142291515:     	movq	0x80(%rax), %rcx
14229151c:     	movq	%rcx, 0x378(%rbp)
142291523:     	movq	0x90(%rax), %rcx
14229152a:     	movq	%rcx, 0x130(%rbp)
142291531:     	movl	0x9c(%rax), %ecx
142291537:     	movl	%ecx, 0x2d0(%rbp)
14229153d:     	movq	0xa0(%rax), %rcx
142291544:     	movq	0xb0(%rax), %r9
14229154b:     	movq	%r9, 0x238(%rbp)
142291552:     	movl	0xbc(%rax), %r9d
142291559:     	movl	%r9d, 0x4b8(%rbp)
142291560:     	movq	0xc0(%rax), %r9
142291567:     	movq	0xd0(%rax), %rdx
14229156e:     	movq	%rdx, 0x218(%rbp)
142291575:     	movl	0xdc(%rax), %edx
14229157b:     	movl	%edx, 0x4ac(%rbp)
142291581:     	movq	0xe0(%rax), %rdx
142291588:     	movq	0xf0(%rax), %r8
14229158f:     	movq	%r8, 0x258(%rbp)
142291596:     	movl	0xfc(%rax), %r8d
14229159d:     	movl	%r8d, 0x4c0(%rbp)
1422915a4:     	movq	0x100(%rax), %r8
1422915ab:     	movq	%r8, 0x398(%rbp)
1422915b2:     	movq	0x110(%rax), %r11
1422915b9:     	movq	%r11, 0x168(%rbp)
1422915c0:     	movl	0x11c(%rax), %r11d
1422915c7:     	movl	%r11d, 0x2e0(%rbp)
1422915ce:     	movq	0x120(%rax), %r11
1422915d5:     	movq	0x130(%rax), %rsi
1422915dc:     	movq	%rsi, 0x260(%rbp)
1422915e3:     	movl	0x13c(%rax), %esi
1422915e9:     	movl	%esi, 0x4c4(%rbp)
1422915ef:     	movq	0x140(%rax), %rsi
1422915f6:     	movq	0x150(%rax), %rbx
1422915fd:     	movq	%rbx, 0xf8(%rbp)
142291604:     	movl	0x15c(%rax), %ebx
14229160a:     	movl	%ebx, 0x2c4(%rbp)
142291610:     	movq	0x160(%rax), %rbx
142291617:     	movq	0x170(%rax), %rdi
14229161e:     	movq	%rdi, 0x268(%rbp)
142291625:     	movl	0x17c(%rax), %edi
14229162b:     	movl	%edi, 0x4c8(%rbp)
142291631:     	movq	0x180(%rax), %rdi
142291638:     	movq	0x190(%rax), %r15
14229163f:     	movq	%r15, 0xa8(%rbp)
142291646:     	movl	0x19c(%rax), %r15d
14229164d:     	movl	%r15d, 0x2b4(%rbp)
142291654:     	movq	0x1a0(%rax), %r15
14229165b:     	movq	0x1b0(%rax), %r12
142291662:     	movq	%r12, 0x90(%rbp)
142291669:     	movl	0x1bc(%rax), %r12d
142291670:     	movl	%r12d, 0x2b0(%rbp)
142291677:     	movq	0x1c0(%rax), %r12
14229167e:     	movq	0x1d0(%rax), %r13
142291685:     	movq	%r13, 0x100(%rbp)
14229168c:     	movl	0x1dc(%rax), %r13d
142291693:     	movl	%r13d, 0x2c8(%rbp)
14229169a:     	movq	0x1e0(%rax), %r13
1422916a1:     	movq	0x1f0(%rax), %r8
1422916a8:     	movq	%r8, 0xb8(%rbp)
1422916af:     	movl	0x1fc(%rax), %r8d
1422916b6:     	movl	%r8d, 0x2bc(%rbp)
1422916bd:     	movl	0x1c(%rax), %r8d
1422916c1:     	movl	%r8d, 0x2e4(%rbp)
1422916c8:     	movq	(%rax), %r8
1422916cb:     	movq	0x10(%rax), %rax
1422916cf:     	movq	%rax, 0x170(%rbp)
1422916d6:     	movss	0x6a2f96(%rip), %xmm7   # 0x142934674
1422916de:     	xorps	%xmm8, %xmm8
1422916e2:     	movss	0x694185(%rip), %xmm9   # 0x142925870
1422916eb:     	movss	0x694294(%rip), %xmm10  # 0x142925988
1422916f4:     	movaps	0xda9be4(%rip), %xmm11  # 0x14303b2e0
1422916fc:     	movss	0x73b92b(%rip), %xmm12  # 0x1429cd030
142291705:     	movaps	0x6941d3(%rip), %xmm13  # 0x1429258e0
14229170d:     	movq	%r10, 0x528(%rbp)
142291714:     	movzbl	0x108(%r10), %eax
14229171c:     	movb	%al, 0x67f(%rbp)
142291722:     	movq	0x858(%rbp), %r10
142291729:     	movq	0x200(%rbp), %rax
142291730:     	movq	(%rax), %rax
142291733:     	movq	%rax, 0x338(%rbp)
14229173a:     	movq	%r15, 0x1b8(%rbp)
142291741:     	leaq	0x20(%r15), %rax
142291745:     	movq	%rax, 0x98(%rbp)
14229174c:     	movq	%rsi, 0x248(%rbp)
142291753:     	leaq	0x20(%rsi), %rax
142291757:     	movq	%rax, 0x138(%rbp)
14229175e:     	movq	%r12, 0x250(%rbp)
142291765:     	leaq	0x20(%r12), %rax
14229176a:     	movq	%rax, 0x140(%rbp)
142291771:     	movl	0x1c(%r10), %eax
142291775:     	movl	%eax, 0x2b8(%rbp)
14229177b:     	movq	(%r10), %rax
14229177e:     	movq	0x10(%r10), %r10
142291782:     	movq	%r10, 0xb0(%rbp)
142291789:     	movq	%rax, 0x1e0(%rbp)
142291790:     	leaq	0x20(%rax), %rax
142291794:     	movq	%rax, 0xd8(%rbp)
14229179b:     	movq	%r13, 0x1e8(%rbp)
1422917a2:     	leaq	0x20(%r13), %rax
1422917a6:     	movq	0x500(%rbp), %r13
1422917ad:     	movq	%rax, 0xe8(%rbp)
1422917b4:     	movq	%r14, 0x270(%rbp)
1422917bb:     	leaq	0x20(%r14), %rax
1422917bf:     	movq	%rax, 0x148(%rbp)
1422917c6:     	movq	%rcx, 0x3e0(%rbp)
1422917cd:     	leaq	0x20(%rcx), %rax
1422917d1:     	movq	%rax, 0x298(%rbp)
1422917d8:     	movq	%r9, 0x3d8(%rbp)
1422917df:     	leaq	0x20(%r9), %rax
1422917e3:     	movq	%rax, 0x278(%rbp)
1422917ea:     	movq	0x510(%rbp), %rax
1422917f1:     	leaq	0x20(%rax), %rax
1422917f5:     	movq	%rax, 0x280(%rbp)
1422917fc:     	movq	0x518(%rbp), %rax
142291803:     	leaq	0x20(%rax), %rax
142291807:     	movq	%rax, 0x290(%rbp)
14229180e:     	movq	0x520(%rbp), %rax
142291815:     	leaq	0x20(%rax), %rax
142291819:     	movq	%rax, 0x2a0(%rbp)
142291820:     	movq	%rbx, 0x370(%rbp)
142291827:     	leaq	0x20(%rbx), %rax
14229182b:     	movq	%rax, 0x108(%rbp)
142291832:     	movq	0x3d0(%rbp), %rax
142291839:     	leaq	0x20(%rax), %rax
14229183d:     	movq	%rax, 0xf0(%rbp)
142291844:     	movq	%rdx, 0x360(%rbp)
14229184b:     	leaq	0x20(%rdx), %rax
14229184f:     	movq	%rax, 0x110(%rbp)
142291856:     	movq	%r11, 0x368(%rbp)
14229185d:     	leaq	0x20(%r11), %rax
142291861:     	movq	%rax, 0x118(%rbp)
142291868:     	movq	0x380(%rbp), %rax
14229186f:     	leaq	0x20(%rax), %rax
142291873:     	movq	%rax, 0x188(%rbp)
14229187a:     	movq	0x378(%rbp), %rax
142291881:     	leaq	0x20(%rax), %rax
142291885:     	movq	%rax, 0x178(%rbp)
14229188c:     	movq	0x390(%rbp), %rax
142291893:     	leaq	0x20(%rax), %rax
142291897:     	movq	%rax, 0x190(%rbp)
14229189e:     	movq	0x398(%rbp), %rax
1422918a5:     	leaq	0x20(%rax), %rax
1422918a9:     	movq	%rax, 0x198(%rbp)
1422918b0:     	movq	0x358(%rbp), %rax
1422918b7:     	leaq	0x20(%rax), %rax
1422918bb:     	movq	%rax, 0x128(%rbp)
1422918c2:     	movq	0x388(%rbp), %rax
1422918c9:     	leaq	0x20(%rax), %rax
1422918cd:     	movq	%rax, 0x1a0(%rbp)
1422918d4:     	movq	%r8, 0x2a8(%rbp)
1422918db:     	leaq	0x20(%r8), %rax
1422918df:     	movq	%rax, 0x1a8(%rbp)
1422918e6:     	movq	%rdi, 0x1d8(%rbp)
1422918ed:     	leaq	0x20(%rdi), %rax
1422918f1:     	movq	%rax, 0xd0(%rbp)
1422918f8:     	movl	$0x8, %eax
1422918fd:     	movq	%rax, 0x1f0(%rbp)
142291904:     	movl	$0x0, %r14d
14229190a:     	movq	%rax, 0x1f8(%rbp)
142291911:     	movq	$0x0, 0x3c8(%rbp)
14229191c:     	movq	$0x0, 0x318(%rbp)
142291927:     	movq	$0x0, 0x3b8(%rbp)
142291932:     	movq	%rax, 0x3b0(%rbp)
142291939:     	movq	$0x0, 0x680(%rbp)
142291944:     	movq	$0x0, 0x350(%rbp)
14229194f:     	movl	$0x0, 0x5f8(%rbp)
142291959:     	xorl	%eax, %eax
14229195b:     	movl	$0x0, 0x5f4(%rbp)
142291965:     	movq	$0x0, 0x2f0(%rbp)
142291970:     	jmp	0x1422919a8 <.text+0x22909a8>
142291972:     	nopw	%cs:(%rax,%rax)
142291980:     	movq	0x1d8(%rbp), %rdx
142291987:     	movq	0x28(%rdx), %rax
14229198b:     	movq	%rsi, %rcx
14229198e:     	shlq	$0x4, %rcx
142291992:     	movq	%rdi, (%rax,%rcx)
142291996:     	movb	$0x0, 0x8(%rax,%rcx)
14229199b:     	incq	%rsi
14229199e:     	movq	%rsi, 0x30(%rdx)
1422919a2:     	incq	0x40(%rdx)
1422919a6:     	movb	$0x1, %al
1422919a8:     	cmpb	$0x0, 0x67f(%rbp)
1422919af:     	je	0x142291a70 <.text+0x2290a70>
1422919b5:     	movq	0x318(%rbp), %r12
1422919bc:     	movq	0x3b8(%rbp), %r9
1422919c3:     	cmpq	%r12, %r9
1422919c6:     	movq	0x310(%rbp), %r15
1422919cd:     	movq	0x1d0(%rbp), %r8
1422919d4:     	jne	0x142291a54 <.text+0x2290a54>
1422919d6:     	nopw	%cs:(%rax,%rax)
1422919e0:     	cmpq	%r8, %r15
1422919e3:     	je	0x142296843 <.text+0x2295843>
1422919e9:     	movl	(%r15), %edx
1422919ec:     	addq	$0x4, %r15
1422919f0:     	movq	0x1a8(%r13), %rcx
1422919f7:     	leaq	(%rdx,%rdx,8), %rdx
1422919fb:     	movq	0x10(%rcx,%rdx,8), %r12
142291a00:     	testq	%r12, %r12
142291a03:     	je	0x1422919e0 <.text+0x22909e0>
142291a05:     	leaq	(%rcx,%rdx,8), %rcx
142291a09:     	movq	0x528(%rbp), %rdx
142291a10:     	movq	0x110(%rdx), %rdx
142291a17:     	movq	0x38(%rcx), %r8
142291a1b:     	movq	(%r8,%rdx,8), %rdx
142291a1f:     	movq	0x8(%rcx), %r8
142291a23:     	movq	%r8, 0x1f0(%rbp)
142291a2a:     	movq	0x18(%rcx), %rcx
142291a2e:     	notq	%rdx
142291a31:     	leaq	(%rdx,%rdx,2), %rdx
142291a35:     	shlq	$0x4, %rdx
142291a39:     	movq	0x10(%rcx,%rdx), %r8
142291a3e:     	movq	%r8, 0x3c8(%rbp)
142291a45:     	movq	0x28(%rcx,%rdx), %rcx
142291a4a:     	movq	%rcx, 0x348(%rbp)
142291a51:     	xorl	%r9d, %r9d
142291a54:     	movq	0x1f0(%rbp), %rcx
142291a5b:     	leaq	(%rcx,%r9,8), %rcx
142291a5f:     	movl	%r9d, %edx
142291a62:     	jmp	0x142291b41 <.text+0x2290b41>
142291a67:     	nopw	(%rax,%rax)
142291a70:     	movq	0x318(%rbp), %r12
142291a77:     	movq	0x3b8(%rbp), %r9
142291a7e:     	cmpq	%r12, %r9
142291a81:     	movq	0x310(%rbp), %r15
142291a88:     	movq	0x1d0(%rbp), %r8
142291a8f:     	jne	0x142291b2a <.text+0x2290b2a>
142291a95:     	nopw	%cs:(%rax,%rax)
142291aa0:     	cmpq	%r8, %r15
142291aa3:     	je	0x142296843 <.text+0x2295843>
142291aa9:     	movl	(%r15), %edx
142291aac:     	addq	$0x4, %r15
142291ab0:     	movq	0x100(%r13), %rcx
142291ab7:     	leaq	(%rdx,%rdx,4), %rdx
142291abb:     	shlq	$0x5, %rdx
142291abf:     	movq	0x58(%rcx,%rdx), %r12
142291ac4:     	testq	%r12, %r12
142291ac7:     	je	0x142291aa0 <.text+0x2290aa0>
142291ac9:     	addq	%rdx, %rcx
142291acc:     	movl	0x94(%rcx), %edx
142291ad2:     	movq	0x1a8(%r13), %r8
142291ad9:     	leaq	(%rdx,%rdx,8), %rdx
142291add:     	movq	0x528(%rbp), %r9
142291ae4:     	movq	0x110(%r9), %r9
142291aeb:     	movq	0x18(%r8,%rdx,8), %r10
142291af0:     	movq	0x38(%r8,%rdx,8), %rdx
142291af5:     	movq	(%rdx,%r9,8), %rdx
142291af9:     	notq	%rdx
142291afc:     	leaq	(%rdx,%rdx,2), %rdx
142291b00:     	shlq	$0x4, %rdx
142291b04:     	movq	0x10(%r10,%rdx), %r8
142291b09:     	movq	%r8, 0x3c8(%rbp)
142291b10:     	movq	0x28(%r10,%rdx), %rdx
142291b15:     	movq	%rdx, 0x348(%rbp)
142291b1c:     	movq	0x50(%rcx), %rcx
142291b20:     	movq	%rcx, 0x1f8(%rbp)
142291b27:     	xorl	%r9d, %r9d
142291b2a:     	movq	%r9, %rdx
142291b2d:     	shlq	$0x4, %rdx
142291b31:     	movq	0x1f8(%rbp), %r8
142291b38:     	leaq	(%r8,%rdx), %rcx
142291b3c:     	movl	0x8(%r8,%rdx), %edx
142291b41:     	leaq	(%rdx,%rdx,4), %r10
142291b45:     	shlq	$0x5, %r10
142291b49:     	addq	0x3c8(%rbp), %r10
142291b50:     	movq	0x348(%rbp), %r8
142291b57:     	leaq	(%r8,%rdx,4), %rdx
142291b5b:     	movq	(%rcx), %rsi
142291b5e:     	movq	%rsi, %rcx
142291b61:     	shrq	$0x20, %rcx
142291b65:     	je	0x142296843 <.text+0x2295843>
142291b6b:     	movq	%r9, 0x3b8(%rbp)
142291b72:     	movq	%r14, 0x6a8(%rbp)
142291b79:     	movl	0x5ec(%rbp), %eax
142291b7f:     	movl	%eax, (%rdx)
142291b81:     	movq	0x660(%rbp), %rbx
142291b88:     	movl	0xa0(%rbx), %ecx
142291b8e:     	movss	0xa4(%rbx), %xmm1
142291b96:     	movq	%r10, 0x708(%rbp)
142291b9d:     	callq	0x14093fa90 <.text+0x93ea90>
142291ba2:     	movl	%eax, %edi
142291ba4:     	movl	0xa8(%rbx), %ecx
142291baa:     	movss	0xac(%rbx), %xmm1
142291bb2:     	callq	0x14093fa90 <.text+0x93ea90>
142291bb7:     	movq	0x708(%rbp), %r11
142291bbe:     	movl	%eax, %ecx
142291bc0:     	movl	%edi, %eax
142291bc2:     	orb	%cl, %al
142291bc4:     	je	0x142291bdb <.text+0x2290bdb>
142291bc6:     	testb	$0x1, (%r11)
142291bca:     	jne	0x142291bdb <.text+0x2290bdb>
142291bcc:     	movq	0x60(%r11), %rdx
142291bd0:     	movq	$0x1, (%r11)
142291bd7:     	movq	%rdx, 0x8(%r11)
142291bdb:     	movq	0x338(%rbp), %rdx
142291be2:     	cmpl	$0x1, 0x10(%rdx)
142291be6:     	jne	0x142291c90 <.text+0x2290c90>
142291bec:     	cmpq	%rsi, 0x18(%rdx)
142291bf0:     	sete	%dl
142291bf3:     	movl	%edx, 0x6cc(%rbp)
142291bf9:     	testb	%dl, %al
142291bfb:     	je	0x142291c9a <.text+0x2290c9a>
142291c01:     	movzbl	(%r11), %eax
142291c05:     	leaq	0x8(%r11), %r8
142291c09:     	testb	%al, %al
142291c0b:     	movl	$0x0, %edx
142291c10:     	cmoveq	%rdx, %r8
142291c14:     	testb	$0x1, %al
142291c16:     	je	0x142296cc1 <.text+0x2295cc1>
142291c1c:     	testb	%dil, %dil
142291c1f:     	je	0x142294ca8 <.text+0x2293ca8>
142291c25:     	movq	0x20(%r11), %r9
142291c29:     	leaq	0x1(%r9), %r10
142291c2d:     	jmp	0x142291c58 <.text+0x2290c58>
142291c2f:     	nop
142291c30:     	xorl	%edx, %edx
142291c32:     	divl	%r9d
142291c35:     	movq	%rdx, (%r8)
142291c38:     	movq	0x48(%r11), %rax
142291c3c:     	cmpq	%rax, %rdx
142291c3f:     	jae	0x142296c3e <.text+0x2295c3e>
142291c45:     	movq	0x40(%r11), %rax
142291c49:     	leaq	(%rdx,%rdx,4), %rdx
142291c4d:     	cmpb	$0x5, 0x18(%rax,%rdx,8)
142291c52:     	jne	0x142294ca8 <.text+0x2293ca8>
142291c58:     	decq	%r10
142291c5b:     	je	0x142294ca8 <.text+0x2293ca8>
142291c61:     	movq	(%r8), %rax
142291c64:     	incq	%rax
142291c67:     	movq	%rax, %rdx
142291c6a:     	orq	%r9, %rdx
142291c6d:     	shrq	$0x20, %rdx
142291c71:     	je	0x142291c30 <.text+0x2290c30>
142291c73:     	xorl	%edx, %edx
142291c75:     	divq	%r9
142291c78:     	movq	%rdx, (%r8)
142291c7b:     	movq	0x48(%r11), %rax
142291c7f:     	cmpq	%rax, %rdx
142291c82:     	jb	0x142291c45 <.text+0x2290c45>
142291c84:     	jmp	0x142296c3e <.text+0x2295c3e>
142291c89:     	nopl	(%rax)
142291c90:     	movl	$0x0, 0x6cc(%rbp)
142291c9a:     	movq	%r12, 0x318(%rbp)
142291ca1:     	movq	%r15, 0x310(%rbp)
142291ca8:     	incq	0x3b8(%rbp)
142291caf:     	incq	%r14
142291cb2:     	movq	%r14, 0x508(%rbp)
142291cb9:     	movq	0x18(%r11), %r8
142291cbd:     	movq	0x20(%r11), %rax
142291cc1:     	leaq	(%r8,%rax,8), %rax
142291cc5:     	movq	%rax, 0x628(%rbp)
142291ccc:     	movb	$0x2, 0x6a6(%rbp)
142291cd3:     	movb	$0x4, 0x6f6(%rbp)
142291cda:     	movq	$0x0, 0x1c8(%rbp)
142291ce5:     	movb	$0x3, 0x6de(%rbp)
142291cec:     	movl	$0x0, 0x5f0(%rbp)
142291cf6:     	movb	$0x3, 0x6ff(%rbp)
142291cfd:     	movq	$0x0, 0x340(%rbp)
142291d08:     	movb	$0x2, 0x6a7(%rbp)
142291d0f:     	movl	$0x0, 0x6ec(%rbp)
142291d19:     	movq	$0x0, 0x328(%rbp)
142291d24:     	movq	$0x0, 0x330(%rbp)
142291d2f:     	movl	$0x0, 0x4a4(%rbp)
142291d39:     	movb	$0x3, 0x6f7(%rbp)
142291d40:     	movq	$0x0, 0x1c0(%rbp)
142291d4b:     	xorl	%ecx, %ecx
142291d4d:     	jmp	0x142291e04 <.text+0x2290e04>
142291d52:     	nopw	%cs:(%rax,%rax)
142291d60:     	xorl	%r13d, %r13d
142291d63:     	movq	%r12, %r8
142291d66:     	addq	$0x8, %r8
142291d6a:     	movq	0x608(%rbp), %rdx
142291d71:     	movq	0x178(%rdx), %rdx
142291d78:     	movq	(%rcx,%rdx,8), %rcx
142291d7c:     	notq	%rcx
142291d7f:     	leaq	(%rcx,%rcx,2), %rcx
142291d83:     	shlq	$0x4, %rcx
142291d87:     	movq	0x10(%rax,%rcx), %r12
142291d8c:     	movq	0x28(%rax,%rcx), %r15
142291d91:     	movq	0x700(%rbp), %rdx
142291d98:     	leaq	(%rdx,%rdx,8), %rax
142291d9c:     	leaq	(%r11,%rdx,4), %rcx
142291da0:     	shrq	$0x20, %rcx
142291da4:     	movq	%rcx, 0x3b0(%rbp)
142291dab:     	movl	0x6f8(%rbp), %ecx
142291db1:     	movq	0x6d0(%rbp), %r9
142291db8:     	movl	%ecx, (%r9,%rdx,4)
142291dbc:     	movb	$0x0, 0x20(%r10,%rax,4)
142291dc2:     	movq	0x708(%rbp), %r11
142291dc9:     	movq	0x8(%r11), %rcx
142291dcd:     	cmpq	%rcx, 0x6e0(%rbp)
142291dd4:     	movq	(%r11), %rcx
142291dd7:     	sete	%r9b
142291ddb:     	andb	%cl, %r9b
142291dde:     	movq	0x2f0(%rbp), %rcx
142291de5:     	orb	%r9b, %cl
142291de8:     	andb	$0x1, %cl
142291deb:     	movq	%rcx, 0x2f0(%rbp)
142291df2:     	movq	%r14, %rcx
142291df5:     	incq	%rcx
142291df8:     	ucomiss	0x18(%r10,%rax,4), %xmm12
142291dfe:     	jbe	0x1422924f4 <.text+0x22914f4>
142291e04:     	cmpq	0x628(%rbp), %r8
142291e0b:     	je	0x1422947f0 <.text+0x22937f0>
142291e11:     	movq	%rcx, %r14
142291e14:     	movq	%rcx, 0x6e0(%rbp)
142291e1b:     	cmpb	$0x9, 0x30(%r11)
142291e20:     	movq	0x88(%rbp), %r15
142291e27:     	movq	0x608(%rbp), %r13
142291e2e:     	jne	0x142291e96 <.text+0x2290e96>
142291e30:     	movl	0x34(%r11), %edi
142291e34:     	movq	0x90(%rbp), %rax
142291e3b:     	movl	0x2b0(%rbp), %ecx
142291e41:     	movl	%ecx, (%rax)
142291e43:     	movq	0x1b8(%rbp), %rax
142291e4a:     	movq	0x30(%rax), %rsi
142291e4e:     	movq	0x40(%rax), %rbx
142291e52:     	cmpq	0x20(%rax), %rsi
142291e56:     	jne	0x142291e71 <.text+0x2290e71>
142291e58:     	movq	0x98(%rbp), %rcx
142291e5f:     	leaq	0xda9462(%rip), %rdx    # 0x14303b2c8
142291e66:     	movq	%r8, %r12
142291e69:     	callq	0x140562630 <.text+0x561630>
142291e6e:     	movq	%r12, %r8
142291e71:     	movq	0x1b8(%rbp), %rdx
142291e78:     	movq	0x28(%rdx), %rax
142291e7c:     	movq	%rsi, %rcx
142291e7f:     	shlq	$0x4, %rcx
142291e83:     	movq	%rbx, (%rax,%rcx)
142291e87:     	movl	%edi, 0x8(%rax,%rcx)
142291e8b:     	incq	%rsi
142291e8e:     	movq	%rsi, 0x30(%rdx)
142291e92:     	incq	0x40(%rdx)
142291e96:     	movq	(%r8), %rax
142291e99:     	movl	%eax, %ecx
142291e9b:     	cmpq	0x10(%r15), %rcx
142291e9f:     	jae	0x142296be8 <.text+0x2295be8>
142291ea5:     	movq	%r8, %r12
142291ea8:     	movq	0x8(%r15), %r8
142291eac:     	shrq	$0x20, %rax
142291eb0:     	leaq	(%rcx,%rcx,4), %r9
142291eb4:     	movl	$0x1, %edx
142291eb9:     	cmpl	%eax, (%r8,%r9,4)
142291ebd:     	jne	0x142296bf1 <.text+0x2295bf1>
142291ec3:     	leaq	(%r8,%r9,4), %r10
142291ec7:     	movl	0x4(%r10), %r8d
142291ecb:     	movl	$0xffffffff, %r9d       # imm = 0xFFFFFFFF
142291ed1:     	cmpq	%r9, %r8
142291ed4:     	je	0x142296bf1 <.text+0x2295bf1>
142291eda:     	xorl	%edx, %edx
142291edc:     	cmpq	%r8, 0x38(%r13)
142291ee0:     	jbe	0x142296bf8 <.text+0x2295bf8>
142291ee6:     	movq	0x28(%r13), %r9
142291eea:     	movl	%r8d, %r11d
142291eed:     	shrl	$0x6, %r11d
142291ef1:     	movq	(%r9,%r11,8), %r9
142291ef5:     	btq	%r8, %r9
142291ef9:     	jae	0x142296bf8 <.text+0x2295bf8>
142291eff:     	cmpq	%r8, 0x108(%r15)
142291f06:     	jbe	0x142291f20 <.text+0x2290f20>
142291f08:     	leaq	(%r8,%r8,4), %r8
142291f0c:     	shlq	$0x5, %r8
142291f10:     	addq	0x100(%r15), %r8
142291f17:     	jmp	0x142291f23 <.text+0x2290f23>
142291f19:     	nopl	(%rax)
142291f20:     	xorl	%r8d, %r8d
142291f23:     	movl	0xc(%r10), %eax
142291f27:     	movq	0x1a8(%r15), %rcx
142291f2e:     	leaq	(%rax,%rax,8), %r9
142291f32:     	movq	0x40(%rcx,%r9,8), %rdx
142291f37:     	movq	0x18(%rcx,%r9,8), %rax
142291f3c:     	movq	0x38(%rcx,%r9,8), %rcx
142291f41:     	movq	0x118(%r13), %r11
142291f48:     	movq	0x88(%r8), %r9
142291f4f:     	cmpq	%r11, %r9
142291f52:     	jbe	0x142291fa0 <.text+0x2290fa0>
142291f54:     	movq	0x80(%r8), %rsi
142291f5b:     	cmpq	$0x0, (%rsi,%r11,8)
142291f60:     	je	0x142291fa0 <.text+0x2290fa0>
142291f62:     	movb	$0x1, %r15b
142291f65:     	cmpq	%r11, %rdx
142291f68:     	jbe	0x142291fc0 <.text+0x2290fc0>
142291f6a:     	movq	(%rcx,%r11,8), %r11
142291f6e:     	testq	%r11, %r11
142291f71:     	je	0x142291fc0 <.text+0x2290fc0>
142291f73:     	notq	%r11
142291f76:     	leaq	(%r11,%r11,2), %r11
142291f7a:     	shlq	$0x4, %r11
142291f7e:     	movq	0x10(%rax,%r11), %r11
142291f83:     	movq	%r11, 0x4d0(%rbp)
142291f8a:     	movq	0x120(%r13), %r11
142291f91:     	cmpq	%r11, %r9
142291f94:     	jbe	0x142292030 <.text+0x2291030>
142291f9a:     	jmp	0x142291fd7 <.text+0x2290fd7>
142291f9c:     	nopl	(%rax)
142291fa0:     	movq	$0x0, 0x4d0(%rbp)
142291fab:     	xorl	%r15d, %r15d
142291fae:     	movq	0x120(%r13), %r11
142291fb5:     	cmpq	%r11, %r9
142291fb8:     	jbe	0x142292030 <.text+0x2291030>
142291fba:     	jmp	0x142291fd7 <.text+0x2290fd7>
142291fbc:     	nopl	(%rax)
142291fc0:     	movq	$0x0, 0x4d0(%rbp)
142291fcb:     	movq	0x120(%r13), %r11
142291fd2:     	cmpq	%r11, %r9
142291fd5:     	jbe	0x142292030 <.text+0x2291030>
142291fd7:     	movq	0x80(%r8), %rsi
142291fde:     	cmpq	$0x0, (%rsi,%r11,8)
142291fe3:     	je	0x142292030 <.text+0x2291030>
142291fe5:     	movb	$0x1, %sil
142291fe8:     	movq	%rsi, 0x2f8(%rbp)
142291fef:     	cmpq	%r11, %rdx
142291ff2:     	jbe	0x1422920a0 <.text+0x22910a0>
142291ff8:     	movq	(%rcx,%r11,8), %r11
142291ffc:     	testq	%r11, %r11
142291fff:     	je	0x1422920a0 <.text+0x22910a0>
142292005:     	notq	%r11
142292008:     	leaq	(%r11,%r11,2), %r11
14229200c:     	shlq	$0x4, %r11
142292010:     	movq	0x10(%rax,%r11), %r11
142292015:     	movq	%r11, 0x4d8(%rbp)
14229201c:     	movq	0x128(%r13), %r11
142292023:     	cmpq	%r11, %r9
142292026:     	jbe	0x1422920b7 <.text+0x22910b7>
14229202c:     	jmp	0x142292052 <.text+0x2291052>
14229202e:     	nop
142292030:     	movq	$0x0, 0x4d8(%rbp)
14229203b:     	movq	$0x0, 0x2f8(%rbp)
142292046:     	movq	0x128(%r13), %r11
14229204d:     	cmpq	%r11, %r9
142292050:     	jbe	0x1422920b7 <.text+0x22910b7>
142292052:     	movq	0x80(%r8), %rsi
142292059:     	cmpq	$0x0, (%rsi,%r11,8)
14229205e:     	setne	%r11b
142292062:     	movl	%r11d, 0x4a0(%rbp)
142292069:     	movq	0x130(%r13), %r11
142292070:     	cmpq	%r11, %r9
142292073:     	ja	0x1422920cd <.text+0x22910cd>
142292075:     	movl	$0x0, 0x49c(%rbp)
14229207f:     	movq	0x138(%r13), %r11
142292086:     	cmpq	%r11, %r9
142292089:     	ja	0x1422920f0 <.text+0x22910f0>
14229208b:     	movl	$0x0, 0x498(%rbp)
142292095:     	jmp	0x142292107 <.text+0x2291107>
142292097:     	nopw	(%rax,%rax)
1422920a0:     	movq	$0x0, 0x4d8(%rbp)
1422920ab:     	movq	0x128(%r13), %r11
1422920b2:     	cmpq	%r11, %r9
1422920b5:     	ja	0x142292052 <.text+0x2291052>
1422920b7:     	movl	$0x0, 0x4a0(%rbp)
1422920c1:     	movq	0x130(%r13), %r11
1422920c8:     	cmpq	%r11, %r9
1422920cb:     	jbe	0x142292075 <.text+0x2291075>
1422920cd:     	movq	0x80(%r8), %rsi
1422920d4:     	cmpq	$0x0, (%rsi,%r11,8)
1422920d9:     	setne	%r11b
1422920dd:     	movl	%r11d, 0x49c(%rbp)
1422920e4:     	movq	0x138(%r13), %r11
1422920eb:     	cmpq	%r11, %r9
1422920ee:     	jbe	0x14229208b <.text+0x229108b>
1422920f0:     	movq	0x80(%r8), %rsi
1422920f7:     	cmpq	$0x0, (%rsi,%r11,8)
1422920fc:     	setne	%r11b
142292100:     	movl	%r11d, 0x498(%rbp)
142292107:     	movl	0x10(%r10), %r10d
14229210b:     	movq	%r10, 0x700(%rbp)
142292112:     	movq	0x110(%r13), %r10
142292119:     	movq	(%rcx,%r10,8), %r10
14229211d:     	notq	%r10
142292120:     	leaq	(%r10,%r10,2), %rsi
142292124:     	shlq	$0x4, %rsi
142292128:     	movq	0x10(%rax,%rsi), %r10
14229212d:     	movq	0x20(%rax,%rsi), %r11
142292132:     	movq	0x28(%rax,%rsi), %rsi
142292137:     	movq	%rsi, 0x6d0(%rbp)
14229213e:     	movq	0x140(%r13), %rsi
142292145:     	cmpq	%rsi, %r9
142292148:     	jbe	0x1422921a0 <.text+0x22911a0>
14229214a:     	movq	0x80(%r8), %rdi
142292151:     	cmpq	$0x0, (%rdi,%rsi,8)
142292156:     	je	0x1422921a0 <.text+0x22911a0>
142292158:     	movb	$0x1, %dil
14229215b:     	movq	%rdi, 0x308(%rbp)
142292162:     	cmpq	%rsi, %rdx
142292165:     	jbe	0x1422921f0 <.text+0x22911f0>
14229216b:     	movq	(%rcx,%rsi,8), %rsi
14229216f:     	testq	%rsi, %rsi
142292172:     	je	0x1422921f0 <.text+0x22911f0>
142292178:     	notq	%rsi
14229217b:     	leaq	(%rsi,%rsi,2), %rsi
14229217f:     	shlq	$0x4, %rsi
142292183:     	movq	0x10(%rax,%rsi), %rsi
142292188:     	movq	%rsi, 0x5e0(%rbp)
14229218f:     	movq	0x148(%r13), %rsi
142292196:     	cmpq	%rsi, %r9
142292199:     	jbe	0x142292207 <.text+0x2291207>
14229219b:     	jmp	0x1422921c2 <.text+0x22911c2>
14229219d:     	nopl	(%rax)
1422921a0:     	movq	$0x0, 0x5e0(%rbp)
1422921ab:     	movq	$0x0, 0x308(%rbp)
1422921b6:     	movq	0x148(%r13), %rsi
1422921bd:     	cmpq	%rsi, %r9
1422921c0:     	jbe	0x142292207 <.text+0x2291207>
1422921c2:     	movq	0x80(%r8), %rdi
1422921c9:     	cmpq	$0x0, (%rdi,%rsi,8)
1422921ce:     	setne	%dil
1422921d2:     	movq	0x150(%r13), %rsi
1422921d9:     	cmpq	%rsi, %r9
1422921dc:     	movq	%r15, 0x80(%rbp)
1422921e3:     	ja	0x14229221c <.text+0x229121c>
1422921e5:     	jmp	0x142292270 <.text+0x2291270>
1422921ea:     	nopw	(%rax,%rax)
1422921f0:     	movq	$0x0, 0x5e0(%rbp)
1422921fb:     	movq	0x148(%r13), %rsi
142292202:     	cmpq	%rsi, %r9
142292205:     	ja	0x1422921c2 <.text+0x22911c2>
142292207:     	xorl	%edi, %edi
142292209:     	movq	0x150(%r13), %rsi
142292210:     	cmpq	%rsi, %r9
142292213:     	movq	%r15, 0x80(%rbp)
14229221a:     	jbe	0x142292270 <.text+0x2291270>
14229221c:     	movq	0x80(%r8), %rbx
142292223:     	cmpq	$0x0, (%rbx,%rsi,8)
142292228:     	je	0x142292270 <.text+0x2291270>
14229222a:     	movb	$0x1, %bl
14229222c:     	cmpq	%rsi, %rdx
14229222f:     	jbe	0x1422922e0 <.text+0x22912e0>
142292235:     	movq	(%rcx,%rsi,8), %rsi
142292239:     	testq	%rsi, %rsi
14229223c:     	je	0x1422922e0 <.text+0x22912e0>
142292242:     	notq	%rsi
142292245:     	leaq	(%rsi,%rsi,2), %rsi
142292249:     	shlq	$0x4, %rsi
14229224d:     	movq	0x10(%rax,%rsi), %rsi
142292252:     	movq	0x158(%r13), %r15
142292259:     	cmpq	%r15, %r9
14229225c:     	jbe	0x1422922ee <.text+0x22912ee>
142292262:     	jmp	0x142292280 <.text+0x2291280>
142292264:     	nopw	%cs:(%rax,%rax)
142292270:     	xorl	%esi, %esi
142292272:     	xorl	%ebx, %ebx
142292274:     	movq	0x158(%r13), %r15
14229227b:     	cmpq	%r15, %r9
14229227e:     	jbe	0x1422922ee <.text+0x22912ee>
142292280:     	movq	0x80(%r8), %r13
142292287:     	cmpq	$0x0, (%r13,%r15,8)
14229228d:     	je	0x14229233d <.text+0x229133d>
142292293:     	movb	$0x1, %r13b
142292296:     	movq	%r13, 0x300(%rbp)
14229229d:     	cmpq	%r15, %rdx
1422922a0:     	movq	0x608(%rbp), %r13
1422922a7:     	jbe	0x142292320 <.text+0x2291320>
1422922a9:     	movq	(%rcx,%r15,8), %r15
1422922ad:     	testq	%r15, %r15
1422922b0:     	je	0x142292320 <.text+0x2291320>
1422922b2:     	notq	%r15
1422922b5:     	leaq	(%r15,%r15,2), %r15
1422922b9:     	shlq	$0x4, %r15
1422922bd:     	movq	0x10(%rax,%r15), %r15
1422922c2:     	movq	%r15, 0x4f0(%rbp)
1422922c9:     	movq	0x160(%r13), %r15
1422922d0:     	cmpq	%r15, %r9
1422922d3:     	ja	0x142292370 <.text+0x2291370>
1422922d9:     	jmp	0x1422923d0 <.text+0x22913d0>
1422922de:     	nop
1422922e0:     	xorl	%esi, %esi
1422922e2:     	movq	0x158(%r13), %r15
1422922e9:     	cmpq	%r15, %r9
1422922ec:     	ja	0x142292280 <.text+0x2291280>
1422922ee:     	movq	$0x0, 0x4f0(%rbp)
1422922f9:     	movq	$0x0, 0x300(%rbp)
142292304:     	movq	0x160(%r13), %r15
14229230b:     	cmpq	%r15, %r9
14229230e:     	ja	0x142292370 <.text+0x2291370>
142292310:     	jmp	0x1422923d0 <.text+0x22913d0>
142292315:     	nopw	%cs:(%rax,%rax)
142292320:     	movq	$0x0, 0x4f0(%rbp)
14229232b:     	movq	0x160(%r13), %r15
142292332:     	cmpq	%r15, %r9
142292335:     	jbe	0x1422923d0 <.text+0x22913d0>
14229233b:     	jmp	0x142292370 <.text+0x2291370>
14229233d:     	movq	$0x0, 0x4f0(%rbp)
142292348:     	movq	$0x0, 0x300(%rbp)
142292353:     	movq	0x608(%rbp), %r13
14229235a:     	movq	0x160(%r13), %r15
142292361:     	cmpq	%r15, %r9
142292364:     	jbe	0x1422923d0 <.text+0x22913d0>
142292366:     	nopw	%cs:(%rax,%rax)
142292370:     	movq	0x80(%r8), %r13
142292377:     	cmpq	$0x0, (%r13,%r15,8)
14229237d:     	je	0x1422923d0 <.text+0x22913d0>
14229237f:     	movb	$0x1, %r13b
142292382:     	movq	%r13, 0x3a8(%rbp)
142292389:     	cmpq	%r15, %rdx
14229238c:     	jbe	0x142292400 <.text+0x2291400>
14229238e:     	movq	(%rcx,%r15,8), %r15
142292392:     	testq	%r15, %r15
142292395:     	je	0x142292400 <.text+0x2291400>
142292397:     	notq	%r15
14229239a:     	leaq	(%r15,%r15,2), %r15
14229239e:     	shlq	$0x4, %r15
1422923a2:     	movq	0x10(%rax,%r15), %r15
1422923a7:     	movq	%r15, 0x4e8(%rbp)
1422923ae:     	movq	0x608(%rbp), %r15
1422923b5:     	movq	0x168(%r15), %r15
1422923bc:     	cmpq	%r15, %r9
1422923bf:     	jbe	0x142292480 <.text+0x2291480>
1422923c5:     	jmp	0x14229241e <.text+0x229141e>
1422923c7:     	nopw	(%rax,%rax)
1422923d0:     	movq	$0x0, 0x4e8(%rbp)
1422923db:     	movq	$0x0, 0x3a8(%rbp)
1422923e6:     	movq	0x608(%rbp), %r15
1422923ed:     	movq	0x168(%r15), %r15
1422923f4:     	cmpq	%r15, %r9
1422923f7:     	jbe	0x142292480 <.text+0x2291480>
1422923fd:     	jmp	0x14229241e <.text+0x229141e>
1422923ff:     	nop
142292400:     	movq	$0x0, 0x4e8(%rbp)
14229240b:     	movq	0x608(%rbp), %r15
142292412:     	movq	0x168(%r15), %r15
142292419:     	cmpq	%r15, %r9
14229241c:     	jbe	0x142292480 <.text+0x2291480>
14229241e:     	movq	0x80(%r8), %r8
142292425:     	cmpq	$0x0, (%r8,%r15,8)
14229242a:     	je	0x142292480 <.text+0x2291480>
14229242c:     	movb	$0x1, %r8b
14229242f:     	movq	%r8, 0x3a0(%rbp)
142292436:     	cmpq	%r15, %rdx
142292439:     	jbe	0x1422924b0 <.text+0x22914b0>
14229243b:     	movq	(%rcx,%r15,8), %r8
14229243f:     	testq	%r8, %r8
142292442:     	je	0x1422924b0 <.text+0x22914b0>
142292444:     	notq	%r8
142292447:     	leaq	(%r8,%r8,2), %r8
14229244b:     	shlq	$0x4, %r8
14229244f:     	movq	0x10(%rax,%r8), %r8
142292454:     	movq	%r8, 0x4e0(%rbp)
14229245b:     	movq	0x608(%rbp), %r8
142292462:     	movq	0x170(%r8), %r8
142292469:     	cmpq	%r8, %rdx
14229246c:     	jbe	0x142291d60 <.text+0x2290d60>
142292472:     	jmp	0x1422924d2 <.text+0x22914d2>
142292474:     	nopw	%cs:(%rax,%rax)
142292480:     	movq	$0x0, 0x4e0(%rbp)
14229248b:     	movq	$0x0, 0x3a0(%rbp)
142292496:     	movq	0x608(%rbp), %r8
14229249d:     	movq	0x170(%r8), %r8
1422924a4:     	cmpq	%r8, %rdx
1422924a7:     	jbe	0x142291d60 <.text+0x2290d60>
1422924ad:     	jmp	0x1422924d2 <.text+0x22914d2>
1422924af:     	nop
1422924b0:     	movq	$0x0, 0x4e0(%rbp)
1422924bb:     	movq	0x608(%rbp), %r8
1422924c2:     	movq	0x170(%r8), %r8
1422924c9:     	cmpq	%r8, %rdx
1422924cc:     	jbe	0x142291d60 <.text+0x2290d60>
1422924d2:     	movq	(%rcx,%r8,8), %rdx
1422924d6:     	testq	%rdx, %rdx
1422924d9:     	je	0x142291d60 <.text+0x2290d60>
1422924df:     	notq	%rdx
1422924e2:     	leaq	(%rdx,%rdx,2), %rdx
1422924e6:     	shlq	$0x4, %rdx
1422924ea:     	movq	0x10(%rax,%rdx), %r13
1422924ef:     	jmp	0x142291d63 <.text+0x2290d63>
1422924f4:     	movb	%r9b, 0x717(%rbp)
1422924fb:     	movq	%r15, 0x3c0(%rbp)
142292502:     	movq	%r12, 0x4f8(%rbp)
142292509:     	movq	%rcx, 0x6c0(%rbp)
142292510:     	movq	%r8, 0x6b8(%rbp)
142292517:     	leaq	(%r10,%rax,4), %rax
14229251b:     	movq	%rax, 0x6b0(%rbp)
142292522:     	movq	%rdx, %rax
142292525:     	shlq	$0x4, %rax
142292529:     	addq	%rax, %r13
14229252c:     	leaq	0x530(%rbp), %r14
142292533:     	movq	%r14, %rcx
142292536:     	callq	0x1409a8fb0 <.text+0x9a7fb0>
14229253b:     	movl	$0xc, 0x638(%rbp)
142292545:     	movl	0x5c(%rbp), %eax
142292548:     	leaq	0x63c(%rbp), %rcx
14229254f:     	movl	%eax, 0x10(%rcx)
142292552:     	movups	0x4c(%rbp), %xmm0
142292556:     	movups	%xmm0, (%rcx)
142292559:     	leaq	0x3e8(%rbp), %r12
142292560:     	movq	%r12, %rcx
142292563:     	movq	%r14, %rdx
142292566:     	leaq	0x638(%rbp), %r15
14229256d:     	movq	%r15, %r8
142292570:     	callq	0x1409a8fd0 <.text+0x9a7fd0>
142292575:     	movl	$0xd, 0x530(%rbp)
14229257f:     	movl	0x48(%rbp), %eax
142292582:     	leaq	0x534(%rbp), %rcx
142292589:     	movl	%eax, 0x10(%rcx)
14229258c:     	movups	0x38(%rbp), %xmm0
142292590:     	movups	%xmm0, (%rcx)
142292593:     	leaq	-0x50(%rbp), %rcx
142292597:     	movq	%r12, %rdx
14229259a:     	movq	%r14, %r8
14229259d:     	callq	0x1409a9020 <.text+0x9a8020>
1422925a2:     	testb	%bl, %bl
1422925a4:     	movq	%r13, 0x630(%rbp)
1422925ab:     	je	0x142292734 <.text+0x2291734>
1422925b1:     	leaq	0x6e0(%rbp), %rax
1422925b8:     	movq	%rax, 0x3e8(%rbp)
1422925bf:     	leaq	-0x21ca106(%rip), %rax  # 0x1400c84c0 <.text+0xc74c0>
1422925c6:     	movq	%rax, 0x3f0(%rbp)
1422925cd:     	leaq	0x6a8(%rbp), %rcx
1422925d4:     	movq	%rcx, 0x3f8(%rbp)
1422925db:     	movq	%rax, 0x400(%rbp)
1422925e2:     	leaq	0xda913f(%rip), %rax    # 0x14303b728
1422925e9:     	movq	%rax, 0x530(%rbp)
1422925f0:     	movq	$0x2, 0x538(%rbp)
1422925fb:     	movq	%r12, 0x540(%rbp)
142292602:     	movq	$0x2, 0x548(%rbp)
14229260d:     	movq	$0x0, 0x550(%rbp)
142292618:     	movq	%r15, %rcx
14229261b:     	movq	%r14, %rdx
14229261e:     	callq	0x140d79f90 <.text+0xd78f90>
142292623:     	movsd	(%r13), %xmm0
142292629:     	movsd	0x8(%r13), %xmm1
14229262f:     	movddup	%xmm0, %xmm2            # xmm2 = xmm0[0,0]
142292633:     	mulps	%xmm13, %xmm1
142292637:     	movddup	%xmm1, %xmm3            # xmm3 = xmm1[0,0]
14229263b:     	addps	%xmm2, %xmm3
14229263e:     	subps	%xmm1, %xmm0
142292641:     	shufps	$0xe4, %xmm3, %xmm0     # xmm0 = xmm0[0,1],xmm3[2,3]
142292645:     	movups	%xmm0, 0x3f0(%rbp)
14229264c:     	movl	$0x0, 0x3ec(%rbp)
142292656:     	movb	$0x3, 0x3e8(%rbp)
14229265d:     	movb	$0x0, 0x614(%rbp)
142292664:     	movb	$0x1, 0x6a4(%rbp)
14229266b:     	movq	%r14, %rcx
14229266e:     	movq	%r12, %rdx
142292671:     	leaq	0x614(%rbp), %r8
142292678:     	callq	0x1409a8e10 <.text+0x9a7e10>
14229267d:     	movq	%r12, %rbx
142292680:     	movb	$0x0, 0x6a4(%rbp)
142292687:     	movq	0x650(%rbp), %rcx
14229268e:     	movq	%r15, %rdx
142292691:     	movq	%r14, %r8
142292694:     	callq	0x1422bd120 <.text+0x22bc120>
142292699:     	movq	%rax, 0x3e8(%rbp)
1422926a0:     	movq	%rdx, 0x3f0(%rbp)
1422926a7:     	movq	%rbx, %rcx
1422926aa:     	callq	0x14092e680 <.text+0x92d680>
1422926af:     	testb	%al, %al
1422926b1:     	movl	0x6ec(%rbp), %r13d
1422926b8:     	je	0x142292749 <.text+0x2291749>
1422926be:     	movq	0x6a8(%rbp), %rax
1422926c5:     	movq	%rax, 0x658(%rbp)
1422926cc:     	movl	0x6f8(%rbp), %eax
1422926d2:     	movq	0x700(%rbp), %rcx
1422926d9:     	movq	0x6d0(%rbp), %rdx
1422926e0:     	movl	%eax, (%rdx,%rcx,4)
1422926e3:     	movq	0x6b0(%rbp), %rax
1422926ea:     	movb	$0x1, 0x20(%rax)
1422926ee:     	movl	$0x1, %eax
1422926f3:     	movq	0x700(%rbp), %rcx
1422926fa:     	leaq	(%rcx,%rcx,2), %rcx
1422926fe:     	leaq	(%rsi,%rcx,4), %rcx
142292702:     	movzbl	(%rcx), %edx
142292705:     	movb	%dl, 0x6de(%rbp)
14229270b:     	movzbl	0x1(%rcx), %edx
14229270f:     	movb	%dl, 0x6df(%rbp)
142292715:     	movzwl	0x2(%rcx), %edx
142292719:     	movw	%dx, 0x626(%rbp)
142292720:     	movl	0x4(%rcx), %edx
142292723:     	movl	%edx, 0x5fc(%rbp)
142292729:     	movl	0x8(%rcx), %ecx
14229272c:     	movl	%ecx, 0x600(%rbp)
142292732:     	jmp	0x142292764 <.text+0x2291764>
142292734:     	xorl	%r14d, %r14d
142292737:     	movq	%r12, %rbx
14229273a:     	movq	%r15, %rsi
14229273d:     	movl	0x6ec(%rbp), %r13d
142292744:     	jmp	0x1422927d2 <.text+0x22917d2>
142292749:     	cmpq	$0x0, 0x680(%rbp)
142292751:     	movzbl	0x717(%rbp), %r8d
142292759:     	je	0x142292e3a <.text+0x2291e3a>
14229275f:     	movl	$0x1, %eax
142292764:     	movq	%rax, 0x680(%rbp)
14229276b:     	movq	%rbx, %rcx
14229276e:     	callq	0x14092e5f0 <.text+0x92d5f0>
142292773:     	movb	$0x1, %r14b
142292776:     	movb	$0x1, %cl
142292778:     	testb	%al, %al
14229277a:     	leaq	0x638(%rbp), %rsi
142292781:     	je	0x142292788 <.text+0x2291788>
142292783:     	movl	%ecx, %r13d
142292786:     	jmp	0x1422927d2 <.text+0x22917d2>
142292788:     	movq	0x660(%rbp), %rax
14229278f:     	movl	0x98(%rax), %ecx
142292795:     	movss	0x9c(%rax), %xmm1
14229279d:     	callq	0x14093fa90 <.text+0x93ea90>
1422927a2:     	testb	%al, %al
1422927a4:     	je	0x1422927cf <.text+0x22917cf>
1422927a6:     	movq	0x708(%rbp), %rax
1422927ad:     	cmpl	$0x1, (%rax)
1422927b0:     	jne	0x1422927cf <.text+0x22917cf>
1422927b2:     	movq	0x8(%rax), %rax
1422927b6:     	cmpq	0x6e0(%rbp), %rax
1422927bd:     	sete	%r14b
1422927c1:     	movzbl	%r13b, %ecx
1422927c5:     	movl	$0x1, %eax
1422927ca:     	cmovel	%eax, %ecx
1422927cd:     	jmp	0x142292783 <.text+0x2291783>
1422927cf:     	xorl	%r14d, %r14d
1422927d2:     	movq	0x700(%rbp), %rax
1422927d9:     	shlq	$0x5, %rax
1422927dd:     	addq	%rax, 0x4f8(%rbp)
1422927e4:     	testb	%dil, %dil
1422927e7:     	je	0x142292951 <.text+0x2291951>
1422927ed:     	leaq	0x6e0(%rbp), %rax
1422927f4:     	movq	%rax, 0x3e8(%rbp)
1422927fb:     	leaq	-0x21ca342(%rip), %rax  # 0x1400c84c0 <.text+0xc74c0>
142292802:     	movq	%rax, 0x3f0(%rbp)
142292809:     	leaq	0x6a8(%rbp), %rcx
142292810:     	movq	%rcx, 0x3f8(%rbp)
142292817:     	movq	%rax, 0x400(%rbp)
14229281e:     	leaq	0xda8f33(%rip), %rax    # 0x14303b758
142292825:     	movq	%rax, 0x530(%rbp)
14229282c:     	movq	$0x2, 0x538(%rbp)
142292837:     	movq	%rbx, 0x540(%rbp)
14229283e:     	movq	$0x2, 0x548(%rbp)
142292849:     	movq	$0x0, 0x550(%rbp)
142292854:     	movq	%rsi, %rcx
142292857:     	movq	%rsi, %rdi
14229285a:     	leaq	0x530(%rbp), %rsi
142292861:     	movq	%rsi, %rdx
142292864:     	callq	0x140d79f90 <.text+0xd78f90>
142292869:     	movq	0x630(%rbp), %rax
142292870:     	movsd	(%rax), %xmm0
142292874:     	movsd	0x8(%rax), %xmm1
142292879:     	movddup	%xmm0, %xmm2            # xmm2 = xmm0[0,0]
14229287d:     	mulps	%xmm13, %xmm1
142292881:     	movddup	%xmm1, %xmm3            # xmm3 = xmm1[0,0]
142292885:     	addps	%xmm2, %xmm3
142292888:     	subps	%xmm1, %xmm0
14229288b:     	shufps	$0xe4, %xmm3, %xmm0     # xmm0 = xmm0[0,1],xmm3[2,3]
14229288f:     	movups	%xmm0, 0x3f0(%rbp)
142292896:     	movl	$0x0, 0x3ec(%rbp)
1422928a0:     	movb	$0x3, 0x3e8(%rbp)
1422928a7:     	movb	$0x0, 0x614(%rbp)
1422928ae:     	movb	$0x1, 0x6a3(%rbp)
1422928b5:     	movq	%rsi, %rcx
1422928b8:     	movq	%rbx, %rdx
1422928bb:     	leaq	0x614(%rbp), %r8
1422928c2:     	callq	0x1409a8e10 <.text+0x9a7e10>
1422928c7:     	movb	$0x0, 0x6a3(%rbp)
1422928ce:     	movq	0x650(%rbp), %rcx
1422928d5:     	movq	%rdi, %rdx
1422928d8:     	movq	%rsi, %r8
1422928db:     	callq	0x1422bd120 <.text+0x22bc120>
1422928e0:     	movq	%rax, 0x3e8(%rbp)
1422928e7:     	movq	%rdx, 0x3f0(%rbp)
1422928ee:     	movq	0x708(%rbp), %rax
1422928f5:     	movzbl	0x89(%rax), %ebx
1422928fc:     	leaq	0x3e8(%rbp), %rcx
142292903:     	callq	0x14092e680 <.text+0x92d680>
142292908:     	testb	%al, %al
14229290a:     	je	0x142292ade <.text+0x2291ade>
142292910:     	movq	0x6a8(%rbp), %rax
142292917:     	movq	%rax, 0x658(%rbp)
14229291e:     	movl	0x6f8(%rbp), %eax
142292924:     	movq	0x700(%rbp), %rcx
14229292b:     	movq	0x6d0(%rbp), %rdx
142292932:     	movl	%eax, (%rdx,%rcx,4)
142292935:     	movq	0x6b0(%rbp), %rax
14229293c:     	movb	$0x1, 0x20(%rax)
142292940:     	xorb	$0x1, %bl
142292943:     	movb	$0x1, %al
142292945:     	movq	%rax, 0x330(%rbp)
14229294c:     	jmp	0x142292aec <.text+0x2291aec>
142292951:     	movl	%r13d, %r12d
142292954:     	leaq	0x530(%rbp), %rdi
14229295b:     	movq	0x6b8(%rbp), %r8
142292962:     	movq	0x6c0(%rbp), %rcx
142292969:     	movq	0x3a8(%rbp), %rax
142292970:     	testb	%al, %al
142292972:     	je	0x142292ca7 <.text+0x2291ca7>
142292978:     	leaq	0x6e0(%rbp), %rax
14229297f:     	movq	%rax, 0x3e8(%rbp)
142292986:     	leaq	-0x21ca4cd(%rip), %rax  # 0x1400c84c0 <.text+0xc74c0>
14229298d:     	movq	%rax, 0x3f0(%rbp)
142292994:     	leaq	0x6a8(%rbp), %rcx
14229299b:     	movq	%rcx, 0x3f8(%rbp)
1422929a2:     	movq	%rax, 0x400(%rbp)
1422929a9:     	leaq	0xda8e18(%rip), %rax    # 0x14303b7c8
1422929b0:     	movq	%rax, 0x530(%rbp)
1422929b7:     	movq	$0x2, 0x538(%rbp)
1422929c2:     	movq	%rbx, 0x540(%rbp)
1422929c9:     	movq	$0x2, 0x548(%rbp)
1422929d4:     	movq	$0x0, 0x550(%rbp)
1422929df:     	movq	%rsi, %rcx
1422929e2:     	movq	%rdi, %rdx
1422929e5:     	callq	0x140d79f90 <.text+0xd78f90>
1422929ea:     	movq	0x630(%rbp), %rax
1422929f1:     	movsd	(%rax), %xmm0
1422929f5:     	movsd	0x8(%rax), %xmm1
1422929fa:     	movddup	%xmm0, %xmm2            # xmm2 = xmm0[0,0]
1422929fe:     	mulps	%xmm13, %xmm1
142292a02:     	movddup	%xmm1, %xmm3            # xmm3 = xmm1[0,0]
142292a06:     	addps	%xmm2, %xmm3
142292a09:     	subps	%xmm1, %xmm0
142292a0c:     	shufps	$0xe4, %xmm3, %xmm0     # xmm0 = xmm0[0,1],xmm3[2,3]
142292a10:     	movups	%xmm0, 0x3f0(%rbp)
142292a17:     	movl	$0x0, 0x3ec(%rbp)
142292a21:     	movb	$0x3, 0x3e8(%rbp)
142292a28:     	movb	$0x0, 0x614(%rbp)
142292a2f:     	movb	$0x1, 0x6a2(%rbp)
142292a36:     	movq	%rdi, %rcx
142292a39:     	movq	%rbx, %rdx
142292a3c:     	leaq	0x614(%rbp), %r8
142292a43:     	callq	0x1409a8e10 <.text+0x9a7e10>
142292a48:     	movq	0x650(%rbp), %r14
142292a4f:     	movq	0x680(%rbp), %r13
142292a56:     	movb	$0x0, 0x6a2(%rbp)
142292a5d:     	movq	%r14, %rcx
142292a60:     	movq	%rsi, %rdx
142292a63:     	movq	%rdi, %r8
142292a66:     	callq	0x1422bd120 <.text+0x22bc120>
142292a6b:     	movq	%rax, 0x3e8(%rbp)
142292a72:     	movq	%rdx, 0x3f0(%rbp)
142292a79:     	movq	%rbx, %rcx
142292a7c:     	callq	0x14092e680 <.text+0x92d680>
142292a81:     	testb	%al, %al
142292a83:     	je	0x142292bb7 <.text+0x2291bb7>
142292a89:     	movq	0x6a8(%rbp), %rax
142292a90:     	movq	%rax, 0x658(%rbp)
142292a97:     	movl	0x6f8(%rbp), %eax
142292a9d:     	movq	0x700(%rbp), %rcx
142292aa4:     	movq	0x6d0(%rbp), %rdx
142292aab:     	movl	%eax, (%rdx,%rcx,4)
142292aae:     	movq	0x6b0(%rbp), %rax
142292ab5:     	movb	$0x1, 0x20(%rax)
142292ab9:     	movl	$0x1, %eax
142292abe:     	movq	0x700(%rbp), %rcx
142292ac5:     	movq	0x4e8(%rbp), %rdx
142292acc:     	movzbl	(%rdx,%rcx), %ecx
142292ad0:     	movb	%cl, 0x6f6(%rbp)
142292ad6:     	movq	%rax, %r13
142292ad9:     	jmp	0x142292f6c <.text+0x2291f6c>
142292ade:     	cmpq	$0x0, 0x680(%rbp)
142292ae6:     	je	0x142292ea4 <.text+0x2291ea4>
142292aec:     	movl	$0x1, %eax
142292af1:     	movq	%rax, 0x680(%rbp)
142292af8:     	leaq	0x3e8(%rbp), %rcx
142292aff:     	callq	0x14092e5f0 <.text+0x92d5f0>
142292b04:     	movb	$0x1, %sil
142292b07:     	movb	$0x1, %r12b
142292b0a:     	testb	%al, %al
142292b0c:     	je	0x142292b61 <.text+0x2291b61>
142292b0e:     	movzbl	0x13a20f4(%rip), %eax   # 0x143634c09
142292b15:     	testb	%bl, %bl
142292b17:     	je	0x142292be7 <.text+0x2291be7>
142292b1d:     	movl	$0x1f, %ecx
142292b22:     	movl	$0x1, %edx
142292b27:     	callq	0x140613c10 <.text+0x612c10>
142292b2c:     	testq	%rax, %rax
142292b2f:     	leaq	0x3e8(%rbp), %rbx
142292b36:     	je	0x142296c62 <.text+0x2295c62>
142292b3c:     	movq	%rax, %rdi
142292b3f:     	movl	%esi, %r15d
142292b42:     	movups	0xda8c5f(%rip), %xmm0   # 0x14303b7a8
142292b49:     	movups	%xmm0, 0xf(%rax)
142292b4d:     	movups	0xda8c45(%rip), %xmm0   # 0x14303b799
142292b54:     	movups	%xmm0, (%rax)
142292b57:     	movl	$0x1f, %esi
142292b5c:     	jmp	0x142292c2a <.text+0x2291c2a>
142292b61:     	movq	0x660(%rbp), %rax
142292b68:     	movl	0x98(%rax), %ecx
142292b6e:     	movss	0x9c(%rax), %xmm1
142292b76:     	callq	0x14093fa90 <.text+0x93ea90>
142292b7b:     	testb	%al, %al
142292b7d:     	je	0x142292bd3 <.text+0x2291bd3>
142292b7f:     	movq	0x708(%rbp), %rax
142292b86:     	cmpl	$0x1, (%rax)
142292b89:     	jne	0x142292bd3 <.text+0x2291bd3>
142292b8b:     	movq	0x8(%rax), %rax
142292b8f:     	cmpq	0x6e0(%rbp), %rax
142292b96:     	sete	%sil
142292b9a:     	movzbl	%r13b, %r12d
142292b9e:     	movl	$0x1, %eax
142292ba3:     	cmovel	%eax, %r12d
142292ba7:     	movzbl	0x13a205b(%rip), %eax   # 0x143634c09
142292bae:     	testb	%bl, %bl
142292bb0:     	je	0x142292be7 <.text+0x2291be7>
142292bb2:     	jmp	0x142292b1d <.text+0x2291b1d>
142292bb7:     	testq	%r13, %r13
142292bba:     	movzbl	0x717(%rbp), %r8d
142292bc2:     	je	0x142292f0d <.text+0x2291f0d>
142292bc8:     	movl	$0x1, %r13d
142292bce:     	jmp	0x142292f6c <.text+0x2291f6c>
142292bd3:     	xorl	%esi, %esi
142292bd5:     	movl	%r13d, %r12d
142292bd8:     	movzbl	0x13a202a(%rip), %eax   # 0x143634c09
142292bdf:     	testb	%bl, %bl
142292be1:     	jne	0x142292b1d <.text+0x2291b1d>
142292be7:     	movl	$0x21, %ecx
142292bec:     	movl	$0x1, %edx
142292bf1:     	callq	0x140613c10 <.text+0x612c10>
142292bf6:     	testq	%rax, %rax
142292bf9:     	leaq	0x3e8(%rbp), %rbx
142292c00:     	je	0x142296ccd <.text+0x2295ccd>
142292c06:     	movq	%rax, %rdi
142292c09:     	movl	%esi, %r15d
142292c0c:     	movups	0xda8b75(%rip), %xmm0   # 0x14303b788
142292c13:     	movups	%xmm0, 0x10(%rax)
142292c17:     	movups	0xda8b5a(%rip), %xmm0   # 0x14303b778
142292c1e:     	movups	%xmm0, (%rax)
142292c21:     	movb	$0x65, 0x20(%rax)
142292c25:     	movl	$0x21, %esi
142292c2a:     	movq	0x6b8(%rbp), %r8
142292c31:     	movq	0x6c0(%rbp), %rcx
142292c38:     	movl	0x6f8(%rbp), %eax
142292c3e:     	movq	0x700(%rbp), %rdx
142292c45:     	movq	0x3c0(%rbp), %r9
142292c4c:     	movl	%eax, (%r9,%rdx,4)
142292c50:     	movq	0x4f8(%rbp), %r14
142292c57:     	movq	(%r14), %rdx
142292c5a:     	testq	%rdx, %rdx
142292c5d:     	je	0x142292c7c <.text+0x2291c7c>
142292c5f:     	movq	0x8(%r14), %rcx
142292c63:     	movl	$0x1, %r8d
142292c69:     	callq	0x140613c20 <.text+0x612c20>
142292c6e:     	movq	0x6c0(%rbp), %rcx
142292c75:     	movq	0x6b8(%rbp), %r8
142292c7c:     	movq	%rsi, (%r14)
142292c7f:     	movq	%rdi, 0x8(%r14)
142292c83:     	movq	%rsi, 0x10(%r14)
142292c87:     	leaq	0x530(%rbp), %rdi
142292c8e:     	leaq	0x638(%rbp), %rsi
142292c95:     	movl	%r15d, %r14d
142292c98:     	movq	0x3a8(%rbp), %rax
142292c9f:     	testb	%al, %al
142292ca1:     	jne	0x142292978 <.text+0x2291978>
142292ca7:     	movq	0x680(%rbp), %r13
142292cae:     	movq	0x3a0(%rbp), %rax
142292cb5:     	testb	%al, %al
142292cb7:     	je	0x142292fea <.text+0x2291fea>
142292cbd:     	leaq	0x6e0(%rbp), %rax
142292cc4:     	movq	%rax, 0x3e8(%rbp)
142292ccb:     	leaq	-0x21ca812(%rip), %rax  # 0x1400c84c0 <.text+0xc74c0>
142292cd2:     	movq	%rax, 0x3f0(%rbp)
142292cd9:     	leaq	0x6a8(%rbp), %rcx
142292ce0:     	movq	%rcx, 0x3f8(%rbp)
142292ce7:     	movq	%rax, 0x400(%rbp)
142292cee:     	leaq	0xda8b0b(%rip), %rax    # 0x14303b800
142292cf5:     	movq	%rax, 0x530(%rbp)
142292cfc:     	movq	$0x2, 0x538(%rbp)
142292d07:     	movq	%rbx, 0x540(%rbp)
142292d0e:     	movq	$0x2, 0x548(%rbp)
142292d19:     	movq	$0x0, 0x550(%rbp)
142292d24:     	movq	%rsi, %rcx
142292d27:     	movq	%rdi, %rdx
142292d2a:     	callq	0x140d79f90 <.text+0xd78f90>
142292d2f:     	movq	0x630(%rbp), %rax
142292d36:     	movsd	(%rax), %xmm0
142292d3a:     	movsd	0x8(%rax), %xmm1
142292d3f:     	movddup	%xmm0, %xmm2            # xmm2 = xmm0[0,0]
142292d43:     	mulps	%xmm13, %xmm1
142292d47:     	movddup	%xmm1, %xmm3            # xmm3 = xmm1[0,0]
142292d4b:     	addps	%xmm2, %xmm3
142292d4e:     	subps	%xmm1, %xmm0
142292d51:     	shufps	$0xe4, %xmm3, %xmm0     # xmm0 = xmm0[0,1],xmm3[2,3]
142292d55:     	movups	%xmm0, 0x3f0(%rbp)
142292d5c:     	movl	$0x0, 0x3ec(%rbp)
142292d66:     	movb	$0x3, 0x3e8(%rbp)
142292d6d:     	movb	$0x0, 0x614(%rbp)
142292d74:     	movb	$0x1, 0x6a1(%rbp)
142292d7b:     	movq	%rdi, %rcx
142292d7e:     	movq	%rbx, %rdx
142292d81:     	leaq	0x614(%rbp), %r8
142292d88:     	callq	0x1409a8e10 <.text+0x9a7e10>
142292d8d:     	movb	$0x0, 0x6a1(%rbp)
142292d94:     	movq	0x650(%rbp), %rcx
142292d9b:     	movq	%rsi, %rdx
142292d9e:     	movq	%rdi, %r8
142292da1:     	callq	0x1422bd120 <.text+0x22bc120>
142292da6:     	movq	%rax, 0x3e8(%rbp)
142292dad:     	movq	%rdx, 0x3f0(%rbp)
142292db4:     	movq	%rbx, %rcx
142292db7:     	callq	0x14092e680 <.text+0x92d680>
142292dbc:     	testb	%al, %al
142292dbe:     	je	0x142292e1e <.text+0x2291e1e>
142292dc0:     	movq	0x6a8(%rbp), %rax
142292dc7:     	movq	%rax, 0x658(%rbp)
142292dce:     	movl	0x6f8(%rbp), %eax
142292dd4:     	movq	0x700(%rbp), %rcx
142292ddb:     	movq	0x6d0(%rbp), %rdx
142292de2:     	movl	%eax, (%rdx,%rcx,4)
142292de5:     	movq	0x6b0(%rbp), %rax
142292dec:     	movb	$0x1, 0x20(%rax)
142292df0:     	movl	$0x1, %eax
142292df5:     	movq	0x700(%rbp), %rcx
142292dfc:     	movq	0x4e0(%rbp), %rdx
142292e03:     	movl	(%rdx,%rcx,4), %ecx
142292e06:     	movl	%ecx, 0x604(%rbp)
142292e0c:     	movl	$0x1, 0x4a4(%rbp)
142292e16:     	movq	%rax, %r13
142292e19:     	jmp	0x1422931c3 <.text+0x22921c3>
142292e1e:     	testq	%r13, %r13
142292e21:     	movzbl	0x717(%rbp), %r8d
142292e29:     	je	0x142293164 <.text+0x2292164>
142292e2f:     	movl	$0x1, %r13d
142292e35:     	jmp	0x1422931c3 <.text+0x22921c3>
142292e3a:     	cmpb	$0x0, 0x6cc(%rbp)
142292e41:     	je	0x142292e85 <.text+0x2291e85>
142292e43:     	movq	0x708(%rbp), %rax
142292e4a:     	testb	$0x1, (%rax)
142292e4d:     	je	0x142292e85 <.text+0x2291e85>
142292e4f:     	movq	0x708(%rbp), %rax
142292e56:     	movq	0x8(%rax), %rax
142292e5a:     	cmpq	%rax, 0x6e0(%rbp)
142292e61:     	jne	0x142292e85 <.text+0x2291e85>
142292e63:     	movl	0x6f8(%rbp), %eax
142292e69:     	movq	0x700(%rbp), %rcx
142292e70:     	movq	0x6d0(%rbp), %rdx
142292e77:     	movl	%eax, (%rdx,%rcx,4)
142292e7a:     	movq	0x6b0(%rbp), %rax
142292e81:     	movb	$0x1, 0x20(%rax)
142292e85:     	movl	$0x0, %eax
142292e8a:     	movl	$0x0, %ecx
142292e8f:     	movq	%rcx, 0x680(%rbp)
142292e96:     	testb	%r8b, %r8b
142292e99:     	jne	0x1422926f3 <.text+0x22916f3>
142292e9f:     	jmp	0x14229276b <.text+0x229176b>
142292ea4:     	cmpb	$0x0, 0x6cc(%rbp)
142292eab:     	movq	0x708(%rbp), %rax
142292eb2:     	je	0x142292ee8 <.text+0x2291ee8>
142292eb4:     	testb	$0x1, (%rax)
142292eb7:     	je	0x142292ee8 <.text+0x2291ee8>
142292eb9:     	movq	0x8(%rax), %rax
142292ebd:     	cmpq	%rax, 0x6e0(%rbp)
142292ec4:     	jne	0x142292ee8 <.text+0x2291ee8>
142292ec6:     	movl	0x6f8(%rbp), %eax
142292ecc:     	movq	0x700(%rbp), %rcx
142292ed3:     	movq	0x6d0(%rbp), %rdx
142292eda:     	movl	%eax, (%rdx,%rcx,4)
142292edd:     	movq	0x6b0(%rbp), %rax
142292ee4:     	movb	$0x1, 0x20(%rax)
142292ee8:     	cmpb	$0x0, 0x717(%rbp)
142292eef:     	je	0x142292efd <.text+0x2291efd>
142292ef1:     	xorb	$0x1, %bl
142292ef4:     	movb	$0x1, %al
142292ef6:     	movq	%rax, 0x330(%rbp)
142292efd:     	movq	$0x0, 0x680(%rbp)
142292f08:     	jmp	0x142292af8 <.text+0x2291af8>
142292f0d:     	cmpb	$0x0, 0x6cc(%rbp)
142292f14:     	je	0x142292f58 <.text+0x2291f58>
142292f16:     	movq	0x708(%rbp), %rax
142292f1d:     	testb	$0x1, (%rax)
142292f20:     	je	0x142292f58 <.text+0x2291f58>
142292f22:     	movq	0x708(%rbp), %rax
142292f29:     	movq	0x8(%rax), %rax
142292f2d:     	cmpq	%rax, 0x6e0(%rbp)
142292f34:     	jne	0x142292f58 <.text+0x2291f58>
142292f36:     	movl	0x6f8(%rbp), %eax
142292f3c:     	movq	0x700(%rbp), %rcx
142292f43:     	movq	0x6d0(%rbp), %rdx
142292f4a:     	movl	%eax, (%rdx,%rcx,4)
142292f4d:     	movq	0x6b0(%rbp), %rax
142292f54:     	movb	$0x1, 0x20(%rax)
142292f58:     	movl	$0x0, %eax
142292f5d:     	movl	$0x0, %r13d
142292f63:     	testb	%r8b, %r8b
142292f66:     	jne	0x142292abe <.text+0x2291abe>
142292f6c:     	movq	%rbx, %rcx
142292f6f:     	callq	0x14092e5f0 <.text+0x92d5f0>
142292f74:     	movb	$0x1, %r14b
142292f77:     	movb	$0x1, %cl
142292f79:     	testb	%al, %al
142292f7b:     	jne	0x142292fca <.text+0x2291fca>
142292f7d:     	movq	0x660(%rbp), %rax
142292f84:     	movl	0x98(%rax), %ecx
142292f8a:     	movss	0x9c(%rax), %xmm1
142292f92:     	callq	0x14093fa90 <.text+0x93ea90>
142292f97:     	testb	%al, %al
142292f99:     	je	0x142292fc4 <.text+0x2291fc4>
142292f9b:     	movq	0x708(%rbp), %rax
142292fa2:     	cmpl	$0x1, (%rax)
142292fa5:     	jne	0x142292fc4 <.text+0x2291fc4>
142292fa7:     	movq	0x8(%rax), %rax
142292fab:     	cmpq	0x6e0(%rbp), %rax
142292fb2:     	sete	%r14b
142292fb6:     	movzbl	%r12b, %ecx
142292fba:     	movl	$0x1, %eax
142292fbf:     	cmovel	%eax, %ecx
142292fc2:     	jmp	0x142292fca <.text+0x2291fca>
142292fc4:     	xorl	%r14d, %r14d
142292fc7:     	movl	%r12d, %ecx
142292fca:     	movl	%ecx, %r12d
142292fcd:     	movq	0x6b8(%rbp), %r8
142292fd4:     	movq	0x6c0(%rbp), %rcx
142292fdb:     	movq	0x3a0(%rbp), %rax
142292fe2:     	testb	%al, %al
142292fe4:     	jne	0x142292cbd <.text+0x2291cbd>
142292fea:     	cmpb	$0x0, 0x4a0(%rbp)
142292ff1:     	je	0x142293248 <.text+0x2292248>
142292ff7:     	leaq	0x6e0(%rbp), %rax
142292ffe:     	movq	%rax, 0x3e8(%rbp)
142293005:     	leaq	-0x21cab4c(%rip), %rax  # 0x1400c84c0 <.text+0xc74c0>
14229300c:     	movq	%rax, 0x3f0(%rbp)
142293013:     	leaq	0x6a8(%rbp), %rcx
14229301a:     	movq	%rcx, 0x3f8(%rbp)
142293021:     	movq	%rax, 0x400(%rbp)
142293028:     	leaq	0xda8801(%rip), %rax    # 0x14303b830
14229302f:     	movq	%rax, 0x530(%rbp)
142293036:     	movq	$0x2, 0x538(%rbp)
142293041:     	movq	%rbx, 0x540(%rbp)
142293048:     	movq	$0x2, 0x548(%rbp)
142293053:     	movq	$0x0, 0x550(%rbp)
14229305e:     	movq	%rsi, %rcx
142293061:     	movq	%rdi, %rdx
142293064:     	callq	0x140d79f90 <.text+0xd78f90>
142293069:     	movq	0x630(%rbp), %rax
142293070:     	movsd	(%rax), %xmm0
142293074:     	movsd	0x8(%rax), %xmm1
142293079:     	movddup	%xmm0, %xmm2            # xmm2 = xmm0[0,0]
14229307d:     	mulps	%xmm13, %xmm1
142293081:     	movddup	%xmm1, %xmm3            # xmm3 = xmm1[0,0]
142293085:     	addps	%xmm2, %xmm3
142293088:     	subps	%xmm1, %xmm0
14229308b:     	shufps	$0xe4, %xmm3, %xmm0     # xmm0 = xmm0[0,1],xmm3[2,3]
14229308f:     	movups	%xmm0, 0x3f0(%rbp)
142293096:     	movl	$0x0, 0x3ec(%rbp)
1422930a0:     	movb	$0x3, 0x3e8(%rbp)
1422930a7:     	movb	$0x0, 0x614(%rbp)
1422930ae:     	movb	$0x1, 0x6a0(%rbp)
1422930b5:     	movq	%rdi, %rcx
1422930b8:     	movq	%rbx, %rdx
1422930bb:     	leaq	0x614(%rbp), %r8
1422930c2:     	callq	0x1409a8e10 <.text+0x9a7e10>
1422930c7:     	movb	$0x0, 0x6a0(%rbp)
1422930ce:     	movq	0x650(%rbp), %rcx
1422930d5:     	movq	%rsi, %rdx
1422930d8:     	movq	%rdi, %r8
1422930db:     	callq	0x1422bd120 <.text+0x22bc120>
1422930e0:     	movq	%rax, 0x3e8(%rbp)
1422930e7:     	movq	%rdx, 0x3f0(%rbp)
1422930ee:     	movq	0x708(%rbp), %rax
1422930f5:     	movzbl	0x86(%rax), %esi
1422930fc:     	movq	%rbx, %rcx
1422930ff:     	callq	0x14092e680 <.text+0x92d680>
142293104:     	testb	%al, %al
142293106:     	je	0x142293150 <.text+0x2292150>
142293108:     	movq	0x6a8(%rbp), %rax
14229310f:     	movq	%rax, 0x658(%rbp)
142293116:     	movl	0x6f8(%rbp), %eax
14229311c:     	movq	0x700(%rbp), %rcx
142293123:     	movq	0x6d0(%rbp), %rdx
14229312a:     	movl	%eax, (%rdx,%rcx,4)
14229312d:     	movq	0x6b0(%rbp), %rax
142293134:     	movb	$0x1, 0x20(%rax)
142293138:     	xorb	$0x1, %sil
14229313c:     	movb	$0x1, %al
14229313e:     	movq	%rax, 0x340(%rbp)
142293145:     	movl	$0x1, %r13d
14229314b:     	jmp	0x142293447 <.text+0x2292447>
142293150:     	testq	%r13, %r13
142293153:     	je	0x1422933ea <.text+0x22923ea>
142293159:     	movl	$0x1, %r13d
14229315f:     	jmp	0x142293447 <.text+0x2292447>
142293164:     	cmpb	$0x0, 0x6cc(%rbp)
14229316b:     	je	0x1422931af <.text+0x22921af>
14229316d:     	movq	0x708(%rbp), %rax
142293174:     	testb	$0x1, (%rax)
142293177:     	je	0x1422931af <.text+0x22921af>
142293179:     	movq	0x708(%rbp), %rax
142293180:     	movq	0x8(%rax), %rax
142293184:     	cmpq	%rax, 0x6e0(%rbp)
14229318b:     	jne	0x1422931af <.text+0x22921af>
14229318d:     	movl	0x6f8(%rbp), %eax
142293193:     	movq	0x700(%rbp), %rcx
14229319a:     	movq	0x6d0(%rbp), %rdx
1422931a1:     	movl	%eax, (%rdx,%rcx,4)
1422931a4:     	movq	0x6b0(%rbp), %rax
1422931ab:     	movb	$0x1, 0x20(%rax)
1422931af:     	movl	$0x0, %eax
1422931b4:     	movl	$0x0, %r13d
1422931ba:     	testb	%r8b, %r8b
1422931bd:     	jne	0x142292df5 <.text+0x2291df5>
1422931c3:     	movq	%rbx, %rcx
1422931c6:     	callq	0x14092e5f0 <.text+0x92d5f0>
1422931cb:     	movb	$0x1, %cl
1422931cd:     	movq	%rcx, 0x1c0(%rbp)
1422931d4:     	movb	$0x1, %r14b
1422931d7:     	movb	$0x1, %cl
1422931d9:     	testb	%al, %al
1422931db:     	jne	0x14229322a <.text+0x229222a>
1422931dd:     	movq	0x660(%rbp), %rax
1422931e4:     	movl	0x98(%rax), %ecx
1422931ea:     	movss	0x9c(%rax), %xmm1
1422931f2:     	callq	0x14093fa90 <.text+0x93ea90>
1422931f7:     	testb	%al, %al
1422931f9:     	je	0x142293224 <.text+0x2292224>
1422931fb:     	movq	0x708(%rbp), %rax
142293202:     	cmpl	$0x1, (%rax)
142293205:     	jne	0x142293224 <.text+0x2292224>
142293207:     	movq	0x8(%rax), %rax
14229320b:     	cmpq	0x6e0(%rbp), %rax
142293212:     	sete	%r14b
142293216:     	movzbl	%r12b, %ecx
14229321a:     	movl	$0x1, %eax
14229321f:     	cmovel	%eax, %ecx
142293222:     	jmp	0x14229322a <.text+0x229222a>
142293224:     	xorl	%r14d, %r14d
142293227:     	movl	%r12d, %ecx
14229322a:     	movl	%ecx, %r12d
14229322d:     	movq	0x6b8(%rbp), %r8
142293234:     	movq	0x6c0(%rbp), %rcx
14229323b:     	cmpb	$0x0, 0x4a0(%rbp)
142293242:     	jne	0x142292ff7 <.text+0x2291ff7>
142293248:     	movl	%r12d, %r15d
14229324b:     	cmpb	$0x0, 0x49c(%rbp)
142293252:     	je	0x1422935b5 <.text+0x22925b5>
142293258:     	movq	%r13, %rbx
14229325b:     	leaq	0x6e0(%rbp), %rax
142293262:     	movq	%rax, 0x3e8(%rbp)
142293269:     	leaq	-0x21cadb0(%rip), %rax  # 0x1400c84c0 <.text+0xc74c0>
142293270:     	movq	%rax, 0x3f0(%rbp)
142293277:     	leaq	0x6a8(%rbp), %rcx
14229327e:     	movq	%rcx, 0x3f8(%rbp)
142293285:     	movq	%rax, 0x400(%rbp)
14229328c:     	leaq	0xda8605(%rip), %rax    # 0x14303b898
142293293:     	movq	%rax, 0x530(%rbp)
14229329a:     	movq	$0x2, 0x538(%rbp)
1422932a5:     	leaq	0x3e8(%rbp), %rsi
1422932ac:     	movq	%rsi, 0x540(%rbp)
1422932b3:     	movq	$0x2, 0x548(%rbp)
1422932be:     	movq	$0x0, 0x550(%rbp)
1422932c9:     	leaq	0x638(%rbp), %r13
1422932d0:     	movq	%r13, %rcx
1422932d3:     	movq	%rdi, %rdx
1422932d6:     	callq	0x140d79f90 <.text+0xd78f90>
1422932db:     	movq	0x630(%rbp), %rax
1422932e2:     	movsd	(%rax), %xmm0
1422932e6:     	movsd	0x8(%rax), %xmm1
1422932eb:     	movddup	%xmm0, %xmm2            # xmm2 = xmm0[0,0]
1422932ef:     	mulps	%xmm13, %xmm1
1422932f3:     	movddup	%xmm1, %xmm3            # xmm3 = xmm1[0,0]
1422932f7:     	addps	%xmm2, %xmm3
1422932fa:     	subps	%xmm1, %xmm0
1422932fd:     	shufps	$0xe4, %xmm3, %xmm0     # xmm0 = xmm0[0,1],xmm3[2,3]
142293301:     	movups	%xmm0, 0x3f0(%rbp)
142293308:     	movl	$0x0, 0x3ec(%rbp)
142293312:     	movb	$0x3, 0x3e8(%rbp)
142293319:     	movb	$0x0, 0x614(%rbp)
142293320:     	movb	$0x1, 0x69f(%rbp)
142293327:     	movq	%rdi, %rcx
14229332a:     	movq	%rsi, %rdx
14229332d:     	leaq	0x614(%rbp), %r8
142293334:     	callq	0x1409a8e10 <.text+0x9a7e10>
142293339:     	movq	0x650(%rbp), %rcx
142293340:     	movb	$0x0, 0x69f(%rbp)
142293347:     	movq	%r13, %rdx
14229334a:     	movq	%rdi, %r8
14229334d:     	callq	0x1422bd120 <.text+0x22bc120>
142293352:     	movq	%rax, 0x3e8(%rbp)
142293359:     	movq	%rdx, 0x3f0(%rbp)
142293360:     	movq	0x708(%rbp), %rax
142293367:     	movzbl	0x87(%rax), %eax
14229336e:     	movb	%al, 0x530(%rbp)
142293374:     	leaq	0x3e8(%rbp), %rcx
14229337b:     	callq	0x14092e680 <.text+0x92d680>
142293380:     	testb	%al, %al
142293382:     	je	0x1422933d6 <.text+0x22923d6>
142293384:     	movq	0x6a8(%rbp), %rax
14229338b:     	movq	%rax, 0x658(%rbp)
142293392:     	movl	0x6f8(%rbp), %eax
142293398:     	movq	0x700(%rbp), %rcx
14229339f:     	movq	0x6d0(%rbp), %rdx
1422933a6:     	movl	%eax, (%rdx,%rcx,4)
1422933a9:     	movq	0x6b0(%rbp), %rax
1422933b0:     	movb	$0x1, 0x20(%rax)
1422933b4:     	movl	$0x1, %esi
1422933b9:     	movq	%rdi, %rcx
1422933bc:     	callq	0x1408c5660 <.text+0x8c4660>
1422933c1:     	movzbl	0x530(%rbp), %eax
1422933c8:     	movb	%al, 0x6a6(%rbp)
1422933ce:     	movq	%rsi, %r13
1422933d1:     	jmp	0x1422937cd <.text+0x22927cd>
1422933d6:     	testq	%rbx, %rbx
1422933d9:     	je	0x14229376a <.text+0x229276a>
1422933df:     	movl	$0x1, %r13d
1422933e5:     	jmp	0x1422937cd <.text+0x22927cd>
1422933ea:     	cmpb	$0x0, 0x6cc(%rbp)
1422933f1:     	movq	0x708(%rbp), %rax
1422933f8:     	je	0x14229342e <.text+0x229242e>
1422933fa:     	testb	$0x1, (%rax)
1422933fd:     	je	0x14229342e <.text+0x229242e>
1422933ff:     	movq	0x8(%rax), %rax
142293403:     	cmpq	%rax, 0x6e0(%rbp)
14229340a:     	jne	0x14229342e <.text+0x229242e>
14229340c:     	movl	0x6f8(%rbp), %eax
142293412:     	movq	0x700(%rbp), %rcx
142293419:     	movq	0x6d0(%rbp), %rdx
142293420:     	movl	%eax, (%rdx,%rcx,4)
142293423:     	movq	0x6b0(%rbp), %rax
14229342a:     	movb	$0x1, 0x20(%rax)
14229342e:     	cmpb	$0x0, 0x717(%rbp)
142293435:     	je	0x142293444 <.text+0x2292444>
142293437:     	xorb	$0x1, %sil
14229343b:     	movb	$0x1, %al
14229343d:     	movq	%rax, 0x340(%rbp)
142293444:     	xorl	%r13d, %r13d
142293447:     	movq	%rbx, %rcx
14229344a:     	callq	0x14092e5f0 <.text+0x92d5f0>
14229344f:     	movb	$0x1, %r14b
142293452:     	movb	$0x1, %r15b
142293455:     	testb	%al, %al
142293457:     	movq	0x3c0(%rbp), %rbx
14229345e:     	je	0x1422934aa <.text+0x22924aa>
142293460:     	movzbl	0x13a17a2(%rip), %eax   # 0x143634c09
142293467:     	testb	%sil, %sil
14229346a:     	je	0x142293517 <.text+0x2292517>
142293470:     	movl	$0x1a, %ecx
142293475:     	movl	$0x1, %edx
14229347a:     	callq	0x140613c10 <.text+0x612c10>
14229347f:     	testq	%rax, %rax
142293482:     	je	0x142296cf9 <.text+0x2295cf9>
142293488:     	movq	%rax, %rdi
14229348b:     	movups	0xda83e5(%rip), %xmm0   # 0x14303b877
142293492:     	movups	%xmm0, 0xa(%rax)
142293496:     	movups	0xda83d0(%rip), %xmm0   # 0x14303b86d
14229349d:     	movups	%xmm0, (%rax)
1422934a0:     	movl	$0x1a, %esi
1422934a5:     	jmp	0x14229354c <.text+0x229254c>
1422934aa:     	movq	0x660(%rbp), %rax
1422934b1:     	movl	0x98(%rax), %ecx
1422934b7:     	movss	0x9c(%rax), %xmm1
1422934bf:     	callq	0x14093fa90 <.text+0x93ea90>
1422934c4:     	testb	%al, %al
1422934c6:     	je	0x142293501 <.text+0x2292501>
1422934c8:     	movq	0x708(%rbp), %rax
1422934cf:     	cmpl	$0x1, (%rax)
1422934d2:     	jne	0x142293501 <.text+0x2292501>
1422934d4:     	movq	0x8(%rax), %rax
1422934d8:     	cmpq	0x6e0(%rbp), %rax
1422934df:     	sete	%r14b
1422934e3:     	movzbl	%r12b, %r15d
1422934e7:     	movl	$0x1, %eax
1422934ec:     	cmovel	%eax, %r15d
1422934f0:     	movzbl	0x13a1712(%rip), %eax   # 0x143634c09
1422934f7:     	testb	%sil, %sil
1422934fa:     	je	0x142293517 <.text+0x2292517>
1422934fc:     	jmp	0x142293470 <.text+0x2292470>
142293501:     	xorl	%r14d, %r14d
142293504:     	movl	%r12d, %r15d
142293507:     	movzbl	0x13a16fb(%rip), %eax   # 0x143634c09
14229350e:     	testb	%sil, %sil
142293511:     	jne	0x142293470 <.text+0x2292470>
142293517:     	movl	$0x1d, %ecx
14229351c:     	movl	$0x1, %edx
142293521:     	callq	0x140613c10 <.text+0x612c10>
142293526:     	testq	%rax, %rax
142293529:     	je	0x142296ce3 <.text+0x2295ce3>
14229352f:     	movq	%rax, %rdi
142293532:     	movups	0xda8324(%rip), %xmm0   # 0x14303b85d
142293539:     	movups	%xmm0, 0xd(%rax)
14229353d:     	movups	0xda830c(%rip), %xmm0   # 0x14303b850
142293544:     	movups	%xmm0, (%rax)
142293547:     	movl	$0x1d, %esi
14229354c:     	movq	0x6b8(%rbp), %r8
142293553:     	movq	0x6c0(%rbp), %rcx
14229355a:     	movl	0x6f8(%rbp), %eax
142293560:     	movq	0x700(%rbp), %rdx
142293567:     	movl	%eax, (%rbx,%rdx,4)
14229356a:     	movq	0x4f8(%rbp), %rbx
142293571:     	movq	(%rbx), %rdx
142293574:     	testq	%rdx, %rdx
142293577:     	je	0x142293596 <.text+0x2292596>
142293579:     	movq	0x8(%rbx), %rcx
14229357d:     	movl	$0x1, %r8d
142293583:     	callq	0x140613c20 <.text+0x612c20>
142293588:     	movq	0x6c0(%rbp), %rcx
14229358f:     	movq	0x6b8(%rbp), %r8
142293596:     	movq	%rsi, (%rbx)
142293599:     	movq	%rdi, 0x8(%rbx)
14229359d:     	movq	%rsi, 0x10(%rbx)
1422935a1:     	leaq	0x530(%rbp), %rdi
1422935a8:     	cmpb	$0x0, 0x49c(%rbp)
1422935af:     	jne	0x142293258 <.text+0x2292258>
1422935b5:     	movl	%r15d, %esi
1422935b8:     	cmpb	$0x0, 0x498(%rbp)
1422935bf:     	leaq	0x3e8(%rbp), %r15
1422935c6:     	je	0x14229392d <.text+0x229292d>
1422935cc:     	movq	%r13, %rbx
1422935cf:     	leaq	0x6e0(%rbp), %rax
1422935d6:     	movq	%rax, 0x3e8(%rbp)
1422935dd:     	leaq	-0x21cb124(%rip), %rax  # 0x1400c84c0 <.text+0xc74c0>
1422935e4:     	movq	%rax, 0x3f0(%rbp)
1422935eb:     	leaq	0x6a8(%rbp), %rcx
1422935f2:     	movq	%rcx, 0x3f8(%rbp)
1422935f9:     	movq	%rax, 0x400(%rbp)
142293600:     	leaq	0xda8301(%rip), %rax    # 0x14303b908
142293607:     	movq	%rax, 0x530(%rbp)
14229360e:     	movq	$0x2, 0x538(%rbp)
142293619:     	movq	%r15, 0x540(%rbp)
142293620:     	movq	$0x2, 0x548(%rbp)
14229362b:     	movq	$0x0, 0x550(%rbp)
142293636:     	leaq	0x638(%rbp), %r13
14229363d:     	movq	%r13, %rcx
142293640:     	movq	%rdi, %rdx
142293643:     	callq	0x140d79f90 <.text+0xd78f90>
142293648:     	movq	0x630(%rbp), %rax
14229364f:     	movsd	(%rax), %xmm0
142293653:     	movsd	0x8(%rax), %xmm1
142293658:     	movddup	%xmm0, %xmm2            # xmm2 = xmm0[0,0]
14229365c:     	mulps	%xmm13, %xmm1
142293660:     	movddup	%xmm1, %xmm3            # xmm3 = xmm1[0,0]
142293664:     	addps	%xmm2, %xmm3
142293667:     	subps	%xmm1, %xmm0
14229366a:     	shufps	$0xe4, %xmm3, %xmm0     # xmm0 = xmm0[0,1],xmm3[2,3]
14229366e:     	movups	%xmm0, 0x3f0(%rbp)
142293675:     	movl	$0x0, 0x3ec(%rbp)
14229367f:     	movb	$0x3, 0x3e8(%rbp)
142293686:     	movb	$0x0, 0x614(%rbp)
14229368d:     	movb	$0x1, 0x69e(%rbp)
142293694:     	movq	%rdi, %rcx
142293697:     	movq	%r15, %rdx
14229369a:     	leaq	0x614(%rbp), %r8
1422936a1:     	callq	0x1409a8e10 <.text+0x9a7e10>
1422936a6:     	movb	$0x0, 0x69e(%rbp)
1422936ad:     	movq	0x650(%rbp), %rcx
1422936b4:     	movq	%r13, %rdx
1422936b7:     	movq	%rdi, %r8
1422936ba:     	callq	0x1422bd120 <.text+0x22bc120>
1422936bf:     	movq	%rax, 0x3e8(%rbp)
1422936c6:     	movq	%rdx, 0x3f0(%rbp)
1422936cd:     	movq	0x708(%rbp), %rax
1422936d4:     	movzbl	0x88(%rax), %eax
1422936db:     	movb	%al, 0x530(%rbp)
1422936e1:     	movq	%r15, %rcx
1422936e4:     	callq	0x14092e680 <.text+0x92d680>
1422936e9:     	testb	%al, %al
1422936eb:     	je	0x142293747 <.text+0x2292747>
1422936ed:     	movq	0x6a8(%rbp), %rax
1422936f4:     	movq	%rax, 0x658(%rbp)
1422936fb:     	movl	0x6f8(%rbp), %eax
142293701:     	movq	0x700(%rbp), %rcx
142293708:     	movq	0x6d0(%rbp), %rdx
14229370f:     	movl	%eax, (%rdx,%rcx,4)
142293712:     	movq	0x6b0(%rbp), %rax
142293719:     	movb	$0x1, 0x20(%rax)
14229371d:     	movl	$0x1, %eax
142293722:     	movq	0x630(%rbp), %rbx
142293729:     	movzbl	0x530(%rbp), %ecx
142293730:     	xorb	$0x1, %cl
142293733:     	movb	%cl, 0x6a7(%rbp)
142293739:     	movb	%cl, 0x530(%rbp)
14229373f:     	movq	%rax, %r13
142293742:     	jmp	0x142293b21 <.text+0x2292b21>
142293747:     	testq	%rbx, %rbx
14229374a:     	movzbl	0x717(%rbp), %r8d
142293752:     	je	0x142293abb <.text+0x2292abb>
142293758:     	movl	$0x1, %r13d
14229375e:     	movq	0x630(%rbp), %rbx
142293765:     	jmp	0x142293b21 <.text+0x2292b21>
14229376a:     	cmpb	$0x0, 0x6cc(%rbp)
142293771:     	je	0x1422937b5 <.text+0x22927b5>
142293773:     	movq	0x708(%rbp), %rax
14229377a:     	testb	$0x1, (%rax)
14229377d:     	je	0x1422937b5 <.text+0x22927b5>
14229377f:     	movq	0x708(%rbp), %rax
142293786:     	movq	0x8(%rax), %rax
14229378a:     	cmpq	%rax, 0x6e0(%rbp)
142293791:     	jne	0x1422937b5 <.text+0x22927b5>
142293793:     	movl	0x6f8(%rbp), %eax
142293799:     	movq	0x700(%rbp), %rcx
1422937a0:     	movq	0x6d0(%rbp), %rdx
1422937a7:     	movl	%eax, (%rdx,%rcx,4)
1422937aa:     	movq	0x6b0(%rbp), %rax
1422937b1:     	movb	$0x1, 0x20(%rax)
1422937b5:     	movl	$0x0, %esi
1422937ba:     	movl	$0x0, %r13d
1422937c0:     	cmpb	$0x0, 0x717(%rbp)
1422937c7:     	jne	0x1422933b9 <.text+0x22923b9>
1422937cd:     	leaq	0x3e8(%rbp), %rcx
1422937d4:     	callq	0x14092e5f0 <.text+0x92d5f0>
1422937d9:     	movb	$0x1, %bl
1422937db:     	movb	$0x1, %sil
1422937de:     	testb	%al, %al
1422937e0:     	jne	0x14229382d <.text+0x229282d>
1422937e2:     	movq	0x660(%rbp), %rax
1422937e9:     	movl	0x98(%rax), %ecx
1422937ef:     	movss	0x9c(%rax), %xmm1
1422937f7:     	callq	0x14093fa90 <.text+0x93ea90>
1422937fc:     	testb	%al, %al
1422937fe:     	je	0x142293828 <.text+0x2292828>
142293800:     	movq	0x708(%rbp), %rax
142293807:     	cmpl	$0x1, (%rax)
14229380a:     	jne	0x142293828 <.text+0x2292828>
14229380c:     	movq	0x8(%rax), %rax
142293810:     	cmpq	0x6e0(%rbp), %rax
142293817:     	sete	%bl
14229381a:     	movzbl	%r15b, %esi
14229381e:     	movl	$0x1, %eax
142293823:     	cmovel	%eax, %esi
142293826:     	jmp	0x14229382d <.text+0x229282d>
142293828:     	xorl	%ebx, %ebx
14229382a:     	movl	%r15d, %esi
14229382d:     	movq	%rdi, %rcx
142293830:     	callq	0x140833230 <.text+0x832230>
142293835:     	movzbl	0x13a13cd(%rip), %ecx   # 0x143634c09
14229383c:     	testb	%al, %al
14229383e:     	je	0x14229387a <.text+0x229287a>
142293840:     	movl	$0x1e, %ecx
142293845:     	movl	$0x1, %edx
14229384a:     	callq	0x140613c10 <.text+0x612c10>
14229384f:     	testq	%rax, %rax
142293852:     	je	0x142296c87 <.text+0x2295c87>
142293858:     	movq	%rax, %rdi
14229385b:     	movl	%ebx, %r15d
14229385e:     	movups	0xda8080(%rip), %xmm0   # 0x14303b8e5
142293865:     	movups	%xmm0, 0xe(%rax)
142293869:     	movups	0xda8067(%rip), %xmm0   # 0x14303b8d7
142293870:     	movups	%xmm0, (%rax)
142293873:     	movl	$0x1e, %ebx
142293878:     	jmp	0x1422938b2 <.text+0x22928b2>
14229387a:     	movl	$0x1f, %ecx
14229387f:     	movl	$0x1, %edx
142293884:     	callq	0x140613c10 <.text+0x612c10>
142293889:     	testq	%rax, %rax
14229388c:     	je	0x142296c62 <.text+0x2295c62>
142293892:     	movq	%rax, %rdi
142293895:     	movl	%ebx, %r15d
142293898:     	movups	0xda8028(%rip), %xmm0   # 0x14303b8c7
14229389f:     	movups	%xmm0, 0xf(%rax)
1422938a3:     	movups	0xda800e(%rip), %xmm0   # 0x14303b8b8
1422938aa:     	movups	%xmm0, (%rax)
1422938ad:     	movl	$0x1f, %ebx
1422938b2:     	movq	0x6b8(%rbp), %r8
1422938b9:     	movq	0x6c0(%rbp), %rcx
1422938c0:     	movl	0x6f8(%rbp), %eax
1422938c6:     	movq	0x700(%rbp), %rdx
1422938cd:     	movq	0x3c0(%rbp), %r9
1422938d4:     	movl	%eax, (%r9,%rdx,4)
1422938d8:     	movq	0x4f8(%rbp), %r14
1422938df:     	movq	(%r14), %rdx
1422938e2:     	testq	%rdx, %rdx
1422938e5:     	je	0x142293904 <.text+0x2292904>
1422938e7:     	movq	0x8(%r14), %rcx
1422938eb:     	movl	$0x1, %r8d
1422938f1:     	callq	0x140613c20 <.text+0x612c20>
1422938f6:     	movq	0x6c0(%rbp), %rcx
1422938fd:     	movq	0x6b8(%rbp), %r8
142293904:     	movq	%rbx, (%r14)
142293907:     	movq	%rdi, 0x8(%r14)
14229390b:     	movq	%rbx, 0x10(%r14)
14229390f:     	leaq	0x530(%rbp), %rdi
142293916:     	movl	%r15d, %r14d
142293919:     	cmpb	$0x0, 0x498(%rbp)
142293920:     	leaq	0x3e8(%rbp), %r15
142293927:     	jne	0x1422935cc <.text+0x22925cc>
14229392d:     	movl	%esi, 0x6ec(%rbp)
142293933:     	movq	0x630(%rbp), %rbx
14229393a:     	cmpb	$0x0, 0x2f8(%rbp)
142293941:     	leaq	0x638(%rbp), %rsi
142293948:     	je	0x142293c88 <.text+0x2292c88>
14229394e:     	leaq	0x6e0(%rbp), %rax
142293955:     	movq	%rax, 0x3e8(%rbp)
14229395c:     	leaq	-0x21cb4a3(%rip), %rax  # 0x1400c84c0 <.text+0xc74c0>
142293963:     	movq	%rax, 0x3f0(%rbp)
14229396a:     	leaq	0x6a8(%rbp), %rcx
142293971:     	movq	%rcx, 0x3f8(%rbp)
142293978:     	movq	%rax, 0x400(%rbp)
14229397f:     	leaq	0xda7fb2(%rip), %rax    # 0x14303b938
142293986:     	movq	%rax, 0x530(%rbp)
14229398d:     	movq	$0x2, 0x538(%rbp)
142293998:     	movq	%r15, 0x540(%rbp)
14229399f:     	movq	$0x2, 0x548(%rbp)
1422939aa:     	movq	$0x0, 0x550(%rbp)
1422939b5:     	movq	%rsi, %rcx
1422939b8:     	movq	%rdi, %rdx
1422939bb:     	callq	0x140d79f90 <.text+0xd78f90>
1422939c0:     	movsd	(%rbx), %xmm0
1422939c4:     	movsd	0x8(%rbx), %xmm1
1422939c9:     	movddup	%xmm0, %xmm2            # xmm2 = xmm0[0,0]
1422939cd:     	mulps	%xmm13, %xmm1
1422939d1:     	movddup	%xmm1, %xmm3            # xmm3 = xmm1[0,0]
1422939d5:     	addps	%xmm2, %xmm3
1422939d8:     	subps	%xmm1, %xmm0
1422939db:     	shufps	$0xe4, %xmm3, %xmm0     # xmm0 = xmm0[0,1],xmm3[2,3]
1422939df:     	movups	%xmm0, 0x3f0(%rbp)
1422939e6:     	movl	$0x0, 0x3ec(%rbp)
1422939f0:     	movb	$0x3, 0x3e8(%rbp)
1422939f7:     	movb	$0x0, 0x614(%rbp)
1422939fe:     	movb	$0x1, 0x69d(%rbp)
142293a05:     	movq	%rdi, %rcx
142293a08:     	movq	%r15, %rdx
142293a0b:     	leaq	0x614(%rbp), %r8
142293a12:     	callq	0x1409a8e10 <.text+0x9a7e10>
142293a17:     	movb	$0x0, 0x69d(%rbp)
142293a1e:     	movq	0x650(%rbp), %rcx
142293a25:     	movq	%rsi, %rdx
142293a28:     	movq	%rdi, %r8
142293a2b:     	callq	0x1422bd120 <.text+0x22bc120>
142293a30:     	movq	%rax, 0x3e8(%rbp)
142293a37:     	movq	%rdx, 0x3f0(%rbp)
142293a3e:     	movq	%r15, %rcx
142293a41:     	callq	0x14092e680 <.text+0x92d680>
142293a46:     	testb	%al, %al
142293a48:     	je	0x142293a9f <.text+0x2292a9f>
142293a4a:     	movq	0x6a8(%rbp), %rax
142293a51:     	movq	%rax, 0x658(%rbp)
142293a58:     	movl	0x6f8(%rbp), %eax
142293a5e:     	movq	0x700(%rbp), %rcx
142293a65:     	movq	0x6d0(%rbp), %rdx
142293a6c:     	movl	%eax, (%rdx,%rcx,4)
142293a6f:     	movq	0x6b0(%rbp), %rax
142293a76:     	movb	$0x1, 0x20(%rax)
142293a7a:     	movl	$0x1, %eax
142293a7f:     	movq	0x700(%rbp), %rcx
142293a86:     	movq	0x4d8(%rbp), %rdx
142293a8d:     	movzbl	(%rdx,%rcx), %ecx
142293a91:     	movb	%cl, 0x6f7(%rbp)
142293a97:     	movq	%rax, %r13
142293a9a:     	jmp	0x142293e7c <.text+0x2292e7c>
142293a9f:     	testq	%r13, %r13
142293aa2:     	movzbl	0x717(%rbp), %r8d
142293aaa:     	je	0x142293e1d <.text+0x2292e1d>
142293ab0:     	movl	$0x1, %r13d
142293ab6:     	jmp	0x142293e7c <.text+0x2292e7c>
142293abb:     	cmpb	$0x0, 0x6cc(%rbp)
142293ac2:     	movq	0x630(%rbp), %rbx
142293ac9:     	je	0x142293b0d <.text+0x2292b0d>
142293acb:     	movq	0x708(%rbp), %rax
142293ad2:     	testb	$0x1, (%rax)
142293ad5:     	je	0x142293b0d <.text+0x2292b0d>
142293ad7:     	movq	0x708(%rbp), %rax
142293ade:     	movq	0x8(%rax), %rax
142293ae2:     	cmpq	%rax, 0x6e0(%rbp)
142293ae9:     	jne	0x142293b0d <.text+0x2292b0d>
142293aeb:     	movl	0x6f8(%rbp), %eax
142293af1:     	movq	0x700(%rbp), %rcx
142293af8:     	movq	0x6d0(%rbp), %rdx
142293aff:     	movl	%eax, (%rdx,%rcx,4)
142293b02:     	movq	0x6b0(%rbp), %rax
142293b09:     	movb	$0x1, 0x20(%rax)
142293b0d:     	movl	$0x0, %eax
142293b12:     	movl	$0x0, %r13d
142293b18:     	testb	%r8b, %r8b
142293b1b:     	jne	0x142293729 <.text+0x2292729>
142293b21:     	movq	%r15, %rcx
142293b24:     	callq	0x14092e5f0 <.text+0x92d5f0>
142293b29:     	movb	$0x1, %r12b
142293b2c:     	movb	$0x1, %cl
142293b2e:     	movl	%ecx, 0x6ec(%rbp)
142293b34:     	testb	%al, %al
142293b36:     	jne	0x142293b8e <.text+0x2292b8e>
142293b38:     	movq	0x660(%rbp), %rax
142293b3f:     	movl	0x98(%rax), %ecx
142293b45:     	movss	0x9c(%rax), %xmm1
142293b4d:     	callq	0x14093fa90 <.text+0x93ea90>
142293b52:     	testb	%al, %al
142293b54:     	je	0x142293b85 <.text+0x2292b85>
142293b56:     	movq	0x708(%rbp), %rax
142293b5d:     	cmpl	$0x1, (%rax)
142293b60:     	jne	0x142293b85 <.text+0x2292b85>
142293b62:     	movq	0x8(%rax), %rax
142293b66:     	cmpq	0x6e0(%rbp), %rax
142293b6d:     	sete	%r12b
142293b71:     	movzbl	%sil, %ecx
142293b75:     	movl	$0x1, %eax
142293b7a:     	cmovel	%eax, %ecx
142293b7d:     	movl	%ecx, 0x6ec(%rbp)
142293b83:     	jmp	0x142293b8e <.text+0x2292b8e>
142293b85:     	xorl	%r12d, %r12d
142293b88:     	movl	%esi, 0x6ec(%rbp)
142293b8e:     	movq	%rdi, %rcx
142293b91:     	callq	0x140833230 <.text+0x832230>
142293b96:     	movzbl	0x13a106c(%rip), %ecx   # 0x143634c09
142293b9d:     	testb	%al, %al
142293b9f:     	je	0x142293bd8 <.text+0x2292bd8>
142293ba1:     	movl	$0x1e, %ecx
142293ba6:     	movl	$0x1, %edx
142293bab:     	callq	0x140613c10 <.text+0x612c10>
142293bb0:     	testq	%rax, %rax
142293bb3:     	je	0x142296c87 <.text+0x2295c87>
142293bb9:     	movq	%rax, %rdi
142293bbc:     	movups	0xda7d22(%rip), %xmm0   # 0x14303b8e5
142293bc3:     	movups	%xmm0, 0xe(%rax)
142293bc7:     	movups	0xda7d09(%rip), %xmm0   # 0x14303b8d7
142293bce:     	movups	%xmm0, (%rax)
142293bd1:     	movl	$0x1e, %esi
142293bd6:     	jmp	0x142293c0d <.text+0x2292c0d>
142293bd8:     	movl	$0x1f, %ecx
142293bdd:     	movl	$0x1, %edx
142293be2:     	callq	0x140613c10 <.text+0x612c10>
142293be7:     	testq	%rax, %rax
142293bea:     	je	0x142296c62 <.text+0x2295c62>
142293bf0:     	movq	%rax, %rdi
142293bf3:     	movups	0xda7ccd(%rip), %xmm0   # 0x14303b8c7
142293bfa:     	movups	%xmm0, 0xf(%rax)
142293bfe:     	movups	0xda7cb3(%rip), %xmm0   # 0x14303b8b8
142293c05:     	movups	%xmm0, (%rax)
142293c08:     	movl	$0x1f, %esi
142293c0d:     	movq	0x6b8(%rbp), %r8
142293c14:     	movq	0x6c0(%rbp), %rcx
142293c1b:     	movl	0x6f8(%rbp), %eax
142293c21:     	movq	0x700(%rbp), %rdx
142293c28:     	movq	0x3c0(%rbp), %r9
142293c2f:     	movl	%eax, (%r9,%rdx,4)
142293c33:     	movq	0x4f8(%rbp), %r14
142293c3a:     	movq	(%r14), %rdx
142293c3d:     	testq	%rdx, %rdx
142293c40:     	je	0x142293c5f <.text+0x2292c5f>
142293c42:     	movq	0x8(%r14), %rcx
142293c46:     	movl	$0x1, %r8d
142293c4c:     	callq	0x140613c20 <.text+0x612c20>
142293c51:     	movq	0x6c0(%rbp), %rcx
142293c58:     	movq	0x6b8(%rbp), %r8
142293c5f:     	movq	%rsi, (%r14)
142293c62:     	movq	%rdi, 0x8(%r14)
142293c66:     	movq	%rsi, 0x10(%r14)
142293c6a:     	leaq	0x530(%rbp), %rdi
142293c71:     	movl	%r12d, %r14d
142293c74:     	cmpb	$0x0, 0x2f8(%rbp)
142293c7b:     	leaq	0x638(%rbp), %rsi
142293c82:     	jne	0x14229394e <.text+0x229294e>
142293c88:     	cmpb	$0x0, 0x300(%rbp)
142293c8f:     	je	0x142293f01 <.text+0x2292f01>
142293c95:     	leaq	0x6e0(%rbp), %rax
142293c9c:     	movq	%rax, 0x3e8(%rbp)
142293ca3:     	leaq	-0x21cb7ea(%rip), %rax  # 0x1400c84c0 <.text+0xc74c0>
142293caa:     	movq	%rax, 0x3f0(%rbp)
142293cb1:     	leaq	0x6a8(%rbp), %rcx
142293cb8:     	movq	%rcx, 0x3f8(%rbp)
142293cbf:     	movq	%rax, 0x400(%rbp)
142293cc6:     	leaq	0xda7ca3(%rip), %rax    # 0x14303b970
142293ccd:     	movq	%rax, 0x530(%rbp)
142293cd4:     	movq	$0x2, 0x538(%rbp)
142293cdf:     	movq	%r15, 0x540(%rbp)
142293ce6:     	movq	$0x2, 0x548(%rbp)
142293cf1:     	movq	$0x0, 0x550(%rbp)
142293cfc:     	movq	%rsi, %rcx
142293cff:     	movq	%rdi, %rdx
142293d02:     	callq	0x140d79f90 <.text+0xd78f90>
142293d07:     	movsd	(%rbx), %xmm0
142293d0b:     	movsd	0x8(%rbx), %xmm1
142293d10:     	movddup	%xmm0, %xmm2            # xmm2 = xmm0[0,0]
142293d14:     	mulps	%xmm13, %xmm1
142293d18:     	movddup	%xmm1, %xmm3            # xmm3 = xmm1[0,0]
142293d1c:     	addps	%xmm2, %xmm3
142293d1f:     	subps	%xmm1, %xmm0
142293d22:     	shufps	$0xe4, %xmm3, %xmm0     # xmm0 = xmm0[0,1],xmm3[2,3]
142293d26:     	movups	%xmm0, 0x3f0(%rbp)
142293d2d:     	movl	$0x0, 0x3ec(%rbp)
142293d37:     	movb	$0x3, 0x3e8(%rbp)
142293d3e:     	movb	$0x0, 0x614(%rbp)
142293d45:     	movb	$0x1, 0x69c(%rbp)
142293d4c:     	movq	%rdi, %rcx
142293d4f:     	movq	%r15, %rdx
142293d52:     	leaq	0x614(%rbp), %r8
142293d59:     	callq	0x1409a8e10 <.text+0x9a7e10>
142293d5e:     	movb	$0x0, 0x69c(%rbp)
142293d65:     	movq	0x650(%rbp), %rcx
142293d6c:     	movq	%rsi, %rdx
142293d6f:     	movq	%rdi, %r8
142293d72:     	callq	0x1422bd120 <.text+0x22bc120>
142293d77:     	movq	%rax, 0x3e8(%rbp)
142293d7e:     	movq	%rdx, 0x3f0(%rbp)
142293d85:     	movq	%r15, %rcx
142293d88:     	callq	0x14092e680 <.text+0x92d680>
142293d8d:     	testb	%al, %al
142293d8f:     	je	0x142293dfa <.text+0x2292dfa>
142293d91:     	movq	0x6a8(%rbp), %rax
142293d98:     	movq	%rax, 0x658(%rbp)
142293d9f:     	movl	0x6f8(%rbp), %eax
142293da5:     	movq	0x700(%rbp), %rcx
142293dac:     	movq	0x6d0(%rbp), %rdx
142293db3:     	movl	%eax, (%rdx,%rcx,4)
142293db6:     	movq	0x6b0(%rbp), %rax
142293dbd:     	movb	$0x1, 0x20(%rax)
142293dc1:     	movl	$0x1, %eax
142293dc6:     	movq	0x708(%rbp), %rcx
142293dcd:     	movl	$0x0, 0x28(%rcx)
142293dd4:     	movq	0x700(%rbp), %rcx
142293ddb:     	movq	0x4f0(%rbp), %rdx
142293de2:     	movss	(%rdx,%rcx,4), %xmm14
142293de8:     	movl	$0x1, 0x5f0(%rbp)
142293df2:     	movq	%rax, %r13
142293df5:     	jmp	0x1422940db <.text+0x22930db>
142293dfa:     	testq	%r13, %r13
142293dfd:     	movq	0x708(%rbp), %rcx
142293e04:     	movzbl	0x717(%rbp), %r9d
142293e0c:     	je	0x142294089 <.text+0x2293089>
142293e12:     	movl	$0x1, %r13d
142293e18:     	jmp	0x1422940db <.text+0x22930db>
142293e1d:     	cmpb	$0x0, 0x6cc(%rbp)
142293e24:     	je	0x142293e68 <.text+0x2292e68>
142293e26:     	movq	0x708(%rbp), %rax
142293e2d:     	testb	$0x1, (%rax)
142293e30:     	je	0x142293e68 <.text+0x2292e68>
142293e32:     	movq	0x708(%rbp), %rax
142293e39:     	movq	0x8(%rax), %rax
142293e3d:     	cmpq	%rax, 0x6e0(%rbp)
142293e44:     	jne	0x142293e68 <.text+0x2292e68>
142293e46:     	movl	0x6f8(%rbp), %eax
142293e4c:     	movq	0x700(%rbp), %rcx
142293e53:     	movq	0x6d0(%rbp), %rdx
142293e5a:     	movl	%eax, (%rdx,%rcx,4)
142293e5d:     	movq	0x6b0(%rbp), %rax
142293e64:     	movb	$0x1, 0x20(%rax)
142293e68:     	movl	$0x0, %eax
142293e6d:     	movl	$0x0, %r13d
142293e73:     	testb	%r8b, %r8b
142293e76:     	jne	0x142293a7f <.text+0x2292a7f>
142293e7c:     	movq	%r15, %rcx
142293e7f:     	callq	0x14092e5f0 <.text+0x92d5f0>
142293e84:     	movb	$0x1, %r14b
142293e87:     	movb	$0x1, %cl
142293e89:     	testb	%al, %al
142293e8b:     	jne	0x142293ee0 <.text+0x2292ee0>
142293e8d:     	movq	0x660(%rbp), %rax
142293e94:     	movl	0x98(%rax), %ecx
142293e9a:     	movss	0x9c(%rax), %xmm1
142293ea2:     	callq	0x14093fa90 <.text+0x93ea90>
142293ea7:     	testb	%al, %al
142293ea9:     	je	0x142293ed7 <.text+0x2292ed7>
142293eab:     	movq	0x708(%rbp), %rax
142293eb2:     	cmpl	$0x1, (%rax)
142293eb5:     	jne	0x142293ed7 <.text+0x2292ed7>
142293eb7:     	movq	0x8(%rax), %rax
142293ebb:     	cmpq	0x6e0(%rbp), %rax
142293ec2:     	sete	%r14b
142293ec6:     	movzbl	0x6ec(%rbp), %ecx
142293ecd:     	movl	$0x1, %eax
142293ed2:     	cmovel	%eax, %ecx
142293ed5:     	jmp	0x142293ee0 <.text+0x2292ee0>
142293ed7:     	xorl	%r14d, %r14d
142293eda:     	movl	0x6ec(%rbp), %ecx
142293ee0:     	movl	%ecx, 0x6ec(%rbp)
142293ee6:     	movq	0x6b8(%rbp), %r8
142293eed:     	movq	0x6c0(%rbp), %rcx
142293ef4:     	cmpb	$0x0, 0x300(%rbp)
142293efb:     	jne	0x142293c95 <.text+0x2292c95>
142293f01:     	movq	0x708(%rbp), %r11
142293f08:     	cmpb	$0x0, 0x308(%rbp)
142293f0f:     	je	0x1422943a6 <.text+0x22933a6>
142293f15:     	leaq	0x6e0(%rbp), %rax
142293f1c:     	movq	%rax, 0x3e8(%rbp)
142293f23:     	leaq	-0x21cba6a(%rip), %rax  # 0x1400c84c0 <.text+0xc74c0>
142293f2a:     	movq	%rax, 0x3f0(%rbp)
142293f31:     	leaq	0x6a8(%rbp), %rcx
142293f38:     	movq	%rcx, 0x3f8(%rbp)
142293f3f:     	movq	%rax, 0x400(%rbp)
142293f46:     	leaq	0xda7a53(%rip), %rax    # 0x14303b9a0
142293f4d:     	movq	%rax, 0x530(%rbp)
142293f54:     	movq	$0x2, 0x538(%rbp)
142293f5f:     	movq	%r15, 0x540(%rbp)
142293f66:     	movq	$0x2, 0x548(%rbp)
142293f71:     	movq	$0x0, 0x550(%rbp)
142293f7c:     	movq	%rsi, %rcx
142293f7f:     	movq	%rdi, %rdx
142293f82:     	callq	0x140d79f90 <.text+0xd78f90>
142293f87:     	movsd	(%rbx), %xmm0
142293f8b:     	movsd	0x8(%rbx), %xmm1
142293f90:     	movddup	%xmm0, %xmm2            # xmm2 = xmm0[0,0]
142293f94:     	mulps	%xmm13, %xmm1
142293f98:     	movddup	%xmm1, %xmm3            # xmm3 = xmm1[0,0]
142293f9c:     	addps	%xmm2, %xmm3
142293f9f:     	subps	%xmm1, %xmm0
142293fa2:     	shufps	$0xe4, %xmm3, %xmm0     # xmm0 = xmm0[0,1],xmm3[2,3]
142293fa6:     	movups	%xmm0, 0x3f0(%rbp)
142293fad:     	movl	$0x0, 0x3ec(%rbp)
142293fb7:     	movb	$0x3, 0x3e8(%rbp)
142293fbe:     	movb	$0x0, 0x614(%rbp)
142293fc5:     	movb	$0x1, 0x69b(%rbp)
142293fcc:     	movq	%rdi, %rcx
142293fcf:     	movq	%r15, %rdx
142293fd2:     	leaq	0x614(%rbp), %r8
142293fd9:     	callq	0x1409a8e10 <.text+0x9a7e10>
142293fde:     	movb	$0x0, 0x69b(%rbp)
142293fe5:     	movq	0x650(%rbp), %rcx
142293fec:     	movq	%rsi, %rdx
142293fef:     	movq	%rdi, %r8
142293ff2:     	callq	0x1422bd120 <.text+0x22bc120>
142293ff7:     	movq	%rax, 0x3e8(%rbp)
142293ffe:     	movq	%rdx, 0x3f0(%rbp)
142294005:     	movq	%r15, %rcx
142294008:     	callq	0x14092e680 <.text+0x92d680>
14229400d:     	testb	%al, %al
14229400f:     	je	0x142294066 <.text+0x2293066>
142294011:     	movq	0x6a8(%rbp), %rax
142294018:     	movq	%rax, 0x658(%rbp)
14229401f:     	movl	0x6f8(%rbp), %eax
142294025:     	movq	0x700(%rbp), %rcx
14229402c:     	movq	0x6d0(%rbp), %rdx
142294033:     	movl	%eax, (%rdx,%rcx,4)
142294036:     	movq	0x6b0(%rbp), %rax
14229403d:     	movb	$0x1, 0x20(%rax)
142294041:     	movl	$0x1, %eax
142294046:     	movq	0x5e0(%rbp), %rsi
14229404d:     	movq	0x700(%rbp), %rcx
142294054:     	movzbl	(%rsi,%rcx), %ecx
142294058:     	movb	%cl, 0x6ff(%rbp)
14229405e:     	movq	%rax, %r13
142294061:     	jmp	0x1422941e6 <.text+0x22931e6>
142294066:     	testq	%r13, %r13
142294069:     	movq	0x5e0(%rbp), %rsi
142294070:     	movzbl	0x717(%rbp), %r8d
142294078:     	je	0x142294187 <.text+0x2293187>
14229407e:     	movl	$0x1, %r13d
142294084:     	jmp	0x1422941e6 <.text+0x22931e6>
142294089:     	cmpb	$0x0, 0x6cc(%rbp)
142294090:     	je	0x1422940c7 <.text+0x22930c7>
142294092:     	testb	$0x1, (%rcx)
142294095:     	je	0x1422940c7 <.text+0x22930c7>
142294097:     	movq	0x8(%rcx), %rax
14229409b:     	cmpq	%rax, 0x6e0(%rbp)
1422940a2:     	jne	0x1422940c7 <.text+0x22930c7>
1422940a4:     	movl	0x6f8(%rbp), %eax
1422940aa:     	movq	0x700(%rbp), %rdx
1422940b1:     	movq	0x6d0(%rbp), %r8
1422940b8:     	movl	%eax, (%r8,%rdx,4)
1422940bc:     	movq	0x6b0(%rbp), %rax
1422940c3:     	movb	$0x1, 0x20(%rax)
1422940c7:     	movl	$0x0, %eax
1422940cc:     	movl	$0x0, %r13d
1422940d2:     	testb	%r9b, %r9b
1422940d5:     	jne	0x142293dcd <.text+0x2292dcd>
1422940db:     	movq	%r15, %rcx
1422940de:     	callq	0x14092e5f0 <.text+0x92d5f0>
1422940e3:     	testb	%al, %al
1422940e5:     	je	0x142294121 <.text+0x2293121>
1422940e7:     	movq	0x708(%rbp), %r11
1422940ee:     	movl	$0x0, 0x28(%r11)
1422940f6:     	movb	$0x1, %r14b
1422940f9:     	movb	$0x1, %al
1422940fb:     	movl	%eax, 0x6ec(%rbp)
142294101:     	movq	0x6b8(%rbp), %r8
142294108:     	movq	0x6c0(%rbp), %rcx
14229410f:     	cmpb	$0x0, 0x308(%rbp)
142294116:     	je	0x1422943a6 <.text+0x22933a6>
14229411c:     	jmp	0x142293f15 <.text+0x2292f15>
142294121:     	movq	0x660(%rbp), %rax
142294128:     	movl	0x98(%rax), %ecx
14229412e:     	movss	0x9c(%rax), %xmm1
142294136:     	callq	0x14093fa90 <.text+0x93ea90>
14229413b:     	testb	%al, %al
14229413d:     	je	0x14229415d <.text+0x229315d>
14229413f:     	movq	0x708(%rbp), %r11
142294146:     	cmpl	$0x1, (%r11)
14229414a:     	jne	0x142294388 <.text+0x2293388>
142294150:     	movq	0x8(%r11), %rax
142294154:     	cmpq	0x6e0(%rbp), %rax
14229415b:     	je	0x1422940e7 <.text+0x22930e7>
14229415d:     	xorl	%r14d, %r14d
142294160:     	movq	0x708(%rbp), %r11
142294167:     	movq	0x6b8(%rbp), %r8
14229416e:     	movq	0x6c0(%rbp), %rcx
142294175:     	cmpb	$0x0, 0x308(%rbp)
14229417c:     	je	0x1422943a6 <.text+0x22933a6>
142294182:     	jmp	0x142293f15 <.text+0x2292f15>
142294187:     	cmpb	$0x0, 0x6cc(%rbp)
14229418e:     	je	0x1422941d2 <.text+0x22931d2>
142294190:     	movq	0x708(%rbp), %rax
142294197:     	testb	$0x1, (%rax)
14229419a:     	je	0x1422941d2 <.text+0x22931d2>
14229419c:     	movq	0x708(%rbp), %rax
1422941a3:     	movq	0x8(%rax), %rax
1422941a7:     	cmpq	%rax, 0x6e0(%rbp)
1422941ae:     	jne	0x1422941d2 <.text+0x22931d2>
1422941b0:     	movl	0x6f8(%rbp), %eax
1422941b6:     	movq	0x700(%rbp), %rcx
1422941bd:     	movq	0x6d0(%rbp), %rdx
1422941c4:     	movl	%eax, (%rdx,%rcx,4)
1422941c7:     	movq	0x6b0(%rbp), %rax
1422941ce:     	movb	$0x1, 0x20(%rax)
1422941d2:     	movl	$0x0, %eax
1422941d7:     	movl	$0x0, %r13d
1422941dd:     	testb	%r8b, %r8b
1422941e0:     	jne	0x14229404d <.text+0x229304d>
1422941e6:     	movq	%r15, %rcx
1422941e9:     	callq	0x14092e5f0 <.text+0x92d5f0>
1422941ee:     	testb	%al, %al
1422941f0:     	je	0x142294233 <.text+0x2293233>
1422941f2:     	movq	0x700(%rbp), %rax
1422941f9:     	movzbl	(%rsi,%rax), %ecx
1422941fd:     	callq	0x140b06770 <.text+0xb05770>
142294202:     	movl	0x6f0(%rbp), %eax
142294208:     	movq	0x690(%rbp), %rcx
14229420f:     	movl	%eax, (%rcx)
142294211:     	movq	0x688(%rbp), %rax
142294218:     	movss	%xmm0, 0x3c(%rax)
14229421d:     	movb	$0x1, %r14b
142294220:     	movb	$0x1, %al
142294222:     	movq	%rax, 0x328(%rbp)
142294229:     	movb	$0x1, %al
14229422b:     	movl	%eax, 0x6ec(%rbp)
142294231:     	jmp	0x142294272 <.text+0x2293272>
142294233:     	movq	0x660(%rbp), %rax
14229423a:     	movl	0x98(%rax), %ecx
142294240:     	movss	0x9c(%rax), %xmm1
142294248:     	callq	0x14093fa90 <.text+0x93ea90>
14229424d:     	testb	%al, %al
14229424f:     	je	0x14229426f <.text+0x229326f>
142294251:     	movq	0x708(%rbp), %r11
142294258:     	cmpl	$0x1, (%r11)
14229425c:     	jne	0x1422947c3 <.text+0x22937c3>
142294262:     	movq	0x8(%r11), %rax
142294266:     	cmpq	0x6e0(%rbp), %rax
14229426d:     	je	0x1422941f2 <.text+0x22931f2>
14229426f:     	xorl	%r14d, %r14d
142294272:     	movq	0x708(%rbp), %r11
142294279:     	movq	0x6b8(%rbp), %r8
142294280:     	movq	0x6c0(%rbp), %rcx
142294287:     	cmpb	$0x0, 0x80(%rbp)
14229428e:     	jne	0x1422943b3 <.text+0x22933b3>
142294294:     	movq	0x6b0(%rbp), %rax
14229429b:     	cmpb	$0x0, 0x20(%rax)
14229429f:     	je	0x1422942e3 <.text+0x22932e3>
1422942a1:     	movq	0x6a8(%rbp), %rax
1422942a8:     	movq	%rax, 0x210(%rbp)
1422942af:     	movq	0x6e0(%rbp), %rax
1422942b6:     	movq	%rax, 0x230(%rbp)
1422942bd:     	movl	-0x50(%rbp), %eax
1422942c0:     	movl	%eax, 0x5f8(%rbp)
1422942c6:     	leaq	-0x4c(%rbp), %rax
1422942ca:     	movups	(%rax), %xmm0
1422942cd:     	movaps	%xmm0, 0x60(%rbp)
1422942d1:     	movl	0x10(%rax), %eax
1422942d4:     	movl	%eax, 0x70(%rbp)
1422942d7:     	movl	$0x1, %eax
1422942dc:     	movq	%rax, 0x350(%rbp)
1422942e3:     	testb	%r14b, %r14b
1422942e6:     	je	0x14229437c <.text+0x229337c>
1422942ec:     	movl	0x2b8(%rbp), %eax
1422942f2:     	movq	0xb0(%rbp), %rcx
1422942f9:     	movl	%eax, (%rcx)
1422942fb:     	movq	0x1e0(%rbp), %rax
142294302:     	movq	0x30(%rax), %rsi
142294306:     	movq	0x40(%rax), %rdi
14229430a:     	cmpq	0x20(%rax), %rsi
14229430e:     	jne	0x14229432a <.text+0x229332a>
142294310:     	movq	0xd8(%rbp), %rcx
142294317:     	leaq	0xda6faa(%rip), %rdx    # 0x14303b2c8
14229431e:     	callq	0x140562c30 <.text+0x561c30>
142294323:     	movq	0x708(%rbp), %r11
14229432a:     	movq	0x1e0(%rbp), %r9
142294331:     	movq	0x28(%r9), %rax
142294335:     	movq	%rsi, %rcx
142294338:     	shlq	$0x5, %rcx
14229433c:     	leaq	-0x4c(%rbp), %r8
142294340:     	movq	0x54(%r8), %rdx
142294344:     	movq	%rdx, 0x10(%rax,%rcx)
142294349:     	movups	0x44(%r8), %xmm0
14229434e:     	movups	%xmm0, (%rax,%rcx)
142294352:     	movq	%rdi, 0x18(%rax,%rcx)
142294357:     	incq	%rsi
14229435a:     	movq	%rsi, 0x30(%r9)
14229435e:     	incq	0x40(%r9)
142294362:     	movq	0x6b8(%rbp), %r8
142294369:     	movq	0x6c0(%rbp), %rcx
142294370:     	movq	%r13, 0x680(%rbp)
142294377:     	jmp	0x142291e04 <.text+0x2290e04>
14229437c:     	movq	%r13, 0x680(%rbp)
142294383:     	jmp	0x142291e04 <.text+0x2290e04>
142294388:     	xorl	%r14d, %r14d
14229438b:     	movq	0x6b8(%rbp), %r8
142294392:     	movq	0x6c0(%rbp), %rcx
142294399:     	cmpb	$0x0, 0x308(%rbp)
1422943a0:     	jne	0x142293f15 <.text+0x2292f15>
1422943a6:     	cmpb	$0x0, 0x80(%rbp)
1422943ad:     	je	0x142294294 <.text+0x2293294>
1422943b3:     	movq	%r13, %rbx
1422943b6:     	movq	0x700(%rbp), %rax
1422943bd:     	movq	0x4d0(%rbp), %rcx
1422943c4:     	movl	(%rcx,%rax,4), %esi
1422943c7:     	movq	%rdi, %rcx
1422943ca:     	callq	0x1409a8fb0 <.text+0x9a7fb0>
1422943cf:     	cmpl	$0x309, %esi            # imm = 0x309
1422943d5:     	jne	0x1422943fd <.text+0x22933fd>
1422943d7:     	movl	$0xa, 0x638(%rbp)
1422943e1:     	movq	%r15, %rcx
1422943e4:     	movq	%rdi, %rdx
1422943e7:     	leaq	0x638(%rbp), %r13
1422943ee:     	movq	%r13, %r8
1422943f1:     	callq	0x1409a8fd0 <.text+0x9a7fd0>
1422943f6:     	movl	$0xb, %eax
1422943fb:     	jmp	0x142294449 <.text+0x2293449>
1422943fd:     	movl	$0xc, 0x638(%rbp)
142294407:     	movl	0x5c(%rbp), %eax
14229440a:     	leaq	0x63c(%rbp), %rcx
142294411:     	movl	%eax, 0x10(%rcx)
142294414:     	movups	0x4c(%rbp), %xmm0
142294418:     	movups	%xmm0, (%rcx)
14229441b:     	movq	%r15, %rcx
14229441e:     	movq	%rdi, %rdx
142294421:     	leaq	0x638(%rbp), %r13
142294428:     	movq	%r13, %r8
14229442b:     	callq	0x1409a8fd0 <.text+0x9a7fd0>
142294430:     	movl	0x48(%rbp), %eax
142294433:     	leaq	0x534(%rbp), %rcx
14229443a:     	movl	%eax, 0x10(%rcx)
14229443d:     	movups	0x38(%rbp), %xmm0
142294441:     	movups	%xmm0, (%rcx)
142294444:     	movl	$0xd, %eax
142294449:     	movl	%eax, 0x530(%rbp)
14229444f:     	leaq	-0x50(%rbp), %rcx
142294453:     	movq	%r15, %rdx
142294456:     	movq	%rdi, %r8
142294459:     	callq	0x1409a9020 <.text+0x9a8020>
14229445e:     	leaq	0x6e0(%rbp), %rax
142294465:     	movq	%rax, 0x3e8(%rbp)
14229446c:     	leaq	-0x21cbfb3(%rip), %rax  # 0x1400c84c0 <.text+0xc74c0>
142294473:     	movq	%rax, 0x3f0(%rbp)
14229447a:     	leaq	0x6a8(%rbp), %rcx
142294481:     	movq	%rcx, 0x3f8(%rbp)
142294488:     	movq	%rax, 0x400(%rbp)
14229448f:     	leaq	0xda753a(%rip), %rax    # 0x14303b9d0
142294496:     	movq	%rax, 0x530(%rbp)
14229449d:     	movq	$0x2, 0x538(%rbp)
1422944a8:     	movq	%r15, 0x540(%rbp)
1422944af:     	movq	$0x2, 0x548(%rbp)
1422944ba:     	movq	$0x0, 0x550(%rbp)
1422944c5:     	movq	%r13, %rcx
1422944c8:     	movq	%rdi, %rdx
1422944cb:     	callq	0x140d79f90 <.text+0xd78f90>
1422944d0:     	movq	0x630(%rbp), %r14
1422944d7:     	movsd	(%r14), %xmm0
1422944dc:     	movsd	0x8(%r14), %xmm1
1422944e2:     	movddup	%xmm0, %xmm2            # xmm2 = xmm0[0,0]
1422944e6:     	mulps	%xmm13, %xmm1
1422944ea:     	movddup	%xmm1, %xmm3            # xmm3 = xmm1[0,0]
1422944ee:     	addps	%xmm2, %xmm3
1422944f1:     	subps	%xmm1, %xmm0
1422944f4:     	shufps	$0xe4, %xmm3, %xmm0     # xmm0 = xmm0[0,1],xmm3[2,3]
1422944f8:     	movups	%xmm0, 0x3f0(%rbp)
1422944ff:     	movl	$0x0, 0x3ec(%rbp)
142294509:     	movb	$0x3, 0x3e8(%rbp)
142294510:     	movb	$0x0, 0x614(%rbp)
142294517:     	movb	$0x1, 0x6dd(%rbp)
14229451e:     	movq	%rdi, %rcx
142294521:     	movq	%r15, %rdx
142294524:     	leaq	0x614(%rbp), %r8
14229452b:     	callq	0x1409a8e10 <.text+0x9a7e10>
142294530:     	movb	$0x1, 0x6dd(%rbp)
142294537:     	movq	%r15, %rcx
14229453a:     	movq	%rdi, %rdx
14229453d:     	xorl	%r8d, %r8d
142294540:     	callq	0x1409a8ee0 <.text+0x9a7ee0>
142294545:     	movb	$0x0, 0x6dd(%rbp)
14229454c:     	movq	0x650(%rbp), %rcx
142294553:     	leaq	0x638(%rbp), %rdx
14229455a:     	movq	%r15, %r8
14229455d:     	callq	0x1422bd120 <.text+0x22bc120>
142294562:     	movq	%rax, 0x530(%rbp)
142294569:     	movq	%rdx, 0x538(%rbp)
142294570:     	movq	%rdi, %rcx
142294573:     	callq	0x14092e680 <.text+0x92d680>
142294578:     	testb	%al, %al
14229457a:     	je	0x1422946c1 <.text+0x22936c1>
142294580:     	movq	0x6a8(%rbp), %rax
142294587:     	movq	%rax, 0x658(%rbp)
14229458e:     	movl	0x6f8(%rbp), %eax
142294594:     	movq	0x700(%rbp), %rcx
14229459b:     	movq	0x6d0(%rbp), %rdx
1422945a2:     	movl	%eax, (%rdx,%rcx,4)
1422945a5:     	movq	0x6b0(%rbp), %rax
1422945ac:     	movb	$0x1, 0x20(%rax)
1422945b0:     	movl	$0x1, %r13d
1422945b6:     	movq	0x708(%rbp), %r8
1422945bd:     	cmpl	$0x309, %esi            # imm = 0x309
1422945c3:     	jne	0x14229463b <.text+0x229363b>
1422945c5:     	movq	0x320(%rbp), %rax
1422945cc:     	movsd	(%rax), %xmm6
1422945d0:     	movq	0xf8(%rbp), %rax
1422945d7:     	movl	0x2c4(%rbp), %ecx
1422945dd:     	movl	%ecx, (%rax)
1422945df:     	movq	0x248(%rbp), %rax
1422945e6:     	movq	0x30(%rax), %rdi
1422945ea:     	movq	0x40(%rax), %rbx
1422945ee:     	cmpq	0x20(%rax), %rdi
1422945f2:     	jne	0x14229460e <.text+0x229360e>
1422945f4:     	movq	0x138(%rbp), %rcx
1422945fb:     	leaq	0xda6cc6(%rip), %rdx    # 0x14303b2c8
142294602:     	callq	0x140562630 <.text+0x561630>
142294607:     	movq	0x708(%rbp), %r8
14229460e:     	movq	0x248(%rbp), %rdx
142294615:     	movq	0x28(%rdx), %rax
142294619:     	movq	%rdi, %rcx
14229461c:     	shlq	$0x4, %rcx
142294620:     	movq	%rbx, (%rax,%rcx)
142294624:     	movlps	%xmm6, 0x8(%rax,%rcx)
142294629:     	incq	%rdi
14229462c:     	movq	%rdi, 0x30(%rdx)
142294630:     	incq	0x40(%rdx)
142294634:     	leaq	0x530(%rbp), %rdi
14229463b:     	movl	$0x1, %eax
142294640:     	movq	%rax, 0x1c8(%rbp)
142294647:     	cmpb	$0x4, 0x30(%r8)
14229464c:     	jne	0x1422946d5 <.text+0x22936d5>
142294652:     	movl	0x34(%r8), %ebx
142294656:     	movq	0x100(%rbp), %rax
14229465d:     	movl	0x2c8(%rbp), %ecx
142294663:     	movl	%ecx, (%rax)
142294665:     	movq	0x250(%rbp), %rax
14229466c:     	movq	0x30(%rax), %rdi
142294670:     	movq	0x40(%rax), %r15
142294674:     	cmpq	0x20(%rax), %rdi
142294678:     	jne	0x14229468d <.text+0x229368d>
14229467a:     	movq	0x140(%rbp), %rcx
142294681:     	leaq	0xda6c40(%rip), %rdx    # 0x14303b2c8
142294688:     	callq	0x140562630 <.text+0x561630>
14229468d:     	movq	0x250(%rbp), %rdx
142294694:     	movq	0x28(%rdx), %rax
142294698:     	movq	%rdi, %rcx
14229469b:     	shlq	$0x4, %rcx
14229469f:     	movq	%r15, (%rax,%rcx)
1422946a3:     	movl	%ebx, 0x8(%rax,%rcx)
1422946a7:     	incq	%rdi
1422946aa:     	movq	%rdi, 0x30(%rdx)
1422946ae:     	incq	0x40(%rdx)
1422946b2:     	movl	%esi, 0x4a8(%rbp)
1422946b8:     	leaq	0x530(%rbp), %rdi
1422946bf:     	jmp	0x14229472e <.text+0x229372e>
1422946c1:     	testq	%rbx, %rbx
1422946c4:     	movq	0x708(%rbp), %r8
1422946cb:     	je	0x1422946dd <.text+0x22936dd>
1422946cd:     	movl	$0x1, %r13d
1422946d3:     	jmp	0x14229472e <.text+0x229372e>
1422946d5:     	movl	%esi, 0x4a8(%rbp)
1422946db:     	jmp	0x14229472e <.text+0x229372e>
1422946dd:     	cmpb	$0x0, 0x6cc(%rbp)
1422946e4:     	je	0x14229471b <.text+0x229371b>
1422946e6:     	testb	$0x1, (%r8)
1422946ea:     	je	0x14229471b <.text+0x229371b>
1422946ec:     	movq	0x8(%r8), %rax
1422946f0:     	cmpq	%rax, 0x6e0(%rbp)
1422946f7:     	jne	0x14229471b <.text+0x229371b>
1422946f9:     	movl	0x6f8(%rbp), %eax
1422946ff:     	movq	0x700(%rbp), %rcx
142294706:     	movq	0x6d0(%rbp), %rdx
14229470d:     	movl	%eax, (%rdx,%rcx,4)
142294710:     	movq	0x6b0(%rbp), %rax
142294717:     	movb	$0x1, 0x20(%rax)
14229471b:     	movl	$0x0, %r13d
142294721:     	cmpb	$0x0, 0x717(%rbp)
142294728:     	jne	0x1422945bd <.text+0x22935bd>
14229472e:     	movq	%rdi, %rcx
142294731:     	callq	0x14092e5f0 <.text+0x92d5f0>
142294736:     	movb	$0x1, %r14b
142294739:     	movb	$0x1, %cl
14229473b:     	testb	%al, %al
14229473d:     	jne	0x142294792 <.text+0x2293792>
14229473f:     	movq	0x660(%rbp), %rax
142294746:     	movl	0x98(%rax), %ecx
14229474c:     	movss	0x9c(%rax), %xmm1
142294754:     	callq	0x14093fa90 <.text+0x93ea90>
142294759:     	testb	%al, %al
14229475b:     	je	0x142294789 <.text+0x2293789>
14229475d:     	movq	0x708(%rbp), %rax
142294764:     	cmpl	$0x1, (%rax)
142294767:     	jne	0x142294789 <.text+0x2293789>
142294769:     	movq	0x8(%rax), %rax
14229476d:     	cmpq	0x6e0(%rbp), %rax
142294774:     	sete	%r14b
142294778:     	movzbl	0x6ec(%rbp), %ecx
14229477f:     	movl	$0x1, %eax
142294784:     	cmovel	%eax, %ecx
142294787:     	jmp	0x142294792 <.text+0x2293792>
142294789:     	xorl	%r14d, %r14d
14229478c:     	movl	0x6ec(%rbp), %ecx
142294792:     	movl	%ecx, 0x6ec(%rbp)
142294798:     	movq	0x708(%rbp), %r11
14229479f:     	movq	0x6b8(%rbp), %r8
1422947a6:     	movq	0x6c0(%rbp), %rcx
1422947ad:     	movq	0x6b0(%rbp), %rax
1422947b4:     	cmpb	$0x0, 0x20(%rax)
1422947b8:     	jne	0x1422942a1 <.text+0x22932a1>
1422947be:     	jmp	0x1422942e3 <.text+0x22932e3>
1422947c3:     	xorl	%r14d, %r14d
1422947c6:     	movq	0x6b8(%rbp), %r8
1422947cd:     	movq	0x6c0(%rbp), %rcx
1422947d4:     	cmpb	$0x0, 0x80(%rbp)
1422947db:     	je	0x142294294 <.text+0x2293294>
1422947e1:     	jmp	0x1422943b3 <.text+0x22933b3>
1422947e6:     	nopw	%cs:(%rax,%rax)
1422947f0:     	movq	0x660(%rbp), %rax
1422947f7:     	movl	0x98(%rax), %ecx
1422947fd:     	movss	0x9c(%rax), %xmm1
142294805:     	callq	0x14093fa90 <.text+0x93ea90>
14229480a:     	testb	%al, %al
14229480c:     	leaq	0x3e8(%rbp), %rbx
142294813:     	je	0x142294b10 <.text+0x2293b10>
142294819:     	testb	$0x1, 0x5f4(%rbp)
142294820:     	movq	0x708(%rbp), %r9
142294827:     	movzbl	0x6de(%rbp), %r14d
14229482f:     	jne	0x142294871 <.text+0x2293871>
142294831:     	movq	0x668(%rbp), %rax
142294838:     	movl	0x6d8(%rbp), %ecx
14229483e:     	movl	%ecx, (%rax)
142294840:     	movq	0x670(%rbp), %rcx
142294847:     	leaq	0x10(%rbp), %rdx
14229484b:     	callq	0x140a86510 <.text+0xa85510>
142294850:     	movq	0x708(%rbp), %r9
142294857:     	xorl	%edi, %edi
142294859:     	movl	$0x0, 0x700(%rbp)
142294863:     	movl	$0x0, 0x5f4(%rbp)
14229486d:     	testb	%al, %al
14229486f:     	je	0x1422948e6 <.text+0x22938e6>
142294871:     	movq	0xb8(%rbp), %rax
142294878:     	movl	0x2bc(%rbp), %ecx
14229487e:     	movl	%ecx, (%rax)
142294880:     	movq	0x1e8(%rbp), %rax
142294887:     	movq	0x30(%rax), %rsi
14229488b:     	movq	0x40(%rax), %rdi
14229488f:     	cmpq	0x20(%rax), %rsi
142294893:     	jne	0x1422948af <.text+0x22938af>
142294895:     	movq	0xe8(%rbp), %rcx
14229489c:     	leaq	0xea22cd(%rip), %rdx    # 0x143136b70
1422948a3:     	callq	0x140562870 <.text+0x561870>
1422948a8:     	movq	0x708(%rbp), %r9
1422948af:     	movq	0x1e8(%rbp), %rcx
1422948b6:     	movq	0x28(%rcx), %rax
1422948ba:     	movq	%rdi, (%rax,%rsi,8)
1422948be:     	incq	%rsi
1422948c1:     	movq	%rsi, 0x30(%rcx)
1422948c5:     	incq	0x40(%rcx)
1422948c9:     	movb	$0x1, %al
1422948cb:     	movl	%eax, 0x5f4(%rbp)
1422948d1:     	movq	0x328(%rbp), %rax
1422948d8:     	movl	%eax, %edi
1422948da:     	movl	0x6ec(%rbp), %eax
1422948e0:     	movl	%eax, 0x700(%rbp)
1422948e6:     	movzbl	0x30(%r9), %eax
1422948eb:     	movb	$0x1, %r13b
1422948ee:     	cmpb	$0xa, %al
1422948f0:     	jne	0x142294b44 <.text+0x2293b44>
1422948f6:     	movb	$0x1, %r13b
1422948f9:     	testb	$0x1, 0x330(%rbp)
142294900:     	je	0x142294f00 <.text+0x2293f00>
142294906:     	movl	0x34(%r9), %esi
14229490a:     	movl	$0x0, 0x28(%r9)
142294912:     	testb	$0x1, 0x700(%rbp)
142294919:     	je	0x142294f24 <.text+0x2293f24>
14229491f:     	movzbl	0x89(%r9), %r15d
142294927:     	movl	0x2cc(%rbp), %eax
14229492d:     	movq	0x120(%rbp), %rcx
142294934:     	movl	%eax, (%rcx)
142294936:     	movq	0x270(%rbp), %rax
14229493d:     	movq	0x30(%rax), %rbx
142294941:     	movq	0x40(%rax), %r12
142294945:     	cmpq	0x20(%rax), %rbx
142294949:     	jne	0x142294965 <.text+0x2293965>
14229494b:     	movq	0x148(%rbp), %rcx
142294952:     	leaq	0xda696f(%rip), %rdx    # 0x14303b2c8
142294959:     	callq	0x140562630 <.text+0x561630>
14229495e:     	movq	0x708(%rbp), %r9
142294965:     	xorl	$0x1, %r15d
142294969:     	shlq	$0x20, %r15
14229496d:     	orq	%rsi, %r15
142294970:     	movabsq	$0x10000000000, %rax    # imm = 0x10000000000
14229497a:     	orq	%rax, %r15
14229497d:     	movq	0x270(%rbp), %rdx
142294984:     	movq	0x28(%rdx), %rax
142294988:     	movq	%rbx, %rcx
14229498b:     	shlq	$0x4, %rcx
14229498f:     	movq	%r12, (%rax,%rcx)
142294993:     	movq	%r15, 0x8(%rax,%rcx)
142294998:     	incq	%rbx
14229499b:     	movq	%rbx, 0x30(%rdx)
14229499f:     	incq	0x40(%rdx)
1422949a3:     	movq	0x668(%rbp), %rax
1422949aa:     	movl	0x6d8(%rbp), %ecx
1422949b0:     	movl	%ecx, (%rax)
1422949b2:     	movzbl	0x89(%r9), %ebx
1422949ba:     	movzbl	0x13a0248(%rip), %eax   # 0x143634c09
1422949c1:     	movl	$0x30, %ecx
1422949c6:     	movl	$0x8, %edx
1422949cb:     	movq	%r9, %r15
1422949ce:     	callq	0x140613c10 <.text+0x612c10>
1422949d3:     	testq	%rax, %rax
1422949d6:     	je	0x142296c78 <.text+0x2295c78>
1422949dc:     	movq	$0x1, (%rax)
1422949e3:     	movq	$0x1, 0x8(%rax)
1422949eb:     	movl	%esi, 0x10(%rax)
1422949ee:     	movb	%bl, 0x14(%rax)
1422949f1:     	xorb	$0x1, %bl
1422949f4:     	movb	%bl, 0x15(%rax)
1422949f7:     	movb	$0xc, 0x18(%rax)
1422949fb:     	movq	%rax, 0x538(%rbp)
142294a02:     	movq	$0x9, 0x530(%rbp)
142294a0d:     	movq	0x670(%rbp), %rcx
142294a14:     	leaq	0x530(%rbp), %rdx
142294a1b:     	leaq	0xda6aae(%rip), %r8     # 0x14303b4d0
142294a22:     	callq	0x140a84ec0 <.text+0xa83ec0>
142294a27:     	movzbl	0x30(%r15), %eax
142294a2c:     	xorl	%r13d, %r13d
142294a2f:     	leaq	0x3e8(%rbp), %rbx
142294a36:     	movq	%r15, %r9
142294a39:     	cmpb	$0x6, %al
142294a3b:     	je	0x142294b4c <.text+0x2293b4c>
142294a41:     	nopw	%cs:(%rax,%rax)
142294a50:     	movzbl	0x6ff(%rbp), %ecx
142294a57:     	cmpb	$0x9, 0x30(%r9)
142294a5c:     	jne	0x142294f00 <.text+0x2293f00>
142294a62:     	movl	0x34(%r9), %r15d
142294a66:     	movl	%edi, %ebx
142294a68:     	andb	$0x1, %bl
142294a6b:     	cmpb	$0x3, %cl
142294a6e:     	jne	0x142294d28 <.text+0x2293d28>
142294a74:     	movss	0x6c(%r9), %xmm6
142294a7a:     	movq	0x218(%rbp), %rax
142294a81:     	movl	0x4ac(%rbp), %ecx
142294a87:     	movl	%ecx, (%rax)
142294a89:     	movq	0x3d8(%rbp), %rax
142294a90:     	movq	0x30(%rax), %rsi
142294a94:     	movq	0x40(%rax), %rdi
142294a98:     	cmpq	0x20(%rax), %rsi
142294a9c:     	jne	0x142294ab8 <.text+0x2293ab8>
142294a9e:     	movq	0x278(%rbp), %rcx
142294aa5:     	leaq	0xda681c(%rip), %rdx    # 0x14303b2c8
142294aac:     	callq	0x140562db0 <.text+0x561db0>
142294ab1:     	movq	0x708(%rbp), %r9
142294ab8:     	movq	0x3d8(%rbp), %rdx
142294abf:     	movq	0x28(%rdx), %rax
142294ac3:     	leaq	(%rsi,%rsi,2), %rcx
142294ac7:     	movq	%rdi, (%rax,%rcx,8)
142294acb:     	movl	%r15d, 0x8(%rax,%rcx,8)
142294ad0:     	movss	%xmm6, 0xc(%rax,%rcx,8)
142294ad6:     	movb	%bl, 0x10(%rax,%rcx,8)
142294ada:     	incq	%rsi
142294add:     	movq	%rsi, 0x30(%rdx)
142294ae1:     	incq	0x40(%rdx)
142294ae5:     	leaq	0x3e8(%rbp), %rbx
142294aec:     	cmpl	$0x1, 0x1c8(%rbp)
142294af3:     	movl	%r13d, 0x6d0(%rbp)
142294afa:     	jne	0x142294f60 <.text+0x2293f60>
142294b00:     	jmp	0x142294f10 <.text+0x2293f10>
142294b05:     	nopw	%cs:(%rax,%rax)
142294b10:     	movq	0x328(%rbp), %rax
142294b17:     	movl	%eax, %edi
142294b19:     	movl	0x6ec(%rbp), %eax
142294b1f:     	movl	%eax, 0x700(%rbp)
142294b25:     	movq	0x708(%rbp), %r9
142294b2c:     	movzbl	0x6de(%rbp), %r14d
142294b34:     	movzbl	0x30(%r9), %eax
142294b39:     	movb	$0x1, %r13b
142294b3c:     	cmpb	$0xa, %al
142294b3e:     	je	0x1422948f6 <.text+0x22938f6>
142294b44:     	cmpb	$0x6, %al
142294b46:     	jne	0x142294a50 <.text+0x2293a50>
142294b4c:     	movl	0x34(%r9), %r15d
142294b50:     	movzbl	0x86(%r9), %ebx
142294b58:     	testb	$0x1, 0x340(%rbp)
142294b5f:     	je	0x142294e60 <.text+0x2293e60>
142294b65:     	movq	0x238(%rbp), %rax
142294b6c:     	movl	0x4b8(%rbp), %ecx
142294b72:     	movl	%ecx, (%rax)
142294b74:     	movq	0x3e0(%rbp), %rax
142294b7b:     	movq	0x30(%rax), %rsi
142294b7f:     	movq	0x40(%rax), %r12
142294b83:     	cmpq	0x20(%rax), %rsi
142294b87:     	jne	0x142294ba3 <.text+0x2293ba3>
142294b89:     	movq	0x298(%rbp), %rcx
142294b90:     	leaq	0xda6731(%rip), %rdx    # 0x14303b2c8
142294b97:     	callq	0x140562630 <.text+0x561630>
142294b9c:     	movq	0x708(%rbp), %r9
142294ba3:     	xorb	$0x1, %bl
142294ba6:     	movl	0x700(%rbp), %r10d
142294bad:     	movl	%r10d, %eax
142294bb0:     	andb	$0x1, %al
142294bb2:     	movzbl	%al, %eax
142294bb5:     	shlq	$0x28, %rax
142294bb9:     	movzbl	%bl, %ecx
142294bbc:     	shlq	$0x20, %rcx
142294bc0:     	orq	%rax, %rcx
142294bc3:     	orq	%r15, %rcx
142294bc6:     	movq	0x3e0(%rbp), %r8
142294bcd:     	movq	0x28(%r8), %rax
142294bd1:     	movq	%rsi, %rdx
142294bd4:     	shlq	$0x4, %rdx
142294bd8:     	movq	%r12, (%rax,%rdx)
142294bdc:     	movq	%rcx, 0x8(%rax,%rdx)
142294be1:     	incq	%rsi
142294be4:     	movq	%rsi, 0x30(%r8)
142294be8:     	incq	0x40(%r8)
142294bec:     	movl	$0x0, 0x28(%r9)
142294bf4:     	testb	$0x1, %r10b
142294bf8:     	je	0x142295ef3 <.text+0x2294ef3>
142294bfe:     	movq	0x668(%rbp), %rax
142294c05:     	movl	0x6d8(%rbp), %ecx
142294c0b:     	movl	%ecx, (%rax)
142294c0d:     	movzbl	0x86(%r9), %ebx
142294c15:     	movzbl	0x139ffed(%rip), %eax   # 0x143634c09
142294c1c:     	movl	$0x30, %ecx
142294c21:     	movl	$0x8, %edx
142294c26:     	callq	0x140613c10 <.text+0x612c10>
142294c2b:     	testq	%rax, %rax
142294c2e:     	je	0x142296c78 <.text+0x2295c78>
142294c34:     	movq	$0x1, (%rax)
142294c3b:     	movq	$0x1, 0x8(%rax)
142294c43:     	movl	%r15d, 0x10(%rax)
142294c47:     	movb	%bl, 0x14(%rax)
142294c4a:     	xorb	$0x1, %bl
142294c4d:     	movb	%bl, 0x15(%rax)
142294c50:     	movb	$0xa, 0x18(%rax)
142294c54:     	movq	%rax, 0x538(%rbp)
142294c5b:     	movq	$0x9, 0x530(%rbp)
142294c66:     	movq	0x670(%rbp), %rcx
142294c6d:     	leaq	0x530(%rbp), %rdx
142294c74:     	leaq	0xda686d(%rip), %r8     # 0x14303b4e8
142294c7b:     	callq	0x140a84ec0 <.text+0xa83ec0>
142294c80:     	xorl	%r13d, %r13d
142294c83:     	leaq	0x3e8(%rbp), %rbx
142294c8a:     	movq	0x708(%rbp), %r9
142294c91:     	movzbl	0x6ff(%rbp), %ecx
142294c98:     	cmpb	$0x9, 0x30(%r9)
142294c9d:     	je	0x142294a62 <.text+0x2293a62>
142294ca3:     	jmp	0x142294f00 <.text+0x2293f00>
142294ca8:     	movb	$0x1, %al
142294caa:     	movl	%eax, 0x6cc(%rbp)
142294cb0:     	testb	%cl, %cl
142294cb2:     	je	0x142291c9a <.text+0x2290c9a>
142294cb8:     	movq	0x20(%r11), %rcx
142294cbc:     	leaq	-0x1(%rcx), %r9
142294cc0:     	leaq	0x1(%rcx), %r10
142294cc4:     	jmp	0x142294cf7 <.text+0x2293cf7>
142294cc6:     	nopw	%cs:(%rax,%rax)
142294cd0:     	xorl	%edx, %edx
142294cd2:     	divl	%ecx
142294cd4:     	movq	%rdx, (%r8)
142294cd7:     	movq	0x48(%r11), %rax
142294cdb:     	cmpq	%rax, %rdx
142294cde:     	jae	0x142296c50 <.text+0x2295c50>
142294ce4:     	movq	0x40(%r11), %rax
142294ce8:     	leaq	(%rdx,%rdx,4), %rdx
142294cec:     	cmpb	$0x5, 0x18(%rax,%rdx,8)
142294cf1:     	jne	0x142291c9a <.text+0x2290c9a>
142294cf7:     	decq	%r10
142294cfa:     	je	0x142291c9a <.text+0x2290c9a>
142294d00:     	movq	(%r8), %rax
142294d03:     	addq	%r9, %rax
142294d06:     	movq	%rax, %rdx
142294d09:     	orq	%rcx, %rdx
142294d0c:     	shrq	$0x20, %rdx
142294d10:     	je	0x142294cd0 <.text+0x2293cd0>
142294d12:     	xorl	%edx, %edx
142294d14:     	divq	%rcx
142294d17:     	movq	%rdx, (%r8)
142294d1a:     	movq	0x48(%r11), %rax
142294d1e:     	cmpq	%rax, %rdx
142294d21:     	jb	0x142294ce4 <.text+0x2293ce4>
142294d23:     	jmp	0x142296c50 <.text+0x2295c50>
142294d28:     	callq	0x140b06770 <.text+0xb05770>
142294d2d:     	movaps	%xmm0, %xmm6
142294d30:     	movq	0x218(%rbp), %rax
142294d37:     	movl	0x4ac(%rbp), %ecx
142294d3d:     	movl	%ecx, (%rax)
142294d3f:     	movq	0x3d8(%rbp), %rax
142294d46:     	movq	0x30(%rax), %rsi
142294d4a:     	movq	0x40(%rax), %r12
142294d4e:     	cmpq	0x20(%rax), %rsi
142294d52:     	jne	0x142294d67 <.text+0x2293d67>
142294d54:     	movq	0x278(%rbp), %rcx
142294d5b:     	leaq	0xda6566(%rip), %rdx    # 0x14303b2c8
142294d62:     	callq	0x140562db0 <.text+0x561db0>
142294d67:     	movq	0x3d8(%rbp), %rdx
142294d6e:     	movq	0x28(%rdx), %rax
142294d72:     	leaq	(%rsi,%rsi,2), %rcx
142294d76:     	movq	%r12, (%rax,%rcx,8)
142294d7a:     	movl	%r15d, 0x8(%rax,%rcx,8)
142294d7f:     	movss	%xmm6, 0xc(%rax,%rcx,8)
142294d85:     	movb	%bl, 0x10(%rax,%rcx,8)
142294d89:     	incq	%rsi
142294d8c:     	movq	%rsi, 0x30(%rdx)
142294d90:     	incq	0x40(%rdx)
142294d94:     	testb	$0x1, %dil
142294d98:     	movq	0x708(%rbp), %r9
142294d9f:     	je	0x142294f3a <.text+0x2293f3a>
142294da5:     	movq	0x668(%rbp), %rax
142294dac:     	movl	0x6d8(%rbp), %ecx
142294db2:     	movl	%ecx, (%rax)
142294db4:     	movss	0x6c(%r9), %xmm15
142294dba:     	movzbl	0x6ff(%rbp), %ecx
142294dc1:     	callq	0x140b06770 <.text+0xb05770>
142294dc6:     	movaps	%xmm0, %xmm6
142294dc9:     	movzbl	0x139fe39(%rip), %eax   # 0x143634c09
142294dd0:     	movl	$0x30, %ecx
142294dd5:     	movl	$0x8, %edx
142294dda:     	callq	0x140613c10 <.text+0x612c10>
142294ddf:     	testq	%rax, %rax
142294de2:     	leaq	0x530(%rbp), %rdx
142294de9:     	leaq	0x3e8(%rbp), %rbx
142294df0:     	je	0x142296c78 <.text+0x2295c78>
142294df6:     	movq	$0x1, (%rax)
142294dfd:     	movq	$0x1, 0x8(%rax)
142294e05:     	movb	$0xb, 0x18(%rax)
142294e09:     	movl	%r15d, 0x1c(%rax)
142294e0d:     	movss	%xmm15, 0x20(%rax)
142294e13:     	movss	%xmm6, 0x24(%rax)
142294e18:     	movq	%rax, 0x538(%rbp)
142294e1f:     	movq	$0x9, 0x530(%rbp)
142294e2a:     	movq	0x670(%rbp), %rcx
142294e31:     	leaq	0xda66c8(%rip), %r8     # 0x14303b500
142294e38:     	callq	0x140a84ec0 <.text+0xa83ec0>
142294e3d:     	xorl	%r13d, %r13d
142294e40:     	movq	0x708(%rbp), %r9
142294e47:     	cmpl	$0x1, 0x1c8(%rbp)
142294e4e:     	movl	%r13d, 0x6d0(%rbp)
142294e55:     	jne	0x142294f60 <.text+0x2293f60>
142294e5b:     	jmp	0x142294f10 <.text+0x2293f10>
142294e60:     	movq	0x238(%rbp), %rax
142294e67:     	movl	0x4b8(%rbp), %ecx
142294e6d:     	movl	%ecx, (%rax)
142294e6f:     	movq	0x3e0(%rbp), %rax
142294e76:     	movq	0x30(%rax), %rsi
142294e7a:     	movq	0x40(%rax), %r12
142294e7e:     	cmpq	0x20(%rax), %rsi
142294e82:     	jne	0x142294e9e <.text+0x2293e9e>
142294e84:     	movq	0x298(%rbp), %rcx
142294e8b:     	leaq	0xda6436(%rip), %rdx    # 0x14303b2c8
142294e92:     	callq	0x140562630 <.text+0x561630>
142294e97:     	movq	0x708(%rbp), %r9
142294e9e:     	movzbl	%bl, %eax
142294ea1:     	movl	0x700(%rbp), %ecx
142294ea7:     	andb	$0x1, %cl
142294eaa:     	movzbl	%cl, %ecx
142294ead:     	shlq	$0x28, %rcx
142294eb1:     	shlq	$0x20, %rax
142294eb5:     	orq	%rcx, %rax
142294eb8:     	orq	%r15, %rax
142294ebb:     	movq	0x3e0(%rbp), %r8
142294ec2:     	movq	0x28(%r8), %rcx
142294ec6:     	movq	%rsi, %rdx
142294ec9:     	shlq	$0x4, %rdx
142294ecd:     	movq	%r12, (%rcx,%rdx)
142294ed1:     	movq	%rax, 0x8(%rcx,%rdx)
142294ed6:     	incq	%rsi
142294ed9:     	movq	%rsi, 0x30(%r8)
142294edd:     	incq	0x40(%r8)
142294ee1:     	leaq	0x3e8(%rbp), %rbx
142294ee8:     	movzbl	0x6ff(%rbp), %ecx
142294eef:     	cmpb	$0x9, 0x30(%r9)
142294ef4:     	je	0x142294a62 <.text+0x2293a62>
142294efa:     	nopw	(%rax,%rax)
142294f00:     	cmpl	$0x1, 0x1c8(%rbp)
142294f07:     	movl	%r13d, 0x6d0(%rbp)
142294f0e:     	jne	0x142294f60 <.text+0x2293f60>
142294f10:     	movl	$0x1, 0x28(%r9)
142294f18:     	movl	0x4a8(%rbp), %eax
142294f1e:     	movl	%eax, 0x2c(%r9)
142294f22:     	jmp	0x142294f71 <.text+0x2293f71>
142294f24:     	xorl	%r13d, %r13d
142294f27:     	movzbl	0x6ff(%rbp), %ecx
142294f2e:     	cmpb	$0x9, 0x30(%r9)
142294f33:     	jne	0x142294f00 <.text+0x2293f00>
142294f35:     	jmp	0x142294a62 <.text+0x2293a62>
142294f3a:     	xorl	%r13d, %r13d
142294f3d:     	leaq	0x3e8(%rbp), %rbx
142294f44:     	cmpl	$0x1, 0x1c8(%rbp)
142294f4b:     	movl	%r13d, 0x6d0(%rbp)
142294f52:     	je	0x142294f10 <.text+0x2293f10>
142294f54:     	nopw	%cs:(%rax,%rax)
142294f60:     	cmpb	$0x3, 0x6f7(%rbp)
142294f67:     	je	0x142294f71 <.text+0x2293f71>
142294f69:     	movl	$0x0, 0x28(%r9)
142294f71:     	movq	0x320(%rbp), %rsi
142294f78:     	movss	(%rsi), %xmm0
142294f7c:     	movss	0x4(%rsi), %xmm1
142294f81:     	subss	0x50(%r9), %xmm0
142294f87:     	subss	0x54(%r9), %xmm1
142294f8d:     	callq	0x140c90e20 <.text+0xc8fe20>
142294f92:     	movaps	%xmm1, %xmm0
142294f95:     	addss	%xmm7, %xmm0
142294f99:     	movaps	%xmm1, %xmm2
142294f9c:     	cmpltss	%xmm8, %xmm2
142294fa2:     	andps	%xmm2, %xmm0
142294fa5:     	andnps	%xmm1, %xmm2
142294fa8:     	orps	%xmm0, %xmm2
142294fab:     	movq	0x708(%rbp), %rax
142294fb2:     	movss	0x58(%rax), %xmm0
142294fb7:     	movq	0x708(%rbp), %rax
142294fbe:     	movss	0x5c(%rax), %xmm1
142294fc3:     	callq	0x1425f24d0 <.text+0x25f14d0>
142294fc8:     	movaps	%xmm0, %xmm1
142294fcb:     	movq	0x708(%rbp), %rax
142294fd2:     	movss	0x8c(%rax), %xmm0
142294fda:     	callq	0x140c90e70 <.text+0xc8fe70>
142294fdf:     	movq	0x708(%rbp), %r10
142294fe6:     	movsd	0x50(%r10), %xmm2
142294fec:     	unpcklps	%xmm1, %xmm0            # xmm0 = xmm0[0],xmm1[0],xmm0[1],xmm1[1]
142294fef:     	addps	%xmm2, %xmm0
142294ff2:     	movsd	(%rsi), %xmm1
142294ff6:     	subps	%xmm1, %xmm0
142294ff9:     	mulps	%xmm0, %xmm0
142294ffc:     	movshdup	%xmm0, %xmm1            # xmm1 = xmm0[1,1,3,3]
142295000:     	addss	%xmm0, %xmm1
142295004:     	xorps	%xmm0, %xmm0
142295007:     	sqrtss	%xmm1, %xmm0
14229500b:     	movss	0x90(%r10), %xmm1
142295014:     	movq	0xa0(%rbp), %rax
14229501b:     	mulss	0xe8(%rax), %xmm1
142295023:     	mulss	%xmm9, %xmm1
142295028:     	addss	%xmm10, %xmm1
14229502d:     	ucomiss	%xmm1, %xmm0
142295030:     	leaq	0x530(%rbp), %rdi
142295037:     	jbe	0x142295046 <.text+0x2294046>
142295039:     	testb	$0x1, 0x2f0(%rbp)
142295040:     	je	0x1422951fa <.text+0x22941fa>
142295046:     	movq	0x320(%rbp), %rsi
14229504d:     	movq	%rsi, 0x3e8(%rbp)
142295054:     	leaq	-0x101347b(%rip), %rax  # 0x141281be0 <.text+0x1280be0>
14229505b:     	movq	%rax, 0x3f0(%rbp)
142295062:     	leaq	0xda64bf(%rip), %rax    # 0x14303b528
142295069:     	movq	%rax, 0x530(%rbp)
142295070:     	movq	$0x1, 0x538(%rbp)
14229507b:     	movq	%rbx, 0x540(%rbp)
142295082:     	movq	$0x1, 0x548(%rbp)
14229508d:     	movq	$0x0, 0x550(%rbp)
142295098:     	leaq	0x638(%rbp), %r15
14229509f:     	movq	%r15, %rcx
1422950a2:     	movq	%rdi, %rdx
1422950a5:     	callq	0x140d79f90 <.text+0xd78f90>
1422950aa:     	movddup	(%rsi), %xmm0           # xmm0 = mem[0,0]
1422950ae:     	movq	%r15, %rsi
1422950b1:     	addps	%xmm11, %xmm0
1422950b5:     	movups	%xmm0, 0x3f0(%rbp)
1422950bc:     	movl	$0x3c23d70a, 0x3ec(%rbp) # imm = 0x3C23D70A
1422950c6:     	movb	$0x3, 0x3e8(%rbp)
1422950cd:     	movb	$0x0, -0x50(%rbp)
1422950d1:     	movb	$0x1, 0x6eb(%rbp)
1422950d8:     	movq	%rdi, %rcx
1422950db:     	movq	%rbx, %rdx
1422950de:     	leaq	-0x50(%rbp), %r15
1422950e2:     	movq	%r15, %r8
1422950e5:     	callq	0x1409a8e10 <.text+0x9a7e10>
1422950ea:     	movb	$0x1, 0x6a5(%rbp)
1422950f1:     	movq	%r15, %rcx
1422950f4:     	callq	0x1409a8fb0 <.text+0x9a7fb0>
1422950f9:     	movb	$0x0, 0x6a5(%rbp)
142295100:     	movq	%rbx, %rcx
142295103:     	movq	%rdi, %rdx
142295106:     	movq	%r15, %r8
142295109:     	callq	0x1409a8e80 <.text+0x9a7e80>
14229510e:     	movb	$0x0, 0x6eb(%rbp)
142295115:     	movq	0x650(%rbp), %rcx
14229511c:     	movq	%rsi, %rdx
14229511f:     	movq	%rbx, %r8
142295122:     	callq	0x1422bd120 <.text+0x22bc120>
142295127:     	movb	$0x1, %dil
14229512a:     	movq	0x708(%rbp), %r10
142295131:     	movzbl	0x30(%r10), %eax
142295136:     	cmpb	$0x2, %al
142295138:     	jne	0x142295220 <.text+0x2294220>
14229513e:     	movq	0x38(%r10), %r15
142295142:     	cmpb	$0x3, %r14b
142295146:     	jne	0x14229523b <.text+0x229423b>
14229514c:     	testb	%dil, %dil
14229514f:     	jne	0x142295400 <.text+0x2294400>
142295155:     	movl	0x80(%r10), %eax
14229515c:     	movl	%eax, 0x538(%rbp)
142295162:     	movq	0x78(%r10), %rax
142295166:     	movq	%rax, 0x530(%rbp)
14229516d:     	movl	0x4b0(%rbp), %eax
142295173:     	movq	0x220(%rbp), %rcx
14229517a:     	movl	%eax, (%rcx)
14229517c:     	movq	0x510(%rbp), %rax
142295183:     	movq	0x30(%rax), %rsi
142295187:     	movq	0x40(%rax), %rbx
14229518b:     	cmpq	0x20(%rax), %rsi
14229518f:     	jne	0x1422951ab <.text+0x22941ab>
142295191:     	movq	0x280(%rbp), %rcx
142295198:     	leaq	0xda6129(%rip), %rdx    # 0x14303b2c8
14229519f:     	callq	0x140562c30 <.text+0x561c30>
1422951a4:     	movq	0x708(%rbp), %r10
1422951ab:     	movl	0x700(%rbp), %eax
1422951b1:     	andb	$0x1, %al
1422951b3:     	movq	0x510(%rbp), %r9
1422951ba:     	movq	0x28(%r9), %rcx
1422951be:     	movq	%rsi, %rdx
1422951c1:     	shlq	$0x5, %rdx
1422951c5:     	movq	%rbx, (%rcx,%rdx)
1422951c9:     	movq	%r15, 0x8(%rcx,%rdx)
1422951ce:     	movq	0x530(%rbp), %r8
1422951d5:     	movq	%r8, 0x10(%rcx,%rdx)
1422951da:     	movl	0x538(%rbp), %r8d
1422951e1:     	movl	%r8d, 0x18(%rcx,%rdx)
1422951e6:     	movb	%al, 0x1c(%rcx,%rdx)
1422951ea:     	incq	%rsi
1422951ed:     	movq	%rsi, 0x30(%r9)
1422951f1:     	incq	0x40(%r9)
1422951f5:     	jmp	0x142295400 <.text+0x2294400>
1422951fa:     	movl	$0x0, 0x28(%r10)
142295202:     	xorl	%edi, %edi
142295204:     	movzbl	0x30(%r10), %eax
142295209:     	cmpb	$0x2, %al
14229520b:     	je	0x14229513e <.text+0x229413e>
142295211:     	nopw	%cs:(%rax,%rax)
142295220:     	cmpb	$0xb, %al
142295222:     	movq	0x500(%rbp), %r13
142295229:     	movq	0x508(%rbp), %r14
142295230:     	je	0x14229541b <.text+0x229441b>
142295236:     	jmp	0x142295600 <.text+0x2294600>
14229523b:     	movl	0x4b0(%rbp), %eax
142295241:     	movq	0x220(%rbp), %rcx
142295248:     	movl	%eax, (%rcx)
14229524a:     	movq	0x510(%rbp), %rax
142295251:     	movq	0x30(%rax), %rsi
142295255:     	movq	0x40(%rax), %rbx
142295259:     	cmpq	0x20(%rax), %rsi
14229525d:     	jne	0x142295279 <.text+0x2294279>
14229525f:     	movq	0x280(%rbp), %rcx
142295266:     	leaq	0xda605b(%rip), %rdx    # 0x14303b2c8
14229526d:     	callq	0x140562c30 <.text+0x561c30>
142295272:     	movq	0x708(%rbp), %r10
142295279:     	movl	0x700(%rbp), %r11d
142295280:     	movl	%r11d, %eax
142295283:     	andb	$0x1, %al
142295285:     	movq	0x510(%rbp), %r8
14229528c:     	movq	0x28(%r8), %rcx
142295290:     	movq	%rsi, %rdx
142295293:     	shlq	$0x5, %rdx
142295297:     	movq	%rbx, (%rcx,%rdx)
14229529b:     	movq	%r15, 0x8(%rcx,%rdx)
1422952a0:     	movb	%r14b, 0x10(%rcx,%rdx)
1422952a5:     	movzbl	0x6df(%rbp), %r9d
1422952ad:     	movb	%r9b, 0x11(%rcx,%rdx)
1422952b2:     	movzwl	0x626(%rbp), %r9d
1422952ba:     	movw	%r9w, 0x12(%rcx,%rdx)
1422952c0:     	movl	0x5fc(%rbp), %r9d
1422952c7:     	movl	%r9d, 0x14(%rcx,%rdx)
1422952cc:     	movl	0x600(%rbp), %r9d
1422952d3:     	movl	%r9d, 0x18(%rcx,%rdx)
1422952d8:     	movb	%al, 0x1c(%rcx,%rdx)
1422952dc:     	incq	%rsi
1422952df:     	movq	%rsi, 0x30(%r8)
1422952e3:     	incq	0x40(%r8)
1422952e7:     	testb	$0x1, %r11b
1422952eb:     	je	0x142295400 <.text+0x2294400>
1422952f1:     	cmpb	%r14b, 0x78(%r10)
1422952f5:     	jne	0x14229532d <.text+0x229432d>
1422952f7:     	cmpb	$0x2, %r14b
1422952fb:     	jne	0x142295400 <.text+0x2294400>
142295301:     	movl	0x5fc(%rbp), %eax
142295307:     	cmpl	%eax, 0x7c(%r10)
14229530b:     	jne	0x14229532d <.text+0x229432d>
14229530d:     	movzbl	0x6df(%rbp), %eax
142295314:     	cmpb	%al, 0x79(%r10)
142295318:     	jne	0x14229532d <.text+0x229432d>
14229531a:     	movl	0x600(%rbp), %eax
142295320:     	cmpl	%eax, 0x80(%r10)
142295327:     	je	0x142295400 <.text+0x2294400>
14229532d:     	leaq	0x78(%r10), %rax
142295331:     	movq	0x668(%rbp), %rcx
142295338:     	movl	0x6d8(%rbp), %edx
14229533e:     	movl	%edx, (%rcx)
142295340:     	movl	0x8(%rax), %ecx
142295343:     	movl	%ecx, 0x538(%rbp)
142295349:     	movq	(%rax), %rax
14229534c:     	movq	%rax, 0x530(%rbp)
142295353:     	movzbl	0x139f8af(%rip), %eax   # 0x143634c09
14229535a:     	movl	$0x30, %ecx
14229535f:     	movl	$0x8, %edx
142295364:     	callq	0x140613c10 <.text+0x612c10>
142295369:     	testq	%rax, %rax
14229536c:     	je	0x142296c78 <.text+0x2295c78>
142295372:     	movq	$0x1, (%rax)
142295379:     	movq	$0x1, 0x8(%rax)
142295381:     	movq	%r15, 0x10(%rax)
142295385:     	movq	0x530(%rbp), %rcx
14229538c:     	movq	%rcx, 0x18(%rax)
142295390:     	movl	0x538(%rbp), %ecx
142295396:     	movl	%ecx, 0x20(%rax)
142295399:     	movb	%r14b, 0x24(%rax)
14229539d:     	movzbl	0x6df(%rbp), %ecx
1422953a4:     	movb	%cl, 0x25(%rax)
1422953a7:     	movzwl	0x626(%rbp), %ecx
1422953ae:     	movw	%cx, 0x26(%rax)
1422953b2:     	movl	0x5fc(%rbp), %ecx
1422953b8:     	movl	%ecx, 0x28(%rax)
1422953bb:     	movl	0x600(%rbp), %ecx
1422953c1:     	movl	%ecx, 0x2c(%rax)
1422953c4:     	movq	%rax, 0x538(%rbp)
1422953cb:     	movq	$0x9, 0x530(%rbp)
1422953d6:     	movq	0x670(%rbp), %rcx
1422953dd:     	leaq	0x530(%rbp), %rdx
1422953e4:     	leaq	0xda614d(%rip), %r8     # 0x14303b538
1422953eb:     	callq	0x140a84ec0 <.text+0xa83ec0>
1422953f0:     	movq	0x708(%rbp), %r10
1422953f7:     	nopw	(%rax,%rax)
142295400:     	movzbl	0x30(%r10), %eax
142295405:     	cmpb	$0xb, %al
142295407:     	movq	0x500(%rbp), %r13
14229540e:     	movq	0x508(%rbp), %r14
142295415:     	jne	0x142295600 <.text+0x2294600>
14229541b:     	movl	0x34(%r10), %r15d
14229541f:     	cmpb	$0x4, 0x6f6(%rbp)
142295426:     	jne	0x1422954ad <.text+0x22944ad>
14229542c:     	testb	%dil, %dil
14229542f:     	jne	0x142295600 <.text+0x2294600>
142295435:     	movzbl	0x84(%r10), %ebx
14229543d:     	movl	0x4b4(%rbp), %eax
142295443:     	movq	0x228(%rbp), %rcx
14229544a:     	movl	%eax, (%rcx)
14229544c:     	movq	0x518(%rbp), %rax
142295453:     	movq	0x30(%rax), %rsi
142295457:     	movq	0x40(%rax), %r12
14229545b:     	cmpq	0x20(%rax), %rsi
14229545f:     	jne	0x14229547b <.text+0x229447b>
142295461:     	movq	0x290(%rbp), %rcx
142295468:     	leaq	0xda5e59(%rip), %rdx    # 0x14303b2c8
14229546f:     	callq	0x140562630 <.text+0x561630>
142295474:     	movq	0x708(%rbp), %r10
14229547b:     	shlq	$0x20, %rbx
14229547f:     	orq	%r15, %rbx
142295482:     	movq	0x518(%rbp), %rdx
142295489:     	movq	0x28(%rdx), %rax
14229548d:     	movq	%rsi, %rcx
142295490:     	shlq	$0x4, %rcx
142295494:     	movq	%r12, (%rax,%rcx)
142295498:     	movq	%rbx, 0x8(%rax,%rcx)
14229549d:     	incq	%rsi
1422954a0:     	movq	%rsi, 0x30(%rdx)
1422954a4:     	incq	0x40(%rdx)
1422954a8:     	jmp	0x142295600 <.text+0x2294600>
1422954ad:     	movl	0x4b4(%rbp), %eax
1422954b3:     	movq	0x228(%rbp), %rcx
1422954ba:     	movl	%eax, (%rcx)
1422954bc:     	movq	0x518(%rbp), %rax
1422954c3:     	movq	0x30(%rax), %rsi
1422954c7:     	movq	0x40(%rax), %rbx
1422954cb:     	cmpq	0x20(%rax), %rsi
1422954cf:     	jne	0x1422954eb <.text+0x22944eb>
1422954d1:     	movq	0x290(%rbp), %rcx
1422954d8:     	leaq	0xda5de9(%rip), %rdx    # 0x14303b2c8
1422954df:     	callq	0x140562630 <.text+0x561630>
1422954e4:     	movq	0x708(%rbp), %r10
1422954eb:     	movl	0x700(%rbp), %r9d
1422954f2:     	movl	%r9d, %eax
1422954f5:     	andb	$0x1, %al
1422954f7:     	movzbl	%al, %eax
1422954fa:     	shlq	$0x28, %rax
1422954fe:     	movzbl	0x6f6(%rbp), %ecx
142295505:     	shlq	$0x20, %rcx
142295509:     	orq	%rax, %rcx
14229550c:     	orq	%r15, %rcx
14229550f:     	movq	0x518(%rbp), %r8
142295516:     	movq	0x28(%r8), %rax
14229551a:     	movq	%rsi, %rdx
14229551d:     	shlq	$0x4, %rdx
142295521:     	movq	%rbx, (%rax,%rdx)
142295525:     	movq	%rcx, 0x8(%rax,%rdx)
14229552a:     	incq	%rsi
14229552d:     	movq	%rsi, 0x30(%r8)
142295531:     	incq	0x40(%r8)
142295535:     	testb	$0x1, %r9b
142295539:     	je	0x142295600 <.text+0x2294600>
14229553f:     	movzbl	0x6f6(%rbp), %eax
142295546:     	cmpb	%al, 0x84(%r10)
14229554d:     	je	0x142295600 <.text+0x2294600>
142295553:     	movq	0x668(%rbp), %rax
14229555a:     	movl	0x6d8(%rbp), %ecx
142295560:     	movl	%ecx, (%rax)
142295562:     	movzbl	0x84(%r10), %ebx
14229556a:     	movzbl	0x139f698(%rip), %eax   # 0x143634c09
142295571:     	movl	$0x30, %ecx
142295576:     	movl	$0x8, %edx
14229557b:     	callq	0x140613c10 <.text+0x612c10>
142295580:     	testq	%rax, %rax
142295583:     	je	0x142296c78 <.text+0x2295c78>
142295589:     	movq	$0x1, (%rax)
142295590:     	movq	$0x1, 0x8(%rax)
142295598:     	movl	%r15d, 0x10(%rax)
14229559c:     	movb	%bl, 0x14(%rax)
14229559f:     	movzbl	0x6f6(%rbp), %ebx
1422955a6:     	movb	%bl, 0x15(%rax)
1422955a9:     	movb	$0xe, 0x18(%rax)
1422955ad:     	movq	%rax, 0x538(%rbp)
1422955b4:     	movq	$0x9, 0x530(%rbp)
1422955bf:     	movq	0x670(%rbp), %rcx
1422955c6:     	leaq	0x530(%rbp), %rdx
1422955cd:     	leaq	0xda5f7c(%rip), %r8     # 0x14303b550
1422955d4:     	callq	0x140a84ec0 <.text+0xa83ec0>
1422955d9:     	movl	0x6f0(%rbp), %eax
1422955df:     	movq	0x690(%rbp), %rcx
1422955e6:     	movl	%eax, (%rcx)
1422955e8:     	movq	0x688(%rbp), %rax
1422955ef:     	movb	%bl, 0x14(%rax)
1422955f2:     	movq	0x708(%rbp), %r10
1422955f9:     	nopl	(%rax)
142295600:     	testb	$0x1, 0x1c0(%rbp)
142295607:     	je	0x1422957e0 <.text+0x22947e0>
14229560d:     	movzbl	0x30(%r10), %eax
142295612:     	leal	-0xb(%rax), %ecx
142295615:     	cmpb	$0x1, %cl
142295618:     	ja	0x1422957e5 <.text+0x22947e5>
14229561e:     	movl	0x34(%r10), %r15d
142295622:     	cmpl	$0x1, 0x4a4(%rbp)
142295629:     	jne	0x142295769 <.text+0x2294769>
14229562f:     	movl	0x4cc(%rbp), %eax
142295635:     	movq	0x288(%rbp), %rcx
14229563c:     	movl	%eax, (%rcx)
14229563e:     	movq	0x520(%rbp), %rax
142295645:     	movq	0x30(%rax), %rsi
142295649:     	movq	0x40(%rax), %rdi
14229564d:     	cmpq	0x20(%rax), %rsi
142295651:     	jne	0x14229566d <.text+0x229466d>
142295653:     	movq	0x2a0(%rbp), %rcx
14229565a:     	leaq	0xda5c67(%rip), %rdx    # 0x14303b2c8
142295661:     	callq	0x140562db0 <.text+0x561db0>
142295666:     	movq	0x708(%rbp), %r10
14229566d:     	movl	0x700(%rbp), %r11d
142295674:     	movl	%r11d, %eax
142295677:     	andb	$0x1, %al
142295679:     	movq	0x520(%rbp), %r8
142295680:     	movq	0x28(%r8), %rcx
142295684:     	leaq	(%rsi,%rsi,2), %rdx
142295688:     	movq	%rdi, (%rcx,%rdx,8)
14229568c:     	movl	%r15d, 0x8(%rcx,%rdx,8)
142295691:     	movl	0x604(%rbp), %r9d
142295698:     	movl	%r9d, 0xc(%rcx,%rdx,8)
14229569d:     	movb	%al, 0x10(%rcx,%rdx,8)
1422956a1:     	incq	%rsi
1422956a4:     	movq	%rsi, 0x30(%r8)
1422956a8:     	incq	0x40(%r8)
1422956ac:     	testb	$0x1, %r11b
1422956b0:     	je	0x1422957e0 <.text+0x22947e0>
1422956b6:     	movl	0x604(%rbp), %eax
1422956bc:     	cmpl	%eax, 0x74(%r10)
1422956c0:     	je	0x1422957e0 <.text+0x22947e0>
1422956c6:     	movq	0x668(%rbp), %rax
1422956cd:     	movl	0x6d8(%rbp), %ecx
1422956d3:     	movl	%ecx, (%rax)
1422956d5:     	movl	0x74(%r10), %esi
1422956d9:     	movzbl	0x139f529(%rip), %eax   # 0x143634c09
1422956e0:     	movl	$0x30, %ecx
1422956e5:     	movl	$0x8, %edx
1422956ea:     	callq	0x140613c10 <.text+0x612c10>
1422956ef:     	testq	%rax, %rax
1422956f2:     	je	0x142296c78 <.text+0x2295c78>
1422956f8:     	movq	$0x1, (%rax)
1422956ff:     	movq	$0x1, 0x8(%rax)
142295707:     	movb	$0xf, 0x18(%rax)
14229570b:     	movl	%r15d, 0x1c(%rax)
14229570f:     	movl	%esi, 0x20(%rax)
142295712:     	movl	0x604(%rbp), %esi
142295718:     	movl	%esi, 0x24(%rax)
14229571b:     	movq	%rax, 0x538(%rbp)
142295722:     	movq	$0x9, 0x530(%rbp)
14229572d:     	movq	0x670(%rbp), %rcx
142295734:     	leaq	0x530(%rbp), %rdx
14229573b:     	leaq	0xda5e26(%rip), %r8     # 0x14303b568
142295742:     	callq	0x140a84ec0 <.text+0xa83ec0>
142295747:     	movl	0x6f0(%rbp), %eax
14229574d:     	movq	0x690(%rbp), %rcx
142295754:     	movl	%eax, (%rcx)
142295756:     	movq	0x688(%rbp), %rax
14229575d:     	movl	%esi, 0x10(%rax)
142295760:     	movq	0x708(%rbp), %r10
142295767:     	jmp	0x1422957e0 <.text+0x22947e0>
142295769:     	testb	%dil, %dil
14229576c:     	jne	0x1422957e0 <.text+0x22947e0>
14229576e:     	movl	0x74(%r10), %edi
142295772:     	movl	0x4cc(%rbp), %eax
142295778:     	movq	0x288(%rbp), %rcx
14229577f:     	movl	%eax, (%rcx)
142295781:     	movq	0x520(%rbp), %rax
142295788:     	movq	0x30(%rax), %rsi
14229578c:     	movq	0x40(%rax), %rbx
142295790:     	cmpq	0x20(%rax), %rsi
142295794:     	jne	0x1422957b0 <.text+0x22947b0>
142295796:     	movq	0x2a0(%rbp), %rcx
14229579d:     	leaq	0xda5b24(%rip), %rdx    # 0x14303b2c8
1422957a4:     	callq	0x140562db0 <.text+0x561db0>
1422957a9:     	movq	0x708(%rbp), %r10
1422957b0:     	movq	0x520(%rbp), %rdx
1422957b7:     	movq	0x28(%rdx), %rax
1422957bb:     	leaq	(%rsi,%rsi,2), %rcx
1422957bf:     	movq	%rbx, (%rax,%rcx,8)
1422957c3:     	movl	%r15d, 0x8(%rax,%rcx,8)
1422957c8:     	movl	%edi, 0xc(%rax,%rcx,8)
1422957cc:     	movb	$0x0, 0x10(%rax,%rcx,8)
1422957d1:     	incq	%rsi
1422957d4:     	movq	%rsi, 0x30(%rdx)
1422957d8:     	incq	0x40(%rdx)
1422957dc:     	nopl	(%rax)
1422957e0:     	movzbl	0x30(%r10), %eax
1422957e5:     	testb	%al, %al
1422957e7:     	jne	0x142295d73 <.text+0x2294d73>
1422957ed:     	movq	0x38(%r10), %rdi
1422957f1:     	movq	0x208(%rbp), %rsi
1422957f8:     	movq	%rsi, %rcx
1422957fb:     	movq	%rdi, %rdx
1422957fe:     	leaq	0xda5d7b(%rip), %r8     # 0x14303b580
142295805:     	callq	0x140a91ef0 <.text+0xa90ef0>
14229580a:     	testq	%rax, %rax
14229580d:     	je	0x142296cb5 <.text+0x2295cb5>
142295813:     	movzbl	0x299(%rax), %r13d
14229581b:     	movq	%rsi, %rcx
14229581e:     	movq	%rdi, %rdx
142295821:     	leaq	0xda5d88(%rip), %r8     # 0x14303b5b0
142295828:     	callq	0x140a91ef0 <.text+0xa90ef0>
14229582d:     	testq	%rax, %rax
142295830:     	movq	0xc0(%rbp), %rbx
142295837:     	movq	0xc8(%rbp), %r15
14229583e:     	je	0x142296ca9 <.text+0x2295ca9>
142295844:     	movzbl	0x295(%rax), %r12d
14229584c:     	movq	%rsi, %rcx
14229584f:     	movq	%rdi, %rdx
142295852:     	leaq	0xda5d87(%rip), %r8     # 0x14303b5e0
142295859:     	callq	0x140a91ef0 <.text+0xa90ef0>
14229585e:     	testq	%rax, %rax
142295861:     	je	0x142296c9d <.text+0x2295c9d>
142295867:     	movss	0x290(%rax), %xmm6
14229586f:     	movq	%rsi, %rcx
142295872:     	movq	%rdi, %rdx
142295875:     	callq	0x140a922b0 <.text+0xa912b0>
14229587a:     	movq	%rax, %rcx
14229587d:     	shrq	$0x20, %rcx
142295881:     	jne	0x1422958ee <.text+0x22948ee>
142295883:     	movl	$0x0, 0x628(%rbp)
14229588d:     	movq	0x708(%rbp), %r10
142295894:     	cmpb	$0x3, 0x6f7(%rbp)
14229589b:     	movq	0x688(%rbp), %rcx
1422958a2:     	jne	0x1422959b7 <.text+0x22949b7>
1422958a8:     	movzbl	0x85(%r10), %eax
1422958b0:     	movb	%al, 0x6f7(%rbp)
1422958b6:     	cmpb	%al, %r13b
1422958b9:     	je	0x142295a72 <.text+0x2294a72>
1422958bf:     	movq	0x268(%rbp), %rax
1422958c6:     	movl	0x4c8(%rbp), %ecx
1422958cc:     	movl	%ecx, (%rax)
1422958ce:     	movq	0x370(%rbp), %rax
1422958d5:     	movq	0x30(%rax), %rbx
1422958d9:     	movq	0x40(%rax), %r15
1422958dd:     	xorl	%esi, %esi
1422958df:     	cmpq	0x20(%rax), %rbx
1422958e3:     	jne	0x142295a33 <.text+0x2294a33>
1422958e9:     	jmp	0x142295a19 <.text+0x2294a19>
1422958ee:     	movl	%eax, %eax
1422958f0:     	cmpq	0x10(%r15), %rax
1422958f4:     	movq	0x708(%rbp), %r10
1422958fb:     	jae	0x142295999 <.text+0x2294999>
142295901:     	movq	0x8(%r15), %rdx
142295905:     	leaq	(%rax,%rax,4), %rax
142295909:     	cmpl	%ecx, (%rdx,%rax,4)
14229590c:     	jne	0x142295999 <.text+0x2294999>
142295912:     	leaq	(%rdx,%rax,4), %rax
142295916:     	movl	0x4(%rax), %ecx
142295919:     	movl	$0xffffffff, %edx       # imm = 0xFFFFFFFF
14229591e:     	cmpq	%rdx, %rcx
142295921:     	je	0x142295999 <.text+0x2294999>
142295923:     	cmpq	%rcx, 0x38(%rbx)
142295927:     	jbe	0x142295999 <.text+0x2294999>
142295929:     	movq	0x28(%rbx), %rdx
14229592d:     	movl	%ecx, %r8d
142295930:     	shrl	$0x6, %r8d
142295934:     	movq	(%rdx,%r8,8), %rdx
142295938:     	btq	%rcx, %rdx
14229593c:     	jae	0x142295999 <.text+0x2294999>
14229593e:     	movl	0xc(%rax), %ecx
142295941:     	movl	0x10(%rax), %eax
142295944:     	movq	0x1a8(%r15), %rdx
14229594b:     	leaq	(%rcx,%rcx,8), %rcx
14229594f:     	movq	0x110(%rbx), %r8
142295956:     	movq	0x18(%rdx,%rcx,8), %r9
14229595b:     	movq	0x38(%rdx,%rcx,8), %rcx
142295960:     	movq	(%rcx,%r8,8), %rcx
142295964:     	notq	%rcx
142295967:     	leaq	(%rcx,%rcx,2), %rcx
14229596b:     	shlq	$0x4, %rcx
14229596f:     	movq	0x10(%r9,%rcx), %rcx
142295974:     	imulq	$0x58, %rax, %rax
142295978:     	movzbl	0x3d(%rcx,%rax), %eax
14229597d:     	movl	%eax, 0x628(%rbp)
142295983:     	cmpb	$0x3, 0x6f7(%rbp)
14229598a:     	movq	0x688(%rbp), %rcx
142295991:     	je	0x1422958a8 <.text+0x22948a8>
142295997:     	jmp	0x1422959b7 <.text+0x22949b7>
142295999:     	movl	$0x0, 0x628(%rbp)
1422959a3:     	cmpb	$0x3, 0x6f7(%rbp)
1422959aa:     	movq	0x688(%rbp), %rcx
1422959b1:     	je	0x1422958a8 <.text+0x22948a8>
1422959b7:     	testb	$0x1, 0x700(%rbp)
1422959be:     	je	0x1422959d9 <.text+0x22949d9>
1422959c0:     	movl	0x6f0(%rbp), %eax
1422959c6:     	movq	0x690(%rbp), %rdx
1422959cd:     	movl	%eax, (%rdx)
1422959cf:     	movzbl	0x6f7(%rbp), %eax
1422959d6:     	movb	%al, 0x42(%rcx)
1422959d9:     	movzbl	0x85(%r10), %r13d
1422959e1:     	movl	0x700(%rbp), %esi
1422959e7:     	andb	$0x1, %sil
1422959eb:     	movq	0x268(%rbp), %rax
1422959f2:     	movl	0x4c8(%rbp), %ecx
1422959f8:     	movl	%ecx, (%rax)
1422959fa:     	movq	0x370(%rbp), %rax
142295a01:     	movq	0x30(%rax), %rbx
142295a05:     	movq	0x40(%rax), %r15
142295a09:     	movl	$0x0, 0x6d0(%rbp)
142295a13:     	cmpq	0x20(%rax), %rbx
142295a17:     	jne	0x142295a33 <.text+0x2294a33>
142295a19:     	movq	0x108(%rbp), %rcx
142295a20:     	leaq	0xda58a1(%rip), %rdx    # 0x14303b2c8
142295a27:     	callq	0x140562db0 <.text+0x561db0>
142295a2c:     	movq	0x708(%rbp), %r10
142295a33:     	movq	0x370(%rbp), %rdx
142295a3a:     	movq	0x28(%rdx), %rax
142295a3e:     	leaq	(%rbx,%rbx,2), %rcx
142295a42:     	movq	%r15, (%rax,%rcx,8)
142295a46:     	movq	%rdi, 0x8(%rax,%rcx,8)
142295a4b:     	movb	%r13b, 0x10(%rax,%rcx,8)
142295a50:     	movzbl	0x6f7(%rbp), %r8d
142295a58:     	movb	%r8b, 0x11(%rax,%rcx,8)
142295a5d:     	movb	%sil, 0x12(%rax,%rcx,8)
142295a62:     	movb	%sil, 0x13(%rax,%rcx,8)
142295a67:     	incq	%rbx
142295a6a:     	movq	%rbx, 0x30(%rdx)
142295a6e:     	incq	0x40(%rdx)
142295a72:     	movl	0x5f0(%rbp), %eax
142295a78:     	cmpl	$0x1, %eax
142295a7b:     	movq	0x500(%rbp), %r14
142295a82:     	jne	0x142295b5a <.text+0x2294b5a>
142295a88:     	movss	0x70(%r10), %xmm6
142295a8e:     	movl	0x700(%rbp), %ebx
142295a94:     	andb	$0x1, %bl
142295a97:     	movl	0x4bc(%rbp), %eax
142295a9d:     	movq	0x240(%rbp), %rcx
142295aa4:     	movl	%eax, (%rcx)
142295aa6:     	movq	0x3d0(%rbp), %rax
142295aad:     	movq	0x30(%rax), %rsi
142295ab1:     	movq	0x40(%rax), %r15
142295ab5:     	cmpq	0x20(%rax), %rsi
142295ab9:     	jne	0x142295ad5 <.text+0x2294ad5>
142295abb:     	movq	0xf0(%rbp), %rcx
142295ac2:     	leaq	0xda57ff(%rip), %rdx    # 0x14303b2c8
142295ac9:     	callq	0x140562c30 <.text+0x561c30>
142295ace:     	movq	0x708(%rbp), %r10
142295ad5:     	movq	0x3d0(%rbp), %rdx
142295adc:     	movq	0x28(%rdx), %rax
142295ae0:     	movq	%rsi, %rcx
142295ae3:     	shlq	$0x5, %rcx
142295ae7:     	movq	%r15, (%rax,%rcx)
142295aeb:     	movq	%rdi, 0x8(%rax,%rcx)
142295af0:     	movss	%xmm6, 0x10(%rax,%rcx)
142295af6:     	movss	%xmm14, 0x14(%rax,%rcx)
142295afd:     	movb	%bl, 0x18(%rax,%rcx)
142295b01:     	incq	%rsi
142295b04:     	movq	%rsi, 0x30(%rdx)
142295b08:     	incq	0x40(%rdx)
142295b0c:     	movzbl	0x6a6(%rbp), %r13d
142295b14:     	cmpb	$0x2, %r13b
142295b18:     	jne	0x142295b97 <.text+0x2294b97>
142295b1a:     	movzbl	0x87(%r10), %r13d
142295b22:     	cmpb	%r13b, %r12b
142295b25:     	je	0x142295c52 <.text+0x2294c52>
142295b2b:     	movq	0x258(%rbp), %rax
142295b32:     	movl	0x4c0(%rbp), %ecx
142295b38:     	movl	%ecx, (%rax)
142295b3a:     	movq	0x360(%rbp), %rax
142295b41:     	movq	0x30(%rax), %rsi
142295b45:     	movq	0x40(%rax), %r15
142295b49:     	xorl	%ebx, %ebx
142295b4b:     	cmpq	0x20(%rax), %rsi
142295b4f:     	je	0x142295c07 <.text+0x2294c07>
142295b55:     	jmp	0x142295c21 <.text+0x2294c21>
142295b5a:     	movss	0x70(%r10), %xmm14
142295b60:     	ucomiss	%xmm14, %xmm6
142295b64:     	jne	0x142295b68 <.text+0x2294b68>
142295b66:     	jnp	0x142295b0c <.text+0x2294b0c>
142295b68:     	movl	0x4bc(%rbp), %eax
142295b6e:     	movq	0x240(%rbp), %rcx
142295b75:     	movl	%eax, (%rcx)
142295b77:     	movq	0x3d0(%rbp), %rax
142295b7e:     	movq	0x30(%rax), %rsi
142295b82:     	movq	0x40(%rax), %r15
142295b86:     	xorl	%ebx, %ebx
142295b88:     	cmpq	0x20(%rax), %rsi
142295b8c:     	je	0x142295abb <.text+0x2294abb>
142295b92:     	jmp	0x142295ad5 <.text+0x2294ad5>
142295b97:     	testb	$0x1, 0x700(%rbp)
142295b9e:     	movq	0x688(%rbp), %rdx
142295ba5:     	je	0x142295bce <.text+0x2294bce>
142295ba7:     	movl	%r13d, %eax
142295baa:     	movl	0x6f0(%rbp), %ecx
142295bb0:     	movq	0x690(%rbp), %r8
142295bb7:     	movl	%ecx, (%r8)
142295bba:     	andb	$0x1, %al
142295bbc:     	movl	%eax, %r13d
142295bbf:     	movb	%al, 0x40(%rdx)
142295bc2:     	movl	0x68(%r10), %eax
142295bc6:     	movl	%ecx, (%r8)
142295bc9:     	movl	%eax, 0x18(%rdx)
142295bcc:     	jmp	0x142295bd2 <.text+0x2294bd2>
142295bce:     	andb	$0x1, %r13b
142295bd2:     	movzbl	0x87(%r10), %r12d
142295bda:     	movl	0x700(%rbp), %ebx
142295be0:     	andb	$0x1, %bl
142295be3:     	movq	0x258(%rbp), %rax
142295bea:     	movl	0x4c0(%rbp), %ecx
142295bf0:     	movl	%ecx, (%rax)
142295bf2:     	movq	0x360(%rbp), %rax
142295bf9:     	movq	0x30(%rax), %rsi
142295bfd:     	movq	0x40(%rax), %r15
142295c01:     	cmpq	0x20(%rax), %rsi
142295c05:     	jne	0x142295c21 <.text+0x2294c21>
142295c07:     	movq	0x110(%rbp), %rcx
142295c0e:     	leaq	0xda56b3(%rip), %rdx    # 0x14303b2c8
142295c15:     	callq	0x140562db0 <.text+0x561db0>
142295c1a:     	movq	0x708(%rbp), %r10
142295c21:     	movq	0x360(%rbp), %rdx
142295c28:     	movq	0x28(%rdx), %rax
142295c2c:     	leaq	(%rsi,%rsi,2), %rcx
142295c30:     	movq	%r15, (%rax,%rcx,8)
142295c34:     	movq	%rdi, 0x8(%rax,%rcx,8)
142295c39:     	movb	%r12b, 0x10(%rax,%rcx,8)
142295c3e:     	movb	%r13b, 0x11(%rax,%rcx,8)
142295c43:     	movb	%bl, 0x12(%rax,%rcx,8)
142295c47:     	incq	%rsi
142295c4a:     	movq	%rsi, 0x30(%rdx)
142295c4e:     	incq	0x40(%rdx)
142295c52:     	movzbl	0x6a7(%rbp), %r12d
142295c5a:     	cmpb	$0x2, %r12b
142295c5e:     	movq	0x688(%rbp), %rcx
142295c65:     	jne	0x142295c87 <.text+0x2294c87>
142295c67:     	movzbl	0x88(%r10), %r12d
142295c6f:     	cmpb	%r12b, 0x628(%rbp)
142295c76:     	movq	%r14, %r13
142295c79:     	jne	0x142295cac <.text+0x2294cac>
142295c7b:     	movq	0x508(%rbp), %r14
142295c82:     	jmp	0x142295d73 <.text+0x2294d73>
142295c87:     	testb	$0x1, 0x700(%rbp)
142295c8e:     	movq	%r14, %r13
142295c91:     	je	0x142295cdb <.text+0x2294cdb>
142295c93:     	movl	0x6f0(%rbp), %eax
142295c99:     	movq	0x690(%rbp), %rdx
142295ca0:     	movl	%eax, (%rdx)
142295ca2:     	andb	$0x1, %r12b
142295ca6:     	movb	%r12b, 0x41(%rcx)
142295caa:     	jmp	0x142295cdf <.text+0x2294cdf>
142295cac:     	movq	0x260(%rbp), %rax
142295cb3:     	movl	0x4c4(%rbp), %ecx
142295cb9:     	movl	%ecx, (%rax)
142295cbb:     	movq	0x368(%rbp), %rax
142295cc2:     	movq	0x30(%rax), %rsi
142295cc6:     	movq	0x40(%rax), %r15
142295cca:     	xorl	%ebx, %ebx
142295ccc:     	cmpq	0x20(%rax), %rsi
142295cd0:     	movq	0x508(%rbp), %r14
142295cd7:     	je	0x142295d21 <.text+0x2294d21>
142295cd9:     	jmp	0x142295d3b <.text+0x2294d3b>
142295cdb:     	andb	$0x1, %r12b
142295cdf:     	movq	0x508(%rbp), %r14
142295ce6:     	movzbl	0x88(%r10), %eax
142295cee:     	movl	%eax, 0x628(%rbp)
142295cf4:     	movl	0x700(%rbp), %ebx
142295cfa:     	andb	$0x1, %bl
142295cfd:     	movq	0x260(%rbp), %rax
142295d04:     	movl	0x4c4(%rbp), %ecx
142295d0a:     	movl	%ecx, (%rax)
142295d0c:     	movq	0x368(%rbp), %rax
142295d13:     	movq	0x30(%rax), %rsi
142295d17:     	movq	0x40(%rax), %r15
142295d1b:     	cmpq	0x20(%rax), %rsi
142295d1f:     	jne	0x142295d3b <.text+0x2294d3b>
142295d21:     	movq	0x118(%rbp), %rcx
142295d28:     	leaq	0xda5599(%rip), %rdx    # 0x14303b2c8
142295d2f:     	callq	0x140562db0 <.text+0x561db0>
142295d34:     	movq	0x708(%rbp), %r10
142295d3b:     	movq	0x368(%rbp), %rdx
142295d42:     	movq	0x28(%rdx), %rax
142295d46:     	leaq	(%rsi,%rsi,2), %rcx
142295d4a:     	movq	%r15, (%rax,%rcx,8)
142295d4e:     	movq	%rdi, 0x8(%rax,%rcx,8)
142295d53:     	movl	0x628(%rbp), %r8d
142295d5a:     	movb	%r8b, 0x10(%rax,%rcx,8)
142295d5f:     	movb	%r12b, 0x11(%rax,%rcx,8)
142295d64:     	movb	%bl, 0x12(%rax,%rcx,8)
142295d68:     	incq	%rsi
142295d6b:     	movq	%rsi, 0x30(%rdx)
142295d6f:     	incq	0x40(%rdx)
142295d73:     	movl	0x68(%r10), %eax
142295d77:     	cmpb	$0x0, 0x28(%r10)
142295d7c:     	movl	%eax, %edi
142295d7e:     	je	0x142295d84 <.text+0x2294d84>
142295d80:     	movl	0x2c(%r10), %edi
142295d84:     	cmpl	$0x309, %edi            # imm = 0x309
142295d8a:     	jne	0x142295de0 <.text+0x2294de0>
142295d8c:     	movb	$0x1, %al
142295d8e:     	testb	$0x1, 0x700(%rbp)
142295d95:     	je	0x1422919a8 <.text+0x22909a8>
142295d9b:     	movq	0xa8(%rbp), %rax
142295da2:     	movl	0x2b4(%rbp), %ecx
142295da8:     	movl	%ecx, (%rax)
142295daa:     	movq	0x1d8(%rbp), %rax
142295db1:     	movq	0x30(%rax), %rsi
142295db5:     	movq	0x40(%rax), %rdi
142295db9:     	cmpq	0x20(%rax), %rsi
142295dbd:     	jne	0x142291980 <.text+0x2290980>
142295dc3:     	movq	0xd0(%rbp), %rcx
142295dca:     	leaq	0xda54f7(%rip), %rdx    # 0x14303b2c8
142295dd1:     	callq	0x140562630 <.text+0x561630>
142295dd6:     	jmp	0x142291980 <.text+0x2290980>
142295ddb:     	nopl	(%rax,%rax)
142295de0:     	cmpl	%edi, %eax
142295de2:     	setne	%r15b
142295de6:     	andb	0x700(%rbp), %r15b
142295ded:     	movzbl	0x30(%r10), %eax
142295df2:     	leaq	0xda5c1f(%rip), %rcx    # 0x14303ba18
142295df9:     	movslq	(%rcx,%rax,4), %rax
142295dfd:     	addq	%rcx, %rax
142295e00:     	jmpq	*%rax
142295e02:     	movl	0x34(%r10), %r12d
142295e06:     	movq	0xe0(%rbp), %rax
142295e0d:     	movl	0x2c0(%rbp), %ecx
142295e13:     	movl	%ecx, (%rax)
142295e15:     	movq	0x358(%rbp), %rax
142295e1c:     	movq	0x30(%rax), %rsi
142295e20:     	movq	0x40(%rax), %rbx
142295e24:     	cmpq	0x20(%rax), %rsi
142295e28:     	jne	0x142295e44 <.text+0x2294e44>
142295e2a:     	movq	0x128(%rbp), %rcx
142295e31:     	leaq	0xda5490(%rip), %rdx    # 0x14303b2c8
142295e38:     	callq	0x140562db0 <.text+0x561db0>
142295e3d:     	movq	0x708(%rbp), %r10
142295e44:     	movl	0x700(%rbp), %r8d
142295e4b:     	movl	0x6d0(%rbp), %r9d
142295e52:     	andb	%r8b, %r9b
142295e55:     	movq	0x358(%rbp), %rdx
142295e5c:     	movq	0x28(%rdx), %rax
142295e60:     	leaq	(%rsi,%rsi,2), %rcx
142295e64:     	movq	%rbx, (%rax,%rcx,8)
142295e68:     	movl	%r12d, 0x8(%rax,%rcx,8)
142295e6d:     	movl	%edi, 0xc(%rax,%rcx,8)
142295e71:     	movb	%r9b, 0x10(%rax,%rcx,8)
142295e76:     	incq	%rsi
142295e79:     	movq	%rsi, 0x30(%rdx)
142295e7d:     	incq	0x40(%rdx)
142295e81:     	movb	$0x1, %al
142295e83:     	testb	$0x1, %r8b
142295e87:     	je	0x1422919a8 <.text+0x22909a8>
142295e8d:     	movzbl	0x30(%r10), %eax
142295e92:     	addl	$-0x3, %eax
142295e95:     	cmpl	$0x8, %eax
142295e98:     	ja	0x1422966d7 <.text+0x22956d7>
142295e9e:     	leaq	0xda5ba7(%rip), %rcx    # 0x14303ba4c
142295ea5:     	movslq	(%rcx,%rax,4), %rax
142295ea9:     	addq	%rcx, %rax
142295eac:     	jmpq	*%rax
142295eae:     	cmpl	$0x3e7, %edi            # imm = 0x3E7
142295eb4:     	je	0x142295ec9 <.text+0x2294ec9>
142295eb6:     	testl	%edi, %edi
142295eb8:     	jne	0x142296816 <.text+0x2295816>
142295ebe:     	cmpb	$0x0, 0x31(%r10)
142295ec3:     	jne	0x142296816 <.text+0x2295816>
142295ec9:     	movl	0x6f0(%rbp), %eax
142295ecf:     	movq	0x690(%rbp), %rcx
142295ed6:     	movl	%eax, (%rcx)
142295ed8:     	movq	0x688(%rbp), %rax
142295edf:     	movl	$0x0, (%rax)
142295ee5:     	testb	%r15b, %r15b
142295ee8:     	je	0x142295d9b <.text+0x2294d9b>
142295eee:     	jmp	0x1422966e0 <.text+0x22956e0>
142295ef3:     	xorl	%r13d, %r13d
142295ef6:     	leaq	0x3e8(%rbp), %rbx
142295efd:     	movzbl	0x6ff(%rbp), %ecx
142295f04:     	cmpb	$0x9, 0x30(%r9)
142295f09:     	je	0x142294a62 <.text+0x2293a62>
142295f0f:     	jmp	0x142294f00 <.text+0x2293f00>
142295f14:     	movq	0x38(%r10), %rbx
142295f18:     	movq	%rbx, 0x3e8(%rbp)
142295f1f:     	movq	0x130(%rbp), %rax
142295f26:     	movl	0x2d0(%rbp), %ecx
142295f2c:     	movl	%ecx, (%rax)
142295f2e:     	movq	0x378(%rbp), %rax
142295f35:     	movq	0x30(%rax), %rsi
142295f39:     	movq	0x40(%rax), %r12
142295f3d:     	cmpq	0x20(%rax), %rsi
142295f41:     	jne	0x142295f5d <.text+0x2294f5d>
142295f43:     	movq	0x178(%rbp), %rcx
142295f4a:     	leaq	0xda5377(%rip), %rdx    # 0x14303b2c8
142295f51:     	callq	0x140562db0 <.text+0x561db0>
142295f56:     	movq	0x708(%rbp), %r10
142295f5d:     	movl	0x700(%rbp), %r9d
142295f64:     	movl	%r9d, %eax
142295f67:     	andb	$0x1, %al
142295f69:     	movq	0x378(%rbp), %r8
142295f70:     	movq	0x28(%r8), %rcx
142295f74:     	leaq	(%rsi,%rsi,2), %rdx
142295f78:     	movq	%r12, (%rcx,%rdx,8)
142295f7c:     	movq	%rbx, 0x8(%rcx,%rdx,8)
142295f81:     	movl	%edi, 0x10(%rcx,%rdx,8)
142295f85:     	movb	%al, 0x14(%rcx,%rdx,8)
142295f89:     	incq	%rsi
142295f8c:     	movq	%rsi, 0x30(%r8)
142295f90:     	incq	0x40(%r8)
142295f94:     	testb	$0x1, %r9b
142295f98:     	je	0x142296010 <.text+0x2295010>
142295f9a:     	cmpb	$0x7, 0x30(%r10)
142295f9f:     	movl	0x6f0(%rbp), %eax
142295fa5:     	movq	0x690(%rbp), %rcx
142295fac:     	movl	%eax, (%rcx)
142295fae:     	jne	0x1422965dc <.text+0x22955dc>
142295fb4:     	movq	0x688(%rbp), %rsi
142295fbb:     	movl	%edi, 0x24(%rsi)
142295fbe:     	leaq	0x3e8(%rbp), %rcx
142295fc5:     	callq	0x1400885d0 <.text+0x875d0>
142295fca:     	movq	0x180(%rbp), %rcx
142295fd1:     	movq	0x10(%rcx), %rdx
142295fd5:     	cmpq	%rdx, %rax
142295fd8:     	jae	0x142296d0f <.text+0x2295d0f>
142295fde:     	movq	0x8(%rcx), %rcx
142295fe2:     	movl	(%rcx,%rax,4), %eax
142295fe5:     	movl	%eax, 0x1c(%rsi)
142295fe8:     	movl	0x6f0(%rbp), %eax
142295fee:     	movq	0x690(%rbp), %rcx
142295ff5:     	movl	%eax, (%rcx)
142295ff7:     	movb	$0x2, 0x42(%rsi)
142295ffb:     	movq	0x708(%rbp), %r10
142296002:     	testb	%r15b, %r15b
142296005:     	je	0x142295d9b <.text+0x2294d9b>
14229600b:     	jmp	0x1422965ef <.text+0x22955ef>
142296010:     	movb	$0x1, %al
142296012:     	jmp	0x1422919a8 <.text+0x22909a8>
142296017:     	movq	0x38(%r10), %rbx
14229601b:     	movq	0x158(%rbp), %rax
142296022:     	movl	0x2d8(%rbp), %ecx
142296028:     	movl	%ecx, (%rax)
14229602a:     	movq	0x388(%rbp), %rax
142296031:     	movq	0x30(%rax), %rsi
142296035:     	movq	0x40(%rax), %r12
142296039:     	cmpq	0x20(%rax), %rsi
14229603d:     	jne	0x142296059 <.text+0x2295059>
14229603f:     	movq	0x1a0(%rbp), %rcx
142296046:     	leaq	0xda527b(%rip), %rdx    # 0x14303b2c8
14229604d:     	callq	0x140562db0 <.text+0x561db0>
142296052:     	movq	0x708(%rbp), %r10
142296059:     	movl	0x700(%rbp), %r9d
142296060:     	movl	%r9d, %eax
142296063:     	andb	$0x1, %al
142296065:     	movq	0x388(%rbp), %r8
14229606c:     	movq	0x28(%r8), %rcx
142296070:     	leaq	(%rsi,%rsi,2), %rdx
142296074:     	movq	%r12, (%rcx,%rdx,8)
142296078:     	movq	%rbx, 0x8(%rcx,%rdx,8)
14229607d:     	movl	%edi, 0x10(%rcx,%rdx,8)
142296081:     	movb	%al, 0x14(%rcx,%rdx,8)
142296085:     	incq	%rsi
142296088:     	movq	%rsi, 0x30(%r8)
14229608c:     	incq	0x40(%r8)
142296090:     	movb	$0x1, %al
142296092:     	testb	$0x1, %r9b
142296096:     	je	0x1422919a8 <.text+0x22909a8>
14229609c:     	movl	0x6f0(%rbp), %eax
1422960a2:     	movq	0x690(%rbp), %rcx
1422960a9:     	movl	%eax, (%rcx)
1422960ab:     	movq	0x688(%rbp), %rax
1422960b2:     	movl	%edi, 0x20(%rax)
1422960b5:     	testb	%r15b, %r15b
1422960b8:     	je	0x142295d9b <.text+0x2294d9b>
1422960be:     	movq	0x668(%rbp), %rax
1422960c5:     	movl	0x6d8(%rbp), %ecx
1422960cb:     	movl	%ecx, (%rax)
1422960cd:     	movl	0x68(%r10), %esi
1422960d1:     	movzbl	0x139eb31(%rip), %eax   # 0x143634c09
1422960d8:     	movl	$0x30, %ecx
1422960dd:     	movl	$0x8, %edx
1422960e2:     	callq	0x140613c10 <.text+0x612c10>
1422960e7:     	testq	%rax, %rax
1422960ea:     	je	0x142296c78 <.text+0x2295c78>
1422960f0:     	movq	$0x1, (%rax)
1422960f7:     	movq	$0x1, 0x8(%rax)
1422960ff:     	movb	$0x5, 0x18(%rax)
142296103:     	movq	%rbx, 0x20(%rax)
142296107:     	movl	%esi, 0x28(%rax)
14229610a:     	movl	%edi, 0x2c(%rax)
14229610d:     	movq	%rax, 0x538(%rbp)
142296114:     	movq	$0x9, 0x530(%rbp)
14229611f:     	movq	0x670(%rbp), %rcx
142296126:     	leaq	0x530(%rbp), %rdx
14229612d:     	leaq	0xda5524(%rip), %r8     # 0x14303b658
142296134:     	callq	0x140a84ec0 <.text+0xa83ec0>
142296139:     	jmp	0x142295d9b <.text+0x2294d9b>
14229613e:     	movl	0x34(%r10), %esi
142296142:     	movl	0x2d4(%rbp), %eax
142296148:     	movq	0x150(%rbp), %rcx
14229614f:     	movl	%eax, (%rcx)
142296151:     	movq	0x380(%rbp), %rax
142296158:     	movq	0x30(%rax), %rbx
14229615c:     	movq	0x40(%rax), %r12
142296160:     	cmpq	0x20(%rax), %rbx
142296164:     	jne	0x142296180 <.text+0x2295180>
142296166:     	movq	0x188(%rbp), %rcx
14229616d:     	leaq	0xda5154(%rip), %rdx    # 0x14303b2c8
142296174:     	callq	0x140562db0 <.text+0x561db0>
142296179:     	movq	0x708(%rbp), %r10
142296180:     	movl	0x700(%rbp), %eax
142296186:     	andb	$0x1, %al
142296188:     	movq	0x380(%rbp), %r8
14229618f:     	movq	0x28(%r8), %rcx
142296193:     	leaq	(%rbx,%rbx,2), %rdx
142296197:     	movq	%r12, (%rcx,%rdx,8)
14229619b:     	movl	%esi, 0x8(%rcx,%rdx,8)
14229619f:     	movl	%edi, 0xc(%rcx,%rdx,8)
1422961a3:     	movb	%al, 0x10(%rcx,%rdx,8)
1422961a7:     	incq	%rbx
1422961aa:     	movq	%rbx, 0x30(%r8)
1422961ae:     	incq	0x40(%r8)
1422961b2:     	testb	%r15b, %r15b
1422961b5:     	je	0x142295d8c <.text+0x2294d8c>
1422961bb:     	movq	0x668(%rbp), %rax
1422961c2:     	movl	0x6d8(%rbp), %ecx
1422961c8:     	movl	%ecx, (%rax)
1422961ca:     	movl	0x68(%r10), %ebx
1422961ce:     	movzbl	0x139ea34(%rip), %eax   # 0x143634c09
1422961d5:     	movl	$0x30, %ecx
1422961da:     	movl	$0x8, %edx
1422961df:     	callq	0x140613c10 <.text+0x612c10>
1422961e4:     	testq	%rax, %rax
1422961e7:     	je	0x142296c78 <.text+0x2295c78>
1422961ed:     	movq	$0x1, (%rax)
1422961f4:     	movq	$0x1, 0x8(%rax)
1422961fc:     	movb	$0xd, 0x18(%rax)
142296200:     	movl	%esi, 0x1c(%rax)
142296203:     	movl	%ebx, 0x20(%rax)
142296206:     	movl	%edi, 0x24(%rax)
142296209:     	movq	%rax, 0x538(%rbp)
142296210:     	movq	$0x9, 0x530(%rbp)
14229621b:     	movq	0x670(%rbp), %rcx
142296222:     	leaq	0x530(%rbp), %rdx
142296229:     	leaq	0xda54b8(%rip), %r8     # 0x14303b6e8
142296230:     	callq	0x140a84ec0 <.text+0xa83ec0>
142296235:     	movl	0x6f0(%rbp), %eax
14229623b:     	movq	0x690(%rbp), %rcx
142296242:     	movl	%eax, (%rcx)
142296244:     	movq	0x688(%rbp), %rax
14229624b:     	movl	%edi, 0xc(%rax)
14229624e:     	jmp	0x142295d9b <.text+0x2294d9b>
142296253:     	movq	0x38(%r10), %rbx
142296257:     	movq	%rbx, 0x2e8(%rbp)
14229625e:     	movl	0x2e4(%rbp), %eax
142296264:     	movq	0x170(%rbp), %rcx
14229626b:     	movl	%eax, (%rcx)
14229626d:     	movq	0x2a8(%rbp), %rax
142296274:     	movq	0x30(%rax), %rsi
142296278:     	movq	0x40(%rax), %r12
14229627c:     	cmpq	0x20(%rax), %rsi
142296280:     	jne	0x142296295 <.text+0x2295295>
142296282:     	movq	0x1a8(%rbp), %rcx
142296289:     	leaq	0xda5038(%rip), %rdx    # 0x14303b2c8
142296290:     	callq	0x140562db0 <.text+0x561db0>
142296295:     	movl	0x700(%rbp), %r8d
14229629c:     	movl	0x6d0(%rbp), %r9d
1422962a3:     	andb	%r8b, %r9b
1422962a6:     	movq	0x2a8(%rbp), %rdx
1422962ad:     	movq	0x28(%rdx), %rax
1422962b1:     	leaq	(%rsi,%rsi,2), %rcx
1422962b5:     	movq	%r12, (%rax,%rcx,8)
1422962b9:     	movq	%rbx, 0x8(%rax,%rcx,8)
1422962be:     	movl	%edi, 0x10(%rax,%rcx,8)
1422962c2:     	movb	%r9b, 0x14(%rax,%rcx,8)
1422962c7:     	incq	%rsi
1422962ca:     	movq	%rsi, 0x30(%rdx)
1422962ce:     	incq	0x40(%rdx)
1422962d2:     	movb	$0x1, %al
1422962d4:     	testb	$0x1, %r8b
1422962d8:     	je	0x1422919a8 <.text+0x22909a8>
1422962de:     	movq	0x688(%rbp), %rcx
1422962e5:     	movl	%edi, 0x1c(%rcx)
1422962e8:     	movl	0x6f0(%rbp), %eax
1422962ee:     	movq	0x690(%rbp), %rdx
1422962f5:     	movl	%eax, (%rdx)
1422962f7:     	movl	%edi, 0x18(%rcx)
1422962fa:     	movq	0x2e8(%rbp), %rdx
142296301:     	movq	0x208(%rbp), %rcx
142296308:     	leaq	0xda5301(%rip), %r8     # 0x14303b610
14229630f:     	callq	0x140a91ef0 <.text+0xa90ef0>
142296314:     	testq	%rax, %rax
142296317:     	je	0x142296780 <.text+0x2295780>
14229631d:     	cmpl	$0x2, 0x258(%rax)
142296324:     	jae	0x142296760 <.text+0x2295760>
14229632a:     	addq	$0x299, %rax            # imm = 0x299
142296330:     	movq	%rax, %rsi
142296333:     	movq	%rax, %rcx
142296336:     	callq	0x1408c5670 <.text+0x8c4670>
14229633b:     	testb	%al, %al
14229633d:     	je	0x14229637f <.text+0x229537f>
14229633f:     	leaq	0x2e8(%rbp), %rcx
142296346:     	callq	0x1400885d0 <.text+0x875d0>
14229634b:     	movq	0x1b0(%rbp), %rcx
142296352:     	movq	0x10(%rcx), %rdx
142296356:     	cmpq	%rdx, %rax
142296359:     	jae	0x142296d1e <.text+0x2295d1e>
14229635f:     	movq	0x8(%rcx), %rcx
142296363:     	movl	(%rcx,%rax,4), %eax
142296366:     	movl	0x6f0(%rbp), %ecx
14229636c:     	movq	0x690(%rbp), %rdx
142296373:     	movl	%ecx, (%rdx)
142296375:     	movq	0x688(%rbp), %rcx
14229637c:     	movl	%eax, 0x24(%rcx)
14229637f:     	movzbl	(%rsi), %eax
142296382:     	movl	0x6f0(%rbp), %ecx
142296388:     	movq	0x690(%rbp), %rdx
14229638f:     	movl	%ecx, (%rdx)
142296391:     	movq	0x688(%rbp), %rcx
142296398:     	movb	%al, 0x42(%rcx)
14229639b:     	jmp	0x142296780 <.text+0x2295780>
1422963a0:     	movq	0x38(%r10), %rbx
1422963a4:     	movq	0x160(%rbp), %rax
1422963ab:     	movl	0x2dc(%rbp), %ecx
1422963b1:     	movl	%ecx, (%rax)
1422963b3:     	movq	0x390(%rbp), %rax
1422963ba:     	movq	0x30(%rax), %rsi
1422963be:     	movq	0x40(%rax), %r12
1422963c2:     	cmpq	0x20(%rax), %rsi
1422963c6:     	jne	0x1422963e2 <.text+0x22953e2>
1422963c8:     	movq	0x190(%rbp), %rcx
1422963cf:     	leaq	0xda4ef2(%rip), %rdx    # 0x14303b2c8
1422963d6:     	callq	0x140562db0 <.text+0x561db0>
1422963db:     	movq	0x708(%rbp), %r10
1422963e2:     	movl	0x700(%rbp), %r9d
1422963e9:     	movl	%r9d, %eax
1422963ec:     	andb	$0x1, %al
1422963ee:     	movq	0x390(%rbp), %r8
1422963f5:     	movq	0x28(%r8), %rcx
1422963f9:     	leaq	(%rsi,%rsi,2), %rdx
1422963fd:     	movq	%r12, (%rcx,%rdx,8)
142296401:     	movq	%rbx, 0x8(%rcx,%rdx,8)
142296406:     	movl	%edi, 0x10(%rcx,%rdx,8)
14229640a:     	movb	%al, 0x14(%rcx,%rdx,8)
14229640e:     	incq	%rsi
142296411:     	movq	%rsi, 0x30(%r8)
142296415:     	incq	0x40(%r8)
142296419:     	movb	$0x1, %al
14229641b:     	testb	$0x1, %r9b
14229641f:     	je	0x1422919a8 <.text+0x22909a8>
142296425:     	movl	0x6f0(%rbp), %eax
14229642b:     	movq	0x690(%rbp), %rcx
142296432:     	movl	%eax, (%rcx)
142296434:     	movq	0x688(%rbp), %rax
14229643b:     	movl	%edi, 0x38(%rax)
14229643e:     	testb	%r15b, %r15b
142296441:     	je	0x142295d9b <.text+0x2294d9b>
142296447:     	movq	0x668(%rbp), %rax
14229644e:     	movl	0x6d8(%rbp), %ecx
142296454:     	movl	%ecx, (%rax)
142296456:     	movl	0x68(%r10), %esi
14229645a:     	movzbl	0x139e7a8(%rip), %eax   # 0x143634c09
142296461:     	movl	$0x30, %ecx
142296466:     	movl	$0x8, %edx
14229646b:     	callq	0x140613c10 <.text+0x612c10>
142296470:     	testq	%rax, %rax
142296473:     	je	0x142296c78 <.text+0x2295c78>
142296479:     	movq	$0x1, (%rax)
142296480:     	movq	$0x1, 0x8(%rax)
142296488:     	movb	$0x4, 0x18(%rax)
14229648c:     	movq	%rbx, 0x20(%rax)
142296490:     	movl	%esi, 0x28(%rax)
142296493:     	movl	%edi, 0x2c(%rax)
142296496:     	movq	%rax, 0x538(%rbp)
14229649d:     	movq	$0x9, 0x530(%rbp)
1422964a8:     	movq	0x670(%rbp), %rcx
1422964af:     	leaq	0x530(%rbp), %rdx
1422964b6:     	leaq	0xda51cb(%rip), %r8     # 0x14303b688
1422964bd:     	callq	0x140a84ec0 <.text+0xa83ec0>
1422964c2:     	jmp	0x142295d9b <.text+0x2294d9b>
1422964c7:     	movl	0x34(%r10), %esi
1422964cb:     	movq	0x168(%rbp), %rax
1422964d2:     	movl	0x2e0(%rbp), %ecx
1422964d8:     	movl	%ecx, (%rax)
1422964da:     	movq	0x398(%rbp), %rax
1422964e1:     	movq	0x30(%rax), %rbx
1422964e5:     	movq	0x40(%rax), %r12
1422964e9:     	cmpq	0x20(%rax), %rbx
1422964ed:     	jne	0x142296509 <.text+0x2295509>
1422964ef:     	movq	0x198(%rbp), %rcx
1422964f6:     	leaq	0xda4dcb(%rip), %rdx    # 0x14303b2c8
1422964fd:     	callq	0x140562db0 <.text+0x561db0>
142296502:     	movq	0x708(%rbp), %r10
142296509:     	movl	0x700(%rbp), %eax
14229650f:     	andb	$0x1, %al
142296511:     	movq	0x398(%rbp), %r8
142296518:     	movq	0x28(%r8), %rcx
14229651c:     	leaq	(%rbx,%rbx,2), %rdx
142296520:     	movq	%r12, (%rcx,%rdx,8)
142296524:     	movl	%esi, 0x8(%rcx,%rdx,8)
142296528:     	movl	%edi, 0xc(%rcx,%rdx,8)
14229652c:     	movb	%al, 0x10(%rcx,%rdx,8)
142296530:     	incq	%rbx
142296533:     	movq	%rbx, 0x30(%r8)
142296537:     	incq	0x40(%r8)
14229653b:     	testb	%r15b, %r15b
14229653e:     	je	0x142295d8c <.text+0x2294d8c>
142296544:     	movl	0x6f0(%rbp), %eax
14229654a:     	movq	0x690(%rbp), %rcx
142296551:     	movl	%eax, (%rcx)
142296553:     	movq	0x688(%rbp), %rax
14229655a:     	movl	%edi, 0x30(%rax)
14229655d:     	movq	0x668(%rbp), %rax
142296564:     	movl	0x6d8(%rbp), %ecx
14229656a:     	movl	%ecx, (%rax)
14229656c:     	movl	0x68(%r10), %ebx
142296570:     	movzbl	0x139e692(%rip), %eax   # 0x143634c09
142296577:     	movl	$0x30, %ecx
14229657c:     	movl	$0x8, %edx
142296581:     	callq	0x140613c10 <.text+0x612c10>
142296586:     	testq	%rax, %rax
142296589:     	je	0x142296c78 <.text+0x2295c78>
14229658f:     	movq	$0x1, (%rax)
142296596:     	movq	$0x1, 0x8(%rax)
14229659e:     	movb	$0x8, 0x18(%rax)
1422965a2:     	movl	%esi, 0x1c(%rax)
1422965a5:     	movl	%ebx, 0x20(%rax)
1422965a8:     	movl	%edi, 0x24(%rax)
1422965ab:     	movq	%rax, 0x538(%rbp)
1422965b2:     	movq	$0x9, 0x530(%rbp)
1422965bd:     	movq	0x670(%rbp), %rcx
1422965c4:     	leaq	0x530(%rbp), %rdx
1422965cb:     	leaq	0xda509e(%rip), %r8     # 0x14303b670
1422965d2:     	callq	0x140a84ec0 <.text+0xa83ec0>
1422965d7:     	jmp	0x142295d9b <.text+0x2294d9b>
1422965dc:     	movq	0x688(%rbp), %rax
1422965e3:     	movl	%edi, 0x2c(%rax)
1422965e6:     	testb	%r15b, %r15b
1422965e9:     	je	0x142295d9b <.text+0x2294d9b>
1422965ef:     	movq	0x668(%rbp), %rax
1422965f6:     	movl	0x6d8(%rbp), %ecx
1422965fc:     	movl	%ecx, (%rax)
1422965fe:     	movq	0x3e8(%rbp), %rsi
142296605:     	movl	0x68(%r10), %ebx
142296609:     	movzbl	0x139e5f9(%rip), %eax   # 0x143634c09
142296610:     	movl	$0x30, %ecx
142296615:     	movl	$0x8, %edx
14229661a:     	callq	0x140613c10 <.text+0x612c10>
14229661f:     	testq	%rax, %rax
142296622:     	je	0x142296c78 <.text+0x2295c78>
142296628:     	movq	$0x1, (%rax)
14229662f:     	movq	$0x1, 0x8(%rax)
142296637:     	movb	$0x9, 0x18(%rax)
14229663b:     	movq	%rsi, 0x20(%rax)
14229663f:     	movl	%ebx, 0x28(%rax)
142296642:     	movl	%edi, 0x2c(%rax)
142296645:     	movq	%rax, 0x538(%rbp)
14229664c:     	movq	$0x9, 0x530(%rbp)
142296657:     	movq	0x670(%rbp), %rcx
14229665e:     	leaq	0x530(%rbp), %rdx
142296665:     	leaq	0xda504c(%rip), %r8     # 0x14303b6b8
14229666c:     	callq	0x140a84ec0 <.text+0xa83ec0>
142296671:     	jmp	0x142295d9b <.text+0x2294d9b>
142296676:     	movl	0x6f0(%rbp), %eax
14229667c:     	movq	0x690(%rbp), %rcx
142296683:     	movl	%eax, (%rcx)
142296685:     	movq	0x688(%rbp), %rax
14229668c:     	movl	%edi, 0x28(%rax)
14229668f:     	testb	%r15b, %r15b
142296692:     	je	0x142295d9b <.text+0x2294d9b>
142296698:     	jmp	0x1422966e0 <.text+0x22956e0>
14229669a:     	movl	0x6f0(%rbp), %eax
1422966a0:     	movq	0x690(%rbp), %rcx
1422966a7:     	movl	%eax, (%rcx)
1422966a9:     	movq	0x688(%rbp), %rax
1422966b0:     	movl	%edi, 0x34(%rax)
1422966b3:     	testb	%r15b, %r15b
1422966b6:     	je	0x142295d9b <.text+0x2294d9b>
1422966bc:     	jmp	0x1422966e0 <.text+0x22956e0>
1422966be:     	movl	0x6f0(%rbp), %eax
1422966c4:     	movq	0x690(%rbp), %rcx
1422966cb:     	movl	%eax, (%rcx)
1422966cd:     	movq	0x688(%rbp), %rax
1422966d4:     	movl	%edi, 0x8(%rax)
1422966d7:     	testb	%r15b, %r15b
1422966da:     	je	0x142295d9b <.text+0x2294d9b>
1422966e0:     	movq	0x668(%rbp), %rax
1422966e7:     	movl	0x6d8(%rbp), %ecx
1422966ed:     	movl	%ecx, (%rax)
1422966ef:     	movl	0x68(%r10), %esi
1422966f3:     	movzbl	0x139e50f(%rip), %eax   # 0x143634c09
1422966fa:     	movl	$0x30, %ecx
1422966ff:     	movl	$0x8, %edx
142296704:     	callq	0x140613c10 <.text+0x612c10>
142296709:     	testq	%rax, %rax
14229670c:     	je	0x142296c78 <.text+0x2295c78>
142296712:     	movq	$0x1, (%rax)
142296719:     	movq	$0x1, 0x8(%rax)
142296721:     	movb	$0x7, 0x18(%rax)
142296725:     	movl	%r12d, 0x1c(%rax)
142296729:     	movl	%esi, 0x20(%rax)
14229672c:     	movl	%edi, 0x24(%rax)
14229672f:     	movq	%rax, 0x538(%rbp)
142296736:     	movq	$0x9, 0x530(%rbp)
142296741:     	movq	0x670(%rbp), %rcx
142296748:     	leaq	0x530(%rbp), %rdx
14229674f:     	leaq	0xda4f7a(%rip), %r8     # 0x14303b6d0
142296756:     	callq	0x140a84ec0 <.text+0xa83ec0>
14229675b:     	jmp	0x142295d9b <.text+0x2294d9b>
142296760:     	movzbl	0x295(%rax), %eax
142296767:     	movl	0x6f0(%rbp), %ecx
14229676d:     	movq	0x690(%rbp), %rdx
142296774:     	movl	%ecx, (%rdx)
142296776:     	movq	0x688(%rbp), %rcx
14229677d:     	movb	%al, 0x40(%rcx)
142296780:     	testb	%r15b, %r15b
142296783:     	movq	0x708(%rbp), %rdx
14229678a:     	je	0x142295d9b <.text+0x2294d9b>
142296790:     	movq	0x668(%rbp), %rax
142296797:     	movl	0x6d8(%rbp), %ecx
14229679d:     	movl	%ecx, (%rax)
14229679f:     	movl	0x68(%rdx), %esi
1422967a2:     	movq	0x2e8(%rbp), %rbx
1422967a9:     	movzbl	0x139e459(%rip), %eax   # 0x143634c09
1422967b0:     	movl	$0x30, %ecx
1422967b5:     	movl	$0x8, %edx
1422967ba:     	callq	0x140613c10 <.text+0x612c10>
1422967bf:     	testq	%rax, %rax
1422967c2:     	je	0x142296c78 <.text+0x2295c78>
1422967c8:     	movq	$0x1, (%rax)
1422967cf:     	movq	$0x1, 0x8(%rax)
1422967d7:     	movb	$0x3, 0x18(%rax)
1422967db:     	movq	%rbx, 0x20(%rax)
1422967df:     	movl	%esi, 0x28(%rax)
1422967e2:     	movl	%edi, 0x2c(%rax)
1422967e5:     	movq	%rax, 0x538(%rbp)
1422967ec:     	movq	$0x9, 0x530(%rbp)
1422967f7:     	movq	0x670(%rbp), %rcx
1422967fe:     	leaq	0x530(%rbp), %rdx
142296805:     	leaq	0xda4e34(%rip), %r8     # 0x14303b640
14229680c:     	callq	0x140a84ec0 <.text+0xa83ec0>
142296811:     	jmp	0x142295d9b <.text+0x2294d9b>
142296816:     	movl	0x6f0(%rbp), %eax
14229681c:     	movq	0x690(%rbp), %rcx
142296823:     	movl	%eax, (%rcx)
142296825:     	movq	0x688(%rbp), %rax
14229682c:     	movl	$0x1, (%rax)
142296832:     	movl	%edi, 0x4(%rax)
142296835:     	testb	%r15b, %r15b
142296838:     	je	0x142295d9b <.text+0x2294d9b>
14229683e:     	jmp	0x1422966e0 <.text+0x22956e0>
142296843:     	movq	0x830(%rbp), %r8
14229684a:     	testb	$0x1, %al
14229684c:     	je	0x14229688d <.text+0x229588d>
14229684e:     	movzbl	(%r8), %eax
142296852:     	movl	%eax, %edx
142296854:     	movq	0x350(%rbp), %r9
14229685b:     	andb	%r9b, %dl
14229685e:     	movl	%r9d, %ecx
142296861:     	testb	$0x1, %dl
142296864:     	movq	0x680(%rbp), %r14
14229686b:     	je	0x1422968a7 <.text+0x22958a7>
14229686d:     	movq	0x210(%rbp), %rax
142296874:     	cmpq	%rax, 0x8(%r8)
142296878:     	jne	0x1422968b2 <.text+0x22958b2>
14229687a:     	movq	0x230(%rbp), %rax
142296881:     	cmpq	%rax, 0x10(%r8)
142296885:     	je	0x142296939 <.text+0x2295939>
14229688b:     	jmp	0x1422968b2 <.text+0x22958b2>
14229688d:     	movq	$0x0, (%r8)
142296894:     	xorl	%eax, %eax
142296896:     	movq	0x350(%rbp), %r9
14229689d:     	movl	%r9d, %ecx
1422968a0:     	movq	0x680(%rbp), %r14
1422968a7:     	orb	%al, %cl
1422968a9:     	testb	$0x1, %cl
1422968ac:     	je	0x142296939 <.text+0x2295939>
1422968b2:     	movq	%r9, (%r8)
1422968b5:     	movq	0x210(%rbp), %rax
1422968bc:     	movq	%rax, 0x8(%r8)
1422968c0:     	movq	0x230(%rbp), %rax
1422968c7:     	movq	%rax, 0x10(%r8)
1422968cb:     	cmpl	$0x0, 0x5f8(%rbp)
1422968d2:     	je	0x142296939 <.text+0x2295939>
1422968d4:     	movq	0x858(%rbp), %rdx
1422968db:     	movl	0x1c(%rdx), %eax
1422968de:     	movq	0x10(%rdx), %rcx
1422968e2:     	movl	%eax, (%rcx)
1422968e4:     	movq	(%rdx), %rsi
1422968e7:     	movq	0x30(%rsi), %rdi
1422968eb:     	movq	0x40(%rsi), %rbx
1422968ef:     	cmpq	0x20(%rsi), %rdi
1422968f3:     	jne	0x142296905 <.text+0x2295905>
1422968f5:     	leaq	0x20(%rsi), %rcx
1422968f9:     	leaq	0xda49c8(%rip), %rdx    # 0x14303b2c8
142296900:     	callq	0x140562c30 <.text+0x561c30>
142296905:     	movq	0x28(%rsi), %rax
142296909:     	movq	%rdi, %rcx
14229690c:     	shlq	$0x5, %rcx
142296910:     	movl	0x5f8(%rbp), %edx
142296916:     	movl	%edx, (%rax,%rcx)
142296919:     	movaps	0x60(%rbp), %xmm0
14229691d:     	movups	%xmm0, 0x4(%rax,%rcx)
142296922:     	movl	0x70(%rbp), %edx
142296925:     	movl	%edx, 0x14(%rax,%rcx)
142296929:     	movq	%rbx, 0x18(%rax,%rcx)
14229692e:     	incq	%rdi
142296931:     	movq	%rdi, 0x30(%rsi)
142296935:     	incq	0x40(%rsi)
142296939:     	testb	$0x1, %r14b
14229693d:     	je	0x142296b86 <.text+0x2295b86>
142296943:     	movq	0x528(%rbp), %r9
14229694a:     	movq	0xf8(%r9), %rax
142296951:     	movq	0x100(%r9), %rcx
142296958:     	leaq	(%rax,%rcx,4), %rcx
14229695c:     	movq	0x200(%rbp), %rdx
142296963:     	movl	0x1c(%rdx), %r8d
142296967:     	movl	%r8d, 0x608(%rbp)
14229696e:     	movq	0x10(%rdx), %r8
142296972:     	cmpb	$0x0, 0x108(%r9)
14229697a:     	je	0x142296a7e <.text+0x2295a7e>
142296980:     	movl	$0x8, %r11d
142296986:     	xorl	%r9d, %r9d
142296989:     	xorl	%esi, %esi
14229698b:     	xorl	%r10d, %r10d
14229698e:     	xorl	%ebx, %ebx
142296990:     	cmpq	%r10, %rbx
142296993:     	jne	0x142296a10 <.text+0x2295a10>
142296995:     	movq	0x500(%rbp), %rdi
14229699c:     	nopl	(%rax)
1422969a0:     	cmpq	%rcx, %rax
1422969a3:     	je	0x142296b86 <.text+0x2295b86>
1422969a9:     	movl	(%rax), %r10d
1422969ac:     	addq	$0x4, %rax
1422969b0:     	movq	0x1a8(%rdi), %r11
1422969b7:     	leaq	(%r10,%r10,8), %rsi
1422969bb:     	movq	0x10(%r11,%rsi,8), %r10
1422969c0:     	testq	%r10, %r10
1422969c3:     	je	0x1422969a0 <.text+0x22959a0>
1422969c5:     	leaq	(%r11,%rsi,8), %rsi
1422969c9:     	movq	0x528(%rbp), %r11
1422969d0:     	movq	0x110(%r11), %r11
1422969d7:     	movq	0x38(%rsi), %rdi
1422969db:     	movq	(%rdi,%r11,8), %rdi
1422969df:     	movq	0x8(%rsi), %r11
1422969e3:     	movq	0x18(%rsi), %rbx
1422969e7:     	notq	%rdi
1422969ea:     	leaq	(%rdi,%rdi,2), %rdi
1422969ee:     	shlq	$0x4, %rdi
1422969f2:     	movq	0x10(%rbx,%rdi), %rsi
1422969f7:     	movq	0x28(%rbx,%rdi), %rdi
1422969fc:     	xorl	%r12d, %r12d
1422969ff:     	jmp	0x142296a13 <.text+0x2295a13>
142296a01:     	nopw	%cs:(%rax,%rax)
142296a10:     	movq	%rbx, %r12
142296a13:     	movq	(%r11,%r12,8), %r14
142296a17:     	movq	%r14, %rbx
142296a1a:     	shrq	$0x20, %rbx
142296a1e:     	je	0x142296b86 <.text+0x2295b86>
142296a24:     	leaq	0x1(%r12), %rbx
142296a29:     	leaq	0x1(%r9), %r15
142296a2d:     	movl	%r12d, %r12d
142296a30:     	leaq	(%r12,%r12,4), %r13
142296a34:     	shlq	$0x5, %r13
142296a38:     	movl	0x5ec(%rbp), %edx
142296a3e:     	movl	%edx, (%rdi,%r12,4)
142296a42:     	movq	$0x0, (%rsi,%r13)
142296a4a:     	cmpq	0x658(%rbp), %r9
142296a51:     	movq	%r15, %r9
142296a54:     	jne	0x142296990 <.text+0x2295990>
142296a5a:     	movl	0x608(%rbp), %edx
142296a60:     	movl	%edx, (%r8)
142296a63:     	movq	0x338(%rbp), %rdx
142296a6a:     	movq	$0x1, 0x10(%rdx)
142296a72:     	movq	%r14, 0x18(%rdx)
142296a76:     	movq	%r15, %r9
142296a79:     	jmp	0x142296990 <.text+0x2295990>
142296a7e:     	movl	$0x8, %r11d
142296a84:     	xorl	%r9d, %r9d
142296a87:     	xorl	%esi, %esi
142296a89:     	xorl	%r10d, %r10d
142296a8c:     	xorl	%ebx, %ebx
142296a8e:     	cmpq	%r10, %rbx
142296a91:     	jne	0x142296b15 <.text+0x2295b15>
142296a97:     	movq	0x500(%rbp), %rdi
142296a9e:     	nop
142296aa0:     	cmpq	%rcx, %rax
142296aa3:     	je	0x142296b86 <.text+0x2295b86>
142296aa9:     	movl	(%rax), %r10d
142296aac:     	addq	$0x4, %rax
142296ab0:     	movq	0x100(%rdi), %r11
142296ab7:     	leaq	(%r10,%r10,4), %rsi
142296abb:     	shlq	$0x5, %rsi
142296abf:     	movq	0x58(%r11,%rsi), %r10
142296ac4:     	testq	%r10, %r10
142296ac7:     	je	0x142296aa0 <.text+0x2295aa0>
142296ac9:     	addq	%rsi, %r11
142296acc:     	movl	0x94(%r11), %esi
142296ad3:     	movq	0x1a8(%rdi), %rdi
142296ada:     	leaq	(%rsi,%rsi,8), %rsi
142296ade:     	movq	0x528(%rbp), %rbx
142296ae5:     	movq	0x110(%rbx), %rbx
142296aec:     	movq	0x18(%rdi,%rsi,8), %r14
142296af1:     	movq	0x38(%rdi,%rsi,8), %rsi
142296af6:     	movq	(%rsi,%rbx,8), %rsi
142296afa:     	notq	%rsi
142296afd:     	leaq	(%rsi,%rsi,2), %rdi
142296b01:     	shlq	$0x4, %rdi
142296b05:     	movq	0x10(%r14,%rdi), %rsi
142296b0a:     	movq	0x28(%r14,%rdi), %rdi
142296b0f:     	movq	0x50(%r11), %r11
142296b13:     	xorl	%ebx, %ebx
142296b15:     	movq	%rbx, %r12
142296b18:     	shlq	$0x4, %r12
142296b1c:     	movq	(%r11,%r12), %r14
142296b20:     	movq	%r14, %r15
142296b23:     	shrq	$0x20, %r15
142296b27:     	je	0x142296b86 <.text+0x2295b86>
142296b29:     	addq	%r11, %r12
142296b2c:     	incq	%rbx
142296b2f:     	leaq	0x1(%r9), %r15
142296b33:     	movl	0x8(%r12), %r12d
142296b38:     	leaq	(%r12,%r12,4), %r13
142296b3c:     	shlq	$0x5, %r13
142296b40:     	movl	0x5ec(%rbp), %edx
142296b46:     	movl	%edx, (%rdi,%r12,4)
142296b4a:     	movq	$0x0, (%rsi,%r13)
142296b52:     	cmpq	0x658(%rbp), %r9
142296b59:     	movq	%r15, %r9
142296b5c:     	jne	0x142296a8e <.text+0x2295a8e>
142296b62:     	movl	0x608(%rbp), %edx
142296b68:     	movl	%edx, (%r8)
142296b6b:     	movq	0x338(%rbp), %rdx
142296b72:     	movq	$0x1, 0x10(%rdx)
142296b7a:     	movq	%r14, 0x18(%rdx)
142296b7e:     	movq	%r15, %r9
142296b81:     	jmp	0x142296a8e <.text+0x2295a8e>
142296b86:     	movaps	0x720(%rbp), %xmm6
142296b8d:     	movaps	0x730(%rbp), %xmm7
142296b94:     	movaps	0x740(%rbp), %xmm8
142296b9c:     	movaps	0x750(%rbp), %xmm9
142296ba4:     	movaps	0x760(%rbp), %xmm10
142296bac:     	movaps	0x770(%rbp), %xmm11
142296bb4:     	movaps	0x780(%rbp), %xmm12
142296bbc:     	movaps	0x790(%rbp), %xmm13
142296bc4:     	movaps	0x7a0(%rbp), %xmm14
142296bcc:     	movaps	0x7b0(%rbp), %xmm15
142296bd4:     	addq	$0x848, %rsp            # imm = 0x848
142296bdb:     	popq	%rbx
142296bdc:     	popq	%rdi
142296bdd:     	popq	%rsi
142296bde:     	popq	%r12
142296be0:     	popq	%r13
142296be2:     	popq	%r14
142296be4:     	popq	%r15
142296be6:     	popq	%rbp
142296be7:     	retq
142296be8:     	shrq	$0x20, %rax
142296bec:     	movl	$0x1, %edx
142296bf1:     	movq	0x3b0(%rbp), %r8
142296bf8:     	movl	%edx, 0x530(%rbp)
142296bfe:     	movl	%r8d, 0x534(%rbp)
142296c05:     	shlq	$0x20, %rax
142296c09:     	orq	%rcx, %rax
142296c0c:     	movq	%rax, 0x538(%rbp)
142296c13:     	leaq	0xda4ae6(%rip), %rax    # 0x14303b700
142296c1a:     	movq	%rax, 0x20(%rsp)
142296c1f:     	leaq	0xda46ea(%rip), %rcx    # 0x14303b310
142296c26:     	leaq	0xda46c3(%rip), %r9     # 0x14303b2f0
142296c2d:     	leaq	0x530(%rbp), %r8
142296c34:     	movl	$0x2b, %edx
142296c39:     	callq	0x1428d9760 <.text+0x28d8760>
142296c3e:     	leaq	0xda485b(%rip), %r8     # 0x14303b4a0
142296c45:     	movq	%rdx, %rcx
142296c48:     	movq	%rax, %rdx
142296c4b:     	callq	0x1428d9518 <.text+0x28d8518>
142296c50:     	leaq	0xda4861(%rip), %r8     # 0x14303b4b8
142296c57:     	movq	%rdx, %rcx
142296c5a:     	movq	%rax, %rdx
142296c5d:     	callq	0x1428d9518 <.text+0x28d8518>
142296c62:     	leaq	0xda471f(%rip), %r8     # 0x14303b388
142296c69:     	movl	$0x1, %ecx
142296c6e:     	movl	$0x1f, %edx
142296c73:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
142296c78:     	movl	$0x8, %ecx
142296c7d:     	movl	$0x30, %edx
142296c82:     	callq	0x1428d9000 <.text+0x28d8000>
142296c87:     	leaq	0xda46fa(%rip), %r8     # 0x14303b388
142296c8e:     	movl	$0x1, %ecx
142296c93:     	movl	$0x1e, %edx
142296c98:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
142296c9d:     	leaq	0xda4954(%rip), %rcx    # 0x14303b5f8
142296ca4:     	callq	0x1428d9310 <.text+0x28d8310>
142296ca9:     	leaq	0xda4918(%rip), %rcx    # 0x14303b5c8
142296cb0:     	callq	0x1428d9310 <.text+0x28d8310>
142296cb5:     	leaq	0xda48dc(%rip), %rcx    # 0x14303b598
142296cbc:     	callq	0x1428d9310 <.text+0x28d8310>
142296cc1:     	leaq	0xda47c0(%rip), %rcx    # 0x14303b488
142296cc8:     	callq	0x1428d9310 <.text+0x28d8310>
142296ccd:     	leaq	0xda46b4(%rip), %r8     # 0x14303b388
142296cd4:     	movl	$0x1, %ecx
142296cd9:     	movl	$0x21, %edx
142296cde:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
142296ce3:     	leaq	0xda469e(%rip), %r8     # 0x14303b388
142296cea:     	movl	$0x1, %ecx
142296cef:     	movl	$0x1d, %edx
142296cf4:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
142296cf9:     	leaq	0xda4688(%rip), %r8     # 0x14303b388
142296d00:     	movl	$0x1, %ecx
142296d05:     	movl	$0x1a, %edx
142296d0a:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
142296d0f:     	leaq	0xda498a(%rip), %r8     # 0x14303b6a0
142296d16:     	movq	%rax, %rcx
142296d19:     	callq	0x1428d9518 <.text+0x28d8518>
142296d1e:     	leaq	0xda4903(%rip), %r8     # 0x14303b628
142296d25:     	movq	%rax, %rcx
142296d28:     	callq	0x1428d9518 <.text+0x28d8518>
142296d2d:     	int3
142296d2e:     	nop
142296d30:     	movq	%rdx, 0x10(%rsp)
142296d35:     	pushq	%rbp
142296d36:     	pushq	%r15
142296d38:     	pushq	%r14
142296d3a:     	pushq	%r13
142296d3c:     	pushq	%r12
142296d3e:     	pushq	%rsi
142296d3f:     	pushq	%rdi
142296d40:     	pushq	%rbx
142296d41:     	subq	$0xc8, %rsp
142296d48:     	leaq	0x80(%rdx), %rbp
142296d4f:     	movaps	%xmm15, 0x20(%rsp)
142296d55:     	movaps	%xmm14, 0x30(%rsp)
142296d5b:     	movaps	%xmm13, 0x40(%rsp)
142296d61:     	movaps	%xmm12, 0x50(%rsp)
142296d67:     	movaps	%xmm11, 0x60(%rsp)
142296d6d:     	movaps	%xmm10, 0x70(%rsp)
142296d73:     	movaps	%xmm9, 0x80(%rsp)
142296d7c:     	movaps	%xmm8, 0x90(%rsp)
142296d85:     	movaps	%xmm7, 0xa0(%rsp)
142296d8d:     	movaps	%xmm6, 0xb0(%rsp)
142296d95:     	cmpb	$0x0, 0x69b(%rbp)
142296d9c:     	je	0x142296dbc <.text+0x2295dbc>
142296d9e:     	movq	0x638(%rbp), %rdx
142296da5:     	testq	%rdx, %rdx
142296da8:     	je	0x142296dbc <.text+0x2295dbc>
142296daa:     	movq	0x640(%rbp), %rcx
142296db1:     	movl	$0x1, %r8d
142296db7:     	callq	0x140613c20 <.text+0x612c20>
142296dbc:     	movaps	0xb0(%rsp), %xmm6
142296dc4:     	movaps	0xa0(%rsp), %xmm7
142296dcc:     	movaps	0x90(%rsp), %xmm8
142296dd5:     	movaps	0x80(%rsp), %xmm9
142296dde:     	movaps	0x70(%rsp), %xmm10
142296de4:     	movaps	0x60(%rsp), %xmm11
142296dea:     	movaps	0x50(%rsp), %xmm12
142296df0:     	movaps	0x40(%rsp), %xmm13
142296df6:     	movaps	0x30(%rsp), %xmm14
142296dfc:     	movaps	0x20(%rsp), %xmm15
142296e02:     	addq	$0xc8, %rsp
142296e09:     	popq	%rbx
142296e0a:     	popq	%rdi
142296e0b:     	popq	%rsi
142296e0c:     	popq	%r12
142296e0e:     	popq	%r13
142296e10:     	popq	%r14
142296e12:     	popq	%r15
142296e14:     	popq	%rbp
142296e15:     	retq
142296e16:     	nopw	%cs:(%rax,%rax)
142296e20:     	movq	%rdx, 0x10(%rsp)
142296e25:     	pushq	%rbp
142296e26:     	pushq	%r15
142296e28:     	pushq	%r14
142296e2a:     	pushq	%r13
142296e2c:     	pushq	%r12
142296e2e:     	pushq	%rsi
142296e2f:     	pushq	%rdi
142296e30:     	pushq	%rbx
142296e31:     	subq	$0xc8, %rsp
142296e38:     	leaq	0x80(%rdx), %rbp
142296e3f:     	movaps	%xmm15, 0x20(%rsp)
142296e45:     	movaps	%xmm14, 0x30(%rsp)
142296e4b:     	movaps	%xmm13, 0x40(%rsp)
142296e51:     	movaps	%xmm12, 0x50(%rsp)
142296e57:     	movaps	%xmm11, 0x60(%rsp)
142296e5d:     	movaps	%xmm10, 0x70(%rsp)
142296e63:     	movaps	%xmm9, 0x80(%rsp)
142296e6c:     	movaps	%xmm8, 0x90(%rsp)
142296e75:     	movaps	%xmm7, 0xa0(%rsp)
142296e7d:     	movaps	%xmm6, 0xb0(%rsp)
142296e85:     	cmpb	$0x0, 0x69c(%rbp)
142296e8c:     	je	0x142296eac <.text+0x2295eac>
142296e8e:     	movq	0x638(%rbp), %rdx
142296e95:     	testq	%rdx, %rdx
142296e98:     	je	0x142296eac <.text+0x2295eac>
142296e9a:     	movq	0x640(%rbp), %rcx
142296ea1:     	movl	$0x1, %r8d
142296ea7:     	callq	0x140613c20 <.text+0x612c20>
142296eac:     	movaps	0xb0(%rsp), %xmm6
142296eb4:     	movaps	0xa0(%rsp), %xmm7
142296ebc:     	movaps	0x90(%rsp), %xmm8
142296ec5:     	movaps	0x80(%rsp), %xmm9
142296ece:     	movaps	0x70(%rsp), %xmm10
142296ed4:     	movaps	0x60(%rsp), %xmm11
142296eda:     	movaps	0x50(%rsp), %xmm12
142296ee0:     	movaps	0x40(%rsp), %xmm13
142296ee6:     	movaps	0x30(%rsp), %xmm14
142296eec:     	movaps	0x20(%rsp), %xmm15
142296ef2:     	addq	$0xc8, %rsp
142296ef9:     	popq	%rbx
142296efa:     	popq	%rdi
142296efb:     	popq	%rsi
142296efc:     	popq	%r12
142296efe:     	popq	%r13
142296f00:     	popq	%r14
142296f02:     	popq	%r15
142296f04:     	popq	%rbp
142296f05:     	retq
142296f06:     	nopw	%cs:(%rax,%rax)
142296f10:     	movq	%rdx, 0x10(%rsp)
142296f15:     	pushq	%rbp
142296f16:     	pushq	%r15
142296f18:     	pushq	%r14
142296f1a:     	pushq	%r13
142296f1c:     	pushq	%r12
142296f1e:     	pushq	%rsi
142296f1f:     	pushq	%rdi
142296f20:     	pushq	%rbx
142296f21:     	subq	$0xc8, %rsp
142296f28:     	leaq	0x80(%rdx), %rbp
142296f2f:     	movaps	%xmm15, 0x20(%rsp)
142296f35:     	movaps	%xmm14, 0x30(%rsp)
142296f3b:     	movaps	%xmm13, 0x40(%rsp)
142296f41:     	movaps	%xmm12, 0x50(%rsp)
142296f47:     	movaps	%xmm11, 0x60(%rsp)
142296f4d:     	movaps	%xmm10, 0x70(%rsp)
142296f53:     	movaps	%xmm9, 0x80(%rsp)
142296f5c:     	movaps	%xmm8, 0x90(%rsp)
142296f65:     	movaps	%xmm7, 0xa0(%rsp)
142296f6d:     	movaps	%xmm6, 0xb0(%rsp)
142296f75:     	cmpb	$0x0, 0x69d(%rbp)
142296f7c:     	je	0x142296f9c <.text+0x2295f9c>
142296f7e:     	movq	0x638(%rbp), %rdx
142296f85:     	testq	%rdx, %rdx
142296f88:     	je	0x142296f9c <.text+0x2295f9c>
142296f8a:     	movq	0x640(%rbp), %rcx
142296f91:     	movl	$0x1, %r8d
142296f97:     	callq	0x140613c20 <.text+0x612c20>
142296f9c:     	movaps	0xb0(%rsp), %xmm6
142296fa4:     	movaps	0xa0(%rsp), %xmm7
142296fac:     	movaps	0x90(%rsp), %xmm8
142296fb5:     	movaps	0x80(%rsp), %xmm9
142296fbe:     	movaps	0x70(%rsp), %xmm10
142296fc4:     	movaps	0x60(%rsp), %xmm11
142296fca:     	movaps	0x50(%rsp), %xmm12
142296fd0:     	movaps	0x40(%rsp), %xmm13
142296fd6:     	movaps	0x30(%rsp), %xmm14
142296fdc:     	movaps	0x20(%rsp), %xmm15
142296fe2:     	addq	$0xc8, %rsp
142296fe9:     	popq	%rbx
142296fea:     	popq	%rdi
142296feb:     	popq	%rsi
142296fec:     	popq	%r12
142296fee:     	popq	%r13
142296ff0:     	popq	%r14
142296ff2:     	popq	%r15
142296ff4:     	popq	%rbp
142296ff5:     	retq
142296ff6:     	nopw	%cs:(%rax,%rax)
142297000:     	movq	%rdx, 0x10(%rsp)
142297005:     	pushq	%rbp
142297006:     	pushq	%r15
142297008:     	pushq	%r14
14229700a:     	pushq	%r13
14229700c:     	pushq	%r12
14229700e:     	pushq	%rsi
14229700f:     	pushq	%rdi
142297010:     	pushq	%rbx
142297011:     	subq	$0xc8, %rsp
142297018:     	leaq	0x80(%rdx), %rbp
14229701f:     	movaps	%xmm15, 0x20(%rsp)
142297025:     	movaps	%xmm14, 0x30(%rsp)
14229702b:     	movaps	%xmm13, 0x40(%rsp)
142297031:     	movaps	%xmm12, 0x50(%rsp)
142297037:     	movaps	%xmm11, 0x60(%rsp)
14229703d:     	movaps	%xmm10, 0x70(%rsp)
142297043:     	movaps	%xmm9, 0x80(%rsp)
14229704c:     	movaps	%xmm8, 0x90(%rsp)
142297055:     	movaps	%xmm7, 0xa0(%rsp)
14229705d:     	movaps	%xmm6, 0xb0(%rsp)
142297065:     	cmpb	$0x0, 0x69e(%rbp)
14229706c:     	je	0x14229708c <.text+0x229608c>
14229706e:     	movq	0x638(%rbp), %rdx
142297075:     	testq	%rdx, %rdx
142297078:     	je	0x14229708c <.text+0x229608c>
14229707a:     	movq	0x640(%rbp), %rcx
142297081:     	movl	$0x1, %r8d
142297087:     	callq	0x140613c20 <.text+0x612c20>
14229708c:     	movaps	0xb0(%rsp), %xmm6
142297094:     	movaps	0xa0(%rsp), %xmm7
14229709c:     	movaps	0x90(%rsp), %xmm8
1422970a5:     	movaps	0x80(%rsp), %xmm9
1422970ae:     	movaps	0x70(%rsp), %xmm10
1422970b4:     	movaps	0x60(%rsp), %xmm11
1422970ba:     	movaps	0x50(%rsp), %xmm12
1422970c0:     	movaps	0x40(%rsp), %xmm13
1422970c6:     	movaps	0x30(%rsp), %xmm14
1422970cc:     	movaps	0x20(%rsp), %xmm15
1422970d2:     	addq	$0xc8, %rsp
1422970d9:     	popq	%rbx
1422970da:     	popq	%rdi
1422970db:     	popq	%rsi
1422970dc:     	popq	%r12
1422970de:     	popq	%r13
1422970e0:     	popq	%r14
1422970e2:     	popq	%r15
1422970e4:     	popq	%rbp
1422970e5:     	retq
1422970e6:     	nopw	%cs:(%rax,%rax)
1422970f0:     	movq	%rdx, 0x10(%rsp)
1422970f5:     	pushq	%rbp
1422970f6:     	pushq	%r15
1422970f8:     	pushq	%r14
1422970fa:     	pushq	%r13
1422970fc:     	pushq	%r12
1422970fe:     	pushq	%rsi
1422970ff:     	pushq	%rdi
142297100:     	pushq	%rbx
142297101:     	subq	$0xc8, %rsp
142297108:     	leaq	0x80(%rdx), %rbp
14229710f:     	movaps	%xmm15, 0x20(%rsp)
142297115:     	movaps	%xmm14, 0x30(%rsp)
14229711b:     	movaps	%xmm13, 0x40(%rsp)
142297121:     	movaps	%xmm12, 0x50(%rsp)
142297127:     	movaps	%xmm11, 0x60(%rsp)
14229712d:     	movaps	%xmm10, 0x70(%rsp)
142297133:     	movaps	%xmm9, 0x80(%rsp)
14229713c:     	movaps	%xmm8, 0x90(%rsp)
142297145:     	movaps	%xmm7, 0xa0(%rsp)
14229714d:     	movaps	%xmm6, 0xb0(%rsp)
142297155:     	cmpb	$0x0, 0x69f(%rbp)
14229715c:     	je	0x14229717c <.text+0x229617c>
14229715e:     	movq	0x638(%rbp), %rdx
142297165:     	testq	%rdx, %rdx
142297168:     	je	0x14229717c <.text+0x229617c>
14229716a:     	movq	0x640(%rbp), %rcx
142297171:     	movl	$0x1, %r8d
142297177:     	callq	0x140613c20 <.text+0x612c20>
14229717c:     	movaps	0xb0(%rsp), %xmm6
142297184:     	movaps	0xa0(%rsp), %xmm7
14229718c:     	movaps	0x90(%rsp), %xmm8
142297195:     	movaps	0x80(%rsp), %xmm9
14229719e:     	movaps	0x70(%rsp), %xmm10
1422971a4:     	movaps	0x60(%rsp), %xmm11
1422971aa:     	movaps	0x50(%rsp), %xmm12
1422971b0:     	movaps	0x40(%rsp), %xmm13
1422971b6:     	movaps	0x30(%rsp), %xmm14
1422971bc:     	movaps	0x20(%rsp), %xmm15
1422971c2:     	addq	$0xc8, %rsp
1422971c9:     	popq	%rbx
1422971ca:     	popq	%rdi
1422971cb:     	popq	%rsi
1422971cc:     	popq	%r12
1422971ce:     	popq	%r13
1422971d0:     	popq	%r14
1422971d2:     	popq	%r15
1422971d4:     	popq	%rbp
1422971d5:     	retq
1422971d6:     	nopw	%cs:(%rax,%rax)
1422971e0:     	movq	%rdx, 0x10(%rsp)
1422971e5:     	pushq	%rbp
1422971e6:     	pushq	%r15
1422971e8:     	pushq	%r14
1422971ea:     	pushq	%r13
1422971ec:     	pushq	%r12
1422971ee:     	pushq	%rsi
1422971ef:     	pushq	%rdi
1422971f0:     	pushq	%rbx
1422971f1:     	subq	$0xc8, %rsp
1422971f8:     	leaq	0x80(%rdx), %rbp
1422971ff:     	movaps	%xmm15, 0x20(%rsp)
142297205:     	movaps	%xmm14, 0x30(%rsp)
14229720b:     	movaps	%xmm13, 0x40(%rsp)
142297211:     	movaps	%xmm12, 0x50(%rsp)
142297217:     	movaps	%xmm11, 0x60(%rsp)
14229721d:     	movaps	%xmm10, 0x70(%rsp)
142297223:     	movaps	%xmm9, 0x80(%rsp)
14229722c:     	movaps	%xmm8, 0x90(%rsp)
142297235:     	movaps	%xmm7, 0xa0(%rsp)
14229723d:     	movaps	%xmm6, 0xb0(%rsp)
142297245:     	cmpb	$0x0, 0x6a0(%rbp)
14229724c:     	je	0x14229726c <.text+0x229626c>
14229724e:     	movq	0x638(%rbp), %rdx
142297255:     	testq	%rdx, %rdx
142297258:     	je	0x14229726c <.text+0x229626c>
14229725a:     	movq	0x640(%rbp), %rcx
142297261:     	movl	$0x1, %r8d
142297267:     	callq	0x140613c20 <.text+0x612c20>
14229726c:     	movaps	0xb0(%rsp), %xmm6
142297274:     	movaps	0xa0(%rsp), %xmm7
14229727c:     	movaps	0x90(%rsp), %xmm8
142297285:     	movaps	0x80(%rsp), %xmm9
14229728e:     	movaps	0x70(%rsp), %xmm10
142297294:     	movaps	0x60(%rsp), %xmm11
14229729a:     	movaps	0x50(%rsp), %xmm12
1422972a0:     	movaps	0x40(%rsp), %xmm13
1422972a6:     	movaps	0x30(%rsp), %xmm14
1422972ac:     	movaps	0x20(%rsp), %xmm15
1422972b2:     	addq	$0xc8, %rsp
1422972b9:     	popq	%rbx
1422972ba:     	popq	%rdi
1422972bb:     	popq	%rsi
1422972bc:     	popq	%r12
1422972be:     	popq	%r13
1422972c0:     	popq	%r14
1422972c2:     	popq	%r15
1422972c4:     	popq	%rbp
1422972c5:     	retq
1422972c6:     	nopw	%cs:(%rax,%rax)
1422972d0:     	movq	%rdx, 0x10(%rsp)
1422972d5:     	pushq	%rbp
1422972d6:     	pushq	%r15
1422972d8:     	pushq	%r14
1422972da:     	pushq	%r13
1422972dc:     	pushq	%r12
1422972de:     	pushq	%rsi
1422972df:     	pushq	%rdi
1422972e0:     	pushq	%rbx
1422972e1:     	subq	$0xc8, %rsp
1422972e8:     	leaq	0x80(%rdx), %rbp
1422972ef:     	movaps	%xmm15, 0x20(%rsp)
1422972f5:     	movaps	%xmm14, 0x30(%rsp)
1422972fb:     	movaps	%xmm13, 0x40(%rsp)
142297301:     	movaps	%xmm12, 0x50(%rsp)
142297307:     	movaps	%xmm11, 0x60(%rsp)
14229730d:     	movaps	%xmm10, 0x70(%rsp)
142297313:     	movaps	%xmm9, 0x80(%rsp)
14229731c:     	movaps	%xmm8, 0x90(%rsp)
142297325:     	movaps	%xmm7, 0xa0(%rsp)
14229732d:     	movaps	%xmm6, 0xb0(%rsp)
142297335:     	cmpb	$0x0, 0x6a1(%rbp)
14229733c:     	je	0x14229735c <.text+0x229635c>
14229733e:     	movq	0x638(%rbp), %rdx
142297345:     	testq	%rdx, %rdx
142297348:     	je	0x14229735c <.text+0x229635c>
14229734a:     	movq	0x640(%rbp), %rcx
142297351:     	movl	$0x1, %r8d
142297357:     	callq	0x140613c20 <.text+0x612c20>
14229735c:     	movaps	0xb0(%rsp), %xmm6
142297364:     	movaps	0xa0(%rsp), %xmm7
14229736c:     	movaps	0x90(%rsp), %xmm8
142297375:     	movaps	0x80(%rsp), %xmm9
14229737e:     	movaps	0x70(%rsp), %xmm10
142297384:     	movaps	0x60(%rsp), %xmm11
14229738a:     	movaps	0x50(%rsp), %xmm12
142297390:     	movaps	0x40(%rsp), %xmm13
142297396:     	movaps	0x30(%rsp), %xmm14
14229739c:     	movaps	0x20(%rsp), %xmm15
1422973a2:     	addq	$0xc8, %rsp
1422973a9:     	popq	%rbx
1422973aa:     	popq	%rdi
1422973ab:     	popq	%rsi
1422973ac:     	popq	%r12
1422973ae:     	popq	%r13
1422973b0:     	popq	%r14
1422973b2:     	popq	%r15
1422973b4:     	popq	%rbp
1422973b5:     	retq
1422973b6:     	nopw	%cs:(%rax,%rax)
1422973c0:     	movq	%rdx, 0x10(%rsp)
1422973c5:     	pushq	%rbp
1422973c6:     	pushq	%r15
1422973c8:     	pushq	%r14
1422973ca:     	pushq	%r13
1422973cc:     	pushq	%r12
1422973ce:     	pushq	%rsi
1422973cf:     	pushq	%rdi
1422973d0:     	pushq	%rbx
1422973d1:     	subq	$0xc8, %rsp
1422973d8:     	leaq	0x80(%rdx), %rbp
1422973df:     	movaps	%xmm15, 0x20(%rsp)
1422973e5:     	movaps	%xmm14, 0x30(%rsp)
1422973eb:     	movaps	%xmm13, 0x40(%rsp)
1422973f1:     	movaps	%xmm12, 0x50(%rsp)
1422973f7:     	movaps	%xmm11, 0x60(%rsp)
1422973fd:     	movaps	%xmm10, 0x70(%rsp)
142297403:     	movaps	%xmm9, 0x80(%rsp)
14229740c:     	movaps	%xmm8, 0x90(%rsp)
142297415:     	movaps	%xmm7, 0xa0(%rsp)
14229741d:     	movaps	%xmm6, 0xb0(%rsp)
142297425:     	cmpb	$0x0, 0x6a2(%rbp)
14229742c:     	je	0x14229744c <.text+0x229644c>
14229742e:     	movq	0x638(%rbp), %rdx
142297435:     	testq	%rdx, %rdx
142297438:     	je	0x14229744c <.text+0x229644c>
14229743a:     	movq	0x640(%rbp), %rcx
142297441:     	movl	$0x1, %r8d
142297447:     	callq	0x140613c20 <.text+0x612c20>
14229744c:     	movaps	0xb0(%rsp), %xmm6
142297454:     	movaps	0xa0(%rsp), %xmm7
14229745c:     	movaps	0x90(%rsp), %xmm8
142297465:     	movaps	0x80(%rsp), %xmm9
14229746e:     	movaps	0x70(%rsp), %xmm10
142297474:     	movaps	0x60(%rsp), %xmm11
14229747a:     	movaps	0x50(%rsp), %xmm12
142297480:     	movaps	0x40(%rsp), %xmm13
142297486:     	movaps	0x30(%rsp), %xmm14
14229748c:     	movaps	0x20(%rsp), %xmm15
142297492:     	addq	$0xc8, %rsp
142297499:     	popq	%rbx
14229749a:     	popq	%rdi
14229749b:     	popq	%rsi
14229749c:     	popq	%r12
14229749e:     	popq	%r13
1422974a0:     	popq	%r14
1422974a2:     	popq	%r15
1422974a4:     	popq	%rbp
1422974a5:     	retq
1422974a6:     	nopw	%cs:(%rax,%rax)
1422974b0:     	movq	%rdx, 0x10(%rsp)
1422974b5:     	pushq	%rbp
1422974b6:     	pushq	%r15
1422974b8:     	pushq	%r14
1422974ba:     	pushq	%r13
1422974bc:     	pushq	%r12
1422974be:     	pushq	%rsi
1422974bf:     	pushq	%rdi
1422974c0:     	pushq	%rbx
1422974c1:     	subq	$0xc8, %rsp
1422974c8:     	leaq	0x80(%rdx), %rbp
1422974cf:     	movaps	%xmm15, 0x20(%rsp)
1422974d5:     	movaps	%xmm14, 0x30(%rsp)
1422974db:     	movaps	%xmm13, 0x40(%rsp)
1422974e1:     	movaps	%xmm12, 0x50(%rsp)
1422974e7:     	movaps	%xmm11, 0x60(%rsp)
1422974ed:     	movaps	%xmm10, 0x70(%rsp)
1422974f3:     	movaps	%xmm9, 0x80(%rsp)
1422974fc:     	movaps	%xmm8, 0x90(%rsp)
142297505:     	movaps	%xmm7, 0xa0(%rsp)
14229750d:     	movaps	%xmm6, 0xb0(%rsp)
142297515:     	cmpb	$0x0, 0x6a3(%rbp)
14229751c:     	je	0x14229753c <.text+0x229653c>
14229751e:     	movq	0x638(%rbp), %rdx
142297525:     	testq	%rdx, %rdx
142297528:     	je	0x14229753c <.text+0x229653c>
14229752a:     	movq	0x640(%rbp), %rcx
142297531:     	movl	$0x1, %r8d
142297537:     	callq	0x140613c20 <.text+0x612c20>
14229753c:     	movaps	0xb0(%rsp), %xmm6
142297544:     	movaps	0xa0(%rsp), %xmm7
14229754c:     	movaps	0x90(%rsp), %xmm8
142297555:     	movaps	0x80(%rsp), %xmm9
14229755e:     	movaps	0x70(%rsp), %xmm10
142297564:     	movaps	0x60(%rsp), %xmm11
14229756a:     	movaps	0x50(%rsp), %xmm12
142297570:     	movaps	0x40(%rsp), %xmm13
142297576:     	movaps	0x30(%rsp), %xmm14
14229757c:     	movaps	0x20(%rsp), %xmm15
142297582:     	addq	$0xc8, %rsp
142297589:     	popq	%rbx
14229758a:     	popq	%rdi
14229758b:     	popq	%rsi
14229758c:     	popq	%r12
14229758e:     	popq	%r13
142297590:     	popq	%r14
142297592:     	popq	%r15
142297594:     	popq	%rbp
142297595:     	retq
142297596:     	nopw	%cs:(%rax,%rax)
1422975a0:     	movq	%rdx, 0x10(%rsp)
1422975a5:     	pushq	%rbp
1422975a6:     	pushq	%r15
1422975a8:     	pushq	%r14
1422975aa:     	pushq	%r13
1422975ac:     	pushq	%r12
1422975ae:     	pushq	%rsi
1422975af:     	pushq	%rdi
1422975b0:     	pushq	%rbx
1422975b1:     	subq	$0xc8, %rsp
1422975b8:     	leaq	0x80(%rdx), %rbp
1422975bf:     	movaps	%xmm15, 0x20(%rsp)
1422975c5:     	movaps	%xmm14, 0x30(%rsp)
1422975cb:     	movaps	%xmm13, 0x40(%rsp)
1422975d1:     	movaps	%xmm12, 0x50(%rsp)
1422975d7:     	movaps	%xmm11, 0x60(%rsp)
1422975dd:     	movaps	%xmm10, 0x70(%rsp)
1422975e3:     	movaps	%xmm9, 0x80(%rsp)
1422975ec:     	movaps	%xmm8, 0x90(%rsp)
1422975f5:     	movaps	%xmm7, 0xa0(%rsp)
1422975fd:     	movaps	%xmm6, 0xb0(%rsp)
142297605:     	cmpb	$0x0, 0x6a4(%rbp)
14229760c:     	je	0x14229762c <.text+0x229662c>
14229760e:     	movq	0x638(%rbp), %rdx
142297615:     	testq	%rdx, %rdx
142297618:     	je	0x14229762c <.text+0x229662c>
14229761a:     	movq	0x640(%rbp), %rcx
142297621:     	movl	$0x1, %r8d
142297627:     	callq	0x140613c20 <.text+0x612c20>
14229762c:     	movaps	0xb0(%rsp), %xmm6
142297634:     	movaps	0xa0(%rsp), %xmm7
14229763c:     	movaps	0x90(%rsp), %xmm8
142297645:     	movaps	0x80(%rsp), %xmm9
14229764e:     	movaps	0x70(%rsp), %xmm10
142297654:     	movaps	0x60(%rsp), %xmm11
14229765a:     	movaps	0x50(%rsp), %xmm12
142297660:     	movaps	0x40(%rsp), %xmm13
142297666:     	movaps	0x30(%rsp), %xmm14
14229766c:     	movaps	0x20(%rsp), %xmm15
142297672:     	addq	$0xc8, %rsp
142297679:     	popq	%rbx
14229767a:     	popq	%rdi
14229767b:     	popq	%rsi
14229767c:     	popq	%r12
14229767e:     	popq	%r13
142297680:     	popq	%r14
142297682:     	popq	%r15
142297684:     	popq	%rbp
142297685:     	retq
142297686:     	nopw	%cs:(%rax,%rax)
142297690:     	movq	%rdx, 0x10(%rsp)
142297695:     	pushq	%rbp
142297696:     	pushq	%r15
142297698:     	pushq	%r14
14229769a:     	pushq	%r13
14229769c:     	pushq	%r12
14229769e:     	pushq	%rsi
14229769f:     	pushq	%rdi
1422976a0:     	pushq	%rbx
1422976a1:     	subq	$0xc8, %rsp
1422976a8:     	leaq	0x80(%rdx), %rbp
1422976af:     	movaps	%xmm15, 0x20(%rsp)
1422976b5:     	movaps	%xmm14, 0x30(%rsp)
1422976bb:     	movaps	%xmm13, 0x40(%rsp)
1422976c1:     	movaps	%xmm12, 0x50(%rsp)
1422976c7:     	movaps	%xmm11, 0x60(%rsp)
1422976cd:     	movaps	%xmm10, 0x70(%rsp)
1422976d3:     	movaps	%xmm9, 0x80(%rsp)
1422976dc:     	movaps	%xmm8, 0x90(%rsp)
1422976e5:     	movaps	%xmm7, 0xa0(%rsp)
1422976ed:     	movaps	%xmm6, 0xb0(%rsp)
1422976f5:     	cmpb	$0x0, 0x6dd(%rbp)
1422976fc:     	je	0x14229771c <.text+0x229671c>
1422976fe:     	movq	0x638(%rbp), %rdx
142297705:     	testq	%rdx, %rdx
142297708:     	je	0x14229771c <.text+0x229671c>
14229770a:     	movq	0x640(%rbp), %rcx
142297711:     	movl	$0x1, %r8d
142297717:     	callq	0x140613c20 <.text+0x612c20>
14229771c:     	movaps	0xb0(%rsp), %xmm6
142297724:     	movaps	0xa0(%rsp), %xmm7
14229772c:     	movaps	0x90(%rsp), %xmm8
142297735:     	movaps	0x80(%rsp), %xmm9
14229773e:     	movaps	0x70(%rsp), %xmm10
142297744:     	movaps	0x60(%rsp), %xmm11
14229774a:     	movaps	0x50(%rsp), %xmm12
142297750:     	movaps	0x40(%rsp), %xmm13
142297756:     	movaps	0x30(%rsp), %xmm14
14229775c:     	movaps	0x20(%rsp), %xmm15
142297762:     	addq	$0xc8, %rsp
142297769:     	popq	%rbx
14229776a:     	popq	%rdi
14229776b:     	popq	%rsi
14229776c:     	popq	%r12
14229776e:     	popq	%r13
142297770:     	popq	%r14
142297772:     	popq	%r15
142297774:     	popq	%rbp
142297775:     	retq
142297776:     	nopw	%cs:(%rax,%rax)
142297780:     	movq	%rdx, 0x10(%rsp)
142297785:     	pushq	%rbp
142297786:     	pushq	%r15
142297788:     	pushq	%r14
14229778a:     	pushq	%r13
14229778c:     	pushq	%r12
14229778e:     	pushq	%rsi
14229778f:     	pushq	%rdi
142297790:     	pushq	%rbx
142297791:     	subq	$0xc8, %rsp
142297798:     	leaq	0x80(%rdx), %rbp
14229779f:     	movaps	%xmm15, 0x20(%rsp)
1422977a5:     	movaps	%xmm14, 0x30(%rsp)
1422977ab:     	movaps	%xmm13, 0x40(%rsp)
1422977b1:     	movaps	%xmm12, 0x50(%rsp)
1422977b7:     	movaps	%xmm11, 0x60(%rsp)
1422977bd:     	movaps	%xmm10, 0x70(%rsp)
1422977c3:     	movaps	%xmm9, 0x80(%rsp)
1422977cc:     	movaps	%xmm8, 0x90(%rsp)
1422977d5:     	movaps	%xmm7, 0xa0(%rsp)
1422977dd:     	movaps	%xmm6, 0xb0(%rsp)
1422977e5:     	cmpb	$0x0, 0x6a5(%rbp)
1422977ec:     	je	0x14229785b <.text+0x229685b>
1422977ee:     	leaq	0x530(%rbp), %rcx
1422977f5:     	callq	0x140895470 <.text+0x894470>
1422977fa:     	movb	$0x1, 0x6eb(%rbp)
142297801:     	movaps	0xb0(%rsp), %xmm6
142297809:     	movaps	0xa0(%rsp), %xmm7
142297811:     	movaps	0x90(%rsp), %xmm8
14229781a:     	movaps	0x80(%rsp), %xmm9
142297823:     	movaps	0x70(%rsp), %xmm10
142297829:     	movaps	0x60(%rsp), %xmm11
14229782f:     	movaps	0x50(%rsp), %xmm12
142297835:     	movaps	0x40(%rsp), %xmm13
14229783b:     	movaps	0x30(%rsp), %xmm14
142297841:     	movaps	0x20(%rsp), %xmm15
142297847:     	addq	$0xc8, %rsp
14229784e:     	popq	%rbx
14229784f:     	popq	%rdi
142297850:     	popq	%rsi
142297851:     	popq	%r12
142297853:     	popq	%r13
142297855:     	popq	%r14
142297857:     	popq	%r15
142297859:     	popq	%rbp
14229785a:     	retq
14229785b:     	movb	$0x1, 0x6eb(%rbp)
142297862:     	movaps	0xb0(%rsp), %xmm6
14229786a:     	movaps	0xa0(%rsp), %xmm7
142297872:     	movaps	0x90(%rsp), %xmm8
14229787b:     	movaps	0x80(%rsp), %xmm9
142297884:     	movaps	0x70(%rsp), %xmm10
14229788a:     	movaps	0x60(%rsp), %xmm11
142297890:     	movaps	0x50(%rsp), %xmm12
142297896:     	movaps	0x40(%rsp), %xmm13
14229789c:     	movaps	0x30(%rsp), %xmm14
1422978a2:     	movaps	0x20(%rsp), %xmm15
1422978a8:     	addq	$0xc8, %rsp
1422978af:     	popq	%rbx
1422978b0:     	popq	%rdi
1422978b1:     	popq	%rsi
1422978b2:     	popq	%r12
1422978b4:     	popq	%r13
1422978b6:     	popq	%r14
1422978b8:     	popq	%r15
1422978ba:     	popq	%rbp
1422978bb:     	retq
1422978bc:     	nopl	(%rax)
1422978c0:     	movq	%rdx, 0x10(%rsp)
1422978c5:     	pushq	%rbp
1422978c6:     	pushq	%r15
1422978c8:     	pushq	%r14
1422978ca:     	pushq	%r13
1422978cc:     	pushq	%r12
1422978ce:     	pushq	%rsi
1422978cf:     	pushq	%rdi
1422978d0:     	pushq	%rbx
1422978d1:     	subq	$0xc8, %rsp
1422978d8:     	leaq	0x80(%rdx), %rbp
1422978df:     	movaps	%xmm15, 0x20(%rsp)
1422978e5:     	movaps	%xmm14, 0x30(%rsp)
1422978eb:     	movaps	%xmm13, 0x40(%rsp)
1422978f1:     	movaps	%xmm12, 0x50(%rsp)
1422978f7:     	movaps	%xmm11, 0x60(%rsp)
1422978fd:     	movaps	%xmm10, 0x70(%rsp)
142297903:     	movaps	%xmm9, 0x80(%rsp)
14229790c:     	movaps	%xmm8, 0x90(%rsp)
142297915:     	movaps	%xmm7, 0xa0(%rsp)
14229791d:     	movaps	%xmm6, 0xb0(%rsp)
142297925:     	cmpb	$0x0, 0x6eb(%rbp)
14229792c:     	je	0x14229794c <.text+0x229694c>
14229792e:     	movq	0x638(%rbp), %rdx
142297935:     	testq	%rdx, %rdx
142297938:     	je	0x14229794c <.text+0x229694c>
14229793a:     	movq	0x640(%rbp), %rcx
142297941:     	movl	$0x1, %r8d
142297947:     	callq	0x140613c20 <.text+0x612c20>
14229794c:     	movaps	0xb0(%rsp), %xmm6
142297954:     	movaps	0xa0(%rsp), %xmm7
14229795c:     	movaps	0x90(%rsp), %xmm8
142297965:     	movaps	0x80(%rsp), %xmm9
14229796e:     	movaps	0x70(%rsp), %xmm10
142297974:     	movaps	0x60(%rsp), %xmm11
14229797a:     	movaps	0x50(%rsp), %xmm12
142297980:     	movaps	0x40(%rsp), %xmm13
142297986:     	movaps	0x30(%rsp), %xmm14
14229798c:     	movaps	0x20(%rsp), %xmm15
142297992:     	addq	$0xc8, %rsp
142297999:     	popq	%rbx
14229799a:     	popq	%rdi
14229799b:     	popq	%rsi
14229799c:     	popq	%r12
14229799e:     	popq	%r13
1422979a0:     	popq	%r14
1422979a2:     	popq	%r15
1422979a4:     	popq	%rbp
1422979a5:     	retq
