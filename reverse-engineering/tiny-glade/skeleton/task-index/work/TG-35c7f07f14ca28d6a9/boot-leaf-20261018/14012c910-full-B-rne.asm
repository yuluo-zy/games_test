14012c910: 55 push rbp
14012c911: 4157 push r15
14012c913: 4156 push r14
14012c915: 4154 push r12
14012c917: 56 push rsi
14012c918: 57 push rdi
14012c919: 53 push rbx
14012c91a: 4881ec50020000 sub rsp, 0x250
14012c921: 488dac2480000000 lea rbp, [rsp + 0x80]
14012c929: 48c785c8010000feffffff mov qword ptr [rbp + 0x1c8], 0xfffffffffffffffe
14012c934: 4889d3 mov rbx, rdx
14012c937: 4889ce mov rsi, rcx
14012c93a: 488d7938 lea rdi, [rcx + 0x38]
14012c93e: 488d81a8050000 lea rax, [rcx + 0x5a8]
14012c945: 4889bda8010000 mov qword ptr [rbp + 0x1a8], rdi
14012c94c: 488985b0010000 mov qword ptr [rbp + 0x1b0], rax
14012c953: 4883b98800000000 cmp qword ptr [rcx + 0x88], 0
14012c95b: 48b865d43391553f9e1e movabs rax, 0x1e9e3f559133d465
14012c965: 48b96c052a2050a35942 movabs rcx, 0x4259a350202a056c
14012c96f: 0f8495000000 je 0x14012ca0a
14012c975: 488b5670 mov rdx, qword ptr [rsi + 0x70]
14012c979: 4c8b4678 mov r8, qword ptr [rsi + 0x78]
14012c97d: 4d89c1 mov r9, r8
14012c980: 4921c1 and r9, rax
14012c983: 4531d2 xor r10d, r10d
14012c986: 660f6f05923c8202 movdqa xmm0, xmmword ptr [rip + 0x2823c92]
14012c98e: 660f76c9 pcmpeqd xmm1, xmm1
14012c992: f3420f6f140a movdqu xmm2, xmmword ptr [rdx + r9]
14012c998: 660f6fda movdqa xmm3, xmm2
14012c99c: 660f74d8 pcmpeqb xmm3, xmm0
14012c9a0: 66440fd7db pmovmskb r11d, xmm3
14012c9a5: 4585db test r11d, r11d
14012c9a8: 7533 jne 0x14012c9dd
14012c9aa: 660f74d1 pcmpeqb xmm2, xmm1
14012c9ae: 66440fd7da pmovmskb r11d, xmm2
14012c9b3: 4585db test r11d, r11d
14012c9b6: 7552 jne 0x14012ca0a
14012c9b8: 4d01d1 add r9, r10
14012c9bb: 4983c110 add r9, 0x10
14012c9bf: 4983c210 add r10, 0x10
14012c9c3: 4d21c1 and r9, r8
14012c9c6: ebca jmp 0x14012c992
14012c9c8: 0f1f840000000000 nop dword ptr [rax + rax]
14012c9d0: 458d73ff lea r14d, [r11 - 1]
14012c9d4: 664521de and r14w, r11w
14012c9d8: 4589f3 mov r11d, r14d
14012c9db: 74cd je 0x14012c9aa
14012c9dd: f3450fbcf3 tzcnt r14d, r11d
14012c9e2: 4d01ce add r14, r9
14012c9e5: 4d21c6 and r14, r8
14012c9e8: 49f7de neg r14
14012c9eb: 4f8d3476 lea r14, [r14 + r14*2]
14012c9ef: 4a394cf2e8 cmp qword ptr [rdx + r14*8 - 0x18], rcx
14012c9f4: 75da jne 0x14012c9d0
14012c9f6: 4a3944f2f0 cmp qword ptr [rdx + r14*8 - 0x10], rax
14012c9fb: 75d3 jne 0x14012c9d0
14012c9fd: 4a8d04f2 lea rax, [rdx + r14*8]
14012ca01: 4c8b70f8 mov r14, qword ptr [rax - 8]
14012ca05: e9ed020000 jmp 0x14012ccf7
14012ca0a: 0fb69698000000 movzx edx, byte ptr [rsi + 0x98]
14012ca11: 488b96d8000000 mov rdx, qword ptr [rsi + 0xd8]
14012ca18: 4c8b8ee0000000 mov r9, qword ptr [rsi + 0xe0]
14012ca1f: 4d89ca mov r10, r9
14012ca22: 4921c2 and r10, rax
14012ca25: 4531db xor r11d, r11d
14012ca28: 660f6f05f03b8202 movdqa xmm0, xmmword ptr [rip + 0x2823bf0]
14012ca30: 660f76c9 pcmpeqd xmm1, xmm1
14012ca34: f3420f6f1412 movdqu xmm2, xmmword ptr [rdx + r10]
14012ca3a: 660f6fda movdqa xmm3, xmm2
14012ca3e: 660f74d8 pcmpeqb xmm3, xmm0
14012ca42: 66440fd7f3 pmovmskb r14d, xmm3
14012ca47: 4585f6 test r14d, r14d
14012ca4a: 7531 jne 0x14012ca7d
14012ca4c: 660f74d1 pcmpeqb xmm2, xmm1
14012ca50: 66440fd7c2 pmovmskb r8d, xmm2
14012ca55: 4585c0 test r8d, r8d
14012ca58: 0f85ef000000 jne 0x14012cb4d
14012ca5e: 4d01da add r10, r11
14012ca61: 4983c210 add r10, 0x10
14012ca65: 4983c310 add r11, 0x10
14012ca69: 4d21ca and r10, r9
14012ca6c: ebc6 jmp 0x14012ca34
14012ca6e: 6690 nop 
14012ca70: 458d46ff lea r8d, [r14 - 1]
14012ca74: 664521f0 and r8w, r14w
14012ca78: 4589c6 mov r14d, r8d
14012ca7b: 74cf je 0x14012ca4c
14012ca7d: f3450fbcfe tzcnt r15d, r14d
14012ca82: 4d01d7 add r15, r10
14012ca85: 4d21cf and r15, r9
14012ca88: 4d89f8 mov r8, r15
14012ca8b: 49f7d8 neg r8
14012ca8e: 4e8d24fd00000000 lea r12, [r15*8]
14012ca96: 49c1e007 shl r8, 7
14012ca9a: 4d29e0 sub r8, r12
14012ca9d: 4a398c0278ffffff cmp qword ptr [rdx + r8 - 0x88], rcx
14012caa5: 75c9 jne 0x14012ca70
14012caa7: 4a39440280 cmp qword ptr [rdx + r8 - 0x80], rax
14012caac: 75c2 jne 0x14012ca70
14012caae: 4d89fa mov r10, r15
14012cab1: 49c1e207 shl r10, 7
14012cab5: 4f8d1cfa lea r11, [r10 + r15*8]
14012cab9: 49c1fb03 sar r11, 3
14012cabd: 49baf1f0f0f0f0f0f0f0 movabs r10, 0xf0f0f0f0f0f0f0f1
14012cac7: 4d0fafd3 imul r10, r11
14012cacb: 4d8d5af0 lea r11, [r10 - 0x10]
14012cacf: 4d21cb and r11, r9
14012cad2: f3420f6f041a movdqu xmm0, xmmword ptr [rdx + r11]
14012cad8: 660f76c9 pcmpeqd xmm1, xmm1
14012cadc: 660f74c1 pcmpeqb xmm0, xmm1
14012cae0: 66440fd7f0 pmovmskb r14d, xmm0
14012cae5: f3420f6f0412 movdqu xmm0, xmmword ptr [rdx + r10]
14012caeb: 660f74c1 pcmpeqb xmm0, xmm1
14012caef: 66440fd7c8 pmovmskb r9d, xmm0
14012caf4: 4585f6 test r14d, r14d
14012caf7: 740b je 0x14012cb04
14012caf9: 66450fbdf6 bsr r14w, r14w
14012cafe: 4183f60f xor r14d, 0xf
14012cb02: eb05 jmp 0x14012cb09
14012cb04: 6641be1000 mov r14w, 0x10
14012cb09: 4901d0 add r8, rdx
14012cb0c: 4181c900000100 or r9d, 0x10000
14012cb13: f3450fbcc9 tzcnt r9d, r9d
14012cb18: 4501ce add r14d, r9d
14012cb1b: 41b180 mov r9b, 0x80
14012cb1e: 664183fe0f cmp r14w, 0xf
14012cb23: 770a ja 0x14012cb2f
14012cb25: 48ff86e8000000 inc qword ptr [rsi + 0xe8]
14012cb2c: 41b1ff mov r9b, 0xff
14012cb2f: 46880c12 mov byte ptr [rdx + r10], r9b
14012cb33: 46884c1a10 mov byte ptr [rdx + r11 + 0x10], r9b
14012cb38: 48ff8ef0000000 dec qword ptr [rsi + 0xf0]
14012cb3f: 498b5088 mov rdx, qword ptr [r8 - 0x78]
14012cb43: 4883fa02 cmp rdx, 2
14012cb47: 0f8537010000 jne 0x14012cc84
14012cb4d: 4c8bb6a8050000 mov r14, qword ptr [rsi + 0x5a8]
14012cb54: 4d8d7e01 lea r15, [r14 + 1]
14012cb58: 4c89bea8050000 mov qword ptr [rsi + 0x5a8], r15
14012cb5f: 48ba0000000000000080 movabs rdx, 0x8000000000000000
14012cb69: 48899580010000 mov qword ptr [rbp + 0x180], rdx
14012cb70: 488d1595a78202 lea rdx, [rip + 0x282a795] ; @14295730c 'country_core::systems::render_loop::compute_passes::leaf_particles::LoadedParticleEmitterCountco'
14012cb77: 48899588010000 mov qword ptr [rbp + 0x188], rdx
14012cb7e: 48c785900100005e000000 mov qword ptr [rbp + 0x190], 0x5e
14012cb89: 66c785a00100000001 mov word ptr [rbp + 0x1a0], 0x100
14012cb92: 48c7854801000001000000 mov qword ptr [rbp + 0x148], 1
14012cb9d: 48898d50010000 mov qword ptr [rbp + 0x150], rcx
14012cba4: 48898558010000 mov qword ptr [rbp + 0x158], rax
14012cbab: 48c7857001000008000000 mov qword ptr [rbp + 0x170], 8
14012cbb6: 48c7857801000008000000 mov qword ptr [rbp + 0x178], 8
14012cbc1: 48c7859801000000000000 mov qword ptr [rbp + 0x198], 0
14012cbcc: c685a201000001 mov byte ptr [rbp + 0x1a2], 1
14012cbd3: 48c7856001000000000000 mov qword ptr [rbp + 0x160], 0
14012cbde: 488d4da8 lea rcx, [rbp - 0x58]
14012cbe2: 4c8d8548010000 lea r8, [rbp + 0x148]
14012cbe9: 4c89f2 mov rdx, r14
14012cbec: e87f2cc100 call 0x140d3f870 ; bevy_ecs::component::ComponentInfo::new
14012cbf1: 4c397e48 cmp qword ptr [rsi + 0x48], r15
14012cbf5: 7319 jae 0x14012cc10
14012cbf7: c685c701000001 mov byte ptr [rbp + 0x1c7], 1
14012cbfe: 4c8d057b3c8202 lea r8, [rip + 0x2823c7b]
14012cc05: 4889f9 mov rcx, rdi
14012cc08: 4c89fa mov rdx, r15
14012cc0b: e8808a1800 call 0x1402b5690 ; alloc::vec::Vec::resize_with<enum2$<core::option::Option<bevy_ecs::component::ComponentInfo> >,alloc::alloc::Global,bevy_ecs::component::impl$19::register_component_inner::closure_env$0>
14012cc10: 4c8b7e40 mov r15, qword ptr [rsi + 0x40]
14012cc14: 4d69e6d0000000 imul r12, r14, 0xd0
14012cc1b: 4b8d0427 lea rax, [r15 + r12]
14012cc1f: 488985b8010000 mov qword ptr [rbp + 0x1b8], rax
14012cc26: 488d4d78 lea rcx, [rbp + 0x78]
14012cc2a: 488d55a8 lea rdx, [rbp - 0x58]
14012cc2e: 41b8d0000000 mov r8d, 0xd0
14012cc34: e8c76f7f02 call 0x142923c00 ; 
14012cc39: 43833c2702 cmp dword ptr [r15 + r12], 2
14012cc3e: 740c je 0x14012cc4c
14012cc40: 488b8db8010000 mov rcx, qword ptr [rbp + 0x1b8]
14012cc47: e8043f4c00 call 0x1405f0b50 ; core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo> | core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo> | core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo> | core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo> | core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo> | core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo> | core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo> | core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo> | core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo> | core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo> | core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo> | core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo> | core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo> | core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo> | core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo> | core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo> | core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo> | core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo>
14012cc4c: 488d5578 lea rdx, [rbp + 0x78]
14012cc50: 41b8d0000000 mov r8d, 0xd0
14012cc56: 488b8db8010000 mov rcx, qword ptr [rbp + 0x1b8]
14012cc5d: e89e6f7f02 call 0x142923c00 ; 
14012cc62: 488d4e70 lea rcx, [rsi + 0x70]
14012cc66: 48ba6c052a2050a35942 movabs rdx, 0x4259a350202a056c
14012cc70: 49b865d43391553f9e1e movabs r8, 0x1e9e3f559133d465
14012cc7a: 4d89f1 mov r9, r14
14012cc7d: e89e5ff5ff call 0x140082c20 ; hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global> | hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global>
14012cc82: eb73 jmp 0x14012ccf7
14012cc84: 410f1040f0 movups xmm0, xmmword ptr [r8 - 0x10]
14012cc89: 0f1185e0000000 movups xmmword ptr [rbp + 0xe0], xmm0
14012cc90: 410f1040e0 movups xmm0, xmmword ptr [r8 - 0x20]
14012cc95: 0f1185d0000000 movups xmmword ptr [rbp + 0xd0], xmm0
14012cc9c: 410f1040d0 movups xmm0, xmmword ptr [r8 - 0x30]
14012cca1: 0f1185c0000000 movups xmmword ptr [rbp + 0xc0], xmm0
14012cca8: f3410f6f4090 movdqu xmm0, xmmword ptr [r8 - 0x70]
14012ccae: f3410f6f48a0 movdqu xmm1, xmmword ptr [r8 - 0x60]
14012ccb4: f3410f6f50b0 movdqu xmm2, xmmword ptr [r8 - 0x50]
14012ccba: f3410f6f58c0 movdqu xmm3, xmmword ptr [r8 - 0x40]
14012ccc0: f30f7f9db0000000 movdqu xmmword ptr [rbp + 0xb0], xmm3
14012ccc8: f30f7f95a0000000 movdqu xmmword ptr [rbp + 0xa0], xmm2
14012ccd0: f30f7f8d90000000 movdqu xmmword ptr [rbp + 0x90], xmm1
14012ccd8: f30f7f8580000000 movdqu xmmword ptr [rbp + 0x80], xmm0
14012cce0: 48895578 mov qword ptr [rbp + 0x78], rdx
14012cce4: 488d4d78 lea rcx, [rbp + 0x78]
14012cce8: 488d95a8010000 lea rdx, [rbp + 0x1a8]
14012ccef: e84c2dc100 call 0x140d3fa40 ; bevy_ecs::component::QueuedRegistration::register
14012ccf4: 4989c6 mov r14, rax
14012ccf7: 48899d48010000 mov qword ptr [rbp + 0x148], rbx
14012ccfe: 8b9eb0050000 mov ebx, dword ptr [rsi + 0x5b0]
14012cd04: 4889f1 mov rcx, rsi
14012cd07: e844b5bc00 call 0x140cf8250 ; bevy_ecs::world::World::flush_components
14012cd0c: 488d86f8000000 lea rax, [rsi + 0xf8]
14012cd13: 4c8975a8 mov qword ptr [rbp - 0x58], r14
14012cd17: 488d8ed8010000 lea rcx, [rsi + 0x1d8]
14012cd1e: 48897d78 mov qword ptr [rbp + 0x78], rdi
14012cd22: 488d55a8 lea rdx, [rbp - 0x58]
14012cd26: 48899580000000 mov qword ptr [rbp + 0x80], rdx
14012cd2d: 48898588000000 mov qword ptr [rbp + 0x88], rax
14012cd34: 4c8d4578 lea r8, [rbp + 0x78]
14012cd38: 4c89f2 mov rdx, r14
14012cd3b: e840c43d00 call 0x140509180 ; bevy_ecs::storage::sparse_set::SparseSet::get_or_insert_with<bevy_ecs::component::ComponentId,bevy_ecs::storage::resource::ResourceData<true>,bevy_ecs::storage::resource::impl$2::initialize_with::closure_env$0<true,bevy_ecs::world::impl$2::initialize_resource_internal::closure_env$0> >
14012cd40: 4889c7 mov rdi, rax
14012cd43: 4883782000 cmp qword ptr [rax + 0x20], 0
14012cd48: 7413 je 0x14012cd5d
14012cd4a: 4c8d8548010000 lea r8, [rbp + 0x148]
14012cd51: 4889f9 mov rcx, rdi
14012cd54: 31d2 xor edx, edx
14012cd56: e81529bd00 call 0x140cff670 ; bevy_ecs::storage::blob_vec::BlobVec::replace_unchecked
14012cd5b: eb2c jmp 0x14012cd89
14012cd5d: 48837f1800 cmp qword ptr [rdi + 0x18], 0
14012cd62: 743d je 0x14012cda1
14012cd64: 31c9 xor ecx, ecx
14012cd66: 488d4101 lea rax, [rcx + 1]
14012cd6a: 48894720 mov qword ptr [rdi + 0x20], rax
14012cd6e: 4c8b4708 mov r8, qword ptr [rdi + 8]
14012cd72: 490fafc8 imul rcx, r8
14012cd76: 48034f10 add rcx, qword ptr [rdi + 0x10]
14012cd7a: 488d9548010000 lea rdx, [rbp + 0x148]
14012cd81: e87a6e7f02 call 0x142923c00 ; 
14012cd86: 895f58 mov dword ptr [rdi + 0x58], ebx
14012cd89: 895f5c mov dword ptr [rdi + 0x5c], ebx
14012cd8c: 4889f0 mov rax, rsi
14012cd8f: 4881c450020000 add rsp, 0x250
14012cd96: 5b pop rbx
14012cd97: 5f pop rdi
14012cd98: 5e pop rsi
14012cd99: 415c pop r12
14012cd9b: 415e pop r14
14012cd9d: 415f pop r15
14012cd9f: 5d pop rbp
14012cda0: c3 ret 
14012cda1: ba01000000 mov edx, 1
14012cda6: 4889f9 mov rcx, rdi
14012cda9: e8921e7a02 call 0x1428cec40 ; bevy_ecs::storage::blob_vec::impl$1::reserve::do_reserve
14012cdae: 488b4f20 mov rcx, qword ptr [rdi + 0x20]
14012cdb2: ebb2 jmp 0x14012cd66
14012cdb4: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14012cdc0: 4889542410 mov qword ptr [rsp + 0x10], rdx
14012cdc5: 55 push rbp
14012cdc6: 4157 push r15
14012cdc8: 4156 push r14
14012cdca: 4154 push r12
14012cdcc: 56 push rsi
14012cdcd: 57 push rdi
14012cdce: 53 push rbx
14012cdcf: 4883ec20 sub rsp, 0x20
14012cdd3: 488daa80000000 lea rbp, [rdx + 0x80]
14012cdda: 488d5578 lea rdx, [rbp + 0x78]
14012cdde: 41b8d0000000 mov r8d, 0xd0
14012cde4: 488b8db8010000 mov rcx, qword ptr [rbp + 0x1b8]
14012cdeb: e8106e7f02 call 0x142923c00 ; 
14012cdf0: c685c701000000 mov byte ptr [rbp + 0x1c7], 0
14012cdf7: 4883c420 add rsp, 0x20
14012cdfb: 5b pop rbx
14012cdfc: 5f pop rdi
14012cdfd: 5e pop rsi
14012cdfe: 415c pop r12
14012ce00: 415e pop r14
14012ce02: 415f pop r15
14012ce04: 5d pop rbp
14012ce05: c3 ret 
14012ce06: 662e0f1f840000000000 nop word ptr cs:[rax + rax]
14012ce10: 4889542410 mov qword ptr [rsp + 0x10], rdx
14012ce15: 55 push rbp
14012ce16: 4157 push r15
14012ce18: 4156 push r14
14012ce1a: 4154 push r12
14012ce1c: 56 push rsi
14012ce1d: 57 push rdi
14012ce1e: 53 push rbx
14012ce1f: 4883ec20 sub rsp, 0x20
14012ce23: 488daa80000000 lea rbp, [rdx + 0x80]
14012ce2a: 80bdc701000000 cmp byte ptr [rbp + 0x1c7], 0
14012ce31: 750f jne 0x14012ce42
14012ce33: 4883c420 add rsp, 0x20
14012ce37: 5b pop rbx
14012ce38: 5f pop rdi
14012ce39: 5e pop rsi
14012ce3a: 415c pop r12
14012ce3c: 415e pop r14
14012ce3e: 415f pop r15
14012ce40: 5d pop rbp
14012ce41: c3 ret 
14012ce42: 488d4da8 lea rcx, [rbp - 0x58]
14012ce46: e8053d4c00 call 0x1405f0b50 ; core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo> | core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo> | core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo> | core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo> | core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo> | core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo> | core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo> | core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo> | core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo> | core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo> | core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo> | core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo> | core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo> | core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo> | core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo> | core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo> | core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo> | core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo>
14012ce4b: ebe6 jmp 0x14012ce33
