
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14156b970: 56                          	push	rsi
14156b971: 48 83 ec 20                 	sub	rsp, 0x20
14156b975: 89 ce                       	mov	esi, ecx
14156b977: 0f b6 05 8b 92 0c 02        	movzx	eax, byte ptr [rip + 0x20c928b] # 0x143634c09
14156b97e: b9 04 00 00 00              	mov	ecx, 0x4
14156b983: ba 04 00 00 00              	mov	edx, 0x4
14156b988: e8 83 82 0a ff              	call	0x140613c10 <.text+0x612c10>
14156b98d: 48 85 c0                    	test	rax, rax
14156b990: 74 0f                       	je	0x14156b9a1 <.text+0x156a9a1>
14156b992: 89 30                       	mov	dword ptr [rax], esi
14156b994: 48 8d 15 8d a6 70 01        	lea	rdx, [rip + 0x170a68d]  # 0x142c76028
14156b99b: 48 83 c4 20                 	add	rsp, 0x20
14156b99f: 5e                          	pop	rsi
14156b9a0: c3                          	ret
14156b9a1: b9 04 00 00 00              	mov	ecx, 0x4
14156b9a6: ba 04 00 00 00              	mov	edx, 0x4
14156b9ab: e8 50 d6 36 01              	call	0x1428d9000 <.text+0x28d8000>
14156b9b0: cc                          	int3
