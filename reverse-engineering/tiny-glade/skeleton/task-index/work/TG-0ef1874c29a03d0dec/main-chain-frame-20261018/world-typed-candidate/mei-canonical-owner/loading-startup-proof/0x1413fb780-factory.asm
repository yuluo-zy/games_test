
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1413fb780: 55                          	push	rbp
1413fb781: 56                          	push	rsi
1413fb782: 57                          	push	rdi
1413fb783: 48 81 ec c0 01 00 00        	sub	rsp, 0x1c0
1413fb78a: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
1413fb792: 48 c7 85 38 01 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x138], -0x2
1413fb79d: 48 89 ce                    	mov	rsi, rcx
1413fb7a0: 0f 57 c0                    	xorps	xmm0, xmm0
1413fb7a3: 0f 11 45 c0                 	movups	xmmword ptr [rbp - 0x40], xmm0
1413fb7a7: 0f 11 45 d8                 	movups	xmmword ptr [rbp - 0x28], xmm0
1413fb7ab: 0f 11 45 f0                 	movups	xmmword ptr [rbp - 0x10], xmm0
1413fb7af: 0f 11 45 08                 	movups	xmmword ptr [rbp + 0x8], xmm0
1413fb7b3: 0f 11 45 20                 	movups	xmmword ptr [rbp + 0x20], xmm0
1413fb7b7: c7 45 30 00 00 00 00        	mov	dword ptr [rbp + 0x30], 0x0
1413fb7be: 0f 11 45 58                 	movups	xmmword ptr [rbp + 0x58], xmm0
1413fb7c2: 0f 11 45 70                 	movups	xmmword ptr [rbp + 0x70], xmm0
1413fb7c6: 0f 11 85 88 00 00 00        	movups	xmmword ptr [rbp + 0x88], xmm0
1413fb7cd: 0f 11 85 a0 00 00 00        	movups	xmmword ptr [rbp + 0xa0], xmm0
1413fb7d4: 0f 11 85 b8 00 00 00        	movups	xmmword ptr [rbp + 0xb8], xmm0
1413fb7db: c7 85 c8 00 00 00 00 00 00 00       	mov	dword ptr [rbp + 0xc8], 0x0
1413fb7e5: 48 c7 45 a0 00 00 00 00     	mov	qword ptr [rbp - 0x60], 0x0
1413fb7ed: 48 c7 45 a8 08 00 00 00     	mov	qword ptr [rbp - 0x58], 0x8
1413fb7f5: 48 c7 45 b0 00 00 00 00     	mov	qword ptr [rbp - 0x50], 0x0
1413fb7fd: 48 c7 45 b8 10 00 00 00     	mov	qword ptr [rbp - 0x48], 0x10
1413fb805: 48 c7 45 d0 10 00 00 00     	mov	qword ptr [rbp - 0x30], 0x10
1413fb80d: 48 c7 45 e8 10 00 00 00     	mov	qword ptr [rbp - 0x18], 0x10
1413fb815: 48 c7 45 00 10 00 00 00     	mov	qword ptr [rbp], 0x10
1413fb81d: 48 c7 45 18 10 00 00 00     	mov	qword ptr [rbp + 0x18], 0x10
1413fb825: 48 b8 00 00 00 00 00 00 00 80       	movabs	rax, -0x8000000000000000
1413fb82f: 48 89 45 38                 	mov	qword ptr [rbp + 0x38], rax
1413fb833: 48 8d 0d 9b f4 85 01        	lea	rcx, [rip + 0x185f49b]  # 0x142c5acd5
1413fb83a: 48 89 4d 40                 	mov	qword ptr [rbp + 0x40], rcx
1413fb83e: 48 c7 45 48 3d 00 00 00     	mov	qword ptr [rbp + 0x48], 0x3d
1413fb846: 48 c7 45 50 10 00 00 00     	mov	qword ptr [rbp + 0x50], 0x10
1413fb84e: 48 c7 45 68 10 00 00 00     	mov	qword ptr [rbp + 0x68], 0x10
1413fb856: 48 c7 85 80 00 00 00 10 00 00 00    	mov	qword ptr [rbp + 0x80], 0x10
1413fb861: 48 c7 85 98 00 00 00 10 00 00 00    	mov	qword ptr [rbp + 0x98], 0x10
1413fb86c: 48 c7 85 b0 00 00 00 10 00 00 00    	mov	qword ptr [rbp + 0xb0], 0x10
1413fb877: c7 85 d0 00 00 00 00 00 00 00       	mov	dword ptr [rbp + 0xd0], 0x0
1413fb881: 66 c7 85 d4 00 00 00 01 00  	mov	word ptr [rbp + 0xd4], 0x1
1413fb88a: 48 89 85 f0 00 00 00        	mov	qword ptr [rbp + 0xf0], rax
1413fb891: c7 85 30 01 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x130], 0x0
1413fb89b: 0f b6 05 67 93 23 02        	movzx	eax, byte ptr [rip + 0x2239367] # 0x143634c09
1413fb8a2: b9 98 01 00 00              	mov	ecx, 0x198
1413fb8a7: ba 08 00 00 00              	mov	edx, 0x8
1413fb8ac: e8 5f 83 21 ff              	call	0x140613c10 <.text+0x612c10>
1413fb8b1: 48 85 c0                    	test	rax, rax
1413fb8b4: 74 35                       	je	0x1413fb8eb <.text+0x13fa8eb>
1413fb8b6: 48 89 c7                    	mov	rdi, rax
1413fb8b9: 48 8d 55 a0                 	lea	rdx, [rbp - 0x60]
1413fb8bd: 41 b8 98 01 00 00           	mov	r8d, 0x198
1413fb8c3: 48 89 c1                    	mov	rcx, rax
1413fb8c6: e8 35 83 52 01              	call	0x142923c00 <.text+0x2922c00>
1413fb8cb: 4c 8d 05 06 f1 85 01        	lea	r8, [rip + 0x185f106]   # 0x142c5a9d8
1413fb8d2: 48 89 f1                    	mov	rcx, rsi
1413fb8d5: 48 89 fa                    	mov	rdx, rdi
1413fb8d8: e8 33 92 8e ff              	call	0x140ce4b10 <.text+0xce3b10>
1413fb8dd: 48 89 f0                    	mov	rax, rsi
1413fb8e0: 48 81 c4 c0 01 00 00        	add	rsp, 0x1c0
1413fb8e7: 5f                          	pop	rdi
1413fb8e8: 5e                          	pop	rsi
1413fb8e9: 5d                          	pop	rbp
1413fb8ea: c3                          	ret
1413fb8eb: b9 08 00 00 00              	mov	ecx, 0x8
1413fb8f0: ba 98 01 00 00              	mov	edx, 0x198
1413fb8f5: e8 06 d7 4d 01              	call	0x1428d9000 <.text+0x28d8000>
1413fb8fa: 0f 0b                       	ud2
1413fb8fc: 0f 1f 40 00                 	nop	dword ptr [rax]
1413fb900: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1413fb905: 55                          	push	rbp
1413fb906: 56                          	push	rsi
1413fb907: 57                          	push	rdi
1413fb908: 48 83 ec 20                 	sub	rsp, 0x20
1413fb90c: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1413fb913: 48 8d 4d a0                 	lea	rcx, [rbp - 0x60]
1413fb917: e8 24 1b 20 ff              	call	0x1405fd440 <.text+0x5fc440>
1413fb91c: 90                          	nop
1413fb91d: 48 83 c4 20                 	add	rsp, 0x20
1413fb921: 5f                          	pop	rdi
1413fb922: 5e                          	pop	rsi
1413fb923: 5d                          	pop	rbp
1413fb924: c3                          	ret
