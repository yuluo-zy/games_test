
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140c04840: 55                          	push	rbp
140c04841: 41 57                       	push	r15
140c04843: 41 56                       	push	r14
140c04845: 41 54                       	push	r12
140c04847: 56                          	push	rsi
140c04848: 57                          	push	rdi
140c04849: 53                          	push	rbx
140c0484a: 48 81 ec 10 01 00 00        	sub	rsp, 0x110
140c04851: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
140c04859: 48 c7 85 88 00 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x88], -0x2
140c04864: 4c 89 c7                    	mov	rdi, r8
140c04867: 49 89 d4                    	mov	r12, rdx
140c0486a: 48 89 ce                    	mov	rsi, rcx
140c0486d: 4c 8d b2 f0 09 00 00        	lea	r14, [rdx + 0x9f0]
140c04874: b1 01                       	mov	cl, 0x1
140c04876: 31 c0                       	xor	eax, eax
140c04878: f0                          	lock
140c04879: 0f b0 8a f0 09 00 00        	cmpxchg	byte ptr [rdx + 0x9f0], cl
140c04880: 0f 85 ce 02 00 00           	jne	0x140c04b54 <.text+0xc03b54>
140c04886: 4c 89 f1                    	mov	rcx, r14
140c04889: e8 e2 d3 07 00              	call	0x140c81c70 <.text+0xc80c70>
140c0488e: 4d 8d bc 24 f8 09 00 00     	lea	r15, [r12 + 0x9f8]
140c04896: 49 8b 84 24 f8 09 00 00     	mov	rax, qword ptr [r12 + 0x9f8]
140c0489e: 48 83 f8 04                 	cmp	rax, 0x4
140c048a2: 0f 85 b7 01 00 00           	jne	0x140c04a5f <.text+0xc03a5f>
140c048a8: 4c 89 75 78                 	mov	qword ptr [rbp + 0x78], r14
140c048ac: 41 8b 84 24 d4 0a 00 00     	mov	eax, dword ptr [r12 + 0xad4]
140c048b4: 89 44 24 20                 	mov	dword ptr [rsp + 0x20], eax
140c048b8: 4c 8d 05 c9 3e f1 01        	lea	r8, [rip + 0x1f13ec9]   # 0x142b18788
140c048bf: 48 8d 4d a8                 	lea	rcx, [rbp - 0x58]
140c048c3: 41 b9 08 00 00 00           	mov	r9d, 0x8
140c048c9: 4c 89 e2                    	mov	rdx, r12
140c048cc: e8 cf ab fe ff              	call	0x140bef4a0 <.text+0xbee4a0>
140c048d1: c6 85 87 00 00 00 01        	mov	byte ptr [rbp + 0x87], 0x1
140c048d8: 48 8d 5d a8                 	lea	rbx, [rbp - 0x58]
140c048dc: 48 89 d9                    	mov	rcx, rbx
140c048df: 4c 89 e2                    	mov	rdx, r12
140c048e2: 45 31 c0                    	xor	r8d, r8d
140c048e5: e8 46 89 fe ff              	call	0x140bed230 <.text+0xbec230>
140c048ea: 48 8b 0f                    	mov	rcx, qword ptr [rdi]
140c048ed: 4c 8b 57 08                 	mov	r10, qword ptr [rdi + 0x8]
140c048f1: 4c 8b 47 10                 	mov	r8, qword ptr [rdi + 0x10]
140c048f5: 4c 8b 4f 18                 	mov	r9, qword ptr [rdi + 0x18]
140c048f9: 48 8b 51 10                 	mov	rdx, qword ptr [rcx + 0x10]
140c048fd: 8b 42 0c                    	mov	eax, dword ptr [rdx + 0xc]
140c04900: 3b 41 18                    	cmp	eax, dword ptr [rcx + 0x18]
140c04903: 75 2c                       	jne	0x140c04931 <.text+0xc03931>
140c04905: 83 7d a8 01                 	cmp	dword ptr [rbp - 0x58], 0x1
140c04909: 0f 85 b7 02 00 00           	jne	0x140c04bc6 <.text+0xc03bc6>
140c0490f: 4c 8b 5d b0                 	mov	r11, qword ptr [rbp - 0x50]
140c04913: 48 8b 02                    	mov	rax, qword ptr [rdx]
140c04916: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
140c04920: 4c 39 d8                    	cmp	rax, r11
140c04923: 4c 89 df                    	mov	rdi, r11
140c04926: 48 0f 47 f8                 	cmova	rdi, rax
140c0492a: f0                          	lock
140c0492b: 48 0f b1 3a                 	cmpxchg	qword ptr [rdx], rdi
140c0492f: 75 ef                       	jne	0x140c04920 <.text+0xc03920>
140c04931: 48 8b 01                    	mov	rax, qword ptr [rcx]
140c04934: 48 8b 4d f0                 	mov	rcx, qword ptr [rbp - 0x10]
140c04938: 48 8b 50 20                 	mov	rdx, qword ptr [rax + 0x20]
140c0493c: 4d 8b 58 10                 	mov	r11, qword ptr [r8 + 0x10]
140c04940: 41 8b 43 0c                 	mov	eax, dword ptr [r11 + 0xc]
140c04944: 41 3b 40 18                 	cmp	eax, dword ptr [r8 + 0x18]
140c04948: 75 27                       	jne	0x140c04971 <.text+0xc03971>
140c0494a: 83 7d a8 01                 	cmp	dword ptr [rbp - 0x58], 0x1
140c0494e: 0f 85 18 03 00 00           	jne	0x140c04c6c <.text+0xc03c6c>
140c04954: 48 8b 7d b0                 	mov	rdi, qword ptr [rbp - 0x50]
140c04958: 49 8b 03                    	mov	rax, qword ptr [r11]
140c0495b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
140c04960: 48 39 f8                    	cmp	rax, rdi
140c04963: 48 89 fb                    	mov	rbx, rdi
140c04966: 48 0f 47 d8                 	cmova	rbx, rax
140c0496a: f0                          	lock
140c0496b: 49 0f b1 1b                 	cmpxchg	qword ptr [r11], rbx
140c0496f: 75 ef                       	jne	0x140c04960 <.text+0xc03960>
140c04971: 49 8b 00                    	mov	rax, qword ptr [r8]
140c04974: 4c 8b 40 20                 	mov	r8, qword ptr [rax + 0x20]
140c04978: 49 8b 01                    	mov	rax, qword ptr [r9]
140c0497b: 48 85 c0                    	test	rax, rax
140c0497e: 74 04                       	je	0x140c04984 <.text+0xc03984>
140c04980: 49 8b 41 08                 	mov	rax, qword ptr [r9 + 0x8]
140c04984: 0f 57 c0                    	xorps	xmm0, xmm0
140c04987: 0f 29 45 20                 	movaps	xmmword ptr [rbp + 0x20], xmm0
140c0498b: 48 89 45 30                 	mov	qword ptr [rbp + 0x30], rax
140c0498f: 48 8d 45 20                 	lea	rax, [rbp + 0x20]
140c04993: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
140c04998: 41 b9 01 00 00 00           	mov	r9d, 0x1
140c0499e: 41 ff 92 70 03 00 00        	call	qword ptr [r10 + 0x370]
140c049a5: 48 8b 45 f8                 	mov	rax, qword ptr [rbp - 0x8]
140c049a9: 48 89 45 70                 	mov	qword ptr [rbp + 0x70], rax
140c049ad: 0f 10 45 e8                 	movups	xmm0, xmmword ptr [rbp - 0x18]
140c049b1: 0f 29 45 60                 	movaps	xmmword ptr [rbp + 0x60], xmm0
140c049b5: 0f 10 45 a8                 	movups	xmm0, xmmword ptr [rbp - 0x58]
140c049b9: 0f 10 4d b8                 	movups	xmm1, xmmword ptr [rbp - 0x48]
140c049bd: 0f 10 55 c8                 	movups	xmm2, xmmword ptr [rbp - 0x38]
140c049c1: 0f 10 5d d8                 	movups	xmm3, xmmword ptr [rbp - 0x28]
140c049c5: 0f 29 5d 50                 	movaps	xmmword ptr [rbp + 0x50], xmm3
140c049c9: 0f 29 55 40                 	movaps	xmmword ptr [rbp + 0x40], xmm2
140c049cd: 0f 29 4d 30                 	movaps	xmmword ptr [rbp + 0x30], xmm1
140c049d1: 0f 29 45 20                 	movaps	xmmword ptr [rbp + 0x20], xmm0
140c049d5: 4c 89 7d 18                 	mov	qword ptr [rbp + 0x18], r15
140c049d9: 49 8b 07                    	mov	rax, qword ptr [r15]
140c049dc: 48 83 f8 04                 	cmp	rax, 0x4
140c049e0: 74 41                       	je	0x140c04a23 <.text+0xc03a23>
140c049e2: 48 85 c0                    	test	rax, rax
140c049e5: 74 09                       	je	0x140c049f0 <.text+0xc039f0>
140c049e7: 83 f8 03                    	cmp	eax, 0x3
140c049ea: 0f 85 cf 02 00 00           	jne	0x140c04cbf <.text+0xc03cbf>
140c049f0: 49 8b 94 24 10 0a 00 00     	mov	rdx, qword ptr [r12 + 0xa10]
140c049f8: 48 85 d2                    	test	rdx, rdx
140c049fb: 74 17                       	je	0x140c04a14 <.text+0xc03a14>
140c049fd: 49 8b 8c 24 18 0a 00 00     	mov	rcx, qword ptr [r12 + 0xa18]
140c04a05: 48 c1 e2 03                 	shl	rdx, 0x3
140c04a09: 41 b8 08 00 00 00           	mov	r8d, 0x8
140c04a0f: e8 0c f2 a0 ff              	call	0x140613c20 <.text+0x612c20>
140c04a14: 49 81 c4 28 0a 00 00        	add	r12, 0xa28
140c04a1b: 4c 89 e1                    	mov	rcx, r12
140c04a1e: e8 dd ab 9d ff              	call	0x1405df600 <.text+0x5de600>
140c04a23: 48 8b 45 70                 	mov	rax, qword ptr [rbp + 0x70]
140c04a27: 48 8b 4d 18                 	mov	rcx, qword ptr [rbp + 0x18]
140c04a2b: 48 89 41 50                 	mov	qword ptr [rcx + 0x50], rax
140c04a2f: 0f 28 45 60                 	movaps	xmm0, xmmword ptr [rbp + 0x60]
140c04a33: 0f 11 41 40                 	movups	xmmword ptr [rcx + 0x40], xmm0
140c04a37: 0f 28 45 20                 	movaps	xmm0, xmmword ptr [rbp + 0x20]
140c04a3b: 0f 28 4d 30                 	movaps	xmm1, xmmword ptr [rbp + 0x30]
140c04a3f: 0f 28 55 40                 	movaps	xmm2, xmmword ptr [rbp + 0x40]
140c04a43: 0f 28 5d 50                 	movaps	xmm3, xmmword ptr [rbp + 0x50]
140c04a47: 0f 11 59 30                 	movups	xmmword ptr [rcx + 0x30], xmm3
140c04a4b: 0f 11 51 20                 	movups	xmmword ptr [rcx + 0x20], xmm2
140c04a4f: 0f 11 49 10                 	movups	xmmword ptr [rcx + 0x10], xmm1
140c04a53: 0f 11 01                    	movups	xmmword ptr [rcx], xmm0
140c04a56: 4c 8b 75 78                 	mov	r14, qword ptr [rbp + 0x78]
140c04a5a: e9 c6 00 00 00              	jmp	0x140c04b25 <.text+0xc03b25>
140c04a5f: 48 8b 0f                    	mov	rcx, qword ptr [rdi]
140c04a62: 4c 8b 57 08                 	mov	r10, qword ptr [rdi + 0x8]
140c04a66: 4c 8b 47 10                 	mov	r8, qword ptr [rdi + 0x10]
140c04a6a: 4c 8b 4f 18                 	mov	r9, qword ptr [rdi + 0x18]
140c04a6e: 48 8b 51 10                 	mov	rdx, qword ptr [rcx + 0x10]
140c04a72: 44 8b 5a 0c                 	mov	r11d, dword ptr [rdx + 0xc]
140c04a76: 44 3b 59 18                 	cmp	r11d, dword ptr [rcx + 0x18]
140c04a7a: 75 25                       	jne	0x140c04aa1 <.text+0xc03aa1>
140c04a7c: 83 f8 01                    	cmp	eax, 0x1
140c04a7f: 0f 85 ee 00 00 00           	jne	0x140c04b73 <.text+0xc03b73>
140c04a85: 4d 8b 9c 24 00 0a 00 00     	mov	r11, qword ptr [r12 + 0xa00]
140c04a8d: 48 8b 02                    	mov	rax, qword ptr [rdx]
140c04a90: 4c 39 d8                    	cmp	rax, r11
140c04a93: 4c 89 df                    	mov	rdi, r11
140c04a96: 48 0f 47 f8                 	cmova	rdi, rax
140c04a9a: f0                          	lock
140c04a9b: 48 0f b1 3a                 	cmpxchg	qword ptr [rdx], rdi
140c04a9f: 75 ef                       	jne	0x140c04a90 <.text+0xc03a90>
140c04aa1: 48 8b 01                    	mov	rax, qword ptr [rcx]
140c04aa4: 49 8b 8c 24 40 0a 00 00     	mov	rcx, qword ptr [r12 + 0xa40]
140c04aac: 48 8b 50 20                 	mov	rdx, qword ptr [rax + 0x20]
140c04ab0: 4d 8b 58 10                 	mov	r11, qword ptr [r8 + 0x10]
140c04ab4: 41 8b 43 0c                 	mov	eax, dword ptr [r11 + 0xc]
140c04ab8: 41 3b 40 18                 	cmp	eax, dword ptr [r8 + 0x18]
140c04abc: 75 33                       	jne	0x140c04af1 <.text+0xc03af1>
140c04abe: 41 83 3f 01                 	cmp	dword ptr [r15], 0x1
140c04ac2: 0f 85 54 01 00 00           	jne	0x140c04c1c <.text+0xc03c1c>
140c04ac8: 49 8b bc 24 00 0a 00 00     	mov	rdi, qword ptr [r12 + 0xa00]
140c04ad0: 49 8b 03                    	mov	rax, qword ptr [r11]
140c04ad3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140c04ae0: 48 39 f8                    	cmp	rax, rdi
140c04ae3: 48 89 fb                    	mov	rbx, rdi
140c04ae6: 48 0f 47 d8                 	cmova	rbx, rax
140c04aea: f0                          	lock
140c04aeb: 49 0f b1 1b                 	cmpxchg	qword ptr [r11], rbx
140c04aef: 75 ef                       	jne	0x140c04ae0 <.text+0xc03ae0>
140c04af1: 49 8b 00                    	mov	rax, qword ptr [r8]
140c04af4: 4c 8b 40 20                 	mov	r8, qword ptr [rax + 0x20]
140c04af8: 49 8b 01                    	mov	rax, qword ptr [r9]
140c04afb: 48 85 c0                    	test	rax, rax
140c04afe: 74 04                       	je	0x140c04b04 <.text+0xc03b04>
140c04b00: 49 8b 41 08                 	mov	rax, qword ptr [r9 + 0x8]
140c04b04: 0f 57 c0                    	xorps	xmm0, xmm0
140c04b07: 0f 29 45 20                 	movaps	xmmword ptr [rbp + 0x20], xmm0
140c04b0b: 48 89 45 30                 	mov	qword ptr [rbp + 0x30], rax
140c04b0f: 48 8d 45 20                 	lea	rax, [rbp + 0x20]
140c04b13: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
140c04b18: 41 b9 01 00 00 00           	mov	r9d, 0x1
140c04b1e: 41 ff 92 70 03 00 00        	call	qword ptr [r10 + 0x370]
140c04b25: 48 c7 06 0b 00 00 00        	mov	qword ptr [rsi], 0xb
140c04b2c: 4c 89 f1                    	mov	rcx, r14
140c04b2f: e8 dc d2 07 00              	call	0x140c81e10 <.text+0xc80e10>
140c04b34: 31 c9                       	xor	ecx, ecx
140c04b36: b0 01                       	mov	al, 0x1
140c04b38: f0                          	lock
140c04b39: 41 0f b0 0e                 	cmpxchg	byte ptr [r14], cl
140c04b3d: 75 28                       	jne	0x140c04b67 <.text+0xc03b67>
140c04b3f: 48 89 f0                    	mov	rax, rsi
140c04b42: 48 81 c4 10 01 00 00        	add	rsp, 0x110
140c04b49: 5b                          	pop	rbx
140c04b4a: 5f                          	pop	rdi
140c04b4b: 5e                          	pop	rsi
140c04b4c: 41 5c                       	pop	r12
140c04b4e: 41 5e                       	pop	r14
140c04b50: 41 5f                       	pop	r15
140c04b52: 5d                          	pop	rbp
140c04b53: c3                          	ret
140c04b54: 4c 89 f1                    	mov	rcx, r14
140c04b57: 41 b8 00 ca 9a 3b           	mov	r8d, 0x3b9aca00
140c04b5d: e8 ae 86 cc 01              	call	0x1428cd210 <.text+0x28cc210>
140c04b62: e9 1f fd ff ff              	jmp	0x140c04886 <.text+0xc03886>
140c04b67: 4c 89 f1                    	mov	rcx, r14
140c04b6a: 31 d2                       	xor	edx, edx
140c04b6c: e8 1f 90 cc 01              	call	0x1428cdb90 <.text+0x28ccb90>
140c04b71: eb cc                       	jmp	0x140c04b3f <.text+0xc03b3f>
140c04b73: 4c 89 75 78                 	mov	qword ptr [rbp + 0x78], r14
140c04b77: 4c 89 7d a8                 	mov	qword ptr [rbp - 0x58], r15
140c04b7b: 48 8d 05 9e 8a fe ff        	lea	rax, [rip - 0x17562]    # 0x140bed620 <.text+0xbec620>
140c04b82: 48 89 45 b0                 	mov	qword ptr [rbp - 0x50], rax
140c04b86: 48 8d 05 23 00 f1 01        	lea	rax, [rip + 0x1f10023]  # 0x142b14bb0
140c04b8d: 48 89 45 20                 	mov	qword ptr [rbp + 0x20], rax
140c04b91: 48 c7 45 28 01 00 00 00     	mov	qword ptr [rbp + 0x28], 0x1
140c04b99: 48 c7 45 40 00 00 00 00     	mov	qword ptr [rbp + 0x40], 0x0
140c04ba1: 48 8d 45 a8                 	lea	rax, [rbp - 0x58]
140c04ba5: 48 89 45 30                 	mov	qword ptr [rbp + 0x30], rax
140c04ba9: 48 c7 45 38 01 00 00 00     	mov	qword ptr [rbp + 0x38], 0x1
140c04bb1: 48 8d 15 38 00 f1 01        	lea	rdx, [rip + 0x1f10038]  # 0x142b14bf0
140c04bb8: 48 8d 4d 20                 	lea	rcx, [rbp + 0x20]
140c04bbc: e8 cf 47 cd 01              	call	0x1428d9390 <.text+0x28d8390>
140c04bc1: e9 15 01 00 00              	jmp	0x140c04cdb <.text+0xc03cdb>
140c04bc6: 48 89 5d 08                 	mov	qword ptr [rbp + 0x8], rbx
140c04bca: 48 8d 05 4f 8a fe ff        	lea	rax, [rip - 0x175b1]    # 0x140bed620 <.text+0xbec620>
140c04bd1: 48 89 45 10                 	mov	qword ptr [rbp + 0x10], rax
140c04bd5: 48 8d 05 d4 ff f0 01        	lea	rax, [rip + 0x1f0ffd4]  # 0x142b14bb0
140c04bdc: 48 89 45 20                 	mov	qword ptr [rbp + 0x20], rax
140c04be0: 48 c7 45 28 01 00 00 00     	mov	qword ptr [rbp + 0x28], 0x1
140c04be8: 48 c7 45 40 00 00 00 00     	mov	qword ptr [rbp + 0x40], 0x0
140c04bf0: 48 8d 45 08                 	lea	rax, [rbp + 0x8]
140c04bf4: 48 89 45 30                 	mov	qword ptr [rbp + 0x30], rax
140c04bf8: 48 c7 45 38 01 00 00 00     	mov	qword ptr [rbp + 0x38], 0x1
140c04c00: c6 85 87 00 00 00 01        	mov	byte ptr [rbp + 0x87], 0x1
140c04c07: 48 8d 15 e2 ff f0 01        	lea	rdx, [rip + 0x1f0ffe2]  # 0x142b14bf0
140c04c0e: 48 8d 4d 20                 	lea	rcx, [rbp + 0x20]
140c04c12: e8 79 47 cd 01              	call	0x1428d9390 <.text+0x28d8390>
140c04c17: e9 bf 00 00 00              	jmp	0x140c04cdb <.text+0xc03cdb>
140c04c1c: 4c 89 75 78                 	mov	qword ptr [rbp + 0x78], r14
140c04c20: 4c 89 7d a8                 	mov	qword ptr [rbp - 0x58], r15
140c04c24: 48 8d 05 f5 89 fe ff        	lea	rax, [rip - 0x1760b]    # 0x140bed620 <.text+0xbec620>
140c04c2b: 48 89 45 b0                 	mov	qword ptr [rbp - 0x50], rax
140c04c2f: 48 8d 05 7a ff f0 01        	lea	rax, [rip + 0x1f0ff7a]  # 0x142b14bb0
140c04c36: 48 89 45 20                 	mov	qword ptr [rbp + 0x20], rax
140c04c3a: 48 c7 45 28 01 00 00 00     	mov	qword ptr [rbp + 0x28], 0x1
140c04c42: 48 c7 45 40 00 00 00 00     	mov	qword ptr [rbp + 0x40], 0x0
140c04c4a: 48 8d 45 a8                 	lea	rax, [rbp - 0x58]
140c04c4e: 48 89 45 30                 	mov	qword ptr [rbp + 0x30], rax
140c04c52: 48 c7 45 38 01 00 00 00     	mov	qword ptr [rbp + 0x38], 0x1
140c04c5a: 48 8d 15 8f ff f0 01        	lea	rdx, [rip + 0x1f0ff8f]  # 0x142b14bf0
140c04c61: 48 8d 4d 20                 	lea	rcx, [rbp + 0x20]
140c04c65: e8 26 47 cd 01              	call	0x1428d9390 <.text+0x28d8390>
140c04c6a: eb 6f                       	jmp	0x140c04cdb <.text+0xc03cdb>
140c04c6c: 48 89 5d 08                 	mov	qword ptr [rbp + 0x8], rbx
140c04c70: 48 8d 05 a9 89 fe ff        	lea	rax, [rip - 0x17657]    # 0x140bed620 <.text+0xbec620>
140c04c77: 48 89 45 10                 	mov	qword ptr [rbp + 0x10], rax
140c04c7b: 48 8d 05 2e ff f0 01        	lea	rax, [rip + 0x1f0ff2e]  # 0x142b14bb0
140c04c82: 48 89 45 20                 	mov	qword ptr [rbp + 0x20], rax
140c04c86: 48 c7 45 28 01 00 00 00     	mov	qword ptr [rbp + 0x28], 0x1
140c04c8e: 48 c7 45 40 00 00 00 00     	mov	qword ptr [rbp + 0x40], 0x0
140c04c96: 48 8d 45 08                 	lea	rax, [rbp + 0x8]
140c04c9a: 48 89 45 30                 	mov	qword ptr [rbp + 0x30], rax
140c04c9e: 48 c7 45 38 01 00 00 00     	mov	qword ptr [rbp + 0x38], 0x1
140c04ca6: c6 85 87 00 00 00 01        	mov	byte ptr [rbp + 0x87], 0x1
140c04cad: 48 8d 15 3c ff f0 01        	lea	rdx, [rip + 0x1f0ff3c]  # 0x142b14bf0
140c04cb4: 48 8d 4d 20                 	lea	rcx, [rbp + 0x20]
140c04cb8: e8 d3 46 cd 01              	call	0x1428d9390 <.text+0x28d8390>
140c04cbd: eb 1c                       	jmp	0x140c04cdb <.text+0xc03cdb>
140c04cbf: 4c 89 65 00                 	mov	qword ptr [rbp], r12
140c04cc3: 48 8d 0d 8e de f0 01        	lea	rcx, [rip + 0x1f0de8e]  # 0x142b12b58
140c04cca: 4c 8d 05 1f df f0 01        	lea	r8, [rip + 0x1f0df1f]   # 0x142b12bf0
140c04cd1: ba 6e 00 00 00              	mov	edx, 0x6e
140c04cd6: e8 55 47 cd 01              	call	0x1428d9430 <.text+0x28d8430>
140c04cdb: 0f 0b                       	ud2
140c04cdd: 0f 1f 00                    	nop	dword ptr [rax]
140c04ce0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140c04ce5: 55                          	push	rbp
140c04ce6: 41 57                       	push	r15
140c04ce8: 41 56                       	push	r14
140c04cea: 41 54                       	push	r12
140c04cec: 56                          	push	rsi
140c04ced: 57                          	push	rdi
140c04cee: 53                          	push	rbx
140c04cef: 48 83 ec 30                 	sub	rsp, 0x30
140c04cf3: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140c04cfa: 48 8b 4d 00                 	mov	rcx, qword ptr [rbp]
140c04cfe: 48 81 c1 10 0a 00 00        	add	rcx, 0xa10
140c04d05: e8 86 a5 9f ff              	call	0x1405ff290 <.text+0x5fe290>
140c04d0a: 90                          	nop
140c04d0b: 48 83 c4 30                 	add	rsp, 0x30
140c04d0f: 5b                          	pop	rbx
140c04d10: 5f                          	pop	rdi
140c04d11: 5e                          	pop	rsi
140c04d12: 41 5c                       	pop	r12
140c04d14: 41 5e                       	pop	r14
140c04d16: 41 5f                       	pop	r15
140c04d18: 5d                          	pop	rbp
140c04d19: c3                          	ret
140c04d1a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
140c04d20: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140c04d25: 55                          	push	rbp
140c04d26: 41 57                       	push	r15
140c04d28: 41 56                       	push	r14
140c04d2a: 41 54                       	push	r12
140c04d2c: 56                          	push	rsi
140c04d2d: 57                          	push	rdi
140c04d2e: 53                          	push	rbx
140c04d2f: 48 83 ec 30                 	sub	rsp, 0x30
140c04d33: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140c04d3a: 80 bd 87 00 00 00 00        	cmp	byte ptr [rbp + 0x87], 0x0
140c04d41: 74 19                       	je	0x140c04d5c <.text+0xc03d5c>
140c04d43: 48 8d 4d a8                 	lea	rcx, [rbp - 0x58]
140c04d47: e8 b4 91 fd ff              	call	0x140bddf00 <.text+0xbdcf00>
140c04d4c: 90                          	nop
140c04d4d: 48 83 c4 30                 	add	rsp, 0x30
140c04d51: 5b                          	pop	rbx
140c04d52: 5f                          	pop	rdi
140c04d53: 5e                          	pop	rsi
140c04d54: 41 5c                       	pop	r12
140c04d56: 41 5e                       	pop	r14
140c04d58: 41 5f                       	pop	r15
140c04d5a: 5d                          	pop	rbp
140c04d5b: c3                          	ret
140c04d5c: 48 83 c4 30                 	add	rsp, 0x30
140c04d60: 5b                          	pop	rbx
140c04d61: 5f                          	pop	rdi
140c04d62: 5e                          	pop	rsi
140c04d63: 41 5c                       	pop	r12
140c04d65: 41 5e                       	pop	r14
140c04d67: 41 5f                       	pop	r15
140c04d69: 5d                          	pop	rbp
140c04d6a: c3                          	ret
140c04d6b: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
140c04d70: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140c04d75: 55                          	push	rbp
140c04d76: 41 57                       	push	r15
140c04d78: 41 56                       	push	r14
140c04d7a: 41 54                       	push	r12
140c04d7c: 56                          	push	rsi
140c04d7d: 57                          	push	rdi
140c04d7e: 53                          	push	rbx
140c04d7f: 48 83 ec 30                 	sub	rsp, 0x30
140c04d83: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140c04d8a: 48 8b 4d 78                 	mov	rcx, qword ptr [rbp + 0x78]
140c04d8e: e8 9d ce 55 ff              	call	0x140161c30 <.text+0x160c30>
140c04d93: 90                          	nop
140c04d94: 48 83 c4 30                 	add	rsp, 0x30
140c04d98: 5b                          	pop	rbx
140c04d99: 5f                          	pop	rdi
140c04d9a: 5e                          	pop	rsi
140c04d9b: 41 5c                       	pop	r12
140c04d9d: 41 5e                       	pop	r14
140c04d9f: 41 5f                       	pop	r15
140c04da1: 5d                          	pop	rbp
140c04da2: c3                          	ret
140c04da3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nop	word ptr cs:[rax + rax]
140c04db0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140c04db5: 55                          	push	rbp
140c04db6: 41 57                       	push	r15
140c04db8: 41 56                       	push	r14
140c04dba: 41 54                       	push	r12
140c04dbc: 56                          	push	rsi
140c04dbd: 57                          	push	rdi
140c04dbe: 53                          	push	rbx
140c04dbf: 48 83 ec 30                 	sub	rsp, 0x30
140c04dc3: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140c04dca: 48 8b 45 70                 	mov	rax, qword ptr [rbp + 0x70]
140c04dce: 48 8b 4d 18                 	mov	rcx, qword ptr [rbp + 0x18]
140c04dd2: 48 89 41 50                 	mov	qword ptr [rcx + 0x50], rax
140c04dd6: 0f 28 45 60                 	movaps	xmm0, xmmword ptr [rbp + 0x60]
140c04dda: 0f 11 41 40                 	movups	xmmword ptr [rcx + 0x40], xmm0
140c04dde: 0f 28 45 20                 	movaps	xmm0, xmmword ptr [rbp + 0x20]
140c04de2: 0f 28 4d 30                 	movaps	xmm1, xmmword ptr [rbp + 0x30]
140c04de6: 0f 28 55 40                 	movaps	xmm2, xmmword ptr [rbp + 0x40]
140c04dea: 0f 28 5d 50                 	movaps	xmm3, xmmword ptr [rbp + 0x50]
140c04dee: 0f 11 59 30                 	movups	xmmword ptr [rcx + 0x30], xmm3
140c04df2: 0f 11 51 20                 	movups	xmmword ptr [rcx + 0x20], xmm2
140c04df6: 0f 11 49 10                 	movups	xmmword ptr [rcx + 0x10], xmm1
140c04dfa: 0f 11 01                    	movups	xmmword ptr [rcx], xmm0
140c04dfd: c6 85 87 00 00 00 00        	mov	byte ptr [rbp + 0x87], 0x0
140c04e04: 48 83 c4 30                 	add	rsp, 0x30
140c04e08: 5b                          	pop	rbx
140c04e09: 5f                          	pop	rdi
140c04e0a: 5e                          	pop	rsi
140c04e0b: 41 5c                       	pop	r12
140c04e0d: 41 5e                       	pop	r14
140c04e0f: 41 5f                       	pop	r15
140c04e11: 5d                          	pop	rbp
140c04e12: c3                          	ret
