141470d00: push rbp
141470d01: push r15
141470d03: push r14
141470d05: push r13
141470d07: push r12
141470d09: push rsi
141470d0a: push rdi
141470d0b: push rbx
141470d0c: sub rsp, 0x468
141470d13: lea rbp, [rsp + 0x80]
141470d1b: movaps xmmword ptr [rbp + 0x3d0], xmm15
141470d23: movdqa xmmword ptr [rbp + 0x3c0], xmm14
141470d2c: movaps xmmword ptr [rbp + 0x3b0], xmm13
141470d34: movaps xmmword ptr [rbp + 0x3a0], xmm12
141470d3c: movaps xmmword ptr [rbp + 0x390], xmm11
141470d44: movaps xmmword ptr [rbp + 0x380], xmm10
141470d4c: movaps xmmword ptr [rbp + 0x370], xmm9
141470d54: movaps xmmword ptr [rbp + 0x360], xmm8
141470d5c: movaps xmmword ptr [rbp + 0x350], xmm7
141470d63: movaps xmmword ptr [rbp + 0x340], xmm6
141470d6a: mov qword ptr [rbp + 0x338], 0xfffffffffffffffe
141470d75: mov r14, r9
141470d78: mov rbx, r8
141470d7b: mov r12, rdx
141470d7e: mov rdi, rcx
141470d81: lea rcx, [rbp + 0x58]
141470d85: call 0x140d59140 ; std::fs::read_to_string::inner
141470d8a: mov rcx, qword ptr [rbp + 0x58]
141470d8e: mov rsi, qword ptr [rbp + 0x60]
141470d92: mov rax, rcx
141470d95: neg rax
141470d98: jno 0x141470e28
141470d9e: mov qword ptr [rbp + 0x250], rsi
141470da5: mov byte ptr [rbp + 0x32e], 1
141470dac: lea rcx, [rbp + 0x160]
141470db3: call 0x140d56700 ; std::backtrace::Backtrace::capture
141470db8: movups xmm0, xmmword ptr [rbp + 0x160]
141470dbf: movups xmm1, xmmword ptr [rbp + 0x170]
141470dc6: movups xmm2, xmmword ptr [rbp + 0x180]
141470dcd: movaps xmmword ptr [rbp + 0x290], xmm2
141470dd4: movaps xmmword ptr [rbp + 0x280], xmm1
141470ddb: movaps xmmword ptr [rbp + 0x270], xmm0
141470de2: lea rax, [rip + 0x17f368a]
141470de9: mov qword ptr [rbp + 0x160], rax
141470df0: mov qword ptr [rbp + 0x168], 0x11
141470dfb: mov qword ptr [rbp + 0x170], rsi
141470e02: mov byte ptr [rbp + 0x32e], 0
141470e09: lea rcx, [rbp + 0x160]
141470e10: lea rdx, [rbp + 0x270]
141470e17: call 0x1428a8e80 ; anyhow::Error::construct<anyhow::error::ContextError<ref$<str$>,std::io::error::Error> >
141470e1c: mov qword ptr [rdi], rax
141470e1f: mov byte ptr [rdi + 0xf], 2
141470e23: jmp 0x14147149a
141470e28: mov qword ptr [rbp + 0xf0], rcx
141470e2f: mov r8, qword ptr [rbp + 0x68]
141470e33: lea rcx, [rbp + 0x58]
141470e37: mov qword ptr [rbp + 0xf8], rsi
141470e3e: mov rdx, rsi
141470e41: call 0x141405930 ; serde_json::de::from_str<enum2$<serde_json::value::Value> >
141470e46: movzx eax, byte ptr [rbp + 0x58]
141470e4a: cmp al, 6
141470e4c: jne 0x141470ee0
141470e52: mov rsi, qword ptr [rbp + 0x60]
141470e56: mov qword ptr [rbp + 0x250], rsi
141470e5d: mov byte ptr [rbp + 0x32d], 1
141470e64: lea rcx, [rbp + 0x160]
141470e6b: call 0x140d56700 ; std::backtrace::Backtrace::capture
141470e70: movups xmm0, xmmword ptr [rbp + 0x160]
141470e77: movups xmm1, xmmword ptr [rbp + 0x170]
141470e7e: movups xmm2, xmmword ptr [rbp + 0x180]
141470e85: movaps xmmword ptr [rbp + 0x290], xmm2
141470e8c: movaps xmmword ptr [rbp + 0x280], xmm1
141470e93: movaps xmmword ptr [rbp + 0x270], xmm0
141470e9a: lea rax, [rip + 0x17f35e3]
141470ea1: mov qword ptr [rbp + 0x160], rax
141470ea8: mov qword ptr [rbp + 0x168], 0xc
141470eb3: mov qword ptr [rbp + 0x170], rsi
141470eba: mov byte ptr [rbp + 0x32d], 0
141470ec1: lea rcx, [rbp + 0x160]
141470ec8: lea rdx, [rbp + 0x270]
141470ecf: call 0x1428a9200 ; anyhow::Error::construct<anyhow::error::ContextError<ref$<str$>,serde_json::error::Error> >
141470ed4: mov qword ptr [rdi], rax
141470ed7: mov byte ptr [rdi + 0xf], 2
141470edb: jmp 0x14147147c
141470ee0: mov ecx, dword ptr [rbp + 0x59]
141470ee3: mov edx, dword ptr [rbp + 0x5c]
141470ee6: mov dword ptr [rbp + 0x13c], edx
141470eec: mov dword ptr [rbp + 0x139], ecx
141470ef2: mov rcx, qword ptr [rbp + 0x60]
141470ef6: movdqu xmm0, xmmword ptr [rbp + 0x68]
141470efb: movdqu xmmword ptr [rbp + 0x148], xmm0
141470f03: mov byte ptr [rbp + 0x138], al
141470f09: mov qword ptr [rbp + 0x140], rcx
141470f10: lea rcx, [rip + 0x17f3579]
141470f17: lea r8, [rbp + 0x138]
141470f1e: mov edx, 7
141470f23: call 0x1414744d0 ; serde_json::value::index::impl$1::index_into
141470f28: test rax, rax
141470f2b: jne 0x141470f34
141470f2d: mov rax, qword ptr [rip + 0x21bf824]
141470f34: movzx ecx, byte ptr [rax]
141470f37: lea rdx, [rip + 0x17f3dea]
141470f3e: movsxd rcx, dword ptr [rdx + rcx*4]
141470f42: add rcx, rdx
141470f45: jmp rcx
141470f47: mov byte ptr [rbp + 0x160], 0
141470f4e: jmp 0x141471001
141470f53: add rax, 8
141470f57: lea rcx, [rbp + 0x168]
141470f5e: lea r8, [rip + 0x17f2feb]
141470f65: mov rdx, rax
141470f68: call 0x1412f8270 ; alloc::vec::impl$11::clone<enum2$<serde_json::value::Value>,alloc::alloc::Global>
141470f6d: mov byte ptr [rbp + 0x160], 4
141470f74: jmp 0x141471001
141470f79: movdqu xmm0, xmmword ptr [rax + 8]
141470f7e: movdqu xmmword ptr [rbp + 0x168], xmm0
141470f86: mov byte ptr [rbp + 0x160], 2
141470f8d: jmp 0x141471001
141470f8f: add rax, 8
141470f93: lea rcx, [rbp + 0x168]
141470f9a: mov rdx, rax
141470f9d: call 0x140d7bd90 ; alloc::string::impl$6::clone
141470fa2: mov byte ptr [rbp + 0x160], 3
141470fa9: jmp 0x141471001
141470fab: movzx eax, byte ptr [rax + 1]
141470faf: mov byte ptr [rbp + 0x161], al
141470fb5: mov byte ptr [rbp + 0x160], 1
141470fbc: jmp 0x141471001
141470fbe: cmp qword ptr [rax + 0x18], 0
141470fc3: je 0x141470fe4
141470fc5: mov rdx, qword ptr [rax + 8]
141470fc9: test rdx, rdx
141470fcc: je 0x141473300
141470fd2: lea rcx, [rbp + 0x168]
141470fd9: mov r8, qword ptr [rax + 0x10]
141470fdd: call 0x14143c8c0 ; alloc::collections::btree::map::impl$2::clone::clone_subtree<alloc::string::String,enum2$<serde_json::value::Value>,alloc::alloc::Global>
141470fe2: jmp 0x141470ffa
141470fe4: mov qword ptr [rbp + 0x168], 0
141470fef: mov qword ptr [rbp + 0x178], 0
141470ffa: mov byte ptr [rbp + 0x160], 5
141471001: lea r15, [rbp + 0x270]
141471008: lea rdx, [rbp + 0x160]
14147100f: mov rcx, r15
141471012: call 0x14159a8f0 ; serde_json::value::de::impl$2::deserialize_i32<serde::de::impls::impl$54::deserialize::PrimitiveVisitor>
141471017: test byte ptr [rbp + 0x270], 1
14147101e: je 0x141471031
141471020: mov rcx, qword ptr [rbp + 0x278]
141471027: call 0x1428a9850 ; anyhow::error::impl$1::from<serde_json::error::Error>
14147102c: jmp 0x1414713f8
141471031: mov r13d, dword ptr [rbp + 0x274]
141471038: mov dword ptr [rbp + 0x300], r13d
14147103f: cmp r13d, 3
141471043: jge 0x1414710ce
141471049: lea rax, [rbp + 0x300]
141471050: mov qword ptr [rbp + 0x270], rax
141471057: lea rax, [rip - 0x6e3ade]
14147105e: mov qword ptr [rbp + 0x278], rax
141471065: lea rcx, [rip + 0x17f393c]
14147106c: mov qword ptr [rbp + 0x280], rcx
141471073: mov qword ptr [rbp + 0x288], rax
14147107a: lea rax, [rip + 0x17f3967]
141471081: mov qword ptr [rbp + 0x160], rax
141471088: mov qword ptr [rbp + 0x168], 2
141471093: mov qword ptr [rbp + 0x170], r15
14147109a: mov qword ptr [rbp + 0x178], 2
1414710a5: mov qword ptr [rbp + 0x180], 0
1414710b0: lea rcx, [rbp - 0x38]
1414710b4: lea rdx, [rbp + 0x160]
1414710bb: call 0x140d79f90 ; alloc::fmt::format::format_inner
1414710c0: lea rcx, [rbp - 0x38]
1414710c4: call 0x1428a83e0 ; anyhow::Error::msg<alloc::string::String>
1414710c9: jmp 0x1414713f8
1414710ce: cmp r13d, 0x1b
1414710d2: jbe 0x14147115d
1414710d8: lea rax, [rbp + 0x300]
1414710df: mov qword ptr [rbp + 0x270], rax
1414710e6: lea rax, [rip - 0x6e3b6d]
1414710ed: mov qword ptr [rbp + 0x278], rax
1414710f4: lea rcx, [rip + 0x17f34e5]
1414710fb: mov qword ptr [rbp + 0x280], rcx
141471102: mov qword ptr [rbp + 0x288], rax
141471109: lea rax, [rip + 0x17f3878]
141471110: mov qword ptr [rbp + 0x160], rax
141471117: mov qword ptr [rbp + 0x168], 2
141471122: mov qword ptr [rbp + 0x170], r15
141471129: mov qword ptr [rbp + 0x178], 2
141471134: mov qword ptr [rbp + 0x180], 0
14147113f: lea rcx, [rbp - 0x50]
141471143: lea rdx, [rbp + 0x160]
14147114a: call 0x140d79f90 ; alloc::fmt::format::format_inner
14147114f: lea rcx, [rbp - 0x50]
141471153: call 0x1428a83e0 ; anyhow::Error::msg<alloc::string::String>
141471158: jmp 0x1414713f8
14147115d: lea rcx, [rip + 0x17f3333]
141471164: lea r8, [rbp + 0x138]
14147116b: mov edx, 0x15
141471170: call 0x1414744d0 ; serde_json::value::index::impl$1::index_into
141471175: test rax, rax
141471178: jne 0x141471181
14147117a: mov rax, qword ptr [rip + 0x21bf5d7]
141471181: movzx ecx, byte ptr [rax]
141471184: lea rdx, [rip + 0x17f3bb5]
14147118b: movsxd rcx, dword ptr [rdx + rcx*4]
14147118f: add rcx, rdx
141471192: jmp rcx
141471194: mov byte ptr [rbp + 0x160], 0
14147119b: jmp 0x141471222
1414711a0: add rax, 8
1414711a4: lea rcx, [rbp + 0x168]
1414711ab: lea r8, [rip + 0x17f2d9e]
1414711b2: mov rdx, rax
1414711b5: call 0x1412f8270 ; alloc::vec::impl$11::clone<enum2$<serde_json::value::Value>,alloc::alloc::Global>
1414711ba: mov byte ptr [rbp + 0x160], 4
1414711c1: jmp 0x141471222
1414711c3: movdqu xmm0, xmmword ptr [rax + 8]
1414711c8: movdqu xmmword ptr [rbp + 0x168], xmm0
1414711d0: mov byte ptr [rbp + 0x160], 2
1414711d7: jmp 0x141471222
1414711d9: add rax, 8
1414711dd: lea rcx, [rbp + 0x168]
1414711e4: mov rdx, rax
1414711e7: call 0x140d7bd90 ; alloc::string::impl$6::clone
1414711ec: mov byte ptr [rbp + 0x160], 3
1414711f3: jmp 0x141471222
1414711f5: movzx eax, byte ptr [rax + 1]
1414711f9: mov byte ptr [rbp + 0x161], al
1414711ff: mov byte ptr [rbp + 0x160], 1
141471206: jmp 0x141471222
141471208: add rax, 8
14147120c: lea rcx, [rbp + 0x168]
141471213: mov rdx, rax
141471216: call 0x14143d1c0 ; alloc::collections::btree::map::impl$2::clone<alloc::string::String,enum2$<serde_json::value::Value>,alloc::alloc::Global>
14147121b: mov byte ptr [rbp + 0x160], 5
141471222: lea rcx, [rbp + 0x270]
141471229: lea rdx, [rbp + 0x160]
141471230: call 0x14159a8f0 ; serde_json::value::de::impl$2::deserialize_i32<serde::de::impls::impl$54::deserialize::PrimitiveVisitor>
141471235: test byte ptr [rbp + 0x270], 1
14147123c: je 0x141471265
14147123e: mov rax, qword ptr [rbp + 0x278]
141471245: lea rcx, [rbp + 0x60]
141471249: mov qword ptr [rbp + 0x60], rax
14147124d: mov dword ptr [rbp + 0x58], 1
141471254: mov dword ptr [rbp + 0x11c], 0x19
14147125e: call 0x1405e88a0 ; core::ptr::drop_in_place<serde_json::error::Error>
141471263: jmp 0x141471271
141471265: mov eax, dword ptr [rbp + 0x274]
14147126b: mov dword ptr [rbp + 0x11c], eax
141471271: lea rcx, [rip + 0x17f3234]
141471278: lea r8, [rbp + 0x138]
14147127f: mov edx, 6
141471284: call 0x1414744d0 ; serde_json::value::index::impl$1::index_into
141471289: test rax, rax
14147128c: jne 0x141471295
14147128e: mov rax, qword ptr [rip + 0x21bf4c3]
141471295: movzx ecx, byte ptr [rax]
141471298: lea rdx, [rip + 0x17f3ab9]
14147129f: movsxd rcx, dword ptr [rdx + rcx*4]
1414712a3: add rcx, rdx
1414712a6: jmp rcx
1414712a8: mov byte ptr [rbp + 0x160], 0
1414712af: jmp 0x141471336
1414712b4: add rax, 8
1414712b8: lea rcx, [rbp + 0x168]
1414712bf: lea r8, [rip + 0x17f2c8a]
1414712c6: mov rdx, rax
1414712c9: call 0x1412f8270 ; alloc::vec::impl$11::clone<enum2$<serde_json::value::Value>,alloc::alloc::Global>
1414712ce: mov byte ptr [rbp + 0x160], 4
1414712d5: jmp 0x141471336
1414712d7: movdqu xmm0, xmmword ptr [rax + 8]
1414712dc: movdqu xmmword ptr [rbp + 0x168], xmm0
1414712e4: mov byte ptr [rbp + 0x160], 2
1414712eb: jmp 0x141471336
1414712ed: add rax, 8
1414712f1: lea rcx, [rbp + 0x168]
1414712f8: mov rdx, rax
1414712fb: call 0x140d7bd90 ; alloc::string::impl$6::clone
141471300: mov byte ptr [rbp + 0x160], 3
141471307: jmp 0x141471336
141471309: movzx eax, byte ptr [rax + 1]
14147130d: mov byte ptr [rbp + 0x161], al
141471313: mov byte ptr [rbp + 0x160], 1
14147131a: jmp 0x141471336
14147131c: add rax, 8
141471320: lea rcx, [rbp + 0x168]
141471327: mov rdx, rax
14147132a: call 0x14143d1c0 ; alloc::collections::btree::map::impl$2::clone<alloc::string::String,enum2$<serde_json::value::Value>,alloc::alloc::Global>
14147132f: mov byte ptr [rbp + 0x160], 5
141471336: lea rax, [rip + 0x1820efb]
14147133d: mov qword ptr [rsp + 0x20], rax
141471342: mov qword ptr [rsp + 0x28], 9
14147134b: lea r8, [rip + 0x1820e74]
141471352: lea rcx, [rbp + 0x58]
141471356: lea rdx, [rbp + 0x160]
14147135d: mov r9d, 0x18
141471363: call 0x1415a1530 ; serde_json::value::de::impl$2::deserialize_struct<country_core::systems::camera_rig::_::impl$0::deserialize::__Visitor>
141471368: cmp dword ptr [rbp + 0x58], 1
14147136c: jne 0x1414714ff
141471372: mov rsi, qword ptr [rbp + 0x60]
141471376: mov qword ptr [rbp + 0x250], rsi
14147137d: mov byte ptr [rbp + 0x32c], 1
141471384: lea rcx, [rbp + 0x160]
14147138b: call 0x140d56700 ; std::backtrace::Backtrace::capture
141471390: movdqu xmm0, xmmword ptr [rbp + 0x160]
141471398: movdqu xmm1, xmmword ptr [rbp + 0x170]
1414713a0: movups xmm2, xmmword ptr [rbp + 0x180]
1414713a7: movaps xmmword ptr [rbp + 0x290], xmm2
1414713ae: movdqa xmmword ptr [rbp + 0x280], xmm1
1414713b6: movdqa xmmword ptr [rbp + 0x270], xmm0
1414713be: lea rax, [rip + 0x17f30ed]
1414713c5: mov qword ptr [rbp + 0x160], rax
1414713cc: mov qword ptr [rbp + 0x168], 0xe
1414713d7: mov qword ptr [rbp + 0x170], rsi
1414713de: mov byte ptr [rbp + 0x32c], 0
1414713e5: lea rcx, [rbp + 0x160]
1414713ec: lea rdx, [rbp + 0x270]
1414713f3: call 0x1428a9200 ; anyhow::Error::construct<anyhow::error::ContextError<ref$<str$>,serde_json::error::Error> >
1414713f8: mov qword ptr [rdi], rax
1414713fb: mov byte ptr [rdi + 0xf], 2
1414713ff: movzx eax, byte ptr [rbp + 0x138]
141471406: cmp eax, 3
141471409: jb 0x14147147c
14147140b: je 0x141471450
14147140d: cmp eax, 4
141471410: jne 0x141471470
141471412: mov rcx, qword ptr [rbp + 0x148]
141471419: mov rdx, qword ptr [rbp + 0x150]
141471420: mov qword ptr [rbp + 0x320], rcx
141471427: call 0x140888d70 ; core::ptr::drop_in_place<slice2$<enum2$<serde_json::value::Value> > >
14147142c: mov rdx, qword ptr [rbp + 0x140]
141471433: test rdx, rdx
141471436: je 0x14147147c
141471438: shl rdx, 5
14147143c: mov r8d, 8
141471442: mov rcx, qword ptr [rbp + 0x320]
141471449: call 0x140613c20 ; 0x140613c20
14147144e: jmp 0x14147147c
141471450: mov rdx, qword ptr [rbp + 0x140]
141471457: test rdx, rdx
14147145a: je 0x14147147c
14147145c: mov rcx, qword ptr [rbp + 0x148]
141471463: mov r8d, 1
141471469: call 0x140613c20 ; 0x140613c20
14147146e: jmp 0x14147147c
141471470: lea rcx, [rbp + 0x140]
141471477: call 0x140898f40 ; core::ptr::drop_in_place<serde_json::map::Map<alloc::string::String,enum2$<serde_json::value::Value> > >
14147147c: mov rdx, qword ptr [rbp + 0xf0]
141471483: test rdx, rdx
141471486: je 0x14147149a
141471488: mov r8d, 1
14147148e: mov rcx, qword ptr [rbp + 0xf8]
141471495: call 0x140613c20 ; 0x140613c20
14147149a: mov rax, rdi
14147149d: movaps xmm6, xmmword ptr [rbp + 0x340]
1414714a4: movaps xmm7, xmmword ptr [rbp + 0x350]
1414714ab: movaps xmm8, xmmword ptr [rbp + 0x360]
1414714b3: movaps xmm9, xmmword ptr [rbp + 0x370]
1414714bb: movaps xmm10, xmmword ptr [rbp + 0x380]
1414714c3: movaps xmm11, xmmword ptr [rbp + 0x390]
1414714cb: movaps xmm12, xmmword ptr [rbp + 0x3a0]
1414714d3: movaps xmm13, xmmword ptr [rbp + 0x3b0]
1414714db: movaps xmm14, xmmword ptr [rbp + 0x3c0]
1414714e3: movaps xmm15, xmmword ptr [rbp + 0x3d0]
1414714eb: add rsp, 0x468
1414714f2: pop rbx
1414714f3: pop rdi
1414714f4: pop rsi
1414714f5: pop r12
1414714f7: pop r13
1414714f9: pop r14
1414714fb: pop r15
1414714fd: pop rbp
1414714fe: ret 
1414714ff: movss xmm11, dword ptr [rbp + 0x5c]
141471505: mov rax, qword ptr [rbp + 0x60]
141471509: mov qword ptr [rbp + 0x18], rax
14147150d: mov rax, qword ptr [rbp + 0x68]
141471511: mov qword ptr [rbp + 0x10], rax
141471515: movups xmm10, xmmword ptr [rbp + 0x70]
14147151a: lea rcx, [rip + 0x17f2f9f]
141471521: lea r8, [rbp + 0x138]
141471528: mov edx, 0xb
14147152d: call 0x1414744d0 ; serde_json::value::index::impl$1::index_into
141471532: test rax, rax
141471535: jne 0x14147153e
141471537: mov rax, qword ptr [rip + 0x21bf21a]
14147153e: lea rcx, [rbp + 0x270]
141471545: mov rdx, rax
141471548: call 0x14146cf00 ; serde_json::value::impl$4::clone
14147154d: lea rax, [rip + 0x167cd7c]
141471554: mov qword ptr [rsp + 0x20], rax
141471559: mov qword ptr [rsp + 0x28], 3
141471562: lea r8, [rip + 0x167cd2f]
141471569: lea rcx, [rbp + 0x160]
141471570: lea rdx, [rbp + 0x270]
141471577: mov r9d, 9
14147157d: call 0x14159f730 ; serde_json::value::de::impl$2::deserialize_struct<country_core::resources::time_of_day::_::impl$0::deserialize::__Visitor>
141471582: mov eax, dword ptr [rbp + 0x160]
141471588: test eax, eax
14147158a: je 0x1414716c1
141471590: lea rcx, [rbp + 0x168]
141471597: call 0x1405e88a0 ; core::ptr::drop_in_place<serde_json::error::Error>
14147159c: lea rcx, [rip + 0x17f2f1d]
1414715a3: lea r8, [rbp + 0x138]
1414715aa: mov edx, 0xb
1414715af: call 0x1414744d0 ; serde_json::value::index::impl$1::index_into
1414715b4: test rax, rax
1414715b7: jne 0x1414715c0
1414715b9: mov rax, qword ptr [rip + 0x21bf198]
1414715c0: lea rcx, [rbp + 0x160]
1414715c7: mov rdx, rax
1414715ca: call 0x14146cf00 ; serde_json::value::impl$4::clone
1414715cf: lea rcx, [rbp + 0x270]
1414715d6: lea rdx, [rbp + 0x160]
1414715dd: call 0x14159a370 ; serde_json::value::de::impl$2::deserialize_f32<serde::de::impls::impl$76::deserialize::PrimitiveVisitor>
1414715e2: cmp dword ptr [rbp + 0x270], 0
1414715e9: je 0x141471704
1414715ef: lea rcx, [rbp + 0x278]
1414715f6: call 0x1405e88a0 ; core::ptr::drop_in_place<serde_json::error::Error>
1414715fb: mov rax, qword ptr [rip + 0x21bada6]
141471602: mov rax, qword ptr [rax]
141471605: movaps xmm12, xmmword ptr [rip + 0x17f26e3]
14147160d: movaps xmm8, xmmword ptr [rip + 0x17f26eb]
141471615: mov dword ptr [rbp + 0x2f4], 0
14147161f: xorps xmm13, xmm13
141471623: mov word ptr [rbp + 0x316], 0xcf
14147162c: cmp rax, 2
141471630: jb 0x141471743
141471636: lea rax, [rip + 0x17f2ebb]
14147163d: mov qword ptr [rbp + 0x160], rax
141471644: mov qword ptr [rbp + 0x168], 1
14147164f: mov qword ptr [rbp + 0x170], 8
14147165a: pxor xmm0, xmm0
14147165e: movdqu xmmword ptr [rbp + 0x178], xmm0
141471666: lea rcx, [rip + 0x17f2e9b]
14147166d: call 0x14067dd80 ; toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut
141471672: lea rcx, [rip + 0x17f2acf]
141471679: mov qword ptr [rbp + 0x270], rcx
141471680: mov qword ptr [rbp + 0x278], 0x26
14147168b: mov qword ptr [rbp + 0x280], rcx
141471692: mov qword ptr [rbp + 0x288], 0x26
14147169d: mov qword ptr [rbp + 0x290], rax
1414716a4: lea rcx, [rbp + 0x160]
1414716ab: lea r8, [rbp + 0x270]
1414716b2: mov edx, 2
1414716b7: call 0x1404b1430 ; log::__private_api::log<tuple$<>,log::__private_api::GlobalLogger>
1414716bc: jmp 0x141471743
1414716c1: mov eax, dword ptr [rbp + 0x164]
1414716c7: mov dword ptr [rbp + 0x2f4], eax
1414716cd: movups xmm8, xmmword ptr [rbp + 0x168]
1414716d5: movups xmm12, xmmword ptr [rbp + 0x178]
1414716dd: movss xmm13, dword ptr [rbp + 0x188]
1414716e6: movzx eax, word ptr [rbp + 0x18c]
1414716ed: mov word ptr [rbp + 0x316], ax
1414716f4: je 0x141471743
1414716f6: lea rcx, [rbp + 0x168]
1414716fd: call 0x1405e88a0 ; core::ptr::drop_in_place<serde_json::error::Error>
141471702: jmp 0x141471743
141471704: movss xmm8, dword ptr [rbp + 0x274]
14147170d: je 0x14147171b
14147170f: lea rcx, [rbp + 0x278]
141471716: call 0x1405e88a0 ; core::ptr::drop_in_place<serde_json::error::Error>
14147171b: shufps xmm8, xmmword ptr [rip + 0x17f25bc], 0xe2
141471724: movaps xmm12, xmmword ptr [rip + 0x17f25c4]
14147172c: mov dword ptr [rbp + 0x2f4], 0
141471736: xorps xmm13, xmm13
14147173a: mov word ptr [rbp + 0x316], 0xcf
141471743: lea rcx, [rip + 0x17f2dd6]
14147174a: lea r8, [rbp + 0x138]
141471751: mov edx, 5
141471756: call 0x1414744d0 ; serde_json::value::index::impl$1::index_into
14147175b: test rax, rax
14147175e: jne 0x141471767
141471760: mov rax, qword ptr [rip + 0x21beff1]
141471767: lea rcx, [rbp + 0x160]
14147176e: mov rdx, rax
141471771: call 0x14146cf00 ; serde_json::value::impl$4::clone
141471776: lea rax, [rip + 0x182083b]
14147177d: mov qword ptr [rsp + 0x20], rax
141471782: mov qword ptr [rsp + 0x28], 1
14147178b: lea r8, [rip + 0x1820806]
141471792: lea rcx, [rbp + 0x270]
141471799: lea rdx, [rbp + 0x160]
1414717a0: mov r9d, 0x14
1414717a6: call 0x1415a2140 ; serde_json::value::de::impl$2::deserialize_struct<country_core::resources::critters::sheep::control::_::impl$0::deserialize::__Visitor>
1414717ab: lea rcx, [rbp + 0x160]
1414717b2: lea rdx, [rbp + 0x270]
1414717b9: call 0x14146cbb0 ; core::result::Result::inspect_err<country_core::resources::critters::sheep::control::SheepControlSnapshot,serde_json::error::Error,country_core::systems::save::load_game::parse_saved_game_data::closure_env$0>
1414717be: xor eax, eax
1414717c0: cmp rax, qword ptr [rbp + 0x160]
1414717c7: jno 0x1414717e5
1414717c9: movabs rax, 0x8000000000000000
1414717d3: mov qword ptr [rbp + 0x40], rax
1414717d7: lea rcx, [rbp + 0x168]
1414717de: call 0x1405e88a0 ; core::ptr::drop_in_place<serde_json::error::Error>
1414717e3: jmp 0x1414717fd
1414717e5: mov rax, qword ptr [rbp + 0x170]
1414717ec: mov qword ptr [rbp + 0x50], rax
1414717f0: movdqu xmm0, xmmword ptr [rbp + 0x160]
1414717f8: movdqa xmmword ptr [rbp + 0x40], xmm0
1414717fd: lea rcx, [rip + 0x17f2d21]
141471804: lea r8, [rbp + 0x138]
14147180b: mov edx, 6
141471810: call 0x1414744d0 ; serde_json::value::index::impl$1::index_into
141471815: test rax, rax
141471818: jne 0x141471821
14147181a: mov rax, qword ptr [rip + 0x21bef37]
141471821: lea rcx, [rbp + 0x270]
141471828: mov rdx, rax
14147182b: call 0x14146cf00 ; serde_json::value::impl$4::clone
141471830: lea rax, [rip + 0x1623f71]
141471837: mov qword ptr [rsp + 0x20], rax
14147183c: mov qword ptr [rsp + 0x28], 0xf
141471845: lea r8, [rip + 0x1623eac]
14147184c: lea rcx, [rbp + 0x160]
141471853: lea rdx, [rbp + 0x270]
14147185a: mov r9d, 0x17
141471860: call 0x1415a0130 ; serde_json::value::de::impl$2::deserialize_struct<country_core::resources::glade_color_customization::_::impl$0::deserialize::__Visitor>
141471865: mov edx, dword ptr [rbp + 0x160]
14147186b: lea rcx, [rbp + 0x168]
141471872: cmp edx, 1
141471875: jne 0x141471981
14147187b: call 0x1405e88a0 ; core::ptr::drop_in_place<serde_json::error::Error>
141471880: mov rax, qword ptr [rip + 0x21bab21]
141471887: mov rax, qword ptr [rax]
14147188a: cmp rax, 3
14147188e: jb 0x14147191a
141471894: lea rax, [rip + 0x17f2d05]
14147189b: mov qword ptr [rbp + 0x160], rax
1414718a2: mov qword ptr [rbp + 0x168], 1
1414718ad: mov qword ptr [rbp + 0x170], 8
1414718b8: pxor xmm0, xmm0
1414718bc: movdqu xmmword ptr [rbp + 0x178], xmm0
1414718c4: lea rcx, [rip + 0x17f2ce5]
1414718cb: call 0x14067dd80 ; toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut
1414718d0: lea rcx, [rip + 0x17f2871]
1414718d7: mov qword ptr [rbp + 0x270], rcx
1414718de: mov qword ptr [rbp + 0x278], 0x26
1414718e9: mov qword ptr [rbp + 0x280], rcx
1414718f0: mov qword ptr [rbp + 0x288], 0x26
1414718fb: mov qword ptr [rbp + 0x290], rax
141471902: lea rcx, [rbp + 0x160]
141471909: lea r8, [rbp + 0x270]
141471910: mov edx, 3
141471915: call 0x1404b1430 ; log::__private_api::log<tuple$<>,log::__private_api::GlobalLogger>
14147191a: mov eax, dword ptr [r14 + 0x14]
14147191e: mov dword ptr [rbp + 0x120], eax
141471924: movd xmm0, dword ptr [r14 + 0x18]
14147192a: movd xmm1, dword ptr [r14 + 0x10]
141471930: punpckldq xmm1, xmm0
141471934: movq xmm14, xmm1
141471939: movaps xmm7, xmmword ptr [rip + 0x17f23d0]
141471940: xorps xmm9, xmm9
141471944: mov byte ptr [rbp + 0x32f], 1
14147194b: movss xmm15, dword ptr [rip + 0x14b3fb0]
141471954: mov dword ptr [rbp + 0x124], 0
14147195e: mov dword ptr [rbp + 0x128], 0
141471968: mov dword ptr [rbp + 0x12c], 0
141471972: mov dword ptr [rbp + 0x130], 0
14147197c: jmp 0x141471a92
141471981: movups xmm7, xmmword ptr [rbp + 0x164]
141471988: mov eax, dword ptr [rbp + 0x174]
14147198e: mov dword ptr [rbp + 0x124], eax
141471994: movzx eax, byte ptr [rbp + 0x178]
14147199b: mov dword ptr [rbp + 0x128], eax
1414719a1: mov eax, dword ptr [rbp + 0x17c]
1414719a7: mov dword ptr [rbp + 0x120], eax
1414719ad: movdqu xmm14, xmmword ptr [rbp + 0x180]
1414719b6: movups xmm9, xmmword ptr [rbp + 0x190]
1414719be: movss xmm15, dword ptr [rbp + 0x1a0]
1414719c7: movzx eax, byte ptr [rbp + 0x1a4]
1414719ce: mov dword ptr [rbp + 0x12c], eax
1414719d4: movzx eax, byte ptr [rbp + 0x1a5]
1414719db: mov dword ptr [rbp + 0x130], eax
1414719e1: movzx eax, byte ptr [rbp + 0x1a6]
1414719e8: mov byte ptr [rbp + 0x32f], al
1414719ee: mov rax, qword ptr [rip + 0x21ba9b3]
1414719f5: mov rax, qword ptr [rax]
1414719f8: cmp rax, 3
1414719fc: jb 0x141471a89
141471a02: mov dword ptr [rbp + 0x320], edx
141471a08: lea rax, [rip + 0x17f2b41]
141471a0f: mov qword ptr [rbp + 0x270], rax
141471a16: mov qword ptr [rbp + 0x278], 1
141471a21: mov qword ptr [rbp + 0x280], 8
141471a2c: pxor xmm0, xmm0
141471a30: movdqu xmmword ptr [rbp + 0x288], xmm0
141471a38: lea rcx, [rip + 0x17f2b21]
141471a3f: call 0x14067dd80 ; toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut
141471a44: lea rcx, [rip + 0x17f26fd]
141471a4b: mov qword ptr [rbp + 0x58], rcx
141471a4f: mov qword ptr [rbp + 0x60], 0x26
141471a57: mov qword ptr [rbp + 0x68], rcx
141471a5b: mov qword ptr [rbp + 0x70], 0x26
141471a63: mov qword ptr [rbp + 0x78], rax
141471a67: lea rcx, [rbp + 0x270]
141471a6e: lea r8, [rbp + 0x58]
141471a72: mov edx, 3
141471a77: call 0x1404b1430 ; log::__private_api::log<tuple$<>,log::__private_api::GlobalLogger>
141471a7c: lea rcx, [rbp + 0x168]
141471a83: mov edx, dword ptr [rbp + 0x320]
141471a89: test edx, edx
141471a8b: je 0x141471a92
141471a8d: call 0x1405e88a0 ; core::ptr::drop_in_place<serde_json::error::Error>
141471a92: lea rcx, [rip + 0x17f2b2f]
141471a99: lea r8, [rbp + 0x138]
141471aa0: mov edx, 7
141471aa5: call 0x1414744d0 ; serde_json::value::index::impl$1::index_into
141471aaa: test rax, rax
141471aad: je 0x141471aba
141471aaf: movzx ecx, byte ptr [rax]
141471ab2: mov byte ptr [rax], 0
141471ab5: cmp cl, 6
141471ab8: jne 0x141471ac3
141471aba: mov byte ptr [rbp + 0x250], 0
141471ac1: jmp 0x141471ae3
141471ac3: mov byte ptr [rbp + 0x250], cl
141471ac9: movdqu xmm0, xmmword ptr [rax + 1]
141471ace: movdqu xmm1, xmmword ptr [rax + 0x10]
141471ad3: movdqu xmmword ptr [rbp + 0x251], xmm0
141471adb: movdqu xmmword ptr [rbp + 0x260], xmm1
141471ae3: lea rax, [rip + 0x167c566]
141471aea: mov qword ptr [rsp + 0x20], rax
141471aef: mov qword ptr [rsp + 0x28], 4
141471af8: lea r8, [rip + 0x167c519]
141471aff: lea rcx, [rbp + 0x160]
141471b06: lea r14, [rbp + 0x250]
141471b0d: mov r9d, 7
141471b13: mov rdx, r14
141471b16: call 0x1415a0f30 ; serde_json::value::de::impl$2::deserialize_struct<country_core::resources::history::_::impl$0::deserialize::__Visitor>
141471b1b: lea r8, [rip + 0x17f2aad]
141471b22: lea rcx, [rbp + 0x270]
141471b29: lea rdx, [rbp + 0x160]
141471b30: mov r9d, 0xf
141471b36: call 0x1415a5cd0 ; anyhow::context::impl$0::context<country_core::resources::history::History,serde_json::error::Error,ref$<str$> >
141471b3b: mov rax, qword ptr [rbp + 0x270]
141471b42: mov rdx, qword ptr [rbp + 0x278]
141471b49: mov rcx, rax
141471b4c: neg rcx
141471b4f: jno 0x141471b8a
141471b51: mov qword ptr [rdi], rdx
141471b54: mov byte ptr [rdi + 0xf], 2
141471b58: mov rax, qword ptr [rbp + 0x40]
141471b5c: mov rcx, rax
141471b5f: neg rcx
141471b62: jo 0x1414713ff
141471b68: jae 0x1414713ff
141471b6e: mov rcx, qword ptr [rbp + 0x48]
141471b72: shl rax, 3
141471b76: lea rdx, [rax + rax*4]
141471b7a: mov r8d, 4
141471b80: call 0x140613c20 ; 0x140613c20
141471b85: jmp 0x1414713ff
141471b8a: mov qword ptr [rbp + 0x28], rbx
141471b8e: mov rcx, qword ptr [rbp + 0x2e0]
141471b95: mov qword ptr [rbp + 0xc8], rcx
141471b9c: movups xmm0, xmmword ptr [rbp + 0x2d0]
141471ba3: movups xmmword ptr [rbp + 0xb8], xmm0
141471baa: movups xmm0, xmmword ptr [rbp + 0x2c0]
141471bb1: movups xmmword ptr [rbp + 0xa8], xmm0
141471bb8: movups xmm0, xmmword ptr [rbp + 0x280]
141471bbf: movups xmm1, xmmword ptr [rbp + 0x290]
141471bc6: movups xmm2, xmmword ptr [rbp + 0x2a0]
141471bcd: movups xmm3, xmmword ptr [rbp + 0x2b0]
141471bd4: movups xmmword ptr [rbp + 0x98], xmm3
141471bdb: movups xmmword ptr [rbp + 0x88], xmm2
141471be2: movups xmmword ptr [rbp + 0x78], xmm1
141471be6: movups xmmword ptr [rbp + 0x68], xmm0
141471bea: mov qword ptr [rbp + 0x58], rax
141471bee: mov qword ptr [rbp + 0x60], rdx
141471bf2: mov eax, dword ptr [rbp + 0xc0]
141471bf8: mov dword ptr [rbp + 0xd4], eax
141471bfe: mov byte ptr [rbp + 0x336], 0
141471c05: mov dword ptr [rbp + 0x2f0], 0
141471c0f: mov eax, 0
141471c14: mov ebx, 0
141471c19: cmp r13d, 0x1b
141471c1d: jne 0x141471d3c
141471c23: mov dword ptr [rbp + 0x304], eax
141471c29: mov rax, qword ptr [rip + 0x21ba778]
141471c30: mov rax, qword ptr [rax]
141471c33: cmp rax, 3
141471c37: jb 0x141471cc1
141471c3d: lea rax, [rip + 0x17f2af4]
141471c44: mov qword ptr [rbp + 0x160], rax
141471c4b: mov qword ptr [rbp + 0x168], 1
141471c56: mov qword ptr [rbp + 0x170], 8
141471c61: xorps xmm0, xmm0
141471c64: movups xmmword ptr [rbp + 0x178], xmm0
141471c6b: lea rcx, [rip + 0x17f2ad6]
141471c72: call 0x14067dd80 ; toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut
141471c77: lea rcx, [rip + 0x17f24ca]
141471c7e: mov qword ptr [rbp + 0x270], rcx
141471c85: mov qword ptr [rbp + 0x278], 0x26
141471c90: mov qword ptr [rbp + 0x280], rcx
141471c97: mov qword ptr [rbp + 0x288], 0x26
141471ca2: mov qword ptr [rbp + 0x290], rax
141471ca9: lea rcx, [rbp + 0x160]
141471cb0: lea r8, [rbp + 0x270]
141471cb7: mov edx, 3
141471cbc: call 0x1404b1430 ; log::__private_api::log<tuple$<>,log::__private_api::GlobalLogger>
141471cc1: lea rcx, [rip + 0x17f2b00]
141471cc8: lea r8, [rbp + 0x138]
141471ccf: mov edx, 8
141471cd4: call 0x1414744d0 ; serde_json::value::index::impl$1::index_into
141471cd9: mov r15, rax
141471cdc: lea rax, [rip + 0x17f2afd]
141471ce3: mov qword ptr [rbp + 0x160], rax
141471cea: mov qword ptr [rbp + 0x168], 1
141471cf5: mov qword ptr [rbp + 0x170], 8
141471d00: xorps xmm0, xmm0
141471d03: movups xmmword ptr [rbp + 0x178], xmm0
141471d0a: lea rcx, [rbp + 0x160]
141471d11: call 0x1428a8020 ; anyhow::__private::format_err
141471d16: mov qword ptr [rbp + 0x160], rax
141471d1d: test r15, r15
141471d20: je 0x141472ade
141471d26: lea rcx, [rbp + 0x160]
141471d2d: call 0x140d4cc30 ; anyhow::error::impl$6::drop
141471d32: xor ecx, ecx
141471d34: mov rax, r15
141471d37: jmp 0x141472ae3
141471d3c: mov rax, qword ptr [rip + 0x21ba665]
141471d43: mov rax, qword ptr [rax]
141471d46: cmp rax, 3
141471d4a: jb 0x141471e11
141471d50: lea rax, [rbp + 0x300]
141471d57: mov qword ptr [rbp + 0x250], rax
141471d5e: lea rax, [rip - 0x6e47e5]
141471d65: mov qword ptr [rbp + 0x258], rax
141471d6c: lea rcx, [rip + 0x17f286d]
141471d73: mov qword ptr [rbp + 0x260], rcx
141471d7a: mov qword ptr [rbp + 0x268], rax
141471d81: lea rax, [rip + 0x17f28c0]
141471d88: mov qword ptr [rbp + 0x160], rax
141471d8f: mov qword ptr [rbp + 0x168], 3
141471d9a: mov qword ptr [rbp + 0x180], 0
141471da5: mov qword ptr [rbp + 0x170], r14
141471dac: mov qword ptr [rbp + 0x178], 2
141471db7: lea rcx, [rip + 0x17f28ba]
141471dbe: call 0x14067dd80 ; toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut
141471dc3: lea rcx, [rip + 0x17f237e]
141471dca: mov qword ptr [rbp + 0x270], rcx
141471dd1: mov qword ptr [rbp + 0x278], 0x26
141471ddc: mov qword ptr [rbp + 0x280], rcx
141471de3: mov qword ptr [rbp + 0x288], 0x26
141471dee: mov qword ptr [rbp + 0x290], rax
141471df5: lea rcx, [rbp + 0x160]
141471dfc: lea r8, [rbp + 0x270]
141471e03: mov edx, 3
141471e08: call 0x1404b1430 ; log::__private_api::log<tuple$<>,log::__private_api::GlobalLogger>
141471e0d: mov rdx, qword ptr [rbp + 0x60]
141471e11: movss dword ptr [rbp + 0x134], xmm15
141471e1a: movaps xmmword ptr [rbp - 0x10], xmm9
141471e1f: movaps xmmword ptr [rbp - 0x20], xmm7
141471e23: mov qword ptr [rbp + 0x38], rdi
141471e27: cmp r13d, 0x10
141471e2b: setb byte ptr [rbp + 0x336]
141471e32: mov byte ptr [rbp + 0xd8], 1
141471e39: mov qword ptr [rbp + 0xb0], 0
141471e44: mov rax, qword ptr [rbp + 0x68]
141471e48: lea rax, [rax + rax*4]
141471e4c: lea rax, [rdx + rax*8]
141471e50: mov qword ptr [rbp + 0x20], rax
141471e54: mov r10, 0xffffffffffffffff
141471e5b: movabs r14, 0x800000000000000e
141471e65: mov edi, 2
141471e6a: movss xmm15, dword ptr [rip + 0x161059d]
141471e73: movss xmm6, dword ptr [rip + 0x14b3a89]
141471e7b: movss xmm7, dword ptr [rip + 0x14ee7f1]
141471e83: movss xmm9, dword ptr [rip + 0x14dab84]
141471e8c: mov dword ptr [rbp + 0x304], 0
141471e96: mov dword ptr [rbp + 0x2f0], 0
141471ea0: mov dword ptr [rbp + 0x330], r13d
141471ea7: mov qword ptr [rbp + 0x110], r12
141471eae: cmp rdx, qword ptr [rbp + 0x20]
141471eb2: je 0x141472f5e
141471eb8: mov qword ptr [rbp + 0x2f8], rdx
141471ebf: mov rax, qword ptr [rdx + 0x10]
141471ec3: test rax, rax
141471ec6: je 0x141472331
141471ecc: mov rcx, qword ptr [rbp + 0x2f8]
141471ed3: mov rbx, qword ptr [rcx + 8]
141471ed7: lea rax, [rax + rax*2]
141471edb: lea r15, [rbx + rax*8]
141471edf: mov qword ptr [rbp + 8], r15
141471ee3: jmp 0x141471ef7
141471ee5: mov dword ptr [rax + rcx + 0x10], r8d
141471eea: add rbx, 0x18
141471eee: cmp rbx, r15
141471ef1: je 0x141472331
141471ef7: mov rax, qword ptr [rbx]
141471efa: cmp rax, 7
141471efe: jne 0x141472147
141471f04: mov rsi, qword ptr [rbx + 0x10]
141471f08: test rsi, rsi
141471f0b: je 0x1414721f0
141471f11: mov qword ptr [rbp], rbx
141471f15: mov r15, qword ptr [rbx + 8]
141471f19: shl rsi, 4
141471f1d: add rsi, r15
141471f20: jmp 0x141471f3d
141471f22: nop word ptr cs:[rax + rax]
141471f30: add r15, 0x10
141471f34: cmp r15, rsi
141471f37: je 0x14147212e
141471f3d: mov r12, qword ptr [r15 + 8]
141471f41: test r12, r12
141471f44: je 0x141471f30
141471f46: mov rbx, qword ptr [r15]
141471f49: shl r12, 3
141471f4d: xor r13d, r13d
141471f50: jmp 0x141471f6f
141471f52: xor eax, eax
141471f54: nop word ptr cs:[rax + rax]
141471f60: or byte ptr [rbp + 0x336], al
141471f66: add r13, 8
141471f6a: cmp r12, r13
141471f6d: je 0x141471f30
141471f6f: mov rcx, qword ptr [rbx + r13]
141471f73: mov eax, 1
141471f78: lock cmpxchg qword ptr [rcx + 8], r10
141471f7e: jne 0x1414732b8
141471f84: mov rax, qword ptr [rcx]
141471f87: mov qword ptr [rcx + 8], 1
141471f8f: cmp rax, 1
141471f93: jne 0x1414732b8
141471f99: mov r8, qword ptr [rbx + r13]
141471f9d: mov rcx, qword ptr [r8 + 0x10]
141471fa1: cmp rcx, r14
141471fa4: jne 0x141472090
141471faa: lea rcx, [r8 + 0x10]
141471fae: mov rax, qword ptr [r8 + 0x18]
141471fb2: mov qword ptr [rbp + 0x100], rax
141471fb9: mov r9d, dword ptr [r8 + 0x20]
141471fbd: lea rax, [r8 + 0x24]
141471fc1: mov qword ptr [rbp + 0x318], rax
141471fc8: cmp byte ptr [r8 + 0x3c], 0
141471fcd: lea rax, [rbp + 0x160]
141471fd4: lea rdx, [rbp + 0x270]
141471fdb: cmovne rax, rdx
141471fdf: mov edx, r9d
141471fe2: cmovne edx, edi
141471fe5: mov dword ptr [rbp + 0x158], edx
141471feb: cmove r9d, edi
141471fef: mov dword ptr [rbp + 0x320], r9d
141471ff6: mov rdx, qword ptr [r8 + 0x34]
141471ffa: mov qword ptr [rax + 0x10], rdx
141471ffe: movups xmm0, xmmword ptr [r8 + 0x24]
141472003: movups xmmword ptr [rax], xmm0
141472006: mov qword ptr [rbp + 0x308], r8
14147200d: call 0x140a82940 ; core::ptr::drop_in_place<enum2$<country_core::resources::history::wall_compound_edit::WallHistoryEdit> >
141472012: movabs rax, 0x8000000000000003
14147201c: mov rdx, qword ptr [rbp + 0x308]
141472023: mov qword ptr [rdx + 0x10], rax
141472027: mov rax, qword ptr [rbp + 0x100]
14147202e: mov qword ptr [rdx + 0x18], rax
141472032: mov eax, dword ptr [rbp + 0x158]
141472038: mov dword ptr [rdx + 0x20], eax
14147203b: mov rax, qword ptr [rbp + 0x170]
141472042: mov rcx, qword ptr [rbp + 0x318]
141472049: mov qword ptr [rcx + 0x10], rax
14147204d: movups xmm0, xmmword ptr [rbp + 0x160]
141472054: movups xmmword ptr [rcx], xmm0
141472057: mov eax, dword ptr [rbp + 0x320]
14147205d: mov dword ptr [rdx + 0x3c], eax
141472060: movups xmm0, xmmword ptr [rbp + 0x270]
141472067: movups xmmword ptr [rdx + 0x40], xmm0
14147206b: mov rax, qword ptr [rbp + 0x280]
141472072: mov qword ptr [rdx + 0x50], rax
141472076: mov al, 1
141472078: mov r10, 0xffffffffffffffff
14147207f: jmp 0x141471f60
141472084: nop word ptr cs:[rax + rax]
141472090: xor eax, eax
141472092: movabs rdx, 0x8000000000000010
14147209c: cmp rcx, rdx
14147209f: jl 0x141471f60
1414720a5: cmp dword ptr [r8 + 0xf8], 2
1414720ad: jae 0x141471f60
1414720b3: movss xmm0, dword ptr [r8 + 0xfc]
1414720bc: ucomiss xmm0, dword ptr [r8 + 0x100]
1414720c4: jne 0x141471f52
1414720ca: jp 0x141471f52
1414720d0: addss xmm0, xmm15
1414720d5: ucomiss xmm0, xmm0
1414720d8: jp 0x141472ef2
1414720de: movd eax, xmm0
1414720e2: and eax, 0x7fffffff
1414720e7: cmp eax, 0x7f800000
1414720ec: je 0x141472f16
1414720f2: xorps xmm1, xmm1
1414720f5: maxss xmm1, xmm0
1414720f9: movaps xmm0, xmm6
1414720fc: minss xmm0, xmm1
141472100: movss dword ptr [r8 + 0xfc], xmm0
141472109: addss xmm0, xmm7
14147210d: ucomiss xmm0, xmm9
141472111: jae 0x141472f3a
141472117: movaps xmm1, xmm6
14147211a: minss xmm1, xmm0
14147211e: movss dword ptr [r8 + 0x100], xmm1
141472127: mov al, 1
141472129: jmp 0x141471f60
14147212e: mov rbx, qword ptr [rbp]
141472132: mov rax, qword ptr [rbx]
141472135: mov r13d, dword ptr [rbp + 0x330]
14147213c: mov r12, qword ptr [rbp + 0x110]
141472143: mov r15, qword ptr [rbp + 8]
141472147: test rax, rax
14147214a: jne 0x1414721f0
141472150: mov rcx, qword ptr [rbx + 8]
141472154: mov eax, 1
141472159: lock cmpxchg qword ptr [rcx + 8], r10
14147215f: jne 0x1414732d0
141472165: mov rax, qword ptr [rcx]
141472168: mov qword ptr [rcx + 8], 1
141472170: cmp rax, 1
141472174: jne 0x1414732d0
14147217a: mov rax, qword ptr [rbx + 8]
14147217e: xor ecx, ecx
141472180: cmp rcx, qword ptr [rax + 0x10]
141472184: jno 0x1414721f0
141472186: cmp qword ptr [rax + 0x28], 2
14147218b: jne 0x1414721f0
14147218d: mov rax, qword ptr [rax + 0x20]
141472191: movss xmm2, dword ptr [rax + 8]
141472196: movsd xmm1, qword ptr [rax + 0xc]
14147219b: movss xmm0, dword ptr [rax + 0x14]
1414721a0: subss xmm2, xmm0
1414721a4: movsd xmm3, qword ptr [rax]
1414721a8: subps xmm3, xmm1
1414721ab: mulps xmm3, xmm3
1414721ae: movshdup xmm4, xmm3
1414721b2: addss xmm4, xmm3
1414721b6: mulss xmm2, xmm2
1414721ba: addss xmm2, xmm4
1414721be: sqrtss xmm2, xmm2
1414721c2: ucomiss xmm2, dword ptr [rip + 0x14ee4c7]
1414721c9: jne 0x1414721f0
1414721cb: jp 0x1414721f0
1414721cd: movss xmm2, dword ptr [rip + 0x155ae8b]
1414721d5: addps xmm1, xmm2
1414721d8: movlps qword ptr [rax + 0xc], xmm1
1414721dc: addss xmm0, dword ptr [rip + 0x14ee4ac]
1414721e4: movss dword ptr [rax + 0x14], xmm0
1414721e9: mov byte ptr [rbp + 0x336], 1
1414721f0: cmp r13d, 0x14
1414721f4: jae 0x141471eea
1414721fa: cmp dword ptr [rbx], 9
1414721fd: jne 0x141471eea
141472203: mov rcx, qword ptr [rbx + 8]
141472207: mov eax, 1
14147220c: lock cmpxchg qword ptr [rcx + 8], r10
141472212: jne 0x1414732e8
141472218: mov rax, qword ptr [rcx]
14147221b: mov qword ptr [rcx + 8], 1
141472223: cmp rax, 1
141472227: jne 0x1414732e8
14147222d: mov rax, qword ptr [rbx + 8]
141472231: movzx ecx, byte ptr [rax + 0x18]
141472235: cmp ecx, 4
141472238: je 0x14147224f
14147223a: cmp ecx, 7
14147223d: jne 0x141471eea
141472243: mov ecx, 0x14
141472248: mov edx, 0x10
14147224d: jmp 0x141472259
14147224f: mov ecx, 0x1c
141472254: mov edx, 0x18
141472259: mov r9d, dword ptr [rax + rdx + 0x10]
14147225e: cmp r9, 8
141472262: ja 0x1414722a6
141472264: lea r11, [rip + 0x17f2b05]
14147226b: movsxd r8, dword ptr [r11 + r9*4]
14147226f: add r8, r11
141472272: jmp r8
141472275: mov r8d, r9d
141472278: mov dword ptr [rax + rdx + 0x10], r8d
14147227d: mov r8d, dword ptr [rax + rcx + 0x10]
141472282: cmp r8, 8
141472286: ja 0x1414722b7
141472288: lea r9, [rip + 0x17f2b05]
14147228f: movsxd rdx, dword ptr [r9 + r8*4]
141472293: add rdx, r9
141472296: jmp rdx
141472298: mov edx, 2
14147229d: mov dword ptr [rax + rcx + 0x10], edx
1414722a1: jmp 0x141471eea
1414722a6: mov r8d, 1
1414722ac: cmp r9d, 0x3e7
1414722b3: je 0x141472275
1414722b5: jmp 0x141472278
1414722b7: mov edx, 1
1414722bc: cmp r8d, 0x3e7
1414722c3: je 0x141471ee5
1414722c9: jmp 0x141472328
1414722cb: mov r8d, 2
1414722d1: jmp 0x141472278
1414722d3: mov r8d, 3
1414722d9: jmp 0x141472278
1414722db: mov edx, 3
1414722e0: mov dword ptr [rax + rcx + 0x10], edx
1414722e4: jmp 0x141471eea
1414722e9: mov r8d, 5
1414722ef: jmp 0x141472278
1414722f1: mov edx, 5
1414722f6: mov dword ptr [rax + rcx + 0x10], edx
1414722fa: jmp 0x141471eea
1414722ff: mov r8d, 8
141472305: jmp 0x141472278
14147230a: mov edx, 8
14147230f: mov dword ptr [rax + rcx + 0x10], edx
141472313: jmp 0x141471eea
141472318: mov r8d, 4
14147231e: jmp 0x141472278
141472323: mov edx, 4
141472328: mov dword ptr [rax + rcx + 0x10], edx
14147232c: jmp 0x141471eea
141472331: cmp r13d, 0x10
141472335: jae 0x1414724ac
14147233b: mov r8, qword ptr [rbp + 0x2f8]
141472342: mov rcx, qword ptr [r8 + 0x18]
141472346: mov eax, 1
14147234b: xor edx, edx
14147234d: lock cmpxchg qword ptr [rcx], rdx
141472352: lea rsi, [r8 + 0x18]
141472356: mov r15, qword ptr [r8 + 0x18]
14147235a: jne 0x141472376
14147235c: mov rax, qword ptr [r15 + 8]
141472360: cmp rax, 1
141472364: jne 0x1414727ba
14147236a: mov qword ptr [r15], 1
141472371: jmp 0x141472499
141472376: mov qword ptr [rbp + 0x318], rsi
14147237d: lea rdx, [r15 + 0x10]
141472381: mov byte ptr [rbp + 0x337], 1
141472388: lea rcx, [rbp + 0x270]
14147238f: mov qword ptr [rbp + 0x308], rdx
141472396: call 0x140b5d170 ; alloc::sync::UniqueArcUninit::new<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global>
14147239b: movaps xmmword ptr [rbp + 0x100], xmm8
1414723a3: movdqa xmm8, xmm14
1414723a8: movaps xmm14, xmm13
1414723ac: movaps xmm13, xmm12
1414723b0: movaps xmm12, xmm11
1414723b4: movaps xmm11, xmm10
1414723b8: mov rsi, qword ptr [rbp + 0x270]
1414723bf: mov rbx, qword ptr [rbp + 0x280]
1414723c6: mov qword ptr [rbp + 0x320], r15
1414723cd: movups xmm10, xmmword ptr [r15 + 0x30]
1414723d2: mov byte ptr [rbp + 0x335], 1
1414723d9: lea rcx, [rbp + 0x160]
1414723e0: mov rdx, qword ptr [rbp + 0x308]
1414723e7: call 0x140b72b40 ; hashbrown::raw::impl$13::clone<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>
1414723ec: lea rax, [rsi - 1]
1414723f0: mov rcx, rsi
1414723f3: xor rcx, rax
1414723f6: lea rdx, [rsi + 0xf]
1414723fa: neg rsi
1414723fd: and rsi, rdx
141472400: add rsi, -0x10
141472404: cmp rcx, rax
141472407: mov rax, 0xffffffffffffffff
14147240e: cmovbe rsi, rax
141472412: movups xmmword ptr [rbp + 0x180], xmm10
14147241a: movups xmm0, xmmword ptr [rbp + 0x180]
141472421: movups xmmword ptr [rbx + rsi + 0x30], xmm0
141472426: movups xmm0, xmmword ptr [rbp + 0x160]
14147242d: movups xmm1, xmmword ptr [rbp + 0x170]
141472434: movups xmmword ptr [rbx + rsi + 0x20], xmm1
141472439: mov qword ptr [rbp + 0x308], rbx
141472440: movups xmmword ptr [rbx + rsi + 0x10], xmm0
141472445: test byte ptr [rbp + 0x288], 1
14147244c: je 0x141473338
141472452: mov rax, qword ptr [rbp + 0x320]
141472459: lock dec qword ptr [rax]
14147245d: movaps xmm10, xmm11
141472461: jne 0x14147246f
141472463: mov rcx, qword ptr [rbp + 0x318]
14147246a: call 0x140b5cb60 ; alloc::sync::Arc::drop_slow<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global>
14147246f: mov rax, qword ptr [rbp + 0x318]
141472476: mov r15, qword ptr [rbp + 0x308]
14147247d: mov qword ptr [rax], r15
141472480: movaps xmm11, xmm12
141472484: movaps xmm12, xmm13
141472488: movaps xmm13, xmm14
14147248c: movdqa xmm14, xmm8
141472491: movaps xmm8, xmmword ptr [rbp + 0x100]
141472499: add r15, 0x10
14147249d: mov byte ptr [rbp + 0x337], 1
1414724a4: mov rcx, r15
1414724a7: call 0x140b79bd0 ; hashbrown::raw::RawTable::clear<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,alloc::alloc::Global>
1414724ac: mov byte ptr [rbp + 0x337], 1
1414724b3: mov rcx, qword ptr [rbp + 0x2f8]
1414724ba: lea rdx, [rbp + 0x336]
1414724c1: lea r8, [rbp + 0x300]
1414724c8: call 0x1412f50f0 ; alloc::vec::Vec::retain<enum2$<country_core::resources::history::EditType>,alloc::alloc::Global,country_core::systems::save::load_game::parse_saved_game_data::closure_env$2>
1414724cd: mov rax, qword ptr [rbp + 0x2f8]
1414724d4: lea rdx, [rax + 0x28]
1414724d8: cmp r13d, 0x14
1414724dc: jae 0x141472877
1414724e2: mov rcx, qword ptr [rax + 0x18]
1414724e6: cmp qword ptr [rcx + 0x28], 0
1414724eb: mov r10, 0xffffffffffffffff
1414724f2: je 0x141471eae
1414724f8: mov eax, 1
1414724fd: lock cmpxchg qword ptr [rcx + 8], r10
141472503: jne 0x14147335b
141472509: mov rax, qword ptr [rcx]
14147250c: mov qword ptr [rcx + 8], 1
141472514: cmp rax, 1
141472518: jne 0x14147335b
14147251e: mov rax, qword ptr [rbp + 0x2f8]
141472525: mov rax, qword ptr [rax + 0x18]
141472529: mov rcx, qword ptr [rax + 0x28]
14147252d: test rcx, rcx
141472530: je 0x14147288a
141472536: mov rbx, qword ptr [rax + 0x10]
14147253a: lea rax, [rbx + 0x10]
14147253e: mov qword ptr [rbp + 0x158], rax
141472545: movdqa xmm0, xmmword ptr [rbx]
141472549: pmovmskb eax, xmm0
14147254d: not eax
14147254f: mov qword ptr [rbp + 0x320], rax
141472556: mov qword ptr [rbp + 0x100], rdx
14147255d: jmp 0x14147258b
14147255f: mov rcx, qword ptr [rbp + 0x318]
141472566: dec rcx
141472569: mov r10, 0xffffffffffffffff
141472570: mov rax, qword ptr [rbp + 0x2f8]
141472577: mov rdx, qword ptr [rbp + 0x100]
14147257e: mov rbx, qword ptr [rbp + 0x308]
141472585: je 0x14147287e
14147258b: mov r8, qword ptr [rbp + 0x320]
141472592: test r8w, r8w
141472596: mov qword ptr [rbp + 0x318], rcx
14147259d: je 0x1414725a8
14147259f: lea eax, [r8 - 1]
1414725a3: and eax, r8d
1414725a6: jmp 0x1414725e3
1414725a8: mov rax, qword ptr [rbp + 0x158]
1414725af: movdqa xmm0, xmmword ptr [rax]
1414725b3: pmovmskb r8d, xmm0
1414725b8: add rbx, -0x700
1414725bf: add rax, 0x10
1414725c3: cmp r8d, 0xffff
1414725ca: je 0x1414725af
1414725cc: mov qword ptr [rbp + 0x158], rax
1414725d3: mov ecx, 0xfffffffe
1414725d8: sub ecx, r8d
1414725db: not r8d
1414725de: mov eax, r8d
1414725e1: and eax, ecx
1414725e3: mov ecx, r8d
1414725e6: tzcnt ecx, ecx
1414725ea: mov qword ptr [rbp + 0x320], rax
1414725f1: neg rcx
1414725f4: imul rcx, rcx, 0x70
1414725f8: mov r8, qword ptr [rbx + rcx - 0x60]
1414725fd: mov r10, qword ptr [rbx + rcx - 0x48]
141472602: movdqa xmm0, xmmword ptr [r8]
141472607: pmovmskb edx, xmm0
14147260b: not edx
14147260d: lea r11, [r8 + 0x10]
141472611: mov rsi, qword ptr [rbx + rcx - 0x30]
141472616: movdqa xmm0, xmmword ptr [rsi]
14147261a: pmovmskb r9d, xmm0
14147261f: not r9d
141472622: lea rax, [rsi + 0x10]
141472626: mov qword ptr [rbp + 0x308], rbx
14147262d: mov rcx, qword ptr [rbx + rcx - 0x18]
141472632: test r8, r8
141472635: jne 0x14147267e
141472637: jmp 0x1414726dc
14147263c: lea r12d, [r9 - 1]
141472640: and r12d, r9d
141472643: dec rcx
141472646: mov r15d, r9d
141472649: mov rbx, rsi
14147264c: xor r8d, r8d
14147264f: mov r9d, r12d
141472652: tzcnt r15d, r15d
141472657: neg r15
14147265a: imul r15, r15, 0x58
14147265e: movzx r12d, byte ptr [rbx + r15 - 6]
141472664: add r12b, 0xc8
141472668: cmp r12b, 2
14147266c: mov r12, qword ptr [rbp + 0x110]
141472673: jbe 0x141472734
141472679: test r8, r8
14147267c: je 0x1414726dc
14147267e: test r10, r10
141472681: je 0x1414726d9
141472683: test dx, dx
141472686: je 0x1414726a0
141472688: lea r12d, [rdx - 1]
14147268c: and r12d, edx
14147268f: dec r10
141472692: mov r15d, edx
141472695: mov rbx, r8
141472698: mov edx, r12d
14147269b: jmp 0x141472652
14147269d: nop dword ptr [rax]
1414726a0: movdqa xmm0, xmmword ptr [r11]
1414726a5: pmovmskb r15d, xmm0
1414726aa: add r8, -0x580
1414726b1: add r11, 0x10
1414726b5: cmp r15d, 0xffff
1414726bc: je 0x1414726a0
1414726be: mov ebx, 0xfffffffe
1414726c3: sub ebx, r15d
1414726c6: not r15d
1414726c9: mov edx, r15d
1414726cc: and edx, ebx
1414726ce: dec r10
1414726d1: mov rbx, r8
1414726d4: jmp 0x141472652
1414726d9: xor r10d, r10d
1414726dc: test rsi, rsi
1414726df: je 0x14147255f
1414726e5: test rcx, rcx
1414726e8: je 0x14147255f
1414726ee: test r9w, r9w
1414726f2: jne 0x14147263c
1414726f8: nop dword ptr [rax + rax]
141472700: movdqa xmm0, xmmword ptr [rax]
141472704: pmovmskb r9d, xmm0
141472709: add rsi, -0x580
141472710: add rax, 0x10
141472714: cmp r9d, 0xffff
14147271b: je 0x141472700
14147271d: mov r8d, 0xfffffffe
141472723: sub r8d, r9d
141472726: not r9d
141472729: mov r12d, r9d
14147272c: and r12d, r8d
14147272f: jmp 0x141472643
141472734: add rbx, r15
141472737: mov r12d, dword ptr [rbx - 0x28]
14147273b: cmp r12, 8
14147273f: ja 0x141472758
141472741: lea r13, [rip + 0x17f2670]
141472748: movsxd r15, dword ptr [r13 + r12*4]
14147274d: add r15, r13
141472750: jmp r15
141472753: mov r15d, r12d
141472756: jmp 0x141472792
141472758: mov r15d, 1
14147275e: cmp r12d, 0x3e7
141472765: jne 0x141472799
141472767: mov r15d, r12d
14147276a: jmp 0x141472799
14147276c: mov r15d, 2
141472772: jmp 0x141472792
141472774: mov r15d, 3
14147277a: jmp 0x141472792
14147277c: mov r15d, 5
141472782: jmp 0x141472792
141472784: mov r15d, 8
14147278a: jmp 0x141472792
14147278c: mov r15d, 4
141472792: mov r13d, dword ptr [rbp + 0x330]
141472799: mov dword ptr [rbx - 0x28], r15d
14147279d: mov bl, 1
14147279f: mov dword ptr [rbp + 0x2f0], ebx
1414727a5: mov r12, qword ptr [rbp + 0x110]
1414727ac: test r8, r8
1414727af: jne 0x14147267e
1414727b5: jmp 0x1414726dc
1414727ba: lea rax, [r15 + 8]
1414727be: mov qword ptr [rbp + 0x318], rax
1414727c5: mov qword ptr [rbp + 0x320], r15
1414727cc: lea rbx, [r15 + 0x10]
1414727d0: lea rcx, [rbp + 0x160]
1414727d7: mov rdx, rbx
1414727da: call 0x140b5d170 ; alloc::sync::UniqueArcUninit::new<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global>
1414727df: mov rax, qword ptr [rbp + 0x160]
1414727e6: mov r15, qword ptr [rbp + 0x170]
1414727ed: lea rcx, [rax - 1]
1414727f1: mov rdx, rax
1414727f4: xor rdx, rcx
1414727f7: lea r8, [rax + 0xf]
1414727fb: neg rax
1414727fe: and rax, r8
141472801: add rax, -0x10
141472805: cmp rdx, rcx
141472808: mov rcx, 0xffffffffffffffff
14147280f: cmovbe rax, rcx
141472813: movups xmm0, xmmword ptr [rbx]
141472816: movups xmm1, xmmword ptr [rbx + 0x10]
14147281a: movups xmm2, xmmword ptr [rbx + 0x20]
14147281e: movups xmmword ptr [r15 + rax + 0x30], xmm2
141472824: movups xmmword ptr [r15 + rax + 0x20], xmm1
14147282a: movups xmmword ptr [r15 + rax + 0x10], xmm0
141472830: test byte ptr [rbp + 0x178], 1
141472837: je 0x14147334d
14147283d: mov qword ptr [rsi], r15
141472840: mov rcx, qword ptr [rbp + 0x320]
141472847: cmp rcx, -1
14147284b: je 0x141472499
141472851: mov rax, qword ptr [rbp + 0x318]
141472858: lock dec qword ptr [rax]
14147285c: jne 0x141472499
141472862: mov edx, 0x40
141472867: mov r8d, 8
14147286d: call 0x140613c20 ; 0x140613c20
141472872: jmp 0x141472499
141472877: mov r10, 0xffffffffffffffff
14147287e: cmp r13d, 0x15
141472882: jae 0x141471eae
141472888: jmp 0x141472891
14147288a: mov rax, qword ptr [rbp + 0x2f8]
141472891: mov rcx, qword ptr [rax + 0x18]
141472895: cmp qword ptr [rcx + 0x28], 0
14147289a: je 0x141471eae
1414728a0: mov eax, 1
1414728a5: lock cmpxchg qword ptr [rcx + 8], r10
1414728ab: jne 0x141473370
1414728b1: mov rax, qword ptr [rcx]
1414728b4: mov qword ptr [rcx + 8], 1
1414728bc: cmp rax, 1
1414728c0: jne 0x141473370
1414728c6: mov rax, qword ptr [rbp + 0x2f8]
1414728cd: mov rax, qword ptr [rax + 0x18]
1414728d1: mov rcx, qword ptr [rax + 0x28]
1414728d5: test rcx, rcx
1414728d8: je 0x141471eae
1414728de: mov qword ptr [rbp + 0x100], rdx
1414728e5: mov rax, qword ptr [rax + 0x10]
1414728e9: lea rdx, [rax + 0x10]
1414728ed: mov qword ptr [rbp + 0x308], rdx
1414728f4: mov qword ptr [rbp + 0x318], rax
1414728fb: movdqa xmm0, xmmword ptr [rax]
1414728ff: pmovmskb eax, xmm0
141472903: not eax
141472905: mov qword ptr [rbp + 0x320], rax
14147290c: mov r13, rcx
14147290f: jmp 0x14147291a
141472911: dec r13
141472914: je 0x141472abd
14147291a: mov rdx, qword ptr [rbp + 0x320]
141472921: test dx, dx
141472924: je 0x14147292d
141472926: lea eax, [rdx - 1]
141472929: and eax, edx
14147292b: jmp 0x141472970
14147292d: mov rax, qword ptr [rbp + 0x318]
141472934: mov rcx, qword ptr [rbp + 0x308]
14147293b: movdqa xmm0, xmmword ptr [rcx]
14147293f: pmovmskb edx, xmm0
141472943: add rax, -0x700
141472949: add rcx, 0x10
14147294d: cmp edx, 0xffff
141472953: je 0x14147293b
141472955: mov qword ptr [rbp + 0x308], rcx
14147295c: mov qword ptr [rbp + 0x318], rax
141472963: mov ecx, 0xfffffffe
141472968: sub ecx, edx
14147296a: not edx
14147296c: mov eax, edx
14147296e: and eax, ecx
141472970: mov ecx, edx
141472972: tzcnt ecx, ecx
141472976: mov qword ptr [rbp + 0x320], rax
14147297d: neg rcx
141472980: imul rax, rcx, 0x70
141472984: mov rbx, qword ptr [rbp + 0x318]
14147298b: mov r8, qword ptr [rbx + rax - 0x60]
141472990: mov rcx, qword ptr [rbx + rax - 0x48]
141472995: movdqa xmm0, xmmword ptr [r8]
14147299a: pmovmskb edx, xmm0
14147299e: not edx
1414729a0: lea r10, [r8 + 0x10]
1414729a4: mov r11, qword ptr [rbx + rax - 0x30]
1414729a9: movdqa xmm0, xmmword ptr [r11]
1414729ae: pmovmskb r9d, xmm0
1414729b3: not r9d
1414729b6: lea rsi, [r11 + 0x10]
1414729ba: mov rax, qword ptr [rbx + rax - 0x18]
1414729bf: test r8, r8
1414729c2: jne 0x1414729fb
1414729c4: jmp 0x141472a4f
1414729c9: lea r12d, [r9 - 1]
1414729cd: and r12d, r9d
1414729d0: dec rax
1414729d3: mov ebx, r9d
1414729d6: mov r15, r11
1414729d9: xor r8d, r8d
1414729dc: mov r9d, r12d
1414729df: tzcnt ebx, ebx
1414729e3: neg rbx
1414729e6: imul rbx, rbx, 0x58
1414729ea: cmp byte ptr [r15 + rbx - 6], 0x3a
1414729f0: je 0x141472a9f
1414729f6: test r8, r8
1414729f9: je 0x141472a4f
1414729fb: test rcx, rcx
1414729fe: je 0x141472a4d
141472a00: test dx, dx
141472a03: je 0x141472a19
141472a05: lea r12d, [rdx - 1]
141472a09: and r12d, edx
141472a0c: dec rcx
141472a0f: mov ebx, edx
141472a11: mov r15, r8
141472a14: mov edx, r12d
141472a17: jmp 0x1414729df
141472a19: movdqa xmm0, xmmword ptr [r10]
141472a1e: pmovmskb ebx, xmm0
141472a22: add r8, -0x580
141472a29: add r10, 0x10
141472a2d: cmp ebx, 0xffff
141472a33: je 0x141472a19
141472a35: mov r15d, 0xfffffffe
141472a3b: sub r15d, ebx
141472a3e: not ebx
141472a40: mov edx, ebx
141472a42: and edx, r15d
141472a45: dec rcx
141472a48: mov r15, r8
141472a4b: jmp 0x1414729df
141472a4d: xor ecx, ecx
141472a4f: test r11, r11
141472a52: je 0x141472911
141472a58: test rax, rax
141472a5b: je 0x141472911
141472a61: test r9w, r9w
141472a65: jne 0x1414729c9
141472a6b: movdqa xmm0, xmmword ptr [rsi]
141472a6f: pmovmskb r9d, xmm0
141472a74: add r11, -0x580
141472a7b: add rsi, 0x10
141472a7f: cmp r9d, 0xffff
141472a86: je 0x141472a6b
141472a88: mov r8d, 0xfffffffe
141472a8e: sub r8d, r9d
141472a91: not r9d
141472a94: mov r12d, r9d
141472a97: and r12d, r8d
141472a9a: jmp 0x1414729d0
141472a9f: add r15, rbx
141472aa2: mov dword ptr [r15 - 0x28], 0x3e7
141472aaa: mov bl, 1
141472aac: mov dword ptr [rbp + 0x304], ebx
141472ab2: test r8, r8
141472ab5: jne 0x1414729fb
141472abb: jmp 0x141472a4f
141472abd: mov r13d, dword ptr [rbp + 0x330]
141472ac4: mov r12, qword ptr [rbp + 0x110]
141472acb: mov r10, 0xffffffffffffffff
141472ad2: mov rdx, qword ptr [rbp + 0x100]
141472ad9: jmp 0x141471eae
141472ade: mov ecx, 1
141472ae3: mov rdx, rax
141472ae6: call 0x14146ccf0 ; core::result::Result::and_then<ref$<enum2$<serde_json::value::Value> >,anyhow::Error,country_core::resources::history::HistoryIdentity,country_core::systems::save::load_game::parse_saved_game_data::closure_env$4>
141472aeb: mov r15, rdx
141472aee: test rax, rax
141472af1: je 0x141472c8f
141472af7: mov qword ptr [rbp + 0xd8], r15
141472afe: mov rsi, qword ptr [rip + 0x21b98a3]
141472b05: mov rax, qword ptr [rsi]
141472b08: cmp rax, 3
141472b0c: jb 0x141472bba
141472b12: lea rax, [rbp + 0xd8]
141472b19: mov qword ptr [rbp + 0x250], rax
141472b20: lea rax, [rip - 0x725f07]
141472b27: mov qword ptr [rbp + 0x258], rax
141472b2e: lea rax, [rip + 0x17f1cdb]
141472b35: mov qword ptr [rbp + 0x160], rax
141472b3c: mov qword ptr [rbp + 0x168], 2
141472b47: mov qword ptr [rbp + 0x180], 0
141472b52: mov qword ptr [rbp + 0x170], r14
141472b59: mov qword ptr [rbp + 0x178], 1
141472b64: lea rcx, [rip + 0x17f1cc5]
141472b6b: call 0x14067dd80 ; toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut
141472b70: lea rcx, [rip + 0x17f15d1]
141472b77: mov qword ptr [rbp + 0x270], rcx
141472b7e: mov qword ptr [rbp + 0x278], 0x26
141472b89: mov qword ptr [rbp + 0x280], rcx
141472b90: mov qword ptr [rbp + 0x288], 0x26
141472b9b: mov qword ptr [rbp + 0x290], rax
141472ba2: lea rcx, [rbp + 0x160]
141472ba9: lea r8, [rbp + 0x270]
141472bb0: mov edx, 3
141472bb5: call 0x1404b1430 ; log::__private_api::log<tuple$<>,log::__private_api::GlobalLogger>
141472bba: mov rcx, r12
141472bbd: mov rdx, qword ptr [rbp + 0x28]
141472bc1: call 0x141673140 ; country_core::systems::save::save_file_identity::derive_fallback_save_file_identity
141472bc6: mov r15, rax
141472bc9: mov qword ptr [rbp + 0x30], rax
141472bcd: mov rax, qword ptr [rsi]
141472bd0: cmp rax, 3
141472bd4: jb 0x141472c83
141472bda: lea rax, [rbp + 0x30]
141472bde: mov qword ptr [rbp + 0x250], rax
141472be5: lea rax, [rip - 0x6e552c]
141472bec: mov qword ptr [rbp + 0x258], rax
141472bf3: lea rax, [rip + 0x17f1c76]
141472bfa: mov qword ptr [rbp + 0x160], rax
141472c01: mov qword ptr [rbp + 0x168], 1
141472c0c: mov qword ptr [rbp + 0x180], 0
141472c17: mov qword ptr [rbp + 0x170], r14
141472c1e: mov qword ptr [rbp + 0x178], 1
141472c29: lea rcx, [rip + 0x17f1c50]
141472c30: call 0x14067dd80 ; toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut
141472c35: lea rcx, [rip + 0x17f150c]
141472c3c: mov qword ptr [rbp + 0x270], rcx
141472c43: mov qword ptr [rbp + 0x278], 0x26
141472c4e: mov qword ptr [rbp + 0x280], rcx
141472c55: mov qword ptr [rbp + 0x288], 0x26
141472c60: mov qword ptr [rbp + 0x290], rax
141472c67: lea rcx, [rbp + 0x160]
141472c6e: lea r8, [rbp + 0x270]
141472c75: mov edx, 3
141472c7a: call 0x1404b1430 ; log::__private_api::log<tuple$<>,log::__private_api::GlobalLogger>
141472c7f: mov r15, qword ptr [rbp + 0x30]
141472c83: lea rcx, [rbp + 0xd8]
141472c8a: call 0x140d4cc30 ; anyhow::error::impl$6::drop
141472c8f: movsd xmm6, qword ptr [rbp + 0xc0]
141472c97: mov rdx, qword ptr [rbp + 0x60]
141472c9b: mov rax, qword ptr [rbp + 0x68]
141472c9f: lea rax, [rax + rax*4]
141472ca3: lea r8, [rdx + rax*8]
141472ca7: lea r9, [rip + 0x17f119a]
141472cae: lea rcx, [rbp + 0xd8]
141472cb5: call 0x14130dec0 ; alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::history::MultiEdit,core::iter::adapters::cloned::Cloned<core::slice::iter::Iter<country_core::resources::history::MultiEdit> > >
141472cba: mov rdx, qword ptr [rbp + 0x78]
141472cbe: mov r8, qword ptr [rbp + 0x80]
141472cc5: shl r8, 4
141472cc9: add r8, rdx
141472ccc: lea r9, [rip + 0x17f1175]
141472cd3: lea rcx, [rbp + 0x250]
141472cda: call 0x1413056c0 ; alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::history::ResolvedHistoryChainSnapshot,core::iter::adapters::filter_map::FilterMap<core::slice::iter::Iter<country_core::resources::history::HistoryChainSnapshot>,country_core::resources::history::impl$1::serialize::closure_env$0<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > > > >
141472cdf: mov rsi, qword ptr [rbp + 0x40]
141472ce3: mov rax, rsi
141472ce6: neg rax
141472ce9: jno 0x141472d01
141472ceb: mov r14, rdi
141472cee: movabs rax, 0x8000000000000000
141472cf8: mov qword ptr [rbp + 0x270], rax
141472cff: jmp 0x141472d1b
141472d01: mov r14, rdi
141472d04: lea r8, [rip + 0x17f1ea5]
141472d0b: lea rcx, [rbp + 0x270]
141472d12: lea rdx, [rbp + 0x40]
141472d16: call 0x1412f8930 ; alloc::vec::impl$11::clone<country_core::resources::critters::sheep::control::SheepSnapshot,alloc::alloc::Global>
141472d1b: mov rdx, qword ptr [rbp + 0x18]
141472d1f: mov rax, rdx
141472d22: shr rax, 0x20
141472d26: mov qword ptr [rbp + 0x1a8], r15
141472d2d: mov ecx, dword ptr [rbp + 0x11c]
141472d33: mov dword ptr [rbp + 0x220], ecx
141472d39: movss dword ptr [rbp + 0x224], xmm11
141472d42: mov dword ptr [rbp + 0x228], edx
141472d48: mov dword ptr [rbp + 0x22c], eax
141472d4e: mov rax, qword ptr [rbp + 0x10]
141472d52: mov qword ptr [rbp + 0x230], rax
141472d59: movups xmmword ptr [rbp + 0x238], xmm10
141472d61: mov eax, dword ptr [rbp + 0x2f4]
141472d67: mov dword ptr [rbp + 0x1f4], eax
141472d6d: movups xmmword ptr [rbp + 0x1f8], xmm8
141472d75: movups xmmword ptr [rbp + 0x208], xmm12
141472d7d: movss dword ptr [rbp + 0x218], xmm13
141472d86: movzx eax, word ptr [rbp + 0x316]
141472d8d: mov word ptr [rbp + 0x21c], ax
141472d94: movlps qword ptr [rbp + 0x248], xmm6
141472d9b: movups xmm0, xmmword ptr [rbp + 0xd8]
141472da2: movaps xmmword ptr [rbp + 0x160], xmm0
141472da9: mov rax, qword ptr [rbp + 0xe8]
141472db0: mov qword ptr [rbp + 0x170], rax
141472db7: movups xmm0, xmmword ptr [rbp + 0x250]
141472dbe: movups xmmword ptr [rbp + 0x178], xmm0
141472dc5: mov rax, qword ptr [rbp + 0x260]
141472dcc: mov qword ptr [rbp + 0x188], rax
141472dd3: mov rax, qword ptr [rbp + 0x280]
141472dda: mov qword ptr [rbp + 0x1a0], rax
141472de1: movups xmm0, xmmword ptr [rbp + 0x270]
141472de8: movaps xmmword ptr [rbp + 0x190], xmm0
141472def: movaps xmmword ptr [rbp + 0x1b0], xmm7
141472df6: mov eax, dword ptr [rbp + 0x124]
141472dfc: mov dword ptr [rbp + 0x1c0], eax
141472e02: mov eax, dword ptr [rbp + 0x128]
141472e08: mov byte ptr [rbp + 0x1c4], al
141472e0e: mov eax, dword ptr [rbp + 0x120]
141472e14: mov dword ptr [rbp + 0x1c8], eax
141472e1a: movdqu xmmword ptr [rbp + 0x1cc], xmm14
141472e23: movups xmmword ptr [rbp + 0x1dc], xmm9
141472e2b: movss dword ptr [rbp + 0x1ec], xmm15
141472e34: mov eax, dword ptr [rbp + 0x12c]
141472e3a: mov byte ptr [rbp + 0x1f0], al
141472e40: mov eax, dword ptr [rbp + 0x130]
141472e46: mov byte ptr [rbp + 0x1f1], al
141472e4c: movzx eax, byte ptr [rbp + 0x32f]
141472e53: mov byte ptr [rbp + 0x1f2], al
141472e59: call 0x14146cec0 ; alloc::alloc::exchange_malloc
141472e5e: mov rdi, rax
141472e61: lea rdx, [rbp + 0x160]
141472e68: mov r8d, 0xf0
141472e6e: mov rcx, rax
141472e71: call 0x142923c00 ; 0x142923c00
141472e76: cmp r13d, 0x1b
141472e7a: setne al
141472e7d: or al, byte ptr [rbp + 0x336]
141472e83: mov r8d, dword ptr [rbp + 0x2f0]
141472e8a: and r8b, 1
141472e8e: mov edx, dword ptr [rbp + 0x304]
141472e94: and dl, 1
141472e97: mov qword ptr [r14], rdi
141472e9a: mov rdi, r14
141472e9d: mov ecx, dword ptr [rbp + 0xd4]
141472ea3: mov dword ptr [r14 + 8], ecx
141472ea7: mov byte ptr [r14 + 0xc], al
141472eab: mov byte ptr [r14 + 0xd], r8b
141472eaf: mov byte ptr [r14 + 0xe], dl
141472eb3: mov byte ptr [r14 + 0xf], bl
141472eb7: lea rcx, [rbp + 0x58]
141472ebb: call 0x14088acf0 ; core::ptr::drop_in_place<country_core::resources::history::History>
141472ec0: mov rax, rsi
141472ec3: neg rax
141472ec6: jo 0x141472ee1
141472ec8: jae 0x141472ee1
141472eca: mov rcx, qword ptr [rbp + 0x48]
141472ece: shl rsi, 3
141472ed2: lea rdx, [rsi + rsi*4]
141472ed6: mov r8d, 4
141472edc: call 0x140613c20 ; 0x140613c20
141472ee1: lea rcx, [rbp + 0x138]
141472ee8: call 0x14085da90 ; core::ptr::drop_in_place<enum2$<serde_json::value::Value> >
141472eed: jmp 0x14147147c
141472ef2: mov byte ptr [rbp + 0x337], 1
141472ef9: lea rcx, [rip + 0x16324d0]
141472f00: lea r8, [rip + 0x1664bb1]
141472f07: mov edx, 0x1d
141472f0c: call 0x1428d9430 ; core::panicking::panic
141472f11: jmp 0x141473383
141472f16: mov byte ptr [rbp + 0x337], 1
141472f1d: lea rcx, [rip + 0x1632574]
141472f24: lea r8, [rip + 0x1664b8d]
141472f2b: mov edx, 0x22
141472f30: call 0x1428d9430 ; core::panicking::panic
141472f35: jmp 0x141473383
141472f3a: mov byte ptr [rbp + 0x337], 1
141472f41: lea rcx, [rip + 0x1632550]
141472f48: lea r8, [rip + 0x1664b81]
141472f4f: mov edx, 0x22
141472f54: call 0x1428d9430 ; core::panicking::panic
141472f59: jmp 0x141473383
141472f5e: cmp r13d, 0x13
141472f62: setl bl
141472f65: jge 0x14147319b
141472f6b: mov rax, qword ptr [rip + 0x21b9436]
141472f72: mov rax, qword ptr [rax]
141472f75: cmp rax, 3
141472f79: jb 0x14147303c
141472f7f: lea rax, [rip + 0x17f170a]
141472f86: mov qword ptr [rbp + 0x250], rax
141472f8d: lea rax, [rip - 0x6e5a14]
141472f94: mov qword ptr [rbp + 0x258], rax
141472f9b: lea rax, [rip + 0x17f172e]
141472fa2: mov qword ptr [rbp + 0x160], rax
141472fa9: mov qword ptr [rbp + 0x168], 2
141472fb4: mov qword ptr [rbp + 0x180], 0
141472fbf: lea rax, [rbp + 0x250]
141472fc6: mov qword ptr [rbp + 0x170], rax
141472fcd: mov qword ptr [rbp + 0x178], 1
141472fd8: mov byte ptr [rbp + 0x337], 1
141472fdf: lea rcx, [rip + 0x17f170a]
141472fe6: call 0x14067dd80 ; toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut
141472feb: lea rcx, [rip + 0x17f1156]
141472ff2: mov qword ptr [rbp + 0x270], rcx
141472ff9: mov qword ptr [rbp + 0x278], 0x26
141473004: mov qword ptr [rbp + 0x280], rcx
14147300b: mov qword ptr [rbp + 0x288], 0x26
141473016: mov qword ptr [rbp + 0x290], rax
14147301d: mov byte ptr [rbp + 0x337], 1
141473024: lea rcx, [rbp + 0x160]
14147302b: lea r8, [rbp + 0x270]
141473032: mov edx, 3
141473037: call 0x1404b1430 ; log::__private_api::log<tuple$<>,log::__private_api::GlobalLogger>
14147303c: cmp r13d, 0x11
141473040: jge 0x14147319b
141473046: mov qword ptr [rbp + 0xb0], 0
141473051: mov rax, qword ptr [rbp + 0x68]
141473055: test rax, rax
141473058: je 0x14147319b
14147305e: mov rcx, qword ptr [rbp + 0x60]
141473062: lea rax, [rax + rax*4]
141473066: lea rdx, [rcx + rax*8]
14147306a: xor r8d, r8d
14147306d: mov r9, 0xffffffffffffffff
141473074: movss xmm0, dword ptr [rip + 0x14c152c]
14147307c: movss xmm1, dword ptr [rip + 0x160c9ac]
141473084: movabs r10, 0x8000000000000001
14147308e: movaps xmm2, xmmword ptr [rip + 0x17b604b]
141473095: movaps xmm3, xmmword ptr [rip + 0x1647924]
14147309c: jmp 0x1414730ab
14147309e: add rcx, 0x28
1414730a2: cmp rcx, rdx
1414730a5: je 0x14147319b
1414730ab: mov rax, qword ptr [rcx + 0x10]
1414730af: test rax, rax
1414730b2: je 0x14147309e
1414730b4: mov r11, qword ptr [rcx + 8]
1414730b8: shl rax, 3
1414730bc: lea rsi, [rax + rax*2]
1414730c0: xor edi, edi
1414730c2: jmp 0x1414730d4
1414730c4: cmp qword ptr [r11 + rdi], 0
1414730c9: je 0x14147313a
1414730cb: add rdi, 0x18
1414730cf: cmp rsi, rdi
1414730d2: je 0x14147309e
1414730d4: cmp qword ptr [r11 + rdi], 0
1414730d9: jne 0x1414730cb
1414730db: mov r14, qword ptr [r11 + rdi + 8]
1414730e0: mov eax, 1
1414730e5: lock cmpxchg qword ptr [r14 + 8], r9
1414730eb: jne 0x14147330e
1414730f1: mov rax, qword ptr [r14]
1414730f4: mov qword ptr [r14 + 8], 1
1414730fc: cmp rax, 1
141473100: jne 0x14147330e
141473106: mov rax, qword ptr [r11 + rdi + 8]
14147310b: cmp r8, qword ptr [rax + 0x10]
14147310f: jno 0x1414730c4
141473111: movss xmm4, dword ptr [rax + 0x34]
141473116: sqrtss xmm4, xmm4
14147311a: mulss xmm4, xmm0
14147311e: xorps xmm5, xmm5
141473121: maxss xmm5, xmm4
141473125: movaps xmm4, xmm1
141473128: minss xmm4, xmm5
14147312c: movss dword ptr [rax + 0x34], xmm4
141473131: mov byte ptr [rbp + 0x336], 1
141473138: jmp 0x1414730c4
14147313a: mov r14, qword ptr [r11 + rdi + 8]
14147313f: mov eax, 1
141473144: lock cmpxchg qword ptr [r14 + 8], r9
14147314a: jne 0x141473323
141473150: mov rax, qword ptr [r14]
141473153: mov qword ptr [r14 + 8], 1
14147315b: cmp rax, 1
14147315f: jne 0x141473323
141473165: mov rax, qword ptr [r11 + rdi + 8]
14147316a: cmp qword ptr [rax + 0x10], r10
14147316e: jne 0x1414730cb
141473174: movsd xmm4, qword ptr [rax + 0x1c]
141473179: sqrtps xmm4, xmm4
14147317c: mulps xmm4, xmm2
14147317f: xorps xmm5, xmm5
141473182: maxps xmm5, xmm4
141473185: movaps xmm4, xmm3
141473188: minps xmm4, xmm5
14147318b: movlps qword ptr [rax + 0x1c], xmm4
14147318f: mov byte ptr [rbp + 0x336], 1
141473196: jmp 0x1414730cb
14147319b: cmp byte ptr [rbp + 0x336], 1
1414731a2: jne 0x1414731ad
1414731a4: movzx edx, byte ptr [rbp + 0xd8]
1414731ab: jmp 0x1414731b6
1414731ad: mov byte ptr [rbp + 0xd8], 0
1414731b4: xor edx, edx
1414731b6: mov byte ptr [rbp + 0x337], 0
1414731bd: and dl, 1
1414731c0: lea rcx, [rbp + 0x58]
1414731c4: call 0x140a86980 ; country_core::resources::history::History::finish_potential_mutation
1414731c9: test byte ptr [rbp + 0x336], 1
1414731d0: je 0x141473290
1414731d6: mov rax, qword ptr [rip + 0x21b91cb]
1414731dd: mov rax, qword ptr [rax]
1414731e0: cmp rax, 3
1414731e4: mov rdi, qword ptr [rbp + 0x38]
1414731e8: movaps xmm7, xmmword ptr [rbp - 0x20]
1414731ec: movaps xmm9, xmmword ptr [rbp - 0x10]
1414731f1: movss xmm15, dword ptr [rbp + 0x134]
1414731fa: lea r14, [rbp + 0x250]
141473201: jb 0x141471cc1
141473207: lea rax, [rip + 0x17f1592]
14147320e: mov qword ptr [rbp + 0x160], rax
141473215: mov qword ptr [rbp + 0x168], 1
141473220: mov qword ptr [rbp + 0x170], 8
14147322b: xorps xmm0, xmm0
14147322e: movups xmmword ptr [rbp + 0x178], xmm0
141473235: lea rcx, [rip + 0x17f1574]
14147323c: call 0x14067dd80 ; toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut
141473241: lea rcx, [rip + 0x17f0f00]
141473248: mov qword ptr [rbp + 0x270], rcx
14147324f: mov qword ptr [rbp + 0x278], 0x26
14147325a: mov qword ptr [rbp + 0x280], rcx
141473261: mov qword ptr [rbp + 0x288], 0x26
14147326c: mov qword ptr [rbp + 0x290], rax
141473273: lea rcx, [rbp + 0x160]
14147327a: lea r8, [rbp + 0x270]
141473281: mov edx, 3
141473286: call 0x1404b1430 ; log::__private_api::log<tuple$<>,log::__private_api::GlobalLogger>
14147328b: jmp 0x141471cc1
141473290: mov rdi, qword ptr [rbp + 0x38]
141473294: movaps xmm7, xmmword ptr [rbp - 0x20]
141473298: movaps xmm9, xmmword ptr [rbp - 0x10]
14147329d: movss xmm15, dword ptr [rbp + 0x134]
1414732a6: mov eax, dword ptr [rbp + 0x304]
1414732ac: lea r14, [rbp + 0x250]
1414732b3: jmp 0x141471c23
1414732b8: mov byte ptr [rbp + 0x337], 1
1414732bf: lea rcx, [rip + 0x17f1632]
1414732c6: call 0x1428d9310 ; core::option::unwrap_failed
1414732cb: jmp 0x141473383
1414732d0: mov byte ptr [rbp + 0x337], 1
1414732d7: lea rcx, [rip + 0x17f1632]
1414732de: call 0x1428d9310 ; core::option::unwrap_failed
1414732e3: jmp 0x141473383
1414732e8: mov byte ptr [rbp + 0x337], 1
1414732ef: lea rcx, [rip + 0x17f1632]
1414732f6: call 0x1428d9310 ; core::option::unwrap_failed
1414732fb: jmp 0x141473383
141473300: lea rcx, [rip + 0x17ebd71]
141473307: call 0x1428d9310 ; core::option::unwrap_failed
14147330c: jmp 0x141473383
14147330e: mov byte ptr [rbp + 0x337], 1
141473315: lea rcx, [rip + 0x17f157c]
14147331c: call 0x1428d9310 ; core::option::unwrap_failed
141473321: jmp 0x141473383
141473323: mov byte ptr [rbp + 0x337], 1
14147332a: lea rcx, [rip + 0x17f157f]
141473331: call 0x1428d9310 ; core::option::unwrap_failed
141473336: jmp 0x141473383
141473338: mov byte ptr [rbp + 0x335], 0
14147333f: lea rcx, [rip + 0x167e0b2]
141473346: call 0x1428d9310 ; core::option::unwrap_failed
14147334b: jmp 0x141473383
14147334d: lea rcx, [rip + 0x167e0a4]
141473354: call 0x1428d9310 ; core::option::unwrap_failed
141473359: jmp 0x141473383
14147335b: mov byte ptr [rbp + 0x337], 1
141473362: lea rcx, [rip + 0x17f155f]
141473369: call 0x1428d9310 ; core::option::unwrap_failed
14147336e: jmp 0x141473383
141473370: mov byte ptr [rbp + 0x337], 1
141473377: lea rcx, [rip + 0x17f1562]
14147337e: call 0x1428d9310 ; core::option::unwrap_failed
141473383: ud2 
141473385: nop word ptr cs:[rax + rax]
141473390: mov qword ptr [rsp + 0x10], rdx
141473395: push rbp
141473396: push r15
141473398: push r14
14147339a: push r13
14147339c: push r12
14147339e: push rsi
14147339f: push rdi
1414733a0: push rbx
1414733a1: sub rsp, 0xd8
1414733a8: lea rbp, [rdx + 0x80]
1414733af: movaps xmmword ptr [rsp + 0x30], xmm15
1414733b5: movdqa xmmword ptr [rsp + 0x40], xmm14
1414733bc: movaps xmmword ptr [rsp + 0x50], xmm13
1414733c2: movaps xmmword ptr [rsp + 0x60], xmm12
1414733c8: movaps xmmword ptr [rsp + 0x70], xmm11
1414733ce: movaps xmmword ptr [rsp + 0x80], xmm10
1414733d7: movaps xmmword ptr [rsp + 0x90], xmm9
1414733e0: movaps xmmword ptr [rsp + 0xa0], xmm8
1414733e9: movaps xmmword ptr [rsp + 0xb0], xmm7
1414733f1: movaps xmmword ptr [rsp + 0xc0], xmm6
1414733f9: cmp byte ptr [rbp + 0x32e], 0
141473400: jne 0x14147345f
141473402: movaps xmm6, xmmword ptr [rsp + 0xc0]
14147340a: movaps xmm7, xmmword ptr [rsp + 0xb0]
141473412: movaps xmm8, xmmword ptr [rsp + 0xa0]
14147341b: movaps xmm9, xmmword ptr [rsp + 0x90]
141473424: movaps xmm10, xmmword ptr [rsp + 0x80]
14147342d: movaps xmm11, xmmword ptr [rsp + 0x70]
141473433: movaps xmm12, xmmword ptr [rsp + 0x60]
141473439: movaps xmm13, xmmword ptr [rsp + 0x50]
14147343f: movaps xmm14, xmmword ptr [rsp + 0x40]
141473445: movaps xmm15, xmmword ptr [rsp + 0x30]
14147344b: add rsp, 0xd8
141473452: pop rbx
141473453: pop rdi
141473454: pop rsi
141473455: pop r12
141473457: pop r13
141473459: pop r14
14147345b: pop r15
14147345d: pop rbp
14147345e: ret 
14147345f: lea rcx, [rbp + 0x250]
141473466: call 0x1405e7a00 ; core::ptr::drop_in_place<std::io::error::Error>
14147346b: jmp 0x141473402
14147346d: nop dword ptr [rax]
141473470: mov qword ptr [rsp + 0x10], rdx
141473475: push rbp
141473476: push r15
141473478: push r14
14147347a: push r13
14147347c: push r12
14147347e: push rsi
14147347f: push rdi
141473480: push rbx
141473481: sub rsp, 0xd8
141473488: lea rbp, [rdx + 0x80]
14147348f: movaps xmmword ptr [rsp + 0x30], xmm15
141473495: movdqa xmmword ptr [rsp + 0x40], xmm14
14147349c: movaps xmmword ptr [rsp + 0x50], xmm13
1414734a2: movaps xmmword ptr [rsp + 0x60], xmm12
1414734a8: movaps xmmword ptr [rsp + 0x70], xmm11
1414734ae: movaps xmmword ptr [rsp + 0x80], xmm10
1414734b7: movaps xmmword ptr [rsp + 0x90], xmm9
1414734c0: movaps xmmword ptr [rsp + 0xa0], xmm8
1414734c9: movaps xmmword ptr [rsp + 0xb0], xmm7
1414734d1: movaps xmmword ptr [rsp + 0xc0], xmm6
1414734d9: mov rdx, qword ptr [rbp + 0xf0]
1414734e0: test rdx, rdx
1414734e3: je 0x1414734f7
1414734e5: mov r8d, 1
1414734eb: mov rcx, qword ptr [rbp + 0xf8]
1414734f2: call 0x140613c20 ; 0x140613c20
1414734f7: movaps xmm6, xmmword ptr [rsp + 0xc0]
1414734ff: movaps xmm7, xmmword ptr [rsp + 0xb0]
141473507: movaps xmm8, xmmword ptr [rsp + 0xa0]
141473510: movaps xmm9, xmmword ptr [rsp + 0x90]
141473519: movaps xmm10, xmmword ptr [rsp + 0x80]
141473522: movaps xmm11, xmmword ptr [rsp + 0x70]
141473528: movaps xmm12, xmmword ptr [rsp + 0x60]
14147352e: movaps xmm13, xmmword ptr [rsp + 0x50]
141473534: movaps xmm14, xmmword ptr [rsp + 0x40]
14147353a: movaps xmm15, xmmword ptr [rsp + 0x30]
141473540: add rsp, 0xd8
141473547: pop rbx
141473548: pop rdi
141473549: pop rsi
14147354a: pop r12
14147354c: pop r13
14147354e: pop r14
141473550: pop r15
141473552: pop rbp
141473553: ret 
141473554: nop word ptr cs:[rax + rax]
141473560: mov qword ptr [rsp + 0x10], rdx
141473565: push rbp
141473566: push r15
141473568: push r14
14147356a: push r13
14147356c: push r12
14147356e: push rsi
14147356f: push rdi
141473570: push rbx
141473571: sub rsp, 0xd8
141473578: lea rbp, [rdx + 0x80]
14147357f: movaps xmmword ptr [rsp + 0x30], xmm15
141473585: movdqa xmmword ptr [rsp + 0x40], xmm14
14147358c: movaps xmmword ptr [rsp + 0x50], xmm13
141473592: movaps xmmword ptr [rsp + 0x60], xmm12
141473598: movaps xmmword ptr [rsp + 0x70], xmm11
14147359e: movaps xmmword ptr [rsp + 0x80], xmm10
1414735a7: movaps xmmword ptr [rsp + 0x90], xmm9
1414735b0: movaps xmmword ptr [rsp + 0xa0], xmm8
1414735b9: movaps xmmword ptr [rsp + 0xb0], xmm7
1414735c1: movaps xmmword ptr [rsp + 0xc0], xmm6
1414735c9: cmp byte ptr [rbp + 0x32d], 0
1414735d0: je 0x1414735de
1414735d2: lea rcx, [rbp + 0x250]
1414735d9: call 0x1405e88a0 ; core::ptr::drop_in_place<serde_json::error::Error>
1414735de: movaps xmm6, xmmword ptr [rsp + 0xc0]
1414735e6: movaps xmm7, xmmword ptr [rsp + 0xb0]
1414735ee: movaps xmm8, xmmword ptr [rsp + 0xa0]
1414735f7: movaps xmm9, xmmword ptr [rsp + 0x90]
141473600: movaps xmm10, xmmword ptr [rsp + 0x80]
141473609: movaps xmm11, xmmword ptr [rsp + 0x70]
14147360f: movaps xmm12, xmmword ptr [rsp + 0x60]
141473615: movaps xmm13, xmmword ptr [rsp + 0x50]
14147361b: movdqa xmm14, xmmword ptr [rsp + 0x40]
141473622: movaps xmm15, xmmword ptr [rsp + 0x30]
141473628: add rsp, 0xd8
14147362f: pop rbx
141473630: pop rdi
141473631: pop rsi
141473632: pop r12
141473634: pop r13
141473636: pop r14
141473638: pop r15
14147363a: pop rbp
14147363b: ret 
14147363c: nop dword ptr [rax]
141473640: mov qword ptr [rsp + 0x10], rdx
141473645: push rbp
141473646: push r15
141473648: push r14
14147364a: push r13
14147364c: push r12
14147364e: push rsi
14147364f: push rdi
141473650: push rbx
141473651: sub rsp, 0xd8
141473658: lea rbp, [rdx + 0x80]
14147365f: movaps xmmword ptr [rsp + 0x30], xmm15
141473665: movdqa xmmword ptr [rsp + 0x40], xmm14
14147366c: movaps xmmword ptr [rsp + 0x50], xmm13
141473672: movaps xmmword ptr [rsp + 0x60], xmm12
141473678: movaps xmmword ptr [rsp + 0x70], xmm11
14147367e: movaps xmmword ptr [rsp + 0x80], xmm10
141473687: movaps xmmword ptr [rsp + 0x90], xmm9
141473690: movaps xmmword ptr [rsp + 0xa0], xmm8
141473699: movaps xmmword ptr [rsp + 0xb0], xmm7
1414736a1: movaps xmmword ptr [rsp + 0xc0], xmm6
1414736a9: lea rcx, [rbp + 0x138]
1414736b0: call 0x14085da90 ; core::ptr::drop_in_place<enum2$<serde_json::value::Value> >
1414736b5: movaps xmm6, xmmword ptr [rsp + 0xc0]
1414736bd: movaps xmm7, xmmword ptr [rsp + 0xb0]
1414736c5: movaps xmm8, xmmword ptr [rsp + 0xa0]
1414736ce: movaps xmm9, xmmword ptr [rsp + 0x90]
1414736d7: movaps xmm10, xmmword ptr [rsp + 0x80]
1414736e0: movaps xmm11, xmmword ptr [rsp + 0x70]
1414736e6: movaps xmm12, xmmword ptr [rsp + 0x60]
1414736ec: movaps xmm13, xmmword ptr [rsp + 0x50]
1414736f2: movdqa xmm14, xmmword ptr [rsp + 0x40]
1414736f9: movaps xmm15, xmmword ptr [rsp + 0x30]
1414736ff: add rsp, 0xd8
141473706: pop rbx
141473707: pop rdi
141473708: pop rsi
141473709: pop r12
14147370b: pop r13
14147370d: pop r14
14147370f: pop r15
141473711: pop rbp
141473712: ret 
141473713: nop word ptr cs:[rax + rax]
141473720: mov qword ptr [rsp + 0x10], rdx
141473725: push rbp
141473726: push r15
141473728: push r14
14147372a: push r13
14147372c: push r12
14147372e: push rsi
14147372f: push rdi
141473730: push rbx
141473731: sub rsp, 0xd8
141473738: lea rbp, [rdx + 0x80]
14147373f: movaps xmmword ptr [rsp + 0x30], xmm15
141473745: movdqa xmmword ptr [rsp + 0x40], xmm14
14147374c: movaps xmmword ptr [rsp + 0x50], xmm13
141473752: movaps xmmword ptr [rsp + 0x60], xmm12
141473758: movaps xmmword ptr [rsp + 0x70], xmm11
14147375e: movaps xmmword ptr [rsp + 0x80], xmm10
141473767: movaps xmmword ptr [rsp + 0x90], xmm9
141473770: movaps xmmword ptr [rsp + 0xa0], xmm8
141473779: movaps xmmword ptr [rsp + 0xb0], xmm7
141473781: movaps xmmword ptr [rsp + 0xc0], xmm6
141473789: cmp byte ptr [rbp + 0x32c], 0
141473790: je 0x14147379e
141473792: lea rcx, [rbp + 0x250]
141473799: call 0x1405e88a0 ; core::ptr::drop_in_place<serde_json::error::Error>
14147379e: movaps xmm6, xmmword ptr [rsp + 0xc0]
1414737a6: movaps xmm7, xmmword ptr [rsp + 0xb0]
1414737ae: movaps xmm8, xmmword ptr [rsp + 0xa0]
1414737b7: movaps xmm9, xmmword ptr [rsp + 0x90]
1414737c0: movaps xmm10, xmmword ptr [rsp + 0x80]
1414737c9: movaps xmm11, xmmword ptr [rsp + 0x70]
1414737cf: movaps xmm12, xmmword ptr [rsp + 0x60]
1414737d5: movaps xmm13, xmmword ptr [rsp + 0x50]
1414737db: movdqa xmm14, xmmword ptr [rsp + 0x40]
1414737e2: movaps xmm15, xmmword ptr [rsp + 0x30]
1414737e8: add rsp, 0xd8
1414737ef: pop rbx
1414737f0: pop rdi
1414737f1: pop rsi
1414737f2: pop r12
1414737f4: pop r13
1414737f6: pop r14
1414737f8: pop r15
1414737fa: pop rbp
1414737fb: ret 
1414737fc: nop dword ptr [rax]
141473800: mov qword ptr [rsp + 0x10], rdx
141473805: push rbp
141473806: push r15
141473808: push r14
14147380a: push r13
14147380c: push r12
14147380e: push rsi
14147380f: push rdi
141473810: push rbx
141473811: sub rsp, 0xd8
141473818: lea rbp, [rdx + 0x80]
14147381f: movaps xmmword ptr [rsp + 0x30], xmm15
141473825: movdqa xmmword ptr [rsp + 0x40], xmm14
14147382c: movaps xmmword ptr [rsp + 0x50], xmm13
141473832: movaps xmmword ptr [rsp + 0x60], xmm12
141473838: movaps xmmword ptr [rsp + 0x70], xmm11
14147383e: movaps xmmword ptr [rsp + 0x80], xmm10
141473847: movaps xmmword ptr [rsp + 0x90], xmm9
141473850: movaps xmmword ptr [rsp + 0xa0], xmm8
141473859: movaps xmmword ptr [rsp + 0xb0], xmm7
141473861: movaps xmmword ptr [rsp + 0xc0], xmm6
141473869: mov rax, qword ptr [rbp + 0x40]
14147386d: lea rcx, [rax*2]
141473875: test rcx, rcx
141473878: je 0x141473891
14147387a: mov rcx, qword ptr [rbp + 0x48]
14147387e: shl rax, 3
141473882: lea rdx, [rax + rax*4]
141473886: mov r8d, 4
14147388c: call 0x140613c20 ; 0x140613c20
141473891: movaps xmm6, xmmword ptr [rsp + 0xc0]
141473899: movaps xmm7, xmmword ptr [rsp + 0xb0]
1414738a1: movaps xmm8, xmmword ptr [rsp + 0xa0]
1414738aa: movaps xmm9, xmmword ptr [rsp + 0x90]
1414738b3: movaps xmm10, xmmword ptr [rsp + 0x80]
1414738bc: movaps xmm11, xmmword ptr [rsp + 0x70]
1414738c2: movaps xmm12, xmmword ptr [rsp + 0x60]
1414738c8: movaps xmm13, xmmword ptr [rsp + 0x50]
1414738ce: movdqa xmm14, xmmword ptr [rsp + 0x40]
1414738d5: movaps xmm15, xmmword ptr [rsp + 0x30]
1414738db: add rsp, 0xd8
1414738e2: pop rbx
1414738e3: pop rdi
1414738e4: pop rsi
1414738e5: pop r12
1414738e7: pop r13
1414738e9: pop r14
1414738eb: pop r15
1414738ed: pop rbp
1414738ee: ret 
1414738ef: nop 
1414738f0: mov qword ptr [rsp + 0x10], rdx
1414738f5: push rbp
1414738f6: push r15
1414738f8: push r14
1414738fa: push r13
1414738fc: push r12
1414738fe: push rsi
1414738ff: push rdi
141473900: push rbx
141473901: sub rsp, 0xd8
141473908: lea rbp, [rdx + 0x80]
14147390f: movaps xmmword ptr [rsp + 0x30], xmm15
141473915: movdqa xmmword ptr [rsp + 0x40], xmm14
14147391c: movaps xmmword ptr [rsp + 0x50], xmm13
141473922: movaps xmmword ptr [rsp + 0x60], xmm12
141473928: movaps xmmword ptr [rsp + 0x70], xmm11
14147392e: movaps xmmword ptr [rsp + 0x80], xmm10
141473937: movaps xmmword ptr [rsp + 0x90], xmm9
141473940: movaps xmmword ptr [rsp + 0xa0], xmm8
141473949: movaps xmmword ptr [rsp + 0xb0], xmm7
141473951: movaps xmmword ptr [rsp + 0xc0], xmm6
141473959: cmp dword ptr [rbp + 0x320], 0
141473960: je 0x14147396e
141473962: lea rcx, [rbp + 0x168]
141473969: call 0x1405e88a0 ; core::ptr::drop_in_place<serde_json::error::Error>
14147396e: movaps xmm6, xmmword ptr [rsp + 0xc0]
141473976: movaps xmm7, xmmword ptr [rsp + 0xb0]
14147397e: movaps xmm8, xmmword ptr [rsp + 0xa0]
141473987: movaps xmm9, xmmword ptr [rsp + 0x90]
141473990: movaps xmm10, xmmword ptr [rsp + 0x80]
141473999: movaps xmm11, xmmword ptr [rsp + 0x70]
14147399f: movaps xmm12, xmmword ptr [rsp + 0x60]
1414739a5: movaps xmm13, xmmword ptr [rsp + 0x50]
1414739ab: movdqa xmm14, xmmword ptr [rsp + 0x40]
1414739b2: movaps xmm15, xmmword ptr [rsp + 0x30]
1414739b8: add rsp, 0xd8
1414739bf: pop rbx
1414739c0: pop rdi
1414739c1: pop rsi
1414739c2: pop r12
1414739c4: pop r13
1414739c6: pop r14
1414739c8: pop r15
1414739ca: pop rbp
1414739cb: ret 
1414739cc: nop dword ptr [rax]
1414739d0: mov qword ptr [rsp + 0x10], rdx
1414739d5: push rbp
1414739d6: push r15
1414739d8: push r14
1414739da: push r13
1414739dc: push r12
1414739de: push rsi
1414739df: push rdi
1414739e0: push rbx
1414739e1: sub rsp, 0xd8
1414739e8: lea rbp, [rdx + 0x80]
1414739ef: movaps xmmword ptr [rsp + 0x30], xmm15
1414739f5: movdqa xmmword ptr [rsp + 0x40], xmm14
1414739fc: movaps xmmword ptr [rsp + 0x50], xmm13
141473a02: movaps xmmword ptr [rsp + 0x60], xmm12
141473a08: movaps xmmword ptr [rsp + 0x70], xmm11
141473a0e: movaps xmmword ptr [rsp + 0x80], xmm10
141473a17: movaps xmmword ptr [rsp + 0x90], xmm9
141473a20: movaps xmmword ptr [rsp + 0xa0], xmm8
141473a29: movaps xmmword ptr [rsp + 0xb0], xmm7
141473a31: movaps xmmword ptr [rsp + 0xc0], xmm6
141473a39: lea rcx, [rbp + 0x58]
141473a3d: call 0x14088acf0 ; core::ptr::drop_in_place<country_core::resources::history::History>
141473a42: movaps xmm6, xmmword ptr [rsp + 0xc0]
141473a4a: movaps xmm7, xmmword ptr [rsp + 0xb0]
141473a52: movaps xmm8, xmmword ptr [rsp + 0xa0]
141473a5b: movaps xmm9, xmmword ptr [rsp + 0x90]
141473a64: movaps xmm10, xmmword ptr [rsp + 0x80]
141473a6d: movaps xmm11, xmmword ptr [rsp + 0x70]
141473a73: movaps xmm12, xmmword ptr [rsp + 0x60]
141473a79: movaps xmm13, xmmword ptr [rsp + 0x50]
141473a7f: movdqa xmm14, xmmword ptr [rsp + 0x40]
141473a86: movaps xmm15, xmmword ptr [rsp + 0x30]
141473a8c: add rsp, 0xd8
141473a93: pop rbx
141473a94: pop rdi
141473a95: pop rsi
141473a96: pop r12
141473a98: pop r13
141473a9a: pop r14
141473a9c: pop r15
141473a9e: pop rbp
141473a9f: ret 
141473aa0: mov qword ptr [rsp + 0x10], rdx
141473aa5: push rbp
141473aa6: push r15
141473aa8: push r14
141473aaa: push r13
141473aac: push r12
141473aae: push rsi
141473aaf: push rdi
141473ab0: push rbx
141473ab1: sub rsp, 0xd8
141473ab8: lea rbp, [rdx + 0x80]
141473abf: movaps xmmword ptr [rsp + 0x30], xmm15
141473ac5: movdqa xmmword ptr [rsp + 0x40], xmm14
141473acc: movaps xmmword ptr [rsp + 0x50], xmm13
141473ad2: movaps xmmword ptr [rsp + 0x60], xmm12
141473ad8: movaps xmmword ptr [rsp + 0x70], xmm11
141473ade: movaps xmmword ptr [rsp + 0x80], xmm10
141473ae7: movaps xmmword ptr [rsp + 0x90], xmm9
141473af0: movaps xmmword ptr [rsp + 0xa0], xmm8
141473af9: movaps xmmword ptr [rsp + 0xb0], xmm7
141473b01: movaps xmmword ptr [rsp + 0xc0], xmm6
141473b09: mov rdx, qword ptr [rbp + 0x250]
141473b10: test rdx, rdx
141473b13: je 0x141473b2b
141473b15: mov rcx, qword ptr [rbp + 0x258]
141473b1c: shl rdx, 4
141473b20: mov r8d, 8
141473b26: call 0x140613c20 ; 0x140613c20
141473b2b: movaps xmm6, xmmword ptr [rsp + 0xc0]
141473b33: movaps xmm7, xmmword ptr [rsp + 0xb0]
141473b3b: movaps xmm8, xmmword ptr [rsp + 0xa0]
141473b44: movaps xmm9, xmmword ptr [rsp + 0x90]
141473b4d: movaps xmm10, xmmword ptr [rsp + 0x80]
141473b56: movaps xmm11, xmmword ptr [rsp + 0x70]
141473b5c: movaps xmm12, xmmword ptr [rsp + 0x60]
141473b62: movaps xmm13, xmmword ptr [rsp + 0x50]
141473b68: movdqa xmm14, xmmword ptr [rsp + 0x40]
141473b6f: movaps xmm15, xmmword ptr [rsp + 0x30]
141473b75: add rsp, 0xd8
141473b7c: pop rbx
141473b7d: pop rdi
141473b7e: pop rsi
141473b7f: pop r12
141473b81: pop r13
141473b83: pop r14
141473b85: pop r15
141473b87: pop rbp
141473b88: ret 
141473b89: nop dword ptr [rax]
141473b90: mov qword ptr [rsp + 0x10], rdx
141473b95: push rbp
141473b96: push r15
141473b98: push r14
141473b9a: push r13
141473b9c: push r12
141473b9e: push rsi
141473b9f: push rdi
141473ba0: push rbx
141473ba1: sub rsp, 0xd8
141473ba8: lea rbp, [rdx + 0x80]
141473baf: movaps xmmword ptr [rsp + 0x30], xmm15
141473bb5: movdqa xmmword ptr [rsp + 0x40], xmm14
141473bbc: movaps xmmword ptr [rsp + 0x50], xmm13
141473bc2: movaps xmmword ptr [rsp + 0x60], xmm12
141473bc8: movaps xmmword ptr [rsp + 0x70], xmm11
141473bce: movaps xmmword ptr [rsp + 0x80], xmm10
141473bd7: movaps xmmword ptr [rsp + 0x90], xmm9
141473be0: movaps xmmword ptr [rsp + 0xa0], xmm8
141473be9: movaps xmmword ptr [rsp + 0xb0], xmm7
141473bf1: movaps xmmword ptr [rsp + 0xc0], xmm6
141473bf9: cmp qword ptr [rbp + 0x320], -1
141473c01: je 0x141473c27
141473c03: mov rax, qword ptr [rbp + 0x318]
141473c0a: lock dec qword ptr [rax]
141473c0e: jne 0x141473c27
141473c10: mov edx, 0x40
141473c15: mov r8d, 8
141473c1b: mov rcx, qword ptr [rbp + 0x320]
141473c22: call 0x140613c20 ; 0x140613c20
141473c27: mov byte ptr [rbp + 0x337], 1
141473c2e: movaps xmm6, xmmword ptr [rsp + 0xc0]
141473c36: movaps xmm7, xmmword ptr [rsp + 0xb0]
141473c3e: movaps xmm8, xmmword ptr [rsp + 0xa0]
141473c47: movaps xmm9, xmmword ptr [rsp + 0x90]
141473c50: movaps xmm10, xmmword ptr [rsp + 0x80]
141473c59: movaps xmm11, xmmword ptr [rsp + 0x70]
141473c5f: movaps xmm12, xmmword ptr [rsp + 0x60]
141473c65: movaps xmm13, xmmword ptr [rsp + 0x50]
141473c6b: movaps xmm14, xmmword ptr [rsp + 0x40]
141473c71: movaps xmm15, xmmword ptr [rsp + 0x30]
141473c77: add rsp, 0xd8
141473c7e: pop rbx
141473c7f: pop rdi
141473c80: pop rsi
141473c81: pop r12
141473c83: pop r13
141473c85: pop r14
141473c87: pop r15
141473c89: pop rbp
141473c8a: ret 
141473c8b: nop dword ptr [rax + rax]
141473c90: mov qword ptr [rsp + 0x10], rdx
141473c95: push rbp
141473c96: push r15
141473c98: push r14
141473c9a: push r13
141473c9c: push r12
141473c9e: push rsi
141473c9f: push rdi
141473ca0: push rbx
141473ca1: sub rsp, 0xd8
141473ca8: lea rbp, [rdx + 0x80]
141473caf: movaps xmmword ptr [rsp + 0x30], xmm15
141473cb5: movaps xmmword ptr [rsp + 0x40], xmm14
141473cbb: movaps xmmword ptr [rsp + 0x50], xmm13
141473cc1: movaps xmmword ptr [rsp + 0x60], xmm12
141473cc7: movaps xmmword ptr [rsp + 0x70], xmm11
141473ccd: movaps xmmword ptr [rsp + 0x80], xmm10
141473cd6: movaps xmmword ptr [rsp + 0x90], xmm9
141473cdf: movdqa xmmword ptr [rsp + 0xa0], xmm8
141473ce9: movaps xmmword ptr [rsp + 0xb0], xmm7
141473cf1: movaps xmmword ptr [rsp + 0xc0], xmm6
141473cf9: cmp byte ptr [rbp + 0x335], 0
141473d00: je 0x141473d79
141473d02: mov byte ptr [rbp + 0x337], 1
141473d09: lea rcx, [rbp + 0x270]
141473d10: call 0x140879b30 ; core::ptr::drop_in_place<alloc::sync::UniqueArcUninit<country_core::resources::rt::RtMesh,alloc::alloc::Global> >
141473d15: mov byte ptr [rbp + 0x337], 1
141473d1c: movaps xmm6, xmmword ptr [rsp + 0xc0]
141473d24: movaps xmm7, xmmword ptr [rsp + 0xb0]
141473d2c: movaps xmm8, xmmword ptr [rsp + 0xa0]
141473d35: movaps xmm9, xmmword ptr [rsp + 0x90]
141473d3e: movaps xmm10, xmmword ptr [rsp + 0x80]
141473d47: movaps xmm11, xmmword ptr [rsp + 0x70]
141473d4d: movaps xmm12, xmmword ptr [rsp + 0x60]
141473d53: movaps xmm13, xmmword ptr [rsp + 0x50]
141473d59: movaps xmm14, xmmword ptr [rsp + 0x40]
141473d5f: movaps xmm15, xmmword ptr [rsp + 0x30]
141473d65: add rsp, 0xd8
141473d6c: pop rbx
141473d6d: pop rdi
141473d6e: pop rsi
141473d6f: pop r12
141473d71: pop r13
141473d73: pop r14
141473d75: pop r15
141473d77: pop rbp
141473d78: ret 
141473d79: mov byte ptr [rbp + 0x337], 1
141473d80: movaps xmm6, xmmword ptr [rsp + 0xc0]
141473d88: movaps xmm7, xmmword ptr [rsp + 0xb0]
141473d90: movaps xmm8, xmmword ptr [rsp + 0xa0]
141473d99: movaps xmm9, xmmword ptr [rsp + 0x90]
141473da2: movaps xmm10, xmmword ptr [rsp + 0x80]
141473dab: movaps xmm11, xmmword ptr [rsp + 0x70]
141473db1: movaps xmm12, xmmword ptr [rsp + 0x60]
141473db7: movaps xmm13, xmmword ptr [rsp + 0x50]
141473dbd: movaps xmm14, xmmword ptr [rsp + 0x40]
141473dc3: movaps xmm15, xmmword ptr [rsp + 0x30]
141473dc9: add rsp, 0xd8
141473dd0: pop rbx
141473dd1: pop rdi
141473dd2: pop rsi
141473dd3: pop r12
141473dd5: pop r13
141473dd7: pop r14
141473dd9: pop r15
141473ddb: pop rbp
141473ddc: ret 
141473ddd: nop dword ptr [rax]
141473de0: mov qword ptr [rsp + 0x10], rdx
141473de5: push rbp
141473de6: push r15
141473de8: push r14
141473dea: push r13
141473dec: push r12
141473dee: push rsi
141473def: push rdi
141473df0: push rbx
141473df1: sub rsp, 0xd8
141473df8: lea rbp, [rdx + 0x80]
141473dff: movaps xmmword ptr [rsp + 0x30], xmm15
141473e05: movdqa xmmword ptr [rsp + 0x40], xmm14
141473e0c: movaps xmmword ptr [rsp + 0x50], xmm13
141473e12: movaps xmmword ptr [rsp + 0x60], xmm12
141473e18: movaps xmmword ptr [rsp + 0x70], xmm11
141473e1e: movaps xmmword ptr [rsp + 0x80], xmm10
141473e27: movaps xmmword ptr [rsp + 0x90], xmm9
141473e30: movaps xmmword ptr [rsp + 0xa0], xmm8
141473e39: movaps xmmword ptr [rsp + 0xb0], xmm7
141473e41: movaps xmmword ptr [rsp + 0xc0], xmm6
141473e49: movabs rax, 0x8000000000000003
141473e53: mov rdx, qword ptr [rbp + 0x308]
141473e5a: mov qword ptr [rdx + 0x10], rax
141473e5e: mov rax, qword ptr [rbp + 0x100]
141473e65: mov qword ptr [rdx + 0x18], rax
141473e69: mov eax, dword ptr [rbp + 0x158]
141473e6f: mov dword ptr [rdx + 0x20], eax
141473e72: mov rax, qword ptr [rbp + 0x170]
141473e79: mov rcx, qword ptr [rbp + 0x318]
141473e80: mov qword ptr [rcx + 0x10], rax
141473e84: movups xmm0, xmmword ptr [rbp + 0x160]
141473e8b: movups xmmword ptr [rcx], xmm0
141473e8e: mov eax, dword ptr [rbp + 0x320]
141473e94: mov dword ptr [rdx + 0x3c], eax
141473e97: movups xmm0, xmmword ptr [rbp + 0x270]
141473e9e: movups xmmword ptr [rdx + 0x40], xmm0
141473ea2: mov rax, qword ptr [rbp + 0x280]
141473ea9: mov qword ptr [rdx + 0x50], rax
141473ead: mov byte ptr [rbp + 0x337], 1
141473eb4: movaps xmm6, xmmword ptr [rsp + 0xc0]
141473ebc: movaps xmm7, xmmword ptr [rsp + 0xb0]
141473ec4: movaps xmm8, xmmword ptr [rsp + 0xa0]
141473ecd: movaps xmm9, xmmword ptr [rsp + 0x90]
141473ed6: movaps xmm10, xmmword ptr [rsp + 0x80]
141473edf: movaps xmm11, xmmword ptr [rsp + 0x70]
141473ee5: movaps xmm12, xmmword ptr [rsp + 0x60]
141473eeb: movaps xmm13, xmmword ptr [rsp + 0x50]
141473ef1: movaps xmm14, xmmword ptr [rsp + 0x40]
141473ef7: movaps xmm15, xmmword ptr [rsp + 0x30]
141473efd: add rsp, 0xd8
141473f04: pop rbx
141473f05: pop rdi
141473f06: pop rsi
141473f07: pop r12
141473f09: pop r13
141473f0b: pop r14
141473f0d: pop r15
141473f0f: pop rbp
141473f10: ret 
141473f11: nop word ptr cs:[rax + rax]
141473f20: mov qword ptr [rsp + 0x10], rdx
141473f25: push rbp
141473f26: push r15
141473f28: push r14
141473f2a: push r13
141473f2c: push r12
141473f2e: push rsi
141473f2f: push rdi
141473f30: push rbx
141473f31: sub rsp, 0xd8
141473f38: lea rbp, [rdx + 0x80]
141473f3f: movaps xmmword ptr [rsp + 0x30], xmm15
141473f45: movdqa xmmword ptr [rsp + 0x40], xmm14
141473f4c: movaps xmmword ptr [rsp + 0x50], xmm13
141473f52: movaps xmmword ptr [rsp + 0x60], xmm12
141473f58: movaps xmmword ptr [rsp + 0x70], xmm11
141473f5e: movaps xmmword ptr [rsp + 0x80], xmm10
141473f67: movaps xmmword ptr [rsp + 0x90], xmm9
141473f70: movaps xmmword ptr [rsp + 0xa0], xmm8
141473f79: movaps xmmword ptr [rsp + 0xb0], xmm7
141473f81: movaps xmmword ptr [rsp + 0xc0], xmm6
141473f89: cmp byte ptr [rbp + 0x337], 0
141473f90: je 0x141473ffc
141473f92: lea rcx, [rbp + 0xd8]
141473f99: call 0x14088f8b0 ; core::ptr::drop_in_place<country_core::resources::history::HistoryMutationToken>
141473f9e: movaps xmm6, xmmword ptr [rsp + 0xc0]
141473fa6: movaps xmm7, xmmword ptr [rsp + 0xb0]
141473fae: movaps xmm8, xmmword ptr [rsp + 0xa0]
141473fb7: movaps xmm9, xmmword ptr [rsp + 0x90]
141473fc0: movaps xmm10, xmmword ptr [rsp + 0x80]
141473fc9: movaps xmm11, xmmword ptr [rsp + 0x70]
141473fcf: movaps xmm12, xmmword ptr [rsp + 0x60]
141473fd5: movaps xmm13, xmmword ptr [rsp + 0x50]
141473fdb: movdqa xmm14, xmmword ptr [rsp + 0x40]
141473fe2: movaps xmm15, xmmword ptr [rsp + 0x30]
141473fe8: add rsp, 0xd8
141473fef: pop rbx
141473ff0: pop rdi
141473ff1: pop rsi
141473ff2: pop r12
141473ff4: pop r13
141473ff6: pop r14
141473ff8: pop r15
141473ffa: pop rbp
141473ffb: ret 
141473ffc: movaps xmm6, xmmword ptr [rsp + 0xc0]
141474004: movaps xmm7, xmmword ptr [rsp + 0xb0]
14147400c: movaps xmm8, xmmword ptr [rsp + 0xa0]
141474015: movaps xmm9, xmmword ptr [rsp + 0x90]
14147401e: movaps xmm10, xmmword ptr [rsp + 0x80]
141474027: movaps xmm11, xmmword ptr [rsp + 0x70]
14147402d: movaps xmm12, xmmword ptr [rsp + 0x60]
141474033: movaps xmm13, xmmword ptr [rsp + 0x50]
141474039: movdqa xmm14, xmmword ptr [rsp + 0x40]
141474040: movaps xmm15, xmmword ptr [rsp + 0x30]
141474046: add rsp, 0xd8
14147404d: pop rbx
14147404e: pop rdi
14147404f: pop rsi
141474050: pop r12
141474052: pop r13
141474054: pop r14
141474056: pop r15
141474058: pop rbp
141474059: ret 
14147405a: nop word ptr [rax + rax]
141474060: mov qword ptr [rsp + 0x10], rdx
141474065: push rbp
141474066: push r15
141474068: push r14
14147406a: push r13
14147406c: push r12
14147406e: push rsi
14147406f: push rdi
141474070: push rbx
141474071: sub rsp, 0xd8
141474078: lea rbp, [rdx + 0x80]
14147407f: movaps xmmword ptr [rsp + 0x30], xmm15
141474085: movdqa xmmword ptr [rsp + 0x40], xmm14
14147408c: movaps xmmword ptr [rsp + 0x50], xmm13
141474092: movaps xmmword ptr [rsp + 0x60], xmm12
141474098: movaps xmmword ptr [rsp + 0x70], xmm11
14147409e: movaps xmmword ptr [rsp + 0x80], xmm10
1414740a7: movaps xmmword ptr [rsp + 0x90], xmm9
1414740b0: movaps xmmword ptr [rsp + 0xa0], xmm8
1414740b9: movaps xmmword ptr [rsp + 0xb0], xmm7
1414740c1: movaps xmmword ptr [rsp + 0xc0], xmm6
1414740c9: mov rdx, qword ptr [rbp + 0x140]
1414740d0: test rdx, rdx
1414740d3: je 0x1414740eb
1414740d5: shl rdx, 5
1414740d9: mov r8d, 8
1414740df: mov rcx, qword ptr [rbp + 0x320]
1414740e6: call 0x140613c20 ; 0x140613c20
1414740eb: movaps xmm6, xmmword ptr [rsp + 0xc0]
1414740f3: movaps xmm7, xmmword ptr [rsp + 0xb0]
1414740fb: movaps xmm8, xmmword ptr [rsp + 0xa0]
141474104: movaps xmm9, xmmword ptr [rsp + 0x90]
14147410d: movaps xmm10, xmmword ptr [rsp + 0x80]
141474116: movaps xmm11, xmmword ptr [rsp + 0x70]
14147411c: movaps xmm12, xmmword ptr [rsp + 0x60]
141474122: movaps xmm13, xmmword ptr [rsp + 0x50]
141474128: movdqa xmm14, xmmword ptr [rsp + 0x40]
14147412f: movaps xmm15, xmmword ptr [rsp + 0x30]
141474135: add rsp, 0xd8
14147413c: pop rbx
14147413d: pop rdi
14147413e: pop rsi
14147413f: pop r12
141474141: pop r13
141474143: pop r14
141474145: pop r15
141474147: pop rbp
141474148: ret 
141474149: nop dword ptr [rax]
141474150: mov qword ptr [rsp + 0x10], rdx
141474155: push rbp
141474156: push r15
141474158: push r14
14147415a: push r13
14147415c: push r12
14147415e: push rsi
14147415f: push rdi
141474160: push rbx
141474161: sub rsp, 0xd8
141474168: lea rbp, [rdx + 0x80]
14147416f: movaps xmmword ptr [rsp + 0x30], xmm15
141474175: movdqa xmmword ptr [rsp + 0x40], xmm14
14147417c: movaps xmmword ptr [rsp + 0x50], xmm13
141474182: movaps xmmword ptr [rsp + 0x60], xmm12
141474188: movaps xmmword ptr [rsp + 0x70], xmm11
14147418e: movaps xmmword ptr [rsp + 0x80], xmm10
141474197: movaps xmmword ptr [rsp + 0x90], xmm9
1414741a0: movaps xmmword ptr [rsp + 0xa0], xmm8
1414741a9: movaps xmmword ptr [rsp + 0xb0], xmm7
1414741b1: movaps xmmword ptr [rsp + 0xc0], xmm6
1414741b9: lea rcx, [rbp + 0xd8]
1414741c0: call 0x140894d60 ; core::ptr::drop_in_place<country_core::resources::history::history_chain::HistoryEditChain>
1414741c5: movaps xmm6, xmmword ptr [rsp + 0xc0]
1414741cd: movaps xmm7, xmmword ptr [rsp + 0xb0]
1414741d5: movaps xmm8, xmmword ptr [rsp + 0xa0]
1414741de: movaps xmm9, xmmword ptr [rsp + 0x90]
1414741e7: movaps xmm10, xmmword ptr [rsp + 0x80]
1414741f0: movaps xmm11, xmmword ptr [rsp + 0x70]
1414741f6: movaps xmm12, xmmword ptr [rsp + 0x60]
1414741fc: movaps xmm13, xmmword ptr [rsp + 0x50]
141474202: movdqa xmm14, xmmword ptr [rsp + 0x40]
141474209: movaps xmm15, xmmword ptr [rsp + 0x30]
14147420f: add rsp, 0xd8
141474216: pop rbx
141474217: pop rdi
141474218: pop rsi
141474219: pop r12
14147421b: pop r13
14147421d: pop r14
14147421f: pop r15
141474221: pop rbp
141474222: ret 
141474223: nop word ptr cs:[rax + rax]
141474230: mov qword ptr [rsp + 0x10], rdx
141474235: push rbp
141474236: push r15
141474238: push r14
14147423a: push r13
14147423c: push r12
14147423e: push rsi
14147423f: push rdi
141474240: push rbx
141474241: sub rsp, 0xd8
141474248: lea rbp, [rdx + 0x80]
14147424f: movaps xmmword ptr [rsp + 0x30], xmm15
141474255: movdqa xmmword ptr [rsp + 0x40], xmm14
14147425c: movaps xmmword ptr [rsp + 0x50], xmm13
141474262: movaps xmmword ptr [rsp + 0x60], xmm12
141474268: movaps xmmword ptr [rsp + 0x70], xmm11
14147426e: movaps xmmword ptr [rsp + 0x80], xmm10
141474277: movaps xmmword ptr [rsp + 0x90], xmm9
141474280: movaps xmmword ptr [rsp + 0xa0], xmm8
141474289: movaps xmmword ptr [rsp + 0xb0], xmm7
141474291: movaps xmmword ptr [rsp + 0xc0], xmm6
141474299: lea rcx, [rbp + 0x160]
1414742a0: call 0x14088eb40 ; core::ptr::drop_in_place<country_core::systems::save::save_game::GameSaveData>
1414742a5: movaps xmm6, xmmword ptr [rsp + 0xc0]
1414742ad: movaps xmm7, xmmword ptr [rsp + 0xb0]
1414742b5: movaps xmm8, xmmword ptr [rsp + 0xa0]
1414742be: movaps xmm9, xmmword ptr [rsp + 0x90]
1414742c7: movaps xmm10, xmmword ptr [rsp + 0x80]
1414742d0: movaps xmm11, xmmword ptr [rsp + 0x70]
1414742d6: movaps xmm12, xmmword ptr [rsp + 0x60]
1414742dc: movaps xmm13, xmmword ptr [rsp + 0x50]
1414742e2: movdqa xmm14, xmmword ptr [rsp + 0x40]
1414742e9: movaps xmm15, xmmword ptr [rsp + 0x30]
1414742ef: add rsp, 0xd8
1414742f6: pop rbx
1414742f7: pop rdi
1414742f8: pop rsi
1414742f9: pop r12
1414742fb: pop r13
1414742fd: pop r14
1414742ff: pop r15
141474301: pop rbp
141474302: ret 
141474303: nop word ptr cs:[rax + rax]
141474310: mov qword ptr [rsp + 0x10], rdx
141474315: push rbp
141474316: push r15
141474318: push r14
14147431a: push r13
14147431c: push r12
14147431e: push rsi
14147431f: push rdi
141474320: push rbx
141474321: sub rsp, 0xd8
141474328: lea rbp, [rdx + 0x80]
14147432f: movaps xmmword ptr [rsp + 0x30], xmm15
141474335: movaps xmmword ptr [rsp + 0x40], xmm14
14147433b: movaps xmmword ptr [rsp + 0x50], xmm13
141474341: movaps xmmword ptr [rsp + 0x60], xmm12
141474347: movaps xmmword ptr [rsp + 0x70], xmm11
14147434d: movaps xmmword ptr [rsp + 0x80], xmm10
141474356: movaps xmmword ptr [rsp + 0x90], xmm9
14147435f: movdqa xmmword ptr [rsp + 0xa0], xmm8
141474369: movaps xmmword ptr [rsp + 0xb0], xmm7
141474371: movaps xmmword ptr [rsp + 0xc0], xmm6
141474379: mov rax, qword ptr [rbp + 0x318]
141474380: mov rcx, qword ptr [rbp + 0x308]
141474387: mov qword ptr [rax], rcx
14147438a: mov byte ptr [rbp + 0x335], 0
141474391: movaps xmm6, xmmword ptr [rsp + 0xc0]
141474399: movaps xmm7, xmmword ptr [rsp + 0xb0]
1414743a1: movdqa xmm8, xmmword ptr [rsp + 0xa0]
1414743ab: movaps xmm9, xmmword ptr [rsp + 0x90]
1414743b4: movaps xmm10, xmmword ptr [rsp + 0x80]
1414743bd: movaps xmm11, xmmword ptr [rsp + 0x70]
1414743c3: movaps xmm12, xmmword ptr [rsp + 0x60]
1414743c9: movaps xmm13, xmmword ptr [rsp + 0x50]
1414743cf: movaps xmm14, xmmword ptr [rsp + 0x40]
1414743d5: movaps xmm15, xmmword ptr [rsp + 0x30]
1414743db: add rsp, 0xd8
1414743e2: pop rbx
1414743e3: pop rdi
1414743e4: pop rsi
1414743e5: pop r12
1414743e7: pop r13
1414743e9: pop r14
1414743eb: pop r15
1414743ed: pop rbp
1414743ee: ret 
1414743ef: nop 
1414743f0: mov qword ptr [rsp + 0x10], rdx
1414743f5: push rbp
1414743f6: push r15
1414743f8: push r14
1414743fa: push r13
1414743fc: push r12
1414743fe: push rsi
1414743ff: push rdi
141474400: push rbx
141474401: sub rsp, 0xd8
141474408: lea rbp, [rdx + 0x80]
14147440f: movaps xmmword ptr [rsp + 0x30], xmm15
141474415: movdqa xmmword ptr [rsp + 0x40], xmm14
14147441c: movaps xmmword ptr [rsp + 0x50], xmm13
141474422: movaps xmmword ptr [rsp + 0x60], xmm12
141474428: movaps xmmword ptr [rsp + 0x70], xmm11
14147442e: movaps xmmword ptr [rsp + 0x80], xmm10
141474437: movaps xmmword ptr [rsp + 0x90], xmm9
141474440: movaps xmmword ptr [rsp + 0xa0], xmm8
141474449: movaps xmmword ptr [rsp + 0xb0], xmm7
141474451: movaps xmmword ptr [rsp + 0xc0], xmm6
141474459: lea rcx, [rbp + 0xd8]
141474460: call 0x140d4cc30 ; anyhow::error::impl$6::drop
141474465: movaps xmm6, xmmword ptr [rsp + 0xc0]
14147446d: movaps xmm7, xmmword ptr [rsp + 0xb0]
141474475: movaps xmm8, xmmword ptr [rsp + 0xa0]
14147447e: movaps xmm9, xmmword ptr [rsp + 0x90]
141474487: movaps xmm10, xmmword ptr [rsp + 0x80]
141474490: movaps xmm11, xmmword ptr [rsp + 0x70]
141474496: movaps xmm12, xmmword ptr [rsp + 0x60]
14147449c: movaps xmm13, xmmword ptr [rsp + 0x50]
1414744a2: movdqa xmm14, xmmword ptr [rsp + 0x40]
1414744a9: movaps xmm15, xmmword ptr [rsp + 0x30]
1414744af: add rsp, 0xd8
1414744b6: pop rbx
1414744b7: pop rdi
1414744b8: pop rsi
1414744b9: pop r12
1414744bb: pop r13
1414744bd: pop r14
1414744bf: pop r15
1414744c1: pop rbp
1414744c2: ret 
