0x14083c4a0 push     rbp
0x14083c4a1 push     r15
0x14083c4a3 push     r14
0x14083c4a5 push     rsi
0x14083c4a6 push     rdi
0x14083c4a7 push     rbx
0x14083c4a8 sub      rsp, 0x3b8
0x14083c4af lea      rbp, [rsp + 0x80]
0x14083c4b7 mov      qword ptr [rbp + 0x330], 0xfffffffffffffffe
0x14083c4c2 mov      rsi, qword ptr [rcx]
0x14083c4c5 test     rsi, rsi
0x14083c4c8 je       0x14083c5b6
0x14083c4ce mov      eax, dword ptr [rdx + 0x1c]
0x14083c4d1 mov      rcx, qword ptr [rdx + 0x10]
0x14083c4d5 mov      dword ptr [rcx], eax
0x14083c4d7 mov      rbx, qword ptr [rdx]
0x14083c4da mov      r14, qword ptr [r8]
0x14083c4dd movzx    eax, byte ptr [r14 + 0x100]
0x14083c4e5 mov      byte ptr [rbp + 0x32f], al
0x14083c4eb lea      rdx, [rip + 0x2257019] ; @0x142a9350b 'DJ stationMusic richness: '
0x14083c4f2 lea      rdi, [rbp - 0x58]
0x14083c4f6 mov      r15, r8
0x14083c4f9 mov      r8d, 0xa
0x14083c4ff mov      rcx, rdi
0x14083c502 call     0x140adb2f0 ; egui::containers::window::Window::new::h072c7dbfd17a5832
0x14083c507 mov      eax, dword ptr [r15 + 0x1c]
0x14083c50b mov      rcx, qword ptr [r15 + 0x10]
0x14083c50f mov      dword ptr [rcx], eax
0x14083c511 add      r14, 0xf6
0x14083c518 mov      qword ptr [rbp + 0xd0], r14
0x14083c51f lea      rcx, [rbp + 0x130]
0x14083c526 mov      r8d, 0x188
0x14083c52c mov      rdx, rdi
0x14083c52f call     0x142923c00 ; memcpy
0x14083c534 movzx    eax, byte ptr [rip + 0x2df86ce]
0x14083c53b mov      ecx, 0x18
0x14083c540 mov      edx, 8
0x14083c545 call     0x140613c10 ; __rust_alloc
0x14083c54a test     rax, rax
0x14083c54d mov      qword ptr [rbp + 0x320], rax
0x14083c554 je       0x14083c5c7
0x14083c556 mov      r9, rax
0x14083c559 lea      rax, [rbp + 0x32f]
0x14083c560 mov      qword ptr [r9], rax
0x14083c563 mov      qword ptr [r9 + 8], rbx
0x14083c567 add      rbx, 0x1c0
0x14083c56e mov      qword ptr [r9 + 0x10], rbx
0x14083c572 lea      rax, [rip + 0x2256e87]
0x14083c579 mov      qword ptr [rsp + 0x20], rax
0x14083c57e lea      rcx, [rbp + 0x2b8]
0x14083c585 lea      rdx, [rbp + 0x130]
0x14083c58c mov      r8, rsi
0x14083c58f call     0x140adb610 ; egui::containers::window::Window::show_dyn::h7f985670d5ef5616
0x14083c594 cmp      dword ptr [rbp + 0x2f8], 2
0x14083c59b je       0x14083c5b6
0x14083c59d mov      rax, qword ptr [rbp + 0x2c8]
0x14083c5a4 lock dec qword ptr [rax]
0x14083c5a8 jne      0x14083c5b6
0x14083c5aa lea      rcx, [rbp + 0x2c8]
0x14083c5b1 call     0x140c41170 ; alloc::sync::Arc$LT$T$C$A$GT$::drop_slow::he9d75ed3b56676ff
0x14083c5b6 nop      
0x14083c5b7 add      rsp, 0x3b8
0x14083c5be pop      rbx
0x14083c5bf pop      rdi
0x14083c5c0 pop      rsi
0x14083c5c1 pop      r14
0x14083c5c3 pop      r15
0x14083c5c5 pop      rbp
0x14083c5c6 ret      
0x14083c5c7 mov      ecx, 8
0x14083c5cc mov      edx, 0x18
0x14083c5d1 call     0x1428d9000 ; alloc::alloc::handle_alloc_error::h3e7daf9bcd04547a
0x14083c5d6 ud2      
0x14083c5d8 nop      dword ptr [rax + rax]
0x14083c5e0 mov      qword ptr [rsp + 0x10], rdx
0x14083c5e5 push     rbp
0x14083c5e6 push     r15
0x14083c5e8 push     r14
0x14083c5ea push     rsi
0x14083c5eb push     rdi
0x14083c5ec push     rbx
0x14083c5ed sub      rsp, 0x28
0x14083c5f1 lea      rbp, [rdx + 0x80]
0x14083c5f8 cmp      qword ptr [rbp + 0x320], 0
0x14083c600 je       0x14083c60f
0x14083c602 add      rsp, 0x28
0x14083c606 pop      rbx
0x14083c607 pop      rdi
0x14083c608 pop      rsi
0x14083c609 pop      r14
0x14083c60b pop      r15
0x14083c60d pop      rbp
0x14083c60e ret      
0x14083c60f lea      rcx, [rbp + 0x130]
0x14083c616 call     0x140053a80 ; core::ptr::drop_in_place$LT$egui..containers..window..Window$GT$::h2b1a9de4aaa9869d / core::ptr::drop_in_place$LT$egui..widget_text..WidgetText$GT$::h988252f7ba924154 (.llvm.12463876434711410378)
0x14083c61b jmp      0x14083c602 ; 