140317160: push r15
140317162: push r14
140317164: push rsi
140317165: push rdi
140317166: push rbx
140317167: sub rsp, 0x40
14031716b: mov r14, rdx
14031716e: mov rsi, rcx
140317171: mov rbx, qword ptr [rcx + 0x18]
140317175: add rbx, 0x10
140317179: mov rcx, rbx
14031717c: call 0x140a84290
140317181: mov rdi, qword ptr [rsi + 0x20]
140317185: add rdi, 0x10
140317189: mov rcx, rdi
14031718c: call 0x140a842a0
140317191: mov r15, qword ptr [r14]
140317194: mov rdx, qword ptr [r15 + 0x10]
140317198: cmp qword ptr [r15], rdx
14031719b: je 0x14031726d
1403171a1: mov rax, qword ptr [r15 + 8]
1403171a5: mov byte ptr [rax + rdx], 0x7b
1403171a9: inc rdx
1403171ac: mov qword ptr [r15 + 0x10], rdx
1403171b0: mov qword ptr [rsp + 0x30], r14
1403171b5: mov byte ptr [rsp + 0x38], 1
1403171ba: lea rdx, [rip + 0x26a4125] ; 0x1429bb2e6 b'editsdecoratorsstairs\x00\x07\x9b\x95\xfd?\xa4\x95\xfd\xdd\xa1\x95\xfd\xd5\xa3\x95\xfd\t\x9f\x95\xfd\x84\xa5\x95\xfdl\xa7\x95\xfd\n\xa4\x95\xfdT\xa9\x95\xfd\xe9\xa0'
1403171c1: lea rcx, [rsp + 0x30]
1403171c6: mov r8d, 5
1403171cc: mov r9, rsi
1403171cf: call 0x1404bb660
1403171d4: test rax, rax
1403171d7: jne 0x140317261
1403171dd: mov rcx, rbx
1403171e0: call 0x140a84290
1403171e5: test al, al
1403171e7: jne 0x140317209
1403171e9: lea r9, [rsi + 0x18]
1403171ed: lea rdx, [rip + 0x26a40f7] ; 0x1429bb2eb b'decoratorsstairs\x00\x07\x9b\x95\xfd?\xa4\x95\xfd\xdd\xa1\x95\xfd\xd5\xa3\x95\xfd\t\x9f\x95\xfd\x84\xa5\x95\xfdl\xa7\x95\xfd\n\xa4\x95\xfdT\xa9\x95\xfd\xe9\xa0\x95\xfd`\xa8\x95'
1403171f4: lea rcx, [rsp + 0x30]
1403171f9: mov r8d, 0xa
1403171ff: call 0x1404c1620
140317204: test rax, rax
140317207: jne 0x140317261
140317209: mov rcx, rdi
14031720c: call 0x140a842a0
140317211: test al, al
140317213: jne 0x140317238
140317215: add rsi, 0x20
140317219: lea rdx, [rip + 0x26a40d5] ; 0x1429bb2f5 b'stairs\x00\x07\x9b\x95\xfd?\xa4\x95\xfd\xdd\xa1\x95\xfd\xd5\xa3\x95\xfd\t\x9f\x95\xfd\x84\xa5\x95\xfdl\xa7\x95\xfd\n\xa4\x95\xfdT\xa9\x95\xfd\xe9\xa0\x95\xfd`\xa8\x95\xfd\x15\x9e\x95\xfd\xfd\x9f\x95\xfdx'
140317220: lea rcx, [rsp + 0x30]
140317225: mov r8d, 6
14031722b: mov r9, rsi
14031722e: call 0x1404c04c0
140317233: test rax, rax
140317236: jne 0x140317261
140317238: cmp byte ptr [rsp + 0x38], 0
14031723d: je 0x14031725f
14031723f: mov rax, qword ptr [rsp + 0x30]
140317244: mov rsi, qword ptr [rax]
140317247: mov rdx, qword ptr [rsi + 0x10]
14031724b: cmp qword ptr [rsi], rdx
14031724e: je 0x140317293
140317250: mov rax, qword ptr [rsi + 8]
140317254: mov byte ptr [rax + rdx], 0x7d
140317258: inc rdx
14031725b: mov qword ptr [rsi + 0x10], rdx
14031725f: xor eax, eax
140317261: add rsp, 0x40
140317265: pop rbx
140317266: pop rdi
140317267: pop rsi
140317268: pop r14
14031726a: pop r15
14031726c: ret 
14031726d: mov qword ptr [rsp + 0x20], 1
140317276: mov r8d, 1
14031727c: mov r9d, 1
140317282: mov rcx, r15
140317285: call 0x1428a29e0
14031728a: mov rdx, qword ptr [r15 + 0x10]
14031728e: jmp 0x1403171a1
140317293: mov qword ptr [rsp + 0x20], 1
14031729c: mov r8d, 1
1403172a2: mov r9d, 1
1403172a8: mov rcx, rsi
1403172ab: call 0x1428a29e0
1403172b0: mov rdx, qword ptr [rsi + 0x10]
1403172b4: jmp 0x140317250
