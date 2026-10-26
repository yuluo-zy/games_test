; Original B RNE SHA256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03
; Bounded LLVM static evidence; disassembly range is not a Ghidra size-derived function boundary.

; SOURCE /tmp/key-assign.asm

B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14089b280: 0f b6 c2                    	movzbl	%dl, %eax
14089b283: 66 44 89 84 81 bc 01 00 00  	movw	%r8w, 0x1bc(%rcx,%rax,4)
14089b28c: 66 44 89 8c 81 be 01 00 00  	movw	%r9w, 0x1be(%rcx,%rax,4)
14089b295: c3                          	retq
14089b296: cc                          	int3
14089b297: cc                          	int3
14089b298: cc                          	int3
14089b299: cc                          	int3
14089b29a: cc                          	int3
14089b29b: cc                          	int3
14089b29c: cc                          	int3
14089b29d: cc                          	int3
14089b29e: cc                          	int3
14089b29f: cc                          	int3
14089b2a0: 41 57                       	pushq	%r15
14089b2a2: 41 56                       	pushq	%r14
14089b2a4: 41 55                       	pushq	%r13
14089b2a6: 41 54                       	pushq	%r12
14089b2a8: 56                          	pushq	%rsi
14089b2a9: 57                          	pushq	%rdi
14089b2aa: 55                          	pushq	%rbp
14089b2ab: 53                          	pushq	%rbx
14089b2ac: 48 81 ec e8 00 00 00        	subq	$0xe8, %rsp
14089b2b3: 88 54 24 07                 	movb	%dl, 0x7(%rsp)
14089b2b7: 4c 8d 91 50 01 00 00        	leaq	0x150(%rcx), %r10
14089b2be: 41 8b 30                    	movl	(%r8), %esi
14089b2c1: 45 0f b7 60 04              	movzwl	0x4(%r8), %r12d
14089b2c6: 41 0f b7 68 06              	movzwl	0x6(%r8), %ebp
14089b2cb: 4c 89 84 24 e0 00 00 00     	movq	%r8, 0xe0(%rsp)
14089b2d3: 45 8b 58 08                 	movl	0x8(%r8), %r11d
14089b2d7: 8b 81 50 01 00 00           	movl	0x150(%rcx), %eax
14089b2dd: 3d c3 00 00 00              	cmpl	$0xc3, %eax
14089b2e2: 0f 94 c2                    	sete	%dl
14089b2e5: 81 fe c3 00 00 00           	cmpl	$0xc3, %esi
14089b2eb: 41 0f 94 c1                 	sete	%r9b
14089b2ef: 41 08 d1                    	orb	%dl, %r9b
14089b2f2: 74 13                       	je	0x14089b307 <.text+0x89a307>
14089b2f4: 89 f2                       	movl	%esi, %edx
14089b2f6: 81 f2 c3 00 00 00           	xorl	$0xc3, %edx
14089b2fc: 35 c3 00 00 00              	xorl	$0xc3, %eax
14089b301: 09 d0                       	orl	%edx, %eax
14089b303: 74 0e                       	je	0x14089b313 <.text+0x89a313>
14089b305: eb 13                       	jmp	0x14089b31a <.text+0x89a31a>
14089b307: 39 f0                       	cmpl	%esi, %eax
14089b309: 75 0f                       	jne	0x14089b31a <.text+0x89a31a>
14089b30b: 85 f6                       	testl	%esi, %esi
14089b30d: 0f 84 f1 0a 00 00           	je	0x14089be04 <.text+0x89ae04>
14089b313: 41 c7 02 c3 00 00 00        	movl	$0xc3, (%r10)
14089b31a: 81 fe c3 00 00 00           	cmpl	$0xc3, %esi
14089b320: 0f 94 c2                    	sete	%dl
14089b323: 4c 8d 81 5c 01 00 00        	leaq	0x15c(%rcx), %r8
14089b32a: 8b 81 5c 01 00 00           	movl	0x15c(%rcx), %eax
14089b330: 3d c3 00 00 00              	cmpl	$0xc3, %eax
14089b335: 41 0f 94 c1                 	sete	%r9b
14089b339: 41 08 d1                    	orb	%dl, %r9b
14089b33c: 74 13                       	je	0x14089b351 <.text+0x89a351>
14089b33e: 35 c3 00 00 00              	xorl	$0xc3, %eax
14089b343: 89 f2                       	movl	%esi, %edx
14089b345: 81 f2 c3 00 00 00           	xorl	$0xc3, %edx
14089b34b: 09 c2                       	orl	%eax, %edx
14089b34d: 74 0e                       	je	0x14089b35d <.text+0x89a35d>
14089b34f: eb 13                       	jmp	0x14089b364 <.text+0x89a364>
14089b351: 39 f0                       	cmpl	%esi, %eax
14089b353: 75 0f                       	jne	0x14089b364 <.text+0x89a364>
14089b355: 85 f6                       	testl	%esi, %esi
14089b357: 0f 84 da 0a 00 00           	je	0x14089be37 <.text+0x89ae37>
14089b35d: 41 c7 00 c3 00 00 00        	movl	$0xc3, (%r8)
14089b364: 81 fe c3 00 00 00           	cmpl	$0xc3, %esi
14089b36a: 0f 94 c2                    	sete	%dl
14089b36d: 48 8d b9 68 01 00 00        	leaq	0x168(%rcx), %rdi
14089b374: 8b 81 68 01 00 00           	movl	0x168(%rcx), %eax
14089b37a: 3d c3 00 00 00              	cmpl	$0xc3, %eax
14089b37f: 41 0f 94 c1                 	sete	%r9b
14089b383: 41 08 d1                    	orb	%dl, %r9b
14089b386: 74 13                       	je	0x14089b39b <.text+0x89a39b>
14089b388: 35 c3 00 00 00              	xorl	$0xc3, %eax
14089b38d: 89 f2                       	movl	%esi, %edx
14089b38f: 81 f2 c3 00 00 00           	xorl	$0xc3, %edx
14089b395: 09 c2                       	orl	%eax, %edx
14089b397: 74 0e                       	je	0x14089b3a7 <.text+0x89a3a7>
14089b399: eb 12                       	jmp	0x14089b3ad <.text+0x89a3ad>
14089b39b: 39 f0                       	cmpl	%esi, %eax
14089b39d: 75 0e                       	jne	0x14089b3ad <.text+0x89a3ad>
14089b39f: 85 f6                       	testl	%esi, %esi
14089b3a1: 0f 84 c3 0a 00 00           	je	0x14089be6a <.text+0x89ae6a>
14089b3a7: c7 07 c3 00 00 00           	movl	$0xc3, (%rdi)
14089b3ad: 81 fe c3 00 00 00           	cmpl	$0xc3, %esi
14089b3b3: 0f 94 c2                    	sete	%dl
14089b3b6: 48 8d 99 74 01 00 00        	leaq	0x174(%rcx), %rbx
14089b3bd: 8b 81 74 01 00 00           	movl	0x174(%rcx), %eax
14089b3c3: 3d c3 00 00 00              	cmpl	$0xc3, %eax
14089b3c8: 41 0f 94 c1                 	sete	%r9b
14089b3cc: 41 08 d1                    	orb	%dl, %r9b
14089b3cf: 74 13                       	je	0x14089b3e4 <.text+0x89a3e4>
14089b3d1: 35 c3 00 00 00              	xorl	$0xc3, %eax
14089b3d6: 89 f2                       	movl	%esi, %edx
14089b3d8: 81 f2 c3 00 00 00           	xorl	$0xc3, %edx
14089b3de: 09 c2                       	orl	%eax, %edx
14089b3e0: 74 0e                       	je	0x14089b3f0 <.text+0x89a3f0>
14089b3e2: eb 12                       	jmp	0x14089b3f6 <.text+0x89a3f6>
14089b3e4: 39 f0                       	cmpl	%esi, %eax
14089b3e6: 75 0e                       	jne	0x14089b3f6 <.text+0x89a3f6>
14089b3e8: 85 f6                       	testl	%esi, %esi
14089b3ea: 0f 84 ad 0a 00 00           	je	0x14089be9d <.text+0x89ae9d>
14089b3f0: c7 03 c3 00 00 00           	movl	$0xc3, (%rbx)
14089b3f6: 81 fe c3 00 00 00           	cmpl	$0xc3, %esi
14089b3fc: 0f 94 c2                    	sete	%dl
14089b3ff: 4c 8d b1 80 01 00 00        	leaq	0x180(%rcx), %r14
14089b406: 8b 81 80 01 00 00           	movl	0x180(%rcx), %eax
14089b40c: 3d c3 00 00 00              	cmpl	$0xc3, %eax
14089b411: 41 0f 94 c1                 	sete	%r9b
14089b415: 41 08 d1                    	orb	%dl, %r9b
14089b418: 74 13                       	je	0x14089b42d <.text+0x89a42d>
14089b41a: 35 c3 00 00 00              	xorl	$0xc3, %eax
14089b41f: 89 f2                       	movl	%esi, %edx
14089b421: 81 f2 c3 00 00 00           	xorl	$0xc3, %edx
14089b427: 09 c2                       	orl	%eax, %edx
14089b429: 74 0e                       	je	0x14089b439 <.text+0x89a439>
14089b42b: eb 13                       	jmp	0x14089b440 <.text+0x89a440>
14089b42d: 39 f0                       	cmpl	%esi, %eax
14089b42f: 75 0f                       	jne	0x14089b440 <.text+0x89a440>
14089b431: 85 f6                       	testl	%esi, %esi
14089b433: 0f 84 97 0a 00 00           	je	0x14089bed0 <.text+0x89aed0>
14089b439: 41 c7 06 c3 00 00 00        	movl	$0xc3, (%r14)
14089b440: 81 fe c3 00 00 00           	cmpl	$0xc3, %esi
14089b446: 0f 94 c2                    	sete	%dl
14089b449: 4c 8d b9 8c 01 00 00        	leaq	0x18c(%rcx), %r15
14089b450: 8b 81 8c 01 00 00           	movl	0x18c(%rcx), %eax
14089b456: 3d c3 00 00 00              	cmpl	$0xc3, %eax
14089b45b: 41 0f 94 c1                 	sete	%r9b
14089b45f: 41 08 d1                    	orb	%dl, %r9b
14089b462: 74 13                       	je	0x14089b477 <.text+0x89a477>
14089b464: 35 c3 00 00 00              	xorl	$0xc3, %eax
14089b469: 89 f2                       	movl	%esi, %edx
14089b46b: 81 f2 c3 00 00 00           	xorl	$0xc3, %edx
14089b471: 09 c2                       	orl	%eax, %edx
14089b473: 74 0e                       	je	0x14089b483 <.text+0x89a483>
14089b475: eb 13                       	jmp	0x14089b48a <.text+0x89a48a>
14089b477: 39 f0                       	cmpl	%esi, %eax
14089b479: 75 0f                       	jne	0x14089b48a <.text+0x89a48a>
14089b47b: 85 f6                       	testl	%esi, %esi
14089b47d: 0f 84 80 0a 00 00           	je	0x14089bf03 <.text+0x89af03>
14089b483: 41 c7 07 c3 00 00 00        	movl	$0xc3, (%r15)
14089b48a: 81 fe c3 00 00 00           	cmpl	$0xc3, %esi
14089b490: 0f 94 c2                    	sete	%dl
14089b493: 4c 8d a9 98 01 00 00        	leaq	0x198(%rcx), %r13
14089b49a: 8b 81 98 01 00 00           	movl	0x198(%rcx), %eax
14089b4a0: 3d c3 00 00 00              	cmpl	$0xc3, %eax
14089b4a5: 41 0f 94 c1                 	sete	%r9b
14089b4a9: 41 08 d1                    	orb	%dl, %r9b
14089b4ac: 74 13                       	je	0x14089b4c1 <.text+0x89a4c1>
14089b4ae: 35 c3 00 00 00              	xorl	$0xc3, %eax
14089b4b3: 89 f2                       	movl	%esi, %edx
14089b4b5: 81 f2 c3 00 00 00           	xorl	$0xc3, %edx
14089b4bb: 09 c2                       	orl	%eax, %edx
14089b4bd: 74 0e                       	je	0x14089b4cd <.text+0x89a4cd>
14089b4bf: eb 14                       	jmp	0x14089b4d5 <.text+0x89a4d5>
14089b4c1: 39 f0                       	cmpl	%esi, %eax
14089b4c3: 75 10                       	jne	0x14089b4d5 <.text+0x89a4d5>
14089b4c5: 85 f6                       	testl	%esi, %esi
14089b4c7: 0f 84 69 0a 00 00           	je	0x14089bf36 <.text+0x89af36>
14089b4cd: 41 c7 45 00 c3 00 00 00     	movl	$0xc3, (%r13)
14089b4d5: 81 fe c3 00 00 00           	cmpl	$0xc3, %esi
14089b4db: 0f 94 c2                    	sete	%dl
14089b4de: 48 8d 81 a4 01 00 00        	leaq	0x1a4(%rcx), %rax
14089b4e5: 48 89 84 24 90 00 00 00     	movq	%rax, 0x90(%rsp)
14089b4ed: 8b 81 a4 01 00 00           	movl	0x1a4(%rcx), %eax
14089b4f3: 3d c3 00 00 00              	cmpl	$0xc3, %eax
14089b4f8: 41 0f 94 c1                 	sete	%r9b
14089b4fc: 41 08 d1                    	orb	%dl, %r9b
14089b4ff: 4c 89 94 24 d8 00 00 00     	movq	%r10, 0xd8(%rsp)
14089b507: 74 13                       	je	0x14089b51c <.text+0x89a51c>
14089b509: 35 c3 00 00 00              	xorl	$0xc3, %eax
14089b50e: 89 f2                       	movl	%esi, %edx
14089b510: 81 f2 c3 00 00 00           	xorl	$0xc3, %edx
14089b516: 09 c2                       	orl	%eax, %edx
14089b518: 74 0e                       	je	0x14089b528 <.text+0x89a528>
14089b51a: eb 1a                       	jmp	0x14089b536 <.text+0x89a536>
14089b51c: 39 f0                       	cmpl	%esi, %eax
14089b51e: 75 16                       	jne	0x14089b536 <.text+0x89a536>
14089b520: 85 f6                       	testl	%esi, %esi
14089b522: 0f 84 41 0a 00 00           	je	0x14089bf69 <.text+0x89af69>
14089b528: 48 8b 84 24 90 00 00 00     	movq	0x90(%rsp), %rax
14089b530: c7 00 c3 00 00 00           	movl	$0xc3, (%rax)
14089b536: 81 fe c3 00 00 00           	cmpl	$0xc3, %esi
14089b53c: 0f 94 c2                    	sete	%dl
14089b53f: 48 8d 41 30                 	leaq	0x30(%rcx), %rax
14089b543: 48 89 84 24 98 00 00 00     	movq	%rax, 0x98(%rsp)
14089b54b: 8b 41 30                    	movl	0x30(%rcx), %eax
14089b54e: 3d c3 00 00 00              	cmpl	$0xc3, %eax
14089b553: 41 0f 94 c2                 	sete	%r10b
14089b557: 41 08 d2                    	orb	%dl, %r10b
14089b55a: 74 13                       	je	0x14089b56f <.text+0x89a56f>
14089b55c: 35 c3 00 00 00              	xorl	$0xc3, %eax
14089b561: 89 f2                       	movl	%esi, %edx
14089b563: 81 f2 c3 00 00 00           	xorl	$0xc3, %edx
14089b569: 09 c2                       	orl	%eax, %edx
14089b56b: 74 0e                       	je	0x14089b57b <.text+0x89a57b>
14089b56d: eb 1a                       	jmp	0x14089b589 <.text+0x89a589>
14089b56f: 39 f0                       	cmpl	%esi, %eax
14089b571: 75 16                       	jne	0x14089b589 <.text+0x89a589>
14089b573: 85 f6                       	testl	%esi, %esi
14089b575: 0f 84 21 0a 00 00           	je	0x14089bf9c <.text+0x89af9c>
14089b57b: 48 8b 84 24 98 00 00 00     	movq	0x98(%rsp), %rax
14089b583: c7 00 c3 00 00 00           	movl	$0xc3, (%rax)
14089b589: 81 fe c3 00 00 00           	cmpl	$0xc3, %esi
14089b58f: 0f 94 c2                    	sete	%dl
14089b592: 4c 8d 49 3c                 	leaq	0x3c(%rcx), %r9
14089b596: 8b 41 3c                    	movl	0x3c(%rcx), %eax
14089b599: 3d c3 00 00 00              	cmpl	$0xc3, %eax
14089b59e: 41 0f 94 c2                 	sete	%r10b
14089b5a2: 41 08 d2                    	orb	%dl, %r10b
14089b5a5: 74 13                       	je	0x14089b5ba <.text+0x89a5ba>
14089b5a7: 35 c3 00 00 00              	xorl	$0xc3, %eax
14089b5ac: 89 f2                       	movl	%esi, %edx
14089b5ae: 81 f2 c3 00 00 00           	xorl	$0xc3, %edx
14089b5b4: 09 c2                       	orl	%eax, %edx
14089b5b6: 74 0e                       	je	0x14089b5c6 <.text+0x89a5c6>
14089b5b8: eb 13                       	jmp	0x14089b5cd <.text+0x89a5cd>
14089b5ba: 39 f0                       	cmpl	%esi, %eax
14089b5bc: 75 0f                       	jne	0x14089b5cd <.text+0x89a5cd>
14089b5be: 85 f6                       	testl	%esi, %esi
14089b5c0: 0f 84 03 0a 00 00           	je	0x14089bfc9 <.text+0x89afc9>
14089b5c6: 41 c7 01 c3 00 00 00        	movl	$0xc3, (%r9)
14089b5cd: 81 fe c3 00 00 00           	cmpl	$0xc3, %esi
14089b5d3: 0f 94 c2                    	sete	%dl
14089b5d6: 48 8d 41 18                 	leaq	0x18(%rcx), %rax
14089b5da: 48 89 84 24 88 00 00 00     	movq	%rax, 0x88(%rsp)
14089b5e2: 8b 41 18                    	movl	0x18(%rcx), %eax
14089b5e5: 3d c3 00 00 00              	cmpl	$0xc3, %eax
14089b5ea: 41 0f 94 c2                 	sete	%r10b
14089b5ee: 41 08 d2                    	orb	%dl, %r10b
14089b5f1: 74 13                       	je	0x14089b606 <.text+0x89a606>
14089b5f3: 35 c3 00 00 00              	xorl	$0xc3, %eax
14089b5f8: 89 f2                       	movl	%esi, %edx
14089b5fa: 81 f2 c3 00 00 00           	xorl	$0xc3, %edx
14089b600: 09 c2                       	orl	%eax, %edx
14089b602: 74 0e                       	je	0x14089b612 <.text+0x89a612>
14089b604: eb 1a                       	jmp	0x14089b620 <.text+0x89a620>
14089b606: 39 f0                       	cmpl	%esi, %eax
14089b608: 75 16                       	jne	0x14089b620 <.text+0x89a620>
14089b60a: 85 f6                       	testl	%esi, %esi
14089b60c: 0f 84 e4 09 00 00           	je	0x14089bff6 <.text+0x89aff6>
14089b612: 48 8b 84 24 88 00 00 00     	movq	0x88(%rsp), %rax
14089b61a: c7 00 c3 00 00 00           	movl	$0xc3, (%rax)
14089b620: 81 fe c3 00 00 00           	cmpl	$0xc3, %esi
14089b626: 0f 94 c2                    	sete	%dl
14089b629: 48 8d 41 24                 	leaq	0x24(%rcx), %rax
14089b62d: 48 89 84 24 80 00 00 00     	movq	%rax, 0x80(%rsp)
14089b635: 8b 41 24                    	movl	0x24(%rcx), %eax
14089b638: 3d c3 00 00 00              	cmpl	$0xc3, %eax
14089b63d: 41 0f 94 c2                 	sete	%r10b
14089b641: 41 08 d2                    	orb	%dl, %r10b
14089b644: 74 13                       	je	0x14089b659 <.text+0x89a659>
14089b646: 35 c3 00 00 00              	xorl	$0xc3, %eax
14089b64b: 89 f2                       	movl	%esi, %edx
14089b64d: 81 f2 c3 00 00 00           	xorl	$0xc3, %edx
14089b653: 09 c2                       	orl	%eax, %edx
14089b655: 74 0e                       	je	0x14089b665 <.text+0x89a665>
14089b657: eb 1a                       	jmp	0x14089b673 <.text+0x89a673>
14089b659: 39 f0                       	cmpl	%esi, %eax
14089b65b: 75 16                       	jne	0x14089b673 <.text+0x89a673>
14089b65d: 85 f6                       	testl	%esi, %esi
14089b65f: 0f 84 be 09 00 00           	je	0x14089c023 <.text+0x89b023>
14089b665: 48 8b 84 24 80 00 00 00     	movq	0x80(%rsp), %rax
14089b66d: c7 00 c3 00 00 00           	movl	$0xc3, (%rax)
14089b673: 81 fe c3 00 00 00           	cmpl	$0xc3, %esi
14089b679: 0f 94 c2                    	sete	%dl
14089b67c: 8b 01                       	movl	(%rcx), %eax
14089b67e: 3d c3 00 00 00              	cmpl	$0xc3, %eax
14089b683: 41 0f 94 c2                 	sete	%r10b
14089b687: 41 08 d2                    	orb	%dl, %r10b
14089b68a: 74 13                       	je	0x14089b69f <.text+0x89a69f>
14089b68c: 35 c3 00 00 00              	xorl	$0xc3, %eax
14089b691: 89 f2                       	movl	%esi, %edx
14089b693: 81 f2 c3 00 00 00           	xorl	$0xc3, %edx
14089b699: 09 c2                       	orl	%eax, %edx
14089b69b: 74 0e                       	je	0x14089b6ab <.text+0x89a6ab>
14089b69d: eb 12                       	jmp	0x14089b6b1 <.text+0x89a6b1>
14089b69f: 39 f0                       	cmpl	%esi, %eax
14089b6a1: 75 0e                       	jne	0x14089b6b1 <.text+0x89a6b1>
14089b6a3: 85 f6                       	testl	%esi, %esi
14089b6a5: 0f 84 a5 09 00 00           	je	0x14089c050 <.text+0x89b050>
14089b6ab: c7 01 c3 00 00 00           	movl	$0xc3, (%rcx)
14089b6b1: 81 fe c3 00 00 00           	cmpl	$0xc3, %esi
14089b6b7: 0f 94 c2                    	sete	%dl
14089b6ba: 48 8d 41 0c                 	leaq	0xc(%rcx), %rax
14089b6be: 48 89 44 24 78              	movq	%rax, 0x78(%rsp)
14089b6c3: 8b 41 0c                    	movl	0xc(%rcx), %eax
14089b6c6: 3d c3 00 00 00              	cmpl	$0xc3, %eax
14089b6cb: 41 0f 94 c2                 	sete	%r10b
14089b6cf: 41 08 d2                    	orb	%dl, %r10b
14089b6d2: 74 13                       	je	0x14089b6e7 <.text+0x89a6e7>
14089b6d4: 35 c3 00 00 00              	xorl	$0xc3, %eax
14089b6d9: 89 f2                       	movl	%esi, %edx
14089b6db: 81 f2 c3 00 00 00           	xorl	$0xc3, %edx
14089b6e1: 09 c2                       	orl	%eax, %edx
14089b6e3: 74 0e                       	je	0x14089b6f3 <.text+0x89a6f3>
14089b6e5: eb 17                       	jmp	0x14089b6fe <.text+0x89a6fe>
14089b6e7: 39 f0                       	cmpl	%esi, %eax
14089b6e9: 75 13                       	jne	0x14089b6fe <.text+0x89a6fe>
14089b6eb: 85 f6                       	testl	%esi, %esi
14089b6ed: 0f 84 8a 09 00 00           	je	0x14089c07d <.text+0x89b07d>
14089b6f3: 48 8b 44 24 78              	movq	0x78(%rsp), %rax
14089b6f8: c7 00 c3 00 00 00           	movl	$0xc3, (%rax)
14089b6fe: 81 fe c3 00 00 00           	cmpl	$0xc3, %esi
14089b704: 0f 94 c2                    	sete	%dl
14089b707: 48 8d 41 48                 	leaq	0x48(%rcx), %rax
14089b70b: 48 89 44 24 70              	movq	%rax, 0x70(%rsp)
14089b710: 8b 41 48                    	movl	0x48(%rcx), %eax
14089b713: 3d c3 00 00 00              	cmpl	$0xc3, %eax
14089b718: 41 0f 94 c2                 	sete	%r10b
14089b71c: 41 08 d2                    	orb	%dl, %r10b
14089b71f: 74 13                       	je	0x14089b734 <.text+0x89a734>
14089b721: 35 c3 00 00 00              	xorl	$0xc3, %eax
14089b726: 89 f2                       	movl	%esi, %edx
14089b728: 81 f2 c3 00 00 00           	xorl	$0xc3, %edx
14089b72e: 09 c2                       	orl	%eax, %edx
14089b730: 74 0e                       	je	0x14089b740 <.text+0x89a740>
14089b732: eb 17                       	jmp	0x14089b74b <.text+0x89a74b>
14089b734: 39 f0                       	cmpl	%esi, %eax
14089b736: 75 13                       	jne	0x14089b74b <.text+0x89a74b>
14089b738: 85 f6                       	testl	%esi, %esi
14089b73a: 0f 84 6a 09 00 00           	je	0x14089c0aa <.text+0x89b0aa>
14089b740: 48 8b 44 24 70              	movq	0x70(%rsp), %rax
14089b745: c7 00 c3 00 00 00           	movl	$0xc3, (%rax)
14089b74b: 81 fe c3 00 00 00           	cmpl	$0xc3, %esi
14089b751: 0f 94 c2                    	sete	%dl
14089b754: 48 8d 41 54                 	leaq	0x54(%rcx), %rax
14089b758: 48 89 44 24 68              	movq	%rax, 0x68(%rsp)
14089b75d: 8b 41 54                    	movl	0x54(%rcx), %eax
14089b760: 3d c3 00 00 00              	cmpl	$0xc3, %eax
14089b765: 41 0f 94 c2                 	sete	%r10b
14089b769: 41 08 d2                    	orb	%dl, %r10b
14089b76c: 74 13                       	je	0x14089b781 <.text+0x89a781>
14089b76e: 35 c3 00 00 00              	xorl	$0xc3, %eax
14089b773: 89 f2                       	movl	%esi, %edx
14089b775: 81 f2 c3 00 00 00           	xorl	$0xc3, %edx
14089b77b: 09 c2                       	orl	%eax, %edx
14089b77d: 74 0e                       	je	0x14089b78d <.text+0x89a78d>
14089b77f: eb 17                       	jmp	0x14089b798 <.text+0x89a798>
14089b781: 39 f0                       	cmpl	%esi, %eax
14089b783: 75 13                       	jne	0x14089b798 <.text+0x89a798>
14089b785: 85 f6                       	testl	%esi, %esi
14089b787: 0f 84 4a 09 00 00           	je	0x14089c0d7 <.text+0x89b0d7>
14089b78d: 48 8b 44 24 68              	movq	0x68(%rsp), %rax
14089b792: c7 00 c3 00 00 00           	movl	$0xc3, (%rax)
14089b798: 81 fe c3 00 00 00           	cmpl	$0xc3, %esi
14089b79e: 0f 94 c2                    	sete	%dl
14089b7a1: 48 8d 41 60                 	leaq	0x60(%rcx), %rax
14089b7a5: 48 89 44 24 60              	movq	%rax, 0x60(%rsp)
14089b7aa: 8b 41 60                    	movl	0x60(%rcx), %eax
14089b7ad: 3d c3 00 00 00              	cmpl	$0xc3, %eax
14089b7b2: 41 0f 94 c2                 	sete	%r10b
14089b7b6: 41 08 d2                    	orb	%dl, %r10b
14089b7b9: 74 13                       	je	0x14089b7ce <.text+0x89a7ce>
14089b7bb: 35 c3 00 00 00              	xorl	$0xc3, %eax
14089b7c0: 89 f2                       	movl	%esi, %edx
14089b7c2: 81 f2 c3 00 00 00           	xorl	$0xc3, %edx
14089b7c8: 09 c2                       	orl	%eax, %edx
14089b7ca: 74 0e                       	je	0x14089b7da <.text+0x89a7da>
14089b7cc: eb 17                       	jmp	0x14089b7e5 <.text+0x89a7e5>
14089b7ce: 39 f0                       	cmpl	%esi, %eax
14089b7d0: 75 13                       	jne	0x14089b7e5 <.text+0x89a7e5>
14089b7d2: 85 f6                       	testl	%esi, %esi
14089b7d4: 0f 84 2a 09 00 00           	je	0x14089c104 <.text+0x89b104>
14089b7da: 48 8b 44 24 60              	movq	0x60(%rsp), %rax
14089b7df: c7 00 c3 00 00 00           	movl	$0xc3, (%rax)
14089b7e5: 81 fe c3 00 00 00           	cmpl	$0xc3, %esi
14089b7eb: 0f 94 c2                    	sete	%dl
14089b7ee: 48 8d 41 6c                 	leaq	0x6c(%rcx), %rax
14089b7f2: 48 89 44 24 58              	movq	%rax, 0x58(%rsp)
14089b7f7: 8b 41 6c                    	movl	0x6c(%rcx), %eax
14089b7fa: 3d c3 00 00 00              	cmpl	$0xc3, %eax
14089b7ff: 41 0f 94 c2                 	sete	%r10b
14089b803: 41 08 d2                    	orb	%dl, %r10b
14089b806: 74 13                       	je	0x14089b81b <.text+0x89a81b>
14089b808: 35 c3 00 00 00              	xorl	$0xc3, %eax
14089b80d: 89 f2                       	movl	%esi, %edx
14089b80f: 81 f2 c3 00 00 00           	xorl	$0xc3, %edx
14089b815: 09 c2                       	orl	%eax, %edx
14089b817: 74 0e                       	je	0x14089b827 <.text+0x89a827>
14089b819: eb 17                       	jmp	0x14089b832 <.text+0x89a832>
14089b81b: 39 f0                       	cmpl	%esi, %eax
14089b81d: 75 13                       	jne	0x14089b832 <.text+0x89a832>
14089b81f: 85 f6                       	testl	%esi, %esi
14089b821: 0f 84 0a 09 00 00           	je	0x14089c131 <.text+0x89b131>
14089b827: 48 8b 44 24 58              	movq	0x58(%rsp), %rax
14089b82c: c7 00 c3 00 00 00           	movl	$0xc3, (%rax)
14089b832: 81 fe c3 00 00 00           	cmpl	$0xc3, %esi
14089b838: 0f 94 c2                    	sete	%dl
14089b83b: 48 8d 41 78                 	leaq	0x78(%rcx), %rax
14089b83f: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
14089b844: 8b 41 78                    	movl	0x78(%rcx), %eax
14089b847: 3d c3 00 00 00              	cmpl	$0xc3, %eax
14089b84c: 41 0f 94 c2                 	sete	%r10b
14089b850: 41 08 d2                    	orb	%dl, %r10b
14089b853: 74 13                       	je	0x14089b868 <.text+0x89a868>
14089b855: 35 c3 00 00 00              	xorl	$0xc3, %eax
14089b85a: 89 f2                       	movl	%esi, %edx
14089b85c: 81 f2 c3 00 00 00           	xorl	$0xc3, %edx
14089b862: 09 c2                       	orl	%eax, %edx
14089b864: 74 0e                       	je	0x14089b874 <.text+0x89a874>
14089b866: eb 17                       	jmp	0x14089b87f <.text+0x89a87f>
14089b868: 39 f0                       	cmpl	%esi, %eax
14089b86a: 75 13                       	jne	0x14089b87f <.text+0x89a87f>
14089b86c: 85 f6                       	testl	%esi, %esi
14089b86e: 0f 84 ea 08 00 00           	je	0x14089c15e <.text+0x89b15e>
14089b874: 48 8b 44 24 50              	movq	0x50(%rsp), %rax
14089b879: c7 00 c3 00 00 00           	movl	$0xc3, (%rax)
14089b87f: 81 fe c3 00 00 00           	cmpl	$0xc3, %esi
14089b885: 0f 94 c2                    	sete	%dl
14089b888: 48 8d 81 84 00 00 00        	leaq	0x84(%rcx), %rax
14089b88f: 48 89 44 24 48              	movq	%rax, 0x48(%rsp)
14089b894: 8b 81 84 00 00 00           	movl	0x84(%rcx), %eax
14089b89a: 3d c3 00 00 00              	cmpl	$0xc3, %eax
14089b89f: 41 0f 94 c2                 	sete	%r10b
14089b8a3: 41 08 d2                    	orb	%dl, %r10b
14089b8a6: 74 13                       	je	0x14089b8bb <.text+0x89a8bb>
14089b8a8: 35 c3 00 00 00              	xorl	$0xc3, %eax
14089b8ad: 89 f2                       	movl	%esi, %edx
14089b8af: 81 f2 c3 00 00 00           	xorl	$0xc3, %edx
14089b8b5: 09 c2                       	orl	%eax, %edx
14089b8b7: 74 0e                       	je	0x14089b8c7 <.text+0x89a8c7>
14089b8b9: eb 17                       	jmp	0x14089b8d2 <.text+0x89a8d2>
14089b8bb: 39 f0                       	cmpl	%esi, %eax
14089b8bd: 75 13                       	jne	0x14089b8d2 <.text+0x89a8d2>
14089b8bf: 85 f6                       	testl	%esi, %esi
14089b8c1: 0f 84 c7 08 00 00           	je	0x14089c18e <.text+0x89b18e>
14089b8c7: 48 8b 44 24 48              	movq	0x48(%rsp), %rax
14089b8cc: c7 00 c3 00 00 00           	movl	$0xc3, (%rax)
14089b8d2: 81 fe c3 00 00 00           	cmpl	$0xc3, %esi
14089b8d8: 0f 94 c2                    	sete	%dl
14089b8db: 48 8d 81 90 00 00 00        	leaq	0x90(%rcx), %rax
14089b8e2: 48 89 44 24 40              	movq	%rax, 0x40(%rsp)
14089b8e7: 8b 81 90 00 00 00           	movl	0x90(%rcx), %eax
14089b8ed: 3d c3 00 00 00              	cmpl	$0xc3, %eax
14089b8f2: 41 0f 94 c2                 	sete	%r10b
14089b8f6: 41 08 d2                    	orb	%dl, %r10b
14089b8f9: 74 13                       	je	0x14089b90e <.text+0x89a90e>
14089b8fb: 35 c3 00 00 00              	xorl	$0xc3, %eax
14089b900: 89 f2                       	movl	%esi, %edx
14089b902: 81 f2 c3 00 00 00           	xorl	$0xc3, %edx
14089b908: 09 c2                       	orl	%eax, %edx
14089b90a: 74 0e                       	je	0x14089b91a <.text+0x89a91a>
14089b90c: eb 17                       	jmp	0x14089b925 <.text+0x89a925>
14089b90e: 39 f0                       	cmpl	%esi, %eax
14089b910: 75 13                       	jne	0x14089b925 <.text+0x89a925>
14089b912: 85 f6                       	testl	%esi, %esi
14089b914: 0f 84 a7 08 00 00           	je	0x14089c1c1 <.text+0x89b1c1>
14089b91a: 48 8b 44 24 40              	movq	0x40(%rsp), %rax
14089b91f: c7 00 c3 00 00 00           	movl	$0xc3, (%rax)
14089b925: 81 fe c3 00 00 00           	cmpl	$0xc3, %esi
14089b92b: 0f 94 c2                    	sete	%dl
14089b92e: 48 8d 81 9c 00 00 00        	leaq	0x9c(%rcx), %rax
14089b935: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
14089b93a: 8b 81 9c 00 00 00           	movl	0x9c(%rcx), %eax
14089b940: 3d c3 00 00 00              	cmpl	$0xc3, %eax
14089b945: 41 0f 94 c2                 	sete	%r10b
14089b949: 41 08 d2                    	orb	%dl, %r10b
14089b94c: 74 13                       	je	0x14089b961 <.text+0x89a961>
14089b94e: 35 c3 00 00 00              	xorl	$0xc3, %eax
14089b953: 89 f2                       	movl	%esi, %edx
14089b955: 81 f2 c3 00 00 00           	xorl	$0xc3, %edx
14089b95b: 09 c2                       	orl	%eax, %edx
14089b95d: 74 0e                       	je	0x14089b96d <.text+0x89a96d>
14089b95f: eb 17                       	jmp	0x14089b978 <.text+0x89a978>
14089b961: 39 f0                       	cmpl	%esi, %eax
14089b963: 75 13                       	jne	0x14089b978 <.text+0x89a978>
14089b965: 85 f6                       	testl	%esi, %esi
14089b967: 0f 84 87 08 00 00           	je	0x14089c1f4 <.text+0x89b1f4>
14089b96d: 48 8b 44 24 38              	movq	0x38(%rsp), %rax
14089b972: c7 00 c3 00 00 00           	movl	$0xc3, (%rax)
14089b978: 81 fe c3 00 00 00           	cmpl	$0xc3, %esi
14089b97e: 0f 94 c2                    	sete	%dl
14089b981: 48 8d 81 a8 00 00 00        	leaq	0xa8(%rcx), %rax
14089b988: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
14089b98d: 8b 81 a8 00 00 00           	movl	0xa8(%rcx), %eax
14089b993: 3d c3 00 00 00              	cmpl	$0xc3, %eax
14089b998: 41 0f 94 c2                 	sete	%r10b
14089b99c: 41 08 d2                    	orb	%dl, %r10b
14089b99f: 74 13                       	je	0x14089b9b4 <.text+0x89a9b4>
14089b9a1: 35 c3 00 00 00              	xorl	$0xc3, %eax
14089b9a6: 89 f2                       	movl	%esi, %edx
14089b9a8: 81 f2 c3 00 00 00           	xorl	$0xc3, %edx
14089b9ae: 09 c2                       	orl	%eax, %edx
14089b9b0: 74 0e                       	je	0x14089b9c0 <.text+0x89a9c0>
14089b9b2: eb 17                       	jmp	0x14089b9cb <.text+0x89a9cb>
14089b9b4: 39 f0                       	cmpl	%esi, %eax
14089b9b6: 75 13                       	jne	0x14089b9cb <.text+0x89a9cb>
14089b9b8: 85 f6                       	testl	%esi, %esi
14089b9ba: 0f 84 67 08 00 00           	je	0x14089c227 <.text+0x89b227>
14089b9c0: 48 8b 44 24 30              	movq	0x30(%rsp), %rax
14089b9c5: c7 00 c3 00 00 00           	movl	$0xc3, (%rax)
14089b9cb: 81 fe c3 00 00 00           	cmpl	$0xc3, %esi
14089b9d1: 0f 94 c2                    	sete	%dl
14089b9d4: 48 8d 81 b4 00 00 00        	leaq	0xb4(%rcx), %rax
14089b9db: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
14089b9e0: 8b 81 b4 00 00 00           	movl	0xb4(%rcx), %eax
14089b9e6: 3d c3 00 00 00              	cmpl	$0xc3, %eax
14089b9eb: 41 0f 94 c2                 	sete	%r10b
14089b9ef: 41 08 d2                    	orb	%dl, %r10b
14089b9f2: 74 13                       	je	0x14089ba07 <.text+0x89aa07>
14089b9f4: 35 c3 00 00 00              	xorl	$0xc3, %eax
14089b9f9: 89 f2                       	movl	%esi, %edx
14089b9fb: 81 f2 c3 00 00 00           	xorl	$0xc3, %edx
14089ba01: 09 c2                       	orl	%eax, %edx
14089ba03: 74 0e                       	je	0x14089ba13 <.text+0x89aa13>
14089ba05: eb 17                       	jmp	0x14089ba1e <.text+0x89aa1e>
14089ba07: 39 f0                       	cmpl	%esi, %eax
14089ba09: 75 13                       	jne	0x14089ba1e <.text+0x89aa1e>
14089ba0b: 85 f6                       	testl	%esi, %esi
14089ba0d: 0f 84 47 08 00 00           	je	0x14089c25a <.text+0x89b25a>
14089ba13: 48 8b 44 24 28              	movq	0x28(%rsp), %rax
14089ba18: c7 00 c3 00 00 00           	movl	$0xc3, (%rax)
14089ba1e: 81 fe c3 00 00 00           	cmpl	$0xc3, %esi
14089ba24: 0f 94 c2                    	sete	%dl
14089ba27: 48 8d 81 c0 00 00 00        	leaq	0xc0(%rcx), %rax
14089ba2e: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14089ba33: 8b 81 c0 00 00 00           	movl	0xc0(%rcx), %eax
14089ba39: 3d c3 00 00 00              	cmpl	$0xc3, %eax
14089ba3e: 41 0f 94 c2                 	sete	%r10b
14089ba42: 41 08 d2                    	orb	%dl, %r10b
14089ba45: 74 13                       	je	0x14089ba5a <.text+0x89aa5a>
14089ba47: 35 c3 00 00 00              	xorl	$0xc3, %eax
14089ba4c: 89 f2                       	movl	%esi, %edx
14089ba4e: 81 f2 c3 00 00 00           	xorl	$0xc3, %edx
14089ba54: 09 c2                       	orl	%eax, %edx
14089ba56: 74 0e                       	je	0x14089ba66 <.text+0x89aa66>
14089ba58: eb 17                       	jmp	0x14089ba71 <.text+0x89aa71>
14089ba5a: 39 f0                       	cmpl	%esi, %eax
14089ba5c: 75 13                       	jne	0x14089ba71 <.text+0x89aa71>
14089ba5e: 85 f6                       	testl	%esi, %esi
14089ba60: 0f 84 27 08 00 00           	je	0x14089c28d <.text+0x89b28d>
14089ba66: 48 8b 44 24 20              	movq	0x20(%rsp), %rax
14089ba6b: c7 00 c3 00 00 00           	movl	$0xc3, (%rax)
14089ba71: 81 fe c3 00 00 00           	cmpl	$0xc3, %esi
14089ba77: 0f 94 c2                    	sete	%dl
14089ba7a: 48 8d 81 cc 00 00 00        	leaq	0xcc(%rcx), %rax
14089ba81: 48 89 44 24 18              	movq	%rax, 0x18(%rsp)
14089ba86: 8b 81 cc 00 00 00           	movl	0xcc(%rcx), %eax
14089ba8c: 3d c3 00 00 00              	cmpl	$0xc3, %eax
14089ba91: 41 0f 94 c2                 	sete	%r10b
14089ba95: 41 08 d2                    	orb	%dl, %r10b
14089ba98: 74 13                       	je	0x14089baad <.text+0x89aaad>
14089ba9a: 35 c3 00 00 00              	xorl	$0xc3, %eax
14089ba9f: 89 f2                       	movl	%esi, %edx
14089baa1: 81 f2 c3 00 00 00           	xorl	$0xc3, %edx
14089baa7: 09 c2                       	orl	%eax, %edx
14089baa9: 74 0e                       	je	0x14089bab9 <.text+0x89aab9>
14089baab: eb 17                       	jmp	0x14089bac4 <.text+0x89aac4>
14089baad: 39 f0                       	cmpl	%esi, %eax
14089baaf: 75 13                       	jne	0x14089bac4 <.text+0x89aac4>
14089bab1: 85 f6                       	testl	%esi, %esi
14089bab3: 0f 84 07 08 00 00           	je	0x14089c2c0 <.text+0x89b2c0>
14089bab9: 48 8b 44 24 18              	movq	0x18(%rsp), %rax
14089babe: c7 00 c3 00 00 00           	movl	$0xc3, (%rax)
14089bac4: 81 fe c3 00 00 00           	cmpl	$0xc3, %esi
14089baca: 0f 94 c2                    	sete	%dl
14089bacd: 48 8d 81 e4 00 00 00        	leaq	0xe4(%rcx), %rax
14089bad4: 48 89 44 24 10              	movq	%rax, 0x10(%rsp)
14089bad9: 8b 81 e4 00 00 00           	movl	0xe4(%rcx), %eax
14089badf: 3d c3 00 00 00              	cmpl	$0xc3, %eax
14089bae4: 41 0f 94 c2                 	sete	%r10b
14089bae8: 41 08 d2                    	orb	%dl, %r10b
14089baeb: 74 13                       	je	0x14089bb00 <.text+0x89ab00>
14089baed: 35 c3 00 00 00              	xorl	$0xc3, %eax
14089baf2: 89 f2                       	movl	%esi, %edx
14089baf4: 81 f2 c3 00 00 00           	xorl	$0xc3, %edx
14089bafa: 09 c2                       	orl	%eax, %edx
14089bafc: 74 0e                       	je	0x14089bb0c <.text+0x89ab0c>
14089bafe: eb 17                       	jmp	0x14089bb17 <.text+0x89ab17>
14089bb00: 39 f0                       	cmpl	%esi, %eax
14089bb02: 75 13                       	jne	0x14089bb17 <.text+0x89ab17>
14089bb04: 85 f6                       	testl	%esi, %esi
14089bb06: 0f 84 e7 07 00 00           	je	0x14089c2f3 <.text+0x89b2f3>
14089bb0c: 48 8b 44 24 10              	movq	0x10(%rsp), %rax
14089bb11: c7 00 c3 00 00 00           	movl	$0xc3, (%rax)
14089bb17: 81 fe c3 00 00 00           	cmpl	$0xc3, %esi
14089bb1d: 0f 94 c2                    	sete	%dl
14089bb20: 48 8d 81 f0 00 00 00        	leaq	0xf0(%rcx), %rax
14089bb27: 48 89 44 24 08              	movq	%rax, 0x8(%rsp)
14089bb2c: 8b 81 f0 00 00 00           	movl	0xf0(%rcx), %eax
14089bb32: 3d c3 00 00 00              	cmpl	$0xc3, %eax
14089bb37: 41 0f 94 c2                 	sete	%r10b
14089bb3b: 41 08 d2                    	orb	%dl, %r10b
14089bb3e: 4c 89 84 24 d0 00 00 00     	movq	%r8, 0xd0(%rsp)
14089bb46: 74 13                       	je	0x14089bb5b <.text+0x89ab5b>
14089bb48: 35 c3 00 00 00              	xorl	$0xc3, %eax
14089bb4d: 89 f2                       	movl	%esi, %edx
14089bb4f: 81 f2 c3 00 00 00           	xorl	$0xc3, %edx
14089bb55: 09 c2                       	orl	%eax, %edx
14089bb57: 74 0e                       	je	0x14089bb67 <.text+0x89ab67>
14089bb59: eb 17                       	jmp	0x14089bb72 <.text+0x89ab72>
14089bb5b: 39 f0                       	cmpl	%esi, %eax
14089bb5d: 75 13                       	jne	0x14089bb72 <.text+0x89ab72>
14089bb5f: 85 f6                       	testl	%esi, %esi
14089bb61: 0f 84 bf 07 00 00           	je	0x14089c326 <.text+0x89b326>
14089bb67: 48 8b 44 24 08              	movq	0x8(%rsp), %rax
14089bb6c: c7 00 c3 00 00 00           	movl	$0xc3, (%rax)
14089bb72: 81 fe c3 00 00 00           	cmpl	$0xc3, %esi
14089bb78: 0f 94 c2                    	sete	%dl
14089bb7b: 4c 8d 81 fc 00 00 00        	leaq	0xfc(%rcx), %r8
14089bb82: 8b 81 fc 00 00 00           	movl	0xfc(%rcx), %eax
14089bb88: 3d c3 00 00 00              	cmpl	$0xc3, %eax
14089bb8d: 41 0f 94 c2                 	sete	%r10b
14089bb91: 41 08 d2                    	orb	%dl, %r10b
14089bb94: 48 89 9c 24 c0 00 00 00     	movq	%rbx, 0xc0(%rsp)
14089bb9c: 74 13                       	je	0x14089bbb1 <.text+0x89abb1>
14089bb9e: 35 c3 00 00 00              	xorl	$0xc3, %eax
14089bba3: 89 f2                       	movl	%esi, %edx
14089bba5: 81 f2 c3 00 00 00           	xorl	$0xc3, %edx
14089bbab: 09 c2                       	orl	%eax, %edx
14089bbad: 74 0e                       	je	0x14089bbbd <.text+0x89abbd>
14089bbaf: eb 13                       	jmp	0x14089bbc4 <.text+0x89abc4>
14089bbb1: 39 f0                       	cmpl	%esi, %eax
14089bbb3: 75 0f                       	jne	0x14089bbc4 <.text+0x89abc4>
14089bbb5: 85 f6                       	testl	%esi, %esi
14089bbb7: 0f 84 9c 07 00 00           	je	0x14089c359 <.text+0x89b359>
14089bbbd: 41 c7 00 c3 00 00 00        	movl	$0xc3, (%r8)
14089bbc4: 81 fe c3 00 00 00           	cmpl	$0xc3, %esi
14089bbca: 41 0f 94 c2                 	sete	%r10b
14089bbce: 48 8d 81 08 01 00 00        	leaq	0x108(%rcx), %rax
14089bbd5: 8b 91 08 01 00 00           	movl	0x108(%rcx), %edx
14089bbdb: 81 fa c3 00 00 00           	cmpl	$0xc3, %edx
14089bbe1: 0f 94 c3                    	sete	%bl
14089bbe4: 44 08 d3                    	orb	%r10b, %bl
14089bbe7: 48 89 bc 24 c8 00 00 00     	movq	%rdi, 0xc8(%rsp)
14089bbef: 74 17                       	je	0x14089bc08 <.text+0x89ac08>
14089bbf1: 81 f2 c3 00 00 00           	xorl	$0xc3, %edx
14089bbf7: 41 89 f2                    	movl	%esi, %r10d
14089bbfa: 41 81 f2 c3 00 00 00        	xorl	$0xc3, %r10d
14089bc01: 41 09 d2                    	orl	%edx, %r10d
14089bc04: 74 0e                       	je	0x14089bc14 <.text+0x89ac14>
14089bc06: eb 12                       	jmp	0x14089bc1a <.text+0x89ac1a>
14089bc08: 39 f2                       	cmpl	%esi, %edx
14089bc0a: 75 0e                       	jne	0x14089bc1a <.text+0x89ac1a>
14089bc0c: 85 f6                       	testl	%esi, %esi
14089bc0e: 0f 84 78 07 00 00           	je	0x14089c38c <.text+0x89b38c>
14089bc14: c7 00 c3 00 00 00           	movl	$0xc3, (%rax)
14089bc1a: 81 fe c3 00 00 00           	cmpl	$0xc3, %esi
14089bc20: 41 0f 94 c2                 	sete	%r10b
14089bc24: 48 8d b9 14 01 00 00        	leaq	0x114(%rcx), %rdi
14089bc2b: 8b 91 14 01 00 00           	movl	0x114(%rcx), %edx
14089bc31: 81 fa c3 00 00 00           	cmpl	$0xc3, %edx
14089bc37: 0f 94 c3                    	sete	%bl
14089bc3a: 44 08 d3                    	orb	%r10b, %bl
14089bc3d: 4c 89 b4 24 b8 00 00 00     	movq	%r14, 0xb8(%rsp)
14089bc45: 74 17                       	je	0x14089bc5e <.text+0x89ac5e>
14089bc47: 81 f2 c3 00 00 00           	xorl	$0xc3, %edx
14089bc4d: 41 89 f2                    	movl	%esi, %r10d
14089bc50: 41 81 f2 c3 00 00 00        	xorl	$0xc3, %r10d
14089bc57: 41 09 d2                    	orl	%edx, %r10d
14089bc5a: 74 0e                       	je	0x14089bc6a <.text+0x89ac6a>
14089bc5c: eb 12                       	jmp	0x14089bc70 <.text+0x89ac70>
14089bc5e: 39 f2                       	cmpl	%esi, %edx
14089bc60: 75 0e                       	jne	0x14089bc70 <.text+0x89ac70>
14089bc62: 85 f6                       	testl	%esi, %esi
14089bc64: 0f 84 55 07 00 00           	je	0x14089c3bf <.text+0x89b3bf>
14089bc6a: c7 07 c3 00 00 00           	movl	$0xc3, (%rdi)
14089bc70: 81 fe c3 00 00 00           	cmpl	$0xc3, %esi
14089bc76: 41 0f 94 c2                 	sete	%r10b
14089bc7a: 48 8d 99 20 01 00 00        	leaq	0x120(%rcx), %rbx
14089bc81: 8b 91 20 01 00 00           	movl	0x120(%rcx), %edx
14089bc87: 81 fa c3 00 00 00           	cmpl	$0xc3, %edx
14089bc8d: 41 0f 94 c6                 	sete	%r14b
14089bc91: 45 08 d6                    	orb	%r10b, %r14b
14089bc94: 4c 89 bc 24 b0 00 00 00     	movq	%r15, 0xb0(%rsp)
14089bc9c: 74 17                       	je	0x14089bcb5 <.text+0x89acb5>
14089bc9e: 81 f2 c3 00 00 00           	xorl	$0xc3, %edx
14089bca4: 41 89 f2                    	movl	%esi, %r10d
14089bca7: 41 81 f2 c3 00 00 00        	xorl	$0xc3, %r10d
14089bcae: 41 09 d2                    	orl	%edx, %r10d
14089bcb1: 74 0e                       	je	0x14089bcc1 <.text+0x89acc1>
14089bcb3: eb 12                       	jmp	0x14089bcc7 <.text+0x89acc7>
14089bcb5: 39 f2                       	cmpl	%esi, %edx
14089bcb7: 75 0e                       	jne	0x14089bcc7 <.text+0x89acc7>
14089bcb9: 85 f6                       	testl	%esi, %esi
14089bcbb: 0f 84 31 07 00 00           	je	0x14089c3f2 <.text+0x89b3f2>
14089bcc1: c7 03 c3 00 00 00           	movl	$0xc3, (%rbx)
14089bcc7: 4c 89 84 24 a8 00 00 00     	movq	%r8, 0xa8(%rsp)
14089bccf: 81 fe c3 00 00 00           	cmpl	$0xc3, %esi
14089bcd5: 41 0f 94 c2                 	sete	%r10b
14089bcd9: 48 8d 91 2c 01 00 00        	leaq	0x12c(%rcx), %rdx
14089bce0: 44 8b b1 2c 01 00 00        	movl	0x12c(%rcx), %r14d
14089bce7: 41 81 fe c3 00 00 00        	cmpl	$0xc3, %r14d
14089bcee: 41 0f 94 c7                 	sete	%r15b
14089bcf2: 45 08 d7                    	orb	%r10b, %r15b
14089bcf5: 48 89 bc 24 a0 00 00 00     	movq	%rdi, 0xa0(%rsp)
14089bcfd: 4d 89 c8                    	movq	%r9, %r8
14089bd00: 74 18                       	je	0x14089bd1a <.text+0x89ad1a>
14089bd02: 41 81 f6 c3 00 00 00        	xorl	$0xc3, %r14d
14089bd09: 41 89 f2                    	movl	%esi, %r10d
14089bd0c: 41 81 f2 c3 00 00 00        	xorl	$0xc3, %r10d
14089bd13: 45 09 f2                    	orl	%r14d, %r10d
14089bd16: 74 0f                       	je	0x14089bd27 <.text+0x89ad27>
14089bd18: eb 13                       	jmp	0x14089bd2d <.text+0x89ad2d>
14089bd1a: 41 39 f6                    	cmpl	%esi, %r14d
14089bd1d: 75 0e                       	jne	0x14089bd2d <.text+0x89ad2d>
14089bd1f: 85 f6                       	testl	%esi, %esi
14089bd21: 0f 84 fe 06 00 00           	je	0x14089c425 <.text+0x89b425>
14089bd27: c7 02 c3 00 00 00           	movl	$0xc3, (%rdx)
14089bd2d: 48 89 c7                    	movq	%rax, %rdi
14089bd30: 81 fe c3 00 00 00           	cmpl	$0xc3, %esi
14089bd36: 41 0f 94 c2                 	sete	%r10b
14089bd3a: 4c 8d b1 38 01 00 00        	leaq	0x138(%rcx), %r14
14089bd41: 44 8b b9 38 01 00 00        	movl	0x138(%rcx), %r15d
14089bd48: 41 81 ff c3 00 00 00        	cmpl	$0xc3, %r15d
14089bd4f: 41 0f 94 c1                 	sete	%r9b
14089bd53: 4c 89 e8                    	movq	%r13, %rax
14089bd56: 45 08 d1                    	orb	%r10b, %r9b
14089bd59: 74 18                       	je	0x14089bd73 <.text+0x89ad73>
14089bd5b: 41 81 f7 c3 00 00 00        	xorl	$0xc3, %r15d
14089bd62: 41 89 f1                    	movl	%esi, %r9d
14089bd65: 41 81 f1 c3 00 00 00        	xorl	$0xc3, %r9d
14089bd6c: 45 09 f9                    	orl	%r15d, %r9d
14089bd6f: 74 0f                       	je	0x14089bd80 <.text+0x89ad80>
14089bd71: eb 14                       	jmp	0x14089bd87 <.text+0x89ad87>
14089bd73: 41 39 f7                    	cmpl	%esi, %r15d
14089bd76: 75 0f                       	jne	0x14089bd87 <.text+0x89ad87>
14089bd78: 85 f6                       	testl	%esi, %esi
14089bd7a: 0f 84 d9 06 00 00           	je	0x14089c459 <.text+0x89b459>
14089bd80: 41 c7 06 c3 00 00 00        	movl	$0xc3, (%r14)
14089bd87: 45 89 dd                    	movl	%r11d, %r13d
14089bd8a: 41 89 eb                    	movl	%ebp, %r11d
14089bd8d: 81 fe c3 00 00 00           	cmpl	$0xc3, %esi
14089bd93: 41 0f 94 c1                 	sete	%r9b
14089bd97: 4c 8d b9 44 01 00 00        	leaq	0x144(%rcx), %r15
14089bd9e: 44 8b 91 44 01 00 00        	movl	0x144(%rcx), %r10d
14089bda5: 41 81 fa c3 00 00 00        	cmpl	$0xc3, %r10d
14089bdac: 40 0f 94 c5                 	sete	%bpl
14089bdb0: 44 08 cd                    	orb	%r9b, %bpl
14089bdb3: 74 14                       	je	0x14089bdc9 <.text+0x89adc9>
14089bdb5: 41 81 f2 c3 00 00 00        	xorl	$0xc3, %r10d
14089bdbc: 81 f6 c3 00 00 00           	xorl	$0xc3, %esi
14089bdc2: 44 09 d6                    	orl	%r10d, %esi
14089bdc5: 74 0f                       	je	0x14089bdd6 <.text+0x89add6>
14089bdc7: eb 14                       	jmp	0x14089bddd <.text+0x89addd>
14089bdc9: 41 39 f2                    	cmpl	%esi, %r10d
14089bdcc: 75 0f                       	jne	0x14089bddd <.text+0x89addd>
14089bdce: 85 f6                       	testl	%esi, %esi
14089bdd0: 0f 84 b7 06 00 00           	je	0x14089c48d <.text+0x89b48d>
14089bdd6: 41 c7 07 c3 00 00 00        	movl	$0xc3, (%r15)
14089bddd: 44 0f b6 4c 24 07           	movzbl	0x7(%rsp), %r9d
14089bde3: 4c 8d 15 02 ec 1f 02        	leaq	0x21fec02(%rip), %r10   # 0x142a9a9ec
14089bdea: 4f 63 0c 8a                 	movslq	(%r10,%r9,4), %r9
14089bdee: 4d 01 d1                    	addq	%r10, %r9
14089bdf1: 4c 8b 94 24 98 00 00 00     	movq	0x98(%rsp), %r10
14089bdf9: 41 ff e1                    	jmpq	*%r9
14089bdfc: 4d 89 c2                    	movq	%r8, %r10
14089bdff: e9 fb 07 00 00              	jmp	0x14089c5ff <.text+0x89b5ff>
14089be04: 66 44 39 a1 54 01 00 00     	cmpw	%r12w, 0x154(%rcx)
14089be0c: 0f 85 08 f5 ff ff           	jne	0x14089b31a <.text+0x89a31a>
14089be12: 44 89 e0                    	movl	%r12d, %eax
14089be15: 48 8d 15 14 e9 1f 02        	leaq	0x21fe914(%rip), %rdx   # 0x142a9a730
14089be1c: 48 63 04 82                 	movslq	(%rdx,%rax,4), %rax
14089be20: 48 01 d0                    	addq	%rdx, %rax
14089be23: ff e0                       	jmpq	*%rax
14089be25: 44 39 99 58 01 00 00        	cmpl	%r11d, 0x158(%rcx)
14089be2c: 0f 84 e1 f4 ff ff           	je	0x14089b313 <.text+0x89a313>
14089be32: e9 e3 f4 ff ff              	jmp	0x14089b31a <.text+0x89a31a>
14089be37: 66 44 39 a1 60 01 00 00     	cmpw	%r12w, 0x160(%rcx)
14089be3f: 0f 85 1f f5 ff ff           	jne	0x14089b364 <.text+0x89a364>
14089be45: 44 89 e0                    	movl	%r12d, %eax
14089be48: 48 8d 15 f5 e8 1f 02        	leaq	0x21fe8f5(%rip), %rdx   # 0x142a9a744
14089be4f: 48 63 04 82                 	movslq	(%rdx,%rax,4), %rax
14089be53: 48 01 d0                    	addq	%rdx, %rax
14089be56: ff e0                       	jmpq	*%rax
14089be58: 44 39 99 64 01 00 00        	cmpl	%r11d, 0x164(%rcx)
14089be5f: 0f 84 f8 f4 ff ff           	je	0x14089b35d <.text+0x89a35d>
14089be65: e9 fa f4 ff ff              	jmp	0x14089b364 <.text+0x89a364>
14089be6a: 66 44 39 a1 6c 01 00 00     	cmpw	%r12w, 0x16c(%rcx)
14089be72: 0f 85 35 f5 ff ff           	jne	0x14089b3ad <.text+0x89a3ad>
14089be78: 44 89 e0                    	movl	%r12d, %eax
14089be7b: 48 8d 15 d6 e8 1f 02        	leaq	0x21fe8d6(%rip), %rdx   # 0x142a9a758
14089be82: 48 63 04 82                 	movslq	(%rdx,%rax,4), %rax
14089be86: 48 01 d0                    	addq	%rdx, %rax
14089be89: ff e0                       	jmpq	*%rax
14089be8b: 44 39 99 70 01 00 00        	cmpl	%r11d, 0x170(%rcx)
14089be92: 0f 84 0f f5 ff ff           	je	0x14089b3a7 <.text+0x89a3a7>
14089be98: e9 10 f5 ff ff              	jmp	0x14089b3ad <.text+0x89a3ad>
14089be9d: 66 44 39 a1 78 01 00 00     	cmpw	%r12w, 0x178(%rcx)
14089bea5: 0f 85 4b f5 ff ff           	jne	0x14089b3f6 <.text+0x89a3f6>
14089beab: 44 89 e0                    	movl	%r12d, %eax
14089beae: 48 8d 15 b7 e8 1f 02        	leaq	0x21fe8b7(%rip), %rdx   # 0x142a9a76c
14089beb5: 48 63 04 82                 	movslq	(%rdx,%rax,4), %rax
14089beb9: 48 01 d0                    	addq	%rdx, %rax
14089bebc: ff e0                       	jmpq	*%rax
14089bebe: 44 39 99 7c 01 00 00        	cmpl	%r11d, 0x17c(%rcx)
14089bec5: 0f 84 25 f5 ff ff           	je	0x14089b3f0 <.text+0x89a3f0>
14089becb: e9 26 f5 ff ff              	jmp	0x14089b3f6 <.text+0x89a3f6>
14089bed0: 66 44 39 a1 84 01 00 00     	cmpw	%r12w, 0x184(%rcx)
14089bed8: 0f 85 62 f5 ff ff           	jne	0x14089b440 <.text+0x89a440>
14089bede: 44 89 e0                    	movl	%r12d, %eax
14089bee1: 48 8d 15 98 e8 1f 02        	leaq	0x21fe898(%rip), %rdx   # 0x142a9a780
14089bee8: 48 63 04 82                 	movslq	(%rdx,%rax,4), %rax
14089beec: 48 01 d0                    	addq	%rdx, %rax
14089beef: ff e0                       	jmpq	*%rax
14089bef1: 44 39 99 88 01 00 00        	cmpl	%r11d, 0x188(%rcx)
14089bef8: 0f 84 3b f5 ff ff           	je	0x14089b439 <.text+0x89a439>
14089befe: e9 3d f5 ff ff              	jmp	0x14089b440 <.text+0x89a440>
14089bf03: 66 44 39 a1 90 01 00 00     	cmpw	%r12w, 0x190(%rcx)
14089bf0b: 0f 85 79 f5 ff ff           	jne	0x14089b48a <.text+0x89a48a>
14089bf11: 44 89 e0                    	movl	%r12d, %eax
14089bf14: 48 8d 15 79 e8 1f 02        	leaq	0x21fe879(%rip), %rdx   # 0x142a9a794
14089bf1b: 48 63 04 82                 	movslq	(%rdx,%rax,4), %rax
14089bf1f: 48 01 d0                    	addq	%rdx, %rax
14089bf22: ff e0                       	jmpq	*%rax
14089bf24: 44 39 99 94 01 00 00        	cmpl	%r11d, 0x194(%rcx)
14089bf2b: 0f 84 52 f5 ff ff           	je	0x14089b483 <.text+0x89a483>
14089bf31: e9 54 f5 ff ff              	jmp	0x14089b48a <.text+0x89a48a>
14089bf36: 66 44 39 a1 9c 01 00 00     	cmpw	%r12w, 0x19c(%rcx)
14089bf3e: 0f 85 91 f5 ff ff           	jne	0x14089b4d5 <.text+0x89a4d5>
14089bf44: 44 89 e0                    	movl	%r12d, %eax
14089bf47: 48 8d 15 5a e8 1f 02        	leaq	0x21fe85a(%rip), %rdx   # 0x142a9a7a8
14089bf4e: 48 63 04 82                 	movslq	(%rdx,%rax,4), %rax
14089bf52: 48 01 d0                    	addq	%rdx, %rax
14089bf55: ff e0                       	jmpq	*%rax
14089bf57: 44 39 99 a0 01 00 00        	cmpl	%r11d, 0x1a0(%rcx)
14089bf5e: 0f 84 69 f5 ff ff           	je	0x14089b4cd <.text+0x89a4cd>
14089bf64: e9 6c f5 ff ff              	jmp	0x14089b4d5 <.text+0x89a4d5>
14089bf69: 66 44 39 a1 a8 01 00 00     	cmpw	%r12w, 0x1a8(%rcx)
14089bf71: 0f 85 bf f5 ff ff           	jne	0x14089b536 <.text+0x89a536>
14089bf77: 44 89 e0                    	movl	%r12d, %eax
14089bf7a: 48 8d 15 3b e8 1f 02        	leaq	0x21fe83b(%rip), %rdx   # 0x142a9a7bc
14089bf81: 48 63 04 82                 	movslq	(%rdx,%rax,4), %rax
14089bf85: 48 01 d0                    	addq	%rdx, %rax
14089bf88: ff e0                       	jmpq	*%rax
14089bf8a: 44 39 99 ac 01 00 00        	cmpl	%r11d, 0x1ac(%rcx)
14089bf91: 0f 84 91 f5 ff ff           	je	0x14089b528 <.text+0x89a528>
14089bf97: e9 9a f5 ff ff              	jmp	0x14089b536 <.text+0x89a536>
14089bf9c: 66 44 39 61 34              	cmpw	%r12w, 0x34(%rcx)
14089bfa1: 0f 85 e2 f5 ff ff           	jne	0x14089b589 <.text+0x89a589>
14089bfa7: 44 89 e0                    	movl	%r12d, %eax
14089bfaa: 48 8d 15 1f e8 1f 02        	leaq	0x21fe81f(%rip), %rdx   # 0x142a9a7d0
14089bfb1: 48 63 04 82                 	movslq	(%rdx,%rax,4), %rax
14089bfb5: 48 01 d0                    	addq	%rdx, %rax
14089bfb8: ff e0                       	jmpq	*%rax
14089bfba: 44 39 59 38                 	cmpl	%r11d, 0x38(%rcx)
14089bfbe: 0f 84 b7 f5 ff ff           	je	0x14089b57b <.text+0x89a57b>
14089bfc4: e9 c0 f5 ff ff              	jmp	0x14089b589 <.text+0x89a589>
14089bfc9: 66 44 39 61 40              	cmpw	%r12w, 0x40(%rcx)
14089bfce: 0f 85 f9 f5 ff ff           	jne	0x14089b5cd <.text+0x89a5cd>
14089bfd4: 44 89 e0                    	movl	%r12d, %eax
14089bfd7: 48 8d 15 06 e8 1f 02        	leaq	0x21fe806(%rip), %rdx   # 0x142a9a7e4
14089bfde: 48 63 04 82                 	movslq	(%rdx,%rax,4), %rax
14089bfe2: 48 01 d0                    	addq	%rdx, %rax
14089bfe5: ff e0                       	jmpq	*%rax
14089bfe7: 44 39 59 44                 	cmpl	%r11d, 0x44(%rcx)
14089bfeb: 0f 84 d5 f5 ff ff           	je	0x14089b5c6 <.text+0x89a5c6>
14089bff1: e9 d7 f5 ff ff              	jmp	0x14089b5cd <.text+0x89a5cd>
14089bff6: 66 44 39 61 1c              	cmpw	%r12w, 0x1c(%rcx)
14089bffb: 0f 85 1f f6 ff ff           	jne	0x14089b620 <.text+0x89a620>
14089c001: 44 89 e0                    	movl	%r12d, %eax
14089c004: 48 8d 15 ed e7 1f 02        	leaq	0x21fe7ed(%rip), %rdx   # 0x142a9a7f8
14089c00b: 48 63 04 82                 	movslq	(%rdx,%rax,4), %rax
14089c00f: 48 01 d0                    	addq	%rdx, %rax
14089c012: ff e0                       	jmpq	*%rax
14089c014: 44 39 59 20                 	cmpl	%r11d, 0x20(%rcx)
14089c018: 0f 84 f4 f5 ff ff           	je	0x14089b612 <.text+0x89a612>
14089c01e: e9 fd f5 ff ff              	jmp	0x14089b620 <.text+0x89a620>
14089c023: 66 44 39 61 28              	cmpw	%r12w, 0x28(%rcx)
14089c028: 0f 85 45 f6 ff ff           	jne	0x14089b673 <.text+0x89a673>
14089c02e: 44 89 e0                    	movl	%r12d, %eax
14089c031: 48 8d 15 d4 e7 1f 02        	leaq	0x21fe7d4(%rip), %rdx   # 0x142a9a80c
14089c038: 48 63 04 82                 	movslq	(%rdx,%rax,4), %rax
14089c03c: 48 01 d0                    	addq	%rdx, %rax
14089c03f: ff e0                       	jmpq	*%rax
14089c041: 44 39 59 2c                 	cmpl	%r11d, 0x2c(%rcx)
14089c045: 0f 84 1a f6 ff ff           	je	0x14089b665 <.text+0x89a665>
14089c04b: e9 23 f6 ff ff              	jmp	0x14089b673 <.text+0x89a673>
14089c050: 66 44 39 61 04              	cmpw	%r12w, 0x4(%rcx)
14089c055: 0f 85 56 f6 ff ff           	jne	0x14089b6b1 <.text+0x89a6b1>
14089c05b: 44 89 e0                    	movl	%r12d, %eax
14089c05e: 48 8d 15 bb e7 1f 02        	leaq	0x21fe7bb(%rip), %rdx   # 0x142a9a820
14089c065: 48 63 04 82                 	movslq	(%rdx,%rax,4), %rax
14089c069: 48 01 d0                    	addq	%rdx, %rax
14089c06c: ff e0                       	jmpq	*%rax
14089c06e: 44 39 59 08                 	cmpl	%r11d, 0x8(%rcx)
14089c072: 0f 84 33 f6 ff ff           	je	0x14089b6ab <.text+0x89a6ab>
14089c078: e9 34 f6 ff ff              	jmp	0x14089b6b1 <.text+0x89a6b1>
14089c07d: 66 44 39 61 10              	cmpw	%r12w, 0x10(%rcx)
14089c082: 0f 85 76 f6 ff ff           	jne	0x14089b6fe <.text+0x89a6fe>
14089c088: 44 89 e0                    	movl	%r12d, %eax
14089c08b: 48 8d 15 a2 e7 1f 02        	leaq	0x21fe7a2(%rip), %rdx   # 0x142a9a834
14089c092: 48 63 04 82                 	movslq	(%rdx,%rax,4), %rax
14089c096: 48 01 d0                    	addq	%rdx, %rax
14089c099: ff e0                       	jmpq	*%rax
14089c09b: 44 39 59 14                 	cmpl	%r11d, 0x14(%rcx)
14089c09f: 0f 84 4e f6 ff ff           	je	0x14089b6f3 <.text+0x89a6f3>
14089c0a5: e9 54 f6 ff ff              	jmp	0x14089b6fe <.text+0x89a6fe>
14089c0aa: 66 44 39 61 4c              	cmpw	%r12w, 0x4c(%rcx)
14089c0af: 0f 85 96 f6 ff ff           	jne	0x14089b74b <.text+0x89a74b>
14089c0b5: 44 89 e0                    	movl	%r12d, %eax
14089c0b8: 48 8d 15 89 e7 1f 02        	leaq	0x21fe789(%rip), %rdx   # 0x142a9a848
14089c0bf: 48 63 04 82                 	movslq	(%rdx,%rax,4), %rax
14089c0c3: 48 01 d0                    	addq	%rdx, %rax
14089c0c6: ff e0                       	jmpq	*%rax
14089c0c8: 44 39 59 50                 	cmpl	%r11d, 0x50(%rcx)
14089c0cc: 0f 84 6e f6 ff ff           	je	0x14089b740 <.text+0x89a740>
14089c0d2: e9 74 f6 ff ff              	jmp	0x14089b74b <.text+0x89a74b>
14089c0d7: 66 44 39 61 58              	cmpw	%r12w, 0x58(%rcx)
14089c0dc: 0f 85 b6 f6 ff ff           	jne	0x14089b798 <.text+0x89a798>
14089c0e2: 44 89 e0                    	movl	%r12d, %eax
14089c0e5: 48 8d 15 70 e7 1f 02        	leaq	0x21fe770(%rip), %rdx   # 0x142a9a85c
14089c0ec: 48 63 04 82                 	movslq	(%rdx,%rax,4), %rax
14089c0f0: 48 01 d0                    	addq	%rdx, %rax
14089c0f3: ff e0                       	jmpq	*%rax
14089c0f5: 44 39 59 5c                 	cmpl	%r11d, 0x5c(%rcx)
14089c0f9: 0f 84 8e f6 ff ff           	je	0x14089b78d <.text+0x89a78d>
14089c0ff: e9 94 f6 ff ff              	jmp	0x14089b798 <.text+0x89a798>
14089c104: 66 44 39 61 64              	cmpw	%r12w, 0x64(%rcx)
14089c109: 0f 85 d6 f6 ff ff           	jne	0x14089b7e5 <.text+0x89a7e5>
14089c10f: 44 89 e0                    	movl	%r12d, %eax
14089c112: 48 8d 15 57 e7 1f 02        	leaq	0x21fe757(%rip), %rdx   # 0x142a9a870
14089c119: 48 63 04 82                 	movslq	(%rdx,%rax,4), %rax
14089c11d: 48 01 d0                    	addq	%rdx, %rax
14089c120: ff e0                       	jmpq	*%rax
14089c122: 44 39 59 68                 	cmpl	%r11d, 0x68(%rcx)
14089c126: 0f 84 ae f6 ff ff           	je	0x14089b7da <.text+0x89a7da>
14089c12c: e9 b4 f6 ff ff              	jmp	0x14089b7e5 <.text+0x89a7e5>
14089c131: 66 44 39 61 70              	cmpw	%r12w, 0x70(%rcx)
14089c136: 0f 85 f6 f6 ff ff           	jne	0x14089b832 <.text+0x89a832>
14089c13c: 44 89 e0                    	movl	%r12d, %eax
14089c13f: 48 8d 15 3e e7 1f 02        	leaq	0x21fe73e(%rip), %rdx   # 0x142a9a884
14089c146: 48 63 04 82                 	movslq	(%rdx,%rax,4), %rax
14089c14a: 48 01 d0                    	addq	%rdx, %rax
14089c14d: ff e0                       	jmpq	*%rax
14089c14f: 44 39 59 74                 	cmpl	%r11d, 0x74(%rcx)
14089c153: 0f 84 ce f6 ff ff           	je	0x14089b827 <.text+0x89a827>
14089c159: e9 d4 f6 ff ff              	jmp	0x14089b832 <.text+0x89a832>
14089c15e: 66 44 39 61 7c              	cmpw	%r12w, 0x7c(%rcx)
14089c163: 0f 85 16 f7 ff ff           	jne	0x14089b87f <.text+0x89a87f>
14089c169: 44 89 e0                    	movl	%r12d, %eax
14089c16c: 48 8d 15 25 e7 1f 02        	leaq	0x21fe725(%rip), %rdx   # 0x142a9a898
14089c173: 48 63 04 82                 	movslq	(%rdx,%rax,4), %rax
14089c177: 48 01 d0                    	addq	%rdx, %rax
14089c17a: ff e0                       	jmpq	*%rax
14089c17c: 44 39 99 80 00 00 00        	cmpl	%r11d, 0x80(%rcx)
14089c183: 0f 84 eb f6 ff ff           	je	0x14089b874 <.text+0x89a874>
14089c189: e9 f1 f6 ff ff              	jmp	0x14089b87f <.text+0x89a87f>
14089c18e: 66 44 39 a1 88 00 00 00     	cmpw	%r12w, 0x88(%rcx)
14089c196: 0f 85 36 f7 ff ff           	jne	0x14089b8d2 <.text+0x89a8d2>
14089c19c: 44 89 e0                    	movl	%r12d, %eax
14089c19f: 48 8d 15 06 e7 1f 02        	leaq	0x21fe706(%rip), %rdx   # 0x142a9a8ac
14089c1a6: 48 63 04 82                 	movslq	(%rdx,%rax,4), %rax
14089c1aa: 48 01 d0                    	addq	%rdx, %rax
14089c1ad: ff e0                       	jmpq	*%rax
14089c1af: 44 39 99 8c 00 00 00        	cmpl	%r11d, 0x8c(%rcx)
14089c1b6: 0f 84 0b f7 ff ff           	je	0x14089b8c7 <.text+0x89a8c7>
14089c1bc: e9 11 f7 ff ff              	jmp	0x14089b8d2 <.text+0x89a8d2>
14089c1c1: 66 44 39 a1 94 00 00 00     	cmpw	%r12w, 0x94(%rcx)
14089c1c9: 0f 85 56 f7 ff ff           	jne	0x14089b925 <.text+0x89a925>
14089c1cf: 44 89 e0                    	movl	%r12d, %eax
14089c1d2: 48 8d 15 e7 e6 1f 02        	leaq	0x21fe6e7(%rip), %rdx   # 0x142a9a8c0
14089c1d9: 48 63 04 82                 	movslq	(%rdx,%rax,4), %rax
14089c1dd: 48 01 d0                    	addq	%rdx, %rax
14089c1e0: ff e0                       	jmpq	*%rax
14089c1e2: 44 39 99 98 00 00 00        	cmpl	%r11d, 0x98(%rcx)
14089c1e9: 0f 84 2b f7 ff ff           	je	0x14089b91a <.text+0x89a91a>
14089c1ef: e9 31 f7 ff ff              	jmp	0x14089b925 <.text+0x89a925>
14089c1f4: 66 44 39 a1 a0 00 00 00     	cmpw	%r12w, 0xa0(%rcx)
14089c1fc: 0f 85 76 f7 ff ff           	jne	0x14089b978 <.text+0x89a978>
14089c202: 44 89 e0                    	movl	%r12d, %eax
14089c205: 48 8d 15 c8 e6 1f 02        	leaq	0x21fe6c8(%rip), %rdx   # 0x142a9a8d4
14089c20c: 48 63 04 82                 	movslq	(%rdx,%rax,4), %rax
14089c210: 48 01 d0                    	addq	%rdx, %rax
14089c213: ff e0                       	jmpq	*%rax
14089c215: 44 39 99 a4 00 00 00        	cmpl	%r11d, 0xa4(%rcx)
14089c21c: 0f 84 4b f7 ff ff           	je	0x14089b96d <.text+0x89a96d>
14089c222: e9 51 f7 ff ff              	jmp	0x14089b978 <.text+0x89a978>
14089c227: 66 44 39 a1 ac 00 00 00     	cmpw	%r12w, 0xac(%rcx)
14089c22f: 0f 85 96 f7 ff ff           	jne	0x14089b9cb <.text+0x89a9cb>
14089c235: 44 89 e0                    	movl	%r12d, %eax
14089c238: 48 8d 15 a9 e6 1f 02        	leaq	0x21fe6a9(%rip), %rdx   # 0x142a9a8e8
14089c23f: 48 63 04 82                 	movslq	(%rdx,%rax,4), %rax
14089c243: 48 01 d0                    	addq	%rdx, %rax
14089c246: ff e0                       	jmpq	*%rax
14089c248: 44 39 99 b0 00 00 00        	cmpl	%r11d, 0xb0(%rcx)
14089c24f: 0f 84 6b f7 ff ff           	je	0x14089b9c0 <.text+0x89a9c0>
14089c255: e9 71 f7 ff ff              	jmp	0x14089b9cb <.text+0x89a9cb>
14089c25a: 66 44 39 a1 b8 00 00 00     	cmpw	%r12w, 0xb8(%rcx)
14089c262: 0f 85 b6 f7 ff ff           	jne	0x14089ba1e <.text+0x89aa1e>
14089c268: 44 89 e0                    	movl	%r12d, %eax
14089c26b: 48 8d 15 8a e6 1f 02        	leaq	0x21fe68a(%rip), %rdx   # 0x142a9a8fc
14089c272: 48 63 04 82                 	movslq	(%rdx,%rax,4), %rax
14089c276: 48 01 d0                    	addq	%rdx, %rax
14089c279: ff e0                       	jmpq	*%rax
14089c27b: 44 39 99 bc 00 00 00        	cmpl	%r11d, 0xbc(%rcx)
14089c282: 0f 84 8b f7 ff ff           	je	0x14089ba13 <.text+0x89aa13>
14089c288: e9 91 f7 ff ff              	jmp	0x14089ba1e <.text+0x89aa1e>
14089c28d: 66 44 39 a1 c4 00 00 00     	cmpw	%r12w, 0xc4(%rcx)
14089c295: 0f 85 d6 f7 ff ff           	jne	0x14089ba71 <.text+0x89aa71>
14089c29b: 44 89 e0                    	movl	%r12d, %eax
14089c29e: 48 8d 15 6b e6 1f 02        	leaq	0x21fe66b(%rip), %rdx   # 0x142a9a910
14089c2a5: 48 63 04 82                 	movslq	(%rdx,%rax,4), %rax
14089c2a9: 48 01 d0                    	addq	%rdx, %rax
14089c2ac: ff e0                       	jmpq	*%rax
14089c2ae: 44 39 99 c8 00 00 00        	cmpl	%r11d, 0xc8(%rcx)
14089c2b5: 0f 84 ab f7 ff ff           	je	0x14089ba66 <.text+0x89aa66>
14089c2bb: e9 b1 f7 ff ff              	jmp	0x14089ba71 <.text+0x89aa71>
14089c2c0: 66 44 39 a1 d0 00 00 00     	cmpw	%r12w, 0xd0(%rcx)
14089c2c8: 0f 85 f6 f7 ff ff           	jne	0x14089bac4 <.text+0x89aac4>
14089c2ce: 44 89 e0                    	movl	%r12d, %eax
14089c2d1: 48 8d 15 4c e6 1f 02        	leaq	0x21fe64c(%rip), %rdx   # 0x142a9a924
14089c2d8: 48 63 04 82                 	movslq	(%rdx,%rax,4), %rax
14089c2dc: 48 01 d0                    	addq	%rdx, %rax
14089c2df: ff e0                       	jmpq	*%rax
14089c2e1: 44 39 99 d4 00 00 00        	cmpl	%r11d, 0xd4(%rcx)
14089c2e8: 0f 84 cb f7 ff ff           	je	0x14089bab9 <.text+0x89aab9>
14089c2ee: e9 d1 f7 ff ff              	jmp	0x14089bac4 <.text+0x89aac4>
14089c2f3: 66 44 39 a1 e8 00 00 00     	cmpw	%r12w, 0xe8(%rcx)
14089c2fb: 0f 85 16 f8 ff ff           	jne	0x14089bb17 <.text+0x89ab17>
14089c301: 44 89 e0                    	movl	%r12d, %eax
14089c304: 48 8d 15 2d e6 1f 02        	leaq	0x21fe62d(%rip), %rdx   # 0x142a9a938
14089c30b: 48 63 04 82                 	movslq	(%rdx,%rax,4), %rax
14089c30f: 48 01 d0                    	addq	%rdx, %rax
14089c312: ff e0                       	jmpq	*%rax
14089c314: 44 39 99 ec 00 00 00        	cmpl	%r11d, 0xec(%rcx)
14089c31b: 0f 84 eb f7 ff ff           	je	0x14089bb0c <.text+0x89ab0c>
14089c321: e9 f1 f7 ff ff              	jmp	0x14089bb17 <.text+0x89ab17>
14089c326: 66 44 39 a1 f4 00 00 00     	cmpw	%r12w, 0xf4(%rcx)
14089c32e: 0f 85 3e f8 ff ff           	jne	0x14089bb72 <.text+0x89ab72>
14089c334: 44 89 e0                    	movl	%r12d, %eax
14089c337: 48 8d 15 0e e6 1f 02        	leaq	0x21fe60e(%rip), %rdx   # 0x142a9a94c
14089c33e: 48 63 04 82                 	movslq	(%rdx,%rax,4), %rax
14089c342: 48 01 d0                    	addq	%rdx, %rax
14089c345: ff e0                       	jmpq	*%rax
14089c347: 44 39 99 f8 00 00 00        	cmpl	%r11d, 0xf8(%rcx)
14089c34e: 0f 84 13 f8 ff ff           	je	0x14089bb67 <.text+0x89ab67>
14089c354: e9 19 f8 ff ff              	jmp	0x14089bb72 <.text+0x89ab72>
14089c359: 66 44 39 a1 00 01 00 00     	cmpw	%r12w, 0x100(%rcx)
14089c361: 0f 85 5d f8 ff ff           	jne	0x14089bbc4 <.text+0x89abc4>
14089c367: 44 89 e0                    	movl	%r12d, %eax
14089c36a: 48 8d 15 ef e5 1f 02        	leaq	0x21fe5ef(%rip), %rdx   # 0x142a9a960
14089c371: 48 63 04 82                 	movslq	(%rdx,%rax,4), %rax
14089c375: 48 01 d0                    	addq	%rdx, %rax
14089c378: ff e0                       	jmpq	*%rax
14089c37a: 44 39 99 04 01 00 00        	cmpl	%r11d, 0x104(%rcx)
14089c381: 0f 84 36 f8 ff ff           	je	0x14089bbbd <.text+0x89abbd>
14089c387: e9 38 f8 ff ff              	jmp	0x14089bbc4 <.text+0x89abc4>
14089c38c: 66 44 39 a1 0c 01 00 00     	cmpw	%r12w, 0x10c(%rcx)
14089c394: 0f 85 80 f8 ff ff           	jne	0x14089bc1a <.text+0x89ac1a>
14089c39a: 44 89 e2                    	movl	%r12d, %edx
14089c39d: 4c 8d 15 d0 e5 1f 02        	leaq	0x21fe5d0(%rip), %r10   # 0x142a9a974
14089c3a4: 49 63 14 92                 	movslq	(%r10,%rdx,4), %rdx
14089c3a8: 4c 01 d2                    	addq	%r10, %rdx
14089c3ab: ff e2                       	jmpq	*%rdx
14089c3ad: 44 39 99 10 01 00 00        	cmpl	%r11d, 0x110(%rcx)
14089c3b4: 0f 84 5a f8 ff ff           	je	0x14089bc14 <.text+0x89ac14>
14089c3ba: e9 5b f8 ff ff              	jmp	0x14089bc1a <.text+0x89ac1a>
14089c3bf: 66 44 39 a1 18 01 00 00     	cmpw	%r12w, 0x118(%rcx)
14089c3c7: 0f 85 a3 f8 ff ff           	jne	0x14089bc70 <.text+0x89ac70>
14089c3cd: 44 89 e2                    	movl	%r12d, %edx
14089c3d0: 4c 8d 15 b1 e5 1f 02        	leaq	0x21fe5b1(%rip), %r10   # 0x142a9a988
14089c3d7: 49 63 14 92                 	movslq	(%r10,%rdx,4), %rdx
14089c3db: 4c 01 d2                    	addq	%r10, %rdx
14089c3de: ff e2                       	jmpq	*%rdx
14089c3e0: 44 39 99 1c 01 00 00        	cmpl	%r11d, 0x11c(%rcx)
14089c3e7: 0f 84 7d f8 ff ff           	je	0x14089bc6a <.text+0x89ac6a>
14089c3ed: e9 7e f8 ff ff              	jmp	0x14089bc70 <.text+0x89ac70>
14089c3f2: 66 44 39 a1 24 01 00 00     	cmpw	%r12w, 0x124(%rcx)
14089c3fa: 0f 85 c7 f8 ff ff           	jne	0x14089bcc7 <.text+0x89acc7>
14089c400: 44 89 e2                    	movl	%r12d, %edx
14089c403: 4c 8d 15 92 e5 1f 02        	leaq	0x21fe592(%rip), %r10   # 0x142a9a99c
14089c40a: 49 63 14 92                 	movslq	(%r10,%rdx,4), %rdx
14089c40e: 4c 01 d2                    	addq	%r10, %rdx
14089c411: ff e2                       	jmpq	*%rdx
14089c413: 44 39 99 28 01 00 00        	cmpl	%r11d, 0x128(%rcx)
14089c41a: 0f 84 a1 f8 ff ff           	je	0x14089bcc1 <.text+0x89acc1>
14089c420: e9 a2 f8 ff ff              	jmp	0x14089bcc7 <.text+0x89acc7>
14089c425: 66 44 39 a1 30 01 00 00     	cmpw	%r12w, 0x130(%rcx)
14089c42d: 0f 85 fa f8 ff ff           	jne	0x14089bd2d <.text+0x89ad2d>
14089c433: 45 89 e2                    	movl	%r12d, %r10d
14089c436: 4c 8d 35 73 e5 1f 02        	leaq	0x21fe573(%rip), %r14   # 0x142a9a9b0
14089c43d: 4f 63 14 96                 	movslq	(%r14,%r10,4), %r10
14089c441: 4d 01 f2                    	addq	%r14, %r10
14089c444: 41 ff e2                    	jmpq	*%r10
14089c447: 44 39 99 34 01 00 00        	cmpl	%r11d, 0x134(%rcx)
14089c44e: 0f 84 d3 f8 ff ff           	je	0x14089bd27 <.text+0x89ad27>
14089c454: e9 d4 f8 ff ff              	jmp	0x14089bd2d <.text+0x89ad2d>
14089c459: 66 44 39 a1 3c 01 00 00     	cmpw	%r12w, 0x13c(%rcx)
14089c461: 0f 85 20 f9 ff ff           	jne	0x14089bd87 <.text+0x89ad87>
14089c467: 45 89 e1                    	movl	%r12d, %r9d
14089c46a: 4c 8d 15 53 e5 1f 02        	leaq	0x21fe553(%rip), %r10   # 0x142a9a9c4
14089c471: 4f 63 0c 8a                 	movslq	(%r10,%r9,4), %r9
14089c475: 4d 01 d1                    	addq	%r10, %r9
14089c478: 41 ff e1                    	jmpq	*%r9
14089c47b: 44 39 99 40 01 00 00        	cmpl	%r11d, 0x140(%rcx)
14089c482: 0f 84 f8 f8 ff ff           	je	0x14089bd80 <.text+0x89ad80>
14089c488: e9 fa f8 ff ff              	jmp	0x14089bd87 <.text+0x89ad87>
14089c48d: 66 44 39 a1 48 01 00 00     	cmpw	%r12w, 0x148(%rcx)
14089c495: 0f 85 42 f9 ff ff           	jne	0x14089bddd <.text+0x89addd>
14089c49b: 45 89 e1                    	movl	%r12d, %r9d
14089c49e: 4c 8d 15 33 e5 1f 02        	leaq	0x21fe533(%rip), %r10   # 0x142a9a9d8
14089c4a5: 4f 63 0c 8a                 	movslq	(%r10,%r9,4), %r9
14089c4a9: 4d 01 d1                    	addq	%r10, %r9
14089c4ac: 41 ff e1                    	jmpq	*%r9
14089c4af: 44 39 a9 4c 01 00 00        	cmpl	%r13d, 0x14c(%rcx)
14089c4b6: 0f 84 1a f9 ff ff           	je	0x14089bdd6 <.text+0x89add6>
14089c4bc: e9 1c f9 ff ff              	jmp	0x14089bddd <.text+0x89addd>
14089c4c1: 4c 8b 94 24 90 00 00 00     	movq	0x90(%rsp), %r10
14089c4c9: e9 31 01 00 00              	jmp	0x14089c5ff <.text+0x89b5ff>
14089c4ce: 48 81 c1 b0 01 00 00        	addq	$0x1b0, %rcx            # imm = 0x1B0
14089c4d5: e9 d1 00 00 00              	jmp	0x14089c5ab <.text+0x89b5ab>
14089c4da: 4d 89 fa                    	movq	%r15, %r10
14089c4dd: e9 1d 01 00 00              	jmp	0x14089c5ff <.text+0x89b5ff>
14089c4e2: 49 89 c2                    	movq	%rax, %r10
14089c4e5: e9 15 01 00 00              	jmp	0x14089c5ff <.text+0x89b5ff>
14089c4ea: 4c 8b 54 24 20              	movq	0x20(%rsp), %r10
14089c4ef: e9 0b 01 00 00              	jmp	0x14089c5ff <.text+0x89b5ff>
14089c4f4: 4c 8b 54 24 30              	movq	0x30(%rsp), %r10
14089c4f9: e9 01 01 00 00              	jmp	0x14089c5ff <.text+0x89b5ff>
14089c4fe: 4c 8b 94 24 b0 00 00 00     	movq	0xb0(%rsp), %r10
14089c506: e9 f4 00 00 00              	jmp	0x14089c5ff <.text+0x89b5ff>
14089c50b: 4c 8b 54 24 48              	movq	0x48(%rsp), %r10
14089c510: e9 ea 00 00 00              	jmp	0x14089c5ff <.text+0x89b5ff>
14089c515: 4c 8b 54 24 40              	movq	0x40(%rsp), %r10
14089c51a: e9 e0 00 00 00              	jmp	0x14089c5ff <.text+0x89b5ff>
14089c51f: 4c 8b 54 24 58              	movq	0x58(%rsp), %r10
14089c524: e9 d6 00 00 00              	jmp	0x14089c5ff <.text+0x89b5ff>
14089c529: 49 89 d2                    	movq	%rdx, %r10
14089c52c: e9 ce 00 00 00              	jmp	0x14089c5ff <.text+0x89b5ff>
14089c531: 49 89 da                    	movq	%rbx, %r10
14089c534: e9 c6 00 00 00              	jmp	0x14089c5ff <.text+0x89b5ff>
14089c539: 4c 8b 94 24 88 00 00 00     	movq	0x88(%rsp), %r10
14089c541: e9 b9 00 00 00              	jmp	0x14089c5ff <.text+0x89b5ff>
14089c546: 4c 8b 54 24 28              	movq	0x28(%rsp), %r10
14089c54b: e9 af 00 00 00              	jmp	0x14089c5ff <.text+0x89b5ff>
14089c550: 4c 8b 94 24 80 00 00 00     	movq	0x80(%rsp), %r10
14089c558: e9 a2 00 00 00              	jmp	0x14089c5ff <.text+0x89b5ff>
14089c55d: 4c 8b 54 24 68              	movq	0x68(%rsp), %r10
14089c562: e9 98 00 00 00              	jmp	0x14089c5ff <.text+0x89b5ff>
14089c567: 4c 8b 94 24 c0 00 00 00     	movq	0xc0(%rsp), %r10
14089c56f: e9 8b 00 00 00              	jmp	0x14089c5ff <.text+0x89b5ff>
14089c574: 4c 8b 54 24 18              	movq	0x18(%rsp), %r10
14089c579: e9 81 00 00 00              	jmp	0x14089c5ff <.text+0x89b5ff>
14089c57e: 4c 8b 54 24 08              	movq	0x8(%rsp), %r10
14089c583: eb 7a                       	jmp	0x14089c5ff <.text+0x89b5ff>
14089c585: 4c 8b 54 24 78              	movq	0x78(%rsp), %r10
14089c58a: eb 73                       	jmp	0x14089c5ff <.text+0x89b5ff>
14089c58c: 4c 8b 94 24 a8 00 00 00     	movq	0xa8(%rsp), %r10
14089c594: eb 69                       	jmp	0x14089c5ff <.text+0x89b5ff>
14089c596: 4c 8b 54 24 38              	movq	0x38(%rsp), %r10
14089c59b: eb 62                       	jmp	0x14089c5ff <.text+0x89b5ff>
14089c59d: 4c 8b 54 24 70              	movq	0x70(%rsp), %r10
14089c5a2: eb 5b                       	jmp	0x14089c5ff <.text+0x89b5ff>
14089c5a4: 48 81 c1 d8 00 00 00        	addq	$0xd8, %rcx
14089c5ab: 49 89 ca                    	movq	%rcx, %r10
14089c5ae: eb 4f                       	jmp	0x14089c5ff <.text+0x89b5ff>
14089c5b0: 4c 8b 54 24 50              	movq	0x50(%rsp), %r10
14089c5b5: eb 48                       	jmp	0x14089c5ff <.text+0x89b5ff>
14089c5b7: 4c 8b 54 24 60              	movq	0x60(%rsp), %r10
14089c5bc: eb 41                       	jmp	0x14089c5ff <.text+0x89b5ff>
14089c5be: 4c 8b 94 24 c8 00 00 00     	movq	0xc8(%rsp), %r10
14089c5c6: eb 37                       	jmp	0x14089c5ff <.text+0x89b5ff>
14089c5c8: 4c 8b 94 24 b8 00 00 00     	movq	0xb8(%rsp), %r10
14089c5d0: eb 2d                       	jmp	0x14089c5ff <.text+0x89b5ff>
14089c5d2: 49 89 fa                    	movq	%rdi, %r10
14089c5d5: eb 28                       	jmp	0x14089c5ff <.text+0x89b5ff>
14089c5d7: 4c 8b 54 24 10              	movq	0x10(%rsp), %r10
14089c5dc: eb 21                       	jmp	0x14089c5ff <.text+0x89b5ff>
14089c5de: 4c 8b 94 24 a0 00 00 00     	movq	0xa0(%rsp), %r10
14089c5e6: eb 17                       	jmp	0x14089c5ff <.text+0x89b5ff>
14089c5e8: 4c 8b 94 24 d8 00 00 00     	movq	0xd8(%rsp), %r10
14089c5f0: eb 0d                       	jmp	0x14089c5ff <.text+0x89b5ff>
14089c5f2: 4c 8b 94 24 d0 00 00 00     	movq	0xd0(%rsp), %r10
14089c5fa: eb 03                       	jmp	0x14089c5ff <.text+0x89b5ff>
14089c5fc: 4d 89 f2                    	movq	%r14, %r10
14089c5ff: 48 8b 8c 24 e0 00 00 00     	movq	0xe0(%rsp), %rcx
14089c607: 8b 41 08                    	movl	0x8(%rcx), %eax
14089c60a: 41 89 42 08                 	movl	%eax, 0x8(%r10)
14089c60e: 48 8b 01                    	movq	(%rcx), %rax
14089c611: 49 89 02                    	movq	%rax, (%r10)
14089c614: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
14089c61b: 5b                          	popq	%rbx
14089c61c: 5d                          	popq	%rbp
14089c61d: 5f                          	popq	%rdi
14089c61e: 5e                          	popq	%rsi
14089c61f: 41 5c                       	popq	%r12
14089c621: 41 5d                       	popq	%r13
14089c623: 41 5e                       	popq	%r14
14089c625: 41 5f                       	popq	%r15
14089c627: c3                          	retq
14089c628: 66 39 a9 56 01 00 00        	cmpw	%bp, 0x156(%rcx)
14089c62f: 0f 84 de ec ff ff           	je	0x14089b313 <.text+0x89a313>
14089c635: e9 e0 ec ff ff              	jmp	0x14089b31a <.text+0x89a31a>
14089c63a: 66 39 a9 62 01 00 00        	cmpw	%bp, 0x162(%rcx)
14089c641: 0f 84 16 ed ff ff           	je	0x14089b35d <.text+0x89a35d>
14089c647: e9 18 ed ff ff              	jmp	0x14089b364 <.text+0x89a364>
14089c64c: 66 39 a9 6e 01 00 00        	cmpw	%bp, 0x16e(%rcx)
14089c653: 0f 84 4e ed ff ff           	je	0x14089b3a7 <.text+0x89a3a7>
14089c659: e9 4f ed ff ff              	jmp	0x14089b3ad <.text+0x89a3ad>
14089c65e: 66 39 a9 7a 01 00 00        	cmpw	%bp, 0x17a(%rcx)
14089c665: 0f 84 85 ed ff ff           	je	0x14089b3f0 <.text+0x89a3f0>
14089c66b: e9 86 ed ff ff              	jmp	0x14089b3f6 <.text+0x89a3f6>
14089c670: 66 39 a9 86 01 00 00        	cmpw	%bp, 0x186(%rcx)
14089c677: 0f 84 bc ed ff ff           	je	0x14089b439 <.text+0x89a439>
14089c67d: e9 be ed ff ff              	jmp	0x14089b440 <.text+0x89a440>
14089c682: 66 39 a9 92 01 00 00        	cmpw	%bp, 0x192(%rcx)
14089c689: 0f 84 f4 ed ff ff           	je	0x14089b483 <.text+0x89a483>
14089c68f: e9 f6 ed ff ff              	jmp	0x14089b48a <.text+0x89a48a>
14089c694: 66 39 a9 9e 01 00 00        	cmpw	%bp, 0x19e(%rcx)
14089c69b: 0f 84 2c ee ff ff           	je	0x14089b4cd <.text+0x89a4cd>
14089c6a1: e9 2f ee ff ff              	jmp	0x14089b4d5 <.text+0x89a4d5>
14089c6a6: 66 39 a9 aa 01 00 00        	cmpw	%bp, 0x1aa(%rcx)
14089c6ad: 0f 84 75 ee ff ff           	je	0x14089b528 <.text+0x89a528>
14089c6b3: e9 7e ee ff ff              	jmp	0x14089b536 <.text+0x89a536>
14089c6b8: 66 39 69 36                 	cmpw	%bp, 0x36(%rcx)
14089c6bc: 0f 84 b9 ee ff ff           	je	0x14089b57b <.text+0x89a57b>
14089c6c2: e9 c2 ee ff ff              	jmp	0x14089b589 <.text+0x89a589>
14089c6c7: 66 39 69 42                 	cmpw	%bp, 0x42(%rcx)
14089c6cb: 0f 84 f5 ee ff ff           	je	0x14089b5c6 <.text+0x89a5c6>
14089c6d1: e9 f7 ee ff ff              	jmp	0x14089b5cd <.text+0x89a5cd>
14089c6d6: 66 39 69 1e                 	cmpw	%bp, 0x1e(%rcx)
14089c6da: 0f 84 32 ef ff ff           	je	0x14089b612 <.text+0x89a612>
14089c6e0: e9 3b ef ff ff              	jmp	0x14089b620 <.text+0x89a620>
14089c6e5: 66 39 69 2a                 	cmpw	%bp, 0x2a(%rcx)
14089c6e9: 0f 84 76 ef ff ff           	je	0x14089b665 <.text+0x89a665>
14089c6ef: e9 7f ef ff ff              	jmp	0x14089b673 <.text+0x89a673>
14089c6f4: 66 39 69 06                 	cmpw	%bp, 0x6(%rcx)
14089c6f8: 0f 84 ad ef ff ff           	je	0x14089b6ab <.text+0x89a6ab>
14089c6fe: e9 ae ef ff ff              	jmp	0x14089b6b1 <.text+0x89a6b1>
14089c703: 66 39 69 12                 	cmpw	%bp, 0x12(%rcx)
14089c707: 0f 84 e6 ef ff ff           	je	0x14089b6f3 <.text+0x89a6f3>
14089c70d: e9 ec ef ff ff              	jmp	0x14089b6fe <.text+0x89a6fe>
14089c712: 66 39 69 4e                 	cmpw	%bp, 0x4e(%rcx)
14089c716: 0f 84 24 f0 ff ff           	je	0x14089b740 <.text+0x89a740>
14089c71c: e9 2a f0 ff ff              	jmp	0x14089b74b <.text+0x89a74b>
14089c721: 66 39 69 5a                 	cmpw	%bp, 0x5a(%rcx)
14089c725: 0f 84 62 f0 ff ff           	je	0x14089b78d <.text+0x89a78d>
14089c72b: e9 68 f0 ff ff              	jmp	0x14089b798 <.text+0x89a798>
14089c730: 66 39 69 66                 	cmpw	%bp, 0x66(%rcx)
14089c734: 0f 84 a0 f0 ff ff           	je	0x14089b7da <.text+0x89a7da>
14089c73a: e9 a6 f0 ff ff              	jmp	0x14089b7e5 <.text+0x89a7e5>
14089c73f: 66 39 69 72                 	cmpw	%bp, 0x72(%rcx)
14089c743: 0f 84 de f0 ff ff           	je	0x14089b827 <.text+0x89a827>
14089c749: e9 e4 f0 ff ff              	jmp	0x14089b832 <.text+0x89a832>
14089c74e: 66 39 69 7e                 	cmpw	%bp, 0x7e(%rcx)
14089c752: 0f 84 1c f1 ff ff           	je	0x14089b874 <.text+0x89a874>
14089c758: e9 22 f1 ff ff              	jmp	0x14089b87f <.text+0x89a87f>
14089c75d: 66 39 a9 8a 00 00 00        	cmpw	%bp, 0x8a(%rcx)
14089c764: 0f 84 5d f1 ff ff           	je	0x14089b8c7 <.text+0x89a8c7>
14089c76a: e9 63 f1 ff ff              	jmp	0x14089b8d2 <.text+0x89a8d2>
14089c76f: 66 39 a9 96 00 00 00        	cmpw	%bp, 0x96(%rcx)
14089c776: 0f 84 9e f1 ff ff           	je	0x14089b91a <.text+0x89a91a>
14089c77c: e9 a4 f1 ff ff              	jmp	0x14089b925 <.text+0x89a925>
14089c781: 66 39 a9 a2 00 00 00        	cmpw	%bp, 0xa2(%rcx)
14089c788: 0f 84 df f1 ff ff           	je	0x14089b96d <.text+0x89a96d>
14089c78e: e9 e5 f1 ff ff              	jmp	0x14089b978 <.text+0x89a978>
14089c793: 66 39 a9 ae 00 00 00        	cmpw	%bp, 0xae(%rcx)
14089c79a: 0f 84 20 f2 ff ff           	je	0x14089b9c0 <.text+0x89a9c0>
14089c7a0: e9 26 f2 ff ff              	jmp	0x14089b9cb <.text+0x89a9cb>
14089c7a5: 66 39 a9 ba 00 00 00        	cmpw	%bp, 0xba(%rcx)
14089c7ac: 0f 84 61 f2 ff ff           	je	0x14089ba13 <.text+0x89aa13>
14089c7b2: e9 67 f2 ff ff              	jmp	0x14089ba1e <.text+0x89aa1e>
14089c7b7: 66 39 a9 c6 00 00 00        	cmpw	%bp, 0xc6(%rcx)
14089c7be: 0f 84 a2 f2 ff ff           	je	0x14089ba66 <.text+0x89aa66>
14089c7c4: e9 a8 f2 ff ff              	jmp	0x14089ba71 <.text+0x89aa71>
14089c7c9: 66 39 a9 d2 00 00 00        	cmpw	%bp, 0xd2(%rcx)
14089c7d0: 0f 84 e3 f2 ff ff           	je	0x14089bab9 <.text+0x89aab9>
14089c7d6: e9 e9 f2 ff ff              	jmp	0x14089bac4 <.text+0x89aac4>
14089c7db: 66 39 a9 ea 00 00 00        	cmpw	%bp, 0xea(%rcx)
14089c7e2: 0f 84 24 f3 ff ff           	je	0x14089bb0c <.text+0x89ab0c>
14089c7e8: e9 2a f3 ff ff              	jmp	0x14089bb17 <.text+0x89ab17>
14089c7ed: 66 39 a9 f6 00 00 00        	cmpw	%bp, 0xf6(%rcx)
14089c7f4: 0f 84 6d f3 ff ff           	je	0x14089bb67 <.text+0x89ab67>
14089c7fa: e9 73 f3 ff ff              	jmp	0x14089bb72 <.text+0x89ab72>
14089c7ff: 66 39 a9 02 01 00 00        	cmpw	%bp, 0x102(%rcx)
14089c806: 0f 84 b1 f3 ff ff           	je	0x14089bbbd <.text+0x89abbd>
14089c80c: e9 b3 f3 ff ff              	jmp	0x14089bbc4 <.text+0x89abc4>
14089c811: 66 39 a9 0e 01 00 00        	cmpw	%bp, 0x10e(%rcx)
14089c818: 0f 84 f6 f3 ff ff           	je	0x14089bc14 <.text+0x89ac14>
14089c81e: e9 f7 f3 ff ff              	jmp	0x14089bc1a <.text+0x89ac1a>
14089c823: 66 39 a9 1a 01 00 00        	cmpw	%bp, 0x11a(%rcx)
14089c82a: 0f 84 3a f4 ff ff           	je	0x14089bc6a <.text+0x89ac6a>
14089c830: e9 3b f4 ff ff              	jmp	0x14089bc70 <.text+0x89ac70>
14089c835: 66 39 a9 26 01 00 00        	cmpw	%bp, 0x126(%rcx)
14089c83c: 0f 84 7f f4 ff ff           	je	0x14089bcc1 <.text+0x89acc1>
14089c842: e9 80 f4 ff ff              	jmp	0x14089bcc7 <.text+0x89acc7>
14089c847: 66 39 a9 32 01 00 00        	cmpw	%bp, 0x132(%rcx)
14089c84e: 0f 84 d3 f4 ff ff           	je	0x14089bd27 <.text+0x89ad27>
14089c854: e9 d4 f4 ff ff              	jmp	0x14089bd2d <.text+0x89ad2d>
14089c859: 66 39 a9 3e 01 00 00        	cmpw	%bp, 0x13e(%rcx)
14089c860: 0f 84 1a f5 ff ff           	je	0x14089bd80 <.text+0x89ad80>
14089c866: e9 1c f5 ff ff              	jmp	0x14089bd87 <.text+0x89ad87>
14089c86b: 66 44 39 99 4a 01 00 00     	cmpw	%r11w, 0x14a(%rcx)
14089c873: 0f 84 5d f5 ff ff           	je	0x14089bdd6 <.text+0x89add6>
14089c879: e9 5f f5 ff ff              	jmp	0x14089bddd <.text+0x89addd>
14089c87e: cc                          	int3
14089c87f: cc                          	int3
14089c880: 0f b6 09                    	movzbl	(%rcx), %ecx
14089c883: 0f b7 84 8a bc 01 00 00     	movzwl	0x1bc(%rdx,%rcx,4), %eax
14089c88b: 0f b7 94 8a be 01 00 00     	movzwl	0x1be(%rdx,%rcx,4), %edx
14089c893: c3                          	retq
14089c894: cc                          	int3
14089c895: cc                          	int3
14089c896: cc                          	int3
14089c897: cc                          	int3
14089c898: cc                          	int3
14089c899: cc                          	int3
14089c89a: cc                          	int3
14089c89b: cc                          	int3
14089c89c: cc                          	int3
14089c89d: cc                          	int3
14089c89e: cc                          	int3
14089c89f: cc                          	int3
14089c8a0: 41 57                       	pushq	%r15
14089c8a2: 41 56                       	pushq	%r14
14089c8a4: 56                          	pushq	%rsi
14089c8a5: 57                          	pushq	%rdi
14089c8a6: 53                          	pushq	%rbx
14089c8a7: 48 83 ec 20                 	subq	$0x20, %rsp
14089c8ab: 48 89 ce                    	movq	%rcx, %rsi
14089c8ae: 0f b6 02                    	movzbl	(%rdx), %eax
14089c8b1: 85 c0                       	testl	%eax, %eax
14089c8b3: 74 13                       	je	0x14089c8c8 <.text+0x89b8c8>
14089c8b5: 83 f8 01                    	cmpl	$0x1, %eax
14089c8b8: 75 1c                       	jne	0x14089c8d6 <.text+0x89b8d6>
14089c8ba: 48 8d 0d 39 c7 1f 02        	leaq	0x21fc739(%rip), %rcx   # 0x142a98ffa
14089c8c1: ba 10 00 00 00              	movl	$0x10, %edx
14089c8c6: eb 1a                       	jmp	0x14089c8e2 <.text+0x89b8e2>
14089c8c8: 48 8d 0d 19 c7 1f 02        	leaq	0x21fc719(%rip), %rcx   # 0x142a98fe8
14089c8cf: ba 12 00 00 00              	movl	$0x12, %edx
14089c8d4: eb 0c                       	jmp	0x14089c8e2 <.text+0x89b8e2>
14089c8d6: 48 8d 0d 2d c7 1f 02        	leaq	0x21fc72d(%rip), %rcx   # 0x142a9900a
14089c8dd: ba 1b 00 00 00              	movl	$0x1b, %edx
14089c8e2: e8 99 0b 30 00              	callq	0x140b9d480 <.text+0xb9c480>
14089c8e7: 48 89 c3                    	movq	%rax, %rbx
14089c8ea: 48 89 d7                    	movq	%rdx, %rdi
14089c8ed: 48 85 d2                    	testq	%rdx, %rdx
14089c8f0: 78 5c                       	js	0x14089c94e <.text+0x89b94e>
14089c8f2: 74 24                       	je	0x14089c918 <.text+0x89b918>
14089c8f4: 0f b6 05 0e 83 d9 02        	movzbl	0x2d9830e(%rip), %eax   # 0x143634c09
14089c8fb: 41 bf 01 00 00 00           	movl	$0x1, %r15d
14089c901: ba 01 00 00 00              	movl	$0x1, %edx
14089c906: 48 89 f9                    	movq	%rdi, %rcx
14089c909: e8 02 73 d7 ff              	callq	0x140613c10 <.text+0x612c10>
14089c90e: 48 85 c0                    	testq	%rax, %rax
14089c911: 74 3e                       	je	0x14089c951 <.text+0x89b951>
14089c913: 49 89 c6                    	movq	%rax, %r14
14089c916: eb 06                       	jmp	0x14089c91e <.text+0x89b91e>
14089c918: 41 be 01 00 00 00           	movl	$0x1, %r14d
14089c91e: 4c 89 f1                    	movq	%r14, %rcx
14089c921: 48 89 da                    	movq	%rbx, %rdx
14089c924: 49 89 f8                    	movq	%rdi, %r8
14089c927: e8 d4 72 08 02              	callq	0x142923c00 <.text+0x2922c00>
14089c92c: 48 89 3e                    	movq	%rdi, (%rsi)
14089c92f: 4c 89 76 08                 	movq	%r14, 0x8(%rsi)
14089c933: 48 89 7e 10                 	movq	%rdi, 0x10(%rsi)
14089c937: 48 c7 46 18 00 00 00 00     	movq	$0x0, 0x18(%rsi)
14089c93f: 48 89 f0                    	movq	%rsi, %rax
14089c942: 48 83 c4 20                 	addq	$0x20, %rsp
14089c946: 5b                          	popq	%rbx
14089c947: 5f                          	popq	%rdi
14089c948: 5e                          	popq	%rsi
14089c949: 41 5e                       	popq	%r14
14089c94b: 41 5f                       	popq	%r15
14089c94d: c3                          	retq
14089c94e: 45 31 ff                    	xorl	%r15d, %r15d
14089c951: 4c 8d 05 48 54 1f 02        	leaq	0x21f5448(%rip), %r8    # 0x142a91da0
14089c958: 4c 89 f9                    	movq	%r15, %rcx
14089c95b: 48 89 fa                    	movq	%rdi, %rdx
14089c95e: e8 80 c6 03 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14089c963: cc                          	int3
14089c964: cc                          	int3
14089c965: cc                          	int3
14089c966: cc                          	int3
14089c967: cc                          	int3
14089c968: cc                          	int3
14089c969: cc                          	int3
14089c96a: cc                          	int3
14089c96b: cc                          	int3
14089c96c: cc                          	int3
14089c96d: cc                          	int3
14089c96e: cc                          	int3
14089c96f: cc                          	int3
14089c970: 56                          	pushq	%rsi
14089c971: 48 83 ec 70                 	subq	$0x70, %rsp
14089c975: 48 89 ce                    	movq	%rcx, %rsi
14089c978: 0f b7 c2                    	movzwl	%dx, %eax
14089c97b: 48 8d 0d fe e0 1f 02        	leaq	0x21fe0fe(%rip), %rcx   # 0x142a9aa80
14089c982: 48 63 04 81                 	movslq	(%rcx,%rax,4), %rax
14089c986: 48 01 c8                    	addq	%rcx, %rax
14089c989: ff e0                       	jmpq	*%rax
14089c98b: 0f b6 05 77 82 d9 02        	movzbl	0x2d98277(%rip), %eax   # 0x143634c09
14089c992: b9 03 00 00 00              	movl	$0x3, %ecx
14089c997: ba 01 00 00 00              	movl	$0x1, %edx
14089c99c: e8 6f 72 d7 ff              	callq	0x140613c10 <.text+0x612c10>
14089c9a1: 48 85 c0                    	testq	%rax, %rax
14089c9a4: 0f 84 65 01 00 00           	je	0x14089cb0f <.text+0x89bb0f>
14089c9aa: c6 40 02 42                 	movb	$0x42, 0x2(%rax)
14089c9ae: 66 c7 00 4c 4d              	movw	$0x4d4c, (%rax)         # imm = 0x4D4C
14089c9b3: e9 df 00 00 00              	jmp	0x14089ca97 <.text+0x89ba97>
14089c9b8: 0f b6 05 4a 82 d9 02        	movzbl	0x2d9824a(%rip), %eax   # 0x143634c09
14089c9bf: b9 14 00 00 00              	movl	$0x14, %ecx
14089c9c4: ba 01 00 00 00              	movl	$0x1, %edx
14089c9c9: e8 42 72 d7 ff              	callq	0x140613c10 <.text+0x612c10>
14089c9ce: 48 85 c0                    	testq	%rax, %rax
14089c9d1: 0f 84 4e 01 00 00           	je	0x14089cb25 <.text+0x89bb25>
14089c9d7: 0f 10 05 58 c6 1f 02        	movups	0x21fc658(%rip), %xmm0  # 0x142a99036
14089c9de: 0f 11 00                    	movups	%xmm0, (%rax)
14089c9e1: c7 40 10 74 74 6f 6e        	movl	$0x6e6f7474, 0x10(%rax) # imm = 0x6E6F7474
14089c9e8: 48 c7 06 14 00 00 00        	movq	$0x14, (%rsi)
14089c9ef: 48 89 46 08                 	movq	%rax, 0x8(%rsi)
14089c9f3: 48 c7 46 10 14 00 00 00     	movq	$0x14, 0x10(%rsi)
14089c9fb: e9 06 01 00 00              	jmp	0x14089cb06 <.text+0x89bb06>
14089ca00: 0f b6 05 02 82 d9 02        	movzbl	0x2d98202(%rip), %eax   # 0x143634c09
14089ca07: b9 03 00 00 00              	movl	$0x3, %ecx
14089ca0c: ba 01 00 00 00              	movl	$0x1, %edx
14089ca11: e8 fa 71 d7 ff              	callq	0x140613c10 <.text+0x612c10>
14089ca16: 48 85 c0                    	testq	%rax, %rax
14089ca19: 0f 84 f0 00 00 00           	je	0x14089cb0f <.text+0x89bb0f>
14089ca1f: c6 40 02 42                 	movb	$0x42, 0x2(%rax)
14089ca23: 66 c7 00 4d 4d              	movw	$0x4d4d, (%rax)         # imm = 0x4D4D
14089ca28: eb 6d                       	jmp	0x14089ca97 <.text+0x89ba97>
14089ca2a: 0f b6 05 d8 81 d9 02        	movzbl	0x2d981d8(%rip), %eax   # 0x143634c09
14089ca31: b9 11 00 00 00              	movl	$0x11, %ecx
14089ca36: ba 01 00 00 00              	movl	$0x1, %edx
14089ca3b: e8 d0 71 d7 ff              	callq	0x140613c10 <.text+0x612c10>
14089ca40: 48 85 c0                    	testq	%rax, %rax
14089ca43: 0f 84 f2 00 00 00           	je	0x14089cb3b <.text+0x89bb3b>
14089ca49: 0f 10 05 d5 c5 1f 02        	movups	0x21fc5d5(%rip), %xmm0  # 0x142a99025
14089ca50: 0f 11 00                    	movups	%xmm0, (%rax)
14089ca53: c6 40 10 6e                 	movb	$0x6e, 0x10(%rax)
14089ca57: 48 c7 06 11 00 00 00        	movq	$0x11, (%rsi)
14089ca5e: 48 89 46 08                 	movq	%rax, 0x8(%rsi)
14089ca62: 48 c7 46 10 11 00 00 00     	movq	$0x11, 0x10(%rsi)
14089ca6a: e9 97 00 00 00              	jmp	0x14089cb06 <.text+0x89bb06>
14089ca6f: 0f b6 05 93 81 d9 02        	movzbl	0x2d98193(%rip), %eax   # 0x143634c09
14089ca76: b9 03 00 00 00              	movl	$0x3, %ecx
14089ca7b: ba 01 00 00 00              	movl	$0x1, %edx
14089ca80: e8 8b 71 d7 ff              	callq	0x140613c10 <.text+0x612c10>
14089ca85: 48 85 c0                    	testq	%rax, %rax
14089ca88: 0f 84 81 00 00 00           	je	0x14089cb0f <.text+0x89bb0f>
14089ca8e: c6 40 02 42                 	movb	$0x42, 0x2(%rax)
14089ca92: 66 c7 00 52 4d              	movw	$0x4d52, (%rax)         # imm = 0x4D52
14089ca97: 48 c7 06 03 00 00 00        	movq	$0x3, (%rsi)
14089ca9e: 48 89 46 08                 	movq	%rax, 0x8(%rsi)
14089caa2: 48 c7 46 10 03 00 00 00     	movq	$0x3, 0x10(%rsi)
14089caaa: eb 5a                       	jmp	0x14089cb06 <.text+0x89bb06>
14089caac: 66 44 89 44 24 2e           	movw	%r8w, 0x2e(%rsp)
14089cab2: 48 8d 44 24 2e              	leaq	0x2e(%rsp), %rax
14089cab7: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
14089cabc: 48 8d 05 4d 08 4f 00        	leaq	0x4f084d(%rip), %rax    # 0x140d8d310 <.text+0xd8c310>
14089cac3: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
14089cac8: 48 8d 05 89 c5 1f 02        	leaq	0x21fc589(%rip), %rax   # 0x142a99058
14089cacf: 48 89 44 24 40              	movq	%rax, 0x40(%rsp)
14089cad4: 48 c7 44 24 48 01 00 00 00  	movq	$0x1, 0x48(%rsp)
14089cadd: 48 8d 44 24 30              	leaq	0x30(%rsp), %rax
14089cae2: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
14089cae7: 48 c7 44 24 58 01 00 00 00  	movq	$0x1, 0x58(%rsp)
14089caf0: 48 c7 44 24 60 00 00 00 00  	movq	$0x0, 0x60(%rsp)
14089caf9: 48 8d 54 24 40              	leaq	0x40(%rsp), %rdx
14089cafe: 48 89 f1                    	movq	%rsi, %rcx
14089cb01: e8 8a d4 4d 00              	callq	0x140d79f90 <.text+0xd78f90>
14089cb06: 48 89 f0                    	movq	%rsi, %rax
14089cb09: 48 83 c4 70                 	addq	$0x70, %rsp
14089cb0d: 5e                          	popq	%rsi
14089cb0e: c3                          	retq
14089cb0f: 4c 8d 05 0a c4 1f 02        	leaq	0x21fc40a(%rip), %r8    # 0x142a98f20
14089cb16: b9 01 00 00 00              	movl	$0x1, %ecx
14089cb1b: ba 03 00 00 00              	movl	$0x3, %edx
14089cb20: e8 be c4 03 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14089cb25: 4c 8d 05 f4 c3 1f 02        	leaq	0x21fc3f4(%rip), %r8    # 0x142a98f20
14089cb2c: b9 01 00 00 00              	movl	$0x1, %ecx
14089cb31: ba 14 00 00 00              	movl	$0x14, %edx
14089cb36: e8 a8 c4 03 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14089cb3b: 4c 8d 05 de c3 1f 02        	leaq	0x21fc3de(%rip), %r8    # 0x142a98f20
14089cb42: b9 01 00 00 00              	movl	$0x1, %ecx
14089cb47: ba 11 00 00 00              	movl	$0x11, %edx
14089cb4c: e8 92 c4 03 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14089cb51: cc                          	int3
14089cb52: cc                          	int3
14089cb53: cc                          	int3
14089cb54: cc                          	int3
14089cb55: cc                          	int3
14089cb56: cc                          	int3
14089cb57: cc                          	int3
14089cb58: cc                          	int3
14089cb59: cc                          	int3
14089cb5a: cc                          	int3
14089cb5b: cc                          	int3
14089cb5c: cc                          	int3
14089cb5d: cc                          	int3
14089cb5e: cc                          	int3
14089cb5f: cc                          	int3
14089cb60: 41 57                       	pushq	%r15
14089cb62: 41 56                       	pushq	%r14
14089cb64: 56                          	pushq	%rsi
14089cb65: 57                          	pushq	%rdi
14089cb66: 53                          	pushq	%rbx
14089cb67: 48 81 ec 90 00 00 00        	subq	$0x90, %rsp
14089cb6e: 48 89 ce                    	movq	%rcx, %rsi
14089cb71: 0f b6 02                    	movzbl	(%rdx), %eax
14089cb74: 48 8d 0d 1d df 1f 02        	leaq	0x21fdf1d(%rip), %rcx   # 0x142a9aa98
14089cb7b: 48 63 04 81                 	movslq	(%rcx,%rax,4), %rax
14089cb7f: 48 01 c8                    	addq	%rcx, %rax
14089cb82: ff e0                       	jmpq	*%rax
14089cb84: 48 8d 0d dd c4 1f 02        	leaq	0x21fc4dd(%rip), %rcx   # 0x142a99068
14089cb8b: e9 af 04 00 00              	jmp	0x14089d03f <.text+0x89c03f>
14089cb90: 0f b6 05 72 80 d9 02        	movzbl	0x2d98072(%rip), %eax   # 0x143634c09
14089cb97: b9 0b 00 00 00              	movl	$0xb, %ecx
14089cb9c: ba 01 00 00 00              	movl	$0x1, %edx
14089cba1: e8 6a 70 d7 ff              	callq	0x140613c10 <.text+0x612c10>
14089cba6: 48 85 c0                    	testq	%rax, %rax
14089cba9: 0f 84 a8 06 00 00           	je	0x14089d257 <.text+0x89c257>
14089cbaf: 48 b9 55 49 20 54 61 62 20 4e       	movabsq	$0x4e20626154204955, %rcx # imm = 0x4E20626154204955
14089cbb9: 48 89 08                    	movq	%rcx, (%rax)
14089cbbc: c7 40 07 4e 65 78 74        	movl	$0x7478654e, 0x7(%rax)  # imm = 0x7478654E
14089cbc3: e9 a4 00 00 00              	jmp	0x14089cc6c <.text+0x89bc6c>
14089cbc8: 0f b6 05 3a 80 d9 02        	movzbl	0x2d9803a(%rip), %eax   # 0x143634c09
14089cbcf: b9 16 00 00 00              	movl	$0x16, %ecx
14089cbd4: ba 01 00 00 00              	movl	$0x1, %edx
14089cbd9: e8 32 70 d7 ff              	callq	0x140613c10 <.text+0x612c10>
14089cbde: 48 85 c0                    	testq	%rax, %rax
14089cbe1: 0f 84 9c 06 00 00           	je	0x14089d283 <.text+0x89c283>
14089cbe7: 0f 10 05 9c c6 1f 02        	movups	0x21fc69c(%rip), %xmm0  # 0x142a9928a
14089cbee: 0f 11 00                    	movups	%xmm0, (%rax)
14089cbf1: 48 b9 6e 61 70 73 68 6f 74 73       	movabsq	$0x73746f687370616e, %rcx # imm = 0x73746F687370616E
14089cbfb: 48 89 48 0e                 	movq	%rcx, 0xe(%rax)
14089cbff: 48 c7 06 16 00 00 00        	movq	$0x16, (%rsi)
14089cc06: 48 89 46 08                 	movq	%rax, 0x8(%rsi)
14089cc0a: 48 c7 46 10 16 00 00 00     	movq	$0x16, 0x10(%rsi)
14089cc12: e9 8a 04 00 00              	jmp	0x14089d0a1 <.text+0x89c0a1>
14089cc17: 48 8d 0d 4f c6 1f 02        	leaq	0x21fc64f(%rip), %rcx   # 0x142a9926d
14089cc1e: ba 1d 00 00 00              	movl	$0x1d, %edx
14089cc23: e8 58 08 30 00              	callq	0x140b9d480 <.text+0xb9c480>
14089cc28: 48 89 d7                    	movq	%rdx, %rdi
14089cc2b: 48 85 d2                    	testq	%rdx, %rdx
14089cc2e: 0f 89 21 04 00 00           	jns	0x14089d055 <.text+0x89c055>
14089cc34: e9 55 05 00 00              	jmp	0x14089d18e <.text+0x89c18e>
14089cc39: 0f b6 05 c9 7f d9 02        	movzbl	0x2d97fc9(%rip), %eax   # 0x143634c09
14089cc40: b9 0b 00 00 00              	movl	$0xb, %ecx
14089cc45: ba 01 00 00 00              	movl	$0x1, %edx
14089cc4a: e8 c1 6f d7 ff              	callq	0x140613c10 <.text+0x612c10>
14089cc4f: 48 85 c0                    	testq	%rax, %rax
14089cc52: 0f 84 ff 05 00 00           	je	0x14089d257 <.text+0x89c257>
14089cc58: 48 b9 55 49 20 54 61 62 20 50       	movabsq	$0x5020626154204955, %rcx # imm = 0x5020626154204955
14089cc62: 48 89 08                    	movq	%rcx, (%rax)
14089cc65: c7 40 07 50 72 65 76        	movl	$0x76657250, 0x7(%rax)  # imm = 0x76657250
14089cc6c: 48 c7 06 0b 00 00 00        	movq	$0xb, (%rsi)
14089cc73: 48 89 46 08                 	movq	%rax, 0x8(%rsi)
14089cc77: 48 c7 46 10 0b 00 00 00     	movq	$0xb, 0x10(%rsi)
14089cc7f: e9 1d 04 00 00              	jmp	0x14089d0a1 <.text+0x89c0a1>
14089cc84: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14089cc8e: 48 89 06                    	movq	%rax, (%rsi)
14089cc91: 48 8d 05 1d c5 1f 02        	leaq	0x21fc51d(%rip), %rax   # 0x142a991b5
14089cc98: eb 14                       	jmp	0x14089ccae <.text+0x89bcae>
14089cc9a: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14089cca4: 48 89 06                    	movq	%rax, (%rsi)
14089cca7: 48 8d 05 ea c4 1f 02        	leaq	0x21fc4ea(%rip), %rax   # 0x142a99198
14089ccae: 48 89 46 18                 	movq	%rax, 0x18(%rsi)
14089ccb2: 48 c7 46 20 0e 00 00 00     	movq	$0xe, 0x20(%rsi)
14089ccba: e9 ea 03 00 00              	jmp	0x14089d0a9 <.text+0x89c0a9>
14089ccbf: 0f b6 05 43 7f d9 02        	movzbl	0x2d97f43(%rip), %eax   # 0x143634c09
14089ccc6: b9 05 00 00 00              	movl	$0x5, %ecx
14089cccb: ba 01 00 00 00              	movl	$0x1, %edx
14089ccd0: e8 3b 6f d7 ff              	callq	0x140613c10 <.text+0x612c10>
14089ccd5: 48 85 c0                    	testq	%rax, %rax
14089ccd8: 0f 84 bb 05 00 00           	je	0x14089d299 <.text+0x89c299>
14089ccde: c6 40 04 70                 	movb	$0x70, 0x4(%rax)
14089cce2: c7 00 55 49 20 55           	movl	$0x55204955, (%rax)     # imm = 0x55204955
14089cce8: 48 c7 06 05 00 00 00        	movq	$0x5, (%rsi)
14089ccef: 48 89 46 08                 	movq	%rax, 0x8(%rsi)
14089ccf3: 48 c7 46 10 05 00 00 00     	movq	$0x5, 0x10(%rsi)
14089ccfb: e9 a1 03 00 00              	jmp	0x14089d0a1 <.text+0x89c0a1>
14089cd00: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14089cd0a: 48 89 06                    	movq	%rax, (%rsi)
14089cd0d: 48 8d 05 51 c4 1f 02        	leaq	0x21fc451(%rip), %rax   # 0x142a99165
14089cd14: e9 ad 01 00 00              	jmp	0x14089cec6 <.text+0x89bec6>
14089cd19: 48 8d 0d da c2 1f 02        	leaq	0x21fc2da(%rip), %rcx   # 0x142a98ffa
14089cd20: ba 10 00 00 00              	movl	$0x10, %edx
14089cd25: e8 56 07 30 00              	callq	0x140b9d480 <.text+0xb9c480>
14089cd2a: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
14089cd2f: 48 89 54 24 40              	movq	%rdx, 0x40(%rsp)
14089cd34: 48 8d 44 24 38              	leaq	0x38(%rsp), %rax
14089cd39: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
14089cd3e: 48 8d 05 4b d1 86 ff        	leaq	-0x792eb5(%rip), %rax   # 0x140109e90 <.text+0x108e90>
14089cd45: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
14089cd4a: 48 8d 05 97 c3 1f 02        	leaq	0x21fc397(%rip), %rax   # 0x142a990e8
14089cd51: e9 49 02 00 00              	jmp	0x14089cf9f <.text+0x89bf9f>
14089cd56: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14089cd60: 48 89 06                    	movq	%rax, (%rsi)
14089cd63: 48 8d 05 07 c4 1f 02        	leaq	0x21fc407(%rip), %rax   # 0x142a99171
14089cd6a: 48 89 46 18                 	movq	%rax, 0x18(%rsi)
14089cd6e: 48 c7 46 20 12 00 00 00     	movq	$0x12, 0x20(%rsi)
14089cd76: e9 2e 03 00 00              	jmp	0x14089d0a9 <.text+0x89c0a9>
14089cd7b: 48 8d 0d c0 c3 1f 02        	leaq	0x21fc3c0(%rip), %rcx   # 0x142a99142
14089cd82: ba 13 00 00 00              	movl	$0x13, %edx
14089cd87: e8 f4 06 30 00              	callq	0x140b9d480 <.text+0xb9c480>
14089cd8c: 48 89 d7                    	movq	%rdx, %rdi
14089cd8f: 48 85 d2                    	testq	%rdx, %rdx
14089cd92: 0f 89 bd 02 00 00           	jns	0x14089d055 <.text+0x89c055>
14089cd98: e9 f1 03 00 00              	jmp	0x14089d18e <.text+0x89c18e>
14089cd9d: 48 8d 0d 97 c4 1f 02        	leaq	0x21fc497(%rip), %rcx   # 0x142a9923b
14089cda4: ba 1a 00 00 00              	movl	$0x1a, %edx
14089cda9: e8 d2 06 30 00              	callq	0x140b9d480 <.text+0xb9c480>
14089cdae: 48 89 d7                    	movq	%rdx, %rdi
14089cdb1: 48 85 d2                    	testq	%rdx, %rdx
14089cdb4: 0f 89 9b 02 00 00           	jns	0x14089d055 <.text+0x89c055>
14089cdba: e9 cf 03 00 00              	jmp	0x14089d18e <.text+0x89c18e>
14089cdbf: 48 8d 0d 5a c4 1f 02        	leaq	0x21fc45a(%rip), %rcx   # 0x142a99220
14089cdc6: e9 ad 03 00 00              	jmp	0x14089d178 <.text+0x89c178>
14089cdcb: 48 8d 0d 28 c2 1f 02        	leaq	0x21fc228(%rip), %rcx   # 0x142a98ffa
14089cdd2: ba 10 00 00 00              	movl	$0x10, %edx
14089cdd7: e9 97 01 00 00              	jmp	0x14089cf73 <.text+0x89bf73>
14089cddc: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14089cde6: 48 89 06                    	movq	%rax, (%rsi)
14089cde9: 48 8d 05 b6 c3 1f 02        	leaq	0x21fc3b6(%rip), %rax   # 0x142a991a6
14089cdf0: 48 89 46 18                 	movq	%rax, 0x18(%rsi)
14089cdf4: 48 c7 46 20 0f 00 00 00     	movq	$0xf, 0x20(%rsi)
14089cdfc: e9 a8 02 00 00              	jmp	0x14089d0a9 <.text+0x89c0a9>
14089ce01: 48 8d 0d f2 c1 1f 02        	leaq	0x21fc1f2(%rip), %rcx   # 0x142a98ffa
14089ce08: ba 10 00 00 00              	movl	$0x10, %edx
14089ce0d: eb 0c                       	jmp	0x14089ce1b <.text+0x89be1b>
14089ce0f: 48 8d 0d d2 c1 1f 02        	leaq	0x21fc1d2(%rip), %rcx   # 0x142a98fe8
14089ce16: ba 12 00 00 00              	movl	$0x12, %edx
14089ce1b: e8 60 06 30 00              	callq	0x140b9d480 <.text+0xb9c480>
14089ce20: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
14089ce25: 48 89 54 24 40              	movq	%rdx, 0x40(%rsp)
14089ce2a: 48 8d 44 24 38              	leaq	0x38(%rsp), %rax
14089ce2f: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
14089ce34: 48 8d 05 55 d0 86 ff        	leaq	-0x792fab(%rip), %rax   # 0x140109e90 <.text+0x108e90>
14089ce3b: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
14089ce40: 48 8d 05 79 c2 1f 02        	leaq	0x21fc279(%rip), %rax   # 0x142a990c0
14089ce47: e9 53 01 00 00              	jmp	0x14089cf9f <.text+0x89bf9f>
14089ce4c: 48 8d 0d 27 c2 1f 02        	leaq	0x21fc227(%rip), %rcx   # 0x142a9907a
14089ce53: ba 14 00 00 00              	movl	$0x14, %edx
14089ce58: e8 23 06 30 00              	callq	0x140b9d480 <.text+0xb9c480>
14089ce5d: 48 89 d7                    	movq	%rdx, %rdi
14089ce60: 48 85 d2                    	testq	%rdx, %rdx
14089ce63: 0f 89 ec 01 00 00           	jns	0x14089d055 <.text+0x89c055>
14089ce69: e9 20 03 00 00              	jmp	0x14089d18e <.text+0x89c18e>
14089ce6e: 0f b6 05 94 7d d9 02        	movzbl	0x2d97d94(%rip), %eax   # 0x143634c09
14089ce75: b9 08 00 00 00              	movl	$0x8, %ecx
14089ce7a: ba 01 00 00 00              	movl	$0x1, %edx
14089ce7f: e8 8c 6d d7 ff              	callq	0x140613c10 <.text+0x612c10>
14089ce84: 48 85 c0                    	testq	%rax, %rax
14089ce87: 0f 84 22 04 00 00           	je	0x14089d2af <.text+0x89c2af>
14089ce8d: 48 b9 55 49 20 52 69 67 68 74       	movabsq	$0x7468676952204955, %rcx # imm = 0x7468676952204955
14089ce97: 48 89 08                    	movq	%rcx, (%rax)
14089ce9a: 48 c7 06 08 00 00 00        	movq	$0x8, (%rsi)
14089cea1: 48 89 46 08                 	movq	%rax, 0x8(%rsi)
14089cea5: 48 c7 46 10 08 00 00 00     	movq	$0x8, 0x10(%rsi)
14089cead: e9 ef 01 00 00              	jmp	0x14089d0a1 <.text+0x89c0a1>
14089ceb2: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14089cebc: 48 89 06                    	movq	%rax, (%rsi)
14089cebf: 48 8d 05 fd c2 1f 02        	leaq	0x21fc2fd(%rip), %rax   # 0x142a991c3
14089cec6: 48 89 46 18                 	movq	%rax, 0x18(%rsi)
14089ceca: 48 c7 46 20 0c 00 00 00     	movq	$0xc, 0x20(%rsi)
14089ced2: e9 d2 01 00 00              	jmp	0x14089d0a9 <.text+0x89c0a9>
14089ced7: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14089cee1: 48 89 06                    	movq	%rax, (%rsi)
14089cee4: 48 8d 05 02 c3 1f 02        	leaq	0x21fc302(%rip), %rax   # 0x142a991ed
14089ceeb: 48 89 46 18                 	movq	%rax, 0x18(%rsi)
14089ceef: 48 c7 46 20 13 00 00 00     	movq	$0x13, 0x20(%rsi)
14089cef7: e9 ad 01 00 00              	jmp	0x14089d0a9 <.text+0x89c0a9>
14089cefc: 48 8d 0d f7 c0 1f 02        	leaq	0x21fc0f7(%rip), %rcx   # 0x142a98ffa
14089cf03: ba 10 00 00 00              	movl	$0x10, %edx
14089cf08: e8 73 05 30 00              	callq	0x140b9d480 <.text+0xb9c480>
14089cf0d: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
14089cf12: 48 89 54 24 40              	movq	%rdx, 0x40(%rsp)
14089cf17: 48 8d 44 24 38              	leaq	0x38(%rsp), %rax
14089cf1c: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
14089cf21: 48 8d 05 68 cf 86 ff        	leaq	-0x793098(%rip), %rax   # 0x140109e90 <.text+0x108e90>
14089cf28: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
14089cf2d: 48 8d 05 dc c1 1f 02        	leaq	0x21fc1dc(%rip), %rax   # 0x142a99110
14089cf34: eb 69                       	jmp	0x14089cf9f <.text+0x89bf9f>
14089cf36: 48 8d 0d c3 c2 1f 02        	leaq	0x21fc2c3(%rip), %rcx   # 0x142a99200
14089cf3d: e9 f2 01 00 00              	jmp	0x14089d134 <.text+0x89c134>
14089cf42: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14089cf4c: 48 89 06                    	movq	%rax, (%rsi)
14089cf4f: 48 8d 05 2d c2 1f 02        	leaq	0x21fc22d(%rip), %rax   # 0x142a99183
14089cf56: 48 89 46 18                 	movq	%rax, 0x18(%rsi)
14089cf5a: 48 c7 46 20 15 00 00 00     	movq	$0x15, 0x20(%rsi)
14089cf62: e9 42 01 00 00              	jmp	0x14089d0a9 <.text+0x89c0a9>
14089cf67: 48 8d 0d 7a c0 1f 02        	leaq	0x21fc07a(%rip), %rcx   # 0x142a98fe8
14089cf6e: ba 12 00 00 00              	movl	$0x12, %edx
14089cf73: e8 08 05 30 00              	callq	0x140b9d480 <.text+0xb9c480>
14089cf78: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
14089cf7d: 48 89 54 24 40              	movq	%rdx, 0x40(%rsp)
14089cf82: 48 8d 44 24 38              	leaq	0x38(%rsp), %rax
14089cf87: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
14089cf8c: 48 8d 05 fd ce 86 ff        	leaq	-0x793103(%rip), %rax   # 0x140109e90 <.text+0x108e90>
14089cf93: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
14089cf98: 48 8d 05 f9 c0 1f 02        	leaq	0x21fc0f9(%rip), %rax   # 0x142a99098
14089cf9f: 48 89 44 24 60              	movq	%rax, 0x60(%rsp)
14089cfa4: 48 c7 44 24 68 02 00 00 00  	movq	$0x2, 0x68(%rsp)
14089cfad: 48 8d 44 24 28              	leaq	0x28(%rsp), %rax
14089cfb2: 48 89 44 24 70              	movq	%rax, 0x70(%rsp)
14089cfb7: 48 c7 44 24 78 01 00 00 00  	movq	$0x1, 0x78(%rsp)
14089cfc0: 48 c7 84 24 80 00 00 00 00 00 00 00 	movq	$0x0, 0x80(%rsp)
14089cfcc: 48 8d 4c 24 48              	leaq	0x48(%rsp), %rcx
14089cfd1: 48 8d 54 24 60              	leaq	0x60(%rsp), %rdx
14089cfd6: e8 b5 cf 4d 00              	callq	0x140d79f90 <.text+0xd78f90>
14089cfdb: 48 8b 44 24 58              	movq	0x58(%rsp), %rax
14089cfe0: 48 89 46 10                 	movq	%rax, 0x10(%rsi)
14089cfe4: 0f 10 44 24 48              	movups	0x48(%rsp), %xmm0
14089cfe9: 0f 11 06                    	movups	%xmm0, (%rsi)
14089cfec: e9 b0 00 00 00              	jmp	0x14089d0a1 <.text+0x89c0a1>
14089cff1: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14089cffb: 48 89 06                    	movq	%rax, (%rsi)
14089cffe: 48 8d 05 ca c1 1f 02        	leaq	0x21fc1ca(%rip), %rax   # 0x142a991cf
14089d005: 48 89 46 18                 	movq	%rax, 0x18(%rsi)
14089d009: 48 c7 46 20 0d 00 00 00     	movq	$0xd, 0x20(%rsi)
14089d011: e9 93 00 00 00              	jmp	0x14089d0a9 <.text+0x89c0a9>
14089d016: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14089d020: 48 89 06                    	movq	%rax, (%rsi)
14089d023: 48 8d 05 2b c1 1f 02        	leaq	0x21fc12b(%rip), %rax   # 0x142a99155
14089d02a: 48 89 46 18                 	movq	%rax, 0x18(%rsi)
14089d02e: 48 c7 46 20 10 00 00 00     	movq	$0x10, 0x20(%rsi)
14089d036: eb 71                       	jmp	0x14089d0a9 <.text+0x89c0a9>
14089d038: 48 8d 0d f1 c0 1f 02        	leaq	0x21fc0f1(%rip), %rcx   # 0x142a99130
14089d03f: ba 12 00 00 00              	movl	$0x12, %edx
14089d044: e8 37 04 30 00              	callq	0x140b9d480 <.text+0xb9c480>
14089d049: 48 89 d7                    	movq	%rdx, %rdi
14089d04c: 48 85 d2                    	testq	%rdx, %rdx
14089d04f: 0f 88 39 01 00 00           	js	0x14089d18e <.text+0x89c18e>
14089d055: 48 89 c3                    	movq	%rax, %rbx
14089d058: 74 28                       	je	0x14089d082 <.text+0x89c082>
14089d05a: 0f b6 05 a8 7b d9 02        	movzbl	0x2d97ba8(%rip), %eax   # 0x143634c09
14089d061: 41 bf 01 00 00 00           	movl	$0x1, %r15d
14089d067: ba 01 00 00 00              	movl	$0x1, %edx
14089d06c: 48 89 f9                    	movq	%rdi, %rcx
14089d06f: e8 9c 6b d7 ff              	callq	0x140613c10 <.text+0x612c10>
14089d074: 48 85 c0                    	testq	%rax, %rax
14089d077: 0f 84 14 01 00 00           	je	0x14089d191 <.text+0x89c191>
14089d07d: 49 89 c6                    	movq	%rax, %r14
14089d080: eb 06                       	jmp	0x14089d088 <.text+0x89c088>
14089d082: 41 be 01 00 00 00           	movl	$0x1, %r14d
14089d088: 4c 89 f1                    	movq	%r14, %rcx
14089d08b: 48 89 da                    	movq	%rbx, %rdx
14089d08e: 49 89 f8                    	movq	%rdi, %r8
14089d091: e8 6a 6b 08 02              	callq	0x142923c00 <.text+0x2922c00>
14089d096: 48 89 3e                    	movq	%rdi, (%rsi)
14089d099: 4c 89 76 08                 	movq	%r14, 0x8(%rsi)
14089d09d: 48 89 7e 10                 	movq	%rdi, 0x10(%rsi)
14089d0a1: 48 c7 46 18 00 00 00 00     	movq	$0x0, 0x18(%rsi)
14089d0a9: 48 89 f0                    	movq	%rsi, %rax
14089d0ac: 48 81 c4 90 00 00 00        	addq	$0x90, %rsp
14089d0b3: 5b                          	popq	%rbx
14089d0b4: 5f                          	popq	%rdi
14089d0b5: 5e                          	popq	%rsi
14089d0b6: 41 5e                       	popq	%r14
14089d0b8: 41 5f                       	popq	%r15
14089d0ba: c3                          	retq
14089d0bb: 0f b6 05 47 7b d9 02        	movzbl	0x2d97b47(%rip), %eax   # 0x143634c09
14089d0c2: b9 07 00 00 00              	movl	$0x7, %ecx
14089d0c7: ba 01 00 00 00              	movl	$0x1, %edx
14089d0cc: e8 3f 6b d7 ff              	callq	0x140613c10 <.text+0x612c10>
14089d0d1: 48 85 c0                    	testq	%rax, %rax
14089d0d4: 0f 84 93 01 00 00           	je	0x14089d26d <.text+0x89c26d>
14089d0da: c7 40 03 4c 65 66 74        	movl	$0x7466654c, 0x3(%rax)  # imm = 0x7466654C
14089d0e1: c7 00 55 49 20 4c           	movl	$0x4c204955, (%rax)     # imm = 0x4C204955
14089d0e7: eb 2c                       	jmp	0x14089d115 <.text+0x89c115>
14089d0e9: 0f b6 05 19 7b d9 02        	movzbl	0x2d97b19(%rip), %eax   # 0x143634c09
14089d0f0: b9 07 00 00 00              	movl	$0x7, %ecx
14089d0f5: ba 01 00 00 00              	movl	$0x1, %edx
14089d0fa: e8 11 6b d7 ff              	callq	0x140613c10 <.text+0x612c10>
14089d0ff: 48 85 c0                    	testq	%rax, %rax
14089d102: 0f 84 65 01 00 00           	je	0x14089d26d <.text+0x89c26d>
14089d108: c7 40 03 44 6f 77 6e        	movl	$0x6e776f44, 0x3(%rax)  # imm = 0x6E776F44
14089d10f: c7 00 55 49 20 44           	movl	$0x44204955, (%rax)     # imm = 0x44204955
14089d115: 48 c7 06 07 00 00 00        	movq	$0x7, (%rsi)
14089d11c: 48 89 46 08                 	movq	%rax, 0x8(%rsi)
14089d120: 48 c7 46 10 07 00 00 00     	movq	$0x7, 0x10(%rsi)
14089d128: e9 74 ff ff ff              	jmp	0x14089d0a1 <.text+0x89c0a1>
14089d12d: 48 8d 0d dc c0 1f 02        	leaq	0x21fc0dc(%rip), %rcx   # 0x142a99210
14089d134: ba 10 00 00 00              	movl	$0x10, %edx
14089d139: e8 42 03 30 00              	callq	0x140b9d480 <.text+0xb9c480>
14089d13e: 48 89 d7                    	movq	%rdx, %rdi
14089d141: 48 85 d2                    	testq	%rdx, %rdx
14089d144: 0f 89 0b ff ff ff           	jns	0x14089d055 <.text+0x89c055>
14089d14a: eb 42                       	jmp	0x14089d18e <.text+0x89c18e>
14089d14c: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14089d156: 48 89 06                    	movq	%rax, (%rsi)
14089d159: 48 8d 05 7c c0 1f 02        	leaq	0x21fc07c(%rip), %rax   # 0x142a991dc
14089d160: 48 89 46 18                 	movq	%rax, 0x18(%rsi)
14089d164: 48 c7 46 20 11 00 00 00     	movq	$0x11, 0x20(%rsi)
14089d16c: e9 38 ff ff ff              	jmp	0x14089d0a9 <.text+0x89c0a9>
14089d171: 48 8d 0d 92 be 1f 02        	leaq	0x21fbe92(%rip), %rcx   # 0x142a9900a
14089d178: ba 1b 00 00 00              	movl	$0x1b, %edx
14089d17d: e8 fe 02 30 00              	callq	0x140b9d480 <.text+0xb9c480>
14089d182: 48 89 d7                    	movq	%rdx, %rdi
14089d185: 48 85 d2                    	testq	%rdx, %rdx
14089d188: 0f 89 c7 fe ff ff           	jns	0x14089d055 <.text+0x89c055>
14089d18e: 45 31 ff                    	xorl	%r15d, %r15d
14089d191: 4c 8d 05 08 4c 1f 02        	leaq	0x21f4c08(%rip), %r8    # 0x142a91da0
14089d198: 4c 89 f9                    	movq	%r15, %rcx
14089d19b: 48 89 fa                    	movq	%rdi, %rdx
14089d19e: e8 40 be 03 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14089d1a3: 0f b6 05 5f 7a d9 02        	movzbl	0x2d97a5f(%rip), %eax   # 0x143634c09
14089d1aa: b9 09 00 00 00              	movl	$0x9, %ecx
14089d1af: ba 01 00 00 00              	movl	$0x1, %edx
14089d1b4: e8 57 6a d7 ff              	callq	0x140613c10 <.text+0x612c10>
14089d1b9: 48 85 c0                    	testq	%rax, %rax
14089d1bc: 0f 84 03 01 00 00           	je	0x14089d2c5 <.text+0x89c2c5>
14089d1c2: 48 b9 55 49 20 43 61 6e 63 65       	movabsq	$0x65636e6143204955, %rcx # imm = 0x65636E6143204955
14089d1cc: 48 89 08                    	movq	%rcx, (%rax)
14089d1cf: c6 40 08 6c                 	movb	$0x6c, 0x8(%rax)
14089d1d3: 48 c7 06 09 00 00 00        	movq	$0x9, (%rsi)
14089d1da: 48 89 46 08                 	movq	%rax, 0x8(%rsi)
14089d1de: 48 c7 46 10 09 00 00 00     	movq	$0x9, 0x10(%rsi)
14089d1e6: e9 b6 fe ff ff              	jmp	0x14089d0a1 <.text+0x89c0a1>
14089d1eb: 0f b6 05 17 7a d9 02        	movzbl	0x2d97a17(%rip), %eax   # 0x143634c09
14089d1f2: b9 0a 00 00 00              	movl	$0xa, %ecx
14089d1f7: ba 01 00 00 00              	movl	$0x1, %edx
14089d1fc: e8 0f 6a d7 ff              	callq	0x140613c10 <.text+0x612c10>
14089d201: 48 85 c0                    	testq	%rax, %rax
14089d204: 0f 84 d1 00 00 00           	je	0x14089d2db <.text+0x89c2db>
14089d20a: 48 b9 55 49 20 43 6f 6e 66 69       	movabsq	$0x69666e6f43204955, %rcx # imm = 0x69666E6F43204955
14089d214: 48 89 08                    	movq	%rcx, (%rax)
14089d217: 66 c7 40 08 72 6d           	movw	$0x6d72, 0x8(%rax)      # imm = 0x6D72
14089d21d: 48 c7 06 0a 00 00 00        	movq	$0xa, (%rsi)
14089d224: 48 89 46 08                 	movq	%rax, 0x8(%rsi)
14089d228: 48 c7 46 10 0a 00 00 00     	movq	$0xa, 0x10(%rsi)
14089d230: e9 6c fe ff ff              	jmp	0x14089d0a1 <.text+0x89c0a1>
14089d235: 48 8d 0d 19 c0 1f 02        	leaq	0x21fc019(%rip), %rcx   # 0x142a99255
14089d23c: ba 18 00 00 00              	movl	$0x18, %edx
14089d241: e8 3a 02 30 00              	callq	0x140b9d480 <.text+0xb9c480>
14089d246: 48 89 d7                    	movq	%rdx, %rdi
14089d249: 48 85 d2                    	testq	%rdx, %rdx
14089d24c: 0f 89 03 fe ff ff           	jns	0x14089d055 <.text+0x89c055>
14089d252: e9 37 ff ff ff              	jmp	0x14089d18e <.text+0x89c18e>
14089d257: 4c 8d 05 42 4b 1f 02        	leaq	0x21f4b42(%rip), %r8    # 0x142a91da0
14089d25e: b9 01 00 00 00              	movl	$0x1, %ecx
14089d263: ba 0b 00 00 00              	movl	$0xb, %edx
14089d268: e8 76 bd 03 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14089d26d: 4c 8d 05 2c 4b 1f 02        	leaq	0x21f4b2c(%rip), %r8    # 0x142a91da0
14089d274: b9 01 00 00 00              	movl	$0x1, %ecx
14089d279: ba 07 00 00 00              	movl	$0x7, %edx
14089d27e: e8 60 bd 03 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14089d283: 4c 8d 05 16 4b 1f 02        	leaq	0x21f4b16(%rip), %r8    # 0x142a91da0
14089d28a: b9 01 00 00 00              	movl	$0x1, %ecx
14089d28f: ba 16 00 00 00              	movl	$0x16, %edx
14089d294: e8 4a bd 03 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14089d299: 4c 8d 05 00 4b 1f 02        	leaq	0x21f4b00(%rip), %r8    # 0x142a91da0
14089d2a0: b9 01 00 00 00              	movl	$0x1, %ecx
14089d2a5: ba 05 00 00 00              	movl	$0x5, %edx
14089d2aa: e8 34 bd 03 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14089d2af: 4c 8d 05 ea 4a 1f 02        	leaq	0x21f4aea(%rip), %r8    # 0x142a91da0
14089d2b6: b9 01 00 00 00              	movl	$0x1, %ecx
14089d2bb: ba 08 00 00 00              	movl	$0x8, %edx
14089d2c0: e8 1e bd 03 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14089d2c5: 4c 8d 05 d4 4a 1f 02        	leaq	0x21f4ad4(%rip), %r8    # 0x142a91da0
14089d2cc: b9 01 00 00 00              	movl	$0x1, %ecx
14089d2d1: ba 09 00 00 00              	movl	$0x9, %edx
14089d2d6: e8 08 bd 03 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14089d2db: 4c 8d 05 be 4a 1f 02        	leaq	0x21f4abe(%rip), %r8    # 0x142a91da0
14089d2e2: b9 01 00 00 00              	movl	$0x1, %ecx
14089d2e7: ba 0a 00 00 00              	movl	$0xa, %edx
14089d2ec: e8 f2 bc 03 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14089d2f1: cc                          	int3
14089d2f2: cc                          	int3
14089d2f3: cc                          	int3
14089d2f4: cc                          	int3
14089d2f5: cc                          	int3
14089d2f6: cc                          	int3
14089d2f7: cc                          	int3
14089d2f8: cc                          	int3
14089d2f9: cc                          	int3
14089d2fa: cc                          	int3
14089d2fb: cc                          	int3
14089d2fc: cc                          	int3
14089d2fd: cc                          	int3
14089d2fe: cc                          	int3
14089d2ff: cc                          	int3
14089d300: 48 89 c8                    	movq	%rcx, %rax
14089d303: 0f b6 0a                    	movzbl	(%rdx), %ecx
14089d306: 48 8d 15 1f d8 1f 02        	leaq	0x21fd81f(%rip), %rdx   # 0x142a9ab2c
14089d30d: 48 63 0c 8a                 	movslq	(%rdx,%rcx,4), %rcx
14089d311: 48 01 d1                    	addq	%rdx, %rcx
14089d314: ff e1                       	jmpq	*%rcx
14089d316: 49 83 c0 30                 	addq	$0x30, %r8
14089d31a: e9 57 01 00 00              	jmp	0x14089d476 <.text+0x89c476>
14089d31f: 49 81 c0 a4 01 00 00        	addq	$0x1a4, %r8             # imm = 0x1A4
14089d326: e9 4b 01 00 00              	jmp	0x14089d476 <.text+0x89c476>
14089d32b: 49 81 c0 b0 01 00 00        	addq	$0x1b0, %r8             # imm = 0x1B0
14089d332: e9 3f 01 00 00              	jmp	0x14089d476 <.text+0x89c476>
14089d337: 49 81 c0 44 01 00 00        	addq	$0x144, %r8             # imm = 0x144
14089d33e: e9 33 01 00 00              	jmp	0x14089d476 <.text+0x89c476>
14089d343: 49 81 c0 98 01 00 00        	addq	$0x198, %r8             # imm = 0x198
14089d34a: e9 27 01 00 00              	jmp	0x14089d476 <.text+0x89c476>
14089d34f: 49 81 c0 c0 00 00 00        	addq	$0xc0, %r8
14089d356: e9 1b 01 00 00              	jmp	0x14089d476 <.text+0x89c476>
14089d35b: 49 81 c0 a8 00 00 00        	addq	$0xa8, %r8
14089d362: e9 0f 01 00 00              	jmp	0x14089d476 <.text+0x89c476>
14089d367: 49 81 c0 8c 01 00 00        	addq	$0x18c, %r8             # imm = 0x18C
14089d36e: e9 03 01 00 00              	jmp	0x14089d476 <.text+0x89c476>
14089d373: 49 81 c0 84 00 00 00        	addq	$0x84, %r8
14089d37a: e9 f7 00 00 00              	jmp	0x14089d476 <.text+0x89c476>
14089d37f: 49 83 c0 24                 	addq	$0x24, %r8
14089d383: e9 ee 00 00 00              	jmp	0x14089d476 <.text+0x89c476>
14089d388: 49 81 c0 90 00 00 00        	addq	$0x90, %r8
14089d38f: e9 e2 00 00 00              	jmp	0x14089d476 <.text+0x89c476>
14089d394: 49 83 c0 6c                 	addq	$0x6c, %r8
14089d398: e9 d9 00 00 00              	jmp	0x14089d476 <.text+0x89c476>
14089d39d: 49 81 c0 2c 01 00 00        	addq	$0x12c, %r8             # imm = 0x12C
14089d3a4: e9 cd 00 00 00              	jmp	0x14089d476 <.text+0x89c476>
14089d3a9: 49 81 c0 20 01 00 00        	addq	$0x120, %r8             # imm = 0x120
14089d3b0: e9 c1 00 00 00              	jmp	0x14089d476 <.text+0x89c476>
14089d3b5: 49 83 c0 3c                 	addq	$0x3c, %r8
14089d3b9: e9 b8 00 00 00              	jmp	0x14089d476 <.text+0x89c476>
14089d3be: 49 81 c0 b4 00 00 00        	addq	$0xb4, %r8
14089d3c5: e9 ac 00 00 00              	jmp	0x14089d476 <.text+0x89c476>
14089d3ca: 49 83 c0 18                 	addq	$0x18, %r8
14089d3ce: e9 a3 00 00 00              	jmp	0x14089d476 <.text+0x89c476>
14089d3d3: 49 83 c0 54                 	addq	$0x54, %r8
14089d3d7: e9 9a 00 00 00              	jmp	0x14089d476 <.text+0x89c476>
14089d3dc: 49 81 c0 74 01 00 00        	addq	$0x174, %r8             # imm = 0x174
14089d3e3: e9 8e 00 00 00              	jmp	0x14089d476 <.text+0x89c476>
14089d3e8: 49 81 c0 cc 00 00 00        	addq	$0xcc, %r8
14089d3ef: e9 82 00 00 00              	jmp	0x14089d476 <.text+0x89c476>
14089d3f4: 49 81 c0 f0 00 00 00        	addq	$0xf0, %r8
14089d3fb: eb 79                       	jmp	0x14089d476 <.text+0x89c476>
14089d3fd: 49 83 c0 0c                 	addq	$0xc, %r8
14089d401: eb 73                       	jmp	0x14089d476 <.text+0x89c476>
14089d403: 49 81 c0 fc 00 00 00        	addq	$0xfc, %r8
14089d40a: eb 6a                       	jmp	0x14089d476 <.text+0x89c476>
14089d40c: 49 81 c0 9c 00 00 00        	addq	$0x9c, %r8
14089d413: eb 61                       	jmp	0x14089d476 <.text+0x89c476>
14089d415: 49 83 c0 48                 	addq	$0x48, %r8
14089d419: eb 5b                       	jmp	0x14089d476 <.text+0x89c476>
14089d41b: 49 81 c0 d8 00 00 00        	addq	$0xd8, %r8
14089d422: eb 52                       	jmp	0x14089d476 <.text+0x89c476>
14089d424: 49 83 c0 78                 	addq	$0x78, %r8
14089d428: eb 4c                       	jmp	0x14089d476 <.text+0x89c476>
14089d42a: 49 83 c0 60                 	addq	$0x60, %r8
14089d42e: eb 46                       	jmp	0x14089d476 <.text+0x89c476>
14089d430: 49 81 c0 68 01 00 00        	addq	$0x168, %r8             # imm = 0x168
14089d437: eb 3d                       	jmp	0x14089d476 <.text+0x89c476>
14089d439: 49 81 c0 80 01 00 00        	addq	$0x180, %r8             # imm = 0x180
14089d440: eb 34                       	jmp	0x14089d476 <.text+0x89c476>
14089d442: 49 81 c0 08 01 00 00        	addq	$0x108, %r8             # imm = 0x108
14089d449: eb 2b                       	jmp	0x14089d476 <.text+0x89c476>
14089d44b: 49 81 c0 e4 00 00 00        	addq	$0xe4, %r8
14089d452: eb 22                       	jmp	0x14089d476 <.text+0x89c476>
14089d454: 49 81 c0 14 01 00 00        	addq	$0x114, %r8             # imm = 0x114
14089d45b: eb 19                       	jmp	0x14089d476 <.text+0x89c476>
14089d45d: 49 81 c0 50 01 00 00        	addq	$0x150, %r8             # imm = 0x150
14089d464: eb 10                       	jmp	0x14089d476 <.text+0x89c476>
14089d466: 49 81 c0 5c 01 00 00        	addq	$0x15c, %r8             # imm = 0x15C
14089d46d: eb 07                       	jmp	0x14089d476 <.text+0x89c476>
14089d46f: 49 81 c0 38 01 00 00        	addq	$0x138, %r8             # imm = 0x138
14089d476: 41 8b 48 08                 	movl	0x8(%r8), %ecx
14089d47a: 89 48 08                    	movl	%ecx, 0x8(%rax)
14089d47d: 49 8b 08                    	movq	(%r8), %rcx
14089d480: 48 89 08                    	movq	%rcx, (%rax)
14089d483: c3                          	retq
14089d484: cc                          	int3
14089d485: cc                          	int3
14089d486: cc                          	int3
14089d487: cc                          	int3
14089d488: cc                          	int3
14089d489: cc                          	int3
14089d48a: cc                          	int3
14089d48b: cc                          	int3
14089d48c: cc                          	int3
14089d48d: cc                          	int3
14089d48e: cc                          	int3
14089d48f: cc                          	int3
14089d490: 0f b6 01                    	movzbl	(%rcx), %eax
14089d493: 04 7b                       	addb	$0x7b, %al
14089d495: 3c f3                       	cmpb	$-0xd, %al
14089d497: 77 1d                       	ja	0x14089d4b6 <.text+0x89c4b6>
14089d499: 0f b6 c0                    	movzbl	%al, %eax
14089d49c: c1 e0 03                    	shll	$0x3, %eax
14089d49f: 48 8d 0d 12 c3 1f 02        	leaq	0x21fc312(%rip), %rcx   # 0x142a997b8
14089d4a6: 48 8b 14 08                 	movq	(%rax,%rcx), %rdx
14089d4aa: 48 8d 0d a7 ca 1f 02        	leaq	0x21fcaa7(%rip), %rcx   # 0x142a99f58
14089d4b1: 48 8b 04 08                 	movq	(%rax,%rcx), %rax
14089d4b5: c3                          	retq
14089d4b6: 31 c0                       	xorl	%eax, %eax
14089d4b8: c3                          	retq
14089d4b9: cc                          	int3
14089d4ba: cc                          	int3
14089d4bb: cc                          	int3
14089d4bc: cc                          	int3
14089d4bd: cc                          	int3
14089d4be: cc                          	int3
14089d4bf: cc                          	int3

; SOURCE /tmp/key-mouse.asm

B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14089c880: 0f b6 09                    	movzbl	(%rcx), %ecx
14089c883: 0f b7 84 8a bc 01 00 00     	movzwl	0x1bc(%rdx,%rcx,4), %eax
14089c88b: 0f b7 94 8a be 01 00 00     	movzwl	0x1be(%rdx,%rcx,4), %edx
14089c893: c3                          	retq
14089c894: cc                          	int3
14089c895: cc                          	int3
14089c896: cc                          	int3
14089c897: cc                          	int3
14089c898: cc                          	int3
14089c899: cc                          	int3
14089c89a: cc                          	int3
14089c89b: cc                          	int3
14089c89c: cc                          	int3
14089c89d: cc                          	int3
14089c89e: cc                          	int3
14089c89f: cc                          	int3
14089c8a0: 41 57                       	pushq	%r15
14089c8a2: 41 56                       	pushq	%r14
14089c8a4: 56                          	pushq	%rsi
14089c8a5: 57                          	pushq	%rdi
14089c8a6: 53                          	pushq	%rbx
14089c8a7: 48 83 ec 20                 	subq	$0x20, %rsp
14089c8ab: 48 89 ce                    	movq	%rcx, %rsi
14089c8ae: 0f b6 02                    	movzbl	(%rdx), %eax
14089c8b1: 85 c0                       	testl	%eax, %eax
14089c8b3: 74 13                       	je	0x14089c8c8 <.text+0x89b8c8>
14089c8b5: 83 f8 01                    	cmpl	$0x1, %eax
14089c8b8: 75 1c                       	jne	0x14089c8d6 <.text+0x89b8d6>
14089c8ba: 48 8d 0d 39 c7 1f 02        	leaq	0x21fc739(%rip), %rcx   # 0x142a98ffa
14089c8c1: ba 10 00 00 00              	movl	$0x10, %edx
14089c8c6: eb 1a                       	jmp	0x14089c8e2 <.text+0x89b8e2>
14089c8c8: 48 8d 0d 19 c7 1f 02        	leaq	0x21fc719(%rip), %rcx   # 0x142a98fe8
14089c8cf: ba 12 00 00 00              	movl	$0x12, %edx
14089c8d4: eb 0c                       	jmp	0x14089c8e2 <.text+0x89b8e2>
14089c8d6: 48 8d 0d 2d c7 1f 02        	leaq	0x21fc72d(%rip), %rcx   # 0x142a9900a
14089c8dd: ba 1b 00 00 00              	movl	$0x1b, %edx
14089c8e2: e8 99 0b 30 00              	callq	0x140b9d480 <.text+0xb9c480>
14089c8e7: 48 89 c3                    	movq	%rax, %rbx
14089c8ea: 48 89 d7                    	movq	%rdx, %rdi
14089c8ed: 48 85 d2                    	testq	%rdx, %rdx
14089c8f0: 78 5c                       	js	0x14089c94e <.text+0x89b94e>
14089c8f2: 74 24                       	je	0x14089c918 <.text+0x89b918>
14089c8f4: 0f b6 05 0e 83 d9 02        	movzbl	0x2d9830e(%rip), %eax   # 0x143634c09
14089c8fb: 41 bf 01 00 00 00           	movl	$0x1, %r15d
14089c901: ba 01 00 00 00              	movl	$0x1, %edx
14089c906: 48 89 f9                    	movq	%rdi, %rcx
14089c909: e8 02 73 d7 ff              	callq	0x140613c10 <.text+0x612c10>
14089c90e: 48 85 c0                    	testq	%rax, %rax
14089c911: 74 3e                       	je	0x14089c951 <.text+0x89b951>
14089c913: 49 89 c6                    	movq	%rax, %r14
14089c916: eb 06                       	jmp	0x14089c91e <.text+0x89b91e>
14089c918: 41 be 01 00 00 00           	movl	$0x1, %r14d
14089c91e: 4c 89 f1                    	movq	%r14, %rcx
14089c921: 48 89 da                    	movq	%rbx, %rdx
14089c924: 49 89 f8                    	movq	%rdi, %r8
14089c927: e8 d4 72 08 02              	callq	0x142923c00 <.text+0x2922c00>
14089c92c: 48 89 3e                    	movq	%rdi, (%rsi)
14089c92f: 4c 89 76 08                 	movq	%r14, 0x8(%rsi)
14089c933: 48 89 7e 10                 	movq	%rdi, 0x10(%rsi)
14089c937: 48 c7 46 18 00 00 00 00     	movq	$0x0, 0x18(%rsi)
14089c93f: 48 89 f0                    	movq	%rsi, %rax
14089c942: 48 83 c4 20                 	addq	$0x20, %rsp
14089c946: 5b                          	popq	%rbx
14089c947: 5f                          	popq	%rdi
14089c948: 5e                          	popq	%rsi
14089c949: 41 5e                       	popq	%r14
14089c94b: 41 5f                       	popq	%r15
14089c94d: c3                          	retq
14089c94e: 45 31 ff                    	xorl	%r15d, %r15d
14089c951: 4c 8d 05 48 54 1f 02        	leaq	0x21f5448(%rip), %r8    # 0x142a91da0
14089c958: 4c 89 f9                    	movq	%r15, %rcx
14089c95b: 48 89 fa                    	movq	%rdi, %rdx
14089c95e: e8 80 c6 03 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14089c963: cc                          	int3
14089c964: cc                          	int3
14089c965: cc                          	int3
14089c966: cc                          	int3
14089c967: cc                          	int3
14089c968: cc                          	int3
14089c969: cc                          	int3
14089c96a: cc                          	int3
14089c96b: cc                          	int3
14089c96c: cc                          	int3
14089c96d: cc                          	int3
14089c96e: cc                          	int3
14089c96f: cc                          	int3
14089c970: 56                          	pushq	%rsi
14089c971: 48 83 ec 70                 	subq	$0x70, %rsp
14089c975: 48 89 ce                    	movq	%rcx, %rsi
14089c978: 0f b7 c2                    	movzwl	%dx, %eax
14089c97b: 48 8d 0d fe e0 1f 02        	leaq	0x21fe0fe(%rip), %rcx   # 0x142a9aa80
14089c982: 48 63 04 81                 	movslq	(%rcx,%rax,4), %rax
14089c986: 48 01 c8                    	addq	%rcx, %rax
14089c989: ff e0                       	jmpq	*%rax
14089c98b: 0f b6 05 77 82 d9 02        	movzbl	0x2d98277(%rip), %eax   # 0x143634c09
14089c992: b9 03 00 00 00              	movl	$0x3, %ecx
14089c997: ba 01 00 00 00              	movl	$0x1, %edx
14089c99c: e8 6f 72 d7 ff              	callq	0x140613c10 <.text+0x612c10>
14089c9a1: 48 85 c0                    	testq	%rax, %rax
14089c9a4: 0f 84 65 01 00 00           	je	0x14089cb0f <.text+0x89bb0f>
14089c9aa: c6 40 02 42                 	movb	$0x42, 0x2(%rax)
14089c9ae: 66 c7 00 4c 4d              	movw	$0x4d4c, (%rax)         # imm = 0x4D4C
14089c9b3: e9 df 00 00 00              	jmp	0x14089ca97 <.text+0x89ba97>
14089c9b8: 0f b6 05 4a 82 d9 02        	movzbl	0x2d9824a(%rip), %eax   # 0x143634c09
14089c9bf: b9 14 00 00 00              	movl	$0x14, %ecx
14089c9c4: ba 01 00 00 00              	movl	$0x1, %edx
14089c9c9: e8 42 72 d7 ff              	callq	0x140613c10 <.text+0x612c10>
14089c9ce: 48 85 c0                    	testq	%rax, %rax
14089c9d1: 0f 84 4e 01 00 00           	je	0x14089cb25 <.text+0x89bb25>
14089c9d7: 0f 10 05 58 c6 1f 02        	movups	0x21fc658(%rip), %xmm0  # 0x142a99036
14089c9de: 0f 11 00                    	movups	%xmm0, (%rax)
14089c9e1: c7 40 10 74 74 6f 6e        	movl	$0x6e6f7474, 0x10(%rax) # imm = 0x6E6F7474
14089c9e8: 48 c7 06 14 00 00 00        	movq	$0x14, (%rsi)
14089c9ef: 48 89 46 08                 	movq	%rax, 0x8(%rsi)
14089c9f3: 48 c7 46 10 14 00 00 00     	movq	$0x14, 0x10(%rsi)
14089c9fb: e9 06 01 00 00              	jmp	0x14089cb06 <.text+0x89bb06>
14089ca00: 0f b6 05 02 82 d9 02        	movzbl	0x2d98202(%rip), %eax   # 0x143634c09
14089ca07: b9 03 00 00 00              	movl	$0x3, %ecx
14089ca0c: ba 01 00 00 00              	movl	$0x1, %edx
14089ca11: e8 fa 71 d7 ff              	callq	0x140613c10 <.text+0x612c10>
14089ca16: 48 85 c0                    	testq	%rax, %rax
14089ca19: 0f 84 f0 00 00 00           	je	0x14089cb0f <.text+0x89bb0f>
14089ca1f: c6 40 02 42                 	movb	$0x42, 0x2(%rax)
14089ca23: 66 c7 00 4d 4d              	movw	$0x4d4d, (%rax)         # imm = 0x4D4D
14089ca28: eb 6d                       	jmp	0x14089ca97 <.text+0x89ba97>
14089ca2a: 0f b6 05 d8 81 d9 02        	movzbl	0x2d981d8(%rip), %eax   # 0x143634c09
14089ca31: b9 11 00 00 00              	movl	$0x11, %ecx
14089ca36: ba 01 00 00 00              	movl	$0x1, %edx
14089ca3b: e8 d0 71 d7 ff              	callq	0x140613c10 <.text+0x612c10>
14089ca40: 48 85 c0                    	testq	%rax, %rax
14089ca43: 0f 84 f2 00 00 00           	je	0x14089cb3b <.text+0x89bb3b>
14089ca49: 0f 10 05 d5 c5 1f 02        	movups	0x21fc5d5(%rip), %xmm0  # 0x142a99025
14089ca50: 0f 11 00                    	movups	%xmm0, (%rax)
14089ca53: c6 40 10 6e                 	movb	$0x6e, 0x10(%rax)
14089ca57: 48 c7 06 11 00 00 00        	movq	$0x11, (%rsi)
14089ca5e: 48 89 46 08                 	movq	%rax, 0x8(%rsi)
14089ca62: 48 c7 46 10 11 00 00 00     	movq	$0x11, 0x10(%rsi)
14089ca6a: e9 97 00 00 00              	jmp	0x14089cb06 <.text+0x89bb06>
14089ca6f: 0f b6 05 93 81 d9 02        	movzbl	0x2d98193(%rip), %eax   # 0x143634c09
14089ca76: b9 03 00 00 00              	movl	$0x3, %ecx
14089ca7b: ba 01 00 00 00              	movl	$0x1, %edx
14089ca80: e8 8b 71 d7 ff              	callq	0x140613c10 <.text+0x612c10>
14089ca85: 48 85 c0                    	testq	%rax, %rax
14089ca88: 0f 84 81 00 00 00           	je	0x14089cb0f <.text+0x89bb0f>
14089ca8e: c6 40 02 42                 	movb	$0x42, 0x2(%rax)
14089ca92: 66 c7 00 52 4d              	movw	$0x4d52, (%rax)         # imm = 0x4D52
14089ca97: 48 c7 06 03 00 00 00        	movq	$0x3, (%rsi)
14089ca9e: 48 89 46 08                 	movq	%rax, 0x8(%rsi)
14089caa2: 48 c7 46 10 03 00 00 00     	movq	$0x3, 0x10(%rsi)
14089caaa: eb 5a                       	jmp	0x14089cb06 <.text+0x89bb06>
14089caac: 66 44 89 44 24 2e           	movw	%r8w, 0x2e(%rsp)
14089cab2: 48 8d 44 24 2e              	leaq	0x2e(%rsp), %rax
14089cab7: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
14089cabc: 48 8d 05 4d 08 4f 00        	leaq	0x4f084d(%rip), %rax    # 0x140d8d310 <.text+0xd8c310>
14089cac3: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
14089cac8: 48 8d 05 89 c5 1f 02        	leaq	0x21fc589(%rip), %rax   # 0x142a99058
14089cacf: 48 89 44 24 40              	movq	%rax, 0x40(%rsp)
14089cad4: 48 c7 44 24 48 01 00 00 00  	movq	$0x1, 0x48(%rsp)
14089cadd: 48 8d 44 24 30              	leaq	0x30(%rsp), %rax
14089cae2: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
14089cae7: 48 c7 44 24 58 01 00 00 00  	movq	$0x1, 0x58(%rsp)
14089caf0: 48 c7 44 24 60 00 00 00 00  	movq	$0x0, 0x60(%rsp)
14089caf9: 48 8d 54 24 40              	leaq	0x40(%rsp), %rdx
14089cafe: 48 89 f1                    	movq	%rsi, %rcx
14089cb01: e8 8a d4 4d 00              	callq	0x140d79f90 <.text+0xd78f90>
14089cb06: 48 89 f0                    	movq	%rsi, %rax
14089cb09: 48 83 c4 70                 	addq	$0x70, %rsp
14089cb0d: 5e                          	popq	%rsi
14089cb0e: c3                          	retq
14089cb0f: 4c 8d 05 0a c4 1f 02        	leaq	0x21fc40a(%rip), %r8    # 0x142a98f20
14089cb16: b9 01 00 00 00              	movl	$0x1, %ecx
14089cb1b: ba 03 00 00 00              	movl	$0x3, %edx
14089cb20: e8 be c4 03 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14089cb25: 4c 8d 05 f4 c3 1f 02        	leaq	0x21fc3f4(%rip), %r8    # 0x142a98f20
14089cb2c: b9 01 00 00 00              	movl	$0x1, %ecx
14089cb31: ba 14 00 00 00              	movl	$0x14, %edx
14089cb36: e8 a8 c4 03 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14089cb3b: 4c 8d 05 de c3 1f 02        	leaq	0x21fc3de(%rip), %r8    # 0x142a98f20
14089cb42: b9 01 00 00 00              	movl	$0x1, %ecx
14089cb47: ba 11 00 00 00              	movl	$0x11, %edx
14089cb4c: e8 92 c4 03 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14089cb51: cc                          	int3
14089cb52: cc                          	int3
14089cb53: cc                          	int3
14089cb54: cc                          	int3
14089cb55: cc                          	int3
14089cb56: cc                          	int3
14089cb57: cc                          	int3
14089cb58: cc                          	int3
14089cb59: cc                          	int3
14089cb5a: cc                          	int3
14089cb5b: cc                          	int3
14089cb5c: cc                          	int3
14089cb5d: cc                          	int3
14089cb5e: cc                          	int3
14089cb5f: cc                          	int3

; SOURCE /tmp/key-category.asm

B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140085370: 56                          	pushq	%rsi
140085371: 57                          	pushq	%rdi
140085372: 48 81 ec a8 00 00 00        	subq	$0xa8, %rsp
140085379: 48 89 d6                    	movq	%rdx, %rsi
14008537c: 48 8b 39                    	movq	(%rcx), %rdi
14008537f: 48 8d 05 52 c1 8a 02        	leaq	0x28ac152(%rip), %rax   # 0x1429314d8
140085386: 48 63 0c b8                 	movslq	(%rax,%rdi,4), %rcx
14008538a: 48 01 c1                    	addq	%rax, %rcx
14008538d: ff e1                       	jmpq	*%rcx
14008538f: 48 8d 0d 6a bb 8a 02        	leaq	0x28abb6a(%rip), %rcx   # 0x142930f00
140085396: ba 1e 00 00 00              	movl	$0x1e, %edx
14008539b: eb 44                       	jmp	0x1400853e1 <.text+0x843e1>
14008539d: 48 8d 0d 7a bb 8a 02        	leaq	0x28abb7a(%rip), %rcx   # 0x142930f1e
1400853a4: ba 10 00 00 00              	movl	$0x10, %edx
1400853a9: eb 36                       	jmp	0x1400853e1 <.text+0x843e1>
1400853ab: 48 8d 0d 31 bb 8a 02        	leaq	0x28abb31(%rip), %rcx   # 0x142930ee3
1400853b2: ba 1d 00 00 00              	movl	$0x1d, %edx
1400853b7: eb 28                       	jmp	0x1400853e1 <.text+0x843e1>
1400853b9: 48 8d 0d 8a bb 8a 02        	leaq	0x28abb8a(%rip), %rcx   # 0x142930f4a
1400853c0: eb 1a                       	jmp	0x1400853dc <.text+0x843dc>
1400853c2: 48 8d 05 9d bb 8a 02        	leaq	0x28abb9d(%rip), %rax   # 0x142930f66
1400853c9: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
1400853ce: ba 0c 00 00 00              	movl	$0xc, %edx
1400853d3: eb 16                       	jmp	0x1400853eb <.text+0x843eb>
1400853d5: 48 8d 0d 52 bb 8a 02        	leaq	0x28abb52(%rip), %rcx   # 0x142930f2e
1400853dc: ba 1c 00 00 00              	movl	$0x1c, %edx
1400853e1: e8 9a 80 b1 00              	callq	0x140b9d480 <.text+0xb9c480>
1400853e6: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
1400853eb: 48 89 54 24 30              	movq	%rdx, 0x30(%rsp)
1400853f0: 48 ff c7                    	incq	%rdi
1400853f3: 48 89 7c 24 38              	movq	%rdi, 0x38(%rsp)
1400853f8: 48 c7 44 24 40 07 00 00 00  	movq	$0x7, 0x40(%rsp)
140085401: 48 8d 44 24 28              	leaq	0x28(%rsp), %rax
140085406: 48 89 44 24 48              	movq	%rax, 0x48(%rsp)
14008540b: 48 8d 05 7e 4a 08 00        	leaq	0x84a7e(%rip), %rax     # 0x140109e90 <.text+0x108e90>
140085412: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
140085417: 48 8d 44 24 38              	leaq	0x38(%rsp), %rax
14008541c: 48 89 44 24 58              	movq	%rax, 0x58(%rsp)
140085421: 48 8d 05 98 82 d0 00        	leaq	0xd08298(%rip), %rax    # 0x140d8d6c0 <.text+0xd8c6c0>
140085428: 48 89 44 24 60              	movq	%rax, 0x60(%rsp)
14008542d: 48 8d 4c 24 40              	leaq	0x40(%rsp), %rcx
140085432: 48 89 4c 24 68              	movq	%rcx, 0x68(%rsp)
140085437: 48 89 44 24 70              	movq	%rax, 0x70(%rsp)
14008543c: 48 8b 4e 30                 	movq	0x30(%rsi), %rcx
140085440: 48 8b 56 38                 	movq	0x38(%rsi), %rdx
140085444: 48 8d 05 2d bb 8a 02        	leaq	0x28abb2d(%rip), %rax   # 0x142930f78
14008544b: 48 89 44 24 78              	movq	%rax, 0x78(%rsp)
140085450: 48 c7 84 24 80 00 00 00 04 00 00 00 	movq	$0x4, 0x80(%rsp)
14008545c: 48 8d 44 24 48              	leaq	0x48(%rsp), %rax
140085461: 48 89 84 24 88 00 00 00     	movq	%rax, 0x88(%rsp)
140085469: 48 c7 84 24 90 00 00 00 03 00 00 00 	movq	$0x3, 0x90(%rsp)
140085475: 48 c7 84 24 98 00 00 00 00 00 00 00 	movq	$0x0, 0x98(%rsp)
140085481: 4c 8d 44 24 78              	leaq	0x78(%rsp), %r8
140085486: e8 45 06 d0 00              	callq	0x140d85ad0 <.text+0xd84ad0>
14008548b: 90                          	nop
14008548c: 48 81 c4 a8 00 00 00        	addq	$0xa8, %rsp
140085493: 5f                          	popq	%rdi
140085494: 5e                          	popq	%rsi
140085495: c3                          	retq
140085496: cc                          	int3
140085497: cc                          	int3
140085498: cc                          	int3
140085499: cc                          	int3
14008549a: cc                          	int3
14008549b: cc                          	int3
14008549c: cc                          	int3
14008549d: cc                          	int3
14008549e: cc                          	int3
14008549f: cc                          	int3

; SOURCE /tmp/key-submenu.asm

B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1400854a0: 55                          	pushq	%rbp
1400854a1: 41 57                       	pushq	%r15
1400854a3: 41 56                       	pushq	%r14
1400854a5: 41 55                       	pushq	%r13
1400854a7: 41 54                       	pushq	%r12
1400854a9: 56                          	pushq	%rsi
1400854aa: 57                          	pushq	%rdi
1400854ab: 53                          	pushq	%rbx
1400854ac: 48 81 ec a8 02 00 00        	subq	$0x2a8, %rsp            # imm = 0x2A8
1400854b3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1400854bb: 48 c7 85 20 02 00 00 fe ff ff ff    	movq	$-0x2, 0x220(%rbp)
1400854c6: 4d 89 c6                    	movq	%r8, %r14
1400854c9: 48 89 95 60 01 00 00        	movq	%rdx, 0x160(%rbp)
1400854d0: 48 89 8d d8 01 00 00        	movq	%rcx, 0x1d8(%rbp)
1400854d7: 48 8d 0d da ba 8a 02        	leaq	0x28abada(%rip), %rcx   # 0x142930fb8
1400854de: ba 10 00 00 00              	movl	$0x10, %edx
1400854e3: e8 98 7f b1 00              	callq	0x140b9d480 <.text+0xb9c480>
1400854e8: 48 89 45 08                 	movq	%rax, 0x8(%rbp)
1400854ec: 48 89 55 10                 	movq	%rdx, 0x10(%rbp)
1400854f0: 48 8d 75 08                 	leaq	0x8(%rbp), %rsi
1400854f4: 48 89 b5 a0 00 00 00        	movq	%rsi, 0xa0(%rbp)
1400854fb: 48 8d 3d 8e 49 08 00        	leaq	0x8498e(%rip), %rdi     # 0x140109e90 <.text+0x108e90>
140085502: 48 89 bd a8 00 00 00        	movq	%rdi, 0xa8(%rbp)
140085509: 48 8d 05 e0 ba 8a 02        	leaq	0x28abae0(%rip), %rax   # 0x142930ff0
140085510: 48 89 85 18 01 00 00        	movq	%rax, 0x118(%rbp)
140085517: 48 c7 85 20 01 00 00 02 00 00 00    	movq	$0x2, 0x120(%rbp)
140085522: 48 8d 9d a0 00 00 00        	leaq	0xa0(%rbp), %rbx
140085529: 48 89 9d 28 01 00 00        	movq	%rbx, 0x128(%rbp)
140085530: 48 c7 85 30 01 00 00 01 00 00 00    	movq	$0x1, 0x130(%rbp)
14008553b: 48 c7 85 38 01 00 00 00 00 00 00    	movq	$0x0, 0x138(%rbp)
140085546: 48 8d 8d 88 00 00 00        	leaq	0x88(%rbp), %rcx
14008554d: 48 8d 95 18 01 00 00        	leaq	0x118(%rbp), %rdx
140085554: e8 37 4a cf 00              	callq	0x140d79f90 <.text+0xd78f90>
140085559: 48 8d 0d b0 ba 8a 02        	leaq	0x28abab0(%rip), %rcx   # 0x142931010
140085560: ba 12 00 00 00              	movl	$0x12, %edx
140085565: e8 16 7f b1 00              	callq	0x140b9d480 <.text+0xb9c480>
14008556a: 48 89 45 08                 	movq	%rax, 0x8(%rbp)
14008556e: 48 89 55 10                 	movq	%rdx, 0x10(%rbp)
140085572: 48 89 b5 a0 00 00 00        	movq	%rsi, 0xa0(%rbp)
140085579: 48 89 bd a8 00 00 00        	movq	%rdi, 0xa8(%rbp)
140085580: 48 8d 05 b1 ba 8a 02        	leaq	0x28abab1(%rip), %rax   # 0x142931038
140085587: 48 89 85 18 01 00 00        	movq	%rax, 0x118(%rbp)
14008558e: 48 c7 85 20 01 00 00 02 00 00 00    	movq	$0x2, 0x120(%rbp)
140085599: 48 89 9d 28 01 00 00        	movq	%rbx, 0x128(%rbp)
1400855a0: 48 c7 85 30 01 00 00 01 00 00 00    	movq	$0x1, 0x130(%rbp)
1400855ab: 48 c7 85 38 01 00 00 00 00 00 00    	movq	$0x0, 0x138(%rbp)
1400855b6: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
1400855ba: 48 8d 95 18 01 00 00        	leaq	0x118(%rbp), %rdx
1400855c1: e8 ca 49 cf 00              	callq	0x140d79f90 <.text+0xd78f90>
1400855c6: 48 8b 75 d0                 	movq	-0x30(%rbp), %rsi
1400855ca: 48 8b 45 d8                 	movq	-0x28(%rbp), %rax
1400855ce: 48 89 85 68 01 00 00        	movq	%rax, 0x168(%rbp)
1400855d5: 48 8b 7d e0                 	movq	-0x20(%rbp), %rdi
1400855d9: 0f b6 05 29 f6 5a 03        	movzbl	0x35af629(%rip), %eax   # 0x143634c09
1400855e0: b9 00 01 00 00              	movl	$0x100, %ecx            # imm = 0x100
1400855e5: ba 08 00 00 00              	movl	$0x8, %edx
1400855ea: e8 21 e6 58 00              	callq	0x140613c10 <.text+0x612c10>
1400855ef: 48 85 c0                    	testq	%rax, %rax
1400855f2: 48 89 b5 c8 00 00 00        	movq	%rsi, 0xc8(%rbp)
1400855f9: 0f 84 f4 19 00 00           	je	0x140086ff3 <.text+0x85ff3>
1400855ff: 48 89 85 00 02 00 00        	movq	%rax, 0x200(%rbp)
140085606: 4c 8b a5 90 00 00 00        	movq	0x90(%rbp), %r12
14008560d: 4c 8b ad 98 00 00 00        	movq	0x98(%rbp), %r13
140085614: 0f b6 05 ee f5 5a 03        	movzbl	0x35af5ee(%rip), %eax   # 0x143634c09
14008561b: b9 04 00 00 00              	movl	$0x4, %ecx
140085620: ba 01 00 00 00              	movl	$0x1, %edx
140085625: e8 e6 e5 58 00              	callq	0x140613c10 <.text+0x612c10>
14008562a: 48 85 c0                    	testq	%rax, %rax
14008562d: 0f 84 d4 19 00 00           	je	0x140087007 <.text+0x86007>
140085633: 48 89 85 f8 01 00 00        	movq	%rax, 0x1f8(%rbp)
14008563a: c7 00 04 02 05 03           	movl	$0x3050204, (%rax)      # imm = 0x3050204
140085640: 48 8d 0d 18 ba 8a 02        	leaq	0x28aba18(%rip), %rcx   # 0x14293105f
140085647: ba 16 00 00 00              	movl	$0x16, %edx
14008564c: e8 2f 7e b1 00              	callq	0x140b9d480 <.text+0xb9c480>
140085651: 48 89 c6                    	movq	%rax, %rsi
140085654: 48 89 d3                    	movq	%rdx, %rbx
140085657: 0f b6 05 ab f5 5a 03        	movzbl	0x35af5ab(%rip), %eax   # 0x143634c09
14008565e: b9 02 00 00 00              	movl	$0x2, %ecx
140085663: ba 01 00 00 00              	movl	$0x1, %edx
140085668: e8 a3 e5 58 00              	callq	0x140613c10 <.text+0x612c10>
14008566d: 48 85 c0                    	testq	%rax, %rax
140085670: 0f 84 a5 19 00 00           	je	0x14008701b <.text+0x8601b>
140085676: 4c 89 b5 c0 00 00 00        	movq	%r14, 0xc0(%rbp)
14008567d: 48 89 85 f0 01 00 00        	movq	%rax, 0x1f0(%rbp)
140085684: 66 c7 00 01 00              	movw	$0x1, (%rax)
140085689: 0f b6 05 79 f5 5a 03        	movzbl	0x35af579(%rip), %eax   # 0x143634c09
140085690: b9 02 00 00 00              	movl	$0x2, %ecx
140085695: ba 01 00 00 00              	movl	$0x1, %edx
14008569a: e8 71 e5 58 00              	callq	0x140613c10 <.text+0x612c10>
14008569f: 48 85 c0                    	testq	%rax, %rax
1400856a2: 0f 84 87 19 00 00           	je	0x14008702f <.text+0x8602f>
1400856a8: 48 89 85 a0 01 00 00        	movq	%rax, 0x1a0(%rbp)
1400856af: 66 c7 00 06 07              	movw	$0x706, (%rax)          # imm = 0x706
1400856b4: 48 8d 0d ba b9 8a 02        	leaq	0x28ab9ba(%rip), %rcx   # 0x142931075
1400856bb: ba 15 00 00 00              	movl	$0x15, %edx
1400856c0: e8 bb 7d b1 00              	callq	0x140b9d480 <.text+0xb9c480>
1400856c5: 49 89 c6                    	movq	%rax, %r14
1400856c8: 49 89 d7                    	movq	%rdx, %r15
1400856cb: 0f b6 05 37 f5 5a 03        	movzbl	0x35af537(%rip), %eax   # 0x143634c09
1400856d2: b9 02 00 00 00              	movl	$0x2, %ecx
1400856d7: ba 01 00 00 00              	movl	$0x1, %edx
1400856dc: e8 2f e5 58 00              	callq	0x140613c10 <.text+0x612c10>
1400856e1: 48 85 c0                    	testq	%rax, %rax
1400856e4: 0f 84 59 19 00 00           	je	0x140087043 <.text+0x86043>
1400856ea: 66 c7 00 08 09              	movw	$0x908, (%rax)          # imm = 0x908
1400856ef: 48 8b 95 00 02 00 00        	movq	0x200(%rbp), %rdx
1400856f6: 48 c7 82 c0 00 00 00 02 00 00 00    	movq	$0x2, 0xc0(%rdx)
140085701: 48 89 82 c8 00 00 00        	movq	%rax, 0xc8(%rdx)
140085708: 48 c7 82 d0 00 00 00 02 00 00 00    	movq	$0x2, 0xd0(%rdx)
140085713: 48 c7 02 04 00 00 00        	movq	$0x4, (%rdx)
14008571a: 48 8b 85 f8 01 00 00        	movq	0x1f8(%rbp), %rax
140085721: 48 89 42 08                 	movq	%rax, 0x8(%rdx)
140085725: 48 c7 42 10 04 00 00 00     	movq	$0x4, 0x10(%rdx)
14008572d: 4c 89 62 18                 	movq	%r12, 0x18(%rdx)
140085731: 4c 89 6a 20                 	movq	%r13, 0x20(%rdx)
140085735: 48 8d 05 1c b9 8a 02        	leaq	0x28ab91c(%rip), %rax   # 0x142931058
14008573c: 48 89 42 28                 	movq	%rax, 0x28(%rdx)
140085740: 48 c7 42 30 07 00 00 00     	movq	$0x7, 0x30(%rdx)
140085748: c6 42 38 01                 	movb	$0x1, 0x38(%rdx)
14008574c: 48 c7 42 40 02 00 00 00     	movq	$0x2, 0x40(%rdx)
140085754: 48 8b 8d f0 01 00 00        	movq	0x1f0(%rbp), %rcx
14008575b: 48 89 4a 48                 	movq	%rcx, 0x48(%rdx)
14008575f: 48 c7 42 50 02 00 00 00     	movq	$0x2, 0x50(%rdx)
140085767: 48 89 72 58                 	movq	%rsi, 0x58(%rdx)
14008576b: 48 89 5a 60                 	movq	%rbx, 0x60(%rdx)
14008576f: c6 42 78 04                 	movb	$0x4, 0x78(%rdx)
140085773: 48 c7 82 80 00 00 00 02 00 00 00    	movq	$0x2, 0x80(%rdx)
14008577e: 48 8b 8d a0 01 00 00        	movq	0x1a0(%rbp), %rcx
140085785: 48 89 8a 88 00 00 00        	movq	%rcx, 0x88(%rdx)
14008578c: 48 c7 82 90 00 00 00 02 00 00 00    	movq	$0x2, 0x90(%rdx)
140085797: 48 8b 8d 68 01 00 00        	movq	0x168(%rbp), %rcx
14008579e: 48 89 8a 98 00 00 00        	movq	%rcx, 0x98(%rdx)
1400857a5: 48 89 ba a0 00 00 00        	movq	%rdi, 0xa0(%rdx)
1400857ac: 48 89 82 a8 00 00 00        	movq	%rax, 0xa8(%rdx)
1400857b3: 48 c7 82 b0 00 00 00 07 00 00 00    	movq	$0x7, 0xb0(%rdx)
1400857be: c6 82 b8 00 00 00 00        	movb	$0x0, 0xb8(%rdx)
1400857c5: 4c 89 b2 d8 00 00 00        	movq	%r14, 0xd8(%rdx)
1400857cc: 4c 89 ba e0 00 00 00        	movq	%r15, 0xe0(%rdx)
1400857d3: 48 8d 05 b0 b8 8a 02        	leaq	0x28ab8b0(%rip), %rax   # 0x14293108a
1400857da: 48 89 82 e8 00 00 00        	movq	%rax, 0xe8(%rdx)
1400857e1: 48 c7 82 f0 00 00 00 0d 00 00 00    	movq	$0xd, 0xf0(%rdx)
1400857ec: c6 82 f8 00 00 00 03        	movb	$0x3, 0xf8(%rdx)
1400857f3: 48 c7 85 00 01 00 00 04 00 00 00    	movq	$0x4, 0x100(%rbp)
1400857fe: 48 89 95 08 01 00 00        	movq	%rdx, 0x108(%rbp)
140085805: 48 c7 85 10 01 00 00 04 00 00 00    	movq	$0x4, 0x110(%rbp)
140085810: 0f b6 05 f2 f3 5a 03        	movzbl	0x35af3f2(%rip), %eax   # 0x143634c09
140085817: b9 40 01 00 00              	movl	$0x140, %ecx            # imm = 0x140
14008581c: ba 08 00 00 00              	movl	$0x8, %edx
140085821: e8 ea e3 58 00              	callq	0x140613c10 <.text+0x612c10>
140085826: 48 85 c0                    	testq	%rax, %rax
140085829: 0f 84 28 18 00 00           	je	0x140087057 <.text+0x86057>
14008582f: 48 89 c6                    	movq	%rax, %rsi
140085832: 0f b6 05 d0 f3 5a 03        	movzbl	0x35af3d0(%rip), %eax   # 0x143634c09
140085839: b9 01 00 00 00              	movl	$0x1, %ecx
14008583e: ba 01 00 00 00              	movl	$0x1, %edx
140085843: e8 c8 e3 58 00              	callq	0x140613c10 <.text+0x612c10>
140085848: 48 85 c0                    	testq	%rax, %rax
14008584b: 0f 84 21 18 00 00           	je	0x140087072 <.text+0x86072>
140085851: 48 89 c7                    	movq	%rax, %rdi
140085854: c6 00 0a                    	movb	$0xa, (%rax)
140085857: 0f b6 05 ab f3 5a 03        	movzbl	0x35af3ab(%rip), %eax   # 0x143634c09
14008585e: b9 01 00 00 00              	movl	$0x1, %ecx
140085863: ba 01 00 00 00              	movl	$0x1, %edx
140085868: e8 a3 e3 58 00              	callq	0x140613c10 <.text+0x612c10>
14008586d: 48 85 c0                    	testq	%rax, %rax
140085870: 0f 84 17 18 00 00           	je	0x14008708d <.text+0x8608d>
140085876: 48 89 c3                    	movq	%rax, %rbx
140085879: c6 00 0b                    	movb	$0xb, (%rax)
14008587c: 0f b6 05 86 f3 5a 03        	movzbl	0x35af386(%rip), %eax   # 0x143634c09
140085883: b9 01 00 00 00              	movl	$0x1, %ecx
140085888: ba 01 00 00 00              	movl	$0x1, %edx
14008588d: e8 7e e3 58 00              	callq	0x140613c10 <.text+0x612c10>
140085892: 48 85 c0                    	testq	%rax, %rax
140085895: 0f 84 14 18 00 00           	je	0x1400870af <.text+0x860af>
14008589b: 49 89 c6                    	movq	%rax, %r14
14008589e: c6 00 0c                    	movb	$0xc, (%rax)
1400858a1: 0f b6 05 61 f3 5a 03        	movzbl	0x35af361(%rip), %eax   # 0x143634c09
1400858a8: b9 01 00 00 00              	movl	$0x1, %ecx
1400858ad: ba 01 00 00 00              	movl	$0x1, %edx
1400858b2: e8 59 e3 58 00              	callq	0x140613c10 <.text+0x612c10>
1400858b7: 48 85 c0                    	testq	%rax, %rax
1400858ba: 0f 84 18 18 00 00           	je	0x1400870d8 <.text+0x860d8>
1400858c0: 49 89 c7                    	movq	%rax, %r15
1400858c3: c6 00 0d                    	movb	$0xd, (%rax)
1400858c6: 0f b6 05 3c f3 5a 03        	movzbl	0x35af33c(%rip), %eax   # 0x143634c09
1400858cd: b9 01 00 00 00              	movl	$0x1, %ecx
1400858d2: ba 01 00 00 00              	movl	$0x1, %edx
1400858d7: e8 34 e3 58 00              	callq	0x140613c10 <.text+0x612c10>
1400858dc: 48 85 c0                    	testq	%rax, %rax
1400858df: 0f 84 23 18 00 00           	je	0x140087108 <.text+0x86108>
1400858e5: c6 00 0e                    	movb	$0xe, (%rax)
1400858e8: 48 c7 86 00 01 00 00 01 00 00 00    	movq	$0x1, 0x100(%rsi)
1400858f3: 48 89 86 08 01 00 00        	movq	%rax, 0x108(%rsi)
1400858fa: 48 c7 86 10 01 00 00 01 00 00 00    	movq	$0x1, 0x110(%rsi)
140085905: 0f 10 85 b0 01 00 00        	movups	0x1b0(%rbp), %xmm0
14008590c: 0f 11 86 28 01 00 00        	movups	%xmm0, 0x128(%rsi)
140085913: 8b 85 14 02 00 00           	movl	0x214(%rbp), %eax
140085919: 8b 8d 17 02 00 00           	movl	0x217(%rbp), %ecx
14008591f: 89 86 39 01 00 00           	movl	%eax, 0x139(%rsi)
140085925: 89 8e 3c 01 00 00           	movl	%ecx, 0x13c(%rsi)
14008592b: 48 c7 06 01 00 00 00        	movq	$0x1, (%rsi)
140085932: 48 89 7e 08                 	movq	%rdi, 0x8(%rsi)
140085936: 48 c7 46 10 01 00 00 00     	movq	$0x1, 0x10(%rsi)
14008593e: 48 8d 05 52 b7 8a 02        	leaq	0x28ab752(%rip), %rax   # 0x142931097
140085945: 48 89 46 18                 	movq	%rax, 0x18(%rsi)
140085949: 48 c7 46 20 08 00 00 00     	movq	$0x8, 0x20(%rsi)
140085951: 0f 10 85 b0 01 00 00        	movups	0x1b0(%rbp), %xmm0
140085958: 0f 11 46 28                 	movups	%xmm0, 0x28(%rsi)
14008595c: c6 46 38 04                 	movb	$0x4, 0x38(%rsi)
140085960: 8b 85 14 02 00 00           	movl	0x214(%rbp), %eax
140085966: 8b 8d 17 02 00 00           	movl	0x217(%rbp), %ecx
14008596c: 89 46 39                    	movl	%eax, 0x39(%rsi)
14008596f: 89 4e 3c                    	movl	%ecx, 0x3c(%rsi)
140085972: 48 c7 46 40 01 00 00 00     	movq	$0x1, 0x40(%rsi)
14008597a: 48 89 5e 48                 	movq	%rbx, 0x48(%rsi)
14008597e: 48 c7 46 50 01 00 00 00     	movq	$0x1, 0x50(%rsi)
140085986: 48 8d 05 12 b7 8a 02        	leaq	0x28ab712(%rip), %rax   # 0x14293109f
14008598d: 48 89 46 58                 	movq	%rax, 0x58(%rsi)
140085991: 48 c7 46 60 04 00 00 00     	movq	$0x4, 0x60(%rsi)
140085999: 0f 10 85 b0 01 00 00        	movups	0x1b0(%rbp), %xmm0
1400859a0: 0f 11 46 68                 	movups	%xmm0, 0x68(%rsi)
1400859a4: c6 46 78 04                 	movb	$0x4, 0x78(%rsi)
1400859a8: 8b 85 14 02 00 00           	movl	0x214(%rbp), %eax
1400859ae: 8b 8d 17 02 00 00           	movl	0x217(%rbp), %ecx
1400859b4: 89 4e 7c                    	movl	%ecx, 0x7c(%rsi)
1400859b7: 89 46 79                    	movl	%eax, 0x79(%rsi)
1400859ba: 48 c7 86 80 00 00 00 01 00 00 00    	movq	$0x1, 0x80(%rsi)
1400859c5: 4c 89 b6 88 00 00 00        	movq	%r14, 0x88(%rsi)
1400859cc: 48 c7 86 90 00 00 00 01 00 00 00    	movq	$0x1, 0x90(%rsi)
1400859d7: 48 8d 05 c5 b6 8a 02        	leaq	0x28ab6c5(%rip), %rax   # 0x1429310a3
1400859de: 48 89 86 98 00 00 00        	movq	%rax, 0x98(%rsi)
1400859e5: 48 c7 86 a0 00 00 00 0a 00 00 00    	movq	$0xa, 0xa0(%rsi)
1400859f0: 0f 10 85 b0 01 00 00        	movups	0x1b0(%rbp), %xmm0
1400859f7: 0f 11 86 a8 00 00 00        	movups	%xmm0, 0xa8(%rsi)
1400859fe: c6 86 b8 00 00 00 04        	movb	$0x4, 0xb8(%rsi)
140085a05: 8b 85 14 02 00 00           	movl	0x214(%rbp), %eax
140085a0b: 8b 8d 17 02 00 00           	movl	0x217(%rbp), %ecx
140085a11: 89 8e bc 00 00 00           	movl	%ecx, 0xbc(%rsi)
140085a17: 89 86 b9 00 00 00           	movl	%eax, 0xb9(%rsi)
140085a1d: 48 c7 86 c0 00 00 00 01 00 00 00    	movq	$0x1, 0xc0(%rsi)
140085a28: 4c 89 be c8 00 00 00        	movq	%r15, 0xc8(%rsi)
140085a2f: 48 c7 86 d0 00 00 00 01 00 00 00    	movq	$0x1, 0xd0(%rsi)
140085a3a: 48 8d 05 6c b6 8a 02        	leaq	0x28ab66c(%rip), %rax   # 0x1429310ad
140085a41: 48 89 86 d8 00 00 00        	movq	%rax, 0xd8(%rsi)
140085a48: 48 c7 86 e0 00 00 00 09 00 00 00    	movq	$0x9, 0xe0(%rsi)
140085a53: 0f 10 85 b0 01 00 00        	movups	0x1b0(%rbp), %xmm0
140085a5a: 0f 11 86 e8 00 00 00        	movups	%xmm0, 0xe8(%rsi)
140085a61: c6 86 f8 00 00 00 04        	movb	$0x4, 0xf8(%rsi)
140085a68: 8b 85 14 02 00 00           	movl	0x214(%rbp), %eax
140085a6e: 8b 8d 17 02 00 00           	movl	0x217(%rbp), %ecx
140085a74: 89 8e fc 00 00 00           	movl	%ecx, 0xfc(%rsi)
140085a7a: 89 86 f9 00 00 00           	movl	%eax, 0xf9(%rsi)
140085a80: 48 8d 05 2f b6 8a 02        	leaq	0x28ab62f(%rip), %rax   # 0x1429310b6
140085a87: 48 89 86 18 01 00 00        	movq	%rax, 0x118(%rsi)
140085a8e: 48 c7 86 20 01 00 00 06 00 00 00    	movq	$0x6, 0x120(%rsi)
140085a99: c6 86 38 01 00 00 04        	movb	$0x4, 0x138(%rsi)
140085aa0: 48 c7 85 e8 00 00 00 05 00 00 00    	movq	$0x5, 0xe8(%rbp)
140085aab: 48 89 b5 f0 00 00 00        	movq	%rsi, 0xf0(%rbp)
140085ab2: 48 c7 85 f8 00 00 00 05 00 00 00    	movq	$0x5, 0xf8(%rbp)
140085abd: 0f b6 05 45 f1 5a 03        	movzbl	0x35af145(%rip), %eax   # 0x143634c09
140085ac4: b9 00 01 00 00              	movl	$0x100, %ecx            # imm = 0x100
140085ac9: ba 08 00 00 00              	movl	$0x8, %edx
140085ace: e8 3d e1 58 00              	callq	0x140613c10 <.text+0x612c10>
140085ad3: 48 85 c0                    	testq	%rax, %rax
140085ad6: 0f 84 63 16 00 00           	je	0x14008713f <.text+0x8613f>
140085adc: 48 89 c6                    	movq	%rax, %rsi
140085adf: 0f b6 05 23 f1 5a 03        	movzbl	0x35af123(%rip), %eax   # 0x143634c09
140085ae6: b9 01 00 00 00              	movl	$0x1, %ecx
140085aeb: ba 01 00 00 00              	movl	$0x1, %edx
140085af0: e8 1b e1 58 00              	callq	0x140613c10 <.text+0x612c10>
140085af5: 48 85 c0                    	testq	%rax, %rax
140085af8: 0f 84 63 16 00 00           	je	0x140087161 <.text+0x86161>
140085afe: 48 89 c7                    	movq	%rax, %rdi
140085b01: c6 00 0f                    	movb	$0xf, (%rax)
140085b04: 0f b6 05 fe f0 5a 03        	movzbl	0x35af0fe(%rip), %eax   # 0x143634c09
140085b0b: b9 01 00 00 00              	movl	$0x1, %ecx
140085b10: ba 01 00 00 00              	movl	$0x1, %edx
140085b15: e8 f6 e0 58 00              	callq	0x140613c10 <.text+0x612c10>
140085b1a: 48 85 c0                    	testq	%rax, %rax
140085b1d: 0f 84 59 16 00 00           	je	0x14008717c <.text+0x8617c>
140085b23: 48 89 c3                    	movq	%rax, %rbx
140085b26: c6 00 10                    	movb	$0x10, (%rax)
140085b29: 0f b6 05 d9 f0 5a 03        	movzbl	0x35af0d9(%rip), %eax   # 0x143634c09
140085b30: b9 01 00 00 00              	movl	$0x1, %ecx
140085b35: ba 01 00 00 00              	movl	$0x1, %edx
140085b3a: e8 d1 e0 58 00              	callq	0x140613c10 <.text+0x612c10>
140085b3f: 48 85 c0                    	testq	%rax, %rax
140085b42: 0f 84 56 16 00 00           	je	0x14008719e <.text+0x8619e>
140085b48: 49 89 c6                    	movq	%rax, %r14
140085b4b: c6 00 13                    	movb	$0x13, (%rax)
140085b4e: 0f b6 05 b4 f0 5a 03        	movzbl	0x35af0b4(%rip), %eax   # 0x143634c09
140085b55: b9 01 00 00 00              	movl	$0x1, %ecx
140085b5a: ba 01 00 00 00              	movl	$0x1, %edx
140085b5f: e8 ac e0 58 00              	callq	0x140613c10 <.text+0x612c10>
140085b64: 48 85 c0                    	testq	%rax, %rax
140085b67: 0f 84 5a 16 00 00           	je	0x1400871c7 <.text+0x861c7>
140085b6d: c6 00 14                    	movb	$0x14, (%rax)
140085b70: 48 c7 86 c0 00 00 00 01 00 00 00    	movq	$0x1, 0xc0(%rsi)
140085b7b: 48 89 86 c8 00 00 00        	movq	%rax, 0xc8(%rsi)
140085b82: 48 c7 86 d0 00 00 00 01 00 00 00    	movq	$0x1, 0xd0(%rsi)
140085b8d: 0f 10 85 b0 01 00 00        	movups	0x1b0(%rbp), %xmm0
140085b94: 0f 11 86 e8 00 00 00        	movups	%xmm0, 0xe8(%rsi)
140085b9b: 8b 85 14 02 00 00           	movl	0x214(%rbp), %eax
140085ba1: 8b 8d 17 02 00 00           	movl	0x217(%rbp), %ecx
140085ba7: 89 86 f9 00 00 00           	movl	%eax, 0xf9(%rsi)
140085bad: 89 8e fc 00 00 00           	movl	%ecx, 0xfc(%rsi)
140085bb3: 48 c7 06 01 00 00 00        	movq	$0x1, (%rsi)
140085bba: 48 89 7e 08                 	movq	%rdi, 0x8(%rsi)
140085bbe: 48 c7 46 10 01 00 00 00     	movq	$0x1, 0x10(%rsi)
140085bc6: 48 8d 05 ef b4 8a 02        	leaq	0x28ab4ef(%rip), %rax   # 0x1429310bc
140085bcd: 48 89 46 18                 	movq	%rax, 0x18(%rsi)
140085bd1: 48 c7 46 20 07 00 00 00     	movq	$0x7, 0x20(%rsi)
140085bd9: 0f 10 85 b0 01 00 00        	movups	0x1b0(%rbp), %xmm0
140085be0: 0f 11 46 28                 	movups	%xmm0, 0x28(%rsi)
140085be4: c6 46 38 04                 	movb	$0x4, 0x38(%rsi)
140085be8: 8b 85 14 02 00 00           	movl	0x214(%rbp), %eax
140085bee: 8b 8d 17 02 00 00           	movl	0x217(%rbp), %ecx
140085bf4: 89 46 39                    	movl	%eax, 0x39(%rsi)
140085bf7: 89 4e 3c                    	movl	%ecx, 0x3c(%rsi)
140085bfa: 48 c7 46 40 01 00 00 00     	movq	$0x1, 0x40(%rsi)
140085c02: 48 89 5e 48                 	movq	%rbx, 0x48(%rsi)
140085c06: 48 c7 46 50 01 00 00 00     	movq	$0x1, 0x50(%rsi)
140085c0e: 48 8d 05 ae b4 8a 02        	leaq	0x28ab4ae(%rip), %rax   # 0x1429310c3
140085c15: 48 89 46 58                 	movq	%rax, 0x58(%rsi)
140085c19: 48 c7 46 60 06 00 00 00     	movq	$0x6, 0x60(%rsi)
140085c21: 0f 10 85 b0 01 00 00        	movups	0x1b0(%rbp), %xmm0
140085c28: 0f 11 46 68                 	movups	%xmm0, 0x68(%rsi)
140085c2c: c6 46 78 04                 	movb	$0x4, 0x78(%rsi)
140085c30: 8b 85 14 02 00 00           	movl	0x214(%rbp), %eax
140085c36: 8b 8d 17 02 00 00           	movl	0x217(%rbp), %ecx
140085c3c: 89 4e 7c                    	movl	%ecx, 0x7c(%rsi)
140085c3f: 89 46 79                    	movl	%eax, 0x79(%rsi)
140085c42: 48 c7 86 80 00 00 00 01 00 00 00    	movq	$0x1, 0x80(%rsi)
140085c4d: 4c 89 b6 88 00 00 00        	movq	%r14, 0x88(%rsi)
140085c54: 48 c7 86 90 00 00 00 01 00 00 00    	movq	$0x1, 0x90(%rsi)
140085c5f: 48 8d 05 63 b4 8a 02        	leaq	0x28ab463(%rip), %rax   # 0x1429310c9
140085c66: 48 89 86 98 00 00 00        	movq	%rax, 0x98(%rsi)
140085c6d: 48 c7 86 a0 00 00 00 09 00 00 00    	movq	$0x9, 0xa0(%rsi)
140085c78: 0f 10 85 b0 01 00 00        	movups	0x1b0(%rbp), %xmm0
140085c7f: 0f 11 86 a8 00 00 00        	movups	%xmm0, 0xa8(%rsi)
140085c86: c6 86 b8 00 00 00 04        	movb	$0x4, 0xb8(%rsi)
140085c8d: 8b 85 14 02 00 00           	movl	0x214(%rbp), %eax
140085c93: 8b 8d 17 02 00 00           	movl	0x217(%rbp), %ecx
140085c99: 89 8e bc 00 00 00           	movl	%ecx, 0xbc(%rsi)
140085c9f: 89 86 b9 00 00 00           	movl	%eax, 0xb9(%rsi)
140085ca5: 48 8d 05 26 b4 8a 02        	leaq	0x28ab426(%rip), %rax   # 0x1429310d2
140085cac: 48 89 86 d8 00 00 00        	movq	%rax, 0xd8(%rsi)
140085cb3: 48 c7 86 e0 00 00 00 0b 00 00 00    	movq	$0xb, 0xe0(%rsi)
140085cbe: c6 86 f8 00 00 00 04        	movb	$0x4, 0xf8(%rsi)
140085cc5: 48 c7 85 78 01 00 00 04 00 00 00    	movq	$0x4, 0x178(%rbp)
140085cd0: 48 89 b5 80 01 00 00        	movq	%rsi, 0x180(%rbp)
140085cd7: 48 c7 85 88 01 00 00 04 00 00 00    	movq	$0x4, 0x188(%rbp)
140085ce2: 0f b6 05 20 ef 5a 03        	movzbl	0x35aef20(%rip), %eax   # 0x143634c09
140085ce9: b9 00 02 00 00              	movl	$0x200, %ecx            # imm = 0x200
140085cee: ba 08 00 00 00              	movl	$0x8, %edx
140085cf3: e8 18 df 58 00              	callq	0x140613c10 <.text+0x612c10>
140085cf8: 48 89 85 00 02 00 00        	movq	%rax, 0x200(%rbp)
140085cff: 48 85 c0                    	testq	%rax, %rax
140085d02: 0f 84 ef 14 00 00           	je	0x1400871f7 <.text+0x861f7>
140085d08: 48 8d 0d ce b3 8a 02        	leaq	0x28ab3ce(%rip), %rcx   # 0x1429310dd
140085d0f: ba 06 00 00 00              	movl	$0x6, %edx
140085d14: e8 67 77 b1 00              	callq	0x140b9d480 <.text+0xb9c480>
140085d19: 48 89 c7                    	movq	%rax, %rdi
140085d1c: 49 89 d4                    	movq	%rdx, %r12
140085d1f: 0f b6 05 e3 ee 5a 03        	movzbl	0x35aeee3(%rip), %eax   # 0x143634c09
140085d26: b9 01 00 00 00              	movl	$0x1, %ecx
140085d2b: ba 01 00 00 00              	movl	$0x1, %edx
140085d30: e8 db de 58 00              	callq	0x140613c10 <.text+0x612c10>
140085d35: 48 85 c0                    	testq	%rax, %rax
140085d38: 0f 84 e2 14 00 00           	je	0x140087220 <.text+0x86220>
140085d3e: 48 89 85 f8 01 00 00        	movq	%rax, 0x1f8(%rbp)
140085d45: c6 00 1c                    	movb	$0x1c, (%rax)
140085d48: 48 8d 0d 94 b3 8a 02        	leaq	0x28ab394(%rip), %rcx   # 0x1429310e3
140085d4f: ba 1e 00 00 00              	movl	$0x1e, %edx
140085d54: e8 27 77 b1 00              	callq	0x140b9d480 <.text+0xb9c480>
140085d59: 48 89 c6                    	movq	%rax, %rsi
140085d5c: 48 89 d3                    	movq	%rdx, %rbx
140085d5f: 0f b6 05 a3 ee 5a 03        	movzbl	0x35aeea3(%rip), %eax   # 0x143634c09
140085d66: b9 01 00 00 00              	movl	$0x1, %ecx
140085d6b: ba 01 00 00 00              	movl	$0x1, %edx
140085d70: e8 9b de 58 00              	callq	0x140613c10 <.text+0x612c10>
140085d75: 48 85 c0                    	testq	%rax, %rax
140085d78: 0f 84 b6 14 00 00           	je	0x140087234 <.text+0x86234>
140085d7e: 48 89 85 f0 01 00 00        	movq	%rax, 0x1f0(%rbp)
140085d85: c6 00 1d                    	movb	$0x1d, (%rax)
140085d88: 48 8d 0d 72 b3 8a 02        	leaq	0x28ab372(%rip), %rcx   # 0x142931101
140085d8f: ba 1b 00 00 00              	movl	$0x1b, %edx
140085d94: e8 e7 76 b1 00              	callq	0x140b9d480 <.text+0xb9c480>
140085d99: 49 89 c6                    	movq	%rax, %r14
140085d9c: 49 89 d7                    	movq	%rdx, %r15
140085d9f: 0f b6 05 63 ee 5a 03        	movzbl	0x35aee63(%rip), %eax   # 0x143634c09
140085da6: b9 01 00 00 00              	movl	$0x1, %ecx
140085dab: ba 01 00 00 00              	movl	$0x1, %edx
140085db0: e8 5b de 58 00              	callq	0x140613c10 <.text+0x612c10>
140085db5: 48 85 c0                    	testq	%rax, %rax
140085db8: 0f 84 8a 14 00 00           	je	0x140087248 <.text+0x86248>
140085dbe: 48 89 85 a0 01 00 00        	movq	%rax, 0x1a0(%rbp)
140085dc5: c6 00 1e                    	movb	$0x1e, (%rax)
140085dc8: 48 8d 0d 4d b3 8a 02        	leaq	0x28ab34d(%rip), %rcx   # 0x14293111c
140085dcf: ba 1c 00 00 00              	movl	$0x1c, %edx
140085dd4: e8 a7 76 b1 00              	callq	0x140b9d480 <.text+0xb9c480>
140085dd9: 48 89 95 e0 01 00 00        	movq	%rdx, 0x1e0(%rbp)
140085de0: 49 89 c5                    	movq	%rax, %r13
140085de3: 0f b6 05 1f ee 5a 03        	movzbl	0x35aee1f(%rip), %eax   # 0x143634c09
140085dea: b9 01 00 00 00              	movl	$0x1, %ecx
140085def: ba 01 00 00 00              	movl	$0x1, %edx
140085df4: e8 17 de 58 00              	callq	0x140613c10 <.text+0x612c10>
140085df9: 48 85 c0                    	testq	%rax, %rax
140085dfc: 0f 84 5a 14 00 00           	je	0x14008725c <.text+0x8625c>
140085e02: 48 89 85 90 01 00 00        	movq	%rax, 0x190(%rbp)
140085e09: c6 00 1f                    	movb	$0x1f, (%rax)
140085e0c: 48 8d 0d 25 b3 8a 02        	leaq	0x28ab325(%rip), %rcx   # 0x142931138
140085e13: ba 1b 00 00 00              	movl	$0x1b, %edx
140085e18: e8 63 76 b1 00              	callq	0x140b9d480 <.text+0xb9c480>
140085e1d: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
140085e24: 48 89 95 c8 01 00 00        	movq	%rdx, 0x1c8(%rbp)
140085e2b: 0f b6 05 d7 ed 5a 03        	movzbl	0x35aedd7(%rip), %eax   # 0x143634c09
140085e32: b9 01 00 00 00              	movl	$0x1, %ecx
140085e37: ba 01 00 00 00              	movl	$0x1, %edx
140085e3c: e8 cf dd 58 00              	callq	0x140613c10 <.text+0x612c10>
140085e41: 48 85 c0                    	testq	%rax, %rax
140085e44: 0f 84 26 14 00 00           	je	0x140087270 <.text+0x86270>
140085e4a: 48 89 85 98 01 00 00        	movq	%rax, 0x198(%rbp)
140085e51: c6 00 20                    	movb	$0x20, (%rax)
140085e54: 48 8d 0d f8 b2 8a 02        	leaq	0x28ab2f8(%rip), %rcx   # 0x142931153
140085e5b: ba 19 00 00 00              	movl	$0x19, %edx
140085e60: e8 1b 76 b1 00              	callq	0x140b9d480 <.text+0xb9c480>
140085e65: 48 89 85 d0 01 00 00        	movq	%rax, 0x1d0(%rbp)
140085e6c: 48 89 95 a8 01 00 00        	movq	%rdx, 0x1a8(%rbp)
140085e73: 0f b6 05 8f ed 5a 03        	movzbl	0x35aed8f(%rip), %eax   # 0x143634c09
140085e7a: b9 01 00 00 00              	movl	$0x1, %ecx
140085e7f: ba 01 00 00 00              	movl	$0x1, %edx
140085e84: e8 87 dd 58 00              	callq	0x140613c10 <.text+0x612c10>
140085e89: 48 85 c0                    	testq	%rax, %rax
140085e8c: 0f 84 f2 13 00 00           	je	0x140087284 <.text+0x86284>
140085e92: 48 89 85 e8 01 00 00        	movq	%rax, 0x1e8(%rbp)
140085e99: c6 00 21                    	movb	$0x21, (%rax)
140085e9c: 48 8d 0d c9 b2 8a 02        	leaq	0x28ab2c9(%rip), %rcx   # 0x14293116c
140085ea3: ba 22 00 00 00              	movl	$0x22, %edx
140085ea8: e8 d3 75 b1 00              	callq	0x140b9d480 <.text+0xb9c480>
140085ead: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
140085eb1: 48 89 55 40                 	movq	%rdx, 0x40(%rbp)
140085eb5: 0f b6 05 4d ed 5a 03        	movzbl	0x35aed4d(%rip), %eax   # 0x143634c09
140085ebc: b9 01 00 00 00              	movl	$0x1, %ecx
140085ec1: ba 01 00 00 00              	movl	$0x1, %edx
140085ec6: e8 45 dd 58 00              	callq	0x140613c10 <.text+0x612c10>
140085ecb: 48 85 c0                    	testq	%rax, %rax
140085ece: 0f 84 c4 13 00 00           	je	0x140087298 <.text+0x86298>
140085ed4: 48 89 85 70 01 00 00        	movq	%rax, 0x170(%rbp)
140085edb: c6 00 22                    	movb	$0x22, (%rax)
140085ede: 48 8d 0d a9 b2 8a 02        	leaq	0x28ab2a9(%rip), %rcx   # 0x14293118e
140085ee5: ba 1e 00 00 00              	movl	$0x1e, %edx
140085eea: e8 91 75 b1 00              	callq	0x140b9d480 <.text+0xb9c480>
140085eef: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
140085ef3: 48 89 55 50                 	movq	%rdx, 0x50(%rbp)
140085ef7: 0f b6 05 0b ed 5a 03        	movzbl	0x35aed0b(%rip), %eax   # 0x143634c09
140085efe: b9 01 00 00 00              	movl	$0x1, %ecx
140085f03: ba 01 00 00 00              	movl	$0x1, %edx
140085f08: e8 03 dd 58 00              	callq	0x140613c10 <.text+0x612c10>
140085f0d: 48 85 c0                    	testq	%rax, %rax
140085f10: 0f 84 96 13 00 00           	je	0x1400872ac <.text+0x862ac>
140085f16: c6 00 23                    	movb	$0x23, (%rax)
140085f19: 48 8b 8d 00 02 00 00        	movq	0x200(%rbp), %rcx
140085f20: 48 c7 81 c0 01 00 00 01 00 00 00    	movq	$0x1, 0x1c0(%rcx)
140085f2b: 48 89 81 c8 01 00 00        	movq	%rax, 0x1c8(%rcx)
140085f32: 48 c7 81 d0 01 00 00 01 00 00 00    	movq	$0x1, 0x1d0(%rcx)
140085f3d: 0f 10 85 b0 01 00 00        	movups	0x1b0(%rbp), %xmm0
140085f44: 0f 11 81 e8 01 00 00        	movups	%xmm0, 0x1e8(%rcx)
140085f4b: 8b 85 14 02 00 00           	movl	0x214(%rbp), %eax
140085f51: 89 81 f9 01 00 00           	movl	%eax, 0x1f9(%rcx)
140085f57: 8b 85 17 02 00 00           	movl	0x217(%rbp), %eax
140085f5d: 89 81 fc 01 00 00           	movl	%eax, 0x1fc(%rcx)
140085f63: 48 c7 01 01 00 00 00        	movq	$0x1, (%rcx)
140085f6a: 48 8b 85 f8 01 00 00        	movq	0x1f8(%rbp), %rax
140085f71: 48 89 41 08                 	movq	%rax, 0x8(%rcx)
140085f75: 48 c7 41 10 01 00 00 00     	movq	$0x1, 0x10(%rcx)
140085f7d: 48 89 79 18                 	movq	%rdi, 0x18(%rcx)
140085f81: 4c 89 61 20                 	movq	%r12, 0x20(%rcx)
140085f85: 0f 10 85 b0 01 00 00        	movups	0x1b0(%rbp), %xmm0
140085f8c: 0f 11 41 28                 	movups	%xmm0, 0x28(%rcx)
140085f90: c6 41 38 04                 	movb	$0x4, 0x38(%rcx)
140085f94: 8b 85 14 02 00 00           	movl	0x214(%rbp), %eax
140085f9a: 89 41 39                    	movl	%eax, 0x39(%rcx)
140085f9d: 8b 85 17 02 00 00           	movl	0x217(%rbp), %eax
140085fa3: 89 41 3c                    	movl	%eax, 0x3c(%rcx)
140085fa6: 48 c7 41 40 01 00 00 00     	movq	$0x1, 0x40(%rcx)
140085fae: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
140085fb5: 48 89 41 48                 	movq	%rax, 0x48(%rcx)
140085fb9: 48 c7 41 50 01 00 00 00     	movq	$0x1, 0x50(%rcx)
140085fc1: 48 89 71 58                 	movq	%rsi, 0x58(%rcx)
140085fc5: 48 89 59 60                 	movq	%rbx, 0x60(%rcx)
140085fc9: 0f 10 85 b0 01 00 00        	movups	0x1b0(%rbp), %xmm0
140085fd0: 0f 11 41 68                 	movups	%xmm0, 0x68(%rcx)
140085fd4: c6 41 78 04                 	movb	$0x4, 0x78(%rcx)
140085fd8: 8b 85 17 02 00 00           	movl	0x217(%rbp), %eax
140085fde: 89 41 7c                    	movl	%eax, 0x7c(%rcx)
140085fe1: 8b 85 14 02 00 00           	movl	0x214(%rbp), %eax
140085fe7: 89 41 79                    	movl	%eax, 0x79(%rcx)
140085fea: 48 c7 81 80 00 00 00 01 00 00 00    	movq	$0x1, 0x80(%rcx)
140085ff5: 48 8b 85 a0 01 00 00        	movq	0x1a0(%rbp), %rax
140085ffc: 48 89 81 88 00 00 00        	movq	%rax, 0x88(%rcx)
140086003: 48 c7 81 90 00 00 00 01 00 00 00    	movq	$0x1, 0x90(%rcx)
14008600e: 4c 89 b1 98 00 00 00        	movq	%r14, 0x98(%rcx)
140086015: 4c 89 b9 a0 00 00 00        	movq	%r15, 0xa0(%rcx)
14008601c: 0f 10 85 b0 01 00 00        	movups	0x1b0(%rbp), %xmm0
140086023: 0f 11 81 a8 00 00 00        	movups	%xmm0, 0xa8(%rcx)
14008602a: c6 81 b8 00 00 00 04        	movb	$0x4, 0xb8(%rcx)
140086031: 8b 85 17 02 00 00           	movl	0x217(%rbp), %eax
140086037: 89 81 bc 00 00 00           	movl	%eax, 0xbc(%rcx)
14008603d: 8b 85 14 02 00 00           	movl	0x214(%rbp), %eax
140086043: 89 81 b9 00 00 00           	movl	%eax, 0xb9(%rcx)
140086049: 48 c7 81 c0 00 00 00 01 00 00 00    	movq	$0x1, 0xc0(%rcx)
140086054: 48 8b 85 90 01 00 00        	movq	0x190(%rbp), %rax
14008605b: 48 89 81 c8 00 00 00        	movq	%rax, 0xc8(%rcx)
140086062: 48 c7 81 d0 00 00 00 01 00 00 00    	movq	$0x1, 0xd0(%rcx)
14008606d: 4c 89 a9 d8 00 00 00        	movq	%r13, 0xd8(%rcx)
140086074: 48 8b 85 e0 01 00 00        	movq	0x1e0(%rbp), %rax
14008607b: 48 89 81 e0 00 00 00        	movq	%rax, 0xe0(%rcx)
140086082: 0f 10 85 b0 01 00 00        	movups	0x1b0(%rbp), %xmm0
140086089: 0f 11 81 e8 00 00 00        	movups	%xmm0, 0xe8(%rcx)
140086090: c6 81 f8 00 00 00 04        	movb	$0x4, 0xf8(%rcx)
140086097: 8b 85 17 02 00 00           	movl	0x217(%rbp), %eax
14008609d: 89 81 fc 00 00 00           	movl	%eax, 0xfc(%rcx)
1400860a3: 8b 85 14 02 00 00           	movl	0x214(%rbp), %eax
1400860a9: 89 81 f9 00 00 00           	movl	%eax, 0xf9(%rcx)
1400860af: 48 c7 81 00 01 00 00 01 00 00 00    	movq	$0x1, 0x100(%rcx)
1400860ba: 48 8b 85 98 01 00 00        	movq	0x198(%rbp), %rax
1400860c1: 48 89 81 08 01 00 00        	movq	%rax, 0x108(%rcx)
1400860c8: 48 c7 81 10 01 00 00 01 00 00 00    	movq	$0x1, 0x110(%rcx)
1400860d3: 48 8b 85 c0 01 00 00        	movq	0x1c0(%rbp), %rax
1400860da: 48 89 81 18 01 00 00        	movq	%rax, 0x118(%rcx)
1400860e1: 48 8b 85 c8 01 00 00        	movq	0x1c8(%rbp), %rax
1400860e8: 48 89 81 20 01 00 00        	movq	%rax, 0x120(%rcx)
1400860ef: 0f 10 85 b0 01 00 00        	movups	0x1b0(%rbp), %xmm0
1400860f6: 0f 11 81 28 01 00 00        	movups	%xmm0, 0x128(%rcx)
1400860fd: c6 81 38 01 00 00 04        	movb	$0x4, 0x138(%rcx)
140086104: 8b 85 17 02 00 00           	movl	0x217(%rbp), %eax
14008610a: 89 81 3c 01 00 00           	movl	%eax, 0x13c(%rcx)
140086110: 8b 85 14 02 00 00           	movl	0x214(%rbp), %eax
140086116: 89 81 39 01 00 00           	movl	%eax, 0x139(%rcx)
14008611c: 48 c7 81 40 01 00 00 01 00 00 00    	movq	$0x1, 0x140(%rcx)
140086127: 48 8b 85 e8 01 00 00        	movq	0x1e8(%rbp), %rax
14008612e: 48 89 81 48 01 00 00        	movq	%rax, 0x148(%rcx)
140086135: 48 c7 81 50 01 00 00 01 00 00 00    	movq	$0x1, 0x150(%rcx)
140086140: 48 8b 85 d0 01 00 00        	movq	0x1d0(%rbp), %rax
140086147: 48 89 81 58 01 00 00        	movq	%rax, 0x158(%rcx)
14008614e: 48 8b 85 a8 01 00 00        	movq	0x1a8(%rbp), %rax
140086155: 48 89 81 60 01 00 00        	movq	%rax, 0x160(%rcx)
14008615c: 0f 10 85 b0 01 00 00        	movups	0x1b0(%rbp), %xmm0
140086163: 0f 11 81 68 01 00 00        	movups	%xmm0, 0x168(%rcx)
14008616a: c6 81 78 01 00 00 04        	movb	$0x4, 0x178(%rcx)
140086171: 8b 85 17 02 00 00           	movl	0x217(%rbp), %eax
140086177: 89 81 7c 01 00 00           	movl	%eax, 0x17c(%rcx)
14008617d: 8b 85 14 02 00 00           	movl	0x214(%rbp), %eax
140086183: 89 81 79 01 00 00           	movl	%eax, 0x179(%rcx)
140086189: 48 c7 81 80 01 00 00 01 00 00 00    	movq	$0x1, 0x180(%rcx)
140086194: 48 8b 85 70 01 00 00        	movq	0x170(%rbp), %rax
14008619b: 48 89 81 88 01 00 00        	movq	%rax, 0x188(%rcx)
1400861a2: 48 c7 81 90 01 00 00 01 00 00 00    	movq	$0x1, 0x190(%rcx)
1400861ad: 48 8b 45 38                 	movq	0x38(%rbp), %rax
1400861b1: 48 89 81 98 01 00 00        	movq	%rax, 0x198(%rcx)
1400861b8: 48 8b 45 40                 	movq	0x40(%rbp), %rax
1400861bc: 48 89 81 a0 01 00 00        	movq	%rax, 0x1a0(%rcx)
1400861c3: 0f 10 85 b0 01 00 00        	movups	0x1b0(%rbp), %xmm0
1400861ca: 0f 11 81 a8 01 00 00        	movups	%xmm0, 0x1a8(%rcx)
1400861d1: c6 81 b8 01 00 00 04        	movb	$0x4, 0x1b8(%rcx)
1400861d8: 8b 85 17 02 00 00           	movl	0x217(%rbp), %eax
1400861de: 89 81 bc 01 00 00           	movl	%eax, 0x1bc(%rcx)
1400861e4: 8b 85 14 02 00 00           	movl	0x214(%rbp), %eax
1400861ea: 89 81 b9 01 00 00           	movl	%eax, 0x1b9(%rcx)
1400861f0: 48 8b 45 48                 	movq	0x48(%rbp), %rax
1400861f4: 48 89 81 d8 01 00 00        	movq	%rax, 0x1d8(%rcx)
1400861fb: 48 8b 45 50                 	movq	0x50(%rbp), %rax
1400861ff: 48 89 81 e0 01 00 00        	movq	%rax, 0x1e0(%rcx)
140086206: c6 81 f8 01 00 00 04        	movb	$0x4, 0x1f8(%rcx)
14008620d: 48 c7 85 d0 00 00 00 08 00 00 00    	movq	$0x8, 0xd0(%rbp)
140086218: 48 89 8d d8 00 00 00        	movq	%rcx, 0xd8(%rbp)
14008621f: 48 8b 05 72 5e 5a 03        	movq	0x35a5e72(%rip), %rax   # 0x14362c098
140086226: 80 38 00                    	cmpb	$0x0, (%rax)
140086229: 48 c7 85 e0 00 00 00 08 00 00 00    	movq	$0x8, 0xe0(%rbp)
140086234: 0f 85 ed 01 00 00           	jne	0x140086427 <.text+0x85427>
14008623a: 0f b6 05 c8 e9 5a 03        	movzbl	0x35ae9c8(%rip), %eax   # 0x143634c09
140086241: b9 01 00 00 00              	movl	$0x1, %ecx
140086246: ba 01 00 00 00              	movl	$0x1, %edx
14008624b: e8 c0 d9 58 00              	callq	0x140613c10 <.text+0x612c10>
140086250: 48 85 c0                    	testq	%rax, %rax
140086253: 0f 84 47 11 00 00           	je	0x1400873a0 <.text+0x863a0>
140086259: 48 89 85 00 02 00 00        	movq	%rax, 0x200(%rbp)
140086260: c6 00 11                    	movb	$0x11, (%rax)
140086263: 48 8b b5 88 01 00 00        	movq	0x188(%rbp), %rsi
14008626a: 48 83 fe 02                 	cmpq	$0x2, %rsi
14008626e: 0f 82 4d 0d 00 00           	jb	0x140086fc1 <.text+0x85fc1>
140086274: 48 3b b5 78 01 00 00        	cmpq	0x178(%rbp), %rsi
14008627b: 75 13                       	jne	0x140086290 <.text+0x85290>
14008627d: 48 8d 15 5c af 8a 02        	leaq	0x28aaf5c(%rip), %rdx   # 0x1429311e0
140086284: 48 8d 8d 78 01 00 00        	leaq	0x178(%rbp), %rcx
14008628b: e8 20 c5 4d 00              	callq	0x1405627b0 <.text+0x5617b0>
140086290: 48 8b bd 80 01 00 00        	movq	0x180(%rbp), %rdi
140086297: 48 83 fe 02                 	cmpq	$0x2, %rsi
14008629b: 74 1e                       	je	0x1400862bb <.text+0x852bb>
14008629d: 48 8d 97 80 00 00 00        	leaq	0x80(%rdi), %rdx
1400862a4: 48 8d 8f c0 00 00 00        	leaq	0xc0(%rdi), %rcx
1400862ab: 49 89 f0                    	movq	%rsi, %r8
1400862ae: 49 c1 e0 06                 	shlq	$0x6, %r8
1400862b2: 49 83 c0 80                 	addq	$-0x80, %r8
1400862b6: e8 55 d9 89 02              	callq	0x142923c10 <.text+0x2922c10>
1400862bb: 48 c7 87 80 00 00 00 01 00 00 00    	movq	$0x1, 0x80(%rdi)
1400862c6: 48 8b 85 00 02 00 00        	movq	0x200(%rbp), %rax
1400862cd: 48 89 87 88 00 00 00        	movq	%rax, 0x88(%rdi)
1400862d4: 48 c7 87 90 00 00 00 01 00 00 00    	movq	$0x1, 0x90(%rdi)
1400862df: 48 8d 05 c6 ae 8a 02        	leaq	0x28aaec6(%rip), %rax   # 0x1429311ac
1400862e6: 48 89 87 98 00 00 00        	movq	%rax, 0x98(%rdi)
1400862ed: 48 c7 87 a0 00 00 00 04 00 00 00    	movq	$0x4, 0xa0(%rdi)
1400862f8: 0f 10 85 b0 01 00 00        	movups	0x1b0(%rbp), %xmm0
1400862ff: 0f 11 87 a8 00 00 00        	movups	%xmm0, 0xa8(%rdi)
140086306: c6 87 b8 00 00 00 04        	movb	$0x4, 0xb8(%rdi)
14008630d: 8b 85 14 02 00 00           	movl	0x214(%rbp), %eax
140086313: 8b 8d 17 02 00 00           	movl	0x217(%rbp), %ecx
140086319: 89 87 b9 00 00 00           	movl	%eax, 0xb9(%rdi)
14008631f: 89 8f bc 00 00 00           	movl	%ecx, 0xbc(%rdi)
140086325: 48 ff c6                    	incq	%rsi
140086328: 48 89 b5 88 01 00 00        	movq	%rsi, 0x188(%rbp)
14008632f: 0f b6 05 d3 e8 5a 03        	movzbl	0x35ae8d3(%rip), %eax   # 0x143634c09
140086336: b9 01 00 00 00              	movl	$0x1, %ecx
14008633b: ba 01 00 00 00              	movl	$0x1, %edx
140086340: e8 cb d8 58 00              	callq	0x140613c10 <.text+0x612c10>
140086345: 48 85 c0                    	testq	%rax, %rax
140086348: 0f 84 7f 10 00 00           	je	0x1400873cd <.text+0x863cd>
14008634e: 48 89 85 00 02 00 00        	movq	%rax, 0x200(%rbp)
140086355: c6 00 12                    	movb	$0x12, (%rax)
140086358: 48 8b b5 88 01 00 00        	movq	0x188(%rbp), %rsi
14008635f: 48 83 fe 03                 	cmpq	$0x3, %rsi
140086363: 0f 82 71 0c 00 00           	jb	0x140086fda <.text+0x85fda>
140086369: 48 3b b5 78 01 00 00        	cmpq	0x178(%rbp), %rsi
140086370: 75 13                       	jne	0x140086385 <.text+0x85385>
140086372: 48 8d 15 87 ae 8a 02        	leaq	0x28aae87(%rip), %rdx   # 0x142931200
140086379: 48 8d 8d 78 01 00 00        	leaq	0x178(%rbp), %rcx
140086380: e8 2b c4 4d 00              	callq	0x1405627b0 <.text+0x5617b0>
140086385: 48 8b bd 80 01 00 00        	movq	0x180(%rbp), %rdi
14008638c: 48 83 fe 03                 	cmpq	$0x3, %rsi
140086390: 74 21                       	je	0x1400863b3 <.text+0x853b3>
140086392: 48 8d 97 c0 00 00 00        	leaq	0xc0(%rdi), %rdx
140086399: 48 8d 8f 00 01 00 00        	leaq	0x100(%rdi), %rcx
1400863a0: 49 89 f0                    	movq	%rsi, %r8
1400863a3: 49 c1 e0 06                 	shlq	$0x6, %r8
1400863a7: 49 81 c0 40 ff ff ff        	addq	$-0xc0, %r8
1400863ae: e8 5d d8 89 02              	callq	0x142923c10 <.text+0x2922c10>
1400863b3: 48 c7 87 c0 00 00 00 01 00 00 00    	movq	$0x1, 0xc0(%rdi)
1400863be: 48 8b 85 00 02 00 00        	movq	0x200(%rbp), %rax
1400863c5: 48 89 87 c8 00 00 00        	movq	%rax, 0xc8(%rdi)
1400863cc: 48 c7 87 d0 00 00 00 01 00 00 00    	movq	$0x1, 0xd0(%rdi)
1400863d7: 48 8d 05 1a ae 8a 02        	leaq	0x28aae1a(%rip), %rax   # 0x1429311f8
1400863de: 48 89 87 d8 00 00 00        	movq	%rax, 0xd8(%rdi)
1400863e5: 48 c7 87 e0 00 00 00 05 00 00 00    	movq	$0x5, 0xe0(%rdi)
1400863f0: 0f 10 85 b0 01 00 00        	movups	0x1b0(%rbp), %xmm0
1400863f7: 0f 11 87 e8 00 00 00        	movups	%xmm0, 0xe8(%rdi)
1400863fe: c6 87 f8 00 00 00 04        	movb	$0x4, 0xf8(%rdi)
140086405: 8b 85 14 02 00 00           	movl	0x214(%rbp), %eax
14008640b: 8b 8d 17 02 00 00           	movl	0x217(%rbp), %ecx
140086411: 89 87 f9 00 00 00           	movl	%eax, 0xf9(%rdi)
140086417: 89 8f fc 00 00 00           	movl	%ecx, 0xfc(%rdi)
14008641d: 48 ff c6                    	incq	%rsi
140086420: 48 89 b5 88 01 00 00        	movq	%rsi, 0x188(%rbp)
140086427: 0f b6 05 db e7 5a 03        	movzbl	0x35ae7db(%rip), %eax   # 0x143634c09
14008642e: b9 40 01 00 00              	movl	$0x140, %ecx            # imm = 0x140
140086433: ba 08 00 00 00              	movl	$0x8, %edx
140086438: e8 d3 d7 58 00              	callq	0x140613c10 <.text+0x612c10>
14008643d: 48 89 85 98 01 00 00        	movq	%rax, 0x198(%rbp)
140086444: 48 85 c0                    	testq	%rax, %rax
140086447: 0f 84 73 0e 00 00           	je	0x1400872c0 <.text+0x862c0>
14008644d: 48 8d 0d c4 ad 8a 02        	leaq	0x28aadc4(%rip), %rcx   # 0x142931218
140086454: ba 14 00 00 00              	movl	$0x14, %edx
140086459: e8 22 70 b1 00              	callq	0x140b9d480 <.text+0xb9c480>
14008645e: 48 89 c7                    	movq	%rax, %rdi
140086461: 49 89 d6                    	movq	%rdx, %r14
140086464: 0f b6 05 9e e7 5a 03        	movzbl	0x35ae79e(%rip), %eax   # 0x143634c09
14008646b: b9 02 00 00 00              	movl	$0x2, %ecx
140086470: ba 01 00 00 00              	movl	$0x1, %edx
140086475: e8 96 d7 58 00              	callq	0x140613c10 <.text+0x612c10>
14008647a: 48 85 c0                    	testq	%rax, %rax
14008647d: 0f 84 6d 0e 00 00           	je	0x1400872f0 <.text+0x862f0>
140086483: 48 89 85 00 02 00 00        	movq	%rax, 0x200(%rbp)
14008648a: 66 c7 00 15 16              	movw	$0x1615, (%rax)         # imm = 0x1615
14008648f: 48 8d 0d 96 ad 8a 02        	leaq	0x28aad96(%rip), %rcx   # 0x14293122c
140086496: ba 1b 00 00 00              	movl	$0x1b, %edx
14008649b: e8 e0 6f b1 00              	callq	0x140b9d480 <.text+0xb9c480>
1400864a0: 49 89 c7                    	movq	%rax, %r15
1400864a3: 49 89 d4                    	movq	%rdx, %r12
1400864a6: 0f b6 05 5c e7 5a 03        	movzbl	0x35ae75c(%rip), %eax   # 0x143634c09
1400864ad: b9 01 00 00 00              	movl	$0x1, %ecx
1400864b2: ba 01 00 00 00              	movl	$0x1, %edx
1400864b7: e8 54 d7 58 00              	callq	0x140613c10 <.text+0x612c10>
1400864bc: 48 85 c0                    	testq	%rax, %rax
1400864bf: 0f 84 3f 0e 00 00           	je	0x140087304 <.text+0x86304>
1400864c5: 48 89 85 f8 01 00 00        	movq	%rax, 0x1f8(%rbp)
1400864cc: c6 00 17                    	movb	$0x17, (%rax)
1400864cf: 48 8d 0d 79 ad 8a 02        	leaq	0x28aad79(%rip), %rcx   # 0x14293124f
1400864d6: ba 1b 00 00 00              	movl	$0x1b, %edx
1400864db: e8 a0 6f b1 00              	callq	0x140b9d480 <.text+0xb9c480>
1400864e0: 48 89 c6                    	movq	%rax, %rsi
1400864e3: 49 89 d5                    	movq	%rdx, %r13
1400864e6: 0f b6 05 1c e7 5a 03        	movzbl	0x35ae71c(%rip), %eax   # 0x143634c09
1400864ed: b9 01 00 00 00              	movl	$0x1, %ecx
1400864f2: ba 01 00 00 00              	movl	$0x1, %edx
1400864f7: e8 14 d7 58 00              	callq	0x140613c10 <.text+0x612c10>
1400864fc: 48 85 c0                    	testq	%rax, %rax
1400864ff: 0f 84 13 0e 00 00           	je	0x140087318 <.text+0x86318>
140086505: 48 89 85 f0 01 00 00        	movq	%rax, 0x1f0(%rbp)
14008650c: c6 00 18                    	movb	$0x18, (%rax)
14008650f: 48 8d 0d 54 ad 8a 02        	leaq	0x28aad54(%rip), %rcx   # 0x14293126a
140086516: ba 18 00 00 00              	movl	$0x18, %edx
14008651b: e8 60 6f b1 00              	callq	0x140b9d480 <.text+0xb9c480>
140086520: 48 89 85 90 01 00 00        	movq	%rax, 0x190(%rbp)
140086527: 48 89 d3                    	movq	%rdx, %rbx
14008652a: 0f b6 05 d8 e6 5a 03        	movzbl	0x35ae6d8(%rip), %eax   # 0x143634c09
140086531: b9 01 00 00 00              	movl	$0x1, %ecx
140086536: ba 01 00 00 00              	movl	$0x1, %edx
14008653b: e8 d0 d6 58 00              	callq	0x140613c10 <.text+0x612c10>
140086540: 48 85 c0                    	testq	%rax, %rax
140086543: 0f 84 e3 0d 00 00           	je	0x14008732c <.text+0x8632c>
140086549: 48 89 85 a0 01 00 00        	movq	%rax, 0x1a0(%rbp)
140086550: c6 00 1a                    	movb	$0x1a, (%rax)
140086553: 48 8d 0d 28 ad 8a 02        	leaq	0x28aad28(%rip), %rcx   # 0x142931282
14008655a: ba 1d 00 00 00              	movl	$0x1d, %edx
14008655f: e8 1c 6f b1 00              	callq	0x140b9d480 <.text+0xb9c480>
140086564: 48 89 85 e0 01 00 00        	movq	%rax, 0x1e0(%rbp)
14008656b: 48 89 95 e8 01 00 00        	movq	%rdx, 0x1e8(%rbp)
140086572: 0f b6 05 90 e6 5a 03        	movzbl	0x35ae690(%rip), %eax   # 0x143634c09
140086579: b9 01 00 00 00              	movl	$0x1, %ecx
14008657e: ba 01 00 00 00              	movl	$0x1, %edx
140086583: e8 88 d6 58 00              	callq	0x140613c10 <.text+0x612c10>
140086588: 48 85 c0                    	testq	%rax, %rax
14008658b: 0f 84 af 0d 00 00           	je	0x140087340 <.text+0x86340>
140086591: c6 00 1b                    	movb	$0x1b, (%rax)
140086594: 48 8b 8d 98 01 00 00        	movq	0x198(%rbp), %rcx
14008659b: 48 c7 81 00 01 00 00 01 00 00 00    	movq	$0x1, 0x100(%rcx)
1400865a6: 48 89 81 08 01 00 00        	movq	%rax, 0x108(%rcx)
1400865ad: 48 c7 81 10 01 00 00 01 00 00 00    	movq	$0x1, 0x110(%rcx)
1400865b8: 0f 10 85 b0 01 00 00        	movups	0x1b0(%rbp), %xmm0
1400865bf: 0f 11 81 28 01 00 00        	movups	%xmm0, 0x128(%rcx)
1400865c6: 8b 85 14 02 00 00           	movl	0x214(%rbp), %eax
1400865cc: 89 81 39 01 00 00           	movl	%eax, 0x139(%rcx)
1400865d2: 8b 85 17 02 00 00           	movl	0x217(%rbp), %eax
1400865d8: 89 81 3c 01 00 00           	movl	%eax, 0x13c(%rcx)
1400865de: 48 c7 01 02 00 00 00        	movq	$0x2, (%rcx)
1400865e5: 48 8b 85 00 02 00 00        	movq	0x200(%rbp), %rax
1400865ec: 48 89 41 08                 	movq	%rax, 0x8(%rcx)
1400865f0: 48 c7 41 10 02 00 00 00     	movq	$0x2, 0x10(%rcx)
1400865f8: 48 89 79 18                 	movq	%rdi, 0x18(%rcx)
1400865fc: 4c 89 71 20                 	movq	%r14, 0x20(%rcx)
140086600: 0f 10 85 b0 01 00 00        	movups	0x1b0(%rbp), %xmm0
140086607: 0f 11 41 28                 	movups	%xmm0, 0x28(%rcx)
14008660b: c6 41 38 04                 	movb	$0x4, 0x38(%rcx)
14008660f: 8b 85 14 02 00 00           	movl	0x214(%rbp), %eax
140086615: 89 41 39                    	movl	%eax, 0x39(%rcx)
140086618: 8b 85 17 02 00 00           	movl	0x217(%rbp), %eax
14008661e: 89 41 3c                    	movl	%eax, 0x3c(%rcx)
140086621: 48 c7 41 40 01 00 00 00     	movq	$0x1, 0x40(%rcx)
140086629: 48 8b 85 f8 01 00 00        	movq	0x1f8(%rbp), %rax
140086630: 48 89 41 48                 	movq	%rax, 0x48(%rcx)
140086634: 48 c7 41 50 01 00 00 00     	movq	$0x1, 0x50(%rcx)
14008663c: 4c 89 79 58                 	movq	%r15, 0x58(%rcx)
140086640: 4c 89 61 60                 	movq	%r12, 0x60(%rcx)
140086644: 48 8d 05 fc ab 8a 02        	leaq	0x28aabfc(%rip), %rax   # 0x142931247
14008664b: 48 89 41 68                 	movq	%rax, 0x68(%rcx)
14008664f: 48 c7 41 70 08 00 00 00     	movq	$0x8, 0x70(%rcx)
140086657: c6 41 78 02                 	movb	$0x2, 0x78(%rcx)
14008665b: 48 c7 81 80 00 00 00 01 00 00 00    	movq	$0x1, 0x80(%rcx)
140086666: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
14008666d: 48 89 81 88 00 00 00        	movq	%rax, 0x88(%rcx)
140086674: 48 c7 81 90 00 00 00 01 00 00 00    	movq	$0x1, 0x90(%rcx)
14008667f: 48 89 b1 98 00 00 00        	movq	%rsi, 0x98(%rcx)
140086686: 4c 89 a9 a0 00 00 00        	movq	%r13, 0xa0(%rcx)
14008668d: 0f 10 85 b0 01 00 00        	movups	0x1b0(%rbp), %xmm0
140086694: 0f 11 81 a8 00 00 00        	movups	%xmm0, 0xa8(%rcx)
14008669b: c6 81 b8 00 00 00 04        	movb	$0x4, 0xb8(%rcx)
1400866a2: 8b 85 17 02 00 00           	movl	0x217(%rbp), %eax
1400866a8: 89 81 bc 00 00 00           	movl	%eax, 0xbc(%rcx)
1400866ae: 8b 85 14 02 00 00           	movl	0x214(%rbp), %eax
1400866b4: 89 81 b9 00 00 00           	movl	%eax, 0xb9(%rcx)
1400866ba: 48 c7 81 c0 00 00 00 01 00 00 00    	movq	$0x1, 0xc0(%rcx)
1400866c5: 48 8b 85 a0 01 00 00        	movq	0x1a0(%rbp), %rax
1400866cc: 48 89 81 c8 00 00 00        	movq	%rax, 0xc8(%rcx)
1400866d3: 48 c7 81 d0 00 00 00 01 00 00 00    	movq	$0x1, 0xd0(%rcx)
1400866de: 48 8b 85 90 01 00 00        	movq	0x190(%rbp), %rax
1400866e5: 48 89 81 d8 00 00 00        	movq	%rax, 0xd8(%rcx)
1400866ec: 48 89 99 e0 00 00 00        	movq	%rbx, 0xe0(%rcx)
1400866f3: 48 89 cb                    	movq	%rcx, %rbx
1400866f6: 0f 10 85 b0 01 00 00        	movups	0x1b0(%rbp), %xmm0
1400866fd: 0f 11 81 e8 00 00 00        	movups	%xmm0, 0xe8(%rcx)
140086704: c6 81 f8 00 00 00 04        	movb	$0x4, 0xf8(%rcx)
14008670b: 8b 85 17 02 00 00           	movl	0x217(%rbp), %eax
140086711: 89 81 fc 00 00 00           	movl	%eax, 0xfc(%rcx)
140086717: 8b 85 14 02 00 00           	movl	0x214(%rbp), %eax
14008671d: 89 81 f9 00 00 00           	movl	%eax, 0xf9(%rcx)
140086723: 48 8b 85 e0 01 00 00        	movq	0x1e0(%rbp), %rax
14008672a: 48 89 81 18 01 00 00        	movq	%rax, 0x118(%rcx)
140086731: 48 8b 85 e8 01 00 00        	movq	0x1e8(%rbp), %rax
140086738: 48 89 81 20 01 00 00        	movq	%rax, 0x120(%rcx)
14008673f: c6 81 38 01 00 00 04        	movb	$0x4, 0x138(%rcx)
140086746: 48 c7 45 70 05 00 00 00     	movq	$0x5, 0x70(%rbp)
14008674e: 48 89 4d 78                 	movq	%rcx, 0x78(%rbp)
140086752: 48 c7 85 80 00 00 00 05 00 00 00    	movq	$0x5, 0x80(%rbp)
14008675d: 0f b6 05 a5 e4 5a 03        	movzbl	0x35ae4a5(%rip), %eax   # 0x143634c09
140086764: b9 40 00 00 00              	movl	$0x40, %ecx
140086769: ba 08 00 00 00              	movl	$0x8, %edx
14008676e: e8 9d d4 58 00              	callq	0x140613c10 <.text+0x612c10>
140086773: 48 85 c0                    	testq	%rax, %rax
140086776: 0f 84 d8 0b 00 00           	je	0x140087354 <.text+0x86354>
14008677c: 48 89 c7                    	movq	%rax, %rdi
14008677f: 0f b6 05 83 e4 5a 03        	movzbl	0x35ae483(%rip), %eax   # 0x143634c09
140086786: b9 01 00 00 00              	movl	$0x1, %ecx
14008678b: ba 01 00 00 00              	movl	$0x1, %edx
140086790: e8 7b d4 58 00              	callq	0x140613c10 <.text+0x612c10>
140086795: 48 85 c0                    	testq	%rax, %rax
140086798: 48 8b b5 c0 00 00 00        	movq	0xc0(%rbp), %rsi
14008679f: 0f 84 e3 0b 00 00           	je	0x140087388 <.text+0x86388>
1400867a5: c6 00 24                    	movb	$0x24, (%rax)
1400867a8: 48 c7 07 01 00 00 00        	movq	$0x1, (%rdi)
1400867af: 48 89 47 08                 	movq	%rax, 0x8(%rdi)
1400867b3: 48 c7 47 10 01 00 00 00     	movq	$0x1, 0x10(%rdi)
1400867bb: 0f 10 85 b0 01 00 00        	movups	0x1b0(%rbp), %xmm0
1400867c2: 0f 11 47 28                 	movups	%xmm0, 0x28(%rdi)
1400867c6: 8b 85 14 02 00 00           	movl	0x214(%rbp), %eax
1400867cc: 8b 8d 17 02 00 00           	movl	0x217(%rbp), %ecx
1400867d2: 89 47 39                    	movl	%eax, 0x39(%rdi)
1400867d5: 89 4f 3c                    	movl	%ecx, 0x3c(%rdi)
1400867d8: 48 8d 05 c0 aa 8a 02        	leaq	0x28aaac0(%rip), %rax   # 0x14293129f
1400867df: 48 89 47 18                 	movq	%rax, 0x18(%rdi)
1400867e3: 48 c7 47 20 12 00 00 00     	movq	$0x12, 0x20(%rdi)
1400867eb: c6 47 38 04                 	movb	$0x4, 0x38(%rdi)
1400867ef: 48 c7 45 58 01 00 00 00     	movq	$0x1, 0x58(%rbp)
1400867f7: 48 89 7d 60                 	movq	%rdi, 0x60(%rbp)
1400867fb: 48 c7 45 68 01 00 00 00     	movq	$0x1, 0x68(%rbp)
140086803: 48 c7 45 30 00 00 00 00     	movq	$0x0, 0x30(%rbp)
14008680b: 48 8b 06                    	movq	(%rsi), %rax
14008680e: 48 8d 0d df ac 8a 02        	leaq	0x28aacdf(%rip), %rcx   # 0x1429314f4
140086815: 48 63 04 81                 	movslq	(%rcx,%rax,4), %rax
140086819: 48 01 c8                    	addq	%rcx, %rax
14008681c: ff e0                       	jmpq	*%rax
14008681e: 4c 8b b5 00 01 00 00        	movq	0x100(%rbp), %r14
140086825: 48 8b 9d 08 01 00 00        	movq	0x108(%rbp), %rbx
14008682c: b0 01                       	movb	$0x1, %al
14008682e: 89 85 a8 01 00 00           	movl	%eax, 0x1a8(%rbp)
140086834: 48 8b 85 10 01 00 00        	movq	0x110(%rbp), %rax
14008683b: c7 85 e0 01 00 00 00 00 00 00       	movl	$0x0, 0x1e0(%rbp)
140086845: b1 01                       	movb	$0x1, %cl
140086847: 89 8d d0 01 00 00           	movl	%ecx, 0x1d0(%rbp)
14008684d: b1 01                       	movb	$0x1, %cl
14008684f: 89 8d c8 01 00 00           	movl	%ecx, 0x1c8(%rbp)
140086855: b1 01                       	movb	$0x1, %cl
140086857: 89 8d c0 01 00 00           	movl	%ecx, 0x1c0(%rbp)
14008685d: b1 01                       	movb	$0x1, %cl
14008685f: 89 8d e8 01 00 00           	movl	%ecx, 0x1e8(%rbp)
140086865: e9 2f 01 00 00              	jmp	0x140086999 <.text+0x85999>
14008686a: b0 01                       	movb	$0x1, %al
14008686c: 89 85 a8 01 00 00           	movl	%eax, 0x1a8(%rbp)
140086872: 41 be 05 00 00 00           	movl	$0x5, %r14d
140086878: c7 85 d0 01 00 00 00 00 00 00       	movl	$0x0, 0x1d0(%rbp)
140086882: b8 05 00 00 00              	movl	$0x5, %eax
140086887: e9 ed 00 00 00              	jmp	0x140086979 <.text+0x85979>
14008688c: 4c 8b b5 78 01 00 00        	movq	0x178(%rbp), %r14
140086893: 48 8b 9d 80 01 00 00        	movq	0x180(%rbp), %rbx
14008689a: b0 01                       	movb	$0x1, %al
14008689c: 89 85 a8 01 00 00           	movl	%eax, 0x1a8(%rbp)
1400868a2: 48 8b 85 88 01 00 00        	movq	0x188(%rbp), %rax
1400868a9: c7 85 c0 01 00 00 00 00 00 00       	movl	$0x0, 0x1c0(%rbp)
1400868b3: b1 01                       	movb	$0x1, %cl
1400868b5: 89 8d d0 01 00 00           	movl	%ecx, 0x1d0(%rbp)
1400868bb: b1 01                       	movb	$0x1, %cl
1400868bd: 89 8d c8 01 00 00           	movl	%ecx, 0x1c8(%rbp)
1400868c3: e9 c1 00 00 00              	jmp	0x140086989 <.text+0x85989>
1400868c8: 4c 8b b5 d0 00 00 00        	movq	0xd0(%rbp), %r14
1400868cf: 48 8b 9d d8 00 00 00        	movq	0xd8(%rbp), %rbx
1400868d6: b0 01                       	movb	$0x1, %al
1400868d8: 89 85 a8 01 00 00           	movl	%eax, 0x1a8(%rbp)
1400868de: 48 8b 85 e0 00 00 00        	movq	0xe0(%rbp), %rax
1400868e5: c7 85 c8 01 00 00 00 00 00 00       	movl	$0x0, 0x1c8(%rbp)
1400868ef: b1 01                       	movb	$0x1, %cl
1400868f1: 89 8d d0 01 00 00           	movl	%ecx, 0x1d0(%rbp)
1400868f7: e9 85 00 00 00              	jmp	0x140086981 <.text+0x85981>
1400868fc: 4c 8b b5 e8 00 00 00        	movq	0xe8(%rbp), %r14
140086903: 48 8b 9d f0 00 00 00        	movq	0xf0(%rbp), %rbx
14008690a: b0 01                       	movb	$0x1, %al
14008690c: 89 85 a8 01 00 00           	movl	%eax, 0x1a8(%rbp)
140086912: 48 8b 85 f8 00 00 00        	movq	0xf8(%rbp), %rax
140086919: c7 85 e8 01 00 00 00 00 00 00       	movl	$0x0, 0x1e8(%rbp)
140086923: b1 01                       	movb	$0x1, %cl
140086925: 89 8d d0 01 00 00           	movl	%ecx, 0x1d0(%rbp)
14008692b: b1 01                       	movb	$0x1, %cl
14008692d: 89 8d c8 01 00 00           	movl	%ecx, 0x1c8(%rbp)
140086933: b1 01                       	movb	$0x1, %cl
140086935: 89 8d c0 01 00 00           	movl	%ecx, 0x1c0(%rbp)
14008693b: eb 54                       	jmp	0x140086991 <.text+0x85991>
14008693d: b0 01                       	movb	$0x1, %al
14008693f: 89 85 d0 01 00 00           	movl	%eax, 0x1d0(%rbp)
140086945: bb 08 00 00 00              	movl	$0x8, %ebx
14008694a: 45 31 f6                    	xorl	%r14d, %r14d
14008694d: 31 c0                       	xorl	%eax, %eax
14008694f: b1 01                       	movb	$0x1, %cl
140086951: 89 8d a8 01 00 00           	movl	%ecx, 0x1a8(%rbp)
140086957: eb 20                       	jmp	0x140086979 <.text+0x85979>
140086959: b0 01                       	movb	$0x1, %al
14008695b: 89 85 d0 01 00 00           	movl	%eax, 0x1d0(%rbp)
140086961: 41 be 01 00 00 00           	movl	$0x1, %r14d
140086967: c7 85 a8 01 00 00 00 00 00 00       	movl	$0x0, 0x1a8(%rbp)
140086971: 48 89 fb                    	movq	%rdi, %rbx
140086974: b8 01 00 00 00              	movl	$0x1, %eax
140086979: b1 01                       	movb	$0x1, %cl
14008697b: 89 8d c8 01 00 00           	movl	%ecx, 0x1c8(%rbp)
140086981: b1 01                       	movb	$0x1, %cl
140086983: 89 8d c0 01 00 00           	movl	%ecx, 0x1c0(%rbp)
140086989: b1 01                       	movb	$0x1, %cl
14008698b: 89 8d e8 01 00 00           	movl	%ecx, 0x1e8(%rbp)
140086991: b1 01                       	movb	$0x1, %cl
140086993: 89 8d e0 01 00 00           	movl	%ecx, 0x1e0(%rbp)
140086999: 48 89 c7                    	movq	%rax, %rdi
14008699c: 48 c1 e7 06                 	shlq	$0x6, %rdi
1400869a0: 48 01 df                    	addq	%rbx, %rdi
1400869a3: 48 89 9d a0 00 00 00        	movq	%rbx, 0xa0(%rbp)
1400869aa: 48 89 9d a8 00 00 00        	movq	%rbx, 0xa8(%rbp)
1400869b1: 4c 89 b5 b0 00 00 00        	movq	%r14, 0xb0(%rbp)
1400869b8: 48 89 bd b8 00 00 00        	movq	%rdi, 0xb8(%rbp)
1400869bf: 49 89 df                    	movq	%rbx, %r15
1400869c2: 48 85 c0                    	testq	%rax, %rax
1400869c5: 0f 84 83 02 00 00           	je	0x140086c4e <.text+0x85c4e>
1400869cb: 4c 89 b5 70 01 00 00        	movq	%r14, 0x170(%rbp)
1400869d2: 48 8d 46 08                 	leaq	0x8(%rsi), %rax
1400869d6: 48 89 85 a0 01 00 00        	movq	%rax, 0x1a0(%rbp)
1400869dd: 48 8d 46 10                 	leaq	0x10(%rsi), %rax
1400869e1: 48 89 85 90 01 00 00        	movq	%rax, 0x190(%rbp)
1400869e8: 4c 8d 6e 1c                 	leaq	0x1c(%rsi), %r13
1400869ec: 48 89 9d 98 01 00 00        	movq	%rbx, 0x198(%rbp)
1400869f3: 48 8d 43 40                 	leaq	0x40(%rbx), %rax
1400869f7: 48 8d 75 08                 	leaq	0x8(%rbp), %rsi
1400869fb: 48 8d 9d 18 01 00 00        	leaq	0x118(%rbp), %rbx
140086a02: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140086a10: 49 89 c7                    	movq	%rax, %r15
140086a13: 48 8b 40 c0                 	movq	-0x40(%rax), %rax
140086a17: 48 89 85 f8 01 00 00        	movq	%rax, 0x1f8(%rbp)
140086a1e: 48 f7 d8                    	negq	%rax
140086a21: 0f 80 0b 02 00 00           	jo	0x140086c32 <.text+0x85c32>
140086a27: 4d 8d 77 c0                 	leaq	-0x40(%r15), %r14
140086a2b: 49 8b 47 c8                 	movq	-0x38(%r15), %rax
140086a2f: 48 89 85 00 02 00 00        	movq	%rax, 0x200(%rbp)
140086a36: 4c 89 bd f0 01 00 00        	movq	%r15, 0x1f0(%rbp)
140086a3d: 4d 8b 7f d0                 	movq	-0x30(%r15), %r15
140086a41: 49 8b 46 18                 	movq	0x18(%r14), %rax
140086a45: 49 8b 4e 20                 	movq	0x20(%r14), %rcx
140086a49: 48 89 45 f8                 	movq	%rax, -0x8(%rbp)
140086a4d: 48 89 4d 00                 	movq	%rcx, (%rbp)
140086a51: 49 8b 56 38                 	movq	0x38(%r14), %rdx
140086a55: 48 89 55 c0                 	movq	%rdx, -0x40(%rbp)
140086a59: 41 0f 10 46 28              	movups	0x28(%r14), %xmm0
140086a5e: 0f 29 45 b0                 	movaps	%xmm0, -0x50(%rbp)
140086a62: 48 8b 95 d8 01 00 00        	movq	0x1d8(%rbp), %rdx
140086a69: 4c 8b 62 08                 	movq	0x8(%rdx), %r12
140086a6d: 48 89 45 08                 	movq	%rax, 0x8(%rbp)
140086a71: 48 89 4d 10                 	movq	%rcx, 0x10(%rbp)
140086a75: c6 85 1b 02 00 00 01        	movb	$0x1, 0x21b(%rbp)
140086a7c: 48 89 f1                    	movq	%rsi, %rcx
140086a7f: e8 8c d1 5f 00              	callq	0x140683c10 <.text+0x682c10>
140086a84: 48 89 85 18 01 00 00        	movq	%rax, 0x118(%rbp)
140086a8b: 48 b8 3b 81 28 ff 69 31 22 41       	movabsq	$0x41223169ff28813b, %rax # imm = 0x41223169FF28813B
140086a95: 48 89 85 20 01 00 00        	movq	%rax, 0x120(%rbp)
140086a9c: 48 b8 d9 68 a9 a2 68 72 9b a7       	movabsq	$-0x58648d975d569727, %rax # imm = 0xA79B7268A2A968D9
140086aa6: 48 89 85 28 01 00 00        	movq	%rax, 0x128(%rbp)
140086aad: c6 85 1b 02 00 00 01        	movb	$0x1, 0x21b(%rbp)
140086ab4: 48 89 d9                    	movq	%rbx, %rcx
140086ab7: e8 64 74 53 00              	callq	0x1405bdf20 <.text+0x5bcf20>
140086abc: 4c 89 a5 18 01 00 00        	movq	%r12, 0x118(%rbp)
140086ac3: 48 89 85 20 01 00 00        	movq	%rax, 0x120(%rbp)
140086aca: c6 85 1b 02 00 00 01        	movb	$0x1, 0x21b(%rbp)
140086ad1: 48 89 d9                    	movq	%rbx, %rcx
140086ad4: e8 37 6e 53 00              	callq	0x1405bd910 <.text+0x5bc910>
140086ad9: c6 85 1b 02 00 00 01        	movb	$0x1, 0x21b(%rbp)
140086ae0: 48 89 d9                    	movq	%rbx, %rcx
140086ae3: 48 8b 95 d8 01 00 00        	movq	0x1d8(%rbp), %rdx
140086aea: 49 89 c0                    	movq	%rax, %r8
140086aed: e8 be f6 5f 00              	callq	0x1406861b0 <.text+0x6851b0>
140086af2: c6 85 1b 02 00 00 01        	movb	$0x1, 0x21b(%rbp)
140086af9: 41 b9 16 00 00 00           	movl	$0x16, %r9d
140086aff: 48 89 f1                    	movq	%rsi, %rcx
140086b02: 48 89 da                    	movq	%rbx, %rdx
140086b05: 4c 8d 05 c0 a7 8a 02        	leaq	0x28aa7c0(%rip), %r8    # 0x1429312cc
140086b0c: e8 cf 53 60 00              	callq	0x14068bee0 <.text+0x68aee0>
140086b11: 49 83 ff 01                 	cmpq	$0x1, %r15
140086b15: 75 49                       	jne	0x140086b60 <.text+0x85b60>
140086b17: c6 85 1b 02 00 00 01        	movb	$0x1, 0x21b(%rbp)
140086b1e: 48 89 d9                    	movq	%rbx, %rcx
140086b21: 48 8b 95 00 02 00 00        	movq	0x200(%rbp), %rdx
140086b28: e8 33 60 81 00              	callq	0x14089cb60 <.text+0x89bb60>
140086b2d: 0f 10 85 30 01 00 00        	movups	0x130(%rbp), %xmm0
140086b34: 0f 11 45 e8                 	movups	%xmm0, -0x18(%rbp)
140086b38: 48 8b 95 18 01 00 00        	movq	0x118(%rbp), %rdx
140086b3f: 48 89 d0                    	movq	%rdx, %rax
140086b42: 48 f7 d8                    	negq	%rax
140086b45: 70 21                       	jo	0x140086b68 <.text+0x85b68>
140086b47: 73 1f                       	jae	0x140086b68 <.text+0x85b68>
140086b49: 48 8b 8d 20 01 00 00        	movq	0x120(%rbp), %rcx
140086b50: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140086b56: e8 c5 d0 58 00              	callq	0x140613c20 <.text+0x612c20>
140086b5b: eb 0b                       	jmp	0x140086b68 <.text+0x85b68>
140086b5d: 0f 1f 00                    	nopl	(%rax)
140086b60: 48 c7 45 e8 00 00 00 00     	movq	$0x0, -0x18(%rbp)
140086b68: c6 85 1b 02 00 00 01        	movb	$0x1, 0x21b(%rbp)
140086b6f: 48 8d 45 f8                 	leaq	-0x8(%rbp), %rax
140086b73: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140086b78: 41 b8 06 00 00 00           	movl	$0x6, %r8d
140086b7e: 48 89 f1                    	movq	%rsi, %rcx
140086b81: 48 8d 15 5a a7 8a 02        	leaq	0x28aa75a(%rip), %rdx   # 0x1429312e2
140086b88: 4c 8d 4d e8                 	leaq	-0x18(%rbp), %r9
140086b8c: e8 6f c1 47 00              	callq	0x140502d00 <.text+0x501d00>
140086b91: 48 8b 85 f8 01 00 00        	movq	0x1f8(%rbp), %rax
140086b98: 48 89 85 18 01 00 00        	movq	%rax, 0x118(%rbp)
140086b9f: 48 8b 85 00 02 00 00        	movq	0x200(%rbp), %rax
140086ba6: 48 89 85 20 01 00 00        	movq	%rax, 0x120(%rbp)
140086bad: 4c 89 bd 28 01 00 00        	movq	%r15, 0x128(%rbp)
140086bb4: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
140086bbb: 48 89 85 30 01 00 00        	movq	%rax, 0x130(%rbp)
140086bc2: 48 8d 45 30                 	leaq	0x30(%rbp), %rax
140086bc6: 48 89 85 38 01 00 00        	movq	%rax, 0x138(%rbp)
140086bcd: 48 8b 85 a0 01 00 00        	movq	0x1a0(%rbp), %rax
140086bd4: 48 89 85 40 01 00 00        	movq	%rax, 0x140(%rbp)
140086bdb: 48 8b 85 90 01 00 00        	movq	0x190(%rbp), %rax
140086be2: 48 89 85 48 01 00 00        	movq	%rax, 0x148(%rbp)
140086be9: 4c 89 ad 50 01 00 00        	movq	%r13, 0x150(%rbp)
140086bf0: 48 8d 45 b0                 	leaq	-0x50(%rbp), %rax
140086bf4: 48 89 85 58 01 00 00        	movq	%rax, 0x158(%rbp)
140086bfb: c6 85 1b 02 00 00 00        	movb	$0x0, 0x21b(%rbp)
140086c02: 41 b8 03 00 00 00           	movl	$0x3, %r8d
140086c08: 48 89 f1                    	movq	%rsi, %rcx
140086c0b: 48 8d 15 d6 a6 8a 02        	leaq	0x28aa6d6(%rip), %rdx   # 0x1429312e8
140086c12: 49 89 d9                    	movq	%rbx, %r9
140086c15: e8 d6 b4 47 00              	callq	0x1405020f0 <.text+0x5010f0>
140086c1a: 4c 8b bd f0 01 00 00        	movq	0x1f0(%rbp), %r15
140086c21: 49 8d 47 40                 	leaq	0x40(%r15), %rax
140086c25: 49 83 c6 40                 	addq	$0x40, %r14
140086c29: 49 39 fe                    	cmpq	%rdi, %r14
140086c2c: 0f 85 de fd ff ff           	jne	0x140086a10 <.text+0x85a10>
140086c32: 4c 89 bd a8 00 00 00        	movq	%r15, 0xa8(%rbp)
140086c39: 48 8b b5 c0 00 00 00        	movq	0xc0(%rbp), %rsi
140086c40: 48 8b 9d 98 01 00 00        	movq	0x198(%rbp), %rbx
140086c47: 4c 8b b5 70 01 00 00        	movq	0x170(%rbp), %r14
140086c4e: 4c 29 ff                    	subq	%r15, %rdi
140086c51: 74 2f                       	je	0x140086c82 <.text+0x85c82>
140086c53: 48 c1 ef 06                 	shrq	$0x6, %rdi
140086c57: 49 83 c7 08                 	addq	$0x8, %r15
140086c5b: eb 0c                       	jmp	0x140086c69 <.text+0x85c69>
140086c5d: 0f 1f 00                    	nopl	(%rax)
140086c60: 49 83 c7 40                 	addq	$0x40, %r15
140086c64: 48 ff cf                    	decq	%rdi
140086c67: 74 19                       	je	0x140086c82 <.text+0x85c82>
140086c69: 49 8b 57 f8                 	movq	-0x8(%r15), %rdx
140086c6d: 48 85 d2                    	testq	%rdx, %rdx
140086c70: 74 ee                       	je	0x140086c60 <.text+0x85c60>
140086c72: 49 8b 0f                    	movq	(%r15), %rcx
140086c75: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140086c7b: e8 a0 cf 58 00              	callq	0x140613c20 <.text+0x612c20>
140086c80: eb de                       	jmp	0x140086c60 <.text+0x85c60>
140086c82: 4d 85 f6                    	testq	%r14, %r14
140086c85: 74 15                       	je	0x140086c9c <.text+0x85c9c>
140086c87: 49 c1 e6 06                 	shlq	$0x6, %r14
140086c8b: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140086c91: 48 89 d9                    	movq	%rbx, %rcx
140086c94: 4c 89 f2                    	movq	%r14, %rdx
140086c97: e8 84 cf 58 00              	callq	0x140613c20 <.text+0x612c20>
140086c9c: 83 3e 05                    	cmpl	$0x5, (%rsi)
140086c9f: 75 5f                       	jne	0x140086d00 <.text+0x85d00>
140086ca1: 48 8d 0d 09 a6 8a 02        	leaq	0x28aa609(%rip), %rcx   # 0x1429312b1
140086ca8: ba 1b 00 00 00              	movl	$0x1b, %edx
140086cad: e8 ce 67 b1 00              	callq	0x140b9d480 <.text+0xb9c480>
140086cb2: 49 89 d1                    	movq	%rdx, %r9
140086cb5: 48 8d 8d 18 01 00 00        	leaq	0x118(%rbp), %rcx
140086cbc: 48 8b 95 d8 01 00 00        	movq	0x1d8(%rbp), %rdx
140086cc3: 49 89 c0                    	movq	%rax, %r8
140086cc6: e8 f5 52 60 00              	callq	0x14068bfc0 <.text+0x68afc0>
140086ccb: 48 8d 4d 08                 	leaq	0x8(%rbp), %rcx
140086ccf: 48 8d 95 18 01 00 00        	leaq	0x118(%rbp), %rdx
140086cd6: f3 0f 10 15 72 a0 8a 02     	movss	0x28aa072(%rip), %xmm2  # 0x142930d50
140086cde: e8 2d 7c 5f 00              	callq	0x14067e910 <.text+0x67d910>
140086ce3: 48 8d 4d 08                 	leaq	0x8(%rbp), %rcx
140086ce7: e8 64 b0 5f 00              	callq	0x140681d50 <.text+0x680d50>
140086cec: 48 89 c1                    	movq	%rax, %rcx
140086cef: e8 8c cf 5f 00              	callq	0x140683c80 <.text+0x682c80>
140086cf4: 84 c0                       	testb	%al, %al
140086cf6: 74 08                       	je	0x140086d00 <.text+0x85d00>
140086cf8: 48 c7 46 08 02 00 00 00     	movq	$0x2, 0x8(%rsi)
140086d00: 80 bd a8 01 00 00 00        	cmpb	$0x0, 0x1a8(%rbp)
140086d07: 74 54                       	je	0x140086d5d <.text+0x85d5d>
140086d09: 48 8b 75 60                 	movq	0x60(%rbp), %rsi
140086d0d: 48 8b 7d 68                 	movq	0x68(%rbp), %rdi
140086d11: 48 85 ff                    	testq	%rdi, %rdi
140086d14: 74 2c                       	je	0x140086d42 <.text+0x85d42>
140086d16: 48 8d 5e 08                 	leaq	0x8(%rsi), %rbx
140086d1a: eb 0d                       	jmp	0x140086d29 <.text+0x85d29>
140086d1c: 0f 1f 40 00                 	nopl	(%rax)
140086d20: 48 83 c3 40                 	addq	$0x40, %rbx
140086d24: 48 ff cf                    	decq	%rdi
140086d27: 74 19                       	je	0x140086d42 <.text+0x85d42>
140086d29: 48 8b 53 f8                 	movq	-0x8(%rbx), %rdx
140086d2d: 48 85 d2                    	testq	%rdx, %rdx
140086d30: 74 ee                       	je	0x140086d20 <.text+0x85d20>
140086d32: 48 8b 0b                    	movq	(%rbx), %rcx
140086d35: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140086d3b: e8 e0 ce 58 00              	callq	0x140613c20 <.text+0x612c20>
140086d40: eb de                       	jmp	0x140086d20 <.text+0x85d20>
140086d42: 48 8b 55 58                 	movq	0x58(%rbp), %rdx
140086d46: 48 85 d2                    	testq	%rdx, %rdx
140086d49: 74 12                       	je	0x140086d5d <.text+0x85d5d>
140086d4b: 48 c1 e2 06                 	shlq	$0x6, %rdx
140086d4f: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140086d55: 48 89 f1                    	movq	%rsi, %rcx
140086d58: e8 c3 ce 58 00              	callq	0x140613c20 <.text+0x612c20>
140086d5d: 80 bd d0 01 00 00 00        	cmpb	$0x0, 0x1d0(%rbp)
140086d64: 74 57                       	je	0x140086dbd <.text+0x85dbd>
140086d66: 48 8b 75 78                 	movq	0x78(%rbp), %rsi
140086d6a: 48 8b bd 80 00 00 00        	movq	0x80(%rbp), %rdi
140086d71: 48 85 ff                    	testq	%rdi, %rdi
140086d74: 74 2c                       	je	0x140086da2 <.text+0x85da2>
140086d76: 48 8d 5e 08                 	leaq	0x8(%rsi), %rbx
140086d7a: eb 0d                       	jmp	0x140086d89 <.text+0x85d89>
140086d7c: 0f 1f 40 00                 	nopl	(%rax)
140086d80: 48 83 c3 40                 	addq	$0x40, %rbx
140086d84: 48 ff cf                    	decq	%rdi
140086d87: 74 19                       	je	0x140086da2 <.text+0x85da2>
140086d89: 48 8b 53 f8                 	movq	-0x8(%rbx), %rdx
140086d8d: 48 85 d2                    	testq	%rdx, %rdx
140086d90: 74 ee                       	je	0x140086d80 <.text+0x85d80>
140086d92: 48 8b 0b                    	movq	(%rbx), %rcx
140086d95: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140086d9b: e8 80 ce 58 00              	callq	0x140613c20 <.text+0x612c20>
140086da0: eb de                       	jmp	0x140086d80 <.text+0x85d80>
140086da2: 48 8b 55 70                 	movq	0x70(%rbp), %rdx
140086da6: 48 85 d2                    	testq	%rdx, %rdx
140086da9: 74 12                       	je	0x140086dbd <.text+0x85dbd>
140086dab: 48 c1 e2 06                 	shlq	$0x6, %rdx
140086daf: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140086db5: 48 89 f1                    	movq	%rsi, %rcx
140086db8: e8 63 ce 58 00              	callq	0x140613c20 <.text+0x612c20>
140086dbd: 80 bd c8 01 00 00 00        	cmpb	$0x0, 0x1c8(%rbp)
140086dc4: 74 5a                       	je	0x140086e20 <.text+0x85e20>
140086dc6: 48 8b b5 d8 00 00 00        	movq	0xd8(%rbp), %rsi
140086dcd: 48 8b bd e0 00 00 00        	movq	0xe0(%rbp), %rdi
140086dd4: 48 85 ff                    	testq	%rdi, %rdi
140086dd7: 74 29                       	je	0x140086e02 <.text+0x85e02>
140086dd9: 48 8d 5e 08                 	leaq	0x8(%rsi), %rbx
140086ddd: eb 0a                       	jmp	0x140086de9 <.text+0x85de9>
140086ddf: 90                          	nop
140086de0: 48 83 c3 40                 	addq	$0x40, %rbx
140086de4: 48 ff cf                    	decq	%rdi
140086de7: 74 19                       	je	0x140086e02 <.text+0x85e02>
140086de9: 48 8b 53 f8                 	movq	-0x8(%rbx), %rdx
140086ded: 48 85 d2                    	testq	%rdx, %rdx
140086df0: 74 ee                       	je	0x140086de0 <.text+0x85de0>
140086df2: 48 8b 0b                    	movq	(%rbx), %rcx
140086df5: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140086dfb: e8 20 ce 58 00              	callq	0x140613c20 <.text+0x612c20>
140086e00: eb de                       	jmp	0x140086de0 <.text+0x85de0>
140086e02: 48 8b 95 d0 00 00 00        	movq	0xd0(%rbp), %rdx
140086e09: 48 85 d2                    	testq	%rdx, %rdx
140086e0c: 74 12                       	je	0x140086e20 <.text+0x85e20>
140086e0e: 48 c1 e2 06                 	shlq	$0x6, %rdx
140086e12: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140086e18: 48 89 f1                    	movq	%rsi, %rcx
140086e1b: e8 00 ce 58 00              	callq	0x140613c20 <.text+0x612c20>
140086e20: 80 bd c0 01 00 00 00        	cmpb	$0x0, 0x1c0(%rbp)
140086e27: 74 67                       	je	0x140086e90 <.text+0x85e90>
140086e29: 48 8b b5 80 01 00 00        	movq	0x180(%rbp), %rsi
140086e30: 48 8b bd 88 01 00 00        	movq	0x188(%rbp), %rdi
140086e37: 48 85 ff                    	testq	%rdi, %rdi
140086e3a: 74 36                       	je	0x140086e72 <.text+0x85e72>
140086e3c: 48 8d 5e 08                 	leaq	0x8(%rsi), %rbx
140086e40: eb 17                       	jmp	0x140086e59 <.text+0x85e59>
140086e42: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140086e50: 48 83 c3 40                 	addq	$0x40, %rbx
140086e54: 48 ff cf                    	decq	%rdi
140086e57: 74 19                       	je	0x140086e72 <.text+0x85e72>
140086e59: 48 8b 53 f8                 	movq	-0x8(%rbx), %rdx
140086e5d: 48 85 d2                    	testq	%rdx, %rdx
140086e60: 74 ee                       	je	0x140086e50 <.text+0x85e50>
140086e62: 48 8b 0b                    	movq	(%rbx), %rcx
140086e65: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140086e6b: e8 b0 cd 58 00              	callq	0x140613c20 <.text+0x612c20>
140086e70: eb de                       	jmp	0x140086e50 <.text+0x85e50>
140086e72: 48 8b 95 78 01 00 00        	movq	0x178(%rbp), %rdx
140086e79: 48 85 d2                    	testq	%rdx, %rdx
140086e7c: 74 12                       	je	0x140086e90 <.text+0x85e90>
140086e7e: 48 c1 e2 06                 	shlq	$0x6, %rdx
140086e82: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140086e88: 48 89 f1                    	movq	%rsi, %rcx
140086e8b: e8 90 cd 58 00              	callq	0x140613c20 <.text+0x612c20>
140086e90: 80 bd e8 01 00 00 00        	cmpb	$0x0, 0x1e8(%rbp)
140086e97: 74 67                       	je	0x140086f00 <.text+0x85f00>
140086e99: 48 8b b5 f0 00 00 00        	movq	0xf0(%rbp), %rsi
140086ea0: 48 8b bd f8 00 00 00        	movq	0xf8(%rbp), %rdi
140086ea7: 48 85 ff                    	testq	%rdi, %rdi
140086eaa: 74 36                       	je	0x140086ee2 <.text+0x85ee2>
140086eac: 48 8d 5e 08                 	leaq	0x8(%rsi), %rbx
140086eb0: eb 17                       	jmp	0x140086ec9 <.text+0x85ec9>
140086eb2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140086ec0: 48 83 c3 40                 	addq	$0x40, %rbx
140086ec4: 48 ff cf                    	decq	%rdi
140086ec7: 74 19                       	je	0x140086ee2 <.text+0x85ee2>
140086ec9: 48 8b 53 f8                 	movq	-0x8(%rbx), %rdx
140086ecd: 48 85 d2                    	testq	%rdx, %rdx
140086ed0: 74 ee                       	je	0x140086ec0 <.text+0x85ec0>
140086ed2: 48 8b 0b                    	movq	(%rbx), %rcx
140086ed5: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140086edb: e8 40 cd 58 00              	callq	0x140613c20 <.text+0x612c20>
140086ee0: eb de                       	jmp	0x140086ec0 <.text+0x85ec0>
140086ee2: 48 8b 95 e8 00 00 00        	movq	0xe8(%rbp), %rdx
140086ee9: 48 85 d2                    	testq	%rdx, %rdx
140086eec: 74 12                       	je	0x140086f00 <.text+0x85f00>
140086eee: 48 c1 e2 06                 	shlq	$0x6, %rdx
140086ef2: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140086ef8: 48 89 f1                    	movq	%rsi, %rcx
140086efb: e8 20 cd 58 00              	callq	0x140613c20 <.text+0x612c20>
140086f00: 80 bd e0 01 00 00 00        	cmpb	$0x0, 0x1e0(%rbp)
140086f07: 74 67                       	je	0x140086f70 <.text+0x85f70>
140086f09: 48 8b b5 08 01 00 00        	movq	0x108(%rbp), %rsi
140086f10: 48 8b bd 10 01 00 00        	movq	0x110(%rbp), %rdi
140086f17: 48 85 ff                    	testq	%rdi, %rdi
140086f1a: 74 36                       	je	0x140086f52 <.text+0x85f52>
140086f1c: 48 8d 5e 08                 	leaq	0x8(%rsi), %rbx
140086f20: eb 17                       	jmp	0x140086f39 <.text+0x85f39>
140086f22: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140086f30: 48 83 c3 40                 	addq	$0x40, %rbx
140086f34: 48 ff cf                    	decq	%rdi
140086f37: 74 19                       	je	0x140086f52 <.text+0x85f52>
140086f39: 48 8b 53 f8                 	movq	-0x8(%rbx), %rdx
140086f3d: 48 85 d2                    	testq	%rdx, %rdx
140086f40: 74 ee                       	je	0x140086f30 <.text+0x85f30>
140086f42: 48 8b 0b                    	movq	(%rbx), %rcx
140086f45: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140086f4b: e8 d0 cc 58 00              	callq	0x140613c20 <.text+0x612c20>
140086f50: eb de                       	jmp	0x140086f30 <.text+0x85f30>
140086f52: 48 8b 95 00 01 00 00        	movq	0x100(%rbp), %rdx
140086f59: 48 85 d2                    	testq	%rdx, %rdx
140086f5c: 74 12                       	je	0x140086f70 <.text+0x85f70>
140086f5e: 48 c1 e2 06                 	shlq	$0x6, %rdx
140086f62: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140086f68: 48 89 f1                    	movq	%rsi, %rcx
140086f6b: e8 b0 cc 58 00              	callq	0x140613c20 <.text+0x612c20>
140086f70: 48 8b 95 c8 00 00 00        	movq	0xc8(%rbp), %rdx
140086f77: 48 85 d2                    	testq	%rdx, %rdx
140086f7a: 74 12                       	je	0x140086f8e <.text+0x85f8e>
140086f7c: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140086f82: 48 8b 8d 68 01 00 00        	movq	0x168(%rbp), %rcx
140086f89: e8 92 cc 58 00              	callq	0x140613c20 <.text+0x612c20>
140086f8e: 48 8b 95 88 00 00 00        	movq	0x88(%rbp), %rdx
140086f95: 48 85 d2                    	testq	%rdx, %rdx
140086f98: 74 12                       	je	0x140086fac <.text+0x85fac>
140086f9a: 48 8b 8d 90 00 00 00        	movq	0x90(%rbp), %rcx
140086fa1: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140086fa7: e8 74 cc 58 00              	callq	0x140613c20 <.text+0x612c20>
140086fac: 90                          	nop
140086fad: 48 81 c4 a8 02 00 00        	addq	$0x2a8, %rsp            # imm = 0x2A8
140086fb4: 5b                          	popq	%rbx
140086fb5: 5f                          	popq	%rdi
140086fb6: 5e                          	popq	%rsi
140086fb7: 41 5c                       	popq	%r12
140086fb9: 41 5d                       	popq	%r13
140086fbb: 41 5e                       	popq	%r14
140086fbd: 41 5f                       	popq	%r15
140086fbf: 5d                          	popq	%rbp
140086fc0: c3                          	retq
140086fc1: 4c 8d 05 18 a2 8a 02        	leaq	0x28aa218(%rip), %r8    # 0x1429311e0
140086fc8: b9 02 00 00 00              	movl	$0x2, %ecx
140086fcd: 48 89 f2                    	movq	%rsi, %rdx
140086fd0: e8 ac 20 85 02              	callq	0x1428d9081 <.text+0x28d8081>
140086fd5: e9 1e 04 00 00              	jmp	0x1400873f8 <.text+0x863f8>
140086fda: 4c 8d 05 1f a2 8a 02        	leaq	0x28aa21f(%rip), %r8    # 0x142931200
140086fe1: b9 03 00 00 00              	movl	$0x3, %ecx
140086fe6: 48 89 f2                    	movq	%rsi, %rdx
140086fe9: e8 93 20 85 02              	callq	0x1428d9081 <.text+0x28d8081>
140086fee: e9 05 04 00 00              	jmp	0x1400873f8 <.text+0x863f8>
140086ff3: b9 08 00 00 00              	movl	$0x8, %ecx
140086ff8: ba 00 01 00 00              	movl	$0x100, %edx            # imm = 0x100
140086ffd: e8 fe 1f 85 02              	callq	0x1428d9000 <.text+0x28d8000>

; SOURCE /tmp/key-row-action.asm

B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140502d00: 41 57                       	pushq	%r15
140502d02: 41 56                       	pushq	%r14
140502d04: 41 55                       	pushq	%r13
140502d06: 41 54                       	pushq	%r12
140502d08: 56                          	pushq	%rsi
140502d09: 57                          	pushq	%rdi
140502d0a: 55                          	pushq	%rbp
140502d0b: 53                          	pushq	%rbx
140502d0c: 48 81 ec f8 00 00 00        	subq	$0xf8, %rsp
140502d13: 4c 89 ce                    	movq	%r9, %rsi
140502d16: 48 89 cb                    	movq	%rcx, %rbx
140502d19: 48 8b ac 24 60 01 00 00     	movq	0x160(%rsp), %rbp
140502d21: 48 89 94 24 d8 00 00 00     	movq	%rdx, 0xd8(%rsp)
140502d29: 4c 89 84 24 e0 00 00 00     	movq	%r8, 0xe0(%rsp)
140502d31: 48 8d 79 10                 	leaq	0x10(%rcx), %rdi
140502d35: 48 89 f9                    	movq	%rdi, %rcx
140502d38: e8 73 3b 18 00              	callq	0x1406868b0 <.text+0x6858b0>
140502d3d: 49 89 c4                    	movq	%rax, %r12
140502d40: 4c 8d bc 24 d8 00 00 00     	leaq	0xd8(%rsp), %r15
140502d48: 4c 89 f9                    	movq	%r15, %rcx
140502d4b: e8 c0 0e 18 00              	callq	0x140683c10 <.text+0x682c10>
140502d50: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
140502d55: 4c 8d 74 24 38              	leaq	0x38(%rsp), %r14
140502d5a: 4c 89 f1                    	movq	%r14, %rcx
140502d5d: e8 6e 58 b8 ff              	callq	0x1400885d0 <.text+0x875d0>
140502d62: 48 89 44 24 58              	movq	%rax, 0x58(%rsp)
140502d67: 48 b8 3b 81 28 ff 69 31 22 41       	movabsq	$0x41223169ff28813b, %rax # imm = 0x41223169FF28813B
140502d71: 48 89 44 24 60              	movq	%rax, 0x60(%rsp)
140502d76: 49 bd d9 68 a9 a2 68 72 9b a7       	movabsq	$-0x58648d975d569727, %r13 # imm = 0xA79B7268A2A968D9
140502d80: 4c 89 6c 24 68              	movq	%r13, 0x68(%rsp)
140502d85: 48 8d 4c 24 58              	leaq	0x58(%rsp), %rcx
140502d8a: e8 91 b1 0b 00              	callq	0x1405bdf20 <.text+0x5bcf20>
140502d8f: 4c 89 64 24 58              	movq	%r12, 0x58(%rsp)
140502d94: 48 89 44 24 60              	movq	%rax, 0x60(%rsp)
140502d99: 48 8d 4c 24 58              	leaq	0x58(%rsp), %rcx
140502d9e: e8 6d ab 0b 00              	callq	0x1405bd910 <.text+0x5bc910>
140502da3: 49 89 c4                    	movq	%rax, %r12
140502da6: 48 89 f9                    	movq	%rdi, %rcx
140502da9: 48 89 c2                    	movq	%rax, %rdx
140502dac: e8 9f 36 18 00              	callq	0x140686450 <.text+0x685450>
140502db1: 84 c0                       	testb	%al, %al
140502db3: 74 15                       	je	0x140502dca <.text+0x501dca>
140502db5: 48 8d 4c 24 38              	leaq	0x38(%rsp), %rcx
140502dba: 48 89 fa                    	movq	%rdi, %rdx
140502dbd: 4d 89 e0                    	movq	%r12, %r8
140502dc0: e8 6b 37 18 00              	callq	0x140686530 <.text+0x685530>
140502dc5: e9 53 01 00 00              	jmp	0x140502f1d <.text+0x501f1d>
140502dca: 48 8b 05 d7 95 12 03        	movq	0x31295d7(%rip), %rax   # 0x14362c3a8
140502dd1: 48 8b 00                    	movq	(%rax), %rax
140502dd4: 48 85 c0                    	testq	%rax, %rax
140502dd7: 0f 84 d8 00 00 00           	je	0x140502eb5 <.text+0x501eb5>
140502ddd: 48 89 5c 24 38              	movq	%rbx, 0x38(%rsp)
140502de2: 48 8d 05 b7 0a 03 00        	leaq	0x30ab7(%rip), %rax     # 0x1405338a0 <.text+0x5328a0>
140502de9: 48 89 44 24 40              	movq	%rax, 0x40(%rsp)
140502dee: 4c 89 7c 24 48              	movq	%r15, 0x48(%rsp)
140502df3: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
140502df8: 48 8d 0d 59 af 4e 02        	leaq	0x24eaf59(%rip), %rcx   # 0x1429edd58
140502dff: e8 7c af 17 00              	callq	0x14067dd80 <.text+0x67cd80>
140502e04: 0f 10 00                    	movups	(%rax), %xmm0
140502e07: 8b 40 10                    	movl	0x10(%rax), %eax
140502e0a: 48 c7 84 24 88 00 00 00 01 00 00 00 	movq	$0x1, 0x88(%rsp)
140502e16: 48 8d 0d 0b af 4e 02        	leaq	0x24eaf0b(%rip), %rcx   # 0x1429edd28
140502e1d: 48 89 8c 24 90 00 00 00     	movq	%rcx, 0x90(%rsp)
140502e25: 48 c7 84 24 98 00 00 00 14 00 00 00 	movq	$0x14, 0x98(%rsp)
140502e31: 48 8d 15 60 ae 4e 02        	leaq	0x24eae60(%rip), %rdx   # 0x1429edc98
140502e38: 48 89 94 24 a8 00 00 00     	movq	%rdx, 0xa8(%rsp)
140502e40: 48 c7 84 24 b0 00 00 00 02 00 00 00 	movq	$0x2, 0xb0(%rsp)
140502e4c: 4c 89 b4 24 b8 00 00 00     	movq	%r14, 0xb8(%rsp)
140502e54: 48 c7 84 24 c0 00 00 00 02 00 00 00 	movq	$0x2, 0xc0(%rsp)
140502e60: 48 c7 84 24 c8 00 00 00 00 00 00 00 	movq	$0x0, 0xc8(%rsp)
140502e6c: 48 c7 44 24 58 00 00 00 00  	movq	$0x0, 0x58(%rsp)
140502e75: 48 89 4c 24 60              	movq	%rcx, 0x60(%rsp)
140502e7a: 48 c7 44 24 68 14 00 00 00  	movq	$0x14, 0x68(%rsp)
140502e83: 48 c7 44 24 70 00 00 00 00  	movq	$0x0, 0x70(%rsp)
140502e8c: 0f 11 44 24 78              	movups	%xmm0, 0x78(%rsp)
140502e91: c7 84 24 a0 00 00 00 01 00 00 00    	movl	$0x1, 0xa0(%rsp)
140502e9c: 89 84 24 a4 00 00 00        	movl	%eax, 0xa4(%rsp)
140502ea3: 48 8d 8c 24 e8 00 00 00     	leaq	0xe8(%rsp), %rcx
140502eab: 48 8d 54 24 58              	leaq	0x58(%rsp), %rdx
140502eb0: e8 7b 74 84 00              	callq	0x140d4a330 <.text+0xd49330>
140502eb5: 0f 10 84 24 d8 00 00 00     	movups	0xd8(%rsp), %xmm0
140502ebd: 48 8b 5b 18                 	movq	0x18(%rbx), %rbx
140502ec1: 0f 11 84 24 e8 00 00 00     	movups	%xmm0, 0xe8(%rsp)
140502ec9: 48 8d 8c 24 e8 00 00 00     	leaq	0xe8(%rsp), %rcx
140502ed1: e8 3a 0d 18 00              	callq	0x140683c10 <.text+0x682c10>
140502ed6: 48 89 44 24 58              	movq	%rax, 0x58(%rsp)
140502edb: 48 b8 3b 81 28 ff 69 31 22 41       	movabsq	$0x41223169ff28813b, %rax # imm = 0x41223169FF28813B
140502ee5: 48 89 44 24 60              	movq	%rax, 0x60(%rsp)
140502eea: 4c 89 6c 24 68              	movq	%r13, 0x68(%rsp)
140502eef: 48 8d 4c 24 58              	leaq	0x58(%rsp), %rcx
140502ef4: e8 27 b0 0b 00              	callq	0x1405bdf20 <.text+0x5bcf20>
140502ef9: 48 89 5c 24 58              	movq	%rbx, 0x58(%rsp)
140502efe: 48 89 44 24 60              	movq	%rax, 0x60(%rsp)
140502f03: 48 8d 4c 24 58              	leaq	0x58(%rsp), %rcx
140502f08: e8 03 aa 0b 00              	callq	0x1405bd910 <.text+0x5bc910>
140502f0d: 48 8d 4c 24 38              	leaq	0x38(%rsp), %rcx
140502f12: 48 89 fa                    	movq	%rdi, %rdx
140502f15: 49 89 c0                    	movq	%rax, %r8
140502f18: e8 93 32 18 00              	callq	0x1406861b0 <.text+0x6851b0>
140502f1d: 48 8b 06                    	movq	(%rsi), %rax
140502f20: 4c 8b 45 00                 	movq	(%rbp), %r8
140502f24: 4c 8b 4d 08                 	movq	0x8(%rbp), %r9
140502f28: 48 85 c0                    	testq	%rax, %rax
140502f2b: 74 27                       	je	0x140502f54 <.text+0x501f54>
140502f2d: 48 8b 4e 08                 	movq	0x8(%rsi), %rcx
140502f31: 48 89 4c 24 28              	movq	%rcx, 0x28(%rsp)
140502f36: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140502f3b: c7 44 24 30 00 00 40 42     	movl	$0x42400000, 0x30(%rsp) # imm = 0x42400000
140502f43: 48 8d 4c 24 58              	leaq	0x58(%rsp), %rcx
140502f48: 48 8d 54 24 38              	leaq	0x38(%rsp), %rdx
140502f4d: e8 5e b1 e2 ff              	callq	0x14032e0b0 <.text+0x32d0b0>
140502f52: eb 2a                       	jmp	0x140502f7e <.text+0x501f7e>
140502f54: 48 8d 74 24 58              	leaq	0x58(%rsp), %rsi
140502f59: 48 8d 54 24 38              	leaq	0x38(%rsp), %rdx
140502f5e: 48 89 f1                    	movq	%rsi, %rcx
140502f61: e8 9a 8f 18 00              	callq	0x14068bf00 <.text+0x68af00>
140502f66: 48 89 f1                    	movq	%rsi, %rcx
140502f69: e8 c2 ed 17 00              	callq	0x140681d30 <.text+0x680d30>
140502f6e: f3 0f 10 0d 8a 16 43 02     	movss	0x243168a(%rip), %xmm1  # 0x142934600
140502f76: 48 89 c1                    	movq	%rax, %rcx
140502f79: e8 d2 33 c0 ff              	callq	0x140106350 <.text+0x105350>
140502f7e: 90                          	nop
140502f7f: 48 81 c4 f8 00 00 00        	addq	$0xf8, %rsp
140502f86: 5b                          	popq	%rbx
140502f87: 5d                          	popq	%rbp
140502f88: 5f                          	popq	%rdi
140502f89: 5e                          	popq	%rsi
140502f8a: 41 5c                       	popq	%r12
140502f8c: 41 5d                       	popq	%r13
140502f8e: 41 5e                       	popq	%r14
140502f90: 41 5f                       	popq	%r15
140502f92: c3                          	retq

; SOURCE /tmp/key-row-consumers.asm

B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140503f00: ff 48 89                    	decl	-0x77(%rax)
140503f03: 44 24 50                    	andb	$0x50, %al
140503f06: 49 bd 3b 81 28 ff 69 31 22 41       	movabsq	$0x41223169ff28813b, %r13 # imm = 0x41223169FF28813B
140503f10: 4c 89 6c 24 58              	movq	%r13, 0x58(%rsp)
140503f15: 48 bd d9 68 a9 a2 68 72 9b a7       	movabsq	$-0x58648d975d569727, %rbp # imm = 0xA79B7268A2A968D9
140503f1f: 48 89 6c 24 60              	movq	%rbp, 0x60(%rsp)
140503f24: 48 8d 4c 24 50              	leaq	0x50(%rsp), %rcx
140503f29: e8 f2 9f 0b 00              	callq	0x1405bdf20 <.text+0x5bcf20>
140503f2e: 4c 89 64 24 50              	movq	%r12, 0x50(%rsp)
140503f33: 48 89 44 24 58              	movq	%rax, 0x58(%rsp)
140503f38: 48 8d 4c 24 50              	leaq	0x50(%rsp), %rcx
140503f3d: e8 ce 99 0b 00              	callq	0x1405bd910 <.text+0x5bc910>
140503f42: 49 89 c4                    	movq	%rax, %r12
140503f45: 48 89 f9                    	movq	%rdi, %rcx
140503f48: 48 89 c2                    	movq	%rax, %rdx
140503f4b: e8 00 25 18 00              	callq	0x140686450 <.text+0x685450>
140503f50: 84 c0                       	testb	%al, %al
140503f52: 74 1b                       	je	0x140503f6f <.text+0x502f6f>
140503f54: 48 8d 9c 24 d0 00 00 00     	leaq	0xd0(%rsp), %rbx
140503f5c: 48 89 d9                    	movq	%rbx, %rcx
140503f5f: 48 89 fa                    	movq	%rdi, %rdx
140503f62: 4d 89 e0                    	movq	%r12, %r8
140503f65: e8 c6 25 18 00              	callq	0x140686530 <.text+0x685530>
140503f6a: e9 46 01 00 00              	jmp	0x1405040b5 <.text+0x5030b5>
140503f6f: 48 8b 05 32 84 12 03        	movq	0x3128432(%rip), %rax   # 0x14362c3a8
140503f76: 48 8b 00                    	movq	(%rax), %rax
140503f79: 48 85 c0                    	testq	%rax, %rax
140503f7c: 0f 84 d8 00 00 00           	je	0x14050405a <.text+0x50305a>
140503f82: 48 89 5c 24 20              	movq	%rbx, 0x20(%rsp)
140503f87: 48 8d 05 12 f9 02 00        	leaq	0x2f912(%rip), %rax     # 0x1405338a0 <.text+0x5328a0>
140503f8e: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
140503f93: 4c 89 7c 24 30              	movq	%r15, 0x30(%rsp)
140503f98: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
140503f9d: 48 8d 0d b4 9d 4e 02        	leaq	0x24e9db4(%rip), %rcx   # 0x1429edd58
140503fa4: e8 d7 9d 17 00              	callq	0x14067dd80 <.text+0x67cd80>
140503fa9: 0f 10 00                    	movups	(%rax), %xmm0
140503fac: 8b 40 10                    	movl	0x10(%rax), %eax
140503faf: 48 c7 84 24 80 00 00 00 01 00 00 00 	movq	$0x1, 0x80(%rsp)
140503fbb: 48 8d 0d 66 9d 4e 02        	leaq	0x24e9d66(%rip), %rcx   # 0x1429edd28
140503fc2: 48 89 8c 24 88 00 00 00     	movq	%rcx, 0x88(%rsp)
140503fca: 48 c7 84 24 90 00 00 00 14 00 00 00 	movq	$0x14, 0x90(%rsp)
140503fd6: 48 8d 15 bb 9c 4e 02        	leaq	0x24e9cbb(%rip), %rdx   # 0x1429edc98
140503fdd: 48 89 94 24 a0 00 00 00     	movq	%rdx, 0xa0(%rsp)
140503fe5: 48 c7 84 24 a8 00 00 00 02 00 00 00 	movq	$0x2, 0xa8(%rsp)
140503ff1: 4c 89 b4 24 b0 00 00 00     	movq	%r14, 0xb0(%rsp)
140503ff9: 48 c7 84 24 b8 00 00 00 02 00 00 00 	movq	$0x2, 0xb8(%rsp)
140504005: 48 c7 84 24 c0 00 00 00 00 00 00 00 	movq	$0x0, 0xc0(%rsp)
140504011: 48 c7 44 24 50 00 00 00 00  	movq	$0x0, 0x50(%rsp)
14050401a: 48 89 4c 24 58              	movq	%rcx, 0x58(%rsp)
14050401f: 48 c7 44 24 60 14 00 00 00  	movq	$0x14, 0x60(%rsp)
140504028: 48 c7 44 24 68 00 00 00 00  	movq	$0x0, 0x68(%rsp)
140504031: 0f 11 44 24 70              	movups	%xmm0, 0x70(%rsp)
140504036: c7 84 24 98 00 00 00 01 00 00 00    	movl	$0x1, 0x98(%rsp)
140504041: 89 84 24 9c 00 00 00        	movl	%eax, 0x9c(%rsp)
140504048: 48 8d 8c 24 d0 00 00 00     	leaq	0xd0(%rsp), %rcx
140504050: 48 8d 54 24 50              	leaq	0x50(%rsp), %rdx
140504055: e8 d6 62 84 00              	callq	0x140d4a330 <.text+0xd49330>
14050405a: 0f 10 44 24 40              	movups	0x40(%rsp), %xmm0
14050405f: 48 8b 5b 18                 	movq	0x18(%rbx), %rbx
140504063: 0f 11 44 24 20              	movups	%xmm0, 0x20(%rsp)
140504068: 48 8d 4c 24 20              	leaq	0x20(%rsp), %rcx
14050406d: e8 9e fb 17 00              	callq	0x140683c10 <.text+0x682c10>
140504072: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
140504077: 4c 89 6c 24 58              	movq	%r13, 0x58(%rsp)
14050407c: 48 89 6c 24 60              	movq	%rbp, 0x60(%rsp)
140504081: 48 8d 4c 24 50              	leaq	0x50(%rsp), %rcx
140504086: e8 95 9e 0b 00              	callq	0x1405bdf20 <.text+0x5bcf20>
14050408b: 48 89 5c 24 50              	movq	%rbx, 0x50(%rsp)
140504090: 48 89 44 24 58              	movq	%rax, 0x58(%rsp)
140504095: 48 8d 4c 24 50              	leaq	0x50(%rsp), %rcx
14050409a: e8 71 98 0b 00              	callq	0x1405bd910 <.text+0x5bc910>
14050409f: 48 8d 9c 24 d0 00 00 00     	leaq	0xd0(%rsp), %rbx
1405040a7: 48 89 d9                    	movq	%rbx, %rcx
1405040aa: 48 89 fa                    	movq	%rdi, %rdx
1405040ad: 49 89 c0                    	movq	%rax, %r8
1405040b0: e8 fb 20 18 00              	callq	0x1406861b0 <.text+0x6851b0>
1405040b5: 4c 8d 05 7d 9d 4e 02        	leaq	0x24e9d7d(%rip), %r8    # 0x1429ede39
1405040bc: 48 8d 7c 24 50              	leaq	0x50(%rsp), %rdi
1405040c1: 41 b9 09 00 00 00           	movl	$0x9, %r9d
1405040c7: 48 89 f9                    	movq	%rdi, %rcx
1405040ca: 48 89 da                    	movq	%rbx, %rdx
1405040cd: e8 ee d0 17 00              	callq	0x1406811c0 <.text+0x6801c0>
1405040d2: 48 8d 5c 24 20              	leaq	0x20(%rsp), %rbx
1405040d7: 48 89 d9                    	movq	%rbx, %rcx
1405040da: 48 89 fa                    	movq	%rdi, %rdx
1405040dd: 41 b0 12                    	movb	$0x12, %r8b
1405040e0: e8 db d5 17 00              	callq	0x1406816c0 <.text+0x6806c0>
1405040e5: 48 89 d9                    	movq	%rbx, %rcx
1405040e8: e8 63 dc 17 00              	callq	0x140681d50 <.text+0x680d50>
1405040ed: 48 89 c1                    	movq	%rax, %rcx
1405040f0: e8 8b fb 17 00              	callq	0x140683c80 <.text+0x682c80>
1405040f5: 84 c0                       	testb	%al, %al
1405040f7: 74 03                       	je	0x1405040fc <.text+0x5030fc>
1405040f9: c6 06 01                    	movb	$0x1, (%rsi)
1405040fc: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
140504103: 5b                          	popq	%rbx
140504104: 5d                          	popq	%rbp
140504105: 5f                          	popq	%rdi
140504106: 5e                          	popq	%rsi
140504107: 41 5c                       	popq	%r12
140504109: 41 5d                       	popq	%r13
14050410b: 41 5e                       	popq	%r14
14050410d: 41 5f                       	popq	%r15
14050410f: c3                          	retq
140504110: 55                          	pushq	%rbp
140504111: 41 57                       	pushq	%r15
140504113: 41 56                       	pushq	%r14
140504115: 41 55                       	pushq	%r13
140504117: 41 54                       	pushq	%r12
140504119: 56                          	pushq	%rsi
14050411a: 57                          	pushq	%rdi
14050411b: 53                          	pushq	%rbx
14050411c: 48 81 ec 28 01 00 00        	subq	$0x128, %rsp            # imm = 0x128
140504123: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14050412b: 0f 29 b5 90 00 00 00        	movaps	%xmm6, 0x90(%rbp)
140504132: 48 c7 85 88 00 00 00 fe ff ff ff    	movq	$-0x2, 0x88(%rbp)
14050413d: 48 89 55 08                 	movq	%rdx, 0x8(%rbp)
140504141: 48 8b 01                    	movq	(%rcx), %rax
140504144: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
140504148: 4c 8b 61 08                 	movq	0x8(%rcx), %r12
14050414c: 48 8b 79 10                 	movq	0x10(%rcx), %rdi
140504150: 4c 01 e7                    	addq	%r12, %rdi
140504153: 4c 8b 79 18                 	movq	0x18(%rcx), %r15
140504157: 4c 8b 71 20                 	movq	0x20(%rcx), %r14
14050415b: 48 8b 41 28                 	movq	0x28(%rcx), %rax
14050415f: 48 89 45 d0                 	movq	%rax, -0x30(%rbp)
140504163: 48 89 4d d8                 	movq	%rcx, -0x28(%rbp)
140504167: 48 8b 41 30                 	movq	0x30(%rcx), %rax
14050416b: 48 89 45 c8                 	movq	%rax, -0x38(%rbp)
14050416f: 48 8d 5d 10                 	leaq	0x10(%rbp), %rbx
140504173: f3 0f 10 35 85 04 43 02     	movss	0x2430485(%rip), %xmm6  # 0x142934600
14050417b: 4c 8d 6d b0                 	leaq	-0x50(%rbp), %r13
14050417f: 48 8d 75 20                 	leaq	0x20(%rbp), %rsi
140504183: 4c 89 65 e8                 	movq	%r12, -0x18(%rbp)
140504187: 49 39 fc                    	cmpq	%rdi, %r12
14050418a: 75 3b                       	jne	0x1405041c7 <.text+0x5031c7>
14050418c: e9 93 01 00 00              	jmp	0x140504324 <.text+0x503324>
140504191: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1405041a0: 0f b6 85 87 00 00 00        	movzbl	0x87(%rbp), %eax
1405041a7: c1 e0 08                    	shll	$0x8, %eax
1405041aa: 48 8b 4d d0                 	movq	-0x30(%rbp), %rcx
1405041ae: 48 89 01                    	movq	%rax, (%rcx)
1405041b1: 48 8b 45 c8                 	movq	-0x38(%rbp), %rax
1405041b5: c7 00 c3 00 00 00           	movl	$0xc3, (%rax)
1405041bb: 49 ff 06                    	incq	(%r14)
1405041be: 49 39 fc                    	cmpq	%rdi, %r12
1405041c1: 0f 84 5d 01 00 00           	je	0x140504324 <.text+0x503324>
1405041c7: 41 0f b6 04 24              	movzbl	(%r12), %eax
1405041cc: 88 85 87 00 00 00           	movb	%al, 0x87(%rbp)
1405041d2: 48 89 d9                    	movq	%rbx, %rcx
1405041d5: 48 8d 95 87 00 00 00        	leaq	0x87(%rbp), %rdx
1405041dc: 4d 89 f8                    	movq	%r15, %r8
1405041df: e8 1c 91 39 00              	callq	0x14089d300 <.text+0x89c300>
1405041e4: ba 01 00 00 00              	movl	$0x1, %edx
1405041e9: 81 7d 10 c3 00 00 00        	cmpl	$0xc3, 0x10(%rbp)
1405041f0: 48 8d 05 c1 9b 4e 02        	leaq	0x24e9bc1(%rip), %rax   # 0x1429eddb8
1405041f7: 74 2d                       	je	0x140504226 <.text+0x503226>
1405041f9: 8b 45 18                    	movl	0x18(%rbp), %eax
1405041fc: 89 45 28                    	movl	%eax, 0x28(%rbp)
1405041ff: 48 8b 45 10                 	movq	0x10(%rbp), %rax
140504203: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
140504207: 48 89 f1                    	movq	%rsi, %rcx
14050420a: e8 81 92 39 00              	callq	0x14089d490 <.text+0x89c490>
14050420f: 48 85 c0                    	testq	%rax, %rax
140504212: 48 8d 0d 57 a1 4e 02        	leaq	0x24ea157(%rip), %rcx   # 0x1429ee370
140504219: 48 0f 44 c1                 	cmoveq	%rcx, %rax
14050421d: b9 03 00 00 00              	movl	$0x3, %ecx
140504222: 48 0f 44 d1                 	cmoveq	%rcx, %rdx
140504226: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
14050422a: 48 89 55 f8                 	movq	%rdx, -0x8(%rbp)
14050422e: 48 8d 45 f0                 	leaq	-0x10(%rbp), %rax
140504232: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
140504236: 48 8d 05 53 5c c0 ff        	leaq	-0x3fa3ad(%rip), %rax   # 0x140109e90 <.text+0x108e90>
14050423d: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
140504241: 48 8d 05 50 9b 4e 02        	leaq	0x24e9b50(%rip), %rax   # 0x1429edd98
140504248: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
14050424c: 48 c7 45 28 02 00 00 00     	movq	$0x2, 0x28(%rbp)
140504254: 48 89 5d 30                 	movq	%rbx, 0x30(%rbp)
140504258: 48 c7 45 38 01 00 00 00     	movq	$0x1, 0x38(%rbp)
140504260: 48 c7 45 40 00 00 00 00     	movq	$0x0, 0x40(%rbp)
140504268: 4c 89 e9                    	movq	%r13, %rcx
14050426b: 48 89 f2                    	movq	%rsi, %rdx
14050426e: e8 1d 5d 87 00              	callq	0x140d79f90 <.text+0xd78f90>
140504273: 48 8b 45 b0                 	movq	-0x50(%rbp), %rax
140504277: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
14050427b: 4c 8b 45 b8                 	movq	-0x48(%rbp), %r8
14050427f: 4c 8b 4d c0                 	movq	-0x40(%rbp), %r9
140504283: 49 8b 06                    	movq	(%r14), %rax
140504286: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14050428b: 48 89 f1                    	movq	%rsi, %rcx
14050428e: 48 8b 55 08                 	movq	0x8(%rbp), %rdx
140504292: 4c 89 45 50                 	movq	%r8, 0x50(%rbp)
140504296: e8 65 82 18 00              	callq	0x14068c500 <.text+0x68b500>
14050429b: 48 89 f1                    	movq	%rsi, %rcx
14050429e: e8 9d da 17 00              	callq	0x140681d40 <.text+0x680d40>
1405042a3: 48 89 c1                    	movq	%rax, %rcx
1405042a6: 0f 28 ce                    	movaps	%xmm6, %xmm1
1405042a9: e8 a2 20 c0 ff              	callq	0x140106350 <.text+0x105350>
1405042ae: 48 8b 45 30                 	movq	0x30(%rbp), %rax
1405042b2: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
1405042b6: 0f 10 45 20                 	movups	0x20(%rbp), %xmm0
1405042ba: 0f 29 45 60                 	movaps	%xmm0, 0x60(%rbp)
1405042be: 48 8d 4d 60                 	leaq	0x60(%rbp), %rcx
1405042c2: e8 89 da 17 00              	callq	0x140681d50 <.text+0x680d50>
1405042c7: 48 89 c1                    	movq	%rax, %rcx
1405042ca: e8 b1 f9 17 00              	callq	0x140683c80 <.text+0x682c80>
1405042cf: 49 ff c4                    	incq	%r12
1405042d2: 84 c0                       	testb	%al, %al
1405042d4: 74 2a                       	je	0x140504300 <.text+0x503300>
1405042d6: 48 8b 55 58                 	movq	0x58(%rbp), %rdx
1405042da: 48 85 d2                    	testq	%rdx, %rdx
1405042dd: 0f 84 bd fe ff ff           	je	0x1405041a0 <.text+0x5031a0>
1405042e3: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1405042e9: 48 8b 4d 50                 	movq	0x50(%rbp), %rcx
1405042ed: e8 2e f9 10 00              	callq	0x140613c20 <.text+0x612c20>
1405042f2: e9 a9 fe ff ff              	jmp	0x1405041a0 <.text+0x5031a0>
1405042f7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140504300: 48 8b 55 58                 	movq	0x58(%rbp), %rdx
140504304: 48 85 d2                    	testq	%rdx, %rdx
140504307: 74 0f                       	je	0x140504318 <.text+0x503318>
140504309: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14050430f: 48 8b 4d 50                 	movq	0x50(%rbp), %rcx
140504313: e8 08 f9 10 00              	callq	0x140613c20 <.text+0x612c20>
140504318: 49 ff 06                    	incq	(%r14)
14050431b: 49 39 fc                    	cmpq	%rdi, %r12
14050431e: 0f 85 a3 fe ff ff           	jne	0x1405041c7 <.text+0x5031c7>
140504324: 48 8b 55 e0                 	movq	-0x20(%rbp), %rdx
140504328: 48 85 d2                    	testq	%rdx, %rdx
14050432b: 74 0f                       	je	0x14050433c <.text+0x50333c>
14050432d: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140504333: 48 8b 4d e8                 	movq	-0x18(%rbp), %rcx
140504337: e8 e4 f8 10 00              	callq	0x140613c20 <.text+0x612c20>
14050433c: 48 8b 45 d8                 	movq	-0x28(%rbp), %rax
140504340: 48 8b 40 40                 	movq	0x40(%rax), %rax
140504344: 0f b6 78 10                 	movzbl	0x10(%rax), %edi
140504348: 40 80 ff 04                 	cmpb	$0x4, %dil
14050434c: 0f 84 e6 02 00 00           	je	0x140504638 <.text+0x503638>
140504352: 48 8b 30                    	movq	(%rax), %rsi
140504355: 4c 8b 60 08                 	movq	0x8(%rax), %r12
140504359: 4c 8d 05 10 9a 4e 02        	leaq	0x24e9a10(%rip), %r8    # 0x1429edd70
140504360: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
140504364: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14050436a: 4c 8b 6d 08                 	movq	0x8(%rbp), %r13
14050436e: 4c 89 ea                    	movq	%r13, %rdx
140504371: e8 8a 7b 18 00              	callq	0x14068bf00 <.text+0x68af00>
140504376: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
14050437a: 4c 89 ea                    	movq	%r13, %rdx
14050437d: 49 89 f0                    	movq	%rsi, %r8
140504380: 4d 89 e1                    	movq	%r12, %r9
140504383: e8 78 7b 18 00              	callq	0x14068bf00 <.text+0x68af00>
140504388: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
14050438c: e8 9f d9 17 00              	callq	0x140681d30 <.text+0x680d30>
140504391: f3 0f 10 0d 67 02 43 02     	movss	0x2430267(%rip), %xmm1  # 0x142934600
140504399: 48 89 c1                    	movq	%rax, %rcx
14050439c: e8 af 1f c0 ff              	callq	0x140106350 <.text+0x105350>
1405043a1: 40 80 ff 03                 	cmpb	$0x3, %dil
1405043a5: 0f 84 8d 02 00 00           	je	0x140504638 <.text+0x503638>
1405043ab: 40 88 bd 86 00 00 00        	movb	%dil, 0x86(%rbp)
1405043b2: 48 8d 8d 86 00 00 00        	leaq	0x86(%rbp), %rcx
1405043b9: 4c 89 fa                    	movq	%r15, %rdx
1405043bc: e8 bf 84 39 00              	callq	0x14089c880 <.text+0x89b880>
1405043c1: 0f b7 c0                    	movzwl	%ax, %eax
1405043c4: 48 8d 0d 35 a0 4e 02        	leaq	0x24ea035(%rip), %rcx   # 0x1429ee400
1405043cb: 48 63 04 81                 	movslq	(%rcx,%rax,4), %rax
1405043cf: 48 01 c8                    	addq	%rcx, %rax
1405043d2: ff e0                       	jmpq	*%rax
1405043d4: 0f b6 05 2e 08 13 03        	movzbl	0x313082e(%rip), %eax   # 0x143634c09
1405043db: b9 03 00 00 00              	movl	$0x3, %ecx
1405043e0: ba 01 00 00 00              	movl	$0x1, %edx
1405043e5: e8 26 f8 10 00              	callq	0x140613c10 <.text+0x612c10>
1405043ea: 48 85 c0                    	testq	%rax, %rax
1405043ed: 0f 84 60 02 00 00           	je	0x140504653 <.text+0x503653>
1405043f3: c6 40 02 42                 	movb	$0x42, 0x2(%rax)
1405043f7: 66 c7 00 4c 4d              	movw	$0x4d4c, (%rax)         # imm = 0x4D4C
1405043fc: e9 d5 00 00 00              	jmp	0x1405044d6 <.text+0x5034d6>
140504401: 0f b6 05 01 08 13 03        	movzbl	0x3130801(%rip), %eax   # 0x143634c09
140504408: b9 07 00 00 00              	movl	$0x7, %ecx
14050440d: ba 01 00 00 00              	movl	$0x1, %edx
140504412: e8 f9 f7 10 00              	callq	0x140613c10 <.text+0x612c10>
140504417: 48 85 c0                    	testq	%rax, %rax
14050441a: 0f 84 49 02 00 00           	je	0x140504669 <.text+0x503669>
140504420: c7 40 03 77 61 72 64        	movl	$0x64726177, 0x3(%rax)  # imm = 0x64726177
140504427: c7 00 46 6f 72 77           	movl	$0x77726f46, (%rax)     # imm = 0x77726F46
14050442d: 48 c7 45 60 07 00 00 00     	movq	$0x7, 0x60(%rbp)
140504435: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
140504439: 48 c7 45 70 07 00 00 00     	movq	$0x7, 0x70(%rbp)
140504441: e9 f1 00 00 00              	jmp	0x140504537 <.text+0x503537>
140504446: 0f b6 05 bc 07 13 03        	movzbl	0x31307bc(%rip), %eax   # 0x143634c09
14050444d: b9 03 00 00 00              	movl	$0x3, %ecx
140504452: ba 01 00 00 00              	movl	$0x1, %edx
140504457: e8 b4 f7 10 00              	callq	0x140613c10 <.text+0x612c10>
14050445c: 48 85 c0                    	testq	%rax, %rax
14050445f: 0f 84 ee 01 00 00           	je	0x140504653 <.text+0x503653>
140504465: c6 40 02 42                 	movb	$0x42, 0x2(%rax)
140504469: 66 c7 00 4d 4d              	movw	$0x4d4d, (%rax)         # imm = 0x4D4D
14050446e: eb 66                       	jmp	0x1405044d6 <.text+0x5034d6>

; SOURCE /tmp/key-row-mouse-tail.asm

B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140504324: 48 8b 55 e0                 	movq	-0x20(%rbp), %rdx
140504328: 48 85 d2                    	testq	%rdx, %rdx
14050432b: 74 0f                       	je	0x14050433c <.text+0x50333c>
14050432d: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140504333: 48 8b 4d e8                 	movq	-0x18(%rbp), %rcx
140504337: e8 e4 f8 10 00              	callq	0x140613c20 <.text+0x612c20>
14050433c: 48 8b 45 d8                 	movq	-0x28(%rbp), %rax
140504340: 48 8b 40 40                 	movq	0x40(%rax), %rax
140504344: 0f b6 78 10                 	movzbl	0x10(%rax), %edi
140504348: 40 80 ff 04                 	cmpb	$0x4, %dil
14050434c: 0f 84 e6 02 00 00           	je	0x140504638 <.text+0x503638>
140504352: 48 8b 30                    	movq	(%rax), %rsi
140504355: 4c 8b 60 08                 	movq	0x8(%rax), %r12
140504359: 4c 8d 05 10 9a 4e 02        	leaq	0x24e9a10(%rip), %r8    # 0x1429edd70
140504360: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
140504364: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14050436a: 4c 8b 6d 08                 	movq	0x8(%rbp), %r13
14050436e: 4c 89 ea                    	movq	%r13, %rdx
140504371: e8 8a 7b 18 00              	callq	0x14068bf00 <.text+0x68af00>
140504376: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
14050437a: 4c 89 ea                    	movq	%r13, %rdx
14050437d: 49 89 f0                    	movq	%rsi, %r8
140504380: 4d 89 e1                    	movq	%r12, %r9
140504383: e8 78 7b 18 00              	callq	0x14068bf00 <.text+0x68af00>
140504388: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
14050438c: e8 9f d9 17 00              	callq	0x140681d30 <.text+0x680d30>
140504391: f3 0f 10 0d 67 02 43 02     	movss	0x2430267(%rip), %xmm1  # 0x142934600
140504399: 48 89 c1                    	movq	%rax, %rcx
14050439c: e8 af 1f c0 ff              	callq	0x140106350 <.text+0x105350>
1405043a1: 40 80 ff 03                 	cmpb	$0x3, %dil
1405043a5: 0f 84 8d 02 00 00           	je	0x140504638 <.text+0x503638>
1405043ab: 40 88 bd 86 00 00 00        	movb	%dil, 0x86(%rbp)
1405043b2: 48 8d 8d 86 00 00 00        	leaq	0x86(%rbp), %rcx
1405043b9: 4c 89 fa                    	movq	%r15, %rdx
1405043bc: e8 bf 84 39 00              	callq	0x14089c880 <.text+0x89b880>
1405043c1: 0f b7 c0                    	movzwl	%ax, %eax
1405043c4: 48 8d 0d 35 a0 4e 02        	leaq	0x24ea035(%rip), %rcx   # 0x1429ee400
1405043cb: 48 63 04 81                 	movslq	(%rcx,%rax,4), %rax
1405043cf: 48 01 c8                    	addq	%rcx, %rax
1405043d2: ff e0                       	jmpq	*%rax
1405043d4: 0f b6 05 2e 08 13 03        	movzbl	0x313082e(%rip), %eax   # 0x143634c09
1405043db: b9 03 00 00 00              	movl	$0x3, %ecx
1405043e0: ba 01 00 00 00              	movl	$0x1, %edx
1405043e5: e8 26 f8 10 00              	callq	0x140613c10 <.text+0x612c10>
1405043ea: 48 85 c0                    	testq	%rax, %rax
1405043ed: 0f 84 60 02 00 00           	je	0x140504653 <.text+0x503653>
1405043f3: c6 40 02 42                 	movb	$0x42, 0x2(%rax)
1405043f7: 66 c7 00 4c 4d              	movw	$0x4d4c, (%rax)         # imm = 0x4D4C
1405043fc: e9 d5 00 00 00              	jmp	0x1405044d6 <.text+0x5034d6>
140504401: 0f b6 05 01 08 13 03        	movzbl	0x3130801(%rip), %eax   # 0x143634c09
140504408: b9 07 00 00 00              	movl	$0x7, %ecx
14050440d: ba 01 00 00 00              	movl	$0x1, %edx
140504412: e8 f9 f7 10 00              	callq	0x140613c10 <.text+0x612c10>
140504417: 48 85 c0                    	testq	%rax, %rax
14050441a: 0f 84 49 02 00 00           	je	0x140504669 <.text+0x503669>
140504420: c7 40 03 77 61 72 64        	movl	$0x64726177, 0x3(%rax)  # imm = 0x64726177
140504427: c7 00 46 6f 72 77           	movl	$0x77726f46, (%rax)     # imm = 0x77726F46
14050442d: 48 c7 45 60 07 00 00 00     	movq	$0x7, 0x60(%rbp)
140504435: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
140504439: 48 c7 45 70 07 00 00 00     	movq	$0x7, 0x70(%rbp)
140504441: e9 f1 00 00 00              	jmp	0x140504537 <.text+0x503537>
140504446: 0f b6 05 bc 07 13 03        	movzbl	0x31307bc(%rip), %eax   # 0x143634c09
14050444d: b9 03 00 00 00              	movl	$0x3, %ecx
140504452: ba 01 00 00 00              	movl	$0x1, %edx
140504457: e8 b4 f7 10 00              	callq	0x140613c10 <.text+0x612c10>
14050445c: 48 85 c0                    	testq	%rax, %rax
14050445f: 0f 84 ee 01 00 00           	je	0x140504653 <.text+0x503653>
140504465: c6 40 02 42                 	movb	$0x42, 0x2(%rax)
140504469: 66 c7 00 4d 4d              	movw	$0x4d4d, (%rax)         # imm = 0x4D4D
14050446e: eb 66                       	jmp	0x1405044d6 <.text+0x5034d6>
140504470: 0f b6 05 92 07 13 03        	movzbl	0x3130792(%rip), %eax   # 0x143634c09
140504477: b9 04 00 00 00              	movl	$0x4, %ecx
14050447c: ba 01 00 00 00              	movl	$0x1, %edx
140504481: e8 8a f7 10 00              	callq	0x140613c10 <.text+0x612c10>
140504486: 48 85 c0                    	testq	%rax, %rax
140504489: 0f 84 f0 01 00 00           	je	0x14050467f <.text+0x50367f>
14050448f: c7 00 42 61 63 6b           	movl	$0x6b636142, (%rax)     # imm = 0x6B636142
140504495: 48 c7 45 60 04 00 00 00     	movq	$0x4, 0x60(%rbp)
14050449d: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
1405044a1: 48 c7 45 70 04 00 00 00     	movq	$0x4, 0x70(%rbp)
1405044a9: e9 89 00 00 00              	jmp	0x140504537 <.text+0x503537>
1405044ae: 0f b6 05 54 07 13 03        	movzbl	0x3130754(%rip), %eax   # 0x143634c09
1405044b5: b9 03 00 00 00              	movl	$0x3, %ecx
1405044ba: ba 01 00 00 00              	movl	$0x1, %edx
1405044bf: e8 4c f7 10 00              	callq	0x140613c10 <.text+0x612c10>
1405044c4: 48 85 c0                    	testq	%rax, %rax
1405044c7: 0f 84 86 01 00 00           	je	0x140504653 <.text+0x503653>
1405044cd: c6 40 02 42                 	movb	$0x42, 0x2(%rax)
1405044d1: 66 c7 00 52 4d              	movw	$0x4d52, (%rax)         # imm = 0x4D52
1405044d6: 48 c7 45 60 03 00 00 00     	movq	$0x3, 0x60(%rbp)
1405044de: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
1405044e2: 48 c7 45 70 03 00 00 00     	movq	$0x3, 0x70(%rbp)
1405044ea: eb 4b                       	jmp	0x140504537 <.text+0x503537>
1405044ec: 66 89 55 10                 	movw	%dx, 0x10(%rbp)
1405044f0: 48 89 5d f0                 	movq	%rbx, -0x10(%rbp)
1405044f4: 48 8d 05 15 8e 88 00        	leaq	0x888e15(%rip), %rax    # 0x140d8d310 <.text+0xd8c310>
1405044fb: 48 89 45 f8                 	movq	%rax, -0x8(%rbp)
1405044ff: 48 8d 05 7a 98 4e 02        	leaq	0x24e987a(%rip), %rax   # 0x1429edd80
140504506: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
14050450a: 48 c7 45 28 01 00 00 00     	movq	$0x1, 0x28(%rbp)
140504512: 48 8d 45 f0                 	leaq	-0x10(%rbp), %rax
140504516: 48 89 45 30                 	movq	%rax, 0x30(%rbp)
14050451a: 48 c7 45 38 01 00 00 00     	movq	$0x1, 0x38(%rbp)
140504522: 48 c7 45 40 00 00 00 00     	movq	$0x0, 0x40(%rbp)
14050452a: 48 8d 4d 60                 	leaq	0x60(%rbp), %rcx
14050452e: 48 8d 55 20                 	leaq	0x20(%rbp), %rdx
140504532: e8 59 5a 87 00              	callq	0x140d79f90 <.text+0xd78f90>
140504537: 48 8d 45 60                 	leaq	0x60(%rbp), %rax
14050453b: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
14050453f: 48 8d 05 8a 75 b3 ff        	leaq	-0x4c8a76(%rip), %rax   # 0x14003bad0 <.text+0x3aad0>
140504546: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
14050454a: 48 8d 05 47 98 4e 02        	leaq	0x24e9847(%rip), %rax   # 0x1429edd98
140504551: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
140504555: 48 c7 45 28 02 00 00 00     	movq	$0x2, 0x28(%rbp)
14050455d: 48 89 5d 30                 	movq	%rbx, 0x30(%rbp)
140504561: 48 c7 45 38 01 00 00 00     	movq	$0x1, 0x38(%rbp)
140504569: 48 c7 45 40 00 00 00 00     	movq	$0x0, 0x40(%rbp)
140504571: 48 8d 4d f0                 	leaq	-0x10(%rbp), %rcx
140504575: 48 8d 55 20                 	leaq	0x20(%rbp), %rdx
140504579: e8 12 5a 87 00              	callq	0x140d79f90 <.text+0xd78f90>
14050457e: 48 8b 55 60                 	movq	0x60(%rbp), %rdx
140504582: 48 85 d2                    	testq	%rdx, %rdx
140504585: 74 0f                       	je	0x140504596 <.text+0x503596>
140504587: 48 8b 4d 68                 	movq	0x68(%rbp), %rcx
14050458b: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140504591: e8 8a f6 10 00              	callq	0x140613c20 <.text+0x612c20>
140504596: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
14050459a: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
14050459e: 4c 8b 45 f8                 	movq	-0x8(%rbp), %r8
1405045a2: 4c 8b 4d 00                 	movq	(%rbp), %r9
1405045a6: 49 8b 06                    	movq	(%r14), %rax
1405045a9: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1405045ae: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
1405045b2: 48 8b 55 08                 	movq	0x8(%rbp), %rdx
1405045b6: 4c 89 45 50                 	movq	%r8, 0x50(%rbp)
1405045ba: e8 41 7f 18 00              	callq	0x14068c500 <.text+0x68b500>
1405045bf: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
1405045c3: e8 78 d7 17 00              	callq	0x140681d40 <.text+0x680d40>
1405045c8: f3 0f 10 0d 30 00 43 02     	movss	0x2430030(%rip), %xmm1  # 0x142934600
1405045d0: 48 89 c1                    	movq	%rax, %rcx
1405045d3: e8 78 1d c0 ff              	callq	0x140106350 <.text+0x105350>
1405045d8: 48 8b 45 30                 	movq	0x30(%rbp), %rax
1405045dc: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
1405045e0: 0f 10 45 20                 	movups	0x20(%rbp), %xmm0
1405045e4: 0f 29 45 60                 	movaps	%xmm0, 0x60(%rbp)
1405045e8: 48 8d 4d 60                 	leaq	0x60(%rbp), %rcx
1405045ec: e8 5f d7 17 00              	callq	0x140681d50 <.text+0x680d50>
1405045f1: 48 89 c1                    	movq	%rax, %rcx
1405045f4: e8 87 f6 17 00              	callq	0x140683c80 <.text+0x682c80>
1405045f9: 89 c3                       	movl	%eax, %ebx
1405045fb: 48 8b 55 58                 	movq	0x58(%rbp), %rdx
1405045ff: 48 85 d2                    	testq	%rdx, %rdx
140504602: 74 0f                       	je	0x140504613 <.text+0x503613>
140504604: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14050460a: 48 8b 4d 50                 	movq	0x50(%rbp), %rcx
14050460e: e8 0d f6 10 00              	callq	0x140613c20 <.text+0x612c20>
140504613: 84 db                       	testb	%bl, %bl
140504615: 74 21                       	je	0x140504638 <.text+0x503638>
140504617: 0f b6 85 86 00 00 00        	movzbl	0x86(%rbp), %eax
14050461e: c1 e0 08                    	shll	$0x8, %eax
140504621: 48 ff c0                    	incq	%rax
140504624: 48 8b 4d d0                 	movq	-0x30(%rbp), %rcx
140504628: 48 89 01                    	movq	%rax, (%rcx)
14050462b: 48 8b 45 d8                 	movq	-0x28(%rbp), %rax
14050462f: 48 8b 40 38                 	movq	0x38(%rax), %rax
140504633: 66 c7 00 06 00              	movw	$0x6, (%rax)
140504638: 0f 28 b5 90 00 00 00        	movaps	0x90(%rbp), %xmm6
14050463f: 48 81 c4 28 01 00 00        	addq	$0x128, %rsp            # imm = 0x128
140504646: 5b                          	popq	%rbx
140504647: 5f                          	popq	%rdi
140504648: 5e                          	popq	%rsi
140504649: 41 5c                       	popq	%r12
14050464b: 41 5d                       	popq	%r13
14050464d: 41 5e                       	popq	%r14
14050464f: 41 5f                       	popq	%r15
140504651: 5d                          	popq	%rbp
140504652: c3                          	retq
140504653: 4c 8d 05 06 96 4e 02        	leaq	0x24e9606(%rip), %r8    # 0x1429edc60
14050465a: b9 01 00 00 00              	movl	$0x1, %ecx
14050465f: ba 03 00 00 00              	movl	$0x3, %edx
140504664: e8 7a 49 3d 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140504669: 4c 8d 05 f0 95 4e 02        	leaq	0x24e95f0(%rip), %r8    # 0x1429edc60
140504670: b9 01 00 00 00              	movl	$0x1, %ecx
140504675: ba 07 00 00 00              	movl	$0x7, %edx
14050467a: e8 64 49 3d 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14050467f: 4c 8d 05 da 95 4e 02        	leaq	0x24e95da(%rip), %r8    # 0x1429edc60
140504686: b9 01 00 00 00              	movl	$0x1, %ecx
14050468b: ba 04 00 00 00              	movl	$0x4, %edx
140504690: e8 4e 49 3d 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140504695: cc                          	int3
140504696: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1405046a0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1405046a5: 55                          	pushq	%rbp
1405046a6: 41 57                       	pushq	%r15
1405046a8: 41 56                       	pushq	%r14
1405046aa: 41 55                       	pushq	%r13
1405046ac: 41 54                       	pushq	%r12
1405046ae: 56                          	pushq	%rsi
1405046af: 57                          	pushq	%rdi
1405046b0: 53                          	pushq	%rbx
1405046b1: 48 83 ec 38                 	subq	$0x38, %rsp
1405046b5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1405046bc: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
1405046c1: 48 8b 55 e0                 	movq	-0x20(%rbp), %rdx
1405046c5: 48 85 d2                    	testq	%rdx, %rdx
1405046c8: 74 0f                       	je	0x1405046d9 <.text+0x5036d9>
1405046ca: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1405046d0: 48 8b 4d e8                 	movq	-0x18(%rbp), %rcx
1405046d4: e8 47 f5 10 00              	callq	0x140613c20 <.text+0x612c20>
1405046d9: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
1405046de: 48 83 c4 38                 	addq	$0x38, %rsp

; SOURCE /tmp/key-modal.asm

B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140507f00: 55                          	pushq	%rbp
140507f01: 41 57                       	pushq	%r15
140507f03: 41 56                       	pushq	%r14
140507f05: 41 55                       	pushq	%r13
140507f07: 41 54                       	pushq	%r12
140507f09: 56                          	pushq	%rsi
140507f0a: 57                          	pushq	%rdi
140507f0b: 53                          	pushq	%rbx
140507f0c: 48 81 ec 38 01 00 00        	subq	$0x138, %rsp            # imm = 0x138
140507f13: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140507f1b: 48 c7 85 b0 00 00 00 fe ff ff ff    	movq	$-0x2, 0xb0(%rbp)
140507f26: 48 89 d6                    	movq	%rdx, %rsi
140507f29: 48 89 cb                    	movq	%rcx, %rbx
140507f2c: 4c 8b 31                    	movq	(%rcx), %r14
140507f2f: 48 89 e9                    	movq	%rbp, %rcx
140507f32: 4c 89 f2                    	movq	%r14, %rdx
140507f35: e8 26 4c 39 00              	callq	0x14089cb60 <.text+0x89bb60>
140507f3a: 4c 8b 45 18                 	movq	0x18(%rbp), %r8
140507f3e: 4d 85 c0                    	testq	%r8, %r8
140507f41: 74 24                       	je	0x140507f67 <.text+0x506f67>
140507f43: 4c 8b 4d 20                 	movq	0x20(%rbp), %r9
140507f47: 4c 89 4c 24 28              	movq	%r9, 0x28(%rsp)
140507f4c: 4c 89 44 24 20              	movq	%r8, 0x20(%rsp)
140507f51: c7 44 24 30 00 00 80 42     	movl	$0x42800000, 0x30(%rsp) # imm = 0x42800000
140507f59: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
140507f5d: 48 89 f2                    	movq	%rsi, %rdx
140507f60: e8 4b 61 e2 ff              	callq	0x14032e0b0 <.text+0x32d0b0>
140507f65: eb 39                       	jmp	0x140507fa0 <.text+0x506fa0>
140507f67: 31 c0                       	xorl	%eax, %eax
140507f69: 48 3b 45 00                 	cmpq	(%rbp), %rax
140507f6d: 0f 80 14 03 00 00           	jo	0x140508287 <.text+0x507287>
140507f73: 4c 8b 45 08                 	movq	0x8(%rbp), %r8
140507f77: 4c 8b 4d 10                 	movq	0x10(%rbp), %r9
140507f7b: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
140507f7f: 48 89 f2                    	movq	%rsi, %rdx
140507f82: e8 79 3f 18 00              	callq	0x14068bf00 <.text+0x68af00>
140507f87: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
140507f8b: e8 a0 9d 17 00              	callq	0x140681d30 <.text+0x680d30>
140507f90: f3 0f 10 0d e4 d8 41 02     	movss	0x241d8e4(%rip), %xmm1  # 0x14292587c
140507f98: 48 89 c1                    	movq	%rax, %rcx
140507f9b: e8 b0 e3 bf ff              	callq	0x140106350 <.text+0x105350>
140507fa0: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
140507fa4: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140507faa: 48 89 f2                    	movq	%rsi, %rdx
140507fad: 45 31 c9                    	xorl	%r9d, %r9d
140507fb0: e8 4b 3f 18 00              	callq	0x14068bf00 <.text+0x68af00>
140507fb5: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140507fb9: 4c 8b 00                    	movq	(%rax), %r8
140507fbc: 48 8d 7d 28                 	leaq	0x28(%rbp), %rdi
140507fc0: 48 89 f9                    	movq	%rdi, %rcx
140507fc3: 4c 89 f2                    	movq	%r14, %rdx
140507fc6: e8 35 53 39 00              	callq	0x14089d300 <.text+0x89c300>
140507fcb: 81 7d 28 c3 00 00 00        	cmpl	$0xc3, 0x28(%rbp)
140507fd2: 75 0e                       	jne	0x140507fe2 <.text+0x506fe2>
140507fd4: 48 8d 0d dd 5d 4e 02        	leaq	0x24e5ddd(%rip), %rcx   # 0x1429eddb8
140507fdb: ba 01 00 00 00              	movl	$0x1, %edx
140507fe0: eb 2e                       	jmp	0x140508010 <.text+0x507010>
140507fe2: 8b 45 30                    	movl	0x30(%rbp), %eax
140507fe5: 89 45 58                    	movl	%eax, 0x58(%rbp)
140507fe8: 48 8b 45 28                 	movq	0x28(%rbp), %rax
140507fec: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
140507ff0: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
140507ff4: e8 97 54 39 00              	callq	0x14089d490 <.text+0x89c490>
140507ff9: 48 85 c0                    	testq	%rax, %rax
140507ffc: 48 8d 0d 6d 63 4e 02        	leaq	0x24e636d(%rip), %rcx   # 0x1429ee370
140508003: 48 0f 45 c8                 	cmovneq	%rax, %rcx
140508007: b8 03 00 00 00              	movl	$0x3, %eax
14050800c: 48 0f 44 d0                 	cmoveq	%rax, %rdx
140508010: 48 89 4d d8                 	movq	%rcx, -0x28(%rbp)
140508014: 48 89 55 e0                 	movq	%rdx, -0x20(%rbp)
140508018: 48 8b 43 10                 	movq	0x10(%rbx), %rax
14050801c: 8b 18                       	movl	(%rax), %ebx
14050801e: 4c 8b 70 04                 	movq	0x4(%rax), %r14
140508022: 48 8d 0d d7 62 4e 02        	leaq	0x24e62d7(%rip), %rcx   # 0x1429ee300
140508029: ba 20 00 00 00              	movl	$0x20, %edx
14050802e: e8 4d 54 69 00              	callq	0x140b9d480 <.text+0xb9c480>
140508033: 48 89 45 28                 	movq	%rax, 0x28(%rbp)
140508037: 48 89 55 30                 	movq	%rdx, 0x30(%rbp)
14050803b: 48 89 bd 80 00 00 00        	movq	%rdi, 0x80(%rbp)
140508042: 4c 8d 2d 47 1e c0 ff        	leaq	-0x3fe1b9(%rip), %r13   # 0x140109e90 <.text+0x108e90>
140508049: 4c 89 ad 88 00 00 00        	movq	%r13, 0x88(%rbp)
140508050: 48 8d 05 d1 62 4e 02        	leaq	0x24e62d1(%rip), %rax   # 0x1429ee328
140508057: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
14050805b: 48 c7 45 58 02 00 00 00     	movq	$0x2, 0x58(%rbp)
140508063: 4c 8d a5 80 00 00 00        	leaq	0x80(%rbp), %r12
14050806a: 4c 89 65 60                 	movq	%r12, 0x60(%rbp)
14050806e: 48 c7 45 68 01 00 00 00     	movq	$0x1, 0x68(%rbp)
140508076: 48 c7 45 70 00 00 00 00     	movq	$0x0, 0x70(%rbp)
14050807e: 48 8d 4d e8                 	leaq	-0x18(%rbp), %rcx
140508082: 48 8d 55 50                 	leaq	0x50(%rbp), %rdx
140508086: e8 05 1f 87 00              	callq	0x140d79f90 <.text+0xd78f90>
14050808b: 81 fb c3 00 00 00           	cmpl	$0xc3, %ebx
140508091: 75 1b                       	jne	0x1405080ae <.text+0x5070ae>
140508093: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
140508097: 48 89 85 90 00 00 00        	movq	%rax, 0x90(%rbp)
14050809e: 0f 10 45 e8                 	movups	-0x18(%rbp), %xmm0
1405080a2: 0f 29 85 80 00 00 00        	movaps	%xmm0, 0x80(%rbp)
1405080a9: e9 c0 00 00 00              	jmp	0x14050816e <.text+0x50716e>
1405080ae: 48 8b 45 e8                 	movq	-0x18(%rbp), %rax
1405080b2: 48 89 85 a8 00 00 00        	movq	%rax, 0xa8(%rbp)
1405080b9: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
1405080bd: 48 89 85 a0 00 00 00        	movq	%rax, 0xa0(%rbp)
1405080c4: 89 5d 50                    	movl	%ebx, 0x50(%rbp)
1405080c7: 4c 89 75 54                 	movq	%r14, 0x54(%rbp)
1405080cb: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
1405080cf: e8 bc 53 39 00              	callq	0x14089d490 <.text+0x89c490>
1405080d4: 49 89 c6                    	movq	%rax, %r14
1405080d7: 48 89 d3                    	movq	%rdx, %rbx
1405080da: 48 85 c0                    	testq	%rax, %rax
1405080dd: b8 03 00 00 00              	movl	$0x3, %eax
1405080e2: 48 0f 44 d8                 	cmoveq	%rax, %rbx
1405080e6: 48 85 db                    	testq	%rbx, %rbx
1405080e9: 0f 88 7b 01 00 00           	js	0x14050826a <.text+0x50726a>
1405080ef: 74 2b                       	je	0x14050811c <.text+0x50711c>
1405080f1: 0f b6 05 11 cb 12 03        	movzbl	0x312cb11(%rip), %eax   # 0x143634c09
1405080f8: b8 01 00 00 00              	movl	$0x1, %eax
1405080fd: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
140508101: ba 01 00 00 00              	movl	$0x1, %edx
140508106: 48 89 d9                    	movq	%rbx, %rcx
140508109: e8 02 bb 10 00              	callq	0x140613c10 <.text+0x612c10>
14050810e: 48 85 c0                    	testq	%rax, %rax
140508111: 0f 84 5b 01 00 00           	je	0x140508272 <.text+0x507272>
140508117: 49 89 c7                    	movq	%rax, %r15
14050811a: eb 06                       	jmp	0x140508122 <.text+0x507122>
14050811c: 41 bf 01 00 00 00           	movl	$0x1, %r15d
140508122: 4d 85 f6                    	testq	%r14, %r14
140508125: 48 8d 15 44 62 4e 02        	leaq	0x24e6244(%rip), %rdx   # 0x1429ee370
14050812c: 49 0f 45 d6                 	cmovneq	%r14, %rdx
140508130: 4c 89 f9                    	movq	%r15, %rcx
140508133: 49 89 d8                    	movq	%rbx, %r8
140508136: e8 c5 ba 41 02              	callq	0x142923c00 <.text+0x2922c00>
14050813b: 48 89 9d 80 00 00 00        	movq	%rbx, 0x80(%rbp)
140508142: 4c 89 bd 88 00 00 00        	movq	%r15, 0x88(%rbp)
140508149: 48 89 9d 90 00 00 00        	movq	%rbx, 0x90(%rbp)
140508150: 48 8b 95 a8 00 00 00        	movq	0xa8(%rbp), %rdx
140508157: 48 85 d2                    	testq	%rdx, %rdx
14050815a: 74 12                       	je	0x14050816e <.text+0x50716e>
14050815c: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140508162: 48 8b 8d a0 00 00 00        	movq	0xa0(%rbp), %rcx
140508169: e8 b2 ba 10 00              	callq	0x140613c20 <.text+0x612c20>
14050816e: 48 8d 45 d8                 	leaq	-0x28(%rbp), %rax
140508172: 48 89 45 28                 	movq	%rax, 0x28(%rbp)
140508176: 4c 89 6d 30                 	movq	%r13, 0x30(%rbp)
14050817a: 4c 89 65 38                 	movq	%r12, 0x38(%rbp)
14050817e: 48 8d 05 4b 39 b3 ff        	leaq	-0x4cc6b5(%rip), %rax   # 0x14003bad0 <.text+0x3aad0>
140508185: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
140508189: 48 8d 05 c0 61 4e 02        	leaq	0x24e61c0(%rip), %rax   # 0x1429ee350
140508190: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
140508194: 48 c7 45 58 02 00 00 00     	movq	$0x2, 0x58(%rbp)
14050819c: 48 89 7d 60                 	movq	%rdi, 0x60(%rbp)
1405081a0: 48 c7 45 68 02 00 00 00     	movq	$0x2, 0x68(%rbp)
1405081a8: 48 c7 45 70 00 00 00 00     	movq	$0x0, 0x70(%rbp)
1405081b0: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
1405081b4: 48 8d 55 50                 	leaq	0x50(%rbp), %rdx
1405081b8: e8 d3 1d 87 00              	callq	0x140d79f90 <.text+0xd78f90>
1405081bd: 48 8b 45 c0                 	movq	-0x40(%rbp), %rax
1405081c1: 48 89 85 a8 00 00 00        	movq	%rax, 0xa8(%rbp)
1405081c8: 4c 8b 45 c8                 	movq	-0x38(%rbp), %r8
1405081cc: 4c 8b 4d d0                 	movq	-0x30(%rbp), %r9
1405081d0: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
1405081d4: 48 89 f2                    	movq	%rsi, %rdx
1405081d7: 4c 89 85 a0 00 00 00        	movq	%r8, 0xa0(%rbp)
1405081de: e8 1d 3d 18 00              	callq	0x14068bf00 <.text+0x68af00>
1405081e3: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
1405081e7: e8 44 9b 17 00              	callq	0x140681d30 <.text+0x680d30>
1405081ec: f3 0f 10 0d 88 d6 41 02     	movss	0x241d688(%rip), %xmm1  # 0x14292587c
1405081f4: 48 89 c1                    	movq	%rax, %rcx
1405081f7: e8 54 e1 bf ff              	callq	0x140106350 <.text+0x105350>
1405081fc: 48 8b 95 a8 00 00 00        	movq	0xa8(%rbp), %rdx
140508203: 48 85 d2                    	testq	%rdx, %rdx
140508206: 74 12                       	je	0x14050821a <.text+0x50721a>
140508208: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14050820e: 48 8b 8d a0 00 00 00        	movq	0xa0(%rbp), %rcx
140508215: e8 06 ba 10 00              	callq	0x140613c20 <.text+0x612c20>
14050821a: 48 8b 95 80 00 00 00        	movq	0x80(%rbp), %rdx
140508221: 48 85 d2                    	testq	%rdx, %rdx
140508224: 74 12                       	je	0x140508238 <.text+0x507238>
140508226: 48 8b 8d 88 00 00 00        	movq	0x88(%rbp), %rcx
14050822d: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140508233: e8 e8 b9 10 00              	callq	0x140613c20 <.text+0x612c20>
140508238: 48 8b 55 00                 	movq	(%rbp), %rdx
14050823c: 48 89 d0                    	movq	%rdx, %rax
14050823f: 48 f7 d8                    	negq	%rax
140508242: 70 11                       	jo	0x140508255 <.text+0x507255>
140508244: 73 0f                       	jae	0x140508255 <.text+0x507255>
140508246: 48 8b 4d 08                 	movq	0x8(%rbp), %rcx
14050824a: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140508250: e8 cb b9 10 00              	callq	0x140613c20 <.text+0x612c20>
140508255: 90                          	nop
140508256: 48 81 c4 38 01 00 00        	addq	$0x138, %rsp            # imm = 0x138
14050825d: 5b                          	popq	%rbx
14050825e: 5f                          	popq	%rdi
14050825f: 5e                          	popq	%rsi
140508260: 41 5c                       	popq	%r12
140508262: 41 5d                       	popq	%r13
140508264: 41 5e                       	popq	%r14
140508266: 41 5f                       	popq	%r15
140508268: 5d                          	popq	%rbp
140508269: c3                          	retq
14050826a: 48 c7 45 48 00 00 00 00     	movq	$0x0, 0x48(%rbp)
140508272: 4c 8d 05 e7 59 4e 02        	leaq	0x24e59e7(%rip), %r8    # 0x1429edc60
140508279: 48 8b 4d 48                 	movq	0x48(%rbp), %rcx
14050827d: 48 89 da                    	movq	%rbx, %rdx
140508280: e8 5e 0d 3d 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140508285: eb 0c                       	jmp	0x140508293 <.text+0x507293>
140508287: 48 8d 0d 5a 60 4e 02        	leaq	0x24e605a(%rip), %rcx   # 0x1429ee2e8
14050828e: e8 7d 10 3d 02              	callq	0x1428d9310 <.text+0x28d8310>
140508293: 0f 0b                       	ud2
140508295: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
1405082a0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1405082a5: 55                          	pushq	%rbp
1405082a6: 41 57                       	pushq	%r15
1405082a8: 41 56                       	pushq	%r14
1405082aa: 41 55                       	pushq	%r13
1405082ac: 41 54                       	pushq	%r12
1405082ae: 56                          	pushq	%rsi
1405082af: 57                          	pushq	%rdi
1405082b0: 53                          	pushq	%rbx
1405082b1: 48 83 ec 38                 	subq	$0x38, %rsp
1405082b5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1405082bc: 48 8b 55 00                 	movq	(%rbp), %rdx
1405082c0: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
1405082c8: 48 85 c0                    	testq	%rax, %rax
1405082cb: 74 0f                       	je	0x1405082dc <.text+0x5072dc>
1405082cd: 48 8b 4d 08                 	movq	0x8(%rbp), %rcx
1405082d1: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1405082d7: e8 44 b9 10 00              	callq	0x140613c20 <.text+0x612c20>
1405082dc: 90                          	nop
1405082dd: 48 83 c4 38                 	addq	$0x38, %rsp
1405082e1: 5b                          	popq	%rbx
1405082e2: 5f                          	popq	%rdi
1405082e3: 5e                          	popq	%rsi
1405082e4: 41 5c                       	popq	%r12
1405082e6: 41 5d                       	popq	%r13
1405082e8: 41 5e                       	popq	%r14
1405082ea: 41 5f                       	popq	%r15
1405082ec: 5d                          	popq	%rbp
1405082ed: c3                          	retq
1405082ee: 66 90                       	nop
1405082f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1405082f5: 55                          	pushq	%rbp
1405082f6: 41 57                       	pushq	%r15
1405082f8: 41 56                       	pushq	%r14
1405082fa: 41 55                       	pushq	%r13
1405082fc: 41 54                       	pushq	%r12
1405082fe: 56                          	pushq	%rsi
1405082ff: 57                          	pushq	%rdi
140508300: 53                          	pushq	%rbx
140508301: 48 83 ec 38                 	subq	$0x38, %rsp
140508305: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14050830c: 48 83 bd a8 00 00 00 00     	cmpq	$0x0, 0xa8(%rbp)
140508314: 74 19                       	je	0x14050832f <.text+0x50732f>
140508316: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14050831c: 48 8b 8d a0 00 00 00        	movq	0xa0(%rbp), %rcx
140508323: 48 8b 95 a8 00 00 00        	movq	0xa8(%rbp), %rdx
14050832a: e8 f1 b8 10 00              	callq	0x140613c20 <.text+0x612c20>
14050832f: 90                          	nop
140508330: 48 83 c4 38                 	addq	$0x38, %rsp
140508334: 5b                          	popq	%rbx
140508335: 5f                          	popq	%rdi
140508336: 5e                          	popq	%rsi
140508337: 41 5c                       	popq	%r12
140508339: 41 5d                       	popq	%r13
14050833b: 41 5e                       	popq	%r14
14050833d: 41 5f                       	popq	%r15
14050833f: 5d                          	popq	%rbp
140508340: c3                          	retq
140508341: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
140508350: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140508355: 55                          	pushq	%rbp
140508356: 41 57                       	pushq	%r15
140508358: 41 56                       	pushq	%r14
14050835a: 41 55                       	pushq	%r13
14050835c: 41 54                       	pushq	%r12
14050835e: 56                          	pushq	%rsi
14050835f: 57                          	pushq	%rdi
140508360: 53                          	pushq	%rbx
140508361: 48 83 ec 38                 	subq	$0x38, %rsp
140508365: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14050836c: 48 8b 95 80 00 00 00        	movq	0x80(%rbp), %rdx
140508373: 48 85 d2                    	testq	%rdx, %rdx
140508376: 74 12                       	je	0x14050838a <.text+0x50738a>
140508378: 48 8b 8d 88 00 00 00        	movq	0x88(%rbp), %rcx
14050837f: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140508385: e8 96 b8 10 00              	callq	0x140613c20 <.text+0x612c20>
14050838a: 90                          	nop
14050838b: 48 83 c4 38                 	addq	$0x38, %rsp
14050838f: 5b                          	popq	%rbx
140508390: 5f                          	popq	%rdi
140508391: 5e                          	popq	%rsi
140508392: 41 5c                       	popq	%r12
140508394: 41 5d                       	popq	%r13
140508396: 41 5e                       	popq	%r14
140508398: 41 5f                       	popq	%r15
14050839a: 5d                          	popq	%rbp
14050839b: c3                          	retq
14050839c: 0f 1f 40 00                 	nopl	(%rax)
1405083a0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1405083a5: 55                          	pushq	%rbp
1405083a6: 41 57                       	pushq	%r15
1405083a8: 41 56                       	pushq	%r14
1405083aa: 41 55                       	pushq	%r13
1405083ac: 41 54                       	pushq	%r12
1405083ae: 56                          	pushq	%rsi
1405083af: 57                          	pushq	%rdi
1405083b0: 53                          	pushq	%rbx
1405083b1: 48 83 ec 38                 	subq	$0x38, %rsp
1405083b5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1405083bc: 48 83 bd a8 00 00 00 00     	cmpq	$0x0, 0xa8(%rbp)
1405083c4: 74 19                       	je	0x1405083df <.text+0x5073df>
1405083c6: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1405083cc: 48 8b 8d a0 00 00 00        	movq	0xa0(%rbp), %rcx
1405083d3: 48 8b 95 a8 00 00 00        	movq	0xa8(%rbp), %rdx
1405083da: e8 41 b8 10 00              	callq	0x140613c20 <.text+0x612c20>
1405083df: 90                          	nop
1405083e0: 48 83 c4 38                 	addq	$0x38, %rsp
1405083e4: 5b                          	popq	%rbx
1405083e5: 5f                          	popq	%rdi
1405083e6: 5e                          	popq	%rsi
1405083e7: 41 5c                       	popq	%r12
1405083e9: 41 5d                       	popq	%r13
1405083eb: 41 5e                       	popq	%r14
1405083ed: 41 5f                       	popq	%r15
1405083ef: 5d                          	popq	%rbp
1405083f0: c3                          	retq
1405083f1: cc                          	int3
1405083f2: cc                          	int3
1405083f3: cc                          	int3
1405083f4: cc                          	int3
1405083f5: cc                          	int3
1405083f6: cc                          	int3
1405083f7: cc                          	int3
1405083f8: cc                          	int3
1405083f9: cc                          	int3
1405083fa: cc                          	int3
1405083fb: cc                          	int3
1405083fc: cc                          	int3
1405083fd: cc                          	int3
1405083fe: cc                          	int3
1405083ff: cc                          	int3
140508400: 41 57                       	pushq	%r15
140508402: 41 56                       	pushq	%r14
140508404: 41 54                       	pushq	%r12
140508406: 56                          	pushq	%rsi
140508407: 57                          	pushq	%rdi
140508408: 53                          	pushq	%rbx
140508409: 48 83 ec 68                 	subq	$0x68, %rsp
14050840d: 48 89 d7                    	movq	%rdx, %rdi
140508410: 48 89 ce                    	movq	%rcx, %rsi
140508413: 48 8b 01                    	movq	(%rcx), %rax
140508416: 4c 8b 30                    	movq	(%rax), %r14
140508419: 41 81 3e c3 00 00 00        	cmpl	$0xc3, (%r14)
140508420: 0f 85 ad 00 00 00           	jne	0x1405084d3 <.text+0x5074d3>
140508426: 48 8b 5f 08                 	movq	0x8(%rdi), %rbx
14050842a: 48 c7 44 24 38 00 00 00 00  	movq	$0x0, 0x38(%rsp)
140508433: 48 8d 4c 24 38              	leaq	0x38(%rsp), %rcx
140508438: e8 93 01 b8 ff              	callq	0x1400885d0 <.text+0x875d0>
14050843d: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140508442: 48 b8 3b 81 28 ff 69 31 22 41       	movabsq	$0x41223169ff28813b, %rax # imm = 0x41223169FF28813B
14050844c: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
140508451: 48 b8 d9 68 a9 a2 68 72 9b a7       	movabsq	$-0x58648d975d569727, %rax # imm = 0xA79B7268A2A968D9
14050845b: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
140508460: 48 8d 4c 24 20              	leaq	0x20(%rsp), %rcx
140508465: e8 b6 5a 0b 00              	callq	0x1405bdf20 <.text+0x5bcf20>
14050846a: 48 89 5c 24 20              	movq	%rbx, 0x20(%rsp)
14050846f: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
140508474: 48 8d 4c 24 20              	leaq	0x20(%rsp), %rcx
140508479: e8 92 54 0b 00              	callq	0x1405bd910 <.text+0x5bc910>
14050847e: 48 8d 5c 24 50              	leaq	0x50(%rsp), %rbx
140508483: 48 89 d9                    	movq	%rbx, %rcx
140508486: 48 89 fa                    	movq	%rdi, %rdx
140508489: 49 89 c0                    	movq	%rax, %r8
14050848c: e8 1f dd 17 00              	callq	0x1406861b0 <.text+0x6851b0>
140508491: 48 8d 0d f9 5e 4e 02        	leaq	0x24e5ef9(%rip), %rcx   # 0x1429ee391
140508498: ba 06 00 00 00              	movl	$0x6, %edx
14050849d: e8 de 4f 69 00              	callq	0x140b9d480 <.text+0xb9c480>
1405084a2: 49 89 d1                    	movq	%rdx, %r9
1405084a5: 48 8d 7c 24 20              	leaq	0x20(%rsp), %rdi
1405084aa: 48 89 f9                    	movq	%rdi, %rcx
1405084ad: 48 89 da                    	movq	%rbx, %rdx
1405084b0: 49 89 c0                    	movq	%rax, %r8
1405084b3: e8 08 3b 18 00              	callq	0x14068bfc0 <.text+0x68afc0>
1405084b8: 48 8d 5c 24 38              	leaq	0x38(%rsp), %rbx
1405084bd: 48 89 d9                    	movq	%rbx, %rcx
1405084c0: 48 89 fa                    	movq	%rdi, %rdx
1405084c3: 41 b0 12                    	movb	$0x12, %r8b
1405084c6: e8 05 62 17 00              	callq	0x14067e6d0 <.text+0x67d6d0>
1405084cb: 48 89 d9                    	movq	%rbx, %rcx
1405084ce: e9 f9 00 00 00              	jmp	0x1405085cc <.text+0x5075cc>
1405084d3: 48 8d 0d 99 5e 4e 02        	leaq	0x24e5e99(%rip), %rcx   # 0x1429ee373
1405084da: ba 1e 00 00 00              	movl	$0x1e, %edx
1405084df: e8 9c 4f 69 00              	callq	0x140b9d480 <.text+0xb9c480>
1405084e4: 49 89 d1                    	movq	%rdx, %r9
1405084e7: 48 8d 5c 24 20              	leaq	0x20(%rsp), %rbx
1405084ec: 48 89 d9                    	movq	%rbx, %rcx
1405084ef: 48 89 fa                    	movq	%rdi, %rdx
1405084f2: 49 89 c0                    	movq	%rax, %r8
1405084f5: e8 c6 3a 18 00              	callq	0x14068bfc0 <.text+0x68afc0>
1405084fa: 48 89 d9                    	movq	%rbx, %rcx
1405084fd: e8 4e 99 17 00              	callq	0x140681e50 <.text+0x680e50>
140508502: 48 89 c1                    	movq	%rax, %rcx
140508505: e8 46 b7 17 00              	callq	0x140683c50 <.text+0x682c50>
14050850a: 48 89 c1                    	movq	%rax, %rcx
14050850d: e8 6e b7 17 00              	callq	0x140683c80 <.text+0x682c80>
140508512: 84 c0                       	testb	%al, %al
140508514: 74 79                       	je	0x14050858f <.text+0x50758f>
140508516: 48 8b 46 08                 	movq	0x8(%rsi), %rax
14050851a: c6 00 01                    	movb	$0x1, (%rax)
14050851d: 48 8b 46 10                 	movq	0x10(%rsi), %rax
140508521: 8b 48 1c                    	movl	0x1c(%rax), %ecx
140508524: 48 8b 50 10                 	movq	0x10(%rax), %rdx
140508528: 89 0a                       	movl	%ecx, (%rdx)
14050852a: 48 8b 18                    	movq	(%rax), %rbx
14050852d: 4c 8b 7b 30                 	movq	0x30(%rbx), %r15
140508531: 4c 8b 63 40                 	movq	0x40(%rbx), %r12
140508535: 4c 3b 7b 20                 	cmpq	0x20(%rbx), %r15
140508539: 75 10                       	jne	0x14050854b <.text+0x50754b>
14050853b: 48 8d 4b 20                 	leaq	0x20(%rbx), %rcx
14050853f: 48 8d 15 8a 4f 45 02        	leaq	0x2454f8a(%rip), %rdx   # 0x14295d4d0
140508546: e8 25 a3 05 00              	callq	0x140562870 <.text+0x561870>
14050854b: 48 8b 43 28                 	movq	0x28(%rbx), %rax
14050854f: 4e 89 24 f8                 	movq	%r12, (%rax,%r15,8)
140508553: 49 ff c7                    	incq	%r15
140508556: 4c 89 7b 30                 	movq	%r15, 0x30(%rbx)
14050855a: 48 ff 43 40                 	incq	0x40(%rbx)
14050855e: 48 8b 46 18                 	movq	0x18(%rsi), %rax
140508562: 8b 48 1c                    	movl	0x1c(%rax), %ecx
140508565: 48 8b 50 10                 	movq	0x10(%rax), %rdx
140508569: 89 0a                       	movl	%ecx, (%rdx)
14050856b: 48 8b 08                    	movq	(%rax), %rcx
14050856e: 48 8b 46 20                 	movq	0x20(%rsi), %rax
140508572: 0f b6 10                    	movzbl	(%rax), %edx
140508575: 41 8b 46 08                 	movl	0x8(%r14), %eax
140508579: 89 44 24 28                 	movl	%eax, 0x28(%rsp)
14050857d: 49 8b 06                    	movq	(%r14), %rax
140508580: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140508585: 4c 8d 44 24 20              	leaq	0x20(%rsp), %r8
14050858a: e8 11 2d 39 00              	callq	0x14089b2a0 <.text+0x89a2a0>
14050858f: 48 8d 0d fb 5d 4e 02        	leaq	0x24e5dfb(%rip), %rcx   # 0x1429ee391
140508596: ba 06 00 00 00              	movl	$0x6, %edx
14050859b: e8 e0 4e 69 00              	callq	0x140b9d480 <.text+0xb9c480>
1405085a0: 49 89 d1                    	movq	%rdx, %r9
1405085a3: 48 8d 5c 24 20              	leaq	0x20(%rsp), %rbx
1405085a8: 48 89 d9                    	movq	%rbx, %rcx
1405085ab: 48 89 fa                    	movq	%rdi, %rdx
1405085ae: 49 89 c0                    	movq	%rax, %r8
1405085b1: e8 0a 3a 18 00              	callq	0x14068bfc0 <.text+0x68afc0>
1405085b6: 48 8d 7c 24 38              	leaq	0x38(%rsp), %rdi
1405085bb: 48 89 f9                    	movq	%rdi, %rcx
1405085be: 48 89 da                    	movq	%rbx, %rdx
1405085c1: 41 b0 12                    	movb	$0x12, %r8b
1405085c4: e8 07 61 17 00              	callq	0x14067e6d0 <.text+0x67d6d0>
1405085c9: 48 89 f9                    	movq	%rdi, %rcx
1405085cc: e8 7f 97 17 00              	callq	0x140681d50 <.text+0x680d50>
1405085d1: 48 89 c1                    	movq	%rax, %rcx
1405085d4: e8 a7 b6 17 00              	callq	0x140683c80 <.text+0x682c80>
1405085d9: 84 c0                       	testb	%al, %al
1405085db: 74 07                       	je	0x1405085e4 <.text+0x5075e4>
1405085dd: 48 8b 46 08                 	movq	0x8(%rsi), %rax
1405085e1: c6 00 01                    	movb	$0x1, (%rax)
1405085e4: 48 83 c4 68                 	addq	$0x68, %rsp
1405085e8: 5b                          	popq	%rbx
1405085e9: 5f                          	popq	%rdi
1405085ea: 5e                          	popq	%rsi
1405085eb: 41 5c                       	popq	%r12
1405085ed: 41 5e                       	popq	%r14
1405085ef: 41 5f                       	popq	%r15
1405085f1: c3                          	retq
1405085f2: cc                          	int3
1405085f3: cc                          	int3
1405085f4: cc                          	int3
1405085f5: cc                          	int3
1405085f6: cc                          	int3
1405085f7: cc                          	int3
1405085f8: cc                          	int3
1405085f9: cc                          	int3
1405085fa: cc                          	int3
1405085fb: cc                          	int3
1405085fc: cc                          	int3
1405085fd: cc                          	int3
1405085fe: cc                          	int3
1405085ff: cc                          	int3
140508600: 41 57                       	pushq	%r15
140508602: 41 56                       	pushq	%r14
140508604: 56                          	pushq	%rsi
140508605: 57                          	pushq	%rdi
140508606: 53                          	pushq	%rbx
140508607: 48 81 ec f0 01 00 00        	subq	$0x1f0, %rsp            # imm = 0x1F0
14050860e: 48 89 d7                    	movq	%rdx, %rdi
140508611: 48 89 ce                    	movq	%rcx, %rsi
140508614: 48 8d 0d 86 5d 4e 02        	leaq	0x24e5d86(%rip), %rcx   # 0x1429ee3a1
14050861b: ba 11 00 00 00              	movl	$0x11, %edx
140508620: e8 5b 4e 69 00              	callq	0x140b9d480 <.text+0xb9c480>
140508625: 49 89 d1                    	movq	%rdx, %r9
140508628: 48 8d 5c 24 28              	leaq	0x28(%rsp), %rbx
14050862d: 48 89 d9                    	movq	%rbx, %rcx
140508630: 48 89 fa                    	movq	%rdi, %rdx
140508633: 49 89 c0                    	movq	%rax, %r8
140508636: e8 85 39 18 00              	callq	0x14068bfc0 <.text+0x68afc0>
14050863b: 48 89 d9                    	movq	%rbx, %rcx
14050863e: e8 0d 97 17 00              	callq	0x140681d50 <.text+0x680d50>
140508643: 48 89 c1                    	movq	%rax, %rcx
140508646: e8 35 b6 17 00              	callq	0x140683c80 <.text+0x682c80>
14050864b: 84 c0                       	testb	%al, %al
14050864d: 74 7d                       	je	0x1405086cc <.text+0x5076cc>
14050864f: 48 8b 06                    	movq	(%rsi), %rax
140508652: c6 00 03                    	movb	$0x3, (%rax)
140508655: 48 8b 46 08                 	movq	0x8(%rsi), %rax
140508659: 8b 48 1c                    	movl	0x1c(%rax), %ecx
14050865c: 48 8b 50 10                 	movq	0x10(%rax), %rdx
140508660: 89 0a                       	movl	%ecx, (%rdx)
140508662: 48 8b 18                    	movq	(%rax), %rbx
140508665: 4c 8b 73 30                 	movq	0x30(%rbx), %r14
140508669: 4c 8b 7b 40                 	movq	0x40(%rbx), %r15
14050866d: 4c 3b 73 20                 	cmpq	0x20(%rbx), %r14
140508671: 75 10                       	jne	0x140508683 <.text+0x507683>
140508673: 48 8d 4b 20                 	leaq	0x20(%rbx), %rcx
140508677: 48 8d 15 52 4e 45 02        	leaq	0x2454e52(%rip), %rdx   # 0x14295d4d0
14050867e: e8 ed a1 05 00              	callq	0x140562870 <.text+0x561870>
140508683: 48 8b 43 28                 	movq	0x28(%rbx), %rax
140508687: 4e 89 3c f0                 	movq	%r15, (%rax,%r14,8)
14050868b: 49 ff c6                    	incq	%r14
14050868e: 4c 89 73 30                 	movq	%r14, 0x30(%rbx)
140508692: 48 ff 43 40                 	incq	0x40(%rbx)
140508696: 48 8d 5c 24 28              	leaq	0x28(%rsp), %rbx
14050869b: 48 89 d9                    	movq	%rbx, %rcx
14050869e: e8 6d 2a 39 00              	callq	0x14089b110 <.text+0x89a110>
1405086a3: 48 8b 46 10                 	movq	0x10(%rsi), %rax
1405086a7: 8b 48 1c                    	movl	0x1c(%rax), %ecx
1405086aa: 48 8b 50 10                 	movq	0x10(%rax), %rdx
1405086ae: 89 0a                       	movl	%ecx, (%rdx)
1405086b0: 48 8b 08                    	movq	(%rax), %rcx
1405086b3: 41 b8 c8 01 00 00           	movl	$0x1c8, %r8d            # imm = 0x1C8
1405086b9: 48 89 da                    	movq	%rbx, %rdx
1405086bc: e8 3f b5 41 02              	callq	0x142923c00 <.text+0x2922c00>
1405086c1: 48 8b 46 18                 	movq	0x18(%rsi), %rax
1405086c5: 48 c7 00 00 00 00 00        	movq	$0x0, (%rax)
1405086cc: 48 8d 0d df 5c 4e 02        	leaq	0x24e5cdf(%rip), %rcx   # 0x1429ee3b2
1405086d3: ba 0d 00 00 00              	movl	$0xd, %edx
1405086d8: e8 a3 4d 69 00              	callq	0x140b9d480 <.text+0xb9c480>
1405086dd: 49 89 d1                    	movq	%rdx, %r9
1405086e0: 48 8d 5c 24 28              	leaq	0x28(%rsp), %rbx
1405086e5: 48 89 d9                    	movq	%rbx, %rcx
1405086e8: 48 89 fa                    	movq	%rdi, %rdx
1405086eb: 49 89 c0                    	movq	%rax, %r8
1405086ee: e8 cd 38 18 00              	callq	0x14068bfc0 <.text+0x68afc0>
1405086f3: 48 89 d9                    	movq	%rbx, %rcx
1405086f6: e8 55 96 17 00              	callq	0x140681d50 <.text+0x680d50>
1405086fb: 48 89 c1                    	movq	%rax, %rcx
1405086fe: e8 7d b5 17 00              	callq	0x140683c80 <.text+0x682c80>
140508703: 84 c0                       	testb	%al, %al
140508705: 74 06                       	je	0x14050870d <.text+0x50770d>
140508707: 48 8b 06                    	movq	(%rsi), %rax
14050870a: c6 00 03                    	movb	$0x3, (%rax)
14050870d: 48 81 c4 f0 01 00 00        	addq	$0x1f0, %rsp            # imm = 0x1F0
140508714: 5b                          	popq	%rbx
140508715: 5f                          	popq	%rdi
140508716: 5e                          	popq	%rsi
140508717: 41 5e                       	popq	%r14
140508719: 41 5f                       	popq	%r15
14050871b: c3                          	retq
14050871c: cc                          	int3
14050871d: cc                          	int3
14050871e: cc                          	int3
14050871f: cc                          	int3
140508720: 55                          	pushq	%rbp
140508721: 41 57                       	pushq	%r15
140508723: 41 56                       	pushq	%r14
140508725: 41 54                       	pushq	%r12
140508727: 56                          	pushq	%rsi
140508728: 57                          	pushq	%rdi
140508729: 53                          	pushq	%rbx
14050872a: 48 81 ec 30 01 00 00        	subq	$0x130, %rsp            # imm = 0x130
140508731: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140508739: 48 c7 85 a8 00 00 00 fe ff ff ff    	movq	$-0x2, 0xa8(%rbp)
140508744: 48 89 d6                    	movq	%rdx, %rsi
140508747: 48 89 cf                    	movq	%rcx, %rdi
14050874a: 48 8b 19                    	movq	(%rcx), %rbx
14050874d: 48 8d 4d 28                 	leaq	0x28(%rbp), %rcx
140508751: 48 89 da                    	movq	%rbx, %rdx
140508754: e8 47 41 39 00              	callq	0x14089c8a0 <.text+0x89b8a0>
140508759: 4c 8b 45 40                 	movq	0x40(%rbp), %r8
14050875d: 4d 85 c0                    	testq	%r8, %r8
140508760: 74 24                       	je	0x140508786 <.text+0x507786>
140508762: 4c 8b 4d 48                 	movq	0x48(%rbp), %r9
140508766: 4c 89 4c 24 28              	movq	%r9, 0x28(%rsp)
14050876b: 4c 89 44 24 20              	movq	%r8, 0x20(%rsp)
140508770: c7 44 24 30 00 00 80 42     	movl	$0x42800000, 0x30(%rsp) # imm = 0x42800000
140508778: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
14050877c: 48 89 f2                    	movq	%rsi, %rdx
14050877f: e8 2c 59 e2 ff              	callq	0x14032e0b0 <.text+0x32d0b0>
140508784: eb 39                       	jmp	0x1405087bf <.text+0x5077bf>
140508786: 31 c0                       	xorl	%eax, %eax
140508788: 48 3b 45 28                 	cmpq	0x28(%rbp), %rax
14050878c: 0f 80 4c 02 00 00           	jo	0x1405089de <.text+0x5079de>
140508792: 4c 8b 45 30                 	movq	0x30(%rbp), %r8
140508796: 4c 8b 4d 38                 	movq	0x38(%rbp), %r9
14050879a: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
14050879e: 48 89 f2                    	movq	%rsi, %rdx
1405087a1: e8 5a 37 18 00              	callq	0x14068bf00 <.text+0x68af00>
1405087a6: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
1405087aa: e8 81 95 17 00              	callq	0x140681d30 <.text+0x680d30>
1405087af: f3 0f 10 0d c5 d0 41 02     	movss	0x241d0c5(%rip), %xmm1  # 0x14292587c
1405087b7: 48 89 c1                    	movq	%rax, %rcx
1405087ba: e8 91 db bf ff              	callq	0x140106350 <.text+0x105350>
1405087bf: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
1405087c3: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1405087c9: 48 89 f2                    	movq	%rsi, %rdx
1405087cc: 45 31 c9                    	xorl	%r9d, %r9d
1405087cf: e8 2c 37 18 00              	callq	0x14068bf00 <.text+0x68af00>
1405087d4: 48 8b 47 08                 	movq	0x8(%rdi), %rax
1405087d8: 48 8b 10                    	movq	(%rax), %rdx
1405087db: 48 89 d9                    	movq	%rbx, %rcx
1405087de: e8 9d 40 39 00              	callq	0x14089c880 <.text+0x89b880>
1405087e3: 41 89 d0                    	movl	%edx, %r8d
1405087e6: 48 8d 5d f0                 	leaq	-0x10(%rbp), %rbx
1405087ea: 48 89 d9                    	movq	%rbx, %rcx
1405087ed: 89 c2                       	movl	%eax, %edx
1405087ef: e8 7c 41 39 00              	callq	0x14089c970 <.text+0x89b970>
1405087f4: 48 8b 47 10                 	movq	0x10(%rdi), %rax
1405087f8: 66 8b 38                    	movw	(%rax), %di
1405087fb: 66 44 8b 70 02              	movw	0x2(%rax), %r14w
140508800: 48 8d 0d f9 5a 4e 02        	leaq	0x24e5af9(%rip), %rcx   # 0x1429ee300
140508807: ba 20 00 00 00              	movl	$0x20, %edx
14050880c: e8 6f 4c 69 00              	callq	0x140b9d480 <.text+0xb9c480>
140508811: 48 89 45 08                 	movq	%rax, 0x8(%rbp)
140508815: 48 89 55 10                 	movq	%rdx, 0x10(%rbp)
140508819: 4c 8d 7d 08                 	leaq	0x8(%rbp), %r15
14050881d: 4c 89 bd 80 00 00 00        	movq	%r15, 0x80(%rbp)
140508824: 48 8d 05 65 16 c0 ff        	leaq	-0x3fe99b(%rip), %rax   # 0x140109e90 <.text+0x108e90>
14050882b: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
140508832: 48 8d 05 a7 5b 4e 02        	leaq	0x24e5ba7(%rip), %rax   # 0x1429ee3e0
140508839: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
14050883d: 48 c7 45 58 02 00 00 00     	movq	$0x2, 0x58(%rbp)
140508845: 4c 8d a5 80 00 00 00        	leaq	0x80(%rbp), %r12
14050884c: 4c 89 65 60                 	movq	%r12, 0x60(%rbp)
140508850: 48 c7 45 68 01 00 00 00     	movq	$0x1, 0x68(%rbp)
140508858: 48 c7 45 70 00 00 00 00     	movq	$0x0, 0x70(%rbp)
140508860: 48 8d 4d d8                 	leaq	-0x28(%rbp), %rcx
140508864: 48 8d 55 50                 	leaq	0x50(%rbp), %rdx
140508868: e8 23 17 87 00              	callq	0x140d79f90 <.text+0xd78f90>
14050886d: 66 83 ff 06                 	cmpw	$0x6, %di
140508871: 75 18                       	jne	0x14050888b <.text+0x50788b>
140508873: 48 8b 45 e8                 	movq	-0x18(%rbp), %rax
140508877: 48 89 85 90 00 00 00        	movq	%rax, 0x90(%rbp)
14050887e: 0f 10 45 d8                 	movups	-0x28(%rbp), %xmm0
140508882: 0f 29 85 80 00 00 00        	movaps	%xmm0, 0x80(%rbp)
140508889: eb 45                       	jmp	0x1405088d0 <.text+0x5078d0>
14050888b: 48 8b 45 d8                 	movq	-0x28(%rbp), %rax
14050888f: 48 89 85 a0 00 00 00        	movq	%rax, 0xa0(%rbp)
140508896: 48 8b 45 e0                 	movq	-0x20(%rbp), %rax
14050889a: 48 89 85 98 00 00 00        	movq	%rax, 0x98(%rbp)
1405088a1: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
1405088a8: 89 fa                       	movl	%edi, %edx
1405088aa: 45 89 f0                    	movl	%r14d, %r8d
1405088ad: e8 be 40 39 00              	callq	0x14089c970 <.text+0x89b970>
1405088b2: 48 8b 95 a0 00 00 00        	movq	0xa0(%rbp), %rdx
1405088b9: 48 85 d2                    	testq	%rdx, %rdx
1405088bc: 74 12                       	je	0x1405088d0 <.text+0x5078d0>
1405088be: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1405088c4: 48 8b 8d 98 00 00 00        	movq	0x98(%rbp), %rcx
1405088cb: e8 50 b3 10 00              	callq	0x140613c20 <.text+0x612c20>
1405088d0: 48 89 5d 08                 	movq	%rbx, 0x8(%rbp)
1405088d4: 48 8d 05 f5 31 b3 ff        	leaq	-0x4cce0b(%rip), %rax   # 0x14003bad0 <.text+0x3aad0>
1405088db: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
1405088df: 4c 89 65 18                 	movq	%r12, 0x18(%rbp)
1405088e3: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
1405088e7: 48 8d 05 62 5a 4e 02        	leaq	0x24e5a62(%rip), %rax   # 0x1429ee350
1405088ee: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
1405088f2: 48 c7 45 58 02 00 00 00     	movq	$0x2, 0x58(%rbp)
1405088fa: 4c 89 7d 60                 	movq	%r15, 0x60(%rbp)
1405088fe: 48 c7 45 68 02 00 00 00     	movq	$0x2, 0x68(%rbp)
140508906: 48 c7 45 70 00 00 00 00     	movq	$0x0, 0x70(%rbp)
14050890e: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
140508912: 48 8d 55 50                 	leaq	0x50(%rbp), %rdx
140508916: e8 75 16 87 00              	callq	0x140d79f90 <.text+0xd78f90>
14050891b: 48 8b 45 c0                 	movq	-0x40(%rbp), %rax
14050891f: 48 89 85 a0 00 00 00        	movq	%rax, 0xa0(%rbp)
140508926: 4c 8b 45 c8                 	movq	-0x38(%rbp), %r8
14050892a: 4c 8b 4d d0                 	movq	-0x30(%rbp), %r9
14050892e: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
140508932: 48 89 f2                    	movq	%rsi, %rdx
140508935: 4c 89 85 98 00 00 00        	movq	%r8, 0x98(%rbp)
14050893c: e8 bf 35 18 00              	callq	0x14068bf00 <.text+0x68af00>
140508941: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
140508945: e8 e6 93 17 00              	callq	0x140681d30 <.text+0x680d30>
14050894a: f3 0f 10 0d 2a cf 41 02     	movss	0x241cf2a(%rip), %xmm1  # 0x14292587c
140508952: 48 89 c1                    	movq	%rax, %rcx
140508955: e8 f6 d9 bf ff              	callq	0x140106350 <.text+0x105350>
14050895a: 48 8b 95 a0 00 00 00        	movq	0xa0(%rbp), %rdx
140508961: 48 85 d2                    	testq	%rdx, %rdx
140508964: 74 12                       	je	0x140508978 <.text+0x507978>
140508966: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14050896c: 48 8b 8d 98 00 00 00        	movq	0x98(%rbp), %rcx
140508973: e8 a8 b2 10 00              	callq	0x140613c20 <.text+0x612c20>
140508978: 48 8b 95 80 00 00 00        	movq	0x80(%rbp), %rdx
14050897f: 48 85 d2                    	testq	%rdx, %rdx
140508982: 74 12                       	je	0x140508996 <.text+0x507996>
140508984: 48 8b 8d 88 00 00 00        	movq	0x88(%rbp), %rcx
14050898b: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140508991: e8 8a b2 10 00              	callq	0x140613c20 <.text+0x612c20>
140508996: 48 8b 55 f0                 	movq	-0x10(%rbp), %rdx
14050899a: 48 85 d2                    	testq	%rdx, %rdx
14050899d: 74 0f                       	je	0x1405089ae <.text+0x5079ae>
14050899f: 48 8b 4d f8                 	movq	-0x8(%rbp), %rcx
1405089a3: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1405089a9: e8 72 b2 10 00              	callq	0x140613c20 <.text+0x612c20>
1405089ae: 48 8b 55 28                 	movq	0x28(%rbp), %rdx
1405089b2: 48 89 d0                    	movq	%rdx, %rax
1405089b5: 48 f7 d8                    	negq	%rax
1405089b8: 70 11                       	jo	0x1405089cb <.text+0x5079cb>
1405089ba: 73 0f                       	jae	0x1405089cb <.text+0x5079cb>
1405089bc: 48 8b 4d 30                 	movq	0x30(%rbp), %rcx
1405089c0: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1405089c6: e8 55 b2 10 00              	callq	0x140613c20 <.text+0x612c20>
1405089cb: 90                          	nop
1405089cc: 48 81 c4 30 01 00 00        	addq	$0x130, %rsp            # imm = 0x130
1405089d3: 5b                          	popq	%rbx
1405089d4: 5f                          	popq	%rdi
1405089d5: 5e                          	popq	%rsi
1405089d6: 41 5c                       	popq	%r12
1405089d8: 41 5e                       	popq	%r14
1405089da: 41 5f                       	popq	%r15
1405089dc: 5d                          	popq	%rbp
1405089dd: c3                          	retq
1405089de: 48 8d 0d db 59 4e 02        	leaq	0x24e59db(%rip), %rcx   # 0x1429ee3c0
1405089e5: e8 26 09 3d 02              	callq	0x1428d9310 <.text+0x28d8310>
1405089ea: 0f 0b                       	ud2
1405089ec: 0f 1f 40 00                 	nopl	(%rax)
1405089f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1405089f5: 55                          	pushq	%rbp
1405089f6: 41 57                       	pushq	%r15
1405089f8: 41 56                       	pushq	%r14
1405089fa: 41 54                       	pushq	%r12
1405089fc: 56                          	pushq	%rsi
1405089fd: 57                          	pushq	%rdi
1405089fe: 53                          	pushq	%rbx
1405089ff: 48 83 ec 40                 	subq	$0x40, %rsp
140508a03: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140508a0a: 48 8b 55 28                 	movq	0x28(%rbp), %rdx
140508a0e: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
140508a16: 48 85 c0                    	testq	%rax, %rax
140508a19: 74 0f                       	je	0x140508a2a <.text+0x507a2a>
140508a1b: 48 8b 4d 30                 	movq	0x30(%rbp), %rcx
140508a1f: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140508a25: e8 f6 b1 10 00              	callq	0x140613c20 <.text+0x612c20>
140508a2a: 90                          	nop
140508a2b: 48 83 c4 40                 	addq	$0x40, %rsp
140508a2f: 5b                          	popq	%rbx
140508a30: 5f                          	popq	%rdi
140508a31: 5e                          	popq	%rsi
140508a32: 41 5c                       	popq	%r12
140508a34: 41 5e                       	popq	%r14
140508a36: 41 5f                       	popq	%r15
140508a38: 5d                          	popq	%rbp
140508a39: c3                          	retq
140508a3a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140508a40: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140508a45: 55                          	pushq	%rbp
140508a46: 41 57                       	pushq	%r15
140508a48: 41 56                       	pushq	%r14
140508a4a: 41 54                       	pushq	%r12
140508a4c: 56                          	pushq	%rsi
140508a4d: 57                          	pushq	%rdi
140508a4e: 53                          	pushq	%rbx
140508a4f: 48 83 ec 40                 	subq	$0x40, %rsp
140508a53: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140508a5a: 48 8b 55 f0                 	movq	-0x10(%rbp), %rdx
140508a5e: 48 85 d2                    	testq	%rdx, %rdx
140508a61: 74 0f                       	je	0x140508a72 <.text+0x507a72>
140508a63: 48 8b 4d f8                 	movq	-0x8(%rbp), %rcx
140508a67: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140508a6d: e8 ae b1 10 00              	callq	0x140613c20 <.text+0x612c20>
140508a72: 90                          	nop
140508a73: 48 83 c4 40                 	addq	$0x40, %rsp
140508a77: 5b                          	popq	%rbx
140508a78: 5f                          	popq	%rdi
140508a79: 5e                          	popq	%rsi
140508a7a: 41 5c                       	popq	%r12
140508a7c: 41 5e                       	popq	%r14
140508a7e: 41 5f                       	popq	%r15
140508a80: 5d                          	popq	%rbp
140508a81: c3                          	retq
140508a82: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140508a90: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140508a95: 55                          	pushq	%rbp
140508a96: 41 57                       	pushq	%r15
140508a98: 41 56                       	pushq	%r14
140508a9a: 41 54                       	pushq	%r12
140508a9c: 56                          	pushq	%rsi
140508a9d: 57                          	pushq	%rdi
140508a9e: 53                          	pushq	%rbx
140508a9f: 48 83 ec 40                 	subq	$0x40, %rsp
140508aa3: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140508aaa: 48 83 bd a0 00 00 00 00     	cmpq	$0x0, 0xa0(%rbp)
140508ab2: 74 19                       	je	0x140508acd <.text+0x507acd>
140508ab4: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140508aba: 48 8b 8d 98 00 00 00        	movq	0x98(%rbp), %rcx
140508ac1: 48 8b 95 a0 00 00 00        	movq	0xa0(%rbp), %rdx
140508ac8: e8 53 b1 10 00              	callq	0x140613c20 <.text+0x612c20>
140508acd: 90                          	nop
140508ace: 48 83 c4 40                 	addq	$0x40, %rsp
140508ad2: 5b                          	popq	%rbx
140508ad3: 5f                          	popq	%rdi
140508ad4: 5e                          	popq	%rsi
140508ad5: 41 5c                       	popq	%r12
140508ad7: 41 5e                       	popq	%r14
140508ad9: 41 5f                       	popq	%r15
140508adb: 5d                          	popq	%rbp
140508adc: c3                          	retq
140508add: 0f 1f 00                    	nopl	(%rax)
140508ae0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140508ae5: 55                          	pushq	%rbp
140508ae6: 41 57                       	pushq	%r15
140508ae8: 41 56                       	pushq	%r14
140508aea: 41 54                       	pushq	%r12
140508aec: 56                          	pushq	%rsi
140508aed: 57                          	pushq	%rdi
140508aee: 53                          	pushq	%rbx
140508aef: 48 83 ec 40                 	subq	$0x40, %rsp
140508af3: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140508afa: 48 8b 95 80 00 00 00        	movq	0x80(%rbp), %rdx
140508b01: 48 85 d2                    	testq	%rdx, %rdx
140508b04: 74 12                       	je	0x140508b18 <.text+0x507b18>
140508b06: 48 8b 8d 88 00 00 00        	movq	0x88(%rbp), %rcx
140508b0d: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140508b13: e8 08 b1 10 00              	callq	0x140613c20 <.text+0x612c20>
140508b18: 90                          	nop
140508b19: 48 83 c4 40                 	addq	$0x40, %rsp
140508b1d: 5b                          	popq	%rbx
140508b1e: 5f                          	popq	%rdi
140508b1f: 5e                          	popq	%rsi
140508b20: 41 5c                       	popq	%r12
140508b22: 41 5e                       	popq	%r14
140508b24: 41 5f                       	popq	%r15
140508b26: 5d                          	popq	%rbp
140508b27: c3                          	retq
140508b28: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
140508b30: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140508b35: 55                          	pushq	%rbp
140508b36: 41 57                       	pushq	%r15
140508b38: 41 56                       	pushq	%r14
140508b3a: 41 54                       	pushq	%r12
140508b3c: 56                          	pushq	%rsi
140508b3d: 57                          	pushq	%rdi
140508b3e: 53                          	pushq	%rbx
140508b3f: 48 83 ec 40                 	subq	$0x40, %rsp
140508b43: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140508b4a: 48 83 bd a0 00 00 00 00     	cmpq	$0x0, 0xa0(%rbp)
140508b52: 74 19                       	je	0x140508b6d <.text+0x507b6d>
140508b54: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140508b5a: 48 8b 8d 98 00 00 00        	movq	0x98(%rbp), %rcx
140508b61: 48 8b 95 a0 00 00 00        	movq	0xa0(%rbp), %rdx
140508b68: e8 b3 b0 10 00              	callq	0x140613c20 <.text+0x612c20>
140508b6d: 90                          	nop
140508b6e: 48 83 c4 40                 	addq	$0x40, %rsp
140508b72: 5b                          	popq	%rbx
140508b73: 5f                          	popq	%rdi
140508b74: 5e                          	popq	%rsi
140508b75: 41 5c                       	popq	%r12
140508b77: 41 5e                       	popq	%r14
140508b79: 41 5f                       	popq	%r15
140508b7b: 5d                          	popq	%rbp
140508b7c: c3                          	retq
140508b7d: cc                          	int3
140508b7e: cc                          	int3
140508b7f: cc                          	int3
140508b80: 41 57                       	pushq	%r15
140508b82: 41 56                       	pushq	%r14
140508b84: 41 54                       	pushq	%r12
140508b86: 56                          	pushq	%rsi
140508b87: 57                          	pushq	%rdi
140508b88: 53                          	pushq	%rbx
140508b89: 48 83 ec 38                 	subq	$0x38, %rsp
140508b8d: 48 89 d7                    	movq	%rdx, %rdi
140508b90: 48 89 ce                    	movq	%rcx, %rsi
140508b93: 48 8b 01                    	movq	(%rcx), %rax
140508b96: 4c 8b 30                    	movq	(%rax), %r14
140508b99: 66 41 83 3e 06              	cmpw	$0x6, (%r14)
140508b9e: 0f 84 a8 00 00 00           	je	0x140508c4c <.text+0x507c4c>
140508ba4: 48 8d 0d c8 57 4e 02        	leaq	0x24e57c8(%rip), %rcx   # 0x1429ee373
140508bab: ba 1e 00 00 00              	movl	$0x1e, %edx
140508bb0: e8 cb 48 69 00              	callq	0x140b9d480 <.text+0xb9c480>
140508bb5: 49 89 d1                    	movq	%rdx, %r9
140508bb8: 48 8d 5c 24 20              	leaq	0x20(%rsp), %rbx
140508bbd: 48 89 d9                    	movq	%rbx, %rcx
140508bc0: 48 89 fa                    	movq	%rdi, %rdx
140508bc3: 49 89 c0                    	movq	%rax, %r8
140508bc6: e8 f5 33 18 00              	callq	0x14068bfc0 <.text+0x68afc0>
140508bcb: 48 89 d9                    	movq	%rbx, %rcx
140508bce: e8 7d 91 17 00              	callq	0x140681d50 <.text+0x680d50>
140508bd3: 48 89 c1                    	movq	%rax, %rcx
140508bd6: e8 a5 b0 17 00              	callq	0x140683c80 <.text+0x682c80>
140508bdb: 84 c0                       	testb	%al, %al
140508bdd: 74 6d                       	je	0x140508c4c <.text+0x507c4c>
140508bdf: 48 8b 46 08                 	movq	0x8(%rsi), %rax
140508be3: c6 00 01                    	movb	$0x1, (%rax)
140508be6: 48 8b 46 10                 	movq	0x10(%rsi), %rax
140508bea: 8b 48 1c                    	movl	0x1c(%rax), %ecx
140508bed: 48 8b 50 10                 	movq	0x10(%rax), %rdx
140508bf1: 89 0a                       	movl	%ecx, (%rdx)
140508bf3: 48 8b 18                    	movq	(%rax), %rbx
140508bf6: 4c 8b 7b 30                 	movq	0x30(%rbx), %r15
140508bfa: 4c 8b 63 40                 	movq	0x40(%rbx), %r12
140508bfe: 4c 3b 7b 20                 	cmpq	0x20(%rbx), %r15
140508c02: 75 10                       	jne	0x140508c14 <.text+0x507c14>
140508c04: 48 8d 4b 20                 	leaq	0x20(%rbx), %rcx
140508c08: 48 8d 15 c1 48 45 02        	leaq	0x24548c1(%rip), %rdx   # 0x14295d4d0
140508c0f: e8 5c 9c 05 00              	callq	0x140562870 <.text+0x561870>
140508c14: 48 8b 43 28                 	movq	0x28(%rbx), %rax
140508c18: 4e 89 24 f8                 	movq	%r12, (%rax,%r15,8)
140508c1c: 49 ff c7                    	incq	%r15
140508c1f: 4c 89 7b 30                 	movq	%r15, 0x30(%rbx)
140508c23: 48 ff 43 40                 	incq	0x40(%rbx)
140508c27: 48 8b 46 18                 	movq	0x18(%rsi), %rax
140508c2b: 8b 48 1c                    	movl	0x1c(%rax), %ecx
140508c2e: 48 8b 50 10                 	movq	0x10(%rax), %rdx
140508c32: 89 0a                       	movl	%ecx, (%rdx)
140508c34: 48 8b 08                    	movq	(%rax), %rcx
140508c37: 48 8b 46 20                 	movq	0x20(%rsi), %rax
140508c3b: 0f b6 10                    	movzbl	(%rax), %edx
140508c3e: 45 0f b7 06                 	movzwl	(%r14), %r8d
140508c42: 45 0f b7 4e 02              	movzwl	0x2(%r14), %r9d
140508c47: e8 34 26 39 00              	callq	0x14089b280 <.text+0x89a280>
140508c4c: 48 8d 0d 3e 57 4e 02        	leaq	0x24e573e(%rip), %rcx   # 0x1429ee391
140508c53: ba 06 00 00 00              	movl	$0x6, %edx
140508c58: e8 23 48 69 00              	callq	0x140b9d480 <.text+0xb9c480>
140508c5d: 49 89 d1                    	movq	%rdx, %r9
140508c60: 48 8d 5c 24 20              	leaq	0x20(%rsp), %rbx
140508c65: 48 89 d9                    	movq	%rbx, %rcx
140508c68: 48 89 fa                    	movq	%rdi, %rdx
140508c6b: 49 89 c0                    	movq	%rax, %r8
140508c6e: e8 4d 33 18 00              	callq	0x14068bfc0 <.text+0x68afc0>
140508c73: 48 89 d9                    	movq	%rbx, %rcx
140508c76: e8 d5 90 17 00              	callq	0x140681d50 <.text+0x680d50>
140508c7b: 48 89 c1                    	movq	%rax, %rcx
140508c7e: e8 fd af 17 00              	callq	0x140683c80 <.text+0x682c80>
140508c83: 84 c0                       	testb	%al, %al
140508c85: 74 07                       	je	0x140508c8e <.text+0x507c8e>
140508c87: 48 8b 46 08                 	movq	0x8(%rsi), %rax
140508c8b: c6 00 01                    	movb	$0x1, (%rax)
140508c8e: 48 83 c4 38                 	addq	$0x38, %rsp
140508c92: 5b                          	popq	%rbx
140508c93: 5f                          	popq	%rdi
140508c94: 5e                          	popq	%rsi
140508c95: 41 5c                       	popq	%r12
140508c97: 41 5e                       	popq	%r14
140508c99: 41 5f                       	popq	%r15
140508c9b: c3                          	retq
140508c9c: cc                          	int3
140508c9d: cc                          	int3
140508c9e: cc                          	int3
140508c9f: cc                          	int3
140508ca0: 55                          	pushq	%rbp
140508ca1: 41 57                       	pushq	%r15
140508ca3: 41 56                       	pushq	%r14
140508ca5: 41 55                       	pushq	%r13
140508ca7: 41 54                       	pushq	%r12
140508ca9: 56                          	pushq	%rsi
140508caa: 57                          	pushq	%rdi
140508cab: 53                          	pushq	%rbx
140508cac: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp

; SOURCE /tmp/key-header.asm

B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1404ff030: 55                          	pushq	%rbp
1404ff031: 41 57                       	pushq	%r15
1404ff033: 41 56                       	pushq	%r14
1404ff035: 41 55                       	pushq	%r13
1404ff037: 41 54                       	pushq	%r12
1404ff039: 56                          	pushq	%rsi
1404ff03a: 57                          	pushq	%rdi
1404ff03b: 53                          	pushq	%rbx
1404ff03c: 48 81 ec 18 01 00 00        	subq	$0x118, %rsp            # imm = 0x118
1404ff043: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1404ff04b: 48 c7 85 90 00 00 00 fe ff ff ff    	movq	$-0x2, 0x90(%rbp)
1404ff056: 4c 89 8d 88 00 00 00        	movq	%r9, 0x88(%rbp)
1404ff05d: 48 89 cb                    	movq	%rcx, %rbx
1404ff060: 48 89 55 d8                 	movq	%rdx, -0x28(%rbp)
1404ff064: 4c 89 45 e0                 	movq	%r8, -0x20(%rbp)
1404ff068: 48 8d 79 10                 	leaq	0x10(%rcx), %rdi
1404ff06c: 48 89 f9                    	movq	%rdi, %rcx
1404ff06f: e8 3c 78 18 00              	callq	0x1406868b0 <.text+0x6858b0>
1404ff074: 49 89 c4                    	movq	%rax, %r12
1404ff077: 4c 8d 7d d8                 	leaq	-0x28(%rbp), %r15
1404ff07b: 4c 89 f9                    	movq	%r15, %rcx
1404ff07e: e8 8d 4b 18 00              	callq	0x140683c10 <.text+0x682c10>
1404ff083: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
1404ff087: 4c 8d 75 68                 	leaq	0x68(%rbp), %r14
1404ff08b: 4c 89 f1                    	movq	%r14, %rcx
1404ff08e: e8 3d 95 b8 ff              	callq	0x1400885d0 <.text+0x875d0>
1404ff093: 48 89 45 e8                 	movq	%rax, -0x18(%rbp)
1404ff097: 49 bd 3b 81 28 ff 69 31 22 41       	movabsq	$0x41223169ff28813b, %r13 # imm = 0x41223169FF28813B
1404ff0a1: 4c 89 6d f0                 	movq	%r13, -0x10(%rbp)
1404ff0a5: 48 be d9 68 a9 a2 68 72 9b a7       	movabsq	$-0x58648d975d569727, %rsi # imm = 0xA79B7268A2A968D9
1404ff0af: 48 89 75 f8                 	movq	%rsi, -0x8(%rbp)
1404ff0b3: 48 8d 4d e8                 	leaq	-0x18(%rbp), %rcx
1404ff0b7: e8 64 ee 0b 00              	callq	0x1405bdf20 <.text+0x5bcf20>
1404ff0bc: 4c 89 65 e8                 	movq	%r12, -0x18(%rbp)
1404ff0c0: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
1404ff0c4: 48 8d 4d e8                 	leaq	-0x18(%rbp), %rcx
1404ff0c8: e8 43 e8 0b 00              	callq	0x1405bd910 <.text+0x5bc910>
1404ff0cd: 49 89 c4                    	movq	%rax, %r12
1404ff0d0: 48 89 f9                    	movq	%rdi, %rcx
1404ff0d3: 48 89 c2                    	movq	%rax, %rdx
1404ff0d6: e8 75 73 18 00              	callq	0x140686450 <.text+0x685450>
1404ff0db: 84 c0                       	testb	%al, %al
1404ff0dd: 74 7e                       	je	0x1404ff15d <.text+0x4fe15d>
1404ff0df: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
1404ff0e3: 48 89 fa                    	movq	%rdi, %rdx
1404ff0e6: 4d 89 e0                    	movq	%r12, %r8
1404ff0e9: e8 42 74 18 00              	callq	0x140686530 <.text+0x685530>
1404ff0ee: 48 c7 45 e8 00 00 00 00     	movq	$0x0, -0x18(%rbp)
1404ff0f6: 48 c7 45 f0 01 00 00 00     	movq	$0x1, -0x10(%rbp)
1404ff0fe: 48 c7 45 f8 00 00 00 00     	movq	$0x0, -0x8(%rbp)
1404ff106: 48 8d 4d 68                 	leaq	0x68(%rbp), %rcx
1404ff10a: 48 8d 55 e8                 	leaq	-0x18(%rbp), %rdx
1404ff10e: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1404ff114: 45 31 c9                    	xorl	%r9d, %r9d
1404ff117: e8 d4 eb 17 00              	callq	0x14067dcf0 <.text+0x67ccf0>
1404ff11c: f3 0f 10 05 ac 27 4d 02     	movss	0x24d27ac(%rip), %xmm0  # 0x1429d18d0
1404ff124: e8 57 ca b3 ff              	callq	0x14003bb80 <.text+0x3ab80>
1404ff129: 48 8b 85 88 00 00 00        	movq	0x88(%rbp), %rax
1404ff130: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
1404ff135: f3 0f 11 44 24 20           	movss	%xmm0, 0x20(%rsp)
1404ff13b: c7 44 24 28 00 00 70 42     	movl	$0x42700000, 0x28(%rsp) # imm = 0x42700000
1404ff143: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
1404ff147: 4c 8d 4d 68                 	leaq	0x68(%rbp), %r9
1404ff14b: ba 01 00 00 00              	movl	$0x1, %edx
1404ff150: 45 31 c0                    	xorl	%r8d, %r8d
1404ff153: e8 48 fa e2 ff              	callq	0x14032eba0 <.text+0x32dba0>
1404ff158: e9 6c 01 00 00              	jmp	0x1404ff2c9 <.text+0x4fe2c9>
1404ff15d: 48 8b 05 44 d2 12 03        	movq	0x312d244(%rip), %rax   # 0x14362c3a8
1404ff164: 48 8b 00                    	movq	(%rax), %rax
1404ff167: 48 85 c0                    	testq	%rax, %rax
1404ff16a: 0f 84 a5 00 00 00           	je	0x1404ff215 <.text+0x4fe215>
1404ff170: 48 89 5d 68                 	movq	%rbx, 0x68(%rbp)
1404ff174: 48 8d 05 25 47 03 00        	leaq	0x34725(%rip), %rax     # 0x1405338a0 <.text+0x5328a0>
1404ff17b: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
1404ff17f: 4c 89 7d 78                 	movq	%r15, 0x78(%rbp)
1404ff183: 48 89 85 80 00 00 00        	movq	%rax, 0x80(%rbp)
1404ff18a: 48 8d 0d c7 eb 4e 02        	leaq	0x24eebc7(%rip), %rcx   # 0x1429edd58
1404ff191: e8 ea eb 17 00              	callq	0x14067dd80 <.text+0x67cd80>
1404ff196: 0f 10 00                    	movups	(%rax), %xmm0
1404ff199: 8b 40 10                    	movl	0x10(%rax), %eax
1404ff19c: 48 c7 45 18 01 00 00 00     	movq	$0x1, 0x18(%rbp)
1404ff1a4: 48 8d 0d 7d eb 4e 02        	leaq	0x24eeb7d(%rip), %rcx   # 0x1429edd28
1404ff1ab: 48 89 4d 20                 	movq	%rcx, 0x20(%rbp)
1404ff1af: 48 c7 45 28 14 00 00 00     	movq	$0x14, 0x28(%rbp)
1404ff1b7: 48 8d 15 da ea 4e 02        	leaq	0x24eeada(%rip), %rdx   # 0x1429edc98
1404ff1be: 48 89 55 38                 	movq	%rdx, 0x38(%rbp)
1404ff1c2: 48 c7 45 40 02 00 00 00     	movq	$0x2, 0x40(%rbp)
1404ff1ca: 4c 89 75 48                 	movq	%r14, 0x48(%rbp)
1404ff1ce: 48 c7 45 50 02 00 00 00     	movq	$0x2, 0x50(%rbp)
1404ff1d6: 48 c7 45 58 00 00 00 00     	movq	$0x0, 0x58(%rbp)
1404ff1de: 48 c7 45 e8 00 00 00 00     	movq	$0x0, -0x18(%rbp)
1404ff1e6: 48 89 4d f0                 	movq	%rcx, -0x10(%rbp)
1404ff1ea: 48 c7 45 f8 14 00 00 00     	movq	$0x14, -0x8(%rbp)
1404ff1f2: 48 c7 45 00 00 00 00 00     	movq	$0x0, (%rbp)
1404ff1fa: 0f 11 45 08                 	movups	%xmm0, 0x8(%rbp)
1404ff1fe: c7 45 30 01 00 00 00        	movl	$0x1, 0x30(%rbp)
1404ff205: 89 45 34                    	movl	%eax, 0x34(%rbp)
1404ff208: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
1404ff20c: 48 8d 55 e8                 	leaq	-0x18(%rbp), %rdx
1404ff210: e8 1b b1 84 00              	callq	0x140d4a330 <.text+0xd49330>
1404ff215: 0f 10 45 d8                 	movups	-0x28(%rbp), %xmm0
1404ff219: 48 8b 5b 18                 	movq	0x18(%rbx), %rbx
1404ff21d: 0f 11 45 68                 	movups	%xmm0, 0x68(%rbp)
1404ff221: 48 8d 4d 68                 	leaq	0x68(%rbp), %rcx
1404ff225: e8 e6 49 18 00              	callq	0x140683c10 <.text+0x682c10>
1404ff22a: 48 89 45 e8                 	movq	%rax, -0x18(%rbp)
1404ff22e: 4c 89 6d f0                 	movq	%r13, -0x10(%rbp)
1404ff232: 48 89 75 f8                 	movq	%rsi, -0x8(%rbp)
1404ff236: 48 8d 4d e8                 	leaq	-0x18(%rbp), %rcx
1404ff23a: e8 e1 ec 0b 00              	callq	0x1405bdf20 <.text+0x5bcf20>
1404ff23f: 48 89 5d e8                 	movq	%rbx, -0x18(%rbp)
1404ff243: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
1404ff247: 48 8d 4d e8                 	leaq	-0x18(%rbp), %rcx
1404ff24b: e8 c0 e6 0b 00              	callq	0x1405bd910 <.text+0x5bc910>
1404ff250: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
1404ff254: 48 89 fa                    	movq	%rdi, %rdx
1404ff257: 49 89 c0                    	movq	%rax, %r8
1404ff25a: e8 51 6f 18 00              	callq	0x1406861b0 <.text+0x6851b0>
1404ff25f: 48 c7 45 e8 00 00 00 00     	movq	$0x0, -0x18(%rbp)
1404ff267: 48 c7 45 f0 01 00 00 00     	movq	$0x1, -0x10(%rbp)
1404ff26f: 48 c7 45 f8 00 00 00 00     	movq	$0x0, -0x8(%rbp)
1404ff277: 48 8d 4d 68                 	leaq	0x68(%rbp), %rcx
1404ff27b: 48 8d 55 e8                 	leaq	-0x18(%rbp), %rdx
1404ff27f: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1404ff285: 45 31 c9                    	xorl	%r9d, %r9d
1404ff288: e8 63 ea 17 00              	callq	0x14067dcf0 <.text+0x67ccf0>
1404ff28d: f3 0f 10 05 3b 26 4d 02     	movss	0x24d263b(%rip), %xmm0  # 0x1429d18d0
1404ff295: e8 e6 c8 b3 ff              	callq	0x14003bb80 <.text+0x3ab80>
1404ff29a: 48 8b 85 88 00 00 00        	movq	0x88(%rbp), %rax
1404ff2a1: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
1404ff2a6: f3 0f 11 44 24 20           	movss	%xmm0, 0x20(%rsp)
1404ff2ac: c7 44 24 28 00 00 70 42     	movl	$0x42700000, 0x28(%rsp) # imm = 0x42700000
1404ff2b4: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
1404ff2b8: 4c 8d 4d 68                 	leaq	0x68(%rbp), %r9
1404ff2bc: ba 01 00 00 00              	movl	$0x1, %edx
1404ff2c1: 45 31 c0                    	xorl	%r8d, %r8d
1404ff2c4: e8 d7 f8 e2 ff              	callq	0x14032eba0 <.text+0x32dba0>
1404ff2c9: 48 8b 55 e8                 	movq	-0x18(%rbp), %rdx
1404ff2cd: 48 85 d2                    	testq	%rdx, %rdx
1404ff2d0: 74 0f                       	je	0x1404ff2e1 <.text+0x4fe2e1>
1404ff2d2: 48 8b 4d f0                 	movq	-0x10(%rbp), %rcx
1404ff2d6: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1404ff2dc: e8 3f 49 11 00              	callq	0x140613c20 <.text+0x612c20>
1404ff2e1: 90                          	nop
1404ff2e2: 48 81 c4 18 01 00 00        	addq	$0x118, %rsp            # imm = 0x118
1404ff2e9: 5b                          	popq	%rbx
1404ff2ea: 5f                          	popq	%rdi
1404ff2eb: 5e                          	popq	%rsi
1404ff2ec: 41 5c                       	popq	%r12
1404ff2ee: 41 5d                       	popq	%r13
1404ff2f0: 41 5e                       	popq	%r14
1404ff2f2: 41 5f                       	popq	%r15
1404ff2f4: 5d                          	popq	%rbp
1404ff2f5: c3                          	retq
1404ff2f6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)

; SOURCE /tmp/key-header-title.asm

B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140501870: 55                          	pushq	%rbp
140501871: 41 57                       	pushq	%r15
140501873: 41 56                       	pushq	%r14
140501875: 41 55                       	pushq	%r13
140501877: 41 54                       	pushq	%r12
140501879: 56                          	pushq	%rsi
14050187a: 57                          	pushq	%rdi
14050187b: 53                          	pushq	%rbx
14050187c: 48 81 ec 18 01 00 00        	subq	$0x118, %rsp            # imm = 0x118
140501883: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14050188b: 48 c7 85 90 00 00 00 fe ff ff ff    	movq	$-0x2, 0x90(%rbp)
140501896: 48 89 cf                    	movq	%rcx, %rdi
140501899: 48 89 55 c0                 	movq	%rdx, -0x40(%rbp)
14050189d: 4c 89 45 c8                 	movq	%r8, -0x38(%rbp)
1405018a1: 48 8d 71 10                 	leaq	0x10(%rcx), %rsi
1405018a5: 48 89 f1                    	movq	%rsi, %rcx
1405018a8: e8 03 50 18 00              	callq	0x1406868b0 <.text+0x6858b0>
1405018ad: 49 89 c7                    	movq	%rax, %r15
1405018b0: 4c 8d 75 c0                 	leaq	-0x40(%rbp), %r14
1405018b4: 4c 89 f1                    	movq	%r14, %rcx
1405018b7: e8 54 23 18 00              	callq	0x140683c10 <.text+0x682c10>
1405018bc: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
1405018c0: 48 8d 5d 68                 	leaq	0x68(%rbp), %rbx
1405018c4: 48 89 d9                    	movq	%rbx, %rcx
1405018c7: e8 04 6d b8 ff              	callq	0x1400885d0 <.text+0x875d0>
1405018cc: 48 89 45 e8                 	movq	%rax, -0x18(%rbp)
1405018d0: 49 bc 3b 81 28 ff 69 31 22 41       	movabsq	$0x41223169ff28813b, %r12 # imm = 0x41223169FF28813B
1405018da: 4c 89 65 f0                 	movq	%r12, -0x10(%rbp)
1405018de: 49 bd d9 68 a9 a2 68 72 9b a7       	movabsq	$-0x58648d975d569727, %r13 # imm = 0xA79B7268A2A968D9
1405018e8: 4c 89 6d f8                 	movq	%r13, -0x8(%rbp)
1405018ec: 48 8d 4d e8                 	leaq	-0x18(%rbp), %rcx
1405018f0: e8 2b c6 0b 00              	callq	0x1405bdf20 <.text+0x5bcf20>
1405018f5: 4c 89 7d e8                 	movq	%r15, -0x18(%rbp)
1405018f9: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
1405018fd: 48 8d 4d e8                 	leaq	-0x18(%rbp), %rcx
140501901: e8 0a c0 0b 00              	callq	0x1405bd910 <.text+0x5bc910>
140501906: 49 89 c7                    	movq	%rax, %r15
140501909: 48 89 f1                    	movq	%rsi, %rcx
14050190c: 48 89 c2                    	movq	%rax, %rdx
14050190f: e8 3c 4b 18 00              	callq	0x140686450 <.text+0x685450>
140501914: 84 c0                       	testb	%al, %al
140501916: 0f 84 8d 00 00 00           	je	0x1405019a9 <.text+0x5009a9>
14050191c: 48 8d 7d a8                 	leaq	-0x58(%rbp), %rdi
140501920: 48 89 f9                    	movq	%rdi, %rcx
140501923: 48 89 f2                    	movq	%rsi, %rdx
140501926: 4d 89 f8                    	movq	%r15, %r8
140501929: e8 02 4c 18 00              	callq	0x140686530 <.text+0x685530>
14050192e: 48 8d 0d 8c c9 4e 02        	leaq	0x24ec98c(%rip), %rcx   # 0x1429ee2c1
140501935: ba 22 00 00 00              	movl	$0x22, %edx
14050193a: e8 41 bb 69 00              	callq	0x140b9d480 <.text+0xb9c480>
14050193f: 49 89 d0                    	movq	%rdx, %r8
140501942: 48 8d 4d e8                 	leaq	-0x18(%rbp), %rcx
140501946: 48 89 c2                    	movq	%rax, %rdx
140501949: e8 f2 93 87 00              	callq	0x140d7ad40 <.text+0xd79d40>
14050194e: 4c 8b 45 f0                 	movq	-0x10(%rbp), %r8
140501952: 4c 8b 4d f8                 	movq	-0x8(%rbp), %r9
140501956: 48 8d 4d 68                 	leaq	0x68(%rbp), %rcx
14050195a: 48 89 fa                    	movq	%rdi, %rdx
14050195d: 4c 89 85 88 00 00 00        	movq	%r8, 0x88(%rbp)
140501964: e8 97 a5 18 00              	callq	0x14068bf00 <.text+0x68af00>
140501969: 48 8d 4d 68                 	leaq	0x68(%rbp), %rcx
14050196d: e8 be 03 18 00              	callq	0x140681d30 <.text+0x680d30>
140501972: f3 0f 10 0d 1a c2 4e 02     	movss	0x24ec21a(%rip), %xmm1  # 0x1429edb94
14050197a: 48 89 c1                    	movq	%rax, %rcx
14050197d: e8 ce 49 c0 ff              	callq	0x140106350 <.text+0x105350>
140501982: 48 8b 45 78                 	movq	0x78(%rbp), %rax
140501986: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
14050198a: 0f 10 45 68                 	movups	0x68(%rbp), %xmm0
14050198e: 0f 29 45 d0                 	movaps	%xmm0, -0x30(%rbp)
140501992: 48 8d 4d 68                 	leaq	0x68(%rbp), %rcx
140501996: 48 8d 55 d0                 	leaq	-0x30(%rbp), %rdx
14050199a: 66 41 b8 20 03              	movw	$0x320, %r8w            # imm = 0x320
14050199f: e8 bc ca 17 00              	callq	0x14067e460 <.text+0x67d460>
1405019a4: e9 7b 01 00 00              	jmp	0x140501b24 <.text+0x500b24>
1405019a9: 48 8b 05 f8 a9 12 03        	movq	0x312a9f8(%rip), %rax   # 0x14362c3a8
1405019b0: 48 8b 00                    	movq	(%rax), %rax
1405019b3: 48 85 c0                    	testq	%rax, %rax
1405019b6: 0f 84 a5 00 00 00           	je	0x140501a61 <.text+0x500a61>
1405019bc: 48 89 7d 68                 	movq	%rdi, 0x68(%rbp)
1405019c0: 48 8d 05 d9 1e 03 00        	leaq	0x31ed9(%rip), %rax     # 0x1405338a0 <.text+0x5328a0>
1405019c7: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
1405019cb: 4c 89 75 78                 	movq	%r14, 0x78(%rbp)
1405019cf: 48 89 85 80 00 00 00        	movq	%rax, 0x80(%rbp)
1405019d6: 48 8d 0d 7b c3 4e 02        	leaq	0x24ec37b(%rip), %rcx   # 0x1429edd58
1405019dd: e8 9e c3 17 00              	callq	0x14067dd80 <.text+0x67cd80>
1405019e2: 0f 10 00                    	movups	(%rax), %xmm0
1405019e5: 8b 40 10                    	movl	0x10(%rax), %eax
1405019e8: 48 c7 45 18 01 00 00 00     	movq	$0x1, 0x18(%rbp)
1405019f0: 48 8d 0d 31 c3 4e 02        	leaq	0x24ec331(%rip), %rcx   # 0x1429edd28
1405019f7: 48 89 4d 20                 	movq	%rcx, 0x20(%rbp)
1405019fb: 48 c7 45 28 14 00 00 00     	movq	$0x14, 0x28(%rbp)
140501a03: 48 8d 15 8e c2 4e 02        	leaq	0x24ec28e(%rip), %rdx   # 0x1429edc98
140501a0a: 48 89 55 38                 	movq	%rdx, 0x38(%rbp)
140501a0e: 48 c7 45 40 02 00 00 00     	movq	$0x2, 0x40(%rbp)
140501a16: 48 89 5d 48                 	movq	%rbx, 0x48(%rbp)
140501a1a: 48 c7 45 50 02 00 00 00     	movq	$0x2, 0x50(%rbp)
140501a22: 48 c7 45 58 00 00 00 00     	movq	$0x0, 0x58(%rbp)
140501a2a: 48 c7 45 e8 00 00 00 00     	movq	$0x0, -0x18(%rbp)
140501a32: 48 89 4d f0                 	movq	%rcx, -0x10(%rbp)
140501a36: 48 c7 45 f8 14 00 00 00     	movq	$0x14, -0x8(%rbp)
140501a3e: 48 c7 45 00 00 00 00 00     	movq	$0x0, (%rbp)
140501a46: 0f 11 45 08                 	movups	%xmm0, 0x8(%rbp)
140501a4a: c7 45 30 01 00 00 00        	movl	$0x1, 0x30(%rbp)
140501a51: 89 45 34                    	movl	%eax, 0x34(%rbp)
140501a54: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
140501a58: 48 8d 55 e8                 	leaq	-0x18(%rbp), %rdx
140501a5c: e8 cf 88 84 00              	callq	0x140d4a330 <.text+0xd49330>
140501a61: 0f 10 45 c0                 	movups	-0x40(%rbp), %xmm0
140501a65: 48 8b 7f 18                 	movq	0x18(%rdi), %rdi
140501a69: 0f 11 45 68                 	movups	%xmm0, 0x68(%rbp)
140501a6d: 48 8d 4d 68                 	leaq	0x68(%rbp), %rcx
140501a71: e8 9a 21 18 00              	callq	0x140683c10 <.text+0x682c10>
140501a76: 48 89 45 e8                 	movq	%rax, -0x18(%rbp)
140501a7a: 4c 89 65 f0                 	movq	%r12, -0x10(%rbp)
140501a7e: 4c 89 6d f8                 	movq	%r13, -0x8(%rbp)
140501a82: 48 8d 4d e8                 	leaq	-0x18(%rbp), %rcx
140501a86: e8 95 c4 0b 00              	callq	0x1405bdf20 <.text+0x5bcf20>
140501a8b: 48 89 7d e8                 	movq	%rdi, -0x18(%rbp)
140501a8f: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
140501a93: 48 8d 4d e8                 	leaq	-0x18(%rbp), %rcx
140501a97: e8 74 be 0b 00              	callq	0x1405bd910 <.text+0x5bc910>
140501a9c: 48 8d 7d a8                 	leaq	-0x58(%rbp), %rdi
140501aa0: 48 89 f9                    	movq	%rdi, %rcx
140501aa3: 48 89 f2                    	movq	%rsi, %rdx
140501aa6: 49 89 c0                    	movq	%rax, %r8
140501aa9: e8 02 47 18 00              	callq	0x1406861b0 <.text+0x6851b0>
140501aae: 48 8d 0d 0c c8 4e 02        	leaq	0x24ec80c(%rip), %rcx   # 0x1429ee2c1
140501ab5: ba 22 00 00 00              	movl	$0x22, %edx
140501aba: e8 c1 b9 69 00              	callq	0x140b9d480 <.text+0xb9c480>
140501abf: 49 89 d0                    	movq	%rdx, %r8
140501ac2: 48 8d 4d e8                 	leaq	-0x18(%rbp), %rcx
140501ac6: 48 89 c2                    	movq	%rax, %rdx
140501ac9: e8 72 92 87 00              	callq	0x140d7ad40 <.text+0xd79d40>
140501ace: 4c 8b 45 f0                 	movq	-0x10(%rbp), %r8
140501ad2: 4c 8b 4d f8                 	movq	-0x8(%rbp), %r9
140501ad6: 48 8d 4d 68                 	leaq	0x68(%rbp), %rcx
140501ada: 48 89 fa                    	movq	%rdi, %rdx
140501add: 4c 89 85 88 00 00 00        	movq	%r8, 0x88(%rbp)
140501ae4: e8 17 a4 18 00              	callq	0x14068bf00 <.text+0x68af00>
140501ae9: 48 8d 4d 68                 	leaq	0x68(%rbp), %rcx
140501aed: e8 3e 02 18 00              	callq	0x140681d30 <.text+0x680d30>
140501af2: f3 0f 10 0d 9a c0 4e 02     	movss	0x24ec09a(%rip), %xmm1  # 0x1429edb94
140501afa: 48 89 c1                    	movq	%rax, %rcx
140501afd: e8 4e 48 c0 ff              	callq	0x140106350 <.text+0x105350>
140501b02: 48 8b 45 78                 	movq	0x78(%rbp), %rax
140501b06: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
140501b0a: 0f 10 45 68                 	movups	0x68(%rbp), %xmm0
140501b0e: 0f 29 45 d0                 	movaps	%xmm0, -0x30(%rbp)
140501b12: 48 8d 4d 68                 	leaq	0x68(%rbp), %rcx
140501b16: 48 8d 55 d0                 	leaq	-0x30(%rbp), %rdx
140501b1a: 66 41 b8 20 03              	movw	$0x320, %r8w            # imm = 0x320
140501b1f: e8 3c c9 17 00              	callq	0x14067e460 <.text+0x67d460>
140501b24: 48 8b 55 e8                 	movq	-0x18(%rbp), %rdx
140501b28: 48 85 d2                    	testq	%rdx, %rdx
140501b2b: 74 12                       	je	0x140501b3f <.text+0x500b3f>
140501b2d: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140501b33: 48 8b 8d 88 00 00 00        	movq	0x88(%rbp), %rcx
140501b3a: e8 e1 20 11 00              	callq	0x140613c20 <.text+0x612c20>
140501b3f: 90                          	nop
140501b40: 48 81 c4 18 01 00 00        	addq	$0x118, %rsp            # imm = 0x118
140501b47: 5b                          	popq	%rbx
140501b48: 5f                          	popq	%rdi
140501b49: 5e                          	popq	%rsi
140501b4a: 41 5c                       	popq	%r12
140501b4c: 41 5d                       	popq	%r13
140501b4e: 41 5e                       	popq	%r14
140501b50: 41 5f                       	popq	%r15
140501b52: 5d                          	popq	%rbp
140501b53: c3                          	retq
140501b54: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140501b60: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140501b65: 55                          	pushq	%rbp
140501b66: 41 57                       	pushq	%r15
140501b68: 41 56                       	pushq	%r14
140501b6a: 41 55                       	pushq	%r13
140501b6c: 41 54                       	pushq	%r12
140501b6e: 56                          	pushq	%rsi
140501b6f: 57                          	pushq	%rdi
140501b70: 53                          	pushq	%rbx
140501b71: 48 83 ec 28                 	subq	$0x28, %rsp
140501b75: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140501b7c: 48 8b 55 e8                 	movq	-0x18(%rbp), %rdx
140501b80: 48 85 d2                    	testq	%rdx, %rdx
140501b83: 74 12                       	je	0x140501b97 <.text+0x500b97>
140501b85: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140501b8b: 48 8b 8d 88 00 00 00        	movq	0x88(%rbp), %rcx
140501b92: e8 89 20 11 00              	callq	0x140613c20 <.text+0x612c20>
140501b97: 90                          	nop
140501b98: 48 83 c4 28                 	addq	$0x28, %rsp
140501b9c: 5b                          	popq	%rbx
140501b9d: 5f                          	popq	%rdi
140501b9e: 5e                          	popq	%rsi
140501b9f: 41 5c                       	popq	%r12
140501ba1: 41 5d                       	popq	%r13
140501ba3: 41 5e                       	popq	%r14
140501ba5: 41 5f                       	popq	%r15
140501ba7: 5d                          	popq	%rbp
140501ba8: c3                          	retq
140501ba9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140501bb0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140501bb5: 55                          	pushq	%rbp
140501bb6: 41 57                       	pushq	%r15
140501bb8: 41 56                       	pushq	%r14
140501bba: 41 55                       	pushq	%r13
140501bbc: 41 54                       	pushq	%r12
140501bbe: 56                          	pushq	%rsi
140501bbf: 57                          	pushq	%rdi
140501bc0: 53                          	pushq	%rbx
140501bc1: 48 83 ec 28                 	subq	$0x28, %rsp
140501bc5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140501bcc: 48 8b 55 e8                 	movq	-0x18(%rbp), %rdx

; SOURCE /tmp/key-numeric-button.asm

B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14068c500: 41 57                       	pushq	%r15
14068c502: 41 56                       	pushq	%r14
14068c504: 56                          	pushq	%rsi
14068c505: 57                          	pushq	%rdi
14068c506: 53                          	pushq	%rbx
14068c507: 48 83 ec 60                 	subq	$0x60, %rsp
14068c50b: 4c 89 cf                    	movq	%r9, %rdi
14068c50e: 4c 89 c3                    	movq	%r8, %rbx
14068c511: 49 89 d6                    	movq	%rdx, %r14
14068c514: 48 89 ce                    	movq	%rcx, %rsi
14068c517: 48 8b 84 24 b0 00 00 00     	movq	0xb0(%rsp), %rax
14068c51f: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14068c524: 48 b8 b3 d8 4b 56 3b 2c db 64       	movabsq	$0x64db2c3b564bd8b3, %rax # imm = 0x64DB2C3B564BD8B3
14068c52e: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
14068c533: 48 b8 8f ab 97 45 fe fc 5b 61       	movabsq	$0x615bfcfe4597ab8f, %rax # imm = 0x615BFCFE4597AB8F
14068c53d: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
14068c542: 48 8d 4c 24 20              	leaq	0x20(%rsp), %rcx
14068c547: e8 d4 19 f3 ff              	callq	0x1405bdf20 <.text+0x5bcf20>
14068c54c: 49 8b 4e 08                 	movq	0x8(%r14), %rcx
14068c550: 48 89 4c 24 20              	movq	%rcx, 0x20(%rsp)
14068c555: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
14068c55a: 48 8d 4c 24 20              	leaq	0x20(%rsp), %rcx
14068c55f: e8 ac 13 f3 ff              	callq	0x1405bd910 <.text+0x5bc910>
14068c564: 4c 8d 7c 24 20              	leaq	0x20(%rsp), %r15
14068c569: 4c 89 f9                    	movq	%r15, %rcx
14068c56c: 4c 89 f2                    	movq	%r14, %rdx
14068c56f: 49 89 c0                    	movq	%rax, %r8
14068c572: e8 39 9c ff ff              	callq	0x1406861b0 <.text+0x6851b0>
14068c577: 4c 89 f9                    	movq	%r15, %rcx
14068c57a: 31 d2                       	xorl	%edx, %edx
14068c57c: e8 1f 16 91 00              	callq	0x140f9dba0 <.text+0xf9cba0>
14068c581: 48 89 c1                    	movq	%rax, %rcx
14068c584: 48 89 da                    	movq	%rbx, %rdx
14068c587: 49 89 f8                    	movq	%rdi, %r8
14068c58a: e8 31 ff 91 00              	callq	0x140fac4c0 <.text+0xfab4c0>
14068c58f: 48 8b 44 24 30              	movq	0x30(%rsp), %rax
14068c594: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
14068c599: f3 0f 6f 44 24 20           	movdqu	0x20(%rsp), %xmm0
14068c59f: 66 0f 7f 44 24 40           	movdqa	%xmm0, 0x40(%rsp)
14068c5a5: 48 8b 7c 24 40              	movq	0x40(%rsp), %rdi
14068c5aa: 48 83 bf b8 01 00 00 00     	cmpq	$0x0, 0x1b8(%rdi)
14068c5b2: 0f 84 bd 01 00 00           	je	0x14068c775 <.text+0x68b775>
14068c5b8: 48 8d 54 24 48              	leaq	0x48(%rsp), %rdx
14068c5bd: 48 8d 8f c0 01 00 00        	leaq	0x1c0(%rdi), %rcx
14068c5c4: e8 a7 47 f2 ff              	callq	0x1405b0d70 <.text+0x5afd70>
14068c5c9: 48 8b 8f a0 01 00 00        	movq	0x1a0(%rdi), %rcx
14068c5d0: 48 8b 97 a8 01 00 00        	movq	0x1a8(%rdi), %rdx
14068c5d7: 49 89 d0                    	movq	%rdx, %r8
14068c5da: 49 21 c0                    	andq	%rax, %r8
14068c5dd: 48 c1 e8 39                 	shrq	$0x39, %rax
14068c5e1: 66 0f 6e c0                 	movd	%eax, %xmm0
14068c5e5: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
14068c5e9: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14068c5ee: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14068c5f3: 48 8d 81 40 ff ff ff        	leaq	-0xc0(%rcx), %rax
14068c5fa: 45 31 c9                    	xorl	%r9d, %r9d
14068c5fd: 4c 8b 54 24 48              	movq	0x48(%rsp), %r10
14068c602: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
14068c606: f3 42 0f 6f 14 01           	movdqu	(%rcx,%r8), %xmm2
14068c60c: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
14068c610: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
14068c614: 66 44 0f d7 db              	pmovmskb	%xmm3, %r11d
14068c619: 45 85 db                    	testl	%r11d, %r11d
14068c61c: 74 32                       	je	0x14068c650 <.text+0x68b650>
14068c61e: f3 41 0f bc fb              	tzcntl	%r11d, %edi
14068c623: 4c 01 c7                    	addq	%r8, %rdi
14068c626: 48 21 d7                    	andq	%rdx, %rdi
14068c629: 48 f7 df                    	negq	%rdi
14068c62c: 48 8d 3c 7f                 	leaq	(%rdi,%rdi,2), %rdi
14068c630: 48 c1 e7 06                 	shlq	$0x6, %rdi
14068c634: 4c 3b 14 38                 	cmpq	(%rax,%rdi), %r10
14068c638: 74 38                       	je	0x14068c672 <.text+0x68b672>
14068c63a: 41 8d 7b ff                 	leal	-0x1(%r11), %edi
14068c63e: 66 44 21 df                 	andw	%r11w, %di
14068c642: 41 89 fb                    	movl	%edi, %r11d
14068c645: 75 d7                       	jne	0x14068c61e <.text+0x68b61e>
14068c647: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
14068c650: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
14068c654: 66 44 0f d7 da              	pmovmskb	%xmm2, %r11d
14068c659: 45 85 db                    	testl	%r11d, %r11d
14068c65c: 0f 85 13 01 00 00           	jne	0x14068c775 <.text+0x68b775>
14068c662: 4d 01 c8                    	addq	%r9, %r8
14068c665: 49 83 c0 10                 	addq	$0x10, %r8
14068c669: 49 83 c1 10                 	addq	$0x10, %r9
14068c66d: 49 21 d0                    	andq	%rdx, %r8
14068c670: eb 94                       	jmp	0x14068c606 <.text+0x68b606>
14068c672: 80 4c 39 ff 20              	orb	$0x20, -0x1(%rcx,%rdi)
14068c677: 48 8b 44 24 50              	movq	0x50(%rsp), %rax
14068c67c: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
14068c681: 66 0f 6f 44 24 40           	movdqa	0x40(%rsp), %xmm0
14068c687: 66 0f 7f 44 24 20           	movdqa	%xmm0, 0x20(%rsp)
14068c68d: 48 8b 7c 24 20              	movq	0x20(%rsp), %rdi
14068c692: 48 83 bf b8 01 00 00 00     	cmpq	$0x0, 0x1b8(%rdi)
14068c69a: 0f 84 e1 00 00 00           	je	0x14068c781 <.text+0x68b781>
14068c6a0: 48 8d 54 24 28              	leaq	0x28(%rsp), %rdx
14068c6a5: 48 8d 8f c0 01 00 00        	leaq	0x1c0(%rdi), %rcx
14068c6ac: e8 bf 46 f2 ff              	callq	0x1405b0d70 <.text+0x5afd70>
14068c6b1: 48 8b 8f a0 01 00 00        	movq	0x1a0(%rdi), %rcx
14068c6b8: 48 8b 97 a8 01 00 00        	movq	0x1a8(%rdi), %rdx
14068c6bf: 49 89 d0                    	movq	%rdx, %r8
14068c6c2: 49 21 c0                    	andq	%rax, %r8
14068c6c5: 48 c1 e8 39                 	shrq	$0x39, %rax
14068c6c9: 66 0f 6e c0                 	movd	%eax, %xmm0
14068c6cd: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
14068c6d1: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14068c6d6: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14068c6db: 48 8d 81 40 ff ff ff        	leaq	-0xc0(%rcx), %rax
14068c6e2: 45 31 c9                    	xorl	%r9d, %r9d
14068c6e5: 4c 8b 54 24 28              	movq	0x28(%rsp), %r10
14068c6ea: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
14068c6ee: f3 42 0f 6f 14 01           	movdqu	(%rcx,%r8), %xmm2
14068c6f4: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
14068c6f8: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
14068c6fc: 66 44 0f d7 db              	pmovmskb	%xmm3, %r11d
14068c701: 45 85 db                    	testl	%r11d, %r11d
14068c704: 74 2a                       	je	0x14068c730 <.text+0x68b730>
14068c706: f3 41 0f bc fb              	tzcntl	%r11d, %edi
14068c70b: 4c 01 c7                    	addq	%r8, %rdi
14068c70e: 48 21 d7                    	andq	%rdx, %rdi
14068c711: 48 f7 df                    	negq	%rdi
14068c714: 48 8d 3c 7f                 	leaq	(%rdi,%rdi,2), %rdi
14068c718: 48 c1 e7 06                 	shlq	$0x6, %rdi
14068c71c: 4c 3b 14 38                 	cmpq	(%rax,%rdi), %r10
14068c720: 74 2c                       	je	0x14068c74e <.text+0x68b74e>
14068c722: 41 8d 7b ff                 	leal	-0x1(%r11), %edi
14068c726: 66 44 21 df                 	andw	%r11w, %di
14068c72a: 41 89 fb                    	movl	%edi, %r11d
14068c72d: 75 d7                       	jne	0x14068c706 <.text+0x68b706>
14068c72f: 90                          	nop
14068c730: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
14068c734: 66 44 0f d7 da              	pmovmskb	%xmm2, %r11d
14068c739: 45 85 db                    	testl	%r11d, %r11d
14068c73c: 75 43                       	jne	0x14068c781 <.text+0x68b781>
14068c73e: 4d 01 c8                    	addq	%r9, %r8
14068c741: 49 83 c0 10                 	addq	$0x10, %r8
14068c745: 49 83 c1 10                 	addq	$0x10, %r9
14068c749: 49 21 d0                    	andq	%rdx, %r8
14068c74c: eb a0                       	jmp	0x14068c6ee <.text+0x68b6ee>
14068c74e: 66 c7 44 39 fc 00 02        	movw	$0x200, -0x4(%rcx,%rdi) # imm = 0x200
14068c755: 48 8b 44 24 30              	movq	0x30(%rsp), %rax
14068c75a: 48 89 46 10                 	movq	%rax, 0x10(%rsi)
14068c75e: 0f 28 44 24 20              	movaps	0x20(%rsp), %xmm0
14068c763: 0f 11 06                    	movups	%xmm0, (%rsi)
14068c766: 48 89 f0                    	movq	%rsi, %rax
14068c769: 48 83 c4 60                 	addq	$0x60, %rsp
14068c76d: 5b                          	popq	%rbx
14068c76e: 5f                          	popq	%rdi
14068c76f: 5e                          	popq	%rsi
14068c770: 41 5e                       	popq	%r14
14068c772: 41 5f                       	popq	%r15
14068c774: c3                          	retq
14068c775: 48 8d 0d 44 b8 38 02        	leaq	0x238b844(%rip), %rcx   # 0x142a17fc0
14068c77c: e8 8f cb 24 02              	callq	0x1428d9310 <.text+0x28d8310>
14068c781: 48 8d 0d 68 b8 38 02        	leaq	0x238b868(%rip), %rcx   # 0x142a17ff0
14068c788: e8 83 cb 24 02              	callq	0x1428d9310 <.text+0x28d8310>
14068c78d: cc                          	int3
14068c78e: cc                          	int3
14068c78f: cc                          	int3

; SOURCE /tmp/key-translation-native.asm

B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1417cd0a0: 41 57                       	pushq	%r15
1417cd0a2: 41 56                       	pushq	%r14
1417cd0a4: 41 55                       	pushq	%r13
1417cd0a6: 41 54                       	pushq	%r12
1417cd0a8: 56                          	pushq	%rsi
1417cd0a9: 57                          	pushq	%rdi
1417cd0aa: 55                          	pushq	%rbp
1417cd0ab: 53                          	pushq	%rbx
1417cd0ac: 48 83 ec 78                 	subq	$0x78, %rsp
1417cd0b0: 0f 29 7c 24 60              	movaps	%xmm7, 0x60(%rsp)
1417cd0b5: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
1417cd0ba: 4c 89 c7                    	movq	%r8, %rdi
1417cd0bd: 48 89 d6                    	movq	%rdx, %rsi
1417cd0c0: 48 89 cb                    	movq	%rcx, %rbx
1417cd0c3: 48 8b 84 24 18 01 00 00     	movq	0x118(%rsp), %rax
1417cd0cb: 4c 8b 38                    	movq	(%rax), %r15
1417cd0ce: 4d 8b 21                    	movq	(%r9), %r12
1417cd0d1: 4d 85 e4                    	testq	%r12, %r12
1417cd0d4: 74 3c                       	je	0x1417cd112 <.text+0x17cc112>
1417cd0d6: e8 95 64 15 01              	callq	0x142923570 <.text+0x2922570>
1417cd0db: 4d 8b 34 24                 	movq	(%r12), %r14
1417cd0df: f0                          	lock
1417cd0e0: 49 ff 06                    	incq	(%r14)
1417cd0e3: 0f 8e 56 12 00 00           	jle	0x1417ce33f <.text+0x17cd33f>
1417cd0e9: 4c 89 74 24 20              	movq	%r14, 0x20(%rsp)
1417cd0ee: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
1417cd0f3: 48 89 c1                    	movq	%rax, %rcx
1417cd0f6: e8 f5 63 15 01              	callq	0x1429234f0 <.text+0x29224f0>
1417cd0fb: 89 c5                       	movl	%eax, %ebp
1417cd0fd: f0                          	lock
1417cd0fe: 49 ff 0e                    	decq	(%r14)
1417cd101: 75 0a                       	jne	0x1417cd10d <.text+0x17cc10d>
1417cd103: 48 8d 4c 24 20              	leaq	0x20(%rsp), %rcx
1417cd108: e8 43 82 3b ff              	callq	0x140b85350 <.text+0xb84350>
1417cd10d: 40 84 ed                    	testb	%bpl, %bpl
1417cd110: 75 0a                       	jne	0x1417cd11c <.text+0x17cc11c>
1417cd112: 48 8b 07                    	movq	(%rdi), %rax
1417cd115: 48 83 78 38 00              	cmpq	$0x0, 0x38(%rax)
1417cd11a: 75 0a                       	jne	0x1417cd126 <.text+0x17cc126>
1417cd11c: 48 8b 06                    	movq	(%rsi), %rax
1417cd11f: 48 83 78 38 00              	cmpq	$0x0, 0x38(%rax)
1417cd124: 74 21                       	je	0x1417cd147 <.text+0x17cc147>
1417cd126: 48 8b 84 24 f0 00 00 00     	movq	0xf0(%rsp), %rax
1417cd12e: 8b 48 1c                    	movl	0x1c(%rax), %ecx
1417cd131: 48 8b 50 10                 	movq	0x10(%rax), %rdx
1417cd135: 89 0a                       	movl	%ecx, (%rdx)
1417cd137: 4c 8b 30                    	movq	(%rax), %r14
1417cd13a: e8 a1 64 59 ff              	callq	0x140d635e0 <.text+0xd625e0>
1417cd13f: 49 89 46 10                 	movq	%rax, 0x10(%r14)
1417cd143: 41 89 56 18                 	movl	%edx, 0x18(%r14)
1417cd147: 48 8b 8c 24 e8 00 00 00     	movq	0xe8(%rsp), %rcx
1417cd14f: 48 8b 84 24 e0 00 00 00     	movq	0xe0(%rsp), %rax
1417cd157: 8b 51 1c                    	movl	0x1c(%rcx), %edx
1417cd15a: 4c 8b 41 10                 	movq	0x10(%rcx), %r8
1417cd15e: 41 89 10                    	movl	%edx, (%r8)
1417cd161: 4c 8b 31                    	movq	(%rcx), %r14
1417cd164: 48 8b 3f                    	movq	(%rdi), %rdi
1417cd167: 48 83 7f 18 00              	cmpq	$0x0, 0x18(%rdi)
1417cd16c: 0f 84 8a 00 00 00           	je	0x1417cd1fc <.text+0x17cc1fc>
1417cd172: 48 8b 0f                    	movq	(%rdi), %rcx
1417cd175: 48 8b 57 08                 	movq	0x8(%rdi), %rdx
1417cd179: 49 b8 40 1a 26 64 ed 49 88 25       	movabsq	$0x258849ed64261a40, %r8 # imm = 0x258849ED64261A40
1417cd183: 49 21 d0                    	andq	%rdx, %r8
1417cd186: 45 31 c9                    	xorl	%r9d, %r9d
1417cd189: 66 0f 6f 05 6f 33 18 01     	movdqa	0x118336f(%rip), %xmm0  # 0x142950500
1417cd191: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
1417cd195: f3 42 0f 6f 14 01           	movdqu	(%rcx,%r8), %xmm2
1417cd19b: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
1417cd19f: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
1417cd1a3: 66 44 0f d7 d3              	pmovmskb	%xmm3, %r10d
1417cd1a8: 45 85 d2                    	testl	%r10d, %r10d
1417cd1ab: 74 2d                       	je	0x1417cd1da <.text+0x17cc1da>
1417cd1ad: 0f 1f 00                    	nopl	(%rax)
1417cd1b0: f3 45 0f bc da              	tzcntl	%r10d, %r11d
1417cd1b5: 4d 01 c3                    	addq	%r8, %r11
1417cd1b8: 49 21 d3                    	andq	%rdx, %r11
1417cd1bb: 49 c1 e3 02                 	shlq	$0x2, %r11
1417cd1bf: 49 89 cd                    	movq	%rcx, %r13
1417cd1c2: 4d 29 dd                    	subq	%r11, %r13
1417cd1c5: 66 41 83 7d fc 00           	cmpw	$0x0, -0x4(%r13)
1417cd1cb: 74 2b                       	je	0x1417cd1f8 <.text+0x17cc1f8>
1417cd1cd: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
1417cd1d1: 66 45 21 d3                 	andw	%r10w, %r11w
1417cd1d5: 45 89 da                    	movl	%r11d, %r10d
1417cd1d8: 75 d6                       	jne	0x1417cd1b0 <.text+0x17cc1b0>
1417cd1da: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
1417cd1de: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
1417cd1e3: 45 85 d2                    	testl	%r10d, %r10d
1417cd1e6: 75 14                       	jne	0x1417cd1fc <.text+0x17cc1fc>
1417cd1e8: 4d 01 c8                    	addq	%r9, %r8
1417cd1eb: 49 83 c0 10                 	addq	$0x10, %r8
1417cd1ef: 49 83 c1 10                 	addq	$0x10, %r9
1417cd1f3: 49 21 d0                    	andq	%rdx, %r8
1417cd1f6: eb 9d                       	jmp	0x1417cd195 <.text+0x17cc195>
1417cd1f8: b1 01                       	movb	$0x1, %cl
1417cd1fa: eb 02                       	jmp	0x1417cd1fe <.text+0x17cc1fe>
1417cd1fc: 31 c9                       	xorl	%ecx, %ecx
1417cd1fe: 41 08 8e 88 00 00 00        	orb	%cl, 0x88(%r14)
1417cd205: 48 8b 00                    	movq	(%rax), %rax
1417cd208: 48 8b 08                    	movq	(%rax), %rcx
1417cd20b: 48 83 f9 02                 	cmpq	$0x2, %rcx
1417cd20f: 0f 84 cb 02 00 00           	je	0x1417cd4e0 <.text+0x17cc4e0>
1417cd215: 83 f9 0a                    	cmpl	$0xa, %ecx
1417cd218: 75 0a                       	jne	0x1417cd224 <.text+0x17cc224>
1417cd21a: 80 78 08 01                 	cmpb	$0x1, 0x8(%rax)
1417cd21e: 0f 85 bc 02 00 00           	jne	0x1417cd4e0 <.text+0x17cc4e0>
1417cd224: 48 8b 36                    	movq	(%rsi), %rsi
1417cd227: c7 44 24 20 73 00 00 00     	movl	$0x73, 0x20(%rsp)
1417cd22f: 48 8d 4e 20                 	leaq	0x20(%rsi), %rcx
1417cd233: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417cd238: e8 b3 dc 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417cd23d: 48 85 c0                    	testq	%rax, %rax
1417cd240: 0f 95 c0                    	setne	%al
1417cd243: 41 08 86 9a 00 00 00        	orb	%al, 0x9a(%r14)
1417cd24a: 41 8b 87 50 01 00 00        	movl	0x150(%r15), %eax
1417cd251: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cd256: 0f 84 da 0c 00 00           	je	0x1417cdf36 <.text+0x17ccf36>
1417cd25c: 49 8b 8f 54 01 00 00        	movq	0x154(%r15), %rcx
1417cd263: 89 44 24 20                 	movl	%eax, 0x20(%rsp)
1417cd267: 48 89 4c 24 24              	movq	%rcx, 0x24(%rsp)
1417cd26c: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417cd271: 48 89 f1                    	movq	%rsi, %rcx
1417cd274: e8 77 dc 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417cd279: 48 85 c0                    	testq	%rax, %rax
1417cd27c: 0f 95 c0                    	setne	%al
1417cd27f: 41 08 86 9a 00 00 00        	orb	%al, 0x9a(%r14)
1417cd286: 41 8b 87 5c 01 00 00        	movl	0x15c(%r15), %eax
1417cd28d: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cd292: 0f 84 b9 0c 00 00           	je	0x1417cdf51 <.text+0x17ccf51>
1417cd298: 49 8b 8f 60 01 00 00        	movq	0x160(%r15), %rcx
1417cd29f: 89 44 24 20                 	movl	%eax, 0x20(%rsp)
1417cd2a3: 48 89 4c 24 24              	movq	%rcx, 0x24(%rsp)
1417cd2a8: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417cd2ad: 48 89 f1                    	movq	%rsi, %rcx
1417cd2b0: e8 3b dc 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417cd2b5: 48 85 c0                    	testq	%rax, %rax
1417cd2b8: 0f 95 c0                    	setne	%al
1417cd2bb: 41 08 86 9b 00 00 00        	orb	%al, 0x9b(%r14)
1417cd2c2: 41 8b 87 68 01 00 00        	movl	0x168(%r15), %eax
1417cd2c9: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cd2ce: 0f 84 98 0c 00 00           	je	0x1417cdf6c <.text+0x17ccf6c>
1417cd2d4: 49 8b 8f 6c 01 00 00        	movq	0x16c(%r15), %rcx
1417cd2db: 89 44 24 20                 	movl	%eax, 0x20(%rsp)
1417cd2df: 48 89 4c 24 24              	movq	%rcx, 0x24(%rsp)
1417cd2e4: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417cd2e9: 48 89 f1                    	movq	%rsi, %rcx
1417cd2ec: e8 ff db 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417cd2f1: 48 85 c0                    	testq	%rax, %rax
1417cd2f4: 0f 95 c0                    	setne	%al
1417cd2f7: 41 08 86 9c 00 00 00        	orb	%al, 0x9c(%r14)
1417cd2fe: 41 8b 87 74 01 00 00        	movl	0x174(%r15), %eax
1417cd305: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cd30a: 0f 84 77 0c 00 00           	je	0x1417cdf87 <.text+0x17ccf87>
1417cd310: 49 8b 8f 78 01 00 00        	movq	0x178(%r15), %rcx
1417cd317: 89 44 24 20                 	movl	%eax, 0x20(%rsp)
1417cd31b: 48 89 4c 24 24              	movq	%rcx, 0x24(%rsp)
1417cd320: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417cd325: 48 89 f1                    	movq	%rsi, %rcx
1417cd328: e8 c3 db 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417cd32d: 48 85 c0                    	testq	%rax, %rax
1417cd330: 0f 95 c0                    	setne	%al
1417cd333: 41 08 86 9d 00 00 00        	orb	%al, 0x9d(%r14)
1417cd33a: 41 8b 87 80 01 00 00        	movl	0x180(%r15), %eax
1417cd341: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cd346: 0f 84 56 0c 00 00           	je	0x1417cdfa2 <.text+0x17ccfa2>
1417cd34c: 49 8b 8f 84 01 00 00        	movq	0x184(%r15), %rcx
1417cd353: 89 44 24 20                 	movl	%eax, 0x20(%rsp)
1417cd357: 48 89 4c 24 24              	movq	%rcx, 0x24(%rsp)
1417cd35c: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417cd361: 48 89 f1                    	movq	%rsi, %rcx
1417cd364: e8 87 db 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417cd369: 48 85 c0                    	testq	%rax, %rax
1417cd36c: 0f 95 c0                    	setne	%al
1417cd36f: 41 08 86 9e 00 00 00        	orb	%al, 0x9e(%r14)
1417cd376: 41 8b 87 8c 01 00 00        	movl	0x18c(%r15), %eax
1417cd37d: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cd382: 0f 84 35 0c 00 00           	je	0x1417cdfbd <.text+0x17ccfbd>
1417cd388: 49 8b 8f 90 01 00 00        	movq	0x190(%r15), %rcx
1417cd38f: 89 44 24 20                 	movl	%eax, 0x20(%rsp)
1417cd393: 48 89 4c 24 24              	movq	%rcx, 0x24(%rsp)
1417cd398: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417cd39d: 48 89 f1                    	movq	%rsi, %rcx
1417cd3a0: e8 4b db 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417cd3a5: 48 85 c0                    	testq	%rax, %rax
1417cd3a8: 0f 95 c0                    	setne	%al
1417cd3ab: 41 08 86 9f 00 00 00        	orb	%al, 0x9f(%r14)
1417cd3b2: 41 8b 87 98 01 00 00        	movl	0x198(%r15), %eax
1417cd3b9: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cd3be: 0f 84 14 0c 00 00           	je	0x1417cdfd8 <.text+0x17ccfd8>
1417cd3c4: 49 8b 8f 9c 01 00 00        	movq	0x19c(%r15), %rcx
1417cd3cb: 89 44 24 20                 	movl	%eax, 0x20(%rsp)
1417cd3cf: 48 89 4c 24 24              	movq	%rcx, 0x24(%rsp)
1417cd3d4: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417cd3d9: 48 89 f1                    	movq	%rsi, %rcx
1417cd3dc: e8 0f db 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417cd3e1: 48 85 c0                    	testq	%rax, %rax
1417cd3e4: 0f 95 c0                    	setne	%al
1417cd3e7: 41 08 86 a0 00 00 00        	orb	%al, 0xa0(%r14)
1417cd3ee: 41 8b 87 a4 01 00 00        	movl	0x1a4(%r15), %eax
1417cd3f5: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cd3fa: 0f 84 f3 0b 00 00           	je	0x1417cdff3 <.text+0x17ccff3>
1417cd400: 49 8b 8f a8 01 00 00        	movq	0x1a8(%r15), %rcx
1417cd407: 89 44 24 20                 	movl	%eax, 0x20(%rsp)
1417cd40b: 48 89 4c 24 24              	movq	%rcx, 0x24(%rsp)
1417cd410: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417cd415: 48 89 f1                    	movq	%rsi, %rcx
1417cd418: e8 d3 da 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417cd41d: 48 85 c0                    	testq	%rax, %rax
1417cd420: 0f 95 c0                    	setne	%al
1417cd423: 41 08 86 a1 00 00 00        	orb	%al, 0xa1(%r14)
1417cd42a: 4d 85 e4                    	testq	%r12, %r12
1417cd42d: 0f 84 d2 0b 00 00           	je	0x1417ce005 <.text+0x17cd005>
1417cd433: 41 8b 87 38 01 00 00        	movl	0x138(%r15), %eax
1417cd43a: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cd43f: 0f 85 40 0c 00 00           	jne	0x1417ce085 <.text+0x17cd085>
1417cd445: 31 c0                       	xorl	%eax, %eax
1417cd447: 41 08 86 a7 00 00 00        	orb	%al, 0xa7(%r14)
1417cd44e: 41 8b 87 44 01 00 00        	movl	0x144(%r15), %eax
1417cd455: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cd45a: 0f 84 61 0c 00 00           	je	0x1417ce0c1 <.text+0x17cd0c1>
1417cd460: 49 8b 8f 48 01 00 00        	movq	0x148(%r15), %rcx
1417cd467: 89 44 24 20                 	movl	%eax, 0x20(%rsp)
1417cd46b: 48 89 4c 24 24              	movq	%rcx, 0x24(%rsp)
1417cd470: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417cd475: 48 89 f1                    	movq	%rsi, %rcx
1417cd478: e8 73 da 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417cd47d: 48 85 c0                    	testq	%rax, %rax
1417cd480: 0f 95 c0                    	setne	%al
1417cd483: 41 08 86 a8 00 00 00        	orb	%al, 0xa8(%r14)
1417cd48a: 41 8b 87 b0 01 00 00        	movl	0x1b0(%r15), %eax
1417cd491: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cd496: 0f 84 40 0c 00 00           	je	0x1417ce0dc <.text+0x17cd0dc>
1417cd49c: 49 8b 8f b4 01 00 00        	movq	0x1b4(%r15), %rcx
1417cd4a3: 89 44 24 20                 	movl	%eax, 0x20(%rsp)
1417cd4a7: 48 89 4c 24 24              	movq	%rcx, 0x24(%rsp)
1417cd4ac: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417cd4b1: 48 89 f1                    	movq	%rsi, %rcx
1417cd4b4: e8 37 da 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417cd4b9: 48 85 c0                    	testq	%rax, %rax
1417cd4bc: 0f 95 c0                    	setne	%al
1417cd4bf: 41 08 86 a9 00 00 00        	orb	%al, 0xa9(%r14)
1417cd4c6: 48 8b bc 24 10 01 00 00     	movq	0x110(%rsp), %rdi
1417cd4ce: 80 3d 03 78 e6 01 00        	cmpb	$0x0, 0x1e67803(%rip)   # 0x143634cd8
1417cd4d5: 0f 84 68 0d 00 00           	je	0x1417ce243 <.text+0x17cd243>
1417cd4db: e9 1a 0c 00 00              	jmp	0x1417ce0fa <.text+0x17cd0fa>
1417cd4e0: 48 8b 1b                    	movq	(%rbx), %rbx
1417cd4e3: f3 0f 10 73 28              	movss	0x28(%rbx), %xmm6
1417cd4e8: f3 0f 59 35 44 fb 1f 01     	mulss	0x11ffb44(%rip), %xmm6  # 0x1429cd034
1417cd4f0: 41 8b 07                    	movl	(%r15), %eax
1417cd4f3: 48 8b 36                    	movq	(%rsi), %rsi
1417cd4f6: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cd4fb: 74 2f                       	je	0x1417cd52c <.text+0x17cc52c>
1417cd4fd: 49 8b 4f 04                 	movq	0x4(%r15), %rcx
1417cd501: 89 44 24 20                 	movl	%eax, 0x20(%rsp)
1417cd505: 48 89 4c 24 24              	movq	%rcx, 0x24(%rsp)
1417cd50a: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417cd50f: 48 89 f1                    	movq	%rsi, %rcx
1417cd512: e8 d9 d9 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417cd517: 48 85 c0                    	testq	%rax, %rax
1417cd51a: 74 10                       	je	0x1417cd52c <.text+0x17cc52c>
1417cd51c: f3 41 0f 10 46 6c           	movss	0x6c(%r14), %xmm0
1417cd522: f3 0f 5c c6                 	subss	%xmm6, %xmm0
1417cd526: f3 41 0f 11 46 6c           	movss	%xmm0, 0x6c(%r14)
1417cd52c: 41 8b 47 0c                 	movl	0xc(%r15), %eax
1417cd530: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cd535: 74 2f                       	je	0x1417cd566 <.text+0x17cc566>
1417cd537: 49 8b 4f 10                 	movq	0x10(%r15), %rcx
1417cd53b: 89 44 24 20                 	movl	%eax, 0x20(%rsp)
1417cd53f: 48 89 4c 24 24              	movq	%rcx, 0x24(%rsp)
1417cd544: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417cd549: 48 89 f1                    	movq	%rsi, %rcx
1417cd54c: e8 9f d9 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417cd551: 48 85 c0                    	testq	%rax, %rax
1417cd554: 74 10                       	je	0x1417cd566 <.text+0x17cc566>
1417cd556: f3 41 0f 10 46 6c           	movss	0x6c(%r14), %xmm0
1417cd55c: f3 0f 58 c6                 	addss	%xmm6, %xmm0
1417cd560: f3 41 0f 11 46 6c           	movss	%xmm0, 0x6c(%r14)
1417cd566: 41 8b 47 18                 	movl	0x18(%r15), %eax
1417cd56a: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cd56f: 74 2f                       	je	0x1417cd5a0 <.text+0x17cc5a0>
1417cd571: 49 8b 4f 1c                 	movq	0x1c(%r15), %rcx
1417cd575: 89 44 24 20                 	movl	%eax, 0x20(%rsp)
1417cd579: 48 89 4c 24 24              	movq	%rcx, 0x24(%rsp)
1417cd57e: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417cd583: 48 89 f1                    	movq	%rsi, %rcx
1417cd586: e8 65 d9 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417cd58b: 48 85 c0                    	testq	%rax, %rax
1417cd58e: 74 10                       	je	0x1417cd5a0 <.text+0x17cc5a0>
1417cd590: f3 41 0f 10 46 68           	movss	0x68(%r14), %xmm0
1417cd596: f3 0f 5c c6                 	subss	%xmm6, %xmm0
1417cd59a: f3 41 0f 11 46 68           	movss	%xmm0, 0x68(%r14)
1417cd5a0: 41 8b 47 24                 	movl	0x24(%r15), %eax
1417cd5a4: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cd5a9: 74 2f                       	je	0x1417cd5da <.text+0x17cc5da>
1417cd5ab: 49 8b 4f 28                 	movq	0x28(%r15), %rcx
1417cd5af: 89 44 24 20                 	movl	%eax, 0x20(%rsp)
1417cd5b3: 48 89 4c 24 24              	movq	%rcx, 0x24(%rsp)
1417cd5b8: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417cd5bd: 48 89 f1                    	movq	%rsi, %rcx
1417cd5c0: e8 2b d9 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417cd5c5: 48 85 c0                    	testq	%rax, %rax
1417cd5c8: 74 10                       	je	0x1417cd5da <.text+0x17cc5da>
1417cd5ca: f3 41 0f 10 46 68           	movss	0x68(%r14), %xmm0
1417cd5d0: f3 0f 58 c6                 	addss	%xmm6, %xmm0
1417cd5d4: f3 41 0f 11 46 68           	movss	%xmm0, 0x68(%r14)
1417cd5da: 41 8b 47 3c                 	movl	0x3c(%r15), %eax
1417cd5de: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cd5e3: 74 2f                       	je	0x1417cd614 <.text+0x17cc614>
1417cd5e5: 49 8b 4f 40                 	movq	0x40(%r15), %rcx
1417cd5e9: 89 44 24 20                 	movl	%eax, 0x20(%rsp)
1417cd5ed: 48 89 4c 24 24              	movq	%rcx, 0x24(%rsp)
1417cd5f2: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417cd5f7: 48 89 f1                    	movq	%rsi, %rcx
1417cd5fa: e8 f1 d8 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417cd5ff: 48 85 c0                    	testq	%rax, %rax
1417cd602: 74 10                       	je	0x1417cd614 <.text+0x17cc614>
1417cd604: f3 41 0f 10 46 70           	movss	0x70(%r14), %xmm0
1417cd60a: f3 0f 5c c6                 	subss	%xmm6, %xmm0
1417cd60e: f3 41 0f 11 46 70           	movss	%xmm0, 0x70(%r14)
1417cd614: 41 8b 47 30                 	movl	0x30(%r15), %eax
1417cd618: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cd61d: 74 2f                       	je	0x1417cd64e <.text+0x17cc64e>
1417cd61f: 49 8b 4f 34                 	movq	0x34(%r15), %rcx
1417cd623: 89 44 24 20                 	movl	%eax, 0x20(%rsp)
1417cd627: 48 89 4c 24 24              	movq	%rcx, 0x24(%rsp)
1417cd62c: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417cd631: 48 89 f1                    	movq	%rsi, %rcx
1417cd634: e8 b7 d8 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417cd639: 48 85 c0                    	testq	%rax, %rax
1417cd63c: 74 10                       	je	0x1417cd64e <.text+0x17cc64e>
1417cd63e: f3 41 0f 10 46 70           	movss	0x70(%r14), %xmm0
1417cd644: f3 0f 58 c6                 	addss	%xmm6, %xmm0
1417cd648: f3 41 0f 11 46 70           	movss	%xmm0, 0x70(%r14)
1417cd64e: 41 8b 47 60                 	movl	0x60(%r15), %eax
1417cd652: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cd657: 74 41                       	je	0x1417cd69a <.text+0x17cc69a>
1417cd659: 49 8b 4f 64                 	movq	0x64(%r15), %rcx
1417cd65d: 89 44 24 20                 	movl	%eax, 0x20(%rsp)
1417cd661: 48 89 4c 24 24              	movq	%rcx, 0x24(%rsp)
1417cd666: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417cd66b: 48 89 f1                    	movq	%rsi, %rcx
1417cd66e: e8 7d d8 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417cd673: 48 85 c0                    	testq	%rax, %rax
1417cd676: 74 22                       	je	0x1417cd69a <.text+0x17cc69a>
1417cd678: f3 0f 10 05 50 82 15 01     	movss	0x1158250(%rip), %xmm0  # 0x1429258d0
1417cd680: f3 0f 59 c6                 	mulss	%xmm6, %xmm0
1417cd684: f2 41 0f 10 4e 78           	movsd	0x78(%r14), %xmm1
1417cd68a: 66 0f ef d2                 	pxor	%xmm2, %xmm2
1417cd68e: f3 0f 10 d0                 	movss	%xmm0, %xmm2            # xmm2 = xmm0[0],xmm2[1,2,3]
1417cd692: 0f 58 d1                    	addps	%xmm1, %xmm2
1417cd695: 41 0f 13 56 78              	movlps	%xmm2, 0x78(%r14)
1417cd69a: 41 8b 47 6c                 	movl	0x6c(%r15), %eax
1417cd69e: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cd6a3: 74 33                       	je	0x1417cd6d8 <.text+0x17cc6d8>
1417cd6a5: 49 8b 4f 70                 	movq	0x70(%r15), %rcx
1417cd6a9: 89 44 24 20                 	movl	%eax, 0x20(%rsp)
1417cd6ad: 48 89 4c 24 24              	movq	%rcx, 0x24(%rsp)
1417cd6b2: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417cd6b7: 48 89 f1                    	movq	%rsi, %rcx
1417cd6ba: e8 31 d8 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417cd6bf: 48 85 c0                    	testq	%rax, %rax
1417cd6c2: 74 14                       	je	0x1417cd6d8 <.text+0x17cc6d8>
1417cd6c4: f3 0f 59 35 b4 1a 21 01     	mulss	0x1211ab4(%rip), %xmm6  # 0x1429df180
1417cd6cc: f3 41 0f 58 76 78           	addss	0x78(%r14), %xmm6
1417cd6d2: f3 41 0f 11 76 78           	movss	%xmm6, 0x78(%r14)
1417cd6d8: 41 8b 47 54                 	movl	0x54(%r15), %eax
1417cd6dc: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cd6e1: 74 48                       	je	0x1417cd72b <.text+0x17cc72b>
1417cd6e3: 49 8b 4f 58                 	movq	0x58(%r15), %rcx
1417cd6e7: 89 44 24 20                 	movl	%eax, 0x20(%rsp)
1417cd6eb: 48 89 4c 24 24              	movq	%rcx, 0x24(%rsp)
1417cd6f0: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417cd6f5: 48 89 f1                    	movq	%rsi, %rcx
1417cd6f8: e8 f3 d7 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417cd6fd: 48 85 c0                    	testq	%rax, %rax
1417cd700: 74 29                       	je	0x1417cd72b <.text+0x17cc72b>
1417cd702: f3 0f 10 43 28              	movss	0x28(%rbx), %xmm0
1417cd707: f3 0f 59 05 21 56 51 01     	mulss	0x1515621(%rip), %xmm0  # 0x142ce2d30
1417cd70f: f2 41 0f 10 8e 80 00 00 00  	movsd	0x80(%r14), %xmm1
1417cd718: 66 0f ef d2                 	pxor	%xmm2, %xmm2
1417cd71c: f3 0f 10 d0                 	movss	%xmm0, %xmm2            # xmm2 = xmm0[0],xmm2[1,2,3]
1417cd720: 0f 58 d1                    	addps	%xmm1, %xmm2
1417cd723: 41 0f 13 96 80 00 00 00     	movlps	%xmm2, 0x80(%r14)
1417cd72b: 4c 8b ac 24 f8 00 00 00     	movq	0xf8(%rsp), %r13
1417cd733: 41 8b 47 48                 	movl	0x48(%r15), %eax
1417cd737: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cd73c: 74 3e                       	je	0x1417cd77c <.text+0x17cc77c>
1417cd73e: 49 8b 4f 4c                 	movq	0x4c(%r15), %rcx
1417cd742: 89 44 24 20                 	movl	%eax, 0x20(%rsp)
1417cd746: 48 89 4c 24 24              	movq	%rcx, 0x24(%rsp)
1417cd74b: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417cd750: 48 89 f1                    	movq	%rsi, %rcx
1417cd753: e8 98 d7 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417cd758: 48 85 c0                    	testq	%rax, %rax
1417cd75b: 74 1f                       	je	0x1417cd77c <.text+0x17cc77c>
1417cd75d: f3 0f 10 43 28              	movss	0x28(%rbx), %xmm0
1417cd762: f3 0f 59 05 ca 55 51 01     	mulss	0x15155ca(%rip), %xmm0  # 0x142ce2d34
1417cd76a: f3 41 0f 58 86 80 00 00 00  	addss	0x80(%r14), %xmm0
1417cd773: f3 41 0f 11 86 80 00 00 00  	movss	%xmm0, 0x80(%r14)
1417cd77c: 48 8b 84 24 08 01 00 00     	movq	0x108(%rsp), %rax
1417cd784: 49 8b 4d 00                 	movq	(%r13), %rcx
1417cd788: 4d 8b 5d 08                 	movq	0x8(%r13), %r11
1417cd78c: 4c 8b 29                    	movq	(%rcx), %r13
1417cd78f: 31 db                       	xorl	%ebx, %ebx
1417cd791: 4d 89 e8                    	movq	%r13, %r8
1417cd794: 4d 2b 43 18                 	subq	0x18(%r11), %r8
1417cd798: 4c 0f 42 c3                 	cmovbq	%rbx, %r8
1417cd79c: 4d 2b 6b 38                 	subq	0x38(%r11), %r13
1417cd7a0: 4c 0f 42 eb                 	cmovbq	%rbx, %r13
1417cd7a4: 4d 89 c1                    	movq	%r8, %r9
1417cd7a7: 49 c1 e1 05                 	shlq	$0x5, %r9
1417cd7ab: 4d 03 4b 08                 	addq	0x8(%r11), %r9
1417cd7af: 49 8b 53 10                 	movq	0x10(%r11), %rdx
1417cd7b3: 4c 29 c2                    	subq	%r8, %rdx
1417cd7b6: 48 0f 42 d3                 	cmovbq	%rbx, %rdx
1417cd7ba: bd 08 00 00 00              	movl	$0x8, %ebp
1417cd7bf: 4c 0f 42 cd                 	cmovbq	%rbp, %r9
1417cd7c3: 4d 89 ea                    	movq	%r13, %r10
1417cd7c6: 49 c1 e2 05                 	shlq	$0x5, %r10
1417cd7ca: 4d 03 53 28                 	addq	0x28(%r11), %r10
1417cd7ce: 4d 8b 43 30                 	movq	0x30(%r11), %r8
1417cd7d2: 4d 29 e8                    	subq	%r13, %r8
1417cd7d5: 4c 0f 42 c3                 	cmovbq	%rbx, %r8
1417cd7d9: 4c 0f 42 d5                 	cmovbq	%rbp, %r10
1417cd7dd: 4d 8b 5b 40                 	movq	0x40(%r11), %r11
1417cd7e1: 4d 29 c3                    	subq	%r8, %r11
1417cd7e4: 49 29 d3                    	subq	%rdx, %r11
1417cd7e7: 4c 89 19                    	movq	%r11, (%rcx)
1417cd7ea: 48 c1 e2 05                 	shlq	$0x5, %rdx
1417cd7ee: 4c 01 ca                    	addq	%r9, %rdx
1417cd7f1: 49 c1 e0 05                 	shlq	$0x5, %r8
1417cd7f5: 4d 01 d0                    	addq	%r10, %r8
1417cd7f8: f3 0f 10 05 04 81 15 01     	movss	0x1158104(%rip), %xmm0  # 0x142925904
1417cd800: f3 0f 10 0d fc 6d 16 01     	movss	0x1166dfc(%rip), %xmm1  # 0x142934604
1417cd808: eb 2b                       	jmp	0x1417cd835 <.text+0x17cc835>
1417cd80a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1417cd810: 0f 28 d1                    	movaps	%xmm1, %xmm2
1417cd813: f3 41 0f 59 51 14           	mulss	0x14(%r9), %xmm2
1417cd819: 66 0f ef db                 	pxor	%xmm3, %xmm3
1417cd81d: f3 0f 10 da                 	movss	%xmm2, %xmm3            # xmm3 = xmm2[0],xmm3[1,2,3]
1417cd821: f2 41 0f 10 56 78           	movsd	0x78(%r14), %xmm2
1417cd827: 0f 58 d3                    	addps	%xmm3, %xmm2
1417cd82a: 41 0f 13 56 78              	movlps	%xmm2, 0x78(%r14)
1417cd82f: 49 89 d9                    	movq	%rbx, %r9
1417cd832: 4d 89 da                    	movq	%r11, %r10
1417cd835: 4d 85 c9                    	testq	%r9, %r9
1417cd838: 74 26                       	je	0x1417cd860 <.text+0x17cc860>
1417cd83a: 49 39 d1                    	cmpq	%rdx, %r9
1417cd83d: 74 21                       	je	0x1417cd860 <.text+0x17cc860>
1417cd83f: 49 8d 59 20                 	leaq	0x20(%r9), %rbx
1417cd843: 4d 89 d3                    	movq	%r10, %r11
1417cd846: 48 ff 01                    	incq	(%rcx)
1417cd849: 41 80 79 18 00              	cmpb	$0x0, 0x18(%r9)
1417cd84e: 0f 28 d0                    	movaps	%xmm0, %xmm2
1417cd851: 75 c0                       	jne	0x1417cd813 <.text+0x17cc813>
1417cd853: eb bb                       	jmp	0x1417cd810 <.text+0x17cc810>
1417cd855: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
1417cd860: 4d 85 d2                    	testq	%r10, %r10
1417cd863: 74 1d                       	je	0x1417cd882 <.text+0x17cc882>
1417cd865: 4d 39 c2                    	cmpq	%r8, %r10
1417cd868: 74 18                       	je	0x1417cd882 <.text+0x17cc882>
1417cd86a: 4d 8d 5a 20                 	leaq	0x20(%r10), %r11
1417cd86e: 31 db                       	xorl	%ebx, %ebx
1417cd870: 4d 89 d1                    	movq	%r10, %r9
1417cd873: 48 ff 01                    	incq	(%rcx)
1417cd876: 41 80 79 18 00              	cmpb	$0x0, 0x18(%r9)
1417cd87b: 0f 28 d0                    	movaps	%xmm0, %xmm2
1417cd87e: 75 93                       	jne	0x1417cd813 <.text+0x17cc813>
1417cd880: eb 8e                       	jmp	0x1417cd810 <.text+0x17cc810>
1417cd882: 48 89 7c 24 38              	movq	%rdi, 0x38(%rsp)
1417cd887: 4c 89 64 24 40              	movq	%r12, 0x40(%rsp)
1417cd88c: 48 89 74 24 48              	movq	%rsi, 0x48(%rsp)
1417cd891: 48 8b 08                    	movq	(%rax), %rcx
1417cd894: 4c 8b 58 08                 	movq	0x8(%rax), %r11
1417cd898: 48 8b 01                    	movq	(%rcx), %rax
1417cd89b: 31 db                       	xorl	%ebx, %ebx
1417cd89d: 48 89 c2                    	movq	%rax, %rdx
1417cd8a0: 49 2b 53 18                 	subq	0x18(%r11), %rdx
1417cd8a4: 48 0f 42 d3                 	cmovbq	%rbx, %rdx
1417cd8a8: 49 2b 43 38                 	subq	0x38(%r11), %rax
1417cd8ac: 48 0f 42 c3                 	cmovbq	%rbx, %rax
1417cd8b0: 4d 8b 53 10                 	movq	0x10(%r11), %r10
1417cd8b4: 4d 8b 43 30                 	movq	0x30(%r11), %r8
1417cd8b8: 4c 89 44 24 30              	movq	%r8, 0x30(%rsp)
1417cd8bd: 49 29 c0                    	subq	%rax, %r8
1417cd8c0: 4c 0f 42 c3                 	cmovbq	%rbx, %r8
1417cd8c4: 41 0f 96 c4                 	setbe	%r12b
1417cd8c8: 4c 89 d5                    	movq	%r10, %rbp
1417cd8cb: 48 29 d5                    	subq	%rdx, %rbp
1417cd8ce: 48 0f 42 eb                 	cmovbq	%rbx, %rbp
1417cd8d2: 4d 8b 6b 08                 	movq	0x8(%r11), %r13
1417cd8d6: 49 8b 5b 28                 	movq	0x28(%r11), %rbx
1417cd8da: 40 0f 96 c7                 	setbe	%dil
1417cd8de: 4d 8b 4b 40                 	movq	0x40(%r11), %r9
1417cd8e2: 4d 89 cb                    	movq	%r9, %r11
1417cd8e5: 4d 29 c3                    	subq	%r8, %r11
1417cd8e8: 49 29 eb                    	subq	%rbp, %r11
1417cd8eb: 4c 89 19                    	movq	%r11, (%rcx)
1417cd8ee: 0f 57 f6                    	xorps	%xmm6, %xmm6
1417cd8f1: 0f 57 ff                    	xorps	%xmm7, %xmm7
1417cd8f4: 44 84 e7                    	testb	%r12b, %dil
1417cd8f7: 75 60                       	jne	0x1417cd959 <.text+0x17cc959>
1417cd8f9: 49 89 c3                    	movq	%rax, %r11
1417cd8fc: 49 c1 e3 04                 	shlq	$0x4, %r11
1417cd900: 4c 01 db                    	addq	%r11, %rbx
1417cd903: 49 c1 e0 04                 	shlq	$0x4, %r8
1417cd907: 49 89 d3                    	movq	%rdx, %r11
1417cd90a: 49 c1 e3 04                 	shlq	$0x4, %r11
1417cd90e: 4d 01 dd                    	addq	%r11, %r13
1417cd911: 48 c1 e5 04                 	shlq	$0x4, %rbp
1417cd915: 45 31 db                    	xorl	%r11d, %r11d
1417cd918: 49 39 d2                    	cmpq	%rdx, %r10
1417cd91b: ba 08 00 00 00              	movl	$0x8, %edx
1417cd920: 4c 0f 42 ea                 	cmovbq	%rdx, %r13
1417cd924: 4d 8d 54 2d f0              	leaq	-0x10(%r13,%rbp), %r10
1417cd929: 4d 0f 47 da                 	cmovaq	%r10, %r11
1417cd92d: 48 39 44 24 30              	cmpq	%rax, 0x30(%rsp)
1417cd932: 48 0f 42 da                 	cmovbq	%rdx, %rbx
1417cd936: 4a 8d 44 03 f0              	leaq	-0x10(%rbx,%r8), %rax
1417cd93b: 49 0f 46 c3                 	cmovbeq	%r11, %rax
1417cd93f: 4c 89 09                    	movq	%r9, (%rcx)
1417cd942: f2 0f 10 40 08              	movsd	0x8(%rax), %xmm0
1417cd947: 0f 59 c0                    	mulps	%xmm0, %xmm0
1417cd94a: f3 0f 16 c8                 	movshdup	%xmm0, %xmm1            # xmm1 = xmm0[1,1,3,3]
1417cd94e: f3 0f 58 c8                 	addss	%xmm0, %xmm1
1417cd952: 0f 57 ff                    	xorps	%xmm7, %xmm7
1417cd955: f3 0f 51 f9                 	sqrtss	%xmm1, %xmm7
1417cd959: 41 0f b7 9f c4 01 00 00     	movzwl	0x1c4(%r15), %ebx
1417cd961: 41 0f b7 af c6 01 00 00     	movzwl	0x1c6(%r15), %ebp
1417cd969: 48 8b 7c 24 38              	movq	0x38(%rsp), %rdi
1417cd96e: 48 89 f9                    	movq	%rdi, %rcx
1417cd971: 89 da                       	movl	%ebx, %edx
1417cd973: 41 89 e8                    	movl	%ebp, %r8d
1417cd976: e8 15 cf 0c ff              	callq	0x14089a890 <.text+0x899890>
1417cd97b: 84 c0                       	testb	%al, %al
1417cd97d: 48 8b 74 24 48              	movq	0x48(%rsp), %rsi
1417cd982: 4c 8b 64 24 40              	movq	0x40(%rsp), %r12
1417cd987: 4c 8b ac 24 00 01 00 00     	movq	0x100(%rsp), %r13
1417cd98f: 75 1b                       	jne	0x1417cd9ac <.text+0x17cc9ac>
1417cd991: 48 89 f9                    	movq	%rdi, %rcx
1417cd994: 89 da                       	movl	%ebx, %edx
1417cd996: 41 89 e8                    	movl	%ebp, %r8d
1417cd999: e8 b2 d1 0c ff              	callq	0x14089ab50 <.text+0x899b50>
1417cd99e: f3 41 0f 10 75 00           	movss	(%r13), %xmm6
1417cd9a4: 84 c0                       	testb	%al, %al
1417cd9a6: 74 0a                       	je	0x1417cd9b2 <.text+0x17cc9b2>
1417cd9a8: f3 0f 58 f7                 	addss	%xmm7, %xmm6
1417cd9ac: f3 41 0f 11 75 00           	movss	%xmm6, (%r13)
1417cd9b2: 48 89 f9                    	movq	%rdi, %rcx
1417cd9b5: 89 da                       	movl	%ebx, %edx
1417cd9b7: 41 89 e8                    	movl	%ebp, %r8d
1417cd9ba: e8 31 d0 0c ff              	callq	0x14089a9f0 <.text+0x8999f0>
1417cd9bf: f3 0f 10 05 b1 f6 1f 01     	movss	0x11ff6b1(%rip), %xmm0  # 0x1429cd078
1417cd9c7: 0f 2e c6                    	ucomiss	%xmm6, %xmm0
1417cd9ca: 0f 97 c3                    	seta	%bl
1417cd9cd: 20 c3                       	andb	%al, %bl
1417cd9cf: 41 0a 9e 89 00 00 00        	orb	0x89(%r14), %bl
1417cd9d6: 41 88 9e 89 00 00 00        	movb	%bl, 0x89(%r14)
1417cd9dd: 41 8b 87 14 01 00 00        	movl	0x114(%r15), %eax
1417cd9e4: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cd9e9: 75 04                       	jne	0x1417cd9ef <.text+0x17cc9ef>
1417cd9eb: 31 c0                       	xorl	%eax, %eax
1417cd9ed: eb 23                       	jmp	0x1417cda12 <.text+0x17cca12>
1417cd9ef: 49 8b 8f 18 01 00 00        	movq	0x118(%r15), %rcx
1417cd9f6: 89 44 24 20                 	movl	%eax, 0x20(%rsp)
1417cd9fa: 48 89 4c 24 24              	movq	%rcx, 0x24(%rsp)
1417cd9ff: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417cda04: 48 89 f1                    	movq	%rsi, %rcx
1417cda07: e8 e4 d4 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417cda0c: 48 85 c0                    	testq	%rax, %rax
1417cda0f: 0f 95 c0                    	setne	%al
1417cda12: 08 c3                       	orb	%al, %bl
1417cda14: 41 88 9e 89 00 00 00        	movb	%bl, 0x89(%r14)
1417cda1b: 41 0f b7 97 bc 01 00 00     	movzwl	0x1bc(%r15), %edx
1417cda23: 45 0f b7 87 be 01 00 00     	movzwl	0x1be(%r15), %r8d
1417cda2b: 48 89 f9                    	movq	%rdi, %rcx
1417cda2e: e8 1d d1 0c ff              	callq	0x14089ab50 <.text+0x899b50>
1417cda33: 41 08 86 8a 00 00 00        	orb	%al, 0x8a(%r14)
1417cda3a: 41 0f b7 97 c0 01 00 00     	movzwl	0x1c0(%r15), %edx
1417cda42: 45 0f b7 87 c2 01 00 00     	movzwl	0x1c2(%r15), %r8d
1417cda4a: 48 89 f9                    	movq	%rdi, %rcx
1417cda4d: e8 fe d0 0c ff              	callq	0x14089ab50 <.text+0x899b50>
1417cda52: 41 08 86 8b 00 00 00        	orb	%al, 0x8b(%r14)
1417cda59: 41 8b 47 78                 	movl	0x78(%r15), %eax
1417cda5d: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cda62: 0f 85 13 03 00 00           	jne	0x1417cdd7b <.text+0x17ccd7b>
1417cda68: 31 c0                       	xorl	%eax, %eax
1417cda6a: 41 08 86 8c 00 00 00        	orb	%al, 0x8c(%r14)
1417cda71: 41 8b 87 84 00 00 00        	movl	0x84(%r15), %eax
1417cda78: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cda7d: 0f 84 31 03 00 00           	je	0x1417cddb4 <.text+0x17ccdb4>
1417cda83: 49 8b 8f 88 00 00 00        	movq	0x88(%r15), %rcx
1417cda8a: 89 44 24 20                 	movl	%eax, 0x20(%rsp)
1417cda8e: 48 89 4c 24 24              	movq	%rcx, 0x24(%rsp)
1417cda93: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417cda98: 48 89 f1                    	movq	%rsi, %rcx
1417cda9b: e8 50 d4 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417cdaa0: 48 85 c0                    	testq	%rax, %rax
1417cdaa3: 0f 95 c0                    	setne	%al
1417cdaa6: 41 08 86 8d 00 00 00        	orb	%al, 0x8d(%r14)
1417cdaad: 41 8b 87 90 00 00 00        	movl	0x90(%r15), %eax
1417cdab4: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cdab9: 0f 84 10 03 00 00           	je	0x1417cddcf <.text+0x17ccdcf>
1417cdabf: 49 8b 8f 94 00 00 00        	movq	0x94(%r15), %rcx
1417cdac6: 89 44 24 20                 	movl	%eax, 0x20(%rsp)
1417cdaca: 48 89 4c 24 24              	movq	%rcx, 0x24(%rsp)
1417cdacf: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417cdad4: 48 89 f1                    	movq	%rsi, %rcx
1417cdad7: e8 14 d4 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417cdadc: 48 85 c0                    	testq	%rax, %rax
1417cdadf: 0f 95 c0                    	setne	%al
1417cdae2: 41 08 86 8e 00 00 00        	orb	%al, 0x8e(%r14)
1417cdae9: 41 8b 87 9c 00 00 00        	movl	0x9c(%r15), %eax
1417cdaf0: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cdaf5: 0f 84 ef 02 00 00           	je	0x1417cddea <.text+0x17ccdea>
1417cdafb: 49 8b 8f a0 00 00 00        	movq	0xa0(%r15), %rcx
1417cdb02: 89 44 24 20                 	movl	%eax, 0x20(%rsp)
1417cdb06: 48 89 4c 24 24              	movq	%rcx, 0x24(%rsp)
1417cdb0b: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417cdb10: 48 89 f1                    	movq	%rsi, %rcx
1417cdb13: e8 d8 d3 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417cdb18: 48 85 c0                    	testq	%rax, %rax
1417cdb1b: 0f 95 c0                    	setne	%al
1417cdb1e: 41 08 86 8f 00 00 00        	orb	%al, 0x8f(%r14)
1417cdb25: 41 8b 87 a8 00 00 00        	movl	0xa8(%r15), %eax
1417cdb2c: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cdb31: 0f 84 ce 02 00 00           	je	0x1417cde05 <.text+0x17cce05>
1417cdb37: 49 8b 8f ac 00 00 00        	movq	0xac(%r15), %rcx
1417cdb3e: 89 44 24 20                 	movl	%eax, 0x20(%rsp)
1417cdb42: 48 89 4c 24 24              	movq	%rcx, 0x24(%rsp)
1417cdb47: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417cdb4c: 48 89 f1                    	movq	%rsi, %rcx
1417cdb4f: e8 9c d3 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417cdb54: 48 85 c0                    	testq	%rax, %rax
1417cdb57: 0f 95 c0                    	setne	%al
1417cdb5a: 41 08 86 91 00 00 00        	orb	%al, 0x91(%r14)
1417cdb61: 41 8b 87 b4 00 00 00        	movl	0xb4(%r15), %eax
1417cdb68: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cdb6d: 0f 84 ad 02 00 00           	je	0x1417cde20 <.text+0x17cce20>
1417cdb73: 49 8b 8f b8 00 00 00        	movq	0xb8(%r15), %rcx
1417cdb7a: 89 44 24 20                 	movl	%eax, 0x20(%rsp)
1417cdb7e: 48 89 4c 24 24              	movq	%rcx, 0x24(%rsp)
1417cdb83: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417cdb88: 48 89 f1                    	movq	%rsi, %rcx
1417cdb8b: e8 60 d3 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417cdb90: 48 85 c0                    	testq	%rax, %rax
1417cdb93: 0f 95 c0                    	setne	%al
1417cdb96: 41 08 86 90 00 00 00        	orb	%al, 0x90(%r14)
1417cdb9d: 41 8b 87 c0 00 00 00        	movl	0xc0(%r15), %eax
1417cdba4: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cdba9: 0f 84 8c 02 00 00           	je	0x1417cde3b <.text+0x17cce3b>
1417cdbaf: 49 8b 8f c4 00 00 00        	movq	0xc4(%r15), %rcx
1417cdbb6: 89 44 24 20                 	movl	%eax, 0x20(%rsp)
1417cdbba: 48 89 4c 24 24              	movq	%rcx, 0x24(%rsp)
1417cdbbf: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417cdbc4: 48 89 f1                    	movq	%rsi, %rcx
1417cdbc7: e8 24 d3 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417cdbcc: 48 85 c0                    	testq	%rax, %rax
1417cdbcf: 0f 95 c0                    	setne	%al
1417cdbd2: 41 08 86 92 00 00 00        	orb	%al, 0x92(%r14)
1417cdbd9: 41 8b 87 cc 00 00 00        	movl	0xcc(%r15), %eax
1417cdbe0: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cdbe5: 0f 84 6b 02 00 00           	je	0x1417cde56 <.text+0x17cce56>
1417cdbeb: 49 8b 8f d0 00 00 00        	movq	0xd0(%r15), %rcx
1417cdbf2: 89 44 24 20                 	movl	%eax, 0x20(%rsp)
1417cdbf6: 48 89 4c 24 24              	movq	%rcx, 0x24(%rsp)
1417cdbfb: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417cdc00: 48 89 f1                    	movq	%rsi, %rcx
1417cdc03: e8 e8 d2 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417cdc08: 48 85 c0                    	testq	%rax, %rax
1417cdc0b: 0f 95 c0                    	setne	%al
1417cdc0e: 41 08 86 93 00 00 00        	orb	%al, 0x93(%r14)
1417cdc15: 41 8b 87 d8 00 00 00        	movl	0xd8(%r15), %eax
1417cdc1c: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cdc21: 0f 84 4a 02 00 00           	je	0x1417cde71 <.text+0x17cce71>
1417cdc27: 49 8b 8f dc 00 00 00        	movq	0xdc(%r15), %rcx
1417cdc2e: 89 44 24 20                 	movl	%eax, 0x20(%rsp)
1417cdc32: 48 89 4c 24 24              	movq	%rcx, 0x24(%rsp)
1417cdc37: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417cdc3c: 48 89 f1                    	movq	%rsi, %rcx
1417cdc3f: e8 ac d2 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417cdc44: 48 85 c0                    	testq	%rax, %rax
1417cdc47: 0f 95 c0                    	setne	%al
1417cdc4a: 41 08 86 94 00 00 00        	orb	%al, 0x94(%r14)
1417cdc51: 41 8b 87 e4 00 00 00        	movl	0xe4(%r15), %eax
1417cdc58: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cdc5d: 0f 84 29 02 00 00           	je	0x1417cde8c <.text+0x17cce8c>
1417cdc63: 49 8b 8f e8 00 00 00        	movq	0xe8(%r15), %rcx
1417cdc6a: 89 44 24 20                 	movl	%eax, 0x20(%rsp)
1417cdc6e: 48 89 4c 24 24              	movq	%rcx, 0x24(%rsp)
1417cdc73: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417cdc78: 48 89 f1                    	movq	%rsi, %rcx
1417cdc7b: e8 70 d2 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417cdc80: 48 85 c0                    	testq	%rax, %rax
1417cdc83: 0f 95 c0                    	setne	%al
1417cdc86: 41 08 86 95 00 00 00        	orb	%al, 0x95(%r14)
1417cdc8d: 41 8b 87 f0 00 00 00        	movl	0xf0(%r15), %eax
1417cdc94: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cdc99: 0f 84 08 02 00 00           	je	0x1417cdea7 <.text+0x17ccea7>
1417cdc9f: 49 8b 8f f4 00 00 00        	movq	0xf4(%r15), %rcx
1417cdca6: 89 44 24 20                 	movl	%eax, 0x20(%rsp)
1417cdcaa: 48 89 4c 24 24              	movq	%rcx, 0x24(%rsp)
1417cdcaf: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417cdcb4: 48 89 f1                    	movq	%rsi, %rcx
1417cdcb7: e8 34 d2 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417cdcbc: 48 85 c0                    	testq	%rax, %rax
1417cdcbf: 0f 95 c0                    	setne	%al
1417cdcc2: 41 08 86 96 00 00 00        	orb	%al, 0x96(%r14)
1417cdcc9: 41 8b 87 20 01 00 00        	movl	0x120(%r15), %eax
1417cdcd0: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cdcd5: 0f 84 e7 01 00 00           	je	0x1417cdec2 <.text+0x17ccec2>
1417cdcdb: 49 8b 8f 24 01 00 00        	movq	0x124(%r15), %rcx
1417cdce2: 89 44 24 20                 	movl	%eax, 0x20(%rsp)
1417cdce6: 48 89 4c 24 24              	movq	%rcx, 0x24(%rsp)
1417cdceb: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417cdcf0: 48 89 f1                    	movq	%rsi, %rcx
1417cdcf3: e8 f8 d1 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417cdcf8: 48 85 c0                    	testq	%rax, %rax
1417cdcfb: 0f 95 c0                    	setne	%al
1417cdcfe: 41 08 86 97 00 00 00        	orb	%al, 0x97(%r14)
1417cdd05: 41 8b 87 fc 00 00 00        	movl	0xfc(%r15), %eax
1417cdd0c: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cdd11: 0f 84 c6 01 00 00           	je	0x1417cdedd <.text+0x17ccedd>
1417cdd17: 49 8b 8f 00 01 00 00        	movq	0x100(%r15), %rcx
1417cdd1e: 89 44 24 20                 	movl	%eax, 0x20(%rsp)
1417cdd22: 48 89 4c 24 24              	movq	%rcx, 0x24(%rsp)
1417cdd27: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417cdd2c: 48 89 f1                    	movq	%rsi, %rcx
1417cdd2f: e8 bc d1 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417cdd34: 48 85 c0                    	testq	%rax, %rax
1417cdd37: 0f 95 c0                    	setne	%al
1417cdd3a: 41 08 86 98 00 00 00        	orb	%al, 0x98(%r14)
1417cdd41: 41 8b 87 08 01 00 00        	movl	0x108(%r15), %eax
1417cdd48: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cdd4d: 0f 84 a5 01 00 00           	je	0x1417cdef8 <.text+0x17ccef8>
1417cdd53: 49 8b 8f 0c 01 00 00        	movq	0x10c(%r15), %rcx
1417cdd5a: 89 44 24 20                 	movl	%eax, 0x20(%rsp)
1417cdd5e: 48 89 4c 24 24              	movq	%rcx, 0x24(%rsp)
1417cdd63: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417cdd68: 48 89 f1                    	movq	%rsi, %rcx
1417cdd6b: e8 80 d1 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417cdd70: 48 85 c0                    	testq	%rax, %rax
1417cdd73: 0f 95 c0                    	setne	%al
1417cdd76: e9 7f 01 00 00              	jmp	0x1417cdefa <.text+0x17ccefa>
1417cdd7b: 49 8b 4f 7c                 	movq	0x7c(%r15), %rcx
1417cdd7f: 89 44 24 20                 	movl	%eax, 0x20(%rsp)
1417cdd83: 48 89 4c 24 24              	movq	%rcx, 0x24(%rsp)
1417cdd88: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417cdd8d: 48 89 f1                    	movq	%rsi, %rcx
1417cdd90: e8 5b d1 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417cdd95: 48 85 c0                    	testq	%rax, %rax
1417cdd98: 0f 95 c0                    	setne	%al
1417cdd9b: 41 08 86 8c 00 00 00        	orb	%al, 0x8c(%r14)
1417cdda2: 41 8b 87 84 00 00 00        	movl	0x84(%r15), %eax
1417cdda9: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cddae: 0f 85 cf fc ff ff           	jne	0x1417cda83 <.text+0x17cca83>
1417cddb4: 31 c0                       	xorl	%eax, %eax
1417cddb6: 41 08 86 8d 00 00 00        	orb	%al, 0x8d(%r14)
1417cddbd: 41 8b 87 90 00 00 00        	movl	0x90(%r15), %eax
1417cddc4: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cddc9: 0f 85 f0 fc ff ff           	jne	0x1417cdabf <.text+0x17ccabf>
1417cddcf: 31 c0                       	xorl	%eax, %eax
1417cddd1: 41 08 86 8e 00 00 00        	orb	%al, 0x8e(%r14)
1417cddd8: 41 8b 87 9c 00 00 00        	movl	0x9c(%r15), %eax
1417cdddf: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cdde4: 0f 85 11 fd ff ff           	jne	0x1417cdafb <.text+0x17ccafb>
1417cddea: 31 c0                       	xorl	%eax, %eax
1417cddec: 41 08 86 8f 00 00 00        	orb	%al, 0x8f(%r14)
1417cddf3: 41 8b 87 a8 00 00 00        	movl	0xa8(%r15), %eax
1417cddfa: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cddff: 0f 85 32 fd ff ff           	jne	0x1417cdb37 <.text+0x17ccb37>
1417cde05: 31 c0                       	xorl	%eax, %eax
1417cde07: 41 08 86 91 00 00 00        	orb	%al, 0x91(%r14)
1417cde0e: 41 8b 87 b4 00 00 00        	movl	0xb4(%r15), %eax
1417cde15: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cde1a: 0f 85 53 fd ff ff           	jne	0x1417cdb73 <.text+0x17ccb73>
1417cde20: 31 c0                       	xorl	%eax, %eax
1417cde22: 41 08 86 90 00 00 00        	orb	%al, 0x90(%r14)
1417cde29: 41 8b 87 c0 00 00 00        	movl	0xc0(%r15), %eax
1417cde30: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cde35: 0f 85 74 fd ff ff           	jne	0x1417cdbaf <.text+0x17ccbaf>
1417cde3b: 31 c0                       	xorl	%eax, %eax
1417cde3d: 41 08 86 92 00 00 00        	orb	%al, 0x92(%r14)
1417cde44: 41 8b 87 cc 00 00 00        	movl	0xcc(%r15), %eax
1417cde4b: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cde50: 0f 85 95 fd ff ff           	jne	0x1417cdbeb <.text+0x17ccbeb>
1417cde56: 31 c0                       	xorl	%eax, %eax
1417cde58: 41 08 86 93 00 00 00        	orb	%al, 0x93(%r14)
1417cde5f: 41 8b 87 d8 00 00 00        	movl	0xd8(%r15), %eax
1417cde66: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cde6b: 0f 85 b6 fd ff ff           	jne	0x1417cdc27 <.text+0x17ccc27>
1417cde71: 31 c0                       	xorl	%eax, %eax
1417cde73: 41 08 86 94 00 00 00        	orb	%al, 0x94(%r14)
1417cde7a: 41 8b 87 e4 00 00 00        	movl	0xe4(%r15), %eax
1417cde81: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cde86: 0f 85 d7 fd ff ff           	jne	0x1417cdc63 <.text+0x17ccc63>
1417cde8c: 31 c0                       	xorl	%eax, %eax
1417cde8e: 41 08 86 95 00 00 00        	orb	%al, 0x95(%r14)
1417cde95: 41 8b 87 f0 00 00 00        	movl	0xf0(%r15), %eax
1417cde9c: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cdea1: 0f 85 f8 fd ff ff           	jne	0x1417cdc9f <.text+0x17ccc9f>
1417cdea7: 31 c0                       	xorl	%eax, %eax
1417cdea9: 41 08 86 96 00 00 00        	orb	%al, 0x96(%r14)
1417cdeb0: 41 8b 87 20 01 00 00        	movl	0x120(%r15), %eax
1417cdeb7: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cdebc: 0f 85 19 fe ff ff           	jne	0x1417cdcdb <.text+0x17cccdb>
1417cdec2: 31 c0                       	xorl	%eax, %eax
1417cdec4: 41 08 86 97 00 00 00        	orb	%al, 0x97(%r14)
1417cdecb: 41 8b 87 fc 00 00 00        	movl	0xfc(%r15), %eax
1417cded2: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cded7: 0f 85 3a fe ff ff           	jne	0x1417cdd17 <.text+0x17ccd17>
1417cdedd: 31 c0                       	xorl	%eax, %eax
1417cdedf: 41 08 86 98 00 00 00        	orb	%al, 0x98(%r14)
1417cdee6: 41 8b 87 08 01 00 00        	movl	0x108(%r15), %eax
1417cdeed: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cdef2: 0f 85 5b fe ff ff           	jne	0x1417cdd53 <.text+0x17ccd53>
1417cdef8: 31 c0                       	xorl	%eax, %eax
1417cdefa: 41 08 86 99 00 00 00        	orb	%al, 0x99(%r14)
1417cdf01: c7 44 24 20 73 00 00 00     	movl	$0x73, 0x20(%rsp)
1417cdf09: 48 8d 4e 20                 	leaq	0x20(%rsi), %rcx
1417cdf0d: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417cdf12: e8 d9 cf 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417cdf17: 48 85 c0                    	testq	%rax, %rax
1417cdf1a: 0f 95 c0                    	setne	%al
1417cdf1d: 41 08 86 a3 00 00 00        	orb	%al, 0xa3(%r14)
1417cdf24: 41 8b 87 50 01 00 00        	movl	0x150(%r15), %eax
1417cdf2b: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cdf30: 0f 85 26 f3 ff ff           	jne	0x1417cd25c <.text+0x17cc25c>
1417cdf36: 31 c0                       	xorl	%eax, %eax
1417cdf38: 41 08 86 9a 00 00 00        	orb	%al, 0x9a(%r14)
1417cdf3f: 41 8b 87 5c 01 00 00        	movl	0x15c(%r15), %eax
1417cdf46: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cdf4b: 0f 85 47 f3 ff ff           	jne	0x1417cd298 <.text+0x17cc298>
1417cdf51: 31 c0                       	xorl	%eax, %eax
1417cdf53: 41 08 86 9b 00 00 00        	orb	%al, 0x9b(%r14)
1417cdf5a: 41 8b 87 68 01 00 00        	movl	0x168(%r15), %eax
1417cdf61: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cdf66: 0f 85 68 f3 ff ff           	jne	0x1417cd2d4 <.text+0x17cc2d4>
1417cdf6c: 31 c0                       	xorl	%eax, %eax
1417cdf6e: 41 08 86 9c 00 00 00        	orb	%al, 0x9c(%r14)
1417cdf75: 41 8b 87 74 01 00 00        	movl	0x174(%r15), %eax
1417cdf7c: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cdf81: 0f 85 89 f3 ff ff           	jne	0x1417cd310 <.text+0x17cc310>
1417cdf87: 31 c0                       	xorl	%eax, %eax
1417cdf89: 41 08 86 9d 00 00 00        	orb	%al, 0x9d(%r14)
1417cdf90: 41 8b 87 80 01 00 00        	movl	0x180(%r15), %eax
1417cdf97: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cdf9c: 0f 85 aa f3 ff ff           	jne	0x1417cd34c <.text+0x17cc34c>
1417cdfa2: 31 c0                       	xorl	%eax, %eax
1417cdfa4: 41 08 86 9e 00 00 00        	orb	%al, 0x9e(%r14)
1417cdfab: 41 8b 87 8c 01 00 00        	movl	0x18c(%r15), %eax
1417cdfb2: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cdfb7: 0f 85 cb f3 ff ff           	jne	0x1417cd388 <.text+0x17cc388>
1417cdfbd: 31 c0                       	xorl	%eax, %eax
1417cdfbf: 41 08 86 9f 00 00 00        	orb	%al, 0x9f(%r14)
1417cdfc6: 41 8b 87 98 01 00 00        	movl	0x198(%r15), %eax
1417cdfcd: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cdfd2: 0f 85 ec f3 ff ff           	jne	0x1417cd3c4 <.text+0x17cc3c4>
1417cdfd8: 31 c0                       	xorl	%eax, %eax
1417cdfda: 41 08 86 a0 00 00 00        	orb	%al, 0xa0(%r14)
1417cdfe1: 41 8b 87 a4 01 00 00        	movl	0x1a4(%r15), %eax
1417cdfe8: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417cdfed: 0f 85 0d f4 ff ff           	jne	0x1417cd400 <.text+0x17cc400>
1417cdff3: 31 c0                       	xorl	%eax, %eax
1417cdff5: 41 08 86 a1 00 00 00        	orb	%al, 0xa1(%r14)
1417cdffc: 4d 85 e4                    	testq	%r12, %r12
1417cdfff: 0f 85 2e f4 ff ff           	jne	0x1417cd433 <.text+0x17cc433>
1417ce005: 41 8b 87 2c 01 00 00        	movl	0x12c(%r15), %eax
1417ce00c: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417ce011: 74 57                       	je	0x1417ce06a <.text+0x17cd06a>
1417ce013: 49 8b 8f 30 01 00 00        	movq	0x130(%r15), %rcx
1417ce01a: 89 44 24 20                 	movl	%eax, 0x20(%rsp)
1417ce01e: 48 89 4c 24 24              	movq	%rcx, 0x24(%rsp)
1417ce023: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417ce028: 48 89 f1                    	movq	%rsi, %rcx
1417ce02b: e8 c0 ce 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417ce030: 48 85 c0                    	testq	%rax, %rax
1417ce033: 74 35                       	je	0x1417ce06a <.text+0x17cd06a>
1417ce035: c7 44 24 20 3d 00 00 00     	movl	$0x3d, 0x20(%rsp)
1417ce03d: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417ce042: 48 89 f1                    	movq	%rsi, %rcx
1417ce045: e8 a6 ce 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417ce04a: 48 85 c0                    	testq	%rax, %rax
1417ce04d: 0f 94 c0                    	sete	%al
1417ce050: 41 08 86 a5 00 00 00        	orb	%al, 0xa5(%r14)
1417ce057: 41 8b 87 38 01 00 00        	movl	0x138(%r15), %eax
1417ce05e: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417ce063: 75 20                       	jne	0x1417ce085 <.text+0x17cd085>
1417ce065: e9 db f3 ff ff              	jmp	0x1417cd445 <.text+0x17cc445>
1417ce06a: 31 c0                       	xorl	%eax, %eax
1417ce06c: 41 08 86 a5 00 00 00        	orb	%al, 0xa5(%r14)
1417ce073: 41 8b 87 38 01 00 00        	movl	0x138(%r15), %eax
1417ce07a: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417ce07f: 0f 84 c0 f3 ff ff           	je	0x1417cd445 <.text+0x17cc445>
1417ce085: 49 8b 8f 3c 01 00 00        	movq	0x13c(%r15), %rcx
1417ce08c: 89 44 24 20                 	movl	%eax, 0x20(%rsp)
1417ce090: 48 89 4c 24 24              	movq	%rcx, 0x24(%rsp)
1417ce095: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417ce09a: 48 89 f1                    	movq	%rsi, %rcx
1417ce09d: e8 4e ce 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417ce0a2: 48 85 c0                    	testq	%rax, %rax
1417ce0a5: 0f 95 c0                    	setne	%al
1417ce0a8: 41 08 86 a7 00 00 00        	orb	%al, 0xa7(%r14)
1417ce0af: 41 8b 87 44 01 00 00        	movl	0x144(%r15), %eax
1417ce0b6: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417ce0bb: 0f 85 9f f3 ff ff           	jne	0x1417cd460 <.text+0x17cc460>
1417ce0c1: 31 c0                       	xorl	%eax, %eax
1417ce0c3: 41 08 86 a8 00 00 00        	orb	%al, 0xa8(%r14)
1417ce0ca: 41 8b 87 b0 01 00 00        	movl	0x1b0(%r15), %eax
1417ce0d1: 3d c3 00 00 00              	cmpl	$0xc3, %eax
1417ce0d6: 0f 85 c0 f3 ff ff           	jne	0x1417cd49c <.text+0x17cc49c>
1417ce0dc: 31 c0                       	xorl	%eax, %eax
1417ce0de: 41 08 86 a9 00 00 00        	orb	%al, 0xa9(%r14)
1417ce0e5: 48 8b bc 24 10 01 00 00     	movq	0x110(%rsp), %rdi
1417ce0ed: 80 3d e4 6b e6 01 00        	cmpb	$0x0, 0x1e66be4(%rip)   # 0x143634cd8
1417ce0f4: 0f 84 49 01 00 00           	je	0x1417ce243 <.text+0x17cd243>
1417ce0fa: c7 44 24 20 35 00 00 00     	movl	$0x35, 0x20(%rsp)
1417ce102: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417ce107: 48 89 f1                    	movq	%rsi, %rcx
1417ce10a: e8 e1 cd 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417ce10f: 48 85 c0                    	testq	%rax, %rax
1417ce112: 74 1d                       	je	0x1417ce131 <.text+0x17cd131>
1417ce114: c7 44 24 20 3d 00 00 00     	movl	$0x3d, 0x20(%rsp)
1417ce11c: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417ce121: 48 89 f1                    	movq	%rsi, %rcx
1417ce124: e8 c7 cd 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417ce129: 48 85 c0                    	testq	%rax, %rax
1417ce12c: 0f 95 c0                    	setne	%al
1417ce12f: eb 02                       	jmp	0x1417ce133 <.text+0x17cd133>
1417ce131: 31 c0                       	xorl	%eax, %eax
1417ce133: 41 08 86 a2 00 00 00        	orb	%al, 0xa2(%r14)
1417ce13a: c7 44 24 20 40 00 00 00     	movl	$0x40, 0x20(%rsp)
1417ce142: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417ce147: 48 89 f1                    	movq	%rsi, %rcx
1417ce14a: e8 a1 cd 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417ce14f: 48 85 c0                    	testq	%rax, %rax
1417ce152: 0f 95 c0                    	setne	%al
1417ce155: 41 08 86 a4 00 00 00        	orb	%al, 0xa4(%r14)
1417ce15c: c7 44 24 20 ab 00 00 00     	movl	$0xab, 0x20(%rsp)
1417ce164: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417ce169: 48 89 f1                    	movq	%rsi, %rcx
1417ce16c: e8 7f cd 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417ce171: 48 85 c0                    	testq	%rax, %rax
1417ce174: 74 1d                       	je	0x1417ce193 <.text+0x17cd193>
1417ce176: c7 44 24 20 3d 00 00 00     	movl	$0x3d, 0x20(%rsp)
1417ce17e: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417ce183: 48 89 f1                    	movq	%rsi, %rcx
1417ce186: e8 65 cd 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417ce18b: 48 85 c0                    	testq	%rax, %rax
1417ce18e: 0f 95 c0                    	setne	%al
1417ce191: eb 02                       	jmp	0x1417ce195 <.text+0x17cd195>
1417ce193: 31 c0                       	xorl	%eax, %eax
1417ce195: 41 08 86 a6 00 00 00        	orb	%al, 0xa6(%r14)
1417ce19c: c7 44 24 20 25 00 00 00     	movl	$0x25, 0x20(%rsp)
1417ce1a4: 48 8d 4e 20                 	leaq	0x20(%rsi), %rcx
1417ce1a8: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417ce1ad: e8 3e cd 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417ce1b2: 48 85 c0                    	testq	%rax, %rax
1417ce1b5: 74 1d                       	je	0x1417ce1d4 <.text+0x17cd1d4>
1417ce1b7: c7 44 24 20 33 00 00 00     	movl	$0x33, 0x20(%rsp)
1417ce1bf: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417ce1c4: 48 89 f1                    	movq	%rsi, %rcx
1417ce1c7: e8 24 cd 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417ce1cc: 48 85 c0                    	testq	%rax, %rax
1417ce1cf: 0f 94 c0                    	sete	%al
1417ce1d2: eb 02                       	jmp	0x1417ce1d6 <.text+0x17cd1d6>
1417ce1d4: 31 c0                       	xorl	%eax, %eax
1417ce1d6: 41 08 86 aa 00 00 00        	orb	%al, 0xaa(%r14)
1417ce1dd: c7 44 24 20 23 00 00 00     	movl	$0x23, 0x20(%rsp)
1417ce1e5: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417ce1ea: 48 89 f1                    	movq	%rsi, %rcx
1417ce1ed: e8 fe cc 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417ce1f2: 48 85 c0                    	testq	%rax, %rax
1417ce1f5: 0f 95 c0                    	setne	%al
1417ce1f8: 41 08 86 ab 00 00 00        	orb	%al, 0xab(%r14)
1417ce1ff: c7 44 24 20 29 00 00 00     	movl	$0x29, 0x20(%rsp)
1417ce207: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417ce20c: 48 89 f1                    	movq	%rsi, %rcx
1417ce20f: e8 dc cc 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417ce214: 48 85 c0                    	testq	%rax, %rax
1417ce217: 0f 95 c0                    	setne	%al
1417ce21a: 41 08 86 ac 00 00 00        	orb	%al, 0xac(%r14)
1417ce221: c7 44 24 20 a6 00 00 00     	movl	$0xa6, 0x20(%rsp)
1417ce229: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417ce22e: 48 89 f1                    	movq	%rsi, %rcx
1417ce231: e8 ba cc 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417ce236: 48 85 c0                    	testq	%rax, %rax
1417ce239: 0f 95 c0                    	setne	%al
1417ce23c: 41 08 86 ad 00 00 00        	orb	%al, 0xad(%r14)
1417ce243: c7 44 24 20 a7 00 00 00     	movl	$0xa7, 0x20(%rsp)
1417ce24b: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1417ce250: 48 89 f1                    	movq	%rsi, %rcx
1417ce253: e8 98 cc 0c ff              	callq	0x14089aef0 <.text+0x899ef0>
1417ce258: 48 85 c0                    	testq	%rax, %rax
1417ce25b: 0f 95 c0                    	setne	%al
1417ce25e: 41 08 86 ae 00 00 00        	orb	%al, 0xae(%r14)
1417ce265: 48 8b 07                    	movq	(%rdi), %rax
1417ce268: 4c 8b 5f 08                 	movq	0x8(%rdi), %r11
1417ce26c: 48 8b 08                    	movq	(%rax), %rcx
1417ce26f: 31 f6                       	xorl	%esi, %esi
1417ce271: 48 89 ca                    	movq	%rcx, %rdx
1417ce274: 49 2b 53 18                 	subq	0x18(%r11), %rdx
1417ce278: 48 0f 42 d6                 	cmovbq	%rsi, %rdx
1417ce27c: 49 2b 4b 38                 	subq	0x38(%r11), %rcx
1417ce280: 48 0f 42 ce                 	cmovbq	%rsi, %rcx
1417ce284: 4d 8b 53 08                 	movq	0x8(%r11), %r10
1417ce288: 4d 8b 4b 30                 	movq	0x30(%r11), %r9
1417ce28c: 4d 89 c8                    	movq	%r9, %r8
1417ce28f: 49 29 c8                    	subq	%rcx, %r8
1417ce292: 4c 0f 42 c6                 	cmovbq	%rsi, %r8
1417ce296: 49 8b 5b 10                 	movq	0x10(%r11), %rbx
1417ce29a: 40 0f 96 c5                 	setbe	%bpl
1417ce29e: 48 89 df                    	movq	%rbx, %rdi
1417ce2a1: 48 29 d7                    	subq	%rdx, %rdi
1417ce2a4: 48 0f 42 fe                 	cmovbq	%rsi, %rdi
1417ce2a8: 49 8b 73 28                 	movq	0x28(%r11), %rsi
1417ce2ac: 4d 8b 5b 40                 	movq	0x40(%r11), %r11
1417ce2b0: 41 0f 96 c7                 	setbe	%r15b
1417ce2b4: 4d 89 dd                    	movq	%r11, %r13
1417ce2b7: 4d 29 c5                    	subq	%r8, %r13
1417ce2ba: 49 29 fd                    	subq	%rdi, %r13
1417ce2bd: 4c 89 28                    	movq	%r13, (%rax)
1417ce2c0: 41 84 ef                    	testb	%bpl, %r15b
1417ce2c3: 75 5f                       	jne	0x1417ce324 <.text+0x17cd324>
1417ce2c5: 49 89 cf                    	movq	%rcx, %r15
1417ce2c8: 49 c1 e7 04                 	shlq	$0x4, %r15
1417ce2cc: 4c 01 fe                    	addq	%r15, %rsi
1417ce2cf: 49 c1 e0 04                 	shlq	$0x4, %r8
1417ce2d3: 49 89 d7                    	movq	%rdx, %r15
1417ce2d6: 49 c1 e7 04                 	shlq	$0x4, %r15
1417ce2da: 4d 01 fa                    	addq	%r15, %r10
1417ce2dd: 48 c1 e7 04                 	shlq	$0x4, %rdi
1417ce2e1: 45 31 ff                    	xorl	%r15d, %r15d
1417ce2e4: 48 39 d3                    	cmpq	%rdx, %rbx
1417ce2e7: ba 08 00 00 00              	movl	$0x8, %edx
1417ce2ec: 4c 0f 42 d2                 	cmovbq	%rdx, %r10
1417ce2f0: 4d 8d 54 3a f0              	leaq	-0x10(%r10,%rdi), %r10
1417ce2f5: 4d 0f 47 fa                 	cmovaq	%r10, %r15
1417ce2f9: 49 39 c9                    	cmpq	%rcx, %r9
1417ce2fc: 48 0f 42 f2                 	cmovbq	%rdx, %rsi
1417ce300: 4a 8d 4c 06 f0              	leaq	-0x10(%rsi,%r8), %rcx
1417ce305: 49 0f 46 cf                 	cmovbeq	%r15, %rcx
1417ce309: 4c 89 18                    	movq	%r11, (%rax)
1417ce30c: 80 79 08 00                 	cmpb	$0x0, 0x8(%rcx)
1417ce310: 74 0a                       	je	0x1417ce31c <.text+0x17cd31c>
1417ce312: 41 c6 86 99 00 00 00 01     	movb	$0x1, 0x99(%r14)
1417ce31a: eb 08                       	jmp	0x1417ce324 <.text+0x17cd324>
1417ce31c: 41 c6 86 98 00 00 00 01     	movb	$0x1, 0x98(%r14)
1417ce324: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1417ce329: 0f 28 7c 24 60              	movaps	0x60(%rsp), %xmm7
1417ce32e: 48 83 c4 78                 	addq	$0x78, %rsp
1417ce332: 5b                          	popq	%rbx
1417ce333: 5d                          	popq	%rbp
1417ce334: 5f                          	popq	%rdi
1417ce335: 5e                          	popq	%rsi
1417ce336: 41 5c                       	popq	%r12
1417ce338: 41 5d                       	popq	%r13
1417ce33a: 41 5e                       	popq	%r14
1417ce33c: 41 5f                       	popq	%r15
1417ce33e: c3                          	retq
1417ce33f: 0f 0b                       	ud2
1417ce341: cc                          	int3
1417ce342: cc                          	int3
1417ce343: cc                          	int3
1417ce344: cc                          	int3
1417ce345: cc                          	int3
1417ce346: cc                          	int3
1417ce347: cc                          	int3
1417ce348: cc                          	int3
1417ce349: cc                          	int3
1417ce34a: cc                          	int3
1417ce34b: cc                          	int3
1417ce34c: cc                          	int3
1417ce34d: cc                          	int3
1417ce34e: cc                          	int3
1417ce34f: cc                          	int3
1417ce350: 41 57                       	pushq	%r15
1417ce352: 41 56                       	pushq	%r14
1417ce354: 41 54                       	pushq	%r12
1417ce356: 56                          	pushq	%rsi
1417ce357: 57                          	pushq	%rdi
1417ce358: 53                          	pushq	%rbx
1417ce359: 48 83 ec 68                 	subq	$0x68, %rsp
1417ce35d: 48 89 d7                    	movq	%rdx, %rdi
1417ce360: 48 89 ce                    	movq	%rcx, %rsi
1417ce363: 83 39 01                    	cmpl	$0x1, (%rcx)
1417ce366: 75 60                       	jne	0x1417ce3c8 <.text+0x17cd3c8>
1417ce368: 48 8b 87 a0 05 00 00        	movq	0x5a0(%rdi), %rax
1417ce36f: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
1417ce374: 48 39 46 28                 	cmpq	%rax, 0x28(%rsi)
1417ce378: 0f 84 a5 00 00 00           	je	0x1417ce423 <.text+0x17cd423>
1417ce37e: 48 8d 05 93 5a 51 01        	leaq	0x1515a93(%rip), %rax   # 0x142ce3e18
1417ce385: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
1417ce38a: 48 c7 44 24 40 01 00 00 00  	movq	$0x1, 0x40(%rsp)
1417ce393: 48 c7 44 24 48 08 00 00 00  	movq	$0x8, 0x48(%rsp)
1417ce39c: 0f 57 c0                    	xorps	%xmm0, %xmm0
1417ce39f: 0f 11 44 24 50              	movups	%xmm0, 0x50(%rsp)
1417ce3a4: 48 83 c6 28                 	addq	$0x28, %rsi
1417ce3a8: 48 8d 05 e9 5a 51 01        	leaq	0x1515ae9(%rip), %rax   # 0x142ce3e98
1417ce3af: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1417ce3b4: 4c 8d 44 24 30              	leaq	0x30(%rsp), %r8
1417ce3b9: 4c 8d 4c 24 38              	leaq	0x38(%rsp), %r9
1417ce3be: 31 c9                       	xorl	%ecx, %ecx
1417ce3c0: 48 89 f2                    	movq	%rsi, %rdx
1417ce3c3: e8 19 fd 0f 01              	callq	0x1428ce0e1 <.text+0x28cd0e1>
1417ce3c8: 48 8d 5e 30                 	leaq	0x30(%rsi), %rbx
1417ce3cc: 48 89 f9                    	movq	%rdi, %rcx
1417ce3cf: 48 89 da                    	movq	%rbx, %rdx
1417ce3d2: e8 39 2c b9 ff              	callq	0x141361010 <.text+0x1360010>
1417ce3d7: 49 89 c6                    	movq	%rax, %r14
1417ce3da: 48 89 f9                    	movq	%rdi, %rcx
1417ce3dd: 48 89 da                    	movq	%rbx, %rdx
1417ce3e0: e8 7b 26 b8 ff              	callq	0x141350a60 <.text+0x134fa60>
1417ce3e5: 49 89 c7                    	movq	%rax, %r15
1417ce3e8: 48 89 f9                    	movq	%rdi, %rcx
1417ce3eb: 48 89 da                    	movq	%rbx, %rdx
1417ce3ee: e8 bd 28 b8 ff              	callq	0x141350cb0 <.text+0x134fcb0>
1417ce3f3: 49 89 c4                    	movq	%rax, %r12
1417ce3f6: 48 89 f9                    	movq	%rdi, %rcx
1417ce3f9: 48 89 da                    	movq	%rbx, %rdx
1417ce3fc: e8 0f 4f b9 ff              	callq	0x141363310 <.text+0x1362310>
1417ce401: 48 8b 8f a0 05 00 00        	movq	0x5a0(%rdi), %rcx
1417ce408: 48 c7 06 01 00 00 00        	movq	$0x1, (%rsi)
1417ce40f: 4c 89 76 08                 	movq	%r14, 0x8(%rsi)
1417ce413: 4c 89 7e 10                 	movq	%r15, 0x10(%rsi)
1417ce417: 4c 89 66 18                 	movq	%r12, 0x18(%rsi)
1417ce41b: 48 89 46 20                 	movq	%rax, 0x20(%rsi)
1417ce41f: 48 89 4e 28                 	movq	%rcx, 0x28(%rsi)
1417ce423: b8 00 50 cc 3d              	movl	$0x3dcc5000, %eax       # imm = 0x3DCC5000
1417ce428: 03 87 b0 05 00 00           	addl	0x5b0(%rdi), %eax
1417ce42e: 89 86 60 01 00 00           	movl	%eax, 0x160(%rsi)
1417ce434: 48 83 c4 68                 	addq	$0x68, %rsp
1417ce438: 5b                          	popq	%rbx
1417ce439: 5f                          	popq	%rdi
1417ce43a: 5e                          	popq	%rsi
1417ce43b: 41 5c                       	popq	%r12
1417ce43d: 41 5e                       	popq	%r14
1417ce43f: 41 5f                       	popq	%r15
1417ce441: c3                          	retq
1417ce442: cc                          	int3
1417ce443: cc                          	int3
1417ce444: cc                          	int3
1417ce445: cc                          	int3
1417ce446: cc                          	int3
1417ce447: cc                          	int3
1417ce448: cc                          	int3
1417ce449: cc                          	int3
1417ce44a: cc                          	int3
1417ce44b: cc                          	int3
1417ce44c: cc                          	int3
1417ce44d: cc                          	int3
1417ce44e: cc                          	int3
1417ce44f: cc                          	int3
1417ce450: 41 57                       	pushq	%r15
1417ce452: 41 56                       	pushq	%r14
1417ce454: 41 55                       	pushq	%r13
1417ce456: 41 54                       	pushq	%r12
1417ce458: 56                          	pushq	%rsi
1417ce459: 57                          	pushq	%rdi
1417ce45a: 53                          	pushq	%rbx
1417ce45b: 48 83 ec 60                 	subq	$0x60, %rsp
1417ce45f: 48 89 d7                    	movq	%rdx, %rdi
1417ce462: 48 89 ce                    	movq	%rcx, %rsi
1417ce465: 83 39 01                    	cmpl	$0x1, (%rcx)
1417ce468: 75 60                       	jne	0x1417ce4ca <.text+0x17cd4ca>
1417ce46a: 48 8b 87 a0 05 00 00        	movq	0x5a0(%rdi), %rax
1417ce471: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
1417ce476: 48 39 46 30                 	cmpq	%rax, 0x30(%rsi)
1417ce47a: 0f 84 b7 00 00 00           	je	0x1417ce537 <.text+0x17cd537>
1417ce480: 48 8d 05 91 59 51 01        	leaq	0x1515991(%rip), %rax   # 0x142ce3e18
1417ce487: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
1417ce48c: 48 c7 44 24 38 01 00 00 00  	movq	$0x1, 0x38(%rsp)
1417ce495: 48 c7 44 24 40 08 00 00 00  	movq	$0x8, 0x40(%rsp)
1417ce49e: 0f 57 c0                    	xorps	%xmm0, %xmm0
1417ce4a1: 0f 11 44 24 48              	movups	%xmm0, 0x48(%rsp)
1417ce4a6: 48 83 c6 30                 	addq	$0x30, %rsi
1417ce4aa: 48 8d 05 e7 59 51 01        	leaq	0x15159e7(%rip), %rax   # 0x142ce3e98
1417ce4b1: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1417ce4b6: 4c 8d 44 24 28              	leaq	0x28(%rsp), %r8
1417ce4bb: 4c 8d 4c 24 30              	leaq	0x30(%rsp), %r9
1417ce4c0: 31 c9                       	xorl	%ecx, %ecx
1417ce4c2: 48 89 f2                    	movq	%rsi, %rdx
1417ce4c5: e8 17 fc 0f 01              	callq	0x1428ce0e1 <.text+0x28cd0e1>
1417ce4ca: 48 8d 5e 38                 	leaq	0x38(%rsi), %rbx
1417ce4ce: 48 89 f9                    	movq	%rdi, %rcx
1417ce4d1: 48 89 da                    	movq	%rbx, %rdx
1417ce4d4: e8 f7 9e b7 ff              	callq	0x1413483d0 <.text+0x13473d0>
1417ce4d9: 49 89 c6                    	movq	%rax, %r14
1417ce4dc: 48 89 f9                    	movq	%rdi, %rcx
1417ce4df: 48 89 da                    	movq	%rbx, %rdx
1417ce4e2: e8 79 25 b8 ff              	callq	0x141350a60 <.text+0x134fa60>
1417ce4e7: 49 89 c7                    	movq	%rax, %r15
1417ce4ea: 48 89 f9                    	movq	%rdi, %rcx
1417ce4ed: 48 89 da                    	movq	%rbx, %rdx
1417ce4f0: e8 2b 24 b9 ff              	callq	0x141360920 <.text+0x135f920>
1417ce4f5: 49 89 c4                    	movq	%rax, %r12
1417ce4f8: 48 89 f9                    	movq	%rdi, %rcx
1417ce4fb: 48 89 da                    	movq	%rbx, %rdx
1417ce4fe: e8 ad 27 b8 ff              	callq	0x141350cb0 <.text+0x134fcb0>

; SETTINGS KEY PAGE: bounded original complete root disassembly excerpt
140246304: 80 38 0a                    	cmpb	$0xa, (%rax)
140246307: 75 7f                       	jne	0x140246388 <.text+0x245388>
140246309: 48 89 df                    	movq	%rbx, %rdi
14024630c: 48 8d 15 6b a7 75 02        	leaq	0x275a76b(%rip), %rdx   # 0x1429a0a7e
140246313: 48 8d 8d 58 03 00 00        	leaq	0x358(%rbp), %rcx
14024631a: 41 b8 06 00 00 00           	movl	$0x6, %r8d
140246320: e8 4b b5 2b 00              	callq	0x140501870 <.text+0x500870>
140246325: 49 8d 5d 10                 	leaq	0x10(%r13), %rbx
140246329: 48 89 9d 80 01 00 00        	movq	%rbx, 0x180(%rbp)
140246330: 49 8d 75 1c                 	leaq	0x1c(%r13), %rsi
140246334: 48 89 b5 30 03 00 00        	movq	%rsi, 0x330(%rbp)
14024633b: 41 80 7d 08 03              	cmpb	$0x3, 0x8(%r13)
140246340: 0f 85 3b 03 00 00           	jne	0x140246681 <.text+0x245681>
140246346: 45 85 ff                    	testl	%r15d, %r15d
140246349: 0f 8e ab 05 00 00           	jle	0x1402468fa <.text+0x2458fa>
14024634f: 49 8b 45 00                 	movq	(%r13), %rax
140246353: 44 89 f9                    	movl	%r15d, %ecx
140246356: 83 e1 03                    	andl	$0x3, %ecx
140246359: 0f 84 0e 09 00 00           	je	0x140246c6d <.text+0x245c6d>
14024635f: 31 d2                       	xorl	%edx, %edx
140246361: 4c 8d 05 e0 b2 75 02        	leaq	0x275b2e0(%rip), %r8    # 0x1429a1648
140246368: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
140246370: 49 8b 04 c0                 	movq	(%r8,%rax,8), %rax
140246374: ff c2                       	incl	%edx
140246376: 39 d1                       	cmpl	%edx, %ecx
140246378: 75 f6                       	jne	0x140246370 <.text+0x245370>
14024637a: 49 89 45 00                 	movq	%rax, (%r13)
14024637e: 44 89 f9                    	movl	%r15d, %ecx
140246381: 29 d1                       	subl	%edx, %ecx
140246383: e9 e8 08 00 00              	jmp	0x140246c70 <.text+0x245c70>
140246388: 48 8b bd f8 04 00 00        	movq	0x4f8(%rbp), %rdi
14024638f: 40 84 f6                    	testb	%sil, %sil
140246392: 75 5b                       	jne	0x1402463ef <.text+0x2453ef>
140246394: 48 8d 15 e3 a6 75 02        	leaq	0x275a6e3(%rip), %rdx   # 0x1429a0a7e
14024639b: 48 8d b5 58 03 00 00        	leaq	0x358(%rbp), %rsi
1402463a2: 41 b8 06 00 00 00           	movl	$0x6, %r8d
1402463a8: 48 89 f1                    	movq	%rsi, %rcx
1402463ab: e8 00 af 2b 00              	callq	0x1405012b0 <.text+0x5002b0>
1402463b0: 48 8b 85 f8 03 00 00        	movq	0x3f8(%rbp), %rax
1402463b7: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
1402463be: 48 89 9d c8 01 00 00        	movq	%rbx, 0x1c8(%rbp)
1402463c5: 48 8d 85 48 03 00 00        	leaq	0x348(%rbp), %rax
1402463cc: 48 89 85 d0 01 00 00        	movq	%rax, 0x1d0(%rbp)
1402463d3: 48 8d 15 aa a6 75 02        	leaq	0x275a6aa(%rip), %rdx   # 0x1429a0a84
1402463da: 4c 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %r9
1402463e1: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
1402463e7: 48 89 f1                    	movq	%rsi, %rcx
1402463ea: e8 01 88 2b 00              	callq	0x1404febf0 <.text+0x4fdbf0>
1402463ef: 48 c7 85 a0 03 00 00 00 00 00 00    	movq	$0x0, 0x3a0(%rbp)
1402463fa: 48 c7 85 a8 03 00 00 01 00 00 00    	movq	$0x1, 0x3a8(%rbp)
140246405: 48 c7 85 b0 03 00 00 00 00 00 00    	movq	$0x0, 0x3b0(%rbp)
140246410: c6 85 30 03 00 00 01        	movb	$0x1, 0x330(%rbp)
140246417: 48 8d 85 10 03 00 00        	leaq	0x310(%rbp), %rax
14024641e: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
140246425: 48 89 bd c8 01 00 00        	movq	%rdi, 0x1c8(%rbp)
14024642c: 48 8d b5 a0 03 00 00        	leaq	0x3a0(%rbp), %rsi
140246433: 48 89 b5 d0 01 00 00        	movq	%rsi, 0x1d0(%rbp)
14024643a: 48 8d 85 a0 00 00 00        	leaq	0xa0(%rbp), %rax
140246441: 48 89 85 d8 01 00 00        	movq	%rax, 0x1d8(%rbp)
140246448: 0f 28 45 40                 	movaps	0x40(%rbp), %xmm0
14024644c: 0f 28 4d 50                 	movaps	0x50(%rbp), %xmm1
140246450: 0f 11 85 e0 01 00 00        	movups	%xmm0, 0x1e0(%rbp)
140246457: 0f 11 8d f0 01 00 00        	movups	%xmm1, 0x1f0(%rbp)
14024645e: 0f 28 85 00 01 00 00        	movaps	0x100(%rbp), %xmm0
140246465: 0f 28 8d 10 01 00 00        	movaps	0x110(%rbp), %xmm1
14024646c: 0f 11 8d 10 02 00 00        	movups	%xmm1, 0x210(%rbp)
140246473: 0f 11 85 00 02 00 00        	movups	%xmm0, 0x200(%rbp)
14024647a: 41 0f 10 06                 	movups	(%r14), %xmm0
14024647e: 41 0f 10 4e 10              	movups	0x10(%r14), %xmm1
140246483: 0f 11 85 20 02 00 00        	movups	%xmm0, 0x220(%rbp)
14024648a: 0f 11 8d 30 02 00 00        	movups	%xmm1, 0x230(%rbp)
140246491: 0f 28 85 20 01 00 00        	movaps	0x120(%rbp), %xmm0
140246498: 0f 28 8d 30 01 00 00        	movaps	0x130(%rbp), %xmm1
14024649f: 0f 11 85 40 02 00 00        	movups	%xmm0, 0x240(%rbp)
1402464a6: 0f 11 8d 50 02 00 00        	movups	%xmm1, 0x250(%rbp)
1402464ad: 0f 28 85 80 00 00 00        	movaps	0x80(%rbp), %xmm0
1402464b4: 0f 28 8d 90 00 00 00        	movaps	0x90(%rbp), %xmm1
1402464bb: 0f 11 8d 70 02 00 00        	movups	%xmm1, 0x270(%rbp)
1402464c2: 0f 11 85 60 02 00 00        	movups	%xmm0, 0x260(%rbp)
1402464c9: 0f 28 45 60                 	movaps	0x60(%rbp), %xmm0
1402464cd: 0f 28 4d 70                 	movaps	0x70(%rbp), %xmm1
1402464d1: 0f 11 8d 90 02 00 00        	movups	%xmm1, 0x290(%rbp)
1402464d8: 0f 11 85 80 02 00 00        	movups	%xmm0, 0x280(%rbp)
1402464df: 48 89 9d a0 02 00 00        	movq	%rbx, 0x2a0(%rbp)
1402464e6: 48 8d 45 c0                 	leaq	-0x40(%rbp), %rax
1402464ea: 48 89 85 a8 02 00 00        	movq	%rax, 0x2a8(%rbp)
1402464f1: 48 8d 85 30 03 00 00        	leaq	0x330(%rbp), %rax
1402464f8: 48 89 85 b0 02 00 00        	movq	%rax, 0x2b0(%rbp)
1402464ff: 0f 28 85 e0 00 00 00        	movaps	0xe0(%rbp), %xmm0
140246506: 0f 28 8d f0 00 00 00        	movaps	0xf0(%rbp), %xmm1
14024650d: 0f 11 8d e0 02 00 00        	movups	%xmm1, 0x2e0(%rbp)
140246514: 0f 11 85 d0 02 00 00        	movups	%xmm0, 0x2d0(%rbp)
14024651b: 0f 28 85 c0 00 00 00        	movaps	0xc0(%rbp), %xmm0
140246522: 0f 28 8d d0 00 00 00        	movaps	0xd0(%rbp), %xmm1
140246529: 0f 11 8d 00 03 00 00        	movups	%xmm1, 0x300(%rbp)
140246530: 0f 11 85 f0 02 00 00        	movups	%xmm0, 0x2f0(%rbp)
140246537: 48 8b 85 88 01 00 00        	movq	0x188(%rbp), %rax
14024653e: 48 89 85 b8 02 00 00        	movq	%rax, 0x2b8(%rbp)
140246545: 48 89 e8                    	movq	%rbp, %rax
140246548: 48 89 85 c0 02 00 00        	movq	%rax, 0x2c0(%rbp)
14024654f: 48 8d 45 e0                 	leaq	-0x20(%rbp), %rax
140246553: 48 89 85 c8 02 00 00        	movq	%rax, 0x2c8(%rbp)
14024655a: 48 8d 15 e9 a4 75 02        	leaq	0x275a4e9(%rip), %rdx   # 0x1429a0a4a
140246561: 48 8d 8d 58 03 00 00        	leaq	0x358(%rbp), %rcx
140246568: 4c 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %r9
14024656f: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140246575: e8 66 c5 2b 00              	callq	0x140502ae0 <.text+0x501ae0>
14024657a: 48 89 5c 24 20              	movq	%rbx, 0x20(%rsp)
14024657f: 48 8d 15 08 a5 75 02        	leaq	0x275a508(%rip), %rdx   # 0x1429a0a8e
140246586: 48 8d 8d 58 03 00 00        	leaq	0x358(%rbp), %rcx
14024658d: 4c 8d 8d 10 03 00 00        	leaq	0x310(%rbp), %r9
140246594: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14024659a: e8 61 b6 2b 00              	callq	0x140501c00 <.text+0x500c00>
14024659f: 80 bd 30 03 00 00 00        	cmpb	$0x0, 0x330(%rbp)
1402465a6: 0f 84 ae 00 00 00           	je	0x14024665a <.text+0x24565a>
1402465ac: 48 8b 85 78 03 00 00        	movq	0x378(%rbp), %rax
1402465b3: 48 89 85 e0 01 00 00        	movq	%rax, 0x1e0(%rbp)
1402465ba: 0f 10 85 58 03 00 00        	movups	0x358(%rbp), %xmm0
1402465c1: 0f 10 8d 68 03 00 00        	movups	0x368(%rbp), %xmm1
1402465c8: 0f 29 8d d0 01 00 00        	movaps	%xmm1, 0x1d0(%rbp)
1402465cf: 0f 29 85 c0 01 00 00        	movaps	%xmm0, 0x1c0(%rbp)
1402465d6: 4c 8d 05 b5 a4 75 02        	leaq	0x275a4b5(%rip), %r8    # 0x1429a0a92
1402465dd: 48 8d 8d c0 03 00 00        	leaq	0x3c0(%rbp), %rcx
1402465e4: 48 8d 95 c0 01 00 00        	leaq	0x1c0(%rbp), %rdx
1402465eb: 41 b9 11 00 00 00           	movl	$0x11, %r9d
1402465f1: e8 5a 7f 2b 00              	callq	0x1404fe550 <.text+0x4fd550>
1402465f6: 48 8d 05 b8 a4 75 02        	leaq	0x275a4b8(%rip), %rax   # 0x1429a0ab5
1402465fd: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140246602: 48 c7 44 24 28 0a 00 00 00  	movq	$0xa, 0x28(%rsp)
14024660b: 4c 8d 05 91 a4 75 02        	leaq	0x275a491(%rip), %r8    # 0x1429a0aa3
140246612: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
140246619: 48 8d 95 c0 03 00 00        	leaq	0x3c0(%rbp), %rdx
140246620: 41 b9 12 00 00 00           	movl	$0x12, %r9d
140246626: e8 e5 7b 0e 00              	callq	0x14032e210 <.text+0x32d210>
14024662b: 48 83 bd b0 03 00 00 00     	cmpq	$0x0, 0x3b0(%rbp)
140246633: 74 25                       	je	0x14024665a <.text+0x24565a>
140246635: 48 89 74 24 20              	movq	%rsi, 0x20(%rsp)
14024663a: 48 8d 15 7e a4 75 02        	leaq	0x275a47e(%rip), %rdx   # 0x1429a0abf
140246641: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
140246648: 4c 8d 8d 05 04 00 00        	leaq	0x405(%rbp), %r9
14024664f: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140246655: e8 b6 d0 2b 00              	callq	0x140503710 <.text+0x502710>
14024665a: 48 8b 95 a0 03 00 00        	movq	0x3a0(%rbp), %rdx
140246661: 48 85 d2                    	testq	%rdx, %rdx
140246664: 0f 84 8a 06 00 00           	je	0x140246cf4 <.text+0x245cf4>
14024666a: 48 8b 8d a8 03 00 00        	movq	0x3a8(%rbp), %rcx
140246671: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140246677: e8 a4 d5 3c 00              	callq	0x140613c20 <.text+0x612c20>
14024667c: e9 73 06 00 00              	jmp	0x140246cf4 <.text+0x245cf4>
140246681: 4c 8b bd 28 05 00 00        	movq	0x528(%rbp), %r15
140246688: 4d 8d 75 08                 	leaq	0x8(%r13), %r14
14024668c: 4c 8d 05 b7 a3 75 02        	leaq	0x275a3b7(%rip), %r8    # 0x1429a0a4a
140246693: 48 8d 8d a0 03 00 00        	leaq	0x3a0(%rbp), %rcx
14024669a: 48 8d 95 58 03 00 00        	leaq	0x358(%rbp), %rdx
1402466a1: 41 b9 08 00 00 00           	movl	$0x8, %r9d
1402466a7: e8 94 7c 2b 00              	callq	0x1404fe340 <.text+0x4fd340>
1402466ac: 41 0f b6 06                 	movzbl	(%r14), %eax
1402466b0: 48 8d 0d 01 b0 75 02        	leaq	0x275b001(%rip), %rcx   # 0x1429a16b8
1402466b7: 48 63 04 81                 	movslq	(%rcx,%rax,4), %rax
1402466bb: 48 01 c8                    	addq	%rcx, %rax
1402466be: ff e0                       	jmpq	*%rax
1402466c0: 4c 89 b5 98 03 00 00        	movq	%r14, 0x398(%rbp)
1402466c7: 48 8b b5 18 05 00 00        	movq	0x518(%rbp), %rsi
1402466ce: 48 8b bd a8 03 00 00        	movq	0x3a8(%rbp), %rdi
1402466d5: 48 c7 85 c0 03 00 00 00 00 00 00    	movq	$0x0, 0x3c0(%rbp)
1402466e0: 48 8d 8d c0 03 00 00        	leaq	0x3c0(%rbp), %rcx
1402466e7: e8 e4 1e e4 ff              	callq	0x1400885d0 <.text+0x875d0>
1402466ec: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
1402466f3: 48 b8 3b 81 28 ff 69 31 22 41       	movabsq	$0x41223169ff28813b, %rax # imm = 0x41223169FF28813B
1402466fd: 48 89 85 c8 01 00 00        	movq	%rax, 0x1c8(%rbp)
140246704: 48 b8 d9 68 a9 a2 68 72 9b a7       	movabsq	$-0x58648d975d569727, %rax # imm = 0xA79B7268A2A968D9
14024670e: 48 89 85 d0 01 00 00        	movq	%rax, 0x1d0(%rbp)
140246715: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
14024671c: e8 ff 77 37 00              	callq	0x1405bdf20 <.text+0x5bcf20>
140246721: 48 89 bd c0 01 00 00        	movq	%rdi, 0x1c0(%rbp)
140246728: 48 89 85 c8 01 00 00        	movq	%rax, 0x1c8(%rbp)
14024672f: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
140246736: e8 d5 71 37 00              	callq	0x1405bd910 <.text+0x5bc910>
14024673b: 48 8d bd c0 01 00 00        	leaq	0x1c0(%rbp), %rdi
140246742: 48 8d 95 a0 03 00 00        	leaq	0x3a0(%rbp), %rdx
140246749: 48 89 f9                    	movq	%rdi, %rcx
14024674c: 49 89 c0                    	movq	%rax, %r8
14024674f: e8 5c fa 43 00              	callq	0x1406861b0 <.text+0x6851b0>
140246754: 4c 8d 05 85 a3 75 02        	leaq	0x275a385(%rip), %r8    # 0x1429a0ae0
14024675b: 48 8d 8d c0 03 00 00        	leaq	0x3c0(%rbp), %rcx
140246762: 41 b9 0f 00 00 00           	movl	$0xf, %r9d
140246768: 48 89 fa                    	movq	%rdi, %rdx
14024676b: e8 70 57 44 00              	callq	0x14068bee0 <.text+0x68aee0>
140246770: 41 ff 45 0c                 	incl	0xc(%r13)
140246774: 48 8b 06                    	movq	(%rsi), %rax
140246777: 4c 8b 5e 08                 	movq	0x8(%rsi), %r11
14024677b: 48 8b 08                    	movq	(%rax), %rcx
14024677e: 45 31 d2                    	xorl	%r10d, %r10d
140246781: 48 89 ca                    	movq	%rcx, %rdx
140246784: 49 2b 53 18                 	subq	0x18(%r11), %rdx
140246788: 49 0f 42 d2                 	cmovbq	%r10, %rdx
14024678c: 49 2b 4b 38                 	subq	0x38(%r11), %rcx
140246790: 49 0f 42 ca                 	cmovbq	%r10, %rcx
140246794: 4d 8b 4b 10                 	movq	0x10(%r11), %r9
140246798: 4d 8b 43 30                 	movq	0x30(%r11), %r8
14024679c: 4c 89 c6                    	movq	%r8, %rsi
14024679f: 48 29 ce                    	subq	%rcx, %rsi
1402467a2: 49 0f 42 f2                 	cmovbq	%r10, %rsi
1402467a6: 41 0f 96 c5                 	setbe	%r13b
1402467aa: 4c 89 cf                    	movq	%r9, %rdi
1402467ad: 48 29 d7                    	subq	%rdx, %rdi
1402467b0: 49 0f 42 fa                 	cmovbq	%r10, %rdi
1402467b4: 41 0f 96 c7                 	setbe	%r15b
1402467b8: 4d 8b 73 40                 	movq	0x40(%r11), %r14
1402467bc: 4d 89 f2                    	movq	%r14, %r10
1402467bf: 49 29 f2                    	subq	%rsi, %r10
1402467c2: 49 29 fa                    	subq	%rdi, %r10
1402467c5: 4d 8b 63 08                 	movq	0x8(%r11), %r12
1402467c9: 4d 8b 5b 28                 	movq	0x28(%r11), %r11
1402467cd: 4c 89 10                    	movq	%r10, (%rax)
1402467d0: 45 84 ef                    	testb	%r13b, %r15b
1402467d3: 4c 8b bd 10 05 00 00        	movq	0x510(%rbp), %r15
1402467da: 75 6d                       	jne	0x140246849 <.text+0x245849>
1402467dc: 4c 6b d1 58                 	imulq	$0x58, %rcx, %r10
1402467e0: 4d 01 d3                    	addq	%r10, %r11
1402467e3: 4c 6b d6 58                 	imulq	$0x58, %rsi, %r10
1402467e7: 48 6b f2 58                 	imulq	$0x58, %rdx, %rsi
1402467eb: 49 01 f4                    	addq	%rsi, %r12
1402467ee: 48 6b f7 58                 	imulq	$0x58, %rdi, %rsi
1402467f2: 31 ff                       	xorl	%edi, %edi
1402467f4: 49 39 d1                    	cmpq	%rdx, %r9
1402467f7: ba 08 00 00 00              	movl	$0x8, %edx
1402467fc: 4c 0f 42 e2                 	cmovbq	%rdx, %r12
140246800: 4d 8d 4c 34 a8              	leaq	-0x58(%r12,%rsi), %r9
140246805: 49 0f 47 f9                 	cmovaq	%r9, %rdi
140246809: 49 39 c8                    	cmpq	%rcx, %r8
14024680c: 4c 0f 42 da                 	cmovbq	%rdx, %r11
140246810: 4b 8d 4c 13 a8              	leaq	-0x58(%r11,%r10), %rcx
140246815: 48 0f 46 cf                 	cmovbeq	%rdi, %rcx
140246819: 4c 89 30                    	movq	%r14, (%rax)
14024681c: 83 79 48 73                 	cmpl	$0x73, 0x48(%rcx)
140246820: 74 27                       	je	0x140246849 <.text+0x245849>
140246822: 80 79 54 00                 	cmpb	$0x0, 0x54(%rcx)
140246826: 75 21                       	jne	0x140246849 <.text+0x245849>
140246828: 41 81 7f 10 c3 00 00 00     	cmpl	$0xc3, 0x10(%r15)
140246830: 75 17                       	jne	0x140246849 <.text+0x245849>
140246832: 41 83 7f 0c 02              	cmpl	$0x2, 0xc(%r15)
140246837: 72 10                       	jb	0x140246849 <.text+0x245849>
140246839: 48 83 c1 48                 	addq	$0x48, %rcx
14024683d: 48 8b 01                    	movq	(%rcx), %rax
140246840: 8b 49 08                    	movl	0x8(%rcx), %ecx
140246843: 89 4b 08                    	movl	%ecx, 0x8(%rbx)
140246846: 48 89 03                    	movq	%rax, (%rbx)
140246849: 49 83 c7 09                 	addq	$0x9, %r15
14024684d: 4c 89 bd c0 01 00 00        	movq	%r15, 0x1c0(%rbp)
140246854: 48 8d b5 60 01 00 00        	leaq	0x160(%rbp), %rsi
14024685b: 48 89 b5 c8 01 00 00        	movq	%rsi, 0x1c8(%rbp)
140246862: 48 89 9d d0 01 00 00        	movq	%rbx, 0x1d0(%rbp)
140246869: 48 8d 15 7f a2 75 02        	leaq	0x275a27f(%rip), %rdx   # 0x1429a0aef
140246870: 48 8d bd c0 03 00 00        	leaq	0x3c0(%rbp), %rdi
140246877: 4c 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %r9
14024687e: 41 b8 06 00 00 00           	movl	$0x6, %r8d
140246884: 48 89 f9                    	movq	%rdi, %rcx
140246887: e8 44 81 2b 00              	callq	0x1404fe9d0 <.text+0x4fd9d0>
14024688c: c6 85 07 04 00 00 00        	movb	$0x0, 0x407(%rbp)
140246893: 48 8d 85 80 01 00 00        	leaq	0x180(%rbp), %rax
14024689a: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
1402468a1: 48 8d 85 07 04 00 00        	leaq	0x407(%rbp), %rax
1402468a8: 48 89 85 c8 01 00 00        	movq	%rax, 0x1c8(%rbp)
1402468af: 48 8b 85 28 05 00 00        	movq	0x528(%rbp), %rax
1402468b6: 48 89 85 d0 01 00 00        	movq	%rax, 0x1d0(%rbp)
1402468bd: 48 89 b5 d8 01 00 00        	movq	%rsi, 0x1d8(%rbp)
1402468c4: 4c 89 bd e0 01 00 00        	movq	%r15, 0x1e0(%rbp)
1402468cb: 48 8d 15 23 a2 75 02        	leaq	0x275a223(%rip), %rdx   # 0x1429a0af5
1402468d2: 4c 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %r9
1402468d9: 41 b8 06 00 00 00           	movl	$0x6, %r8d
1402468df: 48 89 f9                    	movq	%rdi, %rcx
1402468e2: e8 d9 8c 2b 00              	callq	0x1404ff5c0 <.text+0x4fe5c0>
1402468e7: e9 65 03 00 00              	jmp	0x140246c51 <.text+0x245c51>
1402468ec: 48 8d 0d 65 89 77 02        	leaq	0x2778965(%rip), %rcx   # 0x1429bf258
1402468f3: e8 18 2a 69 02              	callq	0x1428d9310 <.text+0x28d8310>
1402468f8: 0f 0b                       	ud2
1402468fa: 0f 89 9c 03 00 00           	jns	0x140246c9c <.text+0x245c9c>
140246900: 49 8b 45 00                 	movq	(%r13), %rax
140246904: 44 89 f9                    	movl	%r15d, %ecx
140246907: f7 d9                       	negl	%ecx
140246909: 83 e1 03                    	andl	$0x3, %ecx
14024690c: 0f 84 78 04 00 00           	je	0x140246d8a <.text+0x245d8a>
140246912: f7 d9                       	negl	%ecx
140246914: 31 d2                       	xorl	%edx, %edx
140246916: 4c 8d 05 63 ad 75 02        	leaq	0x275ad63(%rip), %r8    # 0x1429a1680
14024691d: 49 8b 04 c0                 	movq	(%r8,%rax,8), %rax
140246921: ff ca                       	decl	%edx
140246923: 39 d1                       	cmpl	%edx, %ecx
140246925: 75 f6                       	jne	0x14024691d <.text+0x24591d>
140246927: 49 89 45 00                 	movq	%rax, (%r13)
14024692b: 44 89 f9                    	movl	%r15d, %ecx
14024692e: 29 d1                       	subl	%edx, %ecx
140246930: e9 58 04 00 00              	jmp	0x140246d8d <.text+0x245d8d>
140246935: 48 8b b5 a8 03 00 00        	movq	0x3a8(%rbp), %rsi
14024693c: 48 c7 85 c0 03 00 00 00 00 00 00    	movq	$0x0, 0x3c0(%rbp)
140246947: 48 8d 8d c0 03 00 00        	leaq	0x3c0(%rbp), %rcx
14024694e: e8 7d 1c e4 ff              	callq	0x1400885d0 <.text+0x875d0>
140246953: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
14024695a: 48 b8 3b 81 28 ff 69 31 22 41       	movabsq	$0x41223169ff28813b, %rax # imm = 0x41223169FF28813B
140246964: 48 89 85 c8 01 00 00        	movq	%rax, 0x1c8(%rbp)
14024696b: 48 b8 d9 68 a9 a2 68 72 9b a7       	movabsq	$-0x58648d975d569727, %rax # imm = 0xA79B7268A2A968D9
140246975: 48 89 85 d0 01 00 00        	movq	%rax, 0x1d0(%rbp)
14024697c: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
140246983: e8 98 75 37 00              	callq	0x1405bdf20 <.text+0x5bcf20>
140246988: 48 89 b5 c0 01 00 00        	movq	%rsi, 0x1c0(%rbp)
14024698f: 48 89 85 c8 01 00 00        	movq	%rax, 0x1c8(%rbp)
140246996: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
14024699d: e8 6e 6f 37 00              	callq	0x1405bd910 <.text+0x5bc910>
1402469a2: 48 8d 9d c0 03 00 00        	leaq	0x3c0(%rbp), %rbx
1402469a9: 48 8d 95 a0 03 00 00        	leaq	0x3a0(%rbp), %rdx
1402469b0: 48 89 d9                    	movq	%rbx, %rcx
1402469b3: 49 89 c0                    	movq	%rax, %r8
1402469b6: e8 f5 f7 43 00              	callq	0x1406861b0 <.text+0x6851b0>
1402469bb: 4c 8d 05 1e a1 75 02        	leaq	0x275a11e(%rip), %r8    # 0x1429a0ae0
1402469c2: 48 8d bd c0 01 00 00        	leaq	0x1c0(%rbp), %rdi
1402469c9: 41 b9 0f 00 00 00           	movl	$0xf, %r9d
1402469cf: 48 89 f9                    	movq	%rdi, %rcx
1402469d2: 48 89 da                    	movq	%rbx, %rdx
1402469d5: e8 06 55 44 00              	callq	0x14068bee0 <.text+0x68aee0>
1402469da: 48 8d 15 0e a1 75 02        	leaq	0x275a10e(%rip), %rdx   # 0x1429a0aef
1402469e1: 41 b8 06 00 00 00           	movl	$0x6, %r8d
1402469e7: 48 89 f9                    	movq	%rdi, %rcx
1402469ea: e8 51 90 2b 00              	callq	0x1404ffa40 <.text+0x4fea40>
1402469ef: 4c 89 b5 c0 03 00 00        	movq	%r14, 0x3c0(%rbp)
1402469f6: 4c 89 bd c8 03 00 00        	movq	%r15, 0x3c8(%rbp)
1402469fd: 48 8d 85 60 01 00 00        	leaq	0x160(%rbp), %rax
140246a04: 48 89 85 d0 03 00 00        	movq	%rax, 0x3d0(%rbp)
140246a0b: 4c 89 ad d8 03 00 00        	movq	%r13, 0x3d8(%rbp)
140246a12: 48 8d 15 dc a0 75 02        	leaq	0x275a0dc(%rip), %rdx   # 0x1429a0af5
140246a19: 4c 8d 8d c0 03 00 00        	leaq	0x3c0(%rbp), %r9
140246a20: 41 b8 06 00 00 00           	movl	$0x6, %r8d
140246a26: 48 89 f9                    	movq	%rdi, %rcx
140246a29: e8 32 ba 2b 00              	callq	0x140502460 <.text+0x501460>
140246a2e: e9 c1 02 00 00              	jmp	0x140246cf4 <.text+0x245cf4>
140246a33: 4c 89 b5 98 03 00 00        	movq	%r14, 0x398(%rbp)
140246a3a: 48 8b 9d 20 05 00 00        	movq	0x520(%rbp), %rbx
140246a41: 48 8b bd a8 03 00 00        	movq	0x3a8(%rbp), %rdi
140246a48: 48 c7 85 c0 03 00 00 00 00 00 00    	movq	$0x0, 0x3c0(%rbp)
140246a53: 48 8d 8d c0 03 00 00        	leaq	0x3c0(%rbp), %rcx
140246a5a: e8 71 1b e4 ff              	callq	0x1400885d0 <.text+0x875d0>
140246a5f: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
140246a66: 48 b8 3b 81 28 ff 69 31 22 41       	movabsq	$0x41223169ff28813b, %rax # imm = 0x41223169FF28813B
140246a70: 48 89 85 c8 01 00 00        	movq	%rax, 0x1c8(%rbp)
140246a77: 48 b8 d9 68 a9 a2 68 72 9b a7       	movabsq	$-0x58648d975d569727, %rax # imm = 0xA79B7268A2A968D9
140246a81: 48 89 85 d0 01 00 00        	movq	%rax, 0x1d0(%rbp)
140246a88: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
140246a8f: e8 8c 74 37 00              	callq	0x1405bdf20 <.text+0x5bcf20>
140246a94: 48 89 bd c0 01 00 00        	movq	%rdi, 0x1c0(%rbp)
140246a9b: 48 89 85 c8 01 00 00        	movq	%rax, 0x1c8(%rbp)
140246aa2: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
140246aa9: e8 62 6e 37 00              	callq	0x1405bd910 <.text+0x5bc910>
140246aae: 48 8d bd c0 01 00 00        	leaq	0x1c0(%rbp), %rdi
140246ab5: 48 8d 95 a0 03 00 00        	leaq	0x3a0(%rbp), %rdx
140246abc: 48 89 f9                    	movq	%rdi, %rcx
140246abf: 49 89 c0                    	movq	%rax, %r8
140246ac2: e8 e9 f6 43 00              	callq	0x1406861b0 <.text+0x6851b0>
140246ac7: 4c 8d 05 12 a0 75 02        	leaq	0x275a012(%rip), %r8    # 0x1429a0ae0
140246ace: 48 8d 8d c0 03 00 00        	leaq	0x3c0(%rbp), %rcx
140246ad5: 41 b9 0f 00 00 00           	movl	$0xf, %r9d
140246adb: 48 89 fa                    	movq	%rdi, %rdx
140246ade: e8 fd 53 44 00              	callq	0x14068bee0 <.text+0x68aee0>
140246ae3: 48 8b 03                    	movq	(%rbx), %rax
140246ae6: 4c 8b 53 08                 	movq	0x8(%rbx), %r10
140246aea: 48 8b 08                    	movq	(%rax), %rcx
140246aed: 31 ff                       	xorl	%edi, %edi
140246aef: 49 89 c8                    	movq	%rcx, %r8
140246af2: 4d 2b 42 18                 	subq	0x18(%r10), %r8
140246af6: 4c 0f 42 c7                 	cmovbq	%rdi, %r8
140246afa: 49 2b 4a 38                 	subq	0x38(%r10), %rcx
140246afe: 48 0f 42 cf                 	cmovbq	%rdi, %rcx
140246b02: 49 8b 52 08                 	movq	0x8(%r10), %rdx
140246b06: 48 89 95 f8 03 00 00        	movq	%rdx, 0x3f8(%rbp)
140246b0d: 4d 8b 4a 30                 	movq	0x30(%r10), %r9
140246b11: 4c 89 ca                    	movq	%r9, %rdx
140246b14: 48 29 ca                    	subq	%rcx, %rdx
140246b17: 48 0f 42 d7                 	cmovbq	%rdi, %rdx
140246b1b: 4d 8b 62 10                 	movq	0x10(%r10), %r12
140246b1f: 41 0f 96 c6                 	setbe	%r14b
140246b23: 4c 89 e3                    	movq	%r12, %rbx
140246b26: 4c 29 c3                    	subq	%r8, %rbx
140246b29: 48 0f 42 df                 	cmovbq	%rdi, %rbx
140246b2d: 49 8b 7a 28                 	movq	0x28(%r10), %rdi
140246b31: 4d 8b 5a 40                 	movq	0x40(%r10), %r11
140246b35: 41 0f 96 c7                 	setbe	%r15b
140246b39: 4d 89 da                    	movq	%r11, %r10
140246b3c: 49 29 d2                    	subq	%rdx, %r10
140246b3f: 49 29 da                    	subq	%rbx, %r10
140246b42: 4c 8b ad 10 05 00 00        	movq	0x510(%rbp), %r13
140246b49: 4c 89 10                    	movq	%r10, (%rax)
140246b4c: 45 84 f7                    	testb	%r14b, %r15b
140246b4f: 75 62                       	jne	0x140246bb3 <.text+0x245bb3>
140246b51: 45 31 d2                    	xorl	%r10d, %r10d
140246b54: 4d 39 c4                    	cmpq	%r8, %r12
140246b57: 4f 8d 04 40                 	leaq	(%r8,%r8,2), %r8
140246b5b: 4c 8b b5 f8 03 00 00        	movq	0x3f8(%rbp), %r14
140246b62: 4f 8d 04 c6                 	leaq	(%r14,%r8,8), %r8
140246b66: 41 be 08 00 00 00           	movl	$0x8, %r14d
140246b6c: 4d 0f 42 c6                 	cmovbq	%r14, %r8
140246b70: 48 8d 1c 5b                 	leaq	(%rbx,%rbx,2), %rbx
140246b74: 4d 8d 44 d8 e8              	leaq	-0x18(%r8,%rbx,8), %r8
140246b79: 4d 0f 47 d0                 	cmovaq	%r8, %r10
140246b7d: 49 39 c9                    	cmpq	%rcx, %r9
140246b80: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140246b84: 48 8d 0c cf                 	leaq	(%rdi,%rcx,8), %rcx
140246b88: 49 0f 42 ce                 	cmovbq	%r14, %rcx
140246b8c: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
140246b90: 48 8d 4c d1 e8              	leaq	-0x18(%rcx,%rdx,8), %rcx
140246b95: 49 0f 46 ca                 	cmovbeq	%r10, %rcx
140246b99: 4c 89 18                    	movq	%r11, (%rax)
140246b9c: 0f b7 41 10                 	movzwl	0x10(%rcx), %eax
140246ba0: 66 85 c0                    	testw	%ax, %ax
140246ba3: 74 0e                       	je	0x140246bb3 <.text+0x245bb3>
140246ba5: 0f b7 49 12                 	movzwl	0x12(%rcx), %ecx
140246ba9: 66 41 89 45 1c              	movw	%ax, 0x1c(%r13)
140246bae: 66 41 89 4d 1e              	movw	%cx, 0x1e(%r13)
140246bb3: 49 83 c5 09                 	addq	$0x9, %r13
140246bb7: 4c 89 ad c0 01 00 00        	movq	%r13, 0x1c0(%rbp)
140246bbe: 48 8d 9d 60 01 00 00        	leaq	0x160(%rbp), %rbx
140246bc5: 48 89 9d c8 01 00 00        	movq	%rbx, 0x1c8(%rbp)
140246bcc: 48 89 b5 d0 01 00 00        	movq	%rsi, 0x1d0(%rbp)
140246bd3: 48 8d 15 15 9f 75 02        	leaq	0x2759f15(%rip), %rdx   # 0x1429a0aef
140246bda: 48 8d bd c0 03 00 00        	leaq	0x3c0(%rbp), %rdi
140246be1: 4c 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %r9
140246be8: 41 b8 06 00 00 00           	movl	$0x6, %r8d
140246bee: 48 89 f9                    	movq	%rdi, %rcx
140246bf1: e8 1a 82 2b 00              	callq	0x1404fee10 <.text+0x4fde10>
140246bf6: c6 85 07 04 00 00 00        	movb	$0x0, 0x407(%rbp)
140246bfd: 48 8d 85 30 03 00 00        	leaq	0x330(%rbp), %rax
140246c04: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
140246c0b: 48 8d 85 07 04 00 00        	leaq	0x407(%rbp), %rax
140246c12: 48 89 85 c8 01 00 00        	movq	%rax, 0x1c8(%rbp)
140246c19: 48 8b 85 28 05 00 00        	movq	0x528(%rbp), %rax
140246c20: 48 89 85 d0 01 00 00        	movq	%rax, 0x1d0(%rbp)
140246c27: 48 89 9d d8 01 00 00        	movq	%rbx, 0x1d8(%rbp)
140246c2e: 4c 89 ad e0 01 00 00        	movq	%r13, 0x1e0(%rbp)
140246c35: 48 8d 15 b9 9e 75 02        	leaq	0x2759eb9(%rip), %rdx   # 0x1429a0af5
140246c3c: 4c 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %r9
140246c43: 41 b8 06 00 00 00           	movl	$0x6, %r8d
140246c49: 48 89 f9                    	movq	%rdi, %rcx
140246c4c: e8 2f ba 2b 00              	callq	0x140502680 <.text+0x501680>
140246c51: 80 bd 07 04 00 00 00        	cmpb	$0x0, 0x407(%rbp)
140246c58: 0f 84 96 00 00 00           	je	0x140246cf4 <.text+0x245cf4>
140246c5e: 48 8b 85 98 03 00 00        	movq	0x398(%rbp), %rax
140246c65: c6 00 03                    	movb	$0x3, (%rax)
140246c68: e9 87 00 00 00              	jmp	0x140246cf4 <.text+0x245cf4>
140246c6d: 44 89 f9                    	movl	%r15d, %ecx
140246c70: 41 83 ff 04                 	cmpl	$0x4, %r15d
140246c74: 72 26                       	jb	0x140246c9c <.text+0x245c9c>
140246c76: 83 c1 04                    	addl	$0x4, %ecx
140246c79: 48 8d 15 c8 a9 75 02        	leaq	0x275a9c8(%rip), %rdx   # 0x1429a1648
140246c80: 48 8b 04 c2                 	movq	(%rdx,%rax,8), %rax
140246c84: 48 8b 04 c2                 	movq	(%rdx,%rax,8), %rax
140246c88: 48 8b 04 c2                 	movq	(%rdx,%rax,8), %rax
140246c8c: 48 8b 04 c2                 	movq	(%rdx,%rax,8), %rax
140246c90: 83 c1 fc                    	addl	$-0x4, %ecx
140246c93: 83 f9 04                    	cmpl	$0x4, %ecx
140246c96: 7f e8                       	jg	0x140246c80 <.text+0x245c80>
140246c98: 49 89 45 00                 	movq	%rax, (%r13)
140246c9c: 48 8d 15 e1 9d 75 02        	leaq	0x2759de1(%rip), %rdx   # 0x1429a0a84
140246ca3: 48 8d 9d 58 03 00 00        	leaq	0x358(%rbp), %rbx
140246caa: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
140246cb0: 48 89 d9                    	movq	%rbx, %rcx
140246cb3: 4d 89 e9                    	movq	%r13, %r9
140246cb6: e8 75 83 2b 00              	callq	0x1404ff030 <.text+0x4fe030>
140246cbb: 4c 89 6c 24 20              	movq	%r13, 0x20(%rsp)
140246cc0: 48 8d 15 83 9d 75 02        	leaq	0x2759d83(%rip), %rdx   # 0x1429a0a4a
140246cc7: 4c 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %r9
140246cce: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140246cd4: 48 89 d9                    	movq	%rbx, %rcx
140246cd7: e8 34 94 2b 00              	callq	0x140500110 <.text+0x4ff110>
140246cdc: 48 8d 15 ab 9d 75 02        	leaq	0x2759dab(%rip), %rdx   # 0x1429a0a8e
140246ce3: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140246ce9: 48 89 d9                    	movq	%rbx, %rcx
140246cec: 49 89 f9                    	movq	%rdi, %r9
140246cef: e8 ac c2 2b 00              	callq	0x140502fa0 <.text+0x501fa0>
140246cf4: 0f 28 b5 10 04 00 00        	movaps	0x410(%rbp), %xmm6
140246cfb: 0f 28 bd 20 04 00 00        	movaps	0x420(%rbp), %xmm7
140246d02: 44 0f 28 85 30 04 00 00     	movaps	0x430(%rbp), %xmm8
140246d0a: 44 0f 28 8d 40 04 00 00     	movaps	0x440(%rbp), %xmm9
140246d12: 44 0f 28 95 50 04 00 00     	movaps	0x450(%rbp), %xmm10
140246d1a: 44 0f 28 9d 60 04 00 00     	movaps	0x460(%rbp), %xmm11
140246d22: 48 81 c4 f8 04 00 00        	addq	$0x4f8, %rsp            # imm = 0x4F8
140246d29: 5b                          	popq	%rbx
140246d2a: 5f                          	popq	%rdi
140246d2b: 5e                          	popq	%rsi
140246d2c: 41 5c                       	popq	%r12
140246d2e: 41 5d                       	popq	%r13
140246d30: 41 5e                       	popq	%r14
140246d32: 41 5f                       	popq	%r15
140246d34: 5d                          	popq	%rbp
140246d35: c3                          	retq
140246d36: 0f 2e 05 33 9a 75 02        	ucomiss	0x2759a33(%rip), %xmm0  # 0x1429a0770
140246d3d: 72 1e                       	jb	0x140246d5d <.text+0x245d5d>
140246d3f: f3 0f 10 0d 2d 9a 75 02     	movss	0x2759a2d(%rip), %xmm1  # 0x1429a0774
140246d47: 0f 2e c8                    	ucomiss	%xmm0, %xmm1
140246d4a: 72 11                       	jb	0x140246d5d <.text+0x245d5d>
140246d4c: 48 8d 0d 72 9c 75 02        	leaq	0x2759c72(%rip), %rcx   # 0x1429a09c5
140246d53: b8 04 00 00 00              	movl	$0x4, %eax
140246d58: e9 86 f0 ff ff              	jmp	0x140245de3 <.text+0x244de3>
140246d5d: 80 fa 03                    	cmpb	$0x3, %dl
140246d60: 0f 84 7d f0 ff ff           	je	0x140245de3 <.text+0x244de3>
140246d66: 31 c0                       	xorl	%eax, %eax
140246d68: 80 fa 04                    	cmpb	$0x4, %dl
140246d6b: 0f 94 c0                    	sete	%al
140246d6e: 48 8d 44 80 05              	leaq	0x5(%rax,%rax,4), %rax
140246d73: 48 8d 15 3b 9c 75 02        	leaq	0x2759c3b(%rip), %rdx   # 0x1429a09b5
140246d7a: 48 8d 0d 2f 9c 75 02        	leaq	0x2759c2f(%rip), %rcx   # 0x1429a09b0
140246d81: 48 0f 44 ca                 	cmoveq	%rdx, %rcx
140246d85: e9 59 f0 ff ff              	jmp	0x140245de3 <.text+0x244de3>
140246d8a: 44 89 f9                    	movl	%r15d, %ecx
140246d8d: 41 83 ff fc                 	cmpl	$-0x4, %r15d
140246d91: 0f 87 05 ff ff ff           	ja	0x140246c9c <.text+0x245c9c>
140246d97: 48 8d 15 e2 a8 75 02        	leaq	0x275a8e2(%rip), %rdx   # 0x1429a1680
140246d9e: 48 8b 04 c2                 	movq	(%rdx,%rax,8), %rax
140246da2: 48 8b 04 c2                 	movq	(%rdx,%rax,8), %rax
140246da6: 48 8b 04 c2                 	movq	(%rdx,%rax,8), %rax
140246daa: 48 8b 04 c2                 	movq	(%rdx,%rax,8), %rax
140246dae: 83 c1 04                    	addl	$0x4, %ecx
140246db1: 75 eb                       	jne	0x140246d9e <.text+0x245d9e>
140246db3: e9 e0 fe ff ff              	jmp	0x140246c98 <.text+0x245c98>
140246db8: 48 8d 0d 09 9d 75 02        	leaq	0x2759d09(%rip), %rcx   # 0x1429a0ac8
140246dbf: e8 4c 25 69 02              	callq	0x1428d9310 <.text+0x28d8310>
140246dc4: cc                          	int3
