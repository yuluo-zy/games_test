
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b1aca0: 55                          	push	rbp
140b1aca1: 41 56                       	push	r14
140b1aca3: 56                          	push	rsi
140b1aca4: 57                          	push	rdi
140b1aca5: 53                          	push	rbx
140b1aca6: 48 81 ec 60 01 00 00        	sub	rsp, 0x160
140b1acad: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
140b1acb5: 48 c7 85 d8 00 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0xd8], -0x2
140b1acc0: 4c 89 c6                    	mov	rsi, r8
140b1acc3: 48 89 d7                    	mov	rdi, rdx
140b1acc6: 48 89 ca                    	mov	rdx, rcx
140b1acc9: c6 85 d7 00 00 00 01        	mov	byte ptr [rbp + 0xd7], 0x1
140b1acd0: 4c 89 c9                    	mov	rcx, r9
140b1acd3: 48 89 95 c8 00 00 00        	mov	qword ptr [rbp + 0xc8], rdx
140b1acda: e8 01 fa d9 ff              	call	0x1408ba6e0 <.text+0x8b96e0>
140b1acdf: 48 8b 8d c8 00 00 00        	mov	rcx, qword ptr [rbp + 0xc8]
140b1ace6: 48 8b 41 70                 	mov	rax, qword ptr [rcx + 0x70]
140b1acea: 48 89 85 a0 00 00 00        	mov	qword ptr [rbp + 0xa0], rax
140b1acf1: 0f 10 41 60                 	movups	xmm0, xmmword ptr [rcx + 0x60]
140b1acf5: 0f 29 85 90 00 00 00        	movaps	xmmword ptr [rbp + 0x90], xmm0
140b1acfc: 0f 10 41 50                 	movups	xmm0, xmmword ptr [rcx + 0x50]
140b1ad00: 0f 29 85 80 00 00 00        	movaps	xmmword ptr [rbp + 0x80], xmm0
140b1ad07: 0f 10 41 40                 	movups	xmm0, xmmword ptr [rcx + 0x40]
140b1ad0b: 0f 29 45 70                 	movaps	xmmword ptr [rbp + 0x70], xmm0
140b1ad0f: 0f 10 01                    	movups	xmm0, xmmword ptr [rcx]
140b1ad12: 0f 10 49 10                 	movups	xmm1, xmmword ptr [rcx + 0x10]
140b1ad16: 0f 10 51 20                 	movups	xmm2, xmmword ptr [rcx + 0x20]
140b1ad1a: 0f 10 59 30                 	movups	xmm3, xmmword ptr [rcx + 0x30]
140b1ad1e: 0f 29 5d 60                 	movaps	xmmword ptr [rbp + 0x60], xmm3
140b1ad22: 0f 29 55 50                 	movaps	xmmword ptr [rbp + 0x50], xmm2
140b1ad26: 0f 29 4d 40                 	movaps	xmmword ptr [rbp + 0x40], xmm1
140b1ad2a: 0f 29 45 30                 	movaps	xmmword ptr [rbp + 0x30], xmm0
140b1ad2e: 48 b8 00 00 00 00 00 00 00 80       	movabs	rax, -0x8000000000000000
140b1ad38: 48 89 85 a8 00 00 00        	mov	qword ptr [rbp + 0xa8], rax
140b1ad3f: 48 85 f6                    	test	rsi, rsi
140b1ad42: 0f 88 8c 00 00 00           	js	0x140b1add4 <.text+0xb19dd4>
140b1ad48: 74 24                       	je	0x140b1ad6e <.text+0xb19d6e>
140b1ad4a: 0f b6 05 b8 9e b1 02        	movzx	eax, byte ptr [rip + 0x2b19eb8] # 0x143634c09
140b1ad51: 41 be 01 00 00 00           	mov	r14d, 0x1
140b1ad57: ba 01 00 00 00              	mov	edx, 0x1
140b1ad5c: 48 89 f1                    	mov	rcx, rsi
140b1ad5f: e8 ac 8e af ff              	call	0x140613c10 <.text+0x612c10>
140b1ad64: 48 85 c0                    	test	rax, rax
140b1ad67: 74 6e                       	je	0x140b1add7 <.text+0xb19dd7>
140b1ad69: 48 89 c3                    	mov	rbx, rax
140b1ad6c: eb 05                       	jmp	0x140b1ad73 <.text+0xb19d73>
140b1ad6e: bb 01 00 00 00              	mov	ebx, 0x1
140b1ad73: 4c 8b b5 30 01 00 00        	mov	r14, qword ptr [rbp + 0x130]
140b1ad7a: 48 89 d9                    	mov	rcx, rbx
140b1ad7d: 48 89 fa                    	mov	rdx, rdi
140b1ad80: 49 89 f0                    	mov	r8, rsi
140b1ad83: e8 78 8e e0 01              	call	0x142923c00 <.text+0x2922c00>
140b1ad88: 48 89 b5 a8 00 00 00        	mov	qword ptr [rbp + 0xa8], rsi
140b1ad8f: 48 89 9d b0 00 00 00        	mov	qword ptr [rbp + 0xb0], rbx
140b1ad96: 48 89 b5 b8 00 00 00        	mov	qword ptr [rbp + 0xb8], rsi
140b1ad9d: 48 8d 75 a0                 	lea	rsi, [rbp - 0x60]
140b1ada1: 48 8d 55 30                 	lea	rdx, [rbp + 0x30]
140b1ada5: 41 b8 90 00 00 00           	mov	r8d, 0x90
140b1adab: 48 89 f1                    	mov	rcx, rsi
140b1adae: e8 4d 8e e0 01              	call	0x142923c00 <.text+0x2922c00>
140b1adb3: c6 85 d7 00 00 00 00        	mov	byte ptr [rbp + 0xd7], 0x0
140b1adba: 4c 89 f1                    	mov	rcx, r14
140b1adbd: 48 89 f2                    	mov	rdx, rsi
140b1adc0: e8 9b 1e e1 ff              	call	0x14092cc60 <.text+0x92bc60>
140b1adc5: 90                          	nop
140b1adc6: 48 81 c4 60 01 00 00        	add	rsp, 0x160
140b1adcd: 5b                          	pop	rbx
140b1adce: 5f                          	pop	rdi
140b1adcf: 5e                          	pop	rsi
140b1add0: 41 5e                       	pop	r14
140b1add2: 5d                          	pop	rbp
140b1add3: c3                          	ret
140b1add4: 45 31 f6                    	xor	r14d, r14d
140b1add7: 4c 8d 05 da 41 11 02        	lea	r8, [rip + 0x21141da]   # 0x142c2efb8
140b1adde: 4c 89 f1                    	mov	rcx, r14
140b1ade1: 48 89 f2                    	mov	rdx, rsi
140b1ade4: e8 fa e1 db 01              	call	0x1428d8fe3 <.text+0x28d7fe3>
140b1ade9: 0f 0b                       	ud2
140b1adeb: 0f 1f 44 00 00              	nop	dword ptr [rax + rax]
140b1adf0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140b1adf5: 55                          	push	rbp
140b1adf6: 41 56                       	push	r14
140b1adf8: 56                          	push	rsi
140b1adf9: 57                          	push	rdi
140b1adfa: 53                          	push	rbx
140b1adfb: 48 83 ec 20                 	sub	rsp, 0x20
140b1adff: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140b1ae06: 80 bd d7 00 00 00 00        	cmp	byte ptr [rbp + 0xd7], 0x0
140b1ae0d: 75 0b                       	jne	0x140b1ae1a <.text+0xb19e1a>
140b1ae0f: 48 83 c4 20                 	add	rsp, 0x20
140b1ae13: 5b                          	pop	rbx
140b1ae14: 5f                          	pop	rdi
140b1ae15: 5e                          	pop	rsi
140b1ae16: 41 5e                       	pop	r14
140b1ae18: 5d                          	pop	rbp
140b1ae19: c3                          	ret
140b1ae1a: 48 8b 8d c8 00 00 00        	mov	rcx, qword ptr [rbp + 0xc8]
140b1ae21: e8 0a dd d6 ff              	call	0x140888b30 <.text+0x887b30>
140b1ae26: eb e7                       	jmp	0x140b1ae0f <.text+0xb19e0f>
140b1ae28: 0f 1f 84 00 00 00 00 00     	nop	dword ptr [rax + rax]
140b1ae30: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140b1ae35: 55                          	push	rbp
140b1ae36: 41 56                       	push	r14
140b1ae38: 56                          	push	rsi
140b1ae39: 57                          	push	rdi
140b1ae3a: 53                          	push	rbx
140b1ae3b: 48 83 ec 20                 	sub	rsp, 0x20
140b1ae3f: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140b1ae46: c6 85 d7 00 00 00 00        	mov	byte ptr [rbp + 0xd7], 0x0
140b1ae4d: 48 8d 4d 30                 	lea	rcx, [rbp + 0x30]
140b1ae51: e8 ea de d7 ff              	call	0x140898d40 <.text+0x897d40>
140b1ae56: c6 85 d7 00 00 00 00        	mov	byte ptr [rbp + 0xd7], 0x0
140b1ae5d: 48 83 c4 20                 	add	rsp, 0x20
140b1ae61: 5b                          	pop	rbx
140b1ae62: 5f                          	pop	rdi
140b1ae63: 5e                          	pop	rsi
140b1ae64: 41 5e                       	pop	r14
140b1ae66: 5d                          	pop	rbp
140b1ae67: c3                          	ret
