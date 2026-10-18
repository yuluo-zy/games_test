
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140bd51b0: 48 83 ec 28                 	sub	rsp, 0x28
140bd51b4: 48 8b 01                    	mov	rax, qword ptr [rcx]
140bd51b7: 48 85 c0                    	test	rax, rax
140bd51ba: 74 18                       	je	0x140bd51d4 <.text+0xbd41d4>
140bd51bc: 4c 8b 49 08                 	mov	r9, qword ptr [rcx + 0x8]
140bd51c0: 4d 39 c1                    	cmp	r9, r8
140bd51c3: 75 15                       	jne	0x140bd51da <.text+0xbd41da>
140bd51c5: 48 89 d1                    	mov	rcx, rdx
140bd51c8: 48 89 c2                    	mov	rdx, rax
140bd51cb: 48 83 c4 28                 	add	rsp, 0x28
140bd51cf: e9 2c ea d4 01              	jmp	0x142923c00 <.text+0x2922c00>
140bd51d4: 90                          	nop
140bd51d5: 48 83 c4 28                 	add	rsp, 0x28
140bd51d9: c3                          	ret
140bd51da: 48 8d 05 87 a7 f3 01        	lea	rax, [rip + 0x1f3a787]  # 0x142b0f968
140bd51e1: 4c 89 c1                    	mov	rcx, r8
140bd51e4: 4c 89 ca                    	mov	rdx, r9
140bd51e7: 49 89 c0                    	mov	r8, rax
140bd51ea: e8 f1 46 d0 01              	call	0x1428d98e0 <.text+0x28d88e0>
140bd51ef: cc                          	int3
