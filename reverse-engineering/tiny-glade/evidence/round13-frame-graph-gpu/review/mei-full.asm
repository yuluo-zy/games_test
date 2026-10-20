
D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1409c6700: 55                          	pushq	%rbp
1409c6701: 41 57                       	pushq	%r15
1409c6703: 41 56                       	pushq	%r14
1409c6705: 41 55                       	pushq	%r13
1409c6707: 41 54                       	pushq	%r12
1409c6709: 56                          	pushq	%rsi
1409c670a: 57                          	pushq	%rdi
1409c670b: 53                          	pushq	%rbx
1409c670c: 48 81 ec c8 09 00 00        	subq	$0x9c8, %rsp            # imm = 0x9C8
1409c6713: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1409c671b: 48 c7 85 40 09 00 00 fe ff ff ff    	movq	$-0x2, 0x940(%rbp)
1409c6726: 45 89 cc                    	movl	%r9d, %r12d
1409c6729: 4c 89 85 30 09 00 00        	movq	%r8, 0x930(%rbp)
1409c6730: 48 89 d6                    	movq	%rdx, %rsi
1409c6733: 48 89 8d 20 09 00 00        	movq	%rcx, 0x920(%rbp)
1409c673a: e8 c1 02 25 00              	callq	0x140c16a00 <.text+0xc15a00>
1409c673f: 84 c0                       	testb	%al, %al
1409c6741: 88 85 3e 09 00 00           	movb	%al, 0x93e(%rbp)
1409c6747: 74 5f                       	je	0x1409c67a8 <.text+0x9c57a8>
1409c6749: 4c 8d 35 ab 92 0f 02        	leaq	0x20f92ab(%rip), %r14   # 0x142abf9fb
1409c6750: bf 45 00 00 00              	movl	$0x45, %edi
1409c6755: ba 45 00 00 00              	movl	$0x45, %edx
1409c675a: 4c 89 f1                    	movq	%r14, %rcx
1409c675d: e8 1e 7f ff ff              	callq	0x1409be680 <.text+0x9bd680>
1409c6762: 48 83 f8 01                 	cmpq	$0x1, %rax
1409c6766: 0f 85 b1 00 00 00           	jne	0x1409c681d <.text+0x9c581d>
1409c676c: 49 89 d1                    	movq	%rdx, %r9
1409c676f: 48 85 d2                    	testq	%rdx, %rdx
1409c6772: 74 3d                       	je	0x1409c67b1 <.text+0x9c57b1>
1409c6774: 49 83 f9 45                 	cmpq	$0x45, %r9
1409c6778: 73 35                       	jae	0x1409c67af <.text+0x9c57af>
1409c677a: 48 8d 05 7a 92 0f 02        	leaq	0x20f927a(%rip), %rax   # 0x142abf9fb
1409c6781: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
1409c6786: 7f 29                       	jg	0x1409c67b1 <.text+0x9c57b1>
1409c6788: 48 8d 05 81 93 0f 02        	leaq	0x20f9381(%rip), %rax   # 0x142abfb10
1409c678f: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1409c6794: 48 8d 0d 60 92 0f 02        	leaq	0x20f9260(%rip), %rcx   # 0x142abf9fb
1409c679b: ba 45 00 00 00              	movl	$0x45, %edx
1409c67a0: 45 31 c0                    	xorl	%r8d, %r8d
1409c67a3: e8 98 31 f1 01              	callq	0x1428d9940 <.text+0x28d8940>
1409c67a8: 31 c0                       	xorl	%eax, %eax
1409c67aa: e9 c9 01 00 00              	jmp	0x1409c6978 <.text+0x9c5978>
1409c67af: 75 d7                       	jne	0x1409c6788 <.text+0x9c5788>
1409c67b1: 4c 8d 35 43 92 0f 02        	leaq	0x20f9243(%rip), %r14   # 0x142abf9fb
1409c67b8: 4c 89 f1                    	movq	%r14, %rcx
1409c67bb: 4c 89 ca                    	movq	%r9, %rdx
1409c67be: e8 bd 7e ff ff              	callq	0x1409be680 <.text+0x9bd680>
1409c67c3: 48 83 f8 01                 	cmpq	$0x1, %rax
1409c67c7: 75 54                       	jne	0x1409c681d <.text+0x9c581d>
1409c67c9: 49 89 d0                    	movq	%rdx, %r8
1409c67cc: 49 83 c0 02                 	addq	$0x2, %r8
1409c67d0: 74 39                       	je	0x1409c680b <.text+0x9c580b>
1409c67d2: 49 83 f8 45                 	cmpq	$0x45, %r8
1409c67d6: 73 31                       	jae	0x1409c6809 <.text+0x9c5809>
1409c67d8: 48 8d 05 1c 92 0f 02        	leaq	0x20f921c(%rip), %rax   # 0x142abf9fb
1409c67df: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
1409c67e4: 7f 25                       	jg	0x1409c680b <.text+0x9c580b>
1409c67e6: 48 8d 05 3b 93 0f 02        	leaq	0x20f933b(%rip), %rax   # 0x142abfb28
1409c67ed: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1409c67f2: 48 8d 0d 02 92 0f 02        	leaq	0x20f9202(%rip), %rcx   # 0x142abf9fb
1409c67f9: ba 45 00 00 00              	movl	$0x45, %edx
1409c67fe: 41 b9 45 00 00 00           	movl	$0x45, %r9d
1409c6804: e8 37 31 f1 01              	callq	0x1428d9940 <.text+0x28d8940>
1409c6809: 75 db                       	jne	0x1409c67e6 <.text+0x9c57e6>
1409c680b: bf 43 00 00 00              	movl	$0x43, %edi
1409c6810: 48 29 d7                    	subq	%rdx, %rdi
1409c6813: 4c 8d 35 e1 91 0f 02        	leaq	0x20f91e1(%rip), %r14   # 0x142abf9fb
1409c681a: 4d 01 c6                    	addq	%r8, %r14
1409c681d: 4c 8d 05 10 97 0f 02        	leaq	0x20f9710(%rip), %r8    # 0x142abff34
1409c6824: 4c 8d 3d c4 96 0f 02        	leaq	0x20f96c4(%rip), %r15   # 0x142abfeef
1409c682b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
1409c6830: 41 0f be 40 ff              	movsbl	-0x1(%r8), %eax
1409c6835: 85 c0                       	testl	%eax, %eax
1409c6837: 78 17                       	js	0x1409c6850 <.text+0x9c5850>
1409c6839: 49 ff c8                    	decq	%r8
1409c683c: 83 f8 5c                    	cmpl	$0x5c, %eax
1409c683f: 75 63                       	jne	0x1409c68a4 <.text+0x9c58a4>
1409c6841: eb 72                       	jmp	0x1409c68b5 <.text+0x9c58b5>
1409c6843: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1409c6850: 41 0f b6 48 fe              	movzbl	-0x2(%r8), %ecx
1409c6855: 80 f9 c0                    	cmpb	$-0x40, %cl
1409c6858: 7d 23                       	jge	0x1409c687d <.text+0x9c587d>
1409c685a: 41 0f b6 50 fd              	movzbl	-0x3(%r8), %edx
1409c685f: 80 fa c0                    	cmpb	$-0x40, %dl
1409c6862: 7d 22                       	jge	0x1409c6886 <.text+0x9c5886>
1409c6864: 45 0f b6 48 fc              	movzbl	-0x4(%r8), %r9d
1409c6869: 49 83 c0 fc                 	addq	$-0x4, %r8
1409c686d: 41 83 e1 07                 	andl	$0x7, %r9d
1409c6871: 41 c1 e1 06                 	shll	$0x6, %r9d
1409c6875: 83 e2 3f                    	andl	$0x3f, %edx
1409c6878: 44 09 ca                    	orl	%r9d, %edx
1409c687b: eb 10                       	jmp	0x1409c688d <.text+0x9c588d>
1409c687d: 49 83 c0 fe                 	addq	$-0x2, %r8
1409c6881: 83 e1 1f                    	andl	$0x1f, %ecx
1409c6884: eb 0f                       	jmp	0x1409c6895 <.text+0x9c5895>
1409c6886: 49 83 c0 fd                 	addq	$-0x3, %r8
1409c688a: 83 e2 0f                    	andl	$0xf, %edx
1409c688d: c1 e2 06                    	shll	$0x6, %edx
1409c6890: 83 e1 3f                    	andl	$0x3f, %ecx
1409c6893: 09 d1                       	orl	%edx, %ecx
1409c6895: c1 e1 06                    	shll	$0x6, %ecx
1409c6898: 24 3f                       	andb	$0x3f, %al
1409c689a: 0f b6 c0                    	movzbl	%al, %eax
1409c689d: 09 c8                       	orl	%ecx, %eax
1409c689f: 83 f8 5c                    	cmpl	$0x5c, %eax
1409c68a2: 74 11                       	je	0x1409c68b5 <.text+0x9c58b5>
1409c68a4: 83 f8 2f                    	cmpl	$0x2f, %eax
1409c68a7: 74 0c                       	je	0x1409c68b5 <.text+0x9c58b5>
1409c68a9: 4d 39 f8                    	cmpq	%r15, %r8
1409c68ac: 75 82                       	jne	0x1409c6830 <.text+0x9c5830>
1409c68ae: bb 45 00 00 00              	movl	$0x45, %ebx
1409c68b3: eb 4c                       	jmp	0x1409c6901 <.text+0x9c5901>
1409c68b5: 4c 8d 3d 33 96 0f 02        	leaq	0x20f9633(%rip), %r15   # 0x142abfeef
1409c68bc: 4d 29 f8                    	subq	%r15, %r8
1409c68bf: 49 ff c0                    	incq	%r8
1409c68c2: 74 32                       	je	0x1409c68f6 <.text+0x9c58f6>
1409c68c4: 49 83 f8 45                 	cmpq	$0x45, %r8
1409c68c8: 73 2a                       	jae	0x1409c68f4 <.text+0x9c58f4>
1409c68ca: 43 80 3c 38 bf              	cmpb	$-0x41, (%r8,%r15)
1409c68cf: 7f 25                       	jg	0x1409c68f6 <.text+0x9c58f6>
1409c68d1: 48 8d 05 18 92 0f 02        	leaq	0x20f9218(%rip), %rax   # 0x142abfaf0
1409c68d8: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1409c68dd: 48 8d 0d 0b 96 0f 02        	leaq	0x20f960b(%rip), %rcx   # 0x142abfeef
1409c68e4: ba 45 00 00 00              	movl	$0x45, %edx
1409c68e9: 41 b9 45 00 00 00           	movl	$0x45, %r9d
1409c68ef: e8 4c 30 f1 01              	callq	0x1428d9940 <.text+0x28d8940>
1409c68f4: 75 db                       	jne	0x1409c68d1 <.text+0x9c58d1>
1409c68f6: bb 45 00 00 00              	movl	$0x45, %ebx
1409c68fb: 4c 29 c3                    	subq	%r8, %rbx
1409c68fe: 4d 01 c7                    	addq	%r8, %r15
1409c6901: e8 2a 18 25 00              	callq	0x140c18130 <.text+0xc17130>
1409c6906: 48 89 c1                    	movq	%rax, %rcx
1409c6909: 48 8b 00                    	movq	(%rax), %rax
1409c690c: 48 83 f8 01                 	cmpq	$0x1, %rax
1409c6910: 0f 85 a0 07 00 00           	jne	0x1409c70b6 <.text+0x9c60b6>
1409c6916: 48 83 c1 08                 	addq	$0x8, %rcx
1409c691a: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1409c691e: 0f 85 cc 07 00 00           	jne	0x1409c70f0 <.text+0x9c60f0>
1409c6924: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1409c692b: 48 89 8d 28 09 00 00        	movq	%rcx, 0x928(%rbp)
1409c6932: 48 83 c1 08                 	addq	$0x8, %rcx
1409c6936: 48 89 5c 24 20              	movq	%rbx, 0x20(%rsp)
1409c693b: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
1409c6944: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
1409c694d: 4c 89 f2                    	movq	%r14, %rdx
1409c6950: 49 89 f8                    	movq	%rdi, %r8
1409c6953: 4d 89 f9                    	movq	%r15, %r9
1409c6956: e8 e5 03 25 00              	callq	0x140c16d40 <.text+0xc15d40>
1409c695b: 48 8b 8d 28 09 00 00        	movq	0x928(%rbp), %rcx
1409c6962: 48 ff 01                    	incq	(%rcx)
1409c6965: 48 89 85 f0 08 00 00        	movq	%rax, 0x8f0(%rbp)
1409c696c: 48 89 85 d0 08 00 00        	movq	%rax, 0x8d0(%rbp)
1409c6973: b8 01 00 00 00              	movl	$0x1, %eax
1409c6978: 48 89 85 c8 08 00 00        	movq	%rax, 0x8c8(%rbp)
1409c697f: 4c 8d 7e 38                 	leaq	0x38(%rsi), %r15
1409c6983: 48 ba b3 61 b4 27 07 ee 97 85       	movabsq	$-0x7a6811f8d84b9e4d, %rdx # imm = 0x8597EE0727B461B3
1409c698d: 49 b8 5c f1 15 ae c5 7e 29 8b       	movabsq	$-0x74d6813a51ea0ea4, %r8 # imm = 0x8B297EC5AE15F15C
1409c6997: 4c 89 f9                    	movq	%r15, %rcx
1409c699a: e8 51 6d 71 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
1409c699f: 48 85 c0                    	testq	%rax, %rax
1409c69a2: 0f 84 54 07 00 00           	je	0x1409c70fc <.text+0x9c60fc>
1409c69a8: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
1409c69af: 0f 86 47 07 00 00           	jbe	0x1409c70fc <.text+0x9c60fc>
1409c69b5: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
1409c69bc: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
1409c69c0: 48 85 c9                    	testq	%rcx, %rcx
1409c69c3: 0f 84 33 07 00 00           	je	0x1409c70fc <.text+0x9c60fc>
1409c69c9: 48 8b 86 e0 01 00 00        	movq	0x1e0(%rsi), %rax
1409c69d0: 48 f7 d1                    	notq	%rcx
1409c69d3: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1409c69d7: 48 c1 e1 05                 	shlq	$0x5, %rcx
1409c69db: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
1409c69e1: 0f 84 15 07 00 00           	je	0x1409c70fc <.text+0x9c60fc>
1409c69e7: 48 01 c8                    	addq	%rcx, %rax
1409c69ea: 48 8b 48 10                 	movq	0x10(%rax), %rcx
1409c69ee: 48 8d 15 33 96 0f 02        	leaq	0x20f9633(%rip), %rdx   # 0x142ac0028
1409c69f5: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1409c69fb: e8 a0 10 f7 ff              	callq	0x140937aa0 <.text+0x936aa0>
1409c6a00: a8 01                       	testb	$0x1, %al
1409c6a02: 0f 84 33 07 00 00           	je	0x1409c713b <.text+0x9c613b>
1409c6a08: 49 89 d6                    	movq	%rdx, %r14
1409c6a0b: 48 ba a6 12 23 8c a9 dc 4d 86       	movabsq	$-0x79b2235673dced5a, %rdx # imm = 0x864DDCA98C2312A6
1409c6a15: 49 b8 4e 48 9a 9e 36 e9 b0 27       	movabsq	$0x27b0e9369e9a484e, %r8 # imm = 0x27B0E9369E9A484E
1409c6a1f: 4c 89 f9                    	movq	%r15, %rcx
1409c6a22: e8 c9 6c 71 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
1409c6a27: 48 85 c0                    	testq	%rax, %rax
1409c6a2a: 0f 84 da 06 00 00           	je	0x1409c710a <.text+0x9c610a>
1409c6a30: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
1409c6a37: 0f 86 cd 06 00 00           	jbe	0x1409c710a <.text+0x9c610a>
1409c6a3d: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
1409c6a44: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
1409c6a48: 48 85 c9                    	testq	%rcx, %rcx
1409c6a4b: 0f 84 b9 06 00 00           	je	0x1409c710a <.text+0x9c610a>
1409c6a51: 48 8b 86 e0 01 00 00        	movq	0x1e0(%rsi), %rax
1409c6a58: 48 f7 d1                    	notq	%rcx
1409c6a5b: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1409c6a5f: 48 c1 e1 05                 	shlq	$0x5, %rcx
1409c6a63: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
1409c6a69: 0f 84 9b 06 00 00           	je	0x1409c710a <.text+0x9c610a>
1409c6a6f: 48 01 c8                    	addq	%rcx, %rax
1409c6a72: 48 8b 40 10                 	movq	0x10(%rax), %rax
1409c6a76: 8b 08                       	movl	(%rax), %ecx
1409c6a78: 48 89 8d e8 08 00 00        	movq	%rcx, 0x8e8(%rbp)
1409c6a7f: 8b 40 04                    	movl	0x4(%rax), %eax
1409c6a82: 48 89 85 28 09 00 00        	movq	%rax, 0x928(%rbp)
1409c6a89: 48 ba dc 69 1a 85 8b 25 8c 95       	movabsq	$-0x6a73da747ae59624, %rdx # imm = 0x958C258B851A69DC
1409c6a93: 49 b8 e8 2b 71 61 cb bc be 47       	movabsq	$0x47bebccb61712be8, %r8 # imm = 0x47BEBCCB61712BE8
1409c6a9d: 4c 89 f9                    	movq	%r15, %rcx
1409c6aa0: e8 4b 6c 71 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
1409c6aa5: 48 85 c0                    	testq	%rax, %rax
1409c6aa8: 0f 84 6a 06 00 00           	je	0x1409c7118 <.text+0x9c6118>
1409c6aae: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
1409c6ab5: 0f 86 5d 06 00 00           	jbe	0x1409c7118 <.text+0x9c6118>
1409c6abb: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
1409c6ac2: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
1409c6ac6: 48 85 c9                    	testq	%rcx, %rcx
1409c6ac9: 0f 84 49 06 00 00           	je	0x1409c7118 <.text+0x9c6118>
1409c6acf: 48 8b 86 e0 01 00 00        	movq	0x1e0(%rsi), %rax
1409c6ad6: 48 f7 d1                    	notq	%rcx
1409c6ad9: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1409c6add: 48 c1 e1 05                 	shlq	$0x5, %rcx
1409c6ae1: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
1409c6ae7: 0f 84 2b 06 00 00           	je	0x1409c7118 <.text+0x9c6118>
1409c6aed: 48 01 c8                    	addq	%rcx, %rax
1409c6af0: 48 8b 48 10                 	movq	0x10(%rax), %rcx
1409c6af4: e8 77 8b 1c 00              	callq	0x140b8f670 <.text+0xb8e670>
1409c6af9: 48 89 c7                    	movq	%rax, %rdi
1409c6afc: 49 89 d5                    	movq	%rdx, %r13
1409c6aff: 45 84 e4                    	testb	%r12b, %r12b
1409c6b02: 0f 84 c0 00 00 00           	je	0x1409c6bc8 <.text+0x9c5bc8>
1409c6b08: 48 8b 85 30 09 00 00        	movq	0x930(%rbp), %rax
1409c6b0f: 48 8d 48 04                 	leaq	0x4(%rax), %rcx
1409c6b13: e8 78 ec 20 00              	callq	0x140bd5790 <.text+0xbd4790>
1409c6b18: 48 8d 8d b0 07 00 00        	leaq	0x7b0(%rbp), %rcx
1409c6b1f: 48 89 c2                    	movq	%rax, %rdx
1409c6b22: 41 b8 2b 00 00 00           	movl	$0x2b, %r8d
1409c6b28: e8 73 e7 20 00              	callq	0x140bd52a0 <.text+0xbd42a0>
1409c6b2d: 0f 10 85 b0 07 00 00        	movups	0x7b0(%rbp), %xmm0
1409c6b34: 0f 10 8d c0 07 00 00        	movups	0x7c0(%rbp), %xmm1
1409c6b3b: 0f 29 4d e0                 	movaps	%xmm1, -0x20(%rbp)
1409c6b3f: 0f 29 45 d0                 	movaps	%xmm0, -0x30(%rbp)
1409c6b43: 48 8d 8d d0 03 00 00        	leaq	0x3d0(%rbp), %rcx
1409c6b4a: 48 8d 55 d0                 	leaq	-0x30(%rbp), %rdx
1409c6b4e: e8 8d 01 21 00              	callq	0x140bd6ce0 <.text+0xbd5ce0>
1409c6b53: 48 8d 05 1e 95 0f 02        	leaq	0x20f951e(%rip), %rax   # 0x142ac0078
1409c6b5a: 48 89 85 c0 03 00 00        	movq	%rax, 0x3c0(%rbp)
1409c6b61: 48 c7 85 c8 03 00 00 08 00 00 00    	movq	$0x8, 0x3c8(%rbp)
1409c6b6c: 48 8d 95 c0 03 00 00        	leaq	0x3c0(%rbp), %rdx
1409c6b73: 48 8b 8d 20 09 00 00        	movq	0x920(%rbp), %rcx
1409c6b7a: e8 51 29 22 00              	callq	0x140be94d0 <.text+0xbe84d0>
1409c6b7f: 0f 10 85 b0 07 00 00        	movups	0x7b0(%rbp), %xmm0
1409c6b86: 0f 10 8d c0 07 00 00        	movups	0x7c0(%rbp), %xmm1
1409c6b8d: 0f 11 8d 10 09 00 00        	movups	%xmm1, 0x910(%rbp)
1409c6b94: 0f 11 85 00 09 00 00        	movups	%xmm0, 0x900(%rbp)
1409c6b9b: 89 85 fc 08 00 00           	movl	%eax, 0x8fc(%rbp)
1409c6ba1: 45 31 e4                    	xorl	%r12d, %r12d
1409c6ba4: 31 db                       	xorl	%ebx, %ebx
1409c6ba6: 80 bd 1e 09 00 00 07        	cmpb	$0x7, 0x91e(%rbp)
1409c6bad: 48 8b 85 30 09 00 00        	movq	0x930(%rbp), %rax
1409c6bb4: 4c 0f 44 e0                 	cmoveq	%rax, %r12
1409c6bb8: 0f 95 c3                    	setne	%bl
1409c6bbb: 48 8d 8d fc 08 00 00        	leaq	0x8fc(%rbp), %rcx
1409c6bc2: 48 0f 44 c8                 	cmoveq	%rax, %rcx
1409c6bc6: eb 13                       	jmp	0x1409c6bdb <.text+0x9c5bdb>
1409c6bc8: c6 85 1e 09 00 00 07        	movb	$0x7, 0x91e(%rbp)
1409c6bcf: 31 db                       	xorl	%ebx, %ebx
1409c6bd1: 48 8b 8d 30 09 00 00        	movq	0x930(%rbp), %rcx
1409c6bd8: 49 89 cc                    	movq	%rcx, %r12
1409c6bdb: 48 83 c1 04                 	addq	$0x4, %rcx
1409c6bdf: e8 ac eb 20 00              	callq	0x140bd5790 <.text+0xbd4790>
1409c6be4: 89 9d c0 03 00 00           	movl	%ebx, 0x3c0(%rbp)
1409c6bea: c7 85 c4 03 00 00 00 00 00 00       	movl	$0x0, 0x3c4(%rbp)
1409c6bf4: 4c 89 a5 c8 03 00 00        	movq	%r12, 0x3c8(%rbp)
1409c6bfb: 48 c7 85 d0 03 00 00 00 00 00 00    	movq	$0x0, 0x3d0(%rbp)
1409c6c06: 48 8d 8d fc 08 00 00        	leaq	0x8fc(%rbp), %rcx
1409c6c0d: 48 89 8d d8 03 00 00        	movq	%rcx, 0x3d8(%rbp)
1409c6c14: c7 85 b0 07 00 00 02 00 00 00       	movl	$0x2, 0x7b0(%rbp)
1409c6c1e: 48 8b 8d 20 09 00 00        	movq	0x920(%rbp), %rcx
1409c6c25: 48 89 4c 24 40              	movq	%rcx, 0x40(%rsp)
1409c6c2a: 48 89 74 24 38              	movq	%rsi, 0x38(%rsp)
1409c6c2f: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
1409c6c34: 48 8d 85 b0 07 00 00        	leaq	0x7b0(%rbp), %rax
1409c6c3b: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
1409c6c40: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1409c6c49: 48 8d 15 30 94 0f 02        	leaq	0x20f9430(%rip), %rdx   # 0x142ac0080
1409c6c50: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
1409c6c54: 4c 8d 8d c0 03 00 00        	leaq	0x3c0(%rbp), %r9
1409c6c5b: 41 b8 06 00 00 00           	movl	$0x6, %r8d
1409c6c61: e8 0a 0d ea ff              	callq	0x140867970 <.text+0x866970>
1409c6c66: c6 85 3f 09 00 00 01        	movb	$0x1, 0x93f(%rbp)
1409c6c6d: e8 de fa 21 00              	callq	0x140be6750 <.text+0xbe5750>
1409c6c72: 41 89 c4                    	movl	%eax, %r12d
1409c6c75: c6 85 3f 09 00 00 01        	movb	$0x1, 0x93f(%rbp)
1409c6c7c: 31 c9                       	xorl	%ecx, %ecx
1409c6c7e: e8 9d 2e 71 ff              	callq	0x1400d9b20 <.text+0xd8b20>
1409c6c83: 48 85 c0                    	testq	%rax, %rax
1409c6c86: 0f 84 bd 04 00 00           	je	0x1409c7149 <.text+0x9c6149>
1409c6c8c: 44 89 a5 58 08 00 00        	movl	%r12d, 0x858(%rbp)
1409c6c93: 66 c7 85 60 08 00 00 00 00  	movw	$0x0, 0x860(%rbp)
1409c6c9c: c7 85 5c 08 00 00 00 00 80 3f       	movl	$0x3f800000, 0x85c(%rbp) # imm = 0x3F800000
1409c6ca6: 48 c7 85 b0 07 00 00 00 00 00 00    	movq	$0x0, 0x7b0(%rbp)
1409c6cb1: 4c 89 b5 50 08 00 00        	movq	%r14, 0x850(%rbp)
1409c6cb8: 48 c7 85 c0 07 00 00 00 00 00 00    	movq	$0x0, 0x7c0(%rbp)
1409c6cc3: 48 c7 85 c8 07 00 00 04 00 00 00    	movq	$0x4, 0x7c8(%rbp)
1409c6cce: 48 c7 85 d0 07 00 00 00 00 00 00    	movq	$0x0, 0x7d0(%rbp)
1409c6cd9: 0f 10 00                    	movups	(%rax), %xmm0
1409c6cdc: 48 ff 00                    	incq	(%rax)
1409c6cdf: 0f 10 0d 4a fa 0c 02        	movups	0x20cfa4a(%rip), %xmm1  # 0x142a96730
1409c6ce6: 0f 11 8d 30 08 00 00        	movups	%xmm1, 0x830(%rbp)
1409c6ced: 0f 10 0d 2c fa 0c 02        	movups	0x20cfa2c(%rip), %xmm1  # 0x142a96720
1409c6cf4: 0f 11 8d 20 08 00 00        	movups	%xmm1, 0x820(%rbp)
1409c6cfb: 0f 11 85 40 08 00 00        	movups	%xmm0, 0x840(%rbp)
1409c6d02: 48 b8 03 00 00 00 01 00 00 00       	movabsq	$0x100000003, %rax # imm = 0x100000003
1409c6d0c: 48 89 85 f0 07 00 00        	movq	%rax, 0x7f0(%rbp)
1409c6d13: 48 c7 85 d8 07 00 00 00 00 00 00    	movq	$0x0, 0x7d8(%rbp)
1409c6d1e: 48 c7 85 e0 07 00 00 01 00 00 00    	movq	$0x1, 0x7e0(%rbp)
1409c6d29: 48 c7 85 e8 07 00 00 00 00 00 00    	movq	$0x0, 0x7e8(%rbp)
1409c6d34: c6 85 3f 09 00 00 01        	movb	$0x1, 0x93f(%rbp)
1409c6d3b: 48 8d 8d c0 03 00 00        	leaq	0x3c0(%rbp), %rcx
1409c6d42: 48 8d 95 b0 07 00 00        	leaq	0x7b0(%rbp), %rdx
1409c6d49: 4c 8d 45 d0                 	leaq	-0x30(%rbp), %r8
1409c6d4d: 45 31 c9                    	xorl	%r9d, %r9d
1409c6d50: e8 8b ee f3 ff              	callq	0x140905be0 <.text+0x904be0>
1409c6d55: 80 bd 9e 04 00 00 08        	cmpb	$0x8, 0x49e(%rbp)
1409c6d5c: 74 3d                       	je	0x1409c6d9b <.text+0x9c5d9b>
1409c6d5e: f3 48 0f 2a 9d e8 08 00 00  	cvtsi2ssq	0x8e8(%rbp), %xmm3
1409c6d67: 0f 57 c0                    	xorps	%xmm0, %xmm0
1409c6d6a: f3 48 0f 2a 85 28 09 00 00  	cvtsi2ssq	0x928(%rbp), %xmm0
1409c6d73: 48 8d 8d 78 04 00 00        	leaq	0x478(%rbp), %rcx
1409c6d7a: ba e8 00 00 00              	movl	$0xe8, %edx
1409c6d7f: 48 03 95 a8 04 00 00        	addq	0x4a8(%rbp), %rdx
1409c6d86: f3 0f 11 44 24 20           	movss	%xmm0, 0x20(%rsp)
1409c6d8c: 49 b8 59 05 86 b7 25 06 5a 0a       	movabsq	$0xa5a0625b7860559, %r8 # imm = 0xA5A0625B7860559
1409c6d96: e8 a5 4b fa ff              	callq	0x14096b940 <.text+0x96a940>
1409c6d9b: 4c 8d b5 b0 07 00 00        	leaq	0x7b0(%rbp), %r14
1409c6da2: 4c 8d a5 c0 03 00 00        	leaq	0x3c0(%rbp), %r12
1409c6da9: 41 b8 f8 00 00 00           	movl	$0xf8, %r8d
1409c6daf: 4c 89 f1                    	movq	%r14, %rcx
1409c6db2: 4c 89 e2                    	movq	%r12, %rdx
1409c6db5: e8 46 ce f5 01              	callq	0x142923c00 <.text+0x2922c00>
1409c6dba: 48 b8 65 18 a0 91 ab 39 9f 57       	movabsq	$0x579f39ab91a01865, %rax # imm = 0x579F39AB91A01865
1409c6dc4: 48 89 85 a8 08 00 00        	movq	%rax, 0x8a8(%rbp)
1409c6dcb: c7 85 c0 03 00 00 02 00 00 00       	movl	$0x2, 0x3c0(%rbp)
1409c6dd5: 4c 89 64 24 20              	movq	%r12, 0x20(%rsp)
1409c6dda: 48 8d 8d a8 08 00 00        	leaq	0x8a8(%rbp), %rcx
1409c6de1: 49 b8 8c 97 66 69 03 f5 5b 81       	movabsq	$-0x7ea40afc96996874, %r8 # imm = 0x815BF5036966978C
1409c6deb: 4c 89 f2                    	movq	%r14, %rdx
1409c6dee: 41 b1 1d                    	movb	$0x1d, %r9b
1409c6df1: e8 1a 18 f4 ff              	callq	0x140908610 <.text+0x907610>
1409c6df6: 4c 8d b5 c0 03 00 00        	leaq	0x3c0(%rbp), %r14
1409c6dfd: 4c 8d a5 b0 07 00 00        	leaq	0x7b0(%rbp), %r12
1409c6e04: 41 b8 f8 00 00 00           	movl	$0xf8, %r8d
1409c6e0a: 4c 89 f1                    	movq	%r14, %rcx
1409c6e0d: 4c 89 e2                    	movq	%r12, %rdx
1409c6e10: e8 eb cd f5 01              	callq	0x142923c00 <.text+0x2922c00>
1409c6e15: 48 89 bd a8 08 00 00        	movq	%rdi, 0x8a8(%rbp)
1409c6e1c: c7 85 b0 07 00 00 02 00 00 00       	movl	$0x2, 0x7b0(%rbp)
1409c6e26: 4c 89 64 24 20              	movq	%r12, 0x20(%rsp)
1409c6e2b: 48 8d bd a8 08 00 00        	leaq	0x8a8(%rbp), %rdi
1409c6e32: 49 b8 52 33 56 24 f4 91 39 75       	movabsq	$0x753991f424563352, %r8 # imm = 0x753991F424563352
1409c6e3c: 48 89 f9                    	movq	%rdi, %rcx
1409c6e3f: 4c 89 f2                    	movq	%r14, %rdx
1409c6e42: 41 b1 1d                    	movb	$0x1d, %r9b
1409c6e45: e8 76 68 f3 ff              	callq	0x1408fd6c0 <.text+0x8fc6c0>
1409c6e4a: 4c 8d b5 b0 07 00 00        	leaq	0x7b0(%rbp), %r14
1409c6e51: 48 8d 95 c0 03 00 00        	leaq	0x3c0(%rbp), %rdx
1409c6e58: 41 b8 f8 00 00 00           	movl	$0xf8, %r8d
1409c6e5e: 4c 89 f1                    	movq	%r14, %rcx
1409c6e61: e8 9a cd f5 01              	callq	0x142923c00 <.text+0x2922c00>
1409c6e66: c6 85 3f 09 00 00 01        	movb	$0x1, 0x93f(%rbp)
1409c6e6d: 48 8d 8d c0 03 00 00        	leaq	0x3c0(%rbp), %rcx
1409c6e74: 49 b8 bf 02 8b bc 7c be be d7       	movabsq	$-0x284141834374fd41, %r8 # imm = 0xD7BEBE7CBC8B02BF
1409c6e7e: 4c 89 f2                    	movq	%r14, %rdx
1409c6e81: e8 0a 57 c6 00              	callq	0x14162c590 <.text+0x162b590>
1409c6e86: c6 85 3f 09 00 00 01        	movb	$0x1, 0x93f(%rbp)
1409c6e8d: 48 8d 8d b0 07 00 00        	leaq	0x7b0(%rbp), %rcx
1409c6e94: 48 8d 95 c0 03 00 00        	leaq	0x3c0(%rbp), %rdx
1409c6e9b: 49 b8 b3 33 33 c2 af 62 2f 91       	movabsq	$-0x6ed09d503dcccc4d, %r8 # imm = 0x912F62AFC23333B3
1409c6ea5: e8 e6 2f c6 00              	callq	0x141629e90 <.text+0x1628e90>
1409c6eaa: 80 a5 58 08 00 00 7f        	andb	$0x7f, 0x858(%rbp)
1409c6eb1: 0f b6 85 5b 08 00 00        	movzbl	0x85b(%rbp), %eax
1409c6eb8: 24 9f                       	andb	$-0x61, %al
1409c6eba: 0c 40                       	orb	$0x40, %al
1409c6ebc: 88 85 5b 08 00 00           	movb	%al, 0x85b(%rbp)
1409c6ec2: 4c 8d b5 c0 03 00 00        	leaq	0x3c0(%rbp), %r14
1409c6ec9: 48 8d 95 b0 07 00 00        	leaq	0x7b0(%rbp), %rdx
1409c6ed0: 41 b8 f8 00 00 00           	movl	$0xf8, %r8d
1409c6ed6: 4c 89 f1                    	movq	%r14, %rcx
1409c6ed9: e8 22 cd f5 01              	callq	0x142923c00 <.text+0x2922c00>
1409c6ede: 4d 01 ed                    	addq	%r13, %r13
1409c6ee1: 4a 8d 14 6d 00 00 00 00     	leaq	(,%r13,2), %rdx
1409c6ee9: 4c 01 ea                    	addq	%r13, %rdx
1409c6eec: c6 85 3f 09 00 00 01        	movb	$0x1, 0x93f(%rbp)
1409c6ef3: 4c 89 f1                    	movq	%r14, %rcx
1409c6ef6: e8 95 fb f3 ff              	callq	0x140906a90 <.text+0x905a90>
1409c6efb: 48 8d 55 d0                 	leaq	-0x30(%rbp), %rdx
1409c6eff: 41 b8 e8 03 00 00           	movl	$0x3e8, %r8d            # imm = 0x3E8
1409c6f05: 4c 89 f1                    	movq	%r14, %rcx
1409c6f08: e8 f3 cc f5 01              	callq	0x142923c00 <.text+0x2922c00>
1409c6f0d: c6 85 3f 09 00 00 00        	movb	$0x0, 0x93f(%rbp)
1409c6f14: 4c 89 f1                    	movq	%r14, %rcx
1409c6f17: e8 54 14 ea ff              	callq	0x140868370 <.text+0x867370>
1409c6f1c: 80 bd 1e 09 00 00 07        	cmpb	$0x7, 0x91e(%rbp)
1409c6f23: 0f 84 2d 01 00 00           	je	0x1409c7056 <.text+0x9c6056>
1409c6f29: 8b 85 1c 09 00 00           	movl	0x91c(%rbp), %eax
1409c6f2f: 89 85 d0 07 00 00           	movl	%eax, 0x7d0(%rbp)
1409c6f35: 0f 10 85 fc 08 00 00        	movups	0x8fc(%rbp), %xmm0
1409c6f3c: 0f 10 8d 0c 09 00 00        	movups	0x90c(%rbp), %xmm1
1409c6f43: 0f 29 8d c0 07 00 00        	movaps	%xmm1, 0x7c0(%rbp)
1409c6f4a: 0f 29 85 b0 07 00 00        	movaps	%xmm0, 0x7b0(%rbp)
1409c6f51: c6 85 3f 09 00 00 00        	movb	$0x0, 0x93f(%rbp)
1409c6f58: 48 ba b3 61 b4 27 07 ee 97 85       	movabsq	$-0x7a6811f8d84b9e4d, %rdx # imm = 0x8597EE0727B461B3
1409c6f62: 49 b8 5c f1 15 ae c5 7e 29 8b       	movabsq	$-0x74d6813a51ea0ea4, %r8 # imm = 0x8B297EC5AE15F15C
1409c6f6c: 4c 89 f9                    	movq	%r15, %rcx
1409c6f6f: e8 7c 67 71 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
1409c6f74: 48 85 c0                    	testq	%rax, %rax
1409c6f77: 0f 84 a9 01 00 00           	je	0x1409c7126 <.text+0x9c6126>
1409c6f7d: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
1409c6f84: 0f 86 9c 01 00 00           	jbe	0x1409c7126 <.text+0x9c6126>
1409c6f8a: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
1409c6f91: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
1409c6f95: 48 85 c9                    	testq	%rcx, %rcx
1409c6f98: 0f 84 88 01 00 00           	je	0x1409c7126 <.text+0x9c6126>
1409c6f9e: 48 8b 86 e0 01 00 00        	movq	0x1e0(%rsi), %rax
1409c6fa5: 48 f7 d1                    	notq	%rcx
1409c6fa8: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1409c6fac: 48 c1 e1 05                 	shlq	$0x5, %rcx
1409c6fb0: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
1409c6fb6: 0f 84 6a 01 00 00           	je	0x1409c7126 <.text+0x9c6126>
1409c6fbc: 48 01 c8                    	addq	%rcx, %rax
1409c6fbf: 48 8b 48 10                 	movq	0x10(%rax), %rcx
1409c6fc3: c6 85 3f 09 00 00 00        	movb	$0x0, 0x93f(%rbp)
1409c6fca: 48 8d 15 cf 90 0f 02        	leaq	0x20f90cf(%rip), %rdx   # 0x142ac00a0
1409c6fd1: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
1409c6fd7: e8 c4 0a f7 ff              	callq	0x140937aa0 <.text+0x936aa0>
1409c6fdc: a8 01                       	testb	$0x1, %al
1409c6fde: 0f 84 7a 01 00 00           	je	0x1409c715e <.text+0x9c615e>
1409c6fe4: c7 85 a8 08 00 00 00 00 00 00       	movl	$0x0, 0x8a8(%rbp)
1409c6fee: 48 8b 85 30 09 00 00        	movq	0x930(%rbp), %rax
1409c6ff5: 48 89 85 b0 08 00 00        	movq	%rax, 0x8b0(%rbp)
1409c6ffc: 8b 85 d0 07 00 00           	movl	0x7d0(%rbp), %eax
1409c7002: 89 85 e0 03 00 00           	movl	%eax, 0x3e0(%rbp)
1409c7008: 0f 28 85 b0 07 00 00        	movaps	0x7b0(%rbp), %xmm0
1409c700f: 0f 28 8d c0 07 00 00        	movaps	0x7c0(%rbp), %xmm1
1409c7016: 0f 29 8d d0 03 00 00        	movaps	%xmm1, 0x3d0(%rbp)
1409c701d: 0f 29 85 c0 03 00 00        	movaps	%xmm0, 0x3c0(%rbp)
1409c7024: c6 85 3f 09 00 00 00        	movb	$0x0, 0x93f(%rbp)
1409c702b: 4c 89 74 24 30              	movq	%r14, 0x30(%rsp)
1409c7030: 48 89 7c 24 28              	movq	%rdi, 0x28(%rsp)
1409c7035: 48 89 54 24 20              	movq	%rdx, 0x20(%rsp)
1409c703a: 4c 8d 05 87 90 0f 02        	leaq	0x20f9087(%rip), %r8    # 0x142ac00c8
1409c7041: 41 b9 11 00 00 00           	movl	$0x11, %r9d
1409c7047: 48 8b 8d 20 09 00 00        	movq	0x920(%rbp), %rcx
1409c704e: 48 89 f2                    	movq	%rsi, %rdx
1409c7051: e8 6a 6e 00 00              	callq	0x1409cdec0 <.text+0x9ccec0>
1409c7056: 80 bd 3e 09 00 00 00        	cmpb	$0x0, 0x93e(%rbp)
1409c705d: 74 43                       	je	0x1409c70a2 <.text+0x9c60a2>
1409c705f: e8 cc 10 25 00              	callq	0x140c18130 <.text+0xc17130>
1409c7064: 48 89 c1                    	movq	%rax, %rcx
1409c7067: 48 8b 00                    	movq	(%rax), %rax
1409c706a: 48 83 f8 01                 	cmpq	$0x1, %rax
1409c706e: 75 60                       	jne	0x1409c70d0 <.text+0x9c60d0>
1409c7070: 48 83 c1 08                 	addq	$0x8, %rcx
1409c7074: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1409c7078: 75 76                       	jne	0x1409c70f0 <.text+0x9c60f0>
1409c707a: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1409c7081: 48 89 8d 30 09 00 00        	movq	%rcx, 0x930(%rbp)
1409c7088: 48 83 c1 08                 	addq	$0x8, %rcx
1409c708c: 48 8b 95 f0 08 00 00        	movq	0x8f0(%rbp), %rdx
1409c7093: e8 58 00 25 00              	callq	0x140c170f0 <.text+0xc160f0>
1409c7098: 48 8b 85 30 09 00 00        	movq	0x930(%rbp), %rax
1409c709f: 48 ff 00                    	incq	(%rax)
1409c70a2: 48 81 c4 c8 09 00 00        	addq	$0x9c8, %rsp            # imm = 0x9C8
1409c70a9: 5b                          	popq	%rbx
1409c70aa: 5f                          	popq	%rdi
1409c70ab: 5e                          	popq	%rsi
1409c70ac: 41 5c                       	popq	%r12
1409c70ae: 41 5d                       	popq	%r13
1409c70b0: 41 5e                       	popq	%r14
1409c70b2: 41 5f                       	popq	%r15
1409c70b4: 5d                          	popq	%rbp
1409c70b5: c3                          	retq
1409c70b6: 48 85 c0                    	testq	%rax, %rax
1409c70b9: 75 29                       	jne	0x1409c70e4 <.text+0x9c60e4>
1409c70bb: 31 d2                       	xorl	%edx, %edx
1409c70bd: e8 0e 17 f3 01              	callq	0x1428f87d0 <.text+0x28f77d0>
1409c70c2: 48 89 c1                    	movq	%rax, %rcx
1409c70c5: 48 85 c0                    	testq	%rax, %rax
1409c70c8: 0f 85 4c f8 ff ff           	jne	0x1409c691a <.text+0x9c591a>
1409c70ce: eb 14                       	jmp	0x1409c70e4 <.text+0x9c60e4>
1409c70d0: 48 85 c0                    	testq	%rax, %rax
1409c70d3: 75 0f                       	jne	0x1409c70e4 <.text+0x9c60e4>
1409c70d5: 31 d2                       	xorl	%edx, %edx
1409c70d7: e8 f4 16 f3 01              	callq	0x1428f87d0 <.text+0x28f77d0>
1409c70dc: 48 89 c1                    	movq	%rax, %rcx
1409c70df: 48 85 c0                    	testq	%rax, %rax
1409c70e2: 75 90                       	jne	0x1409c7074 <.text+0x9c6074>
1409c70e4: 48 8d 0d b5 39 27 02        	leaq	0x22739b5(%rip), %rcx   # 0x142c3aaa0
1409c70eb: e8 e0 0f f1 01              	callq	0x1428d80d0 <.text+0x28d70d0>
1409c70f0: 48 8d 0d 79 3a 27 02        	leaq	0x2273a79(%rip), %rcx   # 0x142c3ab70
1409c70f7: e8 54 21 f1 01              	callq	0x1428d9250 <.text+0x28d8250>
1409c70fc: 48 8d 0d 0d 8f 0f 02        	leaq	0x20f8f0d(%rip), %rcx   # 0x142ac0010
1409c7103: e8 08 22 f1 01              	callq	0x1428d9310 <.text+0x28d8310>
1409c7108: eb 67                       	jmp	0x1409c7171 <.text+0x9c6171>
1409c710a: 48 8d 0d 37 8f 0f 02        	leaq	0x20f8f37(%rip), %rcx   # 0x142ac0048
1409c7111: e8 fa 21 f1 01              	callq	0x1428d9310 <.text+0x28d8310>
1409c7116: eb 59                       	jmp	0x1409c7171 <.text+0x9c6171>
1409c7118: 48 8d 0d 41 8f 0f 02        	leaq	0x20f8f41(%rip), %rcx   # 0x142ac0060
1409c711f: e8 ec 21 f1 01              	callq	0x1428d9310 <.text+0x28d8310>
1409c7124: eb 4b                       	jmp	0x1409c7171 <.text+0x9c6171>
1409c7126: c6 85 3f 09 00 00 00        	movb	$0x0, 0x93f(%rbp)
1409c712d: 48 8d 0d 54 8f 0f 02        	leaq	0x20f8f54(%rip), %rcx   # 0x142ac0088
1409c7134: e8 d7 21 f1 01              	callq	0x1428d9310 <.text+0x28d8310>
1409c7139: eb 36                       	jmp	0x1409c7171 <.text+0x9c6171>
1409c713b: 48 8d 0d ee 8e 0f 02        	leaq	0x20f8eee(%rip), %rcx   # 0x142ac0030
1409c7142: e8 c9 21 f1 01              	callq	0x1428d9310 <.text+0x28d8310>
1409c7147: eb 28                       	jmp	0x1409c7171 <.text+0x9c6171>
1409c7149: c6 85 3f 09 00 00 01        	movb	$0x1, 0x93f(%rbp)
1409c7150: 48 8d 0d 49 39 27 02        	leaq	0x2273949(%rip), %rcx   # 0x142c3aaa0
1409c7157: e8 74 0f f1 01              	callq	0x1428d80d0 <.text+0x28d70d0>
1409c715c: eb 13                       	jmp	0x1409c7171 <.text+0x9c6171>
1409c715e: c6 85 3f 09 00 00 00        	movb	$0x0, 0x93f(%rbp)
1409c7165: 48 8d 0d 44 8f 0f 02        	leaq	0x20f8f44(%rip), %rcx   # 0x142ac00b0
1409c716c: e8 9f 21 f1 01              	callq	0x1428d9310 <.text+0x28d8310>
1409c7171: 0f 0b                       	ud2
