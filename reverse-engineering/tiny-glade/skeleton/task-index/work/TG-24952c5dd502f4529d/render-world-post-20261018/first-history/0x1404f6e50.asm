
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1404f6e50: 55                          	push	rbp
1404f6e51: 41 57                       	push	r15
1404f6e53: 41 56                       	push	r14
1404f6e55: 56                          	push	rsi
1404f6e56: 57                          	push	rdi
1404f6e57: 53                          	push	rbx
1404f6e58: 48 81 ec e8 01 00 00        	sub	rsp, 0x1e8
1404f6e5f: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
1404f6e67: 48 c7 85 60 01 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x160], -0x2
1404f6e72: 48 89 d7                    	mov	rdi, rdx
1404f6e75: 48 89 ce                    	mov	rsi, rcx
1404f6e78: 48 8b 9d c0 01 00 00        	mov	rbx, qword ptr [rbp + 0x1c0]
1404f6e7f: 4c 89 c1                    	mov	rcx, r8
1404f6e82: 4c 89 ca                    	mov	rdx, r9
1404f6e85: e8 b6 87 71 00              	call	0x140c0f640 <.text+0xc0e640>
1404f6e8a: 48 89 85 50 01 00 00        	mov	qword ptr [rbp + 0x150], rax
1404f6e91: 48 83 bf c8 00 00 00 00     	cmp	qword ptr [rdi + 0xc8], 0x0
1404f6e99: 0f 84 a3 02 00 00           	je	0x1404f7142 <.text+0x4f6142>
1404f6e9f: 48 8d 8f d0 00 00 00        	lea	rcx, [rdi + 0xd0]
1404f6ea6: 4c 8d bd 50 01 00 00        	lea	r15, [rbp + 0x150]
1404f6ead: 4c 89 fa                    	mov	rdx, r15
1404f6eb0: e8 8b a7 0b 00              	call	0x1405b1640 <.text+0x5b0640>
1404f6eb5: 4c 8b b7 b0 00 00 00        	mov	r14, qword ptr [rdi + 0xb0]
1404f6ebc: 48 8b 8f b8 00 00 00        	mov	rcx, qword ptr [rdi + 0xb8]
1404f6ec3: 48 89 ca                    	mov	rdx, rcx
1404f6ec6: 48 21 c2                    	and	rdx, rax
1404f6ec9: 48 c1 e8 39                 	shr	rax, 0x39
1404f6ecd: 66 0f 6e c0                 	movd	xmm0, eax
1404f6ed1: 66 0f 60 c0                 	punpcklbw	xmm0, xmm0      # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1404f6ed5: f2 0f 70 c0 00              	pshuflw	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1404f6eda: 66 0f 70 c0 00              	pshufd	xmm0, xmm0, 0x0         # xmm0 = xmm0[0,0,0,0]
1404f6edf: 4d 8d 46 98                 	lea	r8, [r14 - 0x68]
1404f6ee3: 45 31 c9                    	xor	r9d, r9d
1404f6ee6: 48 8b 85 50 01 00 00        	mov	rax, qword ptr [rbp + 0x150]
1404f6eed: 66 0f 76 c9                 	pcmpeqd	xmm1, xmm1
1404f6ef1: f3 41 0f 6f 14 16           	movdqu	xmm2, xmmword ptr [r14 + rdx]
1404f6ef7: 66 0f 6f da                 	movdqa	xmm3, xmm2
1404f6efb: 66 0f 74 d8                 	pcmpeqb	xmm3, xmm0
1404f6eff: 66 44 0f d7 d3              	pmovmskb	r10d, xmm3
1404f6f04: 45 85 d2                    	test	r10d, r10d
1404f6f07: 74 27                       	je	0x1404f6f30 <.text+0x4f5f30>
1404f6f09: f3 45 0f bc da              	tzcnt	r11d, r10d
1404f6f0e: 49 01 d3                    	add	r11, rdx
1404f6f11: 49 21 cb                    	and	r11, rcx
1404f6f14: 49 f7 db                    	neg	r11
1404f6f17: 4d 6b db 68                 	imul	r11, r11, 0x68
1404f6f1b: 4b 3b 04 18                 	cmp	rax, qword ptr [r8 + r11]
1404f6f1f: 74 31                       	je	0x1404f6f52 <.text+0x4f5f52>
1404f6f21: 45 8d 5a ff                 	lea	r11d, [r10 - 0x1]
1404f6f25: 66 45 21 d3                 	and	r11w, r10w
1404f6f29: 45 89 da                    	mov	r10d, r11d
1404f6f2c: 75 db                       	jne	0x1404f6f09 <.text+0x4f5f09>
1404f6f2e: 66 90                       	nop
1404f6f30: 66 0f 74 d1                 	pcmpeqb	xmm2, xmm1
1404f6f34: 66 44 0f d7 d2              	pmovmskb	r10d, xmm2
1404f6f39: 45 85 d2                    	test	r10d, r10d
1404f6f3c: 0f 85 00 02 00 00           	jne	0x1404f7142 <.text+0x4f6142>
1404f6f42: 4c 01 ca                    	add	rdx, r9
1404f6f45: 48 83 c2 10                 	add	rdx, 0x10
1404f6f49: 49 83 c1 10                 	add	r9, 0x10
1404f6f4d: 48 21 ca                    	and	rdx, rcx
1404f6f50: eb 9f                       	jmp	0x1404f6ef1 <.text+0x4f5ef1>
1404f6f52: 43 83 7c 1e a0 02           	cmp	dword ptr [r14 + r11 - 0x60], 0x2
1404f6f58: 0f 85 8f 03 00 00           	jne	0x1404f72ed <.text+0x4f62ed>
1404f6f5e: 4d 01 de                    	add	r14, r11
1404f6f61: 41 0f b6 4e fe              	movzx	ecx, byte ptr [r14 - 0x2]
1404f6f66: 80 f9 07                    	cmp	cl, 0x7
1404f6f69: 0f 84 ae 03 00 00           	je	0x1404f731d <.text+0x4f631d>
1404f6f6f: 41 8b 56 f4                 	mov	edx, dword ptr [r14 - 0xc]
1404f6f73: 4d 8b 46 ec                 	mov	r8, qword ptr [r14 - 0x14]
1404f6f77: 44 8b 4b 14                 	mov	r9d, dword ptr [rbx + 0x14]
1404f6f7b: 49 31 d1                    	xor	r9, rdx
1404f6f7e: 4c 33 43 0c                 	xor	r8, qword ptr [rbx + 0xc]
1404f6f82: 4d 09 c8                    	or	r8, r9
1404f6f85: 0f 85 ac 00 00 00           	jne	0x1404f7037 <.text+0x4f6037>
1404f6f8b: 3a 4b 1e                    	cmp	cl, byte ptr [rbx + 0x1e]
1404f6f8e: 0f 85 02 04 00 00           	jne	0x1404f7396 <.text+0x4f6396>
1404f6f94: 41 8b 46 e0                 	mov	eax, dword ptr [r14 - 0x20]
1404f6f98: 3b 03                       	cmp	eax, dword ptr [rbx]
1404f6f9a: 0f 85 f6 03 00 00           	jne	0x1404f7396 <.text+0x4f6396>
1404f6fa0: 41 8b 46 e4                 	mov	eax, dword ptr [r14 - 0x1c]
1404f6fa4: 3b 43 04                    	cmp	eax, dword ptr [rbx + 0x4]
1404f6fa7: 0f 85 e9 03 00 00           	jne	0x1404f7396 <.text+0x4f6396>
1404f6fad: 41 8b 46 e8                 	mov	eax, dword ptr [r14 - 0x18]
1404f6fb1: 3b 43 08                    	cmp	eax, dword ptr [rbx + 0x8]
1404f6fb4: 0f 85 dc 03 00 00           	jne	0x1404f7396 <.text+0x4f6396>
1404f6fba: 41 0f b6 46 fc              	movzx	eax, byte ptr [r14 - 0x4]
1404f6fbf: 3a 43 1c                    	cmp	al, byte ptr [rbx + 0x1c]
1404f6fc2: 0f 85 ce 03 00 00           	jne	0x1404f7396 <.text+0x4f6396>
1404f6fc8: 41 8b 46 f8                 	mov	eax, dword ptr [r14 - 0x8]
1404f6fcc: 3b 43 18                    	cmp	eax, dword ptr [rbx + 0x18]
1404f6fcf: 0f 85 c1 03 00 00           	jne	0x1404f7396 <.text+0x4f6396>
1404f6fd5: 41 0f b6 46 fd              	movzx	eax, byte ptr [r14 - 0x3]
1404f6fda: 3a 43 1d                    	cmp	al, byte ptr [rbx + 0x1d]
1404f6fdd: 0f 85 b3 03 00 00           	jne	0x1404f7396 <.text+0x4f6396>
1404f6fe3: 45 0f b6 4e a4              	movzx	r9d, byte ptr [r14 - 0x5c]
1404f6fe8: 49 8d 56 c8                 	lea	rdx, [r14 - 0x38]
1404f6fec: 48 8d 4d 14                 	lea	rcx, [rbp + 0x14]
1404f6ff0: 49 89 f8                    	mov	r8, rdi
1404f6ff3: e8 a8 f0 6d 00              	call	0x140bd60a0 <.text+0xbd50a0>
1404f6ff8: 8b 45 14                    	mov	eax, dword ptr [rbp + 0x14]
1404f6ffb: 41 c7 46 a0 00 00 00 00     	mov	dword ptr [r14 - 0x60], 0x0
1404f7003: 41 89 46 a4                 	mov	dword ptr [r14 - 0x5c], eax
1404f7007: 0f 10 45 18                 	movups	xmm0, xmmword ptr [rbp + 0x18]
1404f700b: 0f 10 4d 28                 	movups	xmm1, xmmword ptr [rbp + 0x28]
1404f700f: 41 0f 11 4e b8              	movups	xmmword ptr [r14 - 0x48], xmm1
1404f7014: 41 0f 11 46 a8              	movups	xmmword ptr [r14 - 0x58], xmm0
1404f7019: 0f 10 45 14                 	movups	xmm0, xmmword ptr [rbp + 0x14]
1404f701d: 0f 10 4d 24                 	movups	xmm1, xmmword ptr [rbp + 0x24]
1404f7021: 0f 11 06                    	movups	xmmword ptr [rsi], xmm0
1404f7024: 0f 11 4e 10                 	movups	xmmword ptr [rsi + 0x10], xmm1
1404f7028: 8b 45 34                    	mov	eax, dword ptr [rbp + 0x34]
1404f702b: 89 46 20                    	mov	dword ptr [rsi + 0x20], eax
1404f702e: c6 46 24 01                 	mov	byte ptr [rsi + 0x24], 0x1
1404f7032: e9 a3 02 00 00              	jmp	0x1404f72da <.text+0x4f62da>
1404f7037: 48 8b 0d 6a 53 13 03        	mov	rcx, qword ptr [rip + 0x313536a] # 0x14362c3a8
1404f703e: 48 8b 09                    	mov	rcx, qword ptr [rcx]
1404f7041: 48 83 f9 05                 	cmp	rcx, 0x5
1404f7045: 0f 82 f7 00 00 00           	jb	0x1404f7142 <.text+0x4f6142>
1404f704b: 49 83 c6 ec                 	add	r14, -0x14
1404f704f: 48 8d 43 0c                 	lea	rax, [rbx + 0xc]
1404f7053: 4c 89 7d 38                 	mov	qword ptr [rbp + 0x38], r15
1404f7057: 48 8d 0d 12 87 71 00        	lea	rcx, [rip + 0x718712]   # 0x140c0f770 <.text+0xc0e770>
1404f705e: 48 89 4d 40                 	mov	qword ptr [rbp + 0x40], rcx
1404f7062: 4c 89 75 48                 	mov	qword ptr [rbp + 0x48], r14
1404f7066: 48 8d 0d 93 fd 0b 00        	lea	rcx, [rip + 0xbfd93]    # 0x1405b6e00 <.text+0x5b5e00>
1404f706d: 48 89 4d 50                 	mov	qword ptr [rbp + 0x50], rcx
1404f7071: 48 89 45 58                 	mov	qword ptr [rbp + 0x58], rax
1404f7075: 48 89 4d 60                 	mov	qword ptr [rbp + 0x60], rcx
1404f7079: 48 8d 0d 10 5e 4f 02        	lea	rcx, [rip + 0x24f5e10]  # 0x1429ece90
1404f7080: e8 fb 6c 18 00              	call	0x14067dd80 <.text+0x67cd80>
1404f7085: 0f 10 00                    	movups	xmm0, xmmword ptr [rax]
1404f7088: 8b 40 10                    	mov	eax, dword ptr [rax + 0x10]
1404f708b: 48 c7 85 c8 00 00 00 05 00 00 00    	mov	qword ptr [rbp + 0xc8], 0x5
1404f7096: 48 8d 0d 0b 5e 4f 02        	lea	rcx, [rip + 0x24f5e0b]  # 0x1429ecea8
1404f709d: 48 89 8d d0 00 00 00        	mov	qword ptr [rbp + 0xd0], rcx
1404f70a4: 48 c7 85 d8 00 00 00 16 00 00 00    	mov	qword ptr [rbp + 0xd8], 0x16
1404f70af: 48 8d 15 aa 5d 4f 02        	lea	rdx, [rip + 0x24f5daa]  # 0x1429ece60
1404f70b6: 48 89 95 e8 00 00 00        	mov	qword ptr [rbp + 0xe8], rdx
1404f70bd: 48 c7 85 f0 00 00 00 03 00 00 00    	mov	qword ptr [rbp + 0xf0], 0x3
1404f70c8: 48 8d 55 38                 	lea	rdx, [rbp + 0x38]
1404f70cc: 48 89 95 f8 00 00 00        	mov	qword ptr [rbp + 0xf8], rdx
1404f70d3: 48 c7 85 00 01 00 00 03 00 00 00    	mov	qword ptr [rbp + 0x100], 0x3
1404f70de: 48 c7 85 08 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x108], 0x0
1404f70e9: 48 c7 85 98 00 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x98], 0x0
1404f70f4: 48 89 8d a0 00 00 00        	mov	qword ptr [rbp + 0xa0], rcx
1404f70fb: 48 c7 85 a8 00 00 00 16 00 00 00    	mov	qword ptr [rbp + 0xa8], 0x16
1404f7106: 48 c7 85 b0 00 00 00 00 00 00 00    	mov	qword ptr [rbp + 0xb0], 0x0
1404f7111: 0f 11 85 b8 00 00 00        	movups	xmmword ptr [rbp + 0xb8], xmm0
1404f7118: c7 85 e0 00 00 00 01 00 00 00       	mov	dword ptr [rbp + 0xe0], 0x1
1404f7122: 89 85 e4 00 00 00           	mov	dword ptr [rbp + 0xe4], eax
1404f7128: 48 8d 8d 18 01 00 00        	lea	rcx, [rbp + 0x118]
1404f712f: 48 8d 95 98 00 00 00        	lea	rdx, [rbp + 0x98]
1404f7136: e8 f5 31 85 00              	call	0x140d4a330 <.text+0xd49330>
1404f713b: 48 8b 85 50 01 00 00        	mov	rax, qword ptr [rbp + 0x150]
1404f7142: 48 8b 97 a8 00 00 00        	mov	rdx, qword ptr [rdi + 0xa8]
1404f7149: 4c 8b 48 f8                 	mov	r9, qword ptr [rax - 0x8]
1404f714d: 48 c7 45 b8 00 00 00 00     	mov	qword ptr [rbp - 0x48], 0x0
1404f7155: f3 0f 6f 03                 	movdqu	xmm0, xmmword ptr [rbx]
1404f7159: f3 0f 6f 4b 10              	movdqu	xmm1, xmmword ptr [rbx + 0x10]
1404f715e: 66 0f 7f 4d e0              	movdqa	xmmword ptr [rbp - 0x20], xmm1
1404f7163: 66 0f 7f 45 d0              	movdqa	xmmword ptr [rbp - 0x30], xmm0
1404f7168: 48 8d 4d b8                 	lea	rcx, [rbp - 0x48]
1404f716c: 48 89 4c 24 28              	mov	qword ptr [rsp + 0x28], rcx
1404f7171: 48 8d 4d d0                 	lea	rcx, [rbp - 0x30]
1404f7175: 48 89 4c 24 20              	mov	qword ptr [rsp + 0x20], rcx
1404f717a: 4c 8d b5 98 00 00 00        	lea	r14, [rbp + 0x98]
1404f7181: 4c 89 f1                    	mov	rcx, r14
1404f7184: 49 89 c0                    	mov	r8, rax
1404f7187: e8 64 34 0c 00              	call	0x1405ba5f0 <.text+0x5b95f0>
1404f718c: 48 8d 4d 38                 	lea	rcx, [rbp + 0x38]
1404f7190: 4c 89 f2                    	mov	rdx, r14
1404f7193: 49 89 d8                    	mov	r8, rbx
1404f7196: e8 25 ab e1 ff              	call	0x140311cc0 <.text+0x310cc0>
1404f719b: 0f b6 45 6e                 	movzx	eax, byte ptr [rbp + 0x6e]
1404f719f: 48 8b 4d 38                 	mov	rcx, qword ptr [rbp + 0x38]
1404f71a3: 3c 07                       	cmp	al, 0x7
1404f71a5: 75 0c                       	jne	0x1404f71b3 <.text+0x4f61b3>
1404f71a7: 48 89 0e                    	mov	qword ptr [rsi], rcx
1404f71aa: c6 46 24 02                 	mov	byte ptr [rsi + 0x24], 0x2
1404f71ae: e9 27 01 00 00              	jmp	0x1404f72da <.text+0x4f62da>
1404f71b3: 0f 10 45 40                 	movups	xmm0, xmmword ptr [rbp + 0x40]
1404f71b7: 0f 10 4d 50                 	movups	xmm1, xmmword ptr [rbp + 0x50]
1404f71bb: f3 0f 6f 55 5e              	movdqu	xmm2, xmmword ptr [rbp + 0x5e]
1404f71c0: f3 0f 7f 95 3e 01 00 00     	movdqu	xmmword ptr [rbp + 0x13e], xmm2
1404f71c8: 0f 11 8d 30 01 00 00        	movups	xmmword ptr [rbp + 0x130], xmm1
1404f71cf: 0f 11 85 20 01 00 00        	movups	xmmword ptr [rbp + 0x120], xmm0
1404f71d6: 0f b6 55 6f                 	movzx	edx, byte ptr [rbp + 0x6f]
1404f71da: 48 89 8d 18 01 00 00        	mov	qword ptr [rbp + 0x118], rcx
1404f71e1: 88 85 4e 01 00 00           	mov	byte ptr [rbp + 0x14e], al
1404f71e7: 88 95 4f 01 00 00           	mov	byte ptr [rbp + 0x14f], dl
1404f71ed: c6 85 5f 01 00 00 01        	mov	byte ptr [rbp + 0x15f], 0x1
1404f71f4: 48 8d 4d f0                 	lea	rcx, [rbp - 0x10]
1404f71f8: 48 8d 95 18 01 00 00        	lea	rdx, [rbp + 0x118]
1404f71ff: 49 89 f8                    	mov	r8, rdi
1404f7202: 45 31 c9                    	xor	r9d, r9d
1404f7205: e8 96 ee 6d 00              	call	0x140bd60a0 <.text+0xbd50a0>
1404f720a: 48 81 c7 b0 00 00 00        	add	rdi, 0xb0
1404f7211: 48 8b 85 48 01 00 00        	mov	rax, qword ptr [rbp + 0x148]
1404f7218: 48 89 85 f0 00 00 00        	mov	qword ptr [rbp + 0xf0], rax
1404f721f: 0f 10 85 18 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x118]
1404f7226: 0f 10 8d 28 01 00 00        	movups	xmm1, xmmword ptr [rbp + 0x128]
1404f722d: f3 0f 6f 95 38 01 00 00     	movdqu	xmm2, xmmword ptr [rbp + 0x138]
1404f7235: f3 0f 7f 95 e0 00 00 00     	movdqu	xmmword ptr [rbp + 0xe0], xmm2
1404f723d: 0f 11 8d d0 00 00 00        	movups	xmmword ptr [rbp + 0xd0], xmm1
1404f7244: 0f 11 85 c0 00 00 00        	movups	xmmword ptr [rbp + 0xc0], xmm0
1404f724b: 8b 45 f0                    	mov	eax, dword ptr [rbp - 0x10]
1404f724e: 0f 10 45 f4                 	movups	xmm0, xmmword ptr [rbp - 0xc]
1404f7252: 0f 11 85 a0 00 00 00        	movups	xmmword ptr [rbp + 0xa0], xmm0
1404f7259: 0f 10 45 04                 	movups	xmm0, xmmword ptr [rbp + 0x4]
1404f725d: 0f 11 85 b0 00 00 00        	movups	xmmword ptr [rbp + 0xb0], xmm0
1404f7264: 89 85 9c 00 00 00           	mov	dword ptr [rbp + 0x9c], eax
1404f726a: c7 85 98 00 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x98], 0x0
1404f7274: 4c 8b 85 50 01 00 00        	mov	r8, qword ptr [rbp + 0x150]
1404f727b: c6 85 5f 01 00 00 00        	mov	byte ptr [rbp + 0x15f], 0x0
1404f7282: 48 8d 4d 38                 	lea	rcx, [rbp + 0x38]
1404f7286: 4c 8d 8d 98 00 00 00        	lea	r9, [rbp + 0x98]
1404f728d: 48 89 fa                    	mov	rdx, rdi
1404f7290: e8 bb 1c dd ff              	call	0x1402c8f50 <.text+0x2c7f50>
1404f7295: 83 7d 38 03                 	cmp	dword ptr [rbp + 0x38], 0x3
1404f7299: 74 22                       	je	0x1404f72bd <.text+0x4f62bd>
1404f729b: 80 bd 96 00 00 00 07        	cmp	byte ptr [rbp + 0x96], 0x7
1404f72a2: 74 09                       	je	0x1404f72ad <.text+0x4f62ad>
1404f72a4: 48 8d 4d 60                 	lea	rcx, [rbp + 0x60]
1404f72a8: e8 d3 48 b4 ff              	call	0x14003bb80 <.text+0x3ab80>
1404f72ad: c6 85 5f 01 00 00 00        	mov	byte ptr [rbp + 0x15f], 0x0
1404f72b4: 48 8d 4d 60                 	lea	rcx, [rbp + 0x60]
1404f72b8: e8 f3 b1 6e 00              	call	0x140be24b0 <.text+0xbe14b0>
1404f72bd: 8b 45 10                    	mov	eax, dword ptr [rbp + 0x10]
1404f72c0: 89 46 20                    	mov	dword ptr [rsi + 0x20], eax
1404f72c3: f3 0f 6f 45 f0              	movdqu	xmm0, xmmword ptr [rbp - 0x10]
1404f72c8: f3 0f 6f 4d 00              	movdqu	xmm1, xmmword ptr [rbp]
1404f72cd: f3 0f 7f 4e 10              	movdqu	xmmword ptr [rsi + 0x10], xmm1
1404f72d2: f3 0f 7f 06                 	movdqu	xmmword ptr [rsi], xmm0
1404f72d6: c6 46 24 00                 	mov	byte ptr [rsi + 0x24], 0x0
1404f72da: 48 89 f0                    	mov	rax, rsi
1404f72dd: 48 81 c4 e8 01 00 00        	add	rsp, 0x1e8
1404f72e4: 5b                          	pop	rbx
1404f72e5: 5f                          	pop	rdi
1404f72e6: 5e                          	pop	rsi
1404f72e7: 41 5e                       	pop	r14
1404f72e9: 41 5f                       	pop	r15
1404f72eb: 5d                          	pop	rbp
1404f72ec: c3                          	ret
1404f72ed: 4c 89 bd 18 01 00 00        	mov	qword ptr [rbp + 0x118], r15
1404f72f4: 48 8d 05 75 84 71 00        	lea	rax, [rip + 0x718475]   # 0x140c0f770 <.text+0xc0e770>
1404f72fb: 48 89 85 20 01 00 00        	mov	qword ptr [rbp + 0x120], rax
1404f7302: 48 8d 05 07 5b 4f 02        	lea	rax, [rip + 0x24f5b07]  # 0x1429ece10
1404f7309: 48 89 85 98 00 00 00        	mov	qword ptr [rbp + 0x98], rax
1404f7310: 48 c7 85 a0 00 00 00 01 00 00 00    	mov	qword ptr [rbp + 0xa0], 0x1
1404f731b: eb 2e                       	jmp	0x1404f734b <.text+0x4f634b>
1404f731d: 4c 89 bd 18 01 00 00        	mov	qword ptr [rbp + 0x118], r15
1404f7324: 48 8d 05 45 84 71 00        	lea	rax, [rip + 0x718445]   # 0x140c0f770 <.text+0xc0e770>
1404f732b: 48 89 85 20 01 00 00        	mov	qword ptr [rbp + 0x120], rax
1404f7332: 48 8d 05 c7 5b 4f 02        	lea	rax, [rip + 0x24f5bc7]  # 0x1429ecf00
1404f7339: 48 89 85 98 00 00 00        	mov	qword ptr [rbp + 0x98], rax
1404f7340: 48 c7 85 a0 00 00 00 02 00 00 00    	mov	qword ptr [rbp + 0xa0], 0x2
1404f734b: 48 8d 85 18 01 00 00        	lea	rax, [rbp + 0x118]
1404f7352: 48 89 85 a8 00 00 00        	mov	qword ptr [rbp + 0xa8], rax
1404f7359: 48 c7 85 b0 00 00 00 01 00 00 00    	mov	qword ptr [rbp + 0xb0], 0x1
1404f7364: 48 c7 85 b8 00 00 00 00 00 00 00    	mov	qword ptr [rbp + 0xb8], 0x0
1404f736f: 48 8d 7d 38                 	lea	rdi, [rbp + 0x38]
1404f7373: 48 8d 95 98 00 00 00        	lea	rdx, [rbp + 0x98]
1404f737a: 48 89 f9                    	mov	rcx, rdi
1404f737d: e8 0e 2c 88 00              	call	0x140d79f90 <.text+0xd78f90>
1404f7382: 48 89 f9                    	mov	rcx, rdi
1404f7385: e8 96 b7 3a 02              	call	0x1428a2b20 <.text+0x28a1b20>
1404f738a: 48 89 06                    	mov	qword ptr [rsi], rax
1404f738d: c6 46 24 02                 	mov	byte ptr [rsi + 0x24], 0x2
1404f7391: e9 44 ff ff ff              	jmp	0x1404f72da <.text+0x4f62da>
1404f7396: 48 c7 85 98 00 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x98], 0x0
1404f73a1: 49 83 c6 e0                 	add	r14, -0x20
1404f73a5: 48 8d 05 14 5b 4f 02        	lea	rax, [rip + 0x24f5b14]  # 0x1429ecec0
1404f73ac: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1404f73b1: 4c 8d 8d 98 00 00 00        	lea	r9, [rbp + 0x98]
1404f73b8: 31 c9                       	xor	ecx, ecx
1404f73ba: 4c 89 f2                    	mov	rdx, r14
1404f73bd: 49 89 d8                    	mov	r8, rbx
1404f73c0: e8 c5 25 3b 02              	call	0x1428a998a <.text+0x28a898a>
1404f73c5: cc                          	int3
1404f73c6: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
1404f73d0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1404f73d5: 55                          	push	rbp
1404f73d6: 41 57                       	push	r15
1404f73d8: 41 56                       	push	r14
1404f73da: 56                          	push	rsi
1404f73db: 57                          	push	rdi
1404f73dc: 53                          	push	rbx
1404f73dd: 48 83 ec 38                 	sub	rsp, 0x38
1404f73e1: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1404f73e8: c6 85 5f 01 00 00 00        	mov	byte ptr [rbp + 0x15f], 0x0
1404f73ef: 48 8d 4d 60                 	lea	rcx, [rbp + 0x60]
1404f73f3: e8 b8 b0 6e 00              	call	0x140be24b0 <.text+0xbe14b0>
1404f73f8: c6 85 5f 01 00 00 00        	mov	byte ptr [rbp + 0x15f], 0x0
1404f73ff: 48 83 c4 38                 	add	rsp, 0x38
1404f7403: 5b                          	pop	rbx
1404f7404: 5f                          	pop	rdi
1404f7405: 5e                          	pop	rsi
1404f7406: 41 5e                       	pop	r14
1404f7408: 41 5f                       	pop	r15
1404f740a: 5d                          	pop	rbp
1404f740b: c3                          	ret
1404f740c: 0f 1f 40 00                 	nop	dword ptr [rax]
1404f7410: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1404f7415: 55                          	push	rbp
1404f7416: 41 57                       	push	r15
1404f7418: 41 56                       	push	r14
1404f741a: 56                          	push	rsi
1404f741b: 57                          	push	rdi
1404f741c: 53                          	push	rbx
1404f741d: 48 83 ec 38                 	sub	rsp, 0x38
1404f7421: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1404f7428: 80 bd 5f 01 00 00 00        	cmp	byte ptr [rbp + 0x15f], 0x0
1404f742f: 75 0d                       	jne	0x1404f743e <.text+0x4f643e>
1404f7431: 48 83 c4 38                 	add	rsp, 0x38
1404f7435: 5b                          	pop	rbx
1404f7436: 5f                          	pop	rdi
1404f7437: 5e                          	pop	rsi
1404f7438: 41 5e                       	pop	r14
1404f743a: 41 5f                       	pop	r15
1404f743c: 5d                          	pop	rbp
1404f743d: c3                          	ret
1404f743e: 48 8d 8d 18 01 00 00        	lea	rcx, [rbp + 0x118]
1404f7445: e8 96 71 0f 00              	call	0x1405ee5e0 <.text+0x5ed5e0>
1404f744a: eb e5                       	jmp	0x1404f7431 <.text+0x4f6431>
