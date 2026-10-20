
D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141673140:     	pushq	%rbp
141673141:     	pushq	%rsi
141673142:     	pushq	%rdi
141673143:     	pushq	%rbx
141673144:     	subq	$0x108, %rsp            # imm = 0x108
14167314b:     	leaq	0x80(%rsp), %rbp
141673153:     	movq	$-0x2, 0x80(%rbp)
14167315e:     	movq	%rdx, %rdi
141673161:     	movq	%rcx, %rbx
141673164:     	callq	0x140d61e00 <.text+0xd60e00>
141673169:     	movq	%rax, -0x8(%rbp)
14167316d:     	movq	%rdx, (%rbp)
141673171:     	leaq	-0x8(%rbp), %rcx
141673175:     	callq	0x14145aa70 <.text+0x1459a70>
14167317a:     	movq	%rax, %rsi
14167317d:     	leaq	-0x8(%rbp), %rcx
141673181:     	movq	%rbx, %rdx
141673184:     	movq	%rdi, %r8
141673187:     	callq	0x140d6a710 <.text+0xd69710>
14167318c:     	movq	-0x8(%rbp), %rcx
141673190:     	movq	(%rbp), %rax
141673194:     	cmpq	$0x2, %rcx
141673198:     	jne	0x1416731d7 <.text+0x16721d7>
14167319a:     	movl	%eax, %ecx
14167319c:     	andl	$0x3, %ecx
14167319f:     	cmpl	$0x1, %ecx
1416731a2:     	jne	0x1416732fb <.text+0x16722fb>
1416731a8:     	leaq	-0x1(%rax), %rcx
1416731ac:     	movq	%rcx, 0x68(%rbp)
1416731b0:     	movq	-0x1(%rax), %rcx
1416731b4:     	movq	%rcx, 0x70(%rbp)
1416731b8:     	movq	0x7(%rax), %rax
1416731bc:     	movq	%rax, 0x78(%rbp)
1416731c0:     	movq	(%rax), %rax
1416731c3:     	testq	%rax, %rax
1416731c6:     	je	0x1416732cd <.text+0x16722cd>
1416731cc:     	movq	0x70(%rbp), %rcx
1416731d0:     	callq	*%rax
1416731d2:     	jmp	0x1416732cd <.text+0x16722cd>
1416731d7:     	movq	0x48(%rbp), %rdx
1416731db:     	movq	%rdx, -0x10(%rbp)
1416731df:     	movups	0x8(%rbp), %xmm0
1416731e3:     	movups	0x18(%rbp), %xmm1
1416731e7:     	movups	0x28(%rbp), %xmm2
1416731eb:     	movups	0x38(%rbp), %xmm3
1416731ef:     	movups	%xmm3, -0x20(%rbp)
1416731f3:     	movups	%xmm2, -0x30(%rbp)
1416731f7:     	movups	%xmm1, -0x40(%rbp)
1416731fb:     	movups	%xmm0, -0x50(%rbp)
1416731ff:     	movq	%rcx, -0x60(%rbp)
141673203:     	movq	%rax, -0x58(%rbp)
141673207:     	movq	-0x28(%rbp), %rax
14167320b:     	movq	%rax, 0x58(%rbp)
14167320f:     	movq	%rsi, -0x8(%rbp)
141673213:     	leaq	0x58(%rbp), %rax
141673217:     	movq	%rax, (%rbp)
14167321b:     	leaq	-0x8(%rbp), %rcx
14167321f:     	callq	0x14145a8e0 <.text+0x14598e0>
141673224:     	movq	%rax, %rsi
141673227:     	leaq	0x58(%rbp), %rcx
14167322b:     	leaq	-0x60(%rbp), %rdx
14167322f:     	callq	0x140d59e60 <.text+0xd58e60>
141673234:     	movl	0x58(%rbp), %edi
141673237:     	testl	%edi, %edi
141673239:     	je	0x141673275 <.text+0x1672275>
14167323b:     	movq	0x60(%rbp), %rax
14167323f:     	movl	%eax, %ecx
141673241:     	andl	$0x3, %ecx
141673244:     	cmpl	$0x1, %ecx
141673247:     	jne	0x1416732fb <.text+0x16722fb>
14167324d:     	leaq	-0x1(%rax), %rcx
141673251:     	movq	%rcx, 0x68(%rbp)
141673255:     	movq	-0x1(%rax), %rcx
141673259:     	movq	%rcx, 0x70(%rbp)
14167325d:     	movq	0x7(%rax), %rax
141673261:     	movq	%rax, 0x78(%rbp)
141673265:     	movq	(%rax), %rax
141673268:     	testq	%rax, %rax
14167326b:     	je	0x1416732cd <.text+0x16722cd>
14167326d:     	movq	0x70(%rbp), %rcx
141673271:     	callq	*%rax
141673273:     	jmp	0x1416732cd <.text+0x16722cd>
141673275:     	movq	0x5c(%rbp), %rax
141673279:     	movq	%rax, 0x50(%rbp)
14167327d:     	movq	%rsi, -0x8(%rbp)
141673281:     	leaq	0x50(%rbp), %rax
141673285:     	movq	%rax, (%rbp)
141673289:     	leaq	-0x8(%rbp), %rcx
14167328d:     	callq	0x14145a8e0 <.text+0x14598e0>
141673292:     	movq	%rax, %rsi
141673295:     	testl	%edi, %edi
141673297:     	je	0x1416732fb <.text+0x16722fb>
141673299:     	movq	0x60(%rbp), %rax
14167329d:     	movl	%eax, %ecx
14167329f:     	andl	$0x3, %ecx
1416732a2:     	cmpl	$0x1, %ecx
1416732a5:     	jne	0x1416732fb <.text+0x16722fb>
1416732a7:     	leaq	-0x1(%rax), %rcx
1416732ab:     	movq	%rcx, 0x68(%rbp)
1416732af:     	movq	-0x1(%rax), %rcx
1416732b3:     	movq	%rcx, 0x70(%rbp)
1416732b7:     	movq	0x7(%rax), %rax
1416732bb:     	movq	%rax, 0x78(%rbp)
1416732bf:     	movq	(%rax), %rax
1416732c2:     	testq	%rax, %rax
1416732c5:     	je	0x1416732cd <.text+0x16722cd>
1416732c7:     	movq	0x70(%rbp), %rcx
1416732cb:     	callq	*%rax
1416732cd:     	movq	0x70(%rbp), %rcx
1416732d1:     	movq	0x78(%rbp), %rax
1416732d5:     	movq	0x8(%rax), %rdx
1416732d9:     	testq	%rdx, %rdx
1416732dc:     	je	0x1416732e7 <.text+0x16722e7>
1416732de:     	movq	0x10(%rax), %r8
1416732e2:     	callq	0x140613c20 <.text+0x612c20>
1416732e7:     	movl	$0x18, %edx
1416732ec:     	movl	$0x8, %r8d
1416732f2:     	movq	0x68(%rbp), %rcx
1416732f6:     	callq	0x140613c20 <.text+0x612c20>
1416732fb:     	movq	%rsi, %rax
1416732fe:     	addq	$0x108, %rsp            # imm = 0x108
141673305:     	popq	%rbx
141673306:     	popq	%rdi
141673307:     	popq	%rsi
141673308:     	popq	%rbp
141673309:     	retq
14167330a:     	nopw	(%rax,%rax)
141673310:     	movq	%rdx, 0x10(%rsp)
141673315:     	pushq	%rbp
141673316:     	pushq	%rsi
141673317:     	pushq	%rdi
141673318:     	pushq	%rbx
141673319:     	subq	$0x28, %rsp
14167331d:     	leaq	0x80(%rdx), %rbp
141673324:     	movq	0x70(%rbp), %rcx
141673328:     	movq	0x78(%rbp), %rax
14167332c:     	movq	0x8(%rax), %rdx
141673330:     	testq	%rdx, %rdx
141673333:     	je	0x141673342 <.text+0x1672342>
141673335:     	movq	0x78(%rbp), %rax
141673339:     	movq	0x10(%rax), %r8
14167333d:     	callq	0x140613c20 <.text+0x612c20>
141673342:     	movl	$0x18, %edx
141673347:     	movl	$0x8, %r8d
14167334d:     	movq	0x68(%rbp), %rcx
141673351:     	callq	0x140613c20 <.text+0x612c20>
141673356:     	nop
141673357:     	addq	$0x28, %rsp
14167335b:     	popq	%rbx
14167335c:     	popq	%rdi
14167335d:     	popq	%rsi
14167335e:     	popq	%rbp
14167335f:     	retq
141673360:     	movq	%rdx, 0x10(%rsp)
141673365:     	pushq	%rbp
141673366:     	pushq	%rsi
141673367:     	pushq	%rdi
141673368:     	pushq	%rbx
141673369:     	subq	$0x28, %rsp
14167336d:     	leaq	0x80(%rdx), %rbp
141673374:     	movq	0x70(%rbp), %rcx
141673378:     	movq	0x78(%rbp), %rax
14167337c:     	movq	0x8(%rax), %rdx
141673380:     	testq	%rdx, %rdx
141673383:     	je	0x141673392 <.text+0x1672392>
141673385:     	movq	0x78(%rbp), %rax
141673389:     	movq	0x10(%rax), %r8
14167338d:     	callq	0x140613c20 <.text+0x612c20>
141673392:     	movl	$0x18, %edx
141673397:     	movl	$0x8, %r8d
14167339d:     	movq	0x68(%rbp), %rcx
1416733a1:     	callq	0x140613c20 <.text+0x612c20>
1416733a6:     	nop
1416733a7:     	addq	$0x28, %rsp
1416733ab:     	popq	%rbx
1416733ac:     	popq	%rdi
1416733ad:     	popq	%rsi
1416733ae:     	popq	%rbp
1416733af:     	retq
1416733b0:     	movq	%rdx, 0x10(%rsp)
1416733b5:     	pushq	%rbp
1416733b6:     	pushq	%rsi
1416733b7:     	pushq	%rdi
1416733b8:     	pushq	%rbx
1416733b9:     	subq	$0x28, %rsp
1416733bd:     	leaq	0x80(%rdx), %rbp
1416733c4:     	movq	0x70(%rbp), %rcx
1416733c8:     	movq	0x78(%rbp), %rax
1416733cc:     	movq	0x8(%rax), %rdx
1416733d0:     	testq	%rdx, %rdx
1416733d3:     	je	0x1416733e2 <.text+0x16723e2>
1416733d5:     	movq	0x78(%rbp), %rax
1416733d9:     	movq	0x10(%rax), %r8
1416733dd:     	callq	0x140613c20 <.text+0x612c20>
1416733e2:     	movl	$0x18, %edx
1416733e7:     	movl	$0x8, %r8d
1416733ed:     	movq	0x68(%rbp), %rcx
1416733f1:     	callq	0x140613c20 <.text+0x612c20>
1416733f6:     	nop
1416733f7:     	addq	$0x28, %rsp
1416733fb:     	popq	%rbx
1416733fc:     	popq	%rdi
1416733fd:     	popq	%rsi
1416733fe:     	popq	%rbp
1416733ff:     	retq
