
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1415fbc50: 41 57                       	push	r15
1415fbc52: 41 56                       	push	r14
1415fbc54: 41 55                       	push	r13
1415fbc56: 41 54                       	push	r12
1415fbc58: 56                          	push	rsi
1415fbc59: 57                          	push	rdi
1415fbc5a: 55                          	push	rbp
1415fbc5b: 53                          	push	rbx
1415fbc5c: 48 83 ec 48                 	sub	rsp, 0x48
1415fbc60: 48 89 ce                    	mov	rsi, rcx
1415fbc63: 48 8b 3a                    	mov	rdi, qword ptr [rdx]
1415fbc66: 48 8b 42 08                 	mov	rax, qword ptr [rdx + 0x8]
1415fbc6a: 48 89 44 24 30              	mov	qword ptr [rsp + 0x30], rax
1415fbc6f: 8b 42 10                    	mov	eax, dword ptr [rdx + 0x10]
1415fbc72: 89 44 24 2c                 	mov	dword ptr [rsp + 0x2c], eax
1415fbc76: 31 ed                       	xor	ebp, ebp
1415fbc78: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
1415fbc80: 4c 8d 0d 61 2d 68 01        	lea	r9, [rip + 0x1682d61]   # 0x142c7e9e8
1415fbc87: 0f b6 0d 0e 91 03 02        	movzx	ecx, byte ptr [rip + 0x203910e] # 0x143634d9c
1415fbc8e: 0f b6 15 08 91 03 02        	movzx	edx, byte ptr [rip + 0x2039108] # 0x143634d9d
1415fbc95: 80 f2 01                    	xor	dl, 0x1
1415fbc98: eb 19                       	jmp	0x1415fbcb3 <.text+0x15facb3>
1415fbc9a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1415fbca0: 3c 02                       	cmp	al, 0x2
1415fbca2: 41 0f 92 c0                 	setb	r8b
1415fbca6: 41 20 d0                    	and	r8b, dl
1415fbca9: 44 3a 46 0a                 	cmp	r8b, byte ptr [rsi + 0xa]
1415fbcad: 0f 84 b7 01 00 00           	je	0x1415fbe6a <.text+0x15fae6a>
1415fbcb3: 89 e8                       	mov	eax, ebp
1415fbcb5: 44 0f b6 f5                 	movzx	r14d, bpl
1415fbcb9: 4f 63 04 b1                 	movsxd	r8, dword ptr [r9 + 4*r14]
1415fbcbd: 4d 01 c8                    	add	r8, r9
1415fbcc0: 41 ff e0                    	jmp	r8
1415fbcc3: 40 b5 01                    	mov	bpl, 0x1
1415fbcc6: f6 c1 01                    	test	cl, 0x1
1415fbcc9: 74 d5                       	je	0x1415fbca0 <.text+0x15faca0>
1415fbccb: e9 90 01 00 00              	jmp	0x1415fbe60 <.text+0x15fae60>
1415fbcd0: 40 b5 11                    	mov	bpl, 0x11
1415fbcd3: f6 c1 01                    	test	cl, 0x1
1415fbcd6: 74 c8                       	je	0x1415fbca0 <.text+0x15faca0>
1415fbcd8: e9 83 01 00 00              	jmp	0x1415fbe60 <.text+0x15fae60>
1415fbcdd: 40 b5 0f                    	mov	bpl, 0xf
1415fbce0: f6 c1 01                    	test	cl, 0x1
1415fbce3: 74 bb                       	je	0x1415fbca0 <.text+0x15faca0>
1415fbce5: e9 76 01 00 00              	jmp	0x1415fbe60 <.text+0x15fae60>
1415fbcea: 40 b5 0c                    	mov	bpl, 0xc
1415fbced: f6 c1 01                    	test	cl, 0x1
1415fbcf0: 74 ae                       	je	0x1415fbca0 <.text+0x15faca0>
1415fbcf2: e9 69 01 00 00              	jmp	0x1415fbe60 <.text+0x15fae60>
1415fbcf7: 40 b5 05                    	mov	bpl, 0x5
1415fbcfa: f6 c1 01                    	test	cl, 0x1
1415fbcfd: 74 a1                       	je	0x1415fbca0 <.text+0x15faca0>
1415fbcff: e9 5c 01 00 00              	jmp	0x1415fbe60 <.text+0x15fae60>
1415fbd04: 40 b5 0d                    	mov	bpl, 0xd
1415fbd07: f6 c1 01                    	test	cl, 0x1
1415fbd0a: 74 94                       	je	0x1415fbca0 <.text+0x15faca0>
1415fbd0c: e9 4f 01 00 00              	jmp	0x1415fbe60 <.text+0x15fae60>
1415fbd11: 40 b5 0a                    	mov	bpl, 0xa
1415fbd14: f6 c1 01                    	test	cl, 0x1
1415fbd17: 74 87                       	je	0x1415fbca0 <.text+0x15faca0>
1415fbd19: e9 42 01 00 00              	jmp	0x1415fbe60 <.text+0x15fae60>
1415fbd1e: 40 b5 1a                    	mov	bpl, 0x1a
1415fbd21: f6 c1 01                    	test	cl, 0x1
1415fbd24: 0f 84 76 ff ff ff           	je	0x1415fbca0 <.text+0x15faca0>
1415fbd2a: e9 31 01 00 00              	jmp	0x1415fbe60 <.text+0x15fae60>
1415fbd2f: 40 b5 19                    	mov	bpl, 0x19
1415fbd32: f6 c1 01                    	test	cl, 0x1
1415fbd35: 0f 84 65 ff ff ff           	je	0x1415fbca0 <.text+0x15faca0>
1415fbd3b: e9 20 01 00 00              	jmp	0x1415fbe60 <.text+0x15fae60>
1415fbd40: 40 b5 03                    	mov	bpl, 0x3
1415fbd43: f6 c1 01                    	test	cl, 0x1
1415fbd46: 0f 84 54 ff ff ff           	je	0x1415fbca0 <.text+0x15faca0>
1415fbd4c: e9 0f 01 00 00              	jmp	0x1415fbe60 <.text+0x15fae60>
1415fbd51: 40 b5 10                    	mov	bpl, 0x10
1415fbd54: f6 c1 01                    	test	cl, 0x1
1415fbd57: 0f 84 43 ff ff ff           	je	0x1415fbca0 <.text+0x15faca0>
1415fbd5d: e9 fe 00 00 00              	jmp	0x1415fbe60 <.text+0x15fae60>
1415fbd62: 40 b5 04                    	mov	bpl, 0x4
1415fbd65: f6 c1 01                    	test	cl, 0x1
1415fbd68: 0f 84 32 ff ff ff           	je	0x1415fbca0 <.text+0x15faca0>
1415fbd6e: e9 ed 00 00 00              	jmp	0x1415fbe60 <.text+0x15fae60>
1415fbd73: 40 b5 08                    	mov	bpl, 0x8
1415fbd76: f6 c1 01                    	test	cl, 0x1
1415fbd79: 0f 84 21 ff ff ff           	je	0x1415fbca0 <.text+0x15faca0>
1415fbd7f: e9 dc 00 00 00              	jmp	0x1415fbe60 <.text+0x15fae60>
1415fbd84: 40 b5 02                    	mov	bpl, 0x2
1415fbd87: f6 c1 01                    	test	cl, 0x1
1415fbd8a: 0f 84 10 ff ff ff           	je	0x1415fbca0 <.text+0x15faca0>
1415fbd90: e9 cb 00 00 00              	jmp	0x1415fbe60 <.text+0x15fae60>
1415fbd95: 40 b5 12                    	mov	bpl, 0x12
1415fbd98: f6 c1 01                    	test	cl, 0x1
1415fbd9b: 0f 84 ff fe ff ff           	je	0x1415fbca0 <.text+0x15faca0>
1415fbda1: e9 ba 00 00 00              	jmp	0x1415fbe60 <.text+0x15fae60>
1415fbda6: 40 b5 15                    	mov	bpl, 0x15
1415fbda9: f6 c1 01                    	test	cl, 0x1
1415fbdac: 0f 84 ee fe ff ff           	je	0x1415fbca0 <.text+0x15faca0>
1415fbdb2: e9 a9 00 00 00              	jmp	0x1415fbe60 <.text+0x15fae60>
1415fbdb7: 40 b5 06                    	mov	bpl, 0x6
1415fbdba: f6 c1 01                    	test	cl, 0x1
1415fbdbd: 0f 84 dd fe ff ff           	je	0x1415fbca0 <.text+0x15faca0>
1415fbdc3: e9 98 00 00 00              	jmp	0x1415fbe60 <.text+0x15fae60>
1415fbdc8: 40 b5 16                    	mov	bpl, 0x16
1415fbdcb: f6 c1 01                    	test	cl, 0x1
1415fbdce: 0f 84 cc fe ff ff           	je	0x1415fbca0 <.text+0x15faca0>
1415fbdd4: e9 87 00 00 00              	jmp	0x1415fbe60 <.text+0x15fae60>
1415fbdd9: 40 b5 0e                    	mov	bpl, 0xe
1415fbddc: f6 c1 01                    	test	cl, 0x1
1415fbddf: 0f 84 bb fe ff ff           	je	0x1415fbca0 <.text+0x15faca0>
1415fbde5: eb 79                       	jmp	0x1415fbe60 <.text+0x15fae60>
1415fbde7: 40 b5 07                    	mov	bpl, 0x7
1415fbdea: f6 c1 01                    	test	cl, 0x1
1415fbded: 0f 84 ad fe ff ff           	je	0x1415fbca0 <.text+0x15faca0>
1415fbdf3: eb 6b                       	jmp	0x1415fbe60 <.text+0x15fae60>
1415fbdf5: 40 b5 13                    	mov	bpl, 0x13
1415fbdf8: f6 c1 01                    	test	cl, 0x1
1415fbdfb: 0f 84 9f fe ff ff           	je	0x1415fbca0 <.text+0x15faca0>
1415fbe01: eb 5d                       	jmp	0x1415fbe60 <.text+0x15fae60>
1415fbe03: 40 b5 0b                    	mov	bpl, 0xb
1415fbe06: f6 c1 01                    	test	cl, 0x1
1415fbe09: 0f 84 91 fe ff ff           	je	0x1415fbca0 <.text+0x15faca0>
1415fbe0f: eb 4f                       	jmp	0x1415fbe60 <.text+0x15fae60>
1415fbe11: 40 b5 09                    	mov	bpl, 0x9
1415fbe14: f6 c1 01                    	test	cl, 0x1
1415fbe17: 0f 84 83 fe ff ff           	je	0x1415fbca0 <.text+0x15faca0>
1415fbe1d: eb 41                       	jmp	0x1415fbe60 <.text+0x15fae60>
1415fbe1f: 40 b5 17                    	mov	bpl, 0x17
1415fbe22: f6 c1 01                    	test	cl, 0x1
1415fbe25: 0f 84 75 fe ff ff           	je	0x1415fbca0 <.text+0x15faca0>
1415fbe2b: eb 33                       	jmp	0x1415fbe60 <.text+0x15fae60>
1415fbe2d: 40 b5 14                    	mov	bpl, 0x14
1415fbe30: f6 c1 01                    	test	cl, 0x1
1415fbe33: 0f 84 67 fe ff ff           	je	0x1415fbca0 <.text+0x15faca0>
1415fbe39: eb 25                       	jmp	0x1415fbe60 <.text+0x15fae60>
1415fbe3b: 40 b5 18                    	mov	bpl, 0x18
1415fbe3e: f6 c1 01                    	test	cl, 0x1
1415fbe41: 0f 84 59 fe ff ff           	je	0x1415fbca0 <.text+0x15faca0>
1415fbe47: eb 17                       	jmp	0x1415fbe60 <.text+0x15fae60>
1415fbe49: 40 b5 1b                    	mov	bpl, 0x1b
1415fbe4c: f6 c1 01                    	test	cl, 0x1
1415fbe4f: 0f 84 4b fe ff ff           	je	0x1415fbca0 <.text+0x15faca0>
1415fbe55: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
1415fbe60: 80 7e 0a 00                 	cmp	byte ptr [rsi + 0xa], 0x0
1415fbe64: 0f 84 49 fe ff ff           	je	0x1415fbcb3 <.text+0x15facb3>
1415fbe6a: 0f b6 5e 08                 	movzx	ebx, byte ptr [rsi + 0x8]
1415fbe6e: 0f b6 4e 09                 	movzx	ecx, byte ptr [rsi + 0x9]
1415fbe72: 48 89 4c 24 40              	mov	qword ptr [rsp + 0x40], rcx
1415fbe77: 38 cb                       	cmp	bl, cl
1415fbe79: 0f 83 08 fe ff ff           	jae	0x1415fbc87 <.text+0x15fac87>
1415fbe7f: c0 e0 02                    	shl	al, 0x2
1415fbe82: 0f b6 c0                    	movzx	eax, al
1415fbe85: 48 89 44 24 38              	mov	qword ptr [rsp + 0x38], rax
1415fbe8a: 4c 8d 3c 03                 	lea	r15, [rbx + rax]
1415fbe8e: 89 d8                       	mov	eax, ebx
1415fbe90: c1 e0 04                    	shl	eax, 0x4
1415fbe93: 41 c1 e6 06                 	shl	r14d, 0x6
1415fbe97: 49 01 c6                    	add	r14, rax
1415fbe9a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1415fbea0: 48 8b 0e                    	mov	rcx, qword ptr [rsi]
1415fbea3: 48 8b 51 10                 	mov	rdx, qword ptr [rcx + 0x10]
1415fbea7: 4c 39 fa                    	cmp	rdx, r15
1415fbeaa: 0f 86 ff 00 00 00           	jbe	0x1415fbfaf <.text+0x15fafaf>
1415fbeb0: 48 8b 41 08                 	mov	rax, qword ptr [rcx + 0x8]
1415fbeb4: 42 8b 04 b8                 	mov	eax, dword ptr [rax + 4*r15]
1415fbeb8: 80 7e 0a 00                 	cmp	byte ptr [rsi + 0xa], 0x0
1415fbebc: 74 22                       	je	0x1415fbee0 <.text+0x15faee0>
1415fbebe: 48 8b 4c 24 38              	mov	rcx, qword ptr [rsp + 0x38]
1415fbec3: 01 d9                       	add	ecx, ebx
1415fbec5: 89 4c 24 20                 	mov	dword ptr [rsp + 0x20], ecx
1415fbec9: 89 44 24 24                 	mov	dword ptr [rsp + 0x24], eax
1415fbecd: 4c 8d 44 24 20              	lea	r8, [rsp + 0x20]
1415fbed2: 41 b9 08 00 00 00           	mov	r9d, 0x8
1415fbed8: eb 39                       	jmp	0x1415fbf13 <.text+0x15faf13>
1415fbeda: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1415fbee0: 48 8b 51 28                 	mov	rdx, qword ptr [rcx + 0x28]
1415fbee4: 4c 39 fa                    	cmp	rdx, r15
1415fbee7: 0f 86 d1 00 00 00           	jbe	0x1415fbfbe <.text+0x15fafbe>
1415fbeed: 48 8b 49 20                 	mov	rcx, qword ptr [rcx + 0x20]
1415fbef1: 42 8b 0c b9                 	mov	ecx, dword ptr [rcx + 4*r15]
1415fbef5: 48 8b 54 24 38              	mov	rdx, qword ptr [rsp + 0x38]
1415fbefa: 01 da                       	add	edx, ebx
1415fbefc: 89 54 24 20                 	mov	dword ptr [rsp + 0x20], edx
1415fbf00: 89 44 24 24                 	mov	dword ptr [rsp + 0x24], eax
1415fbf04: 89 4c 24 28                 	mov	dword ptr [rsp + 0x28], ecx
1415fbf08: 4c 8d 44 24 20              	lea	r8, [rsp + 0x20]
1415fbf0d: 41 b9 0c 00 00 00           	mov	r9d, 0xc
1415fbf13: 48 89 f9                    	mov	rcx, rdi
1415fbf16: 48 8b 54 24 30              	mov	rdx, qword ptr [rsp + 0x30]
1415fbf1b: e8 30 bb 0f 00              	call	0x1416f7a50 <.text+0x16f6a50>
1415fbf20: 4c 8b 2f                    	mov	r13, qword ptr [rdi]
1415fbf23: 4c 8b 67 08                 	mov	r12, qword ptr [rdi + 0x8]
1415fbf27: 41 80 bc 24 ba 0a 00 00 00  	cmp	byte ptr [r12 + 0xaba], 0x0
1415fbf30: 74 0c                       	je	0x1415fbf3e <.text+0x15faf3e>
1415fbf32: 49 8b 55 48                 	mov	rdx, qword ptr [r13 + 0x48]
1415fbf36: 4c 89 e1                    	mov	rcx, r12
1415fbf39: e8 52 21 5f ff              	call	0x140bee090 <.text+0xbed090>
1415fbf3e: f0                          	lock
1415fbf3f: 49 ff 84 24 a0 0a 00 00     	inc	qword ptr [r12 + 0xaa0]
1415fbf47: 4d 8b 6d 48                 	mov	r13, qword ptr [r13 + 0x48]
1415fbf4b: 48 8b 4f 10                 	mov	rcx, qword ptr [rdi + 0x10]
1415fbf4f: 8b 54 24 2c                 	mov	edx, dword ptr [rsp + 0x2c]
1415fbf53: e8 78 ba 5e ff              	call	0x140be79d0 <.text+0xbe69d0>
1415fbf58: 4c 89 e1                    	mov	rcx, r12
1415fbf5b: 48 89 c2                    	mov	rdx, rax
1415fbf5e: 45 31 c0                    	xor	r8d, r8d
1415fbf61: e8 0a 35 5f ff              	call	0x140bef470 <.text+0xbee470>
1415fbf66: a8 01                       	test	al, 0x1
1415fbf68: 74 39                       	je	0x1415fbfa3 <.text+0x15fafa3>
1415fbf6a: 48 ff c3                    	inc	rbx
1415fbf6d: 4c 89 e9                    	mov	rcx, r13
1415fbf70: 4d 89 f0                    	mov	r8, r14
1415fbf73: 41 ff 94 24 68 03 00 00     	call	qword ptr [r12 + 0x368]
1415fbf7b: 49 ff c7                    	inc	r15
1415fbf7e: 49 83 c6 10                 	add	r14, 0x10
1415fbf82: 48 3b 5c 24 40              	cmp	rbx, qword ptr [rsp + 0x40]
1415fbf87: 0f 82 13 ff ff ff           	jb	0x1415fbea0 <.text+0x15faea0>
1415fbf8d: e9 ee fc ff ff              	jmp	0x1415fbc80 <.text+0x15fac80>
1415fbf92: 48 83 c4 48                 	add	rsp, 0x48
1415fbf96: 5b                          	pop	rbx
1415fbf97: 5d                          	pop	rbp
1415fbf98: 5f                          	pop	rdi
1415fbf99: 5e                          	pop	rsi
1415fbf9a: 41 5c                       	pop	r12
1415fbf9c: 41 5d                       	pop	r13
1415fbf9e: 41 5e                       	pop	r14
1415fbfa0: 41 5f                       	pop	r15
1415fbfa2: c3                          	ret
1415fbfa3: 48 8d 0d 06 a2 4b 01        	lea	rcx, [rip + 0x14ba206]  # 0x142ab61b0
1415fbfaa: e8 61 d3 2d 01              	call	0x1428d9310 <.text+0x28d8310>
1415fbfaf: 4c 8d 05 32 1d 68 01        	lea	r8, [rip + 0x1681d32]   # 0x142c7dce8
1415fbfb6: 4c 89 f9                    	mov	rcx, r15
1415fbfb9: e8 5a d5 2d 01              	call	0x1428d9518 <.text+0x28d8518>
1415fbfbe: 4c 8d 05 3b 1d 68 01        	lea	r8, [rip + 0x1681d3b]   # 0x142c7dd00
1415fbfc5: 4c 89 f9                    	mov	rcx, r15
1415fbfc8: e8 4b d5 2d 01              	call	0x1428d9518 <.text+0x28d8518>
1415fbfcd: cc                          	int3
