
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1404dacb3: 89 fa                       	movl	%edi, %edx
1404dacb5: 41 89 f0                    	movl	%esi, %r8d
1404dacb8: e8 43 ef 46 00              	callq	0x140949c00 <.text+0x948c00>
1404dacbd: c6 85 a7 14 00 00 01        	movb	$0x1, 0x14a7(%rbp)
1404dacc4: c6 85 a6 14 00 00 01        	movb	$0x1, 0x14a6(%rbp)
1404daccb: c6 85 a5 14 00 00 01        	movb	$0x1, 0x14a5(%rbp)
1404dacd2: 48 8b 8d 88 14 00 00        	movq	0x1488(%rbp), %rcx
1404dacd9: 48 ba 8b 04 7b e6 61 26 c0 a4       	movabsq	$-0x5b3fd99e1984fb75, %rdx # imm = 0xA4C02661E67B048B
1404dace3: 49 b8 ec 67 2a aa e6 61 e7 ea       	movabsq	$-0x15189e1955d59814, %r8 # imm = 0xEAE761E6AA2A67EC
1404daced: e8 fe 29 c0 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
1404dacf2: 48 85 c0                    	testq	%rax, %rax
1404dacf5: 0f 84 a9 45 00 00           	je	0x1404df2a4 <.text+0x4de2a4>
1404dacfb: 49 39 94 24 48 02 00 00     	cmpq	%rdx, 0x248(%r12)
1404dad03: 0f 86 9b 45 00 00           	jbe	0x1404df2a4 <.text+0x4de2a4>
1404dad09: 49 8b 84 24 40 02 00 00     	movq	0x240(%r12), %rax
1404dad11: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
1404dad15: 48 85 c9                    	testq	%rcx, %rcx
1404dad18: 0f 84 86 45 00 00           	je	0x1404df2a4 <.text+0x4de2a4>
1404dad1e: 49 8b 84 24 10 02 00 00     	movq	0x210(%r12), %rax
1404dad26: 48 f7 d1                    	notq	%rcx
1404dad29: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1404dad2d: 48 c1 e1 05                 	shlq	$0x5, %rcx
1404dad31: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
1404dad37: 0f 84 67 45 00 00           	je	0x1404df2a4 <.text+0x4de2a4>
1404dad3d: 48 01 c8                    	addq	%rcx, %rax
1404dad40: 48 8b 48 10                 	movq	0x10(%rax), %rcx
1404dad44: 41 8b 94 24 e0 05 00 00     	movl	0x5e0(%r12), %edx
1404dad4c: 89 50 5c                    	movl	%edx, 0x5c(%rax)
1404dad4f: 89 39                       	movl	%edi, (%rcx)
1404dad51: 89 71 04                    	movl	%esi, 0x4(%rcx)
