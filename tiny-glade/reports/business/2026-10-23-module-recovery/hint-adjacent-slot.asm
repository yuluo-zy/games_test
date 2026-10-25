
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14228b040:     	pushq	%r15
14228b042:     	pushq	%r14
14228b044:     	pushq	%r13
14228b046:     	pushq	%r12
14228b048:     	pushq	%rsi
14228b049:     	pushq	%rdi
14228b04a:     	pushq	%rbp
14228b04b:     	pushq	%rbx
14228b04c:     	subq	$0xd8, %rsp
14228b053:     	movq	%r9, %rdi
14228b056:     	movq	%rcx, %r14
14228b059:     	movq	%rdx, 0xc8(%rsp)
14228b061:     	movq	%r8, 0xd0(%rsp)
14228b069:     	leaq	0x10(%rcx), %rsi
14228b06d:     	movq	%rsi, %rcx
14228b070:     	callq	0x1406868b0 <.text+0x6858b0>
14228b075:     	movq	%rax, %r12
14228b078:     	leaq	0xc8(%rsp), %r15
14228b080:     	movq	%r15, %rcx
14228b083:     	callq	0x140683c10 <.text+0x682c10>
14228b088:     	movq	%rax, 0x28(%rsp)
14228b08d:     	leaq	0x28(%rsp), %rbx
14228b092:     	movq	%rbx, %rcx
14228b095:     	callq	0x1400885d0 <.text+0x875d0>
14228b09a:     	movq	%rax, 0x48(%rsp)
14228b09f:     	movabsq	$0x41223169ff28813b, %r13 # imm = 0x41223169FF28813B
14228b0a9:     	movq	%r13, 0x50(%rsp)
14228b0ae:     	movabsq	$-0x58648d975d569727, %rbp # imm = 0xA79B7268A2A968D9
14228b0b8:     	movq	%rbp, 0x58(%rsp)
14228b0bd:     	leaq	0x48(%rsp), %rcx
14228b0c2:     	callq	0x14228d080 <.text+0x228c080>
14228b0c7:     	movq	%r12, 0x48(%rsp)
14228b0cc:     	movq	%rax, 0x50(%rsp)
14228b0d1:     	leaq	0x48(%rsp), %rcx
14228b0d6:     	callq	0x14228d1f0 <.text+0x228c1f0>
14228b0db:     	movq	%rax, %r12
14228b0de:     	movq	%rsi, %rcx
14228b0e1:     	movq	%rax, %rdx
14228b0e4:     	callq	0x140686450 <.text+0x685450>
14228b0e9:     	testb	%al, %al
14228b0eb:     	je	0x14228b1bf <.text+0x228a1bf>
14228b0f1:     	movq	(%rdi), %r14
14228b0f4:     	movq	0x8(%rdi), %rbx
14228b0f8:     	movq	0x10(%rdi), %r13
14228b0fc:     	leaq	0x28(%rsp), %rcx
14228b101:     	movq	%rsi, %rdx
14228b104:     	movq	%r12, %r8
14228b107:     	callq	0x140686530 <.text+0x685530>
14228b10c:     	movq	0x10(%r14), %rax
14228b110:     	testq	%rax, %rax
14228b113:     	je	0x14228b3ba <.text+0x228a3ba>
14228b119:     	movq	0x8(%r14), %r15
14228b11d:     	shlq	$0x4, %rax
14228b121:     	leaq	(%rax,%rax,2), %r12
14228b125:     	xorl	%ebp, %ebp
14228b127:     	leaq	0x48(%rsp), %rdi
14228b12c:     	nopl	(%rax)
14228b130:     	leaq	0x30(%r15), %rsi
14228b134:     	movq	0x30(%rsp), %r14
14228b139:     	movq	%rbp, 0x20(%rsp)
14228b13e:     	incq	%rbp
14228b141:     	leaq	0x20(%rsp), %rcx
14228b146:     	callq	0x1400885d0 <.text+0x875d0>
14228b14b:     	movq	%rax, 0x48(%rsp)
14228b150:     	movabsq	$0x41223169ff28813b, %rax # imm = 0x41223169FF28813B
14228b15a:     	movq	%rax, 0x50(%rsp)
14228b15f:     	movabsq	$-0x58648d975d569727, %rax # imm = 0xA79B7268A2A968D9
14228b169:     	movq	%rax, 0x58(%rsp)
14228b16e:     	movq	%rdi, %rcx
14228b171:     	callq	0x14228d080 <.text+0x228c080>
14228b176:     	movq	%r14, 0x48(%rsp)
14228b17b:     	movq	%rax, 0x50(%rsp)
14228b180:     	movq	%rdi, %rcx
14228b183:     	callq	0x14228d1f0 <.text+0x228c1f0>
14228b188:     	movq	%rdi, %rcx
14228b18b:     	leaq	0x28(%rsp), %rdx
14228b190:     	movq	%rax, %r8
14228b193:     	callq	0x1406861b0 <.text+0x6851b0>
14228b198:     	movq	(%r13), %r9
14228b19c:     	incq	%r9
14228b19f:     	movq	%r15, %rcx
14228b1a2:     	movq	%rbx, %rdx
14228b1a5:     	movq	%rdi, %r8
14228b1a8:     	callq	0x141c52700 <.text+0x1c51700>
14228b1ad:     	movq	%rsi, %r15
14228b1b0:     	addq	$-0x30, %r12
14228b1b4:     	jne	0x14228b130 <.text+0x228a130>
14228b1ba:     	jmp	0x14228b3ba <.text+0x228a3ba>
14228b1bf:     	movq	0x13a11e2(%rip), %rax   # 0x14362c3a8
14228b1c6:     	movq	(%rax), %rax
14228b1c9:     	testq	%rax, %rax
14228b1cc:     	je	0x14228b2a4 <.text+0x228a2a4>
14228b1d2:     	movq	%r14, 0x28(%rsp)
14228b1d7:     	leaq	-0x1d5793e(%rip), %rax  # 0x1405338a0 <.text+0x5328a0>
14228b1de:     	movq	%rax, 0x30(%rsp)
14228b1e3:     	movq	%r15, 0x38(%rsp)
14228b1e8:     	movq	%rax, 0x40(%rsp)
14228b1ed:     	leaq	0xdaf42c(%rip), %rcx    # 0x14303a620
14228b1f4:     	callq	0x14067dd80 <.text+0x67cd80>
14228b1f9:     	movups	(%rax), %xmm0
14228b1fc:     	movl	0x10(%rax), %eax
14228b1ff:     	movq	$0x1, 0x78(%rsp)
14228b208:     	leaq	0xdaf429(%rip), %rcx    # 0x14303a638
14228b20f:     	movq	%rcx, 0x80(%rsp)
14228b217:     	movq	$0x14, 0x88(%rsp)
14228b223:     	leaq	0xdaf37e(%rip), %rdx    # 0x14303a5a8
14228b22a:     	movq	%rdx, 0x98(%rsp)
14228b232:     	movq	$0x2, 0xa0(%rsp)
14228b23e:     	movq	%rbx, 0xa8(%rsp)
14228b246:     	movq	$0x2, 0xb0(%rsp)
14228b252:     	movq	$0x0, 0xb8(%rsp)
14228b25e:     	movq	$0x0, 0x48(%rsp)
14228b267:     	movq	%rcx, 0x50(%rsp)
14228b26c:     	movq	$0x14, 0x58(%rsp)
14228b275:     	movq	$0x0, 0x60(%rsp)
14228b27e:     	movups	%xmm0, 0x68(%rsp)
14228b283:     	movl	$0x1, 0x90(%rsp)
14228b28e:     	movl	%eax, 0x94(%rsp)
14228b295:     	leaq	0x20(%rsp), %rcx
14228b29a:     	leaq	0x48(%rsp), %rdx
14228b29f:     	callq	0x140d4a330 <.text+0xd49330>
14228b2a4:     	movq	(%rdi), %r15
14228b2a7:     	movq	0x8(%rdi), %rbx
14228b2ab:     	movq	0x10(%rdi), %r12
14228b2af:     	movups	0xc8(%rsp), %xmm0
14228b2b7:     	movq	0x18(%r14), %rdi
14228b2bb:     	movups	%xmm0, 0x28(%rsp)
14228b2c0:     	leaq	0x28(%rsp), %rcx
14228b2c5:     	callq	0x140683c10 <.text+0x682c10>
14228b2ca:     	movq	%rax, 0x48(%rsp)
14228b2cf:     	movq	%r13, 0x50(%rsp)
14228b2d4:     	movq	%rbp, 0x58(%rsp)
14228b2d9:     	leaq	0x48(%rsp), %rcx
14228b2de:     	callq	0x14228d080 <.text+0x228c080>
14228b2e3:     	movq	%rdi, 0x48(%rsp)
14228b2e8:     	movq	%rax, 0x50(%rsp)
14228b2ed:     	leaq	0x48(%rsp), %rcx
14228b2f2:     	callq	0x14228d1f0 <.text+0x228c1f0>
14228b2f7:     	leaq	0x28(%rsp), %rcx
14228b2fc:     	movq	%rsi, %rdx
14228b2ff:     	movq	%rax, %r8
14228b302:     	callq	0x1406861b0 <.text+0x6851b0>
14228b307:     	movq	0x10(%r15), %rax
14228b30b:     	testq	%rax, %rax
14228b30e:     	je	0x14228b3ba <.text+0x228a3ba>
14228b314:     	movq	0x8(%r15), %r15
14228b318:     	shlq	$0x4, %rax
14228b31c:     	leaq	(%rax,%rax,2), %r13
14228b320:     	xorl	%ebp, %ebp
14228b322:     	leaq	0x48(%rsp), %rdi
14228b327:     	nopw	(%rax,%rax)
14228b330:     	leaq	0x30(%r15), %rsi
14228b334:     	movq	0x30(%rsp), %r14
14228b339:     	movq	%rbp, 0x20(%rsp)
14228b33e:     	incq	%rbp
14228b341:     	leaq	0x20(%rsp), %rcx
14228b346:     	callq	0x1400885d0 <.text+0x875d0>
14228b34b:     	movq	%rax, 0x48(%rsp)
14228b350:     	movabsq	$0x41223169ff28813b, %rax # imm = 0x41223169FF28813B
14228b35a:     	movq	%rax, 0x50(%rsp)
14228b35f:     	movabsq	$-0x58648d975d569727, %rax # imm = 0xA79B7268A2A968D9
14228b369:     	movq	%rax, 0x58(%rsp)
14228b36e:     	movq	%rdi, %rcx
14228b371:     	callq	0x14228d080 <.text+0x228c080>
14228b376:     	movq	%r14, 0x48(%rsp)
14228b37b:     	movq	%rax, 0x50(%rsp)
14228b380:     	movq	%rdi, %rcx
14228b383:     	callq	0x14228d1f0 <.text+0x228c1f0>
14228b388:     	movq	%rdi, %rcx
14228b38b:     	leaq	0x28(%rsp), %rdx
14228b390:     	movq	%rax, %r8
14228b393:     	callq	0x1406861b0 <.text+0x6851b0>
14228b398:     	movq	(%r12), %r9
14228b39c:     	incq	%r9
14228b39f:     	movq	%r15, %rcx
14228b3a2:     	movq	%rbx, %rdx
14228b3a5:     	movq	%rdi, %r8
14228b3a8:     	callq	0x141c52700 <.text+0x1c51700>
14228b3ad:     	movq	%rsi, %r15
14228b3b0:     	addq	$-0x30, %r13
14228b3b4:     	jne	0x14228b330 <.text+0x228a330>
14228b3ba:     	addq	$0xd8, %rsp
14228b3c1:     	popq	%rbx
14228b3c2:     	popq	%rbp
14228b3c3:     	popq	%rdi
14228b3c4:     	popq	%rsi
14228b3c5:     	popq	%r12
14228b3c7:     	popq	%r13
14228b3c9:     	popq	%r14
14228b3cb:     	popq	%r15
14228b3cd:     	retq
14228b3ce:     	int3
14228b3cf:     	int3
