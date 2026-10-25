140963340: 55 push rbp
140963341: 4157 push r15
140963343: 4156 push r14
140963345: 4155 push r13
140963347: 4154 push r12
140963349: 56 push rsi
14096334a: 57 push rdi
14096334b: 53 push rbx
14096334c: 4881ecf8000000 sub rsp, 0xf8
140963353: 488dac2480000000 lea rbp, [rsp + 0x80]
14096335b: 48c74570feffffff mov qword ptr [rbp + 0x70], 0xfffffffffffffffe
140963363: 48894d50 mov qword ptr [rbp + 0x50], rcx
140963367: 48895540 mov qword ptr [rbp + 0x40], rdx
14096336b: 44884548 mov byte ptr [rbp + 0x48], r8b
14096336f: 48c7455800000000 mov qword ptr [rbp + 0x58], 0
140963377: 48c7456008000000 mov qword ptr [rbp + 0x60], 8
14096337f: 48c7456800000000 mov qword ptr [rbp + 0x68], 0
140963387: 4c8d65a8 lea r12, [rbp - 0x58]
14096338b: 4c8d6df8 lea r13, [rbp - 8]
14096338f: 488d7da0 lea rdi, [rbp - 0x60]
140963393: 488d5d40 lea rbx, [rbp + 0x40]
140963397: 48be0100000000000080 movabs rsi, 0x8000000000000001
1409633a1: 4c8d7d58 lea r15, [rbp + 0x58]
1409633a5: eb48 jmp 0x1409633ef
1409633a7: 660f1f840000000000 nop word ptr [rax + rax]
1409633b0: 488b4560 mov rax, qword ptr [rbp + 0x60]
1409633b4: 4b8d0cb6 lea rcx, [r14 + r14*4]
1409633b8: 48c1e104 shl rcx, 4
1409633bc: 0f1045f0 movups xmm0, xmmword ptr [rbp - 0x10]
1409633c0: 0f104d00 movups xmm1, xmmword ptr [rbp]
1409633c4: 0f105510 movups xmm2, xmmword ptr [rbp + 0x10]
1409633c8: 0f105d20 movups xmm3, xmmword ptr [rbp + 0x20]
1409633cc: 0f106530 movups xmm4, xmmword ptr [rbp + 0x30]
1409633d0: 0f11640840 movups xmmword ptr [rax + rcx + 0x40], xmm4
1409633d5: 0f115c0830 movups xmmword ptr [rax + rcx + 0x30], xmm3
1409633da: 0f11540820 movups xmmword ptr [rax + rcx + 0x20], xmm2
1409633df: 0f114c0810 movups xmmword ptr [rax + rcx + 0x10], xmm1
1409633e4: 0f110408 movups xmmword ptr [rax + rcx], xmm0
1409633e8: 49ffc6 inc r14
1409633eb: 4c897568 mov qword ptr [rbp + 0x68], r14
1409633ef: 4889f9 mov rcx, rdi
1409633f2: 4889da mov rdx, rbx
1409633f5: e8168faa00 call 0x14140c310 ; serde_json::de::impl$9::next_element_seed<serde_json::read::StrRead,core::marker::PhantomData<country_core::startup::clearing::PlacementConfigEntry> >
1409633fa: 488b45a0 mov rax, qword ptr [rbp - 0x60]
1409633fe: 4889c1 mov rcx, rax
140963401: 48f7d9 neg rcx
140963404: 0f8083000000 jo 0x14096348d
14096340a: 4839f0 cmp rax, rsi
14096340d: 745a je 0x140963469
14096340f: 488945f0 mov qword ptr [rbp - 0x10], rax
140963413: 498b442440 mov rax, qword ptr [r12 + 0x40]
140963418: 49894540 mov qword ptr [r13 + 0x40], rax
14096341c: 410f100424 movups xmm0, xmmword ptr [r12]
140963421: 410f104c2410 movups xmm1, xmmword ptr [r12 + 0x10]
140963427: 410f10542420 movups xmm2, xmmword ptr [r12 + 0x20]
14096342d: 410f105c2430 movups xmm3, xmmword ptr [r12 + 0x30]
140963433: 410f115d30 movups xmmword ptr [r13 + 0x30], xmm3
140963438: 410f115520 movups xmmword ptr [r13 + 0x20], xmm2
14096343d: 410f114d10 movups xmmword ptr [r13 + 0x10], xmm1
140963442: 410f114500 movups xmmword ptr [r13], xmm0
140963447: 4c8b7568 mov r14, qword ptr [rbp + 0x68]
14096344b: 4c3b7558 cmp r14, qword ptr [rbp + 0x58]
14096344f: 0f855bffffff jne 0x1409633b0
140963455: 4c89f9 mov rcx, r15
140963458: 488d15411f1502 lea rdx, [rip + 0x2151f41]
14096345f: e8dc00c0ff call 0x140563540 ; alloc::raw_vec::RawVec::grow_one<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<system_wall_constructor::construct_elevation_supports::find_overlap_segments::OverhangSupportSegment,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<country_core::resources::time_of_day::TodSequenceEntry,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<asset::texture::desc::TextureAssetDesc,alloc::alloc::Global>
140963464: e947ffffff jmp 0x1409633b0
140963469: 488b45a8 mov rax, qword ptr [rbp - 0x58]
14096346d: 488b7550 mov rsi, qword ptr [rbp + 0x50]
140963471: 48894608 mov qword ptr [rsi + 8], rax
140963475: 48b80000000000000080 movabs rax, 0x8000000000000000
14096347f: 488906 mov qword ptr [rsi], rax
140963482: 488d4d58 lea rcx, [rbp + 0x58]
140963486: e80558f3ff call 0x140898c90 ; core::ptr::drop_in_place<alloc::vec::Vec<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global> > | core::ptr::drop_in_place<alloc::vec::Vec<sentry_types::protocol::attachment::Attachment,alloc::alloc::Global> >
14096348b: eb13 jmp 0x1409634a0
14096348d: 488b4568 mov rax, qword ptr [rbp + 0x68]
140963491: 488b7550 mov rsi, qword ptr [rbp + 0x50]
140963495: 48894610 mov qword ptr [rsi + 0x10], rax
140963499: 0f104558 movups xmm0, xmmword ptr [rbp + 0x58]
14096349d: 0f1106 movups xmmword ptr [rsi], xmm0
1409634a0: 4889f0 mov rax, rsi
1409634a3: 4881c4f8000000 add rsp, 0xf8
1409634aa: 5b pop rbx
1409634ab: 5f pop rdi
1409634ac: 5e pop rsi
1409634ad: 415c pop r12
1409634af: 415d pop r13
1409634b1: 415e pop r14
1409634b3: 415f pop r15
1409634b5: 5d pop rbp
1409634b6: c3 ret 
1409634b7: 660f1f840000000000 nop word ptr [rax + rax]
1409634c0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1409634c5: 55 push rbp
1409634c6: 4157 push r15
1409634c8: 4156 push r14
1409634ca: 4155 push r13
1409634cc: 4154 push r12
1409634ce: 56 push rsi
1409634cf: 57 push rdi
1409634d0: 53 push rbx
1409634d1: 4883ec28 sub rsp, 0x28
1409634d5: 488daa80000000 lea rbp, [rdx + 0x80]
1409634dc: 488d4d58 lea rcx, [rbp + 0x58]
1409634e0: e8ab57f3ff call 0x140898c90 ; core::ptr::drop_in_place<alloc::vec::Vec<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global> > | core::ptr::drop_in_place<alloc::vec::Vec<sentry_types::protocol::attachment::Attachment,alloc::alloc::Global> >
1409634e5: 90 nop 
1409634e6: 4883c428 add rsp, 0x28
1409634ea: 5b pop rbx
1409634eb: 5f pop rdi
1409634ec: 5e pop rsi
1409634ed: 415c pop r12
1409634ef: 415d pop r13
1409634f1: 415e pop r14
1409634f3: 415f pop r15
1409634f5: 5d pop rbp
1409634f6: c3 ret 
1409634f7: 660f1f840000000000 nop word ptr [rax + rax]
140963500: 4889542410 mov qword ptr [rsp + 0x10], rdx
140963505: 55 push rbp
140963506: 4157 push r15
140963508: 4156 push r14
14096350a: 4155 push r13
14096350c: 4154 push r12
14096350e: 56 push rsi
14096350f: 57 push rdi
140963510: 53 push rbx
140963511: 4883ec28 sub rsp, 0x28
140963515: 488daa80000000 lea rbp, [rdx + 0x80]
14096351c: 488d4df0 lea rcx, [rbp - 0x10]
140963520: e84b00d0ff call 0x140663570 ; core::ptr::drop_in_place<sentry_types::protocol::attachment::Attachment> | core::ptr::drop_in_place<country_core::startup::clearing::PlacementConfigEntry>
140963525: 90 nop 
140963526: 4883c428 add rsp, 0x28
14096352a: 5b pop rbx
14096352b: 5f pop rdi
14096352c: 5e pop rsi
14096352d: 415c pop r12
14096352f: 415d pop r13
140963531: 415e pop r14
140963533: 415f pop r15
140963535: 5d pop rbp
140963536: c3 ret 
