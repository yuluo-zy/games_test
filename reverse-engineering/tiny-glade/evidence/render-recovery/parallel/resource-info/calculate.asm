D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140530430: 41 57                       	pushq	%r15
140530432: 41 56                       	pushq	%r14
140530434: 41 55                       	pushq	%r13
140530436: 41 54                       	pushq	%r12
140530438: 56                          	pushq	%rsi
140530439: 57                          	pushq	%rdi
14053043a: 55                          	pushq	%rbp
14053043b: 53                          	pushq	%rbx
14053043c: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140530443: 4c 89 ce                    	movq	%r9, %rsi
140530446: 4c 89 44 24 38              	movq	%r8, 0x38(%rsp)
14053044b: 49 89 d7                    	movq	%rdx, %r15
14053044e: 49 89 ce                    	movq	%rcx, %r14
140530451: e8 ba 3f 6a 00              	callq	0x140bd4410 <.text+0xbd3410>
140530456: 49 8b 57 08                 	movq	0x8(%r15), %rdx
14053045a: 4d 8b 47 10                 	movq	0x10(%r15), %r8
14053045e: 31 c0                       	xorl	%eax, %eax
140530460: 49 89 f5                    	movq	%rsi, %r13
140530463: 89 f1                       	movl	%esi, %ecx
140530465: 83 c1 ff                    	addl	$-0x1, %ecx
140530468: 0f 92 c0                    	setb	%al
14053046b: 89 44 24 60                 	movl	%eax, 0x60(%rsp)
14053046f: 89 4c 24 64                 	movl	%ecx, 0x64(%rsp)
140530473: 4b 8d 04 c0                 	leaq	(%r8,%r8,8), %rax
140530477: 48 89 54 24 48              	movq	%rdx, 0x48(%rsp)
14053047c: 48 8d 3c c2                 	leaq	(%rdx,%rax,8), %rdi
140530480: 49 8d 46 40                 	leaq	0x40(%r14), %rax
140530484: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
140530489: 49 8b 46 30                 	movq	0x30(%r14), %rax
14053048d: 49 8b 56 40                 	movq	0x40(%r14), %rdx
140530491: 48 29 d0                    	subq	%rdx, %rax
140530494: 4c 89 44 24 30              	movq	%r8, 0x30(%rsp)
140530499: 4c 39 c0                    	cmpq	%r8, %rax
14053049c: 0f 82 2e 04 00 00           	jb	0x1405308d0 <.text+0x52f8d0>
1405304a2: 49 8b 46 38                 	movq	0x38(%r14), %rax
1405304a6: 48 8b 4c 24 48              	movq	0x48(%rsp), %rcx
1405304ab: 48 89 8c 24 a0 00 00 00     	movq	%rcx, 0xa0(%rsp)
1405304b3: 48 89 bc 24 a8 00 00 00     	movq	%rdi, 0xa8(%rsp)
1405304bb: 48 8d 4c 24 60              	leaq	0x60(%rsp), %rcx
1405304c0: 48 89 8c 24 b0 00 00 00     	movq	%rcx, 0xb0(%rsp)
1405304c8: 48 8b 4c 24 50              	movq	0x50(%rsp), %rcx
1405304cd: 48 89 8c 24 88 00 00 00     	movq	%rcx, 0x88(%rsp)
1405304d5: 48 89 94 24 90 00 00 00     	movq	%rdx, 0x90(%rsp)
1405304dd: 48 89 84 24 98 00 00 00     	movq	%rax, 0x98(%rsp)
1405304e5: 48 8d 8c 24 a0 00 00 00     	leaq	0xa0(%rsp), %rcx
1405304ed: 48 8d 94 24 88 00 00 00     	leaq	0x88(%rsp), %rdx
1405304f5: e8 06 d5 f5 ff              	callq	0x14048da00 <.text+0x48ca00>
1405304fa: 4d 8b 66 58                 	movq	0x58(%r14), %r12
1405304fe: 48 8b 5c 24 30              	movq	0x30(%rsp), %rbx
140530503: 48 89 df                    	movq	%rbx, %rdi
140530506: 4c 29 e7                    	subq	%r12, %rdi
140530509: 76 67                       	jbe	0x140530572 <.text+0x52f572>
14053050b: 49 8b 46 48                 	movq	0x48(%r14), %rax
14053050f: 4c 29 e0                    	subq	%r12, %rax
140530512: 4c 89 e3                    	movq	%r12, %rbx
140530515: 48 39 f8                    	cmpq	%rdi, %rax
140530518: 0f 82 fb 03 00 00           	jb	0x140530919 <.text+0x52f919>
14053051e: 49 8b 6e 50                 	movq	0x50(%r14), %rbp
140530522: 48 8d 0c 9d 00 00 00 00     	leaq	(,%rbx,4), %rcx
14053052a: 48 01 e9                    	addq	%rbp, %rcx
14053052d: 48 83 ff 02                 	cmpq	$0x2, %rdi
140530531: 72 36                       	jb	0x140530569 <.text+0x52f569>
140530533: 4d 89 e0                    	movq	%r12, %r8
140530536: 49 f7 d0                    	notq	%r8
140530539: 48 8b 74 24 30              	movq	0x30(%rsp), %rsi
14053053e: 49 01 f0                    	addq	%rsi, %r8
140530541: 49 c1 e0 02                 	shlq	$0x2, %r8
140530545: 31 d2                       	xorl	%edx, %edx
140530547: e8 d4 36 3f 02              	callq	0x142923c20 <.text+0x2922c20>
14053054c: 48 8d 04 33                 	leaq	(%rbx,%rsi), %rax
140530550: 48 c1 e0 02                 	shlq	$0x2, %rax
140530554: 49 c1 e4 02                 	shlq	$0x2, %r12
140530558: 4c 29 e0                    	subq	%r12, %rax
14053055b: 48 8d 0c 28                 	leaq	(%rax,%rbp), %rcx
14053055f: 48 83 c1 fc                 	addq	$-0x4, %rcx
140530563: 48 01 fb                    	addq	%rdi, %rbx
140530566: 48 ff cb                    	decq	%rbx
140530569: c7 01 00 00 00 00           	movl	$0x0, (%rcx)
14053056f: 48 ff c3                    	incq	%rbx
140530572: 49 89 5e 58                 	movq	%rbx, 0x58(%r14)
140530576: 4d 8b 66 70                 	movq	0x70(%r14), %r12
14053057a: 48 8b 5c 24 30              	movq	0x30(%rsp), %rbx
14053057f: 48 89 df                    	movq	%rbx, %rdi
140530582: 4c 29 e7                    	subq	%r12, %rdi
140530585: 76 67                       	jbe	0x1405305ee <.text+0x52f5ee>
140530587: 49 8b 46 60                 	movq	0x60(%r14), %rax
14053058b: 4c 29 e0                    	subq	%r12, %rax
14053058e: 4c 89 e3                    	movq	%r12, %rbx
140530591: 48 39 f8                    	cmpq	%rdi, %rax
140530594: 0f 82 bf 03 00 00           	jb	0x140530959 <.text+0x52f959>
14053059a: 49 8b 6e 68                 	movq	0x68(%r14), %rbp
14053059e: 48 8d 0c 9d 00 00 00 00     	leaq	(,%rbx,4), %rcx
1405305a6: 48 01 e9                    	addq	%rbp, %rcx
1405305a9: 48 83 ff 02                 	cmpq	$0x2, %rdi
1405305ad: 72 36                       	jb	0x1405305e5 <.text+0x52f5e5>
1405305af: 4d 89 e0                    	movq	%r12, %r8
1405305b2: 49 f7 d0                    	notq	%r8
1405305b5: 48 8b 74 24 30              	movq	0x30(%rsp), %rsi
1405305ba: 49 01 f0                    	addq	%rsi, %r8
1405305bd: 49 c1 e0 02                 	shlq	$0x2, %r8
1405305c1: 31 d2                       	xorl	%edx, %edx
1405305c3: e8 58 36 3f 02              	callq	0x142923c20 <.text+0x2922c20>
1405305c8: 48 8d 04 33                 	leaq	(%rbx,%rsi), %rax
1405305cc: 48 c1 e0 02                 	shlq	$0x2, %rax
1405305d0: 49 c1 e4 02                 	shlq	$0x2, %r12
1405305d4: 4c 29 e0                    	subq	%r12, %rax
1405305d7: 48 8d 0c 28                 	leaq	(%rax,%rbp), %rcx
1405305db: 48 83 c1 fc                 	addq	$-0x4, %rcx
1405305df: 48 01 fb                    	addq	%rdi, %rbx
1405305e2: 48 ff cb                    	decq	%rbx
1405305e5: c7 01 00 00 00 00           	movl	$0x0, (%rcx)
1405305eb: 48 ff c3                    	incq	%rbx
1405305ee: 49 89 5e 70                 	movq	%rbx, 0x70(%r14)
1405305f2: 4c 89 f1                    	movq	%r14, %rcx
1405305f5: 4c 89 fa                    	movq	%r15, %rdx
1405305f8: e8 b3 3e 6a 00              	callq	0x140bd44b0 <.text+0xbd34b0>
1405305fd: 4c 89 f1                    	movq	%r14, %rcx
140530600: 4c 89 fa                    	movq	%r15, %rdx
140530603: e8 58 3f 6a 00              	callq	0x140bd4560 <.text+0xbd3560>
140530608: 4d 85 ed                    	testq	%r13, %r13
14053060b: 48 8b 4c 24 38              	movq	0x38(%rsp), %rcx
140530610: 0f 84 95 02 00 00           	je	0x1405308ab <.text+0x52f8ab>
140530616: 49 c1 e5 06                 	shlq	$0x6, %r13
14053061a: 49 01 cd                    	addq	%rcx, %r13
14053061d: 4c 89 6c 24 70              	movq	%r13, 0x70(%rsp)
140530622: 49 8d 46 18                 	leaq	0x18(%r14), %rax
140530626: 48 89 44 24 68              	movq	%rax, 0x68(%rsp)
14053062b: 45 31 e4                    	xorl	%r12d, %r12d
14053062e: eb 2b                       	jmp	0x14053065b <.text+0x52f65b>
140530630: 48 8b 4c 24 38              	movq	0x38(%rsp), %rcx
140530635: 48 83 c1 40                 	addq	$0x40, %rcx
140530639: 49 ff c4                    	incq	%r12
14053063c: 49 8b 46 20                 	movq	0x20(%r14), %rax
140530640: 42 89 3c f8                 	movl	%edi, (%rax,%r15,8)
140530644: 42 89 5c f8 04              	movl	%ebx, 0x4(%rax,%r15,8)
140530649: 49 ff c7                    	incq	%r15
14053064c: 4d 89 7e 28                 	movq	%r15, 0x28(%r14)
140530650: 48 3b 4c 24 70              	cmpq	0x70(%rsp), %rcx
140530655: 0f 84 50 02 00 00           	je	0x1405308ab <.text+0x52f8ab>
14053065b: 49 8b 46 10                 	movq	0x10(%r14), %rax
14053065f: 48 89 44 24 78              	movq	%rax, 0x78(%rsp)
140530664: 48 89 4c 24 38              	movq	%rcx, 0x38(%rsp)
140530669: 48 8b 41 10                 	movq	0x10(%rcx), %rax
14053066d: 48 85 c0                    	testq	%rax, %rax
140530670: 0f 84 fa 01 00 00           	je	0x140530870 <.text+0x52f870>
140530676: 48 8b 4c 24 38              	movq	0x38(%rsp), %rcx
14053067b: 48 8b 79 08                 	movq	0x8(%rcx), %rdi
14053067f: 48 c1 e0 02                 	shlq	$0x2, %rax
140530683: 48 8d 34 40                 	leaq	(%rax,%rax,2), %rsi
140530687: 4c 89 a4 24 80 00 00 00     	movq	%r12, 0x80(%rsp)
14053068f: eb 49                       	jmp	0x1405306da <.text+0x52f6da>
140530691: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1405306a0: e8 9b c7 6b 00              	callq	0x140bece40 <.text+0xbebe40>
1405306a5: 48 89 c3                    	movq	%rax, %rbx
1405306a8: 49 8b 56 58                 	movq	0x58(%r14), %rdx
1405306ac: 48 39 d0                    	cmpq	%rdx, %rax
1405306af: 0f 83 e4 02 00 00           	jae	0x140530999 <.text+0x52f999>
1405306b5: 48 c1 e3 02                 	shlq	$0x2, %rbx
1405306b9: 49 03 5e 50                 	addq	0x50(%r14), %rbx
1405306bd: 44 89 f9                    	movl	%r15d, %ecx
1405306c0: e8 0b 43 6a 00              	callq	0x140bd49d0 <.text+0xbd39d0>
1405306c5: 09 03                       	orl	%eax, (%rbx)
1405306c7: 48 8b 74 24 58              	movq	0x58(%rsp), %rsi
1405306cc: 48 83 c7 0c                 	addq	$0xc, %rdi
1405306d0: 48 83 c6 f4                 	addq	$-0xc, %rsi
1405306d4: 0f 84 96 01 00 00           	je	0x140530870 <.text+0x52f870>
1405306da: 48 89 f9                    	movq	%rdi, %rcx
1405306dd: e8 5e c7 6b 00              	callq	0x140bece40 <.text+0xbebe40>
1405306e2: 48 8b 4c 24 50              	movq	0x50(%rsp), %rcx
1405306e7: 48 8b 11                    	movq	(%rcx), %rdx
1405306ea: 48 39 d0                    	cmpq	%rdx, %rax
1405306ed: 0f 83 17 02 00 00           	jae	0x14053090a <.text+0x52f90a>
1405306f3: 48 89 74 24 58              	movq	%rsi, 0x58(%rsp)
1405306f8: 49 8b 4e 10                 	movq	0x10(%r14), %rcx
1405306fc: 49 8b 5e 38                 	movq	0x38(%r14), %rbx
140530700: 4c 8d 24 40                 	leaq	(%rax,%rax,2), %r12
140530704: e8 77 44 6a 00              	callq	0x140bd4b80 <.text+0xbd3b80>
140530709: 89 44 24 44                 	movl	%eax, 0x44(%rsp)
14053070d: 44 8b 3f                    	movl	(%rdi), %r15d
140530710: 8b 57 04                    	movl	0x4(%rdi), %edx
140530713: 42 8b 6c e3 14              	movl	0x14(%rbx,%r12,8), %ebp
140530718: 0f b6 77 08                 	movzbl	0x8(%rdi), %esi
14053071c: 4d 8b 6e 10                 	movq	0x10(%r14), %r13
140530720: 4d 3b 2e                    	cmpq	(%r14), %r13
140530723: 75 17                       	jne	0x14053073c <.text+0x52f73c>
140530725: 4c 89 f1                    	movq	%r14, %rcx
140530728: 89 54 24 40                 	movl	%edx, 0x40(%rsp)
14053072c: 48 8d 15 c5 2d 4c 02        	leaq	0x24c2dc5(%rip), %rdx   # 0x1429f34f8
140530733: e8 c8 2e 03 00              	callq	0x140563600 <.text+0x562600>
140530738: 8b 54 24 40                 	movl	0x40(%rsp), %edx
14053073c: 4a 8d 1c e3                 	leaq	(%rbx,%r12,8), %rbx
140530740: 49 8b 46 08                 	movq	0x8(%r14), %rax
140530744: 4a 8d 0c ad 00 00 00 00     	leaq	(,%r13,4), %rcx
14053074c: 4c 01 e9                    	addq	%r13, %rcx
14053074f: 44 89 3c 88                 	movl	%r15d, (%rax,%rcx,4)
140530753: 89 6c 88 04                 	movl	%ebp, 0x4(%rax,%rcx,4)
140530757: c7 44 88 08 ff ff ff ff     	movl	$0xffffffff, 0x8(%rax,%rcx,4) # imm = 0xFFFFFFFF
14053075f: 89 54 88 0c                 	movl	%edx, 0xc(%rax,%rcx,4)
140530763: 40 88 74 88 10              	movb	%sil, 0x10(%rax,%rcx,4)
140530768: 49 ff c5                    	incq	%r13
14053076b: 4d 89 6e 10                 	movq	%r13, 0x10(%r14)
14053076f: 4c 8b a4 24 80 00 00 00     	movq	0x80(%rsp), %r12
140530777: 4c 89 e1                    	movq	%r12, %rcx
14053077a: e8 c1 43 6a 00              	callq	0x140bd4b40 <.text+0xbd3b40>
14053077f: f6 03 01                    	testb	$0x1, (%rbx)
140530782: 75 09                       	jne	0x14053078d <.text+0x52f78d>
140530784: c7 03 01 00 00 00           	movl	$0x1, (%rbx)
14053078a: 89 43 04                    	movl	%eax, 0x4(%rbx)
14053078d: 83 7b 08 01                 	cmpl	$0x1, 0x8(%rbx)
140530791: 75 2d                       	jne	0x1405307c0 <.text+0x52f7c0>
140530793: 8b 73 0c                    	movl	0xc(%rbx), %esi
140530796: 4c 89 e1                    	movq	%r12, %rcx
140530799: e8 a2 43 6a 00              	callq	0x140bd4b40 <.text+0xbd3b40>
14053079e: 39 f0                       	cmpl	%esi, %eax
1405307a0: 0f 47 f0                    	cmoval	%eax, %esi
1405307a3: 89 73 0c                    	movl	%esi, 0xc(%rbx)
1405307a6: 83 7b 10 ff                 	cmpl	$-0x1, 0x10(%rbx)
1405307aa: 8b 44 24 44                 	movl	0x44(%rsp), %eax
1405307ae: 75 2f                       	jne	0x1405307df <.text+0x52f7df>
1405307b0: eb 2a                       	jmp	0x1405307dc <.text+0x52f7dc>
1405307b2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1405307c0: 4c 89 e1                    	movq	%r12, %rcx
1405307c3: e8 78 43 6a 00              	callq	0x140bd4b40 <.text+0xbd3b40>
1405307c8: c7 43 08 01 00 00 00        	movl	$0x1, 0x8(%rbx)
1405307cf: 89 43 0c                    	movl	%eax, 0xc(%rbx)
1405307d2: 83 7b 10 ff                 	cmpl	$-0x1, 0x10(%rbx)
1405307d6: 8b 44 24 44                 	movl	0x44(%rsp), %eax
1405307da: 75 03                       	jne	0x1405307df <.text+0x52f7df>
1405307dc: 89 43 10                    	movl	%eax, 0x10(%rbx)
1405307df: 89 43 14                    	movl	%eax, 0x14(%rbx)
1405307e2: 0f b6 57 08                 	movzbl	0x8(%rdi), %edx
1405307e6: 48 8d 8c 24 88 00 00 00     	leaq	0x88(%rsp), %rcx
1405307ee: e8 dd 05 6e 00              	callq	0x140c10dd0 <.text+0xc0fdd0>
1405307f3: 44 8b bc 24 8c 00 00 00     	movl	0x8c(%rsp), %r15d
1405307fb: 48 89 f9                    	movq	%rdi, %rcx
1405307fe: e8 3d c6 6b 00              	callq	0x140bece40 <.text+0xbebe40>
140530803: 48 3b 44 24 30              	cmpq	0x30(%rsp), %rax
140530808: 0f 83 e8 00 00 00           	jae	0x1405308f6 <.text+0x52f8f6>
14053080e: 48 8d 04 c0                 	leaq	(%rax,%rax,8), %rax
140530812: 48 8b 4c 24 48              	movq	0x48(%rsp), %rcx
140530817: 48 8d 0c c1                 	leaq	(%rcx,%rax,8), %rcx
14053081b: e8 f0 64 6a 00              	callq	0x140bd6d10 <.text+0xbd5d10>
140530820: 3c 02                       	cmpb	$0x2, %al
140530822: 48 8b 74 24 58              	movq	0x58(%rsp), %rsi
140530827: 0f 84 9f fe ff ff           	je	0x1405306cc <.text+0x52f6cc>
14053082d: 0f b6 c0                    	movzbl	%al, %eax
140530830: 48 89 f9                    	movq	%rdi, %rcx
140530833: 83 f8 01                    	cmpl	$0x1, %eax
140530836: 0f 85 64 fe ff ff           	jne	0x1405306a0 <.text+0x52f6a0>
14053083c: e8 ff c5 6b 00              	callq	0x140bece40 <.text+0xbebe40>
140530841: 48 89 c3                    	movq	%rax, %rbx
140530844: 49 8b 56 70                 	movq	0x70(%r14), %rdx
140530848: 48 39 d0                    	cmpq	%rdx, %rax
14053084b: 0f 83 57 01 00 00           	jae	0x1405309a8 <.text+0x52f9a8>
140530851: 48 c1 e3 02                 	shlq	$0x2, %rbx
140530855: 49 03 5e 68                 	addq	0x68(%r14), %rbx
140530859: 44 89 f9                    	movl	%r15d, %ecx
14053085c: e8 9f 42 6a 00              	callq	0x140bd4b00 <.text+0xbd3b00>
140530861: e9 5f fe ff ff              	jmp	0x1405306c5 <.text+0x52f6c5>
140530866: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140530870: 48 8b 4c 24 78              	movq	0x78(%rsp), %rcx
140530875: e8 06 43 6a 00              	callq	0x140bd4b80 <.text+0xbd3b80>
14053087a: 89 c7                       	movl	%eax, %edi
14053087c: 49 8b 4e 10                 	movq	0x10(%r14), %rcx
140530880: e8 fb 42 6a 00              	callq	0x140bd4b80 <.text+0xbd3b80>
140530885: 89 c3                       	movl	%eax, %ebx
140530887: 4d 8b 7e 28                 	movq	0x28(%r14), %r15
14053088b: 4d 3b 7e 18                 	cmpq	0x18(%r14), %r15
14053088f: 0f 85 9b fd ff ff           	jne	0x140530630 <.text+0x52f630>
140530895: 48 8b 4c 24 68              	movq	0x68(%rsp), %rcx
14053089a: 48 8d 15 27 2c 4c 02        	leaq	0x24c2c27(%rip), %rdx   # 0x1429f34c8
1405308a1: e8 ca 25 03 00              	callq	0x140562e70 <.text+0x561e70>
1405308a6: e9 85 fd ff ff              	jmp	0x140530630 <.text+0x52f630>
1405308ab: 4c 89 f1                    	movq	%r14, %rcx
1405308ae: e8 1d 3f 6a 00              	callq	0x140bd47d0 <.text+0xbd37d0>
1405308b3: 4c 89 f1                    	movq	%r14, %rcx
1405308b6: e8 a5 3f 6a 00              	callq	0x140bd4860 <.text+0xbd3860>
1405308bb: 90                          	nop
1405308bc: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
1405308c3: 5b                          	popq	%rbx
1405308c4: 5d                          	popq	%rbp
1405308c5: 5f                          	popq	%rdi
1405308c6: 5e                          	popq	%rsi
1405308c7: 41 5c                       	popq	%r12
1405308c9: 41 5d                       	popq	%r13
1405308cb: 41 5e                       	popq	%r14
1405308cd: 41 5f                       	popq	%r15
1405308cf: c3                          	retq
1405308d0: 49 8d 4e 30                 	leaq	0x30(%r14), %rcx
1405308d4: 48 c7 44 24 20 18 00 00 00  	movq	$0x18, 0x20(%rsp)
1405308dd: 41 b9 04 00 00 00           	movl	$0x4, %r9d
1405308e3: 4c 8b 44 24 30              	movq	0x30(%rsp), %r8
1405308e8: e8 f3 20 37 02              	callq	0x1428a29e0 <.text+0x28a19e0>
1405308ed: 49 8b 56 40                 	movq	0x40(%r14), %rdx
1405308f1: e9 ac fb ff ff              	jmp	0x1405304a2 <.text+0x52f4a2>
1405308f6: 4c 8d 05 13 2c 4c 02        	leaq	0x24c2c13(%rip), %r8    # 0x1429f3510
1405308fd: 48 89 c1                    	movq	%rax, %rcx
140530900: 48 8b 54 24 30              	movq	0x30(%rsp), %rdx
140530905: e8 0e 8c 3a 02              	callq	0x1428d9518 <.text+0x28d8518>
14053090a: 4c 8d 05 cf 2b 4c 02        	leaq	0x24c2bcf(%rip), %r8    # 0x1429f34e0
140530911: 48 89 c1                    	movq	%rax, %rcx
140530914: e8 ff 8b 3a 02              	callq	0x1428d9518 <.text+0x28d8518>
140530919: 49 8d 4e 48                 	leaq	0x48(%r14), %rcx
14053091d: 48 c7 44 24 20 04 00 00 00  	movq	$0x4, 0x20(%rsp)
140530926: 41 b9 04 00 00 00           	movl	$0x4, %r9d
14053092c: 4c 89 e2                    	movq	%r12, %rdx
14053092f: 49 89 f8                    	movq	%rdi, %r8
140530932: e8 a9 20 37 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140530937: 49 8b 5e 58                 	movq	0x58(%r14), %rbx
14053093b: 49 8b 6e 50                 	movq	0x50(%r14), %rbp
14053093f: 48 8d 0c 9d 00 00 00 00     	leaq	(,%rbx,4), %rcx
140530947: 48 01 e9                    	addq	%rbp, %rcx
14053094a: 48 83 ff 02                 	cmpq	$0x2, %rdi
14053094e: 0f 83 df fb ff ff           	jae	0x140530533 <.text+0x52f533>
140530954: e9 10 fc ff ff              	jmp	0x140530569 <.text+0x52f569>
140530959: 49 8d 4e 60                 	leaq	0x60(%r14), %rcx
14053095d: 48 c7 44 24 20 04 00 00 00  	movq	$0x4, 0x20(%rsp)
140530966: 41 b9 04 00 00 00           	movl	$0x4, %r9d
14053096c: 4c 89 e2                    	movq	%r12, %rdx
14053096f: 49 89 f8                    	movq	%rdi, %r8
140530972: e8 69 20 37 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140530977: 49 8b 5e 70                 	movq	0x70(%r14), %rbx
14053097b: 49 8b 6e 68                 	movq	0x68(%r14), %rbp
14053097f: 48 8d 0c 9d 00 00 00 00     	leaq	(,%rbx,4), %rcx
140530987: 48 01 e9                    	addq	%rbp, %rcx
14053098a: 48 83 ff 02                 	cmpq	$0x2, %rdi
14053098e: 0f 83 1b fc ff ff           	jae	0x1405305af <.text+0x52f5af>
140530994: e9 4c fc ff ff              	jmp	0x1405305e5 <.text+0x52f5e5>
140530999: 4c 8d 05 88 2b 4c 02        	leaq	0x24c2b88(%rip), %r8    # 0x1429f3528
1405309a0: 48 89 d9                    	movq	%rbx, %rcx
1405309a3: e8 70 8b 3a 02              	callq	0x1428d9518 <.text+0x28d8518>
1405309a8: 4c 8d 05 91 2b 4c 02        	leaq	0x24c2b91(%rip), %r8    # 0x1429f3540
1405309af: 48 89 d9                    	movq	%rbx, %rcx
1405309b2: e8 61 8b 3a 02              	callq	0x1428d9518 <.text+0x28d8518>
1405309b7: cc                          	int3
