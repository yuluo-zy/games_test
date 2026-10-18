
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141614b00: 55                          	push	rbp
141614b01: 41 57                       	push	r15
141614b03: 41 56                       	push	r14
141614b05: 41 54                       	push	r12
141614b07: 56                          	push	rsi
141614b08: 57                          	push	rdi
141614b09: 53                          	push	rbx
141614b0a: 48 81 ec 00 01 00 00        	sub	rsp, 0x100
141614b11: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
141614b19: 48 c7 45 78 fe ff ff ff     	mov	qword ptr [rbp + 0x78], -0x2
141614b21: 4d 85 c0                    	test	r8, r8
141614b24: 0f 84 f4 00 00 00           	je	0x141614c1e <.text+0x1613c1e>
141614b2a: 4d 89 ce                    	mov	r14, r9
141614b2d: 4c 89 c8                    	mov	rax, r9
141614b30: 48 83 e0 03                 	and	rax, 0x3
141614b34: 48 89 45 18                 	mov	qword ptr [rbp + 0x18], rax
141614b38: 0f 85 a5 00 00 00           	jne	0x141614be3 <.text+0x1613be3>
141614b3e: 4c 89 c3                    	mov	rbx, r8
141614b41: 4c 89 c0                    	mov	rax, r8
141614b44: 48 83 e0 03                 	and	rax, 0x3
141614b48: 48 89 45 18                 	mov	qword ptr [rbp + 0x18], rax
141614b4c: 0f 85 a2 00 00 00           	jne	0x141614bf4 <.text+0x1613bf4>
141614b52: 48 89 d7                    	mov	rdi, rdx
141614b55: 48 89 ce                    	mov	rsi, rcx
141614b58: 4c 8d 7d c0                 	lea	r15, [rbp - 0x40]
141614b5c: 4c 89 f9                    	mov	rcx, r15
141614b5f: 4c 89 f2                    	mov	rdx, r14
141614b62: 41 b8 22 00 00 00           	mov	r8d, 0x22
141614b68: 41 b1 01                    	mov	r9b, 0x1
141614b6b: e8 70 02 5c ff              	call	0x140bd4de0 <.text+0xbd3de0>
141614b70: 4c 8d 65 68                 	lea	r12, [rbp + 0x68]
141614b74: 41 b8 04 00 00 00           	mov	r8d, 0x4
141614b7a: 4c 89 e1                    	mov	rcx, r12
141614b7d: 4c 89 fa                    	mov	rdx, r15
141614b80: e8 9b 03 5c ff              	call	0x140bd4f20 <.text+0xbd3f20>
141614b85: 4c 89 74 24 30              	mov	qword ptr [rsp + 0x30], r14
141614b8a: 48 89 5c 24 28              	mov	qword ptr [rsp + 0x28], rbx
141614b8f: 4c 89 64 24 20              	mov	qword ptr [rsp + 0x20], r12
141614b94: 4c 8d 05 54 ad 66 01        	lea	r8, [rip + 0x166ad54]   # 0x142c7f8ef
141614b9b: 48 8d 4d 18                 	lea	rcx, [rbp + 0x18]
141614b9f: 41 b9 1d 00 00 00           	mov	r9d, 0x1d
141614ba5: 48 89 fa                    	mov	rdx, rdi
141614ba8: e8 13 e2 08 00              	call	0x1416a2dc0 <.text+0x16a1dc0>
141614bad: 83 7d 18 0b                 	cmp	dword ptr [rbp + 0x18], 0xb
141614bb1: 0f 85 de 00 00 00           	jne	0x141614c95 <.text+0x1613c95>
141614bb7: 48 8b 45 40                 	mov	rax, qword ptr [rbp + 0x40]
141614bbb: 48 89 46 20                 	mov	qword ptr [rsi + 0x20], rax
141614bbf: 0f 10 45 20                 	movups	xmm0, xmmword ptr [rbp + 0x20]
141614bc3: 0f 10 4d 30                 	movups	xmm1, xmmword ptr [rbp + 0x30]
141614bc7: 0f 11 4e 10                 	movups	xmmword ptr [rsi + 0x10], xmm1
141614bcb: 0f 11 06                    	movups	xmmword ptr [rsi], xmm0
141614bce: 48 89 f0                    	mov	rax, rsi
141614bd1: 48 81 c4 00 01 00 00        	add	rsp, 0x100
141614bd8: 5b                          	pop	rbx
141614bd9: 5f                          	pop	rdi
141614bda: 5e                          	pop	rsi
141614bdb: 41 5c                       	pop	r12
141614bdd: 41 5e                       	pop	r14
141614bdf: 41 5f                       	pop	r15
141614be1: 5d                          	pop	rbp
141614be2: c3                          	ret
141614be3: 48 c7 45 c0 00 00 00 00     	mov	qword ptr [rbp - 0x40], 0x0
141614beb: 48 8d 05 56 53 6b 01        	lea	rax, [rip + 0x16b5356]  # 0x142cc9f48
141614bf2: eb 0f                       	jmp	0x141614c03 <.text+0x1613c03>
141614bf4: 48 c7 45 c0 00 00 00 00     	mov	qword ptr [rbp - 0x40], 0x0
141614bfc: 48 8d 05 5d 53 6b 01        	lea	rax, [rip + 0x16b535d]  # 0x142cc9f60
141614c03: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
141614c08: 4c 8d 05 31 53 6b 01        	lea	r8, [rip + 0x16b5331]   # 0x142cc9f40
141614c0f: 48 8d 55 18                 	lea	rdx, [rbp + 0x18]
141614c13: 4c 8d 4d c0                 	lea	r9, [rbp - 0x40]
141614c17: 31 c9                       	xor	ecx, ecx
141614c19: e8 78 49 2c 01              	call	0x1428d9596 <.text+0x28d8596>
141614c1e: 48 8d 05 50 9d 48 01        	lea	rax, [rip + 0x1489d50]  # 0x142a9e975
141614c25: 48 89 45 68                 	mov	qword ptr [rbp + 0x68], rax
141614c29: 48 c7 45 70 42 00 00 00     	mov	qword ptr [rbp + 0x70], 0x42
141614c31: 48 8d 05 d8 ac 66 01        	lea	rax, [rip + 0x166acd8]  # 0x142c7f910
141614c38: 48 89 45 18                 	mov	qword ptr [rbp + 0x18], rax
141614c3c: 48 8d 05 5d ec f1 fe        	lea	rax, [rip - 0x10e13a3]  # 0x1405338a0 <.text+0x5328a0>
141614c43: 48 89 45 20                 	mov	qword ptr [rbp + 0x20], rax
141614c47: 48 8d 45 68                 	lea	rax, [rbp + 0x68]
141614c4b: 48 89 45 28                 	mov	qword ptr [rbp + 0x28], rax
141614c4f: 48 8d 05 3a 52 af fe        	lea	rax, [rip - 0x150adc6]  # 0x140109e90 <.text+0x108e90>
141614c56: 48 89 45 30                 	mov	qword ptr [rbp + 0x30], rax
141614c5a: 48 8d 05 cf aa 66 01        	lea	rax, [rip + 0x166aacf]  # 0x142c7f730
141614c61: 48 89 45 c0                 	mov	qword ptr [rbp - 0x40], rax
141614c65: 48 c7 45 c8 03 00 00 00     	mov	qword ptr [rbp - 0x38], 0x3
141614c6d: 48 c7 45 e0 00 00 00 00     	mov	qword ptr [rbp - 0x20], 0x0
141614c75: 48 8d 45 18                 	lea	rax, [rbp + 0x18]
141614c79: 48 89 45 d0                 	mov	qword ptr [rbp - 0x30], rax
141614c7d: 48 c7 45 d8 02 00 00 00     	mov	qword ptr [rbp - 0x28], 0x2
141614c85: 48 8d 15 fc aa 66 01        	lea	rdx, [rip + 0x166aafc]  # 0x142c7f788
141614c8c: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
141614c90: e8 fb 46 2c 01              	call	0x1428d9390 <.text+0x28d8390>
141614c95: 0f 10 45 58                 	movups	xmm0, xmmword ptr [rbp + 0x58]
141614c99: 0f 29 45 00                 	movaps	xmmword ptr [rbp], xmm0
141614c9d: 0f 10 45 18                 	movups	xmm0, xmmword ptr [rbp + 0x18]
141614ca1: 0f 10 4d 28                 	movups	xmm1, xmmword ptr [rbp + 0x28]
141614ca5: 0f 10 55 38                 	movups	xmm2, xmmword ptr [rbp + 0x38]
141614ca9: 0f 10 5d 48                 	movups	xmm3, xmmword ptr [rbp + 0x48]
141614cad: 0f 29 5d f0                 	movaps	xmmword ptr [rbp - 0x10], xmm3
141614cb1: 0f 29 55 e0                 	movaps	xmmword ptr [rbp - 0x20], xmm2
141614cb5: 0f 29 4d d0                 	movaps	xmmword ptr [rbp - 0x30], xmm1
141614cb9: 0f 29 45 c0                 	movaps	xmmword ptr [rbp - 0x40], xmm0
141614cbd: 48 8d 05 e4 b4 66 01        	lea	rax, [rip + 0x166b4e4]  # 0x142c801a8
141614cc4: 48 89 44 24 20              	mov	qword ptr [rsp + 0x20], rax
141614cc9: 48 8d 0d 18 9e 66 01        	lea	rcx, [rip + 0x1669e18]  # 0x142c7eae8
141614cd0: 4c 8d 0d f1 9d 66 01        	lea	r9, [rip + 0x1669df1]   # 0x142c7eac8
141614cd7: 4c 8d 45 c0                 	lea	r8, [rbp - 0x40]
141614cdb: ba 2b 00 00 00              	mov	edx, 0x2b
141614ce0: e8 7b 4a 2c 01              	call	0x1428d9760 <.text+0x28d8760>
141614ce5: 0f 0b                       	ud2
141614ce7: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
141614cf0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
141614cf5: 55                          	push	rbp
141614cf6: 41 57                       	push	r15
141614cf8: 41 56                       	push	r14
141614cfa: 41 54                       	push	r12
141614cfc: 56                          	push	rsi
141614cfd: 57                          	push	rdi
141614cfe: 53                          	push	rbx
141614cff: 48 83 ec 40                 	sub	rsp, 0x40
141614d03: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
141614d0a: 48 8d 4d c0                 	lea	rcx, [rbp - 0x40]
141614d0e: e8 5d 9d 1b ff              	call	0x1407cea70 <.text+0x7cda70>
141614d13: 90                          	nop
141614d14: 48 83 c4 40                 	add	rsp, 0x40
141614d18: 5b                          	pop	rbx
141614d19: 5f                          	pop	rdi
141614d1a: 5e                          	pop	rsi
141614d1b: 41 5c                       	pop	r12
141614d1d: 41 5e                       	pop	r14
141614d1f: 41 5f                       	pop	r15
141614d21: 5d                          	pop	rbp
141614d22: c3                          	ret
