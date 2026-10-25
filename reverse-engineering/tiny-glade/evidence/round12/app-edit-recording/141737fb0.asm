
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141737fb0:     	pushq	%rbp
141737fb1:     	pushq	%rsi
141737fb2:     	pushq	%rdi
141737fb3:     	pushq	%rbx
141737fb4:     	subq	$0x428, %rsp            # imm = 0x428
141737fbb:     	leaq	0x80(%rsp), %rbp
141737fc3:     	movq	$-0x2, 0x3a0(%rbp)
141737fce:     	movq	%rcx, %rsi
141737fd1:     	movzbl	0x1efcc31(%rip), %eax   # 0x143634c09
141737fd8:     	movl	$0x2a0, %ecx            # imm = 0x2A0
141737fdd:     	movl	$0x8, %edx
141737fe2:     	callq	0x140613c10 <.text+0x612c10>
141737fe7:     	testq	%rax, %rax
141737fea:     	je	0x141738473 <.text+0x1737473>
141737ff0:     	movq	%rax, 0x398(%rbp)
141737ff7:     	xorps	%xmm0, %xmm0
141737ffa:     	movups	%xmm0, 0x278(%rbp)
141738001:     	movups	%xmm0, 0x290(%rbp)
141738008:     	movups	%xmm0, 0x2a8(%rbp)
14173800f:     	movups	%xmm0, 0x2c0(%rbp)
141738016:     	movups	%xmm0, 0x2d8(%rbp)
14173801d:     	movl	$0x0, 0x2e8(%rbp)
141738027:     	movups	%xmm0, 0x310(%rbp)
14173802e:     	movups	%xmm0, 0x328(%rbp)
141738035:     	movups	%xmm0, 0x340(%rbp)
14173803c:     	movups	%xmm0, 0x358(%rbp)
141738043:     	movups	%xmm0, 0x370(%rbp)
14173804a:     	movl	$0x0, 0x380(%rbp)
141738054:     	movq	$0x0, 0x1d8(%rbp)
14173805f:     	movq	$0x0, 0x258(%rbp)
14173806a:     	movq	$0x8, 0x260(%rbp)
141738075:     	movq	$0x0, 0x268(%rbp)
141738080:     	movq	$0x10, 0x270(%rbp)
14173808b:     	movq	$0x10, 0x288(%rbp)
141738096:     	movq	$0x10, 0x2a0(%rbp)
1417380a1:     	movq	$0x10, 0x2b8(%rbp)
1417380ac:     	movq	$0x10, 0x2d0(%rbp)
1417380b7:     	movabsq	$-0x8000000000000000, %rbx # imm = 0x8000000000000000
1417380c1:     	movq	%rbx, 0x2f0(%rbp)
1417380c8:     	leaq	0x15954c5(%rip), %rax   # 0x142ccd594
1417380cf:     	movq	%rax, 0x2f8(%rbp)
1417380d6:     	movq	$0x4b, 0x300(%rbp)
1417380e1:     	movq	$0x10, 0x308(%rbp)
1417380ec:     	movq	$0x10, 0x320(%rbp)
1417380f7:     	movq	$0x10, 0x338(%rbp)
141738102:     	movq	$0x10, 0x350(%rbp)
14173810d:     	movq	$0x10, 0x368(%rbp)
141738118:     	movl	$0x0, 0x388(%rbp)
141738122:     	movw	$0x1, 0x38c(%rbp)
14173812b:     	movl	$0x0, 0x390(%rbp)
141738135:     	movzbl	0x1efcacd(%rip), %eax   # 0x143634c09
14173813c:     	movl	$0x1c0, %ecx            # imm = 0x1C0
141738141:     	movl	$0x8, %edx
141738146:     	callq	0x140613c10 <.text+0x612c10>
14173814b:     	testq	%rax, %rax
14173814e:     	je	0x141738482 <.text+0x1737482>
141738154:     	movq	%rax, %rdi
141738157:     	leaq	0x1d8(%rbp), %rdx
14173815e:     	movl	$0x1c0, %r8d            # imm = 0x1C0
141738164:     	movq	%rax, %rcx
141738167:     	callq	0x142923c00 <.text+0x2922c00>
14173816c:     	leaq	0x159505d(%rip), %r8    # 0x142ccd1d0
141738173:     	leaq	0x168(%rbp), %rcx
14173817a:     	movq	%rdi, %rdx
14173817d:     	callq	0x140ce4b10 <.text+0xce3b10>
141738182:     	leaq	0xf8(%rbp), %rcx
141738189:     	callq	0x14143b610 <.text+0x143a610>
14173818e:     	leaq	0x88(%rbp), %rcx
141738195:     	callq	0x14143b290 <.text+0x143a290>
14173819a:     	leaq	0x18(%rbp), %rcx
14173819e:     	callq	0x140a9f240 <.text+0xa9e240>
1417381a3:     	leaq	-0x58(%rbp), %rcx
1417381a7:     	callq	0x141740460 <.text+0x173f460>
1417381ac:     	leaq	0x1d8(%rbp), %rcx
1417381b3:     	callq	0x140a26240 <.text+0xa25240>
1417381b8:     	movups	0x1c8(%rbp), %xmm0
1417381bf:     	movq	0x398(%rbp), %rax
1417381c6:     	movups	%xmm0, 0x60(%rax)
1417381ca:     	movups	0x1b8(%rbp), %xmm0
1417381d1:     	movups	%xmm0, 0x50(%rax)
1417381d5:     	movups	0x1a8(%rbp), %xmm0
1417381dc:     	movups	%xmm0, 0x40(%rax)
1417381e0:     	movups	0x168(%rbp), %xmm0
1417381e7:     	movups	0x178(%rbp), %xmm1
1417381ee:     	movups	0x188(%rbp), %xmm2
1417381f5:     	movups	0x198(%rbp), %xmm3
1417381fc:     	movups	%xmm3, 0x30(%rax)
141738200:     	movups	%xmm2, 0x20(%rax)
141738204:     	movups	%xmm1, 0x10(%rax)
141738208:     	movups	%xmm0, (%rax)
14173820b:     	movups	0xf8(%rbp), %xmm0
141738212:     	movups	0x108(%rbp), %xmm1
141738219:     	movups	0x118(%rbp), %xmm2
141738220:     	movups	0x128(%rbp), %xmm3
141738227:     	movups	%xmm0, 0x70(%rax)
14173822b:     	movups	%xmm1, 0x80(%rax)
141738232:     	movups	%xmm2, 0x90(%rax)
141738239:     	movups	%xmm3, 0xa0(%rax)
141738240:     	movups	0x138(%rbp), %xmm0
141738247:     	movups	%xmm0, 0xb0(%rax)
14173824e:     	movups	0x148(%rbp), %xmm0
141738255:     	movups	%xmm0, 0xc0(%rax)
14173825c:     	movups	0x158(%rbp), %xmm0
141738263:     	movups	%xmm0, 0xd0(%rax)
14173826a:     	movups	0xe8(%rbp), %xmm0
141738271:     	movups	%xmm0, 0x140(%rax)
141738278:     	movups	0xd8(%rbp), %xmm0
14173827f:     	movups	%xmm0, 0x130(%rax)
141738286:     	movups	0xc8(%rbp), %xmm0
14173828d:     	movups	%xmm0, 0x120(%rax)
141738294:     	movups	0x88(%rbp), %xmm0
14173829b:     	movups	0x98(%rbp), %xmm1
1417382a2:     	movups	0xa8(%rbp), %xmm2
1417382a9:     	movups	0xb8(%rbp), %xmm3
1417382b0:     	movups	%xmm3, 0x110(%rax)
1417382b7:     	movups	%xmm2, 0x100(%rax)
1417382be:     	movups	%xmm1, 0xf0(%rax)
1417382c5:     	movups	%xmm0, 0xe0(%rax)
1417382cc:     	movups	0x18(%rbp), %xmm0
1417382d0:     	movups	0x28(%rbp), %xmm1
1417382d4:     	movups	0x38(%rbp), %xmm2
1417382d8:     	movups	0x48(%rbp), %xmm3
1417382dc:     	movups	%xmm0, 0x150(%rax)
1417382e3:     	movups	%xmm1, 0x160(%rax)
1417382ea:     	movups	%xmm2, 0x170(%rax)
1417382f1:     	movups	%xmm3, 0x180(%rax)
1417382f8:     	movups	0x58(%rbp), %xmm0
1417382fc:     	movups	%xmm0, 0x190(%rax)
141738303:     	movups	0x68(%rbp), %xmm0
141738307:     	movups	%xmm0, 0x1a0(%rax)
14173830e:     	movups	0x78(%rbp), %xmm0
141738312:     	movups	%xmm0, 0x1b0(%rax)
141738319:     	movups	-0x58(%rbp), %xmm0
14173831d:     	movups	-0x48(%rbp), %xmm1
141738321:     	movups	-0x38(%rbp), %xmm2
141738325:     	movups	-0x28(%rbp), %xmm3
141738329:     	movups	%xmm0, 0x1c0(%rax)
141738330:     	movups	%xmm1, 0x1d0(%rax)
141738337:     	movups	%xmm2, 0x1e0(%rax)
14173833e:     	movups	%xmm3, 0x1f0(%rax)
141738345:     	movups	-0x18(%rbp), %xmm0
141738349:     	movups	%xmm0, 0x200(%rax)
141738350:     	movups	0x8(%rbp), %xmm0
141738354:     	movups	%xmm0, 0x220(%rax)
14173835b:     	movups	-0x8(%rbp), %xmm0
14173835f:     	movups	%xmm0, 0x210(%rax)
141738366:     	movups	0x238(%rbp), %xmm0
14173836d:     	movups	%xmm0, 0x290(%rax)
141738374:     	movups	0x228(%rbp), %xmm0
14173837b:     	movups	%xmm0, 0x280(%rax)
141738382:     	movups	0x218(%rbp), %xmm0
141738389:     	movups	%xmm0, 0x270(%rax)
141738390:     	movups	0x1d8(%rbp), %xmm0
141738397:     	movups	0x1e8(%rbp), %xmm1
14173839e:     	movups	0x1f8(%rbp), %xmm2
1417383a5:     	movups	0x208(%rbp), %xmm3
1417383ac:     	movups	%xmm3, 0x260(%rax)
1417383b3:     	movups	%xmm2, 0x250(%rax)
1417383ba:     	movups	%xmm0, 0x230(%rax)
1417383c1:     	movups	%xmm1, 0x240(%rax)
1417383c8:     	movq	$0x6, 0x1e0(%rbp)
1417383d3:     	movq	%rax, 0x1e8(%rbp)
1417383da:     	movq	$0x6, 0x1f0(%rbp)
1417383e5:     	movq	$0x0, 0x1f8(%rbp)
1417383f0:     	movq	$0x8, 0x200(%rbp)
1417383fb:     	xorps	%xmm0, %xmm0
1417383fe:     	movups	%xmm0, 0x208(%rbp)
141738405:     	movq	%rbx, 0x1d8(%rbp)
14173840c:     	leaq	0x210(%rbp), %rcx
141738413:     	callq	0x1416a00a0 <.text+0x169f0a0>
141738418:     	movups	0x238(%rbp), %xmm0
14173841f:     	movups	%xmm0, 0x60(%rsi)
141738423:     	movups	0x228(%rbp), %xmm0
14173842a:     	movups	%xmm0, 0x50(%rsi)
14173842e:     	movups	0x218(%rbp), %xmm0
141738435:     	movups	%xmm0, 0x40(%rsi)
141738439:     	movups	0x1d8(%rbp), %xmm0
141738440:     	movups	0x1e8(%rbp), %xmm1
141738447:     	movups	0x1f8(%rbp), %xmm2
14173844e:     	movups	0x208(%rbp), %xmm3
141738455:     	movups	%xmm3, 0x30(%rsi)
141738459:     	movups	%xmm2, 0x20(%rsi)
14173845d:     	movups	%xmm1, 0x10(%rsi)
141738461:     	movups	%xmm0, (%rsi)
141738464:     	movq	%rsi, %rax
141738467:     	addq	$0x428, %rsp            # imm = 0x428
14173846e:     	popq	%rbx
14173846f:     	popq	%rdi
141738470:     	popq	%rsi
141738471:     	popq	%rbp
141738472:     	retq
141738473:     	movl	$0x8, %ecx
141738478:     	movl	$0x2a0, %edx            # imm = 0x2A0
14173847d:     	callq	0x1428d9000 <.text+0x28d8000>
141738482:     	movl	$0x8, %ecx
141738487:     	movl	$0x1c0, %edx            # imm = 0x1C0
14173848c:     	callq	0x1428d9000 <.text+0x28d8000>
141738491:     	ud2
