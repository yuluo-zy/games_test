0x140f98b20 push rbp
0x140f98b21 push rsi
0x140f98b22 push rdi
0x140f98b23 push rbx
0x140f98b24 sub rsp, 0x248
0x140f98b2b lea rbp, [rsp + 0x80]
0x140f98b33 mov qword ptr [rbp + 0x1c0], 0xfffffffffffffffe
0x140f98b3e mov rsi, rcx
0x140f98b41 movzx eax, byte ptr [rip + 0x269c0c1] ; 0x143634c09 b'Z\x19\x030\x14'
0x140f98b48 mov ecx, 0x70
0x140f98b4d mov edx, 8
0x140f98b52 call 0x140613c10
0x140f98b57 test rax, rax
0x140f98b5a je 0x140f98da2
0x140f98b60 mov qword ptr [rbp + 0x1b8], rax
0x140f98b67 xorps xmm0, xmm0
0x140f98b6a movups xmmword ptr [rbp + 0x98], xmm0
0x140f98b71 movups xmmword ptr [rbp + 0xb0], xmm0
0x140f98b78 movups xmmword ptr [rbp + 0xc8], xmm0
0x140f98b7f movups xmmword ptr [rbp + 0xe0], xmm0
0x140f98b86 movups xmmword ptr [rbp + 0xf8], xmm0
0x140f98b8d mov dword ptr [rbp + 0x108], 0
0x140f98b97 movups xmmword ptr [rbp + 0x130], xmm0
0x140f98b9e movups xmmword ptr [rbp + 0x148], xmm0
0x140f98ba5 movups xmmword ptr [rbp + 0x160], xmm0
0x140f98bac movups xmmword ptr [rbp + 0x178], xmm0
0x140f98bb3 movups xmmword ptr [rbp + 0x190], xmm0
0x140f98bba mov dword ptr [rbp + 0x1a0], 0
0x140f98bc4 mov qword ptr [rbp + 0x18], 0
0x140f98bcc mov qword ptr [rbp + 0x78], 0
0x140f98bd4 mov qword ptr [rbp + 0x80], 8
0x140f98bdf mov qword ptr [rbp + 0x88], 0
0x140f98bea mov qword ptr [rbp + 0x90], 0x10
0x140f98bf5 mov qword ptr [rbp + 0xa8], 0x10
0x140f98c00 mov qword ptr [rbp + 0xc0], 0x10
0x140f98c0b mov qword ptr [rbp + 0xd8], 0x10
0x140f98c16 mov qword ptr [rbp + 0xf0], 0x10
0x140f98c21 movabs rbx, 0x8000000000000000
0x140f98c2b mov qword ptr [rbp + 0x110], rbx
0x140f98c32 lea rax, [rip + 0x1c16304] ; 0x142baef3d b'toolbar::mei::core::mei_pre_updatetoolbar::mei::core::load_fontstoolbar::mei::core::mei_post_updatecrates/systems/toolbar/src/mei/core.rs'
0x140f98c39 mov qword ptr [rbp + 0x118], rax
0x140f98c40 mov qword ptr [rbp + 0x120], 0x22
0x140f98c4b mov qword ptr [rbp + 0x128], 0x10
0x140f98c56 mov qword ptr [rbp + 0x140], 0x10
0x140f98c61 mov qword ptr [rbp + 0x158], 0x10
0x140f98c6c mov qword ptr [rbp + 0x170], 0x10
0x140f98c77 mov qword ptr [rbp + 0x188], 0x10
0x140f98c82 mov dword ptr [rbp + 0x1a8], 0
0x140f98c8c mov word ptr [rbp + 0x1ac], 1
0x140f98c95 mov dword ptr [rbp + 0x1b0], 0
0x140f98c9f movzx eax, byte ptr [rip + 0x269bf63] ; 0x143634c09 b'Z\x19\x030\x14'
0x140f98ca6 mov ecx, 0x1a0
0x140f98cab mov edx, 8
0x140f98cb0 call 0x140613c10
0x140f98cb5 test rax, rax
0x140f98cb8 je 0x140f98db1
0x140f98cbe mov rdi, rax
0x140f98cc1 lea rdx, [rbp + 0x18]
0x140f98cc5 mov r8d, 0x1a0
0x140f98ccb mov rcx, rax
0x140f98cce call 0x142923c00
0x140f98cd3 lea r8, [rip + 0x1c15f56] ; 0x142baec30 b'0,]@\x01'
0x140f98cda lea rcx, [rbp - 0x58]
0x140f98cde mov rdx, rdi
0x140f98ce1 call 0x140ce4b10
0x140f98ce6 movups xmm0, xmmword ptr [rbp + 8]
0x140f98cea mov rax, qword ptr [rbp + 0x1b8]
0x140f98cf1 movups xmmword ptr [rax + 0x60], xmm0
0x140f98cf5 movups xmm0, xmmword ptr [rbp - 8]
0x140f98cf9 movups xmmword ptr [rax + 0x50], xmm0
0x140f98cfd movups xmm0, xmmword ptr [rbp - 0x18]
0x140f98d01 movups xmmword ptr [rax + 0x40], xmm0
0x140f98d05 movups xmm0, xmmword ptr [rbp - 0x58]
0x140f98d09 movups xmm1, xmmword ptr [rbp - 0x48]
0x140f98d0d movups xmm2, xmmword ptr [rbp - 0x38]
0x140f98d11 movups xmm3, xmmword ptr [rbp - 0x28]
0x140f98d15 movups xmmword ptr [rax + 0x30], xmm3
0x140f98d19 movups xmmword ptr [rax + 0x20], xmm2
0x140f98d1d movups xmmword ptr [rax + 0x10], xmm1
0x140f98d21 movups xmmword ptr [rax], xmm0
0x140f98d24 mov qword ptr [rbp + 0x20], 1
0x140f98d2c mov qword ptr [rbp + 0x28], rax
0x140f98d30 mov qword ptr [rbp + 0x30], 1
0x140f98d38 mov qword ptr [rbp + 0x38], 0
0x140f98d40 mov qword ptr [rbp + 0x40], 8
0x140f98d48 xorps xmm0, xmm0
0x140f98d4b movups xmmword ptr [rbp + 0x48], xmm0
0x140f98d4f mov qword ptr [rbp + 0x18], rbx
0x140f98d53 lea rcx, [rbp + 0x50]
0x140f98d57 call 0x140fd4720
0x140f98d5c movups xmm0, xmmword ptr [rbp + 0x78]
0x140f98d60 movups xmmword ptr [rsi + 0x60], xmm0
0x140f98d64 movups xmm0, xmmword ptr [rbp + 0x68]
0x140f98d68 movups xmmword ptr [rsi + 0x50], xmm0
0x140f98d6c movups xmm0, xmmword ptr [rbp + 0x58]
0x140f98d70 movups xmmword ptr [rsi + 0x40], xmm0
0x140f98d74 movups xmm0, xmmword ptr [rbp + 0x18]
0x140f98d78 movups xmm1, xmmword ptr [rbp + 0x28]
0x140f98d7c movups xmm2, xmmword ptr [rbp + 0x38]
0x140f98d80 movups xmm3, xmmword ptr [rbp + 0x48]
0x140f98d84 movups xmmword ptr [rsi + 0x30], xmm3
0x140f98d88 movups xmmword ptr [rsi + 0x20], xmm2
0x140f98d8c movups xmmword ptr [rsi + 0x10], xmm1
0x140f98d90 movups xmmword ptr [rsi], xmm0
0x140f98d93 mov rax, rsi
0x140f98d96 add rsp, 0x248
0x140f98d9d pop rbx
0x140f98d9e pop rdi
0x140f98d9f pop rsi
0x140f98da0 pop rbp
0x140f98da1 ret 
0x140f98da2 mov ecx, 8
0x140f98da7 mov edx, 0x70
0x140f98dac call 0x1428d9000
0x140f98db1 mov ecx, 8
0x140f98db6 mov edx, 0x1a0
0x140f98dbb call 0x1428d9000
0x140f98dc0 ud2 
0x140f98dc2 nop word ptr cs:[rax + rax]
0x140f98dd0 mov qword ptr [rsp + 0x10], rdx
0x140f98dd5 push rbp
0x140f98dd6 push rsi
0x140f98dd7 push rdi
0x140f98dd8 push rbx
0x140f98dd9 sub rsp, 0x28
0x140f98ddd lea rbp, [rdx + 0x80]
0x140f98de4 lea rcx, [rbp + 0x18]
0x140f98de8 call 0x1405d2c30
0x140f98ded nop 
0x140f98dee add rsp, 0x28
0x140f98df2 pop rbx
0x140f98df3 pop rdi
0x140f98df4 pop rsi
0x140f98df5 pop rbp
0x140f98df6 ret 
0x140f98df7 nop word ptr [rax + rax]
0x140f98e00 mov qword ptr [rsp + 0x10], rdx
0x140f98e05 push rbp
0x140f98e06 push rsi
0x140f98e07 push rdi
0x140f98e08 push rbx
0x140f98e09 sub rsp, 0x28
0x140f98e0d lea rbp, [rdx + 0x80]
0x140f98e14 mov edx, 0x70
0x140f98e19 mov r8d, 8
0x140f98e1f mov rcx, qword ptr [rbp + 0x1b8]
0x140f98e26 call 0x140613c20
0x140f98e2b nop 
0x140f98e2c add rsp, 0x28
0x140f98e30 pop rbx
0x140f98e31 pop rdi
0x140f98e32 pop rsi
0x140f98e33 pop rbp
0x140f98e34 ret 
0x140f98e35 nop word ptr cs:[rax + rax]
0x140f98e40 mov qword ptr [rsp + 0x10], rdx
0x140f98e45 push rbp
0x140f98e46 push rsi
0x140f98e47 push rdi
0x140f98e48 push rbx
0x140f98e49 sub rsp, 0x28
0x140f98e4d lea rbp, [rdx + 0x80]
0x140f98e54 lea rcx, [rbp + 0x18]
0x140f98e58 call 0x140029880
0x140f98e5d nop 
0x140f98e5e add rsp, 0x28
0x140f98e62 pop rbx
0x140f98e63 pop rdi
0x140f98e64 pop rsi
0x140f98e65 pop rbp
0x140f98e66 ret 