140b38810: push rbp
140b38811: push rsi
140b38812: push rdi
140b38813: push rbx
140b38814: sub rsp, 0x68
140b38818: lea rbp, [rsp + 0x60]
140b3881d: mov qword ptr [rbp], 0xfffffffffffffffe
140b38825: mov rsi, rcx
140b38828: mov rdi, qword ptr [rdx + 8]
140b3882c: mov rbx, qword ptr [rdx + 0x10]
140b38830: movzx eax, byte ptr [rip + 0x2afc3d2]
140b38837: mov ecx, 0x80
140b3883c: mov edx, 1
140b38841: call 0x140613c10
140b38846: test rax, rax
140b38849: je 0x140b38911
140b3884f: mov qword ptr [rbp - 0x18], 0x80
140b38857: mov qword ptr [rbp - 0x10], rax
140b3885b: mov qword ptr [rbp - 8], 0
140b38863: lea rax, [rbp - 0x18]
140b38867: mov qword ptr [rbp - 0x20], rax
140b3886b: shl rbx, 4
140b3886f: add rbx, rdi
140b38872: lea r9, [rip + 0x1fb5387]
140b38879: lea rcx, [rbp - 0x38]
140b3887d: mov rdx, rdi
140b38880: mov r8, rbx
140b38883: call 0x1413056c0
140b38888: lea rcx, [rbp - 0x20]
140b3888c: lea rdx, [rbp - 0x38]
140b38890: call 0x141581df0
140b38895: mov rdi, rax
140b38898: mov rdx, qword ptr [rbp - 0x38]
140b3889c: test rdx, rdx
140b3889f: je 0x140b388b4
140b388a1: mov rcx, qword ptr [rbp - 0x30]
140b388a5: shl rdx, 4
140b388a9: mov r8d, 8
140b388af: call 0x140613c20
140b388b4: test rdi, rdi
140b388b7: je 0x140b388d3
140b388b9: mov rdx, qword ptr [rbp - 0x18]
140b388bd: test rdx, rdx
140b388c0: je 0x140b388e3
140b388c2: mov rcx, qword ptr [rbp - 0x10]
140b388c6: mov r8d, 1
140b388cc: call 0x140613c20
140b388d1: jmp 0x140b388e3
140b388d3: mov rax, qword ptr [rbp - 0x18]
140b388d7: mov rdi, qword ptr [rbp - 0x10]
140b388db: mov rcx, rax
140b388de: neg rcx
140b388e1: jno 0x140b38900
140b388e3: mov qword ptr [rsi + 8], rdi
140b388e7: movabs rax, 0x8000000000000000
140b388f1: mov qword ptr [rsi], rax
140b388f4: mov rax, rsi
140b388f7: add rsp, 0x68
140b388fb: pop rbx
140b388fc: pop rdi
140b388fd: pop rsi
140b388fe: pop rbp
140b388ff: ret 
140b38900: mov rcx, qword ptr [rbp - 8]
140b38904: mov qword ptr [rsi], rax
140b38907: mov qword ptr [rsi + 8], rdi
140b3890b: mov qword ptr [rsi + 0x10], rcx
140b3890f: jmp 0x140b388f4
140b38911: lea r8, [rip + 0x1fb5178]
140b38918: mov ecx, 1
140b3891d: mov edx, 0x80
140b38922: call 0x1428d8fe3
140b38927: int3 
140b38928: nop dword ptr [rax + rax]
140b38930: mov qword ptr [rsp + 0x10], rdx
140b38935: push rbp
140b38936: push rsi
140b38937: push rdi
140b38938: push rbx
140b38939: sub rsp, 0x28
140b3893d: lea rbp, [rdx + 0x60]
140b38941: mov rdx, qword ptr [rbp - 0x38]
140b38945: test rdx, rdx
140b38948: je 0x140b3895d
140b3894a: mov rcx, qword ptr [rbp - 0x30]
140b3894e: shl rdx, 4
140b38952: mov r8d, 8
140b38958: call 0x140613c20
140b3895d: nop 
140b3895e: add rsp, 0x28
140b38962: pop rbx
140b38963: pop rdi
140b38964: pop rsi
140b38965: pop rbp
140b38966: ret 
140b38967: nop word ptr [rax + rax]
140b38970: mov qword ptr [rsp + 0x10], rdx
140b38975: push rbp
140b38976: push rsi
140b38977: push rdi
140b38978: push rbx
140b38979: sub rsp, 0x28
140b3897d: lea rbp, [rdx + 0x60]
140b38981: mov rdx, qword ptr [rbp - 0x18]
140b38985: test rdx, rdx
140b38988: je 0x140b38999
140b3898a: mov rcx, qword ptr [rbp - 0x10]
140b3898e: mov r8d, 1
140b38994: call 0x140613c20
140b38999: nop 
140b3899a: add rsp, 0x28
140b3899e: pop rbx
140b3899f: pop rdi
140b389a0: pop rsi
140b389a1: pop rbp
140b389a2: ret 
