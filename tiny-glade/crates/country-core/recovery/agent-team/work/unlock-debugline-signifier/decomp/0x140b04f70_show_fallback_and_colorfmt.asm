
/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b0507d: f2 0f 10 05 2b 96 e2 01     	movsd	0x1e2962b(%rip), %xmm0  # 0x14292e6b0
140b05085: f2 0f 11 45 18              	movsd	%xmm0, 0x18(%rbp)
140b0508a: c7 45 20 00 00 80 3f        	movl	$0x3f800000, 0x20(%rbp) # imm = 0x3F800000
140b05091: e9 87 00 00 00              	jmp	0x140b0511d <.text+0xb0411d>
140b05096: 48 89 45 c0                 	movq	%rax, -0x40(%rbp)
140b0509a: 41 0f 10 46 20              	movups	0x20(%r14), %xmm0
140b0509f: 0f 11 45 c8                 	movups	%xmm0, -0x38(%rbp)
140b050a3: 45 0f b6 46 45              	movzbl	0x45(%r14), %r8d
140b050a8: 0f 57 c9                    	xorps	%xmm1, %xmm1
140b050ab: 48 83 fb 02                 	cmpq	$0x2, %rbx
140b050af: 0f 83 9d 00 00 00           	jae	0x140b05152 <.text+0xb04152>
140b050b5: e9 cf 01 00 00              	jmp	0x140b05289 <.text+0xb04289>
140b050ba: f2 0f 10 05 9e bc e2 01     	movsd	0x1e2bc9e(%rip), %xmm0  # 0x142930d60
140b050c2: f2 0f 11 45 18              	movsd	%xmm0, 0x18(%rbp)
140b050c7: c7 45 20 cd cc cc 3d        	movl	$0x3dcccccd, 0x20(%rbp) # imm = 0x3DCCCCCD
140b050ce: eb 4d                       	jmp	0x140b0511d <.text+0xb0411d>
140b050d0: 48 c7 45 18 00 00 00 00     	movq	$0x0, 0x18(%rbp)
140b050d8: c7 45 20 00 00 80 3f        	movl	$0x3f800000, 0x20(%rbp) # imm = 0x3F800000
140b050df: eb 3c                       	jmp	0x140b0511d <.text+0xb0411d>
140b050e1: f2 0f 10 05 c7 f4 e2 01     	movsd	0x1e2f4c7(%rip), %xmm0  # 0x1429345b0
140b050e9: f2 0f 11 45 18              	movsd	%xmm0, 0x18(%rbp)
140b050ee: c7 45 20 00 00 00 00        	movl	$0x0, 0x20(%rbp)
140b050f5: eb 26                       	jmp	0x140b0511d <.text+0xb0411d>
140b050f7: f3 0f 10 05 05 08 e2 01     	movss	0x1e20805(%rip), %xmm0  # 0x142925904
140b050ff: 0f 13 45 18                 	movlps	%xmm0, 0x18(%rbp)
140b05103: c7 45 20 00 00 00 00        	movl	$0x0, 0x20(%rbp)
140b0510a: eb 11                       	jmp	0x140b0511d <.text+0xb0411d>
140b0510c: 49 8d 46 34                 	leaq	0x34(%r14), %rax
140b05110: 8b 48 08                    	movl	0x8(%rax), %ecx
140b05113: 89 4d 20                    	movl	%ecx, 0x20(%rbp)
140b05116: 48 8b 00                    	movq	(%rax), %rax
140b05119: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
140b0511d: c6 45 77 00                 	movb	$0x0, 0x77(%rbp)
140b05121: 4c 8d 0d d8 1f fe 01        	leaq	0x1fe1fd8(%rip), %r9    # 0x142ae7100
140b05128: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
140b0512c: 48 8d 55 18                 	leaq	0x18(%rbp), %rdx
140b05130: 49 89 d8                    	movq	%rbx, %r8
140b05133: e8 28 21 7f 00              	callq	0x1412f7260 <.text+0x12f6260>
140b05138: 4c 8b 75 68                 	movq	0x68(%rbp), %r14
140b0513c: 4c 8b 55 f8                 	movq	-0x8(%rbp), %r10
140b05140: 45 0f b6 46 45              	movzbl	0x45(%r14), %r8d
140b05145: 0f 57 c9                    	xorps	%xmm1, %xmm1
140b05148: 48 83 fb 02                 	cmpq	$0x2, %rbx
140b0514c: 0f 82 37 01 00 00           	jb	0x140b05289 <.text+0xb04289>
;; ---- DebugColor impl$7::fmt blocks ----

/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b05c70: 48 83 ec 38                 	subq	$0x38, %rsp
140b05c74: 48 89 d0                    	movq	%rdx, %rax
140b05c77: 8b 11                       	movl	(%rcx), %edx
140b05c79: 4c 8d 05 b4 16 fe 01        	leaq	0x1fe16b4(%rip), %r8    # 0x142ae7334
140b05c80: 49 63 14 90                 	movslq	(%r8,%rdx,4), %rdx
140b05c84: 4c 01 c2                    	addq	%r8, %rdx
140b05c87: ff e2                       	jmpq	*%rdx
140b05c89: 48 8d 15 31 15 fe 01        	leaq	0x1fe1531(%rip), %rdx   # 0x142ae71c1
140b05c90: eb 29                       	jmp	0x140b05cbb <.text+0xb04cbb>
140b05c92: 48 8d 15 39 15 fe 01        	leaq	0x1fe1539(%rip), %rdx   # 0x142ae71d2
140b05c99: eb 07                       	jmp	0x140b05ca2 <.text+0xb04ca2>
140b05c9b: 48 8d 15 27 15 fe 01        	leaq	0x1fe1527(%rip), %rdx   # 0x142ae71c9
140b05ca2: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140b05ca8: 48 89 c1                    	movq	%rax, %rcx
140b05cab: 48 83 c4 38                 	addq	$0x38, %rsp
140b05caf: e9 dc 0e 28 00              	jmp	0x140d86b90 <.text+0xd85b90>
140b05cb4: 48 8d 15 12 15 fe 01        	leaq	0x1fe1512(%rip), %rdx   # 0x142ae71cd
140b05cbb: 41 b8 05 00 00 00           	movl	$0x5, %r8d
140b05cc1: 48 89 c1                    	movq	%rax, %rcx
140b05cc4: 48 83 c4 38                 	addq	$0x38, %rsp
140b05cc8: e9 c3 0e 28 00              	jmp	0x140d86b90 <.text+0xd85b90>
140b05ccd: 48 8d 15 f2 14 fe 01        	leaq	0x1fe14f2(%rip), %rdx   # 0x142ae71c6
140b05cd4: 41 b8 03 00 00 00           	movl	$0x3, %r8d
140b05cda: 48 89 c1                    	movq	%rax, %rcx
140b05cdd: 48 83 c4 38                 	addq	$0x38, %rsp
140b05ce1: e9 aa 0e 28 00              	jmp	0x140d86b90 <.text+0xd85b90>
140b05ce6: 48 83 c1 04                 	addq	$0x4, %rcx
140b05cea: 48 89 4c 24 30              	movq	%rcx, 0x30(%rsp)
140b05cef: 48 8d 0d e2 14 fe 01        	leaq	0x1fe14e2(%rip), %rcx   # 0x142ae71d8
140b05cf6: 48 89 4c 24 20              	movq	%rcx, 0x20(%rsp)
140b05cfb: 48 8d 15 f6 14 fe 01        	leaq	0x1fe14f6(%rip), %rdx   # 0x142ae71f8
140b05d02: 4c 8d 4c 24 30              	leaq	0x30(%rsp), %r9
140b05d07: 41 b8 06 00 00 00           	movl	$0x6, %r8d
140b05d0d: 48 89 c1                    	movq	%rax, %rcx
140b05d10: e8 bb 15 28 00              	callq	0x140d872d0 <.text+0xd862d0>
140b05d15: 90                          	nop
140b05d16: 48 83 c4 38                 	addq	$0x38, %rsp
140b05d1a: c3                          	retq
140b05d1b: cc                          	int3
140b05d1c: cc                          	int3
140b05d1d: cc                          	int3
140b05d1e: cc                          	int3
140b05d1f: cc                          	int3
