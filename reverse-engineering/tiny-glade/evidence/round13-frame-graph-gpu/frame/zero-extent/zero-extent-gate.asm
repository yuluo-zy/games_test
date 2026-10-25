
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1404dc850: 48 8d 8d e0 13 00 00        	leaq	0x13e0(%rbp), %rcx
1404dc857: e8 b4 c9 12 00              	callq	0x140609210 <.text+0x608210>
1404dc85c: 48 8b 8d 88 14 00 00        	movq	0x1488(%rbp), %rcx
1404dc863: 48 ba 8b 04 7b e6 61 26 c0 a4       	movabsq	$-0x5b3fd99e1984fb75, %rdx # imm = 0xA4C02661E67B048B
1404dc86d: 49 b8 ec 67 2a aa e6 61 e7 ea       	movabsq	$-0x15189e1955d59814, %r8 # imm = 0xEAE761E6AA2A67EC
1404dc877: e8 74 0e c0 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
1404dc87c: 48 85 c0                    	testq	%rax, %rax
1404dc87f: 4c 8b 85 78 14 00 00        	movq	0x1478(%rbp), %r8
1404dc886: 0f 84 e8 28 00 00           	je	0x1404df174 <.text+0x4de174>
1404dc88c: 49 39 90 48 02 00 00        	cmpq	%rdx, 0x248(%r8)
1404dc893: 0f 86 db 28 00 00           	jbe	0x1404df174 <.text+0x4de174>
1404dc899: 49 8b 80 40 02 00 00        	movq	0x240(%r8), %rax
1404dc8a0: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
1404dc8a4: 48 85 c9                    	testq	%rcx, %rcx
1404dc8a7: 0f 84 c7 28 00 00           	je	0x1404df174 <.text+0x4de174>
1404dc8ad: 49 8b 80 10 02 00 00        	movq	0x210(%r8), %rax
1404dc8b4: 48 f7 d1                    	notq	%rcx
1404dc8b7: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1404dc8bb: 48 c1 e1 05                 	shlq	$0x5, %rcx
1404dc8bf: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
1404dc8c5: 0f 84 a9 28 00 00           	je	0x1404df174 <.text+0x4de174>
1404dc8cb: 48 01 c8                    	addq	%rcx, %rax
1404dc8ce: 48 8b 40 10                 	movq	0x10(%rax), %rax
1404dc8d2: 48 8b 00                    	movq	(%rax), %rax
1404dc8d5: 48 89 85 98 14 00 00        	movq	%rax, 0x1498(%rbp)
1404dc8dc: 48 8b 8d 88 14 00 00        	movq	0x1488(%rbp), %rcx
1404dc8e3: 48 ba 80 ee bc 5f 6a 38 35 33       	movabsq	$0x3335386a5fbcee80, %rdx # imm = 0x3335386A5FBCEE80
1404dc8ed: 49 b8 60 ec 65 1a f3 59 2a 8f       	movabsq	$-0x70d5a60ce59a13a0, %r8 # imm = 0x8F2A59F31A65EC60
1404dc8f7: e8 f4 0d c0 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
1404dc8fc: 48 85 c0                    	testq	%rax, %rax
1404dc8ff: 4c 8b 85 78 14 00 00        	movq	0x1478(%rbp), %r8
1404dc906: 0f 84 57 28 00 00           	je	0x1404df163 <.text+0x4de163>
1404dc90c: 49 39 90 48 02 00 00        	cmpq	%rdx, 0x248(%r8)
1404dc913: 0f 86 4a 28 00 00           	jbe	0x1404df163 <.text+0x4de163>
1404dc919: 49 8b 80 40 02 00 00        	movq	0x240(%r8), %rax
1404dc920: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
1404dc924: 48 85 c9                    	testq	%rcx, %rcx
1404dc927: 0f 84 36 28 00 00           	je	0x1404df163 <.text+0x4de163>
1404dc92d: 49 8b 80 10 02 00 00        	movq	0x210(%r8), %rax
1404dc934: 48 f7 d1                    	notq	%rcx
1404dc937: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1404dc93b: 48 c1 e1 05                 	shlq	$0x5, %rcx
1404dc93f: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
1404dc945: 0f 84 18 28 00 00           	je	0x1404df163 <.text+0x4de163>
1404dc94b: 4c 8b 85 98 14 00 00        	movq	0x1498(%rbp), %r8
1404dc952: 45 85 c0                    	testl	%r8d, %r8d
1404dc955: 0f 95 c2                    	setne	%dl
1404dc958: 49 c1 e8 20                 	shrq	$0x20, %r8
1404dc95c: 41 0f 95 c0                 	setne	%r8b
1404dc960: 41 84 d0                    	testb	%dl, %r8b
1404dc963: 75 1b                       	jne	0x1404dc980 <.text+0x4db980>
1404dc965: 31 c9                       	xorl	%ecx, %ecx
1404dc967: ba 40 42 0f 00              	movl	$0xf4240, %edx          # imm = 0xF4240
1404dc96c: e8 0f 90 87 00              	callq	0x140d55980 <.text+0xd54980>
1404dc971: e9 d7 1e 00 00              	jmp	0x1404de84d <.text+0x4dd84d>
1404dc976: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
