
/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14090e080: 55                          	push	rbp
14090e081: 56                          	push	rsi
14090e082: 48 81 ec 88 00 00 00        	sub	rsp, 0x88
14090e089: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
14090e091: 48 c7 45 00 fe ff ff ff     	mov	qword ptr [rbp], -0x2
14090e099: 48 89 d6                    	mov	rsi, rdx
14090e09c: 48 c7 45 a8 00 00 00 00     	mov	qword ptr [rbp - 0x58], 0x0
14090e0a4: 48 c7 45 b0 08 00 00 00     	mov	qword ptr [rbp - 0x50], 0x8
14090e0ac: 48 c7 45 b8 00 00 00 00     	mov	qword ptr [rbp - 0x48], 0x0
14090e0b4: 48 c7 45 d8 01 00 00 00     	mov	qword ptr [rbp - 0x28], 0x1
14090e0bc: 48 8d 05 cd 81 1c 02        	lea	rax, [rip + 0x21c81cd]  # 0x142ad6290
14090e0c3: 48 89 45 e0                 	mov	qword ptr [rbp - 0x20], rax
14090e0c7: 48 c7 45 c0 00 00 00 00     	mov	qword ptr [rbp - 0x40], 0x0
14090e0cf: 48 c7 45 c8 08 00 00 00     	mov	qword ptr [rbp - 0x38], 0x8
14090e0d7: 48 c7 45 d0 00 00 00 00     	mov	qword ptr [rbp - 0x30], 0x0
14090e0df: c6 45 e8 00                 	mov	byte ptr [rbp - 0x18], 0x0
14090e0e3: 48 8d 15 c6 d5 19 02        	lea	rdx, [rip + 0x219d5c6]  # 0x142aab6b0
14090e0ea: 48 8d 4d a8                 	lea	rcx, [rbp - 0x58]
14090e0ee: e8 fd 48 c5 ff              	call	0x1405629f0 <.text+0x5619f0>
14090e0f3: 48 8b 45 b0                 	mov	rax, qword ptr [rbp - 0x50]
14090e0f7: 0f 10 45 c0                 	movups	xmm0, xmmword ptr [rbp - 0x40]
14090e0fb: 0f 10 4d d0                 	movups	xmm1, xmmword ptr [rbp - 0x30]
14090e0ff: 0f 10 55 e0                 	movups	xmm2, xmmword ptr [rbp - 0x20]
14090e103: 0f 11 50 20                 	movups	xmmword ptr [rax + 0x20], xmm2
14090e107: 0f 11 48 10                 	movups	xmmword ptr [rax + 0x10], xmm1
14090e10b: 0f 11 00                    	movups	xmmword ptr [rax], xmm0
14090e10e: 48 c7 45 b8 01 00 00 00     	mov	qword ptr [rbp - 0x48], 0x1
14090e116: 80 3d 73 6c d2 02 00        	cmp	byte ptr [rip + 0x2d26c73], 0x0 # 0x143634d90
14090e11d: 0f 85 9e 00 00 00           	jne	0x14090e1c1 <.text+0x90d1c1>
14090e123: 48 c7 45 d8 01 00 00 00     	mov	qword ptr [rbp - 0x28], 0x1
14090e12b: 48 8d 0d 2e 81 1c 02        	lea	rcx, [rip + 0x21c812e]  # 0x142ad6260
14090e132: 48 89 4d e0                 	mov	qword ptr [rbp - 0x20], rcx
14090e136: 48 c7 45 c0 00 00 00 00     	mov	qword ptr [rbp - 0x40], 0x0
14090e13e: 48 c7 45 c8 08 00 00 00     	mov	qword ptr [rbp - 0x38], 0x8
14090e146: 48 c7 45 d0 00 00 00 00     	mov	qword ptr [rbp - 0x30], 0x0
14090e14e: c6 45 e8 00                 	mov	byte ptr [rbp - 0x18], 0x0
14090e152: 48 83 7d a8 01              	cmp	qword ptr [rbp - 0x58], 0x1
14090e157: 75 14                       	jne	0x14090e16d <.text+0x90d16d>
14090e159: 48 8d 15 68 d5 19 02        	lea	rdx, [rip + 0x219d568]  # 0x142aab6c8
14090e160: 48 8d 4d a8                 	lea	rcx, [rbp - 0x58]
14090e164: e8 87 48 c5 ff              	call	0x1405629f0 <.text+0x5619f0>
14090e169: 48 8b 45 b0                 	mov	rax, qword ptr [rbp - 0x50]
14090e16d: 48 8b 4d c0                 	mov	rcx, qword ptr [rbp - 0x40]
14090e171: 48 8b 55 c8                 	mov	rdx, qword ptr [rbp - 0x38]
14090e175: 4c 8b 45 d0                 	mov	r8, qword ptr [rbp - 0x30]
14090e179: 4c 8b 4d d8                 	mov	r9, qword ptr [rbp - 0x28]
14090e17d: 4c 8b 55 e0                 	mov	r10, qword ptr [rbp - 0x20]
14090e181: 4c 89 50 50                 	mov	qword ptr [rax + 0x50], r10
14090e185: 44 0f b6 55 e8              	movzx	r10d, byte ptr [rbp - 0x18]
14090e18a: 44 88 50 58                 	mov	byte ptr [rax + 0x58], r10b
14090e18e: 44 8b 55 e9                 	mov	r10d, dword ptr [rbp - 0x17]
14090e192: 44 89 50 59                 	mov	dword ptr [rax + 0x59], r10d
14090e196: 44 0f b7 55 ed              	movzx	r10d, word ptr [rbp - 0x13]
14090e19b: 66 44 89 50 5d              	mov	word ptr [rax + 0x5d], r10w
14090e1a0: 44 0f b6 55 ef              	movzx	r10d, byte ptr [rbp - 0x11]
14090e1a5: 44 88 50 5f                 	mov	byte ptr [rax + 0x5f], r10b
14090e1a9: 4c 89 40 40                 	mov	qword ptr [rax + 0x40], r8
14090e1ad: 4c 89 48 48                 	mov	qword ptr [rax + 0x48], r9
14090e1b1: 48 89 48 30                 	mov	qword ptr [rax + 0x30], rcx
14090e1b5: 48 89 50 38                 	mov	qword ptr [rax + 0x38], rdx
14090e1b9: 48 c7 45 b8 02 00 00 00     	mov	qword ptr [rbp - 0x48], 0x2
14090e1c1: 48 8b 45 b8                 	mov	rax, qword ptr [rbp - 0x48]
14090e1c5: 48 89 45 d0                 	mov	qword ptr [rbp - 0x30], rax
14090e1c9: 0f 10 45 a8                 	movups	xmm0, xmmword ptr [rbp - 0x58]
14090e1cd: 0f 29 45 c0                 	movaps	xmmword ptr [rbp - 0x40], xmm0
14090e1d1: c6 45 ff 00                 	mov	byte ptr [rbp - 0x1], 0x0
14090e1d5: 48 8d 55 c0                 	lea	rdx, [rbp - 0x40]
14090e1d9: 48 89 f1                    	mov	rcx, rsi
14090e1dc: e8 cf ed c3 00              	call	0x14154cfb0 <.text+0x154bfb0>
14090e1e1: 90                          	nop
14090e1e2: 48 81 c4 88 00 00 00        	add	rsp, 0x88
14090e1e9: 5e                          	pop	rsi
14090e1ea: 5d                          	pop	rbp
14090e1eb: c3                          	ret
14090e1ec: 0f 1f 40 00                 	nop	dword ptr [rax]
14090e1f0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14090e1f5: 55                          	push	rbp
14090e1f6: 56                          	push	rsi
14090e1f7: 48 83 ec 28                 	sub	rsp, 0x28
14090e1fb: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14090e202: 80 7d ff 00                 	cmp	byte ptr [rbp - 0x1], 0x0
14090e206: 75 07                       	jne	0x14090e20f <.text+0x90d20f>
14090e208: 48 83 c4 28                 	add	rsp, 0x28
14090e20c: 5e                          	pop	rsi
14090e20d: 5d                          	pop	rbp
14090e20e: c3                          	ret
14090e20f: 48 8d 4d a8                 	lea	rcx, [rbp - 0x58]
14090e213: e8 f8 83 f8 ff              	call	0x140896610 <.text+0x895610>
14090e218: eb ee                       	jmp	0x14090e208 <.text+0x90d208>
14090e21a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
14090e220: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14090e225: 55                          	push	rbp
14090e226: 56                          	push	rsi
14090e227: 48 83 ec 28                 	sub	rsp, 0x28
14090e22b: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14090e232: c6 45 ff 01                 	mov	byte ptr [rbp - 0x1], 0x1
14090e236: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
14090e23a: e8 c1 dd f7 ff              	call	0x14088c000 <.text+0x88b000>
14090e23f: c6 45 ff 01                 	mov	byte ptr [rbp - 0x1], 0x1
14090e243: 48 83 c4 28                 	add	rsp, 0x28
14090e247: 5e                          	pop	rsi
14090e248: 5d                          	pop	rbp
14090e249: c3                          	ret
14090e24a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
14090e250: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14090e255: 55                          	push	rbp
14090e256: 56                          	push	rsi
14090e257: 48 83 ec 28                 	sub	rsp, 0x28
14090e25b: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14090e262: c6 45 ff 01                 	mov	byte ptr [rbp - 0x1], 0x1
14090e266: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
14090e26a: e8 91 dd f7 ff              	call	0x14088c000 <.text+0x88b000>
14090e26f: c6 45 ff 01                 	mov	byte ptr [rbp - 0x1], 0x1
14090e273: 48 83 c4 28                 	add	rsp, 0x28
14090e277: 5e                          	pop	rsi
14090e278: 5d                          	pop	rbp
14090e279: c3                          	ret
