
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1400f50e0:     	pushq	%rbp
1400f50e1:     	pushq	%r14
1400f50e3:     	pushq	%rsi
1400f50e4:     	pushq	%rdi
1400f50e5:     	pushq	%rbx
1400f50e6:     	subq	$0x310, %rsp            # imm = 0x310
1400f50ed:     	leaq	0x80(%rsp), %rbp
1400f50f5:     	movq	$-0x2, 0x288(%rbp)
1400f5100:     	movl	%edx, %ebx
1400f5102:     	movq	%rcx, %rsi
1400f5105:     	movzbl	0x353fafd(%rip), %eax   # 0x143634c09
1400f510c:     	movl	$0xe0, %ecx
1400f5111:     	movl	$0x8, %edx
1400f5116:     	callq	0x140613c10 <.text+0x612c10>
1400f511b:     	testq	%rax, %rax
1400f511e:     	je	0x1400f53bc <.text+0xf43bc>
1400f5124:     	movq	%rax, 0x280(%rbp)
1400f512b:     	xorps	%xmm0, %xmm0
1400f512e:     	movups	%xmm0, -0x40(%rbp)
1400f5132:     	movups	%xmm0, -0x28(%rbp)
1400f5136:     	movups	%xmm0, -0x10(%rbp)
1400f513a:     	movups	%xmm0, 0x8(%rbp)
1400f513e:     	movups	%xmm0, 0x20(%rbp)
1400f5142:     	movl	$0x0, 0x30(%rbp)
1400f5149:     	movups	%xmm0, 0x58(%rbp)
1400f514d:     	movups	%xmm0, 0x70(%rbp)
1400f5151:     	movups	%xmm0, 0x88(%rbp)
1400f5158:     	movups	%xmm0, 0xa0(%rbp)
1400f515f:     	movups	%xmm0, 0xb8(%rbp)
1400f5166:     	movl	$0x0, 0xc8(%rbp)
1400f5170:     	movq	$0x0, -0x60(%rbp)
1400f5178:     	movq	$0x8, -0x58(%rbp)
1400f5180:     	movq	$0x0, -0x50(%rbp)
1400f5188:     	movq	$0x10, -0x48(%rbp)
1400f5190:     	movq	$0x10, -0x30(%rbp)
1400f5198:     	movq	$0x10, -0x18(%rbp)
1400f51a0:     	movq	$0x10, (%rbp)
1400f51a8:     	movq	$0x10, 0x18(%rbp)
1400f51b0:     	movabsq	$-0x8000000000000000, %r14 # imm = 0x8000000000000000
1400f51ba:     	movq	%r14, 0x38(%rbp)
1400f51be:     	leaq	0x28563b5(%rip), %rax   # 0x14294b57a
1400f51c5:     	movq	%rax, 0x40(%rbp)
1400f51c9:     	movq	$0x50, 0x48(%rbp)
1400f51d1:     	movq	$0x10, 0x50(%rbp)
1400f51d9:     	movq	$0x10, 0x68(%rbp)
1400f51e1:     	movq	$0x10, 0x80(%rbp)
1400f51ec:     	movq	$0x10, 0x98(%rbp)
1400f51f7:     	movq	$0x10, 0xb0(%rbp)
1400f5202:     	movl	$0x0, 0xd0(%rbp)
1400f520c:     	movw	$0x1, 0xd4(%rbp)
1400f5215:     	movq	%r14, 0x1e0(%rbp)
1400f521c:     	movl	$0x0, 0x208(%rbp)
1400f5226:     	movzbl	0x353f9dc(%rip), %eax   # 0x143634c09
1400f522d:     	movl	$0x270, %ecx            # imm = 0x270
1400f5232:     	movl	$0x8, %edx
1400f5237:     	callq	0x140613c10 <.text+0x612c10>
1400f523c:     	testq	%rax, %rax
1400f523f:     	je	0x1400f53cb <.text+0xf43cb>
1400f5245:     	movq	%rax, %rdi
1400f5248:     	leaq	-0x60(%rbp), %rdx
1400f524c:     	movl	$0x270, %r8d            # imm = 0x270
1400f5252:     	movq	%rax, %rcx
1400f5255:     	callq	0x142923c00 <.text+0x2922c00>
1400f525a:     	leaq	0x2855def(%rip), %r8    # 0x14294b050
1400f5261:     	leaq	0x210(%rbp), %rcx
1400f5268:     	movq	%rdi, %rdx
1400f526b:     	callq	0x140ce4b10 <.text+0xce3b10>
1400f5270:     	leaq	-0x60(%rbp), %rcx
1400f5274:     	callq	0x140369210 <.text+0x368210>
1400f5279:     	movups	0x270(%rbp), %xmm0
1400f5280:     	movq	0x280(%rbp), %rax
1400f5287:     	movups	%xmm0, 0x60(%rax)
1400f528b:     	movups	0x260(%rbp), %xmm0
1400f5292:     	movups	%xmm0, 0x50(%rax)
1400f5296:     	movups	0x250(%rbp), %xmm0
1400f529d:     	movups	%xmm0, 0x40(%rax)
1400f52a1:     	movups	0x210(%rbp), %xmm0
1400f52a8:     	movups	0x220(%rbp), %xmm1
1400f52af:     	movups	0x230(%rbp), %xmm2
1400f52b6:     	movups	0x240(%rbp), %xmm3
1400f52bd:     	movups	%xmm3, 0x30(%rax)
1400f52c1:     	movups	%xmm2, 0x20(%rax)
1400f52c5:     	movups	%xmm1, 0x10(%rax)
1400f52c9:     	movups	%xmm0, (%rax)
1400f52cc:     	movups	-0x60(%rbp), %xmm0
1400f52d0:     	movups	-0x50(%rbp), %xmm1
1400f52d4:     	movups	-0x40(%rbp), %xmm2
1400f52d8:     	movups	-0x30(%rbp), %xmm3
1400f52dc:     	movups	%xmm0, 0x70(%rax)
1400f52e0:     	movups	%xmm1, 0x80(%rax)
1400f52e7:     	movups	%xmm2, 0x90(%rax)
1400f52ee:     	movups	%xmm3, 0xa0(%rax)
1400f52f5:     	movups	-0x20(%rbp), %xmm0
1400f52f9:     	movups	%xmm0, 0xb0(%rax)
1400f5300:     	movups	-0x10(%rbp), %xmm0
1400f5304:     	movups	%xmm0, 0xc0(%rax)
1400f530b:     	movups	(%rbp), %xmm0
1400f530f:     	movups	%xmm0, 0xd0(%rax)
1400f5316:     	movq	$0x2, -0x58(%rbp)
1400f531e:     	movq	%rax, -0x50(%rbp)
1400f5322:     	movq	$0x2, -0x48(%rbp)
1400f532a:     	movq	$0x0, -0x40(%rbp)
1400f5332:     	movq	$0x8, -0x38(%rbp)
1400f533a:     	xorps	%xmm0, %xmm0
1400f533d:     	movups	%xmm0, -0x30(%rbp)
1400f5341:     	movq	%r14, -0x60(%rbp)
1400f5345:     	movb	%bl, 0x210(%rbp)
1400f534b:     	movq	0x353701e(%rip), %rcx   # 0x14362c370
1400f5352:     	leaq	0x2835c3f(%rip), %r8    # 0x14292af98
1400f5359:     	leaq	0x210(%rbp), %rdx
1400f5360:     	callq	0x140364d20 <.text+0x363d20>
1400f5365:     	movq	%rdx, %r8
1400f5368:     	leaq	-0x60(%rbp), %rcx
1400f536c:     	movq	%rax, %rdx
1400f536f:     	callq	0x1400d68c0 <.text+0xd58c0>
1400f5374:     	movups	(%rbp), %xmm0
1400f5378:     	movups	%xmm0, 0x60(%rsi)
1400f537c:     	movups	-0x10(%rbp), %xmm0
1400f5380:     	movups	%xmm0, 0x50(%rsi)
1400f5384:     	movups	-0x20(%rbp), %xmm0
1400f5388:     	movups	%xmm0, 0x40(%rsi)
1400f538c:     	movups	-0x60(%rbp), %xmm0
1400f5390:     	movups	-0x50(%rbp), %xmm1
1400f5394:     	movups	-0x40(%rbp), %xmm2
1400f5398:     	movups	-0x30(%rbp), %xmm3
1400f539c:     	movups	%xmm3, 0x30(%rsi)
1400f53a0:     	movups	%xmm2, 0x20(%rsi)
1400f53a4:     	movups	%xmm1, 0x10(%rsi)
1400f53a8:     	movups	%xmm0, (%rsi)
1400f53ab:     	movq	%rsi, %rax
1400f53ae:     	addq	$0x310, %rsp            # imm = 0x310
1400f53b5:     	popq	%rbx
1400f53b6:     	popq	%rdi
1400f53b7:     	popq	%rsi
1400f53b8:     	popq	%r14
1400f53ba:     	popq	%rbp
1400f53bb:     	retq
1400f53bc:     	movl	$0x8, %ecx
1400f53c1:     	movl	$0xe0, %edx
1400f53c6:     	callq	0x1428d9000 <.text+0x28d8000>
1400f53cb:     	movl	$0x8, %ecx
1400f53d0:     	movl	$0x270, %edx            # imm = 0x270
1400f53d5:     	callq	0x1428d9000 <.text+0x28d8000>
1400f53da:     	ud2
