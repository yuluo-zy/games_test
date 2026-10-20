
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140790400: 55                          	pushq	%rbp
140790401: 41 57                       	pushq	%r15
140790403: 41 56                       	pushq	%r14
140790405: 41 55                       	pushq	%r13
140790407: 41 54                       	pushq	%r12
140790409: 56                          	pushq	%rsi
14079040a: 57                          	pushq	%rdi
14079040b: 53                          	pushq	%rbx
14079040c: 48 81 ec 48 02 00 00        	subq	$0x248, %rsp            # imm = 0x248
140790413: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14079041b: 44 0f 29 a5 b0 01 00 00     	movaps	%xmm12, 0x1b0(%rbp)
140790423: 44 0f 29 9d a0 01 00 00     	movaps	%xmm11, 0x1a0(%rbp)
14079042b: 44 0f 29 95 90 01 00 00     	movaps	%xmm10, 0x190(%rbp)
140790433: 44 0f 29 8d 80 01 00 00     	movaps	%xmm9, 0x180(%rbp)
14079043b: 44 0f 29 85 70 01 00 00     	movaps	%xmm8, 0x170(%rbp)
140790443: 0f 29 bd 60 01 00 00        	movaps	%xmm7, 0x160(%rbp)
14079044a: 0f 29 b5 50 01 00 00        	movaps	%xmm6, 0x150(%rbp)
140790451: 48 c7 85 48 01 00 00 fe ff ff ff    	movq	$-0x2, 0x148(%rbp)
14079045c: 4c 89 8d 38 01 00 00        	movq	%r9, 0x138(%rbp)
140790463: 4c 89 c7                    	movq	%r8, %rdi
140790466: 48 89 55 48                 	movq	%rdx, 0x48(%rbp)
14079046a: 48 89 cb                    	movq	%rcx, %rbx
14079046d: e8 8e 65 48 00              	callq	0x140c16a00 <.text+0xc15a00>
140790472: 84 c0                       	testb	%al, %al
140790474: 48 89 7d 58                 	movq	%rdi, 0x58(%rbp)
140790478: 48 89 9d f0 00 00 00        	movq	%rbx, 0xf0(%rbp)
14079047f: 74 60                       	je	0x1407904e1 <.text+0x78f4e1>
140790481: 4c 8d 25 20 0b 2f 02        	leaq	0x22f0b20(%rip), %r12   # 0x142a80fa8
140790488: 41 bf 44 00 00 00           	movl	$0x44, %r15d
14079048e: ba 44 00 00 00              	movl	$0x44, %edx
140790493: 4c 89 e1                    	movq	%r12, %rcx
140790496: e8 15 ca ff ff              	callq	0x14078ceb0 <.text+0x78beb0>
14079049b: 48 83 f8 01                 	cmpq	$0x1, %rax
14079049f: 0f 85 b2 00 00 00           	jne	0x140790557 <.text+0x78f557>
1407904a5: 49 89 d1                    	movq	%rdx, %r9
1407904a8: 48 85 d2                    	testq	%rdx, %rdx
1407904ab: 74 3d                       	je	0x1407904ea <.text+0x78f4ea>
1407904ad: 49 83 f9 44                 	cmpq	$0x44, %r9
1407904b1: 73 35                       	jae	0x1407904e8 <.text+0x78f4e8>
1407904b3: 48 8d 05 ee 0a 2f 02        	leaq	0x22f0aee(%rip), %rax   # 0x142a80fa8
1407904ba: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
1407904bf: 7f 29                       	jg	0x1407904ea <.text+0x78f4ea>
1407904c1: 48 8d 05 e8 0b 2f 02        	leaq	0x22f0be8(%rip), %rax   # 0x142a810b0
1407904c8: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407904cd: 48 8d 0d d4 0a 2f 02        	leaq	0x22f0ad4(%rip), %rcx   # 0x142a80fa8
1407904d4: ba 44 00 00 00              	movl	$0x44, %edx
1407904d9: 45 31 c0                    	xorl	%r8d, %r8d
1407904dc: e8 5f 94 14 02              	callq	0x1428d9940 <.text+0x28d8940>
1407904e1: 31 c9                       	xorl	%ecx, %ecx
1407904e3: e9 c9 01 00 00              	jmp	0x1407906b1 <.text+0x78f6b1>
1407904e8: 75 d7                       	jne	0x1407904c1 <.text+0x78f4c1>
1407904ea: 4c 8d 25 b7 0a 2f 02        	leaq	0x22f0ab7(%rip), %r12   # 0x142a80fa8
1407904f1: 4c 89 e1                    	movq	%r12, %rcx
1407904f4: 4c 89 ca                    	movq	%r9, %rdx
1407904f7: e8 b4 c9 ff ff              	callq	0x14078ceb0 <.text+0x78beb0>
1407904fc: 48 83 f8 01                 	cmpq	$0x1, %rax
140790500: 75 55                       	jne	0x140790557 <.text+0x78f557>
140790502: 49 89 d0                    	movq	%rdx, %r8
140790505: 49 83 c0 02                 	addq	$0x2, %r8
140790509: 74 39                       	je	0x140790544 <.text+0x78f544>
14079050b: 49 83 f8 44                 	cmpq	$0x44, %r8
14079050f: 73 31                       	jae	0x140790542 <.text+0x78f542>
140790511: 48 8d 05 90 0a 2f 02        	leaq	0x22f0a90(%rip), %rax   # 0x142a80fa8
140790518: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
14079051d: 7f 25                       	jg	0x140790544 <.text+0x78f544>
14079051f: 48 8d 05 a2 0b 2f 02        	leaq	0x22f0ba2(%rip), %rax   # 0x142a810c8
140790526: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14079052b: 48 8d 0d 76 0a 2f 02        	leaq	0x22f0a76(%rip), %rcx   # 0x142a80fa8
140790532: ba 44 00 00 00              	movl	$0x44, %edx
140790537: 41 b9 44 00 00 00           	movl	$0x44, %r9d
14079053d: e8 fe 93 14 02              	callq	0x1428d9940 <.text+0x28d8940>
140790542: 75 db                       	jne	0x14079051f <.text+0x78f51f>
140790544: 41 bf 42 00 00 00           	movl	$0x42, %r15d
14079054a: 49 29 d7                    	subq	%rdx, %r15
14079054d: 4c 8d 25 54 0a 2f 02        	leaq	0x22f0a54(%rip), %r12   # 0x142a80fa8
140790554: 4d 01 c4                    	addq	%r8, %r12
140790557: 4c 8d 05 00 0c 2f 02        	leaq	0x22f0c00(%rip), %r8    # 0x142a8115e
14079055e: 4c 8d 2d c1 0b 2f 02        	leaq	0x22f0bc1(%rip), %r13   # 0x142a81126
140790565: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140790570: 41 0f be 40 ff              	movsbl	-0x1(%r8), %eax
140790575: 85 c0                       	testl	%eax, %eax
140790577: 78 17                       	js	0x140790590 <.text+0x78f590>
140790579: 49 ff c8                    	decq	%r8
14079057c: 83 f8 5c                    	cmpl	$0x5c, %eax
14079057f: 75 63                       	jne	0x1407905e4 <.text+0x78f5e4>
140790581: eb 72                       	jmp	0x1407905f5 <.text+0x78f5f5>
140790583: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140790590: 41 0f b6 48 fe              	movzbl	-0x2(%r8), %ecx
140790595: 80 f9 c0                    	cmpb	$-0x40, %cl
140790598: 7d 23                       	jge	0x1407905bd <.text+0x78f5bd>
14079059a: 41 0f b6 50 fd              	movzbl	-0x3(%r8), %edx
14079059f: 80 fa c0                    	cmpb	$-0x40, %dl
1407905a2: 7d 22                       	jge	0x1407905c6 <.text+0x78f5c6>
1407905a4: 45 0f b6 48 fc              	movzbl	-0x4(%r8), %r9d
1407905a9: 49 83 c0 fc                 	addq	$-0x4, %r8
1407905ad: 41 83 e1 07                 	andl	$0x7, %r9d
1407905b1: 41 c1 e1 06                 	shll	$0x6, %r9d
1407905b5: 83 e2 3f                    	andl	$0x3f, %edx
1407905b8: 44 09 ca                    	orl	%r9d, %edx
1407905bb: eb 10                       	jmp	0x1407905cd <.text+0x78f5cd>
1407905bd: 49 83 c0 fe                 	addq	$-0x2, %r8
1407905c1: 83 e1 1f                    	andl	$0x1f, %ecx
1407905c4: eb 0f                       	jmp	0x1407905d5 <.text+0x78f5d5>
1407905c6: 49 83 c0 fd                 	addq	$-0x3, %r8
1407905ca: 83 e2 0f                    	andl	$0xf, %edx
1407905cd: c1 e2 06                    	shll	$0x6, %edx
1407905d0: 83 e1 3f                    	andl	$0x3f, %ecx
1407905d3: 09 d1                       	orl	%edx, %ecx
1407905d5: c1 e1 06                    	shll	$0x6, %ecx
1407905d8: 24 3f                       	andb	$0x3f, %al
1407905da: 0f b6 c0                    	movzbl	%al, %eax
1407905dd: 09 c8                       	orl	%ecx, %eax
1407905df: 83 f8 5c                    	cmpl	$0x5c, %eax
1407905e2: 74 11                       	je	0x1407905f5 <.text+0x78f5f5>
1407905e4: 83 f8 2f                    	cmpl	$0x2f, %eax
1407905e7: 74 0c                       	je	0x1407905f5 <.text+0x78f5f5>
1407905e9: 4d 39 e8                    	cmpq	%r13, %r8
1407905ec: 75 82                       	jne	0x140790570 <.text+0x78f570>
1407905ee: be 38 00 00 00              	movl	$0x38, %esi
1407905f3: eb 4c                       	jmp	0x140790641 <.text+0x78f641>
1407905f5: 4c 8d 2d 2a 0b 2f 02        	leaq	0x22f0b2a(%rip), %r13   # 0x142a81126
1407905fc: 4d 29 e8                    	subq	%r13, %r8
1407905ff: 49 ff c0                    	incq	%r8
140790602: 74 32                       	je	0x140790636 <.text+0x78f636>
140790604: 49 83 f8 38                 	cmpq	$0x38, %r8
140790608: 73 2a                       	jae	0x140790634 <.text+0x78f634>
14079060a: 43 80 3c 28 bf              	cmpb	$-0x41, (%r8,%r13)
14079060f: 7f 25                       	jg	0x140790636 <.text+0x78f636>
140790611: 48 8d 05 78 0a 2f 02        	leaq	0x22f0a78(%rip), %rax   # 0x142a81090
140790618: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14079061d: 48 8d 0d 02 0b 2f 02        	leaq	0x22f0b02(%rip), %rcx   # 0x142a81126
140790624: ba 38 00 00 00              	movl	$0x38, %edx
140790629: 41 b9 38 00 00 00           	movl	$0x38, %r9d
14079062f: e8 0c 93 14 02              	callq	0x1428d9940 <.text+0x28d8940>
140790634: 75 db                       	jne	0x140790611 <.text+0x78f611>
140790636: be 38 00 00 00              	movl	$0x38, %esi
14079063b: 4c 29 c6                    	subq	%r8, %rsi
14079063e: 4d 01 c5                    	addq	%r8, %r13
140790641: e8 ea 7a 48 00              	callq	0x140c18130 <.text+0xc17130>
140790646: 48 89 c1                    	movq	%rax, %rcx
140790649: 48 8b 00                    	movq	(%rax), %rax
14079064c: 48 83 f8 01                 	cmpq	$0x1, %rax
140790650: 0f 85 1f 09 00 00           	jne	0x140790f75 <.text+0x78ff75>
140790656: 48 83 c1 08                 	addq	$0x8, %rcx
14079065a: 48 83 39 00                 	cmpq	$0x0, (%rcx)
14079065e: 0f 85 74 0a 00 00           	jne	0x1407910d8 <.text+0x7900d8>
140790664: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
14079066b: 48 89 8d 30 01 00 00        	movq	%rcx, 0x130(%rbp)
140790672: 48 83 c1 08                 	addq	$0x8, %rcx
140790676: 48 89 74 24 20              	movq	%rsi, 0x20(%rsp)
14079067b: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
140790684: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
14079068d: 4c 89 e2                    	movq	%r12, %rdx
140790690: 4d 89 f8                    	movq	%r15, %r8
140790693: 4d 89 e9                    	movq	%r13, %r9
140790696: e8 a5 66 48 00              	callq	0x140c16d40 <.text+0xc15d40>
14079069b: 48 8b 8d 30 01 00 00        	movq	0x130(%rbp), %rcx
1407906a2: 48 ff 01                    	incq	(%rcx)
1407906a5: 48 89 85 c0 00 00 00        	movq	%rax, 0xc0(%rbp)
1407906ac: b9 01 00 00 00              	movl	$0x1, %ecx
1407906b1: 48 8b 95 38 02 00 00        	movq	0x238(%rbp), %rdx
1407906b8: 48 89 8d b8 00 00 00        	movq	%rcx, 0xb8(%rbp)
1407906bf: 48 8b 85 38 01 00 00        	movq	0x138(%rbp), %rax
1407906c6: 48 c7 40 28 00 00 00 00     	movq	$0x0, 0x28(%rax)
1407906ce: 48 8b 0a                    	movq	(%rdx), %rcx
1407906d1: 4c 8b 52 08                 	movq	0x8(%rdx), %r10
1407906d5: 48 8b 11                    	movq	(%rcx), %rdx
1407906d8: 31 f6                       	xorl	%esi, %esi
1407906da: 49 89 d0                    	movq	%rdx, %r8
1407906dd: 4d 2b 42 18                 	subq	0x18(%r10), %r8
1407906e1: 4c 0f 42 c6                 	cmovbq	%rsi, %r8
1407906e5: 49 2b 52 38                 	subq	0x38(%r10), %rdx
1407906e9: 48 0f 42 d6                 	cmovbq	%rsi, %rdx
1407906ed: 49 8b 7a 10                 	movq	0x10(%r10), %rdi
1407906f1: 4d 8b 5a 30                 	movq	0x30(%r10), %r11
1407906f5: 4d 89 d9                    	movq	%r11, %r9
1407906f8: 49 29 d1                    	subq	%rdx, %r9
1407906fb: 4c 0f 42 ce                 	cmovbq	%rsi, %r9
1407906ff: 41 0f 97 c5                 	seta	%r13b
140790703: 48 89 fb                    	movq	%rdi, %rbx
140790706: 4c 29 c3                    	subq	%r8, %rbx
140790709: 48 0f 42 de                 	cmovbq	%rsi, %rbx
14079070d: 49 8b 42 40                 	movq	0x40(%r10), %rax
140790711: 41 0f 97 c6                 	seta	%r14b
140790715: 48 89 c6                    	movq	%rax, %rsi
140790718: 4c 29 ce                    	subq	%r9, %rsi
14079071b: 48 29 de                    	subq	%rbx, %rsi
14079071e: 4d 8b 62 08                 	movq	0x8(%r10), %r12
140790722: 4d 8b 7a 28                 	movq	0x28(%r10), %r15
140790726: 48 89 31                    	movq	%rsi, (%rcx)
140790729: 4c 8b 95 40 02 00 00        	movq	0x240(%rbp), %r10
140790730: 45 08 ee                    	orb	%r13b, %r14b
140790733: 44 88 b5 47 01 00 00        	movb	%r14b, 0x147(%rbp)
14079073a: 41 80 fe 01                 	cmpb	$0x1, %r14b
14079073e: 75 50                       	jne	0x140790790 <.text+0x78f790>
140790740: 48 89 d6                    	movq	%rdx, %rsi
140790743: 48 c1 e6 04                 	shlq	$0x4, %rsi
140790747: 49 01 f7                    	addq	%rsi, %r15
14079074a: 49 c1 e1 04                 	shlq	$0x4, %r9
14079074e: 4c 89 c6                    	movq	%r8, %rsi
140790751: 48 c1 e6 04                 	shlq	$0x4, %rsi
140790755: 49 01 f4                    	addq	%rsi, %r12
140790758: 48 c1 e3 04                 	shlq	$0x4, %rbx
14079075c: 31 f6                       	xorl	%esi, %esi
14079075e: 4c 39 c7                    	cmpq	%r8, %rdi
140790761: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140790767: 4d 0f 42 e0                 	cmovbq	%r8, %r12
14079076b: 49 8d 7c 1c f0              	leaq	-0x10(%r12,%rbx), %rdi
140790770: 48 0f 47 f7                 	cmovaq	%rdi, %rsi
140790774: 49 39 d3                    	cmpq	%rdx, %r11
140790777: 4d 0f 42 f8                 	cmovbq	%r8, %r15
14079077b: 4b 8d 54 0f f0              	leaq	-0x10(%r15,%r9), %rdx
140790780: 48 0f 46 d6                 	cmovbeq	%rsi, %rdx
140790784: 48 89 01                    	movq	%rax, (%rcx)
140790787: 8b 42 08                    	movl	0x8(%rdx), %eax
14079078a: 89 85 e0 00 00 00           	movl	%eax, 0xe0(%rbp)
140790790: 48 8b 85 38 01 00 00        	movq	0x138(%rbp), %rax
140790797: 48 83 c0 18                 	addq	$0x18, %rax
14079079b: 48 89 85 d0 00 00 00        	movq	%rax, 0xd0(%rbp)
1407907a2: 48 8b 95 f0 00 00 00        	movq	0xf0(%rbp), %rdx
1407907a9: 48 8b 0a                    	movq	(%rdx), %rcx
1407907ac: 48 8b 42 08                 	movq	0x8(%rdx), %rax
1407907b0: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
1407907b7: 8b 42 14                    	movl	0x14(%rdx), %eax
1407907ba: 89 85 28 01 00 00           	movl	%eax, 0x128(%rbp)
1407907c0: 4c 8b a9 f8 00 00 00        	movq	0xf8(%rcx), %r13
1407907c7: 48 8b 81 00 01 00 00        	movq	0x100(%rcx), %rax
1407907ce: 4d 8d 7c 85 00              	leaq	(%r13,%rax,4), %r15
1407907d3: 48 89 8d 20 01 00 00        	movq	%rcx, 0x120(%rbp)
1407907da: 0f b6 81 08 01 00 00        	movzbl	0x108(%rcx), %eax
1407907e1: 88 85 f0 00 00 00           	movb	%al, 0xf0(%rbp)
1407907e7: 49 8b 02                    	movq	(%r10), %rax
1407907ea: 48 83 c0 58                 	addq	$0x58, %rax
1407907ee: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
1407907f2: 48 8b 85 48 02 00 00        	movq	0x248(%rbp), %rax
1407907f9: 48 8b 00                    	movq	(%rax), %rax
1407907fc: 48 89 85 08 01 00 00        	movq	%rax, 0x108(%rbp)
140790803: b8 08 00 00 00              	movl	$0x8, %eax
140790808: 48 89 85 c8 00 00 00        	movq	%rax, 0xc8(%rbp)
14079080f: 48 c7 85 18 01 00 00 00 00 00 00    	movq	$0x0, 0x118(%rbp)
14079081a: f3 0f 10 3d 1e 05 2f 02     	movss	0x22f051e(%rip), %xmm7  # 0x142a80d40
140790822: f3 44 0f 10 05 49 fe 1c 02  	movss	0x21cfe49(%rip), %xmm8  # 0x142960674
14079082b: f3 44 0f 10 0d 34 c8 23 02  	movss	0x223c834(%rip), %xmm9  # 0x1429cd068
140790834: f3 44 0f 10 15 67 f4 2e 02  	movss	0x22ef467(%rip), %xmm10 # 0x142a7fca4
14079083d: f3 44 0f 10 1d fe 04 2f 02  	movss	0x22f04fe(%rip), %xmm11 # 0x142a80d44
140790846: f3 44 0f 10 25 b5 50 19 02  	movss	0x21950b5(%rip), %xmm12 # 0x142925904
14079084f: 45 31 e4                    	xorl	%r12d, %r12d
140790852: 45 31 f6                    	xorl	%r14d, %r14d
140790855: 48 c7 85 30 01 00 00 00 00 00 00    	movq	$0x0, 0x130(%rbp)
140790860: c7 85 2c 01 00 00 00 00 00 00       	movl	$0x0, 0x12c(%rbp)
14079086a: eb 07                       	jmp	0x140790873 <.text+0x78f873>
14079086c: 0f 1f 40 00                 	nopl	(%rax)
140790870: 49 ff c4                    	incq	%r12
140790873: 80 bd f0 00 00 00 00        	cmpb	$0x0, 0xf0(%rbp)
14079087a: 74 44                       	je	0x1407908c0 <.text+0x78f8c0>
14079087c: 4d 39 f4                    	cmpq	%r14, %r12
14079087f: 0f 85 e8 01 00 00           	jne	0x140790a6d <.text+0x78fa6d>
140790885: 4d 39 fd                    	cmpq	%r15, %r13
140790888: 0f 84 18 04 00 00           	je	0x140790ca6 <.text+0x78fca6>
14079088e: 48 8b 85 00 01 00 00        	movq	0x100(%rbp), %rax
140790895: 48 8b 80 a8 01 00 00        	movq	0x1a8(%rax), %rax
14079089c: 0f 1f 40 00                 	nopl	(%rax)
1407908a0: 41 8b 4d 00                 	movl	(%r13), %ecx
1407908a4: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
1407908a8: 4c 8b 74 c8 10              	movq	0x10(%rax,%rcx,8), %r14
1407908ad: 4d 85 f6                    	testq	%r14, %r14
1407908b0: 75 52                       	jne	0x140790904 <.text+0x78f904>
1407908b2: 49 83 c5 04                 	addq	$0x4, %r13
1407908b6: 4d 39 fd                    	cmpq	%r15, %r13
1407908b9: 75 e5                       	jne	0x1407908a0 <.text+0x78f8a0>
1407908bb: e9 e6 03 00 00              	jmp	0x140790ca6 <.text+0x78fca6>
1407908c0: 4d 39 f4                    	cmpq	%r14, %r12
1407908c3: 0f 85 30 02 00 00           	jne	0x140790af9 <.text+0x78faf9>
1407908c9: 4d 39 fd                    	cmpq	%r15, %r13
1407908cc: 0f 84 d4 03 00 00           	je	0x140790ca6 <.text+0x78fca6>
1407908d2: 48 8b 85 00 01 00 00        	movq	0x100(%rbp), %rax
1407908d9: 48 8b 80 00 01 00 00        	movq	0x100(%rax), %rax
1407908e0: 41 8b 4d 00                 	movl	(%r13), %ecx
1407908e4: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
1407908e8: 48 c1 e1 05                 	shlq	$0x5, %rcx
1407908ec: 4c 8b 74 08 58              	movq	0x58(%rax,%rcx), %r14
1407908f1: 4d 85 f6                    	testq	%r14, %r14
1407908f4: 75 7f                       	jne	0x140790975 <.text+0x78f975>
1407908f6: 49 83 c5 04                 	addq	$0x4, %r13
1407908fa: 4d 39 fd                    	cmpq	%r15, %r13
1407908fd: 75 e1                       	jne	0x1407908e0 <.text+0x78f8e0>
1407908ff: e9 a2 03 00 00              	jmp	0x140790ca6 <.text+0x78fca6>
140790904: 48 8d 0c c8                 	leaq	(%rax,%rcx,8), %rcx
140790908: 48 8b 51 40                 	movq	0x40(%rcx), %rdx
14079090c: 48 8b 41 18                 	movq	0x18(%rcx), %rax
140790910: 48 8b 49 38                 	movq	0x38(%rcx), %rcx
140790914: 4c 8b 85 20 01 00 00        	movq	0x120(%rbp), %r8
14079091b: 4d 8b 80 18 01 00 00        	movq	0x118(%r8), %r8
140790922: 4c 39 c2                    	cmpq	%r8, %rdx
140790925: 0f 86 d6 00 00 00           	jbe	0x140790a01 <.text+0x78fa01>
14079092b: 4e 8b 04 c1                 	movq	(%rcx,%r8,8), %r8
14079092f: 4d 85 c0                    	testq	%r8, %r8
140790932: 0f 84 c9 00 00 00           	je	0x140790a01 <.text+0x78fa01>
140790938: 49 f7 d0                    	notq	%r8
14079093b: 4f 8d 04 40                 	leaq	(%r8,%r8,2), %r8
14079093f: 49 c1 e0 04                 	shlq	$0x4, %r8
140790943: 4e 8b 44 00 10              	movq	0x10(%rax,%r8), %r8
140790948: 4c 89 85 30 01 00 00        	movq	%r8, 0x130(%rbp)
14079094f: 4c 8b 85 20 01 00 00        	movq	0x120(%rbp), %r8
140790956: 4d 8b 80 20 01 00 00        	movq	0x120(%r8), %r8
14079095d: 4c 39 c2                    	cmpq	%r8, %rdx
140790960: 0f 87 bd 00 00 00           	ja	0x140790a23 <.text+0x78fa23>
140790966: c7 85 2c 01 00 00 00 00 00 00       	movl	$0x0, 0x12c(%rbp)
140790970: e9 bc 00 00 00              	jmp	0x140790a31 <.text+0x78fa31>
140790975: 48 01 c8                    	addq	%rcx, %rax
140790978: 8b 88 94 00 00 00           	movl	0x94(%rax), %ecx
14079097e: 48 8b 95 00 01 00 00        	movq	0x100(%rbp), %rdx
140790985: 4c 8b 8a a8 01 00 00        	movq	0x1a8(%rdx), %r9
14079098c: 4c 8d 14 c9                 	leaq	(%rcx,%rcx,8), %r10
140790990: 4b 8b 4c d1 18              	movq	0x18(%r9,%r10,8), %rcx
140790995: 4b 8b 54 d1 38              	movq	0x38(%r9,%r10,8), %rdx
14079099a: 4c 8b 85 20 01 00 00        	movq	0x120(%rbp), %r8
1407909a1: 4d 8b 80 18 01 00 00        	movq	0x118(%r8), %r8
1407909a8: 4f 39 44 d1 40              	cmpq	%r8, 0x40(%r9,%r10,8)
1407909ad: 0f 86 c2 00 00 00           	jbe	0x140790a75 <.text+0x78fa75>
1407909b3: 4e 8b 04 c2                 	movq	(%rdx,%r8,8), %r8
1407909b7: 4d 85 c0                    	testq	%r8, %r8
1407909ba: 0f 84 b5 00 00 00           	je	0x140790a75 <.text+0x78fa75>
1407909c0: 49 f7 d0                    	notq	%r8
1407909c3: 4f 8d 04 40                 	leaq	(%r8,%r8,2), %r8
1407909c7: 49 c1 e0 04                 	shlq	$0x4, %r8
1407909cb: 4e 8b 44 01 10              	movq	0x10(%rcx,%r8), %r8
1407909d0: 4c 89 85 30 01 00 00        	movq	%r8, 0x130(%rbp)
1407909d7: 4c 8b 85 20 01 00 00        	movq	0x120(%rbp), %r8
1407909de: 4d 8b 80 20 01 00 00        	movq	0x120(%r8), %r8
1407909e5: 4c 39 80 88 00 00 00        	cmpq	%r8, 0x88(%rax)
1407909ec: 0f 87 a9 00 00 00           	ja	0x140790a9b <.text+0x78fa9b>
1407909f2: c7 85 2c 01 00 00 00 00 00 00       	movl	$0x0, 0x12c(%rbp)
1407909fc: e9 b1 00 00 00              	jmp	0x140790ab2 <.text+0x78fab2>
140790a01: 48 c7 85 30 01 00 00 00 00 00 00    	movq	$0x0, 0x130(%rbp)
140790a0c: 4c 8b 85 20 01 00 00        	movq	0x120(%rbp), %r8
140790a13: 4d 8b 80 20 01 00 00        	movq	0x120(%r8), %r8
140790a1a: 4c 39 c2                    	cmpq	%r8, %rdx
140790a1d: 0f 86 43 ff ff ff           	jbe	0x140790966 <.text+0x78f966>
140790a23: 4a 83 3c c1 00              	cmpq	$0x0, (%rcx,%r8,8)
140790a28: 0f 95 c2                    	setne	%dl
140790a2b: 89 95 2c 01 00 00           	movl	%edx, 0x12c(%rbp)
140790a31: 48 8b 95 20 01 00 00        	movq	0x120(%rbp), %rdx
140790a38: 48 8b 92 10 01 00 00        	movq	0x110(%rdx), %rdx
140790a3f: 48 8b 0c d1                 	movq	(%rcx,%rdx,8), %rcx
140790a43: 48 f7 d1                    	notq	%rcx
140790a46: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140790a4a: 48 c1 e1 04                 	shlq	$0x4, %rcx
140790a4e: 48 8b 54 08 10              	movq	0x10(%rax,%rcx), %rdx
140790a53: 48 89 95 18 01 00 00        	movq	%rdx, 0x118(%rbp)
140790a5a: 48 8b 44 08 28              	movq	0x28(%rax,%rcx), %rax
140790a5f: 48 89 85 f8 00 00 00        	movq	%rax, 0xf8(%rbp)
140790a66: 49 83 c5 04                 	addq	$0x4, %r13
140790a6a: 45 31 e4                    	xorl	%r12d, %r12d
140790a6d: 44 89 e0                    	movl	%r12d, %eax
140790a70: e9 96 00 00 00              	jmp	0x140790b0b <.text+0x78fb0b>
140790a75: 48 c7 85 30 01 00 00 00 00 00 00    	movq	$0x0, 0x130(%rbp)
140790a80: 4c 8b 85 20 01 00 00        	movq	0x120(%rbp), %r8
140790a87: 4d 8b 80 20 01 00 00        	movq	0x120(%r8), %r8
140790a8e: 4c 39 80 88 00 00 00        	cmpq	%r8, 0x88(%rax)
140790a95: 0f 86 57 ff ff ff           	jbe	0x1407909f2 <.text+0x78f9f2>
140790a9b: 4c 8b 88 80 00 00 00        	movq	0x80(%rax), %r9
140790aa2: 4b 83 3c c1 00              	cmpq	$0x0, (%r9,%r8,8)
140790aa7: 41 0f 95 c0                 	setne	%r8b
140790aab: 44 89 85 2c 01 00 00        	movl	%r8d, 0x12c(%rbp)
140790ab2: 4c 8b 85 20 01 00 00        	movq	0x120(%rbp), %r8
140790ab9: 4d 8b 80 10 01 00 00        	movq	0x110(%r8), %r8
140790ac0: 4a 8b 14 c2                 	movq	(%rdx,%r8,8), %rdx
140790ac4: 48 f7 d2                    	notq	%rdx
140790ac7: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
140790acb: 48 c1 e2 04                 	shlq	$0x4, %rdx
140790acf: 4c 8b 44 11 10              	movq	0x10(%rcx,%rdx), %r8
140790ad4: 4c 89 85 18 01 00 00        	movq	%r8, 0x118(%rbp)
140790adb: 48 8b 4c 11 28              	movq	0x28(%rcx,%rdx), %rcx
140790ae0: 48 89 8d f8 00 00 00        	movq	%rcx, 0xf8(%rbp)
140790ae7: 49 83 c5 04                 	addq	$0x4, %r13
140790aeb: 48 8b 40 50                 	movq	0x50(%rax), %rax
140790aef: 48 89 85 c8 00 00 00        	movq	%rax, 0xc8(%rbp)
140790af6: 45 31 e4                    	xorl	%r12d, %r12d
140790af9: 4c 89 e0                    	movq	%r12, %rax
140790afc: 48 c1 e0 04                 	shlq	$0x4, %rax
140790b00: 48 8b 8d c8 00 00 00        	movq	0xc8(%rbp), %rcx
140790b07: 8b 44 01 08                 	movl	0x8(%rcx,%rax), %eax
140790b0b: 48 8d 0c c0                 	leaq	(%rax,%rax,8), %rcx
140790b0f: 48 8b 95 18 01 00 00        	movq	0x118(%rbp), %rdx
140790b16: 48 8d 1c 8a                 	leaq	(%rdx,%rcx,4), %rbx
140790b1a: 48 8b 8d f8 00 00 00        	movq	0xf8(%rbp), %rcx
140790b21: 48 8d 34 81                 	leaq	(%rcx,%rax,4), %rsi
140790b25: 48 8b 8d 30 01 00 00        	movq	0x130(%rbp), %rcx
140790b2c: 48 8d 3c 81                 	leaq	(%rcx,%rax,4), %rdi
140790b30: 8b 4b 1c                    	movl	0x1c(%rbx), %ecx
140790b33: e8 08 57 50 00              	callq	0x140c96240 <.text+0xc95240>
140790b38: 48 89 45 f8                 	movq	%rax, -0x8(%rbp)
140790b3c: 48 8d 4d f8                 	leaq	-0x8(%rbp), %rcx
140790b40: e8 3b e8 51 00              	callq	0x140caf380 <.text+0xcae380>
140790b45: 0f 28 f0                    	movaps	%xmm0, %xmm6
140790b48: f3 0f 59 f7                 	mulss	%xmm7, %xmm6
140790b4c: f3 41 0f 58 f0              	addss	%xmm8, %xmm6
140790b51: 8b 85 e0 00 00 00           	movl	0xe0(%rbp), %eax
140790b57: 39 43 1c                    	cmpl	%eax, 0x1c(%rbx)
140790b5a: 0f 94 c0                    	sete	%al
140790b5d: 84 85 47 01 00 00           	testb	%al, 0x147(%rbp)
140790b63: 0f 84 b7 00 00 00           	je	0x140790c20 <.text+0x78fc20>
140790b69: 8b 85 28 01 00 00           	movl	0x128(%rbp), %eax
140790b6f: 89 06                       	movl	%eax, (%rsi)
140790b71: c6 43 20 01                 	movb	$0x1, 0x20(%rbx)
140790b75: 48 8b 85 38 01 00 00        	movq	0x138(%rbp), %rax
140790b7c: 48 8b 70 28                 	movq	0x28(%rax), %rsi
140790b80: 48 81 fe ff 7f 00 00        	cmpq	$0x7fff, %rsi           # imm = 0x7FFF
140790b87: 0f 87 e3 fc ff ff           	ja	0x140790870 <.text+0x78f870>
140790b8d: 48 89 f8                    	movq	%rdi, %rax
140790b90: 8b 7b 1c                    	movl	0x1c(%rbx), %edi
140790b93: 8b 00                       	movl	(%rax), %eax
140790b95: 89 85 e4 00 00 00           	movl	%eax, 0xe4(%rbp)
140790b9b: f3 0f 10 73 18              	movss	0x18(%rbx), %xmm6
140790ba0: 8b 43 08                    	movl	0x8(%rbx), %eax
140790ba3: 48 8d 4d 62                 	leaq	0x62(%rbp), %rcx
140790ba7: 89 41 08                    	movl	%eax, 0x8(%rcx)
140790baa: 48 8b 03                    	movq	(%rbx), %rax
140790bad: 48 89 01                    	movq	%rax, (%rcx)
140790bb0: 48 8b 85 d0 00 00 00        	movq	0xd0(%rbp), %rax
140790bb7: 48 3b 30                    	cmpq	(%rax), %rsi
140790bba: 75 13                       	jne	0x140790bcf <.text+0x78fbcf>
140790bbc: 48 8b 8d d0 00 00 00        	movq	0xd0(%rbp), %rcx
140790bc3: 48 8d 15 16 08 2f 02        	leaq	0x22f0816(%rip), %rdx   # 0x142a813e0
140790bca: e8 b1 bf ff ff              	callq	0x14078cb80 <.text+0x78bb80>
140790bcf: 4c 8b 8d 38 01 00 00        	movq	0x138(%rbp), %r9
140790bd6: 49 8b 41 20                 	movq	0x20(%r9), %rax
140790bda: 48 8d 0c f6                 	leaq	(%rsi,%rsi,8), %rcx
140790bde: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140790be2: 48 01 f1                    	addq	%rsi, %rcx
140790be5: 89 3c 08                    	movl	%edi, (%rax,%rcx)
140790be8: 8b 95 e4 00 00 00           	movl	0xe4(%rbp), %edx
140790bee: 89 54 08 04                 	movl	%edx, 0x4(%rax,%rcx)
140790bf2: f3 0f 11 74 08 08           	movss	%xmm6, 0x8(%rax,%rcx)
140790bf8: 66 c7 44 08 0c 00 00        	movw	$0x0, 0xc(%rax,%rcx)
140790bff: 48 8b 55 60                 	movq	0x60(%rbp), %rdx
140790c03: 4c 8b 45 66                 	movq	0x66(%rbp), %r8
140790c07: 48 89 54 08 0e              	movq	%rdx, 0xe(%rax,%rcx)
140790c0c: 4c 89 44 08 14              	movq	%r8, 0x14(%rax,%rcx)
140790c11: 48 ff c6                    	incq	%rsi
140790c14: 49 89 71 28                 	movq	%rsi, 0x28(%r9)
140790c18: 49 ff c4                    	incq	%r12
140790c1b: e9 53 fc ff ff              	jmp	0x140790873 <.text+0x78f873>
140790c20: f6 85 2c 01 00 00 01        	testb	$0x1, 0x12c(%rbp)
140790c27: 74 14                       	je	0x140790c3d <.text+0x78fc3d>
140790c29: 8b 85 28 01 00 00           	movl	0x128(%rbp), %eax
140790c2f: 89 06                       	movl	%eax, (%rsi)
140790c31: c6 43 20 00                 	movb	$0x0, 0x20(%rbx)
140790c35: 49 ff c4                    	incq	%r12
140790c38: e9 36 fc ff ff              	jmp	0x140790873 <.text+0x78f873>
140790c3d: 48 8b 4d 50                 	movq	0x50(%rbp), %rcx
140790c41: e8 2a ee 30 00              	callq	0x140a9fa70 <.text+0xa9ea70>
140790c46: 84 c0                       	testb	%al, %al
140790c48: 74 28                       	je	0x140790c72 <.text+0x78fc72>
140790c4a: 0f 28 ce                    	movaps	%xmm6, %xmm1
140790c4d: f3 41 0f 59 c9              	mulss	%xmm9, %xmm1
140790c52: f3 41 0f 58 ca              	addss	%xmm10, %xmm1
140790c57: 48 8b 85 08 01 00 00        	movq	0x108(%rbp), %rax
140790c5e: f3 0f 10 40 08              	movss	0x8(%rax), %xmm0
140790c63: 0f 2e c1                    	ucomiss	%xmm1, %xmm0
140790c66: 72 0a                       	jb	0x140790c72 <.text+0x78fc72>
140790c68: 44 0f 2e d8                 	ucomiss	%xmm0, %xmm11
140790c6c: 0f 87 f7 fe ff ff           	ja	0x140790b69 <.text+0x78fb69>
140790c72: 48 8b 85 08 01 00 00        	movq	0x108(%rbp), %rax
140790c79: f3 0f 10 40 24              	movss	0x24(%rax), %xmm0
140790c7e: 0f c2 f0 02                 	cmpleps	%xmm0, %xmm6
140790c82: 41 0f c2 c4 01              	cmpltps	%xmm12, %xmm0
140790c87: 0f 54 c6                    	andps	%xmm6, %xmm0
140790c8a: 66 0f 7e c0                 	movd	%xmm0, %eax
140790c8e: 8b 8d 28 01 00 00           	movl	0x128(%rbp), %ecx
140790c94: 89 0e                       	movl	%ecx, (%rsi)
140790c96: 24 01                       	andb	$0x1, %al
140790c98: 88 43 20                    	movb	%al, 0x20(%rbx)
140790c9b: 0f 84 cf fb ff ff           	je	0x140790870 <.text+0x78f870>
140790ca1: e9 cf fe ff ff              	jmp	0x140790b75 <.text+0x78fb75>
140790ca6: 48 8b 85 38 01 00 00        	movq	0x138(%rbp), %rax
140790cad: 48 8b 48 20                 	movq	0x20(%rax), %rcx
140790cb1: 48 8b 50 28                 	movq	0x28(%rax), %rdx
140790cb5: 48 8d 45 f8                 	leaq	-0x8(%rbp), %rax
140790cb9: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
140790cbd: 48 83 fa 02                 	cmpq	$0x2, %rdx
140790cc1: 0f 83 1d 04 00 00           	jae	0x1407910e4 <.text+0x7900e4>
140790cc7: c7 85 dc 00 00 00 00 00 00 00       	movl	$0x0, 0xdc(%rbp)
140790cd1: 48 8d 85 dc 00 00 00        	leaq	0xdc(%rbp), %rax
140790cd8: 48 89 45 d8                 	movq	%rax, -0x28(%rbp)
140790cdc: 48 8b 85 30 02 00 00        	movq	0x230(%rbp), %rax
140790ce3: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
140790ce7: 48 8b 85 38 01 00 00        	movq	0x138(%rbp), %rax
140790cee: 48 8d 78 30                 	leaq	0x30(%rax), %rdi
140790cf2: 48 8d 70 48                 	leaq	0x48(%rax), %rsi
140790cf6: 48 c7 85 10 01 00 00 ff ff ff ff    	movq	$-0x1, 0x110(%rbp)
140790d01: 48 c7 85 e8 00 00 00 00 00 00 00    	movq	$0x0, 0xe8(%rbp)
140790d0c: 48 8b 48 20                 	movq	0x20(%rax), %rcx
140790d10: 48 8b 50 28                 	movq	0x28(%rax), %rdx
140790d14: 4c 8b 40 08                 	movq	0x8(%rax), %r8
140790d18: 4c 8b 48 10                 	movq	0x10(%rax), %r9
140790d1c: 4c 8d 7d d8                 	leaq	-0x28(%rbp), %r15
140790d20: 4c 89 7d 60                 	movq	%r15, 0x60(%rbp)
140790d24: 48 89 7d 68                 	movq	%rdi, 0x68(%rbp)
140790d28: 48 8d 9d 10 01 00 00        	leaq	0x110(%rbp), %rbx
140790d2f: 48 89 5d 70                 	movq	%rbx, 0x70(%rbp)
140790d33: 4c 8d b5 e8 00 00 00        	leaq	0xe8(%rbp), %r14
140790d3a: 4c 89 75 78                 	movq	%r14, 0x78(%rbp)
140790d3e: 48 89 b5 80 00 00 00        	movq	%rsi, 0x80(%rbp)
140790d45: 4c 8d 65 60                 	leaq	0x60(%rbp), %r12
140790d49: 4c 89 64 24 20              	movq	%r12, 0x20(%rsp)
140790d4e: e8 4d 72 ac 00              	callq	0x141257fa0 <.text+0x1256fa0>
140790d53: 48 83 bd 10 01 00 00 ff     	cmpq	$-0x1, 0x110(%rbp)
140790d5b: 74 21                       	je	0x140790d7e <.text+0x78fd7e>
140790d5d: 48 8b 85 38 01 00 00        	movq	0x138(%rbp), %rax
140790d64: 48 8b 48 38                 	movq	0x38(%rax), %rcx
140790d68: 48 8b 50 40                 	movq	0x40(%rax), %rdx
140790d6c: 48 8d 45 f8                 	leaq	-0x8(%rbp), %rax
140790d70: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
140790d74: 48 83 fa 02                 	cmpq	$0x2, %rdx
140790d78: 0f 83 af 03 00 00           	jae	0x14079112d <.text+0x79012d>
140790d7e: 48 c7 85 b0 00 00 00 00 00 00 00    	movq	$0x0, 0xb0(%rbp)
140790d89: 48 8b 85 38 01 00 00        	movq	0x138(%rbp), %rax
140790d90: 48 8b 48 20                 	movq	0x20(%rax), %rcx
140790d94: 48 8b 50 28                 	movq	0x28(%rax), %rdx
140790d98: 4c 8b 40 08                 	movq	0x8(%rax), %r8
140790d9c: 4c 8b 48 10                 	movq	0x10(%rax), %r9
140790da0: 4c 89 7d 60                 	movq	%r15, 0x60(%rbp)
140790da4: 48 89 7d 68                 	movq	%rdi, 0x68(%rbp)
140790da8: 48 8d 85 b0 00 00 00        	leaq	0xb0(%rbp), %rax
140790daf: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
140790db3: 48 89 75 78                 	movq	%rsi, 0x78(%rbp)
140790db7: 48 89 9d 80 00 00 00        	movq	%rbx, 0x80(%rbp)
140790dbe: 4c 89 b5 88 00 00 00        	movq	%r14, 0x88(%rbp)
140790dc5: 4c 89 64 24 20              	movq	%r12, 0x20(%rsp)
140790dca: e8 01 76 ac 00              	callq	0x1412583d0 <.text+0x12573d0>
140790dcf: 48 8b 45 48                 	movq	0x48(%rbp), %rax
140790dd3: 48 8b 08                    	movq	(%rax), %rcx
140790dd6: 48 8d 15 0b 08 49 02        	leaq	0x249080b(%rip), %rdx   # 0x142c215e8
140790ddd: e8 2e 78 ac 00              	callq	0x141258610 <.text+0x1257610>
140790de2: 48 89 c3                    	movq	%rax, %rbx
140790de5: 48 85 c0                    	testq	%rax, %rax
140790de8: 0f 84 d3 03 00 00           	je	0x1407911c1 <.text+0x7901c1>
140790dee: 48 8d 05 8f 09 49 02        	leaq	0x249098f(%rip), %rax   # 0x142c21784
140790df5: 48 89 45 f8                 	movq	%rax, -0x8(%rbp)
140790df9: 48 c7 45 00 35 00 00 00     	movq	$0x35, (%rbp)
140790e01: 49 89 de                    	movq	%rbx, %r14
140790e04: 49 83 c6 10                 	addq	$0x10, %r14
140790e08: 48 83 7b 18 35              	cmpq	$0x35, 0x18(%rbx)
140790e0d: 0f 85 ef 02 00 00           	jne	0x140791102 <.text+0x790102>
140790e13: 49 8b 16                    	movq	(%r14), %rdx
140790e16: 48 8d 0d 67 09 49 02        	leaq	0x2490967(%rip), %rcx   # 0x142c21784
140790e1d: 41 b8 35 00 00 00           	movl	$0x35, %r8d
140790e23: e8 c8 2d 19 02              	callq	0x142923bf0 <.text+0x2922bf0>
140790e28: 85 c0                       	testl	%eax, %eax
140790e2a: 0f 85 d2 02 00 00           	jne	0x140791102 <.text+0x790102>
140790e30: 48 8b 0b                    	movq	(%rbx), %rcx
140790e33: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140790e37: ff 50 18                    	callq	*0x18(%rax)
140790e3a: 48 89 c3                    	movq	%rax, %rbx
140790e3d: 48 85 c0                    	testq	%rax, %rax
140790e40: 0f 84 7b 03 00 00           	je	0x1407911c1 <.text+0x7901c1>
140790e46: 48 8b 85 10 01 00 00        	movq	0x110(%rbp), %rax
140790e4d: 48 3b 85 e8 00 00 00        	cmpq	0xe8(%rbp), %rax
140790e54: 0f 87 8c 01 00 00           	ja	0x140790fe6 <.text+0x78ffe6>
140790e5a: 48 8b 45 58                 	movq	0x58(%rbp), %rax
140790e5e: 48 8b 08                    	movq	(%rax), %rcx
140790e61: e8 3a 6e 1f 00              	callq	0x140987ca0 <.text+0x986ca0>
140790e66: 4c 8b 85 e8 00 00 00        	movq	0xe8(%rbp), %r8
140790e6d: 49 83 f8 ff                 	cmpq	$-0x1, %r8
140790e71: 0f 84 1a 03 00 00           	je	0x140791191 <.text+0x790191>
140790e77: 48 8b 8d 10 01 00 00        	movq	0x110(%rbp), %rcx
140790e7e: 49 8d 50 01                 	leaq	0x1(%r8), %rdx
140790e82: 48 39 ca                    	cmpq	%rcx, %rdx
140790e85: 0f 82 14 03 00 00           	jb	0x14079119f <.text+0x79019f>
140790e8b: 4c 8b 8d 38 01 00 00        	movq	0x138(%rbp), %r9
140790e92: 4d 8b 49 58                 	movq	0x58(%r9), %r9
140790e96: 4d 39 c8                    	cmpq	%r9, %r8
140790e99: 0f 83 0e 03 00 00           	jae	0x1407911ad <.text+0x7901ad>
140790e9f: 49 89 d2                    	movq	%rdx, %r10
140790ea2: 49 29 ca                    	subq	%rcx, %r10
140790ea5: 0f 84 3b 01 00 00           	je	0x140790fe6 <.text+0x78ffe6>
140790eab: 48 c1 e2 04                 	shlq	$0x4, %rdx
140790eaf: 48 3b 53 20                 	cmpq	0x20(%rbx), %rdx
140790eb3: 0f 87 2d 01 00 00           	ja	0x140790fe6 <.text+0x78ffe6>
140790eb9: 48 c1 e1 04                 	shlq	$0x4, %rcx
140790ebd: 48 8b 95 38 01 00 00        	movq	0x138(%rbp), %rdx
140790ec4: 4c 8b 4a 50                 	movq	0x50(%rdx), %r9
140790ec8: 49 01 c9                    	addq	%rcx, %r9
140790ecb: 49 c1 e2 04                 	shlq	$0x4, %r10
140790ecf: 49 89 d8                    	movq	%rbx, %r8
140790ed2: 49 83 c0 08                 	addq	$0x8, %r8
140790ed6: 48 c7 45 c8 00 00 00 00     	movq	$0x0, -0x38(%rbp)
140790ede: 4c 89 55 d0                 	movq	%r10, -0x30(%rbp)
140790ee2: 48 89 4c 24 38              	movq	%rcx, 0x38(%rsp)
140790ee7: 48 8d 4d c8                 	leaq	-0x38(%rbp), %rcx
140790eeb: 48 89 4c 24 28              	movq	%rcx, 0x28(%rsp)
140790ef0: 4c 89 54 24 20              	movq	%r10, 0x20(%rsp)
140790ef5: 48 c7 44 24 30 01 00 00 00  	movq	$0x1, 0x30(%rsp)
140790efe: 48 8d 4d 60                 	leaq	0x60(%rbp), %rcx
140790f02: 48 89 c2                    	movq	%rax, %rdx
140790f05: e8 96 e0 45 00              	callq	0x140beefa0 <.text+0xbedfa0>
140790f0a: 48 8b 45 60                 	movq	0x60(%rbp), %rax
140790f0e: 48 83 f8 0b                 	cmpq	$0xb, %rax
140790f12: 0f 84 ce 00 00 00           	je	0x140790fe6 <.text+0x78ffe6>
140790f18: 48 8b 8d a8 00 00 00        	movq	0xa8(%rbp), %rcx
140790f1f: 48 89 4d 40                 	movq	%rcx, 0x40(%rbp)
140790f23: 0f 10 45 68                 	movups	0x68(%rbp), %xmm0
140790f27: 0f 10 4d 78                 	movups	0x78(%rbp), %xmm1
140790f2b: 0f 10 95 88 00 00 00        	movups	0x88(%rbp), %xmm2
140790f32: 0f 10 9d 98 00 00 00        	movups	0x98(%rbp), %xmm3
140790f39: 0f 11 5d 30                 	movups	%xmm3, 0x30(%rbp)
140790f3d: 0f 11 55 20                 	movups	%xmm2, 0x20(%rbp)
140790f41: 0f 11 4d 10                 	movups	%xmm1, 0x10(%rbp)
140790f45: 0f 11 45 00                 	movups	%xmm0, (%rbp)
140790f49: 48 89 45 f8                 	movq	%rax, -0x8(%rbp)
140790f4d: 48 8d 50 f7                 	leaq	-0x9(%rax), %rdx
140790f51: 4c 8d 40 f8                 	leaq	-0x8(%rax), %r8
140790f55: 31 c9                       	xorl	%ecx, %ecx
140790f57: 48 83 fa 02                 	cmpq	$0x2, %rdx
140790f5b: 49 0f 42 c8                 	cmovbq	%r8, %rcx
140790f5f: 48 85 c9                    	testq	%rcx, %rcx
140790f62: 74 32                       	je	0x140790f96 <.text+0x78ff96>
140790f64: 48 83 f9 01                 	cmpq	$0x1, %rcx
140790f68: 75 64                       	jne	0x140790fce <.text+0x78ffce>
140790f6a: 48 8d 4d 00                 	leaq	(%rbp), %rcx
140790f6e: e8 bd 75 0f 00              	callq	0x140888530 <.text+0x887530>
140790f73: eb 71                       	jmp	0x140790fe6 <.text+0x78ffe6>
140790f75: 48 85 c0                    	testq	%rax, %rax
140790f78: 0f 85 4e 01 00 00           	jne	0x1407910cc <.text+0x7900cc>
140790f7e: 31 d2                       	xorl	%edx, %edx
140790f80: e8 fb 15 16 02              	callq	0x1428f2580 <.text+0x28f1580>
140790f85: 48 89 c1                    	movq	%rax, %rcx
140790f88: 48 85 c0                    	testq	%rax, %rax
140790f8b: 0f 85 c9 f6 ff ff           	jne	0x14079065a <.text+0x78f65a>
140790f91: e9 36 01 00 00              	jmp	0x1407910cc <.text+0x7900cc>
140790f96: 48 83 f8 05                 	cmpq	$0x5, %rax
140790f9a: 77 23                       	ja	0x140790fbf <.text+0x78ffbf>
140790f9c: b9 32 00 00 00              	movl	$0x32, %ecx
140790fa1: 48 0f a3 c1                 	btq	%rax, %rcx
140790fa5: 73 18                       	jae	0x140790fbf <.text+0x78ffbf>
140790fa7: 48 8b 55 00                 	movq	(%rbp), %rdx
140790fab: 48 85 d2                    	testq	%rdx, %rdx
140790fae: 74 0f                       	je	0x140790fbf <.text+0x78ffbf>
140790fb0: 48 8b 4d 08                 	movq	0x8(%rbp), %rcx
140790fb4: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140790fba: e8 61 2c e8 ff              	callq	0x140613c20 <.text+0x612c20>
140790fbf: 48 8b 55 18                 	movq	0x18(%rbp), %rdx
140790fc3: 48 85 d2                    	testq	%rdx, %rdx
140790fc6: 74 1e                       	je	0x140790fe6 <.text+0x78ffe6>
140790fc8: 48 8b 4d 20                 	movq	0x20(%rbp), %rcx
140790fcc: eb 0d                       	jmp	0x140790fdb <.text+0x78ffdb>
140790fce: 48 8b 55 00                 	movq	(%rbp), %rdx
140790fd2: 48 85 d2                    	testq	%rdx, %rdx
140790fd5: 74 0f                       	je	0x140790fe6 <.text+0x78ffe6>
140790fd7: 48 8b 4d 08                 	movq	0x8(%rbp), %rcx
140790fdb: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140790fe1: e8 3a 2c e8 ff              	callq	0x140613c20 <.text+0x612c20>
140790fe6: 48 8b 85 b0 00 00 00        	movq	0xb0(%rbp), %rax
140790fed: 48 89 03                    	movq	%rax, (%rbx)
140790ff0: 48 8b 95 38 01 00 00        	movq	0x138(%rbp), %rdx
140790ff7: 0f 10 42 18                 	movups	0x18(%rdx), %xmm0
140790ffb: 0f 10 0a                    	movups	(%rdx), %xmm1
140790ffe: 0f 11 4a 18                 	movups	%xmm1, 0x18(%rdx)
140791002: 0f 11 02                    	movups	%xmm0, (%rdx)
140791005: 48 8b 42 28                 	movq	0x28(%rdx), %rax
140791009: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
14079100d: 48 89 4a 28                 	movq	%rcx, 0x28(%rdx)
140791011: 48 89 42 10                 	movq	%rax, 0x10(%rdx)
140791015: 48 83 bd b8 00 00 00 00     	cmpq	$0x0, 0xb8(%rbp)
14079101d: 74 4b                       	je	0x14079106a <.text+0x79006a>
14079101f: e8 0c 71 48 00              	callq	0x140c18130 <.text+0xc17130>
140791024: 48 89 c1                    	movq	%rax, %rcx
140791027: 48 8b 00                    	movq	(%rax), %rax
14079102a: 48 83 f8 01                 	cmpq	$0x1, %rax
14079102e: 0f 85 80 00 00 00           	jne	0x1407910b4 <.text+0x7900b4>
140791034: 48 83 c1 08                 	addq	$0x8, %rcx
140791038: 48 83 39 00                 	cmpq	$0x0, (%rcx)
14079103c: 0f 85 96 00 00 00           	jne	0x1407910d8 <.text+0x7900d8>
140791042: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
140791049: 48 89 8d 38 01 00 00        	movq	%rcx, 0x138(%rbp)
140791050: 48 83 c1 08                 	addq	$0x8, %rcx
140791054: 48 8b 95 c0 00 00 00        	movq	0xc0(%rbp), %rdx
14079105b: e8 90 60 48 00              	callq	0x140c170f0 <.text+0xc160f0>
140791060: 48 8b 85 38 01 00 00        	movq	0x138(%rbp), %rax
140791067: 48 ff 00                    	incq	(%rax)
14079106a: 0f 28 b5 50 01 00 00        	movaps	0x150(%rbp), %xmm6
140791071: 0f 28 bd 60 01 00 00        	movaps	0x160(%rbp), %xmm7
140791078: 44 0f 28 85 70 01 00 00     	movaps	0x170(%rbp), %xmm8
140791080: 44 0f 28 8d 80 01 00 00     	movaps	0x180(%rbp), %xmm9
140791088: 44 0f 28 95 90 01 00 00     	movaps	0x190(%rbp), %xmm10
140791090: 44 0f 28 9d a0 01 00 00     	movaps	0x1a0(%rbp), %xmm11
140791098: 44 0f 28 a5 b0 01 00 00     	movaps	0x1b0(%rbp), %xmm12
1407910a0: 48 81 c4 48 02 00 00        	addq	$0x248, %rsp            # imm = 0x248
1407910a7: 5b                          	popq	%rbx
1407910a8: 5f                          	popq	%rdi
1407910a9: 5e                          	popq	%rsi
1407910aa: 41 5c                       	popq	%r12
1407910ac: 41 5d                       	popq	%r13
1407910ae: 41 5e                       	popq	%r14
1407910b0: 41 5f                       	popq	%r15
1407910b2: 5d                          	popq	%rbp
1407910b3: c3                          	retq
1407910b4: 48 85 c0                    	testq	%rax, %rax
1407910b7: 75 13                       	jne	0x1407910cc <.text+0x7900cc>
1407910b9: 31 d2                       	xorl	%edx, %edx
1407910bb: e8 c0 14 16 02              	callq	0x1428f2580 <.text+0x28f1580>
1407910c0: 48 89 c1                    	movq	%rax, %rcx
1407910c3: 48 85 c0                    	testq	%rax, %rax
1407910c6: 0f 85 6c ff ff ff           	jne	0x140791038 <.text+0x790038>
1407910cc: 48 8d 0d f5 02 49 02        	leaq	0x24902f5(%rip), %rcx   # 0x142c213c8
1407910d3: e8 f8 6f 14 02              	callq	0x1428d80d0 <.text+0x28d70d0>
1407910d8: 48 8d 0d 59 03 49 02        	leaq	0x2490359(%rip), %rcx   # 0x142c21438
1407910df: e8 6c 81 14 02              	callq	0x1428d9250 <.text+0x28d8250>
1407910e4: 48 83 fa 15                 	cmpq	$0x15, %rdx
1407910e8: 0f 83 e1 00 00 00           	jae	0x1407911cf <.text+0x7901cf>
1407910ee: 4c 8d 4d 60                 	leaq	0x60(%rbp), %r9
1407910f2: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1407910f8: e8 a3 a8 ac 00              	callq	0x14125b9a0 <.text+0x125a9a0>
1407910fd: e9 c5 fb ff ff              	jmp	0x140790cc7 <.text+0x78fcc7>
140791102: 48 c7 45 60 00 00 00 00     	movq	$0x0, 0x60(%rbp)
14079110a: 48 8d 05 bf 04 49 02        	leaq	0x24904bf(%rip), %rax   # 0x142c215d0
140791111: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140791116: 48 8d 55 f8                 	leaq	-0x8(%rbp), %rdx
14079111a: 4c 8d 4d 60                 	leaq	0x60(%rbp), %r9
14079111e: 31 c9                       	xorl	%ecx, %ecx
140791120: 4d 89 f0                    	movq	%r14, %r8
140791123: e8 19 9a 13 02              	callq	0x1428cab41 <.text+0x28c9b41>
140791128: e9 a0 00 00 00              	jmp	0x1407911cd <.text+0x7901cd>
14079112d: 48 83 fa 15                 	cmpq	$0x15, %rdx
140791131: 0f 83 a6 00 00 00           	jae	0x1407911dd <.text+0x7901dd>
140791137: 48 8d 04 51                 	leaq	(%rcx,%rdx,2), %rax
14079113b: 48 8d 51 02                 	leaq	0x2(%rcx), %rdx
14079113f: 41 b8 02 00 00 00           	movl	$0x2, %r8d
140791145: eb 18                       	jmp	0x14079115f <.text+0x79015f>
140791147: 49 89 ca                    	movq	%rcx, %r10
14079114a: 66 45 89 0a                 	movw	%r9w, (%r10)
14079114e: 48 83 c2 02                 	addq	$0x2, %rdx
140791152: 49 83 c0 02                 	addq	$0x2, %r8
140791156: 48 39 c2                    	cmpq	%rax, %rdx
140791159: 0f 84 1f fc ff ff           	je	0x140790d7e <.text+0x78fd7e>
14079115f: 44 0f b7 5a fe              	movzwl	-0x2(%rdx), %r11d
140791164: 44 0f b7 0a                 	movzwl	(%rdx), %r9d
140791168: 66 45 39 cb                 	cmpw	%r9w, %r11w
14079116c: 73 e0                       	jae	0x14079114e <.text+0x79014e>
14079116e: 4d 89 c2                    	movq	%r8, %r10
140791171: 66 46 89 1c 11              	movw	%r11w, (%rcx,%r10)
140791176: 49 83 fa 02                 	cmpq	$0x2, %r10
14079117a: 74 cb                       	je	0x140791147 <.text+0x790147>
14079117c: 46 0f b7 5c 11 fc           	movzwl	-0x4(%rcx,%r10), %r11d
140791182: 49 83 c2 fe                 	addq	$-0x2, %r10
140791186: 66 45 39 cb                 	cmpw	%r9w, %r11w
14079118a: 72 e5                       	jb	0x140791171 <.text+0x790171>
14079118c: 49 01 ca                    	addq	%rcx, %r10
14079118f: eb b9                       	jmp	0x14079114a <.text+0x79014a>
140791191: 48 8d 0d 30 02 2f 02        	leaq	0x22f0230(%rip), %rcx   # 0x142a813c8
140791198: e8 03 87 14 02              	callq	0x1428d98a0 <.text+0x28d88a0>
14079119d: eb 2e                       	jmp	0x1407911cd <.text+0x7901cd>
14079119f: 4c 8d 05 22 02 2f 02        	leaq	0x22f0222(%rip), %r8    # 0x142a813c8
1407911a6: e8 a5 86 14 02              	callq	0x1428d9850 <.text+0x28d8850>
1407911ab: eb 20                       	jmp	0x1407911cd <.text+0x7901cd>
1407911ad: 4c 8d 05 14 02 2f 02        	leaq	0x22f0214(%rip), %r8    # 0x142a813c8
1407911b4: 48 89 d1                    	movq	%rdx, %rcx
1407911b7: 4c 89 ca                    	movq	%r9, %rdx
1407911ba: e8 81 86 14 02              	callq	0x1428d9840 <.text+0x28d8840>
1407911bf: eb 0c                       	jmp	0x1407911cd <.text+0x7901cd>
1407911c1: 48 8d 0d e8 01 2f 02        	leaq	0x22f01e8(%rip), %rcx   # 0x142a813b0
1407911c8: e8 43 81 14 02              	callq	0x1428d9310 <.text+0x28d8310>
1407911cd: 0f 0b                       	ud2
1407911cf: 4c 8d 45 60                 	leaq	0x60(%rbp), %r8
1407911d3: e8 08 bc ac 00              	callq	0x14125cde0 <.text+0x125bde0>
1407911d8: e9 ea fa ff ff              	jmp	0x140790cc7 <.text+0x78fcc7>
1407911dd: 4c 8d 45 60                 	leaq	0x60(%rbp), %r8
1407911e1: e8 9a ba ac 00              	callq	0x14125cc80 <.text+0x125bc80>
1407911e6: e9 93 fb ff ff              	jmp	0x140790d7e <.text+0x78fd7e>
1407911eb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
1407911f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407911f5: 55                          	pushq	%rbp
1407911f6: 41 57                       	pushq	%r15
1407911f8: 41 56                       	pushq	%r14
1407911fa: 41 55                       	pushq	%r13
1407911fc: 41 54                       	pushq	%r12
1407911fe: 56                          	pushq	%rsi
1407911ff: 57                          	pushq	%rdi
