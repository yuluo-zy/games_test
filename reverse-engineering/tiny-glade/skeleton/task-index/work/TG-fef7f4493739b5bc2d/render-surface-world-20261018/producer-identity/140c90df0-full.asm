
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140c90df0: 56                          	push	rsi
140c90df1: 48 83 ec 30                 	sub	rsp, 0x30
140c90df5: 0f 29 74 24 20              	movaps	xmmword ptr [rsp + 0x20], xmm6
140c90dfa: 0f 28 f1                    	movaps	xmm6, xmm1
140c90dfd: e8 1e 91 c2 ff              	call	0x1408b9f20 <.text+0x8b8f20>
140c90e02: 0f b7 f0                    	movzx	esi, ax
140c90e05: 0f 28 c6                    	movaps	xmm0, xmm6
140c90e08: e8 13 91 c2 ff              	call	0x1408b9f20 <.text+0x8b8f20>
140c90e0d: c1 e0 10                    	shl	eax, 0x10
140c90e10: 09 f0                       	or	eax, esi
140c90e12: 0f 28 74 24 20              	movaps	xmm6, xmmword ptr [rsp + 0x20]
140c90e17: 48 83 c4 30                 	add	rsp, 0x30
140c90e1b: 5e                          	pop	rsi
140c90e1c: c3                          	ret
