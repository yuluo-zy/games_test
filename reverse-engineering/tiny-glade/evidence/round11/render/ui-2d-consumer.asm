
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1409c4470: 55                          	pushq	%rbp
1409c4471: 41 57                       	pushq	%r15
1409c4473: 41 56                       	pushq	%r14
1409c4475: 41 55                       	pushq	%r13
1409c4477: 41 54                       	pushq	%r12
1409c4479: 56                          	pushq	%rsi
1409c447a: 57                          	pushq	%rdi
1409c447b: 53                          	pushq	%rbx
1409c447c: 48 81 ec 68 0e 00 00        	subq	$0xe68, %rsp            # imm = 0xE68
1409c4483: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1409c448b: 0f 29 bd d0 0d 00 00        	movaps	%xmm7, 0xdd0(%rbp)
1409c4492: 0f 29 b5 c0 0d 00 00        	movaps	%xmm6, 0xdc0(%rbp)
1409c4499: 48 c7 85 b8 0d 00 00 fe ff ff ff    	movq	$-0x2, 0xdb8(%rbp)
1409c44a4: 4c 89 c7                    	movq	%r8, %rdi
1409c44a7: 49 89 d6                    	movq	%rdx, %r14
1409c44aa: 48 89 ce                    	movq	%rcx, %rsi
1409c44ad: e8 4e 25 25 00              	callq	0x140c16a00 <.text+0xc15a00>
1409c44b2: 84 c0                       	testb	%al, %al
1409c44b4: 74 62                       	je	0x1409c4518 <.text+0x9c3518>
1409c44b6: 4d 89 f5                    	movq	%r14, %r13
1409c44b9: 4c 8d 35 83 b5 0f 02        	leaq	0x20fb583(%rip), %r14   # 0x142abfa43
1409c44c0: bb 47 00 00 00              	movl	$0x47, %ebx
1409c44c5: ba 47 00 00 00              	movl	$0x47, %edx
1409c44ca: 4c 89 f1                    	movq	%r14, %rcx
1409c44cd: e8 ae a1 ff ff              	callq	0x1409be680 <.text+0x9bd680>
1409c44d2: 48 83 f8 01                 	cmpq	$0x1, %rax
1409c44d6: 0f 85 b1 00 00 00           	jne	0x1409c458d <.text+0x9c358d>
1409c44dc: 49 89 d1                    	movq	%rdx, %r9
1409c44df: 48 85 d2                    	testq	%rdx, %rdx
1409c44e2: 74 3d                       	je	0x1409c4521 <.text+0x9c3521>
1409c44e4: 49 83 f9 47                 	cmpq	$0x47, %r9
1409c44e8: 73 35                       	jae	0x1409c451f <.text+0x9c351f>
1409c44ea: 48 8d 05 52 b5 0f 02        	leaq	0x20fb552(%rip), %rax   # 0x142abfa43
1409c44f1: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
1409c44f6: 7f 29                       	jg	0x1409c4521 <.text+0x9c3521>
1409c44f8: 48 8d 05 11 b6 0f 02        	leaq	0x20fb611(%rip), %rax   # 0x142abfb10
1409c44ff: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1409c4504: 48 8d 0d 38 b5 0f 02        	leaq	0x20fb538(%rip), %rcx   # 0x142abfa43
1409c450b: ba 47 00 00 00              	movl	$0x47, %edx
1409c4510: 45 31 c0                    	xorl	%r8d, %r8d
1409c4513: e8 28 54 f1 01              	callq	0x1428d9940 <.text+0x28d8940>
1409c4518: 31 c0                       	xorl	%eax, %eax
1409c451a: e9 c7 01 00 00              	jmp	0x1409c46e6 <.text+0x9c36e6>
1409c451f: 75 d7                       	jne	0x1409c44f8 <.text+0x9c34f8>
1409c4521: 4c 8d 35 1b b5 0f 02        	leaq	0x20fb51b(%rip), %r14   # 0x142abfa43
1409c4528: 4c 89 f1                    	movq	%r14, %rcx
1409c452b: 4c 89 ca                    	movq	%r9, %rdx
1409c452e: e8 4d a1 ff ff              	callq	0x1409be680 <.text+0x9bd680>
1409c4533: 48 83 f8 01                 	cmpq	$0x1, %rax
1409c4537: 75 54                       	jne	0x1409c458d <.text+0x9c358d>
1409c4539: 49 89 d0                    	movq	%rdx, %r8
1409c453c: 49 83 c0 02                 	addq	$0x2, %r8
1409c4540: 74 39                       	je	0x1409c457b <.text+0x9c357b>
1409c4542: 49 83 f8 47                 	cmpq	$0x47, %r8
1409c4546: 73 31                       	jae	0x1409c4579 <.text+0x9c3579>
1409c4548: 48 8d 05 f4 b4 0f 02        	leaq	0x20fb4f4(%rip), %rax   # 0x142abfa43
1409c454f: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
1409c4554: 7f 25                       	jg	0x1409c457b <.text+0x9c357b>
1409c4556: 48 8d 05 cb b5 0f 02        	leaq	0x20fb5cb(%rip), %rax   # 0x142abfb28
1409c455d: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1409c4562: 48 8d 0d da b4 0f 02        	leaq	0x20fb4da(%rip), %rcx   # 0x142abfa43
1409c4569: ba 47 00 00 00              	movl	$0x47, %edx
1409c456e: 41 b9 47 00 00 00           	movl	$0x47, %r9d
1409c4574: e8 c7 53 f1 01              	callq	0x1428d9940 <.text+0x28d8940>
1409c4579: 75 db                       	jne	0x1409c4556 <.text+0x9c3556>
1409c457b: bb 45 00 00 00              	movl	$0x45, %ebx
1409c4580: 48 29 d3                    	subq	%rdx, %rbx
1409c4583: 4c 8d 35 b9 b4 0f 02        	leaq	0x20fb4b9(%rip), %r14   # 0x142abfa43
1409c458a: 4d 01 c6                    	addq	%r8, %r14
1409c458d: 4c 8d 05 a0 b9 0f 02        	leaq	0x20fb9a0(%rip), %r8    # 0x142abff34
1409c4594: 4c 8d 3d 54 b9 0f 02        	leaq	0x20fb954(%rip), %r15   # 0x142abfeef
1409c459b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
1409c45a0: 41 0f be 40 ff              	movsbl	-0x1(%r8), %eax
1409c45a5: 85 c0                       	testl	%eax, %eax
1409c45a7: 78 17                       	js	0x1409c45c0 <.text+0x9c35c0>
1409c45a9: 49 ff c8                    	decq	%r8
1409c45ac: 83 f8 5c                    	cmpl	$0x5c, %eax
1409c45af: 75 63                       	jne	0x1409c4614 <.text+0x9c3614>
1409c45b1: eb 73                       	jmp	0x1409c4626 <.text+0x9c3626>
1409c45b3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1409c45c0: 41 0f b6 48 fe              	movzbl	-0x2(%r8), %ecx
1409c45c5: 80 f9 c0                    	cmpb	$-0x40, %cl
1409c45c8: 7d 23                       	jge	0x1409c45ed <.text+0x9c35ed>
1409c45ca: 41 0f b6 50 fd              	movzbl	-0x3(%r8), %edx
1409c45cf: 80 fa c0                    	cmpb	$-0x40, %dl
1409c45d2: 7d 22                       	jge	0x1409c45f6 <.text+0x9c35f6>
1409c45d4: 45 0f b6 48 fc              	movzbl	-0x4(%r8), %r9d
1409c45d9: 49 83 c0 fc                 	addq	$-0x4, %r8
1409c45dd: 41 83 e1 07                 	andl	$0x7, %r9d
1409c45e1: 41 c1 e1 06                 	shll	$0x6, %r9d
1409c45e5: 83 e2 3f                    	andl	$0x3f, %edx
1409c45e8: 44 09 ca                    	orl	%r9d, %edx
1409c45eb: eb 10                       	jmp	0x1409c45fd <.text+0x9c35fd>
1409c45ed: 49 83 c0 fe                 	addq	$-0x2, %r8
1409c45f1: 83 e1 1f                    	andl	$0x1f, %ecx
1409c45f4: eb 0f                       	jmp	0x1409c4605 <.text+0x9c3605>
1409c45f6: 49 83 c0 fd                 	addq	$-0x3, %r8
1409c45fa: 83 e2 0f                    	andl	$0xf, %edx
1409c45fd: c1 e2 06                    	shll	$0x6, %edx
1409c4600: 83 e1 3f                    	andl	$0x3f, %ecx
1409c4603: 09 d1                       	orl	%edx, %ecx
1409c4605: c1 e1 06                    	shll	$0x6, %ecx
1409c4608: 24 3f                       	andb	$0x3f, %al
1409c460a: 0f b6 c0                    	movzbl	%al, %eax
1409c460d: 09 c8                       	orl	%ecx, %eax
1409c460f: 83 f8 5c                    	cmpl	$0x5c, %eax
1409c4612: 74 12                       	je	0x1409c4626 <.text+0x9c3626>
1409c4614: 83 f8 2f                    	cmpl	$0x2f, %eax
1409c4617: 74 0d                       	je	0x1409c4626 <.text+0x9c3626>
1409c4619: 4d 39 f8                    	cmpq	%r15, %r8
1409c461c: 75 82                       	jne	0x1409c45a0 <.text+0x9c35a0>
1409c461e: 41 bc 45 00 00 00           	movl	$0x45, %r12d
1409c4624: eb 4d                       	jmp	0x1409c4673 <.text+0x9c3673>
1409c4626: 4c 8d 3d c2 b8 0f 02        	leaq	0x20fb8c2(%rip), %r15   # 0x142abfeef
1409c462d: 4d 29 f8                    	subq	%r15, %r8
1409c4630: 49 ff c0                    	incq	%r8
1409c4633: 74 32                       	je	0x1409c4667 <.text+0x9c3667>
1409c4635: 49 83 f8 45                 	cmpq	$0x45, %r8
1409c4639: 73 2a                       	jae	0x1409c4665 <.text+0x9c3665>
1409c463b: 43 80 3c 38 bf              	cmpb	$-0x41, (%r8,%r15)
1409c4640: 7f 25                       	jg	0x1409c4667 <.text+0x9c3667>
1409c4642: 48 8d 05 a7 b4 0f 02        	leaq	0x20fb4a7(%rip), %rax   # 0x142abfaf0
1409c4649: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1409c464e: 48 8d 0d 9a b8 0f 02        	leaq	0x20fb89a(%rip), %rcx   # 0x142abfeef
1409c4655: ba 45 00 00 00              	movl	$0x45, %edx
1409c465a: 41 b9 45 00 00 00           	movl	$0x45, %r9d
1409c4660: e8 db 52 f1 01              	callq	0x1428d9940 <.text+0x28d8940>
1409c4665: 75 db                       	jne	0x1409c4642 <.text+0x9c3642>
1409c4667: 41 bc 45 00 00 00           	movl	$0x45, %r12d
1409c466d: 4d 29 c4                    	subq	%r8, %r12
1409c4670: 4d 01 c7                    	addq	%r8, %r15
1409c4673: e8 b8 3a 25 00              	callq	0x140c18130 <.text+0xc17130>
1409c4678: 48 89 c1                    	movq	%rax, %rcx
1409c467b: 48 8b 00                    	movq	(%rax), %rax
1409c467e: 48 83 f8 01                 	cmpq	$0x1, %rax
1409c4682: 0f 85 83 13 00 00           	jne	0x1409c5a0b <.text+0x9c4a0b>
1409c4688: 48 83 c1 08                 	addq	$0x8, %rcx
1409c468c: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1409c4690: 0f 85 99 13 00 00           	jne	0x1409c5a2f <.text+0x9c4a2f>
1409c4696: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1409c469d: 48 89 8d a8 0d 00 00        	movq	%rcx, 0xda8(%rbp)
1409c46a4: 48 83 c1 08                 	addq	$0x8, %rcx
1409c46a8: 4c 89 64 24 20              	movq	%r12, 0x20(%rsp)
1409c46ad: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
1409c46b6: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
1409c46bf: 4c 89 f2                    	movq	%r14, %rdx
1409c46c2: 49 89 d8                    	movq	%rbx, %r8
1409c46c5: 4d 89 f9                    	movq	%r15, %r9
1409c46c8: e8 73 26 25 00              	callq	0x140c16d40 <.text+0xc15d40>
1409c46cd: 48 8b 8d a8 0d 00 00        	movq	0xda8(%rbp), %rcx
1409c46d4: 48 ff 01                    	incq	(%rcx)
1409c46d7: 48 89 85 78 0d 00 00        	movq	%rax, 0xd78(%rbp)
1409c46de: b8 01 00 00 00              	movl	$0x1, %eax
1409c46e3: 4d 89 ee                    	movq	%r13, %r14
1409c46e6: 48 89 85 70 0d 00 00        	movq	%rax, 0xd70(%rbp)
1409c46ed: 49 8d 5e 38                 	leaq	0x38(%r14), %rbx
1409c46f1: 48 ba b3 61 b4 27 07 ee 97 85       	movabsq	$-0x7a6811f8d84b9e4d, %rdx # imm = 0x8597EE0727B461B3
1409c46fb: 49 b8 5c f1 15 ae c5 7e 29 8b       	movabsq	$-0x74d6813a51ea0ea4, %r8 # imm = 0x8B297EC5AE15F15C
1409c4705: 48 89 d9                    	movq	%rbx, %rcx
1409c4708: e8 e3 8f 71 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
1409c470d: 48 85 c0                    	testq	%rax, %rax
1409c4710: 0f 84 ac 13 00 00           	je	0x1409c5ac2 <.text+0x9c4ac2>
1409c4716: 49 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%r14)
1409c471d: 0f 86 9f 13 00 00           	jbe	0x1409c5ac2 <.text+0x9c4ac2>
1409c4723: 49 8b 86 10 02 00 00        	movq	0x210(%r14), %rax
1409c472a: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
1409c472e: 48 85 c9                    	testq	%rcx, %rcx
1409c4731: 0f 84 8b 13 00 00           	je	0x1409c5ac2 <.text+0x9c4ac2>
1409c4737: 49 8b 86 e0 01 00 00        	movq	0x1e0(%r14), %rax
1409c473e: 48 f7 d1                    	notq	%rcx
1409c4741: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1409c4745: 48 c1 e1 05                 	shlq	$0x5, %rcx
1409c4749: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
1409c474f: 0f 84 6d 13 00 00           	je	0x1409c5ac2 <.text+0x9c4ac2>
1409c4755: 48 01 c8                    	addq	%rcx, %rax
1409c4758: 48 8b 48 10                 	movq	0x10(%rax), %rcx
1409c475c: 48 8d 15 ed b7 0f 02        	leaq	0x20fb7ed(%rip), %rdx   # 0x142abff50
1409c4763: 41 b8 05 00 00 00           	movl	$0x5, %r8d
1409c4769: e8 32 33 f7 ff              	callq	0x140937aa0 <.text+0x936aa0>
1409c476e: 48 89 95 90 0d 00 00        	movq	%rdx, 0xd90(%rbp)
1409c4775: a8 01                       	testb	$0x1, %al
1409c4777: 0f 84 aa 13 00 00           	je	0x1409c5b27 <.text+0x9c4b27>
1409c477d: 48 ba b3 61 b4 27 07 ee 97 85       	movabsq	$-0x7a6811f8d84b9e4d, %rdx # imm = 0x8597EE0727B461B3
1409c4787: 49 b8 5c f1 15 ae c5 7e 29 8b       	movabsq	$-0x74d6813a51ea0ea4, %r8 # imm = 0x8B297EC5AE15F15C
1409c4791: 48 89 d9                    	movq	%rbx, %rcx
1409c4794: e8 57 8f 71 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
1409c4799: 48 85 c0                    	testq	%rax, %rax
1409c479c: 0f 84 31 13 00 00           	je	0x1409c5ad3 <.text+0x9c4ad3>
1409c47a2: 49 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%r14)
1409c47a9: 0f 86 24 13 00 00           	jbe	0x1409c5ad3 <.text+0x9c4ad3>
1409c47af: 49 8b 86 10 02 00 00        	movq	0x210(%r14), %rax
1409c47b6: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
1409c47ba: 48 85 c9                    	testq	%rcx, %rcx
1409c47bd: 0f 84 10 13 00 00           	je	0x1409c5ad3 <.text+0x9c4ad3>
1409c47c3: 49 8b 86 e0 01 00 00        	movq	0x1e0(%r14), %rax
1409c47ca: 48 f7 d1                    	notq	%rcx
1409c47cd: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1409c47d1: 48 c1 e1 05                 	shlq	$0x5, %rcx
1409c47d5: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
1409c47db: 0f 84 f2 12 00 00           	je	0x1409c5ad3 <.text+0x9c4ad3>
1409c47e1: 48 01 c8                    	addq	%rcx, %rax
1409c47e4: 48 8b 48 10                 	movq	0x10(%rax), %rcx
1409c47e8: 48 8d 15 99 b7 0f 02        	leaq	0x20fb799(%rip), %rdx   # 0x142abff88
1409c47ef: 41 b8 12 00 00 00           	movl	$0x12, %r8d
1409c47f5: e8 a6 32 f7 ff              	callq	0x140937aa0 <.text+0x936aa0>
1409c47fa: 48 89 95 88 0d 00 00        	movq	%rdx, 0xd88(%rbp)
1409c4801: a8 01                       	testb	$0x1, %al
1409c4803: 0f 84 2c 13 00 00           	je	0x1409c5b35 <.text+0x9c4b35>
1409c4809: 48 ba a6 12 23 8c a9 dc 4d 86       	movabsq	$-0x79b2235673dced5a, %rdx # imm = 0x864DDCA98C2312A6
1409c4813: 49 b8 4e 48 9a 9e 36 e9 b0 27       	movabsq	$0x27b0e9369e9a484e, %r8 # imm = 0x27B0E9369E9A484E
1409c481d: 48 89 d9                    	movq	%rbx, %rcx
1409c4820: e8 cb 8e 71 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
1409c4825: 48 85 c0                    	testq	%rax, %rax
1409c4828: 0f 84 b3 12 00 00           	je	0x1409c5ae1 <.text+0x9c4ae1>
1409c482e: 49 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%r14)
1409c4835: 0f 86 a6 12 00 00           	jbe	0x1409c5ae1 <.text+0x9c4ae1>
1409c483b: 49 8b 86 10 02 00 00        	movq	0x210(%r14), %rax
1409c4842: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
1409c4846: 48 85 c9                    	testq	%rcx, %rcx
1409c4849: 0f 84 92 12 00 00           	je	0x1409c5ae1 <.text+0x9c4ae1>
1409c484f: 49 8b 86 e0 01 00 00        	movq	0x1e0(%r14), %rax
1409c4856: 48 f7 d1                    	notq	%rcx
1409c4859: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1409c485d: 48 c1 e1 05                 	shlq	$0x5, %rcx
1409c4861: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
1409c4867: 0f 84 74 12 00 00           	je	0x1409c5ae1 <.text+0x9c4ae1>
1409c486d: 48 01 c8                    	addq	%rcx, %rax
1409c4870: 48 8b 40 10                 	movq	0x10(%rax), %rax
1409c4874: 44 8b 28                    	movl	(%rax), %r13d
1409c4877: 44 8b 60 04                 	movl	0x4(%rax), %r12d
1409c487b: 48 8d 8d b0 03 00 00        	leaq	0x3b0(%rbp), %rcx
1409c4882: 4c 89 f2                    	movq	%r14, %rdx
1409c4885: e8 86 48 c9 00              	callq	0x141659110 <.text+0x1658110>
1409c488a: 48 89 9d 58 0d 00 00        	movq	%rbx, 0xd58(%rbp)
1409c4891: 48 8d 8d b0 03 00 00        	leaq	0x3b0(%rbp), %rcx
1409c4898: 4c 89 f2                    	movq	%r14, %rdx
1409c489b: e8 f0 35 c9 00              	callq	0x141657e90 <.text+0x1656e90>
1409c48a0: 49 8b 96 a0 05 00 00        	movq	0x5a0(%r14), %rdx
1409c48a7: 48 8b 8d b0 03 00 00        	movq	0x3b0(%rbp), %rcx
1409c48ae: 48 39 d1                    	cmpq	%rdx, %rcx
1409c48b1: 0f 85 84 11 00 00           	jne	0x1409c5a3b <.text+0x9c4a3b>
1409c48b7: 45 8b 8e b4 05 00 00        	movl	0x5b4(%r14), %r9d
1409c48be: 41 8b 86 b0 05 00 00        	movl	0x5b0(%r14), %eax
1409c48c5: 48 8d 8d a0 0b 00 00        	leaq	0xba0(%rbp), %rcx
1409c48cc: 89 44 24 20                 	movl	%eax, 0x20(%rsp)
1409c48d0: 48 8d 9d b0 03 00 00        	leaq	0x3b0(%rbp), %rbx
1409c48d7: 4c 89 f2                    	movq	%r14, %rdx
1409c48da: 49 89 d8                    	movq	%rbx, %r8
1409c48dd: e8 ae 13 dd 00              	callq	0x141795c90 <.text+0x1794c90>
1409c48e2: 49 8d 86 f8 00 00 00        	leaq	0xf8(%r14), %rax
1409c48e9: 49 8d 8e a0 01 00 00        	leaq	0x1a0(%r14), %rcx
1409c48f0: 4c 89 b5 80 0b 00 00        	movq	%r14, 0xb80(%rbp)
1409c48f7: 48 89 8d 68 0d 00 00        	movq	%rcx, 0xd68(%rbp)
1409c48fe: 48 89 8d 88 0b 00 00        	movq	%rcx, 0xb88(%rbp)
1409c4905: 48 89 85 60 0d 00 00        	movq	%rax, 0xd60(%rbp)
1409c490c: 48 89 85 90 0b 00 00        	movq	%rax, 0xb90(%rbp)
1409c4913: 48 89 9d 98 0b 00 00        	movq	%rbx, 0xb98(%rbp)
1409c491a: 4c 8d 7d c8                 	leaq	-0x38(%rbp), %r15
1409c491e: 48 8d 9d 80 0b 00 00        	leaq	0xb80(%rbp), %rbx
1409c4925: 41 b8 b8 00 00 00           	movl	$0xb8, %r8d
1409c492b: 4c 89 f9                    	movq	%r15, %rcx
1409c492e: 48 89 da                    	movq	%rbx, %rdx
1409c4931: e8 ca f2 f5 01              	callq	0x142923c00 <.text+0x2922c00>
1409c4936: 4c 8d 05 5b b0 0f 02        	leaq	0x20fb05b(%rip), %r8    # 0x142abf998
1409c493d: 48 8d 8d 40 0d 00 00        	leaq	0xd40(%rbp), %rcx
1409c4944: 4c 89 fa                    	movq	%r15, %rdx
1409c4947: e8 44 20 94 00              	callq	0x141306990 <.text+0x1305990>
1409c494c: 4c 89 b5 98 0d 00 00        	movq	%r14, 0xd98(%rbp)
1409c4953: c6 85 b5 0d 00 00 01        	movb	$0x1, 0xdb5(%rbp)
1409c495a: 48 8d 8d b0 03 00 00        	leaq	0x3b0(%rbp), %rcx
1409c4961: e8 7a c2 c0 ff              	callq	0x1405d0be0 <.text+0x5cfbe0>
1409c4966: 48 8b 85 48 0d 00 00        	movq	0xd48(%rbp), %rax
1409c496d: 48 89 85 a0 0d 00 00        	movq	%rax, 0xda0(%rbp)
1409c4974: 4c 8b b5 50 0d 00 00        	movq	0xd50(%rbp), %r14
1409c497b: 4c 89 bd b0 03 00 00        	movq	%r15, 0x3b0(%rbp)
1409c4982: 49 83 fe 02                 	cmpq	$0x2, %r14
1409c4986: 0f 83 c0 10 00 00           	jae	0x1409c5a4c <.text+0x9c4a4c>
1409c498c: 48 8d 4f 04                 	leaq	0x4(%rdi), %rcx
1409c4990: c6 85 b5 0d 00 00 01        	movb	$0x1, 0xdb5(%rbp)
1409c4997: e8 f4 0d 21 00              	callq	0x140bd5790 <.text+0xbd4790>
1409c499c: c7 85 b0 03 00 00 00 00 00 00       	movl	$0x0, 0x3b0(%rbp)
1409c49a6: 48 89 bd b8 03 00 00        	movq	%rdi, 0x3b8(%rbp)
1409c49ad: c7 85 80 0b 00 00 02 00 00 00       	movl	$0x2, 0xb80(%rbp)
1409c49b7: c6 85 b5 0d 00 00 01        	movb	$0x1, 0xdb5(%rbp)
1409c49be: 48 89 74 24 40              	movq	%rsi, 0x40(%rsp)
1409c49c3: 48 8b 8d 98 0d 00 00        	movq	0xd98(%rbp), %rcx
1409c49ca: 48 89 4c 24 38              	movq	%rcx, 0x38(%rsp)
1409c49cf: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
1409c49d4: 48 89 5c 24 28              	movq	%rbx, 0x28(%rsp)
1409c49d9: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1409c49e2: 48 8d 15 e7 b5 0f 02        	leaq	0x20fb5e7(%rip), %rdx   # 0x142abffd0
1409c49e9: 48 8d 4d c8                 	leaq	-0x38(%rbp), %rcx
1409c49ed: 4c 8d bd b0 03 00 00        	leaq	0x3b0(%rbp), %r15
1409c49f4: 41 b8 05 00 00 00           	movl	$0x5, %r8d
1409c49fa: 4d 89 f9                    	movq	%r15, %r9
1409c49fd: e8 6e 2f ea ff              	callq	0x140867970 <.text+0x866970>
1409c4a02: f3 49 0f 2a f5              	cvtsi2ss	%r13, %xmm6
1409c4a07: f3 49 0f 2a fc              	cvtsi2ss	%r12, %xmm7
1409c4a0c: 48 8b 85 40 0d 00 00        	movq	0xd40(%rbp), %rax
1409c4a13: 48 89 85 80 0d 00 00        	movq	%rax, 0xd80(%rbp)
1409c4a1a: 4b 8d 04 76                 	leaq	(%r14,%r14,2), %rax
1409c4a1e: 48 c1 e0 04                 	shlq	$0x4, %rax
1409c4a22: 48 8b 9d a0 0d 00 00        	movq	0xda0(%rbp), %rbx
1409c4a29: 48 01 d8                    	addq	%rbx, %rax
1409c4a2c: 48 89 85 a8 0d 00 00        	movq	%rax, 0xda8(%rbp)
1409c4a33: 48 3b 9d a8 0d 00 00        	cmpq	0xda8(%rbp), %rbx
1409c4a3a: 0f 84 31 08 00 00           	je	0x1409c5271 <.text+0x9c4271>
1409c4a40: 4c 8b 33                    	movq	(%rbx), %r14
1409c4a43: 4d 85 f6                    	testq	%r14, %r14
1409c4a46: 0f 84 25 08 00 00           	je	0x1409c5271 <.text+0x9c4271>
1409c4a4c: 48 8b 7b 08                 	movq	0x8(%rbx), %rdi
1409c4a50: 48 8b 73 10                 	movq	0x10(%rbx), %rsi
1409c4a54: 48 85 f6                    	testq	%rsi, %rsi
1409c4a57: 0f 84 13 04 00 00           	je	0x1409c4e70 <.text+0x9c3e70>
1409c4a5d: e8 ee 1c 22 00              	callq	0x140be6750 <.text+0xbe5750>
1409c4a62: 41 89 c4                    	movl	%eax, %r12d
1409c4a65: 31 c9                       	xorl	%ecx, %ecx
1409c4a67: e8 b4 50 71 ff              	callq	0x1400d9b20 <.text+0xd8b20>
1409c4a6c: 48 85 c0                    	testq	%rax, %rax
1409c4a6f: 0f 84 2b 10 00 00           	je	0x1409c5aa0 <.text+0x9c4aa0>
1409c4a75: 44 89 a5 20 0d 00 00        	movl	%r12d, 0xd20(%rbp)
1409c4a7c: 66 c7 85 28 0d 00 00 00 00  	movw	$0x0, 0xd28(%rbp)
1409c4a85: c7 85 24 0d 00 00 00 00 80 3f       	movl	$0x3f800000, 0xd24(%rbp) # imm = 0x3F800000
1409c4a8f: 48 c7 85 78 0c 00 00 00 00 00 00    	movq	$0x0, 0xc78(%rbp)
1409c4a9a: 48 8b 8d 88 0d 00 00        	movq	0xd88(%rbp), %rcx
1409c4aa1: 48 89 8d 18 0d 00 00        	movq	%rcx, 0xd18(%rbp)
1409c4aa8: 48 c7 85 88 0c 00 00 00 00 00 00    	movq	$0x0, 0xc88(%rbp)
1409c4ab3: 48 c7 85 90 0c 00 00 04 00 00 00    	movq	$0x4, 0xc90(%rbp)
1409c4abe: 48 c7 85 98 0c 00 00 00 00 00 00    	movq	$0x0, 0xc98(%rbp)
1409c4ac9: 0f 10 00                    	movups	(%rax), %xmm0
1409c4acc: 48 ff 00                    	incq	(%rax)
1409c4acf: 0f 10 0d 5a 1c 0d 02        	movups	0x20d1c5a(%rip), %xmm1  # 0x142a96730
1409c4ad6: 48 8d 85 e8 0c 00 00        	leaq	0xce8(%rbp), %rax
1409c4add: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
1409c4ae1: 0f 10 0d 38 1c 0d 02        	movups	0x20d1c38(%rip), %xmm1  # 0x142a96720
1409c4ae8: 0f 11 08                    	movups	%xmm1, (%rax)
1409c4aeb: 0f 11 85 08 0d 00 00        	movups	%xmm0, 0xd08(%rbp)
1409c4af2: 48 b8 03 00 00 00 01 00 00 00       	movabsq	$0x100000003, %rax # imm = 0x100000003
1409c4afc: 48 89 85 b8 0c 00 00        	movq	%rax, 0xcb8(%rbp)
1409c4b03: 48 c7 85 a0 0c 00 00 00 00 00 00    	movq	$0x0, 0xca0(%rbp)
1409c4b0e: 48 c7 85 a8 0c 00 00 01 00 00 00    	movq	$0x1, 0xca8(%rbp)
1409c4b19: 48 c7 85 b0 0c 00 00 00 00 00 00    	movq	$0x0, 0xcb0(%rbp)
1409c4b24: 4c 89 f9                    	movq	%r15, %rcx
1409c4b27: 48 8d 95 78 0c 00 00        	leaq	0xc78(%rbp), %rdx
1409c4b2e: 4c 8d 45 c8                 	leaq	-0x38(%rbp), %r8
1409c4b32: 45 31 c9                    	xorl	%r9d, %r9d
1409c4b35: e8 a6 10 f4 ff              	callq	0x140905be0 <.text+0x904be0>
1409c4b3a: 80 bd 8e 04 00 00 08        	cmpb	$0x8, 0x48e(%rbp)
1409c4b41: 74 36                       	je	0x1409c4b79 <.text+0x9c3b79>
1409c4b43: f3 41 0f 10 1e              	movss	(%r14), %xmm3
1409c4b48: f3 41 0f 10 46 04           	movss	0x4(%r14), %xmm0
1409c4b4e: 48 8b 95 98 04 00 00        	movq	0x498(%rbp), %rdx
1409c4b55: b8 e8 00 00 00              	movl	$0xe8, %eax
1409c4b5a: 48 01 c2                    	addq	%rax, %rdx
1409c4b5d: f3 0f 11 44 24 20           	movss	%xmm0, 0x20(%rsp)
1409c4b63: 48 8d 8d 68 04 00 00        	leaq	0x468(%rbp), %rcx
1409c4b6a: 49 b8 0f b5 93 3e bd 1f 0d 12       	movabsq	$0x120d1fbd3e93b50f, %r8 # imm = 0x120D1FBD3E93B50F
1409c4b74: e8 c7 6d fa ff              	callq	0x14096b940 <.text+0x96a940>
1409c4b79: 41 b8 f8 00 00 00           	movl	$0xf8, %r8d
1409c4b7f: 48 8d 8d 80 0b 00 00        	leaq	0xb80(%rbp), %rcx
1409c4b86: 4c 89 fa                    	movq	%r15, %rdx
1409c4b89: e8 72 f0 f5 01              	callq	0x142923c00 <.text+0x2922c00>
1409c4b8e: 80 bd 5e 0c 00 00 08        	cmpb	$0x8, 0xc5e(%rbp)
1409c4b95: 74 37                       	je	0x1409c4bce <.text+0x9c3bce>
1409c4b97: f3 41 0f 10 5e 08           	movss	0x8(%r14), %xmm3
1409c4b9d: f3 41 0f 10 46 0c           	movss	0xc(%r14), %xmm0
1409c4ba3: 48 8b 95 68 0c 00 00        	movq	0xc68(%rbp), %rdx
1409c4baa: b8 e8 00 00 00              	movl	$0xe8, %eax
1409c4baf: 48 01 c2                    	addq	%rax, %rdx
1409c4bb2: f3 0f 11 44 24 20           	movss	%xmm0, 0x20(%rsp)
1409c4bb8: 48 8d 8d 38 0c 00 00        	leaq	0xc38(%rbp), %rcx
1409c4bbf: 49 b8 c5 b9 41 2b 38 8d fc a6       	movabsq	$-0x590372c7d4be463b, %r8 # imm = 0xA6FC8D382B41B9C5
1409c4bc9: e8 72 6d fa ff              	callq	0x14096b940 <.text+0x96a940>
1409c4bce: 41 b8 f8 00 00 00           	movl	$0xf8, %r8d
1409c4bd4: 48 8d 8d 80 0a 00 00        	leaq	0xa80(%rbp), %rcx
1409c4bdb: 4c 8d b5 80 0b 00 00        	leaq	0xb80(%rbp), %r14
1409c4be2: 4c 89 f2                    	movq	%r14, %rdx
1409c4be5: e8 16 f0 f5 01              	callq	0x142923c00 <.text+0x2922c00>
1409c4bea: 80 bd 5e 0b 00 00 08        	cmpb	$0x8, 0xb5e(%rbp)
1409c4bf1: 74 2e                       	je	0x1409c4c21 <.text+0x9c3c21>
1409c4bf3: 48 8b 95 68 0b 00 00        	movq	0xb68(%rbp), %rdx
1409c4bfa: b8 e8 00 00 00              	movl	$0xe8, %eax
1409c4bff: 48 01 c2                    	addq	%rax, %rdx
1409c4c02: f3 0f 11 7c 24 20           	movss	%xmm7, 0x20(%rsp)
1409c4c08: 48 8d 8d 38 0b 00 00        	leaq	0xb38(%rbp), %rcx
1409c4c0f: 49 b8 87 47 1c 99 9b ab fd 69       	movabsq	$0x69fdab9b991c4787, %r8 # imm = 0x69FDAB9B991C4787
1409c4c19: 0f 28 de                    	movaps	%xmm6, %xmm3
1409c4c1c: e8 1f 6d fa ff              	callq	0x14096b940 <.text+0x96a940>
1409c4c21: 41 b8 f8 00 00 00           	movl	$0xf8, %r8d
1409c4c27: 48 8d 8d 90 08 00 00        	leaq	0x890(%rbp), %rcx
1409c4c2e: 48 8d 95 80 0a 00 00        	leaq	0xa80(%rbp), %rdx
1409c4c35: e8 c6 ef f5 01              	callq	0x142923c00 <.text+0x2922c00>
1409c4c3a: 80 bd 6e 09 00 00 08        	cmpb	$0x8, 0x96e(%rbp)
1409c4c41: 74 2a                       	je	0x1409c4c6d <.text+0x9c3c6d>
1409c4c43: f3 0f 10 5e 18              	movss	0x18(%rsi), %xmm3
1409c4c48: 48 8b 95 78 09 00 00        	movq	0x978(%rbp), %rdx
1409c4c4f: b8 e8 00 00 00              	movl	$0xe8, %eax
1409c4c54: 48 01 c2                    	addq	%rax, %rdx
1409c4c57: 48 8d 8d 48 09 00 00        	leaq	0x948(%rbp), %rcx
1409c4c5e: 49 b8 d6 22 e8 ce da a5 b2 ea       	movabsq	$-0x154d5a253117dd2a, %r8 # imm = 0xEAB2A5DACEE822D6
1409c4c68: e8 c3 6d fa ff              	callq	0x14096ba30 <.text+0x96aa30>
1409c4c6d: 41 b8 f8 00 00 00           	movl	$0xf8, %r8d
1409c4c73: 48 8d 8d 98 07 00 00        	leaq	0x798(%rbp), %rcx
1409c4c7a: 48 8d 95 90 08 00 00        	leaq	0x890(%rbp), %rdx
1409c4c81: e8 7a ef f5 01              	callq	0x142923c00 <.text+0x2922c00>
1409c4c86: 80 bd 76 08 00 00 08        	cmpb	$0x8, 0x876(%rbp)
1409c4c8d: 74 2a                       	je	0x1409c4cb9 <.text+0x9c3cb9>
1409c4c8f: 44 0f b6 4e 21              	movzbl	0x21(%rsi), %r9d
1409c4c94: 48 8b 95 80 08 00 00        	movq	0x880(%rbp), %rdx
1409c4c9b: b8 e8 00 00 00              	movl	$0xe8, %eax
1409c4ca0: 48 01 c2                    	addq	%rax, %rdx
1409c4ca3: 48 8d 8d 50 08 00 00        	leaq	0x850(%rbp), %rcx
1409c4caa: 49 b8 f1 2e d8 9b 4f a6 8e 0d       	movabsq	$0xd8ea64f9bd82ef1, %r8 # imm = 0xD8EA64F9BD82EF1
1409c4cb4: e8 97 6b fa ff              	callq	0x14096b850 <.text+0x96a850>
1409c4cb9: 41 b8 f8 00 00 00           	movl	$0xf8, %r8d
1409c4cbf: 48 8d 8d 88 09 00 00        	leaq	0x988(%rbp), %rcx
1409c4cc6: 48 8d 95 98 07 00 00        	leaq	0x798(%rbp), %rdx
1409c4ccd: e8 2e ef f5 01              	callq	0x142923c00 <.text+0x2922c00>
1409c4cd2: 48 8b 4f 08                 	movq	0x8(%rdi), %rcx
1409c4cd6: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
1409c4cda: c6 85 b3 0d 00 00 01        	movb	$0x1, 0xdb3(%rbp)
1409c4ce1: e8 9a 80 2d 00              	callq	0x140c9cd80 <.text+0xc9bd80>
1409c4ce6: 49 89 c4                    	movq	%rax, %r12
1409c4ce9: c7 85 b0 03 00 00 00 00 00 00       	movl	$0x0, 0x3b0(%rbp)
1409c4cf3: c7 85 b8 03 00 00 00 00 00 00       	movl	$0x0, 0x3b8(%rbp)
1409c4cfd: c7 85 c0 03 00 00 00 00 00 00       	movl	$0x0, 0x3c0(%rbp)
1409c4d07: c7 85 c8 03 00 00 00 00 00 00       	movl	$0x0, 0x3c8(%rbp)
1409c4d11: c7 85 d0 03 00 00 00 00 00 00       	movl	$0x0, 0x3d0(%rbp)
1409c4d1b: c7 85 d8 03 00 00 00 00 00 00       	movl	$0x0, 0x3d8(%rbp)
1409c4d25: 48 8d 85 e0 03 00 00        	leaq	0x3e0(%rbp), %rax
1409c4d2c: c6 40 08 00                 	movb	$0x0, 0x8(%rax)
1409c4d30: 48 c7 00 00 00 00 00        	movq	$0x0, (%rax)
1409c4d37: c6 85 b3 0d 00 00 01        	movb	$0x1, 0xdb3(%rbp)
1409c4d3e: 4c 89 f1                    	movq	%r14, %rcx
1409c4d41: 4c 89 fa                    	movq	%r15, %rdx
1409c4d44: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1409c4d4a: e8 91 0a 21 00              	callq	0x140bd57e0 <.text+0xbd47e0>
1409c4d4f: 44 8b 47 18                 	movl	0x18(%rdi), %r8d
1409c4d53: c6 85 b3 0d 00 00 01        	movb	$0x1, 0xdb3(%rbp)
1409c4d5a: 4c 89 f9                    	movq	%r15, %rcx
1409c4d5d: 4c 89 f2                    	movq	%r14, %rdx
1409c4d60: e8 fb 0b 21 00              	callq	0x140bd5960 <.text+0xbd4960>
1409c4d65: c6 85 b3 0d 00 00 01        	movb	$0x1, 0xdb3(%rbp)
1409c4d6c: 48 8d 8d 90 08 00 00        	leaq	0x890(%rbp), %rcx
1409c4d73: 4c 89 fa                    	movq	%r15, %rdx
1409c4d76: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1409c4d7c: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1409c4d82: e8 19 0c 21 00              	callq	0x140bd59a0 <.text+0xbd49a0>
1409c4d87: 4c 89 a5 98 07 00 00        	movq	%r12, 0x798(%rbp)
1409c4d8e: 0f 10 85 90 08 00 00        	movups	0x890(%rbp), %xmm0
1409c4d95: 0f 10 8d a0 08 00 00        	movups	0x8a0(%rbp), %xmm1
1409c4d9c: 0f 10 95 b0 08 00 00        	movups	0x8b0(%rbp), %xmm2
1409c4da3: 0f 10 9d bc 08 00 00        	movups	0x8bc(%rbp), %xmm3
1409c4daa: 0f 11 9d dc 03 00 00        	movups	%xmm3, 0x3dc(%rbp)
1409c4db1: 0f 29 95 d0 03 00 00        	movaps	%xmm2, 0x3d0(%rbp)
1409c4db8: 0f 29 8d c0 03 00 00        	movaps	%xmm1, 0x3c0(%rbp)
1409c4dbf: 0f 29 85 b0 03 00 00        	movaps	%xmm0, 0x3b0(%rbp)
1409c4dc6: 4c 89 7c 24 20              	movq	%r15, 0x20(%rsp)
1409c4dcb: 48 8d 8d 98 07 00 00        	leaq	0x798(%rbp), %rcx
1409c4dd2: 48 8d 95 88 09 00 00        	leaq	0x988(%rbp), %rdx
1409c4dd9: 49 b8 87 fe 4b e7 c2 86 06 74       	movabsq	$0x740686c2e74bfe87, %r8 # imm = 0x740686C2E74BFE87
1409c4de3: 41 b1 1d                    	movb	$0x1d, %r9b
1409c4de6: e8 25 38 f4 ff              	callq	0x140908610 <.text+0x907610>
1409c4deb: 48 8b b5 88 09 00 00        	movq	0x988(%rbp), %rsi
1409c4df2: 41 b8 a0 00 00 00           	movl	$0xa0, %r8d
1409c4df8: 4c 89 f1                    	movq	%r14, %rcx
1409c4dfb: 4c 8d ad 90 09 00 00        	leaq	0x990(%rbp), %r13
1409c4e02: 4c 89 ea                    	movq	%r13, %rdx
1409c4e05: e8 f6 ed f5 01              	callq	0x142923c00 <.text+0x2922c00>
1409c4e0a: 0f b6 bd 30 0a 00 00        	movzbl	0xa30(%rbp), %edi
1409c4e11: 4c 89 f2                    	movq	%r14, %rdx
1409c4e14: 44 0f b7 b5 31 0a 00 00     	movzwl	0xa31(%rbp), %r14d
1409c4e1c: 44 0f b6 a5 33 0a 00 00     	movzbl	0xa33(%rbp), %r12d
1409c4e24: 41 0f 10 85 a4 00 00 00     	movups	0xa4(%r13), %xmm0
1409c4e2c: 41 0f 10 8d b4 00 00 00     	movups	0xb4(%r13), %xmm1
1409c4e34: 41 0f 10 95 c4 00 00 00     	movups	0xc4(%r13), %xmm2
1409c4e3c: 41 0f 10 9d d4 00 00 00     	movups	0xd4(%r13), %xmm3
1409c4e44: 0f 29 85 80 0a 00 00        	movaps	%xmm0, 0xa80(%rbp)
1409c4e4b: 0f 29 8d 90 0a 00 00        	movaps	%xmm1, 0xa90(%rbp)
1409c4e52: 0f 29 95 a0 0a 00 00        	movaps	%xmm2, 0xaa0(%rbp)
1409c4e59: 0f 29 9d b0 0a 00 00        	movaps	%xmm3, 0xab0(%rbp)
1409c4e60: 41 0f 10 85 e0 00 00 00     	movups	0xe0(%r13), %xmm0
1409c4e68: e9 5a 03 00 00              	jmp	0x1409c51c7 <.text+0x9c41c7>
1409c4e6d: 0f 1f 00                    	nopl	(%rax)
1409c4e70: 48 8b 73 18                 	movq	0x18(%rbx), %rsi
1409c4e74: 4c 8b 6b 28                 	movq	0x28(%rbx), %r13
1409c4e78: e8 d3 18 22 00              	callq	0x140be6750 <.text+0xbe5750>
1409c4e7d: 41 89 c4                    	movl	%eax, %r12d
1409c4e80: 31 c9                       	xorl	%ecx, %ecx
1409c4e82: e8 99 4c 71 ff              	callq	0x1400d9b20 <.text+0xd8b20>
1409c4e87: 48 85 c0                    	testq	%rax, %rax
1409c4e8a: 0f 84 21 0c 00 00           	je	0x1409c5ab1 <.text+0x9c4ab1>
1409c4e90: 44 89 a5 20 0d 00 00        	movl	%r12d, 0xd20(%rbp)
1409c4e97: 66 c7 85 28 0d 00 00 00 00  	movw	$0x0, 0xd28(%rbp)
1409c4ea0: c7 85 24 0d 00 00 00 00 80 3f       	movl	$0x3f800000, 0xd24(%rbp) # imm = 0x3F800000
1409c4eaa: 48 c7 85 78 0c 00 00 00 00 00 00    	movq	$0x0, 0xc78(%rbp)
1409c4eb5: 48 8b 8d 90 0d 00 00        	movq	0xd90(%rbp), %rcx
1409c4ebc: 48 89 8d 18 0d 00 00        	movq	%rcx, 0xd18(%rbp)
1409c4ec3: 48 c7 85 88 0c 00 00 00 00 00 00    	movq	$0x0, 0xc88(%rbp)
1409c4ece: 48 c7 85 90 0c 00 00 04 00 00 00    	movq	$0x4, 0xc90(%rbp)
1409c4ed9: 48 c7 85 98 0c 00 00 00 00 00 00    	movq	$0x0, 0xc98(%rbp)
1409c4ee4: 0f 10 00                    	movups	(%rax), %xmm0
1409c4ee7: 48 ff 00                    	incq	(%rax)
1409c4eea: 0f 10 0d 3f 18 0d 02        	movups	0x20d183f(%rip), %xmm1  # 0x142a96730
1409c4ef1: 48 8d 85 e8 0c 00 00        	leaq	0xce8(%rbp), %rax
1409c4ef8: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
1409c4efc: 0f 10 0d 1d 18 0d 02        	movups	0x20d181d(%rip), %xmm1  # 0x142a96720
1409c4f03: 0f 11 08                    	movups	%xmm1, (%rax)
1409c4f06: 0f 11 85 08 0d 00 00        	movups	%xmm0, 0xd08(%rbp)
1409c4f0d: 48 b8 03 00 00 00 01 00 00 00       	movabsq	$0x100000003, %rax # imm = 0x100000003
1409c4f17: 48 89 85 b8 0c 00 00        	movq	%rax, 0xcb8(%rbp)
1409c4f1e: 48 c7 85 a0 0c 00 00 00 00 00 00    	movq	$0x0, 0xca0(%rbp)
1409c4f29: 48 c7 85 a8 0c 00 00 01 00 00 00    	movq	$0x1, 0xca8(%rbp)
1409c4f34: 48 c7 85 b0 0c 00 00 00 00 00 00    	movq	$0x0, 0xcb0(%rbp)
1409c4f3f: 4c 89 f9                    	movq	%r15, %rcx
1409c4f42: 48 8d 95 78 0c 00 00        	leaq	0xc78(%rbp), %rdx
1409c4f49: 4c 8d 45 c8                 	leaq	-0x38(%rbp), %r8
1409c4f4d: 45 31 c9                    	xorl	%r9d, %r9d
1409c4f50: e8 8b 0c f4 ff              	callq	0x140905be0 <.text+0x904be0>
1409c4f55: 80 bd 8e 04 00 00 08        	cmpb	$0x8, 0x48e(%rbp)
1409c4f5c: 74 36                       	je	0x1409c4f94 <.text+0x9c3f94>
1409c4f5e: f3 41 0f 10 1e              	movss	(%r14), %xmm3
1409c4f63: f3 41 0f 10 46 04           	movss	0x4(%r14), %xmm0
1409c4f69: 48 8b 95 98 04 00 00        	movq	0x498(%rbp), %rdx
1409c4f70: b8 e8 00 00 00              	movl	$0xe8, %eax
1409c4f75: 48 01 c2                    	addq	%rax, %rdx
1409c4f78: f3 0f 11 44 24 20           	movss	%xmm0, 0x20(%rsp)
1409c4f7e: 48 8d 8d 68 04 00 00        	leaq	0x468(%rbp), %rcx
1409c4f85: 49 b8 0f b5 93 3e bd 1f 0d 12       	movabsq	$0x120d1fbd3e93b50f, %r8 # imm = 0x120D1FBD3E93B50F
1409c4f8f: e8 ac 69 fa ff              	callq	0x14096b940 <.text+0x96a940>
1409c4f94: 41 b8 f8 00 00 00           	movl	$0xf8, %r8d
1409c4f9a: 48 8d 8d 80 0b 00 00        	leaq	0xb80(%rbp), %rcx
1409c4fa1: 4c 89 fa                    	movq	%r15, %rdx
1409c4fa4: e8 57 ec f5 01              	callq	0x142923c00 <.text+0x2922c00>
1409c4fa9: 80 bd 5e 0c 00 00 08        	cmpb	$0x8, 0xc5e(%rbp)
1409c4fb0: 74 37                       	je	0x1409c4fe9 <.text+0x9c3fe9>
1409c4fb2: f3 41 0f 10 5e 08           	movss	0x8(%r14), %xmm3
1409c4fb8: f3 41 0f 10 46 0c           	movss	0xc(%r14), %xmm0
1409c4fbe: 48 8b 95 68 0c 00 00        	movq	0xc68(%rbp), %rdx
1409c4fc5: b8 e8 00 00 00              	movl	$0xe8, %eax
1409c4fca: 48 01 c2                    	addq	%rax, %rdx
1409c4fcd: f3 0f 11 44 24 20           	movss	%xmm0, 0x20(%rsp)
1409c4fd3: 48 8d 8d 38 0c 00 00        	leaq	0xc38(%rbp), %rcx
1409c4fda: 49 b8 c5 b9 41 2b 38 8d fc a6       	movabsq	$-0x590372c7d4be463b, %r8 # imm = 0xA6FC8D382B41B9C5
1409c4fe4: e8 57 69 fa ff              	callq	0x14096b940 <.text+0x96a940>
1409c4fe9: 41 b8 f8 00 00 00           	movl	$0xf8, %r8d
1409c4fef: 48 8d 8d 80 0a 00 00        	leaq	0xa80(%rbp), %rcx
1409c4ff6: 4c 8d b5 80 0b 00 00        	leaq	0xb80(%rbp), %r14
1409c4ffd: 4c 89 f2                    	movq	%r14, %rdx
1409c5000: e8 fb eb f5 01              	callq	0x142923c00 <.text+0x2922c00>
1409c5005: 80 bd 5e 0b 00 00 08        	cmpb	$0x8, 0xb5e(%rbp)
1409c500c: 74 2e                       	je	0x1409c503c <.text+0x9c403c>
1409c500e: 48 8b 95 68 0b 00 00        	movq	0xb68(%rbp), %rdx
1409c5015: b8 e8 00 00 00              	movl	$0xe8, %eax
1409c501a: 48 01 c2                    	addq	%rax, %rdx
1409c501d: f3 0f 11 7c 24 20           	movss	%xmm7, 0x20(%rsp)
1409c5023: 48 8d 8d 38 0b 00 00        	leaq	0xb38(%rbp), %rcx
1409c502a: 49 b8 87 47 1c 99 9b ab fd 69       	movabsq	$0x69fdab9b991c4787, %r8 # imm = 0x69FDAB9B991C4787
1409c5034: 0f 28 de                    	movaps	%xmm6, %xmm3
1409c5037: e8 04 69 fa ff              	callq	0x14096b940 <.text+0x96a940>
1409c503c: 41 b8 f8 00 00 00           	movl	$0xf8, %r8d
1409c5042: 48 8d 8d 90 08 00 00        	leaq	0x890(%rbp), %rcx
1409c5049: 48 8d 95 80 0a 00 00        	leaq	0xa80(%rbp), %rdx
1409c5050: e8 ab eb f5 01              	callq	0x142923c00 <.text+0x2922c00>
1409c5055: 80 bd 6e 09 00 00 08        	cmpb	$0x8, 0x96e(%rbp)
1409c505c: 74 2c                       	je	0x1409c508a <.text+0x9c408a>
1409c505e: 4d 85 ed                    	testq	%r13, %r13
1409c5061: 41 0f 95 c1                 	setne	%r9b
1409c5065: 48 8b 95 78 09 00 00        	movq	0x978(%rbp), %rdx
1409c506c: b8 e8 00 00 00              	movl	$0xe8, %eax
1409c5071: 48 01 c2                    	addq	%rax, %rdx
1409c5074: 48 8d 8d 48 09 00 00        	leaq	0x948(%rbp), %rcx
1409c507b: 49 b8 25 39 5f 45 f7 4a 59 35       	movabsq	$0x35594af7455f3925, %r8 # imm = 0x35594AF7455F3925
1409c5085: e8 c6 67 fa ff              	callq	0x14096b850 <.text+0x96a850>
1409c508a: 41 b8 f8 00 00 00           	movl	$0xf8, %r8d
1409c5090: 48 8d 8d 98 07 00 00        	leaq	0x798(%rbp), %rcx
1409c5097: 48 8d 95 90 08 00 00        	leaq	0x890(%rbp), %rdx
1409c509e: e8 5d eb f5 01              	callq	0x142923c00 <.text+0x2922c00>
1409c50a3: 48 85 f6                    	testq	%rsi, %rsi
1409c50a6: 74 10                       	je	0x1409c50b8 <.text+0x9c40b8>
1409c50a8: 44 0f b6 4e 02              	movzbl	0x2(%rsi), %r9d
1409c50ad: 80 bd 76 08 00 00 08        	cmpb	$0x8, 0x876(%rbp)
1409c50b4: 75 0e                       	jne	0x1409c50c4 <.text+0x9c40c4>
1409c50b6: eb 31                       	jmp	0x1409c50e9 <.text+0x9c40e9>
1409c50b8: 45 31 c9                    	xorl	%r9d, %r9d
1409c50bb: 80 bd 76 08 00 00 08        	cmpb	$0x8, 0x876(%rbp)
1409c50c2: 74 25                       	je	0x1409c50e9 <.text+0x9c40e9>
1409c50c4: 48 8b 95 80 08 00 00        	movq	0x880(%rbp), %rdx
1409c50cb: b8 e8 00 00 00              	movl	$0xe8, %eax
1409c50d0: 48 01 c2                    	addq	%rax, %rdx
1409c50d3: 48 8d 8d 50 08 00 00        	leaq	0x850(%rbp), %rcx
1409c50da: 49 b8 67 6c 52 a5 d8 03 58 4c       	movabsq	$0x4c5803d8a5526c67, %r8 # imm = 0x4C5803D8A5526C67
1409c50e4: e8 67 67 fa ff              	callq	0x14096b850 <.text+0x96a850>
1409c50e9: 41 b8 f8 00 00 00           	movl	$0xf8, %r8d
1409c50ef: 48 8d b5 88 09 00 00        	leaq	0x988(%rbp), %rsi
1409c50f6: 48 89 f1                    	movq	%rsi, %rcx
1409c50f9: 48 8d 95 98 07 00 00        	leaq	0x798(%rbp), %rdx
1409c5100: e8 fb ea f5 01              	callq	0x142923c00 <.text+0x2922c00>
1409c5105: 48 8b 4f 08                 	movq	0x8(%rdi), %rcx
1409c5109: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
1409c510d: c6 85 b1 0d 00 00 01        	movb	$0x1, 0xdb1(%rbp)
1409c5114: e8 67 7c 2d 00              	callq	0x140c9cd80 <.text+0xc9bd80>
1409c5119: 48 89 85 90 08 00 00        	movq	%rax, 0x890(%rbp)
1409c5120: c7 85 b0 03 00 00 02 00 00 00       	movl	$0x2, 0x3b0(%rbp)
1409c512a: 4c 89 7c 24 20              	movq	%r15, 0x20(%rsp)
1409c512f: 48 8d 8d 90 08 00 00        	leaq	0x890(%rbp), %rcx
1409c5136: 48 89 f2                    	movq	%rsi, %rdx
1409c5139: 49 b8 87 fe 4b e7 c2 86 06 74       	movabsq	$0x740686c2e74bfe87, %r8 # imm = 0x740686C2E74BFE87
1409c5143: 41 b1 1d                    	movb	$0x1d, %r9b
1409c5146: e8 c5 34 f4 ff              	callq	0x140908610 <.text+0x907610>
1409c514b: 48 8b b5 88 09 00 00        	movq	0x988(%rbp), %rsi
1409c5152: 41 b8 a0 00 00 00           	movl	$0xa0, %r8d
1409c5158: 4c 89 f1                    	movq	%r14, %rcx
1409c515b: 48 8d 95 90 09 00 00        	leaq	0x990(%rbp), %rdx
1409c5162: e8 99 ea f5 01              	callq	0x142923c00 <.text+0x2922c00>
1409c5167: 0f b6 bd 30 0a 00 00        	movzbl	0xa30(%rbp), %edi
1409c516e: 4c 89 f2                    	movq	%r14, %rdx
1409c5171: 44 0f b7 b5 31 0a 00 00     	movzwl	0xa31(%rbp), %r14d
1409c5179: 44 0f b6 a5 33 0a 00 00     	movzbl	0xa33(%rbp), %r12d
1409c5181: 48 8d 85 90 09 00 00        	leaq	0x990(%rbp), %rax
1409c5188: 0f 10 80 a4 00 00 00        	movups	0xa4(%rax), %xmm0
1409c518f: 0f 10 88 b4 00 00 00        	movups	0xb4(%rax), %xmm1
1409c5196: 0f 10 90 c4 00 00 00        	movups	0xc4(%rax), %xmm2
1409c519d: 0f 10 98 d4 00 00 00        	movups	0xd4(%rax), %xmm3
1409c51a4: 0f 29 85 80 0a 00 00        	movaps	%xmm0, 0xa80(%rbp)
1409c51ab: 0f 29 8d 90 0a 00 00        	movaps	%xmm1, 0xa90(%rbp)
1409c51b2: 0f 29 95 a0 0a 00 00        	movaps	%xmm2, 0xaa0(%rbp)
1409c51b9: 0f 29 9d b0 0a 00 00        	movaps	%xmm3, 0xab0(%rbp)
1409c51c0: 0f 10 80 e0 00 00 00        	movups	0xe0(%rax), %xmm0
1409c51c7: 0f 11 85 bc 0a 00 00        	movups	%xmm0, 0xabc(%rbp)
1409c51ce: 40 80 e7 7f                 	andb	$0x7f, %dil
1409c51d2: 41 80 e4 9f                 	andb	$-0x61, %r12b
1409c51d6: 41 80 cc 40                 	orb	$0x40, %r12b
1409c51da: 48 89 b5 b0 03 00 00        	movq	%rsi, 0x3b0(%rbp)
1409c51e1: 41 b8 a0 00 00 00           	movl	$0xa0, %r8d
1409c51e7: 48 8d 8d b8 03 00 00        	leaq	0x3b8(%rbp), %rcx
1409c51ee: e8 0d ea f5 01              	callq	0x142923c00 <.text+0x2922c00>
1409c51f3: 40 88 bd 58 04 00 00        	movb	%dil, 0x458(%rbp)
1409c51fa: 66 44 89 b5 59 04 00 00     	movw	%r14w, 0x459(%rbp)
1409c5202: 44 88 a5 5b 04 00 00        	movb	%r12b, 0x45b(%rbp)
1409c5209: 0f 28 85 80 0a 00 00        	movaps	0xa80(%rbp), %xmm0
1409c5210: 0f 28 8d 90 0a 00 00        	movaps	0xa90(%rbp), %xmm1
1409c5217: 0f 28 95 a0 0a 00 00        	movaps	0xaa0(%rbp), %xmm2
1409c521e: 0f 28 9d b0 0a 00 00        	movaps	0xab0(%rbp), %xmm3
1409c5225: 48 8d 85 e0 03 00 00        	leaq	0x3e0(%rbp), %rax
1409c522c: 0f 11 40 7c                 	movups	%xmm0, 0x7c(%rax)
1409c5230: 0f 11 88 8c 00 00 00        	movups	%xmm1, 0x8c(%rax)
1409c5237: 0f 11 90 9c 00 00 00        	movups	%xmm2, 0x9c(%rax)
1409c523e: 0f 11 98 ac 00 00 00        	movups	%xmm3, 0xac(%rax)
1409c5245: 0f 10 85 bc 0a 00 00        	movups	0xabc(%rbp), %xmm0
1409c524c: 0f 11 80 b8 00 00 00        	movups	%xmm0, 0xb8(%rax)
1409c5253: ba 06 00 00 00              	movl	$0x6, %edx
1409c5258: 4c 89 f9                    	movq	%r15, %rcx
1409c525b: e8 30 18 f4 ff              	callq	0x140906a90 <.text+0x905a90>
1409c5260: 48 83 c3 30                 	addq	$0x30, %rbx
1409c5264: 48 3b 9d a8 0d 00 00        	cmpq	0xda8(%rbp), %rbx
1409c526b: 0f 85 cf f7 ff ff           	jne	0x1409c4a40 <.text+0x9c3a40>
1409c5271: 48 8b 85 80 0d 00 00        	movq	0xd80(%rbp), %rax
1409c5278: 48 85 c0                    	testq	%rax, %rax
1409c527b: 74 1a                       	je	0x1409c5297 <.text+0x9c4297>
1409c527d: 48 c1 e0 04                 	shlq	$0x4, %rax
1409c5281: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
1409c5285: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1409c528b: 48 8b 8d a0 0d 00 00        	movq	0xda0(%rbp), %rcx
1409c5292: e8 89 e9 c4 ff              	callq	0x140613c20 <.text+0x612c20>
1409c5297: c6 85 b2 0d 00 00 01        	movb	$0x1, 0xdb2(%rbp)
1409c529e: 48 8d 8d b0 03 00 00        	leaq	0x3b0(%rbp), %rcx
1409c52a5: 48 8b bd 98 0d 00 00        	movq	0xd98(%rbp), %rdi
1409c52ac: 48 89 fa                    	movq	%rdi, %rdx
1409c52af: e8 9c 45 c9 00              	callq	0x141659850 <.text+0x1658850>
1409c52b4: 48 8b 9d 58 0d 00 00        	movq	0xd58(%rbp), %rbx
1409c52bb: 48 8d b5 b0 03 00 00        	leaq	0x3b0(%rbp), %rsi
1409c52c2: 48 89 f1                    	movq	%rsi, %rcx
1409c52c5: 48 89 fa                    	movq	%rdi, %rdx
1409c52c8: e8 c3 2b c9 00              	callq	0x141657e90 <.text+0x1656e90>
1409c52cd: 48 8b 97 a0 05 00 00        	movq	0x5a0(%rdi), %rdx
1409c52d4: 48 8b 8d b0 03 00 00        	movq	0x3b0(%rbp), %rcx
1409c52db: 48 39 d1                    	cmpq	%rdx, %rcx
1409c52de: 0f 85 9a 07 00 00           	jne	0x1409c5a7e <.text+0x9c4a7e>
1409c52e4: 8b 87 b0 05 00 00           	movl	0x5b0(%rdi), %eax
1409c52ea: 48 8b 85 a8 04 00 00        	movq	0x4a8(%rbp), %rax
1409c52f1: 48 8b 8d b0 04 00 00        	movq	0x4b0(%rbp), %rcx
1409c52f8: 48 8d 0c 88                 	leaq	(%rax,%rcx,4), %rcx
1409c52fc: 0f b6 95 b8 04 00 00        	movzbl	0x4b8(%rbp), %edx
1409c5303: 0f 57 c0                    	xorps	%xmm0, %xmm0
1409c5306: 0f 11 85 d8 0b 00 00        	movups	%xmm0, 0xbd8(%rbp)
1409c530d: 0f 11 85 c8 0b 00 00        	movups	%xmm0, 0xbc8(%rbp)
1409c5314: 48 c7 85 e8 0b 00 00 00 00 00 00    	movq	$0x0, 0xbe8(%rbp)
1409c531f: 48 89 bd 80 0b 00 00        	movq	%rdi, 0xb80(%rbp)
1409c5326: 4c 8b 85 68 0d 00 00        	movq	0xd68(%rbp), %r8
1409c532d: 4c 89 85 88 0b 00 00        	movq	%r8, 0xb88(%rbp)
1409c5334: 4c 8b 85 60 0d 00 00        	movq	0xd60(%rbp), %r8
1409c533b: 4c 89 85 90 0b 00 00        	movq	%r8, 0xb90(%rbp)
1409c5342: 48 89 b5 98 0b 00 00        	movq	%rsi, 0xb98(%rbp)
1409c5349: 48 89 85 a0 0b 00 00        	movq	%rax, 0xba0(%rbp)
1409c5350: 48 89 8d a8 0b 00 00        	movq	%rcx, 0xba8(%rbp)
1409c5357: 48 c7 85 b0 0b 00 00 08 00 00 00    	movq	$0x8, 0xbb0(%rbp)
1409c5362: 48 c7 85 b8 0b 00 00 00 00 00 00    	movq	$0x0, 0xbb8(%rbp)
1409c536d: 48 c7 85 c0 0b 00 00 08 00 00 00    	movq	$0x8, 0xbc0(%rbp)
1409c5378: 88 95 f0 0b 00 00           	movb	%dl, 0xbf0(%rbp)
1409c537e: 8b 85 80 0a 00 00           	movl	0xa80(%rbp), %eax
1409c5384: 8b 8d 83 0a 00 00           	movl	0xa83(%rbp), %ecx
1409c538a: 89 85 f1 0b 00 00           	movl	%eax, 0xbf1(%rbp)
1409c5390: 89 8d f4 0b 00 00           	movl	%ecx, 0xbf4(%rbp)
1409c5396: 4c 8d 05 fb a5 0f 02        	leaq	0x20fa5fb(%rip), %r8    # 0x142abf998
1409c539d: 48 8d 8d 78 0c 00 00        	leaq	0xc78(%rbp), %rcx
1409c53a4: 48 8d 95 80 0b 00 00        	leaq	0xb80(%rbp), %rdx
1409c53ab: e8 d0 04 94 00              	callq	0x141305880 <.text+0x1304880>
1409c53b0: c6 85 b7 0d 00 00 01        	movb	$0x1, 0xdb7(%rbp)
1409c53b7: c6 85 b6 0d 00 00 01        	movb	$0x1, 0xdb6(%rbp)
1409c53be: 48 89 f1                    	movq	%rsi, %rcx
1409c53c1: e8 1a b8 c0 ff              	callq	0x1405d0be0 <.text+0x5cfbe0>
1409c53c6: c6 85 b7 0d 00 00 01        	movb	$0x1, 0xdb7(%rbp)
1409c53cd: c6 85 b6 0d 00 00 01        	movb	$0x1, 0xdb6(%rbp)
1409c53d4: 48 ba b3 61 b4 27 07 ee 97 85       	movabsq	$-0x7a6811f8d84b9e4d, %rdx # imm = 0x8597EE0727B461B3
1409c53de: 49 b8 5c f1 15 ae c5 7e 29 8b       	movabsq	$-0x74d6813a51ea0ea4, %r8 # imm = 0x8B297EC5AE15F15C
1409c53e8: 48 89 d9                    	movq	%rbx, %rcx
1409c53eb: e8 00 83 71 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
1409c53f0: 48 85 c0                    	testq	%rax, %rax
1409c53f3: 0f 84 f6 06 00 00           	je	0x1409c5aef <.text+0x9c4aef>
1409c53f9: 48 39 97 18 02 00 00        	cmpq	%rdx, 0x218(%rdi)
1409c5400: 0f 86 e9 06 00 00           	jbe	0x1409c5aef <.text+0x9c4aef>
1409c5406: 48 8b 87 10 02 00 00        	movq	0x210(%rdi), %rax
1409c540d: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
1409c5411: 48 85 c9                    	testq	%rcx, %rcx
1409c5414: 0f 84 d5 06 00 00           	je	0x1409c5aef <.text+0x9c4aef>
1409c541a: 48 8b 87 e0 01 00 00        	movq	0x1e0(%rdi), %rax
1409c5421: 48 f7 d1                    	notq	%rcx
1409c5424: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1409c5428: 48 c1 e1 05                 	shlq	$0x5, %rcx
1409c542c: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
1409c5432: 0f 84 b7 06 00 00           	je	0x1409c5aef <.text+0x9c4aef>
1409c5438: 48 01 c8                    	addq	%rcx, %rax
1409c543b: 48 8b 48 10                 	movq	0x10(%rax), %rcx
1409c543f: c6 85 b7 0d 00 00 01        	movb	$0x1, 0xdb7(%rbp)
1409c5446: c6 85 b6 0d 00 00 01        	movb	$0x1, 0xdb6(%rbp)
1409c544d: 48 8d 15 9c ab 0f 02        	leaq	0x20fab9c(%rip), %rdx   # 0x142abfff0
1409c5454: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1409c545a: e8 41 26 f7 ff              	callq	0x140937aa0 <.text+0x936aa0>
1409c545f: 48 89 95 88 0d 00 00        	movq	%rdx, 0xd88(%rbp)
1409c5466: a8 01                       	testb	$0x1, %al
1409c5468: 0f 84 d5 06 00 00           	je	0x1409c5b43 <.text+0x9c4b43>
1409c546e: c6 85 b7 0d 00 00 01        	movb	$0x1, 0xdb7(%rbp)
1409c5475: c6 85 b6 0d 00 00 01        	movb	$0x1, 0xdb6(%rbp)
1409c547c: 48 ba 29 f7 80 5b 19 62 c3 4b       	movabsq	$0x4bc362195b80f729, %rdx # imm = 0x4BC362195B80F729
1409c5486: 49 b8 fb ce cb 80 ce 98 27 98       	movabsq	$-0x67d867317f343105, %r8 # imm = 0x982798CE80CBCEFB
1409c5490: 48 89 d9                    	movq	%rbx, %rcx
1409c5493: e8 58 82 71 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
1409c5498: 48 85 c0                    	testq	%rax, %rax
1409c549b: 0f 84 6a 06 00 00           	je	0x1409c5b0b <.text+0x9c4b0b>
1409c54a1: 48 39 97 18 02 00 00        	cmpq	%rdx, 0x218(%rdi)
1409c54a8: 0f 86 5d 06 00 00           	jbe	0x1409c5b0b <.text+0x9c4b0b>
1409c54ae: 48 8b 87 10 02 00 00        	movq	0x210(%rdi), %rax
1409c54b5: 48 8b 04 d0                 	movq	(%rax,%rdx,8), %rax
1409c54b9: 48 85 c0                    	testq	%rax, %rax
1409c54bc: 0f 84 49 06 00 00           	je	0x1409c5b0b <.text+0x9c4b0b>
1409c54c2: 48 8b 8f e0 01 00 00        	movq	0x1e0(%rdi), %rcx
1409c54c9: 48 f7 d0                    	notq	%rax
1409c54cc: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
1409c54d0: 48 c1 e0 05                 	shlq	$0x5, %rax
1409c54d4: 48 83 7c 01 20 00           	cmpq	$0x0, 0x20(%rcx,%rax)
1409c54da: 0f 84 2b 06 00 00           	je	0x1409c5b0b <.text+0x9c4b0b>
1409c54e0: 48 8b 85 78 0c 00 00        	movq	0xc78(%rbp), %rax
1409c54e7: 48 89 85 90 0d 00 00        	movq	%rax, 0xd90(%rbp)
1409c54ee: 4c 8b bd 80 0c 00 00        	movq	0xc80(%rbp), %r15
1409c54f5: 48 8b 85 88 0c 00 00        	movq	0xc88(%rbp), %rax
1409c54fc: 48 c1 e0 04                 	shlq	$0x4, %rax
1409c5500: 4c 01 f8                    	addq	%r15, %rax
1409c5503: 48 89 85 a8 0d 00 00        	movq	%rax, 0xda8(%rbp)
1409c550a: 4c 8d ad a0 07 00 00        	leaq	0x7a0(%rbp), %r13
1409c5511: 4c 8d a5 80 0b 00 00        	leaq	0xb80(%rbp), %r12
1409c5518: 4c 89 bd 98 0d 00 00        	movq	%r15, 0xd98(%rbp)
1409c551f: 4c 3b bd a8 0d 00 00        	cmpq	0xda8(%rbp), %r15
1409c5526: 0f 84 05 04 00 00           	je	0x1409c5931 <.text+0x9c4931>
1409c552c: 0f 1f 40 00                 	nopl	(%rax)
1409c5530: 49 8b 1f                    	movq	(%r15), %rbx
1409c5533: 4d 8b 77 08                 	movq	0x8(%r15), %r14
1409c5537: e8 14 12 22 00              	callq	0x140be6750 <.text+0xbe5750>
1409c553c: 89 c7                       	movl	%eax, %edi
1409c553e: 31 c9                       	xorl	%ecx, %ecx
1409c5540: e8 db 45 71 ff              	callq	0x1400d9b20 <.text+0xd8b20>
1409c5545: 48 85 c0                    	testq	%rax, %rax
1409c5548: 0f 84 41 05 00 00           	je	0x1409c5a8f <.text+0x9c4a8f>
1409c554e: 89 bd 30 0a 00 00           	movl	%edi, 0xa30(%rbp)
1409c5554: 66 c7 85 38 0a 00 00 00 00  	movw	$0x0, 0xa38(%rbp)
1409c555d: c7 85 34 0a 00 00 00 00 80 3f       	movl	$0x3f800000, 0xa34(%rbp) # imm = 0x3F800000
1409c5567: 48 c7 85 88 09 00 00 00 00 00 00    	movq	$0x0, 0x988(%rbp)
1409c5572: 48 8b 8d 88 0d 00 00        	movq	0xd88(%rbp), %rcx
1409c5579: 48 89 8d 28 0a 00 00        	movq	%rcx, 0xa28(%rbp)
1409c5580: 48 c7 85 98 09 00 00 00 00 00 00    	movq	$0x0, 0x998(%rbp)
1409c558b: 48 c7 85 a0 09 00 00 04 00 00 00    	movq	$0x4, 0x9a0(%rbp)
1409c5596: 48 c7 85 a8 09 00 00 00 00 00 00    	movq	$0x0, 0x9a8(%rbp)
1409c55a1: 0f 10 00                    	movups	(%rax), %xmm0
1409c55a4: 48 ff 00                    	incq	(%rax)
1409c55a7: 0f 10 0d 82 11 0d 02        	movups	0x20d1182(%rip), %xmm1  # 0x142a96730
1409c55ae: 48 8d 85 f8 09 00 00        	leaq	0x9f8(%rbp), %rax
1409c55b5: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
1409c55b9: 0f 10 0d 60 11 0d 02        	movups	0x20d1160(%rip), %xmm1  # 0x142a96720
1409c55c0: 0f 11 08                    	movups	%xmm1, (%rax)
1409c55c3: 0f 11 85 18 0a 00 00        	movups	%xmm0, 0xa18(%rbp)
1409c55ca: 48 b8 03 00 00 00 01 00 00 00       	movabsq	$0x100000003, %rax # imm = 0x100000003
1409c55d4: 48 89 85 c8 09 00 00        	movq	%rax, 0x9c8(%rbp)
1409c55db: 48 c7 85 b0 09 00 00 00 00 00 00    	movq	$0x0, 0x9b0(%rbp)
1409c55e6: 48 c7 85 b8 09 00 00 01 00 00 00    	movq	$0x1, 0x9b8(%rbp)
1409c55f1: 48 c7 85 c0 09 00 00 00 00 00 00    	movq	$0x0, 0x9c0(%rbp)
1409c55fc: 48 89 f1                    	movq	%rsi, %rcx
1409c55ff: 48 8d 95 88 09 00 00        	leaq	0x988(%rbp), %rdx
1409c5606: 4c 8d 45 c8                 	leaq	-0x38(%rbp), %r8
1409c560a: 45 31 c9                    	xorl	%r9d, %r9d
1409c560d: e8 ce 05 f4 ff              	callq	0x140905be0 <.text+0x904be0>
1409c5612: 80 bd 8e 04 00 00 08        	cmpb	$0x8, 0x48e(%rbp)
1409c5619: 74 2b                       	je	0x1409c5646 <.text+0x9c4646>
1409c561b: 48 8b 95 98 04 00 00        	movq	0x498(%rbp), %rdx
1409c5622: b8 e8 00 00 00              	movl	$0xe8, %eax
1409c5627: 48 01 c2                    	addq	%rax, %rdx
1409c562a: 48 8d 8d 68 04 00 00        	leaq	0x468(%rbp), %rcx
1409c5631: 49 b8 f5 51 86 b0 3f 67 ab c5       	movabsq	$-0x3a5498c04f79ae0b, %r8 # imm = 0xC5AB673FB08651F5
1409c563b: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1409c5641: e8 2a 5f fa ff              	callq	0x14096b570 <.text+0x96a570>
1409c5646: 41 b8 f8 00 00 00           	movl	$0xf8, %r8d
1409c564c: 4c 89 e1                    	movq	%r12, %rcx
1409c564f: 48 89 f2                    	movq	%rsi, %rdx
1409c5652: e8 a9 e5 f5 01              	callq	0x142923c00 <.text+0x2922c00>
1409c5657: 80 bd 5e 0c 00 00 08        	cmpb	$0x8, 0xc5e(%rbp)
1409c565e: 74 3c                       	je	0x1409c569c <.text+0x9c469c>
1409c5660: 48 8b 95 68 0c 00 00        	movq	0xc68(%rbp), %rdx
1409c5667: b8 e8 00 00 00              	movl	$0xe8, %eax
1409c566c: 48 01 c2                    	addq	%rax, %rdx
1409c566f: 8b 43 10                    	movl	0x10(%rbx), %eax
1409c5672: 89 85 b8 03 00 00           	movl	%eax, 0x3b8(%rbp)
1409c5678: 48 8b 43 08                 	movq	0x8(%rbx), %rax
1409c567c: 48 89 85 b0 03 00 00        	movq	%rax, 0x3b0(%rbp)
1409c5683: 48 8d 8d 38 0c 00 00        	leaq	0xc38(%rbp), %rcx
1409c568a: 49 b8 0f b5 93 3e bd 1f 0d 12       	movabsq	$0x120d1fbd3e93b50f, %r8 # imm = 0x120D1FBD3E93B50F
1409c5694: 49 89 f1                    	movq	%rsi, %r9
1409c5697: e8 c4 5f fa ff              	callq	0x14096b660 <.text+0x96a660>
1409c569c: 41 b8 f8 00 00 00           	movl	$0xf8, %r8d
1409c56a2: 48 8d 8d 80 0a 00 00        	leaq	0xa80(%rbp), %rcx
1409c56a9: 4c 89 e2                    	movq	%r12, %rdx
1409c56ac: e8 4f e5 f5 01              	callq	0x142923c00 <.text+0x2922c00>
1409c56b1: 80 bd 5e 0b 00 00 08        	cmpb	$0x8, 0xb5e(%rbp)
1409c56b8: 74 34                       	je	0x1409c56ee <.text+0x9c46ee>
1409c56ba: f3 0f 10 1b                 	movss	(%rbx), %xmm3
1409c56be: f3 0f 10 43 04              	movss	0x4(%rbx), %xmm0
1409c56c3: 48 8b 95 68 0b 00 00        	movq	0xb68(%rbp), %rdx
1409c56ca: b8 e8 00 00 00              	movl	$0xe8, %eax
1409c56cf: 48 01 c2                    	addq	%rax, %rdx
1409c56d2: f3 0f 11 44 24 20           	movss	%xmm0, 0x20(%rsp)
1409c56d8: 48 8d 8d 38 0b 00 00        	leaq	0xb38(%rbp), %rcx
1409c56df: 49 b8 c5 b9 41 2b 38 8d fc a6       	movabsq	$-0x590372c7d4be463b, %r8 # imm = 0xA6FC8D382B41B9C5
1409c56e9: e8 52 62 fa ff              	callq	0x14096b940 <.text+0x96a940>
1409c56ee: 41 b8 f8 00 00 00           	movl	$0xf8, %r8d
1409c56f4: 48 8d 8d 90 08 00 00        	leaq	0x890(%rbp), %rcx
1409c56fb: 48 8d 95 80 0a 00 00        	leaq	0xa80(%rbp), %rdx
1409c5702: e8 f9 e4 f5 01              	callq	0x142923c00 <.text+0x2922c00>
1409c5707: 80 bd 6e 09 00 00 08        	cmpb	$0x8, 0x96e(%rbp)
1409c570e: 74 2e                       	je	0x1409c573e <.text+0x9c473e>
1409c5710: 48 8b 95 78 09 00 00        	movq	0x978(%rbp), %rdx
1409c5717: b8 e8 00 00 00              	movl	$0xe8, %eax
1409c571c: 48 01 c2                    	addq	%rax, %rdx
1409c571f: f3 0f 11 7c 24 20           	movss	%xmm7, 0x20(%rsp)
1409c5725: 48 8d 8d 48 09 00 00        	leaq	0x948(%rbp), %rcx
1409c572c: 49 b8 87 47 1c 99 9b ab fd 69       	movabsq	$0x69fdab9b991c4787, %r8 # imm = 0x69FDAB9B991C4787
1409c5736: 0f 28 de                    	movaps	%xmm6, %xmm3
1409c5739: e8 02 62 fa ff              	callq	0x14096b940 <.text+0x96a940>
1409c573e: 41 b8 f8 00 00 00           	movl	$0xf8, %r8d
1409c5744: 48 8d 8d 98 07 00 00        	leaq	0x798(%rbp), %rcx
1409c574b: 48 8d 95 90 08 00 00        	leaq	0x890(%rbp), %rdx
1409c5752: e8 a9 e4 f5 01              	callq	0x142923c00 <.text+0x2922c00>
1409c5757: 49 8b 4e 08                 	movq	0x8(%r14), %rcx
1409c575b: 49 8b 56 10                 	movq	0x10(%r14), %rdx
1409c575f: c6 85 b4 0d 00 00 01        	movb	$0x1, 0xdb4(%rbp)
1409c5766: e8 15 76 2d 00              	callq	0x140c9cd80 <.text+0xc9bd80>
1409c576b: 48 89 c7                    	movq	%rax, %rdi
1409c576e: c7 85 b0 03 00 00 00 00 00 00       	movl	$0x0, 0x3b0(%rbp)
1409c5778: c7 85 b8 03 00 00 00 00 00 00       	movl	$0x0, 0x3b8(%rbp)
1409c5782: c7 85 c0 03 00 00 00 00 00 00       	movl	$0x0, 0x3c0(%rbp)
1409c578c: c7 85 c8 03 00 00 00 00 00 00       	movl	$0x0, 0x3c8(%rbp)
1409c5796: c7 85 d0 03 00 00 00 00 00 00       	movl	$0x0, 0x3d0(%rbp)
1409c57a0: c7 85 d8 03 00 00 00 00 00 00       	movl	$0x0, 0x3d8(%rbp)
1409c57aa: 48 8d 85 e0 03 00 00        	leaq	0x3e0(%rbp), %rax
1409c57b1: c6 40 08 00                 	movb	$0x0, 0x8(%rax)
1409c57b5: 48 c7 00 00 00 00 00        	movq	$0x0, (%rax)
1409c57bc: c6 85 b4 0d 00 00 01        	movb	$0x1, 0xdb4(%rbp)
1409c57c3: 4c 89 e1                    	movq	%r12, %rcx
1409c57c6: 48 89 f2                    	movq	%rsi, %rdx
1409c57c9: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1409c57cf: e8 0c 00 21 00              	callq	0x140bd57e0 <.text+0xbd47e0>
1409c57d4: 45 8b 46 18                 	movl	0x18(%r14), %r8d
1409c57d8: c6 85 b4 0d 00 00 01        	movb	$0x1, 0xdb4(%rbp)
1409c57df: 48 89 f1                    	movq	%rsi, %rcx
1409c57e2: 4c 89 e2                    	movq	%r12, %rdx
1409c57e5: e8 76 01 21 00              	callq	0x140bd5960 <.text+0xbd4960>
1409c57ea: c6 85 b4 0d 00 00 01        	movb	$0x1, 0xdb4(%rbp)
1409c57f1: 4c 89 e1                    	movq	%r12, %rcx
1409c57f4: 48 89 f2                    	movq	%rsi, %rdx
1409c57f7: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1409c57fd: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1409c5803: e8 98 01 21 00              	callq	0x140bd59a0 <.text+0xbd49a0>
1409c5808: 48 89 bd 80 0a 00 00        	movq	%rdi, 0xa80(%rbp)
1409c580f: 0f 10 85 80 0b 00 00        	movups	0xb80(%rbp), %xmm0
1409c5816: 0f 10 8d 90 0b 00 00        	movups	0xb90(%rbp), %xmm1
1409c581d: 0f 10 95 a0 0b 00 00        	movups	0xba0(%rbp), %xmm2
1409c5824: 0f 10 9d ac 0b 00 00        	movups	0xbac(%rbp), %xmm3
1409c582b: 0f 11 9d dc 03 00 00        	movups	%xmm3, 0x3dc(%rbp)
1409c5832: 0f 29 95 d0 03 00 00        	movaps	%xmm2, 0x3d0(%rbp)
1409c5839: 0f 29 8d c0 03 00 00        	movaps	%xmm1, 0x3c0(%rbp)
1409c5840: 0f 29 85 b0 03 00 00        	movaps	%xmm0, 0x3b0(%rbp)
1409c5847: 48 89 74 24 20              	movq	%rsi, 0x20(%rsp)
1409c584c: 48 8d 8d 80 0a 00 00        	leaq	0xa80(%rbp), %rcx
1409c5853: 48 8d 95 98 07 00 00        	leaq	0x798(%rbp), %rdx
1409c585a: 49 b8 87 fe 4b e7 c2 86 06 74       	movabsq	$0x740686c2e74bfe87, %r8 # imm = 0x740686C2E74BFE87
1409c5864: 41 b1 1d                    	movb	$0x1d, %r9b
1409c5867: e8 a4 2d f4 ff              	callq	0x140908610 <.text+0x907610>
1409c586c: 48 8b bd 98 07 00 00        	movq	0x798(%rbp), %rdi
1409c5873: 41 b8 a0 00 00 00           	movl	$0xa0, %r8d
1409c5879: 48 8d 8d b8 03 00 00        	leaq	0x3b8(%rbp), %rcx
1409c5880: 4c 89 ea                    	movq	%r13, %rdx
1409c5883: e8 78 e3 f5 01              	callq	0x142923c00 <.text+0x2922c00>
1409c5888: 0f b6 85 40 08 00 00        	movzbl	0x840(%rbp), %eax
1409c588f: 0f b7 8d 41 08 00 00        	movzwl	0x841(%rbp), %ecx
1409c5896: 0f b6 95 43 08 00 00        	movzbl	0x843(%rbp), %edx
1409c589d: 41 0f 10 85 a4 00 00 00     	movups	0xa4(%r13), %xmm0
1409c58a5: 41 0f 10 8d b4 00 00 00     	movups	0xb4(%r13), %xmm1
1409c58ad: 41 0f 10 95 c4 00 00 00     	movups	0xc4(%r13), %xmm2
1409c58b5: 41 0f 10 9d d4 00 00 00     	movups	0xd4(%r13), %xmm3
1409c58bd: 4c 8d 85 e0 03 00 00        	leaq	0x3e0(%rbp), %r8
1409c58c4: 41 0f 11 40 7c              	movups	%xmm0, 0x7c(%r8)
1409c58c9: 41 0f 11 88 8c 00 00 00     	movups	%xmm1, 0x8c(%r8)
1409c58d1: 41 0f 11 90 9c 00 00 00     	movups	%xmm2, 0x9c(%r8)
1409c58d9: 41 0f 11 98 ac 00 00 00     	movups	%xmm3, 0xac(%r8)
1409c58e1: 41 0f 10 85 e0 00 00 00     	movups	0xe0(%r13), %xmm0
1409c58e9: 41 0f 11 80 b8 00 00 00     	movups	%xmm0, 0xb8(%r8)
1409c58f1: 24 7f                       	andb	$0x7f, %al
1409c58f3: 80 e2 9f                    	andb	$-0x61, %dl
1409c58f6: 80 ca 40                    	orb	$0x40, %dl
1409c58f9: 48 89 bd b0 03 00 00        	movq	%rdi, 0x3b0(%rbp)
1409c5900: 88 85 58 04 00 00           	movb	%al, 0x458(%rbp)
1409c5906: 66 89 8d 59 04 00 00        	movw	%cx, 0x459(%rbp)
1409c590d: 88 95 5b 04 00 00           	movb	%dl, 0x45b(%rbp)
1409c5913: ba 06 00 00 00              	movl	$0x6, %edx
1409c5918: 48 89 f1                    	movq	%rsi, %rcx
1409c591b: e8 70 11 f4 ff              	callq	0x140906a90 <.text+0x905a90>
1409c5920: 49 83 c7 10                 	addq	$0x10, %r15
1409c5924: 4c 3b bd a8 0d 00 00        	cmpq	0xda8(%rbp), %r15
1409c592b: 0f 85 ff fb ff ff           	jne	0x1409c5530 <.text+0x9c4530>
1409c5931: 48 8b 95 90 0d 00 00        	movq	0xd90(%rbp), %rdx
1409c5938: 48 85 d2                    	testq	%rdx, %rdx
1409c593b: 74 16                       	je	0x1409c5953 <.text+0x9c4953>
1409c593d: 48 c1 e2 04                 	shlq	$0x4, %rdx
1409c5941: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1409c5947: 48 8b 8d 98 0d 00 00        	movq	0xd98(%rbp), %rcx
1409c594e: e8 cd e2 c4 ff              	callq	0x140613c20 <.text+0x612c20>
1409c5953: 48 8d b5 b0 03 00 00        	leaq	0x3b0(%rbp), %rsi
1409c595a: 48 8d 55 c8                 	leaq	-0x38(%rbp), %rdx
1409c595e: 41 b8 e8 03 00 00           	movl	$0x3e8, %r8d            # imm = 0x3E8
1409c5964: 48 89 f1                    	movq	%rsi, %rcx
1409c5967: e8 94 e2 f5 01              	callq	0x142923c00 <.text+0x2922c00>
1409c596c: c6 85 b7 0d 00 00 00        	movb	$0x0, 0xdb7(%rbp)
1409c5973: c6 85 b6 0d 00 00 00        	movb	$0x0, 0xdb6(%rbp)
1409c597a: 48 89 f1                    	movq	%rsi, %rcx
1409c597d: e8 ee 29 ea ff              	callq	0x140868370 <.text+0x867370>
1409c5982: 48 83 bd 70 0d 00 00 00     	cmpq	$0x0, 0xd70(%rbp)
1409c598a: 74 47                       	je	0x1409c59d3 <.text+0x9c49d3>
1409c598c: e8 9f 27 25 00              	callq	0x140c18130 <.text+0xc17130>
1409c5991: 48 89 c1                    	movq	%rax, %rcx
1409c5994: 48 8b 00                    	movq	(%rax), %rax
1409c5997: 48 83 f8 01                 	cmpq	$0x1, %rax
1409c599b: 75 58                       	jne	0x1409c59f5 <.text+0x9c49f5>
1409c599d: 48 83 c1 08                 	addq	$0x8, %rcx
1409c59a1: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1409c59a5: 0f 85 84 00 00 00           	jne	0x1409c5a2f <.text+0x9c4a2f>
1409c59ab: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1409c59b2: 48 89 8d a8 0d 00 00        	movq	%rcx, 0xda8(%rbp)
1409c59b9: 48 83 c1 08                 	addq	$0x8, %rcx
1409c59bd: 48 8b 95 78 0d 00 00        	movq	0xd78(%rbp), %rdx
1409c59c4: e8 27 17 25 00              	callq	0x140c170f0 <.text+0xc160f0>
1409c59c9: 48 8b 85 a8 0d 00 00        	movq	0xda8(%rbp), %rax
1409c59d0: 48 ff 00                    	incq	(%rax)
1409c59d3: 0f 28 b5 c0 0d 00 00        	movaps	0xdc0(%rbp), %xmm6
1409c59da: 0f 28 bd d0 0d 00 00        	movaps	0xdd0(%rbp), %xmm7
1409c59e1: 48 81 c4 68 0e 00 00        	addq	$0xe68, %rsp            # imm = 0xE68
1409c59e8: 5b                          	popq	%rbx
1409c59e9: 5f                          	popq	%rdi
1409c59ea: 5e                          	popq	%rsi
1409c59eb: 41 5c                       	popq	%r12
1409c59ed: 41 5d                       	popq	%r13
1409c59ef: 41 5e                       	popq	%r14
1409c59f1: 41 5f                       	popq	%r15
1409c59f3: 5d                          	popq	%rbp
1409c59f4: c3                          	retq
1409c59f5: 48 85 c0                    	testq	%rax, %rax
1409c59f8: 75 29                       	jne	0x1409c5a23 <.text+0x9c4a23>
1409c59fa: 31 d2                       	xorl	%edx, %edx
1409c59fc: e8 cf 2d f3 01              	callq	0x1428f87d0 <.text+0x28f77d0>
1409c5a01: 48 89 c1                    	movq	%rax, %rcx
1409c5a04: 48 85 c0                    	testq	%rax, %rax
1409c5a07: 75 98                       	jne	0x1409c59a1 <.text+0x9c49a1>
1409c5a09: eb 18                       	jmp	0x1409c5a23 <.text+0x9c4a23>
1409c5a0b: 48 85 c0                    	testq	%rax, %rax
1409c5a0e: 75 13                       	jne	0x1409c5a23 <.text+0x9c4a23>
1409c5a10: 31 d2                       	xorl	%edx, %edx
1409c5a12: e8 b9 2d f3 01              	callq	0x1428f87d0 <.text+0x28f77d0>
1409c5a17: 48 89 c1                    	movq	%rax, %rcx
1409c5a1a: 48 85 c0                    	testq	%rax, %rax
1409c5a1d: 0f 85 69 ec ff ff           	jne	0x1409c468c <.text+0x9c368c>
1409c5a23: 48 8d 0d 76 50 27 02        	leaq	0x2275076(%rip), %rcx   # 0x142c3aaa0
1409c5a2a: e8 a1 26 f1 01              	callq	0x1428d80d0 <.text+0x28d70d0>
1409c5a2f: 48 8d 0d 3a 51 27 02        	leaq	0x227513a(%rip), %rcx   # 0x142c3ab70
1409c5a36: e8 15 38 f1 01              	callq	0x1428d9250 <.text+0x28d8250>
1409c5a3b: 4c 8d 05 a6 be 2b 02        	leaq	0x22bbea6(%rip), %r8    # 0x142c818e8
1409c5a42: e8 b9 97 f0 01              	callq	0x1428cf200 <.text+0x28ce200>
1409c5a47: e9 11 01 00 00              	jmp	0x1409c5b5d <.text+0x9c4b5d>
1409c5a4c: 49 83 fe 15                 	cmpq	$0x15, %r14
1409c5a50: 0f 83 09 01 00 00           	jae	0x1409c5b5f <.text+0x9c4b5f>
1409c5a56: c6 85 b5 0d 00 00 01        	movb	$0x1, 0xdb5(%rbp)
1409c5a5d: 4c 8d 8d b0 03 00 00        	leaq	0x3b0(%rbp), %r9
1409c5a64: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1409c5a6a: 48 8b 8d a0 0d 00 00        	movq	0xda0(%rbp), %rcx
1409c5a71: 4c 89 f2                    	movq	%r14, %rdx
1409c5a74: e8 57 2c 97 00              	callq	0x1413386d0 <.text+0x13376d0>
1409c5a79: e9 0e ef ff ff              	jmp	0x1409c498c <.text+0x9c398c>
1409c5a7e: 4c 8d 05 63 be 2b 02        	leaq	0x22bbe63(%rip), %r8    # 0x142c818e8
1409c5a85: e8 76 97 f0 01              	callq	0x1428cf200 <.text+0x28ce200>
1409c5a8a: e9 ce 00 00 00              	jmp	0x1409c5b5d <.text+0x9c4b5d>
1409c5a8f: 48 8d 0d 0a 50 27 02        	leaq	0x227500a(%rip), %rcx   # 0x142c3aaa0
1409c5a96: e8 35 26 f1 01              	callq	0x1428d80d0 <.text+0x28d70d0>
1409c5a9b: e9 bd 00 00 00              	jmp	0x1409c5b5d <.text+0x9c4b5d>
1409c5aa0: 48 8d 0d f9 4f 27 02        	leaq	0x2274ff9(%rip), %rcx   # 0x142c3aaa0
1409c5aa7: e8 24 26 f1 01              	callq	0x1428d80d0 <.text+0x28d70d0>
1409c5aac: e9 ac 00 00 00              	jmp	0x1409c5b5d <.text+0x9c4b5d>
1409c5ab1: 48 8d 0d e8 4f 27 02        	leaq	0x2274fe8(%rip), %rcx   # 0x142c3aaa0
1409c5ab8: e8 13 26 f1 01              	callq	0x1428d80d0 <.text+0x28d70d0>
1409c5abd: e9 9b 00 00 00              	jmp	0x1409c5b5d <.text+0x9c4b5d>
1409c5ac2: 48 8d 0d 6f a4 0f 02        	leaq	0x20fa46f(%rip), %rcx   # 0x142abff38
1409c5ac9: e8 42 38 f1 01              	callq	0x1428d9310 <.text+0x28d8310>
1409c5ace: e9 8a 00 00 00              	jmp	0x1409c5b5d <.text+0x9c4b5d>
1409c5ad3: 48 8d 0d 96 a4 0f 02        	leaq	0x20fa496(%rip), %rcx   # 0x142abff70
1409c5ada: e8 31 38 f1 01              	callq	0x1428d9310 <.text+0x28d8310>
1409c5adf: eb 7c                       	jmp	0x1409c5b5d <.text+0x9c4b5d>
1409c5ae1: 48 8d 0d d0 a4 0f 02        	leaq	0x20fa4d0(%rip), %rcx   # 0x142abffb8
1409c5ae8: e8 23 38 f1 01              	callq	0x1428d9310 <.text+0x28d8310>
1409c5aed: eb 6e                       	jmp	0x1409c5b5d <.text+0x9c4b5d>
1409c5aef: c6 85 b7 0d 00 00 01        	movb	$0x1, 0xdb7(%rbp)
1409c5af6: c6 85 b6 0d 00 00 01        	movb	$0x1, 0xdb6(%rbp)
1409c5afd: 48 8d 0d d4 a4 0f 02        	leaq	0x20fa4d4(%rip), %rcx   # 0x142abffd8
1409c5b04: e8 07 38 f1 01              	callq	0x1428d9310 <.text+0x28d8310>
1409c5b09: eb 52                       	jmp	0x1409c5b5d <.text+0x9c4b5d>
1409c5b0b: c6 85 b7 0d 00 00 01        	movb	$0x1, 0xdb7(%rbp)
1409c5b12: c6 85 b6 0d 00 00 01        	movb	$0x1, 0xdb6(%rbp)
1409c5b19: 48 8d 0d b8 00 0e 02        	leaq	0x20e00b8(%rip), %rcx   # 0x142aa5bd8
1409c5b20: e8 eb 37 f1 01              	callq	0x1428d9310 <.text+0x28d8310>
1409c5b25: eb 36                       	jmp	0x1409c5b5d <.text+0x9c4b5d>
1409c5b27: 48 8d 0d 2a a4 0f 02        	leaq	0x20fa42a(%rip), %rcx   # 0x142abff58
1409c5b2e: e8 dd 37 f1 01              	callq	0x1428d9310 <.text+0x28d8310>
1409c5b33: eb 28                       	jmp	0x1409c5b5d <.text+0x9c4b5d>
1409c5b35: 48 8d 0d 64 a4 0f 02        	leaq	0x20fa464(%rip), %rcx   # 0x142abffa0
1409c5b3c: e8 cf 37 f1 01              	callq	0x1428d9310 <.text+0x28d8310>
1409c5b41: eb 1a                       	jmp	0x1409c5b5d <.text+0x9c4b5d>
1409c5b43: c6 85 b7 0d 00 00 01        	movb	$0x1, 0xdb7(%rbp)
1409c5b4a: c6 85 b6 0d 00 00 01        	movb	$0x1, 0xdb6(%rbp)
1409c5b51: 48 8d 0d a0 a4 0f 02        	leaq	0x20fa4a0(%rip), %rcx   # 0x142abfff8
1409c5b58: e8 b3 37 f1 01              	callq	0x1428d9310 <.text+0x28d8310>
1409c5b5d: 0f 0b                       	ud2
1409c5b5f: c6 85 b5 0d 00 00 01        	movb	$0x1, 0xdb5(%rbp)
1409c5b66: 4c 8d 85 b0 03 00 00        	leaq	0x3b0(%rbp), %r8
1409c5b6d: 48 8b 8d a0 0d 00 00        	movq	0xda0(%rbp), %rcx
1409c5b74: 4c 89 f2                    	movq	%r14, %rdx
1409c5b77: e8 b4 62 18 00              	callq	0x140b4be30 <.text+0xb4ae30>
1409c5b7c: e9 0b ee ff ff              	jmp	0x1409c498c <.text+0x9c398c>
1409c5b81: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1409c5b90: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409c5b95: 55                          	pushq	%rbp
1409c5b96: 41 57                       	pushq	%r15
1409c5b98: 41 56                       	pushq	%r14
1409c5b9a: 41 55                       	pushq	%r13
1409c5b9c: 41 54                       	pushq	%r12
1409c5b9e: 56                          	pushq	%rsi
1409c5b9f: 57                          	pushq	%rdi
1409c5ba0: 53                          	pushq	%rbx
1409c5ba1: 48 83 ec 68                 	subq	$0x68, %rsp
1409c5ba5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409c5bac: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1409c5bb1: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
1409c5bb6: 48 8b 85 a8 0d 00 00        	movq	0xda8(%rbp), %rax
1409c5bbd: 48 ff 00                    	incq	(%rax)
1409c5bc0: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1409c5bc5: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1409c5bca: 48 83 c4 68                 	addq	$0x68, %rsp
1409c5bce: 5b                          	popq	%rbx
1409c5bcf: 5f                          	popq	%rdi
1409c5bd0: 5e                          	popq	%rsi
1409c5bd1: 41 5c                       	popq	%r12
1409c5bd3: 41 5d                       	popq	%r13
1409c5bd5: 41 5e                       	popq	%r14
1409c5bd7: 41 5f                       	popq	%r15
1409c5bd9: 5d                          	popq	%rbp
1409c5bda: c3                          	retq
1409c5bdb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
1409c5be0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409c5be5: 55                          	pushq	%rbp
1409c5be6: 41 57                       	pushq	%r15
1409c5be8: 41 56                       	pushq	%r14
1409c5bea: 41 55                       	pushq	%r13
1409c5bec: 41 54                       	pushq	%r12
1409c5bee: 56                          	pushq	%rsi
1409c5bef: 57                          	pushq	%rdi
1409c5bf0: 53                          	pushq	%rbx
1409c5bf1: 48 83 ec 68                 	subq	$0x68, %rsp
1409c5bf5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409c5bfc: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1409c5c01: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
1409c5c06: 48 83 bd 70 0d 00 00 00     	cmpq	$0x0, 0xd70(%rbp)
1409c5c0e: 74 2f                       	je	0x1409c5c3f <.text+0x9c4c3f>
1409c5c10: 48 8d 85 78 0d 00 00        	leaq	0xd78(%rbp), %rax
1409c5c17: 48 89 85 38 0d 00 00        	movq	%rax, 0xd38(%rbp)
1409c5c1e: 48 8d 85 38 0d 00 00        	leaq	0xd38(%rbp), %rax
1409c5c25: 48 89 85 30 0d 00 00        	movq	%rax, 0xd30(%rbp)
1409c5c2c: 48 8d 0d 5d 9e 0f 02        	leaq	0x20f9e5d(%rip), %rcx   # 0x142abfa90
1409c5c33: 48 8d 95 30 0d 00 00        	leaq	0xd30(%rbp), %rdx
1409c5c3a: e8 91 96 96 00              	callq	0x14132f2d0 <.text+0x132e2d0>
1409c5c3f: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1409c5c44: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1409c5c49: 48 83 c4 68                 	addq	$0x68, %rsp
1409c5c4d: 5b                          	popq	%rbx
1409c5c4e: 5f                          	popq	%rdi
1409c5c4f: 5e                          	popq	%rsi
1409c5c50: 41 5c                       	popq	%r12
1409c5c52: 41 5d                       	popq	%r13
1409c5c54: 41 5e                       	popq	%r14
1409c5c56: 41 5f                       	popq	%r15
1409c5c58: 5d                          	popq	%rbp
1409c5c59: c3                          	retq
1409c5c5a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1409c5c60: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409c5c65: 55                          	pushq	%rbp
1409c5c66: 41 57                       	pushq	%r15
1409c5c68: 41 56                       	pushq	%r14
1409c5c6a: 41 55                       	pushq	%r13
1409c5c6c: 41 54                       	pushq	%r12
1409c5c6e: 56                          	pushq	%rsi
1409c5c6f: 57                          	pushq	%rdi
1409c5c70: 53                          	pushq	%rbx
1409c5c71: 48 83 ec 68                 	subq	$0x68, %rsp
1409c5c75: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409c5c7c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1409c5c81: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
1409c5c86: 48 8d 8d b0 03 00 00        	leaq	0x3b0(%rbp), %rcx
1409c5c8d: e8 4e af c0 ff              	callq	0x1405d0be0 <.text+0x5cfbe0>
1409c5c92: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1409c5c97: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1409c5c9c: 48 83 c4 68                 	addq	$0x68, %rsp
1409c5ca0: 5b                          	popq	%rbx
1409c5ca1: 5f                          	popq	%rdi
1409c5ca2: 5e                          	popq	%rsi
1409c5ca3: 41 5c                       	popq	%r12
1409c5ca5: 41 5d                       	popq	%r13
1409c5ca7: 41 5e                       	popq	%r14
1409c5ca9: 41 5f                       	popq	%r15
1409c5cab: 5d                          	popq	%rbp
1409c5cac: c3                          	retq
1409c5cad: 0f 1f 00                    	nopl	(%rax)
1409c5cb0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409c5cb5: 55                          	pushq	%rbp
1409c5cb6: 41 57                       	pushq	%r15
1409c5cb8: 41 56                       	pushq	%r14
1409c5cba: 41 55                       	pushq	%r13
1409c5cbc: 41 54                       	pushq	%r12
1409c5cbe: 56                          	pushq	%rsi
1409c5cbf: 57                          	pushq	%rdi
1409c5cc0: 53                          	pushq	%rbx
1409c5cc1: 48 83 ec 68                 	subq	$0x68, %rsp
1409c5cc5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409c5ccc: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1409c5cd1: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
1409c5cd6: 48 8b 85 80 0d 00 00        	movq	0xd80(%rbp), %rax
1409c5cdd: 48 85 c0                    	testq	%rax, %rax
1409c5ce0: 74 1a                       	je	0x1409c5cfc <.text+0x9c4cfc>
1409c5ce2: 48 c1 e0 04                 	shlq	$0x4, %rax
1409c5ce6: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
1409c5cea: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1409c5cf0: 48 8b 8d a0 0d 00 00        	movq	0xda0(%rbp), %rcx
1409c5cf7: e8 24 df c4 ff              	callq	0x140613c20 <.text+0x612c20>
1409c5cfc: c6 85 b2 0d 00 00 01        	movb	$0x1, 0xdb2(%rbp)
1409c5d03: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1409c5d08: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1409c5d0d: 48 83 c4 68                 	addq	$0x68, %rsp
1409c5d11: 5b                          	popq	%rbx
1409c5d12: 5f                          	popq	%rdi
1409c5d13: 5e                          	popq	%rsi
1409c5d14: 41 5c                       	popq	%r12
1409c5d16: 41 5d                       	popq	%r13
1409c5d18: 41 5e                       	popq	%r14
1409c5d1a: 41 5f                       	popq	%r15
1409c5d1c: 5d                          	popq	%rbp
1409c5d1d: c3                          	retq
1409c5d1e: 66 90                       	nop
1409c5d20: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409c5d25: 55                          	pushq	%rbp
1409c5d26: 41 57                       	pushq	%r15
1409c5d28: 41 56                       	pushq	%r14
1409c5d2a: 41 55                       	pushq	%r13
1409c5d2c: 41 54                       	pushq	%r12
1409c5d2e: 56                          	pushq	%rsi
1409c5d2f: 57                          	pushq	%rdi
1409c5d30: 53                          	pushq	%rbx
1409c5d31: 48 83 ec 68                 	subq	$0x68, %rsp
1409c5d35: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409c5d3c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1409c5d41: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
1409c5d46: 48 8d 8d b0 03 00 00        	leaq	0x3b0(%rbp), %rcx
1409c5d4d: e8 8e ae c0 ff              	callq	0x1405d0be0 <.text+0x5cfbe0>
1409c5d52: c6 85 b2 0d 00 00 01        	movb	$0x1, 0xdb2(%rbp)
1409c5d59: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1409c5d5e: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1409c5d63: 48 83 c4 68                 	addq	$0x68, %rsp
1409c5d67: 5b                          	popq	%rbx
1409c5d68: 5f                          	popq	%rdi
1409c5d69: 5e                          	popq	%rsi
1409c5d6a: 41 5c                       	popq	%r12
1409c5d6c: 41 5d                       	popq	%r13
1409c5d6e: 41 5e                       	popq	%r14
1409c5d70: 41 5f                       	popq	%r15
1409c5d72: 5d                          	popq	%rbp
1409c5d73: c3                          	retq
1409c5d74: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
1409c5d80: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409c5d85: 55                          	pushq	%rbp
1409c5d86: 41 57                       	pushq	%r15
1409c5d88: 41 56                       	pushq	%r14
1409c5d8a: 41 55                       	pushq	%r13
1409c5d8c: 41 54                       	pushq	%r12
1409c5d8e: 56                          	pushq	%rsi
1409c5d8f: 57                          	pushq	%rdi
1409c5d90: 53                          	pushq	%rbx
1409c5d91: 48 83 ec 68                 	subq	$0x68, %rsp
1409c5d95: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409c5d9c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1409c5da1: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
1409c5da6: 48 8b 95 90 0d 00 00        	movq	0xd90(%rbp), %rdx
1409c5dad: 48 85 d2                    	testq	%rdx, %rdx
1409c5db0: 74 16                       	je	0x1409c5dc8 <.text+0x9c4dc8>
1409c5db2: 48 c1 e2 04                 	shlq	$0x4, %rdx
1409c5db6: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1409c5dbc: 48 8b 8d 98 0d 00 00        	movq	0xd98(%rbp), %rcx
1409c5dc3: e8 58 de c4 ff              	callq	0x140613c20 <.text+0x612c20>
1409c5dc8: c6 85 b7 0d 00 00 00        	movb	$0x0, 0xdb7(%rbp)
1409c5dcf: c6 85 b6 0d 00 00 01        	movb	$0x1, 0xdb6(%rbp)
1409c5dd6: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1409c5ddb: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1409c5de0: 48 83 c4 68                 	addq	$0x68, %rsp
1409c5de4: 5b                          	popq	%rbx
1409c5de5: 5f                          	popq	%rdi
1409c5de6: 5e                          	popq	%rsi
1409c5de7: 41 5c                       	popq	%r12
1409c5de9: 41 5d                       	popq	%r13
1409c5deb: 41 5e                       	popq	%r14
1409c5ded: 41 5f                       	popq	%r15
1409c5def: 5d                          	popq	%rbp
1409c5df0: c3                          	retq
1409c5df1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1409c5e00: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409c5e05: 55                          	pushq	%rbp
1409c5e06: 41 57                       	pushq	%r15
1409c5e08: 41 56                       	pushq	%r14
1409c5e0a: 41 55                       	pushq	%r13
1409c5e0c: 41 54                       	pushq	%r12
1409c5e0e: 56                          	pushq	%rsi
1409c5e0f: 57                          	pushq	%rdi
1409c5e10: 53                          	pushq	%rbx
1409c5e11: 48 83 ec 68                 	subq	$0x68, %rsp
1409c5e15: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409c5e1c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1409c5e21: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
1409c5e26: 48 8b 85 a8 0d 00 00        	movq	0xda8(%rbp), %rax
1409c5e2d: 48 ff 00                    	incq	(%rax)
1409c5e30: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1409c5e35: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1409c5e3a: 48 83 c4 68                 	addq	$0x68, %rsp
1409c5e3e: 5b                          	popq	%rbx
1409c5e3f: 5f                          	popq	%rdi
1409c5e40: 5e                          	popq	%rsi
1409c5e41: 41 5c                       	popq	%r12
1409c5e43: 41 5d                       	popq	%r13
1409c5e45: 41 5e                       	popq	%r14
1409c5e47: 41 5f                       	popq	%r15
1409c5e49: 5d                          	popq	%rbp
1409c5e4a: c3                          	retq
1409c5e4b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
1409c5e50: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409c5e55: 55                          	pushq	%rbp
1409c5e56: 41 57                       	pushq	%r15
1409c5e58: 41 56                       	pushq	%r14
1409c5e5a: 41 55                       	pushq	%r13
1409c5e5c: 41 54                       	pushq	%r12
1409c5e5e: 56                          	pushq	%rsi
1409c5e5f: 57                          	pushq	%rdi
1409c5e60: 53                          	pushq	%rbx
1409c5e61: 48 83 ec 68                 	subq	$0x68, %rsp
1409c5e65: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409c5e6c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1409c5e71: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
1409c5e76: 80 bd b4 0d 00 00 00        	cmpb	$0x0, 0xdb4(%rbp)
1409c5e7d: 74 27                       	je	0x1409c5ea6 <.text+0x9c4ea6>
1409c5e7f: 48 8d 8d 98 07 00 00        	leaq	0x798(%rbp), %rcx
1409c5e86: e8 35 d7 ec ff              	callq	0x1408935c0 <.text+0x8925c0>
1409c5e8b: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1409c5e90: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1409c5e95: 48 83 c4 68                 	addq	$0x68, %rsp
1409c5e99: 5b                          	popq	%rbx
1409c5e9a: 5f                          	popq	%rdi
1409c5e9b: 5e                          	popq	%rsi
1409c5e9c: 41 5c                       	popq	%r12
1409c5e9e: 41 5d                       	popq	%r13
1409c5ea0: 41 5e                       	popq	%r14
1409c5ea2: 41 5f                       	popq	%r15
1409c5ea4: 5d                          	popq	%rbp
1409c5ea5: c3                          	retq
1409c5ea6: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1409c5eab: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1409c5eb0: 48 83 c4 68                 	addq	$0x68, %rsp
1409c5eb4: 5b                          	popq	%rbx
1409c5eb5: 5f                          	popq	%rdi
1409c5eb6: 5e                          	popq	%rsi
1409c5eb7: 41 5c                       	popq	%r12
1409c5eb9: 41 5d                       	popq	%r13
1409c5ebb: 41 5e                       	popq	%r14
1409c5ebd: 41 5f                       	popq	%r15
1409c5ebf: 5d                          	popq	%rbp
1409c5ec0: c3                          	retq
1409c5ec1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1409c5ed0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409c5ed5: 55                          	pushq	%rbp
1409c5ed6: 41 57                       	pushq	%r15
1409c5ed8: 41 56                       	pushq	%r14
1409c5eda: 41 55                       	pushq	%r13
1409c5edc: 41 54                       	pushq	%r12
1409c5ede: 56                          	pushq	%rsi
1409c5edf: 57                          	pushq	%rdi
1409c5ee0: 53                          	pushq	%rbx
1409c5ee1: 48 83 ec 68                 	subq	$0x68, %rsp
1409c5ee5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409c5eec: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1409c5ef1: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
1409c5ef6: 0f b6 9d b6 0d 00 00        	movzbl	0xdb6(%rbp), %ebx
1409c5efd: 80 bd b7 0d 00 00 00        	cmpb	$0x0, 0xdb7(%rbp)
1409c5f04: 74 46                       	je	0x1409c5f4c <.text+0x9c4f4c>
1409c5f06: 48 8b 95 78 0c 00 00        	movq	0xc78(%rbp), %rdx
1409c5f0d: 48 85 d2                    	testq	%rdx, %rdx
1409c5f10: 74 16                       	je	0x1409c5f28 <.text+0x9c4f28>
1409c5f12: 48 8b 8d 80 0c 00 00        	movq	0xc80(%rbp), %rcx
1409c5f19: 48 c1 e2 04                 	shlq	$0x4, %rdx
1409c5f1d: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1409c5f23: e8 f8 dc c4 ff              	callq	0x140613c20 <.text+0x612c20>
1409c5f28: 80 e3 01                    	andb	$0x1, %bl
1409c5f2b: 88 9d b2 0d 00 00           	movb	%bl, 0xdb2(%rbp)
1409c5f31: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1409c5f36: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1409c5f3b: 48 83 c4 68                 	addq	$0x68, %rsp
1409c5f3f: 5b                          	popq	%rbx
1409c5f40: 5f                          	popq	%rdi
1409c5f41: 5e                          	popq	%rsi
1409c5f42: 41 5c                       	popq	%r12
1409c5f44: 41 5d                       	popq	%r13
1409c5f46: 41 5e                       	popq	%r14
1409c5f48: 41 5f                       	popq	%r15
1409c5f4a: 5d                          	popq	%rbp
1409c5f4b: c3                          	retq
1409c5f4c: 80 e3 01                    	andb	$0x1, %bl
1409c5f4f: 88 9d b2 0d 00 00           	movb	%bl, 0xdb2(%rbp)
1409c5f55: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1409c5f5a: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1409c5f5f: 48 83 c4 68                 	addq	$0x68, %rsp
1409c5f63: 5b                          	popq	%rbx
1409c5f64: 5f                          	popq	%rdi
1409c5f65: 5e                          	popq	%rsi
1409c5f66: 41 5c                       	popq	%r12
1409c5f68: 41 5d                       	popq	%r13
1409c5f6a: 41 5e                       	popq	%r14
1409c5f6c: 41 5f                       	popq	%r15
1409c5f6e: 5d                          	popq	%rbp
1409c5f6f: c3                          	retq
1409c5f70: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409c5f75: 55                          	pushq	%rbp
1409c5f76: 41 57                       	pushq	%r15
1409c5f78: 41 56                       	pushq	%r14
1409c5f7a: 41 55                       	pushq	%r13
1409c5f7c: 41 54                       	pushq	%r12
1409c5f7e: 56                          	pushq	%rsi
1409c5f7f: 57                          	pushq	%rdi
1409c5f80: 53                          	pushq	%rbx
1409c5f81: 48 83 ec 68                 	subq	$0x68, %rsp
1409c5f85: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409c5f8c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1409c5f91: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
1409c5f96: 80 bd b3 0d 00 00 00        	cmpb	$0x0, 0xdb3(%rbp)
1409c5f9d: 74 27                       	je	0x1409c5fc6 <.text+0x9c4fc6>
1409c5f9f: 48 8d 8d 88 09 00 00        	leaq	0x988(%rbp), %rcx
1409c5fa6: e8 15 d6 ec ff              	callq	0x1408935c0 <.text+0x8925c0>
1409c5fab: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1409c5fb0: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1409c5fb5: 48 83 c4 68                 	addq	$0x68, %rsp
1409c5fb9: 5b                          	popq	%rbx
1409c5fba: 5f                          	popq	%rdi
1409c5fbb: 5e                          	popq	%rsi
1409c5fbc: 41 5c                       	popq	%r12
1409c5fbe: 41 5d                       	popq	%r13
1409c5fc0: 41 5e                       	popq	%r14
1409c5fc2: 41 5f                       	popq	%r15
1409c5fc4: 5d                          	popq	%rbp
1409c5fc5: c3                          	retq
1409c5fc6: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1409c5fcb: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1409c5fd0: 48 83 c4 68                 	addq	$0x68, %rsp
1409c5fd4: 5b                          	popq	%rbx
1409c5fd5: 5f                          	popq	%rdi
1409c5fd6: 5e                          	popq	%rsi
1409c5fd7: 41 5c                       	popq	%r12
1409c5fd9: 41 5d                       	popq	%r13
1409c5fdb: 41 5e                       	popq	%r14
1409c5fdd: 41 5f                       	popq	%r15
1409c5fdf: 5d                          	popq	%rbp
1409c5fe0: c3                          	retq
1409c5fe1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1409c5ff0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409c5ff5: 55                          	pushq	%rbp
1409c5ff6: 41 57                       	pushq	%r15
1409c5ff8: 41 56                       	pushq	%r14
1409c5ffa: 41 55                       	pushq	%r13
1409c5ffc: 41 54                       	pushq	%r12
1409c5ffe: 56                          	pushq	%rsi
1409c5fff: 57                          	pushq	%rdi
1409c6000: 53                          	pushq	%rbx
1409c6001: 48 83 ec 68                 	subq	$0x68, %rsp
1409c6005: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409c600c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1409c6011: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
1409c6016: 80 bd b1 0d 00 00 00        	cmpb	$0x0, 0xdb1(%rbp)
1409c601d: 74 27                       	je	0x1409c6046 <.text+0x9c5046>
1409c601f: 48 8d 8d 88 09 00 00        	leaq	0x988(%rbp), %rcx
1409c6026: e8 95 d5 ec ff              	callq	0x1408935c0 <.text+0x8925c0>
1409c602b: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1409c6030: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1409c6035: 48 83 c4 68                 	addq	$0x68, %rsp
1409c6039: 5b                          	popq	%rbx
1409c603a: 5f                          	popq	%rdi
1409c603b: 5e                          	popq	%rsi
1409c603c: 41 5c                       	popq	%r12
1409c603e: 41 5d                       	popq	%r13
1409c6040: 41 5e                       	popq	%r14
1409c6042: 41 5f                       	popq	%r15
1409c6044: 5d                          	popq	%rbp
1409c6045: c3                          	retq
1409c6046: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1409c604b: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1409c6050: 48 83 c4 68                 	addq	$0x68, %rsp
1409c6054: 5b                          	popq	%rbx
1409c6055: 5f                          	popq	%rdi
1409c6056: 5e                          	popq	%rsi
1409c6057: 41 5c                       	popq	%r12
1409c6059: 41 5d                       	popq	%r13
1409c605b: 41 5e                       	popq	%r14
1409c605d: 41 5f                       	popq	%r15
1409c605f: 5d                          	popq	%rbp
1409c6060: c3                          	retq
1409c6061: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1409c6070: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409c6075: 55                          	pushq	%rbp
1409c6076: 41 57                       	pushq	%r15
1409c6078: 41 56                       	pushq	%r14
1409c607a: 41 55                       	pushq	%r13
1409c607c: 41 54                       	pushq	%r12
1409c607e: 56                          	pushq	%rsi
1409c607f: 57                          	pushq	%rdi
1409c6080: 53                          	pushq	%rbx
1409c6081: 48 83 ec 68                 	subq	$0x68, %rsp
1409c6085: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409c608c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1409c6091: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
1409c6096: 80 bd b2 0d 00 00 00        	cmpb	$0x0, 0xdb2(%rbp)
1409c609d: 74 2b                       	je	0x1409c60ca <.text+0x9c50ca>
1409c609f: 48 8d 4d c8                 	leaq	-0x38(%rbp), %rcx
1409c60a3: e8 18 c3 ec ff              	callq	0x1408923c0 <.text+0x8913c0>
1409c60a8: c6 85 b5 0d 00 00 00        	movb	$0x0, 0xdb5(%rbp)
1409c60af: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1409c60b4: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1409c60b9: 48 83 c4 68                 	addq	$0x68, %rsp
1409c60bd: 5b                          	popq	%rbx
1409c60be: 5f                          	popq	%rdi
1409c60bf: 5e                          	popq	%rsi
1409c60c0: 41 5c                       	popq	%r12
1409c60c2: 41 5d                       	popq	%r13
1409c60c4: 41 5e                       	popq	%r14
1409c60c6: 41 5f                       	popq	%r15
1409c60c8: 5d                          	popq	%rbp
1409c60c9: c3                          	retq
1409c60ca: c6 85 b5 0d 00 00 00        	movb	$0x0, 0xdb5(%rbp)
1409c60d1: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1409c60d6: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1409c60db: 48 83 c4 68                 	addq	$0x68, %rsp
1409c60df: 5b                          	popq	%rbx
1409c60e0: 5f                          	popq	%rdi
1409c60e1: 5e                          	popq	%rsi
1409c60e2: 41 5c                       	popq	%r12
1409c60e4: 41 5d                       	popq	%r13
1409c60e6: 41 5e                       	popq	%r14
1409c60e8: 41 5f                       	popq	%r15
1409c60ea: 5d                          	popq	%rbp
1409c60eb: c3                          	retq
1409c60ec: 0f 1f 40 00                 	nopl	(%rax)
1409c60f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409c60f5: 55                          	pushq	%rbp
1409c60f6: 41 57                       	pushq	%r15
1409c60f8: 41 56                       	pushq	%r14
1409c60fa: 41 55                       	pushq	%r13
1409c60fc: 41 54                       	pushq	%r12
1409c60fe: 56                          	pushq	%rsi
1409c60ff: 57                          	pushq	%rdi
1409c6100: 53                          	pushq	%rbx
1409c6101: 48 83 ec 68                 	subq	$0x68, %rsp
1409c6105: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409c610c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1409c6111: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
1409c6116: 80 bd b5 0d 00 00 00        	cmpb	$0x0, 0xdb5(%rbp)
1409c611d: 74 41                       	je	0x1409c6160 <.text+0x9c5160>
1409c611f: 48 8b 85 40 0d 00 00        	movq	0xd40(%rbp), %rax
1409c6126: 48 85 c0                    	testq	%rax, %rax
1409c6129: 74 1a                       	je	0x1409c6145 <.text+0x9c5145>
1409c612b: 48 8b 8d 48 0d 00 00        	movq	0xd48(%rbp), %rcx
1409c6132: 48 c1 e0 04                 	shlq	$0x4, %rax
1409c6136: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
1409c613a: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1409c6140: e8 db da c4 ff              	callq	0x140613c20 <.text+0x612c20>
1409c6145: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1409c614a: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1409c614f: 48 83 c4 68                 	addq	$0x68, %rsp
1409c6153: 5b                          	popq	%rbx
1409c6154: 5f                          	popq	%rdi
1409c6155: 5e                          	popq	%rsi
1409c6156: 41 5c                       	popq	%r12
1409c6158: 41 5d                       	popq	%r13
1409c615a: 41 5e                       	popq	%r14
1409c615c: 41 5f                       	popq	%r15
1409c615e: 5d                          	popq	%rbp
1409c615f: c3                          	retq
1409c6160: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1409c6165: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1409c616a: 48 83 c4 68                 	addq	$0x68, %rsp
1409c616e: 5b                          	popq	%rbx
1409c616f: 5f                          	popq	%rdi
1409c6170: 5e                          	popq	%rsi
1409c6171: 41 5c                       	popq	%r12
1409c6173: 41 5d                       	popq	%r13
1409c6175: 41 5e                       	popq	%r14
1409c6177: 41 5f                       	popq	%r15
1409c6179: 5d                          	popq	%rbp
1409c617a: c3                          	retq
1409c617b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
1409c6180: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409c6185: 55                          	pushq	%rbp
1409c6186: 41 57                       	pushq	%r15
1409c6188: 41 56                       	pushq	%r14
1409c618a: 41 55                       	pushq	%r13
1409c618c: 41 54                       	pushq	%r12
1409c618e: 56                          	pushq	%rsi
1409c618f: 57                          	pushq	%rdi
1409c6190: 53                          	pushq	%rbx
1409c6191: 48 83 ec 68                 	subq	$0x68, %rsp
1409c6195: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409c619c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1409c61a1: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
1409c61a6: 48 8d 8d b0 03 00 00        	leaq	0x3b0(%rbp), %rcx
1409c61ad: e8 0e d4 ec ff              	callq	0x1408935c0 <.text+0x8925c0>
1409c61b2: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1409c61b7: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1409c61bc: 48 83 c4 68                 	addq	$0x68, %rsp
1409c61c0: 5b                          	popq	%rbx
1409c61c1: 5f                          	popq	%rdi
1409c61c2: 5e                          	popq	%rsi
1409c61c3: 41 5c                       	popq	%r12
1409c61c5: 41 5d                       	popq	%r13
1409c61c7: 41 5e                       	popq	%r14
1409c61c9: 41 5f                       	popq	%r15
1409c61cb: 5d                          	popq	%rbp
1409c61cc: c3                          	retq
1409c61cd: 0f 1f 00                    	nopl	(%rax)
1409c61d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409c61d5: 55                          	pushq	%rbp
1409c61d6: 41 57                       	pushq	%r15
1409c61d8: 41 56                       	pushq	%r14
1409c61da: 41 55                       	pushq	%r13
1409c61dc: 41 54                       	pushq	%r12
1409c61de: 56                          	pushq	%rsi
1409c61df: 57                          	pushq	%rdi
1409c61e0: 53                          	pushq	%rbx
1409c61e1: 48 83 ec 68                 	subq	$0x68, %rsp
1409c61e5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409c61ec: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1409c61f1: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
1409c61f6: 48 8d 8d 80 0b 00 00        	leaq	0xb80(%rbp), %rcx
1409c61fd: e8 be d3 ec ff              	callq	0x1408935c0 <.text+0x8925c0>
1409c6202: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1409c6207: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1409c620c: 48 83 c4 68                 	addq	$0x68, %rsp
1409c6210: 5b                          	popq	%rbx
1409c6211: 5f                          	popq	%rdi
1409c6212: 5e                          	popq	%rsi
1409c6213: 41 5c                       	popq	%r12
1409c6215: 41 5d                       	popq	%r13
1409c6217: 41 5e                       	popq	%r14
1409c6219: 41 5f                       	popq	%r15
1409c621b: 5d                          	popq	%rbp
1409c621c: c3                          	retq
1409c621d: 0f 1f 00                    	nopl	(%rax)
1409c6220: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409c6225: 55                          	pushq	%rbp
1409c6226: 41 57                       	pushq	%r15
1409c6228: 41 56                       	pushq	%r14
1409c622a: 41 55                       	pushq	%r13
1409c622c: 41 54                       	pushq	%r12
1409c622e: 56                          	pushq	%rsi
1409c622f: 57                          	pushq	%rdi
1409c6230: 53                          	pushq	%rbx
1409c6231: 48 83 ec 68                 	subq	$0x68, %rsp
1409c6235: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409c623c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1409c6241: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
1409c6246: 48 8d 8d 80 0a 00 00        	leaq	0xa80(%rbp), %rcx
1409c624d: e8 6e d3 ec ff              	callq	0x1408935c0 <.text+0x8925c0>
1409c6252: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1409c6257: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1409c625c: 48 83 c4 68                 	addq	$0x68, %rsp
1409c6260: 5b                          	popq	%rbx
1409c6261: 5f                          	popq	%rdi
1409c6262: 5e                          	popq	%rsi
1409c6263: 41 5c                       	popq	%r12
1409c6265: 41 5d                       	popq	%r13
1409c6267: 41 5e                       	popq	%r14
1409c6269: 41 5f                       	popq	%r15
1409c626b: 5d                          	popq	%rbp
1409c626c: c3                          	retq
1409c626d: 0f 1f 00                    	nopl	(%rax)
1409c6270: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409c6275: 55                          	pushq	%rbp
1409c6276: 41 57                       	pushq	%r15
1409c6278: 41 56                       	pushq	%r14
1409c627a: 41 55                       	pushq	%r13
1409c627c: 41 54                       	pushq	%r12
1409c627e: 56                          	pushq	%rsi
1409c627f: 57                          	pushq	%rdi
1409c6280: 53                          	pushq	%rbx
1409c6281: 48 83 ec 68                 	subq	$0x68, %rsp
1409c6285: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409c628c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1409c6291: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
1409c6296: 48 8d 8d 90 08 00 00        	leaq	0x890(%rbp), %rcx
1409c629d: e8 1e d3 ec ff              	callq	0x1408935c0 <.text+0x8925c0>
1409c62a2: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1409c62a7: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1409c62ac: 48 83 c4 68                 	addq	$0x68, %rsp
1409c62b0: 5b                          	popq	%rbx
1409c62b1: 5f                          	popq	%rdi
1409c62b2: 5e                          	popq	%rsi
1409c62b3: 41 5c                       	popq	%r12
1409c62b5: 41 5d                       	popq	%r13
1409c62b7: 41 5e                       	popq	%r14
1409c62b9: 41 5f                       	popq	%r15
1409c62bb: 5d                          	popq	%rbp
1409c62bc: c3                          	retq
1409c62bd: 0f 1f 00                    	nopl	(%rax)
1409c62c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409c62c5: 55                          	pushq	%rbp
1409c62c6: 41 57                       	pushq	%r15
1409c62c8: 41 56                       	pushq	%r14
1409c62ca: 41 55                       	pushq	%r13
1409c62cc: 41 54                       	pushq	%r12
1409c62ce: 56                          	pushq	%rsi
1409c62cf: 57                          	pushq	%rdi
1409c62d0: 53                          	pushq	%rbx
1409c62d1: 48 83 ec 68                 	subq	$0x68, %rsp
1409c62d5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409c62dc: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1409c62e1: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
1409c62e6: c6 85 b4 0d 00 00 00        	movb	$0x0, 0xdb4(%rbp)
1409c62ed: 48 8d 8d 98 07 00 00        	leaq	0x798(%rbp), %rcx
1409c62f4: e8 c7 d2 ec ff              	callq	0x1408935c0 <.text+0x8925c0>
1409c62f9: c6 85 b4 0d 00 00 00        	movb	$0x0, 0xdb4(%rbp)
1409c6300: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1409c6305: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1409c630a: 48 83 c4 68                 	addq	$0x68, %rsp
1409c630e: 5b                          	popq	%rbx
1409c630f: 5f                          	popq	%rdi
1409c6310: 5e                          	popq	%rsi
1409c6311: 41 5c                       	popq	%r12
1409c6313: 41 5d                       	popq	%r13
1409c6315: 41 5e                       	popq	%r14
1409c6317: 41 5f                       	popq	%r15
1409c6319: 5d                          	popq	%rbp
1409c631a: c3                          	retq
1409c631b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
1409c6320: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409c6325: 55                          	pushq	%rbp
1409c6326: 41 57                       	pushq	%r15
1409c6328: 41 56                       	pushq	%r14
1409c632a: 41 55                       	pushq	%r13
1409c632c: 41 54                       	pushq	%r12
1409c632e: 56                          	pushq	%rsi
1409c632f: 57                          	pushq	%rdi
1409c6330: 53                          	pushq	%rbx
1409c6331: 48 83 ec 68                 	subq	$0x68, %rsp
1409c6335: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409c633c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1409c6341: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
1409c6346: 48 8d 8d b0 03 00 00        	leaq	0x3b0(%rbp), %rcx
1409c634d: e8 6e d2 ec ff              	callq	0x1408935c0 <.text+0x8925c0>
1409c6352: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1409c6357: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1409c635c: 48 83 c4 68                 	addq	$0x68, %rsp
1409c6360: 5b                          	popq	%rbx
1409c6361: 5f                          	popq	%rdi
1409c6362: 5e                          	popq	%rsi
1409c6363: 41 5c                       	popq	%r12
1409c6365: 41 5d                       	popq	%r13
1409c6367: 41 5e                       	popq	%r14
1409c6369: 41 5f                       	popq	%r15
1409c636b: 5d                          	popq	%rbp
1409c636c: c3                          	retq
1409c636d: 0f 1f 00                    	nopl	(%rax)
1409c6370: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409c6375: 55                          	pushq	%rbp
1409c6376: 41 57                       	pushq	%r15
1409c6378: 41 56                       	pushq	%r14
1409c637a: 41 55                       	pushq	%r13
1409c637c: 41 54                       	pushq	%r12
1409c637e: 56                          	pushq	%rsi
1409c637f: 57                          	pushq	%rdi
1409c6380: 53                          	pushq	%rbx
1409c6381: 48 83 ec 68                 	subq	$0x68, %rsp
1409c6385: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409c638c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1409c6391: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
1409c6396: 48 8d 8d 80 0b 00 00        	leaq	0xb80(%rbp), %rcx
1409c639d: e8 1e d2 ec ff              	callq	0x1408935c0 <.text+0x8925c0>
1409c63a2: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1409c63a7: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1409c63ac: 48 83 c4 68                 	addq	$0x68, %rsp
1409c63b0: 5b                          	popq	%rbx
1409c63b1: 5f                          	popq	%rdi
1409c63b2: 5e                          	popq	%rsi
1409c63b3: 41 5c                       	popq	%r12
1409c63b5: 41 5d                       	popq	%r13
1409c63b7: 41 5e                       	popq	%r14
1409c63b9: 41 5f                       	popq	%r15
1409c63bb: 5d                          	popq	%rbp
1409c63bc: c3                          	retq
1409c63bd: 0f 1f 00                    	nopl	(%rax)
1409c63c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409c63c5: 55                          	pushq	%rbp
1409c63c6: 41 57                       	pushq	%r15
1409c63c8: 41 56                       	pushq	%r14
1409c63ca: 41 55                       	pushq	%r13
1409c63cc: 41 54                       	pushq	%r12
1409c63ce: 56                          	pushq	%rsi
1409c63cf: 57                          	pushq	%rdi
1409c63d0: 53                          	pushq	%rbx
1409c63d1: 48 83 ec 68                 	subq	$0x68, %rsp
1409c63d5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409c63dc: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1409c63e1: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
1409c63e6: 48 8d 8d 80 0a 00 00        	leaq	0xa80(%rbp), %rcx
1409c63ed: e8 ce d1 ec ff              	callq	0x1408935c0 <.text+0x8925c0>
1409c63f2: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1409c63f7: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1409c63fc: 48 83 c4 68                 	addq	$0x68, %rsp
1409c6400: 5b                          	popq	%rbx
1409c6401: 5f                          	popq	%rdi
1409c6402: 5e                          	popq	%rsi
1409c6403: 41 5c                       	popq	%r12
1409c6405: 41 5d                       	popq	%r13
1409c6407: 41 5e                       	popq	%r14
1409c6409: 41 5f                       	popq	%r15
1409c640b: 5d                          	popq	%rbp
1409c640c: c3                          	retq
1409c640d: 0f 1f 00                    	nopl	(%rax)
1409c6410: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409c6415: 55                          	pushq	%rbp
1409c6416: 41 57                       	pushq	%r15
1409c6418: 41 56                       	pushq	%r14
1409c641a: 41 55                       	pushq	%r13
1409c641c: 41 54                       	pushq	%r12
1409c641e: 56                          	pushq	%rsi
1409c641f: 57                          	pushq	%rdi
1409c6420: 53                          	pushq	%rbx
1409c6421: 48 83 ec 68                 	subq	$0x68, %rsp
1409c6425: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409c642c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1409c6431: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
1409c6436: 48 8d 8d 90 08 00 00        	leaq	0x890(%rbp), %rcx
1409c643d: e8 7e d1 ec ff              	callq	0x1408935c0 <.text+0x8925c0>
1409c6442: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1409c6447: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1409c644c: 48 83 c4 68                 	addq	$0x68, %rsp
1409c6450: 5b                          	popq	%rbx
1409c6451: 5f                          	popq	%rdi
1409c6452: 5e                          	popq	%rsi
1409c6453: 41 5c                       	popq	%r12
1409c6455: 41 5d                       	popq	%r13
1409c6457: 41 5e                       	popq	%r14
1409c6459: 41 5f                       	popq	%r15
1409c645b: 5d                          	popq	%rbp
1409c645c: c3                          	retq
1409c645d: 0f 1f 00                    	nopl	(%rax)
1409c6460: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409c6465: 55                          	pushq	%rbp
1409c6466: 41 57                       	pushq	%r15
1409c6468: 41 56                       	pushq	%r14
1409c646a: 41 55                       	pushq	%r13
1409c646c: 41 54                       	pushq	%r12
1409c646e: 56                          	pushq	%rsi
1409c646f: 57                          	pushq	%rdi
1409c6470: 53                          	pushq	%rbx
1409c6471: 48 83 ec 68                 	subq	$0x68, %rsp
1409c6475: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409c647c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1409c6481: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
1409c6486: 48 8d 8d 98 07 00 00        	leaq	0x798(%rbp), %rcx
1409c648d: e8 2e d1 ec ff              	callq	0x1408935c0 <.text+0x8925c0>
1409c6492: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1409c6497: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1409c649c: 48 83 c4 68                 	addq	$0x68, %rsp
1409c64a0: 5b                          	popq	%rbx
1409c64a1: 5f                          	popq	%rdi
1409c64a2: 5e                          	popq	%rsi
1409c64a3: 41 5c                       	popq	%r12
1409c64a5: 41 5d                       	popq	%r13
1409c64a7: 41 5e                       	popq	%r14
1409c64a9: 41 5f                       	popq	%r15
1409c64ab: 5d                          	popq	%rbp
1409c64ac: c3                          	retq
1409c64ad: 0f 1f 00                    	nopl	(%rax)
1409c64b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409c64b5: 55                          	pushq	%rbp
1409c64b6: 41 57                       	pushq	%r15
1409c64b8: 41 56                       	pushq	%r14
1409c64ba: 41 55                       	pushq	%r13
1409c64bc: 41 54                       	pushq	%r12
1409c64be: 56                          	pushq	%rsi
1409c64bf: 57                          	pushq	%rdi
1409c64c0: 53                          	pushq	%rbx
1409c64c1: 48 83 ec 68                 	subq	$0x68, %rsp
1409c64c5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409c64cc: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1409c64d1: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
1409c64d6: c6 85 b3 0d 00 00 00        	movb	$0x0, 0xdb3(%rbp)
1409c64dd: 48 8d 8d 88 09 00 00        	leaq	0x988(%rbp), %rcx
1409c64e4: e8 d7 d0 ec ff              	callq	0x1408935c0 <.text+0x8925c0>
1409c64e9: c6 85 b3 0d 00 00 00        	movb	$0x0, 0xdb3(%rbp)
1409c64f0: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1409c64f5: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1409c64fa: 48 83 c4 68                 	addq	$0x68, %rsp
1409c64fe: 5b                          	popq	%rbx
1409c64ff: 5f                          	popq	%rdi
1409c6500: 5e                          	popq	%rsi
1409c6501: 41 5c                       	popq	%r12
1409c6503: 41 5d                       	popq	%r13
1409c6505: 41 5e                       	popq	%r14
1409c6507: 41 5f                       	popq	%r15
1409c6509: 5d                          	popq	%rbp
1409c650a: c3                          	retq
1409c650b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
1409c6510: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409c6515: 55                          	pushq	%rbp
1409c6516: 41 57                       	pushq	%r15
1409c6518: 41 56                       	pushq	%r14
1409c651a: 41 55                       	pushq	%r13
1409c651c: 41 54                       	pushq	%r12
1409c651e: 56                          	pushq	%rsi
1409c651f: 57                          	pushq	%rdi
1409c6520: 53                          	pushq	%rbx
1409c6521: 48 83 ec 68                 	subq	$0x68, %rsp
1409c6525: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409c652c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1409c6531: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
1409c6536: 48 8d 8d b0 03 00 00        	leaq	0x3b0(%rbp), %rcx
1409c653d: e8 7e d0 ec ff              	callq	0x1408935c0 <.text+0x8925c0>
1409c6542: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1409c6547: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1409c654c: 48 83 c4 68                 	addq	$0x68, %rsp
1409c6550: 5b                          	popq	%rbx
1409c6551: 5f                          	popq	%rdi
1409c6552: 5e                          	popq	%rsi
1409c6553: 41 5c                       	popq	%r12
1409c6555: 41 5d                       	popq	%r13
1409c6557: 41 5e                       	popq	%r14
1409c6559: 41 5f                       	popq	%r15
1409c655b: 5d                          	popq	%rbp
1409c655c: c3                          	retq
1409c655d: 0f 1f 00                    	nopl	(%rax)
1409c6560: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409c6565: 55                          	pushq	%rbp
1409c6566: 41 57                       	pushq	%r15
1409c6568: 41 56                       	pushq	%r14
1409c656a: 41 55                       	pushq	%r13
1409c656c: 41 54                       	pushq	%r12
1409c656e: 56                          	pushq	%rsi
1409c656f: 57                          	pushq	%rdi
1409c6570: 53                          	pushq	%rbx
1409c6571: 48 83 ec 68                 	subq	$0x68, %rsp
1409c6575: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409c657c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1409c6581: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
1409c6586: 48 8d 8d 80 0b 00 00        	leaq	0xb80(%rbp), %rcx
1409c658d: e8 2e d0 ec ff              	callq	0x1408935c0 <.text+0x8925c0>
1409c6592: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1409c6597: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1409c659c: 48 83 c4 68                 	addq	$0x68, %rsp
1409c65a0: 5b                          	popq	%rbx
1409c65a1: 5f                          	popq	%rdi
1409c65a2: 5e                          	popq	%rsi
1409c65a3: 41 5c                       	popq	%r12
1409c65a5: 41 5d                       	popq	%r13
1409c65a7: 41 5e                       	popq	%r14
1409c65a9: 41 5f                       	popq	%r15
1409c65ab: 5d                          	popq	%rbp
1409c65ac: c3                          	retq
1409c65ad: 0f 1f 00                    	nopl	(%rax)
1409c65b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409c65b5: 55                          	pushq	%rbp
1409c65b6: 41 57                       	pushq	%r15
1409c65b8: 41 56                       	pushq	%r14
1409c65ba: 41 55                       	pushq	%r13
1409c65bc: 41 54                       	pushq	%r12
1409c65be: 56                          	pushq	%rsi
1409c65bf: 57                          	pushq	%rdi
1409c65c0: 53                          	pushq	%rbx
1409c65c1: 48 83 ec 68                 	subq	$0x68, %rsp
1409c65c5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409c65cc: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1409c65d1: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
1409c65d6: 48 8d 8d 80 0a 00 00        	leaq	0xa80(%rbp), %rcx
1409c65dd: e8 de cf ec ff              	callq	0x1408935c0 <.text+0x8925c0>
1409c65e2: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1409c65e7: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1409c65ec: 48 83 c4 68                 	addq	$0x68, %rsp
1409c65f0: 5b                          	popq	%rbx
1409c65f1: 5f                          	popq	%rdi
1409c65f2: 5e                          	popq	%rsi
1409c65f3: 41 5c                       	popq	%r12
1409c65f5: 41 5d                       	popq	%r13
1409c65f7: 41 5e                       	popq	%r14
1409c65f9: 41 5f                       	popq	%r15
1409c65fb: 5d                          	popq	%rbp
1409c65fc: c3                          	retq
1409c65fd: 0f 1f 00                    	nopl	(%rax)
1409c6600: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409c6605: 55                          	pushq	%rbp
1409c6606: 41 57                       	pushq	%r15
1409c6608: 41 56                       	pushq	%r14
1409c660a: 41 55                       	pushq	%r13
1409c660c: 41 54                       	pushq	%r12
1409c660e: 56                          	pushq	%rsi
1409c660f: 57                          	pushq	%rdi
1409c6610: 53                          	pushq	%rbx
1409c6611: 48 83 ec 68                 	subq	$0x68, %rsp
1409c6615: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409c661c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1409c6621: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
1409c6626: 48 8d 8d 90 08 00 00        	leaq	0x890(%rbp), %rcx
1409c662d: e8 8e cf ec ff              	callq	0x1408935c0 <.text+0x8925c0>
1409c6632: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1409c6637: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1409c663c: 48 83 c4 68                 	addq	$0x68, %rsp
1409c6640: 5b                          	popq	%rbx
1409c6641: 5f                          	popq	%rdi
1409c6642: 5e                          	popq	%rsi
1409c6643: 41 5c                       	popq	%r12
1409c6645: 41 5d                       	popq	%r13
1409c6647: 41 5e                       	popq	%r14
1409c6649: 41 5f                       	popq	%r15
1409c664b: 5d                          	popq	%rbp
1409c664c: c3                          	retq
1409c664d: 0f 1f 00                    	nopl	(%rax)
1409c6650: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409c6655: 55                          	pushq	%rbp
1409c6656: 41 57                       	pushq	%r15
1409c6658: 41 56                       	pushq	%r14
1409c665a: 41 55                       	pushq	%r13
1409c665c: 41 54                       	pushq	%r12
1409c665e: 56                          	pushq	%rsi
1409c665f: 57                          	pushq	%rdi
1409c6660: 53                          	pushq	%rbx
1409c6661: 48 83 ec 68                 	subq	$0x68, %rsp
1409c6665: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409c666c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1409c6671: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
1409c6676: 48 8d 8d 98 07 00 00        	leaq	0x798(%rbp), %rcx
1409c667d: e8 3e cf ec ff              	callq	0x1408935c0 <.text+0x8925c0>
1409c6682: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1409c6687: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1409c668c: 48 83 c4 68                 	addq	$0x68, %rsp
1409c6690: 5b                          	popq	%rbx
1409c6691: 5f                          	popq	%rdi
1409c6692: 5e                          	popq	%rsi
1409c6693: 41 5c                       	popq	%r12
1409c6695: 41 5d                       	popq	%r13
1409c6697: 41 5e                       	popq	%r14
1409c6699: 41 5f                       	popq	%r15
1409c669b: 5d                          	popq	%rbp
1409c669c: c3                          	retq
1409c669d: 0f 1f 00                    	nopl	(%rax)
