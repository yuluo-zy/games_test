
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1400ae1b0: 41 57                       	pushq	%r15
1400ae1b2: 41 56                       	pushq	%r14
1400ae1b4: 41 55                       	pushq	%r13
1400ae1b6: 41 54                       	pushq	%r12
1400ae1b8: 56                          	pushq	%rsi
1400ae1b9: 57                          	pushq	%rdi
1400ae1ba: 55                          	pushq	%rbp
1400ae1bb: 53                          	pushq	%rbx
1400ae1bc: 48 81 ec 58 04 00 00        	subq	$0x458, %rsp            # imm = 0x458
1400ae1c3: bf 01 00 00 00              	movl	$0x1, %edi
1400ae1c8: f0                          	lock
1400ae1c9: 0f c1 ba b0 05 00 00        	xaddl	%edi, 0x5b0(%rdx)
1400ae1d0: 31 c0                       	xorl	%eax, %eax
1400ae1d2: 48 3b 81 38 01 00 00        	cmpq	0x138(%rcx), %rax
1400ae1d9: 0f 80 27 10 00 00           	jo	0x1400af206 <.text+0xae206>
1400ae1df: 48 89 ce                    	movq	%rcx, %rsi
1400ae1e2: 48 8b 89 70 01 00 00        	movq	0x170(%rcx), %rcx
1400ae1e9: 48 8b 82 18 02 00 00        	movq	0x218(%rdx), %rax
1400ae1f0: 48 39 c8                    	cmpq	%rcx, %rax
1400ae1f3: 0f 86 3b 0c 00 00           	jbe	0x1400aee34 <.text+0xade34>
1400ae1f9: 4c 8b 82 10 02 00 00        	movq	0x210(%rdx), %r8
1400ae200: 4d 8b 0c c8                 	movq	(%r8,%rcx,8), %r9
1400ae204: 4d 85 c9                    	testq	%r9, %r9
1400ae207: 0f 84 27 0c 00 00           	je	0x1400aee34 <.text+0xade34>
1400ae20d: 48 8b 8a e0 01 00 00        	movq	0x1e0(%rdx), %rcx
1400ae214: 49 f7 d1                    	notq	%r9
1400ae217: 4b 8d 1c 49                 	leaq	(%r9,%r9,2), %rbx
1400ae21b: 48 c1 e3 05                 	shlq	$0x5, %rbx
1400ae21f: 48 83 7c 19 20 00           	cmpq	$0x0, 0x20(%rcx,%rbx)
1400ae225: 0f 84 09 0c 00 00           	je	0x1400aee34 <.text+0xade34>
1400ae22b: 4c 8b 8e 78 01 00 00        	movq	0x178(%rsi), %r9
1400ae232: 4c 39 c8                    	cmpq	%r9, %rax
1400ae235: 0f 86 20 0c 00 00           	jbe	0x1400aee5b <.text+0xade5b>
1400ae23b: 4f 8b 0c c8                 	movq	(%r8,%r9,8), %r9
1400ae23f: 4d 85 c9                    	testq	%r9, %r9
1400ae242: 0f 84 13 0c 00 00           	je	0x1400aee5b <.text+0xade5b>
1400ae248: 49 f7 d1                    	notq	%r9
1400ae24b: 4f 8d 34 49                 	leaq	(%r9,%r9,2), %r14
1400ae24f: 49 c1 e6 05                 	shlq	$0x5, %r14
1400ae253: 4a 83 7c 31 20 00           	cmpq	$0x0, 0x20(%rcx,%r14)
1400ae259: 0f 84 fc 0b 00 00           	je	0x1400aee5b <.text+0xade5b>
1400ae25f: 4c 8b 8e 80 01 00 00        	movq	0x180(%rsi), %r9
1400ae266: 4c 39 c8                    	cmpq	%r9, %rax
1400ae269: 0f 86 13 0c 00 00           	jbe	0x1400aee82 <.text+0xade82>
1400ae26f: 4f 8b 0c c8                 	movq	(%r8,%r9,8), %r9
1400ae273: 4d 85 c9                    	testq	%r9, %r9
1400ae276: 0f 84 06 0c 00 00           	je	0x1400aee82 <.text+0xade82>
1400ae27c: 49 f7 d1                    	notq	%r9
1400ae27f: 4f 8d 24 49                 	leaq	(%r9,%r9,2), %r12
1400ae283: 49 c1 e4 05                 	shlq	$0x5, %r12
1400ae287: 4a 83 7c 21 20 00           	cmpq	$0x0, 0x20(%rcx,%r12)
1400ae28d: 0f 84 ef 0b 00 00           	je	0x1400aee82 <.text+0xade82>
1400ae293: 4c 8b 8e 88 01 00 00        	movq	0x188(%rsi), %r9
1400ae29a: 4c 39 c8                    	cmpq	%r9, %rax
1400ae29d: 0f 86 06 0c 00 00           	jbe	0x1400aeea9 <.text+0xadea9>
1400ae2a3: 4f 8b 0c c8                 	movq	(%r8,%r9,8), %r9
1400ae2a7: 4d 85 c9                    	testq	%r9, %r9
1400ae2aa: 0f 84 f9 0b 00 00           	je	0x1400aeea9 <.text+0xadea9>
1400ae2b0: 49 f7 d1                    	notq	%r9
1400ae2b3: 4b 8d 2c 49                 	leaq	(%r9,%r9,2), %rbp
1400ae2b7: 48 c1 e5 05                 	shlq	$0x5, %rbp
1400ae2bb: 48 83 7c 29 20 00           	cmpq	$0x0, 0x20(%rcx,%rbp)
1400ae2c1: 0f 84 e2 0b 00 00           	je	0x1400aeea9 <.text+0xadea9>
1400ae2c7: 4c 8b 8e 90 01 00 00        	movq	0x190(%rsi), %r9
1400ae2ce: 4c 39 c8                    	cmpq	%r9, %rax
1400ae2d1: 0f 86 f9 0b 00 00           	jbe	0x1400aeed0 <.text+0xaded0>
1400ae2d7: 4f 8b 0c c8                 	movq	(%r8,%r9,8), %r9
1400ae2db: 4d 85 c9                    	testq	%r9, %r9
1400ae2de: 0f 84 ec 0b 00 00           	je	0x1400aeed0 <.text+0xaded0>
1400ae2e4: 49 f7 d1                    	notq	%r9
1400ae2e7: 4f 8d 2c 49                 	leaq	(%r9,%r9,2), %r13
1400ae2eb: 49 c1 e5 05                 	shlq	$0x5, %r13
1400ae2ef: 4a 83 7c 29 20 00           	cmpq	$0x0, 0x20(%rcx,%r13)
1400ae2f5: 0f 84 d5 0b 00 00           	je	0x1400aeed0 <.text+0xaded0>
1400ae2fb: 4c 8b 8e 98 01 00 00        	movq	0x198(%rsi), %r9
1400ae302: 4c 39 c8                    	cmpq	%r9, %rax
1400ae305: 0f 86 ec 0b 00 00           	jbe	0x1400aeef7 <.text+0xadef7>
1400ae30b: 4f 8b 0c c8                 	movq	(%r8,%r9,8), %r9
1400ae30f: 4d 85 c9                    	testq	%r9, %r9
1400ae312: 0f 84 df 0b 00 00           	je	0x1400aeef7 <.text+0xadef7>
1400ae318: 49 f7 d1                    	notq	%r9
1400ae31b: 4f 8d 14 49                 	leaq	(%r9,%r9,2), %r10
1400ae31f: 49 c1 e2 05                 	shlq	$0x5, %r10
1400ae323: 4a 83 7c 11 20 00           	cmpq	$0x0, 0x20(%rcx,%r10)
1400ae329: 0f 84 c8 0b 00 00           	je	0x1400aeef7 <.text+0xadef7>
1400ae32f: 4c 8b 8e a0 01 00 00        	movq	0x1a0(%rsi), %r9
1400ae336: 4c 39 c8                    	cmpq	%r9, %rax
1400ae339: 0f 86 df 0b 00 00           	jbe	0x1400aef1e <.text+0xadf1e>
1400ae33f: 4f 8b 0c c8                 	movq	(%r8,%r9,8), %r9
1400ae343: 4d 85 c9                    	testq	%r9, %r9
1400ae346: 0f 84 d2 0b 00 00           	je	0x1400aef1e <.text+0xadf1e>
1400ae34c: 49 f7 d1                    	notq	%r9
1400ae34f: 4f 8d 1c 49                 	leaq	(%r9,%r9,2), %r11
1400ae353: 49 c1 e3 05                 	shlq	$0x5, %r11
1400ae357: 4a 83 7c 19 20 00           	cmpq	$0x0, 0x20(%rcx,%r11)
1400ae35d: 0f 84 bb 0b 00 00           	je	0x1400aef1e <.text+0xadf1e>
1400ae363: 4c 8b 8e a8 01 00 00        	movq	0x1a8(%rsi), %r9
1400ae36a: 4c 39 c8                    	cmpq	%r9, %rax
1400ae36d: 0f 86 d2 0b 00 00           	jbe	0x1400aef45 <.text+0xadf45>
1400ae373: 4b 8b 04 c8                 	movq	(%r8,%r9,8), %rax
1400ae377: 48 85 c0                    	testq	%rax, %rax
1400ae37a: 0f 84 c5 0b 00 00           	je	0x1400aef45 <.text+0xadf45>
1400ae380: 48 f7 d0                    	notq	%rax
1400ae383: 4c 8d 3c 40                 	leaq	(%rax,%rax,2), %r15
1400ae387: 49 c1 e7 05                 	shlq	$0x5, %r15
1400ae38b: 4a 83 7c 39 20 00           	cmpq	$0x0, 0x20(%rcx,%r15)
1400ae391: 0f 84 ae 0b 00 00           	je	0x1400aef45 <.text+0xadf45>
1400ae397: 48 01 cb                    	addq	%rcx, %rbx
1400ae39a: 48 8b 43 10                 	movq	0x10(%rbx), %rax
1400ae39e: 48 89 84 24 20 04 00 00     	movq	%rax, 0x420(%rsp)
1400ae3a6: 8b 86 30 01 00 00           	movl	0x130(%rsi), %eax
1400ae3ac: 49 01 ce                    	addq	%rcx, %r14
1400ae3af: 4d 8b 46 10                 	movq	0x10(%r14), %r8
1400ae3b3: 4c 89 84 24 28 04 00 00     	movq	%r8, 0x428(%rsp)
1400ae3bb: 49 01 cc                    	addq	%rcx, %r12
1400ae3be: 4d 8b 44 24 10              	movq	0x10(%r12), %r8
1400ae3c3: 4c 89 84 24 30 04 00 00     	movq	%r8, 0x430(%rsp)
1400ae3cb: 48 01 cd                    	addq	%rcx, %rbp
1400ae3ce: 4c 8b 45 10                 	movq	0x10(%rbp), %r8
1400ae3d2: 4c 89 84 24 38 04 00 00     	movq	%r8, 0x438(%rsp)
1400ae3da: 49 01 cd                    	addq	%rcx, %r13
1400ae3dd: 4d 8b 45 10                 	movq	0x10(%r13), %r8
1400ae3e1: 4c 89 84 24 40 04 00 00     	movq	%r8, 0x440(%rsp)
1400ae3e9: 49 01 ca                    	addq	%rcx, %r10
1400ae3ec: 4d 8b 42 10                 	movq	0x10(%r10), %r8
1400ae3f0: 4c 89 84 24 48 04 00 00     	movq	%r8, 0x448(%rsp)
1400ae3f8: 49 01 cb                    	addq	%rcx, %r11
1400ae3fb: 4d 8b 43 10                 	movq	0x10(%r11), %r8
1400ae3ff: 4c 89 84 24 50 04 00 00     	movq	%r8, 0x450(%rsp)
1400ae407: 49 01 cf                    	addq	%rcx, %r15
1400ae40a: 4d 8b 4f 10                 	movq	0x10(%r15), %r9
1400ae40e: 8b 8a b0 05 00 00           	movl	0x5b0(%rdx), %ecx
1400ae414: 48 8b 8e b0 01 00 00        	movq	0x1b0(%rsi), %rcx
1400ae41b: 48 39 8a 18 02 00 00        	cmpq	%rcx, 0x218(%rdx)
1400ae422: 0f 86 44 0b 00 00           	jbe	0x1400aef6c <.text+0xadf6c>
1400ae428: 4c 8b 82 10 02 00 00        	movq	0x210(%rdx), %r8
1400ae42f: 49 8b 0c c8                 	movq	(%r8,%rcx,8), %rcx
1400ae433: 48 85 c9                    	testq	%rcx, %rcx
1400ae436: 0f 84 30 0b 00 00           	je	0x1400aef6c <.text+0xadf6c>
1400ae43c: 4c 8b 82 e0 01 00 00        	movq	0x1e0(%rdx), %r8
1400ae443: 48 f7 d1                    	notq	%rcx
1400ae446: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1400ae44a: 48 c1 e1 05                 	shlq	$0x5, %rcx
1400ae44e: 49 83 7c 08 20 00           	cmpq	$0x0, 0x20(%r8,%rcx)
1400ae454: 0f 84 12 0b 00 00           	je	0x1400aef6c <.text+0xadf6c>
1400ae45a: 49 01 c8                    	addq	%rcx, %r8
1400ae45d: 4c 89 84 24 18 04 00 00     	movq	%r8, 0x418(%rsp)
1400ae465: 49 8b 48 10                 	movq	0x10(%r8), %rcx
1400ae469: 48 89 8c 24 10 04 00 00     	movq	%rcx, 0x410(%rsp)
1400ae471: 8b 8a b0 05 00 00           	movl	0x5b0(%rdx), %ecx
1400ae477: 48 8b 8e b8 01 00 00        	movq	0x1b8(%rsi), %rcx
1400ae47e: 48 39 8a 18 02 00 00        	cmpq	%rcx, 0x218(%rdx)
1400ae485: 0f 86 08 0b 00 00           	jbe	0x1400aef93 <.text+0xadf93>
1400ae48b: 4c 8b 82 10 02 00 00        	movq	0x210(%rdx), %r8
1400ae492: 49 8b 0c c8                 	movq	(%r8,%rcx,8), %rcx
1400ae496: 48 85 c9                    	testq	%rcx, %rcx
1400ae499: 0f 84 f4 0a 00 00           	je	0x1400aef93 <.text+0xadf93>
1400ae49f: 4c 8b 82 e0 01 00 00        	movq	0x1e0(%rdx), %r8
1400ae4a6: 48 f7 d1                    	notq	%rcx
1400ae4a9: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1400ae4ad: 48 c1 e1 05                 	shlq	$0x5, %rcx
1400ae4b1: 49 83 7c 08 20 00           	cmpq	$0x0, 0x20(%r8,%rcx)
1400ae4b7: 0f 84 d6 0a 00 00           	je	0x1400aef93 <.text+0xadf93>
1400ae4bd: 49 01 c8                    	addq	%rcx, %r8
1400ae4c0: 4c 89 84 24 08 04 00 00     	movq	%r8, 0x408(%rsp)
1400ae4c8: 49 8b 48 10                 	movq	0x10(%r8), %rcx
1400ae4cc: 48 89 8c 24 00 04 00 00     	movq	%rcx, 0x400(%rsp)
1400ae4d4: 8b 8a b0 05 00 00           	movl	0x5b0(%rdx), %ecx
1400ae4da: 48 8b 8e c0 01 00 00        	movq	0x1c0(%rsi), %rcx
1400ae4e1: 48 39 8a 18 02 00 00        	cmpq	%rcx, 0x218(%rdx)
1400ae4e8: 0f 86 cc 0a 00 00           	jbe	0x1400aefba <.text+0xadfba>
1400ae4ee: 4c 8b 82 10 02 00 00        	movq	0x210(%rdx), %r8
1400ae4f5: 49 8b 0c c8                 	movq	(%r8,%rcx,8), %rcx
1400ae4f9: 48 85 c9                    	testq	%rcx, %rcx
1400ae4fc: 0f 84 b8 0a 00 00           	je	0x1400aefba <.text+0xadfba>
1400ae502: 4c 8b 82 e0 01 00 00        	movq	0x1e0(%rdx), %r8
1400ae509: 48 f7 d1                    	notq	%rcx
1400ae50c: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1400ae510: 48 c1 e1 05                 	shlq	$0x5, %rcx
1400ae514: 49 83 7c 08 20 00           	cmpq	$0x0, 0x20(%r8,%rcx)
1400ae51a: 0f 84 9a 0a 00 00           	je	0x1400aefba <.text+0xadfba>
1400ae520: 4c 89 8c 24 f8 03 00 00     	movq	%r9, 0x3f8(%rsp)
1400ae528: 49 01 c8                    	addq	%rcx, %r8
1400ae52b: 4c 89 84 24 f0 03 00 00     	movq	%r8, 0x3f0(%rsp)
1400ae533: 49 8b 48 10                 	movq	0x10(%r8), %rcx
1400ae537: 48 89 8c 24 e8 03 00 00     	movq	%rcx, 0x3e8(%rsp)
1400ae53f: 8b 8a b0 05 00 00           	movl	0x5b0(%rdx), %ecx
1400ae545: 48 8b 8e c8 01 00 00        	movq	0x1c8(%rsi), %rcx
1400ae54c: 48 39 8a 18 02 00 00        	cmpq	%rcx, 0x218(%rdx)
1400ae553: 0f 86 88 0a 00 00           	jbe	0x1400aefe1 <.text+0xadfe1>
1400ae559: 4c 8b 82 10 02 00 00        	movq	0x210(%rdx), %r8
1400ae560: 4d 8b 04 c8                 	movq	(%r8,%rcx,8), %r8
1400ae564: 4d 85 c0                    	testq	%r8, %r8
1400ae567: 0f 84 74 0a 00 00           	je	0x1400aefe1 <.text+0xadfe1>
1400ae56d: 48 8b 8a e0 01 00 00        	movq	0x1e0(%rdx), %rcx
1400ae574: 49 f7 d0                    	notq	%r8
1400ae577: 4f 8d 04 40                 	leaq	(%r8,%r8,2), %r8
1400ae57b: 49 c1 e0 05                 	shlq	$0x5, %r8
1400ae57f: 4a 83 7c 01 20 00           	cmpq	$0x0, 0x20(%rcx,%r8)
1400ae585: 0f 84 56 0a 00 00           	je	0x1400aefe1 <.text+0xadfe1>
1400ae58b: 4c 89 9c 24 d8 03 00 00     	movq	%r11, 0x3d8(%rsp)
1400ae593: 4c 89 94 24 e0 03 00 00     	movq	%r10, 0x3e0(%rsp)
1400ae59b: 4c 01 c1                    	addq	%r8, %rcx
1400ae59e: 48 89 8c 24 d0 03 00 00     	movq	%rcx, 0x3d0(%rsp)
1400ae5a6: 48 8b 49 10                 	movq	0x10(%rcx), %rcx
1400ae5aa: 48 89 8c 24 c8 03 00 00     	movq	%rcx, 0x3c8(%rsp)
1400ae5b2: 8b 8a b0 05 00 00           	movl	0x5b0(%rdx), %ecx
1400ae5b8: 4c 8b 86 d0 01 00 00        	movq	0x1d0(%rsi), %r8
1400ae5bf: 4c 39 82 18 02 00 00        	cmpq	%r8, 0x218(%rdx)
1400ae5c6: 0f 86 3c 0a 00 00           	jbe	0x1400af008 <.text+0xae008>
1400ae5cc: 48 8b 8a 10 02 00 00        	movq	0x210(%rdx), %rcx
1400ae5d3: 4e 8b 0c c1                 	movq	(%rcx,%r8,8), %r9
1400ae5d7: 4d 85 c9                    	testq	%r9, %r9
1400ae5da: 0f 84 28 0a 00 00           	je	0x1400af008 <.text+0xae008>
1400ae5e0: 4c 8b 82 e0 01 00 00        	movq	0x1e0(%rdx), %r8
1400ae5e7: 49 f7 d1                    	notq	%r9
1400ae5ea: 4f 8d 0c 49                 	leaq	(%r9,%r9,2), %r9
1400ae5ee: 49 c1 e1 05                 	shlq	$0x5, %r9
1400ae5f2: 4b 83 7c 08 20 00           	cmpq	$0x0, 0x20(%r8,%r9)
1400ae5f8: 0f 84 0a 0a 00 00           	je	0x1400af008 <.text+0xae008>
1400ae5fe: 4d 01 c8                    	addq	%r9, %r8
1400ae601: 49 8b 48 10                 	movq	0x10(%r8), %rcx
1400ae605: 48 89 8c 24 b8 03 00 00     	movq	%rcx, 0x3b8(%rsp)
1400ae60d: 8b 8a b0 05 00 00           	movl	0x5b0(%rdx), %ecx
1400ae613: 4c 8b 9e d8 01 00 00        	movq	0x1d8(%rsi), %r11
1400ae61a: 4c 8b 8e e0 01 00 00        	movq	0x1e0(%rsi), %r9
1400ae621: 4c 8b 92 18 02 00 00        	movq	0x218(%rdx), %r10
1400ae628: 4d 39 da                    	cmpq	%r11, %r10
1400ae62b: 4c 89 ac 24 c0 03 00 00     	movq	%r13, 0x3c0(%rsp)
1400ae633: 76 4b                       	jbe	0x1400ae680 <.text+0xad680>
1400ae635: 48 8b 8a 10 02 00 00        	movq	0x210(%rdx), %rcx
1400ae63c: 4e 8b 1c d9                 	movq	(%rcx,%r11,8), %r11
1400ae640: 4d 85 db                    	testq	%r11, %r11
1400ae643: 74 3b                       	je	0x1400ae680 <.text+0xad680>
1400ae645: 48 8b 8a e0 01 00 00        	movq	0x1e0(%rdx), %rcx
1400ae64c: 49 f7 d3                    	notq	%r11
1400ae64f: 4f 8d 1c 5b                 	leaq	(%r11,%r11,2), %r11
1400ae653: 49 c1 e3 05                 	shlq	$0x5, %r11
1400ae657: 4a 83 7c 19 20 00           	cmpq	$0x0, 0x20(%rcx,%r11)
1400ae65d: 74 21                       	je	0x1400ae680 <.text+0xad680>
1400ae65f: 4c 01 d9                    	addq	%r11, %rcx
1400ae662: 4c 8b 59 10                 	movq	0x10(%rcx), %r11
1400ae666: 4c 8d 69 58                 	leaq	0x58(%rcx), %r13
1400ae66a: 4c 89 ac 24 50 03 00 00     	movq	%r13, 0x350(%rsp)
1400ae672: 48 83 c1 5c                 	addq	$0x5c, %rcx
1400ae676: 48 89 8c 24 58 03 00 00     	movq	%rcx, 0x358(%rsp)
1400ae67e: eb 03                       	jmp	0x1400ae683 <.text+0xad683>
1400ae680: 45 31 db                    	xorl	%r11d, %r11d
1400ae683: 4d 39 ca                    	cmpq	%r9, %r10
1400ae686: 0f 86 8c 09 00 00           	jbe	0x1400af018 <.text+0xae018>
1400ae68c: 48 8b 8a 10 02 00 00        	movq	0x210(%rdx), %rcx
1400ae693: 4e 8b 0c c9                 	movq	(%rcx,%r9,8), %r9
1400ae697: 4d 85 c9                    	testq	%r9, %r9
1400ae69a: 0f 84 78 09 00 00           	je	0x1400af018 <.text+0xae018>
1400ae6a0: 4c 8b aa e0 01 00 00        	movq	0x1e0(%rdx), %r13
1400ae6a7: 49 f7 d1                    	notq	%r9
1400ae6aa: 4f 8d 0c 49                 	leaq	(%r9,%r9,2), %r9
1400ae6ae: 49 c1 e1 05                 	shlq	$0x5, %r9
1400ae6b2: 4b 83 7c 0d 20 00           	cmpq	$0x0, 0x20(%r13,%r9)
1400ae6b8: 0f 84 5a 09 00 00           	je	0x1400af018 <.text+0xae018>
1400ae6be: 4c 89 9c 24 b0 03 00 00     	movq	%r11, 0x3b0(%rsp)
1400ae6c6: 4d 01 cd                    	addq	%r9, %r13
1400ae6c9: 49 8b 4d 10                 	movq	0x10(%r13), %rcx
1400ae6cd: 48 89 8c 24 a8 03 00 00     	movq	%rcx, 0x3a8(%rsp)
1400ae6d5: 8b 8a b0 05 00 00           	movl	0x5b0(%rdx), %ecx
1400ae6db: 4c 8b 8e e8 01 00 00        	movq	0x1e8(%rsi), %r9
1400ae6e2: 4c 39 8a 18 02 00 00        	cmpq	%r9, 0x218(%rdx)
1400ae6e9: 0f 86 4d 09 00 00           	jbe	0x1400af03c <.text+0xae03c>
1400ae6ef: 48 8b 8a 10 02 00 00        	movq	0x210(%rdx), %rcx
1400ae6f6: 4e 8b 14 c9                 	movq	(%rcx,%r9,8), %r10
1400ae6fa: 4d 85 d2                    	testq	%r10, %r10
1400ae6fd: 0f 84 39 09 00 00           	je	0x1400af03c <.text+0xae03c>
1400ae703: 4c 8b 8a e0 01 00 00        	movq	0x1e0(%rdx), %r9
1400ae70a: 49 f7 d2                    	notq	%r10
1400ae70d: 4f 8d 14 52                 	leaq	(%r10,%r10,2), %r10
1400ae711: 49 c1 e2 05                 	shlq	$0x5, %r10
1400ae715: 4b 83 7c 11 20 00           	cmpq	$0x0, 0x20(%r9,%r10)
1400ae71b: 0f 84 1b 09 00 00           	je	0x1400af03c <.text+0xae03c>
1400ae721: 4d 01 d1                    	addq	%r10, %r9
1400ae724: 49 8b 49 10                 	movq	0x10(%r9), %rcx
1400ae728: 48 89 8c 24 a0 03 00 00     	movq	%rcx, 0x3a0(%rsp)
1400ae730: 8b 8a b0 05 00 00           	movl	0x5b0(%rdx), %ecx
1400ae736: 4c 8b 96 f0 01 00 00        	movq	0x1f0(%rsi), %r10
1400ae73d: 4c 39 92 18 02 00 00        	cmpq	%r10, 0x218(%rdx)
1400ae744: 0f 86 16 09 00 00           	jbe	0x1400af060 <.text+0xae060>
1400ae74a: 48 8b 8a 10 02 00 00        	movq	0x210(%rdx), %rcx
1400ae751: 4e 8b 1c d1                 	movq	(%rcx,%r10,8), %r11
1400ae755: 4d 85 db                    	testq	%r11, %r11
1400ae758: 0f 84 02 09 00 00           	je	0x1400af060 <.text+0xae060>
1400ae75e: 4c 8b 92 e0 01 00 00        	movq	0x1e0(%rdx), %r10
1400ae765: 49 f7 d3                    	notq	%r11
1400ae768: 4f 8d 1c 5b                 	leaq	(%r11,%r11,2), %r11
1400ae76c: 49 c1 e3 05                 	shlq	$0x5, %r11
1400ae770: 4b 83 7c 1a 20 00           	cmpq	$0x0, 0x20(%r10,%r11)
1400ae776: 0f 84 e4 08 00 00           	je	0x1400af060 <.text+0xae060>
1400ae77c: 48 89 ac 24 80 03 00 00     	movq	%rbp, 0x380(%rsp)
1400ae784: 4c 89 a4 24 88 03 00 00     	movq	%r12, 0x388(%rsp)
1400ae78c: 4c 89 b4 24 90 03 00 00     	movq	%r14, 0x390(%rsp)
1400ae794: 48 89 9c 24 98 03 00 00     	movq	%rbx, 0x398(%rsp)
1400ae79c: 4d 01 da                    	addq	%r11, %r10
1400ae79f: 49 8b 4a 10                 	movq	0x10(%r10), %rcx
1400ae7a3: 48 89 8c 24 78 03 00 00     	movq	%rcx, 0x378(%rsp)
1400ae7ab: 8b 8a b0 05 00 00           	movl	0x5b0(%rdx), %ecx
1400ae7b1: 4c 8b 9e f8 01 00 00        	movq	0x1f8(%rsi), %r11
1400ae7b8: 4c 39 9a 18 02 00 00        	cmpq	%r11, 0x218(%rdx)
1400ae7bf: 0f 86 bf 08 00 00           	jbe	0x1400af084 <.text+0xae084>
1400ae7c5: 48 8b 8a 10 02 00 00        	movq	0x210(%rdx), %rcx
1400ae7cc: 4e 8b 1c d9                 	movq	(%rcx,%r11,8), %r11
1400ae7d0: 4d 85 db                    	testq	%r11, %r11
1400ae7d3: 0f 84 ab 08 00 00           	je	0x1400af084 <.text+0xae084>
1400ae7d9: 4c 8b a2 e0 01 00 00        	movq	0x1e0(%rdx), %r12
1400ae7e0: 49 f7 d3                    	notq	%r11
1400ae7e3: 4f 8d 1c 5b                 	leaq	(%r11,%r11,2), %r11
1400ae7e7: 49 c1 e3 05                 	shlq	$0x5, %r11
1400ae7eb: 4b 83 7c 1c 20 00           	cmpq	$0x0, 0x20(%r12,%r11)
1400ae7f1: 0f 84 8d 08 00 00           	je	0x1400af084 <.text+0xae084>
1400ae7f7: 4d 01 dc                    	addq	%r11, %r12
1400ae7fa: 49 8b 4c 24 10              	movq	0x10(%r12), %rcx
1400ae7ff: 48 89 8c 24 70 03 00 00     	movq	%rcx, 0x370(%rsp)
1400ae807: 8b 8a b0 05 00 00           	movl	0x5b0(%rdx), %ecx
1400ae80d: 4c 8b 9e 00 02 00 00        	movq	0x200(%rsi), %r11
1400ae814: 48 8b aa 18 02 00 00        	movq	0x218(%rdx), %rbp
1400ae81b: 4c 39 dd                    	cmpq	%r11, %rbp
1400ae81e: 0f 86 0f 09 00 00           	jbe	0x1400af133 <.text+0xae133>
1400ae824: 4c 8b b2 10 02 00 00        	movq	0x210(%rdx), %r14
1400ae82b: 4f 8b 1c de                 	movq	(%r14,%r11,8), %r11
1400ae82f: 4d 85 db                    	testq	%r11, %r11
1400ae832: 0f 84 fb 08 00 00           	je	0x1400af133 <.text+0xae133>
1400ae838: 48 8b 9a e0 01 00 00        	movq	0x1e0(%rdx), %rbx
1400ae83f: 49 f7 d3                    	notq	%r11
1400ae842: 4f 8d 1c 5b                 	leaq	(%r11,%r11,2), %r11
1400ae846: 49 c1 e3 05                 	shlq	$0x5, %r11
1400ae84a: 4a 83 7c 1b 20 00           	cmpq	$0x0, 0x20(%rbx,%r11)
1400ae850: 0f 84 dd 08 00 00           	je	0x1400af133 <.text+0xae133>
1400ae856: 48 8b 8e 08 02 00 00        	movq	0x208(%rsi), %rcx
1400ae85d: 48 39 cd                    	cmpq	%rcx, %rbp
1400ae860: 0f 86 f1 08 00 00           	jbe	0x1400af157 <.text+0xae157>
1400ae866: 4d 8b 34 ce                 	movq	(%r14,%rcx,8), %r14
1400ae86a: 4d 85 f6                    	testq	%r14, %r14
1400ae86d: 0f 84 e4 08 00 00           	je	0x1400af157 <.text+0xae157>
1400ae873: 49 f7 d6                    	notq	%r14
1400ae876: 4b 8d 2c 76                 	leaq	(%r14,%r14,2), %rbp
1400ae87a: 48 c1 e5 05                 	shlq	$0x5, %rbp
1400ae87e: 48 83 7c 2b 20 00           	cmpq	$0x0, 0x20(%rbx,%rbp)
1400ae884: 0f 84 cd 08 00 00           	je	0x1400af157 <.text+0xae157>
1400ae88a: 49 01 db                    	addq	%rbx, %r11
1400ae88d: 48 01 dd                    	addq	%rbx, %rbp
1400ae890: 49 8b 4b 10                 	movq	0x10(%r11), %rcx
1400ae894: 48 89 8c 24 68 03 00 00     	movq	%rcx, 0x368(%rsp)
1400ae89c: 48 8b 4d 10                 	movq	0x10(%rbp), %rcx
1400ae8a0: 48 89 8c 24 60 03 00 00     	movq	%rcx, 0x360(%rsp)
1400ae8a8: 48 8b 8c 24 20 04 00 00     	movq	0x420(%rsp), %rcx
1400ae8b0: 48 89 8c 24 b0 00 00 00     	movq	%rcx, 0xb0(%rsp)
1400ae8b8: 48 8b 9c 24 98 03 00 00     	movq	0x398(%rsp), %rbx
1400ae8c0: 48 8d 4b 5c                 	leaq	0x5c(%rbx), %rcx
1400ae8c4: 48 83 c3 58                 	addq	$0x58, %rbx
1400ae8c8: 48 89 9c 24 b8 00 00 00     	movq	%rbx, 0xb8(%rsp)
1400ae8d0: 48 89 8c 24 c0 00 00 00     	movq	%rcx, 0xc0(%rsp)
1400ae8d8: 89 84 24 c8 00 00 00        	movl	%eax, 0xc8(%rsp)
1400ae8df: 89 bc 24 cc 00 00 00        	movl	%edi, 0xcc(%rsp)
1400ae8e6: 48 8b 8c 24 28 04 00 00     	movq	0x428(%rsp), %rcx
1400ae8ee: 48 89 8c 24 d0 00 00 00     	movq	%rcx, 0xd0(%rsp)
1400ae8f6: 48 8b 9c 24 90 03 00 00     	movq	0x390(%rsp), %rbx
1400ae8fe: 48 8d 4b 58                 	leaq	0x58(%rbx), %rcx
1400ae902: 48 89 8c 24 d8 00 00 00     	movq	%rcx, 0xd8(%rsp)
1400ae90a: 48 83 c3 5c                 	addq	$0x5c, %rbx
1400ae90e: 48 89 9c 24 e0 00 00 00     	movq	%rbx, 0xe0(%rsp)
1400ae916: 89 84 24 e8 00 00 00        	movl	%eax, 0xe8(%rsp)
1400ae91d: 89 bc 24 ec 00 00 00        	movl	%edi, 0xec(%rsp)
1400ae924: 48 8b 8c 24 30 04 00 00     	movq	0x430(%rsp), %rcx
1400ae92c: 48 89 8c 24 f0 00 00 00     	movq	%rcx, 0xf0(%rsp)
1400ae934: 48 8b 9c 24 88 03 00 00     	movq	0x388(%rsp), %rbx
1400ae93c: 48 8d 4b 58                 	leaq	0x58(%rbx), %rcx
1400ae940: 48 89 8c 24 f8 00 00 00     	movq	%rcx, 0xf8(%rsp)
1400ae948: 48 83 c3 5c                 	addq	$0x5c, %rbx
1400ae94c: 48 89 9c 24 00 01 00 00     	movq	%rbx, 0x100(%rsp)
1400ae954: 89 84 24 08 01 00 00        	movl	%eax, 0x108(%rsp)
1400ae95b: 89 bc 24 0c 01 00 00        	movl	%edi, 0x10c(%rsp)
1400ae962: 48 8b 8c 24 38 04 00 00     	movq	0x438(%rsp), %rcx
1400ae96a: 48 89 8c 24 10 01 00 00     	movq	%rcx, 0x110(%rsp)
1400ae972: 48 8b 9c 24 80 03 00 00     	movq	0x380(%rsp), %rbx
1400ae97a: 48 8d 4b 58                 	leaq	0x58(%rbx), %rcx
1400ae97e: 48 89 8c 24 18 01 00 00     	movq	%rcx, 0x118(%rsp)
1400ae986: 48 83 c3 5c                 	addq	$0x5c, %rbx
1400ae98a: 48 89 9c 24 20 01 00 00     	movq	%rbx, 0x120(%rsp)
1400ae992: 89 84 24 28 01 00 00        	movl	%eax, 0x128(%rsp)
1400ae999: 89 bc 24 2c 01 00 00        	movl	%edi, 0x12c(%rsp)
1400ae9a0: 48 8b 8c 24 40 04 00 00     	movq	0x440(%rsp), %rcx
1400ae9a8: 48 89 8c 24 30 01 00 00     	movq	%rcx, 0x130(%rsp)
1400ae9b0: 48 8b 9c 24 c0 03 00 00     	movq	0x3c0(%rsp), %rbx
1400ae9b8: 48 8d 4b 5c                 	leaq	0x5c(%rbx), %rcx
1400ae9bc: 48 83 c3 58                 	addq	$0x58, %rbx
1400ae9c0: 48 89 9c 24 38 01 00 00     	movq	%rbx, 0x138(%rsp)
1400ae9c8: 48 89 8c 24 40 01 00 00     	movq	%rcx, 0x140(%rsp)
1400ae9d0: 89 84 24 48 01 00 00        	movl	%eax, 0x148(%rsp)
1400ae9d7: 89 bc 24 4c 01 00 00        	movl	%edi, 0x14c(%rsp)
1400ae9de: 48 8b 8c 24 48 04 00 00     	movq	0x448(%rsp), %rcx
1400ae9e6: 48 89 8c 24 50 01 00 00     	movq	%rcx, 0x150(%rsp)
1400ae9ee: 48 8b 9c 24 e0 03 00 00     	movq	0x3e0(%rsp), %rbx
1400ae9f6: 48 8d 4b 58                 	leaq	0x58(%rbx), %rcx
1400ae9fa: 48 89 8c 24 58 01 00 00     	movq	%rcx, 0x158(%rsp)
1400aea02: 48 83 c3 5c                 	addq	$0x5c, %rbx
1400aea06: 48 89 9c 24 60 01 00 00     	movq	%rbx, 0x160(%rsp)
1400aea0e: 89 84 24 68 01 00 00        	movl	%eax, 0x168(%rsp)
1400aea15: 89 bc 24 6c 01 00 00        	movl	%edi, 0x16c(%rsp)
1400aea1c: 48 8b 8c 24 50 04 00 00     	movq	0x450(%rsp), %rcx
1400aea24: 48 89 8c 24 70 01 00 00     	movq	%rcx, 0x170(%rsp)
1400aea2c: 48 8b 9c 24 d8 03 00 00     	movq	0x3d8(%rsp), %rbx
1400aea34: 48 8d 4b 5c                 	leaq	0x5c(%rbx), %rcx
1400aea38: 48 83 c3 58                 	addq	$0x58, %rbx
1400aea3c: 48 89 9c 24 78 01 00 00     	movq	%rbx, 0x178(%rsp)
1400aea44: 48 89 8c 24 80 01 00 00     	movq	%rcx, 0x180(%rsp)
1400aea4c: 89 84 24 88 01 00 00        	movl	%eax, 0x188(%rsp)
1400aea53: 89 bc 24 8c 01 00 00        	movl	%edi, 0x18c(%rsp)
1400aea5a: 48 8b 8c 24 f8 03 00 00     	movq	0x3f8(%rsp), %rcx
1400aea62: 48 89 8c 24 90 01 00 00     	movq	%rcx, 0x190(%rsp)
1400aea6a: 49 8d 4f 5c                 	leaq	0x5c(%r15), %rcx
1400aea6e: 49 83 c7 58                 	addq	$0x58, %r15
1400aea72: 4c 89 bc 24 98 01 00 00     	movq	%r15, 0x198(%rsp)
1400aea7a: 48 89 8c 24 a0 01 00 00     	movq	%rcx, 0x1a0(%rsp)
1400aea82: 89 84 24 a8 01 00 00        	movl	%eax, 0x1a8(%rsp)
1400aea89: 89 bc 24 ac 01 00 00        	movl	%edi, 0x1ac(%rsp)
1400aea90: 48 8b 8c 24 10 04 00 00     	movq	0x410(%rsp), %rcx
1400aea98: 48 89 8c 24 b0 01 00 00     	movq	%rcx, 0x1b0(%rsp)
1400aeaa0: 48 8b 9c 24 18 04 00 00     	movq	0x418(%rsp), %rbx
1400aeaa8: 48 8d 4b 5c                 	leaq	0x5c(%rbx), %rcx
1400aeaac: 48 83 c3 58                 	addq	$0x58, %rbx
1400aeab0: 48 89 9c 24 b8 01 00 00     	movq	%rbx, 0x1b8(%rsp)
1400aeab8: 48 89 8c 24 c0 01 00 00     	movq	%rcx, 0x1c0(%rsp)
1400aeac0: 89 84 24 c8 01 00 00        	movl	%eax, 0x1c8(%rsp)
1400aeac7: 89 bc 24 cc 01 00 00        	movl	%edi, 0x1cc(%rsp)
1400aeace: 48 8b 8c 24 00 04 00 00     	movq	0x400(%rsp), %rcx
1400aead6: 48 89 8c 24 d0 01 00 00     	movq	%rcx, 0x1d0(%rsp)
1400aeade: 48 8b 9c 24 08 04 00 00     	movq	0x408(%rsp), %rbx
1400aeae6: 48 8d 4b 5c                 	leaq	0x5c(%rbx), %rcx
1400aeaea: 48 83 c3 58                 	addq	$0x58, %rbx
1400aeaee: 48 89 9c 24 d8 01 00 00     	movq	%rbx, 0x1d8(%rsp)
1400aeaf6: 48 89 8c 24 e0 01 00 00     	movq	%rcx, 0x1e0(%rsp)
1400aeafe: 89 84 24 e8 01 00 00        	movl	%eax, 0x1e8(%rsp)
1400aeb05: 89 bc 24 ec 01 00 00        	movl	%edi, 0x1ec(%rsp)
1400aeb0c: 48 8b 8c 24 e8 03 00 00     	movq	0x3e8(%rsp), %rcx
1400aeb14: 48 89 8c 24 f0 01 00 00     	movq	%rcx, 0x1f0(%rsp)
1400aeb1c: 48 8b 9c 24 f0 03 00 00     	movq	0x3f0(%rsp), %rbx
1400aeb24: 48 8d 4b 5c                 	leaq	0x5c(%rbx), %rcx
1400aeb28: 48 83 c3 58                 	addq	$0x58, %rbx
1400aeb2c: 48 89 9c 24 f8 01 00 00     	movq	%rbx, 0x1f8(%rsp)
1400aeb34: 48 89 8c 24 00 02 00 00     	movq	%rcx, 0x200(%rsp)
1400aeb3c: 89 84 24 08 02 00 00        	movl	%eax, 0x208(%rsp)
1400aeb43: 89 bc 24 0c 02 00 00        	movl	%edi, 0x20c(%rsp)
1400aeb4a: 48 8b 8c 24 c8 03 00 00     	movq	0x3c8(%rsp), %rcx
1400aeb52: 48 89 8c 24 10 02 00 00     	movq	%rcx, 0x210(%rsp)
1400aeb5a: 48 8b 9c 24 d0 03 00 00     	movq	0x3d0(%rsp), %rbx
1400aeb62: 48 8d 4b 5c                 	leaq	0x5c(%rbx), %rcx
1400aeb66: 48 83 c3 58                 	addq	$0x58, %rbx
1400aeb6a: 48 89 9c 24 18 02 00 00     	movq	%rbx, 0x218(%rsp)
1400aeb72: 48 89 8c 24 20 02 00 00     	movq	%rcx, 0x220(%rsp)
1400aeb7a: 89 84 24 28 02 00 00        	movl	%eax, 0x228(%rsp)
1400aeb81: 89 bc 24 2c 02 00 00        	movl	%edi, 0x22c(%rsp)
1400aeb88: 48 8b 8c 24 b8 03 00 00     	movq	0x3b8(%rsp), %rcx
1400aeb90: 48 89 8c 24 30 02 00 00     	movq	%rcx, 0x230(%rsp)
1400aeb98: 49 8d 48 5c                 	leaq	0x5c(%r8), %rcx
1400aeb9c: 49 83 c0 58                 	addq	$0x58, %r8
1400aeba0: 4c 89 84 24 38 02 00 00     	movq	%r8, 0x238(%rsp)
1400aeba8: 48 89 8c 24 40 02 00 00     	movq	%rcx, 0x240(%rsp)
1400aebb0: 89 84 24 48 02 00 00        	movl	%eax, 0x248(%rsp)
1400aebb7: 89 bc 24 4c 02 00 00        	movl	%edi, 0x24c(%rsp)
1400aebbe: 48 8b 8c 24 a8 03 00 00     	movq	0x3a8(%rsp), %rcx
1400aebc6: 48 89 8c 24 50 02 00 00     	movq	%rcx, 0x250(%rsp)
1400aebce: 49 8d 4d 5c                 	leaq	0x5c(%r13), %rcx
1400aebd2: 49 83 c5 58                 	addq	$0x58, %r13
1400aebd6: 4c 89 ac 24 58 02 00 00     	movq	%r13, 0x258(%rsp)
1400aebde: 48 89 8c 24 60 02 00 00     	movq	%rcx, 0x260(%rsp)
1400aebe6: 89 84 24 68 02 00 00        	movl	%eax, 0x268(%rsp)
1400aebed: 89 bc 24 6c 02 00 00        	movl	%edi, 0x26c(%rsp)
1400aebf4: 48 8b 8c 24 b0 03 00 00     	movq	0x3b0(%rsp), %rcx
1400aebfc: 48 89 8c 24 70 02 00 00     	movq	%rcx, 0x270(%rsp)
1400aec04: 48 8b 8c 24 50 03 00 00     	movq	0x350(%rsp), %rcx
1400aec0c: 48 89 8c 24 78 02 00 00     	movq	%rcx, 0x278(%rsp)
1400aec14: 4c 8d 86 38 01 00 00        	leaq	0x138(%rsi), %r8
1400aec1b: 48 8b 8c 24 58 03 00 00     	movq	0x358(%rsp), %rcx
1400aec23: 48 89 8c 24 80 02 00 00     	movq	%rcx, 0x280(%rsp)
1400aec2b: 49 8d 4b 5c                 	leaq	0x5c(%r11), %rcx
1400aec2f: 49 83 c3 58                 	addq	$0x58, %r11
1400aec33: 89 84 24 88 02 00 00        	movl	%eax, 0x288(%rsp)
1400aec3a: 89 bc 24 8c 02 00 00        	movl	%edi, 0x28c(%rsp)
1400aec41: 48 8b 9c 24 a0 03 00 00     	movq	0x3a0(%rsp), %rbx
1400aec49: 48 89 9c 24 90 02 00 00     	movq	%rbx, 0x290(%rsp)
1400aec51: 49 8d 59 5c                 	leaq	0x5c(%r9), %rbx
1400aec55: 49 83 c1 58                 	addq	$0x58, %r9
1400aec59: 4c 89 8c 24 98 02 00 00     	movq	%r9, 0x298(%rsp)
1400aec61: 4d 8d 6c 24 5c              	leaq	0x5c(%r12), %r13
1400aec66: 49 83 c4 58                 	addq	$0x58, %r12
1400aec6a: 48 89 9c 24 a0 02 00 00     	movq	%rbx, 0x2a0(%rsp)
1400aec72: 4d 8d 4a 58                 	leaq	0x58(%r10), %r9
1400aec76: 49 83 c2 5c                 	addq	$0x5c, %r10
1400aec7a: 89 84 24 a8 02 00 00        	movl	%eax, 0x2a8(%rsp)
1400aec81: 89 bc 24 ac 02 00 00        	movl	%edi, 0x2ac(%rsp)
1400aec88: 48 89 94 24 b0 02 00 00     	movq	%rdx, 0x2b0(%rsp)
1400aec90: 4c 8d 75 58                 	leaq	0x58(%rbp), %r14
1400aec94: 48 83 c5 5c                 	addq	$0x5c, %rbp
1400aec98: 48 c7 84 24 b8 02 00 00 00 00 00 00 	movq	$0x0, 0x2b8(%rsp)
1400aeca4: 4c 89 84 24 c0 02 00 00     	movq	%r8, 0x2c0(%rsp)
1400aecac: 48 8b 94 24 78 03 00 00     	movq	0x378(%rsp), %rdx
1400aecb4: 48 89 94 24 d0 02 00 00     	movq	%rdx, 0x2d0(%rsp)
1400aecbc: 4c 8d 84 24 50 01 00 00     	leaq	0x150(%rsp), %r8
1400aecc4: 4c 89 8c 24 d8 02 00 00     	movq	%r9, 0x2d8(%rsp)
1400aeccc: 4c 8d 8c 24 90 01 00 00     	leaq	0x190(%rsp), %r9
1400aecd4: 4c 89 94 24 e0 02 00 00     	movq	%r10, 0x2e0(%rsp)
1400aecdc: 4c 8d 94 24 b0 01 00 00     	leaq	0x1b0(%rsp), %r10
1400aece4: 89 84 24 e8 02 00 00        	movl	%eax, 0x2e8(%rsp)
1400aeceb: 89 bc 24 ec 02 00 00        	movl	%edi, 0x2ec(%rsp)
1400aecf2: 48 8b 94 24 70 03 00 00     	movq	0x370(%rsp), %rdx
1400aecfa: 48 89 94 24 f0 02 00 00     	movq	%rdx, 0x2f0(%rsp)
1400aed02: 48 8d 9c 24 d0 01 00 00     	leaq	0x1d0(%rsp), %rbx
1400aed0a: 4c 89 a4 24 f8 02 00 00     	movq	%r12, 0x2f8(%rsp)
1400aed12: 4c 8d bc 24 f0 01 00 00     	leaq	0x1f0(%rsp), %r15
1400aed1a: 4c 89 ac 24 00 03 00 00     	movq	%r13, 0x300(%rsp)
1400aed22: 4c 8d a4 24 10 02 00 00     	leaq	0x210(%rsp), %r12
1400aed2a: 89 84 24 08 03 00 00        	movl	%eax, 0x308(%rsp)
1400aed31: 89 bc 24 0c 03 00 00        	movl	%edi, 0x30c(%rsp)
1400aed38: 48 8b 94 24 68 03 00 00     	movq	0x368(%rsp), %rdx
1400aed40: 48 89 94 24 10 03 00 00     	movq	%rdx, 0x310(%rsp)
1400aed48: 4c 8d ac 24 30 02 00 00     	leaq	0x230(%rsp), %r13
1400aed50: 4c 89 9c 24 18 03 00 00     	movq	%r11, 0x318(%rsp)
1400aed58: 4c 8d 9c 24 50 02 00 00     	leaq	0x250(%rsp), %r11
1400aed60: 48 89 8c 24 20 03 00 00     	movq	%rcx, 0x320(%rsp)
1400aed68: 48 8d 94 24 90 02 00 00     	leaq	0x290(%rsp), %rdx
1400aed70: 89 84 24 28 03 00 00        	movl	%eax, 0x328(%rsp)
1400aed77: 89 bc 24 2c 03 00 00        	movl	%edi, 0x32c(%rsp)
1400aed7e: 48 8b 8c 24 60 03 00 00     	movq	0x360(%rsp), %rcx
1400aed86: 48 89 8c 24 30 03 00 00     	movq	%rcx, 0x330(%rsp)
1400aed8e: 48 8d 8c 24 b0 02 00 00     	leaq	0x2b0(%rsp), %rcx
1400aed96: 4c 89 b4 24 38 03 00 00     	movq	%r14, 0x338(%rsp)
1400aed9e: 4c 8d b4 24 d0 02 00 00     	leaq	0x2d0(%rsp), %r14
1400aeda6: 48 89 ac 24 40 03 00 00     	movq	%rbp, 0x340(%rsp)
1400aedae: 48 8d ac 24 10 03 00 00     	leaq	0x310(%rsp), %rbp
1400aedb6: 89 84 24 48 03 00 00        	movl	%eax, 0x348(%rsp)
1400aedbd: 48 8d 84 24 30 03 00 00     	leaq	0x330(%rsp), %rax
1400aedc5: 89 bc 24 4c 03 00 00        	movl	%edi, 0x34c(%rsp)
1400aedcc: 48 89 44 24 70              	movq	%rax, 0x70(%rsp)
1400aedd1: 48 89 6c 24 68              	movq	%rbp, 0x68(%rsp)
1400aedd6: 4c 89 74 24 60              	movq	%r14, 0x60(%rsp)
1400aeddb: 48 89 4c 24 58              	movq	%rcx, 0x58(%rsp)
1400aede0: 48 89 54 24 50              	movq	%rdx, 0x50(%rsp)
1400aede5: 4c 89 5c 24 48              	movq	%r11, 0x48(%rsp)
1400aedea: 4c 89 6c 24 40              	movq	%r13, 0x40(%rsp)
1400aedef: 4c 89 64 24 38              	movq	%r12, 0x38(%rsp)
1400aedf4: 4c 89 7c 24 30              	movq	%r15, 0x30(%rsp)
1400aedf9: 48 89 5c 24 28              	movq	%rbx, 0x28(%rsp)
1400aedfe: 4c 89 54 24 20              	movq	%r10, 0x20(%rsp)
1400aee03: 48 8d 8c 24 b0 00 00 00     	leaq	0xb0(%rsp), %rcx
1400aee0b: 48 8d 94 24 d0 00 00 00     	leaq	0xd0(%rsp), %rdx
1400aee13: e8 98 0b 32 00              	callq	0x1403cf9b0 <.text+0x3ce9b0>
1400aee18: 89 be 30 01 00 00           	movl	%edi, 0x130(%rsi)
1400aee1e: 31 c0                       	xorl	%eax, %eax
1400aee20: 48 81 c4 58 04 00 00        	addq	$0x458, %rsp            # imm = 0x458
1400aee27: 5b                          	popq	%rbx
1400aee28: 5d                          	popq	%rbp
1400aee29: 5f                          	popq	%rdi
1400aee2a: 5e                          	popq	%rsi
1400aee2b: 41 5c                       	popq	%r12
1400aee2d: 41 5d                       	popq	%r13
1400aee2f: 41 5e                       	popq	%r14
1400aee31: 41 5f                       	popq	%r15
1400aee33: c3                          	retq
1400aee34: 48 81 c6 98 00 00 00        	addq	$0x98, %rsi
1400aee3b: 48 8d 05 82 47 88 02        	leaq	0x2884782(%rip), %rax   # 0x1429335c4
1400aee42: 48 89 84 24 80 00 00 00     	movq	%rax, 0x80(%rsp)
1400aee4a: 48 c7 84 24 88 00 00 00 30 00 00 00 	movq	$0x30, 0x88(%rsp)
1400aee56: e9 1e 03 00 00              	jmp	0x1400af179 <.text+0xae179>
1400aee5b: 48 81 c6 98 00 00 00        	addq	$0x98, %rsi
1400aee62: 48 8d 05 08 47 88 02        	leaq	0x2884708(%rip), %rax   # 0x142933571
1400aee69: 48 89 84 24 80 00 00 00     	movq	%rax, 0x80(%rsp)
1400aee71: 48 c7 84 24 88 00 00 00 2c 00 00 00 	movq	$0x2c, 0x88(%rsp)
1400aee7d: e9 f7 02 00 00              	jmp	0x1400af179 <.text+0xae179>
1400aee82: 48 81 c6 98 00 00 00        	addq	$0x98, %rsi
1400aee89: 48 8d 05 30 46 88 02        	leaq	0x2884630(%rip), %rax   # 0x1429334c0
1400aee90: 48 89 84 24 80 00 00 00     	movq	%rax, 0x80(%rsp)
1400aee98: 48 c7 84 24 88 00 00 00 55 00 00 00 	movq	$0x55, 0x88(%rsp)
1400aeea4: e9 d0 02 00 00              	jmp	0x1400af179 <.text+0xae179>
1400aeea9: 48 81 c6 98 00 00 00        	addq	$0x98, %rsi
1400aeeb0: 48 8d 05 3d 47 88 02        	leaq	0x288473d(%rip), %rax   # 0x1429335f4
1400aeeb7: 48 89 84 24 80 00 00 00     	movq	%rax, 0x80(%rsp)
1400aeebf: 48 c7 84 24 88 00 00 00 2a 00 00 00 	movq	$0x2a, 0x88(%rsp)
1400aeecb: e9 a9 02 00 00              	jmp	0x1400af179 <.text+0xae179>
1400aeed0: 48 81 c6 98 00 00 00        	addq	$0x98, %rsi
1400aeed7: 48 8d 05 ed 44 88 02        	leaq	0x28844ed(%rip), %rax   # 0x1429333cb
1400aeede: 48 89 84 24 80 00 00 00     	movq	%rax, 0x80(%rsp)
1400aeee6: 48 c7 84 24 88 00 00 00 2d 00 00 00 	movq	$0x2d, 0x88(%rsp)
1400aeef2: e9 82 02 00 00              	jmp	0x1400af179 <.text+0xae179>
1400aeef7: 48 81 c6 98 00 00 00        	addq	$0x98, %rsi
1400aeefe: 48 8d 05 47 46 88 02        	leaq	0x2884647(%rip), %rax   # 0x14293354c
1400aef05: 48 89 84 24 80 00 00 00     	movq	%rax, 0x80(%rsp)
1400aef0d: 48 c7 84 24 88 00 00 00 25 00 00 00 	movq	$0x25, 0x88(%rsp)
1400aef19: e9 5b 02 00 00              	jmp	0x1400af179 <.text+0xae179>
1400aef1e: 48 81 c6 98 00 00 00        	addq	$0x98, %rsi
1400aef25: 48 8d 05 71 46 88 02        	leaq	0x2884671(%rip), %rax   # 0x14293359d
1400aef2c: 48 89 84 24 80 00 00 00     	movq	%rax, 0x80(%rsp)
1400aef34: 48 c7 84 24 88 00 00 00 27 00 00 00 	movq	$0x27, 0x88(%rsp)
1400aef40: e9 34 02 00 00              	jmp	0x1400af179 <.text+0xae179>
1400aef45: 48 81 c6 98 00 00 00        	addq	$0x98, %rsi
1400aef4c: 48 8d 05 e9 49 88 02        	leaq	0x28849e9(%rip), %rax   # 0x14293393c
1400aef53: 48 89 84 24 80 00 00 00     	movq	%rax, 0x80(%rsp)
1400aef5b: 48 c7 84 24 88 00 00 00 2e 00 00 00 	movq	$0x2e, 0x88(%rsp)
1400aef67: e9 3a 01 00 00              	jmp	0x1400af0a6 <.text+0xae0a6>
1400aef6c: 48 81 c6 98 00 00 00        	addq	$0x98, %rsi
1400aef73: 48 8d 05 82 47 88 02        	leaq	0x2884782(%rip), %rax   # 0x1429336fc
1400aef7a: 48 89 84 24 80 00 00 00     	movq	%rax, 0x80(%rsp)
1400aef82: 48 c7 84 24 88 00 00 00 1e 00 00 00 	movq	$0x1e, 0x88(%rsp)
1400aef8e: e9 13 01 00 00              	jmp	0x1400af0a6 <.text+0xae0a6>
1400aef93: 48 81 c6 98 00 00 00        	addq	$0x98, %rsi
1400aef9a: 48 8d 05 79 47 88 02        	leaq	0x2884779(%rip), %rax   # 0x14293371a
1400aefa1: 48 89 84 24 80 00 00 00     	movq	%rax, 0x80(%rsp)
1400aefa9: 48 c7 84 24 88 00 00 00 61 00 00 00 	movq	$0x61, 0x88(%rsp)
1400aefb5: e9 ec 00 00 00              	jmp	0x1400af0a6 <.text+0xae0a6>
1400aefba: 48 81 c6 98 00 00 00        	addq	$0x98, %rsi
1400aefc1: 48 8d 05 ca 48 88 02        	leaq	0x28848ca(%rip), %rax   # 0x142933892
1400aefc8: 48 89 84 24 80 00 00 00     	movq	%rax, 0x80(%rsp)
1400aefd0: 48 c7 84 24 88 00 00 00 58 00 00 00 	movq	$0x58, 0x88(%rsp)
1400aefdc: e9 c5 00 00 00              	jmp	0x1400af0a6 <.text+0xae0a6>
1400aefe1: 48 81 c6 98 00 00 00        	addq	$0x98, %rsi
1400aefe8: 48 8d 05 8c 47 88 02        	leaq	0x288478c(%rip), %rax   # 0x14293377b
1400aefef: 48 89 84 24 80 00 00 00     	movq	%rax, 0x80(%rsp)
1400aeff7: 48 c7 84 24 88 00 00 00 59 00 00 00 	movq	$0x59, 0x88(%rsp)
1400af003: e9 9e 00 00 00              	jmp	0x1400af0a6 <.text+0xae0a6>
1400af008: 48 81 c6 98 00 00 00        	addq	$0x98, %rsi
1400af00f: 48 8d 05 be 47 88 02        	leaq	0x28847be(%rip), %rax   # 0x1429337d4
1400af016: eb 56                       	jmp	0x1400af06e <.text+0xae06e>
1400af018: 48 81 c6 98 00 00 00        	addq	$0x98, %rsi
1400af01f: 48 8d 05 c4 48 88 02        	leaq	0x28848c4(%rip), %rax   # 0x1429338ea
1400af026: 48 89 84 24 80 00 00 00     	movq	%rax, 0x80(%rsp)
1400af02e: 48 c7 84 24 88 00 00 00 52 00 00 00 	movq	$0x52, 0x88(%rsp)
1400af03a: eb 6a                       	jmp	0x1400af0a6 <.text+0xae0a6>
1400af03c: 48 81 c6 98 00 00 00        	addq	$0x98, %rsi
1400af043: 48 8d 05 5e 46 88 02        	leaq	0x288465e(%rip), %rax   # 0x1429336a8
1400af04a: 48 89 84 24 80 00 00 00     	movq	%rax, 0x80(%rsp)
1400af052: 48 c7 84 24 88 00 00 00 54 00 00 00 	movq	$0x54, 0x88(%rsp)
1400af05e: eb 46                       	jmp	0x1400af0a6 <.text+0xae0a6>
1400af060: 48 81 c6 98 00 00 00        	addq	$0x98, %rsi
1400af067: 48 8d 05 c5 47 88 02        	leaq	0x28847c5(%rip), %rax   # 0x142933833
1400af06e: 48 89 84 24 80 00 00 00     	movq	%rax, 0x80(%rsp)
1400af076: 48 c7 84 24 88 00 00 00 5f 00 00 00 	movq	$0x5f, 0x88(%rsp)
1400af082: eb 22                       	jmp	0x1400af0a6 <.text+0xae0a6>
1400af084: 48 81 c6 98 00 00 00        	addq	$0x98, %rsi
1400af08b: 48 8d 05 be 45 88 02        	leaq	0x28845be(%rip), %rax   # 0x142933650
1400af092: 48 89 84 24 80 00 00 00     	movq	%rax, 0x80(%rsp)
1400af09a: 48 c7 84 24 88 00 00 00 39 00 00 00 	movq	$0x39, 0x88(%rsp)
1400af0a6: 48 89 b4 24 90 00 00 00     	movq	%rsi, 0x90(%rsp)
1400af0ae: 48 8d 05 1b ca f8 ff        	leaq	-0x735e5(%rip), %rax    # 0x14003bad0 <.text+0x3aad0>
1400af0b5: 48 89 84 24 98 00 00 00     	movq	%rax, 0x98(%rsp)
1400af0bd: 48 8d 84 24 80 00 00 00     	leaq	0x80(%rsp), %rax
1400af0c5: 48 89 84 24 a0 00 00 00     	movq	%rax, 0xa0(%rsp)
1400af0cd: 48 8d 05 bc ad 05 00        	leaq	0x5adbc(%rip), %rax     # 0x140109e90 <.text+0x108e90>
1400af0d4: 48 89 84 24 a8 00 00 00     	movq	%rax, 0xa8(%rsp)
1400af0dc: 48 8d 05 3d 43 88 02        	leaq	0x288433d(%rip), %rax   # 0x142933420
1400af0e3: 48 89 84 24 b0 00 00 00     	movq	%rax, 0xb0(%rsp)
1400af0eb: 48 c7 84 24 b8 00 00 00 02 00 00 00 	movq	$0x2, 0xb8(%rsp)
1400af0f7: 48 c7 84 24 d0 00 00 00 00 00 00 00 	movq	$0x0, 0xd0(%rsp)
1400af103: 48 8d 84 24 90 00 00 00     	leaq	0x90(%rsp), %rax
1400af10b: 48 89 84 24 c0 00 00 00     	movq	%rax, 0xc0(%rsp)
1400af113: 48 c7 84 24 c8 00 00 00 02 00 00 00 	movq	$0x2, 0xc8(%rsp)
1400af11f: 48 8d 15 6a 45 88 02        	leaq	0x288456a(%rip), %rdx   # 0x142933690
1400af126: 48 8d 8c 24 b0 00 00 00     	leaq	0xb0(%rsp), %rcx
1400af12e: e8 5d a2 82 02              	callq	0x1428d9390 <.text+0x28d8390>
1400af133: 48 81 c6 98 00 00 00        	addq	$0x98, %rsi
1400af13a: 48 8d 05 d4 43 88 02        	leaq	0x28843d4(%rip), %rax   # 0x142933515
1400af141: 48 89 84 24 80 00 00 00     	movq	%rax, 0x80(%rsp)
1400af149: 48 c7 84 24 88 00 00 00 37 00 00 00 	movq	$0x37, 0x88(%rsp)
1400af155: eb 22                       	jmp	0x1400af179 <.text+0xae179>
1400af157: 48 81 c6 98 00 00 00        	addq	$0x98, %rsi
1400af15e: 48 8d 05 b9 44 88 02        	leaq	0x28844b9(%rip), %rax   # 0x14293361e
1400af165: 48 89 84 24 80 00 00 00     	movq	%rax, 0x80(%rsp)
1400af16d: 48 c7 84 24 88 00 00 00 32 00 00 00 	movq	$0x32, 0x88(%rsp)
1400af179: 48 89 b4 24 90 00 00 00     	movq	%rsi, 0x90(%rsp)
1400af181: 48 8d 05 48 c9 f8 ff        	leaq	-0x736b8(%rip), %rax    # 0x14003bad0 <.text+0x3aad0>
1400af188: 48 89 84 24 98 00 00 00     	movq	%rax, 0x98(%rsp)
1400af190: 48 8d 84 24 80 00 00 00     	leaq	0x80(%rsp), %rax
1400af198: 48 89 84 24 a0 00 00 00     	movq	%rax, 0xa0(%rsp)
1400af1a0: 48 8d 05 e9 ac 05 00        	leaq	0x5ace9(%rip), %rax     # 0x140109e90 <.text+0x108e90>
1400af1a7: 48 89 84 24 a8 00 00 00     	movq	%rax, 0xa8(%rsp)
1400af1af: 48 8d 05 6a 42 88 02        	leaq	0x288426a(%rip), %rax   # 0x142933420
1400af1b6: 48 89 84 24 b0 00 00 00     	movq	%rax, 0xb0(%rsp)
1400af1be: 48 c7 84 24 b8 00 00 00 02 00 00 00 	movq	$0x2, 0xb8(%rsp)
1400af1ca: 48 c7 84 24 d0 00 00 00 00 00 00 00 	movq	$0x0, 0xd0(%rsp)
1400af1d6: 48 8d 84 24 90 00 00 00     	leaq	0x90(%rsp), %rax
1400af1de: 48 89 84 24 c0 00 00 00     	movq	%rax, 0xc0(%rsp)
1400af1e6: 48 c7 84 24 c8 00 00 00 02 00 00 00 	movq	$0x2, 0xc8(%rsp)
1400af1f2: 48 8d 15 af 42 88 02        	leaq	0x28842af(%rip), %rdx   # 0x1429334a8
1400af1f9: 48 8d 8c 24 b0 00 00 00     	leaq	0xb0(%rsp), %rcx
1400af201: e8 8a a1 82 02              	callq	0x1428d9390 <.text+0x28d8390>
1400af206: 48 8d 0d 3b 48 88 02        	leaq	0x288483b(%rip), %rcx   # 0x142933a48
1400af20d: 4c 8d 05 94 48 88 02        	leaq	0x2884894(%rip), %r8    # 0x142933aa8
1400af214: ba 59 00 00 00              	movl	$0x59, %edx
1400af219: e8 12 a1 82 02              	callq	0x1428d9330 <.text+0x28d8330>
1400af21e: cc                          	int3
