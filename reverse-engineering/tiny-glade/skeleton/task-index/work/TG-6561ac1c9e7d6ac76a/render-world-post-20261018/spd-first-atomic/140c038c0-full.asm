
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140c038c0: 56                          	push	rsi
140c038c1: 48 81 ec d0 00 00 00        	sub	rsp, 0xd0
140c038c8: 48 89 ce                    	mov	rsi, rcx
140c038cb: 48 8b 8c 24 00 01 00 00     	mov	rcx, qword ptr [rsp + 0x100]
140c038d3: 48 8d 42 78                 	lea	rax, [rdx + 0x78]
140c038d7: 4c 8d 92 70 08 00 00        	lea	r10, [rdx + 0x870]
140c038de: 4c 8b 9a 70 08 00 00        	mov	r11, qword ptr [rdx + 0x870]
140c038e5: 4d 85 db                    	test	r11, r11
140c038e8: 4d 0f 44 d3                 	cmove	r10, r11
140c038ec: 4c 8b 9a 48 06 00 00        	mov	r11, qword ptr [rdx + 0x648]
140c038f3: 48 8b 92 78 06 00 00        	mov	rdx, qword ptr [rdx + 0x678]
140c038fa: 48 83 c2 10                 	add	rdx, 0x10
140c038fe: 0f 10 01                    	movups	xmm0, xmmword ptr [rcx]
140c03901: 0f 29 84 24 c0 00 00 00     	movaps	xmmword ptr [rsp + 0xc0], xmm0
140c03909: 4c 89 4c 24 38              	mov	qword ptr [rsp + 0x38], r9
140c0390e: 4c 89 44 24 30              	mov	qword ptr [rsp + 0x30], r8
140c03913: 48 8d 8c 24 c0 00 00 00     	lea	rcx, [rsp + 0xc0]
140c0391b: 48 89 4c 24 28              	mov	qword ptr [rsp + 0x28], rcx
140c03920: 48 89 54 24 20              	mov	qword ptr [rsp + 0x20], rdx
140c03925: 48 8d 4c 24 70              	lea	rcx, [rsp + 0x70]
140c0392a: 48 89 c2                    	mov	rdx, rax
140c0392d: 4d 89 d0                    	mov	r8, r10
140c03930: 4d 89 d9                    	mov	r9, r11
140c03933: e8 c8 a8 fe ff              	call	0x140bee200 <.text+0xbed200>
140c03938: 48 8b 44 24 70              	mov	rax, qword ptr [rsp + 0x70]
140c0393d: 48 8d 4e 08                 	lea	rcx, [rsi + 0x8]
140c03941: 0f 10 44 24 78              	movups	xmm0, xmmword ptr [rsp + 0x78]
140c03946: 0f 29 44 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm0
140c0394b: 0f 10 84 24 88 00 00 00     	movups	xmm0, xmmword ptr [rsp + 0x88]
140c03953: 0f 29 44 24 50              	movaps	xmmword ptr [rsp + 0x50], xmm0
140c03958: 0f 10 84 24 98 00 00 00     	movups	xmm0, xmmword ptr [rsp + 0x98]
140c03960: 0f 29 44 24 60              	movaps	xmmword ptr [rsp + 0x60], xmm0
140c03965: 48 83 f8 0b                 	cmp	rax, 0xb
140c03969: 75 46                       	jne	0x140c039b1 <.text+0xc029b1>
140c0396b: 0f 28 44 24 40              	movaps	xmm0, xmmword ptr [rsp + 0x40]
140c03970: 0f 28 4c 24 50              	movaps	xmm1, xmmword ptr [rsp + 0x50]
140c03975: 0f 28 54 24 60              	movaps	xmm2, xmmword ptr [rsp + 0x60]
140c0397a: 0f 29 94 24 90 00 00 00     	movaps	xmmword ptr [rsp + 0x90], xmm2
140c03982: 0f 29 8c 24 80 00 00 00     	movaps	xmmword ptr [rsp + 0x80], xmm1
140c0398a: 0f 29 44 24 70              	movaps	xmmword ptr [rsp + 0x70], xmm0
140c0398f: 48 8b 94 24 98 00 00 00     	mov	rdx, qword ptr [rsp + 0x98]
140c03997: 48 89 51 20                 	mov	qword ptr [rcx + 0x20], rdx
140c0399b: 0f 10 84 24 88 00 00 00     	movups	xmm0, xmmword ptr [rsp + 0x88]
140c039a3: 0f 11 41 10                 	movups	xmmword ptr [rcx + 0x10], xmm0
140c039a7: 0f 10 44 24 78              	movups	xmm0, xmmword ptr [rsp + 0x78]
140c039ac: 0f 11 01                    	movups	xmmword ptr [rcx], xmm0
140c039af: eb 32                       	jmp	0x140c039e3 <.text+0xc029e3>
140c039b1: 48 8b 94 24 a8 00 00 00     	mov	rdx, qword ptr [rsp + 0xa8]
140c039b9: 0f 10 84 24 b0 00 00 00     	movups	xmm0, xmmword ptr [rsp + 0xb0]
140c039c1: 0f 28 4c 24 40              	movaps	xmm1, xmmword ptr [rsp + 0x40]
140c039c6: 0f 28 54 24 50              	movaps	xmm2, xmmword ptr [rsp + 0x50]
140c039cb: 0f 28 5c 24 60              	movaps	xmm3, xmmword ptr [rsp + 0x60]
140c039d0: 0f 11 59 20                 	movups	xmmword ptr [rcx + 0x20], xmm3
140c039d4: 0f 11 51 10                 	movups	xmmword ptr [rcx + 0x10], xmm2
140c039d8: 0f 11 09                    	movups	xmmword ptr [rcx], xmm1
140c039db: 48 89 56 38                 	mov	qword ptr [rsi + 0x38], rdx
140c039df: 0f 11 46 40                 	movups	xmmword ptr [rsi + 0x40], xmm0
140c039e3: 48 89 06                    	mov	qword ptr [rsi], rax
140c039e6: 48 89 f0                    	mov	rax, rsi
140c039e9: 48 81 c4 d0 00 00 00        	add	rsp, 0xd0
140c039f0: 5e                          	pop	rsi
140c039f1: c3                          	ret
