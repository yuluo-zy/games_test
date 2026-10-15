0x1405f4180 push     rsi
0x1405f4181 sub      rsp, 0x20
0x1405f4185 mov      rsi, rcx
0x1405f4188 mov      rdx, qword ptr [rcx + 0x10]
0x1405f418c mov      rax, rdx
0x1405f418f neg      rax
0x1405f4192 jo       0x1405f41a5
0x1405f4194 jae      0x1405f41a5
0x1405f4196 mov      rcx, qword ptr [rsi + 0x18]
0x1405f419a mov      r8d, 1
0x1405f41a0 call     0x140613c20
0x1405f41a5 mov      rdx, qword ptr [rsi + 0x30]
0x1405f41a9 mov      rax, rdx
0x1405f41ac neg      rax
0x1405f41af jo       0x1405f41c2
0x1405f41b1 jae      0x1405f41c2
0x1405f41b3 mov      rcx, qword ptr [rsi + 0x38]
0x1405f41b7 mov      r8d, 1
0x1405f41bd call     0x140613c20
0x1405f41c2 mov      rdx, qword ptr [rsi + 0x50]
0x1405f41c6 mov      rax, rdx
0x1405f41c9 neg      rax
0x1405f41cc jo       0x1405f41df
0x1405f41ce jae      0x1405f41df
0x1405f41d0 mov      rcx, qword ptr [rsi + 0x58]
0x1405f41d4 mov      r8d, 1
0x1405f41da call     0x140613c20
0x1405f41df mov      rdx, qword ptr [rsi + 0x70]
0x1405f41e3 mov      rax, rdx
0x1405f41e6 neg      rax
0x1405f41e9 jo       0x1405f41fc
0x1405f41eb jae      0x1405f41fc
0x1405f41ed mov      rcx, qword ptr [rsi + 0x78]
0x1405f41f1 mov      r8d, 1
0x1405f41f7 call     0x140613c20
0x1405f41fc mov      rdx, qword ptr [rsi + 0x90]
0x1405f4203 mov      rax, rdx
0x1405f4206 neg      rax
0x1405f4209 jo       0x1405f421f
0x1405f420b jae      0x1405f421f
0x1405f420d mov      rcx, qword ptr [rsi + 0x98]
0x1405f4214 mov      r8d, 1
0x1405f421a call     0x140613c20
0x1405f421f mov      rdx, qword ptr [rsi + 0xc0]
0x1405f4226 test     rdx, rdx
0x1405f4229 je       0x1405f425c
0x1405f422b mov      rax, rdx
0x1405f422e shl      rax, 4
0x1405f4232 add      rdx, rax
0x1405f4235 add      rdx, 0x21
0x1405f4239 je       0x1405f425c
0x1405f423b mov      rcx, qword ptr [rsi + 0xb8]
0x1405f4242 neg      rax
0x1405f4245 add      rcx, rax
0x1405f4248 add      rcx, -0x10
0x1405f424c mov      r8d, 0x10
0x1405f4252 add      rsp, 0x20
0x1405f4256 pop      rsi
0x1405f4257 jmp      0x140613c20
0x1405f425c nop      
0x1405f425d add      rsp, 0x20
0x1405f4261 pop      rsi
0x1405f4262 ret      