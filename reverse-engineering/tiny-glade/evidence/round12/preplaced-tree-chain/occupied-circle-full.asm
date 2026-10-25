
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1425defd0:     	pushq	%r15
1425defd2:     	pushq	%r14
1425defd4:     	pushq	%r13
1425defd6:     	pushq	%r12
1425defd8:     	pushq	%rsi
1425defd9:     	pushq	%rdi
1425defda:     	pushq	%rbp
1425defdb:     	pushq	%rbx
1425defdc:     	subq	$0xd8, %rsp
1425defe3:     	movaps	%xmm10, 0xc0(%rsp)
1425defec:     	movaps	%xmm9, 0xb0(%rsp)
1425deff5:     	movaps	%xmm8, 0xa0(%rsp)
1425deffe:     	movaps	%xmm7, 0x90(%rsp)
1425df006:     	movaps	%xmm6, 0x80(%rsp)
1425df00e:     	movq	%r8, %rsi
1425df011:     	movq	%rdx, %rdi
1425df014:     	movq	%rcx, %rbx
1425df017:     	leaq	0x38(%rsp), %rcx
1425df01c:     	movq	%rbx, %rdx
1425df01f:     	callq	0x140aa6600 <.text+0xaa5600>
1425df024:     	movsd	0x38(%rsp), %xmm10
1425df02b:     	movsd	0x40(%rsp), %xmm9
1425df032:     	leaq	0x70(%rsp), %rcx
1425df037:     	movq	%rdi, 0x68(%rsp)
1425df03c:     	movq	%rdi, %rdx
1425df03f:     	callq	0x140c98140 <.text+0xc97140>
1425df044:     	movq	%rbx, %rcx
1425df047:     	callq	0x140aa65e0 <.text+0xaa55e0>
1425df04c:     	movaps	%xmm0, %xmm6
1425df04f:     	movsd	0x70(%rsp), %xmm8
1425df056:     	movsd	0x78(%rsp), %xmm7
1425df05c:     	movaps	%xmm10, %xmm0
1425df060:     	maxps	%xmm8, %xmm0
1425df064:     	cmpunordps	%xmm8, %xmm8
1425df069:     	andps	%xmm8, %xmm10
1425df06d:     	andnps	%xmm0, %xmm8
1425df071:     	orps	%xmm10, %xmm8
1425df075:     	leaq	0x38(%rsp), %rcx
1425df07a:     	movq	%rbx, %rdx
1425df07d:     	callq	0x140aa6600 <.text+0xaa5600>
1425df082:     	movsd	0x38(%rsp), %xmm0
1425df088:     	cmpleps	%xmm8, %xmm0
1425df08d:     	unpcklps	%xmm0, %xmm0            # xmm0 = xmm0[0,0,1,1]
1425df090:     	movmskpd	%xmm0, %eax
1425df094:     	cmpl	$0x3, %eax
1425df097:     	jne	0x1425df2b8 <.text+0x25de2b8>
1425df09d:     	movaps	%xmm9, %xmm0
1425df0a1:     	minps	%xmm7, %xmm0
1425df0a4:     	cmpunordps	%xmm7, %xmm7
1425df0a8:     	andps	%xmm7, %xmm9
1425df0ac:     	andnps	%xmm0, %xmm7
1425df0af:     	orps	%xmm9, %xmm7
1425df0b3:     	movsd	0x40(%rsp), %xmm0
1425df0b9:     	movaps	%xmm7, %xmm1
1425df0bc:     	cmpleps	%xmm0, %xmm1
1425df0c0:     	unpcklps	%xmm1, %xmm1            # xmm1 = xmm1[0,0,1,1]
1425df0c3:     	movmskpd	%xmm1, %eax
1425df0c7:     	cmpl	$0x3, %eax
1425df0ca:     	jne	0x1425df2d0 <.text+0x25de2d0>
1425df0d0:     	movq	%rbx, %rcx
1425df0d3:     	callq	0x140aa65e0 <.text+0xaa55e0>
1425df0d8:     	ucomiss	0x346791(%rip), %xmm0   # 0x142925870
1425df0df:     	jbe	0x1425df2e8 <.text+0x25de2e8>
1425df0e5:     	movq	%rsi, 0x50(%rsp)
1425df0ea:     	movshdup	%xmm8, %xmm2            # xmm2 = xmm8[1,1,3,3]
1425df0ef:     	movq	%rbx, %rcx
1425df0f2:     	movaps	%xmm8, %xmm1
1425df0f6:     	callq	0x140aa6520 <.text+0xaa5520>
1425df0fb:     	movl	%eax, %r13d
1425df0fe:     	xorl	%r12d, %r12d
1425df101:     	testl	%eax, %eax
1425df103:     	cmovlel	%r12d, %r13d
1425df107:     	testl	%edx, %edx
1425df109:     	cmovgl	%edx, %r12d
1425df10d:     	xorps	%xmm0, %xmm0
1425df110:     	movshdup	%xmm7, %xmm2            # xmm2 = xmm7[1,1,3,3]
1425df114:     	addss	%xmm0, %xmm7
1425df118:     	addss	%xmm0, %xmm2
1425df11c:     	movq	%rbx, %rcx
1425df11f:     	movaps	%xmm7, %xmm1
1425df122:     	callq	0x140aa6520 <.text+0xaa5520>
1425df127:     	movl	%eax, %r14d
1425df12a:     	movq	0x48(%rbx), %r15
1425df12e:     	leal	-0x1(%r15), %eax
1425df132:     	movq	%r15, %r8
1425df135:     	shrq	$0x20, %r8
1425df139:     	decl	%r8d
1425df13c:     	cmpl	%r14d, %eax
1425df13f:     	cmovll	%eax, %r14d
1425df143:     	cmpl	%edx, %r8d
1425df146:     	cmovgel	%edx, %r8d
1425df14a:     	cmpl	%r8d, %r12d
1425df14d:     	jg	0x1425df279 <.text+0x25de279>
1425df153:     	cmpl	%r14d, %r13d
1425df156:     	jle	0x1425df17d <.text+0x25de17d>
1425df158:     	nopl	(%rax,%rax)
1425df160:     	xorl	%eax, %eax
1425df162:     	cmpl	%r8d, %r12d
1425df165:     	setl	%cl
1425df168:     	jge	0x1425df279 <.text+0x25de279>
1425df16e:     	movb	%cl, %al
1425df170:     	addl	%eax, %r12d
1425df173:     	cmpl	%r8d, %r12d
1425df176:     	jle	0x1425df160 <.text+0x25de160>
1425df178:     	jmp	0x1425df279 <.text+0x25de279>
1425df17d:     	mulss	0x3466eb(%rip), %xmm6   # 0x142925870
1425df185:     	mulss	0x34f567(%rip), %xmm6   # 0x14292e6f4
1425df18d:     	movq	0x8(%rbx), %rax
1425df191:     	movq	%rax, 0x58(%rsp)
1425df196:     	movq	0x10(%rbx), %rax
1425df19a:     	movq	%rax, 0x30(%rsp)
1425df19f:     	movq	%rbx, 0x60(%rsp)
1425df1a4:     	movl	%r13d, 0x2c(%rsp)
1425df1a9:     	movq	%r8, 0x48(%rsp)
1425df1ae:     	movq	0x60(%rsp), %rbp
1425df1b3:     	movl	%r12d, %r15d
1425df1b6:     	xorl	%r12d, %r12d
1425df1b9:     	cmpl	%r8d, %r15d
1425df1bc:     	setl	%r12b
1425df1c0:     	addl	%r15d, %r12d
1425df1c3:     	nopw	%cs:(%rax,%rax)
1425df1d0:     	xorl	%edi, %edi
1425df1d2:     	cmpl	%r14d, %r13d
1425df1d5:     	setl	%bl
1425df1d8:     	movq	%rbp, %rcx
1425df1db:     	movl	%r13d, %edx
1425df1de:     	movl	%r15d, %r8d
1425df1e1:     	callq	0x140aa65d0 <.text+0xaa55d0>
1425df1e6:     	movq	%rax, %rsi
1425df1e9:     	movq	%rbp, %rcx
1425df1ec:     	movq	%rax, %rdx
1425df1ef:     	callq	0x140aa5c70 <.text+0xaa4c70>
1425df1f4:     	movq	0x68(%rsp), %rax
1425df1f9:     	movsd	(%rax), %xmm2
1425df1fd:     	unpcklps	%xmm1, %xmm0            # xmm0 = xmm0[0],xmm1[0],xmm0[1],xmm1[1]
1425df200:     	subps	%xmm2, %xmm0
1425df203:     	mulps	%xmm0, %xmm0
1425df206:     	movshdup	%xmm0, %xmm1            # xmm1 = xmm0[1,1,3,3]
1425df20a:     	addss	%xmm0, %xmm1
1425df20e:     	xorps	%xmm0, %xmm0
1425df211:     	sqrtss	%xmm1, %xmm0
1425df215:     	movss	0x8(%rax), %xmm1
1425df21a:     	addss	%xmm6, %xmm1
1425df21e:     	ucomiss	%xmm0, %xmm1
1425df221:     	jbe	0x1425df243 <.text+0x25de243>
1425df223:     	cmpq	%rsi, 0x30(%rsp)
1425df228:     	jbe	0x1425df300 <.text+0x25de300>
1425df22e:     	leaq	(%rsi,%rsi,2), %rax
1425df232:     	shlq	$0x4, %rax
1425df236:     	movq	0x58(%rsp), %rcx
1425df23b:     	cmpq	$0x0, 0x10(%rcx,%rax)
1425df241:     	jne	0x1425df271 <.text+0x25de271>
1425df243:     	cmpl	%r14d, %r13d
1425df246:     	jge	0x1425df257 <.text+0x25de257>
1425df248:     	movb	%bl, %dil
1425df24b:     	addl	%edi, %r13d
1425df24e:     	cmpl	%r14d, %r13d
1425df251:     	jle	0x1425df1d0 <.text+0x25de1d0>
1425df257:     	movq	0x48(%rsp), %r8
1425df25c:     	cmpl	%r8d, %r15d
1425df25f:     	movl	0x2c(%rsp), %r13d
1425df264:     	jge	0x1425df279 <.text+0x25de279>
1425df266:     	cmpl	%r8d, %r12d
1425df269:     	jle	0x1425df1b3 <.text+0x25de1b3>
1425df26f:     	jmp	0x1425df279 <.text+0x25de279>
1425df271:     	movq	0x50(%rsp), %rax
1425df276:     	movb	$0x0, (%rax)
1425df279:     	movaps	0x80(%rsp), %xmm6
1425df281:     	movaps	0x90(%rsp), %xmm7
1425df289:     	movaps	0xa0(%rsp), %xmm8
1425df292:     	movaps	0xb0(%rsp), %xmm9
1425df29b:     	movaps	0xc0(%rsp), %xmm10
1425df2a4:     	addq	$0xd8, %rsp
1425df2ab:     	popq	%rbx
1425df2ac:     	popq	%rbp
1425df2ad:     	popq	%rdi
1425df2ae:     	popq	%rsi
1425df2af:     	popq	%r12
1425df2b1:     	popq	%r13
1425df2b3:     	popq	%r14
1425df2b5:     	popq	%r15
1425df2b7:     	retq
1425df2b8:     	leaq	0xb55c29(%rip), %rcx    # 0x143134ee8
1425df2bf:     	leaq	0xb55cda(%rip), %r8     # 0x143134fa0
1425df2c6:     	movl	$0x44, %edx
1425df2cb:     	callq	0x1428d9430 <.text+0x28d8430>
1425df2d0:     	leaq	0xb55ce1(%rip), %rcx    # 0x143134fb8
1425df2d7:     	leaq	0xb55d22(%rip), %r8     # 0x143135000
1425df2de:     	movl	$0x44, %edx
1425df2e3:     	callq	0x1428d9430 <.text+0x28d8430>
1425df2e8:     	leaq	0xb55d29(%rip), %rcx    # 0x143135018
1425df2ef:     	leaq	0xb55d5a(%rip), %r8     # 0x143135050
1425df2f6:     	movl	$0x31, %edx
1425df2fb:     	callq	0x1428d9430 <.text+0x28d8430>
1425df300:     	leaq	0xb55d61(%rip), %r8     # 0x143135068
1425df307:     	movq	%rsi, %rcx
1425df30a:     	movq	0x30(%rsp), %rdx
1425df30f:     	callq	0x1428d9518 <.text+0x28d8518>
1425df314:     	int3
