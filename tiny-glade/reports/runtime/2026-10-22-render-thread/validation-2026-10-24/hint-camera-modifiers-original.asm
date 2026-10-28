
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14227c910: 55                          	push	rbp
14227c911: 41 57                       	push	r15
14227c913: 41 56                       	push	r14
14227c915: 41 55                       	push	r13
14227c917: 41 54                       	push	r12
14227c919: 56                          	push	rsi
14227c91a: 57                          	push	rdi
14227c91b: 53                          	push	rbx
14227c91c: 48 81 ec b8 01 00 00        	sub	rsp, 0x1b8
14227c923: 48 8d ac 24 80 00 00 00     	lea	rbp, [rsp + 0x80]
14227c92b: 44 0f 29 8d 20 01 00 00     	movaps	xmmword ptr [rbp + 0x120], xmm9
14227c933: 44 0f 29 85 10 01 00 00     	movaps	xmmword ptr [rbp + 0x110], xmm8
14227c93b: 0f 29 bd 00 01 00 00        	movaps	xmmword ptr [rbp + 0x100], xmm7
14227c942: 0f 29 b5 f0 00 00 00        	movaps	xmmword ptr [rbp + 0xf0], xmm6
14227c949: 48 c7 85 e8 00 00 00 fe ff ff ff    	mov	qword ptr [rbp + 0xe8], -0x2
14227c954: 49 8b 01                    	mov	rax, qword ptr [r9]
14227c957: 83 38 01                    	cmp	dword ptr [rax], 0x1
14227c95a: 0f 85 b7 00 00 00           	jne	0x14227ca17 <.text+0x227ba17>
14227c960: 4c 89 c7                    	mov	rdi, r8
14227c963: 48 8b 12                    	mov	rdx, qword ptr [rdx]
14227c966: 48 89 95 d0 00 00 00        	mov	qword ptr [rbp + 0xd0], rdx
14227c96d: 80 ba f6 01 00 00 01        	cmp	byte ptr [rdx + 0x1f6], 0x1
14227c974: 75 31                       	jne	0x14227c9a7 <.text+0x227b9a7>
14227c976: 48 8b 95 d0 00 00 00        	mov	rdx, qword ptr [rbp + 0xd0]
14227c97d: 80 ba f7 01 00 00 00        	cmp	byte ptr [rdx + 0x1f7], 0x0
14227c984: 74 21                       	je	0x14227c9a7 <.text+0x227b9a7>
14227c986: 48 8b 95 d0 00 00 00        	mov	rdx, qword ptr [rbp + 0xd0]
14227c98d: 80 ba f8 01 00 00 01        	cmp	byte ptr [rdx + 0x1f8], 0x1
14227c994: 75 11                       	jne	0x14227c9a7 <.text+0x227b9a7>
14227c996: 41 8b 51 1c                 	mov	edx, dword ptr [r9 + 0x1c]
14227c99a: 4d 8b 41 10                 	mov	r8, qword ptr [r9 + 0x10]
14227c99e: 41 89 10                    	mov	dword ptr [r8], edx
14227c9a1: c7 00 02 00 00 00           	mov	dword ptr [rax], 0x2
14227c9a7: 48 8b 85 a8 01 00 00        	mov	rax, qword ptr [rbp + 0x1a8]
14227c9ae: 48 8b 31                    	mov	rsi, qword ptr [rcx]
14227c9b1: 8b 0e                       	mov	ecx, dword ptr [rsi]
14227c9b3: 83 f9 06                    	cmp	ecx, 0x6
14227c9b6: 41 0f 94 c0                 	sete	r8b
14227c9ba: 0f b6 56 04                 	movzx	edx, byte ptr [rsi + 0x4]
14227c9be: 80 fa 01                    	cmp	dl, 0x1
14227c9c1: 41 0f 94 c1                 	sete	r9b
14227c9c5: b3 01                       	mov	bl, 0x1
14227c9c7: 45 84 c8                    	test	r8b, r9b
14227c9ca: 75 33                       	jne	0x14227c9ff <.text+0x227b9ff>
14227c9cc: 83 f9 06                    	cmp	ecx, 0x6
14227c9cf: 75 08                       	jne	0x14227c9d9 <.text+0x227b9d9>
14227c9d1: 80 fa 02                    	cmp	dl, 0x2
14227c9d4: 0f 94 c3                    	sete	bl
14227c9d7: eb 26                       	jmp	0x14227c9ff <.text+0x227b9ff>
14227c9d9: 41 b6 01                    	mov	r14b, 0x1
14227c9dc: 31 db                       	xor	ebx, ebx
14227c9de: 83 f9 02                    	cmp	ecx, 0x2
14227c9e1: 75 1c                       	jne	0x14227c9ff <.text+0x227b9ff>
14227c9e3: 48 8b 08                    	mov	rcx, qword ptr [rax]
14227c9e6: e8 95 8c 64 fe              	call	0x1408c5680 <.text+0x8c4680>
14227c9eb: 84 c0                       	test	al, al
14227c9ed: 74 23                       	je	0x14227ca12 <.text+0x227ba12>
14227c9ef: 83 3e 0a                    	cmp	dword ptr [rsi], 0xa
14227c9f2: 0f 94 c0                    	sete	al
14227c9f5: 41 08 c6                    	or	r14b, al
14227c9f8: 44 08 f3                    	or	bl, r14b
14227c9fb: 74 4c                       	je	0x14227ca49 <.text+0x227ba49>
14227c9fd: eb 18                       	jmp	0x14227ca17 <.text+0x227ba17>
14227c9ff: 83 f9 01                    	cmp	ecx, 0x1
14227ca02: 41 0f 94 c6                 	sete	r14b
14227ca06: 48 8b 08                    	mov	rcx, qword ptr [rax]
14227ca09: e8 72 8c 64 fe              	call	0x1408c5680 <.text+0x8c4680>
14227ca0e: 84 c0                       	test	al, al
14227ca10: 75 dd                       	jne	0x14227c9ef <.text+0x227b9ef>
14227ca12: 44 08 f3                    	or	bl, r14b
14227ca15: 74 32                       	je	0x14227ca49 <.text+0x227ba49>
14227ca17: 0f 28 b5 f0 00 00 00        	movaps	xmm6, xmmword ptr [rbp + 0xf0]
14227ca1e: 0f 28 bd 00 01 00 00        	movaps	xmm7, xmmword ptr [rbp + 0x100]
14227ca25: 44 0f 28 85 10 01 00 00     	movaps	xmm8, xmmword ptr [rbp + 0x110]
14227ca2d: 44 0f 28 8d 20 01 00 00     	movaps	xmm9, xmmword ptr [rbp + 0x120]
14227ca35: 48 81 c4 b8 01 00 00        	add	rsp, 0x1b8
14227ca3c: 5b                          	pop	rbx
14227ca3d: 5f                          	pop	rdi
14227ca3e: 5e                          	pop	rsi
14227ca3f: 41 5c                       	pop	r12
14227ca41: 41 5d                       	pop	r13
14227ca43: 41 5e                       	pop	r14
14227ca45: 41 5f                       	pop	r15
14227ca47: 5d                          	pop	rbp
14227ca48: c3                          	ret
14227ca49: 48 8b 85 c8 01 00 00        	mov	rax, qword ptr [rbp + 0x1c8]
