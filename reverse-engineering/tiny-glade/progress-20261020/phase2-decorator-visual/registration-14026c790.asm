14026c790 push rbp
14026c791 push rsi
14026c792 push rdi
14026c793 push rbx
14026c794 sub rsp, 0x7c8
14026c79b lea rbp, [rsp + 0x80]
14026c7a3 mov qword ptr [rbp + 0x740], 0xfffffffffffffffe
14026c7ae mov rsi, rcx
14026c7b1 movups xmm0, xmmword ptr [rdx + 0x60]
14026c7b5 movaps xmmword ptr [rbp + 0x720], xmm0
14026c7bc movups xmm0, xmmword ptr [rdx + 0x50]
14026c7c0 movaps xmmword ptr [rbp + 0x710], xmm0
14026c7c7 movups xmm0, xmmword ptr [rdx + 0x40]
14026c7cb movaps xmmword ptr [rbp + 0x700], xmm0
14026c7d2 movups xmm0, xmmword ptr [rdx]
14026c7d5 movups xmm1, xmmword ptr [rdx + 0x10]
14026c7d9 movups xmm2, xmmword ptr [rdx + 0x20]
14026c7dd movups xmm3, xmmword ptr [rdx + 0x30]
14026c7e1 movaps xmmword ptr [rbp + 0x6f0], xmm3
14026c7e8 movaps xmmword ptr [rbp + 0x6e0], xmm2
14026c7ef movaps xmmword ptr [rbp + 0x6d0], xmm1
14026c7f6 movaps xmmword ptr [rbp + 0x6c0], xmm0
14026c7fd movups xmm0, xmmword ptr [rdx + 0xd0]
14026c804 movaps xmmword ptr [rbp + 0x6b0], xmm0
14026c80b movups xmm0, xmmword ptr [rdx + 0xc0]
14026c812 movaps xmmword ptr [rbp + 0x6a0], xmm0
14026c819 movups xmm0, xmmword ptr [rdx + 0xb0]
14026c820 movaps xmmword ptr [rbp + 0x690], xmm0
14026c827 movups xmm0, xmmword ptr [rdx + 0x70]
14026c82b movups xmm1, xmmword ptr [rdx + 0x80]
14026c832 movups xmm2, xmmword ptr [rdx + 0x90]
14026c839 movups xmm3, xmmword ptr [rdx + 0xa0]
14026c840 movaps xmmword ptr [rbp + 0x680], xmm3
14026c847 movaps xmmword ptr [rbp + 0x670], xmm2
14026c84e movaps xmmword ptr [rbp + 0x660], xmm1
14026c855 movaps xmmword ptr [rbp + 0x650], xmm0
14026c85c movzx eax, byte ptr [rip + 0x33c83a6]
14026c863 mov ecx, 0x1c0
14026c868 mov edx, 8
14026c86d call 0x140613c10
14026c872 test rax, rax
14026c875 je 0x14026cc7a
14026c87b mov qword ptr [rbp + 0x730], rax
14026c882 xorps xmm0, xmm0
14026c885 movups xmmword ptr [rbp - 0x38], xmm0
14026c889 movups xmmword ptr [rbp - 0x20], xmm0
14026c88d movups xmmword ptr [rbp - 8], xmm0
14026c891 movups xmmword ptr [rbp + 0x10], xmm0
14026c895 movups xmmword ptr [rbp + 0x28], xmm0
14026c899 mov dword ptr [rbp + 0x38], 0
14026c8a0 movups xmmword ptr [rbp + 0x60], xmm0
14026c8a4 movups xmmword ptr [rbp + 0x78], xmm0
14026c8a8 movups xmmword ptr [rbp + 0x90], xmm0
14026c8af movups xmmword ptr [rbp + 0xa8], xmm0
14026c8b6 movups xmmword ptr [rbp + 0xc0], xmm0
14026c8bd mov dword ptr [rbp + 0xd0], 0
14026c8c7 mov qword ptr [rbp - 0x58], 0
14026c8cf mov qword ptr [rbp - 0x50], 8
14026c8d7 mov qword ptr [rbp - 0x48], 0
14026c8df mov qword ptr [rbp - 0x40], 0x10
14026c8e7 mov qword ptr [rbp - 0x28], 0x10
14026c8ef mov qword ptr [rbp - 0x10], 0x10
14026c8f7 mov qword ptr [rbp + 8], 0x10
14026c8ff mov qword ptr [rbp + 0x20], 0x10
14026c907 movabs rbx, 0x8000000000000000
14026c911 mov qword ptr [rbp + 0x40], rbx
14026c915 lea rax, [rip + 0x273acc9]
14026c91c mov qword ptr [rbp + 0x48], rax
14026c920 mov qword ptr [rbp + 0x50], 0x58
14026c928 mov qword ptr [rbp + 0x58], 0x10
14026c930 mov qword ptr [rbp + 0x70], 0x10
14026c938 mov qword ptr [rbp + 0x88], 0x10
14026c943 mov qword ptr [rbp + 0xa0], 0x10
14026c94e mov qword ptr [rbp + 0xb8], 0x10
14026c959 mov dword ptr [rbp + 0xd8], 0
14026c963 mov word ptr [rbp + 0xdc], 1
14026c96c mov qword ptr [rbp + 0x1d0], rbx
14026c973 mov dword ptr [rbp + 0x4f8], 0
14026c97d movzx eax, byte ptr [rip + 0x33c8285]
14026c984 mov ecx, 0x558
14026c989 mov edx, 8
14026c98e call 0x140613c10
14026c993 test rax, rax
14026c996 je 0x14026cc92
14026c99c mov rdi, rax
14026c99f lea rdx, [rbp - 0x58]
14026c9a3 mov r8d, 0x558
14026c9a9 mov rcx, rax
14026c9ac call 0x142923c00
14026c9b1 mov byte ptr [rbp + 0x73f], 1
14026c9b8 lea r8, [rip + 0x273ab51]
14026c9bf lea rcx, [rbp + 0x500]
14026c9c6 mov rdx, rdi
14026c9c9 call 0x140ce4b10
14026c9ce movaps xmm0, xmmword ptr [rbp + 0x720]
14026c9d5 movaps xmmword ptr [rbp + 0x640], xmm0
14026c9dc movaps xmm0, xmmword ptr [rbp + 0x710]
14026c9e3 movaps xmmword ptr [rbp + 0x630], xmm0
14026c9ea movaps xmm0, xmmword ptr [rbp + 0x700]
14026c9f1 movaps xmmword ptr [rbp + 0x620], xmm0
14026c9f8 movaps xmm0, xmmword ptr [rbp + 0x6c0]
14026c9ff movaps xmm1, xmmword ptr [rbp + 0x6d0]
14026ca06 movaps xmm2, xmmword ptr [rbp + 0x6e0]
14026ca0d movaps xmm3, xmmword ptr [rbp + 0x6f0]
14026ca14 movaps xmmword ptr [rbp + 0x610], xmm3
14026ca1b movaps xmmword ptr [rbp + 0x600], xmm2
14026ca22 movaps xmmword ptr [rbp + 0x5f0], xmm1
14026ca29 movaps xmmword ptr [rbp + 0x5e0], xmm0
14026ca30 movaps xmm0, xmmword ptr [rbp + 0x6b0]
14026ca37 movaps xmmword ptr [rbp + 0x5d0], xmm0
14026ca3e movaps xmm0, xmmword ptr [rbp + 0x6a0]
14026ca45 movaps xmmword ptr [rbp + 0x5c0], xmm0
14026ca4c movaps xmm0, xmmword ptr [rbp + 0x690]
14026ca53 movaps xmmword ptr [rbp + 0x5b0], xmm0
14026ca5a movaps xmm0, xmmword ptr [rbp + 0x650]
14026ca61 movaps xmm1, xmmword ptr [rbp + 0x660]
14026ca68 movaps xmm2, xmmword ptr [rbp + 0x670]
14026ca6f movaps xmm3, xmmword ptr [rbp + 0x680]
14026ca76 movaps xmmword ptr [rbp + 0x5a0], xmm3
14026ca7d movaps xmmword ptr [rbp + 0x590], xmm2
14026ca84 movaps xmmword ptr [rbp + 0x580], xmm1
14026ca8b movaps xmmword ptr [rbp + 0x570], xmm0
14026ca92 lea rcx, [rbp - 0x58]
14026ca96 call 0x1400b1230
14026ca9b movups xmm0, xmmword ptr [rbp + 0x560]
14026caa2 mov rax, qword ptr [rbp + 0x730]
14026caa9 movups xmmword ptr [rax + 0x60], xmm0
14026caad movups xmm0, xmmword ptr [rbp + 0x550]
14026cab4 movups xmmword ptr [rax + 0x50], xmm0
14026cab8 movups xmm0, xmmword ptr [rbp + 0x540]
14026cabf movups xmmword ptr [rax + 0x40], xmm0
14026cac3 movups xmm0, xmmword ptr [rbp + 0x500]
14026caca movups xmm1, xmmword ptr [rbp + 0x510]
14026cad1 movups xmm2, xmmword ptr [rbp + 0x520]
14026cad8 movups xmm3, xmmword ptr [rbp + 0x530]
14026cadf movups xmmword ptr [rax + 0x30], xmm3
14026cae3 movups xmmword ptr [rax + 0x20], xmm2
14026cae7 movups xmmword ptr [rax + 0x10], xmm1
14026caeb movups xmmword ptr [rax], xmm0
14026caee movaps xmm0, xmmword ptr [rbp + 0x5e0]
14026caf5 movaps xmm1, xmmword ptr [rbp + 0x5f0]
14026cafc movaps xmm2, xmmword ptr [rbp + 0x600]
14026cb03 movaps xmm3, xmmword ptr [rbp + 0x610]
14026cb0a movups xmmword ptr [rax + 0x70], xmm0
14026cb0e movups xmmword ptr [rax + 0x80], xmm1
14026cb15 movups xmmword ptr [rax + 0x90], xmm2
14026cb1c movups xmmword ptr [rax + 0xa0], xmm3
14026cb23 movaps xmm0, xmmword ptr [rbp + 0x620]
14026cb2a movups xmmword ptr [rax + 0xb0], xmm0
14026cb31 movaps xmm0, xmmword ptr [rbp + 0x630]
14026cb38 movups xmmword ptr [rax + 0xc0], xmm0
14026cb3f movaps xmm0, xmmword ptr [rbp + 0x640]
14026cb46 movups xmmword ptr [rax + 0xd0], xmm0
14026cb4d movaps xmm0, xmmword ptr [rbp + 0x5d0]
14026cb54 movups xmmword ptr [rax + 0x140], xmm0
14026cb5b movaps xmm0, xmmword ptr [rbp + 0x5c0]
14026cb62 movups xmmword ptr [rax + 0x130], xmm0
14026cb69 movaps xmm0, xmmword ptr [rbp + 0x5b0]
14026cb70 movups xmmword ptr [rax + 0x120], xmm0
14026cb77 movaps xmm0, xmmword ptr [rbp + 0x570]
14026cb7e movaps xmm1, xmmword ptr [rbp + 0x580]
14026cb85 movaps xmm2, xmmword ptr [rbp + 0x590]
14026cb8c movaps xmm3, xmmword ptr [rbp + 0x5a0]
14026cb93 movups xmmword ptr [rax + 0x110], xmm3
14026cb9a movups xmmword ptr [rax + 0x100], xmm2
14026cba1 movups xmmword ptr [rax + 0xf0], xmm1
14026cba8 movups xmmword ptr [rax + 0xe0], xmm0
14026cbaf movups xmm0, xmmword ptr [rbp - 0x58]
14026cbb3 movups xmm1, xmmword ptr [rbp - 0x48]
14026cbb7 movups xmm2, xmmword ptr [rbp - 0x38]
14026cbbb movups xmm3, xmmword ptr [rbp - 0x28]
14026cbbf movups xmmword ptr [rax + 0x150], xmm0
14026cbc6 movups xmmword ptr [rax + 0x160], xmm1
14026cbcd movups xmmword ptr [rax + 0x170], xmm2
14026cbd4 movups xmmword ptr [rax + 0x180], xmm3
14026cbdb movups xmm0, xmmword ptr [rbp - 0x18]
14026cbdf movups xmmword ptr [rax + 0x190], xmm0
14026cbe6 movups xmm0, xmmword ptr [rbp - 8]
14026cbea movups xmmword ptr [rax + 0x1a0], xmm0
14026cbf1 movups xmm0, xmmword ptr [rbp + 8]
14026cbf5 movups xmmword ptr [rax + 0x1b0], xmm0
14026cbfc mov qword ptr [rbp - 0x50], 4
14026cc04 mov qword ptr [rbp - 0x48], rax
14026cc08 mov qword ptr [rbp - 0x40], 4
14026cc10 mov qword ptr [rbp - 0x38], 0
14026cc18 mov qword ptr [rbp - 0x30], 8
14026cc20 xorps xmm0, xmm0
14026cc23 movups xmmword ptr [rbp - 0x28], xmm0
14026cc27 mov qword ptr [rbp - 0x58], rbx
14026cc2b lea rcx, [rbp - 0x20]
14026cc2f call 0x140d2fa20
14026cc34 movups xmm0, xmmword ptr [rbp + 8]
14026cc38 movups xmmword ptr [rsi + 0x60], xmm0
14026cc3c movups xmm0, xmmword ptr [rbp - 8]
14026cc40 movups xmmword ptr [rsi + 0x50], xmm0
14026cc44 movups xmm0, xmmword ptr [rbp - 0x18]
14026cc48 movups xmmword ptr [rsi + 0x40], xmm0
14026cc4c movups xmm0, xmmword ptr [rbp - 0x58]
14026cc50 movups xmm1, xmmword ptr [rbp - 0x48]
14026cc54 movups xmm2, xmmword ptr [rbp - 0x38]
14026cc58 movups xmm3, xmmword ptr [rbp - 0x28]
14026cc5c movups xmmword ptr [rsi + 0x30], xmm3
14026cc60 movups xmmword ptr [rsi + 0x20], xmm2
14026cc64 movups xmmword ptr [rsi + 0x10], xmm1
14026cc68 movups xmmword ptr [rsi], xmm0
14026cc6b mov rax, rsi
14026cc6e add rsp, 0x7c8
14026cc75 pop rbx
14026cc76 pop rdi
14026cc77 pop rsi
14026cc78 pop rbp
14026cc79 ret 
14026cc7a mov byte ptr [rbp + 0x73e], 1
14026cc81 mov ecx, 8
14026cc86 mov edx, 0x1c0
14026cc8b call 0x1428d9000
14026cc90 jmp 0x14026cca1
14026cc92 mov ecx, 8
14026cc97 mov edx, 0x558
14026cc9c call 0x1428d9000
14026cca1 ud2 
14026cca3 nop word ptr cs:[rax + rax]
14026ccb0 mov qword ptr [rsp + 0x10], rdx
14026ccb5 push rbp
14026ccb6 push rsi
14026ccb7 push rdi
14026ccb8 push rbx
14026ccb9 sub rsp, 0x28
14026ccbd lea rbp, [rdx + 0x80]
14026ccc4 mov byte ptr [rbp + 0x73f], 1
14026cccb lea rcx, [rbp - 0x58]
14026cccf call 0x1405e57e0
14026ccd4 mov byte ptr [rbp + 0x73f], 1
14026ccdb add rsp, 0x28
14026ccdf pop rbx
14026cce0 pop rdi
14026cce1 pop rsi
14026cce2 pop rbp
14026cce3 ret 
14026cce4 nop word ptr cs:[rax + rax]
14026ccf0 mov qword ptr [rsp + 0x10], rdx
14026ccf5 push rbp
14026ccf6 push rsi
14026ccf7 push rdi
14026ccf8 push rbx
14026ccf9 sub rsp, 0x28
14026ccfd lea rbp, [rdx + 0x80]
14026cd04 movzx ebx, byte ptr [rbp + 0x73f]
14026cd0b mov edx, 0x1c0
14026cd10 mov r8d, 8
14026cd16 mov rcx, qword ptr [rbp + 0x730]
14026cd1d call 0x140613c20
14026cd22 mov byte ptr [rbp + 0x73e], bl
14026cd28 add rsp, 0x28
14026cd2c pop rbx
14026cd2d pop rdi
14026cd2e pop rsi
14026cd2f pop rbp
14026cd30 ret 
14026cd31 nop word ptr cs:[rax + rax]
14026cd40 mov qword ptr [rsp + 0x10], rdx
14026cd45 push rbp
14026cd46 push rsi
14026cd47 push rdi
14026cd48 push rbx
14026cd49 sub rsp, 0x28
14026cd4d lea rbp, [rdx + 0x80]
14026cd54 lea rcx, [rbp + 0x570]
14026cd5b call 0x140029880
14026cd60 lea rcx, [rbp + 0x5e0]
14026cd67 call 0x140029880
14026cd6c lea rcx, [rbp + 0x500]
14026cd73 call 0x140029880
14026cd78 mov byte ptr [rbp + 0x73f], 0
14026cd7f add rsp, 0x28
14026cd83 pop rbx
14026cd84 pop rdi
14026cd85 pop rsi
14026cd86 pop rbp
14026cd87 ret 
14026cd88 nop dword ptr [rax + rax]
14026cd90 mov qword ptr [rsp + 0x10], rdx
14026cd95 push rbp
14026cd96 push rsi
14026cd97 push rdi
14026cd98 push rbx
14026cd99 sub rsp, 0x28
14026cd9d lea rbp, [rdx + 0x80]
14026cda4 cmp byte ptr [rbp + 0x73e], 0
14026cdab jne 0x14026cdb6
14026cdad add rsp, 0x28
14026cdb1 pop rbx
14026cdb2 pop rdi
14026cdb3 pop rsi
14026cdb4 pop rbp
14026cdb5 ret 
14026cdb6 lea rcx, [rbp + 0x650]
14026cdbd call 0x140029880
14026cdc2 lea rcx, [rbp + 0x6c0]
14026cdc9 call 0x140029880
14026cdce jmp 0x14026cdad
14026cdd0 mov qword ptr [rsp + 0x10], rdx
14026cdd5 push rbp
14026cdd6 push rsi
14026cdd7 push rdi
14026cdd8 push rbx
14026cdd9 sub rsp, 0x28
14026cddd lea rbp, [rdx + 0x80]
14026cde4 lea rcx, [rbp - 0x58]
14026cde8 call 0x140029880
14026cded nop 
14026cdee add rsp, 0x28
14026cdf2 pop rbx
14026cdf3 pop rdi
14026cdf4 pop rsi
14026cdf5 pop rbp
14026cdf6 ret 