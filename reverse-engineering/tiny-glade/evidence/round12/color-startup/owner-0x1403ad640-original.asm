
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1403ad640: 55                          	pushq	%rbp
1403ad641: 41 57                       	pushq	%r15
1403ad643: 41 56                       	pushq	%r14
1403ad645: 41 55                       	pushq	%r13
1403ad647: 41 54                       	pushq	%r12
1403ad649: 56                          	pushq	%rsi
1403ad64a: 57                          	pushq	%rdi
1403ad64b: 53                          	pushq	%rbx
1403ad64c: 48 81 ec 68 02 00 00        	subq	$0x268, %rsp            # imm = 0x268
1403ad653: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1403ad65b: 48 c7 85 e0 01 00 00 fe ff ff ff    	movq	$-0x2, 0x1e0(%rbp)
1403ad666: 48 89 ce                    	movq	%rcx, %rsi
1403ad669: 4c 8b 3d 00 ea 27 03        	movq	0x327ea00(%rip), %r15   # 0x14362c070
1403ad670: 41 83 3f 02                 	cmpl	$0x2, (%r15)
1403ad674: 75 16                       	jne	0x1403ad68c <.text+0x3ac68c>
1403ad676: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1403ad680: 48 89 46 08                 	movq	%rax, 0x8(%rsi)
1403ad684: 48 89 06                    	movq	%rax, (%rsi)
1403ad687: e9 3e 07 00 00              	jmp	0x1403addca <.text+0x3acdca>
1403ad68c: 48 c7 44 24 20 09 00 00 00  	movq	$0x9, 0x20(%rsp)
1403ad695: 4c 8d 0d e4 e3 61 02        	leaq	0x261e3e4(%rip), %r9    # 0x1429cba80
1403ad69c: 48 8d bd 30 01 00 00        	leaq	0x130(%rbp), %rdi
1403ad6a3: 48 89 f9                    	movq	%rdi, %rcx
1403ad6a6: e8 65 51 9b 00              	callq	0x140d62810 <.text+0xd61810>
1403ad6ab: c7 85 50 01 00 00 00 00 00 00       	movl	$0x0, 0x150(%rbp)
1403ad6b5: 0f 57 c0                    	xorps	%xmm0, %xmm0
1403ad6b8: 0f 11 85 58 01 00 00        	movups	%xmm0, 0x158(%rbp)
1403ad6bf: c7 85 68 01 00 00 07 00 00 00       	movl	$0x7, 0x168(%rbp)
1403ad6c9: 48 c7 85 6c 01 00 00 00 00 00 00    	movq	$0x0, 0x16c(%rbp)
1403ad6d4: 66 c7 85 74 01 00 00 00 00  	movw	$0x0, 0x174(%rbp)
1403ad6dd: c6 85 71 01 00 00 01        	movb	$0x1, 0x171(%rbp)
1403ad6e4: c6 85 74 01 00 00 01        	movb	$0x1, 0x174(%rbp)
1403ad6eb: c6 85 73 01 00 00 01        	movb	$0x1, 0x173(%rbp)
1403ad6f2: 48 89 f9                    	movq	%rdi, %rcx
1403ad6f5: e8 d6 97 9c 00              	callq	0x140d76ed0 <.text+0xd75ed0>
1403ad6fa: 49 89 d0                    	movq	%rdx, %r8
1403ad6fd: 48 8d 9d 50 01 00 00        	leaq	0x150(%rbp), %rbx
1403ad704: 48 89 d9                    	movq	%rbx, %rcx
1403ad707: 48 89 c2                    	movq	%rax, %rdx
1403ad70a: e8 b1 c6 9a 00              	callq	0x140d59dc0 <.text+0xd58dc0>
1403ad70f: 48 89 d7                    	movq	%rdx, %rdi
1403ad712: a8 01                       	testb	$0x1, %al
1403ad714: 0f 84 40 01 00 00           	je	0x1403ad85a <.text+0x3ac85a>
1403ad71a: 48 89 bd 90 00 00 00        	movq	%rdi, 0x90(%rbp)
1403ad721: 48 8d 85 90 00 00 00        	leaq	0x90(%rbp), %rax
1403ad728: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
1403ad72f: 48 8d 05 da d8 9a 00        	leaq	0x9ad8da(%rip), %rax    # 0x140d5b010 <.text+0xd5a010>
1403ad736: 48 89 85 08 01 00 00        	movq	%rax, 0x108(%rbp)
1403ad73d: 48 c7 85 50 01 00 00 02 00 00 00    	movq	$0x2, 0x150(%rbp)
1403ad748: 48 c7 85 60 01 00 00 02 00 00 00    	movq	$0x2, 0x160(%rbp)
1403ad753: 48 c7 85 70 01 00 00 00 00 00 00    	movq	$0x0, 0x170(%rbp)
1403ad75e: 48 b8 20 00 00 00 04 00 00 00       	movabsq	$0x400000020, %rax # imm = 0x400000020
1403ad768: 48 89 85 78 01 00 00        	movq	%rax, 0x178(%rbp)
1403ad76f: c6 85 80 01 00 00 03        	movb	$0x3, 0x180(%rbp)
1403ad776: 48 8d 05 f3 e2 61 02        	leaq	0x261e2f3(%rip), %rax   # 0x1429cba70
1403ad77d: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
1403ad781: 48 c7 45 f8 01 00 00 00     	movq	$0x1, -0x8(%rbp)
1403ad789: 48 8d 85 00 01 00 00        	leaq	0x100(%rbp), %rax
1403ad790: 48 89 45 00                 	movq	%rax, (%rbp)
1403ad794: 48 c7 45 08 01 00 00 00     	movq	$0x1, 0x8(%rbp)
1403ad79c: 48 89 5d 10                 	movq	%rbx, 0x10(%rbp)
1403ad7a0: 48 c7 45 18 01 00 00 00     	movq	$0x1, 0x18(%rbp)
1403ad7a8: 48 8d 8d b8 00 00 00        	leaq	0xb8(%rbp), %rcx
1403ad7af: 48 8d 55 f0                 	leaq	-0x10(%rbp), %rdx
1403ad7b3: e8 d8 c7 9c 00              	callq	0x140d79f90 <.text+0xd78f90>
1403ad7b8: 48 8b 9d b8 00 00 00        	movq	0xb8(%rbp), %rbx
1403ad7bf: 48 8b bd c0 00 00 00        	movq	0xc0(%rbp), %rdi
1403ad7c6: 4c 8b b5 c8 00 00 00        	movq	0xc8(%rbp), %r14
1403ad7cd: 48 8b 85 90 00 00 00        	movq	0x90(%rbp), %rax
1403ad7d4: 89 c1                       	movl	%eax, %ecx
1403ad7d6: 83 e1 03                    	andl	$0x3, %ecx
1403ad7d9: 83 f9 01                    	cmpl	$0x1, %ecx
1403ad7dc: 75 69                       	jne	0x1403ad847 <.text+0x3ac847>
1403ad7de: 48 8d 48 ff                 	leaq	-0x1(%rax), %rcx
1403ad7e2: 48 89 8d c0 01 00 00        	movq	%rcx, 0x1c0(%rbp)
1403ad7e9: 48 8b 48 ff                 	movq	-0x1(%rax), %rcx
1403ad7ed: 48 89 8d d8 01 00 00        	movq	%rcx, 0x1d8(%rbp)
1403ad7f4: 48 8b 40 07                 	movq	0x7(%rax), %rax
1403ad7f8: 48 89 85 d0 01 00 00        	movq	%rax, 0x1d0(%rbp)
1403ad7ff: 48 8b 00                    	movq	(%rax), %rax
1403ad802: 48 85 c0                    	testq	%rax, %rax
1403ad805: 74 09                       	je	0x1403ad810 <.text+0x3ac810>
1403ad807: 48 8b 8d d8 01 00 00        	movq	0x1d8(%rbp), %rcx
1403ad80e: ff d0                       	callq	*%rax
1403ad810: 48 8b 8d d8 01 00 00        	movq	0x1d8(%rbp), %rcx
1403ad817: 48 8b 85 d0 01 00 00        	movq	0x1d0(%rbp), %rax
1403ad81e: 48 8b 50 08                 	movq	0x8(%rax), %rdx
1403ad822: 48 85 d2                    	testq	%rdx, %rdx
1403ad825: 74 09                       	je	0x1403ad830 <.text+0x3ac830>
1403ad827: 4c 8b 40 10                 	movq	0x10(%rax), %r8
1403ad82b: e8 f0 63 26 00              	callq	0x140613c20 <.text+0x612c20>
1403ad830: ba 18 00 00 00              	movl	$0x18, %edx
1403ad835: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1403ad83b: 48 8b 8d c0 01 00 00        	movq	0x1c0(%rbp), %rcx
1403ad842: e8 d9 63 26 00              	callq	0x140613c20 <.text+0x612c20>
1403ad847: 48 b8 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %rax # imm = 0x8000000000000001
1403ad851: 48 39 c3                    	cmpq	%rax, %rbx
1403ad854: 0f 85 9b 02 00 00           	jne	0x1403adaf5 <.text+0x3acaf5>
1403ad85a: 48 89 bd c8 01 00 00        	movq	%rdi, 0x1c8(%rbp)
1403ad861: 4d 8b 6f 18                 	movq	0x18(%r15), %r13
1403ad865: 49 83 7d 10 00              	cmpq	$0x0, 0x10(%r13)
1403ad86a: 0f 84 1d 02 00 00           	je	0x1403ada8d <.text+0x3aca8d>
1403ad870: e8 ab 69 6d 00              	callq	0x140a84220 <.text+0xa83220>
1403ad875: 48 89 85 d8 01 00 00        	movq	%rax, 0x1d8(%rbp)
1403ad87c: 41 8b 45 70                 	movl	0x70(%r13), %eax
1403ad880: 89 85 d0 01 00 00           	movl	%eax, 0x1d0(%rbp)
1403ad886: 49 8b 57 10                 	movq	0x10(%r15), %rdx
1403ad88a: 48 8d 8d 90 00 00 00        	leaq	0x90(%rbp), %rcx
1403ad891: e8 7a 3e 56 00              	callq	0x140911710 <.text+0x910710>
1403ad896: 49 8b 47 28                 	movq	0x28(%r15), %rax
1403ad89a: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
1403ad8a1: 41 8b 45 68                 	movl	0x68(%r13), %eax
1403ad8a5: 89 85 b4 01 00 00           	movl	%eax, 0x1b4(%rbp)
1403ad8ab: 41 8b 5d 6c                 	movl	0x6c(%r13), %ebx
1403ad8af: 4c 89 e9                    	movq	%r13, %rcx
1403ad8b2: e8 39 44 78 00              	callq	0x140b31cf0 <.text+0xb30cf0>
1403ad8b7: 49 89 c7                    	movq	%rax, %r15
1403ad8ba: 49 89 d4                    	movq	%rdx, %r12
1403ad8bd: 4c 89 e9                    	movq	%r13, %rcx
1403ad8c0: e8 7b 71 6d 00              	callq	0x140a84a40 <.text+0xa83a40>
1403ad8c5: 49 89 d0                    	movq	%rdx, %r8
1403ad8c8: 4c 8d 0d 31 06 59 02        	leaq	0x2590631(%rip), %r9    # 0x14293df00
1403ad8cf: 48 8d 8d 98 01 00 00        	leaq	0x198(%rbp), %rcx
1403ad8d6: 48 89 c2                    	movq	%rax, %rdx
1403ad8d9: e8 82 fa f0 ff              	callq	0x1402bd360 <.text+0x2bc360>
1403ad8de: 48 8b bd a0 01 00 00        	movq	0x1a0(%rbp), %rdi
1403ad8e5: 4c 8b b5 a8 01 00 00        	movq	0x1a8(%rbp), %r14
1403ad8ec: 4c 8d 6d f0                 	leaq	-0x10(%rbp), %r13
1403ad8f0: 4c 89 e9                    	movq	%r13, %rcx
1403ad8f3: e8 08 7d 51 00              	callq	0x1408c5600 <.text+0x8c4600>
1403ad8f8: 4c 89 6c 24 68              	movq	%r13, 0x68(%rsp)
1403ad8fd: 4c 89 74 24 58              	movq	%r14, 0x58(%rsp)
1403ad902: 48 89 7c 24 50              	movq	%rdi, 0x50(%rsp)
1403ad907: 4c 89 64 24 48              	movq	%r12, 0x48(%rsp)
1403ad90c: 4c 89 7c 24 40              	movq	%r15, 0x40(%rsp)
1403ad911: 89 5c 24 38                 	movl	%ebx, 0x38(%rsp)
1403ad915: 8b 85 b4 01 00 00           	movl	0x1b4(%rbp), %eax
1403ad91b: 89 44 24 30                 	movl	%eax, 0x30(%rsp)
1403ad91f: 48 8b 85 c0 01 00 00        	movq	0x1c0(%rbp), %rax
1403ad926: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
1403ad92b: 48 8d 85 90 00 00 00        	leaq	0x90(%rbp), %rax
1403ad932: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1403ad937: 48 c7 44 24 60 00 00 00 00  	movq	$0x0, 0x60(%rsp)
1403ad940: 48 8d 8d 50 01 00 00        	leaq	0x150(%rbp), %rcx
1403ad947: 48 8d 95 c8 01 00 00        	leaq	0x1c8(%rbp), %rdx
1403ad94e: 4c 8b 85 d8 01 00 00        	movq	0x1d8(%rbp), %r8
1403ad955: 44 8b 8d d0 01 00 00        	movl	0x1d0(%rbp), %r9d
1403ad95c: e8 3f 43 e9 ff              	callq	0x140241ca0 <.text+0x240ca0>
1403ad961: 48 8b bd 50 01 00 00        	movq	0x150(%rbp), %rdi
1403ad968: 48 8b 9d 58 01 00 00        	movq	0x158(%rbp), %rbx
1403ad96f: 48 89 f8                    	movq	%rdi, %rax
1403ad972: 48 f7 d8                    	negq	%rax
1403ad975: 0f 81 ba 01 00 00           	jno	0x1403adb35 <.text+0x3acb35>
1403ad97b: 48 89 9d b8 01 00 00        	movq	%rbx, 0x1b8(%rbp)
1403ad982: 48 8d 85 b8 01 00 00        	leaq	0x1b8(%rbp), %rax
1403ad989: 48 89 85 f0 00 00 00        	movq	%rax, 0xf0(%rbp)
1403ad990: 48 8d 05 79 f2 99 00        	leaq	0x99f279(%rip), %rax    # 0x140d4cc10 <.text+0xd4bc10>
1403ad997: 48 89 85 f8 00 00 00        	movq	%rax, 0xf8(%rbp)
1403ad99e: 48 c7 85 b8 00 00 00 02 00 00 00    	movq	$0x2, 0xb8(%rbp)
1403ad9a9: 48 c7 85 c8 00 00 00 02 00 00 00    	movq	$0x2, 0xc8(%rbp)
1403ad9b4: 48 c7 85 d8 00 00 00 00 00 00 00    	movq	$0x0, 0xd8(%rbp)
1403ad9bf: 48 b8 20 00 00 00 04 00 00 00       	movabsq	$0x400000020, %rax # imm = 0x400000020
1403ad9c9: 48 89 85 e0 00 00 00        	movq	%rax, 0xe0(%rbp)
1403ad9d0: c6 85 e8 00 00 00 03        	movb	$0x3, 0xe8(%rbp)
1403ad9d7: 48 8d 05 92 e0 61 02        	leaq	0x261e092(%rip), %rax   # 0x1429cba70
1403ad9de: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
1403ad9e5: 48 c7 85 08 01 00 00 01 00 00 00    	movq	$0x1, 0x108(%rbp)
1403ad9f0: 48 8d 85 f0 00 00 00        	leaq	0xf0(%rbp), %rax
1403ad9f7: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
1403ad9fe: 48 c7 85 18 01 00 00 01 00 00 00    	movq	$0x1, 0x118(%rbp)
1403ada09: 48 8d 85 b8 00 00 00        	leaq	0xb8(%rbp), %rax
1403ada10: 48 89 85 20 01 00 00        	movq	%rax, 0x120(%rbp)
1403ada17: 48 c7 85 28 01 00 00 01 00 00 00    	movq	$0x1, 0x128(%rbp)
1403ada22: 48 8d 4d 78                 	leaq	0x78(%rbp), %rcx
1403ada26: 48 8d 95 00 01 00 00        	leaq	0x100(%rbp), %rdx
1403ada2d: e8 5e c5 9c 00              	callq	0x140d79f90 <.text+0xd78f90>
1403ada32: 48 8b 7d 78                 	movq	0x78(%rbp), %rdi
1403ada36: 48 8b 9d 80 00 00 00        	movq	0x80(%rbp), %rbx
1403ada3d: 4c 8b b5 88 00 00 00        	movq	0x88(%rbp), %r14
1403ada44: 48 8d 8d b8 01 00 00        	leaq	0x1b8(%rbp), %rcx
1403ada4b: e8 e0 f1 99 00              	callq	0x140d4cc30 <.text+0xd4bc30>
1403ada50: 48 89 7e 08                 	movq	%rdi, 0x8(%rsi)
1403ada54: 48 89 5e 10                 	movq	%rbx, 0x10(%rsi)
1403ada58: 4c 89 76 18                 	movq	%r14, 0x18(%rsi)
1403ada5c: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1403ada66: 48 89 06                    	movq	%rax, (%rsi)
1403ada69: 48 8b 95 98 01 00 00        	movq	0x198(%rbp), %rdx
1403ada70: 48 85 d2                    	testq	%rdx, %rdx
1403ada73: 74 62                       	je	0x1403adad7 <.text+0x3acad7>
1403ada75: 48 8b 8d a0 01 00 00        	movq	0x1a0(%rbp), %rcx
1403ada7c: 48 c1 e2 04                 	shlq	$0x4, %rdx
1403ada80: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1403ada86: e8 95 61 26 00              	callq	0x140613c20 <.text+0x612c20>
1403ada8b: eb 4a                       	jmp	0x1403adad7 <.text+0x3acad7>
1403ada8d: 0f b6 05 75 71 28 03        	movzbl	0x3287175(%rip), %eax   # 0x143634c09
1403ada94: b9 10 00 00 00              	movl	$0x10, %ecx
1403ada99: ba 01 00 00 00              	movl	$0x1, %edx
1403ada9e: e8 6d 61 26 00              	callq	0x140613c10 <.text+0x612c10>
1403adaa3: 48 85 c0                    	testq	%rax, %rax
1403adaa6: 0f 84 35 03 00 00           	je	0x1403adde1 <.text+0x3acde1>
1403adaac: 0f 10 05 d6 df 61 02        	movups	0x261dfd6(%rip), %xmm0  # 0x1429cba89
1403adab3: 0f 11 00                    	movups	%xmm0, (%rax)
1403adab6: 48 c7 46 08 10 00 00 00     	movq	$0x10, 0x8(%rsi)
1403adabe: 48 89 46 10                 	movq	%rax, 0x10(%rsi)
1403adac2: 48 c7 46 18 10 00 00 00     	movq	$0x10, 0x18(%rsi)
1403adaca: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1403adad4: 48 89 06                    	movq	%rax, (%rsi)
1403adad7: 48 8b 8d c8 01 00 00        	movq	0x1c8(%rbp), %rcx
1403adade: ff 15 0c 49 de 02           	callq	*0x2de490c(%rip)        # 0x1431923f0
1403adae4: 48 8b 95 30 01 00 00        	movq	0x130(%rbp), %rdx
1403adaeb: 48 85 d2                    	testq	%rdx, %rdx
1403adaee: 75 2e                       	jne	0x1403adb1e <.text+0x3acb1e>
1403adaf0: e9 d5 02 00 00              	jmp	0x1403addca <.text+0x3acdca>
1403adaf5: 48 89 5e 08                 	movq	%rbx, 0x8(%rsi)
1403adaf9: 48 89 7e 10                 	movq	%rdi, 0x10(%rsi)
1403adafd: 4c 89 76 18                 	movq	%r14, 0x18(%rsi)
1403adb01: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1403adb0b: 48 89 06                    	movq	%rax, (%rsi)
1403adb0e: 48 8b 95 30 01 00 00        	movq	0x130(%rbp), %rdx
1403adb15: 48 85 d2                    	testq	%rdx, %rdx
1403adb18: 0f 84 ac 02 00 00           	je	0x1403addca <.text+0x3acdca>
1403adb1e: 48 8b 8d 38 01 00 00        	movq	0x138(%rbp), %rcx
1403adb25: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403adb2b: e8 f0 60 26 00              	callq	0x140613c20 <.text+0x612c20>
1403adb30: e9 95 02 00 00              	jmp	0x1403addca <.text+0x3acdca>
1403adb35: 4c 8b 8d 70 01 00 00        	movq	0x170(%rbp), %r9
1403adb3c: 4d 85 c9                    	testq	%r9, %r9
1403adb3f: 74 35                       	je	0x1403adb76 <.text+0x3acb76>
1403adb41: 49 8d 41 01                 	leaq	0x1(%r9), %rax
1403adb45: b9 18 00 00 00              	movl	$0x18, %ecx
1403adb4a: 48 f7 e1                    	mulq	%rcx
1403adb4d: 48 83 c0 0f                 	addq	$0xf, %rax
1403adb51: 48 83 e0 f0                 	andq	$-0x10, %rax
1403adb55: 49 01 c1                    	addq	%rax, %r9
1403adb58: 49 83 c1 11                 	addq	$0x11, %r9
1403adb5c: 74 18                       	je	0x1403adb76 <.text+0x3acb76>
1403adb5e: 48 8b 8d 68 01 00 00        	movq	0x168(%rbp), %rcx
1403adb65: 48 29 c1                    	subq	%rax, %rcx
1403adb68: 41 b8 10 00 00 00           	movl	$0x10, %r8d
1403adb6e: 4c 89 ca                    	movq	%r9, %rdx
1403adb71: e8 aa 60 26 00              	callq	0x140613c20 <.text+0x612c20>
1403adb76: 48 85 ff                    	testq	%rdi, %rdi
1403adb79: 4c 8b 35 f0 e4 27 03        	movq	0x327e4f0(%rip), %r14   # 0x14362c070
1403adb80: 74 15                       	je	0x1403adb97 <.text+0x3acb97>
1403adb82: 48 c1 e7 03                 	shlq	$0x3, %rdi
1403adb86: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1403adb8c: 48 89 d9                    	movq	%rbx, %rcx
1403adb8f: 48 89 fa                    	movq	%rdi, %rdx
1403adb92: e8 89 60 26 00              	callq	0x140613c20 <.text+0x612c20>
1403adb97: 48 8b 95 98 01 00 00        	movq	0x198(%rbp), %rdx
1403adb9e: 48 85 d2                    	testq	%rdx, %rdx
1403adba1: 74 16                       	je	0x1403adbb9 <.text+0x3acbb9>
1403adba3: 48 8b 8d a0 01 00 00        	movq	0x1a0(%rbp), %rcx
1403adbaa: 48 c1 e2 04                 	shlq	$0x4, %rdx
1403adbae: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1403adbb4: e8 67 60 26 00              	callq	0x140613c20 <.text+0x612c20>
1403adbb9: 41 83 3e 01                 	cmpl	$0x1, (%r14)
1403adbbd: 75 76                       	jne	0x1403adc35 <.text+0x3acc35>
1403adbbf: 4d 8b 76 08                 	movq	0x8(%r14), %r14
1403adbc3: 4d 85 f6                    	testq	%r14, %r14
1403adbc6: 0f 84 2d 02 00 00           	je	0x1403addf9 <.text+0x3acdf9>
1403adbcc: 4d 8b 66 10                 	movq	0x10(%r14), %r12
1403adbd0: 4d 85 e4                    	testq	%r12, %r12
1403adbd3: 0f 88 2e 02 00 00           	js	0x1403ade07 <.text+0x3ace07>
1403adbd9: 41 0f b6 5e 58              	movzbl	0x58(%r14), %ebx
1403adbde: 74 64                       	je	0x1403adc44 <.text+0x3acc44>
1403adbe0: 4d 8b 7e 08                 	movq	0x8(%r14), %r15
1403adbe4: 0f b6 05 1e 70 28 03        	movzbl	0x328701e(%rip), %eax   # 0x143634c09
1403adbeb: bf 01 00 00 00              	movl	$0x1, %edi
1403adbf0: ba 01 00 00 00              	movl	$0x1, %edx
1403adbf5: 4c 89 e1                    	movq	%r12, %rcx
1403adbf8: e8 13 60 26 00              	callq	0x140613c10 <.text+0x612c10>
1403adbfd: 48 85 c0                    	testq	%rax, %rax
1403adc00: 0f 84 03 02 00 00           	je	0x1403ade09 <.text+0x3ace09>
1403adc06: 4d 8d 4c 24 ff              	leaq	-0x1(%r12), %r9
1403adc0b: 4d 39 cc                    	cmpq	%r9, %r12
1403adc0e: 4d 0f 42 cc                 	cmovbq	%r12, %r9
1403adc12: 49 83 f9 20                 	cmpq	$0x20, %r9
1403adc16: 0f 92 c1                    	setb	%cl
1403adc19: 48 89 c2                    	movq	%rax, %rdx
1403adc1c: 4c 29 fa                    	subq	%r15, %rdx
1403adc1f: 48 83 fa 20                 	cmpq	$0x20, %rdx
1403adc23: 0f 92 c2                    	setb	%dl
1403adc26: 08 ca                       	orb	%cl, %dl
1403adc28: 74 21                       	je	0x1403adc4b <.text+0x3acc4b>
1403adc2a: 45 31 c9                    	xorl	%r9d, %r9d
1403adc2d: 4c 89 e1                    	movq	%r12, %rcx
1403adc30: 4c 89 fa                    	movq	%r15, %rdx
1403adc33: eb 5a                       	jmp	0x1403adc8f <.text+0x3acc8f>
1403adc35: 49 bc 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %r12 # imm = 0x8000000000000000
1403adc3f: e9 16 01 00 00              	jmp	0x1403add5a <.text+0x3acd5a>
1403adc44: b8 01 00 00 00              	movl	$0x1, %eax
1403adc49: eb 6e                       	jmp	0x1403adcb9 <.text+0x3accb9>
1403adc4b: 49 ff c1                    	incq	%r9
1403adc4e: 44 89 c9                    	movl	%r9d, %ecx
1403adc51: 83 e1 1f                    	andl	$0x1f, %ecx
1403adc54: ba 20 00 00 00              	movl	$0x20, %edx
1403adc59: 48 0f 45 d1                 	cmovneq	%rcx, %rdx
1403adc5d: 49 29 d1                    	subq	%rdx, %r9
1403adc60: 4c 89 e1                    	movq	%r12, %rcx
1403adc63: 4c 29 c9                    	subq	%r9, %rcx
1403adc66: 4b 8d 14 0f                 	leaq	(%r15,%r9), %rdx
1403adc6a: 45 31 c0                    	xorl	%r8d, %r8d
1403adc6d: 0f 1f 00                    	nopl	(%rax)
1403adc70: 43 0f 10 04 07              	movups	(%r15,%r8), %xmm0
1403adc75: 43 0f 10 4c 07 10           	movups	0x10(%r15,%r8), %xmm1
1403adc7b: 42 0f 11 04 00              	movups	%xmm0, (%rax,%r8)
1403adc80: 42 0f 11 4c 00 10           	movups	%xmm1, 0x10(%rax,%r8)
1403adc86: 49 83 c0 20                 	addq	$0x20, %r8
1403adc8a: 4d 39 c1                    	cmpq	%r8, %r9
1403adc8d: 75 e1                       	jne	0x1403adc70 <.text+0x3acc70>
1403adc8f: 4d 01 e7                    	addq	%r12, %r15
1403adc92: 49 89 c0                    	movq	%rax, %r8
1403adc95: 4d 01 c8                    	addq	%r9, %r8
1403adc98: 45 31 d2                    	xorl	%r10d, %r10d
1403adc9b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
1403adca0: 4e 8d 0c 12                 	leaq	(%rdx,%r10), %r9
1403adca4: 4d 39 f9                    	cmpq	%r15, %r9
1403adca7: 74 10                       	je	0x1403adcb9 <.text+0x3accb9>
1403adca9: 45 0f b6 09                 	movzbl	(%r9), %r9d
1403adcad: 47 88 0c 10                 	movb	%r9b, (%r8,%r10)
1403adcb1: 49 ff c2                    	incq	%r10
1403adcb4: 4c 39 d1                    	cmpq	%r10, %rcx
1403adcb7: 75 e7                       	jne	0x1403adca0 <.text+0x3acca0>
1403adcb9: 31 c9                       	xorl	%ecx, %ecx
1403adcbb: 49 3b 4e 18                 	cmpq	0x18(%r14), %rcx
1403adcbf: 71 0f                       	jno	0x1403adcd0 <.text+0x3accd0>
1403adcc1: 48 ba 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rdx # imm = 0x8000000000000000
1403adccb: e9 81 00 00 00              	jmp	0x1403add51 <.text+0x3acd51>
1403adcd0: 48 89 85 d0 01 00 00        	movq	%rax, 0x1d0(%rbp)
1403adcd7: 4c 89 a5 d8 01 00 00        	movq	%r12, 0x1d8(%rbp)
1403adcde: 49 8d 56 18                 	leaq	0x18(%r14), %rdx
1403adce2: 41 8a 7e 40                 	movb	0x40(%r14), %dil
1403adce6: 48 8d 8d 50 01 00 00        	leaq	0x150(%rbp), %rcx
1403adced: e8 9e e0 9c 00              	callq	0x140d7bd90 <.text+0xd7ad90>
1403adcf2: 40 88 bd 78 01 00 00        	movb	%dil, 0x178(%rbp)
1403adcf9: 41 0f 10 46 30              	movups	0x30(%r14), %xmm0
1403adcfe: 0f 11 85 68 01 00 00        	movups	%xmm0, 0x168(%rbp)
1403add05: f2 41 0f 10 4e 48           	movsd	0x48(%r14), %xmm1
1403add0b: f3 41 0f 10 46 50           	movss	0x50(%r14), %xmm0
1403add11: 45 0f b6 4e 54              	movzbl	0x54(%r14), %r9d
1403add16: 41 0f b6 4e 55              	movzbl	0x55(%r14), %ecx
1403add1b: 48 8b 95 50 01 00 00        	movq	0x150(%rbp), %rdx
1403add22: 0f 10 95 58 01 00 00        	movups	0x158(%rbp), %xmm2
1403add29: 0f 29 55 50                 	movaps	%xmm2, 0x50(%rbp)
1403add2d: 0f 10 95 68 01 00 00        	movups	0x168(%rbp), %xmm2
1403add34: 0f 29 55 60                 	movaps	%xmm2, 0x60(%rbp)
1403add38: 4c 8b 85 78 01 00 00        	movq	0x178(%rbp), %r8
1403add3f: 4c 89 45 70                 	movq	%r8, 0x70(%rbp)
1403add43: 4c 8b a5 d8 01 00 00        	movq	0x1d8(%rbp), %r12
1403add4a: 48 8b 85 d0 01 00 00        	movq	0x1d0(%rbp), %rax
1403add51: 41 0f 10 56 60              	movups	0x60(%r14), %xmm2
1403add56: 0f 11 55 40                 	movups	%xmm2, 0x40(%rbp)
1403add5a: 0f 10 95 30 01 00 00        	movups	0x130(%rbp), %xmm2
1403add61: 0f 10 9d 40 01 00 00        	movups	0x140(%rbp), %xmm3
1403add68: 0f 11 5e 10                 	movups	%xmm3, 0x10(%rsi)
1403add6c: 0f 11 16                    	movups	%xmm2, (%rsi)
1403add6f: 4c 89 66 20                 	movq	%r12, 0x20(%rsi)
1403add73: 48 89 46 28                 	movq	%rax, 0x28(%rsi)
1403add77: 4c 89 66 30                 	movq	%r12, 0x30(%rsi)
1403add7b: 48 89 56 38                 	movq	%rdx, 0x38(%rsi)
1403add7f: 0f 28 55 50                 	movaps	0x50(%rbp), %xmm2
1403add83: 0f 28 5d 60                 	movaps	0x60(%rbp), %xmm3
1403add87: 0f 11 56 40                 	movups	%xmm2, 0x40(%rsi)
1403add8b: 0f 11 5e 50                 	movups	%xmm3, 0x50(%rsi)
1403add8f: 48 8b 55 70                 	movq	0x70(%rbp), %rdx
1403add93: 48 89 56 60                 	movq	%rdx, 0x60(%rsi)
1403add97: 0f 13 4e 68                 	movlps	%xmm1, 0x68(%rsi)
1403add9b: f3 0f 11 46 70              	movss	%xmm0, 0x70(%rsi)
1403adda0: 44 88 4e 74                 	movb	%r9b, 0x74(%rsi)
1403adda4: 88 4e 75                    	movb	%cl, 0x75(%rsi)
1403adda7: 88 5e 78                    	movb	%bl, 0x78(%rsi)
1403addaa: 0f 10 45 39                 	movups	0x39(%rbp), %xmm0
1403addae: 0f 11 46 79                 	movups	%xmm0, 0x79(%rsi)
1403addb2: 48 8b 45 48                 	movq	0x48(%rbp), %rax
1403addb6: 48 89 86 88 00 00 00        	movq	%rax, 0x88(%rsi)
1403addbd: 48 8b 8d c8 01 00 00        	movq	0x1c8(%rbp), %rcx
1403addc4: ff 15 26 46 de 02           	callq	*0x2de4626(%rip)        # 0x1431923f0
1403addca: 48 89 f0                    	movq	%rsi, %rax
1403addcd: 48 81 c4 68 02 00 00        	addq	$0x268, %rsp            # imm = 0x268
1403addd4: 5b                          	popq	%rbx
1403addd5: 5f                          	popq	%rdi
1403addd6: 5e                          	popq	%rsi
1403addd7: 41 5c                       	popq	%r12
1403addd9: 41 5d                       	popq	%r13
1403adddb: 41 5e                       	popq	%r14
1403adddd: 41 5f                       	popq	%r15
1403adddf: 5d                          	popq	%rbp
1403adde0: c3                          	retq
1403adde1: 4c 8d 05 a0 d9 61 02        	leaq	0x261d9a0(%rip), %r8    # 0x1429cb788
1403adde8: b9 01 00 00 00              	movl	$0x1, %ecx
1403added: ba 10 00 00 00              	movl	$0x10, %edx
1403addf2: e8 ec b1 52 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1403addf7: eb 22                       	jmp	0x1403ade1b <.text+0x3ace1b>
1403addf9: 48 8d 0d a0 dc 61 02        	leaq	0x261dca0(%rip), %rcx   # 0x1429cbaa0
1403ade00: e8 0b b5 52 02              	callq	0x1428d9310 <.text+0x28d8310>
1403ade05: eb 14                       	jmp	0x1403ade1b <.text+0x3ace1b>
1403ade07: 31 ff                       	xorl	%edi, %edi
1403ade09: 4c 8d 05 40 06 60 02        	leaq	0x2600640(%rip), %r8    # 0x1429ae450
1403ade10: 48 89 f9                    	movq	%rdi, %rcx
1403ade13: 4c 89 e2                    	movq	%r12, %rdx
1403ade16: e8 c8 b1 52 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1403ade1b: 0f 0b                       	ud2
1403ade1d: 0f 1f 00                    	nopl	(%rax)
1403ade20: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1403ade25: 55                          	pushq	%rbp
1403ade26: 41 57                       	pushq	%r15
1403ade28: 41 56                       	pushq	%r14
1403ade2a: 41 55                       	pushq	%r13
1403ade2c: 41 54                       	pushq	%r12
1403ade2e: 56                          	pushq	%rsi
1403ade2f: 57                          	pushq	%rdi
1403ade30: 53                          	pushq	%rbx
1403ade31: 48 83 ec 78                 	subq	$0x78, %rsp
1403ade35: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1403ade3c: 48 8b 95 30 01 00 00        	movq	0x130(%rbp), %rdx
1403ade43: 48 85 d2                    	testq	%rdx, %rdx
1403ade46: 74 12                       	je	0x1403ade5a <.text+0x3ace5a>
1403ade48: 48 8b 8d 38 01 00 00        	movq	0x138(%rbp), %rcx
1403ade4f: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403ade55: e8 c6 5d 26 00              	callq	0x140613c20 <.text+0x612c20>
1403ade5a: 90                          	nop
1403ade5b: 48 83 c4 78                 	addq	$0x78, %rsp
1403ade5f: 5b                          	popq	%rbx
1403ade60: 5f                          	popq	%rdi
1403ade61: 5e                          	popq	%rsi
1403ade62: 41 5c                       	popq	%r12
1403ade64: 41 5d                       	popq	%r13
1403ade66: 41 5e                       	popq	%r14
1403ade68: 41 5f                       	popq	%r15
1403ade6a: 5d                          	popq	%rbp
1403ade6b: c3                          	retq
1403ade6c: 0f 1f 40 00                 	nopl	(%rax)
1403ade70: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1403ade75: 55                          	pushq	%rbp
1403ade76: 41 57                       	pushq	%r15
1403ade78: 41 56                       	pushq	%r14
1403ade7a: 41 55                       	pushq	%r13
1403ade7c: 41 54                       	pushq	%r12
1403ade7e: 56                          	pushq	%rsi
1403ade7f: 57                          	pushq	%rdi
1403ade80: 53                          	pushq	%rbx
1403ade81: 48 83 ec 78                 	subq	$0x78, %rsp
1403ade85: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1403ade8c: 48 8d 8d 90 00 00 00        	leaq	0x90(%rbp), %rcx
1403ade93: e8 68 9b 23 00              	callq	0x1405e7a00 <.text+0x5e6a00>
1403ade98: 90                          	nop
1403ade99: 48 83 c4 78                 	addq	$0x78, %rsp
1403ade9d: 5b                          	popq	%rbx
1403ade9e: 5f                          	popq	%rdi
1403ade9f: 5e                          	popq	%rsi
1403adea0: 41 5c                       	popq	%r12
1403adea2: 41 5d                       	popq	%r13
1403adea4: 41 5e                       	popq	%r14
1403adea6: 41 5f                       	popq	%r15
1403adea8: 5d                          	popq	%rbp
1403adea9: c3                          	retq
1403adeaa: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1403adeb0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1403adeb5: 55                          	pushq	%rbp
1403adeb6: 41 57                       	pushq	%r15
1403adeb8: 41 56                       	pushq	%r14
1403adeba: 41 55                       	pushq	%r13
1403adebc: 41 54                       	pushq	%r12
1403adebe: 56                          	pushq	%rsi
1403adebf: 57                          	pushq	%rdi
1403adec0: 53                          	pushq	%rbx
1403adec1: 48 83 ec 78                 	subq	$0x78, %rsp
1403adec5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1403adecc: 48 8b 8d d8 01 00 00        	movq	0x1d8(%rbp), %rcx
1403aded3: 48 8b 85 d0 01 00 00        	movq	0x1d0(%rbp), %rax
1403adeda: 48 8b 50 08                 	movq	0x8(%rax), %rdx
1403adede: 48 85 d2                    	testq	%rdx, %rdx
1403adee1: 74 10                       	je	0x1403adef3 <.text+0x3acef3>
1403adee3: 48 8b 85 d0 01 00 00        	movq	0x1d0(%rbp), %rax
1403adeea: 4c 8b 40 10                 	movq	0x10(%rax), %r8
1403adeee: e8 2d 5d 26 00              	callq	0x140613c20 <.text+0x612c20>
1403adef3: ba 18 00 00 00              	movl	$0x18, %edx
1403adef8: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1403adefe: 48 8b 8d c0 01 00 00        	movq	0x1c0(%rbp), %rcx
1403adf05: e8 16 5d 26 00              	callq	0x140613c20 <.text+0x612c20>
1403adf0a: 90                          	nop
1403adf0b: 48 83 c4 78                 	addq	$0x78, %rsp
1403adf0f: 5b                          	popq	%rbx
1403adf10: 5f                          	popq	%rdi
1403adf11: 5e                          	popq	%rsi
1403adf12: 41 5c                       	popq	%r12
1403adf14: 41 5d                       	popq	%r13
1403adf16: 41 5e                       	popq	%r14
1403adf18: 41 5f                       	popq	%r15
1403adf1a: 5d                          	popq	%rbp
1403adf1b: c3                          	retq
1403adf1c: 0f 1f 40 00                 	nopl	(%rax)
1403adf20: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1403adf25: 55                          	pushq	%rbp
1403adf26: 41 57                       	pushq	%r15
1403adf28: 41 56                       	pushq	%r14
1403adf2a: 41 55                       	pushq	%r13
1403adf2c: 41 54                       	pushq	%r12
1403adf2e: 56                          	pushq	%rsi
1403adf2f: 57                          	pushq	%rdi
1403adf30: 53                          	pushq	%rbx
1403adf31: 48 83 ec 78                 	subq	$0x78, %rsp
1403adf35: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1403adf3c: 48 8b 8d c8 01 00 00        	movq	0x1c8(%rbp), %rcx
1403adf43: ff 15 a7 44 de 02           	callq	*0x2de44a7(%rip)        # 0x1431923f0
1403adf49: 90                          	nop
1403adf4a: 48 83 c4 78                 	addq	$0x78, %rsp
1403adf4e: 5b                          	popq	%rbx
1403adf4f: 5f                          	popq	%rdi
1403adf50: 5e                          	popq	%rsi
1403adf51: 41 5c                       	popq	%r12
1403adf53: 41 5d                       	popq	%r13
1403adf55: 41 5e                       	popq	%r14
1403adf57: 41 5f                       	popq	%r15
1403adf59: 5d                          	popq	%rbp
1403adf5a: c3                          	retq
1403adf5b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
1403adf60: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1403adf65: 55                          	pushq	%rbp
1403adf66: 41 57                       	pushq	%r15
1403adf68: 41 56                       	pushq	%r14
1403adf6a: 41 55                       	pushq	%r13
1403adf6c: 41 54                       	pushq	%r12
1403adf6e: 56                          	pushq	%rsi
1403adf6f: 57                          	pushq	%rdi
1403adf70: 53                          	pushq	%rbx
1403adf71: 48 83 ec 78                 	subq	$0x78, %rsp
1403adf75: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1403adf7c: 48 8b 95 98 01 00 00        	movq	0x198(%rbp), %rdx
1403adf83: 48 85 d2                    	testq	%rdx, %rdx
1403adf86: 74 16                       	je	0x1403adf9e <.text+0x3acf9e>
1403adf88: 48 8b 8d a0 01 00 00        	movq	0x1a0(%rbp), %rcx
1403adf8f: 48 c1 e2 04                 	shlq	$0x4, %rdx
1403adf93: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1403adf99: e8 82 5c 26 00              	callq	0x140613c20 <.text+0x612c20>
1403adf9e: 90                          	nop
1403adf9f: 48 83 c4 78                 	addq	$0x78, %rsp
1403adfa3: 5b                          	popq	%rbx
1403adfa4: 5f                          	popq	%rdi
1403adfa5: 5e                          	popq	%rsi
1403adfa6: 41 5c                       	popq	%r12
1403adfa8: 41 5d                       	popq	%r13
1403adfaa: 41 5e                       	popq	%r14
1403adfac: 41 5f                       	popq	%r15
1403adfae: 5d                          	popq	%rbp
1403adfaf: c3                          	retq
1403adfb0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1403adfb5: 55                          	pushq	%rbp
1403adfb6: 41 57                       	pushq	%r15
1403adfb8: 41 56                       	pushq	%r14
1403adfba: 41 55                       	pushq	%r13
1403adfbc: 41 54                       	pushq	%r12
1403adfbe: 56                          	pushq	%rsi
1403adfbf: 57                          	pushq	%rdi
1403adfc0: 53                          	pushq	%rbx
1403adfc1: 48 83 ec 78                 	subq	$0x78, %rsp
1403adfc5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1403adfcc: 48 8d 8d b8 01 00 00        	leaq	0x1b8(%rbp), %rcx
1403adfd3: e8 58 ec 99 00              	callq	0x140d4cc30 <.text+0xd4bc30>
1403adfd8: 90                          	nop
1403adfd9: 48 83 c4 78                 	addq	$0x78, %rsp
1403adfdd: 5b                          	popq	%rbx
1403adfde: 5f                          	popq	%rdi
1403adfdf: 5e                          	popq	%rsi
1403adfe0: 41 5c                       	popq	%r12
1403adfe2: 41 5d                       	popq	%r13
1403adfe4: 41 5e                       	popq	%r14
1403adfe6: 41 5f                       	popq	%r15
1403adfe8: 5d                          	popq	%rbp
1403adfe9: c3                          	retq
1403adfea: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1403adff0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1403adff5: 55                          	pushq	%rbp
1403adff6: 41 57                       	pushq	%r15
1403adff8: 41 56                       	pushq	%r14
1403adffa: 41 55                       	pushq	%r13
1403adffc: 41 54                       	pushq	%r12
1403adffe: 56                          	pushq	%rsi
1403adfff: 57                          	pushq	%rdi
1403ae000: 53                          	pushq	%rbx
1403ae001: 48 83 ec 78                 	subq	$0x78, %rsp
1403ae005: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1403ae00c: 48 83 bd d8 01 00 00 00     	cmpq	$0x0, 0x1d8(%rbp)
1403ae014: 74 19                       	je	0x1403ae02f <.text+0x3ad02f>
1403ae016: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403ae01c: 48 8b 8d d0 01 00 00        	movq	0x1d0(%rbp), %rcx
1403ae023: 48 8b 95 d8 01 00 00        	movq	0x1d8(%rbp), %rdx
1403ae02a: e8 f1 5b 26 00              	callq	0x140613c20 <.text+0x612c20>
1403ae02f: 90                          	nop
1403ae030: 48 83 c4 78                 	addq	$0x78, %rsp
1403ae034: 5b                          	popq	%rbx
1403ae035: 5f                          	popq	%rdi
1403ae036: 5e                          	popq	%rsi
1403ae037: 41 5c                       	popq	%r12
1403ae039: 41 5d                       	popq	%r13
1403ae03b: 41 5e                       	popq	%r14
1403ae03d: 41 5f                       	popq	%r15
1403ae03f: 5d                          	popq	%rbp
1403ae040: c3                          	retq
