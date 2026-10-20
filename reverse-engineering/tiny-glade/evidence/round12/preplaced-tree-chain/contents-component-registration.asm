
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141552040:     	pushq	%rbp
141552041:     	pushq	%r15
141552043:     	pushq	%r14
141552045:     	pushq	%rsi
141552046:     	pushq	%rdi
141552047:     	pushq	%rbx
141552048:     	subq	$0x2d8, %rsp            # imm = 0x2D8
14155204f:     	leaq	0x80(%rsp), %rbp
141552057:     	movq	$-0x2, 0x250(%rbp)
141552062:     	movq	%rcx, %rsi
141552065:     	movq	$0x0, 0x228(%rbp)
141552070:     	movq	$0x8, 0x230(%rbp)
14155207b:     	movq	$0x0, 0x238(%rbp)
141552086:     	movq	(%rcx), %rdi
141552089:     	cmpq	$0x0, 0x30(%rdi)
14155208e:     	movabsq	$-0x47cf3fc999470331, %rax # imm = 0xB830C03666B8FCCF
141552098:     	movabsq	$-0x4d6baee718106f7b, %rcx # imm = 0xB2945118E7EF9085
1415520a2:     	je	0x14155213a <.text+0x155113a>
1415520a8:     	movq	0x18(%rdi), %rdx
1415520ac:     	movq	0x20(%rdi), %r8
1415520b0:     	movq	%r8, %r9
1415520b3:     	andq	%rax, %r9
1415520b6:     	xorl	%r10d, %r10d
1415520b9:     	movdqa	0x140f4ef(%rip), %xmm0  # 0x1429615b0
1415520c1:     	pcmpeqd	%xmm1, %xmm1
1415520c5:     	movdqu	(%rdx,%r9), %xmm2
1415520cb:     	movdqa	%xmm2, %xmm3
1415520cf:     	pcmpeqb	%xmm0, %xmm3
1415520d3:     	pmovmskb	%xmm3, %r11d
1415520d8:     	testl	%r11d, %r11d
1415520db:     	jne	0x14155210d <.text+0x155110d>
1415520dd:     	pcmpeqb	%xmm1, %xmm2
1415520e1:     	pmovmskb	%xmm2, %r11d
1415520e6:     	testl	%r11d, %r11d
1415520e9:     	jne	0x14155213a <.text+0x155113a>
1415520eb:     	addq	%r10, %r9
1415520ee:     	addq	$0x10, %r9
1415520f2:     	addq	$0x10, %r10
1415520f6:     	andq	%r8, %r9
1415520f9:     	jmp	0x1415520c5 <.text+0x15510c5>
1415520fb:     	nopl	(%rax,%rax)
141552100:     	leal	-0x1(%r11), %ebx
141552104:     	andw	%r11w, %bx
141552108:     	movl	%ebx, %r11d
14155210b:     	je	0x1415520dd <.text+0x15510dd>
14155210d:     	tzcntl	%r11d, %ebx
141552112:     	addq	%r9, %rbx
141552115:     	andq	%r8, %rbx
141552118:     	negq	%rbx
14155211b:     	leaq	(%rbx,%rbx,2), %rbx
14155211f:     	cmpq	%rcx, -0x18(%rdx,%rbx,8)
141552124:     	jne	0x141552100 <.text+0x1551100>
141552126:     	cmpq	%rax, -0x10(%rdx,%rbx,8)
14155212b:     	jne	0x141552100 <.text+0x1551100>
14155212d:     	leaq	(%rdx,%rbx,8), %rax
141552131:     	movq	-0x8(%rax), %rbx
141552135:     	jmp	0x1415524f3 <.text+0x15514f3>
14155213a:     	movzbl	0x60(%rdi), %edx
14155213e:     	movq	0x80(%rdi), %rdx
141552145:     	movq	0x88(%rdi), %r9
14155214c:     	movq	%r9, %r10
14155214f:     	andq	%rax, %r10
141552152:     	xorl	%r11d, %r11d
141552155:     	movdqa	0x140f453(%rip), %xmm0  # 0x1429615b0
14155215d:     	pcmpeqd	%xmm1, %xmm1
141552161:     	movdqu	(%rdx,%r10), %xmm2
141552167:     	movdqa	%xmm2, %xmm3
14155216b:     	pcmpeqb	%xmm0, %xmm3
14155216f:     	pmovmskb	%xmm3, %ebx
141552173:     	testl	%ebx, %ebx
141552175:     	jne	0x1415521ad <.text+0x15511ad>
141552177:     	pcmpeqb	%xmm1, %xmm2
14155217b:     	pmovmskb	%xmm2, %r8d
141552180:     	testl	%r8d, %r8d
141552183:     	jne	0x141552277 <.text+0x1551277>
141552189:     	addq	%r11, %r10
14155218c:     	addq	$0x10, %r10
141552190:     	addq	$0x10, %r11
141552194:     	andq	%r9, %r10
141552197:     	jmp	0x141552161 <.text+0x1551161>
141552199:     	nopl	(%rax)
1415521a0:     	leal	-0x1(%rbx), %r8d
1415521a4:     	andw	%bx, %r8w
1415521a8:     	movl	%r8d, %ebx
1415521ab:     	je	0x141552177 <.text+0x1551177>
1415521ad:     	tzcntl	%ebx, %r14d
1415521b2:     	addq	%r10, %r14
1415521b5:     	andq	%r9, %r14
1415521b8:     	movq	%r14, %r8
1415521bb:     	negq	%r8
1415521be:     	leaq	(,%r14,8), %r15
1415521c6:     	shlq	$0x7, %r8
1415521ca:     	subq	%r15, %r8
1415521cd:     	cmpq	%rcx, -0x88(%rdx,%r8)
1415521d5:     	jne	0x1415521a0 <.text+0x15511a0>
1415521d7:     	cmpq	%rax, -0x80(%rdx,%r8)
1415521dc:     	jne	0x1415521a0 <.text+0x15511a0>
1415521de:     	movq	%r14, %r10
1415521e1:     	shlq	$0x7, %r10
1415521e5:     	leaq	(%r10,%r14,8), %r11
1415521e9:     	sarq	$0x3, %r11
1415521ed:     	movabsq	$-0xf0f0f0f0f0f0f0f, %r10 # imm = 0xF0F0F0F0F0F0F0F1
1415521f7:     	imulq	%r11, %r10
1415521fb:     	leaq	-0x10(%r10), %r11
1415521ff:     	andq	%r9, %r11
141552202:     	movdqu	(%rdx,%r11), %xmm0
141552208:     	pcmpeqd	%xmm1, %xmm1
14155220c:     	pcmpeqb	%xmm1, %xmm0
141552210:     	pmovmskb	%xmm0, %ebx
141552214:     	movdqu	(%rdx,%r10), %xmm0
14155221a:     	pcmpeqb	%xmm1, %xmm0
14155221e:     	pmovmskb	%xmm0, %r9d
141552223:     	testl	%ebx, %ebx
141552225:     	je	0x141552230 <.text+0x1551230>
141552227:     	bsrw	%bx, %bx
14155222b:     	xorl	$0xf, %ebx
14155222e:     	jmp	0x141552234 <.text+0x1551234>
141552230:     	movw	$0x10, %bx
141552234:     	addq	%rdx, %r8
141552237:     	orl	$0x10000, %r9d          # imm = 0x10000
14155223e:     	tzcntl	%r9d, %r9d
141552243:     	addl	%r9d, %ebx
141552246:     	movb	$-0x80, %r9b
141552249:     	cmpw	$0xf, %bx
14155224d:     	ja	0x141552259 <.text+0x1551259>
14155224f:     	incq	0x90(%rdi)
141552256:     	movb	$-0x1, %r9b
141552259:     	movb	%r9b, (%rdx,%r10)
14155225d:     	movb	%r9b, 0x10(%rdx,%r11)
141552262:     	decq	0x98(%rdi)
141552269:     	movq	-0x78(%r8), %rdx
14155226d:     	cmpq	$0x2, %rdx
141552271:     	jne	0x14155247e <.text+0x155147e>
141552277:     	movq	0x8(%rsi), %rdx
14155227b:     	movq	(%rdx), %rbx
14155227e:     	leaq	0x1(%rbx), %r14
141552282:     	movq	%r14, (%rdx)
141552285:     	movabsq	$-0x8000000000000000, %rdx # imm = 0x8000000000000000
14155228f:     	movq	%rdx, 0x200(%rbp)
141552296:     	leaq	0x16e5779(%rip), %rdx   # 0x142c37a16
14155229d:     	movq	%rdx, 0x208(%rbp)
1415522a4:     	movq	$0x2f, 0x210(%rbp)
1415522af:     	movw	$0x100, 0x220(%rbp)     # imm = 0x100
1415522b8:     	movq	$0x1, 0x1c8(%rbp)
1415522c3:     	movq	%rcx, 0x1d0(%rbp)
1415522ca:     	movq	%rax, 0x1d8(%rbp)
1415522d1:     	movq	$0x8, 0x1f0(%rbp)
1415522dc:     	movq	$0x58, 0x1f8(%rbp)
1415522e7:     	leaq	-0x1407cce(%rip), %rax  # 0x14014a620 <.text+0x149620>
1415522ee:     	movq	%rax, 0x218(%rbp)
1415522f5:     	movb	$0x1, 0x222(%rbp)
1415522fc:     	movq	$0x0, 0x1e0(%rbp)
141552307:     	leaq	0x80(%rbp), %rcx
14155230e:     	leaq	0x1c8(%rbp), %r8
141552315:     	movq	%rbx, %rdx
141552318:     	callq	0x140d3f870 <.text+0xd3e870>
14155231d:     	cmpq	%r14, 0x10(%rdi)
141552321:     	jae	0x14155233c <.text+0x155133c>
141552323:     	movb	$0x1, 0x24f(%rbp)
14155232a:     	leaq	0x171a34f(%rip), %r8    # 0x142c6c680
141552331:     	movq	%rdi, %rcx
141552334:     	movq	%r14, %rdx
141552337:     	callq	0x1412f4340 <.text+0x12f3340>
14155233c:     	movq	0x8(%rdi), %r15
