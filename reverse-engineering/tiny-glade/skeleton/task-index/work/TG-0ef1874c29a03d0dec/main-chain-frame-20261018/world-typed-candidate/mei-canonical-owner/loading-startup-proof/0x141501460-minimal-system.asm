
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141501460: 56                          	push	rsi
141501461: 48 83 ec 20                 	sub	rsp, 0x20
141501465: 48 89 ce                    	mov	rsi, rcx
141501468: e8 83 7b ff ff              	call	0x1414f8ff0 <.text+0x14f7ff0>
14150146d: f6 06 01                    	test	byte ptr [rsi], 0x1
141501470: 74 08                       	je	0x14150147a <.text+0x150047a>
141501472: 31 c0                       	xor	eax, eax
141501474: 48 83 c4 20                 	add	rsp, 0x20
141501478: 5e                          	pop	rsi
141501479: c3                          	ret
14150147a: 48 8d 0d c0 a4 76 01        	lea	rcx, [rip + 0x176a4c0]  # 0x142c6b941
141501481: 4c 8d 05 98 a5 76 01        	lea	r8, [rip + 0x176a598]   # 0x142c6ba20
141501488: ba 59 00 00 00              	mov	edx, 0x59
14150148d: e8 9e 7e 3d 01              	call	0x1428d9330 <.text+0x28d8330>
141501492: cc                          	int3
