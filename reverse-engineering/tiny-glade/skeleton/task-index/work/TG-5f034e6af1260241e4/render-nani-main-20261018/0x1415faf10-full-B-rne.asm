
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1415faf10: 41 57                       	push	r15
1415faf12: 41 56                       	push	r14
1415faf14: 41 55                       	push	r13
1415faf16: 41 54                       	push	r12
1415faf18: 56                          	push	rsi
1415faf19: 57                          	push	rdi
1415faf1a: 55                          	push	rbp
1415faf1b: 53                          	push	rbx
1415faf1c: 48 83 ec 58                 	sub	rsp, 0x58
1415faf20: 4d 89 c7                    	mov	r15, r8
1415faf23: 48 89 d7                    	mov	rdi, rdx
1415faf26: 0f b6 71 08                 	movzx	esi, byte ptr [rcx + 0x8]
1415faf2a: 0f b6 59 09                 	movzx	ebx, byte ptr [rcx + 0x9]
1415faf2e: 45 8b 40 3c                 	mov	r8d, dword ptr [r8 + 0x3c]
1415faf32: 41 8b 47 40                 	mov	eax, dword ptr [r15 + 0x40]
1415faf36: 31 d2                       	xor	edx, edx
1415faf38: 31 ed                       	xor	ebp, ebp
1415faf3a: 89 44 24 40                 	mov	dword ptr [rsp + 0x40], eax
1415faf3e: 83 f8 01                    	cmp	eax, 0x1
1415faf41: 41 0f 94 c3                 	sete	r11b
1415faf45: 41 0f 95 c1                 	setne	r9b
1415faf49: 31 c0                       	xor	eax, eax
1415faf4b: 44 89 44 24 44              	mov	dword ptr [rsp + 0x44], r8d
1415faf50: 41 83 f8 01                 	cmp	r8d, 0x1
1415faf54: 41 0f 94 c2                 	sete	r10b
1415faf58: 41 0f 95 c0                 	setne	r8b
1415faf5c: 48 89 74 24 30              	mov	qword ptr [rsp + 0x30], rsi
1415faf61: 48 89 5c 24 50              	mov	qword ptr [rsp + 0x50], rbx
1415faf66: 40 38 de                    	cmp	sil, bl
1415faf69: 0f 83 38 03 00 00           	jae	0x1415fb2a7 <.text+0x15fa2a7>
1415faf6f: 31 f6                       	xor	esi, esi
1415faf71: 4c 8b 29                    	mov	r13, qword ptr [rcx]
1415faf74: 41 8b 4f 38                 	mov	ecx, dword ptr [r15 + 0x38]
1415faf78: 89 4c 24 2c                 	mov	dword ptr [rsp + 0x2c], ecx
1415faf7c: 44 88 da                    	mov	dl, r11b
1415faf7f: 44 88 cd                    	mov	bpl, r9b
1415faf82: 01 d5                       	add	ebp, edx
1415faf84: 44 88 d0                    	mov	al, r10b
1415faf87: 44 88 c6                    	mov	sil, r8b
1415faf8a: 01 c6                       	add	esi, eax
1415faf8c: 89 74 24 4c                 	mov	dword ptr [rsp + 0x4c], esi
1415faf90: 31 c9                       	xor	ecx, ecx
1415faf92: 48 8d 15 1f 38 68 01        	lea	rdx, [rip + 0x168381f]  # 0x142c7e7b8
1415faf99: eb 10                       	jmp	0x1415fafab <.text+0x15f9fab>
1415faf9b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
1415fafa0: 48 8d 15 11 38 68 01        	lea	rdx, [rip + 0x1683811]  # 0x142c7e7b8
1415fafa7: 8b 4c 24 48                 	mov	ecx, dword ptr [rsp + 0x48]
1415fafab: 89 c8                       	mov	eax, ecx
1415fafad: 0f b6 c9                    	movzx	ecx, cl
1415fafb0: 48 89 4c 24 38              	mov	qword ptr [rsp + 0x38], rcx
1415fafb5: 48 63 0c 8a                 	movsxd	rcx, dword ptr [rdx + 4*rcx]
1415fafb9: 48 01 d1                    	add	rcx, rdx
1415fafbc: ff e1                       	jmp	rcx
1415fafbe: b1 01                       	mov	cl, 0x1
1415fafc0: 80 3d d5 9d 03 02 00        	cmp	byte ptr [rip + 0x2039dd5], 0x0 # 0x143634d9c
1415fafc7: 75 e2                       	jne	0x1415fafab <.text+0x15f9fab>
1415fafc9: e9 e2 01 00 00              	jmp	0x1415fb1b0 <.text+0x15fa1b0>
1415fafce: b1 11                       	mov	cl, 0x11
1415fafd0: 80 3d c5 9d 03 02 00        	cmp	byte ptr [rip + 0x2039dc5], 0x0 # 0x143634d9c
1415fafd7: 75 d2                       	jne	0x1415fafab <.text+0x15f9fab>
1415fafd9: e9 d2 01 00 00              	jmp	0x1415fb1b0 <.text+0x15fa1b0>
1415fafde: b1 0f                       	mov	cl, 0xf
1415fafe0: 80 3d b5 9d 03 02 00        	cmp	byte ptr [rip + 0x2039db5], 0x0 # 0x143634d9c
1415fafe7: 75 c2                       	jne	0x1415fafab <.text+0x15f9fab>
1415fafe9: e9 c2 01 00 00              	jmp	0x1415fb1b0 <.text+0x15fa1b0>
1415fafee: b1 0c                       	mov	cl, 0xc
1415faff0: 80 3d a5 9d 03 02 00        	cmp	byte ptr [rip + 0x2039da5], 0x0 # 0x143634d9c
1415faff7: 75 b2                       	jne	0x1415fafab <.text+0x15f9fab>
1415faff9: e9 b2 01 00 00              	jmp	0x1415fb1b0 <.text+0x15fa1b0>
1415faffe: b1 05                       	mov	cl, 0x5
1415fb000: 80 3d 95 9d 03 02 00        	cmp	byte ptr [rip + 0x2039d95], 0x0 # 0x143634d9c
1415fb007: 75 a2                       	jne	0x1415fafab <.text+0x15f9fab>
1415fb009: e9 a2 01 00 00              	jmp	0x1415fb1b0 <.text+0x15fa1b0>
1415fb00e: b1 0d                       	mov	cl, 0xd
1415fb010: 80 3d 85 9d 03 02 00        	cmp	byte ptr [rip + 0x2039d85], 0x0 # 0x143634d9c
1415fb017: 75 92                       	jne	0x1415fafab <.text+0x15f9fab>
1415fb019: e9 92 01 00 00              	jmp	0x1415fb1b0 <.text+0x15fa1b0>
1415fb01e: b1 0a                       	mov	cl, 0xa
1415fb020: 80 3d 75 9d 03 02 00        	cmp	byte ptr [rip + 0x2039d75], 0x0 # 0x143634d9c
1415fb027: 75 82                       	jne	0x1415fafab <.text+0x15f9fab>
1415fb029: e9 82 01 00 00              	jmp	0x1415fb1b0 <.text+0x15fa1b0>
1415fb02e: b1 1a                       	mov	cl, 0x1a
1415fb030: 80 3d 65 9d 03 02 00        	cmp	byte ptr [rip + 0x2039d65], 0x0 # 0x143634d9c
1415fb037: 0f 85 6e ff ff ff           	jne	0x1415fafab <.text+0x15f9fab>
1415fb03d: e9 6e 01 00 00              	jmp	0x1415fb1b0 <.text+0x15fa1b0>
1415fb042: b1 19                       	mov	cl, 0x19
1415fb044: 80 3d 51 9d 03 02 00        	cmp	byte ptr [rip + 0x2039d51], 0x0 # 0x143634d9c
1415fb04b: 0f 85 5a ff ff ff           	jne	0x1415fafab <.text+0x15f9fab>
1415fb051: e9 5a 01 00 00              	jmp	0x1415fb1b0 <.text+0x15fa1b0>
1415fb056: b1 03                       	mov	cl, 0x3
1415fb058: 80 3d 3d 9d 03 02 00        	cmp	byte ptr [rip + 0x2039d3d], 0x0 # 0x143634d9c
1415fb05f: 0f 85 46 ff ff ff           	jne	0x1415fafab <.text+0x15f9fab>
1415fb065: e9 46 01 00 00              	jmp	0x1415fb1b0 <.text+0x15fa1b0>
1415fb06a: b1 10                       	mov	cl, 0x10
1415fb06c: 80 3d 29 9d 03 02 00        	cmp	byte ptr [rip + 0x2039d29], 0x0 # 0x143634d9c
1415fb073: 0f 85 32 ff ff ff           	jne	0x1415fafab <.text+0x15f9fab>
1415fb079: e9 32 01 00 00              	jmp	0x1415fb1b0 <.text+0x15fa1b0>
1415fb07e: b1 04                       	mov	cl, 0x4
1415fb080: 80 3d 15 9d 03 02 00        	cmp	byte ptr [rip + 0x2039d15], 0x0 # 0x143634d9c
1415fb087: 0f 85 1e ff ff ff           	jne	0x1415fafab <.text+0x15f9fab>
1415fb08d: e9 1e 01 00 00              	jmp	0x1415fb1b0 <.text+0x15fa1b0>
1415fb092: b1 08                       	mov	cl, 0x8
1415fb094: 80 3d 01 9d 03 02 00        	cmp	byte ptr [rip + 0x2039d01], 0x0 # 0x143634d9c
1415fb09b: 0f 85 0a ff ff ff           	jne	0x1415fafab <.text+0x15f9fab>
1415fb0a1: e9 0a 01 00 00              	jmp	0x1415fb1b0 <.text+0x15fa1b0>
1415fb0a6: b1 02                       	mov	cl, 0x2
1415fb0a8: 80 3d ed 9c 03 02 00        	cmp	byte ptr [rip + 0x2039ced], 0x0 # 0x143634d9c
1415fb0af: 0f 85 f6 fe ff ff           	jne	0x1415fafab <.text+0x15f9fab>
1415fb0b5: e9 f6 00 00 00              	jmp	0x1415fb1b0 <.text+0x15fa1b0>
1415fb0ba: b1 12                       	mov	cl, 0x12
1415fb0bc: 80 3d d9 9c 03 02 00        	cmp	byte ptr [rip + 0x2039cd9], 0x0 # 0x143634d9c
1415fb0c3: 0f 85 e2 fe ff ff           	jne	0x1415fafab <.text+0x15f9fab>
1415fb0c9: e9 e2 00 00 00              	jmp	0x1415fb1b0 <.text+0x15fa1b0>
1415fb0ce: b1 15                       	mov	cl, 0x15
1415fb0d0: 80 3d c5 9c 03 02 00        	cmp	byte ptr [rip + 0x2039cc5], 0x0 # 0x143634d9c
1415fb0d7: 0f 85 ce fe ff ff           	jne	0x1415fafab <.text+0x15f9fab>
1415fb0dd: e9 ce 00 00 00              	jmp	0x1415fb1b0 <.text+0x15fa1b0>
1415fb0e2: b1 06                       	mov	cl, 0x6
1415fb0e4: 80 3d b1 9c 03 02 00        	cmp	byte ptr [rip + 0x2039cb1], 0x0 # 0x143634d9c
1415fb0eb: 0f 85 ba fe ff ff           	jne	0x1415fafab <.text+0x15f9fab>
1415fb0f1: e9 ba 00 00 00              	jmp	0x1415fb1b0 <.text+0x15fa1b0>
1415fb0f6: b1 16                       	mov	cl, 0x16
1415fb0f8: 80 3d 9d 9c 03 02 00        	cmp	byte ptr [rip + 0x2039c9d], 0x0 # 0x143634d9c
1415fb0ff: 0f 85 a6 fe ff ff           	jne	0x1415fafab <.text+0x15f9fab>
1415fb105: e9 a6 00 00 00              	jmp	0x1415fb1b0 <.text+0x15fa1b0>
1415fb10a: b1 0e                       	mov	cl, 0xe
1415fb10c: 80 3d 89 9c 03 02 00        	cmp	byte ptr [rip + 0x2039c89], 0x0 # 0x143634d9c
1415fb113: 0f 85 92 fe ff ff           	jne	0x1415fafab <.text+0x15f9fab>
1415fb119: e9 92 00 00 00              	jmp	0x1415fb1b0 <.text+0x15fa1b0>
1415fb11e: b1 07                       	mov	cl, 0x7
1415fb120: 80 3d 75 9c 03 02 00        	cmp	byte ptr [rip + 0x2039c75], 0x0 # 0x143634d9c
1415fb127: 0f 85 7e fe ff ff           	jne	0x1415fafab <.text+0x15f9fab>
1415fb12d: e9 7e 00 00 00              	jmp	0x1415fb1b0 <.text+0x15fa1b0>
1415fb132: b1 13                       	mov	cl, 0x13
1415fb134: 80 3d 61 9c 03 02 00        	cmp	byte ptr [rip + 0x2039c61], 0x0 # 0x143634d9c
1415fb13b: 0f 85 6a fe ff ff           	jne	0x1415fafab <.text+0x15f9fab>
1415fb141: eb 6d                       	jmp	0x1415fb1b0 <.text+0x15fa1b0>
1415fb143: b1 0b                       	mov	cl, 0xb
1415fb145: 80 3d 50 9c 03 02 00        	cmp	byte ptr [rip + 0x2039c50], 0x0 # 0x143634d9c
1415fb14c: 0f 85 59 fe ff ff           	jne	0x1415fafab <.text+0x15f9fab>
1415fb152: eb 5c                       	jmp	0x1415fb1b0 <.text+0x15fa1b0>
1415fb154: b1 09                       	mov	cl, 0x9
1415fb156: 80 3d 3f 9c 03 02 00        	cmp	byte ptr [rip + 0x2039c3f], 0x0 # 0x143634d9c
1415fb15d: 0f 85 48 fe ff ff           	jne	0x1415fafab <.text+0x15f9fab>
1415fb163: eb 4b                       	jmp	0x1415fb1b0 <.text+0x15fa1b0>
1415fb165: b1 17                       	mov	cl, 0x17
1415fb167: 80 3d 2e 9c 03 02 00        	cmp	byte ptr [rip + 0x2039c2e], 0x0 # 0x143634d9c
1415fb16e: 0f 85 37 fe ff ff           	jne	0x1415fafab <.text+0x15f9fab>
1415fb174: eb 3a                       	jmp	0x1415fb1b0 <.text+0x15fa1b0>
1415fb176: b1 14                       	mov	cl, 0x14
1415fb178: 80 3d 1d 9c 03 02 00        	cmp	byte ptr [rip + 0x2039c1d], 0x0 # 0x143634d9c
1415fb17f: 0f 85 26 fe ff ff           	jne	0x1415fafab <.text+0x15f9fab>
1415fb185: eb 29                       	jmp	0x1415fb1b0 <.text+0x15fa1b0>
1415fb187: b1 18                       	mov	cl, 0x18
1415fb189: 80 3d 0c 9c 03 02 00        	cmp	byte ptr [rip + 0x2039c0c], 0x0 # 0x143634d9c
1415fb190: 0f 85 15 fe ff ff           	jne	0x1415fafab <.text+0x15f9fab>
1415fb196: eb 18                       	jmp	0x1415fb1b0 <.text+0x15fa1b0>
1415fb198: b1 1b                       	mov	cl, 0x1b
1415fb19a: 80 3d fb 9b 03 02 00        	cmp	byte ptr [rip + 0x2039bfb], 0x0 # 0x143634d9c
1415fb1a1: 0f 85 04 fe ff ff           	jne	0x1415fafab <.text+0x15f9fab>
1415fb1a7: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1415fb1b0: 3c 01                       	cmp	al, 0x1
1415fb1b2: 77 0d                       	ja	0x1415fb1c1 <.text+0x15fa1c1>
1415fb1b4: 80 3d e2 9b 03 02 00        	cmp	byte ptr [rip + 0x2039be2], 0x0 # 0x143634d9d
1415fb1bb: 0f 84 ea fd ff ff           	je	0x1415fafab <.text+0x15f9fab>
1415fb1c1: c0 e0 02                    	shl	al, 0x2
1415fb1c4: 44 0f b6 f0                 	movzx	r14d, al
1415fb1c8: 83 7c 24 2c 00              	cmp	dword ptr [rsp + 0x2c], 0x0
1415fb1cd: 0f 84 90 02 00 00           	je	0x1415fb463 <.text+0x15fa463>
1415fb1d3: 83 7c 24 44 00              	cmp	dword ptr [rsp + 0x44], 0x0
1415fb1d8: 0f 84 ce 02 00 00           	je	0x1415fb4ac <.text+0x15fa4ac>
1415fb1de: 89 4c 24 48                 	mov	dword ptr [rsp + 0x48], ecx
1415fb1e2: 83 7c 24 40 00              	cmp	dword ptr [rsp + 0x40], 0x0
1415fb1e7: 0f 84 04 03 00 00           	je	0x1415fb4f1 <.text+0x15fa4f1>
1415fb1ed: 48 8b 5c 24 30              	mov	rbx, qword ptr [rsp + 0x30]
1415fb1f2: 49 01 de                    	add	r14, rbx
1415fb1f5: eb 4d                       	jmp	0x1415fb244 <.text+0x15fa244>
1415fb1f7: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1415fb200: b8 40 00 00 00              	mov	eax, 0x40
1415fb205: 31 d2                       	xor	edx, edx
1415fb207: f7 74 24 2c                 	div	dword ptr [rsp + 0x2c]
1415fb20b: 48 ff c3                    	inc	rbx
1415fb20e: 83 fa 01                    	cmp	edx, 0x1
1415fb211: 83 d8 ff                    	sbb	eax, -0x1
1415fb214: f0                          	lock
1415fb215: 49 ff 87 a0 0a 00 00        	inc	qword ptr [r15 + 0xaa0]
1415fb21c: 48 8b 4d 48                 	mov	rcx, qword ptr [rbp + 0x48]
1415fb220: 89 c2                       	mov	edx, eax
1415fb222: 44 8b 44 24 4c              	mov	r8d, dword ptr [rsp + 0x4c]
1415fb227: 89 f5                       	mov	ebp, esi
1415fb229: 41 89 f1                    	mov	r9d, esi
1415fb22c: 41 ff 97 60 03 00 00        	call	qword ptr [r15 + 0x360]
1415fb233: 49 ff c6                    	inc	r14
1415fb236: 48 3b 5c 24 50              	cmp	rbx, qword ptr [rsp + 0x50]
1415fb23b: 4d 89 e7                    	mov	r15, r12
1415fb23e: 0f 83 5c fd ff ff           	jae	0x1415fafa0 <.text+0x15f9fa0>
1415fb244: 49 8b 55 28                 	mov	rdx, qword ptr [r13 + 0x28]
1415fb248: 4c 39 f2                    	cmp	rdx, r14
1415fb24b: 0f 86 e5 02 00 00           	jbe	0x1415fb536 <.text+0x15fa536>
1415fb251: 89 ee                       	mov	esi, ebp
1415fb253: 49 8b 45 20                 	mov	rax, qword ptr [r13 + 0x20]
1415fb257: 42 8b 04 b0                 	mov	eax, dword ptr [rax + 4*r14]
1415fb25b: 48 8b 4c 24 38              	mov	rcx, qword ptr [rsp + 0x38]
1415fb260: 89 4c 24 24                 	mov	dword ptr [rsp + 0x24], ecx
1415fb264: 89 44 24 28                 	mov	dword ptr [rsp + 0x28], eax
1415fb268: 4c 8d 44 24 24              	lea	r8, [rsp + 0x24]
1415fb26d: 41 b9 08 00 00 00           	mov	r9d, 0x8
1415fb273: 48 89 f9                    	mov	rcx, rdi
1415fb276: 4d 89 fc                    	mov	r12, r15
1415fb279: 4c 89 fa                    	mov	rdx, r15
1415fb27c: e8 cf c7 0f 00              	call	0x1416f7a50 <.text+0x16f6a50>
1415fb281: 48 8b 2f                    	mov	rbp, qword ptr [rdi]
1415fb284: 4c 8b 7f 08                 	mov	r15, qword ptr [rdi + 0x8]
1415fb288: 41 80 bf ba 0a 00 00 01     	cmp	byte ptr [r15 + 0xaba], 0x1
1415fb290: 0f 85 6a ff ff ff           	jne	0x1415fb200 <.text+0x15fa200>
1415fb296: 48 8b 55 48                 	mov	rdx, qword ptr [rbp + 0x48]
1415fb29a: 4c 89 f9                    	mov	rcx, r15
1415fb29d: e8 ee 2d 5f ff              	call	0x140bee090 <.text+0xbed090>
1415fb2a2: e9 59 ff ff ff              	jmp	0x1415fb200 <.text+0x15fa200>
1415fb2a7: f6 05 ee 9a 03 02 01        	test	byte ptr [rip + 0x2039aee], 0x1 # 0x143634d9c
1415fb2ae: 0f 84 8e 00 00 00           	je	0x1415fb342 <.text+0x15fa342>
1415fb2b4: 31 c9                       	xor	ecx, ecx
1415fb2b6: 48 8d 05 8b 34 68 01        	lea	rax, [rip + 0x168348b]  # 0x142c7e748
1415fb2bd: eb 11                       	jmp	0x1415fb2d0 <.text+0x15fa2d0>
1415fb2bf: b1 18                       	mov	cl, 0x18
1415fb2c1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
1415fb2d0: 0f b6 c9                    	movzx	ecx, cl
1415fb2d3: 48 63 14 88                 	movsxd	rdx, dword ptr [rax + 4*rcx]
1415fb2d7: 48 01 c2                    	add	rdx, rax
1415fb2da: b1 02                       	mov	cl, 0x2
1415fb2dc: ff e2                       	jmp	rdx
1415fb2de: b1 01                       	mov	cl, 0x1
1415fb2e0: eb ee                       	jmp	0x1415fb2d0 <.text+0x15fa2d0>
1415fb2e2: b1 14                       	mov	cl, 0x14
1415fb2e4: eb ea                       	jmp	0x1415fb2d0 <.text+0x15fa2d0>
1415fb2e6: b1 0f                       	mov	cl, 0xf
1415fb2e8: eb e6                       	jmp	0x1415fb2d0 <.text+0x15fa2d0>
1415fb2ea: b1 1a                       	mov	cl, 0x1a
1415fb2ec: eb e2                       	jmp	0x1415fb2d0 <.text+0x15fa2d0>
1415fb2ee: b1 16                       	mov	cl, 0x16
1415fb2f0: eb de                       	jmp	0x1415fb2d0 <.text+0x15fa2d0>
1415fb2f2: b1 09                       	mov	cl, 0x9
1415fb2f4: eb da                       	jmp	0x1415fb2d0 <.text+0x15fa2d0>
1415fb2f6: b1 07                       	mov	cl, 0x7
1415fb2f8: eb d6                       	jmp	0x1415fb2d0 <.text+0x15fa2d0>
1415fb2fa: b1 03                       	mov	cl, 0x3
1415fb2fc: eb d2                       	jmp	0x1415fb2d0 <.text+0x15fa2d0>
1415fb2fe: b1 08                       	mov	cl, 0x8
1415fb300: eb ce                       	jmp	0x1415fb2d0 <.text+0x15fa2d0>
1415fb302: b1 19                       	mov	cl, 0x19
1415fb304: eb ca                       	jmp	0x1415fb2d0 <.text+0x15fa2d0>
1415fb306: b1 0a                       	mov	cl, 0xa
1415fb308: eb c6                       	jmp	0x1415fb2d0 <.text+0x15fa2d0>
1415fb30a: b1 13                       	mov	cl, 0x13
1415fb30c: eb c2                       	jmp	0x1415fb2d0 <.text+0x15fa2d0>
1415fb30e: b1 06                       	mov	cl, 0x6
1415fb310: eb be                       	jmp	0x1415fb2d0 <.text+0x15fa2d0>
1415fb312: b1 05                       	mov	cl, 0x5
1415fb314: eb ba                       	jmp	0x1415fb2d0 <.text+0x15fa2d0>
1415fb316: b1 04                       	mov	cl, 0x4
1415fb318: eb b6                       	jmp	0x1415fb2d0 <.text+0x15fa2d0>
1415fb31a: b1 1b                       	mov	cl, 0x1b
1415fb31c: eb b2                       	jmp	0x1415fb2d0 <.text+0x15fa2d0>
1415fb31e: b1 0e                       	mov	cl, 0xe
1415fb320: eb ae                       	jmp	0x1415fb2d0 <.text+0x15fa2d0>
1415fb322: b1 15                       	mov	cl, 0x15
1415fb324: eb aa                       	jmp	0x1415fb2d0 <.text+0x15fa2d0>
1415fb326: b1 0d                       	mov	cl, 0xd
1415fb328: eb a6                       	jmp	0x1415fb2d0 <.text+0x15fa2d0>
1415fb32a: b1 11                       	mov	cl, 0x11
1415fb32c: eb a2                       	jmp	0x1415fb2d0 <.text+0x15fa2d0>
1415fb32e: b1 10                       	mov	cl, 0x10
1415fb330: eb 9e                       	jmp	0x1415fb2d0 <.text+0x15fa2d0>
1415fb332: b1 12                       	mov	cl, 0x12
1415fb334: eb 9a                       	jmp	0x1415fb2d0 <.text+0x15fa2d0>
1415fb336: b1 0b                       	mov	cl, 0xb
1415fb338: eb 96                       	jmp	0x1415fb2d0 <.text+0x15fa2d0>
1415fb33a: b1 17                       	mov	cl, 0x17
1415fb33c: eb 92                       	jmp	0x1415fb2d0 <.text+0x15fa2d0>
1415fb33e: b1 0c                       	mov	cl, 0xc
1415fb340: eb 8e                       	jmp	0x1415fb2d0 <.text+0x15fa2d0>
1415fb342: f6 05 54 9a 03 02 01        	test	byte ptr [rip + 0x2039a54], 0x1 # 0x143634d9d
1415fb349: 0f 84 83 00 00 00           	je	0x1415fb3d2 <.text+0x15fa3d2>
1415fb34f: 31 c9                       	xor	ecx, ecx
1415fb351: 48 8d 05 80 33 68 01        	lea	rax, [rip + 0x1683380]  # 0x142c7e6d8
1415fb358: eb 06                       	jmp	0x1415fb360 <.text+0x15fa360>
1415fb35a: b1 18                       	mov	cl, 0x18
1415fb35c: 0f 1f 40 00                 	nop	dword ptr [rax]
1415fb360: 0f b6 c9                    	movzx	ecx, cl
1415fb363: 48 63 14 88                 	movsxd	rdx, dword ptr [rax + 4*rcx]
1415fb367: 48 01 c2                    	add	rdx, rax
1415fb36a: b1 02                       	mov	cl, 0x2
1415fb36c: ff e2                       	jmp	rdx
1415fb36e: b1 01                       	mov	cl, 0x1
1415fb370: eb ee                       	jmp	0x1415fb360 <.text+0x15fa360>
1415fb372: b1 14                       	mov	cl, 0x14
1415fb374: eb ea                       	jmp	0x1415fb360 <.text+0x15fa360>
1415fb376: b1 0f                       	mov	cl, 0xf
1415fb378: eb e6                       	jmp	0x1415fb360 <.text+0x15fa360>
1415fb37a: b1 1a                       	mov	cl, 0x1a
1415fb37c: eb e2                       	jmp	0x1415fb360 <.text+0x15fa360>
1415fb37e: b1 16                       	mov	cl, 0x16
1415fb380: eb de                       	jmp	0x1415fb360 <.text+0x15fa360>
1415fb382: b1 09                       	mov	cl, 0x9
1415fb384: eb da                       	jmp	0x1415fb360 <.text+0x15fa360>
1415fb386: b1 07                       	mov	cl, 0x7
1415fb388: eb d6                       	jmp	0x1415fb360 <.text+0x15fa360>
1415fb38a: b1 03                       	mov	cl, 0x3
1415fb38c: eb d2                       	jmp	0x1415fb360 <.text+0x15fa360>
1415fb38e: b1 08                       	mov	cl, 0x8
1415fb390: eb ce                       	jmp	0x1415fb360 <.text+0x15fa360>
1415fb392: b1 19                       	mov	cl, 0x19
1415fb394: eb ca                       	jmp	0x1415fb360 <.text+0x15fa360>
1415fb396: b1 0a                       	mov	cl, 0xa
1415fb398: eb c6                       	jmp	0x1415fb360 <.text+0x15fa360>
1415fb39a: b1 13                       	mov	cl, 0x13
1415fb39c: eb c2                       	jmp	0x1415fb360 <.text+0x15fa360>
1415fb39e: b1 06                       	mov	cl, 0x6
1415fb3a0: eb be                       	jmp	0x1415fb360 <.text+0x15fa360>
1415fb3a2: b1 05                       	mov	cl, 0x5
1415fb3a4: eb ba                       	jmp	0x1415fb360 <.text+0x15fa360>
1415fb3a6: b1 04                       	mov	cl, 0x4
1415fb3a8: eb b6                       	jmp	0x1415fb360 <.text+0x15fa360>
1415fb3aa: b1 1b                       	mov	cl, 0x1b
1415fb3ac: eb b2                       	jmp	0x1415fb360 <.text+0x15fa360>
1415fb3ae: b1 0e                       	mov	cl, 0xe
1415fb3b0: eb ae                       	jmp	0x1415fb360 <.text+0x15fa360>
1415fb3b2: b1 15                       	mov	cl, 0x15
1415fb3b4: eb aa                       	jmp	0x1415fb360 <.text+0x15fa360>
1415fb3b6: b1 0d                       	mov	cl, 0xd
1415fb3b8: eb a6                       	jmp	0x1415fb360 <.text+0x15fa360>
1415fb3ba: b1 11                       	mov	cl, 0x11
1415fb3bc: eb a2                       	jmp	0x1415fb360 <.text+0x15fa360>
1415fb3be: b1 10                       	mov	cl, 0x10
1415fb3c0: eb 9e                       	jmp	0x1415fb360 <.text+0x15fa360>
1415fb3c2: b1 12                       	mov	cl, 0x12
1415fb3c4: eb 9a                       	jmp	0x1415fb360 <.text+0x15fa360>
1415fb3c6: b1 0b                       	mov	cl, 0xb
1415fb3c8: eb 96                       	jmp	0x1415fb360 <.text+0x15fa360>
1415fb3ca: b1 17                       	mov	cl, 0x17
1415fb3cc: eb 92                       	jmp	0x1415fb360 <.text+0x15fa360>
1415fb3ce: b1 0c                       	mov	cl, 0xc
1415fb3d0: eb 8e                       	jmp	0x1415fb360 <.text+0x15fa360>
1415fb3d2: 31 c9                       	xor	ecx, ecx
1415fb3d4: 48 8d 05 8d 32 68 01        	lea	rax, [rip + 0x168328d]  # 0x142c7e668
1415fb3db: eb 03                       	jmp	0x1415fb3e0 <.text+0x15fa3e0>
1415fb3dd: b1 18                       	mov	cl, 0x18
1415fb3df: 90                          	nop
1415fb3e0: 0f b6 c9                    	movzx	ecx, cl
1415fb3e3: 48 63 14 88                 	movsxd	rdx, dword ptr [rax + 4*rcx]
1415fb3e7: 48 01 c2                    	add	rdx, rax
1415fb3ea: b1 02                       	mov	cl, 0x2
1415fb3ec: ff e2                       	jmp	rdx
1415fb3ee: b1 01                       	mov	cl, 0x1
1415fb3f0: eb ee                       	jmp	0x1415fb3e0 <.text+0x15fa3e0>
1415fb3f2: b1 14                       	mov	cl, 0x14
1415fb3f4: eb ea                       	jmp	0x1415fb3e0 <.text+0x15fa3e0>
1415fb3f6: b1 0f                       	mov	cl, 0xf
1415fb3f8: eb e6                       	jmp	0x1415fb3e0 <.text+0x15fa3e0>
1415fb3fa: b1 1a                       	mov	cl, 0x1a
1415fb3fc: eb e2                       	jmp	0x1415fb3e0 <.text+0x15fa3e0>
1415fb3fe: b1 16                       	mov	cl, 0x16
1415fb400: eb de                       	jmp	0x1415fb3e0 <.text+0x15fa3e0>
1415fb402: b1 09                       	mov	cl, 0x9
1415fb404: eb da                       	jmp	0x1415fb3e0 <.text+0x15fa3e0>
1415fb406: b1 07                       	mov	cl, 0x7
1415fb408: eb d6                       	jmp	0x1415fb3e0 <.text+0x15fa3e0>
1415fb40a: b1 03                       	mov	cl, 0x3
1415fb40c: eb d2                       	jmp	0x1415fb3e0 <.text+0x15fa3e0>
1415fb40e: b1 08                       	mov	cl, 0x8
1415fb410: eb ce                       	jmp	0x1415fb3e0 <.text+0x15fa3e0>
1415fb412: b1 19                       	mov	cl, 0x19
1415fb414: eb ca                       	jmp	0x1415fb3e0 <.text+0x15fa3e0>
1415fb416: b1 0a                       	mov	cl, 0xa
1415fb418: eb c6                       	jmp	0x1415fb3e0 <.text+0x15fa3e0>
1415fb41a: b1 13                       	mov	cl, 0x13
1415fb41c: eb c2                       	jmp	0x1415fb3e0 <.text+0x15fa3e0>
1415fb41e: b1 06                       	mov	cl, 0x6
1415fb420: eb be                       	jmp	0x1415fb3e0 <.text+0x15fa3e0>
1415fb422: b1 05                       	mov	cl, 0x5
1415fb424: eb ba                       	jmp	0x1415fb3e0 <.text+0x15fa3e0>
1415fb426: b1 04                       	mov	cl, 0x4
1415fb428: eb b6                       	jmp	0x1415fb3e0 <.text+0x15fa3e0>
1415fb42a: b1 1b                       	mov	cl, 0x1b
1415fb42c: eb b2                       	jmp	0x1415fb3e0 <.text+0x15fa3e0>
1415fb42e: b1 0e                       	mov	cl, 0xe
1415fb430: eb ae                       	jmp	0x1415fb3e0 <.text+0x15fa3e0>
1415fb432: b1 15                       	mov	cl, 0x15
1415fb434: eb aa                       	jmp	0x1415fb3e0 <.text+0x15fa3e0>
1415fb436: b1 0d                       	mov	cl, 0xd
1415fb438: eb a6                       	jmp	0x1415fb3e0 <.text+0x15fa3e0>
1415fb43a: b1 11                       	mov	cl, 0x11
1415fb43c: eb a2                       	jmp	0x1415fb3e0 <.text+0x15fa3e0>
1415fb43e: b1 10                       	mov	cl, 0x10
1415fb440: eb 9e                       	jmp	0x1415fb3e0 <.text+0x15fa3e0>
1415fb442: b1 12                       	mov	cl, 0x12
1415fb444: eb 9a                       	jmp	0x1415fb3e0 <.text+0x15fa3e0>
1415fb446: b1 0b                       	mov	cl, 0xb
1415fb448: eb 96                       	jmp	0x1415fb3e0 <.text+0x15fa3e0>
1415fb44a: b1 17                       	mov	cl, 0x17
1415fb44c: eb 92                       	jmp	0x1415fb3e0 <.text+0x15fa3e0>
1415fb44e: b1 0c                       	mov	cl, 0xc
1415fb450: eb 8e                       	jmp	0x1415fb3e0 <.text+0x15fa3e0>
1415fb452: 48 83 c4 58                 	add	rsp, 0x58
1415fb456: 5b                          	pop	rbx
1415fb457: 5d                          	pop	rbp
1415fb458: 5f                          	pop	rdi
1415fb459: 5e                          	pop	rsi
1415fb45a: 41 5c                       	pop	r12
1415fb45c: 41 5d                       	pop	r13
1415fb45e: 41 5e                       	pop	r14
1415fb460: 41 5f                       	pop	r15
1415fb462: c3                          	ret
1415fb463: 4c 03 74 24 30              	add	r14, qword ptr [rsp + 0x30]
1415fb468: 49 8b 55 28                 	mov	rdx, qword ptr [r13 + 0x28]
1415fb46c: 4c 39 f2                    	cmp	rdx, r14
1415fb46f: 0f 86 c1 00 00 00           	jbe	0x1415fb536 <.text+0x15fa536>
1415fb475: 49 8b 45 20                 	mov	rax, qword ptr [r13 + 0x20]
1415fb479: 42 8b 04 b0                 	mov	eax, dword ptr [rax + 4*r14]
1415fb47d: 48 8b 4c 24 38              	mov	rcx, qword ptr [rsp + 0x38]
1415fb482: 89 4c 24 24                 	mov	dword ptr [rsp + 0x24], ecx
1415fb486: 89 44 24 28                 	mov	dword ptr [rsp + 0x28], eax
1415fb48a: 4c 8d 44 24 24              	lea	r8, [rsp + 0x24]
1415fb48f: 41 b9 08 00 00 00           	mov	r9d, 0x8
1415fb495: 48 89 f9                    	mov	rcx, rdi
1415fb498: 4c 89 fa                    	mov	rdx, r15
1415fb49b: e8 b0 c5 0f 00              	call	0x1416f7a50 <.text+0x16f6a50>
1415fb4a0: 48 8d 0d 21 ad 4b 01        	lea	rcx, [rip + 0x14bad21]  # 0x142ab61c8
1415fb4a7: e8 34 e5 2d 01              	call	0x1428d99e0 <.text+0x28d89e0>
1415fb4ac: 4c 03 74 24 30              	add	r14, qword ptr [rsp + 0x30]
1415fb4b1: 49 8b 55 28                 	mov	rdx, qword ptr [r13 + 0x28]
1415fb4b5: 4c 39 f2                    	cmp	rdx, r14
1415fb4b8: 76 7c                       	jbe	0x1415fb536 <.text+0x15fa536>
1415fb4ba: 49 8b 45 20                 	mov	rax, qword ptr [r13 + 0x20]
1415fb4be: 42 8b 04 b0                 	mov	eax, dword ptr [rax + 4*r14]
1415fb4c2: 48 8b 4c 24 38              	mov	rcx, qword ptr [rsp + 0x38]
1415fb4c7: 89 4c 24 24                 	mov	dword ptr [rsp + 0x24], ecx
1415fb4cb: 89 44 24 28                 	mov	dword ptr [rsp + 0x28], eax
1415fb4cf: 4c 8d 44 24 24              	lea	r8, [rsp + 0x24]
1415fb4d4: 41 b9 08 00 00 00           	mov	r9d, 0x8
1415fb4da: 48 89 f9                    	mov	rcx, rdi
1415fb4dd: 4c 89 fa                    	mov	rdx, r15
1415fb4e0: e8 6b c5 0f 00              	call	0x1416f7a50 <.text+0x16f6a50>
1415fb4e5: 48 8d 0d f4 ac 4b 01        	lea	rcx, [rip + 0x14bacf4]  # 0x142ab61e0
1415fb4ec: e8 ef e4 2d 01              	call	0x1428d99e0 <.text+0x28d89e0>
1415fb4f1: 4c 03 74 24 30              	add	r14, qword ptr [rsp + 0x30]
1415fb4f6: 49 8b 55 28                 	mov	rdx, qword ptr [r13 + 0x28]
1415fb4fa: 4c 39 f2                    	cmp	rdx, r14
1415fb4fd: 76 37                       	jbe	0x1415fb536 <.text+0x15fa536>
1415fb4ff: 49 8b 45 20                 	mov	rax, qword ptr [r13 + 0x20]
1415fb503: 42 8b 04 b0                 	mov	eax, dword ptr [rax + 4*r14]
1415fb507: 48 8b 4c 24 38              	mov	rcx, qword ptr [rsp + 0x38]
1415fb50c: 89 4c 24 24                 	mov	dword ptr [rsp + 0x24], ecx
1415fb510: 89 44 24 28                 	mov	dword ptr [rsp + 0x28], eax
1415fb514: 4c 8d 44 24 24              	lea	r8, [rsp + 0x24]
1415fb519: 41 b9 08 00 00 00           	mov	r9d, 0x8
1415fb51f: 48 89 f9                    	mov	rcx, rdi
1415fb522: 4c 89 fa                    	mov	rdx, r15
1415fb525: e8 26 c5 0f 00              	call	0x1416f7a50 <.text+0x16f6a50>
1415fb52a: 48 8d 0d c7 ac 4b 01        	lea	rcx, [rip + 0x14bacc7]  # 0x142ab61f8
1415fb531: e8 aa e4 2d 01              	call	0x1428d99e0 <.text+0x28d89e0>
1415fb536: 4c 8d 05 0b 28 68 01        	lea	r8, [rip + 0x168280b]   # 0x142c7dd48
1415fb53d: 4c 89 f1                    	mov	rcx, r14
1415fb540: e8 d3 df 2d 01              	call	0x1428d9518 <.text+0x28d8518>
1415fb545: cc                          	int3
