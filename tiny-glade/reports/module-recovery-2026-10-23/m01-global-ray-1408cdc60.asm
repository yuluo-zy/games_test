
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408cdc60: 41 57                       	push	r15
1408cdc62: 41 56                       	push	r14
1408cdc64: 41 55                       	push	r13
1408cdc66: 41 54                       	push	r12
1408cdc68: 56                          	push	rsi
1408cdc69: 57                          	push	rdi
1408cdc6a: 55                          	push	rbp
1408cdc6b: 53                          	push	rbx
1408cdc6c: 48 81 ec 58 01 00 00        	sub	rsp, 0x158
1408cdc73: 44 0f 29 84 24 40 01 00 00  	movaps	xmmword ptr [rsp + 0x140], xmm8
1408cdc7c: 0f 29 bc 24 30 01 00 00     	movaps	xmmword ptr [rsp + 0x130], xmm7
1408cdc84: 0f 29 b4 24 20 01 00 00     	movaps	xmmword ptr [rsp + 0x120], xmm6
1408cdc8c: 48 89 d3                    	mov	rbx, rdx
1408cdc8f: 49 89 cd                    	mov	r13, rcx
1408cdc92: 8b bc 24 d0 01 00 00        	mov	edi, dword ptr [rsp + 0x1d0]
1408cdc99: f3 0f 10 b4 24 c8 01 00 00  	movss	xmm6, dword ptr [rsp + 0x1c8]
1408cdca2: 48 8b 84 24 c0 01 00 00     	mov	rax, qword ptr [rsp + 0x1c0]
1408cdcaa: 4c 89 4c 24 70              	mov	qword ptr [rsp + 0x70], r9
1408cdcaf: 48 89 44 24 78              	mov	qword ptr [rsp + 0x78], rax
1408cdcb4: 4c 8d 8a 80 3d 00 00        	lea	r9, [rdx + 0x3d80]
1408cdcbb: 41 0f 10 38                 	movups	xmm7, xmmword ptr [r8]
1408cdcbf: 4c 89 84 24 c0 00 00 00     	mov	qword ptr [rsp + 0xc0], r8
1408cdcc7: f2 45 0f 10 40 10           	movsd	xmm8, qword ptr [r8 + 0x10]
1408cdccd: 41 be 04 00 00 00           	mov	r14d, 0x4
1408cdcd3: bd 20 00 00 00              	mov	ebp, 0x20
1408cdcd8: 45 31 ff                    	xor	r15d, r15d
1408cdcdb: 4c 8d 54 24 70              	lea	r10, [rsp + 0x70]
1408cdce0: 48 8d 8c 24 c8 00 00 00     	lea	rcx, [rsp + 0xc8]
1408cdce8: 4c 8d 25 69 5b 1d 02        	lea	r12, [rip + 0x21d5b69]  # 0x142aa3858
1408cdcef: 4c 8d 9c 24 80 00 00 00     	lea	r11, [rsp + 0x80]
1408cdcf7: 48 89 44 24 58              	mov	qword ptr [rsp + 0x58], rax
1408cdcfc: 4c 89 f8                    	mov	rax, r15
1408cdcff: 90                          	nop
1408cdd00: 48 81 fd b0 3c 00 00        	cmp	rbp, 0x3cb0
1408cdd07: 0f 84 52 01 00 00           	je	0x1408cde5f <.text+0x8cce5f>
1408cdd0d: 48 81 c5 90 03 00 00        	add	rbp, 0x390
1408cdd14: 4c 8d 78 01                 	lea	r15, [rax + 0x1]
1408cdd18: 0f a3 c7                    	bt	edi, eax
1408cdd1b: 4c 89 f8                    	mov	rax, r15
1408cdd1e: 73 e0                       	jae	0x1408cdd00 <.text+0x8ccd00>
1408cdd20: 44 89 44 24 48              	mov	dword ptr [rsp + 0x48], r8d
1408cdd25: 89 54 24 4c                 	mov	dword ptr [rsp + 0x4c], edx
1408cdd29: 4c 89 6c 24 50              	mov	qword ptr [rsp + 0x50], r13
1408cdd2e: 4c 8d 2c 2b                 	lea	r13, [rbx + rbp]
1408cdd32: 49 81 c5 70 fc ff ff        	add	r13, -0x390
1408cdd39: 48 89 9c 24 c8 00 00 00     	mov	qword ptr [rsp + 0xc8], rbx
1408cdd41: 4c 89 ac 24 d0 00 00 00     	mov	qword ptr [rsp + 0xd0], r13
1408cdd49: 4c 89 94 24 d8 00 00 00     	mov	qword ptr [rsp + 0xd8], r10
1408cdd51: 48 8d 94 2b a0 fc ff ff     	lea	rdx, [rbx + rbp - 0x360]
1408cdd59: 0f 29 bc 24 00 01 00 00     	movaps	xmmword ptr [rsp + 0x100], xmm7
1408cdd61: 44 0f 13 84 24 10 01 00 00  	movlps	qword ptr [rsp + 0x110], xmm8
1408cdd6a: 48 89 8c 24 80 00 00 00     	mov	qword ptr [rsp + 0x80], rcx
1408cdd72: 4c 89 a4 24 88 00 00 00     	mov	qword ptr [rsp + 0x88], r12
1408cdd7a: 48 c7 84 24 90 00 00 00 00 00 00 00 	mov	qword ptr [rsp + 0x90], 0x0
1408cdd86: c7 84 24 a0 00 00 00 00 00 00 00    	mov	dword ptr [rsp + 0xa0], 0x0
1408cdd91: c7 84 24 ac 00 00 00 00 00 00 00    	mov	dword ptr [rsp + 0xac], 0x0
1408cdd9c: 4c 89 de                    	mov	rsi, r11
1408cdd9f: 4c 89 5c 24 38              	mov	qword ptr [rsp + 0x38], r11
1408cdda4: f3 0f 11 74 24 28           	movss	dword ptr [rsp + 0x28], xmm6
1408cddaa: 48 8d 84 24 00 01 00 00     	lea	rax, [rsp + 0x100]
1408cddb2: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1408cddb7: c6 44 24 30 01              	mov	byte ptr [rsp + 0x30], 0x1
1408cddbc: 48 8d 8c 24 e0 00 00 00     	lea	rcx, [rsp + 0xe0]
1408cddc4: 4d 89 cc                    	mov	r12, r9
1408cddc7: 4d 89 c8                    	mov	r8, r9
1408cddca: 4d 89 e9                    	mov	r9, r13
1408cddcd: e8 fe 67 2c 00              	call	0x140b945d0 <.text+0xb935d0>
1408cddd2: 83 bc 24 e8 00 00 00 04     	cmp	dword ptr [rsp + 0xe8], 0x4
1408cddda: 75 14                       	jne	0x1408cddf0 <.text+0x8ccdf0>
1408cdddc: 4c 8b 6c 24 50              	mov	r13, qword ptr [rsp + 0x50]
1408cdde1: 4d 89 e1                    	mov	r9, r12
1408cdde4: 8b 54 24 4c                 	mov	edx, dword ptr [rsp + 0x4c]
1408cdde8: 44 8b 44 24 48              	mov	r8d, dword ptr [rsp + 0x48]
1408cdded: eb 54                       	jmp	0x1408cde43 <.text+0x8cce43>
1408cddef: 90                          	nop
1408cddf0: 8b 94 24 e0 00 00 00        	mov	edx, dword ptr [rsp + 0xe0]
1408cddf7: 44 8b 84 24 e4 00 00 00     	mov	r8d, dword ptr [rsp + 0xe4]
1408cddff: 4c 8b b4 24 e8 00 00 00     	mov	r14, qword ptr [rsp + 0xe8]
1408cde07: f3 0f 10 b4 24 f0 00 00 00  	movss	xmm6, dword ptr [rsp + 0xf0]
1408cde10: 48 8d 8c 24 f4 00 00 00     	lea	rcx, [rsp + 0xf4]
1408cde18: 48 8b 01                    	mov	rax, qword ptr [rcx]
1408cde1b: 48 89 44 24 60              	mov	qword ptr [rsp + 0x60], rax
1408cde20: 8b 41 08                    	mov	eax, dword ptr [rcx + 0x8]
1408cde23: 89 44 24 68                 	mov	dword ptr [rsp + 0x68], eax
1408cde27: 4c 89 f0                    	mov	rax, r14
1408cde2a: 48 c1 e8 20                 	shr	rax, 0x20
1408cde2e: 48 89 44 24 58              	mov	qword ptr [rsp + 0x58], rax
1408cde33: 4c 89 ac 24 b8 00 00 00     	mov	qword ptr [rsp + 0xb8], r13
1408cde3b: 4c 8b 6c 24 50              	mov	r13, qword ptr [rsp + 0x50]
1408cde40: 4d 89 e1                    	mov	r9, r12
1408cde43: 4c 8d 54 24 70              	lea	r10, [rsp + 0x70]
1408cde48: 48 8d 8c 24 c8 00 00 00     	lea	rcx, [rsp + 0xc8]
1408cde50: 4c 8d 25 01 5a 1d 02        	lea	r12, [rip + 0x21d5a01]  # 0x142aa3858
1408cde57: 49 89 f3                    	mov	r11, rsi
1408cde5a: e9 9d fe ff ff              	jmp	0x1408cdcfc <.text+0x8cccfc>
1408cde5f: 48 8b 8c 24 b8 00 00 00     	mov	rcx, qword ptr [rsp + 0xb8]
1408cde67: 48 89 8c 24 80 00 00 00     	mov	qword ptr [rsp + 0x80], rcx
1408cde6f: 89 94 24 88 00 00 00        	mov	dword ptr [rsp + 0x88], edx
1408cde76: 44 89 84 24 8c 00 00 00     	mov	dword ptr [rsp + 0x8c], r8d
1408cde7e: 44 89 b4 24 90 00 00 00     	mov	dword ptr [rsp + 0x90], r14d
1408cde86: 48 8b 44 24 58              	mov	rax, qword ptr [rsp + 0x58]
1408cde8b: 89 84 24 94 00 00 00        	mov	dword ptr [rsp + 0x94], eax
1408cde92: f3 0f 11 b4 24 98 00 00 00  	movss	dword ptr [rsp + 0x98], xmm6
1408cde9b: 48 8b 44 24 60              	mov	rax, qword ptr [rsp + 0x60]
1408cdea0: 48 89 84 24 9c 00 00 00     	mov	qword ptr [rsp + 0x9c], rax
1408cdea8: 8b 44 24 68                 	mov	eax, dword ptr [rsp + 0x68]
1408cdeac: 89 84 24 a4 00 00 00        	mov	dword ptr [rsp + 0xa4], eax
1408cdeb3: 41 83 fe 04                 	cmp	r14d, 0x4
1408cdeb7: 75 0a                       	jne	0x1408cdec3 <.text+0x8ccec3>
1408cdeb9: 41 c7 45 34 02 00 00 00     	mov	dword ptr [r13 + 0x34], 0x2
1408cdec1: eb 5d                       	jmp	0x1408cdf20 <.text+0x8ccf20>
1408cdec3: e8 78 5a 2c 00              	call	0x140b93940 <.text+0xb92940>
1408cdec8: 48 85 c0                    	test	rax, rax
1408cdecb: 0f 84 94 00 00 00           	je	0x1408cdf65 <.text+0x8ccf65>
1408cded1: 48 8b 08                    	mov	rcx, qword ptr [rax]
1408cded4: 41 89 c8                    	mov	r8d, ecx
1408cded7: 4c 39 43 10                 	cmp	qword ptr [rbx + 0x10], r8
1408cdedb: 76 70                       	jbe	0x1408cdf4d <.text+0x8ccf4d>
1408cdedd: 48 8b 53 08                 	mov	rdx, qword ptr [rbx + 0x8]
1408cdee1: 48 85 d2                    	test	rdx, rdx
1408cdee4: 74 67                       	je	0x1408cdf4d <.text+0x8ccf4d>
1408cdee6: 48 c1 e9 20                 	shr	rcx, 0x20
1408cdeea: 83 c9 01                    	or	ecx, 0x1
1408cdeed: 4f 8d 0c 80                 	lea	r9, [r8 + 4*r8]
1408cdef1: 42 39 4c ca 20              	cmp	dword ptr [rdx + 8*r9 + 0x20], ecx
1408cdef6: 75 55                       	jne	0x1408cdf4d <.text+0x8ccf4d>
1408cdef8: 4c 8d 84 24 90 00 00 00     	lea	r8, [rsp + 0x90]
1408cdf00: 4a 8d 0c ca                 	lea	rcx, [rdx + 8*r9]
1408cdf04: 48 83 c0 34                 	add	rax, 0x34
1408cdf08: 48 89 4c 24 20              	mov	qword ptr [rsp + 0x20], rcx
1408cdf0d: 4c 89 e9                    	mov	rcx, r13
1408cdf10: 48 8b 94 24 c0 00 00 00     	mov	rdx, qword ptr [rsp + 0xc0]
1408cdf18: 49 89 c1                    	mov	r9, rax
1408cdf1b: e8 80 05 00 00              	call	0x1408ce4a0 <.text+0x8cd4a0>
1408cdf20: 0f 28 b4 24 20 01 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x120]
1408cdf28: 0f 28 bc 24 30 01 00 00     	movaps	xmm7, xmmword ptr [rsp + 0x130]
1408cdf30: 44 0f 28 84 24 40 01 00 00  	movaps	xmm8, xmmword ptr [rsp + 0x140]
1408cdf39: 48 81 c4 58 01 00 00        	add	rsp, 0x158
1408cdf40: 5b                          	pop	rbx
1408cdf41: 5d                          	pop	rbp
1408cdf42: 5f                          	pop	rdi
1408cdf43: 5e                          	pop	rsi
1408cdf44: 41 5c                       	pop	r12
1408cdf46: 41 5d                       	pop	r13
1408cdf48: 41 5e                       	pop	r14
1408cdf4a: 41 5f                       	pop	r15
1408cdf4c: c3                          	ret
1408cdf4d: 48 8d 0d 8c f9 39 02        	lea	rcx, [rip + 0x239f98c]  # 0x142c6d8e0
1408cdf54: 4c 8d 05 4d 5d 1d 02        	lea	r8, [rip + 0x21d5d4d]   # 0x142aa3ca8
1408cdf5b: ba 18 00 00 00              	mov	edx, 0x18
1408cdf60: e8 c0 a7 00 02              	call	0x1428d8725 <.text+0x28d7725>
1408cdf65: 48 8d 0d 24 5d 1d 02        	lea	rcx, [rip + 0x21d5d24]  # 0x142aa3c90
1408cdf6c: e8 9f b3 00 02              	call	0x1428d9310 <.text+0x28d8310>
1408cdf71: cc                          	int3
