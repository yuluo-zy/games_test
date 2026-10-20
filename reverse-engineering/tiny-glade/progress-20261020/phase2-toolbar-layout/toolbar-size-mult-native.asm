
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140feff70: 56                          	pushq	%rsi
140feff71: 57                          	pushq	%rdi
140feff72: 48 81 ec 88 00 00 00        	subq	$0x88, %rsp
140feff79: b8 01 00 00 00              	movl	$0x1, %eax
140feff7e: f0                          	lock
140feff7f: 0f c1 82 b0 05 00 00        	xaddl	%eax, 0x5b0(%rdx)
140feff86: f6 01 01                    	testb	$0x1, (%rcx)
140feff89: 0f 84 69 02 00 00           	je	0x140ff01f8 <.text+0xfef1f8>
140feff8f: 4c 8b 49 08                 	movq	0x8(%rcx), %r9
140feff93: 4c 8b 82 18 02 00 00        	movq	0x218(%rdx), %r8
140feff9a: 4d 39 c8                    	cmpq	%r9, %r8
140feff9d: 0f 86 2d 01 00 00           	jbe	0x140ff00d0 <.text+0xfef0d0>
140feffa3: 4c 8b 9a 10 02 00 00        	movq	0x210(%rdx), %r11
140feffaa: 4f 8b 14 cb                 	movq	(%r11,%r9,8), %r10
140feffae: 4d 85 d2                    	testq	%r10, %r10
140feffb1: 0f 84 19 01 00 00           	je	0x140ff00d0 <.text+0xfef0d0>
140feffb7: 4c 8b 8a e0 01 00 00        	movq	0x1e0(%rdx), %r9
140feffbe: 49 f7 d2                    	notq	%r10
140feffc1: 4f 8d 14 52                 	leaq	(%r10,%r10,2), %r10
140feffc5: 49 c1 e2 05                 	shlq	$0x5, %r10
140feffc9: 4b 83 7c 11 20 00           	cmpq	$0x0, 0x20(%r9,%r10)
140feffcf: 0f 84 fb 00 00 00           	je	0x140ff00d0 <.text+0xfef0d0>
140feffd5: 48 8b 71 10                 	movq	0x10(%rcx), %rsi
140feffd9: 49 39 f0                    	cmpq	%rsi, %r8
140feffdc: 0f 86 0c 01 00 00           	jbe	0x140ff00ee <.text+0xfef0ee>
140feffe2: 49 8b 34 f3                 	movq	(%r11,%rsi,8), %rsi
140feffe6: 48 85 f6                    	testq	%rsi, %rsi
140feffe9: 0f 84 ff 00 00 00           	je	0x140ff00ee <.text+0xfef0ee>
140feffef: 48 f7 d6                    	notq	%rsi
140fefff2: 48 8d 34 76                 	leaq	(%rsi,%rsi,2), %rsi
140fefff6: 48 c1 e6 05                 	shlq	$0x5, %rsi
140fefffa: 49 83 7c 31 20 00           	cmpq	$0x0, 0x20(%r9,%rsi)
140ff0000: 0f 84 e8 00 00 00           	je	0x140ff00ee <.text+0xfef0ee>
140ff0006: 48 8b 79 18                 	movq	0x18(%rcx), %rdi
140ff000a: 49 39 f8                    	cmpq	%rdi, %r8
140ff000d: 0f 86 60 01 00 00           	jbe	0x140ff0173 <.text+0xfef173>
140ff0013: 4d 8b 04 fb                 	movq	(%r11,%rdi,8), %r8
140ff0017: 4d 85 c0                    	testq	%r8, %r8
140ff001a: 0f 84 53 01 00 00           	je	0x140ff0173 <.text+0xfef173>
140ff0020: 49 f7 d0                    	notq	%r8
140ff0023: 4f 8d 04 40                 	leaq	(%r8,%r8,2), %r8
140ff0027: 49 c1 e0 05                 	shlq	$0x5, %r8
140ff002b: 4b 83 7c 01 20 00           	cmpq	$0x0, 0x20(%r9,%r8)
140ff0031: 0f 84 3c 01 00 00           	je	0x140ff0173 <.text+0xfef173>
140ff0037: 4d 01 ca                    	addq	%r9, %r10
140ff003a: 4d 8b 52 10                 	movq	0x10(%r10), %r10
140ff003e: 4c 01 ce                    	addq	%r9, %rsi
140ff0041: 4c 8b 5e 10                 	movq	0x10(%rsi), %r11
140ff0045: 4d 01 c8                    	addq	%r9, %r8
140ff0048: 4d 8b 48 10                 	movq	0x10(%r8), %r9
140ff004c: 8b 92 b0 05 00 00           	movl	0x5b0(%rdx), %edx
140ff0052: 41 0f b6 b2 f0 00 00 00     	movzbl	0xf0(%r10), %esi
140ff005a: 8d 56 ff                    	leal	-0x1(%rsi), %edx
140ff005d: 31 ff                       	xorl	%edi, %edi
140ff005f: 80 fa 02                    	cmpb	$0x2, %dl
140ff0062: 40 0f 92 c7                 	setb	%dil
140ff0066: 41 8b 14 bb                 	movl	(%r11,%rdi,4), %edx
140ff006a: f3 41 0f 10 8a e8 00 00 00  	movss	0xe8(%r10), %xmm1
140ff0073: f3 0f 10 05 e1 38 bd 01     	movss	0x1bd38e1(%rip), %xmm0  # 0x142bc395c
140ff007b: f3 0f 59 c1                 	mulss	%xmm1, %xmm0
140ff007f: f3 0f 58 05 ad cf 9d 01     	addss	0x19dcfad(%rip), %xmm0  # 0x1429cd034
140ff0087: 40 80 fe 01                 	cmpb	$0x1, %sil
140ff008b: 75 14                       	jne	0x140ff00a1 <.text+0xfef0a1>
140ff008d: f3 0f 59 0d cb 38 bd 01     	mulss	0x1bd38cb(%rip), %xmm1  # 0x142bc3960
140ff0095: f3 0f 58 c1                 	addss	%xmm1, %xmm0
140ff0099: f3 0f 58 05 c3 38 bd 01     	addss	0x1bd38c3(%rip), %xmm0  # 0x142bc3964
140ff00a1: 0f 57 c9                    	xorps	%xmm1, %xmm1
140ff00a4: f3 48 0f 2a ca              	cvtsi2ss	%rdx, %xmm1
140ff00a9: f3 0f 5e c8                 	divss	%xmm0, %xmm1
140ff00ad: f3 0f 5d 0d 4f 58 93 01     	minss	0x193584f(%rip), %xmm1  # 0x142925904
140ff00b5: 41 89 40 5c                 	movl	%eax, 0x5c(%r8)
140ff00b9: f3 41 0f 11 09              	movss	%xmm1, (%r9)
140ff00be: 89 81 58 01 00 00           	movl	%eax, 0x158(%rcx)
140ff00c4: 31 c0                       	xorl	%eax, %eax
140ff00c6: 48 81 c4 88 00 00 00        	addq	$0x88, %rsp
140ff00cd: 5f                          	popq	%rdi
140ff00ce: 5e                          	popq	%rsi
140ff00cf: c3                          	retq
140ff00d0: 48 81 c1 c0 00 00 00        	addq	$0xc0, %rcx
140ff00d7: 48 8d 05 45 3a bd 01        	leaq	0x1bd3a45(%rip), %rax   # 0x142bc3b23
140ff00de: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
140ff00e3: 48 c7 44 24 30 25 00 00 00  	movq	$0x25, 0x30(%rsp)
140ff00ec: eb 1c                       	jmp	0x140ff010a <.text+0xfef10a>
140ff00ee: 48 81 c1 c0 00 00 00        	addq	$0xc0, %rcx
140ff00f5: 48 8d 05 8c 39 bd 01        	leaq	0x1bd398c(%rip), %rax   # 0x142bc3a88
140ff00fc: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
140ff0101: 48 c7 44 24 30 2b 00 00 00  	movq	$0x2b, 0x30(%rsp)
140ff010a: 48 89 4c 24 38              	movq	%rcx, 0x38(%rsp)
140ff010f: 48 8d 05 ba b9 04 ff        	leaq	-0xfb4646(%rip), %rax   # 0x14003bad0 <.text+0x3aad0>
140ff0116: 48 89 44 24 40              	movq	%rax, 0x40(%rsp)
140ff011b: 48 8d 44 24 28              	leaq	0x28(%rsp), %rax
140ff0120: 48 89 44 24 48              	movq	%rax, 0x48(%rsp)
140ff0125: 48 8d 05 64 9d 11 ff        	leaq	-0xee629c(%rip), %rax   # 0x140109e90 <.text+0x108e90>
140ff012c: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
140ff0131: 48 8d 05 b0 38 bd 01        	leaq	0x1bd38b0(%rip), %rax   # 0x142bc39e8
140ff0138: 48 89 44 24 58              	movq	%rax, 0x58(%rsp)
140ff013d: 48 c7 44 24 60 02 00 00 00  	movq	$0x2, 0x60(%rsp)
140ff0146: 48 c7 44 24 78 00 00 00 00  	movq	$0x0, 0x78(%rsp)
140ff014f: 48 8d 44 24 38              	leaq	0x38(%rsp), %rax
140ff0154: 48 89 44 24 68              	movq	%rax, 0x68(%rsp)
140ff0159: 48 c7 44 24 70 02 00 00 00  	movq	$0x2, 0x70(%rsp)
140ff0162: 48 8d 15 07 39 bd 01        	leaq	0x1bd3907(%rip), %rdx   # 0x142bc3a70
140ff0169: 48 8d 4c 24 58              	leaq	0x58(%rsp), %rcx
140ff016e: e8 1d 92 8e 01              	callq	0x1428d9390 <.text+0x28d8390>
140ff0173: 48 81 c1 c0 00 00 00        	addq	$0xc0, %rcx
140ff017a: 48 8d 05 7a 39 bd 01        	leaq	0x1bd397a(%rip), %rax   # 0x142bc3afb
140ff0181: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
140ff0186: 48 c7 44 24 30 28 00 00 00  	movq	$0x28, 0x30(%rsp)
140ff018f: 48 89 4c 24 38              	movq	%rcx, 0x38(%rsp)
140ff0194: 48 8d 05 35 b9 04 ff        	leaq	-0xfb46cb(%rip), %rax   # 0x14003bad0 <.text+0x3aad0>
140ff019b: 48 89 44 24 40              	movq	%rax, 0x40(%rsp)
140ff01a0: 48 8d 44 24 28              	leaq	0x28(%rsp), %rax
140ff01a5: 48 89 44 24 48              	movq	%rax, 0x48(%rsp)
140ff01aa: 48 8d 05 df 9c 11 ff        	leaq	-0xee6321(%rip), %rax   # 0x140109e90 <.text+0x108e90>
140ff01b1: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
140ff01b6: 48 8d 05 2b 38 bd 01        	leaq	0x1bd382b(%rip), %rax   # 0x142bc39e8
140ff01bd: 48 89 44 24 58              	movq	%rax, 0x58(%rsp)
140ff01c2: 48 c7 44 24 60 02 00 00 00  	movq	$0x2, 0x60(%rsp)
140ff01cb: 48 c7 44 24 78 00 00 00 00  	movq	$0x0, 0x78(%rsp)
140ff01d4: 48 8d 44 24 38              	leaq	0x38(%rsp), %rax
140ff01d9: 48 89 44 24 68              	movq	%rax, 0x68(%rsp)
140ff01de: 48 c7 44 24 70 02 00 00 00  	movq	$0x2, 0x70(%rsp)
140ff01e7: 48 8d 15 9a 39 bd 01        	leaq	0x1bd399a(%rip), %rdx   # 0x142bc3b88
140ff01ee: 48 8d 4c 24 58              	leaq	0x58(%rsp), %rcx
140ff01f3: e8 98 91 8e 01              	callq	0x1428d9390 <.text+0x28d8390>
140ff01f8: 48 8d 0d 81 3a bd 01        	leaq	0x1bd3a81(%rip), %rcx   # 0x142bc3c80
140ff01ff: 4c 8d 05 da 3a bd 01        	leaq	0x1bd3ada(%rip), %r8    # 0x142bc3ce0
140ff0206: ba 59 00 00 00              	movl	$0x59, %edx
140ff020b: e8 20 91 8e 01              	callq	0x1428d9330 <.text+0x28d8330>
140ff0210: cc                          	int3
