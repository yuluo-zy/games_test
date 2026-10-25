
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b67b50: 56                          	push	rsi
140b67b51: 57                          	push	rdi
140b67b52: 53                          	push	rbx
140b67b53: 48 81 ec c0 00 00 00        	sub	rsp, 0xc0
140b67b5a: 48 89 ce                    	mov	rsi, rcx
140b67b5d: 48 83 c1 38                 	add	rcx, 0x38
140b67b61: 48 ba 84 a7 2b 96 51 d2 1b 8e       	movabs	rdx, -0x71e42dae69d4587c
140b67b6b: 49 b8 9b 52 e6 c0 74 23 78 6a       	movabs	r8, 0x6a782374c0e6529b
140b67b75: e8 76 5b 57 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
140b67b7a: 48 85 c0                    	test	rax, rax
140b67b7d: 0f 84 9b 00 00 00           	je	0x140b67c1e <.text+0xb66c1e>
140b67b83: 48 39 96 18 02 00 00        	cmp	qword ptr [rsi + 0x218], rdx
140b67b8a: 0f 86 8e 00 00 00           	jbe	0x140b67c1e <.text+0xb66c1e>
140b67b90: 48 8b 86 10 02 00 00        	mov	rax, qword ptr [rsi + 0x210]
140b67b97: 48 8b 0c d0                 	mov	rcx, qword ptr [rax + 8*rdx]
140b67b9b: 48 85 c9                    	test	rcx, rcx
140b67b9e: 74 7e                       	je	0x140b67c1e <.text+0xb66c1e>
140b67ba0: 48 8b 86 e0 01 00 00        	mov	rax, qword ptr [rsi + 0x1e0]
140b67ba7: 48 f7 d1                    	not	rcx
140b67baa: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
140b67bae: 48 c1 e1 05                 	shl	rcx, 0x5
140b67bb2: 48 83 7c 08 20 00           	cmp	qword ptr [rax + rcx + 0x20], 0x0
140b67bb8: 74 64                       	je	0x140b67c1e <.text+0xb66c1e>
140b67bba: 48 01 c8                    	add	rax, rcx
140b67bbd: 48 8b 48 10                 	mov	rcx, qword ptr [rax + 0x10]
140b67bc1: 8b 96 b0 05 00 00           	mov	edx, dword ptr [rsi + 0x5b0]
140b67bc7: 89 50 5c                    	mov	dword ptr [rax + 0x5c], edx
140b67bca: e8 51 a0 b3 00              	call	0x1416a1c20 <.text+0x16a0c20>
140b67bcf: 48 89 c7                    	mov	rdi, rax
140b67bd2: 48 8d 5c 24 50              	lea	rbx, [rsp + 0x50]
140b67bd7: 48 89 d9                    	mov	rcx, rbx
140b67bda: e8 d1 3c e3 ff              	call	0x14099b8b0 <.text+0x99a8b0>
140b67bdf: 48 8d 4c 24 30              	lea	rcx, [rsp + 0x30]
140b67be4: 48 89 fa                    	mov	rdx, rdi
140b67be7: 49 89 d8                    	mov	r8, rbx
140b67bea: 45 31 c9                    	xor	r9d, r9d
140b67bed: e8 3e 71 b3 00              	call	0x14169ed30 <.text+0x169dd30>
140b67bf2: 48 8b 54 24 30              	mov	rdx, qword ptr [rsp + 0x30]
140b67bf7: 48 85 d2                    	test	rdx, rdx
140b67bfa: 74 14                       	je	0x140b67c10 <.text+0xb66c10>
140b67bfc: 48 8b 4c 24 38              	mov	rcx, qword ptr [rsp + 0x38]
140b67c01: 48 c1 e2 04                 	shl	rdx, 0x4
140b67c05: 41 b8 08 00 00 00           	mov	r8d, 0x8
140b67c0b: e8 10 c0 aa ff              	call	0x140613c20 <.text+0x612c20>
140b67c10: 48 89 f0                    	mov	rax, rsi
140b67c13: 48 81 c4 c0 00 00 00        	add	rsp, 0xc0
140b67c1a: 5b                          	pop	rbx
140b67c1b: 5f                          	pop	rdi
140b67c1c: 5e                          	pop	rsi
140b67c1d: c3                          	ret
140b67c1e: 48 8d 05 9b b0 f8 01        	lea	rax, [rip + 0x1f8b09b]  # 0x142af2cc0
140b67c25: 48 89 44 24 30              	mov	qword ptr [rsp + 0x30], rax
140b67c2a: 48 c7 44 24 38 27 00 00 00  	mov	qword ptr [rsp + 0x38], 0x27
140b67c33: 48 8d 44 24 30              	lea	rax, [rsp + 0x30]
140b67c38: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
140b67c3d: 48 8d 05 4c 22 5a ff        	lea	rax, [rip - 0xa5ddb4]   # 0x140109e90 <.text+0x108e90>
140b67c44: 48 89 44 24 28              	mov	qword ptr [rsp + 0x28], rax
140b67c49: 48 8d 05 a0 b1 f8 01        	lea	rax, [rip + 0x1f8b1a0]  # 0x142af2df0
140b67c50: 48 89 44 24 50              	mov	qword ptr [rsp + 0x50], rax
140b67c55: 48 c7 44 24 58 02 00 00 00  	mov	qword ptr [rsp + 0x58], 0x2
140b67c5e: 48 c7 44 24 70 00 00 00 00  	mov	qword ptr [rsp + 0x70], 0x0
140b67c67: 48 8d 44 24 20              	lea	rax, [rsp + 0x20]
140b67c6c: 48 89 44 24 60              	mov	qword ptr [rsp + 0x60], rax
140b67c71: 48 c7 44 24 68 01 00 00 00  	mov	qword ptr [rsp + 0x68], 0x1
140b67c7a: 48 8d 15 27 b0 f8 01        	lea	rdx, [rip + 0x1f8b027]  # 0x142af2ca8
140b67c81: 48 8d 4c 24 50              	lea	rcx, [rsp + 0x50]
140b67c86: e8 05 17 d7 01              	call	0x1428d9390 <.text+0x28d8390>
140b67c8b: cc                          	int3
