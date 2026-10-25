
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a28950: 55                          	push	rbp
140a28951: 41 57                       	push	r15
140a28953: 41 56                       	push	r14
140a28955: 41 55                       	push	r13
140a28957: 41 54                       	push	r12
140a28959: 56                          	push	rsi
140a2895a: 57                          	push	rdi
140a2895b: 53                          	push	rbx
140a2895c: 48 81 ec 88 06 00 00        	sub	rsp, 0x688
140a28963: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
140a2896b: 48 c7 85 00 06 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x600], -0x2
140a28976: 89 d0                       	mov	eax, edx
140a28978: 48 89 ce                    	mov	rsi, rcx
140a2897b: 48 8b 9d 88 06 00 00        	mov	rbx, qword ptr [rbp + 0x688]
140a28982: 4c 8b b5 80 06 00 00        	mov	r14, qword ptr [rbp + 0x680]
140a28989: 48 8b bd 78 06 00 00        	mov	rdi, qword ptr [rbp + 0x678]
140a28990: 4c 8b bd 70 06 00 00        	mov	r15, qword ptr [rbp + 0x670]
140a28997: 4c 89 4c 24 28              	mov	qword ptr [rsp + 0x28], r9
140a2899c: 4c 89 44 24 20              	mov	qword ptr [rsp + 0x20], r8
140a289a1: 48 8d 15 d8 1f 0a 02        	lea	rdx, [rip + 0x20a1fd8]  # 0x142aca980
140a289a8: 4c 8d a5 d8 02 00 00        	lea	r12, [rbp + 0x2d8]
140a289af: 41 b8 12 00 00 00           	mov	r8d, 0x12
140a289b5: 4c 89 e1                    	mov	rcx, r12
140a289b8: 41 89 c1                    	mov	r9d, eax
140a289bb: e8 90 69 bd 00              	call	0x1415ff350 <.text+0x15fe350>
140a289c0: 4c 8d ad 68 04 00 00        	lea	r13, [rbp + 0x468]
140a289c7: 49 b8 4d f3 05 c9 b1 38 6c 68       	movabs	r8, 0x686c38b1c905f34d
140a289d1: 4c 89 e9                    	mov	rcx, r13
140a289d4: 4c 89 e2                    	mov	rdx, r12
140a289d7: 41 b9 01 00 00 00           	mov	r9d, 0x1
140a289dd: e8 3e 00 be 00              	call	0x141608a20 <.text+0x1607a20>
140a289e2: 49 b8 9b ec 49 c2 df d9 05 d0       	movabs	r8, -0x2ffa26203db61365
140a289ec: 4c 89 e1                    	mov	rcx, r12
140a289ef: 4c 89 ea                    	mov	rdx, r13
140a289f2: 41 b9 01 00 00 00           	mov	r9d, 0x1
140a289f8: e8 63 f8 bd 00              	call	0x141608260 <.text+0x1607260>
140a289fd: c7 44 24 20 01 00 00 00     	mov	dword ptr [rsp + 0x20], 0x1
140a28a05: 49 b8 3b 42 1d a4 2e da fa 77       	movabs	r8, 0x77fada2ea41d423b
140a28a0f: 4c 89 e9                    	mov	rcx, r13
140a28a12: 4c 89 e2                    	mov	rdx, r12
140a28a15: 4d 89 f9                    	mov	r9, r15
140a28a18: e8 e3 55 f4 ff              	call	0x14096e000 <.text+0x96d000>
140a28a1d: c7 44 24 20 01 00 00 00     	mov	dword ptr [rsp + 0x20], 0x1
140a28a25: 4c 8d bd d8 02 00 00        	lea	r15, [rbp + 0x2d8]
140a28a2c: 49 b8 75 a5 7b 13 be 93 b5 7a       	movabs	r8, 0x7ab593be137ba575
140a28a36: 4c 89 f9                    	mov	rcx, r15
140a28a39: 4c 89 ea                    	mov	rdx, r13
140a28a3c: 49 89 f9                    	mov	r9, rdi
140a28a3f: e8 bc 55 f4 ff              	call	0x14096e000 <.text+0x96d000>
140a28a44: 48 8d bd 68 04 00 00        	lea	rdi, [rbp + 0x468]
140a28a4b: 49 b8 bd cc bd d5 b2 ae 75 2a       	movabs	r8, 0x2a75aeb2d5bdccbd
140a28a55: 48 89 f9                    	mov	rcx, rdi
140a28a58: 4c 89 fa                    	mov	rdx, r15
140a28a5b: 4d 89 f1                    	mov	r9, r14
140a28a5e: e8 3d 58 f4 ff              	call	0x14096e2a0 <.text+0x96d2a0>
140a28a63: 48 8d 4d b8                 	lea	rcx, [rbp - 0x48]
140a28a67: 49 b8 f1 97 e4 bf 72 72 8a 66       	movabs	r8, 0x668a7272bfe497f1
140a28a71: 48 89 fa                    	mov	rdx, rdi
140a28a74: 49 89 d9                    	mov	r9, rbx
140a28a77: e8 24 58 f4 ff              	call	0x14096e2a0 <.text+0x96d2a0>
140a28a7c: 48 8d 9d d8 02 00 00        	lea	rbx, [rbp + 0x2d8]
140a28a83: 48 8d 55 b8                 	lea	rdx, [rbp - 0x48]
140a28a87: 49 b8 ed b8 c7 43 bc 36 ab 35       	movabs	r8, 0x35ab36bc43c7b8ed
140a28a91: 48 89 d9                    	mov	rcx, rbx
140a28a94: 83 bd 90 06 00 00 00        	cmp	dword ptr [rbp + 0x690], 0x0
140a28a9b: 74 35                       	je	0x140a28ad2 <.text+0xa27ad2>
140a28a9d: e8 ce 42 be 00              	call	0x14160cd70 <.text+0x160bd70>
140a28aa2: 48 8d 8d 48 01 00 00        	lea	rcx, [rbp + 0x148]
140a28aa9: 49 b8 c5 67 7f 3b 90 a9 83 37       	movabs	r8, 0x3783a9903b7f67c5
140a28ab3: 48 89 da                    	mov	rdx, rbx
140a28ab6: 41 b9 01 00 00 00           	mov	r9d, 0x1
140a28abc: e8 1f 07 be 00              	call	0x1416091e0 <.text+0x16081e0>
140a28ac1: 4c 8b b5 98 06 00 00        	mov	r14, qword ptr [rbp + 0x698]
140a28ac8: 4d 85 f6                    	test	r14, r14
140a28acb: 75 39                       	jne	0x140a28b06 <.text+0xa27b06>
140a28acd: e9 df 00 00 00              	jmp	0x140a28bb1 <.text+0xa27bb1>
140a28ad2: e8 b9 54 be 00              	call	0x14160df90 <.text+0x160cf90>
140a28ad7: 48 8d 8d 48 01 00 00        	lea	rcx, [rbp + 0x148]
140a28ade: 49 b8 c5 67 7f 3b 90 a9 83 37       	movabs	r8, 0x3783a9903b7f67c5
140a28ae8: 48 89 da                    	mov	rdx, rbx
140a28aeb: 41 b9 01 00 00 00           	mov	r9d, 0x1
140a28af1: e8 0a 03 be 00              	call	0x141608e00 <.text+0x1607e00>
140a28af6: 4c 8b b5 98 06 00 00        	mov	r14, qword ptr [rbp + 0x698]
140a28afd: 4d 85 f6                    	test	r14, r14
140a28b00: 0f 84 ab 00 00 00           	je	0x140a28bb1 <.text+0xa27bb1>
140a28b06: 48 8d 95 48 01 00 00        	lea	rdx, [rbp + 0x148]
140a28b0d: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a28b13: 48 89 d9                    	mov	rcx, rbx
140a28b16: e8 e5 b0 ef 01              	call	0x142923c00 <.text+0x2922c00>
140a28b1b: 49 8d 46 08                 	lea	rax, [r14 + 0x8]
140a28b1f: 48 89 85 f8 05 00 00        	mov	qword ptr [rbp + 0x5f8], rax
140a28b26: c7 85 68 04 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x468], 0x2
140a28b30: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
140a28b35: 48 8d 8d f8 05 00 00        	lea	rcx, [rbp + 0x5f8]
140a28b3c: 49 b8 69 a5 a8 89 2f 81 b6 62       	movabs	r8, 0x62b6812f89a8a569
140a28b46: 48 89 da                    	mov	rdx, rbx
140a28b49: 41 b1 19                    	mov	r9b, 0x19
140a28b4c: e8 ef 56 ec ff              	call	0x1408ee240 <.text+0x8ed240>
140a28b51: 48 8d 8d 68 04 00 00        	lea	rcx, [rbp + 0x468]
140a28b58: 48 8d 95 d8 02 00 00        	lea	rdx, [rbp + 0x2d8]
140a28b5f: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a28b65: e8 96 b0 ef 01              	call	0x142923c00 <.text+0x2922c00>
140a28b6a: 80 bd 9e 05 00 00 08        	cmp	byte ptr [rbp + 0x59e], 0x8
140a28b71: 74 25                       	je	0x140a28b98 <.text+0xa27b98>
140a28b73: 4d 8b 0e                    	mov	r9, qword ptr [r14]
140a28b76: 48 8d 8d 78 05 00 00        	lea	rcx, [rbp + 0x578]
140a28b7d: ba e8 00 00 00              	mov	edx, 0xe8
140a28b82: 48 03 95 a0 05 00 00        	add	rdx, qword ptr [rbp + 0x5a0]
140a28b89: 49 b8 d3 41 03 96 13 36 d2 e8       	movabs	r8, -0x172dc9ec69fcbe2d
140a28b93: e8 c8 2b f4 ff              	call	0x14096b760 <.text+0x96a760>
140a28b98: 48 8d 8d 48 01 00 00        	lea	rcx, [rbp + 0x148]
140a28b9f: 48 8d 95 68 04 00 00        	lea	rdx, [rbp + 0x468]
140a28ba6: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a28bac: e8 4f b0 ef 01              	call	0x142923c00 <.text+0x2922c00>
140a28bb1: 48 8d 95 48 01 00 00        	lea	rdx, [rbp + 0x148]
140a28bb8: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a28bbe: 48 89 f1                    	mov	rcx, rsi
140a28bc1: e8 3a b0 ef 01              	call	0x142923c00 <.text+0x2922c00>
140a28bc6: 48 89 f0                    	mov	rax, rsi
140a28bc9: 48 81 c4 88 06 00 00        	add	rsp, 0x688
140a28bd0: 5b                          	pop	rbx
140a28bd1: 5f                          	pop	rdi
140a28bd2: 5e                          	pop	rsi
140a28bd3: 41 5c                       	pop	r12
140a28bd5: 41 5d                       	pop	r13
140a28bd7: 41 5e                       	pop	r14
140a28bd9: 41 5f                       	pop	r15
140a28bdb: 5d                          	pop	rbp
140a28bdc: c3                          	ret
140a28bdd: 0f 1f 00                    	nop	dword ptr [rax]
140a28be0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a28be5: 55                          	push	rbp
140a28be6: 41 57                       	push	r15
140a28be8: 41 56                       	push	r14
140a28bea: 41 55                       	push	r13
140a28bec: 41 54                       	push	r12
140a28bee: 56                          	push	rsi
140a28bef: 57                          	push	rdi
140a28bf0: 53                          	push	rbx
140a28bf1: 48 83 ec 38                 	sub	rsp, 0x38
140a28bf5: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a28bfc: 48 8d 8d d8 02 00 00        	lea	rcx, [rbp + 0x2d8]
140a28c03: e8 a8 e3 e6 ff              	call	0x140896fb0 <.text+0x895fb0>
140a28c08: 90                          	nop
140a28c09: 48 83 c4 38                 	add	rsp, 0x38
140a28c0d: 5b                          	pop	rbx
140a28c0e: 5f                          	pop	rdi
140a28c0f: 5e                          	pop	rsi
140a28c10: 41 5c                       	pop	r12
140a28c12: 41 5d                       	pop	r13
140a28c14: 41 5e                       	pop	r14
140a28c16: 41 5f                       	pop	r15
140a28c18: 5d                          	pop	rbp
140a28c19: c3                          	ret
140a28c1a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
140a28c20: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a28c25: 55                          	push	rbp
140a28c26: 41 57                       	push	r15
140a28c28: 41 56                       	push	r14
140a28c2a: 41 55                       	push	r13
140a28c2c: 41 54                       	push	r12
140a28c2e: 56                          	push	rsi
140a28c2f: 57                          	push	rdi
140a28c30: 53                          	push	rbx
140a28c31: 48 83 ec 38                 	sub	rsp, 0x38
140a28c35: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a28c3c: 48 8d 8d 68 04 00 00        	lea	rcx, [rbp + 0x468]
140a28c43: e8 68 e3 e6 ff              	call	0x140896fb0 <.text+0x895fb0>
140a28c48: 90                          	nop
140a28c49: 48 83 c4 38                 	add	rsp, 0x38
140a28c4d: 5b                          	pop	rbx
140a28c4e: 5f                          	pop	rdi
140a28c4f: 5e                          	pop	rsi
140a28c50: 41 5c                       	pop	r12
140a28c52: 41 5d                       	pop	r13
140a28c54: 41 5e                       	pop	r14
140a28c56: 41 5f                       	pop	r15
140a28c58: 5d                          	pop	rbp
140a28c59: c3                          	ret
