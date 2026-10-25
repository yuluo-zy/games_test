140b1ae70: 55 push rbp
140b1ae71: 56 push rsi
140b1ae72: 4881ece8000000 sub rsp, 0xe8
140b1ae79: 488dac2480000000 lea rbp, [rsp + 0x80]
140b1ae81: 48c74560feffffff mov qword ptr [rbp + 0x60], 0xfffffffffffffffe
140b1ae89: 4889d6 mov rsi, rdx
140b1ae8c: 488b01 mov rax, qword ptr [rcx]
140b1ae8f: 488b10 mov rdx, qword ptr [rax]
140b1ae92: 4885d2 test rdx, rdx
140b1ae95: 745a je 0x140b1aef1
140b1ae97: 4883c210 add rdx, 0x10
140b1ae9b: 4c8d0506e6fc01 lea r8, [rip + 0x1fce606] ; 0x142ae94a8 'snapshot:/Loadingcrates/country-core/src/systems/audio/game_loading_audio.rs'
140b1aea2: 488d4d20 lea rcx, [rbp + 0x20]
140b1aea6: 41b911000000 mov r9d, 0x11
140b1aeac: e88f95f3ff call 0x140a54440 ; country_core::resources::audio_resources::AudioCoreInner::instantiate_audio
140b1aeb1: 48837d2002 cmp qword ptr [rbp + 0x20], 2
140b1aeb6: 0f8582000000 jne 0x140b1af3e
140b1aebc: 488b4528 mov rax, qword ptr [rbp + 0x28]
140b1aec0: 488945a8 mov qword ptr [rbp - 0x58], rax
140b1aec4: 488d052de6fc01 lea rax, [rip + 0x1fce62d]
140b1aecb: 4889442420 mov qword ptr [rsp + 0x20], rax
140b1aed0: 488d0db1e1fc01 lea rcx, [rip + 0x1fce1b1] ; 0x142ae9088 'called `Result::unwrap()` on an `Err` value'
140b1aed7: 4c8d0d8ae1fc01 lea r9, [rip + 0x1fce18a]
140b1aede: 4c8d45a8 lea r8, [rbp - 0x58]
140b1aee2: ba2b000000 mov edx, 0x2b
140b1aee7: e874e8db01 call 0x1428d9760 ; core::result::unwrap_failed
140b1aeec: e9b2010000 jmp 0x140b1b0a3
140b1aef1: 0fb605119db102 movzx eax, byte ptr [rip + 0x2b19d11]
140b1aef8: b911000000 mov ecx, 0x11
140b1aefd: ba01000000 mov edx, 1
140b1af02: e8098dafff call 0x140613c10 ; 
140b1af07: 4885c0 test rax, rax
140b1af0a: 0f8495010000 je 0x140b1b0a5
140b1af10: 0f100591e5fc01 movups xmm0, xmmword ptr [rip + 0x1fce591] ; 0x142ae94a8 'snapshot:/Loadingcrates/country-core/src/systems/audio/game_loading_audio.rs'
140b1af17: 0f1100 movups xmmword ptr [rax], xmm0
140b1af1a: c6401067 mov byte ptr [rax + 0x10], 0x67
140b1af1e: 48c7452000000000 mov qword ptr [rbp + 0x20], 0
140b1af26: 48c7453011000000 mov qword ptr [rbp + 0x30], 0x11
140b1af2e: 48894538 mov qword ptr [rbp + 0x38], rax
140b1af32: 48c7454011000000 mov qword ptr [rbp + 0x40], 0x11
140b1af3a: c6454802 mov byte ptr [rbp + 0x48], 2
140b1af3e: 488b4520 mov rax, qword ptr [rbp + 0x20]
140b1af42: 488b4d28 mov rcx, qword ptr [rbp + 0x28]
140b1af46: 488b5530 mov rdx, qword ptr [rbp + 0x30]
140b1af4a: 4c8b4538 mov r8, qword ptr [rbp + 0x38]
140b1af4e: 4c8b4d40 mov r9, qword ptr [rbp + 0x40]
140b1af52: 4c894d00 mov qword ptr [rbp], r9
140b1af56: 440fb64d48 movzx r9d, byte ptr [rbp + 0x48]
140b1af5b: 44884d08 mov byte ptr [rbp + 8], r9b
140b1af5f: 448b4d49 mov r9d, dword ptr [rbp + 0x49]
140b1af63: 44894d09 mov dword ptr [rbp + 9], r9d
140b1af67: 440fb74d4d movzx r9d, word ptr [rbp + 0x4d]
140b1af6c: 6644894d0d mov word ptr [rbp + 0xd], r9w
140b1af71: 440fb64d4f movzx r9d, byte ptr [rbp + 0x4f]
140b1af76: 44884d0f mov byte ptr [rbp + 0xf], r9b
140b1af7a: 488955f0 mov qword ptr [rbp - 0x10], rdx
140b1af7e: 4c8945f8 mov qword ptr [rbp - 8], r8
140b1af82: 488945e0 mov qword ptr [rbp - 0x20], rax
140b1af86: 48894de8 mov qword ptr [rbp - 0x18], rcx
140b1af8a: 837de001 cmp dword ptr [rbp - 0x20], 1
140b1af8e: 7525 jne 0x140b1afb5
140b1af90: 488d55e8 lea rdx, [rbp - 0x18]
140b1af94: c6455f01 mov byte ptr [rbp + 0x5f], 1
140b1af98: 488d4da8 lea rcx, [rbp - 0x58]
140b1af9c: e82fc39100 call 0x1414372d0 ; libfmod::EventInstance::start
140b1afa1: 48b80400000000000080 movabs rax, 0x8000000000000004
140b1afab: 483945a8 cmp qword ptr [rbp - 0x58], rax
140b1afaf: 0f85a6000000 jne 0x140b1b05b
140b1afb5: 488b45e0 mov rax, qword ptr [rbp - 0x20]
140b1afb9: 488b4de8 mov rcx, qword ptr [rbp - 0x18]
140b1afbd: 488b55f0 mov rdx, qword ptr [rbp - 0x10]
140b1afc1: 4c8b45f8 mov r8, qword ptr [rbp - 8]
140b1afc5: 4c8b4d00 mov r9, qword ptr [rbp]
140b1afc9: 4c894d40 mov qword ptr [rbp + 0x40], r9
140b1afcd: 448b4d08 mov r9d, dword ptr [rbp + 8]
140b1afd1: 44894d48 mov dword ptr [rbp + 0x48], r9d
140b1afd5: 440fb74d0c movzx r9d, word ptr [rbp + 0xc]
140b1afda: 6644894d4c mov word ptr [rbp + 0x4c], r9w
140b1afdf: 440fb64d0e movzx r9d, byte ptr [rbp + 0xe]
140b1afe4: 44884d4e mov byte ptr [rbp + 0x4e], r9b
140b1afe8: 440fb64d0f movzx r9d, byte ptr [rbp + 0xf]
140b1afed: 44884d4f mov byte ptr [rbp + 0x4f], r9b
140b1aff1: 48895530 mov qword ptr [rbp + 0x30], rdx
140b1aff5: 4c894538 mov qword ptr [rbp + 0x38], r8
140b1aff9: 48894520 mov qword ptr [rbp + 0x20], rax
140b1affd: 48894d28 mov qword ptr [rbp + 0x28], rcx
140b1b001: 8b461c mov eax, dword ptr [rsi + 0x1c]
140b1b004: 488b4e10 mov rcx, qword ptr [rsi + 0x10]
140b1b008: 8901 mov dword ptr [rcx], eax
140b1b00a: 488b36 mov rsi, qword ptr [rsi]
140b1b00d: 833e02 cmp dword ptr [rsi], 2
140b1b010: 7428 je 0x140b1b03a
140b1b012: 48897518 mov qword ptr [rbp + 0x18], rsi
140b1b016: 4889f1 mov rcx, rsi
140b1b019: e862f2d7ff call 0x14089a280 ; country_core::resources::audio_resources::impl$13::drop
140b1b01e: 488b7518 mov rsi, qword ptr [rbp + 0x18]
140b1b022: 488b5610 mov rdx, qword ptr [rsi + 0x10]
140b1b026: 4885d2 test rdx, rdx
140b1b029: 740f je 0x140b1b03a
140b1b02b: 488b4e18 mov rcx, qword ptr [rsi + 0x18]
140b1b02f: 41b801000000 mov r8d, 1
140b1b035: e8e68bafff call 0x140613c20 ; 
140b1b03a: 0f284520 movaps xmm0, xmmword ptr [rbp + 0x20]
140b1b03e: 0f284d30 movaps xmm1, xmmword ptr [rbp + 0x30]
140b1b042: 0f285540 movaps xmm2, xmmword ptr [rbp + 0x40]
140b1b046: 0f115620 movups xmmword ptr [rsi + 0x20], xmm2
140b1b04a: 0f114e10 movups xmmword ptr [rsi + 0x10], xmm1
140b1b04e: 0f1106 movups xmmword ptr [rsi], xmm0
140b1b051: 4881c4e8000000 add rsp, 0xe8
140b1b058: 5e pop rsi
140b1b059: 5d pop rbp
140b1b05a: c3 ret 
140b1b05b: 488b45d8 mov rax, qword ptr [rbp - 0x28]
140b1b05f: 48894550 mov qword ptr [rbp + 0x50], rax
140b1b063: 0f1045a8 movups xmm0, xmmword ptr [rbp - 0x58]
140b1b067: 0f104db8 movups xmm1, xmmword ptr [rbp - 0x48]
140b1b06b: 0f1055c8 movups xmm2, xmmword ptr [rbp - 0x38]
140b1b06f: 0f295540 movaps xmmword ptr [rbp + 0x40], xmm2
140b1b073: 0f294d30 movaps xmmword ptr [rbp + 0x30], xmm1
140b1b077: 0f294520 movaps xmmword ptr [rbp + 0x20], xmm0
140b1b07b: 488d053e7bfb01 lea rax, [rip + 0x1fb7b3e]
140b1b082: 4889442420 mov qword ptr [rsp + 0x20], rax
140b1b087: 488d0d7255fb01 lea rcx, [rip + 0x1fb5572] ; 0x142ad0600 'called `Result::unwrap()` on an `Err` value'
140b1b08e: 4c8d0d2b55fb01 lea r9, [rip + 0x1fb552b]
140b1b095: 4c8d4520 lea r8, [rbp + 0x20]
140b1b099: ba2b000000 mov edx, 0x2b
140b1b09e: e8bde6db01 call 0x1428d9760 ; core::result::unwrap_failed
140b1b0a3: 0f0b ud2 
140b1b0a5: 4c8d050457fb01 lea r8, [rip + 0x1fb5704]
140b1b0ac: b901000000 mov ecx, 1
140b1b0b1: ba11000000 mov edx, 0x11
140b1b0b6: e828dfdb01 call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140b1b0bb: cc int3 
140b1b0bc: 0f1f4000 nop dword ptr [rax]
140b1b0c0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140b1b0c5: 55 push rbp
140b1b0c6: 56 push rsi
140b1b0c7: 4883ec28 sub rsp, 0x28
140b1b0cb: 488daa80000000 lea rbp, [rdx + 0x80]
140b1b0d2: c6455f01 mov byte ptr [rbp + 0x5f], 1
140b1b0d6: 488d4d20 lea rcx, [rbp + 0x20]
140b1b0da: e8d175f3ff call 0x140a526b0 ; core::ptr::drop_in_place<enum2$<libfmod::Error> >
140b1b0df: c6455f01 mov byte ptr [rbp + 0x5f], 1
140b1b0e3: 4883c428 add rsp, 0x28
140b1b0e7: 5e pop rsi
140b1b0e8: 5d pop rbp
140b1b0e9: c3 ret 
140b1b0ea: 660f1f440000 nop word ptr [rax + rax]
140b1b0f0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140b1b0f5: 55 push rbp
140b1b0f6: 56 push rsi
140b1b0f7: 4883ec28 sub rsp, 0x28
140b1b0fb: 488daa80000000 lea rbp, [rdx + 0x80]
140b1b102: 488b4518 mov rax, qword ptr [rbp + 0x18]
140b1b106: 488b5010 mov rdx, qword ptr [rax + 0x10]
140b1b10a: 4885d2 test rdx, rdx
140b1b10d: 7413 je 0x140b1b122
140b1b10f: 488b4518 mov rax, qword ptr [rbp + 0x18]
140b1b113: 488b4818 mov rcx, qword ptr [rax + 0x18]
140b1b117: 41b801000000 mov r8d, 1
140b1b11d: e8fe8aafff call 0x140613c20 ; 
140b1b122: 0f284520 movaps xmm0, xmmword ptr [rbp + 0x20]
140b1b126: 0f284d30 movaps xmm1, xmmword ptr [rbp + 0x30]
140b1b12a: 0f285540 movaps xmm2, xmmword ptr [rbp + 0x40]
140b1b12e: 488b4518 mov rax, qword ptr [rbp + 0x18]
140b1b132: 0f115020 movups xmmword ptr [rax + 0x20], xmm2
140b1b136: 0f114810 movups xmmword ptr [rax + 0x10], xmm1
140b1b13a: 0f1100 movups xmmword ptr [rax], xmm0
140b1b13d: c6455f00 mov byte ptr [rbp + 0x5f], 0
140b1b141: 4883c428 add rsp, 0x28
140b1b145: 5e pop rsi
140b1b146: 5d pop rbp
140b1b147: c3 ret 
140b1b148: 0f1f840000000000 nop dword ptr [rax + rax]
140b1b150: 4889542410 mov qword ptr [rsp + 0x10], rdx
140b1b155: 55 push rbp
140b1b156: 56 push rsi
140b1b157: 4883ec28 sub rsp, 0x28
140b1b15b: 488daa80000000 lea rbp, [rdx + 0x80]
140b1b162: 807d5f00 cmp byte ptr [rbp + 0x5f], 0
140b1b166: 7507 jne 0x140b1b16f
140b1b168: 4883c428 add rsp, 0x28
140b1b16c: 5e pop rsi
140b1b16d: 5d pop rbp
140b1b16e: c3 ret 
140b1b16f: 488d4de0 lea rcx, [rbp - 0x20]
140b1b173: e808f562ff call 0x14014a680 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::resources::critters::sheep::SheepAudio> | core::ptr::drop_in_place<tiny_glade::rollercoaster::CoasterAudioInstance> | core::ptr::drop_in_place<country_core::resources::audio_resources::AudioInstance> | core::ptr::drop_in_place<country_core::resources::critters::ButterflyAudio> | core::ptr::drop_in_place<country_core::resources::critters::sheep::SheepAudio> | core::ptr::drop_in_place<country_core::resources::audio_resources::AudioInstance> | bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::resources::critters::ButterflyAudio> | core::ptr::drop_in_place<country_core::resources::critters::Wren> | core::ptr::drop_in_place<country_core::resources::critters::ButterflyAudio> | bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::resources::critters::Wren>
140b1b178: ebee jmp 0x140b1b168
140b1b17a: 660f1f440000 nop word ptr [rax + rax]
140b1b180: 4889542410 mov qword ptr [rsp + 0x10], rdx
140b1b185: 55 push rbp
140b1b186: 56 push rsi
140b1b187: 4883ec28 sub rsp, 0x28
140b1b18b: 488daa80000000 lea rbp, [rdx + 0x80]
140b1b192: 488d4da8 lea rcx, [rbp - 0x58]
140b1b196: e8951a2300 call 0x140d4cc30 ; anyhow::error::impl$6::drop
140b1b19b: 90 nop 
140b1b19c: 4883c428 add rsp, 0x28
140b1b1a0: 5e pop rsi
140b1b1a1: 5d pop rbp
140b1b1a2: c3 ret 
