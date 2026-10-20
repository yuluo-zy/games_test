
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14201a0d0: 41 57                       	pushq	%r15
14201a0d2: 41 56                       	pushq	%r14
14201a0d4: 41 55                       	pushq	%r13
14201a0d6: 41 54                       	pushq	%r12
14201a0d8: 56                          	pushq	%rsi
14201a0d9: 57                          	pushq	%rdi
14201a0da: 55                          	pushq	%rbp
14201a0db: 53                          	pushq	%rbx
14201a0dc: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
14201a0e3: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
14201a0ec: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
14201a0f4: 0f 29 b4 24 80 00 00 00     	movaps	%xmm6, 0x80(%rsp)
14201a0fc: 4c 89 ce                    	movq	%r9, %rsi
14201a0ff: 4c 89 c7                    	movq	%r8, %rdi
14201a102: 4c 8b b4 24 58 01 00 00     	movq	0x158(%rsp), %r14
14201a10a: 48 8b 94 24 20 01 00 00     	movq	0x120(%rsp), %rdx
14201a112: 41 0f 28 46 10              	movaps	0x10(%r14), %xmm0
14201a117: 41 0f 28 4e 20              	movaps	0x20(%r14), %xmm1
14201a11c: 41 0f 28 56 30              	movaps	0x30(%r14), %xmm2
14201a121: 0f 29 54 24 60              	movaps	%xmm2, 0x60(%rsp)
14201a126: 0f 29 4c 24 50              	movaps	%xmm1, 0x50(%rsp)
14201a12b: 0f 29 44 24 40              	movaps	%xmm0, 0x40(%rsp)
14201a130: 48 89 4c 24 38              	movq	%rcx, 0x38(%rsp)
14201a135: 48 8b 01                    	movq	(%rcx), %rax
14201a138: 0f b6 0a                    	movzbl	(%rdx), %ecx
14201a13b: 4d 8b 68 40                 	movq	0x40(%r8), %r13
14201a13f: 49 39 c5                    	cmpq	%rax, %r13
14201a142: 76 1e                       	jbe	0x14201a162 <.text+0x2019162>
14201a144: 48 8b 57 38                 	movq	0x38(%rdi), %rdx
14201a148: 48 8b 14 c2                 	movq	(%rdx,%rax,8), %rdx
14201a14c: 48 85 d2                    	testq	%rdx, %rdx
14201a14f: 74 11                       	je	0x14201a162 <.text+0x2019162>
14201a151: 48 f7 d2                    	notq	%rdx
14201a154: 4c 8d 3c 52                 	leaq	(%rdx,%rdx,2), %r15
14201a158: 49 c1 e7 04                 	shlq	$0x4, %r15
14201a15c: 4c 03 7f 18                 	addq	0x18(%rdi), %r15
14201a160: eb 03                       	jmp	0x14201a165 <.text+0x2019165>
14201a162: 45 31 ff                    	xorl	%r15d, %r15d
14201a165: 8b 9c 24 50 01 00 00        	movl	0x150(%rsp), %ebx
14201a16c: 44 8b a4 24 48 01 00 00     	movl	0x148(%rsp), %r12d
14201a174: f3 45 0f 10 46 58           	movss	0x58(%r14), %xmm8
14201a17a: 41 8b 6e 50                 	movl	0x50(%r14), %ebp
14201a17e: f3 41 0f 10 7e 54           	movss	0x54(%r14), %xmm7
14201a184: f3 41 0f 10 76 68           	movss	0x68(%r14), %xmm6
14201a18a: 84 c9                       	testb	%cl, %cl
14201a18c: 74 42                       	je	0x14201a1d0 <.text+0x20191d0>
14201a18e: 80 bc 24 60 01 00 00 00     	cmpb	$0x0, 0x160(%rsp)
14201a196: 74 63                       	je	0x14201a1fb <.text+0x20191fb>
14201a198: 49 39 c5                    	cmpq	%rax, %r13
14201a19b: 0f 86 82 00 00 00           	jbe	0x14201a223 <.text+0x2019223>
14201a1a1: 48 8b 4f 38                 	movq	0x38(%rdi), %rcx
14201a1a5: 48 8b 04 c1                 	movq	(%rcx,%rax,8), %rax
14201a1a9: 48 85 c0                    	testq	%rax, %rax
14201a1ac: 74 75                       	je	0x14201a223 <.text+0x2019223>
14201a1ae: 48 8b 4f 18                 	movq	0x18(%rdi), %rcx
14201a1b2: 48 f7 d0                    	notq	%rax
14201a1b5: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14201a1b9: 48 c1 e0 04                 	shlq	$0x4, %rax
14201a1bd: 48 8b 44 01 18              	movq	0x18(%rcx,%rax), %rax
14201a1c2: 48 85 c0                    	testq	%rax, %rax
14201a1c5: 74 5c                       	je	0x14201a223 <.text+0x2019223>
14201a1c7: 48 8d 4c 24 40              	leaq	0x40(%rsp), %rcx
14201a1cc: ff d0                       	callq	*%rax
14201a1ce: eb 53                       	jmp	0x14201a223 <.text+0x2019223>
14201a1d0: 44 8b a4 24 48 01 00 00     	movl	0x148(%rsp), %r12d
14201a1d8: 4d 8b 47 08                 	movq	0x8(%r15), %r8
14201a1dc: 4c 89 c1                    	movq	%r8, %rcx
14201a1df: 49 0f af cc                 	imulq	%r12, %rcx
14201a1e3: 49 03 4f 10                 	addq	0x10(%r15), %rcx
14201a1e7: 48 8d 54 24 40              	leaq	0x40(%rsp), %rdx
14201a1ec: e8 1f 9a 90 00              	callq	0x142923c10 <.text+0x2922c10>
14201a1f1: 49 8b 47 20                 	movq	0x20(%r15), %rax
14201a1f5: 42 89 1c a0                 	movl	%ebx, (%rax,%r12,4)
14201a1f9: eb 18                       	jmp	0x14201a213 <.text+0x2019213>
14201a1fb: 44 8b a4 24 48 01 00 00     	movl	0x148(%rsp), %r12d
14201a203: 4c 8d 44 24 40              	leaq	0x40(%rsp), %r8
14201a208: 4c 89 f9                    	movq	%r15, %rcx
14201a20b: 4c 89 e2                    	movq	%r12, %rdx
14201a20e: e8 ed 51 ce fe              	callq	0x140cff400 <.text+0xcfe400>
14201a213: 49 8b 47 28                 	movq	0x28(%r15), %rax
14201a217: 42 89 1c a0                 	movl	%ebx, (%rax,%r12,4)
14201a21b: 44 8b a4 24 48 01 00 00     	movl	0x148(%rsp), %r12d
14201a223: f3 44 0f 11 44 24 40        	movss	%xmm8, 0x40(%rsp)
14201a22a: 48 8b 44 24 38              	movq	0x38(%rsp), %rax
14201a22f: 48 8b 40 08                 	movq	0x8(%rax), %rax
14201a233: 48 8b 8c 24 20 01 00 00     	movq	0x120(%rsp), %rcx
14201a23b: 0f b6 49 01                 	movzbl	0x1(%rcx), %ecx
14201a23f: 49 39 c5                    	cmpq	%rax, %r13
14201a242: 76 4b                       	jbe	0x14201a28f <.text+0x201928f>
14201a244: 48 8b 57 38                 	movq	0x38(%rdi), %rdx
14201a248: 48 8b 14 c2                 	movq	(%rdx,%rax,8), %rdx
14201a24c: 48 85 d2                    	testq	%rdx, %rdx
14201a24f: 74 3e                       	je	0x14201a28f <.text+0x201928f>
14201a251: 48 f7 d2                    	notq	%rdx
14201a254: 4c 8d 3c 52                 	leaq	(%rdx,%rdx,2), %r15
14201a258: 49 c1 e7 04                 	shlq	$0x4, %r15
14201a25c: 4c 03 7f 18                 	addq	0x18(%rdi), %r15
14201a260: 84 c9                       	testb	%cl, %cl
14201a262: 75 32                       	jne	0x14201a296 <.text+0x2019296>
14201a264: 44 8b a4 24 48 01 00 00     	movl	0x148(%rsp), %r12d
14201a26c: 4d 8b 47 08                 	movq	0x8(%r15), %r8
14201a270: 4c 89 c1                    	movq	%r8, %rcx
14201a273: 49 0f af cc                 	imulq	%r12, %rcx
14201a277: 49 03 4f 10                 	addq	0x10(%r15), %rcx
14201a27b: 48 8d 54 24 40              	leaq	0x40(%rsp), %rdx
14201a280: e8 8b 99 90 00              	callq	0x142923c10 <.text+0x2922c10>
14201a285: 49 8b 47 20                 	movq	0x20(%r15), %rax
14201a289: 42 89 1c a0                 	movl	%ebx, (%rax,%r12,4)
14201a28d: eb 5d                       	jmp	0x14201a2ec <.text+0x20192ec>
14201a28f: 45 31 ff                    	xorl	%r15d, %r15d
14201a292: 84 c9                       	testb	%cl, %cl
14201a294: 74 ce                       	je	0x14201a264 <.text+0x2019264>
14201a296: 80 bc 24 60 01 00 00 00     	cmpb	$0x0, 0x160(%rsp)
14201a29e: 74 34                       	je	0x14201a2d4 <.text+0x20192d4>
14201a2a0: 49 39 c5                    	cmpq	%rax, %r13
14201a2a3: 76 57                       	jbe	0x14201a2fc <.text+0x20192fc>
14201a2a5: 48 8b 4f 38                 	movq	0x38(%rdi), %rcx
14201a2a9: 48 8b 04 c1                 	movq	(%rcx,%rax,8), %rax
14201a2ad: 48 85 c0                    	testq	%rax, %rax
14201a2b0: 74 4a                       	je	0x14201a2fc <.text+0x20192fc>
14201a2b2: 48 8b 4f 18                 	movq	0x18(%rdi), %rcx
14201a2b6: 48 f7 d0                    	notq	%rax
14201a2b9: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14201a2bd: 48 c1 e0 04                 	shlq	$0x4, %rax
14201a2c1: 48 8b 44 01 18              	movq	0x18(%rcx,%rax), %rax
14201a2c6: 48 85 c0                    	testq	%rax, %rax
14201a2c9: 74 31                       	je	0x14201a2fc <.text+0x20192fc>
14201a2cb: 48 8d 4c 24 40              	leaq	0x40(%rsp), %rcx
14201a2d0: ff d0                       	callq	*%rax
14201a2d2: eb 28                       	jmp	0x14201a2fc <.text+0x20192fc>
14201a2d4: 44 8b a4 24 48 01 00 00     	movl	0x148(%rsp), %r12d
14201a2dc: 4c 8d 44 24 40              	leaq	0x40(%rsp), %r8
14201a2e1: 4c 89 f9                    	movq	%r15, %rcx
14201a2e4: 4c 89 e2                    	movq	%r12, %rdx
14201a2e7: e8 14 51 ce fe              	callq	0x140cff400 <.text+0xcfe400>
14201a2ec: 49 8b 47 28                 	movq	0x28(%r15), %rax
14201a2f0: 42 89 1c a0                 	movl	%ebx, (%rax,%r12,4)
14201a2f4: 44 8b a4 24 48 01 00 00     	movl	0x148(%rsp), %r12d
14201a2fc: 48 8b 44 24 38              	movq	0x38(%rsp), %rax
14201a301: 48 8b 40 10                 	movq	0x10(%rax), %rax
14201a305: 48 8b 8c 24 20 01 00 00     	movq	0x120(%rsp), %rcx
14201a30d: 0f b6 49 02                 	movzbl	0x2(%rcx), %ecx
14201a311: 49 39 c5                    	cmpq	%rax, %r13
14201a314: 76 4b                       	jbe	0x14201a361 <.text+0x2019361>
14201a316: 48 8b 57 38                 	movq	0x38(%rdi), %rdx
14201a31a: 48 8b 14 c2                 	movq	(%rdx,%rax,8), %rdx
14201a31e: 48 85 d2                    	testq	%rdx, %rdx
14201a321: 74 3e                       	je	0x14201a361 <.text+0x2019361>
14201a323: 48 f7 d2                    	notq	%rdx
14201a326: 4c 8d 3c 52                 	leaq	(%rdx,%rdx,2), %r15
14201a32a: 49 c1 e7 04                 	shlq	$0x4, %r15
14201a32e: 4c 03 7f 18                 	addq	0x18(%rdi), %r15
14201a332: 84 c9                       	testb	%cl, %cl
14201a334: 75 32                       	jne	0x14201a368 <.text+0x2019368>
14201a336: 44 8b a4 24 48 01 00 00     	movl	0x148(%rsp), %r12d
14201a33e: 4d 8b 47 08                 	movq	0x8(%r15), %r8
14201a342: 4c 89 c1                    	movq	%r8, %rcx
14201a345: 49 0f af cc                 	imulq	%r12, %rcx
14201a349: 49 03 4f 10                 	addq	0x10(%r15), %rcx
14201a34d: 48 8d 54 24 40              	leaq	0x40(%rsp), %rdx
14201a352: e8 b9 98 90 00              	callq	0x142923c10 <.text+0x2922c10>
14201a357: 49 8b 47 20                 	movq	0x20(%r15), %rax
14201a35b: 42 89 1c a0                 	movl	%ebx, (%rax,%r12,4)
14201a35f: eb 5d                       	jmp	0x14201a3be <.text+0x20193be>
14201a361: 45 31 ff                    	xorl	%r15d, %r15d
14201a364: 84 c9                       	testb	%cl, %cl
14201a366: 74 ce                       	je	0x14201a336 <.text+0x2019336>
14201a368: 80 bc 24 60 01 00 00 00     	cmpb	$0x0, 0x160(%rsp)
14201a370: 74 34                       	je	0x14201a3a6 <.text+0x20193a6>
14201a372: 49 39 c5                    	cmpq	%rax, %r13
14201a375: 76 57                       	jbe	0x14201a3ce <.text+0x20193ce>
14201a377: 48 8b 4f 38                 	movq	0x38(%rdi), %rcx
14201a37b: 48 8b 04 c1                 	movq	(%rcx,%rax,8), %rax
14201a37f: 48 85 c0                    	testq	%rax, %rax
14201a382: 74 4a                       	je	0x14201a3ce <.text+0x20193ce>
14201a384: 48 8b 4f 18                 	movq	0x18(%rdi), %rcx
14201a388: 48 f7 d0                    	notq	%rax
14201a38b: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14201a38f: 48 c1 e0 04                 	shlq	$0x4, %rax
14201a393: 48 8b 44 01 18              	movq	0x18(%rcx,%rax), %rax
14201a398: 48 85 c0                    	testq	%rax, %rax
14201a39b: 74 31                       	je	0x14201a3ce <.text+0x20193ce>
14201a39d: 48 8d 4c 24 40              	leaq	0x40(%rsp), %rcx
14201a3a2: ff d0                       	callq	*%rax
14201a3a4: eb 28                       	jmp	0x14201a3ce <.text+0x20193ce>
14201a3a6: 44 8b a4 24 48 01 00 00     	movl	0x148(%rsp), %r12d
14201a3ae: 4c 8d 44 24 40              	leaq	0x40(%rsp), %r8
14201a3b3: 4c 89 f9                    	movq	%r15, %rcx
14201a3b6: 4c 89 e2                    	movq	%r12, %rdx
14201a3b9: e8 42 50 ce fe              	callq	0x140cff400 <.text+0xcfe400>
14201a3be: 49 8b 47 28                 	movq	0x28(%r15), %rax
14201a3c2: 42 89 1c a0                 	movl	%ebx, (%rax,%r12,4)
14201a3c6: 44 8b a4 24 48 01 00 00     	movl	0x148(%rsp), %r12d
14201a3ce: 89 6c 24 40                 	movl	%ebp, 0x40(%rsp)
14201a3d2: f3 0f 11 7c 24 44           	movss	%xmm7, 0x44(%rsp)
14201a3d8: 48 8b 44 24 38              	movq	0x38(%rsp), %rax
14201a3dd: 48 8b 40 18                 	movq	0x18(%rax), %rax
14201a3e1: 48 8b 8c 24 20 01 00 00     	movq	0x120(%rsp), %rcx
14201a3e9: 0f b6 49 03                 	movzbl	0x3(%rcx), %ecx
14201a3ed: 49 39 c5                    	cmpq	%rax, %r13
14201a3f0: 76 46                       	jbe	0x14201a438 <.text+0x2019438>
14201a3f2: 48 8b 57 38                 	movq	0x38(%rdi), %rdx
14201a3f6: 48 8b 14 c2                 	movq	(%rdx,%rax,8), %rdx
14201a3fa: 48 85 d2                    	testq	%rdx, %rdx
14201a3fd: 74 39                       	je	0x14201a438 <.text+0x2019438>
14201a3ff: 48 f7 d2                    	notq	%rdx
14201a402: 4c 8d 3c 52                 	leaq	(%rdx,%rdx,2), %r15
14201a406: 49 c1 e7 04                 	shlq	$0x4, %r15
14201a40a: 4c 03 7f 18                 	addq	0x18(%rdi), %r15
14201a40e: 84 c9                       	testb	%cl, %cl
14201a410: 75 2d                       	jne	0x14201a43f <.text+0x201943f>
14201a412: 45 89 e4                    	movl	%r12d, %r12d
14201a415: 4d 8b 47 08                 	movq	0x8(%r15), %r8
14201a419: 4c 89 c1                    	movq	%r8, %rcx
14201a41c: 49 0f af cc                 	imulq	%r12, %rcx
14201a420: 49 03 4f 10                 	addq	0x10(%r15), %rcx
14201a424: 48 8d 54 24 40              	leaq	0x40(%rsp), %rdx
14201a429: e8 e2 97 90 00              	callq	0x142923c10 <.text+0x2922c10>
14201a42e: 49 8b 47 20                 	movq	0x20(%r15), %rax
14201a432: 42 89 1c a0                 	movl	%ebx, (%rax,%r12,4)
14201a436: eb 58                       	jmp	0x14201a490 <.text+0x2019490>
14201a438: 45 31 ff                    	xorl	%r15d, %r15d
14201a43b: 84 c9                       	testb	%cl, %cl
14201a43d: 74 d3                       	je	0x14201a412 <.text+0x2019412>
14201a43f: 80 bc 24 60 01 00 00 00     	cmpb	$0x0, 0x160(%rsp)
14201a447: 74 34                       	je	0x14201a47d <.text+0x201947d>
14201a449: 49 39 c5                    	cmpq	%rax, %r13
14201a44c: 76 4a                       	jbe	0x14201a498 <.text+0x2019498>
14201a44e: 48 8b 4f 38                 	movq	0x38(%rdi), %rcx
14201a452: 48 8b 04 c1                 	movq	(%rcx,%rax,8), %rax
14201a456: 48 85 c0                    	testq	%rax, %rax
14201a459: 74 3d                       	je	0x14201a498 <.text+0x2019498>
14201a45b: 48 8b 4f 18                 	movq	0x18(%rdi), %rcx
14201a45f: 48 f7 d0                    	notq	%rax
14201a462: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14201a466: 48 c1 e0 04                 	shlq	$0x4, %rax
14201a46a: 48 8b 44 01 18              	movq	0x18(%rcx,%rax), %rax
14201a46f: 48 85 c0                    	testq	%rax, %rax
14201a472: 74 24                       	je	0x14201a498 <.text+0x2019498>
14201a474: 48 8d 4c 24 40              	leaq	0x40(%rsp), %rcx
14201a479: ff d0                       	callq	*%rax
14201a47b: eb 1b                       	jmp	0x14201a498 <.text+0x2019498>
14201a47d: 45 89 e4                    	movl	%r12d, %r12d
14201a480: 4c 8d 44 24 40              	leaq	0x40(%rsp), %r8
14201a485: 4c 89 f9                    	movq	%r15, %rcx
14201a488: 4c 89 e2                    	movq	%r12, %rdx
14201a48b: e8 70 4f ce fe              	callq	0x140cff400 <.text+0xcfe400>
14201a490: 49 8b 47 28                 	movq	0x28(%r15), %rax
14201a494: 42 89 1c a0                 	movl	%ebx, (%rax,%r12,4)
14201a498: 41 0f 10 06                 	movups	(%r14), %xmm0
14201a49c: 0f 29 44 24 40              	movaps	%xmm0, 0x40(%rsp)
14201a4a1: 48 8b 44 24 38              	movq	0x38(%rsp), %rax
14201a4a6: 48 8b 40 20                 	movq	0x20(%rax), %rax
14201a4aa: 48 8b 8c 24 20 01 00 00     	movq	0x120(%rsp), %rcx
14201a4b2: 0f b6 49 04                 	movzbl	0x4(%rcx), %ecx
14201a4b6: 49 39 c5                    	cmpq	%rax, %r13
14201a4b9: 4c 89 6c 24 78              	movq	%r13, 0x78(%rsp)
14201a4be: 8b ac 24 48 01 00 00        	movl	0x148(%rsp), %ebp
14201a4c5: 76 4a                       	jbe	0x14201a511 <.text+0x2019511>
14201a4c7: 48 8b 57 38                 	movq	0x38(%rdi), %rdx
14201a4cb: 48 8b 14 c2                 	movq	(%rdx,%rax,8), %rdx
14201a4cf: 48 85 d2                    	testq	%rdx, %rdx
14201a4d2: 74 3d                       	je	0x14201a511 <.text+0x2019511>
14201a4d4: 48 f7 d2                    	notq	%rdx
14201a4d7: 4c 8d 3c 52                 	leaq	(%rdx,%rdx,2), %r15
14201a4db: 49 c1 e7 04                 	shlq	$0x4, %r15
14201a4df: 4c 03 7f 18                 	addq	0x18(%rdi), %r15
14201a4e3: 4d 8d 6e 40                 	leaq	0x40(%r14), %r13
14201a4e7: 84 c9                       	testb	%cl, %cl
14201a4e9: 75 31                       	jne	0x14201a51c <.text+0x201951c>
14201a4eb: 41 89 ec                    	movl	%ebp, %r12d
14201a4ee: 4d 8b 47 08                 	movq	0x8(%r15), %r8
14201a4f2: 4c 89 c1                    	movq	%r8, %rcx
14201a4f5: 49 0f af cc                 	imulq	%r12, %rcx
14201a4f9: 49 03 4f 10                 	addq	0x10(%r15), %rcx
14201a4fd: 48 8d 54 24 40              	leaq	0x40(%rsp), %rdx
14201a502: e8 09 97 90 00              	callq	0x142923c10 <.text+0x2922c10>
14201a507: 49 8b 47 20                 	movq	0x20(%r15), %rax
14201a50b: 42 89 1c a0                 	movl	%ebx, (%rax,%r12,4)
14201a50f: eb 5e                       	jmp	0x14201a56f <.text+0x201956f>
14201a511: 45 31 ff                    	xorl	%r15d, %r15d
14201a514: 4d 8d 6e 40                 	leaq	0x40(%r14), %r13
14201a518: 84 c9                       	testb	%cl, %cl
14201a51a: 74 cf                       	je	0x14201a4eb <.text+0x20194eb>
14201a51c: 80 bc 24 60 01 00 00 00     	cmpb	$0x0, 0x160(%rsp)
14201a524: 74 36                       	je	0x14201a55c <.text+0x201955c>
14201a526: 48 39 44 24 78              	cmpq	%rax, 0x78(%rsp)
14201a52b: 76 4a                       	jbe	0x14201a577 <.text+0x2019577>
14201a52d: 48 8b 4f 38                 	movq	0x38(%rdi), %rcx
14201a531: 48 8b 04 c1                 	movq	(%rcx,%rax,8), %rax
14201a535: 48 85 c0                    	testq	%rax, %rax
14201a538: 74 3d                       	je	0x14201a577 <.text+0x2019577>
14201a53a: 48 8b 4f 18                 	movq	0x18(%rdi), %rcx
14201a53e: 48 f7 d0                    	notq	%rax
14201a541: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14201a545: 48 c1 e0 04                 	shlq	$0x4, %rax
14201a549: 48 8b 44 01 18              	movq	0x18(%rcx,%rax), %rax
14201a54e: 48 85 c0                    	testq	%rax, %rax
14201a551: 74 24                       	je	0x14201a577 <.text+0x2019577>
14201a553: 48 8d 4c 24 40              	leaq	0x40(%rsp), %rcx
14201a558: ff d0                       	callq	*%rax
14201a55a: eb 1b                       	jmp	0x14201a577 <.text+0x2019577>
14201a55c: 41 89 ec                    	movl	%ebp, %r12d
14201a55f: 4c 8d 44 24 40              	leaq	0x40(%rsp), %r8
14201a564: 4c 89 f9                    	movq	%r15, %rcx
14201a567: 4c 89 e2                    	movq	%r12, %rdx
14201a56a: e8 91 4e ce fe              	callq	0x140cff400 <.text+0xcfe400>
14201a56f: 49 8b 47 28                 	movq	0x28(%r15), %rax
14201a573: 42 89 1c a0                 	movl	%ebx, (%rax,%r12,4)
14201a577: 41 0f 10 45 00              	movups	(%r13), %xmm0
14201a57c: 0f 29 44 24 40              	movaps	%xmm0, 0x40(%rsp)
14201a581: 48 8b 44 24 38              	movq	0x38(%rsp), %rax
14201a586: 48 8b 40 28                 	movq	0x28(%rax), %rax
14201a58a: 48 8b 8c 24 20 01 00 00     	movq	0x120(%rsp), %rcx
14201a592: 0f b6 49 05                 	movzbl	0x5(%rcx), %ecx
14201a596: 4c 8b 6c 24 78              	movq	0x78(%rsp), %r13
14201a59b: 49 39 c5                    	cmpq	%rax, %r13
14201a59e: 76 4a                       	jbe	0x14201a5ea <.text+0x20195ea>
14201a5a0: 48 8b 57 38                 	movq	0x38(%rdi), %rdx
14201a5a4: 48 8b 14 c2                 	movq	(%rdx,%rax,8), %rdx
14201a5a8: 48 85 d2                    	testq	%rdx, %rdx
14201a5ab: 74 3d                       	je	0x14201a5ea <.text+0x20195ea>
14201a5ad: 48 f7 d2                    	notq	%rdx
14201a5b0: 4c 8d 3c 52                 	leaq	(%rdx,%rdx,2), %r15
14201a5b4: 49 c1 e7 04                 	shlq	$0x4, %r15
14201a5b8: 4c 03 7f 18                 	addq	0x18(%rdi), %r15
14201a5bc: 49 83 c6 5c                 	addq	$0x5c, %r14
14201a5c0: 84 c9                       	testb	%cl, %cl
14201a5c2: 75 31                       	jne	0x14201a5f5 <.text+0x20195f5>
14201a5c4: 41 89 ec                    	movl	%ebp, %r12d
14201a5c7: 4d 8b 47 08                 	movq	0x8(%r15), %r8
14201a5cb: 4c 89 c1                    	movq	%r8, %rcx
14201a5ce: 49 0f af cc                 	imulq	%r12, %rcx
14201a5d2: 49 03 4f 10                 	addq	0x10(%r15), %rcx
14201a5d6: 48 8d 54 24 40              	leaq	0x40(%rsp), %rdx
14201a5db: e8 30 96 90 00              	callq	0x142923c10 <.text+0x2922c10>
14201a5e0: 49 8b 47 20                 	movq	0x20(%r15), %rax
14201a5e4: 42 89 1c a0                 	movl	%ebx, (%rax,%r12,4)
14201a5e8: eb 5c                       	jmp	0x14201a646 <.text+0x2019646>
14201a5ea: 45 31 ff                    	xorl	%r15d, %r15d
14201a5ed: 49 83 c6 5c                 	addq	$0x5c, %r14
14201a5f1: 84 c9                       	testb	%cl, %cl
14201a5f3: 74 cf                       	je	0x14201a5c4 <.text+0x20195c4>
14201a5f5: 80 bc 24 60 01 00 00 00     	cmpb	$0x0, 0x160(%rsp)
14201a5fd: 74 34                       	je	0x14201a633 <.text+0x2019633>
14201a5ff: 49 39 c5                    	cmpq	%rax, %r13
14201a602: 76 4a                       	jbe	0x14201a64e <.text+0x201964e>
14201a604: 48 8b 4f 38                 	movq	0x38(%rdi), %rcx
14201a608: 48 8b 04 c1                 	movq	(%rcx,%rax,8), %rax
14201a60c: 48 85 c0                    	testq	%rax, %rax
14201a60f: 74 3d                       	je	0x14201a64e <.text+0x201964e>
14201a611: 48 8b 4f 18                 	movq	0x18(%rdi), %rcx
14201a615: 48 f7 d0                    	notq	%rax
14201a618: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14201a61c: 48 c1 e0 04                 	shlq	$0x4, %rax
14201a620: 48 8b 44 01 18              	movq	0x18(%rcx,%rax), %rax
14201a625: 48 85 c0                    	testq	%rax, %rax
14201a628: 74 24                       	je	0x14201a64e <.text+0x201964e>
14201a62a: 48 8d 4c 24 40              	leaq	0x40(%rsp), %rcx
14201a62f: ff d0                       	callq	*%rax
14201a631: eb 1b                       	jmp	0x14201a64e <.text+0x201964e>
14201a633: 41 89 ec                    	movl	%ebp, %r12d
14201a636: 4c 8d 44 24 40              	leaq	0x40(%rsp), %r8
14201a63b: 4c 89 f9                    	movq	%r15, %rcx
14201a63e: 4c 89 e2                    	movq	%r12, %rdx
14201a641: e8 ba 4d ce fe              	callq	0x140cff400 <.text+0xcfe400>
14201a646: 49 8b 47 28                 	movq	0x28(%r15), %rax
14201a64a: 42 89 1c a0                 	movl	%ebx, (%rax,%r12,4)
14201a64e: 41 8b 46 08                 	movl	0x8(%r14), %eax
14201a652: 89 44 24 48                 	movl	%eax, 0x48(%rsp)
14201a656: 49 8b 06                    	movq	(%r14), %rax
14201a659: 48 89 44 24 40              	movq	%rax, 0x40(%rsp)
14201a65e: 48 8b 44 24 38              	movq	0x38(%rsp), %rax
14201a663: 48 8b 40 30                 	movq	0x30(%rax), %rax
14201a667: 48 8b 8c 24 20 01 00 00     	movq	0x120(%rsp), %rcx
14201a66f: 0f b6 49 06                 	movzbl	0x6(%rcx), %ecx
14201a673: 49 39 c5                    	cmpq	%rax, %r13
14201a676: 76 4e                       	jbe	0x14201a6c6 <.text+0x20196c6>
14201a678: 48 8b 57 38                 	movq	0x38(%rdi), %rdx
14201a67c: 48 8b 14 c2                 	movq	(%rdx,%rax,8), %rdx
14201a680: 48 85 d2                    	testq	%rdx, %rdx
14201a683: 74 41                       	je	0x14201a6c6 <.text+0x20196c6>
14201a685: 48 f7 d2                    	notq	%rdx
14201a688: 4c 8d 3c 52                 	leaq	(%rdx,%rdx,2), %r15
14201a68c: 49 c1 e7 04                 	shlq	$0x4, %r15
14201a690: 4c 03 7f 18                 	addq	0x18(%rdi), %r15
14201a694: 84 c9                       	testb	%cl, %cl
14201a696: 75 35                       	jne	0x14201a6cd <.text+0x20196cd>
14201a698: 41 89 ee                    	movl	%ebp, %r14d
14201a69b: 4d 8b 47 08                 	movq	0x8(%r15), %r8
14201a69f: 4c 89 c1                    	movq	%r8, %rcx
14201a6a2: 49 0f af ce                 	imulq	%r14, %rcx
14201a6a6: 49 03 4f 10                 	addq	0x10(%r15), %rcx
14201a6aa: 48 8d 54 24 40              	leaq	0x40(%rsp), %rdx
14201a6af: e8 5c 95 90 00              	callq	0x142923c10 <.text+0x2922c10>
14201a6b4: 49 8b 47 20                 	movq	0x20(%r15), %rax
14201a6b8: 42 89 1c b0                 	movl	%ebx, (%rax,%r14,4)
14201a6bc: 49 8b 47 28                 	movq	0x28(%r15), %rax
14201a6c0: 42 89 1c b0                 	movl	%ebx, (%rax,%r14,4)
14201a6c4: eb 60                       	jmp	0x14201a726 <.text+0x2019726>
14201a6c6: 45 31 ff                    	xorl	%r15d, %r15d
14201a6c9: 84 c9                       	testb	%cl, %cl
14201a6cb: 74 cb                       	je	0x14201a698 <.text+0x2019698>
14201a6cd: 80 bc 24 60 01 00 00 00     	cmpb	$0x0, 0x160(%rsp)
14201a6d5: 74 34                       	je	0x14201a70b <.text+0x201970b>
14201a6d7: 49 39 c5                    	cmpq	%rax, %r13
14201a6da: 76 4a                       	jbe	0x14201a726 <.text+0x2019726>
14201a6dc: 48 8b 4f 38                 	movq	0x38(%rdi), %rcx
14201a6e0: 48 8b 04 c1                 	movq	(%rcx,%rax,8), %rax
14201a6e4: 48 85 c0                    	testq	%rax, %rax
14201a6e7: 74 3d                       	je	0x14201a726 <.text+0x2019726>
14201a6e9: 48 8b 4f 18                 	movq	0x18(%rdi), %rcx
14201a6ed: 48 f7 d0                    	notq	%rax
14201a6f0: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14201a6f4: 48 c1 e0 04                 	shlq	$0x4, %rax
14201a6f8: 48 8b 44 01 18              	movq	0x18(%rcx,%rax), %rax
14201a6fd: 48 85 c0                    	testq	%rax, %rax
14201a700: 74 24                       	je	0x14201a726 <.text+0x2019726>
14201a702: 48 8d 4c 24 40              	leaq	0x40(%rsp), %rcx
14201a707: ff d0                       	callq	*%rax
14201a709: eb 1b                       	jmp	0x14201a726 <.text+0x2019726>
14201a70b: 41 89 ec                    	movl	%ebp, %r12d
14201a70e: 4c 8d 44 24 40              	leaq	0x40(%rsp), %r8
14201a713: 4c 89 f9                    	movq	%r15, %rcx
14201a716: 4c 89 e2                    	movq	%r12, %rdx
14201a719: e8 e2 4c ce fe              	callq	0x140cff400 <.text+0xcfe400>
14201a71e: 49 8b 47 28                 	movq	0x28(%r15), %rax
14201a722: 42 89 1c a0                 	movl	%ebx, (%rax,%r12,4)
14201a726: f3 0f 11 74 24 40           	movss	%xmm6, 0x40(%rsp)
14201a72c: 48 8b 44 24 38              	movq	0x38(%rsp), %rax
14201a731: 48 8b 40 38                 	movq	0x38(%rax), %rax
14201a735: 48 8b 8c 24 20 01 00 00     	movq	0x120(%rsp), %rcx
14201a73d: 0f b6 51 07                 	movzbl	0x7(%rcx), %edx
14201a741: 49 39 c5                    	cmpq	%rax, %r13
14201a744: 76 67                       	jbe	0x14201a7ad <.text+0x20197ad>
14201a746: 48 8b 4f 38                 	movq	0x38(%rdi), %rcx
14201a74a: 48 8b 0c c1                 	movq	(%rcx,%rax,8), %rcx
14201a74e: 48 85 c9                    	testq	%rcx, %rcx
14201a751: 74 5a                       	je	0x14201a7ad <.text+0x20197ad>
14201a753: 48 f7 d1                    	notq	%rcx
14201a756: 4c 8d 34 49                 	leaq	(%rcx,%rcx,2), %r14
14201a75a: 49 c1 e6 04                 	shlq	$0x4, %r14
14201a75e: 4c 03 77 18                 	addq	0x18(%rdi), %r14
14201a762: 4c 8b a4 24 38 01 00 00     	movq	0x138(%rsp), %r12
14201a76a: 48 8b 8c 24 30 01 00 00     	movq	0x130(%rsp), %rcx
14201a772: 84 d2                       	testb	%dl, %dl
14201a774: 75 4e                       	jne	0x14201a7c4 <.text+0x20197c4>
14201a776: 41 89 ed                    	movl	%ebp, %r13d
14201a779: 4d 8b 46 08                 	movq	0x8(%r14), %r8
14201a77d: 4c 89 c0                    	movq	%r8, %rax
14201a780: 49 0f af c5                 	imulq	%r13, %rax
14201a784: 49 03 46 10                 	addq	0x10(%r14), %rax
14201a788: 48 8d 54 24 40              	leaq	0x40(%rsp), %rdx
14201a78d: 49 89 cf                    	movq	%rcx, %r15
14201a790: 48 89 c1                    	movq	%rax, %rcx
14201a793: e8 78 94 90 00              	callq	0x142923c10 <.text+0x2922c10>
14201a798: 4c 89 f9                    	movq	%r15, %rcx
14201a79b: 49 8b 46 20                 	movq	0x20(%r14), %rax
14201a79f: 42 89 1c a8                 	movl	%ebx, (%rax,%r13,4)
14201a7a3: 49 8b 46 28                 	movq	0x28(%r14), %rax
14201a7a7: 42 89 1c a8                 	movl	%ebx, (%rax,%r13,4)
14201a7ab: eb 7f                       	jmp	0x14201a82c <.text+0x201982c>
14201a7ad: 45 31 f6                    	xorl	%r14d, %r14d
14201a7b0: 4c 8b a4 24 38 01 00 00     	movq	0x138(%rsp), %r12
14201a7b8: 48 8b 8c 24 30 01 00 00     	movq	0x130(%rsp), %rcx
14201a7c0: 84 d2                       	testb	%dl, %dl
14201a7c2: 74 b2                       	je	0x14201a776 <.text+0x2019776>
14201a7c4: 80 bc 24 60 01 00 00 00     	cmpb	$0x0, 0x160(%rsp)
14201a7cc: 74 3d                       	je	0x14201a80b <.text+0x201980b>
14201a7ce: 49 39 c5                    	cmpq	%rax, %r13
14201a7d1: 76 59                       	jbe	0x14201a82c <.text+0x201982c>
14201a7d3: 48 8b 57 38                 	movq	0x38(%rdi), %rdx
14201a7d7: 48 8b 04 c2                 	movq	(%rdx,%rax,8), %rax
14201a7db: 48 85 c0                    	testq	%rax, %rax
14201a7de: 74 4c                       	je	0x14201a82c <.text+0x201982c>
14201a7e0: 48 8b 57 18                 	movq	0x18(%rdi), %rdx
14201a7e4: 48 f7 d0                    	notq	%rax
14201a7e7: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14201a7eb: 48 c1 e0 04                 	shlq	$0x4, %rax
14201a7ef: 48 8b 54 02 18              	movq	0x18(%rdx,%rax), %rdx
14201a7f4: 48 85 d2                    	testq	%rdx, %rdx
14201a7f7: 74 33                       	je	0x14201a82c <.text+0x201982c>
14201a7f9: 48 8d 44 24 40              	leaq	0x40(%rsp), %rax
14201a7fe: 49 89 ce                    	movq	%rcx, %r14
14201a801: 48 89 c1                    	movq	%rax, %rcx
14201a804: ff d2                       	callq	*%rdx
14201a806: 4c 89 f1                    	movq	%r14, %rcx
14201a809: eb 21                       	jmp	0x14201a82c <.text+0x201982c>
14201a80b: 41 89 ef                    	movl	%ebp, %r15d
14201a80e: 4c 8d 44 24 40              	leaq	0x40(%rsp), %r8
14201a813: 49 89 cd                    	movq	%rcx, %r13
14201a816: 4c 89 f1                    	movq	%r14, %rcx
14201a819: 4c 89 fa                    	movq	%r15, %rdx
14201a81c: e8 df 4b ce fe              	callq	0x140cff400 <.text+0xcfe400>
14201a821: 4c 89 e9                    	movq	%r13, %rcx
14201a824: 49 8b 46 28                 	movq	0x28(%r14), %rax
14201a828: 42 89 1c b8                 	movl	%ebx, (%rax,%r15,4)
14201a82c: 4c 39 e1                    	cmpq	%r12, %rcx
14201a82f: 74 32                       	je	0x14201a863 <.text+0x2019863>
14201a831: 4c 8b b4 24 40 01 00 00     	movq	0x140(%rsp), %r14
14201a839: 0f 1f 80 00 00 00 00        	nopl	(%rax)
14201a840: 4c 8d 79 10                 	leaq	0x10(%rcx), %r15
14201a844: 4c 89 74 24 28              	movq	%r14, 0x28(%rsp)
14201a849: 89 6c 24 20                 	movl	%ebp, 0x20(%rsp)
14201a84d: 48 89 fa                    	movq	%rdi, %rdx
14201a850: 49 89 f0                    	movq	%rsi, %r8
14201a853: 41 89 d9                    	movl	%ebx, %r9d
14201a856: e8 35 60 d2 fe              	callq	0x140d40890 <.text+0xd3f890>
14201a85b: 4c 89 f9                    	movq	%r15, %rcx
14201a85e: 4d 39 e7                    	cmpq	%r12, %r15
14201a861: 75 dd                       	jne	0x14201a840 <.text+0x2019840>
14201a863: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
14201a86b: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
14201a873: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
14201a87c: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
14201a883: 5b                          	popq	%rbx
14201a884: 5d                          	popq	%rbp
14201a885: 5f                          	popq	%rdi
14201a886: 5e                          	popq	%rsi
14201a887: 41 5c                       	popq	%r12
14201a889: 41 5d                       	popq	%r13
14201a88b: 41 5e                       	popq	%r14
14201a88d: 41 5f                       	popq	%r15
14201a88f: c3                          	retq
