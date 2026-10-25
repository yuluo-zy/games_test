
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a2c790: 55                          	push	rbp
140a2c791: 56                          	push	rsi
140a2c792: 57                          	push	rdi
140a2c793: 48 81 ec d0 01 00 00        	sub	rsp, 0x1d0
140a2c79a: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
140a2c7a2: 48 c7 85 48 01 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x148], -0x2
140a2c7ad: 48 89 ce                    	mov	rsi, rcx
140a2c7b0: 0f 57 c0                    	xorps	xmm0, xmm0
140a2c7b3: 0f 11 45 c8                 	movups	xmmword ptr [rbp - 0x38], xmm0
140a2c7b7: 0f 11 45 e0                 	movups	xmmword ptr [rbp - 0x20], xmm0
140a2c7bb: 0f 11 45 f8                 	movups	xmmword ptr [rbp - 0x8], xmm0
140a2c7bf: 0f 11 45 10                 	movups	xmmword ptr [rbp + 0x10], xmm0
140a2c7c3: 0f 11 45 28                 	movups	xmmword ptr [rbp + 0x28], xmm0
140a2c7c7: c7 45 38 00 00 00 00        	mov	dword ptr [rbp + 0x38], 0x0
140a2c7ce: 0f 11 45 60                 	movups	xmmword ptr [rbp + 0x60], xmm0
140a2c7d2: 0f 11 45 78                 	movups	xmmword ptr [rbp + 0x78], xmm0
140a2c7d6: 0f 11 85 90 00 00 00        	movups	xmmword ptr [rbp + 0x90], xmm0
140a2c7dd: 0f 11 85 a8 00 00 00        	movups	xmmword ptr [rbp + 0xa8], xmm0
140a2c7e4: 0f 11 85 c0 00 00 00        	movups	xmmword ptr [rbp + 0xc0], xmm0
140a2c7eb: c7 85 d0 00 00 00 00 00 00 00       	mov	dword ptr [rbp + 0xd0], 0x0
140a2c7f5: 48 c7 45 a8 00 00 00 00     	mov	qword ptr [rbp - 0x58], 0x0
140a2c7fd: 48 c7 45 b0 08 00 00 00     	mov	qword ptr [rbp - 0x50], 0x8
140a2c805: 48 c7 45 b8 00 00 00 00     	mov	qword ptr [rbp - 0x48], 0x0
140a2c80d: 48 c7 45 c0 10 00 00 00     	mov	qword ptr [rbp - 0x40], 0x10
140a2c815: 48 c7 45 d8 10 00 00 00     	mov	qword ptr [rbp - 0x28], 0x10
140a2c81d: 48 c7 45 f0 10 00 00 00     	mov	qword ptr [rbp - 0x10], 0x10
140a2c825: 48 c7 45 08 10 00 00 00     	mov	qword ptr [rbp + 0x8], 0x10
140a2c82d: 48 c7 45 20 10 00 00 00     	mov	qword ptr [rbp + 0x20], 0x10
140a2c835: 48 b8 00 00 00 00 00 00 00 80       	movabs	rax, -0x8000000000000000
140a2c83f: 48 89 45 40                 	mov	qword ptr [rbp + 0x40], rax
140a2c843: 48 8d 0d b6 ed 09 02        	lea	rcx, [rip + 0x209edb6]  # 0x142acb600
140a2c84a: 48 89 4d 48                 	mov	qword ptr [rbp + 0x48], rcx
140a2c84e: 48 c7 45 50 71 00 00 00     	mov	qword ptr [rbp + 0x50], 0x71
140a2c856: 48 c7 45 58 10 00 00 00     	mov	qword ptr [rbp + 0x58], 0x10
140a2c85e: 48 c7 45 70 10 00 00 00     	mov	qword ptr [rbp + 0x70], 0x10
140a2c866: 48 c7 85 88 00 00 00 10 00 00 00    	mov	qword ptr [rbp + 0x88], 0x10
140a2c871: 48 c7 85 a0 00 00 00 10 00 00 00    	mov	qword ptr [rbp + 0xa0], 0x10
140a2c87c: 48 c7 85 b8 00 00 00 10 00 00 00    	mov	qword ptr [rbp + 0xb8], 0x10
140a2c887: c7 85 d8 00 00 00 00 00 00 00       	mov	dword ptr [rbp + 0xd8], 0x0
140a2c891: 66 c7 85 dc 00 00 00 01 00  	mov	word ptr [rbp + 0xdc], 0x1
140a2c89a: 48 89 85 00 01 00 00        	mov	qword ptr [rbp + 0x100], rax
140a2c8a1: c7 85 40 01 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x140], 0x0
140a2c8ab: 0f b6 05 57 83 c0 02        	movzx	eax, byte ptr [rip + 0x2c08357] # 0x143634c09
140a2c8b2: b9 a0 01 00 00              	mov	ecx, 0x1a0
140a2c8b7: ba 08 00 00 00              	mov	edx, 0x8
140a2c8bc: e8 4f 73 be ff              	call	0x140613c10 <.text+0x612c10>
140a2c8c1: 48 85 c0                    	test	rax, rax
140a2c8c4: 74 35                       	je	0x140a2c8fb <.text+0xa2b8fb>
140a2c8c6: 48 89 c7                    	mov	rdi, rax
140a2c8c9: 48 8d 55 a8                 	lea	rdx, [rbp - 0x58]
140a2c8cd: 41 b8 a0 01 00 00           	mov	r8d, 0x1a0
140a2c8d3: 48 89 c1                    	mov	rcx, rax
140a2c8d6: e8 25 73 ef 01              	call	0x142923c00 <.text+0x2922c00>
140a2c8db: 4c 8d 05 86 e9 09 02        	lea	r8, [rip + 0x209e986]   # 0x142acb268
140a2c8e2: 48 89 f1                    	mov	rcx, rsi
140a2c8e5: 48 89 fa                    	mov	rdx, rdi
140a2c8e8: e8 23 82 2b 00              	call	0x140ce4b10 <.text+0xce3b10>
140a2c8ed: 48 89 f0                    	mov	rax, rsi
140a2c8f0: 48 81 c4 d0 01 00 00        	add	rsp, 0x1d0
140a2c8f7: 5f                          	pop	rdi
140a2c8f8: 5e                          	pop	rsi
140a2c8f9: 5d                          	pop	rbp
140a2c8fa: c3                          	ret
140a2c8fb: b9 08 00 00 00              	mov	ecx, 0x8
140a2c900: ba a0 01 00 00              	mov	edx, 0x1a0
140a2c905: e8 f6 c6 ea 01              	call	0x1428d9000 <.text+0x28d8000>
140a2c90a: 0f 0b                       	ud2
140a2c90c: 0f 1f 40 00                 	nop	dword ptr [rax]
140a2c910: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a2c915: 55                          	push	rbp
140a2c916: 56                          	push	rsi
140a2c917: 57                          	push	rdi
140a2c918: 48 83 ec 20                 	sub	rsp, 0x20
140a2c91c: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a2c923: 48 8d 4d a8                 	lea	rcx, [rbp - 0x58]
140a2c927: e8 e4 2a bc ff              	call	0x1405ef410 <.text+0x5ee410>
140a2c92c: 90                          	nop
140a2c92d: 48 83 c4 20                 	add	rsp, 0x20
140a2c931: 5f                          	pop	rdi
140a2c932: 5e                          	pop	rsi
140a2c933: 5d                          	pop	rbp
140a2c934: c3                          	ret
