
D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14062e8d0: 48 83 ec 78                 	subq	$0x78, %rsp
14062e8d4: 48 89 d0                    	movq	%rdx, %rax
14062e8d7: 48 89 ca                    	movq	%rcx, %rdx
14062e8da: 49 8b 48 40                 	movq	0x40(%r8), %rcx
14062e8de: 48 89 4c 24 60              	movq	%rcx, 0x60(%rsp)
14062e8e3: 41 0f 10 00                 	movups	(%r8), %xmm0
14062e8e7: 41 0f 10 48 10              	movups	0x10(%r8), %xmm1
14062e8ec: 41 0f 10 50 20              	movups	0x20(%r8), %xmm2
14062e8f1: 41 0f 10 58 30              	movups	0x30(%r8), %xmm3
14062e8f6: 0f 29 5c 24 50              	movaps	%xmm3, 0x50(%rsp)
14062e8fb: 0f 29 54 24 40              	movaps	%xmm2, 0x40(%rsp)
14062e900: 0f 29 4c 24 30              	movaps	%xmm1, 0x30(%rsp)
14062e905: 0f 29 44 24 20              	movaps	%xmm0, 0x20(%rsp)
14062e90a: c6 44 24 68 00              	movb	$0x0, 0x68(%rsp)
14062e90f: 48 8d 4c 24 20              	leaq	0x20(%rsp), %rcx
14062e914: 49 89 c0                    	movq	%rax, %r8
14062e917: e8 c4 d5 79 00              	callq	0x140dcbee0 <.text+0xdcaee0>
14062e91c: 90                          	nop
14062e91d: 48 83 c4 78                 	addq	$0x78, %rsp
14062e921: c3                          	retq
14062e922: cc                          	int3
14062e923: cc                          	int3
14062e924: cc                          	int3
14062e925: cc                          	int3
14062e926: cc                          	int3
14062e927: cc                          	int3
14062e928: cc                          	int3
14062e929: cc                          	int3
14062e92a: cc                          	int3
14062e92b: cc                          	int3
14062e92c: cc                          	int3
14062e92d: cc                          	int3
14062e92e: cc                          	int3
14062e92f: cc                          	int3
14062e930: 48 8b 09                    	movq	(%rcx), %rcx
14062e933: e9 08 00 00 00              	jmp	0x14062e940 <.text+0x62d940>
14062e938: cc                          	int3
14062e939: cc                          	int3
14062e93a: cc                          	int3
14062e93b: cc                          	int3
14062e93c: cc                          	int3
14062e93d: cc                          	int3
14062e93e: cc                          	int3
14062e93f: cc                          	int3
14062e940: 48 83 ec 68                 	subq	$0x68, %rsp
14062e944: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14062e94e: 48 33 01                    	xorq	(%rcx), %rax
14062e951: 48 83 f8 04                 	cmpq	$0x4, %rax
14062e955: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14062e95b: 4c 0f 42 c0                 	cmovbq	%rax, %r8
14062e95f: 48 8d 05 4a dd 3d 02        	leaq	0x23ddd4a(%rip), %rax   # 0x142a0c6b0
14062e966: 4e 63 04 80                 	movslq	(%rax,%r8,4), %r8
14062e96a: 49 01 c0                    	addq	%rax, %r8
14062e96d: 41 ff e0                    	jmpq	*%r8
14062e970: 48 8d 05 f1 da 3d 02        	leaq	0x23ddaf1(%rip), %rax   # 0x142a0c468
14062e977: 41 b8 5c 00 00 00           	movl	$0x5c, %r8d
14062e97d: e9 8f 00 00 00              	jmp	0x14062ea11 <.text+0x62da11>
14062e982: 48 89 4c 24 20              	movq	%rcx, 0x20(%rsp)
14062e987: 48 8d 44 24 20              	leaq	0x20(%rsp), %rax
14062e98c: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
14062e991: 48 8d 05 88 4f a2 ff        	leaq	-0x5db078(%rip), %rax   # 0x140053920 <.text+0x52920>
14062e998: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
14062e99d: 48 8d 05 6c dc 3d 02        	leaq	0x23ddc6c(%rip), %rax   # 0x142a0c610
14062e9a4: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
14062e9a9: 48 c7 44 24 40 01 00 00 00  	movq	$0x1, 0x40(%rsp)
14062e9b2: 48 c7 44 24 58 00 00 00 00  	movq	$0x0, 0x58(%rsp)
14062e9bb: 48 8d 44 24 28              	leaq	0x28(%rsp), %rax
14062e9c0: 48 89 44 24 48              	movq	%rax, 0x48(%rsp)
14062e9c5: 48 c7 44 24 50 01 00 00 00  	movq	$0x1, 0x50(%rsp)
14062e9ce: 48 8b 4a 30                 	movq	0x30(%rdx), %rcx
14062e9d2: 48 8b 52 38                 	movq	0x38(%rdx), %rdx
14062e9d6: 4c 8d 44 24 38              	leaq	0x38(%rsp), %r8
14062e9db: e8 f0 70 75 00              	callq	0x140d85ad0 <.text+0xd84ad0>
14062e9e0: 90                          	nop
14062e9e1: 48 83 c4 68                 	addq	$0x68, %rsp
14062e9e5: c3                          	retq
14062e9e6: 48 8d 05 25 db 3d 02        	leaq	0x23ddb25(%rip), %rax   # 0x142a0c512
14062e9ed: 41 b8 4b 00 00 00           	movl	$0x4b, %r8d
14062e9f3: eb 1c                       	jmp	0x14062ea11 <.text+0x62da11>
14062e9f5: 48 8d 05 61 db 3d 02        	leaq	0x23ddb61(%rip), %rax   # 0x142a0c55d
14062e9fc: 41 b8 7f 00 00 00           	movl	$0x7f, %r8d
14062ea02: eb 0d                       	jmp	0x14062ea11 <.text+0x62da11>
14062ea04: 48 8d 05 b9 da 3d 02        	leaq	0x23ddab9(%rip), %rax   # 0x142a0c4c4
14062ea0b: 41 b8 4e 00 00 00           	movl	$0x4e, %r8d
14062ea11: 48 89 d1                    	movq	%rdx, %rcx
14062ea14: 48 89 c2                    	movq	%rax, %rdx
14062ea17: 48 83 c4 68                 	addq	$0x68, %rsp
14062ea1b: e9 70 81 75 00              	jmp	0x140d86b90 <.text+0xd85b90>
14062ea20: 48 81 ec 88 00 00 00        	subq	$0x88, %rsp
14062ea27: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14062ea31: 48 33 01                    	xorq	(%rcx), %rax
14062ea34: 48 83 f8 04                 	cmpq	$0x4, %rax
14062ea38: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14062ea3e: 4c 0f 42 c0                 	cmovbq	%rax, %r8
14062ea42: 48 89 4c 24 30              	movq	%rcx, 0x30(%rsp)
14062ea47: 48 8d 05 76 dc 3d 02        	leaq	0x23ddc76(%rip), %rax   # 0x142a0c6c4
14062ea4e: 4a 63 0c 80                 	movslq	(%rax,%r8,4), %rcx
14062ea52: 48 01 c1                    	addq	%rax, %rcx
14062ea55: ff e1                       	jmpq	*%rcx
14062ea57: 48 8d 05 c2 db 3d 02        	leaq	0x23ddbc2(%rip), %rax   # 0x142a0c620
14062ea5e: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
