1414062b0: 55 push rbp
1414062b1: 56 push rsi
1414062b2: 4881ec98000000 sub rsp, 0x98
1414062b9: 488dac2480000000 lea rbp, [rsp + 0x80]
1414062c1: 48c74510feffffff mov qword ptr [rbp + 0x10], 0xfffffffffffffffe
1414062c9: 4889ce mov rsi, rcx
1414062cc: 488955f0 mov qword ptr [rbp - 0x10], rdx
1414062d0: 4c8945f8 mov qword ptr [rbp - 8], r8
1414062d4: 48c7450000000000 mov qword ptr [rbp], 0
1414062dc: 48c745d800000000 mov qword ptr [rbp - 0x28], 0
1414062e4: 48c745e001000000 mov qword ptr [rbp - 0x20], 1
1414062ec: 48c745e800000000 mov qword ptr [rbp - 0x18], 0
1414062f4: c6450880 mov byte ptr [rbp + 8], 0x80
1414062f8: 488d4dc0 lea rcx, [rbp - 0x40]
1414062fc: 488d55d8 lea rdx, [rbp - 0x28]
141406300: e85bb10000 call 0x141411460 ; serde_json::de::impl$7::deserialize_seq<serde_json::read::StrRead,serde::de::impls::impl$19::deserialize::VecVisitor<country_core::startup::clearing::PlacementConfigEntry> >
141406305: 31c0 xor eax, eax
141406307: 483b45c0 cmp rax, qword ptr [rbp - 0x40]
14140630b: 7117 jno 0x141406324
14140630d: 488b45c8 mov rax, qword ptr [rbp - 0x38]
141406311: 48894608 mov qword ptr [rsi + 8], rax
141406315: 48b80000000000000080 movabs rax, 0x8000000000000000
14140631f: 488906 mov qword ptr [rsi], rax
141406322: eb71 jmp 0x141406395
141406324: 488b45d0 mov rax, qword ptr [rbp - 0x30]
141406328: 488945b0 mov qword ptr [rbp - 0x50], rax
14140632c: 0f1045c0 movups xmm0, xmmword ptr [rbp - 0x40]
141406330: 0f2945a0 movaps xmmword ptr [rbp - 0x60], xmm0
141406334: 488b45f8 mov rax, qword ptr [rbp - 8]
141406338: 488b4d00 mov rcx, qword ptr [rbp]
14140633c: 4839c1 cmp rcx, rax
14140633f: 7345 jae 0x141406386
141406341: 488b55f0 mov rdx, qword ptr [rbp - 0x10]
141406345: 48f7d8 neg rax
141406348: 48ffc1 inc rcx
14140634b: 49b80026000001000000 movabs r8, 0x100002600
141406355: 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
141406360: 440fb64c0aff movzx r9d, byte ptr [rdx + rcx - 1]
141406366: 4983f920 cmp r9, 0x20
14140636a: 774e ja 0x1414063ba
14140636c: 4d0fa3c8 bt r8, r9
141406370: 7348 jae 0x1414063ba
141406372: 48894d00 mov qword ptr [rbp], rcx
141406376: 4c8d0c08 lea r9, [rax + rcx]
14140637a: 49ffc1 inc r9
14140637d: 48ffc1 inc rcx
141406380: 4983f901 cmp r9, 1
141406384: 75da jne 0x141406360
141406386: 488b45b0 mov rax, qword ptr [rbp - 0x50]
14140638a: 48894610 mov qword ptr [rsi + 0x10], rax
14140638e: 0f2845a0 movaps xmm0, xmmword ptr [rbp - 0x60]
141406392: 0f1106 movups xmmword ptr [rsi], xmm0
141406395: 488b55d8 mov rdx, qword ptr [rbp - 0x28]
141406399: 4885d2 test rdx, rdx
14140639c: 740f je 0x1414063ad
14140639e: 488b4de0 mov rcx, qword ptr [rbp - 0x20]
1414063a2: 41b801000000 mov r8d, 1
1414063a8: e873d820ff call 0x140613c20 ; 
1414063ad: 4889f0 mov rax, rsi
1414063b0: 4881c498000000 add rsp, 0x98
1414063b7: 5e pop rsi
1414063b8: 5d pop rbp
1414063b9: c3 ret 
1414063ba: 48c745c013000000 mov qword ptr [rbp - 0x40], 0x13
1414063c2: 488d4dd8 lea rcx, [rbp - 0x28]
1414063c6: 488d55c0 lea rdx, [rbp - 0x40]
1414063ca: e851a54901 call 0x1428a0920 ; serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::SliceRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead> | serde_json::de::Deserializer::peek_error<serde_json::read::StrRead>
1414063cf: 48894608 mov qword ptr [rsi + 8], rax
1414063d3: 48b80000000000000080 movabs rax, 0x8000000000000000
1414063dd: 488906 mov qword ptr [rsi], rax
1414063e0: 488d4da0 lea rcx, [rbp - 0x60]
1414063e4: e8a72849ff call 0x140898c90 ; core::ptr::drop_in_place<alloc::vec::Vec<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global> > | core::ptr::drop_in_place<alloc::vec::Vec<sentry_types::protocol::attachment::Attachment,alloc::alloc::Global> >
1414063e9: ebaa jmp 0x141406395
1414063eb: 0f1f440000 nop dword ptr [rax + rax]
1414063f0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1414063f5: 55 push rbp
1414063f6: 56 push rsi
1414063f7: 4883ec28 sub rsp, 0x28
1414063fb: 488daa80000000 lea rbp, [rdx + 0x80]
141406402: 488b55d8 mov rdx, qword ptr [rbp - 0x28]
141406406: 4885d2 test rdx, rdx
141406409: 740f je 0x14140641a
14140640b: 488b4de0 mov rcx, qword ptr [rbp - 0x20]
14140640f: 41b801000000 mov r8d, 1
141406415: e806d820ff call 0x140613c20 ; 
14140641a: 90 nop 
14140641b: 4883c428 add rsp, 0x28
14140641f: 5e pop rsi
141406420: 5d pop rbp
141406421: c3 ret 
141406422: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
141406430: 4889542410 mov qword ptr [rsp + 0x10], rdx
141406435: 55 push rbp
141406436: 56 push rsi
141406437: 4883ec28 sub rsp, 0x28
14140643b: 488daa80000000 lea rbp, [rdx + 0x80]
141406442: 488d4da0 lea rcx, [rbp - 0x60]
141406446: e8452849ff call 0x140898c90 ; core::ptr::drop_in_place<alloc::vec::Vec<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global> > | core::ptr::drop_in_place<alloc::vec::Vec<sentry_types::protocol::attachment::Attachment,alloc::alloc::Global> >
14140644b: 90 nop 
14140644c: 4883c428 add rsp, 0x28
141406450: 5e pop rsi
141406451: 5d pop rbp
141406452: c3 ret 
