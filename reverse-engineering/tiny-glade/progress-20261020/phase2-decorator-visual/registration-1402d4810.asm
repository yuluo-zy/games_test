1402d4810 push rbp
1402d4811 push rsi
1402d4812 push rdi
1402d4813 push rbx
1402d4814 sub rsp, 0x238
1402d481b lea rbp, [rsp + 0x80]
1402d4823 mov qword ptr [rbp + 0x1b0], 0xfffffffffffffffe
1402d482e mov rsi, rcx
1402d4831 movzx eax, byte ptr [rip + 0x33603d1]
1402d4838 mov ecx, 0xe0
1402d483d mov edx, 8
1402d4842 call 0x140613c10
1402d4847 test rax, rax
1402d484a je 0x1402d4adf
1402d4850 mov qword ptr [rbp + 0x1a8], rax
1402d4857 xorps xmm0, xmm0
1402d485a movups xmmword ptr [rbp + 0x88], xmm0
1402d4861 movups xmmword ptr [rbp + 0xa0], xmm0
1402d4868 movups xmmword ptr [rbp + 0xb8], xmm0
1402d486f movups xmmword ptr [rbp + 0xd0], xmm0
1402d4876 movups xmmword ptr [rbp + 0xe8], xmm0
1402d487d mov dword ptr [rbp + 0xf8], 0
1402d4887 movups xmmword ptr [rbp + 0x120], xmm0
1402d488e movups xmmword ptr [rbp + 0x138], xmm0
1402d4895 movups xmmword ptr [rbp + 0x150], xmm0
1402d489c movups xmmword ptr [rbp + 0x168], xmm0
1402d48a3 movups xmmword ptr [rbp + 0x180], xmm0
1402d48aa mov dword ptr [rbp + 0x190], 0
1402d48b4 mov qword ptr [rbp + 0x18], 0
1402d48bc mov qword ptr [rbp + 0x68], 0
1402d48c4 mov qword ptr [rbp + 0x70], 8
1402d48cc mov qword ptr [rbp + 0x78], 0
1402d48d4 mov qword ptr [rbp + 0x80], 0x10
1402d48df mov qword ptr [rbp + 0x98], 0x10
1402d48ea mov qword ptr [rbp + 0xb0], 0x10
1402d48f5 mov qword ptr [rbp + 0xc8], 0x10
1402d4900 mov qword ptr [rbp + 0xe0], 0x10
1402d490b movabs rbx, 0x8000000000000000
1402d4915 mov qword ptr [rbp + 0x100], rbx
1402d491c lea rax, [rip + 0x26de155]
1402d4923 mov qword ptr [rbp + 0x108], rax
1402d492a mov qword ptr [rbp + 0x110], 0x4f
1402d4935 mov qword ptr [rbp + 0x118], 0x10
1402d4940 mov qword ptr [rbp + 0x130], 0x10
1402d494b mov qword ptr [rbp + 0x148], 0x10
1402d4956 mov qword ptr [rbp + 0x160], 0x10
1402d4961 mov qword ptr [rbp + 0x178], 0x10
1402d496c mov dword ptr [rbp + 0x198], 0
1402d4976 mov word ptr [rbp + 0x19c], 1
1402d497f mov dword ptr [rbp + 0x1a0], 0
1402d4989 movzx eax, byte ptr [rip + 0x3360279]
1402d4990 mov ecx, 0x190
1402d4995 mov edx, 8
1402d499a call 0x140613c10
1402d499f test rax, rax
1402d49a2 je 0x1402d4aee
1402d49a8 mov rdi, rax
1402d49ab lea rdx, [rbp + 0x18]
1402d49af mov r8d, 0x190
1402d49b5 mov rcx, rax
1402d49b8 call 0x142923c00
1402d49bd lea r8, [rip + 0x26ddf14]
1402d49c4 lea rcx, [rbp - 0x58]
1402d49c8 mov rdx, rdi
1402d49cb call 0x140ce4b10
1402d49d0 lea rcx, [rbp + 0x18]
1402d49d4 call 0x140526530
1402d49d9 movups xmm0, xmmword ptr [rbp + 8]
1402d49dd mov rax, qword ptr [rbp + 0x1a8]
1402d49e4 movups xmmword ptr [rax + 0x60], xmm0
1402d49e8 movups xmm0, xmmword ptr [rbp - 8]
1402d49ec movups xmmword ptr [rax + 0x50], xmm0
1402d49f0 movups xmm0, xmmword ptr [rbp - 0x18]
1402d49f4 movups xmmword ptr [rax + 0x40], xmm0
1402d49f8 movups xmm0, xmmword ptr [rbp - 0x58]
1402d49fc movups xmm1, xmmword ptr [rbp - 0x48]
1402d4a00 movups xmm2, xmmword ptr [rbp - 0x38]
1402d4a04 movups xmm3, xmmword ptr [rbp - 0x28]
1402d4a08 movups xmmword ptr [rax + 0x30], xmm3
1402d4a0c movups xmmword ptr [rax + 0x20], xmm2
1402d4a10 movups xmmword ptr [rax + 0x10], xmm1
1402d4a14 movups xmmword ptr [rax], xmm0
1402d4a17 movups xmm0, xmmword ptr [rbp + 0x18]
1402d4a1b movups xmm1, xmmword ptr [rbp + 0x28]
1402d4a1f movups xmm2, xmmword ptr [rbp + 0x38]
1402d4a23 movups xmm3, xmmword ptr [rbp + 0x48]
1402d4a27 movups xmmword ptr [rax + 0x70], xmm0
1402d4a2b movups xmmword ptr [rax + 0x80], xmm1
1402d4a32 movups xmmword ptr [rax + 0x90], xmm2
1402d4a39 movups xmmword ptr [rax + 0xa0], xmm3
1402d4a40 movups xmm0, xmmword ptr [rbp + 0x58]
1402d4a44 movups xmmword ptr [rax + 0xb0], xmm0
1402d4a4b movups xmm0, xmmword ptr [rbp + 0x68]
1402d4a4f movups xmmword ptr [rax + 0xc0], xmm0
1402d4a56 movups xmm0, xmmword ptr [rbp + 0x78]
1402d4a5a movups xmmword ptr [rax + 0xd0], xmm0
1402d4a61 mov qword ptr [rbp + 0x20], 2
1402d4a69 mov qword ptr [rbp + 0x28], rax
1402d4a6d mov qword ptr [rbp + 0x30], 2
1402d4a75 mov qword ptr [rbp + 0x38], 0
1402d4a7d mov qword ptr [rbp + 0x40], 8
1402d4a85 xorps xmm0, xmm0
1402d4a88 movups xmmword ptr [rbp + 0x48], xmm0
1402d4a8c mov qword ptr [rbp + 0x18], rbx
1402d4a90 lea rcx, [rbp + 0x50]
1402d4a94 call 0x140d2fa20
1402d4a99 movups xmm0, xmmword ptr [rbp + 0x78]
1402d4a9d movups xmmword ptr [rsi + 0x60], xmm0
1402d4aa1 movups xmm0, xmmword ptr [rbp + 0x68]
1402d4aa5 movups xmmword ptr [rsi + 0x50], xmm0
1402d4aa9 movups xmm0, xmmword ptr [rbp + 0x58]
1402d4aad movups xmmword ptr [rsi + 0x40], xmm0
1402d4ab1 movups xmm0, xmmword ptr [rbp + 0x18]
1402d4ab5 movups xmm1, xmmword ptr [rbp + 0x28]
1402d4ab9 movups xmm2, xmmword ptr [rbp + 0x38]
1402d4abd movups xmm3, xmmword ptr [rbp + 0x48]
1402d4ac1 movups xmmword ptr [rsi + 0x30], xmm3
1402d4ac5 movups xmmword ptr [rsi + 0x20], xmm2
1402d4ac9 movups xmmword ptr [rsi + 0x10], xmm1
1402d4acd movups xmmword ptr [rsi], xmm0
1402d4ad0 mov rax, rsi
1402d4ad3 add rsp, 0x238
1402d4ada pop rbx
1402d4adb pop rdi
1402d4adc pop rsi
1402d4add pop rbp
1402d4ade ret 
1402d4adf mov ecx, 8
1402d4ae4 mov edx, 0xe0
1402d4ae9 call 0x1428d9000
1402d4aee mov ecx, 8
1402d4af3 mov edx, 0x190
1402d4af8 call 0x1428d9000
1402d4afd ud2 
1402d4aff nop 
1402d4b00 mov qword ptr [rsp + 0x10], rdx
1402d4b05 push rbp
1402d4b06 push rsi
1402d4b07 push rdi
1402d4b08 push rbx
1402d4b09 sub rsp, 0x28
1402d4b0d lea rbp, [rdx + 0x80]
1402d4b14 lea rcx, [rbp + 0x18]
1402d4b18 call 0x1405d2540
1402d4b1d nop 
1402d4b1e add rsp, 0x28
1402d4b22 pop rbx
1402d4b23 pop rdi
1402d4b24 pop rsi
1402d4b25 pop rbp
1402d4b26 ret 
1402d4b27 nop word ptr [rax + rax]
1402d4b30 mov qword ptr [rsp + 0x10], rdx
1402d4b35 push rbp
1402d4b36 push rsi
1402d4b37 push rdi
1402d4b38 push rbx
1402d4b39 sub rsp, 0x28
1402d4b3d lea rbp, [rdx + 0x80]
1402d4b44 mov edx, 0xe0
1402d4b49 mov r8d, 8
1402d4b4f mov rcx, qword ptr [rbp + 0x1a8]
1402d4b56 call 0x140613c20
1402d4b5b nop 
1402d4b5c add rsp, 0x28
1402d4b60 pop rbx
1402d4b61 pop rdi
1402d4b62 pop rsi
1402d4b63 pop rbp
1402d4b64 ret 
1402d4b65 nop word ptr cs:[rax + rax]
1402d4b70 mov qword ptr [rsp + 0x10], rdx
1402d4b75 push rbp
1402d4b76 push rsi
1402d4b77 push rdi
1402d4b78 push rbx
1402d4b79 sub rsp, 0x28
1402d4b7d lea rbp, [rdx + 0x80]
1402d4b84 lea rcx, [rbp - 0x58]
1402d4b88 call 0x140029880
1402d4b8d nop 
1402d4b8e add rsp, 0x28
1402d4b92 pop rbx
1402d4b93 pop rdi
1402d4b94 pop rsi
1402d4b95 pop rbp
1402d4b96 ret 
1402d4b97 nop word ptr [rax + rax]
1402d4ba0 mov qword ptr [rsp + 0x10], rdx
1402d4ba5 push rbp
1402d4ba6 push rsi
1402d4ba7 push rdi
1402d4ba8 push rbx
1402d4ba9 sub rsp, 0x28
1402d4bad lea rbp, [rdx + 0x80]
1402d4bb4 lea rcx, [rbp + 0x18]
1402d4bb8 call 0x140029880
1402d4bbd nop 
1402d4bbe add rsp, 0x28
1402d4bc2 pop rbx
1402d4bc3 pop rdi
1402d4bc4 pop rsi
1402d4bc5 pop rbp
1402d4bc6 ret 