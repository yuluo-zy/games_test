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
