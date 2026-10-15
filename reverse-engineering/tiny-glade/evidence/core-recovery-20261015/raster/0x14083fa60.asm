0x14083fa60 push     r14
0x14083fa62 push     rsi
0x14083fa63 push     rdi
0x14083fa64 push     rbx
0x14083fa65 sub      rsp, 0x68
0x14083fa69 mov      rsi, rdx
0x14083fa6c mov      rdi, rcx
0x14083fa6f mov      eax, dword ptr [rcx + 0x1c]
0x14083fa72 mov      rcx, qword ptr [rcx + 0x10]
0x14083fa76 mov      dword ptr [rcx], eax
0x14083fa78 mov      rbx, qword ptr [rdi]
0x14083fa7b lea      rdx, [rip + 0x23ee78e]
0x14083fa82 mov      rcx, rsi
0x14083fa85 call     0x140969720
0x14083fa8a test     rax, rax
0x14083fa8d je       0x14083fe13
0x14083fa93 mov      r14, rax
0x14083fa96 lea      rax, [rip + 0x225dbc0]
0x14083fa9d mov      qword ptr [rsp + 0x28], rax
0x14083faa2 mov      qword ptr [rsp + 0x30], 0x41
0x14083faab cmp      qword ptr [r14 + 0x18], 0x41
0x14083fab0 jne      0x14083fdb9
0x14083fab6 mov      rdx, qword ptr [r14 + 0x10]
0x14083faba lea      rcx, [rip + 0x225db9c]
0x14083fac1 mov      r8d, 0x41
0x14083fac7 call     0x142923bf0
0x14083facc test     eax, eax
0x14083face jne      0x14083fdb9
0x14083fad4 mov      rcx, qword ptr [r14]
0x14083fad7 mov      rax, qword ptr [r14 + 8]
0x14083fadb call     qword ptr [rax + 0x18]