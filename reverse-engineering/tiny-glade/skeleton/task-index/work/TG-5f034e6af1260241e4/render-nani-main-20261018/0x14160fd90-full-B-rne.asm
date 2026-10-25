
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14160fd90: 55                          	push	rbp
14160fd91: 41 57                       	push	r15
14160fd93: 41 56                       	push	r14
14160fd95: 41 54                       	push	r12
14160fd97: 56                          	push	rsi
14160fd98: 57                          	push	rdi
14160fd99: 53                          	push	rbx
14160fd9a: 48 81 ec 00 01 00 00        	sub	rsp, 0x100
14160fda1: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
14160fda9: 48 c7 45 78 fe ff ff ff     	mov	qword ptr [rbp + 0x78], -0x2
14160fdb1: 4d 85 c0                    	test	r8, r8
14160fdb4: 0f 84 f4 00 00 00           	je	0x14160feae <.text+0x160eeae>
14160fdba: 4d 89 ce                    	mov	r14, r9
14160fdbd: 4c 89 c8                    	mov	rax, r9
14160fdc0: 48 83 e0 03                 	and	rax, 0x3
14160fdc4: 48 89 45 18                 	mov	qword ptr [rbp + 0x18], rax
14160fdc8: 0f 85 a5 00 00 00           	jne	0x14160fe73 <.text+0x160ee73>
14160fdce: 4c 89 c3                    	mov	rbx, r8
14160fdd1: 4c 89 c0                    	mov	rax, r8
14160fdd4: 48 83 e0 03                 	and	rax, 0x3
14160fdd8: 48 89 45 18                 	mov	qword ptr [rbp + 0x18], rax
14160fddc: 0f 85 a2 00 00 00           	jne	0x14160fe84 <.text+0x160ee84>
14160fde2: 48 89 d7                    	mov	rdi, rdx
14160fde5: 48 89 ce                    	mov	rsi, rcx
14160fde8: 4c 8d 7d c0                 	lea	r15, [rbp - 0x40]
14160fdec: 4c 89 f9                    	mov	rcx, r15
14160fdef: 4c 89 f2                    	mov	rdx, r14
14160fdf2: 41 b8 22 00 00 00           	mov	r8d, 0x22
14160fdf8: 41 b1 01                    	mov	r9b, 0x1
14160fdfb: e8 e0 4f 5c ff              	call	0x140bd4de0 <.text+0xbd3de0>
14160fe00: 4c 8d 65 68                 	lea	r12, [rbp + 0x68]
14160fe04: 41 b8 04 00 00 00           	mov	r8d, 0x4
14160fe0a: 4c 89 e1                    	mov	rcx, r12
14160fe0d: 4c 89 fa                    	mov	rdx, r15
14160fe10: e8 0b 51 5c ff              	call	0x140bd4f20 <.text+0xbd3f20>
14160fe15: 4c 89 74 24 30              	mov	qword ptr [rsp + 0x30], r14
14160fe1a: 48 89 5c 24 28              	mov	qword ptr [rsp + 0x28], rbx
14160fe1f: 4c 89 64 24 20              	mov	qword ptr [rsp + 0x20], r12
14160fe24: 4c 8d 05 85 ff 66 01        	lea	r8, [rip + 0x166ff85]   # 0x142c7fdb0
14160fe2b: 48 8d 4d 18                 	lea	rcx, [rbp + 0x18]
14160fe2f: 41 b9 1d 00 00 00           	mov	r9d, 0x1d
14160fe35: 48 89 fa                    	mov	rdx, rdi
14160fe38: e8 83 2f 09 00              	call	0x1416a2dc0 <.text+0x16a1dc0>
14160fe3d: 83 7d 18 0b                 	cmp	dword ptr [rbp + 0x18], 0xb
14160fe41: 0f 85 de 00 00 00           	jne	0x14160ff25 <.text+0x160ef25>
14160fe47: 48 8b 45 40                 	mov	rax, qword ptr [rbp + 0x40]
14160fe4b: 48 89 46 20                 	mov	qword ptr [rsi + 0x20], rax
14160fe4f: 0f 10 45 20                 	movups	xmm0, xmmword ptr [rbp + 0x20]
14160fe53: 0f 10 4d 30                 	movups	xmm1, xmmword ptr [rbp + 0x30]
14160fe57: 0f 11 4e 10                 	movups	xmmword ptr [rsi + 0x10], xmm1
14160fe5b: 0f 11 06                    	movups	xmmword ptr [rsi], xmm0
14160fe5e: 48 89 f0                    	mov	rax, rsi
14160fe61: 48 81 c4 00 01 00 00        	add	rsp, 0x100
14160fe68: 5b                          	pop	rbx
14160fe69: 5f                          	pop	rdi
14160fe6a: 5e                          	pop	rsi
14160fe6b: 41 5c                       	pop	r12
14160fe6d: 41 5e                       	pop	r14
14160fe6f: 41 5f                       	pop	r15
14160fe71: 5d                          	pop	rbp
14160fe72: c3                          	ret
14160fe73: 48 c7 45 c0 00 00 00 00     	mov	qword ptr [rbp - 0x40], 0x0
14160fe7b: 48 8d 05 c6 a0 6b 01        	lea	rax, [rip + 0x16ba0c6]  # 0x142cc9f48
14160fe82: eb 0f                       	jmp	0x14160fe93 <.text+0x160ee93>
14160fe84: 48 c7 45 c0 00 00 00 00     	mov	qword ptr [rbp - 0x40], 0x0
14160fe8c: 48 8d 05 cd a0 6b 01        	lea	rax, [rip + 0x16ba0cd]  # 0x142cc9f60
14160fe93: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
14160fe98: 4c 8d 05 a1 a0 6b 01        	lea	r8, [rip + 0x16ba0a1]   # 0x142cc9f40
14160fe9f: 48 8d 55 18                 	lea	rdx, [rbp + 0x18]
14160fea3: 4c 8d 4d c0                 	lea	r9, [rbp - 0x40]
14160fea7: 31 c9                       	xor	ecx, ecx
14160fea9: e8 e8 96 2c 01              	call	0x1428d9596 <.text+0x28d8596>
14160feae: 48 8d 05 95 de 48 01        	lea	rax, [rip + 0x148de95]  # 0x142a9dd4a
14160feb5: 48 89 45 68                 	mov	qword ptr [rbp + 0x68], rax
14160feb9: 48 c7 45 70 42 00 00 00     	mov	qword ptr [rbp + 0x70], 0x42
14160fec1: 48 8d 05 08 ff 66 01        	lea	rax, [rip + 0x166ff08]  # 0x142c7fdd0
14160fec8: 48 89 45 18                 	mov	qword ptr [rbp + 0x18], rax
14160fecc: 48 8d 05 cd 39 f2 fe        	lea	rax, [rip - 0x10dc633]  # 0x1405338a0 <.text+0x5328a0>
14160fed3: 48 89 45 20                 	mov	qword ptr [rbp + 0x20], rax
14160fed7: 48 8d 45 68                 	lea	rax, [rbp + 0x68]
14160fedb: 48 89 45 28                 	mov	qword ptr [rbp + 0x28], rax
14160fedf: 48 8d 05 aa 9f af fe        	lea	rax, [rip - 0x1506056]  # 0x140109e90 <.text+0x108e90>
14160fee6: 48 89 45 30                 	mov	qword ptr [rbp + 0x30], rax
14160feea: 48 8d 05 3f f8 66 01        	lea	rax, [rip + 0x166f83f]  # 0x142c7f730
14160fef1: 48 89 45 c0                 	mov	qword ptr [rbp - 0x40], rax
14160fef5: 48 c7 45 c8 03 00 00 00     	mov	qword ptr [rbp - 0x38], 0x3
14160fefd: 48 c7 45 e0 00 00 00 00     	mov	qword ptr [rbp - 0x20], 0x0
14160ff05: 48 8d 45 18                 	lea	rax, [rbp + 0x18]
14160ff09: 48 89 45 d0                 	mov	qword ptr [rbp - 0x30], rax
14160ff0d: 48 c7 45 d8 02 00 00 00     	mov	qword ptr [rbp - 0x28], 0x2
14160ff15: 48 8d 15 6c f8 66 01        	lea	rdx, [rip + 0x166f86c]  # 0x142c7f788
14160ff1c: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
14160ff20: e8 6b 94 2c 01              	call	0x1428d9390 <.text+0x28d8390>
14160ff25: 0f 10 45 58                 	movups	xmm0, xmmword ptr [rbp + 0x58]
14160ff29: 0f 29 45 00                 	movaps	xmmword ptr [rbp], xmm0
14160ff2d: 0f 10 45 18                 	movups	xmm0, xmmword ptr [rbp + 0x18]
14160ff31: 0f 10 4d 28                 	movups	xmm1, xmmword ptr [rbp + 0x28]
14160ff35: 0f 10 55 38                 	movups	xmm2, xmmword ptr [rbp + 0x38]
14160ff39: 0f 10 5d 48                 	movups	xmm3, xmmword ptr [rbp + 0x48]
14160ff3d: 0f 29 5d f0                 	movaps	xmmword ptr [rbp - 0x10], xmm3
14160ff41: 0f 29 55 e0                 	movaps	xmmword ptr [rbp - 0x20], xmm2
14160ff45: 0f 29 4d d0                 	movaps	xmmword ptr [rbp - 0x30], xmm1
14160ff49: 0f 29 45 c0                 	movaps	xmmword ptr [rbp - 0x40], xmm0
14160ff4d: 48 8d 05 54 02 67 01        	lea	rax, [rip + 0x1670254]  # 0x142c801a8
14160ff54: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
14160ff59: 48 8d 0d 88 eb 66 01        	lea	rcx, [rip + 0x166eb88]  # 0x142c7eae8
14160ff60: 4c 8d 0d 61 eb 66 01        	lea	r9, [rip + 0x166eb61]   # 0x142c7eac8
14160ff67: 4c 8d 45 c0                 	lea	r8, [rbp - 0x40]
14160ff6b: ba 2b 00 00 00              	mov	edx, 0x2b
14160ff70: e8 eb 97 2c 01              	call	0x1428d9760 <.text+0x28d8760>
14160ff75: 0f 0b                       	ud2
14160ff77: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
14160ff80: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14160ff85: 55                          	push	rbp
14160ff86: 41 57                       	push	r15
14160ff88: 41 56                       	push	r14
14160ff8a: 41 54                       	push	r12
14160ff8c: 56                          	push	rsi
14160ff8d: 57                          	push	rdi
14160ff8e: 53                          	push	rbx
14160ff8f: 48 83 ec 40                 	sub	rsp, 0x40
14160ff93: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14160ff9a: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
14160ff9e: e8 cd ea 1b ff              	call	0x1407cea70 <.text+0x7cda70>
14160ffa3: 90                          	nop
14160ffa4: 48 83 c4 40                 	add	rsp, 0x40
14160ffa8: 5b                          	pop	rbx
14160ffa9: 5f                          	pop	rdi
14160ffaa: 5e                          	pop	rsi
14160ffab: 41 5c                       	pop	r12
14160ffad: 41 5e                       	pop	r14
14160ffaf: 41 5f                       	pop	r15
14160ffb1: 5d                          	pop	rbp
14160ffb2: c3                          	ret
