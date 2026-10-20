
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a26240:     	pushq	%rbp
140a26241:     	pushq	%rsi
140a26242:     	pushq	%rdi
140a26243:     	subq	$0x2c0, %rsp            # imm = 0x2C0
140a2624a:     	leaq	0x80(%rsp), %rbp
140a26252:     	movq	$-0x2, 0x238(%rbp)
140a2625d:     	movq	%rcx, %rsi
140a26260:     	xorps	%xmm0, %xmm0
140a26263:     	movups	%xmm0, -0x40(%rbp)
140a26267:     	movups	%xmm0, -0x28(%rbp)
140a2626b:     	movups	%xmm0, -0x10(%rbp)
140a2626f:     	movups	%xmm0, 0x8(%rbp)
140a26273:     	movups	%xmm0, 0x20(%rbp)
140a26277:     	movl	$0x0, 0x30(%rbp)
140a2627e:     	movups	%xmm0, 0x58(%rbp)
140a26282:     	movups	%xmm0, 0x70(%rbp)
140a26286:     	movups	%xmm0, 0x88(%rbp)
140a2628d:     	movups	%xmm0, 0xa0(%rbp)
140a26294:     	movups	%xmm0, 0xb8(%rbp)
140a2629b:     	movl	$0x0, 0xc8(%rbp)
140a262a5:     	movq	$0x0, -0x60(%rbp)
140a262ad:     	movq	$0x8, -0x58(%rbp)
140a262b5:     	movq	$0x0, -0x50(%rbp)
140a262bd:     	movq	$0x10, -0x48(%rbp)
140a262c5:     	movq	$0x10, -0x30(%rbp)
140a262cd:     	movq	$0x10, -0x18(%rbp)
140a262d5:     	movq	$0x10, (%rbp)
140a262dd:     	movq	$0x10, 0x18(%rbp)
140a262e5:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140a262ef:     	movq	%rax, 0x38(%rbp)
140a262f3:     	leaq	0x20a449e(%rip), %rcx   # 0x142aca798
140a262fa:     	movq	%rcx, 0x40(%rbp)
140a262fe:     	movq	$0x50, 0x48(%rbp)
140a26306:     	movq	$0x10, 0x50(%rbp)
140a2630e:     	movq	$0x10, 0x68(%rbp)
140a26316:     	movq	$0x10, 0x80(%rbp)
140a26321:     	movq	$0x10, 0x98(%rbp)
140a2632c:     	movq	$0x10, 0xb0(%rbp)
140a26337:     	movl	$0x0, 0xd0(%rbp)
140a26341:     	movw	$0x1, 0xd4(%rbp)
140a2634a:     	movq	%rax, 0x1d0(%rbp)
140a26351:     	movl	$0x0, 0x230(%rbp)
140a2635b:     	movzbl	0x2c0e8a7(%rip), %eax   # 0x143634c09
140a26362:     	movl	$0x298, %ecx            # imm = 0x298
140a26367:     	movl	$0x8, %edx
140a2636c:     	callq	0x140613c10 <.text+0x612c10>
140a26371:     	testq	%rax, %rax
140a26374:     	je	0x140a263ab <.text+0xa253ab>
140a26376:     	movq	%rax, %rdi
140a26379:     	leaq	-0x60(%rbp), %rdx
140a2637d:     	movl	$0x298, %r8d            # imm = 0x298
140a26383:     	movq	%rax, %rcx
140a26386:     	callq	0x142923c00 <.text+0x2922c00>
140a2638b:     	leaq	0x20a3fd6(%rip), %r8    # 0x142aca368
140a26392:     	movq	%rsi, %rcx
140a26395:     	movq	%rdi, %rdx
140a26398:     	callq	0x140ce4b10 <.text+0xce3b10>
140a2639d:     	movq	%rsi, %rax
140a263a0:     	addq	$0x2c0, %rsp            # imm = 0x2C0
140a263a7:     	popq	%rdi
140a263a8:     	popq	%rsi
140a263a9:     	popq	%rbp
140a263aa:     	retq
140a263ab:     	movl	$0x8, %ecx
140a263b0:     	movl	$0x298, %edx            # imm = 0x298
140a263b5:     	callq	0x1428d9000 <.text+0x28d8000>
140a263ba:     	ud2
