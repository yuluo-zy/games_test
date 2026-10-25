
D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140631d20: 48 89 c8                    	movq	%rcx, %rax
140631d23: 44 88 42 38                 	movb	%r8b, 0x38(%rdx)
140631d27: 0f 10 02                    	movups	(%rdx), %xmm0
140631d2a: 0f 10 4a 10                 	movups	0x10(%rdx), %xmm1
140631d2e: 0f 10 52 20                 	movups	0x20(%rdx), %xmm2
140631d32: 0f 11 51 20                 	movups	%xmm2, 0x20(%rcx)
140631d36: 0f 11 49 10                 	movups	%xmm1, 0x10(%rcx)
140631d3a: 0f 11 01                    	movups	%xmm0, (%rcx)
140631d3d: 48 8b 4a 30                 	movq	0x30(%rdx), %rcx
140631d41: 48 89 48 30                 	movq	%rcx, 0x30(%rax)
140631d45: 0f b6 4a 38                 	movzbl	0x38(%rdx), %ecx
140631d49: 88 48 38                    	movb	%cl, 0x38(%rax)
140631d4c: 8b 4a 39                    	movl	0x39(%rdx), %ecx
140631d4f: 89 48 39                    	movl	%ecx, 0x39(%rax)
140631d52: 0f b7 4a 3d                 	movzwl	0x3d(%rdx), %ecx
140631d56: 66 89 48 3d                 	movw	%cx, 0x3d(%rax)
140631d5a: 0f b6 4a 3f                 	movzbl	0x3f(%rdx), %ecx
140631d5e: 88 48 3f                    	movb	%cl, 0x3f(%rax)
140631d61: c3                          	retq
140631d62: cc                          	int3
140631d63: cc                          	int3
140631d64: cc                          	int3
140631d65: cc                          	int3
140631d66: cc                          	int3
140631d67: cc                          	int3
140631d68: cc                          	int3
140631d69: cc                          	int3
140631d6a: cc                          	int3
140631d6b: cc                          	int3
140631d6c: cc                          	int3
140631d6d: cc                          	int3
140631d6e: cc                          	int3
140631d6f: cc                          	int3
