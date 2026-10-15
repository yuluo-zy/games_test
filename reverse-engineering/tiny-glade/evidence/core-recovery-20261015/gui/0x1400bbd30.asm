0x1400bbd30 push     rbp
0x1400bbd31 push     r14
0x1400bbd33 push     rsi
0x1400bbd34 push     rdi
0x1400bbd35 push     rbx
0x1400bbd36 sub      rsp, 0x320
0x1400bbd3d lea      rbp, [rsp + 0x80]
0x1400bbd45 mov      qword ptr [rbp + 0x298], 0xfffffffffffffffe
0x1400bbd50 mov      rsi, rdx
0x1400bbd53 mov      rdi, rcx
0x1400bbd56 mov      rbx, qword ptr [rcx]
0x1400bbd59 mov      rcx, qword ptr [rbx]
0x1400bbd5c call     0x1409114d0
0x1400bbd61 movss    dword ptr [rbp + 0x178], xmm0
0x1400bbd69 movzx    eax, byte ptr [rip + 0x3578e99]
0x1400bbd70 mov      ecx, 0x10
0x1400bbd75 mov      edx, 8
0x1400bbd7a call     0x140613c10
0x1400bbd7f test     rax, rax
0x1400bbd82 je       0x1400bc055
0x1400bbd88 lea      rcx, [rbp + 0x178]
0x1400bbd8f mov      qword ptr [rax], rcx
0x1400bbd92 mov      qword ptr [rax + 8], rbx
0x1400bbd96 lea      rcx, [rip + 0x2878c73]
0x1400bbd9d mov      qword ptr [rsp + 0x20], rcx
0x1400bbda2 lea      rcx, [rbp - 0x58]
0x1400bbda6 mov      rdx, rsi
0x1400bbda9 xor      r8d, r8d
0x1400bbdac mov      r9, rax
0x1400bbdaf call     0x1400c4230
0x1400bbdb4 mov      rax, qword ptr [rbp - 0x48]
0x1400bbdb8 lock dec qword ptr [rax]
0x1400bbdbc jne      0x1400bbdc7
0x1400bbdbe lea      rcx, [rbp - 0x48]
0x1400bbdc2 call     0x140c41170
0x1400bbdc7 lea      rdx, [rip + 0x287954e] ; @0x14293531c 'ModeSnapshotsDisable terrain hover highlightDisable wall snappingDisable eraser higlightHide bru'
0x1400bbdce lea      rcx, [rbp - 0x58]
0x1400bbdd2 mov      r8d, 4
0x1400bbdd8 call     0x1400ff0b0
0x1400bbddd mov      rbx, qword ptr [rdi + 8]
0x1400bbde1 mov      eax, 0x103
0x1400bbde6 add      rax, qword ptr [rbx]
0x1400bbde9 mov      qword ptr [rbp + 0xd0], rax
0x1400bbdf0 lea      rax, [rip + 0x8ab9]
0x1400bbdf7 mov      qword ptr [rbp + 0xd8], rax
0x1400bbdfe lea      rax, [rip + 0x287923b]
0x1400bbe05 mov      qword ptr [rbp + 0x178], rax
0x1400bbe0c mov      qword ptr [rbp + 0x180], 1
0x1400bbe17 lea      rax, [rbp + 0xd0]
0x1400bbe1e mov      qword ptr [rbp + 0x188], rax
0x1400bbe25 mov      qword ptr [rbp + 0x190], 1
0x1400bbe30 mov      qword ptr [rbp + 0x198], 0
0x1400bbe3b mov      byte ptr [rbp + 0x297], 1
0x1400bbe42 lea      rcx, [rbp + 0x160]
0x1400bbe49 lea      rdx, [rbp + 0x178]
0x1400bbe50 call     0x140d79f90
0x1400bbe55 mov      byte ptr [rbp + 0x297], 0
0x1400bbe5c lea      rcx, [rbp + 0x178]
0x1400bbe63 lea      rdx, [rbp - 0x58]
0x1400bbe67 lea      r8, [rbp + 0x160]
0x1400bbe6e call     0x1400c0fd0
0x1400bbe73 lea      rcx, [rbp - 0x58]
0x1400bbe77 lea      rdx, [rbp + 0x178]
0x1400bbe7e mov      r8d, 0x110
0x1400bbe84 call     0x142923c00
0x1400bbe89 movzx    eax, byte ptr [rip + 0x3578d79]
0x1400bbe90 mov      ecx, 8
0x1400bbe95 mov      edx, 8
0x1400bbe9a call     0x140613c10
0x1400bbe9f test     rax, rax
0x1400bbea2 mov      qword ptr [rbp + 0x288], rax
0x1400bbea9 je       0x1400bc064
0x1400bbeaf mov      r9, rax
0x1400bbeb2 mov      qword ptr [rax], rbx
0x1400bbeb5 lea      rax, [rip + 0x2890d1c]
0x1400bbebc mov      qword ptr [rsp + 0x20], rax
0x1400bbec1 lea      rcx, [rbp + 0xd0]
0x1400bbec8 lea      rdx, [rbp + 0x178]
0x1400bbecf mov      r8, rsi
0x1400bbed2 call     0x1400ff3e0
0x1400bbed7 mov      rax, qword ptr [rbp + 0xe0]
0x1400bbede lock dec qword ptr [rax]
0x1400bbee2 jne      0x1400bbef0
0x1400bbee4 lea      rcx, [rbp + 0xe0]
0x1400bbeeb call     0x140c41170
0x1400bbef0 mov      rdi, qword ptr [rdi + 0x10]
0x1400bbef4 lea      rdx, [rip + 0x2879425] ; @0x142935320 'SnapshotsDisable terrain hover highlightDisable wall snappingDisable eraser higlightHide brush p'
0x1400bbefb lea      rcx, [rbp + 0xd0]
0x1400bbf02 mov      r8d, 9
0x1400bbf08 call     0x14015eea0
0x1400bbf0d movzx    eax, byte ptr [rip + 0x3578cf5]
0x1400bbf14 mov      ecx, 0x10
0x1400bbf19 mov      edx, 8
0x1400bbf1e call     0x140613c10
0x1400bbf23 test     rax, rax
0x1400bbf26 mov      qword ptr [rbp + 0x288], rax
0x1400bbf2d je       0x1400bc075
0x1400bbf33 mov      r14, rax
0x1400bbf36 mov      qword ptr [rax], rbx
0x1400bbf39 mov      qword ptr [rax + 8], rdi
0x1400bbf3d mov      byte ptr [rbp + 0x296], 1
0x1400bbf44 lea      rcx, [rbp + 0x178]
0x1400bbf4b lea      rdx, [rbp + 0xd0]
0x1400bbf52 mov      r8d, 0x90
0x1400bbf58 call     0x142923c00
0x1400bbf5d lea      rax, [rbp + 0x296]
0x1400bbf64 mov      qword ptr [rbp + 0x208], rax
0x1400bbf6b mov      qword ptr [rbp + 0x210], r14
0x1400bbf72 lea      rax, [rip + 0x2878837]
0x1400bbf79 mov      qword ptr [rbp + 0x218], rax
0x1400bbf80 movzx    eax, byte ptr [rip + 0x3578c82]
0x1400bbf87 mov      ecx, 0xa8
0x1400bbf8c mov      edx, 8
0x1400bbf91 call     0x140613c10
0x1400bbf96 test     rax, rax
0x1400bbf99 je       0x1400bc086
0x1400bbf9f mov      rdi, rax
0x1400bbfa2 lea      rdx, [rbp + 0x178]
0x1400bbfa9 mov      r8d, 0xa8
0x1400bbfaf mov      rcx, rax
0x1400bbfb2 call     0x142923c00
0x1400bbfb7 lea      rax, [rip + 0x28a0842]
0x1400bbfbe mov      qword ptr [rsp + 0x20], rax
0x1400bbfc3 lea      rcx, [rbp - 0x58]
0x1400bbfc7 movabs   r8, 0x20001000000
0x1400bbfd1 mov      rdx, rsi
0x1400bbfd4 mov      r9, rdi
0x1400bbfd7 call     0x1400c2f70
0x1400bbfdc lea      rcx, [rbp + 0x178]
0x1400bbfe3 lea      rdx, [rbp - 0x58]
0x1400bbfe7 mov      r8d, 0xc8
0x1400bbfed call     0x142923c00
0x1400bbff2 mov      rax, qword ptr [rbp + 0x80]
0x1400bbff9 lock dec qword ptr [rax]
0x1400bbffd jne      0x1400bc00b
0x1400bbfff lea      rcx, [rbp + 0x80]
0x1400bc006 call     0x140c41170
0x1400bc00b mov      rax, qword ptr [rbp + 0x1e8]
0x1400bc012 lock dec qword ptr [rax]
0x1400bc016 jne      0x1400bc024
0x1400bc018 lea      rcx, [rbp + 0x1e8]
0x1400bc01f call     0x140c41170
0x1400bc024 cmp      dword ptr [rbp + 0x1b8], 2
0x1400bc02b je       0x1400bc046
0x1400bc02d mov      rax, qword ptr [rbp + 0x188]
0x1400bc034 lock dec qword ptr [rax]
0x1400bc038 jne      0x1400bc046
0x1400bc03a lea      rcx, [rbp + 0x188]
0x1400bc041 call     0x140c41170
0x1400bc046 nop      
0x1400bc047 add      rsp, 0x320
0x1400bc04e pop      rbx
0x1400bc04f pop      rdi
0x1400bc050 pop      rsi
0x1400bc051 pop      r14
0x1400bc053 pop      rbp
0x1400bc054 ret      
0x1400bc055 mov      ecx, 8
0x1400bc05a mov      edx, 0x10
0x1400bc05f call     0x1428d9000
0x1400bc064 mov      ecx, 8
0x1400bc069 mov      edx, 8
0x1400bc06e call     0x1428d9000
0x1400bc073 jmp      0x1400bc095
0x1400bc075 mov      ecx, 8
0x1400bc07a mov      edx, 0x10
0x1400bc07f call     0x1428d9000
0x1400bc084 jmp      0x1400bc095
0x1400bc086 mov      ecx, 8
0x1400bc08b mov      edx, 0xa8