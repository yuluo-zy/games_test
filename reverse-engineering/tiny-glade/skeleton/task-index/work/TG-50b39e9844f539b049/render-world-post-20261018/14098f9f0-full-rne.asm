
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14098f9f0: 55                          	push	rbp
14098f9f1: 41 57                       	push	r15
14098f9f3: 41 56                       	push	r14
14098f9f5: 41 55                       	push	r13
14098f9f7: 41 54                       	push	r12
14098f9f9: 56                          	push	rsi
14098f9fa: 57                          	push	rdi
14098f9fb: 53                          	push	rbx
14098f9fc: 48 81 ec 68 04 00 00        	sub	rsp, 0x468
14098fa03: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
14098fa0b: 48 c7 85 e0 03 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x3e0], -0x2
14098fa16: 4d 89 c7                    	mov	r15, r8
14098fa19: 48 89 d7                    	mov	rdi, rdx
14098fa1c: 48 89 cb                    	mov	rbx, rcx
14098fa1f: 48 83 ba 88 00 00 00 00     	cmp	qword ptr [rdx + 0x88], 0x0
14098fa27: 48 be ce f8 83 f9 3d 7e 7d e6       	movabs	rsi, -0x198281c2067c0732
14098fa31: 49 be ab 70 30 41 7f d8 5e c9       	movabs	r14, -0x36a12780becf8f55
14098fa3b: 0f 84 a6 00 00 00           	je	0x14098fae7 <.text+0x98eae7>
14098fa41: 48 8b 47 70                 	mov	rax, qword ptr [rdi + 0x70]
14098fa45: 48 8b 4f 78                 	mov	rcx, qword ptr [rdi + 0x78]
14098fa49: 48 89 ca                    	mov	rdx, rcx
14098fa4c: 48 21 f2                    	and	rdx, rsi
14098fa4f: 45 31 c0                    	xor	r8d, r8d
14098fa52: 66 0f 6f 05 96 1c fd 01     	movdqa	xmm0, xmmword ptr [rip + 0x1fd1c96] # 0x1429616f0
14098fa5a: 66 0f 76 c9                 	pcmpeqd	xmm1, xmm1
14098fa5e: f3 0f 6f 14 10              	movdqu	xmm2, xmmword ptr [rax + rdx]
14098fa63: 66 0f 6f da                 	movdqa	xmm3, xmm2
14098fa67: 66 0f 74 d8                 	pcmpeqb	xmm3, xmm0
14098fa6b: 66 44 0f d7 cb              	pmovmskb	r9d, xmm3
14098fa70: 45 85 c9                    	test	r9d, r9d
14098fa73: 75 38                       	jne	0x14098faad <.text+0x98eaad>
14098fa75: 66 0f 74 d1                 	pcmpeqb	xmm2, xmm1
14098fa79: 66 44 0f d7 ca              	pmovmskb	r9d, xmm2
14098fa7e: 45 85 c9                    	test	r9d, r9d
14098fa81: 75 64                       	jne	0x14098fae7 <.text+0x98eae7>
14098fa83: 4c 01 c2                    	add	rdx, r8
14098fa86: 48 83 c2 10                 	add	rdx, 0x10
14098fa8a: 49 83 c0 10                 	add	r8, 0x10
14098fa8e: 48 21 ca                    	and	rdx, rcx
14098fa91: eb cb                       	jmp	0x14098fa5e <.text+0x98ea5e>
14098fa93: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
14098faa0: 45 8d 51 ff                 	lea	r10d, [r9 - 0x1]
14098faa4: 66 45 21 ca                 	and	r10w, r9w
14098faa8: 45 89 d1                    	mov	r9d, r10d
14098faab: 74 c8                       	je	0x14098fa75 <.text+0x98ea75>
14098faad: f3 45 0f bc d1              	tzcnt	r10d, r9d
14098fab2: 49 01 d2                    	add	r10, rdx
14098fab5: 49 21 ca                    	and	r10, rcx
14098fab8: 49 f7 da                    	neg	r10
14098fabb: 4f 8d 14 52                 	lea	r10, [r10 + 2*r10]
14098fabf: 4e 39 74 d0 e8              	cmp	qword ptr [rax + 8*r10 - 0x18], r14
14098fac4: 75 da                       	jne	0x14098faa0 <.text+0x98eaa0>
14098fac6: 4a 39 74 d0 f0              	cmp	qword ptr [rax + 8*r10 - 0x10], rsi
14098facb: 75 d3                       	jne	0x14098faa0 <.text+0x98eaa0>
14098facd: 4a 8d 04 d0                 	lea	rax, [rax + 8*r10]
14098fad1: 48 8b 70 f8                 	mov	rsi, qword ptr [rax - 0x8]
14098fad5: 48 39 b7 18 02 00 00        	cmp	qword ptr [rdi + 0x218], rsi
14098fadc: 0f 87 0a 01 00 00           	ja	0x14098fbec <.text+0x98ebec>
14098fae2: e9 a2 0b 00 00              	jmp	0x140990689 <.text+0x98f689>
14098fae7: 8b 87 90 00 00 00           	mov	eax, dword ptr [rdi + 0x90]
14098faed: 3d fd ff ff 3f              	cmp	eax, 0x3ffffffd
14098faf2: 0f 87 56 0b 00 00           	ja	0x14099064e <.text+0x98f64e>
14098faf8: 8d 48 01                    	lea	ecx, [rax + 0x1]
14098fafb: f0                          	lock
14098fafc: 0f b1 8f 90 00 00 00        	cmpxchg	dword ptr [rdi + 0x90], ecx
14098fb03: 0f 85 45 0b 00 00           	jne	0x14099064e <.text+0x98f64e>
14098fb09: 0f b6 87 98 00 00 00        	movzx	eax, byte ptr [rdi + 0x98]
14098fb10: 48 83 bf f0 00 00 00 00     	cmp	qword ptr [rdi + 0xf0], 0x0
14098fb18: 0f 84 51 0b 00 00           	je	0x14099066f <.text+0x98f66f>
14098fb1e: 48 8b 87 d8 00 00 00        	mov	rax, qword ptr [rdi + 0xd8]
14098fb25: 48 8b 8f e0 00 00 00        	mov	rcx, qword ptr [rdi + 0xe0]
14098fb2c: 48 89 ca                    	mov	rdx, rcx
14098fb2f: 48 21 f2                    	and	rdx, rsi
14098fb32: 45 31 c0                    	xor	r8d, r8d
14098fb35: 66 0f 6f 05 b3 1b fd 01     	movdqa	xmm0, xmmword ptr [rip + 0x1fd1bb3] # 0x1429616f0
14098fb3d: 66 0f 76 c9                 	pcmpeqd	xmm1, xmm1
14098fb41: f3 0f 6f 14 10              	movdqu	xmm2, xmmword ptr [rax + rdx]
14098fb46: 66 0f 6f da                 	movdqa	xmm3, xmm2
14098fb4a: 66 0f 74 d8                 	pcmpeqb	xmm3, xmm0
14098fb4e: 66 44 0f d7 cb              	pmovmskb	r9d, xmm3
14098fb53: 45 85 c9                    	test	r9d, r9d
14098fb56: 75 35                       	jne	0x14098fb8d <.text+0x98eb8d>
14098fb58: 66 0f 74 d1                 	pcmpeqb	xmm2, xmm1
14098fb5c: 66 44 0f d7 ca              	pmovmskb	r9d, xmm2
14098fb61: 45 85 c9                    	test	r9d, r9d
14098fb64: 0f 85 05 0b 00 00           	jne	0x14099066f <.text+0x98f66f>
14098fb6a: 4c 01 c2                    	add	rdx, r8
14098fb6d: 48 83 c2 10                 	add	rdx, 0x10
14098fb71: 49 83 c0 10                 	add	r8, 0x10
14098fb75: 48 21 ca                    	and	rdx, rcx
14098fb78: eb c7                       	jmp	0x14098fb41 <.text+0x98eb41>
14098fb7a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
14098fb80: 45 8d 51 ff                 	lea	r10d, [r9 - 0x1]
14098fb84: 66 45 21 ca                 	and	r10w, r9w
14098fb88: 45 89 d1                    	mov	r9d, r10d
14098fb8b: 74 cb                       	je	0x14098fb58 <.text+0x98eb58>
14098fb8d: f3 45 0f bc d9              	tzcnt	r11d, r9d
14098fb92: 49 01 d3                    	add	r11, rdx
14098fb95: 49 21 cb                    	and	r11, rcx
14098fb98: 4d 89 da                    	mov	r10, r11
14098fb9b: 49 f7 da                    	neg	r10
14098fb9e: 49 c1 e3 03                 	shl	r11, 0x3
14098fba2: 49 c1 e2 07                 	shl	r10, 0x7
14098fba6: 4d 29 da                    	sub	r10, r11
14098fba9: 4e 39 b4 10 78 ff ff ff     	cmp	qword ptr [rax + r10 - 0x88], r14
14098fbb1: 75 cd                       	jne	0x14098fb80 <.text+0x98eb80>
14098fbb3: 4a 39 74 10 80              	cmp	qword ptr [rax + r10 - 0x80], rsi
14098fbb8: 75 c6                       	jne	0x14098fb80 <.text+0x98eb80>
14098fbba: 4c 01 d0                    	add	rax, r10
14098fbbd: 48 8b 70 f8                 	mov	rsi, qword ptr [rax - 0x8]
14098fbc1: ba ff ff ff ff              	mov	edx, 0xffffffff
14098fbc6: f0                          	lock
14098fbc7: 0f c1 97 90 00 00 00        	xadd	dword ptr [rdi + 0x90], edx
14098fbce: ff ca                       	dec	edx
14098fbd0: 89 d0                       	mov	eax, edx
14098fbd2: 25 ff ff ff bf              	and	eax, 0xbfffffff
14098fbd7: f7 d8                       	neg	eax
14098fbd9: 0f 80 b6 0a 00 00           	jo	0x140990695 <.text+0x98f695>
14098fbdf: 48 39 b7 18 02 00 00        	cmp	qword ptr [rdi + 0x218], rsi
14098fbe6: 0f 86 9d 0a 00 00           	jbe	0x140990689 <.text+0x98f689>
14098fbec: 48 8b 87 10 02 00 00        	mov	rax, qword ptr [rdi + 0x210]
14098fbf3: 48 8b 04 f0                 	mov	rax, qword ptr [rax + 8*rsi]
14098fbf7: 48 85 c0                    	test	rax, rax
14098fbfa: 0f 84 89 0a 00 00           	je	0x140990689 <.text+0x98f689>
14098fc00: 4c 8b 8f e0 01 00 00        	mov	r9, qword ptr [rdi + 0x1e0]
14098fc07: 48 f7 d0                    	not	rax
14098fc0a: 48 8d 04 40                 	lea	rax, [rax + 2*rax]
14098fc0e: 48 c1 e0 05                 	shl	rax, 0x5
14098fc12: 49 83 7c 01 20 00           	cmp	qword ptr [r9 + rax + 0x20], 0x0
14098fc18: 0f 84 6b 0a 00 00           	je	0x140990689 <.text+0x98f689>
14098fc1e: 49 01 c1                    	add	r9, rax
14098fc21: 4d 8b 41 10                 	mov	r8, qword ptr [r9 + 0x10]
14098fc25: 8b 97 b0 05 00 00           	mov	edx, dword ptr [rdi + 0x5b0]
14098fc2b: 41 8b 00                    	mov	eax, dword ptr [r8]
14098fc2e: 48 8d 0d bb a3 12 02        	lea	rcx, [rip + 0x212a3bb]  # 0x142ab9ff0
14098fc35: 48 63 04 81                 	movsxd	rax, dword ptr [rcx + 4*rax]
14098fc39: 48 01 c8                    	add	rax, rcx
14098fc3c: ff e0                       	jmp	rax
14098fc3e: 4c 89 8d d0 03 00 00        	mov	qword ptr [rbp + 0x3d0], r9
14098fc45: b0 01                       	mov	al, 0x1
14098fc47: eb 09                       	jmp	0x14098fc52 <.text+0x98ec52>
14098fc49: 4c 89 8d d0 03 00 00        	mov	qword ptr [rbp + 0x3d0], r9
14098fc50: 31 c0                       	xor	eax, eax
14098fc52: 49 89 de                    	mov	r14, rbx
14098fc55: 89 95 dc 03 00 00           	mov	dword ptr [rbp + 0x3dc], edx
14098fc5b: 4c 89 85 c8 03 00 00        	mov	qword ptr [rbp + 0x3c8], r8
14098fc62: 41 8b 48 04                 	mov	ecx, dword ptr [r8 + 0x4]
14098fc66: f7 d1                       	not	ecx
14098fc68: f6 c1 03                    	test	cl, 0x3
14098fc6b: 0f b6 c0                    	movzx	eax, al
14098fc6e: bb 02 00 00 00              	mov	ebx, 0x2
14098fc73: 0f 44 d8                    	cmove	ebx, eax
14098fc76: eb 19                       	jmp	0x14098fc91 <.text+0x98ec91>
14098fc78: 4c 89 8d d0 03 00 00        	mov	qword ptr [rbp + 0x3d0], r9
14098fc7f: 4c 89 85 c8 03 00 00        	mov	qword ptr [rbp + 0x3c8], r8
14098fc86: 49 89 de                    	mov	r14, rbx
14098fc89: 89 95 dc 03 00 00           	mov	dword ptr [rbp + 0x3dc], edx
14098fc8f: b3 02                       	mov	bl, 0x2
14098fc91: 49 8d 4f 04                 	lea	rcx, [r15 + 0x4]
14098fc95: e8 f6 5a 24 00              	call	0x140bd5790 <.text+0xbd4790>
14098fc9a: 4c 89 bd c0 03 00 00        	mov	qword ptr [rbp + 0x3c0], r15
14098fca1: 45 8b 47 0c                 	mov	r8d, dword ptr [r15 + 0xc]
14098fca5: 4c 8d bd d0 01 00 00        	lea	r15, [rbp + 0x1d0]
14098fcac: 4c 89 f9                    	mov	rcx, r15
14098fcaf: 48 89 c6                    	mov	rsi, rax
14098fcb2: 48 89 c2                    	mov	rdx, rax
14098fcb5: e8 e6 55 24 00              	call	0x140bd52a0 <.text+0xbd42a0>
14098fcba: 4c 8d 6d 40                 	lea	r13, [rbp + 0x40]
14098fcbe: 4c 89 e9                    	mov	rcx, r13
14098fcc1: 4c 89 fa                    	mov	rdx, r15
14098fcc4: e8 c7 56 24 00              	call	0x140bd5390 <.text+0xbd4390>
14098fcc9: 4c 8d a5 70 03 00 00        	lea	r12, [rbp + 0x370]
14098fcd0: 4c 89 e1                    	mov	rcx, r12
14098fcd3: 4c 89 ea                    	mov	rdx, r13
14098fcd6: e8 95 56 24 00              	call	0x140bd5370 <.text+0xbd4370>
14098fcdb: 4c 89 64 24 20              	mov	qword ptr [rsp + 0x20], r12
14098fce0: 4c 8d 05 89 a1 12 02        	lea	r8, [rip + 0x212a189]   # 0x142ab9e70
14098fce7: 48 8d 8d d0 01 00 00        	lea	rcx, [rbp + 0x1d0]
14098fcee: 41 b9 1b 00 00 00           	mov	r9d, 0x1b
14098fcf4: 4c 89 f2                    	mov	rdx, r14
14098fcf7: e8 94 cb fd ff              	call	0x14096c890 <.text+0x96b890>
14098fcfc: 80 bd f4 01 00 00 02        	cmp	byte ptr [rbp + 0x1f4], 0x2
14098fd03: 0f 84 c2 09 00 00           	je	0x1409906cb <.text+0x98f6cb>
14098fd09: 48 8b 85 f0 01 00 00        	mov	rax, qword ptr [rbp + 0x1f0]
14098fd10: 48 89 45 30                 	mov	qword ptr [rbp + 0x30], rax
14098fd14: f3 0f 6f 85 d0 01 00 00     	movdqu	xmm0, xmmword ptr [rbp + 0x1d0]
14098fd1c: f3 0f 6f 8d e0 01 00 00     	movdqu	xmm1, xmmword ptr [rbp + 0x1e0]
14098fd24: 66 0f 7f 4d 20              	movdqa	xmmword ptr [rbp + 0x20], xmm1
14098fd29: 66 0f 7f 45 10              	movdqa	xmmword ptr [rbp + 0x10], xmm0
14098fd2e: 80 fb 02                    	cmp	bl, 0x2
14098fd31: 75 3b                       	jne	0x14098fd6e <.text+0x98ed6e>
14098fd33: 44 8b 95 dc 03 00 00        	mov	r10d, dword ptr [rbp + 0x3dc]
14098fd3a: 48 8b b5 d0 03 00 00        	mov	rsi, qword ptr [rbp + 0x3d0]
14098fd41: 44 89 56 5c                 	mov	dword ptr [rsi + 0x5c], r10d
14098fd45: 4c 8b 9d c8 03 00 00        	mov	r11, qword ptr [rbp + 0x3c8]
14098fd4c: 41 8b 0b                    	mov	ecx, dword ptr [r11]
14098fd4f: 31 c0                       	xor	eax, eax
14098fd51: ba 02 00 00 00              	mov	edx, 0x2
14098fd56: 4c 8d 05 a7 a2 12 02        	lea	r8, [rip + 0x212a2a7]   # 0x142aba004
14098fd5d: 4d 63 0c 88                 	movsxd	r9, dword ptr [r8 + 4*rcx]
14098fd61: 4d 01 c1                    	add	r9, r8
14098fd64: 4c 8b a5 c0 03 00 00        	mov	r12, qword ptr [rbp + 0x3c0]
14098fd6b: 41 ff e1                    	jmp	r9
14098fd6e: 4c 89 b5 68 03 00 00        	mov	qword ptr [rbp + 0x368], r14
14098fd75: 4c 8d ad d0 01 00 00        	lea	r13, [rbp + 0x1d0]
14098fd7c: 4c 89 e9                    	mov	rcx, r13
14098fd7f: 48 89 f2                    	mov	rdx, rsi
14098fd82: 41 b8 7a 00 00 00           	mov	r8d, 0x7a
14098fd88: e8 13 55 24 00              	call	0x140bd52a0 <.text+0xbd42a0>
14098fd8d: 4c 8d 7d 40                 	lea	r15, [rbp + 0x40]
14098fd91: 4c 89 f9                    	mov	rcx, r15
14098fd94: 4c 89 ea                    	mov	rdx, r13
14098fd97: 4c 8b ad 68 03 00 00        	mov	r13, qword ptr [rbp + 0x368]
14098fd9e: e8 ed 55 24 00              	call	0x140bd5390 <.text+0xbd4390>
14098fda3: 4c 89 e1                    	mov	rcx, r12
14098fda6: 4c 89 fa                    	mov	rdx, r15
14098fda9: e8 c2 55 24 00              	call	0x140bd5370 <.text+0xbd4370>
14098fdae: 4c 89 64 24 20              	mov	qword ptr [rsp + 0x20], r12
14098fdb3: 4c 8d 05 ee a0 12 02        	lea	r8, [rip + 0x212a0ee]   # 0x142ab9ea8
14098fdba: 48 8d 8d d0 01 00 00        	lea	rcx, [rbp + 0x1d0]
14098fdc1: 41 b9 16 00 00 00           	mov	r9d, 0x16
14098fdc7: 4c 89 ea                    	mov	rdx, r13
14098fdca: e8 c1 ca fd ff              	call	0x14096c890 <.text+0x96b890>
14098fdcf: 80 bd f4 01 00 00 02        	cmp	byte ptr [rbp + 0x1f4], 0x2
14098fdd6: 0f 84 24 09 00 00           	je	0x140990700 <.text+0x98f700>
14098fddc: 48 8b 85 f0 01 00 00        	mov	rax, qword ptr [rbp + 0x1f0]
14098fde3: 48 89 45 00                 	mov	qword ptr [rbp], rax
14098fde7: 0f 10 85 d0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1d0]
14098fdee: 0f 10 8d e0 01 00 00        	movups	xmm1, xmmword ptr [rbp + 0x1e0]
14098fdf5: 0f 29 4d f0                 	movaps	xmmword ptr [rbp - 0x10], xmm1
14098fdf9: 0f 29 45 e0                 	movaps	xmmword ptr [rbp - 0x20], xmm0
14098fdfd: f6 c3 01                    	test	bl, 0x1
14098fe00: 0f 84 6d 05 00 00           	je	0x140990373 <.text+0x98f373>
14098fe06: 48 8d 8d 70 03 00 00        	lea	rcx, [rbp + 0x370]
14098fe0d: 48 89 f2                    	mov	rdx, rsi
14098fe10: 41 b8 7a 00 00 00           	mov	r8d, 0x7a
14098fe16: e8 85 54 24 00              	call	0x140bd52a0 <.text+0xbd42a0>
14098fe1b: 0f 10 85 70 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x370]
14098fe22: 0f 10 8d 80 03 00 00        	movups	xmm1, xmmword ptr [rbp + 0x380]
14098fe29: 0f 29 4d 50                 	movaps	xmmword ptr [rbp + 0x50], xmm1
14098fe2d: 0f 29 45 40                 	movaps	xmmword ptr [rbp + 0x40], xmm0
14098fe31: 48 8d 8d e0 01 00 00        	lea	rcx, [rbp + 0x1e0]
14098fe38: 4c 89 fa                    	mov	rdx, r15
14098fe3b: e8 a0 6e 24 00              	call	0x140bd6ce0 <.text+0xbd5ce0>
14098fe40: 48 8d 05 ce a0 12 02        	lea	rax, [rip + 0x212a0ce]  # 0x142ab9f15
14098fe47: 48 89 85 d0 01 00 00        	mov	qword ptr [rbp + 0x1d0], rax
14098fe4e: 48 c7 85 d8 01 00 00 1a 00 00 00    	mov	qword ptr [rbp + 0x1d8], 0x1a
14098fe59: 48 8d 95 d0 01 00 00        	lea	rdx, [rbp + 0x1d0]
14098fe60: 4c 89 e9                    	mov	rcx, r13
14098fe63: e8 68 96 25 00              	call	0x140be94d0 <.text+0xbe84d0>
14098fe68: 0f 10 85 70 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x370]
14098fe6f: 0f 10 8d 80 03 00 00        	movups	xmm1, xmmword ptr [rbp + 0x380]
14098fe76: 0f 11 45 c0                 	movups	xmmword ptr [rbp - 0x40], xmm0
14098fe7a: 0f 11 4d d0                 	movups	xmmword ptr [rbp - 0x30], xmm1
14098fe7e: 89 45 bc                    	mov	dword ptr [rbp - 0x44], eax
14098fe81: 4c 89 6c 24 30              	mov	qword ptr [rsp + 0x30], r13
14098fe86: 48 89 7c 24 28              	mov	qword ptr [rsp + 0x28], rdi
14098fe8b: 48 c7 44 24 20 23 00 00 00  	mov	qword ptr [rsp + 0x20], 0x23
14098fe94: 48 8d 15 3d a0 12 02        	lea	rdx, [rip + 0x212a03d]  # 0x142ab9ed8
14098fe9b: 4c 8d 0d 50 a0 12 02        	lea	r9, [rip + 0x212a050]   # 0x142ab9ef2
14098fea2: 4c 8d a5 d0 01 00 00        	lea	r12, [rbp + 0x1d0]
14098fea9: 41 b8 1a 00 00 00           	mov	r8d, 0x1a
14098feaf: 4c 89 e1                    	mov	rcx, r12
14098feb2: e8 e9 33 c7 00              	call	0x1416032a0 <.text+0x16022a0>
14098feb7: 48 8b 85 c0 03 00 00        	mov	rax, qword ptr [rbp + 0x3c0]
14098febe: 48 89 85 70 03 00 00        	mov	qword ptr [rbp + 0x370], rax
14098fec5: c7 45 40 02 00 00 00        	mov	dword ptr [rbp + 0x40], 0x2
14098fecc: 4c 89 7c 24 20              	mov	qword ptr [rsp + 0x20], r15
14098fed1: 48 8d 8d 70 03 00 00        	lea	rcx, [rbp + 0x370]
14098fed8: 49 b8 62 69 3c 5f 14 12 e3 0c       	movabs	r8, 0xce312145f3c6962
14098fee2: 4c 89 e2                    	mov	rdx, r12
14098fee5: 41 b1 19                    	mov	r9b, 0x19
14098fee8: e8 53 e3 f5 ff              	call	0x1408ee240 <.text+0x8ed240>
14098feed: 41 b8 90 01 00 00           	mov	r8d, 0x190
14098fef3: 4c 89 f9                    	mov	rcx, r15
14098fef6: 4c 89 e2                    	mov	rdx, r12
14098fef9: e8 02 3d f9 01              	call	0x142923c00 <.text+0x2922c00>
14098fefe: 4c 8d 6d bc                 	lea	r13, [rbp - 0x44]
14098ff02: 4c 89 ad 70 03 00 00        	mov	qword ptr [rbp + 0x370], r13
14098ff09: c7 85 d0 01 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x1d0], 0x2
14098ff13: 4c 89 64 24 20              	mov	qword ptr [rsp + 0x20], r12
14098ff18: 48 8d 8d 70 03 00 00        	lea	rcx, [rbp + 0x370]
14098ff1f: 49 b8 6c 33 5b 07 66 5d 80 ef       	movabs	r8, -0x107fa299f8a4cc94
14098ff29: 4c 89 fa                    	mov	rdx, r15
14098ff2c: 41 b1 32                    	mov	r9b, 0x32
14098ff2f: e8 cc dd f5 ff              	call	0x1408edd00 <.text+0x8ecd00>
14098ff34: 48 8d 8d d0 01 00 00        	lea	rcx, [rbp + 0x1d0]
14098ff3b: 48 8d 55 40                 	lea	rdx, [rbp + 0x40]
14098ff3f: 41 b8 90 01 00 00           	mov	r8d, 0x190
14098ff45: e8 b6 3c f9 01              	call	0x142923c00 <.text+0x2922c00>
14098ff4a: 49 89 f4                    	mov	r12, rsi
14098ff4d: 49 c1 ec 20                 	shr	r12, 0x20
14098ff51: 0f 10 85 70 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x270]
14098ff58: 0f 29 85 b0 03 00 00        	movaps	xmmword ptr [rbp + 0x3b0], xmm0
14098ff5f: 0f 10 85 60 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x260]
14098ff66: 0f 29 85 a0 03 00 00        	movaps	xmmword ptr [rbp + 0x3a0], xmm0
14098ff6d: 0f 10 85 50 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x250]
14098ff74: 0f 29 85 90 03 00 00        	movaps	xmmword ptr [rbp + 0x390], xmm0
14098ff7b: 0f 10 85 40 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x240]
14098ff82: 0f 29 85 80 03 00 00        	movaps	xmmword ptr [rbp + 0x380], xmm0
14098ff89: 0f 10 85 30 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x230]
14098ff90: 0f 29 85 70 03 00 00        	movaps	xmmword ptr [rbp + 0x370], xmm0
14098ff97: 48 8b 85 08 03 00 00        	mov	rax, qword ptr [rbp + 0x308]
14098ff9e: 48 8b 8d 50 03 00 00        	mov	rcx, qword ptr [rbp + 0x350]
14098ffa5: 48 8b 95 e0 01 00 00        	mov	rdx, qword ptr [rbp + 0x1e0]
14098ffac: 48 89 55 50                 	mov	qword ptr [rbp + 0x50], rdx
14098ffb0: 0f 10 85 d0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1d0]
14098ffb7: 0f 29 45 40                 	movaps	xmmword ptr [rbp + 0x40], xmm0
14098ffbb: 0f 10 85 e8 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1e8]
14098ffc2: 0f 29 45 70                 	movaps	xmmword ptr [rbp + 0x70], xmm0
14098ffc6: 48 8b 95 f8 01 00 00        	mov	rdx, qword ptr [rbp + 0x1f8]
14098ffcd: 48 89 95 80 00 00 00        	mov	qword ptr [rbp + 0x80], rdx
14098ffd4: 0f 10 85 00 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x200]
14098ffdb: 0f 11 45 58                 	movups	xmmword ptr [rbp + 0x58], xmm0
14098ffdf: 48 8b 95 10 02 00 00        	mov	rdx, qword ptr [rbp + 0x210]
14098ffe6: 48 89 55 68                 	mov	qword ptr [rbp + 0x68], rdx
14098ffea: 0f 10 85 80 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x280]
14098fff1: 0f 10 8d 90 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x290]
14098fff8: 0f 10 95 a0 02 00 00        	movups	xmm2, xmmword ptr [rbp + 0x2a0]
14098ffff: 0f 10 9d b0 02 00 00        	movups	xmm3, xmmword ptr [rbp + 0x2b0]
140990006: 0f 11 85 b8 00 00 00        	movups	xmmword ptr [rbp + 0xb8], xmm0
14099000d: 0f 11 8d c8 00 00 00        	movups	xmmword ptr [rbp + 0xc8], xmm1
140990014: 0f 11 95 d8 00 00 00        	movups	xmmword ptr [rbp + 0xd8], xmm2
14099001b: 0f 11 9d e8 00 00 00        	movups	xmmword ptr [rbp + 0xe8], xmm3
140990022: 0f 10 85 c0 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2c0]
140990029: 0f 11 85 f8 00 00 00        	movups	xmmword ptr [rbp + 0xf8], xmm0
140990030: 0f 10 85 d0 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2d0]
140990037: 0f 11 85 08 01 00 00        	movups	xmmword ptr [rbp + 0x108], xmm0
14099003e: 48 8b 95 28 02 00 00        	mov	rdx, qword ptr [rbp + 0x228]
140990045: 48 89 95 98 00 00 00        	mov	qword ptr [rbp + 0x98], rdx
14099004c: 0f 10 85 18 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x218]
140990053: 0f 11 85 88 00 00 00        	movups	xmmword ptr [rbp + 0x88], xmm0
14099005a: 48 89 8d 20 01 00 00        	mov	qword ptr [rbp + 0x120], rcx
140990061: 48 89 85 18 01 00 00        	mov	qword ptr [rbp + 0x118], rax
140990068: c7 85 a0 00 00 00 01 00 00 00       	mov	dword ptr [rbp + 0xa0], 0x1
140990072: 89 b5 a4 00 00 00           	mov	dword ptr [rbp + 0xa4], esi
140990078: 44 89 a5 a8 00 00 00        	mov	dword ptr [rbp + 0xa8], r12d
14099007f: c7 85 ac 00 00 00 01 00 00 00       	mov	dword ptr [rbp + 0xac], 0x1
140990089: 48 8d 8d 70 03 00 00        	lea	rcx, [rbp + 0x370]
140990090: 4c 89 fa                    	mov	rdx, r15
140990093: e8 38 d4 04 00              	call	0x1409dd4d0 <.text+0x9dc4d0>
140990098: 80 bd 06 03 00 00 08        	cmp	byte ptr [rbp + 0x306], 0x8
14099009f: 74 0c                       	je	0x1409900ad <.text+0x98f0ad>
1409900a1: 48 8d 8d f0 02 00 00        	lea	rcx, [rbp + 0x2f0]
1409900a8: e8 03 24 25 00              	call	0x140be24b0 <.text+0xbe14b0>
1409900ad: 48 8b 85 68 03 00 00        	mov	rax, qword ptr [rbp + 0x368]
1409900b4: 48 89 44 24 30              	mov	qword ptr [rsp + 0x30], rax
1409900b9: 48 89 7c 24 28              	mov	qword ptr [rsp + 0x28], rdi
1409900be: 48 c7 44 24 20 23 00 00 00  	mov	qword ptr [rsp + 0x20], 0x23
1409900c7: 48 8d 15 61 9e 12 02        	lea	rdx, [rip + 0x2129e61]  # 0x142ab9f2f
1409900ce: 4c 8d 0d 74 9e 12 02        	lea	r9, [rip + 0x2129e74]   # 0x142ab9f49
1409900d5: 48 8d bd d0 01 00 00        	lea	rdi, [rbp + 0x1d0]
1409900dc: 41 b8 1a 00 00 00           	mov	r8d, 0x1a
1409900e2: 48 89 f9                    	mov	rcx, rdi
1409900e5: e8 b6 31 c7 00              	call	0x1416032a0 <.text+0x16022a0>
1409900ea: 4c 89 ad 70 03 00 00        	mov	qword ptr [rbp + 0x370], r13
1409900f1: c7 45 40 02 00 00 00        	mov	dword ptr [rbp + 0x40], 0x2
1409900f8: 4c 89 7c 24 20              	mov	qword ptr [rsp + 0x20], r15
1409900fd: 48 8d 8d 70 03 00 00        	lea	rcx, [rbp + 0x370]
140990104: 49 b8 62 69 3c 5f 14 12 e3 0c       	movabs	r8, 0xce312145f3c6962
14099010e: 48 89 fa                    	mov	rdx, rdi
140990111: 41 b1 19                    	mov	r9b, 0x19
140990114: e8 27 e1 f5 ff              	call	0x1408ee240 <.text+0x8ed240>
140990119: 48 8d 7d 40                 	lea	rdi, [rbp + 0x40]
14099011d: 48 8d 9d d0 01 00 00        	lea	rbx, [rbp + 0x1d0]
140990124: 41 b8 90 01 00 00           	mov	r8d, 0x190
14099012a: 48 89 f9                    	mov	rcx, rdi
14099012d: 48 89 da                    	mov	rdx, rbx
140990130: e8 cb 3a f9 01              	call	0x142923c00 <.text+0x2922c00>
140990135: 48 8d 45 e0                 	lea	rax, [rbp - 0x20]
140990139: 48 89 85 70 03 00 00        	mov	qword ptr [rbp + 0x370], rax
140990140: c7 85 d0 01 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x1d0], 0x2
14099014a: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
14099014f: 48 8d 8d 70 03 00 00        	lea	rcx, [rbp + 0x370]
140990156: 49 b8 6c 33 5b 07 66 5d 80 ef       	movabs	r8, -0x107fa299f8a4cc94
140990160: 48 89 fa                    	mov	rdx, rdi
140990163: 41 b1 32                    	mov	r9b, 0x32
140990166: e8 95 db f5 ff              	call	0x1408edd00 <.text+0x8ecd00>
14099016b: 48 8d bd d0 01 00 00        	lea	rdi, [rbp + 0x1d0]
140990172: 48 8d 5d 40                 	lea	rbx, [rbp + 0x40]
140990176: 41 b8 90 01 00 00           	mov	r8d, 0x190
14099017c: 48 89 f9                    	mov	rcx, rdi
14099017f: 48 89 da                    	mov	rdx, rbx
140990182: e8 79 3a f9 01              	call	0x142923c00 <.text+0x2922c00>
140990187: 48 8d 45 10                 	lea	rax, [rbp + 0x10]
14099018b: 48 89 85 70 03 00 00        	mov	qword ptr [rbp + 0x370], rax
140990192: c7 45 40 02 00 00 00        	mov	dword ptr [rbp + 0x40], 0x2
140990199: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
14099019e: 48 8d 8d 70 03 00 00        	lea	rcx, [rbp + 0x370]
1409901a5: 49 b8 d4 5d a8 e7 5d 69 39 2f       	movabs	r8, 0x2f39695de7a85dd4
1409901af: 48 89 fa                    	mov	rdx, rdi
1409901b2: 41 b1 32                    	mov	r9b, 0x32
1409901b5: e8 46 db f5 ff              	call	0x1408edd00 <.text+0x8ecd00>
1409901ba: 48 8d 4d 40                 	lea	rcx, [rbp + 0x40]
1409901be: 48 8d 95 d0 01 00 00        	lea	rdx, [rbp + 0x1d0]
1409901c5: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409901cb: e8 30 3a f9 01              	call	0x142923c00 <.text+0x2922c00>
1409901d0: 0f 10 85 e0 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xe0]
1409901d7: 0f 29 85 b0 03 00 00        	movaps	xmmword ptr [rbp + 0x3b0], xmm0
1409901de: 0f 10 85 d0 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xd0]
1409901e5: 0f 29 85 a0 03 00 00        	movaps	xmmword ptr [rbp + 0x3a0], xmm0
1409901ec: 0f 10 85 c0 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xc0]
1409901f3: 0f 29 85 90 03 00 00        	movaps	xmmword ptr [rbp + 0x390], xmm0
1409901fa: 0f 10 85 b0 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xb0]
140990201: 0f 29 85 80 03 00 00        	movaps	xmmword ptr [rbp + 0x380], xmm0
140990208: 0f 10 85 a0 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xa0]
14099020f: 0f 29 85 70 03 00 00        	movaps	xmmword ptr [rbp + 0x370], xmm0
140990216: 48 8b 85 78 01 00 00        	mov	rax, qword ptr [rbp + 0x178]
14099021d: 48 8b 8d c0 01 00 00        	mov	rcx, qword ptr [rbp + 0x1c0]
140990224: 48 8b 55 50                 	mov	rdx, qword ptr [rbp + 0x50]
140990228: 48 89 95 e0 01 00 00        	mov	qword ptr [rbp + 0x1e0], rdx
14099022f: 0f 10 45 40                 	movups	xmm0, xmmword ptr [rbp + 0x40]
140990233: 0f 29 85 d0 01 00 00        	movaps	xmmword ptr [rbp + 0x1d0], xmm0
14099023a: 0f 10 45 58                 	movups	xmm0, xmmword ptr [rbp + 0x58]
14099023e: 0f 29 85 00 02 00 00        	movaps	xmmword ptr [rbp + 0x200], xmm0
140990245: 48 8b 55 68                 	mov	rdx, qword ptr [rbp + 0x68]
140990249: 48 89 95 10 02 00 00        	mov	qword ptr [rbp + 0x210], rdx
140990250: 0f 10 45 70                 	movups	xmm0, xmmword ptr [rbp + 0x70]
140990254: 0f 11 85 e8 01 00 00        	movups	xmmword ptr [rbp + 0x1e8], xmm0
14099025b: 48 8b 95 80 00 00 00        	mov	rdx, qword ptr [rbp + 0x80]
140990262: 48 89 95 f8 01 00 00        	mov	qword ptr [rbp + 0x1f8], rdx
140990269: 0f 10 85 f0 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xf0]
140990270: f3 0f 6f 8d 00 01 00 00     	movdqu	xmm1, xmmword ptr [rbp + 0x100]
140990278: f3 0f 6f 95 10 01 00 00     	movdqu	xmm2, xmmword ptr [rbp + 0x110]
140990280: f3 0f 6f 9d 20 01 00 00     	movdqu	xmm3, xmmword ptr [rbp + 0x120]
140990288: 0f 11 85 48 02 00 00        	movups	xmmword ptr [rbp + 0x248], xmm0
14099028f: f3 0f 7f 8d 58 02 00 00     	movdqu	xmmword ptr [rbp + 0x258], xmm1
140990297: f3 0f 7f 95 68 02 00 00     	movdqu	xmmword ptr [rbp + 0x268], xmm2
14099029f: f3 0f 7f 9d 78 02 00 00     	movdqu	xmmword ptr [rbp + 0x278], xmm3
1409902a7: 0f 10 85 30 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x130]
1409902ae: 0f 11 85 88 02 00 00        	movups	xmmword ptr [rbp + 0x288], xmm0
1409902b5: 0f 10 85 40 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x140]
1409902bc: 0f 11 85 98 02 00 00        	movups	xmmword ptr [rbp + 0x298], xmm0
1409902c3: 48 8b 95 98 00 00 00        	mov	rdx, qword ptr [rbp + 0x98]
1409902ca: 48 89 95 28 02 00 00        	mov	qword ptr [rbp + 0x228], rdx
1409902d1: f3 0f 6f 85 88 00 00 00     	movdqu	xmm0, xmmword ptr [rbp + 0x88]
1409902d9: f3 0f 7f 85 18 02 00 00     	movdqu	xmmword ptr [rbp + 0x218], xmm0
1409902e1: 48 89 8d b0 02 00 00        	mov	qword ptr [rbp + 0x2b0], rcx
1409902e8: 48 89 85 a8 02 00 00        	mov	qword ptr [rbp + 0x2a8], rax
1409902ef: c7 85 30 02 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x230], 0x1
1409902f9: 89 b5 34 02 00 00           	mov	dword ptr [rbp + 0x234], esi
1409902ff: 44 89 a5 38 02 00 00        	mov	dword ptr [rbp + 0x238], r12d
140990306: c7 85 3c 02 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x23c], 0x1
140990310: 48 8d 8d 70 03 00 00        	lea	rcx, [rbp + 0x370]
140990317: 48 8d 95 d0 01 00 00        	lea	rdx, [rbp + 0x1d0]
14099031e: e8 ad d1 04 00              	call	0x1409dd4d0 <.text+0x9dc4d0>
140990323: 80 bd 76 01 00 00 08        	cmp	byte ptr [rbp + 0x176], 0x8
14099032a: 4c 8b a5 c0 03 00 00        	mov	r12, qword ptr [rbp + 0x3c0]
140990331: 48 8b b5 d0 03 00 00        	mov	rsi, qword ptr [rbp + 0x3d0]
140990338: 74 0c                       	je	0x140990346 <.text+0x98f346>
14099033a: 48 8d 8d 60 01 00 00        	lea	rcx, [rbp + 0x160]
140990341: e8 6a 21 25 00              	call	0x140be24b0 <.text+0xbe14b0>
140990346: 44 8b 95 dc 03 00 00        	mov	r10d, dword ptr [rbp + 0x3dc]
14099034d: 44 89 56 5c                 	mov	dword ptr [rsi + 0x5c], r10d
140990351: 4c 8b 9d c8 03 00 00        	mov	r11, qword ptr [rbp + 0x3c8]
140990358: 41 8b 0b                    	mov	ecx, dword ptr [r11]
14099035b: 31 c0                       	xor	eax, eax
14099035d: ba 02 00 00 00              	mov	edx, 0x2
140990362: 4c 8d 05 9b 9c 12 02        	lea	r8, [rip + 0x2129c9b]   # 0x142aba004
140990369: 4d 63 0c 88                 	movsxd	r9, dword ptr [r8 + 4*rcx]
14099036d: 4d 01 c1                    	add	r9, r8
140990370: 41 ff e1                    	jmp	r9
140990373: 4c 89 6c 24 30              	mov	qword ptr [rsp + 0x30], r13
140990378: 48 89 7c 24 28              	mov	qword ptr [rsp + 0x28], rdi
14099037d: 48 c7 44 24 20 23 00 00 00  	mov	qword ptr [rsp + 0x20], 0x23
140990386: 48 8d 15 4b 9b 12 02        	lea	rdx, [rip + 0x2129b4b]  # 0x142ab9ed8
14099038d: 4c 8d 0d 5e 9b 12 02        	lea	r9, [rip + 0x2129b5e]   # 0x142ab9ef2
140990394: 48 8d bd d0 01 00 00        	lea	rdi, [rbp + 0x1d0]
14099039b: 41 b8 1a 00 00 00           	mov	r8d, 0x1a
1409903a1: 48 89 f9                    	mov	rcx, rdi
1409903a4: e8 f7 2e c7 00              	call	0x1416032a0 <.text+0x16022a0>
1409903a9: 4c 8b a5 c0 03 00 00        	mov	r12, qword ptr [rbp + 0x3c0]
1409903b0: 4c 89 a5 70 03 00 00        	mov	qword ptr [rbp + 0x370], r12
1409903b7: c7 45 40 02 00 00 00        	mov	dword ptr [rbp + 0x40], 0x2
1409903be: 4c 89 7c 24 20              	mov	qword ptr [rsp + 0x20], r15
1409903c3: 48 8d 8d 70 03 00 00        	lea	rcx, [rbp + 0x370]
1409903ca: 49 b8 62 69 3c 5f 14 12 e3 0c       	movabs	r8, 0xce312145f3c6962
1409903d4: 48 89 fa                    	mov	rdx, rdi
1409903d7: 41 b1 19                    	mov	r9b, 0x19
1409903da: e8 61 de f5 ff              	call	0x1408ee240 <.text+0x8ed240>
1409903df: 48 8d 7d 40                 	lea	rdi, [rbp + 0x40]
1409903e3: 48 8d 9d d0 01 00 00        	lea	rbx, [rbp + 0x1d0]
1409903ea: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409903f0: 48 89 f9                    	mov	rcx, rdi
1409903f3: 48 89 da                    	mov	rdx, rbx
1409903f6: e8 05 38 f9 01              	call	0x142923c00 <.text+0x2922c00>
1409903fb: 48 8d 45 e0                 	lea	rax, [rbp - 0x20]
1409903ff: 48 89 85 70 03 00 00        	mov	qword ptr [rbp + 0x370], rax
140990406: c7 85 d0 01 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x1d0], 0x2
140990410: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
140990415: 48 8d 8d 70 03 00 00        	lea	rcx, [rbp + 0x370]
14099041c: 49 b8 6c 33 5b 07 66 5d 80 ef       	movabs	r8, -0x107fa299f8a4cc94
140990426: 48 89 fa                    	mov	rdx, rdi
140990429: 41 b1 32                    	mov	r9b, 0x32
14099042c: e8 cf d8 f5 ff              	call	0x1408edd00 <.text+0x8ecd00>
140990431: 48 8d 8d d0 01 00 00        	lea	rcx, [rbp + 0x1d0]
140990438: 48 8d 55 40                 	lea	rdx, [rbp + 0x40]
14099043c: 41 b8 90 01 00 00           	mov	r8d, 0x190
140990442: e8 b9 37 f9 01              	call	0x142923c00 <.text+0x2922c00>
140990447: 48 89 f0                    	mov	rax, rsi
14099044a: 48 c1 e8 20                 	shr	rax, 0x20
14099044e: 0f 10 85 70 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x270]
140990455: 0f 29 85 b0 03 00 00        	movaps	xmmword ptr [rbp + 0x3b0], xmm0
14099045c: 0f 10 85 60 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x260]
140990463: 0f 29 85 a0 03 00 00        	movaps	xmmword ptr [rbp + 0x3a0], xmm0
14099046a: 0f 10 85 50 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x250]
140990471: 0f 29 85 90 03 00 00        	movaps	xmmword ptr [rbp + 0x390], xmm0
140990478: 0f 10 85 40 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x240]
14099047f: 0f 29 85 80 03 00 00        	movaps	xmmword ptr [rbp + 0x380], xmm0
140990486: 0f 10 85 30 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x230]
14099048d: 0f 29 85 70 03 00 00        	movaps	xmmword ptr [rbp + 0x370], xmm0
140990494: 48 8b 8d 08 03 00 00        	mov	rcx, qword ptr [rbp + 0x308]
14099049b: 48 8b 95 50 03 00 00        	mov	rdx, qword ptr [rbp + 0x350]
1409904a2: 4c 8b 85 e0 01 00 00        	mov	r8, qword ptr [rbp + 0x1e0]
1409904a9: 4c 89 45 50                 	mov	qword ptr [rbp + 0x50], r8
1409904ad: 0f 10 85 d0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1d0]
1409904b4: 0f 29 45 40                 	movaps	xmmword ptr [rbp + 0x40], xmm0
1409904b8: 0f 10 85 e8 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1e8]
1409904bf: 0f 29 45 70                 	movaps	xmmword ptr [rbp + 0x70], xmm0
1409904c3: 4c 8b 85 f8 01 00 00        	mov	r8, qword ptr [rbp + 0x1f8]
1409904ca: 4c 89 85 80 00 00 00        	mov	qword ptr [rbp + 0x80], r8
1409904d1: 0f 10 85 00 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x200]
1409904d8: 0f 11 45 58                 	movups	xmmword ptr [rbp + 0x58], xmm0
1409904dc: 4c 8b 85 10 02 00 00        	mov	r8, qword ptr [rbp + 0x210]
1409904e3: 4c 89 45 68                 	mov	qword ptr [rbp + 0x68], r8
1409904e7: 0f 10 85 80 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x280]
1409904ee: f3 0f 6f 8d 90 02 00 00     	movdqu	xmm1, xmmword ptr [rbp + 0x290]
1409904f6: f3 0f 6f 95 a0 02 00 00     	movdqu	xmm2, xmmword ptr [rbp + 0x2a0]
1409904fe: f3 0f 6f 9d b0 02 00 00     	movdqu	xmm3, xmmword ptr [rbp + 0x2b0]
140990506: 0f 11 85 b8 00 00 00        	movups	xmmword ptr [rbp + 0xb8], xmm0
14099050d: f3 0f 7f 8d c8 00 00 00     	movdqu	xmmword ptr [rbp + 0xc8], xmm1
140990515: f3 0f 7f 95 d8 00 00 00     	movdqu	xmmword ptr [rbp + 0xd8], xmm2
14099051d: f3 0f 7f 9d e8 00 00 00     	movdqu	xmmword ptr [rbp + 0xe8], xmm3
140990525: 0f 10 85 c0 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2c0]
14099052c: 0f 11 85 f8 00 00 00        	movups	xmmword ptr [rbp + 0xf8], xmm0
140990533: 0f 10 85 d0 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2d0]
14099053a: 0f 11 85 08 01 00 00        	movups	xmmword ptr [rbp + 0x108], xmm0
140990541: 4c 8b 85 28 02 00 00        	mov	r8, qword ptr [rbp + 0x228]
140990548: 4c 89 85 98 00 00 00        	mov	qword ptr [rbp + 0x98], r8
14099054f: f3 0f 6f 85 18 02 00 00     	movdqu	xmm0, xmmword ptr [rbp + 0x218]
140990557: f3 0f 7f 85 88 00 00 00     	movdqu	xmmword ptr [rbp + 0x88], xmm0
14099055f: 48 89 95 20 01 00 00        	mov	qword ptr [rbp + 0x120], rdx
140990566: 48 89 8d 18 01 00 00        	mov	qword ptr [rbp + 0x118], rcx
14099056d: c7 85 a0 00 00 00 01 00 00 00       	mov	dword ptr [rbp + 0xa0], 0x1
140990577: 89 b5 a4 00 00 00           	mov	dword ptr [rbp + 0xa4], esi
14099057d: 89 85 a8 00 00 00           	mov	dword ptr [rbp + 0xa8], eax
140990583: c7 85 ac 00 00 00 01 00 00 00       	mov	dword ptr [rbp + 0xac], 0x1
14099058d: 48 8d 8d 70 03 00 00        	lea	rcx, [rbp + 0x370]
140990594: 48 8d 55 40                 	lea	rdx, [rbp + 0x40]
140990598: e8 33 cf 04 00              	call	0x1409dd4d0 <.text+0x9dc4d0>
14099059d: 80 bd 06 03 00 00 08        	cmp	byte ptr [rbp + 0x306], 0x8
1409905a4: 48 8b b5 d0 03 00 00        	mov	rsi, qword ptr [rbp + 0x3d0]
1409905ab: 74 0c                       	je	0x1409905b9 <.text+0x98f5b9>
1409905ad: 48 8d 8d f0 02 00 00        	lea	rcx, [rbp + 0x2f0]
1409905b4: e8 f7 1e 25 00              	call	0x140be24b0 <.text+0xbe14b0>
1409905b9: 44 8b 95 dc 03 00 00        	mov	r10d, dword ptr [rbp + 0x3dc]
1409905c0: 44 89 56 5c                 	mov	dword ptr [rsi + 0x5c], r10d
1409905c4: 4c 8b 9d c8 03 00 00        	mov	r11, qword ptr [rbp + 0x3c8]
1409905cb: 41 8b 0b                    	mov	ecx, dword ptr [r11]
1409905ce: 31 c0                       	xor	eax, eax
1409905d0: ba 02 00 00 00              	mov	edx, 0x2
1409905d5: 4c 8d 05 28 9a 12 02        	lea	r8, [rip + 0x2129a28]   # 0x142aba004
1409905dc: 4d 63 0c 88                 	movsxd	r9, dword ptr [r8 + 4*rcx]
1409905e0: 4d 01 c1                    	add	r9, r8
1409905e3: 41 ff e1                    	jmp	r9
1409905e6: 41 8b 43 04                 	mov	eax, dword ptr [r11 + 0x4]
1409905ea: ff c0                       	inc	eax
1409905ec: 41 89 43 04                 	mov	dword ptr [r11 + 0x4], eax
1409905f0: 8b 55 30                    	mov	edx, dword ptr [rbp + 0x30]
1409905f3: 41 89 54 24 20              	mov	dword ptr [r12 + 0x20], edx
1409905f8: 66 0f 6f 45 10              	movdqa	xmm0, xmmword ptr [rbp + 0x10]
1409905fd: 66 0f 6f 4d 20              	movdqa	xmm1, xmmword ptr [rbp + 0x20]
140990602: f3 41 0f 7f 4c 24 10        	movdqu	xmmword ptr [r12 + 0x10], xmm1
140990609: f3 41 0f 7f 04 24           	movdqu	xmmword ptr [r12], xmm0
14099060f: 89 ca                       	mov	edx, ecx
140990611: eb 1c                       	jmp	0x14099062f <.text+0x98f62f>
140990613: 41 8b 4b 04                 	mov	ecx, dword ptr [r11 + 0x4]
140990617: 44 8d 41 01                 	lea	r8d, [rcx + 0x1]
14099061b: 83 f9 04                    	cmp	ecx, 0x4
14099061e: ba 03 00 00 00              	mov	edx, 0x3
140990623: 83 da 00                    	sbb	edx, 0x0
140990626: 31 c0                       	xor	eax, eax
140990628: 83 f9 04                    	cmp	ecx, 0x4
14099062b: 41 0f 42 c0                 	cmovb	eax, r8d
14099062f: 44 89 56 5c                 	mov	dword ptr [rsi + 0x5c], r10d
140990633: 41 89 13                    	mov	dword ptr [r11], edx
140990636: 41 89 43 04                 	mov	dword ptr [r11 + 0x4], eax
14099063a: 48 81 c4 68 04 00 00        	add	rsp, 0x468
140990641: 5b                          	pop	rbx
140990642: 5f                          	pop	rdi
140990643: 5e                          	pop	rsi
140990644: 41 5c                       	pop	r12
140990646: 41 5d                       	pop	r13
140990648: 41 5e                       	pop	r14
14099064a: 41 5f                       	pop	r15
14099064c: 5d                          	pop	rbp
14099064d: c3                          	ret
14099064e: 48 8d 8f 90 00 00 00        	lea	rcx, [rdi + 0x90]
140990655: e8 d6 83 f4 01              	call	0x1428d8a30 <.text+0x28d7a30>
14099065a: 0f b6 87 98 00 00 00        	movzx	eax, byte ptr [rdi + 0x98]
140990661: 48 83 bf f0 00 00 00 00     	cmp	qword ptr [rdi + 0xf0], 0x0
140990669: 0f 85 af f4 ff ff           	jne	0x14098fb1e <.text+0x98eb1e>
14099066f: ba ff ff ff ff              	mov	edx, 0xffffffff
140990674: f0                          	lock
140990675: 0f c1 97 90 00 00 00        	xadd	dword ptr [rdi + 0x90], edx
14099067c: ff ca                       	dec	edx
14099067e: 89 d0                       	mov	eax, edx
140990680: 25 ff ff ff bf              	and	eax, 0xbfffffff
140990685: f7 d8                       	neg	eax
140990687: 70 27                       	jo	0x1409906b0 <.text+0x98f6b0>
140990689: 48 8d 0d c8 97 12 02        	lea	rcx, [rip + 0x21297c8]  # 0x142ab9e58
140990690: e8 7b 8c f4 01              	call	0x1428d9310 <.text+0x28d8310>
140990695: 48 8d 8f 90 00 00 00        	lea	rcx, [rdi + 0x90]
14099069c: e8 9f 86 f4 01              	call	0x1428d8d40 <.text+0x28d7d40>
1409906a1: 48 39 b7 18 02 00 00        	cmp	qword ptr [rdi + 0x218], rsi
1409906a8: 0f 87 3e f5 ff ff           	ja	0x14098fbec <.text+0x98ebec>
1409906ae: eb d9                       	jmp	0x140990689 <.text+0x98f689>
1409906b0: 48 81 c7 90 00 00 00        	add	rdi, 0x90
1409906b7: 48 89 f9                    	mov	rcx, rdi
1409906ba: e8 81 86 f4 01              	call	0x1428d8d40 <.text+0x28d7d40>
1409906bf: 48 8d 0d 92 97 12 02        	lea	rcx, [rip + 0x2129792]  # 0x142ab9e58
1409906c6: e8 45 8c f4 01              	call	0x1428d9310 <.text+0x28d8310>
1409906cb: 48 8b 85 d0 01 00 00        	mov	rax, qword ptr [rbp + 0x1d0]
1409906d2: 48 89 45 40                 	mov	qword ptr [rbp + 0x40], rax
1409906d6: 48 8d 05 b3 97 12 02        	lea	rax, [rip + 0x21297b3]  # 0x142ab9e90
1409906dd: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1409906e2: 48 8d 0d ef 93 12 02        	lea	rcx, [rip + 0x21293ef]  # 0x142ab9ad8
1409906e9: 4c 8d 0d c8 93 12 02        	lea	r9, [rip + 0x21293c8]   # 0x142ab9ab8
1409906f0: 4c 8d 45 40                 	lea	r8, [rbp + 0x40]
1409906f4: ba 2b 00 00 00              	mov	edx, 0x2b
1409906f9: e8 62 90 f4 01              	call	0x1428d9760 <.text+0x28d8760>
1409906fe: eb 33                       	jmp	0x140990733 <.text+0x98f733>
140990700: 48 8b 85 d0 01 00 00        	mov	rax, qword ptr [rbp + 0x1d0]
140990707: 48 89 45 40                 	mov	qword ptr [rbp + 0x40], rax
14099070b: 48 8d 05 ae 97 12 02        	lea	rax, [rip + 0x21297ae]  # 0x142ab9ec0
140990712: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
140990717: 48 8d 0d ba 93 12 02        	lea	rcx, [rip + 0x21293ba]  # 0x142ab9ad8
14099071e: 4c 8d 0d 93 93 12 02        	lea	r9, [rip + 0x2129393]   # 0x142ab9ab8
140990725: 4c 8d 45 40                 	lea	r8, [rbp + 0x40]
140990729: ba 2b 00 00 00              	mov	edx, 0x2b
14099072e: e8 2d 90 f4 01              	call	0x1428d9760 <.text+0x28d8760>
140990733: 0f 0b                       	ud2
140990735: 48 8d 0d 54 96 12 02        	lea	rcx, [rip + 0x2129654]  # 0x142ab9d90
14099073c: 4c 8d 05 2d 98 12 02        	lea	r8, [rip + 0x212982d]   # 0x142ab9f70
140990743: ba 28 00 00 00              	mov	edx, 0x28
140990748: e8 e3 8c f4 01              	call	0x1428d9430 <.text+0x28d8430>
14099074d: cc                          	int3
14099074e: 66 90                       	nop
140990750: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140990755: 55                          	push	rbp
140990756: 41 57                       	push	r15
140990758: 41 56                       	push	r14
14099075a: 41 55                       	push	r13
14099075c: 41 54                       	push	r12
14099075e: 56                          	push	rsi
14099075f: 57                          	push	rdi
140990760: 53                          	push	rbx
140990761: 48 83 ec 38                 	sub	rsp, 0x38
140990765: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14099076c: 80 bd 06 03 00 00 08        	cmp	byte ptr [rbp + 0x306], 0x8
140990773: 74 0c                       	je	0x140990781 <.text+0x98f781>
140990775: 48 8d 8d f0 02 00 00        	lea	rcx, [rbp + 0x2f0]
14099077c: e8 2f 1d 25 00              	call	0x140be24b0 <.text+0xbe14b0>
140990781: 90                          	nop
140990782: 48 83 c4 38                 	add	rsp, 0x38
140990786: 5b                          	pop	rbx
140990787: 5f                          	pop	rdi
140990788: 5e                          	pop	rsi
140990789: 41 5c                       	pop	r12
14099078b: 41 5d                       	pop	r13
14099078d: 41 5e                       	pop	r14
14099078f: 41 5f                       	pop	r15
140990791: 5d                          	pop	rbp
140990792: c3                          	ret
140990793: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
1409907a0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409907a5: 55                          	push	rbp
1409907a6: 41 57                       	push	r15
1409907a8: 41 56                       	push	r14
1409907aa: 41 55                       	push	r13
1409907ac: 41 54                       	push	r12
1409907ae: 56                          	push	rsi
1409907af: 57                          	push	rdi
1409907b0: 53                          	push	rbx
1409907b1: 48 83 ec 38                 	sub	rsp, 0x38
1409907b5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409907bc: 48 8d 8d d0 01 00 00        	lea	rcx, [rbp + 0x1d0]
1409907c3: e8 e8 67 f0 ff              	call	0x140896fb0 <.text+0x895fb0>
1409907c8: 90                          	nop
1409907c9: 48 83 c4 38                 	add	rsp, 0x38
1409907cd: 5b                          	pop	rbx
1409907ce: 5f                          	pop	rdi
1409907cf: 5e                          	pop	rsi
1409907d0: 41 5c                       	pop	r12
1409907d2: 41 5d                       	pop	r13
1409907d4: 41 5e                       	pop	r14
1409907d6: 41 5f                       	pop	r15
1409907d8: 5d                          	pop	rbp
1409907d9: c3                          	ret
1409907da: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409907e0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409907e5: 55                          	push	rbp
1409907e6: 41 57                       	push	r15
1409907e8: 41 56                       	push	r14
1409907ea: 41 55                       	push	r13
1409907ec: 41 54                       	push	r12
1409907ee: 56                          	push	rsi
1409907ef: 57                          	push	rdi
1409907f0: 53                          	push	rbx
1409907f1: 48 83 ec 38                 	sub	rsp, 0x38
1409907f5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409907fc: 80 bd 06 03 00 00 08        	cmp	byte ptr [rbp + 0x306], 0x8
140990803: 74 0c                       	je	0x140990811 <.text+0x98f811>
140990805: 48 8d 8d f0 02 00 00        	lea	rcx, [rbp + 0x2f0]
14099080c: e8 9f 1c 25 00              	call	0x140be24b0 <.text+0xbe14b0>
140990811: 90                          	nop
140990812: 48 83 c4 38                 	add	rsp, 0x38
140990816: 5b                          	pop	rbx
140990817: 5f                          	pop	rdi
140990818: 5e                          	pop	rsi
140990819: 41 5c                       	pop	r12
14099081b: 41 5d                       	pop	r13
14099081d: 41 5e                       	pop	r14
14099081f: 41 5f                       	pop	r15
140990821: 5d                          	pop	rbp
140990822: c3                          	ret
140990823: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140990830: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140990835: 55                          	push	rbp
140990836: 41 57                       	push	r15
140990838: 41 56                       	push	r14
14099083a: 41 55                       	push	r13
14099083c: 41 54                       	push	r12
14099083e: 56                          	push	rsi
14099083f: 57                          	push	rdi
140990840: 53                          	push	rbx
140990841: 48 83 ec 38                 	sub	rsp, 0x38
140990845: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14099084c: 48 8d 8d d0 01 00 00        	lea	rcx, [rbp + 0x1d0]
140990853: e8 58 67 f0 ff              	call	0x140896fb0 <.text+0x895fb0>
140990858: 90                          	nop
140990859: 48 83 c4 38                 	add	rsp, 0x38
14099085d: 5b                          	pop	rbx
14099085e: 5f                          	pop	rdi
14099085f: 5e                          	pop	rsi
140990860: 41 5c                       	pop	r12
140990862: 41 5d                       	pop	r13
140990864: 41 5e                       	pop	r14
140990866: 41 5f                       	pop	r15
140990868: 5d                          	pop	rbp
140990869: c3                          	ret
14099086a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
140990870: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140990875: 55                          	push	rbp
140990876: 41 57                       	push	r15
140990878: 41 56                       	push	r14
14099087a: 41 55                       	push	r13
14099087c: 41 54                       	push	r12
14099087e: 56                          	push	rsi
14099087f: 57                          	push	rdi
140990880: 53                          	push	rbx
140990881: 48 83 ec 38                 	sub	rsp, 0x38
140990885: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14099088c: 48 8d 4d 40                 	lea	rcx, [rbp + 0x40]
140990890: e8 1b 67 f0 ff              	call	0x140896fb0 <.text+0x895fb0>
140990895: 90                          	nop
140990896: 48 83 c4 38                 	add	rsp, 0x38
14099089a: 5b                          	pop	rbx
14099089b: 5f                          	pop	rdi
14099089c: 5e                          	pop	rsi
14099089d: 41 5c                       	pop	r12
14099089f: 41 5d                       	pop	r13
1409908a1: 41 5e                       	pop	r14
1409908a3: 41 5f                       	pop	r15
1409908a5: 5d                          	pop	rbp
1409908a6: c3                          	ret
1409908a7: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1409908b0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409908b5: 55                          	push	rbp
1409908b6: 41 57                       	push	r15
1409908b8: 41 56                       	push	r14
1409908ba: 41 55                       	push	r13
1409908bc: 41 54                       	push	r12
1409908be: 56                          	push	rsi
1409908bf: 57                          	push	rdi
1409908c0: 53                          	push	rbx
1409908c1: 48 83 ec 38                 	sub	rsp, 0x38
1409908c5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409908cc: 48 8d 8d d0 01 00 00        	lea	rcx, [rbp + 0x1d0]
1409908d3: e8 d8 66 f0 ff              	call	0x140896fb0 <.text+0x895fb0>
1409908d8: 90                          	nop
1409908d9: 48 83 c4 38                 	add	rsp, 0x38
1409908dd: 5b                          	pop	rbx
1409908de: 5f                          	pop	rdi
1409908df: 5e                          	pop	rsi
1409908e0: 41 5c                       	pop	r12
1409908e2: 41 5d                       	pop	r13
1409908e4: 41 5e                       	pop	r14
1409908e6: 41 5f                       	pop	r15
1409908e8: 5d                          	pop	rbp
1409908e9: c3                          	ret
1409908ea: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409908f0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409908f5: 55                          	push	rbp
1409908f6: 41 57                       	push	r15
1409908f8: 41 56                       	push	r14
1409908fa: 41 55                       	push	r13
1409908fc: 41 54                       	push	r12
1409908fe: 56                          	push	rsi
1409908ff: 57                          	push	rdi
140990900: 53                          	push	rbx
140990901: 48 83 ec 38                 	sub	rsp, 0x38
140990905: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14099090c: 80 bd 76 01 00 00 08        	cmp	byte ptr [rbp + 0x176], 0x8
140990913: 74 0c                       	je	0x140990921 <.text+0x98f921>
140990915: 48 8d 8d 60 01 00 00        	lea	rcx, [rbp + 0x160]
14099091c: e8 8f 1b 25 00              	call	0x140be24b0 <.text+0xbe14b0>
140990921: 90                          	nop
140990922: 48 83 c4 38                 	add	rsp, 0x38
140990926: 5b                          	pop	rbx
140990927: 5f                          	pop	rdi
140990928: 5e                          	pop	rsi
140990929: 41 5c                       	pop	r12
14099092b: 41 5d                       	pop	r13
14099092d: 41 5e                       	pop	r14
14099092f: 41 5f                       	pop	r15
140990931: 5d                          	pop	rbp
140990932: c3                          	ret
140990933: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140990940: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140990945: 55                          	push	rbp
140990946: 41 57                       	push	r15
140990948: 41 56                       	push	r14
14099094a: 41 55                       	push	r13
14099094c: 41 54                       	push	r12
14099094e: 56                          	push	rsi
14099094f: 57                          	push	rdi
140990950: 53                          	push	rbx
140990951: 48 83 ec 38                 	sub	rsp, 0x38
140990955: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14099095c: 48 8d 4d 40                 	lea	rcx, [rbp + 0x40]
140990960: e8 cb c2 3b 00              	call	0x140d4cc30 <.text+0xd4bc30>
140990965: 90                          	nop
140990966: 48 83 c4 38                 	add	rsp, 0x38
14099096a: 5b                          	pop	rbx
14099096b: 5f                          	pop	rdi
14099096c: 5e                          	pop	rsi
14099096d: 41 5c                       	pop	r12
14099096f: 41 5d                       	pop	r13
140990971: 41 5e                       	pop	r14
140990973: 41 5f                       	pop	r15
140990975: 5d                          	pop	rbp
140990976: c3                          	ret
140990977: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
140990980: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140990985: 55                          	push	rbp
140990986: 41 57                       	push	r15
140990988: 41 56                       	push	r14
14099098a: 41 55                       	push	r13
14099098c: 41 54                       	push	r12
14099098e: 56                          	push	rsi
14099098f: 57                          	push	rdi
140990990: 53                          	push	rbx
140990991: 48 83 ec 38                 	sub	rsp, 0x38
140990995: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14099099c: 48 8d 4d 40                 	lea	rcx, [rbp + 0x40]
1409909a0: e8 8b c2 3b 00              	call	0x140d4cc30 <.text+0xd4bc30>
1409909a5: 90                          	nop
1409909a6: 48 83 c4 38                 	add	rsp, 0x38
1409909aa: 5b                          	pop	rbx
1409909ab: 5f                          	pop	rdi
1409909ac: 5e                          	pop	rsi
1409909ad: 41 5c                       	pop	r12
1409909af: 41 5d                       	pop	r13
1409909b1: 41 5e                       	pop	r14
1409909b3: 41 5f                       	pop	r15
1409909b5: 5d                          	pop	rbp
1409909b6: c3                          	ret
1409909b7: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1409909c0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409909c5: 55                          	push	rbp
1409909c6: 41 57                       	push	r15
1409909c8: 41 56                       	push	r14
1409909ca: 41 55                       	push	r13
1409909cc: 41 54                       	push	r12
1409909ce: 56                          	push	rsi
1409909cf: 57                          	push	rdi
1409909d0: 53                          	push	rbx
1409909d1: 48 83 ec 38                 	sub	rsp, 0x38
1409909d5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409909dc: 48 8d 8d d0 01 00 00        	lea	rcx, [rbp + 0x1d0]
1409909e3: e8 c8 65 f0 ff              	call	0x140896fb0 <.text+0x895fb0>
1409909e8: 90                          	nop
1409909e9: 48 83 c4 38                 	add	rsp, 0x38
1409909ed: 5b                          	pop	rbx
1409909ee: 5f                          	pop	rdi
1409909ef: 5e                          	pop	rsi
1409909f0: 41 5c                       	pop	r12
1409909f2: 41 5d                       	pop	r13
1409909f4: 41 5e                       	pop	r14
1409909f6: 41 5f                       	pop	r15
1409909f8: 5d                          	pop	rbp
1409909f9: c3                          	ret
1409909fa: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
140990a00: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140990a05: 55                          	push	rbp
140990a06: 41 57                       	push	r15
140990a08: 41 56                       	push	r14
140990a0a: 41 55                       	push	r13
140990a0c: 41 54                       	push	r12
140990a0e: 56                          	push	rsi
140990a0f: 57                          	push	rdi
140990a10: 53                          	push	rbx
140990a11: 48 83 ec 38                 	sub	rsp, 0x38
140990a15: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140990a1c: 48 8d 4d 40                 	lea	rcx, [rbp + 0x40]
140990a20: e8 8b 65 f0 ff              	call	0x140896fb0 <.text+0x895fb0>
140990a25: 90                          	nop
140990a26: 48 83 c4 38                 	add	rsp, 0x38
140990a2a: 5b                          	pop	rbx
140990a2b: 5f                          	pop	rdi
140990a2c: 5e                          	pop	rsi
140990a2d: 41 5c                       	pop	r12
140990a2f: 41 5d                       	pop	r13
140990a31: 41 5e                       	pop	r14
140990a33: 41 5f                       	pop	r15
140990a35: 5d                          	pop	rbp
140990a36: c3                          	ret
140990a37: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
140990a40: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140990a45: 55                          	push	rbp
140990a46: 41 57                       	push	r15
140990a48: 41 56                       	push	r14
140990a4a: 41 55                       	push	r13
140990a4c: 41 54                       	push	r12
140990a4e: 56                          	push	rsi
140990a4f: 57                          	push	rdi
140990a50: 53                          	push	rbx
140990a51: 48 83 ec 38                 	sub	rsp, 0x38
140990a55: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140990a5c: 48 8d 4d 40                 	lea	rcx, [rbp + 0x40]
140990a60: e8 4b 65 f0 ff              	call	0x140896fb0 <.text+0x895fb0>
140990a65: 90                          	nop
140990a66: 48 83 c4 38                 	add	rsp, 0x38
140990a6a: 5b                          	pop	rbx
140990a6b: 5f                          	pop	rdi
140990a6c: 5e                          	pop	rsi
140990a6d: 41 5c                       	pop	r12
140990a6f: 41 5d                       	pop	r13
140990a71: 41 5e                       	pop	r14
140990a73: 41 5f                       	pop	r15
140990a75: 5d                          	pop	rbp
140990a76: c3                          	ret
