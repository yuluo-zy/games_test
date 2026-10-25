
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140986ad0: 55                          	push	rbp
140986ad1: 56                          	push	rsi
140986ad2: 57                          	push	rdi
140986ad3: 53                          	push	rbx
140986ad4: 48 81 ec 18 03 00 00        	sub	rsp, 0x318
140986adb: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
140986ae3: 48 c7 85 90 02 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x290], -0x2
140986aee: 48 89 ce                    	mov	rsi, rcx
140986af1: 0f b6 05 11 e1 ca 02        	movzx	eax, byte ptr [rip + 0x2cae111] # 0x143634c09
140986af8: b9 c0 01 00 00              	mov	ecx, 0x1c0
140986afd: ba 08 00 00 00              	mov	edx, 0x8
140986b02: e8 09 d1 c8 ff              	call	0x140613c10 <.text+0x612c10>
140986b07: 48 85 c0                    	test	rax, rax
140986b0a: 0f 84 bd 04 00 00           	je	0x140986fcd <.text+0x985fcd>
140986b10: 48 89 85 80 02 00 00        	mov	qword ptr [rbp + 0x280], rax
140986b17: 0f b6 05 eb e0 ca 02        	movzx	eax, byte ptr [rip + 0x2cae0eb] # 0x143634c09
140986b1e: b9 e0 00 00 00              	mov	ecx, 0xe0
140986b23: ba 08 00 00 00              	mov	edx, 0x8
140986b28: e8 e3 d0 c8 ff              	call	0x140613c10 <.text+0x612c10>
140986b2d: 48 85 c0                    	test	rax, rax
140986b30: 0f 84 a6 04 00 00           	je	0x140986fdc <.text+0x985fdc>
140986b36: 48 89 85 88 02 00 00        	mov	qword ptr [rbp + 0x288], rax
140986b3d: 0f 57 c0                    	xorps	xmm0, xmm0
140986b40: 0f 11 85 60 01 00 00        	movups	xmmword ptr [rbp + 0x160], xmm0
140986b47: 0f 11 85 78 01 00 00        	movups	xmmword ptr [rbp + 0x178], xmm0
140986b4e: 0f 11 85 90 01 00 00        	movups	xmmword ptr [rbp + 0x190], xmm0
140986b55: 0f 11 85 a8 01 00 00        	movups	xmmword ptr [rbp + 0x1a8], xmm0
140986b5c: 0f 11 85 c0 01 00 00        	movups	xmmword ptr [rbp + 0x1c0], xmm0
140986b63: c7 85 d0 01 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x1d0], 0x0
140986b6d: 0f 11 85 f8 01 00 00        	movups	xmmword ptr [rbp + 0x1f8], xmm0
140986b74: 0f 11 85 10 02 00 00        	movups	xmmword ptr [rbp + 0x210], xmm0
140986b7b: 0f 11 85 28 02 00 00        	movups	xmmword ptr [rbp + 0x228], xmm0
140986b82: 0f 11 85 40 02 00 00        	movups	xmmword ptr [rbp + 0x240], xmm0
140986b89: 0f 11 85 58 02 00 00        	movups	xmmword ptr [rbp + 0x258], xmm0
140986b90: c7 85 68 02 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x268], 0x0
140986b9a: 48 c7 85 f8 00 00 00 00 00 00 00    	mov	qword ptr [rbp + 0xf8], 0x0
140986ba5: 48 c7 85 40 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x140], 0x0
140986bb0: 48 c7 85 48 01 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x148], 0x8
140986bbb: 48 c7 85 50 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x150], 0x0
140986bc6: 48 c7 85 58 01 00 00 10 00 00 00    	mov	qword ptr [rbp + 0x158], 0x10
140986bd1: 48 c7 85 70 01 00 00 10 00 00 00    	mov	qword ptr [rbp + 0x170], 0x10
140986bdc: 48 c7 85 88 01 00 00 10 00 00 00    	mov	qword ptr [rbp + 0x188], 0x10
140986be7: 48 c7 85 a0 01 00 00 10 00 00 00    	mov	qword ptr [rbp + 0x1a0], 0x10
140986bf2: 48 c7 85 b8 01 00 00 10 00 00 00    	mov	qword ptr [rbp + 0x1b8], 0x10
140986bfd: 48 bb 00 00 00 00 00 00 00 80       	movabs	rbx, -0x8000000000000000
140986c07: 48 89 9d d8 01 00 00        	mov	qword ptr [rbp + 0x1d8], rbx
140986c0e: 48 8d 05 14 1e 13 02        	lea	rax, [rip + 0x2131e14]  # 0x142ab8a29
140986c15: 48 89 85 e0 01 00 00        	mov	qword ptr [rbp + 0x1e0], rax
140986c1c: 48 c7 85 e8 01 00 00 67 00 00 00    	mov	qword ptr [rbp + 0x1e8], 0x67
140986c27: 48 c7 85 f0 01 00 00 10 00 00 00    	mov	qword ptr [rbp + 0x1f0], 0x10
140986c32: 48 c7 85 08 02 00 00 10 00 00 00    	mov	qword ptr [rbp + 0x208], 0x10
140986c3d: 48 c7 85 20 02 00 00 10 00 00 00    	mov	qword ptr [rbp + 0x220], 0x10
140986c48: 48 c7 85 38 02 00 00 10 00 00 00    	mov	qword ptr [rbp + 0x238], 0x10
140986c53: 48 c7 85 50 02 00 00 10 00 00 00    	mov	qword ptr [rbp + 0x250], 0x10
140986c5e: c7 85 70 02 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x270], 0x0
140986c68: 66 c7 85 74 02 00 00 01 00  	mov	word ptr [rbp + 0x274], 0x1
140986c71: c7 85 78 02 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x278], 0x0
140986c7b: 0f b6 05 87 df ca 02        	movzx	eax, byte ptr [rip + 0x2cadf87] # 0x143634c09
140986c82: b9 88 01 00 00              	mov	ecx, 0x188
140986c87: ba 08 00 00 00              	mov	edx, 0x8
140986c8c: e8 7f cf c8 ff              	call	0x140613c10 <.text+0x612c10>
140986c91: 48 85 c0                    	test	rax, rax
140986c94: 0f 84 53 03 00 00           	je	0x140986fed <.text+0x985fed>
140986c9a: 48 89 c7                    	mov	rdi, rax
140986c9d: 48 8d 95 f8 00 00 00        	lea	rdx, [rbp + 0xf8]
140986ca4: 41 b8 88 01 00 00           	mov	r8d, 0x188
140986caa: 48 89 c1                    	mov	rcx, rax
140986cad: e8 4e cf f9 01              	call	0x142923c00 <.text+0x2922c00>
140986cb2: 4c 8d 05 e7 1b 13 02        	lea	r8, [rip + 0x2131be7]   # 0x142ab88a0
140986cb9: 48 8d 8d 88 00 00 00        	lea	rcx, [rbp + 0x88]
140986cc0: 48 89 fa                    	mov	rdx, rdi
140986cc3: e8 48 de 35 00              	call	0x140ce4b10 <.text+0xce3b10>
140986cc8: 48 8d 8d f8 00 00 00        	lea	rcx, [rbp + 0xf8]
140986ccf: e8 4c da 1d 00              	call	0x140b64720 <.text+0xb63720>
140986cd4: 0f 10 85 e8 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xe8]
140986cdb: 48 8b 85 88 02 00 00        	mov	rax, qword ptr [rbp + 0x288]
140986ce2: 0f 11 40 60                 	movups	xmmword ptr [rax + 0x60], xmm0
140986ce6: 0f 10 85 d8 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xd8]
140986ced: 0f 11 40 50                 	movups	xmmword ptr [rax + 0x50], xmm0
140986cf1: 0f 10 85 c8 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xc8]
140986cf8: 0f 11 40 40                 	movups	xmmword ptr [rax + 0x40], xmm0
140986cfc: 0f 10 85 88 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0x88]
140986d03: 0f 10 8d 98 00 00 00        	movups	xmm1, xmmword ptr [rbp + 0x98]
140986d0a: 0f 10 95 a8 00 00 00        	movups	xmm2, xmmword ptr [rbp + 0xa8]
140986d11: 0f 10 9d b8 00 00 00        	movups	xmm3, xmmword ptr [rbp + 0xb8]
140986d18: 0f 11 58 30                 	movups	xmmword ptr [rax + 0x30], xmm3
140986d1c: 0f 11 50 20                 	movups	xmmword ptr [rax + 0x20], xmm2
140986d20: 0f 11 48 10                 	movups	xmmword ptr [rax + 0x10], xmm1
140986d24: 0f 11 00                    	movups	xmmword ptr [rax], xmm0
140986d27: 0f 10 85 f8 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xf8]
140986d2e: 0f 10 8d 08 01 00 00        	movups	xmm1, xmmword ptr [rbp + 0x108]
140986d35: 0f 10 95 18 01 00 00        	movups	xmm2, xmmword ptr [rbp + 0x118]
140986d3c: 0f 10 9d 28 01 00 00        	movups	xmm3, xmmword ptr [rbp + 0x128]
140986d43: 0f 11 40 70                 	movups	xmmword ptr [rax + 0x70], xmm0
140986d47: 0f 11 88 80 00 00 00        	movups	xmmword ptr [rax + 0x80], xmm1
140986d4e: 0f 11 90 90 00 00 00        	movups	xmmword ptr [rax + 0x90], xmm2
140986d55: 0f 11 98 a0 00 00 00        	movups	xmmword ptr [rax + 0xa0], xmm3
140986d5c: 0f 10 85 38 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x138]
140986d63: 0f 11 80 b0 00 00 00        	movups	xmmword ptr [rax + 0xb0], xmm0
140986d6a: 0f 10 85 48 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x148]
140986d71: 0f 11 80 c0 00 00 00        	movups	xmmword ptr [rax + 0xc0], xmm0
140986d78: 0f 10 85 58 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x158]
140986d7f: 0f 11 80 d0 00 00 00        	movups	xmmword ptr [rax + 0xd0], xmm0
140986d86: 48 c7 45 20 02 00 00 00     	mov	qword ptr [rbp + 0x20], 0x2
140986d8e: 48 89 45 28                 	mov	qword ptr [rbp + 0x28], rax
140986d92: 48 c7 45 30 02 00 00 00     	mov	qword ptr [rbp + 0x30], 0x2
140986d9a: 48 c7 45 38 00 00 00 00     	mov	qword ptr [rbp + 0x38], 0x0
140986da2: 48 c7 45 40 08 00 00 00     	mov	qword ptr [rbp + 0x40], 0x8
140986daa: 0f 57 c0                    	xorps	xmm0, xmm0
140986dad: 0f 11 45 48                 	movups	xmmword ptr [rbp + 0x48], xmm0
140986db1: 48 89 5d 18                 	mov	qword ptr [rbp + 0x18], rbx
140986db5: 48 8d 4d a8                 	lea	rcx, [rbp - 0x58]
140986db9: e8 d2 4a d0 00              	call	0x14168b890 <.text+0x168a890>
140986dbe: 48 8d 8d 88 00 00 00        	lea	rcx, [rbp + 0x88]
140986dc5: e8 f6 ef 18 00              	call	0x140b15dc0 <.text+0xb14dc0>
140986dca: 48 8d 8d f8 00 00 00        	lea	rcx, [rbp + 0xf8]
140986dd1: e8 6a dc dd 00              	call	0x141764a40 <.text+0x1763a40>
140986dd6: 0f 10 45 78                 	movups	xmm0, xmmword ptr [rbp + 0x78]
140986dda: 48 8b 85 80 02 00 00        	mov	rax, qword ptr [rbp + 0x280]
140986de1: 0f 11 40 60                 	movups	xmmword ptr [rax + 0x60], xmm0
140986de5: 0f 10 45 68                 	movups	xmm0, xmmword ptr [rbp + 0x68]
140986de9: 0f 11 40 50                 	movups	xmmword ptr [rax + 0x50], xmm0
140986ded: 0f 10 45 58                 	movups	xmm0, xmmword ptr [rbp + 0x58]
140986df1: 0f 11 40 40                 	movups	xmmword ptr [rax + 0x40], xmm0
140986df5: 0f 10 45 18                 	movups	xmm0, xmmword ptr [rbp + 0x18]
140986df9: 0f 10 4d 28                 	movups	xmm1, xmmword ptr [rbp + 0x28]
140986dfd: 0f 10 55 38                 	movups	xmm2, xmmword ptr [rbp + 0x38]
140986e01: 0f 10 5d 48                 	movups	xmm3, xmmword ptr [rbp + 0x48]
140986e05: 0f 11 58 30                 	movups	xmmword ptr [rax + 0x30], xmm3
140986e09: 0f 11 50 20                 	movups	xmmword ptr [rax + 0x20], xmm2
140986e0d: 0f 11 48 10                 	movups	xmmword ptr [rax + 0x10], xmm1
140986e11: 0f 11 00                    	movups	xmmword ptr [rax], xmm0
140986e14: 0f 10 45 a8                 	movups	xmm0, xmmword ptr [rbp - 0x58]
140986e18: 0f 10 4d b8                 	movups	xmm1, xmmword ptr [rbp - 0x48]
140986e1c: 0f 10 55 c8                 	movups	xmm2, xmmword ptr [rbp - 0x38]
140986e20: 0f 10 5d d8                 	movups	xmm3, xmmword ptr [rbp - 0x28]
140986e24: 0f 11 40 70                 	movups	xmmword ptr [rax + 0x70], xmm0
140986e28: 0f 11 88 80 00 00 00        	movups	xmmword ptr [rax + 0x80], xmm1
140986e2f: 0f 11 90 90 00 00 00        	movups	xmmword ptr [rax + 0x90], xmm2
140986e36: 0f 11 98 a0 00 00 00        	movups	xmmword ptr [rax + 0xa0], xmm3
140986e3d: 0f 10 45 e8                 	movups	xmm0, xmmword ptr [rbp - 0x18]
140986e41: 0f 11 80 b0 00 00 00        	movups	xmmword ptr [rax + 0xb0], xmm0
140986e48: 0f 10 45 f8                 	movups	xmm0, xmmword ptr [rbp - 0x8]
140986e4c: 0f 11 80 c0 00 00 00        	movups	xmmword ptr [rax + 0xc0], xmm0
140986e53: 0f 10 45 08                 	movups	xmm0, xmmword ptr [rbp + 0x8]
140986e57: 0f 11 80 d0 00 00 00        	movups	xmmword ptr [rax + 0xd0], xmm0
140986e5e: 0f 10 85 e8 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xe8]
140986e65: 0f 11 80 40 01 00 00        	movups	xmmword ptr [rax + 0x140], xmm0
140986e6c: 0f 10 85 d8 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xd8]
140986e73: 0f 11 80 30 01 00 00        	movups	xmmword ptr [rax + 0x130], xmm0
140986e7a: 0f 10 85 c8 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xc8]
140986e81: 0f 11 80 20 01 00 00        	movups	xmmword ptr [rax + 0x120], xmm0
140986e88: 0f 10 85 88 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0x88]
140986e8f: 0f 10 8d 98 00 00 00        	movups	xmm1, xmmword ptr [rbp + 0x98]
140986e96: 0f 10 95 a8 00 00 00        	movups	xmm2, xmmword ptr [rbp + 0xa8]
140986e9d: 0f 10 9d b8 00 00 00        	movups	xmm3, xmmword ptr [rbp + 0xb8]
140986ea4: 0f 11 98 10 01 00 00        	movups	xmmword ptr [rax + 0x110], xmm3
140986eab: 0f 11 90 00 01 00 00        	movups	xmmword ptr [rax + 0x100], xmm2
140986eb2: 0f 11 88 f0 00 00 00        	movups	xmmword ptr [rax + 0xf0], xmm1
140986eb9: 0f 11 80 e0 00 00 00        	movups	xmmword ptr [rax + 0xe0], xmm0
140986ec0: 0f 10 85 f8 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xf8]
140986ec7: 0f 10 8d 08 01 00 00        	movups	xmm1, xmmword ptr [rbp + 0x108]
140986ece: 0f 10 95 18 01 00 00        	movups	xmm2, xmmword ptr [rbp + 0x118]
140986ed5: 0f 10 9d 28 01 00 00        	movups	xmm3, xmmword ptr [rbp + 0x128]
140986edc: 0f 11 80 50 01 00 00        	movups	xmmword ptr [rax + 0x150], xmm0
140986ee3: 0f 11 88 60 01 00 00        	movups	xmmword ptr [rax + 0x160], xmm1
140986eea: 0f 11 90 70 01 00 00        	movups	xmmword ptr [rax + 0x170], xmm2
140986ef1: 0f 11 98 80 01 00 00        	movups	xmmword ptr [rax + 0x180], xmm3
140986ef8: 0f 10 85 38 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x138]
140986eff: 0f 11 80 90 01 00 00        	movups	xmmword ptr [rax + 0x190], xmm0
140986f06: 0f 10 85 48 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x148]
140986f0d: 0f 11 80 a0 01 00 00        	movups	xmmword ptr [rax + 0x1a0], xmm0
140986f14: 0f 10 85 58 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x158]
140986f1b: 0f 11 80 b0 01 00 00        	movups	xmmword ptr [rax + 0x1b0], xmm0
140986f22: 48 c7 85 00 01 00 00 04 00 00 00    	mov	qword ptr [rbp + 0x100], 0x4
140986f2d: 48 89 85 08 01 00 00        	mov	qword ptr [rbp + 0x108], rax
140986f34: 48 c7 85 10 01 00 00 04 00 00 00    	mov	qword ptr [rbp + 0x110], 0x4
140986f3f: 48 c7 85 18 01 00 00 00 00 00 00    	mov	qword ptr [rbp + 0x118], 0x0
140986f4a: 48 c7 85 20 01 00 00 08 00 00 00    	mov	qword ptr [rbp + 0x120], 0x8
140986f55: 0f 57 c0                    	xorps	xmm0, xmm0
140986f58: 0f 11 85 28 01 00 00        	movups	xmmword ptr [rbp + 0x128], xmm0
140986f5f: 48 89 9d f8 00 00 00        	mov	qword ptr [rbp + 0xf8], rbx
140986f66: 48 8d 8d 30 01 00 00        	lea	rcx, [rbp + 0x130]
140986f6d: e8 2e 91 d1 00              	call	0x1416a00a0 <.text+0x169f0a0>
140986f72: 0f 10 85 58 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x158]
140986f79: 0f 11 46 60                 	movups	xmmword ptr [rsi + 0x60], xmm0
140986f7d: 0f 10 85 48 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x148]
140986f84: 0f 11 46 50                 	movups	xmmword ptr [rsi + 0x50], xmm0
140986f88: 0f 10 85 38 01 00 00        	movups	xmm0, xmmword ptr [rbp + 0x138]
140986f8f: 0f 11 46 40                 	movups	xmmword ptr [rsi + 0x40], xmm0
140986f93: 0f 10 85 f8 00 00 00        	movups	xmm0, xmmword ptr [rbp + 0xf8]
140986f9a: 0f 10 8d 08 01 00 00        	movups	xmm1, xmmword ptr [rbp + 0x108]
140986fa1: 0f 10 95 18 01 00 00        	movups	xmm2, xmmword ptr [rbp + 0x118]
140986fa8: 0f 10 9d 28 01 00 00        	movups	xmm3, xmmword ptr [rbp + 0x128]
140986faf: 0f 11 5e 30                 	movups	xmmword ptr [rsi + 0x30], xmm3
140986fb3: 0f 11 56 20                 	movups	xmmword ptr [rsi + 0x20], xmm2
140986fb7: 0f 11 4e 10                 	movups	xmmword ptr [rsi + 0x10], xmm1
140986fbb: 0f 11 06                    	movups	xmmword ptr [rsi], xmm0
140986fbe: 48 89 f0                    	mov	rax, rsi
140986fc1: 48 81 c4 18 03 00 00        	add	rsp, 0x318
140986fc8: 5b                          	pop	rbx
140986fc9: 5f                          	pop	rdi
140986fca: 5e                          	pop	rsi
140986fcb: 5d                          	pop	rbp
140986fcc: c3                          	ret
140986fcd: b9 08 00 00 00              	mov	ecx, 0x8
140986fd2: ba c0 01 00 00              	mov	edx, 0x1c0
140986fd7: e8 24 20 f5 01              	call	0x1428d9000 <.text+0x28d8000>
140986fdc: b9 08 00 00 00              	mov	ecx, 0x8
140986fe1: ba e0 00 00 00              	mov	edx, 0xe0
140986fe6: e8 15 20 f5 01              	call	0x1428d9000 <.text+0x28d8000>
140986feb: eb 0f                       	jmp	0x140986ffc <.text+0x985ffc>
140986fed: b9 08 00 00 00              	mov	ecx, 0x8
140986ff2: ba 88 01 00 00              	mov	edx, 0x188
140986ff7: e8 04 20 f5 01              	call	0x1428d9000 <.text+0x28d8000>
140986ffc: 0f 0b                       	ud2
140986ffe: 66 90                       	nop
140987000: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140987005: 55                          	push	rbp
140987006: 56                          	push	rsi
140987007: 57                          	push	rdi
140987008: 53                          	push	rbx
140987009: 48 83 ec 28                 	sub	rsp, 0x28
14098700d: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140987014: 48 8d 8d f8 00 00 00        	lea	rcx, [rbp + 0xf8]
14098701b: e8 30 9e c4 ff              	call	0x1405d0e50 <.text+0x5cfe50>
140987020: 90                          	nop
140987021: 48 83 c4 28                 	add	rsp, 0x28
140987025: 5b                          	pop	rbx
140987026: 5f                          	pop	rdi
140987027: 5e                          	pop	rsi
140987028: 5d                          	pop	rbp
140987029: c3                          	ret
14098702a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
140987030: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140987035: 55                          	push	rbp
140987036: 56                          	push	rsi
140987037: 57                          	push	rdi
140987038: 53                          	push	rbx
140987039: 48 83 ec 28                 	sub	rsp, 0x28
14098703d: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140987044: ba e0 00 00 00              	mov	edx, 0xe0
140987049: 41 b8 08 00 00 00           	mov	r8d, 0x8
14098704f: 48 8b 8d 88 02 00 00        	mov	rcx, qword ptr [rbp + 0x288]
140987056: e8 c5 cb c8 ff              	call	0x140613c20 <.text+0x612c20>
14098705b: 90                          	nop
14098705c: 48 83 c4 28                 	add	rsp, 0x28
140987060: 5b                          	pop	rbx
140987061: 5f                          	pop	rdi
140987062: 5e                          	pop	rsi
140987063: 5d                          	pop	rbp
140987064: c3                          	ret
140987065: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
140987070: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140987075: 55                          	push	rbp
140987076: 56                          	push	rsi
140987077: 57                          	push	rdi
140987078: 53                          	push	rbx
140987079: 48 83 ec 28                 	sub	rsp, 0x28
14098707d: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140987084: 48 8d 8d 88 00 00 00        	lea	rcx, [rbp + 0x88]
14098708b: e8 f0 27 6a ff              	call	0x140029880 <.text+0x28880>
140987090: 90                          	nop
140987091: 48 83 c4 28                 	add	rsp, 0x28
140987095: 5b                          	pop	rbx
140987096: 5f                          	pop	rdi
140987097: 5e                          	pop	rsi
140987098: 5d                          	pop	rbp
140987099: c3                          	ret
14098709a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
1409870a0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409870a5: 55                          	push	rbp
1409870a6: 56                          	push	rsi
1409870a7: 57                          	push	rdi
1409870a8: 53                          	push	rbx
1409870a9: 48 83 ec 28                 	sub	rsp, 0x28
1409870ad: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409870b4: ba c0 01 00 00              	mov	edx, 0x1c0
1409870b9: 41 b8 08 00 00 00           	mov	r8d, 0x8
1409870bf: 48 8b 8d 80 02 00 00        	mov	rcx, qword ptr [rbp + 0x280]
1409870c6: e8 55 cb c8 ff              	call	0x140613c20 <.text+0x612c20>
1409870cb: 90                          	nop
1409870cc: 48 83 c4 28                 	add	rsp, 0x28
1409870d0: 5b                          	pop	rbx
1409870d1: 5f                          	pop	rdi
1409870d2: 5e                          	pop	rsi
1409870d3: 5d                          	pop	rbp
1409870d4: c3                          	ret
1409870d5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nop	word ptr cs:[rax + rax]
1409870e0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
1409870e5: 55                          	push	rbp
1409870e6: 56                          	push	rsi
1409870e7: 57                          	push	rdi
1409870e8: 53                          	push	rbx
1409870e9: 48 83 ec 28                 	sub	rsp, 0x28
1409870ed: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
1409870f4: 48 8d 4d 18                 	lea	rcx, [rbp + 0x18]
1409870f8: e8 83 27 6a ff              	call	0x140029880 <.text+0x28880>
1409870fd: 90                          	nop
1409870fe: 48 83 c4 28                 	add	rsp, 0x28
140987102: 5b                          	pop	rbx
140987103: 5f                          	pop	rdi
140987104: 5e                          	pop	rsi
140987105: 5d                          	pop	rbp
140987106: c3                          	ret
140987107: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
140987110: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140987115: 55                          	push	rbp
140987116: 56                          	push	rsi
140987117: 57                          	push	rdi
140987118: 53                          	push	rbx
140987119: 48 83 ec 28                 	sub	rsp, 0x28
14098711d: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140987124: 48 8d 4d a8                 	lea	rcx, [rbp - 0x58]
140987128: e8 53 27 6a ff              	call	0x140029880 <.text+0x28880>
14098712d: 90                          	nop
14098712e: 48 83 c4 28                 	add	rsp, 0x28
140987132: 5b                          	pop	rbx
140987133: 5f                          	pop	rdi
140987134: 5e                          	pop	rsi
140987135: 5d                          	pop	rbp
140987136: c3                          	ret
140987137: 66 0f 1f 84 00 00 00 00 00  	nop	word ptr [rax + rax]
140987140: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140987145: 55                          	push	rbp
140987146: 56                          	push	rsi
140987147: 57                          	push	rdi
140987148: 53                          	push	rbx
140987149: 48 83 ec 28                 	sub	rsp, 0x28
14098714d: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140987154: 48 8d 8d 88 00 00 00        	lea	rcx, [rbp + 0x88]
14098715b: e8 20 27 6a ff              	call	0x140029880 <.text+0x28880>
140987160: 90                          	nop
140987161: 48 83 c4 28                 	add	rsp, 0x28
140987165: 5b                          	pop	rbx
140987166: 5f                          	pop	rdi
140987167: 5e                          	pop	rsi
140987168: 5d                          	pop	rbp
140987169: c3                          	ret
14098716a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
140987170: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140987175: 55                          	push	rbp
140987176: 56                          	push	rsi
140987177: 57                          	push	rdi
140987178: 53                          	push	rbx
140987179: 48 83 ec 28                 	sub	rsp, 0x28
14098717d: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140987184: 48 8d 8d f8 00 00 00        	lea	rcx, [rbp + 0xf8]
14098718b: e8 f0 26 6a ff              	call	0x140029880 <.text+0x28880>
140987190: 90                          	nop
140987191: 48 83 c4 28                 	add	rsp, 0x28
140987195: 5b                          	pop	rbx
140987196: 5f                          	pop	rdi
140987197: 5e                          	pop	rsi
140987198: 5d                          	pop	rbp
140987199: c3                          	ret
