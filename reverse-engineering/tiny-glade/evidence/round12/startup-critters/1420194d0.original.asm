
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1420194d0: 55                          	pushq	%rbp
1420194d1: 41 57                       	pushq	%r15
1420194d3: 41 56                       	pushq	%r14
1420194d5: 41 55                       	pushq	%r13
1420194d7: 41 54                       	pushq	%r12
1420194d9: 56                          	pushq	%rsi
1420194da: 57                          	pushq	%rdi
1420194db: 53                          	pushq	%rbx
1420194dc: 48 81 ec 08 01 00 00        	subq	$0x108, %rsp            # imm = 0x108
1420194e3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1420194eb: 0f 29 7d 70                 	movaps	%xmm7, 0x70(%rbp)
1420194ef: 0f 29 75 60                 	movaps	%xmm6, 0x60(%rbp)
1420194f3: 48 c7 45 58 fe ff ff ff     	movq	$-0x2, 0x58(%rbp)
1420194fb: 4c 89 ce                    	movq	%r9, %rsi
1420194fe: 4c 89 c7                    	movq	%r8, %rdi
142019501: 4c 8b b5 28 01 00 00        	movq	0x128(%rbp), %r14
142019508: 41 0f 28 46 40              	movaps	0x40(%r14), %xmm0
14201950d: 41 0f 28 4e 50              	movaps	0x50(%r14), %xmm1
142019512: 41 0f 28 56 60              	movaps	0x60(%r14), %xmm2
142019517: 0f 29 55 30                 	movaps	%xmm2, 0x30(%rbp)
14201951b: 0f 29 4d 20                 	movaps	%xmm1, 0x20(%rbp)
14201951f: 0f 29 45 10                 	movaps	%xmm0, 0x10(%rbp)
142019523: 41 0f 10 06                 	movups	(%r14), %xmm0
142019527: 41 0f 10 4e 10              	movups	0x10(%r14), %xmm1
14201952c: 41 0f 10 56 20              	movups	0x20(%r14), %xmm2
142019531: 0f 29 55 f0                 	movaps	%xmm2, -0x10(%rbp)
142019535: 0f 29 4d e0                 	movaps	%xmm1, -0x20(%rbp)
142019539: 0f 29 45 d0                 	movaps	%xmm0, -0x30(%rbp)
14201953d: 41 8b 96 98 00 00 00        	movl	0x98(%r14), %edx
142019544: f3 41 0f 10 be 9c 00 00 00  	movss	0x9c(%r14), %xmm7
14201954d: 41 0f 10 46 30              	movups	0x30(%r14), %xmm0
142019552: 0f 29 45 c0                 	movaps	%xmm0, -0x40(%rbp)
142019556: 41 0f 10 46 70              	movups	0x70(%r14), %xmm0
14201955b: 0f 29 45 b0                 	movaps	%xmm0, -0x50(%rbp)
14201955f: 41 8b 86 a8 00 00 00        	movl	0xa8(%r14), %eax
142019566: 89 45 08                    	movl	%eax, 0x8(%rbp)
142019569: 49 8b 86 a0 00 00 00        	movq	0xa0(%r14), %rax
142019570: 48 89 45 00                 	movq	%rax, (%rbp)
142019574: 48 89 4d 40                 	movq	%rcx, 0x40(%rbp)
142019578: 48 8b 01                    	movq	(%rcx), %rax
14201957b: 49 8b 48 40                 	movq	0x40(%r8), %rcx
14201957f: 48 89 4d 48                 	movq	%rcx, 0x48(%rbp)
142019583: 48 39 c1                    	cmpq	%rax, %rcx
142019586: 89 55 0c                    	movl	%edx, 0xc(%rbp)
142019589: 76 1e                       	jbe	0x1420195a9 <.text+0x20185a9>
14201958b: 48 8b 4f 38                 	movq	0x38(%rdi), %rcx
14201958f: 48 8b 0c c1                 	movq	(%rcx,%rax,8), %rcx
142019593: 48 85 c9                    	testq	%rcx, %rcx
142019596: 74 11                       	je	0x1420195a9 <.text+0x20185a9>
142019598: 48 f7 d1                    	notq	%rcx
14201959b: 4c 8d 3c 49                 	leaq	(%rcx,%rcx,2), %r15
14201959f: 49 c1 e7 04                 	shlq	$0x4, %r15
1420195a3: 4c 03 7f 18                 	addq	0x18(%rdi), %r15
1420195a7: eb 03                       	jmp	0x1420195ac <.text+0x20185ac>
1420195a9: 45 31 ff                    	xorl	%r15d, %r15d
1420195ac: 8b 9d 20 01 00 00           	movl	0x120(%rbp), %ebx
1420195b2: 44 8b ad 18 01 00 00        	movl	0x118(%rbp), %r13d
1420195b9: f3 41 0f 10 b6 ac 00 00 00  	movss	0xac(%r14), %xmm6
1420195c2: 48 8b 8d f0 00 00 00        	movq	0xf0(%rbp), %rcx
1420195c9: 80 39 00                    	cmpb	$0x0, (%rcx)
1420195cc: 74 41                       	je	0x14201960f <.text+0x201860f>
1420195ce: 80 bd 30 01 00 00 00        	cmpb	$0x0, 0x130(%rbp)
1420195d5: 74 5d                       	je	0x142019634 <.text+0x2018634>
1420195d7: 48 39 45 48                 	cmpq	%rax, 0x48(%rbp)
1420195db: 76 75                       	jbe	0x142019652 <.text+0x2018652>
1420195dd: 48 8b 4f 38                 	movq	0x38(%rdi), %rcx
1420195e1: 48 8b 04 c1                 	movq	(%rcx,%rax,8), %rax
1420195e5: 48 85 c0                    	testq	%rax, %rax
1420195e8: 74 68                       	je	0x142019652 <.text+0x2018652>
1420195ea: 48 8b 4f 18                 	movq	0x18(%rdi), %rcx
1420195ee: 48 f7 d0                    	notq	%rax
1420195f1: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
1420195f5: 48 c1 e0 04                 	shlq	$0x4, %rax
1420195f9: 48 8b 44 01 18              	movq	0x18(%rcx,%rax), %rax
1420195fe: 48 85 c0                    	testq	%rax, %rax
142019601: 74 4f                       	je	0x142019652 <.text+0x2018652>
142019603: c6 45 57 01                 	movb	$0x1, 0x57(%rbp)
142019607: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
14201960b: ff d0                       	callq	*%rax
14201960d: eb 43                       	jmp	0x142019652 <.text+0x2018652>
14201960f: 45 89 ec                    	movl	%r13d, %r12d
142019612: 4d 8b 47 08                 	movq	0x8(%r15), %r8
142019616: 4c 89 c1                    	movq	%r8, %rcx
142019619: 49 0f af cc                 	imulq	%r12, %rcx
14201961d: 49 03 4f 10                 	addq	0x10(%r15), %rcx
142019621: 48 8d 55 10                 	leaq	0x10(%rbp), %rdx
142019625: e8 e6 a5 90 00              	callq	0x142923c10 <.text+0x2922c10>
14201962a: 49 8b 47 20                 	movq	0x20(%r15), %rax
14201962e: 42 89 1c a0                 	movl	%ebx, (%rax,%r12,4)
142019632: eb 16                       	jmp	0x14201964a <.text+0x201864a>
142019634: 45 89 ec                    	movl	%r13d, %r12d
142019637: c6 45 57 01                 	movb	$0x1, 0x57(%rbp)
14201963b: 4c 8d 45 10                 	leaq	0x10(%rbp), %r8
14201963f: 4c 89 f9                    	movq	%r15, %rcx
142019642: 4c 89 e2                    	movq	%r12, %rdx
142019645: e8 b6 5d ce fe              	callq	0x140cff400 <.text+0xcfe400>
14201964a: 49 8b 47 28                 	movq	0x28(%r15), %rax
14201964e: 42 89 1c a0                 	movl	%ebx, (%rax,%r12,4)
142019652: 49 83 ee 80                 	subq	$-0x80, %r14
142019656: 49 8b 46 10                 	movq	0x10(%r14), %rax
14201965a: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
14201965e: 41 0f 10 06                 	movups	(%r14), %xmm0
142019662: 0f 29 45 10                 	movaps	%xmm0, 0x10(%rbp)
142019666: 48 8b 45 40                 	movq	0x40(%rbp), %rax
14201966a: 48 8b 40 08                 	movq	0x8(%rax), %rax
14201966e: 48 39 45 48                 	cmpq	%rax, 0x48(%rbp)
142019672: 76 4e                       	jbe	0x1420196c2 <.text+0x20186c2>
142019674: 48 8b 4f 38                 	movq	0x38(%rdi), %rcx
142019678: 48 8b 0c c1                 	movq	(%rcx,%rax,8), %rcx
14201967c: 48 85 c9                    	testq	%rcx, %rcx
14201967f: 74 41                       	je	0x1420196c2 <.text+0x20186c2>
142019681: 48 f7 d1                    	notq	%rcx
142019684: 4c 8d 3c 49                 	leaq	(%rcx,%rcx,2), %r15
142019688: 49 c1 e7 04                 	shlq	$0x4, %r15
14201968c: 4c 03 7f 18                 	addq	0x18(%rdi), %r15
142019690: 48 8b 8d f0 00 00 00        	movq	0xf0(%rbp), %rcx
142019697: 80 79 01 00                 	cmpb	$0x0, 0x1(%rcx)
14201969b: 75 35                       	jne	0x1420196d2 <.text+0x20186d2>
14201969d: 45 89 ec                    	movl	%r13d, %r12d
1420196a0: 4d 8b 47 08                 	movq	0x8(%r15), %r8
1420196a4: 4c 89 c1                    	movq	%r8, %rcx
1420196a7: 49 0f af cc                 	imulq	%r12, %rcx
1420196ab: 49 03 4f 10                 	addq	0x10(%r15), %rcx
1420196af: 48 8d 55 10                 	leaq	0x10(%rbp), %rdx
1420196b3: e8 58 a5 90 00              	callq	0x142923c10 <.text+0x2922c10>
1420196b8: 49 8b 47 20                 	movq	0x20(%r15), %rax
1420196bc: 42 89 1c a0                 	movl	%ebx, (%rax,%r12,4)
1420196c0: eb 67                       	jmp	0x142019729 <.text+0x2018729>
1420196c2: 45 31 ff                    	xorl	%r15d, %r15d
1420196c5: 48 8b 8d f0 00 00 00        	movq	0xf0(%rbp), %rcx
1420196cc: 80 79 01 00                 	cmpb	$0x0, 0x1(%rcx)
1420196d0: 74 cb                       	je	0x14201969d <.text+0x201869d>
1420196d2: 80 bd 30 01 00 00 00        	cmpb	$0x0, 0x130(%rbp)
1420196d9: 74 38                       	je	0x142019713 <.text+0x2018713>
1420196db: 48 39 45 48                 	cmpq	%rax, 0x48(%rbp)
1420196df: 76 50                       	jbe	0x142019731 <.text+0x2018731>
1420196e1: 48 8b 4f 38                 	movq	0x38(%rdi), %rcx
1420196e5: 48 8b 04 c1                 	movq	(%rcx,%rax,8), %rax
1420196e9: 48 85 c0                    	testq	%rax, %rax
1420196ec: 74 43                       	je	0x142019731 <.text+0x2018731>
1420196ee: 48 8b 4f 18                 	movq	0x18(%rdi), %rcx
1420196f2: 48 f7 d0                    	notq	%rax
1420196f5: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
1420196f9: 48 c1 e0 04                 	shlq	$0x4, %rax
1420196fd: 48 8b 44 01 18              	movq	0x18(%rcx,%rax), %rax
142019702: 48 85 c0                    	testq	%rax, %rax
142019705: 74 2a                       	je	0x142019731 <.text+0x2018731>
142019707: c6 45 57 01                 	movb	$0x1, 0x57(%rbp)
14201970b: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
14201970f: ff d0                       	callq	*%rax
142019711: eb 1e                       	jmp	0x142019731 <.text+0x2018731>
142019713: 45 89 ec                    	movl	%r13d, %r12d
142019716: c6 45 57 01                 	movb	$0x1, 0x57(%rbp)
14201971a: 4c 8d 45 10                 	leaq	0x10(%rbp), %r8
14201971e: 4c 89 f9                    	movq	%r15, %rcx
142019721: 4c 89 e2                    	movq	%r12, %rdx
142019724: e8 d7 5c ce fe              	callq	0x140cff400 <.text+0xcfe400>
142019729: 49 8b 47 28                 	movq	0x28(%r15), %rax
14201972d: 42 89 1c a0                 	movl	%ebx, (%rax,%r12,4)
142019731: 0f 28 45 d0                 	movaps	-0x30(%rbp), %xmm0
142019735: 0f 28 4d e0                 	movaps	-0x20(%rbp), %xmm1
142019739: 0f 28 55 f0                 	movaps	-0x10(%rbp), %xmm2
14201973d: 0f 29 55 30                 	movaps	%xmm2, 0x30(%rbp)
142019741: 0f 29 4d 20                 	movaps	%xmm1, 0x20(%rbp)
142019745: 0f 29 45 10                 	movaps	%xmm0, 0x10(%rbp)
142019749: 48 8b 45 40                 	movq	0x40(%rbp), %rax
14201974d: 48 8b 40 10                 	movq	0x10(%rax), %rax
142019751: 48 39 45 48                 	cmpq	%rax, 0x48(%rbp)
142019755: 76 4e                       	jbe	0x1420197a5 <.text+0x20187a5>
142019757: 48 8b 4f 38                 	movq	0x38(%rdi), %rcx
14201975b: 48 8b 0c c1                 	movq	(%rcx,%rax,8), %rcx
14201975f: 48 85 c9                    	testq	%rcx, %rcx
142019762: 74 41                       	je	0x1420197a5 <.text+0x20187a5>
142019764: 48 f7 d1                    	notq	%rcx
142019767: 4c 8d 3c 49                 	leaq	(%rcx,%rcx,2), %r15
14201976b: 49 c1 e7 04                 	shlq	$0x4, %r15
14201976f: 4c 03 7f 18                 	addq	0x18(%rdi), %r15
142019773: 48 8b 8d f0 00 00 00        	movq	0xf0(%rbp), %rcx
14201977a: 80 79 02 00                 	cmpb	$0x0, 0x2(%rcx)
14201977e: 75 35                       	jne	0x1420197b5 <.text+0x20187b5>
142019780: 45 89 ec                    	movl	%r13d, %r12d
142019783: 4d 8b 47 08                 	movq	0x8(%r15), %r8
142019787: 4c 89 c1                    	movq	%r8, %rcx
14201978a: 49 0f af cc                 	imulq	%r12, %rcx
14201978e: 49 03 4f 10                 	addq	0x10(%r15), %rcx
142019792: 48 8d 55 10                 	leaq	0x10(%rbp), %rdx
142019796: e8 75 a4 90 00              	callq	0x142923c10 <.text+0x2922c10>
14201979b: 49 8b 47 20                 	movq	0x20(%r15), %rax
14201979f: 42 89 1c a0                 	movl	%ebx, (%rax,%r12,4)
1420197a3: eb 75                       	jmp	0x14201981a <.text+0x201881a>
1420197a5: 45 31 ff                    	xorl	%r15d, %r15d
1420197a8: 48 8b 8d f0 00 00 00        	movq	0xf0(%rbp), %rcx
1420197af: 80 79 02 00                 	cmpb	$0x0, 0x2(%rcx)
1420197b3: 74 cb                       	je	0x142019780 <.text+0x2018780>
1420197b5: 80 bd 30 01 00 00 00        	cmpb	$0x0, 0x130(%rbp)
1420197bc: 74 46                       	je	0x142019804 <.text+0x2018804>
1420197be: 48 39 45 48                 	cmpq	%rax, 0x48(%rbp)
1420197c2: 76 30                       	jbe	0x1420197f4 <.text+0x20187f4>
1420197c4: 48 8b 4f 38                 	movq	0x38(%rdi), %rcx
1420197c8: 48 8b 04 c1                 	movq	(%rcx,%rax,8), %rax
1420197cc: 48 85 c0                    	testq	%rax, %rax
1420197cf: 74 23                       	je	0x1420197f4 <.text+0x20187f4>
1420197d1: 48 8b 4f 18                 	movq	0x18(%rdi), %rcx
1420197d5: 48 f7 d0                    	notq	%rax
1420197d8: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
1420197dc: 48 c1 e0 04                 	shlq	$0x4, %rax
1420197e0: 48 8b 44 01 18              	movq	0x18(%rcx,%rax), %rax
1420197e5: 48 85 c0                    	testq	%rax, %rax
1420197e8: 74 0a                       	je	0x1420197f4 <.text+0x20187f4>
1420197ea: c6 45 57 00                 	movb	$0x0, 0x57(%rbp)
1420197ee: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
1420197f2: ff d0                       	callq	*%rax
1420197f4: 48 8b 45 40                 	movq	0x40(%rbp), %rax
1420197f8: 48 8b 40 18                 	movq	0x18(%rax), %rax
1420197fc: 48 39 45 48                 	cmpq	%rax, 0x48(%rbp)
142019800: 76 7c                       	jbe	0x14201987e <.text+0x201887e>
142019802: eb 2c                       	jmp	0x142019830 <.text+0x2018830>
142019804: 45 89 ec                    	movl	%r13d, %r12d
142019807: c6 45 57 00                 	movb	$0x0, 0x57(%rbp)
14201980b: 4c 8d 45 10                 	leaq	0x10(%rbp), %r8
14201980f: 4c 89 f9                    	movq	%r15, %rcx
142019812: 4c 89 e2                    	movq	%r12, %rdx
142019815: e8 e6 5b ce fe              	callq	0x140cff400 <.text+0xcfe400>
14201981a: 49 8b 47 28                 	movq	0x28(%r15), %rax
14201981e: 42 89 1c a0                 	movl	%ebx, (%rax,%r12,4)
142019822: 48 8b 45 40                 	movq	0x40(%rbp), %rax
142019826: 48 8b 40 18                 	movq	0x18(%rax), %rax
14201982a: 48 39 45 48                 	cmpq	%rax, 0x48(%rbp)
14201982e: 76 4e                       	jbe	0x14201987e <.text+0x201887e>
142019830: 48 8b 4f 38                 	movq	0x38(%rdi), %rcx
142019834: 48 8b 0c c1                 	movq	(%rcx,%rax,8), %rcx
142019838: 48 85 c9                    	testq	%rcx, %rcx
14201983b: 74 41                       	je	0x14201987e <.text+0x201887e>
14201983d: 48 f7 d1                    	notq	%rcx
142019840: 4c 8d 3c 49                 	leaq	(%rcx,%rcx,2), %r15
142019844: 49 c1 e7 04                 	shlq	$0x4, %r15
142019848: 4c 03 7f 18                 	addq	0x18(%rdi), %r15
14201984c: 48 8b 8d f0 00 00 00        	movq	0xf0(%rbp), %rcx
142019853: 80 79 03 00                 	cmpb	$0x0, 0x3(%rcx)
142019857: 75 35                       	jne	0x14201988e <.text+0x201888e>
142019859: 45 89 ec                    	movl	%r13d, %r12d
14201985c: 4d 8b 47 08                 	movq	0x8(%r15), %r8
142019860: 4c 89 c1                    	movq	%r8, %rcx
142019863: 49 0f af cc                 	imulq	%r12, %rcx
142019867: 49 03 4f 10                 	addq	0x10(%r15), %rcx
14201986b: 48 8d 55 10                 	leaq	0x10(%rbp), %rdx
14201986f: e8 9c a3 90 00              	callq	0x142923c10 <.text+0x2922c10>
142019874: 49 8b 47 20                 	movq	0x20(%r15), %rax
142019878: 42 89 1c a0                 	movl	%ebx, (%rax,%r12,4)
14201987c: eb 67                       	jmp	0x1420198e5 <.text+0x20188e5>
14201987e: 45 31 ff                    	xorl	%r15d, %r15d
142019881: 48 8b 8d f0 00 00 00        	movq	0xf0(%rbp), %rcx
142019888: 80 79 03 00                 	cmpb	$0x0, 0x3(%rcx)
14201988c: 74 cb                       	je	0x142019859 <.text+0x2018859>
14201988e: 80 bd 30 01 00 00 00        	cmpb	$0x0, 0x130(%rbp)
142019895: 74 38                       	je	0x1420198cf <.text+0x20188cf>
142019897: 48 39 45 48                 	cmpq	%rax, 0x48(%rbp)
14201989b: 76 50                       	jbe	0x1420198ed <.text+0x20188ed>
14201989d: 48 8b 4f 38                 	movq	0x38(%rdi), %rcx
1420198a1: 48 8b 04 c1                 	movq	(%rcx,%rax,8), %rax
1420198a5: 48 85 c0                    	testq	%rax, %rax
1420198a8: 74 43                       	je	0x1420198ed <.text+0x20188ed>
1420198aa: 48 8b 4f 18                 	movq	0x18(%rdi), %rcx
1420198ae: 48 f7 d0                    	notq	%rax
1420198b1: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
1420198b5: 48 c1 e0 04                 	shlq	$0x4, %rax
1420198b9: 48 8b 44 01 18              	movq	0x18(%rcx,%rax), %rax
1420198be: 48 85 c0                    	testq	%rax, %rax
1420198c1: 74 2a                       	je	0x1420198ed <.text+0x20188ed>
1420198c3: c6 45 57 00                 	movb	$0x0, 0x57(%rbp)
1420198c7: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
1420198cb: ff d0                       	callq	*%rax
1420198cd: eb 1e                       	jmp	0x1420198ed <.text+0x20188ed>
1420198cf: 45 89 ec                    	movl	%r13d, %r12d
1420198d2: c6 45 57 00                 	movb	$0x0, 0x57(%rbp)
1420198d6: 4c 8d 45 10                 	leaq	0x10(%rbp), %r8
1420198da: 4c 89 f9                    	movq	%r15, %rcx
1420198dd: 4c 89 e2                    	movq	%r12, %rdx
1420198e0: e8 1b 5b ce fe              	callq	0x140cff400 <.text+0xcfe400>
1420198e5: 49 8b 47 28                 	movq	0x28(%r15), %rax
1420198e9: 42 89 1c a0                 	movl	%ebx, (%rax,%r12,4)
1420198ed: 8b 45 0c                    	movl	0xc(%rbp), %eax
1420198f0: 89 45 10                    	movl	%eax, 0x10(%rbp)
1420198f3: f3 0f 11 7d 14              	movss	%xmm7, 0x14(%rbp)
1420198f8: 48 8b 45 40                 	movq	0x40(%rbp), %rax
1420198fc: 48 8b 40 20                 	movq	0x20(%rax), %rax
142019900: 48 39 45 48                 	cmpq	%rax, 0x48(%rbp)
142019904: 76 4e                       	jbe	0x142019954 <.text+0x2018954>
142019906: 48 8b 4f 38                 	movq	0x38(%rdi), %rcx
14201990a: 48 8b 0c c1                 	movq	(%rcx,%rax,8), %rcx
14201990e: 48 85 c9                    	testq	%rcx, %rcx
142019911: 74 41                       	je	0x142019954 <.text+0x2018954>
142019913: 48 f7 d1                    	notq	%rcx
142019916: 4c 8d 3c 49                 	leaq	(%rcx,%rcx,2), %r15
14201991a: 49 c1 e7 04                 	shlq	$0x4, %r15
14201991e: 4c 03 7f 18                 	addq	0x18(%rdi), %r15
142019922: 48 8b 8d f0 00 00 00        	movq	0xf0(%rbp), %rcx
142019929: 80 79 04 00                 	cmpb	$0x0, 0x4(%rcx)
14201992d: 75 35                       	jne	0x142019964 <.text+0x2018964>
14201992f: 45 89 ec                    	movl	%r13d, %r12d
142019932: 4d 8b 47 08                 	movq	0x8(%r15), %r8
142019936: 4c 89 c1                    	movq	%r8, %rcx
142019939: 49 0f af cc                 	imulq	%r12, %rcx
14201993d: 49 03 4f 10                 	addq	0x10(%r15), %rcx
142019941: 48 8d 55 10                 	leaq	0x10(%rbp), %rdx
142019945: e8 c6 a2 90 00              	callq	0x142923c10 <.text+0x2922c10>
14201994a: 49 8b 47 20                 	movq	0x20(%r15), %rax
14201994e: 42 89 1c a0                 	movl	%ebx, (%rax,%r12,4)
142019952: eb 67                       	jmp	0x1420199bb <.text+0x20189bb>
142019954: 45 31 ff                    	xorl	%r15d, %r15d
142019957: 48 8b 8d f0 00 00 00        	movq	0xf0(%rbp), %rcx
14201995e: 80 79 04 00                 	cmpb	$0x0, 0x4(%rcx)
142019962: 74 cb                       	je	0x14201992f <.text+0x201892f>
142019964: 80 bd 30 01 00 00 00        	cmpb	$0x0, 0x130(%rbp)
14201996b: 74 38                       	je	0x1420199a5 <.text+0x20189a5>
14201996d: 48 39 45 48                 	cmpq	%rax, 0x48(%rbp)
142019971: 76 50                       	jbe	0x1420199c3 <.text+0x20189c3>
142019973: 48 8b 4f 38                 	movq	0x38(%rdi), %rcx
142019977: 48 8b 04 c1                 	movq	(%rcx,%rax,8), %rax
14201997b: 48 85 c0                    	testq	%rax, %rax
14201997e: 74 43                       	je	0x1420199c3 <.text+0x20189c3>
142019980: 48 8b 4f 18                 	movq	0x18(%rdi), %rcx
142019984: 48 f7 d0                    	notq	%rax
142019987: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14201998b: 48 c1 e0 04                 	shlq	$0x4, %rax
14201998f: 48 8b 44 01 18              	movq	0x18(%rcx,%rax), %rax
142019994: 48 85 c0                    	testq	%rax, %rax
142019997: 74 2a                       	je	0x1420199c3 <.text+0x20189c3>
142019999: c6 45 57 00                 	movb	$0x0, 0x57(%rbp)
14201999d: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
1420199a1: ff d0                       	callq	*%rax
1420199a3: eb 1e                       	jmp	0x1420199c3 <.text+0x20189c3>
1420199a5: 45 89 ec                    	movl	%r13d, %r12d
1420199a8: c6 45 57 00                 	movb	$0x0, 0x57(%rbp)
1420199ac: 4c 8d 45 10                 	leaq	0x10(%rbp), %r8
1420199b0: 4c 89 f9                    	movq	%r15, %rcx
1420199b3: 4c 89 e2                    	movq	%r12, %rdx
1420199b6: e8 45 5a ce fe              	callq	0x140cff400 <.text+0xcfe400>
1420199bb: 49 8b 47 28                 	movq	0x28(%r15), %rax
1420199bf: 42 89 1c a0                 	movl	%ebx, (%rax,%r12,4)
1420199c3: 0f 28 45 c0                 	movaps	-0x40(%rbp), %xmm0
1420199c7: 0f 29 45 10                 	movaps	%xmm0, 0x10(%rbp)
1420199cb: 48 8b 45 40                 	movq	0x40(%rbp), %rax
1420199cf: 48 8b 40 28                 	movq	0x28(%rax), %rax
1420199d3: 48 39 45 48                 	cmpq	%rax, 0x48(%rbp)
1420199d7: 76 4e                       	jbe	0x142019a27 <.text+0x2018a27>
1420199d9: 48 8b 4f 38                 	movq	0x38(%rdi), %rcx
1420199dd: 48 8b 0c c1                 	movq	(%rcx,%rax,8), %rcx
1420199e1: 48 85 c9                    	testq	%rcx, %rcx
1420199e4: 74 41                       	je	0x142019a27 <.text+0x2018a27>
1420199e6: 48 f7 d1                    	notq	%rcx
1420199e9: 4c 8d 3c 49                 	leaq	(%rcx,%rcx,2), %r15
1420199ed: 49 c1 e7 04                 	shlq	$0x4, %r15
1420199f1: 4c 03 7f 18                 	addq	0x18(%rdi), %r15
1420199f5: 48 8b 8d f0 00 00 00        	movq	0xf0(%rbp), %rcx
1420199fc: 80 79 05 00                 	cmpb	$0x0, 0x5(%rcx)
142019a00: 75 35                       	jne	0x142019a37 <.text+0x2018a37>
142019a02: 45 89 ec                    	movl	%r13d, %r12d
142019a05: 4d 8b 47 08                 	movq	0x8(%r15), %r8
142019a09: 4c 89 c1                    	movq	%r8, %rcx
142019a0c: 49 0f af cc                 	imulq	%r12, %rcx
142019a10: 49 03 4f 10                 	addq	0x10(%r15), %rcx
142019a14: 48 8d 55 10                 	leaq	0x10(%rbp), %rdx
142019a18: e8 f3 a1 90 00              	callq	0x142923c10 <.text+0x2922c10>
142019a1d: 49 8b 47 20                 	movq	0x20(%r15), %rax
142019a21: 42 89 1c a0                 	movl	%ebx, (%rax,%r12,4)
142019a25: eb 67                       	jmp	0x142019a8e <.text+0x2018a8e>
142019a27: 45 31 ff                    	xorl	%r15d, %r15d
142019a2a: 48 8b 8d f0 00 00 00        	movq	0xf0(%rbp), %rcx
142019a31: 80 79 05 00                 	cmpb	$0x0, 0x5(%rcx)
142019a35: 74 cb                       	je	0x142019a02 <.text+0x2018a02>
142019a37: 80 bd 30 01 00 00 00        	cmpb	$0x0, 0x130(%rbp)
142019a3e: 74 38                       	je	0x142019a78 <.text+0x2018a78>
142019a40: 48 39 45 48                 	cmpq	%rax, 0x48(%rbp)
142019a44: 76 50                       	jbe	0x142019a96 <.text+0x2018a96>
142019a46: 48 8b 4f 38                 	movq	0x38(%rdi), %rcx
142019a4a: 48 8b 04 c1                 	movq	(%rcx,%rax,8), %rax
142019a4e: 48 85 c0                    	testq	%rax, %rax
142019a51: 74 43                       	je	0x142019a96 <.text+0x2018a96>
142019a53: 48 8b 4f 18                 	movq	0x18(%rdi), %rcx
142019a57: 48 f7 d0                    	notq	%rax
142019a5a: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
142019a5e: 48 c1 e0 04                 	shlq	$0x4, %rax
142019a62: 48 8b 44 01 18              	movq	0x18(%rcx,%rax), %rax
142019a67: 48 85 c0                    	testq	%rax, %rax
142019a6a: 74 2a                       	je	0x142019a96 <.text+0x2018a96>
142019a6c: c6 45 57 00                 	movb	$0x0, 0x57(%rbp)
142019a70: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
142019a74: ff d0                       	callq	*%rax
142019a76: eb 1e                       	jmp	0x142019a96 <.text+0x2018a96>
142019a78: 45 89 ec                    	movl	%r13d, %r12d
142019a7b: c6 45 57 00                 	movb	$0x0, 0x57(%rbp)
142019a7f: 4c 8d 45 10                 	leaq	0x10(%rbp), %r8
142019a83: 4c 89 f9                    	movq	%r15, %rcx
142019a86: 4c 89 e2                    	movq	%r12, %rdx
142019a89: e8 72 59 ce fe              	callq	0x140cff400 <.text+0xcfe400>
142019a8e: 49 8b 47 28                 	movq	0x28(%r15), %rax
142019a92: 42 89 1c a0                 	movl	%ebx, (%rax,%r12,4)
142019a96: 0f 28 45 b0                 	movaps	-0x50(%rbp), %xmm0
142019a9a: 0f 29 45 10                 	movaps	%xmm0, 0x10(%rbp)
142019a9e: 48 8b 45 40                 	movq	0x40(%rbp), %rax
142019aa2: 48 8b 40 30                 	movq	0x30(%rax), %rax
142019aa6: 48 39 45 48                 	cmpq	%rax, 0x48(%rbp)
142019aaa: 76 4e                       	jbe	0x142019afa <.text+0x2018afa>
142019aac: 48 8b 4f 38                 	movq	0x38(%rdi), %rcx
142019ab0: 48 8b 0c c1                 	movq	(%rcx,%rax,8), %rcx
142019ab4: 48 85 c9                    	testq	%rcx, %rcx
142019ab7: 74 41                       	je	0x142019afa <.text+0x2018afa>
142019ab9: 48 f7 d1                    	notq	%rcx
142019abc: 4c 8d 3c 49                 	leaq	(%rcx,%rcx,2), %r15
142019ac0: 49 c1 e7 04                 	shlq	$0x4, %r15
142019ac4: 4c 03 7f 18                 	addq	0x18(%rdi), %r15
142019ac8: 48 8b 8d f0 00 00 00        	movq	0xf0(%rbp), %rcx
142019acf: 80 79 06 00                 	cmpb	$0x0, 0x6(%rcx)
142019ad3: 75 35                       	jne	0x142019b0a <.text+0x2018b0a>
142019ad5: 45 89 ec                    	movl	%r13d, %r12d
142019ad8: 4d 8b 47 08                 	movq	0x8(%r15), %r8
142019adc: 4c 89 c1                    	movq	%r8, %rcx
142019adf: 49 0f af cc                 	imulq	%r12, %rcx
142019ae3: 49 03 4f 10                 	addq	0x10(%r15), %rcx
142019ae7: 48 8d 55 10                 	leaq	0x10(%rbp), %rdx
142019aeb: e8 20 a1 90 00              	callq	0x142923c10 <.text+0x2922c10>
142019af0: 49 8b 47 20                 	movq	0x20(%r15), %rax
142019af4: 42 89 1c a0                 	movl	%ebx, (%rax,%r12,4)
142019af8: eb 67                       	jmp	0x142019b61 <.text+0x2018b61>
142019afa: 45 31 ff                    	xorl	%r15d, %r15d
142019afd: 48 8b 8d f0 00 00 00        	movq	0xf0(%rbp), %rcx
142019b04: 80 79 06 00                 	cmpb	$0x0, 0x6(%rcx)
142019b08: 74 cb                       	je	0x142019ad5 <.text+0x2018ad5>
142019b0a: 80 bd 30 01 00 00 00        	cmpb	$0x0, 0x130(%rbp)
142019b11: 74 38                       	je	0x142019b4b <.text+0x2018b4b>
142019b13: 48 39 45 48                 	cmpq	%rax, 0x48(%rbp)
142019b17: 76 50                       	jbe	0x142019b69 <.text+0x2018b69>
142019b19: 48 8b 4f 38                 	movq	0x38(%rdi), %rcx
142019b1d: 48 8b 04 c1                 	movq	(%rcx,%rax,8), %rax
142019b21: 48 85 c0                    	testq	%rax, %rax
142019b24: 74 43                       	je	0x142019b69 <.text+0x2018b69>
142019b26: 48 8b 4f 18                 	movq	0x18(%rdi), %rcx
142019b2a: 48 f7 d0                    	notq	%rax
142019b2d: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
142019b31: 48 c1 e0 04                 	shlq	$0x4, %rax
142019b35: 48 8b 44 01 18              	movq	0x18(%rcx,%rax), %rax
142019b3a: 48 85 c0                    	testq	%rax, %rax
142019b3d: 74 2a                       	je	0x142019b69 <.text+0x2018b69>
142019b3f: c6 45 57 00                 	movb	$0x0, 0x57(%rbp)
142019b43: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
142019b47: ff d0                       	callq	*%rax
142019b49: eb 1e                       	jmp	0x142019b69 <.text+0x2018b69>
142019b4b: 45 89 ec                    	movl	%r13d, %r12d
142019b4e: c6 45 57 00                 	movb	$0x0, 0x57(%rbp)
142019b52: 4c 8d 45 10                 	leaq	0x10(%rbp), %r8
142019b56: 4c 89 f9                    	movq	%r15, %rcx
142019b59: 4c 89 e2                    	movq	%r12, %rdx
142019b5c: e8 9f 58 ce fe              	callq	0x140cff400 <.text+0xcfe400>
142019b61: 49 8b 47 28                 	movq	0x28(%r15), %rax
142019b65: 42 89 1c a0                 	movl	%ebx, (%rax,%r12,4)
142019b69: 8b 45 08                    	movl	0x8(%rbp), %eax
142019b6c: 89 45 18                    	movl	%eax, 0x18(%rbp)
142019b6f: 48 8b 45 00                 	movq	(%rbp), %rax
142019b73: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
142019b77: 48 8b 45 40                 	movq	0x40(%rbp), %rax
142019b7b: 48 8b 40 38                 	movq	0x38(%rax), %rax
142019b7f: 48 39 45 48                 	cmpq	%rax, 0x48(%rbp)
142019b83: 76 4e                       	jbe	0x142019bd3 <.text+0x2018bd3>
142019b85: 48 8b 4f 38                 	movq	0x38(%rdi), %rcx
142019b89: 48 8b 0c c1                 	movq	(%rcx,%rax,8), %rcx
142019b8d: 48 85 c9                    	testq	%rcx, %rcx
142019b90: 74 41                       	je	0x142019bd3 <.text+0x2018bd3>
142019b92: 48 f7 d1                    	notq	%rcx
142019b95: 4c 8d 3c 49                 	leaq	(%rcx,%rcx,2), %r15
142019b99: 49 c1 e7 04                 	shlq	$0x4, %r15
142019b9d: 4c 03 7f 18                 	addq	0x18(%rdi), %r15
142019ba1: 48 8b 8d f0 00 00 00        	movq	0xf0(%rbp), %rcx
142019ba8: 80 79 07 00                 	cmpb	$0x0, 0x7(%rcx)
142019bac: 75 35                       	jne	0x142019be3 <.text+0x2018be3>
142019bae: 45 89 ec                    	movl	%r13d, %r12d
142019bb1: 4d 8b 47 08                 	movq	0x8(%r15), %r8
142019bb5: 4c 89 c1                    	movq	%r8, %rcx
142019bb8: 49 0f af cc                 	imulq	%r12, %rcx
142019bbc: 49 03 4f 10                 	addq	0x10(%r15), %rcx
142019bc0: 48 8d 55 10                 	leaq	0x10(%rbp), %rdx
142019bc4: e8 47 a0 90 00              	callq	0x142923c10 <.text+0x2922c10>
142019bc9: 49 8b 47 20                 	movq	0x20(%r15), %rax
142019bcd: 42 89 1c a0                 	movl	%ebx, (%rax,%r12,4)
142019bd1: eb 67                       	jmp	0x142019c3a <.text+0x2018c3a>
142019bd3: 45 31 ff                    	xorl	%r15d, %r15d
142019bd6: 48 8b 8d f0 00 00 00        	movq	0xf0(%rbp), %rcx
142019bdd: 80 79 07 00                 	cmpb	$0x0, 0x7(%rcx)
142019be1: 74 cb                       	je	0x142019bae <.text+0x2018bae>
142019be3: 80 bd 30 01 00 00 00        	cmpb	$0x0, 0x130(%rbp)
142019bea: 74 38                       	je	0x142019c24 <.text+0x2018c24>
142019bec: 48 39 45 48                 	cmpq	%rax, 0x48(%rbp)
142019bf0: 76 50                       	jbe	0x142019c42 <.text+0x2018c42>
142019bf2: 48 8b 4f 38                 	movq	0x38(%rdi), %rcx
142019bf6: 48 8b 04 c1                 	movq	(%rcx,%rax,8), %rax
142019bfa: 48 85 c0                    	testq	%rax, %rax
142019bfd: 74 43                       	je	0x142019c42 <.text+0x2018c42>
142019bff: 48 8b 4f 18                 	movq	0x18(%rdi), %rcx
142019c03: 48 f7 d0                    	notq	%rax
142019c06: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
142019c0a: 48 c1 e0 04                 	shlq	$0x4, %rax
142019c0e: 48 8b 44 01 18              	movq	0x18(%rcx,%rax), %rax
142019c13: 48 85 c0                    	testq	%rax, %rax
142019c16: 74 2a                       	je	0x142019c42 <.text+0x2018c42>
142019c18: c6 45 57 00                 	movb	$0x0, 0x57(%rbp)
142019c1c: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
142019c20: ff d0                       	callq	*%rax
142019c22: eb 1e                       	jmp	0x142019c42 <.text+0x2018c42>
142019c24: 45 89 ec                    	movl	%r13d, %r12d
142019c27: c6 45 57 00                 	movb	$0x0, 0x57(%rbp)
142019c2b: 4c 8d 45 10                 	leaq	0x10(%rbp), %r8
142019c2f: 4c 89 f9                    	movq	%r15, %rcx
142019c32: 4c 89 e2                    	movq	%r12, %rdx
142019c35: e8 c6 57 ce fe              	callq	0x140cff400 <.text+0xcfe400>
142019c3a: 49 8b 47 28                 	movq	0x28(%r15), %rax
142019c3e: 42 89 1c a0                 	movl	%ebx, (%rax,%r12,4)
142019c42: f3 0f 11 75 10              	movss	%xmm6, 0x10(%rbp)
142019c47: 48 8b 45 40                 	movq	0x40(%rbp), %rax
142019c4b: 48 8b 40 40                 	movq	0x40(%rax), %rax
142019c4f: 48 39 45 48                 	cmpq	%rax, 0x48(%rbp)
142019c53: 76 59                       	jbe	0x142019cae <.text+0x2018cae>
142019c55: 48 8b 4f 38                 	movq	0x38(%rdi), %rcx
142019c59: 48 8b 0c c1                 	movq	(%rcx,%rax,8), %rcx
142019c5d: 48 85 c9                    	testq	%rcx, %rcx
142019c60: 0f b6 95 30 01 00 00        	movzbl	0x130(%rbp), %edx
142019c67: 0f 84 13 01 00 00           	je	0x142019d80 <.text+0x2018d80>
142019c6d: 48 f7 d1                    	notq	%rcx
142019c70: 4c 8d 34 49                 	leaq	(%rcx,%rcx,2), %r14
142019c74: 49 c1 e6 04                 	shlq	$0x4, %r14
142019c78: 4c 03 77 18                 	addq	0x18(%rdi), %r14
142019c7c: 48 8b 8d f0 00 00 00        	movq	0xf0(%rbp), %rcx
142019c83: 80 79 08 00                 	cmpb	$0x0, 0x8(%rcx)
142019c87: 75 3c                       	jne	0x142019cc5 <.text+0x2018cc5>
142019c89: 45 89 ef                    	movl	%r13d, %r15d
142019c8c: 4d 8b 46 08                 	movq	0x8(%r14), %r8
142019c90: 4c 89 c1                    	movq	%r8, %rcx
142019c93: 49 0f af cf                 	imulq	%r15, %rcx
142019c97: 49 03 4e 10                 	addq	0x10(%r14), %rcx
142019c9b: 48 8d 55 10                 	leaq	0x10(%rbp), %rdx
142019c9f: e8 6c 9f 90 00              	callq	0x142923c10 <.text+0x2922c10>
142019ca4: 49 8b 46 20                 	movq	0x20(%r14), %rax
142019ca8: 42 89 1c b8                 	movl	%ebx, (%rax,%r15,4)
142019cac: eb 69                       	jmp	0x142019d17 <.text+0x2018d17>
142019cae: 45 31 f6                    	xorl	%r14d, %r14d
142019cb1: 0f b6 95 30 01 00 00        	movzbl	0x130(%rbp), %edx
142019cb8: 48 8b 8d f0 00 00 00        	movq	0xf0(%rbp), %rcx
142019cbf: 80 79 08 00                 	cmpb	$0x0, 0x8(%rcx)
142019cc3: 74 c4                       	je	0x142019c89 <.text+0x2018c89>
142019cc5: 84 d2                       	testb	%dl, %dl
142019cc7: 74 38                       	je	0x142019d01 <.text+0x2018d01>
142019cc9: 48 39 45 48                 	cmpq	%rax, 0x48(%rbp)
142019ccd: 76 50                       	jbe	0x142019d1f <.text+0x2018d1f>
142019ccf: 48 8b 4f 38                 	movq	0x38(%rdi), %rcx
142019cd3: 48 8b 04 c1                 	movq	(%rcx,%rax,8), %rax
142019cd7: 48 85 c0                    	testq	%rax, %rax
142019cda: 74 43                       	je	0x142019d1f <.text+0x2018d1f>
142019cdc: 48 8b 4f 18                 	movq	0x18(%rdi), %rcx
142019ce0: 48 f7 d0                    	notq	%rax
142019ce3: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
142019ce7: 48 c1 e0 04                 	shlq	$0x4, %rax
142019ceb: 48 8b 44 01 18              	movq	0x18(%rcx,%rax), %rax
142019cf0: 48 85 c0                    	testq	%rax, %rax
142019cf3: 74 2a                       	je	0x142019d1f <.text+0x2018d1f>
142019cf5: c6 45 57 00                 	movb	$0x0, 0x57(%rbp)
142019cf9: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
142019cfd: ff d0                       	callq	*%rax
142019cff: eb 1e                       	jmp	0x142019d1f <.text+0x2018d1f>
142019d01: 45 89 ef                    	movl	%r13d, %r15d
142019d04: c6 45 57 00                 	movb	$0x0, 0x57(%rbp)
142019d08: 4c 8d 45 10                 	leaq	0x10(%rbp), %r8
142019d0c: 4c 89 f1                    	movq	%r14, %rcx
142019d0f: 4c 89 fa                    	movq	%r15, %rdx
142019d12: e8 e9 56 ce fe              	callq	0x140cff400 <.text+0xcfe400>
142019d17: 49 8b 46 28                 	movq	0x28(%r14), %rax
142019d1b: 42 89 1c b8                 	movl	%ebx, (%rax,%r15,4)
142019d1f: 4c 8b b5 08 01 00 00        	movq	0x108(%rbp), %r14
142019d26: 48 8b 8d 00 01 00 00        	movq	0x100(%rbp), %rcx
142019d2d: 4c 39 f1                    	cmpq	%r14, %rcx
142019d30: 74 32                       	je	0x142019d64 <.text+0x2018d64>
142019d32: 4c 8b bd 10 01 00 00        	movq	0x110(%rbp), %r15
142019d39: 0f 1f 80 00 00 00 00        	nopl	(%rax)
142019d40: 4c 8d 61 10                 	leaq	0x10(%rcx), %r12
142019d44: 4c 89 7c 24 28              	movq	%r15, 0x28(%rsp)
142019d49: 44 89 6c 24 20              	movl	%r13d, 0x20(%rsp)
142019d4e: 48 89 fa                    	movq	%rdi, %rdx
142019d51: 49 89 f0                    	movq	%rsi, %r8
142019d54: 41 89 d9                    	movl	%ebx, %r9d
142019d57: e8 34 6b d2 fe              	callq	0x140d40890 <.text+0xd3f890>
142019d5c: 4c 89 e1                    	movq	%r12, %rcx
142019d5f: 4d 39 f4                    	cmpq	%r14, %r12
142019d62: 75 dc                       	jne	0x142019d40 <.text+0x2018d40>
142019d64: 0f 28 75 60                 	movaps	0x60(%rbp), %xmm6
142019d68: 0f 28 7d 70                 	movaps	0x70(%rbp), %xmm7
142019d6c: 48 81 c4 08 01 00 00        	addq	$0x108, %rsp            # imm = 0x108
142019d73: 5b                          	popq	%rbx
142019d74: 5f                          	popq	%rdi
142019d75: 5e                          	popq	%rsi
142019d76: 41 5c                       	popq	%r12
142019d78: 41 5d                       	popq	%r13
142019d7a: 41 5e                       	popq	%r14
142019d7c: 41 5f                       	popq	%r15
142019d7e: 5d                          	popq	%rbp
142019d7f: c3                          	retq
142019d80: 45 31 f6                    	xorl	%r14d, %r14d
142019d83: 48 8b 8d f0 00 00 00        	movq	0xf0(%rbp), %rcx
142019d8a: 80 79 08 00                 	cmpb	$0x0, 0x8(%rcx)
142019d8e: 0f 85 31 ff ff ff           	jne	0x142019cc5 <.text+0x2018cc5>
142019d94: e9 f0 fe ff ff              	jmp	0x142019c89 <.text+0x2018c89>
142019d99: 0f 1f 80 00 00 00 00        	nopl	(%rax)
142019da0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
142019da5: 55                          	pushq	%rbp
142019da6: 41 57                       	pushq	%r15
142019da8: 41 56                       	pushq	%r14
142019daa: 41 55                       	pushq	%r13
142019dac: 41 54                       	pushq	%r12
142019dae: 56                          	pushq	%rsi
142019daf: 57                          	pushq	%rdi
142019db0: 53                          	pushq	%rbx
142019db1: 48 83 ec 58                 	subq	$0x58, %rsp
142019db5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
142019dbc: 0f 29 7c 24 30              	movaps	%xmm7, 0x30(%rsp)
142019dc1: 0f 29 74 24 40              	movaps	%xmm6, 0x40(%rsp)
142019dc6: 80 7d 57 00                 	cmpb	$0x0, 0x57(%rbp)
142019dca: 75 1b                       	jne	0x142019de7 <.text+0x2018de7>
142019dcc: 0f 28 74 24 40              	movaps	0x40(%rsp), %xmm6
142019dd1: 0f 28 7c 24 30              	movaps	0x30(%rsp), %xmm7
142019dd6: 48 83 c4 58                 	addq	$0x58, %rsp
142019dda: 5b                          	popq	%rbx
142019ddb: 5f                          	popq	%rdi
142019ddc: 5e                          	popq	%rsi
142019ddd: 41 5c                       	popq	%r12
142019ddf: 41 5d                       	popq	%r13
142019de1: 41 5e                       	popq	%r14
142019de3: 41 5f                       	popq	%r15
142019de5: 5d                          	popq	%rbp
142019de6: c3                          	retq
142019de7: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
142019deb: e8 90 08 13 fe              	callq	0x14014a680 <.text+0x149680>
142019df0: eb da                       	jmp	0x142019dcc <.text+0x2018dcc>
