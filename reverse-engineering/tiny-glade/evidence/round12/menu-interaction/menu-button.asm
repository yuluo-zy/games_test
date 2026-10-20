
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14068bfc0:     	pushq	%r15
14068bfc2:     	pushq	%r14
14068bfc4:     	pushq	%rsi
14068bfc5:     	pushq	%rdi
14068bfc6:     	pushq	%rbx
14068bfc7:     	subq	$0x70, %rsp
14068bfcb:     	movq	%r9, %rdi
14068bfce:     	movq	%r8, %rbx
14068bfd1:     	movq	%rdx, %r14
14068bfd4:     	movq	%rcx, %rsi
14068bfd7:     	movq	%r8, 0x60(%rsp)
14068bfdc:     	movq	%r9, 0x68(%rsp)
14068bfe1:     	leaq	0x60(%rsp), %rcx
14068bfe6:     	callq	0x1405bdda0 <.text+0x5bcda0>
14068bfeb:     	movq	%rax, 0x20(%rsp)
14068bff0:     	movabsq	$0x64db2c3b564bd8b3, %rax # imm = 0x64DB2C3B564BD8B3
14068bffa:     	movq	%rax, 0x28(%rsp)
14068bfff:     	movabsq	$0x615bfcfe4597ab8f, %rax # imm = 0x615BFCFE4597AB8F
14068c009:     	movq	%rax, 0x30(%rsp)
14068c00e:     	leaq	0x20(%rsp), %rcx
14068c013:     	callq	0x1405bdf20 <.text+0x5bcf20>
14068c018:     	movq	0x8(%r14), %rcx
14068c01c:     	movq	%rcx, 0x20(%rsp)
14068c021:     	movq	%rax, 0x28(%rsp)
14068c026:     	leaq	0x20(%rsp), %rcx
14068c02b:     	callq	0x1405bd910 <.text+0x5bc910>
14068c030:     	leaq	0x20(%rsp), %r15
14068c035:     	movq	%r15, %rcx
14068c038:     	movq	%r14, %rdx
14068c03b:     	movq	%rax, %r8
14068c03e:     	callq	0x1406861b0 <.text+0x6851b0>
14068c043:     	movq	%r15, %rcx
14068c046:     	xorl	%edx, %edx
14068c048:     	callq	0x140f9dba0 <.text+0xf9cba0>
14068c04d:     	movq	%rax, %rcx
14068c050:     	movq	%rbx, %rdx
14068c053:     	movq	%rdi, %r8
14068c056:     	callq	0x140fac4c0 <.text+0xfab4c0>
14068c05b:     	movq	0x30(%rsp), %rax
14068c060:     	movq	%rax, 0x50(%rsp)
14068c065:     	movdqu	0x20(%rsp), %xmm0
14068c06b:     	movdqa	%xmm0, 0x40(%rsp)
14068c071:     	movq	0x40(%rsp), %rdi
14068c076:     	cmpq	$0x0, 0x1b8(%rdi)
14068c07e:     	je	0x14068c245 <.text+0x68b245>
14068c084:     	leaq	0x48(%rsp), %rdx
14068c089:     	leaq	0x1c0(%rdi), %rcx
14068c090:     	callq	0x1405b0d70 <.text+0x5afd70>
14068c095:     	movq	0x1a0(%rdi), %rcx
14068c09c:     	movq	0x1a8(%rdi), %rdx
14068c0a3:     	movq	%rdx, %r8
14068c0a6:     	andq	%rax, %r8
14068c0a9:     	shrq	$0x39, %rax
14068c0ad:     	movd	%eax, %xmm0
14068c0b1:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
14068c0b5:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14068c0ba:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14068c0bf:     	leaq	-0xc0(%rcx), %rax
14068c0c6:     	xorl	%r9d, %r9d
14068c0c9:     	movq	0x48(%rsp), %r10
14068c0ce:     	pcmpeqd	%xmm1, %xmm1
14068c0d2:     	movdqu	(%rcx,%r8), %xmm2
14068c0d8:     	movdqa	%xmm2, %xmm3
14068c0dc:     	pcmpeqb	%xmm0, %xmm3
14068c0e0:     	pmovmskb	%xmm3, %r11d
14068c0e5:     	testl	%r11d, %r11d
14068c0e8:     	je	0x14068c120 <.text+0x68b120>
14068c0ea:     	tzcntl	%r11d, %edi
14068c0ef:     	addq	%r8, %rdi
14068c0f2:     	andq	%rdx, %rdi
14068c0f5:     	negq	%rdi
14068c0f8:     	leaq	(%rdi,%rdi,2), %rdi
14068c0fc:     	shlq	$0x6, %rdi
14068c100:     	cmpq	(%rax,%rdi), %r10
14068c104:     	je	0x14068c142 <.text+0x68b142>
14068c106:     	leal	-0x1(%r11), %edi
14068c10a:     	andw	%r11w, %di
14068c10e:     	movl	%edi, %r11d
14068c111:     	jne	0x14068c0ea <.text+0x68b0ea>
14068c113:     	nopw	%cs:(%rax,%rax)
14068c120:     	pcmpeqb	%xmm1, %xmm2
14068c124:     	pmovmskb	%xmm2, %r11d
14068c129:     	testl	%r11d, %r11d
14068c12c:     	jne	0x14068c245 <.text+0x68b245>
14068c132:     	addq	%r9, %r8
14068c135:     	addq	$0x10, %r8
14068c139:     	addq	$0x10, %r9
14068c13d:     	andq	%rdx, %r8
14068c140:     	jmp	0x14068c0d2 <.text+0x68b0d2>
14068c142:     	orb	$0x20, -0x1(%rcx,%rdi)
14068c147:     	movq	0x50(%rsp), %rax
14068c14c:     	movq	%rax, 0x30(%rsp)
14068c151:     	movdqa	0x40(%rsp), %xmm0
14068c157:     	movdqa	%xmm0, 0x20(%rsp)
14068c15d:     	movq	0x20(%rsp), %rdi
14068c162:     	cmpq	$0x0, 0x1b8(%rdi)
14068c16a:     	je	0x14068c251 <.text+0x68b251>
14068c170:     	leaq	0x28(%rsp), %rdx
14068c175:     	leaq	0x1c0(%rdi), %rcx
14068c17c:     	callq	0x1405b0d70 <.text+0x5afd70>
14068c181:     	movq	0x1a0(%rdi), %rcx
14068c188:     	movq	0x1a8(%rdi), %rdx
14068c18f:     	movq	%rdx, %r8
14068c192:     	andq	%rax, %r8
14068c195:     	shrq	$0x39, %rax
14068c199:     	movd	%eax, %xmm0
14068c19d:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
14068c1a1:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14068c1a6:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14068c1ab:     	leaq	-0xc0(%rcx), %rax
14068c1b2:     	xorl	%r9d, %r9d
14068c1b5:     	movq	0x28(%rsp), %r10
14068c1ba:     	pcmpeqd	%xmm1, %xmm1
14068c1be:     	movdqu	(%rcx,%r8), %xmm2
14068c1c4:     	movdqa	%xmm2, %xmm3
14068c1c8:     	pcmpeqb	%xmm0, %xmm3
14068c1cc:     	pmovmskb	%xmm3, %r11d
14068c1d1:     	testl	%r11d, %r11d
14068c1d4:     	je	0x14068c200 <.text+0x68b200>
14068c1d6:     	tzcntl	%r11d, %edi
14068c1db:     	addq	%r8, %rdi
14068c1de:     	andq	%rdx, %rdi
14068c1e1:     	negq	%rdi
14068c1e4:     	leaq	(%rdi,%rdi,2), %rdi
14068c1e8:     	shlq	$0x6, %rdi
14068c1ec:     	cmpq	(%rax,%rdi), %r10
14068c1f0:     	je	0x14068c21e <.text+0x68b21e>
14068c1f2:     	leal	-0x1(%r11), %edi
14068c1f6:     	andw	%r11w, %di
14068c1fa:     	movl	%edi, %r11d
14068c1fd:     	jne	0x14068c1d6 <.text+0x68b1d6>
14068c1ff:     	nop
14068c200:     	pcmpeqb	%xmm1, %xmm2
14068c204:     	pmovmskb	%xmm2, %r11d
14068c209:     	testl	%r11d, %r11d
14068c20c:     	jne	0x14068c251 <.text+0x68b251>
14068c20e:     	addq	%r9, %r8
14068c211:     	addq	$0x10, %r8
14068c215:     	addq	$0x10, %r9
14068c219:     	andq	%rdx, %r8
14068c21c:     	jmp	0x14068c1be <.text+0x68b1be>
14068c21e:     	movw	$0x200, -0x4(%rcx,%rdi) # imm = 0x200
14068c225:     	movq	0x30(%rsp), %rax
14068c22a:     	movq	%rax, 0x10(%rsi)
14068c22e:     	movaps	0x20(%rsp), %xmm0
14068c233:     	movups	%xmm0, (%rsi)
14068c236:     	movq	%rsi, %rax
14068c239:     	addq	$0x70, %rsp
14068c23d:     	popq	%rbx
14068c23e:     	popq	%rdi
14068c23f:     	popq	%rsi
14068c240:     	popq	%r14
14068c242:     	popq	%r15
14068c244:     	retq
14068c245:     	leaq	0x238bd74(%rip), %rcx   # 0x142a17fc0
14068c24c:     	callq	0x1428d9310 <.text+0x28d8310>
14068c251:     	leaq	0x238bd98(%rip), %rcx   # 0x142a17ff0
14068c258:     	callq	0x1428d9310 <.text+0x28d8310>
14068c25d:     	int3
