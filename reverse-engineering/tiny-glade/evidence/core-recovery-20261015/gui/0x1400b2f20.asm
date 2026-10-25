0x1400b2f20 push     rbp
0x1400b2f21 push     r15
0x1400b2f23 push     r14
0x1400b2f25 push     r13
0x1400b2f27 push     r12
0x1400b2f29 push     rsi
0x1400b2f2a push     rdi
0x1400b2f2b push     rbx
0x1400b2f2c sub      rsp, 0x358
0x1400b2f33 lea      rbp, [rsp + 0x80]
0x1400b2f3b movaps   xmmword ptr [rbp + 0x2c0], xmm6
0x1400b2f42 mov      qword ptr [rbp + 0x2b8], 0xfffffffffffffffe
0x1400b2f4d mov      rsi, rdx
0x1400b2f50 mov      rax, qword ptr [rcx]
0x1400b2f53 mov      rbx, qword ptr [rcx + 8]
0x1400b2f57 mov      r15d, dword ptr [rax + 0x1c]
0x1400b2f5b mov      r12, qword ptr [rax + 0x10]
0x1400b2f5f mov      dword ptr [r12], r15d
0x1400b2f63 mov      r14, qword ptr [rax]
0x1400b2f66 movzx    eax, byte ptr [rip + 0x3581c9c]
0x1400b2f6d mov      ecx, 8
0x1400b2f72 mov      edx, 8
0x1400b2f77 call     0x140613c10
0x1400b2f7c test     rax, rax
0x1400b2f7f je       0x1400b3b5d
0x1400b2f85 mov      qword ptr [rbp + 0x2b0], rax
0x1400b2f8c mov      qword ptr [rax], r14
0x1400b2f8f lea      rcx, [rbp + 0xe8]
0x1400b2f96 call     0x140c34750
0x1400b2f9b mov      qword ptr [rbp + 0x1f8], 0
0x1400b2fa6 mov      qword ptr [rbp + 0x200], 1
0x1400b2fb1 xorps    xmm0, xmm0
0x1400b2fb4 movups   xmmword ptr [rbp + 0x208], xmm0
0x1400b2fbb mov      qword ptr [rbp + 0x218], 1
0x1400b2fc6 mov      qword ptr [rbp + 0x220], 0
0x1400b2fd1 movups   xmm1, xmmword ptr [rbp + 0xe8]
0x1400b2fd8 movups   xmm2, xmmword ptr [rbp + 0xf8]
0x1400b2fdf movups   xmm3, xmmword ptr [rbp + 0x108]
0x1400b2fe6 movups   xmm4, xmmword ptr [rbp + 0x118]
0x1400b2fed movups   xmmword ptr [rbp - 0x58], xmm1
0x1400b2ff1 movups   xmmword ptr [rbp - 0x48], xmm2
0x1400b2ff5 movups   xmmword ptr [rbp - 0x38], xmm3
0x1400b2ff9 movups   xmmword ptr [rbp - 0x28], xmm4
0x1400b2ffd movups   xmm1, xmmword ptr [rbp + 0x128]
0x1400b3004 movups   xmmword ptr [rbp - 0x18], xmm1
0x1400b3008 movups   xmm1, xmmword ptr [rbp + 0x138]
0x1400b300f movups   xmmword ptr [rbp - 8], xmm1
0x1400b3013 movups   xmm1, xmmword ptr [rbp + 0x148]
0x1400b301a movups   xmmword ptr [rbp + 8], xmm1
0x1400b301e movups   xmmword ptr [rbp + 0x270], xmm0
0x1400b3025 mov      qword ptr [rbp + 0x158], 0
0x1400b3030 mov      qword ptr [rbp + 0x168], 0
0x1400b303b mov      qword ptr [rbp + 0x178], 0
0x1400b3046 movups   xmm1, xmmword ptr [rbp - 0x60]
0x1400b304a movups   xmm2, xmmword ptr [rbp - 0x50]
0x1400b304e movups   xmm3, xmmword ptr [rbp - 0x40]
0x1400b3052 movups   xmm4, xmmword ptr [rbp - 0x30]
0x1400b3056 movups   xmmword ptr [rbp + 0x180], xmm1
0x1400b305d movups   xmmword ptr [rbp + 0x190], xmm2
0x1400b3064 movups   xmmword ptr [rbp + 0x1a0], xmm3
0x1400b306b movups   xmmword ptr [rbp + 0x1b0], xmm4
0x1400b3072 movups   xmm1, xmmword ptr [rbp - 0x20]
0x1400b3076 movups   xmmword ptr [rbp + 0x1c0], xmm1
0x1400b307d movups   xmm1, xmmword ptr [rbp - 0x10]
0x1400b3081 movups   xmmword ptr [rbp + 0x1d0], xmm1
0x1400b3088 movups   xmm1, xmmword ptr [rbp]
0x1400b308c movups   xmmword ptr [rbp + 0x1e0], xmm1
0x1400b3093 mov      rax, qword ptr [rbp + 0x10]
0x1400b3097 mov      qword ptr [rbp + 0x1f0], rax
0x1400b309e mov      dword ptr [rbp + 0x228], 2
0x1400b30a8 movhps   xmm0, qword ptr [rip + 0x2881531]
0x1400b30af movups   xmmword ptr [rbp + 0x230], xmm0
0x1400b30b6 mov      byte ptr [rbp + 0x240], 0
0x1400b30bd movaps   xmm6, xmmword ptr [rip + 0x288150c]
0x1400b30c4 movups   xmmword ptr [rbp + 0x248], xmm6
0x1400b30cb mov      byte ptr [rbp + 0x258], 0
0x1400b30d2 mov      rax, qword ptr [rbp + 0x2b0]
0x1400b30d9 mov      qword ptr [rbp + 0x260], rax
0x1400b30e0 lea      r13, [rip + 0x294efb1]
0x1400b30e7 mov      qword ptr [rbp + 0x268], r13
0x1400b30ee mov      qword ptr [rbp + 0x288], 0
0x1400b30f9 mov      dword ptr [rbp + 0x298], 0x10101
0x1400b3103 mov      byte ptr [rbp + 0x29c], 2
0x1400b310a movzx    eax, byte ptr [rip + 0x3581af8]
0x1400b3111 mov      ecx, 0xd
0x1400b3116 mov      edx, 1
0x1400b311b call     0x140613c10
0x1400b3120 test     rax, rax
0x1400b3123 je       0x1400b3b6c
0x1400b3129 movabs   rcx, 0x7373656e6b636968
0x1400b3133 mov      qword ptr [rax + 5], rcx
0x1400b3137 movabs   rcx, 0x6369687420676f46
0x1400b3141 mov      qword ptr [rax], rcx
0x1400b3144 mov      dword ptr [rbp + 0x2ac], 0
0x1400b314e mov      dword ptr [rbp + 0x2a9], 0
0x1400b3158 mov      rdi, qword ptr [rbp + 0x188]
0x1400b315f mov      ecx, edi
0x1400b3161 and      ecx, 0xfffffffe
0x1400b3164 lea      rdx, [rdi - 3]
0x1400b3168 xor      r8d, r8d
0x1400b316b cmp      ecx, 4
0x1400b316e cmove    r8, rdx
0x1400b3172 test     r8, r8
0x1400b3175 mov      qword ptr [rbp + 0x2b0], rax
0x1400b317c je       0x1400b31b7
0x1400b317e cmp      r8, 1
0x1400b3182 jne      0x1400b3225
0x1400b3188 mov      rdx, qword ptr [rbp + 0x190]
0x1400b318f test     rdx, rdx
0x1400b3192 je       0x1400b31a6
0x1400b3194 mov      rcx, qword ptr [rbp + 0x198]
0x1400b319b mov      r8d, 1
0x1400b31a1 call     0x140613c20
0x1400b31a6 lea      rcx, [rbp + 0x1a8]
0x1400b31ad call     0x140608f80
0x1400b31b2 jmp      0x1400b323e
0x1400b31b7 mov      rdx, qword ptr [rbp + 0x1b8]
0x1400b31be test     rdx, rdx
0x1400b31c1 je       0x1400b31d5
0x1400b31c3 mov      rcx, qword ptr [rbp + 0x1c0]
0x1400b31ca mov      r8d, 1
0x1400b31d0 call     0x140613c20
0x1400b31d5 cmp      edi, 3
0x1400b31d8 je       0x1400b31f8
0x1400b31da cmp      edi, 2
0x1400b31dd jb       0x1400b31f8
0x1400b31df mov      rax, qword ptr [rbp + 0x190]
0x1400b31e6 lock dec qword ptr [rax]
0x1400b31ea jne      0x1400b31f8
0x1400b31ec lea      rcx, [rbp + 0x190]
0x1400b31f3 call     0x14064eb30
0x1400b31f8 mov      rax, qword ptr [rbp + 0x1a0]
0x1400b31ff cmp      rax, 6
0x1400b3203 je       0x1400b323e
0x1400b3205 cmp      eax, 5
0x1400b3208 jb       0x1400b323e
0x1400b320a mov      rax, qword ptr [rbp + 0x1a8]
0x1400b3211 lock dec qword ptr [rax]
0x1400b3215 jne      0x1400b323e
0x1400b3217 lea      rcx, [rbp + 0x1a8]
0x1400b321e call     0x14064eb30
0x1400b3223 jmp      0x1400b323e
0x1400b3225 mov      rax, qword ptr [rbp + 0x190]
0x1400b322c lock dec qword ptr [rax]
0x1400b3230 jne      0x1400b323e
0x1400b3232 lea      rcx, [rbp + 0x190]
0x1400b3239 call     0x140c7de90
0x1400b323e mov      qword ptr [rbp + 0x188], 3
0x1400b3249 movups   xmm0, xmmword ptr [rbp - 0x60]
0x1400b324d movups   xmmword ptr [rbp + 0x190], xmm0
0x1400b3254 mov      qword ptr [rbp + 0x1a0], 6
0x1400b325f movups   xmm0, xmmword ptr [rbp + 0xe8]
0x1400b3266 movups   xmmword ptr [rbp + 0x1a8], xmm0
0x1400b326d mov      qword ptr [rbp + 0x1b8], 0xd
0x1400b3278 mov      rax, qword ptr [rbp + 0x2b0]
0x1400b327f mov      qword ptr [rbp + 0x1c0], rax
0x1400b3286 mov      qword ptr [rbp + 0x1c8], 0xd
0x1400b3291 mov      dword ptr [rbp + 0x1d0], 0
0x1400b329b mov      dword ptr [rbp + 0x1d8], 0
0x1400b32a5 mov      qword ptr [rbp + 0x1e0], 0
0x1400b32b0 mov      byte ptr [rbp + 0x1e8], 0
0x1400b32b7 mov      rax, qword ptr [rbp + 0x2a5]
0x1400b32be mov      qword ptr [rbp + 0x1e9], rax
0x1400b32c5 mov      eax, dword ptr [rbp + 0x2ac]
0x1400b32cb mov      dword ptr [rbp + 0x1f0], eax
0x1400b32d1 lea      rdi, [rbp - 0x60]
0x1400b32d5 lea      rdx, [rbp + 0x158]
0x1400b32dc mov      r8d, 0x148
0x1400b32e2 mov      rcx, rdi
0x1400b32e5 call     0x142923c00
0x1400b32ea lea      rcx, [rbp + 0x158]
0x1400b32f1 mov      rdx, rdi
0x1400b32f4 mov      r8, rsi
0x1400b32f7 call     0x140c77160
0x1400b32fc mov      rax, qword ptr [rbp + 0x168]
0x1400b3303 lock dec qword ptr [rax]
0x1400b3307 jne      0x1400b3315
0x1400b3309 lea      rcx, [rbp + 0x168]
0x1400b3310 call     0x140c41170
0x1400b3315 mov      dword ptr [r12], r15d
0x1400b3319 movzx    eax, byte ptr [rip + 0x35818e9]
0x1400b3320 mov      ecx, 8
0x1400b3325 mov      edx, 8
0x1400b332a call     0x140613c10
0x1400b332f test     rax, rax
0x1400b3332 je       0x1400b3b5d
0x1400b3338 lea      rcx, [r14 + 8]
0x1400b333c mov      qword ptr [rbp + 0x2b0], rax
0x1400b3343 mov      qword ptr [rax], rcx
0x1400b3346 lea      rcx, [rbp + 0xe8]
0x1400b334d call     0x140c34750
0x1400b3352 mov      qword ptr [rbp + 0x1f8], 0
0x1400b335d mov      qword ptr [rbp + 0x200], 1
0x1400b3368 xorps    xmm0, xmm0
0x1400b336b movups   xmmword ptr [rbp + 0x208], xmm0
0x1400b3372 mov      qword ptr [rbp + 0x218], 1
0x1400b337d mov      qword ptr [rbp + 0x220], 0
0x1400b3388 movups   xmm1, xmmword ptr [rbp + 0xe8]
0x1400b338f movups   xmm2, xmmword ptr [rbp + 0xf8]
0x1400b3396 movups   xmm3, xmmword ptr [rbp + 0x108]
0x1400b339d movups   xmm4, xmmword ptr [rbp + 0x118]
0x1400b33a4 movups   xmmword ptr [rbp - 0x58], xmm1
0x1400b33a8 movups   xmmword ptr [rbp - 0x48], xmm2
0x1400b33ac movups   xmmword ptr [rbp - 0x38], xmm3
0x1400b33b0 movups   xmmword ptr [rbp - 0x28], xmm4
0x1400b33b4 movups   xmm1, xmmword ptr [rbp + 0x128]
0x1400b33bb movups   xmmword ptr [rbp - 0x18], xmm1
0x1400b33bf movups   xmm1, xmmword ptr [rbp + 0x138]
0x1400b33c6 movups   xmmword ptr [rbp - 8], xmm1
0x1400b33ca movups   xmm1, xmmword ptr [rbp + 0x148]
0x1400b33d1 movups   xmmword ptr [rbp + 8], xmm1
0x1400b33d5 movups   xmmword ptr [rbp + 0x270], xmm0
0x1400b33dc mov      qword ptr [rbp + 0x158], 0
0x1400b33e7 mov      qword ptr [rbp + 0x168], 0
0x1400b33f2 mov      qword ptr [rbp + 0x178], 0
0x1400b33fd movups   xmm1, xmmword ptr [rbp - 0x60]
0x1400b3401 movups   xmm2, xmmword ptr [rbp - 0x50]
0x1400b3405 movups   xmm3, xmmword ptr [rbp - 0x40]
0x1400b3409 movups   xmm4, xmmword ptr [rbp - 0x30]
0x1400b340d movups   xmmword ptr [rbp + 0x180], xmm1
0x1400b3414 movups   xmmword ptr [rbp + 0x190], xmm2
0x1400b341b movups   xmmword ptr [rbp + 0x1a0], xmm3
0x1400b3422 movups   xmmword ptr [rbp + 0x1b0], xmm4
0x1400b3429 movups   xmm1, xmmword ptr [rbp - 0x20]
0x1400b342d movups   xmmword ptr [rbp + 0x1c0], xmm1
0x1400b3434 movups   xmm1, xmmword ptr [rbp - 0x10]
0x1400b3438 movups   xmmword ptr [rbp + 0x1d0], xmm1
0x1400b343f movups   xmm1, xmmword ptr [rbp]
0x1400b3443 movups   xmmword ptr [rbp + 0x1e0], xmm1
0x1400b344a mov      rax, qword ptr [rbp + 0x10]
0x1400b344e mov      qword ptr [rbp + 0x1f0], rax
0x1400b3455 mov      dword ptr [rbp + 0x228], 2
0x1400b345f movhps   xmm0, qword ptr [rip + 0x2881182]
0x1400b3466 movups   xmmword ptr [rbp + 0x230], xmm0
0x1400b346d mov      byte ptr [rbp + 0x240], 0
0x1400b3474 movups   xmmword ptr [rbp + 0x248], xmm6
0x1400b347b mov      byte ptr [rbp + 0x258], 0
0x1400b3482 mov      rax, qword ptr [rbp + 0x2b0]
0x1400b3489 mov      qword ptr [rbp + 0x260], rax
0x1400b3490 mov      qword ptr [rbp + 0x268], r13
0x1400b3497 mov      qword ptr [rbp + 0x288], 0
0x1400b34a2 mov      dword ptr [rbp + 0x298], 0x10101
0x1400b34ac mov      byte ptr [rbp + 0x29c], 2
0x1400b34b3 movzx    eax, byte ptr [rip + 0x358174f]
0x1400b34ba mov      ecx, 0xd
0x1400b34bf mov      edx, 1
0x1400b34c4 call     0x140613c10
0x1400b34c9 test     rax, rax
0x1400b34cc je       0x1400b3b84
0x1400b34d2 movabs   r13, 0x6874676e65727473
0x1400b34dc mov      qword ptr [rax + 5], r13
0x1400b34e0 movabs   rcx, 0x727473206e696152
0x1400b34ea mov      qword ptr [rax], rcx
0x1400b34ed mov      dword ptr [rbp + 0x2ac], 0
0x1400b34f7 mov      dword ptr [rbp + 0x2a9], 0
0x1400b3501 mov      rdi, qword ptr [rbp + 0x188]
0x1400b3508 mov      ecx, edi
0x1400b350a and      ecx, 0xfffffffe
0x1400b350d lea      rdx, [rdi - 3]
0x1400b3511 xor      r8d, r8d
0x1400b3514 cmp      ecx, 4
0x1400b3517 cmove    r8, rdx
0x1400b351b test     r8, r8
0x1400b351e mov      qword ptr [rbp + 0x2b0], rax
0x1400b3525 je       0x1400b3560
0x1400b3527 cmp      r8, 1
0x1400b352b jne      0x1400b35ce
0x1400b3531 mov      rdx, qword ptr [rbp + 0x190]
0x1400b3538 test     rdx, rdx
0x1400b353b je       0x1400b354f
0x1400b353d mov      rcx, qword ptr [rbp + 0x198]
0x1400b3544 mov      r8d, 1
0x1400b354a call     0x140613c20
0x1400b354f lea      rcx, [rbp + 0x1a8]
0x1400b3556 call     0x140608f80
0x1400b355b jmp      0x1400b35e7
0x1400b3560 mov      rdx, qword ptr [rbp + 0x1b8]
0x1400b3567 test     rdx, rdx
0x1400b356a je       0x1400b357e
0x1400b356c mov      rcx, qword ptr [rbp + 0x1c0]
0x1400b3573 mov      r8d, 1
0x1400b3579 call     0x140613c20
0x1400b357e cmp      edi, 3
0x1400b3581 je       0x1400b35a1
0x1400b3583 cmp      edi, 2
0x1400b3586 jb       0x1400b35a1
0x1400b3588 mov      rax, qword ptr [rbp + 0x190]
0x1400b358f lock dec qword ptr [rax]
0x1400b3593 jne      0x1400b35a1
0x1400b3595 lea      rcx, [rbp + 0x190]
0x1400b359c call     0x14064eb30
0x1400b35a1 mov      rax, qword ptr [rbp + 0x1a0]
0x1400b35a8 cmp      rax, 6
0x1400b35ac je       0x1400b35e7
0x1400b35ae cmp      eax, 5
0x1400b35b1 jb       0x1400b35e7
0x1400b35b3 mov      rax, qword ptr [rbp + 0x1a8]
0x1400b35ba lock dec qword ptr [rax]
0x1400b35be jne      0x1400b35e7
0x1400b35c0 lea      rcx, [rbp + 0x1a8]
0x1400b35c7 call     0x14064eb30
0x1400b35cc jmp      0x1400b35e7
0x1400b35ce mov      rax, qword ptr [rbp + 0x190]
0x1400b35d5 lock dec qword ptr [rax]
0x1400b35d9 jne      0x1400b35e7
0x1400b35db lea      rcx, [rbp + 0x190]
0x1400b35e2 call     0x140c7de90
0x1400b35e7 mov      qword ptr [rbp + 0x188], 3
0x1400b35f2 movups   xmm0, xmmword ptr [rbp - 0x60]
0x1400b35f6 movups   xmmword ptr [rbp + 0x190], xmm0
0x1400b35fd mov      qword ptr [rbp + 0x1a0], 6
0x1400b3608 movups   xmm0, xmmword ptr [rbp + 0xe8]
0x1400b360f movups   xmmword ptr [rbp + 0x1a8], xmm0
0x1400b3616 mov      qword ptr [rbp + 0x1b8], 0xd
0x1400b3621 mov      rax, qword ptr [rbp + 0x2b0]
0x1400b3628 mov      qword ptr [rbp + 0x1c0], rax
0x1400b362f mov      qword ptr [rbp + 0x1c8], 0xd
0x1400b363a mov      dword ptr [rbp + 0x1d0], 0
0x1400b3644 mov      dword ptr [rbp + 0x1d8], 0
0x1400b364e mov      qword ptr [rbp + 0x1e0], 0
0x1400b3659 mov      byte ptr [rbp + 0x1e8], 0
0x1400b3660 mov      rax, qword ptr [rbp + 0x2a5]
0x1400b3667 mov      qword ptr [rbp + 0x1e9], rax
0x1400b366e mov      eax, dword ptr [rbp + 0x2ac]
0x1400b3674 mov      dword ptr [rbp + 0x1f0], eax
0x1400b367a lea      rdi, [rbp - 0x60]
0x1400b367e lea      rdx, [rbp + 0x158]
0x1400b3685 mov      r8d, 0x148
0x1400b368b mov      rcx, rdi
0x1400b368e call     0x142923c00
0x1400b3693 lea      rcx, [rbp + 0x158]
0x1400b369a mov      rdx, rdi
0x1400b369d mov      r8, rsi
0x1400b36a0 call     0x140c77160
0x1400b36a5 mov      rax, qword ptr [rbp + 0x168]
0x1400b36ac lock dec qword ptr [rax]
0x1400b36b0 jne      0x1400b36be
0x1400b36b2 lea      rcx, [rbp + 0x168]
0x1400b36b9 call     0x140c41170
0x1400b36be mov      dword ptr [r12], r15d
0x1400b36c2 movzx    eax, byte ptr [rip + 0x3581540]
0x1400b36c9 mov      ecx, 8
0x1400b36ce mov      edx, 8
0x1400b36d3 call     0x140613c10
0x1400b36d8 test     rax, rax
0x1400b36db je       0x1400b3b5d
0x1400b36e1 add      r14, 0xc
0x1400b36e5 mov      qword ptr [rbp + 0x2b0], rax
0x1400b36ec mov      qword ptr [rax], r14
0x1400b36ef lea      rcx, [rbp + 0xe8]
0x1400b36f6 call     0x140c34750
0x1400b36fb mov      qword ptr [rbp + 0x1f8], 0
0x1400b3706 mov      qword ptr [rbp + 0x200], 1
0x1400b3711 xorps    xmm0, xmm0
0x1400b3714 movups   xmmword ptr [rbp + 0x208], xmm0
0x1400b371b mov      qword ptr [rbp + 0x218], 1
0x1400b3726 mov      qword ptr [rbp + 0x220], 0
0x1400b3731 movups   xmm1, xmmword ptr [rbp + 0xe8]
0x1400b3738 movups   xmm2, xmmword ptr [rbp + 0xf8]
0x1400b373f movups   xmm3, xmmword ptr [rbp + 0x108]
0x1400b3746 movups   xmm4, xmmword ptr [rbp + 0x118]
0x1400b374d movups   xmmword ptr [rbp - 0x58], xmm1
0x1400b3751 movups   xmmword ptr [rbp - 0x48], xmm2
0x1400b3755 movups   xmmword ptr [rbp - 0x38], xmm3
0x1400b3759 movups   xmmword ptr [rbp - 0x28], xmm4
0x1400b375d movups   xmm1, xmmword ptr [rbp + 0x128]
0x1400b3764 movups   xmmword ptr [rbp - 0x18], xmm1
0x1400b3768 movups   xmm1, xmmword ptr [rbp + 0x138]
0x1400b376f movups   xmmword ptr [rbp - 8], xmm1
0x1400b3773 movups   xmm1, xmmword ptr [rbp + 0x148]
0x1400b377a movups   xmmword ptr [rbp + 8], xmm1
0x1400b377e movups   xmmword ptr [rbp + 0x270], xmm0
0x1400b3785 mov      qword ptr [rbp + 0x158], 0
0x1400b3790 mov      qword ptr [rbp + 0x168], 0
0x1400b379b mov      qword ptr [rbp + 0x178], 0
0x1400b37a6 movups   xmm1, xmmword ptr [rbp - 0x60]
0x1400b37aa movups   xmm2, xmmword ptr [rbp - 0x50]
0x1400b37ae movups   xmm3, xmmword ptr [rbp - 0x40]
0x1400b37b2 movups   xmm4, xmmword ptr [rbp - 0x30]
0x1400b37b6 movups   xmmword ptr [rbp + 0x180], xmm1
0x1400b37bd movups   xmmword ptr [rbp + 0x190], xmm2
0x1400b37c4 movups   xmmword ptr [rbp + 0x1a0], xmm3
0x1400b37cb movups   xmmword ptr [rbp + 0x1b0], xmm4
0x1400b37d2 movups   xmm1, xmmword ptr [rbp - 0x20]
0x1400b37d6 movups   xmmword ptr [rbp + 0x1c0], xmm1
0x1400b37dd movups   xmm1, xmmword ptr [rbp - 0x10]
0x1400b37e1 movups   xmmword ptr [rbp + 0x1d0], xmm1
0x1400b37e8 movups   xmm1, xmmword ptr [rbp]
0x1400b37ec movups   xmmword ptr [rbp + 0x1e0], xmm1
0x1400b37f3 mov      rax, qword ptr [rbp + 0x10]
0x1400b37f7 mov      qword ptr [rbp + 0x1f0], rax
0x1400b37fe mov      dword ptr [rbp + 0x228], 2
0x1400b3808 movhps   xmm0, qword ptr [rip + 0x2880dd9]
0x1400b380f movups   xmmword ptr [rbp + 0x230], xmm0
0x1400b3816 mov      byte ptr [rbp + 0x240], 0
0x1400b381d movups   xmmword ptr [rbp + 0x248], xmm6
0x1400b3824 mov      byte ptr [rbp + 0x258], 0
0x1400b382b mov      rax, qword ptr [rbp + 0x2b0]
0x1400b3832 mov      qword ptr [rbp + 0x260], rax
0x1400b3839 lea      rax, [rip + 0x294e858]
0x1400b3840 mov      qword ptr [rbp + 0x268], rax
0x1400b3847 mov      qword ptr [rbp + 0x288], 0
0x1400b3852 mov      dword ptr [rbp + 0x298], 0x10101
0x1400b385c mov      byte ptr [rbp + 0x29c], 2
0x1400b3863 movzx    eax, byte ptr [rip + 0x358139f]
0x1400b386a mov      ecx, 0xd
0x1400b386f mov      edx, 1
0x1400b3874 call     0x140613c10
0x1400b3879 test     rax, rax
0x1400b387c je       0x1400b3b9c
0x1400b3882 mov      qword ptr [rax + 5], r13
0x1400b3886 movabs   rcx, 0x72747320776f6e53
0x1400b3890 mov      qword ptr [rax], rcx
0x1400b3893 mov      dword ptr [rbp + 0x2ac], 0
0x1400b389d mov      dword ptr [rbp + 0x2a9], 0
0x1400b38a7 mov      rdi, qword ptr [rbp + 0x188]
0x1400b38ae mov      ecx, edi
0x1400b38b0 and      ecx, 0xfffffffe
0x1400b38b3 lea      rdx, [rdi - 3]
0x1400b38b7 xor      r8d, r8d
0x1400b38ba cmp      ecx, 4
0x1400b38bd cmove    r8, rdx
0x1400b38c1 test     r8, r8
0x1400b38c4 mov      qword ptr [rbp + 0x2b0], rax
0x1400b38cb je       0x1400b3906
0x1400b38cd cmp      r8, 1
0x1400b38d1 jne      0x1400b3974
0x1400b38d7 mov      rdx, qword ptr [rbp + 0x190]
0x1400b38de test     rdx, rdx
0x1400b38e1 je       0x1400b38f5
0x1400b38e3 mov      rcx, qword ptr [rbp + 0x198]
0x1400b38ea mov      r8d, 1
0x1400b38f0 call     0x140613c20
0x1400b38f5 lea      rcx, [rbp + 0x1a8]
0x1400b38fc call     0x140608f80
0x1400b3901 jmp      0x1400b398d
0x1400b3906 mov      rdx, qword ptr [rbp + 0x1b8]
0x1400b390d test     rdx, rdx
0x1400b3910 je       0x1400b3924
0x1400b3912 mov      rcx, qword ptr [rbp + 0x1c0]
0x1400b3919 mov      r8d, 1
0x1400b391f call     0x140613c20
0x1400b3924 cmp      edi, 3
0x1400b3927 je       0x1400b3947
0x1400b3929 cmp      edi, 2
0x1400b392c jb       0x1400b3947
0x1400b392e mov      rax, qword ptr [rbp + 0x190]
0x1400b3935 lock dec qword ptr [rax]
0x1400b3939 jne      0x1400b3947
0x1400b393b lea      rcx, [rbp + 0x190]
0x1400b3942 call     0x14064eb30
0x1400b3947 mov      rax, qword ptr [rbp + 0x1a0]
0x1400b394e cmp      rax, 6
0x1400b3952 je       0x1400b398d
0x1400b3954 cmp      eax, 5
0x1400b3957 jb       0x1400b398d
0x1400b3959 mov      rax, qword ptr [rbp + 0x1a8]
0x1400b3960 lock dec qword ptr [rax]
0x1400b3964 jne      0x1400b398d
0x1400b3966 lea      rcx, [rbp + 0x1a8]
0x1400b396d call     0x14064eb30
0x1400b3972 jmp      0x1400b398d
0x1400b3974 mov      rax, qword ptr [rbp + 0x190]
0x1400b397b lock dec qword ptr [rax]
0x1400b397f jne      0x1400b398d
0x1400b3981 lea      rcx, [rbp + 0x190]
0x1400b3988 call     0x140c7de90
0x1400b398d mov      qword ptr [rbp + 0x188], 3
0x1400b3998 movups   xmm0, xmmword ptr [rbp - 0x60]
0x1400b399c movups   xmmword ptr [rbp + 0x190], xmm0
0x1400b39a3 mov      qword ptr [rbp + 0x1a0], 6
0x1400b39ae movups   xmm0, xmmword ptr [rbp + 0xe8]
0x1400b39b5 movups   xmmword ptr [rbp + 0x1a8], xmm0
0x1400b39bc mov      qword ptr [rbp + 0x1b8], 0xd
0x1400b39c7 mov      rax, qword ptr [rbp + 0x2b0]
0x1400b39ce mov      qword ptr [rbp + 0x1c0], rax
0x1400b39d5 mov      qword ptr [rbp + 0x1c8], 0xd
0x1400b39e0 mov      dword ptr [rbp + 0x1d0], 0
0x1400b39ea mov      dword ptr [rbp + 0x1d8], 0
0x1400b39f4 mov      qword ptr [rbp + 0x1e0], 0
0x1400b39ff mov      byte ptr [rbp + 0x1e8], 0
0x1400b3a06 mov      rax, qword ptr [rbp + 0x2a5]
0x1400b3a0d mov      qword ptr [rbp + 0x1e9], rax
0x1400b3a14 mov      eax, dword ptr [rbp + 0x2ac]
0x1400b3a1a mov      dword ptr [rbp + 0x1f0], eax
0x1400b3a20 lea      rdi, [rbp - 0x60]
0x1400b3a24 lea      rdx, [rbp + 0x158]
0x1400b3a2b mov      r8d, 0x148
0x1400b3a31 mov      rcx, rdi
0x1400b3a34 call     0x142923c00
0x1400b3a39 lea      rcx, [rbp + 0x158]
0x1400b3a40 mov      rdx, rdi
0x1400b3a43 mov      r8, rsi
0x1400b3a46 call     0x140c77160
0x1400b3a4b mov      rax, qword ptr [rbp + 0x168]
0x1400b3a52 lock dec qword ptr [rax]
0x1400b3a56 jne      0x1400b3a64
0x1400b3a58 lea      rcx, [rbp + 0x168]
0x1400b3a5f call     0x140c41170
0x1400b3a64 mov      eax, dword ptr [rbx + 0x1c]
0x1400b3a67 mov      rcx, qword ptr [rbx + 0x10]
0x1400b3a6b mov      dword ptr [rcx], eax
0x1400b3a6d mov      rdi, qword ptr [rbx]
0x1400b3a70 movzx    eax, byte ptr [rip + 0x3581192]
0x1400b3a77 mov      ecx, 0x15
0x1400b3a7c mov      edx, 1
0x1400b3a81 call     0x140613c10
0x1400b3a86 test     rax, rax
0x1400b3a89 je       0x1400b3bb4
0x1400b3a8f add      rdi, 0x1e
0x1400b3a93 movups   xmm0, xmmword ptr [rip + 0x288181e] ; @0x1429352b8 'Animate rain strengthUse color managementICC profile: '
0x1400b3a9a movups   xmmword ptr [rax], xmm0
0x1400b3a9d mov      qword ptr [rax + 0xd], r13
0x1400b3aa1 mov      qword ptr [rbp + 0x188], 0x15
0x1400b3aac mov      qword ptr [rbp + 0x190], rax
0x1400b3ab3 mov      qword ptr [rbp + 0x198], 0x15
0x1400b3abe mov      dword ptr [rbp + 0x1a0], 0
0x1400b3ac8 mov      qword ptr [rbp + 0x1b0], 0
0x1400b3ad3 mov      dword ptr [rbp + 0x1a8], 0
0x1400b3add mov      qword ptr [rbp + 0x158], 3
0x1400b3ae8 mov      qword ptr [rbp + 0x170], 6
0x1400b3af3 mov      byte ptr [rbp + 0x1b8], 0
0x1400b3afa mov      dword ptr [rbp + 0x1bd], 0
0x1400b3b04 mov      dword ptr [rbp + 0x1c0], 0
0x1400b3b0e mov      qword ptr [rbp + 0x1c8], rdi
0x1400b3b15 mov      byte ptr [rbp + 0x1d0], 0
0x1400b3b1c lea      rcx, [rbp - 0x60]
0x1400b3b20 lea      rdx, [rbp + 0x158]
0x1400b3b27 mov      r8, rsi
0x1400b3b2a call     0x140c48930
0x1400b3b2f mov      rax, qword ptr [rbp - 0x50]
0x1400b3b33 lock dec qword ptr [rax]
0x1400b3b37 jne      0x1400b3b42
0x1400b3b39 lea      rcx, [rbp - 0x50]
0x1400b3b3d call     0x140c41170
0x1400b3b42 movaps   xmm6, xmmword ptr [rbp + 0x2c0]
0x1400b3b49 add      rsp, 0x358
0x1400b3b50 pop      rbx
0x1400b3b51 pop      rdi
0x1400b3b52 pop      rsi
0x1400b3b53 pop      r12
0x1400b3b55 pop      r13
0x1400b3b57 pop      r14
0x1400b3b59 pop      r15
0x1400b3b5b pop      rbp
0x1400b3b5c ret      
0x1400b3b5d mov      ecx, 8
0x1400b3b62 mov      edx, 8
0x1400b3b67 call     0x1428d9000
0x1400b3b6c lea      r8, [rip + 0x28811d5]
0x1400b3b73 mov      ecx, 1
0x1400b3b78 mov      edx, 0xd
0x1400b3b7d call     0x1428d8fe3
0x1400b3b82 jmp      0x1400b3bb2
0x1400b3b84 lea      r8, [rip + 0x28811bd]
0x1400b3b8b mov      ecx, 1
0x1400b3b90 mov      edx, 0xd
0x1400b3b95 call     0x1428d8fe3
0x1400b3b9a jmp      0x1400b3bb2
0x1400b3b9c lea      r8, [rip + 0x28811a5]
0x1400b3ba3 mov      ecx, 1
0x1400b3ba8 mov      edx, 0xd
0x1400b3bad call     0x1428d8fe3
0x1400b3bb2 ud2      
0x1400b3bb4 lea      r8, [rip + 0x288118d]
0x1400b3bbb mov      ecx, 1