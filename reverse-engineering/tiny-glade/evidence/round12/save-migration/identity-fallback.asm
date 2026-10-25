
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141673140: 55                          	pushq	%rbp
141673141: 56                          	pushq	%rsi
141673142: 57                          	pushq	%rdi
141673143: 53                          	pushq	%rbx
141673144: 48 81 ec 08 01 00 00        	subq	$0x108, %rsp            # imm = 0x108
14167314b: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
141673153: 48 c7 85 80 00 00 00 fe ff ff ff    	movq	$-0x2, 0x80(%rbp)
14167315e: 48 89 d7                    	movq	%rdx, %rdi
141673161: 48 89 cb                    	movq	%rcx, %rbx
141673164: e8 97 ec 6e ff              	callq	0x140d61e00 <.text+0xd60e00>
141673169: 48 89 45 f8                 	movq	%rax, -0x8(%rbp)
14167316d: 48 89 55 00                 	movq	%rdx, (%rbp)
141673171: 48 8d 4d f8                 	leaq	-0x8(%rbp), %rcx
141673175: e8 f6 78 de ff              	callq	0x14145aa70 <.text+0x1459a70>
14167317a: 48 89 c6                    	movq	%rax, %rsi
14167317d: 48 8d 4d f8                 	leaq	-0x8(%rbp), %rcx
141673181: 48 89 da                    	movq	%rbx, %rdx
141673184: 49 89 f8                    	movq	%rdi, %r8
141673187: e8 84 75 6f ff              	callq	0x140d6a710 <.text+0xd69710>
14167318c: 48 8b 4d f8                 	movq	-0x8(%rbp), %rcx
141673190: 48 8b 45 00                 	movq	(%rbp), %rax
141673194: 48 83 f9 02                 	cmpq	$0x2, %rcx
141673198: 75 3d                       	jne	0x1416731d7 <.text+0x16721d7>
14167319a: 89 c1                       	movl	%eax, %ecx
14167319c: 83 e1 03                    	andl	$0x3, %ecx
14167319f: 83 f9 01                    	cmpl	$0x1, %ecx
1416731a2: 0f 85 53 01 00 00           	jne	0x1416732fb <.text+0x16722fb>
1416731a8: 48 8d 48 ff                 	leaq	-0x1(%rax), %rcx
1416731ac: 48 89 4d 68                 	movq	%rcx, 0x68(%rbp)
1416731b0: 48 8b 48 ff                 	movq	-0x1(%rax), %rcx
1416731b4: 48 89 4d 70                 	movq	%rcx, 0x70(%rbp)
1416731b8: 48 8b 40 07                 	movq	0x7(%rax), %rax
1416731bc: 48 89 45 78                 	movq	%rax, 0x78(%rbp)
1416731c0: 48 8b 00                    	movq	(%rax), %rax
1416731c3: 48 85 c0                    	testq	%rax, %rax
1416731c6: 0f 84 01 01 00 00           	je	0x1416732cd <.text+0x16722cd>
1416731cc: 48 8b 4d 70                 	movq	0x70(%rbp), %rcx
1416731d0: ff d0                       	callq	*%rax
1416731d2: e9 f6 00 00 00              	jmp	0x1416732cd <.text+0x16722cd>
1416731d7: 48 8b 55 48                 	movq	0x48(%rbp), %rdx
1416731db: 48 89 55 f0                 	movq	%rdx, -0x10(%rbp)
1416731df: 0f 10 45 08                 	movups	0x8(%rbp), %xmm0
1416731e3: 0f 10 4d 18                 	movups	0x18(%rbp), %xmm1
1416731e7: 0f 10 55 28                 	movups	0x28(%rbp), %xmm2
1416731eb: 0f 10 5d 38                 	movups	0x38(%rbp), %xmm3
1416731ef: 0f 11 5d e0                 	movups	%xmm3, -0x20(%rbp)
1416731f3: 0f 11 55 d0                 	movups	%xmm2, -0x30(%rbp)
1416731f7: 0f 11 4d c0                 	movups	%xmm1, -0x40(%rbp)
1416731fb: 0f 11 45 b0                 	movups	%xmm0, -0x50(%rbp)
1416731ff: 48 89 4d a0                 	movq	%rcx, -0x60(%rbp)
141673203: 48 89 45 a8                 	movq	%rax, -0x58(%rbp)
141673207: 48 8b 45 d8                 	movq	-0x28(%rbp), %rax
14167320b: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
14167320f: 48 89 75 f8                 	movq	%rsi, -0x8(%rbp)
141673213: 48 8d 45 58                 	leaq	0x58(%rbp), %rax
141673217: 48 89 45 00                 	movq	%rax, (%rbp)
14167321b: 48 8d 4d f8                 	leaq	-0x8(%rbp), %rcx
14167321f: e8 bc 76 de ff              	callq	0x14145a8e0 <.text+0x14598e0>
141673224: 48 89 c6                    	movq	%rax, %rsi
141673227: 48 8d 4d 58                 	leaq	0x58(%rbp), %rcx
14167322b: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
14167322f: e8 2c 6c 6e ff              	callq	0x140d59e60 <.text+0xd58e60>
141673234: 8b 7d 58                    	movl	0x58(%rbp), %edi
141673237: 85 ff                       	testl	%edi, %edi
141673239: 74 3a                       	je	0x141673275 <.text+0x1672275>
14167323b: 48 8b 45 60                 	movq	0x60(%rbp), %rax
14167323f: 89 c1                       	movl	%eax, %ecx
141673241: 83 e1 03                    	andl	$0x3, %ecx
141673244: 83 f9 01                    	cmpl	$0x1, %ecx
141673247: 0f 85 ae 00 00 00           	jne	0x1416732fb <.text+0x16722fb>
14167324d: 48 8d 48 ff                 	leaq	-0x1(%rax), %rcx
141673251: 48 89 4d 68                 	movq	%rcx, 0x68(%rbp)
141673255: 48 8b 48 ff                 	movq	-0x1(%rax), %rcx
141673259: 48 89 4d 70                 	movq	%rcx, 0x70(%rbp)
14167325d: 48 8b 40 07                 	movq	0x7(%rax), %rax
141673261: 48 89 45 78                 	movq	%rax, 0x78(%rbp)
141673265: 48 8b 00                    	movq	(%rax), %rax
141673268: 48 85 c0                    	testq	%rax, %rax
14167326b: 74 60                       	je	0x1416732cd <.text+0x16722cd>
14167326d: 48 8b 4d 70                 	movq	0x70(%rbp), %rcx
141673271: ff d0                       	callq	*%rax
141673273: eb 58                       	jmp	0x1416732cd <.text+0x16722cd>
141673275: 48 8b 45 5c                 	movq	0x5c(%rbp), %rax
141673279: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
14167327d: 48 89 75 f8                 	movq	%rsi, -0x8(%rbp)
141673281: 48 8d 45 50                 	leaq	0x50(%rbp), %rax
141673285: 48 89 45 00                 	movq	%rax, (%rbp)
141673289: 48 8d 4d f8                 	leaq	-0x8(%rbp), %rcx
14167328d: e8 4e 76 de ff              	callq	0x14145a8e0 <.text+0x14598e0>
141673292: 48 89 c6                    	movq	%rax, %rsi
141673295: 85 ff                       	testl	%edi, %edi
141673297: 74 62                       	je	0x1416732fb <.text+0x16722fb>
141673299: 48 8b 45 60                 	movq	0x60(%rbp), %rax
14167329d: 89 c1                       	movl	%eax, %ecx
14167329f: 83 e1 03                    	andl	$0x3, %ecx
1416732a2: 83 f9 01                    	cmpl	$0x1, %ecx
1416732a5: 75 54                       	jne	0x1416732fb <.text+0x16722fb>
1416732a7: 48 8d 48 ff                 	leaq	-0x1(%rax), %rcx
1416732ab: 48 89 4d 68                 	movq	%rcx, 0x68(%rbp)
1416732af: 48 8b 48 ff                 	movq	-0x1(%rax), %rcx
1416732b3: 48 89 4d 70                 	movq	%rcx, 0x70(%rbp)
1416732b7: 48 8b 40 07                 	movq	0x7(%rax), %rax
1416732bb: 48 89 45 78                 	movq	%rax, 0x78(%rbp)
1416732bf: 48 8b 00                    	movq	(%rax), %rax
1416732c2: 48 85 c0                    	testq	%rax, %rax
1416732c5: 74 06                       	je	0x1416732cd <.text+0x16722cd>
1416732c7: 48 8b 4d 70                 	movq	0x70(%rbp), %rcx
1416732cb: ff d0                       	callq	*%rax
1416732cd: 48 8b 4d 70                 	movq	0x70(%rbp), %rcx
1416732d1: 48 8b 45 78                 	movq	0x78(%rbp), %rax
1416732d5: 48 8b 50 08                 	movq	0x8(%rax), %rdx
1416732d9: 48 85 d2                    	testq	%rdx, %rdx
1416732dc: 74 09                       	je	0x1416732e7 <.text+0x16722e7>
1416732de: 4c 8b 40 10                 	movq	0x10(%rax), %r8
1416732e2: e8 39 09 fa fe              	callq	0x140613c20 <.text+0x612c20>
1416732e7: ba 18 00 00 00              	movl	$0x18, %edx
1416732ec: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1416732f2: 48 8b 4d 68                 	movq	0x68(%rbp), %rcx
1416732f6: e8 25 09 fa fe              	callq	0x140613c20 <.text+0x612c20>
1416732fb: 48 89 f0                    	movq	%rsi, %rax
1416732fe: 48 81 c4 08 01 00 00        	addq	$0x108, %rsp            # imm = 0x108
141673305: 5b                          	popq	%rbx
141673306: 5f                          	popq	%rdi
141673307: 5e                          	popq	%rsi
141673308: 5d                          	popq	%rbp
141673309: c3                          	retq
14167330a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
141673310: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141673315: 55                          	pushq	%rbp
141673316: 56                          	pushq	%rsi
141673317: 57                          	pushq	%rdi
141673318: 53                          	pushq	%rbx
141673319: 48 83 ec 28                 	subq	$0x28, %rsp
14167331d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141673324: 48 8b 4d 70                 	movq	0x70(%rbp), %rcx
141673328: 48 8b 45 78                 	movq	0x78(%rbp), %rax
14167332c: 48 8b 50 08                 	movq	0x8(%rax), %rdx
141673330: 48 85 d2                    	testq	%rdx, %rdx
141673333: 74 0d                       	je	0x141673342 <.text+0x1672342>
141673335: 48 8b 45 78                 	movq	0x78(%rbp), %rax
141673339: 4c 8b 40 10                 	movq	0x10(%rax), %r8
14167333d: e8 de 08 fa fe              	callq	0x140613c20 <.text+0x612c20>
141673342: ba 18 00 00 00              	movl	$0x18, %edx
141673347: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14167334d: 48 8b 4d 68                 	movq	0x68(%rbp), %rcx
141673351: e8 ca 08 fa fe              	callq	0x140613c20 <.text+0x612c20>
141673356: 90                          	nop
141673357: 48 83 c4 28                 	addq	$0x28, %rsp
14167335b: 5b                          	popq	%rbx
14167335c: 5f                          	popq	%rdi
14167335d: 5e                          	popq	%rsi
14167335e: 5d                          	popq	%rbp
14167335f: c3                          	retq
141673360: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141673365: 55                          	pushq	%rbp
141673366: 56                          	pushq	%rsi
141673367: 57                          	pushq	%rdi
141673368: 53                          	pushq	%rbx
141673369: 48 83 ec 28                 	subq	$0x28, %rsp
14167336d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141673374: 48 8b 4d 70                 	movq	0x70(%rbp), %rcx
141673378: 48 8b 45 78                 	movq	0x78(%rbp), %rax
14167337c: 48 8b 50 08                 	movq	0x8(%rax), %rdx
141673380: 48 85 d2                    	testq	%rdx, %rdx
141673383: 74 0d                       	je	0x141673392 <.text+0x1672392>
141673385: 48 8b 45 78                 	movq	0x78(%rbp), %rax
141673389: 4c 8b 40 10                 	movq	0x10(%rax), %r8
14167338d: e8 8e 08 fa fe              	callq	0x140613c20 <.text+0x612c20>
141673392: ba 18 00 00 00              	movl	$0x18, %edx
141673397: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14167339d: 48 8b 4d 68                 	movq	0x68(%rbp), %rcx
1416733a1: e8 7a 08 fa fe              	callq	0x140613c20 <.text+0x612c20>
1416733a6: 90                          	nop
1416733a7: 48 83 c4 28                 	addq	$0x28, %rsp
1416733ab: 5b                          	popq	%rbx
1416733ac: 5f                          	popq	%rdi
1416733ad: 5e                          	popq	%rsi
1416733ae: 5d                          	popq	%rbp
1416733af: c3                          	retq
1416733b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1416733b5: 55                          	pushq	%rbp
1416733b6: 56                          	pushq	%rsi
1416733b7: 57                          	pushq	%rdi
1416733b8: 53                          	pushq	%rbx
1416733b9: 48 83 ec 28                 	subq	$0x28, %rsp
1416733bd: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1416733c4: 48 8b 4d 70                 	movq	0x70(%rbp), %rcx
1416733c8: 48 8b 45 78                 	movq	0x78(%rbp), %rax
1416733cc: 48 8b 50 08                 	movq	0x8(%rax), %rdx
1416733d0: 48 85 d2                    	testq	%rdx, %rdx
1416733d3: 74 0d                       	je	0x1416733e2 <.text+0x16723e2>
1416733d5: 48 8b 45 78                 	movq	0x78(%rbp), %rax
1416733d9: 4c 8b 40 10                 	movq	0x10(%rax), %r8
1416733dd: e8 3e 08 fa fe              	callq	0x140613c20 <.text+0x612c20>
1416733e2: ba 18 00 00 00              	movl	$0x18, %edx
1416733e7: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1416733ed: 48 8b 4d 68                 	movq	0x68(%rbp), %rcx
1416733f1: e8 2a 08 fa fe              	callq	0x140613c20 <.text+0x612c20>
1416733f6: 90                          	nop
1416733f7: 48 83 c4 28                 	addq	$0x28, %rsp
1416733fb: 5b                          	popq	%rbx
1416733fc: 5f                          	popq	%rdi
1416733fd: 5e                          	popq	%rsi
1416733fe: 5d                          	popq	%rbp
1416733ff: c3                          	retq
