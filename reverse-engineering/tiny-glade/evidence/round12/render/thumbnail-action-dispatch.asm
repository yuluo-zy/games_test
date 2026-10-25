
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140687eb0:     	pushq	%rbp
140687eb1:     	pushq	%r15
140687eb3:     	pushq	%r14
140687eb5:     	pushq	%r13
140687eb7:     	pushq	%r12
140687eb9:     	pushq	%rsi
140687eba:     	pushq	%rdi
140687ebb:     	pushq	%rbx
140687ebc:     	subq	$0x98, %rsp
140687ec3:     	leaq	0x80(%rsp), %rbp
140687ecb:     	movdqa	%xmm6, (%rbp)
140687ed0:     	movq	$-0x2, -0x8(%rbp)
140687ed8:     	movq	%r8, -0x60(%rbp)
140687edc:     	movq	%rdx, %rbx
140687edf:     	movq	%rcx, %r14
140687ee2:     	movq	%rdx, -0x18(%rbp)
140687ee6:     	movzbl	0x2facd1c(%rip), %eax   # 0x143634c09
140687eed:     	movl	$0x8, %ecx
140687ef2:     	movl	$0x8, %edx
140687ef7:     	callq	0x140613c10 <.text+0x612c10>
140687efc:     	testq	%rax, %rax
140687eff:     	je	0x1406883e6 <.text+0x6873e6>
140687f05:     	movq	%rbx, (%rax)
140687f08:     	movq	$0x1, -0x38(%rbp)
140687f10:     	movq	%rax, -0x30(%rbp)
140687f14:     	movq	$0x1, -0x28(%rbp)
140687f1c:     	cmpq	$0x0, 0x1e8(%r14)
140687f24:     	je	0x14068806c <.text+0x68706c>
140687f2a:     	leaq	0x1f0(%r14), %rcx
140687f31:     	movq	0x1d0(%r14), %r13
140687f38:     	movq	0x1d8(%r14), %r12
140687f3f:     	leaq	-0x10(%r13), %rax
140687f43:     	movq	%rax, -0x48(%rbp)
140687f47:     	leaq	-0x8(%r13), %rax
140687f4b:     	movq	%rax, -0x20(%rbp)
140687f4f:     	movl	$0x1, %r15d
140687f55:     	movq	$-0x8, %rdi
140687f5c:     	xorl	%esi, %esi
140687f5e:     	leaq	-0x18(%rbp), %rdx
140687f62:     	pcmpeqd	%xmm6, %xmm6
140687f66:     	movq	%rcx, -0x40(%rbp)
140687f6a:     	jmp	0x140687f99 <.text+0x686f99>
140687f6c:     	nopl	(%rax)
140687f70:     	movq	-0x30(%rbp), %rax
140687f74:     	movq	%r15, %rcx
140687f77:     	movq	%rbx, (%rax,%r15,8)
140687f7b:     	incq	%rcx
140687f7e:     	movq	%rcx, %r15
140687f81:     	movq	%rcx, -0x28(%rbp)
140687f85:     	movq	%rbx, -0x18(%rbp)
140687f89:     	addq	$0x8, %rsi
140687f8d:     	addq	$0x8, %rdi
140687f91:     	movq	-0x40(%rbp), %rcx
140687f95:     	leaq	-0x18(%rbp), %rdx
140687f99:     	callq	0x1405b0d70 <.text+0x5afd70>
140687f9e:     	movq	%rax, %rcx
140687fa1:     	shrq	$0x39, %rcx
140687fa5:     	movd	%ecx, %xmm0
140687fa9:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140687fad:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140687fb2:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140687fb7:     	xorl	%ecx, %ecx
140687fb9:     	andq	%r12, %rax
140687fbc:     	movdqu	(%r13,%rax), %xmm1
140687fc3:     	movdqa	%xmm1, %xmm2
140687fc7:     	pcmpeqb	%xmm0, %xmm2
140687fcb:     	pmovmskb	%xmm2, %edx
140687fcf:     	testl	%edx, %edx
140687fd1:     	je	0x140688000 <.text+0x687000>
140687fd3:     	tzcntl	%edx, %r8d
140687fd8:     	addq	%rax, %r8
140687fdb:     	andq	%r12, %r8
140687fde:     	movq	%r8, %r9
140687fe1:     	shlq	$0x4, %r9
140687fe5:     	movq	-0x48(%rbp), %r10
140687fe9:     	subq	%r9, %r10
140687fec:     	cmpq	(%r10), %rbx
140687fef:     	je	0x140688020 <.text+0x687020>
140687ff1:     	leal	-0x1(%rdx), %r8d
140687ff5:     	andw	%dx, %r8w
140687ff9:     	movl	%r8d, %edx
140687ffc:     	jne	0x140687fd3 <.text+0x686fd3>
140687ffe:     	nop
140688000:     	pcmpeqb	%xmm6, %xmm1
140688004:     	pmovmskb	%xmm1, %edx
140688008:     	testl	%edx, %edx
14068800a:     	jne	0x14068804e <.text+0x68704e>
14068800c:     	addq	%rcx, %rax
14068800f:     	addq	$0x10, %rax
140688013:     	addq	$0x10, %rcx
140688017:     	jmp	0x140687fb9 <.text+0x686fb9>
140688019:     	nopl	(%rax)
140688020:     	negq	%r8
140688023:     	shlq	$0x4, %r8
140688027:     	movq	-0x20(%rbp), %rax
14068802b:     	movq	(%rax,%r8), %rbx
14068802f:     	cmpq	-0x38(%rbp), %r15
140688033:     	jne	0x140687f70 <.text+0x686f70>
140688039:     	leaq	-0x38(%rbp), %rcx
14068803d:     	leaq	0x238fff4(%rip), %rdx   # 0x142a18038
140688044:     	callq	0x140562870 <.text+0x561870>
140688049:     	jmp	0x140687f70 <.text+0x686f70>
14068804e:     	movq	-0x30(%rbp), %rax
140688052:     	cmpq	$0x2, %r15
140688056:     	jb	0x140688112 <.text+0x687112>
14068805c:     	movq	%r15, %r10
14068805f:     	shrq	%r10
140688062:     	cmpq	$0x8, %r15
140688066:     	jae	0x140688077 <.text+0x687077>
140688068:     	xorl	%ecx, %ecx
14068806a:     	jmp	0x1406880e2 <.text+0x6870e2>
14068806c:     	movl	$0x1, %r15d
140688072:     	jmp	0x140688112 <.text+0x687112>
140688077:     	movq	%r10, %rcx
14068807a:     	andq	$-0x4, %rcx
14068807e:     	addq	%rax, %rdi
140688081:     	movq	%r10, %rdx
140688084:     	andq	$-0x4, %rdx
140688088:     	xorl	%r8d, %r8d
14068808b:     	nopl	(%rax,%rax)
140688090:     	movdqu	(%rax,%r8,8), %xmm0
140688096:     	movdqu	0x10(%rax,%r8,8), %xmm1
14068809d:     	movdqu	-0x10(%rdi), %xmm2
1406880a2:     	movdqu	(%rdi), %xmm3
1406880a6:     	pshufd	$0x4e, %xmm3, %xmm3     # xmm3 = xmm3[2,3,0,1]
1406880ab:     	pshufd	$0x4e, %xmm2, %xmm2     # xmm2 = xmm2[2,3,0,1]
1406880b0:     	movdqu	%xmm3, (%rax,%r8,8)
1406880b6:     	movdqu	%xmm2, 0x10(%rax,%r8,8)
1406880bd:     	pshufd	$0x4e, %xmm0, %xmm0     # xmm0 = xmm0[2,3,0,1]
1406880c2:     	movdqu	%xmm0, (%rdi)
1406880c6:     	pshufd	$0x4e, %xmm1, %xmm0     # xmm0 = xmm1[2,3,0,1]
1406880cb:     	movdqu	%xmm0, -0x10(%rdi)
1406880d0:     	addq	$0x4, %r8
1406880d4:     	addq	$-0x20, %rdi
1406880d8:     	cmpq	%r8, %rdx
1406880db:     	jne	0x140688090 <.text+0x687090>
1406880dd:     	cmpq	%rcx, %r10
1406880e0:     	je	0x14068810a <.text+0x68710a>
1406880e2:     	negq	%r10
1406880e5:     	leaq	(%rax,%rcx,8), %rdx
1406880e9:     	negq	%rcx
1406880ec:     	addq	%rax, %rsi
1406880ef:     	nop
1406880f0:     	movq	(%rdx), %r8
1406880f3:     	movq	(%rsi,%rcx,8), %r9
1406880f7:     	movq	%r9, (%rdx)
1406880fa:     	movq	%r8, (%rsi,%rcx,8)
1406880fe:     	decq	%rcx
140688101:     	addq	$0x8, %rdx
140688105:     	cmpq	%rcx, %r10
140688108:     	jne	0x1406880f0 <.text+0x6870f0>
14068810a:     	movq	-0x30(%rbp), %rax
14068810e:     	movq	-0x28(%rbp), %r15
140688112:     	movq	-0x38(%rbp), %rcx
140688116:     	movq	%rcx, -0x58(%rbp)
14068811a:     	movq	%r15, -0x50(%rbp)
14068811e:     	leaq	(%rax,%r15,8), %rdi
140688122:     	leaq	0x1c0(%r14), %rcx
140688129:     	movq	%rcx, -0x48(%rbp)
14068812d:     	movq	0x1b8(%r14), %r13
140688134:     	movq	0x1a0(%r14), %r12
14068813b:     	movq	0x1a8(%r14), %rsi
140688142:     	leaq	-0xc0(%r12), %rcx
14068814a:     	movq	%rcx, -0x40(%rbp)
14068814e:     	movq	%rax, %r14
140688151:     	xorl	%r15d, %r15d
140688154:     	pcmpeqd	%xmm6, %xmm6
140688158:     	movq	%rax, -0x20(%rbp)
14068815c:     	jmp	0x140688210 <.text+0x687210>
140688161:     	nopw	%cs:(%rax,%rax)
140688170:     	movdqu	(%r12,%rcx), %xmm1
140688176:     	movdqa	%xmm1, %xmm2
14068817a:     	pcmpeqb	%xmm0, %xmm2
14068817e:     	pmovmskb	%xmm2, %edx
140688182:     	testl	%edx, %edx
140688184:     	je	0x1406881c0 <.text+0x6871c0>
140688186:     	tzcntl	%edx, %r8d
14068818b:     	addq	%rcx, %r8
14068818e:     	andq	%rsi, %r8
140688191:     	negq	%r8
140688194:     	leaq	(%r8,%r8,2), %r8
140688198:     	shlq	$0x6, %r8
14068819c:     	movq	-0x40(%rbp), %r9
1406881a0:     	cmpq	(%r9,%r8), %rbx
1406881a4:     	je	0x1406881e0 <.text+0x6871e0>
1406881a6:     	leal	-0x1(%rdx), %r8d
1406881aa:     	andw	%dx, %r8w
1406881ae:     	movl	%r8d, %edx
1406881b1:     	jne	0x140688186 <.text+0x687186>
1406881b3:     	nopw	%cs:(%rax,%rax)
1406881c0:     	pcmpeqb	%xmm6, %xmm1
1406881c4:     	pmovmskb	%xmm1, %edx
1406881c8:     	testl	%edx, %edx
1406881ca:     	jne	0x140688260 <.text+0x687260>
1406881d0:     	addq	%rax, %rcx
1406881d3:     	addq	$0x10, %rcx
1406881d7:     	addq	$0x10, %rax
1406881db:     	andq	%rsi, %rcx
1406881de:     	jmp	0x140688170 <.text+0x687170>
1406881e0:     	movq	-0x38(%r12,%r8), %rcx
1406881e5:     	testq	%rcx, %rcx
1406881e8:     	je	0x140688260 <.text+0x687260>
1406881ea:     	addq	%r12, %r8
1406881ed:     	movq	-0x30(%r8), %rax
1406881f1:     	movw	$0x0, -0x18(%rbp)
1406881f7:     	movq	-0x60(%rbp), %rdx
1406881fb:     	leaq	-0x18(%rbp), %r8
1406881ff:     	callq	*0x50(%rax)
140688202:     	testb	%al, %al
140688204:     	je	0x140688260 <.text+0x687260>
140688206:     	jmp	0x140688386 <.text+0x687386>
14068820b:     	nopl	(%rax,%rax)
140688210:     	cmpq	%rdi, %r14
140688213:     	je	0x140688266 <.text+0x687266>
140688215:     	incq	%r15
140688218:     	movq	(%r14), %rbx
14068821b:     	movq	%rbx, -0x38(%rbp)
14068821f:     	testq	%r13, %r13
140688222:     	je	0x140688260 <.text+0x687260>
140688224:     	movq	-0x48(%rbp), %rcx
140688228:     	leaq	-0x38(%rbp), %rdx
14068822c:     	callq	0x1405b0d70 <.text+0x5afd70>
140688231:     	movq	%rsi, %rcx
140688234:     	andq	%rax, %rcx
140688237:     	shrq	$0x39, %rax
14068823b:     	movd	%eax, %xmm0
14068823f:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140688243:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140688248:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14068824d:     	xorl	%eax, %eax
14068824f:     	jmp	0x140688170 <.text+0x687170>
140688254:     	nopw	%cs:(%rax,%rax)
140688260:     	addq	$0x8, %r14
140688264:     	jmp	0x140688210 <.text+0x687210>
140688266:     	movl	$0x0, -0xc(%rbp)
14068826d:     	movq	-0x50(%rbp), %r15
140688271:     	movq	-0x20(%rbp), %rax
140688275:     	leaq	(%rax,%r15,8), %r14
140688279:     	leaq	-0x38(%rbp), %rdi
14068827d:     	pcmpeqd	%xmm6, %xmm6
140688281:     	leaq	-0x18(%rbp), %r15
140688285:     	jmp	0x140688330 <.text+0x687330>
14068828a:     	nopw	(%rax,%rax)
140688290:     	movdqu	(%r12,%rcx), %xmm1
140688296:     	movdqa	%xmm1, %xmm2
14068829a:     	pcmpeqb	%xmm0, %xmm2
14068829e:     	pmovmskb	%xmm2, %edx
1406882a2:     	testl	%edx, %edx
1406882a4:     	je	0x1406882e0 <.text+0x6872e0>
1406882a6:     	tzcntl	%edx, %r8d
1406882ab:     	addq	%rcx, %r8
1406882ae:     	andq	%rsi, %r8
1406882b1:     	negq	%r8
1406882b4:     	leaq	(%r8,%r8,2), %r8
1406882b8:     	shlq	$0x6, %r8
1406882bc:     	movq	-0x40(%rbp), %r9
1406882c0:     	cmpq	(%r9,%r8), %rbx
1406882c4:     	je	0x140688300 <.text+0x687300>
1406882c6:     	leal	-0x1(%rdx), %r8d
1406882ca:     	andw	%dx, %r8w
1406882ce:     	movl	%r8d, %edx
1406882d1:     	jne	0x1406882a6 <.text+0x6872a6>
1406882d3:     	nopw	%cs:(%rax,%rax)
1406882e0:     	pcmpeqb	%xmm6, %xmm1
1406882e4:     	pmovmskb	%xmm1, %edx
1406882e8:     	testl	%edx, %edx
1406882ea:     	jne	0x140688380 <.text+0x687380>
1406882f0:     	addq	%rax, %rcx
1406882f3:     	addq	$0x10, %rcx
1406882f7:     	addq	$0x10, %rax
1406882fb:     	andq	%rsi, %rcx
1406882fe:     	jmp	0x140688290 <.text+0x687290>
140688300:     	movq	-0x38(%r12,%r8), %rcx
140688305:     	testq	%rcx, %rcx
140688308:     	je	0x140688380 <.text+0x687380>
14068830a:     	addq	%r12, %r8
14068830d:     	movq	-0x30(%r8), %rax
140688311:     	movb	$0x1, -0x18(%rbp)
140688315:     	movl	-0xc(%rbp), %edx
140688318:     	movb	%dl, -0x17(%rbp)
14068831b:     	movq	-0x60(%rbp), %rdx
14068831f:     	movq	%r15, %r8
140688322:     	callq	*0x50(%rax)
140688325:     	testb	%al, %al
140688327:     	je	0x140688380 <.text+0x687380>
140688329:     	jmp	0x140688399 <.text+0x687399>
14068832b:     	nopl	(%rax,%rax)
140688330:     	cmpq	%r14, -0x20(%rbp)
140688334:     	je	0x14068839e <.text+0x68739e>
140688336:     	movq	-0x8(%r14), %rbx
14068833a:     	movq	%rbx, -0x38(%rbp)
14068833e:     	testq	%r13, %r13
140688341:     	je	0x140688380 <.text+0x687380>
140688343:     	movq	-0x48(%rbp), %rcx
140688347:     	movq	%rdi, %rdx
14068834a:     	callq	0x1405b0d70 <.text+0x5afd70>
14068834f:     	movq	%rsi, %rcx
140688352:     	andq	%rax, %rcx
140688355:     	shrq	$0x39, %rax
140688359:     	movd	%eax, %xmm0
14068835d:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140688361:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140688366:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14068836b:     	xorl	%eax, %eax
14068836d:     	jmp	0x140688290 <.text+0x687290>
140688372:     	nopw	%cs:(%rax,%rax)
140688380:     	addq	$-0x8, %r14
140688384:     	jmp	0x140688330 <.text+0x687330>
140688386:     	movq	-0x50(%rbp), %rdx
14068838a:     	cmpq	%rdx, %r15
14068838d:     	ja	0x1406883d5 <.text+0x6873d5>
14068838f:     	movb	$0x1, %al
140688391:     	movl	%eax, -0xc(%rbp)
140688394:     	jmp	0x140688271 <.text+0x687271>
140688399:     	movb	$0x1, %al
14068839b:     	movl	%eax, -0xc(%rbp)
14068839e:     	movq	-0x58(%rbp), %rdx
1406883a2:     	testq	%rdx, %rdx
1406883a5:     	movq	-0x20(%rbp), %rcx
1406883a9:     	je	0x1406883ba <.text+0x6873ba>
1406883ab:     	shlq	$0x3, %rdx
1406883af:     	movl	$0x8, %r8d
1406883b5:     	callq	0x140613c20 <.text+0x612c20>
1406883ba:     	movl	-0xc(%rbp), %eax
1406883bd:     	movaps	(%rbp), %xmm6
1406883c1:     	addq	$0x98, %rsp
1406883c8:     	popq	%rbx
1406883c9:     	popq	%rdi
1406883ca:     	popq	%rsi
1406883cb:     	popq	%r12
1406883cd:     	popq	%r13
1406883cf:     	popq	%r14
1406883d1:     	popq	%r15
1406883d3:     	popq	%rbp
1406883d4:     	retq
1406883d5:     	leaq	0x238fc44(%rip), %r8    # 0x142a18020
1406883dc:     	movq	%r15, %rcx
1406883df:     	callq	0x1428d9840 <.text+0x28d8840>
1406883e4:     	ud2
1406883e6:     	movl	$0x8, %ecx
1406883eb:     	movl	$0x8, %edx
1406883f0:     	callq	0x1428d9000 <.text+0x28d8000>
1406883f5:     	int3
1406883f6:     	nopw	%cs:(%rax,%rax)
140688400:     	movq	%rdx, 0x10(%rsp)
140688405:     	pushq	%rbp
140688406:     	pushq	%r15
140688408:     	pushq	%r14
14068840a:     	pushq	%r13
14068840c:     	pushq	%r12
14068840e:     	pushq	%rsi
14068840f:     	pushq	%rdi
140688410:     	pushq	%rbx
140688411:     	subq	$0x38, %rsp
140688415:     	leaq	0x80(%rdx), %rbp
14068841c:     	movdqa	%xmm6, 0x20(%rsp)
140688422:     	movq	-0x38(%rbp), %rdx
140688426:     	testq	%rdx, %rdx
140688429:     	je	0x14068843e <.text+0x68743e>
14068842b:     	movq	-0x30(%rbp), %rcx
14068842f:     	shlq	$0x3, %rdx
140688433:     	movl	$0x8, %r8d
140688439:     	callq	0x140613c20 <.text+0x612c20>
14068843e:     	movaps	0x20(%rsp), %xmm6
140688443:     	addq	$0x38, %rsp
140688447:     	popq	%rbx
140688448:     	popq	%rdi
140688449:     	popq	%rsi
14068844a:     	popq	%r12
14068844c:     	popq	%r13
14068844e:     	popq	%r14
140688450:     	popq	%r15
140688452:     	popq	%rbp
140688453:     	retq
140688454:     	nopw	%cs:(%rax,%rax)
140688460:     	movq	%rdx, 0x10(%rsp)
140688465:     	pushq	%rbp
140688466:     	pushq	%r15
140688468:     	pushq	%r14
14068846a:     	pushq	%r13
14068846c:     	pushq	%r12
14068846e:     	pushq	%rsi
14068846f:     	pushq	%rdi
140688470:     	pushq	%rbx
140688471:     	subq	$0x38, %rsp
140688475:     	leaq	0x80(%rdx), %rbp
14068847c:     	movdqa	%xmm6, 0x20(%rsp)
140688482:     	movq	-0x58(%rbp), %rdx
140688486:     	testq	%rdx, %rdx
140688489:     	je	0x14068849e <.text+0x68749e>
14068848b:     	shlq	$0x3, %rdx
14068848f:     	movl	$0x8, %r8d
140688495:     	movq	-0x20(%rbp), %rcx
140688499:     	callq	0x140613c20 <.text+0x612c20>
14068849e:     	movaps	0x20(%rsp), %xmm6
1406884a3:     	addq	$0x38, %rsp
1406884a7:     	popq	%rbx
1406884a8:     	popq	%rdi
1406884a9:     	popq	%rsi
1406884aa:     	popq	%r12
1406884ac:     	popq	%r13
1406884ae:     	popq	%r14
1406884b0:     	popq	%r15
1406884b2:     	popq	%rbp
1406884b3:     	retq
