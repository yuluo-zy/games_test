0x140a57300 push     rbp
0x140a57301 push     rsi
0x140a57302 push     rdi
0x140a57303 sub      rsp, 0xb0
0x140a5730a lea      rbp, [rsp + 0x80]
0x140a57312 mov      qword ptr [rbp + 0x28], 0xfffffffffffffffe
0x140a5731a test     byte ptr [rcx], 1
0x140a5731d je       0x140a57362
0x140a5731f mov      rsi, rcx
0x140a57322 add      rsi, 8
0x140a57326 lea      rcx, [rbp - 0x10]
0x140a5732a mov      rdx, rsi
0x140a5732d call     0x141437730 ; libfmod::EventInstance::get_playback_state::h21d1d298b813d681
0x140a57332 movabs   rdi, 0x8000000000000004
0x140a5733c cmp      qword ptr [rbp - 0x10], rdi
0x140a57340 jne      0x140a5736d
0x140a57342 movzx    eax, byte ptr [rbp - 8]
0x140a57346 mov      ecx, 0xb
0x140a5734b bt       ecx, eax
0x140a5734e jb       0x140a57362
0x140a57350 lea      rcx, [rbp - 0x10]
0x140a57354 mov      rdx, rsi
0x140a57357 call     0x1414372d0 ; libfmod::EventInstance::start::hb8dc190a69906390
0x140a5735c cmp      qword ptr [rbp - 0x10], rdi
0x140a57360 jne      0x140a573b7
0x140a57362 add      rsp, 0xb0
0x140a57369 pop      rdi
0x140a5736a pop      rsi
0x140a5736b pop      rbp
0x140a5736c ret      
0x140a5736d mov      rax, qword ptr [rbp + 0x20]
0x140a57371 mov      qword ptr [rbp - 0x20], rax
0x140a57375 movups   xmm0, xmmword ptr [rbp - 0x10]
0x140a57379 movups   xmm1, xmmword ptr [rbp]
0x140a5737d movups   xmm2, xmmword ptr [rbp + 0x10]
0x140a57381 movaps   xmmword ptr [rbp - 0x30], xmm2
0x140a57385 movaps   xmmword ptr [rbp - 0x40], xmm1
0x140a57389 movaps   xmmword ptr [rbp - 0x50], xmm0
0x140a5738d lea      rax, [rip + 0x207b844]
0x140a57394 mov      qword ptr [rsp + 0x20], rax
0x140a57399 lea      rcx, [rip + 0x2079260] ; @0x142ad0600 'called `Result::unwrap()` on an `Err` value'
0x140a573a0 lea      r9, [rip + 0x2079219]
0x140a573a7 lea      r8, [rbp - 0x50]
0x140a573ab mov      edx, 0x2b
0x140a573b0 call     0x1428d9760 ; core::result::unwrap_failed::h96d74ae09566f4b3
0x140a573b5 jmp      0x140a573ff ; 
0x140a573b7 mov      rax, qword ptr [rbp + 0x20]
0x140a573bb mov      qword ptr [rbp - 0x20], rax
0x140a573bf movups   xmm0, xmmword ptr [rbp - 0x10]
0x140a573c3 movups   xmm1, xmmword ptr [rbp]
0x140a573c7 movups   xmm2, xmmword ptr [rbp + 0x10]
0x140a573cb movaps   xmmword ptr [rbp - 0x30], xmm2
0x140a573cf movaps   xmmword ptr [rbp - 0x40], xmm1
0x140a573d3 movaps   xmmword ptr [rbp - 0x50], xmm0
0x140a573d7 lea      rax, [rip + 0x207b812]
0x140a573de mov      qword ptr [rsp + 0x20], rax
0x140a573e3 lea      rcx, [rip + 0x2079216] ; @0x142ad0600 'called `Result::unwrap()` on an `Err` value'
0x140a573ea lea      r9, [rip + 0x20791cf]
0x140a573f1 lea      r8, [rbp - 0x50]
0x140a573f5 mov      edx, 0x2b
0x140a573fa call     0x1428d9760 ; core::result::unwrap_failed::h96d74ae09566f4b3
0x140a573ff ud2      
0x140a57401 nop      word ptr cs:[rax + rax]
0x140a57410 mov      qword ptr [rsp + 0x10], rdx
0x140a57415 push     rbp
0x140a57416 push     rsi
0x140a57417 push     rdi
0x140a57418 sub      rsp, 0x30
0x140a5741c lea      rbp, [rdx + 0x80]
0x140a57423 lea      rcx, [rbp - 0x50]
0x140a57427 call     0x140a526b0 ; core::ptr::drop_in_place$LT$libfmod..Error$GT$::h60b34b2c39a0f0f1 (.llvm.4456468861159116502)
0x140a5742c nop      
0x140a5742d add      rsp, 0x30
0x140a57431 pop      rdi
0x140a57432 pop      rsi
0x140a57433 pop      rbp
0x140a57434 ret      
0x140a57435 nop      word ptr cs:[rax + rax]
0x140a57440 mov      qword ptr [rsp + 0x10], rdx
0x140a57445 push     rbp
0x140a57446 push     rsi
0x140a57447 push     rdi
0x140a57448 sub      rsp, 0x30
0x140a5744c lea      rbp, [rdx + 0x80]
0x140a57453 lea      rcx, [rbp - 0x50]
0x140a57457 call     0x140a526b0 ; core::ptr::drop_in_place$LT$libfmod..Error$GT$::h60b34b2c39a0f0f1 (.llvm.4456468861159116502)
0x140a5745c nop      
0x140a5745d add      rsp, 0x30
0x140a57461 pop      rdi
0x140a57462 pop      rsi
0x140a57463 pop      rbp
0x140a57464 ret      