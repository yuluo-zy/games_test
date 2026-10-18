
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a28c60: 55                          	push	rbp
140a28c61: 41 57                       	push	r15
140a28c63: 41 56                       	push	r14
140a28c65: 41 55                       	push	r13
140a28c67: 41 54                       	push	r12
140a28c69: 56                          	push	rsi
140a28c6a: 57                          	push	rdi
140a28c6b: 53                          	push	rbx
140a28c6c: 48 81 ec 88 06 00 00        	sub	rsp, 0x688
140a28c73: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
140a28c7b: 48 c7 85 00 06 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0x600], -0x2
140a28c86: 89 d0                       	mov	eax, edx
140a28c88: 48 89 ce                    	mov	rsi, rcx
140a28c8b: 48 8b 9d 88 06 00 00        	mov	rbx, qword ptr [rbp + 0x688]
140a28c92: 4c 8b b5 80 06 00 00        	mov	r14, qword ptr [rbp + 0x680]
140a28c99: 48 8b bd 78 06 00 00        	mov	rdi, qword ptr [rbp + 0x678]
140a28ca0: 4c 8b bd 70 06 00 00        	mov	r15, qword ptr [rbp + 0x670]
140a28ca7: 4c 89 4c 24 28              	mov	qword ptr [rsp + 0x28], r9
140a28cac: 4c 89 44 24 20              	mov	qword ptr [rsp + 0x20], r8
140a28cb1: 48 8d 15 c8 1c 0a 02        	lea	rdx, [rip + 0x20a1cc8]  # 0x142aca980
140a28cb8: 4c 8d a5 d8 02 00 00        	lea	r12, [rbp + 0x2d8]
140a28cbf: 41 b8 12 00 00 00           	mov	r8d, 0x12
140a28cc5: 4c 89 e1                    	mov	rcx, r12
140a28cc8: 41 89 c1                    	mov	r9d, eax
140a28ccb: e8 a0 8a bd 00              	call	0x141601770 <.text+0x1600770>
140a28cd0: 4c 8d ad 68 04 00 00        	lea	r13, [rbp + 0x468]
140a28cd7: 49 b8 4d f3 05 c9 b1 38 6c 68       	movabs	r8, 0x686c38b1c905f34d
140a28ce1: 4c 89 e9                    	mov	rcx, r13
140a28ce4: 4c 89 e2                    	mov	rdx, r12
140a28ce7: 41 b9 01 00 00 00           	mov	r9d, 0x1
140a28ced: e8 2e fd bd 00              	call	0x141608a20 <.text+0x1607a20>
140a28cf2: 49 b8 9b ec 49 c2 df d9 05 d0       	movabs	r8, -0x2ffa26203db61365
140a28cfc: 4c 89 e1                    	mov	rcx, r12
140a28cff: 4c 89 ea                    	mov	rdx, r13
140a28d02: 41 b9 01 00 00 00           	mov	r9d, 0x1
140a28d08: e8 53 f5 bd 00              	call	0x141608260 <.text+0x1607260>
140a28d0d: c7 44 24 20 01 00 00 00     	mov	dword ptr [rsp + 0x20], 0x1
140a28d15: 49 b8 3b 42 1d a4 2e da fa 77       	movabs	r8, 0x77fada2ea41d423b
140a28d1f: 4c 89 e9                    	mov	rcx, r13
140a28d22: 4c 89 e2                    	mov	rdx, r12
140a28d25: 4d 89 f9                    	mov	r9, r15
140a28d28: e8 d3 52 f4 ff              	call	0x14096e000 <.text+0x96d000>
140a28d2d: c7 44 24 20 01 00 00 00     	mov	dword ptr [rsp + 0x20], 0x1
140a28d35: 4c 8d bd d8 02 00 00        	lea	r15, [rbp + 0x2d8]
140a28d3c: 49 b8 75 a5 7b 13 be 93 b5 7a       	movabs	r8, 0x7ab593be137ba575
140a28d46: 4c 89 f9                    	mov	rcx, r15
140a28d49: 4c 89 ea                    	mov	rdx, r13
140a28d4c: 49 89 f9                    	mov	r9, rdi
140a28d4f: e8 ac 52 f4 ff              	call	0x14096e000 <.text+0x96d000>
140a28d54: 48 8d bd 68 04 00 00        	lea	rdi, [rbp + 0x468]
140a28d5b: 49 b8 bd cc bd d5 b2 ae 75 2a       	movabs	r8, 0x2a75aeb2d5bdccbd
140a28d65: 48 89 f9                    	mov	rcx, rdi
140a28d68: 4c 89 fa                    	mov	rdx, r15
140a28d6b: 4d 89 f1                    	mov	r9, r14
140a28d6e: e8 2d 55 f4 ff              	call	0x14096e2a0 <.text+0x96d2a0>
140a28d73: 48 8d 4d b8                 	lea	rcx, [rbp - 0x48]
140a28d77: 49 b8 f1 97 e4 bf 72 72 8a 66       	movabs	r8, 0x668a7272bfe497f1
140a28d81: 48 89 fa                    	mov	rdx, rdi
140a28d84: 49 89 d9                    	mov	r9, rbx
140a28d87: e8 14 55 f4 ff              	call	0x14096e2a0 <.text+0x96d2a0>
140a28d8c: 48 8d 9d d8 02 00 00        	lea	rbx, [rbp + 0x2d8]
140a28d93: 48 8d 55 b8                 	lea	rdx, [rbp - 0x48]
140a28d97: 49 b8 ed b8 c7 43 bc 36 ab 35       	movabs	r8, 0x35ab36bc43c7b8ed
140a28da1: 48 89 d9                    	mov	rcx, rbx
140a28da4: 83 bd 90 06 00 00 00        	cmp	dword ptr [rbp + 0x690], 0x0
140a28dab: 74 35                       	je	0x140a28de2 <.text+0xa27de2>
140a28dad: e8 be 3f be 00              	call	0x14160cd70 <.text+0x160bd70>
140a28db2: 48 8d 8d 48 01 00 00        	lea	rcx, [rbp + 0x148]
140a28db9: 49 b8 c5 67 7f 3b 90 a9 83 37       	movabs	r8, 0x3783a9903b7f67c5
140a28dc3: 48 89 da                    	mov	rdx, rbx
140a28dc6: 41 b9 01 00 00 00           	mov	r9d, 0x1
140a28dcc: e8 0f 04 be 00              	call	0x1416091e0 <.text+0x16081e0>
140a28dd1: 4c 8b b5 98 06 00 00        	mov	r14, qword ptr [rbp + 0x698]
140a28dd8: 4d 85 f6                    	test	r14, r14
140a28ddb: 75 39                       	jne	0x140a28e16 <.text+0xa27e16>
140a28ddd: e9 df 00 00 00              	jmp	0x140a28ec1 <.text+0xa27ec1>
140a28de2: e8 a9 51 be 00              	call	0x14160df90 <.text+0x160cf90>
140a28de7: 48 8d 8d 48 01 00 00        	lea	rcx, [rbp + 0x148]
140a28dee: 49 b8 c5 67 7f 3b 90 a9 83 37       	movabs	r8, 0x3783a9903b7f67c5
140a28df8: 48 89 da                    	mov	rdx, rbx
140a28dfb: 41 b9 01 00 00 00           	mov	r9d, 0x1
140a28e01: e8 fa ff bd 00              	call	0x141608e00 <.text+0x1607e00>
140a28e06: 4c 8b b5 98 06 00 00        	mov	r14, qword ptr [rbp + 0x698]
140a28e0d: 4d 85 f6                    	test	r14, r14
140a28e10: 0f 84 ab 00 00 00           	je	0x140a28ec1 <.text+0xa27ec1>
140a28e16: 48 8d 95 48 01 00 00        	lea	rdx, [rbp + 0x148]
140a28e1d: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a28e23: 48 89 d9                    	mov	rcx, rbx
140a28e26: e8 d5 ad ef 01              	call	0x142923c00 <.text+0x2922c00>
140a28e2b: 49 8d 46 08                 	lea	rax, [r14 + 0x8]
140a28e2f: 48 89 85 f8 05 00 00        	mov	qword ptr [rbp + 0x5f8], rax
140a28e36: c7 85 68 04 00 00 02 00 00 00       	mov	dword ptr [rbp + 0x468], 0x2
140a28e40: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
140a28e45: 48 8d 8d f8 05 00 00        	lea	rcx, [rbp + 0x5f8]
140a28e4c: 49 b8 69 a5 a8 89 2f 81 b6 62       	movabs	r8, 0x62b6812f89a8a569
140a28e56: 48 89 da                    	mov	rdx, rbx
140a28e59: 41 b1 19                    	mov	r9b, 0x19
140a28e5c: e8 df 53 ec ff              	call	0x1408ee240 <.text+0x8ed240>
140a28e61: 48 8d 8d 68 04 00 00        	lea	rcx, [rbp + 0x468]
140a28e68: 48 8d 95 d8 02 00 00        	lea	rdx, [rbp + 0x2d8]
140a28e6f: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a28e75: e8 86 ad ef 01              	call	0x142923c00 <.text+0x2922c00>
140a28e7a: 80 bd 9e 05 00 00 08        	cmp	byte ptr [rbp + 0x59e], 0x8
140a28e81: 74 25                       	je	0x140a28ea8 <.text+0xa27ea8>
140a28e83: 4d 8b 0e                    	mov	r9, qword ptr [r14]
140a28e86: 48 8d 8d 78 05 00 00        	lea	rcx, [rbp + 0x578]
140a28e8d: ba e8 00 00 00              	mov	edx, 0xe8
140a28e92: 48 03 95 a0 05 00 00        	add	rdx, qword ptr [rbp + 0x5a0]
140a28e99: 49 b8 d3 41 03 96 13 36 d2 e8       	movabs	r8, -0x172dc9ec69fcbe2d
140a28ea3: e8 b8 28 f4 ff              	call	0x14096b760 <.text+0x96a760>
140a28ea8: 48 8d 8d 48 01 00 00        	lea	rcx, [rbp + 0x148]
140a28eaf: 48 8d 95 68 04 00 00        	lea	rdx, [rbp + 0x468]
140a28eb6: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a28ebc: e8 3f ad ef 01              	call	0x142923c00 <.text+0x2922c00>
140a28ec1: 48 8d 95 48 01 00 00        	lea	rdx, [rbp + 0x148]
140a28ec8: 41 b8 90 01 00 00           	mov	r8d, 0x190
140a28ece: 48 89 f1                    	mov	rcx, rsi
140a28ed1: e8 2a ad ef 01              	call	0x142923c00 <.text+0x2922c00>
140a28ed6: 48 89 f0                    	mov	rax, rsi
140a28ed9: 48 81 c4 88 06 00 00        	add	rsp, 0x688
140a28ee0: 5b                          	pop	rbx
140a28ee1: 5f                          	pop	rdi
140a28ee2: 5e                          	pop	rsi
140a28ee3: 41 5c                       	pop	r12
140a28ee5: 41 5d                       	pop	r13
140a28ee7: 41 5e                       	pop	r14
140a28ee9: 41 5f                       	pop	r15
140a28eeb: 5d                          	pop	rbp
140a28eec: c3                          	ret
140a28eed: 0f 1f 00                    	nop	dword ptr [rax]
140a28ef0: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a28ef5: 55                          	push	rbp
140a28ef6: 41 57                       	push	r15
140a28ef8: 41 56                       	push	r14
140a28efa: 41 55                       	push	r13
140a28efc: 41 54                       	push	r12
140a28efe: 56                          	push	rsi
140a28eff: 57                          	push	rdi
140a28f00: 53                          	push	rbx
140a28f01: 48 83 ec 38                 	sub	rsp, 0x38
140a28f05: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a28f0c: 48 8d 8d d8 02 00 00        	lea	rcx, [rbp + 0x2d8]
140a28f13: e8 98 e0 e6 ff              	call	0x140896fb0 <.text+0x895fb0>
140a28f18: 90                          	nop
140a28f19: 48 83 c4 38                 	add	rsp, 0x38
140a28f1d: 5b                          	pop	rbx
140a28f1e: 5f                          	pop	rdi
140a28f1f: 5e                          	pop	rsi
140a28f20: 41 5c                       	pop	r12
140a28f22: 41 5d                       	pop	r13
140a28f24: 41 5e                       	pop	r14
140a28f26: 41 5f                       	pop	r15
140a28f28: 5d                          	pop	rbp
140a28f29: c3                          	ret
140a28f2a: 66 0f 1f 44 00 00           	nop	word ptr [rax + rax]
140a28f30: 48 89 54 24 10              	mov	qword ptr [rsp + 0x10], rdx
140a28f35: 55                          	push	rbp
140a28f36: 41 57                       	push	r15
140a28f38: 41 56                       	push	r14
140a28f3a: 41 55                       	push	r13
140a28f3c: 41 54                       	push	r12
140a28f3e: 56                          	push	rsi
140a28f3f: 57                          	push	rdi
140a28f40: 53                          	push	rbx
140a28f41: 48 83 ec 38                 	sub	rsp, 0x38
140a28f45: 48 8d aa 80 00 00 00        	lea	rbp, [rdx + 0x80]
140a28f4c: 48 8d 8d 68 04 00 00        	lea	rcx, [rbp + 0x468]
140a28f53: e8 58 e0 e6 ff              	call	0x140896fb0 <.text+0x895fb0>
140a28f58: 90                          	nop
140a28f59: 48 83 c4 38                 	add	rsp, 0x38
140a28f5d: 5b                          	pop	rbx
140a28f5e: 5f                          	pop	rdi
140a28f5f: 5e                          	pop	rsi
140a28f60: 41 5c                       	pop	r12
140a28f62: 41 5d                       	pop	r13
140a28f64: 41 5e                       	pop	r14
140a28f66: 41 5f                       	pop	r15
140a28f68: 5d                          	pop	rbp
140a28f69: c3                          	ret
