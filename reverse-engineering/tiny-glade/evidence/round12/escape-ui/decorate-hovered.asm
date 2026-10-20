
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14067df20:     	pushq	%r15
14067df22:     	pushq	%r14
14067df24:     	pushq	%r13
14067df26:     	pushq	%r12
14067df28:     	pushq	%rsi
14067df29:     	pushq	%rdi
14067df2a:     	pushq	%rbp
14067df2b:     	pushq	%rbx
14067df2c:     	subq	$0x68, %rsp
14067df30:     	movq	(%rdx), %rbx
14067df33:     	cmpq	$0x0, 0x1b8(%rbx)
14067df3b:     	je	0x14067e1b5 <.text+0x67d1b5>
14067df41:     	movq	%rdx, %rdi
14067df44:     	movq	%rcx, %rsi
14067df47:     	addq	$0x8, %rdx
14067df4b:     	leaq	0x1c0(%rbx), %rcx
14067df52:     	callq	0x1405b0d70 <.text+0x5afd70>
14067df57:     	movq	0x1a0(%rbx), %rcx
14067df5e:     	movq	0x1a8(%rbx), %rdx
14067df65:     	movq	%rdx, %r8
14067df68:     	andq	%rax, %r8
14067df6b:     	shrq	$0x39, %rax
14067df6f:     	movd	%eax, %xmm0
14067df73:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
14067df77:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14067df7c:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14067df81:     	leaq	-0xc0(%rcx), %rax
14067df88:     	movq	0x8(%rdi), %r12
14067df8c:     	xorl	%r9d, %r9d
14067df8f:     	pcmpeqd	%xmm1, %xmm1
14067df93:     	movdqu	(%rcx,%r8), %xmm2
14067df99:     	movdqa	%xmm2, %xmm3
14067df9d:     	pcmpeqb	%xmm0, %xmm3
14067dfa1:     	pmovmskb	%xmm3, %r10d
14067dfa6:     	testl	%r10d, %r10d
14067dfa9:     	je	0x14067dfe0 <.text+0x67cfe0>
14067dfab:     	tzcntl	%r10d, %r11d
14067dfb0:     	addq	%r8, %r11
14067dfb3:     	andq	%rdx, %r11
14067dfb6:     	negq	%r11
14067dfb9:     	leaq	(%r11,%r11,2), %r11
14067dfbd:     	shlq	$0x6, %r11
14067dfc1:     	cmpq	(%rax,%r11), %r12
14067dfc5:     	je	0x14067e002 <.text+0x67d002>
14067dfc7:     	leal	-0x1(%r10), %r11d
14067dfcb:     	andw	%r10w, %r11w
14067dfcf:     	movl	%r11d, %r10d
14067dfd2:     	jne	0x14067dfab <.text+0x67cfab>
14067dfd4:     	nopw	%cs:(%rax,%rax)
14067dfe0:     	pcmpeqb	%xmm1, %xmm2
14067dfe4:     	pmovmskb	%xmm2, %r10d
14067dfe9:     	testl	%r10d, %r10d
14067dfec:     	jne	0x14067e1b5 <.text+0x67d1b5>
14067dff2:     	addq	%r9, %r8
14067dff5:     	addq	$0x10, %r8
14067dff9:     	addq	$0x10, %r9
14067dffd:     	andq	%rdx, %r8
14067e000:     	jmp	0x14067df93 <.text+0x67cf93>
14067e002:     	testb	$0x1, -0x2(%rcx,%r11)
14067e008:     	je	0x14067e363 <.text+0x67d363>
14067e00e:     	movq	%r12, 0x38(%rsp)
14067e013:     	movl	$0xab23d50d, 0x40(%rsp) # imm = 0xAB23D50D
14067e01b:     	leaq	0x38(%rsp), %rcx
14067e020:     	callq	0x140fabc80 <.text+0xfaac80>
14067e025:     	movq	%rax, %rbx
14067e028:     	movq	(%rdi), %r14
14067e02b:     	movabsq	$-0x4e96acdfffb04bbd, %r13 # imm = 0xB1695320004FB443
14067e035:     	movq	%rax, %r15
14067e038:     	xorq	%r13, %r15
14067e03b:     	movq	%r14, %rcx
14067e03e:     	movq	%r15, %rdx
14067e041:     	callq	0x140683ec0 <.text+0x682ec0>
14067e046:     	movb	$0x0, 0x5d(%rax)
14067e04a:     	testb	$0x1, %dl
14067e04d:     	jne	0x14067e0a2 <.text+0x67d0a2>
14067e04f:     	movq	0x10(%r14), %rbp
14067e053:     	cmpq	(%r14), %rbp
14067e056:     	jne	0x14067e067 <.text+0x67d067>
14067e058:     	leaq	0x2530f69(%rip), %rdx   # 0x142baefc8
14067e05f:     	movq	%r14, %rcx
14067e062:     	callq	0x1406319d0 <.text+0x6309d0>
14067e067:     	movq	0x8(%r14), %rax
14067e06b:     	imulq	$0x38, %rbp, %rcx
14067e06f:     	movq	$0x3, (%rax,%rcx)
14067e077:     	movq	%r12, 0x10(%rax,%rcx)
14067e07c:     	movaps	0x2398d8d(%rip), %xmm0  # 0x142a16e10
14067e083:     	movups	%xmm0, 0x18(%rax,%rcx)
14067e088:     	movd	0x22a7874(%rip), %xmm0  # 0x142925904
14067e090:     	movq	%xmm0, 0x28(%rax,%rcx)
14067e096:     	movq	%r15, 0x30(%rax,%rcx)
14067e09b:     	incq	%rbp
14067e09e:     	movq	%rbp, 0x10(%r14)
14067e0a2:     	movq	%r14, 0x38(%rsp)
14067e0a7:     	movq	%r15, 0x40(%rsp)
14067e0ac:     	movq	$0x0, 0x48(%rsp)
14067e0b5:     	leaq	0x2398f0d(%rip), %rax   # 0x142a16fc9
14067e0bc:     	movq	%rax, 0x20(%rsp)
14067e0c1:     	movl	$0x42000000, 0x30(%rsp) # imm = 0x42000000
14067e0c9:     	movq	$0x17, 0x28(%rsp)
14067e0d2:     	leaq	0x2398ee7(%rip), %r8    # 0x142a16fc0
14067e0d9:     	leaq	0x50(%rsp), %rcx
14067e0de:     	leaq	0x38(%rsp), %rdx
14067e0e3:     	movl	$0x9, %r9d
14067e0e9:     	callq	0x140f9fca0 <.text+0xf9eca0>
14067e0ee:     	movq	0x50(%rsp), %r14
14067e0f3:     	cmpq	$0x0, 0x1b8(%r14)
14067e0fb:     	je	0x14067e34f <.text+0x67d34f>
14067e101:     	incq	%rbx
14067e104:     	leaq	0x58(%rsp), %rdx
14067e109:     	leaq	0x1c0(%r14), %rcx
14067e110:     	callq	0x1405b0d70 <.text+0x5afd70>
14067e115:     	movq	0x1a0(%r14), %rcx
14067e11c:     	movq	0x1a8(%r14), %rdx
14067e123:     	movq	%rdx, %r8
14067e126:     	andq	%rax, %r8
14067e129:     	shrq	$0x39, %rax
14067e12d:     	movd	%eax, %xmm0
14067e131:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
14067e135:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14067e13a:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14067e13f:     	leaq	-0xc0(%rcx), %rax
14067e146:     	xorl	%r9d, %r9d
14067e149:     	movq	0x58(%rsp), %r10
14067e14e:     	pcmpeqd	%xmm1, %xmm1
14067e152:     	movdqu	(%rcx,%r8), %xmm2
14067e158:     	movdqa	%xmm2, %xmm3
14067e15c:     	pcmpeqb	%xmm0, %xmm3
14067e160:     	pmovmskb	%xmm3, %r11d
14067e165:     	testl	%r11d, %r11d
14067e168:     	je	0x14067e193 <.text+0x67d193>
14067e16a:     	tzcntl	%r11d, %r14d
14067e16f:     	addq	%r8, %r14
14067e172:     	andq	%rdx, %r14
14067e175:     	negq	%r14
14067e178:     	leaq	(%r14,%r14,2), %r14
14067e17c:     	shlq	$0x6, %r14
14067e180:     	cmpq	(%rax,%r14), %r10
14067e184:     	je	0x14067e1cd <.text+0x67d1cd>
14067e186:     	leal	-0x1(%r11), %ebp
14067e18a:     	andw	%r11w, %bp
14067e18e:     	movl	%ebp, %r11d
14067e191:     	jne	0x14067e16a <.text+0x67d16a>
14067e193:     	pcmpeqb	%xmm1, %xmm2
14067e197:     	pmovmskb	%xmm2, %r11d
14067e19c:     	testl	%r11d, %r11d
14067e19f:     	jne	0x14067e34f <.text+0x67d34f>
14067e1a5:     	addq	%r9, %r8
14067e1a8:     	addq	$0x10, %r8
14067e1ac:     	addq	$0x10, %r9
14067e1b0:     	andq	%rdx, %r8
14067e1b3:     	jmp	0x14067e152 <.text+0x67d152>
14067e1b5:     	leaq	0x239904c(%rip), %rcx   # 0x142a17208
14067e1bc:     	leaq	0x2399e15(%rip), %r8    # 0x142a17fd8
14067e1c3:     	movl	$0x16, %edx
14067e1c8:     	callq	0x1428d9330 <.text+0x28d8330>
14067e1cd:     	movl	$0x42c80000, -0x10(%rcx,%r14) # imm = 0x42C80000
14067e1d6:     	movq	(%rdi), %r14
14067e1d9:     	xorq	%r13, %rbx
14067e1dc:     	movq	%r14, %rcx
14067e1df:     	movq	%rbx, %rdx
14067e1e2:     	callq	0x140683ec0 <.text+0x682ec0>
14067e1e7:     	movb	$0x0, 0x5d(%rax)
14067e1eb:     	testb	$0x1, %dl
14067e1ee:     	jne	0x14067e243 <.text+0x67d243>
14067e1f0:     	movq	0x10(%r14), %r15
14067e1f4:     	cmpq	(%r14), %r15
14067e1f7:     	jne	0x14067e208 <.text+0x67d208>
14067e1f9:     	leaq	0x2530dc8(%rip), %rdx   # 0x142baefc8
14067e200:     	movq	%r14, %rcx
14067e203:     	callq	0x1406319d0 <.text+0x6309d0>
14067e208:     	movq	0x8(%r14), %rax
14067e20c:     	imulq	$0x38, %r15, %rcx
14067e210:     	movq	$0x3, (%rax,%rcx)
14067e218:     	movq	%r12, 0x10(%rax,%rcx)
14067e21d:     	movaps	0x2398bfc(%rip), %xmm0  # 0x142a16e20
14067e224:     	movups	%xmm0, 0x18(%rax,%rcx)
14067e229:     	movq	0x2398bff(%rip), %xmm0  # 0x142a16e30
14067e231:     	movq	%xmm0, 0x28(%rax,%rcx)
14067e237:     	movq	%rbx, 0x30(%rax,%rcx)
14067e23c:     	incq	%r15
14067e23f:     	movq	%r15, 0x10(%r14)
14067e243:     	movq	%r14, 0x38(%rsp)
14067e248:     	movq	%rbx, 0x40(%rsp)
14067e24d:     	movq	$0x0, 0x48(%rsp)
14067e256:     	leaq	0x2398d83(%rip), %rax   # 0x142a16fe0
14067e25d:     	movq	%rax, 0x20(%rsp)
14067e262:     	movl	$0x42000000, 0x30(%rsp) # imm = 0x42000000
14067e26a:     	movq	$0x18, 0x28(%rsp)
14067e273:     	leaq	0x2398d46(%rip), %r8    # 0x142a16fc0
14067e27a:     	leaq	0x50(%rsp), %rcx
14067e27f:     	leaq	0x38(%rsp), %rdx
14067e284:     	movl	$0x9, %r9d
14067e28a:     	callq	0x140f9fca0 <.text+0xf9eca0>
14067e28f:     	movq	0x50(%rsp), %rbx
14067e294:     	cmpq	$0x0, 0x1b8(%rbx)
14067e29c:     	je	0x14067e34f <.text+0x67d34f>
14067e2a2:     	leaq	0x58(%rsp), %rdx
14067e2a7:     	leaq	0x1c0(%rbx), %rcx
14067e2ae:     	callq	0x1405b0d70 <.text+0x5afd70>
14067e2b3:     	movq	0x1a0(%rbx), %rcx
14067e2ba:     	movq	0x1a8(%rbx), %rdx
14067e2c1:     	movq	%rdx, %r8
14067e2c4:     	andq	%rax, %r8
14067e2c7:     	shrq	$0x39, %rax
14067e2cb:     	movd	%eax, %xmm0
14067e2cf:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
14067e2d3:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14067e2d8:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14067e2dd:     	leaq	-0xc0(%rcx), %rax
14067e2e4:     	xorl	%r9d, %r9d
14067e2e7:     	movq	0x58(%rsp), %r10
14067e2ec:     	pcmpeqd	%xmm1, %xmm1
14067e2f0:     	movdqu	(%rcx,%r8), %xmm2
14067e2f6:     	movdqa	%xmm2, %xmm3
14067e2fa:     	pcmpeqb	%xmm0, %xmm3
14067e2fe:     	pmovmskb	%xmm3, %r11d
14067e303:     	testl	%r11d, %r11d
14067e306:     	je	0x14067e331 <.text+0x67d331>
14067e308:     	tzcntl	%r11d, %ebx
14067e30d:     	addq	%r8, %rbx
14067e310:     	andq	%rdx, %rbx
14067e313:     	negq	%rbx
14067e316:     	leaq	(%rbx,%rbx,2), %rbx
14067e31a:     	shlq	$0x6, %rbx
14067e31e:     	cmpq	(%rax,%rbx), %r10
14067e322:     	je	0x14067e35b <.text+0x67d35b>
14067e324:     	leal	-0x1(%r11), %ebx
14067e328:     	andw	%r11w, %bx
14067e32c:     	movl	%ebx, %r11d
14067e32f:     	jne	0x14067e308 <.text+0x67d308>
14067e331:     	pcmpeqb	%xmm1, %xmm2
14067e335:     	pmovmskb	%xmm2, %r11d
14067e33a:     	testl	%r11d, %r11d
14067e33d:     	jne	0x14067e34f <.text+0x67d34f>
14067e33f:     	addq	%r9, %r8
14067e342:     	addq	$0x10, %r8
14067e346:     	addq	$0x10, %r9
14067e34a:     	andq	%rdx, %r8
14067e34d:     	jmp	0x14067e2f0 <.text+0x67d2f0>
14067e34f:     	leaq	0x2399c9a(%rip), %rcx   # 0x142a17ff0
14067e356:     	callq	0x1428d9310 <.text+0x28d8310>
14067e35b:     	movl	$0x42c80000, -0x10(%rcx,%rbx) # imm = 0x42C80000
14067e363:     	movq	0x10(%rdi), %rax
14067e367:     	movq	%rax, 0x10(%rsi)
14067e36b:     	movups	(%rdi), %xmm0
14067e36e:     	movups	%xmm0, (%rsi)
14067e371:     	movq	%rsi, %rax
14067e374:     	addq	$0x68, %rsp
14067e378:     	popq	%rbx
14067e379:     	popq	%rbp
14067e37a:     	popq	%rdi
14067e37b:     	popq	%rsi
14067e37c:     	popq	%r12
14067e37e:     	popq	%r13
14067e380:     	popq	%r14
14067e382:     	popq	%r15
14067e384:     	retq
