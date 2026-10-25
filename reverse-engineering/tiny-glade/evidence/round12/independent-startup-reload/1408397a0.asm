
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408397a0: 48 81 ec 88 00 00 00        	subq	$0x88, %rsp
1408397a7: b8 01 00 00 00              	movl	$0x1, %eax
1408397ac: f0                          	lock
1408397ad: 0f c1 82 b0 05 00 00        	xaddl	%eax, 0x5b0(%rdx)
1408397b4: f6 01 01                    	testb	$0x1, (%rcx)
1408397b7: 0f 84 0d 01 00 00           	je	0x1408398ca <.text+0x8388ca>
1408397bd: 4c 8b 41 08                 	movq	0x8(%rcx), %r8
1408397c1: 4c 39 82 18 02 00 00        	cmpq	%r8, 0x218(%rdx)
1408397c8: 76 7b                       	jbe	0x140839845 <.text+0x838845>
1408397ca: 4c 8b 8a 10 02 00 00        	movq	0x210(%rdx), %r9
1408397d1: 4f 8b 0c c1                 	movq	(%r9,%r8,8), %r9
1408397d5: 4d 85 c9                    	testq	%r9, %r9
1408397d8: 74 6b                       	je	0x140839845 <.text+0x838845>
1408397da: 4c 8b 82 e0 01 00 00        	movq	0x1e0(%rdx), %r8
1408397e1: 49 f7 d1                    	notq	%r9
1408397e4: 4f 8d 0c 49                 	leaq	(%r9,%r9,2), %r9
1408397e8: 49 c1 e1 05                 	shlq	$0x5, %r9
1408397ec: 4b 83 7c 08 20 00           	cmpq	$0x0, 0x20(%r8,%r9)
1408397f2: 74 51                       	je	0x140839845 <.text+0x838845>
1408397f4: 4d 01 c8                    	addq	%r9, %r8
1408397f7: 4d 8b 48 10                 	movq	0x10(%r8), %r9
1408397fb: 8b 92 b0 05 00 00           	movl	0x5b0(%rdx), %edx
140839801: 80 3d 88 b5 df 02 00        	cmpb	$0x0, 0x2dfb588(%rip)   # 0x143634d90
140839808: 75 0a                       	jne	0x140839814 <.text+0x838814>
14083980a: f3 0f 10 05 66 38 19 02     	movss	0x2193866(%rip), %xmm0  # 0x1429cd078
140839812: eb 08                       	jmp	0x14083981c <.text+0x83881c>
140839814: f3 0f 10 05 e8 c0 0e 02     	movss	0x20ec0e8(%rip), %xmm0  # 0x142925904
14083981c: 41 89 40 5c                 	movl	%eax, 0x5c(%r8)
140839820: f3 41 0f 11 01              	movss	%xmm0, (%r9)
140839825: ba ff ff ff ff              	movl	$0xffffffff, %edx       # imm = 0xFFFFFFFF
14083982a: 49 89 51 04                 	movq	%rdx, 0x4(%r9)
14083982e: 66 41 c7 41 0c 00 01        	movw	$0x100, 0xc(%r9)        # imm = 0x100
140839835: 89 81 48 01 00 00           	movl	%eax, 0x148(%rcx)
14083983b: 31 c0                       	xorl	%eax, %eax
14083983d: 48 81 c4 88 00 00 00        	addq	$0x88, %rsp
140839844: c3                          	retq
140839845: 48 81 c1 b0 00 00 00        	addq	$0xb0, %rcx
14083984c: 48 8d 05 d4 96 25 02        	leaq	0x22596d4(%rip), %rax   # 0x142a92f27
140839853: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
140839858: 48 c7 44 24 30 3b 00 00 00  	movq	$0x3b, 0x30(%rsp)
140839861: 48 89 4c 24 38              	movq	%rcx, 0x38(%rsp)
140839866: 48 8d 05 63 22 80 ff        	leaq	-0x7fdd9d(%rip), %rax   # 0x14003bad0 <.text+0x3aad0>
14083986d: 48 89 44 24 40              	movq	%rax, 0x40(%rsp)
140839872: 48 8d 44 24 28              	leaq	0x28(%rsp), %rax
140839877: 48 89 44 24 48              	movq	%rax, 0x48(%rsp)
14083987c: 48 8d 05 0d 06 8d ff        	leaq	-0x72f9f3(%rip), %rax   # 0x140109e90 <.text+0x108e90>
140839883: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
140839888: 48 8d 05 01 97 25 02        	leaq	0x2259701(%rip), %rax   # 0x142a92f90
14083988f: 48 89 44 24 58              	movq	%rax, 0x58(%rsp)
140839894: 48 c7 44 24 60 02 00 00 00  	movq	$0x2, 0x60(%rsp)
14083989d: 48 c7 44 24 78 00 00 00 00  	movq	$0x0, 0x78(%rsp)
1408398a6: 48 8d 44 24 38              	leaq	0x38(%rsp), %rax
1408398ab: 48 89 44 24 68              	movq	%rax, 0x68(%rsp)
1408398b0: 48 c7 44 24 70 02 00 00 00  	movq	$0x2, 0x70(%rsp)
1408398b9: 48 8d 15 58 97 25 02        	leaq	0x2259758(%rip), %rdx   # 0x142a93018
1408398c0: 48 8d 4c 24 58              	leaq	0x58(%rsp), %rcx
1408398c5: e8 c6 fa 09 02              	callq	0x1428d9390 <.text+0x28d8390>
1408398ca: 48 8d 0d b7 98 25 02        	leaq	0x22598b7(%rip), %rcx   # 0x142a93188
1408398d1: 4c 8d 05 10 99 25 02        	leaq	0x2259910(%rip), %r8    # 0x142a931e8
1408398d8: ba 59 00 00 00              	movl	$0x59, %edx
1408398dd: e8 4e fa 09 02              	callq	0x1428d9330 <.text+0x28d8330>
1408398e2: cc                          	int3
