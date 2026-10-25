0x140f98e70 push rbp
0x140f98e71 push rsi
0x140f98e72 push rdi
0x140f98e73 push rbx
0x140f98e74 sub rsp, 0x238
0x140f98e7b lea rbp, [rsp + 0x80]
0x140f98e83 mov qword ptr [rbp + 0x1b0], 0xfffffffffffffffe
0x140f98e8e mov rsi, rcx
0x140f98e91 movzx eax, byte ptr [rip + 0x269bd71]
0x140f98e98 mov ecx, 0xe0
0x140f98e9d mov edx, 8
0x140f98ea2 call 0x140613c10
0x140f98ea7 test rax, rax
0x140f98eaa je 0x140f9913f
0x140f98eb0 mov qword ptr [rbp + 0x1a8], rax
0x140f98eb7 xorps xmm0, xmm0
0x140f98eba movups xmmword ptr [rbp + 0x88], xmm0
0x140f98ec1 movups xmmword ptr [rbp + 0xa0], xmm0
0x140f98ec8 movups xmmword ptr [rbp + 0xb8], xmm0
0x140f98ecf movups xmmword ptr [rbp + 0xd0], xmm0
0x140f98ed6 movups xmmword ptr [rbp + 0xe8], xmm0
0x140f98edd mov dword ptr [rbp + 0xf8], 0
0x140f98ee7 movups xmmword ptr [rbp + 0x120], xmm0
0x140f98eee movups xmmword ptr [rbp + 0x138], xmm0
0x140f98ef5 movups xmmword ptr [rbp + 0x150], xmm0
0x140f98efc movups xmmword ptr [rbp + 0x168], xmm0
0x140f98f03 movups xmmword ptr [rbp + 0x180], xmm0
0x140f98f0a mov dword ptr [rbp + 0x190], 0
0x140f98f14 mov qword ptr [rbp + 0x10], 0
0x140f98f1c mov qword ptr [rbp + 0x68], 0
0x140f98f24 mov qword ptr [rbp + 0x70], 8
0x140f98f2c mov qword ptr [rbp + 0x78], 0
0x140f98f34 mov qword ptr [rbp + 0x80], 0x10
0x140f98f3f mov qword ptr [rbp + 0x98], 0x10
0x140f98f4a mov qword ptr [rbp + 0xb0], 0x10
0x140f98f55 mov qword ptr [rbp + 0xc8], 0x10
0x140f98f60 mov qword ptr [rbp + 0xe0], 0x10
0x140f98f6b movabs rbx, 0x8000000000000000
0x140f98f75 mov qword ptr [rbp + 0x100], rbx
0x140f98f7c lea rax, [rip + 0x1c15ffa]
0x140f98f83 mov qword ptr [rbp + 0x108], rax
0x140f98f8a mov qword ptr [rbp + 0x110], 0x23
0x140f98f95 mov qword ptr [rbp + 0x118], 0x10
0x140f98fa0 mov qword ptr [rbp + 0x130], 0x10
0x140f98fab mov qword ptr [rbp + 0x148], 0x10
0x140f98fb6 mov qword ptr [rbp + 0x160], 0x10
0x140f98fc1 mov qword ptr [rbp + 0x178], 0x10
0x140f98fcc mov dword ptr [rbp + 0x198], 0
0x140f98fd6 mov word ptr [rbp + 0x19c], 1
0x140f98fdf mov dword ptr [rbp + 0x1a0], 0
0x140f98fe9 movzx eax, byte ptr [rip + 0x269bc19]
0x140f98ff0 mov ecx, 0x198
0x140f98ff5 mov edx, 8
0x140f98ffa call 0x140613c10
0x140f98fff test rax, rax
0x140f99002 je 0x140f9914e
0x140f99008 mov rdi, rax
0x140f9900b lea rdx, [rbp + 0x10]
0x140f9900f mov r8d, 0x198
0x140f99015 mov rcx, rax
0x140f99018 call 0x142923c00
0x140f9901d lea r8, [rip + 0x1c15a9c]
0x140f99024 lea rcx, [rbp - 0x60]
0x140f99028 mov rdx, rdi
0x140f9902b call 0x140ce4b10
0x140f99030 lea rcx, [rbp + 0x10]
0x140f99034 call 0x140fed480
0x140f99039 movups xmm0, xmmword ptr [rbp]
0x140f9903d mov rax, qword ptr [rbp + 0x1a8]
0x140f99044 movups xmmword ptr [rax + 0x60], xmm0
0x140f99048 movups xmm0, xmmword ptr [rbp - 0x10]
0x140f9904c movups xmmword ptr [rax + 0x50], xmm0
0x140f99050 movups xmm0, xmmword ptr [rbp - 0x20]
0x140f99054 movups xmmword ptr [rax + 0x40], xmm0
0x140f99058 movups xmm0, xmmword ptr [rbp - 0x60]
0x140f9905c movups xmm1, xmmword ptr [rbp - 0x50]
0x140f99060 movups xmm2, xmmword ptr [rbp - 0x40]
0x140f99064 movups xmm3, xmmword ptr [rbp - 0x30]
0x140f99068 movups xmmword ptr [rax + 0x30], xmm3
0x140f9906c movups xmmword ptr [rax + 0x20], xmm2
0x140f99070 movups xmmword ptr [rax + 0x10], xmm1
0x140f99074 movups xmmword ptr [rax], xmm0
0x140f99077 movups xmm0, xmmword ptr [rbp + 0x10]
0x140f9907b movups xmm1, xmmword ptr [rbp + 0x20]
0x140f9907f movups xmm2, xmmword ptr [rbp + 0x30]
0x140f99083 movups xmm3, xmmword ptr [rbp + 0x40]
0x140f99087 movups xmmword ptr [rax + 0x70], xmm0
0x140f9908b movups xmmword ptr [rax + 0x80], xmm1
0x140f99092 movups xmmword ptr [rax + 0x90], xmm2
0x140f99099 movups xmmword ptr [rax + 0xa0], xmm3
0x140f990a0 movups xmm0, xmmword ptr [rbp + 0x50]
0x140f990a4 movups xmmword ptr [rax + 0xb0], xmm0
0x140f990ab movups xmm0, xmmword ptr [rbp + 0x60]
0x140f990af movups xmmword ptr [rax + 0xc0], xmm0
0x140f990b6 movups xmm0, xmmword ptr [rbp + 0x70]
0x140f990ba movups xmmword ptr [rax + 0xd0], xmm0
0x140f990c1 mov qword ptr [rbp + 0x18], 2
0x140f990c9 mov qword ptr [rbp + 0x20], rax
0x140f990cd mov qword ptr [rbp + 0x28], 2
0x140f990d5 mov qword ptr [rbp + 0x30], 0
0x140f990dd mov qword ptr [rbp + 0x38], 8
0x140f990e5 xorps xmm0, xmm0
0x140f990e8 movups xmmword ptr [rbp + 0x40], xmm0
0x140f990ec mov qword ptr [rbp + 0x10], rbx
0x140f990f0 lea rcx, [rbp + 0x48]
0x140f990f4 call 0x140d2fa20
0x140f990f9 movups xmm0, xmmword ptr [rbp + 0x70]
0x140f990fd movups xmmword ptr [rsi + 0x60], xmm0
0x140f99101 movups xmm0, xmmword ptr [rbp + 0x60]
0x140f99105 movups xmmword ptr [rsi + 0x50], xmm0
0x140f99109 movups xmm0, xmmword ptr [rbp + 0x50]
0x140f9910d movups xmmword ptr [rsi + 0x40], xmm0
0x140f99111 movups xmm0, xmmword ptr [rbp + 0x10]
0x140f99115 movups xmm1, xmmword ptr [rbp + 0x20]
0x140f99119 movups xmm2, xmmword ptr [rbp + 0x30]
0x140f9911d movups xmm3, xmmword ptr [rbp + 0x40]
0x140f99121 movups xmmword ptr [rsi + 0x30], xmm3
0x140f99125 movups xmmword ptr [rsi + 0x20], xmm2
0x140f99129 movups xmmword ptr [rsi + 0x10], xmm1
0x140f9912d movups xmmword ptr [rsi], xmm0
0x140f99130 mov rax, rsi
0x140f99133 add rsp, 0x238
0x140f9913a pop rbx
0x140f9913b pop rdi
0x140f9913c pop rsi
0x140f9913d pop rbp
0x140f9913e ret 
0x140f9913f mov ecx, 8
0x140f99144 mov edx, 0xe0
0x140f99149 call 0x1428d9000
0x140f9914e mov ecx, 8
0x140f99153 mov edx, 0x198
0x140f99158 call 0x1428d9000
