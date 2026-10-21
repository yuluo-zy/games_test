
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
142344df0: 55                          	push	rbp
142344df1: 41 57                       	push	r15
142344df3: 41 56                       	push	r14
142344df5: 41 55                       	push	r13
142344df7: 41 54                       	push	r12
142344df9: 56                          	push	rsi
142344dfa: 57                          	push	rdi
142344dfb: 53                          	push	rbx
142344dfc: 48 81 ec 18 03 00 00        	sub	rsp, 0x318
142344e03: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
142344e0b: 44 0f 29 95 80 02 00 00     	movaps	xmmword ptr [rbp + 0x280], xmm10
142344e13: 44 0f 29 8d 70 02 00 00     	movaps	xmmword ptr [rbp + 0x270], xmm9
142344e1b: 44 0f 29 85 60 02 00 00     	movaps	xmmword ptr [rbp + 0x260], xmm8
142344e23: 0f 29 bd 50 02 00 00        	movaps	xmmword ptr [rbp + 0x250], xmm7
142344e2a: 0f 29 b5 40 02 00 00        	movaps	xmmword ptr [rbp + 0x240], xmm6
142344e31: 48 c7 85 38 02 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x238], -0x2
142344e3c: 4c 89 ce                    	mov	rsi, r9
142344e3f: 4c 89 c3                    	mov	rbx, r8
142344e42: 49 89 d6                    	mov	r14, rdx
142344e45: 49 89 cf                    	mov	r15, rcx
142344e48: 48 8b bd 00 03 00 00        	mov	rdi, qword ptr [rbp + 0x300]
142344e4f: e8 ac 1b 8d fe              	call	0x140c16a00 <.text+0xc15a00>
142344e54: 88 85 36 02 00 00           	mov	byte ptr [rbp + 0x236], al
142344e5a: 84 c0                       	test	al, al
142344e5c: 74 63                       	je	0x142344ec1 <.text+0x2343ec1>
142344e5e: 48 89 b5 18 02 00 00        	mov	qword ptr [rbp + 0x218], rsi
142344e65: be 33 00 00 00              	mov	esi, 0x33
142344e6a: b9 33 00 00 00              	mov	ecx, 0x33
142344e6f: e8 cc e5 ff ff              	call	0x142343440 <.text+0x2342440>
142344e74: 4c 8d 25 be 96 d0 00        	lea	r12, [rip + 0xd096be]   # 0x14304e539
142344e7b: 48 83 f8 01                 	cmp	rax, 0x1
142344e7f: 0f 85 c0 00 00 00           	jne	0x142344f45 <.text+0x2343f45>
142344e85: 49 89 d1                    	mov	r9, rdx
142344e88: 48 85 d2                    	test	rdx, rdx
142344e8b: 74 56                       	je	0x142344ee3 <.text+0x2343ee3>
142344e8d: 49 83 f9 33                 	cmp	r9, 0x33
142344e91: 73 4e                       	jae	0x142344ee1 <.text+0x2343ee1>
142344e93: 48 8d 05 9f 96 d0 00        	lea	rax, [rip + 0xd0969f]   # 0x14304e539
142344e9a: 41 80 3c 01 bf              	cmp	byte ptr [r9 + rax], -0x41
142344e9f: 7f 42                       	jg	0x142344ee3 <.text+0x2343ee3>
142344ea1: 48 8d 05 48 97 d0 00        	lea	rax, [rip + 0xd09748]   # 0x14304e5f0
142344ea8: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
142344ead: 48 8d 0d 85 96 d0 00        	lea	rcx, [rip + 0xd09685]   # 0x14304e539
142344eb4: ba 33 00 00 00              	mov	edx, 0x33
142344eb9: 45 31 c0                    	xor	r8d, r8d
142344ebc: e8 7f 4a 59 00              	call	0x1428d9940 <.text+0x28d8940>
142344ec1: 48 c7 85 f8 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x1f8], 0x0
142344ecc: 48 8b 07                    	mov	rax, qword ptr [rdi]
142344ecf: 80 b8 f7 00 00 00 00        	cmp	byte ptr [rax + 0xf7], 0x0
142344ed6: 0f 85 0b 02 00 00           	jne	0x1423450e7 <.text+0x23440e7>
142344edc: e9 f7 04 00 00              	jmp	0x1423453d8 <.text+0x23443d8>
142344ee1: 75 be                       	jne	0x142344ea1 <.text+0x2343ea1>
142344ee3: 4c 89 c9                    	mov	rcx, r9
142344ee6: e8 55 e5 ff ff              	call	0x142343440 <.text+0x2342440>
142344eeb: 48 83 f8 01                 	cmp	rax, 0x1
142344eef: 75 54                       	jne	0x142344f45 <.text+0x2343f45>
142344ef1: 49 89 d0                    	mov	r8, rdx
142344ef4: 49 83 c0 02                 	add	r8, 0x2
142344ef8: 74 39                       	je	0x142344f33 <.text+0x2343f33>
142344efa: 49 83 f8 33                 	cmp	r8, 0x33
142344efe: 73 31                       	jae	0x142344f31 <.text+0x2343f31>
142344f00: 48 8d 05 32 96 d0 00        	lea	rax, [rip + 0xd09632]   # 0x14304e539
142344f07: 41 80 3c 00 bf              	cmp	byte ptr [r8 + rax], -0x41
142344f0c: 7f 25                       	jg	0x142344f33 <.text+0x2343f33>
142344f0e: 48 8d 05 f3 96 d0 00        	lea	rax, [rip + 0xd096f3]   # 0x14304e608
142344f15: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
142344f1a: 48 8d 0d 18 96 d0 00        	lea	rcx, [rip + 0xd09618]   # 0x14304e539
142344f21: ba 33 00 00 00              	mov	edx, 0x33
142344f26: 41 b9 33 00 00 00           	mov	r9d, 0x33
142344f2c: e8 0f 4a 59 00              	call	0x1428d9940 <.text+0x28d8940>
142344f31: 75 db                       	jne	0x142344f0e <.text+0x2343f0e>
142344f33: be 31 00 00 00              	mov	esi, 0x31
142344f38: 48 29 d6                    	sub	rsi, rdx
142344f3b: 4c 8d 25 f7 95 d0 00        	lea	r12, [rip + 0xd095f7]   # 0x14304e539
142344f42: 4d 01 c4                    	add	r12, r8
142344f45: 4c 8d 05 19 97 d0 00        	lea	r8, [rip + 0xd09719]    # 0x14304e665
142344f4c: 4c 8d 2d e5 96 d0 00        	lea	r13, [rip + 0xd096e5]   # 0x14304e638
142344f53: 4c 89 bd 20 02 00 00        	mov	qword ptr [rbp + 0x220], r15
142344f5a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
142344f60: 41 0f be 40 ff              	movsx	eax, byte ptr [r8 - 0x1]
142344f65: 85 c0                       	test	eax, eax
142344f67: 78 17                       	js	0x142344f80 <.text+0x2343f80>
142344f69: 49 ff c8                    	dec	r8
142344f6c: 83 f8 5c                    	cmp	eax, 0x5c
142344f6f: 75 63                       	jne	0x142344fd4 <.text+0x2343fd4>
142344f71: eb 7b                       	jmp	0x142344fee <.text+0x2343fee>
142344f73: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
142344f80: 41 0f b6 48 fe              	movzx	ecx, byte ptr [r8 - 0x2]
142344f85: 80 f9 c0                    	cmp	cl, -0x40
142344f88: 7d 23                       	jge	0x142344fad <.text+0x2343fad>
142344f8a: 41 0f b6 50 fd              	movzx	edx, byte ptr [r8 - 0x3]
142344f8f: 80 fa c0                    	cmp	dl, -0x40
142344f92: 7d 22                       	jge	0x142344fb6 <.text+0x2343fb6>
142344f94: 45 0f b6 48 fc              	movzx	r9d, byte ptr [r8 - 0x4]
142344f99: 49 83 c0 fc                 	add	r8, -0x4
142344f9d: 41 83 e1 07                 	and	r9d, 0x7
142344fa1: 41 c1 e1 06                 	shl	r9d, 0x6
142344fa5: 83 e2 3f                    	and	edx, 0x3f
142344fa8: 44 09 ca                    	or	edx, r9d
142344fab: eb 10                       	jmp	0x142344fbd <.text+0x2343fbd>
142344fad: 49 83 c0 fe                 	add	r8, -0x2
142344fb1: 83 e1 1f                    	and	ecx, 0x1f
142344fb4: eb 0f                       	jmp	0x142344fc5 <.text+0x2343fc5>
142344fb6: 49 83 c0 fd                 	add	r8, -0x3
142344fba: 83 e2 0f                    	and	edx, 0xf
142344fbd: c1 e2 06                    	shl	edx, 0x6
142344fc0: 83 e1 3f                    	and	ecx, 0x3f
142344fc3: 09 d1                       	or	ecx, edx
142344fc5: c1 e1 06                    	shl	ecx, 0x6
142344fc8: 24 3f                       	and	al, 0x3f
142344fca: 0f b6 c0                    	movzx	eax, al
142344fcd: 09 c8                       	or	eax, ecx
142344fcf: 83 f8 5c                    	cmp	eax, 0x5c
142344fd2: 74 1a                       	je	0x142344fee <.text+0x2343fee>
142344fd4: 83 f8 2f                    	cmp	eax, 0x2f
142344fd7: 74 15                       	je	0x142344fee <.text+0x2343fee>
142344fd9: 4d 39 e8                    	cmp	r8, r13
142344fdc: 75 82                       	jne	0x142344f60 <.text+0x2343f60>
142344fde: 4d 89 f7                    	mov	r15, r14
142344fe1: 49 89 de                    	mov	r14, rbx
142344fe4: 48 89 fb                    	mov	rbx, rdi
142344fe7: bf 2d 00 00 00              	mov	edi, 0x2d
142344fec: eb 55                       	jmp	0x142345043 <.text+0x2344043>
142344fee: 4c 8d 2d 43 96 d0 00        	lea	r13, [rip + 0xd09643]   # 0x14304e638
142344ff5: 4d 29 e8                    	sub	r8, r13
142344ff8: 49 ff c0                    	inc	r8
142344ffb: 74 32                       	je	0x14234502f <.text+0x234402f>
142344ffd: 49 83 f8 2d                 	cmp	r8, 0x2d
142345001: 73 2a                       	jae	0x14234502d <.text+0x234402d>
142345003: 43 80 3c 28 bf              	cmp	byte ptr [r8 + r13], -0x41
142345008: 7f 25                       	jg	0x14234502f <.text+0x234402f>
14234500a: 48 8d 05 bf 95 d0 00        	lea	rax, [rip + 0xd095bf]   # 0x14304e5d0
142345011: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
142345016: 48 8d 0d 1b 96 d0 00        	lea	rcx, [rip + 0xd0961b]   # 0x14304e638
14234501d: ba 2d 00 00 00              	mov	edx, 0x2d
142345022: 41 b9 2d 00 00 00           	mov	r9d, 0x2d
142345028: e8 13 49 59 00              	call	0x1428d9940 <.text+0x28d8940>
14234502d: 75 db                       	jne	0x14234500a <.text+0x234400a>
14234502f: 4d 89 f7                    	mov	r15, r14
142345032: 49 89 de                    	mov	r14, rbx
142345035: 48 89 fb                    	mov	rbx, rdi
142345038: bf 2d 00 00 00              	mov	edi, 0x2d
14234503d: 4c 29 c7                    	sub	rdi, r8
142345040: 4d 01 c5                    	add	r13, r8
142345043: e8 e8 30 8d fe              	call	0x140c18130 <.text+0xc17130>
142345048: 48 89 c1                    	mov	rcx, rax
14234504b: 48 8b 00                    	mov	rax, qword ptr [rax]
14234504e: 48 83 f8 01                 	cmp	rax, 0x1
142345052: 0f 85 ba 03 00 00           	jne	0x142345412 <.text+0x2344412>
142345058: 48 83 c1 08                 	add	rcx, 0x8
14234505c: 48 83 39 00                 	cmp	qword ptr [rcx], 0x0
142345060: 0f 85 11 04 00 00           	jne	0x142345477 <.text+0x2344477>
142345066: 48 c7 01 ff ff ff ff        	mov	qword ptr [rcx], -0x1
14234506d: 48 89 8d 28 02 00 00        	mov	qword ptr [rbp + 0x228], rcx
142345074: 48 83 c1 08                 	add	rcx, 0x8
142345078: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
14234507d: 48 c7 44 24 30 00 00 00 00  	mov	qword ptr [rsp + 0x30], 0x0
142345086: 48 c7 44 24 28 01 00 00 00  	mov	qword ptr [rsp + 0x28], 0x1
14234508f: 4c 89 e2                    	mov	rdx, r12
142345092: 49 89 f0                    	mov	r8, rsi
142345095: 4d 89 e9                    	mov	r9, r13
142345098: e8 a3 1c 8d fe              	call	0x140c16d40 <.text+0xc15d40>
14234509d: 48 89 c6                    	mov	rsi, rax
1423450a0: 48 8b 85 28 02 00 00        	mov	rax, qword ptr [rbp + 0x228]
1423450a7: 48 ff 00                    	inc	qword ptr [rax]
1423450aa: 48 89 b5 00 02 00 00        	mov	qword ptr [rbp + 0x200], rsi
1423450b1: 48 c7 85 f8 01 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x1f8], 0x1
1423450bc: 48 8b 03                    	mov	rax, qword ptr [rbx]
1423450bf: 80 b8 f7 00 00 00 00        	cmp	byte ptr [rax + 0xf7], 0x0
1423450c6: 0f 84 c5 02 00 00           	je	0x142345391 <.text+0x2344391>
1423450cc: 48 89 b5 10 02 00 00        	mov	qword ptr [rbp + 0x210], rsi
1423450d3: 48 8b b5 18 02 00 00        	mov	rsi, qword ptr [rbp + 0x218]
1423450da: 4c 89 f3                    	mov	rbx, r14
1423450dd: 4d 89 fe                    	mov	r14, r15
1423450e0: 4c 8b bd 20 02 00 00        	mov	r15, qword ptr [rbp + 0x220]
1423450e7: 48 8b 85 08 03 00 00        	mov	rax, qword ptr [rbp + 0x308]
1423450ee: 49 8b 0f                    	mov	rcx, qword ptr [r15]
1423450f1: 49 8b 16                    	mov	rdx, qword ptr [r14]
1423450f4: 48 89 8d a0 01 00 00        	mov	qword ptr [rbp + 0x1a0], rcx
1423450fb: 48 89 95 a8 01 00 00        	mov	qword ptr [rbp + 0x1a8], rdx
142345102: 48 c7 85 b8 01 00 00 02 00 00 00    	mov	qword ptr [rbp + 0x1b8], 0x2
14234510d: 48 8b 0b                    	mov	rcx, qword ptr [rbx]
142345110: 48 89 8d 28 02 00 00        	mov	qword ptr [rbp + 0x228], rcx
142345117: 48 8b 00                    	mov	rax, qword ptr [rax]
14234511a: 48 89 85 18 02 00 00        	mov	qword ptr [rbp + 0x218], rax
142345121: 48 8b 06                    	mov	rax, qword ptr [rsi]
142345124: 48 89 85 20 02 00 00        	mov	qword ptr [rbp + 0x220], rax
14234512b: 31 c0                       	xor	eax, eax
14234512d: 4c 8d b5 d8 00 00 00        	lea	r14, [rbp + 0xd8]
142345134: 4c 8d 65 20                 	lea	r12, [rbp + 0x20]
142345138: f3 0f 10 35 68 f4 5e 00     	movss	xmm6, dword ptr [rip + 0x5ef468] # 0x1429345a8
142345140: 0f 28 3d 99 07 5e 00        	movaps	xmm7, xmmword ptr [rip + 0x5e0799] # 0x1429258e0
142345147: 4c 8d 6d c0                 	lea	r13, [rbp - 0x40]
14234514b: eb 06                       	jmp	0x142345153 <.text+0x2344153>
14234514d: 0f 1f 00                    	nop	dword ptr [rax]
142345150: 4c 89 f8                    	mov	rax, r15
142345153: 48 83 f8 02                 	cmp	rax, 0x2
142345157: 0f 84 e4 01 00 00           	je	0x142345341 <.text+0x2344341>
14234515d: 4c 8d 78 01                 	lea	r15, [rax + 0x1]
142345161: 4c 89 bd b0 01 00 00        	mov	qword ptr [rbp + 0x1b0], r15
142345168: 48 8b 84 c5 a0 01 00 00     	mov	rax, qword ptr [rbp + 8*rax + 0x1a0]
142345170: 48 8b 48 20                 	mov	rcx, qword ptr [rax + 0x20]
142345174: 48 89 8d 08 02 00 00        	mov	qword ptr [rbp + 0x208], rcx
14234517b: 48 8b 58 08                 	mov	rbx, qword ptr [rax + 0x8]
14234517f: 48 8b 70 10                 	mov	rsi, qword ptr [rax + 0x10]
142345183: 48 c1 e6 05                 	shl	rsi, 0x5
142345187: 48 01 de                    	add	rsi, rbx
14234518a: 48 8d 85 08 02 00 00        	lea	rax, [rbp + 0x208]
142345191: 48 85 c0                    	test	rax, rax
142345194: 0f 94 c1                    	sete	cl
142345197: 48 8d 95 10 02 00 00        	lea	rdx, [rbp + 0x210]
14234519e: 48 39 d0                    	cmp	rax, rdx
1423451a1: 0f 94 c2                    	sete	dl
1423451a4: 08 ca                       	or	dl, cl
1423451a6: 74 28                       	je	0x1423451d0 <.text+0x23441d0>
1423451a8: 48 85 db                    	test	rbx, rbx
1423451ab: 74 a3                       	je	0x142345150 <.text+0x2344150>
1423451ad: 0f 1f 00                    	nop	dword ptr [rax]
1423451b0: 48 39 f3                    	cmp	rbx, rsi
1423451b3: 74 9b                       	je	0x142345150 <.text+0x2344150>
1423451b5: 48 8d 4b 20                 	lea	rcx, [rbx + 0x20]
1423451b9: 83 7b 10 00                 	cmp	dword ptr [rbx + 0x10], 0x0
1423451bd: 48 89 cb                    	mov	rbx, rcx
1423451c0: 74 ee                       	je	0x1423451b0 <.text+0x23441b0>
1423451c2: 48 8d 41 f0                 	lea	rax, [rcx - 0x10]
1423451c6: 48 89 cb                    	mov	rbx, rcx
1423451c9: 31 ff                       	xor	edi, edi
1423451cb: eb 07                       	jmp	0x1423451d4 <.text+0x23441d4>
1423451cd: 0f 1f 00                    	nop	dword ptr [rax]
1423451d0: 48 8d 78 08                 	lea	rdi, [rax + 0x8]
1423451d4: 48 89 85 f0 01 00 00        	mov	qword ptr [rbp + 0x1f0], rax
1423451db: 44 8b 00                    	mov	r8d, dword ptr [rax]
1423451de: 44 8b 48 04                 	mov	r9d, dword ptr [rax + 0x4]
1423451e2: 4c 89 f1                    	mov	rcx, r14
1423451e5: 48 8b 95 28 02 00 00        	mov	rdx, qword ptr [rbp + 0x228]
1423451ec: e8 0f 2b ca fe              	call	0x140fe7d00 <.text+0xfe6d00>
1423451f1: 83 bd d8 00 00 00 04        	cmp	dword ptr [rbp + 0xd8], 0x4
1423451f8: 0f 85 85 02 00 00           	jne	0x142345483 <.text+0x2344483>
1423451fe: 48 8b 85 e0 00 00 00        	mov	rax, qword ptr [rbp + 0xe0]
142345205: f2 44 0f 10 08              	movsd	xmm9, qword ptr [rax]
14234520a: f2 44 0f 10 40 08           	movsd	xmm8, qword ptr [rax + 0x8]
142345210: 48 8b 85 18 02 00 00        	mov	rax, qword ptr [rbp + 0x218]
142345217: f3 44 0f 10 10              	movss	xmm10, dword ptr [rax]
14234521c: 48 8d 85 f0 01 00 00        	lea	rax, [rbp + 0x1f0]
142345223: 48 89 85 d8 00 00 00        	mov	qword ptr [rbp + 0xd8], rax
14234522a: 48 8d 05 ff a7 b1 ff        	lea	rax, [rip - 0x4e5801]   # 0x141e5fa30 <.text+0x1e5ea30>
142345231: 48 89 85 e0 00 00 00        	mov	qword ptr [rbp + 0xe0], rax
142345238: 48 8d 05 59 94 d0 00        	lea	rax, [rip + 0xd09459]   # 0x14304e698
14234523f: 48 89 45 20                 	mov	qword ptr [rbp + 0x20], rax
142345243: 48 c7 45 28 01 00 00 00     	mov	qword ptr [rbp + 0x28], 0x1
14234524b: 4c 89 75 30                 	mov	qword ptr [rbp + 0x30], r14
14234524f: 48 c7 45 38 01 00 00 00     	mov	qword ptr [rbp + 0x38], 0x1
142345257: 48 c7 45 40 00 00 00 00     	mov	qword ptr [rbp + 0x40], 0x0
14234525f: 48 8d 8d 88 01 00 00        	lea	rcx, [rbp + 0x188]
142345266: 4c 89 e2                    	mov	rdx, r12
142345269: e8 22 4d a3 fe              	call	0x140d79f90 <.text+0xd78f90>
14234526e: f3 44 0f 59 d6              	mulss	xmm10, xmm6
142345273: f3 45 0f 58 d2              	addss	xmm10, xmm10
142345278: 0f 10 85 88 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x188]
14234527f: 0f 29 85 c0 01 00 00        	movaps	xmmword ptr [rbp + 0x1c0], xmm0
142345286: 48 8b 85 98 01 00 00        	mov	rax, qword ptr [rbp + 0x198]
14234528d: 48 89 85 d0 01 00 00        	mov	qword ptr [rbp + 0x1d0], rax
142345294: f3 41 0f 12 c2              	movsldup	xmm0, xmm10             # xmm0 = xmm10[0,0,2,2]
142345299: 44 0f 58 c8                 	addps	xmm9, xmm0
14234529d: 44 0f 59 cf                 	mulps	xmm9, xmm7
1423452a1: 45 0f 58 c1                 	addps	xmm8, xmm9
1423452a5: 41 0f 28 c0                 	movaps	xmm0, xmm8
1423452a9: 41 0f 5c c1                 	subps	xmm0, xmm9
1423452ad: 45 0f 58 c1                 	addps	xmm8, xmm9
1423452b1: 41 0f 16 c0                 	movlhps	xmm0, xmm8              # xmm0 = xmm0[0],xmm8[0]
1423452b5: 0f 11 85 e0 00 00 00        	movups	xmmword ptr [rbp + 0xe0], xmm0
1423452bc: c7 85 dc 00 00 00 0a d7 a3 3c       	mov	dword ptr [rbp + 0xdc], 0x3ca3d70a
1423452c6: c6 85 d8 00 00 00 03        	mov	byte ptr [rbp + 0xd8], 0x3
1423452cd: c6 45 c0 00                 	mov	byte ptr [rbp - 0x40], 0x0
1423452d1: c6 85 37 02 00 00 01        	mov	byte ptr [rbp + 0x237], 0x1
1423452d8: 4c 89 e1                    	mov	rcx, r12
1423452db: 4c 89 f2                    	mov	rdx, r14
1423452de: 4d 89 e8                    	mov	r8, r13
1423452e1: e8 2a 3b 66 fe              	call	0x1409a8e10 <.text+0x9a7e10>
1423452e6: c6 85 37 02 00 00 01        	mov	byte ptr [rbp + 0x237], 0x1
1423452ed: 4c 89 f1                    	mov	rcx, r14
1423452f0: 4c 89 e2                    	mov	rdx, r12
1423452f3: e8 18 3c 66 fe              	call	0x1409a8f10 <.text+0x9a7f10>
1423452f8: c6 85 35 02 00 00 01        	mov	byte ptr [rbp + 0x235], 0x1
1423452ff: 4c 89 e9                    	mov	rcx, r13
142345302: e8 a9 3c 66 fe              	call	0x1409a8fb0 <.text+0x9a7fb0>
142345307: c6 85 35 02 00 00 00        	mov	byte ptr [rbp + 0x235], 0x0
14234530e: 4c 89 e1                    	mov	rcx, r12
142345311: 4c 89 f2                    	mov	rdx, r14
142345314: 4d 89 e8                    	mov	r8, r13
142345317: e8 64 3b 66 fe              	call	0x1409a8e80 <.text+0x9a7e80>
14234531c: c6 85 37 02 00 00 00        	mov	byte ptr [rbp + 0x237], 0x0
142345323: 48 8b 8d 20 02 00 00        	mov	rcx, qword ptr [rbp + 0x220]
14234532a: 48 8d 95 c0 01 00 00        	lea	rdx, [rbp + 0x1c0]
142345331: 4d 89 e0                    	mov	r8, r12
142345334: e8 b7 de 5d fe              	call	0x1409231f0 <.text+0x9221f0>
142345339: 48 89 f8                    	mov	rax, rdi
14234533c: e9 50 fe ff ff              	jmp	0x142345191 <.text+0x2344191>
142345341: 80 bd 36 02 00 00 00        	cmp	byte ptr [rbp + 0x236], 0x0
142345348: 0f 84 8a 00 00 00           	je	0x1423453d8 <.text+0x23443d8>
14234534e: e8 dd 2d 8d fe              	call	0x140c18130 <.text+0xc17130>
142345353: 48 89 c1                    	mov	rcx, rax
142345356: 48 8b 00                    	mov	rax, qword ptr [rax]
142345359: 48 83 f8 01                 	cmp	rax, 0x1
14234535d: 0f 85 e3 00 00 00           	jne	0x142345446 <.text+0x2344446>
142345363: 48 83 c1 08                 	add	rcx, 0x8
142345367: 48 8b 95 10 02 00 00        	mov	rdx, qword ptr [rbp + 0x210]
14234536e: 48 83 39 00                 	cmp	qword ptr [rcx], 0x0
142345372: 0f 85 ff 00 00 00           	jne	0x142345477 <.text+0x2344477>
142345378: 48 c7 01 ff ff ff ff        	mov	qword ptr [rcx], -0x1
14234537f: 48 89 8d 28 02 00 00        	mov	qword ptr [rbp + 0x228], rcx
142345386: 48 83 c1 08                 	add	rcx, 0x8
14234538a: e8 61 1d 8d fe              	call	0x140c170f0 <.text+0xc160f0>
14234538f: eb 3d                       	jmp	0x1423453ce <.text+0x23443ce>
142345391: e8 9a 2d 8d fe              	call	0x140c18130 <.text+0xc17130>
142345396: 48 89 c1                    	mov	rcx, rax
142345399: 48 8b 00                    	mov	rax, qword ptr [rax]
14234539c: 48 83 f8 01                 	cmp	rax, 0x1
1423453a0: 0f 85 86 00 00 00           	jne	0x14234542c <.text+0x234442c>
1423453a6: 48 83 c1 08                 	add	rcx, 0x8
1423453aa: 48 83 39 00                 	cmp	qword ptr [rcx], 0x0
1423453ae: 0f 85 c3 00 00 00           	jne	0x142345477 <.text+0x2344477>
1423453b4: 48 c7 01 ff ff ff ff        	mov	qword ptr [rcx], -0x1
1423453bb: 48 89 8d 28 02 00 00        	mov	qword ptr [rbp + 0x228], rcx
1423453c2: 48 83 c1 08                 	add	rcx, 0x8
1423453c6: 48 89 f2                    	mov	rdx, rsi
1423453c9: e8 22 1d 8d fe              	call	0x140c170f0 <.text+0xc160f0>
1423453ce: 48 8b 85 28 02 00 00        	mov	rax, qword ptr [rbp + 0x228]
1423453d5: 48 ff 00                    	inc	qword ptr [rax]
1423453d8: 0f 28 b5 40 02 00 00        	movaps	xmm6, xmmword ptr [rbp + 0x240]
1423453df: 0f 28 bd 50 02 00 00        	movaps	xmm7, xmmword ptr [rbp + 0x250]
1423453e6: 44 0f 28 85 60 02 00 00     	movaps	xmm8, xmmword ptr [rbp + 0x260]
1423453ee: 44 0f 28 8d 70 02 00 00     	movaps	xmm9, xmmword ptr [rbp + 0x270]
1423453f6: 44 0f 28 95 80 02 00 00     	movaps	xmm10, xmmword ptr [rbp + 0x280]
1423453fe: 48 81 c4 18 03 00 00        	add	rsp, 0x318
142345405: 5b                          	pop	rbx
142345406: 5f                          	pop	rdi
142345407: 5e                          	pop	rsi
142345408: 41 5c                       	pop	r12
14234540a: 41 5d                       	pop	r13
14234540c: 41 5e                       	pop	r14
14234540e: 41 5f                       	pop	r15
142345410: 5d                          	pop	rbp
142345411: c3                          	ret
142345412: 48 85 c0                    	test	rax, rax
142345415: 75 54                       	jne	0x14234546b <.text+0x234446b>
142345417: 31 d2                       	xor	edx, edx
142345419: e8 32 0e 5a 00              	call	0x1428e6250 <.text+0x28e5250>
14234541e: 48 89 c1                    	mov	rcx, rax
142345421: 48 85 c0                    	test	rax, rax
142345424: 0f 85 32 fc ff ff           	jne	0x14234505c <.text+0x234405c>
14234542a: eb 3f                       	jmp	0x14234546b <.text+0x234446b>
14234542c: 48 85 c0                    	test	rax, rax
14234542f: 75 3a                       	jne	0x14234546b <.text+0x234446b>
142345431: 31 d2                       	xor	edx, edx
142345433: e8 18 0e 5a 00              	call	0x1428e6250 <.text+0x28e5250>
142345438: 48 89 c1                    	mov	rcx, rax
14234543b: 48 85 c0                    	test	rax, rax
14234543e: 0f 85 66 ff ff ff           	jne	0x1423453aa <.text+0x23443aa>
142345444: eb 25                       	jmp	0x14234546b <.text+0x234446b>
142345446: 48 85 c0                    	test	rax, rax
142345449: 48 8b 85 10 02 00 00        	mov	rax, qword ptr [rbp + 0x210]
142345450: 75 19                       	jne	0x14234546b <.text+0x234446b>
142345452: 48 89 c6                    	mov	rsi, rax
142345455: 31 d2                       	xor	edx, edx
142345457: e8 f4 0d 5a 00              	call	0x1428e6250 <.text+0x28e5250>
14234545c: 48 89 f2                    	mov	rdx, rsi
14234545f: 48 89 c1                    	mov	rcx, rax
142345462: 48 85 c0                    	test	rax, rax
142345465: 0f 85 03 ff ff ff           	jne	0x14234536e <.text+0x234436e>
14234546b: 48 8d 0d 66 fe 86 00        	lea	rcx, [rip + 0x86fe66]   # 0x142bb52d8
142345472: e8 59 2c 59 00              	call	0x1428d80d0 <.text+0x28d70d0>
142345477: 48 8d 0d ca fe 86 00        	lea	rcx, [rip + 0x86feca]   # 0x142bb5348
14234547e: e8 cd 3d 59 00              	call	0x1428d9250 <.text+0x28d8250>
142345483: 0f 10 85 d8 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xd8]
14234548a: 0f 10 8d e8 00 00 00        	movups	xmm1, xmmword ptr [rbp + 0xe8]
142345491: 0f 29 4d 30                 	movaps	xmmword ptr [rbp + 0x30], xmm1
142345495: 0f 29 45 20                 	movaps	xmmword ptr [rbp + 0x20], xmm0
142345499: 48 8d 05 d0 91 d0 00        	lea	rax, [rip + 0xd091d0]   # 0x14304e670
1423454a0: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1423454a5: 48 8d 0d 44 8f d0 00        	lea	rcx, [rip + 0xd08f44]   # 0x14304e3f0
1423454ac: 4c 8d 0d 1d 8f d0 00        	lea	r9, [rip + 0xd08f1d]    # 0x14304e3d0
1423454b3: 4c 8d 45 20                 	lea	r8, [rbp + 0x20]
1423454b7: ba 2b 00 00 00              	mov	edx, 0x2b
1423454bc: e8 9f 42 59 00              	call	0x1428d9760 <.text+0x28d8760>
1423454c1: 0f 0b                       	ud2
1423454c3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
1423454d0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1423454d5: 55                          	push	rbp
1423454d6: 41 57                       	push	r15
1423454d8: 41 56                       	push	r14
1423454da: 41 55                       	push	r13
1423454dc: 41 54                       	push	r12
1423454de: 56                          	push	rsi
1423454df: 57                          	push	rdi
1423454e0: 53                          	push	rbx
1423454e1: 48 81 ec 88 00 00 00        	sub	rsp, 0x88
1423454e8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1423454ef: 44 0f 29 54 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm10
1423454f5: 44 0f 29 4c 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm9
1423454fb: 44 0f 29 44 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm8
142345501: 0f 29 7c 24 60              	movaps	xmmword ptr [rsp + 0x60], xmm7
142345506: 0f 29 74 24 70              	movaps	xmmword ptr [rsp + 0x70], xmm6
14234550b: 80 bd 36 02 00 00 00        	cmp	byte ptr [rbp + 0x236], 0x0
142345512: 74 2f                       	je	0x142345543 <.text+0x2344543>
142345514: 48 8d 85 00 02 00 00        	lea	rax, [rbp + 0x200]
14234551b: 48 89 85 e8 01 00 00        	mov	qword ptr [rbp + 0x1e8], rax
142345522: 48 8d 85 e8 01 00 00        	lea	rax, [rbp + 0x1e8]
142345529: 48 89 85 e0 01 00 00        	mov	qword ptr [rbp + 0x1e0], rax
142345530: 48 8d 0d 39 90 d0 00        	lea	rcx, [rip + 0xd09039]   # 0x14304e570
142345537: 48 8d 95 e0 01 00 00        	lea	rdx, [rbp + 0x1e0]
14234553e: e8 dd a6 c8 fe              	call	0x140fcfc20 <.text+0xfcec20>
142345543: 0f 28 74 24 70              	movaps	xmm6, xmmword ptr [rsp + 0x70]
142345548: 0f 28 7c 24 60              	movaps	xmm7, xmmword ptr [rsp + 0x60]
14234554d: 44 0f 28 44 24 50           	movaps	xmm8, xmmword ptr [rsp + 0x50]
142345553: 44 0f 28 4c 24 40           	movaps	xmm9, xmmword ptr [rsp + 0x40]
142345559: 44 0f 28 54 24 30           	movaps	xmm10, xmmword ptr [rsp + 0x30]
14234555f: 48 81 c4 88 00 00 00        	add	rsp, 0x88
142345566: 5b                          	pop	rbx
142345567: 5f                          	pop	rdi
142345568: 5e                          	pop	rsi
142345569: 41 5c                       	pop	r12
14234556b: 41 5d                       	pop	r13
14234556d: 41 5e                       	pop	r14
14234556f: 41 5f                       	pop	r15
142345571: 5d                          	pop	rbp
142345572: c3                          	ret
142345573: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
142345580: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
142345585: 55                          	push	rbp
142345586: 41 57                       	push	r15
142345588: 41 56                       	push	r14
14234558a: 41 55                       	push	r13
14234558c: 41 54                       	push	r12
14234558e: 56                          	push	rsi
14234558f: 57                          	push	rdi
142345590: 53                          	push	rbx
142345591: 48 81 ec 88 00 00 00        	sub	rsp, 0x88
142345598: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14234559f: 44 0f 29 54 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm10
1423455a5: 44 0f 29 4c 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm9
1423455ab: 44 0f 29 44 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm8
1423455b1: 0f 29 7c 24 60              	movaps	xmmword ptr [rsp + 0x60], xmm7
1423455b6: 0f 29 74 24 70              	movaps	xmmword ptr [rsp + 0x70], xmm6
1423455bb: 48 8b 85 28 02 00 00        	mov	rax, qword ptr [rbp + 0x228]
1423455c2: 48 ff 00                    	inc	qword ptr [rax]
1423455c5: 0f 28 74 24 70              	movaps	xmm6, xmmword ptr [rsp + 0x70]
1423455ca: 0f 28 7c 24 60              	movaps	xmm7, xmmword ptr [rsp + 0x60]
1423455cf: 44 0f 28 44 24 50           	movaps	xmm8, xmmword ptr [rsp + 0x50]
1423455d5: 44 0f 28 4c 24 40           	movaps	xmm9, xmmword ptr [rsp + 0x40]
1423455db: 44 0f 28 54 24 30           	movaps	xmm10, xmmword ptr [rsp + 0x30]
1423455e1: 48 81 c4 88 00 00 00        	add	rsp, 0x88
1423455e8: 5b                          	pop	rbx
1423455e9: 5f                          	pop	rdi
1423455ea: 5e                          	pop	rsi
1423455eb: 41 5c                       	pop	r12
1423455ed: 41 5d                       	pop	r13
1423455ef: 41 5e                       	pop	r14
1423455f1: 41 5f                       	pop	r15
1423455f3: 5d                          	pop	rbp
1423455f4: c3                          	ret
1423455f5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
142345600: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
142345605: 55                          	push	rbp
142345606: 41 57                       	push	r15
142345608: 41 56                       	push	r14
14234560a: 41 55                       	push	r13
14234560c: 41 54                       	push	r12
14234560e: 56                          	push	rsi
14234560f: 57                          	push	rdi
142345610: 53                          	push	rbx
142345611: 48 81 ec 88 00 00 00        	sub	rsp, 0x88
142345618: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14234561f: 44 0f 29 54 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm10
142345625: 44 0f 29 4c 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm9
14234562b: 44 0f 29 44 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm8
142345631: 0f 29 7c 24 60              	movaps	xmmword ptr [rsp + 0x60], xmm7
142345636: 0f 29 74 24 70              	movaps	xmmword ptr [rsp + 0x70], xmm6
14234563b: 48 8b 85 28 02 00 00        	mov	rax, qword ptr [rbp + 0x228]
142345642: 48 ff 00                    	inc	qword ptr [rax]
142345645: 0f 28 74 24 70              	movaps	xmm6, xmmword ptr [rsp + 0x70]
14234564a: 0f 28 7c 24 60              	movaps	xmm7, xmmword ptr [rsp + 0x60]
14234564f: 44 0f 28 44 24 50           	movaps	xmm8, xmmword ptr [rsp + 0x50]
142345655: 44 0f 28 4c 24 40           	movaps	xmm9, xmmword ptr [rsp + 0x40]
14234565b: 44 0f 28 54 24 30           	movaps	xmm10, xmmword ptr [rsp + 0x30]
142345661: 48 81 c4 88 00 00 00        	add	rsp, 0x88
142345668: 5b                          	pop	rbx
142345669: 5f                          	pop	rdi
14234566a: 5e                          	pop	rsi
14234566b: 41 5c                       	pop	r12
14234566d: 41 5d                       	pop	r13
14234566f: 41 5e                       	pop	r14
142345671: 41 5f                       	pop	r15
142345673: 5d                          	pop	rbp
142345674: c3                          	ret
142345675: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
142345680: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
142345685: 55                          	push	rbp
142345686: 41 57                       	push	r15
142345688: 41 56                       	push	r14
14234568a: 41 55                       	push	r13
14234568c: 41 54                       	push	r12
14234568e: 56                          	push	rsi
14234568f: 57                          	push	rdi
142345690: 53                          	push	rbx
142345691: 48 81 ec 88 00 00 00        	sub	rsp, 0x88
142345698: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14234569f: 44 0f 29 54 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm10
1423456a5: 44 0f 29 4c 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm9
1423456ab: 44 0f 29 44 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm8
1423456b1: 0f 29 7c 24 60              	movaps	xmmword ptr [rsp + 0x60], xmm7
1423456b6: 0f 29 74 24 70              	movaps	xmmword ptr [rsp + 0x70], xmm6
1423456bb: 48 8b 85 28 02 00 00        	mov	rax, qword ptr [rbp + 0x228]
1423456c2: 48 ff 00                    	inc	qword ptr [rax]
1423456c5: 0f 28 74 24 70              	movaps	xmm6, xmmword ptr [rsp + 0x70]
1423456ca: 0f 28 7c 24 60              	movaps	xmm7, xmmword ptr [rsp + 0x60]
1423456cf: 44 0f 28 44 24 50           	movaps	xmm8, xmmword ptr [rsp + 0x50]
1423456d5: 44 0f 28 4c 24 40           	movaps	xmm9, xmmword ptr [rsp + 0x40]
1423456db: 44 0f 28 54 24 30           	movaps	xmm10, xmmword ptr [rsp + 0x30]
1423456e1: 48 81 c4 88 00 00 00        	add	rsp, 0x88
1423456e8: 5b                          	pop	rbx
1423456e9: 5f                          	pop	rdi
1423456ea: 5e                          	pop	rsi
1423456eb: 41 5c                       	pop	r12
1423456ed: 41 5d                       	pop	r13
1423456ef: 41 5e                       	pop	r14
1423456f1: 41 5f                       	pop	r15
1423456f3: 5d                          	pop	rbp
1423456f4: c3                          	ret
1423456f5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
142345700: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
142345705: 55                          	push	rbp
142345706: 41 57                       	push	r15
142345708: 41 56                       	push	r14
14234570a: 41 55                       	push	r13
14234570c: 41 54                       	push	r12
14234570e: 56                          	push	rsi
14234570f: 57                          	push	rdi
142345710: 53                          	push	rbx
142345711: 48 81 ec 88 00 00 00        	sub	rsp, 0x88
142345718: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14234571f: 44 0f 29 54 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm10
142345725: 44 0f 29 4c 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm9
14234572b: 44 0f 29 44 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm8
142345731: 0f 29 7c 24 60              	movaps	xmmword ptr [rsp + 0x60], xmm7
142345736: 0f 29 74 24 70              	movaps	xmmword ptr [rsp + 0x70], xmm6
14234573b: 80 bd 35 02 00 00 00        	cmp	byte ptr [rbp + 0x235], 0x0
142345742: 74 43                       	je	0x142345787 <.text+0x2344787>
142345744: 48 8d 8d d8 00 00 00        	lea	rcx, [rbp + 0xd8]
14234574b: e8 20 fd 54 fe              	call	0x140895470 <.text+0x894470>
142345750: c6 85 37 02 00 00 01        	mov	byte ptr [rbp + 0x237], 0x1
142345757: 0f 28 74 24 70              	movaps	xmm6, xmmword ptr [rsp + 0x70]
14234575c: 0f 28 7c 24 60              	movaps	xmm7, xmmword ptr [rsp + 0x60]
142345761: 44 0f 28 44 24 50           	movaps	xmm8, xmmword ptr [rsp + 0x50]
142345767: 44 0f 28 4c 24 40           	movaps	xmm9, xmmword ptr [rsp + 0x40]
14234576d: 44 0f 28 54 24 30           	movaps	xmm10, xmmword ptr [rsp + 0x30]
142345773: 48 81 c4 88 00 00 00        	add	rsp, 0x88
14234577a: 5b                          	pop	rbx
14234577b: 5f                          	pop	rdi
14234577c: 5e                          	pop	rsi
14234577d: 41 5c                       	pop	r12
14234577f: 41 5d                       	pop	r13
142345781: 41 5e                       	pop	r14
142345783: 41 5f                       	pop	r15
142345785: 5d                          	pop	rbp
142345786: c3                          	ret
142345787: c6 85 37 02 00 00 01        	mov	byte ptr [rbp + 0x237], 0x1
14234578e: 0f 28 74 24 70              	movaps	xmm6, xmmword ptr [rsp + 0x70]
142345793: 0f 28 7c 24 60              	movaps	xmm7, xmmword ptr [rsp + 0x60]
142345798: 44 0f 28 44 24 50           	movaps	xmm8, xmmword ptr [rsp + 0x50]
14234579e: 44 0f 28 4c 24 40           	movaps	xmm9, xmmword ptr [rsp + 0x40]
1423457a4: 44 0f 28 54 24 30           	movaps	xmm10, xmmword ptr [rsp + 0x30]
1423457aa: 48 81 c4 88 00 00 00        	add	rsp, 0x88
1423457b1: 5b                          	pop	rbx
1423457b2: 5f                          	pop	rdi
1423457b3: 5e                          	pop	rsi
1423457b4: 41 5c                       	pop	r12
1423457b6: 41 5d                       	pop	r13
1423457b8: 41 5e                       	pop	r14
1423457ba: 41 5f                       	pop	r15
1423457bc: 5d                          	pop	rbp
1423457bd: c3                          	ret
1423457be: 66 90                       	nop
1423457c0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1423457c5: 55                          	push	rbp
1423457c6: 41 57                       	push	r15
1423457c8: 41 56                       	push	r14
1423457ca: 41 55                       	push	r13
1423457cc: 41 54                       	push	r12
1423457ce: 56                          	push	rsi
1423457cf: 57                          	push	rdi
1423457d0: 53                          	push	rbx
1423457d1: 48 81 ec 88 00 00 00        	sub	rsp, 0x88
1423457d8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1423457df: 44 0f 29 54 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm10
1423457e5: 44 0f 29 4c 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm9
1423457eb: 44 0f 29 44 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm8
1423457f1: 0f 29 7c 24 60              	movaps	xmmword ptr [rsp + 0x60], xmm7
1423457f6: 0f 29 74 24 70              	movaps	xmmword ptr [rsp + 0x70], xmm6
1423457fb: 80 bd 37 02 00 00 00        	cmp	byte ptr [rbp + 0x237], 0x0
142345802: 74 4e                       	je	0x142345852 <.text+0x2344852>
142345804: 48 8b 95 c0 01 00 00        	mov	rdx, qword ptr [rbp + 0x1c0]
14234580b: 48 85 d2                    	test	rdx, rdx
14234580e: 74 12                       	je	0x142345822 <.text+0x2344822>
142345810: 48 8b 8d c8 01 00 00        	mov	rcx, qword ptr [rbp + 0x1c8]
142345817: 41 b8 01 00 00 00           	mov	r8d, 0x1
14234581d: e8 fe e3 2c fe              	call	0x140613c20 <.text+0x612c20>
142345822: 0f 28 74 24 70              	movaps	xmm6, xmmword ptr [rsp + 0x70]
142345827: 0f 28 7c 24 60              	movaps	xmm7, xmmword ptr [rsp + 0x60]
14234582c: 44 0f 28 44 24 50           	movaps	xmm8, xmmword ptr [rsp + 0x50]
142345832: 44 0f 28 4c 24 40           	movaps	xmm9, xmmword ptr [rsp + 0x40]
142345838: 44 0f 28 54 24 30           	movaps	xmm10, xmmword ptr [rsp + 0x30]
14234583e: 48 81 c4 88 00 00 00        	add	rsp, 0x88
142345845: 5b                          	pop	rbx
142345846: 5f                          	pop	rdi
142345847: 5e                          	pop	rsi
142345848: 41 5c                       	pop	r12
14234584a: 41 5d                       	pop	r13
14234584c: 41 5e                       	pop	r14
14234584e: 41 5f                       	pop	r15
142345850: 5d                          	pop	rbp
142345851: c3                          	ret
142345852: 0f 28 74 24 70              	movaps	xmm6, xmmword ptr [rsp + 0x70]
142345857: 0f 28 7c 24 60              	movaps	xmm7, xmmword ptr [rsp + 0x60]
14234585c: 44 0f 28 44 24 50           	movaps	xmm8, xmmword ptr [rsp + 0x50]
142345862: 44 0f 28 4c 24 40           	movaps	xmm9, xmmword ptr [rsp + 0x40]
142345868: 44 0f 28 54 24 30           	movaps	xmm10, xmmword ptr [rsp + 0x30]
14234586e: 48 81 c4 88 00 00 00        	add	rsp, 0x88
142345875: 5b                          	pop	rbx
142345876: 5f                          	pop	rdi
142345877: 5e                          	pop	rsi
142345878: 41 5c                       	pop	r12
14234587a: 41 5d                       	pop	r13
14234587c: 41 5e                       	pop	r14
14234587e: 41 5f                       	pop	r15
142345880: 5d                          	pop	rbp
142345881: c3                          	ret
