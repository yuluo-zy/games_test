
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140bd4e20: 48 83 ec 28                 	sub	rsp, 0x28
140bd4e24: 48 85 d2                    	test	rdx, rdx
140bd4e27: 74 19                       	je	0x140bd4e42 <.text+0xbd3e42>
140bd4e29: 48 89 11                    	mov	qword ptr [rcx], rdx
140bd4e2c: 44 89 41 08                 	mov	dword ptr [rcx + 0x8], r8d
140bd4e30: c6 41 0e 01                 	mov	byte ptr [rcx + 0xe], 0x1
140bd4e34: 66 c7 41 0c 00 00           	mov	word ptr [rcx + 0xc], 0x0
140bd4e3a: 48 89 c8                    	mov	rax, rcx
140bd4e3d: 48 83 c4 28                 	add	rsp, 0x28
140bd4e41: c3                          	ret
140bd4e42: 48 8d 0d e7 a9 f3 01        	lea	rcx, [rip + 0x1f3a9e7]  # 0x142b0f830
140bd4e49: 4c 8d 05 18 aa f3 01        	lea	r8, [rip + 0x1f3aa18]   # 0x142b0f868
140bd4e50: ba 1a 00 00 00              	mov	edx, 0x1a
140bd4e55: e8 d6 45 d0 01              	call	0x1428d9430 <.text+0x28d8430>
140bd4e5a: cc                          	int3
