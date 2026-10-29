
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1400b1230: 55                          	pushq	%rbp
1400b1231: 56                          	pushq	%rsi
1400b1232: 57                          	pushq	%rdi
1400b1233: 48 81 ec 90 01 00 00        	subq	$0x190, %rsp            # imm = 0x190
1400b123a: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1400b1242: 48 c7 85 08 01 00 00 fe ff ff ff    	movq	$-0x2, 0x108(%rbp)
1400b124d: 48 89 ce                    	movq	%rcx, %rsi
1400b1250: 0f 57 c0                    	xorps	%xmm0, %xmm0
1400b1253: 0f 11 45 e8                 	movups	%xmm0, -0x18(%rbp)
1400b1257: 0f 11 45 00                 	movups	%xmm0, (%rbp)
1400b125b: 0f 11 45 18                 	movups	%xmm0, 0x18(%rbp)
1400b125f: 0f 11 45 30                 	movups	%xmm0, 0x30(%rbp)
1400b1263: 0f 11 45 48                 	movups	%xmm0, 0x48(%rbp)
1400b1267: c7 45 58 00 00 00 00        	movl	$0x0, 0x58(%rbp)
1400b126e: 0f 11 85 80 00 00 00        	movups	%xmm0, 0x80(%rbp)
1400b1275: 0f 11 85 98 00 00 00        	movups	%xmm0, 0x98(%rbp)
1400b127c: 0f 11 85 b0 00 00 00        	movups	%xmm0, 0xb0(%rbp)
1400b1283: 0f 11 85 c8 00 00 00        	movups	%xmm0, 0xc8(%rbp)
1400b128a: 0f 11 85 e0 00 00 00        	movups	%xmm0, 0xe0(%rbp)
1400b1291: c7 85 f0 00 00 00 00 00 00 00       	movl	$0x0, 0xf0(%rbp)
1400b129b: 48 c7 45 a0 00 00 00 00     	movq	$0x0, -0x60(%rbp)
1400b12a3: 48 c7 45 c8 00 00 00 00     	movq	$0x0, -0x38(%rbp)
1400b12ab: 48 c7 45 d0 08 00 00 00     	movq	$0x8, -0x30(%rbp)
1400b12b3: 48 c7 45 d8 00 00 00 00     	movq	$0x0, -0x28(%rbp)
1400b12bb: 48 c7 45 e0 10 00 00 00     	movq	$0x10, -0x20(%rbp)
1400b12c3: 48 c7 45 f8 10 00 00 00     	movq	$0x10, -0x8(%rbp)
1400b12cb: 48 c7 45 10 10 00 00 00     	movq	$0x10, 0x10(%rbp)
1400b12d3: 48 c7 45 28 10 00 00 00     	movq	$0x10, 0x28(%rbp)
1400b12db: 48 c7 45 40 10 00 00 00     	movq	$0x10, 0x40(%rbp)
1400b12e3: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1400b12ed: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
1400b12f1: 48 8d 05 e2 31 88 02        	leaq	0x28831e2(%rip), %rax   # 0x1429344da
1400b12f8: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
1400b12fc: 48 c7 45 70 3d 00 00 00     	movq	$0x3d, 0x70(%rbp)
1400b1304: 48 c7 45 78 10 00 00 00     	movq	$0x10, 0x78(%rbp)
1400b130c: 48 c7 85 90 00 00 00 10 00 00 00    	movq	$0x10, 0x90(%rbp)
1400b1317: 48 c7 85 a8 00 00 00 10 00 00 00    	movq	$0x10, 0xa8(%rbp)
1400b1322: 48 c7 85 c0 00 00 00 10 00 00 00    	movq	$0x10, 0xc0(%rbp)
1400b132d: 48 c7 85 d8 00 00 00 10 00 00 00    	movq	$0x10, 0xd8(%rbp)
1400b1338: c7 85 f8 00 00 00 00 00 00 00       	movl	$0x0, 0xf8(%rbp)
1400b1342: 66 c7 85 fc 00 00 00 01 00  	movw	$0x1, 0xfc(%rbp)
1400b134b: c7 85 00 01 00 00 00 00 00 00       	movl	$0x0, 0x100(%rbp)
1400b1355: 0f b6 05 ad 38 58 03        	movzbl	0x35838ad(%rip), %eax   # 0x143634c09
1400b135c: b9 68 01 00 00              	movl	$0x168, %ecx            # imm = 0x168
1400b1361: ba 08 00 00 00              	movl	$0x8, %edx
1400b1366: e8 a5 28 56 00              	callq	0x140613c10 <.text+0x612c10>
1400b136b: 48 85 c0                    	testq	%rax, %rax
1400b136e: 74 35                       	je	0x1400b13a5 <.text+0xb03a5>
1400b1370: 48 89 c7                    	movq	%rax, %rdi
1400b1373: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
1400b1377: 41 b8 68 01 00 00           	movl	$0x168, %r8d            # imm = 0x168
1400b137d: 48 89 c1                    	movq	%rax, %rcx
1400b1380: e8 7b 28 87 02              	callq	0x142923c00 <.text+0x2922c00>
1400b1385: 4c 8d 05 94 2d 88 02        	leaq	0x2882d94(%rip), %r8    # 0x142934120
1400b138c: 48 89 f1                    	movq	%rsi, %rcx
1400b138f: 48 89 fa                    	movq	%rdi, %rdx
1400b1392: e8 79 37 c3 00              	callq	0x140ce4b10 <.text+0xce3b10>
1400b1397: 48 89 f0                    	movq	%rsi, %rax
1400b139a: 48 81 c4 90 01 00 00        	addq	$0x190, %rsp            # imm = 0x190
1400b13a1: 5f                          	popq	%rdi
1400b13a2: 5e                          	popq	%rsi
1400b13a3: 5d                          	popq	%rbp
1400b13a4: c3                          	retq
1400b13a5: b9 08 00 00 00              	movl	$0x8, %ecx
1400b13aa: ba 68 01 00 00              	movl	$0x168, %edx            # imm = 0x168
1400b13af: e8 4c 7c 82 02              	callq	0x1428d9000 <.text+0x28d8000>
