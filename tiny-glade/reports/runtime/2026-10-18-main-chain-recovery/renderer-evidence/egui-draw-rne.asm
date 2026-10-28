
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1400ddc80: 55                          	pushq	%rbp
1400ddc81: 41 57                       	pushq	%r15
1400ddc83: 41 56                       	pushq	%r14
1400ddc85: 41 55                       	pushq	%r13
1400ddc87: 41 54                       	pushq	%r12
1400ddc89: 56                          	pushq	%rsi
1400ddc8a: 57                          	pushq	%rdi
1400ddc8b: 53                          	pushq	%rbx
1400ddc8c: 48 81 ec 68 0b 00 00        	subq	$0xb68, %rsp            # imm = 0xB68
1400ddc93: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1400ddc9b: 44 0f 29 9d d0 0a 00 00     	movaps	%xmm11, 0xad0(%rbp)
1400ddca3: 44 0f 29 95 c0 0a 00 00     	movaps	%xmm10, 0xac0(%rbp)
1400ddcab: 44 0f 29 8d b0 0a 00 00     	movaps	%xmm9, 0xab0(%rbp)
1400ddcb3: 44 0f 29 85 a0 0a 00 00     	movaps	%xmm8, 0xaa0(%rbp)
1400ddcbb: 0f 29 bd 90 0a 00 00        	movaps	%xmm7, 0xa90(%rbp)
1400ddcc2: 66 0f 7f b5 80 0a 00 00     	movdqa	%xmm6, 0xa80(%rbp)
1400ddcca: 48 c7 85 78 0a 00 00 fe ff ff ff    	movq	$-0x2, 0xa78(%rbp)
1400ddcd5: 4c 89 cb                    	movq	%r9, %rbx
1400ddcd8: 4c 89 c6                    	movq	%r8, %rsi
1400ddcdb: 48 89 d7                    	movq	%rdx, %rdi
1400ddcde: 48 89 c8                    	movq	%rcx, %rax
1400ddce1: 48 8b 8a 90 01 00 00        	movq	0x190(%rdx), %rcx
1400ddce8: 48 89 8d f8 09 00 00        	movq	%rcx, 0x9f8(%rbp)
1400ddcef: 48 8d 48 48                 	leaq	0x48(%rax), %rcx
1400ddcf3: 48 89 8d 40 09 00 00        	movq	%rcx, 0x940(%rbp)
1400ddcfa: 48 8b 48 50                 	movq	0x50(%rax), %rcx
1400ddcfe: 48 8b 50 58                 	movq	0x58(%rax), %rdx
1400ddd02: 48 89 95 68 0a 00 00        	movq	%rdx, 0xa68(%rbp)
1400ddd09: 48 89 85 50 0a 00 00        	movq	%rax, 0xa50(%rbp)
1400ddd10: 48 c7 40 58 00 00 00 00     	movq	$0x0, 0x58(%rax)
1400ddd18: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1400ddd20: 48 8b 85 68 0a 00 00        	movq	0xa68(%rbp), %rax
1400ddd27: 48 83 e8 01                 	subq	$0x1, %rax
1400ddd2b: 72 1c                       	jb	0x1400ddd49 <.text+0xdcd49>
1400ddd2d: 48 89 85 68 0a 00 00        	movq	%rax, 0xa68(%rbp)
1400ddd34: 4c 8d 71 28                 	leaq	0x28(%rcx), %r14
1400ddd38: 4c 89 b5 58 0a 00 00        	movq	%r14, 0xa58(%rbp)
1400ddd3f: e8 6c 47 b0 00              	callq	0x140be24b0 <.text+0xbe14b0>
1400ddd44: 4c 89 f1                    	movq	%r14, %rcx
1400ddd47: eb d7                       	jmp	0x1400ddd20 <.text+0xdcd20>
1400ddd49: 48 89 9d b8 07 00 00        	movq	%rbx, 0x7b8(%rbp)
1400ddd50: 48 89 bd b0 07 00 00        	movq	%rdi, 0x7b0(%rbp)
1400ddd57: 48 89 b5 a8 07 00 00        	movq	%rsi, 0x7a8(%rbp)
1400ddd5e: 4c 8b 8d 50 0a 00 00        	movq	0xa50(%rbp), %r9
1400ddd65: 49 8d 51 18                 	leaq	0x18(%r9), %rdx
1400ddd69: 49 8b 41 20                 	movq	0x20(%r9), %rax
1400ddd6d: 4d 8b 41 28                 	movq	0x28(%r9), %r8
1400ddd71: 49 c7 41 28 00 00 00 00     	movq	$0x0, 0x28(%r9)
1400ddd79: 49 6b c8 58                 	imulq	$0x58, %r8, %rcx
1400ddd7d: 48 01 c1                    	addq	%rax, %rcx
1400ddd80: 4c 89 85 b8 09 00 00        	movq	%r8, 0x9b8(%rbp)
1400ddd87: 48 c7 85 c0 09 00 00 00 00 00 00    	movq	$0x0, 0x9c0(%rbp)
1400ddd92: 48 89 85 a0 09 00 00        	movq	%rax, 0x9a0(%rbp)
1400ddd99: 48 89 8d a8 09 00 00        	movq	%rcx, 0x9a8(%rbp)
1400ddda0: 48 89 95 b0 09 00 00        	movq	%rdx, 0x9b0(%rbp)
1400ddda7: 49 8d 51 78                 	leaq	0x78(%r9), %rdx
1400dddab: 48 89 95 48 09 00 00        	movq	%rdx, 0x948(%rbp)
1400dddb2: 49 8d 91 98 00 00 00        	leaq	0x98(%r9), %rdx
1400dddb9: 48 89 95 f0 09 00 00        	movq	%rdx, 0x9f0(%rbp)
1400dddc0: 4c 8d 3d 49 d3 85 02        	leaq	0x285d349(%rip), %r15   # 0x14293b110
1400dddc7: 66 0f 76 f6                 	pcmpeqd	%xmm6, %xmm6
1400dddcb: f3 0f 10 3d 9d ce 85 02     	movss	0x285ce9d(%rip), %xmm7  # 0x14293ac70
1400dddd3: f3 44 0f 10 05 94 7a 84 02  	movss	0x2847a94(%rip), %xmm8  # 0x142925870
1400ddddc: 44 0f 28 0d 6c ce 85 02     	movaps	0x285ce6c(%rip), %xmm9  # 0x14293ac50
1400ddde4: 44 0f 28 15 74 ce 85 02     	movaps	0x285ce74(%rip), %xmm10 # 0x14293ac60
1400dddec: 66 44 0f 6f 1d 8b ce 85 02  	movdqa	0x285ce8b(%rip), %xmm11 # 0x14293ac80
1400dddf5: eb 17                       	jmp	0x1400dde0e <.text+0xdce0e>
1400dddf7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1400dde00: 48 8b 85 a0 09 00 00        	movq	0x9a0(%rbp), %rax
1400dde07: 48 8b 8d a8 09 00 00        	movq	0x9a8(%rbp), %rcx
1400dde0e: 48 39 c8                    	cmpq	%rcx, %rax
1400dde11: 0f 84 5d 0a 00 00           	je	0x1400de874 <.text+0xdd874>
1400dde17: 48 8d 48 58                 	leaq	0x58(%rax), %rcx
1400dde1b: 48 89 8d a0 09 00 00        	movq	%rcx, 0x9a0(%rbp)
1400dde22: 4c 8b 28                    	movq	(%rax), %r13
1400dde25: 48 8b 48 08                 	movq	0x8(%rax), %rcx
1400dde29: 48 89 8d e0 09 00 00        	movq	%rcx, 0x9e0(%rbp)
1400dde30: 48 8b 48 50                 	movq	0x50(%rax), %rcx
1400dde34: 48 89 8d 90 09 00 00        	movq	%rcx, 0x990(%rbp)
1400dde3b: 0f 10 40 40                 	movups	0x40(%rax), %xmm0
1400dde3f: 0f 29 85 80 09 00 00        	movaps	%xmm0, 0x980(%rbp)
1400dde46: 0f 10 40 30                 	movups	0x30(%rax), %xmm0
1400dde4a: 0f 29 85 70 09 00 00        	movaps	%xmm0, 0x970(%rbp)
1400dde51: 0f 10 40 20                 	movups	0x20(%rax), %xmm0
1400dde55: 0f 29 85 60 09 00 00        	movaps	%xmm0, 0x960(%rbp)
1400dde5c: 0f 10 40 10                 	movups	0x10(%rax), %xmm0
1400dde60: 0f 29 85 50 09 00 00        	movaps	%xmm0, 0x950(%rbp)
1400dde67: 49 83 fd 02                 	cmpq	$0x2, %r13
1400dde6b: 0f 84 03 0a 00 00           	je	0x1400de874 <.text+0xdd874>
1400dde71: 48 8b 85 90 09 00 00        	movq	0x990(%rbp), %rax
1400dde78: 48 89 85 00 08 00 00        	movq	%rax, 0x800(%rbp)
1400dde7f: 66 0f 6f 85 50 09 00 00     	movdqa	0x950(%rbp), %xmm0
1400dde87: 0f 28 8d 60 09 00 00        	movaps	0x960(%rbp), %xmm1
1400dde8e: 0f 28 95 70 09 00 00        	movaps	0x970(%rbp), %xmm2
1400dde95: 0f 28 9d 80 09 00 00        	movaps	0x980(%rbp), %xmm3
1400dde9c: 0f 29 9d f0 07 00 00        	movaps	%xmm3, 0x7f0(%rbp)
1400ddea3: 0f 29 95 e0 07 00 00        	movaps	%xmm2, 0x7e0(%rbp)
1400ddeaa: 0f 29 8d d0 07 00 00        	movaps	%xmm1, 0x7d0(%rbp)
1400ddeb1: 66 0f 7f 85 c0 07 00 00     	movdqa	%xmm0, 0x7c0(%rbp)
1400ddeb9: e8 42 8b b3 00              	callq	0x140c16a00 <.text+0xc15a00>
1400ddebe: 84 c0                       	testb	%al, %al
1400ddec0: 74 3e                       	je	0x1400ddf00 <.text+0xdcf00>
1400ddec2: b9 36 00 00 00              	movl	$0x36, %ecx
1400ddec7: e8 f4 f0 ff ff              	callq	0x1400dcfc0 <.text+0xdbfc0>
1400ddecc: be 36 00 00 00              	movl	$0x36, %esi
1400dded1: 48 83 f8 01                 	cmpq	$0x1, %rax
1400dded5: 75 39                       	jne	0x1400ddf10 <.text+0xdcf10>
1400dded7: 49 89 d1                    	movq	%rdx, %r9
1400ddeda: 48 85 d2                    	testq	%rdx, %rdx
1400ddedd: 74 40                       	je	0x1400ddf1f <.text+0xdcf1f>
1400ddedf: 49 83 f9 36                 	cmpq	$0x36, %r9
1400ddee3: 73 34                       	jae	0x1400ddf19 <.text+0xdcf19>
1400ddee5: 48 8d 05 0f cf 85 02        	leaq	0x285cf0f(%rip), %rax   # 0x14293adfb
1400ddeec: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
1400ddef1: 7f 2c                       	jg	0x1400ddf1f <.text+0xdcf1f>
1400ddef3: e9 a1 17 00 00              	jmp	0x1400df699 <.text+0xde699>
1400ddef8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1400ddf00: 45 31 c0                    	xorl	%r8d, %r8d
1400ddf03: e9 c3 01 00 00              	jmp	0x1400de0cb <.text+0xdd0cb>
1400ddf08: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1400ddf10: 4c 8d 25 e4 ce 85 02        	leaq	0x285cee4(%rip), %r12   # 0x14293adfb
1400ddf17: eb 55                       	jmp	0x1400ddf6e <.text+0xdcf6e>
1400ddf19: 0f 85 7a 17 00 00           	jne	0x1400df699 <.text+0xde699>
1400ddf1f: 4c 89 c9                    	movq	%r9, %rcx
1400ddf22: e8 99 f0 ff ff              	callq	0x1400dcfc0 <.text+0xdbfc0>
1400ddf27: 4c 8d 25 cd ce 85 02        	leaq	0x285cecd(%rip), %r12   # 0x14293adfb
1400ddf2e: 48 83 f8 01                 	cmpq	$0x1, %rax
1400ddf32: 75 3a                       	jne	0x1400ddf6e <.text+0xdcf6e>
1400ddf34: 49 89 d4                    	movq	%rdx, %r12
1400ddf37: 49 83 c4 02                 	addq	$0x2, %r12
1400ddf3b: 74 1f                       	je	0x1400ddf5c <.text+0xdcf5c>
1400ddf3d: 49 83 fc 36                 	cmpq	$0x36, %r12
1400ddf41: 73 13                       	jae	0x1400ddf56 <.text+0xdcf56>
1400ddf43: 48 8d 05 b1 ce 85 02        	leaq	0x285ceb1(%rip), %rax   # 0x14293adfb
1400ddf4a: 41 80 3c 04 bf              	cmpb	$-0x41, (%r12,%rax)
1400ddf4f: 7f 0b                       	jg	0x1400ddf5c <.text+0xdcf5c>
1400ddf51: e9 68 17 00 00              	jmp	0x1400df6be <.text+0xde6be>
1400ddf56: 0f 85 62 17 00 00           	jne	0x1400df6be <.text+0xde6be>
1400ddf5c: be 34 00 00 00              	movl	$0x34, %esi
1400ddf61: 48 29 d6                    	subq	%rdx, %rsi
1400ddf64: 48 8d 05 90 ce 85 02        	leaq	0x285ce90(%rip), %rax   # 0x14293adfb
1400ddf6b: 49 01 c4                    	addq	%rax, %r12
1400ddf6e: 4c 8d 35 c0 d1 85 02        	leaq	0x285d1c0(%rip), %r14   # 0x14293b135
1400ddf75: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
1400ddf80: 41 0f be 46 ff              	movsbl	-0x1(%r14), %eax
1400ddf85: 85 c0                       	testl	%eax, %eax
1400ddf87: 78 17                       	js	0x1400ddfa0 <.text+0xdcfa0>
1400ddf89: 49 ff ce                    	decq	%r14
1400ddf8c: 83 f8 5c                    	cmpl	$0x5c, %eax
1400ddf8f: 75 63                       	jne	0x1400ddff4 <.text+0xdcff4>
1400ddf91: eb 7d                       	jmp	0x1400de010 <.text+0xdd010>
1400ddf93: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1400ddfa0: 41 0f b6 4e fe              	movzbl	-0x2(%r14), %ecx
1400ddfa5: 80 f9 c0                    	cmpb	$-0x40, %cl
1400ddfa8: 7d 23                       	jge	0x1400ddfcd <.text+0xdcfcd>
1400ddfaa: 41 0f b6 56 fd              	movzbl	-0x3(%r14), %edx
1400ddfaf: 80 fa c0                    	cmpb	$-0x40, %dl
1400ddfb2: 7d 22                       	jge	0x1400ddfd6 <.text+0xdcfd6>
1400ddfb4: 45 0f b6 46 fc              	movzbl	-0x4(%r14), %r8d
1400ddfb9: 49 83 c6 fc                 	addq	$-0x4, %r14
1400ddfbd: 41 83 e0 07                 	andl	$0x7, %r8d
1400ddfc1: 41 c1 e0 06                 	shll	$0x6, %r8d
1400ddfc5: 83 e2 3f                    	andl	$0x3f, %edx
1400ddfc8: 44 09 c2                    	orl	%r8d, %edx
1400ddfcb: eb 10                       	jmp	0x1400ddfdd <.text+0xdcfdd>
1400ddfcd: 49 83 c6 fe                 	addq	$-0x2, %r14
1400ddfd1: 83 e1 1f                    	andl	$0x1f, %ecx
1400ddfd4: eb 0f                       	jmp	0x1400ddfe5 <.text+0xdcfe5>
1400ddfd6: 49 83 c6 fd                 	addq	$-0x3, %r14
1400ddfda: 83 e2 0f                    	andl	$0xf, %edx
1400ddfdd: c1 e2 06                    	shll	$0x6, %edx
1400ddfe0: 83 e1 3f                    	andl	$0x3f, %ecx
1400ddfe3: 09 d1                       	orl	%edx, %ecx
1400ddfe5: c1 e1 06                    	shll	$0x6, %ecx
1400ddfe8: 24 3f                       	andb	$0x3f, %al
1400ddfea: 0f b6 c0                    	movzbl	%al, %eax
1400ddfed: 09 c8                       	orl	%ecx, %eax
1400ddfef: 83 f8 5c                    	cmpl	$0x5c, %eax
1400ddff2: 74 1c                       	je	0x1400de010 <.text+0xdd010>
1400ddff4: 83 f8 2f                    	cmpl	$0x2f, %eax
1400ddff7: 74 17                       	je	0x1400de010 <.text+0xdd010>
1400ddff9: 4d 39 fe                    	cmpq	%r15, %r14
1400ddffc: 75 82                       	jne	0x1400ddf80 <.text+0xdcf80>
1400ddffe: bb 25 00 00 00              	movl	$0x25, %ebx
1400de003: 4d 89 fe                    	movq	%r15, %r14
1400de006: eb 33                       	jmp	0x1400de03b <.text+0xdd03b>
1400de008: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1400de010: 4d 29 fe                    	subq	%r15, %r14
1400de013: 49 ff c6                    	incq	%r14
1400de016: 74 18                       	je	0x1400de030 <.text+0xdd030>
1400de018: 49 83 fe 25                 	cmpq	$0x25, %r14
1400de01c: 73 0c                       	jae	0x1400de02a <.text+0xdd02a>
1400de01e: 43 80 3c 3e bf              	cmpb	$-0x41, (%r14,%r15)
1400de023: 7f 0b                       	jg	0x1400de030 <.text+0xdd030>
1400de025: e9 44 16 00 00              	jmp	0x1400df66e <.text+0xde66e>
1400de02a: 0f 85 3e 16 00 00           	jne	0x1400df66e <.text+0xde66e>
1400de030: bb 25 00 00 00              	movl	$0x25, %ebx
1400de035: 4c 29 f3                    	subq	%r14, %rbx
1400de038: 4d 01 fe                    	addq	%r15, %r14
1400de03b: e8 f0 a0 b3 00              	callq	0x140c18130 <.text+0xc17130>
1400de040: 48 89 c1                    	movq	%rax, %rcx
1400de043: 48 8b 00                    	movq	(%rax), %rax
1400de046: 48 85 c0                    	testq	%rax, %rax
1400de049: 74 15                       	je	0x1400de060 <.text+0xdd060>
1400de04b: 83 f8 01                    	cmpl	$0x1, %eax
1400de04e: 0f 85 23 15 00 00           	jne	0x1400df577 <.text+0xde577>
1400de054: 48 83 c1 08                 	addq	$0x8, %rcx
1400de058: eb 19                       	jmp	0x1400de073 <.text+0xdd073>
1400de05a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1400de060: 31 d2                       	xorl	%edx, %edx
1400de062: e8 d9 35 7c 02              	callq	0x1428a1640 <.text+0x28a0640>
1400de067: 48 89 c1                    	movq	%rax, %rcx
1400de06a: 48 85 c0                    	testq	%rax, %rax
1400de06d: 0f 84 04 15 00 00           	je	0x1400df577 <.text+0xde577>
1400de073: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1400de077: 0f 85 b2 13 00 00           	jne	0x1400df42f <.text+0xde42f>
1400de07d: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1400de084: 48 89 8d 60 0a 00 00        	movq	%rcx, 0xa60(%rbp)
1400de08b: 48 83 c1 08                 	addq	$0x8, %rcx
1400de08f: 48 89 5c 24 20              	movq	%rbx, 0x20(%rsp)
1400de094: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
1400de09d: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
1400de0a6: 4c 89 e2                    	movq	%r12, %rdx
1400de0a9: 49 89 f0                    	movq	%rsi, %r8
1400de0ac: 4d 89 f1                    	movq	%r14, %r9
1400de0af: e8 8c 8c b3 00              	callq	0x140c16d40 <.text+0xc15d40>
1400de0b4: 48 8b 8d 60 0a 00 00        	movq	0xa60(%rbp), %rcx
1400de0bb: 48 ff 01                    	incq	(%rcx)
1400de0be: 48 89 85 18 09 00 00        	movq	%rax, 0x918(%rbp)
1400de0c5: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1400de0cb: 4c 89 85 10 09 00 00        	movq	%r8, 0x910(%rbp)
1400de0d2: 4c 8b bd e0 07 00 00        	movq	0x7e0(%rbp), %r15
1400de0d9: 49 8d 47 28                 	leaq	0x28(%r15), %rax
1400de0dd: 49 8d 4f 30                 	leaq	0x30(%r15), %rcx
1400de0e1: 31 d2                       	xorl	%edx, %edx
1400de0e3: 48 3b 95 d8 07 00 00        	cmpq	0x7d8(%rbp), %rdx
1400de0ea: 48 8d 95 f0 07 00 00        	leaq	0x7f0(%rbp), %rdx
1400de0f1: 48 0f 41 c2                 	cmovnoq	%rdx, %rax
1400de0f5: 48 8b 38                    	movq	(%rax), %rdi
1400de0f8: 48 8d 85 f8 07 00 00        	leaq	0x7f8(%rbp), %rax
1400de0ff: 48 0f 41 c8                 	cmovnoq	%rax, %rcx
1400de103: 48 8b 19                    	movq	(%rcx), %rbx
1400de106: 4c 89 85 e8 09 00 00        	movq	%r8, 0x9e8(%rbp)
1400de10d: 71 21                       	jno	0x1400de130 <.text+0xdd130>
1400de10f: 4d 8b 67 18                 	movq	0x18(%r15), %r12
1400de113: 4d 8b 77 20                 	movq	0x20(%r15), %r14
1400de117: 45 31 db                    	xorl	%r11d, %r11d
1400de11a: 48 8b 85 58 0a 00 00        	movq	0xa58(%rbp), %rax
1400de121: 48 8b 8d 68 0a 00 00        	movq	0xa68(%rbp), %rcx
1400de128: e9 8f 01 00 00              	jmp	0x1400de2bc <.text+0xdd2bc>
1400de12d: 0f 1f 00                    	nopl	(%rax)
1400de130: 4c 8b b5 e8 07 00 00        	movq	0x7e8(%rbp), %r14
1400de137: 41 b3 01                    	movb	$0x1, %r11b
1400de13a: 4d 85 f6                    	testq	%r14, %r14
1400de13d: 74 4d                       	je	0x1400de18c <.text+0xdd18c>
1400de13f: 4a 8d 34 b5 00 00 00 00     	leaq	(,%r14,4), %rsi
1400de147: 0f b6 05 bb 6a 55 03        	movzbl	0x3556abb(%rip), %eax   # 0x143634c09
1400de14e: ba 01 00 00 00              	movl	$0x1, %edx
1400de153: 48 89 f1                    	movq	%rsi, %rcx
1400de156: e8 b5 5a 53 00              	callq	0x140613c10 <.text+0x612c10>
1400de15b: 48 85 c0                    	testq	%rax, %rax
1400de15e: 0f 84 8d 14 00 00           	je	0x1400df5f1 <.text+0xde5f1>
1400de164: 49 89 c4                    	movq	%rax, %r12
1400de167: 49 83 fe 04                 	cmpq	$0x4, %r14
1400de16b: 0f 92 c0                    	setb	%al
1400de16e: 4c 89 e1                    	movq	%r12, %rcx
1400de171: 4c 29 f9                    	subq	%r15, %rcx
1400de174: 48 83 f9 10                 	cmpq	$0x10, %rcx
1400de178: 0f 92 c1                    	setb	%cl
1400de17b: 08 c1                       	orb	%al, %cl
1400de17d: 80 f9 01                    	cmpb	$0x1, %cl
1400de180: 75 1f                       	jne	0x1400de1a1 <.text+0xdd1a1>
1400de182: 31 c0                       	xorl	%eax, %eax
1400de184: 41 b3 01                    	movb	$0x1, %r11b
1400de187: e9 f4 00 00 00              	jmp	0x1400de280 <.text+0xdd280>
1400de18c: b8 01 00 00 00              	movl	$0x1, %eax
1400de191: 31 c9                       	xorl	%ecx, %ecx
1400de193: 45 31 f6                    	xorl	%r14d, %r14d
1400de196: 41 bc 01 00 00 00           	movl	$0x1, %r12d
1400de19c: e9 1b 01 00 00              	jmp	0x1400de2bc <.text+0xdd2bc>
1400de1a1: 4c 89 f0                    	movq	%r14, %rax
1400de1a4: 48 83 e0 fc                 	andq	$-0x4, %rax
1400de1a8: 31 c9                       	xorl	%ecx, %ecx
1400de1aa: 41 b3 01                    	movb	$0x1, %r11b
1400de1ad: 0f 1f 00                    	nopl	(%rax)
1400de1b0: 41 0f 10 04 8f              	movups	(%r15,%rcx,4), %xmm0
1400de1b5: 41 0f 59 c1                 	mulps	%xmm9, %xmm0
1400de1b9: 41 0f 58 c2                 	addps	%xmm10, %xmm0
1400de1bd: 0f 28 c8                    	movaps	%xmm0, %xmm1
1400de1c0: 0f 28 d0                    	movaps	%xmm0, %xmm2
1400de1c3: 0f c6 d0 ff                 	shufps	$0xff, %xmm0, %xmm2     # xmm2 = xmm2[3,3],xmm0[3,3]
1400de1c7: 0f 57 db                    	xorps	%xmm3, %xmm3
1400de1ca: f3 0f 5f da                 	maxss	%xmm2, %xmm3
1400de1ce: 0f 28 d7                    	movaps	%xmm7, %xmm2
1400de1d1: f3 0f 5d d3                 	minss	%xmm3, %xmm2
1400de1d5: f3 0f 2c d2                 	cvttss2si	%xmm2, %edx
1400de1d9: 0f 57 d2                    	xorps	%xmm2, %xmm2
1400de1dc: f3 0f 5f d0                 	maxss	%xmm0, %xmm2
1400de1e0: 66 0f 15 c8                 	unpckhpd	%xmm0, %xmm1            # xmm1 = xmm1[1],xmm0[1]
1400de1e4: 0f 57 db                    	xorps	%xmm3, %xmm3
1400de1e7: f3 0f 5f d9                 	maxss	%xmm1, %xmm3
1400de1eb: 0f 28 cf                    	movaps	%xmm7, %xmm1
1400de1ee: f3 0f 5d cb                 	minss	%xmm3, %xmm1
1400de1f2: f3 44 0f 2c c1              	cvttss2si	%xmm1, %r8d
1400de1f7: 0f 28 cf                    	movaps	%xmm7, %xmm1
1400de1fa: f3 0f 5d ca                 	minss	%xmm2, %xmm1
1400de1fe: f3 44 0f 2c c9              	cvttss2si	%xmm1, %r9d
1400de203: f3 0f 16 c0                 	movshdup	%xmm0, %xmm0            # xmm0 = xmm0[1,1,3,3]
1400de207: 0f 57 c9                    	xorps	%xmm1, %xmm1
1400de20a: f3 0f 5f c8                 	maxss	%xmm0, %xmm1
1400de20e: 0f 28 c7                    	movaps	%xmm7, %xmm0
1400de211: f3 0f 5d c1                 	minss	%xmm1, %xmm0
1400de215: f3 44 0f 2c d0              	cvttss2si	%xmm0, %r10d
1400de21a: 45 0f b6 d2                 	movzbl	%r10b, %r10d
1400de21e: 45 0f b6 c9                 	movzbl	%r9b, %r9d
1400de222: 66 41 0f 6e c1              	movd	%r9d, %xmm0
1400de227: 66 41 0f c4 c2 02           	pinsrw	$0x2, %r10d, %xmm0
1400de22d: 45 0f b6 c0                 	movzbl	%r8b, %r8d
1400de231: 66 41 0f c4 c0 04           	pinsrw	$0x4, %r8d, %xmm0
1400de237: 0f b6 d2                    	movzbl	%dl, %edx
1400de23a: 66 0f 6f c8                 	movdqa	%xmm0, %xmm1
1400de23e: 66 0f c4 ca 06              	pinsrw	$0x6, %edx, %xmm1
1400de243: 66 41 0f f4 c3              	pmuludq	%xmm11, %xmm0
1400de248: 66 0f 70 c0 e8              	pshufd	$0xe8, %xmm0, %xmm0     # xmm0 = xmm0[0,2,2,3]
1400de24d: 66 0f 70 c9 f5              	pshufd	$0xf5, %xmm1, %xmm1     # xmm1 = xmm1[1,1,3,3]
1400de252: 66 41 0f f4 cb              	pmuludq	%xmm11, %xmm1
1400de257: 66 0f 70 c9 e8              	pshufd	$0xe8, %xmm1, %xmm1     # xmm1 = xmm1[0,2,2,3]
1400de25c: 66 0f 62 c1                 	punpckldq	%xmm1, %xmm0    # xmm0 = xmm0[0],xmm1[0],xmm0[1],xmm1[1]
1400de260: f3 41 0f 7f 04 8c           	movdqu	%xmm0, (%r12,%rcx,4)
1400de266: 48 83 c1 04                 	addq	$0x4, %rcx
1400de26a: 48 39 c8                    	cmpq	%rcx, %rax
1400de26d: 0f 85 3d ff ff ff           	jne	0x1400de1b0 <.text+0xdd1b0>
1400de273: eb 3c                       	jmp	0x1400de2b1 <.text+0xdd2b1>
1400de275: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
1400de280: f3 41 0f 10 04 87           	movss	(%r15,%rax,4), %xmm0
1400de286: f3 0f 59 c7                 	mulss	%xmm7, %xmm0
1400de28a: f3 41 0f 58 c0              	addss	%xmm8, %xmm0
1400de28f: 0f 57 c9                    	xorps	%xmm1, %xmm1
1400de292: f3 0f 5f c8                 	maxss	%xmm0, %xmm1
1400de296: 0f 28 c7                    	movaps	%xmm7, %xmm0
1400de299: f3 0f 5d c1                 	minss	%xmm1, %xmm0
1400de29d: f3 0f 2c c8                 	cvttss2si	%xmm0, %ecx
1400de2a1: 0f b6 c9                    	movzbl	%cl, %ecx
1400de2a4: 69 c9 01 01 01 01           	imull	$0x1010101, %ecx, %ecx  # imm = 0x1010101
1400de2aa: 41 89 0c 84                 	movl	%ecx, (%r12,%rax,4)
1400de2ae: 48 ff c0                    	incq	%rax
1400de2b1: 49 39 c6                    	cmpq	%rax, %r14
1400de2b4: 75 ca                       	jne	0x1400de280 <.text+0xdd280>
1400de2b6: 4c 89 e0                    	movq	%r12, %rax
1400de2b9: 4c 89 f1                    	movq	%r14, %rcx
1400de2bc: 4c 89 ad b0 03 00 00        	movq	%r13, 0x3b0(%rbp)
1400de2c3: 48 8b 95 e0 09 00 00        	movq	0x9e0(%rbp), %rdx
1400de2ca: 48 89 95 b8 03 00 00        	movq	%rdx, 0x3b8(%rbp)
1400de2d1: 48 89 85 58 0a 00 00        	movq	%rax, 0xa58(%rbp)
1400de2d8: 48 89 85 48 0a 00 00        	movq	%rax, 0xa48(%rbp)
1400de2df: 48 89 8d 68 0a 00 00        	movq	%rcx, 0xa68(%rbp)
1400de2e6: 48 89 8d 40 0a 00 00        	movq	%rcx, 0xa40(%rbp)
1400de2ed: 44 89 9d 60 0a 00 00        	movl	%r11d, 0xa60(%rbp)
1400de2f4: 44 88 9d 75 0a 00 00        	movb	%r11b, 0xa75(%rbp)
1400de2fb: 48 8b 8d f0 09 00 00        	movq	0x9f0(%rbp), %rcx
1400de302: 48 8d 95 b0 03 00 00        	leaq	0x3b0(%rbp), %rdx
1400de309: e8 52 3a 4d 00              	callq	0x1405b1d60 <.text+0x5b0d60>
1400de30e: 48 89 c6                    	movq	%rax, %rsi
1400de311: 49 89 c7                    	movq	%rax, %r15
1400de314: 49 c1 ef 39                 	shrq	$0x39, %r15
1400de318: 48 8b 8d 50 0a 00 00        	movq	0xa50(%rbp), %rcx
1400de31f: 48 8b 41 78                 	movq	0x78(%rcx), %rax
1400de323: 48 8b 89 80 00 00 00        	movq	0x80(%rcx), %rcx
1400de32a: 48 89 ca                    	movq	%rcx, %rdx
1400de32d: 48 21 f2                    	andq	%rsi, %rdx
1400de330: 66 41 0f 6e c7              	movd	%r15d, %xmm0
1400de335: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1400de339: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1400de33e: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1400de343: 45 31 c0                    	xorl	%r8d, %r8d
1400de346: f3 0f 6f 0c 10              	movdqu	(%rax,%rdx), %xmm1
1400de34b: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
1400de34f: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
1400de353: 66 44 0f d7 ca              	pmovmskb	%xmm2, %r9d
1400de358: 45 85 c9                    	testl	%r9d, %r9d
1400de35b: 74 43                       	je	0x1400de3a0 <.text+0xdd3a0>
1400de35d: f3 45 0f bc d1              	tzcntl	%r9d, %r10d
1400de362: 49 01 d2                    	addq	%rdx, %r10
1400de365: 49 21 ca                    	andq	%rcx, %r10
1400de368: 49 f7 da                    	negq	%r10
1400de36b: 4f 8d 14 d2                 	leaq	(%r10,%r10,8), %r10
1400de36f: 4e 39 6c d0 b8              	cmpq	%r13, -0x48(%rax,%r10,8)
1400de374: 75 0e                       	jne	0x1400de384 <.text+0xdd384>
1400de376: 4c 8b 9d e0 09 00 00        	movq	0x9e0(%rbp), %r11
1400de37d: 4e 39 5c d0 c0              	cmpq	%r11, -0x40(%rax,%r10,8)
1400de382: 74 4c                       	je	0x1400de3d0 <.text+0xdd3d0>
1400de384: 45 8d 51 ff                 	leal	-0x1(%r9), %r10d
1400de388: 66 45 21 ca                 	andw	%r9w, %r10w
1400de38c: 45 89 d1                    	movl	%r10d, %r9d
1400de38f: 75 cc                       	jne	0x1400de35d <.text+0xdd35d>
1400de391: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1400de3a0: 66 0f 74 ce                 	pcmpeqb	%xmm6, %xmm1
1400de3a4: 66 44 0f d7 c9              	pmovmskb	%xmm1, %r9d
1400de3a9: 45 85 c9                    	testl	%r9d, %r9d
1400de3ac: 0f 85 ee 00 00 00           	jne	0x1400de4a0 <.text+0xdd4a0>
1400de3b2: 4c 01 c2                    	addq	%r8, %rdx
1400de3b5: 48 83 c2 10                 	addq	$0x10, %rdx
1400de3b9: 49 83 c0 10                 	addq	$0x10, %r8
1400de3bd: 48 21 ca                    	andq	%rcx, %rdx
1400de3c0: eb 84                       	jmp	0x1400de346 <.text+0xdd346>
1400de3c2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1400de3d0: 31 c9                       	xorl	%ecx, %ecx
1400de3d2: f6 85 c0 07 00 00 01        	testb	$0x1, 0x7c0(%rbp)
1400de3d9: ba 00 00 00 00              	movl	$0x0, %edx
1400de3de: 74 0c                       	je	0x1400de3ec <.text+0xdd3ec>
1400de3e0: 8b 8d c8 07 00 00           	movl	0x7c8(%rbp), %ecx
1400de3e6: 8b 95 d0 07 00 00           	movl	0x7d0(%rbp), %edx
1400de3ec: 4e 8d 04 d0                 	leaq	(%rax,%r10,8), %r8
1400de3f0: 49 c1 e6 02                 	shlq	$0x2, %r14
1400de3f4: 49 83 c0 c8                 	addq	$-0x38, %r8
1400de3f8: 4c 89 a5 00 0a 00 00        	movq	%r12, 0xa00(%rbp)
1400de3ff: 4c 89 b5 08 0a 00 00        	movq	%r14, 0xa08(%rbp)
1400de406: 89 8d 10 0a 00 00           	movl	%ecx, 0xa10(%rbp)
1400de40c: 89 95 14 0a 00 00           	movl	%edx, 0xa14(%rbp)
1400de412: c7 85 18 0a 00 00 00 00 00 00       	movl	$0x0, 0xa18(%rbp)
1400de41c: 89 bd 1c 0a 00 00           	movl	%edi, 0xa1c(%rbp)
1400de422: 89 9d 20 0a 00 00           	movl	%ebx, 0xa20(%rbp)
1400de428: 48 c7 85 24 0a 00 00 01 00 00 00    	movq	$0x1, 0xa24(%rbp)
1400de433: c7 85 2c 0a 00 00 00 00 00 00       	movl	$0x0, 0xa2c(%rbp)
1400de43d: 48 8b 85 58 0a 00 00        	movq	0xa58(%rbp), %rax
1400de444: 48 89 85 48 0a 00 00        	movq	%rax, 0xa48(%rbp)
1400de44b: 48 8b 85 68 0a 00 00        	movq	0xa68(%rbp), %rax
1400de452: 48 89 85 40 0a 00 00        	movq	%rax, 0xa40(%rbp)
1400de459: 8b 85 60 0a 00 00           	movl	0xa60(%rbp), %eax
1400de45f: 88 85 75 0a 00 00           	movb	%al, 0xa75(%rbp)
1400de465: c6 44 24 28 22              	movb	$0x22, 0x28(%rsp)
1400de46a: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1400de473: 48 8d 4d c8                 	leaq	-0x38(%rbp), %rcx
1400de477: 48 8b 95 f8 09 00 00        	movq	0x9f8(%rbp), %rdx
1400de47e: 4c 8d 8d 00 0a 00 00        	leaq	0xa00(%rbp), %r9
1400de485: e8 36 05 b2 00              	callq	0x140bfe9c0 <.text+0xbfd9c0>
1400de48a: 83 7d c8 0b                 	cmpl	$0xb, -0x38(%rbp)
1400de48e: 0f 84 3c 02 00 00           	je	0x1400de6d0 <.text+0xdd6d0>
1400de494: e9 64 10 00 00              	jmp	0x1400df4fd <.text+0xde4fd>
1400de499: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1400de4a0: 48 8b 85 50 0a 00 00        	movq	0xa50(%rbp), %rax
1400de4a7: 48 83 b8 88 00 00 00 00     	cmpq	$0x0, 0x88(%rax)
1400de4af: 0f 84 2c 03 00 00           	je	0x1400de7e1 <.text+0xdd7e1>
1400de4b5: 4c 89 b5 00 0a 00 00        	movq	%r14, 0xa00(%rbp)
1400de4bc: 0f af df                    	imull	%edi, %ebx
1400de4bf: 48 89 5d c8                 	movq	%rbx, -0x38(%rbp)
1400de4c3: 49 39 de                    	cmpq	%rbx, %r14
1400de4c6: 0f 85 74 0f 00 00           	jne	0x1400df440 <.text+0xde440>
1400de4cc: 48 8b 85 e0 07 00 00        	movq	0x7e0(%rbp), %rax
1400de4d3: 48 8d 48 28                 	leaq	0x28(%rax), %rcx
1400de4d7: 48 83 c0 30                 	addq	$0x30, %rax
1400de4db: 31 d2                       	xorl	%edx, %edx
1400de4dd: 48 3b 95 d8 07 00 00        	cmpq	0x7d8(%rbp), %rdx
1400de4e4: 48 8d 95 f0 07 00 00        	leaq	0x7f0(%rbp), %rdx
1400de4eb: 48 0f 41 ca                 	cmovnoq	%rdx, %rcx
1400de4ef: 8b 09                       	movl	(%rcx), %ecx
1400de4f1: 48 8d 95 f8 07 00 00        	leaq	0x7f8(%rbp), %rdx
1400de4f8: 48 0f 41 c2                 	cmovnoq	%rdx, %rax
1400de4fc: 48 8b 10                    	movq	(%rax), %rdx
1400de4ff: 48 c1 e2 20                 	shlq	$0x20, %rdx
1400de503: 48 09 ca                    	orq	%rcx, %rdx
1400de506: 48 8b 85 58 0a 00 00        	movq	0xa58(%rbp), %rax
1400de50d: 48 89 85 48 0a 00 00        	movq	%rax, 0xa48(%rbp)
1400de514: 48 8b 85 68 0a 00 00        	movq	0xa68(%rbp), %rax
1400de51b: 48 89 85 40 0a 00 00        	movq	%rax, 0xa40(%rbp)
1400de522: 8b 85 60 0a 00 00           	movl	0xa60(%rbp), %eax
1400de528: 88 85 75 0a 00 00           	movb	%al, 0xa75(%rbp)
1400de52e: 48 8d 8d b0 03 00 00        	leaq	0x3b0(%rbp), %rcx
1400de535: 41 b8 2b 00 00 00           	movl	$0x2b, %r8d
1400de53b: e8 60 6d af 00              	callq	0x140bd52a0 <.text+0xbd42a0>
1400de540: 48 8b 85 58 0a 00 00        	movq	0xa58(%rbp), %rax
1400de547: 48 89 85 48 0a 00 00        	movq	%rax, 0xa48(%rbp)
1400de54e: 48 8b 85 68 0a 00 00        	movq	0xa68(%rbp), %rax
1400de555: 48 89 85 40 0a 00 00        	movq	%rax, 0xa40(%rbp)
1400de55c: 8b 85 60 0a 00 00           	movl	0xa60(%rbp), %eax
1400de562: 88 85 75 0a 00 00           	movb	%al, 0xa75(%rbp)
1400de568: 48 8d 8d 00 0a 00 00        	leaq	0xa00(%rbp), %rcx
1400de56f: 48 8d 95 b0 03 00 00        	leaq	0x3b0(%rbp), %rdx
1400de576: e8 f5 6d af 00              	callq	0x140bd5370 <.text+0xbd4370>
1400de57b: 49 c1 e6 02                 	shlq	$0x2, %r14
1400de57f: 48 8b 85 58 0a 00 00        	movq	0xa58(%rbp), %rax
1400de586: 48 89 85 48 0a 00 00        	movq	%rax, 0xa48(%rbp)
1400de58d: 48 8b 85 68 0a 00 00        	movq	0xa68(%rbp), %rax
1400de594: 48 89 85 40 0a 00 00        	movq	%rax, 0xa40(%rbp)
1400de59b: 8b 85 60 0a 00 00           	movl	0xa60(%rbp), %eax
1400de5a1: 88 85 75 0a 00 00           	movb	%al, 0xa75(%rbp)
1400de5a7: 4c 89 74 24 30              	movq	%r14, 0x30(%rsp)
1400de5ac: 4c 89 64 24 28              	movq	%r12, 0x28(%rsp)
1400de5b1: 48 8d 85 00 0a 00 00        	leaq	0xa00(%rbp), %rax
1400de5b8: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1400de5bd: 41 b9 0c 00 00 00           	movl	$0xc, %r9d
1400de5c3: 48 8d 4d c8                 	leaq	-0x38(%rbp), %rcx
1400de5c7: 48 8b 95 f8 09 00 00        	movq	0x9f8(%rbp), %rdx
1400de5ce: 4c 8d 05 e3 cc 85 02        	leaq	0x285cce3(%rip), %r8    # 0x14293b2b8
1400de5d5: e8 26 b5 4d 00              	callq	0x1405b9b00 <.text+0x5b8b00>
1400de5da: 83 7d c8 0b                 	cmpl	$0xb, -0x38(%rbp)
1400de5de: 0f 85 a4 0f 00 00           	jne	0x1400df588 <.text+0xde588>
1400de5e4: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
1400de5e8: 48 8b 41 30                 	movq	0x30(%rcx), %rax
1400de5ec: 48 89 85 e0 03 00 00        	movq	%rax, 0x3e0(%rbp)
1400de5f3: 0f 10 01                    	movups	(%rcx), %xmm0
1400de5f6: f3 0f 6f 49 10              	movdqu	0x10(%rcx), %xmm1
1400de5fb: f3 0f 6f 51 20              	movdqu	0x20(%rcx), %xmm2
1400de600: 66 0f 7f 95 d0 03 00 00     	movdqa	%xmm2, 0x3d0(%rbp)
1400de608: 66 0f 7f 8d c0 03 00 00     	movdqa	%xmm1, 0x3c0(%rbp)
1400de610: 0f 29 85 b0 03 00 00        	movaps	%xmm0, 0x3b0(%rbp)
1400de617: 48 8b 8d 50 0a 00 00        	movq	0xa50(%rbp), %rcx
1400de61e: 48 8b 41 78                 	movq	0x78(%rcx), %rax
1400de622: 48 8b 89 80 00 00 00        	movq	0x80(%rcx), %rcx
1400de629: 48 21 ce                    	andq	%rcx, %rsi
1400de62c: f3 0f 6f 04 30              	movdqu	(%rax,%rsi), %xmm0
1400de631: 66 0f d7 d0                 	pmovmskb	%xmm0, %edx
1400de635: 85 d2                       	testl	%edx, %edx
1400de637: 0f 84 fe 01 00 00           	je	0x1400de83b <.text+0xdd83b>
1400de63d: f3 0f bc d2                 	tzcntl	%edx, %edx
1400de641: 48 01 f2                    	addq	%rsi, %rdx
1400de644: 48 21 ca                    	andq	%rcx, %rdx
1400de647: 44 0f b6 04 10              	movzbl	(%rax,%rdx), %r8d
1400de64c: 45 84 c0                    	testb	%r8b, %r8b
1400de64f: 0f 89 09 02 00 00           	jns	0x1400de85e <.text+0xdd85e>
1400de655: 4c 8d 4a f0                 	leaq	-0x10(%rdx), %r9
1400de659: 49 21 c9                    	andq	%rcx, %r9
1400de65c: 44 88 3c 10                 	movb	%r15b, (%rax,%rdx)
1400de660: 46 88 7c 08 10              	movb	%r15b, 0x10(%rax,%r9)
1400de665: 48 f7 da                    	negq	%rdx
1400de668: 48 8d 0c d2                 	leaq	(%rdx,%rdx,8), %rcx
1400de66c: 41 80 e0 01                 	andb	$0x1, %r8b
1400de670: 41 0f b6 d0                 	movzbl	%r8b, %edx
1400de674: 4c 8b 85 50 0a 00 00        	movq	0xa50(%rbp), %r8
1400de67b: 49 29 90 88 00 00 00        	subq	%rdx, 0x88(%r8)
1400de682: 4c 89 6c c8 b8              	movq	%r13, -0x48(%rax,%rcx,8)
1400de687: 48 8b 95 e0 09 00 00        	movq	0x9e0(%rbp), %rdx
1400de68e: 48 89 54 c8 c0              	movq	%rdx, -0x40(%rax,%rcx,8)
1400de693: 66 0f 6f 85 b0 03 00 00     	movdqa	0x3b0(%rbp), %xmm0
1400de69b: 66 0f 6f 8d c0 03 00 00     	movdqa	0x3c0(%rbp), %xmm1
1400de6a3: 66 0f 6f 95 d0 03 00 00     	movdqa	0x3d0(%rbp), %xmm2
1400de6ab: f3 0f 7f 44 c8 c8           	movdqu	%xmm0, -0x38(%rax,%rcx,8)
1400de6b1: f3 0f 7f 4c c8 d8           	movdqu	%xmm1, -0x28(%rax,%rcx,8)
1400de6b7: f3 0f 7f 54 c8 e8           	movdqu	%xmm2, -0x18(%rax,%rcx,8)
1400de6bd: 48 8b 95 e0 03 00 00        	movq	0x3e0(%rbp), %rdx
1400de6c4: 48 89 54 c8 f8              	movq	%rdx, -0x8(%rax,%rcx,8)
1400de6c9: 49 ff 80 90 00 00 00        	incq	0x90(%r8)
1400de6d0: 4c 8d 3d 39 ca 85 02        	leaq	0x285ca39(%rip), %r15   # 0x14293b110
1400de6d7: 48 8b 8d e8 09 00 00        	movq	0x9e8(%rbp), %rcx
1400de6de: 8b 95 60 0a 00 00           	movl	0xa60(%rbp), %edx
1400de6e4: 48 83 bd 68 0a 00 00 00     	cmpq	$0x0, 0xa68(%rbp)
1400de6ec: 0f 95 c0                    	setne	%al
1400de6ef: 84 c2                       	testb	%al, %dl
1400de6f1: 74 28                       	je	0x1400de71b <.text+0xdd71b>
1400de6f3: 48 8b 85 68 0a 00 00        	movq	0xa68(%rbp), %rax
1400de6fa: 48 8d 14 85 00 00 00 00     	leaq	(,%rax,4), %rdx
1400de702: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1400de708: 48 8b 8d 58 0a 00 00        	movq	0xa58(%rbp), %rcx
1400de70f: e8 0c 55 53 00              	callq	0x140613c20 <.text+0x612c20>
1400de714: 48 8b 8d 10 09 00 00        	movq	0x910(%rbp), %rcx
1400de71b: 48 85 c9                    	testq	%rcx, %rcx
1400de71e: 74 65                       	je	0x1400de785 <.text+0xdd785>
1400de720: e8 0b 9a b3 00              	callq	0x140c18130 <.text+0xc17130>
1400de725: 48 89 c1                    	movq	%rax, %rcx
1400de728: 48 8b 00                    	movq	(%rax), %rax
1400de72b: 48 85 c0                    	testq	%rax, %rax
1400de72e: 74 10                       	je	0x1400de740 <.text+0xdd740>
1400de730: 83 f8 01                    	cmpl	$0x1, %eax
1400de733: 0f 85 2d 0e 00 00           	jne	0x1400df566 <.text+0xde566>
1400de739: 48 83 c1 08                 	addq	$0x8, %rcx
1400de73d: eb 14                       	jmp	0x1400de753 <.text+0xdd753>
1400de73f: 90                          	nop
1400de740: 31 d2                       	xorl	%edx, %edx
1400de742: e8 f9 2e 7c 02              	callq	0x1428a1640 <.text+0x28a0640>
1400de747: 48 89 c1                    	movq	%rax, %rcx
1400de74a: 48 85 c0                    	testq	%rax, %rax
1400de74d: 0f 84 13 0e 00 00           	je	0x1400df566 <.text+0xde566>
1400de753: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1400de757: 0f 85 c1 0c 00 00           	jne	0x1400df41e <.text+0xde41e>
1400de75d: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1400de764: 48 89 8d 60 0a 00 00        	movq	%rcx, 0xa60(%rbp)
1400de76b: 48 83 c1 08                 	addq	$0x8, %rcx
1400de76f: 48 8b 95 18 09 00 00        	movq	0x918(%rbp), %rdx
1400de776: e8 75 89 b3 00              	callq	0x140c170f0 <.text+0xc160f0>
1400de77b: 48 8b 85 60 0a 00 00        	movq	0xa60(%rbp), %rax
1400de782: 48 ff 00                    	incq	(%rax)
1400de785: 48 8b 95 d8 07 00 00        	movq	0x7d8(%rbp), %rdx
1400de78c: 48 89 d0                    	movq	%rdx, %rax
1400de78f: 48 f7 d8                    	negq	%rax
1400de792: 71 2c                       	jno	0x1400de7c0 <.text+0xdd7c0>
1400de794: 48 8b 85 e0 07 00 00        	movq	0x7e0(%rbp), %rax
1400de79b: f0                          	lock
1400de79c: 48 ff 08                    	decq	(%rax)
1400de79f: 0f 85 5b f6 ff ff           	jne	0x1400dde00 <.text+0xdce00>
1400de7a5: 48 8d 8d e0 07 00 00        	leaq	0x7e0(%rbp), %rcx
1400de7ac: e8 8f f6 b9 00              	callq	0x140c7de40 <.text+0xc7ce40>
1400de7b1: e9 4a f6 ff ff              	jmp	0x1400dde00 <.text+0xdce00>
1400de7b6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1400de7c0: 0f 83 3a f6 ff ff           	jae	0x1400dde00 <.text+0xdce00>
1400de7c6: 48 8b 8d e0 07 00 00        	movq	0x7e0(%rbp), %rcx
1400de7cd: 48 c1 e2 02                 	shlq	$0x2, %rdx
1400de7d1: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1400de7d7: e8 44 54 53 00              	callq	0x140613c20 <.text+0x612c20>
1400de7dc: e9 1f f6 ff ff              	jmp	0x1400dde00 <.text+0xdce00>
1400de7e1: 48 8b 85 58 0a 00 00        	movq	0xa58(%rbp), %rax
1400de7e8: 48 89 85 48 0a 00 00        	movq	%rax, 0xa48(%rbp)
1400de7ef: 48 8b 85 68 0a 00 00        	movq	0xa68(%rbp), %rax
1400de7f6: 48 89 85 40 0a 00 00        	movq	%rax, 0xa40(%rbp)
1400de7fd: 8b 85 60 0a 00 00           	movl	0xa60(%rbp), %eax
1400de803: 88 85 75 0a 00 00           	movb	%al, 0xa75(%rbp)
1400de809: ba 01 00 00 00              	movl	$0x1, %edx
1400de80e: 48 8b 8d 48 09 00 00        	movq	0x948(%rbp), %rcx
1400de815: 4c 8b 85 f0 09 00 00        	movq	0x9f0(%rbp), %r8
1400de81c: e8 5f fb 7b 02              	callq	0x14289e380 <.text+0x289d380>
1400de821: 4c 8b ad b0 03 00 00        	movq	0x3b0(%rbp), %r13
1400de828: 48 8b 85 b8 03 00 00        	movq	0x3b8(%rbp), %rax
1400de82f: 48 89 85 e0 09 00 00        	movq	%rax, 0x9e0(%rbp)
1400de836: e9 7a fc ff ff              	jmp	0x1400de4b5 <.text+0xdd4b5>
1400de83b: 41 b8 10 00 00 00           	movl	$0x10, %r8d
1400de841: 4c 01 c6                    	addq	%r8, %rsi
1400de844: 48 21 ce                    	andq	%rcx, %rsi
1400de847: f3 0f 6f 04 30              	movdqu	(%rax,%rsi), %xmm0
1400de84c: 66 0f d7 d0                 	pmovmskb	%xmm0, %edx
1400de850: 49 83 c0 10                 	addq	$0x10, %r8
1400de854: 85 d2                       	testl	%edx, %edx
1400de856: 0f 85 e1 fd ff ff           	jne	0x1400de63d <.text+0xdd63d>
1400de85c: eb e3                       	jmp	0x1400de841 <.text+0xdd841>
1400de85e: 66 0f 6f 00                 	movdqa	(%rax), %xmm0
1400de862: 66 0f d7 d0                 	pmovmskb	%xmm0, %edx
1400de866: f3 0f bc d2                 	tzcntl	%edx, %edx
1400de86a: 44 0f b6 04 10              	movzbl	(%rax,%rdx), %r8d
1400de86f: e9 e1 fd ff ff              	jmp	0x1400de655 <.text+0xdd655>
1400de874: 48 8d 8d a0 09 00 00        	leaq	0x9a0(%rbp), %rcx
1400de87b: e8 30 39 4f 00              	callq	0x1405d21b0 <.text+0x5d11b0>
1400de880: 48 8b b5 a8 07 00 00        	movq	0x7a8(%rbp), %rsi
1400de887: 4c 8d 7e 38                 	leaq	0x38(%rsi), %r15
1400de88b: 48 ba b3 61 b4 27 07 ee 97 85       	movabsq	$-0x7a6811f8d84b9e4d, %rdx # imm = 0x8597EE0727B461B3
1400de895: 49 b8 5c f1 15 ae c5 7e 29 8b       	movabsq	$-0x74d6813a51ea0ea4, %r8 # imm = 0x8B297EC5AE15F15C
1400de89f: 4c 89 f9                    	movq	%r15, %rcx
1400de8a2: e8 49 ee ff ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
1400de8a7: 48 85 c0                    	testq	%rax, %rax
1400de8aa: 0f 84 9a 0d 00 00           	je	0x1400df64a <.text+0xde64a>
1400de8b0: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
1400de8b7: 0f 86 8d 0d 00 00           	jbe	0x1400df64a <.text+0xde64a>
1400de8bd: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
1400de8c4: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
1400de8c8: 48 85 c9                    	testq	%rcx, %rcx
1400de8cb: 0f 84 79 0d 00 00           	je	0x1400df64a <.text+0xde64a>
1400de8d1: 48 8b 86 e0 01 00 00        	movq	0x1e0(%rsi), %rax
1400de8d8: 48 f7 d1                    	notq	%rcx
1400de8db: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1400de8df: 48 c1 e1 05                 	shlq	$0x5, %rcx
1400de8e3: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
1400de8e9: 0f 84 5b 0d 00 00           	je	0x1400df64a <.text+0xde64a>
1400de8ef: 48 01 c8                    	addq	%rcx, %rax
1400de8f2: 48 8b 48 10                 	movq	0x10(%rax), %rcx
1400de8f6: 48 8d 15 53 c8 85 02        	leaq	0x285c853(%rip), %rdx   # 0x14293b150
1400de8fd: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1400de903: e8 98 91 85 00              	callq	0x140937aa0 <.text+0x936aa0>
1400de908: 48 89 95 68 0a 00 00        	movq	%rdx, 0xa68(%rbp)
1400de90f: a8 01                       	testb	$0x1, %al
1400de911: 48 8b bd b0 07 00 00        	movq	0x7b0(%rbp), %rdi
1400de918: 48 8b 9d b8 07 00 00        	movq	0x7b8(%rbp), %rbx
1400de91f: 0f 84 cd 0e 00 00           	je	0x1400df7f2 <.text+0xde7f2>
1400de925: 48 8d 4b 04                 	leaq	0x4(%rbx), %rcx
1400de929: e8 62 6e af 00              	callq	0x140bd5790 <.text+0xbd4790>
1400de92e: 49 89 c6                    	movq	%rax, %r14
1400de931: c7 85 c0 07 00 00 00 00 00 00       	movl	$0x0, 0x7c0(%rbp)
1400de93b: 48 89 9d c8 07 00 00        	movq	%rbx, 0x7c8(%rbp)
1400de942: 48 8d 8d c0 07 00 00        	leaq	0x7c0(%rbp), %rcx
1400de949: e8 a2 a9 82 00              	callq	0x1409092f0 <.text+0x9082f0>
1400de94e: 48 89 c1                    	movq	%rax, %rcx
1400de951: e8 3a 6e af 00              	callq	0x140bd5790 <.text+0xbd4790>
1400de956: 0f 10 85 c0 07 00 00        	movups	0x7c0(%rbp), %xmm0
1400de95d: f3 0f 6f 8d d0 07 00 00     	movdqu	0x7d0(%rbp), %xmm1
1400de965: 66 0f 7f 8d c0 03 00 00     	movdqa	%xmm1, 0x3c0(%rbp)
1400de96d: 0f 29 85 b0 03 00 00        	movaps	%xmm0, 0x3b0(%rbp)
1400de974: c7 85 50 09 00 00 02 00 00 00       	movl	$0x2, 0x950(%rbp)
1400de97e: 48 89 7c 24 40              	movq	%rdi, 0x40(%rsp)
1400de983: 48 89 74 24 38              	movq	%rsi, 0x38(%rsp)
1400de988: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
1400de98d: 48 8d 85 50 09 00 00        	leaq	0x950(%rbp), %rax
1400de994: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
1400de999: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1400de9a2: 48 8d 15 a7 c7 85 02        	leaq	0x285c7a7(%rip), %rdx   # 0x14293b150
1400de9a9: 48 8d 4d c8                 	leaq	-0x38(%rbp), %rcx
1400de9ad: 4c 8d 8d b0 03 00 00        	leaq	0x3b0(%rbp), %r9
1400de9b4: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1400de9ba: e8 b1 8f 78 00              	callq	0x140867970 <.text+0x866970>
1400de9bf: c6 85 74 0a 00 00 01        	movb	$0x1, 0xa74(%rbp)
1400de9c6: 48 ba d1 15 d4 50 39 30 5c 90       	movabsq	$-0x6fa3cfc6af2bea2f, %rdx # imm = 0x905C303950D415D1
1400de9d0: 49 b8 72 c8 35 70 26 de 67 46       	movabsq	$0x4667de267035c872, %r8 # imm = 0x4667DE267035C872
1400de9da: 4c 89 f9                    	movq	%r15, %rcx
1400de9dd: e8 0e ed ff ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
1400de9e2: 48 85 c0                    	testq	%rax, %rax
1400de9e5: 0f 84 6b 0c 00 00           	je	0x1400df656 <.text+0xde656>
1400de9eb: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
1400de9f2: 0f 86 5e 0c 00 00           	jbe	0x1400df656 <.text+0xde656>
1400de9f8: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
1400de9ff: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
1400dea03: 48 85 c9                    	testq	%rcx, %rcx
1400dea06: 0f 84 4a 0c 00 00           	je	0x1400df656 <.text+0xde656>
1400dea0c: 48 8b 86 e0 01 00 00        	movq	0x1e0(%rsi), %rax
1400dea13: 48 f7 d1                    	notq	%rcx
1400dea16: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1400dea1a: 48 c1 e1 05                 	shlq	$0x5, %rcx
1400dea1e: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
1400dea24: 0f 84 2c 0c 00 00           	je	0x1400df656 <.text+0xde656>
1400dea2a: 48 01 c8                    	addq	%rcx, %rax
1400dea2d: 4c 8b 78 10                 	movq	0x10(%rax), %r15
1400dea31: c6 85 74 0a 00 00 01        	movb	$0x1, 0xa74(%rbp)
1400dea38: 4c 89 f9                    	movq	%r15, %rcx
1400dea3b: e8 40 f3 59 00              	callq	0x14067dd80 <.text+0x67cd80>
1400dea40: c6 85 74 0a 00 00 01        	movb	$0x1, 0xa74(%rbp)
1400dea47: 48 89 c1                    	movq	%rax, %rcx
1400dea4a: e8 f1 33 f6 ff              	callq	0x140041e40 <.text+0x40e40>
1400dea4f: 0f 28 f0                    	movaps	%xmm0, %xmm6
1400dea52: c6 85 74 0a 00 00 01        	movb	$0x1, 0xa74(%rbp)
1400dea59: 4c 89 f9                    	movq	%r15, %rcx
1400dea5c: e8 1f f3 59 00              	callq	0x14067dd80 <.text+0x67cd80>
1400dea61: 48 8b 95 50 0a 00 00        	movq	0xa50(%rbp), %rdx
1400dea68: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
1400dea6c: 48 89 8d c0 03 00 00        	movq	%rcx, 0x3c0(%rbp)
1400dea73: f3 0f 6f 02                 	movdqu	(%rdx), %xmm0
1400dea77: 66 0f 7f 85 b0 03 00 00     	movdqa	%xmm0, 0x3b0(%rbp)
1400dea7f: 48 c7 02 00 00 00 00        	movq	$0x0, (%rdx)
1400dea86: 48 c7 42 08 08 00 00 00     	movq	$0x8, 0x8(%rdx)
1400dea8e: 48 c7 42 10 00 00 00 00     	movq	$0x0, 0x10(%rdx)
1400dea96: c6 85 74 0a 00 00 01        	movb	$0x1, 0xa74(%rbp)
1400dea9d: 48 8d 8d c0 07 00 00        	leaq	0x7c0(%rbp), %rcx
1400deaa4: 4c 8d a5 b0 03 00 00        	leaq	0x3b0(%rbp), %r12
1400deaab: 48 89 c2                    	movq	%rax, %rdx
1400deaae: 4d 89 e0                    	movq	%r12, %r8
1400deab1: 0f 28 de                    	movaps	%xmm6, %xmm3
1400deab4: e8 07 a7 b8 00              	callq	0x140c691c0 <.text+0xc681c0>
1400deab9: 4c 89 f2                    	movq	%r14, %rdx
1400deabc: 48 c1 ea 20                 	shrq	$0x20, %rdx
1400deac0: 4c 8b 85 c0 07 00 00        	movq	0x7c0(%rbp), %r8
1400deac7: 48 8b 85 c8 07 00 00        	movq	0x7c8(%rbp), %rax
1400deace: 48 8b 8d d0 07 00 00        	movq	0x7d0(%rbp), %rcx
1400dead5: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
1400dead9: 48 c1 e1 04                 	shlq	$0x4, %rcx
1400deadd: 48 01 c1                    	addq	%rax, %rcx
1400deae0: 48 89 85 f0 08 00 00        	movq	%rax, 0x8f0(%rbp)
1400deae7: 48 89 85 f8 08 00 00        	movq	%rax, 0x8f8(%rbp)
1400deaee: 4c 89 85 00 09 00 00        	movq	%r8, 0x900(%rbp)
1400deaf5: 48 89 8d 08 09 00 00        	movq	%rcx, 0x908(%rbp)
1400deafc: 48 8d bd c8 07 00 00        	leaq	0x7c8(%rbp), %rdi
1400deb03: 45 89 f0                    	movl	%r14d, %r8d
1400deb06: f3 49 0f 2a f8              	cvtsi2ss	%r8, %xmm7
1400deb0b: 89 d2                       	movl	%edx, %edx
1400deb0d: f3 4c 0f 2a c2              	cvtsi2ss	%rdx, %xmm8
1400deb12: 4c 8d b5 00 0a 00 00        	leaq	0xa00(%rbp), %r14
1400deb19: 66 45 0f 76 c9              	pcmpeqd	%xmm9, %xmm9
1400deb1e: 4c 8d ad c0 07 00 00        	leaq	0x7c0(%rbp), %r13
1400deb25: eb 17                       	jmp	0x1400deb3e <.text+0xddb3e>
1400deb27: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1400deb30: 48 8b 85 f8 08 00 00        	movq	0x8f8(%rbp), %rax
1400deb37: 48 8b 8d 08 09 00 00        	movq	0x908(%rbp), %rcx
1400deb3e: 48 39 c8                    	cmpq	%rcx, %rax
1400deb41: 0f 84 03 07 00 00           	je	0x1400df24a <.text+0xde24a>
1400deb47: 48 8d 48 50                 	leaq	0x50(%rax), %rcx
1400deb4b: 48 89 8d f8 08 00 00        	movq	%rcx, 0x8f8(%rbp)
1400deb52: 48 8b 08                    	movq	(%rax), %rcx
1400deb55: 48 8b 50 48                 	movq	0x48(%rax), %rdx
1400deb59: 48 89 95 90 09 00 00        	movq	%rdx, 0x990(%rbp)
1400deb60: 0f 10 40 38                 	movups	0x38(%rax), %xmm0
1400deb64: 0f 29 85 80 09 00 00        	movaps	%xmm0, 0x980(%rbp)
1400deb6b: 0f 10 40 28                 	movups	0x28(%rax), %xmm0
1400deb6f: 0f 29 85 70 09 00 00        	movaps	%xmm0, 0x970(%rbp)
1400deb76: 0f 10 40 18                 	movups	0x18(%rax), %xmm0
1400deb7a: 0f 29 85 60 09 00 00        	movaps	%xmm0, 0x960(%rbp)
1400deb81: 0f 10 40 08                 	movups	0x8(%rax), %xmm0
1400deb85: 0f 29 85 50 09 00 00        	movaps	%xmm0, 0x950(%rbp)
1400deb8c: 48 83 f9 03                 	cmpq	$0x3, %rcx
1400deb90: 0f 84 b4 06 00 00           	je	0x1400df24a <.text+0xde24a>
1400deb96: 48 89 8d a0 09 00 00        	movq	%rcx, 0x9a0(%rbp)
1400deb9d: 48 8b 85 80 09 00 00        	movq	0x980(%rbp), %rax
1400deba4: 48 8d 95 a8 09 00 00        	leaq	0x9a8(%rbp), %rdx
1400debab: 48 89 42 30                 	movq	%rax, 0x30(%rdx)
1400debaf: 0f 28 85 50 09 00 00        	movaps	0x950(%rbp), %xmm0
1400debb6: 0f 28 8d 60 09 00 00        	movaps	0x960(%rbp), %xmm1
1400debbd: 0f 28 95 70 09 00 00        	movaps	0x970(%rbp), %xmm2
1400debc4: 0f 11 52 20                 	movups	%xmm2, 0x20(%rdx)
1400debc8: 0f 11 4a 10                 	movups	%xmm1, 0x10(%rdx)
1400debcc: 0f 11 02                    	movups	%xmm0, (%rdx)
1400debcf: 48 83 f9 02                 	cmpq	$0x2, %rcx
1400debd3: 0f 84 54 0a 00 00           	je	0x1400df62d <.text+0xde62d>
1400debd9: 48 8b 85 a0 09 00 00        	movq	0x9a0(%rbp), %rax
1400debe0: 48 8b 8d a8 09 00 00        	movq	0x9a8(%rbp), %rcx
1400debe7: f3 0f 6f 85 b0 09 00 00     	movdqu	0x9b0(%rbp), %xmm0
1400debef: f3 0f 6f 8d c0 09 00 00     	movdqu	0x9c0(%rbp), %xmm1
1400debf7: f3 0f 6f 95 d0 09 00 00     	movdqu	0x9d0(%rbp), %xmm2
1400debff: 66 0f 7f 95 30 0a 00 00     	movdqa	%xmm2, 0xa30(%rbp)
1400dec07: 66 0f 7f 8d 20 0a 00 00     	movdqa	%xmm1, 0xa20(%rbp)
1400dec0f: 66 0f 7f 85 10 0a 00 00     	movdqa	%xmm0, 0xa10(%rbp)
1400dec17: 48 89 85 00 0a 00 00        	movq	%rax, 0xa00(%rbp)
1400dec1e: 48 89 8d 08 0a 00 00        	movq	%rcx, 0xa08(%rbp)
1400dec25: 48 83 bd 38 0a 00 00 00     	cmpq	$0x0, 0xa38(%rbp)
1400dec2d: 0f 84 ed 00 00 00           	je	0x1400ded20 <.text+0xddd20>
1400dec33: 48 83 bd 20 0a 00 00 00     	cmpq	$0x0, 0xa20(%rbp)
1400dec3b: 0f 84 df 00 00 00           	je	0x1400ded20 <.text+0xddd20>
1400dec41: 48 8b 85 50 0a 00 00        	movq	0xa50(%rbp), %rax
1400dec48: 48 83 b8 90 00 00 00 00     	cmpq	$0x0, 0x90(%rax)
1400dec50: 0f 84 e3 05 00 00           	je	0x1400df239 <.text+0xde239>
1400dec56: 48 8b 8d f0 09 00 00        	movq	0x9f0(%rbp), %rcx
1400dec5d: 4c 89 f2                    	movq	%r14, %rdx
1400dec60: e8 fb 30 4d 00              	callq	0x1405b1d60 <.text+0x5b0d60>
1400dec65: 48 8b 8d 50 0a 00 00        	movq	0xa50(%rbp), %rcx
1400dec6c: 48 8b 59 78                 	movq	0x78(%rcx), %rbx
1400dec70: 48 8b 89 80 00 00 00        	movq	0x80(%rcx), %rcx
1400dec77: 48 89 ca                    	movq	%rcx, %rdx
1400dec7a: 48 21 c2                    	andq	%rax, %rdx
1400dec7d: 48 c1 e8 39                 	shrq	$0x39, %rax
1400dec81: 66 0f 6e c0                 	movd	%eax, %xmm0
1400dec85: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1400dec89: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1400dec8e: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1400dec93: 48 8d 43 b8                 	leaq	-0x48(%rbx), %rax
1400dec97: 4c 8b 85 00 0a 00 00        	movq	0xa00(%rbp), %r8
1400dec9e: 4c 8b 8d 08 0a 00 00        	movq	0xa08(%rbp), %r9
1400deca5: 45 31 d2                    	xorl	%r10d, %r10d
1400deca8: f3 0f 6f 0c 13              	movdqu	(%rbx,%rdx), %xmm1
1400decad: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
1400decb1: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
1400decb5: 66 44 0f d7 da              	pmovmskb	%xmm2, %r11d
1400decba: 45 85 db                    	testl	%r11d, %r11d
1400decbd: 74 31                       	je	0x1400decf0 <.text+0xddcf0>
1400decbf: f3 41 0f bc f3              	tzcntl	%r11d, %esi
1400decc4: 48 01 d6                    	addq	%rdx, %rsi
1400decc7: 48 21 ce                    	andq	%rcx, %rsi
1400decca: 48 f7 de                    	negq	%rsi
1400deccd: 48 8d 34 f6                 	leaq	(%rsi,%rsi,8), %rsi
1400decd1: 4c 3b 04 f0                 	cmpq	(%rax,%rsi,8), %r8
1400decd5: 75 0b                       	jne	0x1400dece2 <.text+0xddce2>
1400decd7: 4c 3b 4c f0 08              	cmpq	0x8(%rax,%rsi,8), %r9
1400decdc: 0f 84 be 00 00 00           	je	0x1400deda0 <.text+0xddda0>
1400dece2: 41 8d 73 ff                 	leal	-0x1(%r11), %esi
1400dece6: 66 44 21 de                 	andw	%r11w, %si
1400decea: 41 89 f3                    	movl	%esi, %r11d
1400deced: 75 d0                       	jne	0x1400decbf <.text+0xddcbf>
1400decef: 90                          	nop
1400decf0: 66 41 0f 74 c9              	pcmpeqb	%xmm9, %xmm1
1400decf5: 66 44 0f d7 d9              	pmovmskb	%xmm1, %r11d
1400decfa: 45 85 db                    	testl	%r11d, %r11d
1400decfd: 0f 85 36 05 00 00           	jne	0x1400df239 <.text+0xde239>
1400ded03: 4c 01 d2                    	addq	%r10, %rdx
1400ded06: 48 83 c2 10                 	addq	$0x10, %rdx
1400ded0a: 49 83 c2 10                 	addq	$0x10, %r10
1400ded0e: 48 21 ca                    	andq	%rcx, %rdx
1400ded11: eb 95                       	jmp	0x1400deca8 <.text+0xddca8>
1400ded13: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1400ded20: 48 8b 95 10 0a 00 00        	movq	0xa10(%rbp), %rdx
1400ded27: 48 85 d2                    	testq	%rdx, %rdx
1400ded2a: 74 16                       	je	0x1400ded42 <.text+0xddd42>
1400ded2c: 48 8b 8d 18 0a 00 00        	movq	0xa18(%rbp), %rcx
1400ded33: 48 c1 e2 02                 	shlq	$0x2, %rdx
1400ded37: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1400ded3d: e8 de 4e 53 00              	callq	0x140613c20 <.text+0x612c20>
1400ded42: 48 8b 85 28 0a 00 00        	movq	0xa28(%rbp), %rax
1400ded49: 48 85 c0                    	testq	%rax, %rax
1400ded4c: 74 1a                       	je	0x1400ded68 <.text+0xddd68>
1400ded4e: 48 8b 8d 30 0a 00 00        	movq	0xa30(%rbp), %rcx
1400ded55: 48 c1 e0 02                 	shlq	$0x2, %rax
1400ded59: 48 8d 14 80                 	leaq	(%rax,%rax,4), %rdx
1400ded5d: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1400ded63: e8 b8 4e 53 00              	callq	0x140613c20 <.text+0x612c20>
1400ded68: 83 bd a0 09 00 00 02        	cmpl	$0x2, 0x9a0(%rbp)
1400ded6f: 0f 85 bb fd ff ff           	jne	0x1400deb30 <.text+0xddb30>
1400ded75: 48 8b 85 a8 09 00 00        	movq	0x9a8(%rbp), %rax
1400ded7c: f0                          	lock
1400ded7d: 48 ff 08                    	decq	(%rax)
1400ded80: 0f 85 aa fd ff ff           	jne	0x1400deb30 <.text+0xddb30>
1400ded86: 48 8d 8d a8 09 00 00        	leaq	0x9a8(%rbp), %rcx
1400ded8d: e8 3e 0c 57 00              	callq	0x14064f9d0 <.text+0x64e9d0>
1400ded92: e9 99 fd ff ff              	jmp	0x1400deb30 <.text+0xddb30>
1400ded97: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1400deda0: 48 8b 95 20 0a 00 00        	movq	0xa20(%rbp), %rdx
1400deda7: 48 c1 e2 02                 	shlq	$0x2, %rdx
1400dedab: 4c 89 e1                    	movq	%r12, %rcx
1400dedae: 41 b8 40 00 00 00           	movl	$0x40, %r8d
1400dedb4: e8 a7 60 af 00              	callq	0x140bd4e60 <.text+0xbd3e60>
1400dedb9: 4c 89 64 24 20              	movq	%r12, 0x20(%rsp)
1400dedbe: 41 b9 11 00 00 00           	movl	$0x11, %r9d
1400dedc4: 4c 89 e9                    	movq	%r13, %rcx
1400dedc7: 48 8b 95 f8 09 00 00        	movq	0x9f8(%rbp), %rdx
1400dedce: 4c 8d 05 cb c3 85 02        	leaq	0x285c3cb(%rip), %r8    # 0x14293b1a0
1400dedd5: e8 a6 fc b0 00              	callq	0x140beea80 <.text+0xbeda80>
1400dedda: 83 bd c0 07 00 00 0b        	cmpl	$0xb, 0x7c0(%rbp)
1400dede1: 0f 85 02 09 00 00           	jne	0x1400df6e9 <.text+0xde6e9>
1400dede7: 48 8b 47 20                 	movq	0x20(%rdi), %rax
1400dedeb: 48 89 85 e0 08 00 00        	movq	%rax, 0x8e0(%rbp)
1400dedf2: 0f 10 07                    	movups	(%rdi), %xmm0
1400dedf5: 0f 10 4f 10                 	movups	0x10(%rdi), %xmm1
1400dedf9: 0f 29 8d d0 08 00 00        	movaps	%xmm1, 0x8d0(%rbp)
1400dee00: 0f 29 85 c0 08 00 00        	movaps	%xmm0, 0x8c0(%rbp)
1400dee07: 4c 8b 85 f8 07 00 00        	movq	0x7f8(%rbp), %r8
1400dee0e: 48 8b 95 20 0a 00 00        	movq	0xa20(%rbp), %rdx
1400dee15: 48 c1 e2 02                 	shlq	$0x2, %rdx
1400dee19: 49 39 d0                    	cmpq	%rdx, %r8
1400dee1c: 0f 85 7b 06 00 00           	jne	0x1400df49d <.text+0xde49d>
1400dee22: 48 8b 8d f0 07 00 00        	movq	0x7f0(%rbp), %rcx
1400dee29: 48 8b 95 18 0a 00 00        	movq	0xa18(%rbp), %rdx
1400dee30: e8 cb 4d 84 02              	callq	0x142923c00 <.text+0x2922c00>
1400dee35: 48 8b 85 38 0a 00 00        	movq	0xa38(%rbp), %rax
1400dee3c: 48 c1 e0 02                 	shlq	$0x2, %rax
1400dee40: 48 8d 14 80                 	leaq	(%rax,%rax,4), %rdx
1400dee44: c6 85 73 0a 00 00 01        	movb	$0x1, 0xa73(%rbp)
1400dee4b: 4c 89 e1                    	movq	%r12, %rcx
1400dee4e: 41 b8 20 00 00 00           	movl	$0x20, %r8d
1400dee54: e8 07 60 af 00              	callq	0x140bd4e60 <.text+0xbd3e60>
1400dee59: c6 85 73 0a 00 00 01        	movb	$0x1, 0xa73(%rbp)
1400dee60: 4c 89 64 24 20              	movq	%r12, 0x20(%rsp)
1400dee65: 41 b9 12 00 00 00           	movl	$0x12, %r9d
1400dee6b: 4c 89 e9                    	movq	%r13, %rcx
1400dee6e: 48 8b 95 f8 09 00 00        	movq	0x9f8(%rbp), %rdx
1400dee75: 4c 8d 05 6c c3 85 02        	leaq	0x285c36c(%rip), %r8    # 0x14293b1e8
1400dee7c: e8 ff fb b0 00              	callq	0x140beea80 <.text+0xbeda80>
1400dee81: 83 bd c0 07 00 00 0b        	cmpl	$0xb, 0x7c0(%rbp)
1400dee88: 0f 85 d7 08 00 00           	jne	0x1400df765 <.text+0xde765>
1400dee8e: 48 8b 47 20                 	movq	0x20(%rdi), %rax
1400dee92: 48 89 85 30 09 00 00        	movq	%rax, 0x930(%rbp)
1400dee99: 0f 10 07                    	movups	(%rdi), %xmm0
1400dee9c: 0f 10 4f 10                 	movups	0x10(%rdi), %xmm1
1400deea0: 0f 29 8d 20 09 00 00        	movaps	%xmm1, 0x920(%rbp)
1400deea7: 0f 29 85 10 09 00 00        	movaps	%xmm0, 0x910(%rbp)
1400deeae: 4c 8b 85 f8 07 00 00        	movq	0x7f8(%rbp), %r8
1400deeb5: 48 8b 85 38 0a 00 00        	movq	0xa38(%rbp), %rax
1400deebc: 48 c1 e0 02                 	shlq	$0x2, %rax
1400deec0: 48 8d 14 80                 	leaq	(%rax,%rax,4), %rdx
1400deec4: 49 39 d0                    	cmpq	%rdx, %r8
1400deec7: 0f 85 ee 05 00 00           	jne	0x1400df4bb <.text+0xde4bb>
1400deecd: 48 8b 8d f0 07 00 00        	movq	0x7f0(%rbp), %rcx
1400deed4: 48 8b 95 30 0a 00 00        	movq	0xa30(%rbp), %rdx
1400deedb: e8 20 4d 84 02              	callq	0x142923c00 <.text+0x2922c00>
1400deee0: c6 85 77 0a 00 00 01        	movb	$0x1, 0xa77(%rbp)
1400deee7: c6 85 76 0a 00 00 01        	movb	$0x1, 0xa76(%rbp)
1400deeee: 4c 89 ff                    	movq	%r15, %rdi
1400deef1: 4c 89 f9                    	movq	%r15, %rcx
1400deef4: e8 87 ee 59 00              	callq	0x14067dd80 <.text+0x67cd80>
1400deef9: c6 85 77 0a 00 00 01        	movb	$0x1, 0xa77(%rbp)
1400def00: c6 85 76 0a 00 00 01        	movb	$0x1, 0xa76(%rbp)
1400def07: 48 89 c1                    	movq	%rax, %rcx
1400def0a: e8 31 2f f6 ff              	callq	0x140041e40 <.text+0x40e40>
1400def0f: 0f 28 f0                    	movaps	%xmm0, %xmm6
1400def12: c6 85 77 0a 00 00 01        	movb	$0x1, 0xa77(%rbp)
1400def19: c6 85 76 0a 00 00 01        	movb	$0x1, 0xa76(%rbp)
1400def20: 4c 89 e9                    	movq	%r13, %rcx
1400def23: 48 8b 95 68 0a 00 00        	movq	0xa68(%rbp), %rdx
1400def2a: 45 31 c0                    	xorl	%r8d, %r8d
1400def2d: e8 1e 89 78 00              	callq	0x140867850 <.text+0x866850>
1400def32: c6 85 77 0a 00 00 01        	movb	$0x1, 0xa77(%rbp)
1400def39: c6 85 76 0a 00 00 01        	movb	$0x1, 0xa76(%rbp)
1400def40: 4c 89 e1                    	movq	%r12, %rcx
1400def43: 4c 89 ea                    	movq	%r13, %rdx
1400def46: 4c 8d 45 c8                 	leaq	-0x38(%rbp), %r8
1400def4a: 45 31 c9                    	xorl	%r9d, %r9d
1400def4d: e8 8e 6c 82 00              	callq	0x140905be0 <.text+0x904be0>
1400def52: 4d 89 f7                    	movq	%r14, %r15
1400def55: 0f b6 05 ad 5c 55 03        	movzbl	0x3555cad(%rip), %eax   # 0x143634c09
1400def5c: b9 08 00 00 00              	movl	$0x8, %ecx
1400def61: ba 01 00 00 00              	movl	$0x1, %edx
1400def66: e8 a5 4c 53 00              	callq	0x140613c10 <.text+0x612c10>
1400def6b: 48 85 c0                    	testq	%rax, %rax
1400def6e: 0f 84 66 08 00 00           	je	0x1400df7da <.text+0xde7da>
1400def74: 49 89 c6                    	movq	%rax, %r14
1400def77: 0f 28 c7                    	movaps	%xmm7, %xmm0
1400def7a: f3 0f 5e c6                 	divss	%xmm6, %xmm0
1400def7e: 41 0f 28 c8                 	movaps	%xmm8, %xmm1
1400def82: f3 0f 5e ce                 	divss	%xmm6, %xmm1
1400def86: 66 0f 7e c8                 	movd	%xmm1, %eax
1400def8a: 48 c1 e0 20                 	shlq	$0x20, %rax
1400def8e: 66 0f 7e c1                 	movd	%xmm0, %ecx
1400def92: 48 09 c1                    	orq	%rax, %rcx
1400def95: 49 89 0e                    	movq	%rcx, (%r14)
1400def98: 48 8b 95 d8 03 00 00        	movq	0x3d8(%rbp), %rdx
1400def9f: 48 85 d2                    	testq	%rdx, %rdx
1400defa2: 74 12                       	je	0x1400defb6 <.text+0xddfb6>
1400defa4: 48 8b 8d e0 03 00 00        	movq	0x3e0(%rbp), %rcx
1400defab: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1400defb1: e8 6a 4c 53 00              	callq	0x140613c20 <.text+0x612c20>
1400defb6: 48 8d 34 f3                 	leaq	(%rbx,%rsi,8), %rsi
1400defba: 48 83 c6 c8                 	addq	$-0x38, %rsi
1400defbe: 48 c7 85 d8 03 00 00 08 00 00 00    	movq	$0x8, 0x3d8(%rbp)
1400defc9: 4c 89 b5 e0 03 00 00        	movq	%r14, 0x3e0(%rbp)
1400defd0: 48 c7 85 e8 03 00 00 08 00 00 00    	movq	$0x8, 0x3e8(%rbp)
1400defdb: 41 b8 f8 00 00 00           	movl	$0xf8, %r8d
1400defe1: 4c 89 eb                    	movq	%r13, %rbx
1400defe4: 4c 89 e9                    	movq	%r13, %rcx
1400defe7: 4d 89 e6                    	movq	%r12, %r14
1400defea: 4c 89 e2                    	movq	%r12, %rdx
1400defed: e8 0e 4c 84 02              	callq	0x142923c00 <.text+0x2922c00>
1400deff2: 48 89 b5 a0 07 00 00        	movq	%rsi, 0x7a0(%rbp)
1400deff9: c7 85 b0 03 00 00 02 00 00 00       	movl	$0x2, 0x3b0(%rbp)
1400df003: 4c 89 64 24 20              	movq	%r12, 0x20(%rsp)
1400df008: 48 8d 8d a0 07 00 00        	leaq	0x7a0(%rbp), %rcx
1400df00f: 4c 89 ea                    	movq	%r13, %rdx
1400df012: 49 b8 5e a9 e9 1c 54 8e 97 d9       	movabsq	$-0x266871abe31656a2, %r8 # imm = 0xD9978E541CE9A95E
1400df01c: 41 b1 1d                    	movb	$0x1d, %r9b
1400df01f: e8 7c 38 29 00              	callq	0x1403728a0 <.text+0x3718a0>
1400df024: 41 b8 f8 00 00 00           	movl	$0xf8, %r8d
1400df02a: 4c 89 f1                    	movq	%r14, %rcx
1400df02d: 48 89 da                    	movq	%rbx, %rdx
1400df030: e8 cb 4b 84 02              	callq	0x142923c00 <.text+0x2922c00>
1400df035: c6 85 77 0a 00 00 01        	movb	$0x1, 0xa77(%rbp)
1400df03c: c6 85 76 0a 00 00 01        	movb	$0x1, 0xa76(%rbp)
1400df043: 48 89 d9                    	movq	%rbx, %rcx
1400df046: 4c 89 f2                    	movq	%r14, %rdx
1400df049: 49 b8 0e 1f 7f 44 a9 39 ef 12       	movabsq	$0x12ef39a9447f1f0e, %r8 # imm = 0x12EF39A9447F1F0E
1400df053: 4c 8d 8d 10 09 00 00        	leaq	0x910(%rbp), %r9
1400df05a: e8 a1 73 82 00              	callq	0x140906400 <.text+0x905400>
1400df05f: 4d 89 fe                    	movq	%r15, %r14
1400df062: 80 a5 68 08 00 00 7f        	andb	$0x7f, 0x868(%rbp)
1400df069: 0f b6 85 6b 08 00 00        	movzbl	0x86b(%rbp), %eax
1400df070: 24 9f                       	andb	$-0x61, %al
1400df072: 0c 40                       	orb	$0x40, %al
1400df074: 88 85 6b 08 00 00           	movb	%al, 0x86b(%rbp)
1400df07a: 41 b8 f8 00 00 00           	movl	$0xf8, %r8d
1400df080: 4c 89 e1                    	movq	%r12, %rcx
1400df083: 4c 89 ea                    	movq	%r13, %rdx
1400df086: e8 75 4b 84 02              	callq	0x142923c00 <.text+0x2922c00>
1400df08b: 4c 8b 85 20 0a 00 00        	movq	0xa20(%rbp), %r8
1400df092: c6 85 77 0a 00 00 01        	movb	$0x1, 0xa77(%rbp)
1400df099: c6 85 76 0a 00 00 01        	movb	$0x1, 0xa76(%rbp)
1400df0a0: 4c 89 e1                    	movq	%r12, %rcx
1400df0a3: 48 8d 95 c0 08 00 00        	leaq	0x8c0(%rbp), %rdx
1400df0aa: e8 01 7c 82 00              	callq	0x140906cb0 <.text+0x905cb0>
1400df0af: 48 8b 85 e0 08 00 00        	movq	0x8e0(%rbp), %rax
1400df0b6: 48 89 85 d0 03 00 00        	movq	%rax, 0x3d0(%rbp)
1400df0bd: 0f 28 85 c0 08 00 00        	movaps	0x8c0(%rbp), %xmm0
1400df0c4: 0f 28 8d d0 08 00 00        	movaps	0x8d0(%rbp), %xmm1
1400df0cb: 0f 29 8d c0 03 00 00        	movaps	%xmm1, 0x3c0(%rbp)
1400df0d2: 0f 29 85 b0 03 00 00        	movaps	%xmm0, 0x3b0(%rbp)
1400df0d9: 48 8b 85 50 0a 00 00        	movq	0xa50(%rbp), %rax
1400df0e0: 48 8b 70 58                 	movq	0x58(%rax), %rsi
1400df0e4: 48 3b 70 48                 	cmpq	0x48(%rax), %rsi
1400df0e8: 49 89 ff                    	movq	%rdi, %r15
1400df0eb: 75 13                       	jne	0x1400df100 <.text+0xde100>
1400df0ed: 48 8b 8d 40 09 00 00        	movq	0x940(%rbp), %rcx
1400df0f4: 48 8d 15 35 c1 85 02        	leaq	0x285c135(%rip), %rdx   # 0x14293b230
1400df0fb: e8 b0 39 48 00              	callq	0x140562ab0 <.text+0x561ab0>
1400df100: 4c 8b 85 50 0a 00 00        	movq	0xa50(%rbp), %r8
1400df107: 49 8b 40 50                 	movq	0x50(%r8), %rax
1400df10b: 48 8d 0c b6                 	leaq	(%rsi,%rsi,4), %rcx
1400df10f: 0f 28 85 b0 03 00 00        	movaps	0x3b0(%rbp), %xmm0
1400df116: 0f 28 8d c0 03 00 00        	movaps	0x3c0(%rbp), %xmm1
1400df11d: 48 8b 95 d0 03 00 00        	movq	0x3d0(%rbp), %rdx
1400df124: 48 89 54 c8 20              	movq	%rdx, 0x20(%rax,%rcx,8)
1400df129: 0f 11 4c c8 10              	movups	%xmm1, 0x10(%rax,%rcx,8)
1400df12e: 0f 11 04 c8                 	movups	%xmm0, (%rax,%rcx,8)
1400df132: 48 8d 5e 01                 	leaq	0x1(%rsi), %rbx
1400df136: 49 89 58 58                 	movq	%rbx, 0x58(%r8)
1400df13a: 48 8b 85 30 09 00 00        	movq	0x930(%rbp), %rax
1400df141: 48 89 85 d0 03 00 00        	movq	%rax, 0x3d0(%rbp)
1400df148: 0f 28 85 10 09 00 00        	movaps	0x910(%rbp), %xmm0
1400df14f: 0f 28 8d 20 09 00 00        	movaps	0x920(%rbp), %xmm1
1400df156: 0f 29 8d c0 03 00 00        	movaps	%xmm1, 0x3c0(%rbp)
1400df15d: 0f 29 85 b0 03 00 00        	movaps	%xmm0, 0x3b0(%rbp)
1400df164: 49 3b 58 48                 	cmpq	0x48(%r8), %rbx
1400df168: 48 8d bd c8 07 00 00        	leaq	0x7c8(%rbp), %rdi
1400df16f: 75 13                       	jne	0x1400df184 <.text+0xde184>
1400df171: 48 8b 8d 40 09 00 00        	movq	0x940(%rbp), %rcx
1400df178: 48 8d 15 c9 c0 85 02        	leaq	0x285c0c9(%rip), %rdx   # 0x14293b248
1400df17f: e8 2c 39 48 00              	callq	0x140562ab0 <.text+0x561ab0>
1400df184: 4c 8b 85 50 0a 00 00        	movq	0xa50(%rbp), %r8
1400df18b: 49 8b 40 50                 	movq	0x50(%r8), %rax
1400df18f: 48 8d 0c 9b                 	leaq	(%rbx,%rbx,4), %rcx
1400df193: 66 0f 6f 85 b0 03 00 00     	movdqa	0x3b0(%rbp), %xmm0
1400df19b: 66 0f 6f 8d c0 03 00 00     	movdqa	0x3c0(%rbp), %xmm1
1400df1a3: 48 8b 95 d0 03 00 00        	movq	0x3d0(%rbp), %rdx
1400df1aa: 48 89 54 c8 20              	movq	%rdx, 0x20(%rax,%rcx,8)
1400df1af: f3 0f 7f 4c c8 10           	movdqu	%xmm1, 0x10(%rax,%rcx,8)
1400df1b5: f3 0f 7f 04 c8              	movdqu	%xmm0, (%rax,%rcx,8)
1400df1ba: 48 83 c6 02                 	addq	$0x2, %rsi
1400df1be: 49 89 70 58                 	movq	%rsi, 0x58(%r8)
1400df1c2: 48 8b 95 10 0a 00 00        	movq	0xa10(%rbp), %rdx
1400df1c9: 48 85 d2                    	testq	%rdx, %rdx
1400df1cc: 74 16                       	je	0x1400df1e4 <.text+0xde1e4>
1400df1ce: 48 8b 8d 18 0a 00 00        	movq	0xa18(%rbp), %rcx
1400df1d5: 48 c1 e2 02                 	shlq	$0x2, %rdx
1400df1d9: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1400df1df: e8 3c 4a 53 00              	callq	0x140613c20 <.text+0x612c20>
1400df1e4: 48 8b 85 28 0a 00 00        	movq	0xa28(%rbp), %rax
1400df1eb: 48 85 c0                    	testq	%rax, %rax
1400df1ee: 74 1a                       	je	0x1400df20a <.text+0xde20a>
1400df1f0: 48 8b 8d 30 0a 00 00        	movq	0xa30(%rbp), %rcx
1400df1f7: 48 c1 e0 02                 	shlq	$0x2, %rax
1400df1fb: 48 8d 14 80                 	leaq	(%rax,%rax,4), %rdx
1400df1ff: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1400df205: e8 16 4a 53 00              	callq	0x140613c20 <.text+0x612c20>
1400df20a: 83 bd a0 09 00 00 02        	cmpl	$0x2, 0x9a0(%rbp)
1400df211: 0f 85 19 f9 ff ff           	jne	0x1400deb30 <.text+0xddb30>
1400df217: 48 8b 85 a8 09 00 00        	movq	0x9a8(%rbp), %rax
1400df21e: f0                          	lock
1400df21f: 48 ff 08                    	decq	(%rax)
1400df222: 0f 85 08 f9 ff ff           	jne	0x1400deb30 <.text+0xddb30>
1400df228: 48 8d 8d a8 09 00 00        	leaq	0x9a8(%rbp), %rcx
1400df22f: e8 9c 07 57 00              	callq	0x14064f9d0 <.text+0x64e9d0>
1400df234: e9 f7 f8 ff ff              	jmp	0x1400deb30 <.text+0xddb30>
1400df239: 48 8d 0d 48 bf 85 02        	leaq	0x285bf48(%rip), %rcx   # 0x14293b188
1400df240: e8 cb a0 7f 02              	callq	0x1428d9310 <.text+0x28d8310>
1400df245: e9 a6 05 00 00              	jmp	0x1400df7f0 <.text+0xde7f0>
1400df24a: c6 85 74 0a 00 00 01        	movb	$0x1, 0xa74(%rbp)
1400df251: 48 8d 8d f0 08 00 00        	leaq	0x8f0(%rbp), %rcx
1400df258: e8 43 6c 52 00              	callq	0x140605ea0 <.text+0x604ea0>
1400df25d: 4c 8b a5 48 09 00 00        	movq	0x948(%rbp), %r12
1400df264: 48 8d b5 b0 03 00 00        	leaq	0x3b0(%rbp), %rsi
1400df26b: 48 8d 55 c8                 	leaq	-0x38(%rbp), %rdx
1400df26f: 41 b8 e8 03 00 00           	movl	$0x3e8, %r8d            # imm = 0x3E8
1400df275: 48 89 f1                    	movq	%rsi, %rcx
1400df278: e8 83 49 84 02              	callq	0x142923c00 <.text+0x2922c00>
1400df27d: c6 85 74 0a 00 00 00        	movb	$0x0, 0xa74(%rbp)
1400df284: 48 89 f1                    	movq	%rsi, %rcx
1400df287: e8 e4 90 78 00              	callq	0x140868370 <.text+0x867370>
1400df28c: 48 8b 95 50 0a 00 00        	movq	0xa50(%rbp), %rdx
1400df293: 48 8d 4a 30                 	leaq	0x30(%rdx), %rcx
1400df297: 48 8b 42 38                 	movq	0x38(%rdx), %rax
1400df29b: 48 8b 7a 40                 	movq	0x40(%rdx), %rdi
1400df29f: 48 c7 42 40 00 00 00 00     	movq	$0x0, 0x40(%rdx)
1400df2a7: 48 89 bd 68 09 00 00        	movq	%rdi, 0x968(%rbp)
1400df2ae: 48 c1 e7 04                 	shlq	$0x4, %rdi
1400df2b2: 48 01 c7                    	addq	%rax, %rdi
1400df2b5: 48 c7 85 70 09 00 00 00 00 00 00    	movq	$0x0, 0x970(%rbp)
1400df2c0: 48 89 bd 58 09 00 00        	movq	%rdi, 0x958(%rbp)
1400df2c7: 48 89 8d 60 09 00 00        	movq	%rcx, 0x960(%rbp)
1400df2ce: 4c 8d b5 c0 03 00 00        	leaq	0x3c0(%rbp), %r14
1400df2d5: 48 8d b5 00 0a 00 00        	leaq	0xa00(%rbp), %rsi
1400df2dc: 48 8d 9d b0 03 00 00        	leaq	0x3b0(%rbp), %rbx
1400df2e3: 48 39 f8                    	cmpq	%rdi, %rax
1400df2e6: 0f 84 f0 00 00 00           	je	0x1400df3dc <.text+0xde3dc>
1400df2ec: 0f 1f 40 00                 	nopl	(%rax)
1400df2f0: 4c 8d 78 10                 	leaq	0x10(%rax), %r15
1400df2f4: 4c 89 bd 50 09 00 00        	movq	%r15, 0x950(%rbp)
1400df2fb: 0f 10 00                    	movups	(%rax), %xmm0
1400df2fe: 0f 29 85 00 0a 00 00        	movaps	%xmm0, 0xa00(%rbp)
1400df305: 48 8b 8d f0 09 00 00        	movq	0x9f0(%rbp), %rcx
1400df30c: 48 89 f2                    	movq	%rsi, %rdx
1400df30f: e8 4c 2a 4d 00              	callq	0x1405b1d60 <.text+0x5b0d60>
1400df314: 48 89 d9                    	movq	%rbx, %rcx
1400df317: 4c 89 e2                    	movq	%r12, %rdx
1400df31a: 49 89 c0                    	movq	%rax, %r8
1400df31d: 49 89 f1                    	movq	%rsi, %r9
1400df320: e8 4b 41 f7 ff              	callq	0x140053470 <.text+0x52470>
1400df325: 83 bd b0 03 00 00 02        	cmpl	$0x2, 0x3b0(%rbp)
1400df32c: 0f 84 ae 01 00 00           	je	0x1400df4e0 <.text+0xde4e0>
1400df332: 49 8b 46 2e                 	movq	0x2e(%r14), %rax
1400df336: 48 89 85 ee 07 00 00        	movq	%rax, 0x7ee(%rbp)
1400df33d: 41 0f 10 06                 	movups	(%r14), %xmm0
1400df341: 41 0f 10 4e 10              	movups	0x10(%r14), %xmm1
1400df346: 41 0f 10 56 20              	movups	0x20(%r14), %xmm2
1400df34b: 0f 29 95 e0 07 00 00        	movaps	%xmm2, 0x7e0(%rbp)
1400df352: 0f 29 8d d0 07 00 00        	movaps	%xmm1, 0x7d0(%rbp)
1400df359: 0f 29 85 c0 07 00 00        	movaps	%xmm0, 0x7c0(%rbp)
1400df360: 0f b6 85 f6 03 00 00        	movzbl	0x3f6(%rbp), %eax
1400df367: 3c 07                       	cmpb	$0x7, %al
1400df369: 0f 84 71 01 00 00           	je	0x1400df4e0 <.text+0xde4e0>
1400df36f: 0f b6 8d f7 03 00 00        	movzbl	0x3f7(%rbp), %ecx
1400df376: 48 8b 95 ee 07 00 00        	movq	0x7ee(%rbp), %rdx
1400df37d: 48 89 95 de 03 00 00        	movq	%rdx, 0x3de(%rbp)
1400df384: 66 0f 6f 85 c0 07 00 00     	movdqa	0x7c0(%rbp), %xmm0
1400df38c: 66 0f 6f 8d d0 07 00 00     	movdqa	0x7d0(%rbp), %xmm1
1400df394: 66 0f 6f 95 e0 07 00 00     	movdqa	0x7e0(%rbp), %xmm2
1400df39c: 66 0f 7f 95 d0 03 00 00     	movdqa	%xmm2, 0x3d0(%rbp)
1400df3a4: 66 0f 7f 8d c0 03 00 00     	movdqa	%xmm1, 0x3c0(%rbp)
1400df3ac: 66 0f 7f 85 b0 03 00 00     	movdqa	%xmm0, 0x3b0(%rbp)
1400df3b4: 88 85 e6 03 00 00           	movb	%al, 0x3e6(%rbp)
1400df3ba: 88 8d e7 03 00 00           	movb	%cl, 0x3e7(%rbp)
1400df3c0: 48 89 d9                    	movq	%rbx, %rcx
1400df3c3: e8 b8 c7 f5 ff              	callq	0x14003bb80 <.text+0x3ab80>
1400df3c8: 48 89 d9                    	movq	%rbx, %rcx
1400df3cb: e8 e0 30 b0 00              	callq	0x140be24b0 <.text+0xbe14b0>
1400df3d0: 4c 89 f8                    	movq	%r15, %rax
1400df3d3: 48 39 f8                    	cmpq	%rdi, %rax
1400df3d6: 0f 85 14 ff ff ff           	jne	0x1400df2f0 <.text+0xde2f0>
1400df3dc: 0f 28 b5 80 0a 00 00        	movaps	0xa80(%rbp), %xmm6
1400df3e3: 0f 28 bd 90 0a 00 00        	movaps	0xa90(%rbp), %xmm7
1400df3ea: 44 0f 28 85 a0 0a 00 00     	movaps	0xaa0(%rbp), %xmm8
1400df3f2: 44 0f 28 8d b0 0a 00 00     	movaps	0xab0(%rbp), %xmm9
1400df3fa: 44 0f 28 95 c0 0a 00 00     	movaps	0xac0(%rbp), %xmm10
1400df402: 44 0f 28 9d d0 0a 00 00     	movaps	0xad0(%rbp), %xmm11
1400df40a: 48 81 c4 68 0b 00 00        	addq	$0xb68, %rsp            # imm = 0xB68
1400df411: 5b                          	popq	%rbx
1400df412: 5f                          	popq	%rdi
1400df413: 5e                          	popq	%rsi
1400df414: 41 5c                       	popq	%r12
1400df416: 41 5d                       	popq	%r13
1400df418: 41 5e                       	popq	%r14
1400df41a: 41 5f                       	popq	%r15
1400df41c: 5d                          	popq	%rbp
1400df41d: c3                          	retq
1400df41e: 48 8d 0d 93 36 85 02        	leaq	0x2853693(%rip), %rcx   # 0x142932ab8
1400df425: e8 26 9e 7f 02              	callq	0x1428d9250 <.text+0x28d8250>
1400df42a: e9 c1 03 00 00              	jmp	0x1400df7f0 <.text+0xde7f0>
1400df42f: 48 8d 0d 82 36 85 02        	leaq	0x2853682(%rip), %rcx   # 0x142932ab8
1400df436: e8 15 9e 7f 02              	callq	0x1428d9250 <.text+0x28d8250>
1400df43b: e9 b0 03 00 00              	jmp	0x1400df7f0 <.text+0xde7f0>
1400df440: 48 c7 85 b0 03 00 00 00 00 00 00    	movq	$0x0, 0x3b0(%rbp)
1400df44b: 48 8b 85 58 0a 00 00        	movq	0xa58(%rbp), %rax
1400df452: 48 89 85 48 0a 00 00        	movq	%rax, 0xa48(%rbp)
1400df459: 48 8b 85 68 0a 00 00        	movq	0xa68(%rbp), %rax
1400df460: 48 89 85 40 0a 00 00        	movq	%rax, 0xa40(%rbp)
1400df467: 8b 85 60 0a 00 00           	movl	0xa60(%rbp), %eax
1400df46d: 88 85 75 0a 00 00           	movb	%al, 0xa75(%rbp)
1400df473: 48 8d 05 26 be 85 02        	leaq	0x285be26(%rip), %rax   # 0x14293b2a0
1400df47a: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1400df47f: 48 8d 95 00 0a 00 00        	leaq	0xa00(%rbp), %rdx
1400df486: 4c 8d 45 c8                 	leaq	-0x38(%rbp), %r8
1400df48a: 4c 8d 8d b0 03 00 00        	leaq	0x3b0(%rbp), %r9
1400df491: 31 c9                       	xorl	%ecx, %ecx
1400df493: e8 fe a0 7f 02              	callq	0x1428d9596 <.text+0x28d8596>
1400df498: e9 53 03 00 00              	jmp	0x1400df7f0 <.text+0xde7f0>
1400df49d: c6 85 73 0a 00 00 01        	movb	$0x1, 0xa73(%rbp)
1400df4a4: 48 8d 05 25 bd 85 02        	leaq	0x285bd25(%rip), %rax   # 0x14293b1d0
1400df4ab: 4c 89 c1                    	movq	%r8, %rcx
1400df4ae: 49 89 c0                    	movq	%rax, %r8
1400df4b1: e8 2a a4 7f 02              	callq	0x1428d98e0 <.text+0x28d88e0>
1400df4b6: e9 35 03 00 00              	jmp	0x1400df7f0 <.text+0xde7f0>
1400df4bb: c6 85 77 0a 00 00 01        	movb	$0x1, 0xa77(%rbp)
1400df4c2: c6 85 76 0a 00 00 01        	movb	$0x1, 0xa76(%rbp)
1400df4c9: 48 8d 05 48 bd 85 02        	leaq	0x285bd48(%rip), %rax   # 0x14293b218
1400df4d0: 4c 89 c1                    	movq	%r8, %rcx
1400df4d3: 49 89 c0                    	movq	%rax, %r8
1400df4d6: e8 05 a4 7f 02              	callq	0x1428d98e0 <.text+0x28d88e0>
1400df4db: e9 10 03 00 00              	jmp	0x1400df7f0 <.text+0xde7f0>
1400df4e0: 48 8d 0d f9 bd 85 02        	leaq	0x285bdf9(%rip), %rcx   # 0x14293b2e0
1400df4e7: 4c 8d 05 0a be 85 02        	leaq	0x285be0a(%rip), %r8    # 0x14293b2f8
1400df4ee: ba 11 00 00 00              	movl	$0x11, %edx
1400df4f3: e8 38 9e 7f 02              	callq	0x1428d9330 <.text+0x28d8330>
1400df4f8: e9 f3 02 00 00              	jmp	0x1400df7f0 <.text+0xde7f0>
1400df4fd: 0f 10 45 08                 	movups	0x8(%rbp), %xmm0
1400df501: 0f 29 85 f0 03 00 00        	movaps	%xmm0, 0x3f0(%rbp)
1400df508: f3 0f 6f 45 c8              	movdqu	-0x38(%rbp), %xmm0
1400df50d: 0f 10 4d d8                 	movups	-0x28(%rbp), %xmm1
1400df511: 0f 10 55 e8                 	movups	-0x18(%rbp), %xmm2
1400df515: 0f 10 5d f8                 	movups	-0x8(%rbp), %xmm3
1400df519: 0f 29 9d e0 03 00 00        	movaps	%xmm3, 0x3e0(%rbp)
1400df520: 0f 29 95 d0 03 00 00        	movaps	%xmm2, 0x3d0(%rbp)
1400df527: 0f 29 8d c0 03 00 00        	movaps	%xmm1, 0x3c0(%rbp)
1400df52e: 66 0f 7f 85 b0 03 00 00     	movdqa	%xmm0, 0x3b0(%rbp)
1400df536: 48 8d 05 4b bd 85 02        	leaq	0x285bd4b(%rip), %rax   # 0x14293b288
1400df53d: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1400df542: 48 8d 0d 87 b8 85 02        	leaq	0x285b887(%rip), %rcx   # 0x14293add0
1400df549: 4c 8d 0d 60 b8 85 02        	leaq	0x285b860(%rip), %r9    # 0x14293adb0
1400df550: 4c 8d 85 b0 03 00 00        	leaq	0x3b0(%rbp), %r8
1400df557: ba 2b 00 00 00              	movl	$0x2b, %edx
1400df55c: e8 ff a1 7f 02              	callq	0x1428d9760 <.text+0x28d8760>
1400df561: e9 8a 02 00 00              	jmp	0x1400df7f0 <.text+0xde7f0>
1400df566: 48 8d 0d a3 34 85 02        	leaq	0x28534a3(%rip), %rcx   # 0x142932a10
1400df56d: e8 5e 8b 7f 02              	callq	0x1428d80d0 <.text+0x28d70d0>
1400df572: e9 79 02 00 00              	jmp	0x1400df7f0 <.text+0xde7f0>
1400df577: 48 8d 0d 92 34 85 02        	leaq	0x2853492(%rip), %rcx   # 0x142932a10
1400df57e: e8 4d 8b 7f 02              	callq	0x1428d80d0 <.text+0x28d70d0>
1400df583: e9 68 02 00 00              	jmp	0x1400df7f0 <.text+0xde7f0>
1400df588: 0f 10 45 08                 	movups	0x8(%rbp), %xmm0
1400df58c: 0f 29 85 f0 03 00 00        	movaps	%xmm0, 0x3f0(%rbp)
1400df593: f3 0f 6f 45 c8              	movdqu	-0x38(%rbp), %xmm0
1400df598: 0f 10 4d d8                 	movups	-0x28(%rbp), %xmm1
1400df59c: 0f 10 55 e8                 	movups	-0x18(%rbp), %xmm2
1400df5a0: 0f 10 5d f8                 	movups	-0x8(%rbp), %xmm3
1400df5a4: 0f 29 9d e0 03 00 00        	movaps	%xmm3, 0x3e0(%rbp)
1400df5ab: 0f 29 95 d0 03 00 00        	movaps	%xmm2, 0x3d0(%rbp)
1400df5b2: 0f 29 8d c0 03 00 00        	movaps	%xmm1, 0x3c0(%rbp)
1400df5b9: 66 0f 7f 85 b0 03 00 00     	movdqa	%xmm0, 0x3b0(%rbp)
1400df5c1: 48 8d 05 00 bd 85 02        	leaq	0x285bd00(%rip), %rax   # 0x14293b2c8
1400df5c8: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1400df5cd: 48 8d 0d fc b7 85 02        	leaq	0x285b7fc(%rip), %rcx   # 0x14293add0
1400df5d4: 4c 8d 0d d5 b7 85 02        	leaq	0x285b7d5(%rip), %r9    # 0x14293adb0
1400df5db: 4c 8d 85 b0 03 00 00        	leaq	0x3b0(%rbp), %r8
1400df5e2: ba 2b 00 00 00              	movl	$0x2b, %edx
1400df5e7: e8 74 a1 7f 02              	callq	0x1428d9760 <.text+0x28d8760>
1400df5ec: e9 ff 01 00 00              	jmp	0x1400df7f0 <.text+0xde7f0>
1400df5f1: 48 8b 85 58 0a 00 00        	movq	0xa58(%rbp), %rax
1400df5f8: 48 89 85 48 0a 00 00        	movq	%rax, 0xa48(%rbp)
1400df5ff: 48 8b 85 68 0a 00 00        	movq	0xa68(%rbp), %rax
1400df606: 48 89 85 40 0a 00 00        	movq	%rax, 0xa40(%rbp)
1400df60d: c6 85 75 0a 00 00 00        	movb	$0x0, 0xa75(%rbp)
1400df614: 4c 8d 05 7d b7 85 02        	leaq	0x285b77d(%rip), %r8    # 0x14293ad98
1400df61b: b9 01 00 00 00              	movl	$0x1, %ecx
1400df620: 48 89 f2                    	movq	%rsi, %rdx
1400df623: e8 bb 99 7f 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1400df628: e9 c3 01 00 00              	jmp	0x1400df7f0 <.text+0xde7f0>
1400df62d: 48 8d 0d 2c bc 85 02        	leaq	0x285bc2c(%rip), %rcx   # 0x14293b260
1400df634: 4c 8d 05 35 bc 85 02        	leaq	0x285bc35(%rip), %r8    # 0x14293b270
1400df63b: ba 0f 00 00 00              	movl	$0xf, %edx
1400df640: e8 eb 9d 7f 02              	callq	0x1428d9430 <.text+0x28d8430>
1400df645: e9 a6 01 00 00              	jmp	0x1400df7f0 <.text+0xde7f0>
1400df64a: 48 8d 0d e7 ba 85 02        	leaq	0x285bae7(%rip), %rcx   # 0x14293b138
1400df651: e8 ba 9c 7f 02              	callq	0x1428d9310 <.text+0x28d8310>
1400df656: c6 85 74 0a 00 00 01        	movb	$0x1, 0xa74(%rbp)
1400df65d: 48 8d 0d 0c bb 85 02        	leaq	0x285bb0c(%rip), %rcx   # 0x14293b170
1400df664: e8 a7 9c 7f 02              	callq	0x1428d9310 <.text+0x28d8310>
1400df669: e9 82 01 00 00              	jmp	0x1400df7f0 <.text+0xde7f0>
1400df66e: 48 8d 05 23 b8 85 02        	leaq	0x285b823(%rip), %rax   # 0x14293ae98
1400df675: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1400df67a: 48 8d 0d 8f ba 85 02        	leaq	0x285ba8f(%rip), %rcx   # 0x14293b110
1400df681: ba 25 00 00 00              	movl	$0x25, %edx
1400df686: 41 b9 25 00 00 00           	movl	$0x25, %r9d
1400df68c: 4d 89 f0                    	movq	%r14, %r8
1400df68f: e8 ac a2 7f 02              	callq	0x1428d9940 <.text+0x28d8940>
1400df694: e9 57 01 00 00              	jmp	0x1400df7f0 <.text+0xde7f0>
1400df699: 48 8d 05 18 b8 85 02        	leaq	0x285b818(%rip), %rax   # 0x14293aeb8
1400df6a0: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1400df6a5: 48 8d 0d 4f b7 85 02        	leaq	0x285b74f(%rip), %rcx   # 0x14293adfb
1400df6ac: ba 36 00 00 00              	movl	$0x36, %edx
1400df6b1: 45 31 c0                    	xorl	%r8d, %r8d
1400df6b4: e8 87 a2 7f 02              	callq	0x1428d9940 <.text+0x28d8940>
1400df6b9: e9 32 01 00 00              	jmp	0x1400df7f0 <.text+0xde7f0>
1400df6be: 48 8d 05 0b b8 85 02        	leaq	0x285b80b(%rip), %rax   # 0x14293aed0
1400df6c5: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1400df6ca: 48 8d 0d 2a b7 85 02        	leaq	0x285b72a(%rip), %rcx   # 0x14293adfb
1400df6d1: ba 36 00 00 00              	movl	$0x36, %edx
1400df6d6: 41 b9 36 00 00 00           	movl	$0x36, %r9d
1400df6dc: 4d 89 e0                    	movq	%r12, %r8
1400df6df: e8 5c a2 7f 02              	callq	0x1428d9940 <.text+0x28d8940>
1400df6e4: e9 07 01 00 00              	jmp	0x1400df7f0 <.text+0xde7f0>
1400df6e9: 0f 10 85 00 08 00 00        	movups	0x800(%rbp), %xmm0
1400df6f0: 0f 29 85 f0 03 00 00        	movaps	%xmm0, 0x3f0(%rbp)
1400df6f7: f3 0f 6f 85 c0 07 00 00     	movdqu	0x7c0(%rbp), %xmm0
1400df6ff: f3 0f 6f 8d d0 07 00 00     	movdqu	0x7d0(%rbp), %xmm1
1400df707: f3 0f 6f 95 e0 07 00 00     	movdqu	0x7e0(%rbp), %xmm2
1400df70f: 0f 10 9d f0 07 00 00        	movups	0x7f0(%rbp), %xmm3
1400df716: 0f 29 9d e0 03 00 00        	movaps	%xmm3, 0x3e0(%rbp)
1400df71d: 66 0f 7f 95 d0 03 00 00     	movdqa	%xmm2, 0x3d0(%rbp)
