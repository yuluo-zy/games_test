
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1409a8e10: 48 89 c8                    	mov	rax, rcx
1409a8e13: 0f 10 02                    	movups	xmm0, xmmword ptr [rdx]
1409a8e16: 0f 10 4a 10                 	movups	xmm1, xmmword ptr [rdx + 0x10]
1409a8e1a: 0f 11 89 98 00 00 00        	movups	xmmword ptr [rcx + 0x98], xmm1
1409a8e21: 0f 11 81 88 00 00 00        	movups	xmmword ptr [rcx + 0x88], xmm0
1409a8e28: 41 0f 10 00                 	movups	xmm0, xmmword ptr [r8]
1409a8e2c: 0f 11 41 78                 	movups	xmmword ptr [rcx + 0x78], xmm0
1409a8e30: c7 41 18 01 00 00 00        	mov	dword ptr [rcx + 0x18], 0x1
1409a8e37: 48 c7 41 20 00 00 00 00     	mov	qword ptr [rcx + 0x20], 0x0
1409a8e3f: c7 41 30 02 00 00 00        	mov	dword ptr [rcx + 0x30], 0x2
1409a8e46: c7 41 48 03 00 00 00        	mov	dword ptr [rcx + 0x48], 0x3
1409a8e4d: c7 41 60 03 00 00 00        	mov	dword ptr [rcx + 0x60], 0x3
1409a8e54: 48 c7 01 00 00 00 00        	mov	qword ptr [rcx], 0x0
1409a8e5b: 48 c7 41 08 08 00 00 00     	mov	qword ptr [rcx + 0x8], 0x8
1409a8e63: 48 c7 41 10 00 00 00 00     	mov	qword ptr [rcx + 0x10], 0x0
1409a8e6b: 66 c7 81 a8 00 00 00 01 01  	mov	word ptr [rcx + 0xa8], 0x101
1409a8e74: c3                          	ret
