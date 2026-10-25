1412388f0 push rbp
1412388f1 push r15
1412388f3 push r14
1412388f5 push r13
1412388f7 push r12
1412388f9 push rsi
1412388fa push rdi
1412388fb push rbx
1412388fc sub rsp, 0x108
141238903 lea rbp, [rsp + 0x80]
14123890b mov qword ptr [rbp + 0x80], 0xfffffffffffffffe
141238916 mov rdi, r9
141238919 mov rbx, r8
14123891c mov rsi, rcx
14123891f mov qword ptr [rbp + 0x38], rdx
141238923 call 0x140c16a00
141238928 test al, al
14123892a mov byte ptr [rbp + 0x78], al
14123892d je 0x14123898c
14123892f mov r14d, 0x47
141238935 mov ecx, 0x47
14123893a call 0x1412384c0
14123893f lea r15, [rip + 0x19e62aa]
141238946 cmp rax, 1
14123894a jne 0x1412389f8
141238950 mov r9, rdx
141238953 test rdx, rdx
141238956 je 0x141238995
141238958 cmp r9, 0x47
14123895c jae 0x141238993
14123895e lea rax, [rip + 0x19e628b]
141238965 cmp byte ptr [r9 + rax], 0xbf
14123896a jg 0x141238995
14123896c lea rax, [rip + 0x19e634d]
141238973 mov qword ptr [rsp + 0x20], rax
141238978 lea rcx, [rip + 0x19e6271]
14123897f mov edx, 0x47
141238984 xor r8d, r8d
141238987 call 0x1428d9940
14123898c xor eax, eax
14123898e jmp 0x141238b4d
141238993 jne 0x14123896c
141238995 mov rcx, r9
141238998 call 0x1412384c0
14123899d cmp rax, 1
1412389a1 jne 0x1412389f8
1412389a3 mov r8, rdx
1412389a6 add r8, 2
1412389aa je 0x1412389e5
1412389ac cmp r8, 0x47
1412389b0 jae 0x1412389e3
1412389b2 lea rax, [rip + 0x19e6237]
1412389b9 cmp byte ptr [r8 + rax], 0xbf
1412389be jg 0x1412389e5
1412389c0 lea rax, [rip + 0x19e6311]
1412389c7 mov qword ptr [rsp + 0x20], rax
1412389cc lea rcx, [rip + 0x19e621d]
1412389d3 mov edx, 0x47
1412389d8 mov r9d, 0x47
1412389de call 0x1428d9940
1412389e3 jne 0x1412389c0
1412389e5 mov r14d, 0x45
1412389eb sub r14, rdx
1412389ee lea r15, [rip + 0x19e61fb]
1412389f5 add r15, r8
1412389f8 lea r8, [rip + 0x19e6350]
1412389ff lea r12, [rip + 0x19e6302]
141238a06 nop word ptr cs:[rax + rax]
141238a10 movsx eax, byte ptr [r8 - 1]
141238a15 test eax, eax
141238a17 js 0x141238a30
141238a19 dec r8
141238a1c cmp eax, 0x5c
141238a1f jne 0x141238a84
141238a21 jmp 0x141238a96
141238a23 nop word ptr cs:[rax + rax]
141238a30 movzx ecx, byte ptr [r8 - 2]
141238a35 cmp cl, 0xc0
141238a38 jge 0x141238a5d
141238a3a movzx edx, byte ptr [r8 - 3]
141238a3f cmp dl, 0xc0
141238a42 jge 0x141238a66
141238a44 movzx r9d, byte ptr [r8 - 4]
141238a49 add r8, -4
141238a4d and r9d, 7
141238a51 shl r9d, 6
141238a55 and edx, 0x3f
141238a58 or edx, r9d
141238a5b jmp 0x141238a6d
141238a5d add r8, -2
141238a61 and ecx, 0x1f
141238a64 jmp 0x141238a75
141238a66 add r8, -3
141238a6a and edx, 0xf
141238a6d shl edx, 6
141238a70 and ecx, 0x3f
141238a73 or ecx, edx
141238a75 shl ecx, 6
141238a78 and al, 0x3f
141238a7a movzx eax, al
141238a7d or eax, ecx
141238a7f cmp eax, 0x5c
141238a82 je 0x141238a96
141238a84 cmp eax, 0x2f
141238a87 je 0x141238a96
141238a89 cmp r8, r12
141238a8c jne 0x141238a10
141238a8e mov r13d, 0x47
141238a94 jmp 0x141238ae3
141238a96 lea r12, [rip + 0x19e626b]
141238a9d sub r8, r12
141238aa0 inc r8
141238aa3 je 0x141238ad7
141238aa5 cmp r8, 0x47
141238aa9 jae 0x141238ad5
141238aab cmp byte ptr [r8 + r12], 0xbf
141238ab0 jg 0x141238ad7
141238ab2 lea rax, [rip + 0x19e61e7]
141238ab9 mov qword ptr [rsp + 0x20], rax
141238abe lea rcx, [rip + 0x19e6243]
141238ac5 mov edx, 0x47
141238aca mov r9d, 0x47
141238ad0 call 0x1428d9940
141238ad5 jne 0x141238ab2
141238ad7 mov r13d, 0x47
141238add sub r13, r8
141238ae0 add r12, r8
141238ae3 call 0x140c18130
141238ae8 mov rcx, rax
141238aeb mov rax, qword ptr [rax]
141238aee cmp rax, 1
141238af2 jne 0x141238c49
141238af8 add rcx, 8
141238afc cmp qword ptr [rcx], 0
141238b00 jne 0x141238c83
141238b06 mov qword ptr [rcx], 0xffffffffffffffff
141238b0d mov qword ptr [rbp + 0x70], rcx
141238b11 add rcx, 8
141238b15 mov qword ptr [rsp + 0x20], r13
141238b1a mov qword ptr [rsp + 0x30], 0
141238b23 mov qword ptr [rsp + 0x28], 1
141238b2c mov rdx, r15
141238b2f mov r8, r14
141238b32 mov r9, r12
141238b35 call 0x140c16d40
141238b3a mov r14, rax
141238b3d mov rax, qword ptr [rbp + 0x70]
141238b41 inc qword ptr [rax]
141238b44 mov qword ptr [rbp + 0x20], r14
141238b48 mov eax, 1
141238b4d mov qword ptr [rbp + 0x18], rax
141238b51 xorps xmm0, xmm0
141238b54 movaps xmmword ptr [rbp + 0x40], xmm0
141238b58 mov qword ptr [rbp + 0x50], 0
141238b60 lea rax, [rbp + 0x50]
141238b64 mov qword ptr [rbp + 0x58], 0
141238b6c mov qword ptr [rbp + 0x60], 8
141238b74 mov qword ptr [rbp + 0x68], 0
141238b7c movups xmm0, xmmword ptr [rbx + 0x90]
141238b83 mov rcx, qword ptr [rbx + 0xc8]
141238b8a mov rdx, qword ptr [rbx + 0xd0]
141238b91 mov dword ptr [rbp - 0x44], 0
141238b98 lea r8, [rbp + 0x40]
141238b9c mov qword ptr [rbp - 0x30], r8
141238ba0 mov qword ptr [rbp - 0x28], rax
141238ba4 lea rax, [rbp + 0x38]
141238ba8 mov qword ptr [rbp - 0x20], rax
141238bac mov qword ptr [rbp - 0x18], rbx
141238bb0 movups xmmword ptr [rbp - 0x10], xmm0
141238bb4 lea rax, [rbp - 0x44]
141238bb8 mov qword ptr [rbp], rax
141238bbc mov qword ptr [rbp + 8], rdi
141238bc0 lea rax, [rbp + 0x58]
141238bc4 mov qword ptr [rbp + 0x10], rax
141238bc8 lea r9, [rbp - 0x30]
141238bcc xorps xmm2, xmm2
141238bcf call 0x1411fff30
141238bd4 mov rax, qword ptr [rbp + 0x50]
141238bd8 mov qword ptr [rsi + 0x10], rax
141238bdc movaps xmm0, xmmword ptr [rbp + 0x40]
141238be0 movups xmmword ptr [rsi], xmm0
141238be3 movups xmm0, xmmword ptr [rbp + 0x58]
141238be7 movups xmmword ptr [rsi + 0x18], xmm0
141238beb mov rax, qword ptr [rbp + 0x68]
141238bef mov qword ptr [rsi + 0x28], rax
141238bf3 cmp byte ptr [rbp + 0x78], 0
141238bf7 je 0x141238c32
141238bf9 call 0x140c18130
141238bfe mov rcx, rax
141238c01 mov rax, qword ptr [rax]
141238c04 cmp rax, 1
141238c08 jne 0x141238c63
141238c0a add rcx, 8
141238c0e cmp qword ptr [rcx], 0
141238c12 jne 0x141238c83
141238c14 mov qword ptr [rcx], 0xffffffffffffffff
141238c1b mov qword ptr [rbp + 0x78], rcx
141238c1f add rcx, 8
141238c23 mov rdx, r14
141238c26 call 0x140c170f0
141238c2b mov rax, qword ptr [rbp + 0x78]
141238c2f inc qword ptr [rax]
141238c32 mov rax, rsi
141238c35 add rsp, 0x108
141238c3c pop rbx
141238c3d pop rdi
141238c3e pop rsi
141238c3f pop r12
141238c41 pop r13
141238c43 pop r14
141238c45 pop r15
141238c47 pop rbp
141238c48 ret 
141238c49 test rax, rax
141238c4c jne 0x141238c77
141238c4e xor edx, edx
141238c50 call 0x1428f15c0
141238c55 mov rcx, rax
141238c58 test rax, rax
141238c5b jne 0x141238afc
141238c61 jmp 0x141238c77
141238c63 test rax, rax
141238c66 jne 0x141238c77
141238c68 xor edx, edx
141238c6a call 0x1428f15c0
141238c6f mov rcx, rax
141238c72 test rax, rax
141238c75 jne 0x141238c0e
141238c77 lea rcx, [rip + 0x19e082a]
141238c7e call 0x1428d80d0
141238c83 lea rcx, [rip + 0x19e088e]
141238c8a call 0x1428d9250
141238c8f int3 
