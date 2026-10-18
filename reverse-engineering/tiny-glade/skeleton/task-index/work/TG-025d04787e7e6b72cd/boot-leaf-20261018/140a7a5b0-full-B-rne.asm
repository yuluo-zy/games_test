140a7a5b0: 55 push rbp
140a7a5b1: 4157 push r15
140a7a5b3: 4156 push r14
140a7a5b5: 4155 push r13
140a7a5b7: 4154 push r12
140a7a5b9: 56 push rsi
140a7a5ba: 57 push rdi
140a7a5bb: 53 push rbx
140a7a5bc: 4881ecf8000000 sub rsp, 0xf8
140a7a5c3: 488dac2480000000 lea rbp, [rsp + 0x80]
140a7a5cb: 48c74570feffffff mov qword ptr [rbp + 0x70], 0xfffffffffffffffe
140a7a5d3: 4c89ce mov rsi, r9
140a7a5d6: 4c89c7 mov rdi, r8
140a7a5d9: 4889d3 mov rbx, rdx
140a7a5dc: 4c8bbdf8000000 mov r15, qword ptr [rbp + 0xf8]
140a7a5e3: 4c8ba5e8000000 mov r12, qword ptr [rbp + 0xe8]
140a7a5ea: 4c8bade0000000 mov r13, qword ptr [rbp + 0xe0]
140a7a5f1: 8b411c mov eax, dword ptr [rcx + 0x1c]
140a7a5f4: 488b5110 mov rdx, qword ptr [rcx + 0x10]
140a7a5f8: 8902 mov dword ptr [rdx], eax
140a7a5fa: 4c8b31 mov r14, qword ptr [rcx]
140a7a5fd: 4c89f1 mov rcx, r14
140a7a600: e82bf70f00 call 0x140b79d30 ; hashbrown::raw::RawTable::clear<tuple$<country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo>,tuple$<country_core::render::ssbo::TypedStorageBuffer<country_core::resources::placement_data::PlacementDataSsbo>,usize> >,alloc::alloc::Global>
140a7a605: 4983c630 add r14, 0x30
140a7a609: 4c89f1 mov rcx, r14
140a7a60c: e8bff40f00 call 0x140b79ad0 ; hashbrown::raw::RawTable::clear<tuple$<std::path::PathBuf,country_core::asset_libraries::Handle<country_core::resources::placement_data::PlacementDataSsbo> >,alloc::alloc::Global>
140a7a611: 8b431c mov eax, dword ptr [rbx + 0x1c]
140a7a614: 488b4b10 mov rcx, qword ptr [rbx + 0x10]
140a7a618: 8901 mov dword ptr [rcx], eax
140a7a61a: 488b03 mov rax, qword ptr [rbx]
140a7a61d: 488945b0 mov qword ptr [rbp - 0x50], rax
140a7a621: 48c745b801000000 mov qword ptr [rbp - 0x48], 1
140a7a629: 488d5d20 lea rbx, [rbp + 0x20]
140a7a62d: 4c8d75b0 lea r14, [rbp - 0x50]
140a7a631: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a7a640: 4889d9 mov rcx, rbx
140a7a643: 4c89f2 mov rdx, r14
140a7a646: e8e5ecad00 call 0x141559330 ; slotmap::basic::impl$9::next<country_core::asset_libraries::audio_trigger_library::AudioTriggerCollectionHandle,utils::geometry::aabb::Aabb3>
140a7a64b: 837d2000 cmp dword ptr [rbp + 0x20], 0
140a7a64f: 75ef jne 0x140a7a640
140a7a651: 8b471c mov eax, dword ptr [rdi + 0x1c]
140a7a654: 488b4f10 mov rcx, qword ptr [rdi + 0x10]
140a7a658: 8901 mov dword ptr [rcx], eax
140a7a65a: 488b3f mov rdi, qword ptr [rdi]
140a7a65d: 4889f9 mov rcx, rdi
140a7a660: e85bee0f00 call 0x140b794c0 ; hashbrown::raw::RawTable::clear<tuple$<enum2$<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh> >,alloc::alloc::Global>
140a7a665: c6473801 mov byte ptr [rdi + 0x38], 1
140a7a669: 8b461c mov eax, dword ptr [rsi + 0x1c]
140a7a66c: 488b4e10 mov rcx, qword ptr [rsi + 0x10]
140a7a670: 8901 mov dword ptr [rcx], eax
140a7a672: 488b36 mov rsi, qword ptr [rsi]
140a7a675: 31ff xor edi, edi
140a7a677: 48bb0200000000000080 movabs rbx, 0x8000000000000002
140a7a681: eb48 jmp 0x140a7a6cb
140a7a683: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a7a690: 48c7443e3800000000 mov qword ptr [rsi + rdi + 0x38], 0
140a7a699: 498d4601 lea rax, [r14 + 1]
140a7a69d: 4889c1 mov rcx, rax
140a7a6a0: 48c1e903 shr rcx, 3
140a7a6a4: 4883e0f8 and rax, 0xfffffffffffffff8
140a7a6a8: 4829c8 sub rax, rcx
140a7a6ab: 4983fe08 cmp r14, 8
140a7a6af: 490f42c6 cmovb rax, r14
140a7a6b3: 4889443e30 mov qword ptr [rsi + rdi + 0x30], rax
140a7a6b8: 48c7843ea800000000000000 mov qword ptr [rsi + rdi + 0xa8], 0
140a7a6c4: 4881c7c0000000 add rdi, 0xc0
140a7a6cb: 4881ff40140000 cmp rdi, 0x1440
140a7a6d2: 7434 je 0x140a7a708
140a7a6d4: 48395c3e08 cmp qword ptr [rsi + rdi + 8], rbx
140a7a6d9: 7ce9 jl 0x140a7a6c4
140a7a6db: 48c7443e1800000000 mov qword ptr [rsi + rdi + 0x18], 0
140a7a6e4: 48837c3e3800 cmp qword ptr [rsi + rdi + 0x38], 0
140a7a6ea: 74cc je 0x140a7a6b8
140a7a6ec: 4c8b743e28 mov r14, qword ptr [rsi + rdi + 0x28]
140a7a6f1: 4d85f6 test r14, r14
140a7a6f4: 749a je 0x140a7a690
140a7a6f6: 488b4c3e20 mov rcx, qword ptr [rsi + rdi + 0x20]
140a7a6fb: 4d8d4611 lea r8, [r14 + 0x11]
140a7a6ff: b2ff mov dl, 0xff
140a7a701: e81a95ea01 call 0x142923c20 ; 
140a7a706: eb88 jmp 0x140a7a690
140a7a708: 418b44241c mov eax, dword ptr [r12 + 0x1c]
140a7a70d: 498b4c2410 mov rcx, qword ptr [r12 + 0x10]
140a7a712: 8901 mov dword ptr [rcx], eax
140a7a714: 498b3424 mov rsi, qword ptr [r12]
140a7a718: 488b4e08 mov rcx, qword ptr [rsi + 8]
140a7a71c: 488b4610 mov rax, qword ptr [rsi + 0x10]
140a7a720: 48c7461000000000 mov qword ptr [rsi + 0x10], 0
140a7a728: 4885c0 test rax, rax
140a7a72b: 7447 je 0x140a7a774
140a7a72d: ba01000000 mov edx, 1
140a7a732: 4829c2 sub rdx, rax
140a7a735: eb21 jmp 0x140a7a758
140a7a737: 660f1f840000000000 nop word ptr [rax + rax]
140a7a740: 488b4560 mov rax, qword ptr [rbp + 0x60]
140a7a744: 4883c050 add rax, 0x50
140a7a748: 488b55e8 mov rdx, qword ptr [rbp - 0x18]
140a7a74c: 48ffc2 inc rdx
140a7a74f: 4883fa01 cmp rdx, 1
140a7a753: 4889c1 mov rcx, rax
140a7a756: 741c je 0x140a7a774
140a7a758: 48894d60 mov qword ptr [rbp + 0x60], rcx
140a7a75c: 488955e8 mov qword ptr [rbp - 0x18], rdx
140a7a760: 488b01 mov rax, qword ptr [rcx]
140a7a763: f048ff08 lock dec qword ptr [rax]
140a7a767: 75d7 jne 0x140a7a740
140a7a769: 488b4d60 mov rcx, qword ptr [rbp + 0x60]
140a7a76d: e8be43bdff call 0x14064eb30 ; alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<slice2$<u8>,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<slice2$<u8>,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global>
140a7a772: ebcc jmp 0x140a7a740
140a7a774: c6461800 mov byte ptr [rsi + 0x18], 0
140a7a778: 498b4500 mov rax, qword ptr [r13]
140a7a77c: 4885c0 test rax, rax
140a7a77f: 7449 je 0x140a7a7ca
140a7a781: 498b4d10 mov rcx, qword ptr [r13 + 0x10]
140a7a785: 418b551c mov edx, dword ptr [r13 + 0x1c]
140a7a789: 8911 mov dword ptr [rcx], edx
140a7a78b: 488b4810 mov rcx, qword ptr [rax + 0x10]
140a7a78f: 4885c9 test rcx, rcx
140a7a792: 7436 je 0x140a7a7ca
140a7a794: 488b4008 mov rax, qword ptr [rax + 8]
140a7a798: 48c1e103 shl rcx, 3
140a7a79c: 488d0c49 lea rcx, [rcx + rcx*2]
140a7a7a0: 31d2 xor edx, edx
140a7a7a2: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a7a7b0: 48c744100800000000 mov qword ptr [rax + rdx + 8], 0
140a7a7b9: c744101000000000 mov dword ptr [rax + rdx + 0x10], 0
140a7a7c1: 4883c218 add rdx, 0x18
140a7a7c5: 4839d1 cmp rcx, rdx
140a7a7c8: 75e6 jne 0x140a7a7b0
140a7a7ca: 498b1f mov rbx, qword ptr [r15]
140a7a7cd: 833b02 cmp dword ptr [rbx], 2
140a7a7d0: 0f85d4010000 jne 0x140a7a9aa
140a7a7d6: 488b85f0000000 mov rax, qword ptr [rbp + 0xf0]
140a7a7dd: 488b00 mov rax, qword ptr [rax]
140a7a7e0: 488b10 mov rdx, qword ptr [rax]
140a7a7e3: 4885d2 test rdx, rdx
140a7a7e6: 745a je 0x140a7a842
140a7a7e8: 4883c210 add rdx, 0x10
140a7a7ec: 4c8d05fdc60502 lea r8, [rip + 0x205c6fd] ; @142ad6ef0 'snapshot:/Loading'
140a7a7f3: 488d4d20 lea rcx, [rbp + 0x20]
140a7a7f7: 41b911000000 mov r9d, 0x11
140a7a7fd: e83e9cfdff call 0x140a54440 ; country_core::resources::audio_resources::AudioCoreInner::instantiate_audio
140a7a802: 48837d2002 cmp qword ptr [rbp + 0x20], 2
140a7a807: 0f8582000000 jne 0x140a7a88f
140a7a80d: 488b4528 mov rax, qword ptr [rbp + 0x28]
140a7a811: 488945b0 mov qword ptr [rbp - 0x50], rax
140a7a815: 488d05ecc60502 lea rax, [rip + 0x205c6ec]
140a7a81c: 4889442420 mov qword ptr [rsp + 0x20], rax
140a7a821: 488d0d78c10502 lea rcx, [rip + 0x205c178] ; @142ad69a0 'called `Result::unwrap()` on an `Err` value'
140a7a828: 4c8d0d51c10502 lea r9, [rip + 0x205c151]
140a7a82f: 4c8d45b0 lea r8, [rbp - 0x50]
140a7a833: ba2b000000 mov edx, 0x2b
140a7a838: e823efe501 call 0x1428d9760 ; core::result::unwrap_failed
140a7a83d: e9c4010000 jmp 0x140a7aa06
140a7a842: 0fb605c0a3bb02 movzx eax, byte ptr [rip + 0x2bba3c0]
140a7a849: b911000000 mov ecx, 0x11
140a7a84e: ba01000000 mov edx, 1
140a7a853: e8b893b9ff call 0x140613c10 ; 
140a7a858: 4885c0 test rax, rax
140a7a85b: 0f84a7010000 je 0x140a7aa08
140a7a861: 0f100588c60502 movups xmm0, xmmword ptr [rip + 0x205c688] ; @142ad6ef0 'snapshot:/Loading'
140a7a868: 0f1100 movups xmmword ptr [rax], xmm0
140a7a86b: c6401067 mov byte ptr [rax + 0x10], 0x67
140a7a86f: 48c7452000000000 mov qword ptr [rbp + 0x20], 0
140a7a877: 48c7453011000000 mov qword ptr [rbp + 0x30], 0x11
140a7a87f: 48894538 mov qword ptr [rbp + 0x38], rax
140a7a883: 48c7454011000000 mov qword ptr [rbp + 0x40], 0x11
140a7a88b: c6454802 mov byte ptr [rbp + 0x48], 2
140a7a88f: 488b4520 mov rax, qword ptr [rbp + 0x20]
140a7a893: 488b4d28 mov rcx, qword ptr [rbp + 0x28]
140a7a897: 488b5530 mov rdx, qword ptr [rbp + 0x30]
140a7a89b: 4c8b4538 mov r8, qword ptr [rbp + 0x38]
140a7a89f: 4c8b4d40 mov r9, qword ptr [rbp + 0x40]
140a7a8a3: 4c894d10 mov qword ptr [rbp + 0x10], r9
140a7a8a7: 440fb64d48 movzx r9d, byte ptr [rbp + 0x48]
140a7a8ac: 44884d18 mov byte ptr [rbp + 0x18], r9b
140a7a8b0: 448b4d49 mov r9d, dword ptr [rbp + 0x49]
140a7a8b4: 44894d19 mov dword ptr [rbp + 0x19], r9d
140a7a8b8: 440fb74d4d movzx r9d, word ptr [rbp + 0x4d]
140a7a8bd: 6644894d1d mov word ptr [rbp + 0x1d], r9w
140a7a8c2: 440fb64d4f movzx r9d, byte ptr [rbp + 0x4f]
140a7a8c7: 44884d1f mov byte ptr [rbp + 0x1f], r9b
140a7a8cb: 48895500 mov qword ptr [rbp], rdx
140a7a8cf: 4c894508 mov qword ptr [rbp + 8], r8
140a7a8d3: 488945f0 mov qword ptr [rbp - 0x10], rax
140a7a8d7: 48894df8 mov qword ptr [rbp - 8], rcx
140a7a8db: 837df001 cmp dword ptr [rbp - 0x10], 1
140a7a8df: 7525 jne 0x140a7a906
140a7a8e1: 488d55f8 lea rdx, [rbp - 8]
140a7a8e5: c6456f01 mov byte ptr [rbp + 0x6f], 1
140a7a8e9: 488d4db0 lea rcx, [rbp - 0x50]
140a7a8ed: e8dec99b00 call 0x1414372d0 ; libfmod::EventInstance::start
140a7a8f2: 48b80400000000000080 movabs rax, 0x8000000000000004
140a7a8fc: 483945b0 cmp qword ptr [rbp - 0x50], rax
140a7a900: 0f85b8000000 jne 0x140a7a9be
140a7a906: 488b45f0 mov rax, qword ptr [rbp - 0x10]
140a7a90a: 488b4df8 mov rcx, qword ptr [rbp - 8]
140a7a90e: 488b5500 mov rdx, qword ptr [rbp]
140a7a912: 4c8b4508 mov r8, qword ptr [rbp + 8]
140a7a916: 4c8b4d10 mov r9, qword ptr [rbp + 0x10]
140a7a91a: 4c894d40 mov qword ptr [rbp + 0x40], r9
140a7a91e: 448b4d18 mov r9d, dword ptr [rbp + 0x18]
140a7a922: 44894d48 mov dword ptr [rbp + 0x48], r9d
140a7a926: 440fb74d1c movzx r9d, word ptr [rbp + 0x1c]
140a7a92b: 6644894d4c mov word ptr [rbp + 0x4c], r9w
140a7a930: 440fb64d1e movzx r9d, byte ptr [rbp + 0x1e]
140a7a935: 44884d4e mov byte ptr [rbp + 0x4e], r9b
140a7a939: 440fb64d1f movzx r9d, byte ptr [rbp + 0x1f]
140a7a93e: 44884d4f mov byte ptr [rbp + 0x4f], r9b
140a7a942: 48895530 mov qword ptr [rbp + 0x30], rdx
140a7a946: 4c894538 mov qword ptr [rbp + 0x38], r8
140a7a94a: 48894520 mov qword ptr [rbp + 0x20], rax
140a7a94e: 48894d28 mov qword ptr [rbp + 0x28], rcx
140a7a952: 418b771c mov esi, dword ptr [r15 + 0x1c]
140a7a956: 498b7f10 mov rdi, qword ptr [r15 + 0x10]
140a7a95a: 8937 mov dword ptr [rdi], esi
140a7a95c: 833b02 cmp dword ptr [rbx], 2
140a7a95f: 7428 je 0x140a7a989
140a7a961: 4889d9 mov rcx, rbx
140a7a964: 48895d60 mov qword ptr [rbp + 0x60], rbx
140a7a968: e813f9e1ff call 0x14089a280 ; country_core::resources::audio_resources::impl$13::drop
140a7a96d: 488b5d60 mov rbx, qword ptr [rbp + 0x60]
140a7a971: 488b5310 mov rdx, qword ptr [rbx + 0x10]
140a7a975: 4885d2 test rdx, rdx
140a7a978: 740f je 0x140a7a989
140a7a97a: 488b4b18 mov rcx, qword ptr [rbx + 0x18]
140a7a97e: 41b801000000 mov r8d, 1
140a7a984: e89792b9ff call 0x140613c20 ; 
140a7a989: 0f284520 movaps xmm0, xmmword ptr [rbp + 0x20]
140a7a98d: 0f284d30 movaps xmm1, xmmword ptr [rbp + 0x30]
140a7a991: 0f285540 movaps xmm2, xmmword ptr [rbp + 0x40]
140a7a995: 0f115320 movups xmmword ptr [rbx + 0x20], xmm2
140a7a999: 0f114b10 movups xmmword ptr [rbx + 0x10], xmm1
140a7a99d: 0f1103 movups xmmword ptr [rbx], xmm0
140a7a9a0: 8937 mov dword ptr [rdi], esi
140a7a9a2: 48c7433001000000 mov qword ptr [rbx + 0x30], 1
140a7a9aa: 4881c4f8000000 add rsp, 0xf8
140a7a9b1: 5b pop rbx
140a7a9b2: 5f pop rdi
140a7a9b3: 5e pop rsi
140a7a9b4: 415c pop r12
140a7a9b6: 415d pop r13
140a7a9b8: 415e pop r14
140a7a9ba: 415f pop r15
140a7a9bc: 5d pop rbp
140a7a9bd: c3 ret 
140a7a9be: 488b45e0 mov rax, qword ptr [rbp - 0x20]
140a7a9c2: 48894550 mov qword ptr [rbp + 0x50], rax
140a7a9c6: 0f1045b0 movups xmm0, xmmword ptr [rbp - 0x50]
140a7a9ca: 0f104dc0 movups xmm1, xmmword ptr [rbp - 0x40]
140a7a9ce: 0f1055d0 movups xmm2, xmmword ptr [rbp - 0x30]
140a7a9d2: 0f295540 movaps xmmword ptr [rbp + 0x40], xmm2
140a7a9d6: 0f294d30 movaps xmmword ptr [rbp + 0x30], xmm1
140a7a9da: 0f294520 movaps xmmword ptr [rbp + 0x20], xmm0
140a7a9de: 488d05db810502 lea rax, [rip + 0x20581db]
140a7a9e5: 4889442420 mov qword ptr [rsp + 0x20], rax
140a7a9ea: 488d0d0f5c0502 lea rcx, [rip + 0x2055c0f] ; @142ad0600 'called `Result::unwrap()` on an `Err` value'
140a7a9f1: 4c8d0dc85b0502 lea r9, [rip + 0x2055bc8]
140a7a9f8: 4c8d4520 lea r8, [rbp + 0x20]
140a7a9fc: ba2b000000 mov edx, 0x2b
140a7aa01: e85aede501 call 0x1428d9760 ; core::result::unwrap_failed
140a7aa06: 0f0b ud2 
140a7aa08: 4c8d05a15d0502 lea r8, [rip + 0x2055da1]
140a7aa0f: b901000000 mov ecx, 1
140a7aa14: ba11000000 mov edx, 0x11
140a7aa19: e8c5e5e501 call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140a7aa1e: cc int3 
140a7aa1f: 90 nop 
140a7aa20: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a7aa25: 55 push rbp
140a7aa26: 4157 push r15
140a7aa28: 4156 push r14
140a7aa2a: 4155 push r13
140a7aa2c: 4154 push r12
140a7aa2e: 56 push rsi
140a7aa2f: 57 push rdi
140a7aa30: 53 push rbx
140a7aa31: 4883ec28 sub rsp, 0x28
140a7aa35: 488daa80000000 lea rbp, [rdx + 0x80]
140a7aa3c: 488b7de8 mov rdi, qword ptr [rbp - 0x18]
140a7aa40: 4885ff test rdi, rdi
140a7aa43: 488b7560 mov rsi, qword ptr [rbp + 0x60]
140a7aa47: 7425 je 0x140a7aa6e
140a7aa49: 4883c650 add rsi, 0x50
140a7aa4d: 48f7df neg rdi
140a7aa50: eb09 jmp 0x140a7aa5b
140a7aa52: 4883c650 add rsi, 0x50
140a7aa56: 48ffcf dec rdi
140a7aa59: 7413 je 0x140a7aa6e
140a7aa5b: 488b06 mov rax, qword ptr [rsi]
140a7aa5e: f048ff08 lock dec qword ptr [rax]
140a7aa62: 75ee jne 0x140a7aa52
140a7aa64: 4889f1 mov rcx, rsi
140a7aa67: e8c440bdff call 0x14064eb30 ; alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<slice2$<u8>,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<slice2$<u8>,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global>
140a7aa6c: ebe4 jmp 0x140a7aa52
140a7aa6e: 4883c428 add rsp, 0x28
140a7aa72: 5b pop rbx
140a7aa73: 5f pop rdi
140a7aa74: 5e pop rsi
140a7aa75: 415c pop r12
140a7aa77: 415d pop r13
140a7aa79: 415e pop r14
140a7aa7b: 415f pop r15
140a7aa7d: 5d pop rbp
140a7aa7e: c3 ret 
140a7aa7f: 90 nop 
140a7aa80: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a7aa85: 55 push rbp
140a7aa86: 4157 push r15
140a7aa88: 4156 push r14
140a7aa8a: 4155 push r13
140a7aa8c: 4154 push r12
140a7aa8e: 56 push rsi
140a7aa8f: 57 push rdi
140a7aa90: 53 push rbx
140a7aa91: 4883ec28 sub rsp, 0x28
140a7aa95: 488daa80000000 lea rbp, [rdx + 0x80]
140a7aa9c: c6456f01 mov byte ptr [rbp + 0x6f], 1
140a7aaa0: 488d4d20 lea rcx, [rbp + 0x20]
140a7aaa4: e8077cfdff call 0x140a526b0 ; core::ptr::drop_in_place<enum2$<libfmod::Error> >
140a7aaa9: c6456f01 mov byte ptr [rbp + 0x6f], 1
140a7aaad: 4883c428 add rsp, 0x28
140a7aab1: 5b pop rbx
140a7aab2: 5f pop rdi
140a7aab3: 5e pop rsi
140a7aab4: 415c pop r12
140a7aab6: 415d pop r13
140a7aab8: 415e pop r14
140a7aaba: 415f pop r15
140a7aabc: 5d pop rbp
140a7aabd: c3 ret 
140a7aabe: 6690 nop 
140a7aac0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a7aac5: 55 push rbp
140a7aac6: 4157 push r15
140a7aac8: 4156 push r14
140a7aaca: 4155 push r13
140a7aacc: 4154 push r12
140a7aace: 56 push rsi
140a7aacf: 57 push rdi
140a7aad0: 53 push rbx
140a7aad1: 4883ec28 sub rsp, 0x28
140a7aad5: 488daa80000000 lea rbp, [rdx + 0x80]
140a7aadc: 488b4560 mov rax, qword ptr [rbp + 0x60]
140a7aae0: 4889c6 mov rsi, rax
140a7aae3: 488b5010 mov rdx, qword ptr [rax + 0x10]
140a7aae7: 4885d2 test rdx, rdx
140a7aaea: 740f je 0x140a7aafb
140a7aaec: 488b4e18 mov rcx, qword ptr [rsi + 0x18]
140a7aaf0: 41b801000000 mov r8d, 1
140a7aaf6: e82591b9ff call 0x140613c20 ; 
140a7aafb: 0f284520 movaps xmm0, xmmword ptr [rbp + 0x20]
140a7aaff: 0f284d30 movaps xmm1, xmmword ptr [rbp + 0x30]
140a7ab03: 0f285540 movaps xmm2, xmmword ptr [rbp + 0x40]
140a7ab07: 0f115620 movups xmmword ptr [rsi + 0x20], xmm2
140a7ab0b: 0f114e10 movups xmmword ptr [rsi + 0x10], xmm1
140a7ab0f: 0f1106 movups xmmword ptr [rsi], xmm0
140a7ab12: c6456f00 mov byte ptr [rbp + 0x6f], 0
140a7ab16: 4883c428 add rsp, 0x28
140a7ab1a: 5b pop rbx
140a7ab1b: 5f pop rdi
140a7ab1c: 5e pop rsi
140a7ab1d: 415c pop r12
140a7ab1f: 415d pop r13
140a7ab21: 415e pop r14
140a7ab23: 415f pop r15
140a7ab25: 5d pop rbp
140a7ab26: c3 ret 
140a7ab27: 660f1f840000000000 nop word ptr [rax + rax]
140a7ab30: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a7ab35: 55 push rbp
140a7ab36: 4157 push r15
140a7ab38: 4156 push r14
140a7ab3a: 4155 push r13
140a7ab3c: 4154 push r12
140a7ab3e: 56 push rsi
140a7ab3f: 57 push rdi
140a7ab40: 53 push rbx
140a7ab41: 4883ec28 sub rsp, 0x28
140a7ab45: 488daa80000000 lea rbp, [rdx + 0x80]
140a7ab4c: 807d6f00 cmp byte ptr [rbp + 0x6f], 0
140a7ab50: 7511 jne 0x140a7ab63
140a7ab52: 4883c428 add rsp, 0x28
140a7ab56: 5b pop rbx
140a7ab57: 5f pop rdi
140a7ab58: 5e pop rsi
140a7ab59: 415c pop r12
140a7ab5b: 415d pop r13
140a7ab5d: 415e pop r14
140a7ab5f: 415f pop r15
140a7ab61: 5d pop rbp
140a7ab62: c3 ret 
140a7ab63: 488d4df0 lea rcx, [rbp - 0x10]
140a7ab67: e814fb6cff call 0x14014a680 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::resources::critters::sheep::SheepAudio> | core::ptr::drop_in_place<tiny_glade::rollercoaster::CoasterAudioInstance> | core::ptr::drop_in_place<country_core::resources::audio_resources::AudioInstance> | core::ptr::drop_in_place<country_core::resources::critters::ButterflyAudio> | core::ptr::drop_in_place<country_core::resources::critters::sheep::SheepAudio> | core::ptr::drop_in_place<country_core::resources::audio_resources::AudioInstance> | bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::resources::critters::ButterflyAudio> | core::ptr::drop_in_place<country_core::resources::critters::Wren> | core::ptr::drop_in_place<country_core::resources::critters::ButterflyAudio> | bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::resources::critters::Wren>
140a7ab6c: ebe4 jmp 0x140a7ab52
140a7ab6e: 6690 nop 
140a7ab70: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a7ab75: 55 push rbp
140a7ab76: 4157 push r15
140a7ab78: 4156 push r14
140a7ab7a: 4155 push r13
140a7ab7c: 4154 push r12
140a7ab7e: 56 push rsi
140a7ab7f: 57 push rdi
140a7ab80: 53 push rbx
140a7ab81: 4883ec28 sub rsp, 0x28
140a7ab85: 488daa80000000 lea rbp, [rdx + 0x80]
140a7ab8c: 488d4db0 lea rcx, [rbp - 0x50]
140a7ab90: e89b202d00 call 0x140d4cc30 ; anyhow::error::impl$6::drop
140a7ab95: 90 nop 
140a7ab96: 4883c428 add rsp, 0x28
140a7ab9a: 5b pop rbx
140a7ab9b: 5f pop rdi
140a7ab9c: 5e pop rsi
140a7ab9d: 415c pop r12
140a7ab9f: 415d pop r13
140a7aba1: 415e pop r14
140a7aba3: 415f pop r15
140a7aba5: 5d pop rbp
140a7aba6: c3 ret 
