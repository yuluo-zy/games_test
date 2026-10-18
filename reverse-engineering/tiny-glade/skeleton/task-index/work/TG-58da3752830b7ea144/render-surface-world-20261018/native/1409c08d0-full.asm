
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1409c08d0: 55                          	push	rbp
1409c08d1: 41 57                       	push	r15
1409c08d3: 41 56                       	push	r14
1409c08d5: 41 55                       	push	r13
1409c08d7: 41 54                       	push	r12
1409c08d9: 56                          	push	rsi
1409c08da: 57                          	push	rdi
1409c08db: 53                          	push	rbx
1409c08dc: 48 81 ec 28 0e 00 00        	sub	rsp, 0xe28
1409c08e3: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
1409c08eb: 48 c7 85 a0 0d 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0xda0], -0x2
1409c08f6: 4d 89 cf                    	mov	r15, r9
1409c08f9: 4d 89 c6                    	mov	r14, r8
1409c08fc: 49 89 d5                    	mov	r13, rdx
1409c08ff: 49 89 cc                    	mov	r12, rcx
1409c0902: 48 8b 9d 10 0e 00 00        	mov	rbx, qword ptr [rbp + 0xe10]
1409c0909: 49 8d 70 04                 	lea	rsi, [r8 + 0x4]
1409c090d: 48 89 f1                    	mov	rcx, rsi
1409c0910: e8 7b 4e 21 00              	call	0x140bd5790 <.text+0xbd4790>
1409c0915: 48 89 c7                    	mov	rdi, rax
1409c0918: 48 89 f1                    	mov	rcx, rsi
1409c091b: e8 70 4e 21 00              	call	0x140bd5790 <.text+0xbd4790>
1409c0920: 48 8d 8d e0 0b 00 00        	lea	rcx, [rbp + 0xbe0]
1409c0927: 48 89 c2                    	mov	rdx, rax
1409c092a: 41 b8 09 00 00 00           	mov	r8d, 0x9
1409c0930: e8 6b 49 21 00              	call	0x140bd52a0 <.text+0xbd42a0>
1409c0935: 0f 10 85 e0 0b 00 00        	movups	xmm0, xmmword ptr [rbp + 0xbe0]
1409c093c: 0f 10 8d f0 0b 00 00        	movups	xmm1, xmmword ptr [rbp + 0xbf0]
1409c0943: 0f 29 8d c0 02 00 00        	movaps	xmmword ptr [rbp + 0x2c0], xmm1
1409c094a: 0f 29 85 b0 02 00 00        	movaps	xmmword ptr [rbp + 0x2b0], xmm0
1409c0951: 48 8d 8d c0 07 00 00        	lea	rcx, [rbp + 0x7c0]
1409c0958: 48 8d 95 b0 02 00 00        	lea	rdx, [rbp + 0x2b0]
1409c095f: e8 7c 63 21 00              	call	0x140bd6ce0 <.text+0xbd5ce0>
1409c0964: 48 8d 05 1e f3 0f 02        	lea	rax, [rip + 0x20ff31e]  # 0x142abfc89
1409c096b: 48 89 85 b0 07 00 00        	mov	qword ptr [rbp + 0x7b0], rax
1409c0972: 48 c7 85 b8 07 00 00 11 00 00 00    	mov	qword ptr [rbp + 0x7b8], 0x11
1409c097d: 48 8d 95 b0 07 00 00        	lea	rdx, [rbp + 0x7b0]
1409c0984: 4c 89 e1                    	mov	rcx, r12
1409c0987: e8 44 8b 22 00              	call	0x140be94d0 <.text+0xbe84d0>
1409c098c: 0f 10 85 e0 0b 00 00        	movups	xmm0, xmmword ptr [rbp + 0xbe0]
1409c0993: 0f 10 8d f0 0b 00 00        	movups	xmm1, xmmword ptr [rbp + 0xbf0]
1409c099a: 0f 11 85 10 02 00 00        	movups	xmmword ptr [rbp + 0x210], xmm0
1409c09a1: 0f 11 8d 20 02 00 00        	movups	xmmword ptr [rbp + 0x220], xmm1
1409c09a8: 89 85 0c 02 00 00           	mov	dword ptr [rbp + 0x20c], eax
1409c09ae: 48 89 f1                    	mov	rcx, rsi
1409c09b1: e8 da 4d 21 00              	call	0x140bd5790 <.text+0xbd4790>
1409c09b6: 48 8d 8d e0 0b 00 00        	lea	rcx, [rbp + 0xbe0]
1409c09bd: 48 89 c2                    	mov	rdx, rax
1409c09c0: 41 b8 62 00 00 00           	mov	r8d, 0x62
1409c09c6: e8 d5 48 21 00              	call	0x140bd52a0 <.text+0xbd42a0>
1409c09cb: 0f 10 85 e0 0b 00 00        	movups	xmm0, xmmword ptr [rbp + 0xbe0]
1409c09d2: 0f 10 8d f0 0b 00 00        	movups	xmm1, xmmword ptr [rbp + 0xbf0]
1409c09d9: 0f 29 8d c0 02 00 00        	movaps	xmmword ptr [rbp + 0x2c0], xmm1
1409c09e0: 0f 29 85 b0 02 00 00        	movaps	xmmword ptr [rbp + 0x2b0], xmm0
1409c09e7: 48 8d 8d c0 07 00 00        	lea	rcx, [rbp + 0x7c0]
1409c09ee: 48 8d 95 b0 02 00 00        	lea	rdx, [rbp + 0x2b0]
1409c09f5: e8 e6 62 21 00              	call	0x140bd6ce0 <.text+0xbd5ce0>
1409c09fa: 48 8d 05 99 f2 0f 02        	lea	rax, [rip + 0x20ff299]  # 0x142abfc9a
1409c0a01: 48 89 85 b0 07 00 00        	mov	qword ptr [rbp + 0x7b0], rax
1409c0a08: 48 c7 85 b8 07 00 00 0d 00 00 00    	mov	qword ptr [rbp + 0x7b8], 0xd
1409c0a13: 48 8d 95 b0 07 00 00        	lea	rdx, [rbp + 0x7b0]
1409c0a1a: 4c 89 e1                    	mov	rcx, r12
1409c0a1d: e8 ae 8a 22 00              	call	0x140be94d0 <.text+0xbe84d0>
1409c0a22: 0f 10 85 e0 0b 00 00        	movups	xmm0, xmmword ptr [rbp + 0xbe0]
1409c0a29: 0f 10 8d f0 0b 00 00        	movups	xmm1, xmmword ptr [rbp + 0xbf0]
1409c0a30: 0f 11 85 a4 01 00 00        	movups	xmmword ptr [rbp + 0x1a4], xmm0
1409c0a37: 0f 11 8d b4 01 00 00        	movups	xmmword ptr [rbp + 0x1b4], xmm1
1409c0a3e: 89 85 a0 01 00 00           	mov	dword ptr [rbp + 0x1a0], eax
1409c0a44: 48 89 b5 78 0d 00 00        	mov	qword ptr [rbp + 0xd78], rsi
1409c0a4b: 48 89 f1                    	mov	rcx, rsi
1409c0a4e: e8 3d 4d 21 00              	call	0x140bd5790 <.text+0xbd4790>
1409c0a53: 48 8d b5 b0 07 00 00        	lea	rsi, [rbp + 0x7b0]
1409c0a5a: 48 89 f1                    	mov	rcx, rsi
1409c0a5d: 48 89 c2                    	mov	rdx, rax
1409c0a60: 41 b8 62 00 00 00           	mov	r8d, 0x62
1409c0a66: e8 35 48 21 00              	call	0x140bd52a0 <.text+0xbd42a0>
1409c0a6b: 48 8d 8d e0 0b 00 00        	lea	rcx, [rbp + 0xbe0]
1409c0a72: 48 89 f2                    	mov	rdx, rsi
1409c0a75: e8 96 49 21 00              	call	0x140bd5410 <.text+0xbd4410>
1409c0a7a: 0f 10 85 e0 0b 00 00        	movups	xmm0, xmmword ptr [rbp + 0xbe0]
1409c0a81: 0f 10 8d f0 0b 00 00        	movups	xmm1, xmmword ptr [rbp + 0xbf0]
1409c0a88: 0f 29 8d c0 02 00 00        	movaps	xmmword ptr [rbp + 0x2c0], xmm1
1409c0a8f: 0f 29 85 b0 02 00 00        	movaps	xmmword ptr [rbp + 0x2b0], xmm0
1409c0a96: 48 8d 8d c0 07 00 00        	lea	rcx, [rbp + 0x7c0]
1409c0a9d: 48 8d 95 b0 02 00 00        	lea	rdx, [rbp + 0x2b0]
1409c0aa4: e8 37 62 21 00              	call	0x140bd6ce0 <.text+0xbd5ce0>
1409c0aa9: 48 8d 05 f7 f1 0f 02        	lea	rax, [rip + 0x20ff1f7]  # 0x142abfca7
1409c0ab0: 48 89 85 b0 07 00 00        	mov	qword ptr [rbp + 0x7b0], rax
1409c0ab7: 48 c7 85 b8 07 00 00 15 00 00 00    	mov	qword ptr [rbp + 0x7b8], 0x15
1409c0ac2: 48 8d 95 b0 07 00 00        	lea	rdx, [rbp + 0x7b0]
1409c0ac9: 4c 89 e1                    	mov	rcx, r12
1409c0acc: e8 ff 89 22 00              	call	0x140be94d0 <.text+0xbe84d0>
1409c0ad1: 0f 10 85 e0 0b 00 00        	movups	xmm0, xmmword ptr [rbp + 0xbe0]
1409c0ad8: 0f 10 8d f0 0b 00 00        	movups	xmm1, xmmword ptr [rbp + 0xbf0]
1409c0adf: 0f 11 85 80 01 00 00        	movups	xmmword ptr [rbp + 0x180], xmm0
1409c0ae6: 0f 11 8d 90 01 00 00        	movups	xmmword ptr [rbp + 0x190], xmm1
1409c0aed: 89 85 7c 01 00 00           	mov	dword ptr [rbp + 0x17c], eax
1409c0af3: 48 8d 8d e0 0b 00 00        	lea	rcx, [rbp + 0xbe0]
1409c0afa: ba 10 00 00 00              	mov	edx, 0x10
1409c0aff: 41 b8 20 00 00 00           	mov	r8d, 0x20
1409c0b05: e8 16 43 21 00              	call	0x140bd4e20 <.text+0xbd3e20>
1409c0b0a: 0f 10 85 e0 0b 00 00        	movups	xmm0, xmmword ptr [rbp + 0xbe0]
1409c0b11: 0f 29 85 b0 02 00 00        	movaps	xmmword ptr [rbp + 0x2b0], xmm0
1409c0b18: 48 8d 8d c0 07 00 00        	lea	rcx, [rbp + 0x7c0]
1409c0b1f: 48 8d 95 b0 02 00 00        	lea	rdx, [rbp + 0x2b0]
1409c0b26: e8 d5 61 21 00              	call	0x140bd6d00 <.text+0xbd5d00>
1409c0b2b: 48 8d 05 8a f1 0f 02        	lea	rax, [rip + 0x20ff18a]  # 0x142abfcbc
1409c0b32: 48 89 85 b0 07 00 00        	mov	qword ptr [rbp + 0x7b0], rax
1409c0b39: 48 c7 85 b8 07 00 00 15 00 00 00    	mov	qword ptr [rbp + 0x7b8], 0x15
1409c0b44: 48 8d 95 b0 07 00 00        	lea	rdx, [rbp + 0x7b0]
1409c0b4b: 4c 89 e1                    	mov	rcx, r12
1409c0b4e: e8 7d 89 22 00              	call	0x140be94d0 <.text+0xbe84d0>
1409c0b53: 0f 10 85 e0 0b 00 00        	movups	xmm0, xmmword ptr [rbp + 0xbe0]
1409c0b5a: 0f 29 85 c0 0b 00 00        	movaps	xmmword ptr [rbp + 0xbc0], xmm0
1409c0b61: 89 85 d0 0b 00 00           	mov	dword ptr [rbp + 0xbd0], eax
1409c0b67: 48 8d 8d e0 0b 00 00        	lea	rcx, [rbp + 0xbe0]
1409c0b6e: ba 10 00 00 00              	mov	edx, 0x10
1409c0b73: 41 b8 20 00 00 00           	mov	r8d, 0x20
1409c0b79: e8 a2 42 21 00              	call	0x140bd4e20 <.text+0xbd3e20>
1409c0b7e: 0f 10 85 e0 0b 00 00        	movups	xmm0, xmmword ptr [rbp + 0xbe0]
1409c0b85: 0f 29 85 b0 02 00 00        	movaps	xmmword ptr [rbp + 0x2b0], xmm0
1409c0b8c: 48 8d 8d c0 07 00 00        	lea	rcx, [rbp + 0x7c0]
1409c0b93: 48 8d 95 b0 02 00 00        	lea	rdx, [rbp + 0x2b0]
1409c0b9a: e8 61 61 21 00              	call	0x140bd6d00 <.text+0xbd5d00>
1409c0b9f: 48 8d 05 2b f1 0f 02        	lea	rax, [rip + 0x20ff12b]  # 0x142abfcd1
1409c0ba6: 48 89 85 b0 07 00 00        	mov	qword ptr [rbp + 0x7b0], rax
1409c0bad: 48 c7 85 b8 07 00 00 13 00 00 00    	mov	qword ptr [rbp + 0x7b8], 0x13
1409c0bb8: 48 8d 95 b0 07 00 00        	lea	rdx, [rbp + 0x7b0]
1409c0bbf: 4c 89 e1                    	mov	rcx, r12
1409c0bc2: e8 09 89 22 00              	call	0x140be94d0 <.text+0xbe84d0>
1409c0bc7: 0f 10 85 e0 0b 00 00        	movups	xmm0, xmmword ptr [rbp + 0xbe0]
1409c0bce: 0f 29 85 a0 06 00 00        	movaps	xmmword ptr [rbp + 0x6a0], xmm0
1409c0bd5: 89 85 b0 06 00 00           	mov	dword ptr [rbp + 0x6b0], eax
1409c0bdb: 48 c7 44 24 20 04 00 00 00  	mov	qword ptr [rsp + 0x20], 0x4
1409c0be4: 48 8d 35 f9 f0 0f 02        	lea	rsi, [rip + 0x20ff0f9]  # 0x142abfce4
1409c0beb: 4c 8d 85 c0 0b 00 00        	lea	r8, [rbp + 0xbc0]
1409c0bf2: 4c 89 e1                    	mov	rcx, r12
1409c0bf5: 4c 89 ea                    	mov	rdx, r13
1409c0bf8: 49 89 f1                    	mov	r9, rsi
1409c0bfb: e8 a0 cd f4 ff              	call	0x14090d9a0 <.text+0x90c9a0>
1409c0c00: 48 c7 44 24 20 04 00 00 00  	mov	qword ptr [rsp + 0x20], 0x4
1409c0c09: 4c 8d 85 a0 06 00 00        	lea	r8, [rbp + 0x6a0]
1409c0c10: 4c 89 e1                    	mov	rcx, r12
1409c0c13: 4c 89 ea                    	mov	rdx, r13
1409c0c16: 49 89 f1                    	mov	r9, rsi
1409c0c19: e8 82 cd f4 ff              	call	0x14090d9a0 <.text+0x90c9a0>
1409c0c1e: 48 8d 4b 04                 	lea	rcx, [rbx + 0x4]
1409c0c22: e8 69 4b 21 00              	call	0x140bd5790 <.text+0xbd4790>
1409c0c27: 48 89 c1                    	mov	rcx, rax
1409c0c2a: 48 c1 e9 20                 	shr	rcx, 0x20
1409c0c2e: 0f af c1                    	imul	eax, ecx
1409c0c31: 89 c6                       	mov	esi, eax
1409c0c33: c1 ee 04                    	shr	esi, 0x4
1409c0c36: 01 c6                       	add	esi, eax
1409c0c38: 48 8d 14 b5 00 00 00 00     	lea	rdx, [4*rsi]
1409c0c40: 48 8d 8d e0 0b 00 00        	lea	rcx, [rbp + 0xbe0]
1409c0c47: 41 b8 20 00 00 00           	mov	r8d, 0x20
1409c0c4d: e8 ce 41 21 00              	call	0x140bd4e20 <.text+0xbd3e20>
1409c0c52: 0f 10 85 e0 0b 00 00        	movups	xmm0, xmmword ptr [rbp + 0xbe0]
1409c0c59: 0f 29 85 b0 02 00 00        	movaps	xmmword ptr [rbp + 0x2b0], xmm0
1409c0c60: 48 8d 8d c0 07 00 00        	lea	rcx, [rbp + 0x7c0]
1409c0c67: 48 8d 95 b0 02 00 00        	lea	rdx, [rbp + 0x2b0]
1409c0c6e: e8 8d 60 21 00              	call	0x140bd6d00 <.text+0xbd5d00>
1409c0c73: 48 8d 05 7a f0 0f 02        	lea	rax, [rip + 0x20ff07a]  # 0x142abfcf4
1409c0c7a: 48 89 85 b0 07 00 00        	mov	qword ptr [rbp + 0x7b0], rax
1409c0c81: 48 c7 85 b8 07 00 00 12 00 00 00    	mov	qword ptr [rbp + 0x7b8], 0x12
1409c0c8c: 48 8d 95 b0 07 00 00        	lea	rdx, [rbp + 0x7b0]
1409c0c93: 4c 89 e1                    	mov	rcx, r12
1409c0c96: e8 35 88 22 00              	call	0x140be94d0 <.text+0xbe84d0>
1409c0c9b: 0f 10 85 e0 0b 00 00        	movups	xmm0, xmmword ptr [rbp + 0xbe0]
1409c0ca2: 0f 29 85 60 02 00 00        	movaps	xmmword ptr [rbp + 0x260], xmm0
1409c0ca9: 89 85 70 02 00 00           	mov	dword ptr [rbp + 0x270], eax
1409c0caf: 48 c1 e6 04                 	shl	rsi, 0x4
1409c0cb3: 48 8d 9d e0 0b 00 00        	lea	rbx, [rbp + 0xbe0]
1409c0cba: 48 89 d9                    	mov	rcx, rbx
1409c0cbd: 48 89 f2                    	mov	rdx, rsi
1409c0cc0: 41 b8 20 00 00 00           	mov	r8d, 0x20
1409c0cc6: e8 55 41 21 00              	call	0x140bd4e20 <.text+0xbd3e20>
1409c0ccb: 0f 10 85 e0 0b 00 00        	movups	xmm0, xmmword ptr [rbp + 0xbe0]
1409c0cd2: 0f 29 85 b0 02 00 00        	movaps	xmmword ptr [rbp + 0x2b0], xmm0
1409c0cd9: 48 8d 8d c0 07 00 00        	lea	rcx, [rbp + 0x7c0]
1409c0ce0: 48 8d 95 b0 02 00 00        	lea	rdx, [rbp + 0x2b0]
1409c0ce7: e8 14 60 21 00              	call	0x140bd6d00 <.text+0xbd5d00>
1409c0cec: 48 8d 05 13 f0 0f 02        	lea	rax, [rip + 0x20ff013]  # 0x142abfd06
1409c0cf3: 48 89 85 b0 07 00 00        	mov	qword ptr [rbp + 0x7b0], rax
1409c0cfa: 48 c7 85 b8 07 00 00 11 00 00 00    	mov	qword ptr [rbp + 0x7b8], 0x11
1409c0d05: 48 8d 95 b0 07 00 00        	lea	rdx, [rbp + 0x7b0]
1409c0d0c: 4c 89 e1                    	mov	rcx, r12
1409c0d0f: e8 bc 87 22 00              	call	0x140be94d0 <.text+0xbe84d0>
1409c0d14: 0f 10 85 e0 0b 00 00        	movups	xmm0, xmmword ptr [rbp + 0xbe0]
1409c0d1b: 0f 29 85 60 01 00 00        	movaps	xmmword ptr [rbp + 0x160], xmm0
1409c0d22: 89 85 70 01 00 00           	mov	dword ptr [rbp + 0x170], eax
1409c0d28: 48 8d b5 b0 07 00 00        	lea	rsi, [rbp + 0x7b0]
1409c0d2f: 48 89 f1                    	mov	rcx, rsi
1409c0d32: 48 89 bd a8 0b 00 00        	mov	qword ptr [rbp + 0xba8], rdi
1409c0d39: 48 89 fa                    	mov	rdx, rdi
1409c0d3c: 41 b8 09 00 00 00           	mov	r8d, 0x9
1409c0d42: e8 59 45 21 00              	call	0x140bd52a0 <.text+0xbd42a0>
1409c0d47: 48 8d bd b0 02 00 00        	lea	rdi, [rbp + 0x2b0]
1409c0d4e: 48 89 f9                    	mov	rcx, rdi
1409c0d51: 48 89 f2                    	mov	rdx, rsi
1409c0d54: e8 17 46 21 00              	call	0x140bd5370 <.text+0xbd4370>
1409c0d59: 48 89 d9                    	mov	rcx, rbx
1409c0d5c: 48 89 fa                    	mov	rdx, rdi
1409c0d5f: e8 6c 46 21 00              	call	0x140bd53d0 <.text+0xbd43d0>
1409c0d64: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
1409c0d69: 4c 8d 05 a7 ef 0f 02        	lea	r8, [rip + 0x20fefa7]   # 0x142abfd17
1409c0d70: 48 8d 8d b0 07 00 00        	lea	rcx, [rbp + 0x7b0]
1409c0d77: 41 b9 0b 00 00 00           	mov	r9d, 0xb
1409c0d7d: 4c 89 e2                    	mov	rdx, r12
1409c0d80: e8 0b c7 fa ff              	call	0x14096d490 <.text+0x96c490>
1409c0d85: 83 bd b0 07 00 00 01        	cmp	dword ptr [rbp + 0x7b0], 0x1
1409c0d8c: 0f 84 47 1d 00 00           	je	0x1409c2ad9 <.text+0x9c1ad9>
1409c0d92: 48 8b 85 f4 07 00 00        	mov	rax, qword ptr [rbp + 0x7f4]
1409c0d99: 48 89 85 f0 02 00 00        	mov	qword ptr [rbp + 0x2f0], rax
1409c0da0: 0f 10 85 b4 07 00 00        	movups	xmm0, xmmword ptr [rbp + 0x7b4]
1409c0da7: 0f 10 8d c4 07 00 00        	movups	xmm1, xmmword ptr [rbp + 0x7c4]
1409c0dae: 0f 10 95 d4 07 00 00        	movups	xmm2, xmmword ptr [rbp + 0x7d4]
1409c0db5: 0f 10 9d e4 07 00 00        	movups	xmm3, xmmword ptr [rbp + 0x7e4]
1409c0dbc: 0f 29 9d e0 02 00 00        	movaps	xmmword ptr [rbp + 0x2e0], xmm3
1409c0dc3: 0f 29 95 d0 02 00 00        	movaps	xmmword ptr [rbp + 0x2d0], xmm2
1409c0dca: 0f 29 8d c0 02 00 00        	movaps	xmmword ptr [rbp + 0x2c0], xmm1
1409c0dd1: 0f 29 85 b0 02 00 00        	movaps	xmmword ptr [rbp + 0x2b0], xmm0
1409c0dd8: 0f b6 85 fc 07 00 00        	movzx	eax, byte ptr [rbp + 0x7fc]
1409c0ddf: 89 85 8c 0d 00 00           	mov	dword ptr [rbp + 0xd8c], eax
1409c0de5: 8b 85 d0 02 00 00           	mov	eax, dword ptr [rbp + 0x2d0]
1409c0deb: 89 85 50 01 00 00           	mov	dword ptr [rbp + 0x150], eax
1409c0df1: 0f 29 8d 40 01 00 00        	movaps	xmmword ptr [rbp + 0x140], xmm1
1409c0df8: 0f 29 85 30 01 00 00        	movaps	xmmword ptr [rbp + 0x130], xmm0
1409c0dff: 0f 10 85 e4 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2e4]
1409c0e06: 0f 29 85 10 01 00 00        	movaps	xmmword ptr [rbp + 0x110], xmm0
1409c0e0d: 0f 10 85 d4 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2d4]
1409c0e14: 0f 29 85 00 01 00 00        	movaps	xmmword ptr [rbp + 0x100], xmm0
1409c0e1b: 8b 85 f4 02 00 00           	mov	eax, dword ptr [rbp + 0x2f4]
1409c0e21: 89 85 20 01 00 00           	mov	dword ptr [rbp + 0x120], eax
1409c0e27: 49 8d 4f 04                 	lea	rcx, [r15 + 0x4]
1409c0e2b: e8 60 49 21 00              	call	0x140bd5790 <.text+0xbd4790>
1409c0e30: c7 85 b0 07 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x7b0], 0x0
1409c0e3a: 4c 89 b5 b8 0b 00 00        	mov	qword ptr [rbp + 0xbb8], r14
1409c0e41: 4c 89 b5 b8 07 00 00        	mov	qword ptr [rbp + 0x7b8], r14
1409c0e48: c7 85 d0 07 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x7d0], 0x1
1409c0e52: 0f 28 05 d7 4a f6 01        	movaps	xmm0, xmmword ptr [rip + 0x1f64ad7] # 0x142925930
1409c0e59: 0f 11 85 d4 07 00 00        	movups	xmmword ptr [rbp + 0x7d4], xmm0
1409c0e60: 48 8d 8d 0c 02 00 00        	lea	rcx, [rbp + 0x20c]
1409c0e67: 48 89 8d e8 07 00 00        	mov	qword ptr [rbp + 0x7e8], rcx
1409c0e6e: c7 85 f0 07 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x7f0], 0x1
1409c0e78: 0f 57 c0                    	xorps	xmm0, xmm0
1409c0e7b: 0f 11 85 f4 07 00 00        	movups	xmmword ptr [rbp + 0x7f4], xmm0
1409c0e82: 48 8d 8d a0 01 00 00        	lea	rcx, [rbp + 0x1a0]
1409c0e89: 48 89 8d 08 08 00 00        	mov	qword ptr [rbp + 0x808], rcx
1409c0e90: c7 85 10 08 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x810], 0x2
1409c0e9a: 48 8d 8d 7c 01 00 00        	lea	rcx, [rbp + 0x17c]
1409c0ea1: 48 89 8d 18 08 00 00        	mov	qword ptr [rbp + 0x818], rcx
1409c0ea8: c7 85 30 08 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x830], 0x1
1409c0eb2: 0f 11 85 34 08 00 00        	movups	xmmword ptr [rbp + 0x834], xmm0
1409c0eb9: 48 8d 8d 30 01 00 00        	lea	rcx, [rbp + 0x130]
1409c0ec0: 48 89 8d 48 08 00 00        	mov	qword ptr [rbp + 0x848], rcx
1409c0ec7: 48 c7 85 e0 0b 00 00 00 00 00 00    	mov	qword ptr [rbp + 0xbe0], 0x0
1409c0ed2: 4c 89 bd b0 0b 00 00        	mov	qword ptr [rbp + 0xbb0], r15
1409c0ed9: 4c 89 bd e8 0b 00 00        	mov	qword ptr [rbp + 0xbe8], r15
1409c0ee0: 48 c7 85 f0 0b 00 00 00 00 00 00    	mov	qword ptr [rbp + 0xbf0], 0x0
1409c0eeb: 4c 89 a5 80 0d 00 00        	mov	qword ptr [rbp + 0xd80], r12
1409c0ef2: 4c 89 64 24 40              	mov	qword ptr [rsp + 0x40], r12
1409c0ef7: 4c 89 6c 24 38              	mov	qword ptr [rsp + 0x38], r13
1409c0efc: 48 89 44 24 30              	mov	qword ptr [rsp + 0x30], rax
1409c0f01: 48 89 5c 24 28              	mov	qword ptr [rsp + 0x28], rbx
1409c0f06: 48 c7 44 24 20 05 00 00 00  	mov	qword ptr [rsp + 0x20], 0x5
1409c0f0f: 48 8d 15 72 ee 0f 02        	lea	rdx, [rip + 0x20fee72]  # 0x142abfd88
1409c0f16: 48 8d 8d b0 02 00 00        	lea	rcx, [rbp + 0x2b0]
1409c0f1d: 4c 8d a5 b0 07 00 00        	lea	r12, [rbp + 0x7b0]
1409c0f24: 41 b8 09 00 00 00           	mov	r8d, 0x9
1409c0f2a: 4d 89 e1                    	mov	r9, r12
1409c0f2d: e8 3e 6a ea ff              	call	0x140867970 <.text+0x866970>
1409c0f32: 4c 89 ad 90 0d 00 00        	mov	qword ptr [rbp + 0xd90], r13
1409c0f39: 49 8d 4d 38                 	lea	rcx, [r13 + 0x38]
1409c0f3d: c6 85 9f 0d 00 00 01        	mov	byte ptr [rbp + 0xd9f], 0x1
1409c0f44: 48 ba 29 f7 80 5b 19 62 c3 4b       	movabs	rdx, 0x4bc362195b80f729
1409c0f4e: 49 b8 fb ce cb 80 ce 98 27 98       	movabs	r8, -0x67d867317f343105
1409c0f58: e8 93 c7 71 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1409c0f5d: 48 85 c0                    	test	rax, rax
1409c0f60: 48 8b 8d 90 0d 00 00        	mov	rcx, qword ptr [rbp + 0xd90]
1409c0f67: 0f 84 54 1b 00 00           	je	0x1409c2ac1 <.text+0x9c1ac1>
1409c0f6d: 48 39 91 18 02 00 00        	cmp	qword ptr [rcx + 0x218], rdx
1409c0f74: 0f 86 47 1b 00 00           	jbe	0x1409c2ac1 <.text+0x9c1ac1>
1409c0f7a: 48 8b 81 10 02 00 00        	mov	rax, qword ptr [rcx + 0x210]
1409c0f81: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1409c0f85: 48 85 c0                    	test	rax, rax
1409c0f88: 0f 84 33 1b 00 00           	je	0x1409c2ac1 <.text+0x9c1ac1>
1409c0f8e: 48 8b 89 e0 01 00 00        	mov	rcx, qword ptr [rcx + 0x1e0]
1409c0f95: 48 f7 d0                    	not	rax
1409c0f98: 48 8d 14 40                 	lea	rdx, [rax + 2*rax]
1409c0f9c: 48 c1 e2 05                 	shl	rdx, 0x5
1409c0fa0: 48 83 7c 11 20 00           	cmp	qword ptr [rcx + rdx + 0x20], 0x0
1409c0fa6: 0f 84 15 1b 00 00           	je	0x1409c2ac1 <.text+0x9c1ac1>
1409c0fac: 48 8b 85 38 0e 00 00        	mov	rax, qword ptr [rbp + 0xe38]
1409c0fb3: 48 01 d1                    	add	rcx, rdx
1409c0fb6: 48 8b 49 10                 	mov	rcx, qword ptr [rcx + 0x10]
1409c0fba: 48 8b 11                    	mov	rdx, qword ptr [rcx]
1409c0fbd: 48 85 d2                    	test	rdx, rdx
1409c0fc0: 78 0a                       	js	0x1409c0fcc <.text+0x9bffcc>
1409c0fc2: 0f 57 c0                    	xorps	xmm0, xmm0
1409c0fc5: f3 48 0f 2a c2              	cvtsi2ss	xmm0, rdx
1409c0fca: eb 18                       	jmp	0x1409c0fe4 <.text+0x9bffe4>
1409c0fcc: 49 89 d0                    	mov	r8, rdx
1409c0fcf: 49 d1 e8                    	shr	r8
1409c0fd2: 83 e2 01                    	and	edx, 0x1
1409c0fd5: 4c 09 c2                    	or	rdx, r8
1409c0fd8: 0f 57 c0                    	xorps	xmm0, xmm0
1409c0fdb: f3 48 0f 2a c2              	cvtsi2ss	xmm0, rdx
1409c0fe0: f3 0f 58 c0                 	addss	xmm0, xmm0
1409c0fe4: f3 0f 2a 49 08              	cvtsi2ss	xmm1, dword ptr [rcx + 0x8]
1409c0fe9: f3 0f 5e 0d 4f 66 fe 01     	divss	xmm1, dword ptr [rip + 0x1fe664f] # 0x1429a7640
1409c0ff1: f3 0f 58 c8                 	addss	xmm1, xmm0
1409c0ff5: f3 0f 11 8d 80 02 00 00     	movss	dword ptr [rbp + 0x280], xmm1
1409c0ffd: 48 8b b8 70 01 00 00        	mov	rdi, qword ptr [rax + 0x170]
1409c1004: 48 8b 80 78 01 00 00        	mov	rax, qword ptr [rax + 0x178]
1409c100b: 48 c1 e0 07                 	shl	rax, 0x7
1409c100f: 48 01 f8                    	add	rax, rdi
1409c1012: 48 89 85 a0 0b 00 00        	mov	qword ptr [rbp + 0xba0], rax
1409c1019: 31 c0                       	xor	eax, eax
1409c101b: 80 bd 20 0e 00 00 00        	cmp	byte ptr [rbp + 0xe20], 0x0
1409c1022: 8b 8d 8c 0d 00 00           	mov	ecx, dword ptr [rbp + 0xd8c]
1409c1028: 0f 44 c8                    	cmove	ecx, eax
1409c102b: 89 8d 8c 0d 00 00           	mov	dword ptr [rbp + 0xd8c], ecx
1409c1031: c6 85 84 02 00 00 00        	mov	byte ptr [rbp + 0x284], 0x0
1409c1038: 4c 8d b5 b8 06 00 00        	lea	r14, [rbp + 0x6b8]
1409c103f: 48 3b bd a0 0b 00 00        	cmp	rdi, qword ptr [rbp + 0xba0]
1409c1046: 0f 84 d5 03 00 00           	je	0x1409c1421 <.text+0x9c0421>
1409c104c: 0f 1f 40 00                 	nop	dword ptr [rax]
1409c1050: 4c 8b 7f 50                 	mov	r15, qword ptr [rdi + 0x50]
1409c1054: 4c 8b 6f 58                 	mov	r13, qword ptr [rdi + 0x58]
1409c1058: c6 85 9f 0d 00 00 01        	mov	byte ptr [rbp + 0xd9f], 0x1
1409c105f: e8 ec 56 22 00              	call	0x140be6750 <.text+0xbe5750>
1409c1064: 89 c6                       	mov	esi, eax
1409c1066: c6 85 9f 0d 00 00 01        	mov	byte ptr [rbp + 0xd9f], 0x1
1409c106d: 31 c9                       	xor	ecx, ecx
1409c106f: e8 ac 8a 71 ff              	call	0x1400d9b20 <.text+0xd8b20>
1409c1074: 48 85 c0                    	test	rax, rax
1409c1077: 0f 84 2c 1a 00 00           	je	0x1409c2aa9 <.text+0x9c1aa9>
1409c107d: 89 b5 60 07 00 00           	mov	dword ptr [rbp + 0x760], esi
1409c1083: 66 c7 85 68 07 00 00 00 00  	mov	word ptr [rbp + 0x768], 0x0
1409c108c: c7 85 64 07 00 00 00 00 80 3f       	mov	dword ptr [rbp + 0x764], 0x3f800000
1409c1096: 48 c7 85 b8 06 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x6b8], 0x1
1409c10a1: 4c 89 bd c0 06 00 00        	mov	qword ptr [rbp + 0x6c0], r15
1409c10a8: 4c 89 ad 58 07 00 00        	mov	qword ptr [rbp + 0x758], r13
1409c10af: 48 c7 85 c8 06 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x6c8], 0x0
1409c10ba: 48 c7 85 d0 06 00 00 04 00 00 00    	mov	qword ptr [rbp + 0x6d0], 0x4
1409c10c5: 48 c7 85 d8 06 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x6d8], 0x0
1409c10d0: 0f 10 00                    	movups	xmm0, xmmword ptr [rax]
1409c10d3: 48 ff 00                    	inc	qword ptr [rax]
1409c10d6: 0f 10 0d 53 56 0d 02        	movups	xmm1, xmmword ptr [rip + 0x20d5653] # 0x142a96730
1409c10dd: 48 8d 85 28 07 00 00        	lea	rax, [rbp + 0x728]
1409c10e4: 0f 11 48 10                 	movups	xmmword ptr [rax + 0x10], xmm1
1409c10e8: 0f 10 0d 31 56 0d 02        	movups	xmm1, xmmword ptr [rip + 0x20d5631] # 0x142a96720
1409c10ef: 0f 11 08                    	movups	xmmword ptr [rax], xmm1
1409c10f2: 0f 11 85 48 07 00 00        	movups	xmmword ptr [rbp + 0x748], xmm0
1409c10f9: 48 b8 03 00 00 00 01 00 00 00       	movabs	rax, 0x100000003
1409c1103: 48 89 85 f8 06 00 00        	mov	qword ptr [rbp + 0x6f8], rax
1409c110a: 48 c7 85 e0 06 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x6e0], 0x0
1409c1115: 48 c7 85 e8 06 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x6e8], 0x1
1409c1120: 48 c7 85 f0 06 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x6f0], 0x0
1409c112b: c6 85 9f 0d 00 00 01        	mov	byte ptr [rbp + 0xd9f], 0x1
1409c1132: 4c 89 e1                    	mov	rcx, r12
1409c1135: 4c 89 f2                    	mov	rdx, r14
1409c1138: 4c 8d 85 b0 02 00 00        	lea	r8, [rbp + 0x2b0]
1409c113f: 45 31 c9                    	xor	r9d, r9d
1409c1142: e8 99 4a f4 ff              	call	0x140905be0 <.text+0x904be0>
1409c1147: c6 85 9f 0d 00 00 01        	mov	byte ptr [rbp + 0xd9f], 0x1
1409c114e: 48 89 d9                    	mov	rcx, rbx
1409c1151: 4c 89 e2                    	mov	rdx, r12
1409c1154: 4c 8d 85 80 02 00 00        	lea	r8, [rbp + 0x280]
1409c115b: 49 89 f9                    	mov	r9, rdi
1409c115e: e8 fd a2 f4 ff              	call	0x14090b460 <.text+0x90a460>
1409c1163: 80 bd be 0c 00 00 08        	cmp	byte ptr [rbp + 0xcbe], 0x8
1409c116a: 74 28                       	je	0x1409c1194 <.text+0x9c0194>
1409c116c: 48 8b 95 c8 0c 00 00        	mov	rdx, qword ptr [rbp + 0xcc8]
1409c1173: b8 e8 00 00 00              	mov	eax, 0xe8
1409c1178: 48 01 c2                    	add	rdx, rax
1409c117b: 48 8d 8d 98 0c 00 00        	lea	rcx, [rbp + 0xc98]
1409c1182: 49 b8 4c a5 59 1f 44 98 8a 41       	movabs	r8, 0x418a98441f59a54c
1409c118c: 41 b1 01                    	mov	r9b, 0x1
1409c118f: e8 bc a6 fa ff              	call	0x14096b850 <.text+0x96a850>
1409c1194: 41 b8 f8 00 00 00           	mov	r8d, 0xf8
1409c119a: 4c 89 e1                    	mov	rcx, r12
1409c119d: 48 89 da                    	mov	rdx, rbx
1409c11a0: e8 5b 2a f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c11a5: c6 85 30 02 00 00 11        	mov	byte ptr [rbp + 0x230], 0x11
1409c11ac: c7 85 b8 06 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x6b8], 0x2
1409c11b6: 4c 89 74 24 20              	mov	qword ptr [rsp + 0x20], r14
1409c11bb: 48 8d 8d 30 02 00 00        	lea	rcx, [rbp + 0x230]
1409c11c2: 4c 89 e2                    	mov	rdx, r12
1409c11c5: 49 b8 a4 1a 84 c9 4d 1f 65 11       	movabs	r8, 0x11651f4dc9841aa4
1409c11cf: 41 b1 1d                    	mov	r9b, 0x1d
1409c11d2: e8 79 7a f4 ff              	call	0x140908c50 <.text+0x907c50>
1409c11d7: 41 b8 f8 00 00 00           	mov	r8d, 0xf8
1409c11dd: 4c 89 f1                    	mov	rcx, r14
1409c11e0: 4c 89 e2                    	mov	rdx, r12
1409c11e3: e8 18 2a f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c11e8: 48 8b 85 10 0e 00 00        	mov	rax, qword ptr [rbp + 0xe10]
1409c11ef: 48 89 85 e0 0b 00 00        	mov	qword ptr [rbp + 0xbe0], rax
1409c11f6: c7 85 b0 07 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x7b0], 0x2
1409c1200: 4c 89 64 24 20              	mov	qword ptr [rsp + 0x20], r12
1409c1205: 48 89 d9                    	mov	rcx, rbx
1409c1208: 4c 89 f2                    	mov	rdx, r14
1409c120b: 49 b8 30 82 02 28 3a 3c 33 2a       	movabs	r8, 0x2a333c3a28028230
1409c1215: 41 b1 1d                    	mov	r9b, 0x1d
1409c1218: e8 e3 d5 f2 ff              	call	0x1408ee800 <.text+0x8ed800>
1409c121d: 41 b8 f8 00 00 00           	mov	r8d, 0xf8
1409c1223: 4c 89 e1                    	mov	rcx, r12
1409c1226: 4c 89 f2                    	mov	rdx, r14
1409c1229: e8 d2 29 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c122e: 48 8b 85 18 0e 00 00        	mov	rax, qword ptr [rbp + 0xe18]
1409c1235: 48 89 85 b8 06 00 00        	mov	qword ptr [rbp + 0x6b8], rax
1409c123c: c7 85 e0 0b 00 00 02 00 00 00       	mov	dword ptr [rbp + 0xbe0], 0x2
1409c1246: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
1409c124b: 4c 89 f1                    	mov	rcx, r14
1409c124e: 4c 89 e2                    	mov	rdx, r12
1409c1251: 49 b8 d0 09 6c 82 85 51 17 04       	movabs	r8, 0x4175185826c09d0
1409c125b: 41 b1 1d                    	mov	r9b, 0x1d
1409c125e: e8 9d d5 f2 ff              	call	0x1408ee800 <.text+0x8ed800>
1409c1263: 41 b8 f8 00 00 00           	mov	r8d, 0xf8
1409c1269: 48 89 d9                    	mov	rcx, rbx
1409c126c: 4c 89 e2                    	mov	rdx, r12
1409c126f: e8 8c 29 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c1274: 48 8d 85 00 01 00 00        	lea	rax, [rbp + 0x100]
1409c127b: 48 89 85 b8 06 00 00        	mov	qword ptr [rbp + 0x6b8], rax
1409c1282: c7 85 b0 07 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x7b0], 0x2
1409c128c: 4c 89 64 24 20              	mov	qword ptr [rsp + 0x20], r12
1409c1291: 4c 89 f1                    	mov	rcx, r14
1409c1294: 48 89 da                    	mov	rdx, rbx
1409c1297: 49 b8 1b de 69 84 51 b2 fa fd       	movabs	r8, -0x2054dae7b9621e5
1409c12a1: 41 b1 1d                    	mov	r9b, 0x1d
1409c12a4: e8 a7 e3 fa ff              	call	0x14096f650 <.text+0x96e650>
1409c12a9: 41 b8 f8 00 00 00           	mov	r8d, 0xf8
1409c12af: 48 8d 4d d0                 	lea	rcx, [rbp - 0x30]
1409c12b3: 48 89 da                    	mov	rdx, rbx
1409c12b6: e8 45 29 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c12bb: 80 bd ae 00 00 00 08        	cmp	byte ptr [rbp + 0xae], 0x8
1409c12c2: 74 2c                       	je	0x1409c12f0 <.text+0x9c02f0>
1409c12c4: 48 8b 95 b8 00 00 00        	mov	rdx, qword ptr [rbp + 0xb8]
1409c12cb: b8 e8 00 00 00              	mov	eax, 0xe8
1409c12d0: 48 01 c2                    	add	rdx, rax
1409c12d3: 48 8d 8d 88 00 00 00        	lea	rcx, [rbp + 0x88]
1409c12da: 49 b8 02 d9 ee 60 b7 09 a1 a9       	movabs	r8, -0x565ef6489f1126fe
1409c12e4: 44 8b 8d 8c 0d 00 00        	mov	r9d, dword ptr [rbp + 0xd8c]
1409c12eb: e8 80 a2 fa ff              	call	0x14096b570 <.text+0x96a570>
1409c12f0: 41 b8 f8 00 00 00           	mov	r8d, 0xf8
1409c12f6: 4c 89 e1                    	mov	rcx, r12
1409c12f9: 48 8d 55 d0                 	lea	rdx, [rbp - 0x30]
1409c12fd: e8 fe 28 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c1302: 48 8b 85 30 0e 00 00        	mov	rax, qword ptr [rbp + 0xe30]
1409c1309: 48 89 85 b8 06 00 00        	mov	qword ptr [rbp + 0x6b8], rax
1409c1310: c7 85 e0 0b 00 00 02 00 00 00       	mov	dword ptr [rbp + 0xbe0], 0x2
1409c131a: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
1409c131f: 4c 89 f1                    	mov	rcx, r14
1409c1322: 4c 89 e2                    	mov	rdx, r12
1409c1325: 49 b8 e3 6b 5a 83 cd 4c a7 b8       	movabs	r8, -0x4758b3327ca5941d
1409c132f: 41 b1 1d                    	mov	r9b, 0x1d
1409c1332: e8 c9 d4 f2 ff              	call	0x1408ee800 <.text+0x8ed800>
1409c1337: 41 b8 f8 00 00 00           	mov	r8d, 0xf8
1409c133d: 48 89 d9                    	mov	rcx, rbx
1409c1340: 4c 89 e2                    	mov	rdx, r12
1409c1343: e8 b8 28 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c1348: 48 8b 85 28 0e 00 00        	mov	rax, qword ptr [rbp + 0xe28]
1409c134f: 48 89 85 b8 06 00 00        	mov	qword ptr [rbp + 0x6b8], rax
1409c1356: c7 85 b0 07 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x7b0], 0x2
1409c1360: 4c 89 64 24 20              	mov	qword ptr [rsp + 0x20], r12
1409c1365: 4c 89 f1                    	mov	rcx, r14
1409c1368: 48 89 da                    	mov	rdx, rbx
1409c136b: 49 b8 3c 4e 88 28 8a 8e 20 e9       	movabs	r8, -0x16df7175d777b1c4
1409c1375: 41 b1 1d                    	mov	r9b, 0x1d
1409c1378: e8 83 d4 f2 ff              	call	0x1408ee800 <.text+0x8ed800>
1409c137d: 41 b8 f8 00 00 00           	mov	r8d, 0xf8
1409c1383: 4c 89 e1                    	mov	rcx, r12
1409c1386: 48 89 da                    	mov	rdx, rbx
1409c1389: e8 72 28 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c138e: c6 85 9f 0d 00 00 01        	mov	byte ptr [rbp + 0xd9f], 0x1
1409c1395: 48 89 d9                    	mov	rcx, rbx
1409c1398: 4c 89 e2                    	mov	rdx, r12
1409c139b: 49 b8 29 de e8 b2 ea 15 34 2a       	movabs	r8, 0x2a3415eab2e8de29
1409c13a5: 4c 8d 8d c0 0b 00 00        	lea	r9, [rbp + 0xbc0]
1409c13ac: e8 2f 54 f4 ff              	call	0x1409067e0 <.text+0x9057e0>
1409c13b1: c6 85 9f 0d 00 00 01        	mov	byte ptr [rbp + 0xd9f], 0x1
1409c13b8: 4c 89 f1                    	mov	rcx, r14
1409c13bb: 48 89 da                    	mov	rdx, rbx
1409c13be: 49 b8 52 06 5f b7 35 20 90 fc       	movabs	r8, -0x36fdfca48a0f9ae
1409c13c8: 4c 8d 8d 60 02 00 00        	lea	r9, [rbp + 0x260]
1409c13cf: e8 0c 54 f4 ff              	call	0x1409067e0 <.text+0x9057e0>
1409c13d4: 0f b6 85 63 07 00 00        	movzx	eax, byte ptr [rbp + 0x763]
1409c13db: 24 9f                       	and	al, -0x61
1409c13dd: 0c 40                       	or	al, 0x40
1409c13df: 88 85 63 07 00 00           	mov	byte ptr [rbp + 0x763], al
1409c13e5: 48 8b 77 70                 	mov	rsi, qword ptr [rdi + 0x70]
1409c13e9: 41 b8 f8 00 00 00           	mov	r8d, 0xf8
1409c13ef: 4c 89 e1                    	mov	rcx, r12
1409c13f2: 4c 89 f2                    	mov	rdx, r14
1409c13f5: e8 06 28 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c13fa: c6 85 9f 0d 00 00 01        	mov	byte ptr [rbp + 0xd9f], 0x1
1409c1401: 4c 89 e1                    	mov	rcx, r12
1409c1404: 48 89 fa                    	mov	rdx, rdi
1409c1407: 4c 8b 85 90 0d 00 00        	mov	r8, qword ptr [rbp + 0xd90]
1409c140e: ff d6                       	call	rsi
1409c1410: 48 83 ef 80                 	sub	rdi, -0x80
1409c1414: 48 3b bd a0 0b 00 00        	cmp	rdi, qword ptr [rbp + 0xba0]
1409c141b: 0f 85 2f fc ff ff           	jne	0x1409c1050 <.text+0x9c0050>
1409c1421: 48 8d b5 b0 07 00 00        	lea	rsi, [rbp + 0x7b0]
1409c1428: 48 8d 95 b0 02 00 00        	lea	rdx, [rbp + 0x2b0]
1409c142f: 41 b8 e8 03 00 00           	mov	r8d, 0x3e8
1409c1435: 48 89 f1                    	mov	rcx, rsi
1409c1438: e8 c3 27 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c143d: c6 85 9f 0d 00 00 00        	mov	byte ptr [rbp + 0xd9f], 0x0
1409c1444: 48 89 f1                    	mov	rcx, rsi
1409c1447: e8 24 6f ea ff              	call	0x140868370 <.text+0x867370>
1409c144c: 48 8b 9d 78 0d 00 00        	mov	rbx, qword ptr [rbp + 0xd78]
1409c1453: 0f 10 03                    	movups	xmm0, xmmword ptr [rbx]
1409c1456: 0f 10 4b 10                 	movups	xmm1, xmmword ptr [rbx + 0x10]
1409c145a: 0f 29 8d c0 07 00 00        	movaps	xmmword ptr [rbp + 0x7c0], xmm1
1409c1461: 0f 29 85 b0 07 00 00        	movaps	xmmword ptr [rbp + 0x7b0], xmm0
1409c1468: 48 8d b5 e0 0b 00 00        	lea	rsi, [rbp + 0xbe0]
1409c146f: 48 8d 95 b0 07 00 00        	lea	rdx, [rbp + 0x7b0]
1409c1476: 48 89 f1                    	mov	rcx, rsi
1409c1479: e8 b2 42 21 00              	call	0x140bd5730 <.text+0xbd4730>
1409c147e: 48 89 f1                    	mov	rcx, rsi
1409c1481: e8 0a 43 21 00              	call	0x140bd5790 <.text+0xbd4790>
1409c1486: 48 8d b5 b0 07 00 00        	lea	rsi, [rbp + 0x7b0]
1409c148d: 48 89 f1                    	mov	rcx, rsi
1409c1490: 48 89 c2                    	mov	rdx, rax
1409c1493: 41 b8 62 00 00 00           	mov	r8d, 0x62
1409c1499: e8 02 3e 21 00              	call	0x140bd52a0 <.text+0xbd42a0>
1409c149e: 48 8d 8d b8 06 00 00        	lea	rcx, [rbp + 0x6b8]
1409c14a5: 48 89 f2                    	mov	rdx, rsi
1409c14a8: e8 63 3f 21 00              	call	0x140bd5410 <.text+0xbd4410>
1409c14ad: 0f 10 85 b8 06 00 00        	movups	xmm0, xmmword ptr [rbp + 0x6b8]
1409c14b4: 0f 10 8d c8 06 00 00        	movups	xmm1, xmmword ptr [rbp + 0x6c8]
1409c14bb: 0f 29 8d c0 02 00 00        	movaps	xmmword ptr [rbp + 0x2c0], xmm1
1409c14c2: 0f 29 85 b0 02 00 00        	movaps	xmmword ptr [rbp + 0x2b0], xmm0
1409c14c9: 48 8d 8d c0 07 00 00        	lea	rcx, [rbp + 0x7c0]
1409c14d0: 48 8d 95 b0 02 00 00        	lea	rdx, [rbp + 0x2b0]
1409c14d7: e8 04 58 21 00              	call	0x140bd6ce0 <.text+0xbd5ce0>
1409c14dc: 48 8d 05 ae e8 0f 02        	lea	rax, [rip + 0x20fe8ae]  # 0x142abfd91
1409c14e3: 48 89 85 b0 07 00 00        	mov	qword ptr [rbp + 0x7b0], rax
1409c14ea: 48 c7 85 b8 07 00 00 17 00 00 00    	mov	qword ptr [rbp + 0x7b8], 0x17
1409c14f5: 48 8d 95 b0 07 00 00        	lea	rdx, [rbp + 0x7b0]
1409c14fc: 48 8b bd 80 0d 00 00        	mov	rdi, qword ptr [rbp + 0xd80]
1409c1503: 48 89 f9                    	mov	rcx, rdi
1409c1506: e8 c5 7f 22 00              	call	0x140be94d0 <.text+0xbe84d0>
1409c150b: 0f 10 85 b8 06 00 00        	movups	xmm0, xmmword ptr [rbp + 0x6b8]
1409c1512: 0f 10 8d c8 06 00 00        	movups	xmm1, xmmword ptr [rbp + 0x6c8]
1409c1519: 0f 11 85 ec 01 00 00        	movups	xmmword ptr [rbp + 0x1ec], xmm0
1409c1520: 0f 11 8d fc 01 00 00        	movups	xmmword ptr [rbp + 0x1fc], xmm1
1409c1527: 89 85 e8 01 00 00           	mov	dword ptr [rbp + 0x1e8], eax
1409c152d: 0f 10 03                    	movups	xmm0, xmmword ptr [rbx]
1409c1530: 0f 10 4b 10                 	movups	xmm1, xmmword ptr [rbx + 0x10]
1409c1534: 0f 29 8d c0 07 00 00        	movaps	xmmword ptr [rbp + 0x7c0], xmm1
1409c153b: 0f 29 85 b0 07 00 00        	movaps	xmmword ptr [rbp + 0x7b0], xmm0
1409c1542: 48 8d b5 e0 0b 00 00        	lea	rsi, [rbp + 0xbe0]
1409c1549: 48 8d 95 b0 07 00 00        	lea	rdx, [rbp + 0x7b0]
1409c1550: 48 89 f1                    	mov	rcx, rsi
1409c1553: e8 d8 41 21 00              	call	0x140bd5730 <.text+0xbd4730>
1409c1558: 48 89 f1                    	mov	rcx, rsi
1409c155b: e8 30 42 21 00              	call	0x140bd5790 <.text+0xbd4790>
1409c1560: 48 8d 8d b8 06 00 00        	lea	rcx, [rbp + 0x6b8]
1409c1567: 48 89 c2                    	mov	rdx, rax
1409c156a: 41 b8 53 00 00 00           	mov	r8d, 0x53
1409c1570: e8 2b 3d 21 00              	call	0x140bd52a0 <.text+0xbd42a0>
1409c1575: 0f 10 85 b8 06 00 00        	movups	xmm0, xmmword ptr [rbp + 0x6b8]
1409c157c: 0f 10 8d c8 06 00 00        	movups	xmm1, xmmword ptr [rbp + 0x6c8]
1409c1583: 0f 29 8d c0 02 00 00        	movaps	xmmword ptr [rbp + 0x2c0], xmm1
1409c158a: 0f 29 85 b0 02 00 00        	movaps	xmmword ptr [rbp + 0x2b0], xmm0
1409c1591: 48 8d 8d c0 07 00 00        	lea	rcx, [rbp + 0x7c0]
1409c1598: 48 8d 95 b0 02 00 00        	lea	rdx, [rbp + 0x2b0]
1409c159f: e8 3c 57 21 00              	call	0x140bd6ce0 <.text+0xbd5ce0>
1409c15a4: 48 8d 05 fd e7 0f 02        	lea	rax, [rip + 0x20fe7fd]  # 0x142abfda8
1409c15ab: 48 89 85 b0 07 00 00        	mov	qword ptr [rbp + 0x7b0], rax
1409c15b2: 48 c7 85 b8 07 00 00 0f 00 00 00    	mov	qword ptr [rbp + 0x7b8], 0xf
1409c15bd: 48 8d 95 b0 07 00 00        	lea	rdx, [rbp + 0x7b0]
1409c15c4: 48 89 f9                    	mov	rcx, rdi
1409c15c7: e8 04 7f 22 00              	call	0x140be94d0 <.text+0xbe84d0>
1409c15cc: 0f 10 85 b8 06 00 00        	movups	xmm0, xmmword ptr [rbp + 0x6b8]
1409c15d3: 0f 10 8d c8 06 00 00        	movups	xmm1, xmmword ptr [rbp + 0x6c8]
1409c15da: 0f 11 85 c8 01 00 00        	movups	xmmword ptr [rbp + 0x1c8], xmm0
1409c15e1: 0f 11 8d d8 01 00 00        	movups	xmmword ptr [rbp + 0x1d8], xmm1
1409c15e8: 89 85 c4 01 00 00           	mov	dword ptr [rbp + 0x1c4], eax
1409c15ee: 4c 8d a5 c4 01 00 00        	lea	r12, [rbp + 0x1c4]
1409c15f5: 48 89 f9                    	mov	rcx, rdi
1409c15f8: 4c 89 e2                    	mov	rdx, r12
1409c15fb: e8 90 c8 01 00              	call	0x1409dde90 <.text+0x9dce90>
1409c1600: 48 89 7c 24 30              	mov	qword ptr [rsp + 0x30], rdi
1409c1605: 4c 8b bd 90 0d 00 00        	mov	r15, qword ptr [rbp + 0xd90]
1409c160c: 4c 89 7c 24 28              	mov	qword ptr [rsp + 0x28], r15
1409c1611: 48 c7 44 24 20 1d 00 00 00  	mov	qword ptr [rsp + 0x20], 0x1d
1409c161a: 48 8d 15 96 e7 0f 02        	lea	rdx, [rip + 0x20fe796]  # 0x142abfdb7
1409c1621: 4c 8d 0d 9d e7 0f 02        	lea	r9, [rip + 0x20fe79d]   # 0x142abfdc5
1409c1628: 48 8d b5 b0 07 00 00        	lea	rsi, [rbp + 0x7b0]
1409c162f: 41 b8 0e 00 00 00           	mov	r8d, 0xe
1409c1635: 48 89 f1                    	mov	rcx, rsi
1409c1638: e8 63 1c c4 00              	call	0x1416032a0 <.text+0x16022a0>
1409c163d: 48 8d bd b0 02 00 00        	lea	rdi, [rbp + 0x2b0]
1409c1644: 49 b8 29 de e8 b2 ea 15 34 2a       	movabs	r8, 0x2a3415eab2e8de29
1409c164e: 4c 8d 8d c0 0b 00 00        	lea	r9, [rbp + 0xbc0]
1409c1655: 48 89 f9                    	mov	rcx, rdi
1409c1658: 48 89 f2                    	mov	rdx, rsi
1409c165b: e8 60 c4 fa ff              	call	0x14096dac0 <.text+0x96cac0>
1409c1660: 49 b8 52 06 5f b7 35 20 90 fc       	movabs	r8, -0x36fdfca48a0f9ae
1409c166a: 4c 8d 8d 60 02 00 00        	lea	r9, [rbp + 0x260]
1409c1671: 48 89 f1                    	mov	rcx, rsi
1409c1674: 48 89 fa                    	mov	rdx, rdi
1409c1677: e8 44 c4 fa ff              	call	0x14096dac0 <.text+0x96cac0>
1409c167c: 48 8b 85 10 0e 00 00        	mov	rax, qword ptr [rbp + 0xe10]
1409c1683: 48 89 85 e0 0b 00 00        	mov	qword ptr [rbp + 0xbe0], rax
1409c168a: c7 85 b0 02 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x2b0], 0x2
1409c1694: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
1409c1699: 48 8d 8d e0 0b 00 00        	lea	rcx, [rbp + 0xbe0]
1409c16a0: 49 b8 30 82 02 28 3a 3c 33 2a       	movabs	r8, 0x2a333c3a28028230
1409c16aa: 48 89 f2                    	mov	rdx, rsi
1409c16ad: 41 b1 19                    	mov	r9b, 0x19
1409c16b0: e8 8b cb f2 ff              	call	0x1408ee240 <.text+0x8ed240>
1409c16b5: 48 8d b5 b0 02 00 00        	lea	rsi, [rbp + 0x2b0]
1409c16bc: 48 8d bd b0 07 00 00        	lea	rdi, [rbp + 0x7b0]
1409c16c3: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c16c9: 48 89 f1                    	mov	rcx, rsi
1409c16cc: 48 89 fa                    	mov	rdx, rdi
1409c16cf: e8 2c 25 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c16d4: 48 8b 85 18 0e 00 00        	mov	rax, qword ptr [rbp + 0xe18]
1409c16db: 48 89 85 e0 0b 00 00        	mov	qword ptr [rbp + 0xbe0], rax
1409c16e2: c7 85 b0 07 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x7b0], 0x2
1409c16ec: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
1409c16f1: 48 8d 8d e0 0b 00 00        	lea	rcx, [rbp + 0xbe0]
1409c16f8: 49 b8 d0 09 6c 82 85 51 17 04       	movabs	r8, 0x4175185826c09d0
1409c1702: 48 89 f2                    	mov	rdx, rsi
1409c1705: 41 b1 19                    	mov	r9b, 0x19
1409c1708: e8 33 cb f2 ff              	call	0x1408ee240 <.text+0x8ed240>
1409c170d: 48 8d 8d b0 07 00 00        	lea	rcx, [rbp + 0x7b0]
1409c1714: 48 8d 95 b0 02 00 00        	lea	rdx, [rbp + 0x2b0]
1409c171b: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c1721: e8 da 24 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c1726: 0f b6 b5 20 0e 00 00        	movzx	esi, byte ptr [rbp + 0xe20]
1409c172d: 80 bd e6 08 00 00 08        	cmp	byte ptr [rbp + 0x8e6], 0x8
1409c1734: 74 25                       	je	0x1409c175b <.text+0x9c075b>
1409c1736: 48 8d 8d c0 08 00 00        	lea	rcx, [rbp + 0x8c0]
1409c173d: ba e8 00 00 00              	mov	edx, 0xe8
1409c1742: 48 03 95 e8 08 00 00        	add	rdx, qword ptr [rbp + 0x8e8]
1409c1749: 49 b8 02 d9 ee 60 b7 09 a1 a9       	movabs	r8, -0x565ef6489f1126fe
1409c1753: 41 89 f1                    	mov	r9d, esi
1409c1756: e8 15 9e fa ff              	call	0x14096b570 <.text+0x96a570>
1409c175b: 48 8d bd b0 02 00 00        	lea	rdi, [rbp + 0x2b0]
1409c1762: 48 8d 9d b0 07 00 00        	lea	rbx, [rbp + 0x7b0]
1409c1769: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c176f: 48 89 f9                    	mov	rcx, rdi
1409c1772: 48 89 da                    	mov	rdx, rbx
1409c1775: e8 86 24 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c177a: 48 8b 85 30 0e 00 00        	mov	rax, qword ptr [rbp + 0xe30]
1409c1781: 48 89 85 e0 0b 00 00        	mov	qword ptr [rbp + 0xbe0], rax
1409c1788: c7 85 b0 07 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x7b0], 0x2
1409c1792: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
1409c1797: 48 8d 8d e0 0b 00 00        	lea	rcx, [rbp + 0xbe0]
1409c179e: 49 b8 e3 6b 5a 83 cd 4c a7 b8       	movabs	r8, -0x4758b3327ca5941d
1409c17a8: 48 89 fa                    	mov	rdx, rdi
1409c17ab: 41 b1 19                    	mov	r9b, 0x19
1409c17ae: e8 8d ca f2 ff              	call	0x1408ee240 <.text+0x8ed240>
1409c17b3: 48 8d bd b0 07 00 00        	lea	rdi, [rbp + 0x7b0]
1409c17ba: 48 8d 9d b0 02 00 00        	lea	rbx, [rbp + 0x2b0]
1409c17c1: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c17c7: 48 89 f9                    	mov	rcx, rdi
1409c17ca: 48 89 da                    	mov	rdx, rbx
1409c17cd: e8 2e 24 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c17d2: 48 b8 65 18 a0 91 ab 39 9f 57       	movabs	rax, 0x579f39ab91a01865
1409c17dc: 48 89 85 e0 0b 00 00        	mov	qword ptr [rbp + 0xbe0], rax
1409c17e3: c7 85 b0 02 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x2b0], 0x2
1409c17ed: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
1409c17f2: 48 8d 8d e0 0b 00 00        	lea	rcx, [rbp + 0xbe0]
1409c17f9: 49 b8 8c 97 66 69 03 f5 5b 81       	movabs	r8, -0x7ea40afc96996874
1409c1803: 48 89 fa                    	mov	rdx, rdi
1409c1806: 41 b1 19                    	mov	r9b, 0x19
1409c1809: e8 c2 3e f4 ff              	call	0x1409056d0 <.text+0x9046d0>
1409c180e: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c1814: 48 89 d9                    	mov	rcx, rbx
1409c1817: 48 89 fa                    	mov	rdx, rdi
1409c181a: e8 e1 23 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c181f: 48 8d 85 0c 02 00 00        	lea	rax, [rbp + 0x20c]
1409c1826: 48 89 85 b8 06 00 00        	mov	qword ptr [rbp + 0x6b8], rax
1409c182d: c7 85 b0 07 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x7b0], 0x2
1409c1837: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
1409c183c: 48 8d 8d b8 06 00 00        	lea	rcx, [rbp + 0x6b8]
1409c1843: 49 b8 83 b8 5a e4 eb 2b 90 53       	movabs	r8, 0x53902bebe45ab883
1409c184d: 48 89 da                    	mov	rdx, rbx
1409c1850: 41 b1 19                    	mov	r9b, 0x19
1409c1853: e8 e8 c9 f2 ff              	call	0x1408ee240 <.text+0x8ed240>
1409c1858: 48 8d 9d e0 0b 00 00        	lea	rbx, [rbp + 0xbe0]
1409c185f: 48 8d 95 b0 02 00 00        	lea	rdx, [rbp + 0x2b0]
1409c1866: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c186c: 48 89 d9                    	mov	rcx, rbx
1409c186f: e8 8c 23 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c1874: c6 85 b0 02 00 00 09        	mov	byte ptr [rbp + 0x2b0], 0x9
1409c187b: c7 85 b0 07 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x7b0], 0x2
1409c1885: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
1409c188a: 48 8d bd b0 02 00 00        	lea	rdi, [rbp + 0x2b0]
1409c1891: 49 b8 f2 c5 70 04 5e 8b b0 61       	movabs	r8, 0x61b08b5e0470c5f2
1409c189b: 48 89 f9                    	mov	rcx, rdi
1409c189e: 48 89 da                    	mov	rdx, rbx
1409c18a1: 41 b1 19                    	mov	r9b, 0x19
1409c18a4: e8 b7 39 f4 ff              	call	0x140905260 <.text+0x904260>
1409c18a9: 48 8d 9d b0 07 00 00        	lea	rbx, [rbp + 0x7b0]
1409c18b0: 48 8d 95 e0 0b 00 00        	lea	rdx, [rbp + 0xbe0]
1409c18b7: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c18bd: 48 89 d9                    	mov	rcx, rbx
1409c18c0: e8 3b 23 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c18c5: c6 85 b8 06 00 00 0a        	mov	byte ptr [rbp + 0x6b8], 0xa
1409c18cc: c7 85 b0 02 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x2b0], 0x2
1409c18d6: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
1409c18db: 48 8d 8d b8 06 00 00        	lea	rcx, [rbp + 0x6b8]
1409c18e2: 49 b8 57 f6 e8 da 4c 1e 7d 29       	movabs	r8, 0x297d1e4cdae8f657
1409c18ec: 48 89 da                    	mov	rdx, rbx
1409c18ef: 41 b1 19                    	mov	r9b, 0x19
1409c18f2: e8 69 39 f4 ff              	call	0x140905260 <.text+0x904260>
1409c18f7: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c18fd: 48 89 f9                    	mov	rcx, rdi
1409c1900: 48 89 da                    	mov	rdx, rbx
1409c1903: e8 f8 22 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c1908: c6 85 e0 0b 00 00 08        	mov	byte ptr [rbp + 0xbe0], 0x8
1409c190f: c7 85 b0 07 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x7b0], 0x2
1409c1919: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
1409c191e: 48 8d 9d e0 0b 00 00        	lea	rbx, [rbp + 0xbe0]
1409c1925: 49 b8 3f ca 8d ac 13 01 16 77       	movabs	r8, 0x77160113ac8dca3f
1409c192f: 48 89 d9                    	mov	rcx, rbx
1409c1932: 48 89 fa                    	mov	rdx, rdi
1409c1935: 41 b1 19                    	mov	r9b, 0x19
1409c1938: e8 23 39 f4 ff              	call	0x140905260 <.text+0x904260>
1409c193d: 4c 8d b5 b0 07 00 00        	lea	r14, [rbp + 0x7b0]
1409c1944: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c194a: 4c 89 f1                    	mov	rcx, r14
1409c194d: 48 89 fa                    	mov	rdx, rdi
1409c1950: e8 ab 22 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c1955: 48 b8 65 18 a0 91 ab 39 9f 57       	movabs	rax, 0x579f39ab91a01865
1409c195f: 48 89 85 b8 06 00 00        	mov	qword ptr [rbp + 0x6b8], rax
1409c1966: c7 85 e0 0b 00 00 02 00 00 00       	mov	dword ptr [rbp + 0xbe0], 0x2
1409c1970: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
1409c1975: 48 8d 8d b8 06 00 00        	lea	rcx, [rbp + 0x6b8]
1409c197c: 49 b8 8c 97 66 69 03 f5 5b 81       	movabs	r8, -0x7ea40afc96996874
1409c1986: 4c 89 f2                    	mov	rdx, r14
1409c1989: 41 b1 19                    	mov	r9b, 0x19
1409c198c: e8 3f 3d f4 ff              	call	0x1409056d0 <.text+0x9046d0>
1409c1991: 48 8d 95 b0 07 00 00        	lea	rdx, [rbp + 0x7b0]
1409c1998: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c199e: 48 89 d9                    	mov	rcx, rbx
1409c19a1: e8 5a 22 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c19a6: 4c 89 f1                    	mov	rcx, r14
1409c19a9: 48 89 da                    	mov	rdx, rbx
1409c19ac: 4d 89 f8                    	mov	r8, r15
1409c19af: e8 7c a8 c3 00              	call	0x1415fc230 <.text+0x15fb230>
1409c19b4: 4c 89 a5 e0 0b 00 00        	mov	qword ptr [rbp + 0xbe0], r12
1409c19bb: c7 85 b0 02 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x2b0], 0x2
1409c19c5: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
1409c19ca: 48 8d 8d e0 0b 00 00        	lea	rcx, [rbp + 0xbe0]
1409c19d1: 49 b8 d6 6b 7f 6b 27 01 59 2c       	movabs	r8, 0x2c5901276b7f6bd6
1409c19db: 4c 89 f2                    	mov	rdx, r14
1409c19de: 41 b1 32                    	mov	r9b, 0x32
1409c19e1: e8 1a c3 f2 ff              	call	0x1408edd00 <.text+0x8ecd00>
1409c19e6: 48 8d 9d b0 07 00 00        	lea	rbx, [rbp + 0x7b0]
1409c19ed: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c19f3: 48 89 f9                    	mov	rcx, rdi
1409c19f6: 48 89 da                    	mov	rdx, rbx
1409c19f9: e8 02 22 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c19fe: 48 8d 85 e8 01 00 00        	lea	rax, [rbp + 0x1e8]
1409c1a05: 48 89 85 e0 0b 00 00        	mov	qword ptr [rbp + 0xbe0], rax
1409c1a0c: c7 85 b0 07 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x7b0], 0x2
1409c1a16: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
1409c1a1b: 48 8d 8d e0 0b 00 00        	lea	rcx, [rbp + 0xbe0]
1409c1a22: 49 b8 42 b3 aa d1 08 f9 c9 ec       	movabs	r8, -0x133606f72e554cbe
1409c1a2c: 48 89 fa                    	mov	rdx, rdi
1409c1a2f: 41 b1 32                    	mov	r9b, 0x32
1409c1a32: e8 c9 c2 f2 ff              	call	0x1408edd00 <.text+0x8ecd00>
1409c1a37: 48 8d 95 b0 02 00 00        	lea	rdx, [rbp + 0x2b0]
1409c1a3e: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c1a44: 4c 89 f1                    	mov	rcx, r14
1409c1a47: e8 b4 21 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c1a4c: 4c 8d a5 b0 02 00 00        	lea	r12, [rbp + 0x2b0]
1409c1a53: 48 bf 3e e9 39 94 88 2b 51 f0       	movabs	rdi, -0xfaed4776bc616c2
1409c1a5d: 48 8d 9d a0 06 00 00        	lea	rbx, [rbp + 0x6a0]
1409c1a64: 4c 89 e1                    	mov	rcx, r12
1409c1a67: 4c 89 f2                    	mov	rdx, r14
1409c1a6a: 49 89 f8                    	mov	r8, rdi
1409c1a6d: 49 89 d9                    	mov	r9, rbx
1409c1a70: e8 eb c2 fa ff              	call	0x14096dd60 <.text+0x96cd60>
1409c1a75: 4c 8d ad b0 07 00 00        	lea	r13, [rbp + 0x7b0]
1409c1a7c: 49 be ab 53 f2 fd 6c 1d 54 06       	movabs	r14, 0x6541d6cfdf253ab
1409c1a86: 4c 8d bd 60 01 00 00        	lea	r15, [rbp + 0x160]
1409c1a8d: 4c 89 e9                    	mov	rcx, r13
1409c1a90: 4c 89 e2                    	mov	rdx, r12
1409c1a93: 4d 89 f0                    	mov	r8, r14
1409c1a96: 4d 89 f9                    	mov	r9, r15
1409c1a99: e8 c2 c2 fa ff              	call	0x14096dd60 <.text+0x96cd60>
1409c1a9e: 48 8d 95 c0 0b 00 00        	lea	rdx, [rbp + 0xbc0]
1409c1aa5: 4c 89 e9                    	mov	rcx, r13
1409c1aa8: 45 31 c0                    	xor	r8d, r8d
1409c1aab: e8 d0 cd fa ff              	call	0x14096e880 <.text+0x96d880>
1409c1ab0: 48 8b 85 80 0d 00 00        	mov	rax, qword ptr [rbp + 0xd80]
1409c1ab7: 48 89 44 24 30              	mov	qword ptr [rsp + 0x30], rax
1409c1abc: 48 8b 85 90 0d 00 00        	mov	rax, qword ptr [rbp + 0xd90]
1409c1ac3: 48 89 44 24 28              	mov	qword ptr [rsp + 0x28], rax
1409c1ac8: 48 c7 44 24 20 1e 00 00 00  	mov	qword ptr [rsp + 0x20], 0x1e
1409c1ad1: 48 8d 15 0a e3 0f 02        	lea	rdx, [rip + 0x20fe30a]  # 0x142abfde2
1409c1ad8: 4c 8d 0d 11 e3 0f 02        	lea	r9, [rip + 0x20fe311]   # 0x142abfdf0
1409c1adf: 4c 8d a5 b0 07 00 00        	lea	r12, [rbp + 0x7b0]
1409c1ae6: 41 b8 0e 00 00 00           	mov	r8d, 0xe
1409c1aec: 4c 89 e1                    	mov	rcx, r12
1409c1aef: e8 ac 17 c4 00              	call	0x1416032a0 <.text+0x16022a0>
1409c1af4: 4c 8d ad b0 02 00 00        	lea	r13, [rbp + 0x2b0]
1409c1afb: 4c 89 e9                    	mov	rcx, r13
1409c1afe: 4c 89 e2                    	mov	rdx, r12
1409c1b01: 49 89 f8                    	mov	r8, rdi
1409c1b04: 49 89 d9                    	mov	r9, rbx
1409c1b07: e8 b4 bf fa ff              	call	0x14096dac0 <.text+0x96cac0>
1409c1b0c: 48 8d bd b0 07 00 00        	lea	rdi, [rbp + 0x7b0]
1409c1b13: 48 89 f9                    	mov	rcx, rdi
1409c1b16: 4c 89 ea                    	mov	rdx, r13
1409c1b19: 4d 89 f0                    	mov	r8, r14
1409c1b1c: 4d 89 f9                    	mov	r9, r15
1409c1b1f: e8 9c bf fa ff              	call	0x14096dac0 <.text+0x96cac0>
1409c1b24: 48 8b 85 10 0e 00 00        	mov	rax, qword ptr [rbp + 0xe10]
1409c1b2b: 48 89 85 e0 0b 00 00        	mov	qword ptr [rbp + 0xbe0], rax
1409c1b32: c7 85 b0 02 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x2b0], 0x2
1409c1b3c: 4c 89 6c 24 20              	mov	qword ptr [rsp + 0x20], r13
1409c1b41: 48 8d 8d e0 0b 00 00        	lea	rcx, [rbp + 0xbe0]
1409c1b48: 49 b8 30 82 02 28 3a 3c 33 2a       	movabs	r8, 0x2a333c3a28028230
1409c1b52: 48 89 fa                    	mov	rdx, rdi
1409c1b55: 41 b1 19                    	mov	r9b, 0x19
1409c1b58: e8 e3 c6 f2 ff              	call	0x1408ee240 <.text+0x8ed240>
1409c1b5d: 48 8d bd b0 02 00 00        	lea	rdi, [rbp + 0x2b0]
1409c1b64: 48 8d 9d b0 07 00 00        	lea	rbx, [rbp + 0x7b0]
1409c1b6b: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c1b71: 48 89 f9                    	mov	rcx, rdi
1409c1b74: 48 89 da                    	mov	rdx, rbx
1409c1b77: e8 84 20 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c1b7c: 48 8b 85 18 0e 00 00        	mov	rax, qword ptr [rbp + 0xe18]
1409c1b83: 48 89 85 e0 0b 00 00        	mov	qword ptr [rbp + 0xbe0], rax
1409c1b8a: c7 85 b0 07 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x7b0], 0x2
1409c1b94: 48 89 5c 24 20              	mov	qword ptr [rsp + 0x20], rbx
1409c1b99: 48 8d 8d e0 0b 00 00        	lea	rcx, [rbp + 0xbe0]
1409c1ba0: 49 b8 d0 09 6c 82 85 51 17 04       	movabs	r8, 0x4175185826c09d0
1409c1baa: 48 89 fa                    	mov	rdx, rdi
1409c1bad: 41 b1 19                    	mov	r9b, 0x19
1409c1bb0: e8 8b c6 f2 ff              	call	0x1408ee240 <.text+0x8ed240>
1409c1bb5: 4c 8b a5 a8 0b 00 00        	mov	r12, qword ptr [rbp + 0xba8]
1409c1bbc: 4c 8b ad b0 0b 00 00        	mov	r13, qword ptr [rbp + 0xbb0]
1409c1bc3: 4c 8b b5 78 0d 00 00        	mov	r14, qword ptr [rbp + 0xd78]
1409c1bca: 48 8d 8d b0 07 00 00        	lea	rcx, [rbp + 0x7b0]
1409c1bd1: 48 8d 95 b0 02 00 00        	lea	rdx, [rbp + 0x2b0]
1409c1bd8: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c1bde: e8 1d 20 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c1be3: 80 bd e6 08 00 00 08        	cmp	byte ptr [rbp + 0x8e6], 0x8
1409c1bea: 4c 8b bd 80 0d 00 00        	mov	r15, qword ptr [rbp + 0xd80]
1409c1bf1: 74 25                       	je	0x1409c1c18 <.text+0x9c0c18>
1409c1bf3: 48 8d 8d c0 08 00 00        	lea	rcx, [rbp + 0x8c0]
1409c1bfa: ba e8 00 00 00              	mov	edx, 0xe8
1409c1bff: 48 03 95 e8 08 00 00        	add	rdx, qword ptr [rbp + 0x8e8]
1409c1c06: 49 b8 02 d9 ee 60 b7 09 a1 a9       	movabs	r8, -0x565ef6489f1126fe
1409c1c10: 41 89 f1                    	mov	r9d, esi
1409c1c13: e8 58 99 fa ff              	call	0x14096b570 <.text+0x96a570>
1409c1c18: 48 8d b5 b0 02 00 00        	lea	rsi, [rbp + 0x2b0]
1409c1c1f: 48 8d bd b0 07 00 00        	lea	rdi, [rbp + 0x7b0]
1409c1c26: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c1c2c: 48 89 f1                    	mov	rcx, rsi
1409c1c2f: 48 89 fa                    	mov	rdx, rdi
1409c1c32: e8 c9 1f f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c1c37: 48 8b 85 30 0e 00 00        	mov	rax, qword ptr [rbp + 0xe30]
1409c1c3e: 48 89 85 e0 0b 00 00        	mov	qword ptr [rbp + 0xbe0], rax
1409c1c45: c7 85 b0 07 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x7b0], 0x2
1409c1c4f: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
1409c1c54: 48 8d 8d e0 0b 00 00        	lea	rcx, [rbp + 0xbe0]
1409c1c5b: 49 b8 e3 6b 5a 83 cd 4c a7 b8       	movabs	r8, -0x4758b3327ca5941d
1409c1c65: 48 89 f2                    	mov	rdx, rsi
1409c1c68: 41 b1 19                    	mov	r9b, 0x19
1409c1c6b: e8 d0 c5 f2 ff              	call	0x1408ee240 <.text+0x8ed240>
1409c1c70: 48 8d b5 b0 07 00 00        	lea	rsi, [rbp + 0x7b0]
1409c1c77: 48 8d bd b0 02 00 00        	lea	rdi, [rbp + 0x2b0]
1409c1c7e: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c1c84: 48 89 f1                    	mov	rcx, rsi
1409c1c87: 48 89 fa                    	mov	rdx, rdi
1409c1c8a: e8 71 1f f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c1c8f: 48 b8 65 18 a0 91 ab 39 9f 57       	movabs	rax, 0x579f39ab91a01865
1409c1c99: 48 89 85 e0 0b 00 00        	mov	qword ptr [rbp + 0xbe0], rax
1409c1ca0: c7 85 b0 02 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x2b0], 0x2
1409c1caa: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
1409c1caf: 48 8d 8d e0 0b 00 00        	lea	rcx, [rbp + 0xbe0]
1409c1cb6: 49 b8 8c 97 66 69 03 f5 5b 81       	movabs	r8, -0x7ea40afc96996874
1409c1cc0: 48 89 f2                    	mov	rdx, rsi
1409c1cc3: 41 b1 19                    	mov	r9b, 0x19
1409c1cc6: e8 05 3a f4 ff              	call	0x1409056d0 <.text+0x9046d0>
1409c1ccb: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c1cd1: 48 89 f9                    	mov	rcx, rdi
1409c1cd4: 48 89 f2                    	mov	rdx, rsi
1409c1cd7: e8 24 1f f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c1cdc: c6 85 e0 0b 00 00 09        	mov	byte ptr [rbp + 0xbe0], 0x9
1409c1ce3: c7 85 b0 07 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x7b0], 0x2
1409c1ced: 48 89 74 24 20              	mov	qword ptr [rsp + 0x20], rsi
1409c1cf2: 48 8d b5 e0 0b 00 00        	lea	rsi, [rbp + 0xbe0]
1409c1cf9: 49 b8 f2 c5 70 04 5e 8b b0 61       	movabs	r8, 0x61b08b5e0470c5f2
1409c1d03: 48 89 f1                    	mov	rcx, rsi
1409c1d06: 48 89 fa                    	mov	rdx, rdi
1409c1d09: 41 b1 19                    	mov	r9b, 0x19
1409c1d0c: e8 4f 35 f4 ff              	call	0x140905260 <.text+0x904260>
1409c1d11: 48 8d bd b0 07 00 00        	lea	rdi, [rbp + 0x7b0]
1409c1d18: 48 8d 95 b0 02 00 00        	lea	rdx, [rbp + 0x2b0]
1409c1d1f: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c1d25: 48 89 f9                    	mov	rcx, rdi
1409c1d28: e8 d3 1e f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c1d2d: c6 85 b8 06 00 00 0a        	mov	byte ptr [rbp + 0x6b8], 0xa
1409c1d34: c7 85 e0 0b 00 00 02 00 00 00       	mov	dword ptr [rbp + 0xbe0], 0x2
1409c1d3e: 48 89 74 24 20              	mov	qword ptr [rsp + 0x20], rsi
1409c1d43: 48 8d 8d b8 06 00 00        	lea	rcx, [rbp + 0x6b8]
1409c1d4a: 49 b8 57 f6 e8 da 4c 1e 7d 29       	movabs	r8, 0x297d1e4cdae8f657
1409c1d54: 48 89 fa                    	mov	rdx, rdi
1409c1d57: 41 b1 19                    	mov	r9b, 0x19
1409c1d5a: e8 01 35 f4 ff              	call	0x140905260 <.text+0x904260>
1409c1d5f: 48 8d bd e0 0b 00 00        	lea	rdi, [rbp + 0xbe0]
1409c1d66: 48 8d b5 b0 07 00 00        	lea	rsi, [rbp + 0x7b0]
1409c1d6d: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c1d73: 48 89 f9                    	mov	rcx, rdi
1409c1d76: 48 89 f2                    	mov	rdx, rsi
1409c1d79: e8 82 1e f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c1d7e: c6 85 b0 02 00 00 08        	mov	byte ptr [rbp + 0x2b0], 0x8
1409c1d85: c7 85 b0 07 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x7b0], 0x2
1409c1d8f: 48 89 74 24 20              	mov	qword ptr [rsp + 0x20], rsi
1409c1d94: 48 8d b5 b0 02 00 00        	lea	rsi, [rbp + 0x2b0]
1409c1d9b: 49 b8 3f ca 8d ac 13 01 16 77       	movabs	r8, 0x77160113ac8dca3f
1409c1da5: 48 89 f1                    	mov	rcx, rsi
1409c1da8: 48 89 fa                    	mov	rdx, rdi
1409c1dab: 41 b1 19                    	mov	r9b, 0x19
1409c1dae: e8 ad 34 f4 ff              	call	0x140905260 <.text+0x904260>
1409c1db3: 48 8d bd b0 07 00 00        	lea	rdi, [rbp + 0x7b0]
1409c1dba: 48 8d 95 e0 0b 00 00        	lea	rdx, [rbp + 0xbe0]
1409c1dc1: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c1dc7: 48 89 f9                    	mov	rcx, rdi
1409c1dca: e8 31 1e f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c1dcf: 48 b8 65 18 a0 91 ab 39 9f 57       	movabs	rax, 0x579f39ab91a01865
1409c1dd9: 48 89 85 b8 06 00 00        	mov	qword ptr [rbp + 0x6b8], rax
1409c1de0: c7 85 b0 02 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x2b0], 0x2
1409c1dea: 48 89 74 24 20              	mov	qword ptr [rsp + 0x20], rsi
1409c1def: 48 8d 8d b8 06 00 00        	lea	rcx, [rbp + 0x6b8]
1409c1df6: 49 b8 8c 97 66 69 03 f5 5b 81       	movabs	r8, -0x7ea40afc96996874
1409c1e00: 48 89 fa                    	mov	rdx, rdi
1409c1e03: 41 b1 19                    	mov	r9b, 0x19
1409c1e06: e8 c5 38 f4 ff              	call	0x1409056d0 <.text+0x9046d0>
1409c1e0b: 48 8d 95 b0 07 00 00        	lea	rdx, [rbp + 0x7b0]
1409c1e12: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c1e18: 48 89 f1                    	mov	rcx, rsi
1409c1e1b: e8 e0 1d f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c1e20: 48 89 f9                    	mov	rcx, rdi
1409c1e23: 48 89 f2                    	mov	rdx, rsi
1409c1e26: 4c 8b 85 90 0d 00 00        	mov	r8, qword ptr [rbp + 0xd90]
1409c1e2d: e8 fe a3 c3 00              	call	0x1415fc230 <.text+0x15fb230>
1409c1e32: 48 8d 85 c4 01 00 00        	lea	rax, [rbp + 0x1c4]
1409c1e39: 48 89 85 e0 0b 00 00        	mov	qword ptr [rbp + 0xbe0], rax
1409c1e40: c7 85 b0 02 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x2b0], 0x2
1409c1e4a: 48 89 74 24 20              	mov	qword ptr [rsp + 0x20], rsi
1409c1e4f: 48 8d 8d e0 0b 00 00        	lea	rcx, [rbp + 0xbe0]
1409c1e56: 49 b8 d6 6b 7f 6b 27 01 59 2c       	movabs	r8, 0x2c5901276b7f6bd6
1409c1e60: 48 89 fa                    	mov	rdx, rdi
1409c1e63: 41 b1 32                    	mov	r9b, 0x32
1409c1e66: e8 95 be f2 ff              	call	0x1408edd00 <.text+0x8ecd00>
1409c1e6b: 48 8d bd b0 02 00 00        	lea	rdi, [rbp + 0x2b0]
1409c1e72: 48 8d b5 b0 07 00 00        	lea	rsi, [rbp + 0x7b0]
1409c1e79: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c1e7f: 48 89 f9                    	mov	rcx, rdi
1409c1e82: 48 89 f2                    	mov	rdx, rsi
1409c1e85: e8 76 1d f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c1e8a: 48 8d 85 e8 01 00 00        	lea	rax, [rbp + 0x1e8]
1409c1e91: 48 89 85 e0 0b 00 00        	mov	qword ptr [rbp + 0xbe0], rax
1409c1e98: c7 85 b0 07 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x7b0], 0x2
1409c1ea2: 48 89 74 24 20              	mov	qword ptr [rsp + 0x20], rsi
1409c1ea7: 48 8d b5 e0 0b 00 00        	lea	rsi, [rbp + 0xbe0]
1409c1eae: 49 b8 42 b3 aa d1 08 f9 c9 ec       	movabs	r8, -0x133606f72e554cbe
1409c1eb8: 48 89 f1                    	mov	rcx, rsi
1409c1ebb: 48 89 fa                    	mov	rdx, rdi
1409c1ebe: 41 b1 32                    	mov	r9b, 0x32
1409c1ec1: e8 3a be f2 ff              	call	0x1408edd00 <.text+0x8ecd00>
1409c1ec6: 48 8d bd b0 07 00 00        	lea	rdi, [rbp + 0x7b0]
1409c1ecd: 48 8d 95 b0 02 00 00        	lea	rdx, [rbp + 0x2b0]
1409c1ed4: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c1eda: 48 89 f9                    	mov	rcx, rdi
1409c1edd: e8 1e 1d f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c1ee2: 48 8d 95 a0 06 00 00        	lea	rdx, [rbp + 0x6a0]
1409c1ee9: 48 89 f9                    	mov	rcx, rdi
1409c1eec: 45 31 c0                    	xor	r8d, r8d
1409c1eef: e8 8c c9 fa ff              	call	0x14096e880 <.text+0x96d880>
1409c1ef4: 4c 89 f1                    	mov	rcx, r14
1409c1ef7: e8 94 38 21 00              	call	0x140bd5790 <.text+0xbd4790>
1409c1efc: 48 89 f9                    	mov	rcx, rdi
1409c1eff: 48 89 c2                    	mov	rdx, rax
1409c1f02: 41 b8 7a 00 00 00           	mov	r8d, 0x7a
1409c1f08: e8 93 33 21 00              	call	0x140bd52a0 <.text+0xbd42a0>
1409c1f0d: 48 8d 9d b0 02 00 00        	lea	rbx, [rbp + 0x2b0]
1409c1f14: 48 89 d9                    	mov	rcx, rbx
1409c1f17: 48 89 fa                    	mov	rdx, rdi
1409c1f1a: e8 51 34 21 00              	call	0x140bd5370 <.text+0xbd4370>
1409c1f1f: 48 89 f1                    	mov	rcx, rsi
1409c1f22: 48 89 da                    	mov	rdx, rbx
1409c1f25: e8 66 34 21 00              	call	0x140bd5390 <.text+0xbd4390>
1409c1f2a: 48 89 74 24 20              	mov	qword ptr [rsp + 0x20], rsi
1409c1f2f: 4c 8d 05 d8 de 0f 02        	lea	r8, [rip + 0x20fded8]   # 0x142abfe0e
1409c1f36: 48 8d 8d b0 07 00 00        	lea	rcx, [rbp + 0x7b0]
1409c1f3d: 41 b9 13 00 00 00           	mov	r9d, 0x13
1409c1f43: 4c 89 fa                    	mov	rdx, r15
1409c1f46: e8 45 b5 fa ff              	call	0x14096d490 <.text+0x96c490>
1409c1f4b: 83 bd b0 07 00 00 01        	cmp	dword ptr [rbp + 0x7b0], 0x1
1409c1f52: 0f 84 bc 0b 00 00           	je	0x1409c2b14 <.text+0x9c1b14>
1409c1f58: 48 8b 85 f4 07 00 00        	mov	rax, qword ptr [rbp + 0x7f4]
1409c1f5f: 48 89 85 f0 02 00 00        	mov	qword ptr [rbp + 0x2f0], rax
1409c1f66: 0f 10 85 b4 07 00 00        	movups	xmm0, xmmword ptr [rbp + 0x7b4]
1409c1f6d: 0f 10 8d c4 07 00 00        	movups	xmm1, xmmword ptr [rbp + 0x7c4]
1409c1f74: 0f 10 95 d4 07 00 00        	movups	xmm2, xmmword ptr [rbp + 0x7d4]
1409c1f7b: 0f 10 9d e4 07 00 00        	movups	xmm3, xmmword ptr [rbp + 0x7e4]
1409c1f82: 0f 29 9d e0 02 00 00        	movaps	xmmword ptr [rbp + 0x2e0], xmm3
1409c1f89: 0f 29 95 d0 02 00 00        	movaps	xmmword ptr [rbp + 0x2d0], xmm2
1409c1f90: 0f 29 8d c0 02 00 00        	movaps	xmmword ptr [rbp + 0x2c0], xmm1
1409c1f97: 0f 29 85 b0 02 00 00        	movaps	xmmword ptr [rbp + 0x2b0], xmm0
1409c1f9e: 0f b6 9d fc 07 00 00        	movzx	ebx, byte ptr [rbp + 0x7fc]
1409c1fa5: 8b 85 d0 02 00 00           	mov	eax, dword ptr [rbp + 0x2d0]
1409c1fab: 89 85 f0 00 00 00           	mov	dword ptr [rbp + 0xf0], eax
1409c1fb1: 0f 29 8d e0 00 00 00        	movaps	xmmword ptr [rbp + 0xe0], xmm1
1409c1fb8: 0f 29 85 d0 00 00 00        	movaps	xmmword ptr [rbp + 0xd0], xmm0
1409c1fbf: 0f 10 85 e4 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2e4]
1409c1fc6: 0f 29 85 40 02 00 00        	movaps	xmmword ptr [rbp + 0x240], xmm0
1409c1fcd: 0f 10 85 d4 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2d4]
1409c1fd4: 0f 29 85 30 02 00 00        	movaps	xmmword ptr [rbp + 0x230], xmm0
1409c1fdb: 8b 85 f4 02 00 00           	mov	eax, dword ptr [rbp + 0x2f4]
1409c1fe1: 89 85 50 02 00 00           	mov	dword ptr [rbp + 0x250], eax
1409c1fe7: 4c 89 f1                    	mov	rcx, r14
1409c1fea: e8 a1 37 21 00              	call	0x140bd5790 <.text+0xbd4790>
1409c1fef: 4c 8d b5 b0 07 00 00        	lea	r14, [rbp + 0x7b0]
1409c1ff6: 4c 89 f1                    	mov	rcx, r14
1409c1ff9: 48 89 c2                    	mov	rdx, rax
1409c1ffc: 41 b8 53 00 00 00           	mov	r8d, 0x53
1409c2002: e8 99 32 21 00              	call	0x140bd52a0 <.text+0xbd42a0>
1409c2007: 48 8d bd b0 02 00 00        	lea	rdi, [rbp + 0x2b0]
1409c200e: 48 89 f9                    	mov	rcx, rdi
1409c2011: 4c 89 f2                    	mov	rdx, r14
1409c2014: e8 57 33 21 00              	call	0x140bd5370 <.text+0xbd4370>
1409c2019: 48 89 f1                    	mov	rcx, rsi
1409c201c: 48 89 fa                    	mov	rdx, rdi
1409c201f: e8 6c 33 21 00              	call	0x140bd5390 <.text+0xbd4390>
1409c2024: 48 89 74 24 20              	mov	qword ptr [rsp + 0x20], rsi
1409c2029: 4c 8d 05 10 de 0f 02        	lea	r8, [rip + 0x20fde10]   # 0x142abfe40
1409c2030: 48 8d 8d b0 07 00 00        	lea	rcx, [rbp + 0x7b0]
1409c2037: 41 b9 10 00 00 00           	mov	r9d, 0x10
1409c203d: 4c 89 fa                    	mov	rdx, r15
1409c2040: e8 4b b4 fa ff              	call	0x14096d490 <.text+0x96c490>
1409c2045: 83 bd b0 07 00 00 01        	cmp	dword ptr [rbp + 0x7b0], 0x1
1409c204c: 0f 84 fd 0a 00 00           	je	0x1409c2b4f <.text+0x9c1b4f>
1409c2052: 23 9d fc 07 00 00           	and	ebx, dword ptr [rbp + 0x7fc]
1409c2058: 48 8b 85 f4 07 00 00        	mov	rax, qword ptr [rbp + 0x7f4]
1409c205f: 48 89 85 f0 02 00 00        	mov	qword ptr [rbp + 0x2f0], rax
1409c2066: 0f 10 85 b4 07 00 00        	movups	xmm0, xmmword ptr [rbp + 0x7b4]
1409c206d: 0f 10 8d c4 07 00 00        	movups	xmm1, xmmword ptr [rbp + 0x7c4]
1409c2074: 0f 10 95 d4 07 00 00        	movups	xmm2, xmmword ptr [rbp + 0x7d4]
1409c207b: 0f 10 9d e4 07 00 00        	movups	xmm3, xmmword ptr [rbp + 0x7e4]
1409c2082: 0f 29 9d e0 02 00 00        	movaps	xmmword ptr [rbp + 0x2e0], xmm3
1409c2089: 0f 29 95 d0 02 00 00        	movaps	xmmword ptr [rbp + 0x2d0], xmm2
1409c2090: 0f 29 8d c0 02 00 00        	movaps	xmmword ptr [rbp + 0x2c0], xmm1
1409c2097: 0f 29 85 b0 02 00 00        	movaps	xmmword ptr [rbp + 0x2b0], xmm0
1409c209e: 8b 85 d0 02 00 00           	mov	eax, dword ptr [rbp + 0x2d0]
1409c20a4: 89 85 a0 02 00 00           	mov	dword ptr [rbp + 0x2a0], eax
1409c20aa: 0f 29 8d 90 02 00 00        	movaps	xmmword ptr [rbp + 0x290], xmm1
1409c20b1: 0f 29 85 80 02 00 00        	movaps	xmmword ptr [rbp + 0x280], xmm0
1409c20b8: 0f 10 85 e4 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2e4]
1409c20bf: 0f 29 45 e0                 	movaps	xmmword ptr [rbp - 0x20], xmm0
1409c20c3: 0f 10 85 d4 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2d4]
1409c20ca: 0f 29 45 d0                 	movaps	xmmword ptr [rbp - 0x30], xmm0
1409c20ce: 8b 85 f4 02 00 00           	mov	eax, dword ptr [rbp + 0x2f4]
1409c20d4: 89 45 f0                    	mov	dword ptr [rbp - 0x10], eax
1409c20d7: 4c 89 7c 24 30              	mov	qword ptr [rsp + 0x30], r15
1409c20dc: 48 8b 85 90 0d 00 00        	mov	rax, qword ptr [rbp + 0xd90]
1409c20e3: 48 89 44 24 28              	mov	qword ptr [rsp + 0x28], rax
1409c20e8: 48 c7 44 24 20 1f 00 00 00  	mov	qword ptr [rsp + 0x20], 0x1f
1409c20f1: 48 8d 15 70 dd 0f 02        	lea	rdx, [rip + 0x20fdd70]  # 0x142abfe68
1409c20f8: 4c 8d 0d 79 dd 0f 02        	lea	r9, [rip + 0x20fdd79]   # 0x142abfe78
1409c20ff: 48 8d b5 b0 07 00 00        	lea	rsi, [rbp + 0x7b0]
1409c2106: 41 b8 10 00 00 00           	mov	r8d, 0x10
1409c210c: 48 89 f1                    	mov	rcx, rsi
1409c210f: e8 8c 11 c4 00              	call	0x1416032a0 <.text+0x16022a0>
1409c2114: 48 8d 85 a0 01 00 00        	lea	rax, [rbp + 0x1a0]
1409c211b: 48 89 85 e0 0b 00 00        	mov	qword ptr [rbp + 0xbe0], rax
1409c2122: c7 85 b0 02 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x2b0], 0x2
1409c212c: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
1409c2131: 48 8d 8d e0 0b 00 00        	lea	rcx, [rbp + 0xbe0]
1409c2138: 49 b8 e4 bb 15 d2 00 f8 28 11       	movabs	r8, 0x1128f800d215bbe4
1409c2142: 48 89 f2                    	mov	rdx, rsi
1409c2145: 41 b1 19                    	mov	r9b, 0x19
1409c2148: e8 f3 c0 f2 ff              	call	0x1408ee240 <.text+0x8ed240>
1409c214d: 48 8d 8d b0 02 00 00        	lea	rcx, [rbp + 0x2b0]
1409c2154: 48 8d 95 b0 07 00 00        	lea	rdx, [rbp + 0x7b0]
1409c215b: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c2161: e8 9a 1a f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c2166: c6 85 9e 0d 00 00 01        	mov	byte ptr [rbp + 0xd9e], 0x1
1409c216d: 48 8d 8d b0 07 00 00        	lea	rcx, [rbp + 0x7b0]
1409c2174: e8 27 36 21 00              	call	0x140bd57a0 <.text+0xbd47a0>
1409c2179: c6 85 9e 0d 00 00 01        	mov	byte ptr [rbp + 0xd9e], 0x1
1409c2180: 48 8d 8d e0 0b 00 00        	lea	rcx, [rbp + 0xbe0]
1409c2187: 48 89 f2                    	mov	rdx, rsi
1409c218a: 41 b8 7b 00 00 00           	mov	r8d, 0x7b
1409c2190: e8 cb 36 21 00              	call	0x140bd5860 <.text+0xbd4860>
1409c2195: c6 85 9e 0d 00 00 01        	mov	byte ptr [rbp + 0xd9e], 0x1
1409c219c: 48 8d 8d b8 06 00 00        	lea	rcx, [rbp + 0x6b8]
1409c21a3: 48 8d 95 e0 0b 00 00        	lea	rdx, [rbp + 0xbe0]
1409c21aa: 41 b8 04 00 00 00           	mov	r8d, 0x4
1409c21b0: e8 6b 36 21 00              	call	0x140bd5820 <.text+0xbd4820>
1409c21b5: 48 8d 85 7c 01 00 00        	lea	rax, [rbp + 0x17c]
1409c21bc: 48 89 85 e0 0b 00 00        	mov	qword ptr [rbp + 0xbe0], rax
1409c21c3: 0f 10 85 b8 06 00 00        	movups	xmm0, xmmword ptr [rbp + 0x6b8]
1409c21ca: 0f 10 8d c8 06 00 00        	movups	xmm1, xmmword ptr [rbp + 0x6c8]
1409c21d1: 0f 10 95 d8 06 00 00        	movups	xmm2, xmmword ptr [rbp + 0x6d8]
1409c21d8: 0f 10 9d e4 06 00 00        	movups	xmm3, xmmword ptr [rbp + 0x6e4]
1409c21df: 0f 11 9d dc 07 00 00        	movups	xmmword ptr [rbp + 0x7dc], xmm3
1409c21e6: 0f 29 95 d0 07 00 00        	movaps	xmmword ptr [rbp + 0x7d0], xmm2
1409c21ed: 0f 29 8d c0 07 00 00        	movaps	xmmword ptr [rbp + 0x7c0], xmm1
1409c21f4: 0f 29 85 b0 07 00 00        	movaps	xmmword ptr [rbp + 0x7b0], xmm0
1409c21fb: 48 89 74 24 20              	mov	qword ptr [rsp + 0x20], rsi
1409c2200: 48 8d 8d e0 0b 00 00        	lea	rcx, [rbp + 0xbe0]
1409c2207: 48 8d 95 b0 02 00 00        	lea	rdx, [rbp + 0x2b0]
1409c220e: 49 b8 b4 79 11 73 07 9e 47 de       	movabs	r8, -0x21b861f88cee864c
1409c2218: 41 b1 19                    	mov	r9b, 0x19
1409c221b: e8 20 c0 f2 ff              	call	0x1408ee240 <.text+0x8ed240>
1409c2220: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c2226: 48 89 f1                    	mov	rcx, rsi
1409c2229: 48 89 fa                    	mov	rdx, rdi
1409c222c: e8 cf 19 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c2231: 4c 89 ad e0 0b 00 00        	mov	qword ptr [rbp + 0xbe0], r13
1409c2238: c7 85 b0 02 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x2b0], 0x2
1409c2242: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
1409c2247: 48 8d 8d e0 0b 00 00        	lea	rcx, [rbp + 0xbe0]
1409c224e: 49 b8 fc fb 49 a0 9d 57 61 48       	movabs	r8, 0x4861579da049fbfc
1409c2258: 48 89 f2                    	mov	rdx, rsi
1409c225b: 41 b1 19                    	mov	r9b, 0x19
1409c225e: e8 dd bf f2 ff              	call	0x1408ee240 <.text+0x8ed240>
1409c2263: 48 8d b5 b0 02 00 00        	lea	rsi, [rbp + 0x2b0]
1409c226a: 48 8d bd b0 07 00 00        	lea	rdi, [rbp + 0x7b0]
1409c2271: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c2277: 48 89 f1                    	mov	rcx, rsi
1409c227a: 48 89 fa                    	mov	rdx, rdi
1409c227d: e8 7e 19 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c2282: 4c 8d bd d0 00 00 00        	lea	r15, [rbp + 0xd0]
1409c2289: 4c 89 bd e0 0b 00 00        	mov	qword ptr [rbp + 0xbe0], r15
1409c2290: c7 85 b0 07 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x7b0], 0x2
1409c229a: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
1409c229f: 48 8d 8d e0 0b 00 00        	lea	rcx, [rbp + 0xbe0]
1409c22a6: 49 b8 ab 9c df c1 c7 60 c2 b8       	movabs	r8, -0x473d9f383e206355
1409c22b0: 48 89 f2                    	mov	rdx, rsi
1409c22b3: 41 b1 32                    	mov	r9b, 0x32
1409c22b6: e8 45 ba f2 ff              	call	0x1408edd00 <.text+0x8ecd00>
1409c22bb: 48 8d 8d b0 07 00 00        	lea	rcx, [rbp + 0x7b0]
1409c22c2: 48 8d 95 b0 02 00 00        	lea	rdx, [rbp + 0x2b0]
1409c22c9: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c22cf: e8 2c 19 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c22d4: 4d 89 e6                    	mov	r14, r12
1409c22d7: 49 c1 ee 20                 	shr	r14, 0x20
1409c22db: 0f 10 85 50 08 00 00        	movups	xmm0, xmmword ptr [rbp + 0x850]
1409c22e2: 0f 29 85 20 0c 00 00        	movaps	xmmword ptr [rbp + 0xc20], xmm0
1409c22e9: 0f 10 85 40 08 00 00        	movups	xmm0, xmmword ptr [rbp + 0x840]
1409c22f0: 0f 29 85 10 0c 00 00        	movaps	xmmword ptr [rbp + 0xc10], xmm0
1409c22f7: 0f 10 85 30 08 00 00        	movups	xmm0, xmmword ptr [rbp + 0x830]
1409c22fe: 0f 29 85 00 0c 00 00        	movaps	xmmword ptr [rbp + 0xc00], xmm0
1409c2305: 0f 10 85 20 08 00 00        	movups	xmm0, xmmword ptr [rbp + 0x820]
1409c230c: 0f 29 85 f0 0b 00 00        	movaps	xmmword ptr [rbp + 0xbf0], xmm0
1409c2313: 0f 10 85 10 08 00 00        	movups	xmm0, xmmword ptr [rbp + 0x810]
1409c231a: 0f 29 85 e0 0b 00 00        	movaps	xmmword ptr [rbp + 0xbe0], xmm0
1409c2321: 48 8b 85 e8 08 00 00        	mov	rax, qword ptr [rbp + 0x8e8]
1409c2328: 48 8b 8d 30 09 00 00        	mov	rcx, qword ptr [rbp + 0x930]
1409c232f: 48 8b 95 c0 07 00 00        	mov	rdx, qword ptr [rbp + 0x7c0]
1409c2336: 48 89 95 c0 02 00 00        	mov	qword ptr [rbp + 0x2c0], rdx
1409c233d: 0f 10 85 b0 07 00 00        	movups	xmm0, xmmword ptr [rbp + 0x7b0]
1409c2344: 0f 29 85 b0 02 00 00        	movaps	xmmword ptr [rbp + 0x2b0], xmm0
1409c234b: 0f 10 85 c8 07 00 00        	movups	xmm0, xmmword ptr [rbp + 0x7c8]
1409c2352: 0f 29 85 e0 02 00 00        	movaps	xmmword ptr [rbp + 0x2e0], xmm0
1409c2359: 48 8b 95 d8 07 00 00        	mov	rdx, qword ptr [rbp + 0x7d8]
1409c2360: 48 89 95 f0 02 00 00        	mov	qword ptr [rbp + 0x2f0], rdx
1409c2367: 0f 10 85 e0 07 00 00        	movups	xmm0, xmmword ptr [rbp + 0x7e0]
1409c236e: 0f 11 85 c8 02 00 00        	movups	xmmword ptr [rbp + 0x2c8], xmm0
1409c2375: 48 8b 95 f0 07 00 00        	mov	rdx, qword ptr [rbp + 0x7f0]
1409c237c: 48 89 95 d8 02 00 00        	mov	qword ptr [rbp + 0x2d8], rdx
1409c2383: 0f 10 85 60 08 00 00        	movups	xmm0, xmmword ptr [rbp + 0x860]
1409c238a: 0f 10 8d 70 08 00 00        	movups	xmm1, xmmword ptr [rbp + 0x870]
1409c2391: 0f 10 95 80 08 00 00        	movups	xmm2, xmmword ptr [rbp + 0x880]
1409c2398: 0f 10 9d 90 08 00 00        	movups	xmm3, xmmword ptr [rbp + 0x890]
1409c239f: 0f 11 85 28 03 00 00        	movups	xmmword ptr [rbp + 0x328], xmm0
1409c23a6: 0f 11 8d 38 03 00 00        	movups	xmmword ptr [rbp + 0x338], xmm1
1409c23ad: 0f 11 95 48 03 00 00        	movups	xmmword ptr [rbp + 0x348], xmm2
1409c23b4: 0f 11 9d 58 03 00 00        	movups	xmmword ptr [rbp + 0x358], xmm3
1409c23bb: 0f 10 85 a0 08 00 00        	movups	xmm0, xmmword ptr [rbp + 0x8a0]
1409c23c2: 0f 11 85 68 03 00 00        	movups	xmmword ptr [rbp + 0x368], xmm0
1409c23c9: 0f 10 85 b0 08 00 00        	movups	xmm0, xmmword ptr [rbp + 0x8b0]
1409c23d0: 0f 11 85 78 03 00 00        	movups	xmmword ptr [rbp + 0x378], xmm0
1409c23d7: 48 8b 95 08 08 00 00        	mov	rdx, qword ptr [rbp + 0x808]
1409c23de: 48 89 95 08 03 00 00        	mov	qword ptr [rbp + 0x308], rdx
1409c23e5: 0f 10 85 f8 07 00 00        	movups	xmm0, xmmword ptr [rbp + 0x7f8]
1409c23ec: 0f 11 85 f8 02 00 00        	movups	xmmword ptr [rbp + 0x2f8], xmm0
1409c23f3: 48 89 8d 90 03 00 00        	mov	qword ptr [rbp + 0x390], rcx
1409c23fa: 48 89 85 88 03 00 00        	mov	qword ptr [rbp + 0x388], rax
1409c2401: c7 85 10 03 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x310], 0x1
1409c240b: 44 89 a5 14 03 00 00        	mov	dword ptr [rbp + 0x314], r12d
1409c2412: 44 89 b5 18 03 00 00        	mov	dword ptr [rbp + 0x318], r14d
1409c2419: c7 85 1c 03 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x31c], 0x1
1409c2423: 48 8d 8d e0 0b 00 00        	lea	rcx, [rbp + 0xbe0]
1409c242a: 48 89 f2                    	mov	rdx, rsi
1409c242d: e8 9e b0 01 00              	call	0x1409dd4d0 <.text+0x9dc4d0>
1409c2432: 80 bd e6 08 00 00 08        	cmp	byte ptr [rbp + 0x8e6], 0x8
1409c2439: 74 0c                       	je	0x1409c2447 <.text+0x9c1447>
1409c243b: 48 8d 8d d0 08 00 00        	lea	rcx, [rbp + 0x8d0]
1409c2442: e8 69 00 22 00              	call	0x140be24b0 <.text+0xbe14b0>
1409c2447: 48 8b 85 80 0d 00 00        	mov	rax, qword ptr [rbp + 0xd80]
1409c244e: 48 89 44 24 30              	mov	qword ptr [rsp + 0x30], rax
1409c2453: 48 8b 85 90 0d 00 00        	mov	rax, qword ptr [rbp + 0xd90]
1409c245a: 48 89 44 24 28              	mov	qword ptr [rsp + 0x28], rax
1409c245f: 48 c7 44 24 20 1f 00 00 00  	mov	qword ptr [rsp + 0x20], 0x1f
1409c2468: 48 8d 15 28 da 0f 02        	lea	rdx, [rip + 0x20fda28]  # 0x142abfe97
1409c246f: 4c 8d 0d 31 da 0f 02        	lea	r9, [rip + 0x20fda31]   # 0x142abfea7
1409c2476: 41 b8 10 00 00 00           	mov	r8d, 0x10
1409c247c: 48 89 f9                    	mov	rcx, rdi
1409c247f: e8 1c 0e c4 00              	call	0x1416032a0 <.text+0x16022a0>
1409c2484: c6 85 e0 0b 00 00 00        	mov	byte ptr [rbp + 0xbe0], 0x0
1409c248b: c7 85 b0 02 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x2b0], 0x2
1409c2495: 48 89 74 24 20              	mov	qword ptr [rsp + 0x20], rsi
1409c249a: 48 8d 8d e0 0b 00 00        	lea	rcx, [rbp + 0xbe0]
1409c24a1: 49 b8 45 41 25 6a 8a 1a 9d b7       	movabs	r8, -0x4862e57595dabebb
1409c24ab: 48 89 fa                    	mov	rdx, rdi
1409c24ae: 41 b1 19                    	mov	r9b, 0x19
1409c24b1: e8 aa 2d f4 ff              	call	0x140905260 <.text+0x904260>
1409c24b6: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c24bc: 48 89 f1                    	mov	rcx, rsi
1409c24bf: 48 89 fa                    	mov	rdx, rdi
1409c24c2: e8 39 17 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c24c7: 48 8d 85 c4 01 00 00        	lea	rax, [rbp + 0x1c4]
1409c24ce: 48 89 85 e0 0b 00 00        	mov	qword ptr [rbp + 0xbe0], rax
1409c24d5: c7 85 b0 07 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x7b0], 0x2
1409c24df: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
1409c24e4: 48 8d 8d e0 0b 00 00        	lea	rcx, [rbp + 0xbe0]
1409c24eb: 49 b8 d6 6b 7f 6b 27 01 59 2c       	movabs	r8, 0x2c5901276b7f6bd6
1409c24f5: 48 89 f2                    	mov	rdx, rsi
1409c24f8: 41 b1 19                    	mov	r9b, 0x19
1409c24fb: e8 40 bd f2 ff              	call	0x1408ee240 <.text+0x8ed240>
1409c2500: 48 8d 8d b0 07 00 00        	lea	rcx, [rbp + 0x7b0]
1409c2507: 48 8d 95 b0 02 00 00        	lea	rdx, [rbp + 0x2b0]
1409c250e: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c2514: e8 e7 16 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c2519: c6 85 9d 0d 00 00 01        	mov	byte ptr [rbp + 0xd9d], 0x1
1409c2520: 48 8d 8d b0 02 00 00        	lea	rcx, [rbp + 0x2b0]
1409c2527: e8 74 32 21 00              	call	0x140bd57a0 <.text+0xbd47a0>
1409c252c: c6 85 9d 0d 00 00 01        	mov	byte ptr [rbp + 0xd9d], 0x1
1409c2533: 48 8d 8d e0 0b 00 00        	lea	rcx, [rbp + 0xbe0]
1409c253a: 48 89 f2                    	mov	rdx, rsi
1409c253d: 41 b8 7b 00 00 00           	mov	r8d, 0x7b
1409c2543: e8 18 33 21 00              	call	0x140bd5860 <.text+0xbd4860>
1409c2548: c6 85 9d 0d 00 00 01        	mov	byte ptr [rbp + 0xd9d], 0x1
1409c254f: 48 8d 8d b8 06 00 00        	lea	rcx, [rbp + 0x6b8]
1409c2556: 48 8d 95 e0 0b 00 00        	lea	rdx, [rbp + 0xbe0]
1409c255d: 41 b8 04 00 00 00           	mov	r8d, 0x4
1409c2563: e8 b8 32 21 00              	call	0x140bd5820 <.text+0xbd4820>
1409c2568: 48 8d 85 e8 01 00 00        	lea	rax, [rbp + 0x1e8]
1409c256f: 48 89 85 e0 0b 00 00        	mov	qword ptr [rbp + 0xbe0], rax
1409c2576: 0f 10 85 b8 06 00 00        	movups	xmm0, xmmword ptr [rbp + 0x6b8]
1409c257d: 0f 10 8d c8 06 00 00        	movups	xmm1, xmmword ptr [rbp + 0x6c8]
1409c2584: 0f 10 95 d8 06 00 00        	movups	xmm2, xmmword ptr [rbp + 0x6d8]
1409c258b: 0f 10 9d e4 06 00 00        	movups	xmm3, xmmword ptr [rbp + 0x6e4]
1409c2592: 0f 11 9d dc 02 00 00        	movups	xmmword ptr [rbp + 0x2dc], xmm3
1409c2599: 0f 29 95 d0 02 00 00        	movaps	xmmword ptr [rbp + 0x2d0], xmm2
1409c25a0: 0f 29 8d c0 02 00 00        	movaps	xmmword ptr [rbp + 0x2c0], xmm1
1409c25a7: 0f 29 85 b0 02 00 00        	movaps	xmmword ptr [rbp + 0x2b0], xmm0
1409c25ae: 48 89 74 24 20              	mov	qword ptr [rsp + 0x20], rsi
1409c25b3: 48 8d 8d e0 0b 00 00        	lea	rcx, [rbp + 0xbe0]
1409c25ba: 48 8d 95 b0 07 00 00        	lea	rdx, [rbp + 0x7b0]
1409c25c1: 49 b8 42 b3 aa d1 08 f9 c9 ec       	movabs	r8, -0x133606f72e554cbe
1409c25cb: 41 b1 19                    	mov	r9b, 0x19
1409c25ce: e8 6d bc f2 ff              	call	0x1408ee240 <.text+0x8ed240>
1409c25d3: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c25d9: 48 89 f1                    	mov	rcx, rsi
1409c25dc: 48 89 fa                    	mov	rdx, rdi
1409c25df: e8 1c 16 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c25e4: 4c 89 ad e0 0b 00 00        	mov	qword ptr [rbp + 0xbe0], r13
1409c25eb: c7 85 b0 07 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x7b0], 0x2
1409c25f5: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
1409c25fa: 48 8d 8d e0 0b 00 00        	lea	rcx, [rbp + 0xbe0]
1409c2601: 49 b8 fc fb 49 a0 9d 57 61 48       	movabs	r8, 0x4861579da049fbfc
1409c260b: 48 89 f2                    	mov	rdx, rsi
1409c260e: 41 b1 19                    	mov	r9b, 0x19
1409c2611: e8 2a bc f2 ff              	call	0x1408ee240 <.text+0x8ed240>
1409c2616: 48 8d b5 b0 07 00 00        	lea	rsi, [rbp + 0x7b0]
1409c261d: 48 8d bd b0 02 00 00        	lea	rdi, [rbp + 0x2b0]
1409c2624: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c262a: 48 89 f1                    	mov	rcx, rsi
1409c262d: 48 89 fa                    	mov	rdx, rdi
1409c2630: e8 cb 15 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c2635: 48 b8 65 18 a0 91 ab 39 9f 57       	movabs	rax, 0x579f39ab91a01865
1409c263f: 48 89 85 e0 0b 00 00        	mov	qword ptr [rbp + 0xbe0], rax
1409c2646: c7 85 b0 02 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x2b0], 0x2
1409c2650: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
1409c2655: 48 8d 8d e0 0b 00 00        	lea	rcx, [rbp + 0xbe0]
1409c265c: 49 b8 8c 97 66 69 03 f5 5b 81       	movabs	r8, -0x7ea40afc96996874
1409c2666: 48 89 f2                    	mov	rdx, rsi
1409c2669: 41 b1 19                    	mov	r9b, 0x19
1409c266c: e8 5f 30 f4 ff              	call	0x1409056d0 <.text+0x9046d0>
1409c2671: 48 8d b5 b0 02 00 00        	lea	rsi, [rbp + 0x2b0]
1409c2678: 48 8d bd b0 07 00 00        	lea	rdi, [rbp + 0x7b0]
1409c267f: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c2685: 48 89 f1                    	mov	rcx, rsi
1409c2688: 48 89 fa                    	mov	rdx, rdi
1409c268b: e8 70 15 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c2690: 48 8b 85 30 0e 00 00        	mov	rax, qword ptr [rbp + 0xe30]
1409c2697: 48 89 85 e0 0b 00 00        	mov	qword ptr [rbp + 0xbe0], rax
1409c269e: c7 85 b0 07 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x7b0], 0x2
1409c26a8: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
1409c26ad: 48 8d 8d e0 0b 00 00        	lea	rcx, [rbp + 0xbe0]
1409c26b4: 49 b8 e3 6b 5a 83 cd 4c a7 b8       	movabs	r8, -0x4758b3327ca5941d
1409c26be: 48 89 f2                    	mov	rdx, rsi
1409c26c1: 41 b1 19                    	mov	r9b, 0x19
1409c26c4: e8 77 bb f2 ff              	call	0x1408ee240 <.text+0x8ed240>
1409c26c9: 48 8d b5 b0 07 00 00        	lea	rsi, [rbp + 0x7b0]
1409c26d0: 48 8d bd b0 02 00 00        	lea	rdi, [rbp + 0x2b0]
1409c26d7: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c26dd: 48 89 f1                    	mov	rcx, rsi
1409c26e0: 48 89 fa                    	mov	rdx, rdi
1409c26e3: e8 18 15 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c26e8: 48 8d 85 0c 02 00 00        	lea	rax, [rbp + 0x20c]
1409c26ef: 48 89 85 e0 0b 00 00        	mov	qword ptr [rbp + 0xbe0], rax
1409c26f6: c7 85 b0 02 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x2b0], 0x2
1409c2700: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
1409c2705: 48 8d 8d e0 0b 00 00        	lea	rcx, [rbp + 0xbe0]
1409c270c: 49 b8 83 b8 5a e4 eb 2b 90 53       	movabs	r8, 0x53902bebe45ab883
1409c2716: 48 89 f2                    	mov	rdx, rsi
1409c2719: 41 b1 19                    	mov	r9b, 0x19
1409c271c: e8 1f bb f2 ff              	call	0x1408ee240 <.text+0x8ed240>
1409c2721: 48 8d b5 b0 02 00 00        	lea	rsi, [rbp + 0x2b0]
1409c2728: 48 8d bd b0 07 00 00        	lea	rdi, [rbp + 0x7b0]
1409c272f: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c2735: 48 89 f1                    	mov	rcx, rsi
1409c2738: 48 89 fa                    	mov	rdx, rdi
1409c273b: e8 c0 14 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c2740: 48 8b 85 b8 0b 00 00        	mov	rax, qword ptr [rbp + 0xbb8]
1409c2747: 48 89 85 e0 0b 00 00        	mov	qword ptr [rbp + 0xbe0], rax
1409c274e: c7 85 b0 07 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x7b0], 0x2
1409c2758: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
1409c275d: 48 8d 8d e0 0b 00 00        	lea	rcx, [rbp + 0xbe0]
1409c2764: 49 b8 5e a9 e9 1c 54 8e 97 d9       	movabs	r8, -0x266871abe31656a2
1409c276e: 48 89 f2                    	mov	rdx, rsi
1409c2771: 41 b1 32                    	mov	r9b, 0x32
1409c2774: e8 87 b5 f2 ff              	call	0x1408edd00 <.text+0x8ecd00>
1409c2779: 48 8d 8d b0 07 00 00        	lea	rcx, [rbp + 0x7b0]
1409c2780: 48 8d 95 b0 02 00 00        	lea	rdx, [rbp + 0x2b0]
1409c2787: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c278d: e8 6e 14 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c2792: 80 bd e6 08 00 00 08        	cmp	byte ptr [rbp + 0x8e6], 0x8
1409c2799: 74 28                       	je	0x1409c27c3 <.text+0x9c17c3>
1409c279b: 83 e3 01                    	and	ebx, 0x1
1409c279e: 48 8d 8d c0 08 00 00        	lea	rcx, [rbp + 0x8c0]
1409c27a5: ba e8 00 00 00              	mov	edx, 0xe8
1409c27aa: 48 03 95 e8 08 00 00        	add	rdx, qword ptr [rbp + 0x8e8]
1409c27b1: 49 b8 02 d9 ee 60 b7 09 a1 a9       	movabs	r8, -0x565ef6489f1126fe
1409c27bb: 41 89 d9                    	mov	r9d, ebx
1409c27be: e8 ad 8d fa ff              	call	0x14096b570 <.text+0x96a570>
1409c27c3: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c27c9: 48 89 f1                    	mov	rcx, rsi
1409c27cc: 48 89 fa                    	mov	rdx, rdi
1409c27cf: e8 2c 14 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c27d4: 48 8d 85 30 02 00 00        	lea	rax, [rbp + 0x230]
1409c27db: 48 89 85 e0 0b 00 00        	mov	qword ptr [rbp + 0xbe0], rax
1409c27e2: c7 85 b0 07 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x7b0], 0x2
1409c27ec: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
1409c27f1: 48 8d 8d e0 0b 00 00        	lea	rcx, [rbp + 0xbe0]
1409c27f8: 49 b8 93 d2 3c b8 4f 97 59 f4       	movabs	r8, -0xba668b047c32d6d
1409c2802: 48 89 f2                    	mov	rdx, rsi
1409c2805: 41 b1 19                    	mov	r9b, 0x19
1409c2808: e8 83 c8 fa ff              	call	0x14096f090 <.text+0x96e090>
1409c280d: 48 8d b5 b0 07 00 00        	lea	rsi, [rbp + 0x7b0]
1409c2814: 48 8d bd b0 02 00 00        	lea	rdi, [rbp + 0x2b0]
1409c281b: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c2821: 48 89 f1                    	mov	rcx, rsi
1409c2824: 48 89 fa                    	mov	rdx, rdi
1409c2827: e8 d4 13 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c282c: 4c 89 bd e0 0b 00 00        	mov	qword ptr [rbp + 0xbe0], r15
1409c2833: c7 85 b0 02 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x2b0], 0x2
1409c283d: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
1409c2842: 48 8d 8d e0 0b 00 00        	lea	rcx, [rbp + 0xbe0]
1409c2849: 49 b8 ab 9c df c1 c7 60 c2 b8       	movabs	r8, -0x473d9f383e206355
1409c2853: 48 89 f2                    	mov	rdx, rsi
1409c2856: 41 b1 32                    	mov	r9b, 0x32
1409c2859: e8 a2 b4 f2 ff              	call	0x1408edd00 <.text+0x8ecd00>
1409c285e: 48 8d b5 b0 02 00 00        	lea	rsi, [rbp + 0x2b0]
1409c2865: 48 8d bd b0 07 00 00        	lea	rdi, [rbp + 0x7b0]
1409c286c: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c2872: 48 89 f1                    	mov	rcx, rsi
1409c2875: 48 89 fa                    	mov	rdx, rdi
1409c2878: e8 83 13 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c287d: 48 8d 45 d0                 	lea	rax, [rbp - 0x30]
1409c2881: 48 89 85 e0 0b 00 00        	mov	qword ptr [rbp + 0xbe0], rax
1409c2888: c7 85 b0 07 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x7b0], 0x2
1409c2892: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
1409c2897: 48 8d 8d e0 0b 00 00        	lea	rcx, [rbp + 0xbe0]
1409c289e: 49 b8 16 40 2f 5a b7 bd d7 69       	movabs	r8, 0x69d7bdb75a2f4016
1409c28a8: 48 89 f2                    	mov	rdx, rsi
1409c28ab: 41 b1 19                    	mov	r9b, 0x19
1409c28ae: e8 dd c7 fa ff              	call	0x14096f090 <.text+0x96e090>
1409c28b3: 48 8d b5 b0 07 00 00        	lea	rsi, [rbp + 0x7b0]
1409c28ba: 48 8d bd b0 02 00 00        	lea	rdi, [rbp + 0x2b0]
1409c28c1: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c28c7: 48 89 f1                    	mov	rcx, rsi
1409c28ca: 48 89 fa                    	mov	rdx, rdi
1409c28cd: e8 2e 13 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c28d2: 48 8d 85 80 02 00 00        	lea	rax, [rbp + 0x280]
1409c28d9: 48 89 85 e0 0b 00 00        	mov	qword ptr [rbp + 0xbe0], rax
1409c28e0: c7 85 b0 02 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x2b0], 0x2
1409c28ea: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
1409c28ef: 48 8d 8d e0 0b 00 00        	lea	rcx, [rbp + 0xbe0]
1409c28f6: 49 b8 fa e8 ea 54 3d a4 c7 96       	movabs	r8, -0x69385bc2ab151706
1409c2900: 48 89 f2                    	mov	rdx, rsi
1409c2903: 41 b1 32                    	mov	r9b, 0x32
1409c2906: e8 f5 b3 f2 ff              	call	0x1408edd00 <.text+0x8ecd00>
1409c290b: 48 8d 8d b0 02 00 00        	lea	rcx, [rbp + 0x2b0]
1409c2912: 48 8d 95 b0 07 00 00        	lea	rdx, [rbp + 0x7b0]
1409c2919: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c291f: e8 dc 12 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c2924: 0f 10 85 50 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x350]
1409c292b: 0f 29 85 20 0c 00 00        	movaps	xmmword ptr [rbp + 0xc20], xmm0
1409c2932: 0f 10 85 40 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x340]
1409c2939: 0f 29 85 10 0c 00 00        	movaps	xmmword ptr [rbp + 0xc10], xmm0
1409c2940: 0f 10 85 30 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x330]
1409c2947: 0f 29 85 00 0c 00 00        	movaps	xmmword ptr [rbp + 0xc00], xmm0
1409c294e: 0f 10 85 20 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x320]
1409c2955: 0f 29 85 f0 0b 00 00        	movaps	xmmword ptr [rbp + 0xbf0], xmm0
1409c295c: 0f 10 85 10 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x310]
1409c2963: 0f 29 85 e0 0b 00 00        	movaps	xmmword ptr [rbp + 0xbe0], xmm0
1409c296a: 48 8b 85 e8 03 00 00        	mov	rax, qword ptr [rbp + 0x3e8]
1409c2971: 48 8b 8d 30 04 00 00        	mov	rcx, qword ptr [rbp + 0x430]
1409c2978: 48 8b 95 c0 02 00 00        	mov	rdx, qword ptr [rbp + 0x2c0]
1409c297f: 48 89 95 c0 07 00 00        	mov	qword ptr [rbp + 0x7c0], rdx
1409c2986: 0f 10 85 b0 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2b0]
1409c298d: 0f 29 85 b0 07 00 00        	movaps	xmmword ptr [rbp + 0x7b0], xmm0
1409c2994: 0f 10 85 c8 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2c8]
1409c299b: 0f 29 85 e0 07 00 00        	movaps	xmmword ptr [rbp + 0x7e0], xmm0
1409c29a2: 48 8b 95 d8 02 00 00        	mov	rdx, qword ptr [rbp + 0x2d8]
1409c29a9: 48 89 95 f0 07 00 00        	mov	qword ptr [rbp + 0x7f0], rdx
1409c29b0: 0f 10 85 e0 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2e0]
1409c29b7: 0f 11 85 c8 07 00 00        	movups	xmmword ptr [rbp + 0x7c8], xmm0
1409c29be: 48 8b 95 f0 02 00 00        	mov	rdx, qword ptr [rbp + 0x2f0]
1409c29c5: 48 89 95 d8 07 00 00        	mov	qword ptr [rbp + 0x7d8], rdx
1409c29cc: 0f 10 85 60 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x360]
1409c29d3: 0f 10 8d 70 03 00 00        	movups	xmm1, xmmword ptr [rbp + 0x370]
1409c29da: 0f 10 95 80 03 00 00        	movups	xmm2, xmmword ptr [rbp + 0x380]
1409c29e1: 0f 10 9d 90 03 00 00        	movups	xmm3, xmmword ptr [rbp + 0x390]
1409c29e8: 0f 11 85 28 08 00 00        	movups	xmmword ptr [rbp + 0x828], xmm0
1409c29ef: 0f 11 8d 38 08 00 00        	movups	xmmword ptr [rbp + 0x838], xmm1
1409c29f6: 0f 11 95 48 08 00 00        	movups	xmmword ptr [rbp + 0x848], xmm2
1409c29fd: 0f 11 9d 58 08 00 00        	movups	xmmword ptr [rbp + 0x858], xmm3
1409c2a04: 0f 10 85 a0 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x3a0]
1409c2a0b: 0f 11 85 68 08 00 00        	movups	xmmword ptr [rbp + 0x868], xmm0
1409c2a12: 0f 10 85 b0 03 00 00        	movups	xmm0, xmmword ptr [rbp + 0x3b0]
1409c2a19: 0f 11 85 78 08 00 00        	movups	xmmword ptr [rbp + 0x878], xmm0
1409c2a20: 48 8b 95 08 03 00 00        	mov	rdx, qword ptr [rbp + 0x308]
1409c2a27: 48 89 95 08 08 00 00        	mov	qword ptr [rbp + 0x808], rdx
1409c2a2e: 0f 10 85 f8 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2f8]
1409c2a35: 0f 11 85 f8 07 00 00        	movups	xmmword ptr [rbp + 0x7f8], xmm0
1409c2a3c: 48 89 8d 90 08 00 00        	mov	qword ptr [rbp + 0x890], rcx
1409c2a43: 48 89 85 88 08 00 00        	mov	qword ptr [rbp + 0x888], rax
1409c2a4a: c7 85 10 08 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x810], 0x1
1409c2a54: 44 89 a5 14 08 00 00        	mov	dword ptr [rbp + 0x814], r12d
1409c2a5b: 44 89 b5 18 08 00 00        	mov	dword ptr [rbp + 0x818], r14d
1409c2a62: c7 85 1c 08 00 00 01 00 00 00       	mov	dword ptr [rbp + 0x81c], 0x1
1409c2a6c: 48 8d 8d e0 0b 00 00        	lea	rcx, [rbp + 0xbe0]
1409c2a73: 48 8d 95 b0 07 00 00        	lea	rdx, [rbp + 0x7b0]
1409c2a7a: e8 51 aa 01 00              	call	0x1409dd4d0 <.text+0x9dc4d0>
1409c2a7f: 80 bd e6 03 00 00 08        	cmp	byte ptr [rbp + 0x3e6], 0x8
1409c2a86: 74 0c                       	je	0x1409c2a94 <.text+0x9c1a94>
1409c2a88: 48 8d 8d d0 03 00 00        	lea	rcx, [rbp + 0x3d0]
1409c2a8f: e8 1c fa 21 00              	call	0x140be24b0 <.text+0xbe14b0>
1409c2a94: 90                          	nop
1409c2a95: 48 81 c4 28 0e 00 00        	add	rsp, 0xe28
1409c2a9c: 5b                          	pop	rbx
1409c2a9d: 5f                          	pop	rdi
1409c2a9e: 5e                          	pop	rsi
1409c2a9f: 41 5c                       	pop	r12
1409c2aa1: 41 5d                       	pop	r13
1409c2aa3: 41 5e                       	pop	r14
1409c2aa5: 41 5f                       	pop	r15
1409c2aa7: 5d                          	pop	rbp
1409c2aa8: c3                          	ret
1409c2aa9: c6 85 9f 0d 00 00 01        	mov	byte ptr [rbp + 0xd9f], 0x1
1409c2ab0: 48 8d 0d e9 7f 27 02        	lea	rcx, [rip + 0x2277fe9]  # 0x142c3aaa0
1409c2ab7: e8 14 56 f1 01              	call	0x1428d80d0 <.text+0x28d70d0>
1409c2abc: e9 c7 00 00 00              	jmp	0x1409c2b88 <.text+0x9c1b88>
1409c2ac1: c6 85 9f 0d 00 00 01        	mov	byte ptr [rbp + 0xd9f], 0x1
1409c2ac8: 48 8d 0d 09 31 0e 02        	lea	rcx, [rip + 0x20e3109]  # 0x142aa5bd8
1409c2acf: e8 3c 68 f1 01              	call	0x1428d9310 <.text+0x28d8310>
1409c2ad4: e9 af 00 00 00              	jmp	0x1409c2b88 <.text+0x9c1b88>
1409c2ad9: 48 8b 85 b8 07 00 00        	mov	rax, qword ptr [rbp + 0x7b8]
1409c2ae0: 48 89 85 b0 02 00 00        	mov	qword ptr [rbp + 0x2b0], rax
1409c2ae7: 48 8d 05 82 d2 0f 02        	lea	rax, [rip + 0x20fd282]  # 0x142abfd70
1409c2aee: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1409c2af3: 48 8d 0d d6 ce 0f 02        	lea	rcx, [rip + 0x20fced6]  # 0x142abf9d0
1409c2afa: 4c 8d 0d af ce 0f 02        	lea	r9, [rip + 0x20fceaf]   # 0x142abf9b0
1409c2b01: 4c 8d 85 b0 02 00 00        	lea	r8, [rbp + 0x2b0]
1409c2b08: ba 2b 00 00 00              	mov	edx, 0x2b
1409c2b0d: e8 4e 6c f1 01              	call	0x1428d9760 <.text+0x28d8760>
1409c2b12: eb 74                       	jmp	0x1409c2b88 <.text+0x9c1b88>
1409c2b14: 48 8b 85 b8 07 00 00        	mov	rax, qword ptr [rbp + 0x7b8]
1409c2b1b: 48 89 85 b0 02 00 00        	mov	qword ptr [rbp + 0x2b0], rax
1409c2b22: 48 8d 05 ff d2 0f 02        	lea	rax, [rip + 0x20fd2ff]  # 0x142abfe28
1409c2b29: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1409c2b2e: 48 8d 0d 9b ce 0f 02        	lea	rcx, [rip + 0x20fce9b]  # 0x142abf9d0
1409c2b35: 4c 8d 0d 74 ce 0f 02        	lea	r9, [rip + 0x20fce74]   # 0x142abf9b0
1409c2b3c: 4c 8d 85 b0 02 00 00        	lea	r8, [rbp + 0x2b0]
1409c2b43: ba 2b 00 00 00              	mov	edx, 0x2b
1409c2b48: e8 13 6c f1 01              	call	0x1428d9760 <.text+0x28d8760>
1409c2b4d: eb 39                       	jmp	0x1409c2b88 <.text+0x9c1b88>
1409c2b4f: 48 8b 85 b8 07 00 00        	mov	rax, qword ptr [rbp + 0x7b8]
1409c2b56: 48 89 85 b0 02 00 00        	mov	qword ptr [rbp + 0x2b0], rax
1409c2b5d: 48 8d 05 ec d2 0f 02        	lea	rax, [rip + 0x20fd2ec]  # 0x142abfe50
1409c2b64: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
1409c2b69: 48 8d 0d 60 ce 0f 02        	lea	rcx, [rip + 0x20fce60]  # 0x142abf9d0
1409c2b70: 4c 8d 0d 39 ce 0f 02        	lea	r9, [rip + 0x20fce39]   # 0x142abf9b0
1409c2b77: 4c 8d 85 b0 02 00 00        	lea	r8, [rbp + 0x2b0]
1409c2b7e: ba 2b 00 00 00              	mov	edx, 0x2b
1409c2b83: e8 d8 6b f1 01              	call	0x1428d9760 <.text+0x28d8760>
1409c2b88: 0f 0b                       	ud2
1409c2b8a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c2b90: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c2b95: 55                          	push	rbp
1409c2b96: 41 57                       	push	r15
1409c2b98: 41 56                       	push	r14
1409c2b9a: 41 55                       	push	r13
1409c2b9c: 41 54                       	push	r12
1409c2b9e: 56                          	push	rsi
1409c2b9f: 57                          	push	rdi
1409c2ba0: 53                          	push	rbx
1409c2ba1: 48 83 ec 48                 	sub	rsp, 0x48
1409c2ba5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c2bac: 48 8d 8d b0 07 00 00        	lea	rcx, [rbp + 0x7b0]
1409c2bb3: e8 f8 43 ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c2bb8: 90                          	nop
1409c2bb9: 48 83 c4 48                 	add	rsp, 0x48
1409c2bbd: 5b                          	pop	rbx
1409c2bbe: 5f                          	pop	rdi
1409c2bbf: 5e                          	pop	rsi
1409c2bc0: 41 5c                       	pop	r12
1409c2bc2: 41 5d                       	pop	r13
1409c2bc4: 41 5e                       	pop	r14
1409c2bc6: 41 5f                       	pop	r15
1409c2bc8: 5d                          	pop	rbp
1409c2bc9: c3                          	ret
1409c2bca: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c2bd0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c2bd5: 55                          	push	rbp
1409c2bd6: 41 57                       	push	r15
1409c2bd8: 41 56                       	push	r14
1409c2bda: 41 55                       	push	r13
1409c2bdc: 41 54                       	push	r12
1409c2bde: 56                          	push	rsi
1409c2bdf: 57                          	push	rdi
1409c2be0: 53                          	push	rbx
1409c2be1: 48 83 ec 48                 	sub	rsp, 0x48
1409c2be5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c2bec: 48 8d 8d b0 02 00 00        	lea	rcx, [rbp + 0x2b0]
1409c2bf3: e8 b8 43 ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c2bf8: 90                          	nop
1409c2bf9: 48 83 c4 48                 	add	rsp, 0x48
1409c2bfd: 5b                          	pop	rbx
1409c2bfe: 5f                          	pop	rdi
1409c2bff: 5e                          	pop	rsi
1409c2c00: 41 5c                       	pop	r12
1409c2c02: 41 5d                       	pop	r13
1409c2c04: 41 5e                       	pop	r14
1409c2c06: 41 5f                       	pop	r15
1409c2c08: 5d                          	pop	rbp
1409c2c09: c3                          	ret
1409c2c0a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c2c10: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c2c15: 55                          	push	rbp
1409c2c16: 41 57                       	push	r15
1409c2c18: 41 56                       	push	r14
1409c2c1a: 41 55                       	push	r13
1409c2c1c: 41 54                       	push	r12
1409c2c1e: 56                          	push	rsi
1409c2c1f: 57                          	push	rdi
1409c2c20: 53                          	push	rbx
1409c2c21: 48 83 ec 48                 	sub	rsp, 0x48
1409c2c25: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c2c2c: 48 8d 8d b0 07 00 00        	lea	rcx, [rbp + 0x7b0]
1409c2c33: e8 78 43 ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c2c38: 90                          	nop
1409c2c39: 48 83 c4 48                 	add	rsp, 0x48
1409c2c3d: 5b                          	pop	rbx
1409c2c3e: 5f                          	pop	rdi
1409c2c3f: 5e                          	pop	rsi
1409c2c40: 41 5c                       	pop	r12
1409c2c42: 41 5d                       	pop	r13
1409c2c44: 41 5e                       	pop	r14
1409c2c46: 41 5f                       	pop	r15
1409c2c48: 5d                          	pop	rbp
1409c2c49: c3                          	ret
1409c2c4a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c2c50: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c2c55: 55                          	push	rbp
1409c2c56: 41 57                       	push	r15
1409c2c58: 41 56                       	push	r14
1409c2c5a: 41 55                       	push	r13
1409c2c5c: 41 54                       	push	r12
1409c2c5e: 56                          	push	rsi
1409c2c5f: 57                          	push	rdi
1409c2c60: 53                          	push	rbx
1409c2c61: 48 83 ec 48                 	sub	rsp, 0x48
1409c2c65: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c2c6c: 48 8d 8d b0 02 00 00        	lea	rcx, [rbp + 0x2b0]
1409c2c73: e8 38 43 ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c2c78: 90                          	nop
1409c2c79: 48 83 c4 48                 	add	rsp, 0x48
1409c2c7d: 5b                          	pop	rbx
1409c2c7e: 5f                          	pop	rdi
1409c2c7f: 5e                          	pop	rsi
1409c2c80: 41 5c                       	pop	r12
1409c2c82: 41 5d                       	pop	r13
1409c2c84: 41 5e                       	pop	r14
1409c2c86: 41 5f                       	pop	r15
1409c2c88: 5d                          	pop	rbp
1409c2c89: c3                          	ret
1409c2c8a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c2c90: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c2c95: 55                          	push	rbp
1409c2c96: 41 57                       	push	r15
1409c2c98: 41 56                       	push	r14
1409c2c9a: 41 55                       	push	r13
1409c2c9c: 41 54                       	push	r12
1409c2c9e: 56                          	push	rsi
1409c2c9f: 57                          	push	rdi
1409c2ca0: 53                          	push	rbx
1409c2ca1: 48 83 ec 48                 	sub	rsp, 0x48
1409c2ca5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c2cac: 48 8d 8d b0 07 00 00        	lea	rcx, [rbp + 0x7b0]
1409c2cb3: e8 f8 42 ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c2cb8: 90                          	nop
1409c2cb9: 48 83 c4 48                 	add	rsp, 0x48
1409c2cbd: 5b                          	pop	rbx
1409c2cbe: 5f                          	pop	rdi
1409c2cbf: 5e                          	pop	rsi
1409c2cc0: 41 5c                       	pop	r12
1409c2cc2: 41 5d                       	pop	r13
1409c2cc4: 41 5e                       	pop	r14
1409c2cc6: 41 5f                       	pop	r15
1409c2cc8: 5d                          	pop	rbp
1409c2cc9: c3                          	ret
1409c2cca: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c2cd0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c2cd5: 55                          	push	rbp
1409c2cd6: 41 57                       	push	r15
1409c2cd8: 41 56                       	push	r14
1409c2cda: 41 55                       	push	r13
1409c2cdc: 41 54                       	push	r12
1409c2cde: 56                          	push	rsi
1409c2cdf: 57                          	push	rdi
1409c2ce0: 53                          	push	rbx
1409c2ce1: 48 83 ec 48                 	sub	rsp, 0x48
1409c2ce5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c2cec: 48 8d 8d b0 02 00 00        	lea	rcx, [rbp + 0x2b0]
1409c2cf3: e8 b8 42 ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c2cf8: 90                          	nop
1409c2cf9: 48 83 c4 48                 	add	rsp, 0x48
1409c2cfd: 5b                          	pop	rbx
1409c2cfe: 5f                          	pop	rdi
1409c2cff: 5e                          	pop	rsi
1409c2d00: 41 5c                       	pop	r12
1409c2d02: 41 5d                       	pop	r13
1409c2d04: 41 5e                       	pop	r14
1409c2d06: 41 5f                       	pop	r15
1409c2d08: 5d                          	pop	rbp
1409c2d09: c3                          	ret
1409c2d0a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c2d10: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c2d15: 55                          	push	rbp
1409c2d16: 41 57                       	push	r15
1409c2d18: 41 56                       	push	r14
1409c2d1a: 41 55                       	push	r13
1409c2d1c: 41 54                       	push	r12
1409c2d1e: 56                          	push	rsi
1409c2d1f: 57                          	push	rdi
1409c2d20: 53                          	push	rbx
1409c2d21: 48 83 ec 48                 	sub	rsp, 0x48
1409c2d25: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c2d2c: 48 8d 8d e0 0b 00 00        	lea	rcx, [rbp + 0xbe0]
1409c2d33: e8 78 42 ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c2d38: 90                          	nop
1409c2d39: 48 83 c4 48                 	add	rsp, 0x48
1409c2d3d: 5b                          	pop	rbx
1409c2d3e: 5f                          	pop	rdi
1409c2d3f: 5e                          	pop	rsi
1409c2d40: 41 5c                       	pop	r12
1409c2d42: 41 5d                       	pop	r13
1409c2d44: 41 5e                       	pop	r14
1409c2d46: 41 5f                       	pop	r15
1409c2d48: 5d                          	pop	rbp
1409c2d49: c3                          	ret
1409c2d4a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c2d50: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c2d55: 55                          	push	rbp
1409c2d56: 41 57                       	push	r15
1409c2d58: 41 56                       	push	r14
1409c2d5a: 41 55                       	push	r13
1409c2d5c: 41 54                       	push	r12
1409c2d5e: 56                          	push	rsi
1409c2d5f: 57                          	push	rdi
1409c2d60: 53                          	push	rbx
1409c2d61: 48 83 ec 48                 	sub	rsp, 0x48
1409c2d65: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c2d6c: 48 8d 8d b0 07 00 00        	lea	rcx, [rbp + 0x7b0]
1409c2d73: e8 38 42 ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c2d78: 90                          	nop
1409c2d79: 48 83 c4 48                 	add	rsp, 0x48
1409c2d7d: 5b                          	pop	rbx
1409c2d7e: 5f                          	pop	rdi
1409c2d7f: 5e                          	pop	rsi
1409c2d80: 41 5c                       	pop	r12
1409c2d82: 41 5d                       	pop	r13
1409c2d84: 41 5e                       	pop	r14
1409c2d86: 41 5f                       	pop	r15
1409c2d88: 5d                          	pop	rbp
1409c2d89: c3                          	ret
1409c2d8a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c2d90: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c2d95: 55                          	push	rbp
1409c2d96: 41 57                       	push	r15
1409c2d98: 41 56                       	push	r14
1409c2d9a: 41 55                       	push	r13
1409c2d9c: 41 54                       	push	r12
1409c2d9e: 56                          	push	rsi
1409c2d9f: 57                          	push	rdi
1409c2da0: 53                          	push	rbx
1409c2da1: 48 83 ec 48                 	sub	rsp, 0x48
1409c2da5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c2dac: 48 8d 8d b0 02 00 00        	lea	rcx, [rbp + 0x2b0]
1409c2db3: e8 f8 41 ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c2db8: 90                          	nop
1409c2db9: 48 83 c4 48                 	add	rsp, 0x48
1409c2dbd: 5b                          	pop	rbx
1409c2dbe: 5f                          	pop	rdi
1409c2dbf: 5e                          	pop	rsi
1409c2dc0: 41 5c                       	pop	r12
1409c2dc2: 41 5d                       	pop	r13
1409c2dc4: 41 5e                       	pop	r14
1409c2dc6: 41 5f                       	pop	r15
1409c2dc8: 5d                          	pop	rbp
1409c2dc9: c3                          	ret
1409c2dca: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c2dd0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c2dd5: 55                          	push	rbp
1409c2dd6: 41 57                       	push	r15
1409c2dd8: 41 56                       	push	r14
1409c2dda: 41 55                       	push	r13
1409c2ddc: 41 54                       	push	r12
1409c2dde: 56                          	push	rsi
1409c2ddf: 57                          	push	rdi
1409c2de0: 53                          	push	rbx
1409c2de1: 48 83 ec 48                 	sub	rsp, 0x48
1409c2de5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c2dec: 48 8d 8d b0 07 00 00        	lea	rcx, [rbp + 0x7b0]
1409c2df3: e8 b8 41 ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c2df8: 90                          	nop
1409c2df9: 48 83 c4 48                 	add	rsp, 0x48
1409c2dfd: 5b                          	pop	rbx
1409c2dfe: 5f                          	pop	rdi
1409c2dff: 5e                          	pop	rsi
1409c2e00: 41 5c                       	pop	r12
1409c2e02: 41 5d                       	pop	r13
1409c2e04: 41 5e                       	pop	r14
1409c2e06: 41 5f                       	pop	r15
1409c2e08: 5d                          	pop	rbp
1409c2e09: c3                          	ret
1409c2e0a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c2e10: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c2e15: 55                          	push	rbp
1409c2e16: 41 57                       	push	r15
1409c2e18: 41 56                       	push	r14
1409c2e1a: 41 55                       	push	r13
1409c2e1c: 41 54                       	push	r12
1409c2e1e: 56                          	push	rsi
1409c2e1f: 57                          	push	rdi
1409c2e20: 53                          	push	rbx
1409c2e21: 48 83 ec 48                 	sub	rsp, 0x48
1409c2e25: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c2e2c: 48 8d 8d b0 07 00 00        	lea	rcx, [rbp + 0x7b0]
1409c2e33: e8 78 41 ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c2e38: 90                          	nop
1409c2e39: 48 83 c4 48                 	add	rsp, 0x48
1409c2e3d: 5b                          	pop	rbx
1409c2e3e: 5f                          	pop	rdi
1409c2e3f: 5e                          	pop	rsi
1409c2e40: 41 5c                       	pop	r12
1409c2e42: 41 5d                       	pop	r13
1409c2e44: 41 5e                       	pop	r14
1409c2e46: 41 5f                       	pop	r15
1409c2e48: 5d                          	pop	rbp
1409c2e49: c3                          	ret
1409c2e4a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c2e50: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c2e55: 55                          	push	rbp
1409c2e56: 41 57                       	push	r15
1409c2e58: 41 56                       	push	r14
1409c2e5a: 41 55                       	push	r13
1409c2e5c: 41 54                       	push	r12
1409c2e5e: 56                          	push	rsi
1409c2e5f: 57                          	push	rdi
1409c2e60: 53                          	push	rbx
1409c2e61: 48 83 ec 48                 	sub	rsp, 0x48
1409c2e65: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c2e6c: 48 8d 8d b0 02 00 00        	lea	rcx, [rbp + 0x2b0]
1409c2e73: e8 38 41 ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c2e78: 90                          	nop
1409c2e79: 48 83 c4 48                 	add	rsp, 0x48
1409c2e7d: 5b                          	pop	rbx
1409c2e7e: 5f                          	pop	rdi
1409c2e7f: 5e                          	pop	rsi
1409c2e80: 41 5c                       	pop	r12
1409c2e82: 41 5d                       	pop	r13
1409c2e84: 41 5e                       	pop	r14
1409c2e86: 41 5f                       	pop	r15
1409c2e88: 5d                          	pop	rbp
1409c2e89: c3                          	ret
1409c2e8a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c2e90: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c2e95: 55                          	push	rbp
1409c2e96: 41 57                       	push	r15
1409c2e98: 41 56                       	push	r14
1409c2e9a: 41 55                       	push	r13
1409c2e9c: 41 54                       	push	r12
1409c2e9e: 56                          	push	rsi
1409c2e9f: 57                          	push	rdi
1409c2ea0: 53                          	push	rbx
1409c2ea1: 48 83 ec 48                 	sub	rsp, 0x48
1409c2ea5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c2eac: 48 8d 8d b0 07 00 00        	lea	rcx, [rbp + 0x7b0]
1409c2eb3: e8 f8 40 ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c2eb8: 90                          	nop
1409c2eb9: 48 83 c4 48                 	add	rsp, 0x48
1409c2ebd: 5b                          	pop	rbx
1409c2ebe: 5f                          	pop	rdi
1409c2ebf: 5e                          	pop	rsi
1409c2ec0: 41 5c                       	pop	r12
1409c2ec2: 41 5d                       	pop	r13
1409c2ec4: 41 5e                       	pop	r14
1409c2ec6: 41 5f                       	pop	r15
1409c2ec8: 5d                          	pop	rbp
1409c2ec9: c3                          	ret
1409c2eca: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c2ed0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c2ed5: 55                          	push	rbp
1409c2ed6: 41 57                       	push	r15
1409c2ed8: 41 56                       	push	r14
1409c2eda: 41 55                       	push	r13
1409c2edc: 41 54                       	push	r12
1409c2ede: 56                          	push	rsi
1409c2edf: 57                          	push	rdi
1409c2ee0: 53                          	push	rbx
1409c2ee1: 48 83 ec 48                 	sub	rsp, 0x48
1409c2ee5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c2eec: 48 8d 8d b0 02 00 00        	lea	rcx, [rbp + 0x2b0]
1409c2ef3: e8 b8 40 ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c2ef8: 90                          	nop
1409c2ef9: 48 83 c4 48                 	add	rsp, 0x48
1409c2efd: 5b                          	pop	rbx
1409c2efe: 5f                          	pop	rdi
1409c2eff: 5e                          	pop	rsi
1409c2f00: 41 5c                       	pop	r12
1409c2f02: 41 5d                       	pop	r13
1409c2f04: 41 5e                       	pop	r14
1409c2f06: 41 5f                       	pop	r15
1409c2f08: 5d                          	pop	rbp
1409c2f09: c3                          	ret
1409c2f0a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c2f10: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c2f15: 55                          	push	rbp
1409c2f16: 41 57                       	push	r15
1409c2f18: 41 56                       	push	r14
1409c2f1a: 41 55                       	push	r13
1409c2f1c: 41 54                       	push	r12
1409c2f1e: 56                          	push	rsi
1409c2f1f: 57                          	push	rdi
1409c2f20: 53                          	push	rbx
1409c2f21: 48 83 ec 48                 	sub	rsp, 0x48
1409c2f25: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c2f2c: 48 8d 8d b0 07 00 00        	lea	rcx, [rbp + 0x7b0]
1409c2f33: e8 78 40 ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c2f38: 90                          	nop
1409c2f39: 48 83 c4 48                 	add	rsp, 0x48
1409c2f3d: 5b                          	pop	rbx
1409c2f3e: 5f                          	pop	rdi
1409c2f3f: 5e                          	pop	rsi
1409c2f40: 41 5c                       	pop	r12
1409c2f42: 41 5d                       	pop	r13
1409c2f44: 41 5e                       	pop	r14
1409c2f46: 41 5f                       	pop	r15
1409c2f48: 5d                          	pop	rbp
1409c2f49: c3                          	ret
1409c2f4a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c2f50: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c2f55: 55                          	push	rbp
1409c2f56: 41 57                       	push	r15
1409c2f58: 41 56                       	push	r14
1409c2f5a: 41 55                       	push	r13
1409c2f5c: 41 54                       	push	r12
1409c2f5e: 56                          	push	rsi
1409c2f5f: 57                          	push	rdi
1409c2f60: 53                          	push	rbx
1409c2f61: 48 83 ec 48                 	sub	rsp, 0x48
1409c2f65: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c2f6c: 48 8d 8d b0 02 00 00        	lea	rcx, [rbp + 0x2b0]
1409c2f73: e8 38 40 ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c2f78: 90                          	nop
1409c2f79: 48 83 c4 48                 	add	rsp, 0x48
1409c2f7d: 5b                          	pop	rbx
1409c2f7e: 5f                          	pop	rdi
1409c2f7f: 5e                          	pop	rsi
1409c2f80: 41 5c                       	pop	r12
1409c2f82: 41 5d                       	pop	r13
1409c2f84: 41 5e                       	pop	r14
1409c2f86: 41 5f                       	pop	r15
1409c2f88: 5d                          	pop	rbp
1409c2f89: c3                          	ret
1409c2f8a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c2f90: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c2f95: 55                          	push	rbp
1409c2f96: 41 57                       	push	r15
1409c2f98: 41 56                       	push	r14
1409c2f9a: 41 55                       	push	r13
1409c2f9c: 41 54                       	push	r12
1409c2f9e: 56                          	push	rsi
1409c2f9f: 57                          	push	rdi
1409c2fa0: 53                          	push	rbx
1409c2fa1: 48 83 ec 48                 	sub	rsp, 0x48
1409c2fa5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c2fac: 48 8d 8d b0 07 00 00        	lea	rcx, [rbp + 0x7b0]
1409c2fb3: e8 f8 3f ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c2fb8: 90                          	nop
1409c2fb9: 48 83 c4 48                 	add	rsp, 0x48
1409c2fbd: 5b                          	pop	rbx
1409c2fbe: 5f                          	pop	rdi
1409c2fbf: 5e                          	pop	rsi
1409c2fc0: 41 5c                       	pop	r12
1409c2fc2: 41 5d                       	pop	r13
1409c2fc4: 41 5e                       	pop	r14
1409c2fc6: 41 5f                       	pop	r15
1409c2fc8: 5d                          	pop	rbp
1409c2fc9: c3                          	ret
1409c2fca: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c2fd0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c2fd5: 55                          	push	rbp
1409c2fd6: 41 57                       	push	r15
1409c2fd8: 41 56                       	push	r14
1409c2fda: 41 55                       	push	r13
1409c2fdc: 41 54                       	push	r12
1409c2fde: 56                          	push	rsi
1409c2fdf: 57                          	push	rdi
1409c2fe0: 53                          	push	rbx
1409c2fe1: 48 83 ec 48                 	sub	rsp, 0x48
1409c2fe5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c2fec: 48 8d 8d b0 02 00 00        	lea	rcx, [rbp + 0x2b0]
1409c2ff3: e8 b8 3f ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c2ff8: 90                          	nop
1409c2ff9: 48 83 c4 48                 	add	rsp, 0x48
1409c2ffd: 5b                          	pop	rbx
1409c2ffe: 5f                          	pop	rdi
1409c2fff: 5e                          	pop	rsi
1409c3000: 41 5c                       	pop	r12
1409c3002: 41 5d                       	pop	r13
1409c3004: 41 5e                       	pop	r14
1409c3006: 41 5f                       	pop	r15
1409c3008: 5d                          	pop	rbp
1409c3009: c3                          	ret
1409c300a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c3010: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c3015: 55                          	push	rbp
1409c3016: 41 57                       	push	r15
1409c3018: 41 56                       	push	r14
1409c301a: 41 55                       	push	r13
1409c301c: 41 54                       	push	r12
1409c301e: 56                          	push	rsi
1409c301f: 57                          	push	rdi
1409c3020: 53                          	push	rbx
1409c3021: 48 83 ec 48                 	sub	rsp, 0x48
1409c3025: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c302c: 48 8d 8d b0 07 00 00        	lea	rcx, [rbp + 0x7b0]
1409c3033: e8 78 3f ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c3038: 90                          	nop
1409c3039: 48 83 c4 48                 	add	rsp, 0x48
1409c303d: 5b                          	pop	rbx
1409c303e: 5f                          	pop	rdi
1409c303f: 5e                          	pop	rsi
1409c3040: 41 5c                       	pop	r12
1409c3042: 41 5d                       	pop	r13
1409c3044: 41 5e                       	pop	r14
1409c3046: 41 5f                       	pop	r15
1409c3048: 5d                          	pop	rbp
1409c3049: c3                          	ret
1409c304a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c3050: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c3055: 55                          	push	rbp
1409c3056: 41 57                       	push	r15
1409c3058: 41 56                       	push	r14
1409c305a: 41 55                       	push	r13
1409c305c: 41 54                       	push	r12
1409c305e: 56                          	push	rsi
1409c305f: 57                          	push	rdi
1409c3060: 53                          	push	rbx
1409c3061: 48 83 ec 48                 	sub	rsp, 0x48
1409c3065: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c306c: 48 8d 8d e0 0b 00 00        	lea	rcx, [rbp + 0xbe0]
1409c3073: e8 38 3f ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c3078: 90                          	nop
1409c3079: 48 83 c4 48                 	add	rsp, 0x48
1409c307d: 5b                          	pop	rbx
1409c307e: 5f                          	pop	rdi
1409c307f: 5e                          	pop	rsi
1409c3080: 41 5c                       	pop	r12
1409c3082: 41 5d                       	pop	r13
1409c3084: 41 5e                       	pop	r14
1409c3086: 41 5f                       	pop	r15
1409c3088: 5d                          	pop	rbp
1409c3089: c3                          	ret
1409c308a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c3090: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c3095: 55                          	push	rbp
1409c3096: 41 57                       	push	r15
1409c3098: 41 56                       	push	r14
1409c309a: 41 55                       	push	r13
1409c309c: 41 54                       	push	r12
1409c309e: 56                          	push	rsi
1409c309f: 57                          	push	rdi
1409c30a0: 53                          	push	rbx
1409c30a1: 48 83 ec 48                 	sub	rsp, 0x48
1409c30a5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c30ac: 48 8d 8d b0 07 00 00        	lea	rcx, [rbp + 0x7b0]
1409c30b3: e8 f8 3e ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c30b8: 90                          	nop
1409c30b9: 48 83 c4 48                 	add	rsp, 0x48
1409c30bd: 5b                          	pop	rbx
1409c30be: 5f                          	pop	rdi
1409c30bf: 5e                          	pop	rsi
1409c30c0: 41 5c                       	pop	r12
1409c30c2: 41 5d                       	pop	r13
1409c30c4: 41 5e                       	pop	r14
1409c30c6: 41 5f                       	pop	r15
1409c30c8: 5d                          	pop	rbp
1409c30c9: c3                          	ret
1409c30ca: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c30d0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c30d5: 55                          	push	rbp
1409c30d6: 41 57                       	push	r15
1409c30d8: 41 56                       	push	r14
1409c30da: 41 55                       	push	r13
1409c30dc: 41 54                       	push	r12
1409c30de: 56                          	push	rsi
1409c30df: 57                          	push	rdi
1409c30e0: 53                          	push	rbx
1409c30e1: 48 83 ec 48                 	sub	rsp, 0x48
1409c30e5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c30ec: 48 8d 8d b0 07 00 00        	lea	rcx, [rbp + 0x7b0]
1409c30f3: e8 b8 3e ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c30f8: 90                          	nop
1409c30f9: 48 83 c4 48                 	add	rsp, 0x48
1409c30fd: 5b                          	pop	rbx
1409c30fe: 5f                          	pop	rdi
1409c30ff: 5e                          	pop	rsi
1409c3100: 41 5c                       	pop	r12
1409c3102: 41 5d                       	pop	r13
1409c3104: 41 5e                       	pop	r14
1409c3106: 41 5f                       	pop	r15
1409c3108: 5d                          	pop	rbp
1409c3109: c3                          	ret
1409c310a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c3110: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c3115: 55                          	push	rbp
1409c3116: 41 57                       	push	r15
1409c3118: 41 56                       	push	r14
1409c311a: 41 55                       	push	r13
1409c311c: 41 54                       	push	r12
1409c311e: 56                          	push	rsi
1409c311f: 57                          	push	rdi
1409c3120: 53                          	push	rbx
1409c3121: 48 83 ec 48                 	sub	rsp, 0x48
1409c3125: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c312c: 48 8d 8d b0 02 00 00        	lea	rcx, [rbp + 0x2b0]
1409c3133: e8 78 3e ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c3138: 90                          	nop
1409c3139: 48 83 c4 48                 	add	rsp, 0x48
1409c313d: 5b                          	pop	rbx
1409c313e: 5f                          	pop	rdi
1409c313f: 5e                          	pop	rsi
1409c3140: 41 5c                       	pop	r12
1409c3142: 41 5d                       	pop	r13
1409c3144: 41 5e                       	pop	r14
1409c3146: 41 5f                       	pop	r15
1409c3148: 5d                          	pop	rbp
1409c3149: c3                          	ret
1409c314a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c3150: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c3155: 55                          	push	rbp
1409c3156: 41 57                       	push	r15
1409c3158: 41 56                       	push	r14
1409c315a: 41 55                       	push	r13
1409c315c: 41 54                       	push	r12
1409c315e: 56                          	push	rsi
1409c315f: 57                          	push	rdi
1409c3160: 53                          	push	rbx
1409c3161: 48 83 ec 48                 	sub	rsp, 0x48
1409c3165: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c316c: 48 8d 8d b0 02 00 00        	lea	rcx, [rbp + 0x2b0]
1409c3173: e8 b8 9a 38 00              	call	0x140d4cc30 <.text+0xd4bc30>
1409c3178: 90                          	nop
1409c3179: 48 83 c4 48                 	add	rsp, 0x48
1409c317d: 5b                          	pop	rbx
1409c317e: 5f                          	pop	rdi
1409c317f: 5e                          	pop	rsi
1409c3180: 41 5c                       	pop	r12
1409c3182: 41 5d                       	pop	r13
1409c3184: 41 5e                       	pop	r14
1409c3186: 41 5f                       	pop	r15
1409c3188: 5d                          	pop	rbp
1409c3189: c3                          	ret
1409c318a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c3190: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c3195: 55                          	push	rbp
1409c3196: 41 57                       	push	r15
1409c3198: 41 56                       	push	r14
1409c319a: 41 55                       	push	r13
1409c319c: 41 54                       	push	r12
1409c319e: 56                          	push	rsi
1409c319f: 57                          	push	rdi
1409c31a0: 53                          	push	rbx
1409c31a1: 48 83 ec 48                 	sub	rsp, 0x48
1409c31a5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c31ac: 48 8d 8d b0 07 00 00        	lea	rcx, [rbp + 0x7b0]
1409c31b3: e8 f8 3d ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c31b8: 90                          	nop
1409c31b9: 48 83 c4 48                 	add	rsp, 0x48
1409c31bd: 5b                          	pop	rbx
1409c31be: 5f                          	pop	rdi
1409c31bf: 5e                          	pop	rsi
1409c31c0: 41 5c                       	pop	r12
1409c31c2: 41 5d                       	pop	r13
1409c31c4: 41 5e                       	pop	r14
1409c31c6: 41 5f                       	pop	r15
1409c31c8: 5d                          	pop	rbp
1409c31c9: c3                          	ret
1409c31ca: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c31d0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c31d5: 55                          	push	rbp
1409c31d6: 41 57                       	push	r15
1409c31d8: 41 56                       	push	r14
1409c31da: 41 55                       	push	r13
1409c31dc: 41 54                       	push	r12
1409c31de: 56                          	push	rsi
1409c31df: 57                          	push	rdi
1409c31e0: 53                          	push	rbx
1409c31e1: 48 83 ec 48                 	sub	rsp, 0x48
1409c31e5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c31ec: 80 bd e6 08 00 00 08        	cmp	byte ptr [rbp + 0x8e6], 0x8
1409c31f3: 74 0c                       	je	0x1409c3201 <.text+0x9c2201>
1409c31f5: 48 8d 8d d0 08 00 00        	lea	rcx, [rbp + 0x8d0]
1409c31fc: e8 af f2 21 00              	call	0x140be24b0 <.text+0xbe14b0>
1409c3201: 90                          	nop
1409c3202: 48 83 c4 48                 	add	rsp, 0x48
1409c3206: 5b                          	pop	rbx
1409c3207: 5f                          	pop	rdi
1409c3208: 5e                          	pop	rsi
1409c3209: 41 5c                       	pop	r12
1409c320b: 41 5d                       	pop	r13
1409c320d: 41 5e                       	pop	r14
1409c320f: 41 5f                       	pop	r15
1409c3211: 5d                          	pop	rbp
1409c3212: c3                          	ret
1409c3213: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
1409c3220: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c3225: 55                          	push	rbp
1409c3226: 41 57                       	push	r15
1409c3228: 41 56                       	push	r14
1409c322a: 41 55                       	push	r13
1409c322c: 41 54                       	push	r12
1409c322e: 56                          	push	rsi
1409c322f: 57                          	push	rdi
1409c3230: 53                          	push	rbx
1409c3231: 48 83 ec 48                 	sub	rsp, 0x48
1409c3235: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c323c: 48 8d 8d b0 07 00 00        	lea	rcx, [rbp + 0x7b0]
1409c3243: e8 68 3d ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c3248: 90                          	nop
1409c3249: 48 83 c4 48                 	add	rsp, 0x48
1409c324d: 5b                          	pop	rbx
1409c324e: 5f                          	pop	rdi
1409c324f: 5e                          	pop	rsi
1409c3250: 41 5c                       	pop	r12
1409c3252: 41 5d                       	pop	r13
1409c3254: 41 5e                       	pop	r14
1409c3256: 41 5f                       	pop	r15
1409c3258: 5d                          	pop	rbp
1409c3259: c3                          	ret
1409c325a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c3260: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c3265: 55                          	push	rbp
1409c3266: 41 57                       	push	r15
1409c3268: 41 56                       	push	r14
1409c326a: 41 55                       	push	r13
1409c326c: 41 54                       	push	r12
1409c326e: 56                          	push	rsi
1409c326f: 57                          	push	rdi
1409c3270: 53                          	push	rbx
1409c3271: 48 83 ec 48                 	sub	rsp, 0x48
1409c3275: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c327c: 48 8d 8d b0 02 00 00        	lea	rcx, [rbp + 0x2b0]
1409c3283: e8 28 3d ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c3288: 90                          	nop
1409c3289: 48 83 c4 48                 	add	rsp, 0x48
1409c328d: 5b                          	pop	rbx
1409c328e: 5f                          	pop	rdi
1409c328f: 5e                          	pop	rsi
1409c3290: 41 5c                       	pop	r12
1409c3292: 41 5d                       	pop	r13
1409c3294: 41 5e                       	pop	r14
1409c3296: 41 5f                       	pop	r15
1409c3298: 5d                          	pop	rbp
1409c3299: c3                          	ret
1409c329a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c32a0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c32a5: 55                          	push	rbp
1409c32a6: 41 57                       	push	r15
1409c32a8: 41 56                       	push	r14
1409c32aa: 41 55                       	push	r13
1409c32ac: 41 54                       	push	r12
1409c32ae: 56                          	push	rsi
1409c32af: 57                          	push	rdi
1409c32b0: 53                          	push	rbx
1409c32b1: 48 83 ec 48                 	sub	rsp, 0x48
1409c32b5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c32bc: c6 85 9d 0d 00 00 00        	mov	byte ptr [rbp + 0xd9d], 0x0
1409c32c3: 48 8d 8d b0 07 00 00        	lea	rcx, [rbp + 0x7b0]
1409c32ca: e8 e1 3c ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c32cf: c6 85 9d 0d 00 00 00        	mov	byte ptr [rbp + 0xd9d], 0x0
1409c32d6: 48 83 c4 48                 	add	rsp, 0x48
1409c32da: 5b                          	pop	rbx
1409c32db: 5f                          	pop	rdi
1409c32dc: 5e                          	pop	rsi
1409c32dd: 41 5c                       	pop	r12
1409c32df: 41 5d                       	pop	r13
1409c32e1: 41 5e                       	pop	r14
1409c32e3: 41 5f                       	pop	r15
1409c32e5: 5d                          	pop	rbp
1409c32e6: c3                          	ret
1409c32e7: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1409c32f0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c32f5: 55                          	push	rbp
1409c32f6: 41 57                       	push	r15
1409c32f8: 41 56                       	push	r14
1409c32fa: 41 55                       	push	r13
1409c32fc: 41 54                       	push	r12
1409c32fe: 56                          	push	rsi
1409c32ff: 57                          	push	rdi
1409c3300: 53                          	push	rbx
1409c3301: 48 83 ec 48                 	sub	rsp, 0x48
1409c3305: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c330c: 48 8d 8d b0 02 00 00        	lea	rcx, [rbp + 0x2b0]
1409c3313: e8 98 3c ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c3318: 90                          	nop
1409c3319: 48 83 c4 48                 	add	rsp, 0x48
1409c331d: 5b                          	pop	rbx
1409c331e: 5f                          	pop	rdi
1409c331f: 5e                          	pop	rsi
1409c3320: 41 5c                       	pop	r12
1409c3322: 41 5d                       	pop	r13
1409c3324: 41 5e                       	pop	r14
1409c3326: 41 5f                       	pop	r15
1409c3328: 5d                          	pop	rbp
1409c3329: c3                          	ret
1409c332a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c3330: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c3335: 55                          	push	rbp
1409c3336: 41 57                       	push	r15
1409c3338: 41 56                       	push	r14
1409c333a: 41 55                       	push	r13
1409c333c: 41 54                       	push	r12
1409c333e: 56                          	push	rsi
1409c333f: 57                          	push	rdi
1409c3340: 53                          	push	rbx
1409c3341: 48 83 ec 48                 	sub	rsp, 0x48
1409c3345: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c334c: 48 8d 8d b0 07 00 00        	lea	rcx, [rbp + 0x7b0]
1409c3353: e8 58 3c ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c3358: 90                          	nop
1409c3359: 48 83 c4 48                 	add	rsp, 0x48
1409c335d: 5b                          	pop	rbx
1409c335e: 5f                          	pop	rdi
1409c335f: 5e                          	pop	rsi
1409c3360: 41 5c                       	pop	r12
1409c3362: 41 5d                       	pop	r13
1409c3364: 41 5e                       	pop	r14
1409c3366: 41 5f                       	pop	r15
1409c3368: 5d                          	pop	rbp
1409c3369: c3                          	ret
1409c336a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c3370: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c3375: 55                          	push	rbp
1409c3376: 41 57                       	push	r15
1409c3378: 41 56                       	push	r14
1409c337a: 41 55                       	push	r13
1409c337c: 41 54                       	push	r12
1409c337e: 56                          	push	rsi
1409c337f: 57                          	push	rdi
1409c3380: 53                          	push	rbx
1409c3381: 48 83 ec 48                 	sub	rsp, 0x48
1409c3385: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c338c: 48 8d 8d b0 02 00 00        	lea	rcx, [rbp + 0x2b0]
1409c3393: e8 18 3c ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c3398: 90                          	nop
1409c3399: 48 83 c4 48                 	add	rsp, 0x48
1409c339d: 5b                          	pop	rbx
1409c339e: 5f                          	pop	rdi
1409c339f: 5e                          	pop	rsi
1409c33a0: 41 5c                       	pop	r12
1409c33a2: 41 5d                       	pop	r13
1409c33a4: 41 5e                       	pop	r14
1409c33a6: 41 5f                       	pop	r15
1409c33a8: 5d                          	pop	rbp
1409c33a9: c3                          	ret
1409c33aa: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c33b0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c33b5: 55                          	push	rbp
1409c33b6: 41 57                       	push	r15
1409c33b8: 41 56                       	push	r14
1409c33ba: 41 55                       	push	r13
1409c33bc: 41 54                       	push	r12
1409c33be: 56                          	push	rsi
1409c33bf: 57                          	push	rdi
1409c33c0: 53                          	push	rbx
1409c33c1: 48 83 ec 48                 	sub	rsp, 0x48
1409c33c5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c33cc: 48 8d 8d b0 07 00 00        	lea	rcx, [rbp + 0x7b0]
1409c33d3: e8 d8 3b ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c33d8: 90                          	nop
1409c33d9: 48 83 c4 48                 	add	rsp, 0x48
1409c33dd: 5b                          	pop	rbx
1409c33de: 5f                          	pop	rdi
1409c33df: 5e                          	pop	rsi
1409c33e0: 41 5c                       	pop	r12
1409c33e2: 41 5d                       	pop	r13
1409c33e4: 41 5e                       	pop	r14
1409c33e6: 41 5f                       	pop	r15
1409c33e8: 5d                          	pop	rbp
1409c33e9: c3                          	ret
1409c33ea: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c33f0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c33f5: 55                          	push	rbp
1409c33f6: 41 57                       	push	r15
1409c33f8: 41 56                       	push	r14
1409c33fa: 41 55                       	push	r13
1409c33fc: 41 54                       	push	r12
1409c33fe: 56                          	push	rsi
1409c33ff: 57                          	push	rdi
1409c3400: 53                          	push	rbx
1409c3401: 48 83 ec 48                 	sub	rsp, 0x48
1409c3405: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c340c: 48 8d 8d b0 02 00 00        	lea	rcx, [rbp + 0x2b0]
1409c3413: e8 98 3b ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c3418: 90                          	nop
1409c3419: 48 83 c4 48                 	add	rsp, 0x48
1409c341d: 5b                          	pop	rbx
1409c341e: 5f                          	pop	rdi
1409c341f: 5e                          	pop	rsi
1409c3420: 41 5c                       	pop	r12
1409c3422: 41 5d                       	pop	r13
1409c3424: 41 5e                       	pop	r14
1409c3426: 41 5f                       	pop	r15
1409c3428: 5d                          	pop	rbp
1409c3429: c3                          	ret
1409c342a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c3430: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c3435: 55                          	push	rbp
1409c3436: 41 57                       	push	r15
1409c3438: 41 56                       	push	r14
1409c343a: 41 55                       	push	r13
1409c343c: 41 54                       	push	r12
1409c343e: 56                          	push	rsi
1409c343f: 57                          	push	rdi
1409c3440: 53                          	push	rbx
1409c3441: 48 83 ec 48                 	sub	rsp, 0x48
1409c3445: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c344c: 48 8d 8d b0 07 00 00        	lea	rcx, [rbp + 0x7b0]
1409c3453: e8 58 3b ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c3458: 90                          	nop
1409c3459: 48 83 c4 48                 	add	rsp, 0x48
1409c345d: 5b                          	pop	rbx
1409c345e: 5f                          	pop	rdi
1409c345f: 5e                          	pop	rsi
1409c3460: 41 5c                       	pop	r12
1409c3462: 41 5d                       	pop	r13
1409c3464: 41 5e                       	pop	r14
1409c3466: 41 5f                       	pop	r15
1409c3468: 5d                          	pop	rbp
1409c3469: c3                          	ret
1409c346a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c3470: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c3475: 55                          	push	rbp
1409c3476: 41 57                       	push	r15
1409c3478: 41 56                       	push	r14
1409c347a: 41 55                       	push	r13
1409c347c: 41 54                       	push	r12
1409c347e: 56                          	push	rsi
1409c347f: 57                          	push	rdi
1409c3480: 53                          	push	rbx
1409c3481: 48 83 ec 48                 	sub	rsp, 0x48
1409c3485: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c348c: 48 8d 8d b0 02 00 00        	lea	rcx, [rbp + 0x2b0]
1409c3493: e8 18 3b ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c3498: 90                          	nop
1409c3499: 48 83 c4 48                 	add	rsp, 0x48
1409c349d: 5b                          	pop	rbx
1409c349e: 5f                          	pop	rdi
1409c349f: 5e                          	pop	rsi
1409c34a0: 41 5c                       	pop	r12
1409c34a2: 41 5d                       	pop	r13
1409c34a4: 41 5e                       	pop	r14
1409c34a6: 41 5f                       	pop	r15
1409c34a8: 5d                          	pop	rbp
1409c34a9: c3                          	ret
1409c34aa: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c34b0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c34b5: 55                          	push	rbp
1409c34b6: 41 57                       	push	r15
1409c34b8: 41 56                       	push	r14
1409c34ba: 41 55                       	push	r13
1409c34bc: 41 54                       	push	r12
1409c34be: 56                          	push	rsi
1409c34bf: 57                          	push	rdi
1409c34c0: 53                          	push	rbx
1409c34c1: 48 83 ec 48                 	sub	rsp, 0x48
1409c34c5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c34cc: 48 8d 8d b0 07 00 00        	lea	rcx, [rbp + 0x7b0]
1409c34d3: e8 d8 3a ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c34d8: 90                          	nop
1409c34d9: 48 83 c4 48                 	add	rsp, 0x48
1409c34dd: 5b                          	pop	rbx
1409c34de: 5f                          	pop	rdi
1409c34df: 5e                          	pop	rsi
1409c34e0: 41 5c                       	pop	r12
1409c34e2: 41 5d                       	pop	r13
1409c34e4: 41 5e                       	pop	r14
1409c34e6: 41 5f                       	pop	r15
1409c34e8: 5d                          	pop	rbp
1409c34e9: c3                          	ret
1409c34ea: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c34f0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c34f5: 55                          	push	rbp
1409c34f6: 41 57                       	push	r15
1409c34f8: 41 56                       	push	r14
1409c34fa: 41 55                       	push	r13
1409c34fc: 41 54                       	push	r12
1409c34fe: 56                          	push	rsi
1409c34ff: 57                          	push	rdi
1409c3500: 53                          	push	rbx
1409c3501: 48 83 ec 48                 	sub	rsp, 0x48
1409c3505: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c350c: 48 8d 8d b0 02 00 00        	lea	rcx, [rbp + 0x2b0]
1409c3513: e8 98 3a ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c3518: 90                          	nop
1409c3519: 48 83 c4 48                 	add	rsp, 0x48
1409c351d: 5b                          	pop	rbx
1409c351e: 5f                          	pop	rdi
1409c351f: 5e                          	pop	rsi
1409c3520: 41 5c                       	pop	r12
1409c3522: 41 5d                       	pop	r13
1409c3524: 41 5e                       	pop	r14
1409c3526: 41 5f                       	pop	r15
1409c3528: 5d                          	pop	rbp
1409c3529: c3                          	ret
1409c352a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c3530: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c3535: 55                          	push	rbp
1409c3536: 41 57                       	push	r15
1409c3538: 41 56                       	push	r14
1409c353a: 41 55                       	push	r13
1409c353c: 41 54                       	push	r12
1409c353e: 56                          	push	rsi
1409c353f: 57                          	push	rdi
1409c3540: 53                          	push	rbx
1409c3541: 48 83 ec 48                 	sub	rsp, 0x48
1409c3545: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c354c: 48 8d 8d b0 07 00 00        	lea	rcx, [rbp + 0x7b0]
1409c3553: e8 58 3a ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c3558: 90                          	nop
1409c3559: 48 83 c4 48                 	add	rsp, 0x48
1409c355d: 5b                          	pop	rbx
1409c355e: 5f                          	pop	rdi
1409c355f: 5e                          	pop	rsi
1409c3560: 41 5c                       	pop	r12
1409c3562: 41 5d                       	pop	r13
1409c3564: 41 5e                       	pop	r14
1409c3566: 41 5f                       	pop	r15
1409c3568: 5d                          	pop	rbp
1409c3569: c3                          	ret
1409c356a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c3570: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c3575: 55                          	push	rbp
1409c3576: 41 57                       	push	r15
1409c3578: 41 56                       	push	r14
1409c357a: 41 55                       	push	r13
1409c357c: 41 54                       	push	r12
1409c357e: 56                          	push	rsi
1409c357f: 57                          	push	rdi
1409c3580: 53                          	push	rbx
1409c3581: 48 83 ec 48                 	sub	rsp, 0x48
1409c3585: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c358c: 80 bd e6 03 00 00 08        	cmp	byte ptr [rbp + 0x3e6], 0x8
1409c3593: 74 0c                       	je	0x1409c35a1 <.text+0x9c25a1>
1409c3595: 48 8d 8d d0 03 00 00        	lea	rcx, [rbp + 0x3d0]
1409c359c: e8 0f ef 21 00              	call	0x140be24b0 <.text+0xbe14b0>
1409c35a1: 90                          	nop
1409c35a2: 48 83 c4 48                 	add	rsp, 0x48
1409c35a6: 5b                          	pop	rbx
1409c35a7: 5f                          	pop	rdi
1409c35a8: 5e                          	pop	rsi
1409c35a9: 41 5c                       	pop	r12
1409c35ab: 41 5d                       	pop	r13
1409c35ad: 41 5e                       	pop	r14
1409c35af: 41 5f                       	pop	r15
1409c35b1: 5d                          	pop	rbp
1409c35b2: c3                          	ret
1409c35b3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
1409c35c0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c35c5: 55                          	push	rbp
1409c35c6: 41 57                       	push	r15
1409c35c8: 41 56                       	push	r14
1409c35ca: 41 55                       	push	r13
1409c35cc: 41 54                       	push	r12
1409c35ce: 56                          	push	rsi
1409c35cf: 57                          	push	rdi
1409c35d0: 53                          	push	rbx
1409c35d1: 48 83 ec 48                 	sub	rsp, 0x48
1409c35d5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c35dc: 80 bd 9d 0d 00 00 00        	cmp	byte ptr [rbp + 0xd9d], 0x0
1409c35e3: 75 11                       	jne	0x1409c35f6 <.text+0x9c25f6>
1409c35e5: 48 83 c4 48                 	add	rsp, 0x48
1409c35e9: 5b                          	pop	rbx
1409c35ea: 5f                          	pop	rdi
1409c35eb: 5e                          	pop	rsi
1409c35ec: 41 5c                       	pop	r12
1409c35ee: 41 5d                       	pop	r13
1409c35f0: 41 5e                       	pop	r14
1409c35f2: 41 5f                       	pop	r15
1409c35f4: 5d                          	pop	rbp
1409c35f5: c3                          	ret
1409c35f6: 48 8d 8d b0 07 00 00        	lea	rcx, [rbp + 0x7b0]
1409c35fd: e8 ae 39 ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c3602: eb e1                       	jmp	0x1409c35e5 <.text+0x9c25e5>
1409c3604: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
1409c3610: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c3615: 55                          	push	rbp
1409c3616: 41 57                       	push	r15
1409c3618: 41 56                       	push	r14
1409c361a: 41 55                       	push	r13
1409c361c: 41 54                       	push	r12
1409c361e: 56                          	push	rsi
1409c361f: 57                          	push	rdi
1409c3620: 53                          	push	rbx
1409c3621: 48 83 ec 48                 	sub	rsp, 0x48
1409c3625: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c362c: 80 bd 9e 0d 00 00 00        	cmp	byte ptr [rbp + 0xd9e], 0x0
1409c3633: 75 11                       	jne	0x1409c3646 <.text+0x9c2646>
1409c3635: 48 83 c4 48                 	add	rsp, 0x48
1409c3639: 5b                          	pop	rbx
1409c363a: 5f                          	pop	rdi
1409c363b: 5e                          	pop	rsi
1409c363c: 41 5c                       	pop	r12
1409c363e: 41 5d                       	pop	r13
1409c3640: 41 5e                       	pop	r14
1409c3642: 41 5f                       	pop	r15
1409c3644: 5d                          	pop	rbp
1409c3645: c3                          	ret
1409c3646: 48 8d 8d b0 02 00 00        	lea	rcx, [rbp + 0x2b0]
1409c364d: e8 5e 39 ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c3652: eb e1                       	jmp	0x1409c3635 <.text+0x9c2635>
1409c3654: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
1409c3660: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c3665: 55                          	push	rbp
1409c3666: 41 57                       	push	r15
1409c3668: 41 56                       	push	r14
1409c366a: 41 55                       	push	r13
1409c366c: 41 54                       	push	r12
1409c366e: 56                          	push	rsi
1409c366f: 57                          	push	rdi
1409c3670: 53                          	push	rbx
1409c3671: 48 83 ec 48                 	sub	rsp, 0x48
1409c3675: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c367c: 80 bd 9f 0d 00 00 00        	cmp	byte ptr [rbp + 0xd9f], 0x0
1409c3683: 75 11                       	jne	0x1409c3696 <.text+0x9c2696>
1409c3685: 48 83 c4 48                 	add	rsp, 0x48
1409c3689: 5b                          	pop	rbx
1409c368a: 5f                          	pop	rdi
1409c368b: 5e                          	pop	rsi
1409c368c: 41 5c                       	pop	r12
1409c368e: 41 5d                       	pop	r13
1409c3690: 41 5e                       	pop	r14
1409c3692: 41 5f                       	pop	r15
1409c3694: 5d                          	pop	rbp
1409c3695: c3                          	ret
1409c3696: 48 8d 8d b0 02 00 00        	lea	rcx, [rbp + 0x2b0]
1409c369d: e8 1e ed ec ff              	call	0x1408923c0 <.text+0x8913c0>
1409c36a2: eb e1                       	jmp	0x1409c3685 <.text+0x9c2685>
1409c36a4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
1409c36b0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c36b5: 55                          	push	rbp
1409c36b6: 41 57                       	push	r15
1409c36b8: 41 56                       	push	r14
1409c36ba: 41 55                       	push	r13
1409c36bc: 41 54                       	push	r12
1409c36be: 56                          	push	rsi
1409c36bf: 57                          	push	rdi
1409c36c0: 53                          	push	rbx
1409c36c1: 48 83 ec 48                 	sub	rsp, 0x48
1409c36c5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c36cc: 48 8d 8d b0 02 00 00        	lea	rcx, [rbp + 0x2b0]
1409c36d3: e8 58 95 38 00              	call	0x140d4cc30 <.text+0xd4bc30>
1409c36d8: 90                          	nop
1409c36d9: 48 83 c4 48                 	add	rsp, 0x48
1409c36dd: 5b                          	pop	rbx
1409c36de: 5f                          	pop	rdi
1409c36df: 5e                          	pop	rsi
1409c36e0: 41 5c                       	pop	r12
1409c36e2: 41 5d                       	pop	r13
1409c36e4: 41 5e                       	pop	r14
1409c36e6: 41 5f                       	pop	r15
1409c36e8: 5d                          	pop	rbp
1409c36e9: c3                          	ret
1409c36ea: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c36f0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c36f5: 55                          	push	rbp
1409c36f6: 41 57                       	push	r15
1409c36f8: 41 56                       	push	r14
1409c36fa: 41 55                       	push	r13
1409c36fc: 41 54                       	push	r12
1409c36fe: 56                          	push	rsi
1409c36ff: 57                          	push	rdi
1409c3700: 53                          	push	rbx
1409c3701: 48 83 ec 48                 	sub	rsp, 0x48
1409c3705: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c370c: 48 8d 8d b0 02 00 00        	lea	rcx, [rbp + 0x2b0]
1409c3713: e8 18 95 38 00              	call	0x140d4cc30 <.text+0xd4bc30>
1409c3718: 90                          	nop
1409c3719: 48 83 c4 48                 	add	rsp, 0x48
1409c371d: 5b                          	pop	rbx
1409c371e: 5f                          	pop	rdi
1409c371f: 5e                          	pop	rsi
1409c3720: 41 5c                       	pop	r12
1409c3722: 41 5d                       	pop	r13
1409c3724: 41 5e                       	pop	r14
1409c3726: 41 5f                       	pop	r15
1409c3728: 5d                          	pop	rbp
1409c3729: c3                          	ret
1409c372a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c3730: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c3735: 55                          	push	rbp
1409c3736: 41 57                       	push	r15
1409c3738: 41 56                       	push	r14
1409c373a: 41 55                       	push	r13
1409c373c: 41 54                       	push	r12
1409c373e: 56                          	push	rsi
1409c373f: 57                          	push	rdi
1409c3740: 53                          	push	rbx
1409c3741: 48 83 ec 48                 	sub	rsp, 0x48
1409c3745: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c374c: c6 85 9e 0d 00 00 00        	mov	byte ptr [rbp + 0xd9e], 0x0
1409c3753: 48 8d 8d b0 02 00 00        	lea	rcx, [rbp + 0x2b0]
1409c375a: e8 51 38 ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c375f: c6 85 9e 0d 00 00 00        	mov	byte ptr [rbp + 0xd9e], 0x0
1409c3766: 48 83 c4 48                 	add	rsp, 0x48
1409c376a: 5b                          	pop	rbx
1409c376b: 5f                          	pop	rdi
1409c376c: 5e                          	pop	rsi
1409c376d: 41 5c                       	pop	r12
1409c376f: 41 5d                       	pop	r13
1409c3771: 41 5e                       	pop	r14
1409c3773: 41 5f                       	pop	r15
1409c3775: 5d                          	pop	rbp
1409c3776: c3                          	ret
1409c3777: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1409c3780: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c3785: 55                          	push	rbp
1409c3786: 41 57                       	push	r15
1409c3788: 41 56                       	push	r14
1409c378a: 41 55                       	push	r13
1409c378c: 41 54                       	push	r12
1409c378e: 56                          	push	rsi
1409c378f: 57                          	push	rdi
1409c3790: 53                          	push	rbx
1409c3791: 48 83 ec 48                 	sub	rsp, 0x48
1409c3795: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c379c: 48 8d 8d b0 07 00 00        	lea	rcx, [rbp + 0x7b0]
1409c37a3: e8 08 38 ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c37a8: 90                          	nop
1409c37a9: 48 83 c4 48                 	add	rsp, 0x48
1409c37ad: 5b                          	pop	rbx
1409c37ae: 5f                          	pop	rdi
1409c37af: 5e                          	pop	rsi
1409c37b0: 41 5c                       	pop	r12
1409c37b2: 41 5d                       	pop	r13
1409c37b4: 41 5e                       	pop	r14
1409c37b6: 41 5f                       	pop	r15
1409c37b8: 5d                          	pop	rbp
1409c37b9: c3                          	ret
1409c37ba: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c37c0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c37c5: 55                          	push	rbp
1409c37c6: 41 57                       	push	r15
1409c37c8: 41 56                       	push	r14
1409c37ca: 41 55                       	push	r13
1409c37cc: 41 54                       	push	r12
1409c37ce: 56                          	push	rsi
1409c37cf: 57                          	push	rdi
1409c37d0: 53                          	push	rbx
1409c37d1: 48 83 ec 48                 	sub	rsp, 0x48
1409c37d5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c37dc: 48 8d 8d b0 02 00 00        	lea	rcx, [rbp + 0x2b0]
1409c37e3: e8 c8 37 ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c37e8: 90                          	nop
1409c37e9: 48 83 c4 48                 	add	rsp, 0x48
1409c37ed: 5b                          	pop	rbx
1409c37ee: 5f                          	pop	rdi
1409c37ef: 5e                          	pop	rsi
1409c37f0: 41 5c                       	pop	r12
1409c37f2: 41 5d                       	pop	r13
1409c37f4: 41 5e                       	pop	r14
1409c37f6: 41 5f                       	pop	r15
1409c37f8: 5d                          	pop	rbp
1409c37f9: c3                          	ret
1409c37fa: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c3800: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c3805: 55                          	push	rbp
1409c3806: 41 57                       	push	r15
1409c3808: 41 56                       	push	r14
1409c380a: 41 55                       	push	r13
1409c380c: 41 54                       	push	r12
1409c380e: 56                          	push	rsi
1409c380f: 57                          	push	rdi
1409c3810: 53                          	push	rbx
1409c3811: 48 83 ec 48                 	sub	rsp, 0x48
1409c3815: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c381c: c6 85 9f 0d 00 00 01        	mov	byte ptr [rbp + 0xd9f], 0x1
1409c3823: 48 8d 8d e0 0b 00 00        	lea	rcx, [rbp + 0xbe0]
1409c382a: e8 91 fd ec ff              	call	0x1408935c0 <.text+0x8925c0>
1409c382f: c6 85 9f 0d 00 00 01        	mov	byte ptr [rbp + 0xd9f], 0x1
1409c3836: 48 83 c4 48                 	add	rsp, 0x48
1409c383a: 5b                          	pop	rbx
1409c383b: 5f                          	pop	rdi
1409c383c: 5e                          	pop	rsi
1409c383d: 41 5c                       	pop	r12
1409c383f: 41 5d                       	pop	r13
1409c3841: 41 5e                       	pop	r14
1409c3843: 41 5f                       	pop	r15
1409c3845: 5d                          	pop	rbp
1409c3846: c3                          	ret
1409c3847: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1409c3850: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c3855: 55                          	push	rbp
1409c3856: 41 57                       	push	r15
1409c3858: 41 56                       	push	r14
1409c385a: 41 55                       	push	r13
1409c385c: 41 54                       	push	r12
1409c385e: 56                          	push	rsi
1409c385f: 57                          	push	rdi
1409c3860: 53                          	push	rbx
1409c3861: 48 83 ec 48                 	sub	rsp, 0x48
1409c3865: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c386c: c6 85 9f 0d 00 00 01        	mov	byte ptr [rbp + 0xd9f], 0x1
1409c3873: 48 8d 8d b0 07 00 00        	lea	rcx, [rbp + 0x7b0]
1409c387a: e8 41 fd ec ff              	call	0x1408935c0 <.text+0x8925c0>
1409c387f: c6 85 9f 0d 00 00 01        	mov	byte ptr [rbp + 0xd9f], 0x1
1409c3886: 48 83 c4 48                 	add	rsp, 0x48
1409c388a: 5b                          	pop	rbx
1409c388b: 5f                          	pop	rdi
1409c388c: 5e                          	pop	rsi
1409c388d: 41 5c                       	pop	r12
1409c388f: 41 5d                       	pop	r13
1409c3891: 41 5e                       	pop	r14
1409c3893: 41 5f                       	pop	r15
1409c3895: 5d                          	pop	rbp
1409c3896: c3                          	ret
1409c3897: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1409c38a0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c38a5: 55                          	push	rbp
1409c38a6: 41 57                       	push	r15
1409c38a8: 41 56                       	push	r14
1409c38aa: 41 55                       	push	r13
1409c38ac: 41 54                       	push	r12
1409c38ae: 56                          	push	rsi
1409c38af: 57                          	push	rdi
1409c38b0: 53                          	push	rbx
1409c38b1: 48 83 ec 48                 	sub	rsp, 0x48
1409c38b5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c38bc: c6 85 9f 0d 00 00 01        	mov	byte ptr [rbp + 0xd9f], 0x1
1409c38c3: 48 8d 8d b8 06 00 00        	lea	rcx, [rbp + 0x6b8]
1409c38ca: e8 f1 fc ec ff              	call	0x1408935c0 <.text+0x8925c0>
1409c38cf: c6 85 9f 0d 00 00 01        	mov	byte ptr [rbp + 0xd9f], 0x1
1409c38d6: 48 83 c4 48                 	add	rsp, 0x48
1409c38da: 5b                          	pop	rbx
1409c38db: 5f                          	pop	rdi
1409c38dc: 5e                          	pop	rsi
1409c38dd: 41 5c                       	pop	r12
1409c38df: 41 5d                       	pop	r13
1409c38e1: 41 5e                       	pop	r14
1409c38e3: 41 5f                       	pop	r15
1409c38e5: 5d                          	pop	rbp
1409c38e6: c3                          	ret
1409c38e7: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1409c38f0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c38f5: 55                          	push	rbp
1409c38f6: 41 57                       	push	r15
1409c38f8: 41 56                       	push	r14
1409c38fa: 41 55                       	push	r13
1409c38fc: 41 54                       	push	r12
1409c38fe: 56                          	push	rsi
1409c38ff: 57                          	push	rdi
1409c3900: 53                          	push	rbx
1409c3901: 48 83 ec 48                 	sub	rsp, 0x48
1409c3905: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c390c: c6 85 9f 0d 00 00 01        	mov	byte ptr [rbp + 0xd9f], 0x1
1409c3913: 48 8d 8d b0 07 00 00        	lea	rcx, [rbp + 0x7b0]
1409c391a: e8 a1 fc ec ff              	call	0x1408935c0 <.text+0x8925c0>
1409c391f: c6 85 9f 0d 00 00 01        	mov	byte ptr [rbp + 0xd9f], 0x1
1409c3926: 48 83 c4 48                 	add	rsp, 0x48
1409c392a: 5b                          	pop	rbx
1409c392b: 5f                          	pop	rdi
1409c392c: 5e                          	pop	rsi
1409c392d: 41 5c                       	pop	r12
1409c392f: 41 5d                       	pop	r13
1409c3931: 41 5e                       	pop	r14
1409c3933: 41 5f                       	pop	r15
1409c3935: 5d                          	pop	rbp
1409c3936: c3                          	ret
1409c3937: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1409c3940: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c3945: 55                          	push	rbp
1409c3946: 41 57                       	push	r15
1409c3948: 41 56                       	push	r14
1409c394a: 41 55                       	push	r13
1409c394c: 41 54                       	push	r12
1409c394e: 56                          	push	rsi
1409c394f: 57                          	push	rdi
1409c3950: 53                          	push	rbx
1409c3951: 48 83 ec 48                 	sub	rsp, 0x48
1409c3955: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c395c: c6 85 9f 0d 00 00 01        	mov	byte ptr [rbp + 0xd9f], 0x1
1409c3963: 48 8d 8d e0 0b 00 00        	lea	rcx, [rbp + 0xbe0]
1409c396a: e8 51 fc ec ff              	call	0x1408935c0 <.text+0x8925c0>
1409c396f: c6 85 9f 0d 00 00 01        	mov	byte ptr [rbp + 0xd9f], 0x1
1409c3976: 48 83 c4 48                 	add	rsp, 0x48
1409c397a: 5b                          	pop	rbx
1409c397b: 5f                          	pop	rdi
1409c397c: 5e                          	pop	rsi
1409c397d: 41 5c                       	pop	r12
1409c397f: 41 5d                       	pop	r13
1409c3981: 41 5e                       	pop	r14
1409c3983: 41 5f                       	pop	r15
1409c3985: 5d                          	pop	rbp
1409c3986: c3                          	ret
1409c3987: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1409c3990: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c3995: 55                          	push	rbp
1409c3996: 41 57                       	push	r15
1409c3998: 41 56                       	push	r14
1409c399a: 41 55                       	push	r13
1409c399c: 41 54                       	push	r12
1409c399e: 56                          	push	rsi
1409c399f: 57                          	push	rdi
1409c39a0: 53                          	push	rbx
1409c39a1: 48 83 ec 48                 	sub	rsp, 0x48
1409c39a5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c39ac: c6 85 9f 0d 00 00 01        	mov	byte ptr [rbp + 0xd9f], 0x1
1409c39b3: 48 8d 4d d0                 	lea	rcx, [rbp - 0x30]
1409c39b7: e8 04 fc ec ff              	call	0x1408935c0 <.text+0x8925c0>
1409c39bc: c6 85 9f 0d 00 00 01        	mov	byte ptr [rbp + 0xd9f], 0x1
1409c39c3: 48 83 c4 48                 	add	rsp, 0x48
1409c39c7: 5b                          	pop	rbx
1409c39c8: 5f                          	pop	rdi
1409c39c9: 5e                          	pop	rsi
1409c39ca: 41 5c                       	pop	r12
1409c39cc: 41 5d                       	pop	r13
1409c39ce: 41 5e                       	pop	r14
1409c39d0: 41 5f                       	pop	r15
1409c39d2: 5d                          	pop	rbp
1409c39d3: c3                          	ret
1409c39d4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
1409c39e0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c39e5: 55                          	push	rbp
1409c39e6: 41 57                       	push	r15
1409c39e8: 41 56                       	push	r14
1409c39ea: 41 55                       	push	r13
1409c39ec: 41 54                       	push	r12
1409c39ee: 56                          	push	rsi
1409c39ef: 57                          	push	rdi
1409c39f0: 53                          	push	rbx
1409c39f1: 48 83 ec 48                 	sub	rsp, 0x48
1409c39f5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c39fc: c6 85 9f 0d 00 00 01        	mov	byte ptr [rbp + 0xd9f], 0x1
1409c3a03: 48 8d 8d b0 07 00 00        	lea	rcx, [rbp + 0x7b0]
1409c3a0a: e8 b1 fb ec ff              	call	0x1408935c0 <.text+0x8925c0>
1409c3a0f: c6 85 9f 0d 00 00 01        	mov	byte ptr [rbp + 0xd9f], 0x1
1409c3a16: 48 83 c4 48                 	add	rsp, 0x48
1409c3a1a: 5b                          	pop	rbx
1409c3a1b: 5f                          	pop	rdi
1409c3a1c: 5e                          	pop	rsi
1409c3a1d: 41 5c                       	pop	r12
1409c3a1f: 41 5d                       	pop	r13
1409c3a21: 41 5e                       	pop	r14
1409c3a23: 41 5f                       	pop	r15
1409c3a25: 5d                          	pop	rbp
1409c3a26: c3                          	ret
1409c3a27: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1409c3a30: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c3a35: 55                          	push	rbp
1409c3a36: 41 57                       	push	r15
1409c3a38: 41 56                       	push	r14
1409c3a3a: 41 55                       	push	r13
1409c3a3c: 41 54                       	push	r12
1409c3a3e: 56                          	push	rsi
1409c3a3f: 57                          	push	rdi
1409c3a40: 53                          	push	rbx
1409c3a41: 48 83 ec 48                 	sub	rsp, 0x48
1409c3a45: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c3a4c: c6 85 9f 0d 00 00 01        	mov	byte ptr [rbp + 0xd9f], 0x1
1409c3a53: 48 8d 8d e0 0b 00 00        	lea	rcx, [rbp + 0xbe0]
1409c3a5a: e8 61 fb ec ff              	call	0x1408935c0 <.text+0x8925c0>
1409c3a5f: c6 85 9f 0d 00 00 01        	mov	byte ptr [rbp + 0xd9f], 0x1
1409c3a66: 48 83 c4 48                 	add	rsp, 0x48
1409c3a6a: 5b                          	pop	rbx
1409c3a6b: 5f                          	pop	rdi
1409c3a6c: 5e                          	pop	rsi
1409c3a6d: 41 5c                       	pop	r12
1409c3a6f: 41 5d                       	pop	r13
1409c3a71: 41 5e                       	pop	r14
1409c3a73: 41 5f                       	pop	r15
1409c3a75: 5d                          	pop	rbp
1409c3a76: c3                          	ret
