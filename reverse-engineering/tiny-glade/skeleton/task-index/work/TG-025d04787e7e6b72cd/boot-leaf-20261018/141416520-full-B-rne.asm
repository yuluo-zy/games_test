141416520: 55 push rbp
141416521: 4157 push r15
141416523: 4156 push r14
141416525: 4155 push r13
141416527: 4154 push r12
141416529: 56 push rsi
14141652a: 57 push rdi
14141652b: 53 push rbx
14141652c: 4881ec38010000 sub rsp, 0x138
141416533: 488dac2480000000 lea rbp, [rsp + 0x80]
14141653b: 0f29b5a0000000 movaps xmmword ptr [rbp + 0xa0], xmm6
141416542: 48c78598000000feffffff mov qword ptr [rbp + 0x98], 0xfffffffffffffffe
14141654d: 4889d7 mov rdi, rdx
141416550: 48894d20 mov qword ptr [rbp + 0x20], rcx
141416554: 4c8b5220 mov r10, qword ptr [rdx + 0x20]
141416558: 488b4228 mov rax, qword ptr [rdx + 0x28]
14141655c: 4c39d0 cmp rax, r10
14141655f: 7345 jae 0x1414165a6
141416561: 488d5f18 lea rbx, [rdi + 0x18]
141416565: 488b5718 mov rdx, qword ptr [rdi + 0x18]
141416569: 48ffc0 inc rax
14141656c: 4d89d0 mov r8, r10
14141656f: 49f7d8 neg r8
141416572: 49bf0026000001000000 movabs r15, 0x100002600
14141657c: 0f1f4000 nop dword ptr [rax]
141416580: 440fb64c02ff movzx r9d, byte ptr [rdx + rax - 1]
141416586: 4983f920 cmp r9, 0x20
14141658a: 7733 ja 0x1414165bf
14141658c: 4d0fa3cf bt r15, r9
141416590: 732d jae 0x1414165bf
141416592: 48894728 mov qword ptr [rdi + 0x28], rax
141416596: 4d8d0c00 lea r9, [r8 + rax]
14141659a: 49ffc1 inc r9
14141659d: 48ffc0 inc rax
1414165a0: 4983f901 cmp r9, 1
1414165a4: 75da jne 0x141416580
1414165a6: 48c745d005000000 mov qword ptr [rbp - 0x30], 5
1414165ae: 488d55d0 lea rdx, [rbp - 0x30]
1414165b2: 4889f9 mov rcx, rdi
1414165b5: e866a34801 call 0x1428a0920 ; serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead>
1414165ba: e93e0e0000 jmp 0x1414173fd
1414165bf: 4183f95b cmp r9d, 0x5b
1414165c3: 0f8453050000 je 0x141416b1c
1414165c9: 4183f97b cmp r9d, 0x7b
1414165cd: 0f85e7080000 jne 0x141416eba
1414165d3: fe4f30 dec byte ptr [rdi + 0x30]
1414165d6: 0f84570f0000 je 0x141417533
1414165dc: 48894728 mov qword ptr [rdi + 0x28], rax
1414165e0: 41b501 mov r13b, 1
1414165e3: 31f6 xor esi, esi
1414165e5: 49b80100000000000080 movabs r8, 0x8000000000000001
1414165ef: 49be0000000000000080 movabs r14, 0x8000000000000000
1414165f9: 488d4dd0 lea rcx, [rbp - 0x30]
1414165fd: 4c89b588000000 mov qword ptr [rbp + 0x88], r14
141416604: 4c89b580000000 mov qword ptr [rbp + 0x80], r14
14141660b: 4c894570 mov qword ptr [rbp + 0x70], r8
14141660f: 4c39d0 cmp rax, r10
141416612: 0f836f040000 jae 0x141416a87
141416618: 488b13 mov rdx, qword ptr [rbx]
14141661b: 0f1f440000 nop dword ptr [rax + rax]
141416620: 440fb60402 movzx r8d, byte ptr [rdx + rax]
141416625: 4983f82c cmp r8, 0x2c
141416629: 7767 ja 0x141416692
14141662b: 4d0fa3c7 bt r15, r8
14141662f: 7311 jae 0x141416642
141416631: 48ffc0 inc rax
141416634: 48894728 mov qword ptr [rdi + 0x28], rax
141416638: 4939c2 cmp r10, rax
14141663b: 75e3 jne 0x141416620
14141663d: e945040000 jmp 0x141416a87
141416642: 4983f82c cmp r8, 0x2c
141416646: 754a jne 0x141416692
141416648: 41f6c501 test r13b, 1
14141664c: 0f85090a0000 jne 0x14141705b
141416652: 48ffc0 inc rax
141416655: 48894728 mov qword ptr [rdi + 0x28], rax
141416659: 4c39d0 cmp rax, r10
14141665c: 0f83d4050000 jae 0x141416c36
141416662: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
141416670: 440fb60402 movzx r8d, byte ptr [rdx + rax]
141416675: 4983f820 cmp r8, 0x20
141416679: 772e ja 0x1414166a9
14141667b: 4d0fa3c7 bt r15, r8
14141667f: 7328 jae 0x1414166a9
141416681: 48ffc0 inc rax
141416684: 48894728 mov qword ptr [rdi + 0x28], rax
141416688: 4939c2 cmp r10, rax
14141668b: 75e3 jne 0x141416670
14141668d: e9a4050000 jmp 0x141416c36
141416692: 4183f87d cmp r8d, 0x7d
141416696: 0f8441050000 je 0x141416bdd
14141669c: 41f6c501 test r13b, 1
1414166a0: 0f849a0e0000 je 0x141417540
1414166a6: 4531ed xor r13d, r13d
1414166a9: 4180f822 cmp r8b, 0x22
1414166ad: 0f8561090000 jne 0x141417014
1414166b3: 48ffc0 inc rax
1414166b6: 48894728 mov qword ptr [rdi + 0x28], rax
1414166ba: 48c7471000000000 mov qword ptr [rdi + 0x10], 0
1414166c2: 488b8588000000 mov rax, qword ptr [rbp + 0x88]
1414166c9: 48894568 mov qword ptr [rbp + 0x68], rax
1414166cd: 488b8580000000 mov rax, qword ptr [rbp + 0x80]
1414166d4: 48894560 mov qword ptr [rbp + 0x60], rax
1414166d8: 488b4570 mov rax, qword ptr [rbp + 0x70]
1414166dc: 48894558 mov qword ptr [rbp + 0x58], rax
1414166e0: c6859700000001 mov byte ptr [rbp + 0x97], 1
1414166e7: 4889da mov rdx, rbx
1414166ea: 4989f8 mov r8, rdi
1414166ed: e81e4277ff call 0x140b8a910 ; serde_json::read::impl$10::parse_str
1414166f2: 837dd002 cmp dword ptr [rbp - 0x30], 2
1414166f6: 4c8b65d8 mov r12, qword ptr [rbp - 0x28]
1414166fa: 0f84c3030000 je 0x141416ac3
141416700: 4c8b45e0 mov r8, qword ptr [rbp - 0x20]
141416704: 488d4d30 lea rcx, [rbp + 0x30]
141416708: 4c89e2 mov rdx, r12
14141670b: e810df5fff call 0x140a14620 ; country_core::startup::clearing::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
141416710: f6453001 test byte ptr [rbp + 0x30], 1
141416714: 0f857f0a0000 jne 0x141417199
14141671a: 0fb64531 movzx eax, byte ptr [rbp + 0x31]
14141671e: 488d0d4b738401 lea rcx, [rip + 0x184734b]
141416725: 48630481 movsxd rax, dword ptr [rcx + rax*4]
141416729: 4801c8 add rax, rcx
14141672c: ffe0 jmp rax
14141672e: 488b8d88000000 mov rcx, qword ptr [rbp + 0x88]
141416735: 4889c8 mov rax, rcx
141416738: 48f7d8 neg rax
14141673b: 0f81ab0e0000 jno 0x1414175ec
141416741: 4c8b4f20 mov r9, qword ptr [rdi + 0x20]
141416745: 488b4728 mov rax, qword ptr [rdi + 0x28]
141416749: 4c39c8 cmp rax, r9
14141674c: 488d4d30 lea rcx, [rbp + 0x30]
141416750: 0f83da0c0000 jae 0x141417430
141416756: 488b13 mov rdx, qword ptr [rbx]
141416759: 48ffc0 inc rax
14141675c: 49f7d9 neg r9
14141675f: 440fb64402ff movzx r8d, byte ptr [rdx + rax - 1]
141416765: 4983f83a cmp r8, 0x3a
141416769: 0f87ec0e0000 ja 0x14141765b
14141676f: 4d0fa3c7 bt r15, r8
141416773: 0f8343020000 jae 0x1414169bc
141416779: 48894728 mov qword ptr [rdi + 0x28], rax
14141677d: 4d8d0401 lea r8, [r9 + rax]
141416781: 49ffc0 inc r8
141416784: 48ffc0 inc rax
141416787: 4983f801 cmp r8, 1
14141678b: 75d2 jne 0x14141675f
14141678d: e99e0c0000 jmp 0x141417430
141416792: 488b8588000000 mov rax, qword ptr [rbp + 0x88]
141416799: 48894568 mov qword ptr [rbp + 0x68], rax
14141679d: 488b8580000000 mov rax, qword ptr [rbp + 0x80]
1414167a4: 48894560 mov qword ptr [rbp + 0x60], rax
1414167a8: 488b4570 mov rax, qword ptr [rbp + 0x70]
1414167ac: 48894558 mov qword ptr [rbp + 0x58], rax
1414167b0: c6859700000001 mov byte ptr [rbp + 0x97], 1
1414167b7: 4889f9 mov rcx, rdi
1414167ba: e84129ffff call 0x141409100 ; serde::de::MapAccess::next_value<serde_json::de::MapAccess<serde_json::read::StrRead>,serde::de::ignored_any::IgnoredAny>
1414167bf: 4989c4 mov r12, rax
1414167c2: 4885c0 test rax, rax
1414167c5: 0f85f8020000 jne 0x141416ac3
1414167cb: 4c8b6550 mov r12, qword ptr [rbp + 0x50]
1414167cf: e99a020000 jmp 0x141416a6e
1414167d4: 488b4570 mov rax, qword ptr [rbp + 0x70]
1414167d8: 49b90100000000000080 movabs r9, 0x8000000000000001
1414167e2: 4c39c8 cmp rax, r9
1414167e5: 0f85930d0000 jne 0x14141757e
1414167eb: 488b4f20 mov rcx, qword ptr [rdi + 0x20]
1414167ef: 488b4728 mov rax, qword ptr [rdi + 0x28]
1414167f3: 4839c8 cmp rax, rcx
1414167f6: 0f83a6090000 jae 0x1414171a2
1414167fc: 488b13 mov rdx, qword ptr [rbx]
1414167ff: 48ffc0 inc rax
141416802: 48f7d9 neg rcx
141416805: 440fb64402ff movzx r8d, byte ptr [rdx + rax - 1]
14141680b: 4983f83a cmp r8, 0x3a
14141680f: 0f87060f0000 ja 0x14141771b
141416815: 4d0fa3c7 bt r15, r8
141416819: 0f83d6000000 jae 0x1414168f5
14141681f: 48894728 mov qword ptr [rdi + 0x28], rax
141416823: 4c8d0401 lea r8, [rcx + rax]
141416827: 49ffc0 inc r8
14141682a: 48ffc0 inc rax
14141682d: 4983f801 cmp r8, 1
141416831: 75d2 jne 0x141416805
141416833: e96a090000 jmp 0x1414171a2
141416838: 85f6 test esi, esi
14141683a: 0f85e00d0000 jne 0x141417620
141416840: 4c8b4f20 mov r9, qword ptr [rdi + 0x20]
141416844: 488b4728 mov rax, qword ptr [rdi + 0x28]
141416848: 4c39c8 cmp rax, r9
14141684b: 488d4d30 lea rcx, [rbp + 0x30]
14141684f: 0f83800c0000 jae 0x1414174d5
141416855: 488b13 mov rdx, qword ptr [rbx]
141416858: 48ffc0 inc rax
14141685b: 49f7d9 neg r9
14141685e: 440fb64402ff movzx r8d, byte ptr [rdx + rax - 1]
141416864: 4983f83a cmp r8, 0x3a
141416868: 0f876f0e0000 ja 0x1414176dd
14141686e: 4d0fa3c7 bt r15, r8
141416872: 0f83ec000000 jae 0x141416964
141416878: 48894728 mov qword ptr [rdi + 0x28], rax
14141687c: 4d8d0401 lea r8, [r9 + rax]
141416880: 49ffc0 inc r8
141416883: 48ffc0 inc rax
141416886: 4983f801 cmp r8, 1
14141688a: 75d2 jne 0x14141685e
14141688c: e9440c0000 jmp 0x1414174d5
141416891: 488b8d80000000 mov rcx, qword ptr [rbp + 0x80]
141416898: 4889c8 mov rax, rcx
14141689b: 48f7d8 neg rax
14141689e: 0f81140d0000 jno 0x1414175b8
1414168a4: 4c8b4f20 mov r9, qword ptr [rdi + 0x20]
1414168a8: 488b4728 mov rax, qword ptr [rdi + 0x28]
1414168ac: 4c39c8 cmp rax, r9
1414168af: 488d4d30 lea rcx, [rbp + 0x30]
1414168b3: 0f83c50b0000 jae 0x14141747e
1414168b9: 488b13 mov rdx, qword ptr [rbx]
1414168bc: 48ffc0 inc rax
1414168bf: 49f7d9 neg r9
1414168c2: 440fb64402ff movzx r8d, byte ptr [rdx + rax - 1]
1414168c8: 4983f83a cmp r8, 0x3a
1414168cc: 0f87ca0d0000 ja 0x14141769c
1414168d2: 4d0fa3c7 bt r15, r8
1414168d6: 0f8336010000 jae 0x141416a12
1414168dc: 48894728 mov qword ptr [rdi + 0x28], rax
1414168e0: 4d8d0401 lea r8, [r9 + rax]
1414168e4: 49ffc0 inc r8
1414168e7: 48ffc0 inc rax
1414168ea: 4983f801 cmp r8, 1
1414168ee: 75d2 jne 0x1414168c2
1414168f0: e9890b0000 jmp 0x14141747e
1414168f5: 4983f83a cmp r8, 0x3a
1414168f9: 0f851c0e0000 jne 0x14141771b
1414168ff: 48894728 mov qword ptr [rdi + 0x28], rax
141416903: 488b8588000000 mov rax, qword ptr [rbp + 0x88]
14141690a: 48894568 mov qword ptr [rbp + 0x68], rax
14141690e: 488b8580000000 mov rax, qword ptr [rbp + 0x80]
141416915: 48894560 mov qword ptr [rbp + 0x60], rax
141416919: 4c894d58 mov qword ptr [rbp + 0x58], r9
14141691d: c6859700000001 mov byte ptr [rbp + 0x97], 1
141416924: 488d4d30 lea rcx, [rbp + 0x30]
141416928: 4889fa mov rdx, rdi
14141692b: e8a06563ff call 0x140a4ced0 ; serde::de::impls::impl$16::deserialize<relative_path::RelativePathBuf,ref_mut$<serde_json::de::Deserializer<serde_json::read::StrRead> > >
141416930: 488b4d30 mov rcx, qword ptr [rbp + 0x30]
141416934: 488b4538 mov rax, qword ptr [rbp + 0x38]
141416938: 48894d70 mov qword ptr [rbp + 0x70], rcx
14141693c: 48ba0100000000000080 movabs rdx, 0x8000000000000001
141416946: 4839d1 cmp rcx, rdx
141416949: 0f8492080000 je 0x1414171e1
14141694f: 488b4d40 mov rcx, qword ptr [rbp + 0x40]
141416953: 48894db8 mov qword ptr [rbp - 0x48], rcx
141416957: 4c8b6550 mov r12, qword ptr [rbp + 0x50]
14141695b: 48894548 mov qword ptr [rbp + 0x48], rax
14141695f: e90a010000 jmp 0x141416a6e
141416964: 4983f83a cmp r8, 0x3a
141416968: 0f856f0d0000 jne 0x1414176dd
14141696e: 48894728 mov qword ptr [rdi + 0x28], rax
141416972: 488b8588000000 mov rax, qword ptr [rbp + 0x88]
141416979: 48894568 mov qword ptr [rbp + 0x68], rax
14141697d: 488b8580000000 mov rax, qword ptr [rbp + 0x80]
141416984: 48894560 mov qword ptr [rbp + 0x60], rax
141416988: 488b4570 mov rax, qword ptr [rbp + 0x70]
14141698c: 48894558 mov qword ptr [rbp + 0x58], rax
141416990: c6859700000001 mov byte ptr [rbp + 0x97], 1
141416997: 4889fa mov rdx, rdi
14141699a: e83166ffff call 0x14140cfd0 ; serde::de::impl$5::deserialize<f32,ref_mut$<serde_json::de::Deserializer<serde_json::read::StrRead> > >
14141699f: f6453001 test byte ptr [rbp + 0x30], 1
1414169a3: 0f85f0070000 jne 0x141417199
1414169a9: f30f107534 movss xmm6, dword ptr [rbp + 0x34]
1414169ae: be01000000 mov esi, 1
1414169b3: 4c8b6550 mov r12, qword ptr [rbp + 0x50]
1414169b7: e9b2000000 jmp 0x141416a6e
1414169bc: 4983f83a cmp r8, 0x3a
1414169c0: 0f85950c0000 jne 0x14141765b
1414169c6: 48894728 mov qword ptr [rdi + 0x28], rax
1414169ca: 4c897568 mov qword ptr [rbp + 0x68], r14
1414169ce: 488b8580000000 mov rax, qword ptr [rbp + 0x80]
1414169d5: 48894560 mov qword ptr [rbp + 0x60], rax
1414169d9: 488b4570 mov rax, qword ptr [rbp + 0x70]
1414169dd: 48894558 mov qword ptr [rbp + 0x58], rax
1414169e1: c6859700000001 mov byte ptr [rbp + 0x97], 1
1414169e8: 4889fa mov rdx, rdi
1414169eb: e830d1ffff call 0x141413b20 ; serde_json::de::impl$7::deserialize_string<serde_json::read::StrRead,serde::de::impls::StringVisitor>
1414169f0: 488b4530 mov rax, qword ptr [rbp + 0x30]
1414169f4: 4c8b6538 mov r12, qword ptr [rbp + 0x38]
1414169f8: 48898588000000 mov qword ptr [rbp + 0x88], rax
1414169ff: 48f7d8 neg rax
141416a02: 0f80670a0000 jo 0x14141746f
141416a08: 488b4540 mov rax, qword ptr [rbp + 0x40]
141416a0c: 488945c0 mov qword ptr [rbp - 0x40], rax
141416a10: eb5c jmp 0x141416a6e
141416a12: 4983f83a cmp r8, 0x3a
141416a16: 0f85800c0000 jne 0x14141769c
141416a1c: 48894728 mov qword ptr [rdi + 0x28], rax
141416a20: 488b8588000000 mov rax, qword ptr [rbp + 0x88]
141416a27: 48894568 mov qword ptr [rbp + 0x68], rax
141416a2b: 4c897560 mov qword ptr [rbp + 0x60], r14
141416a2f: 488b4570 mov rax, qword ptr [rbp + 0x70]
141416a33: 48894558 mov qword ptr [rbp + 0x58], rax
141416a37: c6859700000001 mov byte ptr [rbp + 0x97], 1
141416a3e: 4889fa mov rdx, rdi
141416a41: e84ac0ffff call 0x141412a90 ; serde_json::de::impl$7::deserialize_str<serde_json::read::StrRead,relative_path::impl$56::deserialize::Visitor>
141416a46: 488b4d30 mov rcx, qword ptr [rbp + 0x30]
141416a4a: 488b4538 mov rax, qword ptr [rbp + 0x38]
141416a4e: 48898d80000000 mov qword ptr [rbp + 0x80], rcx
141416a55: 48f7d9 neg rcx
141416a58: 0f805c0a0000 jo 0x1414174ba
141416a5e: 488b4d40 mov rcx, qword ptr [rbp + 0x40]
141416a62: 48894d28 mov qword ptr [rbp + 0x28], rcx
141416a66: 4c8b6550 mov r12, qword ptr [rbp + 0x50]
141416a6a: 48894578 mov qword ptr [rbp + 0x78], rax
141416a6e: 4c8b5720 mov r10, qword ptr [rdi + 0x20]
141416a72: 488b4728 mov rax, qword ptr [rdi + 0x28]
141416a76: 4c896550 mov qword ptr [rbp + 0x50], r12
141416a7a: 488d4dd0 lea rcx, [rbp - 0x30]
141416a7e: 4c39d0 cmp rax, r10
141416a81: 0f8291fbffff jb 0x141416618
141416a87: 48c745d003000000 mov qword ptr [rbp - 0x30], 3
141416a8f: 488b8588000000 mov rax, qword ptr [rbp + 0x88]
141416a96: 48894568 mov qword ptr [rbp + 0x68], rax
141416a9a: 488b8580000000 mov rax, qword ptr [rbp + 0x80]
141416aa1: 48894560 mov qword ptr [rbp + 0x60], rax
141416aa5: 488b4570 mov rax, qword ptr [rbp + 0x70]
141416aa9: 48894558 mov qword ptr [rbp + 0x58], rax
141416aad: c6859700000001 mov byte ptr [rbp + 0x97], 1
141416ab4: 488d55d0 lea rdx, [rbp - 0x30]
141416ab8: 4889f9 mov rcx, rdi
141416abb: e8609e4801 call 0x1428a0920 ; serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead>
141416ac0: 4989c4 mov r12, rax
141416ac3: 31db xor ebx, ebx
141416ac5: 4c8b6d78 mov r13, qword ptr [rbp + 0x78]
141416ac9: 4c8bbd88000000 mov r15, qword ptr [rbp + 0x88]
141416ad0: 48b80200000000000080 movabs rax, 0x8000000000000002
141416ada: 488b5570 mov rdx, qword ptr [rbp + 0x70]
141416ade: 4839c2 cmp rdx, rax
141416ae1: 0f8c66040000 jl 0x141416f4d
141416ae7: 4885d2 test rdx, rdx
141416aea: 0f845d040000 je 0x141416f4d
141416af0: 41b801000000 mov r8d, 1
141416af6: 488b4d48 mov rcx, qword ptr [rbp + 0x48]
141416afa: e821d11fff call 0x140613c20 ; 
141416aff: 488b8580000000 mov rax, qword ptr [rbp + 0x80]
141416b06: 488d044500000000 lea rax, [rax*2]
141416b0e: 4885c0 test rax, rax
141416b11: 0f844e040000 je 0x141416f65
141416b17: e9ed060000 jmp 0x141417209
141416b1c: fe4f30 dec byte ptr [rdi + 0x30]
141416b1f: 0f840e0a0000 je 0x141417533
141416b25: 48894728 mov qword ptr [rdi + 0x28], rax
141416b29: 48897d30 mov qword ptr [rbp + 0x30], rdi
141416b2d: c6453801 mov byte ptr [rbp + 0x38], 1
141416b31: 488d4dd0 lea rcx, [rbp - 0x30]
141416b35: 488d5530 lea rdx, [rbp + 0x30]
141416b39: e8a253ffff call 0x14140bee0 ; serde_json::de::impl$9::next_element_seed<serde_json::read::StrRead,core::marker::PhantomData<alloc::string::String> >
141416b3e: 488b45d0 mov rax, qword ptr [rbp - 0x30]
141416b42: 4c8b65d8 mov r12, qword ptr [rbp - 0x28]
141416b46: 48bb0100000000000080 movabs rbx, 0x8000000000000001
141416b50: 4839d8 cmp rax, rbx
141416b53: 750f jne 0x141416b64
141416b55: 49be0000000000000080 movabs r14, 0x8000000000000000
141416b5f: e9ca010000 jmp 0x141416d2e
141416b64: 48898588000000 mov qword ptr [rbp + 0x88], rax
141416b6b: 48f7d8 neg rax
141416b6e: 0f809e030000 jo 0x141416f12
141416b74: 4c896550 mov qword ptr [rbp + 0x50], r12
141416b78: 488b45e0 mov rax, qword ptr [rbp - 0x20]
141416b7c: 488945c0 mov qword ptr [rbp - 0x40], rax
141416b80: 488d4dd0 lea rcx, [rbp - 0x30]
141416b84: 488d5530 lea rdx, [rbp + 0x30]
141416b88: e8c341ffff call 0x14140ad50 ; serde::de::SeqAccess::next_element<serde_json::de::SeqAccess<serde_json::read::StrRead>,relative_path::RelativePathBuf>
141416b8d: 488b45d0 mov rax, qword ptr [rbp - 0x30]
141416b91: 4c8b6dd8 mov r13, qword ptr [rbp - 0x28]
141416b95: 4839d8 cmp rax, rbx
141416b98: 750c jne 0x141416ba6
141416b9a: 488b9588000000 mov rdx, qword ptr [rbp + 0x88]
141416ba1: e967010000 jmp 0x141416d0d
141416ba6: 48898580000000 mov qword ptr [rbp + 0x80], rax
141416bad: 48f7d8 neg rax
141416bb0: 0f81be000000 jno 0x141416c74
141416bb6: 488d150b688401 lea rdx, [rip + 0x184680b]
141416bbd: 4c8d052c538401 lea r8, [rip + 0x184532c]
141416bc4: b901000000 mov ecx, 1
141416bc9: e852314e01 call 0x1428f9d20 ; serde::de::Error::invalid_length<serde_json::error::Error>
141416bce: 4989c5 mov r13, rax
141416bd1: 488b9588000000 mov rdx, qword ptr [rbp + 0x88]
141416bd8: e930010000 jmp 0x141416d0d
141416bdd: 4c8bb588000000 mov r14, qword ptr [rbp + 0x88]
141416be4: 4c89f0 mov rax, r14
141416be7: 48f7d8 neg rax
141416bea: 0f816c020000 jno 0x141416e5c
141416bf0: 48b80000000000000080 movabs rax, 0x8000000000000000
141416bfa: 4989c7 mov r15, rax
141416bfd: 48894568 mov qword ptr [rbp + 0x68], rax
141416c01: 488b8580000000 mov rax, qword ptr [rbp + 0x80]
141416c08: 48894560 mov qword ptr [rbp + 0x60], rax
141416c0c: 488b4570 mov rax, qword ptr [rbp + 0x70]
141416c10: 48894558 mov qword ptr [rbp + 0x58], rax
141416c14: c6859700000001 mov byte ptr [rbp + 0x97], 1
141416c1b: 488d0db6678401 lea rcx, [rip + 0x18467b6] ; @142c5d3d8 'prefabplacementsparticle_emitterbillboard_expansion_scalestruct GladeAudio with 3 elements'
141416c22: ba06000000 mov edx, 6
141416c27: e8942f4e01 call 0x1428f9bc0 ; serde::de::Error::missing_field<serde_json::error::Error>
141416c2c: 4989c4 mov r12, rax
141416c2f: 31db xor ebx, ebx
141416c31: e9fc020000 jmp 0x141416f32
141416c36: 48c745d005000000 mov qword ptr [rbp - 0x30], 5
141416c3e: 488b8588000000 mov rax, qword ptr [rbp + 0x88]
141416c45: 48894568 mov qword ptr [rbp + 0x68], rax
141416c49: 488b8580000000 mov rax, qword ptr [rbp + 0x80]
141416c50: 48894560 mov qword ptr [rbp + 0x60], rax
141416c54: 488b4570 mov rax, qword ptr [rbp + 0x70]
141416c58: 48894558 mov qword ptr [rbp + 0x58], rax
141416c5c: c6859700000001 mov byte ptr [rbp + 0x97], 1
141416c63: 488d55d0 lea rdx, [rbp - 0x30]
141416c67: 4889f9 mov rcx, rdi
141416c6a: e8b19c4801 call 0x1428a0920 ; serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead>
141416c6f: e94cfeffff jmp 0x141416ac0
141416c74: 488b4d30 mov rcx, qword ptr [rbp + 0x30]
141416c78: 4c8b4120 mov r8, qword ptr [rcx + 0x20]
141416c7c: 488b4128 mov rax, qword ptr [rcx + 0x28]
141416c80: 4c39c0 cmp rax, r8
141416c83: 4c896d78 mov qword ptr [rbp + 0x78], r13
141416c87: 7345 jae 0x141416cce
141416c89: 4c8b65e0 mov r12, qword ptr [rbp - 0x20]
141416c8d: 488b5118 mov rdx, qword ptr [rcx + 0x18]
141416c91: 48ffc0 inc rax
141416c94: 4d89c2 mov r10, r8
141416c97: 49f7da neg r10
141416c9a: 660f1f440000 nop word ptr [rax + rax]
141416ca0: 440fb64c02ff movzx r9d, byte ptr [rdx + rax - 1]
141416ca6: 4983f92c cmp r9, 0x2c
141416caa: 0f8744030000 ja 0x141416ff4
141416cb0: 4d0fa3cf bt r15, r9
141416cb4: 0f83cd020000 jae 0x141416f87
141416cba: 48894128 mov qword ptr [rcx + 0x28], rax
141416cbe: 4d8d0c02 lea r9, [r10 + rax]
141416cc2: 49ffc1 inc r9
141416cc5: 48ffc0 inc rax
141416cc8: 4983f901 cmp r9, 1
141416ccc: 75d2 jne 0x141416ca0
141416cce: 48c745d002000000 mov qword ptr [rbp - 0x30], 2
141416cd6: 488d55d0 lea rdx, [rbp - 0x30]
141416cda: e8419c4801 call 0x1428a0920 ; serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead>
141416cdf: 4989c5 mov r13, rax
141416ce2: 488b4d78 mov rcx, qword ptr [rbp + 0x78]
141416ce6: 488b9588000000 mov rdx, qword ptr [rbp + 0x88]
141416ced: 488b8580000000 mov rax, qword ptr [rbp + 0x80]
141416cf4: 4885c0 test rax, rax
141416cf7: 7414 je 0x141416d0d
141416cf9: 41b801000000 mov r8d, 1
141416cff: 4889d3 mov rbx, rdx
141416d02: 4889c2 mov rdx, rax
141416d05: e816cf1fff call 0x140613c20 ; 
141416d0a: 4889da mov rdx, rbx
141416d0d: 49be0000000000000080 movabs r14, 0x8000000000000000
141416d17: 4885d2 test rdx, rdx
141416d1a: 740f je 0x141416d2b
141416d1c: 41b801000000 mov r8d, 1
141416d22: 488b4d50 mov rcx, qword ptr [rbp + 0x50]
141416d26: e8f5ce1fff call 0x140613c20 ; 
141416d2b: 4d89ec mov r12, r13
141416d2e: fe4730 inc byte ptr [rdi + 0x30]
141416d31: 4c8975d0 mov qword ptr [rbp - 0x30], r14
141416d35: 4c8965d8 mov qword ptr [rbp - 0x28], r12
141416d39: 4889c3 mov rbx, rax
141416d3c: 488945e0 mov qword ptr [rbp - 0x20], rax
141416d40: 4c897de8 mov qword ptr [rbp - 0x18], r15
141416d44: 4c896d78 mov qword ptr [rbp + 0x78], r13
141416d48: 4c896df0 mov qword ptr [rbp - 0x10], r13
141416d4c: 48894d28 mov qword ptr [rbp + 0x28], rcx
141416d50: 48894df8 mov qword ptr [rbp - 8], rcx
141416d54: 4d89c5 mov r13, r8
141416d57: 4c894500 mov qword ptr [rbp], r8
141416d5b: 48895548 mov qword ptr [rbp + 0x48], rdx
141416d5f: 48895508 mov qword ptr [rbp + 8], rdx
141416d63: 4c894d10 mov qword ptr [rbp + 0x10], r9
141416d67: f30f114518 movss dword ptr [rbp + 0x18], xmm0
141416d6c: 4889f9 mov rcx, rdi
141416d6f: e87c17c4fe call 0x1400584f0 ; serde_json::de::Deserializer::end_seq<serde_json::read::SliceRead> | serde_json::de::Deserializer::end_seq<serde_json::read::StrRead> | serde_json::de::Deserializer::end_seq<serde_json::read::SliceRead> | serde_json::de::Deserializer::end_seq<serde_json::read::SliceRead> | serde_json::de::Deserializer::end_seq<serde_json::read::StrRead>
141416d74: 488d4d10 lea rcx, [rbp + 0x10]
141416d78: 0f1001 movups xmm0, xmmword ptr [rcx]
141416d7b: 0f294530 movaps xmmword ptr [rbp + 0x30], xmm0
141416d7f: 4c89f1 mov rcx, r14
141416d82: 48f7d9 neg rcx
141416d85: 7131 jno 0x141416db8
141416d87: 49be0000000000000080 movabs r14, 0x8000000000000000
141416d91: 4885c0 test rax, rax
141416d94: 0f84ec050000 je 0x141417386
141416d9a: 488b08 mov rcx, qword ptr [rax]
141416d9d: 4883f901 cmp rcx, 1
141416da1: 48898580000000 mov qword ptr [rbp + 0x80], rax
141416da8: 747b je 0x141416e25
141416daa: 4885c9 test rcx, rcx
141416dad: 0f8434050000 je 0x1414172e7
141416db3: e91b060000 jmp 0x1414173d3
141416db8: 4885c0 test rax, rax
141416dbb: 0f84bd050000 je 0x14141737e
141416dc1: 4d85f6 test r14, r14
141416dc4: 7417 je 0x141416ddd
141416dc6: 41b801000000 mov r8d, 1
141416dcc: 4889c6 mov rsi, rax
141416dcf: 4c89e1 mov rcx, r12
141416dd2: 4c89f2 mov rdx, r14
141416dd5: e846ce1fff call 0x140613c20 ; 
141416dda: 4889f0 mov rax, rsi
141416ddd: 4d85ff test r15, r15
141416de0: 7418 je 0x141416dfa
141416de2: 41b801000000 mov r8d, 1
141416de8: 4889c6 mov rsi, rax
141416deb: 488b4d78 mov rcx, qword ptr [rbp + 0x78]
141416def: 4c89fa mov rdx, r15
141416df2: e829ce1fff call 0x140613c20 ; 
141416df7: 4889f0 mov rax, rsi
141416dfa: 49be0000000000000080 movabs r14, 0x8000000000000000
141416e04: 4a8d0c6d00000000 lea rcx, [r13*2]
141416e0c: 4885c9 test rcx, rcx
141416e0f: 7525 jne 0x141416e36
141416e11: 4989c4 mov r12, rax
141416e14: 4c89f0 mov rax, r14
141416e17: 48f7d8 neg rax
141416e1a: 0f816e050000 jno 0x14141738e
141416e20: e9cd050000 jmp 0x1414173f2
141416e25: 4889c1 mov rcx, rax
141416e28: 4883c108 add rcx, 8
141416e2c: e8cf0b1dff call 0x1405e7a00 ; core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::option::Option<std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::option::Option<std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error>
141416e31: e99d050000 jmp 0x1414173d3
141416e36: 41b801000000 mov r8d, 1
141416e3c: 4989c4 mov r12, rax
141416e3f: 488b4d48 mov rcx, qword ptr [rbp + 0x48]
141416e43: 4c89ea mov rdx, r13
141416e46: e8d5cd1fff call 0x140613c20 ; 
141416e4b: 4c89f0 mov rax, r14
141416e4e: 48f7d8 neg rax
141416e51: 0f8137050000 jno 0x14141738e
141416e57: e996050000 jmp 0x1414173f2
141416e5c: 4c8bbd80000000 mov r15, qword ptr [rbp + 0x80]
141416e63: 4c89f8 mov rax, r15
141416e66: 48f7d8 neg rax
141416e69: 488b5dc0 mov rbx, qword ptr [rbp - 0x40]
141416e6d: 7166 jno 0x141416ed5
141416e6f: 488d0d68658401 lea rcx, [rip + 0x1846568] ; @142c5d3de 'placementsparticle_emitterbillboard_expansion_scalestruct GladeAudio with 3 elements'
141416e76: ba0a000000 mov edx, 0xa
141416e7b: e8402d4e01 call 0x1428f9bc0 ; serde::de::Error::missing_field<serde_json::error::Error>
141416e80: 4989c4 mov r12, rax
141416e83: b301 mov bl, 1
141416e85: 48b80000000000000080 movabs rax, 0x8000000000000000
141416e8f: 48898580000000 mov qword ptr [rbp + 0x80], rax
141416e96: 4c8bbd88000000 mov r15, qword ptr [rbp + 0x88]
141416e9d: 4d85ff test r15, r15
141416ea0: 0f8489000000 je 0x141416f2f
141416ea6: 41b801000000 mov r8d, 1
141416eac: 488b4d50 mov rcx, qword ptr [rbp + 0x50]
141416eb0: 4c89fa mov rdx, r15
141416eb3: e868cd1fff call 0x140613c20 ; 
141416eb8: eb78 jmp 0x141416f32
141416eba: 4c8d050f558401 lea r8, [rip + 0x184550f]
141416ec1: 488d55cf lea rdx, [rbp - 0x31]
141416ec5: 4889f9 mov rcx, rdi
141416ec8: e8331d4e01 call 0x1428f8c00 ; serde_json::de::Deserializer::peek_invalid_type<serde_json::read::StrRead>
141416ecd: 4989c4 mov r12, rax
141416ed0: e91d050000 jmp 0x1414173f2
141416ed5: 48b80100000000000080 movabs rax, 0x8000000000000001
141416edf: 488b4d70 mov rcx, qword ptr [rbp + 0x70]
141416ee3: 4839c1 cmp rcx, rax
141416ee6: 48b80000000000000080 movabs rax, 0x8000000000000000
141416ef0: 480f45c1 cmovne rax, rcx
141416ef4: 4889c1 mov rcx, rax
141416ef7: 40f6c601 test sil, 1
141416efb: 4c8b6550 mov r12, qword ptr [rbp + 0x50]
141416eff: 7508 jne 0x141416f09
141416f01: f30f1035fbe95001 movss xmm6, dword ptr [rip + 0x150e9fb]
141416f09: 4c8b6d78 mov r13, qword ptr [rbp + 0x78]
141416f0d: e948030000 jmp 0x14141725a
141416f12: 488d15af648401 lea rdx, [rip + 0x18464af]
141416f19: 4c8d05d04f8401 lea r8, [rip + 0x1844fd0]
141416f20: 31c9 xor ecx, ecx
141416f22: e8f92d4e01 call 0x1428f9d20 ; serde::de::Error::invalid_length<serde_json::error::Error>
141416f27: 4989c4 mov r12, rax
141416f2a: e926fcffff jmp 0x141416b55
141416f2f: 4531ff xor r15d, r15d
141416f32: 4c8b6d78 mov r13, qword ptr [rbp + 0x78]
141416f36: 48b80200000000000080 movabs rax, 0x8000000000000002
141416f40: 488b5570 mov rdx, qword ptr [rbp + 0x70]
141416f44: 4839c2 cmp rdx, rax
141416f47: 0f8d9afbffff jge 0x141416ae7
141416f4d: 488b8580000000 mov rax, qword ptr [rbp + 0x80]
141416f54: 488d044500000000 lea rax, [rax*2]
141416f5c: 4885c0 test rax, rax
141416f5f: 0f85a4020000 jne 0x141417209
141416f65: 49be0000000000000080 movabs r14, 0x8000000000000000
141416f6f: 84db test bl, bl
141416f71: 0f84b9020000 je 0x141417230
141416f77: 4c8bbd80000000 mov r15, qword ptr [rbp + 0x80]
141416f7e: 488b5dc0 mov rbx, qword ptr [rbp - 0x40]
141416f82: e9d3020000 jmp 0x14141725a
141416f87: 4983f92c cmp r9, 0x2c
141416f8b: 7567 jne 0x141416ff4
141416f8d: 440fb65538 movzx r10d, byte ptr [rbp + 0x38]
141416f92: 4584d2 test r10b, r10b
141416f95: 0f8503010000 jne 0x14141709e
141416f9b: 48894128 mov qword ptr [rcx + 0x28], rax
141416f9f: 4c39c0 cmp rax, r8
141416fa2: 733a jae 0x141416fde
141416fa4: 48ffc0 inc rax
141416fa7: 41ba01000000 mov r10d, 1
141416fad: 4d29c2 sub r10, r8
141416fb0: 440fb64c02ff movzx r9d, byte ptr [rdx + rax - 1]
141416fb6: 4983f920 cmp r9, 0x20
141416fba: 0f87ec000000 ja 0x1414170ac
141416fc0: 4d0fa3cf bt r15, r9
141416fc4: 0f83e2000000 jae 0x1414170ac
141416fca: 48894128 mov qword ptr [rcx + 0x28], rax
141416fce: 4d8d0402 lea r8, [r10 + rax]
141416fd2: 49ffc0 inc r8
141416fd5: 48ffc0 inc rax
141416fd8: 4983f802 cmp r8, 2
141416fdc: 75d2 jne 0x141416fb0
141416fde: 48c745d005000000 mov qword ptr [rbp - 0x30], 5
141416fe6: 488d55d0 lea rdx, [rbp - 0x30]
141416fea: e831994801 call 0x1428a0920 ; serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead>
141416fef: e9ebfcffff jmp 0x141416cdf
141416ff4: 4183f95d cmp r9d, 0x5d
141416ff8: 0f859b000000 jne 0x141417099
141416ffe: 41b601 mov r14b, 1
141417001: 48b80000000000000080 movabs rax, 0x8000000000000000
14141700b: 48894528 mov qword ptr [rbp + 0x28], rax
14141700f: e9f5000000 jmp 0x141417109
141417014: 410fb6c0 movzx eax, r8b
141417018: 83f87d cmp eax, 0x7d
14141701b: 753e jne 0x14141705b
14141701d: 48c745d012000000 mov qword ptr [rbp - 0x30], 0x12
141417025: 488b8588000000 mov rax, qword ptr [rbp + 0x88]
14141702c: 48894568 mov qword ptr [rbp + 0x68], rax
141417030: 488b8580000000 mov rax, qword ptr [rbp + 0x80]
141417037: 48894560 mov qword ptr [rbp + 0x60], rax
14141703b: 488b4570 mov rax, qword ptr [rbp + 0x70]
14141703f: 48894558 mov qword ptr [rbp + 0x58], rax
141417043: c6859700000001 mov byte ptr [rbp + 0x97], 1
14141704a: 488d55d0 lea rdx, [rbp - 0x30]
14141704e: 4889f9 mov rcx, rdi
141417051: e8ca984801 call 0x1428a0920 ; serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead>
141417056: e965faffff jmp 0x141416ac0
14141705b: 48c745d010000000 mov qword ptr [rbp - 0x30], 0x10
141417063: 488b8588000000 mov rax, qword ptr [rbp + 0x88]
14141706a: 48894568 mov qword ptr [rbp + 0x68], rax
14141706e: 488b8580000000 mov rax, qword ptr [rbp + 0x80]
141417075: 48894560 mov qword ptr [rbp + 0x60], rax
141417079: 488b4570 mov rax, qword ptr [rbp + 0x70]
14141707d: 48894558 mov qword ptr [rbp + 0x58], rax
141417081: c6859700000001 mov byte ptr [rbp + 0x97], 1
141417088: 488d55d0 lea rdx, [rbp - 0x30]
14141708c: 4889f9 mov rcx, rdi
14141708f: e88c984801 call 0x1428a0920 ; serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead>
141417094: e927faffff jmp 0x141416ac0
141417099: 440fb65538 movzx r10d, byte ptr [rbp + 0x38]
14141709e: 41f6c201 test r10b, 1
1414170a2: 0f84d3060000 je 0x14141777b
1414170a8: c6453800 mov byte ptr [rbp + 0x38], 0
1414170ac: 4889ca mov rdx, rcx
1414170af: 4180f95d cmp r9b, 0x5d
1414170b3: 0f84a6060000 je 0x14141775f
1414170b9: 488d4dd0 lea rcx, [rbp - 0x30]
1414170bd: e80e5e63ff call 0x140a4ced0 ; serde::de::impls::impl$16::deserialize<relative_path::RelativePathBuf,ref_mut$<serde_json::de::Deserializer<serde_json::read::StrRead> > >
1414170c2: 488b4dd0 mov rcx, qword ptr [rbp - 0x30]
1414170c6: 488b45d8 mov rax, qword ptr [rbp - 0x28]
1414170ca: 48894548 mov qword ptr [rbp + 0x48], rax
1414170ce: 4839d9 cmp rcx, rbx
1414170d1: 7524 jne 0x1414170f7
1414170d3: 488b4d78 mov rcx, qword ptr [rbp + 0x78]
1414170d7: 488b9588000000 mov rdx, qword ptr [rbp + 0x88]
1414170de: 488b8580000000 mov rax, qword ptr [rbp + 0x80]
1414170e5: 4c8b6d48 mov r13, qword ptr [rbp + 0x48]
1414170e9: 4885c0 test rax, rax
1414170ec: 0f8507fcffff jne 0x141416cf9
1414170f2: e916fcffff jmp 0x141416d0d
1414170f7: 4889c8 mov rax, rcx
1414170fa: 488b75e0 mov rsi, qword ptr [rbp - 0x20]
1414170fe: 48894528 mov qword ptr [rbp + 0x28], rax
141417102: 48f7d8 neg rax
141417105: 410f90c6 seto r14b
141417109: 488d4dd0 lea rcx, [rbp - 0x30]
14141710d: 488d5530 lea rdx, [rbp + 0x30]
141417111: e83a4fffff call 0x14140c050 ; serde_json::de::impl$9::next_element_seed<serde_json::read::StrRead,core::marker::PhantomData<f32> >
141417116: f645d001 test byte ptr [rbp - 0x30], 1
14141711a: 7454 je 0x141417170
14141711c: 488b5dd8 mov rbx, qword ptr [rbp - 0x28]
141417120: 4c8b4d28 mov r9, qword ptr [rbp + 0x28]
141417124: 4d85c9 test r9, r9
141417127: 0f94c0 sete al
14141712a: 4108c6 or r14b, al
14141712d: 488b4d78 mov rcx, qword ptr [rbp + 0x78]
141417131: 488b9588000000 mov rdx, qword ptr [rbp + 0x88]
141417138: 751e jne 0x141417158
14141713a: 41b801000000 mov r8d, 1
141417140: 4989ce mov r14, rcx
141417143: 488b4d48 mov rcx, qword ptr [rbp + 0x48]
141417147: 4989d7 mov r15, rdx
14141714a: 4c89ca mov rdx, r9
14141714d: e8ceca1fff call 0x140613c20 ; 
141417152: 4c89fa mov rdx, r15
141417155: 4c89f1 mov rcx, r14
141417158: 4989dd mov r13, rbx
14141715b: 488b8580000000 mov rax, qword ptr [rbp + 0x80]
141417162: 4885c0 test rax, rax
141417165: 0f858efbffff jne 0x141416cf9
14141716b: e99dfbffff jmp 0x141416d0d
141417170: 4c89e1 mov rcx, r12
141417173: f645d401 test byte ptr [rbp - 0x2c], 1
141417177: 4c8b6d78 mov r13, qword ptr [rbp + 0x78]
14141717b: 4c8bb588000000 mov r14, qword ptr [rbp + 0x88]
141417182: 4c8bbd80000000 mov r15, qword ptr [rbp + 0x80]
141417189: 0f8484030000 je 0x141417513
14141718f: f30f1045d8 movss xmm0, dword ptr [rbp - 0x28]
141417194: e982030000 jmp 0x14141751b
141417199: 4c8b6538 mov r12, qword ptr [rbp + 0x38]
14141719d: e921f9ffff jmp 0x141416ac3
1414171a2: 48c745d003000000 mov qword ptr [rbp - 0x30], 3
1414171aa: 488b8588000000 mov rax, qword ptr [rbp + 0x88]
1414171b1: 48894568 mov qword ptr [rbp + 0x68], rax
1414171b5: 488b8580000000 mov rax, qword ptr [rbp + 0x80]
1414171bc: 48894560 mov qword ptr [rbp + 0x60], rax
1414171c0: 48b80100000000000080 movabs rax, 0x8000000000000001
1414171ca: 48894558 mov qword ptr [rbp + 0x58], rax
1414171ce: c6859700000001 mov byte ptr [rbp + 0x97], 1
1414171d5: 488d55d0 lea rdx, [rbp - 0x30]
1414171d9: 4889f9 mov rcx, rdi
1414171dc: e83f974801 call 0x1428a0920 ; serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead>
1414171e1: 31db xor ebx, ebx
1414171e3: 4989c4 mov r12, rax
1414171e6: 4c8b6d78 mov r13, qword ptr [rbp + 0x78]
1414171ea: 4c8bbd88000000 mov r15, qword ptr [rbp + 0x88]
1414171f1: 488b8580000000 mov rax, qword ptr [rbp + 0x80]
1414171f8: 488d044500000000 lea rax, [rax*2]
141417200: 4885c0 test rax, rax
141417203: 0f845cfdffff je 0x141416f65
141417209: 41b801000000 mov r8d, 1
14141720f: 4c89e9 mov rcx, r13
141417212: 488b9580000000 mov rdx, qword ptr [rbp + 0x80]
141417219: e802ca1fff call 0x140613c20 ; 
14141721e: 49be0000000000000080 movabs r14, 0x8000000000000000
141417228: 84db test bl, bl
14141722a: 0f8547fdffff jne 0x141416f77
141417230: 4a8d047d00000000 lea rax, [r15*2]
141417238: 4885c0 test rax, rax
14141723b: 488b5dc0 mov rbx, qword ptr [rbp - 0x40]
14141723f: 7412 je 0x141417253
141417241: 41b801000000 mov r8d, 1
141417247: 488b4d50 mov rcx, qword ptr [rbp + 0x50]
14141724b: 4c89fa mov rdx, r15
14141724e: e8cdc91fff call 0x140613c20 ; 
141417253: 4c8bbd80000000 mov r15, qword ptr [rbp + 0x80]
14141725a: fe4730 inc byte ptr [rdi + 0x30]
14141725d: 4c8975d0 mov qword ptr [rbp - 0x30], r14
141417261: 4c8965d8 mov qword ptr [rbp - 0x28], r12
141417265: 48895de0 mov qword ptr [rbp - 0x20], rbx
141417269: 4c897de8 mov qword ptr [rbp - 0x18], r15
14141726d: 4c896d78 mov qword ptr [rbp + 0x78], r13
141417271: 4c896df0 mov qword ptr [rbp - 0x10], r13
141417275: 488b4528 mov rax, qword ptr [rbp + 0x28]
141417279: 488945f8 mov qword ptr [rbp - 8], rax
14141727d: 4989cd mov r13, rcx
141417280: 48894d00 mov qword ptr [rbp], rcx
141417284: 488b4548 mov rax, qword ptr [rbp + 0x48]
141417288: 48894508 mov qword ptr [rbp + 8], rax
14141728c: 488b45b8 mov rax, qword ptr [rbp - 0x48]
141417290: 48894510 mov qword ptr [rbp + 0x10], rax
141417294: f30f117518 movss dword ptr [rbp + 0x18], xmm6
141417299: 4889f9 mov rcx, rdi
14141729c: e8af11c4fe call 0x140058450 ; serde_json::de::Deserializer::end_map<serde_json::read::StrRead> | serde_json::de::Deserializer::end_map<serde_json::read::SliceRead> | serde_json::de::Deserializer::end_map<serde_json::read::SliceRead> | serde_json::de::Deserializer::end_map<serde_json::read::StrRead>
1414172a1: 4889c6 mov rsi, rax
1414172a4: 488d4510 lea rax, [rbp + 0x10]
1414172a8: 0f1000 movups xmm0, xmmword ptr [rax]
1414172ab: 0f294530 movaps xmmword ptr [rbp + 0x30], xmm0
1414172af: 4c89f0 mov rax, r14
1414172b2: 48f7d8 neg rax
1414172b5: 715f jno 0x141417316
1414172b7: 49be0000000000000080 movabs r14, 0x8000000000000000
1414172c1: 4885f6 test rsi, rsi
1414172c4: 0f84bc000000 je 0x141417386
1414172ca: 488b06 mov rax, qword ptr [rsi]
1414172cd: 4883f801 cmp rax, 1
1414172d1: 4889b580000000 mov qword ptr [rbp + 0x80], rsi
1414172d8: 0f84e9000000 je 0x1414173c7
1414172de: 4885c0 test rax, rax
1414172e1: 0f85ec000000 jne 0x1414173d3
1414172e7: 488b8580000000 mov rax, qword ptr [rbp + 0x80]
1414172ee: 488b5010 mov rdx, qword ptr [rax + 0x10]
1414172f2: 4885d2 test rdx, rdx
1414172f5: 0f84d8000000 je 0x1414173d3
1414172fb: 488b8580000000 mov rax, qword ptr [rbp + 0x80]
141417302: 488b4808 mov rcx, qword ptr [rax + 8]
141417306: 41b801000000 mov r8d, 1
14141730c: e80fc91fff call 0x140613c20 ; 
141417311: e9bd000000 jmp 0x1414173d3
141417316: 4885f6 test rsi, rsi
141417319: 7463 je 0x14141737e
14141731b: 4d85f6 test r14, r14
14141731e: 7411 je 0x141417331
141417320: 41b801000000 mov r8d, 1
141417326: 4c89e1 mov rcx, r12
141417329: 4c89f2 mov rdx, r14
14141732c: e8efc81fff call 0x140613c20 ; 
141417331: 4d85ff test r15, r15
141417334: 7412 je 0x141417348
141417336: 41b801000000 mov r8d, 1
14141733c: 488b4d78 mov rcx, qword ptr [rbp + 0x78]
141417340: 4c89fa mov rdx, r15
141417343: e8d8c81fff call 0x140613c20 ; 
141417348: 49be0000000000000080 movabs r14, 0x8000000000000000
141417352: 4a8d046d00000000 lea rax, [r13*2]
14141735a: 4885c0 test rax, rax
14141735d: 7412 je 0x141417371
14141735f: 41b801000000 mov r8d, 1
141417365: 488b4d48 mov rcx, qword ptr [rbp + 0x48]
141417369: 4c89ea mov rdx, r13
14141736c: e8afc81fff call 0x140613c20 ; 
141417371: 4989f4 mov r12, rsi
141417374: 4c89f0 mov rax, r14
141417377: 48f7d8 neg rax
14141737a: 7112 jno 0x14141738e
14141737c: eb74 jmp 0x1414173f2
14141737e: 0f284530 movaps xmm0, xmmword ptr [rbp + 0x30]
141417382: 0f2945a0 movaps xmmword ptr [rbp - 0x60], xmm0
141417386: 4c89f0 mov rax, r14
141417389: 48f7d8 neg rax
14141738c: 7064 jo 0x1414173f2
14141738e: 488b4d20 mov rcx, qword ptr [rbp + 0x20]
141417392: 48895910 mov qword ptr [rcx + 0x10], rbx
141417396: 4c897918 mov qword ptr [rcx + 0x18], r15
14141739a: 488b4578 mov rax, qword ptr [rbp + 0x78]
14141739e: 48894120 mov qword ptr [rcx + 0x20], rax
1414173a2: 488b4528 mov rax, qword ptr [rbp + 0x28]
1414173a6: 48894128 mov qword ptr [rcx + 0x28], rax
1414173aa: 4c896930 mov qword ptr [rcx + 0x30], r13
1414173ae: 488b4548 mov rax, qword ptr [rbp + 0x48]
1414173b2: 48894138 mov qword ptr [rcx + 0x38], rax
1414173b6: 0f2845a0 movaps xmm0, xmmword ptr [rbp - 0x60]
1414173ba: 0f114140 movups xmmword ptr [rcx + 0x40], xmm0
1414173be: 4c8931 mov qword ptr [rcx], r14
1414173c1: 4c896108 mov qword ptr [rcx + 8], r12
1414173c5: eb4b jmp 0x141417412
1414173c7: 4889f1 mov rcx, rsi
1414173ca: 4883c108 add rcx, 8
1414173ce: e82d061dff call 0x1405e7a00 ; core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::option::Option<std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::option::Option<std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error>
1414173d3: ba28000000 mov edx, 0x28
1414173d8: 41b808000000 mov r8d, 8
1414173de: 488b8d80000000 mov rcx, qword ptr [rbp + 0x80]
1414173e5: e836c81fff call 0x140613c20 ; 
1414173ea: 4c89f0 mov rax, r14
1414173ed: 48f7d8 neg rax
1414173f0: 719c jno 0x14141738e
1414173f2: 4c89e1 mov rcx, r12
1414173f5: 4889fa mov rdx, rdi
1414173f8: e803ae4801 call 0x1428a2200 ; serde_json::error::Error::fix_position<serde_json::de::impl$5::fix_position::closure_env$0<serde_json::read::SliceRead> > | serde_json::error::Error::fix_position<serde_json::de::impl$5::fix_position::closure_env$0<serde_json::read::StrRead> > | serde_json::error::Error::fix_position<serde_json::de::impl$5::fix_position::closure_env$0<serde_json::read::SliceRead> > | serde_json::error::Error::fix_position<serde_json::de::impl$5::fix_position::closure_env$0<serde_json::read::StrRead> > | serde_json::error::Error::fix_position<serde_json::de::impl$5::fix_position::closure_env$0<serde_json::read::SliceRead> > | serde_json::error::Error::fix_position<serde_json::de::impl$5::fix_position::closure_env$0<serde_json::read::SliceRead> > | serde_json::error::Error::fix_position<serde_json::de::impl$5::fix_position::closure_env$0<serde_json::read::StrRead> > | serde_json::error::Error::fix_position<serde_json::de::impl$5::fix_position::closure_env$0<serde_json::read::StrRead> > | serde_json::error::Error::fix_position<serde_json::de::impl$5::fix_position::closure_env$0<serde_json::read::StrRead> >
1414173fd: 488b4d20 mov rcx, qword ptr [rbp + 0x20]
141417401: 48894108 mov qword ptr [rcx + 8], rax
141417405: 48b80000000000000080 movabs rax, 0x8000000000000000
14141740f: 488901 mov qword ptr [rcx], rax
141417412: 4889c8 mov rax, rcx
141417415: 0f28b5a0000000 movaps xmm6, xmmword ptr [rbp + 0xa0]
14141741c: 4881c438010000 add rsp, 0x138
141417423: 5b pop rbx
141417424: 5f pop rdi
141417425: 5e pop rsi
141417426: 415c pop r12
141417428: 415d pop r13
14141742a: 415e pop r14
14141742c: 415f pop r15
14141742e: 5d pop rbp
14141742f: c3 ret 
141417430: 48c745d003000000 mov qword ptr [rbp - 0x30], 3
141417438: 48b80000000000000080 movabs rax, 0x8000000000000000
141417442: 48894568 mov qword ptr [rbp + 0x68], rax
141417446: 488b8580000000 mov rax, qword ptr [rbp + 0x80]
14141744d: 48894560 mov qword ptr [rbp + 0x60], rax
141417451: 488b4570 mov rax, qword ptr [rbp + 0x70]
141417455: 48894558 mov qword ptr [rbp + 0x58], rax
141417459: c6859700000001 mov byte ptr [rbp + 0x97], 1
141417460: 488d55d0 lea rdx, [rbp - 0x30]
141417464: 4889f9 mov rcx, rdi
141417467: e8b4944801 call 0x1428a0920 ; serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead>
14141746c: 4989c4 mov r12, rax
14141746f: 49bf0000000000000080 movabs r15, 0x8000000000000000
141417479: e9b1f7ffff jmp 0x141416c2f
14141747e: 48c745d003000000 mov qword ptr [rbp - 0x30], 3
141417486: 488b8588000000 mov rax, qword ptr [rbp + 0x88]
14141748d: 48894568 mov qword ptr [rbp + 0x68], rax
141417491: 48b80000000000000080 movabs rax, 0x8000000000000000
14141749b: 48894560 mov qword ptr [rbp + 0x60], rax
14141749f: 488b4570 mov rax, qword ptr [rbp + 0x70]
1414174a3: 48894558 mov qword ptr [rbp + 0x58], rax
1414174a7: c6859700000001 mov byte ptr [rbp + 0x97], 1
1414174ae: 488d55d0 lea rdx, [rbp - 0x30]
1414174b2: 4889f9 mov rcx, rdi
1414174b5: e866944801 call 0x1428a0920 ; serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead>
1414174ba: 48b90000000000000080 movabs rcx, 0x8000000000000000
1414174c4: 48898d80000000 mov qword ptr [rbp + 0x80], rcx
1414174cb: 31db xor ebx, ebx
1414174cd: 4989c4 mov r12, rax
1414174d0: e9f0f5ffff jmp 0x141416ac5
1414174d5: 48c745d003000000 mov qword ptr [rbp - 0x30], 3
1414174dd: 488b8588000000 mov rax, qword ptr [rbp + 0x88]
1414174e4: 48894568 mov qword ptr [rbp + 0x68], rax
1414174e8: 488b8580000000 mov rax, qword ptr [rbp + 0x80]
1414174ef: 48894560 mov qword ptr [rbp + 0x60], rax
1414174f3: 488b4570 mov rax, qword ptr [rbp + 0x70]
1414174f7: 48894558 mov qword ptr [rbp + 0x58], rax
1414174fb: c6859700000001 mov byte ptr [rbp + 0x97], 1
141417502: 488d55d0 lea rdx, [rbp - 0x30]
141417506: 4889f9 mov rcx, rdi
141417509: e812944801 call 0x1428a0920 ; serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead>
14141750e: e9adf5ffff jmp 0x141416ac0
141417513: f30f1005e9e35001 movss xmm0, dword ptr [rip + 0x150e3e9]
14141751b: 4c8b6550 mov r12, qword ptr [rbp + 0x50]
14141751f: 488b45c0 mov rax, qword ptr [rbp - 0x40]
141417523: 488b5548 mov rdx, qword ptr [rbp + 0x48]
141417527: 4c8b4528 mov r8, qword ptr [rbp + 0x28]
14141752b: 4989f1 mov r9, rsi
14141752e: e9fbf7ffff jmp 0x141416d2e
141417533: 48c745d015000000 mov qword ptr [rbp - 0x30], 0x15
14141753b: e96ef0ffff jmp 0x1414165ae
141417540: 48c745d008000000 mov qword ptr [rbp - 0x30], 8
141417548: 488b8588000000 mov rax, qword ptr [rbp + 0x88]
14141754f: 48894568 mov qword ptr [rbp + 0x68], rax
141417553: 488b8580000000 mov rax, qword ptr [rbp + 0x80]
14141755a: 48894560 mov qword ptr [rbp + 0x60], rax
14141755e: 488b4570 mov rax, qword ptr [rbp + 0x70]
141417562: 48894558 mov qword ptr [rbp + 0x58], rax
141417566: c6859700000001 mov byte ptr [rbp + 0x97], 1
14141756d: 488d55d0 lea rdx, [rbp - 0x30]
141417571: 4889f9 mov rcx, rdi
141417574: e8a7934801 call 0x1428a0920 ; serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead>
141417579: e942f5ffff jmp 0x141416ac0
14141757e: 4889c1 mov rcx, rax
141417581: 488b8588000000 mov rax, qword ptr [rbp + 0x88]
141417588: 48894568 mov qword ptr [rbp + 0x68], rax
14141758c: 488b8580000000 mov rax, qword ptr [rbp + 0x80]
141417593: 48894560 mov qword ptr [rbp + 0x60], rax
141417597: 48894d58 mov qword ptr [rbp + 0x58], rcx
14141759b: c6859700000001 mov byte ptr [rbp + 0x97], 1
1414175a2: 488d0d3f5e8401 lea rcx, [rip + 0x1845e3f] ; @142c5d3e8 'particle_emitterbillboard_expansion_scalestruct GladeAudio with 3 elements'
1414175a9: ba10000000 mov edx, 0x10
1414175ae: e8fd274e01 call 0x1428f9db0 ; serde::de::Error::duplicate_field<serde_json::error::Error>
1414175b3: e908f5ffff jmp 0x141416ac0
1414175b8: 488b8588000000 mov rax, qword ptr [rbp + 0x88]
1414175bf: 48894568 mov qword ptr [rbp + 0x68], rax
1414175c3: 48894d60 mov qword ptr [rbp + 0x60], rcx
1414175c7: 488b4570 mov rax, qword ptr [rbp + 0x70]
1414175cb: 48894558 mov qword ptr [rbp + 0x58], rax
1414175cf: c6859700000001 mov byte ptr [rbp + 0x97], 1
1414175d6: 488d0d015e8401 lea rcx, [rip + 0x1845e01] ; @142c5d3de 'placementsparticle_emitterbillboard_expansion_scalestruct GladeAudio with 3 elements'
1414175dd: ba0a000000 mov edx, 0xa
1414175e2: e8c9274e01 call 0x1428f9db0 ; serde::de::Error::duplicate_field<serde_json::error::Error>
1414175e7: e9d4f4ffff jmp 0x141416ac0
1414175ec: 48894d68 mov qword ptr [rbp + 0x68], rcx
1414175f0: 488b8580000000 mov rax, qword ptr [rbp + 0x80]
1414175f7: 48894560 mov qword ptr [rbp + 0x60], rax
1414175fb: 488b4570 mov rax, qword ptr [rbp + 0x70]
1414175ff: 48894558 mov qword ptr [rbp + 0x58], rax
141417603: c6859700000001 mov byte ptr [rbp + 0x97], 1
14141760a: 488d0dc75d8401 lea rcx, [rip + 0x1845dc7] ; @142c5d3d8 'prefabplacementsparticle_emitterbillboard_expansion_scalestruct GladeAudio with 3 elements'
141417611: ba06000000 mov edx, 6
141417616: e895274e01 call 0x1428f9db0 ; serde::de::Error::duplicate_field<serde_json::error::Error>
14141761b: e9a0f4ffff jmp 0x141416ac0
141417620: 488b8588000000 mov rax, qword ptr [rbp + 0x88]
141417627: 48894568 mov qword ptr [rbp + 0x68], rax
14141762b: 488b8580000000 mov rax, qword ptr [rbp + 0x80]
141417632: 48894560 mov qword ptr [rbp + 0x60], rax
141417636: 488b4570 mov rax, qword ptr [rbp + 0x70]
14141763a: 48894558 mov qword ptr [rbp + 0x58], rax
14141763e: c6859700000001 mov byte ptr [rbp + 0x97], 1
141417645: 488d0dac5d8401 lea rcx, [rip + 0x1845dac] ; @142c5d3f8 'billboard_expansion_scalestruct GladeAudio with 3 elements'
14141764c: ba19000000 mov edx, 0x19
141417651: e85a274e01 call 0x1428f9db0 ; serde::de::Error::duplicate_field<serde_json::error::Error>
141417656: e965f4ffff jmp 0x141416ac0
14141765b: 48c745d006000000 mov qword ptr [rbp - 0x30], 6
141417663: 48b80000000000000080 movabs rax, 0x8000000000000000
14141766d: 48894568 mov qword ptr [rbp + 0x68], rax
141417671: 488b8580000000 mov rax, qword ptr [rbp + 0x80]
141417678: 48894560 mov qword ptr [rbp + 0x60], rax
14141767c: 488b4570 mov rax, qword ptr [rbp + 0x70]
141417680: 48894558 mov qword ptr [rbp + 0x58], rax
141417684: c6859700000001 mov byte ptr [rbp + 0x97], 1
14141768b: 488d55d0 lea rdx, [rbp - 0x30]
14141768f: 4889f9 mov rcx, rdi
141417692: e889924801 call 0x1428a0920 ; serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead>
141417697: e9d0fdffff jmp 0x14141746c
14141769c: 48c745d006000000 mov qword ptr [rbp - 0x30], 6
1414176a4: 488b8588000000 mov rax, qword ptr [rbp + 0x88]
1414176ab: 48894568 mov qword ptr [rbp + 0x68], rax
1414176af: 48b80000000000000080 movabs rax, 0x8000000000000000
1414176b9: 48894560 mov qword ptr [rbp + 0x60], rax
1414176bd: 488b4570 mov rax, qword ptr [rbp + 0x70]
1414176c1: 48894558 mov qword ptr [rbp + 0x58], rax
1414176c5: c6859700000001 mov byte ptr [rbp + 0x97], 1
1414176cc: 488d55d0 lea rdx, [rbp - 0x30]
1414176d0: 4889f9 mov rcx, rdi
1414176d3: e848924801 call 0x1428a0920 ; serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead>
1414176d8: e9ddfdffff jmp 0x1414174ba
1414176dd: 48c745d006000000 mov qword ptr [rbp - 0x30], 6
1414176e5: 488b8588000000 mov rax, qword ptr [rbp + 0x88]
1414176ec: 48894568 mov qword ptr [rbp + 0x68], rax
1414176f0: 488b8580000000 mov rax, qword ptr [rbp + 0x80]
1414176f7: 48894560 mov qword ptr [rbp + 0x60], rax
1414176fb: 488b4570 mov rax, qword ptr [rbp + 0x70]
1414176ff: 48894558 mov qword ptr [rbp + 0x58], rax
141417703: c6859700000001 mov byte ptr [rbp + 0x97], 1
14141770a: 488d55d0 lea rdx, [rbp - 0x30]
14141770e: 4889f9 mov rcx, rdi
141417711: e80a924801 call 0x1428a0920 ; serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead>
141417716: e9a5f3ffff jmp 0x141416ac0
14141771b: 48c745d006000000 mov qword ptr [rbp - 0x30], 6
141417723: 488b8588000000 mov rax, qword ptr [rbp + 0x88]
14141772a: 48894568 mov qword ptr [rbp + 0x68], rax
14141772e: 488b8580000000 mov rax, qword ptr [rbp + 0x80]
141417735: 48894560 mov qword ptr [rbp + 0x60], rax
141417739: 48b80100000000000080 movabs rax, 0x8000000000000001
141417743: 48894558 mov qword ptr [rbp + 0x58], rax
141417747: c6859700000001 mov byte ptr [rbp + 0x97], 1
14141774e: 488d55d0 lea rdx, [rbp - 0x30]
141417752: 4889f9 mov rcx, rdi
141417755: e8c6914801 call 0x1428a0920 ; serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead>
14141775a: e982faffff jmp 0x1414171e1
14141775f: 48c745d012000000 mov qword ptr [rbp - 0x30], 0x12
141417767: 488d45d0 lea rax, [rbp - 0x30]
14141776b: 4889d1 mov rcx, rdx
14141776e: 4889c2 mov rdx, rax
141417771: e8aa914801 call 0x1428a0920 ; serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead>
141417776: e964f5ffff jmp 0x141416cdf
14141777b: 48c745d007000000 mov qword ptr [rbp - 0x30], 7
141417783: 488d55d0 lea rdx, [rbp - 0x30]
141417787: e894914801 call 0x1428a0920 ; serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead>
14141778c: e94ef5ffff jmp 0x141416cdf
141417791: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1414177a0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1414177a5: 55 push rbp
1414177a6: 4157 push r15
1414177a8: 4156 push r14
1414177aa: 4155 push r13
1414177ac: 4154 push r12
1414177ae: 56 push rsi
1414177af: 57 push rdi
1414177b0: 53 push rbx
1414177b1: 4883ec38 sub rsp, 0x38
1414177b5: 488daa80000000 lea rbp, [rdx + 0x80]
1414177bc: 0f29742420 movaps xmmword ptr [rsp + 0x20], xmm6
1414177c1: 488b4528 mov rax, qword ptr [rbp + 0x28]
1414177c5: 488d044500000000 lea rax, [rax*2]
1414177cd: 4885c0 test rax, rax
1414177d0: 7413 je 0x1414177e5
1414177d2: 41b801000000 mov r8d, 1
1414177d8: 488b4d48 mov rcx, qword ptr [rbp + 0x48]
1414177dc: 488b5528 mov rdx, qword ptr [rbp + 0x28]
1414177e0: e83bc41fff call 0x140613c20 ; 
1414177e5: 0f28742420 movaps xmm6, xmmword ptr [rsp + 0x20]
1414177ea: 4883c438 add rsp, 0x38
1414177ee: 5b pop rbx
1414177ef: 5f pop rdi
1414177f0: 5e pop rsi
1414177f1: 415c pop r12
1414177f3: 415d pop r13
1414177f5: 415e pop r14
1414177f7: 415f pop r15
1414177f9: 5d pop rbp
1414177fa: c3 ret 
1414177fb: 0f1f440000 nop dword ptr [rax + rax]
141417800: 4889542410 mov qword ptr [rsp + 0x10], rdx
141417805: 55 push rbp
141417806: 4157 push r15
141417808: 4156 push r14
14141780a: 4155 push r13
14141780c: 4154 push r12
14141780e: 56 push rsi
14141780f: 57 push rdi
141417810: 53 push rbx
141417811: 4883ec38 sub rsp, 0x38
141417815: 488daa80000000 lea rbp, [rdx + 0x80]
14141781c: 0f29742420 movaps xmmword ptr [rsp + 0x20], xmm6
141417821: 4883bd8800000000 cmp qword ptr [rbp + 0x88], 0
141417829: 7416 je 0x141417841
14141782b: 41b801000000 mov r8d, 1
141417831: 488b4d50 mov rcx, qword ptr [rbp + 0x50]
141417835: 488b9588000000 mov rdx, qword ptr [rbp + 0x88]
14141783c: e8dfc31fff call 0x140613c20 ; 
141417841: 488b8588000000 mov rax, qword ptr [rbp + 0x88]
141417848: 48894568 mov qword ptr [rbp + 0x68], rax
14141784c: 48b80000000000000080 movabs rax, 0x8000000000000000
141417856: 48894560 mov qword ptr [rbp + 0x60], rax
14141785a: 488b4570 mov rax, qword ptr [rbp + 0x70]
14141785e: 48894558 mov qword ptr [rbp + 0x58], rax
141417862: c6859700000000 mov byte ptr [rbp + 0x97], 0
141417869: 0f28742420 movaps xmm6, xmmword ptr [rsp + 0x20]
14141786e: 4883c438 add rsp, 0x38
141417872: 5b pop rbx
141417873: 5f pop rdi
141417874: 5e pop rsi
141417875: 415c pop r12
141417877: 415d pop r13
141417879: 415e pop r14
14141787b: 415f pop r15
14141787d: 5d pop rbp
14141787e: c3 ret 
14141787f: 90 nop 
141417880: 4889542410 mov qword ptr [rsp + 0x10], rdx
141417885: 55 push rbp
141417886: 4157 push r15
141417888: 4156 push r14
14141788a: 4155 push r13
14141788c: 4154 push r12
14141788e: 56 push rsi
14141788f: 57 push rdi
141417890: 53 push rbx
141417891: 4883ec38 sub rsp, 0x38
141417895: 488daa80000000 lea rbp, [rdx + 0x80]
14141789c: 0f29742420 movaps xmmword ptr [rsp + 0x20], xmm6
1414178a1: ba28000000 mov edx, 0x28
1414178a6: 41b808000000 mov r8d, 8
1414178ac: 488b8d80000000 mov rcx, qword ptr [rbp + 0x80]
1414178b3: e868c31fff call 0x140613c20 ; 
1414178b8: 0f28742420 movaps xmm6, xmmword ptr [rsp + 0x20]
1414178bd: 4883c438 add rsp, 0x38
1414178c1: 5b pop rbx
1414178c2: 5f pop rdi
1414178c3: 5e pop rsi
1414178c4: 415c pop r12
1414178c6: 415d pop r13
1414178c8: 415e pop r14
1414178ca: 415f pop r15
1414178cc: 5d pop rbp
1414178cd: c3 ret 
1414178ce: 6690 nop 
1414178d0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1414178d5: 55 push rbp
1414178d6: 4157 push r15
1414178d8: 4156 push r14
1414178da: 4155 push r13
1414178dc: 4154 push r12
1414178de: 56 push rsi
1414178df: 57 push rdi
1414178e0: 53 push rbx
1414178e1: 4883ec38 sub rsp, 0x38
1414178e5: 488daa80000000 lea rbp, [rdx + 0x80]
1414178ec: 0f29742420 movaps xmmword ptr [rsp + 0x20], xmm6
1414178f1: ba28000000 mov edx, 0x28
1414178f6: 41b808000000 mov r8d, 8
1414178fc: 488b8d80000000 mov rcx, qword ptr [rbp + 0x80]
141417903: e818c31fff call 0x140613c20 ; 
141417908: 0f28742420 movaps xmm6, xmmword ptr [rsp + 0x20]
14141790d: 4883c438 add rsp, 0x38
141417911: 5b pop rbx
141417912: 5f pop rdi
141417913: 5e pop rsi
141417914: 415c pop r12
141417916: 415d pop r13
141417918: 415e pop r14
14141791a: 415f pop r15
14141791c: 5d pop rbp
14141791d: c3 ret 
14141791e: 6690 nop 
141417920: 4889542410 mov qword ptr [rsp + 0x10], rdx
141417925: 55 push rbp
141417926: 4157 push r15
141417928: 4156 push r14
14141792a: 4155 push r13
14141792c: 4154 push r12
14141792e: 56 push rsi
14141792f: 57 push rdi
141417930: 53 push rbx
141417931: 4883ec38 sub rsp, 0x38
141417935: 488daa80000000 lea rbp, [rdx + 0x80]
14141793c: 0f29742420 movaps xmmword ptr [rsp + 0x20], xmm6
141417941: 4883bd8000000000 cmp qword ptr [rbp + 0x80], 0
141417949: 7416 je 0x141417961
14141794b: 41b801000000 mov r8d, 1
141417951: 488b4d78 mov rcx, qword ptr [rbp + 0x78]
141417955: 488b9580000000 mov rdx, qword ptr [rbp + 0x80]
14141795c: e8bfc21fff call 0x140613c20 ; 
141417961: 0f28742420 movaps xmm6, xmmword ptr [rsp + 0x20]
141417966: 4883c438 add rsp, 0x38
14141796a: 5b pop rbx
14141796b: 5f pop rdi
14141796c: 5e pop rsi
14141796d: 415c pop r12
14141796f: 415d pop r13
141417971: 415e pop r14
141417973: 415f pop r15
141417975: 5d pop rbp
141417976: c3 ret 
141417977: 660f1f840000000000 nop word ptr [rax + rax]
141417980: 4889542410 mov qword ptr [rsp + 0x10], rdx
141417985: 55 push rbp
141417986: 4157 push r15
141417988: 4156 push r14
14141798a: 4155 push r13
14141798c: 4154 push r12
14141798e: 56 push rsi
14141798f: 57 push rdi
141417990: 53 push rbx
141417991: 4883ec38 sub rsp, 0x38
141417995: 488daa80000000 lea rbp, [rdx + 0x80]
14141799c: 0f29742420 movaps xmmword ptr [rsp + 0x20], xmm6
1414179a1: 4883bd8800000000 cmp qword ptr [rbp + 0x88], 0
1414179a9: 7416 je 0x1414179c1
1414179ab: 41b801000000 mov r8d, 1
1414179b1: 488b4d50 mov rcx, qword ptr [rbp + 0x50]
1414179b5: 488b9588000000 mov rdx, qword ptr [rbp + 0x88]
1414179bc: e85fc21fff call 0x140613c20 ; 
1414179c1: 0f28742420 movaps xmm6, xmmword ptr [rsp + 0x20]
1414179c6: 4883c438 add rsp, 0x38
1414179ca: 5b pop rbx
1414179cb: 5f pop rdi
1414179cc: 5e pop rsi
1414179cd: 415c pop r12
1414179cf: 415d pop r13
1414179d1: 415e pop r14
1414179d3: 415f pop r15
1414179d5: 5d pop rbp
1414179d6: c3 ret 
1414179d7: 660f1f840000000000 nop word ptr [rax + rax]
1414179e0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1414179e5: 55 push rbp
1414179e6: 4157 push r15
1414179e8: 4156 push r14
1414179ea: 4155 push r13
1414179ec: 4154 push r12
1414179ee: 56 push rsi
1414179ef: 57 push rdi
1414179f0: 53 push rbx
1414179f1: 4883ec38 sub rsp, 0x38
1414179f5: 488daa80000000 lea rbp, [rdx + 0x80]
1414179fc: 0f29742420 movaps xmmword ptr [rsp + 0x20], xmm6
141417a01: 488d4dd0 lea rcx, [rbp - 0x30]
141417a05: e8360bffff call 0x141408540 ; core::ptr::drop_in_place<enum2$<core::result::Result<country_core::startup::clearing::PlacementConfigEntry,serde_json::error::Error> > >
141417a0a: 0f28742420 movaps xmm6, xmmword ptr [rsp + 0x20]
141417a0f: 4883c438 add rsp, 0x38
141417a13: 5b pop rbx
141417a14: 5f pop rdi
141417a15: 5e pop rsi
141417a16: 415c pop r12
141417a18: 415d pop r13
141417a1a: 415e pop r14
141417a1c: 415f pop r15
141417a1e: 5d pop rbp
141417a1f: c3 ret 
141417a20: 4889542410 mov qword ptr [rsp + 0x10], rdx
141417a25: 55 push rbp
141417a26: 4157 push r15
141417a28: 4156 push r14
141417a2a: 4155 push r13
141417a2c: 4154 push r12
141417a2e: 56 push rsi
141417a2f: 57 push rdi
141417a30: 53 push rbx
141417a31: 4883ec38 sub rsp, 0x38
141417a35: 488daa80000000 lea rbp, [rdx + 0x80]
141417a3c: 0f29742420 movaps xmmword ptr [rsp + 0x20], xmm6
141417a41: 488d4dd0 lea rcx, [rbp - 0x30]
141417a45: e8f60affff call 0x141408540 ; core::ptr::drop_in_place<enum2$<core::result::Result<country_core::startup::clearing::PlacementConfigEntry,serde_json::error::Error> > >
141417a4a: 0f28742420 movaps xmm6, xmmword ptr [rsp + 0x20]
141417a4f: 4883c438 add rsp, 0x38
141417a53: 5b pop rbx
141417a54: 5f pop rdi
141417a55: 5e pop rsi
141417a56: 415c pop r12
141417a58: 415d pop r13
141417a5a: 415e pop r14
141417a5c: 415f pop r15
141417a5e: 5d pop rbp
141417a5f: c3 ret 
141417a60: 4889542410 mov qword ptr [rsp + 0x10], rdx
141417a65: 55 push rbp
141417a66: 4157 push r15
141417a68: 4156 push r14
141417a6a: 4155 push r13
141417a6c: 4154 push r12
141417a6e: 56 push rsi
141417a6f: 57 push rdi
141417a70: 53 push rbx
141417a71: 4883ec38 sub rsp, 0x38
141417a75: 488daa80000000 lea rbp, [rdx + 0x80]
141417a7c: 0f29742420 movaps xmmword ptr [rsp + 0x20], xmm6
141417a81: 0fb68597000000 movzx eax, byte ptr [rbp + 0x97]
141417a88: 888588000000 mov byte ptr [rbp + 0x88], al
141417a8e: 488b5558 mov rdx, qword ptr [rbp + 0x58]
141417a92: 488b4560 mov rax, qword ptr [rbp + 0x60]
141417a96: 48898580000000 mov qword ptr [rbp + 0x80], rax
141417a9d: 488b4568 mov rax, qword ptr [rbp + 0x68]
141417aa1: 48894520 mov qword ptr [rbp + 0x20], rax
141417aa5: 4889d0 mov rax, rdx
141417aa8: 48f7d8 neg rax
141417aab: 7023 jo 0x141417ad0
141417aad: 48b80100000000000080 movabs rax, 0x8000000000000001
141417ab7: 4839c2 cmp rdx, rax
141417aba: 742a je 0x141417ae6
141417abc: 4885d2 test rdx, rdx
141417abf: 740f je 0x141417ad0
141417ac1: 41b801000000 mov r8d, 1
141417ac7: 488b4d48 mov rcx, qword ptr [rbp + 0x48]
141417acb: e850c11fff call 0x140613c20 ; 
141417ad0: 0f28742420 movaps xmm6, xmmword ptr [rsp + 0x20]
141417ad5: 4883c438 add rsp, 0x38
141417ad9: 5b pop rbx
141417ada: 5f pop rdi
141417adb: 5e pop rsi
141417adc: 415c pop r12
141417ade: 415d pop r13
141417ae0: 415e pop r14
141417ae2: 415f pop r15
141417ae4: 5d pop rbp
141417ae5: c3 ret 
141417ae6: 0f28742420 movaps xmm6, xmmword ptr [rsp + 0x20]
141417aeb: 4883c438 add rsp, 0x38
141417aef: 5b pop rbx
141417af0: 5f pop rdi
141417af1: 5e pop rsi
141417af2: 415c pop r12
141417af4: 415d pop r13
141417af6: 415e pop r14
141417af8: 415f pop r15
141417afa: 5d pop rbp
141417afb: c3 ret 
141417afc: 0f1f4000 nop dword ptr [rax]
141417b00: 4889542410 mov qword ptr [rsp + 0x10], rdx
141417b05: 55 push rbp
141417b06: 4157 push r15
141417b08: 4156 push r14
141417b0a: 4155 push r13
141417b0c: 4154 push r12
141417b0e: 56 push rsi
141417b0f: 57 push rdi
141417b10: 53 push rbx
141417b11: 4883ec38 sub rsp, 0x38
141417b15: 488daa80000000 lea rbp, [rdx + 0x80]
141417b1c: 0f29742420 movaps xmmword ptr [rsp + 0x20], xmm6
141417b21: 488b8580000000 mov rax, qword ptr [rbp + 0x80]
141417b28: 48f7d8 neg rax
141417b2b: 702e jo 0x141417b5b
141417b2d: 7316 jae 0x141417b45
141417b2f: 41b801000000 mov r8d, 1
141417b35: 488b4d78 mov rcx, qword ptr [rbp + 0x78]
141417b39: 488b9580000000 mov rdx, qword ptr [rbp + 0x80]
141417b40: e8dbc01fff call 0x140613c20 ; 
141417b45: 0f28742420 movaps xmm6, xmmword ptr [rsp + 0x20]
141417b4a: 4883c438 add rsp, 0x38
141417b4e: 5b pop rbx
141417b4f: 5f pop rdi
141417b50: 5e pop rsi
141417b51: 415c pop r12
141417b53: 415d pop r13
141417b55: 415e pop r14
141417b57: 415f pop r15
141417b59: 5d pop rbp
141417b5a: c3 ret 
141417b5b: 0f28742420 movaps xmm6, xmmword ptr [rsp + 0x20]
141417b60: 4883c438 add rsp, 0x38
141417b64: 5b pop rbx
141417b65: 5f pop rdi
141417b66: 5e pop rsi
141417b67: 415c pop r12
141417b69: 415d pop r13
141417b6b: 415e pop r14
141417b6d: 415f pop r15
141417b6f: 5d pop rbp
141417b70: c3 ret 
141417b71: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
141417b80: 4889542410 mov qword ptr [rsp + 0x10], rdx
141417b85: 55 push rbp
141417b86: 4157 push r15
141417b88: 4156 push r14
141417b8a: 4155 push r13
141417b8c: 4154 push r12
141417b8e: 56 push rsi
141417b8f: 57 push rdi
141417b90: 53 push rbx
141417b91: 4883ec38 sub rsp, 0x38
141417b95: 488daa80000000 lea rbp, [rdx + 0x80]
141417b9c: 0f29742420 movaps xmmword ptr [rsp + 0x20], xmm6
141417ba1: 488b4520 mov rax, qword ptr [rbp + 0x20]
141417ba5: 48f7d8 neg rax
141417ba8: 7023 jo 0x141417bcd
141417baa: f6858800000001 test byte ptr [rbp + 0x88], 1
141417bb1: 741a je 0x141417bcd
141417bb3: 48837d2000 cmp qword ptr [rbp + 0x20], 0
141417bb8: 7413 je 0x141417bcd
141417bba: 41b801000000 mov r8d, 1
141417bc0: 488b4d50 mov rcx, qword ptr [rbp + 0x50]
141417bc4: 488b5520 mov rdx, qword ptr [rbp + 0x20]
141417bc8: e853c01fff call 0x140613c20 ; 
141417bcd: 0f28742420 movaps xmm6, xmmword ptr [rsp + 0x20]
141417bd2: 4883c438 add rsp, 0x38
141417bd6: 5b pop rbx
141417bd7: 5f pop rdi
141417bd8: 5e pop rsi
141417bd9: 415c pop r12
141417bdb: 415d pop r13
141417bdd: 415e pop r14
141417bdf: 415f pop r15
141417be1: 5d pop rbp
141417be2: c3 ret 
