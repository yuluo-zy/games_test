0x140259300 push r14
0x140259302 push rsi
0x140259303 push rdi
0x140259304 push rbx
0x140259305 sub rsp, 0x38
0x140259309 mov rsi, rdx
0x14025930c mov edx, dword ptr [rcx]
0x14025930e add edx, -2
0x140259311 cmp edx, 2
0x140259314 mov eax, 2
0x140259319 cmovb eax, edx
0x14025931c test eax, eax
0x14025931e je 0x140259349
0x140259320 cmp eax, 1
0x140259323 jne 0x14025937c
0x140259325 lea rdx, [rsi + 8]
0x140259329 lea r8, [rip + 0x274b883]
0x140259330 mov r9d, 3
0x140259336 mov rcx, rsi
0x140259339 call 0x1404b6100
0x14025933e test rax, rax
0x140259341 je 0x140259466
0x140259347 jmp 0x14025936b
0x140259349 lea rdx, [rsi + 8]
0x14025934d lea r8, [rip + 0x274b85d]
0x140259354 mov r9d, 2
0x14025935a mov rcx, rsi
0x14025935d call 0x1404b6100
0x140259362 test rax, rax
0x140259365 je 0x140259466
0x14025936b mov rcx, rax
0x14025936e add rsp, 0x38
0x140259372 pop rbx
0x140259373 pop rdi
0x140259374 pop rsi
0x140259375 pop r14
0x140259377 jmp 0x1428c98a0
0x14025937c mov rdi, rcx
0x14025937f lea rdx, [rsi + 8]
0x140259383 mov rbx, qword ptr [rsi]
0x140259386 mov rax, qword ptr [rbx + 0x10]
0x14025938a cmp qword ptr [rbx], rax
0x14025938d je 0x1402594a8
0x140259393 mov rcx, qword ptr [rbx + 8]
0x140259397 mov byte ptr [rcx + rax], 0x7b
0x14025939b inc rax
0x14025939e mov qword ptr [rbx + 0x10], rax
0x1402593a2 lea r8, [rip + 0x274b80d]
0x1402593a9 mov r9d, 0xc
0x1402593af mov rcx, rsi
0x1402593b2 call 0x1404b6100
0x1402593b7 test rax, rax
0x1402593ba jne 0x14025949e
0x1402593c0 mov rbx, qword ptr [rsi]
0x1402593c3 mov rdx, qword ptr [rbx + 0x10]
0x1402593c7 cmp qword ptr [rbx], rdx
0x1402593ca je 0x1402594d7
0x1402593d0 mov rax, qword ptr [rbx + 8]
0x1402593d4 mov byte ptr [rax + rdx], 0x3a
0x1402593d8 inc rdx
0x1402593db mov qword ptr [rbx + 0x10], rdx
0x1402593df mov rbx, qword ptr [rsi]
0x1402593e2 mov rdx, qword ptr [rbx + 0x10]
0x1402593e6 cmp qword ptr [rbx], rdx
0x1402593e9 je 0x1402594fd
0x1402593ef mov rax, qword ptr [rbx + 8]
0x1402593f3 mov byte ptr [rax + rdx], 0x7b
0x1402593f7 inc rdx
0x1402593fa mov qword ptr [rbx + 0x10], rdx
0x1402593fe mov qword ptr [rsp + 0x28], rsi
0x140259403 mov byte ptr [rsp + 0x30], 1
0x140259408 lea rdx, [rip + 0x274b7b3]
0x14025940f lea rcx, [rsp + 0x28]
0x140259414 mov r8d, 6
0x14025941a mov r9, rdi
0x14025941d call 0x1404b90f0
0x140259422 test rax, rax
0x140259425 jne 0x140259468
0x140259427 mov rax, qword ptr [rsp + 0x28]
0x14025942c mov rsi, qword ptr [rax]
0x14025942f cmp byte ptr [rsp + 0x30], 0
0x140259434 je 0x140259472
0x140259436 mov rdx, qword ptr [rsi + 0x10]
0x14025943a cmp qword ptr [rsi], rdx
0x14025943d je 0x140259523
0x140259443 mov rax, qword ptr [rsi + 8]
0x140259447 mov byte ptr [rax + rdx], 0x7d
0x14025944b inc rdx
0x14025944e mov qword ptr [rsi + 0x10], rdx
0x140259452 cmp qword ptr [rsi], rdx
0x140259455 je 0x14025947b
0x140259457 mov rax, qword ptr [rsi + 8]
0x14025945b mov byte ptr [rax + rdx], 0x7d
0x14025945f inc rdx
0x140259462 mov qword ptr [rsi + 0x10], rdx
0x140259466 xor eax, eax
0x140259468 add rsp, 0x38
0x14025946c pop rbx
0x14025946d pop rdi
0x14025946e pop rsi
0x14025946f pop r14
0x140259471 ret 
0x140259472 mov rdx, qword ptr [rsi + 0x10]
0x140259476 cmp qword ptr [rsi], rdx
0x140259479 jne 0x140259457
0x14025947b mov qword ptr [rsp + 0x20], 1
0x140259484 mov r8d, 1
0x14025948a mov r9d, 1
0x140259490 mov rcx, rsi
0x140259493 call 0x1428a29e0
0x140259498 mov rdx, qword ptr [rsi + 0x10]
0x14025949c jmp 0x140259457
0x14025949e mov rcx, rax
0x1402594a1 call 0x1428c98a0
0x1402594a6 jmp 0x140259468
0x1402594a8 mov qword ptr [rsp + 0x20], 1
0x1402594b1 mov r8d, 1
0x1402594b7 mov r9d, 1
0x1402594bd mov rcx, rbx
0x1402594c0 mov r14, rdx
0x1402594c3 mov rdx, rax
0x1402594c6 call 0x1428a29e0
0x1402594cb mov rdx, r14
0x1402594ce mov rax, qword ptr [rbx + 0x10]
0x1402594d2 jmp 0x140259393
0x1402594d7 mov qword ptr [rsp + 0x20], 1
0x1402594e0 mov r8d, 1
0x1402594e6 mov r9d, 1
0x1402594ec mov rcx, rbx
0x1402594ef call 0x1428a29e0
0x1402594f4 mov rdx, qword ptr [rbx + 0x10]
0x1402594f8 jmp 0x1402593d0
0x1402594fd mov qword ptr [rsp + 0x20], 1
0x140259506 mov r8d, 1
0x14025950c mov r9d, 1
0x140259512 mov rcx, rbx
0x140259515 call 0x1428a29e0
0x14025951a mov rdx, qword ptr [rbx + 0x10]
0x14025951e jmp 0x1402593ef
0x140259523 mov qword ptr [rsp + 0x20], 1
0x14025952c mov r8d, 1
0x140259532 mov r9d, 1
0x140259538 mov rcx, rsi
0x14025953b call 0x1428a29e0
0x140259540 mov rdx, qword ptr [rsi + 0x10]
0x140259544 jmp 0x140259443
