
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408b8e90: 48 83 ec 28                 	subq	$0x28, %rsp
1408b8e94: 48 89 c8                    	movq	%rcx, %rax
1408b8e97: 0f b6 ca                    	movzbl	%dl, %ecx
1408b8e9a: 48 8d 15 fb 71 1e 02        	leaq	0x21e71fb(%rip), %rdx   # 0x142aa009c
1408b8ea1: 48 63 0c 8a                 	movslq	(%rdx,%rcx,4), %rcx
1408b8ea5: 48 01 d1                    	addq	%rdx, %rcx
1408b8ea8: ff e1                       	jmpq	*%rcx
1408b8eaa: 49 83 c0 18                 	addq	$0x18, %r8
1408b8eae: e9 82 02 00 00              	jmp	0x1408b9135 <.text+0x8b8135>
1408b8eb3: 49 81 c0 a0 01 00 00        	addq	$0x1a0, %r8             # imm = 0x1A0
1408b8eba: e9 5f 02 00 00              	jmp	0x1408b911e <.text+0x8b811e>
1408b8ebf: 49 81 c0 68 01 00 00        	addq	$0x168, %r8             # imm = 0x168
1408b8ec6: e9 46 01 00 00              	jmp	0x1408b9011 <.text+0x8b8011>
1408b8ecb: 49 81 c0 38 01 00 00        	addq	$0x138, %r8             # imm = 0x138
1408b8ed2: b1 07                       	movb	$0x7, %cl
1408b8ed4: e9 47 02 00 00              	jmp	0x1408b9120 <.text+0x8b8120>
1408b8ed9: 49 81 c0 b8 01 00 00        	addq	$0x1b8, %r8             # imm = 0x1B8
1408b8ee0: e9 50 02 00 00              	jmp	0x1408b9135 <.text+0x8b8135>
1408b8ee5: 49 81 c0 88 01 00 00        	addq	$0x188, %r8             # imm = 0x188
1408b8eec: e9 2d 02 00 00              	jmp	0x1408b911e <.text+0x8b811e>
1408b8ef1: 49 81 c0 28 01 00 00        	addq	$0x128, %r8             # imm = 0x128
1408b8ef8: ba 1b 00 00 00              	movl	$0x1b, %edx
1408b8efd: 4c 8d 0d b3 70 1e 02        	leaq	0x21e70b3(%rip), %r9    # 0x142a9ffb7
1408b8f04: 31 c9                       	xorl	%ecx, %ecx
1408b8f06: e9 38 02 00 00              	jmp	0x1408b9143 <.text+0x8b8143>
1408b8f0b: 49 81 c0 30 01 00 00        	addq	$0x130, %r8             # imm = 0x130
1408b8f12: b1 01                       	movb	$0x1, %cl
1408b8f14: e9 07 02 00 00              	jmp	0x1408b9120 <.text+0x8b8120>
1408b8f19: 49 81 c0 f0 00 00 00        	addq	$0xf0, %r8
1408b8f20: e9 10 02 00 00              	jmp	0x1408b9135 <.text+0x8b8135>
1408b8f25: 49 81 c0 20 01 00 00        	addq	$0x120, %r8             # imm = 0x120
1408b8f2c: b1 03                       	movb	$0x3, %cl
1408b8f2e: e9 ed 01 00 00              	jmp	0x1408b9120 <.text+0x8b8120>
1408b8f33: 49 81 c0 98 00 00 00        	addq	$0x98, %r8
1408b8f3a: e9 df 01 00 00              	jmp	0x1408b911e <.text+0x8b811e>
1408b8f3f: 49 81 c0 70 01 00 00        	addq	$0x170, %r8             # imm = 0x170
1408b8f46: e9 d3 01 00 00              	jmp	0x1408b911e <.text+0x8b811e>
1408b8f4b: 49 81 c0 88 00 00 00        	addq	$0x88, %r8
1408b8f52: b1 05                       	movb	$0x5, %cl
1408b8f54: e9 c7 01 00 00              	jmp	0x1408b9120 <.text+0x8b8120>
1408b8f59: 49 81 c0 18 01 00 00        	addq	$0x118, %r8             # imm = 0x118
1408b8f60: e9 d0 01 00 00              	jmp	0x1408b9135 <.text+0x8b8135>
1408b8f65: 49 81 c0 90 01 00 00        	addq	$0x190, %r8             # imm = 0x190
1408b8f6c: e9 ad 01 00 00              	jmp	0x1408b911e <.text+0x8b811e>
1408b8f71: 49 83 c0 70                 	addq	$0x70, %r8
1408b8f75: e9 a4 01 00 00              	jmp	0x1408b911e <.text+0x8b811e>
1408b8f7a: 49 81 c0 98 01 00 00        	addq	$0x198, %r8             # imm = 0x198
1408b8f81: e9 98 01 00 00              	jmp	0x1408b911e <.text+0x8b811e>
1408b8f86: 49 83 c0 38                 	addq	$0x38, %r8
1408b8f8a: e9 a6 01 00 00              	jmp	0x1408b9135 <.text+0x8b8135>
1408b8f8f: 49 81 c0 50 01 00 00        	addq	$0x150, %r8             # imm = 0x150
1408b8f96: eb 79                       	jmp	0x1408b9011 <.text+0x8b8011>
1408b8f98: 49 83 c0 78                 	addq	$0x78, %r8
1408b8f9c: e9 7d 01 00 00              	jmp	0x1408b911e <.text+0x8b811e>
1408b8fa1: 49 83 c0 60                 	addq	$0x60, %r8
1408b8fa5: e9 8b 01 00 00              	jmp	0x1408b9135 <.text+0x8b8135>
1408b8faa: 49 81 c0 e0 00 00 00        	addq	$0xe0, %r8
1408b8fb1: e9 68 01 00 00              	jmp	0x1408b911e <.text+0x8b811e>
1408b8fb6: 49 81 c0 d8 00 00 00        	addq	$0xd8, %r8
1408b8fbd: e9 5c 01 00 00              	jmp	0x1408b911e <.text+0x8b811e>
1408b8fc2: 49 83 c0 28                 	addq	$0x28, %r8
1408b8fc6: e9 6a 01 00 00              	jmp	0x1408b9135 <.text+0x8b8135>
1408b8fcb: 49 81 c0 90 00 00 00        	addq	$0x90, %r8
1408b8fd2: e9 5e 01 00 00              	jmp	0x1408b9135 <.text+0x8b8135>
1408b8fd7: 49 83 c0 30                 	addq	$0x30, %r8
1408b8fdb: e9 55 01 00 00              	jmp	0x1408b9135 <.text+0x8b8135>
1408b8fe0: 49 83 c0 50                 	addq	$0x50, %r8
1408b8fe4: e9 4c 01 00 00              	jmp	0x1408b9135 <.text+0x8b8135>
1408b8fe9: 49 83 c0 20                 	addq	$0x20, %r8
1408b8fed: e9 43 01 00 00              	jmp	0x1408b9135 <.text+0x8b8135>
1408b8ff2: 49 81 c0 08 01 00 00        	addq	$0x108, %r8             # imm = 0x108
1408b8ff9: e9 37 01 00 00              	jmp	0x1408b9135 <.text+0x8b8135>
1408b8ffe: 49 81 c0 a0 00 00 00        	addq	$0xa0, %r8
1408b9005: e9 14 01 00 00              	jmp	0x1408b911e <.text+0x8b811e>
1408b900a: 49 81 c0 58 01 00 00        	addq	$0x158, %r8             # imm = 0x158
1408b9011: b1 04                       	movb	$0x4, %cl
1408b9013: e9 8c 00 00 00              	jmp	0x1408b90a4 <.text+0x8b80a4>
1408b9018: 49 81 c0 b8 00 00 00        	addq	$0xb8, %r8
1408b901f: b1 06                       	movb	$0x6, %cl
1408b9021: e9 fa 00 00 00              	jmp	0x1408b9120 <.text+0x8b8120>
1408b9026: 49 83 c0 40                 	addq	$0x40, %r8
1408b902a: e9 06 01 00 00              	jmp	0x1408b9135 <.text+0x8b8135>
1408b902f: 49 81 c0 c0 00 00 00        	addq	$0xc0, %r8
1408b9036: e9 e3 00 00 00              	jmp	0x1408b911e <.text+0x8b811e>
1408b903b: 49 83 e8 80                 	subq	$-0x80, %r8
1408b903f: e9 da 00 00 00              	jmp	0x1408b911e <.text+0x8b811e>
1408b9044: 49 83 c0 48                 	addq	$0x48, %r8
1408b9048: e9 e8 00 00 00              	jmp	0x1408b9135 <.text+0x8b8135>
1408b904d: 49 81 c0 a8 00 00 00        	addq	$0xa8, %r8
1408b9054: e9 c5 00 00 00              	jmp	0x1408b911e <.text+0x8b811e>
1408b9059: 49 83 c0 68                 	addq	$0x68, %r8
1408b905d: e9 bc 00 00 00              	jmp	0x1408b911e <.text+0x8b811e>
1408b9062: 49 83 c0 58                 	addq	$0x58, %r8
1408b9066: e9 ca 00 00 00              	jmp	0x1408b9135 <.text+0x8b8135>
1408b906b: 49 81 c0 00 01 00 00        	addq	$0x100, %r8             # imm = 0x100
1408b9072: e9 be 00 00 00              	jmp	0x1408b9135 <.text+0x8b8135>
1408b9077: 49 81 c0 10 01 00 00        	addq	$0x110, %r8             # imm = 0x110
1408b907e: e9 b2 00 00 00              	jmp	0x1408b9135 <.text+0x8b8135>
1408b9083: 49 81 c0 78 01 00 00        	addq	$0x178, %r8             # imm = 0x178
1408b908a: e9 a6 00 00 00              	jmp	0x1408b9135 <.text+0x8b8135>
1408b908f: 49 81 c0 c8 00 00 00        	addq	$0xc8, %r8
1408b9096: e9 9a 00 00 00              	jmp	0x1408b9135 <.text+0x8b8135>
1408b909b: 49 81 c0 48 01 00 00        	addq	$0x148, %r8             # imm = 0x148
1408b90a2: b1 07                       	movb	$0x7, %cl
1408b90a4: ba 23 00 00 00              	movl	$0x23, %edx
1408b90a9: 4c 8d 0d 40 6f 1e 02        	leaq	0x21e6f40(%rip), %r9    # 0x142a9fff0
1408b90b0: e9 8e 00 00 00              	jmp	0x1408b9143 <.text+0x8b8143>
1408b90b5: 49 81 c0 b0 00 00 00        	addq	$0xb0, %r8
1408b90bc: eb 60                       	jmp	0x1408b911e <.text+0x8b811e>
1408b90be: 49 81 c0 60 01 00 00        	addq	$0x160, %r8             # imm = 0x160
1408b90c5: b1 02                       	movb	$0x2, %cl
1408b90c7: eb 57                       	jmp	0x1408b9120 <.text+0x8b8120>
1408b90c9: 49 81 c0 d0 00 00 00        	addq	$0xd0, %r8
1408b90d0: eb 4c                       	jmp	0x1408b911e <.text+0x8b811e>
1408b90d2: 49 81 c0 f8 00 00 00        	addq	$0xf8, %r8
1408b90d9: eb 5a                       	jmp	0x1408b9135 <.text+0x8b8135>
1408b90db: 49 83 78 10 00              	cmpq	$0x0, 0x10(%r8)
1408b90e0: 74 77                       	je	0x1408b9159 <.text+0x8b8159>
1408b90e2: 4d 8b 40 08                 	movq	0x8(%r8), %r8
1408b90e6: eb 4d                       	jmp	0x1408b9135 <.text+0x8b8135>
1408b90e8: 49 81 c0 40 01 00 00        	addq	$0x140, %r8             # imm = 0x140
1408b90ef: b1 08                       	movb	$0x8, %cl
1408b90f1: eb 2d                       	jmp	0x1408b9120 <.text+0x8b8120>
1408b90f3: 49 81 c0 e8 00 00 00        	addq	$0xe8, %r8
1408b90fa: eb 22                       	jmp	0x1408b911e <.text+0x8b811e>
1408b90fc: 49 81 c0 c0 01 00 00        	addq	$0x1c0, %r8             # imm = 0x1C0
1408b9103: eb 19                       	jmp	0x1408b911e <.text+0x8b811e>
1408b9105: 49 81 c0 c8 01 00 00        	addq	$0x1c8, %r8             # imm = 0x1C8
1408b910c: eb 10                       	jmp	0x1408b911e <.text+0x8b811e>
1408b910e: 49 81 c0 b0 01 00 00        	addq	$0x1b0, %r8             # imm = 0x1B0
1408b9115: eb 1e                       	jmp	0x1408b9135 <.text+0x8b8135>
1408b9117: 49 81 c0 80 01 00 00        	addq	$0x180, %r8             # imm = 0x180
1408b911e: b1 04                       	movb	$0x4, %cl
1408b9120: ba 1b 00 00 00              	movl	$0x1b, %edx
1408b9125: 4c 8d 0d 8b 6e 1e 02        	leaq	0x21e6e8b(%rip), %r9    # 0x142a9ffb7
1408b912c: eb 15                       	jmp	0x1408b9143 <.text+0x8b8143>
1408b912e: 49 81 c0 a8 01 00 00        	addq	$0x1a8, %r8             # imm = 0x1A8
1408b9135: b1 09                       	movb	$0x9, %cl
1408b9137: ba 17 00 00 00              	movl	$0x17, %edx
1408b913c: 4c 8d 0d 5d 6e 1e 02        	leaq	0x21e6e5d(%rip), %r9    # 0x142a9ffa0
1408b9143: 4d 8b 00                    	movq	(%r8), %r8
1408b9146: 4c 89 00                    	movq	%r8, (%rax)
1408b9149: 4c 89 48 08                 	movq	%r9, 0x8(%rax)
1408b914d: 48 89 50 10                 	movq	%rdx, 0x10(%rax)
1408b9151: 88 48 18                    	movb	%cl, 0x18(%rax)
1408b9154: 48 83 c4 28                 	addq	$0x28, %rsp
1408b9158: c3                          	retq
1408b9159: 4c 8d 05 78 6e 1e 02        	leaq	0x21e6e78(%rip), %r8    # 0x142a9ffd8
1408b9160: 31 c9                       	xorl	%ecx, %ecx
1408b9162: 31 d2                       	xorl	%edx, %edx
1408b9164: e8 af 03 02 02              	callq	0x1428d9518 <.text+0x28d8518>
1408b9169: cc                          	int3
1408b916a: cc                          	int3
1408b916b: cc                          	int3
1408b916c: cc                          	int3
1408b916d: cc                          	int3
1408b916e: cc                          	int3
1408b916f: cc                          	int3
