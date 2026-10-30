
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1400290b0: 55                          	pushq	%rbp
1400290b1: 56                          	pushq	%rsi
1400290b2: 57                          	pushq	%rdi
1400290b3: 48 81 ec b0 01 00 00        	subq	$0x1b0, %rsp            # imm = 0x1B0
1400290ba: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1400290c2: 48 c7 85 28 01 00 00 fe ff ff ff    	movq	$-0x2, 0x128(%rbp)
1400290cd: 48 89 ce                    	movq	%rcx, %rsi
1400290d0: 0f 57 c0                    	xorps	%xmm0, %xmm0
1400290d3: 0f 11 45 08                 	movups	%xmm0, 0x8(%rbp)
1400290d7: 0f 11 45 20                 	movups	%xmm0, 0x20(%rbp)
1400290db: 0f 11 45 38                 	movups	%xmm0, 0x38(%rbp)
1400290df: 0f 11 45 50                 	movups	%xmm0, 0x50(%rbp)
1400290e3: 0f 11 45 68                 	movups	%xmm0, 0x68(%rbp)
1400290e7: c7 45 78 00 00 00 00        	movl	$0x0, 0x78(%rbp)
1400290ee: 0f 11 85 a0 00 00 00        	movups	%xmm0, 0xa0(%rbp)
1400290f5: 0f 11 85 b8 00 00 00        	movups	%xmm0, 0xb8(%rbp)
1400290fc: 0f 11 85 d0 00 00 00        	movups	%xmm0, 0xd0(%rbp)
140029103: 0f 11 85 e8 00 00 00        	movups	%xmm0, 0xe8(%rbp)
14002910a: 0f 11 85 00 01 00 00        	movups	%xmm0, 0x100(%rbp)
140029111: c7 85 10 01 00 00 00 00 00 00       	movl	$0x0, 0x110(%rbp)
14002911b: 48 c7 45 a8 00 00 00 00     	movq	$0x0, -0x58(%rbp)
140029123: 48 c7 45 e8 00 00 00 00     	movq	$0x0, -0x18(%rbp)
14002912b: 48 c7 45 f0 08 00 00 00     	movq	$0x8, -0x10(%rbp)
140029133: 48 c7 45 f8 00 00 00 00     	movq	$0x0, -0x8(%rbp)
14002913b: 48 c7 45 00 10 00 00 00     	movq	$0x10, (%rbp)
140029143: 48 c7 45 18 10 00 00 00     	movq	$0x10, 0x18(%rbp)
14002914b: 48 c7 45 30 10 00 00 00     	movq	$0x10, 0x30(%rbp)
140029153: 48 c7 45 48 10 00 00 00     	movq	$0x10, 0x48(%rbp)
14002915b: 48 c7 45 60 10 00 00 00     	movq	$0x10, 0x60(%rbp)
140029163: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14002916d: 48 89 85 80 00 00 00        	movq	%rax, 0x80(%rbp)
140029174: 48 8d 05 62 e3 8f 02        	leaq	0x28fe362(%rip), %rax   # 0x1429274dd
14002917b: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
140029182: 48 c7 85 90 00 00 00 5e 00 00 00    	movq	$0x5e, 0x90(%rbp)
14002918d: 48 c7 85 98 00 00 00 10 00 00 00    	movq	$0x10, 0x98(%rbp)
140029198: 48 c7 85 b0 00 00 00 10 00 00 00    	movq	$0x10, 0xb0(%rbp)
1400291a3: 48 c7 85 c8 00 00 00 10 00 00 00    	movq	$0x10, 0xc8(%rbp)
1400291ae: 48 c7 85 e0 00 00 00 10 00 00 00    	movq	$0x10, 0xe0(%rbp)
1400291b9: 48 c7 85 f8 00 00 00 10 00 00 00    	movq	$0x10, 0xf8(%rbp)
1400291c4: c7 85 18 01 00 00 00 00 00 00       	movl	$0x0, 0x118(%rbp)
1400291ce: 66 c7 85 1c 01 00 00 01 00  	movw	$0x1, 0x11c(%rbp)
1400291d7: c7 85 20 01 00 00 00 00 00 00       	movl	$0x0, 0x120(%rbp)
1400291e1: 0f b6 05 21 ba 60 03        	movzbl	0x360ba21(%rip), %eax   # 0x143634c09
1400291e8: b9 80 01 00 00              	movl	$0x180, %ecx            # imm = 0x180
1400291ed: ba 08 00 00 00              	movl	$0x8, %edx
1400291f2: e8 19 aa 5e 00              	callq	0x140613c10 <.text+0x612c10>
1400291f7: 48 85 c0                    	testq	%rax, %rax
1400291fa: 74 35                       	je	0x140029231 <.text+0x28231>
1400291fc: 48 89 c7                    	movq	%rax, %rdi
1400291ff: 48 8d 55 a8                 	leaq	-0x58(%rbp), %rdx
140029203: 41 b8 80 01 00 00           	movl	$0x180, %r8d            # imm = 0x180
140029209: 48 89 c1                    	movq	%rax, %rcx
14002920c: e8 ef a9 8f 02              	callq	0x142923c00 <.text+0x2922c00>
140029211: 4c 8d 05 30 dd 8f 02        	leaq	0x28fdd30(%rip), %r8    # 0x142926f48
140029218: 48 89 f1                    	movq	%rsi, %rcx
14002921b: 48 89 fa                    	movq	%rdi, %rdx
14002921e: e8 ed b8 cb 00              	callq	0x140ce4b10 <.text+0xce3b10>
140029223: 48 89 f0                    	movq	%rsi, %rax
140029226: 48 81 c4 b0 01 00 00        	addq	$0x1b0, %rsp            # imm = 0x1B0
14002922d: 5f                          	popq	%rdi
14002922e: 5e                          	popq	%rsi
14002922f: 5d                          	popq	%rbp
140029230: c3                          	retq
140029231: b9 08 00 00 00              	movl	$0x8, %ecx
140029236: ba 80 01 00 00              	movl	$0x180, %edx            # imm = 0x180
14002923b: e8 c0 fd 8a 02              	callq	0x1428d9000 <.text+0x28d8000>
