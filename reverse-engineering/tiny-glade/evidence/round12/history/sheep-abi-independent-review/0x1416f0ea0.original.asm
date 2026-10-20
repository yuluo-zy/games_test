
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1416f0ea0: 55                          	pushq	%rbp
1416f0ea1: 41 57                       	pushq	%r15
1416f0ea3: 41 56                       	pushq	%r14
1416f0ea5: 41 55                       	pushq	%r13
1416f0ea7: 41 54                       	pushq	%r12
1416f0ea9: 56                          	pushq	%rsi
1416f0eaa: 57                          	pushq	%rdi
1416f0eab: 53                          	pushq	%rbx
1416f0eac: 48 81 ec 68 01 00 00        	subq	$0x168, %rsp            # imm = 0x168
1416f0eb3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1416f0ebb: 48 c7 85 e0 00 00 00 fe ff ff ff    	movq	$-0x2, 0xe0(%rbp)
1416f0ec6: 4c 89 cf                    	movq	%r9, %rdi
1416f0ec9: 4d 89 c6                    	movq	%r8, %r14
1416f0ecc: 48 89 d3                    	movq	%rdx, %rbx
1416f0ecf: 48 89 ce                    	movq	%rcx, %rsi
1416f0ed2: 0f 28 05 f7 e4 3a 01        	movaps	0x13ae4f7(%rip), %xmm0  # 0x142a9f3d0
1416f0ed9: 0f 29 45 b0                 	movaps	%xmm0, -0x50(%rbp)
1416f0edd: f2 0f 10 05 fb e4 3a 01     	movsd	0x13ae4fb(%rip), %xmm0  # 0x142a9f3e0
1416f0ee5: f2 0f 11 45 c0              	movsd	%xmm0, -0x40(%rbp)
1416f0eea: 48 8d 4d 48                 	leaq	0x48(%rbp), %rcx
1416f0eee: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
1416f0ef2: e8 39 46 31 ff              	callq	0x140a05530 <.text+0xa04530>
1416f0ef7: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1416f0f01: 48 89 85 c0 00 00 00        	movq	%rax, 0xc0(%rbp)
1416f0f08: 0f b6 05 fa 3c f4 01        	movzbl	0x1f43cfa(%rip), %eax   # 0x143634c09
1416f0f0f: b9 12 00 00 00              	movl	$0x12, %ecx
1416f0f14: ba 01 00 00 00              	movl	$0x1, %edx
1416f0f19: e8 f2 2c f2 fe              	callq	0x140613c10 <.text+0x612c10>
1416f0f1e: 48 85 c0                    	testq	%rax, %rax
1416f0f21: 0f 84 ff 01 00 00           	je	0x1416f1126 <.text+0x16f0126>
1416f0f27: 0f 10 05 12 f4 59 01        	movups	0x159f412(%rip), %xmm0  # 0x142c90340
1416f0f2e: 0f 11 00                    	movups	%xmm0, (%rax)
1416f0f31: 66 c7 40 10 62 65           	movw	$0x6562, 0x10(%rax)     # imm = 0x6562
1416f0f37: 48 c7 85 c0 00 00 00 12 00 00 00    	movq	$0x12, 0xc0(%rbp)
1416f0f42: 48 89 85 c8 00 00 00        	movq	%rax, 0xc8(%rbp)
1416f0f49: 48 c7 85 d0 00 00 00 12 00 00 00    	movq	$0x12, 0xd0(%rbp)
1416f0f54: 4c 8d 7d b0                 	leaq	-0x50(%rbp), %r15
1416f0f58: 48 8d 55 48                 	leaq	0x48(%rbp), %rdx
1416f0f5c: 41 b8 90 00 00 00           	movl	$0x90, %r8d
1416f0f62: 4c 89 f9                    	movq	%r15, %rcx
1416f0f65: e8 96 2c 23 01              	callq	0x142923c00 <.text+0x2922c00>
1416f0f6a: 48 89 d9                    	movq	%rbx, %rcx
1416f0f6d: 4c 89 fa                    	movq	%r15, %rdx
1416f0f70: e8 eb bc 23 ff              	callq	0x14092cc60 <.text+0x92bc60>
1416f0f75: 48 89 85 d8 00 00 00        	movq	%rax, 0xd8(%rbp)
1416f0f7c: 48 8d 15 cf f3 59 01        	leaq	0x159f3cf(%rip), %rdx   # 0x142c90352
1416f0f83: 41 b8 13 00 00 00           	movl	$0x13, %r8d
1416f0f89: 4c 89 f1                    	movq	%r14, %rcx
1416f0f8c: e8 0f 6b 24 ff              	callq	0x140937aa0 <.text+0x936aa0>
1416f0f91: a8 01                       	testb	$0x1, %al
1416f0f93: 0f 84 a5 01 00 00           	je	0x1416f113e <.text+0x16f013e>
1416f0f99: 49 89 d7                    	movq	%rdx, %r15
1416f0f9c: 48 8d 4d 48                 	leaq	0x48(%rbp), %rcx
1416f0fa0: 48 89 fa                    	movq	%rdi, %rdx
1416f0fa3: e8 f8 36 62 ff              	callq	0x140d146a0 <.text+0xd136a0>
1416f0fa8: 4c 8b 65 50                 	movq	0x50(%rbp), %r12
1416f0fac: 4d 85 e4                    	testq	%r12, %r12
1416f0faf: 75 04                       	jne	0x1416f0fb5 <.text+0x16effb5>
1416f0fb1: 4c 8b 65 58                 	movq	0x58(%rbp), %r12
1416f0fb5: 48 8b 5d 68                 	movq	0x68(%rbp), %rbx
1416f0fb9: 49 8b 04 24                 	movq	(%r12), %rax
1416f0fbd: 4d 8b 6c 24 10              	movq	0x10(%r12), %r13
1416f0fc2: 4c 29 e8                    	subq	%r13, %rax
1416f0fc5: 48 83 f8 2f                 	cmpq	$0x2f, %rax
1416f0fc9: 0f 86 0d 01 00 00           	jbe	0x1416f10dc <.text+0x16f00dc>
1416f0fcf: 49 8b 44 24 08              	movq	0x8(%r12), %rax
1416f0fd4: 48 8d 0d a5 c0 be ff        	leaq	-0x413f5b(%rip), %rcx   # 0x1412dd080 <.text+0x12dc080>
1416f0fdb: 4a 89 0c 28                 	movq	%rcx, (%rax,%r13)
1416f0fdf: 48 8d 0d 4a 3d e7 ff        	leaq	-0x18c2b6(%rip), %rcx   # 0x141564d30 <.text+0x1563d30>
1416f0fe6: 4a 89 4c 28 08              	movq	%rcx, 0x8(%rax,%r13)
1416f0feb: 4a 89 5c 28 10              	movq	%rbx, 0x10(%rax,%r13)
1416f0ff0: 48 8b 8d d8 00 00 00        	movq	0xd8(%rbp), %rcx
1416f0ff7: 4a 89 4c 28 18              	movq	%rcx, 0x18(%rax,%r13)
1416f0ffc: 4e 89 7c 28 20              	movq	%r15, 0x20(%rax,%r13)
1416f1001: 42 c6 44 28 28 00           	movb	$0x0, 0x28(%rax,%r13)
1416f1007: 49 83 c5 30                 	addq	$0x30, %r13
1416f100b: 4d 89 6c 24 10              	movq	%r13, 0x10(%r12)
1416f1010: 48 8d 15 3b f3 59 01        	leaq	0x159f33b(%rip), %rdx   # 0x142c90352
1416f1017: 41 b8 13 00 00 00           	movl	$0x13, %r8d
1416f101d: 4c 89 f1                    	movq	%r14, %rcx
1416f1020: e8 7b 6a 24 ff              	callq	0x140937aa0 <.text+0x936aa0>
1416f1025: a8 01                       	testb	$0x1, %al
1416f1027: 0f 84 1d 01 00 00           	je	0x1416f114a <.text+0x16f014a>
1416f102d: 49 89 d6                    	movq	%rdx, %r14
1416f1030: 48 8d 4d 48                 	leaq	0x48(%rbp), %rcx
1416f1034: 48 89 fa                    	movq	%rdi, %rdx
1416f1037: e8 64 36 62 ff              	callq	0x140d146a0 <.text+0xd136a0>
1416f103c: 48 8b 7d 50                 	movq	0x50(%rbp), %rdi
1416f1040: 48 85 ff                    	testq	%rdi, %rdi
1416f1043: 49 bd 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %r13 # imm = 0x8000000000000000
1416f104d: 75 04                       	jne	0x1416f1053 <.text+0x16f0053>
1416f104f: 48 8b 7d 58                 	movq	0x58(%rbp), %rdi
1416f1053: 4c 8b 65 68                 	movq	0x68(%rbp), %r12
1416f1057: 48 8b 07                    	movq	(%rdi), %rax
1416f105a: 4c 8b 7f 10                 	movq	0x10(%rdi), %r15
1416f105e: 4c 29 f8                    	subq	%r15, %rax
1416f1061: 48 83 f8 2f                 	cmpq	$0x2f, %rax
1416f1065: 0f 86 96 00 00 00           	jbe	0x1416f1101 <.text+0x16f0101>
1416f106b: 48 8b 47 08                 	movq	0x8(%rdi), %rax
1416f106f: 48 8d 0d ca 9c be ff        	leaq	-0x416336(%rip), %rcx   # 0x1412dad40 <.text+0x12d9d40>
1416f1076: 4a 89 0c 38                 	movq	%rcx, (%rax,%r15)
1416f107a: 48 8d 0d af 3c e7 ff        	leaq	-0x18c351(%rip), %rcx   # 0x141564d30 <.text+0x1563d30>
1416f1081: 4a 89 4c 38 08              	movq	%rcx, 0x8(%rax,%r15)
1416f1086: 4e 89 64 38 10              	movq	%r12, 0x10(%rax,%r15)
1416f108b: 48 8b 8d d8 00 00 00        	movq	0xd8(%rbp), %rcx
1416f1092: 4a 89 4c 38 18              	movq	%rcx, 0x18(%rax,%r15)
1416f1097: 4e 89 74 38 20              	movq	%r14, 0x20(%rax,%r15)
1416f109c: 42 c6 44 38 28 00           	movb	$0x0, 0x28(%rax,%r15)
1416f10a2: 49 83 c7 30                 	addq	$0x30, %r15
1416f10a6: 4c 89 7f 10                 	movq	%r15, 0x10(%rdi)
1416f10aa: 48 c7 46 40 00 00 00 00     	movq	$0x0, 0x40(%rsi)
1416f10b2: c6 46 48 00                 	movb	$0x0, 0x48(%rsi)
1416f10b6: 48 89 5e 30                 	movq	%rbx, 0x30(%rsi)
1416f10ba: 4c 89 66 38                 	movq	%r12, 0x38(%rsi)
1416f10be: 4c 89 2e                    	movq	%r13, (%rsi)
1416f10c1: 4c 89 6e 18                 	movq	%r13, 0x18(%rsi)
1416f10c5: 48 89 f0                    	movq	%rsi, %rax
1416f10c8: 48 81 c4 68 01 00 00        	addq	$0x168, %rsp            # imm = 0x168
1416f10cf: 5b                          	popq	%rbx
1416f10d0: 5f                          	popq	%rdi
1416f10d1: 5e                          	popq	%rsi
1416f10d2: 41 5c                       	popq	%r12
1416f10d4: 41 5d                       	popq	%r13
1416f10d6: 41 5e                       	popq	%r14
1416f10d8: 41 5f                       	popq	%r15
1416f10da: 5d                          	popq	%rbp
1416f10db: c3                          	retq
1416f10dc: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1416f10e5: 41 b8 30 00 00 00           	movl	$0x30, %r8d
1416f10eb: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1416f10f1: 4c 89 e1                    	movq	%r12, %rcx
1416f10f4: 4c 89 ea                    	movq	%r13, %rdx
1416f10f7: e8 64 89 1b 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1416f10fc: e9 ce fe ff ff              	jmp	0x1416f0fcf <.text+0x16effcf>
1416f1101: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1416f110a: 41 b8 30 00 00 00           	movl	$0x30, %r8d
1416f1110: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1416f1116: 48 89 f9                    	movq	%rdi, %rcx
1416f1119: 4c 89 fa                    	movq	%r15, %rdx
1416f111c: e8 3f 89 1b 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1416f1121: e9 45 ff ff ff              	jmp	0x1416f106b <.text+0x16f006b>
1416f1126: 4c 8d 05 8b de 53 01        	leaq	0x153de8b(%rip), %r8    # 0x142c2efb8
1416f112d: b9 01 00 00 00              	movl	$0x1, %ecx
1416f1132: ba 12 00 00 00              	movl	$0x12, %edx
1416f1137: e8 a7 7e 1e 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1416f113c: 0f 0b                       	ud2
1416f113e: 48 8d 0d 5b f2 59 01        	leaq	0x159f25b(%rip), %rcx   # 0x142c903a0
1416f1145: e8 c6 81 1e 01              	callq	0x1428d9310 <.text+0x28d8310>
1416f114a: 48 8d 0d 67 f2 59 01        	leaq	0x159f267(%rip), %rcx   # 0x142c903b8
1416f1151: e8 ba 81 1e 01              	callq	0x1428d9310 <.text+0x28d8310>
1416f1156: cc                          	int3
1416f1157: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1416f1160: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1416f1165: 55                          	pushq	%rbp
1416f1166: 41 57                       	pushq	%r15
1416f1168: 41 56                       	pushq	%r14
1416f116a: 41 55                       	pushq	%r13
1416f116c: 41 54                       	pushq	%r12
1416f116e: 56                          	pushq	%rsi
1416f116f: 57                          	pushq	%rdi
1416f1170: 53                          	pushq	%rbx
1416f1171: 48 83 ec 28                 	subq	$0x28, %rsp
1416f1175: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1416f117c: 48 8d 4d 48                 	leaq	0x48(%rbp), %rcx
1416f1180: e8 bb 7b 1a ff              	callq	0x140898d40 <.text+0x897d40>
1416f1185: 90                          	nop
1416f1186: 48 83 c4 28                 	addq	$0x28, %rsp
1416f118a: 5b                          	popq	%rbx
1416f118b: 5f                          	popq	%rdi
1416f118c: 5e                          	popq	%rsi
1416f118d: 41 5c                       	popq	%r12
1416f118f: 41 5d                       	popq	%r13
1416f1191: 41 5e                       	popq	%r14
1416f1193: 41 5f                       	popq	%r15
1416f1195: 5d                          	popq	%rbp
1416f1196: c3                          	retq
