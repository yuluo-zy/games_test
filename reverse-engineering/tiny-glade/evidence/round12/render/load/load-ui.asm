
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141e1fe80:     	pushq	%rbp
141e1fe81:     	pushq	%r15
141e1fe83:     	pushq	%r14
141e1fe85:     	pushq	%r13
141e1fe87:     	pushq	%r12
141e1fe89:     	pushq	%rsi
141e1fe8a:     	pushq	%rdi
141e1fe8b:     	pushq	%rbx
141e1fe8c:     	movl	$0x27f8, %eax           # imm = 0x27F8
141e1fe91:     	callq	0x14289aa40 <.text+0x2899a40>
141e1fe96:     	subq	%rax, %rsp
141e1fe99:     	leaq	0x80(%rsp), %rbp
141e1fea1:     	movaps	%xmm8, 0x2760(%rbp)
141e1fea9:     	movdqa	%xmm7, 0x2750(%rbp)
141e1feb1:     	movaps	%xmm6, 0x2740(%rbp)
141e1feb8:     	movq	$-0x2, 0x2738(%rbp)
141e1fec3:     	movq	%r9, %r15
141e1fec6:     	movq	%r8, 0x2608(%rbp)
141e1fecd:     	movq	%rdx, %rsi
141e1fed0:     	movq	%rcx, %r14
141e1fed3:     	movq	0x2808(%rbp), %rax
141e1feda:     	movq	%rax, 0x26f8(%rbp)
141e1fee1:     	callq	0x140c16a00 <.text+0xc15a00>
141e1fee6:     	movl	%eax, %ebx
141e1fee8:     	testb	%al, %al
141e1feea:     	movq	%rsi, 0x24c8(%rbp)
141e1fef1:     	je	0x141e1ff50 <.text+0x1e1ef50>
141e1fef3:     	movl	$0x17, %r12d
141e1fef9:     	movl	$0x17, %ecx
141e1fefe:     	callq	0x141e1eee0 <.text+0x1e1dee0>
141e1ff03:     	leaq	0x116818e(%rip), %r13   # 0x142f88098
141e1ff0a:     	cmpq	$0x1, %rax
141e1ff0e:     	jne	0x141e1ffbc <.text+0x1e1efbc>
141e1ff14:     	movq	%rdx, %r9
141e1ff17:     	testq	%rdx, %rdx
141e1ff1a:     	je	0x141e1ff59 <.text+0x1e1ef59>
141e1ff1c:     	cmpq	$0x17, %r9
141e1ff20:     	jae	0x141e1ff57 <.text+0x1e1ef57>
141e1ff22:     	leaq	0x116816f(%rip), %rax   # 0x142f88098
141e1ff29:     	cmpb	$-0x41, (%r9,%rax)
141e1ff2e:     	jg	0x141e1ff59 <.text+0x1e1ef59>
141e1ff30:     	leaq	0x1168201(%rip), %rax   # 0x142f88138
141e1ff37:     	movq	%rax, 0x20(%rsp)
141e1ff3c:     	leaq	0x1168155(%rip), %rcx   # 0x142f88098
141e1ff43:     	movl	$0x17, %edx
141e1ff48:     	xorl	%r8d, %r8d
141e1ff4b:     	callq	0x1428d9940 <.text+0x28d8940>
141e1ff50:     	xorl	%eax, %eax
141e1ff52:     	jmp	0x141e20114 <.text+0x1e1f114>
141e1ff57:     	jne	0x141e1ff30 <.text+0x1e1ef30>
141e1ff59:     	movq	%r9, %rcx
141e1ff5c:     	callq	0x141e1eee0 <.text+0x1e1dee0>
141e1ff61:     	cmpq	$0x1, %rax
141e1ff65:     	jne	0x141e1ffbc <.text+0x1e1efbc>
141e1ff67:     	movq	%rdx, %r8
141e1ff6a:     	addq	$0x2, %r8
141e1ff6e:     	je	0x141e1ffa9 <.text+0x1e1efa9>
141e1ff70:     	cmpq	$0x17, %r8
141e1ff74:     	jae	0x141e1ffa7 <.text+0x1e1efa7>
141e1ff76:     	leaq	0x116811b(%rip), %rax   # 0x142f88098
141e1ff7d:     	cmpb	$-0x41, (%r8,%rax)
141e1ff82:     	jg	0x141e1ffa9 <.text+0x1e1efa9>
141e1ff84:     	leaq	0x11681c5(%rip), %rax   # 0x142f88150
141e1ff8b:     	movq	%rax, 0x20(%rsp)
141e1ff90:     	leaq	0x1168101(%rip), %rcx   # 0x142f88098
141e1ff97:     	movl	$0x17, %edx
141e1ff9c:     	movl	$0x17, %r9d
141e1ffa2:     	callq	0x1428d9940 <.text+0x28d8940>
141e1ffa7:     	jne	0x141e1ff84 <.text+0x1e1ef84>
141e1ffa9:     	movl	$0x15, %r12d
141e1ffaf:     	subq	%rdx, %r12
141e1ffb2:     	leaq	0x11680df(%rip), %r13   # 0x142f88098
141e1ffb9:     	addq	%r8, %r13
141e1ffbc:     	leaq	0x11681e4(%rip), %r8    # 0x142f881a7
141e1ffc3:     	leaq	0x11681b6(%rip), %rdi   # 0x142f88180
141e1ffca:     	nopw	(%rax,%rax)
141e1ffd0:     	movsbl	-0x1(%r8), %eax
141e1ffd5:     	testl	%eax, %eax
141e1ffd7:     	js	0x141e1fff0 <.text+0x1e1eff0>
141e1ffd9:     	decq	%r8
141e1ffdc:     	cmpl	$0x5c, %eax
141e1ffdf:     	jne	0x141e20044 <.text+0x1e1f044>
141e1ffe1:     	jmp	0x141e20055 <.text+0x1e1f055>
141e1ffe3:     	nopw	%cs:(%rax,%rax)
141e1fff0:     	movzbl	-0x2(%r8), %ecx
141e1fff5:     	cmpb	$-0x40, %cl
141e1fff8:     	jge	0x141e2001d <.text+0x1e1f01d>
141e1fffa:     	movzbl	-0x3(%r8), %edx
141e1ffff:     	cmpb	$-0x40, %dl
141e20002:     	jge	0x141e20026 <.text+0x1e1f026>
141e20004:     	movzbl	-0x4(%r8), %r9d
141e20009:     	addq	$-0x4, %r8
141e2000d:     	andl	$0x7, %r9d
141e20011:     	shll	$0x6, %r9d
141e20015:     	andl	$0x3f, %edx
141e20018:     	orl	%r9d, %edx
141e2001b:     	jmp	0x141e2002d <.text+0x1e1f02d>
141e2001d:     	addq	$-0x2, %r8
141e20021:     	andl	$0x1f, %ecx
141e20024:     	jmp	0x141e20035 <.text+0x1e1f035>
141e20026:     	addq	$-0x3, %r8
141e2002a:     	andl	$0xf, %edx
141e2002d:     	shll	$0x6, %edx
141e20030:     	andl	$0x3f, %ecx
141e20033:     	orl	%edx, %ecx
141e20035:     	shll	$0x6, %ecx
141e20038:     	andb	$0x3f, %al
141e2003a:     	movzbl	%al, %eax
141e2003d:     	orl	%ecx, %eax
141e2003f:     	cmpl	$0x5c, %eax
141e20042:     	je	0x141e20055 <.text+0x1e1f055>
141e20044:     	cmpl	$0x2f, %eax
141e20047:     	je	0x141e20055 <.text+0x1e1f055>
141e20049:     	cmpq	%rdi, %r8
141e2004c:     	jne	0x141e1ffd0 <.text+0x1e1efd0>
141e2004e:     	movl	$0x27, %esi
141e20053:     	jmp	0x141e200a1 <.text+0x1e1f0a1>
141e20055:     	leaq	0x1168124(%rip), %rdi   # 0x142f88180
141e2005c:     	subq	%rdi, %r8
141e2005f:     	incq	%r8
141e20062:     	je	0x141e20096 <.text+0x1e1f096>
141e20064:     	cmpq	$0x27, %r8
141e20068:     	jae	0x141e20094 <.text+0x1e1f094>
141e2006a:     	cmpb	$-0x41, (%r8,%rdi)
141e2006f:     	jg	0x141e20096 <.text+0x1e1f096>
141e20071:     	leaq	0x11680a0(%rip), %rax   # 0x142f88118
141e20078:     	movq	%rax, 0x20(%rsp)
141e2007d:     	leaq	0x11680fc(%rip), %rcx   # 0x142f88180
141e20084:     	movl	$0x27, %edx
141e20089:     	movl	$0x27, %r9d
141e2008f:     	callq	0x1428d9940 <.text+0x28d8940>
141e20094:     	jne	0x141e20071 <.text+0x1e1f071>
141e20096:     	movl	$0x27, %esi
141e2009b:     	subq	%r8, %rsi
141e2009e:     	addq	%r8, %rdi
141e200a1:     	callq	0x140c18130 <.text+0xc17130>
141e200a6:     	movq	%rax, %rcx
141e200a9:     	movq	(%rax), %rax
141e200ac:     	cmpq	$0x1, %rax
141e200b0:     	jne	0x141e2060b <.text+0x1e1f60b>
141e200b6:     	addq	$0x8, %rcx
141e200ba:     	cmpq	$0x0, (%rcx)
141e200be:     	jne	0x141e22e53 <.text+0x1e21e53>
141e200c4:     	movq	$-0x1, (%rcx)
141e200cb:     	movq	%rcx, 0x2728(%rbp)
141e200d2:     	addq	$0x8, %rcx
141e200d6:     	movq	%rsi, 0x20(%rsp)
141e200db:     	movq	$0x0, 0x30(%rsp)
141e200e4:     	movq	$0x1, 0x28(%rsp)
141e200ed:     	movq	%r13, %rdx
141e200f0:     	movq	%r12, %r8
141e200f3:     	movq	%rdi, %r9
141e200f6:     	callq	0x140c16d40 <.text+0xc15d40>
141e200fb:     	movq	%rax, %rdi
141e200fe:     	movq	0x2728(%rbp), %rax
141e20105:     	incq	(%rax)
141e20108:     	movq	%rdi, 0x25a8(%rbp)
141e2010f:     	movl	$0x1, %eax
141e20114:     	movq	%rax, 0x25a0(%rbp)
141e2011b:     	movl	0x1c(%r15), %eax
141e2011f:     	movq	0x10(%r15), %rcx
141e20123:     	movl	%eax, (%rcx)
141e20125:     	movq	(%r15), %rdx
141e20128:     	movq	(%rdx), %rax
141e2012b:     	addq	$-0x2, %rax
141e2012f:     	cmpq	$0x9, %rax
141e20133:     	setae	%cl
141e20136:     	cmpq	$0x4, %rax
141e2013a:     	sete	%al
141e2013d:     	orb	%cl, %al
141e2013f:     	je	0x141e20368 <.text+0x1e1f368>
141e20145:     	movq	%rdx, 0x2700(%rbp)
141e2014c:     	movq	0x27e8(%rbp), %r13
141e20153:     	movl	0x1c(%r13), %eax
141e20157:     	movq	0x10(%r13), %rcx
141e2015b:     	movl	%eax, 0x2714(%rbp)
141e20161:     	movq	%rcx, 0x26e8(%rbp)
141e20168:     	movl	%eax, (%rcx)
141e2016a:     	movq	(%r13), %rax
141e2016e:     	movq	%rax, 0x2728(%rbp)
141e20175:     	leaq	0x116805c(%rip), %rax   # 0x142f881d8
141e2017c:     	movq	%rax, 0x700(%rbp)
141e20183:     	movq	$0xb, 0x708(%rbp)
141e2018e:     	leaq	0x700(%rbp), %rcx
141e20195:     	callq	0x1405bdda0 <.text+0x5bcda0>
141e2019a:     	movq	%rax, %rsi
141e2019d:     	movabsq	$-0x4e96acdfffb04bbd, %rax # imm = 0xB1695320004FB443
141e201a7:     	xorq	%rax, %rsi
141e201aa:     	movq	0x2728(%rbp), %rcx
141e201b1:     	movq	%rsi, %rdx
141e201b4:     	callq	0x140683ec0 <.text+0x682ec0>
141e201b9:     	movb	$0x0, 0x5d(%rax)
141e201bd:     	movb	$0x1, 0xb7(%rax)
141e201c4:     	testb	$0x1, %dl
141e201c7:     	jne	0x141e2020e <.text+0x1e1f20e>
141e201c9:     	movq	0x2728(%rbp), %rax
141e201d0:     	movq	0x10(%rax), %rdi
141e201d4:     	cmpq	(%rax), %rdi
141e201d7:     	jne	0x141e201ec <.text+0x1e1f1ec>
141e201d9:     	leaq	0xd8ede8(%rip), %rdx    # 0x142baefc8
141e201e0:     	movq	0x2728(%rbp), %rcx
141e201e7:     	callq	0x1406319d0 <.text+0x6309d0>
141e201ec:     	movq	0x2728(%rbp), %rdx
141e201f3:     	movq	0x8(%rdx), %rax
141e201f7:     	imulq	$0x38, %rdi, %rcx
141e201fb:     	movl	$0x0, (%rax,%rcx)
141e20202:     	movq	%rsi, 0x30(%rax,%rcx)
141e20207:     	incq	%rdi
141e2020a:     	movq	%rdi, 0x10(%rdx)
141e2020e:     	movq	0x2800(%rbp), %rax
141e20215:     	movq	0x2728(%rbp), %rcx
141e2021c:     	movq	%rcx, 0x2480(%rbp)
141e20223:     	movq	%rsi, 0x2488(%rbp)
141e2022a:     	movq	$0x0, 0x2490(%rbp)
141e20235:     	movq	(%rax), %rax
141e20238:     	movb	$0x1, 0x2734(%rbp)
141e2023f:     	movq	0x2700(%rbp), %rcx
141e20246:     	cmpb	$0x0, 0x38(%rcx)
141e2024a:     	jne	0x141e2025c <.text+0x1e1f25c>
141e2024c:     	cmpb	$0x0, 0x49(%rcx)
141e20250:     	jne	0x141e2025c <.text+0x1e1f25c>
141e20252:     	movzbl	0x10(%rcx), %edx
141e20256:     	movb	%dl, 0x2734(%rbp)
141e2025c:     	movq	0x8(%rax), %r12
141e20260:     	movq	0x10(%rax), %rbx
141e20264:     	movq	0x30(%rcx), %r9
141e20268:     	lock
141e20269:     	incq	(%r9)
141e2026c:     	jle	0x141e22ef7 <.text+0x1e21ef7>
141e20272:     	leaq	0x1(%rbx), %r8
141e20276:     	leaq	0x1167f66(%rip), %rax   # 0x142f881e3
141e2027d:     	movq	%rax, 0x20(%rsp)
141e20282:     	movq	$0x6, 0x28(%rsp)
141e2028b:     	leaq	0x700(%rbp), %rcx
141e20292:     	leaq	0x2480(%rbp), %rdx
141e20299:     	callq	0x140fa06a0 <.text+0xf9f6a0>
141e2029e:     	movq	0x700(%rbp), %rsi
141e202a5:     	cmpq	$0x0, 0x1b8(%rsi)
141e202ad:     	je	0x141e205fa <.text+0x1e1f5fa>
141e202b3:     	leaq	0x708(%rbp), %rdx
141e202ba:     	leaq	0x1c0(%rsi), %rcx
141e202c1:     	callq	0x1405b0d70 <.text+0x5afd70>
141e202c6:     	movq	0x1a0(%rsi), %rcx
141e202cd:     	movq	0x1a8(%rsi), %rdx
141e202d4:     	movq	%rdx, %r8
141e202d7:     	andq	%rax, %r8
141e202da:     	shrq	$0x39, %rax
141e202de:     	movd	%eax, %xmm0
141e202e2:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
141e202e6:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
141e202eb:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
141e202f0:     	leaq	-0xc0(%rcx), %rax
141e202f7:     	movq	0x708(%rbp), %r9
141e202fe:     	xorl	%r10d, %r10d
141e20301:     	pcmpeqd	%xmm1, %xmm1
141e20305:     	movdqu	(%rcx,%r8), %xmm2
141e2030b:     	movdqa	%xmm2, %xmm3
141e2030f:     	pcmpeqb	%xmm0, %xmm3
141e20313:     	pmovmskb	%xmm3, %r11d
141e20318:     	testl	%r11d, %r11d
141e2031b:     	je	0x141e20346 <.text+0x1e1f346>
141e2031d:     	tzcntl	%r11d, %esi
141e20322:     	addq	%r8, %rsi
141e20325:     	andq	%rdx, %rsi
141e20328:     	negq	%rsi
141e2032b:     	leaq	(%rsi,%rsi,2), %rsi
141e2032f:     	shlq	$0x6, %rsi
141e20333:     	cmpq	(%rax,%rsi), %r9
141e20337:     	je	0x141e203b2 <.text+0x1e1f3b2>
141e20339:     	leal	-0x1(%r11), %esi
141e2033d:     	andw	%r11w, %si
141e20341:     	movl	%esi, %r11d
141e20344:     	jne	0x141e2031d <.text+0x1e1f31d>
141e20346:     	pcmpeqb	%xmm1, %xmm2
141e2034a:     	pmovmskb	%xmm2, %r11d
141e2034f:     	testl	%r11d, %r11d
141e20352:     	jne	0x141e205fa <.text+0x1e1f5fa>
141e20358:     	addq	%r10, %r8
141e2035b:     	addq	$0x10, %r8
141e2035f:     	addq	$0x10, %r10
141e20363:     	andq	%rdx, %r8
141e20366:     	jmp	0x141e20305 <.text+0x1e1f305>
141e20368:     	testb	%bl, %bl
141e2036a:     	je	0x141e22dec <.text+0x1e21dec>
141e20370:     	callq	0x140c18130 <.text+0xc17130>
141e20375:     	movq	%rax, %rcx
141e20378:     	movq	(%rax), %rax
141e2037b:     	cmpq	$0x1, %rax
141e2037f:     	jne	0x141e2062c <.text+0x1e1f62c>
141e20385:     	addq	$0x8, %rcx
141e20389:     	cmpq	$0x0, (%rcx)
141e2038d:     	jne	0x141e22e53 <.text+0x1e21e53>
141e20393:     	movq	$-0x1, (%rcx)
141e2039a:     	movq	%rcx, 0x2728(%rbp)
141e203a1:     	addq	$0x8, %rcx
141e203a5:     	movq	%rdi, %rdx
141e203a8:     	callq	0x140c170f0 <.text+0xc160f0>
141e203ad:     	jmp	0x141e22de2 <.text+0x1e21de2>
141e203b2:     	testb	$0x1, 0x2734(%rbp)
141e203b9:     	jne	0x141e203c0 <.text+0x1e1f3c0>
141e203bb:     	orb	$0x4, -0x1(%rcx,%rsi)
141e203c0:     	movq	0x710(%rbp), %rax
141e203c7:     	movq	%rax, 0x1cf0(%rbp)
141e203ce:     	movups	0x700(%rbp), %xmm0
141e203d5:     	movaps	%xmm0, 0x1ce0(%rbp)
141e203dc:     	leaq	0x1ce0(%rbp), %rcx
141e203e3:     	movb	$0x1, %dl
141e203e5:     	callq	0x140f9d590 <.text+0xf9c590>
141e203ea:     	cmpb	$0x0, 0x75(%rax)
141e203ee:     	jne	0x141e20407 <.text+0x1e1f407>
141e203f0:     	movss	0x54(%rax), %xmm0
141e203f5:     	divss	0x50(%rax), %xmm0
141e203fa:     	addss	0xb05502(%rip), %xmm0   # 0x142925904
141e20402:     	movss	%xmm0, 0x5c(%rax)
141e20407:     	movq	0x1cf0(%rbp), %rax
141e2040e:     	movq	%rax, 0x710(%rbp)
141e20415:     	movaps	0x1ce0(%rbp), %xmm0
141e2041c:     	movaps	%xmm0, 0x700(%rbp)
141e20423:     	leaq	0x700(%rbp), %rcx
141e2042a:     	movb	$0x1, %dl
141e2042c:     	callq	0x140f9d590 <.text+0xf9c590>
141e20431:     	movabsq	$0x3e851eb800000001, %rcx # imm = 0x3E851EB800000001
141e2043b:     	movq	%rcx, 0x30(%rax)
141e2043f:     	movq	0x710(%rbp), %rax
141e20446:     	movq	%rax, 0x1cf0(%rbp)
141e2044d:     	movaps	0x700(%rbp), %xmm0
141e20454:     	movaps	%xmm0, 0x1ce0(%rbp)
141e2045b:     	leaq	0x1ce0(%rbp), %rcx
141e20462:     	movb	$0x1, %dl
141e20464:     	callq	0x140f9d590 <.text+0xf9c590>
141e20469:     	movl	$0x41200000, 0x70(%rax) # imm = 0x41200000
141e20470:     	movq	0x1cf0(%rbp), %rax
141e20477:     	movq	%rax, 0x710(%rbp)
141e2047e:     	movaps	0x1ce0(%rbp), %xmm0
141e20485:     	movaps	%xmm0, 0x700(%rbp)
141e2048c:     	leaq	0x700(%rbp), %rcx
141e20493:     	movb	$0x1, %dl
141e20495:     	callq	0x140f9d590 <.text+0xf9c590>
141e2049a:     	movzbl	0x2734(%rbp), %ecx
141e204a1:     	movb	%cl, 0x76(%rax)
141e204a4:     	movq	0x710(%rbp), %rax
141e204ab:     	movq	%rax, 0x24e0(%rbp)
141e204b2:     	movdqa	0x700(%rbp), %xmm0
141e204ba:     	movdqa	%xmm0, 0x24d0(%rbp)
141e204c2:     	leaq	0x24d0(%rbp), %rcx
141e204c9:     	callq	0x140f9c4f0 <.text+0xf9b4f0>
141e204ce:     	movq	0x2700(%rbp), %rdi
141e204d5:     	cmpb	$0x1, 0x38(%rdi)
141e204d9:     	je	0x141e211e5 <.text+0x1e201e5>
141e204df:     	cmpb	$0x0, 0x49(%rdi)
141e204e3:     	jne	0x141e211e5 <.text+0x1e201e5>
141e204e9:     	movzbl	0x77(%rax), %eax
141e204ed:     	movb	%al, 0x2670(%rbp)
141e204f3:     	movq	(%rdi), %rax
141e204f6:     	movq	%rax, 0x2708(%rbp)
141e204fd:     	movq	0x8(%rdi), %rax
141e20501:     	movq	%rax, 0x2718(%rbp)
141e20508:     	leaq	0x24d0(%rbp), %rcx
141e2050f:     	callq	0x140f9c4f0 <.text+0xf9b4f0>
141e20514:     	movq	0x8(%rax), %rcx
141e20518:     	cmpq	%rbx, %rcx
141e2051b:     	setbe	%dl
141e2051e:     	andb	(%rax), %dl
141e20520:     	movzbl	%dl, %eax
141e20523:     	movq	0x2700(%rbp), %rdx
141e2052a:     	movq	%rax, (%rdx)
141e2052d:     	movq	%rcx, 0x8(%rdx)
141e20531:     	leaq	0x24d0(%rbp), %rcx
141e20538:     	callq	0x140f9c4f0 <.text+0xf9b4f0>
141e2053d:     	movq	0x18(%rax), %rcx
141e20541:     	cmpq	%rbx, %rcx
141e20544:     	setbe	%dl
141e20547:     	andb	0x10(%rax), %dl
141e2054a:     	cmpb	$0x1, %dl
141e2054d:     	jne	0x141e206c7 <.text+0x1e1f6c7>
141e20553:     	testq	%rcx, %rcx
141e20556:     	je	0x141e2064d <.text+0x1e1f64d>
141e2055c:     	leaq	-0x1(%rcx), %rax
141e20560:     	cmpq	$0x3, %rax
141e20564:     	jb	0x141e20572 <.text+0x1e1f572>
141e20566:     	movq	0x180bb2b(%rip), %rdx   # 0x14362c098
141e2056d:     	cmpb	$0x0, (%rdx)
141e20570:     	jne	0x141e205b0 <.text+0x1e1f5b0>
141e20572:     	leaq	(%rax,%rax,8), %rax
141e20576:     	leaq	(%rax,%rax,2), %rax
141e2057a:     	leaq	(%rcx,%rax), %r15
141e2057e:     	decq	%r15
141e20581:     	addq	%r12, %r15
141e20584:     	movq	0x2608(%rbp), %rax
141e2058b:     	movq	(%rax), %rdx
141e2058e:     	movq	%r15, %rcx
141e20591:     	callq	0x140a9fa80 <.text+0xa9ea80>
141e20596:     	movb	%al, 0x700(%rbp)
141e2059c:     	leaq	0x700(%rbp), %rcx
141e205a3:     	callq	0x140a9fa70 <.text+0xa9ea70>
141e205a8:     	testb	%al, %al
141e205aa:     	je	0x141e20ac7 <.text+0x1e1fac7>
141e205b0:     	movq	0x2810(%rbp), %rdx
141e205b7:     	movl	0x1c(%rdx), %eax
141e205ba:     	movq	0x10(%rdx), %rcx
141e205be:     	movl	%eax, (%rcx)
141e205c0:     	movq	(%rdx), %rsi
141e205c3:     	movq	0x30(%rsi), %rdi
141e205c7:     	movq	0x40(%rsi), %r14
141e205cb:     	cmpq	0x20(%rsi), %rdi
141e205cf:     	jne	0x141e205e1 <.text+0x1e1f5e1>
141e205d1:     	leaq	0x20(%rsi), %rcx
141e205d5:     	leaq	0xd94c94(%rip), %rdx    # 0x142bb5270
141e205dc:     	callq	0x140562630 <.text+0x561630>
141e205e1:     	movq	0x28(%rsi), %rax
141e205e5:     	movq	%rdi, %rcx
141e205e8:     	shlq	$0x4, %rcx
141e205ec:     	movq	%r14, (%rax,%rcx)
141e205f0:     	movb	$0xc, 0x8(%rax,%rcx)
141e205f5:     	jmp	0x141e206bc <.text+0x1e1f6bc>
141e205fa:     	leaq	0xbf79ef(%rip), %rcx    # 0x142a17ff0
141e20601:     	callq	0x1428d9310 <.text+0x28d8310>
141e20606:     	jmp	0x141e22ef7 <.text+0x1e21ef7>
141e2060b:     	testq	%rax, %rax
141e2060e:     	jne	0x141e22e2a <.text+0x1e21e2a>
141e20614:     	xorl	%edx, %edx
141e20616:     	callq	0x1428e6250 <.text+0x28e5250>
141e2061b:     	movq	%rax, %rcx
141e2061e:     	testq	%rax, %rax
141e20621:     	jne	0x141e200ba <.text+0x1e1f0ba>
141e20627:     	jmp	0x141e22e2a <.text+0x1e21e2a>
141e2062c:     	testq	%rax, %rax
141e2062f:     	jne	0x141e22e2a <.text+0x1e21e2a>
141e20635:     	xorl	%edx, %edx
141e20637:     	callq	0x1428e6250 <.text+0x28e5250>
141e2063c:     	movq	%rax, %rcx
141e2063f:     	testq	%rax, %rax
141e20642:     	jne	0x141e20389 <.text+0x1e1f389>
141e20648:     	jmp	0x141e22e2a <.text+0x1e21e2a>
141e2064d:     	movq	0x27e0(%rbp), %rdx
141e20654:     	movl	0x1c(%rdx), %eax
141e20657:     	movq	0x10(%rdx), %rcx
141e2065b:     	movl	%eax, (%rcx)
141e2065d:     	movq	(%rdx), %rcx
141e20660:     	movq	$0x5, 0x700(%rbp)
141e2066b:     	leaq	0x700(%rbp), %rdx
141e20672:     	callq	0x14093fad0 <.text+0x93ead0>
141e20677:     	movq	0x2810(%rbp), %rdx
141e2067e:     	movl	0x1c(%rdx), %eax
141e20681:     	movq	0x10(%rdx), %rcx
141e20685:     	movl	%eax, (%rcx)
141e20687:     	movq	(%rdx), %rsi
141e2068a:     	movq	0x30(%rsi), %rdi
141e2068e:     	movq	0x40(%rsi), %r14
141e20692:     	cmpq	0x20(%rsi), %rdi
141e20696:     	jne	0x141e206a8 <.text+0x1e1f6a8>
141e20698:     	leaq	0x20(%rsi), %rcx
141e2069c:     	leaq	0xd94bcd(%rip), %rdx    # 0x142bb5270
141e206a3:     	callq	0x140562630 <.text+0x561630>
141e206a8:     	movq	0x28(%rsi), %rax
141e206ac:     	movq	%rdi, %rcx
141e206af:     	shlq	$0x4, %rcx
141e206b3:     	movq	%r14, (%rax,%rcx)
141e206b7:     	movb	$0x7, 0x8(%rax,%rcx)
141e206bc:     	incq	%rdi
141e206bf:     	movq	%rdi, 0x30(%rsi)
141e206c3:     	incq	0x40(%rsi)
141e206c7:     	leaq	0x24d0(%rbp), %rcx
141e206ce:     	callq	0x140f9c4f0 <.text+0xf9b4f0>
141e206d3:     	movss	0x48(%rax), %xmm6
141e206d8:     	movd	0x4c(%rax), %xmm7
141e206dd:     	leaq	0x24d0(%rbp), %rcx
141e206e4:     	callq	0x140f9c4f0 <.text+0xf9b4f0>
141e206e9:     	movq	0x8(%rax), %r14
141e206ed:     	leaq	-0x1(%r14), %rcx
141e206f1:     	cmpq	%rbx, %rcx
141e206f4:     	setb	%dl
141e206f7:     	andb	(%rax), %dl
141e206f9:     	cmpb	$0x1, %dl
141e206fc:     	jne	0x141e2116a <.text+0x1e2016a>
141e20702:     	leaq	(%rcx,%rcx,8), %rax
141e20706:     	leaq	(%rax,%rax,2), %rax
141e2070a:     	addq	%rcx, %rax
141e2070d:     	movups	(%r12,%rax), %xmm0
141e20712:     	movaps	%xmm0, 0x25c0(%rbp)
141e20719:     	movq	0x180b978(%rip), %r15   # 0x14362c098
141e20720:     	cmpq	$0x3, %rcx
141e20724:     	jb	0x141e2087c <.text+0x1e1f87c>
141e2072a:     	cmpb	$0x0, (%r15)
141e2072e:     	je	0x141e2087c <.text+0x1e1f87c>
141e20734:     	movl	0x2714(%rbp), %eax
141e2073a:     	movq	0x26e8(%rbp), %rcx
141e20741:     	movl	%eax, (%rcx)
141e20743:     	leaq	0x1167b16(%rip), %rax   # 0x142f88260
141e2074a:     	movq	%rax, 0x700(%rbp)
141e20751:     	movq	$0x9, 0x708(%rbp)
141e2075c:     	leaq	0x700(%rbp), %rcx
141e20763:     	callq	0x1405bdda0 <.text+0x5bcda0>
141e20768:     	movq	%rax, %rdi
141e2076b:     	movabsq	$-0x4e96acdfffb04bbd, %rax # imm = 0xB1695320004FB443
141e20775:     	xorq	%rax, %rdi
141e20778:     	movq	0x2728(%rbp), %rcx
141e2077f:     	movq	%rdi, %rdx
141e20782:     	callq	0x140683ec0 <.text+0x682ec0>
141e20787:     	movb	$0x0, 0x5d(%rax)
141e2078b:     	testb	$0x1, %dl
141e2078e:     	jne	0x141e20815 <.text+0x1e1f815>
141e20794:     	movq	0x2728(%rbp), %rax
141e2079b:     	movq	0x10(%rax), %rsi
141e2079f:     	cmpq	(%rax), %rsi
141e207a2:     	jne	0x141e207b7 <.text+0x1e1f7b7>
141e207a4:     	leaq	0xd8e81d(%rip), %rdx    # 0x142baefc8
141e207ab:     	movq	0x2728(%rbp), %rcx
141e207b2:     	callq	0x1406319d0 <.text+0x6309d0>
141e207b7:     	movss	0xb8db81(%rip), %xmm0   # 0x1429ae340
141e207bf:     	addss	%xmm6, %xmm0
141e207c3:     	movq	0x2728(%rbp), %rdx
141e207ca:     	movq	0x8(%rdx), %rax
141e207ce:     	imulq	$0x38, %rsi, %rcx
141e207d2:     	movl	$0x4, (%rax,%rcx)
141e207d9:     	movss	%xmm0, 0x4(%rax,%rcx)
141e207df:     	movd	%xmm7, 0x8(%rax,%rcx)
141e207e5:     	movsd	0xb0dee3(%rip), %xmm0   # 0x14292e6d0
141e207ed:     	movsd	%xmm0, 0xc(%rax,%rcx)
141e207f3:     	movl	$0x1, 0x14(%rax,%rcx)
141e207fb:     	movsd	0x11676ed(%rip), %xmm0  # 0x142f87ef0
141e20803:     	movsd	%xmm0, 0x18(%rax,%rcx)
141e20809:     	movq	%rdi, 0x30(%rax,%rcx)
141e2080e:     	incq	%rsi
141e20811:     	movq	%rsi, 0x10(%rdx)
141e20815:     	movq	0x2728(%rbp), %rax
141e2081c:     	movq	%rax, 0x1ce0(%rbp)
141e20823:     	movq	%rdi, 0x1ce8(%rbp)
141e2082a:     	movq	$0x0, 0x1cf0(%rbp)
141e20835:     	movq	$0x0, 0x20(%rsp)
141e2083e:     	leaq	0x11679ff(%rip), %r8    # 0x142f88244
141e20845:     	leaq	0x700(%rbp), %rcx
141e2084c:     	leaq	0x1ce0(%rbp), %rdx
141e20853:     	movl	$0x18, %r9d
141e20859:     	callq	0x140f9e970 <.text+0xf9d970>
141e2085e:     	leaq	0x11679f7(%rip), %rdx   # 0x142f8825c
141e20865:     	leaq	0x700(%rbp), %rcx
141e2086c:     	movl	$0x4, %r8d
141e20872:     	callq	0x140fbe1a0 <.text+0xfbd1a0>
141e20877:     	jmp	0x141e2096c <.text+0x1e1f96c>
141e2087c:     	movq	0x2608(%rbp), %rax
141e20883:     	movq	(%rax), %rdx
141e20886:     	leaq	0x25c0(%rbp), %rcx
141e2088d:     	callq	0x140a9fa80 <.text+0xa9ea80>
141e20892:     	movb	%al, 0x700(%rbp)
141e20898:     	leaq	0x700(%rbp), %rsi
141e2089f:     	movq	%rsi, %rcx
141e208a2:     	callq	0x140833230 <.text+0x832230>
141e208a7:     	testb	%al, %al
141e208a9:     	jne	0x141e2096c <.text+0x1e1f96c>
141e208af:     	movl	0x2714(%rbp), %eax
141e208b5:     	movq	0x26e8(%rbp), %rcx
141e208bc:     	movl	%eax, (%rcx)
141e208be:     	movss	0xb8da7a(%rip), %xmm0   # 0x1429ae340
141e208c6:     	addss	%xmm6, %xmm0
141e208ca:     	movq	0x116761e(%rip), %xmm1  # 0x142f87ef0
141e208d2:     	movq	%xmm1, 0x704(%rbp)
141e208da:     	movl	$0x1, 0x700(%rbp)
141e208e4:     	movq	%rsi, 0x30(%rsp)
141e208e9:     	movd	%xmm7, 0x28(%rsp)
141e208ef:     	movss	%xmm0, 0x20(%rsp)
141e208f5:     	movb	$0x0, 0x48(%rsp)
141e208fa:     	movl	$0xbf800000, 0x40(%rsp) # imm = 0xBF800000
141e20902:     	movl	$0xbf800000, 0x38(%rsp) # imm = 0xBF800000
141e2090a:     	leaq	0x1167927(%rip), %r8    # 0x142f88238
141e20911:     	leaq	0x1ce0(%rbp), %rcx
141e20918:     	movl	$0xc, %r9d
141e2091e:     	movq	0x2728(%rbp), %rdx
141e20925:     	callq	0x140f992e0 <.text+0xf982e0>
141e2092a:     	movq	$0x0, 0x20(%rsp)
141e20933:     	leaq	0x116790a(%rip), %r8    # 0x142f88244
141e2093a:     	leaq	0x700(%rbp), %rcx
141e20941:     	leaq	0x1ce0(%rbp), %rdx
141e20948:     	movl	$0x18, %r9d
141e2094e:     	callq	0x140f9e970 <.text+0xf9d970>
141e20953:     	leaq	0x1167902(%rip), %rdx   # 0x142f8825c
141e2095a:     	leaq	0x700(%rbp), %rcx
141e20961:     	movl	$0x4, %r8d
141e20967:     	callq	0x140fb9a80 <.text+0xfb8a80>
141e2096c:     	movzbl	(%r15), %eax
141e20970:     	orb	0x2734(%rbp), %al
141e20976:     	testb	$0x1, %al
141e20978:     	jne	0x141e209b6 <.text+0x1e1f9b6>
141e2097a:     	cmpq	$0x1, %r14
141e2097e:     	setne	0x38(%rsp)
141e20983:     	movzbl	0x2670(%rbp), %eax
141e2098a:     	movb	%al, 0x40(%rsp)
141e2098e:     	movq	0x2700(%rbp), %rax
141e20995:     	movq	%rax, 0x30(%rsp)
141e2099a:     	movq	%rbx, 0x28(%rsp)
141e2099f:     	movq	%r12, 0x20(%rsp)
141e209a4:     	movq	%r13, %rcx
141e209a7:     	movaps	%xmm6, %xmm1
141e209aa:     	movdqa	%xmm7, %xmm2
141e209ae:     	movq	%r14, %r9
141e209b1:     	callq	0x141e23d20 <.text+0x1e22d20>
141e209b6:     	leaq	0x700(%rbp), %rcx
141e209bd:     	leaq	0x25c0(%rbp), %rdx
141e209c4:     	callq	0x140b46000 <.text+0xb45000>
141e209c9:     	leaq	0x700(%rbp), %r14
141e209d0:     	movq	%r14, %rcx
141e209d3:     	callq	0x140d76ed0 <.text+0xd75ed0>
141e209d8:     	movq	%rdx, %r8
141e209db:     	movq	$0xc, 0x20(%rsp)
141e209e4:     	leaq	0x11677fe(%rip), %r9    # 0x142f881e9
141e209eb:     	leaq	0x26c0(%rbp), %rcx
141e209f2:     	movq	%rax, %rdx
141e209f5:     	callq	0x140d62810 <.text+0xd61810>
141e209fa:     	movq	0x700(%rbp), %rdx
141e20a01:     	testq	%rdx, %rdx
141e20a04:     	movq	0x26f8(%rbp), %rbx
141e20a0b:     	je	0x141e20a1f <.text+0x1e1fa1f>
141e20a0d:     	movq	0x708(%rbp), %rcx
141e20a14:     	movl	$0x1, %r8d
141e20a1a:     	callq	0x140613c20 <.text+0x612c20>
141e20a1f:     	xorl	%eax, %eax
141e20a21:     	cmpq	(%rbx), %rax
141e20a24:     	jo	0x141e2114c <.text+0x1e2014c>
141e20a2a:     	movq	%rbx, %rcx
141e20a2d:     	callq	0x140d76ed0 <.text+0xd75ed0>
141e20a32:     	movq	%rdx, %r8
141e20a35:     	leaq	0x2610(%rbp), %rcx
141e20a3c:     	movq	%rax, %rdx
141e20a3f:     	callq	0x140d62c00 <.text+0xd61c00>
141e20a44:     	leaq	0x26c0(%rbp), %rcx
141e20a4b:     	callq	0x140d76ed0 <.text+0xd75ed0>
141e20a50:     	movq	%rdx, %r8
141e20a53:     	leaq	0x2510(%rbp), %rcx
141e20a5a:     	movq	%rax, %rdx
141e20a5d:     	callq	0x140d62c00 <.text+0xd61c00>
141e20a62:     	movq	0x2618(%rbp), %rdi
141e20a69:     	movq	0x2518(%rbp), %rsi
141e20a70:     	cmpq	%rsi, %rdi
141e20a73:     	jne	0x141e20ec1 <.text+0x1e1fec1>
141e20a79:     	movzbl	0x2648(%rbp), %eax
141e20a80:     	cmpb	0x2548(%rbp), %al
141e20a86:     	jne	0x141e20ec1 <.text+0x1e1fec1>
141e20a8c:     	cmpb	$0x2, 0x2649(%rbp)
141e20a93:     	jne	0x141e20ec1 <.text+0x1e1fec1>
141e20a99:     	cmpb	$0x2, 0x2549(%rbp)
141e20aa0:     	jne	0x141e20ec1 <.text+0x1e1fec1>
141e20aa6:     	movzbl	0x2620(%rbp), %eax
141e20aad:     	cmpb	$0x6, %al
141e20aaf:     	jne	0x141e20d3f <.text+0x1e1fd3f>
141e20ab5:     	cmpb	$0x3, 0x2520(%rbp)
141e20abc:     	jae	0x141e20ea3 <.text+0x1e1fea3>
141e20ac2:     	jmp	0x141e20ec1 <.text+0x1e1fec1>
141e20ac7:     	movq	(%r14), %rax
141e20aca:     	movdqu	(%r15), %xmm0
141e20acf:     	movdqu	0x60(%rax), %xmm1
141e20ad4:     	pcmpeqb	%xmm0, %xmm1
141e20ad8:     	pmovmskb	%xmm1, %eax
141e20adc:     	cmpl	$0xffff, %eax           # imm = 0xFFFF
141e20ae1:     	je	0x141e20e50 <.text+0x1e1fe50>
141e20ae7:     	movdqu	(%r15), %xmm0
141e20aec:     	movdqa	%xmm0, 0x1ce0(%rbp)
141e20af4:     	leaq	0x700(%rbp), %rcx
141e20afb:     	leaq	0x1ce0(%rbp), %rdx
141e20b02:     	callq	0x141e23c00 <.text+0x1e22c00>
141e20b07:     	leaq	0x1ce0(%rbp), %rcx
141e20b0e:     	leaq	0x700(%rbp), %rdx
141e20b15:     	callq	0x141e1f310 <.text+0x1e1e310>
141e20b1a:     	leaq	0x700(%rbp), %rcx
141e20b21:     	leaq	0x1ce0(%rbp), %rdx
141e20b28:     	callq	0x140b46370 <.text+0xb45370>
141e20b2d:     	leaq	0x2510(%rbp), %rcx
141e20b34:     	leaq	0x700(%rbp), %rdx
141e20b3b:     	callq	0x1408bbbc0 <.text+0x8babc0>
141e20b40:     	leaq	0x1ce0(%rbp), %rdi
141e20b47:     	movq	%rdi, %rcx
141e20b4a:     	callq	0x14014b4d0 <.text+0x14a4d0>
141e20b4f:     	leaq	0x700(%rbp), %rcx
141e20b56:     	movq	%r15, %rdx
141e20b59:     	callq	0x140b46000 <.text+0xb45000>
141e20b5e:     	leaq	0x700(%rbp), %rcx
141e20b65:     	callq	0x140d76ed0 <.text+0xd75ed0>
141e20b6a:     	movq	%rdx, %r8
141e20b6d:     	movq	$0xc, 0x20(%rsp)
141e20b76:     	leaq	0x116766c(%rip), %r9    # 0x142f881e9
141e20b7d:     	leaq	0x2610(%rbp), %rcx
141e20b84:     	movq	%rax, %rdx
141e20b87:     	callq	0x140d62810 <.text+0xd61810>
141e20b8c:     	movq	0x700(%rbp), %rdx
141e20b93:     	testq	%rdx, %rdx
141e20b96:     	je	0x141e20baa <.text+0x1e1fbaa>
141e20b98:     	movq	0x708(%rbp), %rcx
141e20b9f:     	movl	$0x1, %r8d
141e20ba5:     	callq	0x140613c20 <.text+0x612c20>
141e20baa:     	movb	$0x1, 0x2737(%rbp)
141e20bb1:     	leaq	0x2610(%rbp), %rcx
141e20bb8:     	callq	0x140d76ed0 <.text+0xd75ed0>
141e20bbd:     	movq	%rdx, %r8
141e20bc0:     	movb	$0x1, 0x2737(%rbp)
141e20bc7:     	leaq	0x25e0(%rbp), %rcx
141e20bce:     	leaq	0x2510(%rbp), %r9
141e20bd5:     	movq	%rax, %rdx
141e20bd8:     	callq	0x141470d00 <.text+0x146fd00>
141e20bdd:     	cmpb	$0x2, 0x25ef(%rbp)
141e20be4:     	jne	0x141e20d5c <.text+0x1e1fd5c>
141e20bea:     	movq	0x25e0(%rbp), %rax
141e20bf1:     	movq	%rax, 0x26c0(%rbp)
141e20bf8:     	movq	0x2610(%rbp), %rax
141e20bff:     	movq	%rax, 0x26f0(%rbp)
141e20c06:     	movq	0x2618(%rbp), %rax
141e20c0d:     	movq	%rax, 0x26b8(%rbp)
141e20c14:     	movdqu	0x2620(%rbp), %xmm0
141e20c1c:     	movdqa	%xmm0, 0x2690(%rbp)
141e20c24:     	callq	0x140d635e0 <.text+0xd625e0>
141e20c29:     	movl	$0x2, %r8d
141e20c2f:     	movq	%rax, %rcx
141e20c32:     	xorl	%r9d, %r9d
141e20c35:     	callq	0x140d63a20 <.text+0xd62a20>
141e20c3a:     	movq	%rax, 0x2680(%rbp)
141e20c41:     	movl	%edx, 0x2678(%rbp)
141e20c47:     	leaq	0x26c0(%rbp), %rax
141e20c4e:     	movq	%rax, 0x1ce0(%rbp)
141e20c55:     	leaq	-0x10d403c(%rip), %rax  # 0x140d4cc20 <.text+0xd4bc20>
141e20c5c:     	movq	%rax, 0x1ce8(%rbp)
141e20c63:     	leaq	0x11675a6(%rip), %rax   # 0x142f88210
141e20c6a:     	movq	%rax, 0x700(%rbp)
141e20c71:     	movq	$0x1, 0x708(%rbp)
141e20c7c:     	movq	$0x0, 0x720(%rbp)
141e20c87:     	movq	%rdi, 0x710(%rbp)
141e20c8e:     	movq	$0x1, 0x718(%rbp)
141e20c99:     	leaq	0x2498(%rbp), %rcx
141e20ca0:     	leaq	0x700(%rbp), %rdx
141e20ca7:     	callq	0x141e1f3a0 <.text+0x1e1e3a0>
141e20cac:     	movaps	0x2690(%rbp), %xmm0
141e20cb3:     	movaps	%xmm0, 0x700(%rbp)
141e20cba:     	xorl	%eax, %eax
141e20cbc:     	movq	0x26f8(%rbp), %rcx
141e20cc3:     	cmpq	(%rcx), %rax
141e20cc6:     	jo	0x141e20cd4 <.text+0x1e1fcd4>
141e20cc8:     	movq	0x26f8(%rbp), %rcx
141e20ccf:     	callq	0x140fa6ca0 <.text+0xfa5ca0>
141e20cd4:     	movq	0x26f0(%rbp), %rax
141e20cdb:     	movq	0x26f8(%rbp), %rcx
141e20ce2:     	movq	%rax, (%rcx)
141e20ce5:     	movq	0x26b8(%rbp), %rax
141e20cec:     	movq	%rax, 0x8(%rcx)
141e20cf0:     	movaps	0x700(%rbp), %xmm0
141e20cf7:     	movups	%xmm0, 0x10(%rcx)
141e20cfb:     	movdqu	0x2498(%rbp), %xmm0
141e20d03:     	movdqu	%xmm0, 0x20(%rcx)
141e20d08:     	movq	0x24a8(%rbp), %rax
141e20d0f:     	movq	%rax, 0x30(%rcx)
141e20d13:     	movq	0x2680(%rbp), %rax
141e20d1a:     	movq	%rax, 0x38(%rcx)
141e20d1e:     	movl	0x2678(%rbp), %eax
141e20d24:     	movl	%eax, 0x40(%rcx)
141e20d27:     	movb	$0x0, 0x2737(%rbp)
141e20d2e:     	leaq	0x26c0(%rbp), %rcx
141e20d35:     	callq	0x140d4cc30 <.text+0xd4bc30>
141e20d3a:     	jmp	0x141e206c7 <.text+0x1e1f6c7>
141e20d3f:     	movzbl	0x2520(%rbp), %ecx
141e20d46:     	cmpb	$0x6, %cl
141e20d49:     	jne	0x141e20e94 <.text+0x1e1fe94>
141e20d4f:     	cmpb	$0x3, %al
141e20d51:     	jae	0x141e20ea3 <.text+0x1e1fea3>
141e20d57:     	jmp	0x141e20ec1 <.text+0x1e1fec1>
141e20d5c:     	movups	0x25e0(%rbp), %xmm0
141e20d63:     	movaps	%xmm0, 0x1d10(%rbp)
141e20d6a:     	movq	0x27f0(%rbp), %rdx
141e20d71:     	movl	0x1c(%rdx), %eax
141e20d74:     	movq	0x10(%rdx), %rcx
141e20d78:     	movl	%eax, (%rcx)
141e20d7a:     	movq	(%rdx), %rax
141e20d7d:     	movb	$0x2, (%rax)
141e20d80:     	movdqu	(%r15), %xmm0
141e20d85:     	movdqa	%xmm0, 0x1d00(%rbp)
141e20d8d:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
141e20d97:     	movq	%rax, 0x718(%rbp)
141e20d9e:     	movq	$0x0, 0x700(%rbp)
141e20da9:     	movq	$0x8, 0x708(%rbp)
141e20db4:     	movq	$0x0, 0x710(%rbp)
141e20dbf:     	movb	$0x0, 0x798(%rbp)
141e20dc6:     	movabsq	$-0x7ffffffffffffffd, %rax # imm = 0x8000000000000003
141e20dd0:     	movq	%rax, 0x1cf8(%rbp)
141e20dd7:     	movq	0x700(%rbp), %rax
141e20dde:     	movq	%rax, 0x1ce0(%rbp)
141e20de5:     	movq	0x708(%rbp), %rax
141e20dec:     	movq	%rax, 0x1ce8(%rbp)
141e20df3:     	movq	$0x0, 0x1cf0(%rbp)
141e20dfe:     	movb	$0x0, 0x1d78(%rbp)
141e20e05:     	leaq	0x11673ec(%rip), %r8    # 0x142f881f8
141e20e0c:     	leaq	0x1ce0(%rbp), %rdx
141e20e13:     	movq	0x27f8(%rbp), %rcx
141e20e1a:     	callq	0x140fe0190 <.text+0xfdf190>
141e20e1f:     	leaq	0x718(%rbp), %rcx
141e20e26:     	movb	$0x1, 0x2737(%rbp)
141e20e2d:     	callq	0x140fb63f0 <.text+0xfb53f0>
141e20e32:     	movq	0x2610(%rbp), %rdx
141e20e39:     	testq	%rdx, %rdx
141e20e3c:     	je	0x141e20e50 <.text+0x1e1fe50>
141e20e3e:     	movq	0x2618(%rbp), %rcx
141e20e45:     	movl	$0x1, %r8d
141e20e4b:     	callq	0x140613c20 <.text+0x612c20>
141e20e50:     	movq	0x27e0(%rbp), %rdx
141e20e57:     	movl	0x1c(%rdx), %eax
141e20e5a:     	movq	0x10(%rdx), %rcx
141e20e5e:     	movl	%eax, (%rcx)
141e20e60:     	movq	(%rdx), %rcx
141e20e63:     	movq	$0x2, 0x700(%rbp)
141e20e6e:     	leaq	0x700(%rbp), %rdx
141e20e75:     	callq	0x14093fad0 <.text+0x93ead0>
141e20e7a:     	leaq	0x116739f(%rip), %r8    # 0x142f88220
141e20e81:     	movq	0x2810(%rbp), %rcx
141e20e88:     	movb	$0x8, %dl
141e20e8a:     	callq	0x140fcfb80 <.text+0xfceb80>
141e20e8f:     	jmp	0x141e206c7 <.text+0x1e1f6c7>
141e20e94:     	cmpb	$0x3, %al
141e20e96:     	setb	%al
141e20e99:     	cmpb	$0x3, %cl
141e20e9c:     	setb	%cl
141e20e9f:     	xorb	%al, %cl
141e20ea1:     	jne	0x141e20ec1 <.text+0x1e1fec1>
141e20ea3:     	movq	0x2610(%rbp), %rcx
141e20eaa:     	movq	0x2510(%rbp), %rdx
141e20eb1:     	movq	%rdi, %r8
141e20eb4:     	callq	0x142923bf0 <.text+0x2922bf0>
141e20eb9:     	testl	%eax, %eax
141e20ebb:     	je	0x141e2100e <.text+0x1e2000e>
141e20ec1:     	movq	0x2610(%rbp), %rcx
141e20ec8:     	movzbl	0x2620(%rbp), %eax
141e20ecf:     	cmpb	$0x6, %al
141e20ed1:     	je	0x141e20eff <.text+0x1e1feff>
141e20ed3:     	movq	0x2640(%rbp), %rdx
141e20eda:     	movq	%rdx, 0x25ff(%rbp)
141e20ee1:     	movups	0x2621(%rbp), %xmm0
141e20ee8:     	movdqu	0x2631(%rbp), %xmm1
141e20ef0:     	movdqa	%xmm1, 0x25f0(%rbp)
141e20ef8:     	movaps	%xmm0, 0x25e0(%rbp)
141e20eff:     	movzbl	0x264a(%rbp), %edx
141e20f06:     	movzwl	0x2648(%rbp), %r8d
141e20f0e:     	movq	%rcx, 0x1ce0(%rbp)
141e20f15:     	movq	%rdi, 0x1ce8(%rbp)
141e20f1c:     	movb	%al, 0x1cf0(%rbp)
141e20f22:     	movaps	0x25e0(%rbp), %xmm0
141e20f29:     	movaps	0x25f0(%rbp), %xmm1
141e20f30:     	movups	%xmm0, 0x1cf1(%rbp)
141e20f37:     	movups	%xmm1, 0x1d01(%rbp)
141e20f3e:     	movq	0x25ff(%rbp), %rax
141e20f45:     	movq	%rax, 0x1d10(%rbp)
141e20f4c:     	movw	%r8w, 0x1d18(%rbp)
141e20f54:     	movb	%dl, 0x1d1a(%rbp)
141e20f5a:     	movq	0x2510(%rbp), %rcx
141e20f61:     	movzbl	0x2520(%rbp), %eax
141e20f68:     	cmpb	$0x6, %al
141e20f6a:     	je	0x141e20f96 <.text+0x1e1ff96>
141e20f6c:     	movq	0x2540(%rbp), %rdx
141e20f73:     	movq	%rdx, 0x26af(%rbp)
141e20f7a:     	movups	0x2521(%rbp), %xmm0
141e20f81:     	movups	0x2531(%rbp), %xmm1
141e20f88:     	movaps	%xmm1, 0x26a0(%rbp)
141e20f8f:     	movaps	%xmm0, 0x2690(%rbp)
141e20f96:     	movzbl	0x254a(%rbp), %edx
141e20f9d:     	movzwl	0x2548(%rbp), %r8d
141e20fa5:     	movq	%rcx, 0x700(%rbp)
141e20fac:     	movq	%rsi, 0x708(%rbp)
141e20fb3:     	movb	%al, 0x710(%rbp)
141e20fb9:     	movaps	0x2690(%rbp), %xmm0
141e20fc0:     	movdqa	0x26a0(%rbp), %xmm1
141e20fc8:     	movups	%xmm0, 0x711(%rbp)
141e20fcf:     	movdqu	%xmm1, 0x721(%rbp)
141e20fd7:     	movq	0x26af(%rbp), %rax
141e20fde:     	movq	%rax, 0x730(%rbp)
141e20fe5:     	movw	%r8w, 0x738(%rbp)
141e20fed:     	movb	%dl, 0x73a(%rbp)
141e20ff3:     	leaq	0x1ce0(%rbp), %rcx
141e20ffa:     	leaq	0x700(%rbp), %rdx
141e21001:     	callq	0x140fb3560 <.text+0xfb2560>
141e21006:     	testb	%al, %al
141e21008:     	je	0x141e2114c <.text+0x1e2014c>
141e2100e:     	callq	0x140d635e0 <.text+0xd625e0>
141e21013:     	xorl	%ecx, %ecx
141e21015:     	cmpq	%rax, 0x38(%rbx)
141e21019:     	setne	%cl
141e2101c:     	movl	$0xff, %eax
141e21021:     	cmovbl	%eax, %ecx
141e21024:     	xorl	%r8d, %r8d
141e21027:     	cmpl	%edx, 0x40(%rbx)
141e2102a:     	setne	%r8b
141e2102e:     	cmovbl	%eax, %r8d
141e21032:     	testb	%cl, %cl
141e21034:     	cmovnel	%ecx, %r8d
141e21038:     	cmpb	$0x1, %r8b
141e2103c:     	jne	0x141e21125 <.text+0x1e20125>
141e21042:     	movl	0x2714(%rbp), %eax
141e21048:     	movq	0x26e8(%rbp), %rcx
141e2104f:     	movl	%eax, (%rcx)
141e21051:     	movl	$0x0, 0x700(%rbp)
141e2105b:     	movq	%r14, 0x30(%rsp)
141e21060:     	movd	%xmm7, 0x28(%rsp)
141e21066:     	movss	%xmm6, 0x20(%rsp)
141e2106c:     	movb	$0x0, 0x48(%rsp)
141e21071:     	movl	$0x0, 0x40(%rsp)
141e21079:     	movl	$0x0, 0x38(%rsp)
141e21081:     	leaq	0x11671e1(%rip), %r8    # 0x142f88269
141e21088:     	leaq	0x2468(%rbp), %rcx
141e2108f:     	movl	$0xd, %r9d
141e21095:     	movq	0x2728(%rbp), %rdx
141e2109c:     	callq	0x140f992e0 <.text+0xf982e0>
141e210a1:     	leaq	0x2468(%rbp), %rcx
141e210a8:     	callq	0x140683ce0 <.text+0x682ce0>
141e210ad:     	testb	%al, %al
141e210af:     	je	0x141e210f3 <.text+0x1e200f3>
141e210b1:     	movq	0x38(%rbx), %rsi
141e210b5:     	movl	0x40(%rbx), %edi
141e210b8:     	callq	0x140d635e0 <.text+0xd625e0>
141e210bd:     	movq	%rax, %rcx
141e210c0:     	xorl	%r8d, %r8d
141e210c3:     	movl	$0xbebc200, %r9d        # imm = 0xBEBC200
141e210c9:     	callq	0x140d63a20 <.text+0xd62a20>
141e210ce:     	xorl	%ecx, %ecx
141e210d0:     	cmpl	%edi, %edx
141e210d2:     	setb	%cl
141e210d5:     	xorl	%r8d, %r8d
141e210d8:     	cmpq	%rsi, %rax
141e210db:     	setb	%r8b
141e210df:     	cmovnel	%r8d, %ecx
141e210e3:     	testb	%cl, %cl
141e210e5:     	cmovnel	%edi, %edx
141e210e8:     	cmovneq	%rsi, %rax
141e210ec:     	movq	%rax, 0x38(%rbx)
141e210f0:     	movl	%edx, 0x40(%rbx)
141e210f3:     	movq	0x28(%rbx), %rdx
141e210f7:     	movq	0x30(%rbx), %r8
141e210fb:     	movq	0x2478(%rbp), %rax
141e21102:     	movq	%rax, 0x710(%rbp)
141e21109:     	movups	0x2468(%rbp), %xmm0
141e21110:     	movaps	%xmm0, 0x700(%rbp)
141e21117:     	leaq	0x700(%rbp), %rcx
141e2111e:     	callq	0x141e23b90 <.text+0x1e22b90>
141e21123:     	jmp	0x141e2114c <.text+0x1e2014c>
141e21125:     	xorl	%eax, %eax
141e21127:     	cmpq	(%rbx), %rax
141e2112a:     	jo	0x141e21138 <.text+0x1e20138>
141e2112c:     	movq	0x26f8(%rbp), %rcx
141e21133:     	callq	0x140fa6ca0 <.text+0xfa5ca0>
141e21138:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
141e21142:     	movq	0x26f8(%rbp), %rcx
141e21149:     	movq	%rax, (%rcx)
141e2114c:     	movq	0x26c0(%rbp), %rdx
141e21153:     	testq	%rdx, %rdx
141e21156:     	je	0x141e2116a <.text+0x1e2016a>
141e21158:     	movq	0x26c8(%rbp), %rcx
141e2115f:     	movl	$0x1, %r8d
141e21165:     	callq	0x140613c20 <.text+0x612c20>
141e2116a:     	movq	0x2700(%rbp), %rdi
141e21171:     	cmpq	$0x0, (%rdi)
141e21175:     	je	0x141e211e5 <.text+0x1e201e5>
141e21177:     	cmpq	$0x0, 0x2708(%rbp)
141e2117f:     	je	0x141e2118e <.text+0x1e2018e>
141e21181:     	movq	0x2718(%rbp), %rax
141e21188:     	cmpq	0x8(%rdi), %rax
141e2118c:     	je	0x141e211e5 <.text+0x1e201e5>
141e2118e:     	movq	0x2810(%rbp), %rdx
141e21195:     	movl	0x1c(%rdx), %eax
141e21198:     	movq	0x10(%rdx), %rcx
141e2119c:     	movl	%eax, (%rcx)
141e2119e:     	movq	(%rdx), %rsi
141e211a1:     	movq	0x30(%rsi), %rdi
141e211a5:     	movq	0x40(%rsi), %rbx
141e211a9:     	cmpq	0x20(%rsi), %rdi
141e211ad:     	jne	0x141e211bf <.text+0x1e201bf>
141e211af:     	leaq	0x20(%rsi), %rcx
141e211b3:     	leaq	0xd940b6(%rip), %rdx    # 0x142bb5270
141e211ba:     	callq	0x140562630 <.text+0x561630>
141e211bf:     	movq	0x28(%rsi), %rax
141e211c3:     	movq	%rdi, %rcx
141e211c6:     	shlq	$0x4, %rcx
141e211ca:     	movq	%rbx, (%rax,%rcx)
141e211ce:     	movb	$0x0, 0x8(%rax,%rcx)
141e211d3:     	incq	%rdi
141e211d6:     	movq	%rdi, 0x30(%rsi)
141e211da:     	incq	0x40(%rsi)
141e211de:     	movq	0x2700(%rbp), %rdi
141e211e5:     	cmpb	$0x0, 0x38(%rdi)
141e211e9:     	je	0x141e217d4 <.text+0x1e207d4>
141e211ef:     	movdqu	0x39(%rdi), %xmm0
141e211f4:     	movdqa	%xmm0, 0x2440(%rbp)
141e211fc:     	movl	0x2714(%rbp), %eax
141e21202:     	movq	0x26e8(%rbp), %rcx
141e21209:     	movl	%eax, (%rcx)
141e2120b:     	leaq	0x1167064(%rip), %rax   # 0x142f88276
141e21212:     	movq	%rax, 0x700(%rbp)
141e21219:     	movq	$0x11, 0x708(%rbp)
141e21224:     	leaq	0x700(%rbp), %rcx
141e2122b:     	callq	0x1405bdda0 <.text+0x5bcda0>
141e21230:     	movq	%rax, %rdi
141e21233:     	movabsq	$-0x4e96acdfffb04bbd, %rax # imm = 0xB1695320004FB443
141e2123d:     	xorq	%rax, %rdi
141e21240:     	movq	0x2728(%rbp), %rcx
141e21247:     	movq	%rdi, %rdx
141e2124a:     	callq	0x140683ec0 <.text+0x682ec0>
141e2124f:     	movb	$0x0, 0x5d(%rax)
141e21253:     	movb	$0x1, 0xb7(%rax)
141e2125a:     	testb	$0x1, %dl
141e2125d:     	jne	0x141e212a4 <.text+0x1e202a4>
141e2125f:     	movq	0x2728(%rbp), %rax
141e21266:     	movq	0x10(%rax), %rsi
141e2126a:     	cmpq	(%rax), %rsi
141e2126d:     	jne	0x141e21282 <.text+0x1e20282>
141e2126f:     	leaq	0xd8dd52(%rip), %rdx    # 0x142baefc8
141e21276:     	movq	0x2728(%rbp), %rcx
141e2127d:     	callq	0x1406319d0 <.text+0x6309d0>
141e21282:     	movq	0x2728(%rbp), %rdx
141e21289:     	movq	0x8(%rdx), %rax
141e2128d:     	imulq	$0x38, %rsi, %rcx
141e21291:     	movl	$0x1, (%rax,%rcx)
141e21298:     	movq	%rdi, 0x30(%rax,%rcx)
141e2129d:     	incq	%rsi
141e212a0:     	movq	%rsi, 0x10(%rdx)
141e212a4:     	movq	0x2728(%rbp), %rax
141e212ab:     	movq	%rax, 0x2690(%rbp)
141e212b2:     	movq	%rdi, 0x2698(%rbp)
141e212b9:     	movq	$0x0, 0x26a0(%rbp)
141e212c4:     	movq	$0x0, 0x700(%rbp)
141e212cf:     	movabsq	$0x41223169ff28813b, %rsi # imm = 0x41223169FF28813B
141e212d9:     	movq	%rsi, 0x708(%rbp)
141e212e0:     	movabsq	$-0x58648d975d569727, %rbx # imm = 0xA79B7268A2A968D9
141e212ea:     	movq	%rbx, 0x710(%rbp)
141e212f1:     	leaq	0x700(%rbp), %rcx
141e212f8:     	callq	0x1405bdf20 <.text+0x5bcf20>
141e212fd:     	movq	%rdi, 0x700(%rbp)
141e21304:     	movq	%rax, 0x708(%rbp)
141e2130b:     	leaq	0x700(%rbp), %rcx
141e21312:     	callq	0x1405bd910 <.text+0x5bc910>
141e21317:     	leaq	0x2610(%rbp), %rcx
141e2131e:     	leaq	0x2690(%rbp), %rdx
141e21325:     	movq	%rax, %r8
141e21328:     	callq	0x1406861b0 <.text+0x6851b0>
141e2132d:     	movq	0x2618(%rbp), %rdi
141e21334:     	movq	$0x0, 0x700(%rbp)
141e2133f:     	movq	%rsi, 0x708(%rbp)
141e21346:     	movq	%rbx, 0x710(%rbp)
141e2134d:     	leaq	0x700(%rbp), %rcx
141e21354:     	callq	0x1405bdf20 <.text+0x5bcf20>
141e21359:     	movq	%rdi, 0x700(%rbp)
141e21360:     	movq	%rax, 0x708(%rbp)
141e21367:     	leaq	0x700(%rbp), %rcx
141e2136e:     	callq	0x1405bd910 <.text+0x5bc910>
141e21373:     	leaq	0x700(%rbp), %rcx
141e2137a:     	leaq	0x2610(%rbp), %rdx
141e21381:     	movq	%rax, %r8
141e21384:     	callq	0x1406861b0 <.text+0x6851b0>
141e21389:     	movq	$0x0, 0x20(%rsp)
141e21392:     	leaq	0x1166eee(%rip), %r8    # 0x142f88287
141e21399:     	leaq	0x1cb8(%rbp), %rcx
141e213a0:     	leaq	0x700(%rbp), %rdx
141e213a7:     	movl	$0x13, %r9d
141e213ad:     	callq	0x140f9e970 <.text+0xf9d970>
141e213b2:     	leaq	0x1166ee1(%rip), %r8    # 0x142f8829a
141e213b9:     	leaq	0x700(%rbp), %rcx
141e213c0:     	leaq	0x1cb8(%rbp), %rdx
141e213c7:     	movl	$0x6, %r9d
141e213cd:     	callq	0x140fb8070 <.text+0xfb7070>
141e213d2:     	movq	0x700(%rbp), %rsi
141e213d9:     	cmpq	$0x0, 0x1b8(%rsi)
141e213e1:     	je	0x141e2175b <.text+0x1e2075b>
141e213e7:     	leaq	0x708(%rbp), %rdx
141e213ee:     	leaq	0x1c0(%rsi), %rcx
141e213f5:     	callq	0x1405b0d70 <.text+0x5afd70>
141e213fa:     	movq	0x1a0(%rsi), %rcx
141e21401:     	movq	0x1a8(%rsi), %rdx
141e21408:     	movq	%rdx, %r8
141e2140b:     	andq	%rax, %r8
141e2140e:     	shrq	$0x39, %rax
141e21412:     	movd	%eax, %xmm0
141e21416:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
141e2141a:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
141e2141f:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
141e21424:     	leaq	-0xc0(%rcx), %rax
141e2142b:     	movq	0x708(%rbp), %r9
141e21432:     	xorl	%r10d, %r10d
141e21435:     	pcmpeqd	%xmm1, %xmm1
141e21439:     	movdqu	(%rcx,%r8), %xmm2
141e2143f:     	movdqa	%xmm2, %xmm3
141e21443:     	pcmpeqb	%xmm0, %xmm3
141e21447:     	pmovmskb	%xmm3, %r11d
141e2144c:     	testl	%r11d, %r11d
141e2144f:     	je	0x141e2147a <.text+0x1e2047a>
141e21451:     	tzcntl	%r11d, %esi
141e21456:     	addq	%r8, %rsi
141e21459:     	andq	%rdx, %rsi
141e2145c:     	negq	%rsi
141e2145f:     	leaq	(%rsi,%rsi,2), %rsi
141e21463:     	shlq	$0x6, %rsi
141e21467:     	cmpq	(%rax,%rsi), %r9
141e2146b:     	je	0x141e2149c <.text+0x1e2049c>
141e2146d:     	leal	-0x1(%r11), %esi
141e21471:     	andw	%r11w, %si
141e21475:     	movl	%esi, %r11d
141e21478:     	jne	0x141e21451 <.text+0x1e20451>
141e2147a:     	pcmpeqb	%xmm1, %xmm2
141e2147e:     	pmovmskb	%xmm2, %r11d
141e21483:     	testl	%r11d, %r11d
141e21486:     	jne	0x141e2175b <.text+0x1e2075b>
141e2148c:     	addq	%r10, %r8
141e2148f:     	addq	$0x10, %r8
141e21493:     	addq	$0x10, %r10
141e21497:     	andq	%rdx, %r8
141e2149a:     	jmp	0x141e21439 <.text+0x1e20439>
141e2149c:     	orb	$0x4, -0x1(%rcx,%rsi)
141e214a1:     	movq	0x710(%rbp), %rax
141e214a8:     	movq	%rax, 0x2520(%rbp)
141e214af:     	movups	0x700(%rbp), %xmm0
141e214b6:     	movaps	%xmm0, 0x2510(%rbp)
141e214bd:     	leaq	0x1166ddc(%rip), %rcx   # 0x142f882a0
141e214c4:     	movl	$0xd, %edx
141e214c9:     	callq	0x140b9d480 <.text+0xb9c480>
141e214ce:     	movq	%rdx, %r9
141e214d1:     	leaq	0x700(%rbp), %rcx
141e214d8:     	leaq	0x2510(%rbp), %rdx
141e214df:     	movq	%rax, %r8
141e214e2:     	callq	0x14068bfc0 <.text+0x68afc0>
141e214e7:     	leaq	0x700(%rbp), %rcx
141e214ee:     	movb	$0x1, %dl
141e214f0:     	callq	0x140f9dba0 <.text+0xf9cba0>
141e214f5:     	movl	$0x7b5a51a0, 0x258(%rax) # imm = 0x7B5A51A0
141e214ff:     	movq	0x710(%rbp), %rax
141e21506:     	movq	%rax, 0x1cf0(%rbp)
141e2150d:     	movups	0x700(%rbp), %xmm0
141e21514:     	movaps	%xmm0, 0x1ce0(%rbp)
141e2151b:     	leaq	0x1ce0(%rbp), %rcx
141e21522:     	movb	$0x1, %dl
141e21524:     	callq	0x140f9dba0 <.text+0xf9cba0>
141e21529:     	movb	$0x12, 0x25d(%rax)
141e21530:     	movdqa	0x1ce0(%rbp), %xmm0
141e21538:     	movdqa	%xmm0, 0x700(%rbp)
141e21540:     	movq	0x1cf0(%rbp), %rax
141e21547:     	movq	%rax, 0x710(%rbp)
141e2154e:     	movq	0x700(%rbp), %rsi
141e21555:     	cmpq	$0x0, 0x1b8(%rsi)
141e2155d:     	je	0x141e21af7 <.text+0x1e20af7>
141e21563:     	leaq	0x708(%rbp), %rdx
141e2156a:     	leaq	0x1c0(%rsi), %rcx
141e21571:     	callq	0x1405b0d70 <.text+0x5afd70>
141e21576:     	movq	0x1a0(%rsi), %rcx
141e2157d:     	movq	0x1a8(%rsi), %rdx
141e21584:     	movq	%rdx, %r8
141e21587:     	andq	%rax, %r8
141e2158a:     	shrq	$0x39, %rax
141e2158e:     	movd	%eax, %xmm0
141e21592:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
141e21596:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
141e2159b:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
141e215a0:     	leaq	-0xc0(%rcx), %rax
141e215a7:     	movq	0x708(%rbp), %r9
141e215ae:     	xorl	%r10d, %r10d
141e215b1:     	pcmpeqd	%xmm1, %xmm1
141e215b5:     	movq	0x2700(%rbp), %rdi
141e215bc:     	movdqu	(%rcx,%r8), %xmm2
141e215c2:     	movdqa	%xmm2, %xmm3
141e215c6:     	pcmpeqb	%xmm0, %xmm3
141e215ca:     	pmovmskb	%xmm3, %r11d
141e215cf:     	testl	%r11d, %r11d
141e215d2:     	je	0x141e215fd <.text+0x1e205fd>
141e215d4:     	tzcntl	%r11d, %esi
141e215d9:     	addq	%r8, %rsi
141e215dc:     	andq	%rdx, %rsi
141e215df:     	negq	%rsi
141e215e2:     	leaq	(%rsi,%rsi,2), %rsi
141e215e6:     	shlq	$0x6, %rsi
141e215ea:     	cmpq	(%rax,%rsi), %r9
141e215ee:     	je	0x141e2161f <.text+0x1e2061f>
141e215f0:     	leal	-0x1(%r11), %esi
141e215f4:     	andw	%r11w, %si
141e215f8:     	movl	%esi, %r11d
141e215fb:     	jne	0x141e215d4 <.text+0x1e205d4>
141e215fd:     	pcmpeqb	%xmm1, %xmm2
141e21601:     	pmovmskb	%xmm2, %r11d
141e21606:     	testl	%r11d, %r11d
141e21609:     	jne	0x141e21af7 <.text+0x1e20af7>
141e2160f:     	addq	%r10, %r8
141e21612:     	addq	$0x10, %r8
141e21616:     	addq	$0x10, %r10
141e2161a:     	andq	%rdx, %r8
141e2161d:     	jmp	0x141e215bc <.text+0x1e205bc>
141e2161f:     	testb	$0x30, -0x2(%rcx,%rsi)
141e21624:     	je	0x141e2162a <.text+0x1e2062a>
141e21626:     	movb	$0x0, 0x38(%rdi)
141e2162a:     	leaq	0x1166c7c(%rip), %rcx   # 0x142f882ad
141e21631:     	movl	$0xc, %edx
141e21636:     	callq	0x140b9d480 <.text+0xb9c480>
141e2163b:     	movq	%rdx, %r9
141e2163e:     	leaq	0x700(%rbp), %rcx
141e21645:     	leaq	0x2510(%rbp), %rdx
141e2164c:     	movq	%rax, %r8
141e2164f:     	callq	0x14068bfc0 <.text+0x68afc0>
141e21654:     	leaq	0x700(%rbp), %rcx
141e2165b:     	movb	$0x1, %dl
141e2165d:     	callq	0x140f9dba0 <.text+0xf9cba0>
141e21662:     	movl	$0x7b5a51a0, 0x258(%rax) # imm = 0x7B5A51A0
141e2166c:     	movdqu	0x700(%rbp), %xmm0
141e21674:     	movdqa	%xmm0, 0x1ce0(%rbp)
141e2167c:     	movq	0x710(%rbp), %rax
141e21683:     	movq	%rax, 0x1cf0(%rbp)
141e2168a:     	movq	0x1ce0(%rbp), %rsi
141e21691:     	cmpq	$0x0, 0x1b8(%rsi)
141e21699:     	je	0x141e22635 <.text+0x1e21635>
141e2169f:     	leaq	0x1ce8(%rbp), %rdx
141e216a6:     	leaq	0x1c0(%rsi), %rcx
141e216ad:     	callq	0x1405b0d70 <.text+0x5afd70>
141e216b2:     	movq	0x1a0(%rsi), %rcx
141e216b9:     	movq	0x1a8(%rsi), %rdx
141e216c0:     	movq	%rdx, %r8
141e216c3:     	andq	%rax, %r8
141e216c6:     	shrq	$0x39, %rax
141e216ca:     	movd	%eax, %xmm0
141e216ce:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
141e216d2:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
141e216d7:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
141e216dc:     	leaq	-0xc0(%rcx), %rax
141e216e3:     	movq	0x1ce8(%rbp), %r9
141e216ea:     	xorl	%r10d, %r10d
141e216ed:     	pcmpeqd	%xmm1, %xmm1
141e216f1:     	movq	0x2700(%rbp), %rdi
141e216f8:     	movdqu	(%rcx,%r8), %xmm2
141e216fe:     	movdqa	%xmm2, %xmm3
141e21702:     	pcmpeqb	%xmm0, %xmm3
141e21706:     	pmovmskb	%xmm3, %r11d
141e2170b:     	testl	%r11d, %r11d
141e2170e:     	je	0x141e21739 <.text+0x1e20739>
141e21710:     	tzcntl	%r11d, %esi
141e21715:     	addq	%r8, %rsi
141e21718:     	andq	%rdx, %rsi
141e2171b:     	negq	%rsi
141e2171e:     	leaq	(%rsi,%rsi,2), %rsi
141e21722:     	shlq	$0x6, %rsi
141e21726:     	cmpq	(%rax,%rsi), %r9
141e2172a:     	je	0x141e2176c <.text+0x1e2076c>
141e2172c:     	leal	-0x1(%r11), %esi
141e21730:     	andw	%r11w, %si
141e21734:     	movl	%esi, %r11d
141e21737:     	jne	0x141e21710 <.text+0x1e20710>
141e21739:     	pcmpeqb	%xmm1, %xmm2
141e2173d:     	pmovmskb	%xmm2, %r11d
141e21742:     	testl	%r11d, %r11d
141e21745:     	jne	0x141e22635 <.text+0x1e21635>
141e2174b:     	addq	%r10, %r8
141e2174e:     	addq	$0x10, %r8
141e21752:     	addq	$0x10, %r10
141e21756:     	andq	%rdx, %r8
141e21759:     	jmp	0x141e216f8 <.text+0x1e206f8>
141e2175b:     	leaq	0xbf6816(%rip), %rcx    # 0x142a17f78
141e21762:     	callq	0x1428d9310 <.text+0x28d8310>
141e21767:     	jmp	0x141e22ef7 <.text+0x1e21ef7>
141e2176c:     	testb	$0x30, -0x2(%rcx,%rsi)
141e21771:     	je	0x141e217d4 <.text+0x1e207d4>
141e21773:     	movq	0x2818(%rbp), %rdx
141e2177a:     	movl	0x1c(%rdx), %eax
141e2177d:     	movq	0x10(%rdx), %rcx
141e21781:     	movl	%eax, (%rcx)
141e21783:     	movq	(%rdx), %rsi
141e21786:     	movq	0x30(%rsi), %rdi
141e2178a:     	movq	0x40(%rsi), %rbx
141e2178e:     	cmpq	0x20(%rsi), %rdi
141e21792:     	jne	0x141e217a4 <.text+0x1e207a4>
141e21794:     	leaq	0x20(%rsi), %rcx
141e21798:     	leaq	0xd93ad1(%rip), %rdx    # 0x142bb5270
141e2179f:     	callq	0x140562db0 <.text+0x561db0>
141e217a4:     	movq	0x28(%rsi), %rax
141e217a8:     	leaq	(%rdi,%rdi,2), %rcx
141e217ac:     	movdqa	0x2440(%rbp), %xmm0
141e217b4:     	movdqu	%xmm0, (%rax,%rcx,8)
141e217b9:     	movq	%rbx, 0x10(%rax,%rcx,8)
141e217be:     	incq	%rdi
141e217c1:     	movq	%rdi, 0x30(%rsi)
141e217c5:     	incq	0x40(%rsi)
141e217c9:     	movq	0x2700(%rbp), %rdi
141e217d0:     	movb	$0x0, 0x38(%rdi)
141e217d4:     	cmpb	$0x0, 0x49(%rdi)
141e217d8:     	je	0x141e227f8 <.text+0x1e217f8>
141e217de:     	movdqu	0x4a(%rdi), %xmm0
141e217e3:     	movdqa	%xmm0, 0x2500(%rbp)
141e217eb:     	movl	0x2714(%rbp), %eax
141e217f1:     	movq	0x26e8(%rbp), %rcx
141e217f8:     	movl	%eax, (%rcx)
141e217fa:     	leaq	0x1166ab8(%rip), %rax   # 0x142f882b9
141e21801:     	movq	%rax, 0x700(%rbp)
141e21808:     	movq	$0x16, 0x708(%rbp)
141e21813:     	leaq	0x700(%rbp), %rcx
141e2181a:     	callq	0x1405bdda0 <.text+0x5bcda0>
141e2181f:     	movq	%rax, %rdi
141e21822:     	movabsq	$-0x4e96acdfffb04bbd, %rax # imm = 0xB1695320004FB443
141e2182c:     	xorq	%rax, %rdi
141e2182f:     	movq	0x2728(%rbp), %rcx
141e21836:     	movq	%rdi, %rdx
141e21839:     	callq	0x140683ec0 <.text+0x682ec0>
141e2183e:     	movb	$0x0, 0x5d(%rax)
141e21842:     	movb	$0x1, 0xb7(%rax)
141e21849:     	testb	$0x1, %dl
141e2184c:     	jne	0x141e21893 <.text+0x1e20893>
141e2184e:     	movq	0x2728(%rbp), %rax
141e21855:     	movq	0x10(%rax), %rsi
141e21859:     	cmpq	(%rax), %rsi
141e2185c:     	jne	0x141e21871 <.text+0x1e20871>
141e2185e:     	leaq	0xd8d763(%rip), %rdx    # 0x142baefc8
141e21865:     	movq	0x2728(%rbp), %rcx
141e2186c:     	callq	0x1406319d0 <.text+0x6309d0>
141e21871:     	movq	0x2728(%rbp), %rdx
141e21878:     	movq	0x8(%rdx), %rax
141e2187c:     	imulq	$0x38, %rsi, %rcx
141e21880:     	movl	$0x1, (%rax,%rcx)
141e21887:     	movq	%rdi, 0x30(%rax,%rcx)
141e2188c:     	incq	%rsi
141e2188f:     	movq	%rsi, 0x10(%rdx)
141e21893:     	movq	0x2728(%rbp), %rax
141e2189a:     	movq	%rax, 0x2450(%rbp)
141e218a1:     	movq	%rdi, 0x2458(%rbp)
141e218a8:     	movq	$0x0, 0x2460(%rbp)
141e218b3:     	movq	$0x0, 0x700(%rbp)
141e218be:     	movabsq	$0x41223169ff28813b, %rsi # imm = 0x41223169FF28813B
141e218c8:     	movq	%rsi, 0x708(%rbp)
141e218cf:     	movabsq	$-0x58648d975d569727, %rbx # imm = 0xA79B7268A2A968D9
141e218d9:     	movq	%rbx, 0x710(%rbp)
141e218e0:     	leaq	0x700(%rbp), %rcx
141e218e7:     	callq	0x1405bdf20 <.text+0x5bcf20>
141e218ec:     	movq	%rdi, 0x700(%rbp)
141e218f3:     	movq	%rax, 0x708(%rbp)
141e218fa:     	leaq	0x700(%rbp), %rcx
141e21901:     	callq	0x1405bd910 <.text+0x5bc910>
141e21906:     	leaq	0x2428(%rbp), %rcx
141e2190d:     	leaq	0x2450(%rbp), %rdx
141e21914:     	movq	%rax, %r8
141e21917:     	callq	0x1406861b0 <.text+0x6851b0>
141e2191c:     	movq	0x2430(%rbp), %rdi
141e21923:     	movq	$0x0, 0x700(%rbp)
141e2192e:     	movq	%rsi, 0x708(%rbp)
141e21935:     	movq	%rbx, 0x710(%rbp)
141e2193c:     	leaq	0x700(%rbp), %rcx
141e21943:     	callq	0x1405bdf20 <.text+0x5bcf20>
141e21948:     	movq	%rdi, 0x700(%rbp)
141e2194f:     	movq	%rax, 0x708(%rbp)
141e21956:     	leaq	0x700(%rbp), %rcx
141e2195d:     	callq	0x1405bd910 <.text+0x5bc910>
141e21962:     	leaq	0x700(%rbp), %rcx
141e21969:     	leaq	0x2428(%rbp), %rdx
141e21970:     	movq	%rax, %r8
141e21973:     	callq	0x1406861b0 <.text+0x6851b0>
141e21978:     	movq	$0x0, 0x20(%rsp)
141e21981:     	leaq	0x1166947(%rip), %r8    # 0x142f882cf
141e21988:     	leaq	0x1c90(%rbp), %rcx
141e2198f:     	leaq	0x700(%rbp), %rdx
141e21996:     	movl	$0x11, %r9d
141e2199c:     	callq	0x140f9e970 <.text+0xf9d970>
141e219a1:     	leaq	0x11668f2(%rip), %r8    # 0x142f8829a
141e219a8:     	leaq	0x700(%rbp), %rcx
141e219af:     	leaq	0x1c90(%rbp), %rdx
141e219b6:     	movl	$0x6, %r9d
141e219bc:     	callq	0x140fb8070 <.text+0xfb7070>
141e219c1:     	movq	0x700(%rbp), %rsi
141e219c8:     	cmpq	$0x0, 0x1b8(%rsi)
141e219d0:     	je	0x141e21b14 <.text+0x1e20b14>
141e219d6:     	leaq	0x708(%rbp), %rdx
141e219dd:     	leaq	0x1c0(%rsi), %rcx
141e219e4:     	callq	0x1405b0d70 <.text+0x5afd70>
141e219e9:     	movq	0x1a0(%rsi), %rcx
141e219f0:     	movq	0x1a8(%rsi), %rdx
141e219f7:     	movq	%rdx, %r8
141e219fa:     	andq	%rax, %r8
141e219fd:     	shrq	$0x39, %rax
141e21a01:     	movd	%eax, %xmm0
141e21a05:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
141e21a09:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
141e21a0e:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
141e21a13:     	leaq	-0xc0(%rcx), %rax
141e21a1a:     	movq	0x708(%rbp), %r9
141e21a21:     	xorl	%r10d, %r10d
141e21a24:     	pcmpeqd	%xmm1, %xmm1
141e21a28:     	movdqu	(%rcx,%r8), %xmm2
141e21a2e:     	movdqa	%xmm2, %xmm3
141e21a32:     	pcmpeqb	%xmm0, %xmm3
141e21a36:     	pmovmskb	%xmm3, %r11d
141e21a3b:     	testl	%r11d, %r11d
141e21a3e:     	je	0x141e21a69 <.text+0x1e20a69>
141e21a40:     	tzcntl	%r11d, %esi
141e21a45:     	addq	%r8, %rsi
141e21a48:     	andq	%rdx, %rsi
141e21a4b:     	negq	%rsi
141e21a4e:     	leaq	(%rsi,%rsi,2), %rsi
141e21a52:     	shlq	$0x6, %rsi
141e21a56:     	cmpq	(%rax,%rsi), %r9
141e21a5a:     	je	0x141e21a8b <.text+0x1e20a8b>
141e21a5c:     	leal	-0x1(%r11), %esi
141e21a60:     	andw	%r11w, %si
141e21a64:     	movl	%esi, %r11d
141e21a67:     	jne	0x141e21a40 <.text+0x1e20a40>
141e21a69:     	pcmpeqb	%xmm1, %xmm2
141e21a6d:     	pmovmskb	%xmm2, %r11d
141e21a72:     	testl	%r11d, %r11d
141e21a75:     	jne	0x141e21b14 <.text+0x1e20b14>
141e21a7b:     	addq	%r10, %r8
141e21a7e:     	addq	$0x10, %r8
141e21a82:     	addq	$0x10, %r10
141e21a86:     	andq	%rdx, %r8
141e21a89:     	jmp	0x141e21a28 <.text+0x1e20a28>
141e21a8b:     	orb	$0x4, -0x1(%rcx,%rsi)
141e21a90:     	movq	0x710(%rbp), %rax
141e21a97:     	movq	%rax, 0x25d0(%rbp)
141e21a9e:     	movups	0x700(%rbp), %xmm0
141e21aa5:     	movaps	%xmm0, 0x25c0(%rbp)
141e21aac:     	movdqa	0x2500(%rbp), %xmm0
141e21ab4:     	movdqa	%xmm0, 0x1ce0(%rbp)
141e21abc:     	leaq	0x700(%rbp), %rcx
141e21ac3:     	leaq	0x1ce0(%rbp), %rdx
141e21aca:     	callq	0x141e23c00 <.text+0x1e22c00>
141e21acf:     	xorl	%eax, %eax
141e21ad1:     	cmpq	0x700(%rbp), %rax
141e21ad8:     	jno	0x141e21b25 <.text+0x1e20b25>
141e21ada:     	leaq	0x2610(%rbp), %rcx
141e21ae1:     	callq	0x140b468f0 <.text+0xb458f0>
141e21ae6:     	leaq	0x708(%rbp), %rcx
141e21aed:     	callq	0x140d4cc30 <.text+0xd4bc30>
141e21af2:     	jmp	0x141e21b81 <.text+0x1e20b81>
141e21af7:     	leaq	0xbf570a(%rip), %rcx    # 0x142a17208
141e21afe:     	leaq	0xbf64d3(%rip), %r8     # 0x142a17fd8
141e21b05:     	movl	$0x16, %edx
141e21b0a:     	callq	0x1428d9330 <.text+0x28d8330>
141e21b0f:     	jmp	0x141e22ef7 <.text+0x1e21ef7>
141e21b14:     	leaq	0xbf645d(%rip), %rcx    # 0x142a17f78
141e21b1b:     	callq	0x1428d9310 <.text+0x28d8310>
141e21b20:     	jmp	0x141e22ef7 <.text+0x1e21ef7>
141e21b25:     	movups	0x750(%rbp), %xmm0
141e21b2c:     	movaps	%xmm0, 0x2660(%rbp)
141e21b33:     	movups	0x740(%rbp), %xmm0
141e21b3a:     	movaps	%xmm0, 0x2650(%rbp)
141e21b41:     	movdqu	0x700(%rbp), %xmm0
141e21b49:     	movdqu	0x710(%rbp), %xmm1
141e21b51:     	movdqu	0x720(%rbp), %xmm2
141e21b59:     	movdqu	0x730(%rbp), %xmm3
141e21b61:     	movdqa	%xmm3, 0x2640(%rbp)
141e21b69:     	movdqa	%xmm2, 0x2630(%rbp)
141e21b71:     	movdqa	%xmm1, 0x2620(%rbp)
141e21b79:     	movdqa	%xmm0, 0x2610(%rbp)
141e21b81:     	movzbl	0x2668(%rbp), %eax
141e21b88:     	movb	%al, 0x26f8(%rbp)
141e21b8e:     	leaq	0x116674b(%rip), %rcx   # 0x142f882e0
141e21b95:     	movl	$0x10, %edx
141e21b9a:     	callq	0x140b9d480 <.text+0xb9c480>
141e21b9f:     	movq	%rax, 0x2708(%rbp)
141e21ba6:     	movq	%rdx, %r14
141e21ba9:     	leaq	0x1166740(%rip), %rcx   # 0x142f882f0
141e21bb0:     	movl	$0x10, %edx
141e21bb5:     	callq	0x140b9d480 <.text+0xb9c480>
141e21bba:     	movq	%rax, %r15
141e21bbd:     	movq	%rdx, %r13
141e21bc0:     	leaq	0x1166739(%rip), %rcx   # 0x142f88300
141e21bc7:     	movl	$0xf, %edx
141e21bcc:     	callq	0x140b9d480 <.text+0xb9c480>
141e21bd1:     	movq	%rax, %r12
141e21bd4:     	movq	%rdx, %rsi
141e21bd7:     	leaq	0x1166731(%rip), %rcx   # 0x142f8830f
141e21bde:     	movl	$0x11, %edx
141e21be3:     	callq	0x140b9d480 <.text+0xb9c480>
141e21be8:     	movq	%rax, %rbx
141e21beb:     	movq	%rdx, %rdi
141e21bee:     	leaq	0x116672b(%rip), %rcx   # 0x142f88320
141e21bf5:     	movl	$0x10, %edx
141e21bfa:     	callq	0x140b9d480 <.text+0xb9c480>
141e21bff:     	movq	$0x0, 0x2510(%rbp)
141e21c0a:     	movq	$0x5, 0x2518(%rbp)
141e21c15:     	leaq	0x2520(%rbp), %rcx
141e21c1c:     	movq	0x2708(%rbp), %r8
141e21c23:     	movq	%r8, 0x2520(%rbp)
141e21c2a:     	movq	%r14, 0x2528(%rbp)
141e21c31:     	movb	$0x0, 0x2530(%rbp)
141e21c38:     	movq	%r15, 0x2538(%rbp)
141e21c3f:     	movq	%r13, 0x2540(%rbp)
141e21c46:     	movb	$0x3, 0x2548(%rbp)
141e21c4d:     	movq	%r12, 0x2550(%rbp)
141e21c54:     	movq	%rsi, 0x2558(%rbp)
141e21c5b:     	movb	$0x1, 0x2560(%rbp)
141e21c62:     	movq	%rbx, 0x2568(%rbp)
141e21c69:     	movq	%rcx, %rbx
141e21c6c:     	movq	%rdi, 0x2570(%rbp)
141e21c73:     	movb	$0x2, 0x2578(%rbp)
141e21c7a:     	movq	%rax, 0x2580(%rbp)
141e21c81:     	movq	%rdx, 0x2588(%rbp)
141e21c88:     	movb	$0x4, 0x2590(%rbp)
141e21c8f:     	leaq	0x2698(%rbp), %r12
141e21c96:     	movq	0x2810(%rbp), %rax
141e21c9d:     	movl	0x1c(%rax), %ecx
141e21ca0:     	movl	%ecx, 0x26b8(%rbp)
141e21ca6:     	movq	(%rax), %rcx
141e21ca9:     	movq	0x10(%rax), %rax
141e21cad:     	movq	%rax, 0x2678(%rbp)
141e21cb4:     	movq	%rcx, 0x26f0(%rbp)
141e21cbb:     	leaq	0x20(%rcx), %rax
141e21cbf:     	movq	%rax, 0x2680(%rbp)
141e21cc6:     	pcmpeqd	%xmm6, %xmm6
141e21cca:     	leaq	-0x1d17e41(%rip), %r14  # 0x140109e90 <.text+0x108e90>
141e21cd1:     	leaq	0x11666d8(%rip), %r15   # 0x142f883b0
141e21cd8:     	xorl	%r13d, %r13d
141e21cdb:     	movq	%r13, %rax
141e21cde:     	cmpq	$0x5, %r13
141e21ce2:     	je	0x141e22652 <.text+0x1e21652>
141e21ce8:     	leaq	0x1(%rax), %r13
141e21cec:     	movq	%r13, 0x2510(%rbp)
141e21cf3:     	leaq	(%rax,%rax,2), %rax
141e21cf7:     	movzbl	0x10(%rbx,%rax,8), %ecx
141e21cfc:     	cmpb	$0x5, %cl
141e21cff:     	je	0x141e22652 <.text+0x1e21652>
141e21d05:     	movq	(%rbx,%rax,8), %rsi
141e21d09:     	movq	0x8(%rbx,%rax,8), %rdi
141e21d0e:     	movq	%rsi, 0x26c0(%rbp)
141e21d15:     	movq	%rdi, 0x26c8(%rbp)
141e21d1c:     	movb	%cl, 0x2708(%rbp)
141e21d22:     	cmpb	%cl, 0x26f8(%rbp)
141e21d28:     	jne	0x141e21d98 <.text+0x1e20d98>
141e21d2a:     	leaq	0x26c0(%rbp), %rax
141e21d31:     	movq	%rax, 0x25e0(%rbp)
141e21d38:     	movq	%r14, 0x25e8(%rbp)
141e21d3f:     	movq	%r15, 0x700(%rbp)
141e21d46:     	movq	$0x2, 0x708(%rbp)
141e21d51:     	leaq	0x25e0(%rbp), %rax
141e21d58:     	movq	%rax, 0x710(%rbp)
141e21d5f:     	movq	$0x1, 0x718(%rbp)
141e21d6a:     	movq	$0x0, 0x720(%rbp)
141e21d75:     	leaq	0x1ce0(%rbp), %rcx
141e21d7c:     	leaq	0x700(%rbp), %rdx
141e21d83:     	callq	0x140d79f90 <.text+0xd78f90>
141e21d88:     	movq	0x1ce8(%rbp), %r8
141e21d8f:     	movq	0x1cf0(%rbp), %rdi
141e21d96:     	jmp	0x141e21e06 <.text+0x1e20e06>
141e21d98:     	testq	%rdi, %rdi
141e21d9b:     	js	0x141e22e90 <.text+0x1e21e90>
141e21da1:     	movq	%r13, 0x2718(%rbp)
141e21da8:     	je	0x141e21dcc <.text+0x1e20dcc>
141e21daa:     	movzbl	0x1812e58(%rip), %eax   # 0x143634c09
141e21db1:     	movl	$0x1, %edx
141e21db6:     	movq	%rdi, %rcx
141e21db9:     	callq	0x140613c10 <.text+0x612c10>
141e21dbe:     	testq	%rax, %rax
141e21dc1:     	je	0x141e22e94 <.text+0x1e21e94>
141e21dc7:     	movq	%rax, %r13
141e21dca:     	jmp	0x141e21dd2 <.text+0x1e20dd2>
141e21dcc:     	movl	$0x1, %r13d
141e21dd2:     	movq	%r13, %rcx
141e21dd5:     	movq	%rsi, %rdx
141e21dd8:     	movq	%rdi, %r8
141e21ddb:     	callq	0x142923c00 <.text+0x2922c00>
141e21de0:     	movq	%rdi, 0x1ce0(%rbp)
141e21de7:     	movq	%r13, 0x1ce8(%rbp)
141e21dee:     	movq	%rdi, 0x1cf0(%rbp)
141e21df5:     	movq	%r13, 0x2670(%rbp)
141e21dfc:     	movq	%r13, %r8
141e21dff:     	movq	0x2718(%rbp), %r13
141e21e06:     	leaq	0x700(%rbp), %rcx
141e21e0d:     	leaq	0x25c0(%rbp), %rdx
141e21e14:     	movq	%r8, 0x2718(%rbp)
141e21e1b:     	movq	%rdi, %r9
141e21e1e:     	callq	0x14068bfc0 <.text+0x68afc0>
141e21e23:     	leaq	0x700(%rbp), %rcx
141e21e2a:     	movb	$0x1, %dl
141e21e2c:     	callq	0x140f9dba0 <.text+0xf9cba0>
141e21e31:     	movzbl	0x2708(%rbp), %ecx
141e21e38:     	cmpb	%cl, 0x26f8(%rbp)
141e21e3e:     	movl	$0x7b5a51a0, %ecx       # imm = 0x7B5A51A0
141e21e43:     	movl	$0xc7b7a7ff, %edx       # imm = 0xC7B7A7FF
141e21e48:     	cmovel	%edx, %ecx
141e21e4b:     	movl	%ecx, 0x258(%rax)
141e21e51:     	movdqu	0x700(%rbp), %xmm0
141e21e59:     	movdqa	%xmm0, 0x2690(%rbp)
141e21e61:     	movq	0x710(%rbp), %rax
141e21e68:     	movq	%rax, 0x26a0(%rbp)
141e21e6f:     	movq	0x2690(%rbp), %rsi
141e21e76:     	cmpq	$0x0, 0x1b8(%rsi)
141e21e7e:     	je	0x141e22618 <.text+0x1e21618>
141e21e84:     	leaq	0x1c0(%rsi), %rcx
141e21e8b:     	movq	%r12, %rdx
141e21e8e:     	callq	0x1405b0d70 <.text+0x5afd70>
141e21e93:     	movq	0x1a0(%rsi), %rcx
141e21e9a:     	movq	0x1a8(%rsi), %rdx
141e21ea1:     	movq	%rdx, %r8
141e21ea4:     	andq	%rax, %r8
141e21ea7:     	shrq	$0x39, %rax
141e21eab:     	movd	%eax, %xmm0
141e21eaf:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
141e21eb3:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
141e21eb8:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
141e21ebd:     	leaq	-0xc0(%rcx), %rax
141e21ec4:     	movq	0x2698(%rbp), %r9
141e21ecb:     	xorl	%r10d, %r10d
141e21ece:     	movdqu	(%rcx,%r8), %xmm1
141e21ed4:     	movdqa	%xmm1, %xmm2
141e21ed8:     	pcmpeqb	%xmm0, %xmm2
141e21edc:     	pmovmskb	%xmm2, %r11d
141e21ee1:     	testl	%r11d, %r11d
141e21ee4:     	je	0x141e21f0f <.text+0x1e20f0f>
141e21ee6:     	tzcntl	%r11d, %esi
141e21eeb:     	addq	%r8, %rsi
141e21eee:     	andq	%rdx, %rsi
141e21ef1:     	negq	%rsi
141e21ef4:     	leaq	(%rsi,%rsi,2), %rsi
141e21ef8:     	shlq	$0x6, %rsi
141e21efc:     	cmpq	(%rax,%rsi), %r9
141e21f00:     	je	0x141e21f31 <.text+0x1e20f31>
141e21f02:     	leal	-0x1(%r11), %esi
141e21f06:     	andw	%r11w, %si
141e21f0a:     	movl	%esi, %r11d
141e21f0d:     	jne	0x141e21ee6 <.text+0x1e20ee6>
141e21f0f:     	pcmpeqb	%xmm6, %xmm1
141e21f13:     	pmovmskb	%xmm1, %r11d
141e21f18:     	testl	%r11d, %r11d
141e21f1b:     	jne	0x141e22618 <.text+0x1e21618>
141e21f21:     	addq	%r10, %r8
141e21f24:     	addq	$0x10, %r8
141e21f28:     	addq	$0x10, %r10
141e21f2c:     	andq	%rdx, %r8
141e21f2f:     	jmp	0x141e21ece <.text+0x1e20ece>
141e21f31:     	testb	$0x30, -0x2(%rcx,%rsi)
141e21f36:     	jne	0x141e21f5f <.text+0x1e20f5f>
141e21f38:     	movq	0x1ce0(%rbp), %rdx
141e21f3f:     	testq	%rdx, %rdx
141e21f42:     	je	0x141e21cdb <.text+0x1e20cdb>
141e21f48:     	movl	$0x1, %r8d
141e21f4e:     	movq	0x2718(%rbp), %rcx
141e21f55:     	callq	0x140613c20 <.text+0x612c20>
141e21f5a:     	jmp	0x141e21cdb <.text+0x1e20cdb>
141e21f5f:     	movq	0x1ce0(%rbp), %rdx
141e21f66:     	testq	%rdx, %rdx
141e21f69:     	je	0x141e21f7d <.text+0x1e20f7d>
141e21f6b:     	movl	$0x1, %r8d
141e21f71:     	movq	0x2718(%rbp), %rcx
141e21f78:     	callq	0x140613c20 <.text+0x612c20>
141e21f7d:     	movq	0x2700(%rbp), %rax
141e21f84:     	movb	$0x0, 0x49(%rax)
141e21f88:     	leaq	0x700(%rbp), %rcx
141e21f8f:     	leaq	0x2610(%rbp), %rdx
141e21f96:     	callq	0x140b46370 <.text+0xb45370>
141e21f9b:     	leaq	0x25e0(%rbp), %rcx
141e21fa2:     	leaq	0x700(%rbp), %rdx
141e21fa9:     	callq	0x1408bbbc0 <.text+0x8babc0>
141e21fae:     	leaq	0x700(%rbp), %rcx
141e21fb5:     	leaq	0x2500(%rbp), %rdx
141e21fbc:     	callq	0x140b46000 <.text+0xb45000>
141e21fc1:     	leaq	0x700(%rbp), %rcx
141e21fc8:     	callq	0x140d76ed0 <.text+0xd75ed0>
141e21fcd:     	movq	%rdx, %r8
141e21fd0:     	movq	$0xc, 0x20(%rsp)
141e21fd9:     	leaq	0x26c0(%rbp), %rcx
141e21fe0:     	movq	%rax, %rdx
141e21fe3:     	leaq	0x11661ff(%rip), %r9    # 0x142f881e9
141e21fea:     	callq	0x140d62810 <.text+0xd61810>
141e21fef:     	movq	0x700(%rbp), %rdx
141e21ff6:     	testq	%rdx, %rdx
141e21ff9:     	movzbl	0x2708(%rbp), %esi
141e22000:     	je	0x141e22014 <.text+0x1e21014>
141e22002:     	movq	0x708(%rbp), %rcx
141e22009:     	movl	$0x1, %r8d
141e2200f:     	callq	0x140613c20 <.text+0x612c20>
141e22014:     	movb	$0x1, 0x2736(%rbp)
141e2201b:     	leaq	0x26c0(%rbp), %rcx
141e22022:     	callq	0x140d76ed0 <.text+0xd75ed0>
141e22027:     	movq	%rdx, %r8
141e2202a:     	movb	$0x1, 0x2736(%rbp)
141e22031:     	leaq	0x24f0(%rbp), %rcx
141e22038:     	movq	%rax, %rdx
141e2203b:     	leaq	0x25e0(%rbp), %r9
141e22042:     	callq	0x141470d00 <.text+0x146fd00>
141e22047:     	cmpb	$0x2, 0x24ff(%rbp)
141e2204e:     	jne	0x141e221c3 <.text+0x1e211c3>
141e22054:     	movq	0x24f0(%rbp), %rax
141e2205b:     	movq	%rax, 0x25b0(%rbp)
141e22062:     	movq	0x26c0(%rbp), %rax
141e22069:     	movq	%rax, 0x2708(%rbp)
141e22070:     	movq	0x26c8(%rbp), %rax
141e22077:     	movq	%rax, 0x2718(%rbp)
141e2207e:     	leaq	0x26d0(%rbp), %rax
141e22085:     	movdqu	(%rax), %xmm0
141e22089:     	movdqa	%xmm0, 0x2690(%rbp)
141e22091:     	callq	0x140d635e0 <.text+0xd625e0>
141e22096:     	movl	$0x2, %r8d
141e2209c:     	movq	%rax, %rcx
141e2209f:     	xorl	%r9d, %r9d
141e220a2:     	callq	0x140d63a20 <.text+0xd62a20>
141e220a7:     	movq	%rax, 0x2688(%rbp)
141e220ae:     	movl	%edx, %edi
141e220b0:     	leaq	0x25b0(%rbp), %rax
141e220b7:     	movq	%rax, 0x1ce0(%rbp)
141e220be:     	leaq	-0x10d54a5(%rip), %rax  # 0x140d4cc20 <.text+0xd4bc20>
141e220c5:     	movq	%rax, 0x1ce8(%rbp)
141e220cc:     	leaq	0x116613d(%rip), %rax   # 0x142f88210
141e220d3:     	movq	%rax, 0x700(%rbp)
141e220da:     	movq	$0x1, 0x708(%rbp)
141e220e5:     	leaq	0x1ce0(%rbp), %rax
141e220ec:     	movq	%rax, 0x710(%rbp)
141e220f3:     	movq	$0x1, 0x718(%rbp)
141e220fe:     	movq	$0x0, 0x720(%rbp)
141e22109:     	leaq	0x2410(%rbp), %rcx
141e22110:     	leaq	0x700(%rbp), %rdx
141e22117:     	callq	0x140d79f90 <.text+0xd78f90>
141e2211c:     	movaps	0x2690(%rbp), %xmm0
141e22123:     	movaps	%xmm0, 0x700(%rbp)
141e2212a:     	movq	0x2808(%rbp), %rsi
141e22131:     	movq	(%rsi), %rdx
141e22134:     	movq	%rdx, %rax
141e22137:     	negq	%rax
141e2213a:     	jo	0x141e22165 <.text+0x1e21165>
141e2213c:     	jae	0x141e2214d <.text+0x1e2114d>
141e2213e:     	movq	0x8(%rsi), %rcx
141e22142:     	movl	$0x1, %r8d
141e22148:     	callq	0x140613c20 <.text+0x612c20>
141e2214d:     	movq	0x20(%rsi), %rdx
141e22151:     	testq	%rdx, %rdx
141e22154:     	je	0x141e22165 <.text+0x1e21165>
141e22156:     	movq	0x28(%rsi), %rcx
141e2215a:     	movl	$0x1, %r8d
141e22160:     	callq	0x140613c20 <.text+0x612c20>
141e22165:     	movq	0x2708(%rbp), %rax
141e2216c:     	movq	%rax, (%rsi)
141e2216f:     	movq	0x2718(%rbp), %rax
141e22176:     	movq	%rax, 0x8(%rsi)
141e2217a:     	movaps	0x700(%rbp), %xmm0
141e22181:     	movups	%xmm0, 0x10(%rsi)
141e22185:     	movdqu	0x2410(%rbp), %xmm0
141e2218d:     	movdqu	%xmm0, 0x20(%rsi)
141e22192:     	movq	0x2420(%rbp), %rax
141e22199:     	movq	%rax, 0x30(%rsi)
141e2219d:     	movq	0x2688(%rbp), %rax
141e221a4:     	movq	%rax, 0x38(%rsi)
141e221a8:     	movl	%edi, 0x40(%rsi)
141e221ab:     	movb	$0x0, 0x2736(%rbp)
141e221b2:     	leaq	0x25b0(%rbp), %rcx
141e221b9:     	callq	0x140d4cc30 <.text+0xd4bc30>
141e221be:     	jmp	0x141e21cdb <.text+0x1e20cdb>
141e221c3:     	movdqu	0x24f0(%rbp), %xmm0
141e221cb:     	movdqa	%xmm0, 0x25b0(%rbp)
141e221d3:     	movq	0x2620(%rbp), %rcx
141e221da:     	testq	%rcx, %rcx
141e221dd:     	js	0x141e22ecc <.text+0x1e21ecc>
141e221e3:     	movq	%rcx, 0x2718(%rbp)
141e221ea:     	je	0x141e22254 <.text+0x1e21254>
141e221ec:     	movq	0x2618(%rbp), %rdi
141e221f3:     	movzbl	0x1812a0f(%rip), %eax   # 0x143634c09
141e221fa:     	movl	$0x1, %eax
141e221ff:     	movq	%rax, 0x2688(%rbp)
141e22206:     	movl	$0x1, %edx
141e2220b:     	movq	%rcx, %rsi
141e2220e:     	callq	0x140613c10 <.text+0x612c10>
141e22213:     	testq	%rax, %rax
141e22216:     	je	0x141e22eda <.text+0x1e21eda>
141e2221c:     	leaq	-0x1(%rsi), %r10
141e22220:     	cmpq	%r10, %rsi
141e22223:     	cmovbq	%rsi, %r10
141e22227:     	cmpq	$0x20, %r10
141e2222b:     	setb	%cl
141e2222e:     	movq	%rax, %rdx
141e22231:     	subq	%rdi, %rdx
141e22234:     	cmpq	$0x20, %rdx
141e22238:     	setb	%dl
141e2223b:     	orb	%cl, %dl
141e2223d:     	movq	%rsi, %r9
141e22240:     	je	0x141e2225c <.text+0x1e2125c>
141e22242:     	movq	%r9, %rcx
141e22245:     	movq	%rdi, %rdx
141e22248:     	xorl	%r10d, %r10d
141e2224b:     	movzbl	0x2708(%rbp), %esi
141e22252:     	jmp	0x141e222a8 <.text+0x1e212a8>
141e22254:     	movl	$0x1, %r11d
141e2225a:     	jmp	0x141e222d6 <.text+0x1e212d6>
141e2225c:     	incq	%r10
141e2225f:     	movl	%r10d, %ecx
141e22262:     	andl	$0x1f, %ecx
141e22265:     	movl	$0x20, %edx
141e2226a:     	cmoveq	%rdx, %rcx
141e2226e:     	subq	%rcx, %r10
141e22271:     	movq	%r9, %rcx
141e22274:     	subq	%r10, %rcx
141e22277:     	leaq	(%rdi,%r10), %rdx
141e2227b:     	xorl	%r8d, %r8d
141e2227e:     	movzbl	0x2708(%rbp), %esi
141e22285:     	movdqu	(%rdi,%r8), %xmm0
141e2228b:     	movdqu	0x10(%rdi,%r8), %xmm1
141e22292:     	movdqu	%xmm0, (%rax,%r8)
141e22298:     	movdqu	%xmm1, 0x10(%rax,%r8)
141e2229f:     	addq	$0x20, %r8
141e222a3:     	cmpq	%r8, %r10
141e222a6:     	jne	0x141e22285 <.text+0x1e21285>
141e222a8:     	addq	%r9, %rdi
141e222ab:     	movq	%rax, %r11
141e222ae:     	movq	%rax, %r8
141e222b1:     	addq	%r10, %r8
141e222b4:     	xorl	%eax, %eax
141e222b6:     	leaq	(%rdx,%rax), %r9
141e222ba:     	cmpq	%rdi, %r9
141e222bd:     	je	0x141e222cf <.text+0x1e212cf>
141e222bf:     	movzbl	(%r9), %r9d
141e222c3:     	movb	%r9b, (%r8,%rax)
141e222c7:     	incq	%rax
141e222ca:     	cmpq	%rax, %rcx
141e222cd:     	jne	0x141e222b6 <.text+0x1e212b6>
141e222cf:     	movq	0x2718(%rbp), %rcx
141e222d6:     	movzbl	0x26e0(%rbp), %r8d
141e222de:     	movzbl	0x2727(%rbp), %r9d
141e222e6:     	xorl	%eax, %eax
141e222e8:     	cmpq	0x2628(%rbp), %rax
141e222ef:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
141e222f9:     	jo	0x141e223a1 <.text+0x1e213a1>
141e222ff:     	movq	%r11, 0x26e0(%rbp)
141e22306:     	movq	%rbx, %rsi
141e22309:     	movb	0x2650(%rbp), %bl
141e2230f:     	leaq	0x1ce0(%rbp), %rcx
141e22316:     	leaq	0x2628(%rbp), %rdx
141e2231d:     	callq	0x140d7bd90 <.text+0xd7ad90>
141e22322:     	movb	%bl, 0x1d08(%rbp)
141e22328:     	movaps	0x2640(%rbp), %xmm0
141e2232f:     	movups	%xmm0, 0x1cf8(%rbp)
141e22336:     	movsd	0x2658(%rbp), %xmm8
141e2233f:     	movd	0x2660(%rbp), %xmm7
141e22347:     	movzbl	0x2664(%rbp), %r8d
141e2234f:     	movzbl	0x2665(%rbp), %r9d
141e22357:     	movq	0x1ce0(%rbp), %rax
141e2235e:     	leaq	0x1ce8(%rbp), %rcx
141e22365:     	movdqu	(%rcx), %xmm0
141e22369:     	movdqu	0x10(%rcx), %xmm1
141e2236e:     	movdqa	%xmm0, 0x2690(%rbp)
141e22376:     	movdqa	%xmm1, 0x26a0(%rbp)
141e2237e:     	movq	0x20(%rcx), %rcx
141e22382:     	movq	%rcx, 0x26b0(%rbp)
141e22389:     	movq	%rsi, %rbx
141e2238c:     	movzbl	0x2708(%rbp), %esi
141e22393:     	movq	0x2718(%rbp), %rcx
141e2239a:     	movq	0x26e0(%rbp), %r11
141e223a1:     	movb	%sil, 0x758(%rbp)
141e223a8:     	movq	%rcx, 0x700(%rbp)
141e223af:     	movq	%r11, 0x708(%rbp)
141e223b6:     	movq	%rcx, 0x710(%rbp)
141e223bd:     	movq	%rax, 0x718(%rbp)
141e223c4:     	movq	0x26b0(%rbp), %rax
141e223cb:     	leaq	0x720(%rbp), %rcx
141e223d2:     	movq	%rax, 0x20(%rcx)
141e223d6:     	movaps	0x2690(%rbp), %xmm0
141e223dd:     	movdqa	0x26a0(%rbp), %xmm1
141e223e5:     	movdqu	%xmm1, 0x10(%rcx)
141e223ea:     	movups	%xmm0, (%rcx)
141e223ed:     	movlps	%xmm8, 0x748(%rbp)
141e223f5:     	movd	%xmm7, 0x750(%rbp)
141e223fd:     	movb	%r8b, 0x26e0(%rbp)
141e22404:     	movb	%r8b, 0x754(%rbp)
141e2240b:     	movb	%r9b, 0x2727(%rbp)
141e22412:     	movb	%r9b, 0x755(%rbp)
141e22419:     	movdqa	0x2500(%rbp), %xmm0
141e22421:     	movdqa	%xmm0, 0x1ce0(%rbp)
141e22429:     	leaq	0x700(%rbp), %rcx
141e22430:     	leaq	0x1ce0(%rbp), %rdx
141e22437:     	callq	0x140b463c0 <.text+0xb453c0>
141e2243c:     	movq	%rax, 0x1ce0(%rbp)
141e22443:     	testq	%rax, %rax
141e22446:     	je	0x141e22454 <.text+0x1e21454>
141e22448:     	leaq	0x1ce0(%rbp), %rcx
141e2244f:     	callq	0x140d4cc30 <.text+0xd4bc30>
141e22454:     	movq	0x700(%rbp), %rdx
141e2245b:     	testq	%rdx, %rdx
141e2245e:     	je	0x141e22472 <.text+0x1e21472>
141e22460:     	movq	0x708(%rbp), %rcx
141e22467:     	movl	$0x1, %r8d
141e2246d:     	callq	0x140613c20 <.text+0x612c20>
141e22472:     	movq	0x718(%rbp), %rdx
141e22479:     	movq	%rdx, %rax
141e2247c:     	negq	%rax
141e2247f:     	jo	0x141e22495 <.text+0x1e21495>
141e22481:     	jae	0x141e22495 <.text+0x1e21495>
141e22483:     	movq	0x720(%rbp), %rcx
141e2248a:     	movl	$0x1, %r8d
141e22490:     	callq	0x140613c20 <.text+0x612c20>
141e22495:     	movq	0x27f0(%rbp), %rdx
141e2249c:     	movl	0x1c(%rdx), %eax
141e2249f:     	movq	0x10(%rdx), %rcx
141e224a3:     	movl	%eax, (%rcx)
141e224a5:     	movq	(%rdx), %rax
141e224a8:     	movb	$0x2, (%rax)
141e224ab:     	movups	0x24f0(%rbp), %xmm0
141e224b2:     	leaq	0x1ce8(%rbp), %rax
141e224b9:     	movups	%xmm0, 0x28(%rax)
141e224bd:     	movdqa	0x2500(%rbp), %xmm0
141e224c5:     	movdqu	%xmm0, 0x18(%rax)
141e224ca:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
141e224d4:     	movq	%rax, 0x718(%rbp)
141e224db:     	movq	$0x0, 0x700(%rbp)
141e224e6:     	movq	$0x8, 0x708(%rbp)
141e224f1:     	movq	$0x0, 0x710(%rbp)
141e224fc:     	movb	$0x0, 0x798(%rbp)
141e22503:     	movabsq	$-0x7ffffffffffffffd, %rax # imm = 0x8000000000000003
141e2250d:     	movq	%rax, 0x1cf8(%rbp)
141e22514:     	movq	0x700(%rbp), %rax
141e2251b:     	movq	%rax, 0x1ce0(%rbp)
141e22522:     	movq	0x708(%rbp), %rax
141e22529:     	movq	%rax, 0x1ce8(%rbp)
141e22530:     	movq	$0x0, 0x1cf0(%rbp)
141e2253b:     	movb	$0x0, 0x1d78(%rbp)
141e22542:     	movq	0x27f8(%rbp), %rcx
141e22549:     	leaq	0x1ce0(%rbp), %rdx
141e22550:     	leaq	0x1165de1(%rip), %r8    # 0x142f88338
141e22557:     	callq	0x140fe0190 <.text+0xfdf190>
141e2255c:     	movq	0x27e0(%rbp), %rdx
141e22563:     	movl	0x1c(%rdx), %eax
141e22566:     	movq	0x10(%rdx), %rcx
141e2256a:     	movl	%eax, (%rcx)
141e2256c:     	movq	(%rdx), %rcx
141e2256f:     	movq	$0x2, 0x700(%rbp)
141e2257a:     	movb	$0x0, 0x2735(%rbp)
141e22581:     	leaq	0x700(%rbp), %rdx
141e22588:     	callq	0x14093fad0 <.text+0x93ead0>
141e2258d:     	movl	0x26b8(%rbp), %eax
141e22593:     	movq	0x2678(%rbp), %rcx
141e2259a:     	movl	%eax, (%rcx)
141e2259c:     	movq	0x26f0(%rbp), %rax
141e225a3:     	movq	0x30(%rax), %rsi
141e225a7:     	movq	0x40(%rax), %rdi
141e225ab:     	cmpq	0x20(%rax), %rsi
141e225af:     	jne	0x141e225cb <.text+0x1e215cb>
141e225b1:     	movb	$0x0, 0x2735(%rbp)
141e225b8:     	movq	0x2680(%rbp), %rcx
141e225bf:     	leaq	0xd92caa(%rip), %rdx    # 0x142bb5270
141e225c6:     	callq	0x140562630 <.text+0x561630>
141e225cb:     	movq	0x26f0(%rbp), %rdx
141e225d2:     	movq	0x28(%rdx), %rax
141e225d6:     	movq	%rsi, %rcx
141e225d9:     	shlq	$0x4, %rcx
141e225dd:     	movq	%rdi, (%rax,%rcx)
141e225e1:     	movb	$0x8, 0x8(%rax,%rcx)
141e225e6:     	incq	%rsi
141e225e9:     	movq	%rsi, 0x30(%rdx)
141e225ed:     	incq	0x40(%rdx)
141e225f1:     	movq	0x26c0(%rbp), %rdx
141e225f8:     	testq	%rdx, %rdx
141e225fb:     	je	0x141e21cdb <.text+0x1e20cdb>
141e22601:     	movq	0x26c8(%rbp), %rcx
141e22608:     	movl	$0x1, %r8d
141e2260e:     	callq	0x140613c20 <.text+0x612c20>
141e22613:     	jmp	0x141e21cdb <.text+0x1e20cdb>
141e22618:     	leaq	0xbf4be9(%rip), %rcx    # 0x142a17208
141e2261f:     	leaq	0xbf59b2(%rip), %r8     # 0x142a17fd8
141e22626:     	movl	$0x16, %edx
141e2262b:     	callq	0x1428d9330 <.text+0x28d8330>
141e22630:     	jmp	0x141e22ef7 <.text+0x1e21ef7>
141e22635:     	leaq	0xbf4bcc(%rip), %rcx    # 0x142a17208
141e2263c:     	leaq	0xbf5995(%rip), %r8     # 0x142a17fd8
141e22643:     	movl	$0x16, %edx
141e22648:     	callq	0x1428d9330 <.text+0x28d8330>
141e2264d:     	jmp	0x141e22ef7 <.text+0x1e21ef7>
141e22652:     	leaq	0x1165cd7(%rip), %r8    # 0x142f88330
141e22659:     	leaq	0x700(%rbp), %rcx
141e22660:     	leaq	0x25c0(%rbp), %rdx
141e22667:     	movl	$0x3, %r9d
141e2266d:     	callq	0x14068bfc0 <.text+0x68afc0>
141e22672:     	leaq	0x700(%rbp), %rcx
141e22679:     	movb	$0x1, %dl
141e2267b:     	callq	0x140f9dba0 <.text+0xf9cba0>
141e22680:     	movl	$0x7b5a51a0, 0x258(%rax) # imm = 0x7B5A51A0
141e2268a:     	movq	0x710(%rbp), %rax
141e22691:     	movq	%rax, 0x1cf0(%rbp)
141e22698:     	movdqu	0x700(%rbp), %xmm0
141e226a0:     	movdqa	%xmm0, 0x1ce0(%rbp)
141e226a8:     	leaq	0x1ce0(%rbp), %rcx
141e226af:     	movb	$0x1, %dl
141e226b1:     	callq	0x140f9dba0 <.text+0xf9cba0>
141e226b6:     	movb	$0x12, 0x25d(%rax)
141e226bd:     	movdqa	0x1ce0(%rbp), %xmm0
141e226c5:     	movdqa	%xmm0, 0x700(%rbp)
141e226cd:     	movq	0x1cf0(%rbp), %rax
141e226d4:     	movq	%rax, 0x710(%rbp)
141e226db:     	movq	0x700(%rbp), %rsi
141e226e2:     	cmpq	$0x0, 0x1b8(%rsi)
141e226ea:     	je	0x141e22e36 <.text+0x1e21e36>
141e226f0:     	leaq	0x708(%rbp), %rdx
141e226f7:     	leaq	0x1c0(%rsi), %rcx
141e226fe:     	callq	0x1405b0d70 <.text+0x5afd70>
141e22703:     	movq	0x1a0(%rsi), %rcx
141e2270a:     	movq	0x1a8(%rsi), %rdx
141e22711:     	movq	%rdx, %r8
141e22714:     	andq	%rax, %r8
141e22717:     	shrq	$0x39, %rax
141e2271b:     	movd	%eax, %xmm0
141e2271f:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
141e22723:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
141e22728:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
141e2272d:     	leaq	-0xc0(%rcx), %rax
141e22734:     	movq	0x708(%rbp), %r9
141e2273b:     	xorl	%r10d, %r10d
141e2273e:     	pcmpeqd	%xmm1, %xmm1
141e22742:     	movq	0x2700(%rbp), %rdi
141e22749:     	movdqu	(%rcx,%r8), %xmm2
141e2274f:     	movdqa	%xmm2, %xmm3
141e22753:     	pcmpeqb	%xmm0, %xmm3
141e22757:     	pmovmskb	%xmm3, %r11d
141e2275c:     	testl	%r11d, %r11d
141e2275f:     	je	0x141e2278a <.text+0x1e2178a>
141e22761:     	tzcntl	%r11d, %esi
141e22766:     	addq	%r8, %rsi
141e22769:     	andq	%rdx, %rsi
141e2276c:     	negq	%rsi
141e2276f:     	leaq	(%rsi,%rsi,2), %rsi
141e22773:     	shlq	$0x6, %rsi
141e22777:     	cmpq	(%rax,%rsi), %r9
141e2277b:     	je	0x141e227ac <.text+0x1e217ac>
141e2277d:     	leal	-0x1(%r11), %esi
141e22781:     	andw	%r11w, %si
141e22785:     	movl	%esi, %r11d
141e22788:     	jne	0x141e22761 <.text+0x1e21761>
141e2278a:     	pcmpeqb	%xmm1, %xmm2
141e2278e:     	pmovmskb	%xmm2, %r11d
141e22793:     	testl	%r11d, %r11d
141e22796:     	jne	0x141e22e36 <.text+0x1e21e36>
141e2279c:     	addq	%r10, %r8
141e2279f:     	addq	$0x10, %r8
141e227a3:     	addq	$0x10, %r10
141e227a7:     	andq	%rdx, %r8
141e227aa:     	jmp	0x141e22749 <.text+0x1e21749>
141e227ac:     	testb	$0x30, -0x2(%rcx,%rsi)
141e227b1:     	je	0x141e227b7 <.text+0x1e217b7>
141e227b3:     	movb	$0x0, 0x49(%rdi)
141e227b7:     	movq	0x2610(%rbp), %rdx
141e227be:     	testq	%rdx, %rdx
141e227c1:     	je	0x141e227d5 <.text+0x1e217d5>
141e227c3:     	movq	0x2618(%rbp), %rcx
141e227ca:     	movl	$0x1, %r8d
141e227d0:     	callq	0x140613c20 <.text+0x612c20>
141e227d5:     	movq	0x2628(%rbp), %rdx
141e227dc:     	movq	%rdx, %rax
141e227df:     	negq	%rax
141e227e2:     	jo	0x141e227f8 <.text+0x1e217f8>
141e227e4:     	jae	0x141e227f8 <.text+0x1e217f8>
141e227e6:     	movq	0x2630(%rbp), %rcx
141e227ed:     	movl	$0x1, %r8d
141e227f3:     	callq	0x140613c20 <.text+0x612c20>
141e227f8:     	testb	$0x1, 0x10(%rdi)
141e227fc:     	je	0x141e22c84 <.text+0x1e21c84>
141e22802:     	movdqu	0x18(%rdi), %xmm0
141e22807:     	movdqa	%xmm0, 0x26c0(%rbp)
141e2280f:     	movl	0x2714(%rbp), %eax
141e22815:     	movq	0x26e8(%rbp), %rcx
141e2281c:     	movl	%eax, (%rcx)
141e2281e:     	leaq	0x1165b2b(%rip), %rax   # 0x142f88350
141e22825:     	movq	%rax, 0x700(%rbp)
141e2282c:     	movq	$0x12, 0x708(%rbp)
141e22837:     	leaq	0x700(%rbp), %rcx
141e2283e:     	callq	0x1405bdda0 <.text+0x5bcda0>
141e22843:     	movq	%rax, %rdi
141e22846:     	movabsq	$-0x4e96acdfffb04bbd, %rax # imm = 0xB1695320004FB443
141e22850:     	xorq	%rax, %rdi
141e22853:     	movq	0x2728(%rbp), %rcx
141e2285a:     	movq	%rdi, %rdx
141e2285d:     	callq	0x140683ec0 <.text+0x682ec0>
141e22862:     	movb	$0x0, 0x5d(%rax)
141e22866:     	movb	$0x1, 0xb7(%rax)
141e2286d:     	testb	$0x1, %dl
141e22870:     	jne	0x141e228b7 <.text+0x1e218b7>
141e22872:     	movq	0x2728(%rbp), %rax
141e22879:     	movq	0x10(%rax), %rsi
141e2287d:     	cmpq	(%rax), %rsi
141e22880:     	jne	0x141e22895 <.text+0x1e21895>
141e22882:     	leaq	0xd8c73f(%rip), %rdx    # 0x142baefc8
141e22889:     	movq	0x2728(%rbp), %rcx
141e22890:     	callq	0x1406319d0 <.text+0x6309d0>
141e22895:     	movq	0x2728(%rbp), %rdx
141e2289c:     	movq	0x8(%rdx), %rax
141e228a0:     	imulq	$0x38, %rsi, %rcx
141e228a4:     	movl	$0x1, (%rax,%rcx)
141e228ab:     	movq	%rdi, 0x30(%rax,%rcx)
141e228b0:     	incq	%rsi
141e228b3:     	movq	%rsi, 0x10(%rdx)
141e228b7:     	movq	0x2728(%rbp), %rax
141e228be:     	movq	%rax, 0x25e0(%rbp)
141e228c5:     	movq	%rdi, 0x25e8(%rbp)
141e228cc:     	movq	$0x0, 0x25f0(%rbp)
141e228d7:     	movq	$0x0, 0x700(%rbp)
141e228e2:     	movabsq	$0x41223169ff28813b, %rsi # imm = 0x41223169FF28813B
141e228ec:     	movq	%rsi, 0x708(%rbp)
141e228f3:     	movabsq	$-0x58648d975d569727, %rbx # imm = 0xA79B7268A2A968D9
141e228fd:     	movq	%rbx, 0x710(%rbp)
141e22904:     	leaq	0x700(%rbp), %rcx
141e2290b:     	callq	0x1405bdf20 <.text+0x5bcf20>
141e22910:     	movq	%rdi, 0x700(%rbp)
141e22917:     	movq	%rax, 0x708(%rbp)
141e2291e:     	leaq	0x700(%rbp), %rcx
141e22925:     	callq	0x1405bd910 <.text+0x5bc910>
141e2292a:     	leaq	0x2690(%rbp), %rcx
141e22931:     	leaq	0x25e0(%rbp), %rdx
141e22938:     	movq	%rax, %r8
141e2293b:     	callq	0x1406861b0 <.text+0x6851b0>
141e22940:     	movq	0x2698(%rbp), %rdi
141e22947:     	movq	$0x0, 0x700(%rbp)
141e22952:     	movq	%rsi, 0x708(%rbp)
141e22959:     	movq	%rbx, 0x710(%rbp)
141e22960:     	leaq	0x700(%rbp), %rcx
141e22967:     	callq	0x1405bdf20 <.text+0x5bcf20>
141e2296c:     	movq	%rdi, 0x700(%rbp)
141e22973:     	movq	%rax, 0x708(%rbp)
141e2297a:     	leaq	0x700(%rbp), %rcx
141e22981:     	callq	0x1405bd910 <.text+0x5bc910>
141e22986:     	leaq	0x700(%rbp), %rcx
141e2298d:     	leaq	0x2690(%rbp), %rdx
141e22994:     	movq	%rax, %r8
141e22997:     	callq	0x1406861b0 <.text+0x6851b0>
141e2299c:     	movq	$0x0, 0x20(%rsp)
141e229a5:     	leaq	0x11659b6(%rip), %r8    # 0x142f88362
141e229ac:     	leaq	0x2510(%rbp), %rcx
141e229b3:     	leaq	0x700(%rbp), %rdx
141e229ba:     	movl	$0x10, %r9d
141e229c0:     	callq	0x140f9e970 <.text+0xf9d970>
141e229c5:     	leaq	0x11659a6(%rip), %rdx   # 0x142f88372
141e229cc:     	leaq	0x2510(%rbp), %rcx
141e229d3:     	movl	$0x6, %r8d
141e229d9:     	callq	0x140fbb410 <.text+0xfba410>
141e229de:     	movb	$0x0, 0x25c0(%rbp)
141e229e5:     	leaq	0x2610(%rbp), %rcx
141e229ec:     	leaq	0x26c0(%rbp), %rdi
141e229f3:     	movq	%rdi, %rdx
141e229f6:     	callq	0x140b46180 <.text+0xb45180>
141e229fb:     	leaq	0x2610(%rbp), %r15
141e22a02:     	movq	%r15, %rcx
141e22a05:     	callq	0x140d76ed0 <.text+0xd75ed0>
141e22a0a:     	movq	%rdx, %r8
141e22a0d:     	leaq	0x1ce0(%rbp), %rcx
141e22a14:     	movq	%rax, %rdx
141e22a17:     	callq	0x140ae8330 <.text+0xae7330>
141e22a1c:     	leaq	0x700(%rbp), %rcx
141e22a23:     	leaq	0x1ce0(%rbp), %rdx
141e22a2a:     	movl	$0x730, %r8d            # imm = 0x730
141e22a30:     	callq	0x142923c00 <.text+0x2922c00>
141e22a35:     	movq	0x700(%rbp), %rsi
141e22a3c:     	cmpq	$0x2, %rsi
141e22a40:     	jne	0x141e22a50 <.text+0x1e21a50>
141e22a42:     	leaq	0x708(%rbp), %rcx
141e22a49:     	callq	0x140d4cc30 <.text+0xd4bc30>
141e22a4e:     	jmp	0x141e22a66 <.text+0x1e21a66>
141e22a50:     	leaq	0x1ce8(%rbp), %rdx
141e22a57:     	leaq	-0x28(%rbp), %rcx
141e22a5b:     	movl	$0x728, %r8d            # imm = 0x728
141e22a61:     	callq	0x142923c00 <.text+0x2922c00>
141e22a66:     	movq	$0x2, 0x700(%rbp)
141e22a71:     	movq	$0x2, 0xe30(%rbp)
141e22a7c:     	movq	%rsi, 0x1560(%rbp)
141e22a83:     	leaq	0x1568(%rbp), %rcx
141e22a8a:     	leaq	-0x28(%rbp), %rdx
141e22a8e:     	movl	$0x728, %r8d            # imm = 0x728
141e22a94:     	callq	0x142923c00 <.text+0x2922c00>
141e22a99:     	leaq	0x1165578(%rip), %r8    # 0x142f88018
141e22aa0:     	leaq	0x1ce0(%rbp), %rcx
141e22aa7:     	leaq	0x700(%rbp), %rdx
141e22aae:     	callq	0x140fcf110 <.text+0xfce110>
141e22ab3:     	movq	0x2610(%rbp), %rdx
141e22aba:     	testq	%rdx, %rdx
141e22abd:     	je	0x141e22ad1 <.text+0x1e21ad1>
141e22abf:     	movq	0x2618(%rbp), %rcx
141e22ac6:     	movl	$0x1, %r8d
141e22acc:     	callq	0x140613c20 <.text+0x612c20>
141e22ad1:     	movq	0x1ce8(%rbp), %r14
141e22ad8:     	movq	0x1cf0(%rbp), %r12
141e22adf:     	movq	%r15, 0x700(%rbp)
141e22ae6:     	cmpq	$0x2, %r12
141e22aea:     	jae	0x141e22e5f <.text+0x1e21e5f>
141e22af0:     	movq	0x2700(%rbp), %rax
141e22af7:     	leaq	0x28(%rax), %rsi
141e22afb:     	movq	%r12, %rcx
141e22afe:     	shrq	$0x3, %rcx
141e22b02:     	movq	%rcx, 0x24c0(%rbp)
141e22b09:     	movq	(%rsi), %rax
141e22b0c:     	cmpq	%rax, %rcx
141e22b0f:     	cmovaeq	%rax, %rcx
141e22b13:     	movq	%rcx, (%rsi)
141e22b16:     	leaq	0x8(,%rcx,8), %rdx
141e22b1e:     	shlq	$0x3, %rcx
141e22b22:     	cmpq	%rdx, %r12
141e22b25:     	cmovbq	%r12, %rdx
141e22b29:     	cmpq	%rcx, %r12
141e22b2c:     	jb	0x141e22e82 <.text+0x1e21e82>
141e22b32:     	subq	%rcx, %rdx
141e22b35:     	leaq	(%rcx,%rcx,2), %rax
141e22b39:     	shlq	$0x4, %rax
141e22b3d:     	addq	%r14, %rax
141e22b40:     	movq	%rax, 0x2610(%rbp)
141e22b47:     	movq	%rdx, 0x2618(%rbp)
141e22b4e:     	movq	%r15, 0x700(%rbp)
141e22b55:     	movq	%rdi, 0x708(%rbp)
141e22b5c:     	movq	0x24c8(%rbp), %rax
141e22b63:     	movq	%rax, 0x710(%rbp)
141e22b6a:     	movq	0x2608(%rbp), %rax
141e22b71:     	movq	%rax, 0x718(%rbp)
141e22b78:     	leaq	0x25c0(%rbp), %rdi
141e22b7f:     	movq	%rdi, 0x720(%rbp)
141e22b86:     	movq	0x27f8(%rbp), %rax
141e22b8d:     	movq	%rax, 0x728(%rbp)
141e22b94:     	movq	0x27f0(%rbp), %rax
141e22b9b:     	movq	%rax, 0x730(%rbp)
141e22ba2:     	movq	0x27e0(%rbp), %rax
141e22ba9:     	movq	%rax, 0x738(%rbp)
141e22bb0:     	leaq	0x2808(%rbp), %rax
141e22bb7:     	movq	%rax, 0x740(%rbp)
141e22bbe:     	leaq	0x11657cb(%rip), %rdx   # 0x142f88390
141e22bc5:     	leaq	0x2510(%rbp), %rcx
141e22bcc:     	leaq	0x700(%rbp), %r9
141e22bd3:     	movl	$0x4, %r8d
141e22bd9:     	callq	0x140fbd420 <.text+0xfbc420>
141e22bde:     	leaq	0x24c0(%rbp), %rax
141e22be5:     	movq	%rax, 0x700(%rbp)
141e22bec:     	movq	%rsi, 0x708(%rbp)
141e22bf3:     	movq	%rdi, 0x710(%rbp)
141e22bfa:     	leaq	0x1165793(%rip), %rdx   # 0x142f88394
141e22c01:     	leaq	0x2510(%rbp), %rcx
141e22c08:     	leaq	0x700(%rbp), %r9
141e22c0f:     	movl	$0x6, %r8d
141e22c15:     	callq	0x140fc23e0 <.text+0xfc13e0>
141e22c1a:     	cmpb	$0x0, 0x25c0(%rbp)
141e22c21:     	je	0x141e22c32 <.text+0x1e21c32>
141e22c23:     	movq	0x2700(%rbp), %rax
141e22c2a:     	movq	$0x0, 0x10(%rax)
141e22c32:     	testq	%r12, %r12
141e22c35:     	je	0x141e22c62 <.text+0x1e21c62>
141e22c37:     	leaq	0x8(%r14), %rsi
141e22c3b:     	jmp	0x141e22c49 <.text+0x1e21c49>
141e22c3d:     	nopl	(%rax)
141e22c40:     	addq	$0x30, %rsi
141e22c44:     	decq	%r12
141e22c47:     	je	0x141e22c62 <.text+0x1e21c62>
141e22c49:     	movq	-0x8(%rsi), %rdx
141e22c4d:     	testq	%rdx, %rdx
141e22c50:     	je	0x141e22c40 <.text+0x1e21c40>
141e22c52:     	movq	(%rsi), %rcx
141e22c55:     	movl	$0x1, %r8d
141e22c5b:     	callq	0x140613c20 <.text+0x612c20>
141e22c60:     	jmp	0x141e22c40 <.text+0x1e21c40>
141e22c62:     	movq	0x1ce0(%rbp), %rax
141e22c69:     	testq	%rax, %rax
141e22c6c:     	je	0x141e22c84 <.text+0x1e21c84>
141e22c6e:     	shlq	$0x4, %rax
141e22c72:     	leaq	(%rax,%rax,2), %rdx
141e22c76:     	movl	$0x8, %r8d
141e22c7c:     	movq	%r14, %rcx
141e22c7f:     	callq	0x140613c20 <.text+0x612c20>
141e22c84:     	testb	$0x1, 0x2734(%rbp)
141e22c8b:     	jne	0x141e22d9b <.text+0x1e21d9b>
141e22c91:     	movl	0x2714(%rbp), %eax
141e22c97:     	movq	0x26e8(%rbp), %rcx
141e22c9e:     	movl	%eax, (%rcx)
141e22ca0:     	leaq	0x11656f3(%rip), %rax   # 0x142f8839a
141e22ca7:     	movq	%rax, 0x700(%rbp)
141e22cae:     	movq	$0x4, 0x708(%rbp)
141e22cb9:     	leaq	0x700(%rbp), %rcx
141e22cc0:     	callq	0x1405bdda0 <.text+0x5bcda0>
141e22cc5:     	movq	%rax, %rsi
141e22cc8:     	movabsq	$-0x4e96acdfffb04bbd, %rax # imm = 0xB1695320004FB443
141e22cd2:     	xorq	%rax, %rsi
141e22cd5:     	movq	0x2728(%rbp), %rcx
141e22cdc:     	movq	%rsi, %rdx
141e22cdf:     	callq	0x140683ec0 <.text+0x682ec0>
141e22ce4:     	movb	$0x0, 0x5d(%rax)
141e22ce8:     	testb	$0x1, %dl
141e22ceb:     	jne	0x141e22d32 <.text+0x1e21d32>
141e22ced:     	movq	0x2728(%rbp), %rax
141e22cf4:     	movq	0x10(%rax), %rdi
141e22cf8:     	cmpq	(%rax), %rdi
141e22cfb:     	jne	0x141e22d10 <.text+0x1e21d10>
141e22cfd:     	leaq	0xd8c2c4(%rip), %rdx    # 0x142baefc8
141e22d04:     	movq	0x2728(%rbp), %rcx
141e22d0b:     	callq	0x1406319d0 <.text+0x6309d0>
141e22d10:     	movq	0x2728(%rbp), %rdx
141e22d17:     	movq	0x8(%rdx), %rax
141e22d1b:     	imulq	$0x38, %rdi, %rcx
141e22d1f:     	movl	$0x2, (%rax,%rcx)
141e22d26:     	movq	%rsi, 0x30(%rax,%rcx)
141e22d2b:     	incq	%rdi
141e22d2e:     	movq	%rdi, 0x10(%rdx)
141e22d32:     	movq	0x2728(%rbp), %rax
141e22d39:     	movq	%rax, 0x1ce0(%rbp)
141e22d40:     	movq	%rsi, 0x1ce8(%rbp)
141e22d47:     	movq	$0x0, 0x1cf0(%rbp)
141e22d52:     	movq	$0x0, 0x20(%rsp)
141e22d5b:     	leaq	0x116563c(%rip), %r8    # 0x142f8839e
141e22d62:     	leaq	0x700(%rbp), %rcx
141e22d69:     	leaq	0x1ce0(%rbp), %rdx
141e22d70:     	movl	$0x9, %r9d
141e22d76:     	callq	0x140f9e970 <.text+0xf9d970>
141e22d7b:     	leaq	0x1165625(%rip), %rdx   # 0x142f883a7
141e22d82:     	leaq	0x700(%rbp), %rcx
141e22d89:     	movl	$0x6, %r8d
141e22d8f:     	movq	0x27e0(%rbp), %r9
141e22d96:     	callq	0x140fb8600 <.text+0xfb7600>
141e22d9b:     	cmpq	$0x0, 0x25a0(%rbp)
141e22da3:     	je	0x141e22dec <.text+0x1e21dec>
141e22da5:     	callq	0x140c18130 <.text+0xc17130>
141e22daa:     	movq	%rax, %rcx
141e22dad:     	movq	(%rax), %rax
141e22db0:     	cmpq	$0x1, %rax
141e22db4:     	jne	0x141e22e16 <.text+0x1e21e16>
141e22db6:     	addq	$0x8, %rcx
141e22dba:     	cmpq	$0x0, (%rcx)
141e22dbe:     	jne	0x141e22e53 <.text+0x1e21e53>
141e22dc4:     	movq	$-0x1, (%rcx)
141e22dcb:     	movq	%rcx, 0x2728(%rbp)
141e22dd2:     	addq	$0x8, %rcx
141e22dd6:     	movq	0x25a8(%rbp), %rdx
141e22ddd:     	callq	0x140c170f0 <.text+0xc160f0>
141e22de2:     	movq	0x2728(%rbp), %rax
141e22de9:     	incq	(%rax)
141e22dec:     	movaps	0x2740(%rbp), %xmm6
141e22df3:     	movaps	0x2750(%rbp), %xmm7
141e22dfa:     	movaps	0x2760(%rbp), %xmm8
141e22e02:     	addq	$0x27f8, %rsp           # imm = 0x27F8
141e22e09:     	popq	%rbx
141e22e0a:     	popq	%rdi
141e22e0b:     	popq	%rsi
141e22e0c:     	popq	%r12
141e22e0e:     	popq	%r13
141e22e10:     	popq	%r14
141e22e12:     	popq	%r15
141e22e14:     	popq	%rbp
141e22e15:     	retq
141e22e16:     	testq	%rax, %rax
141e22e19:     	jne	0x141e22e2a <.text+0x1e21e2a>
141e22e1b:     	xorl	%edx, %edx
141e22e1d:     	callq	0x1428e6250 <.text+0x28e5250>
141e22e22:     	movq	%rax, %rcx
141e22e25:     	testq	%rax, %rax
141e22e28:     	jne	0x141e22dba <.text+0x1e21dba>
141e22e2a:     	leaq	0xd924a7(%rip), %rcx    # 0x142bb52d8
141e22e31:     	callq	0x1428d80d0 <.text+0x28d70d0>
141e22e36:     	leaq	0xbf43cb(%rip), %rcx    # 0x142a17208
141e22e3d:     	leaq	0xbf5194(%rip), %r8     # 0x142a17fd8
141e22e44:     	movl	$0x16, %edx
141e22e49:     	callq	0x1428d9330 <.text+0x28d8330>
141e22e4e:     	jmp	0x141e22ef7 <.text+0x1e21ef7>
141e22e53:     	leaq	0xd924ee(%rip), %rcx    # 0x142bb5348
141e22e5a:     	callq	0x1428d9250 <.text+0x28d8250>
141e22e5f:     	cmpq	$0x15, %r12
141e22e63:     	jae	0x141e22eb5 <.text+0x1e21eb5>
141e22e65:     	leaq	0x700(%rbp), %r9
141e22e6c:     	movl	$0x1, %r8d
141e22e72:     	movq	%r14, %rcx
141e22e75:     	movq	%r12, %rdx
141e22e78:     	callq	0x140fe41e0 <.text+0xfe31e0>
141e22e7d:     	jmp	0x141e22af0 <.text+0x1e21af0>
141e22e82:     	leaq	0x11654ef(%rip), %r8    # 0x142f88378
141e22e89:     	callq	0x1428d9850 <.text+0x28d8850>
141e22e8e:     	jmp	0x141e22ef7 <.text+0x1e21ef7>
141e22e90:     	xorl	%ecx, %ecx
141e22e92:     	jmp	0x141e22ea0 <.text+0x1e21ea0>
141e22e94:     	movl	$0x1, %ecx
141e22e99:     	movq	%rdi, 0x2670(%rbp)
141e22ea0:     	leaq	0x11651d9(%rip), %r8    # 0x142f88080
141e22ea7:     	movq	0x2670(%rbp), %rdx
141e22eae:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
141e22eb3:     	jmp	0x141e22ef7 <.text+0x1e21ef7>
141e22eb5:     	leaq	0x700(%rbp), %r8
141e22ebc:     	movq	%r14, %rcx
141e22ebf:     	movq	%r12, %rdx
141e22ec2:     	callq	0x141e59b70 <.text+0x1e58b70>
141e22ec7:     	jmp	0x141e22af0 <.text+0x1e21af0>
141e22ecc:     	movq	$0x0, 0x2688(%rbp)
141e22ed7:     	movq	%rcx, %rsi
141e22eda:     	movb	$0x1, 0x2735(%rbp)
141e22ee1:     	leaq	0xd921a8(%rip), %r8     # 0x142bb5090
141e22ee8:     	movq	0x2688(%rbp), %rcx
141e22eef:     	movq	%rsi, %rdx
141e22ef2:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
141e22ef7:     	ud2
141e22ef9:     	nopl	(%rax)
141e22f00:     	movq	%rdx, 0x10(%rsp)
141e22f05:     	pushq	%rbp
141e22f06:     	pushq	%r15
141e22f08:     	pushq	%r14
141e22f0a:     	pushq	%r13
141e22f0c:     	pushq	%r12
141e22f0e:     	pushq	%rsi
141e22f0f:     	pushq	%rdi
141e22f10:     	pushq	%rbx
141e22f11:     	subq	$0x88, %rsp
141e22f18:     	leaq	0x80(%rdx), %rbp
141e22f1f:     	movaps	%xmm8, 0x50(%rsp)
141e22f25:     	movdqa	%xmm7, 0x60(%rsp)
141e22f2b:     	movaps	%xmm6, 0x70(%rsp)
141e22f30:     	movq	0x2728(%rbp), %rax
141e22f37:     	incq	(%rax)
141e22f3a:     	movaps	0x70(%rsp), %xmm6
141e22f3f:     	movaps	0x60(%rsp), %xmm7
141e22f44:     	movaps	0x50(%rsp), %xmm8
141e22f4a:     	addq	$0x88, %rsp
141e22f51:     	popq	%rbx
141e22f52:     	popq	%rdi
141e22f53:     	popq	%rsi
141e22f54:     	popq	%r12
141e22f56:     	popq	%r13
141e22f58:     	popq	%r14
141e22f5a:     	popq	%r15
141e22f5c:     	popq	%rbp
141e22f5d:     	retq
141e22f5e:     	nop
141e22f60:     	movq	%rdx, 0x10(%rsp)
141e22f65:     	pushq	%rbp
141e22f66:     	pushq	%r15
141e22f68:     	pushq	%r14
141e22f6a:     	pushq	%r13
141e22f6c:     	pushq	%r12
141e22f6e:     	pushq	%rsi
141e22f6f:     	pushq	%rdi
141e22f70:     	pushq	%rbx
141e22f71:     	subq	$0x88, %rsp
141e22f78:     	leaq	0x80(%rdx), %rbp
141e22f7f:     	movaps	%xmm8, 0x50(%rsp)
141e22f85:     	movdqa	%xmm7, 0x60(%rsp)
141e22f8b:     	movaps	%xmm6, 0x70(%rsp)
141e22f90:     	cmpq	$0x0, 0x25a0(%rbp)
141e22f98:     	je	0x141e22fc9 <.text+0x1e21fc9>
141e22f9a:     	leaq	0x25a8(%rbp), %rax
141e22fa1:     	movq	%rax, 0x24b8(%rbp)
141e22fa8:     	leaq	0x24b8(%rbp), %rax
141e22faf:     	movq	%rax, 0x24b0(%rbp)
141e22fb6:     	leaq	0x11650fb(%rip), %rcx   # 0x142f880b8
141e22fbd:     	leaq	0x24b0(%rbp), %rdx
141e22fc4:     	callq	0x140fcfc20 <.text+0xfcec20>
141e22fc9:     	movaps	0x70(%rsp), %xmm6
141e22fce:     	movaps	0x60(%rsp), %xmm7
141e22fd3:     	movaps	0x50(%rsp), %xmm8
141e22fd9:     	addq	$0x88, %rsp
141e22fe0:     	popq	%rbx
141e22fe1:     	popq	%rdi
141e22fe2:     	popq	%rsi
141e22fe3:     	popq	%r12
141e22fe5:     	popq	%r13
141e22fe7:     	popq	%r14
141e22fe9:     	popq	%r15
141e22feb:     	popq	%rbp
141e22fec:     	retq
141e22fed:     	nopl	(%rax)
141e22ff0:     	movq	%rdx, 0x10(%rsp)
141e22ff5:     	pushq	%rbp
141e22ff6:     	pushq	%r15
141e22ff8:     	pushq	%r14
141e22ffa:     	pushq	%r13
141e22ffc:     	pushq	%r12
141e22ffe:     	pushq	%rsi
141e22fff:     	pushq	%rdi
141e23000:     	pushq	%rbx
141e23001:     	subq	$0x88, %rsp
141e23008:     	leaq	0x80(%rdx), %rbp
141e2300f:     	movaps	%xmm8, 0x50(%rsp)
141e23015:     	movdqa	%xmm7, 0x60(%rsp)
141e2301b:     	movaps	%xmm6, 0x70(%rsp)
141e23020:     	movq	0x2728(%rbp), %rax
141e23027:     	incq	(%rax)
141e2302a:     	movaps	0x70(%rsp), %xmm6
141e2302f:     	movaps	0x60(%rsp), %xmm7
141e23034:     	movaps	0x50(%rsp), %xmm8
141e2303a:     	addq	$0x88, %rsp
141e23041:     	popq	%rbx
141e23042:     	popq	%rdi
141e23043:     	popq	%rsi
141e23044:     	popq	%r12
141e23046:     	popq	%r13
141e23048:     	popq	%r14
141e2304a:     	popq	%r15
141e2304c:     	popq	%rbp
141e2304d:     	retq
141e2304e:     	nop
141e23050:     	movq	%rdx, 0x10(%rsp)
141e23055:     	pushq	%rbp
141e23056:     	pushq	%r15
141e23058:     	pushq	%r14
141e2305a:     	pushq	%r13
141e2305c:     	pushq	%r12
141e2305e:     	pushq	%rsi
141e2305f:     	pushq	%rdi
141e23060:     	pushq	%rbx
141e23061:     	subq	$0x88, %rsp
141e23068:     	leaq	0x80(%rdx), %rbp
141e2306f:     	movaps	%xmm8, 0x50(%rsp)
141e23075:     	movdqa	%xmm7, 0x60(%rsp)
141e2307b:     	movaps	%xmm6, 0x70(%rsp)
141e23080:     	movq	0x700(%rbp), %rdx
141e23087:     	testq	%rdx, %rdx
141e2308a:     	je	0x141e2309e <.text+0x1e2209e>
141e2308c:     	movq	0x708(%rbp), %rcx
141e23093:     	movl	$0x1, %r8d
141e23099:     	callq	0x140613c20 <.text+0x612c20>
141e2309e:     	movaps	0x70(%rsp), %xmm6
141e230a3:     	movaps	0x60(%rsp), %xmm7
141e230a8:     	movaps	0x50(%rsp), %xmm8
141e230ae:     	addq	$0x88, %rsp
141e230b5:     	popq	%rbx
141e230b6:     	popq	%rdi
141e230b7:     	popq	%rsi
141e230b8:     	popq	%r12
141e230ba:     	popq	%r13
141e230bc:     	popq	%r14
141e230be:     	popq	%r15
141e230c0:     	popq	%rbp
141e230c1:     	retq
141e230c2:     	nopw	%cs:(%rax,%rax)
141e230d0:     	movq	%rdx, 0x10(%rsp)
141e230d5:     	pushq	%rbp
141e230d6:     	pushq	%r15
141e230d8:     	pushq	%r14
141e230da:     	pushq	%r13
141e230dc:     	pushq	%r12
141e230de:     	pushq	%rsi
141e230df:     	pushq	%rdi
141e230e0:     	pushq	%rbx
141e230e1:     	subq	$0x88, %rsp
141e230e8:     	leaq	0x80(%rdx), %rbp
141e230ef:     	movaps	%xmm8, 0x50(%rsp)
141e230f5:     	movdqa	%xmm7, 0x60(%rsp)
141e230fb:     	movaps	%xmm6, 0x70(%rsp)
141e23100:     	leaq	0x718(%rbp), %rcx
141e23107:     	callq	0x140fb63f0 <.text+0xfb53f0>
141e2310c:     	movb	$0x1, 0x2737(%rbp)
141e23113:     	movaps	0x70(%rsp), %xmm6
141e23118:     	movdqa	0x60(%rsp), %xmm7
141e2311e:     	movaps	0x50(%rsp), %xmm8
141e23124:     	addq	$0x88, %rsp
141e2312b:     	popq	%rbx
141e2312c:     	popq	%rdi
141e2312d:     	popq	%rsi
141e2312e:     	popq	%r12
141e23130:     	popq	%r13
141e23132:     	popq	%r14
141e23134:     	popq	%r15
141e23136:     	popq	%rbp
141e23137:     	retq
141e23138:     	nopl	(%rax,%rax)
141e23140:     	movq	%rdx, 0x10(%rsp)
141e23145:     	pushq	%rbp
141e23146:     	pushq	%r15
141e23148:     	pushq	%r14
141e2314a:     	pushq	%r13
141e2314c:     	pushq	%r12
141e2314e:     	pushq	%rsi
141e2314f:     	pushq	%rdi
141e23150:     	pushq	%rbx
141e23151:     	subq	$0x88, %rsp
141e23158:     	leaq	0x80(%rdx), %rbp
141e2315f:     	movaps	%xmm8, 0x50(%rsp)
141e23165:     	movdqa	%xmm7, 0x60(%rsp)
141e2316b:     	movaps	%xmm6, 0x70(%rsp)
141e23170:     	cmpq	$0x0, 0x26f0(%rbp)
141e23178:     	je	0x141e23193 <.text+0x1e22193>
141e2317a:     	movl	$0x1, %r8d
141e23180:     	movq	0x26b8(%rbp), %rcx
141e23187:     	movq	0x26f0(%rbp), %rdx
141e2318e:     	callq	0x140613c20 <.text+0x612c20>
141e23193:     	movaps	0x70(%rsp), %xmm6
141e23198:     	movaps	0x60(%rsp), %xmm7
141e2319d:     	movaps	0x50(%rsp), %xmm8
141e231a3:     	addq	$0x88, %rsp
141e231aa:     	popq	%rbx
141e231ab:     	popq	%rdi
141e231ac:     	popq	%rsi
141e231ad:     	popq	%r12
141e231af:     	popq	%r13
141e231b1:     	popq	%r14
141e231b3:     	popq	%r15
141e231b5:     	popq	%rbp
141e231b6:     	retq
141e231b7:     	nopw	(%rax,%rax)
141e231c0:     	movq	%rdx, 0x10(%rsp)
141e231c5:     	pushq	%rbp
141e231c6:     	pushq	%r15
141e231c8:     	pushq	%r14
141e231ca:     	pushq	%r13
141e231cc:     	pushq	%r12
141e231ce:     	pushq	%rsi
141e231cf:     	pushq	%rdi
141e231d0:     	pushq	%rbx
141e231d1:     	subq	$0x88, %rsp
141e231d8:     	leaq	0x80(%rdx), %rbp
141e231df:     	movaps	%xmm8, 0x50(%rsp)
141e231e5:     	movdqa	%xmm7, 0x60(%rsp)
141e231eb:     	movaps	%xmm6, 0x70(%rsp)
141e231f0:     	movq	0x26f0(%rbp), %rax
141e231f7:     	movq	0x26f8(%rbp), %rcx
141e231fe:     	movq	%rax, (%rcx)
141e23201:     	movq	0x26b8(%rbp), %rax
141e23208:     	movq	%rax, 0x8(%rcx)
141e2320c:     	movaps	0x700(%rbp), %xmm0
141e23213:     	movups	%xmm0, 0x10(%rcx)
141e23217:     	movdqu	0x2498(%rbp), %xmm0
141e2321f:     	movdqu	%xmm0, 0x20(%rcx)
141e23224:     	movq	0x24a8(%rbp), %rax
141e2322b:     	movq	%rax, 0x30(%rcx)
141e2322f:     	movq	0x2680(%rbp), %rax
141e23236:     	movq	%rax, 0x38(%rcx)
141e2323a:     	movl	0x2678(%rbp), %eax
141e23240:     	movl	%eax, 0x40(%rcx)
141e23243:     	movaps	0x70(%rsp), %xmm6
141e23248:     	movaps	0x60(%rsp), %xmm7
141e2324d:     	movaps	0x50(%rsp), %xmm8
141e23253:     	addq	$0x88, %rsp
141e2325a:     	popq	%rbx
141e2325b:     	popq	%rdi
141e2325c:     	popq	%rsi
141e2325d:     	popq	%r12
141e2325f:     	popq	%r13
141e23261:     	popq	%r14
141e23263:     	popq	%r15
141e23265:     	popq	%rbp
141e23266:     	retq
141e23267:     	nopw	(%rax,%rax)
141e23270:     	movq	%rdx, 0x10(%rsp)
141e23275:     	pushq	%rbp
141e23276:     	pushq	%r15
141e23278:     	pushq	%r14
141e2327a:     	pushq	%r13
141e2327c:     	pushq	%r12
141e2327e:     	pushq	%rsi
141e2327f:     	pushq	%rdi
141e23280:     	pushq	%rbx
141e23281:     	subq	$0x88, %rsp
141e23288:     	leaq	0x80(%rdx), %rbp
141e2328f:     	movaps	%xmm8, 0x50(%rsp)
141e23295:     	movaps	%xmm7, 0x60(%rsp)
141e2329a:     	movaps	%xmm6, 0x70(%rsp)
141e2329f:     	leaq	0x26c0(%rbp), %rcx
141e232a6:     	callq	0x140d4cc30 <.text+0xd4bc30>
141e232ab:     	movb	$0x0, 0x2737(%rbp)
141e232b2:     	movaps	0x70(%rsp), %xmm6
141e232b7:     	movdqa	0x60(%rsp), %xmm7
141e232bd:     	movaps	0x50(%rsp), %xmm8
141e232c3:     	addq	$0x88, %rsp
141e232ca:     	popq	%rbx
141e232cb:     	popq	%rdi
141e232cc:     	popq	%rsi
141e232cd:     	popq	%r12
141e232cf:     	popq	%r13
141e232d1:     	popq	%r14
141e232d3:     	popq	%r15
141e232d5:     	popq	%rbp
141e232d6:     	retq
141e232d7:     	nopw	(%rax,%rax)
141e232e0:     	movq	%rdx, 0x10(%rsp)
141e232e5:     	pushq	%rbp
141e232e6:     	pushq	%r15
141e232e8:     	pushq	%r14
141e232ea:     	pushq	%r13
141e232ec:     	pushq	%r12
141e232ee:     	pushq	%rsi
141e232ef:     	pushq	%rdi
141e232f0:     	pushq	%rbx
141e232f1:     	subq	$0x88, %rsp
141e232f8:     	leaq	0x80(%rdx), %rbp
141e232ff:     	movaps	%xmm8, 0x50(%rsp)
141e23305:     	movdqa	%xmm7, 0x60(%rsp)
141e2330b:     	movaps	%xmm6, 0x70(%rsp)
141e23310:     	cmpb	$0x0, 0x2737(%rbp)
141e23317:     	je	0x141e2335b <.text+0x1e2235b>
141e23319:     	movq	0x2610(%rbp), %rdx
141e23320:     	testq	%rdx, %rdx
141e23323:     	je	0x141e23337 <.text+0x1e22337>
141e23325:     	movq	0x2618(%rbp), %rcx
141e2332c:     	movl	$0x1, %r8d
141e23332:     	callq	0x140613c20 <.text+0x612c20>
141e23337:     	movaps	0x70(%rsp), %xmm6
141e2333c:     	movaps	0x60(%rsp), %xmm7
141e23341:     	movaps	0x50(%rsp), %xmm8
141e23347:     	addq	$0x88, %rsp
141e2334e:     	popq	%rbx
141e2334f:     	popq	%rdi
141e23350:     	popq	%rsi
141e23351:     	popq	%r12
141e23353:     	popq	%r13
141e23355:     	popq	%r14
141e23357:     	popq	%r15
141e23359:     	popq	%rbp
141e2335a:     	retq
141e2335b:     	movaps	0x70(%rsp), %xmm6
141e23360:     	movaps	0x60(%rsp), %xmm7
141e23365:     	movaps	0x50(%rsp), %xmm8
141e2336b:     	addq	$0x88, %rsp
141e23372:     	popq	%rbx
141e23373:     	popq	%rdi
141e23374:     	popq	%rsi
141e23375:     	popq	%r12
141e23377:     	popq	%r13
141e23379:     	popq	%r14
141e2337b:     	popq	%r15
141e2337d:     	popq	%rbp
141e2337e:     	retq
141e2337f:     	nop
141e23380:     	movq	%rdx, 0x10(%rsp)
141e23385:     	pushq	%rbp
141e23386:     	pushq	%r15
141e23388:     	pushq	%r14
141e2338a:     	pushq	%r13
141e2338c:     	pushq	%r12
141e2338e:     	pushq	%rsi
141e2338f:     	pushq	%rdi
141e23390:     	pushq	%rbx
141e23391:     	subq	$0x88, %rsp
141e23398:     	leaq	0x80(%rdx), %rbp
141e2339f:     	movaps	%xmm8, 0x50(%rsp)
141e233a5:     	movdqa	%xmm7, 0x60(%rsp)
141e233ab:     	movaps	%xmm6, 0x70(%rsp)
141e233b0:     	movq	0x700(%rbp), %rdx
141e233b7:     	testq	%rdx, %rdx
141e233ba:     	je	0x141e233ce <.text+0x1e223ce>
141e233bc:     	movq	0x708(%rbp), %rcx
141e233c3:     	movl	$0x1, %r8d
141e233c9:     	callq	0x140613c20 <.text+0x612c20>
141e233ce:     	movaps	0x70(%rsp), %xmm6
141e233d3:     	movaps	0x60(%rsp), %xmm7
141e233d8:     	movaps	0x50(%rsp), %xmm8
141e233de:     	addq	$0x88, %rsp
141e233e5:     	popq	%rbx
141e233e6:     	popq	%rdi
141e233e7:     	popq	%rsi
141e233e8:     	popq	%r12
141e233ea:     	popq	%r13
141e233ec:     	popq	%r14
141e233ee:     	popq	%r15
141e233f0:     	popq	%rbp
141e233f1:     	retq
141e233f2:     	nopw	%cs:(%rax,%rax)
141e23400:     	movq	%rdx, 0x10(%rsp)
141e23405:     	pushq	%rbp
141e23406:     	pushq	%r15
141e23408:     	pushq	%r14
141e2340a:     	pushq	%r13
141e2340c:     	pushq	%r12
141e2340e:     	pushq	%rsi
141e2340f:     	pushq	%rdi
141e23410:     	pushq	%rbx
141e23411:     	subq	$0x88, %rsp
141e23418:     	leaq	0x80(%rdx), %rbp
141e2341f:     	movaps	%xmm8, 0x50(%rsp)
141e23425:     	movdqa	%xmm7, 0x60(%rsp)
141e2342b:     	movaps	%xmm6, 0x70(%rsp)
141e23430:     	movq	0x26c0(%rbp), %rdx
141e23437:     	testq	%rdx, %rdx
141e2343a:     	je	0x141e2344e <.text+0x1e2244e>
141e2343c:     	movq	0x26c8(%rbp), %rcx
141e23443:     	movl	$0x1, %r8d
141e23449:     	callq	0x140613c20 <.text+0x612c20>
141e2344e:     	movaps	0x70(%rsp), %xmm6
141e23453:     	movaps	0x60(%rsp), %xmm7
141e23458:     	movaps	0x50(%rsp), %xmm8
141e2345e:     	addq	$0x88, %rsp
141e23465:     	popq	%rbx
141e23466:     	popq	%rdi
141e23467:     	popq	%rsi
141e23468:     	popq	%r12
141e2346a:     	popq	%r13
141e2346c:     	popq	%r14
141e2346e:     	popq	%r15
141e23470:     	popq	%rbp
141e23471:     	retq
141e23472:     	nopw	%cs:(%rax,%rax)
141e23480:     	movq	%rdx, 0x10(%rsp)
141e23485:     	pushq	%rbp
141e23486:     	pushq	%r15
141e23488:     	pushq	%r14
141e2348a:     	pushq	%r13
141e2348c:     	pushq	%r12
141e2348e:     	pushq	%rsi
141e2348f:     	pushq	%rdi
141e23490:     	pushq	%rbx
141e23491:     	subq	$0x88, %rsp
141e23498:     	leaq	0x80(%rdx), %rbp
141e2349f:     	movaps	%xmm8, 0x50(%rsp)
141e234a5:     	movdqa	%xmm7, 0x60(%rsp)
141e234ab:     	movaps	%xmm6, 0x70(%rsp)
141e234b0:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
141e234ba:     	movq	0x26f8(%rbp), %rcx
141e234c1:     	movq	%rax, (%rcx)
141e234c4:     	movaps	0x70(%rsp), %xmm6
141e234c9:     	movdqa	0x60(%rsp), %xmm7
141e234cf:     	movaps	0x50(%rsp), %xmm8
141e234d5:     	addq	$0x88, %rsp
141e234dc:     	popq	%rbx
141e234dd:     	popq	%rdi
141e234de:     	popq	%rsi
141e234df:     	popq	%r12
141e234e1:     	popq	%r13
141e234e3:     	popq	%r14
141e234e5:     	popq	%r15
141e234e7:     	popq	%rbp
141e234e8:     	retq
141e234e9:     	nopl	(%rax)
141e234f0:     	movq	%rdx, 0x10(%rsp)
141e234f5:     	pushq	%rbp
141e234f6:     	pushq	%r15
141e234f8:     	pushq	%r14
141e234fa:     	pushq	%r13
141e234fc:     	pushq	%r12
141e234fe:     	pushq	%rsi
141e234ff:     	pushq	%rdi
141e23500:     	pushq	%rbx
141e23501:     	subq	$0x88, %rsp
141e23508:     	leaq	0x80(%rdx), %rbp
141e2350f:     	movaps	%xmm8, 0x50(%rsp)
141e23515:     	movaps	%xmm7, 0x60(%rsp)
141e2351a:     	movdqa	%xmm6, 0x70(%rsp)
141e23520:     	movq	0x1ce0(%rbp), %rdx
141e23527:     	testq	%rdx, %rdx
141e2352a:     	je	0x141e2353e <.text+0x1e2253e>
141e2352c:     	movl	$0x1, %r8d
141e23532:     	movq	0x2718(%rbp), %rcx
141e23539:     	callq	0x140613c20 <.text+0x612c20>
141e2353e:     	movaps	0x70(%rsp), %xmm6
141e23543:     	movdqa	0x60(%rsp), %xmm7
141e23549:     	movaps	0x50(%rsp), %xmm8
141e2354f:     	addq	$0x88, %rsp
141e23556:     	popq	%rbx
141e23557:     	popq	%rdi
141e23558:     	popq	%rsi
141e23559:     	popq	%r12
141e2355b:     	popq	%r13
141e2355d:     	popq	%r14
141e2355f:     	popq	%r15
141e23561:     	popq	%rbp
141e23562:     	retq
141e23563:     	nopw	%cs:(%rax,%rax)
141e23570:     	movq	%rdx, 0x10(%rsp)
141e23575:     	pushq	%rbp
141e23576:     	pushq	%r15
141e23578:     	pushq	%r14
141e2357a:     	pushq	%r13
141e2357c:     	pushq	%r12
141e2357e:     	pushq	%rsi
141e2357f:     	pushq	%rdi
141e23580:     	pushq	%rbx
141e23581:     	subq	$0x88, %rsp
141e23588:     	leaq	0x80(%rdx), %rbp
141e2358f:     	movaps	%xmm8, 0x50(%rsp)
141e23595:     	movaps	%xmm7, 0x60(%rsp)
141e2359a:     	movdqa	%xmm6, 0x70(%rsp)
141e235a0:     	movq	0x700(%rbp), %rdx
141e235a7:     	testq	%rdx, %rdx
141e235aa:     	je	0x141e235be <.text+0x1e225be>
141e235ac:     	movq	0x708(%rbp), %rcx
141e235b3:     	movl	$0x1, %r8d
141e235b9:     	callq	0x140613c20 <.text+0x612c20>
141e235be:     	movaps	0x70(%rsp), %xmm6
141e235c3:     	movdqa	0x60(%rsp), %xmm7
141e235c9:     	movaps	0x50(%rsp), %xmm8
141e235cf:     	addq	$0x88, %rsp
141e235d6:     	popq	%rbx
141e235d7:     	popq	%rdi
141e235d8:     	popq	%rsi
141e235d9:     	popq	%r12
141e235db:     	popq	%r13
141e235dd:     	popq	%r14
141e235df:     	popq	%r15
141e235e1:     	popq	%rbp
141e235e2:     	retq
141e235e3:     	nopw	%cs:(%rax,%rax)
141e235f0:     	movq	%rdx, 0x10(%rsp)
141e235f5:     	pushq	%rbp
141e235f6:     	pushq	%r15
141e235f8:     	pushq	%r14
141e235fa:     	pushq	%r13
141e235fc:     	pushq	%r12
141e235fe:     	pushq	%rsi
141e235ff:     	pushq	%rdi
141e23600:     	pushq	%rbx
141e23601:     	subq	$0x88, %rsp
141e23608:     	leaq	0x80(%rdx), %rbp
141e2360f:     	movaps	%xmm8, 0x50(%rsp)
141e23615:     	movaps	%xmm7, 0x60(%rsp)
141e2361a:     	movdqa	%xmm6, 0x70(%rsp)
141e23620:     	cmpq	$0x0, 0x2718(%rbp)
141e23628:     	je	0x141e23643 <.text+0x1e22643>
141e2362a:     	movl	$0x1, %r8d
141e23630:     	movq	0x26e0(%rbp), %rcx
141e23637:     	movq	0x2718(%rbp), %rdx
141e2363e:     	callq	0x140613c20 <.text+0x612c20>
141e23643:     	movb	$0x1, 0x2735(%rbp)
141e2364a:     	movdqa	0x70(%rsp), %xmm6
141e23650:     	movdqa	0x60(%rsp), %xmm7
141e23656:     	movaps	0x50(%rsp), %xmm8
141e2365c:     	addq	$0x88, %rsp
141e23663:     	popq	%rbx
141e23664:     	popq	%rdi
141e23665:     	popq	%rsi
141e23666:     	popq	%r12
141e23668:     	popq	%r13
141e2366a:     	popq	%r14
141e2366c:     	popq	%r15
141e2366e:     	popq	%rbp
141e2366f:     	retq
141e23670:     	movq	%rdx, 0x10(%rsp)
141e23675:     	pushq	%rbp
141e23676:     	pushq	%r15
141e23678:     	pushq	%r14
141e2367a:     	pushq	%r13
141e2367c:     	pushq	%r12
141e2367e:     	pushq	%rsi
141e2367f:     	pushq	%rdi
141e23680:     	pushq	%rbx
141e23681:     	subq	$0x88, %rsp
141e23688:     	leaq	0x80(%rdx), %rbp
141e2368f:     	movaps	%xmm8, 0x50(%rsp)
141e23695:     	movdqa	%xmm7, 0x60(%rsp)
141e2369b:     	movdqa	%xmm6, 0x70(%rsp)
141e236a1:     	leaq	0x700(%rbp), %rcx
141e236a8:     	callq	0x14014b4d0 <.text+0x14a4d0>
141e236ad:     	movb	$0x1, 0x2735(%rbp)
141e236b4:     	movdqa	0x70(%rsp), %xmm6
141e236ba:     	movdqa	0x60(%rsp), %xmm7
141e236c0:     	movaps	0x50(%rsp), %xmm8
141e236c6:     	addq	$0x88, %rsp
141e236cd:     	popq	%rbx
141e236ce:     	popq	%rdi
141e236cf:     	popq	%rsi
141e236d0:     	popq	%r12
141e236d2:     	popq	%r13
141e236d4:     	popq	%r14
141e236d6:     	popq	%r15
141e236d8:     	popq	%rbp
141e236d9:     	retq
141e236da:     	nopw	(%rax,%rax)
141e236e0:     	movq	%rdx, 0x10(%rsp)
141e236e5:     	pushq	%rbp
141e236e6:     	pushq	%r15
141e236e8:     	pushq	%r14
141e236ea:     	pushq	%r13
141e236ec:     	pushq	%r12
141e236ee:     	pushq	%rsi
141e236ef:     	pushq	%rdi
141e236f0:     	pushq	%rbx
141e236f1:     	subq	$0x88, %rsp
141e236f8:     	leaq	0x80(%rdx), %rbp
141e236ff:     	movaps	%xmm8, 0x50(%rsp)
141e23705:     	movdqa	%xmm7, 0x60(%rsp)
141e2370b:     	movdqa	%xmm6, 0x70(%rsp)
141e23711:     	leaq	0x718(%rbp), %rcx
141e23718:     	callq	0x140fb63f0 <.text+0xfb53f0>
141e2371d:     	movb	$0x0, 0x2735(%rbp)
141e23724:     	movdqa	0x70(%rsp), %xmm6
141e2372a:     	movdqa	0x60(%rsp), %xmm7
141e23730:     	movaps	0x50(%rsp), %xmm8
141e23736:     	addq	$0x88, %rsp
141e2373d:     	popq	%rbx
141e2373e:     	popq	%rdi
141e2373f:     	popq	%rsi
141e23740:     	popq	%r12
141e23742:     	popq	%r13
141e23744:     	popq	%r14
141e23746:     	popq	%r15
141e23748:     	popq	%rbp
141e23749:     	retq
141e2374a:     	nopw	(%rax,%rax)
141e23750:     	movq	%rdx, 0x10(%rsp)
141e23755:     	pushq	%rbp
141e23756:     	pushq	%r15
141e23758:     	pushq	%r14
141e2375a:     	pushq	%r13
141e2375c:     	pushq	%r12
141e2375e:     	pushq	%rsi
141e2375f:     	pushq	%rdi
141e23760:     	pushq	%rbx
141e23761:     	subq	$0x88, %rsp
141e23768:     	leaq	0x80(%rdx), %rbp
141e2376f:     	movaps	%xmm8, 0x50(%rsp)
141e23775:     	movdqa	%xmm7, 0x60(%rsp)
141e2377b:     	movdqa	%xmm6, 0x70(%rsp)
141e23781:     	cmpb	$0x0, 0x2735(%rbp)
141e23788:     	je	0x141e237c2 <.text+0x1e227c2>
141e2378a:     	leaq	0x25b0(%rbp), %rcx
141e23791:     	callq	0x140faa000 <.text+0xfa9000>
141e23796:     	movb	$0x1, 0x2736(%rbp)
141e2379d:     	movdqa	0x70(%rsp), %xmm6
141e237a3:     	movaps	0x60(%rsp), %xmm7
141e237a8:     	movaps	0x50(%rsp), %xmm8
141e237ae:     	addq	$0x88, %rsp
141e237b5:     	popq	%rbx
141e237b6:     	popq	%rdi
141e237b7:     	popq	%rsi
141e237b8:     	popq	%r12
141e237ba:     	popq	%r13
141e237bc:     	popq	%r14
141e237be:     	popq	%r15
141e237c0:     	popq	%rbp
141e237c1:     	retq
141e237c2:     	movb	$0x1, 0x2736(%rbp)
141e237c9:     	movdqa	0x70(%rsp), %xmm6
141e237cf:     	movaps	0x60(%rsp), %xmm7
141e237d4:     	movaps	0x50(%rsp), %xmm8
141e237da:     	addq	$0x88, %rsp
141e237e1:     	popq	%rbx
141e237e2:     	popq	%rdi
141e237e3:     	popq	%rsi
141e237e4:     	popq	%r12
141e237e6:     	popq	%r13
141e237e8:     	popq	%r14
141e237ea:     	popq	%r15
141e237ec:     	popq	%rbp
141e237ed:     	retq
141e237ee:     	nop
141e237f0:     	movq	%rdx, 0x10(%rsp)
141e237f5:     	pushq	%rbp
141e237f6:     	pushq	%r15
141e237f8:     	pushq	%r14
141e237fa:     	pushq	%r13
141e237fc:     	pushq	%r12
141e237fe:     	pushq	%rsi
141e237ff:     	pushq	%rdi
141e23800:     	pushq	%rbx
141e23801:     	subq	$0x88, %rsp
141e23808:     	leaq	0x80(%rdx), %rbp
141e2380f:     	movaps	%xmm8, 0x50(%rsp)
141e23815:     	movaps	%xmm7, 0x60(%rsp)
141e2381a:     	movdqa	%xmm6, 0x70(%rsp)
141e23820:     	cmpq	$0x0, 0x2708(%rbp)
141e23828:     	je	0x141e23843 <.text+0x1e22843>
141e2382a:     	movl	$0x1, %r8d
141e23830:     	movq	0x2718(%rbp), %rcx
141e23837:     	movq	0x2708(%rbp), %rdx
141e2383e:     	callq	0x140613c20 <.text+0x612c20>
141e23843:     	leaq	0x25b0(%rbp), %rcx
141e2384a:     	callq	0x140d4cc30 <.text+0xd4bc30>
141e2384f:     	movb	$0x0, 0x2736(%rbp)
141e23856:     	movdqa	0x70(%rsp), %xmm6
141e2385c:     	movaps	0x60(%rsp), %xmm7
141e23861:     	movaps	0x50(%rsp), %xmm8
141e23867:     	addq	$0x88, %rsp
141e2386e:     	popq	%rbx
141e2386f:     	popq	%rdi
141e23870:     	popq	%rsi
141e23871:     	popq	%r12
141e23873:     	popq	%r13
141e23875:     	popq	%r14
141e23877:     	popq	%r15
141e23879:     	popq	%rbp
141e2387a:     	retq
141e2387b:     	nopl	(%rax,%rax)
141e23880:     	movq	%rdx, 0x10(%rsp)
141e23885:     	pushq	%rbp
141e23886:     	pushq	%r15
141e23888:     	pushq	%r14
141e2388a:     	pushq	%r13
141e2388c:     	pushq	%r12
141e2388e:     	pushq	%rsi
141e2388f:     	pushq	%rdi
141e23890:     	pushq	%rbx
141e23891:     	subq	$0x88, %rsp
141e23898:     	leaq	0x80(%rdx), %rbp
141e2389f:     	movaps	%xmm8, 0x50(%rsp)
141e238a5:     	movaps	%xmm7, 0x60(%rsp)
141e238aa:     	movdqa	%xmm6, 0x70(%rsp)
141e238b0:     	cmpb	$0x0, 0x2736(%rbp)
141e238b7:     	je	0x141e238fc <.text+0x1e228fc>
141e238b9:     	movq	0x26c0(%rbp), %rdx
141e238c0:     	testq	%rdx, %rdx
141e238c3:     	je	0x141e238d7 <.text+0x1e228d7>
141e238c5:     	movq	0x26c8(%rbp), %rcx
141e238cc:     	movl	$0x1, %r8d
141e238d2:     	callq	0x140613c20 <.text+0x612c20>
141e238d7:     	movaps	0x70(%rsp), %xmm6
141e238dc:     	movdqa	0x60(%rsp), %xmm7
141e238e2:     	movaps	0x50(%rsp), %xmm8
141e238e8:     	addq	$0x88, %rsp
141e238ef:     	popq	%rbx
141e238f0:     	popq	%rdi
141e238f1:     	popq	%rsi
141e238f2:     	popq	%r12
141e238f4:     	popq	%r13
141e238f6:     	popq	%r14
141e238f8:     	popq	%r15
141e238fa:     	popq	%rbp
141e238fb:     	retq
141e238fc:     	movaps	0x70(%rsp), %xmm6
141e23901:     	movdqa	0x60(%rsp), %xmm7
141e23907:     	movaps	0x50(%rsp), %xmm8
141e2390d:     	addq	$0x88, %rsp
141e23914:     	popq	%rbx
141e23915:     	popq	%rdi
141e23916:     	popq	%rsi
141e23917:     	popq	%r12
141e23919:     	popq	%r13
141e2391b:     	popq	%r14
141e2391d:     	popq	%r15
141e2391f:     	popq	%rbp
141e23920:     	retq
141e23921:     	nopw	%cs:(%rax,%rax)
141e23930:     	movq	%rdx, 0x10(%rsp)
141e23935:     	pushq	%rbp
141e23936:     	pushq	%r15
141e23938:     	pushq	%r14
141e2393a:     	pushq	%r13
141e2393c:     	pushq	%r12
141e2393e:     	pushq	%rsi
141e2393f:     	pushq	%rdi
141e23940:     	pushq	%rbx
141e23941:     	subq	$0x88, %rsp
141e23948:     	leaq	0x80(%rdx), %rbp
141e2394f:     	movaps	%xmm8, 0x50(%rsp)
141e23955:     	movdqa	%xmm7, 0x60(%rsp)
141e2395b:     	movaps	%xmm6, 0x70(%rsp)
141e23960:     	movq	0x2610(%rbp), %rdx
141e23967:     	testq	%rdx, %rdx
141e2396a:     	je	0x141e2397e <.text+0x1e2297e>
141e2396c:     	movq	0x2618(%rbp), %rcx
141e23973:     	movl	$0x1, %r8d
141e23979:     	callq	0x140613c20 <.text+0x612c20>
141e2397e:     	movaps	0x70(%rsp), %xmm6
141e23983:     	movaps	0x60(%rsp), %xmm7
141e23988:     	movaps	0x50(%rsp), %xmm8
141e2398e:     	addq	$0x88, %rsp
141e23995:     	popq	%rbx
141e23996:     	popq	%rdi
141e23997:     	popq	%rsi
141e23998:     	popq	%r12
141e2399a:     	popq	%r13
141e2399c:     	popq	%r14
141e2399e:     	popq	%r15
141e239a0:     	popq	%rbp
141e239a1:     	retq
141e239a2:     	nopw	%cs:(%rax,%rax)
141e239b0:     	movq	%rdx, 0x10(%rsp)
141e239b5:     	pushq	%rbp
141e239b6:     	pushq	%r15
141e239b8:     	pushq	%r14
141e239ba:     	pushq	%r13
141e239bc:     	pushq	%r12
141e239be:     	pushq	%rsi
141e239bf:     	pushq	%rdi
141e239c0:     	pushq	%rbx
141e239c1:     	subq	$0x88, %rsp
141e239c8:     	leaq	0x80(%rdx), %rbp
141e239cf:     	movaps	%xmm8, 0x50(%rsp)
141e239d5:     	movdqa	%xmm7, 0x60(%rsp)
141e239db:     	movaps	%xmm6, 0x70(%rsp)
141e239e0:     	movq	0x2728(%rbp), %rax
141e239e7:     	incq	(%rax)
141e239ea:     	movaps	0x70(%rsp), %xmm6
141e239ef:     	movaps	0x60(%rsp), %xmm7
141e239f4:     	movaps	0x50(%rsp), %xmm8
141e239fa:     	addq	$0x88, %rsp
141e23a01:     	popq	%rbx
141e23a02:     	popq	%rdi
141e23a03:     	popq	%rsi
141e23a04:     	popq	%r12
141e23a06:     	popq	%r13
141e23a08:     	popq	%r14
141e23a0a:     	popq	%r15
141e23a0c:     	popq	%rbp
141e23a0d:     	retq
141e23a0e:     	nop
141e23a10:     	movq	%rdx, 0x10(%rsp)
141e23a15:     	pushq	%rbp
141e23a16:     	pushq	%r15
141e23a18:     	pushq	%r14
141e23a1a:     	pushq	%r13
141e23a1c:     	pushq	%r12
141e23a1e:     	pushq	%rsi
141e23a1f:     	pushq	%rdi
141e23a20:     	pushq	%rbx
141e23a21:     	subq	$0x88, %rsp
141e23a28:     	leaq	0x80(%rdx), %rbp
141e23a2f:     	movaps	%xmm8, 0x50(%rsp)
141e23a35:     	movdqa	%xmm7, 0x60(%rsp)
141e23a3b:     	movaps	%xmm6, 0x70(%rsp)
141e23a40:     	leaq	0x1ce0(%rbp), %rcx
141e23a47:     	callq	0x14014b4d0 <.text+0x14a4d0>
141e23a4c:     	movaps	0x70(%rsp), %xmm6
141e23a51:     	movaps	0x60(%rsp), %xmm7
141e23a56:     	movaps	0x50(%rsp), %xmm8
141e23a5c:     	addq	$0x88, %rsp
141e23a63:     	popq	%rbx
141e23a64:     	popq	%rdi
141e23a65:     	popq	%rsi
141e23a66:     	popq	%r12
141e23a68:     	popq	%r13
141e23a6a:     	popq	%r14
141e23a6c:     	popq	%r15
141e23a6e:     	popq	%rbp
141e23a6f:     	retq
141e23a70:     	movq	%rdx, 0x10(%rsp)
141e23a75:     	pushq	%rbp
141e23a76:     	pushq	%r15
141e23a78:     	pushq	%r14
141e23a7a:     	pushq	%r13
141e23a7c:     	pushq	%r12
141e23a7e:     	pushq	%rsi
141e23a7f:     	pushq	%rdi
141e23a80:     	pushq	%rbx
141e23a81:     	subq	$0x88, %rsp
141e23a88:     	leaq	0x80(%rdx), %rbp
141e23a8f:     	movaps	%xmm8, 0x50(%rsp)
141e23a95:     	movdqa	%xmm7, 0x60(%rsp)
141e23a9b:     	movaps	%xmm6, 0x70(%rsp)
141e23aa0:     	leaq	0x700(%rbp), %rcx
141e23aa7:     	callq	0x140fb62b0 <.text+0xfb52b0>
141e23aac:     	movaps	0x70(%rsp), %xmm6
141e23ab1:     	movaps	0x60(%rsp), %xmm7
141e23ab6:     	movaps	0x50(%rsp), %xmm8
141e23abc:     	addq	$0x88, %rsp
141e23ac3:     	popq	%rbx
141e23ac4:     	popq	%rdi
141e23ac5:     	popq	%rsi
141e23ac6:     	popq	%r12
141e23ac8:     	popq	%r13
141e23aca:     	popq	%r14
141e23acc:     	popq	%r15
141e23ace:     	popq	%rbp
141e23acf:     	retq
141e23ad0:     	movq	%rdx, 0x10(%rsp)
141e23ad5:     	pushq	%rbp
141e23ad6:     	pushq	%r15
141e23ad8:     	pushq	%r14
141e23ada:     	pushq	%r13
141e23adc:     	pushq	%r12
141e23ade:     	pushq	%rsi
141e23adf:     	pushq	%rdi
141e23ae0:     	pushq	%rbx
141e23ae1:     	subq	$0x88, %rsp
141e23ae8:     	leaq	0x80(%rdx), %rbp
141e23aef:     	movaps	%xmm8, 0x50(%rsp)
141e23af5:     	movdqa	%xmm7, 0x60(%rsp)
141e23afb:     	movaps	%xmm6, 0x70(%rsp)
141e23b00:     	leaq	0x2610(%rbp), %rcx
141e23b07:     	callq	0x14014b4d0 <.text+0x14a4d0>
141e23b0c:     	movaps	0x70(%rsp), %xmm6
141e23b11:     	movaps	0x60(%rsp), %xmm7
141e23b16:     	movaps	0x50(%rsp), %xmm8
141e23b1c:     	addq	$0x88, %rsp
141e23b23:     	popq	%rbx
141e23b24:     	popq	%rdi
141e23b25:     	popq	%rsi
141e23b26:     	popq	%r12
141e23b28:     	popq	%r13
141e23b2a:     	popq	%r14
141e23b2c:     	popq	%r15
141e23b2e:     	popq	%rbp
141e23b2f:     	retq
141e23b30:     	movq	%rdx, 0x10(%rsp)
141e23b35:     	pushq	%rbp
141e23b36:     	pushq	%r15
141e23b38:     	pushq	%r14
141e23b3a:     	pushq	%r13
141e23b3c:     	pushq	%r12
141e23b3e:     	pushq	%rsi
141e23b3f:     	pushq	%rdi
141e23b40:     	pushq	%rbx
141e23b41:     	subq	$0x88, %rsp
141e23b48:     	leaq	0x80(%rdx), %rbp
141e23b4f:     	movaps	%xmm8, 0x50(%rsp)
141e23b55:     	movdqa	%xmm7, 0x60(%rsp)
141e23b5b:     	movaps	%xmm6, 0x70(%rsp)
141e23b60:     	leaq	0x1ce0(%rbp), %rcx
141e23b67:     	callq	0x140877160 <.text+0x876160>
141e23b6c:     	movaps	0x70(%rsp), %xmm6
141e23b71:     	movaps	0x60(%rsp), %xmm7
141e23b76:     	movaps	0x50(%rsp), %xmm8
141e23b7c:     	addq	$0x88, %rsp
141e23b83:     	popq	%rbx
141e23b84:     	popq	%rdi
141e23b85:     	popq	%rsi
141e23b86:     	popq	%r12
141e23b88:     	popq	%r13
141e23b8a:     	popq	%r14
141e23b8c:     	popq	%r15
141e23b8e:     	popq	%rbp
141e23b8f:     	retq
