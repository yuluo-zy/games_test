0x1400b9f00 push     rsi
0x1400b9f01 push     rdi
0x1400b9f02 push     rbx
0x1400b9f03 sub      rsp, 0x110
0x1400b9f0a mov      rsi, rdx
0x1400b9f0d mov      rax, qword ptr [rcx]
0x1400b9f10 mov      ecx, dword ptr [rax + 0x1c]
0x1400b9f13 mov      rdx, qword ptr [rax + 0x10]
0x1400b9f17 mov      dword ptr [rdx], ecx
0x1400b9f19 mov      rdi, qword ptr [rax]
0x1400b9f1c movzx    eax, byte ptr [rip + 0x357ace6]
0x1400b9f23 mov      ecx, 0xc
0x1400b9f28 mov      edx, 1
0x1400b9f2d call     0x140613c10
0x1400b9f32 test     rax, rax
0x1400b9f35 je       0x1400ba494
0x1400b9f3b add      rdi, 5
0x1400b9f3f movabs   rcx, 0x6220656c62616e45
0x1400b9f49 mov      qword ptr [rax], rcx
0x1400b9f4c mov      dword ptr [rax + 8], 0x73647269
0x1400b9f53 mov      qword ptr [rsp + 0x50], 0xc
0x1400b9f5c mov      qword ptr [rsp + 0x58], rax
0x1400b9f61 mov      qword ptr [rsp + 0x60], 0xc
0x1400b9f6a mov      dword ptr [rsp + 0x68], 0
0x1400b9f72 mov      qword ptr [rsp + 0x78], 0
0x1400b9f7b mov      dword ptr [rsp + 0x70], 0
0x1400b9f83 mov      qword ptr [rsp + 0x20], 3
0x1400b9f8c mov      qword ptr [rsp + 0x38], 6
0x1400b9f95 mov      byte ptr [rsp + 0x80], 0
0x1400b9f9d mov      dword ptr [rsp + 0x85], 0
0x1400b9fa8 mov      dword ptr [rsp + 0x88], 0
0x1400b9fb3 mov      qword ptr [rsp + 0x90], rdi
0x1400b9fbb mov      byte ptr [rsp + 0x98], 0
0x1400b9fc3 lea      rcx, [rsp + 0xa0]
0x1400b9fcb lea      rdx, [rsp + 0x20]
0x1400b9fd0 mov      r8, rsi
0x1400b9fd3 call     0x140c48930
0x1400b9fd8 mov      rax, qword ptr [rsp + 0xb0]
0x1400b9fe0 lock dec qword ptr [rax]
0x1400b9fe4 jne      0x1400b9ff3
0x1400b9fe6 lea      rcx, [rsp + 0xb0]
0x1400b9fee call     0x140c41170
0x1400b9ff3 movzx    eax, byte ptr [rip + 0x357ac0f]
0x1400b9ffa mov      ecx, 0xa
0x1400b9fff mov      edx, 1
0x1400ba004 call     0x140613c10
0x1400ba009 test     rax, rax
0x1400ba00c je       0x1400ba4aa
0x1400ba012 movabs   rcx, 0x6f707320776f6853
0x1400ba01c mov      qword ptr [rax], rcx
0x1400ba01f mov      word ptr [rax + 8], 0x7374
0x1400ba025 mov      qword ptr [rsp + 0x50], 0xa
0x1400ba02e mov      qword ptr [rsp + 0x58], rax
0x1400ba033 mov      qword ptr [rsp + 0x60], 0xa
0x1400ba03c mov      dword ptr [rsp + 0x68], 0
0x1400ba044 mov      qword ptr [rsp + 0x78], 0
0x1400ba04d mov      dword ptr [rsp + 0x70], 0
0x1400ba055 mov      qword ptr [rsp + 0x20], 3
0x1400ba05e mov      qword ptr [rsp + 0x38], 6
0x1400ba067 mov      byte ptr [rsp + 0x80], 0
0x1400ba06f mov      dword ptr [rsp + 0x85], 0
0x1400ba07a mov      dword ptr [rsp + 0x88], 0
0x1400ba085 mov      rax, qword ptr [rip + 0x3571114] ; @0x14362b1a0 '!LcC'
0x1400ba08c mov      qword ptr [rsp + 0x90], rax
0x1400ba094 mov      byte ptr [rsp + 0x98], 0
0x1400ba09c lea      rcx, [rsp + 0xa0]
0x1400ba0a4 lea      rdx, [rsp + 0x20]
0x1400ba0a9 mov      r8, rsi
0x1400ba0ac call     0x140c48930
0x1400ba0b1 mov      rax, qword ptr [rsp + 0xb0]
0x1400ba0b9 lock dec qword ptr [rax]
0x1400ba0bd jne      0x1400ba0cc
0x1400ba0bf lea      rcx, [rsp + 0xb0]
0x1400ba0c7 call     0x140c41170
0x1400ba0cc movzx    eax, byte ptr [rip + 0x357ab36]
0x1400ba0d3 mov      ecx, 0xd
0x1400ba0d8 mov      edx, 1
0x1400ba0dd call     0x140613c10
0x1400ba0e2 test     rax, rax
0x1400ba0e5 je       0x1400ba4c0
0x1400ba0eb movabs   rcx, 0x7374736163796172
0x1400ba0f5 mov      qword ptr [rax + 5], rcx
0x1400ba0f9 movabs   rcx, 0x79617220776f6853
0x1400ba103 mov      qword ptr [rax], rcx
0x1400ba106 mov      qword ptr [rsp + 0x50], 0xd
0x1400ba10f mov      qword ptr [rsp + 0x58], rax
0x1400ba114 mov      qword ptr [rsp + 0x60], 0xd
0x1400ba11d mov      dword ptr [rsp + 0x68], 0
0x1400ba125 mov      qword ptr [rsp + 0x78], 0
0x1400ba12e mov      dword ptr [rsp + 0x70], 0
0x1400ba136 mov      qword ptr [rsp + 0x20], 3
0x1400ba13f mov      qword ptr [rsp + 0x38], 6
0x1400ba148 mov      byte ptr [rsp + 0x80], 0
0x1400ba150 mov      dword ptr [rsp + 0x85], 0
0x1400ba15b mov      dword ptr [rsp + 0x88], 0
0x1400ba166 mov      rax, qword ptr [rip + 0x357103b] ; @0x14362b1a8 '"LcC'
0x1400ba16d mov      qword ptr [rsp + 0x90], rax
0x1400ba175 mov      byte ptr [rsp + 0x98], 0
0x1400ba17d lea      rcx, [rsp + 0xa0]
0x1400ba185 lea      rdx, [rsp + 0x20]
0x1400ba18a mov      r8, rsi
0x1400ba18d call     0x140c48930
0x1400ba192 mov      rax, qword ptr [rsp + 0xb0]
0x1400ba19a lock dec qword ptr [rax]
0x1400ba19e jne      0x1400ba1ad
0x1400ba1a0 lea      rcx, [rsp + 0xb0]
0x1400ba1a8 call     0x140c41170
0x1400ba1ad movzx    eax, byte ptr [rip + 0x357aa55]
0x1400ba1b4 mov      ecx, 0x15
0x1400ba1b9 mov      edx, 1
0x1400ba1be call     0x140613c10
0x1400ba1c3 test     rax, rax
0x1400ba1c6 je       0x1400ba4d6
0x1400ba1cc movups   xmm0, xmmword ptr [rip + 0x287b2e5] ; @0x1429354b8 'Show haven populationAirborne: '
0x1400ba1d3 movups   xmmword ptr [rax], xmm0
0x1400ba1d6 movabs   rcx, 0x6e6f6974616c7570
0x1400ba1e0 mov      qword ptr [rax + 0xd], rcx
0x1400ba1e4 mov      qword ptr [rsp + 0x50], 0x15
0x1400ba1ed mov      qword ptr [rsp + 0x58], rax
0x1400ba1f2 mov      qword ptr [rsp + 0x60], 0x15
0x1400ba1fb mov      dword ptr [rsp + 0x68], 0
0x1400ba203 mov      qword ptr [rsp + 0x78], 0
0x1400ba20c mov      dword ptr [rsp + 0x70], 0
0x1400ba214 mov      qword ptr [rsp + 0x20], 3
0x1400ba21d mov      qword ptr [rsp + 0x38], 6
0x1400ba226 mov      byte ptr [rsp + 0x80], 0
0x1400ba22e mov      dword ptr [rsp + 0x85], 0
0x1400ba239 mov      dword ptr [rsp + 0x88], 0
0x1400ba244 mov      rax, qword ptr [rip + 0x3570f4d] ; @0x14362b198 ' LcC'
0x1400ba24b mov      qword ptr [rsp + 0x90], rax
0x1400ba253 mov      byte ptr [rsp + 0x98], 0
0x1400ba25b lea      rcx, [rsp + 0xa0]
0x1400ba263 lea      rdx, [rsp + 0x20]
0x1400ba268 mov      r8, rsi
0x1400ba26b call     0x140c48930
0x1400ba270 mov      rax, qword ptr [rsp + 0xb0]
0x1400ba278 lock dec qword ptr [rax]
0x1400ba27c jne      0x1400ba28b
0x1400ba27e lea      rcx, [rsp + 0xb0]
0x1400ba286 call     0x140c41170
0x1400ba28b mov      rax, qword ptr [rip + 0x3570ef6]
0x1400ba292 mov      qword ptr [rsp + 0x100], rax
0x1400ba29a lea      rdi, [rip + 0xcd341f]
0x1400ba2a1 mov      qword ptr [rsp + 0x108], rdi
0x1400ba2a9 lea      rax, [rip + 0x287b228]
0x1400ba2b0 mov      qword ptr [rsp + 0xa0], rax
0x1400ba2b8 mov      qword ptr [rsp + 0xa8], 1
0x1400ba2c4 lea      rbx, [rsp + 0x100]
0x1400ba2cc mov      qword ptr [rsp + 0xb0], rbx
0x1400ba2d4 mov      qword ptr [rsp + 0xb8], 1
0x1400ba2e0 mov      qword ptr [rsp + 0xc0], 0
0x1400ba2ec lea      rcx, [rsp + 0x50]
0x1400ba2f1 lea      rdx, [rsp + 0xa0]
0x1400ba2f9 call     0x140d79f90
0x1400ba2fe mov      byte ptr [rsp + 0x80], 0
0x1400ba306 mov      dword ptr [rsp + 0x85], 0
0x1400ba311 mov      dword ptr [rsp + 0x88], 0
0x1400ba31c mov      qword ptr [rsp + 0x20], 3
0x1400ba325 mov      qword ptr [rsp + 0x38], 6
0x1400ba32e mov      dword ptr [rsp + 0x68], 0
0x1400ba336 mov      dword ptr [rsp + 0x70], 0
0x1400ba33e mov      qword ptr [rsp + 0x78], 0
0x1400ba347 mov      word ptr [rsp + 0x90], 0x200
0x1400ba351 mov      byte ptr [rsp + 0x92], 2
0x1400ba359 mov      byte ptr [rsp + 0x95], 2
0x1400ba361 lea      rcx, [rsp + 0xa0]
0x1400ba369 lea      rdx, [rsp + 0x20]
0x1400ba36e mov      r8, rsi
0x1400ba371 call     0x140c28d40
0x1400ba376 mov      rax, qword ptr [rsp + 0xb0]
0x1400ba37e lock dec qword ptr [rax]
0x1400ba382 jne      0x1400ba391
0x1400ba384 lea      rcx, [rsp + 0xb0]
0x1400ba38c call     0x140c41170
0x1400ba391 mov      rax, qword ptr [rip + 0x3570df8]
0x1400ba398 mov      qword ptr [rsp + 0x100], rax
0x1400ba3a0 mov      qword ptr [rsp + 0x108], rdi
0x1400ba3a8 lea      rax, [rip + 0x287b149]
0x1400ba3af mov      qword ptr [rsp + 0xa0], rax
0x1400ba3b7 mov      qword ptr [rsp + 0xa8], 1
0x1400ba3c3 mov      qword ptr [rsp + 0xb0], rbx
0x1400ba3cb mov      qword ptr [rsp + 0xb8], 1
0x1400ba3d7 mov      qword ptr [rsp + 0xc0], 0
0x1400ba3e3 lea      rcx, [rsp + 0x50]
0x1400ba3e8 lea      rdx, [rsp + 0xa0]
0x1400ba3f0 call     0x140d79f90
0x1400ba3f5 mov      byte ptr [rsp + 0x80], 0
0x1400ba3fd mov      dword ptr [rsp + 0x85], 0
0x1400ba408 mov      dword ptr [rsp + 0x88], 0
0x1400ba413 mov      qword ptr [rsp + 0x20], 3
0x1400ba41c mov      qword ptr [rsp + 0x38], 6
0x1400ba425 mov      dword ptr [rsp + 0x68], 0
0x1400ba42d mov      dword ptr [rsp + 0x70], 0
0x1400ba435 mov      qword ptr [rsp + 0x78], 0
0x1400ba43e mov      word ptr [rsp + 0x90], 0x200
0x1400ba448 mov      byte ptr [rsp + 0x92], 2
0x1400ba450 mov      byte ptr [rsp + 0x95], 2
0x1400ba458 lea      rcx, [rsp + 0xa0]
0x1400ba460 lea      rdx, [rsp + 0x20]
0x1400ba465 mov      r8, rsi
0x1400ba468 call     0x140c28d40
0x1400ba46d mov      rax, qword ptr [rsp + 0xb0]
0x1400ba475 lock dec qword ptr [rax]
0x1400ba479 jne      0x1400ba488
0x1400ba47b lea      rcx, [rsp + 0xb0]
0x1400ba483 call     0x140c41170
0x1400ba488 nop      
0x1400ba489 add      rsp, 0x110
0x1400ba490 pop      rbx
0x1400ba491 pop      rdi
0x1400ba492 pop      rsi
0x1400ba493 ret      
0x1400ba494 lea      r8, [rip + 0x287a8ad]
0x1400ba49b mov      ecx, 1
0x1400ba4a0 mov      edx, 0xc
0x1400ba4a5 call     0x1428d8fe3
0x1400ba4aa lea      r8, [rip + 0x287a897]
0x1400ba4b1 mov      ecx, 1
0x1400ba4b6 mov      edx, 0xa
0x1400ba4bb call     0x1428d8fe3
0x1400ba4c0 lea      r8, [rip + 0x287a881]
0x1400ba4c7 mov      ecx, 1
0x1400ba4cc mov      edx, 0xd
0x1400ba4d1 call     0x1428d8fe3
0x1400ba4d6 lea      r8, [rip + 0x287a86b]
0x1400ba4dd mov      ecx, 1
0x1400ba4e2 mov      edx, 0x15
0x1400ba4e7 call     0x1428d8fe3