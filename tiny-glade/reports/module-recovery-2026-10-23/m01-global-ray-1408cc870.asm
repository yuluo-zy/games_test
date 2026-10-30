
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408cc870: 56                          	push	rsi
1408cc871: 48 83 ec 40                 	sub	rsp, 0x40
1408cc875: 48 89 ce                    	mov	rsi, rcx
1408cc878: 8b 44 24 70                 	mov	eax, dword ptr [rsp + 0x70]
1408cc87c: 89 44 24 30                 	mov	dword ptr [rsp + 0x30], eax
1408cc880: f3 0f 11 5c 24 28           	movss	dword ptr [rsp + 0x28], xmm3
1408cc886: 45 31 c9                    	xor	r9d, r9d
1408cc889: e8 d2 13 00 00              	call	0x1408cdc60 <.text+0x8ccc60>
1408cc88e: 48 89 f0                    	mov	rax, rsi
1408cc891: 48 83 c4 40                 	add	rsp, 0x40
1408cc895: 5e                          	pop	rsi
1408cc896: c3                          	ret
