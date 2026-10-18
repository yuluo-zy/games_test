140327490: push rbp
140327491: push rsi
140327492: push rdi
140327493: push rbx
140327494: sub rsp, 0x6e8
14032749b: lea rbp, [rsp + 0x80]
1403274a3: mov qword ptr [rbp + 0x660], 0xfffffffffffffffe
1403274ae: mov qword ptr [rbp + 0x650], rdx
1403274b5: mov rsi, rcx
1403274b8: add rcx, 0x38
1403274bc: mov byte ptr [rbp + 0x65f], 1
1403274c3: movabs rdx, 0x8e1bd251962ba784
1403274cd: movabs r8, 0x6a782374c0e6529b
1403274d7: call 0x1400dd6f0
1403274dc: test rax, rax
1403274df: je 0x1403275dc
1403274e5: cmp qword ptr [rsi + 0x218], rdx
1403274ec: jbe 0x1403275dc
1403274f2: mov rax, qword ptr [rsi + 0x210]
1403274f9: mov rcx, qword ptr [rax + rdx*8]
1403274fd: test rcx, rcx
140327500: je 0x1403275dc
140327506: mov rax, qword ptr [rsi + 0x1e0]
14032750d: not rcx
140327510: lea rcx, [rcx + rcx*2]
140327514: shl rcx, 5
140327518: cmp qword ptr [rax + rcx + 0x20], 0
14032751e: je 0x1403275dc
140327524: add rax, rcx
140327527: mov rdi, qword ptr [rax + 0x10]
14032752b: mov ecx, dword ptr [rsi + 0x5b0]
140327531: mov dword ptr [rax + 0x5c], ecx
140327534: lea rcx, [rbp - 0x60]
140327538: mov r8d, 0x310
14032753e: mov rdx, qword ptr [rbp + 0x650]
140327545: call 0x142923c00
14032754a: mov byte ptr [rbp + 0x65e], 1
140327551: mov rcx, rdi
140327554: call 0x140172580
140327559: mov rdi, rax
14032755c: lea rbx, [rbp + 0x2b0]
140327563: lea rdx, [rbp - 0x60]
140327567: mov r8d, 0x310
14032756d: mov rcx, rbx
140327570: call 0x142923c00
140327575: mov byte ptr [rbp + 0x65e], 0
14032757c: lea rcx, [rbp + 0x5c0]
140327583: mov rdx, rbx
140327586: call 0x1400d4610
14032758b: mov byte ptr [rbp + 0x65e], 0
140327592: lea rcx, [rbp + 0x630]
140327599: lea r8, [rbp + 0x5c0]
1403275a0: mov rdx, rdi
1403275a3: xor r9d, r9d
1403275a6: call 0x14016dee0
1403275ab: mov rdx, qword ptr [rbp + 0x630]
1403275b2: test rdx, rdx
1403275b5: je 0x1403275cd
1403275b7: mov rcx, qword ptr [rbp + 0x638]
1403275be: shl rdx, 4
1403275c2: mov r8d, 8
1403275c8: call 0x140613c20
1403275cd: mov rax, rsi
1403275d0: add rsp, 0x6e8
1403275d7: pop rbx
1403275d8: pop rdi
1403275d9: pop rsi
1403275da: pop rbp
1403275db: ret 
1403275dc: lea rax, [rip + 0x2696c55]
1403275e3: mov qword ptr [rbp - 0x60], rax
1403275e7: mov qword ptr [rbp - 0x58], 0x27
1403275ef: lea rax, [rbp - 0x60]
1403275f3: mov qword ptr [rbp + 0x5c0], rax
1403275fa: lea rax, [rip - 0x21d771]
140327601: mov qword ptr [rbp + 0x5c8], rax
140327608: lea rax, [rip + 0x2696d59]
14032760f: mov qword ptr [rbp + 0x2b0], rax
140327616: mov qword ptr [rbp + 0x2b8], 2
140327621: mov qword ptr [rbp + 0x2d0], 0
14032762c: lea rax, [rbp + 0x5c0]
140327633: mov qword ptr [rbp + 0x2c0], rax
14032763a: mov qword ptr [rbp + 0x2c8], 1
140327645: mov byte ptr [rbp + 0x65f], 1
14032764c: lea rdx, [rip + 0x2696b9d]
140327653: lea rcx, [rbp + 0x2b0]
14032765a: call 0x1428d9390
14032765f: ud2 
140327661: nop word ptr cs:[rax + rax]
140327670: mov qword ptr [rsp + 0x10], rdx
140327675: push rbp
140327676: push rsi
140327677: push rdi
140327678: push rbx
140327679: sub rsp, 0x28
14032767d: lea rbp, [rdx + 0x80]
140327684: cmp byte ptr [rbp + 0x65e], 0
14032768b: je 0x14032769d
14032768d: mov byte ptr [rbp + 0x65f], 0
140327694: lea rcx, [rbp - 0x60]
140327698: call 0x14016d290
14032769d: mov byte ptr [rbp + 0x65f], 0
1403276a4: add rsp, 0x28
1403276a8: pop rbx
1403276a9: pop rdi
1403276aa: pop rsi
1403276ab: pop rbp
1403276ac: ret 
1403276ad: nop dword ptr [rax]
1403276b0: mov qword ptr [rsp + 0x10], rdx
1403276b5: push rbp
1403276b6: push rsi
1403276b7: push rdi
1403276b8: push rbx
1403276b9: sub rsp, 0x28
1403276bd: lea rbp, [rdx + 0x80]
1403276c4: cmp byte ptr [rbp + 0x65f], 0
1403276cb: jne 0x1403276d6
1403276cd: add rsp, 0x28
1403276d1: pop rbx
1403276d2: pop rdi
1403276d3: pop rsi
1403276d4: pop rbp
1403276d5: ret 
1403276d6: mov rcx, qword ptr [rbp + 0x650]
1403276dd: call 0x14016d290
1403276e2: jmp 0x1403276cd
