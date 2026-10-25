0x1400c0260 push     rsi
0x1400c0261 push     rdi
0x1400c0262 push     rbx
0x1400c0263 sub      rsp, 0x110
0x1400c026a mov      rsi, rdx
0x1400c026d mov      rdi, qword ptr [rcx]
0x1400c0270 mov      eax, dword ptr [rdi + 0x1c]
0x1400c0273 mov      rcx, qword ptr [rdi + 0x10]
0x1400c0277 mov      dword ptr [rcx], eax
0x1400c0279 mov      rbx, qword ptr [rdi]
0x1400c027c movzx    eax, byte ptr [rip + 0x3574986]
0x1400c0283 mov      ecx, 6
0x1400c0288 mov      edx, 1
0x1400c028d call     0x140613c10
0x1400c0292 test     rax, rax
0x1400c0295 je       0x1400c03af
0x1400c029b add      rbx, 4
0x1400c029f mov      word ptr [rax + 4], 0x656c
0x1400c02a5 mov      dword ptr [rax], 0x62616e45
0x1400c02ab mov      qword ptr [rsp + 0x60], 6
0x1400c02b4 mov      qword ptr [rsp + 0x68], rax
0x1400c02b9 mov      qword ptr [rsp + 0x70], 6
0x1400c02c2 mov      dword ptr [rsp + 0x78], 0
0x1400c02ca mov      qword ptr [rsp + 0x88], 0
0x1400c02d6 mov      dword ptr [rsp + 0x80], 0
0x1400c02e1 mov      qword ptr [rsp + 0x30], 3
0x1400c02ea mov      qword ptr [rsp + 0x48], 6
0x1400c02f3 mov      byte ptr [rsp + 0x90], 0
0x1400c02fb mov      dword ptr [rsp + 0x95], 0
0x1400c0306 mov      dword ptr [rsp + 0x98], 0
0x1400c0311 mov      qword ptr [rsp + 0xa0], rbx
0x1400c0319 mov      byte ptr [rsp + 0xa8], 0
0x1400c0321 lea      rcx, [rsp + 0xb0]
0x1400c0329 lea      rdx, [rsp + 0x30]
0x1400c032e mov      r8, rsi
0x1400c0331 call     0x140c48930
0x1400c0336 mov      rax, qword ptr [rsp + 0xc0]
0x1400c033e lock dec qword ptr [rax]
0x1400c0342 jne      0x1400c0351
0x1400c0344 lea      rcx, [rsp + 0xc0]
0x1400c034c call     0x140c41170
0x1400c0351 movzx    eax, byte ptr [rip + 0x35748b1]
0x1400c0358 mov      ecx, 8
0x1400c035d mov      edx, 8
0x1400c0362 call     0x140613c10
0x1400c0367 test     rax, rax
0x1400c036a je       0x1400c03c5
0x1400c036c mov      qword ptr [rax], rdi
0x1400c036f lea      rcx, [rip + 0x28746fa]
0x1400c0376 mov      qword ptr [rsp + 0x20], rcx
0x1400c037b lea      rcx, [rsp + 0x30]
0x1400c0380 mov      rdx, rsi
0x1400c0383 xor      r8d, r8d
0x1400c0386 mov      r9, rax
0x1400c0389 call     0x1400c4230
0x1400c038e mov      rax, qword ptr [rsp + 0x40]
0x1400c0393 lock dec qword ptr [rax]
0x1400c0397 jne      0x1400c03a3
0x1400c0399 lea      rcx, [rsp + 0x40]
0x1400c039e call     0x140c41170
0x1400c03a3 nop      
0x1400c03a4 add      rsp, 0x110
0x1400c03ab pop      rbx
0x1400c03ac pop      rdi
0x1400c03ad pop      rsi
0x1400c03ae ret      
0x1400c03af lea      r8, [rip + 0x2874992]
0x1400c03b6 mov      ecx, 1
0x1400c03bb mov      edx, 6
0x1400c03c0 call     0x1428d8fe3
0x1400c03c5 mov      ecx, 8
0x1400c03ca mov      edx, 8
0x1400c03cf call     0x1428d9000