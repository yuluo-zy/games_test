
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
142080d70: 56                          	push	rsi
142080d71: 57                          	push	rdi
142080d72: 53                          	push	rbx
142080d73: 48 83 ec 20                 	sub	rsp, 0x20
142080d77: 4c 8b 42 08                 	mov	r8, qword ptr [rdx + 0x8]
142080d7b: 4c 8b 4a 10                 	mov	r9, qword ptr [rdx + 0x10]
142080d7f: 49 8d 41 10                 	lea	rax, [r9 + 0x10]
142080d83: 4c 39 c0                    	cmp	rax, r8
142080d86: 73 4b                       	jae	0x142080dd3 <.text+0x207fdd3>
142080d88: 4d 39 c8                    	cmp	r8, r9
142080d8b: 4d 89 ca                    	mov	r10, r9
142080d8e: 4d 0f 42 d0                 	cmovb	r10, r8
142080d92: 4d 89 c3                    	mov	r11, r8
142080d95: 4d 29 d3                    	sub	r11, r10
142080d98: 49 83 fb 08                 	cmp	r11, 0x8
142080d9c: 72 17                       	jb	0x142080db5 <.text+0x207fdb5>
142080d9e: 49 83 c1 08                 	add	r9, 0x8
142080da2: 4d 39 c8                    	cmp	r8, r9
142080da5: 4d 0f 42 c8                 	cmovb	r9, r8
142080da9: 4d 89 c3                    	mov	r11, r8
142080dac: 4d 29 cb                    	sub	r11, r9
142080daf: 49 83 fb 08                 	cmp	r11, 0x8
142080db3: 73 2e                       	jae	0x142080de3 <.text+0x207fde3>
142080db5: 4c 89 42 10                 	mov	qword ptr [rdx + 0x10], r8
142080db9: 48 8d 05 88 38 ff 00        	lea	rax, [rip + 0xff3888]   # 0x143074648
142080dc0: 48 c7 41 08 00 00 00 00     	mov	qword ptr [rcx + 0x8], 0x0
142080dc8: 48 89 41 10                 	mov	qword ptr [rcx + 0x10], rax
142080dcc: b8 01 00 00 00              	mov	eax, 0x1
142080dd1: eb 02                       	jmp	0x142080dd5 <.text+0x207fdd5>
142080dd3: 31 c0                       	xor	eax, eax
142080dd5: 48 89 01                    	mov	qword ptr [rcx], rax
142080dd8: 48 89 c8                    	mov	rax, rcx
142080ddb: 48 83 c4 20                 	add	rsp, 0x20
142080ddf: 5b                          	pop	rbx
142080de0: 5f                          	pop	rdi
142080de1: 5e                          	pop	rsi
142080de2: c3                          	ret
142080de3: 4c 8b 1a                    	mov	r11, qword ptr [rdx]
142080de6: 4f 8b 14 13                 	mov	r10, qword ptr [r11 + r10]
142080dea: 4b 8b 34 0b                 	mov	rsi, qword ptr [r11 + r9]
142080dee: 48 89 42 10                 	mov	qword ptr [rdx + 0x10], rax
142080df2: 49 89 c1                    	mov	r9, rax
142080df5: 49 01 f1                    	add	r9, rsi
142080df8: 0f 92 c3                    	setb	bl
142080dfb: 48 85 f6                    	test	rsi, rsi
142080dfe: 40 0f 98 c7                 	sets	dil
142080e02: 40 30 df                    	xor	dil, bl
142080e05: 75 2b                       	jne	0x142080e32 <.text+0x207fe32>
142080e07: 4c 89 4a 10                 	mov	qword ptr [rdx + 0x10], r9
142080e0b: 4d 39 c1                    	cmp	r9, r8
142080e0e: 76 0c                       	jbe	0x142080e1c <.text+0x207fe1c>
142080e10: 48 b8 03 00 00 00 25 00 00 00       	movabs	rax, 0x2500000003
142080e1a: eb a4                       	jmp	0x142080dc0 <.text+0x207fdc0>
142080e1c: 49 39 c1                    	cmp	r9, rax
142080e1f: 72 1a                       	jb	0x142080e3b <.text+0x207fe3b>
142080e21: 49 01 c3                    	add	r11, rax
142080e24: 4c 89 59 08                 	mov	qword ptr [rcx + 0x8], r11
142080e28: 48 89 71 10                 	mov	qword ptr [rcx + 0x10], rsi
142080e2c: 4c 89 51 18                 	mov	qword ptr [rcx + 0x18], r10
142080e30: eb 9a                       	jmp	0x142080dcc <.text+0x207fdcc>
142080e32: 48 8d 05 5f 38 ff 00        	lea	rax, [rip + 0xff385f]   # 0x143074698
142080e39: eb 85                       	jmp	0x142080dc0 <.text+0x207fdc0>
142080e3b: 4c 8d 05 6e 4a f5 00        	lea	r8, [rip + 0xf54a6e]    # 0x142fd58b0
142080e42: 48 89 c1                    	mov	rcx, rax
142080e45: 4c 89 ca                    	mov	rdx, r9
142080e48: e8 03 8a 85 00              	call	0x1428d9850 <.text+0x28d8850>
142080e4d: cc                          	int3
