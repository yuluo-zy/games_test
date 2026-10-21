
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
142340660: 56                          	push	rsi
142340661: 48 83 ec 60                 	sub	rsp, 0x60
142340665: 48 89 ce                    	mov	rsi, rcx
142340668: c7 44 24 24 00 00 00 00     	mov	dword ptr [rsp + 0x24], 0x0
142340670: c7 44 24 2c ff ff 7f 7f     	mov	dword ptr [rsp + 0x2c], 0x7f7fffff
142340678: 48 8b 01                    	mov	rax, qword ptr [rcx]
14234067b: 48 8b 08                    	mov	rcx, qword ptr [rax]
14234067e: 0f 10 46 08                 	movups	xmm0, xmmword ptr [rsi + 0x8]
142340682: 0f 29 44 24 30              	movaps	xmmword ptr [rsp + 0x30], xmm0
142340687: 0f 10 46 18                 	movups	xmm0, xmmword ptr [rsi + 0x18]
14234068b: 0f 29 44 24 40              	movaps	xmmword ptr [rsp + 0x40], xmm0
142340690: 48 8d 44 24 2c              	lea	rax, [rsp + 0x2c]
142340695: 48 89 44 24 50              	mov	qword ptr [rsp + 0x50], rax
14234069a: 48 8d 44 24 20              	lea	rax, [rsp + 0x20]
14234069f: 48 89 44 24 58              	mov	qword ptr [rsp + 0x58], rax
1423406a4: 4c 8d 4c 24 30              	lea	r9, [rsp + 0x30]
1423406a9: e8 b2 85 b6 ff              	call	0x141ea8c60 <.text+0x1ea7c60>
1423406ae: 8b 44 24 24                 	mov	eax, dword ptr [rsp + 0x24]
1423406b2: 85 c0                       	test	eax, eax
1423406b4: 74 23                       	je	0x1423406d9 <.text+0x233f6d9>
1423406b6: 48 8b 4c 24 20              	mov	rcx, qword ptr [rsp + 0x20]
1423406bb: 48 8b 56 28                 	mov	rdx, qword ptr [rsi + 0x28]
1423406bf: 44 8b 42 1c                 	mov	r8d, dword ptr [rdx + 0x1c]
1423406c3: 4c 8b 4a 10                 	mov	r9, qword ptr [rdx + 0x10]
1423406c7: 45 89 01                    	mov	dword ptr [r9], r8d
1423406ca: 48 8b 12                    	mov	rdx, qword ptr [rdx]
1423406cd: 48 c7 42 10 01 00 00 00     	mov	qword ptr [rdx + 0x10], 0x1
1423406d5: 48 89 4a 18                 	mov	qword ptr [rdx + 0x18], rcx
1423406d9: 85 c0                       	test	eax, eax
1423406db: 0f 95 c0                    	setne	al
1423406de: 48 83 c4 60                 	add	rsp, 0x60
1423406e2: 5e                          	pop	rsi
1423406e3: c3                          	ret
