
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141553120:     	pushq	%rbp
141553121:     	pushq	%r15
141553123:     	pushq	%r14
141553125:     	pushq	%r13
141553127:     	pushq	%r12
141553129:     	pushq	%rsi
14155312a:     	pushq	%rdi
14155312b:     	pushq	%rbx
14155312c:     	subq	$0x1d8, %rsp            # imm = 0x1D8
141553133:     	leaq	0x80(%rsp), %rbp
14155313b:     	movaps	%xmm11, 0x140(%rbp)
141553143:     	movaps	%xmm10, 0x130(%rbp)
14155314b:     	movaps	%xmm9, 0x120(%rbp)
141553153:     	movaps	%xmm8, 0x110(%rbp)
14155315b:     	movaps	%xmm7, 0x100(%rbp)
141553162:     	movaps	%xmm6, 0xf0(%rbp)
141553169:     	movq	$-0x2, 0xe8(%rbp)
141553174:     	movq	%r9, %rbx
141553177:     	movq	%r8, 0xd8(%rbp)
14155317e:     	movq	%rdx, %r14
141553181:     	movq	%rcx, %rdi
141553184:     	leaq	0x171964d(%rip), %rdx   # 0x142c6c7d8
14155318b:     	leaq	0x60(%rbp), %rcx
14155318f:     	movl	$0x25, %r8d
141553195:     	callq	0x140b0eeb0 <.text+0xb0deb0>
14155319a:     	xorl	%eax, %eax
14155319c:     	cmpq	0x60(%rbp), %rax
1415531a0:     	jo	0x141553717 <.text+0x1552717>
1415531a6:     	movq	0xd0(%rbp), %rax
1415531ad:     	movq	%rax, 0x40(%rbp)
1415531b1:     	movups	0xc0(%rbp), %xmm0
1415531b8:     	movaps	%xmm0, 0x30(%rbp)
1415531bc:     	movups	0xb0(%rbp), %xmm0
1415531c3:     	movaps	%xmm0, 0x20(%rbp)
1415531c7:     	movups	0xa0(%rbp), %xmm0
1415531ce:     	movaps	%xmm0, 0x10(%rbp)
1415531d2:     	movups	0x60(%rbp), %xmm0
1415531d6:     	movups	0x70(%rbp), %xmm1
1415531da:     	movups	0x80(%rbp), %xmm2
1415531e1:     	movups	0x90(%rbp), %xmm3
1415531e8:     	movaps	%xmm3, (%rbp)
1415531ec:     	movaps	%xmm2, -0x10(%rbp)
1415531f0:     	movaps	%xmm1, -0x20(%rbp)
1415531f4:     	movaps	%xmm0, -0x30(%rbp)
1415531f8:     	movq	(%rdi), %rsi
1415531fb:     	movq	(%r14), %r12
1415531fe:     	movq	%r12, 0x20(%rsp)
141553203:     	leaq	0x1719646(%rip), %rdx   # 0x142c6c850
14155320a:     	leaq	-0x30(%rbp), %rcx
14155320e:     	movl	$0x12, %r8d
141553214:     	movq	%rsi, %r9
141553217:     	callq	0x140b1aca0 <.text+0xb19ca0>
14155321c:     	movq	%rax, %r14
14155321f:     	movq	(%rbx), %rdi
141553222:     	leaq	0x1719639(%rip), %rdx   # 0x142c6c862
141553229:     	movl	$0x16, %r8d
14155322f:     	movq	%rdi, %rcx
141553232:     	callq	0x140937aa0 <.text+0x936aa0>
141553237:     	testb	$0x1, %al
141553239:     	je	0x141553749 <.text+0x1552749>
14155323f:     	movq	%rdx, %r15
141553242:     	movq	%r12, 0x58(%rbp)
141553246:     	movq	%rdi, 0x50(%rbp)
14155324a:     	movq	%rsi, 0x48(%rbp)
14155324e:     	leaq	0x60(%rbp), %rcx
141553252:     	movq	0xd8(%rbp), %rdx
141553259:     	callq	0x140d146a0 <.text+0xd136a0>
14155325e:     	movq	0x68(%rbp), %r12
141553262:     	testq	%r12, %r12
141553265:     	jne	0x14155326b <.text+0x155226b>
141553267:     	movq	0x70(%rbp), %r12
14155326b:     	movq	0x1d0(%rbp), %rsi
141553272:     	movq	0x1c0(%rbp), %rdi
141553279:     	movq	0x80(%rbp), %rbx
141553280:     	movq	(%r12), %rax
141553284:     	movq	0x10(%r12), %r13
141553289:     	subq	%r13, %rax
14155328c:     	cmpq	$0x77, %rax
141553290:     	jbe	0x1415536cd <.text+0x15526cd>
141553296:     	movq	0x8(%r12), %rax
14155329b:     	leaq	-0x2794c2(%rip), %rcx   # 0x1412d9de0 <.text+0x12d8de0>
1415532a2:     	movq	%rcx, (%rax,%r13)
1415532a6:     	movaps	0x13eae12(%rip), %xmm8  # 0x14293e0c0
1415532ae:     	movups	%xmm8, 0x8(%rax,%r13)
1415532b4:     	movq	$0x0, 0x18(%rax,%r13)
1415532bd:     	movl	$0x0, 0x20(%rax,%r13)
1415532c6:     	movabsq	$0x3f8000003f800000, %rcx # imm = 0x3F8000003F800000
1415532d0:     	movq	%rcx, 0x24(%rax,%r13)
1415532d5:     	movl	$0x3f800000, 0x2c(%rax,%r13) # imm = 0x3F800000
1415532de:     	movq	%r14, 0x38(%rax,%r13)
1415532e3:     	movq	%r15, 0x40(%rax,%r13)
1415532e8:     	movb	$0x0, 0x48(%rax,%r13)
1415532ee:     	movq	%rbx, 0x58(%rax,%r13)
1415532f3:     	leaq	0x11a36(%rip), %rcx     # 0x141564d30 <.text+0x1563d30>
1415532fa:     	movq	%rcx, 0x68(%rax,%r13)
1415532ff:     	addq	$0x78, %r13
141553303:     	movq	%r13, 0x10(%r12)
141553308:     	movl	0x1c(%rdi), %eax
14155330b:     	movq	0x10(%rdi), %rcx
14155330f:     	movl	%eax, (%rcx)
141553311:     	movq	(%rdi), %r14
141553314:     	movq	(%rsi), %rax
141553317:     	movzbl	0x58(%rax), %eax
14155331b:     	shll	$0x3, %eax
14155331e:     	leaq	0x17198a3(%rip), %rcx   # 0x142c6cbc8
141553325:     	movq	(%rax,%rcx), %r9
141553329:     	leaq	0x17198c0(%rip), %rcx   # 0x142c6cbf0
141553330:     	movq	(%rax,%rcx), %rax
141553334:     	movq	%rax, 0x20(%rsp)
141553339:     	leaq	0x159c1da(%rip), %rdx   # 0x142aef51a
141553340:     	leaq	-0x30(%rbp), %r15
141553344:     	movl	$0xc, %r8d
14155334a:     	movq	%r15, %rcx
14155334d:     	callq	0x140d62810 <.text+0xd61810>
141553352:     	movq	%r15, %rcx
141553355:     	callq	0x140d76ed0 <.text+0xd75ed0>
14155335a:     	movq	%rdx, %r8
14155335d:     	movq	$0x14, 0x20(%rsp)
141553366:     	leaq	0x1719523(%rip), %r9    # 0x142c6c890
14155336d:     	leaq	0x60(%rbp), %rcx
141553371:     	movq	%rax, %rdx
141553374:     	callq	0x140d62810 <.text+0xd61810>
141553379:     	movq	0x1c8(%rbp), %r8
141553380:     	leaq	-0x50(%rbp), %rcx
141553384:     	leaq	0x60(%rbp), %r9
141553388:     	movq	%r14, %rdx
14155338b:     	callq	0x1409365d0 <.text+0x9355d0>
141553390:     	movq	-0x48(%rbp), %rax
141553394:     	movq	%rax, 0xe0(%rbp)
14155339b:     	movq	-0x40(%rbp), %rax
14155339f:     	testq	%rax, %rax
1415533a2:     	je	0x1415534d1 <.text+0x15524d1>
1415533a8:     	shlq	$0x4, %rax
1415533ac:     	leaq	(%rax,%rax,2), %r13
1415533b0:     	xorl	%ebx, %ebx
1415533b2:     	movss	0x13d24b5(%rip), %xmm9  # 0x142925870
1415533bb:     	xorps	%xmm10, %xmm10
1415533bf:     	leaq	-0x278816(%rip), %rdi   # 0x1412dabb0 <.text+0x12d9bb0>
1415533c6:     	movabsq	$0x13fa53c58fdddee8, %rsi # imm = 0x13FA53C58FDDDEE8
1415533d0:     	movq	0xe0(%rbp), %r14
1415533d7:     	movss	0x10(%r14,%rbx), %xmm7
1415533de:     	mulss	%xmm9, %xmm7
1415533e3:     	movaps	%xmm7, %xmm0
1415533e6:     	callq	0x142923fd0 <.text+0x2922fd0>
1415533eb:     	movaps	%xmm0, %xmm6
1415533ee:     	movaps	%xmm7, %xmm0
1415533f1:     	callq	0x142923e80 <.text+0x2922e80>
1415533f6:     	movaps	%xmm0, %xmm7
1415533f9:     	movups	(%r14,%rbx), %xmm11
1415533fe:     	leaq	0x60(%rbp), %rcx
141553402:     	movq	0xd8(%rbp), %rdx
141553409:     	callq	0x140d146a0 <.text+0xd136a0>
14155340e:     	movq	0x68(%rbp), %r15
141553412:     	testq	%r15, %r15
141553415:     	jne	0x14155341b <.text+0x155241b>
141553417:     	movq	0x70(%rbp), %r15
14155341b:     	movq	0x80(%rbp), %r14
141553422:     	movq	(%r15), %rax
141553425:     	movq	0x10(%r15), %r12
141553429:     	subq	%r12, %rax
14155342c:     	cmpq	$0x87, %rax
141553432:     	jbe	0x1415534ac <.text+0x15524ac>
141553434:     	movaps	%xmm6, %xmm0
141553437:     	mulss	%xmm10, %xmm0
14155343c:     	movaps	%xmm0, %xmm1
14155343f:     	unpcklps	%xmm7, %xmm1            # xmm1 = xmm1[0],xmm7[0],xmm1[1],xmm7[1]
141553442:     	unpcklps	%xmm6, %xmm0            # xmm0 = xmm0[0],xmm6[0],xmm0[1],xmm6[1]
141553445:     	movlhps	%xmm1, %xmm0            # xmm0 = xmm0[0],xmm1[0]
141553448:     	movq	0x8(%r15), %rax
14155344c:     	movq	%rdi, (%rax,%r12)
141553450:     	movups	%xmm0, 0x8(%rax,%r12)
141553456:     	movups	%xmm11, 0x18(%rax,%r12)
14155345c:     	shufps	$0xff, %xmm11, %xmm11   # xmm11 = xmm11[3,3,3,3]
141553461:     	movlps	%xmm11, 0x28(%rax,%r12)
141553467:     	movl	$0x0, 0x38(%rax,%r12)
141553470:     	movq	%rsi, 0x40(%rax,%r12)
141553475:     	movb	$0xe, 0x48(%rax,%r12)
14155347b:     	movb	$0x0, 0x58(%rax,%r12)
141553481:     	movq	%r14, 0x68(%rax,%r12)
141553486:     	leaq	0x118a3(%rip), %rcx     # 0x141564d30 <.text+0x1563d30>
14155348d:     	movq	%rcx, 0x78(%rax,%r12)
141553492:     	addq	$0x88, %r12
141553499:     	movq	%r12, 0x10(%r15)
14155349d:     	addq	$0x30, %rbx
1415534a1:     	cmpq	%rbx, %r13
1415534a4:     	jne	0x1415533d0 <.text+0x15523d0>
1415534aa:     	jmp	0x1415534d1 <.text+0x15524d1>
1415534ac:     	movq	$0x1, 0x20(%rsp)
1415534b5:     	movl	$0x88, %r8d
1415534bb:     	movl	$0x1, %r9d
1415534c1:     	movq	%r15, %rcx
1415534c4:     	movq	%r12, %rdx
1415534c7:     	callq	0x1428a9a60 <.text+0x28a8a60>
1415534cc:     	jmp	0x141553434 <.text+0x1552434>
1415534d1:     	movq	-0x50(%rbp), %rax
1415534d5:     	testq	%rax, %rax
1415534d8:     	je	0x1415534f4 <.text+0x15524f4>
1415534da:     	shlq	$0x4, %rax
1415534de:     	leaq	(%rax,%rax,2), %rdx
1415534e2:     	movl	$0x4, %r8d
1415534e8:     	movq	0xe0(%rbp), %rcx
1415534ef:     	callq	0x140613c20 <.text+0x612c20>
1415534f4:     	movq	-0x30(%rbp), %rdx
1415534f8:     	testq	%rdx, %rdx
1415534fb:     	movq	0x48(%rbp), %rsi
1415534ff:     	movq	0x50(%rbp), %rdi
141553503:     	movq	0x58(%rbp), %rbx
141553507:     	je	0x141553518 <.text+0x1552518>
141553509:     	movq	-0x28(%rbp), %rcx
14155350d:     	movl	$0x1, %r8d
141553513:     	callq	0x140613c20 <.text+0x612c20>
141553518:     	leaq	0x1719385(%rip), %rdx   # 0x142c6c8a4
14155351f:     	leaq	0x60(%rbp), %rcx
141553523:     	movl	$0x29, %r8d
141553529:     	callq	0x140b0eeb0 <.text+0xb0deb0>
14155352e:     	xorl	%eax, %eax
141553530:     	cmpq	0x60(%rbp), %rax
141553534:     	jo	0x141553755 <.text+0x1552755>
14155353a:     	movq	0xd0(%rbp), %rax
141553541:     	movq	%rax, 0x40(%rbp)
141553545:     	movups	0xc0(%rbp), %xmm0
14155354c:     	movaps	%xmm0, 0x30(%rbp)
141553550:     	movups	0xb0(%rbp), %xmm0
141553557:     	movaps	%xmm0, 0x20(%rbp)
14155355b:     	movups	0xa0(%rbp), %xmm0
141553562:     	movaps	%xmm0, 0x10(%rbp)
141553566:     	movups	0x60(%rbp), %xmm0
14155356a:     	movups	0x70(%rbp), %xmm1
14155356e:     	movups	0x80(%rbp), %xmm2
141553575:     	movups	0x90(%rbp), %xmm3
14155357c:     	movaps	%xmm3, (%rbp)
141553580:     	movaps	%xmm2, -0x10(%rbp)
141553584:     	movaps	%xmm1, -0x20(%rbp)
141553588:     	movaps	%xmm0, -0x30(%rbp)
14155358c:     	movq	%rbx, 0x20(%rsp)
141553591:     	leaq	0x1719350(%rip), %rdx   # 0x142c6c8e8
141553598:     	leaq	-0x30(%rbp), %rcx
14155359c:     	movl	$0xb, %r8d
1415535a2:     	movq	%rsi, %r9
1415535a5:     	callq	0x140b1aca0 <.text+0xb19ca0>
1415535aa:     	movq	%rax, %r12
1415535ad:     	leaq	0x171933f(%rip), %rdx   # 0x142c6c8f3
1415535b4:     	movl	$0xd, %r8d
1415535ba:     	movq	%rdi, %rcx
1415535bd:     	callq	0x140937aa0 <.text+0x936aa0>
1415535c2:     	testb	$0x1, %al
1415535c4:     	je	0x141553787 <.text+0x1552787>
1415535ca:     	movq	%rdx, %rbx
1415535cd:     	leaq	0x1719344(%rip), %rcx   # 0x142c6c918
1415535d4:     	movl	$0xe, %edx
1415535d9:     	callq	0x140c9cd80 <.text+0xc9bd80>
1415535de:     	movq	%rax, %r14
1415535e1:     	leaq	0x60(%rbp), %rcx
1415535e5:     	movq	0xd8(%rbp), %rdx
1415535ec:     	callq	0x140d146a0 <.text+0xd136a0>
1415535f1:     	movq	0x68(%rbp), %rsi
1415535f5:     	testq	%rsi, %rsi
1415535f8:     	jne	0x1415535fe <.text+0x15525fe>
1415535fa:     	movq	0x70(%rbp), %rsi
1415535fe:     	movq	0x80(%rbp), %rdi
141553605:     	movq	(%rsi), %rax
141553608:     	movq	0x10(%rsi), %r15
14155360c:     	subq	%r15, %rax
14155360f:     	cmpq	$0x87, %rax
141553615:     	jbe	0x1415536f2 <.text+0x15526f2>
14155361b:     	movq	0x8(%rsi), %rax
14155361f:     	leaq	-0x27a3b6(%rip), %rcx   # 0x1412d9270 <.text+0x12d8270>
141553626:     	movq	%rcx, (%rax,%r15)
14155362a:     	movups	%xmm8, 0x8(%rax,%r15)
141553630:     	movq	$0x0, 0x18(%rax,%r15)
141553639:     	movl	$0x0, 0x20(%rax,%r15)
141553642:     	movabsq	$0x3f8000003f800000, %rcx # imm = 0x3F8000003F800000
14155364c:     	movq	%rcx, 0x24(%rax,%r15)
141553651:     	movl	$0x3f800000, 0x2c(%rax,%r15) # imm = 0x3F800000
14155365a:     	movq	%r12, 0x38(%rax,%r15)
14155365f:     	movq	%rbx, 0x40(%rax,%r15)
141553664:     	movq	%r14, 0x48(%rax,%r15)
141553669:     	movb	$0x0, 0x58(%rax,%r15)
14155366f:     	movq	%rdi, 0x68(%rax,%r15)
141553674:     	leaq	0x116b5(%rip), %rcx     # 0x141564d30 <.text+0x1563d30>
14155367b:     	movq	%rcx, 0x78(%rax,%r15)
141553680:     	addq	$0x88, %r15
141553687:     	movq	%r15, 0x10(%rsi)
14155368b:     	movaps	0xf0(%rbp), %xmm6
141553692:     	movaps	0x100(%rbp), %xmm7
141553699:     	movaps	0x110(%rbp), %xmm8
1415536a1:     	movaps	0x120(%rbp), %xmm9
1415536a9:     	movaps	0x130(%rbp), %xmm10
1415536b1:     	movaps	0x140(%rbp), %xmm11
1415536b9:     	addq	$0x1d8, %rsp            # imm = 0x1D8
1415536c0:     	popq	%rbx
1415536c1:     	popq	%rdi
1415536c2:     	popq	%rsi
1415536c3:     	popq	%r12
1415536c5:     	popq	%r13
1415536c7:     	popq	%r14
1415536c9:     	popq	%r15
1415536cb:     	popq	%rbp
1415536cc:     	retq
1415536cd:     	movq	$0x1, 0x20(%rsp)
1415536d6:     	movl	$0x78, %r8d
1415536dc:     	movl	$0x1, %r9d
1415536e2:     	movq	%r12, %rcx
1415536e5:     	movq	%r13, %rdx
1415536e8:     	callq	0x1428a9a60 <.text+0x28a8a60>
1415536ed:     	jmp	0x141553296 <.text+0x1552296>
1415536f2:     	movq	$0x1, 0x20(%rsp)
1415536fb:     	movl	$0x88, %r8d
141553701:     	movl	$0x1, %r9d
141553707:     	movq	%rsi, %rcx
14155370a:     	movq	%r15, %rdx
14155370d:     	callq	0x1428a9a60 <.text+0x28a8a60>
141553712:     	jmp	0x14155361b <.text+0x155261b>
141553717:     	movq	0x68(%rbp), %rax
14155371b:     	movq	%rax, -0x30(%rbp)
14155371f:     	leaq	0x1719112(%rip), %rax   # 0x142c6c838
141553726:     	movq	%rax, 0x20(%rsp)
14155372b:     	leaq	0x1718a8e(%rip), %rcx   # 0x142c6c1c0
141553732:     	leaq	0x1718a67(%rip), %r9    # 0x142c6c1a0
141553739:     	leaq	-0x30(%rbp), %r8
14155373d:     	movl	$0x2b, %edx
141553742:     	callq	0x1428d9760 <.text+0x28d8760>
141553747:     	jmp	0x141553785 <.text+0x1552785>
141553749:     	leaq	0x1719128(%rip), %rcx   # 0x142c6c878
141553750:     	callq	0x1428d9310 <.text+0x28d8310>
141553755:     	movq	0x68(%rbp), %rax
141553759:     	movq	%rax, -0x30(%rbp)
14155375d:     	leaq	0x171916c(%rip), %rax   # 0x142c6c8d0
141553764:     	movq	%rax, 0x20(%rsp)
141553769:     	leaq	0x1718a50(%rip), %rcx   # 0x142c6c1c0
141553770:     	leaq	0x1718a29(%rip), %r9    # 0x142c6c1a0
141553777:     	leaq	-0x30(%rbp), %r8
14155377b:     	movl	$0x2b, %edx
141553780:     	callq	0x1428d9760 <.text+0x28d8760>
141553785:     	ud2
141553787:     	leaq	0x1719172(%rip), %rcx   # 0x142c6c900
14155378e:     	callq	0x1428d9310 <.text+0x28d8310>
141553793:     	int3
141553794:     	nopw	%cs:(%rax,%rax)
1415537a0:     	movq	%rdx, 0x10(%rsp)
1415537a5:     	pushq	%rbp
1415537a6:     	pushq	%r15
1415537a8:     	pushq	%r14
1415537aa:     	pushq	%r13
1415537ac:     	pushq	%r12
1415537ae:     	pushq	%rsi
1415537af:     	pushq	%rdi
1415537b0:     	pushq	%rbx
1415537b1:     	subq	$0x88, %rsp
1415537b8:     	leaq	0x80(%rdx), %rbp
1415537bf:     	movaps	%xmm11, 0x20(%rsp)
1415537c5:     	movaps	%xmm10, 0x30(%rsp)
1415537cb:     	movaps	%xmm9, 0x40(%rsp)
1415537d1:     	movaps	%xmm8, 0x50(%rsp)
1415537d7:     	movaps	%xmm7, 0x60(%rsp)
1415537dc:     	movaps	%xmm6, 0x70(%rsp)
1415537e1:     	movq	-0x30(%rbp), %rdx
1415537e5:     	testq	%rdx, %rdx
1415537e8:     	je	0x1415537f9 <.text+0x15527f9>
1415537ea:     	movq	-0x28(%rbp), %rcx
1415537ee:     	movl	$0x1, %r8d
1415537f4:     	callq	0x140613c20 <.text+0x612c20>
1415537f9:     	movaps	0x70(%rsp), %xmm6
1415537fe:     	movaps	0x60(%rsp), %xmm7
141553803:     	movaps	0x50(%rsp), %xmm8
141553809:     	movaps	0x40(%rsp), %xmm9
14155380f:     	movaps	0x30(%rsp), %xmm10
141553815:     	movaps	0x20(%rsp), %xmm11
14155381b:     	addq	$0x88, %rsp
141553822:     	popq	%rbx
141553823:     	popq	%rdi
141553824:     	popq	%rsi
141553825:     	popq	%r12
141553827:     	popq	%r13
141553829:     	popq	%r14
14155382b:     	popq	%r15
14155382d:     	popq	%rbp
14155382e:     	retq
14155382f:     	nop
141553830:     	movq	%rdx, 0x10(%rsp)
141553835:     	pushq	%rbp
141553836:     	pushq	%r15
141553838:     	pushq	%r14
14155383a:     	pushq	%r13
14155383c:     	pushq	%r12
14155383e:     	pushq	%rsi
14155383f:     	pushq	%rdi
141553840:     	pushq	%rbx
141553841:     	subq	$0x88, %rsp
141553848:     	leaq	0x80(%rdx), %rbp
14155384f:     	movaps	%xmm11, 0x20(%rsp)
141553855:     	movaps	%xmm10, 0x30(%rsp)
14155385b:     	movaps	%xmm9, 0x40(%rsp)
141553861:     	movaps	%xmm8, 0x50(%rsp)
141553867:     	movaps	%xmm7, 0x60(%rsp)
14155386c:     	movaps	%xmm6, 0x70(%rsp)
141553871:     	movq	-0x50(%rbp), %rax
141553875:     	testq	%rax, %rax
141553878:     	je	0x141553894 <.text+0x1552894>
14155387a:     	shlq	$0x4, %rax
14155387e:     	leaq	(%rax,%rax,2), %rdx
141553882:     	movl	$0x4, %r8d
141553888:     	movq	0xe0(%rbp), %rcx
14155388f:     	callq	0x140613c20 <.text+0x612c20>
141553894:     	movaps	0x70(%rsp), %xmm6
141553899:     	movaps	0x60(%rsp), %xmm7
14155389e:     	movaps	0x50(%rsp), %xmm8
1415538a4:     	movaps	0x40(%rsp), %xmm9
1415538aa:     	movaps	0x30(%rsp), %xmm10
1415538b0:     	movaps	0x20(%rsp), %xmm11
1415538b6:     	addq	$0x88, %rsp
1415538bd:     	popq	%rbx
1415538be:     	popq	%rdi
1415538bf:     	popq	%rsi
1415538c0:     	popq	%r12
1415538c2:     	popq	%r13
1415538c4:     	popq	%r14
1415538c6:     	popq	%r15
1415538c8:     	popq	%rbp
1415538c9:     	retq
1415538ca:     	nopw	(%rax,%rax)
1415538d0:     	movq	%rdx, 0x10(%rsp)
1415538d5:     	pushq	%rbp
1415538d6:     	pushq	%r15
1415538d8:     	pushq	%r14
1415538da:     	pushq	%r13
1415538dc:     	pushq	%r12
1415538de:     	pushq	%rsi
1415538df:     	pushq	%rdi
1415538e0:     	pushq	%rbx
1415538e1:     	subq	$0x88, %rsp
1415538e8:     	leaq	0x80(%rdx), %rbp
1415538ef:     	movaps	%xmm11, 0x20(%rsp)
1415538f5:     	movaps	%xmm10, 0x30(%rsp)
1415538fb:     	movaps	%xmm9, 0x40(%rsp)
141553901:     	movaps	%xmm8, 0x50(%rsp)
141553907:     	movaps	%xmm7, 0x60(%rsp)
14155390c:     	movaps	%xmm6, 0x70(%rsp)
141553911:     	leaq	-0x30(%rbp), %rcx
141553915:     	callq	0x140d4cc30 <.text+0xd4bc30>
14155391a:     	movaps	0x70(%rsp), %xmm6
14155391f:     	movaps	0x60(%rsp), %xmm7
141553924:     	movaps	0x50(%rsp), %xmm8
14155392a:     	movaps	0x40(%rsp), %xmm9
141553930:     	movaps	0x30(%rsp), %xmm10
141553936:     	movaps	0x20(%rsp), %xmm11
14155393c:     	addq	$0x88, %rsp
141553943:     	popq	%rbx
141553944:     	popq	%rdi
141553945:     	popq	%rsi
141553946:     	popq	%r12
141553948:     	popq	%r13
14155394a:     	popq	%r14
14155394c:     	popq	%r15
14155394e:     	popq	%rbp
14155394f:     	retq
141553950:     	movq	%rdx, 0x10(%rsp)
141553955:     	pushq	%rbp
141553956:     	pushq	%r15
141553958:     	pushq	%r14
14155395a:     	pushq	%r13
14155395c:     	pushq	%r12
14155395e:     	pushq	%rsi
14155395f:     	pushq	%rdi
141553960:     	pushq	%rbx
141553961:     	subq	$0x88, %rsp
141553968:     	leaq	0x80(%rdx), %rbp
14155396f:     	movaps	%xmm11, 0x20(%rsp)
141553975:     	movaps	%xmm10, 0x30(%rsp)
14155397b:     	movaps	%xmm9, 0x40(%rsp)
141553981:     	movaps	%xmm8, 0x50(%rsp)
141553987:     	movaps	%xmm7, 0x60(%rsp)
14155398c:     	movaps	%xmm6, 0x70(%rsp)
141553991:     	leaq	-0x30(%rbp), %rcx
141553995:     	callq	0x140d4cc30 <.text+0xd4bc30>
14155399a:     	movaps	0x70(%rsp), %xmm6
14155399f:     	movaps	0x60(%rsp), %xmm7
1415539a4:     	movaps	0x50(%rsp), %xmm8
1415539aa:     	movaps	0x40(%rsp), %xmm9
1415539b0:     	movaps	0x30(%rsp), %xmm10
1415539b6:     	movaps	0x20(%rsp), %xmm11
1415539bc:     	addq	$0x88, %rsp
1415539c3:     	popq	%rbx
1415539c4:     	popq	%rdi
1415539c5:     	popq	%rsi
1415539c6:     	popq	%r12
1415539c8:     	popq	%r13
1415539ca:     	popq	%r14
1415539cc:     	popq	%r15
1415539ce:     	popq	%rbp
1415539cf:     	retq
