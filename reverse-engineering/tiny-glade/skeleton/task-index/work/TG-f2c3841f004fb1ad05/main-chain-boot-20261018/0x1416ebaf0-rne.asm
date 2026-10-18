0x1416ebaf0 push rbp
0x1416ebaf1 push rsi
0x1416ebaf2 push rdi
0x1416ebaf3 push rbx
0x1416ebaf4 sub rsp, 0xe8
0x1416ebafb lea rbp, [rsp + 0x80]
0x1416ebb03 mov qword ptr [rbp + 0x60], 0xfffffffffffffffe
0x1416ebb0b mov rsi, rcx
0x1416ebb0e movzx eax, byte ptr [rdx]
0x1416ebb11 lea rcx, [rip + 0x15a42e8]
0x1416ebb18 movsxd rax, dword ptr [rcx + rax*4]
0x1416ebb1c add rax, rcx
0x1416ebb1f jmp rax
0x1416ebb21 mov al, 0x12
0x1416ebb23 jmp 0x1416ebbcb
0x1416ebb28 mov rax, qword ptr [rdx + 0x18]
0x1416ebb2c mov qword ptr [rbp - 0x20], rax
0x1416ebb30 movups xmm0, xmmword ptr [rdx + 8]
0x1416ebb34 movaps xmmword ptr [rbp - 0x30], xmm0
0x1416ebb38 lea rcx, [rbp + 0x38]
0x1416ebb3c lea rdx, [rbp - 0x30]
0x1416ebb40 call 0x1415b5140
0x1416ebb45 movzx eax, byte ptr [rbp + 0x38]
0x1416ebb49 cmp al, 0x16
0x1416ebb4b je 0x1416ebbc2
0x1416ebb4d jmp 0x1416ebbcb
0x1416ebb4f mov ecx, dword ptr [rdx + 8]
0x1416ebb52 mov rax, qword ptr [rdx + 0x10]
0x1416ebb56 mov qword ptr [rbp + 0x40], rax
0x1416ebb5a shl ecx, 3
0x1416ebb5d mov eax, 0xa0804
0x1416ebb62 shr eax, cl
0x1416ebb64 jmp 0x1416ebbcb
0x1416ebb66 movups xmm0, xmmword ptr [rdx + 8]
0x1416ebb6a movups xmmword ptr [rbp - 9], xmm0
0x1416ebb6e mov rax, qword ptr [rdx + 0x18]
0x1416ebb72 mov qword ptr [rbp + 7], rax
0x1416ebb76 movups xmm0, xmmword ptr [rbp - 0x10]
0x1416ebb7a movups xmmword ptr [rbp + 0x39], xmm0
0x1416ebb7e mov rax, qword ptr [rbp - 1]
0x1416ebb82 mov qword ptr [rbp + 0x48], rax
0x1416ebb86 mov rax, qword ptr [rbp + 7]
0x1416ebb8a mov qword ptr [rbp + 0x50], rax
0x1416ebb8e mov al, 0xc
0x1416ebb90 jmp 0x1416ebbcb
0x1416ebb92 movzx eax, byte ptr [rdx + 1]
0x1416ebb96 mov byte ptr [rbp + 0x39], al
0x1416ebb99 xor eax, eax
0x1416ebb9b jmp 0x1416ebbcb
0x1416ebb9d mov rax, qword ptr [rdx + 0x18]
0x1416ebba1 mov qword ptr [rbp - 0x40], rax
0x1416ebba5 movups xmm0, xmmword ptr [rdx + 8]
0x1416ebba9 movaps xmmword ptr [rbp - 0x50], xmm0
0x1416ebbad lea rcx, [rbp + 0x38]
0x1416ebbb1 lea rdx, [rbp - 0x50]
0x1416ebbb5 call 0x1415ba5b0
0x1416ebbba movzx eax, byte ptr [rbp + 0x38]
0x1416ebbbe cmp al, 0x16
0x1416ebbc0 jne 0x1416ebbcb
0x1416ebbc2 mov rdi, qword ptr [rbp + 0x40]
0x1416ebbc6 jmp 0x1416ebda3
0x1416ebbcb mov ecx, dword ptr [rbp + 0x39]
0x1416ebbce mov edx, dword ptr [rbp + 0x3c]
0x1416ebbd1 mov dword ptr [rbp + 0x1c], edx
0x1416ebbd4 mov dword ptr [rbp + 0x19], ecx
0x1416ebbd7 mov rcx, qword ptr [rbp + 0x40]
0x1416ebbdb movups xmm0, xmmword ptr [rbp + 0x48]
0x1416ebbdf movups xmmword ptr [rbp + 0x28], xmm0
0x1416ebbe3 mov byte ptr [rbp + 0x18], al
0x1416ebbe6 mov qword ptr [rbp + 0x20], rcx
0x1416ebbea lea rax, [rip + 0x15a3f2f]
0x1416ebbf1 mov qword ptr [rsp + 0x20], rax
0x1416ebbf6 mov qword ptr [rsp + 0x28], 3
0x1416ebbff lea r8, [rip + 0x15a3f0d]
0x1416ebc06 lea rcx, [rbp + 0x38]
0x1416ebc0a lea rdx, [rbp + 0x18]
0x1416ebc0e mov r9d, 0xc
0x1416ebc14 call 0x141282d20
0x1416ebc19 cmp dword ptr [rbp + 0x38], 1
0x1416ebc1d jne 0x1416ebc57
0x1416ebc1f mov rcx, qword ptr [rbp + 0x40]
0x1416ebc23 mov rax, qword ptr [rcx]
0x1416ebc26 cmp rax, 1
0x1416ebc2a mov qword ptr [rbp + 0x58], rcx
0x1416ebc2e je 0x1416ebc98
0x1416ebc30 test rax, rax
0x1416ebc33 jne 0x1416ebca1
0x1416ebc35 mov rax, qword ptr [rbp + 0x58]
0x1416ebc39 mov rdx, qword ptr [rax + 0x10]
0x1416ebc3d test rdx, rdx
0x1416ebc40 je 0x1416ebca1
0x1416ebc42 mov rax, qword ptr [rbp + 0x58]
0x1416ebc46 mov rcx, qword ptr [rax + 8]
0x1416ebc4a mov r8d, 1
0x1416ebc50 call 0x140613c20
0x1416ebc55 jmp 0x1416ebca1
0x1416ebc57 mov edi, dword ptr [rbp + 0x3c]
0x1416ebc5a mov rbx, qword ptr [rbp + 0x40]
0x1416ebc5e movups xmm0, xmmword ptr [rbp + 0x48]
0x1416ebc62 movaps xmmword ptr [rbp - 0x10], xmm0
0x1416ebc66 lea rcx, [rbp + 0x18]
0x1416ebc6a call 0x1416e72c0
0x1416ebc6f lea ecx, [rdi - 2]
0x1416ebc72 cmp ecx, 2
0x1416ebc75 mov eax, 2
0x1416ebc7a cmovae ecx, eax
0x1416ebc7d test ecx, ecx
0x1416ebc7f je 0x1416ebd62
0x1416ebc85 cmp ecx, 1
0x1416ebc88 jne 0x1416ebd54
0x1416ebc8e mov eax, 3
0x1416ebc93 jmp 0x1416ebd62
0x1416ebc98 add rcx, 8
0x1416ebc9c call 0x1405e7a00
0x1416ebca1 mov edx, 0x28
0x1416ebca6 mov r8d, 8
0x1416ebcac mov rcx, qword ptr [rbp + 0x58]
0x1416ebcb0 call 0x140613c20
0x1416ebcb5 lea rax, [rip + 0x15a3ee4]
0x1416ebcbc mov qword ptr [rsp + 0x20], rax
0x1416ebcc1 mov qword ptr [rsp + 0x28], 2
0x1416ebcca lea r8, [rip + 0x15a3eb7]
0x1416ebcd1 lea rcx, [rbp + 0x38]
0x1416ebcd5 lea rdx, [rbp + 0x18]
0x1416ebcd9 mov r9d, 0x12
0x1416ebcdf call 0x141282f00
0x1416ebce4 cmp dword ptr [rbp + 0x38], 1
0x1416ebce8 jne 0x1416ebd22
0x1416ebcea mov rcx, qword ptr [rbp + 0x40]
0x1416ebcee mov rax, qword ptr [rcx]
0x1416ebcf1 cmp rax, 1
0x1416ebcf5 mov qword ptr [rbp + 0x58], rcx
0x1416ebcf9 je 0x1416ebd69
0x1416ebcfb test rax, rax
0x1416ebcfe jne 0x1416ebd72
0x1416ebd00 mov rax, qword ptr [rbp + 0x58]
0x1416ebd04 mov rdx, qword ptr [rax + 0x10]
0x1416ebd08 test rdx, rdx
0x1416ebd0b je 0x1416ebd72
0x1416ebd0d mov rax, qword ptr [rbp + 0x58]
0x1416ebd11 mov rcx, qword ptr [rax + 8]
0x1416ebd15 mov r8d, 1
0x1416ebd1b call 0x140613c20
0x1416ebd20 jmp 0x1416ebd72
0x1416ebd22 mov edi, dword ptr [rbp + 0x3c]
0x1416ebd25 mov rbx, qword ptr [rbp + 0x40]
0x1416ebd29 movups xmm0, xmmword ptr [rbp + 0x48]
0x1416ebd2d movaps xmmword ptr [rbp - 0x10], xmm0
0x1416ebd31 lea rcx, [rbp + 0x18]
0x1416ebd35 call 0x1416e72c0
0x1416ebd3a mov eax, 2
0x1416ebd3f cmp edi, 2
0x1416ebd42 je 0x1416ebd62
0x1416ebd44 mov qword ptr [rsi + 8], rbx
0x1416ebd48 movaps xmm0, xmmword ptr [rbp - 0x10]
0x1416ebd4c movups xmmword ptr [rsi + 0x10], xmm0
0x1416ebd50 mov eax, edi
0x1416ebd52 jmp 0x1416ebd62
0x1416ebd54 mov qword ptr [rsi + 8], rbx
0x1416ebd58 movaps xmm0, xmmword ptr [rbp - 0x10]
0x1416ebd5c movups xmmword ptr [rsi + 0x10], xmm0
0x1416ebd60 mov eax, edi
0x1416ebd62 mov dword ptr [rsi + 4], eax
0x1416ebd65 xor eax, eax
0x1416ebd67 jmp 0x1416ebdac
0x1416ebd69 add rcx, 8
0x1416ebd6d call 0x1405e7a00
0x1416ebd72 mov edx, 0x28
0x1416ebd77 mov r8d, 8
0x1416ebd7d mov rcx, qword ptr [rbp + 0x58]
0x1416ebd81 call 0x140613c20
0x1416ebd86 lea rcx, [rip + 0x15a3e5b]
0x1416ebd8d mov edx, 0x44
0x1416ebd92 call 0x1428fa000
0x1416ebd97 mov rdi, rax
0x1416ebd9a lea rcx, [rbp + 0x18]
0x1416ebd9e call 0x1416e72c0
0x1416ebda3 mov qword ptr [rsi + 8], rdi
0x1416ebda7 mov eax, 1
0x1416ebdac mov dword ptr [rsi], eax
0x1416ebdae mov rax, rsi
0x1416ebdb1 add rsp, 0xe8
0x1416ebdb8 pop rbx
0x1416ebdb9 pop rdi
0x1416ebdba pop rsi
0x1416ebdbb pop rbp
0x1416ebdbc ret 
0x1416ebdbd nop dword ptr [rax]
0x1416ebdc0 mov qword ptr [rsp + 0x10], rdx
0x1416ebdc5 push rbp
0x1416ebdc6 push rsi
0x1416ebdc7 push rdi
0x1416ebdc8 push rbx
0x1416ebdc9 sub rsp, 0x38
0x1416ebdcd lea rbp, [rdx + 0x80]
0x1416ebdd4 lea rcx, [rbp + 0x18]
0x1416ebdd8 call 0x1416e72c0
0x1416ebddd nop 
0x1416ebdde add rsp, 0x38
0x1416ebde2 pop rbx
0x1416ebde3 pop rdi
0x1416ebde4 pop rsi
0x1416ebde5 pop rbp
0x1416ebde6 ret 
0x1416ebde7 nop word ptr [rax + rax]
0x1416ebdf0 mov qword ptr [rsp + 0x10], rdx
0x1416ebdf5 push rbp
0x1416ebdf6 push rsi
0x1416ebdf7 push rdi
0x1416ebdf8 push rbx
0x1416ebdf9 sub rsp, 0x38
0x1416ebdfd lea rbp, [rdx + 0x80]
0x1416ebe04 mov edx, 0x28
0x1416ebe09 mov r8d, 8
0x1416ebe0f mov rcx, qword ptr [rbp + 0x58]
0x1416ebe13 call 0x140613c20
0x1416ebe18 nop 
0x1416ebe19 add rsp, 0x38
0x1416ebe1d pop rbx
0x1416ebe1e pop rdi
0x1416ebe1f pop rsi
0x1416ebe20 pop rbp
0x1416ebe21 ret 
0x1416ebe22 nop word ptr cs:[rax + rax]
0x1416ebe30 mov qword ptr [rsp + 0x10], rdx
0x1416ebe35 push rbp
0x1416ebe36 push rsi
0x1416ebe37 push rdi
0x1416ebe38 push rbx
0x1416ebe39 sub rsp, 0x38
0x1416ebe3d lea rbp, [rdx + 0x80]
0x1416ebe44 mov edx, 0x28
0x1416ebe49 mov r8d, 8
0x1416ebe4f mov rcx, qword ptr [rbp + 0x58]
0x1416ebe53 call 0x140613c20
0x1416ebe58 nop 
0x1416ebe59 add rsp, 0x38
0x1416ebe5d pop rbx
0x1416ebe5e pop rdi
0x1416ebe5f pop rsi
0x1416ebe60 pop rbp
0x1416ebe61 ret 
