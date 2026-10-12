D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140bd4410: 56                          	pushq	%rsi
140bd4411: 57                          	pushq	%rdi
140bd4412: 48 83 ec 28                 	subq	$0x28, %rsp
140bd4416: 48 89 ce                    	movq	%rcx, %rsi
140bd4419: 48 c7 41 10 00 00 00 00     	movq	$0x0, 0x10(%rcx)
140bd4421: 48 c7 41 28 00 00 00 00     	movq	$0x0, 0x28(%rcx)
140bd4429: 48 c7 41 40 00 00 00 00     	movq	$0x0, 0x40(%rcx)
140bd4431: 48 c7 41 58 00 00 00 00     	movq	$0x0, 0x58(%rcx)
140bd4439: 48 c7 41 70 00 00 00 00     	movq	$0x0, 0x70(%rcx)
140bd4441: 48 83 b9 a8 00 00 00 00     	cmpq	$0x0, 0xa8(%rcx)
140bd4449: 74 4a                       	je	0x140bd4495 <.text+0xbd3495>
140bd444b: 48 8b be 98 00 00 00        	movq	0x98(%rsi), %rdi
140bd4452: 48 85 ff                    	testq	%rdi, %rdi
140bd4455: 74 12                       	je	0x140bd4469 <.text+0xbd3469>
140bd4457: 48 8b 8e 90 00 00 00        	movq	0x90(%rsi), %rcx
140bd445e: 4c 8d 47 11                 	leaq	0x11(%rdi), %r8
140bd4462: b2 ff                       	movb	$-0x1, %dl
140bd4464: e8 b7 f7 d4 01              	callq	0x142923c20 <.text+0x2922c20>
140bd4469: 48 c7 86 a8 00 00 00 00 00 00 00    	movq	$0x0, 0xa8(%rsi)
140bd4474: 48 8d 47 01                 	leaq	0x1(%rdi), %rax
140bd4478: 48 89 c1                    	movq	%rax, %rcx
140bd447b: 48 c1 e9 03                 	shrq	$0x3, %rcx
140bd447f: 48 83 e0 f8                 	andq	$-0x8, %rax
140bd4483: 48 29 c8                    	subq	%rcx, %rax
140bd4486: 48 83 ff 08                 	cmpq	$0x8, %rdi
140bd448a: 48 0f 42 c7                 	cmovbq	%rdi, %rax
140bd448e: 48 89 86 a0 00 00 00        	movq	%rax, 0xa0(%rsi)
140bd4495: 48 c7 86 88 00 00 00 00 00 00 00    	movq	$0x0, 0x88(%rsi)
140bd44a0: 48 83 c4 28                 	addq	$0x28, %rsp
140bd44a4: 5f                          	popq	%rdi
140bd44a5: 5e                          	popq	%rsi
140bd44a6: c3                          	retq
140bd44a7: cc                          	int3
140bd44a8: cc                          	int3
140bd44a9: cc                          	int3
140bd44aa: cc                          	int3
140bd44ab: cc                          	int3
140bd44ac: cc                          	int3
140bd44ad: cc                          	int3
140bd44ae: cc                          	int3
140bd44af: cc                          	int3
140bd44b0: 56                          	pushq	%rsi
140bd44b1: 57                          	pushq	%rdi
140bd44b2: 53                          	pushq	%rbx
140bd44b3: 48 83 ec 20                 	subq	$0x20, %rsp
140bd44b7: 4c 8b 5a 10                 	movq	0x10(%rdx), %r11
140bd44bb: 4d 85 db                    	testq	%r11, %r11
140bd44be: 0f 84 80 00 00 00           	je	0x140bd4544 <.text+0xbd3544>
140bd44c4: 4c 8b 42 08                 	movq	0x8(%rdx), %r8
140bd44c8: 48 8b 51 70                 	movq	0x70(%rcx), %rdx
140bd44cc: 4c 8b 49 68                 	movq	0x68(%rcx), %r9
140bd44d0: 4c 8b 51 50                 	movq	0x50(%rcx), %r10
140bd44d4: 48 8b 41 58                 	movq	0x58(%rcx), %rax
140bd44d8: 49 c1 e3 03                 	shlq	$0x3, %r11
140bd44dc: 4f 8d 1c db                 	leaq	(%r11,%r11,8), %r11
140bd44e0: 31 f6                       	xorl	%esi, %esi
140bd44e2: 31 c9                       	xorl	%ecx, %ecx
140bd44e4: eb 21                       	jmp	0x140bd4507 <.text+0xbd3507>
140bd44e6: bf 18 00 00 00              	movl	$0x18, %edi
140bd44eb: 4c 89 d3                    	movq	%r10, %rbx
140bd44ee: 48 39 c1                    	cmpq	%rax, %rcx
140bd44f1: 73 5a                       	jae	0x140bd454d <.text+0xbd354d>
140bd44f3: 41 8b 3c 38                 	movl	(%r8,%rdi), %edi
140bd44f7: 89 3c 8b                    	movl	%edi, (%rbx,%rcx,4)
140bd44fa: 49 83 c0 48                 	addq	$0x48, %r8
140bd44fe: 48 ff c1                    	incq	%rcx
140bd4501: 49 83 c3 b8                 	addq	$-0x48, %r11
140bd4505: 74 3d                       	je	0x140bd4544 <.text+0xbd3544>
140bd4507: 49 83 38 00                 	cmpq	$0x0, (%r8)
140bd450b: 75 ed                       	jne	0x140bd44fa <.text+0xbd34fa>
140bd450d: 41 0f b6 78 36              	movzbl	0x36(%r8), %edi
140bd4512: 8d 5f f9                    	leal	-0x7(%rdi), %ebx
140bd4515: 48 83 c7 fa                 	addq	$-0x6, %rdi
140bd4519: 80 fb 02                    	cmpb	$0x2, %bl
140bd451c: 48 0f 43 fe                 	cmovaeq	%rsi, %rdi
140bd4520: 48 85 ff                    	testq	%rdi, %rdi
140bd4523: 74 c1                       	je	0x140bd44e6 <.text+0xbd34e6>
140bd4525: 48 83 ff 01                 	cmpq	$0x1, %rdi
140bd4529: 75 cf                       	jne	0x140bd44fa <.text+0xbd34fa>
140bd452b: bf 20 00 00 00              	movl	$0x20, %edi
140bd4530: 4c 89 cb                    	movq	%r9, %rbx
140bd4533: 48 39 d1                    	cmpq	%rdx, %rcx
140bd4536: 72 bb                       	jb	0x140bd44f3 <.text+0xbd34f3>
140bd4538: 4c 8d 05 09 b1 f3 01        	leaq	0x1f3b109(%rip), %r8    # 0x142b0f648
140bd453f: e8 d4 4f d0 01              	callq	0x1428d9518 <.text+0x28d8518>
140bd4544: 90                          	nop
140bd4545: 48 83 c4 20                 	addq	$0x20, %rsp
140bd4549: 5b                          	popq	%rbx
140bd454a: 5f                          	popq	%rdi
140bd454b: 5e                          	popq	%rsi
140bd454c: c3                          	retq
140bd454d: 4c 8d 05 dc b0 f3 01        	leaq	0x1f3b0dc(%rip), %r8    # 0x142b0f630
140bd4554: 48 89 c2                    	movq	%rax, %rdx
140bd4557: e8 bc 4f d0 01              	callq	0x1428d9518 <.text+0x28d8518>
140bd455c: cc                          	int3
140bd455d: cc                          	int3
140bd455e: cc                          	int3
140bd455f: cc                          	int3
140bd4560: 41 57                       	pushq	%r15
140bd4562: 41 56                       	pushq	%r14
140bd4564: 41 54                       	pushq	%r12
140bd4566: 56                          	pushq	%rsi
140bd4567: 57                          	pushq	%rdi
140bd4568: 55                          	pushq	%rbp
140bd4569: 53                          	pushq	%rbx
140bd456a: 48 83 ec 70                 	subq	$0x70, %rsp
140bd456e: 48 8b 42 28                 	movq	0x28(%rdx), %rax
140bd4572: 48 85 c0                    	testq	%rax, %rax
140bd4575: 0f 84 a5 01 00 00           	je	0x140bd4720 <.text+0xbd3720>
140bd457b: 48 89 ce                    	movq	%rcx, %rsi
140bd457e: 4c 8b 7a 20                 	movq	0x20(%rdx), %r15
140bd4582: 48 8d b9 90 00 00 00        	leaq	0x90(%rcx), %rdi
140bd4589: 48 c1 e0 04                 	shlq	$0x4, %rax
140bd458d: 4c 8d 24 40                 	leaq	(%rax,%rax,2), %r12
140bd4591: 48 8d 5c 24 30              	leaq	0x30(%rsp), %rbx
140bd4596: eb 23                       	jmp	0x140bd45bb <.text+0xbd35bb>
140bd4598: 81 fd 00 02 00 00           	cmpl	$0x200, %ebp            # imm = 0x200
140bd459e: 0f 85 8b 01 00 00           	jne	0x140bd472f <.text+0xbd372f>
140bd45a4: b9 20 00 00 00              	movl	$0x20, %ecx
140bd45a9: 42 09 0c b0                 	orl	%ecx, (%rax,%r14,4)
140bd45ad: 49 83 c7 30                 	addq	$0x30, %r15
140bd45b1: 49 83 c4 d0                 	addq	$-0x30, %r12
140bd45b5: 0f 84 65 01 00 00           	je	0x140bd4720 <.text+0xbd3720>
140bd45bb: 49 8d 47 04                 	leaq	0x4(%r15), %rax
140bd45bf: 49 8d 4f 18                 	leaq	0x18(%r15), %rcx
140bd45c3: 41 80 3f 00                 	cmpb	$0x0, (%r15)
140bd45c7: 48 0f 45 c1                 	cmovneq	%rcx, %rax
140bd45cb: 44 8b 30                    	movl	(%rax), %r14d
140bd45ce: 41 0f b6 57 28              	movzbl	0x28(%r15), %edx
140bd45d3: 48 89 d9                    	movq	%rbx, %rcx
140bd45d6: e8 f5 c7 03 00              	callq	0x140c10dd0 <.text+0xc0fdd0>
140bd45db: 49 81 fe 00 00 01 00        	cmpq	$0x10000, %r14          # imm = 0x10000
140bd45e2: 0f 83 9f 01 00 00           	jae	0x140bd4787 <.text+0xbd3787>
140bd45e8: 8b 6c 24 34                 	movl	0x34(%rsp), %ebp
140bd45ec: 48 89 f9                    	movq	%rdi, %rcx
140bd45ef: 44 89 f2                    	movl	%r14d, %edx
140bd45f2: e8 79 96 cd 00              	callq	0x1418adc70 <.text+0x18acc70>
140bd45f7: 85 ed                       	testl	%ebp, %ebp
140bd45f9: 74 b2                       	je	0x140bd45ad <.text+0xbd35ad>
140bd45fb: 41 f6 07 01                 	testb	$0x1, (%r15)
140bd45ff: 74 4f                       	je	0x140bd4650 <.text+0xbd3650>
140bd4601: 48 8b 56 70                 	movq	0x70(%rsi), %rdx
140bd4605: 4c 39 f2                    	cmpq	%r14, %rdx
140bd4608: 0f 86 a2 01 00 00           	jbe	0x140bd47b0 <.text+0xbd37b0>
140bd460e: 48 8b 46 68                 	movq	0x68(%rsi), %rax
140bd4612: 89 e9                       	movl	%ebp, %ecx
140bd4614: 83 e1 01                    	andl	$0x1, %ecx
140bd4617: c1 e1 08                    	shll	$0x8, %ecx
140bd461a: 89 ea                       	movl	%ebp, %edx
140bd461c: 83 e2 06                    	andl	$0x6, %edx
140bd461f: c1 e2 05                    	shll	$0x5, %edx
140bd4622: 09 ca                       	orl	%ecx, %edx
140bd4624: 89 e9                       	movl	%ebp, %ecx
140bd4626: 83 e1 08                    	andl	$0x8, %ecx
140bd4629: 44 8d 04 4a                 	leal	(%rdx,%rcx,2), %r8d
140bd462d: f7 c5 60 80 01 00           	testl	$0x18060, %ebp          # imm = 0x18060
140bd4633: 8d 4c 4a 20                 	leal	0x20(%rdx,%rcx,2), %ecx
140bd4637: 41 0f 44 c8                 	cmovel	%r8d, %ecx
140bd463b: c1 ed 0b                    	shrl	$0xb, %ebp
140bd463e: 83 e5 03                    	andl	$0x3, %ebp
140bd4641: 09 cd                       	orl	%ecx, %ebp
140bd4643: 42 09 2c b0                 	orl	%ebp, (%rax,%r14,4)
140bd4647: e9 61 ff ff ff              	jmp	0x140bd45ad <.text+0xbd35ad>
140bd464c: 0f 1f 40 00                 	nopl	(%rax)
140bd4650: 48 8b 56 58                 	movq	0x58(%rsi), %rdx
140bd4654: 4c 39 f2                    	cmpq	%r14, %rdx
140bd4657: 0f 86 62 01 00 00           	jbe	0x140bd47bf <.text+0xbd37bf>
140bd465d: 48 8b 46 50                 	movq	0x50(%rsi), %rax
140bd4661: 89 6c 24 2c                 	movl	%ebp, 0x2c(%rsp)
140bd4665: 81 fd 1f 02 00 00           	cmpl	$0x21f, %ebp            # imm = 0x21F
140bd466b: 7f 23                       	jg	0x140bd4690 <.text+0xbd3690>
140bd466d: 81 fd ff 00 00 00           	cmpl	$0xff, %ebp
140bd4673: 7f 45                       	jg	0x140bd46ba <.text+0xbd36ba>
140bd4675: 83 fd 20                    	cmpl	$0x20, %ebp
140bd4678: 0f 84 98 00 00 00           	je	0x140bd4716 <.text+0xbd3716>
140bd467e: 83 fd 40                    	cmpl	$0x40, %ebp
140bd4681: 74 2d                       	je	0x140bd46b0 <.text+0xbd36b0>
140bd4683: 81 fd 80 00 00 00           	cmpl	$0x80, %ebp
140bd4689: 74 43                       	je	0x140bd46ce <.text+0xbd36ce>
140bd468b: e9 9f 00 00 00              	jmp	0x140bd472f <.text+0xbd372f>
140bd4690: 81 fd ff 07 00 00           	cmpl	$0x7ff, %ebp            # imm = 0x7FF
140bd4696: 7e 40                       	jle	0x140bd46d8 <.text+0xbd36d8>
140bd4698: 81 fd 00 08 00 00           	cmpl	$0x800, %ebp            # imm = 0x800
140bd469e: 74 6c                       	je	0x140bd470c <.text+0xbd370c>
140bd46a0: 81 fd 00 10 00 00           	cmpl	$0x1000, %ebp           # imm = 0x1000
140bd46a6: 74 5a                       	je	0x140bd4702 <.text+0xbd3702>
140bd46a8: 81 fd 00 80 01 00           	cmpl	$0x18000, %ebp          # imm = 0x18000
140bd46ae: 75 7f                       	jne	0x140bd472f <.text+0xbd372f>
140bd46b0: b9 08 00 00 00              	movl	$0x8, %ecx
140bd46b5: e9 ef fe ff ff              	jmp	0x140bd45a9 <.text+0xbd35a9>
140bd46ba: 81 fd 00 01 00 00           	cmpl	$0x100, %ebp            # imm = 0x100
140bd46c0: 74 0c                       	je	0x140bd46ce <.text+0xbd36ce>
140bd46c2: 81 fd 80 01 00 00           	cmpl	$0x180, %ebp            # imm = 0x180
140bd46c8: 0f 85 ca fe ff ff           	jne	0x140bd4598 <.text+0xbd3598>
140bd46ce: b9 10 00 00 00              	movl	$0x10, %ecx
140bd46d3: e9 d1 fe ff ff              	jmp	0x140bd45a9 <.text+0xbd35a9>
140bd46d8: 81 fd 00 04 00 00           	cmpl	$0x400, %ebp            # imm = 0x400
140bd46de: 0f 84 c0 fe ff ff           	je	0x140bd45a4 <.text+0xbd35a4>
140bd46e4: 81 fd 00 06 00 00           	cmpl	$0x600, %ebp            # imm = 0x600
140bd46ea: 0f 84 b4 fe ff ff           	je	0x140bd45a4 <.text+0xbd35a4>
140bd46f0: 81 fd 20 02 00 00           	cmpl	$0x220, %ebp            # imm = 0x220
140bd46f6: 75 37                       	jne	0x140bd472f <.text+0xbd372f>
140bd46f8: b9 24 00 00 00              	movl	$0x24, %ecx
140bd46fd: e9 a7 fe ff ff              	jmp	0x140bd45a9 <.text+0xbd35a9>
140bd4702: b9 02 00 00 00              	movl	$0x2, %ecx
140bd4707: e9 9d fe ff ff              	jmp	0x140bd45a9 <.text+0xbd35a9>
140bd470c: b9 01 00 00 00              	movl	$0x1, %ecx
140bd4711: e9 93 fe ff ff              	jmp	0x140bd45a9 <.text+0xbd35a9>
140bd4716: b9 04 00 00 00              	movl	$0x4, %ecx
140bd471b: e9 89 fe ff ff              	jmp	0x140bd45a9 <.text+0xbd35a9>
140bd4720: 48 83 c4 70                 	addq	$0x70, %rsp
140bd4724: 5b                          	popq	%rbx
140bd4725: 5d                          	popq	%rbp
140bd4726: 5f                          	popq	%rdi
140bd4727: 5e                          	popq	%rsi
140bd4728: 41 5c                       	popq	%r12
140bd472a: 41 5e                       	popq	%r14
140bd472c: 41 5f                       	popq	%r15
140bd472e: c3                          	retq
140bd472f: 48 8d 44 24 2c              	leaq	0x2c(%rsp), %rax
140bd4734: 48 89 44 24 60              	movq	%rax, 0x60(%rsp)
140bd4739: 48 8d 05 50 33 0b 00        	leaq	0xb3350(%rip), %rax     # 0x140c87a90 <.text+0xc86a90>
140bd4740: 48 89 44 24 68              	movq	%rax, 0x68(%rsp)
140bd4745: 48 8d 05 c4 af f3 01        	leaq	0x1f3afc4(%rip), %rax   # 0x142b0f710
140bd474c: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
140bd4751: 48 c7 44 24 38 01 00 00 00  	movq	$0x1, 0x38(%rsp)
140bd475a: 48 c7 44 24 50 00 00 00 00  	movq	$0x0, 0x50(%rsp)
140bd4763: 48 8d 44 24 60              	leaq	0x60(%rsp), %rax
140bd4768: 48 89 44 24 40              	movq	%rax, 0x40(%rsp)
140bd476d: 48 c7 44 24 48 01 00 00 00  	movq	$0x1, 0x48(%rsp)
140bd4776: 48 8d 15 a3 af f3 01        	leaq	0x1f3afa3(%rip), %rdx   # 0x142b0f720
140bd477d: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
140bd4782: e8 09 4c d0 01              	callq	0x1428d9390 <.text+0x28d8390>
140bd4787: 48 8d 05 d2 ae f3 01        	leaq	0x1f3aed2(%rip), %rax   # 0x142b0f660
140bd478e: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140bd4793: 48 8d 0d 0e ae f3 01        	leaq	0x1f3ae0e(%rip), %rcx   # 0x142b0f5a8
140bd479a: 4c 8d 0d e7 ad f3 01        	leaq	0x1f3ade7(%rip), %r9    # 0x142b0f588
140bd47a1: 4c 8d 44 24 30              	leaq	0x30(%rsp), %r8
140bd47a6: ba 2b 00 00 00              	movl	$0x2b, %edx
140bd47ab: e8 b0 4f d0 01              	callq	0x1428d9760 <.text+0x28d8760>
140bd47b0: 4c 8d 05 d9 ae f3 01        	leaq	0x1f3aed9(%rip), %r8    # 0x142b0f690
140bd47b7: 4c 89 f1                    	movq	%r14, %rcx
140bd47ba: e8 59 4d d0 01              	callq	0x1428d9518 <.text+0x28d8518>
140bd47bf: 4c 8d 05 b2 ae f3 01        	leaq	0x1f3aeb2(%rip), %r8    # 0x142b0f678
140bd47c6: 4c 89 f1                    	movq	%r14, %rcx
140bd47c9: e8 4a 4d d0 01              	callq	0x1428d9518 <.text+0x28d8518>
140bd47ce: cc                          	int3
140bd47cf: cc                          	int3
