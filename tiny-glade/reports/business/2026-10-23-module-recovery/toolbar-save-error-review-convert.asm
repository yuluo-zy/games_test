
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141720490: 56                          	pushq	%rsi
141720491: 48 81 ec b0 00 00 00        	subq	$0xb0, %rsp
141720498: 48 89 ce                    	movq	%rcx, %rsi
14172049b: 48 89 54 24 68              	movq	%rdx, 0x68(%rsp)
1417204a0: 48 8b 0a                    	movq	(%rdx), %rcx
1417204a3: 48 8b 01                    	movq	(%rcx), %rax
1417204a6: 48 ba 88 cb 38 d9 47 f0 f9 03       	movabsq	$0x3f9f047d938cb88, %rdx # imm = 0x3F9F047D938CB88
1417204b0: 49 b8 81 ac a7 8d c2 9f ce 52       	movabsq	$0x52ce9fc28da7ac81, %r8 # imm = 0x52CE9FC28DA7AC81
1417204ba: ff 50 18                    	callq	*0x18(%rax)
1417204bd: 48 85 c0                    	testq	%rax, %rax
1417204c0: 0f 84 03 01 00 00           	je	0x1417205c9 <.text+0x171f5c9>
1417204c6: 48 8b 08                    	movq	(%rax), %rcx
1417204c9: 89 ca                       	movl	%ecx, %edx
1417204cb: 83 e2 03                    	andl	$0x3, %edx
1417204ce: 4c 8d 05 ef ae 5a 01        	leaq	0x15aaeef(%rip), %r8    # 0x142ccb3c4
1417204d5: 49 63 14 90                 	movslq	(%r8,%rdx,4), %rdx
1417204d9: 4c 01 c2                    	addq	%r8, %rdx
1417204dc: ff e2                       	jmpq	*%rdx
1417204de: ba 10 00 00 00              	movl	$0x10, %edx
1417204e3: 80 3c 11 18                 	cmpb	$0x18, (%rcx,%rdx)
1417204e7: 74 26                       	je	0x14172050f <.text+0x171f50f>
1417204e9: e9 db 00 00 00              	jmp	0x1417205c9 <.text+0x171f5c9>
1417204ee: 48 c1 e9 20                 	shrq	$0x20, %rcx
1417204f2: 48 83 f9 27                 	cmpq	$0x27, %rcx
1417204f6: 74 17                       	je	0x14172050f <.text+0x171f50f>
1417204f8: 83 f9 70                    	cmpl	$0x70, %ecx
1417204fb: 74 12                       	je	0x14172050f <.text+0x171f50f>
1417204fd: e9 c7 00 00 00              	jmp	0x1417205c9 <.text+0x171f5c9>
141720502: 48 c1 e9 20                 	shrq	$0x20, %rcx
141720506: 83 f9 18                    	cmpl	$0x18, %ecx
141720509: 0f 85 ba 00 00 00           	jne	0x1417205c9 <.text+0x171f5c9>
14172050f: 48 89 44 24 70              	movq	%rax, 0x70(%rsp)
141720514: 48 8d 44 24 70              	leaq	0x70(%rsp), %rax
141720519: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
14172051e: 48 8d 05 bb ab 02 ff        	leaq	-0xfd5445(%rip), %rax   # 0x14074b0e0 <.text+0x74a0e0>
141720525: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
14172052a: 48 c7 44 24 78 02 00 00 00  	movq	$0x2, 0x78(%rsp)
141720533: 48 c7 84 24 88 00 00 00 02 00 00 00 	movq	$0x2, 0x88(%rsp)
14172053f: 48 c7 84 24 98 00 00 00 00 00 00 00 	movq	$0x0, 0x98(%rsp)
14172054b: 48 b8 20 00 00 00 04 00 00 00       	movabsq	$0x400000020, %rax # imm = 0x400000020
141720555: 48 89 84 24 a0 00 00 00     	movq	%rax, 0xa0(%rsp)
14172055d: c6 84 24 a8 00 00 00 03     	movb	$0x3, 0xa8(%rsp)
141720565: 48 8d 05 44 a3 5a 01        	leaq	0x15aa344(%rip), %rax   # 0x142cca8b0
14172056c: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
141720571: 48 c7 44 24 40 01 00 00 00  	movq	$0x1, 0x40(%rsp)
14172057a: 48 8d 44 24 28              	leaq	0x28(%rsp), %rax
14172057f: 48 89 44 24 48              	movq	%rax, 0x48(%rsp)
141720584: 48 c7 44 24 50 01 00 00 00  	movq	$0x1, 0x50(%rsp)
14172058d: 48 8d 44 24 78              	leaq	0x78(%rsp), %rax
141720592: 48 89 44 24 58              	movq	%rax, 0x58(%rsp)
141720597: 48 c7 44 24 60 01 00 00 00  	movq	$0x1, 0x60(%rsp)
1417205a0: 48 8d 4e 08                 	leaq	0x8(%rsi), %rcx
1417205a4: 48 8d 54 24 38              	leaq	0x38(%rsp), %rdx
1417205a9: e8 e2 99 65 ff              	callq	0x140d79f90 <.text+0xd78f90>
1417205ae: 48 c7 06 00 00 00 00        	movq	$0x0, (%rsi)
1417205b5: e9 b0 00 00 00              	jmp	0x14172066a <.text+0x171f66a>
1417205ba: ba 0f 00 00 00              	movl	$0xf, %edx
1417205bf: 80 3c 11 18                 	cmpb	$0x18, (%rcx,%rdx)
1417205c3: 0f 84 46 ff ff ff           	je	0x14172050f <.text+0x171f50f>
1417205c9: 48 8d 44 24 68              	leaq	0x68(%rsp), %rax
1417205ce: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
1417205d3: 48 8d 05 86 74 e9 fe        	leaq	-0x1168b7a(%rip), %rax  # 0x1405b7a60 <.text+0x5b6a60>
1417205da: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
1417205df: 48 c7 44 24 78 02 00 00 00  	movq	$0x2, 0x78(%rsp)
1417205e8: 48 c7 84 24 88 00 00 00 02 00 00 00 	movq	$0x2, 0x88(%rsp)
1417205f4: 48 c7 84 24 98 00 00 00 00 00 00 00 	movq	$0x0, 0x98(%rsp)
141720600: 48 b8 20 00 00 00 04 00 00 00       	movabsq	$0x400000020, %rax # imm = 0x400000020
14172060a: 48 89 84 24 a0 00 00 00     	movq	%rax, 0xa0(%rsp)
141720612: c6 84 24 a8 00 00 00 03     	movb	$0x3, 0xa8(%rsp)
14172061a: 48 8d 05 8f a2 5a 01        	leaq	0x15aa28f(%rip), %rax   # 0x142cca8b0
141720621: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
141720626: 48 c7 44 24 40 01 00 00 00  	movq	$0x1, 0x40(%rsp)
14172062f: 48 8d 44 24 28              	leaq	0x28(%rsp), %rax
141720634: 48 89 44 24 48              	movq	%rax, 0x48(%rsp)
141720639: 48 c7 44 24 50 01 00 00 00  	movq	$0x1, 0x50(%rsp)
141720642: 48 8d 44 24 78              	leaq	0x78(%rsp), %rax
141720647: 48 89 44 24 58              	movq	%rax, 0x58(%rsp)
14172064c: 48 c7 44 24 60 01 00 00 00  	movq	$0x1, 0x60(%rsp)
141720655: 48 8d 4e 08                 	leaq	0x8(%rsi), %rcx
141720659: 48 8d 54 24 38              	leaq	0x38(%rsp), %rdx
14172065e: e8 2d 99 65 ff              	callq	0x140d79f90 <.text+0xd78f90>
141720663: 48 c7 06 01 00 00 00        	movq	$0x1, (%rsi)
14172066a: 48 89 f0                    	movq	%rsi, %rax
14172066d: 48 81 c4 b0 00 00 00        	addq	$0xb0, %rsp
141720674: 5e                          	popq	%rsi
141720675: c3                          	retq
141720676: cc                          	int3
141720677: cc                          	int3
141720678: cc                          	int3
141720679: cc                          	int3
14172067a: cc                          	int3
14172067b: cc                          	int3
14172067c: cc                          	int3
14172067d: cc                          	int3
14172067e: cc                          	int3
14172067f: cc                          	int3
