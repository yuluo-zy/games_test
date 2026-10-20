
D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141471d3c:     	movq	0x21ba665(%rip), %rax   # 0x14362c3a8
141471d43:     	movq	(%rax), %rax
141471d46:     	cmpq	$0x3, %rax
141471d4a:     	jb	0x141471e11 <.text+0x1470e11>
141471d50:     	leaq	0x300(%rbp), %rax
141471d57:     	movq	%rax, 0x250(%rbp)
141471d5e:     	leaq	-0x6e47e5(%rip), %rax   # 0x140d8d580 <.text+0xd8c580>
141471d65:     	movq	%rax, 0x258(%rbp)
141471d6c:     	leaq	0x17f286d(%rip), %rcx   # 0x142c645e0
141471d73:     	movq	%rcx, 0x260(%rbp)
141471d7a:     	movq	%rax, 0x268(%rbp)
141471d81:     	leaq	0x17f28c0(%rip), %rax   # 0x142c64648
141471d88:     	movq	%rax, 0x160(%rbp)
141471d8f:     	movq	$0x3, 0x168(%rbp)
141471d9a:     	movq	$0x0, 0x180(%rbp)
141471da5:     	movq	%r14, 0x170(%rbp)
141471dac:     	movq	$0x2, 0x178(%rbp)
141471db7:     	leaq	0x17f28ba(%rip), %rcx   # 0x142c64678
141471dbe:     	callq	0x14067dd80 <.text+0x67cd80>
141471dc3:     	leaq	0x17f237e(%rip), %rcx   # 0x142c64148
141471dca:     	movq	%rcx, 0x270(%rbp)
141471dd1:     	movq	$0x26, 0x278(%rbp)
141471ddc:     	movq	%rcx, 0x280(%rbp)
141471de3:     	movq	$0x26, 0x288(%rbp)
141471dee:     	movq	%rax, 0x290(%rbp)
141471df5:     	leaq	0x160(%rbp), %rcx
141471dfc:     	leaq	0x270(%rbp), %r8
141471e03:     	movl	$0x3, %edx
141471e08:     	callq	0x1404b1430 <.text+0x4b0430>
141471e0d:     	movq	0x60(%rbp), %rdx
141471e11:     	movss	%xmm15, 0x134(%rbp)
141471e1a:     	movaps	%xmm9, -0x10(%rbp)
141471e1f:     	movaps	%xmm7, -0x20(%rbp)
141471e23:     	movq	%rdi, 0x38(%rbp)
141471e27:     	cmpl	$0x10, %r13d
141471e2b:     	setb	0x336(%rbp)
141471e32:     	movb	$0x1, 0xd8(%rbp)
141471e39:     	movq	$0x0, 0xb0(%rbp)
141471e44:     	movq	0x68(%rbp), %rax
141471e48:     	leaq	(%rax,%rax,4), %rax
141471e4c:     	leaq	(%rdx,%rax,8), %rax
141471e50:     	movq	%rax, 0x20(%rbp)
141471e54:     	movq	$-0x1, %r10
141471e5b:     	movabsq	$-0x7ffffffffffffff2, %r14 # imm = 0x800000000000000E
141471e65:     	movl	$0x2, %edi
141471e6a:     	movss	0x161059d(%rip), %xmm15 # 0x142a82410
141471e73:     	movss	0x14b3a89(%rip), %xmm6  # 0x142925904
141471e7b:     	movss	0x14ee7f1(%rip), %xmm7  # 0x142960674
141471e83:     	movss	0x14dab84(%rip), %xmm9  # 0x14294ca10
141471e8c:     	movl	$0x0, 0x304(%rbp)
141471e96:     	movl	$0x0, 0x2f0(%rbp)
141471ea0:     	movl	%r13d, 0x330(%rbp)
141471ea7:     	movq	%r12, 0x110(%rbp)
141471eae:     	cmpq	0x20(%rbp), %rdx
141471eb2:     	je	0x141472f5e <.text+0x1471f5e>
141471eb8:     	movq	%rdx, 0x2f8(%rbp)
141471ebf:     	movq	0x10(%rdx), %rax
141471ec3:     	testq	%rax, %rax
141471ec6:     	je	0x141472331 <.text+0x1471331>
141471ecc:     	movq	0x2f8(%rbp), %rcx
141471ed3:     	movq	0x8(%rcx), %rbx
141471ed7:     	leaq	(%rax,%rax,2), %rax
141471edb:     	leaq	(%rbx,%rax,8), %r15
141471edf:     	movq	%r15, 0x8(%rbp)
141471ee3:     	jmp	0x141471ef7 <.text+0x1470ef7>
141471ee5:     	movl	%r8d, 0x10(%rax,%rcx)
141471eea:     	addq	$0x18, %rbx
141471eee:     	cmpq	%r15, %rbx
141471ef1:     	je	0x141472331 <.text+0x1471331>
141471ef7:     	movq	(%rbx), %rax
141471efa:     	cmpq	$0x7, %rax
141471efe:     	jne	0x141472147 <.text+0x1471147>
141471f04:     	movq	0x10(%rbx), %rsi
141471f08:     	testq	%rsi, %rsi
141471f0b:     	je	0x1414721f0 <.text+0x14711f0>
141471f11:     	movq	%rbx, (%rbp)
141471f15:     	movq	0x8(%rbx), %r15
141471f19:     	shlq	$0x4, %rsi
141471f1d:     	addq	%r15, %rsi
141471f20:     	jmp	0x141471f3d <.text+0x1470f3d>
141471f22:     	nopw	%cs:(%rax,%rax)
141471f30:     	addq	$0x10, %r15
141471f34:     	cmpq	%rsi, %r15
141471f37:     	je	0x14147212e <.text+0x147112e>
141471f3d:     	movq	0x8(%r15), %r12
141471f41:     	testq	%r12, %r12
141471f44:     	je	0x141471f30 <.text+0x1470f30>
141471f46:     	movq	(%r15), %rbx
141471f49:     	shlq	$0x3, %r12
141471f4d:     	xorl	%r13d, %r13d
141471f50:     	jmp	0x141471f6f <.text+0x1470f6f>
141471f52:     	xorl	%eax, %eax
141471f54:     	nopw	%cs:(%rax,%rax)
141471f60:     	orb	%al, 0x336(%rbp)
141471f66:     	addq	$0x8, %r13
141471f6a:     	cmpq	%r13, %r12
141471f6d:     	je	0x141471f30 <.text+0x1470f30>
141471f6f:     	movq	(%rbx,%r13), %rcx
141471f73:     	movl	$0x1, %eax
141471f78:     	lock
141471f79:     	cmpxchgq	%r10, 0x8(%rcx)
141471f7e:     	jne	0x1414732b8 <.text+0x14722b8>
141471f84:     	movq	(%rcx), %rax
141471f87:     	movq	$0x1, 0x8(%rcx)
141471f8f:     	cmpq	$0x1, %rax
141471f93:     	jne	0x1414732b8 <.text+0x14722b8>
141471f99:     	movq	(%rbx,%r13), %r8
141471f9d:     	movq	0x10(%r8), %rcx
141471fa1:     	cmpq	%r14, %rcx
141471fa4:     	jne	0x141472090 <.text+0x1471090>
141471faa:     	leaq	0x10(%r8), %rcx
141471fae:     	movq	0x18(%r8), %rax
141471fb2:     	movq	%rax, 0x100(%rbp)
141471fb9:     	movl	0x20(%r8), %r9d
141471fbd:     	leaq	0x24(%r8), %rax
141471fc1:     	movq	%rax, 0x318(%rbp)
141471fc8:     	cmpb	$0x0, 0x3c(%r8)
141471fcd:     	leaq	0x160(%rbp), %rax
141471fd4:     	leaq	0x270(%rbp), %rdx
141471fdb:     	cmovneq	%rdx, %rax
141471fdf:     	movl	%r9d, %edx
141471fe2:     	cmovnel	%edi, %edx
141471fe5:     	movl	%edx, 0x158(%rbp)
141471feb:     	cmovel	%edi, %r9d
141471fef:     	movl	%r9d, 0x320(%rbp)
141471ff6:     	movq	0x34(%r8), %rdx
141471ffa:     	movq	%rdx, 0x10(%rax)
141471ffe:     	movups	0x24(%r8), %xmm0
141472003:     	movups	%xmm0, (%rax)
141472006:     	movq	%r8, 0x308(%rbp)
14147200d:     	callq	0x140a82940 <.text+0xa81940>
141472012:     	movabsq	$-0x7ffffffffffffffd, %rax # imm = 0x8000000000000003
14147201c:     	movq	0x308(%rbp), %rdx
141472023:     	movq	%rax, 0x10(%rdx)
141472027:     	movq	0x100(%rbp), %rax
14147202e:     	movq	%rax, 0x18(%rdx)
141472032:     	movl	0x158(%rbp), %eax
141472038:     	movl	%eax, 0x20(%rdx)
14147203b:     	movq	0x170(%rbp), %rax
141472042:     	movq	0x318(%rbp), %rcx
141472049:     	movq	%rax, 0x10(%rcx)
14147204d:     	movups	0x160(%rbp), %xmm0
141472054:     	movups	%xmm0, (%rcx)
141472057:     	movl	0x320(%rbp), %eax
14147205d:     	movl	%eax, 0x3c(%rdx)
141472060:     	movups	0x270(%rbp), %xmm0
141472067:     	movups	%xmm0, 0x40(%rdx)
14147206b:     	movq	0x280(%rbp), %rax
141472072:     	movq	%rax, 0x50(%rdx)
141472076:     	movb	$0x1, %al
141472078:     	movq	$-0x1, %r10
14147207f:     	jmp	0x141471f60 <.text+0x1470f60>
141472084:     	nopw	%cs:(%rax,%rax)
141472090:     	xorl	%eax, %eax
141472092:     	movabsq	$-0x7ffffffffffffff0, %rdx # imm = 0x8000000000000010
14147209c:     	cmpq	%rdx, %rcx
14147209f:     	jl	0x141471f60 <.text+0x1470f60>
1414720a5:     	cmpl	$0x2, 0xf8(%r8)
1414720ad:     	jae	0x141471f60 <.text+0x1470f60>
1414720b3:     	movss	0xfc(%r8), %xmm0
1414720bc:     	ucomiss	0x100(%r8), %xmm0
1414720c4:     	jne	0x141471f52 <.text+0x1470f52>
1414720ca:     	jp	0x141471f52 <.text+0x1470f52>
1414720d0:     	addss	%xmm15, %xmm0
1414720d5:     	ucomiss	%xmm0, %xmm0
1414720d8:     	jp	0x141472ef2 <.text+0x1471ef2>
1414720de:     	movd	%xmm0, %eax
1414720e2:     	andl	$0x7fffffff, %eax       # imm = 0x7FFFFFFF
1414720e7:     	cmpl	$0x7f800000, %eax       # imm = 0x7F800000
1414720ec:     	je	0x141472f16 <.text+0x1471f16>
1414720f2:     	xorps	%xmm1, %xmm1
1414720f5:     	maxss	%xmm0, %xmm1
1414720f9:     	movaps	%xmm6, %xmm0
1414720fc:     	minss	%xmm1, %xmm0
141472100:     	movss	%xmm0, 0xfc(%r8)
141472109:     	addss	%xmm7, %xmm0
14147210d:     	ucomiss	%xmm9, %xmm0
141472111:     	jae	0x141472f3a <.text+0x1471f3a>
141472117:     	movaps	%xmm6, %xmm1
14147211a:     	minss	%xmm0, %xmm1
14147211e:     	movss	%xmm1, 0x100(%r8)
141472127:     	movb	$0x1, %al
141472129:     	jmp	0x141471f60 <.text+0x1470f60>
14147212e:     	movq	(%rbp), %rbx
141472132:     	movq	(%rbx), %rax
141472135:     	movl	0x330(%rbp), %r13d
14147213c:     	movq	0x110(%rbp), %r12
141472143:     	movq	0x8(%rbp), %r15
141472147:     	testq	%rax, %rax
14147214a:     	jne	0x1414721f0 <.text+0x14711f0>
141472150:     	movq	0x8(%rbx), %rcx
141472154:     	movl	$0x1, %eax
141472159:     	lock
14147215a:     	cmpxchgq	%r10, 0x8(%rcx)
14147215f:     	jne	0x1414732d0 <.text+0x14722d0>
141472165:     	movq	(%rcx), %rax
141472168:     	movq	$0x1, 0x8(%rcx)
141472170:     	cmpq	$0x1, %rax
141472174:     	jne	0x1414732d0 <.text+0x14722d0>
14147217a:     	movq	0x8(%rbx), %rax
14147217e:     	xorl	%ecx, %ecx
141472180:     	cmpq	0x10(%rax), %rcx
141472184:     	jno	0x1414721f0 <.text+0x14711f0>
141472186:     	cmpq	$0x2, 0x28(%rax)
14147218b:     	jne	0x1414721f0 <.text+0x14711f0>
14147218d:     	movq	0x20(%rax), %rax
141472191:     	movss	0x8(%rax), %xmm2
141472196:     	movsd	0xc(%rax), %xmm1
14147219b:     	movss	0x14(%rax), %xmm0
1414721a0:     	subss	%xmm0, %xmm2
1414721a4:     	movsd	(%rax), %xmm3
1414721a8:     	subps	%xmm1, %xmm3
1414721ab:     	mulps	%xmm3, %xmm3
1414721ae:     	movshdup	%xmm3, %xmm4            # xmm4 = xmm3[1,1,3,3]
1414721b2:     	addss	%xmm3, %xmm4
1414721b6:     	mulss	%xmm2, %xmm2
1414721ba:     	addss	%xmm4, %xmm2
1414721be:     	sqrtss	%xmm2, %xmm2
1414721c2:     	ucomiss	0x14ee4c7(%rip), %xmm2  # 0x142960690
1414721c9:     	jne	0x1414721f0 <.text+0x14711f0>
1414721cb:     	jp	0x1414721f0 <.text+0x14711f0>
1414721cd:     	movss	0x155ae8b(%rip), %xmm2  # 0x1429cd060
1414721d5:     	addps	%xmm2, %xmm1
1414721d8:     	movlps	%xmm1, 0xc(%rax)
1414721dc:     	addss	0x14ee4ac(%rip), %xmm0  # 0x142960690
1414721e4:     	movss	%xmm0, 0x14(%rax)
1414721e9:     	movb	$0x1, 0x336(%rbp)
1414721f0:     	cmpl	$0x14, %r13d
1414721f4:     	jae	0x141471eea <.text+0x1470eea>
1414721fa:     	cmpl	$0x9, (%rbx)
1414721fd:     	jne	0x141471eea <.text+0x1470eea>
141472203:     	movq	0x8(%rbx), %rcx
141472207:     	movl	$0x1, %eax
14147220c:     	lock
14147220d:     	cmpxchgq	%r10, 0x8(%rcx)
141472212:     	jne	0x1414732e8 <.text+0x14722e8>
141472218:     	movq	(%rcx), %rax
14147221b:     	movq	$0x1, 0x8(%rcx)
141472223:     	cmpq	$0x1, %rax
141472227:     	jne	0x1414732e8 <.text+0x14722e8>
14147222d:     	movq	0x8(%rbx), %rax
141472231:     	movzbl	0x18(%rax), %ecx
141472235:     	cmpl	$0x4, %ecx
141472238:     	je	0x14147224f <.text+0x147124f>
14147223a:     	cmpl	$0x7, %ecx
14147223d:     	jne	0x141471eea <.text+0x1470eea>
141472243:     	movl	$0x14, %ecx
141472248:     	movl	$0x10, %edx
14147224d:     	jmp	0x141472259 <.text+0x1471259>
14147224f:     	movl	$0x1c, %ecx
141472254:     	movl	$0x18, %edx
141472259:     	movl	0x10(%rax,%rdx), %r9d
14147225e:     	cmpq	$0x8, %r9
141472262:     	ja	0x1414722a6 <.text+0x14712a6>
141472264:     	leaq	0x17f2b05(%rip), %r11   # 0x142c64d70
14147226b:     	movslq	(%r11,%r9,4), %r8
14147226f:     	addq	%r11, %r8
141472272:     	jmpq	*%r8
141472275:     	movl	%r9d, %r8d
141472278:     	movl	%r8d, 0x10(%rax,%rdx)
14147227d:     	movl	0x10(%rax,%rcx), %r8d
141472282:     	cmpq	$0x8, %r8
141472286:     	ja	0x1414722b7 <.text+0x14712b7>
141472288:     	leaq	0x17f2b05(%rip), %r9    # 0x142c64d94
14147228f:     	movslq	(%r9,%r8,4), %rdx
141472293:     	addq	%r9, %rdx
141472296:     	jmpq	*%rdx
141472298:     	movl	$0x2, %edx
14147229d:     	movl	%edx, 0x10(%rax,%rcx)
1414722a1:     	jmp	0x141471eea <.text+0x1470eea>
1414722a6:     	movl	$0x1, %r8d
1414722ac:     	cmpl	$0x3e7, %r9d            # imm = 0x3E7
1414722b3:     	je	0x141472275 <.text+0x1471275>
1414722b5:     	jmp	0x141472278 <.text+0x1471278>
1414722b7:     	movl	$0x1, %edx
1414722bc:     	cmpl	$0x3e7, %r8d            # imm = 0x3E7
1414722c3:     	je	0x141471ee5 <.text+0x1470ee5>
1414722c9:     	jmp	0x141472328 <.text+0x1471328>
1414722cb:     	movl	$0x2, %r8d
1414722d1:     	jmp	0x141472278 <.text+0x1471278>
1414722d3:     	movl	$0x3, %r8d
1414722d9:     	jmp	0x141472278 <.text+0x1471278>
1414722db:     	movl	$0x3, %edx
1414722e0:     	movl	%edx, 0x10(%rax,%rcx)
1414722e4:     	jmp	0x141471eea <.text+0x1470eea>
1414722e9:     	movl	$0x5, %r8d
1414722ef:     	jmp	0x141472278 <.text+0x1471278>
1414722f1:     	movl	$0x5, %edx
1414722f6:     	movl	%edx, 0x10(%rax,%rcx)
1414722fa:     	jmp	0x141471eea <.text+0x1470eea>
1414722ff:     	movl	$0x8, %r8d
141472305:     	jmp	0x141472278 <.text+0x1471278>
14147230a:     	movl	$0x8, %edx
14147230f:     	movl	%edx, 0x10(%rax,%rcx)
141472313:     	jmp	0x141471eea <.text+0x1470eea>
141472318:     	movl	$0x4, %r8d
14147231e:     	jmp	0x141472278 <.text+0x1471278>
141472323:     	movl	$0x4, %edx
141472328:     	movl	%edx, 0x10(%rax,%rcx)
14147232c:     	jmp	0x141471eea <.text+0x1470eea>
141472331:     	cmpl	$0x10, %r13d
141472335:     	jae	0x1414724ac <.text+0x14714ac>
14147233b:     	movq	0x2f8(%rbp), %r8
141472342:     	movq	0x18(%r8), %rcx
141472346:     	movl	$0x1, %eax
14147234b:     	xorl	%edx, %edx
14147234d:     	lock
14147234e:     	cmpxchgq	%rdx, (%rcx)
141472352:     	leaq	0x18(%r8), %rsi
141472356:     	movq	0x18(%r8), %r15
14147235a:     	jne	0x141472376 <.text+0x1471376>
14147235c:     	movq	0x8(%r15), %rax
141472360:     	cmpq	$0x1, %rax
141472364:     	jne	0x1414727ba <.text+0x14717ba>
14147236a:     	movq	$0x1, (%r15)
141472371:     	jmp	0x141472499 <.text+0x1471499>
141472376:     	movq	%rsi, 0x318(%rbp)
14147237d:     	leaq	0x10(%r15), %rdx
141472381:     	movb	$0x1, 0x337(%rbp)
141472388:     	leaq	0x270(%rbp), %rcx
14147238f:     	movq	%rdx, 0x308(%rbp)
141472396:     	callq	0x140b5d170 <.text+0xb5c170>
14147239b:     	movaps	%xmm8, 0x100(%rbp)
1414723a3:     	movdqa	%xmm14, %xmm8
1414723a8:     	movaps	%xmm13, %xmm14
1414723ac:     	movaps	%xmm12, %xmm13
1414723b0:     	movaps	%xmm11, %xmm12
1414723b4:     	movaps	%xmm10, %xmm11
1414723b8:     	movq	0x270(%rbp), %rsi
1414723bf:     	movq	0x280(%rbp), %rbx
1414723c6:     	movq	%r15, 0x320(%rbp)
1414723cd:     	movups	0x30(%r15), %xmm10
1414723d2:     	movb	$0x1, 0x335(%rbp)
1414723d9:     	leaq	0x160(%rbp), %rcx
1414723e0:     	movq	0x308(%rbp), %rdx
1414723e7:     	callq	0x140b72b40 <.text+0xb71b40>
1414723ec:     	leaq	-0x1(%rsi), %rax
1414723f0:     	movq	%rsi, %rcx
1414723f3:     	xorq	%rax, %rcx
1414723f6:     	leaq	0xf(%rsi), %rdx
1414723fa:     	negq	%rsi
1414723fd:     	andq	%rdx, %rsi
141472400:     	addq	$-0x10, %rsi
141472404:     	cmpq	%rax, %rcx
141472407:     	movq	$-0x1, %rax
14147240e:     	cmovbeq	%rax, %rsi
141472412:     	movups	%xmm10, 0x180(%rbp)
14147241a:     	movups	0x180(%rbp), %xmm0
141472421:     	movups	%xmm0, 0x30(%rbx,%rsi)
141472426:     	movups	0x160(%rbp), %xmm0
14147242d:     	movups	0x170(%rbp), %xmm1
141472434:     	movups	%xmm1, 0x20(%rbx,%rsi)
141472439:     	movq	%rbx, 0x308(%rbp)
141472440:     	movups	%xmm0, 0x10(%rbx,%rsi)
141472445:     	testb	$0x1, 0x288(%rbp)
14147244c:     	je	0x141473338 <.text+0x1472338>
141472452:     	movq	0x320(%rbp), %rax
141472459:     	lock
14147245a:     	decq	(%rax)
14147245d:     	movaps	%xmm11, %xmm10
141472461:     	jne	0x14147246f <.text+0x147146f>
141472463:     	movq	0x318(%rbp), %rcx
14147246a:     	callq	0x140b5cb60 <.text+0xb5bb60>
14147246f:     	movq	0x318(%rbp), %rax
141472476:     	movq	0x308(%rbp), %r15
14147247d:     	movq	%r15, (%rax)
141472480:     	movaps	%xmm12, %xmm11
141472484:     	movaps	%xmm13, %xmm12
141472488:     	movaps	%xmm14, %xmm13
14147248c:     	movdqa	%xmm8, %xmm14
141472491:     	movaps	0x100(%rbp), %xmm8
141472499:     	addq	$0x10, %r15
14147249d:     	movb	$0x1, 0x337(%rbp)
1414724a4:     	movq	%r15, %rcx
1414724a7:     	callq	0x140b79bd0 <.text+0xb78bd0>
1414724ac:     	movb	$0x1, 0x337(%rbp)
1414724b3:     	movq	0x2f8(%rbp), %rcx
1414724ba:     	leaq	0x336(%rbp), %rdx
1414724c1:     	leaq	0x300(%rbp), %r8
1414724c8:     	callq	0x1412f50f0 <.text+0x12f40f0>
1414724cd:     	movq	0x2f8(%rbp), %rax
1414724d4:     	leaq	0x28(%rax), %rdx
1414724d8:     	cmpl	$0x14, %r13d
1414724dc:     	jae	0x141472877 <.text+0x1471877>
1414724e2:     	movq	0x18(%rax), %rcx
1414724e6:     	cmpq	$0x0, 0x28(%rcx)
1414724eb:     	movq	$-0x1, %r10
1414724f2:     	je	0x141471eae <.text+0x1470eae>
1414724f8:     	movl	$0x1, %eax
1414724fd:     	lock
1414724fe:     	cmpxchgq	%r10, 0x8(%rcx)
141472503:     	jne	0x14147335b <.text+0x147235b>
141472509:     	movq	(%rcx), %rax
14147250c:     	movq	$0x1, 0x8(%rcx)
141472514:     	cmpq	$0x1, %rax
141472518:     	jne	0x14147335b <.text+0x147235b>
14147251e:     	movq	0x2f8(%rbp), %rax
141472525:     	movq	0x18(%rax), %rax
141472529:     	movq	0x28(%rax), %rcx
14147252d:     	testq	%rcx, %rcx
141472530:     	je	0x14147288a <.text+0x147188a>
141472536:     	movq	0x10(%rax), %rbx
14147253a:     	leaq	0x10(%rbx), %rax
14147253e:     	movq	%rax, 0x158(%rbp)
141472545:     	movdqa	(%rbx), %xmm0
141472549:     	pmovmskb	%xmm0, %eax
14147254d:     	notl	%eax
14147254f:     	movq	%rax, 0x320(%rbp)
141472556:     	movq	%rdx, 0x100(%rbp)
14147255d:     	jmp	0x14147258b <.text+0x147158b>
14147255f:     	movq	0x318(%rbp), %rcx
141472566:     	decq	%rcx
141472569:     	movq	$-0x1, %r10
141472570:     	movq	0x2f8(%rbp), %rax
141472577:     	movq	0x100(%rbp), %rdx
14147257e:     	movq	0x308(%rbp), %rbx
141472585:     	je	0x14147287e <.text+0x147187e>
14147258b:     	movq	0x320(%rbp), %r8
141472592:     	testw	%r8w, %r8w
141472596:     	movq	%rcx, 0x318(%rbp)
14147259d:     	je	0x1414725a8 <.text+0x14715a8>
14147259f:     	leal	-0x1(%r8), %eax
1414725a3:     	andl	%r8d, %eax
1414725a6:     	jmp	0x1414725e3 <.text+0x14715e3>
1414725a8:     	movq	0x158(%rbp), %rax
1414725af:     	movdqa	(%rax), %xmm0
1414725b3:     	pmovmskb	%xmm0, %r8d
1414725b8:     	addq	$-0x700, %rbx           # imm = 0xF900
1414725bf:     	addq	$0x10, %rax
1414725c3:     	cmpl	$0xffff, %r8d           # imm = 0xFFFF
1414725ca:     	je	0x1414725af <.text+0x14715af>
1414725cc:     	movq	%rax, 0x158(%rbp)
1414725d3:     	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
1414725d8:     	subl	%r8d, %ecx
1414725db:     	notl	%r8d
1414725de:     	movl	%r8d, %eax
1414725e1:     	andl	%ecx, %eax
1414725e3:     	movl	%r8d, %ecx
1414725e6:     	tzcntl	%ecx, %ecx
1414725ea:     	movq	%rax, 0x320(%rbp)
1414725f1:     	negq	%rcx
1414725f4:     	imulq	$0x70, %rcx, %rcx
1414725f8:     	movq	-0x60(%rbx,%rcx), %r8
1414725fd:     	movq	-0x48(%rbx,%rcx), %r10
141472602:     	movdqa	(%r8), %xmm0
141472607:     	pmovmskb	%xmm0, %edx
14147260b:     	notl	%edx
14147260d:     	leaq	0x10(%r8), %r11
141472611:     	movq	-0x30(%rbx,%rcx), %rsi
141472616:     	movdqa	(%rsi), %xmm0
14147261a:     	pmovmskb	%xmm0, %r9d
14147261f:     	notl	%r9d
141472622:     	leaq	0x10(%rsi), %rax
141472626:     	movq	%rbx, 0x308(%rbp)
14147262d:     	movq	-0x18(%rbx,%rcx), %rcx
141472632:     	testq	%r8, %r8
141472635:     	jne	0x14147267e <.text+0x147167e>
141472637:     	jmp	0x1414726dc <.text+0x14716dc>
14147263c:     	leal	-0x1(%r9), %r12d
141472640:     	andl	%r9d, %r12d
141472643:     	decq	%rcx
141472646:     	movl	%r9d, %r15d
141472649:     	movq	%rsi, %rbx
14147264c:     	xorl	%r8d, %r8d
14147264f:     	movl	%r12d, %r9d
141472652:     	tzcntl	%r15d, %r15d
141472657:     	negq	%r15
14147265a:     	imulq	$0x58, %r15, %r15
14147265e:     	movzbl	-0x6(%rbx,%r15), %r12d
141472664:     	addb	$-0x38, %r12b
141472668:     	cmpb	$0x2, %r12b
14147266c:     	movq	0x110(%rbp), %r12
141472673:     	jbe	0x141472734 <.text+0x1471734>
141472679:     	testq	%r8, %r8
14147267c:     	je	0x1414726dc <.text+0x14716dc>
14147267e:     	testq	%r10, %r10
141472681:     	je	0x1414726d9 <.text+0x14716d9>
141472683:     	testw	%dx, %dx
141472686:     	je	0x1414726a0 <.text+0x14716a0>
141472688:     	leal	-0x1(%rdx), %r12d
14147268c:     	andl	%edx, %r12d
14147268f:     	decq	%r10
141472692:     	movl	%edx, %r15d
141472695:     	movq	%r8, %rbx
141472698:     	movl	%r12d, %edx
14147269b:     	jmp	0x141472652 <.text+0x1471652>
14147269d:     	nopl	(%rax)
1414726a0:     	movdqa	(%r11), %xmm0
1414726a5:     	pmovmskb	%xmm0, %r15d
1414726aa:     	addq	$-0x580, %r8            # imm = 0xFA80
1414726b1:     	addq	$0x10, %r11
1414726b5:     	cmpl	$0xffff, %r15d          # imm = 0xFFFF
1414726bc:     	je	0x1414726a0 <.text+0x14716a0>
1414726be:     	movl	$0xfffffffe, %ebx       # imm = 0xFFFFFFFE
1414726c3:     	subl	%r15d, %ebx
1414726c6:     	notl	%r15d
1414726c9:     	movl	%r15d, %edx
1414726cc:     	andl	%ebx, %edx
1414726ce:     	decq	%r10
1414726d1:     	movq	%r8, %rbx
1414726d4:     	jmp	0x141472652 <.text+0x1471652>
1414726d9:     	xorl	%r10d, %r10d
1414726dc:     	testq	%rsi, %rsi
1414726df:     	je	0x14147255f <.text+0x147155f>
1414726e5:     	testq	%rcx, %rcx
1414726e8:     	je	0x14147255f <.text+0x147155f>
1414726ee:     	testw	%r9w, %r9w
1414726f2:     	jne	0x14147263c <.text+0x147163c>
1414726f8:     	nopl	(%rax,%rax)
141472700:     	movdqa	(%rax), %xmm0
141472704:     	pmovmskb	%xmm0, %r9d
141472709:     	addq	$-0x580, %rsi           # imm = 0xFA80
141472710:     	addq	$0x10, %rax
141472714:     	cmpl	$0xffff, %r9d           # imm = 0xFFFF
14147271b:     	je	0x141472700 <.text+0x1471700>
14147271d:     	movl	$0xfffffffe, %r8d       # imm = 0xFFFFFFFE
141472723:     	subl	%r9d, %r8d
141472726:     	notl	%r9d
141472729:     	movl	%r9d, %r12d
14147272c:     	andl	%r8d, %r12d
14147272f:     	jmp	0x141472643 <.text+0x1471643>
141472734:     	addq	%r15, %rbx
141472737:     	movl	-0x28(%rbx), %r12d
14147273b:     	cmpq	$0x8, %r12
14147273f:     	ja	0x141472758 <.text+0x1471758>
141472741:     	leaq	0x17f2670(%rip), %r13   # 0x142c64db8
141472748:     	movslq	(%r13,%r12,4), %r15
14147274d:     	addq	%r13, %r15
141472750:     	jmpq	*%r15
141472753:     	movl	%r12d, %r15d
141472756:     	jmp	0x141472792 <.text+0x1471792>
141472758:     	movl	$0x1, %r15d
14147275e:     	cmpl	$0x3e7, %r12d           # imm = 0x3E7
141472765:     	jne	0x141472799 <.text+0x1471799>
141472767:     	movl	%r12d, %r15d
14147276a:     	jmp	0x141472799 <.text+0x1471799>
14147276c:     	movl	$0x2, %r15d
141472772:     	jmp	0x141472792 <.text+0x1471792>
141472774:     	movl	$0x3, %r15d
14147277a:     	jmp	0x141472792 <.text+0x1471792>
14147277c:     	movl	$0x5, %r15d
141472782:     	jmp	0x141472792 <.text+0x1471792>
141472784:     	movl	$0x8, %r15d
14147278a:     	jmp	0x141472792 <.text+0x1471792>
14147278c:     	movl	$0x4, %r15d
141472792:     	movl	0x330(%rbp), %r13d
141472799:     	movl	%r15d, -0x28(%rbx)
14147279d:     	movb	$0x1, %bl
14147279f:     	movl	%ebx, 0x2f0(%rbp)
1414727a5:     	movq	0x110(%rbp), %r12
1414727ac:     	testq	%r8, %r8
1414727af:     	jne	0x14147267e <.text+0x147167e>
1414727b5:     	jmp	0x1414726dc <.text+0x14716dc>
1414727ba:     	leaq	0x8(%r15), %rax
1414727be:     	movq	%rax, 0x318(%rbp)
1414727c5:     	movq	%r15, 0x320(%rbp)
1414727cc:     	leaq	0x10(%r15), %rbx
1414727d0:     	leaq	0x160(%rbp), %rcx
1414727d7:     	movq	%rbx, %rdx
1414727da:     	callq	0x140b5d170 <.text+0xb5c170>
1414727df:     	movq	0x160(%rbp), %rax
1414727e6:     	movq	0x170(%rbp), %r15
1414727ed:     	leaq	-0x1(%rax), %rcx
1414727f1:     	movq	%rax, %rdx
1414727f4:     	xorq	%rcx, %rdx
1414727f7:     	leaq	0xf(%rax), %r8
1414727fb:     	negq	%rax
1414727fe:     	andq	%r8, %rax
141472801:     	addq	$-0x10, %rax
141472805:     	cmpq	%rcx, %rdx
141472808:     	movq	$-0x1, %rcx
14147280f:     	cmovbeq	%rcx, %rax
141472813:     	movups	(%rbx), %xmm0
141472816:     	movups	0x10(%rbx), %xmm1
14147281a:     	movups	0x20(%rbx), %xmm2
14147281e:     	movups	%xmm2, 0x30(%r15,%rax)
141472824:     	movups	%xmm1, 0x20(%r15,%rax)
14147282a:     	movups	%xmm0, 0x10(%r15,%rax)
141472830:     	testb	$0x1, 0x178(%rbp)
141472837:     	je	0x14147334d <.text+0x147234d>
14147283d:     	movq	%r15, (%rsi)
141472840:     	movq	0x320(%rbp), %rcx
141472847:     	cmpq	$-0x1, %rcx
14147284b:     	je	0x141472499 <.text+0x1471499>
141472851:     	movq	0x318(%rbp), %rax
141472858:     	lock
141472859:     	decq	(%rax)
14147285c:     	jne	0x141472499 <.text+0x1471499>
141472862:     	movl	$0x40, %edx
141472867:     	movl	$0x8, %r8d
14147286d:     	callq	0x140613c20 <.text+0x612c20>
141472872:     	jmp	0x141472499 <.text+0x1471499>
141472877:     	movq	$-0x1, %r10
14147287e:     	cmpl	$0x15, %r13d
141472882:     	jae	0x141471eae <.text+0x1470eae>
141472888:     	jmp	0x141472891 <.text+0x1471891>
14147288a:     	movq	0x2f8(%rbp), %rax
141472891:     	movq	0x18(%rax), %rcx
141472895:     	cmpq	$0x0, 0x28(%rcx)
14147289a:     	je	0x141471eae <.text+0x1470eae>
1414728a0:     	movl	$0x1, %eax
1414728a5:     	lock
1414728a6:     	cmpxchgq	%r10, 0x8(%rcx)
1414728ab:     	jne	0x141473370 <.text+0x1472370>
1414728b1:     	movq	(%rcx), %rax
1414728b4:     	movq	$0x1, 0x8(%rcx)
1414728bc:     	cmpq	$0x1, %rax
1414728c0:     	jne	0x141473370 <.text+0x1472370>
1414728c6:     	movq	0x2f8(%rbp), %rax
1414728cd:     	movq	0x18(%rax), %rax
1414728d1:     	movq	0x28(%rax), %rcx
1414728d5:     	testq	%rcx, %rcx
1414728d8:     	je	0x141471eae <.text+0x1470eae>
1414728de:     	movq	%rdx, 0x100(%rbp)
1414728e5:     	movq	0x10(%rax), %rax
1414728e9:     	leaq	0x10(%rax), %rdx
1414728ed:     	movq	%rdx, 0x308(%rbp)
1414728f4:     	movq	%rax, 0x318(%rbp)
1414728fb:     	movdqa	(%rax), %xmm0
1414728ff:     	pmovmskb	%xmm0, %eax
141472903:     	notl	%eax
141472905:     	movq	%rax, 0x320(%rbp)
14147290c:     	movq	%rcx, %r13
14147290f:     	jmp	0x14147291a <.text+0x147191a>
141472911:     	decq	%r13
141472914:     	je	0x141472abd <.text+0x1471abd>
14147291a:     	movq	0x320(%rbp), %rdx
141472921:     	testw	%dx, %dx
141472924:     	je	0x14147292d <.text+0x147192d>
141472926:     	leal	-0x1(%rdx), %eax
141472929:     	andl	%edx, %eax
14147292b:     	jmp	0x141472970 <.text+0x1471970>
14147292d:     	movq	0x318(%rbp), %rax
141472934:     	movq	0x308(%rbp), %rcx
14147293b:     	movdqa	(%rcx), %xmm0
14147293f:     	pmovmskb	%xmm0, %edx
141472943:     	addq	$-0x700, %rax           # imm = 0xF900
141472949:     	addq	$0x10, %rcx
14147294d:     	cmpl	$0xffff, %edx           # imm = 0xFFFF
141472953:     	je	0x14147293b <.text+0x147193b>
141472955:     	movq	%rcx, 0x308(%rbp)
14147295c:     	movq	%rax, 0x318(%rbp)
141472963:     	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
141472968:     	subl	%edx, %ecx
14147296a:     	notl	%edx
14147296c:     	movl	%edx, %eax
14147296e:     	andl	%ecx, %eax
141472970:     	movl	%edx, %ecx
141472972:     	tzcntl	%ecx, %ecx
141472976:     	movq	%rax, 0x320(%rbp)
14147297d:     	negq	%rcx
141472980:     	imulq	$0x70, %rcx, %rax
141472984:     	movq	0x318(%rbp), %rbx
14147298b:     	movq	-0x60(%rbx,%rax), %r8
141472990:     	movq	-0x48(%rbx,%rax), %rcx
141472995:     	movdqa	(%r8), %xmm0
14147299a:     	pmovmskb	%xmm0, %edx
14147299e:     	notl	%edx
1414729a0:     	leaq	0x10(%r8), %r10
1414729a4:     	movq	-0x30(%rbx,%rax), %r11
1414729a9:     	movdqa	(%r11), %xmm0
1414729ae:     	pmovmskb	%xmm0, %r9d
1414729b3:     	notl	%r9d
1414729b6:     	leaq	0x10(%r11), %rsi
1414729ba:     	movq	-0x18(%rbx,%rax), %rax
1414729bf:     	testq	%r8, %r8
1414729c2:     	jne	0x1414729fb <.text+0x14719fb>
1414729c4:     	jmp	0x141472a4f <.text+0x1471a4f>
1414729c9:     	leal	-0x1(%r9), %r12d
1414729cd:     	andl	%r9d, %r12d
1414729d0:     	decq	%rax
1414729d3:     	movl	%r9d, %ebx
1414729d6:     	movq	%r11, %r15
1414729d9:     	xorl	%r8d, %r8d
1414729dc:     	movl	%r12d, %r9d
1414729df:     	tzcntl	%ebx, %ebx
1414729e3:     	negq	%rbx
1414729e6:     	imulq	$0x58, %rbx, %rbx
1414729ea:     	cmpb	$0x3a, -0x6(%r15,%rbx)
1414729f0:     	je	0x141472a9f <.text+0x1471a9f>
1414729f6:     	testq	%r8, %r8
1414729f9:     	je	0x141472a4f <.text+0x1471a4f>
1414729fb:     	testq	%rcx, %rcx
1414729fe:     	je	0x141472a4d <.text+0x1471a4d>
141472a00:     	testw	%dx, %dx
141472a03:     	je	0x141472a19 <.text+0x1471a19>
141472a05:     	leal	-0x1(%rdx), %r12d
141472a09:     	andl	%edx, %r12d
141472a0c:     	decq	%rcx
141472a0f:     	movl	%edx, %ebx
141472a11:     	movq	%r8, %r15
141472a14:     	movl	%r12d, %edx
141472a17:     	jmp	0x1414729df <.text+0x14719df>
141472a19:     	movdqa	(%r10), %xmm0
141472a1e:     	pmovmskb	%xmm0, %ebx
141472a22:     	addq	$-0x580, %r8            # imm = 0xFA80
141472a29:     	addq	$0x10, %r10
141472a2d:     	cmpl	$0xffff, %ebx           # imm = 0xFFFF
141472a33:     	je	0x141472a19 <.text+0x1471a19>
141472a35:     	movl	$0xfffffffe, %r15d      # imm = 0xFFFFFFFE
141472a3b:     	subl	%ebx, %r15d
141472a3e:     	notl	%ebx
141472a40:     	movl	%ebx, %edx
141472a42:     	andl	%r15d, %edx
141472a45:     	decq	%rcx
141472a48:     	movq	%r8, %r15
141472a4b:     	jmp	0x1414729df <.text+0x14719df>
141472a4d:     	xorl	%ecx, %ecx
141472a4f:     	testq	%r11, %r11
141472a52:     	je	0x141472911 <.text+0x1471911>
141472a58:     	testq	%rax, %rax
141472a5b:     	je	0x141472911 <.text+0x1471911>
141472a61:     	testw	%r9w, %r9w
141472a65:     	jne	0x1414729c9 <.text+0x14719c9>
141472a6b:     	movdqa	(%rsi), %xmm0
141472a6f:     	pmovmskb	%xmm0, %r9d
141472a74:     	addq	$-0x580, %r11           # imm = 0xFA80
141472a7b:     	addq	$0x10, %rsi
141472a7f:     	cmpl	$0xffff, %r9d           # imm = 0xFFFF
141472a86:     	je	0x141472a6b <.text+0x1471a6b>
141472a88:     	movl	$0xfffffffe, %r8d       # imm = 0xFFFFFFFE
141472a8e:     	subl	%r9d, %r8d
141472a91:     	notl	%r9d
141472a94:     	movl	%r9d, %r12d
141472a97:     	andl	%r8d, %r12d
141472a9a:     	jmp	0x1414729d0 <.text+0x14719d0>
141472a9f:     	addq	%rbx, %r15
141472aa2:     	movl	$0x3e7, -0x28(%r15)     # imm = 0x3E7
141472aaa:     	movb	$0x1, %bl
141472aac:     	movl	%ebx, 0x304(%rbp)
141472ab2:     	testq	%r8, %r8
141472ab5:     	jne	0x1414729fb <.text+0x14719fb>
141472abb:     	jmp	0x141472a4f <.text+0x1471a4f>
141472abd:     	movl	0x330(%rbp), %r13d
141472ac4:     	movq	0x110(%rbp), %r12
141472acb:     	movq	$-0x1, %r10
141472ad2:     	movq	0x100(%rbp), %rdx
141472ad9:     	jmp	0x141471eae <.text+0x1470eae>
141472ade:     	movl	$0x1, %ecx
141472ae3:     	movq	%rax, %rdx
141472ae6:     	callq	0x14146ccf0 <.text+0x146bcf0>
141472aeb:     	movq	%rdx, %r15
141472aee:     	testq	%rax, %rax
141472af1:     	je	0x141472c8f <.text+0x1471c8f>
141472af7:     	movq	%r15, 0xd8(%rbp)
141472afe:     	movq	0x21b98a3(%rip), %rsi   # 0x14362c3a8
141472b05:     	movq	(%rsi), %rax
141472b08:     	cmpq	$0x3, %rax
141472b0c:     	jb	0x141472bba <.text+0x1471bba>
141472b12:     	leaq	0xd8(%rbp), %rax
141472b19:     	movq	%rax, 0x250(%rbp)
141472b20:     	leaq	-0x725f07(%rip), %rax   # 0x140d4cc20 <.text+0xd4bc20>
141472b27:     	movq	%rax, 0x258(%rbp)
141472b2e:     	leaq	0x17f1cdb(%rip), %rax   # 0x142c64810
141472b35:     	movq	%rax, 0x160(%rbp)
141472b3c:     	movq	$0x2, 0x168(%rbp)
141472b47:     	movq	$0x0, 0x180(%rbp)
141472b52:     	movq	%r14, 0x170(%rbp)
141472b59:     	movq	$0x1, 0x178(%rbp)
141472b64:     	leaq	0x17f1cc5(%rip), %rcx   # 0x142c64830
141472b6b:     	callq	0x14067dd80 <.text+0x67cd80>
141472b70:     	leaq	0x17f15d1(%rip), %rcx   # 0x142c64148
141472b77:     	movq	%rcx, 0x270(%rbp)
141472b7e:     	movq	$0x26, 0x278(%rbp)
141472b89:     	movq	%rcx, 0x280(%rbp)
141472b90:     	movq	$0x26, 0x288(%rbp)
141472b9b:     	movq	%rax, 0x290(%rbp)
141472ba2:     	leaq	0x160(%rbp), %rcx
141472ba9:     	leaq	0x270(%rbp), %r8
141472bb0:     	movl	$0x3, %edx
141472bb5:     	callq	0x1404b1430 <.text+0x4b0430>
141472bba:     	movq	%r12, %rcx
141472bbd:     	movq	0x28(%rbp), %rdx
141472bc1:     	callq	0x141673140 <.text+0x1672140>
141472bc6:     	movq	%rax, %r15
141472bc9:     	movq	%rax, 0x30(%rbp)
141472bcd:     	movq	(%rsi), %rax
141472bd0:     	cmpq	$0x3, %rax
141472bd4:     	jb	0x141472c83 <.text+0x1471c83>
141472bda:     	leaq	0x30(%rbp), %rax
141472bde:     	movq	%rax, 0x250(%rbp)
141472be5:     	leaq	-0x6e552c(%rip), %rax   # 0x140d8d6c0 <.text+0xd8c6c0>
141472bec:     	movq	%rax, 0x258(%rbp)
141472bf3:     	leaq	0x17f1c76(%rip), %rax   # 0x142c64870
141472bfa:     	movq	%rax, 0x160(%rbp)
141472c01:     	movq	$0x1, 0x168(%rbp)
141472c0c:     	movq	$0x0, 0x180(%rbp)
141472c17:     	movq	%r14, 0x170(%rbp)
141472c1e:     	movq	$0x1, 0x178(%rbp)
141472c29:     	leaq	0x17f1c50(%rip), %rcx   # 0x142c64880
141472c30:     	callq	0x14067dd80 <.text+0x67cd80>
141472c35:     	leaq	0x17f150c(%rip), %rcx   # 0x142c64148
141472c3c:     	movq	%rcx, 0x270(%rbp)
141472c43:     	movq	$0x26, 0x278(%rbp)
141472c4e:     	movq	%rcx, 0x280(%rbp)
141472c55:     	movq	$0x26, 0x288(%rbp)
141472c60:     	movq	%rax, 0x290(%rbp)
141472c67:     	leaq	0x160(%rbp), %rcx
141472c6e:     	leaq	0x270(%rbp), %r8
141472c75:     	movl	$0x3, %edx
141472c7a:     	callq	0x1404b1430 <.text+0x4b0430>
141472c7f:     	movq	0x30(%rbp), %r15
141472c83:     	leaq	0xd8(%rbp), %rcx
141472c8a:     	callq	0x140d4cc30 <.text+0xd4bc30>
141472c8f:     	movsd	0xc0(%rbp), %xmm6
141472c97:     	movq	0x60(%rbp), %rdx
141472c9b:     	movq	0x68(%rbp), %rax
141472c9f:     	leaq	(%rax,%rax,4), %rax
141472ca3:     	leaq	(%rdx,%rax,8), %r8
141472ca7:     	leaq	0x17f119a(%rip), %r9    # 0x142c63e48
141472cae:     	leaq	0xd8(%rbp), %rcx
141472cb5:     	callq	0x14130dec0 <.text+0x130cec0>
141472cba:     	movq	0x78(%rbp), %rdx
141472cbe:     	movq	0x80(%rbp), %r8
141472cc5:     	shlq	$0x4, %r8
141472cc9:     	addq	%rdx, %r8
141472ccc:     	leaq	0x17f1175(%rip), %r9    # 0x142c63e48
141472cd3:     	leaq	0x250(%rbp), %rcx
141472cda:     	callq	0x1413056c0 <.text+0x13046c0>
141472cdf:     	movq	0x40(%rbp), %rsi
141472ce3:     	movq	%rsi, %rax
141472ce6:     	negq	%rax
141472ce9:     	jno	0x141472d01 <.text+0x1471d01>
141472ceb:     	movq	%rdi, %r14
141472cee:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
141472cf8:     	movq	%rax, 0x270(%rbp)
141472cff:     	jmp	0x141472d1b <.text+0x1471d1b>
141472d01:     	movq	%rdi, %r14
141472d04:     	leaq	0x17f1ea5(%rip), %r8    # 0x142c64bb0
141472d0b:     	leaq	0x270(%rbp), %rcx
141472d12:     	leaq	0x40(%rbp), %rdx
141472d16:     	callq	0x1412f8930 <.text+0x12f7930>
141472d1b:     	movq	0x18(%rbp), %rdx
141472d1f:     	movq	%rdx, %rax
141472d22:     	shrq	$0x20, %rax
141472d26:     	movq	%r15, 0x1a8(%rbp)
141472d2d:     	movl	0x11c(%rbp), %ecx
141472d33:     	movl	%ecx, 0x220(%rbp)
141472d39:     	movss	%xmm11, 0x224(%rbp)
141472d42:     	movl	%edx, 0x228(%rbp)
141472d48:     	movl	%eax, 0x22c(%rbp)
141472d4e:     	movq	0x10(%rbp), %rax
141472d52:     	movq	%rax, 0x230(%rbp)
141472d59:     	movups	%xmm10, 0x238(%rbp)
141472d61:     	movl	0x2f4(%rbp), %eax
141472d67:     	movl	%eax, 0x1f4(%rbp)
141472d6d:     	movups	%xmm8, 0x1f8(%rbp)
141472d75:     	movups	%xmm12, 0x208(%rbp)
141472d7d:     	movss	%xmm13, 0x218(%rbp)
141472d86:     	movzwl	0x316(%rbp), %eax
141472d8d:     	movw	%ax, 0x21c(%rbp)
141472d94:     	movlps	%xmm6, 0x248(%rbp)
141472d9b:     	movups	0xd8(%rbp), %xmm0
141472da2:     	movaps	%xmm0, 0x160(%rbp)
141472da9:     	movq	0xe8(%rbp), %rax
141472db0:     	movq	%rax, 0x170(%rbp)
141472db7:     	movups	0x250(%rbp), %xmm0
141472dbe:     	movups	%xmm0, 0x178(%rbp)
141472dc5:     	movq	0x260(%rbp), %rax
141472dcc:     	movq	%rax, 0x188(%rbp)
141472dd3:     	movq	0x280(%rbp), %rax
141472dda:     	movq	%rax, 0x1a0(%rbp)
141472de1:     	movups	0x270(%rbp), %xmm0
141472de8:     	movaps	%xmm0, 0x190(%rbp)
141472def:     	movaps	%xmm7, 0x1b0(%rbp)
141472df6:     	movl	0x124(%rbp), %eax
141472dfc:     	movl	%eax, 0x1c0(%rbp)
141472e02:     	movl	0x128(%rbp), %eax
141472e08:     	movb	%al, 0x1c4(%rbp)
141472e0e:     	movl	0x120(%rbp), %eax
141472e14:     	movl	%eax, 0x1c8(%rbp)
141472e1a:     	movdqu	%xmm14, 0x1cc(%rbp)
141472e23:     	movups	%xmm9, 0x1dc(%rbp)
141472e2b:     	movss	%xmm15, 0x1ec(%rbp)
141472e34:     	movl	0x12c(%rbp), %eax
141472e3a:     	movb	%al, 0x1f0(%rbp)
141472e40:     	movl	0x130(%rbp), %eax
141472e46:     	movb	%al, 0x1f1(%rbp)
141472e4c:     	movzbl	0x32f(%rbp), %eax
141472e53:     	movb	%al, 0x1f2(%rbp)
141472e59:     	callq	0x14146cec0 <.text+0x146bec0>
141472e5e:     	movq	%rax, %rdi
141472e61:     	leaq	0x160(%rbp), %rdx
141472e68:     	movl	$0xf0, %r8d
141472e6e:     	movq	%rax, %rcx
141472e71:     	callq	0x142923c00 <.text+0x2922c00>
141472e76:     	cmpl	$0x1b, %r13d
141472e7a:     	setne	%al
141472e7d:     	orb	0x336(%rbp), %al
141472e83:     	movl	0x2f0(%rbp), %r8d
141472e8a:     	andb	$0x1, %r8b
141472e8e:     	movl	0x304(%rbp), %edx
141472e94:     	andb	$0x1, %dl
141472e97:     	movq	%rdi, (%r14)
141472e9a:     	movq	%r14, %rdi
141472e9d:     	movl	0xd4(%rbp), %ecx
141472ea3:     	movl	%ecx, 0x8(%r14)
141472ea7:     	movb	%al, 0xc(%r14)
141472eab:     	movb	%r8b, 0xd(%r14)
141472eaf:     	movb	%dl, 0xe(%r14)
141472eb3:     	movb	%bl, 0xf(%r14)
141472eb7:     	leaq	0x58(%rbp), %rcx
141472ebb:     	callq	0x14088acf0 <.text+0x889cf0>
141472ec0:     	movq	%rsi, %rax
141472ec3:     	negq	%rax
141472ec6:     	jo	0x141472ee1 <.text+0x1471ee1>
141472ec8:     	jae	0x141472ee1 <.text+0x1471ee1>
141472eca:     	movq	0x48(%rbp), %rcx
141472ece:     	shlq	$0x3, %rsi
141472ed2:     	leaq	(%rsi,%rsi,4), %rdx
141472ed6:     	movl	$0x4, %r8d
141472edc:     	callq	0x140613c20 <.text+0x612c20>
141472ee1:     	leaq	0x138(%rbp), %rcx
141472ee8:     	callq	0x14085da90 <.text+0x85ca90>
141472eed:     	jmp	0x14147147c <.text+0x147047c>
141472ef2:     	movb	$0x1, 0x337(%rbp)
141472ef9:     	leaq	0x16324d0(%rip), %rcx   # 0x142aa53d0
141472f00:     	leaq	0x1664bb1(%rip), %r8    # 0x142ad7ab8
141472f07:     	movl	$0x1d, %edx
141472f0c:     	callq	0x1428d9430 <.text+0x28d8430>
141472f11:     	jmp	0x141473383 <.text+0x1472383>
141472f16:     	movb	$0x1, 0x337(%rbp)
141472f1d:     	leaq	0x1632574(%rip), %rcx   # 0x142aa5498
141472f24:     	leaq	0x1664b8d(%rip), %r8    # 0x142ad7ab8
141472f2b:     	movl	$0x22, %edx
141472f30:     	callq	0x1428d9430 <.text+0x28d8430>
141472f35:     	jmp	0x141473383 <.text+0x1472383>
141472f3a:     	movb	$0x1, 0x337(%rbp)
141472f41:     	leaq	0x1632550(%rip), %rcx   # 0x142aa5498
141472f48:     	leaq	0x1664b81(%rip), %r8    # 0x142ad7ad0
141472f4f:     	movl	$0x22, %edx
141472f54:     	callq	0x1428d9430 <.text+0x28d8430>
141472f59:     	jmp	0x141473383 <.text+0x1472383>
141472f5e:     	cmpl	$0x13, %r13d
141472f62:     	setl	%bl
141472f65:     	jge	0x14147319b <.text+0x147219b>
141472f6b:     	movq	0x21b9436(%rip), %rax   # 0x14362c3a8
141472f72:     	movq	(%rax), %rax
141472f75:     	cmpq	$0x3, %rax
141472f79:     	jb	0x14147303c <.text+0x147203c>
141472f7f:     	leaq	0x17f170a(%rip), %rax   # 0x142c64690
141472f86:     	movq	%rax, 0x250(%rbp)
141472f8d:     	leaq	-0x6e5a14(%rip), %rax   # 0x140d8d580 <.text+0xd8c580>
141472f94:     	movq	%rax, 0x258(%rbp)
141472f9b:     	leaq	0x17f172e(%rip), %rax   # 0x142c646d0
141472fa2:     	movq	%rax, 0x160(%rbp)
141472fa9:     	movq	$0x2, 0x168(%rbp)
141472fb4:     	movq	$0x0, 0x180(%rbp)
141472fbf:     	leaq	0x250(%rbp), %rax
141472fc6:     	movq	%rax, 0x170(%rbp)
141472fcd:     	movq	$0x1, 0x178(%rbp)
141472fd8:     	movb	$0x1, 0x337(%rbp)
141472fdf:     	leaq	0x17f170a(%rip), %rcx   # 0x142c646f0
141472fe6:     	callq	0x14067dd80 <.text+0x67cd80>
141472feb:     	leaq	0x17f1156(%rip), %rcx   # 0x142c64148
141472ff2:     	movq	%rcx, 0x270(%rbp)
141472ff9:     	movq	$0x26, 0x278(%rbp)
141473004:     	movq	%rcx, 0x280(%rbp)
14147300b:     	movq	$0x26, 0x288(%rbp)
141473016:     	movq	%rax, 0x290(%rbp)
14147301d:     	movb	$0x1, 0x337(%rbp)
141473024:     	leaq	0x160(%rbp), %rcx
14147302b:     	leaq	0x270(%rbp), %r8
141473032:     	movl	$0x3, %edx
141473037:     	callq	0x1404b1430 <.text+0x4b0430>
14147303c:     	cmpl	$0x11, %r13d
141473040:     	jge	0x14147319b <.text+0x147219b>
141473046:     	movq	$0x0, 0xb0(%rbp)
141473051:     	movq	0x68(%rbp), %rax
141473055:     	testq	%rax, %rax
141473058:     	je	0x14147319b <.text+0x147219b>
14147305e:     	movq	0x60(%rbp), %rcx
141473062:     	leaq	(%rax,%rax,4), %rax
141473066:     	leaq	(%rcx,%rax,8), %rdx
14147306a:     	xorl	%r8d, %r8d
14147306d:     	movq	$-0x1, %r9
141473074:     	movss	0x14c152c(%rip), %xmm0  # 0x1429345a8
14147307c:     	movss	0x160c9ac(%rip), %xmm1  # 0x142a7fa30
141473084:     	movabsq	$-0x7fffffffffffffff, %r10 # imm = 0x8000000000000001
14147308e:     	movaps	0x17b604b(%rip), %xmm2  # 0x142c290e0
141473095:     	movaps	0x1647924(%rip), %xmm3  # 0x142aba9c0
14147309c:     	jmp	0x1414730ab <.text+0x14720ab>
14147309e:     	addq	$0x28, %rcx
1414730a2:     	cmpq	%rdx, %rcx
1414730a5:     	je	0x14147319b <.text+0x147219b>
1414730ab:     	movq	0x10(%rcx), %rax
1414730af:     	testq	%rax, %rax
1414730b2:     	je	0x14147309e <.text+0x147209e>
1414730b4:     	movq	0x8(%rcx), %r11
1414730b8:     	shlq	$0x3, %rax
1414730bc:     	leaq	(%rax,%rax,2), %rsi
1414730c0:     	xorl	%edi, %edi
1414730c2:     	jmp	0x1414730d4 <.text+0x14720d4>
1414730c4:     	cmpq	$0x0, (%r11,%rdi)
1414730c9:     	je	0x14147313a <.text+0x147213a>
1414730cb:     	addq	$0x18, %rdi
1414730cf:     	cmpq	%rdi, %rsi
1414730d2:     	je	0x14147309e <.text+0x147209e>
1414730d4:     	cmpq	$0x0, (%r11,%rdi)
1414730d9:     	jne	0x1414730cb <.text+0x14720cb>
1414730db:     	movq	0x8(%r11,%rdi), %r14
1414730e0:     	movl	$0x1, %eax
1414730e5:     	lock
1414730e6:     	cmpxchgq	%r9, 0x8(%r14)
1414730eb:     	jne	0x14147330e <.text+0x147230e>
1414730f1:     	movq	(%r14), %rax
1414730f4:     	movq	$0x1, 0x8(%r14)
1414730fc:     	cmpq	$0x1, %rax
141473100:     	jne	0x14147330e <.text+0x147230e>
141473106:     	movq	0x8(%r11,%rdi), %rax
14147310b:     	cmpq	0x10(%rax), %r8
14147310f:     	jno	0x1414730c4 <.text+0x14720c4>
141473111:     	movss	0x34(%rax), %xmm4
141473116:     	sqrtss	%xmm4, %xmm4
14147311a:     	mulss	%xmm0, %xmm4
14147311e:     	xorps	%xmm5, %xmm5
141473121:     	maxss	%xmm4, %xmm5
141473125:     	movaps	%xmm1, %xmm4
141473128:     	minss	%xmm5, %xmm4
14147312c:     	movss	%xmm4, 0x34(%rax)
141473131:     	movb	$0x1, 0x336(%rbp)
141473138:     	jmp	0x1414730c4 <.text+0x14720c4>
14147313a:     	movq	0x8(%r11,%rdi), %r14
14147313f:     	movl	$0x1, %eax
141473144:     	lock
141473145:     	cmpxchgq	%r9, 0x8(%r14)
14147314a:     	jne	0x141473323 <.text+0x1472323>
141473150:     	movq	(%r14), %rax
141473153:     	movq	$0x1, 0x8(%r14)
14147315b:     	cmpq	$0x1, %rax
14147315f:     	jne	0x141473323 <.text+0x1472323>
141473165:     	movq	0x8(%r11,%rdi), %rax
14147316a:     	cmpq	%r10, 0x10(%rax)
14147316e:     	jne	0x1414730cb <.text+0x14720cb>
141473174:     	movsd	0x1c(%rax), %xmm4
141473179:     	sqrtps	%xmm4, %xmm4
14147317c:     	mulps	%xmm2, %xmm4
14147317f:     	xorps	%xmm5, %xmm5
141473182:     	maxps	%xmm4, %xmm5
141473185:     	movaps	%xmm3, %xmm4
141473188:     	minps	%xmm5, %xmm4
14147318b:     	movlps	%xmm4, 0x1c(%rax)
14147318f:     	movb	$0x1, 0x336(%rbp)
141473196:     	jmp	0x1414730cb <.text+0x14720cb>
14147319b:     	cmpb	$0x1, 0x336(%rbp)
1414731a2:     	jne	0x1414731ad <.text+0x14721ad>
1414731a4:     	movzbl	0xd8(%rbp), %edx
1414731ab:     	jmp	0x1414731b6 <.text+0x14721b6>
1414731ad:     	movb	$0x0, 0xd8(%rbp)
1414731b4:     	xorl	%edx, %edx
1414731b6:     	movb	$0x0, 0x337(%rbp)
1414731bd:     	andb	$0x1, %dl
1414731c0:     	leaq	0x58(%rbp), %rcx
1414731c4:     	callq	0x140a86980 <.text+0xa85980>
1414731c9:     	testb	$0x1, 0x336(%rbp)
1414731d0:     	je	0x141473290 <.text+0x1472290>
1414731d6:     	movq	0x21b91cb(%rip), %rax   # 0x14362c3a8
1414731dd:     	movq	(%rax), %rax
1414731e0:     	cmpq	$0x3, %rax
1414731e4:     	movq	0x38(%rbp), %rdi
1414731e8:     	movaps	-0x20(%rbp), %xmm7
1414731ec:     	movaps	-0x10(%rbp), %xmm9
1414731f1:     	movss	0x134(%rbp), %xmm15
1414731fa:     	leaq	0x250(%rbp), %r14
141473201:     	jb	0x141471cc1 <.text+0x1470cc1>
141473207:     	leaq	0x17f1592(%rip), %rax   # 0x142c647a0
14147320e:     	movq	%rax, 0x160(%rbp)
141473215:     	movq	$0x1, 0x168(%rbp)
141473220:     	movq	$0x8, 0x170(%rbp)
14147322b:     	xorps	%xmm0, %xmm0
14147322e:     	movups	%xmm0, 0x178(%rbp)
141473235:     	leaq	0x17f1574(%rip), %rcx   # 0x142c647b0
14147323c:     	callq	0x14067dd80 <.text+0x67cd80>
141473241:     	leaq	0x17f0f00(%rip), %rcx   # 0x142c64148
141473248:     	movq	%rcx, 0x270(%rbp)
14147324f:     	movq	$0x26, 0x278(%rbp)
14147325a:     	movq	%rcx, 0x280(%rbp)
141473261:     	movq	$0x26, 0x288(%rbp)
14147326c:     	movq	%rax, 0x290(%rbp)
141473273:     	leaq	0x160(%rbp), %rcx
14147327a:     	leaq	0x270(%rbp), %r8
141473281:     	movl	$0x3, %edx
141473286:     	callq	0x1404b1430 <.text+0x4b0430>
14147328b:     	jmp	0x141471cc1 <.text+0x1470cc1>
141473290:     	movq	0x38(%rbp), %rdi
141473294:     	movaps	-0x20(%rbp), %xmm7
141473298:     	movaps	-0x10(%rbp), %xmm9
14147329d:     	movss	0x134(%rbp), %xmm15
1414732a6:     	movl	0x304(%rbp), %eax
1414732ac:     	leaq	0x250(%rbp), %r14
