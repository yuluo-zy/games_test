
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1413018b0: 55                          	push	rbp
1413018b1: 41 57                       	push	r15
1413018b3: 41 56                       	push	r14
1413018b5: 41 55                       	push	r13
1413018b7: 41 54                       	push	r12
1413018b9: 56                          	push	rsi
1413018ba: 57                          	push	rdi
1413018bb: 53                          	push	rbx
1413018bc: 48 81 ec 08 01 00 00        	sub	rsp, 0x108
1413018c3: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
1413018cb: 48 c7 85 80 00 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x80], -0x2
1413018d6: 4c 89 45 70                 	mov	qword ptr [rbp + 0x70], r8
1413018da: 48 89 d7                    	mov	rdi, rdx
1413018dd: 0f b6 12                    	movzx	edx, byte ptr [rdx]
1413018e0: 48 8b 5f 08                 	mov	rbx, qword ptr [rdi + 0x8]
1413018e4: 48 8b 47 10                 	mov	rax, qword ptr [rdi + 0x10]
1413018e8: 48 89 45 38                 	mov	qword ptr [rbp + 0x38], rax
1413018ec: 48 29 d8                    	sub	rax, rbx
1413018ef: 45 31 ff                    	xor	r15d, r15d
1413018f2: 88 55 7e                    	mov	byte ptr [rbp + 0x7e], dl
1413018f5: 84 d2                       	test	dl, dl
1413018f7: 41 be 00 00 00 00           	mov	r14d, 0x0
1413018fd: 48 89 45 b8                 	mov	qword ptr [rbp - 0x48], rax
141301901: 4c 0f 45 f0                 	cmovne	r14, rax
141301905: 48 89 4d a8                 	mov	qword ptr [rbp - 0x58], rcx
141301909: 0f b6 57 70                 	movzx	edx, byte ptr [rdi + 0x70]
14130190d: 48 8b 77 78                 	mov	rsi, qword ptr [rdi + 0x78]
141301911: 48 8b 87 80 00 00 00        	mov	rax, qword ptr [rdi + 0x80]
141301918: 48 89 45 b0                 	mov	qword ptr [rbp - 0x50], rax
14130191c: 48 89 c1                    	mov	rcx, rax
14130191f: 48 29 f1                    	sub	rcx, rsi
141301922: 88 55 7f                    	mov	byte ptr [rbp + 0x7f], dl
141301925: 84 d2                       	test	dl, dl
141301927: b8 00 00 00 00              	mov	eax, 0x0
14130192c: 48 89 4d c0                 	mov	qword ptr [rbp - 0x40], rcx
141301930: 48 0f 45 c1                 	cmovne	rax, rcx
141301934: 48 8b 8f e0 00 00 00        	mov	rcx, qword ptr [rdi + 0xe0]
14130193b: 48 89 4d 30                 	mov	qword ptr [rbp + 0x30], rcx
14130193f: 48 85 c9                    	test	rcx, rcx
141301942: 74 1b                       	je	0x14130195f <.text+0x130095f>
141301944: 8b 8f ec 00 00 00           	mov	ecx, dword ptr [rdi + 0xec]
14130194a: 2b 8f e8 00 00 00           	sub	ecx, dword ptr [rdi + 0xe8]
141301950: 41 0f 42 cf                 	cmovb	ecx, r15d
141301954: 4c 8d 3c cd 00 00 00 00     	lea	r15, [8*rcx]
14130195c: 49 29 cf                    	sub	r15, rcx
14130195f: 49 01 c6                    	add	r14, rax
141301962: 0f 82 13 03 00 00           	jb	0x141301c7b <.text+0x1300c7b>
141301968: 4d 01 f7                    	add	r15, r14
14130196b: 0f 82 0a 03 00 00           	jb	0x141301c7b <.text+0x1300c7b>
141301971: 45 31 ed                    	xor	r13d, r13d
141301974: b9 0c 00 00 00              	mov	ecx, 0xc
141301979: 4c 89 f8                    	mov	rax, r15
14130197c: 48 f7 e1                    	mul	rcx
14130197f: 49 89 c4                    	mov	r12, rax
141301982: 70 33                       	jo	0x1413019b7 <.text+0x13009b7>
141301984: 48 b8 fc ff ff ff ff ff ff 7f       	movabs	rax, 0x7ffffffffffffffc
14130198e: 49 39 c4                    	cmp	r12, rax
141301991: 77 24                       	ja	0x1413019b7 <.text+0x13009b7>
141301993: 4d 85 e4                    	test	r12, r12
141301996: 74 2e                       	je	0x1413019c6 <.text+0x13009c6>
141301998: 0f b6 05 6a 32 33 02        	movzx	eax, byte ptr [rip + 0x233326a] # 0x143634c09
14130199f: 41 bd 04 00 00 00           	mov	r13d, 0x4
1413019a5: ba 04 00 00 00              	mov	edx, 0x4
1413019aa: 4c 89 e1                    	mov	rcx, r12
1413019ad: e8 5e 22 31 ff              	call	0x140613c10 <.text+0x612c10>
1413019b2: 48 85 c0                    	test	rax, rax
1413019b5: 75 17                       	jne	0x1413019ce <.text+0x13009ce>
1413019b7: 4c 89 e9                    	mov	rcx, r13
1413019ba: 4c 89 e2                    	mov	rdx, r12
1413019bd: 4c 8b 45 70                 	mov	r8, qword ptr [rbp + 0x70]
1413019c1: e8 1d 76 5d 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
1413019c6: b8 04 00 00 00              	mov	eax, 0x4
1413019cb: 45 31 ff                    	xor	r15d, r15d
1413019ce: 31 c9                       	xor	ecx, ecx
1413019d0: 4c 89 7d 40                 	mov	qword ptr [rbp + 0x40], r15
1413019d4: 48 89 45 48                 	mov	qword ptr [rbp + 0x48], rax
1413019d8: 48 c7 45 50 00 00 00 00     	mov	qword ptr [rbp + 0x50], 0x0
1413019e0: 44 8b af e8 00 00 00        	mov	r13d, dword ptr [rdi + 0xe8]
1413019e7: 44 8b a7 ec 00 00 00        	mov	r12d, dword ptr [rdi + 0xec]
1413019ee: 48 83 7d 30 00              	cmp	qword ptr [rbp + 0x30], 0x0
1413019f3: 74 14                       	je	0x141301a09 <.text+0x1300a09>
1413019f5: 44 89 e2                    	mov	edx, r12d
1413019f8: 44 29 ea                    	sub	edx, r13d
1413019fb: 0f 42 d1                    	cmovb	edx, ecx
1413019fe: 48 8d 0c d5 00 00 00 00     	lea	rcx, [8*rdx]
141301a06: 48 29 d1                    	sub	rcx, rdx
141301a09: 49 01 ce                    	add	r14, rcx
141301a0c: 0f 82 98 02 00 00           	jb	0x141301caa <.text+0x1300caa>
141301a12: 4d 39 f7                    	cmp	r15, r14
141301a15: 0f 82 32 02 00 00           	jb	0x141301c4d <.text+0x1300c4d>
141301a1b: 31 c9                       	xor	ecx, ecx
141301a1d: 4c 8b 75 38                 	mov	r14, qword ptr [rbp + 0x38]
141301a21: 48 8d 55 50                 	lea	rdx, [rbp + 0x50]
141301a25: 48 89 55 58                 	mov	qword ptr [rbp + 0x58], rdx
141301a29: 48 89 4d 60                 	mov	qword ptr [rbp + 0x60], rcx
141301a2d: 48 89 45 68                 	mov	qword ptr [rbp + 0x68], rax
141301a31: f6 45 7e 01                 	test	byte ptr [rbp + 0x7e], 0x1
141301a35: 0f 84 d0 00 00 00           	je	0x141301b0b <.text+0x1300b0b>
141301a3b: 48 8d 57 18                 	lea	rdx, [rdi + 0x18]
141301a3f: 4c 8b 42 50                 	mov	r8, qword ptr [rdx + 0x50]
141301a43: 4c 89 45 28                 	mov	qword ptr [rbp + 0x28], r8
141301a47: 0f 10 42 40                 	movups	xmm0, xmmword ptr [rdx + 0x40]
141301a4b: 0f 11 45 18                 	movups	xmmword ptr [rbp + 0x18], xmm0
141301a4f: 0f 10 02                    	movups	xmm0, xmmword ptr [rdx]
141301a52: 0f 10 4a 10                 	movups	xmm1, xmmword ptr [rdx + 0x10]
141301a56: 0f 10 52 20                 	movups	xmm2, xmmword ptr [rdx + 0x20]
141301a5a: 0f 10 5a 30                 	movups	xmm3, xmmword ptr [rdx + 0x30]
141301a5e: 0f 11 5d 08                 	movups	xmmword ptr [rbp + 0x8], xmm3
141301a62: 0f 11 55 f8                 	movups	xmmword ptr [rbp - 0x8], xmm2
141301a66: 0f 11 4d e8                 	movups	xmmword ptr [rbp - 0x18], xmm1
141301a6a: 0f 11 45 d8                 	movups	xmmword ptr [rbp - 0x28], xmm0
141301a6e: 49 39 de                    	cmp	r14, rbx
141301a71: 0f 84 94 00 00 00           	je	0x141301b0b <.text+0x1300b0b>
141301a77: 48 8d 53 01                 	lea	rdx, [rbx + 0x1]
141301a7b: f6 45 b8 01                 	test	byte ptr [rbp - 0x48], 0x1
141301a7f: 74 24                       	je	0x141301aa5 <.text+0x1300aa5>
141301a81: 4c 8d 45 d8                 	lea	r8, [rbp - 0x28]
141301a85: 4c 8d 0c 5b                 	lea	r9, [rbx + 2*rbx]
141301a89: 4c 8d 14 49                 	lea	r10, [rcx + 2*rcx]
141301a8d: 47 8b 5c 88 08              	mov	r11d, dword ptr [r8 + 4*r9 + 0x8]
141301a92: 46 89 5c 90 08              	mov	dword ptr [rax + 4*r10 + 0x8], r11d
141301a97: 4f 8b 04 88                 	mov	r8, qword ptr [r8 + 4*r9]
141301a9b: 4e 89 04 90                 	mov	qword ptr [rax + 4*r10], r8
141301a9f: 48 ff c1                    	inc	rcx
141301aa2: 48 89 d3                    	mov	rbx, rdx
141301aa5: 49 39 d6                    	cmp	r14, rdx
141301aa8: 74 5d                       	je	0x141301b07 <.text+0x1300b07>
141301aaa: 48 8d 14 49                 	lea	rdx, [rcx + 2*rcx]
141301aae: 48 8d 04 90                 	lea	rax, [rax + 4*rdx]
141301ab2: 48 8d 14 5b                 	lea	rdx, [rbx + 2*rbx]
141301ab6: 48 8d 14 95 e4 ff ff ff     	lea	rdx, [4*rdx - 0x1c]
141301abe: 48 01 ea                    	add	rdx, rbp
141301ac1: 45 31 c0                    	xor	r8d, r8d
141301ac4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
141301ad0: 46 8b 4c 02 fc              	mov	r9d, dword ptr [rdx + r8 - 0x4]
141301ad5: 46 89 4c 00 08              	mov	dword ptr [rax + r8 + 0x8], r9d
141301ada: 4e 8b 4c 02 f4              	mov	r9, qword ptr [rdx + r8 - 0xc]
141301adf: 4e 89 0c 00                 	mov	qword ptr [rax + r8], r9
141301ae3: 4e 8b 0c 02                 	mov	r9, qword ptr [rdx + r8]
141301ae7: 4e 89 4c 00 0c              	mov	qword ptr [rax + r8 + 0xc], r9
141301aec: 46 8b 4c 02 08              	mov	r9d, dword ptr [rdx + r8 + 0x8]
141301af1: 46 89 4c 00 14              	mov	dword ptr [rax + r8 + 0x14], r9d
141301af6: 48 83 c1 02                 	add	rcx, 0x2
141301afa: 48 83 c3 02                 	add	rbx, 0x2
141301afe: 49 83 c0 18                 	add	r8, 0x18
141301b02: 49 39 de                    	cmp	r14, rbx
141301b05: 75 c9                       	jne	0x141301ad0 <.text+0x1300ad0>
141301b07: 48 89 4d 60                 	mov	qword ptr [rbp + 0x60], rcx
141301b0b: 48 8b 45 30                 	mov	rax, qword ptr [rbp + 0x30]
141301b0f: 48 85 c0                    	test	rax, rax
141301b12: 74 1c                       	je	0x141301b30 <.text+0x1300b30>
141301b14: 49 c1 e4 20                 	shl	r12, 0x20
141301b18: 4d 09 ec                    	or	r12, r13
141301b1b: 48 89 45 c8                 	mov	qword ptr [rbp - 0x38], rax
141301b1f: 4c 89 65 d0                 	mov	qword ptr [rbp - 0x30], r12
141301b23: 48 8d 4d c8                 	lea	rcx, [rbp - 0x38]
141301b27: 48 8d 55 58                 	lea	rdx, [rbp + 0x58]
141301b2b: e8 80 68 17 00              	call	0x1414783b0 <.text+0x14773b0>
141301b30: f6 45 7f 01                 	test	byte ptr [rbp + 0x7f], 0x1
141301b34: 0f 84 e1 00 00 00           	je	0x141301c1b <.text+0x1300c1b>
141301b3a: 48 81 c7 88 00 00 00        	add	rdi, 0x88
141301b41: 48 8b 47 50                 	mov	rax, qword ptr [rdi + 0x50]
141301b45: 48 89 45 28                 	mov	qword ptr [rbp + 0x28], rax
141301b49: 0f 10 47 40                 	movups	xmm0, xmmword ptr [rdi + 0x40]
141301b4d: 0f 11 45 18                 	movups	xmmword ptr [rbp + 0x18], xmm0
141301b51: 0f 10 07                    	movups	xmm0, xmmword ptr [rdi]
141301b54: 0f 10 4f 10                 	movups	xmm1, xmmword ptr [rdi + 0x10]
141301b58: 0f 10 57 20                 	movups	xmm2, xmmword ptr [rdi + 0x20]
141301b5c: 0f 10 5f 30                 	movups	xmm3, xmmword ptr [rdi + 0x30]
141301b60: 0f 11 5d 08                 	movups	xmmword ptr [rbp + 0x8], xmm3
141301b64: 0f 11 55 f8                 	movups	xmmword ptr [rbp - 0x8], xmm2
141301b68: 0f 11 4d e8                 	movups	xmmword ptr [rbp - 0x18], xmm1
141301b6c: 0f 11 45 d8                 	movups	xmmword ptr [rbp - 0x28], xmm0
141301b70: 48 8b 7d b0                 	mov	rdi, qword ptr [rbp - 0x50]
141301b74: 48 39 f7                    	cmp	rdi, rsi
141301b77: 0f 84 9e 00 00 00           	je	0x141301c1b <.text+0x1300c1b>
141301b7d: 48 8b 45 60                 	mov	rax, qword ptr [rbp + 0x60]
141301b81: 48 8b 4d 68                 	mov	rcx, qword ptr [rbp + 0x68]
141301b85: 48 8d 56 01                 	lea	rdx, [rsi + 0x1]
141301b89: f6 45 c0 01                 	test	byte ptr [rbp - 0x40], 0x1
141301b8d: 74 24                       	je	0x141301bb3 <.text+0x1300bb3>
141301b8f: 4c 8d 45 d8                 	lea	r8, [rbp - 0x28]
141301b93: 4c 8d 0c 76                 	lea	r9, [rsi + 2*rsi]
141301b97: 4c 8d 14 40                 	lea	r10, [rax + 2*rax]
141301b9b: 47 8b 5c 88 08              	mov	r11d, dword ptr [r8 + 4*r9 + 0x8]
141301ba0: 46 89 5c 91 08              	mov	dword ptr [rcx + 4*r10 + 0x8], r11d
141301ba5: 4f 8b 04 88                 	mov	r8, qword ptr [r8 + 4*r9]
141301ba9: 4e 89 04 91                 	mov	qword ptr [rcx + 4*r10], r8
141301bad: 48 ff c0                    	inc	rax
141301bb0: 48 89 d6                    	mov	rsi, rdx
141301bb3: 48 39 d7                    	cmp	rdi, rdx
141301bb6: 74 5f                       	je	0x141301c17 <.text+0x1300c17>
141301bb8: 48 8d 14 40                 	lea	rdx, [rax + 2*rax]
141301bbc: 48 8d 0c 91                 	lea	rcx, [rcx + 4*rdx]
141301bc0: 48 8d 14 76                 	lea	rdx, [rsi + 2*rsi]
141301bc4: 48 8d 14 95 e4 ff ff ff     	lea	rdx, [4*rdx - 0x1c]
141301bcc: 48 01 ea                    	add	rdx, rbp
141301bcf: 45 31 c0                    	xor	r8d, r8d
141301bd2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nop	word ptr cs:[rax + rax]
141301be0: 46 8b 4c 02 fc              	mov	r9d, dword ptr [rdx + r8 - 0x4]
141301be5: 46 89 4c 01 08              	mov	dword ptr [rcx + r8 + 0x8], r9d
141301bea: 4e 8b 4c 02 f4              	mov	r9, qword ptr [rdx + r8 - 0xc]
141301bef: 4e 89 0c 01                 	mov	qword ptr [rcx + r8], r9
141301bf3: 4e 8b 0c 02                 	mov	r9, qword ptr [rdx + r8]
141301bf7: 4e 89 4c 01 0c              	mov	qword ptr [rcx + r8 + 0xc], r9
141301bfc: 46 8b 4c 02 08              	mov	r9d, dword ptr [rdx + r8 + 0x8]
141301c01: 46 89 4c 01 14              	mov	dword ptr [rcx + r8 + 0x14], r9d
141301c06: 48 83 c0 02                 	add	rax, 0x2
141301c0a: 48 83 c6 02                 	add	rsi, 0x2
141301c0e: 49 83 c0 18                 	add	r8, 0x18
141301c12: 48 39 f7                    	cmp	rdi, rsi
141301c15: 75 c9                       	jne	0x141301be0 <.text+0x1300be0>
141301c17: 48 89 45 60                 	mov	qword ptr [rbp + 0x60], rax
141301c1b: 48 8b 45 58                 	mov	rax, qword ptr [rbp + 0x58]
141301c1f: 48 8b 4d 60                 	mov	rcx, qword ptr [rbp + 0x60]
141301c23: 48 89 08                    	mov	qword ptr [rax], rcx
141301c26: 48 8b 4d 50                 	mov	rcx, qword ptr [rbp + 0x50]
141301c2a: 48 8b 45 a8                 	mov	rax, qword ptr [rbp - 0x58]
141301c2e: 48 89 48 10                 	mov	qword ptr [rax + 0x10], rcx
141301c32: 0f 10 45 40                 	movups	xmm0, xmmword ptr [rbp + 0x40]
141301c36: 0f 11 00                    	movups	xmmword ptr [rax], xmm0
141301c39: 48 81 c4 08 01 00 00        	add	rsp, 0x108
141301c40: 5b                          	pop	rbx
141301c41: 5f                          	pop	rdi
141301c42: 5e                          	pop	rsi
141301c43: 41 5c                       	pop	r12
141301c45: 41 5d                       	pop	r13
141301c47: 41 5e                       	pop	r14
141301c49: 41 5f                       	pop	r15
141301c4b: 5d                          	pop	rbp
141301c4c: c3                          	ret
141301c4d: 48 c7 44 24 20 0c 00 00 00  	mov	qword ptr [rsp + 0x20], 0xc
141301c56: 48 8d 4d 40                 	lea	rcx, [rbp + 0x40]
141301c5a: 41 b9 04 00 00 00           	mov	r9d, 0x4
141301c60: 31 d2                       	xor	edx, edx
141301c62: 4d 89 f0                    	mov	r8, r14
141301c65: e8 f6 7d 5a 01              	call	0x1428a9a60 <.text+0x28a8a60>
141301c6a: 4c 8b 75 38                 	mov	r14, qword ptr [rbp + 0x38]
141301c6e: 48 8b 45 48                 	mov	rax, qword ptr [rbp + 0x48]
141301c72: 48 8b 4d 50                 	mov	rcx, qword ptr [rbp + 0x50]
141301c76: e9 a6 fd ff ff              	jmp	0x141301a21 <.text+0x1300a21>
141301c7b: 48 8d 05 26 6e 93 01        	lea	rax, [rip + 0x1936e26]  # 0x142c38aa8
141301c82: 48 89 45 c8                 	mov	qword ptr [rbp - 0x38], rax
141301c86: 48 c7 45 d0 01 00 00 00     	mov	qword ptr [rbp - 0x30], 0x1
141301c8e: 48 c7 45 d8 08 00 00 00     	mov	qword ptr [rbp - 0x28], 0x8
141301c96: 0f 57 c0                    	xorps	xmm0, xmm0
141301c99: 0f 11 45 e0                 	movups	xmmword ptr [rbp - 0x20], xmm0
141301c9d: 48 8d 4d c8                 	lea	rcx, [rbp - 0x38]
141301ca1: 48 8b 55 70                 	mov	rdx, qword ptr [rbp + 0x70]
141301ca5: e8 e6 76 5d 01              	call	0x1428d9390 <.text+0x28d8390>
141301caa: 48 8d 05 f7 6d 93 01        	lea	rax, [rip + 0x1936df7]  # 0x142c38aa8
141301cb1: 48 89 45 c8                 	mov	qword ptr [rbp - 0x38], rax
141301cb5: 48 c7 45 d0 01 00 00 00     	mov	qword ptr [rbp - 0x30], 0x1
141301cbd: 48 c7 45 d8 08 00 00 00     	mov	qword ptr [rbp - 0x28], 0x8
141301cc5: 0f 57 c0                    	xorps	xmm0, xmm0
141301cc8: 0f 11 45 e0                 	movups	xmmword ptr [rbp - 0x20], xmm0
141301ccc: 48 8d 4d c8                 	lea	rcx, [rbp - 0x38]
141301cd0: 48 8b 55 70                 	mov	rdx, qword ptr [rbp + 0x70]
141301cd4: e8 b7 76 5d 01              	call	0x1428d9390 <.text+0x28d8390>
141301cd9: 0f 0b                       	ud2
141301cdb: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
141301ce0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141301ce5: 55                          	push	rbp
141301ce6: 41 57                       	push	r15
141301ce8: 41 56                       	push	r14
141301cea: 41 55                       	push	r13
141301cec: 41 54                       	push	r12
141301cee: 56                          	push	rsi
141301cef: 57                          	push	rdi
141301cf0: 53                          	push	rbx
141301cf1: 48 83 ec 28                 	sub	rsp, 0x28
141301cf5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
141301cfc: 48 8b 45 58                 	mov	rax, qword ptr [rbp + 0x58]
141301d00: 48 8b 4d 60                 	mov	rcx, qword ptr [rbp + 0x60]
141301d04: 48 89 08                    	mov	qword ptr [rax], rcx
141301d07: 48 83 c4 28                 	add	rsp, 0x28
141301d0b: 5b                          	pop	rbx
141301d0c: 5f                          	pop	rdi
141301d0d: 5e                          	pop	rsi
141301d0e: 41 5c                       	pop	r12
141301d10: 41 5d                       	pop	r13
141301d12: 41 5e                       	pop	r14
141301d14: 41 5f                       	pop	r15
141301d16: 5d                          	pop	rbp
141301d17: c3                          	ret
141301d18: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
141301d20: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141301d25: 55                          	push	rbp
141301d26: 41 57                       	push	r15
141301d28: 41 56                       	push	r14
141301d2a: 41 55                       	push	r13
141301d2c: 41 54                       	push	r12
141301d2e: 56                          	push	rsi
141301d2f: 57                          	push	rdi
141301d30: 53                          	push	rbx
141301d31: 48 83 ec 28                 	sub	rsp, 0x28
141301d35: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
141301d3c: 48 8b 45 40                 	mov	rax, qword ptr [rbp + 0x40]
141301d40: 48 85 c0                    	test	rax, rax
141301d43: 74 17                       	je	0x141301d5c <.text+0x1300d5c>
141301d45: 48 8b 4d 48                 	mov	rcx, qword ptr [rbp + 0x48]
141301d49: 48 c1 e0 02                 	shl	rax, 0x2
141301d4d: 48 8d 14 40                 	lea	rdx, [rax + 2*rax]
141301d51: 41 b8 04 00 00 00           	mov	r8d, 0x4
141301d57: e8 c4 1e 31 ff              	call	0x140613c20 <.text+0x612c20>
141301d5c: 90                          	nop
141301d5d: 48 83 c4 28                 	add	rsp, 0x28
141301d61: 5b                          	pop	rbx
141301d62: 5f                          	pop	rdi
141301d63: 5e                          	pop	rsi
141301d64: 41 5c                       	pop	r12
141301d66: 41 5d                       	pop	r13
141301d68: 41 5e                       	pop	r14
141301d6a: 41 5f                       	pop	r15
141301d6c: 5d                          	pop	rbp
141301d6d: c3                          	ret
