
/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14090e280: 41 57                       	push	r15
14090e282: 41 56                       	push	r14
14090e284: 41 54                       	push	r12
14090e286: 56                          	push	rsi
14090e287: 57                          	push	rdi
14090e288: 53                          	push	rbx
14090e289: 48 83 ec 38                 	sub	rsp, 0x38
14090e28d: 48 8b 41 10                 	mov	rax, qword ptr [rcx + 0x10]
14090e291: 48 85 c0                    	test	rax, rax
14090e294: 74 6a                       	je	0x14090e300 <.text+0x90d300>
14090e296: 4c 89 c6                    	mov	rsi, r8
14090e299: 48 89 d7                    	mov	rdi, rdx
14090e29c: 4c 8b 79 08                 	mov	r15, qword ptr [rcx + 0x8]
14090e2a0: 48 c1 e0 04                 	shl	rax, 0x4
14090e2a4: 4c 8d 24 40                 	lea	r12, [rax + 2*rax]
14090e2a8: 48 8d 1d 31 d4 19 02        	lea	rbx, [rip + 0x219d431]  # 0x142aab6e0
14090e2af: 4c 8d 74 24 20              	lea	r14, [rsp + 0x20]
14090e2b4: eb 14                       	jmp	0x14090e2ca <.text+0x90d2ca>
14090e2b6: 66 2e 0f 1f 84 00 00 00 00 00       	nop	word ptr cs:[rax + rax]
14090e2c0: 49 83 c7 30                 	add	r15, 0x30
14090e2c4: 49 83 c4 d0                 	add	r12, -0x30
14090e2c8: 74 36                       	je	0x14090e300 <.text+0x90d300>
14090e2ca: 41 80 7f 28 00              	cmp	byte ptr [r15 + 0x28], 0x0
14090e2cf: 75 ef                       	jne	0x14090e2c0 <.text+0x90d2c0>
14090e2d1: 49 c7 47 10 00 00 00 00     	mov	qword ptr [r15 + 0x10], 0x0
14090e2d9: 48 89 7c 24 20              	mov	qword ptr [rsp + 0x20], rdi
14090e2de: 48 89 74 24 30              	mov	qword ptr [rsp + 0x30], rsi
14090e2e3: 4c 89 7c 24 28              	mov	qword ptr [rsp + 0x28], r15
14090e2e8: 49 8b 4f 18                 	mov	rcx, qword ptr [r15 + 0x18]
14090e2ec: 49 8b 47 20                 	mov	rax, qword ptr [r15 + 0x20]
14090e2f0: 4c 89 f2                    	mov	rdx, r14
14090e2f3: 49 89 d8                    	mov	r8, rbx
14090e2f6: ff 50 28                    	call	qword ptr [rax + 0x28]
14090e2f9: 41 c6 47 28 01              	mov	byte ptr [r15 + 0x28], 0x1
14090e2fe: eb c0                       	jmp	0x14090e2c0 <.text+0x90d2c0>
14090e300: 48 83 c4 38                 	add	rsp, 0x38
14090e304: 5b                          	pop	rbx
14090e305: 5f                          	pop	rdi
14090e306: 5e                          	pop	rsi
14090e307: 41 5c                       	pop	r12
14090e309: 41 5e                       	pop	r14
14090e30b: 41 5f                       	pop	r15
14090e30d: c3                          	ret
