
D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140631ce0: 48 89 c8                    	movq	%rcx, %rax
140631ce3: 89 51 30                    	movl	%edx, 0x30(%rcx)
140631ce6: 44 89 41 34                 	movl	%r8d, 0x34(%rcx)
140631cea: 48 c7 01 00 00 00 00        	movq	$0x0, (%rcx)
140631cf1: 48 c7 41 08 01 00 00 00     	movq	$0x1, 0x8(%rcx)
140631cf9: 0f 57 c0                    	xorps	%xmm0, %xmm0
140631cfc: 0f 11 41 10                 	movups	%xmm0, 0x10(%rcx)
140631d00: 48 c7 41 20 01 00 00 00     	movq	$0x1, 0x20(%rcx)
140631d08: 48 c7 41 28 00 00 00 00     	movq	$0x0, 0x28(%rcx)
140631d10: 66 c7 41 38 00 00           	movw	$0x0, 0x38(%rcx)
140631d16: c3                          	retq
140631d17: cc                          	int3
140631d18: cc                          	int3
140631d19: cc                          	int3
140631d1a: cc                          	int3
140631d1b: cc                          	int3
140631d1c: cc                          	int3
140631d1d: cc                          	int3
140631d1e: cc                          	int3
140631d1f: cc                          	int3
140631d20: 48 89 c8                    	movq	%rcx, %rax
140631d23: 44 88 42 38                 	movb	%r8b, 0x38(%rdx)
140631d27: 0f 10 02                    	movups	(%rdx), %xmm0
140631d2a: 0f 10 4a 10                 	movups	0x10(%rdx), %xmm1
140631d2e: 0f 10 52 20                 	movups	0x20(%rdx), %xmm2
