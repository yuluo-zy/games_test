
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1409bf800: 55                          	push	rbp
1409bf801: 41 57                       	push	r15
1409bf803: 41 56                       	push	r14
1409bf805: 41 55                       	push	r13
1409bf807: 41 54                       	push	r12
1409bf809: 56                          	push	rsi
1409bf80a: 57                          	push	rdi
1409bf80b: 53                          	push	rbx
1409bf80c: 48 81 ec 28 0c 00 00        	sub	rsp, 0xc28
1409bf813: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
1409bf81b: 48 c7 85 a0 0b 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0xba0], -0x2
1409bf826: 4c 89 ce                    	mov	rsi, r9
1409bf829: 4c 89 c7                    	mov	rdi, r8
1409bf82c: 48 89 d3                    	mov	rbx, rdx
1409bf82f: 49 89 cc                    	mov	r12, rcx
1409bf832: 4c 8b b5 10 0c 00 00        	mov	r14, qword ptr [rbp + 0xc10]
1409bf839: 48 8d 8d a0 09 00 00        	lea	rcx, [rbp + 0x9a0]
1409bf840: ba 10 00 00 00              	mov	edx, 0x10
1409bf845: 41 b8 20 00 00 00           	mov	r8d, 0x20
1409bf84b: e8 d0 55 21 00              	call	0x140bd4e20 <.text+0xbd3e20>
1409bf850: 0f 10 85 a0 09 00 00        	movups	xmm0, xmmword ptr [rbp + 0x9a0]
1409bf857: 0f 29 85 c0 01 00 00        	movaps	xmmword ptr [rbp + 0x1c0], xmm0
1409bf85e: 48 8d 8d c0 05 00 00        	lea	rcx, [rbp + 0x5c0]
1409bf865: 48 8d 95 c0 01 00 00        	lea	rdx, [rbp + 0x1c0]
1409bf86c: e8 8f 74 21 00              	call	0x140bd6d00 <.text+0xbd5d00>
1409bf871: 48 8d 05 79 03 10 02        	lea	rax, [rip + 0x2100379]  # 0x142abfbf1
1409bf878: 48 89 85 b0 05 00 00        	mov	qword ptr [rbp + 0x5b0], rax
1409bf87f: 48 c7 85 b8 05 00 00 15 00 00 00    	mov	qword ptr [rbp + 0x5b8], 0x15
1409bf88a: 48 8d 95 b0 05 00 00        	lea	rdx, [rbp + 0x5b0]
1409bf891: 4c 89 e1                    	mov	rcx, r12
1409bf894: e8 37 9c 22 00              	call	0x140be94d0 <.text+0xbe84d0>
1409bf899: 0f 10 85 a0 09 00 00        	movups	xmm0, xmmword ptr [rbp + 0x9a0]
1409bf8a0: 0f 29 85 60 0b 00 00        	movaps	xmmword ptr [rbp + 0xb60], xmm0
1409bf8a7: 89 85 70 0b 00 00           	mov	dword ptr [rbp + 0xb70], eax
1409bf8ad: 4c 8d 85 60 0b 00 00        	lea	r8, [rbp + 0xb60]
1409bf8b4: 4c 89 e1                    	mov	rcx, r12
1409bf8b7: 48 89 da                    	mov	rdx, rbx
1409bf8ba: e8 f1 e2 01 00              	call	0x1409ddbb0 <.text+0x9dcbb0>
1409bf8bf: 49 8d 4e 04                 	lea	rcx, [r14 + 0x4]
1409bf8c3: e8 c8 5e 21 00              	call	0x140bd5790 <.text+0xbd4790>
1409bf8c8: 48 89 c1                    	mov	rcx, rax
1409bf8cb: 48 c1 e9 20                 	shr	rcx, 0x20
1409bf8cf: 0f af c1                    	imul	eax, ecx
1409bf8d2: 89 c2                       	mov	edx, eax
1409bf8d4: c1 ea 04                    	shr	edx, 0x4
1409bf8d7: 01 c2                       	add	edx, eax
1409bf8d9: 48 c1 e2 04                 	shl	rdx, 0x4
1409bf8dd: 4c 8d bd a0 09 00 00        	lea	r15, [rbp + 0x9a0]
1409bf8e4: 4c 89 f9                    	mov	rcx, r15
1409bf8e7: 41 b8 20 00 00 00           	mov	r8d, 0x20
1409bf8ed: e8 2e 55 21 00              	call	0x140bd4e20 <.text+0xbd3e20>
1409bf8f2: 0f 10 85 a0 09 00 00        	movups	xmm0, xmmword ptr [rbp + 0x9a0]
1409bf8f9: 0f 29 85 c0 01 00 00        	movaps	xmmword ptr [rbp + 0x1c0], xmm0
1409bf900: 48 8d 8d c0 05 00 00        	lea	rcx, [rbp + 0x5c0]
1409bf907: 48 8d 95 c0 01 00 00        	lea	rdx, [rbp + 0x1c0]
1409bf90e: e8 ed 73 21 00              	call	0x140bd6d00 <.text+0xbd5d00>
1409bf913: 48 8d 05 ec 02 10 02        	lea	rax, [rip + 0x21002ec]  # 0x142abfc06
1409bf91a: 48 89 85 b0 05 00 00        	mov	qword ptr [rbp + 0x5b0], rax
1409bf921: 48 c7 85 b8 05 00 00 13 00 00 00    	mov	qword ptr [rbp + 0x5b8], 0x13
1409bf92c: 48 8d 95 b0 05 00 00        	lea	rdx, [rbp + 0x5b0]
1409bf933: 4c 89 e1                    	mov	rcx, r12
1409bf936: e8 95 9b 22 00              	call	0x140be94d0 <.text+0xbe84d0>
1409bf93b: 0f 10 85 a0 09 00 00        	movups	xmm0, xmmword ptr [rbp + 0x9a0]
1409bf942: 0f 29 85 30 0b 00 00        	movaps	xmmword ptr [rbp + 0xb30], xmm0
1409bf949: 89 85 40 0b 00 00           	mov	dword ptr [rbp + 0xb40], eax
1409bf94f: 48 8d 4e 04                 	lea	rcx, [rsi + 0x4]
1409bf953: e8 38 5e 21 00              	call	0x140bd5790 <.text+0xbd4790>
1409bf958: c7 85 b0 05 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x5b0], 0x0
1409bf962: 48 89 bd 58 0b 00 00        	mov	qword ptr [rbp + 0xb58], rdi
1409bf969: 48 89 bd b8 05 00 00        	mov	qword ptr [rbp + 0x5b8], rdi
1409bf970: 48 c7 85 a0 09 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x9a0], 0x0
1409bf97b: 48 89 b5 a8 09 00 00        	mov	qword ptr [rbp + 0x9a8], rsi
1409bf982: 48 c7 85 b0 09 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x9b0], 0x0
1409bf98d: 4c 89 a5 50 0b 00 00        	mov	qword ptr [rbp + 0xb50], r12
1409bf994: 4c 89 64 24 40              	mov	qword ptr [rsp + 0x40], r12
1409bf999: 48 89 5c 24 38              	mov	qword ptr [rsp + 0x38], rbx
1409bf99e: 48 89 44 24 30              	mov	qword ptr [rsp + 0x30], rax
1409bf9a3: 4c 89 7c 24 28              	mov	qword ptr [rsp + 0x28], r15
1409bf9a8: 48 c7 44 24 20 01 00 00 00  	mov	qword ptr [rsp + 0x20], 0x1
1409bf9b1: 48 8d 15 61 02 10 02        	lea	rdx, [rip + 0x2100261]  # 0x142abfc19
1409bf9b8: 48 8d 8d c0 01 00 00        	lea	rcx, [rbp + 0x1c0]
1409bf9bf: 4c 8d a5 b0 05 00 00        	lea	r12, [rbp + 0x5b0]
1409bf9c6: 41 b8 0b 00 00 00           	mov	r8d, 0xb
1409bf9cc: 4d 89 e1                    	mov	r9, r12
1409bf9cf: e8 9c 7f ea ff              	call	0x140867970 <.text+0x866970>
1409bf9d4: 48 89 9d 90 0b 00 00        	mov	qword ptr [rbp + 0xb90], rbx
1409bf9db: 48 8d 4b 38                 	lea	rcx, [rbx + 0x38]
1409bf9df: c6 85 9f 0b 00 00 01        	mov	byte ptr [rbp + 0xb9f], 0x1
1409bf9e6: 48 ba 29 f7 80 5b 19 62 c3 4b       	movabs	rdx, 0x4bc362195b80f729
1409bf9f0: 49 b8 fb ce cb 80 ce 98 27 98       	movabs	r8, -0x67d867317f343105
1409bf9fa: e8 f1 dc 71 ff              	call	0x1400dd6f0 <.text+0xdc6f0>
1409bf9ff: 48 85 c0                    	test	rax, rax
1409bfa02: 48 8b 8d 90 0b 00 00        	mov	rcx, qword ptr [rbp + 0xb90]
1409bfa09: 0f 84 fb 09 00 00           	je	0x1409c040a <.text+0x9bf40a>
1409bfa0f: 48 39 91 18 02 00 00        	cmp	qword ptr [rcx + 0x218], rdx
1409bfa16: 0f 86 ee 09 00 00           	jbe	0x1409c040a <.text+0x9bf40a>
1409bfa1c: 48 8b 81 10 02 00 00        	mov	rax, qword ptr [rcx + 0x210]
1409bfa23: 48 8b 04 d0                 	mov	rax, qword ptr [rax + 8*rdx]
1409bfa27: 48 85 c0                    	test	rax, rax
1409bfa2a: 0f 84 da 09 00 00           	je	0x1409c040a <.text+0x9bf40a>
1409bfa30: 48 8b 91 e0 01 00 00        	mov	rdx, qword ptr [rcx + 0x1e0]
1409bfa37: 48 f7 d0                    	not	rax
1409bfa3a: 4c 8d 04 40                 	lea	r8, [rax + 2*rax]
1409bfa3e: 49 c1 e0 05                 	shl	r8, 0x5
1409bfa42: 4a 83 7c 02 20 00           	cmp	qword ptr [rdx + r8 + 0x20], 0x0
1409bfa48: 0f 84 bc 09 00 00           	je	0x1409c040a <.text+0x9bf40a>
1409bfa4e: 48 8b 8d 38 0c 00 00        	mov	rcx, qword ptr [rbp + 0xc38]
1409bfa55: 0f b6 85 20 0c 00 00        	movzx	eax, byte ptr [rbp + 0xc20]
1409bfa5c: 4c 01 c2                    	add	rdx, r8
1409bfa5f: 48 8b 52 10                 	mov	rdx, qword ptr [rdx + 0x10]
1409bfa63: 4c 8b 02                    	mov	r8, qword ptr [rdx]
1409bfa66: 4d 85 c0                    	test	r8, r8
1409bfa69: 78 0a                       	js	0x1409bfa75 <.text+0x9bea75>
1409bfa6b: 0f 57 c0                    	xorps	xmm0, xmm0
1409bfa6e: f3 49 0f 2a c0              	cvtsi2ss	xmm0, r8
1409bfa73: eb 19                       	jmp	0x1409bfa8e <.text+0x9bea8e>
1409bfa75: 4d 89 c1                    	mov	r9, r8
1409bfa78: 49 d1 e9                    	shr	r9
1409bfa7b: 41 83 e0 01                 	and	r8d, 0x1
1409bfa7f: 4d 09 c8                    	or	r8, r9
1409bfa82: 0f 57 c0                    	xorps	xmm0, xmm0
1409bfa85: f3 49 0f 2a c0              	cvtsi2ss	xmm0, r8
1409bfa8a: f3 0f 58 c0                 	addss	xmm0, xmm0
1409bfa8e: f3 0f 2a 4a 08              	cvtsi2ss	xmm1, dword ptr [rdx + 0x8]
1409bfa93: f3 0f 5e 0d a5 7b fe 01     	divss	xmm1, dword ptr [rip + 0x1fe7ba5] # 0x1429a7640
1409bfa9b: f3 0f 58 c8                 	addss	xmm1, xmm0
1409bfa9f: f3 0f 11 8d 80 0b 00 00     	movss	dword ptr [rbp + 0xb80], xmm1
1409bfaa7: c6 85 84 0b 00 00 00        	mov	byte ptr [rbp + 0xb84], 0x0
1409bfaae: 4c 8b a9 70 01 00 00        	mov	r13, qword ptr [rcx + 0x170]
1409bfab5: 48 8b 89 78 01 00 00        	mov	rcx, qword ptr [rcx + 0x178]
1409bfabc: 48 c1 e1 07                 	shl	rcx, 0x7
1409bfac0: 4c 01 e9                    	add	rcx, r13
1409bfac3: 48 89 8d 48 0b 00 00        	mov	qword ptr [rbp + 0xb48], rcx
1409bfaca: 0f b6 c0                    	movzx	eax, al
1409bfacd: 89 85 98 0b 00 00           	mov	dword ptr [rbp + 0xb98], eax
1409bfad3: 48 8d b5 88 0b 00 00        	lea	rsi, [rbp + 0xb88]
1409bfada: 4c 3b ad 48 0b 00 00        	cmp	r13, qword ptr [rbp + 0xb48]
1409bfae1: 0f 84 6d 03 00 00           	je	0x1409bfe54 <.text+0x9bee54>
1409bfae7: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1409bfaf0: 49 8b 5d 50                 	mov	rbx, qword ptr [r13 + 0x50]
1409bfaf4: 49 8b 7d 58                 	mov	rdi, qword ptr [r13 + 0x58]
1409bfaf8: c6 85 9f 0b 00 00 01        	mov	byte ptr [rbp + 0xb9f], 0x1
1409bfaff: e8 4c 6c 22 00              	call	0x140be6750 <.text+0xbe5750>
1409bfb04: 41 89 c6                    	mov	r14d, eax
1409bfb07: c6 85 9f 0b 00 00 01        	mov	byte ptr [rbp + 0xb9f], 0x1
1409bfb0e: 31 c9                       	xor	ecx, ecx
1409bfb10: e8 0b a0 71 ff              	call	0x1400d9b20 <.text+0xd8b20>
1409bfb15: 48 85 c0                    	test	rax, rax
1409bfb18: 0f 84 d7 08 00 00           	je	0x1409c03f5 <.text+0x9bf3f5>
1409bfb1e: 44 89 b5 48 0a 00 00        	mov	dword ptr [rbp + 0xa48], r14d
1409bfb25: 66 c7 85 50 0a 00 00 00 00  	mov	word ptr [rbp + 0xa50], 0x0
1409bfb2e: c7 85 4c 0a 00 00 00 00 80 3f       	mov	dword ptr [rbp + 0xa4c], 0x3f800000
1409bfb38: 48 c7 85 a0 09 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x9a0], 0x1
1409bfb43: 48 89 9d a8 09 00 00        	mov	qword ptr [rbp + 0x9a8], rbx
1409bfb4a: 48 89 bd 40 0a 00 00        	mov	qword ptr [rbp + 0xa40], rdi
1409bfb51: 48 c7 85 b0 09 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x9b0], 0x0
1409bfb5c: 48 c7 85 b8 09 00 00 04 00 00 00    	mov	qword ptr [rbp + 0x9b8], 0x4
1409bfb67: 48 c7 85 c0 09 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x9c0], 0x0
1409bfb72: 0f 10 00                    	movups	xmm0, xmmword ptr [rax]
1409bfb75: 48 ff 00                    	inc	qword ptr [rax]
1409bfb78: 0f 10 0d b1 6b 0d 02        	movups	xmm1, xmmword ptr [rip + 0x20d6bb1] # 0x142a96730
1409bfb7f: 48 8d 85 10 0a 00 00        	lea	rax, [rbp + 0xa10]
1409bfb86: 0f 11 48 10                 	movups	xmmword ptr [rax + 0x10], xmm1
1409bfb8a: 0f 10 0d 8f 6b 0d 02        	movups	xmm1, xmmword ptr [rip + 0x20d6b8f] # 0x142a96720
1409bfb91: 0f 11 08                    	movups	xmmword ptr [rax], xmm1
1409bfb94: 0f 11 85 30 0a 00 00        	movups	xmmword ptr [rbp + 0xa30], xmm0
1409bfb9b: 48 b8 03 00 00 00 01 00 00 00       	movabs	rax, 0x100000003
1409bfba5: 48 89 85 e0 09 00 00        	mov	qword ptr [rbp + 0x9e0], rax
1409bfbac: 48 c7 85 c8 09 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x9c8], 0x0
1409bfbb7: 48 c7 85 d0 09 00 00 01 00 00 00    	mov	qword ptr [rbp + 0x9d0], 0x1
1409bfbc2: 48 c7 85 d8 09 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x9d8], 0x0
1409bfbcd: c6 85 9f 0b 00 00 01        	mov	byte ptr [rbp + 0xb9f], 0x1
1409bfbd4: 4c 89 e1                    	mov	rcx, r12
1409bfbd7: 4c 89 fa                    	mov	rdx, r15
1409bfbda: 4c 8d 85 c0 01 00 00        	lea	r8, [rbp + 0x1c0]
1409bfbe1: 45 31 c9                    	xor	r9d, r9d
1409bfbe4: e8 f7 5f f4 ff              	call	0x140905be0 <.text+0x904be0>
1409bfbe9: c6 85 9f 0b 00 00 01        	mov	byte ptr [rbp + 0xb9f], 0x1
1409bfbf0: 4c 89 f9                    	mov	rcx, r15
1409bfbf3: 4c 89 e2                    	mov	rdx, r12
1409bfbf6: 4c 8d 85 80 0b 00 00        	lea	r8, [rbp + 0xb80]
1409bfbfd: 4d 89 e9                    	mov	r9, r13
1409bfc00: e8 5b b8 f4 ff              	call	0x14090b460 <.text+0x90a460>
1409bfc05: 48 8b 85 10 0c 00 00        	mov	rax, qword ptr [rbp + 0xc10]
1409bfc0c: 48 89 85 88 0b 00 00        	mov	qword ptr [rbp + 0xb88], rax
1409bfc13: c7 85 b0 05 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x5b0], 0x2
1409bfc1d: 4c 89 64 24 20              	mov	qword ptr [rsp + 0x20], r12
1409bfc22: 48 89 f7                    	mov	rdi, rsi
1409bfc25: 48 89 f1                    	mov	rcx, rsi
1409bfc28: 4c 89 fa                    	mov	rdx, r15
1409bfc2b: 49 b8 30 82 02 28 3a 3c 33 2a       	movabs	r8, 0x2a333c3a28028230
1409bfc35: 41 b1 1d                    	mov	r9b, 0x1d
1409bfc38: e8 c3 eb f2 ff              	call	0x1408ee800 <.text+0x8ed800>
1409bfc3d: 41 b8 f8 00 00 00           	mov	r8d, 0xf8
1409bfc43: 4c 89 e1                    	mov	rcx, r12
1409bfc46: 4c 89 fa                    	mov	rdx, r15
1409bfc49: e8 b2 3f f6 01              	call	0x142923c00 <.text+0x2922c00>
1409bfc4e: 48 8b 85 18 0c 00 00        	mov	rax, qword ptr [rbp + 0xc18]
1409bfc55: 48 89 85 88 0b 00 00        	mov	qword ptr [rbp + 0xb88], rax
1409bfc5c: c7 85 a0 09 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x9a0], 0x2
1409bfc66: 4c 89 7c 24 20              	mov	qword ptr [rsp + 0x20], r15
1409bfc6b: 48 89 f9                    	mov	rcx, rdi
1409bfc6e: 4c 89 e2                    	mov	rdx, r12
1409bfc71: 49 b8 d0 09 6c 82 85 51 17 04       	movabs	r8, 0x4175185826c09d0
1409bfc7b: 41 b1 1d                    	mov	r9b, 0x1d
1409bfc7e: e8 7d eb f2 ff              	call	0x1408ee800 <.text+0x8ed800>
1409bfc83: 41 b8 f8 00 00 00           	mov	r8d, 0xf8
1409bfc89: 48 8d 8d c8 00 00 00        	lea	rcx, [rbp + 0xc8]
1409bfc90: 4c 89 e2                    	mov	rdx, r12
1409bfc93: e8 68 3f f6 01              	call	0x142923c00 <.text+0x2922c00>
1409bfc98: 80 bd a6 01 00 00 08        	cmp	byte ptr [rbp + 0x1a6], 0x8
1409bfc9f: 74 2c                       	je	0x1409bfccd <.text+0x9beccd>
1409bfca1: 48 8b 95 b0 01 00 00        	mov	rdx, qword ptr [rbp + 0x1b0]
1409bfca8: b8 e8 00 00 00              	mov	eax, 0xe8
1409bfcad: 48 01 c2                    	add	rdx, rax
1409bfcb0: 48 8d 8d 80 01 00 00        	lea	rcx, [rbp + 0x180]
1409bfcb7: 49 b8 02 d9 ee 60 b7 09 a1 a9       	movabs	r8, -0x565ef6489f1126fe
1409bfcc1: 44 8b 8d 98 0b 00 00        	mov	r9d, dword ptr [rbp + 0xb98]
1409bfcc8: e8 a3 b8 fa ff              	call	0x14096b570 <.text+0x96a570>
1409bfccd: 41 b8 f8 00 00 00           	mov	r8d, 0xf8
1409bfcd3: 4c 89 e1                    	mov	rcx, r12
1409bfcd6: 48 8d bd c8 00 00 00        	lea	rdi, [rbp + 0xc8]
1409bfcdd: 48 89 fa                    	mov	rdx, rdi
1409bfce0: e8 1b 3f f6 01              	call	0x142923c00 <.text+0x2922c00>
1409bfce5: 48 8b 85 30 0c 00 00        	mov	rax, qword ptr [rbp + 0xc30]
1409bfcec: 48 89 85 c8 00 00 00        	mov	qword ptr [rbp + 0xc8], rax
1409bfcf3: c7 85 a0 09 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x9a0], 0x2
1409bfcfd: 4c 89 7c 24 20              	mov	qword ptr [rsp + 0x20], r15
1409bfd02: 48 89 f9                    	mov	rcx, rdi
1409bfd05: 4c 89 e2                    	mov	rdx, r12
1409bfd08: 49 b8 e3 6b 5a 83 cd 4c a7 b8       	movabs	r8, -0x4758b3327ca5941d
1409bfd12: 41 b1 1d                    	mov	r9b, 0x1d
1409bfd15: e8 e6 ea f2 ff              	call	0x1408ee800 <.text+0x8ed800>
1409bfd1a: 41 b8 f8 00 00 00           	mov	r8d, 0xf8
1409bfd20: 4c 89 f9                    	mov	rcx, r15
1409bfd23: 4c 89 e2                    	mov	rdx, r12
1409bfd26: e8 d5 3e f6 01              	call	0x142923c00 <.text+0x2922c00>
1409bfd2b: c6 85 9f 0b 00 00 01        	mov	byte ptr [rbp + 0xb9f], 0x1
1409bfd32: 4c 89 e1                    	mov	rcx, r12
1409bfd35: 4c 89 fa                    	mov	rdx, r15
1409bfd38: 49 b8 4e 0b fe 6e 0d d1 63 18       	movabs	r8, 0x1863d10d6efe0b4e
1409bfd42: 4c 8d 8d 60 0b 00 00        	lea	r9, [rbp + 0xb60]
1409bfd49: e8 92 6a f4 ff              	call	0x1409067e0 <.text+0x9057e0>
1409bfd4e: c6 85 9f 0b 00 00 01        	mov	byte ptr [rbp + 0xb9f], 0x1
1409bfd55: 48 8d 4d d0                 	lea	rcx, [rbp - 0x30]
1409bfd59: 4c 89 e2                    	mov	rdx, r12
1409bfd5c: 49 b8 7b c8 66 79 e3 94 7b 8f       	movabs	r8, -0x70846b1c86993785
1409bfd66: 4c 8d 8d 30 0b 00 00        	lea	r9, [rbp + 0xb30]
1409bfd6d: e8 6e 6a f4 ff              	call	0x1409067e0 <.text+0x9057e0>
1409bfd72: 48 8b 85 28 0c 00 00        	mov	rax, qword ptr [rbp + 0xc28]
1409bfd79: 48 89 85 a0 09 00 00        	mov	qword ptr [rbp + 0x9a0], rax
1409bfd80: c7 85 b0 05 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x5b0], 0x2
1409bfd8a: 4c 89 64 24 20              	mov	qword ptr [rsp + 0x20], r12
1409bfd8f: 4c 89 f9                    	mov	rcx, r15
1409bfd92: 48 8d 55 d0                 	lea	rdx, [rbp - 0x30]
1409bfd96: 49 b8 3c 4e 88 28 8a 8e 20 e9       	movabs	r8, -0x16df7175d777b1c4
1409bfda0: 41 b1 1d                    	mov	r9b, 0x1d
1409bfda3: e8 58 ea f2 ff              	call	0x1408ee800 <.text+0x8ed800>
1409bfda8: 48 8b 7d d0                 	mov	rdi, qword ptr [rbp - 0x30]
1409bfdac: 41 b8 a3 00 00 00           	mov	r8d, 0xa3
1409bfdb2: 4c 8d b5 b8 05 00 00        	lea	r14, [rbp + 0x5b8]
1409bfdb9: 4c 89 f1                    	mov	rcx, r14
1409bfdbc: 48 8d 5d d8                 	lea	rbx, [rbp - 0x28]
1409bfdc0: 48 89 da                    	mov	rdx, rbx
1409bfdc3: e8 38 3e f6 01              	call	0x142923c00 <.text+0x2922c00>
1409bfdc8: 0f b6 45 7b                 	movzx	eax, byte ptr [rbp + 0x7b]
1409bfdcc: 0f 10 83 a4 00 00 00        	movups	xmm0, xmmword ptr [rbx + 0xa4]
1409bfdd3: 0f 10 8b b4 00 00 00        	movups	xmm1, xmmword ptr [rbx + 0xb4]
1409bfdda: 0f 10 93 c4 00 00 00        	movups	xmm2, xmmword ptr [rbx + 0xc4]
1409bfde1: 0f 10 9b d4 00 00 00        	movups	xmm3, xmmword ptr [rbx + 0xd4]
1409bfde8: 41 0f 11 86 a4 00 00 00     	movups	xmmword ptr [r14 + 0xa4], xmm0
1409bfdf0: 41 0f 11 8e b4 00 00 00     	movups	xmmword ptr [r14 + 0xb4], xmm1
1409bfdf8: 41 0f 11 96 c4 00 00 00     	movups	xmmword ptr [r14 + 0xc4], xmm2
1409bfe00: 41 0f 11 9e d4 00 00 00     	movups	xmmword ptr [r14 + 0xd4], xmm3
1409bfe08: 0f 10 83 e0 00 00 00        	movups	xmm0, xmmword ptr [rbx + 0xe0]
1409bfe0f: 41 0f 11 86 e0 00 00 00     	movups	xmmword ptr [r14 + 0xe0], xmm0
1409bfe17: 24 9f                       	and	al, -0x61
1409bfe19: 0c 40                       	or	al, 0x40
1409bfe1b: 4d 8b 4d 70                 	mov	r9, qword ptr [r13 + 0x70]
1409bfe1f: 48 89 bd b0 05 00 00        	mov	qword ptr [rbp + 0x5b0], rdi
1409bfe26: 88 85 5b 06 00 00           	mov	byte ptr [rbp + 0x65b], al
1409bfe2c: c6 85 9f 0b 00 00 01        	mov	byte ptr [rbp + 0xb9f], 0x1
1409bfe33: 4c 89 e1                    	mov	rcx, r12
1409bfe36: 4c 89 ea                    	mov	rdx, r13
1409bfe39: 4c 8b 85 90 0b 00 00        	mov	r8, qword ptr [rbp + 0xb90]
1409bfe40: 41 ff d1                    	call	r9
1409bfe43: 49 83 ed 80                 	sub	r13, -0x80
1409bfe47: 4c 3b ad 48 0b 00 00        	cmp	r13, qword ptr [rbp + 0xb48]
1409bfe4e: 0f 85 9c fc ff ff           	jne	0x1409bfaf0 <.text+0x9beaf0>
1409bfe54: 48 8d b5 b0 05 00 00        	lea	rsi, [rbp + 0x5b0]
1409bfe5b: 48 8d 95 c0 01 00 00        	lea	rdx, [rbp + 0x1c0]
1409bfe62: 41 b8 e8 03 00 00           	mov	r8d, 0x3e8
1409bfe68: 48 89 f1                    	mov	rcx, rsi
1409bfe6b: e8 90 3d f6 01              	call	0x142923c00 <.text+0x2922c00>
1409bfe70: c6 85 9f 0b 00 00 00        	mov	byte ptr [rbp + 0xb9f], 0x0
1409bfe77: 48 89 f1                    	mov	rcx, rsi
1409bfe7a: e8 f1 84 ea ff              	call	0x140868370 <.text+0x867370>
1409bfe7f: 48 8b bd 50 0b 00 00        	mov	rdi, qword ptr [rbp + 0xb50]
1409bfe86: 48 89 7c 24 30              	mov	qword ptr [rsp + 0x30], rdi
1409bfe8b: 4c 8b b5 90 0b 00 00        	mov	r14, qword ptr [rbp + 0xb90]
1409bfe92: 4c 89 74 24 28              	mov	qword ptr [rsp + 0x28], r14
1409bfe97: 48 c7 44 24 20 24 00 00 00  	mov	qword ptr [rsp + 0x20], 0x24
1409bfea0: 48 8d 15 7d fd 0f 02        	lea	rdx, [rip + 0x20ffd7d]  # 0x142abfc24
1409bfea7: 4c 8d 0d 8c fd 0f 02        	lea	r9, [rip + 0x20ffd8c]   # 0x142abfc3a
1409bfeae: 41 b8 16 00 00 00           	mov	r8d, 0x16
1409bfeb4: 48 89 f1                    	mov	rcx, rsi
1409bfeb7: e8 e4 33 c4 00              	call	0x1416032a0 <.text+0x16022a0>
1409bfebc: 48 8d 8d c0 01 00 00        	lea	rcx, [rbp + 0x1c0]
1409bfec3: 49 b8 4e 0b fe 6e 0d d1 63 18       	movabs	r8, 0x1863d10d6efe0b4e
1409bfecd: 4c 8d 8d 60 0b 00 00        	lea	r9, [rbp + 0xb60]
1409bfed4: 48 89 f2                    	mov	rdx, rsi
1409bfed7: e8 84 de fa ff              	call	0x14096dd60 <.text+0x96cd60>
1409bfedc: 0f 10 85 60 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x260]
1409bfee3: 0f 29 85 e0 09 00 00        	movaps	xmmword ptr [rbp + 0x9e0], xmm0
1409bfeea: 0f 10 85 50 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x250]
1409bfef1: 0f 29 85 d0 09 00 00        	movaps	xmmword ptr [rbp + 0x9d0], xmm0
1409bfef8: 0f 10 85 40 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x240]
1409bfeff: 0f 29 85 c0 09 00 00        	movaps	xmmword ptr [rbp + 0x9c0], xmm0
1409bff06: 0f 10 85 30 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x230]
1409bff0d: 0f 29 85 b0 09 00 00        	movaps	xmmword ptr [rbp + 0x9b0], xmm0
1409bff14: 0f 10 85 20 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x220]
1409bff1b: 0f 29 85 a0 09 00 00        	movaps	xmmword ptr [rbp + 0x9a0], xmm0
1409bff22: 48 8b 85 f8 02 00 00        	mov	rax, qword ptr [rbp + 0x2f8]
1409bff29: 48 8b 8d 40 03 00 00        	mov	rcx, qword ptr [rbp + 0x340]
1409bff30: 48 8b 95 d0 01 00 00        	mov	rdx, qword ptr [rbp + 0x1d0]
1409bff37: 48 89 95 c0 05 00 00        	mov	qword ptr [rbp + 0x5c0], rdx
1409bff3e: 0f 10 85 c0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1c0]
1409bff45: 0f 29 85 b0 05 00 00        	movaps	xmmword ptr [rbp + 0x5b0], xmm0
1409bff4c: 0f 10 85 d8 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1d8]
1409bff53: 0f 29 85 e0 05 00 00        	movaps	xmmword ptr [rbp + 0x5e0], xmm0
1409bff5a: 48 8b 95 e8 01 00 00        	mov	rdx, qword ptr [rbp + 0x1e8]
1409bff61: 48 89 95 f0 05 00 00        	mov	qword ptr [rbp + 0x5f0], rdx
1409bff68: 0f 10 85 f0 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x1f0]
1409bff6f: 0f 11 85 c8 05 00 00        	movups	xmmword ptr [rbp + 0x5c8], xmm0
1409bff76: 48 8b 95 00 02 00 00        	mov	rdx, qword ptr [rbp + 0x200]
1409bff7d: 48 89 95 d8 05 00 00        	mov	qword ptr [rbp + 0x5d8], rdx
1409bff84: 0f 10 85 70 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x270]
1409bff8b: 0f 10 8d 80 02 00 00        	movups	xmm1, xmmword ptr [rbp + 0x280]
1409bff92: 0f 10 95 90 02 00 00        	movups	xmm2, xmmword ptr [rbp + 0x290]
1409bff99: 0f 10 9d a0 02 00 00        	movups	xmm3, xmmword ptr [rbp + 0x2a0]
1409bffa0: 0f 11 85 28 06 00 00        	movups	xmmword ptr [rbp + 0x628], xmm0
1409bffa7: 0f 11 8d 38 06 00 00        	movups	xmmword ptr [rbp + 0x638], xmm1
1409bffae: 0f 11 95 48 06 00 00        	movups	xmmword ptr [rbp + 0x648], xmm2
1409bffb5: 0f 11 9d 58 06 00 00        	movups	xmmword ptr [rbp + 0x658], xmm3
1409bffbc: 0f 10 85 b0 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2b0]
1409bffc3: 0f 11 85 68 06 00 00        	movups	xmmword ptr [rbp + 0x668], xmm0
1409bffca: 0f 10 85 c0 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x2c0]
1409bffd1: 0f 11 85 78 06 00 00        	movups	xmmword ptr [rbp + 0x678], xmm0
1409bffd8: 48 8b 95 18 02 00 00        	mov	rdx, qword ptr [rbp + 0x218]
1409bffdf: 48 89 95 08 06 00 00        	mov	qword ptr [rbp + 0x608], rdx
1409bffe6: 0f 10 85 08 02 00 00        	movups	xmm0, xmmword ptr [rbp + 0x208]
1409bffed: 0f 11 85 f8 05 00 00        	movups	xmmword ptr [rbp + 0x5f8], xmm0
1409bfff4: 48 89 8d 90 06 00 00        	mov	qword ptr [rbp + 0x690], rcx
1409bfffb: 48 89 85 88 06 00 00        	mov	qword ptr [rbp + 0x688], rax
1409c0002: 0f 28 05 d7 8f f9 01        	movaps	xmm0, xmmword ptr [rip + 0x1f98fd7] # 0x142958fe0
1409c0009: 0f 29 85 10 06 00 00        	movaps	xmmword ptr [rbp + 0x610], xmm0
1409c0010: 48 8d 8d a0 09 00 00        	lea	rcx, [rbp + 0x9a0]
1409c0017: 48 8d 95 b0 05 00 00        	lea	rdx, [rbp + 0x5b0]
1409c001e: e8 ad d4 01 00              	call	0x1409dd4d0 <.text+0x9dc4d0>
1409c0023: 80 bd f6 02 00 00 08        	cmp	byte ptr [rbp + 0x2f6], 0x8
1409c002a: 74 0c                       	je	0x1409c0038 <.text+0x9bf038>
1409c002c: 48 8d 8d e0 02 00 00        	lea	rcx, [rbp + 0x2e0]
1409c0033: e8 78 24 22 00              	call	0x140be24b0 <.text+0xbe14b0>
1409c0038: 48 89 7c 24 30              	mov	qword ptr [rsp + 0x30], rdi
1409c003d: 4c 89 74 24 28              	mov	qword ptr [rsp + 0x28], r14
1409c0042: 48 c7 44 24 20 1a 00 00 00  	mov	qword ptr [rsp + 0x20], 0x1a
1409c004b: 48 8d 15 0c fc 0f 02        	lea	rdx, [rip + 0x20ffc0c]  # 0x142abfc5e
1409c0052: 4c 8d 0d 16 fc 0f 02        	lea	r9, [rip + 0x20ffc16]   # 0x142abfc6f
1409c0059: 41 b8 11 00 00 00           	mov	r8d, 0x11
1409c005f: 48 89 f1                    	mov	rcx, rsi
1409c0062: e8 39 32 c4 00              	call	0x1416032a0 <.text+0x16022a0>
1409c0067: 48 8d bd c0 01 00 00        	lea	rdi, [rbp + 0x1c0]
1409c006e: 49 b8 4e 0b fe 6e 0d d1 63 18       	movabs	r8, 0x1863d10d6efe0b4e
1409c0078: 4c 8d 8d 60 0b 00 00        	lea	r9, [rbp + 0xb60]
1409c007f: 48 89 f9                    	mov	rcx, rdi
1409c0082: 48 89 f2                    	mov	rdx, rsi
1409c0085: e8 36 da fa ff              	call	0x14096dac0 <.text+0x96cac0>
1409c008a: 49 b8 7b c8 66 79 e3 94 7b 8f       	movabs	r8, -0x70846b1c86993785
1409c0094: 4c 8d 8d 30 0b 00 00        	lea	r9, [rbp + 0xb30]
1409c009b: 48 89 f1                    	mov	rcx, rsi
1409c009e: 48 89 fa                    	mov	rdx, rdi
1409c00a1: e8 1a da fa ff              	call	0x14096dac0 <.text+0x96cac0>
1409c00a6: 48 8b 85 10 0c 00 00        	mov	rax, qword ptr [rbp + 0xc10]
1409c00ad: 48 89 85 a0 09 00 00        	mov	qword ptr [rbp + 0x9a0], rax
1409c00b4: c7 85 c0 01 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x1c0], 0x2
1409c00be: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
1409c00c3: 48 8d 8d a0 09 00 00        	lea	rcx, [rbp + 0x9a0]
1409c00ca: 49 b8 30 82 02 28 3a 3c 33 2a       	movabs	r8, 0x2a333c3a28028230
1409c00d4: 48 89 f2                    	mov	rdx, rsi
1409c00d7: 41 b1 19                    	mov	r9b, 0x19
1409c00da: e8 61 e1 f2 ff              	call	0x1408ee240 <.text+0x8ed240>
1409c00df: 48 8d b5 c0 01 00 00        	lea	rsi, [rbp + 0x1c0]
1409c00e6: 48 8d bd b0 05 00 00        	lea	rdi, [rbp + 0x5b0]
1409c00ed: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c00f3: 48 89 f1                    	mov	rcx, rsi
1409c00f6: 48 89 fa                    	mov	rdx, rdi
1409c00f9: e8 02 3b f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c00fe: 48 8b 85 18 0c 00 00        	mov	rax, qword ptr [rbp + 0xc18]
1409c0105: 48 89 85 a0 09 00 00        	mov	qword ptr [rbp + 0x9a0], rax
1409c010c: c7 85 b0 05 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x5b0], 0x2
1409c0116: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
1409c011b: 48 8d 8d a0 09 00 00        	lea	rcx, [rbp + 0x9a0]
1409c0122: 49 b8 d0 09 6c 82 85 51 17 04       	movabs	r8, 0x4175185826c09d0
1409c012c: 48 89 f2                    	mov	rdx, rsi
1409c012f: 41 b1 19                    	mov	r9b, 0x19
1409c0132: e8 09 e1 f2 ff              	call	0x1408ee240 <.text+0x8ed240>
1409c0137: 48 8d 8d b0 05 00 00        	lea	rcx, [rbp + 0x5b0]
1409c013e: 48 8d 95 c0 01 00 00        	lea	rdx, [rbp + 0x1c0]
1409c0145: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c014b: e8 b0 3a f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c0150: 80 bd e6 06 00 00 08        	cmp	byte ptr [rbp + 0x6e6], 0x8
1409c0157: 74 29                       	je	0x1409c0182 <.text+0x9bf182>
1409c0159: 48 8d 8d c0 06 00 00        	lea	rcx, [rbp + 0x6c0]
1409c0160: ba e8 00 00 00              	mov	edx, 0xe8
1409c0165: 48 03 95 e8 06 00 00        	add	rdx, qword ptr [rbp + 0x6e8]
1409c016c: 49 b8 02 d9 ee 60 b7 09 a1 a9       	movabs	r8, -0x565ef6489f1126fe
1409c0176: 44 8b 8d 98 0b 00 00        	mov	r9d, dword ptr [rbp + 0xb98]
1409c017d: e8 ee b3 fa ff              	call	0x14096b570 <.text+0x96a570>
1409c0182: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c0188: 48 89 f1                    	mov	rcx, rsi
1409c018b: 48 89 fa                    	mov	rdx, rdi
1409c018e: e8 6d 3a f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c0193: 48 8b 85 30 0c 00 00        	mov	rax, qword ptr [rbp + 0xc30]
1409c019a: 48 89 85 a0 09 00 00        	mov	qword ptr [rbp + 0x9a0], rax
1409c01a1: c7 85 b0 05 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x5b0], 0x2
1409c01ab: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
1409c01b0: 48 8d 8d a0 09 00 00        	lea	rcx, [rbp + 0x9a0]
1409c01b7: 49 b8 e3 6b 5a 83 cd 4c a7 b8       	movabs	r8, -0x4758b3327ca5941d
1409c01c1: 48 89 f2                    	mov	rdx, rsi
1409c01c4: 41 b1 19                    	mov	r9b, 0x19
1409c01c7: e8 74 e0 f2 ff              	call	0x1408ee240 <.text+0x8ed240>
1409c01cc: 48 8d b5 b0 05 00 00        	lea	rsi, [rbp + 0x5b0]
1409c01d3: 48 8d bd c0 01 00 00        	lea	rdi, [rbp + 0x1c0]
1409c01da: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c01e0: 48 89 f1                    	mov	rcx, rsi
1409c01e3: 48 89 fa                    	mov	rdx, rdi
1409c01e6: e8 15 3a f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c01eb: 48 bb 65 18 a0 91 ab 39 9f 57       	movabs	rbx, 0x579f39ab91a01865
1409c01f5: 48 89 9d a0 09 00 00        	mov	qword ptr [rbp + 0x9a0], rbx
1409c01fc: c7 85 c0 01 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x1c0], 0x2
1409c0206: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
1409c020b: 48 8d 8d a0 09 00 00        	lea	rcx, [rbp + 0x9a0]
1409c0212: 49 b8 8c 97 66 69 03 f5 5b 81       	movabs	r8, -0x7ea40afc96996874
1409c021c: 48 89 f2                    	mov	rdx, rsi
1409c021f: 41 b1 19                    	mov	r9b, 0x19
1409c0222: e8 a9 54 f4 ff              	call	0x1409056d0 <.text+0x9046d0>
1409c0227: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c022d: 48 89 f9                    	mov	rcx, rdi
1409c0230: 48 89 f2                    	mov	rdx, rsi
1409c0233: e8 c8 39 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c0238: c6 85 a0 09 00 00 09        	mov	byte ptr [rbp + 0x9a0], 0x9
1409c023f: c7 85 b0 05 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x5b0], 0x2
1409c0249: 48 89 74 24 20              	mov	qword ptr [rsp + 0x20], rsi
1409c024e: 48 8d b5 a0 09 00 00        	lea	rsi, [rbp + 0x9a0]
1409c0255: 49 b8 f2 c5 70 04 5e 8b b0 61       	movabs	r8, 0x61b08b5e0470c5f2
1409c025f: 48 89 f1                    	mov	rcx, rsi
1409c0262: 48 89 fa                    	mov	rdx, rdi
1409c0265: 41 b1 19                    	mov	r9b, 0x19
1409c0268: e8 f3 4f f4 ff              	call	0x140905260 <.text+0x904260>
1409c026d: 48 8d bd b0 05 00 00        	lea	rdi, [rbp + 0x5b0]
1409c0274: 48 8d 95 c0 01 00 00        	lea	rdx, [rbp + 0x1c0]
1409c027b: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c0281: 48 89 f9                    	mov	rcx, rdi
1409c0284: e8 77 39 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c0289: c6 85 c8 00 00 00 0a        	mov	byte ptr [rbp + 0xc8], 0xa
1409c0290: c7 85 a0 09 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x9a0], 0x2
1409c029a: 48 89 74 24 20              	mov	qword ptr [rsp + 0x20], rsi
1409c029f: 48 8d 8d c8 00 00 00        	lea	rcx, [rbp + 0xc8]
1409c02a6: 49 b8 57 f6 e8 da 4c 1e 7d 29       	movabs	r8, 0x297d1e4cdae8f657
1409c02b0: 48 89 fa                    	mov	rdx, rdi
1409c02b3: 41 b1 19                    	mov	r9b, 0x19
1409c02b6: e8 a5 4f f4 ff              	call	0x140905260 <.text+0x904260>
1409c02bb: 48 8d bd a0 09 00 00        	lea	rdi, [rbp + 0x9a0]
1409c02c2: 48 8d b5 b0 05 00 00        	lea	rsi, [rbp + 0x5b0]
1409c02c9: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c02cf: 48 89 f9                    	mov	rcx, rdi
1409c02d2: 48 89 f2                    	mov	rdx, rsi
1409c02d5: e8 26 39 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c02da: c6 85 c0 01 00 00 08        	mov	byte ptr [rbp + 0x1c0], 0x8
1409c02e1: c7 85 b0 05 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x5b0], 0x2
1409c02eb: 48 89 74 24 20              	mov	qword ptr [rsp + 0x20], rsi
1409c02f0: 48 8d b5 c0 01 00 00        	lea	rsi, [rbp + 0x1c0]
1409c02f7: 49 b8 3f ca 8d ac 13 01 16 77       	movabs	r8, 0x77160113ac8dca3f
1409c0301: 48 89 f1                    	mov	rcx, rsi
1409c0304: 48 89 fa                    	mov	rdx, rdi
1409c0307: 41 b1 19                    	mov	r9b, 0x19
1409c030a: e8 51 4f f4 ff              	call	0x140905260 <.text+0x904260>
1409c030f: 48 8d bd b0 05 00 00        	lea	rdi, [rbp + 0x5b0]
1409c0316: 48 8d 95 a0 09 00 00        	lea	rdx, [rbp + 0x9a0]
1409c031d: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c0323: 48 89 f9                    	mov	rcx, rdi
1409c0326: e8 d5 38 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c032b: 48 89 9d c8 00 00 00        	mov	qword ptr [rbp + 0xc8], rbx
1409c0332: c7 85 c0 01 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x1c0], 0x2
1409c033c: 48 89 74 24 20              	mov	qword ptr [rsp + 0x20], rsi
1409c0341: 48 8d 8d c8 00 00 00        	lea	rcx, [rbp + 0xc8]
1409c0348: 49 b8 8c 97 66 69 03 f5 5b 81       	movabs	r8, -0x7ea40afc96996874
1409c0352: 48 89 fa                    	mov	rdx, rdi
1409c0355: 41 b1 19                    	mov	r9b, 0x19
1409c0358: e8 73 53 f4 ff              	call	0x1409056d0 <.text+0x9046d0>
1409c035d: 48 8d 95 b0 05 00 00        	lea	rdx, [rbp + 0x5b0]
1409c0364: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c036a: 48 89 f1                    	mov	rcx, rsi
1409c036d: e8 8e 38 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c0372: 48 89 f9                    	mov	rcx, rdi
1409c0375: 48 89 f2                    	mov	rdx, rsi
1409c0378: 4d 89 f0                    	mov	r8, r14
1409c037b: e8 b0 be c3 00              	call	0x1415fc230 <.text+0x15fb230>
1409c0380: 48 8b 85 58 0b 00 00        	mov	rax, qword ptr [rbp + 0xb58]
1409c0387: 48 89 85 a0 09 00 00        	mov	qword ptr [rbp + 0x9a0], rax
1409c038e: c7 85 c0 01 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x1c0], 0x2
1409c0398: 48 89 74 24 20              	mov	qword ptr [rsp + 0x20], rsi
1409c039d: 48 8d 8d a0 09 00 00        	lea	rcx, [rbp + 0x9a0]
1409c03a4: 49 b8 5e a9 e9 1c 54 8e 97 d9       	movabs	r8, -0x266871abe31656a2
1409c03ae: 48 89 fa                    	mov	rdx, rdi
1409c03b1: 41 b1 32                    	mov	r9b, 0x32
1409c03b4: e8 47 d9 f2 ff              	call	0x1408edd00 <.text+0x8ecd00>
1409c03b9: 48 8d 95 b0 05 00 00        	lea	rdx, [rbp + 0x5b0]
1409c03c0: 41 b8 90 01 00 00           	mov	r8d, 0x190
1409c03c6: 48 89 f1                    	mov	rcx, rsi
1409c03c9: e8 32 38 f6 01              	call	0x142923c00 <.text+0x2922c00>
1409c03ce: 48 8d 95 60 0b 00 00        	lea	rdx, [rbp + 0xb60]
1409c03d5: 48 89 f1                    	mov	rcx, rsi
1409c03d8: 45 31 c0                    	xor	r8d, r8d
1409c03db: e8 a0 e4 fa ff              	call	0x14096e880 <.text+0x96d880>
1409c03e0: 90                          	nop
1409c03e1: 48 81 c4 28 0c 00 00        	add	rsp, 0xc28
1409c03e8: 5b                          	pop	rbx
1409c03e9: 5f                          	pop	rdi
1409c03ea: 5e                          	pop	rsi
1409c03eb: 41 5c                       	pop	r12
1409c03ed: 41 5d                       	pop	r13
1409c03ef: 41 5e                       	pop	r14
1409c03f1: 41 5f                       	pop	r15
1409c03f3: 5d                          	pop	rbp
1409c03f4: c3                          	ret
1409c03f5: c6 85 9f 0b 00 00 01        	mov	byte ptr [rbp + 0xb9f], 0x1
1409c03fc: 48 8d 0d 9d a6 27 02        	lea	rcx, [rip + 0x227a69d]  # 0x142c3aaa0
1409c0403: e8 c8 7c f1 01              	call	0x1428d80d0 <.text+0x28d70d0>
1409c0408: eb 13                       	jmp	0x1409c041d <.text+0x9bf41d>
1409c040a: c6 85 9f 0b 00 00 01        	mov	byte ptr [rbp + 0xb9f], 0x1
1409c0411: 48 8d 0d c0 57 0e 02        	lea	rcx, [rip + 0x20e57c0]  # 0x142aa5bd8
1409c0418: e8 f3 8e f1 01              	call	0x1428d9310 <.text+0x28d8310>
1409c041d: 0f 0b                       	ud2
1409c041f: 90                          	nop
1409c0420: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c0425: 55                          	push	rbp
1409c0426: 41 57                       	push	r15
1409c0428: 41 56                       	push	r14
1409c042a: 41 55                       	push	r13
1409c042c: 41 54                       	push	r12
1409c042e: 56                          	push	rsi
1409c042f: 57                          	push	rdi
1409c0430: 53                          	push	rbx
1409c0431: 48 83 ec 48                 	sub	rsp, 0x48
1409c0435: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c043c: 80 bd f6 02 00 00 08        	cmp	byte ptr [rbp + 0x2f6], 0x8
1409c0443: 74 0c                       	je	0x1409c0451 <.text+0x9bf451>
1409c0445: 48 8d 8d e0 02 00 00        	lea	rcx, [rbp + 0x2e0]
1409c044c: e8 5f 20 22 00              	call	0x140be24b0 <.text+0xbe14b0>
1409c0451: 90                          	nop
1409c0452: 48 83 c4 48                 	add	rsp, 0x48
1409c0456: 5b                          	pop	rbx
1409c0457: 5f                          	pop	rdi
1409c0458: 5e                          	pop	rsi
1409c0459: 41 5c                       	pop	r12
1409c045b: 41 5d                       	pop	r13
1409c045d: 41 5e                       	pop	r14
1409c045f: 41 5f                       	pop	r15
1409c0461: 5d                          	pop	rbp
1409c0462: c3                          	ret
1409c0463: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
1409c0470: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c0475: 55                          	push	rbp
1409c0476: 41 57                       	push	r15
1409c0478: 41 56                       	push	r14
1409c047a: 41 55                       	push	r13
1409c047c: 41 54                       	push	r12
1409c047e: 56                          	push	rsi
1409c047f: 57                          	push	rdi
1409c0480: 53                          	push	rbx
1409c0481: 48 83 ec 48                 	sub	rsp, 0x48
1409c0485: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c048c: 48 8d 8d b0 05 00 00        	lea	rcx, [rbp + 0x5b0]
1409c0493: e8 18 6b ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c0498: 90                          	nop
1409c0499: 48 83 c4 48                 	add	rsp, 0x48
1409c049d: 5b                          	pop	rbx
1409c049e: 5f                          	pop	rdi
1409c049f: 5e                          	pop	rsi
1409c04a0: 41 5c                       	pop	r12
1409c04a2: 41 5d                       	pop	r13
1409c04a4: 41 5e                       	pop	r14
1409c04a6: 41 5f                       	pop	r15
1409c04a8: 5d                          	pop	rbp
1409c04a9: c3                          	ret
1409c04aa: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c04b0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c04b5: 55                          	push	rbp
1409c04b6: 41 57                       	push	r15
1409c04b8: 41 56                       	push	r14
1409c04ba: 41 55                       	push	r13
1409c04bc: 41 54                       	push	r12
1409c04be: 56                          	push	rsi
1409c04bf: 57                          	push	rdi
1409c04c0: 53                          	push	rbx
1409c04c1: 48 83 ec 48                 	sub	rsp, 0x48
1409c04c5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c04cc: 48 8d 8d c0 01 00 00        	lea	rcx, [rbp + 0x1c0]
1409c04d3: e8 d8 6a ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c04d8: 90                          	nop
1409c04d9: 48 83 c4 48                 	add	rsp, 0x48
1409c04dd: 5b                          	pop	rbx
1409c04de: 5f                          	pop	rdi
1409c04df: 5e                          	pop	rsi
1409c04e0: 41 5c                       	pop	r12
1409c04e2: 41 5d                       	pop	r13
1409c04e4: 41 5e                       	pop	r14
1409c04e6: 41 5f                       	pop	r15
1409c04e8: 5d                          	pop	rbp
1409c04e9: c3                          	ret
1409c04ea: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c04f0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c04f5: 55                          	push	rbp
1409c04f6: 41 57                       	push	r15
1409c04f8: 41 56                       	push	r14
1409c04fa: 41 55                       	push	r13
1409c04fc: 41 54                       	push	r12
1409c04fe: 56                          	push	rsi
1409c04ff: 57                          	push	rdi
1409c0500: 53                          	push	rbx
1409c0501: 48 83 ec 48                 	sub	rsp, 0x48
1409c0505: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c050c: 48 8d 8d b0 05 00 00        	lea	rcx, [rbp + 0x5b0]
1409c0513: e8 98 6a ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c0518: 90                          	nop
1409c0519: 48 83 c4 48                 	add	rsp, 0x48
1409c051d: 5b                          	pop	rbx
1409c051e: 5f                          	pop	rdi
1409c051f: 5e                          	pop	rsi
1409c0520: 41 5c                       	pop	r12
1409c0522: 41 5d                       	pop	r13
1409c0524: 41 5e                       	pop	r14
1409c0526: 41 5f                       	pop	r15
1409c0528: 5d                          	pop	rbp
1409c0529: c3                          	ret
1409c052a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c0530: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c0535: 55                          	push	rbp
1409c0536: 41 57                       	push	r15
1409c0538: 41 56                       	push	r14
1409c053a: 41 55                       	push	r13
1409c053c: 41 54                       	push	r12
1409c053e: 56                          	push	rsi
1409c053f: 57                          	push	rdi
1409c0540: 53                          	push	rbx
1409c0541: 48 83 ec 48                 	sub	rsp, 0x48
1409c0545: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c054c: 48 8d 8d c0 01 00 00        	lea	rcx, [rbp + 0x1c0]
1409c0553: e8 58 6a ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c0558: 90                          	nop
1409c0559: 48 83 c4 48                 	add	rsp, 0x48
1409c055d: 5b                          	pop	rbx
1409c055e: 5f                          	pop	rdi
1409c055f: 5e                          	pop	rsi
1409c0560: 41 5c                       	pop	r12
1409c0562: 41 5d                       	pop	r13
1409c0564: 41 5e                       	pop	r14
1409c0566: 41 5f                       	pop	r15
1409c0568: 5d                          	pop	rbp
1409c0569: c3                          	ret
1409c056a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c0570: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c0575: 55                          	push	rbp
1409c0576: 41 57                       	push	r15
1409c0578: 41 56                       	push	r14
1409c057a: 41 55                       	push	r13
1409c057c: 41 54                       	push	r12
1409c057e: 56                          	push	rsi
1409c057f: 57                          	push	rdi
1409c0580: 53                          	push	rbx
1409c0581: 48 83 ec 48                 	sub	rsp, 0x48
1409c0585: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c058c: 48 8d 8d b0 05 00 00        	lea	rcx, [rbp + 0x5b0]
1409c0593: e8 18 6a ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c0598: 90                          	nop
1409c0599: 48 83 c4 48                 	add	rsp, 0x48
1409c059d: 5b                          	pop	rbx
1409c059e: 5f                          	pop	rdi
1409c059f: 5e                          	pop	rsi
1409c05a0: 41 5c                       	pop	r12
1409c05a2: 41 5d                       	pop	r13
1409c05a4: 41 5e                       	pop	r14
1409c05a6: 41 5f                       	pop	r15
1409c05a8: 5d                          	pop	rbp
1409c05a9: c3                          	ret
1409c05aa: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c05b0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c05b5: 55                          	push	rbp
1409c05b6: 41 57                       	push	r15
1409c05b8: 41 56                       	push	r14
1409c05ba: 41 55                       	push	r13
1409c05bc: 41 54                       	push	r12
1409c05be: 56                          	push	rsi
1409c05bf: 57                          	push	rdi
1409c05c0: 53                          	push	rbx
1409c05c1: 48 83 ec 48                 	sub	rsp, 0x48
1409c05c5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c05cc: 48 8d 8d c0 01 00 00        	lea	rcx, [rbp + 0x1c0]
1409c05d3: e8 d8 69 ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c05d8: 90                          	nop
1409c05d9: 48 83 c4 48                 	add	rsp, 0x48
1409c05dd: 5b                          	pop	rbx
1409c05de: 5f                          	pop	rdi
1409c05df: 5e                          	pop	rsi
1409c05e0: 41 5c                       	pop	r12
1409c05e2: 41 5d                       	pop	r13
1409c05e4: 41 5e                       	pop	r14
1409c05e6: 41 5f                       	pop	r15
1409c05e8: 5d                          	pop	rbp
1409c05e9: c3                          	ret
1409c05ea: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c05f0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c05f5: 55                          	push	rbp
1409c05f6: 41 57                       	push	r15
1409c05f8: 41 56                       	push	r14
1409c05fa: 41 55                       	push	r13
1409c05fc: 41 54                       	push	r12
1409c05fe: 56                          	push	rsi
1409c05ff: 57                          	push	rdi
1409c0600: 53                          	push	rbx
1409c0601: 48 83 ec 48                 	sub	rsp, 0x48
1409c0605: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c060c: 48 8d 8d b0 05 00 00        	lea	rcx, [rbp + 0x5b0]
1409c0613: e8 98 69 ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c0618: 90                          	nop
1409c0619: 48 83 c4 48                 	add	rsp, 0x48
1409c061d: 5b                          	pop	rbx
1409c061e: 5f                          	pop	rdi
1409c061f: 5e                          	pop	rsi
1409c0620: 41 5c                       	pop	r12
1409c0622: 41 5d                       	pop	r13
1409c0624: 41 5e                       	pop	r14
1409c0626: 41 5f                       	pop	r15
1409c0628: 5d                          	pop	rbp
1409c0629: c3                          	ret
1409c062a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c0630: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c0635: 55                          	push	rbp
1409c0636: 41 57                       	push	r15
1409c0638: 41 56                       	push	r14
1409c063a: 41 55                       	push	r13
1409c063c: 41 54                       	push	r12
1409c063e: 56                          	push	rsi
1409c063f: 57                          	push	rdi
1409c0640: 53                          	push	rbx
1409c0641: 48 83 ec 48                 	sub	rsp, 0x48
1409c0645: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c064c: 48 8d 8d a0 09 00 00        	lea	rcx, [rbp + 0x9a0]
1409c0653: e8 58 69 ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c0658: 90                          	nop
1409c0659: 48 83 c4 48                 	add	rsp, 0x48
1409c065d: 5b                          	pop	rbx
1409c065e: 5f                          	pop	rdi
1409c065f: 5e                          	pop	rsi
1409c0660: 41 5c                       	pop	r12
1409c0662: 41 5d                       	pop	r13
1409c0664: 41 5e                       	pop	r14
1409c0666: 41 5f                       	pop	r15
1409c0668: 5d                          	pop	rbp
1409c0669: c3                          	ret
1409c066a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c0670: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c0675: 55                          	push	rbp
1409c0676: 41 57                       	push	r15
1409c0678: 41 56                       	push	r14
1409c067a: 41 55                       	push	r13
1409c067c: 41 54                       	push	r12
1409c067e: 56                          	push	rsi
1409c067f: 57                          	push	rdi
1409c0680: 53                          	push	rbx
1409c0681: 48 83 ec 48                 	sub	rsp, 0x48
1409c0685: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c068c: 48 8d 8d b0 05 00 00        	lea	rcx, [rbp + 0x5b0]
1409c0693: e8 18 69 ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c0698: 90                          	nop
1409c0699: 48 83 c4 48                 	add	rsp, 0x48
1409c069d: 5b                          	pop	rbx
1409c069e: 5f                          	pop	rdi
1409c069f: 5e                          	pop	rsi
1409c06a0: 41 5c                       	pop	r12
1409c06a2: 41 5d                       	pop	r13
1409c06a4: 41 5e                       	pop	r14
1409c06a6: 41 5f                       	pop	r15
1409c06a8: 5d                          	pop	rbp
1409c06a9: c3                          	ret
1409c06aa: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c06b0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c06b5: 55                          	push	rbp
1409c06b6: 41 57                       	push	r15
1409c06b8: 41 56                       	push	r14
1409c06ba: 41 55                       	push	r13
1409c06bc: 41 54                       	push	r12
1409c06be: 56                          	push	rsi
1409c06bf: 57                          	push	rdi
1409c06c0: 53                          	push	rbx
1409c06c1: 48 83 ec 48                 	sub	rsp, 0x48
1409c06c5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c06cc: 48 8d 8d b0 05 00 00        	lea	rcx, [rbp + 0x5b0]
1409c06d3: e8 d8 68 ed ff              	call	0x140896fb0 <.text+0x895fb0>
1409c06d8: 90                          	nop
1409c06d9: 48 83 c4 48                 	add	rsp, 0x48
1409c06dd: 5b                          	pop	rbx
1409c06de: 5f                          	pop	rdi
1409c06df: 5e                          	pop	rsi
1409c06e0: 41 5c                       	pop	r12
1409c06e2: 41 5d                       	pop	r13
1409c06e4: 41 5e                       	pop	r14
1409c06e6: 41 5f                       	pop	r15
1409c06e8: 5d                          	pop	rbp
1409c06e9: c3                          	ret
1409c06ea: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409c06f0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c06f5: 55                          	push	rbp
1409c06f6: 41 57                       	push	r15
1409c06f8: 41 56                       	push	r14
1409c06fa: 41 55                       	push	r13
1409c06fc: 41 54                       	push	r12
1409c06fe: 56                          	push	rsi
1409c06ff: 57                          	push	rdi
1409c0700: 53                          	push	rbx
1409c0701: 48 83 ec 48                 	sub	rsp, 0x48
1409c0705: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c070c: 80 bd 9f 0b 00 00 00        	cmp	byte ptr [rbp + 0xb9f], 0x0
1409c0713: 75 11                       	jne	0x1409c0726 <.text+0x9bf726>
1409c0715: 48 83 c4 48                 	add	rsp, 0x48
1409c0719: 5b                          	pop	rbx
1409c071a: 5f                          	pop	rdi
1409c071b: 5e                          	pop	rsi
1409c071c: 41 5c                       	pop	r12
1409c071e: 41 5d                       	pop	r13
1409c0720: 41 5e                       	pop	r14
1409c0722: 41 5f                       	pop	r15
1409c0724: 5d                          	pop	rbp
1409c0725: c3                          	ret
1409c0726: 48 8d 8d c0 01 00 00        	lea	rcx, [rbp + 0x1c0]
1409c072d: e8 8e 1c ed ff              	call	0x1408923c0 <.text+0x8913c0>
1409c0732: eb e1                       	jmp	0x1409c0715 <.text+0x9bf715>
1409c0734: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nop	word ptr cs:[rax + rax]
1409c0740: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c0745: 55                          	push	rbp
1409c0746: 41 57                       	push	r15
1409c0748: 41 56                       	push	r14
1409c074a: 41 55                       	push	r13
1409c074c: 41 54                       	push	r12
1409c074e: 56                          	push	rsi
1409c074f: 57                          	push	rdi
1409c0750: 53                          	push	rbx
1409c0751: 48 83 ec 48                 	sub	rsp, 0x48
1409c0755: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c075c: c6 85 9f 0b 00 00 01        	mov	byte ptr [rbp + 0xb9f], 0x1
1409c0763: 48 8d 8d a0 09 00 00        	lea	rcx, [rbp + 0x9a0]
1409c076a: e8 51 2e ed ff              	call	0x1408935c0 <.text+0x8925c0>
1409c076f: c6 85 9f 0b 00 00 01        	mov	byte ptr [rbp + 0xb9f], 0x1
1409c0776: 48 83 c4 48                 	add	rsp, 0x48
1409c077a: 5b                          	pop	rbx
1409c077b: 5f                          	pop	rdi
1409c077c: 5e                          	pop	rsi
1409c077d: 41 5c                       	pop	r12
1409c077f: 41 5d                       	pop	r13
1409c0781: 41 5e                       	pop	r14
1409c0783: 41 5f                       	pop	r15
1409c0785: 5d                          	pop	rbp
1409c0786: c3                          	ret
1409c0787: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1409c0790: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c0795: 55                          	push	rbp
1409c0796: 41 57                       	push	r15
1409c0798: 41 56                       	push	r14
1409c079a: 41 55                       	push	r13
1409c079c: 41 54                       	push	r12
1409c079e: 56                          	push	rsi
1409c079f: 57                          	push	rdi
1409c07a0: 53                          	push	rbx
1409c07a1: 48 83 ec 48                 	sub	rsp, 0x48
1409c07a5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c07ac: c6 85 9f 0b 00 00 01        	mov	byte ptr [rbp + 0xb9f], 0x1
1409c07b3: 48 8d 8d b0 05 00 00        	lea	rcx, [rbp + 0x5b0]
1409c07ba: e8 01 2e ed ff              	call	0x1408935c0 <.text+0x8925c0>
1409c07bf: c6 85 9f 0b 00 00 01        	mov	byte ptr [rbp + 0xb9f], 0x1
1409c07c6: 48 83 c4 48                 	add	rsp, 0x48
1409c07ca: 5b                          	pop	rbx
1409c07cb: 5f                          	pop	rdi
1409c07cc: 5e                          	pop	rsi
1409c07cd: 41 5c                       	pop	r12
1409c07cf: 41 5d                       	pop	r13
1409c07d1: 41 5e                       	pop	r14
1409c07d3: 41 5f                       	pop	r15
1409c07d5: 5d                          	pop	rbp
1409c07d6: c3                          	ret
1409c07d7: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1409c07e0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c07e5: 55                          	push	rbp
1409c07e6: 41 57                       	push	r15
1409c07e8: 41 56                       	push	r14
1409c07ea: 41 55                       	push	r13
1409c07ec: 41 54                       	push	r12
1409c07ee: 56                          	push	rsi
1409c07ef: 57                          	push	rdi
1409c07f0: 53                          	push	rbx
1409c07f1: 48 83 ec 48                 	sub	rsp, 0x48
1409c07f5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c07fc: c6 85 9f 0b 00 00 01        	mov	byte ptr [rbp + 0xb9f], 0x1
1409c0803: 48 8d 8d c8 00 00 00        	lea	rcx, [rbp + 0xc8]
1409c080a: e8 b1 2d ed ff              	call	0x1408935c0 <.text+0x8925c0>
1409c080f: c6 85 9f 0b 00 00 01        	mov	byte ptr [rbp + 0xb9f], 0x1
1409c0816: 48 83 c4 48                 	add	rsp, 0x48
1409c081a: 5b                          	pop	rbx
1409c081b: 5f                          	pop	rdi
1409c081c: 5e                          	pop	rsi
1409c081d: 41 5c                       	pop	r12
1409c081f: 41 5d                       	pop	r13
1409c0821: 41 5e                       	pop	r14
1409c0823: 41 5f                       	pop	r15
1409c0825: 5d                          	pop	rbp
1409c0826: c3                          	ret
1409c0827: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1409c0830: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c0835: 55                          	push	rbp
1409c0836: 41 57                       	push	r15
1409c0838: 41 56                       	push	r14
1409c083a: 41 55                       	push	r13
1409c083c: 41 54                       	push	r12
1409c083e: 56                          	push	rsi
1409c083f: 57                          	push	rdi
1409c0840: 53                          	push	rbx
1409c0841: 48 83 ec 48                 	sub	rsp, 0x48
1409c0845: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c084c: c6 85 9f 0b 00 00 01        	mov	byte ptr [rbp + 0xb9f], 0x1
1409c0853: 48 8d 8d b0 05 00 00        	lea	rcx, [rbp + 0x5b0]
1409c085a: e8 61 2d ed ff              	call	0x1408935c0 <.text+0x8925c0>
1409c085f: c6 85 9f 0b 00 00 01        	mov	byte ptr [rbp + 0xb9f], 0x1
1409c0866: 48 83 c4 48                 	add	rsp, 0x48
1409c086a: 5b                          	pop	rbx
1409c086b: 5f                          	pop	rdi
1409c086c: 5e                          	pop	rsi
1409c086d: 41 5c                       	pop	r12
1409c086f: 41 5d                       	pop	r13
1409c0871: 41 5e                       	pop	r14
1409c0873: 41 5f                       	pop	r15
1409c0875: 5d                          	pop	rbp
1409c0876: c3                          	ret
1409c0877: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
1409c0880: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409c0885: 55                          	push	rbp
1409c0886: 41 57                       	push	r15
1409c0888: 41 56                       	push	r14
1409c088a: 41 55                       	push	r13
1409c088c: 41 54                       	push	r12
1409c088e: 56                          	push	rsi
1409c088f: 57                          	push	rdi
1409c0890: 53                          	push	rbx
1409c0891: 48 83 ec 48                 	sub	rsp, 0x48
1409c0895: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409c089c: c6 85 9f 0b 00 00 01        	mov	byte ptr [rbp + 0xb9f], 0x1
1409c08a3: 48 8d 4d d0                 	lea	rcx, [rbp - 0x30]
1409c08a7: e8 14 2d ed ff              	call	0x1408935c0 <.text+0x8925c0>
1409c08ac: c6 85 9f 0b 00 00 01        	mov	byte ptr [rbp + 0xb9f], 0x1
1409c08b3: 48 83 c4 48                 	add	rsp, 0x48
1409c08b7: 5b                          	pop	rbx
1409c08b8: 5f                          	pop	rdi
1409c08b9: 5e                          	pop	rsi
1409c08ba: 41 5c                       	pop	r12
1409c08bc: 41 5d                       	pop	r13
1409c08be: 41 5e                       	pop	r14
1409c08c0: 41 5f                       	pop	r15
1409c08c2: 5d                          	pop	rbp
1409c08c3: c3                          	ret
