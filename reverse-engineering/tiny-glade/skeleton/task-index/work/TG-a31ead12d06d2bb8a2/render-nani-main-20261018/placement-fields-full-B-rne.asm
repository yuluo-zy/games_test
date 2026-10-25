
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b3b960: 56                          	push	rsi
140b3b961: 57                          	push	rdi
140b3b962: 53                          	push	rbx
140b3b963: 48 83 ec 20                 	sub	rsp, 0x20
140b3b967: 48 89 c8                    	mov	rax, rcx
140b3b96a: 49 8d 48 fe                 	lea	rcx, [r8 - 0x2]
140b3b96e: 48 83 f9 0e                 	cmp	rcx, 0xe
140b3b972: 0f 87 2d 01 00 00           	ja	0x140b3baa5 <.text+0xb3aaa5>
140b3b978: 4c 8d 0d 09 2b fb 01        	lea	r9, [rip + 0x1fb2b09]   # 0x142aee488
140b3b97f: 49 63 0c 89                 	movsxd	rcx, dword ptr [r9 + 4*rcx]
140b3b983: 4c 01 c9                    	add	rcx, r9
140b3b986: ff e1                       	jmp	rcx
140b3b988: 66 81 3a 69 64              	cmp	word ptr [rdx], 0x6469
140b3b98d: 0f 85 12 01 00 00           	jne	0x140b3baa5 <.text+0xb3aaa5>
140b3b993: c6 40 01 03                 	mov	byte ptr [rax + 0x1], 0x3
140b3b997: e9 10 01 00 00              	jmp	0x140b3baac <.text+0xb3aaac>
140b3b99c: 4c 8d 0d bb 28 fb 01        	lea	r9, [rip + 0x1fb28bb]   # 0x142aee25e
140b3b9a3: 48 89 d1                    	mov	rcx, rdx
140b3b9a6: 4c 89 ca                    	mov	rdx, r9
140b3b9a9: 48 89 c6                    	mov	rsi, rax
140b3b9ac: e8 3f 82 de 01              	call	0x142923bf0 <.text+0x2922bf0>
140b3b9b1: 89 c1                       	mov	ecx, eax
140b3b9b3: 48 89 f0                    	mov	rax, rsi
140b3b9b6: 85 c9                       	test	ecx, ecx
140b3b9b8: 0f 85 e7 00 00 00           	jne	0x140b3baa5 <.text+0xb3aaa5>
140b3b9be: c6 40 01 06                 	mov	byte ptr [rax + 0x1], 0x6
140b3b9c2: e9 e5 00 00 00              	jmp	0x140b3baac <.text+0xb3aaac>
140b3b9c7: 4c 8d 0d 9b 28 fb 01        	lea	r9, [rip + 0x1fb289b]   # 0x142aee269
140b3b9ce: 48 89 d1                    	mov	rcx, rdx
140b3b9d1: 4c 89 ca                    	mov	rdx, r9
140b3b9d4: 48 89 c6                    	mov	rsi, rax
140b3b9d7: e8 14 82 de 01              	call	0x142923bf0 <.text+0x2922bf0>
140b3b9dc: 89 c1                       	mov	ecx, eax
140b3b9de: 48 89 f0                    	mov	rax, rsi
140b3b9e1: 48 8d 56 01                 	lea	rdx, [rsi + 0x1]
140b3b9e5: 85 c9                       	test	ecx, ecx
140b3b9e7: 0f 85 bc 00 00 00           	jne	0x140b3baa9 <.text+0xb3aaa9>
140b3b9ed: c6 02 07                    	mov	byte ptr [rdx], 0x7
140b3b9f0: e9 b7 00 00 00              	jmp	0x140b3baac <.text+0xb3aaac>
140b3b9f5: 48 b9 72 6f 74 61 74 69 6f 6e       	movabs	rcx, 0x6e6f697461746f72
140b3b9ff: 48 33 0a                    	xor	rcx, qword ptr [rdx]
140b3ba02: 49 b8 6f 6e 5f 61 6e 67 6c 65       	movabs	r8, 0x656c676e615f6e6f
140b3ba0c: 4c 33 42 06                 	xor	r8, qword ptr [rdx + 0x6]
140b3ba10: 49 09 c8                    	or	r8, rcx
140b3ba13: 0f 85 8c 00 00 00           	jne	0x140b3baa5 <.text+0xb3aaa5>
140b3ba19: eb 18                       	jmp	0x140b3ba33 <.text+0xb3aa33>
140b3ba1b: f3 0f 6f 02                 	movdqu	xmm0, xmmword ptr [rdx]
140b3ba1f: 66 0f 74 05 f9 1f fb 01     	pcmpeqb	xmm0, xmmword ptr [rip + 0x1fb1ff9] # 0x142aeda20
140b3ba27: 66 0f d7 c8                 	pmovmskb	ecx, xmm0
140b3ba2b: 81 f9 ff ff 00 00           	cmp	ecx, 0xffff
140b3ba31: 75 72                       	jne	0x140b3baa5 <.text+0xb3aaa5>
140b3ba33: c6 40 01 02                 	mov	byte ptr [rax + 0x1], 0x2
140b3ba37: eb 73                       	jmp	0x140b3baac <.text+0xb3aaac>
140b3ba39: b9 73 63 61 6c              	mov	ecx, 0x6c616373
140b3ba3e: 33 0a                       	xor	ecx, dword ptr [rdx]
140b3ba40: 0f b6 52 04                 	movzx	edx, byte ptr [rdx + 0x4]
140b3ba44: 83 f2 65                    	xor	edx, 0x65
140b3ba47: 09 ca                       	or	edx, ecx
140b3ba49: 75 5a                       	jne	0x140b3baa5 <.text+0xb3aaa5>
140b3ba4b: c6 40 01 01                 	mov	byte ptr [rax + 0x1], 0x1
140b3ba4f: eb 5b                       	jmp	0x140b3baac <.text+0xb3aaac>
140b3ba51: 48 89 c3                    	mov	rbx, rax
140b3ba54: 48 8d 05 f5 27 fb 01        	lea	rax, [rip + 0x1fb27f5]  # 0x142aee250
140b3ba5b: 48 89 d6                    	mov	rsi, rdx
140b3ba5e: 48 89 d1                    	mov	rcx, rdx
140b3ba61: 48 89 c2                    	mov	rdx, rax
140b3ba64: 4c 89 c7                    	mov	rdi, r8
140b3ba67: e8 84 81 de 01              	call	0x142923bf0 <.text+0x2922bf0>
140b3ba6c: 85 c0                       	test	eax, eax
140b3ba6e: 74 47                       	je	0x140b3bab7 <.text+0xb3aab7>
140b3ba70: 48 8d 15 e0 27 fb 01        	lea	rdx, [rip + 0x1fb27e0]  # 0x142aee257
140b3ba77: 48 89 f1                    	mov	rcx, rsi
140b3ba7a: 49 89 f8                    	mov	r8, rdi
140b3ba7d: e8 6e 81 de 01              	call	0x142923bf0 <.text+0x2922bf0>
140b3ba82: 85 c0                       	test	eax, eax
140b3ba84: 48 89 d8                    	mov	rax, rbx
140b3ba87: 75 1c                       	jne	0x140b3baa5 <.text+0xb3aaa5>
140b3ba89: c6 40 01 05                 	mov	byte ptr [rax + 0x1], 0x5
140b3ba8d: eb 1d                       	jmp	0x140b3baac <.text+0xb3aaac>
140b3ba8f: 48 b9 72 6f 74 61 74 69 6f 6e       	movabs	rcx, 0x6e6f697461746f72
140b3ba99: 48 81 c1 fe ff fe 07        	add	rcx, 0x7fefffe
140b3baa0: 48 39 0a                    	cmp	qword ptr [rdx], rcx
140b3baa3: 74 1b                       	je	0x140b3bac0 <.text+0xb3aac0>
140b3baa5: 48 8d 50 01                 	lea	rdx, [rax + 0x1]
140b3baa9: c6 02 08                    	mov	byte ptr [rdx], 0x8
140b3baac: c6 00 00                    	mov	byte ptr [rax], 0x0
140b3baaf: 48 83 c4 20                 	add	rsp, 0x20
140b3bab3: 5b                          	pop	rbx
140b3bab4: 5f                          	pop	rdi
140b3bab5: 5e                          	pop	rsi
140b3bab6: c3                          	ret
140b3bab7: 48 89 d8                    	mov	rax, rbx
140b3baba: c6 43 01 04                 	mov	byte ptr [rbx + 0x1], 0x4
140b3babe: eb ec                       	jmp	0x140b3baac <.text+0xb3aaac>
140b3bac0: c6 40 01 00                 	mov	byte ptr [rax + 0x1], 0x0
140b3bac4: eb e6                       	jmp	0x140b3baac <.text+0xb3aaac>
