
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a520b0: 55                          	pushq	%rbp
140a520b1: 41 57                       	pushq	%r15
140a520b3: 41 56                       	pushq	%r14
140a520b5: 41 55                       	pushq	%r13
140a520b7: 41 54                       	pushq	%r12
140a520b9: 56                          	pushq	%rsi
140a520ba: 57                          	pushq	%rdi
140a520bb: 53                          	pushq	%rbx
140a520bc: 48 81 ec 38 01 00 00        	subq	$0x138, %rsp            # imm = 0x138
140a520c3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140a520cb: 48 c7 85 b0 00 00 00 fe ff ff ff    	movq	$-0x2, 0xb0(%rbp)
140a520d6: 0f 10 41 10                 	movups	0x10(%rcx), %xmm0
140a520da: 0f 10 49 20                 	movups	0x20(%rcx), %xmm1
140a520de: 0f 10 51 30                 	movups	0x30(%rcx), %xmm2
140a520e2: 0f 29 45 00                 	movaps	%xmm0, (%rbp)
140a520e6: 48 89 8d a8 00 00 00        	movq	%rcx, 0xa8(%rbp)
140a520ed: 48 8b 41 40                 	movq	0x40(%rcx), %rax
140a520f1: 48 89 45 30                 	movq	%rax, 0x30(%rbp)
140a520f5: 0f 29 55 20                 	movaps	%xmm2, 0x20(%rbp)
140a520f9: 0f 29 4d 10                 	movaps	%xmm1, 0x10(%rbp)
140a520fd: 48 8d 75 30                 	leaq	0x30(%rbp), %rsi
140a52101: 48 8b 7d 00                 	movq	(%rbp), %rdi
140a52105: 66 0f 6f 07                 	movdqa	(%rdi), %xmm0
140a52109: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
140a5210d: f7 d0                       	notl	%eax
140a5210f: 48 8d 5f 10                 	leaq	0x10(%rdi), %rbx
140a52113: 4c 8b 75 18                 	movq	0x18(%rbp), %r14
140a52117: eb 2c                       	jmp	0x140a52145 <.text+0xa51145>
140a52119: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140a52120: 44 8d 78 ff                 	leal	-0x1(%rax), %r15d
140a52124: 41 21 c7                    	andl	%eax, %r15d
140a52127: f3 0f bc c0                 	tzcntl	%eax, %eax
140a5212b: 48 f7 d8                    	negq	%rax
140a5212e: 48 8d 04 c0                 	leaq	(%rax,%rax,8), %rax
140a52132: 48 8d 0c c7                 	leaq	(%rdi,%rax,8), %rcx
140a52136: 48 83 c1 d0                 	addq	$-0x30, %rcx
140a5213a: 48 89 f2                    	movq	%rsi, %rdx
140a5213d: e8 fe 14 00 00              	callq	0x140a53640 <.text+0xa52640>
140a52142: 44 89 f8                    	movl	%r15d, %eax
140a52145: 4c 89 75 78                 	movq	%r14, 0x78(%rbp)
140a52149: 49 83 ee 01                 	subq	$0x1, %r14
140a5214d: 72 3c                       	jb	0x140a5218b <.text+0xa5118b>
140a5214f: 66 85 c0                    	testw	%ax, %ax
140a52152: 75 cc                       	jne	0x140a52120 <.text+0xa51120>
140a52154: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140a52160: 66 0f 6f 03                 	movdqa	(%rbx), %xmm0
140a52164: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
140a52168: 48 81 c7 80 fb ff ff        	addq	$-0x480, %rdi           # imm = 0xFB80
140a5216f: 48 83 c3 10                 	addq	$0x10, %rbx
140a52173: 3d ff ff 00 00              	cmpl	$0xffff, %eax           # imm = 0xFFFF
140a52178: 74 e6                       	je	0x140a52160 <.text+0xa51160>
140a5217a: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
140a5217f: 29 c1                       	subl	%eax, %ecx
140a52181: f7 d0                       	notl	%eax
140a52183: 41 89 c7                    	movl	%eax, %r15d
140a52186: 41 21 cf                    	andl	%ecx, %r15d
140a52189: eb 9c                       	jmp	0x140a52127 <.text+0xa51127>
140a5218b: 4c 8b 65 00                 	movq	(%rbp), %r12
140a5218f: 4c 8b 7d 08                 	movq	0x8(%rbp), %r15
140a52193: 4c 8b 6d 18                 	movq	0x18(%rbp), %r13
140a52197: 49 8d 4f 01                 	leaq	0x1(%r15), %rcx
140a5219b: 66 41 0f 6f 04 24           	movdqa	(%r12), %xmm0
140a521a1: 4d 85 ff                    	testq	%r15, %r15
140a521a4: 74 29                       	je	0x140a521cf <.text+0xa511cf>
140a521a6: ba 48 00 00 00              	movl	$0x48, %edx
140a521ab: 48 89 c8                    	movq	%rcx, %rax
140a521ae: 48 f7 e2                    	mulq	%rdx
140a521b1: 48 83 c0 0f                 	addq	$0xf, %rax
140a521b5: 48 83 e0 f0                 	andq	$-0x10, %rax
140a521b9: 49 8d 14 07                 	leaq	(%r15,%rax), %rdx
140a521bd: 48 83 c2 11                 	addq	$0x11, %rdx
140a521c1: 4d 89 e1                    	movq	%r12, %r9
140a521c4: 49 29 c1                    	subq	%rax, %r9
140a521c7: 41 b8 10 00 00 00           	movl	$0x10, %r8d
140a521cd: eb 03                       	jmp	0x140a521d2 <.text+0xa511d2>
140a521cf: 45 31 c0                    	xorl	%r8d, %r8d
140a521d2: 49 8d 7c 24 10              	leaq	0x10(%r12), %rdi
140a521d7: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
140a521db: f7 d0                       	notl	%eax
140a521dd: 4c 01 e1                    	addq	%r12, %rcx
140a521e0: 4c 89 85 88 00 00 00        	movq	%r8, 0x88(%rbp)
140a521e7: 4c 89 45 38                 	movq	%r8, 0x38(%rbp)
140a521eb: 48 89 95 80 00 00 00        	movq	%rdx, 0x80(%rbp)
140a521f2: 48 89 55 40                 	movq	%rdx, 0x40(%rbp)
140a521f6: 4c 89 8d 90 00 00 00        	movq	%r9, 0x90(%rbp)
140a521fd: 4c 89 4d 48                 	movq	%r9, 0x48(%rbp)
140a52201: 4c 89 65 50                 	movq	%r12, 0x50(%rbp)
140a52205: 48 89 7d 58                 	movq	%rdi, 0x58(%rbp)
140a52209: 48 89 4d 60                 	movq	%rcx, 0x60(%rbp)
140a5220d: 66 89 45 68                 	movw	%ax, 0x68(%rbp)
140a52211: 4c 89 6d 70                 	movq	%r13, 0x70(%rbp)
140a52215: 4d 85 ed                    	testq	%r13, %r13
140a52218: 0f 84 c3 01 00 00           	je	0x140a523e1 <.text+0xa513e1>
140a5221e: 48 8d 5d a8                 	leaq	-0x58(%rbp), %rbx
140a52222: 4c 8d 75 a0                 	leaq	-0x60(%rbp), %r14
140a52226: eb 13                       	jmp	0x140a5223b <.text+0xa5123b>
140a52228: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
140a52230: 89 f0                       	movl	%esi, %eax
140a52232: 4d 85 ed                    	testq	%r13, %r13
140a52235: 0f 84 a6 01 00 00           	je	0x140a523e1 <.text+0xa513e1>
140a5223b: 66 85 c0                    	testw	%ax, %ax
140a5223e: 74 10                       	je	0x140a52250 <.text+0xa51250>
140a52240: 8d 70 ff                    	leal	-0x1(%rax), %esi
140a52243: 21 c6                       	andl	%eax, %esi
140a52245: eb 38                       	jmp	0x140a5227f <.text+0xa5127f>
140a52247: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140a52250: 66 0f 6f 07                 	movdqa	(%rdi), %xmm0
140a52254: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
140a52258: 49 81 c4 80 fb ff ff        	addq	$-0x480, %r12           # imm = 0xFB80
140a5225f: 48 83 c7 10                 	addq	$0x10, %rdi
140a52263: 3d ff ff 00 00              	cmpl	$0xffff, %eax           # imm = 0xFFFF
140a52268: 74 e6                       	je	0x140a52250 <.text+0xa51250>
140a5226a: 48 89 7d 58                 	movq	%rdi, 0x58(%rbp)
140a5226e: 4c 89 65 50                 	movq	%r12, 0x50(%rbp)
140a52272: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
140a52277: 29 c1                       	subl	%eax, %ecx
140a52279: f7 d0                       	notl	%eax
140a5227b: 89 c6                       	movl	%eax, %esi
140a5227d: 21 ce                       	andl	%ecx, %esi
140a5227f: 66 89 75 68                 	movw	%si, 0x68(%rbp)
140a52283: 49 ff cd                    	decq	%r13
140a52286: 4c 89 6d 70                 	movq	%r13, 0x70(%rbp)
140a5228a: f3 0f bc c0                 	tzcntl	%eax, %eax
140a5228e: 48 f7 d8                    	negq	%rax
140a52291: 48 8d 0c c0                 	leaq	(%rax,%rax,8), %rcx
140a52295: 4d 8b 44 cc b8              	movq	-0x48(%r12,%rcx,8), %r8
140a5229a: 4d 8b 4c cc c0              	movq	-0x40(%r12,%rcx,8), %r9
140a5229f: 49 8b 44 cc d0              	movq	-0x30(%r12,%rcx,8), %rax
140a522a4: 49 8b 54 cc f8              	movq	-0x8(%r12,%rcx,8), %rdx
140a522a9: 48 89 55 f0                 	movq	%rdx, -0x10(%rbp)
140a522ad: 41 0f 10 44 cc e8           	movups	-0x18(%r12,%rcx,8), %xmm0
140a522b3: 0f 29 45 e0                 	movaps	%xmm0, -0x20(%rbp)
140a522b7: 41 0f 10 44 cc d8           	movups	-0x28(%r12,%rcx,8), %xmm0
140a522bd: 0f 29 45 d0                 	movaps	%xmm0, -0x30(%rbp)
140a522c1: 48 83 f8 02                 	cmpq	$0x2, %rax
140a522c5: 74 72                       	je	0x140a52339 <.text+0xa51339>
140a522c7: 4c 89 8d 98 00 00 00        	movq	%r9, 0x98(%rbp)
140a522ce: 4c 89 85 a0 00 00 00        	movq	%r8, 0xa0(%rbp)
140a522d5: 48 89 45 a0                 	movq	%rax, -0x60(%rbp)
140a522d9: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
140a522dd: 48 89 43 20                 	movq	%rax, 0x20(%rbx)
140a522e1: 66 0f 6f 45 d0              	movdqa	-0x30(%rbp), %xmm0
140a522e6: 0f 28 4d e0                 	movaps	-0x20(%rbp), %xmm1
140a522ea: 0f 11 4b 10                 	movups	%xmm1, 0x10(%rbx)
140a522ee: f3 0f 7f 03                 	movdqu	%xmm0, (%rbx)
140a522f2: 4c 89 f1                    	movq	%r14, %rcx
140a522f5: e8 76 15 00 00              	callq	0x140a53870 <.text+0xa52870>
140a522fa: 48 8b 55 b0                 	movq	-0x50(%rbp), %rdx
140a522fe: 48 85 d2                    	testq	%rdx, %rdx
140a52301: 74 0f                       	je	0x140a52312 <.text+0xa51312>
140a52303: 48 8b 4d b8                 	movq	-0x48(%rbp), %rcx
140a52307: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140a5230d: e8 0e 19 bc ff              	callq	0x140613c20 <.text+0x612c20>
140a52312: 48 8b 95 a0 00 00 00        	movq	0xa0(%rbp), %rdx
140a52319: 48 85 d2                    	testq	%rdx, %rdx
140a5231c: 0f 84 0e ff ff ff           	je	0x140a52230 <.text+0xa51230>
140a52322: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140a52328: 48 8b 8d 98 00 00 00        	movq	0x98(%rbp), %rcx
140a5232f: e8 ec 18 bc ff              	callq	0x140613c20 <.text+0x612c20>
140a52334: e9 f7 fe ff ff              	jmp	0x140a52230 <.text+0xa51230>
140a52339: 4d 85 ed                    	testq	%r13, %r13
140a5233c: 75 1d                       	jne	0x140a5235b <.text+0xa5135b>
140a5233e: e9 9e 00 00 00              	jmp	0x140a523e1 <.text+0xa513e1>
140a52343: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140a52350: 89 de                       	movl	%ebx, %esi
140a52352: 49 ff cd                    	decq	%r13
140a52355: 0f 84 82 00 00 00           	je	0x140a523dd <.text+0xa513dd>
140a5235b: 66 85 f6                    	testw	%si, %si
140a5235e: 74 10                       	je	0x140a52370 <.text+0xa51370>
140a52360: 8d 5e ff                    	leal	-0x1(%rsi), %ebx
140a52363: 21 f3                       	andl	%esi, %ebx
140a52365: eb 31                       	jmp	0x140a52398 <.text+0xa51398>
140a52367: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140a52370: 66 0f 6f 07                 	movdqa	(%rdi), %xmm0
140a52374: 66 0f d7 f0                 	pmovmskb	%xmm0, %esi
140a52378: 49 81 c4 80 fb ff ff        	addq	$-0x480, %r12           # imm = 0xFB80
140a5237f: 48 83 c7 10                 	addq	$0x10, %rdi
140a52383: 81 fe ff ff 00 00           	cmpl	$0xffff, %esi           # imm = 0xFFFF
140a52389: 74 e5                       	je	0x140a52370 <.text+0xa51370>
140a5238b: b8 fe ff ff ff              	movl	$0xfffffffe, %eax       # imm = 0xFFFFFFFE
140a52390: 29 f0                       	subl	%esi, %eax
140a52392: f7 d6                       	notl	%esi
140a52394: 89 f3                       	movl	%esi, %ebx
140a52396: 21 c3                       	andl	%eax, %ebx
140a52398: f3 0f bc c6                 	tzcntl	%esi, %eax
140a5239c: 48 f7 d8                    	negq	%rax
140a5239f: 48 8d 04 c0                 	leaq	(%rax,%rax,8), %rax
140a523a3: 49 8d 34 c4                 	leaq	(%r12,%rax,8), %rsi
140a523a7: 49 8b 54 c4 b8              	movq	-0x48(%r12,%rax,8), %rdx
140a523ac: 48 85 d2                    	testq	%rdx, %rdx
140a523af: 74 0f                       	je	0x140a523c0 <.text+0xa513c0>
140a523b1: 48 8b 4e c0                 	movq	-0x40(%rsi), %rcx
140a523b5: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140a523bb: e8 60 18 bc ff              	callq	0x140613c20 <.text+0x612c20>
140a523c0: 48 8b 56 e0                 	movq	-0x20(%rsi), %rdx
140a523c4: 48 85 d2                    	testq	%rdx, %rdx
140a523c7: 74 87                       	je	0x140a52350 <.text+0xa51350>
140a523c9: 48 8b 4e e8                 	movq	-0x18(%rsi), %rcx
140a523cd: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140a523d3: e8 48 18 bc ff              	callq	0x140613c20 <.text+0x612c20>
140a523d8: e9 73 ff ff ff              	jmp	0x140a52350 <.text+0xa51350>
140a523dd: 66 89 5d 68                 	movw	%bx, 0x68(%rbp)
140a523e1: 4d 85 ff                    	testq	%r15, %r15
140a523e4: 48 8b 95 80 00 00 00        	movq	0x80(%rbp), %rdx
140a523eb: 74 18                       	je	0x140a52405 <.text+0xa51405>
140a523ed: 48 85 d2                    	testq	%rdx, %rdx
140a523f0: 74 13                       	je	0x140a52405 <.text+0xa51405>
140a523f2: 48 8b 8d 90 00 00 00        	movq	0x90(%rbp), %rcx
140a523f9: 4c 8b 85 88 00 00 00        	movq	0x88(%rbp), %r8
140a52400: e8 1b 18 bc ff              	callq	0x140613c20 <.text+0x612c20>
140a52405: 48 8b 8d a8 00 00 00        	movq	0xa8(%rbp), %rcx
140a5240c: 48 8b 01                    	movq	(%rcx), %rax
140a5240f: 48 8b 49 08                 	movq	0x8(%rcx), %rcx
140a52413: 48 85 c0                    	testq	%rax, %rax
140a52416: 74 12                       	je	0x140a5242a <.text+0xa5142a>
140a52418: 83 f8 01                    	cmpl	$0x1, %eax
140a5241b: 75 1a                       	jne	0x140a52437 <.text+0xa51437>
140a5241d: 41 b8 00 ca 9a 3b           	movl	$0x3b9aca00, %r8d       # imm = 0x3B9ACA00
140a52423: e8 48 38 c2 00              	callq	0x141675c70 <.text+0x1674c70>
140a52428: eb 18                       	jmp	0x140a52442 <.text+0xa51442>
140a5242a: 41 b8 00 ca 9a 3b           	movl	$0x3b9aca00, %r8d       # imm = 0x3B9ACA00
140a52430: e8 7b 95 c2 00              	callq	0x14167b9b0 <.text+0x167a9b0>
140a52435: eb 0b                       	jmp	0x140a52442 <.text+0xa51442>
140a52437: 41 b8 00 ca 9a 3b           	movl	$0x3b9aca00, %r8d       # imm = 0x3B9ACA00
140a5243d: e8 ce 03 c8 00              	callq	0x1416d2810 <.text+0x16d1810>
140a52442: 3c 02                       	cmpb	$0x2, %al
140a52444: 74 04                       	je	0x140a5244a <.text+0xa5144a>
140a52446: a8 01                       	testb	$0x1, %al
140a52448: 74 21                       	je	0x140a5246b <.text+0xa5146b>
140a5244a: 48 8b 8d a8 00 00 00        	movq	0xa8(%rbp), %rcx
140a52451: e8 7a 82 e2 ff              	callq	0x14087a6d0 <.text+0x8796d0>
140a52456: 90                          	nop
140a52457: 48 81 c4 38 01 00 00        	addq	$0x138, %rsp            # imm = 0x138
140a5245e: 5b                          	popq	%rbx
140a5245f: 5f                          	popq	%rdi
140a52460: 5e                          	popq	%rsi
140a52461: 41 5c                       	popq	%r12
140a52463: 41 5d                       	popq	%r13
140a52465: 41 5e                       	popq	%r14
140a52467: 41 5f                       	popq	%r15
140a52469: 5d                          	popq	%rbp
140a5246a: c3                          	retq
140a5246b: 48 8d 0d ee 20 05 02        	leaq	0x20520ee(%rip), %rcx   # 0x142aa4560
140a52472: 4c 8d 05 5f 21 05 02        	leaq	0x205215f(%rip), %r8    # 0x142aa45d8
140a52479: ba 28 00 00 00              	movl	$0x28, %edx
140a5247e: e8 ad 6f e8 01              	callq	0x1428d9430 <.text+0x28d8430>
140a52483: 0f 0b                       	ud2
140a52485: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140a52490: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a52495: 55                          	pushq	%rbp
140a52496: 41 57                       	pushq	%r15
140a52498: 41 56                       	pushq	%r14
140a5249a: 41 55                       	pushq	%r13
140a5249c: 41 54                       	pushq	%r12
140a5249e: 56                          	pushq	%rsi
140a5249f: 57                          	pushq	%rdi
140a524a0: 53                          	pushq	%rbx
140a524a1: 48 83 ec 28                 	subq	$0x28, %rsp
140a524a5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a524ac: 48 8b 55 b0                 	movq	-0x50(%rbp), %rdx
140a524b0: 48 85 d2                    	testq	%rdx, %rdx
140a524b3: 74 0f                       	je	0x140a524c4 <.text+0xa514c4>
140a524b5: 48 8b 4d b8                 	movq	-0x48(%rbp), %rcx
140a524b9: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140a524bf: e8 5c 17 bc ff              	callq	0x140613c20 <.text+0x612c20>
140a524c4: 48 83 bd a0 00 00 00 00     	cmpq	$0x0, 0xa0(%rbp)
140a524cc: 74 19                       	je	0x140a524e7 <.text+0xa514e7>
140a524ce: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140a524d4: 48 8b 8d 98 00 00 00        	movq	0x98(%rbp), %rcx
140a524db: 48 8b 95 a0 00 00 00        	movq	0xa0(%rbp), %rdx
140a524e2: e8 39 17 bc ff              	callq	0x140613c20 <.text+0x612c20>
140a524e7: 48 8d 4d 38                 	leaq	0x38(%rbp), %rcx
140a524eb: e8 90 bb e2 ff              	callq	0x14087e080 <.text+0x87d080>
140a524f0: 90                          	nop
140a524f1: 48 83 c4 28                 	addq	$0x28, %rsp
140a524f5: 5b                          	popq	%rbx
140a524f6: 5f                          	popq	%rdi
140a524f7: 5e                          	popq	%rsi
140a524f8: 41 5c                       	popq	%r12
140a524fa: 41 5d                       	popq	%r13
140a524fc: 41 5e                       	popq	%r14
140a524fe: 41 5f                       	popq	%r15
140a52500: 5d                          	popq	%rbp
140a52501: c3                          	retq
140a52502: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140a52510: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a52515: 55                          	pushq	%rbp
140a52516: 41 57                       	pushq	%r15
140a52518: 41 56                       	pushq	%r14
140a5251a: 41 55                       	pushq	%r13
140a5251c: 41 54                       	pushq	%r12
140a5251e: 56                          	pushq	%rsi
140a5251f: 57                          	pushq	%rdi
140a52520: 53                          	pushq	%rbx
140a52521: 48 83 ec 28                 	subq	$0x28, %rsp
140a52525: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a5252c: 48 83 7d 78 00              	cmpq	$0x0, 0x78(%rbp)
140a52531: 74 18                       	je	0x140a5254b <.text+0xa5154b>
140a52533: 48 8d 55 20                 	leaq	0x20(%rbp), %rdx
140a52537: 48 89 e9                    	movq	%rbp, %rcx
140a5253a: 41 b8 48 00 00 00           	movl	$0x48, %r8d
140a52540: 41 b9 10 00 00 00           	movl	$0x10, %r9d
140a52546: e8 15 2f 12 00              	callq	0x140b75460 <.text+0xb74460>
140a5254b: 90                          	nop
140a5254c: 48 83 c4 28                 	addq	$0x28, %rsp
140a52550: 5b                          	popq	%rbx
140a52551: 5f                          	popq	%rdi
140a52552: 5e                          	popq	%rsi
140a52553: 41 5c                       	popq	%r12
140a52555: 41 5d                       	popq	%r13
140a52557: 41 5e                       	popq	%r14
140a52559: 41 5f                       	popq	%r15
140a5255b: 5d                          	popq	%rbp
140a5255c: c3                          	retq
140a5255d: 0f 1f 00                    	nopl	(%rax)
140a52560: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a52565: 55                          	pushq	%rbp
140a52566: 41 57                       	pushq	%r15
140a52568: 41 56                       	pushq	%r14
140a5256a: 41 55                       	pushq	%r13
140a5256c: 41 54                       	pushq	%r12
140a5256e: 56                          	pushq	%rsi
140a5256f: 57                          	pushq	%rdi
140a52570: 53                          	pushq	%rbx
140a52571: 48 83 ec 28                 	subq	$0x28, %rsp
140a52575: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a5257c: 48 8b 8d a8 00 00 00        	movq	0xa8(%rbp), %rcx
140a52583: e8 48 81 e2 ff              	callq	0x14087a6d0 <.text+0x8796d0>
140a52588: 90                          	nop
140a52589: 48 83 c4 28                 	addq	$0x28, %rsp
140a5258d: 5b                          	popq	%rbx
140a5258e: 5f                          	popq	%rdi
140a5258f: 5e                          	popq	%rsi
140a52590: 41 5c                       	popq	%r12
140a52592: 41 5d                       	popq	%r13
140a52594: 41 5e                       	popq	%r14
140a52596: 41 5f                       	popq	%r15
140a52598: 5d                          	popq	%rbp
140a52599: c3                          	retq
