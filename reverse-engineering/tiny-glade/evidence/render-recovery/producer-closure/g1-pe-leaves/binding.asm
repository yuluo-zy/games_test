=== 0x140bee130 size=25  rhapsody::raw::binding::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::bind_pipeline::h0fae21ebb9af99fd
    ; GraphicsDevice::bind_pipeline: tail-call through device fn-table slot [rcx+0x2d8] with (cmd=[rdx+0x48], edx=[r8+0x30]=bindPoint, r8=[r8+0x28]=pipeline). vkCmdBindPipeline wrapper.  [high]
    0x140bee130    488b4248                 mov              rax, qword ptr [rdx + 0x48]
    0x140bee134    418b5030                 mov              edx, dword ptr [r8 + 0x30]
    0x140bee138    4d8b4028                 mov              r8, qword ptr [r8 + 0x28]
    0x140bee13c    4c8b89d8020000           mov              r9, qword ptr [rcx + 0x2d8]
    0x140bee143    4889c1                   mov              rcx, rax
    0x140bee146    49ffe1                   jmp              r9

=== 0x141621310 size=551  rhapsody::raw::binding::impl$1::fmt
    ; impl$1::fmt: Debug impl (diagnostic; body decoded in asm).  [low]
    0x141621310    4883ec68                 sub              rsp, 0x68
    0x141621314    4889d0                   mov              rax, rdx
    0x141621317    8b11                     mov              edx, dword ptr [rcx]
    0x141621319    4c8d0568fc6501           lea              r8, [rip + 0x165fc68] -> 0x142c80f88
    0x141621320    49631490                 movsxd           rdx, dword ptr [r8 + rdx*4]
    0x141621324    4c01c2                   add              rdx, r8
    0x141621327    ffe2                     jmp              rdx
    0x141621329    4883c108                 add              rcx, 8
    0x14162132d    48894c2460               mov              qword ptr [rsp + 0x60], rcx
    0x141621332    488d0dcff16501           lea              rcx, [rip + 0x165f1cf] -> 0x142c80508
    0x141621339    48894c2420               mov              qword ptr [rsp + 0x20], rcx
    0x14162133e    488d15e3f16501           lea              rdx, [rip + 0x165f1e3] -> 0x142c80528
    0x141621345    e919010000               jmp              0x141621463
    0x14162134a    4883c108                 add              rcx, 8
    0x14162134e    48894c2460               mov              qword ptr [rsp + 0x60], rcx
    0x141621353    488d0daef16501           lea              rcx, [rip + 0x165f1ae] -> 0x142c80508
    0x14162135a    48894c2420               mov              qword ptr [rsp + 0x20], rcx
    0x14162135f    488d1530f26501           lea              rdx, [rip + 0x165f230] -> 0x142c80596
    0x141621366    4c8d4c2460               lea              r9, [rsp + 0x60]
    0x14162136b    41b814000000             mov              r8d, 0x14
    0x141621371    e93f010000               jmp              0x1416214b5
    0x141621376    488d5108                 lea              rdx, [rcx + 8]
    0x14162137a    4883c104                 add              rcx, 4
    0x14162137e    48894c2460               mov              qword ptr [rsp + 0x60], rcx
    0x141621383    488d0dbef26501           lea              rcx, [rip + 0x165f2be] -> 0x142c80648
    0x14162138a    48894c2450               mov              qword ptr [rsp + 0x50], rcx
    0x14162138f    488d4c2460               lea              rcx, [rsp + 0x60]
    0x141621394    48894c2448               mov              qword ptr [rsp + 0x48], rcx
    0x141621399    488d0d5af16501           lea              rcx, [rip + 0x165f15a] -> 0x142c804fa
    0x1416213a0    48894c2438               mov              qword ptr [rsp + 0x38], rcx
    0x1416213a5    488d0d7cf26501           lea              rcx, [rip + 0x165f27c] -> 0x142c80628
    0x1416213ac    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x1416213b1    4889542428               mov              qword ptr [rsp + 0x28], rdx
    0x1416213b6    48c744244006000000       mov              qword ptr [rsp + 0x40], 6
    0x1416213bf    48c744242006000000       mov              qword ptr [rsp + 0x20], 6
    0x1416213c8    488d15adf26501           lea              rdx, [rip + 0x165f2ad] -> 0x142c8067c
    0x1416213cf    e948010000               jmp              0x14162151c
    0x1416213d4    4883c108                 add              rcx, 8
    0x1416213d8    48894c2460               mov              qword ptr [rsp + 0x60], rcx
    0x1416213dd    488d0d64f16501           lea              rcx, [rip + 0x165f164] -> 0x142c80548
    0x1416213e4    48894c2420               mov              qword ptr [rsp + 0x20], rcx
    0x1416213e9    488d1578f16501           lea              rdx, [rip + 0x165f178] -> 0x142c80568
    0x1416213f0    eb1c                     jmp              0x14162140e
    0x1416213f2    4883c108                 add              rcx, 8
    0x1416213f6    48894c2460               mov              qword ptr [rsp + 0x60], rcx
    0x1416213fb    488d0d46f16501           lea              rcx, [rip + 0x165f146] -> 0x142c80548
    0x141621402    48894c2420               mov              qword ptr [rsp + 0x20], rcx
    0x141621407    488d1571f16501           lea              rdx, [rip + 0x165f171] -> 0x142c8057f
    0x14162140e    4c8d4c2460               lea              r9, [rsp + 0x60]
    0x141621413    41b817000000             mov              r8d, 0x17
    0x141621419    e997000000               jmp              0x1416214b5
    0x14162141e    4883c108                 add              rcx, 8
    0x141621422    48894c2460               mov              qword ptr [rsp + 0x60], rcx
    0x141621427    488d0dc2f16501           lea              rcx, [rip + 0x165f1c2] -> 0x142c805f0
    0x14162142e    48894c2420               mov              qword ptr [rsp + 0x20], rcx
    0x141621433    488d15d6f16501           lea              rdx, [rip + 0x165f1d6] -> 0x142c80610
    0x14162143a    4c8d4c2460               lea              r9, [rsp + 0x60]
    0x14162143f    41b816000000             mov              r8d, 0x16
    0x141621445    eb6e                     jmp              0x1416214b5
    0x141621447    4883c108                 add              rcx, 8
    0x14162144b    48894c2460               mov              qword ptr [rsp + 0x60], rcx
    0x141621450    488d0db1f06501           lea              rcx, [rip + 0x165f0b1] -> 0x142c80508
    0x141621457    48894c2420               mov              qword ptr [rsp + 0x20], rcx
    0x14162145c    488d15d3f06501           lea              rdx, [rip + 0x165f0d3] -> 0x142c80536
    0x141621463    4c8d4c2460               lea              r9, [rsp + 0x60]
    0x141621468    41b80e000000             mov              r8d, 0xe
    0x14162146e    eb45                     jmp              0x1416214b5
    0x141621470    4883c108                 add              rcx, 8
    0x141621474    48894c2460               mov              qword ptr [rsp + 0x60], rcx
    0x141621479    488d0d30f16501           lea              rcx, [rip + 0x165f130] -> 0x142c805b0
    0x141621480    48894c2420               mov              qword ptr [rsp + 0x20], rcx
    0x141621485    488d1544f16501           lea              rdx, [rip + 0x165f144] -> 0x142c805d0
    0x14162148c    eb1c                     jmp              0x1416214aa
    0x14162148e    4883c108                 add              rcx, 8
    0x141621492    48894c2460               mov              qword ptr [rsp + 0x60], rcx
    0x141621497    488d0d12f16501           lea              rcx, [rip + 0x165f112] -> 0x142c805b0
    0x14162149e    48894c2420               mov              qword ptr [rsp + 0x20], rcx
    0x1416214a3    488d1533f16501           lea              rdx, [rip + 0x165f133] -> 0x142c805dd
    0x1416214aa    4c8d4c2460               lea              r9, [rsp + 0x60]
    0x1416214af    41b80d000000             mov              r8d, 0xd
    0x1416214b5    4889c1                   mov              rcx, rax
    0x1416214b8    e8135e76ff               call             0x140d872d0 -> core::fmt::Formatter::debug_tuple_field1_finish::ha115499c8c7eba6f
    0x1416214bd    90                       nop              
    0x1416214be    4883c468                 add              rsp, 0x68
    0x1416214c2    c3                       ret              
    0x1416214c3    488d5108                 lea              rdx, [rcx + 8]
    0x1416214c7    4883c104                 add              rcx, 4
    0x1416214cb    48894c2460               mov              qword ptr [rsp + 0x60], rcx
    0x1416214d0    488d0d71f16501           lea              rcx, [rip + 0x165f171] -> 0x142c80648
    0x1416214d7    48894c2450               mov              qword ptr [rsp + 0x50], rcx
    0x1416214dc    488d4c2460               lea              rcx, [rsp + 0x60]
    0x1416214e1    48894c2448               mov              qword ptr [rsp + 0x48], rcx
    0x1416214e6    488d0d0df06501           lea              rcx, [rip + 0x165f00d] -> 0x142c804fa
    0x1416214ed    48894c2438               mov              qword ptr [rsp + 0x38], rcx
    0x1416214f2    488d0d2ff16501           lea              rcx, [rip + 0x165f12f] -> 0x142c80628
    0x1416214f9    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x1416214fe    4889542428               mov              qword ptr [rsp + 0x28], rdx
    0x141621503    48c744244006000000       mov              qword ptr [rsp + 0x40], 6
    0x14162150c    48c744242006000000       mov              qword ptr [rsp + 0x20], 6
    0x141621515    488d154cf16501           lea              rdx, [rip + 0x165f14c] -> 0x142c80668
    0x14162151c    4c8d0dd1ef6501           lea              r9, [rip + 0x165efd1] -> 0x142c804f4
    0x141621523    41b814000000             mov              r8d, 0x14
    0x141621529    4889c1                   mov              rcx, rax
    0x14162152c    e86f5776ff               call             0x140d86ca0 -> core::fmt::Formatter::debug_struct_field2_finish::h35c7e6c960cc525a
    0x141621531    90                       nop              
    0x141621532    4883c468                 add              rsp, 0x68
    0x141621536    c3                       ret              
