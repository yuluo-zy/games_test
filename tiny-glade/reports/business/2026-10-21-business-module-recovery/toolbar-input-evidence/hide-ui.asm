
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14233eb50: 41 57                       	push	r15
14233eb52: 41 56                       	push	r14
14233eb54: 41 55                       	push	r13
14233eb56: 41 54                       	push	r12
14233eb58: 56                          	push	rsi
14233eb59: 57                          	push	rdi
14233eb5a: 55                          	push	rbp
14233eb5b: 53                          	push	rbx
14233eb5c: 48 83 ec 68                 	sub	rsp, 0x68
14233eb60: 4c 89 cb                    	mov	rbx, r9
14233eb63: 4c 89 c6                    	mov	rsi, r8
14233eb66: 48 89 d7                    	mov	rdi, rdx
14233eb69: 4c 8b a4 24 e0 00 00 00     	mov	r12, qword ptr [rsp + 0xe0]
14233eb71: 4c 8b bc 24 d0 00 00 00     	mov	r15, qword ptr [rsp + 0xd0]
14233eb79: 48 8b 09                    	mov	rcx, qword ptr [rcx]
14233eb7c: e8 5f 10 60 fe              	call	0x14093fbe0 <.text+0x93ebe0>
14233eb81: 84 c0                       	test	al, al
14233eb83: 74 2c                       	je	0x14233ebb1 <.text+0x233dbb1>
14233eb85: 48 8b 0b                    	mov	rcx, qword ptr [rbx]
14233eb88: e8 b3 a5 66 fe              	call	0x1409a9140 <.text+0x9a8140>
14233eb8d: 84 c0                       	test	al, al
14233eb8f: 74 20                       	je	0x14233ebb1 <.text+0x233dbb1>
14233eb91: 48 8b 84 24 d8 00 00 00     	mov	rax, qword ptr [rsp + 0xd8]
14233eb99: 48 8b 00                    	mov	rax, qword ptr [rax]
14233eb9c: 0f b6 00                    	movzx	eax, byte ptr [rax]
14233eb9f: 89 c1                       	mov	ecx, eax
14233eba1: 80 f1 01                    	xor	cl, 0x1
14233eba4: 89 4c 24 24                 	mov	dword ptr [rsp + 0x24], ecx
14233eba8: 84 c0                       	test	al, al
14233ebaa: 75 0d                       	jne	0x14233ebb9 <.text+0x233dbb9>
14233ebac: 4d 8b 37                    	mov	r14, qword ptr [r15]
14233ebaf: eb 2c                       	jmp	0x14233ebdd <.text+0x233dbdd>
14233ebb1: c7 44 24 24 00 00 00 00     	mov	dword ptr [rsp + 0x24], 0x0
14233ebb9: 4d 8b 37                    	mov	r14, qword ptr [r15]
14233ebbc: 49 8d 5e 10                 	lea	rbx, [r14 + 0x10]
14233ebc0: 48 89 d9                    	mov	rcx, rbx
14233ebc3: e8 88 a5 66 fe              	call	0x1409a9150 <.text+0x9a8150>
14233ebc8: 84 c0                       	test	al, al
14233ebca: 74 11                       	je	0x14233ebdd <.text+0x233dbdd>
14233ebcc: 41 8b 47 1c                 	mov	eax, dword ptr [r15 + 0x1c]
14233ebd0: 49 8b 4f 10                 	mov	rcx, qword ptr [r15 + 0x10]
14233ebd4: 89 01                       	mov	dword ptr [rcx], eax
14233ebd6: 48 c7 03 00 00 00 00        	mov	qword ptr [rbx], 0x0
14233ebdd: 49 8b 04 24                 	mov	rax, qword ptr [r12]
14233ebe1: 49 8b 4c 24 08              	mov	rcx, qword ptr [r12 + 0x8]
14233ebe6: 48 89 4c 24 30              	mov	qword ptr [rsp + 0x30], rcx
14233ebeb: 41 8b 4c 24 14              	mov	ecx, dword ptr [r12 + 0x14]
14233ebf0: 89 4c 24 2c                 	mov	dword ptr [rsp + 0x2c], ecx
14233ebf4: 4c 8b 80 f8 00 00 00        	mov	r8, qword ptr [rax + 0xf8]
14233ebfb: 48 8b 88 00 01 00 00        	mov	rcx, qword ptr [rax + 0x100]
14233ec02: 4d 8d 0c 88                 	lea	r9, [r8 + 4*rcx]
14233ec06: 44 0f b6 90 08 01 00 00     	movzx	r10d, byte ptr [rax + 0x108]
14233ec0e: 4c 8b 27                    	mov	r12, qword ptr [rdi]
14233ec11: 49 ff c4                    	inc	r12
14233ec14: 48 8b 0e                    	mov	rcx, qword ptr [rsi]
14233ec17: 48 83 c1 0a                 	add	rcx, 0xa
14233ec1b: 48 89 4c 24 58              	mov	qword ptr [rsp + 0x58], rcx
14233ec20: 41 8b 4f 1c                 	mov	ecx, dword ptr [r15 + 0x1c]
14233ec24: 89 4c 24 28                 	mov	dword ptr [rsp + 0x28], ecx
14233ec28: 49 8b 5f 10                 	mov	rbx, qword ptr [r15 + 0x10]
14233ec2c: b9 08 00 00 00              	mov	ecx, 0x8
14233ec31: 48 89 4c 24 50              	mov	qword ptr [rsp + 0x50], rcx
14233ec36: 31 ed                       	xor	ebp, ebp
14233ec38: 45 31 ed                    	xor	r13d, r13d
14233ec3b: 45 31 ff                    	xor	r15d, r15d
14233ec3e: 48 89 4c 24 48              	mov	qword ptr [rsp + 0x48], rcx
14233ec43: 31 f6                       	xor	esi, esi
14233ec45: 45 84 d2                    	test	r10b, r10b
14233ec48: 74 46                       	je	0x14233ec90 <.text+0x233dc90>
14233ec4a: 4d 39 fd                    	cmp	r13, r15
14233ec4d: 0f 85 00 01 00 00           	jne	0x14233ed53 <.text+0x233dd53>
14233ec53: 4d 39 c8                    	cmp	r8, r9
14233ec56: 0f 84 61 02 00 00           	je	0x14233eebd <.text+0x233debd>
14233ec5c: 48 8b 4c 24 30              	mov	rcx, qword ptr [rsp + 0x30]
14233ec61: 48 8b 89 a8 01 00 00        	mov	rcx, qword ptr [rcx + 0x1a8]
14233ec68: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
14233ec70: 41 8b 10                    	mov	edx, dword ptr [r8]
14233ec73: 48 8d 14 d2                 	lea	rdx, [rdx + 8*rdx]
14233ec77: 4c 8b 7c d1 10              	mov	r15, qword ptr [rcx + 8*rdx + 0x10]
14233ec7c: 4d 85 ff                    	test	r15, r15
14233ec7f: 75 52                       	jne	0x14233ecd3 <.text+0x233dcd3>
14233ec81: 49 83 c0 04                 	add	r8, 0x4
14233ec85: 4d 39 c8                    	cmp	r8, r9
14233ec88: 75 e6                       	jne	0x14233ec70 <.text+0x233dc70>
14233ec8a: e9 2e 02 00 00              	jmp	0x14233eebd <.text+0x233debd>
14233ec8f: 90                          	nop
14233ec90: 4d 39 fd                    	cmp	r13, r15
14233ec93: 0f 85 24 01 00 00           	jne	0x14233edbd <.text+0x233ddbd>
14233ec99: 4d 39 c8                    	cmp	r8, r9
14233ec9c: 0f 84 1b 02 00 00           	je	0x14233eebd <.text+0x233debd>
14233eca2: 48 8b 4c 24 30              	mov	rcx, qword ptr [rsp + 0x30]
14233eca7: 48 8b 91 00 01 00 00        	mov	rdx, qword ptr [rcx + 0x100]
14233ecae: 66 90                       	nop
14233ecb0: 41 8b 08                    	mov	ecx, dword ptr [r8]
14233ecb3: 48 8d 0c 89                 	lea	rcx, [rcx + 4*rcx]
14233ecb7: 48 c1 e1 05                 	shl	rcx, 0x5
14233ecbb: 4c 8b 7c 0a 58              	mov	r15, qword ptr [rdx + rcx + 0x58]
14233ecc0: 4d 85 ff                    	test	r15, r15
14233ecc3: 75 2e                       	jne	0x14233ecf3 <.text+0x233dcf3>
14233ecc5: 49 83 c0 04                 	add	r8, 0x4
14233ecc9: 4d 39 c8                    	cmp	r8, r9
14233eccc: 75 e2                       	jne	0x14233ecb0 <.text+0x233dcb0>
14233ecce: e9 ea 01 00 00              	jmp	0x14233eebd <.text+0x233debd>
14233ecd3: 48 8d 14 d1                 	lea	rdx, [rcx + 8*rdx]
14233ecd7: 48 8b 4a 38                 	mov	rcx, qword ptr [rdx + 0x38]
14233ecdb: 4c 8b 98 20 01 00 00        	mov	r11, qword ptr [rax + 0x120]
14233ece2: 4c 39 5a 40                 	cmp	qword ptr [rdx + 0x40], r11
14233ece6: 76 30                       	jbe	0x14233ed18 <.text+0x233dd18>
14233ece8: 4a 83 3c d9 00              	cmp	qword ptr [rcx + 8*r11], 0x0
14233eced: 40 0f 95 c7                 	setne	dil
14233ecf1: eb 27                       	jmp	0x14233ed1a <.text+0x233dd1a>
14233ecf3: 48 01 ca                    	add	rdx, rcx
14233ecf6: 48 8b 88 20 01 00 00        	mov	rcx, qword ptr [rax + 0x120]
14233ecfd: 48 39 8a 88 00 00 00        	cmp	qword ptr [rdx + 0x88], rcx
14233ed04: 76 60                       	jbe	0x14233ed66 <.text+0x233dd66>
14233ed06: 4c 8b 9a 80 00 00 00        	mov	r11, qword ptr [rdx + 0x80]
14233ed0d: 49 83 3c cb 00              	cmp	qword ptr [r11 + 8*rcx], 0x0
14233ed12: 40 0f 95 c5                 	setne	bpl
14233ed16: eb 50                       	jmp	0x14233ed68 <.text+0x233dd68>
14233ed18: 31 ff                       	xor	edi, edi
14233ed1a: 4c 8b 5a 18                 	mov	r11, qword ptr [rdx + 0x18]
14233ed1e: 48 8b b0 10 01 00 00        	mov	rsi, qword ptr [rax + 0x110]
14233ed25: 48 8b 0c f1                 	mov	rcx, qword ptr [rcx + 8*rsi]
14233ed29: 48 f7 d1                    	not	rcx
14233ed2c: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
14233ed30: 48 c1 e1 04                 	shl	rcx, 0x4
14233ed34: 49 8b 6c 0b 10              	mov	rbp, qword ptr [r11 + rcx + 0x10]
14233ed39: 4d 8b 5c 0b 28              	mov	r11, qword ptr [r11 + rcx + 0x28]
14233ed3e: 49 83 c0 04                 	add	r8, 0x4
14233ed42: 48 8b 4a 08                 	mov	rcx, qword ptr [rdx + 0x8]
14233ed46: 48 89 4c 24 48              	mov	qword ptr [rsp + 0x48], rcx
14233ed4b: 45 31 ed                    	xor	r13d, r13d
14233ed4e: 89 fe                       	mov	esi, edi
14233ed50: 4c 89 df                    	mov	rdi, r11
14233ed53: 48 8b 4c 24 48              	mov	rcx, qword ptr [rsp + 0x48]
14233ed58: 4a 8b 0c e9                 	mov	rcx, qword ptr [rcx + 8*r13]
14233ed5c: 48 89 4c 24 38              	mov	qword ptr [rsp + 0x38], rcx
14233ed61: 45 89 eb                    	mov	r11d, r13d
14233ed64: eb 71                       	jmp	0x14233edd7 <.text+0x233ddd7>
14233ed66: 31 ed                       	xor	ebp, ebp
14233ed68: 8b 8a 94 00 00 00           	mov	ecx, dword ptr [rdx + 0x94]
14233ed6e: 4c 8b 5c 24 30              	mov	r11, qword ptr [rsp + 0x30]
14233ed73: 4d 8b 9b a8 01 00 00        	mov	r11, qword ptr [r11 + 0x1a8]
14233ed7a: 48 8d 0c c9                 	lea	rcx, [rcx + 8*rcx]
14233ed7e: 49 8b 74 cb 18              	mov	rsi, qword ptr [r11 + 8*rcx + 0x18]
14233ed83: 49 8b 4c cb 38              	mov	rcx, qword ptr [r11 + 8*rcx + 0x38]
14233ed88: 4c 8b 98 10 01 00 00        	mov	r11, qword ptr [rax + 0x110]
14233ed8f: 4a 8b 0c d9                 	mov	rcx, qword ptr [rcx + 8*r11]
14233ed93: 48 f7 d1                    	not	rcx
14233ed96: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
14233ed9a: 48 c1 e1 04                 	shl	rcx, 0x4
14233ed9e: 4c 8b 5c 0e 10              	mov	r11, qword ptr [rsi + rcx + 0x10]
14233eda3: 48 8b 7c 0e 28              	mov	rdi, qword ptr [rsi + rcx + 0x28]
14233eda8: 49 83 c0 04                 	add	r8, 0x4
14233edac: 48 8b 4a 50                 	mov	rcx, qword ptr [rdx + 0x50]
14233edb0: 48 89 4c 24 50              	mov	qword ptr [rsp + 0x50], rcx
14233edb5: 45 31 ed                    	xor	r13d, r13d
14233edb8: 89 ee                       	mov	esi, ebp
14233edba: 4c 89 dd                    	mov	rbp, r11
14233edbd: 4c 89 ea                    	mov	rdx, r13
14233edc0: 48 c1 e2 04                 	shl	rdx, 0x4
14233edc4: 48 8b 4c 24 50              	mov	rcx, qword ptr [rsp + 0x50]
14233edc9: 4c 8b 1c 11                 	mov	r11, qword ptr [rcx + rdx]
14233edcd: 4c 89 5c 24 38              	mov	qword ptr [rsp + 0x38], r11
14233edd2: 44 8b 5c 11 08              	mov	r11d, dword ptr [rcx + rdx + 0x8]
14233edd7: 4b 8d 14 5b                 	lea	rdx, [r11 + 2*r11]
14233eddb: 48 01 ea                    	add	rdx, rbp
14233edde: 4a 8d 0c 9f                 	lea	rcx, [rdi + 4*r11]
14233ede2: 49 ff c5                    	inc	r13
14233ede5: 40 f6 c6 01                 	test	sil, 0x1
14233ede9: 74 25                       	je	0x14233ee10 <.text+0x233de10>
14233edeb: 80 7c 24 24 00              	cmp	byte ptr [rsp + 0x24], 0x0
14233edf0: 74 0e                       	je	0x14233ee00 <.text+0x233de00>
14233edf2: 41 80 3c 24 00              	cmp	byte ptr [r12], 0x0
14233edf7: 4c 8b 5c 24 58              	mov	r11, qword ptr [rsp + 0x58]
14233edfc: 75 1c                       	jne	0x14233ee1a <.text+0x233de1a>
14233edfe: 66 90                       	nop
14233ee00: 44 8b 5c 24 2c              	mov	r11d, dword ptr [rsp + 0x2c]
14233ee05: 44 89 19                    	mov	dword ptr [rcx], r11d
14233ee08: c6 02 00                    	mov	byte ptr [rdx], 0x0
14233ee0b: eb 65                       	jmp	0x14233ee72 <.text+0x233de72>
14233ee0d: 0f 1f 00                    	nop	dword ptr [rax]
14233ee10: 4d 89 e3                    	mov	r11, r12
14233ee13: 80 7c 24 24 00              	cmp	byte ptr [rsp + 0x24], 0x0
14233ee18: 74 e6                       	je	0x14233ee00 <.text+0x233de00>
14233ee1a: 89 74 24 44                 	mov	dword ptr [rsp + 0x44], esi
14233ee1e: 48 89 6c 24 60              	mov	qword ptr [rsp + 0x60], rbp
14233ee23: 4c 89 f5                    	mov	rbp, r14
14233ee26: 49 89 de                    	mov	r14, rbx
14233ee29: 48 89 fb                    	mov	rbx, rdi
14233ee2c: 48 89 c7                    	mov	rdi, rax
14233ee2f: 4c 89 e6                    	mov	rsi, r12
14233ee32: 45 0f b6 23                 	movzx	r12d, byte ptr [r11]
14233ee36: 45 89 e3                    	mov	r11d, r12d
14233ee39: 41 80 f3 01                 	xor	r11b, 0x1
14233ee3d: 48 89 c8                    	mov	rax, rcx
14233ee40: 44 89 d1                    	mov	ecx, r10d
14233ee43: 44 8b 54 24 2c              	mov	r10d, dword ptr [rsp + 0x2c]
14233ee48: 44 89 10                    	mov	dword ptr [rax], r10d
14233ee4b: 41 89 ca                    	mov	r10d, ecx
14233ee4e: 44 88 1a                    	mov	byte ptr [rdx], r11b
14233ee51: 45 84 e4                    	test	r12b, r12b
14233ee54: 49 89 f4                    	mov	r12, rsi
14233ee57: 48 89 f8                    	mov	rax, rdi
14233ee5a: 48 89 df                    	mov	rdi, rbx
14233ee5d: 4c 89 f3                    	mov	rbx, r14
14233ee60: 49 89 ee                    	mov	r14, rbp
14233ee63: 8b 74 24 44                 	mov	esi, dword ptr [rsp + 0x44]
14233ee67: 48 8b 6c 24 60              	mov	rbp, qword ptr [rsp + 0x60]
14233ee6c: 0f 84 d3 fd ff ff           	je	0x14233ec45 <.text+0x233dc45>
14233ee72: 41 83 7e 10 01              	cmp	dword ptr [r14 + 0x10], 0x1
14233ee77: 75 19                       	jne	0x14233ee92 <.text+0x233de92>
14233ee79: 48 8b 4c 24 38              	mov	rcx, qword ptr [rsp + 0x38]
14233ee7e: 49 39 4e 18                 	cmp	qword ptr [r14 + 0x18], rcx
14233ee82: 75 0e                       	jne	0x14233ee92 <.text+0x233de92>
14233ee84: 8b 4c 24 28                 	mov	ecx, dword ptr [rsp + 0x28]
14233ee88: 89 0b                       	mov	dword ptr [rbx], ecx
14233ee8a: 49 c7 46 10 00 00 00 00     	mov	qword ptr [r14 + 0x10], 0x0
14233ee92: 41 83 3e 01                 	cmp	dword ptr [r14], 0x1
14233ee96: 0f 85 a9 fd ff ff           	jne	0x14233ec45 <.text+0x233dc45>
14233ee9c: 48 8b 4c 24 38              	mov	rcx, qword ptr [rsp + 0x38]
14233eea1: 49 39 4e 08                 	cmp	qword ptr [r14 + 0x8], rcx
14233eea5: 0f 85 9a fd ff ff           	jne	0x14233ec45 <.text+0x233dc45>
14233eeab: 8b 4c 24 28                 	mov	ecx, dword ptr [rsp + 0x28]
14233eeaf: 89 0b                       	mov	dword ptr [rbx], ecx
14233eeb1: 49 c7 06 00 00 00 00        	mov	qword ptr [r14], 0x0
14233eeb8: e9 88 fd ff ff              	jmp	0x14233ec45 <.text+0x233dc45>
14233eebd: 48 83 c4 68                 	add	rsp, 0x68
14233eec1: 5b                          	pop	rbx
14233eec2: 5d                          	pop	rbp
14233eec3: 5f                          	pop	rdi
14233eec4: 5e                          	pop	rsi
14233eec5: 41 5c                       	pop	r12
14233eec7: 41 5d                       	pop	r13
14233eec9: 41 5e                       	pop	r14
14233eecb: 41 5f                       	pop	r15
14233eecd: c3                          	ret
