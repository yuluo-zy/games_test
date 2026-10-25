0x1408c0d90 push     rbp
0x1408c0d91 push     r15
0x1408c0d93 push     r14
0x1408c0d95 push     r13
0x1408c0d97 push     r12
0x1408c0d99 push     rsi
0x1408c0d9a push     rdi
0x1408c0d9b push     rbx
0x1408c0d9c sub      rsp, 0x78
0x1408c0da0 lea      rbp, [rsp + 0x70]
0x1408c0da5 mov      qword ptr [rbp], 0xfffffffffffffffe
0x1408c0dad mov      dl, 1
0x1408c0daf xor      eax, eax
0x1408c0db1 lock cmpxchg byte ptr [rip + 0x2d6b22f], dl
0x1408c0db9 jne      0x1408c0f4a
0x1408c0dbf mov      r13, qword ptr [rip + 0x2d6b65a]
0x1408c0dc6 mov      rax, qword ptr [r13]
0x1408c0dca shl      rax, 1
0x1408c0dcd test     rax, rax
0x1408c0dd0 jne      0x1408c0f64
0x1408c0dd6 xor      edi, edi
0x1408c0dd8 movzx    eax, byte ptr [rip + 0x2d6b20a]
0x1408c0ddf test     al, al
0x1408c0de1 jne      0x1408c0f84
0x1408c0de7 mov      rax, qword ptr [rip + 0x2d6b212]
0x1408c0dee mov      qword ptr [rip + 0x2d6b207], 0
0x1408c0df9 mov      r15, qword ptr [rip + 0x2d6b1f8]
0x1408c0e00 mov      rbx, rax
0x1408c0e03 shl      rbx, 5
0x1408c0e07 lea      rdx, [r15 + rbx]
0x1408c0e0b mov      qword ptr [rbp - 0x38], rdx
0x1408c0e0f lea      rdx, [rip + 0x2d6b1da]
0x1408c0e16 mov      qword ptr [rbp - 0x30], rdx
0x1408c0e1a mov      qword ptr [rbp - 0x28], rax
0x1408c0e1e mov      qword ptr [rbp - 0x20], 0
0x1408c0e26 test     rax, rax
0x1408c0e29 je       0x1408c0f14
0x1408c0e2f mov      dword ptr [rbp - 4], edi
0x1408c0e32 mov      r13d, dword ptr [rcx + 0x1c]
0x1408c0e36 mov      rsi, qword ptr [rcx]
0x1408c0e39 mov      rdi, qword ptr [rcx + 0x10]
0x1408c0e3d lea      r14, [r15 + 0x28]
0x1408c0e41 xor      eax, eax
0x1408c0e43 jmp      0x1408c0e5a
0x1408c0e45 nop      word ptr cs:[rax + rax]
0x1408c0e50 lea      rax, [r12 + 0x20]
0x1408c0e55 cmp      rbx, rax
0x1408c0e58 je       0x1408c0ebc
0x1408c0e5a mov      r12, rax
0x1408c0e5d add      rax, r15
0x1408c0e60 add      rax, 0x20
0x1408c0e64 mov      qword ptr [rbp - 0x40], rax
0x1408c0e68 mov      rcx, qword ptr [r15 + r12]
0x1408c0e6c mov      rax, rcx
0x1408c0e6f neg      rax
0x1408c0e72 jo       0x1408c0ebc
0x1408c0e74 mov      qword ptr [rbp - 0x10], rcx
0x1408c0e78 mov      rdx, qword ptr [r15 + r12 + 8]
0x1408c0e7d mov      r8, qword ptr [r15 + r12 + 0x10]
0x1408c0e82 movsx    rax, byte ptr [r15 + r12 + 0x18]
0x1408c0e88 mov      dword ptr [rdi], r13d
0x1408c0e8b lea      rcx, [rip + 0x21e070a]
0x1408c0e92 mov      r9d, dword ptr [rcx + rax*4]
0x1408c0e96 mov      rcx, rsi
0x1408c0e99 mov      qword ptr [rbp - 0x18], rdx
0x1408c0e9d call     0x14093fe50
0x1408c0ea2 mov      rdx, qword ptr [rbp - 0x10]
0x1408c0ea6 test     rdx, rdx
0x1408c0ea9 je       0x1408c0e50
0x1408c0eab mov      r8d, 1
0x1408c0eb1 mov      rcx, qword ptr [rbp - 0x18]
0x1408c0eb5 call     0x140613c20
0x1408c0eba jmp      0x1408c0e50
0x1408c0ebc lea      rax, [rbx - 0x20]
0x1408c0ec0 cmp      rax, r12
0x1408c0ec3 mov      edi, dword ptr [rbp - 4]
0x1408c0ec6 mov      r13, qword ptr [rip + 0x2d6b553]
0x1408c0ecd je       0x1408c0f14
0x1408c0ecf sub      rbx, r12
0x1408c0ed2 add      rbx, -0x20
0x1408c0ed6 mov      rsi, qword ptr [rip + 0x2d6b11b]
0x1408c0edd shr      rbx, 5
0x1408c0ee1 add      r14, r12
0x1408c0ee4 add      r14, rsi
0x1408c0ee7 neg      rsi
0x1408c0eea jmp      0x1408c0ef9
0x1408c0eec nop      dword ptr [rax]
0x1408c0ef0 add      r14, 0x20
0x1408c0ef4 dec      rbx
0x1408c0ef7 je       0x1408c0f14
0x1408c0ef9 mov      rdx, qword ptr [r14 + rsi - 8]
0x1408c0efe test     rdx, rdx
0x1408c0f01 je       0x1408c0ef0
0x1408c0f03 mov      rcx, qword ptr [r14 + rsi]
0x1408c0f07 mov      r8d, 1
0x1408c0f0d call     0x140613c20
0x1408c0f12 jmp      0x1408c0ef0
0x1408c0f14 test     dil, dil
0x1408c0f17 jne      0x1408c0f29
0x1408c0f19 mov      rax, qword ptr [r13]
0x1408c0f1d shl      rax, 1
0x1408c0f20 test     rax, rax
0x1408c0f23 jne      0x1408c0fd9
0x1408c0f29 xor      eax, eax
0x1408c0f2b xchg     byte ptr [rip + 0x2d6b0b7], al
0x1408c0f31 cmp      al, 2
0x1408c0f33 je       0x1408c0fbd
0x1408c0f39 add      rsp, 0x78
0x1408c0f3d pop      rbx
0x1408c0f3e pop      rdi
0x1408c0f3f pop      rsi
0x1408c0f40 pop      r12
0x1408c0f42 pop      r13
0x1408c0f44 pop      r14
0x1408c0f46 pop      r15
0x1408c0f48 pop      rbp
0x1408c0f49 ret      
0x1408c0f4a lea      rax, [rip + 0x2d6b097]
0x1408c0f51 mov      rsi, rcx
0x1408c0f54 mov      rcx, rax
0x1408c0f57 call     0x1428d8750
0x1408c0f5c mov      rcx, rsi
0x1408c0f5f jmp      0x1408c0dbf
0x1408c0f64 mov      rsi, rcx
0x1408c0f67 call     0x1428d86b0
0x1408c0f6c mov      rcx, rsi
0x1408c0f6f mov      edi, eax
0x1408c0f71 xor      dil, 1
0x1408c0f75 movzx    eax, byte ptr [rip + 0x2d6b06d]
0x1408c0f7c test     al, al
0x1408c0f7e je       0x1408c0de7
0x1408c0f84 lea      rax, [rip + 0x2d6b05d]
0x1408c0f8b mov      qword ptr [rbp - 0x40], rax
0x1408c0f8f mov      byte ptr [rbp - 0x38], dil
0x1408c0f93 lea      rax, [rip + 0x21e0386]
0x1408c0f9a mov      qword ptr [rsp + 0x20], rax
0x1408c0f9f lea      rcx, [rip + 0x21df412] ; @0x142aa03b8 'called `Result::unwrap()` on an `Err` value'
0x1408c0fa6 lea      r9, [rip + 0x21df49b] ; @0x142aa0448 'p(]@'
0x1408c0fad lea      r8, [rbp - 0x40]
0x1408c0fb1 mov      edx, 0x2b
0x1408c0fb6 call     0x1428d9760
0x1408c0fbb ud2      
0x1408c0fbd lea      rcx, [rip + 0x2d6b024]
0x1408c0fc4 add      rsp, 0x78
0x1408c0fc8 pop      rbx
0x1408c0fc9 pop      rdi
0x1408c0fca pop      rsi
0x1408c0fcb pop      r12
0x1408c0fcd pop      r13
0x1408c0fcf pop      r14
0x1408c0fd1 pop      r15
0x1408c0fd3 pop      rbp
0x1408c0fd4 jmp      0x1428d8810
0x1408c0fd9 call     0x1428d86b0
0x1408c0fde test     al, al