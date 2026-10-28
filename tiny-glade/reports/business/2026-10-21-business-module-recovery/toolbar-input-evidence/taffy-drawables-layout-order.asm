
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
142340d20: 41 57                       	push	r15
142340d22: 41 56                       	push	r14
142340d24: 41 55                       	push	r13
142340d26: 41 54                       	push	r12
142340d28: 56                          	push	rsi
142340d29: 57                          	push	rdi
142340d2a: 55                          	push	rbp
142340d2b: 53                          	push	rbx
142340d2c: 48 81 ec a8 00 00 00        	sub	rsp, 0xa8
142340d33: 0f 29 b4 24 90 00 00 00     	movaps	xmmword ptr [rsp + 0x90], xmm6
142340d3b: 4c 8b 01                    	mov	r8, qword ptr [rcx]
142340d3e: 48 8b 41 08                 	mov	rax, qword ptr [rcx + 0x8]
142340d42: 48 89 44 24 38              	mov	qword ptr [rsp + 0x38], rax
142340d47: 8b 41 14                    	mov	eax, dword ptr [rcx + 0x14]
142340d4a: 89 44 24 2c                 	mov	dword ptr [rsp + 0x2c], eax
142340d4e: 4d 8b b8 f8 00 00 00        	mov	r15, qword ptr [r8 + 0xf8]
142340d55: 49 8b 80 00 01 00 00        	mov	rax, qword ptr [r8 + 0x100]
142340d5c: 4d 8d 24 87                 	lea	r12, [r15 + 4*rax]
142340d60: 48 8b 02                    	mov	rax, qword ptr [rdx]
142340d63: 48 89 44 24 40              	mov	qword ptr [rsp + 0x40], rax
142340d68: 4c 89 44 24 30              	mov	qword ptr [rsp + 0x30], r8
142340d6d: 41 80 b8 08 01 00 00 00     	cmp	byte ptr [r8 + 0x108], 0x0
142340d75: 0f 84 22 01 00 00           	je	0x142340e9d <.text+0x233fe9d>
142340d7b: 31 ed                       	xor	ebp, ebp
142340d7d: 0f 28 35 5c 4b 5e 00        	movaps	xmm6, xmmword ptr [rip + 0x5e4b5c] # 0x1429258e0
142340d84: 45 31 f6                    	xor	r14d, r14d
142340d87: 45 31 ed                    	xor	r13d, r13d
142340d8a: 31 f6                       	xor	esi, esi
142340d8c: 4c 39 ee                    	cmp	rsi, r13
142340d8f: 0f 85 9f 00 00 00           	jne	0x142340e34 <.text+0x233fe34>
142340d95: 4d 39 e7                    	cmp	r15, r12
142340d98: 0f 84 79 02 00 00           	je	0x142341017 <.text+0x2340017>
142340d9e: 48 8b 44 24 38              	mov	rax, qword ptr [rsp + 0x38]
142340da3: 48 8b 80 a8 01 00 00        	mov	rax, qword ptr [rax + 0x1a8]
142340daa: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
142340db0: 41 8b 0f                    	mov	ecx, dword ptr [r15]
142340db3: 48 8d 0c c9                 	lea	rcx, [rcx + 8*rcx]
142340db7: 4c 8b 6c c8 10              	mov	r13, qword ptr [rax + 8*rcx + 0x10]
142340dbc: 4d 85 ed                    	test	r13, r13
142340dbf: 75 0f                       	jne	0x142340dd0 <.text+0x233fdd0>
142340dc1: 49 83 c7 04                 	add	r15, 0x4
142340dc5: 4d 39 e7                    	cmp	r15, r12
142340dc8: 75 e6                       	jne	0x142340db0 <.text+0x233fdb0>
142340dca: e9 48 02 00 00              	jmp	0x142341017 <.text+0x2340017>
142340dcf: 90                          	nop
142340dd0: 4c 8d 04 c8                 	lea	r8, [rax + 8*rcx]
142340dd4: 48 8b 44 24 30              	mov	rax, qword ptr [rsp + 0x30]
142340dd9: 4c 8b 88 10 01 00 00        	mov	r9, qword ptr [rax + 0x110]
142340de0: 48 8b 88 18 01 00 00        	mov	rcx, qword ptr [rax + 0x118]
142340de7: 49 8b 40 18                 	mov	rax, qword ptr [r8 + 0x18]
142340deb: 49 8b 50 38                 	mov	rdx, qword ptr [r8 + 0x38]
142340def: 4e 8b 0c ca                 	mov	r9, qword ptr [rdx + 8*r9]
142340df3: 49 f7 d1                    	not	r9
142340df6: 4f 8d 0c 49                 	lea	r9, [r9 + 2*r9]
142340dfa: 49 c1 e1 04                 	shl	r9, 0x4
142340dfe: 4e 8b 74 08 10              	mov	r14, qword ptr [rax + r9 + 0x10]
142340e03: 4a 8b 7c 08 28              	mov	rdi, qword ptr [rax + r9 + 0x28]
142340e08: 49 83 c7 04                 	add	r15, 0x4
142340e0c: 49 39 48 40                 	cmp	qword ptr [r8 + 0x40], rcx
142340e10: 76 1e                       	jbe	0x142340e30 <.text+0x233fe30>
142340e12: 48 8b 0c ca                 	mov	rcx, qword ptr [rdx + 8*rcx]
142340e16: 48 85 c9                    	test	rcx, rcx
142340e19: 74 15                       	je	0x142340e30 <.text+0x233fe30>
142340e1b: 48 f7 d1                    	not	rcx
142340e1e: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
142340e22: 48 c1 e1 04                 	shl	rcx, 0x4
142340e26: 48 8b 6c 08 10              	mov	rbp, qword ptr [rax + rcx + 0x10]
142340e2b: eb 05                       	jmp	0x142340e32 <.text+0x233fe32>
142340e2d: 0f 1f 00                    	nop	dword ptr [rax]
142340e30: 31 ed                       	xor	ebp, ebp
142340e32: 31 f6                       	xor	esi, esi
142340e34: 89 f3                       	mov	ebx, esi
142340e36: 48 8d 04 9b                 	lea	rax, [rbx + 4*rbx]
142340e3a: 44 8b 44 85 00              	mov	r8d, dword ptr [rbp + 4*rax]
142340e3f: 44 8b 4c 85 04              	mov	r9d, dword ptr [rbp + 4*rax + 0x4]
142340e44: 48 8d 4c 24 50              	lea	rcx, [rsp + 0x50]
142340e49: 48 8b 54 24 40              	mov	rdx, qword ptr [rsp + 0x40]
142340e4e: e8 ad 6e ca fe              	call	0x140fe7d00 <.text+0xfe6d00>
142340e53: 83 7c 24 50 04              	cmp	dword ptr [rsp + 0x50], 0x4
142340e58: 0f 85 d5 01 00 00           	jne	0x142341033 <.text+0x2340033>
142340e5e: 48 ff c6                    	inc	rsi
142340e61: 48 89 d8                    	mov	rax, rbx
142340e64: 48 c1 e0 04                 	shl	rax, 0x4
142340e68: 48 8b 4c 24 58              	mov	rcx, qword ptr [rsp + 0x58]
142340e6d: f2 0f 10 01                 	movsd	xmm0, qword ptr [rcx]
142340e71: f2 0f 10 49 08              	movsd	xmm1, qword ptr [rcx + 0x8]
142340e76: 0f 28 d0                    	movaps	xmm2, xmm0
142340e79: 0f 59 d6                    	mulps	xmm2, xmm6
142340e7c: 0f 58 d1                    	addps	xmm2, xmm1
142340e7f: 41 0f 13 14 06              	movlps	qword ptr [r14 + rax], xmm2
142340e84: 8b 4c 24 2c                 	mov	ecx, dword ptr [rsp + 0x2c]
142340e88: 89 0c 9f                    	mov	dword ptr [rdi + 4*rbx], ecx
142340e8b: f2 41 0f 11 44 06 08        	movsd	qword ptr [r14 + rax + 0x8], xmm0
142340e92: 4c 39 ee                    	cmp	rsi, r13
142340e95: 0f 84 fa fe ff ff           	je	0x142340d95 <.text+0x233fd95>
142340e9b: eb 97                       	jmp	0x142340e34 <.text+0x233fe34>
142340e9d: bd 08 00 00 00              	mov	ebp, 0x8
142340ea2: 45 31 f6                    	xor	r14d, r14d
142340ea5: 0f 28 35 34 4a 5e 00        	movaps	xmm6, xmmword ptr [rip + 0x5e4a34] # 0x1429258e0
142340eac: 31 f6                       	xor	esi, esi
142340eae: 45 31 ed                    	xor	r13d, r13d
142340eb1: 31 db                       	xor	ebx, ebx
142340eb3: 4c 39 eb                    	cmp	rbx, r13
142340eb6: 0f 85 eb 00 00 00           	jne	0x142340fa7 <.text+0x233ffa7>
142340ebc: 4d 39 e7                    	cmp	r15, r12
142340ebf: 0f 84 52 01 00 00           	je	0x142341017 <.text+0x2340017>
142340ec5: 48 8b 44 24 38              	mov	rax, qword ptr [rsp + 0x38]
142340eca: 48 8b 80 00 01 00 00        	mov	rax, qword ptr [rax + 0x100]
142340ed1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
142340ee0: 41 8b 0f                    	mov	ecx, dword ptr [r15]
142340ee3: 48 8d 0c 89                 	lea	rcx, [rcx + 4*rcx]
142340ee7: 48 c1 e1 05                 	shl	rcx, 0x5
142340eeb: 4c 8b 6c 08 58              	mov	r13, qword ptr [rax + rcx + 0x58]
142340ef0: 4d 85 ed                    	test	r13, r13
142340ef3: 75 1b                       	jne	0x142340f10 <.text+0x233ff10>
142340ef5: 49 83 c7 04                 	add	r15, 0x4
142340ef9: 4d 39 e7                    	cmp	r15, r12
142340efc: 75 e2                       	jne	0x142340ee0 <.text+0x233fee0>
142340efe: e9 14 01 00 00              	jmp	0x142341017 <.text+0x2340017>
142340f03: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
142340f10: 48 01 c8                    	add	rax, rcx
142340f13: 8b 88 94 00 00 00           	mov	ecx, dword ptr [rax + 0x94]
142340f19: 48 8b 54 24 38              	mov	rdx, qword ptr [rsp + 0x38]
142340f1e: 4c 8b 8a a8 01 00 00        	mov	r9, qword ptr [rdx + 0x1a8]
142340f25: 4c 8d 14 c9                 	lea	r10, [rcx + 8*rcx]
142340f29: 48 8b 4c 24 30              	mov	rcx, qword ptr [rsp + 0x30]
142340f2e: 4c 8b 81 18 01 00 00        	mov	r8, qword ptr [rcx + 0x118]
142340f35: 4b 8b 4c d1 18              	mov	rcx, qword ptr [r9 + 8*r10 + 0x18]
142340f3a: 4b 8b 54 d1 38              	mov	rdx, qword ptr [r9 + 8*r10 + 0x38]
142340f3f: 4f 39 44 d1 40              	cmp	qword ptr [r9 + 8*r10 + 0x40], r8
142340f44: 76 2a                       	jbe	0x142340f70 <.text+0x233ff70>
142340f46: 4e 8b 04 c2                 	mov	r8, qword ptr [rdx + 8*r8]
142340f4a: 4d 85 c0                    	test	r8, r8
142340f4d: 74 21                       	je	0x142340f70 <.text+0x233ff70>
142340f4f: 49 f7 d0                    	not	r8
142340f52: 4f 8d 04 40                 	lea	r8, [r8 + 2*r8]
142340f56: 49 c1 e0 04                 	shl	r8, 0x4
142340f5a: 4e 8b 74 01 10              	mov	r14, qword ptr [rcx + r8 + 0x10]
142340f5f: eb 12                       	jmp	0x142340f73 <.text+0x233ff73>
142340f61: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
142340f70: 45 31 f6                    	xor	r14d, r14d
142340f73: 4c 8b 44 24 30              	mov	r8, qword ptr [rsp + 0x30]
142340f78: 4d 8b 80 10 01 00 00        	mov	r8, qword ptr [r8 + 0x110]
142340f7f: 4a 8b 14 c2                 	mov	rdx, qword ptr [rdx + 8*r8]
142340f83: 48 f7 d2                    	not	rdx
142340f86: 48 8d 14 52                 	lea	rdx, [rdx + 2*rdx]
142340f8a: 48 c1 e2 04                 	shl	rdx, 0x4
142340f8e: 48 8b 74 11 10              	mov	rsi, qword ptr [rcx + rdx + 0x10]
142340f93: 48 8b 4c 11 28              	mov	rcx, qword ptr [rcx + rdx + 0x28]
142340f98: 48 89 4c 24 48              	mov	qword ptr [rsp + 0x48], rcx
142340f9d: 49 83 c7 04                 	add	r15, 0x4
142340fa1: 48 8b 68 50                 	mov	rbp, qword ptr [rax + 0x50]
142340fa5: 31 db                       	xor	ebx, ebx
142340fa7: 48 89 d8                    	mov	rax, rbx
142340faa: 48 c1 e0 04                 	shl	rax, 0x4
142340fae: 8b 7c 05 08                 	mov	edi, dword ptr [rbp + rax + 0x8]
142340fb2: 48 8d 04 bf                 	lea	rax, [rdi + 4*rdi]
142340fb6: 45 8b 04 86                 	mov	r8d, dword ptr [r14 + 4*rax]
142340fba: 45 8b 4c 86 04              	mov	r9d, dword ptr [r14 + 4*rax + 0x4]
142340fbf: 48 8d 4c 24 50              	lea	rcx, [rsp + 0x50]
142340fc4: 48 8b 54 24 40              	mov	rdx, qword ptr [rsp + 0x40]
142340fc9: e8 32 6d ca fe              	call	0x140fe7d00 <.text+0xfe6d00>
142340fce: 83 7c 24 50 04              	cmp	dword ptr [rsp + 0x50], 0x4
142340fd3: 75 5e                       	jne	0x142341033 <.text+0x2340033>
142340fd5: 48 ff c3                    	inc	rbx
142340fd8: 48 89 f8                    	mov	rax, rdi
142340fdb: 48 c1 e0 04                 	shl	rax, 0x4
142340fdf: 48 8b 4c 24 58              	mov	rcx, qword ptr [rsp + 0x58]
142340fe4: f2 0f 10 01                 	movsd	xmm0, qword ptr [rcx]
142340fe8: f2 0f 10 49 08              	movsd	xmm1, qword ptr [rcx + 0x8]
142340fed: 0f 28 d0                    	movaps	xmm2, xmm0
142340ff0: 0f 59 d6                    	mulps	xmm2, xmm6
142340ff3: 0f 58 d1                    	addps	xmm2, xmm1
142340ff6: 0f 13 14 06                 	movlps	qword ptr [rsi + rax], xmm2
142340ffa: 8b 4c 24 2c                 	mov	ecx, dword ptr [rsp + 0x2c]
142340ffe: 48 8b 54 24 48              	mov	rdx, qword ptr [rsp + 0x48]
142341003: 89 0c ba                    	mov	dword ptr [rdx + 4*rdi], ecx
142341006: f2 0f 11 44 06 08           	movsd	qword ptr [rsi + rax + 0x8], xmm0
14234100c: 4c 39 eb                    	cmp	rbx, r13
14234100f: 0f 84 a7 fe ff ff           	je	0x142340ebc <.text+0x233febc>
142341015: eb 90                       	jmp	0x142340fa7 <.text+0x233ffa7>
142341017: 0f 28 b4 24 90 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x90]
14234101f: 48 81 c4 a8 00 00 00        	add	rsp, 0xa8
142341026: 5b                          	pop	rbx
142341027: 5d                          	pop	rbp
142341028: 5f                          	pop	rdi
142341029: 5e                          	pop	rsi
14234102a: 41 5c                       	pop	r12
14234102c: 41 5d                       	pop	r13
14234102e: 41 5e                       	pop	r14
142341030: 41 5f                       	pop	r15
142341032: c3                          	ret
142341033: 0f 10 44 24 50              	movups	xmm0, xmmword ptr [rsp + 0x50]
142341038: 0f 10 4c 24 60              	movups	xmm1, xmmword ptr [rsp + 0x60]
14234103d: 0f 29 8c 24 80 00 00 00     	movaps	xmmword ptr [rsp + 0x80], xmm1
142341045: 0f 29 44 24 70              	movaps	xmmword ptr [rsp + 0x70], xmm0
14234104a: 48 8d 05 2f d0 d0 00        	lea	rax, [rip + 0xd0d02f]   # 0x14304e080
142341051: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
142341056: 48 8d 0d ab ce d0 00        	lea	rcx, [rip + 0xd0ceab]   # 0x14304df08
14234105d: 4c 8d 0d 84 ce d0 00        	lea	r9, [rip + 0xd0ce84]    # 0x14304dee8
142341064: 4c 8d 44 24 70              	lea	r8, [rsp + 0x70]
142341069: ba 2b 00 00 00              	mov	edx, 0x2b
14234106e: e8 ed 86 59 00              	call	0x1428d9760 <.text+0x28d8760>
142341073: cc                          	int3
