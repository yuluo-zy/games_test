
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1415fa9b0: 41 57                       	push	r15
1415fa9b2: 41 56                       	push	r14
1415fa9b4: 41 55                       	push	r13
1415fa9b6: 41 54                       	push	r12
1415fa9b8: 56                          	push	rsi
1415fa9b9: 57                          	push	rdi
1415fa9ba: 55                          	push	rbp
1415fa9bb: 53                          	push	rbx
1415fa9bc: 48 83 ec 68                 	sub	rsp, 0x68
1415fa9c0: 0f b6 59 08                 	movzx	ebx, byte ptr [rcx + 0x8]
1415fa9c4: 0f b6 41 09                 	movzx	eax, byte ptr [rcx + 0x9]
1415fa9c8: 48 89 44 24 60              	mov	qword ptr [rsp + 0x60], rax
1415fa9cd: 38 c3                       	cmp	bl, al
1415fa9cf: 0f 83 47 03 00 00           	jae	0x1415fad1c <.text+0x15f9d1c>
1415fa9d5: 48 8b 32                    	mov	rsi, qword ptr [rdx]
1415fa9d8: 48 8b 42 08                 	mov	rax, qword ptr [rdx + 0x8]
1415fa9dc: 48 89 44 24 58              	mov	qword ptr [rsp + 0x58], rax
1415fa9e1: 8b 42 10                    	mov	eax, dword ptr [rdx + 0x10]
1415fa9e4: 89 44 24 30                 	mov	dword ptr [rsp + 0x30], eax
1415fa9e8: 48 8b 29                    	mov	rbp, qword ptr [rcx]
1415fa9eb: 89 d8                       	mov	eax, ebx
1415fa9ed: c1 e0 04                    	shl	eax, 0x4
1415fa9f0: 48 05 c0 06 00 00           	add	rax, 0x6c0
1415fa9f6: 48 89 44 24 50              	mov	qword ptr [rsp + 0x50], rax
1415fa9fb: 31 c0                       	xor	eax, eax
1415fa9fd: 48 89 5c 24 48              	mov	qword ptr [rsp + 0x48], rbx
1415faa02: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nop	word ptr cs:[rax + rax]
1415faa10: 0f b6 f8                    	movzx	edi, al
1415faa13: 48 8d 15 de 3b 68 01        	lea	rdx, [rip + 0x1683bde]  # 0x142c7e5f8
1415faa1a: 48 63 0c ba                 	movsxd	rcx, dword ptr [rdx + 4*rdi]
1415faa1e: 48 01 d1                    	add	rcx, rdx
1415faa21: ff e1                       	jmp	rcx
1415faa23: b1 01                       	mov	cl, 0x1
1415faa25: 80 3d 70 a3 03 02 00        	cmp	byte ptr [rip + 0x203a370], 0x0 # 0x143634d9c
1415faa2c: 0f 84 ce 01 00 00           	je	0x1415fac00 <.text+0x15f9c00>
1415faa32: e9 e2 01 00 00              	jmp	0x1415fac19 <.text+0x15f9c19>
1415faa37: b1 11                       	mov	cl, 0x11
1415faa39: 80 3d 5c a3 03 02 00        	cmp	byte ptr [rip + 0x203a35c], 0x0 # 0x143634d9c
1415faa40: 0f 84 ba 01 00 00           	je	0x1415fac00 <.text+0x15f9c00>
1415faa46: e9 ce 01 00 00              	jmp	0x1415fac19 <.text+0x15f9c19>
1415faa4b: b1 0f                       	mov	cl, 0xf
1415faa4d: 80 3d 48 a3 03 02 00        	cmp	byte ptr [rip + 0x203a348], 0x0 # 0x143634d9c
1415faa54: 0f 84 a6 01 00 00           	je	0x1415fac00 <.text+0x15f9c00>
1415faa5a: e9 ba 01 00 00              	jmp	0x1415fac19 <.text+0x15f9c19>
1415faa5f: b1 0c                       	mov	cl, 0xc
1415faa61: 80 3d 34 a3 03 02 00        	cmp	byte ptr [rip + 0x203a334], 0x0 # 0x143634d9c
1415faa68: 0f 84 92 01 00 00           	je	0x1415fac00 <.text+0x15f9c00>
1415faa6e: e9 a6 01 00 00              	jmp	0x1415fac19 <.text+0x15f9c19>
1415faa73: b1 05                       	mov	cl, 0x5
1415faa75: 80 3d 20 a3 03 02 00        	cmp	byte ptr [rip + 0x203a320], 0x0 # 0x143634d9c
1415faa7c: 0f 84 7e 01 00 00           	je	0x1415fac00 <.text+0x15f9c00>
1415faa82: e9 92 01 00 00              	jmp	0x1415fac19 <.text+0x15f9c19>
1415faa87: b1 0d                       	mov	cl, 0xd
1415faa89: 80 3d 0c a3 03 02 00        	cmp	byte ptr [rip + 0x203a30c], 0x0 # 0x143634d9c
1415faa90: 0f 84 6a 01 00 00           	je	0x1415fac00 <.text+0x15f9c00>
1415faa96: e9 7e 01 00 00              	jmp	0x1415fac19 <.text+0x15f9c19>
1415faa9b: b1 0a                       	mov	cl, 0xa
1415faa9d: 80 3d f8 a2 03 02 00        	cmp	byte ptr [rip + 0x203a2f8], 0x0 # 0x143634d9c
1415faaa4: 0f 84 56 01 00 00           	je	0x1415fac00 <.text+0x15f9c00>
1415faaaa: e9 6a 01 00 00              	jmp	0x1415fac19 <.text+0x15f9c19>
1415faaaf: b1 1a                       	mov	cl, 0x1a
1415faab1: 80 3d e4 a2 03 02 00        	cmp	byte ptr [rip + 0x203a2e4], 0x0 # 0x143634d9c
1415faab8: 0f 84 42 01 00 00           	je	0x1415fac00 <.text+0x15f9c00>
1415faabe: e9 56 01 00 00              	jmp	0x1415fac19 <.text+0x15f9c19>
1415faac3: b1 19                       	mov	cl, 0x19
1415faac5: 80 3d d0 a2 03 02 00        	cmp	byte ptr [rip + 0x203a2d0], 0x0 # 0x143634d9c
1415faacc: 0f 84 2e 01 00 00           	je	0x1415fac00 <.text+0x15f9c00>
1415faad2: e9 42 01 00 00              	jmp	0x1415fac19 <.text+0x15f9c19>
1415faad7: b1 03                       	mov	cl, 0x3
1415faad9: 80 3d bc a2 03 02 00        	cmp	byte ptr [rip + 0x203a2bc], 0x0 # 0x143634d9c
1415faae0: 0f 84 1a 01 00 00           	je	0x1415fac00 <.text+0x15f9c00>
1415faae6: e9 2e 01 00 00              	jmp	0x1415fac19 <.text+0x15f9c19>
1415faaeb: b1 10                       	mov	cl, 0x10
1415faaed: 80 3d a8 a2 03 02 00        	cmp	byte ptr [rip + 0x203a2a8], 0x0 # 0x143634d9c
1415faaf4: 0f 84 06 01 00 00           	je	0x1415fac00 <.text+0x15f9c00>
1415faafa: e9 1a 01 00 00              	jmp	0x1415fac19 <.text+0x15f9c19>
1415faaff: b1 04                       	mov	cl, 0x4
1415fab01: 80 3d 94 a2 03 02 00        	cmp	byte ptr [rip + 0x203a294], 0x0 # 0x143634d9c
1415fab08: 0f 84 f2 00 00 00           	je	0x1415fac00 <.text+0x15f9c00>
1415fab0e: e9 06 01 00 00              	jmp	0x1415fac19 <.text+0x15f9c19>
1415fab13: b1 08                       	mov	cl, 0x8
1415fab15: 80 3d 80 a2 03 02 00        	cmp	byte ptr [rip + 0x203a280], 0x0 # 0x143634d9c
1415fab1c: 0f 84 de 00 00 00           	je	0x1415fac00 <.text+0x15f9c00>
1415fab22: e9 f2 00 00 00              	jmp	0x1415fac19 <.text+0x15f9c19>
1415fab27: b1 02                       	mov	cl, 0x2
1415fab29: 80 3d 6c a2 03 02 00        	cmp	byte ptr [rip + 0x203a26c], 0x0 # 0x143634d9c
1415fab30: 0f 84 ca 00 00 00           	je	0x1415fac00 <.text+0x15f9c00>
1415fab36: e9 de 00 00 00              	jmp	0x1415fac19 <.text+0x15f9c19>
1415fab3b: b1 12                       	mov	cl, 0x12
1415fab3d: 80 3d 58 a2 03 02 00        	cmp	byte ptr [rip + 0x203a258], 0x0 # 0x143634d9c
1415fab44: 0f 84 b6 00 00 00           	je	0x1415fac00 <.text+0x15f9c00>
1415fab4a: e9 ca 00 00 00              	jmp	0x1415fac19 <.text+0x15f9c19>
1415fab4f: b1 15                       	mov	cl, 0x15
1415fab51: 80 3d 44 a2 03 02 00        	cmp	byte ptr [rip + 0x203a244], 0x0 # 0x143634d9c
1415fab58: 0f 84 a2 00 00 00           	je	0x1415fac00 <.text+0x15f9c00>
1415fab5e: e9 b6 00 00 00              	jmp	0x1415fac19 <.text+0x15f9c19>
1415fab63: b1 06                       	mov	cl, 0x6
1415fab65: 80 3d 30 a2 03 02 00        	cmp	byte ptr [rip + 0x203a230], 0x0 # 0x143634d9c
1415fab6c: 0f 84 8e 00 00 00           	je	0x1415fac00 <.text+0x15f9c00>
1415fab72: e9 a2 00 00 00              	jmp	0x1415fac19 <.text+0x15f9c19>
1415fab77: b1 16                       	mov	cl, 0x16
1415fab79: 80 3d 1c a2 03 02 00        	cmp	byte ptr [rip + 0x203a21c], 0x0 # 0x143634d9c
1415fab80: 74 7e                       	je	0x1415fac00 <.text+0x15f9c00>
1415fab82: e9 92 00 00 00              	jmp	0x1415fac19 <.text+0x15f9c19>
1415fab87: b1 0e                       	mov	cl, 0xe
1415fab89: 80 3d 0c a2 03 02 00        	cmp	byte ptr [rip + 0x203a20c], 0x0 # 0x143634d9c
1415fab90: 74 6e                       	je	0x1415fac00 <.text+0x15f9c00>
1415fab92: e9 82 00 00 00              	jmp	0x1415fac19 <.text+0x15f9c19>
1415fab97: b1 07                       	mov	cl, 0x7
1415fab99: 80 3d fc a1 03 02 00        	cmp	byte ptr [rip + 0x203a1fc], 0x0 # 0x143634d9c
1415faba0: 74 5e                       	je	0x1415fac00 <.text+0x15f9c00>
1415faba2: eb 75                       	jmp	0x1415fac19 <.text+0x15f9c19>
1415faba4: b1 13                       	mov	cl, 0x13
1415faba6: 80 3d ef a1 03 02 00        	cmp	byte ptr [rip + 0x203a1ef], 0x0 # 0x143634d9c
1415fabad: 74 51                       	je	0x1415fac00 <.text+0x15f9c00>
1415fabaf: eb 68                       	jmp	0x1415fac19 <.text+0x15f9c19>
1415fabb1: b1 0b                       	mov	cl, 0xb
1415fabb3: 80 3d e2 a1 03 02 00        	cmp	byte ptr [rip + 0x203a1e2], 0x0 # 0x143634d9c
1415fabba: 74 44                       	je	0x1415fac00 <.text+0x15f9c00>
1415fabbc: eb 5b                       	jmp	0x1415fac19 <.text+0x15f9c19>
1415fabbe: b1 09                       	mov	cl, 0x9
1415fabc0: 80 3d d5 a1 03 02 00        	cmp	byte ptr [rip + 0x203a1d5], 0x0 # 0x143634d9c
1415fabc7: 74 37                       	je	0x1415fac00 <.text+0x15f9c00>
1415fabc9: eb 4e                       	jmp	0x1415fac19 <.text+0x15f9c19>
1415fabcb: b1 17                       	mov	cl, 0x17
1415fabcd: 80 3d c8 a1 03 02 00        	cmp	byte ptr [rip + 0x203a1c8], 0x0 # 0x143634d9c
1415fabd4: 74 2a                       	je	0x1415fac00 <.text+0x15f9c00>
1415fabd6: eb 41                       	jmp	0x1415fac19 <.text+0x15f9c19>
1415fabd8: b1 14                       	mov	cl, 0x14
1415fabda: 80 3d bb a1 03 02 00        	cmp	byte ptr [rip + 0x203a1bb], 0x0 # 0x143634d9c
1415fabe1: 74 1d                       	je	0x1415fac00 <.text+0x15f9c00>
1415fabe3: eb 34                       	jmp	0x1415fac19 <.text+0x15f9c19>
1415fabe5: b1 18                       	mov	cl, 0x18
1415fabe7: 80 3d ae a1 03 02 00        	cmp	byte ptr [rip + 0x203a1ae], 0x0 # 0x143634d9c
1415fabee: 74 10                       	je	0x1415fac00 <.text+0x15f9c00>
1415fabf0: eb 27                       	jmp	0x1415fac19 <.text+0x15f9c19>
1415fabf2: b1 1b                       	mov	cl, 0x1b
1415fabf4: 80 3d a1 a1 03 02 00        	cmp	byte ptr [rip + 0x203a1a1], 0x0 # 0x143634d9c
1415fabfb: 75 1c                       	jne	0x1415fac19 <.text+0x15f9c19>
1415fabfd: 0f 1f 00                    	nop	dword ptr [rax]
1415fac00: 3c 01                       	cmp	al, 0x1
1415fac02: 89 c8                       	mov	eax, ecx
1415fac04: 0f 87 06 fe ff ff           	ja	0x1415faa10 <.text+0x15f9a10>
1415fac0a: 80 3d 8c a1 03 02 00        	cmp	byte ptr [rip + 0x203a18c], 0x0 # 0x143634d9d
1415fac11: 89 c8                       	mov	eax, ecx
1415fac13: 0f 85 f7 fd ff ff           	jne	0x1415faa10 <.text+0x15f9a10>
1415fac19: 88 4c 24 2f                 	mov	byte ptr [rsp + 0x2f], cl
1415fac1d: 4c 8d 34 bb                 	lea	r14, [rbx + 4*rdi]
1415fac21: 41 89 ff                    	mov	r15d, edi
1415fac24: 41 c1 e7 06                 	shl	r15d, 0x6
1415fac28: 4c 03 7c 24 50              	add	r15, qword ptr [rsp + 0x50]
1415fac2d: 0f 1f 00                    	nop	dword ptr [rax]
1415fac30: 48 8b 55 28                 	mov	rdx, qword ptr [rbp + 0x28]
1415fac34: 4c 39 f2                    	cmp	rdx, r14
1415fac37: 0f 86 a5 02 00 00           	jbe	0x1415faee2 <.text+0x15f9ee2>
1415fac3d: 48 8b 55 40                 	mov	rdx, qword ptr [rbp + 0x40]
1415fac41: 4c 39 f2                    	cmp	rdx, r14
1415fac44: 0f 86 89 02 00 00           	jbe	0x1415faed3 <.text+0x15f9ed3>
1415fac4a: 48 8b 55 58                 	mov	rdx, qword ptr [rbp + 0x58]
1415fac4e: 4c 39 f2                    	cmp	rdx, r14
1415fac51: 0f 86 9a 02 00 00           	jbe	0x1415faef1 <.text+0x15f9ef1>
1415fac57: 48 8b 45 20                 	mov	rax, qword ptr [rbp + 0x20]
1415fac5b: 42 8b 04 b0                 	mov	eax, dword ptr [rax + 4*r14]
1415fac5f: 48 8b 4d 38                 	mov	rcx, qword ptr [rbp + 0x38]
1415fac63: 48 8b 55 50                 	mov	rdx, qword ptr [rbp + 0x50]
1415fac67: 42 8b 0c b1                 	mov	ecx, dword ptr [rcx + 4*r14]
1415fac6b: 42 8b 14 b2                 	mov	edx, dword ptr [rdx + 4*r14]
1415fac6f: 89 7c 24 34                 	mov	dword ptr [rsp + 0x34], edi
1415fac73: 44 89 74 24 38              	mov	dword ptr [rsp + 0x38], r14d
1415fac78: 89 44 24 3c                 	mov	dword ptr [rsp + 0x3c], eax
1415fac7c: 89 4c 24 40                 	mov	dword ptr [rsp + 0x40], ecx
1415fac80: 89 54 24 44                 	mov	dword ptr [rsp + 0x44], edx
1415fac84: 4c 8d 44 24 34              	lea	r8, [rsp + 0x34]
1415fac89: 41 b9 14 00 00 00           	mov	r9d, 0x14
1415fac8f: 48 89 f1                    	mov	rcx, rsi
1415fac92: 48 8b 54 24 58              	mov	rdx, qword ptr [rsp + 0x58]
1415fac97: e8 b4 cd 0f 00              	call	0x1416f7a50 <.text+0x16f6a50>
1415fac9c: 4c 8b 2e                    	mov	r13, qword ptr [rsi]
1415fac9f: 4c 8b 66 08                 	mov	r12, qword ptr [rsi + 0x8]
1415faca3: 41 80 bc 24 ba 0a 00 00 01  	cmp	byte ptr [r12 + 0xaba], 0x1
1415facac: 75 0c                       	jne	0x1415facba <.text+0x15f9cba>
1415facae: 49 8b 55 48                 	mov	rdx, qword ptr [r13 + 0x48]
1415facb2: 4c 89 e1                    	mov	rcx, r12
1415facb5: e8 d6 33 5f ff              	call	0x140bee090 <.text+0xbed090>
1415facba: f0                          	lock
1415facbb: 49 ff 84 24 a0 0a 00 00     	inc	qword ptr [r12 + 0xaa0]
1415facc3: 4d 8b 6d 48                 	mov	r13, qword ptr [r13 + 0x48]
1415facc7: 48 8b 4e 10                 	mov	rcx, qword ptr [rsi + 0x10]
1415faccb: 8b 54 24 30                 	mov	edx, dword ptr [rsp + 0x30]
1415faccf: e8 fc cc 5e ff              	call	0x140be79d0 <.text+0xbe69d0>
1415facd4: 4c 89 e1                    	mov	rcx, r12
1415facd7: 48 89 c2                    	mov	rdx, rax
1415facda: 45 31 c0                    	xor	r8d, r8d
1415facdd: e8 8e 47 5f ff              	call	0x140bef470 <.text+0xbee470>
1415face2: a8 01                       	test	al, 0x1
1415face4: 0f 84 16 02 00 00           	je	0x1415faf00 <.text+0x15f9f00>
1415facea: 48 ff c3                    	inc	rbx
1415faced: 4c 89 e9                    	mov	rcx, r13
1415facf0: 4d 89 f8                    	mov	r8, r15
1415facf3: 41 ff 94 24 68 03 00 00     	call	qword ptr [r12 + 0x368]
1415facfb: 49 ff c6                    	inc	r14
1415facfe: 49 83 c7 10                 	add	r15, 0x10
1415fad02: 48 3b 5c 24 60              	cmp	rbx, qword ptr [rsp + 0x60]
1415fad07: 0f 82 23 ff ff ff           	jb	0x1415fac30 <.text+0x15f9c30>
1415fad0d: 0f b6 44 24 2f              	movzx	eax, byte ptr [rsp + 0x2f]
1415fad12: 48 8b 5c 24 48              	mov	rbx, qword ptr [rsp + 0x48]
1415fad17: e9 f4 fc ff ff              	jmp	0x1415faa10 <.text+0x15f9a10>
1415fad1c: f6 05 79 a0 03 02 01        	test	byte ptr [rip + 0x203a079], 0x1 # 0x143634d9c
1415fad23: 0f 84 89 00 00 00           	je	0x1415fadb2 <.text+0x15f9db2>
1415fad29: 31 c9                       	xor	ecx, ecx
1415fad2b: 48 8d 05 56 38 68 01        	lea	rax, [rip + 0x1683856]  # 0x142c7e588
1415fad32: eb 0c                       	jmp	0x1415fad40 <.text+0x15f9d40>
1415fad34: b1 18                       	mov	cl, 0x18
1415fad36: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
1415fad40: 0f b6 c9                    	movzx	ecx, cl
1415fad43: 48 63 14 88                 	movsxd	rdx, dword ptr [rax + 4*rcx]
1415fad47: 48 01 c2                    	add	rdx, rax
1415fad4a: b1 02                       	mov	cl, 0x2
1415fad4c: ff e2                       	jmp	rdx
1415fad4e: b1 01                       	mov	cl, 0x1
1415fad50: eb ee                       	jmp	0x1415fad40 <.text+0x15f9d40>
1415fad52: b1 14                       	mov	cl, 0x14
1415fad54: eb ea                       	jmp	0x1415fad40 <.text+0x15f9d40>
1415fad56: b1 0f                       	mov	cl, 0xf
1415fad58: eb e6                       	jmp	0x1415fad40 <.text+0x15f9d40>
1415fad5a: b1 1a                       	mov	cl, 0x1a
1415fad5c: eb e2                       	jmp	0x1415fad40 <.text+0x15f9d40>
1415fad5e: b1 16                       	mov	cl, 0x16
1415fad60: eb de                       	jmp	0x1415fad40 <.text+0x15f9d40>
1415fad62: b1 09                       	mov	cl, 0x9
1415fad64: eb da                       	jmp	0x1415fad40 <.text+0x15f9d40>
1415fad66: b1 07                       	mov	cl, 0x7
1415fad68: eb d6                       	jmp	0x1415fad40 <.text+0x15f9d40>
1415fad6a: b1 03                       	mov	cl, 0x3
1415fad6c: eb d2                       	jmp	0x1415fad40 <.text+0x15f9d40>
1415fad6e: b1 08                       	mov	cl, 0x8
1415fad70: eb ce                       	jmp	0x1415fad40 <.text+0x15f9d40>
1415fad72: b1 19                       	mov	cl, 0x19
1415fad74: eb ca                       	jmp	0x1415fad40 <.text+0x15f9d40>
1415fad76: b1 0a                       	mov	cl, 0xa
1415fad78: eb c6                       	jmp	0x1415fad40 <.text+0x15f9d40>
1415fad7a: b1 13                       	mov	cl, 0x13
1415fad7c: eb c2                       	jmp	0x1415fad40 <.text+0x15f9d40>
1415fad7e: b1 06                       	mov	cl, 0x6
1415fad80: eb be                       	jmp	0x1415fad40 <.text+0x15f9d40>
1415fad82: b1 05                       	mov	cl, 0x5
1415fad84: eb ba                       	jmp	0x1415fad40 <.text+0x15f9d40>
1415fad86: b1 04                       	mov	cl, 0x4
1415fad88: eb b6                       	jmp	0x1415fad40 <.text+0x15f9d40>
1415fad8a: b1 1b                       	mov	cl, 0x1b
1415fad8c: eb b2                       	jmp	0x1415fad40 <.text+0x15f9d40>
1415fad8e: b1 0e                       	mov	cl, 0xe
1415fad90: eb ae                       	jmp	0x1415fad40 <.text+0x15f9d40>
1415fad92: b1 15                       	mov	cl, 0x15
1415fad94: eb aa                       	jmp	0x1415fad40 <.text+0x15f9d40>
1415fad96: b1 0d                       	mov	cl, 0xd
1415fad98: eb a6                       	jmp	0x1415fad40 <.text+0x15f9d40>
1415fad9a: b1 11                       	mov	cl, 0x11
1415fad9c: eb a2                       	jmp	0x1415fad40 <.text+0x15f9d40>
1415fad9e: b1 10                       	mov	cl, 0x10
1415fada0: eb 9e                       	jmp	0x1415fad40 <.text+0x15f9d40>
1415fada2: b1 12                       	mov	cl, 0x12
1415fada4: eb 9a                       	jmp	0x1415fad40 <.text+0x15f9d40>
1415fada6: b1 0b                       	mov	cl, 0xb
1415fada8: eb 96                       	jmp	0x1415fad40 <.text+0x15f9d40>
1415fadaa: b1 17                       	mov	cl, 0x17
1415fadac: eb 92                       	jmp	0x1415fad40 <.text+0x15f9d40>
1415fadae: b1 0c                       	mov	cl, 0xc
1415fadb0: eb 8e                       	jmp	0x1415fad40 <.text+0x15f9d40>
1415fadb2: f6 05 e4 9f 03 02 01        	test	byte ptr [rip + 0x2039fe4], 0x1 # 0x143634d9d
1415fadb9: 0f 84 83 00 00 00           	je	0x1415fae42 <.text+0x15f9e42>
1415fadbf: 31 c9                       	xor	ecx, ecx
1415fadc1: 48 8d 05 50 37 68 01        	lea	rax, [rip + 0x1683750]  # 0x142c7e518
1415fadc8: eb 06                       	jmp	0x1415fadd0 <.text+0x15f9dd0>
1415fadca: b1 18                       	mov	cl, 0x18
1415fadcc: 0f 1f 40 00                 	nop	dword ptr [rax]
1415fadd0: 0f b6 c9                    	movzx	ecx, cl
1415fadd3: 48 63 14 88                 	movsxd	rdx, dword ptr [rax + 4*rcx]
1415fadd7: 48 01 c2                    	add	rdx, rax
1415fadda: b1 02                       	mov	cl, 0x2
1415faddc: ff e2                       	jmp	rdx
1415fadde: b1 01                       	mov	cl, 0x1
1415fade0: eb ee                       	jmp	0x1415fadd0 <.text+0x15f9dd0>
1415fade2: b1 14                       	mov	cl, 0x14
1415fade4: eb ea                       	jmp	0x1415fadd0 <.text+0x15f9dd0>
1415fade6: b1 0f                       	mov	cl, 0xf
1415fade8: eb e6                       	jmp	0x1415fadd0 <.text+0x15f9dd0>
1415fadea: b1 1a                       	mov	cl, 0x1a
1415fadec: eb e2                       	jmp	0x1415fadd0 <.text+0x15f9dd0>
1415fadee: b1 16                       	mov	cl, 0x16
1415fadf0: eb de                       	jmp	0x1415fadd0 <.text+0x15f9dd0>
1415fadf2: b1 09                       	mov	cl, 0x9
1415fadf4: eb da                       	jmp	0x1415fadd0 <.text+0x15f9dd0>
1415fadf6: b1 07                       	mov	cl, 0x7
1415fadf8: eb d6                       	jmp	0x1415fadd0 <.text+0x15f9dd0>
1415fadfa: b1 03                       	mov	cl, 0x3
1415fadfc: eb d2                       	jmp	0x1415fadd0 <.text+0x15f9dd0>
1415fadfe: b1 08                       	mov	cl, 0x8
1415fae00: eb ce                       	jmp	0x1415fadd0 <.text+0x15f9dd0>
1415fae02: b1 19                       	mov	cl, 0x19
1415fae04: eb ca                       	jmp	0x1415fadd0 <.text+0x15f9dd0>
1415fae06: b1 0a                       	mov	cl, 0xa
1415fae08: eb c6                       	jmp	0x1415fadd0 <.text+0x15f9dd0>
1415fae0a: b1 13                       	mov	cl, 0x13
1415fae0c: eb c2                       	jmp	0x1415fadd0 <.text+0x15f9dd0>
1415fae0e: b1 06                       	mov	cl, 0x6
1415fae10: eb be                       	jmp	0x1415fadd0 <.text+0x15f9dd0>
1415fae12: b1 05                       	mov	cl, 0x5
1415fae14: eb ba                       	jmp	0x1415fadd0 <.text+0x15f9dd0>
1415fae16: b1 04                       	mov	cl, 0x4
1415fae18: eb b6                       	jmp	0x1415fadd0 <.text+0x15f9dd0>
1415fae1a: b1 1b                       	mov	cl, 0x1b
1415fae1c: eb b2                       	jmp	0x1415fadd0 <.text+0x15f9dd0>
1415fae1e: b1 0e                       	mov	cl, 0xe
1415fae20: eb ae                       	jmp	0x1415fadd0 <.text+0x15f9dd0>
1415fae22: b1 15                       	mov	cl, 0x15
1415fae24: eb aa                       	jmp	0x1415fadd0 <.text+0x15f9dd0>
1415fae26: b1 0d                       	mov	cl, 0xd
1415fae28: eb a6                       	jmp	0x1415fadd0 <.text+0x15f9dd0>
1415fae2a: b1 11                       	mov	cl, 0x11
1415fae2c: eb a2                       	jmp	0x1415fadd0 <.text+0x15f9dd0>
1415fae2e: b1 10                       	mov	cl, 0x10
1415fae30: eb 9e                       	jmp	0x1415fadd0 <.text+0x15f9dd0>
1415fae32: b1 12                       	mov	cl, 0x12
1415fae34: eb 9a                       	jmp	0x1415fadd0 <.text+0x15f9dd0>
1415fae36: b1 0b                       	mov	cl, 0xb
1415fae38: eb 96                       	jmp	0x1415fadd0 <.text+0x15f9dd0>
1415fae3a: b1 17                       	mov	cl, 0x17
1415fae3c: eb 92                       	jmp	0x1415fadd0 <.text+0x15f9dd0>
1415fae3e: b1 0c                       	mov	cl, 0xc
1415fae40: eb 8e                       	jmp	0x1415fadd0 <.text+0x15f9dd0>
1415fae42: 31 c9                       	xor	ecx, ecx
1415fae44: 48 8d 05 5d 36 68 01        	lea	rax, [rip + 0x168365d]  # 0x142c7e4a8
1415fae4b: eb 03                       	jmp	0x1415fae50 <.text+0x15f9e50>
1415fae4d: b1 18                       	mov	cl, 0x18
1415fae4f: 90                          	nop
1415fae50: 0f b6 c9                    	movzx	ecx, cl
1415fae53: 48 63 14 88                 	movsxd	rdx, dword ptr [rax + 4*rcx]
1415fae57: 48 01 c2                    	add	rdx, rax
1415fae5a: b1 02                       	mov	cl, 0x2
1415fae5c: ff e2                       	jmp	rdx
1415fae5e: b1 01                       	mov	cl, 0x1
1415fae60: eb ee                       	jmp	0x1415fae50 <.text+0x15f9e50>
1415fae62: b1 14                       	mov	cl, 0x14
1415fae64: eb ea                       	jmp	0x1415fae50 <.text+0x15f9e50>
1415fae66: b1 0f                       	mov	cl, 0xf
1415fae68: eb e6                       	jmp	0x1415fae50 <.text+0x15f9e50>
1415fae6a: b1 1a                       	mov	cl, 0x1a
1415fae6c: eb e2                       	jmp	0x1415fae50 <.text+0x15f9e50>
1415fae6e: b1 16                       	mov	cl, 0x16
1415fae70: eb de                       	jmp	0x1415fae50 <.text+0x15f9e50>
1415fae72: b1 09                       	mov	cl, 0x9
1415fae74: eb da                       	jmp	0x1415fae50 <.text+0x15f9e50>
1415fae76: b1 07                       	mov	cl, 0x7
1415fae78: eb d6                       	jmp	0x1415fae50 <.text+0x15f9e50>
1415fae7a: b1 03                       	mov	cl, 0x3
1415fae7c: eb d2                       	jmp	0x1415fae50 <.text+0x15f9e50>
1415fae7e: b1 08                       	mov	cl, 0x8
1415fae80: eb ce                       	jmp	0x1415fae50 <.text+0x15f9e50>
1415fae82: b1 19                       	mov	cl, 0x19
1415fae84: eb ca                       	jmp	0x1415fae50 <.text+0x15f9e50>
1415fae86: b1 0a                       	mov	cl, 0xa
1415fae88: eb c6                       	jmp	0x1415fae50 <.text+0x15f9e50>
1415fae8a: b1 13                       	mov	cl, 0x13
1415fae8c: eb c2                       	jmp	0x1415fae50 <.text+0x15f9e50>
1415fae8e: b1 06                       	mov	cl, 0x6
1415fae90: eb be                       	jmp	0x1415fae50 <.text+0x15f9e50>
1415fae92: b1 05                       	mov	cl, 0x5
1415fae94: eb ba                       	jmp	0x1415fae50 <.text+0x15f9e50>
1415fae96: b1 04                       	mov	cl, 0x4
1415fae98: eb b6                       	jmp	0x1415fae50 <.text+0x15f9e50>
1415fae9a: b1 1b                       	mov	cl, 0x1b
1415fae9c: eb b2                       	jmp	0x1415fae50 <.text+0x15f9e50>
1415fae9e: b1 0e                       	mov	cl, 0xe
1415faea0: eb ae                       	jmp	0x1415fae50 <.text+0x15f9e50>
1415faea2: b1 15                       	mov	cl, 0x15
1415faea4: eb aa                       	jmp	0x1415fae50 <.text+0x15f9e50>
1415faea6: b1 0d                       	mov	cl, 0xd
1415faea8: eb a6                       	jmp	0x1415fae50 <.text+0x15f9e50>
1415faeaa: b1 11                       	mov	cl, 0x11
1415faeac: eb a2                       	jmp	0x1415fae50 <.text+0x15f9e50>
1415faeae: b1 10                       	mov	cl, 0x10
1415faeb0: eb 9e                       	jmp	0x1415fae50 <.text+0x15f9e50>
1415faeb2: b1 12                       	mov	cl, 0x12
1415faeb4: eb 9a                       	jmp	0x1415fae50 <.text+0x15f9e50>
1415faeb6: b1 0b                       	mov	cl, 0xb
1415faeb8: eb 96                       	jmp	0x1415fae50 <.text+0x15f9e50>
1415faeba: b1 17                       	mov	cl, 0x17
1415faebc: eb 92                       	jmp	0x1415fae50 <.text+0x15f9e50>
1415faebe: b1 0c                       	mov	cl, 0xc
1415faec0: eb 8e                       	jmp	0x1415fae50 <.text+0x15f9e50>
1415faec2: 48 83 c4 68                 	add	rsp, 0x68
1415faec6: 5b                          	pop	rbx
1415faec7: 5d                          	pop	rbp
1415faec8: 5f                          	pop	rdi
1415faec9: 5e                          	pop	rsi
1415faeca: 41 5c                       	pop	r12
1415faecc: 41 5d                       	pop	r13
1415faece: 41 5e                       	pop	r14
1415faed0: 41 5f                       	pop	r15
1415faed2: c3                          	ret
1415faed3: 4c 8d 05 9e 2e 68 01        	lea	r8, [rip + 0x1682e9e]   # 0x142c7dd78
1415faeda: 4c 89 f1                    	mov	rcx, r14
1415faedd: e8 36 e6 2d 01              	call	0x1428d9518 <.text+0x28d8518>
1415faee2: 4c 8d 05 77 2e 68 01        	lea	r8, [rip + 0x1682e77]   # 0x142c7dd60
1415faee9: 4c 89 f1                    	mov	rcx, r14
1415faeec: e8 27 e6 2d 01              	call	0x1428d9518 <.text+0x28d8518>
1415faef1: 4c 8d 05 98 2e 68 01        	lea	r8, [rip + 0x1682e98]   # 0x142c7dd90
1415faef8: 4c 89 f1                    	mov	rcx, r14
1415faefb: e8 18 e6 2d 01              	call	0x1428d9518 <.text+0x28d8518>
1415faf00: 48 8d 0d a9 b2 4b 01        	lea	rcx, [rip + 0x14bb2a9]  # 0x142ab61b0
1415faf07: e8 04 e4 2d 01              	call	0x1428d9310 <.text+0x28d8310>
1415faf0c: cc                          	int3
