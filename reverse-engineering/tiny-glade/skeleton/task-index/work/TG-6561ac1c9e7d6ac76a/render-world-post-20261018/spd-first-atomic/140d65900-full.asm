
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140d65900: 55                          	push	rbp
140d65901: 56                          	push	rsi
140d65902: 48 83 ec 28                 	sub	rsp, 0x28
140d65906: 48 8d 6c 24 20              	lea	rbp, [rsp + 0x20]
140d6590b: 48 83 fa 11                 	cmp	rdx, 0x11
140d6590f: 73 13                       	jae	0x140d65924 <.text+0xd64924>
140d65911: ba 08 00 00 00              	mov	edx, 0x8
140d65916: 49 89 c8                    	mov	r8, rcx
140d65919: 48 83 c4 28                 	add	rsp, 0x28
140d6591d: 5e                          	pop	rsi
140d6591e: 5d                          	pop	rbp
140d6591f: e9 8c 06 01 00              	jmp	0x140d75fb0 <.text+0xd74fb0>
140d65924: 48 89 d6                    	mov	rsi, rdx
140d65927: 48 01 d1                    	add	rcx, rdx
140d6592a: ba 08 00 00 00              	mov	edx, 0x8
140d6592f: 49 89 c8                    	mov	r8, rcx
140d65932: e8 79 06 01 00              	call	0x140d75fb0 <.text+0xd74fb0>
140d65937: 48 85 c0                    	test	rax, rax
140d6593a: 74 17                       	je	0x140d65953 <.text+0xd64953>
140d6593c: 48 8d 4e ff                 	lea	rcx, [rsi - 0x1]
140d65940: 48 21 c1                    	and	rcx, rax
140d65943: 48 29 ce                    	sub	rsi, rcx
140d65946: 48 89 c1                    	mov	rcx, rax
140d65949: 48 01 f1                    	add	rcx, rsi
140d6594c: 48 89 44 30 f8              	mov	qword ptr [rax + rsi - 0x8], rax
140d65951: eb 02                       	jmp	0x140d65955 <.text+0xd64955>
140d65953: 31 c9                       	xor	ecx, ecx
140d65955: 48 89 c8                    	mov	rax, rcx
140d65958: 48 83 c4 28                 	add	rsp, 0x28
140d6595c: 5e                          	pop	rsi
140d6595d: 5d                          	pop	rbp
140d6595e: c3                          	ret
