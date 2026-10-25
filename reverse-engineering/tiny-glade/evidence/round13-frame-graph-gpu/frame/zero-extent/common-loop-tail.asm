
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1404de84d: 48 8b 85 a8 13 00 00        	movq	0x13a8(%rbp), %rax
1404de854: 80 38 00                    	cmpb	$0x0, (%rax)
1404de857: 75 6c                       	jne	0x1404de8c5 <.text+0x4dd8c5>
1404de859: 48 8b 8d 20 14 00 00        	movq	0x1420(%rbp), %rcx
1404de860: e8 0b 07 73 00              	callq	0x140c0ef70 <.text+0xc0df70>
1404de865: 48 83 bd 80 13 00 00 02     	cmpq	$0x2, 0x1380(%rbp)
1404de86d: 0f 93 c1                    	setae	%cl
1404de870: 08 c8                       	orb	%cl, %al
1404de872: 3c 01                       	cmpb	$0x1, %al
1404de874: 75 4f                       	jne	0x1404de8c5 <.text+0x4dd8c5>
1404de876: 48 8b 85 78 14 00 00        	movq	0x1478(%rbp), %rax
1404de87d: 48 8b 88 e8 10 00 00        	movq	0x10e8(%rax), %rcx
1404de884: 48 8b 41 10                 	movq	0x10(%rcx), %rax
1404de888: 48 8b 51 18                 	movq	0x18(%rcx), %rdx
1404de88c: f0                          	lock
1404de88d: 48 ff 00                    	incq	(%rax)
1404de890: 0f 8e c4 0d 00 00           	jle	0x1404df65a <.text+0x4de65a>
1404de896: 48 83 c1 20                 	addq	$0x20, %rcx
1404de89a: 48 89 95 38 0c 00 00        	movq	%rdx, 0xc38(%rbp)
1404de8a1: 48 89 85 30 0c 00 00        	movq	%rax, 0xc30(%rbp)
1404de8a8: c6 85 40 0c 00 00 01        	movb	$0x1, 0xc40(%rbp)
1404de8af: 48 8d 95 30 0c 00 00        	leaq	0xc30(%rbp), %rdx
1404de8b6: e8 f5 f6 05 00              	callq	0x14053dfb0 <.text+0x53cfb0>
1404de8bb: 48 8b 85 a8 13 00 00        	movq	0x13a8(%rbp), %rax
1404de8c2: c6 00 01                    	movb	$0x1, (%rax)
1404de8c5: 31 c0                       	xorl	%eax, %eax
1404de8c7: b1 01                       	movb	$0x1, %cl
1404de8c9: f0                          	lock
1404de8ca: 0f b0 0d c0 62 15 03        	cmpxchgb	%cl, 0x31562c0(%rip)    # 0x143634b91
1404de8d1: 0f 85 72 04 00 00           	jne	0x1404ded49 <.text+0x4ddd49>
1404de8d7: 48 8b 05 42 db 14 03        	movq	0x314db42(%rip), %rax   # 0x14362c420
1404de8de: 48 8b 00                    	movq	(%rax), %rax
1404de8e1: 48 d1 e0                    	shlq	%rax
1404de8e4: 48 85 c0                    	testq	%rax, %rax
1404de8e7: 0f 85 6d 04 00 00           	jne	0x1404ded5a <.text+0x4ddd5a>
1404de8ed: 31 c0                       	xorl	%eax, %eax
1404de8ef: 0f b6 0d 9c 62 15 03        	movzbl	0x315629c(%rip), %ecx   # 0x143634b92
1404de8f6: 84 c9                       	testb	%cl, %cl
1404de8f8: 0f 85 12 0c 00 00           	jne	0x1404df510 <.text+0x4de510>
1404de8fe: 0f b6 1d 8e 62 15 03        	movzbl	0x315628e(%rip), %ebx   # 0x143634b93
1404de905: c6 05 87 62 15 03 00        	movb	$0x0, 0x3156287(%rip)   # 0x143634b93
1404de90c: 84 c0                       	testb	%al, %al
1404de90e: 75 16                       	jne	0x1404de926 <.text+0x4dd926>
1404de910: 48 8b 05 09 db 14 03        	movq	0x314db09(%rip), %rax   # 0x14362c420
1404de917: 48 8b 00                    	movq	(%rax), %rax
1404de91a: 48 d1 e0                    	shlq	%rax
1404de91d: 48 85 c0                    	testq	%rax, %rax
1404de920: 0f 85 6f 04 00 00           	jne	0x1404ded95 <.text+0x4ddd95>
1404de926: 31 c0                       	xorl	%eax, %eax
1404de928: 86 05 63 62 15 03           	xchgb	%al, 0x3156263(%rip)    # 0x143634b91
1404de92e: 3c 02                       	cmpb	$0x2, %al
1404de930: 0f 84 3f 04 00 00           	je	0x1404ded75 <.text+0x4ddd75>
1404de936: 84 db                       	testb	%bl, %bl
1404de938: 0f 84 87 ae ff ff           	je	0x1404d97c5 <.text+0x4d87c5>
