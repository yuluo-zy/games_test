
/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a716c0: 48 83 ec 38                 	sub	rsp, 0x38
140a716c4: 48 89 d1                    	mov	rcx, rdx
140a716c7: 49 8b 40 18                 	mov	rax, qword ptr [r8 + 0x18]
140a716cb: 48 8d 15 f7 4a 06 02        	lea	rdx, [rip + 0x2064af7]  # 0x142ad61c9
140a716d2: 48 89 54 24 28              	mov	qword ptr [rsp + 0x28], rdx
140a716d7: 48 c7 44 24 30 14 00 00 00  	mov	qword ptr [rsp + 0x30], 0x14
140a716e0: 48 8d 15 59 4b 06 02        	lea	rdx, [rip + 0x2064b59]  # 0x142ad6240
140a716e7: 48 89 54 24 20              	mov	qword ptr [rsp + 0x20], rdx
140a716ec: 4c 8d 05 05 4b 06 02        	lea	r8, [rip + 0x2064b05]   # 0x142ad61f8
140a716f3: 48 8d 54 24 28              	lea	rdx, [rsp + 0x28]
140a716f8: 41 b9 01 00 00 00           	mov	r9d, 0x1
140a716fe: ff d0                       	call	rax
140a71700: 90                          	nop
140a71701: 48 83 c4 38                 	add	rsp, 0x38
140a71705: c3                          	ret
