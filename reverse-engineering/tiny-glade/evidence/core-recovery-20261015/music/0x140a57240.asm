0x140a57240 push     rbp
0x140a57241 sub      rsp, 0xa0
0x140a57248 lea      rbp, [rsp + 0x80]
0x140a57250 mov      qword ptr [rbp + 0x18], 0xfffffffffffffffe
0x140a57258 cmp      dword ptr [rcx], 1
0x140a5725b jne      0x140a5727d
0x140a5725d mov      rdx, rcx
0x140a57260 add      rdx, 8
0x140a57264 lea      rcx, [rbp - 0x58]
0x140a57268 call     0x1414372d0 ; libfmod::EventInstance::start::hb8dc190a69906390
0x140a5726d movabs   rax, 0x8000000000000004
0x140a57277 cmp      qword ptr [rbp - 0x58], rax
0x140a5727b jne      0x140a57286
0x140a5727d add      rsp, 0xa0
0x140a57284 pop      rbp
0x140a57285 ret      
0x140a57286 mov      rax, qword ptr [rbp - 0x28]
0x140a5728a mov      qword ptr [rbp + 0x10], rax
0x140a5728e movups   xmm0, xmmword ptr [rbp - 0x58]
0x140a57292 movups   xmm1, xmmword ptr [rbp - 0x48]
0x140a57296 movups   xmm2, xmmword ptr [rbp - 0x38]
0x140a5729a movaps   xmmword ptr [rbp], xmm2
0x140a5729e movaps   xmmword ptr [rbp - 0x10], xmm1
0x140a572a2 movaps   xmmword ptr [rbp - 0x20], xmm0
0x140a572a6 lea      rax, [rip + 0x207b913]
0x140a572ad mov      qword ptr [rsp + 0x20], rax
0x140a572b2 lea      rcx, [rip + 0x2079347] ; @0x142ad0600 'called `Result::unwrap()` on an `Err` value'
0x140a572b9 lea      r9, [rip + 0x2079300]
0x140a572c0 lea      r8, [rbp - 0x20]
0x140a572c4 mov      edx, 0x2b
0x140a572c9 call     0x1428d9760 ; core::result::unwrap_failed::h96d74ae09566f4b3
0x140a572ce ud2      
0x140a572d0 mov      qword ptr [rsp + 0x10], rdx
0x140a572d5 push     rbp
0x140a572d6 sub      rsp, 0x30
0x140a572da lea      rbp, [rdx + 0x80]
0x140a572e1 lea      rcx, [rbp - 0x20]
0x140a572e5 call     0x140a526b0 ; core::ptr::drop_in_place$LT$libfmod..Error$GT$::h60b34b2c39a0f0f1 (.llvm.4456468861159116502)
0x140a572ea nop      
0x140a572eb add      rsp, 0x30
0x140a572ef pop      rbp
0x140a572f0 ret      