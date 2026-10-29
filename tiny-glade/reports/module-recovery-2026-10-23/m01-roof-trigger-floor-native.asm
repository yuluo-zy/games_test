
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140dce1d0:     	pushq	%r15
140dce1d2:     	pushq	%r14
140dce1d4:     	pushq	%r13
140dce1d6:     	pushq	%r12
140dce1d8:     	pushq	%rsi
140dce1d9:     	pushq	%rdi
140dce1da:     	pushq	%rbx
140dce1db:     	subq	$0x20, %rsp
140dce1df:     	movq	%rdx, %r14
140dce1e2:     	movq	(%rdx), %r13
140dce1e5:     	movq	0x8(%rdx), %r9
140dce1e9:     	movq	%r9, %rbx
140dce1ec:     	subq	%r13, %rbx
140dce1ef:     	movabsq	$-0x1c71c71c71c71c71, %rdx # imm = 0xE38E38E38E38E38F
140dce1f9:     	movq	%rbx, %rax
140dce1fc:     	mulq	%rdx
140dce1ff:     	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
140dce209:     	cmpq	%rax, %rbx
140dce20c:     	ja	0x140dce360 <.text+0xdcd360>
140dce212:     	movq	%rcx, %rsi
140dce215:     	cmpq	%r13, %r9
140dce218:     	je	0x140dce276 <.text+0xdcd276>
140dce21a:     	movq	%rdx, %rdi
140dce21d:     	movq	%r8, %r15
140dce220:     	movzbl	0x28669e2(%rip), %eax   # 0x143634c09
140dce227:     	movl	$0x4, %r12d
140dce22d:     	movl	$0x4, %edx
140dce232:     	movq	%rbx, %rcx
140dce235:     	callq	0x140613c10 <.text+0x612c10>
140dce23a:     	testq	%rax, %rax
140dce23d:     	je	0x140dce366 <.text+0xdcd366>
140dce243:     	shrq	$0x5, %rdi
140dce247:     	movss	0x10(%r14), %xmm1
140dce24d:     	xorps	%xmm0, %xmm0
140dce250:     	mulss	%xmm1, %xmm0
140dce254:     	movaps	%xmm0, %xmm2
140dce257:     	unpcklps	%xmm1, %xmm2            # xmm2 = xmm2[0],xmm1[0],xmm2[1],xmm1[1]
140dce25a:     	shufps	$0x4, %xmm2, %xmm2      # xmm2 = xmm2[0,1,0,0]
140dce25e:     	unpcklps	%xmm0, %xmm1            # xmm1 = xmm1[0],xmm0[0],xmm1[1],xmm0[1]
140dce261:     	shufps	$0x14, %xmm1, %xmm1     # xmm1 = xmm1[0,1,1,0]
140dce265:     	addq	$-0x24, %rbx
140dce269:     	cmpq	$0x24, %rbx
140dce26d:     	jae	0x140dce282 <.text+0xdcd282>
140dce26f:     	xorl	%ecx, %ecx
140dce271:     	jmp	0x140dce30c <.text+0xdcd30c>
140dce276:     	movl	$0x4, %eax
140dce27b:     	xorl	%edi, %edi
140dce27d:     	jmp	0x140dce342 <.text+0xdcd342>
140dce282:     	movabsq	$0x3fffffffffffffe, %rdx # imm = 0x3FFFFFFFFFFFFFE
140dce28c:     	andq	%rdi, %rdx
140dce28f:     	movl	$0x44, %r8d
140dce295:     	xorl	%ecx, %ecx
140dce297:     	nopw	(%rax,%rax)
140dce2a0:     	movss	-0x24(%r13,%r8), %xmm3
140dce2a7:     	addss	%xmm0, %xmm3
140dce2ab:     	movups	-0x44(%r13,%r8), %xmm4
140dce2b1:     	addps	%xmm2, %xmm4
140dce2b4:     	movups	%xmm4, -0x44(%rax,%r8)
140dce2ba:     	movups	-0x34(%r13,%r8), %xmm4
140dce2c0:     	addps	%xmm1, %xmm4
140dce2c3:     	movups	%xmm4, -0x34(%rax,%r8)
140dce2c9:     	movss	%xmm3, -0x24(%rax,%r8)
140dce2d0:     	movss	(%r13,%r8), %xmm3
140dce2d7:     	addss	%xmm0, %xmm3
140dce2db:     	movups	-0x20(%r13,%r8), %xmm4
140dce2e1:     	addps	%xmm2, %xmm4
140dce2e4:     	movups	%xmm4, -0x20(%rax,%r8)
140dce2ea:     	movups	-0x10(%r13,%r8), %xmm4
140dce2f0:     	addps	%xmm1, %xmm4
140dce2f3:     	movups	%xmm4, -0x10(%rax,%r8)
140dce2f9:     	movss	%xmm3, (%rax,%r8)
140dce2ff:     	addq	$0x2, %rcx
140dce303:     	addq	$0x48, %r8
140dce307:     	cmpq	%rcx, %rdx
140dce30a:     	jne	0x140dce2a0 <.text+0xdcd2a0>
140dce30c:     	testb	$0x1, %dil
140dce310:     	je	0x140dce342 <.text+0xdcd342>
140dce312:     	shlq	$0x2, %rcx
140dce316:     	leaq	(%rcx,%rcx,8), %rcx
140dce31a:     	addss	0x20(%r13,%rcx), %xmm0
140dce321:     	movups	(%r13,%rcx), %xmm3
140dce327:     	addps	%xmm3, %xmm2
140dce32a:     	movups	%xmm2, (%rax,%rcx)
140dce32e:     	movups	0x10(%r13,%rcx), %xmm2
140dce334:     	addps	%xmm2, %xmm1
140dce337:     	movups	%xmm1, 0x10(%rax,%rcx)
140dce33c:     	movss	%xmm0, 0x20(%rax,%rcx)
140dce342:     	movq	%rdi, (%rsi)
140dce345:     	movq	%rax, 0x8(%rsi)
140dce349:     	movq	%rdi, 0x10(%rsi)
140dce34d:     	movq	%rsi, %rax
140dce350:     	addq	$0x20, %rsp
140dce354:     	popq	%rbx
140dce355:     	popq	%rdi
140dce356:     	popq	%rsi
140dce357:     	popq	%r12
140dce359:     	popq	%r13
140dce35b:     	popq	%r14
140dce35d:     	popq	%r15
140dce35f:     	retq
140dce360:     	movq	%r8, %r15
140dce363:     	xorl	%r12d, %r12d
140dce366:     	movq	%r12, %rcx
140dce369:     	movq	%rbx, %rdx
140dce36c:     	movq	%r15, %r8
140dce36f:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140dce374:     	int3
140dce375:     	int3
140dce376:     	int3
140dce377:     	int3
140dce378:     	int3
140dce379:     	int3
140dce37a:     	int3
140dce37b:     	int3
140dce37c:     	int3
140dce37d:     	int3
140dce37e:     	int3
140dce37f:     	int3
