
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140d75fb0: 55                          	push	rbp
140d75fb1: 56                          	push	rsi
140d75fb2: 57                          	push	rdi
140d75fb3: 48 83 ec 20                 	sub	rsp, 0x20
140d75fb7: 48 8d 6c 24 20              	lea	rbp, [rsp + 0x20]
140d75fbc: 4c 89 c6                    	mov	rsi, r8
140d75fbf: 89 d7                       	mov	edi, edx
140d75fc1: ff 15 b1 c5 41 02           	call	qword ptr [rip + 0x241c5b1] # 0x143192578
140d75fc7: 48 85 c0                    	test	rax, rax
140d75fca: 74 16                       	je	0x140d75fe2 <.text+0xd74fe2>
140d75fcc: 48 89 c1                    	mov	rcx, rax
140d75fcf: 89 fa                       	mov	edx, edi
140d75fd1: 49 89 f0                    	mov	r8, rsi
140d75fd4: 48 83 c4 20                 	add	rsp, 0x20
140d75fd8: 5f                          	pop	rdi
140d75fd9: 5e                          	pop	rsi
140d75fda: 5d                          	pop	rbp
140d75fdb: 48 ff 25 16 c6 41 02        	jmp	qword ptr [rip + 0x241c616] # 0x1431925f8
140d75fe2: 31 c0                       	xor	eax, eax
140d75fe4: 48 83 c4 20                 	add	rsp, 0x20
140d75fe8: 5f                          	pop	rdi
140d75fe9: 5e                          	pop	rsi
140d75fea: 5d                          	pop	rbp
140d75feb: c3                          	ret
