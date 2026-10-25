0x14090d850 push r15
0x14090d852 push r14
0x14090d854 push r13
0x14090d856 push r12
0x14090d858 push rsi
0x14090d859 push rdi
0x14090d85a push rbp
0x14090d85b push rbx
0x14090d85c sub rsp, 0x78
0x14090d860 mov r14, r9
0x14090d863 mov rbx, r8
0x14090d866 mov rsi, rdx
0x14090d869 mov rdi, rcx
0x14090d86c movzx ebp, byte ptr [r8 + 0x21]
0x14090d871 lea rcx, [r9 + 4]
0x14090d875 call 0x140bd5790
0x14090d87a mov r15, rax
0x14090d87d xor r13d, r13d
0x14090d880 cmp bpl, 1
0x14090d884 setne bpl
0x14090d888 lea rax, [rip + 0x219d8a3]
0x14090d88f lea r12, [rip + 0x219d87c]
0x14090d896 cmove r12, rax
0x14090d89a lea rcx, [rsi + 0x38]
0x14090d89e movabs rdx, 0x8597ee0727b461b3
0x14090d8a8 movabs r8, 0x8b297ec5ae15f15c
0x14090d8b2 call 0x1400dd6f0
0x14090d8b7 test rax, rax
0x14090d8ba je 0x14090d986
0x14090d8c0 cmp qword ptr [rsi + 0x218], rdx
0x14090d8c7 jbe 0x14090d986
0x14090d8cd mov rax, qword ptr [rsi + 0x210]
0x14090d8d4 mov rcx, qword ptr [rax + rdx*8]
0x14090d8d8 test rcx, rcx
0x14090d8db je 0x14090d986
0x14090d8e1 mov rax, qword ptr [rsi + 0x1e0]
0x14090d8e8 not rcx
0x14090d8eb lea rcx, [rcx + rcx*2]
0x14090d8ef shl rcx, 5
0x14090d8f3 cmp qword ptr [rax + rcx + 0x20], 0
0x14090d8f9 je 0x14090d986
0x14090d8ff mov r13b, bpl
0x14090d902 lea r8, [r13*4 + 0x1b]
0x14090d90a add r8, r13
0x14090d90d add rax, rcx
0x14090d910 mov rcx, qword ptr [rax + 0x10]
0x14090d914 mov rdx, r12
0x14090d917 call 0x140937aa0
0x14090d91c test al, 1
0x14090d91e je 0x14090d992
0x14090d920 movzx eax, byte ptr [rsp + 0xe0]
0x14090d928 mov dword ptr [rsp + 0x58], 2
0x14090d930 mov qword ptr [rsp + 0x60], r14
0x14090d935 mov qword ptr [rsp + 0x48], r15
0x14090d93a mov qword ptr [rsp + 0x40], rbx
0x14090d93f mov byte ptr [rsp + 0x50], al
0x14090d943 lea rax, [rsp + 0x40]
0x14090d948 mov qword ptr [rsp + 0x30], rax
0x14090d94d lea rax, [rsp + 0x58]
0x14090d952 mov qword ptr [rsp + 0x28], rax
0x14090d957 mov qword ptr [rsp + 0x20], rdx
0x14090d95c lea r8, [rip + 0x219d81d]
0x14090d963 mov r9d, 0x14
0x14090d969 mov rcx, rdi
0x14090d96c mov rdx, rsi
0x14090d96f call 0x1409d00a0
0x14090d974 nop 
0x14090d975 add rsp, 0x78
0x14090d979 pop rbx
0x14090d97a pop rbp
0x14090d97b pop rdi
0x14090d97c pop rsi
0x14090d97d pop r12
0x14090d97f pop r13
0x14090d981 pop r14
0x14090d983 pop r15
0x14090d985 ret 
0x14090d986 lea rcx, [rip + 0x219d7c3]
0x14090d98d call 0x1428d9310
0x14090d992 lea rcx, [rip + 0x219d7cf]
0x14090d999 call 0x1428d9310
0x14090d99e int3 