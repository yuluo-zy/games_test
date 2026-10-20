
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14146f6d8: 48 8b 8d 18 02 00 00        	movq	0x218(%rbp), %rcx
14146f6df: 48 85 c9                    	testq	%rcx, %rcx
14146f6e2: 0f 84 b6 00 00 00           	je	0x14146f79e <.text+0x146e79e>
14146f6e8: 48 8b 85 10 02 00 00        	movq	0x210(%rbp), %rax
14146f6ef: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
14146f6f3: 48 8d 0c c8                 	leaq	(%rax,%rcx,8), %rcx
14146f6f7: 48 8d 15 ce 55 7f 01        	leaq	0x17f55ce(%rip), %rdx   # 0x142c64ccc
14146f6fe: eb 0d                       	jmp	0x14146f70d <.text+0x146e70d>
14146f700: 48 83 c0 28                 	addq	$0x28, %rax
14146f704: 48 39 c8                    	cmpq	%rcx, %rax
14146f707: 0f 84 91 00 00 00           	je	0x14146f79e <.text+0x146e79e>
14146f70d: 4c 8b 48 20                 	movq	0x20(%rax), %r9
14146f711: 4d 85 c9                    	testq	%r9, %r9
14146f714: 74 ea                       	je	0x14146f700 <.text+0x146e700>
14146f716: 4c 8b 40 18                 	movq	0x18(%rax), %r8
14146f71a: 4d 6b c9 58                 	imulq	$0x58, %r9, %r9
14146f71e: 45 31 d2                    	xorl	%r10d, %r10d
14146f721: eb 1e                       	jmp	0x14146f741 <.text+0x146e741>
14146f723: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
14146f730: 41 89 f3                    	movl	%esi, %r11d
14146f733: 47 89 5c 10 30              	movl	%r11d, 0x30(%r8,%r10)
14146f738: 49 83 c2 58                 	addq	$0x58, %r10
14146f73c: 4d 39 d1                    	cmpq	%r10, %r9
14146f73f: 74 bf                       	je	0x14146f700 <.text+0x146e700>
14146f741: 47 0f b6 5c 10 52           	movzbl	0x52(%r8,%r10), %r11d
14146f747: 41 80 c3 c8                 	addb	$-0x38, %r11b
14146f74b: 41 80 fb 02                 	cmpb	$0x2, %r11b
14146f74f: 77 e7                       	ja	0x14146f738 <.text+0x146e738>
14146f751: 43 8b 74 10 30              	movl	0x30(%r8,%r10), %esi
14146f756: 48 83 fe 08                 	cmpq	$0x8, %rsi
14146f75a: 77 12                       	ja	0x14146f76e <.text+0x146e76e>
14146f75c: 4c 63 1c b2                 	movslq	(%rdx,%rsi,4), %r11
14146f760: 49 01 d3                    	addq	%rdx, %r11
14146f763: 41 ff e3                    	jmpq	*%r11
14146f766: 41 bb 02 00 00 00           	movl	$0x2, %r11d
14146f76c: eb c5                       	jmp	0x14146f733 <.text+0x146e733>
14146f76e: 41 bb 01 00 00 00           	movl	$0x1, %r11d
14146f774: 81 fe e7 03 00 00           	cmpl	$0x3e7, %esi            # imm = 0x3E7
14146f77a: 74 b4                       	je	0x14146f730 <.text+0x146e730>
14146f77c: eb b5                       	jmp	0x14146f733 <.text+0x146e733>
14146f77e: 41 bb 03 00 00 00           	movl	$0x3, %r11d
14146f784: eb ad                       	jmp	0x14146f733 <.text+0x146e733>
14146f786: 41 bb 05 00 00 00           	movl	$0x5, %r11d
14146f78c: eb a5                       	jmp	0x14146f733 <.text+0x146e733>
14146f78e: 41 bb 08 00 00 00           	movl	$0x8, %r11d
14146f794: eb 9d                       	jmp	0x14146f733 <.text+0x146e733>
14146f796: 41 bb 04 00 00 00           	movl	$0x4, %r11d
14146f79c: eb 95                       	jmp	0x14146f733 <.text+0x146e733>
14146f79e: 4c 8d a5 60 03 00 00        	leaq	0x360(%rbp), %r12
14146f7a5: 48 8d 95 b0 01 00 00        	leaq	0x1b0(%rbp), %rdx
14146f7ac: 41 b8 b0 01 00 00           	movl	$0x1b0, %r8d            # imm = 0x1B0
14146f7b2: 4c 89 e1                    	movq	%r12, %rcx
14146f7b5: e8 46 44 4b 01              	callq	0x142923c00 <.text+0x2922c00>
14146f7ba: 4c 8d b5 b0 05 00 00        	leaq	0x5b0(%rbp), %r14
14146f7c1: 41 b8 80 02 00 00           	movl	$0x280, %r8d            # imm = 0x280
14146f7c7: 4c 89 f1                    	movq	%r14, %rcx
14146f7ca: 4c 89 fa                    	movq	%r15, %rdx
14146f7cd: e8 2e 44 4b 01              	callq	0x142923c00 <.text+0x2922c00>
14146f7d2: c6 85 73 09 00 00 00        	movb	$0x0, 0x973(%rbp)
14146f7d9: 4c 89 e1                    	movq	%r12, %rcx
14146f7dc: 48 8b 95 80 05 00 00        	movq	0x580(%rbp), %rdx
14146f7e3: 4d 89 f0                    	movq	%r14, %r8
14146f7e6: e8 65 a4 33 00              	callq	0x1417a9c50 <.text+0x17a8c50>
14146f7eb: 48 0f ba a5 10 09 00 00 28  	btq	$0x28, 0x910(%rbp)
14146f7f4: 48 8b 85 18 09 00 00        	movq	0x918(%rbp), %rax
14146f7fb: 89 78 68                    	movl	%edi, 0x68(%rax)
14146f7fe: 48 89 c7                    	movq	%rax, %rdi
14146f801: 0f 83 ad e7 ff ff           	jae	0x14146dfb4 <.text+0x146cfb4>
14146f807: 48 8b 47 20                 	movq	0x20(%rdi), %rax
14146f80b: 48 89 85 28 09 00 00        	movq	%rax, 0x928(%rbp)
14146f812: 48 8b 47 28                 	movq	0x28(%rdi), %rax
14146f816: 48 c7 47 28 00 00 00 00     	movq	$0x0, 0x28(%rdi)
14146f81e: 48 89 85 38 09 00 00        	movq	%rax, 0x938(%rbp)
14146f825: 48 85 c0                    	testq	%rax, %rax
14146f828: 0f 84 86 e7 ff ff           	je	0x14146dfb4 <.text+0x146cfb4>
14146f82e: 48 8b b5 38 09 00 00        	movq	0x938(%rbp), %rsi
14146f835: 48 f7 de                    	negq	%rsi
14146f838: b9 01 00 00 00              	movl	$0x1, %ecx
14146f83d: 4c 8b bd 28 09 00 00        	movq	0x928(%rbp), %r15
14146f844: eb 1f                       	jmp	0x14146f865 <.text+0x146e865>
14146f846: 48 8b 8d 58 09 00 00        	movq	0x958(%rbp), %rcx
14146f84d: 48 8d 04 0e                 	leaq	(%rsi,%rcx), %rax
14146f851: 48 ff c0                    	incq	%rax
14146f854: 48 ff c1                    	incq	%rcx
14146f857: 49 83 c7 10                 	addq	$0x10, %r15
14146f85b: 48 83 f8 01                 	cmpq	$0x1, %rax
14146f85f: 0f 84 4f e7 ff ff           	je	0x14146dfb4 <.text+0x146cfb4>
14146f865: 48 89 8d 58 09 00 00        	movq	%rcx, 0x958(%rbp)
14146f86c: 49 8b 07                    	movq	(%r15), %rax
14146f86f: f0                          	lock
14146f870: 48 ff 08                    	decq	(%rax)
14146f873: 75 d1                       	jne	0x14146f846 <.text+0x146e846>
14146f875: 4c 89 f9                    	movq	%r15, %rcx
14146f878: e8 03 65 e7 fe              	callq	0x1402e5d80 <.text+0x2e4d80>
14146f87d: eb c7                       	jmp	0x14146f846 <.text+0x146e846>
14146f87f: 8b 85 b8 01 00 00           	movl	0x1b8(%rbp), %eax
14146f885: 8b 8d bb 01 00 00           	movl	0x1bb(%rbp), %ecx
14146f88b: 89 8d f3 08 00 00           	movl	%ecx, 0x8f3(%rbp)
14146f891: 89 85 f0 08 00 00           	movl	%eax, 0x8f0(%rbp)
14146f897: 80 fb 02                    	cmpb	$0x2, %bl
14146f89a: 0f 84 ee e8 ff ff           	je	0x14146e18e <.text+0x146d18e>
14146f8a0: e9 a4 f1 ff ff              	jmp	0x14146ea49 <.text+0x146da49>
14146f8a5: 48 8d 0d e4 47 7f 01        	leaq	0x17f47e4(%rip), %rcx   # 0x142c64090
14146f8ac: 4c 8d 05 35 48 7f 01        	leaq	0x17f4835(%rip), %r8    # 0x142c640e8
14146f8b3: ba 58 00 00 00              	movl	$0x58, %edx
14146f8b8: e8 73 9b 46 01              	callq	0x1428d9430 <.text+0x28d8430>
14146f8bd: 48 8b 85 b8 05 00 00        	movq	0x5b8(%rbp), %rax
14146f8c4: 48 89 85 68 03 00 00        	movq	%rax, 0x368(%rbp)
14146f8cb: 48 c7 85 60 03 00 00 02 00 00 00    	movq	$0x2, 0x360(%rbp)
