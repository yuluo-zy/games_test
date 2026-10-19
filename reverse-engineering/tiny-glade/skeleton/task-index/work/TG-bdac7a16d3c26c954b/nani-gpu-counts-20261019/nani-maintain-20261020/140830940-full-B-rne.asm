140830940: 55 push rbp
140830941: 4157 push r15
140830943: 4156 push r14
140830945: 4155 push r13
140830947: 4154 push r12
140830949: 56 push rsi
14083094a: 57 push rdi
14083094b: 53 push rbx
14083094c: 4881ece8020000 sub rsp, 0x2e8
140830953: 488dac2480000000 lea rbp, [rsp + 0x80]
14083095b: 48c78560020000feffffff mov qword ptr [rbp + 0x260], 0xfffffffffffffffe
140830966: 4989d6 mov r14, rdx
140830969: 488bb5d0020000 mov rsi, qword ptr [rbp + 0x2d0]
140830970: 4c8b3a mov r15, qword ptr [rdx]
140830973: 488b5e08 mov rbx, qword ptr [rsi + 8]
140830977: 488b7e10 mov rdi, qword ptr [rsi + 0x10]
14083097b: 4c89f9 mov rcx, r15
14083097e: 4889da mov rdx, rbx
140830981: 4989f8 mov r8, rdi
140830984: e8e7035100 call 0x140d40d70 ; bevy_ecs::component::enforce_no_required_components_recursion
140830989: 48c7853002000000000000 mov qword ptr [rbp + 0x230], 0
140830994: 48c7853802000008000000 mov qword ptr [rbp + 0x238], 8
14083099f: 48c7854002000000000000 mov qword ptr [rbp + 0x240], 0
1408309aa: 49837f3000 cmp qword ptr [r15 + 0x30], 0
1408309af: 48b83815e1791d76a7da movabs rax, 0xdaa7761d79e11538
1408309b9: 48b9df13ac4c1f79bee9 movabs rcx, 0xe9be791f4cac13df
1408309c3: 0f849a000000 je 0x140830a63
1408309c9: 498b5718 mov rdx, qword ptr [r15 + 0x18]
1408309cd: 4d8b4720 mov r8, qword ptr [r15 + 0x20]
1408309d1: 4d89c1 mov r9, r8
1408309d4: 4921c1 and r9, rax
1408309d7: 4531d2 xor r10d, r10d
1408309da: 660f6f055e132602 movdqa xmm0, xmmword ptr [rip + 0x226135e]
1408309e2: 660f76c9 pcmpeqd xmm1, xmm1
1408309e6: f3420f6f140a movdqu xmm2, xmmword ptr [rdx + r9]
1408309ec: 660f6fda movdqa xmm3, xmm2
1408309f0: 660f74d8 pcmpeqb xmm3, xmm0
1408309f4: 66440fd7db pmovmskb r11d, xmm3
1408309f9: 4585db test r11d, r11d
1408309fc: 752f jne 0x140830a2d
1408309fe: 660f74d1 pcmpeqb xmm2, xmm1
140830a02: 66440fd7da pmovmskb r11d, xmm2
140830a07: 4585db test r11d, r11d
140830a0a: 7557 jne 0x140830a63
140830a0c: 4d01d1 add r9, r10
140830a0f: 4983c110 add r9, 0x10
140830a13: 4983c210 add r10, 0x10
140830a17: 4d21c1 and r9, r8
140830a1a: ebca jmp 0x1408309e6 ; unknown
140830a1c: 0f1f4000 nop dword ptr [rax]
140830a20: 458d63ff lea r12d, [r11 - 1]
140830a24: 664521dc and r12w, r11w
140830a28: 4589e3 mov r11d, r12d
140830a2b: 74d1 je 0x1408309fe
140830a2d: f3450fbce3 tzcnt r12d, r11d
140830a32: 4d01cc add r12, r9
140830a35: 4d21c4 and r12, r8
140830a38: 49f7dc neg r12
140830a3b: 4f8d2464 lea r12, [r12 + r12*2]
140830a3f: 4a394ce2e8 cmp qword ptr [rdx + r12*8 - 0x18], rcx
140830a44: 75da jne 0x140830a20
140830a46: 4a3944e2f0 cmp qword ptr [rdx + r12*8 - 0x10], rax
140830a4b: 75d3 jne 0x140830a20
140830a4d: 4a8d04e2 lea rax, [rdx + r12*8]
140830a51: 4c8b60f8 mov r12, qword ptr [rax - 8]
140830a55: 483b3e cmp rdi, qword ptr [rsi]
140830a58: 0f8460030000 je 0x140830dbe
140830a5e: e904040000 jmp 0x140830e67 ; unknown
140830a63: 48899d48020000 mov qword ptr [rbp + 0x248], rbx
140830a6a: 410fb65760 movzx edx, byte ptr [r15 + 0x60]
140830a6f: 498b9780000000 mov rdx, qword ptr [r15 + 0x80]
140830a76: 4d8b8f88000000 mov r9, qword ptr [r15 + 0x88]
140830a7d: 4d89ca mov r10, r9
140830a80: 4921c2 and r10, rax
140830a83: 4531db xor r11d, r11d
140830a86: 660f6f05b2122602 movdqa xmm0, xmmword ptr [rip + 0x22612b2]
140830a8e: 660f76c9 pcmpeqd xmm1, xmm1
140830a92: f3420f6f1412 movdqu xmm2, xmmword ptr [rdx + r10]
140830a98: 660f6fda movdqa xmm3, xmm2
140830a9c: 660f74d8 pcmpeqb xmm3, xmm0
140830aa0: 66440fd7e3 pmovmskb r12d, xmm3
140830aa5: 4585e4 test r12d, r12d
140830aa8: 7534 jne 0x140830ade
140830aaa: 660f74d1 pcmpeqb xmm2, xmm1
140830aae: 66440fd7c2 pmovmskb r8d, xmm2
140830ab3: 4585c0 test r8d, r8d
140830ab6: 0f85f2000000 jne 0x140830bae
140830abc: 4d01da add r10, r11
140830abf: 4983c210 add r10, 0x10
140830ac3: 4983c310 add r11, 0x10
140830ac7: 4d21ca and r10, r9
140830aca: ebc6 jmp 0x140830a92 ; unknown
140830acc: 0f1f4000 nop dword ptr [rax]
140830ad0: 458d4424ff lea r8d, [r12 - 1]
140830ad5: 664521e0 and r8w, r12w
140830ad9: 4589c4 mov r12d, r8d
140830adc: 74cc je 0x140830aaa
140830ade: f3450fbcec tzcnt r13d, r12d
140830ae3: 4d01d5 add r13, r10
140830ae6: 4d21cd and r13, r9
140830ae9: 4d89e8 mov r8, r13
140830aec: 49f7d8 neg r8
140830aef: 4a8d1ced00000000 lea rbx, [r13*8]
140830af7: 49c1e007 shl r8, 7
140830afb: 4929d8 sub r8, rbx
140830afe: 4a398c0278ffffff cmp qword ptr [rdx + r8 - 0x88], rcx
140830b06: 75c8 jne 0x140830ad0
140830b08: 4a39440280 cmp qword ptr [rdx + r8 - 0x80], rax
140830b0d: 75c1 jne 0x140830ad0
140830b0f: 4d89ea mov r10, r13
140830b12: 49c1e207 shl r10, 7
140830b16: 4f8d1cea lea r11, [r10 + r13*8]
140830b1a: 49c1fb03 sar r11, 3
140830b1e: 49baf1f0f0f0f0f0f0f0 movabs r10, 0xf0f0f0f0f0f0f0f1
140830b28: 4d0fafd3 imul r10, r11
140830b2c: 4d8d5af0 lea r11, [r10 - 0x10]
140830b30: 4d21cb and r11, r9
140830b33: f3420f6f041a movdqu xmm0, xmmword ptr [rdx + r11]
140830b39: 660f76c9 pcmpeqd xmm1, xmm1
140830b3d: 660f74c1 pcmpeqb xmm0, xmm1
140830b41: 66440fd7e0 pmovmskb r12d, xmm0
140830b46: f3420f6f0412 movdqu xmm0, xmmword ptr [rdx + r10]
140830b4c: 660f74c1 pcmpeqb xmm0, xmm1
140830b50: 66440fd7c8 pmovmskb r9d, xmm0
140830b55: 4585e4 test r12d, r12d
140830b58: 740b je 0x140830b65
140830b5a: 66450fbde4 bsr r12w, r12w
140830b5f: 4183f40f xor r12d, 0xf
140830b63: eb05 jmp 0x140830b6a ; unknown
140830b65: 6641bc1000 mov r12w, 0x10
140830b6a: 4901d0 add r8, rdx
140830b6d: 4181c900000100 or r9d, 0x10000
140830b74: f3450fbcc9 tzcnt r9d, r9d
140830b79: 4501cc add r12d, r9d
140830b7c: 41b180 mov r9b, 0x80
140830b7f: 664183fc0f cmp r12w, 0xf
140830b84: 770a ja 0x140830b90
140830b86: 49ff8790000000 inc qword ptr [r15 + 0x90]
140830b8d: 41b1ff mov r9b, 0xff
140830b90: 46880c12 mov byte ptr [rdx + r10], r9b
140830b94: 46884c1a10 mov byte ptr [rdx + r11 + 0x10], r9b
140830b99: 49ff8f98000000 dec qword ptr [r15 + 0x98]
140830ba0: 498b5088 mov rdx, qword ptr [r8 - 0x78]
140830ba4: 4883fa02 cmp rdx, 2
140830ba8: 0f8534020000 jne 0x140830de2
140830bae: 498b5608 mov rdx, qword ptr [r14 + 8]
140830bb2: 4c8b22 mov r12, qword ptr [rdx]
140830bb5: 4d8d6c2401 lea r13, [r12 + 1]
140830bba: 4c892a mov qword ptr [rdx], r13
140830bbd: 48ba0000000000000080 movabs rdx, 0x8000000000000000
140830bc7: 48899508020000 mov qword ptr [rbp + 0x208], rdx
140830bce: 488d15655a4002 lea rdx, [rip + 0x2405a65]
140830bd5: 48899510020000 mov qword ptr [rbp + 0x210], rdx
140830bdc: 48c7851802000034000000 mov qword ptr [rbp + 0x218], 0x34
140830be7: 66c785280200000001 mov word ptr [rbp + 0x228], 0x100
140830bf0: 48c785d001000001000000 mov qword ptr [rbp + 0x1d0], 1
140830bfb: 48898dd8010000 mov qword ptr [rbp + 0x1d8], rcx
140830c02: 488985e0010000 mov qword ptr [rbp + 0x1e0], rax
140830c09: 48c785f801000001000000 mov qword ptr [rbp + 0x1f8], 1
140830c14: 48c7850002000001000000 mov qword ptr [rbp + 0x200], 1
140830c1f: 48c7852002000000000000 mov qword ptr [rbp + 0x220], 0
140830c2a: c6852a02000001 mov byte ptr [rbp + 0x22a], 1
140830c31: 48c785e801000002000000 mov qword ptr [rbp + 0x1e8], 2
140830c3c: 488d05bdf0ab00 lea rax, [rip + 0xabf0bd]
140830c43: 488985f0010000 mov qword ptr [rbp + 0x1f0], rax
140830c4a: 488d8d80000000 lea rcx, [rbp + 0x80]
140830c51: 4c8d85d0010000 lea r8, [rbp + 0x1d0]
140830c58: 4c89e2 mov rdx, r12
140830c5b: e810ec5000 call 0x140d3f870 ; bevy_ecs::component::ComponentInfo::new
140830c60: 4d396f10 cmp qword ptr [r15 + 0x10], r13
140830c64: 7319 jae 0x140830c7f
140830c66: c6855f02000001 mov byte ptr [rbp + 0x25f], 1
140830c6d: 4c8d05ec132602 lea r8, [rip + 0x22613ec]
140830c74: 4c89f9 mov rcx, r15
140830c77: 4c89ea mov rdx, r13
140830c7a: e8c136ac00 call 0x1412f4340 ; alloc::vec::Vec::resize_with<enum2$<core::option::Option<bevy_ecs::component::ComponentInfo> >,alloc::alloc::Global,bevy_ecs::component::impl$19::register_component_inner::closure_
140830c7f: 498b5f08 mov rbx, qword ptr [r15 + 8]
140830c83: 4d69fcd0000000 imul r15, r12, 0xd0
140830c8a: 4a8d043b lea rax, [rbx + r15]
140830c8e: 48898550020000 mov qword ptr [rbp + 0x250], rax
140830c95: 488d4db0 lea rcx, [rbp - 0x50]
140830c99: 488d9580000000 lea rdx, [rbp + 0x80]
140830ca0: 41b8d0000000 mov r8d, 0xd0
140830ca6: e8552f0f02 call 0x142923c00 ; unknown
140830cab: 42833c3b02 cmp dword ptr [rbx + r15], 2
140830cb0: 740c je 0x140830cbe
140830cb2: 488b8d50020000 mov rcx, qword ptr [rbp + 0x250]
140830cb9: e892fedbff call 0x1405f0b50 ; core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo>
140830cbe: 488d55b0 lea rdx, [rbp - 0x50]
140830cc2: 41b8d0000000 mov r8d, 0xd0
140830cc8: 488b8d50020000 mov rcx, qword ptr [rbp + 0x250]
140830ccf: e82c2f0f02 call 0x142923c00 ; unknown
140830cd4: 498b0e mov rcx, qword ptr [r14]
140830cd7: 4883c118 add rcx, 0x18
140830cdb: 48badf13ac4c1f79bee9 movabs rdx, 0xe9be791f4cac13df
140830ce5: 49b83815e1791d76a7da movabs r8, 0xdaa7761d79e11538
140830cef: 4d89e1 mov r9, r12
140830cf2: e8291f85ff call 0x140082c20 ; hashbrown::map::HashMap::insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global>
140830cf7: 0f1005e2102602 movups xmm0, xmmword ptr [rip + 0x22610e2]
140830cfe: 0f298590000000 movaps xmmword ptr [rbp + 0x90], xmm0
140830d05: f30f6f05c3102602 movdqu xmm0, xmmword ptr [rip + 0x22610c3]
140830d0d: 660f7f8580000000 movdqa xmmword ptr [rbp + 0x80], xmm0
140830d15: c6855e02000001 mov byte ptr [rbp + 0x25e], 1
140830d1c: 488d8530020000 lea rax, [rbp + 0x230]
140830d23: 4889442420 mov qword ptr [rsp + 0x20], rax
140830d28: 4c89f2 mov rdx, r14
140830d2b: e810fcffff call 0x140830940 ; country_core::components::drawable::impl$28::register_required_components
140830d30: 498b06 mov rax, qword ptr [r14]
140830d33: 488b4008 mov rax, qword ptr [rax + 8]
140830d37: 0f288580000000 movaps xmm0, xmmword ptr [rbp + 0x80]
140830d3e: 0f288d90000000 movaps xmm1, xmmword ptr [rbp + 0x90]
140830d45: 0f294dc0 movaps xmmword ptr [rbp - 0x40], xmm1
140830d49: 0f2945b0 movaps xmmword ptr [rbp - 0x50], xmm0
140830d4d: 4a8d4c3860 lea rcx, [rax + r15 + 0x60]
140830d52: 4a8d943880000000 lea rdx, [rax + r15 + 0x80]
140830d5a: 41b820000000 mov r8d, 0x20
140830d60: 41b910000000 mov r9d, 0x10
140830d66: 48898d50020000 mov qword ptr [rbp + 0x250], rcx
140830d6d: e8ce0582ff call 0x140051340 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<bevy_ecs::component::ComponentId,bevy_ecs::component::RequiredComponent>,allocator_api2::stable::alloc::global::Global>
140830d72: 488b9d48020000 mov rbx, qword ptr [rbp + 0x248]
140830d79: 660f6f45b0 movdqa xmm0, xmmword ptr [rbp - 0x50]
140830d7e: 660f6f4dc0 movdqa xmm1, xmmword ptr [rbp - 0x40]
140830d83: 488b8550020000 mov rax, qword ptr [rbp + 0x250]
140830d8a: f30f7f4810 movdqu xmmword ptr [rax + 0x10], xmm1
140830d8f: f30f7f00 movdqu xmmword ptr [rax], xmm0
140830d93: 488b9530020000 mov rdx, qword ptr [rbp + 0x230]
140830d9a: 4885d2 test rdx, rdx
140830d9d: 7416 je 0x140830db5
140830d9f: 488b8d38020000 mov rcx, qword ptr [rbp + 0x238]
140830da6: 48c1e203 shl rdx, 3
140830daa: 41b808000000 mov r8d, 8
140830db0: e86b2edeff call 0x140613c20 ; unknown
140830db5: 483b3e cmp rdi, qword ptr [rsi]
140830db8: 0f85a9000000 jne 0x140830e67
140830dbe: 488d1543142602 lea rdx, [rip + 0x2261443]
140830dc5: 4889f1 mov rcx, rsi
140830dc8: e823103500 call 0x140b81df0 ; alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>
140830dcd: 488b4608 mov rax, qword ptr [rsi + 8]
140830dd1: 4c8924f8 mov qword ptr [rax + rdi*8], r12
140830dd5: 488d4701 lea rax, [rdi + 1]
140830dd9: 48894610 mov qword ptr [rsi + 0x10], rax
140830ddd: e995000000 jmp 0x140830e77 ; unknown
140830de2: 410f1040f0 movups xmm0, xmmword ptr [r8 - 0x10]
140830de7: 0f1185c0010000 movups xmmword ptr [rbp + 0x1c0], xmm0
140830dee: 410f1040e0 movups xmm0, xmmword ptr [r8 - 0x20]
140830df3: 0f1185b0010000 movups xmmword ptr [rbp + 0x1b0], xmm0
140830dfa: 410f1040d0 movups xmm0, xmmword ptr [r8 - 0x30]
140830dff: 0f1185a0010000 movups xmmword ptr [rbp + 0x1a0], xmm0
140830e06: f3410f6f4090 movdqu xmm0, xmmword ptr [r8 - 0x70]
140830e0c: f3410f6f48a0 movdqu xmm1, xmmword ptr [r8 - 0x60]
140830e12: f3410f6f50b0 movdqu xmm2, xmmword ptr [r8 - 0x50]
140830e18: f3410f6f58c0 movdqu xmm3, xmmword ptr [r8 - 0x40]
140830e1e: f30f7f9d90010000 movdqu xmmword ptr [rbp + 0x190], xmm3
140830e26: f30f7f9580010000 movdqu xmmword ptr [rbp + 0x180], xmm2
140830e2e: f30f7f8d70010000 movdqu xmmword ptr [rbp + 0x170], xmm1
140830e36: f30f7f8560010000 movdqu xmmword ptr [rbp + 0x160], xmm0
140830e3e: 48899558010000 mov qword ptr [rbp + 0x158], rdx
140830e45: 488d8d58010000 lea rcx, [rbp + 0x158]
140830e4c: 4c89f2 mov rdx, r14
140830e4f: e8eceb5000 call 0x140d3fa40 ; bevy_ecs::component::QueuedRegistration::register
140830e54: 4989c4 mov r12, rax
140830e57: 488b9d48020000 mov rbx, qword ptr [rbp + 0x248]
140830e5e: 483b3e cmp rdi, qword ptr [rsi]
140830e61: 0f8457ffffff je 0x140830dbe
140830e67: 4c8924fb mov qword ptr [rbx + rdi*8], r12
140830e6b: 4889f8 mov rax, rdi
140830e6e: 48ffc0 inc rax
140830e71: 48894610 mov qword ptr [rsi + 0x10], rax
140830e75: 7404 je 0x140830e7b
140830e77: 48897e10 mov qword ptr [rsi + 0x10], rdi
140830e7b: 4881c4e8020000 add rsp, 0x2e8
140830e82: 5b pop rbx
140830e83: 5f pop rdi
140830e84: 5e pop rsi
140830e85: 415c pop r12
140830e87: 415d pop r13
140830e89: 415e pop r14
140830e8b: 415f pop r15
140830e8d: 5d pop rbp
140830e8e: c3 ret 
140830e8f: 90 nop 
140830e90: 4889542410 mov qword ptr [rsp + 0x10], rdx
140830e95: 55 push rbp
140830e96: 4157 push r15
140830e98: 4156 push r14
140830e9a: 4155 push r13
140830e9c: 4154 push r12
140830e9e: 56 push rsi
140830e9f: 57 push rdi
140830ea0: 53 push rbx
140830ea1: 4883ec28 sub rsp, 0x28
140830ea5: 488daa80000000 lea rbp, [rdx + 0x80]
140830eac: 488d55b0 lea rdx, [rbp - 0x50]
140830eb0: 41b8d0000000 mov r8d, 0xd0
140830eb6: 488b8d50020000 mov rcx, qword ptr [rbp + 0x250]
140830ebd: e83e2d0f02 call 0x142923c00 ; unknown
140830ec2: c6855f02000000 mov byte ptr [rbp + 0x25f], 0
140830ec9: 4883c428 add rsp, 0x28
140830ecd: 5b pop rbx
140830ece: 5f pop rdi
140830ecf: 5e pop rsi
140830ed0: 415c pop r12
140830ed2: 415d pop r13
140830ed4: 415e pop r14
140830ed6: 415f pop r15
140830ed8: 5d pop rbp
140830ed9: c3 ret 
140830eda: 660f1f440000 nop word ptr [rax + rax]
140830ee0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140830ee5: 55 push rbp
140830ee6: 4157 push r15
140830ee8: 4156 push r14
140830eea: 4155 push r13
140830eec: 4154 push r12
140830eee: 56 push rsi
140830eef: 57 push rdi
140830ef0: 53 push rbx
140830ef1: 4883ec28 sub rsp, 0x28
140830ef5: 488daa80000000 lea rbp, [rdx + 0x80]
140830efc: 80bd5f02000000 cmp byte ptr [rbp + 0x25f], 0
140830f03: 740c je 0x140830f11
140830f05: 488d8d80000000 lea rcx, [rbp + 0x80]
140830f0c: e83ffcdbff call 0x1405f0b50 ; core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo>
140830f11: 90 nop 
140830f12: 4883c428 add rsp, 0x28
140830f16: 5b pop rbx
140830f17: 5f pop rdi
140830f18: 5e pop rsi
140830f19: 415c pop r12
140830f1b: 415d pop r13
140830f1d: 415e pop r14
140830f1f: 415f pop r15
140830f21: 5d pop rbp
140830f22: c3 ret 
140830f23: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140830f30: 4889542410 mov qword ptr [rsp + 0x10], rdx
140830f35: 55 push rbp
140830f36: 4157 push r15
140830f38: 4156 push r14
140830f3a: 4155 push r13
140830f3c: 4154 push r12
140830f3e: 56 push rsi
140830f3f: 57 push rdi
140830f40: 53 push rbx
140830f41: 4883ec28 sub rsp, 0x28
140830f45: 488daa80000000 lea rbp, [rdx + 0x80]
140830f4c: 660f6f45b0 movdqa xmm0, xmmword ptr [rbp - 0x50]
140830f51: 660f6f4dc0 movdqa xmm1, xmmword ptr [rbp - 0x40]
140830f56: 488b8550020000 mov rax, qword ptr [rbp + 0x250]
140830f5d: f30f7f4810 movdqu xmmword ptr [rax + 0x10], xmm1
140830f62: f30f7f00 movdqu xmmword ptr [rax], xmm0
140830f66: c6855e02000000 mov byte ptr [rbp + 0x25e], 0
140830f6d: 4883c428 add rsp, 0x28
140830f71: 5b pop rbx
140830f72: 5f pop rdi
140830f73: 5e pop rsi
140830f74: 415c pop r12
140830f76: 415d pop r13
140830f78: 415e pop r14
140830f7a: 415f pop r15
140830f7c: 5d pop rbp
140830f7d: c3 ret 
140830f7e: 6690 nop 
140830f80: 4889542410 mov qword ptr [rsp + 0x10], rdx
140830f85: 55 push rbp
140830f86: 4157 push r15
140830f88: 4156 push r14
140830f8a: 4155 push r13
140830f8c: 4154 push r12
140830f8e: 56 push rsi
140830f8f: 57 push rdi
140830f90: 53 push rbx
140830f91: 4883ec28 sub rsp, 0x28
140830f95: 488daa80000000 lea rbp, [rdx + 0x80]
140830f9c: 80bd5e02000000 cmp byte ptr [rbp + 0x25e], 0
140830fa3: 741f je 0x140830fc4
140830fa5: 488d95a0000000 lea rdx, [rbp + 0xa0]
140830fac: 488d8d80000000 lea rcx, [rbp + 0x80]
140830fb3: 41b820000000 mov r8d, 0x20
140830fb9: 41b910000000 mov r9d, 0x10
140830fbf: e87c0382ff call 0x140051340 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<bevy_ecs::component::ComponentId,bevy_ecs::component::RequiredComponent>,allocator_api2::stable::alloc::global::Global>
140830fc4: 90 nop 
140830fc5: 4883c428 add rsp, 0x28
140830fc9: 5b pop rbx
140830fca: 5f pop rdi
140830fcb: 5e pop rsi
140830fcc: 415c pop r12
140830fce: 415d pop r13
140830fd0: 415e pop r14
140830fd2: 415f pop r15
140830fd4: 5d pop rbp
140830fd5: c3 ret 
140830fd6: 662e0f1f840000000000 nop word ptr cs:[rax + rax]
140830fe0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140830fe5: 55 push rbp
140830fe6: 4157 push r15
140830fe8: 4156 push r14
140830fea: 4155 push r13
140830fec: 4154 push r12
140830fee: 56 push rsi
140830fef: 57 push rdi
140830ff0: 53 push rbx
140830ff1: 4883ec28 sub rsp, 0x28
140830ff5: 488daa80000000 lea rbp, [rdx + 0x80]
140830ffc: 488b9530020000 mov rdx, qword ptr [rbp + 0x230]
140831003: 4885d2 test rdx, rdx
140831006: 7416 je 0x14083101e
140831008: 488b8d38020000 mov rcx, qword ptr [rbp + 0x238]
14083100f: 48c1e203 shl rdx, 3
140831013: 41b808000000 mov r8d, 8
140831019: e8022cdeff call 0x140613c20 ; unknown
14083101e: 90 nop 
14083101f: 4883c428 add rsp, 0x28
140831023: 5b pop rbx
140831024: 5f pop rdi
140831025: 5e pop rsi
140831026: 415c pop r12
140831028: 415d pop r13
14083102a: 415e pop r14
14083102c: 415f pop r15
14083102e: 5d pop rbp
14083102f: c3 ret 
