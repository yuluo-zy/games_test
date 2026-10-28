
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b2de60: 55                          	push	rbp
140b2de61: 41 57                       	push	r15
140b2de63: 41 56                       	push	r14
140b2de65: 41 55                       	push	r13
140b2de67: 41 54                       	push	r12
140b2de69: 56                          	push	rsi
140b2de6a: 57                          	push	rdi
140b2de6b: 53                          	push	rbx
140b2de6c: 48 81 ec 98 01 00 00        	sub	rsp, 0x198
140b2de73: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
140b2de7b: 0f 29 bd 00 01 00 00        	movaps	xmmword ptr [rbp + 0x100], xmm7
140b2de82: 0f 29 b5 f0 00 00 00        	movaps	xmmword ptr [rbp + 0xf0], xmm6
140b2de89: 48 c7 85 e8 00 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0xe8], -0x2
140b2de94: 4d 89 ce                    	mov	r14, r9
140b2de97: 4c 89 c3                    	mov	rbx, r8
140b2de9a: 48 89 d7                    	mov	rdi, rdx
140b2de9d: 48 89 ce                    	mov	rsi, rcx
140b2dea0: e8 5b 8b 0e 00              	call	0x140c16a00 <.text+0xc15a00>
140b2dea5: 84 c0                       	test	al, al
140b2dea7: 48 89 75 c8                 	mov	qword ptr [rbp - 0x38], rsi
140b2deab: 48 89 bd c0 00 00 00        	mov	qword ptr [rbp + 0xc0], rdi
140b2deb2: 74 60                       	je	0x140b2df14 <.text+0xb2cf14>
140b2deb4: 4c 8d 25 79 d4 fb 01        	lea	r12, [rip + 0x1fbd479]  # 0x142aeb334
140b2debb: 41 bf 2f 00 00 00           	mov	r15d, 0x2f
140b2dec1: ba 2f 00 00 00              	mov	edx, 0x2f
140b2dec6: 4c 89 e1                    	mov	rcx, r12
140b2dec9: e8 a2 cf ff ff              	call	0x140b2ae70 <.text+0xb29e70>
140b2dece: 48 83 f8 01                 	cmp	rax, 0x1
140b2ded2: 0f 85 b2 00 00 00           	jne	0x140b2df8a <.text+0xb2cf8a>
140b2ded8: 49 89 d1                    	mov	r9, rdx
140b2dedb: 48 85 d2                    	test	rdx, rdx
140b2dede: 74 3d                       	je	0x140b2df1d <.text+0xb2cf1d>
140b2dee0: 49 83 f9 2f                 	cmp	r9, 0x2f
140b2dee4: 73 35                       	jae	0x140b2df1b <.text+0xb2cf1b>
140b2dee6: 48 8d 05 47 d4 fb 01        	lea	rax, [rip + 0x1fbd447]  # 0x142aeb334
140b2deed: 41 80 3c 01 bf              	cmp	byte ptr [r9 + rax], -0x41
140b2def2: 7f 29                       	jg	0x140b2df1d <.text+0xb2cf1d>
140b2def4: 48 8d 05 ed d4 fb 01        	lea	rax, [rip + 0x1fbd4ed]  # 0x142aeb3e8
140b2defb: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
140b2df00: 48 8d 0d 2d d4 fb 01        	lea	rcx, [rip + 0x1fbd42d]  # 0x142aeb334
140b2df07: ba 2f 00 00 00              	mov	edx, 0x2f
140b2df0c: 45 31 c0                    	xor	r8d, r8d
140b2df0f: e8 2c ba da 01              	call	0x1428d9940 <.text+0x28d8940>
140b2df14: 31 c0                       	xor	eax, eax
140b2df16: e9 cf 01 00 00              	jmp	0x140b2e0ea <.text+0xb2d0ea>
140b2df1b: 75 d7                       	jne	0x140b2def4 <.text+0xb2cef4>
140b2df1d: 4c 8d 25 10 d4 fb 01        	lea	r12, [rip + 0x1fbd410]  # 0x142aeb334
140b2df24: 4c 89 e1                    	mov	rcx, r12
140b2df27: 4c 89 ca                    	mov	rdx, r9
140b2df2a: e8 41 cf ff ff              	call	0x140b2ae70 <.text+0xb29e70>
140b2df2f: 48 83 f8 01                 	cmp	rax, 0x1
140b2df33: 75 55                       	jne	0x140b2df8a <.text+0xb2cf8a>
140b2df35: 49 89 d0                    	mov	r8, rdx
140b2df38: 49 83 c0 02                 	add	r8, 0x2
140b2df3c: 74 39                       	je	0x140b2df77 <.text+0xb2cf77>
140b2df3e: 49 83 f8 2f                 	cmp	r8, 0x2f
140b2df42: 73 31                       	jae	0x140b2df75 <.text+0xb2cf75>
140b2df44: 48 8d 05 e9 d3 fb 01        	lea	rax, [rip + 0x1fbd3e9]  # 0x142aeb334
140b2df4b: 41 80 3c 00 bf              	cmp	byte ptr [r8 + rax], -0x41
140b2df50: 7f 25                       	jg	0x140b2df77 <.text+0xb2cf77>
140b2df52: 48 8d 05 a7 d4 fb 01        	lea	rax, [rip + 0x1fbd4a7]  # 0x142aeb400
140b2df59: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
140b2df5e: 48 8d 0d cf d3 fb 01        	lea	rcx, [rip + 0x1fbd3cf]  # 0x142aeb334
140b2df65: ba 2f 00 00 00              	mov	edx, 0x2f
140b2df6a: 41 b9 2f 00 00 00           	mov	r9d, 0x2f
140b2df70: e8 cb b9 da 01              	call	0x1428d9940 <.text+0x28d8940>
140b2df75: 75 db                       	jne	0x140b2df52 <.text+0xb2cf52>
140b2df77: 41 bf 2d 00 00 00           	mov	r15d, 0x2d
140b2df7d: 49 29 d7                    	sub	r15, rdx
140b2df80: 4c 8d 25 ad d3 fb 01        	lea	r12, [rip + 0x1fbd3ad]  # 0x142aeb334
140b2df87: 4d 01 c4                    	add	r12, r8
140b2df8a: 48 8d 05 2e d9 fb 01        	lea	rax, [rip + 0x1fbd92e]  # 0x142aeb8bf
140b2df91: 4c 8d 2d f8 d8 fb 01        	lea	r13, [rip + 0x1fbd8f8]  # 0x142aeb890
140b2df98: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
140b2dfa0: 0f be 48 ff                 	movsx	ecx, byte ptr [rax - 0x1]
140b2dfa4: 85 c9                       	test	ecx, ecx
140b2dfa6: 78 18                       	js	0x140b2dfc0 <.text+0xb2cfc0>
140b2dfa8: 48 ff c8                    	dec	rax
140b2dfab: 83 f9 5c                    	cmp	ecx, 0x5c
140b2dfae: 75 69                       	jne	0x140b2e019 <.text+0xb2d019>
140b2dfb0: eb 7c                       	jmp	0x140b2e02e <.text+0xb2d02e>
140b2dfb2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nop	word ptr cs:[rax + rax]
140b2dfc0: 0f b6 50 fe                 	movzx	edx, byte ptr [rax - 0x2]
140b2dfc4: 80 fa c0                    	cmp	dl, -0x40
140b2dfc7: 7d 25                       	jge	0x140b2dfee <.text+0xb2cfee>
140b2dfc9: 44 0f b6 40 fd              	movzx	r8d, byte ptr [rax - 0x3]
140b2dfce: 41 80 f8 c0                 	cmp	r8b, -0x40
140b2dfd2: 7d 23                       	jge	0x140b2dff7 <.text+0xb2cff7>
140b2dfd4: 44 0f b6 48 fc              	movzx	r9d, byte ptr [rax - 0x4]
140b2dfd9: 48 83 c0 fc                 	add	rax, -0x4
140b2dfdd: 41 83 e1 07                 	and	r9d, 0x7
140b2dfe1: 41 c1 e1 06                 	shl	r9d, 0x6
140b2dfe5: 41 83 e0 3f                 	and	r8d, 0x3f
140b2dfe9: 45 09 c8                    	or	r8d, r9d
140b2dfec: eb 11                       	jmp	0x140b2dfff <.text+0xb2cfff>
140b2dfee: 48 83 c0 fe                 	add	rax, -0x2
140b2dff2: 83 e2 1f                    	and	edx, 0x1f
140b2dff5: eb 12                       	jmp	0x140b2e009 <.text+0xb2d009>
140b2dff7: 48 83 c0 fd                 	add	rax, -0x3
140b2dffb: 41 83 e0 0f                 	and	r8d, 0xf
140b2dfff: 41 c1 e0 06                 	shl	r8d, 0x6
140b2e003: 83 e2 3f                    	and	edx, 0x3f
140b2e006: 44 09 c2                    	or	edx, r8d
140b2e009: c1 e2 06                    	shl	edx, 0x6
140b2e00c: 80 e1 3f                    	and	cl, 0x3f
140b2e00f: 0f b6 c9                    	movzx	ecx, cl
140b2e012: 09 d1                       	or	ecx, edx
140b2e014: 83 f9 5c                    	cmp	ecx, 0x5c
140b2e017: 74 15                       	je	0x140b2e02e <.text+0xb2d02e>
140b2e019: 83 f9 2f                    	cmp	ecx, 0x2f
140b2e01c: 74 10                       	je	0x140b2e02e <.text+0xb2d02e>
140b2e01e: 4c 39 e8                    	cmp	rax, r13
140b2e021: 0f 85 79 ff ff ff           	jne	0x140b2dfa0 <.text+0xb2cfa0>
140b2e027: bf 2f 00 00 00              	mov	edi, 0x2f
140b2e02c: eb 4f                       	jmp	0x140b2e07d <.text+0xb2d07d>
140b2e02e: 4c 8d 2d 5b d8 fb 01        	lea	r13, [rip + 0x1fbd85b]  # 0x142aeb890
140b2e035: 4c 29 e8                    	sub	rax, r13
140b2e038: 49 89 c0                    	mov	r8, rax
140b2e03b: 49 ff c0                    	inc	r8
140b2e03e: 74 32                       	je	0x140b2e072 <.text+0xb2d072>
140b2e040: 49 83 f8 2f                 	cmp	r8, 0x2f
140b2e044: 73 2a                       	jae	0x140b2e070 <.text+0xb2d070>
140b2e046: 43 80 3c 28 bf              	cmp	byte ptr [r8 + r13], -0x41
140b2e04b: 7f 25                       	jg	0x140b2e072 <.text+0xb2d072>
140b2e04d: 48 8d 05 74 d3 fb 01        	lea	rax, [rip + 0x1fbd374]  # 0x142aeb3c8
140b2e054: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
140b2e059: 48 8d 0d 30 d8 fb 01        	lea	rcx, [rip + 0x1fbd830]  # 0x142aeb890
140b2e060: ba 2f 00 00 00              	mov	edx, 0x2f
140b2e065: 41 b9 2f 00 00 00           	mov	r9d, 0x2f
140b2e06b: e8 d0 b8 da 01              	call	0x1428d9940 <.text+0x28d8940>
140b2e070: 75 db                       	jne	0x140b2e04d <.text+0xb2d04d>
140b2e072: bf 2e 00 00 00              	mov	edi, 0x2e
140b2e077: 48 29 c7                    	sub	rdi, rax
140b2e07a: 4d 01 c5                    	add	r13, r8
140b2e07d: e8 ae a0 0e 00              	call	0x140c18130 <.text+0xc17130>
140b2e082: 48 89 c1                    	mov	rcx, rax
140b2e085: 48 8b 00                    	mov	rax, qword ptr [rax]
140b2e088: 48 83 f8 01                 	cmp	rax, 0x1
140b2e08c: 0f 85 1b 0d 00 00           	jne	0x140b2edad <.text+0xb2ddad>
140b2e092: 48 83 c1 08                 	add	rcx, 0x8
140b2e096: 48 83 39 00                 	cmp	qword ptr [rcx], 0x0
140b2e09a: 0f 85 f9 11 00 00           	jne	0x140b2f299 <.text+0xb2e299>
140b2e0a0: 48 c7 01 ff ff ff ff        	mov	qword ptr [rcx], -0x1
140b2e0a7: 48 89 8d e0 00 00 00        	mov	qword ptr [rbp + 0xe0], rcx
140b2e0ae: 48 83 c1 08                 	add	rcx, 0x8
140b2e0b2: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
140b2e0b7: 48 c7 44 24 30 00 00 00 00  	mov	qword ptr [rsp + 0x30], 0x0
140b2e0c0: 48 c7 44 24 28 01 00 00 00  	mov	qword ptr [rsp + 0x28], 0x1
140b2e0c9: 4c 89 e2                    	mov	rdx, r12
140b2e0cc: 4d 89 f8                    	mov	r8, r15
140b2e0cf: 4d 89 e9                    	mov	r9, r13
140b2e0d2: e8 69 8c 0e 00              	call	0x140c16d40 <.text+0xc15d40>
140b2e0d7: 48 8b 8d e0 00 00 00        	mov	rcx, qword ptr [rbp + 0xe0]
140b2e0de: 48 ff 01                    	inc	qword ptr [rcx]
140b2e0e1: 48 89 45 d8                 	mov	qword ptr [rbp - 0x28], rax
140b2e0e5: b8 01 00 00 00              	mov	eax, 0x1
140b2e0ea: 48 8b 8d 80 01 00 00        	mov	rcx, qword ptr [rbp + 0x180]
140b2e0f1: 48 89 45 d0                 	mov	qword ptr [rbp - 0x30], rax
140b2e0f5: 48 8b 01                    	mov	rax, qword ptr [rcx]
140b2e0f8: 48 8b 49 08                 	mov	rcx, qword ptr [rcx + 0x8]
140b2e0fc: 48 8b 10                    	mov	rdx, qword ptr [rax]
140b2e0ff: 45 31 ed                    	xor	r13d, r13d
140b2e102: 49 89 d0                    	mov	r8, rdx
140b2e105: 4c 2b 41 18                 	sub	r8, qword ptr [rcx + 0x18]
140b2e109: 4d 0f 42 c5                 	cmovb	r8, r13
140b2e10d: 48 2b 51 38                 	sub	rdx, qword ptr [rcx + 0x38]
140b2e111: 49 0f 42 d5                 	cmovb	rdx, r13
140b2e115: 4c 8b 49 30                 	mov	r9, qword ptr [rcx + 0x30]
140b2e119: 49 29 d1                    	sub	r9, rdx
140b2e11c: 4d 0f 42 cd                 	cmovb	r9, r13
140b2e120: 0f 97 c2                    	seta	dl
140b2e123: 4c 8b 51 10                 	mov	r10, qword ptr [rcx + 0x10]
140b2e127: 4d 29 c2                    	sub	r10, r8
140b2e12a: 4d 0f 42 d5                 	cmovb	r10, r13
140b2e12e: 48 8b 49 40                 	mov	rcx, qword ptr [rcx + 0x40]
140b2e132: 41 0f 97 c0                 	seta	r8b
140b2e136: 4d 01 ca                    	add	r10, r9
140b2e139: 41 08 d0                    	or	r8b, dl
140b2e13c: 44 88 85 a8 00 00 00        	mov	byte ptr [rbp + 0xa8], r8b
140b2e143: 4d 0f 45 d5                 	cmovne	r10, r13
140b2e147: 4c 29 d1                    	sub	rcx, r10
140b2e14a: 48 89 08                    	mov	qword ptr [rax], rcx
140b2e14d: 49 8b 06                    	mov	rax, qword ptr [r14]
140b2e150: 49 8b 4e 08                 	mov	rcx, qword ptr [r14 + 0x8]
140b2e154: 48 8b 10                    	mov	rdx, qword ptr [rax]
140b2e157: 49 89 d0                    	mov	r8, rdx
140b2e15a: 4c 2b 41 18                 	sub	r8, qword ptr [rcx + 0x18]
140b2e15e: 4d 0f 42 c5                 	cmovb	r8, r13
140b2e162: 48 2b 51 38                 	sub	rdx, qword ptr [rcx + 0x38]
140b2e166: 49 0f 42 d5                 	cmovb	rdx, r13
140b2e16a: 4c 8b 49 30                 	mov	r9, qword ptr [rcx + 0x30]
140b2e16e: 49 29 d1                    	sub	r9, rdx
140b2e171: 4d 0f 42 cd                 	cmovb	r9, r13
140b2e175: 0f 97 c2                    	seta	dl
140b2e178: 4c 8b 51 10                 	mov	r10, qword ptr [rcx + 0x10]
140b2e17c: 4d 29 c2                    	sub	r10, r8
140b2e17f: 4d 0f 42 d5                 	cmovb	r10, r13
140b2e183: 48 8b 49 40                 	mov	rcx, qword ptr [rcx + 0x40]
140b2e187: 41 0f 97 c0                 	seta	r8b
140b2e18b: 4d 01 ca                    	add	r10, r9
140b2e18e: 41 08 d0                    	or	r8b, dl
140b2e191: 44 88 45 08                 	mov	byte ptr [rbp + 0x8], r8b
140b2e195: 4d 0f 45 d5                 	cmovne	r10, r13
140b2e199: 4c 29 d1                    	sub	rcx, r10
140b2e19c: 48 89 08                    	mov	qword ptr [rax], rcx
140b2e19f: 48 8b 03                    	mov	rax, qword ptr [rbx]
140b2e1a2: 4c 8b 7b 08                 	mov	r15, qword ptr [rbx + 0x8]
140b2e1a6: 48 8b 08                    	mov	rcx, qword ptr [rax]
140b2e1a9: 48 89 ca                    	mov	rdx, rcx
140b2e1ac: 49 2b 57 18                 	sub	rdx, qword ptr [r15 + 0x18]
140b2e1b0: 49 0f 42 d5                 	cmovb	rdx, r13
140b2e1b4: 49 2b 4f 38                 	sub	rcx, qword ptr [r15 + 0x38]
140b2e1b8: 49 0f 42 cd                 	cmovb	rcx, r13
140b2e1bc: 4d 8b 57 10                 	mov	r10, qword ptr [r15 + 0x10]
140b2e1c0: 4d 8b 4f 30                 	mov	r9, qword ptr [r15 + 0x30]
140b2e1c4: 4d 89 c8                    	mov	r8, r9
140b2e1c7: 49 29 c8                    	sub	r8, rcx
140b2e1ca: 4d 0f 42 c5                 	cmovb	r8, r13
140b2e1ce: 40 0f 96 c6                 	setbe	sil
140b2e1d2: 4c 89 d3                    	mov	rbx, r10
140b2e1d5: 48 29 d3                    	sub	rbx, rdx
140b2e1d8: 49 0f 42 dd                 	cmovb	rbx, r13
140b2e1dc: 40 0f 96 c7                 	setbe	dil
140b2e1e0: 4d 8b 77 40                 	mov	r14, qword ptr [r15 + 0x40]
140b2e1e4: 4d 89 f3                    	mov	r11, r14
140b2e1e7: 4d 29 c3                    	sub	r11, r8
140b2e1ea: 49 29 db                    	sub	r11, rbx
140b2e1ed: 4d 8b 67 08                 	mov	r12, qword ptr [r15 + 0x8]
140b2e1f1: 4d 8b 7f 28                 	mov	r15, qword ptr [r15 + 0x28]
140b2e1f5: 4c 89 18                    	mov	qword ptr [rax], r11
140b2e1f8: 40 84 f7                    	test	dil, sil
140b2e1fb: 75 50                       	jne	0x140b2e24d <.text+0xb2d24d>
140b2e1fd: 49 89 cb                    	mov	r11, rcx
140b2e200: 49 c1 e3 04                 	shl	r11, 0x4
140b2e204: 4d 01 df                    	add	r15, r11
140b2e207: 49 c1 e0 04                 	shl	r8, 0x4
140b2e20b: 49 89 d3                    	mov	r11, rdx
140b2e20e: 49 c1 e3 04                 	shl	r11, 0x4
140b2e212: 4d 01 dc                    	add	r12, r11
140b2e215: 48 c1 e3 04                 	shl	rbx, 0x4
140b2e219: 45 31 db                    	xor	r11d, r11d
140b2e21c: 49 39 d2                    	cmp	r10, rdx
140b2e21f: ba 08 00 00 00              	mov	edx, 0x8
140b2e224: 4c 0f 42 e2                 	cmovb	r12, rdx
140b2e228: 4d 8d 54 1c f0              	lea	r10, [r12 + rbx - 0x10]
140b2e22d: 4d 0f 47 da                 	cmova	r11, r10
140b2e231: 49 39 c9                    	cmp	r9, rcx
140b2e234: 4c 0f 42 fa                 	cmovb	r15, rdx
140b2e238: 4b 8d 4c 07 f0              	lea	rcx, [r15 + r8 - 0x10]
140b2e23d: 49 0f 46 cb                 	cmovbe	rcx, r11
140b2e241: 4c 89 30                    	mov	qword ptr [rax], r14
140b2e244: 44 0f b6 69 08              	movzx	r13d, byte ptr [rcx + 0x8]
140b2e249: 41 80 f5 01                 	xor	r13b, 0x1
140b2e24d: 48 8b 75 c8                 	mov	rsi, qword ptr [rbp - 0x38]
140b2e251: 4c 8b 16                    	mov	r10, qword ptr [rsi]
140b2e254: 49 83 7a 10 00              	cmp	qword ptr [r10 + 0x10], 0x0
140b2e259: 0f 84 e8 00 00 00           	je	0x140b2e347 <.text+0xb2d347>
140b2e25f: 49 8b 42 08                 	mov	rax, qword ptr [r10 + 0x8]
140b2e263: 44 8b 30                    	mov	r14d, dword ptr [rax]
140b2e266: 8b 48 0c                    	mov	ecx, dword ptr [rax + 0xc]
140b2e269: 89 4d 18                    	mov	dword ptr [rbp + 0x18], ecx
140b2e26c: 48 8b 48 04                 	mov	rcx, qword ptr [rax + 0x4]
140b2e270: 48 89 4d 10                 	mov	qword ptr [rbp + 0x10], rcx
140b2e274: 48 8b 58 10                 	mov	rbx, qword ptr [rax + 0x10]
140b2e278: 0f 10 40 18                 	movups	xmm0, xmmword ptr [rax + 0x18]
140b2e27c: 0f 29 45 e0                 	movaps	xmmword ptr [rbp - 0x20], xmm0
140b2e280: 0f 10 40 28                 	movups	xmm0, xmmword ptr [rax + 0x28]
140b2e284: 0f 29 45 f0                 	movaps	xmmword ptr [rbp - 0x10], xmm0
140b2e288: 41 0f b6 42 18              	movzx	eax, byte ptr [r10 + 0x18]
140b2e28d: 83 f8 02                    	cmp	eax, 0x2
140b2e290: 4c 89 95 e0 00 00 00        	mov	qword ptr [rbp + 0xe0], r10
140b2e297: 0f 84 c5 00 00 00           	je	0x140b2e362 <.text+0xb2d362>
140b2e29d: 83 f8 01                    	cmp	eax, 0x1
140b2e2a0: 0f 85 c5 00 00 00           	jne	0x140b2e36b <.text+0xb2d36b>
140b2e2a6: 41 80 7a 19 00              	cmp	byte ptr [r10 + 0x19], 0x0
140b2e2ab: 0f 84 9b 03 00 00           	je	0x140b2e64c <.text+0xb2d64c>
140b2e2b1: 41 83 fe 02                 	cmp	r14d, 0x2
140b2e2b5: 0f 85 bc 01 00 00           	jne	0x140b2e477 <.text+0xb2d477>
140b2e2bb: 49 8d 42 19                 	lea	rax, [r10 + 0x19]
140b2e2bf: 48 89 85 b0 00 00 00        	mov	qword ptr [rbp + 0xb0], rax
140b2e2c6: 8b 46 1c                    	mov	eax, dword ptr [rsi + 0x1c]
140b2e2c9: 48 8b 4e 10                 	mov	rcx, qword ptr [rsi + 0x10]
140b2e2cd: 89 01                       	mov	dword ptr [rcx], eax
140b2e2cf: 41 0f b6 42 18              	movzx	eax, byte ptr [r10 + 0x18]
140b2e2d4: 85 c0                       	test	eax, eax
140b2e2d6: 74 24                       	je	0x140b2e2fc <.text+0xb2d2fc>
140b2e2d8: 83 f8 01                    	cmp	eax, 0x1
140b2e2db: 48 8b 85 e0 00 00 00        	mov	rax, qword ptr [rbp + 0xe0]
140b2e2e2: 48 8b 40 20                 	mov	rax, qword ptr [rax + 0x20]
140b2e2e6: f0                          	lock
140b2e2e7: 48 ff 08                    	dec	qword ptr [rax]
140b2e2ea: 75 10                       	jne	0x140b2e2fc <.text+0xb2d2fc>
140b2e2ec: 48 8b 85 e0 00 00 00        	mov	rax, qword ptr [rbp + 0xe0]
140b2e2f3: 48 8d 48 20                 	lea	rcx, [rax + 0x20]
140b2e2f7: e8 34 e0 02 00              	call	0x140b5c330 <.text+0xb5b330>
140b2e2fc: 4c 8b 95 e0 00 00 00        	mov	r10, qword ptr [rbp + 0xe0]
140b2e303: 41 c6 42 18 00              	mov	byte ptr [r10 + 0x18], 0x0
140b2e308: 48 8b 85 9f 00 00 00        	mov	rax, qword ptr [rbp + 0x9f]
140b2e30f: 48 8b 8d b0 00 00 00        	mov	rcx, qword ptr [rbp + 0xb0]
140b2e316: 48 89 41 3f                 	mov	qword ptr [rcx + 0x3f], rax
140b2e31a: 0f 10 45 60                 	movups	xmm0, xmmword ptr [rbp + 0x60]
140b2e31e: f3 0f 6f 4d 70              	movdqu	xmm1, xmmword ptr [rbp + 0x70]
140b2e323: f3 0f 6f 95 80 00 00 00     	movdqu	xmm2, xmmword ptr [rbp + 0x80]
140b2e32b: f3 0f 6f 9d 90 00 00 00     	movdqu	xmm3, xmmword ptr [rbp + 0x90]
140b2e333: f3 0f 7f 59 30              	movdqu	xmmword ptr [rcx + 0x30], xmm3
140b2e338: f3 0f 7f 51 20              	movdqu	xmmword ptr [rcx + 0x20], xmm2
140b2e33d: f3 0f 7f 49 10              	movdqu	xmmword ptr [rcx + 0x10], xmm1
140b2e342: 0f 11 01                    	movups	xmmword ptr [rcx], xmm0
140b2e345: eb 2a                       	jmp	0x140b2e371 <.text+0xb2d371>
140b2e347: 41 be 02 00 00 00           	mov	r14d, 0x2
140b2e34d: 41 0f b6 42 18              	movzx	eax, byte ptr [r10 + 0x18]
140b2e352: 83 f8 02                    	cmp	eax, 0x2
140b2e355: 4c 89 95 e0 00 00 00        	mov	qword ptr [rbp + 0xe0], r10
140b2e35c: 0f 85 3b ff ff ff           	jne	0x140b2e29d <.text+0xb2d29d>
140b2e362: 31 c9                       	xor	ecx, ecx
140b2e364: 31 d2                       	xor	edx, edx
140b2e366: e9 e7 02 00 00              	jmp	0x140b2e652 <.text+0xb2d652>
140b2e36b: 41 83 fe 02                 	cmp	r14d, 0x2
140b2e36f: 75 0b                       	jne	0x140b2e37c <.text+0xb2d37c>
140b2e371: b1 01                       	mov	cl, 0x1
140b2e373: 31 c0                       	xor	eax, eax
140b2e375: b2 01                       	mov	dl, 0x1
140b2e377: e9 d6 02 00 00              	jmp	0x140b2e652 <.text+0xb2d652>
140b2e37c: 4c 89 d1                    	mov	rcx, r10
140b2e37f: 48 89 da                    	mov	rdx, rbx
140b2e382: e8 d9 f8 ff ff              	call	0x140b2dc60 <.text+0xb2cc60>
140b2e387: 44 89 b5 cc 00 00 00        	mov	dword ptr [rbp + 0xcc], r14d
140b2e38e: 48 89 9d b8 00 00 00        	mov	qword ptr [rbp + 0xb8], rbx
140b2e395: 48 89 85 d0 00 00 00        	mov	qword ptr [rbp + 0xd0], rax
140b2e39c: 48 8b 48 38                 	mov	rcx, qword ptr [rax + 0x38]
140b2e3a0: 48 89 4d 70                 	mov	qword ptr [rbp + 0x70], rcx
140b2e3a4: 48 89 85 b0 00 00 00        	mov	qword ptr [rbp + 0xb0], rax
140b2e3ab: 0f 10 40 28                 	movups	xmm0, xmmword ptr [rax + 0x28]
140b2e3af: 0f 29 45 60                 	movaps	xmmword ptr [rbp + 0x60], xmm0
140b2e3b3: 48 8b 95 98 01 00 00        	mov	rdx, qword ptr [rbp + 0x198]
140b2e3ba: 8b 42 1c                    	mov	eax, dword ptr [rdx + 0x1c]
140b2e3bd: 48 8b 4a 10                 	mov	rcx, qword ptr [rdx + 0x10]
140b2e3c1: 89 01                       	mov	dword ptr [rcx], eax
140b2e3c3: 48 8b 1a                    	mov	rbx, qword ptr [rdx]
140b2e3c6: 4c 8b 7b 30                 	mov	r15, qword ptr [rbx + 0x30]
140b2e3ca: 4c 8b 63 40                 	mov	r12, qword ptr [rbx + 0x40]
140b2e3ce: 4c 3b 7b 20                 	cmp	r15, qword ptr [rbx + 0x20]
140b2e3d2: 75 10                       	jne	0x140b2e3e4 <.text+0xb2d3e4>
140b2e3d4: 48 8d 4b 20                 	lea	rcx, [rbx + 0x20]
140b2e3d8: 48 8d 15 69 bd 10 02        	lea	rdx, [rip + 0x210bd69]  # 0x142c3a148
140b2e3df: e8 4c 48 a3 ff              	call	0x140562c30 <.text+0x561c30>
140b2e3e4: 48 8b 43 28                 	mov	rax, qword ptr [rbx + 0x28]
140b2e3e8: 4c 89 f9                    	mov	rcx, r15
140b2e3eb: 48 c1 e1 05                 	shl	rcx, 0x5
140b2e3ef: 48 8b 55 70                 	mov	rdx, qword ptr [rbp + 0x70]
140b2e3f3: 48 89 54 08 10              	mov	qword ptr [rax + rcx + 0x10], rdx
140b2e3f8: 0f 28 45 60                 	movaps	xmm0, xmmword ptr [rbp + 0x60]
140b2e3fc: 0f 11 04 08                 	movups	xmmword ptr [rax + rcx], xmm0
140b2e400: 4c 89 64 08 18              	mov	qword ptr [rax + rcx + 0x18], r12
140b2e405: 49 ff c7                    	inc	r15
140b2e408: 4c 89 7b 30                 	mov	qword ptr [rbx + 0x30], r15
140b2e40c: 48 ff 43 40                 	inc	qword ptr [rbx + 0x40]
140b2e410: 48 8b 45 10                 	mov	rax, qword ptr [rbp + 0x10]
140b2e414: 48 89 45 20                 	mov	qword ptr [rbp + 0x20], rax
140b2e418: 8b 45 18                    	mov	eax, dword ptr [rbp + 0x18]
140b2e41b: 89 45 28                    	mov	dword ptr [rbp + 0x28], eax
140b2e41e: 0f 28 45 e0                 	movaps	xmm0, xmmword ptr [rbp - 0x20]
140b2e422: 0f 28 4d f0                 	movaps	xmm1, xmmword ptr [rbp - 0x10]
140b2e426: 0f 29 45 60                 	movaps	xmmword ptr [rbp + 0x60], xmm0
140b2e42a: 0f 29 4d 70                 	movaps	xmmword ptr [rbp + 0x70], xmm1
140b2e42e: 8b 46 1c                    	mov	eax, dword ptr [rsi + 0x1c]
140b2e431: 48 8b 4e 10                 	mov	rcx, qword ptr [rsi + 0x10]
140b2e435: 89 01                       	mov	dword ptr [rcx], eax
140b2e437: 48 8b 85 e0 00 00 00        	mov	rax, qword ptr [rbp + 0xe0]
140b2e43e: 0f b6 40 18                 	movzx	eax, byte ptr [rax + 0x18]
140b2e442: 85 c0                       	test	eax, eax
140b2e444: 0f 84 9a 01 00 00           	je	0x140b2e5e4 <.text+0xb2d5e4>
140b2e44a: 83 f8 01                    	cmp	eax, 0x1
140b2e44d: 48 8b 85 e0 00 00 00        	mov	rax, qword ptr [rbp + 0xe0]
140b2e454: 48 8b 40 20                 	mov	rax, qword ptr [rax + 0x20]
140b2e458: f0                          	lock
140b2e459: 48 ff 08                    	dec	qword ptr [rax]
140b2e45c: 0f 85 82 01 00 00           	jne	0x140b2e5e4 <.text+0xb2d5e4>
140b2e462: 48 8b 85 e0 00 00 00        	mov	rax, qword ptr [rbp + 0xe0]
140b2e469: 48 8d 48 20                 	lea	rcx, [rax + 0x20]
140b2e46d: e8 be de 02 00              	call	0x140b5c330 <.text+0xb5b330>
140b2e472: e9 6d 01 00 00              	jmp	0x140b2e5e4 <.text+0xb2d5e4>
140b2e477: 49 39 5a 38                 	cmp	qword ptr [r10 + 0x38], rbx
140b2e47b: 48 89 9d b8 00 00 00        	mov	qword ptr [rbp + 0xb8], rbx
140b2e482: 44 89 b5 cc 00 00 00        	mov	dword ptr [rbp + 0xcc], r14d
140b2e489: 75 79                       	jne	0x140b2e504 <.text+0xb2d504>
140b2e48b: 4c 89 d1                    	mov	rcx, r10
140b2e48e: 48 89 da                    	mov	rdx, rbx
140b2e491: e8 ca f7 ff ff              	call	0x140b2dc60 <.text+0xb2cc60>
140b2e496: 48 89 85 b0 00 00 00        	mov	qword ptr [rbp + 0xb0], rax
140b2e49d: 8b 45 18                    	mov	eax, dword ptr [rbp + 0x18]
140b2e4a0: 89 45 28                    	mov	dword ptr [rbp + 0x28], eax
140b2e4a3: 48 8b 45 10                 	mov	rax, qword ptr [rbp + 0x10]
140b2e4a7: 48 89 45 20                 	mov	qword ptr [rbp + 0x20], rax
140b2e4ab: 0f 28 45 e0                 	movaps	xmm0, xmmword ptr [rbp - 0x20]
140b2e4af: 0f 28 4d f0                 	movaps	xmm1, xmmword ptr [rbp - 0x10]
140b2e4b3: 0f 29 45 60                 	movaps	xmmword ptr [rbp + 0x60], xmm0
140b2e4b7: 0f 29 4d 70                 	movaps	xmmword ptr [rbp + 0x70], xmm1
140b2e4bb: 8b 46 1c                    	mov	eax, dword ptr [rsi + 0x1c]
140b2e4be: 48 8b 4e 10                 	mov	rcx, qword ptr [rsi + 0x10]
140b2e4c2: 89 01                       	mov	dword ptr [rcx], eax
140b2e4c4: 48 8b 85 e0 00 00 00        	mov	rax, qword ptr [rbp + 0xe0]
140b2e4cb: 0f b6 40 18                 	movzx	eax, byte ptr [rax + 0x18]
140b2e4cf: 85 c0                       	test	eax, eax
140b2e4d1: 0f 84 0d 01 00 00           	je	0x140b2e5e4 <.text+0xb2d5e4>
140b2e4d7: 83 f8 01                    	cmp	eax, 0x1
140b2e4da: 48 8b 85 e0 00 00 00        	mov	rax, qword ptr [rbp + 0xe0]
140b2e4e1: 48 8b 40 20                 	mov	rax, qword ptr [rax + 0x20]
140b2e4e5: f0                          	lock
140b2e4e6: 48 ff 08                    	dec	qword ptr [rax]
140b2e4e9: 0f 85 f5 00 00 00           	jne	0x140b2e5e4 <.text+0xb2d5e4>
140b2e4ef: 48 8b 85 e0 00 00 00        	mov	rax, qword ptr [rbp + 0xe0]
140b2e4f6: 48 8d 48 20                 	lea	rcx, [rax + 0x20]
140b2e4fa: e8 31 de 02 00              	call	0x140b5c330 <.text+0xb5b330>
140b2e4ff: e9 e0 00 00 00              	jmp	0x140b2e5e4 <.text+0xb2d5e4>
140b2e504: 4c 89 d1                    	mov	rcx, r10
140b2e507: 48 89 da                    	mov	rdx, rbx
140b2e50a: e8 51 f7 ff ff              	call	0x140b2dc60 <.text+0xb2cc60>
140b2e50f: 48 89 85 d0 00 00 00        	mov	qword ptr [rbp + 0xd0], rax
140b2e516: 48 8b 48 38                 	mov	rcx, qword ptr [rax + 0x38]
140b2e51a: 48 89 4d 70                 	mov	qword ptr [rbp + 0x70], rcx
140b2e51e: 48 89 85 b0 00 00 00        	mov	qword ptr [rbp + 0xb0], rax
140b2e525: 0f 10 40 28                 	movups	xmm0, xmmword ptr [rax + 0x28]
140b2e529: 0f 29 45 60                 	movaps	xmmword ptr [rbp + 0x60], xmm0
140b2e52d: 48 8b 95 98 01 00 00        	mov	rdx, qword ptr [rbp + 0x198]
140b2e534: 8b 42 1c                    	mov	eax, dword ptr [rdx + 0x1c]
140b2e537: 48 8b 4a 10                 	mov	rcx, qword ptr [rdx + 0x10]
140b2e53b: 89 01                       	mov	dword ptr [rcx], eax
140b2e53d: 48 8b 1a                    	mov	rbx, qword ptr [rdx]
140b2e540: 4c 8b 7b 30                 	mov	r15, qword ptr [rbx + 0x30]
140b2e544: 4c 8b 63 40                 	mov	r12, qword ptr [rbx + 0x40]
140b2e548: 4c 3b 7b 20                 	cmp	r15, qword ptr [rbx + 0x20]
140b2e54c: 75 10                       	jne	0x140b2e55e <.text+0xb2d55e>
140b2e54e: 48 8d 4b 20                 	lea	rcx, [rbx + 0x20]
140b2e552: 48 8d 15 ef bb 10 02        	lea	rdx, [rip + 0x210bbef]  # 0x142c3a148
140b2e559: e8 d2 46 a3 ff              	call	0x140562c30 <.text+0x561c30>
140b2e55e: 48 8b 43 28                 	mov	rax, qword ptr [rbx + 0x28]
140b2e562: 4c 89 f9                    	mov	rcx, r15
140b2e565: 48 c1 e1 05                 	shl	rcx, 0x5
140b2e569: 48 8b 55 70                 	mov	rdx, qword ptr [rbp + 0x70]
140b2e56d: 48 89 54 08 10              	mov	qword ptr [rax + rcx + 0x10], rdx
140b2e572: 0f 28 45 60                 	movaps	xmm0, xmmword ptr [rbp + 0x60]
140b2e576: 0f 11 04 08                 	movups	xmmword ptr [rax + rcx], xmm0
140b2e57a: 4c 89 64 08 18              	mov	qword ptr [rax + rcx + 0x18], r12
140b2e57f: 49 ff c7                    	inc	r15
140b2e582: 4c 89 7b 30                 	mov	qword ptr [rbx + 0x30], r15
140b2e586: 48 ff 43 40                 	inc	qword ptr [rbx + 0x40]
140b2e58a: 48 8b 45 10                 	mov	rax, qword ptr [rbp + 0x10]
140b2e58e: 48 89 45 20                 	mov	qword ptr [rbp + 0x20], rax
140b2e592: 8b 45 18                    	mov	eax, dword ptr [rbp + 0x18]
140b2e595: 89 45 28                    	mov	dword ptr [rbp + 0x28], eax
140b2e598: 0f 28 45 e0                 	movaps	xmm0, xmmword ptr [rbp - 0x20]
140b2e59c: 0f 28 4d f0                 	movaps	xmm1, xmmword ptr [rbp - 0x10]
140b2e5a0: 0f 29 45 60                 	movaps	xmmword ptr [rbp + 0x60], xmm0
140b2e5a4: 0f 29 4d 70                 	movaps	xmmword ptr [rbp + 0x70], xmm1
140b2e5a8: 8b 46 1c                    	mov	eax, dword ptr [rsi + 0x1c]
140b2e5ab: 48 8b 4e 10                 	mov	rcx, qword ptr [rsi + 0x10]
140b2e5af: 89 01                       	mov	dword ptr [rcx], eax
140b2e5b1: 48 8b 85 e0 00 00 00        	mov	rax, qword ptr [rbp + 0xe0]
140b2e5b8: 0f b6 40 18                 	movzx	eax, byte ptr [rax + 0x18]
140b2e5bc: 85 c0                       	test	eax, eax
140b2e5be: 74 24                       	je	0x140b2e5e4 <.text+0xb2d5e4>
140b2e5c0: 83 f8 01                    	cmp	eax, 0x1
140b2e5c3: 48 8b 85 e0 00 00 00        	mov	rax, qword ptr [rbp + 0xe0]
140b2e5ca: 48 8b 40 20                 	mov	rax, qword ptr [rax + 0x20]
140b2e5ce: f0                          	lock
140b2e5cf: 48 ff 08                    	dec	qword ptr [rax]
140b2e5d2: 75 10                       	jne	0x140b2e5e4 <.text+0xb2d5e4>
140b2e5d4: 48 8b 85 e0 00 00 00        	mov	rax, qword ptr [rbp + 0xe0]
140b2e5db: 48 8d 48 20                 	lea	rcx, [rax + 0x20]
140b2e5df: e8 4c dd 02 00              	call	0x140b5c330 <.text+0xb5b330>
140b2e5e4: 4c 8b 95 e0 00 00 00        	mov	r10, qword ptr [rbp + 0xe0]
140b2e5eb: 66 41 c7 42 18 01 01        	mov	word ptr [r10 + 0x18], 0x101
140b2e5f2: 8b 85 d0 00 00 00           	mov	eax, dword ptr [rbp + 0xd0]
140b2e5f8: 41 89 42 1a                 	mov	dword ptr [r10 + 0x1a], eax
140b2e5fc: 0f b7 85 d4 00 00 00        	movzx	eax, word ptr [rbp + 0xd4]
140b2e603: 66 41 89 42 1e              	mov	word ptr [r10 + 0x1e], ax
140b2e608: 48 8b 85 b0 00 00 00        	mov	rax, qword ptr [rbp + 0xb0]
140b2e60f: 49 89 42 20                 	mov	qword ptr [r10 + 0x20], rax
140b2e613: 44 8b b5 cc 00 00 00        	mov	r14d, dword ptr [rbp + 0xcc]
140b2e61a: 45 89 72 28                 	mov	dword ptr [r10 + 0x28], r14d
140b2e61e: 48 8b 45 20                 	mov	rax, qword ptr [rbp + 0x20]
140b2e622: 49 89 42 2c                 	mov	qword ptr [r10 + 0x2c], rax
140b2e626: 8b 45 28                    	mov	eax, dword ptr [rbp + 0x28]
140b2e629: 41 89 42 34                 	mov	dword ptr [r10 + 0x34], eax
140b2e62d: 48 8b 9d b8 00 00 00        	mov	rbx, qword ptr [rbp + 0xb8]
140b2e634: 49 89 5a 38                 	mov	qword ptr [r10 + 0x38], rbx
140b2e638: 0f 28 45 60                 	movaps	xmm0, xmmword ptr [rbp + 0x60]
140b2e63c: 66 0f 6f 4d 70              	movdqa	xmm1, xmmword ptr [rbp + 0x70]
140b2e641: 41 0f 11 42 40              	movups	xmmword ptr [r10 + 0x40], xmm0
140b2e646: f3 41 0f 7f 4a 50           	movdqu	xmmword ptr [r10 + 0x50], xmm1
140b2e64c: b1 01                       	mov	cl, 0x1
140b2e64e: 31 d2                       	xor	edx, edx
140b2e650: b0 01                       	mov	al, 0x1
140b2e652: 4c 8b bd 88 01 00 00        	mov	r15, qword ptr [rbp + 0x188]
140b2e659: 4c 8b 85 c0 00 00 00        	mov	r8, qword ptr [rbp + 0xc0]
140b2e660: 4d 8b 20                    	mov	r12, qword ptr [r8]
140b2e663: 45 8b 04 24                 	mov	r8d, dword ptr [r12]
140b2e667: 41 83 f8 03                 	cmp	r8d, 0x3
140b2e66b: 41 0f 94 c1                 	sete	r9b
140b2e66f: 44 0f b6 5d 08              	movzx	r11d, byte ptr [rbp + 0x8]
140b2e674: 45 08 cb                    	or	r11b, r9b
140b2e677: 44 0f b6 8d a8 00 00 00     	movzx	r9d, byte ptr [rbp + 0xa8]
140b2e67f: 45 08 d9                    	or	r9b, r11b
140b2e682: 45 08 cd                    	or	r13b, r9b
140b2e685: 74 11                       	je	0x140b2e698 <.text+0xb2d698>
140b2e687: 84 c9                       	test	cl, cl
140b2e689: 74 59                       	je	0x140b2e6e4 <.text+0xb2d6e4>
140b2e68b: 84 d2                       	test	dl, dl
140b2e68d: 0f 85 d7 01 00 00           	jne	0x140b2e86a <.text+0xb2d86a>
140b2e693: e9 4b 06 00 00              	jmp	0x140b2ece3 <.text+0xb2dce3>
140b2e698: 44 89 c1                    	mov	ecx, r8d
140b2e69b: 48 8d 15 ba d3 fb 01        	lea	rdx, [rip + 0x1fbd3ba]  # 0x142aeba5c
140b2e6a2: 48 63 0c 8a                 	movsxd	rcx, dword ptr [rdx + 4*rcx]
140b2e6a6: 48 01 d1                    	add	rcx, rdx
140b2e6a9: ff e1                       	jmp	rcx
140b2e6ab: 31 c9                       	xor	ecx, ecx
140b2e6ad: 84 c0                       	test	al, al
140b2e6af: 75 24                       	jne	0x140b2e6d5 <.text+0xb2d6d5>
140b2e6b1: e9 b4 01 00 00              	jmp	0x140b2e86a <.text+0xb2d86a>
140b2e6b6: 0f 57 c0                    	xorps	xmm0, xmm0
140b2e6b9: 41 0f 2e 44 24 04           	ucomiss	xmm0, dword ptr [r12 + 0x4]
140b2e6bf: 0f 93 c1                    	setae	cl
140b2e6c2: 84 c0                       	test	al, al
140b2e6c4: 75 0f                       	jne	0x140b2e6d5 <.text+0xb2d6d5>
140b2e6c6: e9 9f 01 00 00              	jmp	0x140b2e86a <.text+0xb2d86a>
140b2e6cb: b1 01                       	mov	cl, 0x1
140b2e6cd: 84 c0                       	test	al, al
140b2e6cf: 0f 84 95 01 00 00           	je	0x140b2e86a <.text+0xb2d86a>
140b2e6d5: 0f b6 c0                    	movzx	eax, al
140b2e6d8: 83 f8 02                    	cmp	eax, 0x2
140b2e6db: 0f 85 e0 01 00 00           	jne	0x140b2e8c1 <.text+0xb2d8c1>
140b2e6e1: 45 31 ed                    	xor	r13d, r13d
140b2e6e4: 41 0f b6 42 19              	movzx	eax, byte ptr [r10 + 0x19]
140b2e6e9: 04 fe                       	add	al, -0x2
140b2e6eb: 0f b6 c0                    	movzx	eax, al
140b2e6ee: 3c 03                       	cmp	al, 0x3
140b2e6f0: b9 03 00 00 00              	mov	ecx, 0x3
140b2e6f5: 0f 42 c8                    	cmovb	ecx, eax
140b2e6f8: 49 8d 7a 20                 	lea	rdi, [r10 + 0x20]
140b2e6fc: 0f b6 c1                    	movzx	eax, cl
140b2e6ff: 8d 48 ff                    	lea	ecx, [rax - 0x1]
140b2e702: 83 f9 02                    	cmp	ecx, 0x2
140b2e705: 0f 83 d4 00 00 00           	jae	0x140b2e7df <.text+0xb2d7df>
140b2e70b: 41 f6 c5 01                 	test	r13b, 0x1
140b2e70f: 0f 84 a2 02 00 00           	je	0x140b2e9b7 <.text+0xb2d9b7>
140b2e715: 49 8b 42 58                 	mov	rax, qword ptr [r10 + 0x58]
140b2e719: 48 89 85 90 00 00 00        	mov	qword ptr [rbp + 0x90], rax
140b2e720: f3 41 0f 6f 42 28           	movdqu	xmm0, xmmword ptr [r10 + 0x28]
140b2e726: f3 41 0f 6f 4a 38           	movdqu	xmm1, xmmword ptr [r10 + 0x38]
140b2e72c: f3 41 0f 6f 52 48           	movdqu	xmm2, xmmword ptr [r10 + 0x48]
140b2e732: 66 0f 7f 95 80 00 00 00     	movdqa	xmmword ptr [rbp + 0x80], xmm2
140b2e73a: 66 0f 7f 4d 70              	movdqa	xmmword ptr [rbp + 0x70], xmm1
140b2e73f: 66 0f 7f 45 60              	movdqa	xmmword ptr [rbp + 0x60], xmm0
140b2e744: 49 8b 4a 20                 	mov	rcx, qword ptr [r10 + 0x20]
140b2e748: 49 8b 42 38                 	mov	rax, qword ptr [r10 + 0x38]
140b2e74c: 48 89 8d c0 00 00 00        	mov	qword ptr [rbp + 0xc0], rcx
140b2e753: f0                          	lock
140b2e754: 48 ff 01                    	inc	qword ptr [rcx]
140b2e757: 0f 8e 68 0b 00 00           	jle	0x140b2f2c5 <.text+0xb2e2c5>
140b2e75d: 49 8d 4a 28                 	lea	rcx, [r10 + 0x28]
140b2e761: 48 89 8d a8 00 00 00        	mov	qword ptr [rbp + 0xa8], rcx
140b2e768: 41 83 fe 02                 	cmp	r14d, 0x2
140b2e76c: 0f 95 c1                    	setne	cl
140b2e76f: 48 39 c3                    	cmp	rbx, rax
140b2e772: 0f 94 c0                    	sete	al
140b2e775: 20 c8                       	and	al, cl
140b2e777: 88 45 08                    	mov	byte ptr [rbp + 0x8], al
140b2e77a: 48 8b 85 90 00 00 00        	mov	rax, qword ptr [rbp + 0x90]
140b2e781: 48 89 45 50                 	mov	qword ptr [rbp + 0x50], rax
140b2e785: 0f 28 45 60                 	movaps	xmm0, xmmword ptr [rbp + 0x60]
140b2e789: 0f 28 4d 70                 	movaps	xmm1, xmmword ptr [rbp + 0x70]
140b2e78d: 0f 28 95 80 00 00 00        	movaps	xmm2, xmmword ptr [rbp + 0x80]
140b2e794: 0f 29 55 40                 	movaps	xmmword ptr [rbp + 0x40], xmm2
140b2e798: 0f 29 4d 30                 	movaps	xmmword ptr [rbp + 0x30], xmm1
140b2e79c: 0f 29 45 20                 	movaps	xmmword ptr [rbp + 0x20], xmm0
140b2e7a0: 8b 46 1c                    	mov	eax, dword ptr [rsi + 0x1c]
140b2e7a3: 48 8b 4e 10                 	mov	rcx, qword ptr [rsi + 0x10]
140b2e7a7: 89 01                       	mov	dword ptr [rcx], eax
140b2e7a9: 41 0f b6 42 18              	movzx	eax, byte ptr [r10 + 0x18]
140b2e7ae: 85 c0                       	test	eax, eax
140b2e7b0: 74 14                       	je	0x140b2e7c6 <.text+0xb2d7c6>
140b2e7b2: 83 f8 01                    	cmp	eax, 0x1
140b2e7b5: 48 8b 07                    	mov	rax, qword ptr [rdi]
140b2e7b8: f0                          	lock
140b2e7b9: 48 ff 08                    	dec	qword ptr [rax]
140b2e7bc: 75 08                       	jne	0x140b2e7c6 <.text+0xb2d7c6>
140b2e7be: 48 89 f9                    	mov	rcx, rdi
140b2e7c1: e8 6a db 02 00              	call	0x140b5c330 <.text+0xb5b330>
140b2e7c6: 4c 8b 95 e0 00 00 00        	mov	r10, qword ptr [rbp + 0xe0]
140b2e7cd: 41 c6 42 18 02              	mov	byte ptr [r10 + 0x18], 0x2
140b2e7d2: 0f b6 45 08                 	movzx	eax, byte ptr [rbp + 0x8]
140b2e7d6: 41 88 42 19                 	mov	byte ptr [r10 + 0x19], al
140b2e7da: e9 9b 01 00 00              	jmp	0x140b2e97a <.text+0xb2d97a>
140b2e7df: 85 c0                       	test	eax, eax
140b2e7e1: 0f 84 fc 04 00 00           	je	0x140b2ece3 <.text+0xb2dce3>
140b2e7e7: 41 83 fe 02                 	cmp	r14d, 0x2
140b2e7eb: 0f 85 fa 01 00 00           	jne	0x140b2e9eb <.text+0xb2d9eb>
140b2e7f1: 49 8d 42 19                 	lea	rax, [r10 + 0x19]
140b2e7f5: 48 89 85 c0 00 00 00        	mov	qword ptr [rbp + 0xc0], rax
140b2e7fc: 8b 46 1c                    	mov	eax, dword ptr [rsi + 0x1c]
140b2e7ff: 48 8b 4e 10                 	mov	rcx, qword ptr [rsi + 0x10]
140b2e803: 89 01                       	mov	dword ptr [rcx], eax
140b2e805: 41 0f b6 42 18              	movzx	eax, byte ptr [r10 + 0x18]
140b2e80a: 85 c0                       	test	eax, eax
140b2e80c: 74 14                       	je	0x140b2e822 <.text+0xb2d822>
140b2e80e: 83 f8 01                    	cmp	eax, 0x1
140b2e811: 48 8b 07                    	mov	rax, qword ptr [rdi]
140b2e814: f0                          	lock
140b2e815: 48 ff 08                    	dec	qword ptr [rax]
140b2e818: 75 08                       	jne	0x140b2e822 <.text+0xb2d822>
140b2e81a: 48 89 f9                    	mov	rcx, rdi
140b2e81d: e8 0e db 02 00              	call	0x140b5c330 <.text+0xb5b330>
140b2e822: 48 8b 85 e0 00 00 00        	mov	rax, qword ptr [rbp + 0xe0]
140b2e829: c6 40 18 00                 	mov	byte ptr [rax + 0x18], 0x0
140b2e82d: 48 8b 85 9f 00 00 00        	mov	rax, qword ptr [rbp + 0x9f]
140b2e834: 48 8b 8d c0 00 00 00        	mov	rcx, qword ptr [rbp + 0xc0]
140b2e83b: 48 89 41 3f                 	mov	qword ptr [rcx + 0x3f], rax
140b2e83f: 0f 10 45 60                 	movups	xmm0, xmmword ptr [rbp + 0x60]
140b2e843: f3 0f 6f 4d 70              	movdqu	xmm1, xmmword ptr [rbp + 0x70]
140b2e848: f3 0f 6f 95 80 00 00 00     	movdqu	xmm2, xmmword ptr [rbp + 0x80]
140b2e850: f3 0f 6f 9d 90 00 00 00     	movdqu	xmm3, xmmword ptr [rbp + 0x90]
140b2e858: f3 0f 7f 59 30              	movdqu	xmmword ptr [rcx + 0x30], xmm3
140b2e85d: f3 0f 7f 51 20              	movdqu	xmmword ptr [rcx + 0x20], xmm2
140b2e862: f3 0f 7f 49 10              	movdqu	xmmword ptr [rcx + 0x10], xmm1
140b2e867: 0f 11 01                    	movups	xmmword ptr [rcx], xmm0
140b2e86a: 41 8b 47 1c                 	mov	eax, dword ptr [r15 + 0x1c]
140b2e86e: 49 8b 4f 10                 	mov	rcx, qword ptr [r15 + 0x10]
140b2e872: 89 01                       	mov	dword ptr [rcx], eax
140b2e874: 49 8b 3f                    	mov	rdi, qword ptr [r15]
140b2e877: 48 8b 5f 30                 	mov	rbx, qword ptr [rdi + 0x30]
140b2e87b: 4c 8b 77 40                 	mov	r14, qword ptr [rdi + 0x40]
140b2e87f: 48 3b 5f 20                 	cmp	rbx, qword ptr [rdi + 0x20]
140b2e883: 75 10                       	jne	0x140b2e895 <.text+0xb2d895>
140b2e885: 48 8d 4f 20                 	lea	rcx, [rdi + 0x20]
140b2e889: 48 8d 15 b8 b8 10 02        	lea	rdx, [rip + 0x210b8b8]  # 0x142c3a148
140b2e890: e8 9b 43 a3 ff              	call	0x140562c30 <.text+0x561c30>
140b2e895: 48 8b 47 28                 	mov	rax, qword ptr [rdi + 0x28]
140b2e899: 48 89 d9                    	mov	rcx, rbx
140b2e89c: 48 c1 e1 05                 	shl	rcx, 0x5
140b2e8a0: 4c 89 34 08                 	mov	qword ptr [rax + rcx], r14
140b2e8a4: c7 44 08 08 00 00 00 00     	mov	dword ptr [rax + rcx + 0x8], 0x0
140b2e8ac: c6 44 08 18 00              	mov	byte ptr [rax + rcx + 0x18], 0x0
140b2e8b1: 48 ff c3                    	inc	rbx
140b2e8b4: 48 89 5f 30                 	mov	qword ptr [rdi + 0x30], rbx
140b2e8b8: 48 ff 47 40                 	inc	qword ptr [rdi + 0x40]
140b2e8bc: e9 90 07 00 00              	jmp	0x140b2f051 <.text+0xb2e051>
140b2e8c1: 84 c9                       	test	cl, cl
140b2e8c3: 0f 84 1a 04 00 00           	je	0x140b2ece3 <.text+0xb2dce3>
140b2e8c9: 49 8b 42 58                 	mov	rax, qword ptr [r10 + 0x58]
140b2e8cd: 48 89 85 90 00 00 00        	mov	qword ptr [rbp + 0x90], rax
140b2e8d4: f3 41 0f 6f 42 28           	movdqu	xmm0, xmmword ptr [r10 + 0x28]
140b2e8da: f3 41 0f 6f 4a 38           	movdqu	xmm1, xmmword ptr [r10 + 0x38]
140b2e8e0: f3 41 0f 6f 52 48           	movdqu	xmm2, xmmword ptr [r10 + 0x48]
140b2e8e6: 66 0f 7f 95 80 00 00 00     	movdqa	xmmword ptr [rbp + 0x80], xmm2
140b2e8ee: 66 0f 7f 4d 70              	movdqa	xmmword ptr [rbp + 0x70], xmm1
140b2e8f3: 66 0f 7f 45 60              	movdqa	xmmword ptr [rbp + 0x60], xmm0
140b2e8f8: 49 8b 42 20                 	mov	rax, qword ptr [r10 + 0x20]
140b2e8fc: 48 89 85 c0 00 00 00        	mov	qword ptr [rbp + 0xc0], rax
140b2e903: f0                          	lock
140b2e904: 48 ff 00                    	inc	qword ptr [rax]
140b2e907: 0f 8e b8 09 00 00           	jle	0x140b2f2c5 <.text+0xb2e2c5>
140b2e90d: 49 8d 42 28                 	lea	rax, [r10 + 0x28]
140b2e911: 48 89 85 a8 00 00 00        	mov	qword ptr [rbp + 0xa8], rax
140b2e918: 48 8b 85 90 00 00 00        	mov	rax, qword ptr [rbp + 0x90]
140b2e91f: 48 89 45 50                 	mov	qword ptr [rbp + 0x50], rax
140b2e923: 0f 28 45 60                 	movaps	xmm0, xmmword ptr [rbp + 0x60]
140b2e927: 0f 28 4d 70                 	movaps	xmm1, xmmword ptr [rbp + 0x70]
140b2e92b: 0f 28 95 80 00 00 00        	movaps	xmm2, xmmword ptr [rbp + 0x80]
140b2e932: 0f 29 55 40                 	movaps	xmmword ptr [rbp + 0x40], xmm2
140b2e936: 0f 29 4d 30                 	movaps	xmmword ptr [rbp + 0x30], xmm1
140b2e93a: 0f 29 45 20                 	movaps	xmmword ptr [rbp + 0x20], xmm0
140b2e93e: 8b 46 1c                    	mov	eax, dword ptr [rsi + 0x1c]
140b2e941: 48 8b 4e 10                 	mov	rcx, qword ptr [rsi + 0x10]
140b2e945: 89 01                       	mov	dword ptr [rcx], eax
140b2e947: 41 0f b6 42 18              	movzx	eax, byte ptr [r10 + 0x18]
140b2e94c: 85 c0                       	test	eax, eax
140b2e94e: 74 1c                       	je	0x140b2e96c <.text+0xb2d96c>
140b2e950: 48 8b 8d e0 00 00 00        	mov	rcx, qword ptr [rbp + 0xe0]
140b2e957: 48 83 c1 20                 	add	rcx, 0x20
140b2e95b: 83 f8 01                    	cmp	eax, 0x1
140b2e95e: 48 8b 01                    	mov	rax, qword ptr [rcx]
140b2e961: f0                          	lock
140b2e962: 48 ff 08                    	dec	qword ptr [rax]
140b2e965: 75 05                       	jne	0x140b2e96c <.text+0xb2d96c>
140b2e967: e8 c4 d9 02 00              	call	0x140b5c330 <.text+0xb5b330>
140b2e96c: 4c 8b 95 e0 00 00 00        	mov	r10, qword ptr [rbp + 0xe0]
140b2e973: 66 41 c7 42 18 02 03        	mov	word ptr [r10 + 0x18], 0x302
140b2e97a: 8b 45 60                    	mov	eax, dword ptr [rbp + 0x60]
140b2e97d: 41 89 42 1a                 	mov	dword ptr [r10 + 0x1a], eax
140b2e981: 0f b7 45 64                 	movzx	eax, word ptr [rbp + 0x64]
140b2e985: 66 41 89 42 1e              	mov	word ptr [r10 + 0x1e], ax
140b2e98a: 48 8b 85 c0 00 00 00        	mov	rax, qword ptr [rbp + 0xc0]
140b2e991: 49 89 42 20                 	mov	qword ptr [r10 + 0x20], rax
140b2e995: 48 8b 45 50                 	mov	rax, qword ptr [rbp + 0x50]
140b2e999: 48 8b 8d a8 00 00 00        	mov	rcx, qword ptr [rbp + 0xa8]
140b2e9a0: 48 89 41 30                 	mov	qword ptr [rcx + 0x30], rax
140b2e9a4: 0f 28 45 20                 	movaps	xmm0, xmmword ptr [rbp + 0x20]
140b2e9a8: 66 0f 6f 4d 30              	movdqa	xmm1, xmmword ptr [rbp + 0x30]
140b2e9ad: 66 0f 6f 55 40              	movdqa	xmm2, xmmword ptr [rbp + 0x40]
140b2e9b2: e9 1f 03 00 00              	jmp	0x140b2ecd6 <.text+0xb2dcd6>
140b2e9b7: 49 8d 5a 60                 	lea	rbx, [r10 + 0x60]
140b2e9bb: 49 8b 42 60                 	mov	rax, qword ptr [r10 + 0x60]
140b2e9bf: 89 c1                       	mov	ecx, eax
140b2e9c1: 83 e1 08                    	and	ecx, 0x8
140b2e9c4: c1 e9 03                    	shr	ecx, 0x3
140b2e9c7: 48 83 f8 f0                 	cmp	rax, -0x10
140b2e9cb: 0f 93 c2                    	setae	dl
140b2e9ce: 08 ca                       	or	dl, cl
140b2e9d0: 0f 84 92 00 00 00           	je	0x140b2ea68 <.text+0xb2da68>
140b2e9d6: 48 89 d9                    	mov	rcx, rbx
140b2e9d9: 31 d2                       	xor	edx, edx
140b2e9db: 41 b9 00 ca 9a 3b           	mov	r9d, 0x3b9aca00
140b2e9e1: e8 ca da d9 01              	call	0x1428cc4b0 <.text+0x28cb4b0>
140b2e9e6: e9 8c 00 00 00              	jmp	0x140b2ea77 <.text+0xb2da77>
140b2e9eb: 49 3b 5a 38                 	cmp	rbx, qword ptr [r10 + 0x38]
140b2e9ef: 48 89 9d b8 00 00 00        	mov	qword ptr [rbp + 0xb8], rbx
140b2e9f6: 44 89 b5 cc 00 00 00        	mov	dword ptr [rbp + 0xcc], r14d
140b2e9fd: 0f 85 cb 03 00 00           	jne	0x140b2edce <.text+0xb2ddce>
140b2ea03: 48 8b 07                    	mov	rax, qword ptr [rdi]
140b2ea06: 48 89 85 c0 00 00 00        	mov	qword ptr [rbp + 0xc0], rax
140b2ea0d: f0                          	lock
140b2ea0e: 48 ff 00                    	inc	qword ptr [rax]
140b2ea11: 0f 8e ae 08 00 00           	jle	0x140b2f2c5 <.text+0xb2e2c5>
140b2ea17: 8b 45 18                    	mov	eax, dword ptr [rbp + 0x18]
140b2ea1a: 89 45 28                    	mov	dword ptr [rbp + 0x28], eax
140b2ea1d: 48 8b 45 10                 	mov	rax, qword ptr [rbp + 0x10]
140b2ea21: 48 89 45 20                 	mov	qword ptr [rbp + 0x20], rax
140b2ea25: 0f 28 45 e0                 	movaps	xmm0, xmmword ptr [rbp - 0x20]
140b2ea29: 0f 28 4d f0                 	movaps	xmm1, xmmword ptr [rbp - 0x10]
140b2ea2d: 0f 29 45 60                 	movaps	xmmword ptr [rbp + 0x60], xmm0
140b2ea31: 0f 29 4d 70                 	movaps	xmmword ptr [rbp + 0x70], xmm1
140b2ea35: 8b 46 1c                    	mov	eax, dword ptr [rsi + 0x1c]
140b2ea38: 48 8b 4e 10                 	mov	rcx, qword ptr [rsi + 0x10]
140b2ea3c: 89 01                       	mov	dword ptr [rcx], eax
140b2ea3e: 41 0f b6 42 18              	movzx	eax, byte ptr [r10 + 0x18]
140b2ea43: 85 c0                       	test	eax, eax
140b2ea45: 0f 84 53 04 00 00           	je	0x140b2ee9e <.text+0xb2de9e>
140b2ea4b: 83 f8 01                    	cmp	eax, 0x1
140b2ea4e: 48 8b 07                    	mov	rax, qword ptr [rdi]
140b2ea51: f0                          	lock
140b2ea52: 48 ff 08                    	dec	qword ptr [rax]
140b2ea55: 0f 85 43 04 00 00           	jne	0x140b2ee9e <.text+0xb2de9e>
140b2ea5b: 48 89 f9                    	mov	rcx, rdi
140b2ea5e: e8 cd d8 02 00              	call	0x140b5c330 <.text+0xb5b330>
140b2ea63: e9 36 04 00 00              	jmp	0x140b2ee9e <.text+0xb2de9e>
140b2ea68: 48 8d 48 10                 	lea	rcx, [rax + 0x10]
140b2ea6c: f0                          	lock
140b2ea6d: 48 0f b1 0b                 	cmpxchg	qword ptr [rbx], rcx
140b2ea71: 0f 85 5f ff ff ff           	jne	0x140b2e9d6 <.text+0xb2d9d6>
140b2ea77: 48 89 d9                    	mov	rcx, rbx
140b2ea7a: e8 f1 31 15 00              	call	0x140c81c70 <.text+0xc80c70>
140b2ea7f: 49 89 de                    	mov	r14, rbx
140b2ea82: 49 83 ce 01                 	or	r14, 0x1
140b2ea86: 4c 89 f1                    	mov	rcx, r14
140b2ea89: e8 e2 31 15 00              	call	0x140c81c70 <.text+0xc80c70>
140b2ea8e: 4c 8b ad e0 00 00 00        	mov	r13, qword ptr [rbp + 0xe0]
140b2ea95: 49 83 bd 80 00 00 00 00     	cmp	qword ptr [r13 + 0x80], 0x0
140b2ea9d: 0f 84 24 01 00 00           	je	0x140b2ebc7 <.text+0xb2dbc7>
140b2eaa3: 49 8d 55 38                 	lea	rdx, [r13 + 0x38]
140b2eaa7: 49 8d 8d 88 00 00 00        	lea	rcx, [r13 + 0x88]
140b2eaae: 48 89 95 c0 00 00 00        	mov	qword ptr [rbp + 0xc0], rdx
140b2eab5: e8 b6 22 a8 ff              	call	0x1405b0d70 <.text+0x5afd70>
140b2eaba: 49 8b 4d 68                 	mov	rcx, qword ptr [r13 + 0x68]
140b2eabe: 49 8b 55 70                 	mov	rdx, qword ptr [r13 + 0x70]
140b2eac2: 49 89 d0                    	mov	r8, rdx
140b2eac5: 49 21 c0                    	and	r8, rax
140b2eac8: 48 c1 e8 39                 	shr	rax, 0x39
140b2eacc: 66 0f 6e c0                 	movd	xmm0, eax
140b2ead0: 66 0f 60 c0                 	punpcklbw	xmm0, xmm0      # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140b2ead4: f2 0f 70 c0 00              	pshuflw	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140b2ead9: 66 0f 70 c0 00              	pshufd	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0]
140b2eade: 4c 8d 51 f0                 	lea	r10, [rcx - 0x10]
140b2eae2: 31 c0                       	xor	eax, eax
140b2eae4: 66 0f 76 c9                 	pcmpeqd	xmm1, xmm1
140b2eae8: 45 31 ed                    	xor	r13d, r13d
140b2eaeb: f3 42 0f 6f 14 01           	movdqu	xmm2, xmmword ptr [rcx + r8]
140b2eaf1: 66 0f 6f da                 	movdqa	xmm3, xmm2
140b2eaf5: 66 0f 74 d8                 	pcmpeqb	xmm3, xmm0
140b2eaf9: 66 44 0f d7 db              	pmovmskb	r11d, xmm3
140b2eafe: 45 85 db                    	test	r11d, r11d
140b2eb01: 74 76                       	je	0x140b2eb79 <.text+0xb2db79>
140b2eb03: 4c 8b 8d c0 00 00 00        	mov	r9, qword ptr [rbp + 0xc0]
140b2eb0a: 4d 8b 09                    	mov	r9, qword ptr [r9]
140b2eb0d: 4c 89 4d 08                 	mov	qword ptr [rbp + 0x8], r9
140b2eb11: 48 89 9d a8 00 00 00        	mov	qword ptr [rbp + 0xa8], rbx
140b2eb18: 4c 89 ad b8 00 00 00        	mov	qword ptr [rbp + 0xb8], r13
140b2eb1f: 4c 89 e3                    	mov	rbx, r12
140b2eb22: 4d 89 fc                    	mov	r12, r15
140b2eb25: 49 89 ff                    	mov	r15, rdi
140b2eb28: 49 89 f5                    	mov	r13, rsi
140b2eb2b: 4c 89 d7                    	mov	rdi, r10
140b2eb2e: f3 45 0f bc d3              	tzcnt	r10d, r11d
140b2eb33: 4d 01 c2                    	add	r10, r8
140b2eb36: 49 21 d2                    	and	r10, rdx
140b2eb39: 4c 89 d6                    	mov	rsi, r10
140b2eb3c: 48 c1 e6 04                 	shl	rsi, 0x4
140b2eb40: 49 89 f9                    	mov	r9, rdi
140b2eb43: 49 29 f1                    	sub	r9, rsi
140b2eb46: 48 8b 75 08                 	mov	rsi, qword ptr [rbp + 0x8]
140b2eb4a: 49 3b 31                    	cmp	rsi, qword ptr [r9]
140b2eb4d: 74 4b                       	je	0x140b2eb9a <.text+0xb2db9a>
140b2eb4f: 45 8d 4b ff                 	lea	r9d, [r11 - 0x1]
140b2eb53: 66 45 21 d9                 	and	r9w, r11w
140b2eb57: 45 89 cb                    	mov	r11d, r9d
140b2eb5a: 4c 89 ee                    	mov	rsi, r13
140b2eb5d: 49 89 fa                    	mov	r10, rdi
140b2eb60: 4c 89 ff                    	mov	rdi, r15
140b2eb63: 4d 89 e7                    	mov	r15, r12
140b2eb66: 49 89 dc                    	mov	r12, rbx
140b2eb69: 48 8b 9d a8 00 00 00        	mov	rbx, qword ptr [rbp + 0xa8]
140b2eb70: 4c 8b ad b8 00 00 00        	mov	r13, qword ptr [rbp + 0xb8]
140b2eb77: 75 a6                       	jne	0x140b2eb1f <.text+0xb2db1f>
140b2eb79: 66 0f 74 d1                 	pcmpeqb	xmm2, xmm1
140b2eb7d: 66 44 0f d7 ca              	pmovmskb	r9d, xmm2
140b2eb82: 45 85 c9                    	test	r9d, r9d
140b2eb85: 75 33                       	jne	0x140b2ebba <.text+0xb2dbba>
140b2eb87: 4d 01 e8                    	add	r8, r13
140b2eb8a: 49 83 c0 10                 	add	r8, 0x10
140b2eb8e: 49 83 c5 10                 	add	r13, 0x10
140b2eb92: 49 21 d0                    	and	r8, rdx
140b2eb95: e9 51 ff ff ff              	jmp	0x140b2eaeb <.text+0xb2daeb>
140b2eb9a: 49 f7 da                    	neg	r10
140b2eb9d: 49 c1 e2 04                 	shl	r10, 0x4
140b2eba1: 4c 01 d1                    	add	rcx, r10
140b2eba4: 48 89 c8                    	mov	rax, rcx
140b2eba7: 4c 89 ee                    	mov	rsi, r13
140b2ebaa: 4c 89 ff                    	mov	rdi, r15
140b2ebad: 4d 89 e7                    	mov	r15, r12
140b2ebb0: 49 89 dc                    	mov	r12, rbx
140b2ebb3: 48 8b 9d a8 00 00 00        	mov	rbx, qword ptr [rbp + 0xa8]
140b2ebba: 48 8d 48 f0                 	lea	rcx, [rax - 0x10]
140b2ebbe: 48 85 c0                    	test	rax, rax
140b2ebc1: 48 0f 44 c8                 	cmove	rcx, rax
140b2ebc5: eb 02                       	jmp	0x140b2ebc9 <.text+0xb2dbc9>
140b2ebc7: 31 c9                       	xor	ecx, ecx
140b2ebc9: 48 8d 41 08                 	lea	rax, [rcx + 0x8]
140b2ebcd: 48 85 c9                    	test	rcx, rcx
140b2ebd0: 48 0f 44 c7                 	cmove	rax, rdi
140b2ebd4: 48 8b 00                    	mov	rax, qword ptr [rax]
140b2ebd7: f0                          	lock
140b2ebd8: 48 ff 00                    	inc	qword ptr [rax]
140b2ebdb: 0f 8e e4 06 00 00           	jle	0x140b2f2c5 <.text+0xb2e2c5>
140b2ebe1: 48 89 45 20                 	mov	qword ptr [rbp + 0x20], rax
140b2ebe5: c6 85 df 00 00 00 01        	mov	byte ptr [rbp + 0xdf], 0x1
140b2ebec: 48 89 d9                    	mov	rcx, rbx
140b2ebef: e8 1c 32 15 00              	call	0x140c81e10 <.text+0xc80e10>
140b2ebf4: c6 85 df 00 00 00 01        	mov	byte ptr [rbp + 0xdf], 0x1
140b2ebfb: 4c 89 f1                    	mov	rcx, r14
140b2ebfe: e8 0d 32 15 00              	call	0x140c81e10 <.text+0xc80e10>
140b2ec03: 48 c7 c0 f0 ff ff ff        	mov	rax, -0x10
140b2ec0a: f0                          	lock
140b2ec0b: 48 0f c1 03                 	xadd	qword ptr [rbx], rax
140b2ec0f: 48 83 e0 f2                 	and	rax, -0xe
140b2ec13: 48 83 f8 12                 	cmp	rax, 0x12
140b2ec17: 0f 84 88 06 00 00           	je	0x140b2f2a5 <.text+0xb2e2a5>
140b2ec1d: 48 8b 95 e0 00 00 00        	mov	rdx, qword ptr [rbp + 0xe0]
140b2ec24: 48 8d 4a 28                 	lea	rcx, [rdx + 0x28]
140b2ec28: 48 8b 41 30                 	mov	rax, qword ptr [rcx + 0x30]
140b2ec2c: 48 89 85 90 00 00 00        	mov	qword ptr [rbp + 0x90], rax
140b2ec33: 0f 10 01                    	movups	xmm0, xmmword ptr [rcx]
140b2ec36: 0f 10 49 10                 	movups	xmm1, xmmword ptr [rcx + 0x10]
140b2ec3a: 48 89 8d c0 00 00 00        	mov	qword ptr [rbp + 0xc0], rcx
140b2ec41: 0f 10 51 20                 	movups	xmm2, xmmword ptr [rcx + 0x20]
140b2ec45: 0f 29 95 80 00 00 00        	movaps	xmmword ptr [rbp + 0x80], xmm2
140b2ec4c: 0f 29 4d 70                 	movaps	xmmword ptr [rbp + 0x70], xmm1
140b2ec50: 0f 29 45 60                 	movaps	xmmword ptr [rbp + 0x60], xmm0
140b2ec54: 48 8b 45 20                 	mov	rax, qword ptr [rbp + 0x20]
140b2ec58: 48 89 85 a8 00 00 00        	mov	qword ptr [rbp + 0xa8], rax
140b2ec5f: 8b 46 1c                    	mov	eax, dword ptr [rsi + 0x1c]
140b2ec62: 48 8b 4e 10                 	mov	rcx, qword ptr [rsi + 0x10]
140b2ec66: 89 01                       	mov	dword ptr [rcx], eax
140b2ec68: 0f b6 42 18                 	movzx	eax, byte ptr [rdx + 0x18]
140b2ec6c: 85 c0                       	test	eax, eax
140b2ec6e: 74 14                       	je	0x140b2ec84 <.text+0xb2dc84>
140b2ec70: 83 f8 01                    	cmp	eax, 0x1
140b2ec73: 48 8b 07                    	mov	rax, qword ptr [rdi]
140b2ec76: f0                          	lock
140b2ec77: 48 ff 08                    	dec	qword ptr [rax]
140b2ec7a: 75 08                       	jne	0x140b2ec84 <.text+0xb2dc84>
140b2ec7c: 48 89 f9                    	mov	rcx, rdi
140b2ec7f: e8 ac d6 02 00              	call	0x140b5c330 <.text+0xb5b330>
140b2ec84: 4c 8b 95 e0 00 00 00        	mov	r10, qword ptr [rbp + 0xe0]
140b2ec8b: 66 41 c7 42 18 02 04        	mov	word ptr [r10 + 0x18], 0x402
140b2ec92: 8b 85 d0 00 00 00           	mov	eax, dword ptr [rbp + 0xd0]
140b2ec98: 41 89 42 1a                 	mov	dword ptr [r10 + 0x1a], eax
140b2ec9c: 0f b7 85 d4 00 00 00        	movzx	eax, word ptr [rbp + 0xd4]
140b2eca3: 66 41 89 42 1e              	mov	word ptr [r10 + 0x1e], ax
140b2eca8: 48 8b 85 a8 00 00 00        	mov	rax, qword ptr [rbp + 0xa8]
140b2ecaf: 49 89 42 20                 	mov	qword ptr [r10 + 0x20], rax
140b2ecb3: 48 8b 85 90 00 00 00        	mov	rax, qword ptr [rbp + 0x90]
140b2ecba: 48 8b 8d c0 00 00 00        	mov	rcx, qword ptr [rbp + 0xc0]
140b2ecc1: 48 89 41 30                 	mov	qword ptr [rcx + 0x30], rax
140b2ecc5: 0f 28 45 60                 	movaps	xmm0, xmmword ptr [rbp + 0x60]
140b2ecc9: 66 0f 6f 4d 70              	movdqa	xmm1, xmmword ptr [rbp + 0x70]
140b2ecce: 66 0f 6f 95 80 00 00 00     	movdqa	xmm2, xmmword ptr [rbp + 0x80]
140b2ecd6: f3 0f 7f 51 20              	movdqu	xmmword ptr [rcx + 0x20], xmm2
140b2ecdb: f3 0f 7f 49 10              	movdqu	xmmword ptr [rcx + 0x10], xmm1
140b2ece0: 0f 11 01                    	movups	xmmword ptr [rcx], xmm0
140b2ece3: 49 8b 42 20                 	mov	rax, qword ptr [r10 + 0x20]
140b2ece7: 0f b6 88 88 00 00 00        	movzx	ecx, byte ptr [rax + 0x88]
140b2ecee: 83 f9 02                    	cmp	ecx, 0x2
140b2ecf1: 0f 84 22 02 00 00           	je	0x140b2ef19 <.text+0xb2df19>
140b2ecf7: 83 f9 01                    	cmp	ecx, 0x1
140b2ecfa: 75 5b                       	jne	0x140b2ed57 <.text+0xb2dd57>
140b2ecfc: 0f b6 b8 89 00 00 00        	movzx	edi, byte ptr [rax + 0x89]
140b2ed03: 41 8b 47 1c                 	mov	eax, dword ptr [r15 + 0x1c]
140b2ed07: 49 8b 4f 10                 	mov	rcx, qword ptr [r15 + 0x10]
140b2ed0b: 89 01                       	mov	dword ptr [rcx], eax
140b2ed0d: 49 8b 1f                    	mov	rbx, qword ptr [r15]
140b2ed10: 4c 8b 73 30                 	mov	r14, qword ptr [rbx + 0x30]
140b2ed14: 4c 8b 7b 40                 	mov	r15, qword ptr [rbx + 0x40]
140b2ed18: 4c 3b 73 20                 	cmp	r14, qword ptr [rbx + 0x20]
140b2ed1c: 75 10                       	jne	0x140b2ed2e <.text+0xb2dd2e>
140b2ed1e: 48 8d 4b 20                 	lea	rcx, [rbx + 0x20]
140b2ed22: 48 8d 15 1f b4 10 02        	lea	rdx, [rip + 0x210b41f]  # 0x142c3a148
140b2ed29: e8 02 3f a3 ff              	call	0x140562c30 <.text+0x561c30>
140b2ed2e: 48 8d 43 40                 	lea	rax, [rbx + 0x40]
140b2ed32: 48 8b 53 28                 	mov	rdx, qword ptr [rbx + 0x28]
140b2ed36: 48 83 c3 30                 	add	rbx, 0x30
140b2ed3a: 4d 89 f0                    	mov	r8, r14
140b2ed3d: 49 c1 e0 05                 	shl	r8, 0x5
140b2ed41: 4a 8d 0c 02                 	lea	rcx, [rdx + r8]
140b2ed45: 4e 89 3c 02                 	mov	qword ptr [rdx + r8], r15
140b2ed49: 42 c7 44 02 08 00 00 00 00  	mov	dword ptr [rdx + r8 + 0x8], 0x0
140b2ed52: e9 9c 02 00 00              	jmp	0x140b2eff3 <.text+0xb2dff3>
140b2ed57: 41 8b 47 1c                 	mov	eax, dword ptr [r15 + 0x1c]
140b2ed5b: 49 8b 4f 10                 	mov	rcx, qword ptr [r15 + 0x10]
140b2ed5f: 89 01                       	mov	dword ptr [rcx], eax
140b2ed61: 49 8b 1f                    	mov	rbx, qword ptr [r15]
140b2ed64: 4c 8b 73 30                 	mov	r14, qword ptr [rbx + 0x30]
140b2ed68: 48 8b 7b 40                 	mov	rdi, qword ptr [rbx + 0x40]
140b2ed6c: 4c 3b 73 20                 	cmp	r14, qword ptr [rbx + 0x20]
140b2ed70: 75 10                       	jne	0x140b2ed82 <.text+0xb2dd82>
140b2ed72: 48 8d 4b 20                 	lea	rcx, [rbx + 0x20]
140b2ed76: 48 8d 15 cb b3 10 02        	lea	rdx, [rip + 0x210b3cb]  # 0x142c3a148
140b2ed7d: e8 ae 3e a3 ff              	call	0x140562c30 <.text+0x561c30>
140b2ed82: 48 8d 43 40                 	lea	rax, [rbx + 0x40]
140b2ed86: 48 8b 53 28                 	mov	rdx, qword ptr [rbx + 0x28]
140b2ed8a: 48 83 c3 30                 	add	rbx, 0x30
140b2ed8e: 4d 89 f0                    	mov	r8, r14
140b2ed91: 49 c1 e0 05                 	shl	r8, 0x5
140b2ed95: 4a 8d 0c 02                 	lea	rcx, [rdx + r8]
140b2ed99: 4a 89 3c 02                 	mov	qword ptr [rdx + r8], rdi
140b2ed9d: 42 c7 44 02 08 00 00 00 00  	mov	dword ptr [rdx + r8 + 0x8], 0x0
140b2eda6: 31 ff                       	xor	edi, edi
140b2eda8: e9 46 02 00 00              	jmp	0x140b2eff3 <.text+0xb2dff3>
140b2edad: 48 85 c0                    	test	rax, rax
140b2edb0: 0f 85 d7 04 00 00           	jne	0x140b2f28d <.text+0xb2e28d>
140b2edb6: 31 d2                       	xor	edx, edx
140b2edb8: e8 13 9a dc 01              	call	0x1428f87d0 <.text+0x28f77d0>
140b2edbd: 48 89 c1                    	mov	rcx, rax
140b2edc0: 48 85 c0                    	test	rax, rax
140b2edc3: 0f 85 cd f2 ff ff           	jne	0x140b2e096 <.text+0xb2d096>
140b2edc9: e9 bf 04 00 00              	jmp	0x140b2f28d <.text+0xb2e28d>
140b2edce: 4c 89 d1                    	mov	rcx, r10
140b2edd1: 48 89 da                    	mov	rdx, rbx
140b2edd4: e8 87 ee ff ff              	call	0x140b2dc60 <.text+0xb2cc60>
140b2edd9: 48 89 85 d0 00 00 00        	mov	qword ptr [rbp + 0xd0], rax
140b2ede0: 48 8b 48 38                 	mov	rcx, qword ptr [rax + 0x38]
140b2ede4: 48 89 4d 70                 	mov	qword ptr [rbp + 0x70], rcx
140b2ede8: 48 89 85 c0 00 00 00        	mov	qword ptr [rbp + 0xc0], rax
140b2edef: 0f 10 40 28                 	movups	xmm0, xmmword ptr [rax + 0x28]
140b2edf3: 0f 29 45 60                 	movaps	xmmword ptr [rbp + 0x60], xmm0
140b2edf7: 48 8b 95 98 01 00 00        	mov	rdx, qword ptr [rbp + 0x198]
140b2edfe: 8b 42 1c                    	mov	eax, dword ptr [rdx + 0x1c]
140b2ee01: 48 8b 4a 10                 	mov	rcx, qword ptr [rdx + 0x10]
140b2ee05: 89 01                       	mov	dword ptr [rcx], eax
140b2ee07: 48 8b 1a                    	mov	rbx, qword ptr [rdx]
140b2ee0a: 4c 8b 73 30                 	mov	r14, qword ptr [rbx + 0x30]
140b2ee0e: 4c 8b 6b 40                 	mov	r13, qword ptr [rbx + 0x40]
140b2ee12: 4c 3b 73 20                 	cmp	r14, qword ptr [rbx + 0x20]
140b2ee16: 75 10                       	jne	0x140b2ee28 <.text+0xb2de28>
140b2ee18: 48 8d 4b 20                 	lea	rcx, [rbx + 0x20]
140b2ee1c: 48 8d 15 25 b3 10 02        	lea	rdx, [rip + 0x210b325]  # 0x142c3a148
140b2ee23: e8 08 3e a3 ff              	call	0x140562c30 <.text+0x561c30>
140b2ee28: 48 8b 43 28                 	mov	rax, qword ptr [rbx + 0x28]
140b2ee2c: 4c 89 f1                    	mov	rcx, r14
140b2ee2f: 48 c1 e1 05                 	shl	rcx, 0x5
140b2ee33: 48 8b 55 70                 	mov	rdx, qword ptr [rbp + 0x70]
140b2ee37: 48 89 54 08 10              	mov	qword ptr [rax + rcx + 0x10], rdx
140b2ee3c: 0f 28 45 60                 	movaps	xmm0, xmmword ptr [rbp + 0x60]
140b2ee40: 0f 11 04 08                 	movups	xmmword ptr [rax + rcx], xmm0
140b2ee44: 4c 89 6c 08 18              	mov	qword ptr [rax + rcx + 0x18], r13
140b2ee49: 49 ff c6                    	inc	r14
140b2ee4c: 4c 89 73 30                 	mov	qword ptr [rbx + 0x30], r14
140b2ee50: 48 ff 43 40                 	inc	qword ptr [rbx + 0x40]
140b2ee54: 48 8b 45 10                 	mov	rax, qword ptr [rbp + 0x10]
140b2ee58: 48 89 45 20                 	mov	qword ptr [rbp + 0x20], rax
140b2ee5c: 8b 45 18                    	mov	eax, dword ptr [rbp + 0x18]
140b2ee5f: 89 45 28                    	mov	dword ptr [rbp + 0x28], eax
140b2ee62: 0f 28 45 e0                 	movaps	xmm0, xmmword ptr [rbp - 0x20]
140b2ee66: 0f 28 4d f0                 	movaps	xmm1, xmmword ptr [rbp - 0x10]
140b2ee6a: 0f 29 45 60                 	movaps	xmmword ptr [rbp + 0x60], xmm0
140b2ee6e: 0f 29 4d 70                 	movaps	xmmword ptr [rbp + 0x70], xmm1
140b2ee72: 8b 46 1c                    	mov	eax, dword ptr [rsi + 0x1c]
140b2ee75: 48 8b 4e 10                 	mov	rcx, qword ptr [rsi + 0x10]
140b2ee79: 89 01                       	mov	dword ptr [rcx], eax
140b2ee7b: 48 8b 85 e0 00 00 00        	mov	rax, qword ptr [rbp + 0xe0]
140b2ee82: 0f b6 40 18                 	movzx	eax, byte ptr [rax + 0x18]
140b2ee86: 85 c0                       	test	eax, eax
140b2ee88: 74 14                       	je	0x140b2ee9e <.text+0xb2de9e>
140b2ee8a: 83 f8 01                    	cmp	eax, 0x1
140b2ee8d: 48 8b 07                    	mov	rax, qword ptr [rdi]
140b2ee90: f0                          	lock
140b2ee91: 48 ff 08                    	dec	qword ptr [rax]
140b2ee94: 75 08                       	jne	0x140b2ee9e <.text+0xb2de9e>
140b2ee96: 48 89 f9                    	mov	rcx, rdi
140b2ee99: e8 92 d4 02 00              	call	0x140b5c330 <.text+0xb5b330>
140b2ee9e: 4c 8b 95 e0 00 00 00        	mov	r10, qword ptr [rbp + 0xe0]
140b2eea5: 66 41 c7 42 18 01 01        	mov	word ptr [r10 + 0x18], 0x101
140b2eeac: 8b 85 d0 00 00 00           	mov	eax, dword ptr [rbp + 0xd0]
140b2eeb2: 41 89 42 1a                 	mov	dword ptr [r10 + 0x1a], eax
140b2eeb6: 0f b7 85 d4 00 00 00        	movzx	eax, word ptr [rbp + 0xd4]
140b2eebd: 66 41 89 42 1e              	mov	word ptr [r10 + 0x1e], ax
140b2eec2: 48 8b 85 c0 00 00 00        	mov	rax, qword ptr [rbp + 0xc0]
140b2eec9: 49 89 42 20                 	mov	qword ptr [r10 + 0x20], rax
140b2eecd: 8b 85 cc 00 00 00           	mov	eax, dword ptr [rbp + 0xcc]
140b2eed3: 41 89 42 28                 	mov	dword ptr [r10 + 0x28], eax
140b2eed7: 48 8b 45 20                 	mov	rax, qword ptr [rbp + 0x20]
140b2eedb: 49 89 42 2c                 	mov	qword ptr [r10 + 0x2c], rax
140b2eedf: 8b 45 28                    	mov	eax, dword ptr [rbp + 0x28]
140b2eee2: 41 89 42 34                 	mov	dword ptr [r10 + 0x34], eax
140b2eee6: 48 8b 85 b8 00 00 00        	mov	rax, qword ptr [rbp + 0xb8]
140b2eeed: 49 89 42 38                 	mov	qword ptr [r10 + 0x38], rax
140b2eef1: 0f 28 45 60                 	movaps	xmm0, xmmword ptr [rbp + 0x60]
140b2eef5: 66 0f 6f 4d 70              	movdqa	xmm1, xmmword ptr [rbp + 0x70]
140b2eefa: 41 0f 11 42 40              	movups	xmmword ptr [r10 + 0x40], xmm0
140b2eeff: f3 41 0f 7f 4a 50           	movdqu	xmmword ptr [r10 + 0x50], xmm1
140b2ef05: 49 8b 42 20                 	mov	rax, qword ptr [r10 + 0x20]
140b2ef09: 0f b6 88 88 00 00 00        	movzx	ecx, byte ptr [rax + 0x88]
140b2ef10: 83 f9 02                    	cmp	ecx, 0x2
140b2ef13: 0f 85 de fd ff ff           	jne	0x140b2ecf7 <.text+0xb2dcf7>
140b2ef19: 0f b6 b8 89 00 00 00        	movzx	edi, byte ptr [rax + 0x89]
140b2ef20: f2 0f 10 b8 8c 00 00 00     	movsd	xmm7, qword ptr [rax + 0x8c]
140b2ef28: f3 0f 10 b0 94 00 00 00     	movss	xmm6, dword ptr [rax + 0x94]
140b2ef30: 0f b6 88 98 00 00 00        	movzx	ecx, byte ptr [rax + 0x98]
140b2ef37: 48 8d 15 32 cb fb 01        	lea	rdx, [rip + 0x1fbcb32]  # 0x142aeba70
140b2ef3e: 48 63 0c 8a                 	movsxd	rcx, dword ptr [rdx + 4*rcx]
140b2ef42: 48 01 d1                    	add	rcx, rdx
140b2ef45: ff e1                       	jmp	rcx
140b2ef47: 45 31 ed                    	xor	r13d, r13d
140b2ef4a: eb 4f                       	jmp	0x140b2ef9b <.text+0xb2df9b>
140b2ef4c: b9 08 00 00 00              	mov	ecx, 0x8
140b2ef51: eb 2d                       	jmp	0x140b2ef80 <.text+0xb2df80>
140b2ef53: 49 8b 4a 40                 	mov	rcx, qword ptr [r10 + 0x40]
140b2ef57: 48 8b 90 b0 00 00 00        	mov	rdx, qword ptr [rax + 0xb0]
140b2ef5e: 48 39 ca                    	cmp	rdx, rcx
140b2ef61: 0f 86 52 03 00 00           	jbe	0x140b2f2b9 <.text+0xb2e2b9>
140b2ef67: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
140b2ef6b: 48 c1 e1 02                 	shl	rcx, 0x2
140b2ef6f: 48 03 88 a8 00 00 00        	add	rcx, qword ptr [rax + 0xa8]
140b2ef76: 48 89 c8                    	mov	rax, rcx
140b2ef79: eb 0e                       	jmp	0x140b2ef89 <.text+0xb2df89>
140b2ef7b: b9 04 00 00 00              	mov	ecx, 0x4
140b2ef80: 48 05 98 00 00 00           	add	rax, 0x98
140b2ef86: 48 01 c8                    	add	rax, rcx
140b2ef89: f2 0f 10 00                 	movsd	xmm0, qword ptr [rax]
140b2ef8d: 0f 58 f8                    	addps	xmm7, xmm0
140b2ef90: f3 0f 58 70 08              	addss	xmm6, dword ptr [rax + 0x8]
140b2ef95: 41 bd 01 00 00 00           	mov	r13d, 0x1
140b2ef9b: 41 8b 47 1c                 	mov	eax, dword ptr [r15 + 0x1c]
140b2ef9f: 49 8b 4f 10                 	mov	rcx, qword ptr [r15 + 0x10]
140b2efa3: 89 01                       	mov	dword ptr [rcx], eax
140b2efa5: 49 8b 1f                    	mov	rbx, qword ptr [r15]
140b2efa8: 4c 8b 73 30                 	mov	r14, qword ptr [rbx + 0x30]
140b2efac: 4c 8b 7b 40                 	mov	r15, qword ptr [rbx + 0x40]
140b2efb0: 4c 3b 73 20                 	cmp	r14, qword ptr [rbx + 0x20]
140b2efb4: 75 10                       	jne	0x140b2efc6 <.text+0xb2dfc6>
140b2efb6: 48 8d 4b 20                 	lea	rcx, [rbx + 0x20]
140b2efba: 48 8d 15 87 b1 10 02        	lea	rdx, [rip + 0x210b187]  # 0x142c3a148
140b2efc1: e8 6a 3c a3 ff              	call	0x140562c30 <.text+0x561c30>
140b2efc6: 48 8d 43 40                 	lea	rax, [rbx + 0x40]
140b2efca: 48 8b 53 28                 	mov	rdx, qword ptr [rbx + 0x28]
140b2efce: 48 83 c3 30                 	add	rbx, 0x30
140b2efd2: 4d 89 f0                    	mov	r8, r14
140b2efd5: 49 c1 e0 05                 	shl	r8, 0x5
140b2efd9: 4a 8d 0c 02                 	lea	rcx, [rdx + r8]
140b2efdd: 4e 89 3c 02                 	mov	qword ptr [rdx + r8], r15
140b2efe1: 46 89 6c 02 08              	mov	dword ptr [rdx + r8 + 0x8], r13d
140b2efe6: 42 0f 13 7c 02 0c           	movlps	qword ptr [rdx + r8 + 0xc], xmm7
140b2efec: f3 42 0f 11 74 02 14        	movss	dword ptr [rdx + r8 + 0x14], xmm6
140b2eff3: 40 88 79 18                 	mov	byte ptr [rcx + 0x18], dil
140b2eff7: 49 ff c6                    	inc	r14
140b2effa: 4c 89 33                    	mov	qword ptr [rbx], r14
140b2effd: 48 ff 00                    	inc	qword ptr [rax]
140b2f000: 48 8b 85 e0 00 00 00        	mov	rax, qword ptr [rbp + 0xe0]
140b2f007: f6 40 59 08                 	test	byte ptr [rax + 0x59], 0x8
140b2f00b: 74 44                       	je	0x140b2f051 <.text+0xb2e051>
140b2f00d: 41 83 3c 24 03              	cmp	dword ptr [r12], 0x3
140b2f012: 73 3d                       	jae	0x140b2f051 <.text+0xb2e051>
140b2f014: 48 8b 95 90 01 00 00        	mov	rdx, qword ptr [rbp + 0x190]
140b2f01b: 8b 42 1c                    	mov	eax, dword ptr [rdx + 0x1c]
140b2f01e: 48 8b 4a 10                 	mov	rcx, qword ptr [rdx + 0x10]
140b2f022: 89 01                       	mov	dword ptr [rcx], eax
140b2f024: 48 8b 3a                    	mov	rdi, qword ptr [rdx]
140b2f027: 48 8b 5f 30                 	mov	rbx, qword ptr [rdi + 0x30]
140b2f02b: 4c 8b 77 40                 	mov	r14, qword ptr [rdi + 0x40]
140b2f02f: 49 bf 02 00 00 00 01 00 00 00       	movabs	r15, 0x100000002
140b2f039: 48 3b 5f 20                 	cmp	rbx, qword ptr [rdi + 0x20]
140b2f03d: 75 4d                       	jne	0x140b2f08c <.text+0xb2e08c>
140b2f03f: 48 8d 4f 20                 	lea	rcx, [rdi + 0x20]
140b2f043: 48 8d 15 fe b0 10 02        	lea	rdx, [rip + 0x210b0fe]  # 0x142c3a148
140b2f04a: e8 e1 35 a3 ff              	call	0x140562630 <.text+0x561630>
140b2f04f: eb 3b                       	jmp	0x140b2f08c <.text+0xb2e08c>
140b2f051: 48 8b 95 90 01 00 00        	mov	rdx, qword ptr [rbp + 0x190]
140b2f058: 8b 42 1c                    	mov	eax, dword ptr [rdx + 0x1c]
140b2f05b: 48 8b 4a 10                 	mov	rcx, qword ptr [rdx + 0x10]
140b2f05f: 89 01                       	mov	dword ptr [rcx], eax
140b2f061: 48 8b 3a                    	mov	rdi, qword ptr [rdx]
140b2f064: 48 8b 5f 30                 	mov	rbx, qword ptr [rdi + 0x30]
140b2f068: 4c 8b 77 40                 	mov	r14, qword ptr [rdi + 0x40]
140b2f06c: 49 bf 03 00 00 00 01 00 00 00       	movabs	r15, 0x100000003
140b2f076: 48 3b 5f 20                 	cmp	rbx, qword ptr [rdi + 0x20]
140b2f07a: 75 10                       	jne	0x140b2f08c <.text+0xb2e08c>
140b2f07c: 48 8d 4f 20                 	lea	rcx, [rdi + 0x20]
140b2f080: 48 8d 15 c1 b0 10 02        	lea	rdx, [rip + 0x210b0c1]  # 0x142c3a148
140b2f087: e8 a4 35 a3 ff              	call	0x140562630 <.text+0x561630>
140b2f08c: 48 8b 47 28                 	mov	rax, qword ptr [rdi + 0x28]
140b2f090: 48 89 d9                    	mov	rcx, rbx
140b2f093: 48 c1 e1 04                 	shl	rcx, 0x4
140b2f097: 4c 89 3c 08                 	mov	qword ptr [rax + rcx], r15
140b2f09b: 4c 89 74 08 08              	mov	qword ptr [rax + rcx + 0x8], r14
140b2f0a0: 48 ff c3                    	inc	rbx
140b2f0a3: 48 89 5f 30                 	mov	qword ptr [rdi + 0x30], rbx
140b2f0a7: 48 ff 47 40                 	inc	qword ptr [rdi + 0x40]
140b2f0ab: 48 8b 8d e0 00 00 00        	mov	rcx, qword ptr [rbp + 0xe0]
140b2f0b2: 80 79 18 02                 	cmp	byte ptr [rcx + 0x18], 0x2
140b2f0b6: 0f 82 42 01 00 00           	jb	0x140b2f1fe <.text+0xb2e1fe>
140b2f0bc: 0f b6 41 19                 	movzx	eax, byte ptr [rcx + 0x19]
140b2f0c0: 8d 50 fe                    	lea	edx, [rax - 0x2]
140b2f0c3: 0f b6 d2                    	movzx	edx, dl
140b2f0c6: 80 fa 03                    	cmp	dl, 0x3
140b2f0c9: 41 b8 03 00 00 00           	mov	r8d, 0x3
140b2f0cf: 44 0f 42 c2                 	cmovb	r8d, edx
140b2f0d3: 41 80 f8 01                 	cmp	r8b, 0x1
140b2f0d7: 74 66                       	je	0x140b2f13f <.text+0xb2e13f>
140b2f0d9: 41 0f b6 d0                 	movzx	edx, r8b
140b2f0dd: 83 fa 03                    	cmp	edx, 0x3
140b2f0e0: 0f 85 18 01 00 00           	jne	0x140b2f1fe <.text+0xb2e1fe>
140b2f0e6: a8 01                       	test	al, 0x1
140b2f0e8: 0f 84 98 00 00 00           	je	0x140b2f186 <.text+0xb2e186>
140b2f0ee: 48 8b 41 20                 	mov	rax, qword ptr [rcx + 0x20]
140b2f0f2: 48 8b 88 80 00 00 00        	mov	rcx, qword ptr [rax + 0x80]
140b2f0f9: 48 89 4d 70                 	mov	qword ptr [rbp + 0x70], rcx
140b2f0fd: 0f 10 40 70                 	movups	xmm0, xmmword ptr [rax + 0x70]
140b2f101: 0f 29 45 60                 	movaps	xmmword ptr [rbp + 0x60], xmm0
140b2f105: 48 8b 95 98 01 00 00        	mov	rdx, qword ptr [rbp + 0x198]
140b2f10c: 8b 42 1c                    	mov	eax, dword ptr [rdx + 0x1c]
140b2f10f: 48 8b 4a 10                 	mov	rcx, qword ptr [rdx + 0x10]
140b2f113: 89 01                       	mov	dword ptr [rcx], eax
140b2f115: 48 8b 3a                    	mov	rdi, qword ptr [rdx]
140b2f118: 48 8b 5f 30                 	mov	rbx, qword ptr [rdi + 0x30]
140b2f11c: 4c 8b 77 40                 	mov	r14, qword ptr [rdi + 0x40]
140b2f120: 48 3b 5f 20                 	cmp	rbx, qword ptr [rdi + 0x20]
140b2f124: 0f 85 a1 00 00 00           	jne	0x140b2f1cb <.text+0xb2e1cb>
140b2f12a: 48 8d 4f 20                 	lea	rcx, [rdi + 0x20]
140b2f12e: 48 8d 15 13 b0 10 02        	lea	rdx, [rip + 0x210b013]  # 0x142c3a148
140b2f135: e8 f6 3a a3 ff              	call	0x140562c30 <.text+0x561c30>
140b2f13a: e9 8c 00 00 00              	jmp	0x140b2f1cb <.text+0xb2e1cb>
140b2f13f: 48 8b 41 20                 	mov	rax, qword ptr [rcx + 0x20]
140b2f143: 48 8b 48 50                 	mov	rcx, qword ptr [rax + 0x50]
140b2f147: 48 89 4d 70                 	mov	qword ptr [rbp + 0x70], rcx
140b2f14b: 0f 10 40 40                 	movups	xmm0, xmmword ptr [rax + 0x40]
140b2f14f: 0f 29 45 60                 	movaps	xmmword ptr [rbp + 0x60], xmm0
140b2f153: 48 8b 95 98 01 00 00        	mov	rdx, qword ptr [rbp + 0x198]
140b2f15a: 8b 42 1c                    	mov	eax, dword ptr [rdx + 0x1c]
140b2f15d: 48 8b 4a 10                 	mov	rcx, qword ptr [rdx + 0x10]
140b2f161: 89 01                       	mov	dword ptr [rcx], eax
140b2f163: 48 8b 3a                    	mov	rdi, qword ptr [rdx]
140b2f166: 48 8b 5f 30                 	mov	rbx, qword ptr [rdi + 0x30]
140b2f16a: 4c 8b 77 40                 	mov	r14, qword ptr [rdi + 0x40]
140b2f16e: 48 3b 5f 20                 	cmp	rbx, qword ptr [rdi + 0x20]
140b2f172: 75 57                       	jne	0x140b2f1cb <.text+0xb2e1cb>
140b2f174: 48 8d 4f 20                 	lea	rcx, [rdi + 0x20]
140b2f178: 48 8d 15 c9 af 10 02        	lea	rdx, [rip + 0x210afc9]  # 0x142c3a148
140b2f17f: e8 ac 3a a3 ff              	call	0x140562c30 <.text+0x561c30>
140b2f184: eb 45                       	jmp	0x140b2f1cb <.text+0xb2e1cb>
140b2f186: 48 8b 41 20                 	mov	rax, qword ptr [rcx + 0x20]
140b2f18a: 48 8b 48 68                 	mov	rcx, qword ptr [rax + 0x68]
140b2f18e: 48 89 4d 70                 	mov	qword ptr [rbp + 0x70], rcx
140b2f192: 0f 10 40 58                 	movups	xmm0, xmmword ptr [rax + 0x58]
140b2f196: 0f 29 45 60                 	movaps	xmmword ptr [rbp + 0x60], xmm0
140b2f19a: 48 8b 95 98 01 00 00        	mov	rdx, qword ptr [rbp + 0x198]
140b2f1a1: 8b 42 1c                    	mov	eax, dword ptr [rdx + 0x1c]
140b2f1a4: 48 8b 4a 10                 	mov	rcx, qword ptr [rdx + 0x10]
140b2f1a8: 89 01                       	mov	dword ptr [rcx], eax
140b2f1aa: 48 8b 3a                    	mov	rdi, qword ptr [rdx]
140b2f1ad: 48 8b 5f 30                 	mov	rbx, qword ptr [rdi + 0x30]
140b2f1b1: 4c 8b 77 40                 	mov	r14, qword ptr [rdi + 0x40]
140b2f1b5: 48 3b 5f 20                 	cmp	rbx, qword ptr [rdi + 0x20]
140b2f1b9: 75 10                       	jne	0x140b2f1cb <.text+0xb2e1cb>
140b2f1bb: 48 8d 4f 20                 	lea	rcx, [rdi + 0x20]
140b2f1bf: 48 8d 15 82 af 10 02        	lea	rdx, [rip + 0x210af82]  # 0x142c3a148
140b2f1c6: e8 65 3a a3 ff              	call	0x140562c30 <.text+0x561c30>
140b2f1cb: 48 8b 47 28                 	mov	rax, qword ptr [rdi + 0x28]
140b2f1cf: 48 89 d9                    	mov	rcx, rbx
140b2f1d2: 48 c1 e1 05                 	shl	rcx, 0x5
140b2f1d6: 48 8b 55 70                 	mov	rdx, qword ptr [rbp + 0x70]
140b2f1da: 48 89 54 08 10              	mov	qword ptr [rax + rcx + 0x10], rdx
140b2f1df: 0f 28 45 60                 	movaps	xmm0, xmmword ptr [rbp + 0x60]
140b2f1e3: 0f 11 04 08                 	movups	xmmword ptr [rax + rcx], xmm0
140b2f1e7: 4c 89 74 08 18              	mov	qword ptr [rax + rcx + 0x18], r14
140b2f1ec: 48 ff c3                    	inc	rbx
140b2f1ef: 48 89 5f 30                 	mov	qword ptr [rdi + 0x30], rbx
140b2f1f3: 48 ff 47 40                 	inc	qword ptr [rdi + 0x40]
140b2f1f7: 48 8b 8d e0 00 00 00        	mov	rcx, qword ptr [rbp + 0xe0]
140b2f1fe: 8b 46 1c                    	mov	eax, dword ptr [rsi + 0x1c]
140b2f201: 48 8b 56 10                 	mov	rdx, qword ptr [rsi + 0x10]
140b2f205: 89 02                       	mov	dword ptr [rdx], eax
140b2f207: 48 83 c1 68                 	add	rcx, 0x68
140b2f20b: e8 80 ac 04 00              	call	0x140b79e90 <.text+0xb78e90>
140b2f210: 48 83 7d d0 00              	cmp	qword ptr [rbp - 0x30], 0x0
140b2f215: 74 40                       	je	0x140b2f257 <.text+0xb2e257>
140b2f217: e8 14 8f 0e 00              	call	0x140c18130 <.text+0xc17130>
140b2f21c: 48 89 c1                    	mov	rcx, rax
140b2f21f: 48 8b 00                    	mov	rax, qword ptr [rax]
140b2f222: 48 83 f8 01                 	cmp	rax, 0x1
140b2f226: 75 51                       	jne	0x140b2f279 <.text+0xb2e279>
140b2f228: 48 83 c1 08                 	add	rcx, 0x8
140b2f22c: 48 83 39 00                 	cmp	qword ptr [rcx], 0x0
140b2f230: 75 67                       	jne	0x140b2f299 <.text+0xb2e299>
140b2f232: 48 c7 01 ff ff ff ff        	mov	qword ptr [rcx], -0x1
140b2f239: 48 89 8d e0 00 00 00        	mov	qword ptr [rbp + 0xe0], rcx
140b2f240: 48 83 c1 08                 	add	rcx, 0x8
140b2f244: 48 8b 55 d8                 	mov	rdx, qword ptr [rbp - 0x28]
140b2f248: e8 a3 7e 0e 00              	call	0x140c170f0 <.text+0xc160f0>
140b2f24d: 48 8b 85 e0 00 00 00        	mov	rax, qword ptr [rbp + 0xe0]
140b2f254: 48 ff 00                    	inc	qword ptr [rax]
140b2f257: 0f 28 b5 f0 00 00 00        	movaps	xmm6, xmmword ptr [rbp + 0xf0]
140b2f25e: 0f 28 bd 00 01 00 00        	movaps	xmm7, xmmword ptr [rbp + 0x100]
140b2f265: 48 81 c4 98 01 00 00        	add	rsp, 0x198
140b2f26c: 5b                          	pop	rbx
140b2f26d: 5f                          	pop	rdi
140b2f26e: 5e                          	pop	rsi
140b2f26f: 41 5c                       	pop	r12
140b2f271: 41 5d                       	pop	r13
140b2f273: 41 5e                       	pop	r14
140b2f275: 41 5f                       	pop	r15
140b2f277: 5d                          	pop	rbp
140b2f278: c3                          	ret
140b2f279: 48 85 c0                    	test	rax, rax
140b2f27c: 75 0f                       	jne	0x140b2f28d <.text+0xb2e28d>
140b2f27e: 31 d2                       	xor	edx, edx
140b2f280: e8 4b 95 dc 01              	call	0x1428f87d0 <.text+0x28f77d0>
140b2f285: 48 89 c1                    	mov	rcx, rax
140b2f288: 48 85 c0                    	test	rax, rax
140b2f28b: 75 9f                       	jne	0x140b2f22c <.text+0xb2e22c>
140b2f28d: 48 8d 0d 0c b8 10 02        	lea	rcx, [rip + 0x210b80c]  # 0x142c3aaa0
140b2f294: e8 37 8e da 01              	call	0x1428d80d0 <.text+0x28d70d0>
140b2f299: 48 8d 0d d0 b8 10 02        	lea	rcx, [rip + 0x210b8d0]  # 0x142c3ab70
140b2f2a0: e8 ab 9f da 01              	call	0x1428d9250 <.text+0x28d8250>
140b2f2a5: c6 85 df 00 00 00 01        	mov	byte ptr [rbp + 0xdf], 0x1
140b2f2ac: 48 89 d9                    	mov	rcx, rbx
140b2f2af: e8 9c dc d9 01              	call	0x1428ccf50 <.text+0x28cbf50>
140b2f2b4: e9 64 f9 ff ff              	jmp	0x140b2ec1d <.text+0xb2dc1d>
140b2f2b9: 4c 8d 05 68 e6 f8 01        	lea	r8, [rip + 0x1f8e668]   # 0x142abd928
140b2f2c0: e8 53 a2 da 01              	call	0x1428d9518 <.text+0x28d8518>
140b2f2c5: 0f 0b                       	ud2
140b2f2c7: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
140b2f2d0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140b2f2d5: 55                          	push	rbp
140b2f2d6: 41 57                       	push	r15
140b2f2d8: 41 56                       	push	r14
140b2f2da: 41 55                       	push	r13
140b2f2dc: 41 54                       	push	r12
140b2f2de: 56                          	push	rsi
140b2f2df: 57                          	push	rdi
140b2f2e0: 53                          	push	rbx
140b2f2e1: 48 83 ec 58                 	sub	rsp, 0x58
140b2f2e5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140b2f2ec: 0f 29 7c 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm7
140b2f2f1: 0f 29 74 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm6
140b2f2f6: 48 8b 85 e0 00 00 00        	mov	rax, qword ptr [rbp + 0xe0]
140b2f2fd: 48 ff 00                    	inc	qword ptr [rax]
140b2f300: 0f 28 74 24 40              	movaps	xmm6, xmmword ptr [rsp + 0x40]
140b2f305: 0f 28 7c 24 30              	movaps	xmm7, xmmword ptr [rsp + 0x30]
140b2f30a: 48 83 c4 58                 	add	rsp, 0x58
140b2f30e: 5b                          	pop	rbx
140b2f30f: 5f                          	pop	rdi
140b2f310: 5e                          	pop	rsi
140b2f311: 41 5c                       	pop	r12
140b2f313: 41 5d                       	pop	r13
140b2f315: 41 5e                       	pop	r14
140b2f317: 41 5f                       	pop	r15
140b2f319: 5d                          	pop	rbp
140b2f31a: c3                          	ret
140b2f31b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
140b2f320: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140b2f325: 55                          	push	rbp
140b2f326: 41 57                       	push	r15
140b2f328: 41 56                       	push	r14
140b2f32a: 41 55                       	push	r13
140b2f32c: 41 54                       	push	r12
140b2f32e: 56                          	push	rsi
140b2f32f: 57                          	push	rdi
140b2f330: 53                          	push	rbx
140b2f331: 48 83 ec 58                 	sub	rsp, 0x58
140b2f335: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140b2f33c: 0f 29 7c 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm7
140b2f341: 0f 29 74 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm6
140b2f346: 48 83 7d d0 00              	cmp	qword ptr [rbp - 0x30], 0x0
140b2f34b: 74 20                       	je	0x140b2f36d <.text+0xb2e36d>
140b2f34d: 48 8d 45 d8                 	lea	rax, [rbp - 0x28]
140b2f351: 48 89 45 c0                 	mov	qword ptr [rbp - 0x40], rax
140b2f355: 48 8d 45 c0                 	lea	rax, [rbp - 0x40]
140b2f359: 48 89 45 b8                 	mov	qword ptr [rbp - 0x48], rax
140b2f35d: 48 8d 0d 04 c0 fb 01        	lea	rcx, [rip + 0x1fbc004]  # 0x142aeb368
140b2f364: 48 8d 55 b8                 	lea	rdx, [rbp - 0x48]
140b2f368: e8 63 ff 7f 00              	call	0x14132f2d0 <.text+0x132e2d0>
140b2f36d: 0f 28 74 24 40              	movaps	xmm6, xmmword ptr [rsp + 0x40]
140b2f372: 0f 28 7c 24 30              	movaps	xmm7, xmmword ptr [rsp + 0x30]
140b2f377: 48 83 c4 58                 	add	rsp, 0x58
140b2f37b: 5b                          	pop	rbx
140b2f37c: 5f                          	pop	rdi
140b2f37d: 5e                          	pop	rsi
140b2f37e: 41 5c                       	pop	r12
140b2f380: 41 5d                       	pop	r13
140b2f382: 41 5e                       	pop	r14
140b2f384: 41 5f                       	pop	r15
140b2f386: 5d                          	pop	rbp
140b2f387: c3                          	ret
140b2f388: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
140b2f390: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140b2f395: 55                          	push	rbp
140b2f396: 41 57                       	push	r15
140b2f398: 41 56                       	push	r14
140b2f39a: 41 55                       	push	r13
140b2f39c: 41 54                       	push	r12
140b2f39e: 56                          	push	rsi
140b2f39f: 57                          	push	rdi
140b2f3a0: 53                          	push	rbx
140b2f3a1: 48 83 ec 58                 	sub	rsp, 0x58
140b2f3a5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140b2f3ac: 0f 29 7c 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm7
140b2f3b1: 0f 29 74 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm6
140b2f3b6: 48 8b 85 b0 00 00 00        	mov	rax, qword ptr [rbp + 0xb0]
140b2f3bd: f0                          	lock
140b2f3be: 48 ff 08                    	dec	qword ptr [rax]
140b2f3c1: 75 0c                       	jne	0x140b2f3cf <.text+0xb2e3cf>
140b2f3c3: 48 8d 8d d0 00 00 00        	lea	rcx, [rbp + 0xd0]
140b2f3ca: e8 61 cf 02 00              	call	0x140b5c330 <.text+0xb5b330>
140b2f3cf: 0f 28 74 24 40              	movaps	xmm6, xmmword ptr [rsp + 0x40]
140b2f3d4: 0f 28 7c 24 30              	movaps	xmm7, xmmword ptr [rsp + 0x30]
140b2f3d9: 48 83 c4 58                 	add	rsp, 0x58
140b2f3dd: 5b                          	pop	rbx
140b2f3de: 5f                          	pop	rdi
140b2f3df: 5e                          	pop	rsi
140b2f3e0: 41 5c                       	pop	r12
140b2f3e2: 41 5d                       	pop	r13
140b2f3e4: 41 5e                       	pop	r14
140b2f3e6: 41 5f                       	pop	r15
140b2f3e8: 5d                          	pop	rbp
140b2f3e9: c3                          	ret
140b2f3ea: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
140b2f3f0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140b2f3f5: 55                          	push	rbp
140b2f3f6: 41 57                       	push	r15
140b2f3f8: 41 56                       	push	r14
140b2f3fa: 41 55                       	push	r13
140b2f3fc: 41 54                       	push	r12
140b2f3fe: 56                          	push	rsi
140b2f3ff: 57                          	push	rdi
140b2f400: 53                          	push	rbx
140b2f401: 48 83 ec 58                 	sub	rsp, 0x58
140b2f405: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140b2f40c: 0f 29 7c 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm7
140b2f411: 0f 29 74 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm6
140b2f416: 48 8b 85 b0 00 00 00        	mov	rax, qword ptr [rbp + 0xb0]
140b2f41d: f0                          	lock
140b2f41e: 48 ff 08                    	dec	qword ptr [rax]
140b2f421: 75 0c                       	jne	0x140b2f42f <.text+0xb2e42f>
140b2f423: 48 8d 8d d0 00 00 00        	lea	rcx, [rbp + 0xd0]
140b2f42a: e8 01 cf 02 00              	call	0x140b5c330 <.text+0xb5b330>
140b2f42f: 0f 28 74 24 40              	movaps	xmm6, xmmword ptr [rsp + 0x40]
140b2f434: 0f 28 7c 24 30              	movaps	xmm7, xmmword ptr [rsp + 0x30]
140b2f439: 48 83 c4 58                 	add	rsp, 0x58
140b2f43d: 5b                          	pop	rbx
140b2f43e: 5f                          	pop	rdi
140b2f43f: 5e                          	pop	rsi
140b2f440: 41 5c                       	pop	r12
140b2f442: 41 5d                       	pop	r13
140b2f444: 41 5e                       	pop	r14
140b2f446: 41 5f                       	pop	r15
140b2f448: 5d                          	pop	rbp
140b2f449: c3                          	ret
140b2f44a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
140b2f450: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140b2f455: 55                          	push	rbp
140b2f456: 41 57                       	push	r15
140b2f458: 41 56                       	push	r14
140b2f45a: 41 55                       	push	r13
140b2f45c: 41 54                       	push	r12
140b2f45e: 56                          	push	rsi
140b2f45f: 57                          	push	rdi
140b2f460: 53                          	push	rbx
140b2f461: 48 83 ec 58                 	sub	rsp, 0x58
140b2f465: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140b2f46c: 0f 29 7c 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm7
140b2f471: 0f 29 74 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm6
140b2f476: 80 bd df 00 00 00 00        	cmp	byte ptr [rbp + 0xdf], 0x0
140b2f47d: 74 2e                       	je	0x140b2f4ad <.text+0xb2e4ad>
140b2f47f: 48 8b 45 20                 	mov	rax, qword ptr [rbp + 0x20]
140b2f483: f0                          	lock
140b2f484: 48 ff 08                    	dec	qword ptr [rax]
140b2f487: 75 09                       	jne	0x140b2f492 <.text+0xb2e492>
140b2f489: 48 8d 4d 20                 	lea	rcx, [rbp + 0x20]
140b2f48d: e8 9e ce 02 00              	call	0x140b5c330 <.text+0xb5b330>
140b2f492: 0f 28 74 24 40              	movaps	xmm6, xmmword ptr [rsp + 0x40]
140b2f497: 0f 28 7c 24 30              	movaps	xmm7, xmmword ptr [rsp + 0x30]
140b2f49c: 48 83 c4 58                 	add	rsp, 0x58
140b2f4a0: 5b                          	pop	rbx
140b2f4a1: 5f                          	pop	rdi
140b2f4a2: 5e                          	pop	rsi
140b2f4a3: 41 5c                       	pop	r12
140b2f4a5: 41 5d                       	pop	r13
140b2f4a7: 41 5e                       	pop	r14
140b2f4a9: 41 5f                       	pop	r15
140b2f4ab: 5d                          	pop	rbp
140b2f4ac: c3                          	ret
140b2f4ad: 0f 28 74 24 40              	movaps	xmm6, xmmword ptr [rsp + 0x40]
140b2f4b2: 0f 28 7c 24 30              	movaps	xmm7, xmmword ptr [rsp + 0x30]
140b2f4b7: 48 83 c4 58                 	add	rsp, 0x58
140b2f4bb: 5b                          	pop	rbx
140b2f4bc: 5f                          	pop	rdi
140b2f4bd: 5e                          	pop	rsi
140b2f4be: 41 5c                       	pop	r12
140b2f4c0: 41 5d                       	pop	r13
140b2f4c2: 41 5e                       	pop	r14
140b2f4c4: 41 5f                       	pop	r15
140b2f4c6: 5d                          	pop	rbp
140b2f4c7: c3                          	ret
140b2f4c8: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
140b2f4d0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140b2f4d5: 55                          	push	rbp
140b2f4d6: 41 57                       	push	r15
140b2f4d8: 41 56                       	push	r14
140b2f4da: 41 55                       	push	r13
140b2f4dc: 41 54                       	push	r12
140b2f4de: 56                          	push	rsi
140b2f4df: 57                          	push	rdi
140b2f4e0: 53                          	push	rbx
140b2f4e1: 48 83 ec 58                 	sub	rsp, 0x58
140b2f4e5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140b2f4ec: 0f 29 7c 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm7
140b2f4f1: 0f 29 74 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm6
140b2f4f6: 48 8b 85 c0 00 00 00        	mov	rax, qword ptr [rbp + 0xc0]
140b2f4fd: f0                          	lock
140b2f4fe: 48 ff 08                    	dec	qword ptr [rax]
140b2f501: 75 0c                       	jne	0x140b2f50f <.text+0xb2e50f>
140b2f503: 48 8d 8d d0 00 00 00        	lea	rcx, [rbp + 0xd0]
140b2f50a: e8 21 ce 02 00              	call	0x140b5c330 <.text+0xb5b330>
140b2f50f: 0f 28 74 24 40              	movaps	xmm6, xmmword ptr [rsp + 0x40]
140b2f514: 0f 28 7c 24 30              	movaps	xmm7, xmmword ptr [rsp + 0x30]
140b2f519: 48 83 c4 58                 	add	rsp, 0x58
140b2f51d: 5b                          	pop	rbx
140b2f51e: 5f                          	pop	rdi
140b2f51f: 5e                          	pop	rsi
140b2f520: 41 5c                       	pop	r12
140b2f522: 41 5d                       	pop	r13
140b2f524: 41 5e                       	pop	r14
140b2f526: 41 5f                       	pop	r15
140b2f528: 5d                          	pop	rbp
140b2f529: c3                          	ret
140b2f52a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
140b2f530: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140b2f535: 55                          	push	rbp
140b2f536: 41 57                       	push	r15
140b2f538: 41 56                       	push	r14
140b2f53a: 41 55                       	push	r13
140b2f53c: 41 54                       	push	r12
140b2f53e: 56                          	push	rsi
140b2f53f: 57                          	push	rdi
140b2f540: 53                          	push	rbx
140b2f541: 48 83 ec 58                 	sub	rsp, 0x58
140b2f545: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140b2f54c: 0f 29 7c 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm7
140b2f551: 0f 29 74 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm6
140b2f556: 48 8b 8d e0 00 00 00        	mov	rcx, qword ptr [rbp + 0xe0]
140b2f55d: 66 c7 41 18 01 01           	mov	word ptr [rcx + 0x18], 0x101
140b2f563: 8b 85 d0 00 00 00           	mov	eax, dword ptr [rbp + 0xd0]
140b2f569: 89 41 1a                    	mov	dword ptr [rcx + 0x1a], eax
140b2f56c: 0f b7 85 d4 00 00 00        	movzx	eax, word ptr [rbp + 0xd4]
140b2f573: 66 89 41 1e                 	mov	word ptr [rcx + 0x1e], ax
140b2f577: 48 8b 85 c0 00 00 00        	mov	rax, qword ptr [rbp + 0xc0]
140b2f57e: 48 89 41 20                 	mov	qword ptr [rcx + 0x20], rax
140b2f582: 8b 85 cc 00 00 00           	mov	eax, dword ptr [rbp + 0xcc]
140b2f588: 89 41 28                    	mov	dword ptr [rcx + 0x28], eax
140b2f58b: 48 8b 45 20                 	mov	rax, qword ptr [rbp + 0x20]
140b2f58f: 48 89 41 2c                 	mov	qword ptr [rcx + 0x2c], rax
140b2f593: 8b 45 28                    	mov	eax, dword ptr [rbp + 0x28]
140b2f596: 89 41 34                    	mov	dword ptr [rcx + 0x34], eax
140b2f599: 48 8b 85 b8 00 00 00        	mov	rax, qword ptr [rbp + 0xb8]
140b2f5a0: 48 89 41 38                 	mov	qword ptr [rcx + 0x38], rax
140b2f5a4: 0f 28 45 60                 	movaps	xmm0, xmmword ptr [rbp + 0x60]
140b2f5a8: 66 0f 6f 4d 70              	movdqa	xmm1, xmmword ptr [rbp + 0x70]
140b2f5ad: 0f 11 41 40                 	movups	xmmword ptr [rcx + 0x40], xmm0
140b2f5b1: f3 0f 7f 49 50              	movdqu	xmmword ptr [rcx + 0x50], xmm1
140b2f5b6: 0f 28 74 24 40              	movaps	xmm6, xmmword ptr [rsp + 0x40]
140b2f5bb: 0f 28 7c 24 30              	movaps	xmm7, xmmword ptr [rsp + 0x30]
140b2f5c0: 48 83 c4 58                 	add	rsp, 0x58
140b2f5c4: 5b                          	pop	rbx
140b2f5c5: 5f                          	pop	rdi
140b2f5c6: 5e                          	pop	rsi
140b2f5c7: 41 5c                       	pop	r12
140b2f5c9: 41 5d                       	pop	r13
140b2f5cb: 41 5e                       	pop	r14
140b2f5cd: 41 5f                       	pop	r15
140b2f5cf: 5d                          	pop	rbp
140b2f5d0: c3                          	ret
140b2f5d1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
140b2f5e0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140b2f5e5: 55                          	push	rbp
140b2f5e6: 41 57                       	push	r15
140b2f5e8: 41 56                       	push	r14
140b2f5ea: 41 55                       	push	r13
140b2f5ec: 41 54                       	push	r12
140b2f5ee: 56                          	push	rsi
140b2f5ef: 57                          	push	rdi
140b2f5f0: 53                          	push	rbx
140b2f5f1: 48 83 ec 58                 	sub	rsp, 0x58
140b2f5f5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140b2f5fc: 0f 29 7c 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm7
140b2f601: 0f 29 74 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm6
140b2f606: 48 8b 8d e0 00 00 00        	mov	rcx, qword ptr [rbp + 0xe0]
140b2f60d: 66 c7 41 18 01 01           	mov	word ptr [rcx + 0x18], 0x101
140b2f613: 8b 85 d0 00 00 00           	mov	eax, dword ptr [rbp + 0xd0]
140b2f619: 89 41 1a                    	mov	dword ptr [rcx + 0x1a], eax
140b2f61c: 0f b7 85 d4 00 00 00        	movzx	eax, word ptr [rbp + 0xd4]
140b2f623: 66 89 41 1e                 	mov	word ptr [rcx + 0x1e], ax
140b2f627: 48 8b 85 c0 00 00 00        	mov	rax, qword ptr [rbp + 0xc0]
140b2f62e: 48 89 41 20                 	mov	qword ptr [rcx + 0x20], rax
140b2f632: 8b 85 cc 00 00 00           	mov	eax, dword ptr [rbp + 0xcc]
140b2f638: 89 41 28                    	mov	dword ptr [rcx + 0x28], eax
140b2f63b: 48 8b 45 20                 	mov	rax, qword ptr [rbp + 0x20]
140b2f63f: 48 89 41 2c                 	mov	qword ptr [rcx + 0x2c], rax
140b2f643: 8b 45 28                    	mov	eax, dword ptr [rbp + 0x28]
140b2f646: 89 41 34                    	mov	dword ptr [rcx + 0x34], eax
140b2f649: 48 8b 85 b8 00 00 00        	mov	rax, qword ptr [rbp + 0xb8]
140b2f650: 48 89 41 38                 	mov	qword ptr [rcx + 0x38], rax
140b2f654: 0f 28 45 60                 	movaps	xmm0, xmmword ptr [rbp + 0x60]
140b2f658: 66 0f 6f 4d 70              	movdqa	xmm1, xmmword ptr [rbp + 0x70]
140b2f65d: 0f 11 41 40                 	movups	xmmword ptr [rcx + 0x40], xmm0
140b2f661: f3 0f 7f 49 50              	movdqu	xmmword ptr [rcx + 0x50], xmm1
140b2f666: 0f 28 74 24 40              	movaps	xmm6, xmmword ptr [rsp + 0x40]
140b2f66b: 0f 28 7c 24 30              	movaps	xmm7, xmmword ptr [rsp + 0x30]
140b2f670: 48 83 c4 58                 	add	rsp, 0x58
140b2f674: 5b                          	pop	rbx
140b2f675: 5f                          	pop	rdi
140b2f676: 5e                          	pop	rsi
140b2f677: 41 5c                       	pop	r12
140b2f679: 41 5d                       	pop	r13
140b2f67b: 41 5e                       	pop	r14
140b2f67d: 41 5f                       	pop	r15
140b2f67f: 5d                          	pop	rbp
140b2f680: c3                          	ret
140b2f681: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
140b2f690: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140b2f695: 55                          	push	rbp
140b2f696: 41 57                       	push	r15
140b2f698: 41 56                       	push	r14
140b2f69a: 41 55                       	push	r13
140b2f69c: 41 54                       	push	r12
140b2f69e: 56                          	push	rsi
140b2f69f: 57                          	push	rdi
140b2f6a0: 53                          	push	rbx
140b2f6a1: 48 83 ec 58                 	sub	rsp, 0x58
140b2f6a5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140b2f6ac: 0f 29 7c 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm7
140b2f6b1: 0f 29 74 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm6
140b2f6b6: 48 8b 85 e0 00 00 00        	mov	rax, qword ptr [rbp + 0xe0]
140b2f6bd: 48 ff 00                    	inc	qword ptr [rax]
140b2f6c0: 0f 28 74 24 40              	movaps	xmm6, xmmword ptr [rsp + 0x40]
140b2f6c5: 0f 28 7c 24 30              	movaps	xmm7, xmmword ptr [rsp + 0x30]
140b2f6ca: 48 83 c4 58                 	add	rsp, 0x58
140b2f6ce: 5b                          	pop	rbx
140b2f6cf: 5f                          	pop	rdi
140b2f6d0: 5e                          	pop	rsi
140b2f6d1: 41 5c                       	pop	r12
140b2f6d3: 41 5d                       	pop	r13
140b2f6d5: 41 5e                       	pop	r14
140b2f6d7: 41 5f                       	pop	r15
140b2f6d9: 5d                          	pop	rbp
140b2f6da: c3                          	ret
140b2f6db: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
140b2f6e0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140b2f6e5: 55                          	push	rbp
140b2f6e6: 41 57                       	push	r15
140b2f6e8: 41 56                       	push	r14
140b2f6ea: 41 55                       	push	r13
140b2f6ec: 41 54                       	push	r12
140b2f6ee: 56                          	push	rsi
140b2f6ef: 57                          	push	rdi
140b2f6f0: 53                          	push	rbx
140b2f6f1: 48 83 ec 58                 	sub	rsp, 0x58
140b2f6f5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140b2f6fc: 0f 29 7c 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm7
140b2f701: 0f 29 74 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm6
140b2f706: 48 8b 8d e0 00 00 00        	mov	rcx, qword ptr [rbp + 0xe0]
140b2f70d: 66 c7 41 18 01 01           	mov	word ptr [rcx + 0x18], 0x101
140b2f713: 8b 85 d0 00 00 00           	mov	eax, dword ptr [rbp + 0xd0]
140b2f719: 89 41 1a                    	mov	dword ptr [rcx + 0x1a], eax
140b2f71c: 0f b7 85 d4 00 00 00        	movzx	eax, word ptr [rbp + 0xd4]
140b2f723: 66 89 41 1e                 	mov	word ptr [rcx + 0x1e], ax
140b2f727: 48 8b 85 b0 00 00 00        	mov	rax, qword ptr [rbp + 0xb0]
140b2f72e: 48 89 41 20                 	mov	qword ptr [rcx + 0x20], rax
140b2f732: 8b 85 cc 00 00 00           	mov	eax, dword ptr [rbp + 0xcc]
140b2f738: 89 41 28                    	mov	dword ptr [rcx + 0x28], eax
140b2f73b: 48 8b 45 20                 	mov	rax, qword ptr [rbp + 0x20]
140b2f73f: 48 89 41 2c                 	mov	qword ptr [rcx + 0x2c], rax
140b2f743: 8b 45 28                    	mov	eax, dword ptr [rbp + 0x28]
140b2f746: 89 41 34                    	mov	dword ptr [rcx + 0x34], eax
140b2f749: 48 8b 85 b8 00 00 00        	mov	rax, qword ptr [rbp + 0xb8]
140b2f750: 48 89 41 38                 	mov	qword ptr [rcx + 0x38], rax
140b2f754: 0f 28 45 60                 	movaps	xmm0, xmmword ptr [rbp + 0x60]
140b2f758: 66 0f 6f 4d 70              	movdqa	xmm1, xmmword ptr [rbp + 0x70]
140b2f75d: 0f 11 41 40                 	movups	xmmword ptr [rcx + 0x40], xmm0
140b2f761: f3 0f 7f 49 50              	movdqu	xmmword ptr [rcx + 0x50], xmm1
140b2f766: 0f 28 74 24 40              	movaps	xmm6, xmmword ptr [rsp + 0x40]
140b2f76b: 0f 28 7c 24 30              	movaps	xmm7, xmmword ptr [rsp + 0x30]
140b2f770: 48 83 c4 58                 	add	rsp, 0x58
140b2f774: 5b                          	pop	rbx
140b2f775: 5f                          	pop	rdi
140b2f776: 5e                          	pop	rsi
140b2f777: 41 5c                       	pop	r12
140b2f779: 41 5d                       	pop	r13
140b2f77b: 41 5e                       	pop	r14
140b2f77d: 41 5f                       	pop	r15
140b2f77f: 5d                          	pop	rbp
140b2f780: c3                          	ret
140b2f781: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
140b2f790: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140b2f795: 55                          	push	rbp
140b2f796: 41 57                       	push	r15
140b2f798: 41 56                       	push	r14
140b2f79a: 41 55                       	push	r13
140b2f79c: 41 54                       	push	r12
140b2f79e: 56                          	push	rsi
140b2f79f: 57                          	push	rdi
140b2f7a0: 53                          	push	rbx
140b2f7a1: 48 83 ec 58                 	sub	rsp, 0x58
140b2f7a5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140b2f7ac: 0f 29 7c 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm7
140b2f7b1: 0f 29 74 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm6
140b2f7b6: 48 8b 8d e0 00 00 00        	mov	rcx, qword ptr [rbp + 0xe0]
140b2f7bd: 66 c7 41 18 01 01           	mov	word ptr [rcx + 0x18], 0x101
140b2f7c3: 8b 85 d0 00 00 00           	mov	eax, dword ptr [rbp + 0xd0]
140b2f7c9: 89 41 1a                    	mov	dword ptr [rcx + 0x1a], eax
140b2f7cc: 0f b7 85 d4 00 00 00        	movzx	eax, word ptr [rbp + 0xd4]
140b2f7d3: 66 89 41 1e                 	mov	word ptr [rcx + 0x1e], ax
140b2f7d7: 48 8b 85 b0 00 00 00        	mov	rax, qword ptr [rbp + 0xb0]
140b2f7de: 48 89 41 20                 	mov	qword ptr [rcx + 0x20], rax
140b2f7e2: 8b 85 cc 00 00 00           	mov	eax, dword ptr [rbp + 0xcc]
140b2f7e8: 89 41 28                    	mov	dword ptr [rcx + 0x28], eax
140b2f7eb: 48 8b 45 20                 	mov	rax, qword ptr [rbp + 0x20]
140b2f7ef: 48 89 41 2c                 	mov	qword ptr [rcx + 0x2c], rax
140b2f7f3: 8b 45 28                    	mov	eax, dword ptr [rbp + 0x28]
140b2f7f6: 89 41 34                    	mov	dword ptr [rcx + 0x34], eax
140b2f7f9: 48 8b 85 b8 00 00 00        	mov	rax, qword ptr [rbp + 0xb8]
140b2f800: 48 89 41 38                 	mov	qword ptr [rcx + 0x38], rax
140b2f804: 0f 28 45 60                 	movaps	xmm0, xmmword ptr [rbp + 0x60]
140b2f808: 66 0f 6f 4d 70              	movdqa	xmm1, xmmword ptr [rbp + 0x70]
140b2f80d: 0f 11 41 40                 	movups	xmmword ptr [rcx + 0x40], xmm0
140b2f811: f3 0f 7f 49 50              	movdqu	xmmword ptr [rcx + 0x50], xmm1
140b2f816: 0f 28 74 24 40              	movaps	xmm6, xmmword ptr [rsp + 0x40]
140b2f81b: 0f 28 7c 24 30              	movaps	xmm7, xmmword ptr [rsp + 0x30]
140b2f820: 48 83 c4 58                 	add	rsp, 0x58
140b2f824: 5b                          	pop	rbx
140b2f825: 5f                          	pop	rdi
140b2f826: 5e                          	pop	rsi
140b2f827: 41 5c                       	pop	r12
140b2f829: 41 5d                       	pop	r13
140b2f82b: 41 5e                       	pop	r14
140b2f82d: 41 5f                       	pop	r15
140b2f82f: 5d                          	pop	rbp
140b2f830: c3                          	ret
140b2f831: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
140b2f840: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140b2f845: 55                          	push	rbp
140b2f846: 41 57                       	push	r15
140b2f848: 41 56                       	push	r14
140b2f84a: 41 55                       	push	r13
140b2f84c: 41 54                       	push	r12
140b2f84e: 56                          	push	rsi
140b2f84f: 57                          	push	rdi
140b2f850: 53                          	push	rbx
140b2f851: 48 83 ec 58                 	sub	rsp, 0x58
140b2f855: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140b2f85c: 0f 29 7c 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm7
140b2f861: 0f 29 74 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm6
140b2f866: 48 8b 8d e0 00 00 00        	mov	rcx, qword ptr [rbp + 0xe0]
140b2f86d: 66 c7 41 18 01 01           	mov	word ptr [rcx + 0x18], 0x101
140b2f873: 8b 85 d0 00 00 00           	mov	eax, dword ptr [rbp + 0xd0]
140b2f879: 89 41 1a                    	mov	dword ptr [rcx + 0x1a], eax
140b2f87c: 0f b7 85 d4 00 00 00        	movzx	eax, word ptr [rbp + 0xd4]
140b2f883: 66 89 41 1e                 	mov	word ptr [rcx + 0x1e], ax
140b2f887: 48 8b 85 b0 00 00 00        	mov	rax, qword ptr [rbp + 0xb0]
140b2f88e: 48 89 41 20                 	mov	qword ptr [rcx + 0x20], rax
140b2f892: 8b 85 cc 00 00 00           	mov	eax, dword ptr [rbp + 0xcc]
140b2f898: 89 41 28                    	mov	dword ptr [rcx + 0x28], eax
140b2f89b: 48 8b 45 20                 	mov	rax, qword ptr [rbp + 0x20]
140b2f89f: 48 89 41 2c                 	mov	qword ptr [rcx + 0x2c], rax
140b2f8a3: 8b 45 28                    	mov	eax, dword ptr [rbp + 0x28]
140b2f8a6: 89 41 34                    	mov	dword ptr [rcx + 0x34], eax
140b2f8a9: 48 8b 85 b8 00 00 00        	mov	rax, qword ptr [rbp + 0xb8]
140b2f8b0: 48 89 41 38                 	mov	qword ptr [rcx + 0x38], rax
140b2f8b4: 0f 28 45 60                 	movaps	xmm0, xmmword ptr [rbp + 0x60]
140b2f8b8: 66 0f 6f 4d 70              	movdqa	xmm1, xmmword ptr [rbp + 0x70]
140b2f8bd: 0f 11 41 40                 	movups	xmmword ptr [rcx + 0x40], xmm0
140b2f8c1: f3 0f 7f 49 50              	movdqu	xmmword ptr [rcx + 0x50], xmm1
140b2f8c6: 0f 28 74 24 40              	movaps	xmm6, xmmword ptr [rsp + 0x40]
140b2f8cb: 0f 28 7c 24 30              	movaps	xmm7, xmmword ptr [rsp + 0x30]
140b2f8d0: 48 83 c4 58                 	add	rsp, 0x58
140b2f8d4: 5b                          	pop	rbx
140b2f8d5: 5f                          	pop	rdi
140b2f8d6: 5e                          	pop	rsi
140b2f8d7: 41 5c                       	pop	r12
140b2f8d9: 41 5d                       	pop	r13
140b2f8db: 41 5e                       	pop	r14
140b2f8dd: 41 5f                       	pop	r15
140b2f8df: 5d                          	pop	rbp
140b2f8e0: c3                          	ret
140b2f8e1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
140b2f8f0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140b2f8f5: 55                          	push	rbp
140b2f8f6: 41 57                       	push	r15
140b2f8f8: 41 56                       	push	r14
140b2f8fa: 41 55                       	push	r13
140b2f8fc: 41 54                       	push	r12
140b2f8fe: 56                          	push	rsi
140b2f8ff: 57                          	push	rdi
140b2f900: 53                          	push	rbx
140b2f901: 48 83 ec 58                 	sub	rsp, 0x58
140b2f905: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140b2f90c: 0f 29 7c 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm7
140b2f911: 0f 29 74 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm6
140b2f916: 48 8b 85 e0 00 00 00        	mov	rax, qword ptr [rbp + 0xe0]
140b2f91d: c6 40 18 00                 	mov	byte ptr [rax + 0x18], 0x0
140b2f921: 48 8b 85 9f 00 00 00        	mov	rax, qword ptr [rbp + 0x9f]
140b2f928: 48 8b 8d b0 00 00 00        	mov	rcx, qword ptr [rbp + 0xb0]
140b2f92f: 48 89 41 3f                 	mov	qword ptr [rcx + 0x3f], rax
140b2f933: 0f 10 45 60                 	movups	xmm0, xmmword ptr [rbp + 0x60]
140b2f937: f3 0f 6f 4d 70              	movdqu	xmm1, xmmword ptr [rbp + 0x70]
140b2f93c: f3 0f 6f 95 80 00 00 00     	movdqu	xmm2, xmmword ptr [rbp + 0x80]
140b2f944: f3 0f 6f 9d 90 00 00 00     	movdqu	xmm3, xmmword ptr [rbp + 0x90]
140b2f94c: f3 0f 7f 59 30              	movdqu	xmmword ptr [rcx + 0x30], xmm3
140b2f951: f3 0f 7f 51 20              	movdqu	xmmword ptr [rcx + 0x20], xmm2
140b2f956: f3 0f 7f 49 10              	movdqu	xmmword ptr [rcx + 0x10], xmm1
140b2f95b: 0f 11 01                    	movups	xmmword ptr [rcx], xmm0
140b2f95e: 0f 28 74 24 40              	movaps	xmm6, xmmword ptr [rsp + 0x40]
140b2f963: 0f 28 7c 24 30              	movaps	xmm7, xmmword ptr [rsp + 0x30]
140b2f968: 48 83 c4 58                 	add	rsp, 0x58
140b2f96c: 5b                          	pop	rbx
140b2f96d: 5f                          	pop	rdi
140b2f96e: 5e                          	pop	rsi
140b2f96f: 41 5c                       	pop	r12
140b2f971: 41 5d                       	pop	r13
140b2f973: 41 5e                       	pop	r14
140b2f975: 41 5f                       	pop	r15
140b2f977: 5d                          	pop	rbp
140b2f978: c3                          	ret
140b2f979: 0f 1f 80 00 00 00 00        	nop	dword ptr [rax]
140b2f980: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140b2f985: 55                          	push	rbp
140b2f986: 41 57                       	push	r15
140b2f988: 41 56                       	push	r14
140b2f98a: 41 55                       	push	r13
140b2f98c: 41 54                       	push	r12
140b2f98e: 56                          	push	rsi
140b2f98f: 57                          	push	rdi
140b2f990: 53                          	push	rbx
140b2f991: 48 83 ec 58                 	sub	rsp, 0x58
140b2f995: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140b2f99c: 0f 29 7c 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm7
140b2f9a1: 0f 29 74 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm6
140b2f9a6: 48 8b 8d e0 00 00 00        	mov	rcx, qword ptr [rbp + 0xe0]
140b2f9ad: 66 c7 41 18 02 03           	mov	word ptr [rcx + 0x18], 0x302
140b2f9b3: 8b 45 60                    	mov	eax, dword ptr [rbp + 0x60]
140b2f9b6: 89 41 1a                    	mov	dword ptr [rcx + 0x1a], eax
140b2f9b9: 0f b7 45 64                 	movzx	eax, word ptr [rbp + 0x64]
140b2f9bd: 66 89 41 1e                 	mov	word ptr [rcx + 0x1e], ax
140b2f9c1: 48 8b 85 c0 00 00 00        	mov	rax, qword ptr [rbp + 0xc0]
140b2f9c8: 48 89 41 20                 	mov	qword ptr [rcx + 0x20], rax
140b2f9cc: 48 8b 45 50                 	mov	rax, qword ptr [rbp + 0x50]
140b2f9d0: 48 8b 8d a8 00 00 00        	mov	rcx, qword ptr [rbp + 0xa8]
140b2f9d7: 48 89 41 30                 	mov	qword ptr [rcx + 0x30], rax
140b2f9db: 0f 28 45 20                 	movaps	xmm0, xmmword ptr [rbp + 0x20]
140b2f9df: 66 0f 6f 4d 30              	movdqa	xmm1, xmmword ptr [rbp + 0x30]
140b2f9e4: 66 0f 6f 55 40              	movdqa	xmm2, xmmword ptr [rbp + 0x40]
140b2f9e9: f3 0f 7f 51 20              	movdqu	xmmword ptr [rcx + 0x20], xmm2
140b2f9ee: f3 0f 7f 49 10              	movdqu	xmmword ptr [rcx + 0x10], xmm1
140b2f9f3: 0f 11 01                    	movups	xmmword ptr [rcx], xmm0
140b2f9f6: 0f 28 74 24 40              	movaps	xmm6, xmmword ptr [rsp + 0x40]
140b2f9fb: 0f 28 7c 24 30              	movaps	xmm7, xmmword ptr [rsp + 0x30]
140b2fa00: 48 83 c4 58                 	add	rsp, 0x58
140b2fa04: 5b                          	pop	rbx
140b2fa05: 5f                          	pop	rdi
140b2fa06: 5e                          	pop	rsi
140b2fa07: 41 5c                       	pop	r12
140b2fa09: 41 5d                       	pop	r13
140b2fa0b: 41 5e                       	pop	r14
140b2fa0d: 41 5f                       	pop	r15
140b2fa0f: 5d                          	pop	rbp
140b2fa10: c3                          	ret
140b2fa11: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
140b2fa20: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140b2fa25: 55                          	push	rbp
140b2fa26: 41 57                       	push	r15
140b2fa28: 41 56                       	push	r14
140b2fa2a: 41 55                       	push	r13
140b2fa2c: 41 54                       	push	r12
140b2fa2e: 56                          	push	rsi
140b2fa2f: 57                          	push	rdi
140b2fa30: 53                          	push	rbx
140b2fa31: 48 83 ec 58                 	sub	rsp, 0x58
140b2fa35: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140b2fa3c: 0f 29 7c 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm7
140b2fa41: 0f 29 74 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm6
140b2fa46: 48 8b 8d e0 00 00 00        	mov	rcx, qword ptr [rbp + 0xe0]
140b2fa4d: 66 c7 41 18 02 04           	mov	word ptr [rcx + 0x18], 0x402
140b2fa53: 8b 85 d0 00 00 00           	mov	eax, dword ptr [rbp + 0xd0]
140b2fa59: 89 41 1a                    	mov	dword ptr [rcx + 0x1a], eax
140b2fa5c: 0f b7 85 d4 00 00 00        	movzx	eax, word ptr [rbp + 0xd4]
140b2fa63: 66 89 41 1e                 	mov	word ptr [rcx + 0x1e], ax
140b2fa67: 48 8b 85 a8 00 00 00        	mov	rax, qword ptr [rbp + 0xa8]
140b2fa6e: 48 89 41 20                 	mov	qword ptr [rcx + 0x20], rax
140b2fa72: 48 8b 85 90 00 00 00        	mov	rax, qword ptr [rbp + 0x90]
140b2fa79: 48 8b 8d c0 00 00 00        	mov	rcx, qword ptr [rbp + 0xc0]
140b2fa80: 48 89 41 30                 	mov	qword ptr [rcx + 0x30], rax
140b2fa84: 66 0f 6f 45 60              	movdqa	xmm0, xmmword ptr [rbp + 0x60]
140b2fa89: 66 0f 6f 4d 70              	movdqa	xmm1, xmmword ptr [rbp + 0x70]
140b2fa8e: 66 0f 6f 95 80 00 00 00     	movdqa	xmm2, xmmword ptr [rbp + 0x80]
140b2fa96: f3 0f 7f 51 20              	movdqu	xmmword ptr [rcx + 0x20], xmm2
140b2fa9b: f3 0f 7f 49 10              	movdqu	xmmword ptr [rcx + 0x10], xmm1
140b2faa0: f3 0f 7f 01                 	movdqu	xmmword ptr [rcx], xmm0
140b2faa4: c6 85 df 00 00 00 00        	mov	byte ptr [rbp + 0xdf], 0x0
140b2faab: 0f 28 74 24 40              	movaps	xmm6, xmmword ptr [rsp + 0x40]
140b2fab0: 0f 28 7c 24 30              	movaps	xmm7, xmmword ptr [rsp + 0x30]
140b2fab5: 48 83 c4 58                 	add	rsp, 0x58
140b2fab9: 5b                          	pop	rbx
140b2faba: 5f                          	pop	rdi
140b2fabb: 5e                          	pop	rsi
140b2fabc: 41 5c                       	pop	r12
140b2fabe: 41 5d                       	pop	r13
140b2fac0: 41 5e                       	pop	r14
140b2fac2: 41 5f                       	pop	r15
140b2fac4: 5d                          	pop	rbp
140b2fac5: c3                          	ret
140b2fac6: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
140b2fad0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140b2fad5: 55                          	push	rbp
140b2fad6: 41 57                       	push	r15
140b2fad8: 41 56                       	push	r14
140b2fada: 41 55                       	push	r13
140b2fadc: 41 54                       	push	r12
140b2fade: 56                          	push	rsi
140b2fadf: 57                          	push	rdi
140b2fae0: 53                          	push	rbx
140b2fae1: 48 83 ec 58                 	sub	rsp, 0x58
140b2fae5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140b2faec: 0f 29 7c 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm7
140b2faf1: 0f 29 74 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm6
140b2faf6: 48 8b 8d e0 00 00 00        	mov	rcx, qword ptr [rbp + 0xe0]
140b2fafd: c6 41 18 02                 	mov	byte ptr [rcx + 0x18], 0x2
140b2fb01: 0f b6 45 08                 	movzx	eax, byte ptr [rbp + 0x8]
140b2fb05: 88 41 19                    	mov	byte ptr [rcx + 0x19], al
140b2fb08: 8b 45 60                    	mov	eax, dword ptr [rbp + 0x60]
140b2fb0b: 89 41 1a                    	mov	dword ptr [rcx + 0x1a], eax
140b2fb0e: 0f b7 45 64                 	movzx	eax, word ptr [rbp + 0x64]
140b2fb12: 66 89 41 1e                 	mov	word ptr [rcx + 0x1e], ax
140b2fb16: 48 8b 85 c0 00 00 00        	mov	rax, qword ptr [rbp + 0xc0]
140b2fb1d: 48 89 41 20                 	mov	qword ptr [rcx + 0x20], rax
140b2fb21: 48 8b 45 50                 	mov	rax, qword ptr [rbp + 0x50]
140b2fb25: 48 8b 8d a8 00 00 00        	mov	rcx, qword ptr [rbp + 0xa8]
140b2fb2c: 48 89 41 30                 	mov	qword ptr [rcx + 0x30], rax
140b2fb30: 0f 28 45 20                 	movaps	xmm0, xmmword ptr [rbp + 0x20]
140b2fb34: 66 0f 6f 4d 30              	movdqa	xmm1, xmmword ptr [rbp + 0x30]
140b2fb39: 66 0f 6f 55 40              	movdqa	xmm2, xmmword ptr [rbp + 0x40]
140b2fb3e: f3 0f 7f 51 20              	movdqu	xmmword ptr [rcx + 0x20], xmm2
140b2fb43: f3 0f 7f 49 10              	movdqu	xmmword ptr [rcx + 0x10], xmm1
140b2fb48: 0f 11 01                    	movups	xmmword ptr [rcx], xmm0
140b2fb4b: 0f 28 74 24 40              	movaps	xmm6, xmmword ptr [rsp + 0x40]
140b2fb50: 0f 28 7c 24 30              	movaps	xmm7, xmmword ptr [rsp + 0x30]
140b2fb55: 48 83 c4 58                 	add	rsp, 0x58
140b2fb59: 5b                          	pop	rbx
140b2fb5a: 5f                          	pop	rdi
140b2fb5b: 5e                          	pop	rsi
140b2fb5c: 41 5c                       	pop	r12
140b2fb5e: 41 5d                       	pop	r13
140b2fb60: 41 5e                       	pop	r14
140b2fb62: 41 5f                       	pop	r15
140b2fb64: 5d                          	pop	rbp
140b2fb65: c3                          	ret
140b2fb66: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
140b2fb70: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140b2fb75: 55                          	push	rbp
140b2fb76: 41 57                       	push	r15
140b2fb78: 41 56                       	push	r14
140b2fb7a: 41 55                       	push	r13
140b2fb7c: 41 54                       	push	r12
140b2fb7e: 56                          	push	rsi
140b2fb7f: 57                          	push	rdi
140b2fb80: 53                          	push	rbx
140b2fb81: 48 83 ec 58                 	sub	rsp, 0x58
140b2fb85: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140b2fb8c: 0f 29 7c 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm7
140b2fb91: 0f 29 74 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm6
140b2fb96: 48 8b 85 e0 00 00 00        	mov	rax, qword ptr [rbp + 0xe0]
140b2fb9d: c6 40 18 00                 	mov	byte ptr [rax + 0x18], 0x0
140b2fba1: 48 8b 85 9f 00 00 00        	mov	rax, qword ptr [rbp + 0x9f]
140b2fba8: 48 8b 8d c0 00 00 00        	mov	rcx, qword ptr [rbp + 0xc0]
140b2fbaf: 48 89 41 3f                 	mov	qword ptr [rcx + 0x3f], rax
140b2fbb3: 0f 10 45 60                 	movups	xmm0, xmmword ptr [rbp + 0x60]
140b2fbb7: f3 0f 6f 4d 70              	movdqu	xmm1, xmmword ptr [rbp + 0x70]
140b2fbbc: f3 0f 6f 95 80 00 00 00     	movdqu	xmm2, xmmword ptr [rbp + 0x80]
140b2fbc4: f3 0f 6f 9d 90 00 00 00     	movdqu	xmm3, xmmword ptr [rbp + 0x90]
140b2fbcc: f3 0f 7f 59 30              	movdqu	xmmword ptr [rcx + 0x30], xmm3
140b2fbd1: f3 0f 7f 51 20              	movdqu	xmmword ptr [rcx + 0x20], xmm2
140b2fbd6: f3 0f 7f 49 10              	movdqu	xmmword ptr [rcx + 0x10], xmm1
140b2fbdb: 0f 11 01                    	movups	xmmword ptr [rcx], xmm0
140b2fbde: 0f 28 74 24 40              	movaps	xmm6, xmmword ptr [rsp + 0x40]
140b2fbe3: 0f 28 7c 24 30              	movaps	xmm7, xmmword ptr [rsp + 0x30]
140b2fbe8: 48 83 c4 58                 	add	rsp, 0x58
140b2fbec: 5b                          	pop	rbx
140b2fbed: 5f                          	pop	rdi
140b2fbee: 5e                          	pop	rsi
140b2fbef: 41 5c                       	pop	r12
140b2fbf1: 41 5d                       	pop	r13
140b2fbf3: 41 5e                       	pop	r14
140b2fbf5: 41 5f                       	pop	r15
140b2fbf7: 5d                          	pop	rbp
140b2fbf8: c3                          	ret
