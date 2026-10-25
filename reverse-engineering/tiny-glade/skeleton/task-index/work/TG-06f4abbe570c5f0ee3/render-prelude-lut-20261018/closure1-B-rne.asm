
/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a71980: 48 83 ec 38                 	sub	rsp, 0x38
140a71984: 48 89 d1                    	mov	rcx, rdx
140a71987: 49 8b 40 18                 	mov	rax, qword ptr [r8 + 0x18]
140a7198b: 48 8d 15 4b 48 06 02        	lea	rdx, [rip + 0x206484b]  # 0x142ad61dd
140a71992: 48 89 54 24 28              	mov	qword ptr [rsp + 0x28], rdx
140a71997: 48 c7 44 24 30 18 00 00 00  	mov	qword ptr [rsp + 0x30], 0x18
140a719a0: 48 8d 15 79 48 06 02        	lea	rdx, [rip + 0x2064879]  # 0x142ad6220
140a719a7: 48 89 54 24 20              	mov	qword ptr [rsp + 0x20], rdx
140a719ac: 4c 8d 05 45 48 06 02        	lea	r8, [rip + 0x2064845]   # 0x142ad61f8
140a719b3: 48 8d 54 24 28              	lea	rdx, [rsp + 0x28]
140a719b8: 41 b9 01 00 00 00           	mov	r9d, 0x1
140a719be: ff d0                       	call	rax
140a719c0: 90                          	nop
140a719c1: 48 83 c4 38                 	add	rsp, 0x38
140a719c5: c3                          	ret
