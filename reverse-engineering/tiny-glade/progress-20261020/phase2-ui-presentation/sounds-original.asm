
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14233a7f0: 55                          	pushq	%rbp
14233a7f1: 41 57                       	pushq	%r15
14233a7f3: 41 56                       	pushq	%r14
14233a7f5: 41 55                       	pushq	%r13
14233a7f7: 41 54                       	pushq	%r12
14233a7f9: 56                          	pushq	%rsi
14233a7fa: 57                          	pushq	%rdi
14233a7fb: 53                          	pushq	%rbx
14233a7fc: 48 81 ec f8 00 00 00        	subq	$0xf8, %rsp
14233a803: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14233a80b: 48 c7 45 70 fe ff ff ff     	movq	$-0x2, 0x70(%rbp)
14233a813: 4d 89 cc                    	movq	%r9, %r12
14233a816: 4d 89 c7                    	movq	%r8, %r15
14233a819: 48 89 d3                    	movq	%rdx, %rbx
14233a81c: 48 89 ce                    	movq	%rcx, %rsi
14233a81f: e8 dc c1 8d fe              	callq	0x140c16a00 <.text+0xc15a00>
14233a824: 84 c0                       	testb	%al, %al
14233a826: 88 45 6f                    	movb	%al, 0x6f(%rbp)
14233a829: 4c 89 7d f8                 	movq	%r15, -0x8(%rbp)
14233a82d: 48 89 5d 20                 	movq	%rbx, 0x20(%rbp)
14233a831: 74 63                       	je	0x14233a896 <.text+0x2339896>
14233a833: 4c 89 65 60                 	movq	%r12, 0x60(%rbp)
14233a837: 4c 8d 25 b0 32 d1 00        	leaq	0xd132b0(%rip), %r12    # 0x14304daee
14233a83e: bf 22 00 00 00              	movl	$0x22, %edi
14233a843: ba 22 00 00 00              	movl	$0x22, %edx
14233a848: 4c 89 e1                    	movq	%r12, %rcx
14233a84b: e8 e0 d8 ff ff              	callq	0x142338130 <.text+0x2337130>
14233a850: 48 83 f8 01                 	cmpq	$0x1, %rax
14233a854: 0f 85 b1 00 00 00           	jne	0x14233a90b <.text+0x233990b>
14233a85a: 49 89 d1                    	movq	%rdx, %r9
14233a85d: 48 85 d2                    	testq	%rdx, %rdx
14233a860: 74 3d                       	je	0x14233a89f <.text+0x233989f>
14233a862: 49 83 f9 22                 	cmpq	$0x22, %r9
14233a866: 73 35                       	jae	0x14233a89d <.text+0x233989d>
14233a868: 48 8d 05 7f 32 d1 00        	leaq	0xd1327f(%rip), %rax    # 0x14304daee
14233a86f: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
14233a874: 7f 29                       	jg	0x14233a89f <.text+0x233989f>
14233a876: 48 8d 05 4b 33 d1 00        	leaq	0xd1334b(%rip), %rax    # 0x14304dbc8
14233a87d: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14233a882: 48 8d 0d 65 32 d1 00        	leaq	0xd13265(%rip), %rcx    # 0x14304daee
14233a889: ba 22 00 00 00              	movl	$0x22, %edx
14233a88e: 45 31 c0                    	xorl	%r8d, %r8d
14233a891: e8 aa f0 59 00              	callq	0x1428d9940 <.text+0x28d8940>
14233a896: 31 c0                       	xorl	%eax, %eax
14233a898: e9 c3 01 00 00              	jmp	0x14233aa60 <.text+0x2339a60>
14233a89d: 75 d7                       	jne	0x14233a876 <.text+0x2339876>
14233a89f: 4c 8d 25 48 32 d1 00        	leaq	0xd13248(%rip), %r12    # 0x14304daee
14233a8a6: 4c 89 e1                    	movq	%r12, %rcx
14233a8a9: 4c 89 ca                    	movq	%r9, %rdx
14233a8ac: e8 7f d8 ff ff              	callq	0x142338130 <.text+0x2337130>
14233a8b1: 48 83 f8 01                 	cmpq	$0x1, %rax
14233a8b5: 75 54                       	jne	0x14233a90b <.text+0x233990b>
14233a8b7: 49 89 d0                    	movq	%rdx, %r8
14233a8ba: 49 83 c0 02                 	addq	$0x2, %r8
14233a8be: 74 39                       	je	0x14233a8f9 <.text+0x23398f9>
14233a8c0: 49 83 f8 22                 	cmpq	$0x22, %r8
14233a8c4: 73 31                       	jae	0x14233a8f7 <.text+0x23398f7>
14233a8c6: 48 8d 05 21 32 d1 00        	leaq	0xd13221(%rip), %rax    # 0x14304daee
14233a8cd: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
14233a8d2: 7f 25                       	jg	0x14233a8f9 <.text+0x23398f9>
14233a8d4: 48 8d 05 05 33 d1 00        	leaq	0xd13305(%rip), %rax    # 0x14304dbe0
14233a8db: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14233a8e0: 48 8d 0d 07 32 d1 00        	leaq	0xd13207(%rip), %rcx    # 0x14304daee
14233a8e7: ba 22 00 00 00              	movl	$0x22, %edx
14233a8ec: 41 b9 22 00 00 00           	movl	$0x22, %r9d
14233a8f2: e8 49 f0 59 00              	callq	0x1428d9940 <.text+0x28d8940>
14233a8f7: 75 db                       	jne	0x14233a8d4 <.text+0x23398d4>
14233a8f9: bf 20 00 00 00              	movl	$0x20, %edi
14233a8fe: 48 29 d7                    	subq	%rdx, %rdi
14233a901: 4c 8d 25 e6 31 d1 00        	leaq	0xd131e6(%rip), %r12    # 0x14304daee
14233a908: 4d 01 c4                    	addq	%r8, %r12
14233a90b: 4c 8d 05 f2 33 d1 00        	leaq	0xd133f2(%rip), %r8     # 0x14304dd04
14233a912: 4c 8d 2d bf 33 d1 00        	leaq	0xd133bf(%rip), %r13    # 0x14304dcd8
14233a919: 0f 1f 80 00 00 00 00        	nopl	(%rax)
14233a920: 41 0f be 40 ff              	movsbl	-0x1(%r8), %eax
14233a925: 85 c0                       	testl	%eax, %eax
14233a927: 78 17                       	js	0x14233a940 <.text+0x2339940>
14233a929: 49 ff c8                    	decq	%r8
14233a92c: 83 f8 5c                    	cmpl	$0x5c, %eax
14233a92f: 75 63                       	jne	0x14233a994 <.text+0x2339994>
14233a931: eb 72                       	jmp	0x14233a9a5 <.text+0x23399a5>
14233a933: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
14233a940: 41 0f b6 48 fe              	movzbl	-0x2(%r8), %ecx
14233a945: 80 f9 c0                    	cmpb	$-0x40, %cl
14233a948: 7d 23                       	jge	0x14233a96d <.text+0x233996d>
14233a94a: 41 0f b6 50 fd              	movzbl	-0x3(%r8), %edx
14233a94f: 80 fa c0                    	cmpb	$-0x40, %dl
14233a952: 7d 22                       	jge	0x14233a976 <.text+0x2339976>
14233a954: 45 0f b6 48 fc              	movzbl	-0x4(%r8), %r9d
14233a959: 49 83 c0 fc                 	addq	$-0x4, %r8
14233a95d: 41 83 e1 07                 	andl	$0x7, %r9d
14233a961: 41 c1 e1 06                 	shll	$0x6, %r9d
14233a965: 83 e2 3f                    	andl	$0x3f, %edx
14233a968: 44 09 ca                    	orl	%r9d, %edx
14233a96b: eb 10                       	jmp	0x14233a97d <.text+0x233997d>
14233a96d: 49 83 c0 fe                 	addq	$-0x2, %r8
14233a971: 83 e1 1f                    	andl	$0x1f, %ecx
14233a974: eb 0f                       	jmp	0x14233a985 <.text+0x2339985>
14233a976: 49 83 c0 fd                 	addq	$-0x3, %r8
14233a97a: 83 e2 0f                    	andl	$0xf, %edx
14233a97d: c1 e2 06                    	shll	$0x6, %edx
14233a980: 83 e1 3f                    	andl	$0x3f, %ecx
14233a983: 09 d1                       	orl	%edx, %ecx
14233a985: c1 e1 06                    	shll	$0x6, %ecx
14233a988: 24 3f                       	andb	$0x3f, %al
14233a98a: 0f b6 c0                    	movzbl	%al, %eax
14233a98d: 09 c8                       	orl	%ecx, %eax
14233a98f: 83 f8 5c                    	cmpl	$0x5c, %eax
14233a992: 74 11                       	je	0x14233a9a5 <.text+0x23399a5>
14233a994: 83 f8 2f                    	cmpl	$0x2f, %eax
14233a997: 74 0c                       	je	0x14233a9a5 <.text+0x23399a5>
14233a999: 4d 39 e8                    	cmpq	%r13, %r8
14233a99c: 75 82                       	jne	0x14233a920 <.text+0x2339920>
14233a99e: bb 2c 00 00 00              	movl	$0x2c, %ebx
14233a9a3: eb 4c                       	jmp	0x14233a9f1 <.text+0x23399f1>
14233a9a5: 4c 8d 2d 2c 33 d1 00        	leaq	0xd1332c(%rip), %r13    # 0x14304dcd8
14233a9ac: 4d 29 e8                    	subq	%r13, %r8
14233a9af: 49 ff c0                    	incq	%r8
14233a9b2: 74 32                       	je	0x14233a9e6 <.text+0x23399e6>
14233a9b4: 49 83 f8 2c                 	cmpq	$0x2c, %r8
14233a9b8: 73 2a                       	jae	0x14233a9e4 <.text+0x23399e4>
14233a9ba: 43 80 3c 28 bf              	cmpb	$-0x41, (%r8,%r13)
14233a9bf: 7f 25                       	jg	0x14233a9e6 <.text+0x23399e6>
14233a9c1: 48 8d 05 e0 31 d1 00        	leaq	0xd131e0(%rip), %rax    # 0x14304dba8
14233a9c8: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14233a9cd: 48 8d 0d 04 33 d1 00        	leaq	0xd13304(%rip), %rcx    # 0x14304dcd8
14233a9d4: ba 2c 00 00 00              	movl	$0x2c, %edx
14233a9d9: 41 b9 2c 00 00 00           	movl	$0x2c, %r9d
14233a9df: e8 5c ef 59 00              	callq	0x1428d9940 <.text+0x28d8940>
14233a9e4: 75 db                       	jne	0x14233a9c1 <.text+0x23399c1>
14233a9e6: bb 2c 00 00 00              	movl	$0x2c, %ebx
14233a9eb: 4c 29 c3                    	subq	%r8, %rbx
14233a9ee: 4d 01 c5                    	addq	%r8, %r13
14233a9f1: e8 3a d7 8d fe              	callq	0x140c18130 <.text+0xc17130>
14233a9f6: 48 89 c1                    	movq	%rax, %rcx
14233a9f9: 48 8b 00                    	movq	(%rax), %rax
14233a9fc: 48 83 f8 01                 	cmpq	$0x1, %rax
14233aa00: 0f 85 bf 08 00 00           	jne	0x14233b2c5 <.text+0x233a2c5>
14233aa06: 48 83 c1 08                 	addq	$0x8, %rcx
14233aa0a: 48 83 39 00                 	cmpq	$0x0, (%rcx)
14233aa0e: 0f 85 eb 08 00 00           	jne	0x14233b2ff <.text+0x233a2ff>
14233aa14: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
14233aa1b: 48 89 4d 58                 	movq	%rcx, 0x58(%rbp)
14233aa1f: 48 83 c1 08                 	addq	$0x8, %rcx
14233aa23: 48 89 5c 24 20              	movq	%rbx, 0x20(%rsp)
14233aa28: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
14233aa31: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
14233aa3a: 4c 89 e2                    	movq	%r12, %rdx
14233aa3d: 49 89 f8                    	movq	%rdi, %r8
14233aa40: 4d 89 e9                    	movq	%r13, %r9
14233aa43: e8 f8 c2 8d fe              	callq	0x140c16d40 <.text+0xc15d40>
14233aa48: 48 8b 4d 58                 	movq	0x58(%rbp), %rcx
14233aa4c: 48 ff 01                    	incq	(%rcx)
14233aa4f: 48 89 45 00                 	movq	%rax, (%rbp)
14233aa53: 48 89 45 c8                 	movq	%rax, -0x38(%rbp)
14233aa57: b8 01 00 00 00              	movl	$0x1, %eax
14233aa5c: 4c 8b 65 60                 	movq	0x60(%rbp), %r12
14233aa60: 48 89 45 c0                 	movq	%rax, -0x40(%rbp)
14233aa64: 4c 8b 0e                    	movq	(%rsi), %r9
14233aa67: 48 8b 46 08                 	movq	0x8(%rsi), %rax
14233aa6b: 49 8b 11                    	movq	(%r9), %rdx
14233aa6e: 31 c9                       	xorl	%ecx, %ecx
14233aa70: 49 89 d0                    	movq	%rdx, %r8
14233aa73: 4c 2b 40 18                 	subq	0x18(%rax), %r8
14233aa77: 4c 0f 42 c1                 	cmovbq	%rcx, %r8
14233aa7b: 48 2b 50 38                 	subq	0x38(%rax), %rdx
14233aa7f: 48 0f 42 d1                 	cmovbq	%rcx, %rdx
14233aa83: 4c 89 c7                    	movq	%r8, %rdi
14233aa86: 48 c1 e7 04                 	shlq	$0x4, %rdi
14233aa8a: 48 03 78 08                 	addq	0x8(%rax), %rdi
14233aa8e: 4c 8b 50 10                 	movq	0x10(%rax), %r10
14233aa92: 4d 29 c2                    	subq	%r8, %r10
14233aa95: 4c 0f 42 d1                 	cmovbq	%rcx, %r10
14233aa99: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14233aa9f: 49 0f 42 f8                 	cmovbq	%r8, %rdi
14233aaa3: 49 89 d7                    	movq	%rdx, %r15
14233aaa6: 49 c1 e7 04                 	shlq	$0x4, %r15
14233aaaa: 4c 03 78 28                 	addq	0x28(%rax), %r15
14233aaae: 4c 8b 58 30                 	movq	0x30(%rax), %r11
14233aab2: 49 29 d3                    	subq	%rdx, %r11
14233aab5: 4c 0f 42 d9                 	cmovbq	%rcx, %r11
14233aab9: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
14233aac0: 4d 0f 42 f8                 	cmovbq	%r8, %r15
14233aac4: 48 8b 40 40                 	movq	0x40(%rax), %rax
14233aac8: 4c 29 d8                    	subq	%r11, %rax
14233aacb: 4c 29 d0                    	subq	%r10, %rax
14233aace: 4d 89 ce                    	movq	%r9, %r14
14233aad1: 49 89 01                    	movq	%rax, (%r9)
14233aad4: 49 c1 e2 04                 	shlq	$0x4, %r10
14233aad8: 49 01 fa                    	addq	%rdi, %r10
14233aadb: 4c 89 55 50                 	movq	%r10, 0x50(%rbp)
14233aadf: 49 c1 e3 04                 	shlq	$0x4, %r11
14233aae3: 4d 01 fb                    	addq	%r15, %r11
14233aae6: 4c 89 5d 40                 	movq	%r11, 0x40(%rbp)
14233aaea: 48 8b 01                    	movq	(%rcx), %rax
14233aaed: 48 89 45 30                 	movq	%rax, 0x30(%rbp)
14233aaf1: 48 8b 41 08                 	movq	0x8(%rcx), %rax
14233aaf5: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
14233aaf9: 41 8b 44 24 1c              	movl	0x1c(%r12), %eax
14233aafe: 89 45 58                    	movl	%eax, 0x58(%rbp)
14233ab01: 4d 8b 2c 24                 	movq	(%r12), %r13
14233ab05: 49 8b 44 24 10              	movq	0x10(%r12), %rax
14233ab0a: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
14233ab0e: 49 8d 45 20                 	leaq	0x20(%r13), %rax
14233ab12: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
14233ab16: eb 27                       	jmp	0x14233ab3f <.text+0x2339b3f>
14233ab18: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
14233ab20: 49 8b 45 28                 	movq	0x28(%r13), %rax
14233ab24: 48 89 d9                    	movq	%rbx, %rcx
14233ab27: 48 c1 e1 05                 	shlq	$0x5, %rcx
14233ab2b: 44 89 24 08                 	movl	%r12d, (%rax,%rcx)
14233ab2f: 48 89 74 08 18              	movq	%rsi, 0x18(%rax,%rcx)
14233ab34: 48 ff c3                    	incq	%rbx
14233ab37: 49 89 5d 30                 	movq	%rbx, 0x30(%r13)
14233ab3b: 49 ff 45 40                 	incq	0x40(%r13)
14233ab3f: 48 85 ff                    	testq	%rdi, %rdi
14233ab42: 74 0c                       	je	0x14233ab50 <.text+0x2339b50>
14233ab44: 48 3b 7d 50                 	cmpq	0x50(%rbp), %rdi
14233ab48: 74 06                       	je	0x14233ab50 <.text+0x2339b50>
14233ab4a: 48 8d 4f 10                 	leaq	0x10(%rdi), %rcx
14233ab4e: eb 1c                       	jmp	0x14233ab6c <.text+0x2339b6c>
14233ab50: 4d 85 ff                    	testq	%r15, %r15
14233ab53: 0f 84 dd 00 00 00           	je	0x14233ac36 <.text+0x2339c36>
14233ab59: 4c 3b 7d 40                 	cmpq	0x40(%rbp), %r15
14233ab5d: 0f 84 d3 00 00 00           	je	0x14233ac36 <.text+0x2339c36>
14233ab63: 4c 89 ff                    	movq	%r15, %rdi
14233ab66: 49 83 c7 10                 	addq	$0x10, %r15
14233ab6a: 31 c9                       	xorl	%ecx, %ecx
14233ab6c: 49 ff 06                    	incq	(%r14)
14233ab6f: 48 8b 47 08                 	movq	0x8(%rdi), %rax
14233ab73: 48 89 cf                    	movq	%rcx, %rdi
14233ab76: 89 c1                       	movl	%eax, %ecx
14233ab78: 48 8b 55 60                 	movq	0x60(%rbp), %rdx
14233ab7c: 48 3b 4a 10                 	cmpq	0x10(%rdx), %rcx
14233ab80: 73 7e                       	jae	0x14233ac00 <.text+0x2339c00>
14233ab82: 48 8b 55 60                 	movq	0x60(%rbp), %rdx
14233ab86: 48 8b 52 08                 	movq	0x8(%rdx), %rdx
14233ab8a: 48 c1 e8 20                 	shrq	$0x20, %rax
14233ab8e: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
14233ab92: 39 04 8a                    	cmpl	%eax, (%rdx,%rcx,4)
14233ab95: 75 69                       	jne	0x14233ac00 <.text+0x2339c00>
14233ab97: 48 8d 04 8a                 	leaq	(%rdx,%rcx,4), %rax
14233ab9b: 8b 40 04                    	movl	0x4(%rax), %eax
14233ab9e: b9 ff ff ff ff              	movl	$0xffffffff, %ecx       # imm = 0xFFFFFFFF
14233aba3: 48 39 c8                    	cmpq	%rcx, %rax
14233aba6: 74 58                       	je	0x14233ac00 <.text+0x2339c00>
14233aba8: 48 8b 4d 30                 	movq	0x30(%rbp), %rcx
14233abac: 48 39 41 38                 	cmpq	%rax, 0x38(%rcx)
14233abb0: 76 4e                       	jbe	0x14233ac00 <.text+0x2339c00>
14233abb2: 48 8b 4d 30                 	movq	0x30(%rbp), %rcx
14233abb6: 48 8b 49 28                 	movq	0x28(%rcx), %rcx
14233abba: 89 c2                       	movl	%eax, %edx
14233abbc: c1 ea 06                    	shrl	$0x6, %edx
14233abbf: 48 8b 0c d1                 	movq	(%rcx,%rdx,8), %rcx
14233abc3: 48 0f a3 c1                 	btq	%rax, %rcx
14233abc7: 73 37                       	jae	0x14233ac00 <.text+0x2339c00>
14233abc9: 8b 45 58                    	movl	0x58(%rbp), %eax
14233abcc: 48 8b 4d 38                 	movq	0x38(%rbp), %rcx
14233abd0: 89 01                       	movl	%eax, (%rcx)
14233abd2: 49 8b 5d 30                 	movq	0x30(%r13), %rbx
14233abd6: 49 8b 75 40                 	movq	0x40(%r13), %rsi
14233abda: 41 bc 0a 00 00 00           	movl	$0xa, %r12d
14233abe0: 49 3b 5d 20                 	cmpq	0x20(%r13), %rbx
14233abe4: 0f 85 36 ff ff ff           	jne	0x14233ab20 <.text+0x2339b20>
14233abea: 48 8b 4d 48                 	movq	0x48(%rbp), %rcx
14233abee: 48 8d 15 7b a6 87 00        	leaq	0x87a67b(%rip), %rdx    # 0x142bb5270
14233abf5: e8 36 80 22 fe              	callq	0x140562c30 <.text+0x561c30>
14233abfa: e9 21 ff ff ff              	jmp	0x14233ab20 <.text+0x2339b20>
14233abff: 90                          	nop
14233ac00: 8b 45 58                    	movl	0x58(%rbp), %eax
14233ac03: 48 8b 4d 38                 	movq	0x38(%rbp), %rcx
14233ac07: 89 01                       	movl	%eax, (%rcx)
14233ac09: 49 8b 5d 30                 	movq	0x30(%r13), %rbx
14233ac0d: 49 8b 75 40                 	movq	0x40(%r13), %rsi
14233ac11: 41 bc 0f 00 00 00           	movl	$0xf, %r12d
14233ac17: 49 3b 5d 20                 	cmpq	0x20(%r13), %rbx
14233ac1b: 0f 85 ff fe ff ff           	jne	0x14233ab20 <.text+0x2339b20>
14233ac21: 48 8b 4d 48                 	movq	0x48(%rbp), %rcx
14233ac25: 48 8d 15 44 a6 87 00        	leaq	0x87a644(%rip), %rdx    # 0x142bb5270
14233ac2c: e8 ff 7f 22 fe              	callq	0x140562c30 <.text+0x561c30>
14233ac31: e9 ea fe ff ff              	jmp	0x14233ab20 <.text+0x2339b20>
14233ac36: 48 8b 45 20                 	movq	0x20(%rbp), %rax
14233ac3a: 4c 8b 18                    	movq	(%rax), %r11
14233ac3d: 48 8b 40 08                 	movq	0x8(%rax), %rax
14233ac41: 4d 8b 0b                    	movq	(%r11), %r9
14233ac44: 31 c9                       	xorl	%ecx, %ecx
14233ac46: 4d 89 c8                    	movq	%r9, %r8
14233ac49: 4c 2b 40 18                 	subq	0x18(%rax), %r8
14233ac4d: 4c 0f 42 c1                 	cmovbq	%rcx, %r8
14233ac51: 4c 2b 48 38                 	subq	0x38(%rax), %r9
14233ac55: 4c 0f 42 c9                 	cmovbq	%rcx, %r9
14233ac59: 4c 89 c2                    	movq	%r8, %rdx
14233ac5c: 48 c1 e2 04                 	shlq	$0x4, %rdx
14233ac60: 48 03 50 08                 	addq	0x8(%rax), %rdx
14233ac64: 48 8b 70 10                 	movq	0x10(%rax), %rsi
14233ac68: 4c 29 c6                    	subq	%r8, %rsi
14233ac6b: 48 0f 42 f1                 	cmovbq	%rcx, %rsi
14233ac6f: 41 ba 08 00 00 00           	movl	$0x8, %r10d
14233ac75: 49 0f 42 d2                 	cmovbq	%r10, %rdx
14233ac79: 4d 89 c8                    	movq	%r9, %r8
14233ac7c: 49 c1 e0 04                 	shlq	$0x4, %r8
14233ac80: 4c 03 40 28                 	addq	0x28(%rax), %r8
14233ac84: 48 8b 78 30                 	movq	0x30(%rax), %rdi
14233ac88: 4c 29 cf                    	subq	%r9, %rdi
14233ac8b: 48 0f 42 f9                 	cmovbq	%rcx, %rdi
14233ac8f: 4d 0f 42 c2                 	cmovbq	%r10, %r8
14233ac93: 48 8b 40 40                 	movq	0x40(%rax), %rax
14233ac97: 48 29 f8                    	subq	%rdi, %rax
14233ac9a: 48 29 f0                    	subq	%rsi, %rax
14233ac9d: 4c 89 5d e0                 	movq	%r11, -0x20(%rbp)
14233aca1: 49 89 03                    	movq	%rax, (%r11)
14233aca4: 48 c1 e6 04                 	shlq	$0x4, %rsi
14233aca8: 48 01 d6                    	addq	%rdx, %rsi
14233acab: 48 89 75 e8                 	movq	%rsi, -0x18(%rbp)
14233acaf: 48 c1 e7 04                 	shlq	$0x4, %rdi
14233acb3: 4c 01 c7                    	addq	%r8, %rdi
14233acb6: 48 89 7d f0                 	movq	%rdi, -0x10(%rbp)
14233acba: 48 8b 8d f8 00 00 00        	movq	0xf8(%rbp), %rcx
14233acc1: 48 8b 01                    	movq	(%rcx), %rax
14233acc4: 48 89 45 28                 	movq	%rax, 0x28(%rbp)
14233acc8: 48 8b 71 08                 	movq	0x8(%rcx), %rsi
14233accc: 48 8b 8d 00 01 00 00        	movq	0x100(%rbp), %rcx
14233acd3: 48 8b 01                    	movq	(%rcx), %rax
14233acd6: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
14233acda: 48 8b 41 08                 	movq	0x8(%rcx), %rax
14233acde: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
14233ace2: 48 8b 8d e8 00 00 00        	movq	0xe8(%rbp), %rcx
14233ace9: 48 8b 01                    	movq	(%rcx), %rax
14233acec: 48 89 45 08                 	movq	%rax, 0x8(%rbp)
14233acf0: 48 8b 41 08                 	movq	0x8(%rcx), %rax
14233acf4: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
14233acf8: 48 8b 8d f0 00 00 00        	movq	0xf0(%rbp), %rcx
14233acff: 48 8b 01                    	movq	(%rcx), %rax
14233ad02: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
14233ad06: 48 8b 41 08                 	movq	0x8(%rcx), %rax
14233ad0a: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
14233ad0e: eb 28                       	jmp	0x14233ad38 <.text+0x2339d38>
14233ad10: 49 8b 45 28                 	movq	0x28(%r13), %rax
14233ad14: 4c 89 e1                    	movq	%r12, %rcx
14233ad17: 48 c1 e1 05                 	shlq	$0x5, %rcx
14233ad1b: c7 04 08 0e 00 00 00        	movl	$0xe, (%rax,%rcx)
14233ad22: 48 89 5c 08 18              	movq	%rbx, 0x18(%rax,%rcx)
14233ad27: 49 ff c4                    	incq	%r12
14233ad2a: 4d 89 65 30                 	movq	%r12, 0x30(%r13)
14233ad2e: 49 ff 45 40                 	incq	0x40(%r13)
14233ad32: 48 89 fa                    	movq	%rdi, %rdx
14233ad35: 4d 89 f8                    	movq	%r15, %r8
14233ad38: 48 85 d2                    	testq	%rdx, %rdx
14233ad3b: 74 13                       	je	0x14233ad50 <.text+0x2339d50>
14233ad3d: 48 3b 55 e8                 	cmpq	-0x18(%rbp), %rdx
14233ad41: 74 0d                       	je	0x14233ad50 <.text+0x2339d50>
14233ad43: 48 8d 7a 10                 	leaq	0x10(%rdx), %rdi
14233ad47: 4d 89 c7                    	movq	%r8, %r15
14233ad4a: eb 20                       	jmp	0x14233ad6c <.text+0x2339d6c>
14233ad4c: 0f 1f 40 00                 	nopl	(%rax)
14233ad50: 4d 85 c0                    	testq	%r8, %r8
14233ad53: 0f 84 27 03 00 00           	je	0x14233b080 <.text+0x233a080>
14233ad59: 4c 3b 45 f0                 	cmpq	-0x10(%rbp), %r8
14233ad5d: 0f 84 1d 03 00 00           	je	0x14233b080 <.text+0x233a080>
14233ad63: 4d 8d 78 10                 	leaq	0x10(%r8), %r15
14233ad67: 31 ff                       	xorl	%edi, %edi
14233ad69: 4c 89 c2                    	movq	%r8, %rdx
14233ad6c: 48 8b 45 e0                 	movq	-0x20(%rbp), %rax
14233ad70: 48 ff 00                    	incq	(%rax)
14233ad73: 48 8b 42 08                 	movq	0x8(%rdx), %rax
14233ad77: 89 c1                       	movl	%eax, %ecx
14233ad79: 48 3b 4e 10                 	cmpq	0x10(%rsi), %rcx
14233ad7d: 73 61                       	jae	0x14233ade0 <.text+0x2339de0>
14233ad7f: 48 8b 56 08                 	movq	0x8(%rsi), %rdx
14233ad83: 49 89 c1                    	movq	%rax, %r9
14233ad86: 49 c1 e9 20                 	shrq	$0x20, %r9
14233ad8a: 4c 8d 04 89                 	leaq	(%rcx,%rcx,4), %r8
14233ad8e: 46 39 0c 82                 	cmpl	%r9d, (%rdx,%r8,4)
14233ad92: 75 4c                       	jne	0x14233ade0 <.text+0x2339de0>
14233ad94: 4a 8d 14 82                 	leaq	(%rdx,%r8,4), %rdx
14233ad98: 8b 52 04                    	movl	0x4(%rdx), %edx
14233ad9b: 41 b8 ff ff ff ff           	movl	$0xffffffff, %r8d       # imm = 0xFFFFFFFF
14233ada1: 4c 39 c2                    	cmpq	%r8, %rdx
14233ada4: 74 3a                       	je	0x14233ade0 <.text+0x2339de0>
14233ada6: 4c 8b 45 28                 	movq	0x28(%rbp), %r8
14233adaa: 49 39 50 38                 	cmpq	%rdx, 0x38(%r8)
14233adae: 76 30                       	jbe	0x14233ade0 <.text+0x2339de0>
14233adb0: 4c 8b 45 28                 	movq	0x28(%rbp), %r8
14233adb4: 4d 8b 40 28                 	movq	0x28(%r8), %r8
14233adb8: 41 89 d1                    	movl	%edx, %r9d
14233adbb: 41 c1 e9 06                 	shrl	$0x6, %r9d
14233adbf: 4f 8b 04 c8                 	movq	(%r8,%r9,8), %r8
14233adc3: 49 0f a3 d0                 	btq	%rdx, %r8
14233adc7: 48 89 fa                    	movq	%rdi, %rdx
14233adca: 4d 89 f8                    	movq	%r15, %r8
14233adcd: 0f 82 65 ff ff ff           	jb	0x14233ad38 <.text+0x2339d38>
14233add3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
14233ade0: 48 8b 55 60                 	movq	0x60(%rbp), %rdx
14233ade4: 48 3b 4a 10                 	cmpq	0x10(%rdx), %rcx
14233ade8: 0f 83 92 00 00 00           	jae	0x14233ae80 <.text+0x2339e80>
14233adee: 48 8b 55 60                 	movq	0x60(%rbp), %rdx
14233adf2: 48 8b 52 08                 	movq	0x8(%rdx), %rdx
14233adf6: 49 89 c1                    	movq	%rax, %r9
14233adf9: 49 c1 e9 20                 	shrq	$0x20, %r9
14233adfd: 4c 8d 04 89                 	leaq	(%rcx,%rcx,4), %r8
14233ae01: 46 39 0c 82                 	cmpl	%r9d, (%rdx,%r8,4)
14233ae05: 75 79                       	jne	0x14233ae80 <.text+0x2339e80>
14233ae07: 4a 8d 14 82                 	leaq	(%rdx,%r8,4), %rdx
14233ae0b: 8b 52 04                    	movl	0x4(%rdx), %edx
14233ae0e: 41 b8 ff ff ff ff           	movl	$0xffffffff, %r8d       # imm = 0xFFFFFFFF
14233ae14: 4c 39 c2                    	cmpq	%r8, %rdx
14233ae17: 74 67                       	je	0x14233ae80 <.text+0x2339e80>
14233ae19: 4c 8b 45 30                 	movq	0x30(%rbp), %r8
14233ae1d: 49 39 50 38                 	cmpq	%rdx, 0x38(%r8)
14233ae21: 76 5d                       	jbe	0x14233ae80 <.text+0x2339e80>
14233ae23: 4c 8b 45 30                 	movq	0x30(%rbp), %r8
14233ae27: 4d 8b 40 28                 	movq	0x28(%r8), %r8
14233ae2b: 41 89 d1                    	movl	%edx, %r9d
14233ae2e: 41 c1 e9 06                 	shrl	$0x6, %r9d
14233ae32: 4f 8b 04 c8                 	movq	(%r8,%r9,8), %r8
14233ae36: 49 0f a3 d0                 	btq	%rdx, %r8
14233ae3a: 73 44                       	jae	0x14233ae80 <.text+0x2339e80>
14233ae3c: 8b 45 58                    	movl	0x58(%rbp), %eax
14233ae3f: 48 8b 4d 38                 	movq	0x38(%rbp), %rcx
14233ae43: 89 01                       	movl	%eax, (%rcx)
14233ae45: 4d 8b 65 30                 	movq	0x30(%r13), %r12
14233ae49: 49 8b 5d 40                 	movq	0x40(%r13), %rbx
14233ae4d: 4d 3b 65 20                 	cmpq	0x20(%r13), %r12
14233ae51: 75 10                       	jne	0x14233ae63 <.text+0x2339e63>
14233ae53: 48 8b 4d 48                 	movq	0x48(%rbp), %rcx
14233ae57: 48 8d 15 12 a4 87 00        	leaq	0x87a412(%rip), %rdx    # 0x142bb5270
14233ae5e: e8 cd 7d 22 fe              	callq	0x140562c30 <.text+0x561c30>
14233ae63: 49 8b 45 28                 	movq	0x28(%r13), %rax
14233ae67: 4c 89 e1                    	movq	%r12, %rcx
14233ae6a: 48 c1 e1 05                 	shlq	$0x5, %rcx
14233ae6e: c7 04 08 0b 00 00 00        	movl	$0xb, (%rax,%rcx)
14233ae75: e9 a8 fe ff ff              	jmp	0x14233ad22 <.text+0x2339d22>
14233ae7a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
14233ae80: 48 8b 55 50                 	movq	0x50(%rbp), %rdx
14233ae84: 48 3b 4a 10                 	cmpq	0x10(%rdx), %rcx
14233ae88: 0f 83 f2 00 00 00           	jae	0x14233af80 <.text+0x2339f80>
14233ae8e: 48 8b 55 50                 	movq	0x50(%rbp), %rdx
14233ae92: 48 8b 52 08                 	movq	0x8(%rdx), %rdx
14233ae96: 49 89 c1                    	movq	%rax, %r9
14233ae99: 49 c1 e9 20                 	shrq	$0x20, %r9
14233ae9d: 4c 8d 04 89                 	leaq	(%rcx,%rcx,4), %r8
14233aea1: 46 39 0c 82                 	cmpl	%r9d, (%rdx,%r8,4)
14233aea5: 0f 85 d5 00 00 00           	jne	0x14233af80 <.text+0x2339f80>
14233aeab: 4a 8d 14 82                 	leaq	(%rdx,%r8,4), %rdx
14233aeaf: 44 8b 42 04                 	movl	0x4(%rdx), %r8d
14233aeb3: 41 b9 ff ff ff ff           	movl	$0xffffffff, %r9d       # imm = 0xFFFFFFFF
14233aeb9: 4d 39 c8                    	cmpq	%r9, %r8
14233aebc: 0f 84 be 00 00 00           	je	0x14233af80 <.text+0x2339f80>
14233aec2: 4c 8b 4d 18                 	movq	0x18(%rbp), %r9
14233aec6: 4d 39 41 38                 	cmpq	%r8, 0x38(%r9)
14233aeca: 0f 86 b0 00 00 00           	jbe	0x14233af80 <.text+0x2339f80>
14233aed0: 4c 8b 4d 18                 	movq	0x18(%rbp), %r9
14233aed4: 4d 8b 49 28                 	movq	0x28(%r9), %r9
14233aed8: 45 89 c2                    	movl	%r8d, %r10d
14233aedb: 41 c1 ea 06                 	shrl	$0x6, %r10d
14233aedf: 4f 8b 0c d1                 	movq	(%r9,%r10,8), %r9
14233aee3: 4d 0f a3 c1                 	btq	%r8, %r9
14233aee7: 0f 83 93 00 00 00           	jae	0x14233af80 <.text+0x2339f80>
14233aeed: 44 8b 42 0c                 	movl	0xc(%rdx), %r8d
14233aef1: 8b 52 10                    	movl	0x10(%rdx), %edx
14233aef4: 4c 8b 4d 50                 	movq	0x50(%rbp), %r9
14233aef8: 4d 8b 89 a8 01 00 00        	movq	0x1a8(%r9), %r9
14233aeff: 4f 8d 04 c0                 	leaq	(%r8,%r8,8), %r8
14233af03: 4c 8b 55 18                 	movq	0x18(%rbp), %r10
14233af07: 4d 8b 92 10 01 00 00        	movq	0x110(%r10), %r10
14233af0e: 4f 8b 5c c1 18              	movq	0x18(%r9,%r8,8), %r11
14233af13: 4f 8b 44 c1 38              	movq	0x38(%r9,%r8,8), %r8
14233af18: 4f 8b 04 d0                 	movq	(%r8,%r10,8), %r8
14233af1c: 49 f7 d0                    	notq	%r8
14233af1f: 4f 8d 04 40                 	leaq	(%r8,%r8,2), %r8
14233af23: 49 c1 e0 04                 	shlq	$0x4, %r8
14233af27: 4f 8b 44 03 10              	movq	0x10(%r11,%r8), %r8
14233af2c: 41 80 3c 10 09              	cmpb	$0x9, (%r8,%rdx)
14233af31: 75 4d                       	jne	0x14233af80 <.text+0x2339f80>
14233af33: 8b 45 58                    	movl	0x58(%rbp), %eax
14233af36: 48 8b 4d 38                 	movq	0x38(%rbp), %rcx
14233af3a: 89 01                       	movl	%eax, (%rcx)
14233af3c: 4d 8b 65 30                 	movq	0x30(%r13), %r12
14233af40: 49 8b 5d 40                 	movq	0x40(%r13), %rbx
14233af44: 4d 3b 65 20                 	cmpq	0x20(%r13), %r12
14233af48: 75 10                       	jne	0x14233af5a <.text+0x2339f5a>
14233af4a: 48 8b 4d 48                 	movq	0x48(%rbp), %rcx
14233af4e: 48 8d 15 1b a3 87 00        	leaq	0x87a31b(%rip), %rdx    # 0x142bb5270
14233af55: e8 d6 7c 22 fe              	callq	0x140562c30 <.text+0x561c30>
14233af5a: 49 8b 45 28                 	movq	0x28(%r13), %rax
14233af5e: 4c 89 e1                    	movq	%r12, %rcx
14233af61: 48 c1 e1 05                 	shlq	$0x5, %rcx
14233af65: c7 04 08 1d 00 00 00        	movl	$0x1d, (%rax,%rcx)
14233af6c: e9 b1 fd ff ff              	jmp	0x14233ad22 <.text+0x2339d22>
14233af71: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
14233af80: 48 8b 55 40                 	movq	0x40(%rbp), %rdx
14233af84: 48 3b 4a 10                 	cmpq	0x10(%rdx), %rcx
14233af88: 73 66                       	jae	0x14233aff0 <.text+0x2339ff0>
14233af8a: 48 8b 55 40                 	movq	0x40(%rbp), %rdx
14233af8e: 48 8b 52 08                 	movq	0x8(%rdx), %rdx
14233af92: 49 89 c1                    	movq	%rax, %r9
14233af95: 49 c1 e9 20                 	shrq	$0x20, %r9
14233af99: 4c 8d 04 89                 	leaq	(%rcx,%rcx,4), %r8
14233af9d: 46 39 0c 82                 	cmpl	%r9d, (%rdx,%r8,4)
14233afa1: 75 4d                       	jne	0x14233aff0 <.text+0x2339ff0>
14233afa3: 4a 8d 14 82                 	leaq	(%rdx,%r8,4), %rdx
14233afa7: 8b 52 04                    	movl	0x4(%rdx), %edx
14233afaa: 41 b8 ff ff ff ff           	movl	$0xffffffff, %r8d       # imm = 0xFFFFFFFF
14233afb0: 4c 39 c2                    	cmpq	%r8, %rdx
14233afb3: 74 3b                       	je	0x14233aff0 <.text+0x2339ff0>
14233afb5: 4c 8b 45 08                 	movq	0x8(%rbp), %r8
14233afb9: 49 39 50 38                 	cmpq	%rdx, 0x38(%r8)
14233afbd: 76 31                       	jbe	0x14233aff0 <.text+0x2339ff0>
14233afbf: 4c 8b 45 08                 	movq	0x8(%rbp), %r8
14233afc3: 4d 8b 40 28                 	movq	0x28(%r8), %r8
14233afc7: 41 89 d1                    	movl	%edx, %r9d
14233afca: 41 c1 e9 06                 	shrl	$0x6, %r9d
14233afce: 4f 8b 04 c8                 	movq	(%r8,%r9,8), %r8
14233afd2: 49 0f a3 d0                 	btq	%rdx, %r8
14233afd6: 48 89 fa                    	movq	%rdi, %rdx
14233afd9: 4d 89 f8                    	movq	%r15, %r8
14233afdc: 0f 82 56 fd ff ff           	jb	0x14233ad38 <.text+0x2339d38>
14233afe2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
14233aff0: 48 8b 55 20                 	movq	0x20(%rbp), %rdx
14233aff4: 48 3b 4a 10                 	cmpq	0x10(%rdx), %rcx
14233aff8: 73 56                       	jae	0x14233b050 <.text+0x233a050>
14233affa: 48 8b 55 20                 	movq	0x20(%rbp), %rdx
14233affe: 48 8b 52 08                 	movq	0x8(%rdx), %rdx
14233b002: 48 c1 e8 20                 	shrq	$0x20, %rax
14233b006: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
14233b00a: 39 04 8a                    	cmpl	%eax, (%rdx,%rcx,4)
14233b00d: 75 41                       	jne	0x14233b050 <.text+0x233a050>
14233b00f: 48 8d 04 8a                 	leaq	(%rdx,%rcx,4), %rax
14233b013: 8b 40 04                    	movl	0x4(%rax), %eax
14233b016: b9 ff ff ff ff              	movl	$0xffffffff, %ecx       # imm = 0xFFFFFFFF
14233b01b: 48 39 c8                    	cmpq	%rcx, %rax
14233b01e: 74 30                       	je	0x14233b050 <.text+0x233a050>
14233b020: 48 8b 4d 10                 	movq	0x10(%rbp), %rcx
14233b024: 48 39 41 38                 	cmpq	%rax, 0x38(%rcx)
14233b028: 76 26                       	jbe	0x14233b050 <.text+0x233a050>
14233b02a: 48 8b 4d 10                 	movq	0x10(%rbp), %rcx
14233b02e: 48 8b 49 28                 	movq	0x28(%rcx), %rcx
14233b032: 89 c2                       	movl	%eax, %edx
14233b034: c1 ea 06                    	shrl	$0x6, %edx
14233b037: 48 8b 0c d1                 	movq	(%rcx,%rdx,8), %rcx
14233b03b: 48 0f a3 c1                 	btq	%rax, %rcx
14233b03f: 48 89 fa                    	movq	%rdi, %rdx
14233b042: 4d 89 f8                    	movq	%r15, %r8
14233b045: 0f 82 ed fc ff ff           	jb	0x14233ad38 <.text+0x2339d38>
14233b04b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
14233b050: 8b 45 58                    	movl	0x58(%rbp), %eax
14233b053: 48 8b 4d 38                 	movq	0x38(%rbp), %rcx
14233b057: 89 01                       	movl	%eax, (%rcx)
14233b059: 4d 8b 65 30                 	movq	0x30(%r13), %r12
14233b05d: 49 8b 5d 40                 	movq	0x40(%r13), %rbx
14233b061: 4d 3b 65 20                 	cmpq	0x20(%r13), %r12
14233b065: 0f 85 a5 fc ff ff           	jne	0x14233ad10 <.text+0x2339d10>
14233b06b: 48 8b 4d 48                 	movq	0x48(%rbp), %rcx
14233b06f: 48 8d 15 fa a1 87 00        	leaq	0x87a1fa(%rip), %rdx    # 0x142bb5270
14233b076: e8 b5 7b 22 fe              	callq	0x140562c30 <.text+0x561c30>
14233b07b: e9 90 fc ff ff              	jmp	0x14233ad10 <.text+0x2339d10>
14233b080: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
14233b084: 48 8b 38                    	movq	(%rax), %rdi
14233b087: 48 8b 40 08                 	movq	0x8(%rax), %rax
14233b08b: 48 8b 17                    	movq	(%rdi), %rdx
14233b08e: 31 c9                       	xorl	%ecx, %ecx
14233b090: 49 89 d0                    	movq	%rdx, %r8
14233b093: 4c 2b 40 18                 	subq	0x18(%rax), %r8
14233b097: 4c 0f 42 c1                 	cmovbq	%rcx, %r8
14233b09b: 48 2b 50 38                 	subq	0x38(%rax), %rdx
14233b09f: 48 0f 42 d1                 	cmovbq	%rcx, %rdx
14233b0a3: 4f 8d 3c 40                 	leaq	(%r8,%r8,2), %r15
14233b0a7: 49 c1 e7 03                 	shlq	$0x3, %r15
14233b0ab: 4c 03 78 08                 	addq	0x8(%rax), %r15
14233b0af: 4c 8b 48 10                 	movq	0x10(%rax), %r9
14233b0b3: 4c 8b 50 30                 	movq	0x30(%rax), %r10
14233b0b7: 4d 29 c1                    	subq	%r8, %r9
14233b0ba: 4c 0f 42 c9                 	cmovbq	%rcx, %r9
14233b0be: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14233b0c4: 4d 0f 42 f8                 	cmovbq	%r8, %r15
14233b0c8: 48 8d 1c 52                 	leaq	(%rdx,%rdx,2), %rbx
14233b0cc: 48 c1 e3 03                 	shlq	$0x3, %rbx
14233b0d0: 48 03 58 28                 	addq	0x28(%rax), %rbx
14233b0d4: 49 29 d2                    	subq	%rdx, %r10
14233b0d7: 4c 0f 42 d1                 	cmovbq	%rcx, %r10
14233b0db: 49 0f 42 d8                 	cmovbq	%r8, %rbx
14233b0df: 48 8b 40 40                 	movq	0x40(%rax), %rax
14233b0e3: 4c 29 d0                    	subq	%r10, %rax
14233b0e6: 4c 29 c8                    	subq	%r9, %rax
14233b0e9: 48 89 07                    	movq	%rax, (%rdi)
14233b0ec: 4b 8d 04 49                 	leaq	(%r9,%r9,2), %rax
14233b0f0: 4d 8d 24 c7                 	leaq	(%r15,%rax,8), %r12
14233b0f4: 4b 8d 04 52                 	leaq	(%r10,%r10,2), %rax
14233b0f8: 4c 8d 34 c3                 	leaq	(%rbx,%rax,8), %r14
14233b0fc: 4c 8d 0d 6d a1 87 00        	leaq	0x87a16d(%rip), %r9     # 0x142bb5270
14233b103: eb 2b                       	jmp	0x14233b130 <.text+0x233a130>
14233b105: 49 8b 45 28                 	movq	0x28(%r13), %rax
14233b109: 48 8b 55 60                 	movq	0x60(%rbp), %rdx
14233b10d: 48 89 d1                    	movq	%rdx, %rcx
14233b110: 48 c1 e1 05                 	shlq	$0x5, %rcx
14233b114: 44 8b 45 40                 	movl	0x40(%rbp), %r8d
14233b118: 44 89 04 08                 	movl	%r8d, (%rax,%rcx)
14233b11c: 4c 8b 45 50                 	movq	0x50(%rbp), %r8
14233b120: 4c 89 44 08 18              	movq	%r8, 0x18(%rax,%rcx)
14233b125: 48 ff c2                    	incq	%rdx
14233b128: 49 89 55 30                 	movq	%rdx, 0x30(%r13)
14233b12c: 49 ff 45 40                 	incq	0x40(%r13)
14233b130: 4d 85 ff                    	testq	%r15, %r15
14233b133: 74 1b                       	je	0x14233b150 <.text+0x233a150>
14233b135: 4d 39 e7                    	cmpq	%r12, %r15
14233b138: 74 16                       	je	0x14233b150 <.text+0x233a150>
14233b13a: 49 8d 4f 18                 	leaq	0x18(%r15), %rcx
14233b13e: 4c 89 f8                    	movq	%r15, %rax
14233b141: eb 28                       	jmp	0x14233b16b <.text+0x233a16b>
14233b143: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
14233b150: 48 85 db                    	testq	%rbx, %rbx
14233b153: 0f 84 18 01 00 00           	je	0x14233b271 <.text+0x233a271>
14233b159: 4c 39 f3                    	cmpq	%r14, %rbx
14233b15c: 0f 84 0f 01 00 00           	je	0x14233b271 <.text+0x233a271>
14233b162: 48 89 d8                    	movq	%rbx, %rax
14233b165: 48 83 c3 18                 	addq	$0x18, %rbx
14233b169: 31 c9                       	xorl	%ecx, %ecx
14233b16b: 49 89 cf                    	movq	%rcx, %r15
14233b16e: 48 ff 07                    	incq	(%rdi)
14233b171: 48 8b 48 08                 	movq	0x8(%rax), %rcx
14233b175: 89 ca                       	movl	%ecx, %edx
14233b177: 48 3b 56 10                 	cmpq	0x10(%rsi), %rdx
14233b17b: 73 b3                       	jae	0x14233b130 <.text+0x233a130>
14233b17d: 4c 8b 46 08                 	movq	0x8(%rsi), %r8
14233b181: 48 c1 e9 20                 	shrq	$0x20, %rcx
14233b185: 48 8d 14 92                 	leaq	(%rdx,%rdx,4), %rdx
14233b189: 41 39 0c 90                 	cmpl	%ecx, (%r8,%rdx,4)
14233b18d: 75 a1                       	jne	0x14233b130 <.text+0x233a130>
14233b18f: 49 8d 0c 90                 	leaq	(%r8,%rdx,4), %rcx
14233b193: 8b 49 04                    	movl	0x4(%rcx), %ecx
14233b196: ba ff ff ff ff              	movl	$0xffffffff, %edx       # imm = 0xFFFFFFFF
14233b19b: 48 39 d1                    	cmpq	%rdx, %rcx
14233b19e: 74 90                       	je	0x14233b130 <.text+0x233a130>
14233b1a0: 48 8b 55 28                 	movq	0x28(%rbp), %rdx
14233b1a4: 48 39 4a 38                 	cmpq	%rcx, 0x38(%rdx)
14233b1a8: 76 86                       	jbe	0x14233b130 <.text+0x233a130>
14233b1aa: 48 8b 55 28                 	movq	0x28(%rbp), %rdx
14233b1ae: 48 8b 52 28                 	movq	0x28(%rdx), %rdx
14233b1b2: 41 89 c8                    	movl	%ecx, %r8d
14233b1b5: 41 c1 e8 06                 	shrl	$0x6, %r8d
14233b1b9: 4a 8b 14 c2                 	movq	(%rdx,%r8,8), %rdx
14233b1bd: 48 0f a3 ca                 	btq	%rcx, %rdx
14233b1c1: 0f 83 69 ff ff ff           	jae	0x14233b130 <.text+0x233a130>
14233b1c7: 0f b6 40 10                 	movzbl	0x10(%rax), %eax
14233b1cb: 04 fe                       	addb	$-0x2, %al
14233b1cd: 0f b6 c0                    	movzbl	%al, %eax
14233b1d0: 3c 03                       	cmpb	$0x3, %al
14233b1d2: b9 03 00 00 00              	movl	$0x3, %ecx
14233b1d7: 0f 43 c1                    	cmovael	%ecx, %eax
14233b1da: 0f b6 c0                    	movzbl	%al, %eax
14233b1dd: 48 8d 0d c4 2c d1 00        	leaq	0xd12cc4(%rip), %rcx    # 0x14304dea8
14233b1e4: 48 63 04 81                 	movslq	(%rcx,%rax,4), %rax
14233b1e8: 48 01 c8                    	addq	%rcx, %rax
14233b1eb: ff e0                       	jmpq	*%rax
14233b1ed: 8b 45 58                    	movl	0x58(%rbp), %eax
14233b1f0: 48 8b 4d 38                 	movq	0x38(%rbp), %rcx
14233b1f4: 89 01                       	movl	%eax, (%rcx)
14233b1f6: 49 8b 45 30                 	movq	0x30(%r13), %rax
14233b1fa: 49 8b 4d 40                 	movq	0x40(%r13), %rcx
14233b1fe: 48 89 4d 50                 	movq	%rcx, 0x50(%rbp)
14233b202: c7 45 40 1a 00 00 00        	movl	$0x1a, 0x40(%rbp)
14233b209: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
14233b20d: 49 3b 45 20                 	cmpq	0x20(%r13), %rax
14233b211: 0f 85 ee fe ff ff           	jne	0x14233b105 <.text+0x233a105>
14233b217: 48 8b 4d 48                 	movq	0x48(%rbp), %rcx
14233b21b: 4c 89 ca                    	movq	%r9, %rdx
14233b21e: e8 0d 7a 22 fe              	callq	0x140562c30 <.text+0x561c30>
14233b223: 4c 8d 0d 46 a0 87 00        	leaq	0x87a046(%rip), %r9     # 0x142bb5270
14233b22a: e9 d6 fe ff ff              	jmp	0x14233b105 <.text+0x233a105>
14233b22f: 8b 45 58                    	movl	0x58(%rbp), %eax
14233b232: 48 8b 4d 38                 	movq	0x38(%rbp), %rcx
14233b236: 89 01                       	movl	%eax, (%rcx)
14233b238: 49 8b 45 30                 	movq	0x30(%r13), %rax
14233b23c: 49 8b 4d 40                 	movq	0x40(%r13), %rcx
14233b240: 48 89 4d 50                 	movq	%rcx, 0x50(%rbp)
14233b244: c7 45 40 1b 00 00 00        	movl	$0x1b, 0x40(%rbp)
14233b24b: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
14233b24f: 49 3b 45 20                 	cmpq	0x20(%r13), %rax
14233b253: 0f 85 ac fe ff ff           	jne	0x14233b105 <.text+0x233a105>
14233b259: 48 8b 4d 48                 	movq	0x48(%rbp), %rcx
14233b25d: 4c 89 ca                    	movq	%r9, %rdx
14233b260: e8 cb 79 22 fe              	callq	0x140562c30 <.text+0x561c30>
14233b265: 4c 8d 0d 04 a0 87 00        	leaq	0x87a004(%rip), %r9     # 0x142bb5270
14233b26c: e9 94 fe ff ff              	jmp	0x14233b105 <.text+0x233a105>
14233b271: 80 7d 6f 00                 	cmpb	$0x0, 0x6f(%rbp)
14233b275: 74 3a                       	je	0x14233b2b1 <.text+0x233a2b1>
14233b277: e8 b4 ce 8d fe              	callq	0x140c18130 <.text+0xc17130>
14233b27c: 48 89 c1                    	movq	%rax, %rcx
14233b27f: 48 8b 00                    	movq	(%rax), %rax
14233b282: 48 83 f8 01                 	cmpq	$0x1, %rax
14233b286: 75 57                       	jne	0x14233b2df <.text+0x233a2df>
14233b288: 48 83 c1 08                 	addq	$0x8, %rcx
14233b28c: 48 83 39 00                 	cmpq	$0x0, (%rcx)
14233b290: 75 6d                       	jne	0x14233b2ff <.text+0x233a2ff>
14233b292: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
14233b299: 48 89 4d 60                 	movq	%rcx, 0x60(%rbp)
14233b29d: 48 83 c1 08                 	addq	$0x8, %rcx
14233b2a1: 48 8b 55 00                 	movq	(%rbp), %rdx
14233b2a5: e8 46 be 8d fe              	callq	0x140c170f0 <.text+0xc160f0>
14233b2aa: 48 8b 45 60                 	movq	0x60(%rbp), %rax
14233b2ae: 48 ff 00                    	incq	(%rax)
14233b2b1: 48 81 c4 f8 00 00 00        	addq	$0xf8, %rsp
14233b2b8: 5b                          	popq	%rbx
14233b2b9: 5f                          	popq	%rdi
14233b2ba: 5e                          	popq	%rsi
14233b2bb: 41 5c                       	popq	%r12
14233b2bd: 41 5d                       	popq	%r13
14233b2bf: 41 5e                       	popq	%r14
14233b2c1: 41 5f                       	popq	%r15
14233b2c3: 5d                          	popq	%rbp
14233b2c4: c3                          	retq
14233b2c5: 48 85 c0                    	testq	%rax, %rax
14233b2c8: 75 29                       	jne	0x14233b2f3 <.text+0x233a2f3>
14233b2ca: 31 d2                       	xorl	%edx, %edx
14233b2cc: e8 7f af 5a 00              	callq	0x1428e6250 <.text+0x28e5250>
14233b2d1: 48 89 c1                    	movq	%rax, %rcx
14233b2d4: 48 85 c0                    	testq	%rax, %rax
14233b2d7: 0f 85 2d f7 ff ff           	jne	0x14233aa0a <.text+0x2339a0a>
14233b2dd: eb 14                       	jmp	0x14233b2f3 <.text+0x233a2f3>
14233b2df: 48 85 c0                    	testq	%rax, %rax
14233b2e2: 75 0f                       	jne	0x14233b2f3 <.text+0x233a2f3>
14233b2e4: 31 d2                       	xorl	%edx, %edx
14233b2e6: e8 65 af 5a 00              	callq	0x1428e6250 <.text+0x28e5250>
14233b2eb: 48 89 c1                    	movq	%rax, %rcx
14233b2ee: 48 85 c0                    	testq	%rax, %rax
14233b2f1: 75 99                       	jne	0x14233b28c <.text+0x233a28c>
14233b2f3: 48 8d 0d de 9f 87 00        	leaq	0x879fde(%rip), %rcx    # 0x142bb52d8
14233b2fa: e8 d1 cd 59 00              	callq	0x1428d80d0 <.text+0x28d70d0>
14233b2ff: 48 8d 0d 42 a0 87 00        	leaq	0x87a042(%rip), %rcx    # 0x142bb5348
14233b306: e8 45 df 59 00              	callq	0x1428d9250 <.text+0x28d8250>
14233b30b: cc                          	int3
14233b30c: 0f 1f 40 00                 	nopl	(%rax)
14233b310: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14233b315: 55                          	pushq	%rbp
14233b316: 41 57                       	pushq	%r15
14233b318: 41 56                       	pushq	%r14
14233b31a: 41 55                       	pushq	%r13
14233b31c: 41 54                       	pushq	%r12
14233b31e: 56                          	pushq	%rsi
14233b31f: 57                          	pushq	%rdi
14233b320: 53                          	pushq	%rbx
14233b321: 48 83 ec 38                 	subq	$0x38, %rsp
14233b325: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14233b32c: 48 8b 45 58                 	movq	0x58(%rbp), %rax
14233b330: 48 ff 00                    	incq	(%rax)
14233b333: 48 83 c4 38                 	addq	$0x38, %rsp
14233b337: 5b                          	popq	%rbx
14233b338: 5f                          	popq	%rdi
14233b339: 5e                          	popq	%rsi
14233b33a: 41 5c                       	popq	%r12
14233b33c: 41 5d                       	popq	%r13
14233b33e: 41 5e                       	popq	%r14
14233b340: 41 5f                       	popq	%r15
14233b342: 5d                          	popq	%rbp
14233b343: c3                          	retq
14233b344: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
14233b350: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14233b355: 55                          	pushq	%rbp
14233b356: 41 57                       	pushq	%r15
14233b358: 41 56                       	pushq	%r14
14233b35a: 41 55                       	pushq	%r13
14233b35c: 41 54                       	pushq	%r12
14233b35e: 56                          	pushq	%rsi
14233b35f: 57                          	pushq	%rdi
14233b360: 53                          	pushq	%rbx
14233b361: 48 83 ec 38                 	subq	$0x38, %rsp
14233b365: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14233b36c: 80 7d 6f 00                 	cmpb	$0x0, 0x6f(%rbp)
14233b370: 74 20                       	je	0x14233b392 <.text+0x233a392>
14233b372: 48 8d 45 c8                 	leaq	-0x38(%rbp), %rax
14233b376: 48 89 45 d8                 	movq	%rax, -0x28(%rbp)
14233b37a: 48 8d 45 d8                 	leaq	-0x28(%rbp), %rax
14233b37e: 48 89 45 d0                 	movq	%rax, -0x30(%rbp)
14233b382: 48 8d 0d bf 27 d1 00        	leaq	0xd127bf(%rip), %rcx    # 0x14304db48
14233b389: 48 8d 55 d0                 	leaq	-0x30(%rbp), %rdx
14233b38d: e8 8e 48 c9 fe              	callq	0x140fcfc20 <.text+0xfcec20>
14233b392: 90                          	nop
14233b393: 48 83 c4 38                 	addq	$0x38, %rsp
14233b397: 5b                          	popq	%rbx
14233b398: 5f                          	popq	%rdi
14233b399: 5e                          	popq	%rsi
14233b39a: 41 5c                       	popq	%r12
14233b39c: 41 5d                       	popq	%r13
14233b39e: 41 5e                       	popq	%r14
14233b3a0: 41 5f                       	popq	%r15
14233b3a2: 5d                          	popq	%rbp
14233b3a3: c3                          	retq
14233b3a4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
14233b3b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14233b3b5: 55                          	pushq	%rbp
14233b3b6: 41 57                       	pushq	%r15
14233b3b8: 41 56                       	pushq	%r14
14233b3ba: 41 55                       	pushq	%r13
14233b3bc: 41 54                       	pushq	%r12
14233b3be: 56                          	pushq	%rsi
14233b3bf: 57                          	pushq	%rdi
14233b3c0: 53                          	pushq	%rbx
14233b3c1: 48 83 ec 38                 	subq	$0x38, %rsp
14233b3c5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14233b3cc: 48 8b 45 60                 	movq	0x60(%rbp), %rax
14233b3d0: 48 ff 00                    	incq	(%rax)
14233b3d3: 48 83 c4 38                 	addq	$0x38, %rsp
14233b3d7: 5b                          	popq	%rbx
14233b3d8: 5f                          	popq	%rdi
14233b3d9: 5e                          	popq	%rsi
14233b3da: 41 5c                       	popq	%r12
14233b3dc: 41 5d                       	popq	%r13
14233b3de: 41 5e                       	popq	%r14
14233b3e0: 41 5f                       	popq	%r15
14233b3e2: 5d                          	popq	%rbp
14233b3e3: c3                          	retq
