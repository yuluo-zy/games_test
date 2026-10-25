1415bd310: push rbp
1415bd311: push r15
1415bd313: push r14
1415bd315: push r13
1415bd317: push r12
1415bd319: push rsi
1415bd31a: push rdi
1415bd31b: push rbx
1415bd31c: sub rsp, 0x238
1415bd323: lea rbp, [rsp + 0x80]
1415bd32b: mov qword ptr [rbp + 0x1b0], 0xfffffffffffffffe
1415bd336: mov qword ptr [rbp + 0x138], rcx
1415bd33d: mov rax, qword ptr [rdx + 0x10]
1415bd341: mov qword ptr [rbp + 0x98], rax
1415bd348: lea rbx, [rbp + 0xa0]
1415bd34f: mov rcx, rbx
1415bd352: call 0x140b880d0 ; serde_json::value::de::MapDeserializer::new
1415bd357: movabs rax, 0x8000000000000000
1415bd361: mov qword ptr [rbp + 8], rax
1415bd365: mov qword ptr [rbp + 0x190], rax
1415bd36c: mov qword ptr [rbp + 0x168], rax
1415bd373: lea r14, [rbp + 0x141]
1415bd37a: lea r15, [rbp + 0x108]
1415bd381: lea rsi, [rip + 0x16be14c]
1415bd388: lea r13, [rip + 0x16be131]
1415bd38f: lea rdi, [rip + 0x16be10e]
1415bd396: mov dword ptr [rbp + 0x18c], 0
1415bd3a0: mov dword ptr [rbp + 0x188], 0
1415bd3aa: mov qword ptr [rbp + 0x198], r12
1415bd3b1: jmp 0x1415bd3ea
1415bd3b3: nop word ptr cs:[rax + rax]
1415bd3c0: mov rax, qword ptr [rbp + 0x1a0]
1415bd3c7: mov qword ptr [rbp + 0x168], rax
1415bd3ce: mov rax, qword ptr [rbp + 0x1a8]
1415bd3d5: mov qword ptr [rbp + 0x170], rax
1415bd3dc: mov rax, qword ptr [rbp + 0x180]
1415bd3e3: mov qword ptr [rbp + 0x178], rax
1415bd3ea: mov rcx, r15
1415bd3ed: mov rdx, rbx
1415bd3f0: call 0x14143c450 ; alloc::collections::btree::map::IntoIter::dying_next<alloc::string::String,enum2$<serde_json::value::Value>,alloc::alloc::Global>
1415bd3f5: mov rax, qword ptr [rbp + 0x108]
1415bd3fc: test rax, rax
1415bd3ff: je 0x1415bd924
1415bd405: mov rcx, qword ptr [rbp + 0x118]
1415bd40c: lea rdx, [rcx + rcx*2]
1415bd410: mov r8, qword ptr [rax + rdx*8 + 0x168]
1415bd418: mov r9, qword ptr [rax + rdx*8 + 0x170]
1415bd420: mov qword ptr [rbp + 0x1a0], r9
1415bd427: mov r12, qword ptr [rax + rdx*8 + 0x178]
1415bd42f: shl rcx, 5
1415bd433: movups xmm0, xmmword ptr [rax + rcx]
1415bd437: movups xmm1, xmmword ptr [rax + rcx + 0x10]
1415bd43c: movaps xmmword ptr [rbp + 0x140], xmm0
1415bd443: movaps xmmword ptr [rbp + 0x150], xmm1
1415bd44a: mov rax, r8
1415bd44d: neg rax
1415bd450: jo 0x1415bd924
1415bd456: mov qword ptr [rbp + 0x1a8], r8
1415bd45d: movzx eax, byte ptr [rbp + 0xe8]
1415bd464: cmp rax, 6
1415bd468: ja 0x1415bd61e
1415bd46e: movsxd rax, dword ptr [rdi + rax*4]
1415bd472: add rax, rdi
1415bd475: jmp rax
1415bd477: movaps xmm0, xmmword ptr [rbp + 0x140]
1415bd47e: movaps xmm1, xmmword ptr [rbp + 0x150]
1415bd485: lea rax, [rbp + 0xe8]
1415bd48c: movups xmmword ptr [rax + 0x10], xmm1
1415bd490: movups xmmword ptr [rax], xmm0
1415bd493: add r12, -9
1415bd497: cmp r12, 4
1415bd49b: ja 0x1415bd580
1415bd4a1: movsxd rax, dword ptr [r13 + r12*4]
1415bd4a6: add rax, r13
1415bd4a9: jmp rax
1415bd4ab: mov rcx, qword ptr [rbp + 0x1a0]
1415bd4b2: mov rax, qword ptr [rcx]
1415bd4b5: xor rax, qword ptr [rip + 0x1530b85]
1415bd4bc: movzx ecx, byte ptr [rcx + 8]
1415bd4c0: xor cl, byte ptr [rip + 0x1530b83]
1415bd4c6: movzx ecx, cl
1415bd4c9: or rcx, rax
1415bd4cc: jne 0x1415bd580
1415bd4d2: mov r12b, 3
1415bd4d5: mov rdx, qword ptr [rbp + 0x1a8]
1415bd4dc: test rdx, rdx
1415bd4df: jne 0x1415bd593
1415bd4e5: jmp 0x1415bd650
1415bd4ea: mov rcx, qword ptr [rbp + 0x1a0]
1415bd4f1: mov rax, qword ptr [rcx]
1415bd4f4: xor rax, qword ptr [rip + 0x1530b24]
1415bd4fb: mov rcx, qword ptr [rcx + 3]
1415bd4ff: xor rcx, qword ptr [rip + 0x1530b1c]
1415bd506: or rcx, rax
1415bd509: jne 0x1415bd580
1415bd50b: xor r12d, r12d
1415bd50e: mov rdx, qword ptr [rbp + 0x1a8]
1415bd515: test rdx, rdx
1415bd518: jne 0x1415bd593
1415bd51a: jmp 0x1415bd650
1415bd51f: mov rcx, qword ptr [rbp + 0x1a0]
1415bd526: mov rax, qword ptr [rcx]
1415bd529: xor rax, qword ptr [rip + 0x1530b07]
1415bd530: movzx ecx, word ptr [rcx + 8]
1415bd534: xor cx, word ptr [rip + 0x1530b04]
1415bd53b: movzx ecx, cx
1415bd53e: or rcx, rax
1415bd541: jne 0x1415bd580
1415bd543: mov r12b, 2
1415bd546: mov rdx, qword ptr [rbp + 0x1a8]
1415bd54d: test rdx, rdx
1415bd550: jne 0x1415bd593
1415bd552: jmp 0x1415bd650
1415bd557: mov rcx, qword ptr [rbp + 0x1a0]
1415bd55e: mov rax, qword ptr [rcx]
1415bd561: xor rax, qword ptr [rip + 0x1530ac2]
1415bd568: mov rcx, qword ptr [rcx + 5]
1415bd56c: xor rcx, qword ptr [rip + 0x1530abc]
1415bd573: or rcx, rax
1415bd576: je 0x1415bd62f
1415bd57c: nop dword ptr [rax]
1415bd580: mov r12b, 4
1415bd583: mov rdx, qword ptr [rbp + 0x1a8]
1415bd58a: test rdx, rdx
1415bd58d: je 0x1415bd650
1415bd593: mov r8d, 1
1415bd599: mov rcx, qword ptr [rbp + 0x1a0]
1415bd5a0: call 0x140613c20 ; 0x140613c20
1415bd5a5: movzx eax, r12b
1415bd5a9: movsxd rax, dword ptr [rsi + rax*4]
1415bd5ad: add rax, rsi
1415bd5b0: jmp rax
1415bd5b2: mov rdx, qword ptr [rbp + 0xf0]
1415bd5b9: test rdx, rdx
1415bd5bc: je 0x1415bd477
1415bd5c2: mov rcx, qword ptr [rbp + 0xf8]
1415bd5c9: mov r8d, 1
1415bd5cf: call 0x140613c20 ; 0x140613c20
1415bd5d4: jmp 0x1415bd477
1415bd5d9: mov rcx, qword ptr [rbp + 0xf8]
1415bd5e0: mov rdx, qword ptr [rbp + 0x100]
1415bd5e7: mov qword ptr [rbp + 0x180], rcx
1415bd5ee: call 0x140888d70 ; core::ptr::drop_in_place<slice2$<enum2$<serde_json::value::Value> > >
1415bd5f3: mov rdx, qword ptr [rbp + 0xf0]
1415bd5fa: test rdx, rdx
1415bd5fd: je 0x1415bd477
1415bd603: shl rdx, 5
1415bd607: mov r8d, 8
1415bd60d: mov rcx, qword ptr [rbp + 0x180]
1415bd614: call 0x140613c20 ; 0x140613c20
1415bd619: jmp 0x1415bd477
1415bd61e: lea rcx, [rbp + 0xf0]
1415bd625: call 0x140898f40 ; core::ptr::drop_in_place<serde_json::map::Map<alloc::string::String,enum2$<serde_json::value::Value> > >
1415bd62a: jmp 0x1415bd477
1415bd62f: mov r12b, 1
1415bd632: mov rdx, qword ptr [rbp + 0x1a8]
1415bd639: test rdx, rdx
1415bd63c: jne 0x1415bd593
1415bd642: nop word ptr cs:[rax + rax]
1415bd650: movzx eax, r12b
1415bd654: movsxd rax, dword ptr [rsi + rax*4]
1415bd658: add rax, rsi
1415bd65b: jmp rax
1415bd65d: xor eax, eax
1415bd65f: cmp rax, qword ptr [rbp + 0x168]
1415bd666: mov r12, qword ptr [rbp + 0x198]
1415bd66d: jno 0x1415bdbfa
1415bd673: movzx eax, byte ptr [rbp + 0xe8]
1415bd67a: mov byte ptr [rbp + 0xe8], 6
1415bd681: cmp al, 6
1415bd683: je 0x1415bdaa6
1415bd689: mov byte ptr [rbp + 0x108], al
1415bd68f: movups xmm0, xmmword ptr [rbx + 0x49]
1415bd693: movups xmm1, xmmword ptr [rbx + 0x58]
1415bd697: lea rax, [rbp + 0x109]
1415bd69e: movups xmmword ptr [rax + 0xf], xmm1
1415bd6a2: movups xmmword ptr [rax], xmm0
1415bd6a5: lea rcx, [rbp - 0x28]
1415bd6a9: mov rdx, r15
1415bd6ac: call 0x14159caf0 ; serde_json::value::de::impl$2::deserialize_seq<serde::de::impls::impl$19::deserialize::VecVisitor<country_core::resources::history::ResolvedHistoryChainSnapshot> >
1415bd6b1: mov rax, qword ptr [rbp - 0x28]
1415bd6b5: mov rdx, qword ptr [rbp - 0x20]
1415bd6b9: mov rcx, rax
1415bd6bc: neg rcx
1415bd6bf: jo 0x1415bdb90
1415bd6c5: mov rcx, qword ptr [rbp - 0x18]
1415bd6c9: shl rcx, 4
1415bd6cd: add rcx, rdx
1415bd6d0: mov qword ptr [rbp + 0x140], rdx
1415bd6d7: mov qword ptr [rbp + 0x148], rdx
1415bd6de: mov qword ptr [rbp + 0x150], rax
1415bd6e5: mov qword ptr [rbp + 0x158], rcx
1415bd6ec: lea rcx, [rbp - 0x10]
1415bd6f0: lea rdx, [rbp + 0x140]
1415bd6f7: lea r8, [rip + 0x167b55a]
1415bd6fe: call 0x141737510 ; alloc::vec::in_place_collect::from_iter_in_place<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<country_core::resources::history::ResolvedHistoryChainSnapshot,alloc::alloc::Global>,country_core::resources::history::HistoryChainSnapshot (*)(country_core::resources::history::ResolvedHistoryChainSnapshot)>,country_core::resources::history::HistoryChainSnapshot>
1415bd703: mov rcx, qword ptr [rbp - 0x10]
1415bd707: mov rax, qword ptr [rbp - 8]
1415bd70b: mov qword ptr [rbp + 0x1a8], rax
1415bd712: mov rax, rcx
1415bd715: neg rax
1415bd718: jo 0x1415bdabe
1415bd71e: mov qword ptr [rbp + 0x1a0], rcx
1415bd725: mov rax, qword ptr [rbp]
1415bd729: mov qword ptr [rbp + 0x180], rax
1415bd730: xor eax, eax
1415bd732: cmp rax, qword ptr [rbp + 0x168]
1415bd739: jo 0x1415bd3c0
1415bd73f: lea rcx, [rbp + 0x168]
1415bd746: call 0x140600290 ; core::ptr::drop_in_place<country_core::resources::history::HistoryChainSnapshots>
1415bd74b: jmp 0x1415bd3c0
1415bd750: movzx eax, byte ptr [rbp + 0xe8]
1415bd757: mov byte ptr [rbp + 0xe8], 6
1415bd75e: cmp al, 6
1415bd760: mov r12, qword ptr [rbp + 0x198]
1415bd767: je 0x1415bda93
1415bd76d: mov byte ptr [rbp + 0x140], al
1415bd773: movups xmm0, xmmword ptr [rbx + 0x49]
1415bd777: movups xmm1, xmmword ptr [rbx + 0x58]
1415bd77b: movups xmmword ptr [r14 + 0xf], xmm1
1415bd780: movups xmmword ptr [r14], xmm0
1415bd784: lea rcx, [rbp + 0x140]
1415bd78b: call 0x1415a3940 ; serde_json::value::de::impl$2::deserialize_ignored_any<serde::de::ignored_any::IgnoredAny>
1415bd790: test rax, rax
1415bd793: je 0x1415bd3ea
1415bd799: jmp 0x1415bdab7
1415bd79e: cmp dword ptr [rbp + 0x188], 0
1415bd7a5: mov r12, qword ptr [rbp + 0x198]
1415bd7ac: jne 0x1415bdc72
1415bd7b2: movzx eax, byte ptr [rbp + 0xe8]
1415bd7b9: mov byte ptr [rbp + 0xe8], 6
1415bd7c0: cmp al, 6
1415bd7c2: je 0x1415bdbac
1415bd7c8: mov byte ptr [rbp + 0x140], al
1415bd7ce: movups xmm0, xmmword ptr [rbx + 0x49]
1415bd7d2: movups xmm1, xmmword ptr [rbx + 0x58]
1415bd7d6: movups xmmword ptr [r14 + 0xf], xmm1
1415bd7db: movups xmmword ptr [r14], xmm0
1415bd7df: mov rcx, r15
1415bd7e2: lea rdx, [rbp + 0x140]
1415bd7e9: call 0x14159a8f0 ; serde_json::value::de::impl$2::deserialize_i32<serde::de::impls::impl$54::deserialize::PrimitiveVisitor>
1415bd7ee: test byte ptr [rbp + 0x108], 1
1415bd7f5: jne 0x1415bdba0
1415bd7fb: mov eax, dword ptr [rbp + 0x10c]
1415bd801: mov dword ptr [rbp + 0x128], eax
1415bd807: mov dword ptr [rbp + 0x188], 1
1415bd811: jmp 0x1415bd3ea
1415bd816: cmp dword ptr [rbp + 0x18c], 0
1415bd81d: mov r12, qword ptr [rbp + 0x198]
1415bd824: jne 0x1415bdc46
1415bd82a: movzx eax, byte ptr [rbp + 0xe8]
1415bd831: mov byte ptr [rbp + 0xe8], 6
1415bd838: cmp al, 6
1415bd83a: je 0x1415bdbc2
1415bd840: mov byte ptr [rbp + 0x140], al
1415bd846: movups xmm0, xmmword ptr [rbx + 0x49]
1415bd84a: movups xmm1, xmmword ptr [rbx + 0x58]
1415bd84e: movups xmmword ptr [r14 + 0xf], xmm1
1415bd853: movups xmmword ptr [r14], xmm0
1415bd857: mov rcx, r15
1415bd85a: lea rdx, [rbp + 0x140]
1415bd861: call 0x14159a8f0 ; serde_json::value::de::impl$2::deserialize_i32<serde::de::impls::impl$54::deserialize::PrimitiveVisitor>
1415bd866: test byte ptr [rbp + 0x108], 1
1415bd86d: jne 0x1415bdba0
1415bd873: mov eax, dword ptr [rbp + 0x10c]
1415bd879: mov dword ptr [rbp + 0x12c], eax
1415bd87f: mov dword ptr [rbp + 0x18c], 1
1415bd889: jmp 0x1415bd3ea
1415bd88e: mov rax, qword ptr [rbp + 0x190]
1415bd895: neg rax
1415bd898: mov r12, qword ptr [rbp + 0x198]
1415bd89f: jno 0x1415bdc5c
1415bd8a5: movzx eax, byte ptr [rbp + 0xe8]
1415bd8ac: mov byte ptr [rbp + 0xe8], 6
1415bd8b3: cmp al, 6
1415bd8b5: je 0x1415bdbd8
1415bd8bb: mov byte ptr [rbp + 0x140], al
1415bd8c1: movups xmm0, xmmword ptr [rbx + 0x49]
1415bd8c5: movups xmm1, xmmword ptr [rbx + 0x58]
1415bd8c9: movups xmmword ptr [r14 + 0xf], xmm1
1415bd8ce: movups xmmword ptr [r14], xmm0
1415bd8d2: mov rcx, r15
1415bd8d5: lea rdx, [rbp + 0x140]
1415bd8dc: call 0x14159c190 ; serde_json::value::de::impl$2::deserialize_seq<serde::de::impls::impl$19::deserialize::VecVisitor<country_core::resources::history::MultiEdit> >
1415bd8e1: mov rax, qword ptr [rbp + 0x108]
1415bd8e8: mov rdx, qword ptr [rbp + 0x110]
1415bd8ef: mov rcx, rax
1415bd8f2: neg rcx
1415bd8f5: jo 0x1415bdbee
1415bd8fb: mov rcx, qword ptr [rbp + 0x118]
1415bd902: mov qword ptr [rbp + 8], rax
1415bd906: mov qword ptr [rbp + 0x10], rdx
1415bd90a: mov qword ptr [rbp + 0x130], rcx
1415bd911: mov qword ptr [rbp + 0x18], rcx
1415bd915: mov r12, rdx
1415bd918: mov qword ptr [rbp + 0x190], rax
1415bd91f: jmp 0x1415bd3aa
1415bd924: test byte ptr [rbp + 0x188], 1
1415bd92b: je 0x1415bd959
1415bd92d: mov r9, qword ptr [rbp + 0x190]
1415bd934: mov rax, r9
1415bd937: neg rax
1415bd93a: mov r12, qword ptr [rbp + 0x198]
1415bd941: jno 0x1415bd976
1415bd943: lea rcx, [rip + 0x16bd116]
1415bd94a: mov edx, 0xa
1415bd94f: call 0x1428f9bc0 ; serde::de::Error::missing_field<serde_json::error::Error>
1415bd954: jmp 0x1415bdab7
1415bd959: lea rcx, [rip + 0x16bd0e8]
1415bd960: mov edx, 0xb
1415bd965: call 0x1428f9bc0 ; serde::de::Error::missing_field<serde_json::error::Error>
1415bd96a: mov r12, qword ptr [rbp + 0x198]
1415bd971: jmp 0x1415bdab7
1415bd976: test byte ptr [rbp + 0x18c], 1
1415bd97d: mov eax, 0xffffffff
1415bd982: cmovne eax, dword ptr [rbp + 0x12c]
1415bd989: mov rcx, qword ptr [rbp + 0x168]
1415bd990: xor edx, edx
1415bd992: mov r8, rcx
1415bd995: neg r8
1415bd998: cmovo rcx, rdx
1415bd99c: mov r8d, 8
1415bd9a2: cmovno r8, qword ptr [rbp + 0x170]
1415bd9aa: cmovno rdx, qword ptr [rbp + 0x178]
1415bd9b2: xorps xmm0, xmm0
1415bd9b5: movups xmmword ptr [rbp - 0x38], xmm0
1415bd9b9: mov qword ptr [rbp + 0x20], r9
1415bd9bd: mov qword ptr [rbp + 0x28], r12
1415bd9c1: mov r9, qword ptr [rbp + 0x130]
1415bd9c8: mov qword ptr [rbp + 0x30], r9
1415bd9cc: mov qword ptr [rbp + 0x38], rcx
1415bd9d0: mov qword ptr [rbp + 0x40], r8
1415bd9d4: mov qword ptr [rbp + 0x48], rdx
1415bd9d8: movabs rsi, 0x8000000000000000
1415bd9e2: mov qword ptr [rbp + 0x50], rsi
1415bd9e6: movups xmm1, xmmword ptr [rbp - 0x58]
1415bd9ea: movups xmm2, xmmword ptr [rbp - 0x48]
1415bd9ee: movups xmmword ptr [rbp + 0x58], xmm1
1415bd9f2: movups xmmword ptr [rbp + 0x68], xmm2
1415bd9f6: movups xmmword ptr [rbp + 0x78], xmm0
1415bd9fa: mov ecx, dword ptr [rbp + 0x128]
1415bda00: mov dword ptr [rbp + 0x88], ecx
1415bda06: mov dword ptr [rbp + 0x8c], eax
1415bda0c: mov dword ptr [rbp + 0x90], 0
1415bda16: cmp qword ptr [rbp + 0xe0], 0
1415bda1e: jne 0x1415bdc10
1415bda24: mov rax, qword ptr [rbp + 0x90]
1415bda2b: mov rcx, qword ptr [rbp + 0x138]
1415bda32: mov qword ptr [rcx + 0x70], rax
1415bda36: mov rax, qword ptr [rbp + 0x80]
1415bda3d: mov qword ptr [rcx + 0x60], rax
1415bda41: mov eax, dword ptr [rbp + 0x88]
1415bda47: mov dword ptr [rcx + 0x68], eax
1415bda4a: mov eax, dword ptr [rbp + 0x8c]
1415bda50: mov dword ptr [rcx + 0x6c], eax
1415bda53: movups xmm0, xmmword ptr [rbp + 0x70]
1415bda57: movups xmmword ptr [rcx + 0x50], xmm0
1415bda5b: movups xmm0, xmmword ptr [rbp + 0x60]
1415bda5f: movups xmmword ptr [rcx + 0x40], xmm0
1415bda63: movups xmm0, xmmword ptr [rbp + 0x20]
1415bda67: movups xmm1, xmmword ptr [rbp + 0x30]
1415bda6b: movups xmm2, xmmword ptr [rbp + 0x40]
1415bda6f: movups xmm3, xmmword ptr [rbp + 0x50]
1415bda73: movups xmmword ptr [rcx + 0x30], xmm3
1415bda77: movups xmmword ptr [rcx + 0x20], xmm2
1415bda7b: movups xmmword ptr [rcx + 0x10], xmm1
1415bda7f: movups xmmword ptr [rcx], xmm0
1415bda82: lea rcx, [rbp + 0xa0]
1415bda89: call 0x140887df0 ; core::ptr::drop_in_place<serde_json::map::IntoIter>
1415bda8e: jmp 0x1415bdb69
1415bda93: lea rcx, [rip + 0x16bc7ce]
1415bda9a: mov edx, 0x10
1415bda9f: call 0x1428fa000 ; serde_json::error::impl$8::custom<ref$<str$> >
1415bdaa4: jmp 0x1415bdab7
1415bdaa6: lea rcx, [rip + 0x16bc7bb]
1415bdaad: mov edx, 0x10
1415bdab2: call 0x1428fa000 ; serde_json::error::impl$8::custom<ref$<str$> >
1415bdab7: mov qword ptr [rbp + 0x1a8], rax
1415bdabe: xor eax, eax
1415bdac0: cmp rax, qword ptr [rbp + 0x168]
1415bdac7: jo 0x1415bdad5
1415bdac9: lea rcx, [rbp + 0x168]
1415bdad0: call 0x140600290 ; core::ptr::drop_in_place<country_core::resources::history::HistoryChainSnapshots>
1415bdad5: mov rax, qword ptr [rbp + 0x190]
1415bdadc: neg rax
1415bdadf: jo 0x1415bdb3e
1415bdae1: mov qword ptr [rbp + 0x1a0], 0
1415bdaec: mov rcx, r12
1415bdaef: nop 
1415bdaf0: mov rax, qword ptr [rbp + 0x1a0]
1415bdaf7: cmp qword ptr [rbp + 0x130], rax
1415bdafe: je 0x1415bdb18
1415bdb00: inc rax
1415bdb03: mov qword ptr [rbp + 0x1a0], rax
1415bdb0a: lea rsi, [rcx + 0x28]
1415bdb0e: call 0x14088bae0 ; core::ptr::drop_in_place<country_core::resources::history::MultiEdit>
1415bdb13: mov rcx, rsi
1415bdb16: jmp 0x1415bdaf0
1415bdb18: mov rax, qword ptr [rbp + 0x190]
1415bdb1f: test rax, rax
1415bdb22: mov rcx, qword ptr [rbp + 0x198]
1415bdb29: je 0x1415bdb3e
1415bdb2b: shl rax, 3
1415bdb2f: lea rdx, [rax + rax*4]
1415bdb33: mov r8d, 8
1415bdb39: call 0x140613c20 ; 0x140613c20
1415bdb3e: mov rcx, qword ptr [rbp + 0x138]
1415bdb45: mov rax, qword ptr [rbp + 0x1a8]
1415bdb4c: mov qword ptr [rcx + 8], rax
1415bdb50: movabs rax, 0x8000000000000000
1415bdb5a: mov qword ptr [rcx], rax
1415bdb5d: lea rcx, [rbp + 0xa0]
1415bdb64: call 0x140887df0 ; core::ptr::drop_in_place<serde_json::map::IntoIter>
1415bdb69: lea rcx, [rbp + 0xe8]
1415bdb70: call 0x14088e430 ; core::ptr::drop_in_place<enum2$<core::option::Option<enum2$<serde_json::value::Value> > > >
1415bdb75: mov rax, qword ptr [rbp + 0x138]
1415bdb7c: add rsp, 0x238
1415bdb83: pop rbx
1415bdb84: pop rdi
1415bdb85: pop rsi
1415bdb86: pop r12
1415bdb88: pop r13
1415bdb8a: pop r14
1415bdb8c: pop r15
1415bdb8e: pop rbp
1415bdb8f: ret 
1415bdb90: mov qword ptr [rbp + 0x1a8], rdx
1415bdb97: mov qword ptr [rbp - 8], rdx
1415bdb9b: jmp 0x1415bdabe
1415bdba0: mov rax, qword ptr [rbp + 0x110]
1415bdba7: jmp 0x1415bdab7
1415bdbac: lea rcx, [rip + 0x16bc6b5]
1415bdbb3: mov edx, 0x10
1415bdbb8: call 0x1428fa000 ; serde_json::error::impl$8::custom<ref$<str$> >
1415bdbbd: jmp 0x1415bdab7
1415bdbc2: lea rcx, [rip + 0x16bc69f]
1415bdbc9: mov edx, 0x10
1415bdbce: call 0x1428fa000 ; serde_json::error::impl$8::custom<ref$<str$> >
1415bdbd3: jmp 0x1415bdab7
1415bdbd8: lea rcx, [rip + 0x16bc689]
1415bdbdf: mov edx, 0x10
1415bdbe4: call 0x1428fa000 ; serde_json::error::impl$8::custom<ref$<str$> >
1415bdbe9: jmp 0x1415bdab7
1415bdbee: mov qword ptr [rbp + 0x1a8], rdx
1415bdbf5: jmp 0x1415bdabe
1415bdbfa: lea rcx, [rip + 0x16bce69]
1415bdc01: mov edx, 9
1415bdc06: call 0x1428f9db0 ; serde::de::Error::duplicate_field<serde_json::error::Error>
1415bdc0b: jmp 0x1415bdab7
1415bdc10: lea rdx, [rip + 0x16bc481]
1415bdc17: lea r8, [rip + 0x16bc442]
1415bdc1e: mov rcx, qword ptr [rbp + 0x98]
1415bdc25: call 0x1428f9d20 ; serde::de::Error::invalid_length<serde_json::error::Error>
1415bdc2a: mov rcx, qword ptr [rbp + 0x138]
1415bdc31: mov qword ptr [rcx + 8], rax
1415bdc35: mov qword ptr [rcx], rsi
1415bdc38: lea rcx, [rbp + 0x20]
1415bdc3c: call 0x14088acf0 ; core::ptr::drop_in_place<country_core::resources::history::History>
1415bdc41: jmp 0x1415bda82
1415bdc46: lea rcx, [rip + 0x16bce06]
1415bdc4d: mov edx, 0xd
1415bdc52: call 0x1428f9db0 ; serde::de::Error::duplicate_field<serde_json::error::Error>
1415bdc57: jmp 0x1415bdab7
1415bdc5c: lea rcx, [rip + 0x16bcdfd]
1415bdc63: mov edx, 0xa
1415bdc68: call 0x1428f9db0 ; serde::de::Error::duplicate_field<serde_json::error::Error>
1415bdc6d: jmp 0x1415bdab7
1415bdc72: lea rcx, [rip + 0x16bcdcf]
1415bdc79: mov edx, 0xb
1415bdc7e: call 0x1428f9db0 ; serde::de::Error::duplicate_field<serde_json::error::Error>
1415bdc83: jmp 0x1415bdab7
1415bdc88: nop dword ptr [rax + rax]
1415bdc90: mov qword ptr [rsp + 0x10], rdx
1415bdc95: push rbp
1415bdc96: push r15
1415bdc98: push r14
1415bdc9a: push r13
1415bdc9c: push r12
1415bdc9e: push rsi
1415bdc9f: push rdi
1415bdca0: push rbx
1415bdca1: sub rsp, 0x28
1415bdca5: lea rbp, [rdx + 0x80]
1415bdcac: mov rdx, qword ptr [rbp + 0xf0]
1415bdcb3: test rdx, rdx
1415bdcb6: je 0x1415bdcce
1415bdcb8: shl rdx, 5
1415bdcbc: mov r8d, 8
1415bdcc2: mov rcx, qword ptr [rbp + 0x180]
1415bdcc9: call 0x140613c20 ; 0x140613c20
1415bdcce: nop 
1415bdccf: add rsp, 0x28
1415bdcd3: pop rbx
1415bdcd4: pop rdi
1415bdcd5: pop rsi
1415bdcd6: pop r12
1415bdcd8: pop r13
1415bdcda: pop r14
1415bdcdc: pop r15
1415bdcde: pop rbp
1415bdcdf: ret 
1415bdce0: mov qword ptr [rsp + 0x10], rdx
1415bdce5: push rbp
1415bdce6: push r15
1415bdce8: push r14
1415bdcea: push r13
1415bdcec: push r12
1415bdcee: push rsi
1415bdcef: push rdi
1415bdcf0: push rbx
1415bdcf1: sub rsp, 0x28
1415bdcf5: lea rbp, [rdx + 0x80]
1415bdcfc: movaps xmm0, xmmword ptr [rbp + 0x140]
1415bdd03: movaps xmm1, xmmword ptr [rbp + 0x150]
1415bdd0a: lea rax, [rbp + 0xe8]
1415bdd11: movups xmmword ptr [rax + 0x10], xmm1
1415bdd15: movups xmmword ptr [rax], xmm0
1415bdd18: cmp qword ptr [rbp + 0x1a8], 0
1415bdd20: je 0x1415bdd3b
1415bdd22: mov r8d, 1
1415bdd28: mov rcx, qword ptr [rbp + 0x1a0]
1415bdd2f: mov rdx, qword ptr [rbp + 0x1a8]
1415bdd36: call 0x140613c20 ; 0x140613c20
1415bdd3b: nop 
1415bdd3c: add rsp, 0x28
1415bdd40: pop rbx
1415bdd41: pop rdi
1415bdd42: pop rsi
1415bdd43: pop r12
1415bdd45: pop r13
1415bdd47: pop r14
1415bdd49: pop r15
1415bdd4b: pop rbp
1415bdd4c: ret 
1415bdd4d: nop dword ptr [rax]
1415bdd50: mov qword ptr [rsp + 0x10], rdx
1415bdd55: push rbp
1415bdd56: push r15
1415bdd58: push r14
1415bdd5a: push r13
1415bdd5c: push r12
1415bdd5e: push rsi
1415bdd5f: push rdi
1415bdd60: push rbx
1415bdd61: sub rsp, 0x28
1415bdd65: lea rbp, [rdx + 0x80]
1415bdd6c: mov rax, qword ptr [rbp + 0x1a0]
1415bdd73: mov qword ptr [rbp + 0x168], rax
1415bdd7a: mov rax, qword ptr [rbp + 0x1a8]
1415bdd81: mov qword ptr [rbp + 0x170], rax
1415bdd88: mov rax, qword ptr [rbp + 0x180]
1415bdd8f: mov qword ptr [rbp + 0x178], rax
1415bdd96: add rsp, 0x28
1415bdd9a: pop rbx
1415bdd9b: pop rdi
1415bdd9c: pop rsi
1415bdd9d: pop r12
1415bdd9f: pop r13
1415bdda1: pop r14
1415bdda3: pop r15
1415bdda5: pop rbp
1415bdda6: ret 
1415bdda7: nop word ptr [rax + rax]
1415bddb0: mov qword ptr [rsp + 0x10], rdx
1415bddb5: push rbp
1415bddb6: push r15
1415bddb8: push r14
1415bddba: push r13
1415bddbc: push r12
1415bddbe: push rsi
1415bddbf: push rdi
1415bddc0: push rbx
1415bddc1: sub rsp, 0x28
1415bddc5: lea rbp, [rdx + 0x80]
1415bddcc: mov rdi, qword ptr [rbp + 0x130]
1415bddd3: mov rbx, qword ptr [rbp + 0x1a0]
1415bddda: cmp rdi, rbx
1415bdddd: mov rsi, qword ptr [rbp + 0x198]
1415bdde4: je 0x1415bde05
1415bdde6: nop word ptr cs:[rax + rax]
1415bddf0: lea rax, [rbx + rbx*4]
1415bddf4: lea rcx, [rsi + rax*8]
1415bddf8: call 0x14088bae0 ; core::ptr::drop_in_place<country_core::resources::history::MultiEdit>
1415bddfd: inc rbx
1415bde00: cmp rbx, rdi
1415bde03: jne 0x1415bddf0
1415bde05: add rsp, 0x28
1415bde09: pop rbx
1415bde0a: pop rdi
1415bde0b: pop rsi
1415bde0c: pop r12
1415bde0e: pop r13
1415bde10: pop r14
1415bde12: pop r15
1415bde14: pop rbp
1415bde15: ret 
1415bde16: nop word ptr cs:[rax + rax]
1415bde20: mov qword ptr [rsp + 0x10], rdx
1415bde25: push rbp
1415bde26: push r15
1415bde28: push r14
1415bde2a: push r13
1415bde2c: push r12
1415bde2e: push rsi
1415bde2f: push rdi
1415bde30: push rbx
1415bde31: sub rsp, 0x28
1415bde35: lea rbp, [rdx + 0x80]
1415bde3c: cmp qword ptr [rbp + 0x190], 0
1415bde44: je 0x1415bde67
1415bde46: mov rax, qword ptr [rbp + 0x190]
1415bde4d: shl rax, 3
1415bde51: lea rdx, [rax + rax*4]
1415bde55: mov r8d, 8
1415bde5b: mov rcx, qword ptr [rbp + 0x198]
1415bde62: call 0x140613c20 ; 0x140613c20
1415bde67: nop 
1415bde68: add rsp, 0x28
1415bde6c: pop rbx
1415bde6d: pop rdi
1415bde6e: pop rsi
1415bde6f: pop r12
1415bde71: pop r13
1415bde73: pop r14
1415bde75: pop r15
1415bde77: pop rbp
1415bde78: ret 
1415bde79: nop dword ptr [rax]
1415bde80: mov qword ptr [rsp + 0x10], rdx
1415bde85: push rbp
1415bde86: push r15
1415bde88: push r14
1415bde8a: push r13
1415bde8c: push r12
1415bde8e: push rsi
1415bde8f: push rdi
1415bde90: push rbx
1415bde91: sub rsp, 0x28
1415bde95: lea rbp, [rdx + 0x80]
1415bde9c: xor eax, eax
1415bde9e: cmp rax, qword ptr [rbp + 0x168]
1415bdea5: jno 0x1415bdeb8
1415bdea7: add rsp, 0x28
1415bdeab: pop rbx
1415bdeac: pop rdi
1415bdead: pop rsi
1415bdeae: pop r12
1415bdeb0: pop r13
1415bdeb2: pop r14
1415bdeb4: pop r15
1415bdeb6: pop rbp
1415bdeb7: ret 
1415bdeb8: lea rcx, [rbp + 0x168]
1415bdebf: call 0x140600290 ; core::ptr::drop_in_place<country_core::resources::history::HistoryChainSnapshots>
1415bdec4: nop 
1415bdec5: add rsp, 0x28
1415bdec9: pop rbx
1415bdeca: pop rdi
1415bdecb: pop rsi
1415bdecc: pop r12
1415bdece: pop r13
1415bded0: pop r14
1415bded2: pop r15
1415bded4: pop rbp
1415bded5: ret 
1415bded6: nop word ptr cs:[rax + rax]
1415bdee0: mov qword ptr [rsp + 0x10], rdx
1415bdee5: push rbp
1415bdee6: push r15
1415bdee8: push r14
1415bdeea: push r13
1415bdeec: push r12
1415bdeee: push rsi
1415bdeef: push rdi
1415bdef0: push rbx
1415bdef1: sub rsp, 0x28
1415bdef5: lea rbp, [rdx + 0x80]
1415bdefc: neg qword ptr [rbp + 0x190]
1415bdf03: jo 0x1415bdf20
1415bdf05: lea rcx, [rbp + 8]
1415bdf09: call 0x140894d60 ; core::ptr::drop_in_place<country_core::resources::history::history_chain::HistoryEditChain>
1415bdf0e: nop 
1415bdf0f: add rsp, 0x28
1415bdf13: pop rbx
1415bdf14: pop rdi
1415bdf15: pop rsi
1415bdf16: pop r12
1415bdf18: pop r13
1415bdf1a: pop r14
1415bdf1c: pop r15
1415bdf1e: pop rbp
1415bdf1f: ret 
1415bdf20: add rsp, 0x28
1415bdf24: pop rbx
1415bdf25: pop rdi
1415bdf26: pop rsi
1415bdf27: pop r12
1415bdf29: pop r13
1415bdf2b: pop r14
1415bdf2d: pop r15
1415bdf2f: pop rbp
1415bdf30: ret 
1415bdf31: nop word ptr cs:[rax + rax]
1415bdf40: mov qword ptr [rsp + 0x10], rdx
1415bdf45: push rbp
1415bdf46: push r15
1415bdf48: push r14
1415bdf4a: push r13
1415bdf4c: push r12
1415bdf4e: push rsi
1415bdf4f: push rdi
1415bdf50: push rbx
1415bdf51: sub rsp, 0x28
1415bdf55: lea rbp, [rdx + 0x80]
1415bdf5c: lea rcx, [rbp + 0xa0]
1415bdf63: call 0x14088a500 ; core::ptr::drop_in_place<serde_json::value::de::MapDeserializer>
1415bdf68: nop 
1415bdf69: add rsp, 0x28
1415bdf6d: pop rbx
1415bdf6e: pop rdi
1415bdf6f: pop rsi
1415bdf70: pop r12
1415bdf72: pop r13
1415bdf74: pop r14
1415bdf76: pop r15
1415bdf78: pop rbp
1415bdf79: ret 
1415bdf7a: nop word ptr [rax + rax]
1415bdf80: mov qword ptr [rsp + 0x10], rdx
1415bdf85: push rbp
1415bdf86: push r15
1415bdf88: push r14
1415bdf8a: push r13
1415bdf8c: push r12
1415bdf8e: push rsi
1415bdf8f: push rdi
1415bdf90: push rbx
1415bdf91: sub rsp, 0x28
1415bdf95: lea rbp, [rdx + 0x80]
1415bdf9c: lea rcx, [rbp + 0xe8]
1415bdfa3: call 0x14088e430 ; core::ptr::drop_in_place<enum2$<core::option::Option<enum2$<serde_json::value::Value> > > >
1415bdfa8: nop 
1415bdfa9: add rsp, 0x28
1415bdfad: pop rbx
1415bdfae: pop rdi
1415bdfaf: pop rsi
1415bdfb0: pop r12
1415bdfb2: pop r13
1415bdfb4: pop r14
1415bdfb6: pop r15
1415bdfb8: pop rbp
1415bdfb9: ret 
1415bdfba: nop word ptr [rax + rax]
1415bdfc0: mov qword ptr [rsp + 0x10], rdx
1415bdfc5: push rbp
1415bdfc6: push r15
1415bdfc8: push r14
1415bdfca: push r13
1415bdfcc: push r12
1415bdfce: push rsi
1415bdfcf: push rdi
1415bdfd0: push rbx
1415bdfd1: sub rsp, 0x28
1415bdfd5: lea rbp, [rdx + 0x80]
1415bdfdc: lea rcx, [rbp + 0x20]
1415bdfe0: call 0x14088acf0 ; core::ptr::drop_in_place<country_core::resources::history::History>
1415bdfe5: nop 
1415bdfe6: add rsp, 0x28
1415bdfea: pop rbx
1415bdfeb: pop rdi
1415bdfec: pop rsi
1415bdfed: pop r12
1415bdfef: pop r13
1415bdff1: pop r14
1415bdff3: pop r15
1415bdff5: pop rbp
1415bdff6: ret 
1415bdff7: nop word ptr [rax + rax]
1415be000: mov qword ptr [rsp + 0x10], rdx
1415be005: push rbp
1415be006: push r15
1415be008: push r14
1415be00a: push r13
1415be00c: push r12
1415be00e: push rsi
1415be00f: push rdi
1415be010: push rbx
1415be011: sub rsp, 0x28
1415be015: lea rbp, [rdx + 0x80]
1415be01c: lea rcx, [rbp + 0xe8]
1415be023: call 0x14088e430 ; core::ptr::drop_in_place<enum2$<core::option::Option<enum2$<serde_json::value::Value> > > >
1415be028: nop 
1415be029: add rsp, 0x28
1415be02d: pop rbx
1415be02e: pop rdi
1415be02f: pop rsi
1415be030: pop r12
1415be032: pop r13
1415be034: pop r14
1415be036: pop r15
1415be038: pop rbp
1415be039: ret 
