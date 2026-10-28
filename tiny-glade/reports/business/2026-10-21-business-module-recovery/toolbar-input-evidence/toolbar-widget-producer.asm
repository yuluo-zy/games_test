
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
142338990: 55                          	push	rbp
142338991: 41 57                       	push	r15
142338993: 41 56                       	push	r14
142338995: 41 55                       	push	r13
142338997: 41 54                       	push	r12
142338999: 56                          	push	rsi
14233899a: 57                          	push	rdi
14233899b: 53                          	push	rbx
14233899c: 48 81 ec 28 04 00 00        	sub	rsp, 0x428
1423389a3: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
1423389ab: 44 0f 29 a5 90 03 00 00     	movaps	xmmword ptr [rbp + 0x390], xmm12
1423389b3: 44 0f 29 9d 80 03 00 00     	movaps	xmmword ptr [rbp + 0x380], xmm11
1423389bb: 44 0f 29 95 70 03 00 00     	movaps	xmmword ptr [rbp + 0x370], xmm10
1423389c3: 44 0f 29 8d 60 03 00 00     	movaps	xmmword ptr [rbp + 0x360], xmm9
1423389cb: 44 0f 29 85 50 03 00 00     	movaps	xmmword ptr [rbp + 0x350], xmm8
1423389d3: 0f 29 bd 40 03 00 00        	movaps	xmmword ptr [rbp + 0x340], xmm7
1423389da: 0f 29 b5 30 03 00 00        	movaps	xmmword ptr [rbp + 0x330], xmm6
1423389e1: 48 c7 85 28 03 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x328], -0x2
1423389ec: 4c 89 cb                    	mov	rbx, r9
1423389ef: 4d 89 c4                    	mov	r12, r8
1423389f2: 49 89 d5                    	mov	r13, rdx
1423389f5: 49 89 cf                    	mov	r15, rcx
1423389f8: e8 03 e0 8d fe              	call	0x140c16a00 <.text+0xc15a00>
1423389fd: 84 c0                       	test	al, al
1423389ff: 74 66                       	je	0x142338a67 <.text+0x2337a67>
142338a01: 48 89 9d 18 03 00 00        	mov	qword ptr [rbp + 0x318], rbx
142338a08: 48 8d 3d f8 4f d1 00        	lea	rdi, [rip + 0xd14ff8]   # 0x14304da07
142338a0f: bb 33 00 00 00              	mov	ebx, 0x33
142338a14: ba 33 00 00 00              	mov	edx, 0x33
142338a19: 48 89 f9                    	mov	rcx, rdi
142338a1c: e8 0f f7 ff ff              	call	0x142338130 <.text+0x2337130>
142338a21: 48 83 f8 01                 	cmp	rax, 0x1
142338a25: 0f 85 b1 00 00 00           	jne	0x142338adc <.text+0x2337adc>
142338a2b: 49 89 d1                    	mov	r9, rdx
142338a2e: 48 85 d2                    	test	rdx, rdx
142338a31: 74 3d                       	je	0x142338a70 <.text+0x2337a70>
142338a33: 49 83 f9 33                 	cmp	r9, 0x33
142338a37: 73 35                       	jae	0x142338a6e <.text+0x2337a6e>
142338a39: 48 8d 05 c7 4f d1 00        	lea	rax, [rip + 0xd14fc7]   # 0x14304da07
142338a40: 41 80 3c 01 bf              	cmp	byte ptr [r9 + rax], -0x41
142338a45: 7f 29                       	jg	0x142338a70 <.text+0x2337a70>
142338a47: 48 8d 05 7a 51 d1 00        	lea	rax, [rip + 0xd1517a]   # 0x14304dbc8
142338a4e: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
142338a53: 48 8d 0d ad 4f d1 00        	lea	rcx, [rip + 0xd14fad]   # 0x14304da07
142338a5a: ba 33 00 00 00              	mov	edx, 0x33
142338a5f: 45 31 c0                    	xor	r8d, r8d
142338a62: e8 d9 0e 5a 00              	call	0x1428d9940 <.text+0x28d8940>
142338a67: 31 c0                       	xor	eax, eax
142338a69: e9 d5 01 00 00              	jmp	0x142338c43 <.text+0x2337c43>
142338a6e: 75 d7                       	jne	0x142338a47 <.text+0x2337a47>
142338a70: 48 8d 3d 90 4f d1 00        	lea	rdi, [rip + 0xd14f90]   # 0x14304da07
142338a77: 48 89 f9                    	mov	rcx, rdi
142338a7a: 4c 89 ca                    	mov	rdx, r9
142338a7d: e8 ae f6 ff ff              	call	0x142338130 <.text+0x2337130>
142338a82: 48 83 f8 01                 	cmp	rax, 0x1
142338a86: 75 54                       	jne	0x142338adc <.text+0x2337adc>
142338a88: 49 89 d0                    	mov	r8, rdx
142338a8b: 49 83 c0 02                 	add	r8, 0x2
142338a8f: 74 39                       	je	0x142338aca <.text+0x2337aca>
142338a91: 49 83 f8 33                 	cmp	r8, 0x33
142338a95: 73 31                       	jae	0x142338ac8 <.text+0x2337ac8>
142338a97: 48 8d 05 69 4f d1 00        	lea	rax, [rip + 0xd14f69]   # 0x14304da07
142338a9e: 41 80 3c 00 bf              	cmp	byte ptr [r8 + rax], -0x41
142338aa3: 7f 25                       	jg	0x142338aca <.text+0x2337aca>
142338aa5: 48 8d 05 34 51 d1 00        	lea	rax, [rip + 0xd15134]   # 0x14304dbe0
142338aac: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
142338ab1: 48 8d 0d 4f 4f d1 00        	lea	rcx, [rip + 0xd14f4f]   # 0x14304da07
142338ab8: ba 33 00 00 00              	mov	edx, 0x33
142338abd: 41 b9 33 00 00 00           	mov	r9d, 0x33
142338ac3: e8 78 0e 5a 00              	call	0x1428d9940 <.text+0x28d8940>
142338ac8: 75 db                       	jne	0x142338aa5 <.text+0x2337aa5>
142338aca: bb 31 00 00 00              	mov	ebx, 0x31
142338acf: 48 29 d3                    	sub	rbx, rdx
142338ad2: 48 8d 3d 2e 4f d1 00        	lea	rdi, [rip + 0xd14f2e]   # 0x14304da07
142338ad9: 4c 01 c7                    	add	rdi, r8
142338adc: 4c 8d 05 21 52 d1 00        	lea	r8, [rip + 0xd15221]    # 0x14304dd04
142338ae3: 4c 8d 35 ee 51 d1 00        	lea	r14, [rip + 0xd151ee]   # 0x14304dcd8
142338aea: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
142338af0: 41 0f be 40 ff              	movsx	eax, byte ptr [r8 - 0x1]
142338af5: 85 c0                       	test	eax, eax
142338af7: 78 17                       	js	0x142338b10 <.text+0x2337b10>
142338af9: 49 ff c8                    	dec	r8
142338afc: 83 f8 5c                    	cmp	eax, 0x5c
142338aff: 75 63                       	jne	0x142338b64 <.text+0x2337b64>
142338b01: eb 76                       	jmp	0x142338b79 <.text+0x2337b79>
142338b03: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
142338b10: 41 0f b6 48 fe              	movzx	ecx, byte ptr [r8 - 0x2]
142338b15: 80 f9 c0                    	cmp	cl, -0x40
142338b18: 7d 23                       	jge	0x142338b3d <.text+0x2337b3d>
142338b1a: 41 0f b6 50 fd              	movzx	edx, byte ptr [r8 - 0x3]
142338b1f: 80 fa c0                    	cmp	dl, -0x40
142338b22: 7d 22                       	jge	0x142338b46 <.text+0x2337b46>
142338b24: 45 0f b6 48 fc              	movzx	r9d, byte ptr [r8 - 0x4]
142338b29: 49 83 c0 fc                 	add	r8, -0x4
142338b2d: 41 83 e1 07                 	and	r9d, 0x7
142338b31: 41 c1 e1 06                 	shl	r9d, 0x6
142338b35: 83 e2 3f                    	and	edx, 0x3f
142338b38: 44 09 ca                    	or	edx, r9d
142338b3b: eb 10                       	jmp	0x142338b4d <.text+0x2337b4d>
142338b3d: 49 83 c0 fe                 	add	r8, -0x2
142338b41: 83 e1 1f                    	and	ecx, 0x1f
142338b44: eb 0f                       	jmp	0x142338b55 <.text+0x2337b55>
142338b46: 49 83 c0 fd                 	add	r8, -0x3
142338b4a: 83 e2 0f                    	and	edx, 0xf
142338b4d: c1 e2 06                    	shl	edx, 0x6
142338b50: 83 e1 3f                    	and	ecx, 0x3f
142338b53: 09 d1                       	or	ecx, edx
142338b55: c1 e1 06                    	shl	ecx, 0x6
142338b58: 24 3f                       	and	al, 0x3f
142338b5a: 0f b6 c0                    	movzx	eax, al
142338b5d: 09 c8                       	or	eax, ecx
142338b5f: 83 f8 5c                    	cmp	eax, 0x5c
142338b62: 74 15                       	je	0x142338b79 <.text+0x2337b79>
142338b64: 83 f8 2f                    	cmp	eax, 0x2f
142338b67: 74 10                       	je	0x142338b79 <.text+0x2337b79>
142338b69: 4d 39 f0                    	cmp	r8, r14
142338b6c: 75 82                       	jne	0x142338af0 <.text+0x2337af0>
142338b6e: 4c 89 ee                    	mov	rsi, r13
142338b71: 41 bd 2c 00 00 00           	mov	r13d, 0x2c
142338b77: eb 50                       	jmp	0x142338bc9 <.text+0x2337bc9>
142338b79: 4c 8d 35 58 51 d1 00        	lea	r14, [rip + 0xd15158]   # 0x14304dcd8
142338b80: 4d 29 f0                    	sub	r8, r14
142338b83: 49 ff c0                    	inc	r8
142338b86: 74 32                       	je	0x142338bba <.text+0x2337bba>
142338b88: 49 83 f8 2c                 	cmp	r8, 0x2c
142338b8c: 73 2a                       	jae	0x142338bb8 <.text+0x2337bb8>
142338b8e: 43 80 3c 30 bf              	cmp	byte ptr [r8 + r14], -0x41
142338b93: 7f 25                       	jg	0x142338bba <.text+0x2337bba>
142338b95: 48 8d 05 0c 50 d1 00        	lea	rax, [rip + 0xd1500c]   # 0x14304dba8
142338b9c: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
142338ba1: 48 8d 0d 30 51 d1 00        	lea	rcx, [rip + 0xd15130]   # 0x14304dcd8
142338ba8: ba 2c 00 00 00              	mov	edx, 0x2c
142338bad: 41 b9 2c 00 00 00           	mov	r9d, 0x2c
142338bb3: e8 88 0d 5a 00              	call	0x1428d9940 <.text+0x28d8940>
142338bb8: 75 db                       	jne	0x142338b95 <.text+0x2337b95>
142338bba: 4c 89 ee                    	mov	rsi, r13
142338bbd: 41 bd 2c 00 00 00           	mov	r13d, 0x2c
142338bc3: 4d 29 c5                    	sub	r13, r8
142338bc6: 4d 01 c6                    	add	r14, r8
142338bc9: e8 62 f5 8d fe              	call	0x140c18130 <.text+0xc17130>
142338bce: 48 89 c1                    	mov	rcx, rax
142338bd1: 48 8b 00                    	mov	rax, qword ptr [rax]
142338bd4: 48 83 f8 01                 	cmp	rax, 0x1
142338bd8: 0f 85 db 0e 00 00           	jne	0x142339ab9 <.text+0x2338ab9>
142338bde: 48 83 c1 08                 	add	rcx, 0x8
142338be2: 48 83 39 00                 	cmp	qword ptr [rcx], 0x0
142338be6: 0f 85 f1 0e 00 00           	jne	0x142339add <.text+0x2338add>
142338bec: 48 c7 01 ff ff ff ff        	mov	qword ptr [rcx], -0x1
142338bf3: 48 89 8d d8 02 00 00        	mov	qword ptr [rbp + 0x2d8], rcx
142338bfa: 48 83 c1 08                 	add	rcx, 0x8
142338bfe: 4c 89 6c 24 20              	mov	qword ptr [rsp + 0x20], r13
142338c03: 48 c7 44 24 30 00 00 00 00  	mov	qword ptr [rsp + 0x30], 0x0
142338c0c: 48 c7 44 24 28 01 00 00 00  	mov	qword ptr [rsp + 0x28], 0x1
142338c15: 48 89 fa                    	mov	rdx, rdi
142338c18: 49 89 d8                    	mov	r8, rbx
142338c1b: 4d 89 f1                    	mov	r9, r14
142338c1e: e8 1d e1 8d fe              	call	0x140c16d40 <.text+0xc15d40>
142338c23: 48 8b 8d d8 02 00 00        	mov	rcx, qword ptr [rbp + 0x2d8]
142338c2a: 48 ff 01                    	inc	qword ptr [rcx]
142338c2d: 48 89 85 48 02 00 00        	mov	qword ptr [rbp + 0x248], rax
142338c34: b8 01 00 00 00              	mov	eax, 0x1
142338c39: 49 89 f5                    	mov	r13, rsi
142338c3c: 48 8b 9d 18 03 00 00        	mov	rbx, qword ptr [rbp + 0x318]
142338c43: 48 89 85 40 02 00 00        	mov	qword ptr [rbp + 0x240], rax
142338c4a: 49 8b 04 24                 	mov	rax, qword ptr [r12]
142338c4e: 48 89 85 b0 02 00 00        	mov	qword ptr [rbp + 0x2b0], rax
142338c55: 48 8d 48 10                 	lea	rcx, [rax + 0x10]
142338c59: 48 89 8d b8 02 00 00        	mov	qword ptr [rbp + 0x2b8], rcx
142338c60: e8 eb 04 67 fe              	call	0x1409a9150 <.text+0x9a8150>
142338c65: 84 c0                       	test	al, al
142338c67: 74 65                       	je	0x142338cce <.text+0x2337cce>
142338c69: 48 8b 85 38 04 00 00        	mov	rax, qword ptr [rbp + 0x438]
142338c70: 48 8b 30                    	mov	rsi, qword ptr [rax]
142338c73: 8b 0e                       	mov	ecx, dword ptr [rsi]
142338c75: f3 0f 10 4e 04              	movss	xmm1, dword ptr [rsi + 0x4]
142338c7a: e8 f1 6d 60 fe              	call	0x14093fa70 <.text+0x93ea70>
142338c7f: 84 c0                       	test	al, al
142338c81: 75 23                       	jne	0x142338ca6 <.text+0x2337ca6>
142338c83: f2 0f 10 86 48 01 00 00     	movsd	xmm0, qword ptr [rsi + 0x148]
142338c8b: 0f 59 c0                    	mulps	xmm0, xmm0
142338c8e: f3 0f 16 c8                 	movshdup	xmm1, xmm0              # xmm1 = xmm0[1,1,3,3]
142338c92: f3 0f 58 c8                 	addss	xmm1, xmm0
142338c96: 0f 57 c0                    	xorps	xmm0, xmm0
142338c99: f3 0f 51 c1                 	sqrtss	xmm0, xmm1
142338c9d: 0f 2e 05 60 cc 5e 00        	ucomiss	xmm0, dword ptr [rip + 0x5ecc60] # 0x142925904
142338ca4: 76 28                       	jbe	0x142338cce <.text+0x2337cce>
142338ca6: 41 8b 44 24 1c              	mov	eax, dword ptr [r12 + 0x1c]
142338cab: 49 8b 4c 24 10              	mov	rcx, qword ptr [r12 + 0x10]
142338cb0: 48 8b 95 b8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2b8]
142338cb7: 0f 10 02                    	movups	xmm0, xmmword ptr [rdx]
142338cba: 48 8b 95 b0 02 00 00        	mov	rdx, qword ptr [rbp + 0x2b0]
142338cc1: 0f 11 02                    	movups	xmmword ptr [rdx], xmm0
142338cc4: 89 01                       	mov	dword ptr [rcx], eax
142338cc6: 48 c7 42 10 00 00 00 00     	mov	qword ptr [rdx + 0x10], 0x0
142338cce: 48 8b 8d b8 02 00 00        	mov	rcx, qword ptr [rbp + 0x2b8]
142338cd5: e8 56 d2 64 fe              	call	0x140985f30 <.text+0x984f30>
142338cda: 88 85 13 03 00 00           	mov	byte ptr [rbp + 0x313], al
142338ce0: 4d 8b 07                    	mov	r8, qword ptr [r15]
142338ce3: 49 8b 57 08                 	mov	rdx, qword ptr [r15 + 0x8]
142338ce7: 45 8b 4f 10                 	mov	r9d, dword ptr [r15 + 0x10]
142338ceb: 41 8b 47 14                 	mov	eax, dword ptr [r15 + 0x14]
142338cef: 48 8d 8d 10 01 00 00        	lea	rcx, [rbp + 0x110]
142338cf6: 89 44 24 20                 	mov	dword ptr [rsp + 0x20], eax
142338cfa: 48 89 95 a8 02 00 00        	mov	qword ptr [rbp + 0x2a8], rdx
142338d01: 4c 89 85 18 03 00 00        	mov	qword ptr [rbp + 0x318], r8
142338d08: e8 13 a6 2e 00              	call	0x142623320 <.text+0x2622320>
142338d0d: 48 8b 95 50 04 00 00        	mov	rdx, qword ptr [rbp + 0x450]
142338d14: 48 8b b5 48 04 00 00        	mov	rsi, qword ptr [rbp + 0x448]
142338d1b: 48 8b 8d 40 04 00 00        	mov	rcx, qword ptr [rbp + 0x440]
142338d22: 4c 8b 9d 30 04 00 00        	mov	r11, qword ptr [rbp + 0x430]
142338d29: 4c 8b 8d 28 04 00 00        	mov	r9, qword ptr [rbp + 0x428]
142338d30: 4c 8b 85 20 04 00 00        	mov	r8, qword ptr [rbp + 0x420]
142338d37: 4c 8b 95 18 04 00 00        	mov	r10, qword ptr [rbp + 0x418]
142338d3e: 48 8b 85 10 04 00 00        	mov	rax, qword ptr [rbp + 0x410]
142338d45: 4c 8b 76 08                 	mov	r14, qword ptr [rsi + 0x8]
142338d49: 48 8b 39                    	mov	rdi, qword ptr [rcx]
142338d4c: 48 89 bd a0 02 00 00        	mov	qword ptr [rbp + 0x2a0], rdi
142338d53: 4c 89 b5 d0 02 00 00        	mov	qword ptr [rbp + 0x2d0], r14
142338d5a: 4d 85 f6                    	test	r14, r14
142338d5d: 75 0b                       	jne	0x142338d6a <.text+0x2337d6a>
142338d5f: 48 8b 76 10                 	mov	rsi, qword ptr [rsi + 0x10]
142338d63: 48 89 b5 d0 02 00 00        	mov	qword ptr [rbp + 0x2d0], rsi
142338d6a: 4d 8b 1b                    	mov	r11, qword ptr [r11]
142338d6d: 4c 89 9d 50 02 00 00        	mov	qword ptr [rbp + 0x250], r11
142338d74: 48 8b 12                    	mov	rdx, qword ptr [rdx]
142338d77: 48 89 95 e0 01 00 00        	mov	qword ptr [rbp + 0x1e0], rdx
142338d7e: 49 8b 55 00                 	mov	rdx, qword ptr [r13]
142338d82: 48 89 95 e8 01 00 00        	mov	qword ptr [rbp + 0x1e8], rdx
142338d89: 49 8b 12                    	mov	rdx, qword ptr [r10]
142338d8c: 48 89 95 00 02 00 00        	mov	qword ptr [rbp + 0x200], rdx
142338d93: 49 8b 11                    	mov	rdx, qword ptr [r9]
142338d96: 48 89 95 28 02 00 00        	mov	qword ptr [rbp + 0x228], rdx
142338d9d: 49 8b 10                    	mov	rdx, qword ptr [r8]
142338da0: 48 89 95 30 02 00 00        	mov	qword ptr [rbp + 0x230], rdx
142338da7: 48 8b 51 10                 	mov	rdx, qword ptr [rcx + 0x10]
142338dab: 48 89 95 08 02 00 00        	mov	qword ptr [rbp + 0x208], rdx
142338db2: 41 8b 54 24 1c              	mov	edx, dword ptr [r12 + 0x1c]
142338db7: 89 95 94 02 00 00           	mov	dword ptr [rbp + 0x294], edx
142338dbd: 49 8b 54 24 10              	mov	rdx, qword ptr [r12 + 0x10]
142338dc2: 48 89 95 38 02 00 00        	mov	qword ptr [rbp + 0x238], rdx
142338dc9: 8b 49 1c                    	mov	ecx, dword ptr [rcx + 0x1c]
142338dcc: 89 8d 90 02 00 00           	mov	dword ptr [rbp + 0x290], ecx
142338dd2: 8b 4b 1c                    	mov	ecx, dword ptr [rbx + 0x1c]
142338dd5: 89 8d 88 02 00 00           	mov	dword ptr [rbp + 0x288], ecx
142338ddb: 48 8b 0b                    	mov	rcx, qword ptr [rbx]
142338dde: 48 8b 53 10                 	mov	rdx, qword ptr [rbx + 0x10]
142338de2: 48 89 95 f0 01 00 00        	mov	qword ptr [rbp + 0x1f0], rdx
142338de9: 8b 50 1c                    	mov	edx, dword ptr [rax + 0x1c]
142338dec: 89 95 8c 02 00 00           	mov	dword ptr [rbp + 0x28c], edx
142338df2: 48 8b 10                    	mov	rdx, qword ptr [rax]
142338df5: 48 8b 40 10                 	mov	rax, qword ptr [rax + 0x10]
142338df9: 48 89 85 f8 01 00 00        	mov	qword ptr [rbp + 0x1f8], rax
142338e00: f3 0f 10 35 a0 b7 5f 00     	movss	xmm6, dword ptr [rip + 0x5fb7a0] # 0x1429345a8
142338e08: 0f 28 3d d1 ca 5e 00        	movaps	xmm7, xmmword ptr [rip + 0x5ecad1] # 0x1429258e0
142338e0f: f3 44 0f 10 05 b4 8a 69 00  	movss	xmm8, dword ptr [rip + 0x698ab4] # 0x1429d18cc
142338e18: f2 44 0f 10 0d cf 38 75 00  	movsd	xmm9, qword ptr [rip + 0x7538cf] # 0x142a8c6f0
142338e21: 0f b6 85 a8 01 00 00        	movzx	eax, byte ptr [rbp + 0x1a8]
142338e28: 88 85 12 03 00 00           	mov	byte ptr [rbp + 0x312], al
142338e2e: 48 8b bd 10 01 00 00        	mov	rdi, qword ptr [rbp + 0x110]
142338e35: 48 8b 85 18 01 00 00        	mov	rax, qword ptr [rbp + 0x118]
142338e3c: 48 89 85 70 02 00 00        	mov	qword ptr [rbp + 0x270], rax
142338e43: 48 8b 85 20 01 00 00        	mov	rax, qword ptr [rbp + 0x120]
142338e4a: 48 89 85 78 02 00 00        	mov	qword ptr [rbp + 0x278], rax
142338e51: 48 8b 85 30 01 00 00        	mov	rax, qword ptr [rbp + 0x130]
142338e58: 48 89 85 80 02 00 00        	mov	qword ptr [rbp + 0x280], rax
142338e5f: 48 8b b5 40 01 00 00        	mov	rsi, qword ptr [rbp + 0x140]
142338e66: 4c 8b 85 48 01 00 00        	mov	r8, qword ptr [rbp + 0x148]
142338e6d: 48 8b 85 50 01 00 00        	mov	rax, qword ptr [rbp + 0x150]
142338e74: 48 89 85 e0 02 00 00        	mov	qword ptr [rbp + 0x2e0], rax
142338e7b: 0f b6 85 58 01 00 00        	movzx	eax, byte ptr [rbp + 0x158]
142338e82: 89 85 14 03 00 00           	mov	dword ptr [rbp + 0x314], eax
142338e88: 48 8b 85 60 01 00 00        	mov	rax, qword ptr [rbp + 0x160]
142338e8f: 48 89 85 f0 02 00 00        	mov	qword ptr [rbp + 0x2f0], rax
142338e96: 48 8b 85 68 01 00 00        	mov	rax, qword ptr [rbp + 0x168]
142338e9d: 48 89 85 f8 02 00 00        	mov	qword ptr [rbp + 0x2f8], rax
142338ea4: 48 8b 85 70 01 00 00        	mov	rax, qword ptr [rbp + 0x170]
142338eab: 48 89 85 00 03 00 00        	mov	qword ptr [rbp + 0x300], rax
142338eb2: 48 8b 85 78 01 00 00        	mov	rax, qword ptr [rbp + 0x178]
142338eb9: 48 89 85 c0 02 00 00        	mov	qword ptr [rbp + 0x2c0], rax
142338ec0: 48 8b 85 88 01 00 00        	mov	rax, qword ptr [rbp + 0x188]
142338ec7: 48 89 85 c8 02 00 00        	mov	qword ptr [rbp + 0x2c8], rax
142338ece: 8b 85 94 01 00 00           	mov	eax, dword ptr [rbp + 0x194]
142338ed4: 89 85 ec 02 00 00           	mov	dword ptr [rbp + 0x2ec], eax
142338eda: 0f b6 85 a0 01 00 00        	movzx	eax, byte ptr [rbp + 0x1a0]
142338ee1: 89 85 0c 03 00 00           	mov	dword ptr [rbp + 0x30c], eax
142338ee7: 48 8b 85 a0 02 00 00        	mov	rax, qword ptr [rbp + 0x2a0]
142338eee: 48 83 c0 20                 	add	rax, 0x20
142338ef2: 48 89 85 20 02 00 00        	mov	qword ptr [rbp + 0x220], rax
142338ef9: 48 89 8d 60 02 00 00        	mov	qword ptr [rbp + 0x260], rcx
142338f00: 48 8d 41 20                 	lea	rax, [rcx + 0x20]
142338f04: 48 89 85 10 02 00 00        	mov	qword ptr [rbp + 0x210], rax
142338f0b: 48 89 95 68 02 00 00        	mov	qword ptr [rbp + 0x268], rdx
142338f12: 48 8d 42 20                 	lea	rax, [rdx + 0x20]
142338f16: 48 89 85 18 02 00 00        	mov	qword ptr [rbp + 0x218], rax
142338f1d: eb 33                       	jmp	0x142338f52 <.text+0x2337f52>
142338f1f: 90                          	nop
142338f20: 48 8b 95 68 02 00 00        	mov	rdx, qword ptr [rbp + 0x268]
142338f27: 48 8b 42 28                 	mov	rax, qword ptr [rdx + 0x28]
142338f2b: 48 8d 0c 5b                 	lea	rcx, [rbx + 2*rbx]
142338f2f: 4c 89 24 c8                 	mov	qword ptr [rax + 8*rcx], r12
142338f33: 4c 89 7c c8 08              	mov	qword ptr [rax + 8*rcx + 0x8], r15
142338f38: 44 88 74 c8 10              	mov	byte ptr [rax + 8*rcx + 0x10], r14b
142338f3d: 48 ff c3                    	inc	rbx
142338f40: 48 89 5a 30                 	mov	qword ptr [rdx + 0x30], rbx
142338f44: 48 ff 42 40                 	inc	qword ptr [rdx + 0x40]
142338f48: 4c 8b 85 d8 02 00 00        	mov	r8, qword ptr [rbp + 0x2d8]
142338f4f: 49 ff c0                    	inc	r8
142338f52: f6 85 12 03 00 00 01        	test	byte ptr [rbp + 0x312], 0x1
142338f59: 74 55                       	je	0x142338fb0 <.text+0x2337fb0>
142338f5b: 49 39 f0                    	cmp	r8, rsi
142338f5e: 0f 85 b2 03 00 00           	jne	0x142339316 <.text+0x2338316>
142338f64: 48 8b 95 70 02 00 00        	mov	rdx, qword ptr [rbp + 0x270]
142338f6b: 48 39 d7                    	cmp	rdi, rdx
142338f6e: 0f 84 8c 0a 00 00           	je	0x142339a00 <.text+0x2338a00>
142338f74: 48 8b 85 a8 02 00 00        	mov	rax, qword ptr [rbp + 0x2a8]
142338f7b: 48 8b 80 a8 01 00 00        	mov	rax, qword ptr [rax + 0x1a8]
142338f82: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nop	word ptr cs:[rax + rax]
142338f90: 8b 0f                       	mov	ecx, dword ptr [rdi]
142338f92: 48 8d 0c c9                 	lea	rcx, [rcx + 8*rcx]
142338f96: 48 8b 74 c8 10              	mov	rsi, qword ptr [rax + 8*rcx + 0x10]
142338f9b: 48 85 f6                    	test	rsi, rsi
142338f9e: 75 62                       	jne	0x142339002 <.text+0x2338002>
142338fa0: 48 83 c7 04                 	add	rdi, 0x4
142338fa4: 48 39 d7                    	cmp	rdi, rdx
142338fa7: 75 e7                       	jne	0x142338f90 <.text+0x2337f90>
142338fa9: e9 52 0a 00 00              	jmp	0x142339a00 <.text+0x2338a00>
142338fae: 66 90                       	nop
142338fb0: 49 39 f0                    	cmp	r8, rsi
142338fb3: 0f 85 97 04 00 00           	jne	0x142339450 <.text+0x2338450>
142338fb9: 48 8b 95 70 02 00 00        	mov	rdx, qword ptr [rbp + 0x270]
142338fc0: 48 39 d7                    	cmp	rdi, rdx
142338fc3: 0f 84 37 0a 00 00           	je	0x142339a00 <.text+0x2338a00>
142338fc9: 48 8b 85 a8 02 00 00        	mov	rax, qword ptr [rbp + 0x2a8]
142338fd0: 48 8b 80 00 01 00 00        	mov	rax, qword ptr [rax + 0x100]
142338fd7: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
142338fe0: 8b 0f                       	mov	ecx, dword ptr [rdi]
142338fe2: 48 8d 0c 89                 	lea	rcx, [rcx + 4*rcx]
142338fe6: 48 c1 e1 05                 	shl	rcx, 0x5
142338fea: 48 8b 74 08 58              	mov	rsi, qword ptr [rax + rcx + 0x58]
142338fef: 48 85 f6                    	test	rsi, rsi
142338ff2: 75 75                       	jne	0x142339069 <.text+0x2338069>
142338ff4: 48 83 c7 04                 	add	rdi, 0x4
142338ff8: 48 39 d7                    	cmp	rdi, rdx
142338ffb: 75 e3                       	jne	0x142338fe0 <.text+0x2337fe0>
142338ffd: e9 fe 09 00 00              	jmp	0x142339a00 <.text+0x2338a00>
142339002: 48 8d 04 c8                 	lea	rax, [rax + 8*rcx]
142339006: 48 8b 8d 18 03 00 00        	mov	rcx, qword ptr [rbp + 0x318]
14233900d: 48 8b 91 10 01 00 00        	mov	rdx, qword ptr [rcx + 0x110]
142339014: 48 8b 48 40                 	mov	rcx, qword ptr [rax + 0x40]
142339018: 48 39 d1                    	cmp	rcx, rdx
14233901b: 0f 86 cb 00 00 00           	jbe	0x1423390ec <.text+0x23380ec>
142339021: 4c 8b 40 38                 	mov	r8, qword ptr [rax + 0x38]
142339025: 49 8b 14 d0                 	mov	rdx, qword ptr [r8 + 8*rdx]
142339029: 48 85 d2                    	test	rdx, rdx
14233902c: 0f 84 ba 00 00 00           	je	0x1423390ec <.text+0x23380ec>
142339032: 4c 8b 40 18                 	mov	r8, qword ptr [rax + 0x18]
142339036: 48 f7 d2                    	not	rdx
142339039: 48 8d 14 52                 	lea	rdx, [rdx + 2*rdx]
14233903d: 48 c1 e2 04                 	shl	rdx, 0x4
142339041: 49 8b 54 10 10              	mov	rdx, qword ptr [r8 + rdx + 0x10]
142339046: 48 89 95 f0 02 00 00        	mov	qword ptr [rbp + 0x2f0], rdx
14233904d: 48 8b 95 18 03 00 00        	mov	rdx, qword ptr [rbp + 0x318]
142339054: 48 8b 92 18 01 00 00        	mov	rdx, qword ptr [rdx + 0x118]
14233905b: 48 39 d1                    	cmp	rcx, rdx
14233905e: 0f 87 a6 00 00 00           	ja	0x14233910a <.text+0x233810a>
142339064: e9 cb 00 00 00              	jmp	0x142339134 <.text+0x2338134>
142339069: 48 01 c8                    	add	rax, rcx
14233906c: 8b 88 94 00 00 00           	mov	ecx, dword ptr [rax + 0x94]
142339072: 48 8b 95 a8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2a8]
142339079: 4c 8b 82 a8 01 00 00        	mov	r8, qword ptr [rdx + 0x1a8]
142339080: 4c 8d 0c c9                 	lea	r9, [rcx + 8*rcx]
142339084: 4b 8d 14 c8                 	lea	rdx, [r8 + 8*r9]
142339088: 48 8b 8d 18 03 00 00        	mov	rcx, qword ptr [rbp + 0x318]
14233908f: 48 8b 89 10 01 00 00        	mov	rcx, qword ptr [rcx + 0x110]
142339096: 4f 8b 44 c8 40              	mov	r8, qword ptr [r8 + 8*r9 + 0x40]
14233909b: 49 39 c8                    	cmp	r8, rcx
14233909e: 0f 86 fa 00 00 00           	jbe	0x14233919e <.text+0x233819e>
1423390a4: 4c 8b 4a 38                 	mov	r9, qword ptr [rdx + 0x38]
1423390a8: 49 8b 0c c9                 	mov	rcx, qword ptr [r9 + 8*rcx]
1423390ac: 48 85 c9                    	test	rcx, rcx
1423390af: 0f 84 e9 00 00 00           	je	0x14233919e <.text+0x233819e>
1423390b5: 4c 8b 4a 18                 	mov	r9, qword ptr [rdx + 0x18]
1423390b9: 48 f7 d1                    	not	rcx
1423390bc: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
1423390c0: 48 c1 e1 04                 	shl	rcx, 0x4
1423390c4: 49 8b 4c 09 10              	mov	rcx, qword ptr [r9 + rcx + 0x10]
1423390c9: 48 89 8d f0 02 00 00        	mov	qword ptr [rbp + 0x2f0], rcx
1423390d0: 48 8b 8d 18 03 00 00        	mov	rcx, qword ptr [rbp + 0x318]
1423390d7: 48 8b 89 18 01 00 00        	mov	rcx, qword ptr [rcx + 0x118]
1423390de: 49 39 c8                    	cmp	r8, rcx
1423390e1: 0f 87 d5 00 00 00           	ja	0x1423391bc <.text+0x23381bc>
1423390e7: e9 fa 00 00 00              	jmp	0x1423391e6 <.text+0x23381e6>
1423390ec: 48 c7 85 f0 02 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x2f0], 0x0
1423390f7: 48 8b 95 18 03 00 00        	mov	rdx, qword ptr [rbp + 0x318]
1423390fe: 48 8b 92 18 01 00 00        	mov	rdx, qword ptr [rdx + 0x118]
142339105: 48 39 d1                    	cmp	rcx, rdx
142339108: 76 2a                       	jbe	0x142339134 <.text+0x2338134>
14233910a: 4c 8b 40 38                 	mov	r8, qword ptr [rax + 0x38]
14233910e: 49 8b 14 d0                 	mov	rdx, qword ptr [r8 + 8*rdx]
142339112: 48 85 d2                    	test	rdx, rdx
142339115: 74 1d                       	je	0x142339134 <.text+0x2338134>
142339117: 4c 8b 40 18                 	mov	r8, qword ptr [rax + 0x18]
14233911b: 48 f7 d2                    	not	rdx
14233911e: 48 8d 14 52                 	lea	rdx, [rdx + 2*rdx]
142339122: 48 c1 e2 04                 	shl	rdx, 0x4
142339126: 49 8b 54 10 10              	mov	rdx, qword ptr [r8 + rdx + 0x10]
14233912b: 48 89 95 f8 02 00 00        	mov	qword ptr [rbp + 0x2f8], rdx
142339132: eb 0b                       	jmp	0x14233913f <.text+0x233813f>
142339134: 48 c7 85 f8 02 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x2f8], 0x0
14233913f: 48 8b 95 18 03 00 00        	mov	rdx, qword ptr [rbp + 0x318]
142339146: 4c 8b 82 20 01 00 00        	mov	r8, qword ptr [rdx + 0x120]
14233914d: 48 8b 50 38                 	mov	rdx, qword ptr [rax + 0x38]
142339151: 4c 39 c1                    	cmp	rcx, r8
142339154: 0f 86 df 00 00 00           	jbe	0x142339239 <.text+0x2338239>
14233915a: 4e 8b 0c c2                 	mov	r9, qword ptr [rdx + 8*r8]
14233915e: 4c 8b 40 18                 	mov	r8, qword ptr [rax + 0x18]
142339162: 4d 85 c9                    	test	r9, r9
142339165: 0f 84 d2 00 00 00           	je	0x14233923d <.text+0x233823d>
14233916b: 49 f7 d1                    	not	r9
14233916e: 4f 8d 0c 49                 	lea	r9, [r9 + 2*r9]
142339172: 49 c1 e1 04                 	shl	r9, 0x4
142339176: 4f 8b 4c 08 10              	mov	r9, qword ptr [r8 + r9 + 0x10]
14233917b: 4c 89 8d 00 03 00 00        	mov	qword ptr [rbp + 0x300], r9
142339182: 4c 8b 8d 18 03 00 00        	mov	r9, qword ptr [rbp + 0x318]
142339189: 4d 8b 89 30 01 00 00        	mov	r9, qword ptr [r9 + 0x130]
142339190: 4c 39 c9                    	cmp	rcx, r9
142339193: 0f 87 c2 00 00 00           	ja	0x14233925b <.text+0x233825b>
142339199: e9 06 01 00 00              	jmp	0x1423392a4 <.text+0x23382a4>
14233919e: 48 c7 85 f0 02 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x2f0], 0x0
1423391a9: 48 8b 8d 18 03 00 00        	mov	rcx, qword ptr [rbp + 0x318]
1423391b0: 48 8b 89 18 01 00 00        	mov	rcx, qword ptr [rcx + 0x118]
1423391b7: 49 39 c8                    	cmp	r8, rcx
1423391ba: 76 2a                       	jbe	0x1423391e6 <.text+0x23381e6>
1423391bc: 4c 8b 4a 38                 	mov	r9, qword ptr [rdx + 0x38]
1423391c0: 49 8b 0c c9                 	mov	rcx, qword ptr [r9 + 8*rcx]
1423391c4: 48 85 c9                    	test	rcx, rcx
1423391c7: 74 1d                       	je	0x1423391e6 <.text+0x23381e6>
1423391c9: 4c 8b 4a 18                 	mov	r9, qword ptr [rdx + 0x18]
1423391cd: 48 f7 d1                    	not	rcx
1423391d0: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
1423391d4: 48 c1 e1 04                 	shl	rcx, 0x4
1423391d8: 49 8b 4c 09 10              	mov	rcx, qword ptr [r9 + rcx + 0x10]
1423391dd: 48 89 8d f8 02 00 00        	mov	qword ptr [rbp + 0x2f8], rcx
1423391e4: eb 0b                       	jmp	0x1423391f1 <.text+0x23381f1>
1423391e6: 48 c7 85 f8 02 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x2f8], 0x0
1423391f1: 48 8b 8d 18 03 00 00        	mov	rcx, qword ptr [rbp + 0x318]
1423391f8: 4c 8b 89 20 01 00 00        	mov	r9, qword ptr [rcx + 0x120]
1423391ff: 48 8b 4a 38                 	mov	rcx, qword ptr [rdx + 0x38]
142339203: 4d 39 c8                    	cmp	r8, r9
142339206: 0f 86 24 01 00 00           	jbe	0x142339330 <.text+0x2338330>
14233920c: 4e 8b 0c c9                 	mov	r9, qword ptr [rcx + 8*r9]
142339210: 48 8b 52 18                 	mov	rdx, qword ptr [rdx + 0x18]
142339214: 4d 85 c9                    	test	r9, r9
142339217: 0f 84 17 01 00 00           	je	0x142339334 <.text+0x2338334>
14233921d: 49 f7 d1                    	not	r9
142339220: 4f 8d 0c 49                 	lea	r9, [r9 + 2*r9]
142339224: 49 c1 e1 04                 	shl	r9, 0x4
142339228: 4e 8b 4c 0a 10              	mov	r9, qword ptr [rdx + r9 + 0x10]
14233922d: 4c 89 8d 00 03 00 00        	mov	qword ptr [rbp + 0x300], r9
142339234: e9 06 01 00 00              	jmp	0x14233933f <.text+0x233833f>
142339239: 4c 8b 40 18                 	mov	r8, qword ptr [rax + 0x18]
14233923d: 48 c7 85 00 03 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x300], 0x0
142339248: 4c 8b 8d 18 03 00 00        	mov	r9, qword ptr [rbp + 0x318]
14233924f: 4d 8b 89 30 01 00 00        	mov	r9, qword ptr [r9 + 0x130]
142339256: 4c 39 c9                    	cmp	rcx, r9
142339259: 76 49                       	jbe	0x1423392a4 <.text+0x23382a4>
14233925b: 4e 8b 0c ca                 	mov	r9, qword ptr [rdx + 8*r9]
14233925f: 4d 85 c9                    	test	r9, r9
142339262: 74 40                       	je	0x1423392a4 <.text+0x23382a4>
142339264: 49 f7 d1                    	not	r9
142339267: 4f 8d 0c 49                 	lea	r9, [r9 + 2*r9]
14233926b: 49 c1 e1 04                 	shl	r9, 0x4
14233926f: 4f 8b 4c 08 10              	mov	r9, qword ptr [r8 + r9 + 0x10]
142339274: 4c 89 8d e0 02 00 00        	mov	qword ptr [rbp + 0x2e0], r9
14233927b: 41 b1 01                    	mov	r9b, 0x1
14233927e: 44 89 8d 14 03 00 00        	mov	dword ptr [rbp + 0x314], r9d
142339285: 4c 8b 8d 18 03 00 00        	mov	r9, qword ptr [rbp + 0x318]
14233928c: 4d 8b 89 38 01 00 00        	mov	r9, qword ptr [r9 + 0x138]
142339293: 4c 39 c9                    	cmp	rcx, r9
142339296: 77 29                       	ja	0x1423392c1 <.text+0x23382c1>
142339298: c7 85 0c 03 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x30c], 0x0
1423392a2: eb 2b                       	jmp	0x1423392cf <.text+0x23382cf>
1423392a4: c7 85 14 03 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x314], 0x0
1423392ae: 4c 8b 8d 18 03 00 00        	mov	r9, qword ptr [rbp + 0x318]
1423392b5: 4d 8b 89 38 01 00 00        	mov	r9, qword ptr [r9 + 0x138]
1423392bc: 4c 39 c9                    	cmp	rcx, r9
1423392bf: 76 d7                       	jbe	0x142339298 <.text+0x2338298>
1423392c1: 4a 83 3c ca 00              	cmp	qword ptr [rdx + 8*r9], 0x0
1423392c6: 0f 95 c1                    	setne	cl
1423392c9: 89 8d 0c 03 00 00           	mov	dword ptr [rbp + 0x30c], ecx
1423392cf: 48 83 c7 04                 	add	rdi, 0x4
1423392d3: 48 8b 8d 18 03 00 00        	mov	rcx, qword ptr [rbp + 0x318]
1423392da: 48 8b 89 28 01 00 00        	mov	rcx, qword ptr [rcx + 0x128]
1423392e1: 48 8b 0c ca                 	mov	rcx, qword ptr [rdx + 8*rcx]
1423392e5: 48 f7 d1                    	not	rcx
1423392e8: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
1423392ec: 48 c1 e1 04                 	shl	rcx, 0x4
1423392f0: 49 8b 54 08 10              	mov	rdx, qword ptr [r8 + rcx + 0x10]
1423392f5: 48 89 95 c0 02 00 00        	mov	qword ptr [rbp + 0x2c0], rdx
1423392fc: 49 8b 4c 08 28              	mov	rcx, qword ptr [r8 + rcx + 0x28]
142339301: 48 89 8d c8 02 00 00        	mov	qword ptr [rbp + 0x2c8], rcx
142339308: 48 8b 40 08                 	mov	rax, qword ptr [rax + 0x8]
14233930c: 48 89 85 78 02 00 00        	mov	qword ptr [rbp + 0x278], rax
142339313: 45 31 c0                    	xor	r8d, r8d
142339316: 48 8b 85 78 02 00 00        	mov	rax, qword ptr [rbp + 0x278]
14233931d: 4a 8d 04 c0                 	lea	rax, [rax + 8*r8]
142339321: 4c 89 85 d8 02 00 00        	mov	qword ptr [rbp + 0x2d8], r8
142339328: 45 89 c4                    	mov	r12d, r8d
14233932b: e9 3e 01 00 00              	jmp	0x14233946e <.text+0x233846e>
142339330: 48 8b 52 18                 	mov	rdx, qword ptr [rdx + 0x18]
142339334: 48 c7 85 00 03 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x300], 0x0
14233933f: 4c 8b 8d 18 03 00 00        	mov	r9, qword ptr [rbp + 0x318]
142339346: 4d 8b 91 30 01 00 00        	mov	r10, qword ptr [r9 + 0x130]
14233934d: 4c 8b 88 88 00 00 00        	mov	r9, qword ptr [rax + 0x88]
142339354: 4d 39 d1                    	cmp	r9, r10
142339357: 76 52                       	jbe	0x1423393ab <.text+0x23383ab>
142339359: 4c 8b 98 80 00 00 00        	mov	r11, qword ptr [rax + 0x80]
142339360: 4b 83 3c d3 00              	cmp	qword ptr [r11 + 8*r10], 0x0
142339365: 74 44                       	je	0x1423393ab <.text+0x23383ab>
142339367: 41 b3 01                    	mov	r11b, 0x1
14233936a: 44 89 9d 14 03 00 00        	mov	dword ptr [rbp + 0x314], r11d
142339371: 4d 39 d0                    	cmp	r8, r10
142339374: 76 6b                       	jbe	0x1423393e1 <.text+0x23383e1>
142339376: 4e 8b 04 d1                 	mov	r8, qword ptr [rcx + 8*r10]
14233937a: 4d 85 c0                    	test	r8, r8
14233937d: 74 62                       	je	0x1423393e1 <.text+0x23383e1>
14233937f: 49 f7 d0                    	not	r8
142339382: 4f 8d 04 40                 	lea	r8, [r8 + 2*r8]
142339386: 49 c1 e0 04                 	shl	r8, 0x4
14233938a: 4e 8b 44 02 10              	mov	r8, qword ptr [rdx + r8 + 0x10]
14233938f: 4c 89 85 e0 02 00 00        	mov	qword ptr [rbp + 0x2e0], r8
142339396: 4c 8b 85 18 03 00 00        	mov	r8, qword ptr [rbp + 0x318]
14233939d: 4d 8b 80 38 01 00 00        	mov	r8, qword ptr [r8 + 0x138]
1423393a4: 4d 39 c1                    	cmp	r9, r8
1423393a7: 77 1f                       	ja	0x1423393c8 <.text+0x23383c8>
1423393a9: eb 54                       	jmp	0x1423393ff <.text+0x23383ff>
1423393ab: c7 85 14 03 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x314], 0x0
1423393b5: 4c 8b 85 18 03 00 00        	mov	r8, qword ptr [rbp + 0x318]
1423393bc: 4d 8b 80 38 01 00 00        	mov	r8, qword ptr [r8 + 0x138]
1423393c3: 4d 39 c1                    	cmp	r9, r8
1423393c6: 76 37                       	jbe	0x1423393ff <.text+0x23383ff>
1423393c8: 4c 8b 88 80 00 00 00        	mov	r9, qword ptr [rax + 0x80]
1423393cf: 4b 83 3c c1 00              	cmp	qword ptr [r9 + 8*r8], 0x0
1423393d4: 41 0f 95 c0                 	setne	r8b
1423393d8: 44 89 85 0c 03 00 00        	mov	dword ptr [rbp + 0x30c], r8d
1423393df: eb 28                       	jmp	0x142339409 <.text+0x2338409>
1423393e1: 48 c7 85 e0 02 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x2e0], 0x0
1423393ec: 4c 8b 85 18 03 00 00        	mov	r8, qword ptr [rbp + 0x318]
1423393f3: 4d 8b 80 38 01 00 00        	mov	r8, qword ptr [r8 + 0x138]
1423393fa: 4d 39 c1                    	cmp	r9, r8
1423393fd: 77 c9                       	ja	0x1423393c8 <.text+0x23383c8>
1423393ff: c7 85 0c 03 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x30c], 0x0
142339409: 48 83 c7 04                 	add	rdi, 0x4
14233940d: 4c 8b 85 18 03 00 00        	mov	r8, qword ptr [rbp + 0x318]
142339414: 4d 8b 80 28 01 00 00        	mov	r8, qword ptr [r8 + 0x128]
14233941b: 4a 8b 0c c1                 	mov	rcx, qword ptr [rcx + 8*r8]
14233941f: 48 f7 d1                    	not	rcx
142339422: 48 8d 0c 49                 	lea	rcx, [rcx + 2*rcx]
142339426: 48 c1 e1 04                 	shl	rcx, 0x4
14233942a: 4c 8b 44 0a 10              	mov	r8, qword ptr [rdx + rcx + 0x10]
14233942f: 4c 89 85 c0 02 00 00        	mov	qword ptr [rbp + 0x2c0], r8
142339436: 48 8b 4c 0a 28              	mov	rcx, qword ptr [rdx + rcx + 0x28]
14233943b: 48 89 8d c8 02 00 00        	mov	qword ptr [rbp + 0x2c8], rcx
142339442: 48 8b 40 50                 	mov	rax, qword ptr [rax + 0x50]
142339446: 48 89 85 80 02 00 00        	mov	qword ptr [rbp + 0x280], rax
14233944d: 45 31 c0                    	xor	r8d, r8d
142339450: 4c 89 85 d8 02 00 00        	mov	qword ptr [rbp + 0x2d8], r8
142339457: 4c 89 c1                    	mov	rcx, r8
14233945a: 48 c1 e1 04                 	shl	rcx, 0x4
14233945e: 48 8b 95 80 02 00 00        	mov	rdx, qword ptr [rbp + 0x280]
142339465: 48 8d 04 0a                 	lea	rax, [rdx + rcx]
142339469: 44 8b 64 0a 08              	mov	r12d, dword ptr [rdx + rcx + 0x8]
14233946e: 4d 89 e6                    	mov	r14, r12
142339471: 49 c1 e6 04                 	shl	r14, 0x4
142339475: 4c 03 b5 f0 02 00 00        	add	r14, qword ptr [rbp + 0x2f0]
14233947c: 48 8b 8d f8 02 00 00        	mov	rcx, qword ptr [rbp + 0x2f8]
142339483: 4e 8d 2c a1                 	lea	r13, [rcx + 4*r12]
142339487: 4b 8d 0c a4                 	lea	rcx, [r12 + 4*r12]
14233948b: 48 8b 95 00 03 00 00        	mov	rdx, qword ptr [rbp + 0x300]
142339492: 48 8d 0c 8a                 	lea	rcx, [rdx + 4*rcx]
142339496: 4f 8d 04 64                 	lea	r8, [r12 + 2*r12]
14233949a: 4c 03 85 c0 02 00 00        	add	r8, qword ptr [rbp + 0x2c0]
1423394a1: 48 8b 95 c8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2c8]
1423394a8: 4e 8d 0c a2                 	lea	r9, [rdx + 4*r12]
1423394ac: 4c 03 a5 e0 02 00 00        	add	r12, qword ptr [rbp + 0x2e0]
1423394b3: f6 85 14 03 00 00 01        	test	byte ptr [rbp + 0x314], 0x1
1423394ba: ba 00 00 00 00              	mov	edx, 0x0
1423394bf: 4c 0f 44 e2                 	cmove	r12, rdx
1423394c3: 48 8b 18                    	mov	rbx, qword ptr [rax]
1423394c6: 8b 85 ec 02 00 00           	mov	eax, dword ptr [rbp + 0x2ec]
1423394cc: 4c 89 8d 58 02 00 00        	mov	qword ptr [rbp + 0x258], r9
1423394d3: 41 89 01                    	mov	dword ptr [r9], eax
1423394d6: 4c 89 85 98 02 00 00        	mov	qword ptr [rbp + 0x298], r8
1423394dd: 41 c6 40 01 00              	mov	byte ptr [r8 + 0x1], 0x0
1423394e2: 48 8b 85 b8 02 00 00        	mov	rax, qword ptr [rbp + 0x2b8]
1423394e9: 83 38 01                    	cmp	dword ptr [rax], 0x1
1423394ec: 75 27                       	jne	0x142339515 <.text+0x2338515>
1423394ee: 48 8b 85 b0 02 00 00        	mov	rax, qword ptr [rbp + 0x2b0]
1423394f5: 48 39 58 18                 	cmp	qword ptr [rax + 0x18], rbx
1423394f9: 75 1a                       	jne	0x142339515 <.text+0x2338515>
1423394fb: 8b 85 ec 02 00 00           	mov	eax, dword ptr [rbp + 0x2ec]
142339501: 48 8b 95 58 02 00 00        	mov	rdx, qword ptr [rbp + 0x258]
142339508: 89 02                       	mov	dword ptr [rdx], eax
14233950a: 48 8b 85 98 02 00 00        	mov	rax, qword ptr [rbp + 0x298]
142339511: c6 40 01 01                 	mov	byte ptr [rax + 0x1], 0x1
142339515: f2 45 0f 10 56 08           	movsd	xmm10, qword ptr [r14 + 0x8]
14233951b: 48 8b 85 50 02 00 00        	mov	rax, qword ptr [rbp + 0x250]
142339522: f3 44 0f 10 98 e8 00 00 00  	movss	xmm11, dword ptr [rax + 0xe8]
14233952b: 48 8b 85 e0 01 00 00        	mov	rax, qword ptr [rbp + 0x1e0]
142339532: f3 44 0f 10 20              	movss	xmm12, dword ptr [rax]
142339537: 48 89 4d c0                 	mov	qword ptr [rbp - 0x40], rcx
14233953b: 48 8d 05 0e f0 ff ff        	lea	rax, [rip - 0xff2]      # 0x142338550 <.text+0x2337550>
142339542: 48 89 45 c8                 	mov	qword ptr [rbp - 0x38], rax
142339546: 48 8d 05 cb 47 d1 00        	lea	rax, [rip + 0xd147cb]   # 0x14304dd18
14233954d: 48 89 85 f0 00 00 00        	mov	qword ptr [rbp + 0xf0], rax
142339554: 48 c7 85 f8 00 00 00 01 00 00 00    	mov	qword ptr [rbp + 0xf8], 0x1
14233955f: 48 8d 45 c0                 	lea	rax, [rbp - 0x40]
142339563: 48 89 85 00 01 00 00        	mov	qword ptr [rbp + 0x100], rax
14233956a: 48 c7 85 08 01 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x108], 0x1
142339575: 48 c7 85 10 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x110], 0x0
142339580: 48 8d 8d d8 00 00 00        	lea	rcx, [rbp + 0xd8]
142339587: 48 8d 95 f0 00 00 00        	lea	rdx, [rbp + 0xf0]
14233958e: e8 fd 09 a4 fe              	call	0x140d79f90 <.text+0xd78f90>
142339593: f3 44 0f 59 de              	mulss	xmm11, xmm6
142339598: f3 45 0f 59 dc              	mulss	xmm11, xmm12
14233959d: 0f 10 85 d8 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xd8]
1423395a4: 0f 29 85 b0 01 00 00        	movaps	xmmword ptr [rbp + 0x1b0], xmm0
1423395ab: 48 8b 85 e8 00 00 00        	mov	rax, qword ptr [rbp + 0xe8]
1423395b2: 48 89 85 c0 01 00 00        	mov	qword ptr [rbp + 0x1c0], rax
1423395b9: f2 41 0f 10 06              	movsd	xmm0, qword ptr [r14]
1423395be: f3 41 0f 12 cb              	movsldup	xmm1, xmm11             # xmm1 = xmm11[0,0,2,2]
1423395c3: 44 0f 58 d1                 	addps	xmm10, xmm1
1423395c7: 44 0f 59 d7                 	mulps	xmm10, xmm7
1423395cb: f3 41 0f 10 4d 00           	movss	xmm1, dword ptr [r13]
1423395d1: f2 0f 12 d0                 	movddup	xmm2, xmm0              # xmm2 = xmm0[0,0]
1423395d5: f2 41 0f 12 da              	movddup	xmm3, xmm10             # xmm3 = xmm10[0,0]
1423395da: 0f 5c d3                    	subps	xmm2, xmm3
1423395dd: 44 0f 58 d0                 	addps	xmm10, xmm0
1423395e1: 41 0f c6 d2 4e              	shufps	xmm2, xmm10, 0x4e       # xmm2 = xmm2[2,3],xmm10[0,1]
1423395e6: 0f 11 55 c8                 	movups	xmmword ptr [rbp - 0x38], xmm2
1423395ea: f3 0f 11 4d c4              	movss	dword ptr [rbp - 0x3c], xmm1
1423395ef: c6 45 c0 03                 	mov	byte ptr [rbp - 0x40], 0x3
1423395f3: c6 45 78 00                 	mov	byte ptr [rbp + 0x78], 0x0
1423395f7: c6 85 27 03 00 00 01        	mov	byte ptr [rbp + 0x327], 0x1
1423395fe: 4c 8d ad f0 00 00 00        	lea	r13, [rbp + 0xf0]
142339605: 4c 89 e9                    	mov	rcx, r13
142339608: 4c 8d 75 c0                 	lea	r14, [rbp - 0x40]
14233960c: 4c 89 f2                    	mov	rdx, r14
14233960f: 4c 8d 45 78                 	lea	r8, [rbp + 0x78]
142339613: e8 f8 f7 66 fe              	call	0x1409a8e10 <.text+0x9a7e10>
142339618: c6 85 27 03 00 00 01        	mov	byte ptr [rbp + 0x327], 0x1
14233961f: 4c 89 f1                    	mov	rcx, r14
142339622: 4c 89 ea                    	mov	rdx, r13
142339625: e8 e6 f8 66 fe              	call	0x1409a8f10 <.text+0x9a7f10>
14233962a: c6 85 27 03 00 00 01        	mov	byte ptr [rbp + 0x327], 0x1
142339631: 4c 89 e9                    	mov	rcx, r13
142339634: 4c 89 f2                    	mov	rdx, r14
142339637: 45 31 c0                    	xor	r8d, r8d
14233963a: e8 a1 f8 66 fe              	call	0x1409a8ee0 <.text+0x9a7ee0>
14233963f: c6 85 27 03 00 00 01        	mov	byte ptr [rbp + 0x327], 0x1
142339646: 4c 89 f1                    	mov	rcx, r14
142339649: 4c 89 ea                    	mov	rdx, r13
14233964c: 41 b0 01                    	mov	r8b, 0x1
14233964f: e8 ec f8 66 fe              	call	0x1409a8f40 <.text+0x9a7f40>
142339654: c6 85 26 03 00 00 01        	mov	byte ptr [rbp + 0x326], 0x1
14233965b: 4c 8d 7d 78                 	lea	r15, [rbp + 0x78]
14233965f: 4c 89 f9                    	mov	rcx, r15
142339662: e8 49 f9 66 fe              	call	0x1409a8fb0 <.text+0x9a7fb0>
142339667: c6 85 26 03 00 00 00        	mov	byte ptr [rbp + 0x326], 0x0
14233966e: 4c 89 e9                    	mov	rcx, r13
142339671: 4c 89 f2                    	mov	rdx, r14
142339674: 4d 89 f8                    	mov	r8, r15
142339677: e8 04 f8 66 fe              	call	0x1409a8e80 <.text+0x9a7e80>
14233967c: c6 85 27 03 00 00 00        	mov	byte ptr [rbp + 0x327], 0x0
142339683: 48 8b 8d e8 01 00 00        	mov	rcx, qword ptr [rbp + 0x1e8]
14233968a: 48 8d 95 b0 01 00 00        	lea	rdx, [rbp + 0x1b0]
142339691: 4d 89 e8                    	mov	r8, r13
142339694: e8 57 9b 5e fe              	call	0x1409231f0 <.text+0x9221f0>
142339699: 48 89 45 78                 	mov	qword ptr [rbp + 0x78], rax
14233969d: 48 89 95 80 00 00 00        	mov	qword ptr [rbp + 0x80], rdx
1423396a4: 4d 85 e4                    	test	r12, r12
1423396a7: 49 89 df                    	mov	r15, rbx
1423396aa: 0f 84 00 01 00 00           	je	0x1423397b0 <.text+0x23387b0>
1423396b0: 48 8d 4d 78                 	lea	rcx, [rbp + 0x78]
1423396b4: e8 57 48 5f fe              	call	0x14092df10 <.text+0x92cf10>
1423396b9: 88 85 f0 00 00 00           	mov	byte ptr [rbp + 0xf0], al
1423396bf: 48 8d 8d f0 00 00 00        	lea	rcx, [rbp + 0xf0]
1423396c6: e8 e5 4f 5f fe              	call	0x14092e6b0 <.text+0x92d6b0>
1423396cb: 84 c0                       	test	al, al
1423396cd: 74 62                       	je	0x142339731 <.text+0x2338731>
1423396cf: 45 0f b6 24 24              	movzx	r12d, byte ptr [r12]
1423396d4: 48 8b 85 08 02 00 00        	mov	rax, qword ptr [rbp + 0x208]
1423396db: 8b 8d 90 02 00 00           	mov	ecx, dword ptr [rbp + 0x290]
1423396e1: 89 08                       	mov	dword ptr [rax], ecx
1423396e3: 48 8b 85 a0 02 00 00        	mov	rax, qword ptr [rbp + 0x2a0]
1423396ea: 4c 8b 70 30                 	mov	r14, qword ptr [rax + 0x30]
1423396ee: 4c 8b 68 40                 	mov	r13, qword ptr [rax + 0x40]
1423396f2: 4c 3b 70 20                 	cmp	r14, qword ptr [rax + 0x20]
1423396f6: 75 13                       	jne	0x14233970b <.text+0x233870b>
1423396f8: 48 8b 8d 20 02 00 00        	mov	rcx, qword ptr [rbp + 0x220]
1423396ff: 48 8d 15 6a bb 87 00        	lea	rdx, [rip + 0x87bb6a]   # 0x142bb5270
142339706: e8 25 8f 22 fe              	call	0x140562630 <.text+0x561630>
14233970b: 48 8b 95 a0 02 00 00        	mov	rdx, qword ptr [rbp + 0x2a0]
142339712: 48 8b 42 28                 	mov	rax, qword ptr [rdx + 0x28]
142339716: 4c 89 f1                    	mov	rcx, r14
142339719: 48 c1 e1 04                 	shl	rcx, 0x4
14233971d: 4c 89 2c 08                 	mov	qword ptr [rax + rcx], r13
142339721: 44 88 64 08 08              	mov	byte ptr [rax + rcx + 0x8], r12b
142339726: 49 ff c6                    	inc	r14
142339729: 4c 89 72 30                 	mov	qword ptr [rdx + 0x30], r14
14233972d: 48 ff 42 40                 	inc	qword ptr [rdx + 0x40]
142339731: 48 8d 4d 78                 	lea	rcx, [rbp + 0x78]
142339735: e8 d6 47 5f fe              	call	0x14092df10 <.text+0x92cf10>
14233973a: 3c 03                       	cmp	al, 0x3
14233973c: 0f 85 4e 01 00 00           	jne	0x142339890 <.text+0x2338890>
142339742: 48 8b 8d d0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2d0]
142339749: 48 8b 01                    	mov	rax, qword ptr [rcx]
14233974c: 4c 8b 71 10                 	mov	r14, qword ptr [rcx + 0x10]
142339750: 4c 29 f0                    	sub	rax, r14
142339753: 48 83 f8 27                 	cmp	rax, 0x27
142339757: 0f 86 7a 02 00 00           	jbe	0x1423399d7 <.text+0x23389d7>
14233975d: 48 8b 8d d0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2d0]
142339764: 48 8b 41 08                 	mov	rax, qword ptr [rcx + 0x8]
142339768: 48 8d 15 91 5a ca fe        	lea	rdx, [rip - 0x135a56f]  # 0x140fdf200 <.text+0xfde200>
14233976f: 4a 89 14 30                 	mov	qword ptr [rax + r14], rdx
142339773: 48 8d 15 e6 60 b1 ff        	lea	rdx, [rip - 0x4e9f1a]   # 0x141e4f860 <.text+0x1e4e860>
14233977a: 4a 89 54 30 08              	mov	qword ptr [rax + r14 + 0x8], rdx
14233977f: 4e 89 7c 30 10              	mov	qword ptr [rax + r14 + 0x10], r15
142339784: f2 46 0f 11 4c 30 18        	movsd	qword ptr [rax + r14 + 0x18], xmm9
14233978b: 42 c7 44 30 20 00 00 00 00  	mov	dword ptr [rax + r14 + 0x20], 0x0
142339794: 42 c6 44 30 24 00           	mov	byte ptr [rax + r14 + 0x24], 0x0
14233979a: 49 83 c6 28                 	add	r14, 0x28
14233979e: 4c 89 71 10                 	mov	qword ptr [rcx + 0x10], r14
1423397a2: e9 e9 00 00 00              	jmp	0x142339890 <.text+0x2338890>
1423397a7: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1423397b0: 48 8b 85 00 02 00 00        	mov	rax, qword ptr [rbp + 0x200]
1423397b7: 83 38 0d                    	cmp	dword ptr [rax], 0xd
1423397ba: 0f 85 d0 00 00 00           	jne	0x142339890 <.text+0x2338890>
1423397c0: 48 8d 4d 78                 	lea	rcx, [rbp + 0x78]
1423397c4: e8 b7 4e 5f fe              	call	0x14092e680 <.text+0x92d680>
1423397c9: 84 c0                       	test	al, al
1423397cb: 0f 84 77 f7 ff ff           	je	0x142338f48 <.text+0x2337f48>
1423397d1: 48 8b 85 28 02 00 00        	mov	rax, qword ptr [rbp + 0x228]
1423397d8: f2 44 0f 10 10              	movsd	xmm10, qword ptr [rax]
1423397dd: 48 8b 85 50 02 00 00        	mov	rax, qword ptr [rbp + 0x250]
1423397e4: f3 44 0f 10 98 e8 00 00 00  	movss	xmm11, dword ptr [rax + 0xe8]
1423397ed: 0f b6 05 15 b4 2f 01        	movzx	eax, byte ptr [rip + 0x12fb415] # 0x143634c09
1423397f4: b9 20 00 00 00              	mov	ecx, 0x20
1423397f9: ba 08 00 00 00              	mov	edx, 0x8
1423397fe: e8 0d a4 2d fe              	call	0x140613c10 <.text+0x612c10>
142339803: 48 85 c0                    	test	rax, rax
142339806: 0f 84 dd 02 00 00           	je	0x142339ae9 <.text+0x2338ae9>
14233980c: f3 45 0f 59 d8              	mulss	xmm11, xmm8
142339811: 48 c7 00 01 00 00 00        	mov	qword ptr [rax], 0x1
142339818: 48 c7 40 08 01 00 00 00     	mov	qword ptr [rax + 0x8], 0x1
142339820: 44 0f 13 50 10              	movlps	qword ptr [rax + 0x10], xmm10
142339825: f3 44 0f 11 58 18           	movss	dword ptr [rax + 0x18], xmm11
14233982b: 48 8d 0d f6 44 d1 00        	lea	rcx, [rip + 0xd144f6]   # 0x14304dd28
142339832: 48 89 4c 24 20              	mov	qword ptr [rsp + 0x20], rcx
142339837: 41 b8 12 00 00 00           	mov	r8d, 0x12
14233983d: 48 8d 8d f0 00 00 00        	lea	rcx, [rbp + 0xf0]
142339844: 48 8d 15 0d 45 d1 00        	lea	rdx, [rip + 0xd1450d]   # 0x14304dd58
14233984b: 49 89 c1                    	mov	r9, rax
14233984e: e8 5d 40 56 fe              	call	0x14089d8b0 <.text+0x89c8b0>
142339853: 48 8b 85 10 01 00 00        	mov	rax, qword ptr [rbp + 0x110]
14233985a: 48 89 45 e0                 	mov	qword ptr [rbp - 0x20], rax
14233985e: 0f 10 85 f0 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xf0]
142339865: 0f 10 8d 00 01 00 00        	movups	xmm1, xmmword ptr [rbp + 0x100]
14233986c: 0f 29 4d d0                 	movaps	xmmword ptr [rbp - 0x30], xmm1
142339870: 0f 29 45 c0                 	movaps	xmmword ptr [rbp - 0x40], xmm0
142339874: 48 8b 8d 30 02 00 00        	mov	rcx, qword ptr [rbp + 0x230]
14233987b: 48 8d 55 c0                 	lea	rdx, [rbp - 0x40]
14233987f: e8 0c 37 7f fe              	call	0x140b2cf90 <.text+0xb2bf90>
142339884: e9 bf f6 ff ff              	jmp	0x142338f48 <.text+0x2337f48>
142339889: 0f 1f 80 00 00 00 00        	nop	dword ptr [rax]
142339890: 80 bd 13 03 00 00 00        	cmp	byte ptr [rbp + 0x313], 0x0
142339897: 74 77                       	je	0x142339910 <.text+0x2338910>
142339899: 48 8b 85 98 02 00 00        	mov	rax, qword ptr [rbp + 0x298]
1423398a0: 80 38 01                    	cmp	byte ptr [rax], 0x1
1423398a3: 75 6b                       	jne	0x142339910 <.text+0x2338910>
1423398a5: 48 8d 4d 78                 	lea	rcx, [rbp + 0x78]
1423398a9: e8 d2 4d 5f fe              	call	0x14092e680 <.text+0x92d680>
1423398ae: 84 c0                       	test	al, al
1423398b0: 74 5e                       	je	0x142339910 <.text+0x2338910>
1423398b2: 8b 85 ec 02 00 00           	mov	eax, dword ptr [rbp + 0x2ec]
1423398b8: 48 8b 8d 58 02 00 00        	mov	rcx, qword ptr [rbp + 0x258]
1423398bf: 89 01                       	mov	dword ptr [rcx], eax
1423398c1: 48 8b 85 98 02 00 00        	mov	rax, qword ptr [rbp + 0x298]
1423398c8: c6 40 01 01                 	mov	byte ptr [rax + 0x1], 0x1
1423398cc: 48 8b 8d b8 02 00 00        	mov	rcx, qword ptr [rbp + 0x2b8]
1423398d3: e8 58 c6 64 fe              	call	0x140985f30 <.text+0x984f30>
1423398d8: 22 85 0c 03 00 00           	and	al, byte ptr [rbp + 0x30c]
1423398de: 3c 01                       	cmp	al, 0x1
1423398e0: 75 2e                       	jne	0x142339910 <.text+0x2338910>
1423398e2: 8b 85 94 02 00 00           	mov	eax, dword ptr [rbp + 0x294]
1423398e8: 48 8b 8d 38 02 00 00        	mov	rcx, qword ptr [rbp + 0x238]
1423398ef: 89 01                       	mov	dword ptr [rcx], eax
1423398f1: 48 8b 85 b0 02 00 00        	mov	rax, qword ptr [rbp + 0x2b0]
1423398f8: 48 c7 40 10 01 00 00 00     	mov	qword ptr [rax + 0x10], 0x1
142339900: 4c 89 78 18                 	mov	qword ptr [rax + 0x18], r15
142339904: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
142339910: 48 8d 4d 78                 	lea	rcx, [rbp + 0x78]
142339914: e8 d7 4c 5f fe              	call	0x14092e5f0 <.text+0x92d5f0>
142339919: 84 c0                       	test	al, al
14233991b: 74 5d                       	je	0x14233997a <.text+0x233897a>
14233991d: 8b 85 88 02 00 00           	mov	eax, dword ptr [rbp + 0x288]
142339923: 48 8b 8d f0 01 00 00        	mov	rcx, qword ptr [rbp + 0x1f0]
14233992a: 89 01                       	mov	dword ptr [rcx], eax
14233992c: 48 8b 85 60 02 00 00        	mov	rax, qword ptr [rbp + 0x260]
142339933: 48 8b 58 30                 	mov	rbx, qword ptr [rax + 0x30]
142339937: 4c 8b 70 40                 	mov	r14, qword ptr [rax + 0x40]
14233993b: 48 3b 58 20                 	cmp	rbx, qword ptr [rax + 0x20]
14233993f: 75 13                       	jne	0x142339954 <.text+0x2338954>
142339941: 48 8b 8d 10 02 00 00        	mov	rcx, qword ptr [rbp + 0x210]
142339948: 48 8d 15 21 b9 87 00        	lea	rdx, [rip + 0x87b921]   # 0x142bb5270
14233994f: e8 dc 8c 22 fe              	call	0x140562630 <.text+0x561630>
142339954: 48 8b 95 60 02 00 00        	mov	rdx, qword ptr [rbp + 0x260]
14233995b: 48 8b 42 28                 	mov	rax, qword ptr [rdx + 0x28]
14233995f: 48 89 d9                    	mov	rcx, rbx
142339962: 48 c1 e1 04                 	shl	rcx, 0x4
142339966: 4c 89 34 08                 	mov	qword ptr [rax + rcx], r14
14233996a: 4c 89 7c 08 08              	mov	qword ptr [rax + rcx + 0x8], r15
14233996f: 48 ff c3                    	inc	rbx
142339972: 48 89 5a 30                 	mov	qword ptr [rdx + 0x30], rbx
142339976: 48 ff 42 40                 	inc	qword ptr [rdx + 0x40]
14233997a: 48 8d 4d 78                 	lea	rcx, [rbp + 0x78]
14233997e: e8 8d 45 5f fe              	call	0x14092df10 <.text+0x92cf10>
142339983: 3c 02                       	cmp	al, 0x2
142339985: 0f 84 bd f5 ff ff           	je	0x142338f48 <.text+0x2337f48>
14233998b: 48 8d 4d 78                 	lea	rcx, [rbp + 0x78]
14233998f: e8 7c 45 5f fe              	call	0x14092df10 <.text+0x92cf10>
142339994: 41 89 c6                    	mov	r14d, eax
142339997: 8b 85 8c 02 00 00           	mov	eax, dword ptr [rbp + 0x28c]
14233999d: 48 8b 8d f8 01 00 00        	mov	rcx, qword ptr [rbp + 0x1f8]
1423399a4: 89 01                       	mov	dword ptr [rcx], eax
1423399a6: 48 8b 85 68 02 00 00        	mov	rax, qword ptr [rbp + 0x268]
1423399ad: 48 8b 58 30                 	mov	rbx, qword ptr [rax + 0x30]
1423399b1: 4c 8b 60 40                 	mov	r12, qword ptr [rax + 0x40]
1423399b5: 48 3b 58 20                 	cmp	rbx, qword ptr [rax + 0x20]
1423399b9: 0f 85 61 f5 ff ff           	jne	0x142338f20 <.text+0x2337f20>
1423399bf: 48 8b 8d 18 02 00 00        	mov	rcx, qword ptr [rbp + 0x218]
1423399c6: 48 8d 15 a3 b8 87 00        	lea	rdx, [rip + 0x87b8a3]   # 0x142bb5270
1423399cd: e8 de 93 22 fe              	call	0x140562db0 <.text+0x561db0>
1423399d2: e9 49 f5 ff ff              	jmp	0x142338f20 <.text+0x2337f20>
1423399d7: 48 c7 44 24 20 01 00 00 00  	mov	qword ptr [rsp + 0x20], 0x1
1423399e0: 41 b8 28 00 00 00           	mov	r8d, 0x28
1423399e6: 41 b9 01 00 00 00           	mov	r9d, 0x1
1423399ec: 48 8b 8d d0 02 00 00        	mov	rcx, qword ptr [rbp + 0x2d0]
1423399f3: 4c 89 f2                    	mov	rdx, r14
1423399f6: e8 d5 be 56 00              	call	0x1428a58d0 <.text+0x28a48d0>
1423399fb: e9 5d fd ff ff              	jmp	0x14233975d <.text+0x233875d>
142339a00: 48 83 bd 40 02 00 00 00     	cmp	qword ptr [rbp + 0x240], 0x0
142339a08: 74 4b                       	je	0x142339a55 <.text+0x2338a55>
142339a0a: e8 21 e7 8d fe              	call	0x140c18130 <.text+0xc17130>
142339a0f: 48 89 c1                    	mov	rcx, rax
142339a12: 48 8b 00                    	mov	rax, qword ptr [rax]
142339a15: 48 83 f8 01                 	cmp	rax, 0x1
142339a19: 0f 85 80 00 00 00           	jne	0x142339a9f <.text+0x2338a9f>
142339a1f: 48 83 c1 08                 	add	rcx, 0x8
142339a23: 48 83 39 00                 	cmp	qword ptr [rcx], 0x0
142339a27: 0f 85 b0 00 00 00           	jne	0x142339add <.text+0x2338add>
142339a2d: 48 c7 01 ff ff ff ff        	mov	qword ptr [rcx], -0x1
142339a34: 48 89 8d 18 03 00 00        	mov	qword ptr [rbp + 0x318], rcx
142339a3b: 48 83 c1 08                 	add	rcx, 0x8
142339a3f: 48 8b 95 48 02 00 00        	mov	rdx, qword ptr [rbp + 0x248]
142339a46: e8 a5 d6 8d fe              	call	0x140c170f0 <.text+0xc160f0>
142339a4b: 48 8b 85 18 03 00 00        	mov	rax, qword ptr [rbp + 0x318]
142339a52: 48 ff 00                    	inc	qword ptr [rax]
142339a55: 0f 28 b5 30 03 00 00        	movaps	xmm6, xmmword ptr [rbp + 0x330]
142339a5c: 0f 28 bd 40 03 00 00        	movaps	xmm7, xmmword ptr [rbp + 0x340]
142339a63: 44 0f 28 85 50 03 00 00     	movaps	xmm8, xmmword ptr [rbp + 0x350]
142339a6b: 44 0f 28 8d 60 03 00 00     	movaps	xmm9, xmmword ptr [rbp + 0x360]
142339a73: 44 0f 28 95 70 03 00 00     	movaps	xmm10, xmmword ptr [rbp + 0x370]
142339a7b: 44 0f 28 9d 80 03 00 00     	movaps	xmm11, xmmword ptr [rbp + 0x380]
142339a83: 44 0f 28 a5 90 03 00 00     	movaps	xmm12, xmmword ptr [rbp + 0x390]
142339a8b: 48 81 c4 28 04 00 00        	add	rsp, 0x428
142339a92: 5b                          	pop	rbx
142339a93: 5f                          	pop	rdi
142339a94: 5e                          	pop	rsi
142339a95: 41 5c                       	pop	r12
142339a97: 41 5d                       	pop	r13
142339a99: 41 5e                       	pop	r14
142339a9b: 41 5f                       	pop	r15
142339a9d: 5d                          	pop	rbp
142339a9e: c3                          	ret
142339a9f: 48 85 c0                    	test	rax, rax
142339aa2: 75 2d                       	jne	0x142339ad1 <.text+0x2338ad1>
142339aa4: 31 d2                       	xor	edx, edx
142339aa6: e8 a5 c7 5a 00              	call	0x1428e6250 <.text+0x28e5250>
142339aab: 48 89 c1                    	mov	rcx, rax
142339aae: 48 85 c0                    	test	rax, rax
142339ab1: 0f 85 6c ff ff ff           	jne	0x142339a23 <.text+0x2338a23>
142339ab7: eb 18                       	jmp	0x142339ad1 <.text+0x2338ad1>
142339ab9: 48 85 c0                    	test	rax, rax
142339abc: 75 13                       	jne	0x142339ad1 <.text+0x2338ad1>
142339abe: 31 d2                       	xor	edx, edx
142339ac0: e8 8b c7 5a 00              	call	0x1428e6250 <.text+0x28e5250>
142339ac5: 48 89 c1                    	mov	rcx, rax
142339ac8: 48 85 c0                    	test	rax, rax
142339acb: 0f 85 11 f1 ff ff           	jne	0x142338be2 <.text+0x2337be2>
142339ad1: 48 8d 0d 00 b8 87 00        	lea	rcx, [rip + 0x87b800]   # 0x142bb52d8
142339ad8: e8 f3 e5 59 00              	call	0x1428d80d0 <.text+0x28d70d0>
142339add: 48 8d 0d 64 b8 87 00        	lea	rcx, [rip + 0x87b864]   # 0x142bb5348
142339ae4: e8 67 f7 59 00              	call	0x1428d9250 <.text+0x28d8250>
142339ae9: b9 08 00 00 00              	mov	ecx, 0x8
142339aee: ba 20 00 00 00              	mov	edx, 0x20
142339af3: e8 08 f5 59 00              	call	0x1428d9000 <.text+0x28d8000>
142339af8: 0f 0b                       	ud2
142339afa: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
142339b00: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
142339b05: 55                          	push	rbp
142339b06: 41 57                       	push	r15
142339b08: 41 56                       	push	r14
142339b0a: 41 55                       	push	r13
142339b0c: 41 54                       	push	r12
142339b0e: 56                          	push	rsi
142339b0f: 57                          	push	rdi
142339b10: 53                          	push	rbx
142339b11: 48 81 ec a8 00 00 00        	sub	rsp, 0xa8
142339b18: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
142339b1f: 44 0f 29 64 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm12
142339b25: 44 0f 29 5c 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm11
142339b2b: 44 0f 29 54 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm10
142339b31: 44 0f 29 4c 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm9
142339b37: 44 0f 29 44 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm8
142339b3d: 0f 29 bc 24 80 00 00 00     	movaps	xmmword ptr [rsp + 0x80], xmm7
142339b45: 0f 29 b4 24 90 00 00 00     	movaps	xmmword ptr [rsp + 0x90], xmm6
142339b4d: 48 8b 85 d8 02 00 00        	mov	rax, qword ptr [rbp + 0x2d8]
142339b54: 48 ff 00                    	inc	qword ptr [rax]
142339b57: 0f 28 b4 24 90 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x90]
142339b5f: 0f 28 bc 24 80 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0x80]
142339b67: 44 0f 28 44 24 70           	movaps	xmm8, xmmword ptr [rsp + 0x70]
142339b6d: 44 0f 28 4c 24 60           	movaps	xmm9, xmmword ptr [rsp + 0x60]
142339b73: 44 0f 28 54 24 50           	movaps	xmm10, xmmword ptr [rsp + 0x50]
142339b79: 44 0f 28 5c 24 40           	movaps	xmm11, xmmword ptr [rsp + 0x40]
142339b7f: 44 0f 28 64 24 30           	movaps	xmm12, xmmword ptr [rsp + 0x30]
142339b85: 48 81 c4 a8 00 00 00        	add	rsp, 0xa8
142339b8c: 5b                          	pop	rbx
142339b8d: 5f                          	pop	rdi
142339b8e: 5e                          	pop	rsi
142339b8f: 41 5c                       	pop	r12
142339b91: 41 5d                       	pop	r13
142339b93: 41 5e                       	pop	r14
142339b95: 41 5f                       	pop	r15
142339b97: 5d                          	pop	rbp
142339b98: c3                          	ret
142339b99: 0f 1f 80 00 00 00 00        	nop	dword ptr [rax]
142339ba0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
142339ba5: 55                          	push	rbp
142339ba6: 41 57                       	push	r15
142339ba8: 41 56                       	push	r14
142339baa: 41 55                       	push	r13
142339bac: 41 54                       	push	r12
142339bae: 56                          	push	rsi
142339baf: 57                          	push	rdi
142339bb0: 53                          	push	rbx
142339bb1: 48 81 ec a8 00 00 00        	sub	rsp, 0xa8
142339bb8: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
142339bbf: 44 0f 29 64 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm12
142339bc5: 44 0f 29 5c 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm11
142339bcb: 44 0f 29 54 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm10
142339bd1: 44 0f 29 4c 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm9
142339bd7: 44 0f 29 44 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm8
142339bdd: 0f 29 bc 24 80 00 00 00     	movaps	xmmword ptr [rsp + 0x80], xmm7
142339be5: 0f 29 b4 24 90 00 00 00     	movaps	xmmword ptr [rsp + 0x90], xmm6
142339bed: 48 8b 85 18 03 00 00        	mov	rax, qword ptr [rbp + 0x318]
142339bf4: 48 ff 00                    	inc	qword ptr [rax]
142339bf7: 0f 28 b4 24 90 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x90]
142339bff: 0f 28 bc 24 80 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0x80]
142339c07: 44 0f 28 44 24 70           	movaps	xmm8, xmmword ptr [rsp + 0x70]
142339c0d: 44 0f 28 4c 24 60           	movaps	xmm9, xmmword ptr [rsp + 0x60]
142339c13: 44 0f 28 54 24 50           	movaps	xmm10, xmmword ptr [rsp + 0x50]
142339c19: 44 0f 28 5c 24 40           	movaps	xmm11, xmmword ptr [rsp + 0x40]
142339c1f: 44 0f 28 64 24 30           	movaps	xmm12, xmmword ptr [rsp + 0x30]
142339c25: 48 81 c4 a8 00 00 00        	add	rsp, 0xa8
142339c2c: 5b                          	pop	rbx
142339c2d: 5f                          	pop	rdi
142339c2e: 5e                          	pop	rsi
142339c2f: 41 5c                       	pop	r12
142339c31: 41 5d                       	pop	r13
142339c33: 41 5e                       	pop	r14
142339c35: 41 5f                       	pop	r15
142339c37: 5d                          	pop	rbp
142339c38: c3                          	ret
142339c39: 0f 1f 80 00 00 00 00        	nop	dword ptr [rax]
142339c40: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
142339c45: 55                          	push	rbp
142339c46: 41 57                       	push	r15
142339c48: 41 56                       	push	r14
142339c4a: 41 55                       	push	r13
142339c4c: 41 54                       	push	r12
142339c4e: 56                          	push	rsi
142339c4f: 57                          	push	rdi
142339c50: 53                          	push	rbx
142339c51: 48 81 ec a8 00 00 00        	sub	rsp, 0xa8
142339c58: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
142339c5f: 44 0f 29 64 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm12
142339c65: 44 0f 29 5c 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm11
142339c6b: 44 0f 29 54 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm10
142339c71: 44 0f 29 4c 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm9
142339c77: 44 0f 29 44 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm8
142339c7d: 0f 29 bc 24 80 00 00 00     	movaps	xmmword ptr [rsp + 0x80], xmm7
142339c85: 0f 29 b4 24 90 00 00 00     	movaps	xmmword ptr [rsp + 0x90], xmm6
142339c8d: 80 bd 26 03 00 00 00        	cmp	byte ptr [rbp + 0x326], 0x0
142339c94: 74 52                       	je	0x142339ce8 <.text+0x2338ce8>
142339c96: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
142339c9a: e8 d1 b7 55 fe              	call	0x140895470 <.text+0x894470>
142339c9f: c6 85 27 03 00 00 01        	mov	byte ptr [rbp + 0x327], 0x1
142339ca6: 0f 28 b4 24 90 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x90]
142339cae: 0f 28 bc 24 80 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0x80]
142339cb6: 44 0f 28 44 24 70           	movaps	xmm8, xmmword ptr [rsp + 0x70]
142339cbc: 44 0f 28 4c 24 60           	movaps	xmm9, xmmword ptr [rsp + 0x60]
142339cc2: 44 0f 28 54 24 50           	movaps	xmm10, xmmword ptr [rsp + 0x50]
142339cc8: 44 0f 28 5c 24 40           	movaps	xmm11, xmmword ptr [rsp + 0x40]
142339cce: 44 0f 28 64 24 30           	movaps	xmm12, xmmword ptr [rsp + 0x30]
142339cd4: 48 81 c4 a8 00 00 00        	add	rsp, 0xa8
142339cdb: 5b                          	pop	rbx
142339cdc: 5f                          	pop	rdi
142339cdd: 5e                          	pop	rsi
142339cde: 41 5c                       	pop	r12
142339ce0: 41 5d                       	pop	r13
142339ce2: 41 5e                       	pop	r14
142339ce4: 41 5f                       	pop	r15
142339ce6: 5d                          	pop	rbp
142339ce7: c3                          	ret
142339ce8: c6 85 27 03 00 00 01        	mov	byte ptr [rbp + 0x327], 0x1
142339cef: 0f 28 b4 24 90 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x90]
142339cf7: 0f 28 bc 24 80 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0x80]
142339cff: 44 0f 28 44 24 70           	movaps	xmm8, xmmword ptr [rsp + 0x70]
142339d05: 44 0f 28 4c 24 60           	movaps	xmm9, xmmword ptr [rsp + 0x60]
142339d0b: 44 0f 28 54 24 50           	movaps	xmm10, xmmword ptr [rsp + 0x50]
142339d11: 44 0f 28 5c 24 40           	movaps	xmm11, xmmword ptr [rsp + 0x40]
142339d17: 44 0f 28 64 24 30           	movaps	xmm12, xmmword ptr [rsp + 0x30]
142339d1d: 48 81 c4 a8 00 00 00        	add	rsp, 0xa8
142339d24: 5b                          	pop	rbx
142339d25: 5f                          	pop	rdi
142339d26: 5e                          	pop	rsi
142339d27: 41 5c                       	pop	r12
142339d29: 41 5d                       	pop	r13
142339d2b: 41 5e                       	pop	r14
142339d2d: 41 5f                       	pop	r15
142339d2f: 5d                          	pop	rbp
142339d30: c3                          	ret
142339d31: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nop	word ptr cs:[rax + rax]
142339d40: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
142339d45: 55                          	push	rbp
142339d46: 41 57                       	push	r15
142339d48: 41 56                       	push	r14
142339d4a: 41 55                       	push	r13
142339d4c: 41 54                       	push	r12
142339d4e: 56                          	push	rsi
142339d4f: 57                          	push	rdi
142339d50: 53                          	push	rbx
142339d51: 48 81 ec a8 00 00 00        	sub	rsp, 0xa8
142339d58: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
142339d5f: 44 0f 29 64 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm12
142339d65: 44 0f 29 5c 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm11
142339d6b: 44 0f 29 54 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm10
142339d71: 44 0f 29 4c 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm9
142339d77: 44 0f 29 44 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm8
142339d7d: 0f 29 bc 24 80 00 00 00     	movaps	xmmword ptr [rsp + 0x80], xmm7
142339d85: 0f 29 b4 24 90 00 00 00     	movaps	xmmword ptr [rsp + 0x90], xmm6
142339d8d: 80 bd 27 03 00 00 00        	cmp	byte ptr [rbp + 0x327], 0x0
142339d94: 74 60                       	je	0x142339df6 <.text+0x2338df6>
142339d96: 48 8b 95 b0 01 00 00        	mov	rdx, qword ptr [rbp + 0x1b0]
142339d9d: 48 85 d2                    	test	rdx, rdx
142339da0: 74 12                       	je	0x142339db4 <.text+0x2338db4>
142339da2: 48 8b 8d b8 01 00 00        	mov	rcx, qword ptr [rbp + 0x1b8]
142339da9: 41 b8 01 00 00 00           	mov	r8d, 0x1
142339daf: e8 6c 9e 2d fe              	call	0x140613c20 <.text+0x612c20>
142339db4: 0f 28 b4 24 90 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x90]
142339dbc: 0f 28 bc 24 80 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0x80]
142339dc4: 44 0f 28 44 24 70           	movaps	xmm8, xmmword ptr [rsp + 0x70]
142339dca: 44 0f 28 4c 24 60           	movaps	xmm9, xmmword ptr [rsp + 0x60]
142339dd0: 44 0f 28 54 24 50           	movaps	xmm10, xmmword ptr [rsp + 0x50]
142339dd6: 44 0f 28 5c 24 40           	movaps	xmm11, xmmword ptr [rsp + 0x40]
142339ddc: 44 0f 28 64 24 30           	movaps	xmm12, xmmword ptr [rsp + 0x30]
142339de2: 48 81 c4 a8 00 00 00        	add	rsp, 0xa8
142339de9: 5b                          	pop	rbx
142339dea: 5f                          	pop	rdi
142339deb: 5e                          	pop	rsi
142339dec: 41 5c                       	pop	r12
142339dee: 41 5d                       	pop	r13
142339df0: 41 5e                       	pop	r14
142339df2: 41 5f                       	pop	r15
142339df4: 5d                          	pop	rbp
142339df5: c3                          	ret
142339df6: 0f 28 b4 24 90 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x90]
142339dfe: 0f 28 bc 24 80 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0x80]
142339e06: 44 0f 28 44 24 70           	movaps	xmm8, xmmword ptr [rsp + 0x70]
142339e0c: 44 0f 28 4c 24 60           	movaps	xmm9, xmmword ptr [rsp + 0x60]
142339e12: 44 0f 28 54 24 50           	movaps	xmm10, xmmword ptr [rsp + 0x50]
142339e18: 44 0f 28 5c 24 40           	movaps	xmm11, xmmword ptr [rsp + 0x40]
142339e1e: 44 0f 28 64 24 30           	movaps	xmm12, xmmword ptr [rsp + 0x30]
142339e24: 48 81 c4 a8 00 00 00        	add	rsp, 0xa8
142339e2b: 5b                          	pop	rbx
142339e2c: 5f                          	pop	rdi
142339e2d: 5e                          	pop	rsi
142339e2e: 41 5c                       	pop	r12
142339e30: 41 5d                       	pop	r13
142339e32: 41 5e                       	pop	r14
142339e34: 41 5f                       	pop	r15
142339e36: 5d                          	pop	rbp
142339e37: c3                          	ret
142339e38: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
142339e40: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
142339e45: 55                          	push	rbp
142339e46: 41 57                       	push	r15
142339e48: 41 56                       	push	r14
142339e4a: 41 55                       	push	r13
142339e4c: 41 54                       	push	r12
142339e4e: 56                          	push	rsi
142339e4f: 57                          	push	rdi
142339e50: 53                          	push	rbx
142339e51: 48 81 ec a8 00 00 00        	sub	rsp, 0xa8
142339e58: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
142339e5f: 44 0f 29 64 24 30           	movaps	xmmword ptr [rsp + 0x30], xmm12
142339e65: 44 0f 29 5c 24 40           	movaps	xmmword ptr [rsp + 0x40], xmm11
142339e6b: 44 0f 29 54 24 50           	movaps	xmmword ptr [rsp + 0x50], xmm10
142339e71: 44 0f 29 4c 24 60           	movaps	xmmword ptr [rsp + 0x60], xmm9
142339e77: 44 0f 29 44 24 70           	movaps	xmmword ptr [rsp + 0x70], xmm8
142339e7d: 0f 29 bc 24 80 00 00 00     	movaps	xmmword ptr [rsp + 0x80], xmm7
142339e85: 0f 29 b4 24 90 00 00 00     	movaps	xmmword ptr [rsp + 0x90], xmm6
142339e8d: 48 83 bd 40 02 00 00 00     	cmp	qword ptr [rbp + 0x240], 0x0
142339e95: 74 2f                       	je	0x142339ec6 <.text+0x2338ec6>
142339e97: 48 8d 85 48 02 00 00        	lea	rax, [rbp + 0x248]
142339e9e: 48 89 85 d8 01 00 00        	mov	qword ptr [rbp + 0x1d8], rax
142339ea5: 48 8d 85 d8 01 00 00        	lea	rax, [rbp + 0x1d8]
142339eac: 48 89 85 d0 01 00 00        	mov	qword ptr [rbp + 0x1d0], rax
142339eb3: 48 8d 0d 8e 3c d1 00        	lea	rcx, [rip + 0xd13c8e]   # 0x14304db48
142339eba: 48 8d 95 d0 01 00 00        	lea	rdx, [rbp + 0x1d0]
142339ec1: e8 5a 5d c9 fe              	call	0x140fcfc20 <.text+0xfcec20>
142339ec6: 0f 28 b4 24 90 00 00 00     	movaps	xmm6, xmmword ptr [rsp + 0x90]
142339ece: 0f 28 bc 24 80 00 00 00     	movaps	xmm7, xmmword ptr [rsp + 0x80]
142339ed6: 44 0f 28 44 24 70           	movaps	xmm8, xmmword ptr [rsp + 0x70]
142339edc: 44 0f 28 4c 24 60           	movaps	xmm9, xmmword ptr [rsp + 0x60]
142339ee2: 44 0f 28 54 24 50           	movaps	xmm10, xmmword ptr [rsp + 0x50]
142339ee8: 44 0f 28 5c 24 40           	movaps	xmm11, xmmword ptr [rsp + 0x40]
142339eee: 44 0f 28 64 24 30           	movaps	xmm12, xmmword ptr [rsp + 0x30]
142339ef4: 48 81 c4 a8 00 00 00        	add	rsp, 0xa8
142339efb: 5b                          	pop	rbx
142339efc: 5f                          	pop	rdi
142339efd: 5e                          	pop	rsi
142339efe: 41 5c                       	pop	r12
142339f00: 41 5d                       	pop	r13
142339f02: 41 5e                       	pop	r14
142339f04: 41 5f                       	pop	r15
142339f06: 5d                          	pop	rbp
142339f07: c3                          	ret
