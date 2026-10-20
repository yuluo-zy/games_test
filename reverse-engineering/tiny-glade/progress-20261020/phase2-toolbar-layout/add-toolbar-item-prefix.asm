
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141e4dc40: 55                          	pushq	%rbp
141e4dc41: 41 57                       	pushq	%r15
141e4dc43: 41 56                       	pushq	%r14
141e4dc45: 41 55                       	pushq	%r13
141e4dc47: 41 54                       	pushq	%r12
141e4dc49: 56                          	pushq	%rsi
141e4dc4a: 57                          	pushq	%rdi
141e4dc4b: 53                          	pushq	%rbx
141e4dc4c: 48 81 ec d8 02 00 00        	subq	$0x2d8, %rsp            # imm = 0x2D8
141e4dc53: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
141e4dc5b: 0f 29 b5 40 02 00 00        	movaps	%xmm6, 0x240(%rbp)
141e4dc62: 48 c7 85 38 02 00 00 fe ff ff ff    	movq	$-0x2, 0x238(%rbp)
141e4dc6d: 44 89 c3                    	movl	%r8d, %ebx
141e4dc70: 41 89 d6                    	movl	%edx, %r14d
141e4dc73: 48 89 cf                    	movq	%rcx, %rdi
141e4dc76: 48 8b 85 c8 02 00 00        	movq	0x2c8(%rbp), %rax
141e4dc7d: 48 89 85 28 02 00 00        	movq	%rax, 0x228(%rbp)
141e4dc84: 48 8b b5 c0 02 00 00        	movq	0x2c0(%rbp), %rsi
141e4dc8b: 4c 89 8d 10 02 00 00        	movq	%r9, 0x210(%rbp)
141e4dc92: 41 0f b6 01                 	movzbl	(%r9), %eax
141e4dc96: 83 f8 04                    	cmpl	$0x4, %eax
141e4dc99: 89 85 24 02 00 00           	movl	%eax, 0x224(%rbp)
141e4dc9f: 0f 84 e7 02 00 00           	je	0x141e4df8c <.text+0x1e4cf8c>
141e4dca5: 83 f8 05                    	cmpl	$0x5, %eax
141e4dca8: 0f 85 be 05 00 00           	jne	0x141e4e26c <.text+0x1e4d26c>
141e4dcae: 48 8b 8d 10 02 00 00        	movq	0x210(%rbp), %rcx
141e4dcb5: 48 8b 41 18                 	movq	0x18(%rcx), %rax
141e4dcb9: 48 89 85 a0 01 00 00        	movq	%rax, 0x1a0(%rbp)
141e4dcc0: 0f 10 41 08                 	movups	0x8(%rcx), %xmm0
141e4dcc4: 0f 29 85 90 01 00 00        	movaps	%xmm0, 0x190(%rbp)
141e4dccb: 66 c7 85 78 01 00 00 00 00  	movw	$0x0, 0x178(%rbp)
141e4dcd4: 0f 10 05 5d 4a d7 00        	movups	0xd74a5d(%rip), %xmm0   # 0x142bc2738
141e4dcdb: 0f 11 85 e8 00 00 00        	movups	%xmm0, 0xe8(%rbp)
141e4dce2: 0f 10 05 5f 4a d7 00        	movups	0xd74a5f(%rip), %xmm0   # 0x142bc2748
141e4dce9: 0f 11 85 f8 00 00 00        	movups	%xmm0, 0xf8(%rbp)
141e4dcf0: 0f 10 05 61 4a d7 00        	movups	0xd74a61(%rip), %xmm0   # 0x142bc2758
141e4dcf7: 0f 11 85 08 01 00 00        	movups	%xmm0, 0x108(%rbp)
141e4dcfe: 0f 10 05 63 4a d7 00        	movups	0xd74a63(%rip), %xmm0   # 0x142bc2768
141e4dd05: 0f 11 85 18 01 00 00        	movups	%xmm0, 0x118(%rbp)
141e4dd0c: 0f 10 05 65 4a d7 00        	movups	0xd74a65(%rip), %xmm0   # 0x142bc2778
141e4dd13: 0f 11 85 28 01 00 00        	movups	%xmm0, 0x128(%rbp)
141e4dd1a: 48 c7 85 90 00 00 00 00 00 00 00    	movq	$0x0, 0x90(%rbp)
141e4dd25: 0f 10 05 5c 4a d7 00        	movups	0xd74a5c(%rip), %xmm0   # 0x142bc2788
141e4dd2c: 0f 11 85 38 01 00 00        	movups	%xmm0, 0x138(%rbp)
141e4dd33: 0f 10 05 5e 4a d7 00        	movups	0xd74a5e(%rip), %xmm0   # 0x142bc2798
141e4dd3a: 0f 11 85 48 01 00 00        	movups	%xmm0, 0x148(%rbp)
141e4dd41: 0f 10 05 a0 49 d7 00        	movups	0xd749a0(%rip), %xmm0   # 0x142bc26e8
141e4dd48: 0f 11 85 98 00 00 00        	movups	%xmm0, 0x98(%rbp)
141e4dd4f: 0f 10 05 a2 49 d7 00        	movups	0xd749a2(%rip), %xmm0   # 0x142bc26f8
141e4dd56: 0f 11 85 a8 00 00 00        	movups	%xmm0, 0xa8(%rbp)
141e4dd5d: 0f 10 05 b4 49 d7 00        	movups	0xd749b4(%rip), %xmm0   # 0x142bc2718
141e4dd64: 0f 11 85 c8 00 00 00        	movups	%xmm0, 0xc8(%rbp)
141e4dd6b: 0f 10 05 96 49 d7 00        	movups	0xd74996(%rip), %xmm0   # 0x142bc2708
141e4dd72: 0f 11 85 b8 00 00 00        	movups	%xmm0, 0xb8(%rbp)
141e4dd79: c7 85 7d 01 00 00 04 07 07 07       	movl	$0x7070704, 0x17d(%rbp) # imm = 0x7070704
141e4dd83: 66 c7 85 81 01 00 00 09 04  	movw	$0x409, 0x181(%rbp)     # imm = 0x409
141e4dd8c: 0f 28 05 dd 35 d6 00        	movaps	0xd635dd(%rip), %xmm0   # 0x142bb1370
141e4dd93: 0f 11 85 d8 00 00 00        	movups	%xmm0, 0xd8(%rbp)
141e4dd9a: c6 85 7b 01 00 00 01        	movb	$0x1, 0x17b(%rbp)
141e4dda1: c6 85 7a 01 00 00 00        	movb	$0x0, 0x17a(%rbp)
141e4dda8: 48 c7 85 58 01 00 00 02 00 00 00    	movq	$0x2, 0x158(%rbp)
141e4ddb3: f2 0f 10 05 f5 67 ae 00     	movsd	0xae67f5(%rip), %xmm0   # 0x1429345b0
141e4ddbb: f2 0f 11 85 70 01 00 00     	movsd	%xmm0, 0x170(%rbp)
141e4ddc3: 48 8b 05 c6 48 d7 00        	movq	0xd748c6(%rip), %rax    # 0x142bc2690
141e4ddca: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
141e4ddce: 0f 10 05 ab 48 d7 00        	movups	0xd748ab(%rip), %xmm0   # 0x142bc2680
141e4ddd5: 0f 29 45 30                 	movaps	%xmm0, 0x30(%rbp)
141e4ddd9: 48 8b 05 c8 48 d7 00        	movq	0xd748c8(%rip), %rax    # 0x142bc26a8
141e4dde0: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
141e4dde4: 0f 10 05 ad 48 d7 00        	movups	0xd748ad(%rip), %xmm0   # 0x142bc2698
141e4ddeb: 0f 11 45 48                 	movups	%xmm0, 0x48(%rbp)
141e4ddef: 48 8b 05 ca 48 d7 00        	movq	0xd748ca(%rip), %rax    # 0x142bc26c0
141e4ddf6: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
141e4ddfa: 0f 10 05 af 48 d7 00        	movups	0xd748af(%rip), %xmm0   # 0x142bc26b0
141e4de01: 0f 29 45 60                 	movaps	%xmm0, 0x60(%rbp)
141e4de05: 48 8b 05 cc 48 d7 00        	movq	0xd748cc(%rip), %rax    # 0x142bc26d8
141e4de0c: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
141e4de13: 0f 10 05 ae 48 d7 00        	movups	0xd748ae(%rip), %xmm0   # 0x142bc26c8
141e4de1a: 0f 11 45 78                 	movups	%xmm0, 0x78(%rbp)
141e4de1e: c6 85 7c 01 00 00 00        	movb	$0x0, 0x17c(%rbp)
141e4de25: 0f 57 c0                    	xorps	%xmm0, %xmm0
141e4de28: 0f 29 85 60 01 00 00        	movaps	%xmm0, 0x160(%rbp)
141e4de2f: c6 85 34 02 00 00 01        	movb	$0x1, 0x234(%rbp)
141e4de36: 48 8d 8d b0 01 00 00        	leaq	0x1b0(%rbp), %rcx
141e4de3d: 4c 8d 45 30                 	leaq	0x30(%rbp), %r8
141e4de41: 48 89 fa                    	movq	%rdi, %rdx
141e4de44: e8 b7 97 19 ff              	callq	0x140fe7600 <.text+0xfe6600>
141e4de49: 83 bd b0 01 00 00 04        	cmpl	$0x4, 0x1b0(%rbp)
141e4de50: 0f 85 db 0e 00 00           	jne	0x141e4ed31 <.text+0x1e4dd31>
141e4de56: 44 8b bd b4 01 00 00        	movl	0x1b4(%rbp), %r15d
141e4de5d: 44 8b a5 b8 01 00 00        	movl	0x1b8(%rbp), %r12d
141e4de64: c6 85 34 02 00 00 01        	movb	$0x1, 0x234(%rbp)
141e4de6b: 44 89 64 24 28              	movl	%r12d, 0x28(%rsp)
141e4de70: 44 89 7c 24 20              	movl	%r15d, 0x20(%rsp)
141e4de75: 48 8d 8d d0 01 00 00        	leaq	0x1d0(%rbp), %rcx
141e4de7c: 48 89 fa                    	movq	%rdi, %rdx
141e4de7f: 45 89 f0                    	movl	%r14d, %r8d
141e4de82: 41 89 d9                    	movl	%ebx, %r9d
141e4de85: e8 06 99 19 ff              	callq	0x140fe7790 <.text+0xfe6790>
141e4de8a: 83 bd d0 01 00 00 04        	cmpl	$0x4, 0x1d0(%rbp)
141e4de91: 0f 85 93 0f 00 00           	jne	0x141e4ee2a <.text+0x1e4de2a>
141e4de97: 48 8b 8d 90 01 00 00        	movq	0x190(%rbp), %rcx
141e4de9e: 48 8b 95 98 01 00 00        	movq	0x198(%rbp), %rdx
141e4dea5: 48 8b 85 a0 01 00 00        	movq	0x1a0(%rbp), %rax
141e4deac: 49 89 c6                    	movq	%rax, %r14
141e4deaf: 49 c1 e6 06                 	shlq	$0x6, %r14
141e4deb3: 4e 8d 04 32                 	leaq	(%rdx,%r14), %r8
141e4deb7: 48 89 55 30                 	movq	%rdx, 0x30(%rbp)
141e4debb: 48 89 55 38                 	movq	%rdx, 0x38(%rbp)
141e4debf: 48 89 8d 00 02 00 00        	movq	%rcx, 0x200(%rbp)
141e4dec6: 48 89 4d 40                 	movq	%rcx, 0x40(%rbp)
141e4deca: 4c 89 45 48                 	movq	%r8, 0x48(%rbp)
141e4dece: 48 89 95 f8 01 00 00        	movq	%rdx, 0x1f8(%rbp)
141e4ded5: 49 89 d1                    	movq	%rdx, %r9
141e4ded8: 48 85 c0                    	testq	%rax, %rax
141e4dedb: 0f 84 f8 07 00 00           	je	0x141e4e6d9 <.text+0x1e4d6d9>
141e4dee1: 4c 89 85 18 02 00 00        	movq	%r8, 0x218(%rbp)
141e4dee8: 4c 8d 6d e9                 	leaq	-0x17(%rbp), %r13
141e4deec: 48 8b 85 f8 01 00 00        	movq	0x1f8(%rbp), %rax
141e4def3: 4c 8d 48 40                 	leaq	0x40(%rax), %r9
141e4def7: 48 8d 5d b0                 	leaq	-0x50(%rbp), %rbx
141e4defb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
141e4df00: 41 0f b6 41 f8              	movzbl	-0x8(%r9), %eax
141e4df05: 3c 02                       	cmpb	$0x2, %al
141e4df07: 0f 84 c1 07 00 00           	je	0x141e4e6ce <.text+0x1e4d6ce>
141e4df0d: 49 8d 49 c0                 	leaq	-0x40(%r9), %rcx
141e4df11: 48 8b 51 30                 	movq	0x30(%rcx), %rdx
141e4df15: 48 89 55 e0                 	movq	%rdx, -0x20(%rbp)
141e4df19: 0f 10 01                    	movups	(%rcx), %xmm0
141e4df1c: 0f 10 49 10                 	movups	0x10(%rcx), %xmm1
141e4df20: 0f 10 51 20                 	movups	0x20(%rcx), %xmm2
141e4df24: 0f 29 55 d0                 	movaps	%xmm2, -0x30(%rbp)
141e4df28: 0f 29 4d c0                 	movaps	%xmm1, -0x40(%rbp)
141e4df2c: 0f 29 45 b0                 	movaps	%xmm0, -0x50(%rbp)
141e4df30: 88 45 e8                    	movb	%al, -0x18(%rbp)
141e4df33: 41 8b 41 f9                 	movl	-0x7(%r9), %eax
141e4df37: 41 8b 49 fc                 	movl	-0x4(%r9), %ecx
141e4df3b: 41 89 4d 03                 	movl	%ecx, 0x3(%r13)
141e4df3f: 41 89 45 00                 	movl	%eax, (%r13)
141e4df43: 48 8b 85 28 02 00 00        	movq	0x228(%rbp), %rax
141e4df4a: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
141e4df4f: 48 89 74 24 20              	movq	%rsi, 0x20(%rsp)
141e4df54: 48 89 f9                    	movq	%rdi, %rcx
141e4df57: 44 89 fa                    	movl	%r15d, %edx
141e4df5a: 45 89 e0                    	movl	%r12d, %r8d
141e4df5d: 4c 89 8d 08 02 00 00        	movq	%r9, 0x208(%rbp)
141e4df64: 49 89 d9                    	movq	%rbx, %r9
141e4df67: e8 d4 fc ff ff              	callq	0x141e4dc40 <.text+0x1e4cc40>
141e4df6c: 4c 8b 8d 08 02 00 00        	movq	0x208(%rbp), %r9
141e4df73: 49 83 c1 40                 	addq	$0x40, %r9
141e4df77: 49 83 c6 c0                 	addq	$-0x40, %r14
141e4df7b: 75 83                       	jne	0x141e4df00 <.text+0x1e4cf00>
141e4df7d: 4c 8b 85 18 02 00 00        	movq	0x218(%rbp), %r8
141e4df84: 4d 89 c1                    	movq	%r8, %r9
141e4df87: e9 49 07 00 00              	jmp	0x141e4e6d5 <.text+0x1e4d6d5>
141e4df8c: 48 8b 8d 10 02 00 00        	movq	0x210(%rbp), %rcx
141e4df93: 48 8b 41 18                 	movq	0x18(%rcx), %rax
141e4df97: 48 89 85 a0 01 00 00        	movq	%rax, 0x1a0(%rbp)
141e4df9e: 0f 10 41 08                 	movups	0x8(%rcx), %xmm0
141e4dfa2: 0f 29 85 90 01 00 00        	movaps	%xmm0, 0x190(%rbp)
141e4dfa9: 66 c7 85 78 01 00 00 00 00  	movw	$0x0, 0x178(%rbp)
141e4dfb2: 0f 10 05 7f 47 d7 00        	movups	0xd7477f(%rip), %xmm0   # 0x142bc2738
141e4dfb9: 0f 11 85 e8 00 00 00        	movups	%xmm0, 0xe8(%rbp)
141e4dfc0: 0f 10 05 81 47 d7 00        	movups	0xd74781(%rip), %xmm0   # 0x142bc2748
141e4dfc7: 0f 11 85 f8 00 00 00        	movups	%xmm0, 0xf8(%rbp)
141e4dfce: 0f 10 05 83 47 d7 00        	movups	0xd74783(%rip), %xmm0   # 0x142bc2758
141e4dfd5: 0f 11 85 08 01 00 00        	movups	%xmm0, 0x108(%rbp)
141e4dfdc: 0f 10 05 85 47 d7 00        	movups	0xd74785(%rip), %xmm0   # 0x142bc2768
141e4dfe3: 0f 11 85 18 01 00 00        	movups	%xmm0, 0x118(%rbp)
141e4dfea: 0f 10 05 87 47 d7 00        	movups	0xd74787(%rip), %xmm0   # 0x142bc2778
141e4dff1: 0f 11 85 28 01 00 00        	movups	%xmm0, 0x128(%rbp)
141e4dff8: 48 c7 85 90 00 00 00 00 00 00 00    	movq	$0x0, 0x90(%rbp)
141e4e003: 0f 10 05 7e 47 d7 00        	movups	0xd7477e(%rip), %xmm0   # 0x142bc2788
141e4e00a: 0f 11 85 38 01 00 00        	movups	%xmm0, 0x138(%rbp)
141e4e011: 0f 10 05 80 47 d7 00        	movups	0xd74780(%rip), %xmm0   # 0x142bc2798
141e4e018: 0f 11 85 48 01 00 00        	movups	%xmm0, 0x148(%rbp)
141e4e01f: 0f 10 05 c2 46 d7 00        	movups	0xd746c2(%rip), %xmm0   # 0x142bc26e8
141e4e026: 0f 11 85 98 00 00 00        	movups	%xmm0, 0x98(%rbp)
141e4e02d: 0f 10 05 c4 46 d7 00        	movups	0xd746c4(%rip), %xmm0   # 0x142bc26f8
141e4e034: 0f 11 85 a8 00 00 00        	movups	%xmm0, 0xa8(%rbp)
141e4e03b: 0f 10 05 d6 46 d7 00        	movups	0xd746d6(%rip), %xmm0   # 0x142bc2718
141e4e042: 0f 11 85 c8 00 00 00        	movups	%xmm0, 0xc8(%rbp)
141e4e049: 0f 10 05 b8 46 d7 00        	movups	0xd746b8(%rip), %xmm0   # 0x142bc2708
141e4e050: 0f 11 85 b8 00 00 00        	movups	%xmm0, 0xb8(%rbp)
141e4e057: c7 85 7d 01 00 00 04 07 07 07       	movl	$0x7070704, 0x17d(%rbp) # imm = 0x7070704
141e4e061: 66 c7 85 81 01 00 00 09 04  	movw	$0x409, 0x181(%rbp)     # imm = 0x409
141e4e06a: 0f 28 05 ff 32 d6 00        	movaps	0xd632ff(%rip), %xmm0   # 0x142bb1370
141e4e071: 0f 11 85 d8 00 00 00        	movups	%xmm0, 0xd8(%rbp)
141e4e078: c6 85 7b 01 00 00 00        	movb	$0x0, 0x17b(%rbp)
141e4e07f: c6 85 7a 01 00 00 00        	movb	$0x0, 0x17a(%rbp)
141e4e086: 48 c7 85 58 01 00 00 02 00 00 00    	movq	$0x2, 0x158(%rbp)
141e4e091: f2 0f 10 05 17 65 ae 00     	movsd	0xae6517(%rip), %xmm0   # 0x1429345b0
141e4e099: f2 0f 11 85 70 01 00 00     	movsd	%xmm0, 0x170(%rbp)
141e4e0a1: 48 8b 05 e8 45 d7 00        	movq	0xd745e8(%rip), %rax    # 0x142bc2690
141e4e0a8: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
141e4e0ac: 0f 10 05 cd 45 d7 00        	movups	0xd745cd(%rip), %xmm0   # 0x142bc2680
141e4e0b3: 0f 29 45 30                 	movaps	%xmm0, 0x30(%rbp)
141e4e0b7: 48 8b 05 ea 45 d7 00        	movq	0xd745ea(%rip), %rax    # 0x142bc26a8
141e4e0be: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
141e4e0c2: 0f 10 05 cf 45 d7 00        	movups	0xd745cf(%rip), %xmm0   # 0x142bc2698
141e4e0c9: 0f 11 45 48                 	movups	%xmm0, 0x48(%rbp)
141e4e0cd: 48 8b 05 ec 45 d7 00        	movq	0xd745ec(%rip), %rax    # 0x142bc26c0
141e4e0d4: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
141e4e0d8: 0f 10 05 d1 45 d7 00        	movups	0xd745d1(%rip), %xmm0   # 0x142bc26b0
141e4e0df: 0f 29 45 60                 	movaps	%xmm0, 0x60(%rbp)
141e4e0e3: 48 8b 05 ee 45 d7 00        	movq	0xd745ee(%rip), %rax    # 0x142bc26d8
141e4e0ea: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
141e4e0f1: 0f 10 05 d0 45 d7 00        	movups	0xd745d0(%rip), %xmm0   # 0x142bc26c8
141e4e0f8: 0f 11 45 78                 	movups	%xmm0, 0x78(%rbp)
141e4e0fc: c6 85 7c 01 00 00 00        	movb	$0x0, 0x17c(%rbp)
141e4e103: 0f 57 c0                    	xorps	%xmm0, %xmm0
141e4e106: 0f 29 85 60 01 00 00        	movaps	%xmm0, 0x160(%rbp)
141e4e10d: c6 85 35 02 00 00 01        	movb	$0x1, 0x235(%rbp)
141e4e114: 48 8d 8d b0 01 00 00        	leaq	0x1b0(%rbp), %rcx
141e4e11b: 4c 8d 45 30                 	leaq	0x30(%rbp), %r8
141e4e11f: 48 89 fa                    	movq	%rdi, %rdx
141e4e122: e8 d9 94 19 ff              	callq	0x140fe7600 <.text+0xfe6600>
141e4e127: 83 bd b0 01 00 00 04        	cmpl	$0x4, 0x1b0(%rbp)
141e4e12e: 0f 85 50 0c 00 00           	jne	0x141e4ed84 <.text+0x1e4dd84>
141e4e134: 44 8b bd b4 01 00 00        	movl	0x1b4(%rbp), %r15d
141e4e13b: 44 8b a5 b8 01 00 00        	movl	0x1b8(%rbp), %r12d
141e4e142: c6 85 35 02 00 00 01        	movb	$0x1, 0x235(%rbp)
141e4e149: 44 89 64 24 28              	movl	%r12d, 0x28(%rsp)
141e4e14e: 44 89 7c 24 20              	movl	%r15d, 0x20(%rsp)
141e4e153: 48 8d 8d d0 01 00 00        	leaq	0x1d0(%rbp), %rcx
141e4e15a: 48 89 fa                    	movq	%rdi, %rdx
141e4e15d: 45 89 f0                    	movl	%r14d, %r8d
141e4e160: 41 89 d9                    	movl	%ebx, %r9d
141e4e163: e8 28 96 19 ff              	callq	0x140fe7790 <.text+0xfe6790>
141e4e168: 83 bd d0 01 00 00 04        	cmpl	$0x4, 0x1d0(%rbp)
141e4e16f: 0f 85 ff 0c 00 00           	jne	0x141e4ee74 <.text+0x1e4de74>
141e4e175: 48 8b 8d 90 01 00 00        	movq	0x190(%rbp), %rcx
141e4e17c: 48 8b 95 98 01 00 00        	movq	0x198(%rbp), %rdx
141e4e183: 48 8b 85 a0 01 00 00        	movq	0x1a0(%rbp), %rax
141e4e18a: 49 89 c6                    	movq	%rax, %r14
141e4e18d: 49 c1 e6 06                 	shlq	$0x6, %r14
141e4e191: 4e 8d 04 32                 	leaq	(%rdx,%r14), %r8
141e4e195: 48 89 55 30                 	movq	%rdx, 0x30(%rbp)
141e4e199: 48 89 55 38                 	movq	%rdx, 0x38(%rbp)
141e4e19d: 48 89 8d 00 02 00 00        	movq	%rcx, 0x200(%rbp)
141e4e1a4: 48 89 4d 40                 	movq	%rcx, 0x40(%rbp)
141e4e1a8: 4c 89 45 48                 	movq	%r8, 0x48(%rbp)
141e4e1ac: 48 89 95 f8 01 00 00        	movq	%rdx, 0x1f8(%rbp)
141e4e1b3: 49 89 d1                    	movq	%rdx, %r9
141e4e1b6: 48 85 c0                    	testq	%rax, %rax
141e4e1b9: 0f 84 74 05 00 00           	je	0x141e4e733 <.text+0x1e4d733>
141e4e1bf: 4c 89 85 18 02 00 00        	movq	%r8, 0x218(%rbp)
141e4e1c6: 4c 8d 6d 29                 	leaq	0x29(%rbp), %r13
141e4e1ca: 48 8b 85 f8 01 00 00        	movq	0x1f8(%rbp), %rax
141e4e1d1: 4c 8d 48 40                 	leaq	0x40(%rax), %r9
141e4e1d5: 48 8d 5d f0                 	leaq	-0x10(%rbp), %rbx
141e4e1d9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
141e4e1e0: 41 0f b6 41 f8              	movzbl	-0x8(%r9), %eax
141e4e1e5: 3c 02                       	cmpb	$0x2, %al
141e4e1e7: 0f 84 3b 05 00 00           	je	0x141e4e728 <.text+0x1e4d728>
141e4e1ed: 49 8d 49 c0                 	leaq	-0x40(%r9), %rcx
141e4e1f1: 48 8b 51 30                 	movq	0x30(%rcx), %rdx
141e4e1f5: 48 89 55 20                 	movq	%rdx, 0x20(%rbp)
141e4e1f9: 0f 10 01                    	movups	(%rcx), %xmm0
141e4e1fc: 0f 10 49 10                 	movups	0x10(%rcx), %xmm1
141e4e200: 0f 10 51 20                 	movups	0x20(%rcx), %xmm2
141e4e204: 0f 29 55 10                 	movaps	%xmm2, 0x10(%rbp)
141e4e208: 0f 29 4d 00                 	movaps	%xmm1, (%rbp)
141e4e20c: 0f 29 45 f0                 	movaps	%xmm0, -0x10(%rbp)
141e4e210: 88 45 28                    	movb	%al, 0x28(%rbp)
141e4e213: 41 8b 41 f9                 	movl	-0x7(%r9), %eax
141e4e217: 41 8b 49 fc                 	movl	-0x4(%r9), %ecx
141e4e21b: 41 89 4d 03                 	movl	%ecx, 0x3(%r13)
141e4e21f: 41 89 45 00                 	movl	%eax, (%r13)
141e4e223: 48 8b 85 28 02 00 00        	movq	0x228(%rbp), %rax
141e4e22a: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
141e4e22f: 48 89 74 24 20              	movq	%rsi, 0x20(%rsp)
141e4e234: 48 89 f9                    	movq	%rdi, %rcx
141e4e237: 44 89 fa                    	movl	%r15d, %edx
141e4e23a: 45 89 e0                    	movl	%r12d, %r8d
141e4e23d: 4c 89 8d 08 02 00 00        	movq	%r9, 0x208(%rbp)
141e4e244: 49 89 d9                    	movq	%rbx, %r9
141e4e247: e8 f4 f9 ff ff              	callq	0x141e4dc40 <.text+0x1e4cc40>
141e4e24c: 4c 8b 8d 08 02 00 00        	movq	0x208(%rbp), %r9
141e4e253: 49 83 c1 40                 	addq	$0x40, %r9
141e4e257: 49 83 c6 c0                 	addq	$-0x40, %r14
141e4e25b: 75 83                       	jne	0x141e4e1e0 <.text+0x1e4d1e0>
141e4e25d: 4c 8b 85 18 02 00 00        	movq	0x218(%rbp), %r8
141e4e264: 4d 89 c1                    	movq	%r8, %r9
141e4e267: e9 c3 04 00 00              	jmp	0x141e4e72f <.text+0x1e4d72f>
141e4e26c: 66 c7 85 78 01 00 00 00 00  	movw	$0x0, 0x178(%rbp)
141e4e275: 0f 10 05 bc 44 d7 00        	movups	0xd744bc(%rip), %xmm0   # 0x142bc2738
141e4e27c: 0f 11 85 e8 00 00 00        	movups	%xmm0, 0xe8(%rbp)
141e4e283: 0f 10 05 be 44 d7 00        	movups	0xd744be(%rip), %xmm0   # 0x142bc2748
141e4e28a: 0f 11 85 f8 00 00 00        	movups	%xmm0, 0xf8(%rbp)
141e4e291: 0f 10 05 c0 44 d7 00        	movups	0xd744c0(%rip), %xmm0   # 0x142bc2758
141e4e298: 0f 11 85 08 01 00 00        	movups	%xmm0, 0x108(%rbp)
141e4e29f: 0f 10 05 c2 44 d7 00        	movups	0xd744c2(%rip), %xmm0   # 0x142bc2768
141e4e2a6: 0f 11 85 18 01 00 00        	movups	%xmm0, 0x118(%rbp)
141e4e2ad: 0f 10 05 c4 44 d7 00        	movups	0xd744c4(%rip), %xmm0   # 0x142bc2778
141e4e2b4: 0f 11 85 28 01 00 00        	movups	%xmm0, 0x128(%rbp)
141e4e2bb: 48 c7 85 90 00 00 00 00 00 00 00    	movq	$0x0, 0x90(%rbp)
141e4e2c6: 0f 10 05 bb 44 d7 00        	movups	0xd744bb(%rip), %xmm0   # 0x142bc2788
141e4e2cd: 0f 11 85 38 01 00 00        	movups	%xmm0, 0x138(%rbp)
141e4e2d4: 0f 10 05 bd 44 d7 00        	movups	0xd744bd(%rip), %xmm0   # 0x142bc2798
141e4e2db: 0f 11 85 48 01 00 00        	movups	%xmm0, 0x148(%rbp)
141e4e2e2: 0f 10 05 0f 44 d7 00        	movups	0xd7440f(%rip), %xmm0   # 0x142bc26f8
141e4e2e9: 0f 11 85 a8 00 00 00        	movups	%xmm0, 0xa8(%rbp)
141e4e2f0: 0f 10 05 f1 43 d7 00        	movups	0xd743f1(%rip), %xmm0   # 0x142bc26e8
141e4e2f7: 0f 11 85 98 00 00 00        	movups	%xmm0, 0x98(%rbp)
141e4e2fe: 0f 10 05 13 44 d7 00        	movups	0xd74413(%rip), %xmm0   # 0x142bc2718
141e4e305: 0f 11 85 c8 00 00 00        	movups	%xmm0, 0xc8(%rbp)
141e4e30c: 0f 10 05 f5 43 d7 00        	movups	0xd743f5(%rip), %xmm0   # 0x142bc2708
141e4e313: 0f 11 85 b8 00 00 00        	movups	%xmm0, 0xb8(%rbp)
141e4e31a: c7 85 7d 01 00 00 04 07 07 07       	movl	$0x7070704, 0x17d(%rbp) # imm = 0x7070704
141e4e324: 66 c7 85 81 01 00 00 09 04  	movw	$0x409, 0x181(%rbp)     # imm = 0x409
141e4e32d: 0f 10 05 f4 43 d7 00        	movups	0xd743f4(%rip), %xmm0   # 0x142bc2728
141e4e334: 0f 11 85 d8 00 00 00        	movups	%xmm0, 0xd8(%rbp)
141e4e33b: 66 c7 85 7a 01 00 00 00 00  	movw	$0x0, 0x17a(%rbp)
141e4e344: 48 c7 85 58 01 00 00 02 00 00 00    	movq	$0x2, 0x158(%rbp)
141e4e34f: f2 0f 10 05 59 62 ae 00     	movsd	0xae6259(%rip), %xmm0   # 0x1429345b0
141e4e357: f2 0f 11 85 70 01 00 00     	movsd	%xmm0, 0x170(%rbp)
141e4e35f: 0f 10 05 1a 43 d7 00        	movups	0xd7431a(%rip), %xmm0   # 0x142bc2680
141e4e366: 0f 29 45 30                 	movaps	%xmm0, 0x30(%rbp)
141e4e36a: 48 8b 05 1f 43 d7 00        	movq	0xd7431f(%rip), %rax    # 0x142bc2690
141e4e371: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
141e4e375: 48 8b 05 2c 43 d7 00        	movq	0xd7432c(%rip), %rax    # 0x142bc26a8
141e4e37c: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
141e4e380: 0f 10 05 11 43 d7 00        	movups	0xd74311(%rip), %xmm0   # 0x142bc2698
141e4e387: 0f 11 45 48                 	movups	%xmm0, 0x48(%rbp)
141e4e38b: 48 8b 05 2e 43 d7 00        	movq	0xd7432e(%rip), %rax    # 0x142bc26c0
141e4e392: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
141e4e396: 0f 10 05 13 43 d7 00        	movups	0xd74313(%rip), %xmm0   # 0x142bc26b0
141e4e39d: 0f 29 45 60                 	movaps	%xmm0, 0x60(%rbp)
141e4e3a1: 48 8b 05 30 43 d7 00        	movq	0xd74330(%rip), %rax    # 0x142bc26d8
141e4e3a8: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
141e4e3af: 0f 10 05 12 43 d7 00        	movups	0xd74312(%rip), %xmm0   # 0x142bc26c8
141e4e3b6: 0f 11 45 78                 	movups	%xmm0, 0x78(%rbp)
141e4e3ba: c6 85 7c 01 00 00 00        	movb	$0x0, 0x17c(%rbp)
141e4e3c1: 0f 57 c0                    	xorps	%xmm0, %xmm0
141e4e3c4: 0f 29 85 60 01 00 00        	movaps	%xmm0, 0x160(%rbp)
141e4e3cb: c6 85 37 02 00 00 01        	movb	$0x1, 0x237(%rbp)
141e4e3d2: 48 8d 8d b0 01 00 00        	leaq	0x1b0(%rbp), %rcx
141e4e3d9: 4c 8d 45 30                 	leaq	0x30(%rbp), %r8
141e4e3dd: 48 89 fa                    	movq	%rdi, %rdx
141e4e3e0: e8 1b 92 19 ff              	callq	0x140fe7600 <.text+0xfe6600>
141e4e3e5: 83 bd b0 01 00 00 04        	cmpl	$0x4, 0x1b0(%rbp)
141e4e3ec: 0f 85 e5 09 00 00           	jne	0x141e4edd7 <.text+0x1e4ddd7>
141e4e3f2: 44 8b a5 b4 01 00 00        	movl	0x1b4(%rbp), %r12d
141e4e3f9: 44 8b bd b8 01 00 00        	movl	0x1b8(%rbp), %r15d
141e4e400: c6 85 37 02 00 00 01        	movb	$0x1, 0x237(%rbp)
141e4e407: 44 89 7c 24 28              	movl	%r15d, 0x28(%rsp)
141e4e40c: 44 89 64 24 20              	movl	%r12d, 0x20(%rsp)
141e4e411: 48 8d 8d d0 01 00 00        	leaq	0x1d0(%rbp), %rcx
141e4e418: 48 89 fa                    	movq	%rdi, %rdx
141e4e41b: 45 89 f0                    	movl	%r14d, %r8d
141e4e41e: 41 89 d9                    	movl	%ebx, %r9d
141e4e421: e8 6a 93 19 ff              	callq	0x140fe7790 <.text+0xfe6790>
141e4e426: 83 bd d0 01 00 00 04        	cmpl	$0x4, 0x1d0(%rbp)
141e4e42d: 0f 85 8b 0a 00 00           	jne	0x141e4eebe <.text+0x1e4debe>
141e4e433: 48 8b 85 10 02 00 00        	movq	0x210(%rbp), %rax
141e4e43a: f2 0f 10 70 30              	movsd	0x30(%rax), %xmm6
141e4e43f: 48 8b 58 20                 	movq	0x20(%rax), %rbx
141e4e443: 48 8b 78 28                 	movq	0x28(%rax), %rdi
141e4e447: 0f 57 c0                    	xorps	%xmm0, %xmm0
141e4e44a: 0f 11 45 60                 	movups	%xmm0, 0x60(%rbp)
