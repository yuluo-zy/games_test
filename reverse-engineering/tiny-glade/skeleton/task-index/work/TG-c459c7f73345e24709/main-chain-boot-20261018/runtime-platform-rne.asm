
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140594b00: 48 83 f8 04                 	cmp	rax, 0x4
140594b04: 0f 82 d7 00 00 00           	jb	0x140594be1 <.text+0x593be1>
140594b0a: c6 43 76 01                 	mov	byte ptr [rbx + 0x76], 0x1
140594b0e: c6 43 77 01                 	mov	byte ptr [rbx + 0x77], 0x1
140594b12: c6 43 78 01                 	mov	byte ptr [rbx + 0x78], 0x1
140594b16: c6 43 79 01                 	mov	byte ptr [rbx + 0x79], 0x1
140594b1a: 48 8d 0d 57 aa 46 02        	lea	rcx, [rip + 0x246aa57]  # 0x1429ff578
140594b21: e8 5a 92 0e 00              	call	0x14067dd80 <.text+0x67cd80>
140594b26: f3 0f 6f 00                 	movdqu	xmm0, xmmword ptr [rax]
140594b2a: 8b 40 10                    	mov	eax, dword ptr [rax + 0x10]
140594b2d: 48 c7 83 b0 05 00 00 04 00 00 00    	mov	qword ptr [rbx + 0x5b0], 0x4
140594b38: 48 8d 0d b1 a8 46 02        	lea	rcx, [rip + 0x246a8b1]  # 0x1429ff3f0
140594b3f: 48 89 8b b8 05 00 00        	mov	qword ptr [rbx + 0x5b8], rcx
140594b46: 48 c7 83 c0 05 00 00 14 00 00 00    	mov	qword ptr [rbx + 0x5c0], 0x14
140594b51: 48 8d 15 10 aa 46 02        	lea	rdx, [rip + 0x246aa10]  # 0x1429ff568
140594b58: 48 89 93 d0 05 00 00        	mov	qword ptr [rbx + 0x5d0], rdx
140594b5f: 48 c7 83 d8 05 00 00 01 00 00 00    	mov	qword ptr [rbx + 0x5d8], 0x1
140594b6a: 48 c7 83 e0 05 00 00 08 00 00 00    	mov	qword ptr [rbx + 0x5e0], 0x8
140594b75: 48 c7 83 80 05 00 00 00 00 00 00    	mov	qword ptr [rbx + 0x580], 0x0
140594b80: 66 0f ef c9                 	pxor	xmm1, xmm1
140594b84: f3 0f 7f 8b e8 05 00 00     	movdqu	xmmword ptr [rbx + 0x5e8], xmm1
140594b8c: 48 89 8b 88 05 00 00        	mov	qword ptr [rbx + 0x588], rcx
140594b93: 48 c7 83 90 05 00 00 14 00 00 00    	mov	qword ptr [rbx + 0x590], 0x14
140594b9e: 48 c7 83 98 05 00 00 00 00 00 00    	mov	qword ptr [rbx + 0x598], 0x0
140594ba9: f3 0f 7f 83 a0 05 00 00     	movdqu	xmmword ptr [rbx + 0x5a0], xmm0
140594bb1: c7 83 c8 05 00 00 01 00 00 00       	mov	dword ptr [rbx + 0x5c8], 0x1
140594bbb: 89 83 cc 05 00 00           	mov	dword ptr [rbx + 0x5cc], eax
140594bc1: c6 43 76 01                 	mov	byte ptr [rbx + 0x76], 0x1
140594bc5: c6 43 77 01                 	mov	byte ptr [rbx + 0x77], 0x1
140594bc9: c6 43 78 01                 	mov	byte ptr [rbx + 0x78], 0x1
140594bcd: c6 43 79 01                 	mov	byte ptr [rbx + 0x79], 0x1
140594bd1: 48 8d 4b 4c                 	lea	rcx, [rbx + 0x4c]
140594bd5: 48 8d 93 80 05 00 00        	lea	rdx, [rbx + 0x580]
140594bdc: e8 4f 57 7b 00              	call	0x140d4a330 <.text+0xd49330>
140594be1: 48 85 f6                    	test	rsi, rsi
140594be4: 74 24                       	je	0x140594c0a <.text+0x593c0a>
140594be6: c6 43 76 01                 	mov	byte ptr [rbx + 0x76], 0x1
140594bea: c6 43 77 01                 	mov	byte ptr [rbx + 0x77], 0x1
140594bee: c6 43 78 00                 	mov	byte ptr [rbx + 0x78], 0x0
140594bf2: c6 43 79 01                 	mov	byte ptr [rbx + 0x79], 0x1
140594bf6: 48 8d 8b 98 50 00 00        	lea	rcx, [rbx + 0x5098]
140594bfd: 48 89 f2                    	mov	rdx, rsi
140594c00: e8 1b 07 3b 00              	call	0x140945320 <.text+0x944320>
140594c05: 49 89 c6                    	mov	r14, rax
140594c08: eb 03                       	jmp	0x140594c0d <.text+0x593c0d>
140594c0a: 45 31 f6                    	xor	r14d, r14d
140594c0d: 48 b8 12 00 00 00 00 00 00 80       	movabs	rax, -0x7fffffffffffffee
140594c17: 48 39 83 90 04 00 00        	cmp	qword ptr [rbx + 0x490], rax
140594c1e: 0f 94 c0                    	sete	al
140594c21: 22 83 98 04 00 00           	and	al, byte ptr [rbx + 0x498]
140594c27: 4c 89 b3 68 01 00 00        	mov	qword ptr [rbx + 0x168], r14
140594c2e: 40 b6 02                    	mov	sil, 0x2
140594c31: 40 28 c6                    	sub	sil, al
140594c34: 4d 85 f6                    	test	r14, r14
140594c37: 74 77                       	je	0x140594cb0 <.text+0x593cb0>
140594c39: e8 32 e9 38 02              	call	0x142923570 <.text+0x2922570>
140594c3e: f0                          	lock
140594c3f: 49 ff 06                    	inc	qword ptr [r14]
140594c42: 0f 8e 29 4d 00 00           	jle	0x140599971 <.text+0x598971>
140594c48: 4c 89 b3 80 05 00 00        	mov	qword ptr [rbx + 0x580], r14
140594c4f: 48 89 83 88 05 00 00        	mov	qword ptr [rbx + 0x588], rax
140594c56: 48 89 c1                    	mov	rcx, rax
140594c59: e8 92 e8 38 02              	call	0x1429234f0 <.text+0x29224f0>
140594c5e: 84 c0                       	test	al, al
140594c60: 74 29                       	je	0x140594c8b <.text+0x593c8b>
140594c62: 48 8b 83 80 05 00 00        	mov	rax, qword ptr [rbx + 0x580]
140594c69: f0                          	lock
140594c6a: 48 ff 08                    	dec	qword ptr [rax]
140594c6d: 75 18                       	jne	0x140594c87 <.text+0x593c87>
140594c6f: c6 43 58 01                 	mov	byte ptr [rbx + 0x58], 0x1
140594c73: c6 43 59 01                 	mov	byte ptr [rbx + 0x59], 0x1
140594c77: c6 43 5a 01                 	mov	byte ptr [rbx + 0x5a], 0x1
140594c7b: 48 8d 8b 80 05 00 00        	lea	rcx, [rbx + 0x580]
140594c82: e8 c9 06 5f 00              	call	0x140b85350 <.text+0xb84350>
140594c87: 31 f6                       	xor	esi, esi
140594c89: eb 25                       	jmp	0x140594cb0 <.text+0x593cb0>
140594c8b: 48 8b 83 80 05 00 00        	mov	rax, qword ptr [rbx + 0x580]
140594c92: f0                          	lock
140594c93: 48 ff 08                    	dec	qword ptr [rax]
140594c96: 75 18                       	jne	0x140594cb0 <.text+0x593cb0>
140594c98: c6 43 58 01                 	mov	byte ptr [rbx + 0x58], 0x1
140594c9c: c6 43 59 01                 	mov	byte ptr [rbx + 0x59], 0x1
140594ca0: c6 43 5a 01                 	mov	byte ptr [rbx + 0x5a], 0x1
140594ca4: 48 8d 8b 80 05 00 00        	lea	rcx, [rbx + 0x580]
140594cab: e8 a0 06 5f 00              	call	0x140b85350 <.text+0xb84350>
140594cb0: c6 43 58 01                 	mov	byte ptr [rbx + 0x58], 0x1
140594cb4: c6 43 59 01                 	mov	byte ptr [rbx + 0x59], 0x1
140594cb8: c6 43 5a 01                 	mov	byte ptr [rbx + 0x5a], 0x1
140594cbc: 48 8d 8b 98 50 00 00        	lea	rcx, [rbx + 0x5098]
140594cc3: 89 f2                       	mov	edx, esi
140594cc5: e8 06 55 ba ff              	call	0x14013a1d0 <.text+0x1391d0>
140594cca: 48 8b 05 d7 76 09 03        	mov	rax, qword ptr [rip + 0x30976d7] # 0x14362c3a8
