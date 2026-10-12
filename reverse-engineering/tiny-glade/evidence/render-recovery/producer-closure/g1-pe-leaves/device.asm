=== 0x140be23a0 size=260  rhapsody::raw::device::impl$5::drop
    ; GraphicsDevice::impl$5::drop: wait_idle(); [dev+0x648]->take_out_dropped_waste(dev); lock RawMutex@dev+0x718 (parking_lot); Queue::reclaim_command_buffer_resources(&Queue@dev+0x730, dev); for each Queue in Vec@dev+0x7e8 (len@+0x7f0, stride 0xb0) reclaim; unlock. Device Drop path.  [high]
    0x140be23a0    55                       push             rbp
    0x140be23a1    56                       push             rsi
    0x140be23a2    57                       push             rdi
    0x140be23a3    53                       push             rbx
    0x140be23a4    4883ec38                 sub              rsp, 0x38
    0x140be23a8    488d6c2430               lea              rbp, [rsp + 0x30]
    0x140be23ad    48c74500feffffff         mov              qword ptr [rbp], 0xfffffffffffffffe
    0x140be23b5    4889ce                   mov              rsi, rcx
    0x140be23b8    e8c3650100               call             0x140bf8980 -> rhapsody::raw::device::GraphicsDevice::wait_idle
    0x140be23bd    488b8e48060000           mov              rcx, qword ptr [rsi + 0x648]
    0x140be23c4    4889f2                   mov              rdx, rsi
    0x140be23c7    e8a4e6cd00               call             0x1418c0a70 -> rhapsody::raw::managed::resources::ManagedResources::take_out_dropped_waste
    0x140be23cc    488dbe18070000           lea              rdi, [rsi + 0x718]
    0x140be23d3    b101                     mov              cl, 1
    0x140be23d5    31c0                     xor              eax, eax
    0x140be23d7    f00fb08e18070000         lock cmpxchg     byte ptr [rsi + 0x718], cl
    0x140be23df    7575                     jne              0x140be2456
    0x140be23e1    48897df8                 mov              qword ptr [rbp - 8], rdi
    0x140be23e5    4889f9                   mov              rcx, rdi
    0x140be23e8    e883f80900               call             0x140c81c70 -> parking_lot_core::parking_lot::deadlock_impl::acquire_resource::hb2f16ac28e483830
    0x140be23ed    488d8e30070000           lea              rcx, [rsi + 0x730]
    0x140be23f4    4889f2                   mov              rdx, rsi
    0x140be23f7    e8c4e80000               call             0x140bf0cc0 -> rhapsody::raw::device::Queue::reclaim_command_buffer_resources
    0x140be23fc    488b86f0070000           mov              rax, qword ptr [rsi + 0x7f0]
    0x140be2403    4885c0                   test             rax, rax
    0x140be2406    742f                     je               0x140be2437
    0x140be2408    488bbee8070000           mov              rdi, qword ptr [rsi + 0x7e8]
    0x140be240f    4869d8b0000000           imul             rbx, rax, 0xb0
    0x140be2416    4801fb                   add              rbx, rdi
    0x140be2419    0f1f8000000000           nop              dword ptr [rax]
    0x140be2420    4889f9                   mov              rcx, rdi
    0x140be2423    4889f2                   mov              rdx, rsi
    0x140be2426    e895e80000               call             0x140bf0cc0 -> rhapsody::raw::device::Queue::reclaim_command_buffer_resources
    0x140be242b    4881c7b0000000           add              rdi, 0xb0
    0x140be2432    4839df                   cmp              rdi, rbx
    0x140be2435    75e9                     jne              0x140be2420
    0x140be2437    488b75f8                 mov              rsi, qword ptr [rbp - 8]
    0x140be243b    4889f1                   mov              rcx, rsi
    0x140be243e    e8cdf90900               call             0x140c81e10 -> parking_lot_core::parking_lot::deadlock_impl::release_resource::h1c3ab4db915ad6f2
    0x140be2443    31c9                     xor              ecx, ecx
    0x140be2445    b001                     mov              al, 1
    0x140be2447    f00fb00e                 lock cmpxchg     byte ptr [rsi], cl
    0x140be244b    751c                     jne              0x140be2469
    0x140be244d    4883c438                 add              rsp, 0x38
    0x140be2451    5b                       pop              rbx
    0x140be2452    5f                       pop              rdi
    0x140be2453    5e                       pop              rsi
    0x140be2454    5d                       pop              rbp
    0x140be2455    c3                       ret              
    0x140be2456    4889f9                   mov              rcx, rdi
    0x140be2459    41b800ca9a3b             mov              r8d, 0x3b9aca00
    0x140be245f    e8acadce01               call             0x1428cd210 -> parking_lot::raw_mutex::RawMutex::lock_slow::h7c244b522f551b38
    0x140be2464    e978ffffff               jmp              0x140be23e1
    0x140be2469    4889f1                   mov              rcx, rsi
    0x140be246c    31d2                     xor              edx, edx
    0x140be246e    4883c438                 add              rsp, 0x38
    0x140be2472    5b                       pop              rbx
    0x140be2473    5f                       pop              rdi
    0x140be2474    5e                       pop              rsi
    0x140be2475    5d                       pop              rbp
    0x140be2476    e915b7ce01               jmp              0x1428cdb90 -> parking_lot::raw_mutex::RawMutex::unlock_slow::h56cb6df3e32a2657
    0x140be247b    0f1f440000               nop              dword ptr [rax + rax]
    0x140be2480    4889542410               mov              qword ptr [rsp + 0x10], rdx
    0x140be2485    55                       push             rbp
    0x140be2486    56                       push             rsi
    0x140be2487    57                       push             rdi
    0x140be2488    53                       push             rbx
    0x140be2489    4883ec28                 sub              rsp, 0x28
    0x140be248d    488d6a30                 lea              rbp, [rdx + 0x30]
    0x140be2491    488b4df8                 mov              rcx, qword ptr [rbp - 8]
    0x140be2495    e896f757ff               call             0x140161c30 -> core::ptr::drop_in_place$LT$lock_api..mutex..MutexGuard$LT$parking_lot..raw_mutex..RawMutex$C$alloc..vec..Vec$LT$glam..f32..vec2..Vec2$GT$$GT$$GT$::hc1588d55d4486b0e (.llvm.14454403653567781691)
    0x140be249a    90                       nop              
    0x140be249b    4883c428                 add              rsp, 0x28
    0x140be249f    5b                       pop              rbx
    0x140be24a0    5f                       pop              rdi
    0x140be24a1    5e                       pop              rsi
    0x140be24a2    5d                       pop              rbp
    0x140be24a3    c3                       ret              

=== 0x140bf84c0 size=21  rhapsody::raw::device::GraphicsDevice::device_uuid
    ; GraphicsDevice::device_uuid: returns 16-byte movups from [*(dev+0x658)+0x690]. i.e. pipelineCacheUUID from VkPhysicalDeviceProperties (physical device base = *(dev+0x658)+0x10).  [high]
    0x140bf84c0    4889c8                   mov              rax, rcx
    0x140bf84c3    488b8a58060000           mov              rcx, qword ptr [rdx + 0x658]
    0x140bf84ca    0f108190060000           movups           xmm0, xmmword ptr [rcx + 0x690]
    0x140bf84d1    0f1100                   movups           xmmword ptr [rax], xmm0
    0x140bf84d4    c3                       ret              

=== 0x140bf8950 size=12  rhapsody::raw::device::GraphicsDevice::physical_device
    ; GraphicsDevice::physical_device: returns *(dev+0x658) + 0x10 (addr of the PhysicalDevice object).  [high]
    0x140bf8950    488b8158060000           mov              rax, qword ptr [rcx + 0x658]
    0x140bf8957    4883c010                 add              rax, 0x10
    0x140bf895b    c3                       ret              

=== 0x140bf8960 size=22  rhapsody::raw::device::GraphicsDevice::debug_utils
    ; GraphicsDevice::debug_utils: returns &(dev+0x870) if [dev+0x870]!=0 else NULL (Option<&DebugUtils> accessor).  [medium]
    0x140bf8960    488b9170080000           mov              rdx, qword ptr [rcx + 0x870]
    0x140bf8967    488d8170080000           lea              rax, [rcx + 0x870]
    0x140bf896e    4885d2                   test             rdx, rdx
    0x140bf8971    480f44c2                 cmove            rax, rdx
    0x140bf8975    c3                       ret              

=== 0x140bf8f80 size=23  rhapsody::raw::device::GraphicsDevice::refresh_memory_budget
    ; GraphicsDevice::refresh_memory_budget: if byte[dev+0x1a]!=0 then tail-call PhysicalDevice::refresh_memory_budget(&(*(dev+0x658)+0x10)); else no-op.  [high]
    0x140bf8f80    80791a00                 cmp              byte ptr [rcx + 0x1a], 0
    0x140bf8f84    7410                     je               0x140bf8f96
    0x140bf8f86    488b8958060000           mov              rcx, qword ptr [rcx + 0x658]
    0x140bf8f8d    4883c110                 add              rcx, 0x10
    0x140bf8f91    e98a860000               jmp              0x140c01620 -> rhapsody::raw::physical_device::PhysicalDevice::refresh_memory_budget
    0x140bf8f96    c3                       ret              

=== 0x140bf8fa0 size=7  rhapsody::raw::device::GraphicsDevice::min_subgroup_size
    ; GraphicsDevice::min_subgroup_size: returns (eax=[dev+0x40], edx=[dev+0x44]) — a pair of u32 read from dev.  [high]
    0x140bf8fa0    8b4140                   mov              eax, dword ptr [rcx + 0x40]
    0x140bf8fa3    8b5144                   mov              edx, dword ptr [rcx + 0x44]
    0x140bf8fa6    c3                       ret              
