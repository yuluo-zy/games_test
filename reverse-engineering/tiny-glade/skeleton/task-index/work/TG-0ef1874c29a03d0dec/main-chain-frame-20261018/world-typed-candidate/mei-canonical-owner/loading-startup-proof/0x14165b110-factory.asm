
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14165b110: 55                          	push	rbp
14165b111: 56                          	push	rsi
14165b112: 57                          	push	rdi
14165b113: 48 81 ec 90 01 00 00        	sub	rsp, 0x190
14165b11a: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
14165b122: 48 c7 85 08 01 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x108], -0x2
14165b12d: 48 89 ce                    	mov	rsi, rcx
14165b130: 0f 57 c0                    	xorps	xmm0, xmm0
14165b133: 0f 11 45 e8                 	movups	xmmword ptr [rbp - 0x18], xmm0
14165b137: 0f 11 45 00                 	movups	xmmword ptr [rbp], xmm0
14165b13b: 0f 11 45 18                 	movups	xmmword ptr [rbp + 0x18], xmm0
14165b13f: 0f 11 45 30                 	movups	xmmword ptr [rbp + 0x30], xmm0
14165b143: 0f 11 45 48                 	movups	xmmword ptr [rbp + 0x48], xmm0
14165b147: c7 45 58 00 00 00 00        	mov	dword ptr [rbp + 0x58], 0x0
14165b14e: 0f 11 85 80 00 00 00        	movups	xmmword ptr [rbp + 0x80], xmm0
14165b155: 0f 11 85 98 00 00 00        	movups	xmmword ptr [rbp + 0x98], xmm0
14165b15c: 0f 11 85 b0 00 00 00        	movups	xmmword ptr [rbp + 0xb0], xmm0
14165b163: 0f 11 85 c8 00 00 00        	movups	xmmword ptr [rbp + 0xc8], xmm0
14165b16a: 0f 11 85 e0 00 00 00        	movups	xmmword ptr [rbp + 0xe0], xmm0
14165b171: c7 85 f0 00 00 00 00 00 00 00       	mov	dword ptr [rbp + 0xf0], 0x0
14165b17b: 48 c7 45 a0 00 00 00 00     	mov	qword ptr [rbp - 0x60], 0x0
14165b183: 48 c7 45 c8 00 00 00 00     	mov	qword ptr [rbp - 0x38], 0x0
14165b18b: 48 c7 45 d0 08 00 00 00     	mov	qword ptr [rbp - 0x30], 0x8
14165b193: 48 c7 45 d8 00 00 00 00     	mov	qword ptr [rbp - 0x28], 0x0
14165b19b: 48 c7 45 e0 10 00 00 00     	mov	qword ptr [rbp - 0x20], 0x10
14165b1a3: 48 c7 45 f8 10 00 00 00     	mov	qword ptr [rbp - 0x8], 0x10
14165b1ab: 48 c7 45 10 10 00 00 00     	mov	qword ptr [rbp + 0x10], 0x10
14165b1b3: 48 c7 45 28 10 00 00 00     	mov	qword ptr [rbp + 0x28], 0x10
14165b1bb: 48 c7 45 40 10 00 00 00     	mov	qword ptr [rbp + 0x40], 0x10
14165b1c3: 48 b8 00 00 00 00 00 00 00 80       	movabs	rax, -0x8000000000000000
14165b1cd: 48 89 45 60                 	mov	qword ptr [rbp + 0x60], rax
14165b1d1: 48 8d 05 f0 6f 62 01        	lea	rax, [rip + 0x1626ff0]  # 0x142c821c8
14165b1d8: 48 89 45 68                 	mov	qword ptr [rbp + 0x68], rax
14165b1dc: 48 c7 45 70 31 00 00 00     	mov	qword ptr [rbp + 0x70], 0x31
14165b1e4: 48 c7 45 78 10 00 00 00     	mov	qword ptr [rbp + 0x78], 0x10
14165b1ec: 48 c7 85 90 00 00 00 10 00 00 00    	mov	qword ptr [rbp + 0x90], 0x10
14165b1f7: 48 c7 85 a8 00 00 00 10 00 00 00    	mov	qword ptr [rbp + 0xa8], 0x10
14165b202: 48 c7 85 c0 00 00 00 10 00 00 00    	mov	qword ptr [rbp + 0xc0], 0x10
14165b20d: 48 c7 85 d8 00 00 00 10 00 00 00    	mov	qword ptr [rbp + 0xd8], 0x10
14165b218: c7 85 f8 00 00 00 00 00 00 00       	mov	dword ptr [rbp + 0xf8], 0x0
14165b222: 66 c7 85 fc 00 00 00 01 00  	mov	word ptr [rbp + 0xfc], 0x1
14165b22b: c7 85 00 01 00 00 00 00 00 00       	mov	dword ptr [rbp + 0x100], 0x0
14165b235: 0f b6 05 cd 99 fd 01        	movzx	eax, byte ptr [rip + 0x1fd99cd] # 0x143634c09
14165b23c: b9 68 01 00 00              	mov	ecx, 0x168
14165b241: ba 08 00 00 00              	mov	edx, 0x8
14165b246: e8 c5 89 fb fe              	call	0x140613c10 <.text+0x612c10>
14165b24b: 48 85 c0                    	test	rax, rax
14165b24e: 74 35                       	je	0x14165b285 <.text+0x165a285>
14165b250: 48 89 c7                    	mov	rdi, rax
14165b253: 48 8d 55 a0                 	lea	rdx, [rbp - 0x60]
14165b257: 41 b8 68 01 00 00           	mov	r8d, 0x168
14165b25d: 48 89 c1                    	mov	rcx, rax
14165b260: e8 9b 89 2c 01              	call	0x142923c00 <.text+0x2922c00>
14165b265: 4c 8d 05 c4 6a 62 01        	lea	r8, [rip + 0x1626ac4]   # 0x142c81d30
14165b26c: 48 89 f1                    	mov	rcx, rsi
14165b26f: 48 89 fa                    	mov	rdx, rdi
14165b272: e8 99 98 68 ff              	call	0x140ce4b10 <.text+0xce3b10>
14165b277: 48 89 f0                    	mov	rax, rsi
14165b27a: 48 81 c4 90 01 00 00        	add	rsp, 0x190
14165b281: 5f                          	pop	rdi
14165b282: 5e                          	pop	rsi
14165b283: 5d                          	pop	rbp
14165b284: c3                          	ret
14165b285: b9 08 00 00 00              	mov	ecx, 0x8
14165b28a: ba 68 01 00 00              	mov	edx, 0x168
14165b28f: e8 6c dd 27 01              	call	0x1428d9000 <.text+0x28d8000>
14165b294: 0f 0b                       	ud2
14165b296: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
14165b2a0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
14165b2a5: 55                          	push	rbp
14165b2a6: 56                          	push	rsi
14165b2a7: 57                          	push	rdi
14165b2a8: 48 83 ec 20                 	sub	rsp, 0x20
14165b2ac: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
14165b2b3: 48 8d 4d a0                 	lea	rcx, [rbp - 0x60]
14165b2b7: e8 54 bc f8 fe              	call	0x1405e6f10 <.text+0x5e5f10>
14165b2bc: 90                          	nop
14165b2bd: 48 83 c4 20                 	add	rsp, 0x20
14165b2c1: 5f                          	pop	rdi
14165b2c2: 5e                          	pop	rsi
14165b2c3: 5d                          	pop	rbp
14165b2c4: c3                          	ret
