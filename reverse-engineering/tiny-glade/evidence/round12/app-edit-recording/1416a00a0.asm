
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1416a00a0:     	pushq	%rbp
1416a00a1:     	pushq	%r15
1416a00a3:     	pushq	%r14
1416a00a5:     	pushq	%rsi
1416a00a6:     	pushq	%rdi
1416a00a7:     	pushq	%rbx
1416a00a8:     	subq	$0x38, %rsp
1416a00ac:     	leaq	0x30(%rsp), %rbp
1416a00b1:     	movq	$-0x2, (%rbp)
1416a00b9:     	movq	%rcx, %rsi
1416a00bc:     	callq	0x140d2fa20 <.text+0xd2ea20>
1416a00c1:     	movq	(%rsi), %rax
1416a00c4:     	testq	%rax, %rax
1416a00c7:     	je	0x1416a0284 <.text+0x169f284>
1416a00cd:     	cmpq	$0x0, 0x10(%rsi)
1416a00d2:     	je	0x1416a0268 <.text+0x169f268>
1416a00d8:     	movq	0x8(%rsi), %rdx
1416a00dc:     	movabsq	$-0x120d12fd687d5797, %r9 # imm = 0xEDF2ED029782A869
1416a00e6:     	xorl	%r10d, %r10d
1416a00e9:     	movdqa	0x12bf0ff(%rip), %xmm0  # 0x14295f1f0
1416a00f1:     	pcmpeqd	%xmm1, %xmm1
1416a00f5:     	movabsq	$0x7647bf59ffe455e2, %rcx # imm = 0x7647BF59FFE455E2
1416a00ff:     	movq	%r9, %r11
1416a0102:     	xorl	%edi, %edi
1416a0104:     	andq	%rdx, %r11
1416a0107:     	movdqu	(%rax,%r11), %xmm2
1416a010d:     	movdqa	%xmm2, %xmm3
1416a0111:     	pcmpeqb	%xmm0, %xmm3
1416a0115:     	pmovmskb	%xmm3, %ebx
1416a0119:     	testl	%ebx, %ebx
1416a011b:     	jne	0x1416a016d <.text+0x169f16d>
1416a011d:     	cmpq	$0x1, %rdi
1416a0121:     	movl	$0x1, %edi
1416a0126:     	je	0x1416a0141 <.text+0x169f141>
1416a0128:     	pmovmskb	%xmm2, %r8d
1416a012d:     	xorl	%edi, %edi
1416a012f:     	testl	%r8d, %r8d
1416a0132:     	setne	%dil
1416a0136:     	tzcntl	%r8d, %r8d
1416a013b:     	addq	%r11, %r8
1416a013e:     	andq	%rdx, %r8
1416a0141:     	pcmpeqb	%xmm1, %xmm2
1416a0145:     	pmovmskb	%xmm2, %ebx
1416a0149:     	testl	%ebx, %ebx
1416a014b:     	jne	0x1416a01e9 <.text+0x169f1e9>
1416a0151:     	addq	%r10, %r11
1416a0154:     	addq	$0x10, %r11
1416a0158:     	addq	$0x10, %r10
1416a015c:     	jmp	0x1416a0104 <.text+0x169f104>
1416a015e:     	nop
1416a0160:     	leal	-0x1(%rbx), %r14d
1416a0164:     	andw	%bx, %r14w
1416a0168:     	movl	%r14d, %ebx
1416a016b:     	je	0x1416a011d <.text+0x169f11d>
1416a016d:     	tzcntl	%ebx, %r15d
1416a0172:     	addq	%r11, %r15
1416a0175:     	andq	%rdx, %r15
1416a0178:     	shlq	$0x5, %r15
1416a017c:     	movq	%rax, %r14
1416a017f:     	subq	%r15, %r14
1416a0182:     	cmpq	%rcx, -0x20(%r14)
1416a0186:     	jne	0x1416a0160 <.text+0x169f160>
1416a0188:     	cmpq	%r9, -0x18(%r14)
1416a018c:     	jne	0x1416a0160 <.text+0x169f160>
1416a018e:     	movq	-0x10(%r14), %rax
1416a0192:     	movq	%rax, -0x10(%rbp)
1416a0196:     	movq	-0x8(%r14), %rcx
1416a019a:     	movq	$0x1, -0x10(%r14)
1416a01a2:     	leaq	0x15e824f(%rip), %rax   # 0x142c883f8
1416a01a9:     	movq	%rax, -0x8(%r14)
1416a01ad:     	movq	%rcx, -0x8(%rbp)
1416a01b1:     	movq	(%rcx), %rax
1416a01b4:     	testq	%rax, %rax
1416a01b7:     	je	0x1416a01bf <.text+0x169f1bf>
1416a01b9:     	movq	-0x10(%rbp), %rcx
1416a01bd:     	callq	*%rax
1416a01bf:     	movq	-0x10(%rbp), %rcx
1416a01c3:     	movq	-0x8(%rbp), %rax
1416a01c7:     	movq	0x8(%rax), %rdx
1416a01cb:     	testq	%rdx, %rdx
1416a01ce:     	je	0x1416a025b <.text+0x169f25b>
1416a01d4:     	movq	0x10(%rax), %r8
1416a01d8:     	addq	$0x38, %rsp
1416a01dc:     	popq	%rbx
1416a01dd:     	popq	%rdi
1416a01de:     	popq	%rsi
1416a01df:     	popq	%r14
1416a01e1:     	popq	%r15
1416a01e3:     	popq	%rbp
1416a01e4:     	jmp	0x140613c20 <.text+0x612c20>
1416a01e9:     	movzbl	(%rax,%r8), %r9d
1416a01ee:     	testb	%r9b, %r9b
1416a01f1:     	js	0x1416a0206 <.text+0x169f206>
1416a01f3:     	movdqa	(%rax), %xmm0
1416a01f7:     	pmovmskb	%xmm0, %r8d
1416a01fc:     	tzcntl	%r8d, %r8d
1416a0201:     	movzbl	(%rax,%r8), %r9d
1416a0206:     	andb	$0x1, %r9b
1416a020a:     	movzbl	%r9b, %r9d
1416a020e:     	subq	%r9, 0x10(%rsi)
1416a0212:     	leaq	-0x10(%r8), %r9
1416a0216:     	andq	%rdx, %r9
1416a0219:     	movb	$0x76, (%rax,%r8)
1416a021e:     	movb	$0x76, 0x10(%rax,%r9)
1416a0224:     	incq	0x18(%rsi)
1416a0228:     	shlq	$0x5, %r8
1416a022c:     	movq	%rax, %rdx
1416a022f:     	subq	%r8, %rdx
1416a0232:     	negq	%r8
1416a0235:     	movq	%rcx, -0x20(%rax,%r8)
1416a023a:     	movabsq	$-0x120d12fd687d5797, %rax # imm = 0xEDF2ED029782A869
1416a0244:     	movq	%rax, -0x18(%rdx)
1416a0248:     	movq	$0x1, -0x10(%rdx)
1416a0250:     	leaq	0x15e81a1(%rip), %rax   # 0x142c883f8
1416a0257:     	movq	%rax, -0x8(%rdx)
1416a025b:     	addq	$0x38, %rsp
1416a025f:     	popq	%rbx
1416a0260:     	popq	%rdi
1416a0261:     	popq	%rsi
1416a0262:     	popq	%r14
1416a0264:     	popq	%r15
1416a0266:     	popq	%rbp
1416a0267:     	retq
1416a0268:     	leaq	0x20(%rsi), %r8
1416a026c:     	movl	$0x1, %edx
1416a0271:     	movq	%rsi, %rcx
1416a0274:     	movb	$0x1, %r9b
1416a0277:     	callq	0x1428bae30 <.text+0x28b9e30>
1416a027c:     	movq	(%rsi), %rax
1416a027f:     	jmp	0x1416a00d8 <.text+0x169f0d8>
1416a0284:     	leaq	0x15e818d(%rip), %rcx   # 0x142c88418
1416a028b:     	leaq	0x15e81ae(%rip), %r8    # 0x142c88440
1416a0292:     	movl	$0x28, %edx
1416a0297:     	callq	0x1428d9430 <.text+0x28d8430>
1416a029c:     	int3
