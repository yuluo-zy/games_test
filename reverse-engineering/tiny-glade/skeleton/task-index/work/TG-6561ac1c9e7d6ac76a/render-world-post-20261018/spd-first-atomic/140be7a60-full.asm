
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140be7a60: 56                          	push	rsi
140be7a61: 48 83 ec 60                 	sub	rsp, 0x60
140be7a65: 48 89 ce                    	mov	rsi, rcx
140be7a68: 44 89 c9                    	mov	ecx, r9d
140be7a6b: 48 8b 42 10                 	mov	rax, qword ptr [rdx + 0x10]
140be7a6f: 48 39 c8                    	cmp	rax, rcx
140be7a72: 0f 86 e5 00 00 00           	jbe	0x140be7b5d <.text+0xbe6b5d>
140be7a78: 4c 8b 4a 08                 	mov	r9, qword ptr [rdx + 0x8]
140be7a7c: 48 8b 42 40                 	mov	rax, qword ptr [rdx + 0x40]
140be7a80: 41 0f b7 0c 89              	movzx	ecx, word ptr [r9 + 4*rcx]
140be7a85: 48 39 c8                    	cmp	rax, rcx
140be7a88: 0f 86 de 00 00 00           	jbe	0x140be7b6c <.text+0xbe6b6c>
140be7a8e: 48 8b 52 38                 	mov	rdx, qword ptr [rdx + 0x38]
140be7a92: c1 e1 06                    	shl	ecx, 0x6
140be7a95: 48 8b 04 0a                 	mov	rax, qword ptr [rdx + rcx]
140be7a99: 48 83 f8 02                 	cmp	rax, 0x2
140be7a9d: 0f 83 ab 00 00 00           	jae	0x140be7b4e <.text+0xbe6b4e>
140be7aa3: 48 01 ca                    	add	rdx, rcx
140be7aa6: 49 8b 80 48 06 00 00        	mov	rax, qword ptr [r8 + 0x648]
140be7aad: 0f b7 4a 1c                 	movzx	ecx, word ptr [rdx + 0x1c]
140be7ab1: 66 3b 88 50 0a 00 00        	cmp	cx, word ptr [rax + 0xa50]
140be7ab8: 75 3a                       	jne	0x140be7af4 <.text+0xbe6af4>
140be7aba: 80 7a 1e 00                 	cmp	byte ptr [rdx + 0x1e], 0x0
140be7abe: 75 62                       	jne	0x140be7b22 <.text+0xbe6b22>
140be7ac0: 48 8b 42 10                 	mov	rax, qword ptr [rdx + 0x10]
140be7ac4: 8b 48 0c                    	mov	ecx, dword ptr [rax + 0xc]
140be7ac7: 3b 4a 18                    	cmp	ecx, dword ptr [rdx + 0x18]
140be7aca: 0f 85 ab 00 00 00           	jne	0x140be7b7b <.text+0xbe6b7b>
140be7ad0: 4c 8b 8c 24 90 00 00 00     	mov	r9, qword ptr [rsp + 0x90]
140be7ad8: 48 83 c0 10                 	add	rax, 0x10
140be7adc: 49 83 c0 78                 	add	r8, 0x78
140be7ae0: 48 89 f1                    	mov	rcx, rsi
140be7ae3: 48 89 c2                    	mov	rdx, rax
140be7ae6: e8 e5 4a ff ff              	call	0x140bdc5d0 <.text+0xbdb5d0>
140be7aeb: 48 89 f0                    	mov	rax, rsi
140be7aee: 48 83 c4 60                 	add	rsp, 0x60
140be7af2: 5e                          	pop	rsi
140be7af3: c3                          	ret
140be7af4: 48 c7 44 24 30 00 00 00 00  	mov	qword ptr [rsp + 0x30], 0x0
140be7afd: 48 83 c2 1c                 	add	rdx, 0x1c
140be7b01: 48 05 50 0a 00 00           	add	rax, 0xa50
140be7b07: 48 8d 0d 92 a7 11 02        	lea	rcx, [rip + 0x211a792]  # 0x142d022a0
140be7b0e: 48 89 4c 24 20              	mov	qword ptr [rsp + 0x20], rcx
140be7b13: 4c 8d 4c 24 30              	lea	r9, [rsp + 0x30]
140be7b18: 31 c9                       	xor	ecx, ecx
140be7b1a: 49 89 c0                    	mov	r8, rax
140be7b1d: e8 f4 2c ce 01              	call	0x1428ca816 <.text+0x28c9816>
140be7b22: 48 c7 44 24 30 00 00 00 00  	mov	qword ptr [rsp + 0x30], 0x0
140be7b2b: 48 83 c2 1e                 	add	rdx, 0x1e
140be7b2f: 48 8d 05 8a a7 11 02        	lea	rax, [rip + 0x211a78a]  # 0x142d022c0
140be7b36: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
140be7b3b: 4c 8d 05 76 a7 11 02        	lea	r8, [rip + 0x211a776]   # 0x142d022b8
140be7b42: 4c 8d 4c 24 30              	lea	r9, [rsp + 0x30]
140be7b47: 31 c9                       	xor	ecx, ecx
140be7b49: e8 b4 2d ce 01              	call	0x1428ca902 <.text+0x28c9902>
140be7b4e: 48 83 c0 fe                 	add	rax, -0x2
140be7b52: 48 83 f8 03                 	cmp	rax, 0x3
140be7b56: 73 2f                       	jae	0x140be7b87 <.text+0xbe6b87>
140be7b58: e8 23 2f ce 01              	call	0x1428caa80 <.text+0x28c9a80>
140be7b5d: 4c 8d 05 04 c2 f2 01        	lea	r8, [rip + 0x1f2c204]   # 0x142b13d68
140be7b64: 48 89 c2                    	mov	rdx, rax
140be7b67: e8 ac 19 cf 01              	call	0x1428d9518 <.text+0x28d8518>
140be7b6c: 4c 8d 05 0d c2 f2 01        	lea	r8, [rip + 0x1f2c20d]   # 0x142b13d80
140be7b73: 48 89 c2                    	mov	rdx, rax
140be7b76: e8 9d 19 cf 01              	call	0x1428d9518 <.text+0x28d8518>
140be7b7b: 48 8d 0d 2e c2 f2 01        	lea	rcx, [rip + 0x1f2c22e]  # 0x142b13db0
140be7b82: e8 89 17 cf 01              	call	0x1428d9310 <.text+0x28d8310>
140be7b87: 48 8d 0d 3a 86 f2 01        	lea	rcx, [rip + 0x1f2863a]  # 0x142b101c8
140be7b8e: 4c 8d 05 03 c2 f2 01        	lea	r8, [rip + 0x1f2c203]   # 0x142b13d98
140be7b95: ba 28 00 00 00              	mov	edx, 0x28
140be7b9a: e8 91 18 cf 01              	call	0x1428d9430 <.text+0x28d8430>
140be7b9f: cc                          	int3
