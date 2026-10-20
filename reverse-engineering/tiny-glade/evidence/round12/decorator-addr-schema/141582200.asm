
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141582200: 41 57                       	pushq	%r15
141582202: 41 56                       	pushq	%r14
141582204: 41 54                       	pushq	%r12
141582206: 56                          	pushq	%rsi
141582207: 57                          	pushq	%rdi
141582208: 53                          	pushq	%rbx
141582209: 48 83 ec 28                 	subq	$0x28, %rsp
14158220d: 48 89 ce                    	movq	%rcx, %rsi
141582210: 4c 8b 72 08                 	movq	0x8(%rdx), %r14
141582214: 4c 8b 7a 10                 	movq	0x10(%rdx), %r15
141582218: 48 8b 39                    	movq	(%rcx), %rdi
14158221b: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
14158221f: 48 39 17                    	cmpq	%rdx, (%rdi)
141582222: 0f 84 32 02 00 00           	je	0x14158245a <.text+0x158145a>
141582228: 48 8b 47 08                 	movq	0x8(%rdi), %rax
14158222c: c6 04 10 5b                 	movb	$0x5b, (%rax,%rdx)
141582230: 48 ff c2                    	incq	%rdx
141582233: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
141582237: 4d 85 ff                    	testq	%r15, %r15
14158223a: 0f 84 87 00 00 00           	je	0x1415822c7 <.text+0x15812c7>
141582240: 49 8b 0e                    	movq	(%r14), %rcx
141582243: 49 8b 5e 08                 	movq	0x8(%r14), %rbx
141582247: 48 39 17                    	cmpq	%rdx, (%rdi)
14158224a: 0f 84 30 02 00 00           	je	0x141582480 <.text+0x1581480>
141582250: 48 8b 47 08                 	movq	0x8(%rdi), %rax
141582254: c6 04 10 5b                 	movb	$0x5b, (%rax,%rdx)
141582258: 48 ff c2                    	incq	%rdx
14158225b: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
14158225f: 48 89 f2                    	movq	%rsi, %rdx
141582262: e8 f9 e4 fd ff              	callq	0x141560760 <.text+0x155f760>
141582267: 48 85 c0                    	testq	%rax, %rax
14158226a: 75 75                       	jne	0x1415822e1 <.text+0x15812e1>
14158226c: 48 8b 3e                    	movq	(%rsi), %rdi
14158226f: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
141582273: 48 39 17                    	cmpq	%rdx, (%rdi)
141582276: 0f 84 56 02 00 00           	je	0x1415824d2 <.text+0x15814d2>
14158227c: 48 8b 47 08                 	movq	0x8(%rdi), %rax
141582280: c6 04 10 2c                 	movb	$0x2c, (%rax,%rdx)
141582284: 48 ff c2                    	incq	%rdx
141582287: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
14158228b: 48 89 d9                    	movq	%rbx, %rcx
14158228e: 48 89 f2                    	movq	%rsi, %rdx
141582291: e8 fa f1 23 00              	callq	0x1417c1490 <.text+0x17c0490>
141582296: 48 85 c0                    	testq	%rax, %rax
141582299: 75 46                       	jne	0x1415822e1 <.text+0x15812e1>
14158229b: 48 8b 3e                    	movq	(%rsi), %rdi
14158229e: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
1415822a2: 48 39 17                    	cmpq	%rdx, (%rdi)
1415822a5: 0f 84 4d 02 00 00           	je	0x1415824f8 <.text+0x15814f8>
1415822ab: 48 8b 47 08                 	movq	0x8(%rdi), %rax
1415822af: c6 04 10 5d                 	movb	$0x5d, (%rax,%rdx)
1415822b3: 48 ff c2                    	incq	%rdx
1415822b6: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
1415822ba: 49 83 ff 01                 	cmpq	$0x1, %r15
1415822be: 75 2f                       	jne	0x1415822ef <.text+0x15812ef>
1415822c0: 48 8b 3e                    	movq	(%rsi), %rdi
1415822c3: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
1415822c7: 48 39 17                    	cmpq	%rdx, (%rdi)
1415822ca: 0f 84 dc 01 00 00           	je	0x1415824ac <.text+0x15814ac>
1415822d0: 48 8b 47 08                 	movq	0x8(%rdi), %rax
1415822d4: c6 04 10 5d                 	movb	$0x5d, (%rax,%rdx)
1415822d8: 48 ff c2                    	incq	%rdx
1415822db: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
1415822df: 31 c0                       	xorl	%eax, %eax
1415822e1: 48 83 c4 28                 	addq	$0x28, %rsp
1415822e5: 5b                          	popq	%rbx
1415822e6: 5f                          	popq	%rdi
1415822e7: 5e                          	popq	%rsi
1415822e8: 41 5c                       	popq	%r12
1415822ea: 41 5e                       	popq	%r14
1415822ec: 41 5f                       	popq	%r15
1415822ee: c3                          	retq
1415822ef: 49 c1 e7 04                 	shlq	$0x4, %r15
1415822f3: 4d 01 f7                    	addq	%r14, %r15
1415822f6: 49 83 c6 10                 	addq	$0x10, %r14
1415822fa: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
141582300: 49 8b 0e                    	movq	(%r14), %rcx
141582303: 49 8b 7e 08                 	movq	0x8(%r14), %rdi
141582307: 48 8b 1e                    	movq	(%rsi), %rbx
14158230a: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
14158230e: 48 39 13                    	cmpq	%rdx, (%rbx)
141582311: 0f 84 9f 00 00 00           	je	0x1415823b6 <.text+0x15813b6>
141582317: 48 8b 43 08                 	movq	0x8(%rbx), %rax
14158231b: c6 04 10 2c                 	movb	$0x2c, (%rax,%rdx)
14158231f: 48 ff c2                    	incq	%rdx
141582322: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
141582326: 48 8b 1e                    	movq	(%rsi), %rbx
141582329: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
14158232d: 48 39 13                    	cmpq	%rdx, (%rbx)
141582330: 0f 84 ac 00 00 00           	je	0x1415823e2 <.text+0x15813e2>
141582336: 48 8b 43 08                 	movq	0x8(%rbx), %rax
14158233a: c6 04 10 5b                 	movb	$0x5b, (%rax,%rdx)
14158233e: 48 ff c2                    	incq	%rdx
141582341: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
141582345: 48 89 f2                    	movq	%rsi, %rdx
141582348: e8 13 e4 fd ff              	callq	0x141560760 <.text+0x155f760>
14158234d: 48 85 c0                    	testq	%rax, %rax
141582350: 75 8f                       	jne	0x1415822e1 <.text+0x15812e1>
141582352: 48 8b 1e                    	movq	(%rsi), %rbx
141582355: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
141582359: 48 39 13                    	cmpq	%rdx, (%rbx)
14158235c: 0f 84 ac 00 00 00           	je	0x14158240e <.text+0x158140e>
141582362: 48 8b 43 08                 	movq	0x8(%rbx), %rax
141582366: c6 04 10 2c                 	movb	$0x2c, (%rax,%rdx)
14158236a: 48 ff c2                    	incq	%rdx
14158236d: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
141582371: 48 89 f9                    	movq	%rdi, %rcx
141582374: 48 89 f2                    	movq	%rsi, %rdx
141582377: e8 14 f1 23 00              	callq	0x1417c1490 <.text+0x17c0490>
14158237c: 48 85 c0                    	testq	%rax, %rax
14158237f: 0f 85 5c ff ff ff           	jne	0x1415822e1 <.text+0x15812e1>
141582385: 48 8b 3e                    	movq	(%rsi), %rdi
141582388: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
14158238c: 48 39 17                    	cmpq	%rdx, (%rdi)
14158238f: 0f 84 9f 00 00 00           	je	0x141582434 <.text+0x1581434>
141582395: 48 8b 47 08                 	movq	0x8(%rdi), %rax
141582399: c6 04 10 5d                 	movb	$0x5d, (%rax,%rdx)
14158239d: 48 ff c2                    	incq	%rdx
1415823a0: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
1415823a4: 49 83 c6 10                 	addq	$0x10, %r14
1415823a8: 4d 39 fe                    	cmpq	%r15, %r14
1415823ab: 0f 85 4f ff ff ff           	jne	0x141582300 <.text+0x1581300>
1415823b1: e9 0a ff ff ff              	jmp	0x1415822c0 <.text+0x15812c0>
1415823b6: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1415823bf: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1415823c5: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1415823cb: 49 89 cc                    	movq	%rcx, %r12
1415823ce: 48 89 d9                    	movq	%rbx, %rcx
1415823d1: e8 8a 76 32 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1415823d6: 4c 89 e1                    	movq	%r12, %rcx
1415823d9: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1415823dd: e9 35 ff ff ff              	jmp	0x141582317 <.text+0x1581317>
1415823e2: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1415823eb: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1415823f1: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1415823f7: 49 89 cc                    	movq	%rcx, %r12
1415823fa: 48 89 d9                    	movq	%rbx, %rcx
1415823fd: e8 5e 76 32 01              	callq	0x1428a9a60 <.text+0x28a8a60>
141582402: 4c 89 e1                    	movq	%r12, %rcx
141582405: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
141582409: e9 28 ff ff ff              	jmp	0x141582336 <.text+0x1581336>
14158240e: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
141582417: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14158241d: 41 b9 01 00 00 00           	movl	$0x1, %r9d
141582423: 48 89 d9                    	movq	%rbx, %rcx
141582426: e8 35 76 32 01              	callq	0x1428a9a60 <.text+0x28a8a60>
14158242b: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
14158242f: e9 2e ff ff ff              	jmp	0x141582362 <.text+0x1581362>
141582434: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14158243d: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141582443: 41 b9 01 00 00 00           	movl	$0x1, %r9d
141582449: 48 89 f9                    	movq	%rdi, %rcx
14158244c: e8 0f 76 32 01              	callq	0x1428a9a60 <.text+0x28a8a60>
141582451: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
141582455: e9 3b ff ff ff              	jmp	0x141582395 <.text+0x1581395>
14158245a: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
141582463: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141582469: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14158246f: 48 89 f9                    	movq	%rdi, %rcx
141582472: e8 e9 75 32 01              	callq	0x1428a9a60 <.text+0x28a8a60>
141582477: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
14158247b: e9 a8 fd ff ff              	jmp	0x141582228 <.text+0x1581228>
141582480: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
141582489: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14158248f: 41 b9 01 00 00 00           	movl	$0x1, %r9d
141582495: 49 89 cc                    	movq	%rcx, %r12
141582498: 48 89 f9                    	movq	%rdi, %rcx
14158249b: e8 c0 75 32 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1415824a0: 4c 89 e1                    	movq	%r12, %rcx
1415824a3: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
1415824a7: e9 a4 fd ff ff              	jmp	0x141582250 <.text+0x1581250>
1415824ac: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1415824b5: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1415824bb: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1415824c1: 48 89 f9                    	movq	%rdi, %rcx
1415824c4: e8 97 75 32 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1415824c9: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
1415824cd: e9 fe fd ff ff              	jmp	0x1415822d0 <.text+0x15812d0>
1415824d2: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1415824db: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1415824e1: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1415824e7: 48 89 f9                    	movq	%rdi, %rcx
1415824ea: e8 71 75 32 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1415824ef: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
1415824f3: e9 84 fd ff ff              	jmp	0x14158227c <.text+0x158127c>
1415824f8: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
141582501: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141582507: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14158250d: 48 89 f9                    	movq	%rdi, %rcx
141582510: e8 4b 75 32 01              	callq	0x1428a9a60 <.text+0x28a8a60>
141582515: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
141582519: e9 8d fd ff ff              	jmp	0x1415822ab <.text+0x15812ab>
