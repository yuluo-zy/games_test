0x141439d70 push r14
0x141439d72 push rsi
0x141439d73 push rdi
0x141439d74 push rbx
0x141439d75 sub rsp, 0x88
0x141439d7c mov esi, 1
0x141439d81 lock xadd dword ptr [rdx + 0x5b0], esi
0x141439d89 test byte ptr [rcx], 1
0x141439d8c je 0x141439fef
0x141439d92 mov rax, qword ptr [rcx + 8]
0x141439d96 cmp qword ptr [rdx + 0x218], rax
0x141439d9d jbe 0x141439ee5
0x141439da3 mov r8, qword ptr [rdx + 0x210]
0x141439daa mov r8, qword ptr [r8 + rax*8]
0x141439dae test r8, r8
0x141439db1 je 0x141439ee5
0x141439db7 mov rax, qword ptr [rdx + 0x1e0]
0x141439dbe not r8
0x141439dc1 lea r8, [r8 + r8*2]
0x141439dc5 shl r8, 5
0x141439dc9 cmp qword ptr [rax + r8 + 0x20], 0
0x141439dcf je 0x141439ee5
0x141439dd5 add rax, r8
0x141439dd8 mov rdi, qword ptr [rax + 0x10]
0x141439ddc mov r8d, dword ptr [rdx + 0x5b0]
0x141439de3 mov r8, qword ptr [rcx + 0x18]
0x141439de7 cmp qword ptr [rdx + 0x218], r8
0x141439dee jbe 0x141439f6a
0x141439df4 mov r9, qword ptr [rdx + 0x210]
0x141439dfb mov r8, qword ptr [r9 + r8*8]
0x141439dff test r8, r8
0x141439e02 je 0x141439f6a
0x141439e08 mov rdx, qword ptr [rdx + 0x1e0]
0x141439e0f not r8
0x141439e12 lea r8, [r8 + r8*2]
0x141439e16 shl r8, 5
0x141439e1a cmp qword ptr [rdx + r8 + 0x20], 0
0x141439e20 je 0x141439f6a
0x141439e26 add rdx, r8
0x141439e29 mov rdx, qword ptr [rdx + 0x10]
0x141439e2d mov r8, qword ptr [rcx + 0x10]
0x141439e31 xor r9d, r9d
0x141439e34 mov r10, r8
0x141439e37 sub r10, qword ptr [rdx + 0x18]
0x141439e3b cmovb r10, r9
0x141439e3f sub r8, qword ptr [rdx + 0x38]
0x141439e43 cmovb r8, r9
0x141439e47 mov r11, qword ptr [rdx + 0x30]
0x141439e4b sub r11, r8
0x141439e4e cmovb r11, r9
0x141439e52 setbe r8b
0x141439e56 mov rbx, qword ptr [rdx + 0x10]
0x141439e5a sub rbx, r10
0x141439e5d cmovb rbx, r9
0x141439e61 mov rdx, qword ptr [rdx + 0x40]
0x141439e65 setbe r9b
0x141439e69 add rbx, r11
0x141439e6c mov r10, rdx
0x141439e6f sub r10, rbx
0x141439e72 mov qword ptr [rcx + 0x10], r10
0x141439e76 test r9b, r8b
0x141439e79 jne 0x141439ed0
0x141439e7b mov qword ptr [rcx + 0x10], rdx
0x141439e7f mov dword ptr [rax + 0x5c], esi
0x141439e82 cmp qword ptr [rdi + 0x48], 0
0x141439e87 je 0x141439ed0
0x141439e89 mov rbx, qword ptr [rdi + 0x38]
0x141439e8d test rbx, rbx
0x141439e90 je 0x141439eaa
0x141439e92 mov rax, qword ptr [rdi + 0x30]
0x141439e96 lea r8, [rbx + 0x11]
0x141439e9a mov r14, rcx
0x141439e9d mov rcx, rax
0x141439ea0 mov dl, 0xff
0x141439ea2 call 0x142923c20
0x141439ea7 mov rcx, r14
0x141439eaa mov qword ptr [rdi + 0x48], 0
0x141439eb2 lea rax, [rbx + 1]
0x141439eb6 mov rdx, rax
0x141439eb9 shr rdx, 3
0x141439ebd and rax, 0xfffffffffffffff8
0x141439ec1 sub rax, rdx
0x141439ec4 cmp rbx, 8
0x141439ec8 cmovb rax, rbx
0x141439ecc mov qword ptr [rdi + 0x40], rax
0x141439ed0 mov dword ptr [rcx + 0x158], esi
0x141439ed6 xor eax, eax
0x141439ed8 add rsp, 0x88
0x141439edf pop rbx
0x141439ee0 pop rdi
0x141439ee1 pop rsi
0x141439ee2 pop r14
0x141439ee4 ret 
0x141439ee5 add rcx, 0xc0
0x141439eec lea rax, [rip + 0x18248d1]
0x141439ef3 mov qword ptr [rsp + 0x28], rax
0x141439ef8 mov qword ptr [rsp + 0x30], 0x47
0x141439f01 mov qword ptr [rsp + 0x38], rcx
0x141439f06 lea rax, [rip - 0x13fe43d]
0x141439f0d mov qword ptr [rsp + 0x40], rax
0x141439f12 lea rax, [rsp + 0x28]
0x141439f17 mov qword ptr [rsp + 0x48], rax
0x141439f1c lea rax, [rip - 0x1330093]
0x141439f23 mov qword ptr [rsp + 0x50], rax
0x141439f28 lea rax, [rip + 0x1824611]
0x141439f2f mov qword ptr [rsp + 0x58], rax
0x141439f34 mov qword ptr [rsp + 0x60], 2
0x141439f3d mov qword ptr [rsp + 0x78], 0
0x141439f46 lea rax, [rsp + 0x38]
0x141439f4b mov qword ptr [rsp + 0x68], rax
0x141439f50 mov qword ptr [rsp + 0x70], 2
0x141439f59 lea rdx, [rip + 0x18248b0]
0x141439f60 lea rcx, [rsp + 0x58]
0x141439f65 call 0x1428d9390
0x141439f6a add rcx, 0xc0
0x141439f71 lea rax, [rip + 0x18247e0]
0x141439f78 mov qword ptr [rsp + 0x28], rax
0x141439f7d mov qword ptr [rsp + 0x30], 0x6c
0x141439f86 mov qword ptr [rsp + 0x38], rcx
0x141439f8b lea rax, [rip - 0x13fe4c2]
0x141439f92 mov qword ptr [rsp + 0x40], rax
0x141439f97 lea rax, [rsp + 0x28]
0x141439f9c mov qword ptr [rsp + 0x48], rax
0x141439fa1 lea rax, [rip - 0x1330118]
0x141439fa8 mov qword ptr [rsp + 0x50], rax
0x141439fad lea rax, [rip + 0x182458c]
0x141439fb4 mov qword ptr [rsp + 0x58], rax
0x141439fb9 mov qword ptr [rsp + 0x60], 2
0x141439fc2 mov qword ptr [rsp + 0x78], 0
0x141439fcb lea rax, [rsp + 0x38]
0x141439fd0 mov qword ptr [rsp + 0x68], rax
0x141439fd5 mov qword ptr [rsp + 0x70], 2
0x141439fde lea rdx, [rip + 0x18245e3]
0x141439fe5 lea rcx, [rsp + 0x58]
0x141439fea call 0x1428d9390
0x141439fef lea rcx, [rip + 0x1824b4a]
0x141439ff6 lea r8, [rip + 0x1824ba3]
0x141439ffd mov edx, 0x59
0x14143a002 call 0x1428d9330
0x14143a007 int3 
