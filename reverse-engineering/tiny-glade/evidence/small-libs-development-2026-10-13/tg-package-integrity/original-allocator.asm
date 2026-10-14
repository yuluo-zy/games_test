
D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140613c10:     	jmp	0x140d657b0 <.text+0xd647b0>
140613c15:     	int3
140613c16:     	int3
140613c17:     	int3
140613c18:     	int3
140613c19:     	int3
140613c1a:     	int3
140613c1b:     	int3
140613c1c:     	int3
140613c1d:     	int3
140613c1e:     	int3
140613c1f:     	int3
140613c20:     	jmp	0x140d65810 <.text+0xd64810>
140613c25:     	int3
140613c26:     	int3
140613c27:     	int3
140613c28:     	int3
140613c29:     	int3
140613c2a:     	int3
140613c2b:     	int3
140613c2c:     	int3
140613c2d:     	int3
140613c2e:     	int3
140613c2f:     	int3
140613c30:     	jmp	0x140d65850 <.text+0xd64850>
140613c35:     	int3
140613c36:     	int3
140613c37:     	int3
140613c38:     	int3
140613c39:     	int3
140613c3a:     	int3
140613c3b:     	int3
140613c3c:     	int3
140613c3d:     	int3
140613c3e:     	int3
140613c3f:     	int3
140613c40:     	jmp	0x140d65900 <.text+0xd64900>
140613c45:     	int3
140613c46:     	int3
140613c47:     	int3
140613c48:     	int3
140613c49:     	int3
140613c4a:     	int3
140613c4b:     	int3
140613c4c:     	int3
140613c4d:     	int3
140613c4e:     	int3
140613c4f:     	int3
140613c50:     	jmp	0x140d78910 <.text+0xd77910>
140613c55:     	int3
140613c56:     	int3
140613c57:     	int3
140613c58:     	int3
140613c59:     	int3
140613c5a:     	int3
140613c5b:     	int3
140613c5c:     	int3
140613c5d:     	int3
140613c5e:     	int3
140613c5f:     	int3
140613c60:     	push	rbp
140613c61:     	push	r14
140613c63:     	push	rsi
140613c64:     	push	rdi
140613c65:     	push	rbx
140613c66:     	sub	rsp, 0xd0
140613c6d:     	lea	rbp, [rsp + 0x80]
140613c75:     	mov	qword ptr [rbp + 0x48], -0x2
140613c7d:     	mov	rsi, rcx
140613c80:     	mov	qword ptr [rbp + 0x28], rdx
140613c84:     	mov	qword ptr [rbp + 0x30], r8
140613c88:     	mov	qword ptr [rbp + 0x38], 0x0
140613c90:     	mov	qword ptr [rbp + 0x10], 0x0
140613c98:     	mov	qword ptr [rbp + 0x18], 0x1
140613ca0:     	mov	qword ptr [rbp + 0x20], 0x0
140613ca8:     	mov	byte ptr [rbp + 0x40], -0x80
140613cac:     	lea	rcx, [rbp - 0x50]
140613cb0:     	lea	rdx, [rbp + 0x10]
140613cb4:     	call	0x140616a10 <.text+0x615a10>
140613cb9:     	xor	eax, eax
140613cbb:     	cmp	rax, qword ptr [rbp - 0x50]
140613cbf:     	jno	0x140613cdb <.text+0x612cdb>
140613cc1:     	mov	rax, qword ptr [rbp - 0x48]
140613cc5:     	mov	qword ptr [rsi + 0x8], rax
140613cc9:     	movabs	rax, -0x8000000000000000
140613cd3:     	mov	qword ptr [rsi], rax
140613cd6:     	jmp	0x140613d5d <.text+0x612d5d>
140613cdb:     	movups	xmm0, xmmword ptr [rbp - 0x50]
140613cdf:     	movups	xmm1, xmmword ptr [rbp - 0x40]
140613ce3:     	movups	xmm2, xmmword ptr [rbp - 0x30]
140613ce7:     	movaps	xmmword ptr [rbp], xmm2
140613ceb:     	movaps	xmmword ptr [rbp - 0x10], xmm1
140613cef:     	movaps	xmmword ptr [rbp - 0x20], xmm0
140613cf3:     	mov	rax, qword ptr [rbp + 0x30]
140613cf7:     	mov	rcx, qword ptr [rbp + 0x38]
140613cfb:     	cmp	rcx, rax
140613cfe:     	jae	0x140613d46 <.text+0x612d46>
