140347090: 55 push rbp
140347091: 4157 push r15
140347093: 4156 push r14
140347095: 4155 push r13
140347097: 4154 push r12
140347099: 56 push rsi
14034709a: 57 push rdi
14034709b: 53 push rbx
14034709c: 4881ecc8050000 sub rsp, 0x5c8
1403470a3: 488dac2480000000 lea rbp, [rsp + 0x80]
1403470ab: 440f299530050000 movaps xmmword ptr [rbp + 0x530], xmm10
1403470b3: 440f298d20050000 movaps xmmword ptr [rbp + 0x520], xmm9
1403470bb: 440f298510050000 movaps xmmword ptr [rbp + 0x510], xmm8
1403470c3: 0f29bd00050000 movaps xmmword ptr [rbp + 0x500], xmm7
1403470ca: 0f29b5f0040000 movaps xmmword ptr [rbp + 0x4f0], xmm6
1403470d1: 48c785e8040000feffffff mov qword ptr [rbp + 0x4e8], 0xfffffffffffffffe
1403470dc: 4c8b29 mov r13, qword ptr [rcx]
1403470df: 4c8b7910 mov r15, qword ptr [rcx + 0x10]
1403470e3: 8b791c mov edi, dword ptr [rcx + 0x1c]
1403470e6: 488b4120 mov rax, qword ptr [rcx + 0x20]
1403470ea: 488985e0040000 mov qword ptr [rbp + 0x4e0], rax
1403470f1: 488b4130 mov rax, qword ptr [rcx + 0x30]
1403470f5: 488985f8030000 mov qword ptr [rbp + 0x3f8], rax
1403470fc: 8b413c mov eax, dword ptr [rcx + 0x3c]
1403470ff: 8985d0040000 mov dword ptr [rbp + 0x4d0], eax
140347105: 488b4140 mov rax, qword ptr [rcx + 0x40]
140347109: 488985d8040000 mov qword ptr [rbp + 0x4d8], rax
140347110: 488b4150 mov rax, qword ptr [rcx + 0x50]
140347114: 48898510040000 mov qword ptr [rbp + 0x410], rax
14034711b: 8b415c mov eax, dword ptr [rcx + 0x5c]
14034711e: 8985bc040000 mov dword ptr [rbp + 0x4bc], eax
140347124: 488b4160 mov rax, qword ptr [rcx + 0x60]
140347128: 48898568040000 mov qword ptr [rbp + 0x468], rax
14034712f: 488b4170 mov rax, qword ptr [rcx + 0x70]
140347133: 488985b8020000 mov qword ptr [rbp + 0x2b8], rax
14034713a: 8b417c mov eax, dword ptr [rcx + 0x7c]
14034713d: 89852c040000 mov dword ptr [rbp + 0x42c], eax
140347143: 4c8ba180000000 mov r12, qword ptr [rcx + 0x80]
14034714a: 488b8190000000 mov rax, qword ptr [rcx + 0x90]
140347151: 488985c0020000 mov qword ptr [rbp + 0x2c0], rax
140347158: 8b819c000000 mov eax, dword ptr [rcx + 0x9c]
14034715e: 898530040000 mov dword ptr [rbp + 0x430], eax
140347164: 488b81a0000000 mov rax, qword ptr [rcx + 0xa0]
14034716b: 48898598040000 mov qword ptr [rbp + 0x498], rax
140347172: 488b81b0000000 mov rax, qword ptr [rcx + 0xb0]
140347179: 488985c8020000 mov qword ptr [rbp + 0x2c8], rax
140347180: 8b81bc000000 mov eax, dword ptr [rcx + 0xbc]
140347186: 898534040000 mov dword ptr [rbp + 0x434], eax
14034718c: 488b81c0000000 mov rax, qword ptr [rcx + 0xc0]
140347193: 488985d0020000 mov qword ptr [rbp + 0x2d0], rax
14034719a: 488b81d0000000 mov rax, qword ptr [rcx + 0xd0]
1403471a1: 488985e0020000 mov qword ptr [rbp + 0x2e0], rax
1403471a8: 8b81dc000000 mov eax, dword ptr [rcx + 0xdc]
1403471ae: 89853c040000 mov dword ptr [rbp + 0x43c], eax
1403471b4: 488b81e0000000 mov rax, qword ptr [rcx + 0xe0]
1403471bb: 488985e8020000 mov qword ptr [rbp + 0x2e8], rax
1403471c2: 488b81f0000000 mov rax, qword ptr [rcx + 0xf0]
1403471c9: 488985d8020000 mov qword ptr [rbp + 0x2d8], rax
1403471d0: 8b81fc000000 mov eax, dword ptr [rcx + 0xfc]
1403471d6: 898538040000 mov dword ptr [rbp + 0x438], eax
1403471dc: 488b8100010000 mov rax, qword ptr [rcx + 0x100]
1403471e3: 488985a0040000 mov qword ptr [rbp + 0x4a0], rax
1403471ea: 488b8110010000 mov rax, qword ptr [rcx + 0x110]
1403471f1: 488985f0020000 mov qword ptr [rbp + 0x2f0], rax
1403471f8: 8b811c010000 mov eax, dword ptr [rcx + 0x11c]
1403471fe: 898540040000 mov dword ptr [rbp + 0x440], eax
140347204: 488b9920010000 mov rbx, qword ptr [rcx + 0x120]
14034720b: 488b8130010000 mov rax, qword ptr [rcx + 0x130]
140347212: 488985f8020000 mov qword ptr [rbp + 0x2f8], rax
140347219: 8b813c010000 mov eax, dword ptr [rcx + 0x13c]
14034721f: 898544040000 mov dword ptr [rbp + 0x444], eax
140347225: 488b8140010000 mov rax, qword ptr [rcx + 0x140]
14034722c: 48898538030000 mov qword ptr [rbp + 0x338], rax
140347233: 488b8150010000 mov rax, qword ptr [rcx + 0x150]
14034723a: 48898540030000 mov qword ptr [rbp + 0x340], rax
140347241: 8b815c010000 mov eax, dword ptr [rcx + 0x15c]
140347247: 898560040000 mov dword ptr [rbp + 0x460], eax
14034724d: 488bb160010000 mov rsi, qword ptr [rcx + 0x160]
140347254: 488b8170010000 mov rax, qword ptr [rcx + 0x170]
14034725b: 48898500030000 mov qword ptr [rbp + 0x300], rax
140347262: 8b817c010000 mov eax, dword ptr [rcx + 0x17c]
140347268: 898548040000 mov dword ptr [rbp + 0x448], eax
14034726e: 488b8180010000 mov rax, qword ptr [rcx + 0x180]
140347275: 48898570040000 mov qword ptr [rbp + 0x470], rax
14034727c: 488b8190010000 mov rax, qword ptr [rcx + 0x190]
140347283: 48898508030000 mov qword ptr [rbp + 0x308], rax
14034728a: 8b819c010000 mov eax, dword ptr [rcx + 0x19c]
140347290: 89854c040000 mov dword ptr [rbp + 0x44c], eax
140347296: 488b81a0010000 mov rax, qword ptr [rcx + 0x1a0]
14034729d: 488985a8040000 mov qword ptr [rbp + 0x4a8], rax
1403472a4: 488b81b0010000 mov rax, qword ptr [rcx + 0x1b0]
1403472ab: 48898518040000 mov qword ptr [rbp + 0x418], rax
1403472b2: 8b81bc010000 mov eax, dword ptr [rcx + 0x1bc]
1403472b8: 8985c0040000 mov dword ptr [rbp + 0x4c0], eax
1403472be: 488b81c0010000 mov rax, qword ptr [rcx + 0x1c0]
1403472c5: 48898578040000 mov qword ptr [rbp + 0x478], rax
1403472cc: 488b81d0010000 mov rax, qword ptr [rcx + 0x1d0]
1403472d3: 48898510030000 mov qword ptr [rbp + 0x310], rax
1403472da: 8b81dc010000 mov eax, dword ptr [rcx + 0x1dc]
1403472e0: 898550040000 mov dword ptr [rbp + 0x450], eax
1403472e6: 488b81e0010000 mov rax, qword ptr [rcx + 0x1e0]
1403472ed: 48898580040000 mov qword ptr [rbp + 0x480], rax
1403472f4: 488b81f0010000 mov rax, qword ptr [rcx + 0x1f0]
1403472fb: 48898518030000 mov qword ptr [rbp + 0x318], rax
140347302: 8b81fc010000 mov eax, dword ptr [rcx + 0x1fc]
140347308: 898554040000 mov dword ptr [rbp + 0x454], eax
14034730e: 488b02 mov rax, qword ptr [rdx]
140347311: 48898590040000 mov qword ptr [rbp + 0x490], rax
140347318: 488b4210 mov rax, qword ptr [rdx + 0x10]
14034731c: 48898520030000 mov qword ptr [rbp + 0x320], rax
140347323: 8b421c mov eax, dword ptr [rdx + 0x1c]
140347326: 898558040000 mov dword ptr [rbp + 0x458], eax
14034732c: 488b4220 mov rax, qword ptr [rdx + 0x20]
140347330: 48898588040000 mov qword ptr [rbp + 0x488], rax
140347337: 488b4230 mov rax, qword ptr [rdx + 0x30]
14034733b: 48898528030000 mov qword ptr [rbp + 0x328], rax
140347342: 8b423c mov eax, dword ptr [rdx + 0x3c]
140347345: 89855c040000 mov dword ptr [rbp + 0x45c], eax
14034734b: 488b4240 mov rax, qword ptr [rdx + 0x40]
14034734f: 488985b0040000 mov qword ptr [rbp + 0x4b0], rax
140347356: 488b4250 mov rax, qword ptr [rdx + 0x50]
14034735a: 48898520040000 mov qword ptr [rbp + 0x420], rax
140347361: 8b425c mov eax, dword ptr [rdx + 0x5c]
140347364: 8985c4040000 mov dword ptr [rbp + 0x4c4], eax
14034736a: 488b4260 mov rax, qword ptr [rdx + 0x60]
14034736e: 48898550030000 mov qword ptr [rbp + 0x350], rax
140347375: 488b4270 mov rax, qword ptr [rdx + 0x70]
140347379: 48898558030000 mov qword ptr [rbp + 0x358], rax
140347380: 8b427c mov eax, dword ptr [rdx + 0x7c]
140347383: 898564040000 mov dword ptr [rbp + 0x464], eax
140347389: 488b8288000000 mov rax, qword ptr [rdx + 0x88]
140347390: 48898548030000 mov qword ptr [rbp + 0x348], rax
140347397: 488b8290000000 mov rax, qword ptr [rdx + 0x90]
14034739e: 48898530030000 mov qword ptr [rbp + 0x330], rax
1403473a5: e89663a000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
1403473aa: 4989c6 mov r14, rax
1403473ad: f60001 test byte ptr [rax], 1
1403473b0: 7411 je 0x1403473c3
1403473b2: 4c89f1 mov rcx, r14
1403473b5: 4883c108 add rcx, 8
1403473b9: 498b4608 mov rax, qword ptr [r14 + 8]
1403473bd: 498b5610 mov rdx, qword ptr [r14 + 0x10]
1403473c1: eb1b jmp 0x1403473de ; unknown
1403473c3: e838daa100 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
1403473c8: 4c89f1 mov rcx, r14
1403473cb: 4883c108 add rcx, 8
1403473cf: 49c70601000000 mov qword ptr [r14], 1
1403473d6: 49894608 mov qword ptr [r14 + 8], rax
1403473da: 49895610 mov qword ptr [r14 + 0x10], rdx
1403473de: 4c8d4001 lea r8, [rax + 1]
1403473e2: 4c8901 mov qword ptr [rcx], r8
1403473e5: 0f103d74ba6702 movups xmm7, xmmword ptr [rip + 0x267ba74]
1403473ec: 0f297d50 movaps xmmword ptr [rbp + 0x50], xmm7
1403473f0: 0f103559ba6702 movups xmm6, xmmword ptr [rip + 0x267ba59]
1403473f7: 0f297540 movaps xmmword ptr [rbp + 0x40], xmm6
1403473fb: 48894560 mov qword ptr [rbp + 0x60], rax
1403473ff: 48895568 mov qword ptr [rbp + 0x68], rdx
140347403: e83863a000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
140347408: 4989c6 mov r14, rax
14034740b: f60001 test byte ptr [rax], 1
14034740e: 7411 je 0x140347421
140347410: 4c89f1 mov rcx, r14
140347413: 4883c108 add rcx, 8
140347417: 498b4608 mov rax, qword ptr [r14 + 8]
14034741b: 498b5610 mov rdx, qword ptr [r14 + 0x10]
14034741f: eb1b jmp 0x14034743c ; unknown
140347421: e8dad9a100 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
140347426: 4c89f1 mov rcx, r14
140347429: 4883c108 add rcx, 8
14034742d: 49c70601000000 mov qword ptr [r14], 1
140347434: 49894608 mov qword ptr [r14 + 8], rax
140347438: 49895610 mov qword ptr [r14 + 0x10], rdx
14034743c: 4c8d4001 lea r8, [rax + 1]
140347440: 4c8901 mov qword ptr [rcx], r8
140347443: 0f29bd70030000 movaps xmmword ptr [rbp + 0x370], xmm7
14034744a: 0f29b560030000 movaps xmmword ptr [rbp + 0x360], xmm6
140347451: 48898580030000 mov qword ptr [rbp + 0x380], rax
140347458: 48899588030000 mov qword ptr [rbp + 0x388], rdx
14034745f: e8dc62a000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
140347464: 4989c6 mov r14, rax
140347467: f60001 test byte ptr [rax], 1
14034746a: 7418 je 0x140347484
14034746c: 4d89f0 mov r8, r14
14034746f: 4983c008 add r8, 8
140347473: 498b4608 mov rax, qword ptr [r14 + 8]
140347477: 498b4e10 mov rcx, qword ptr [r14 + 0x10]
14034747b: 48898dc8040000 mov qword ptr [rbp + 0x4c8], rcx
140347482: eb22 jmp 0x1403474a6 ; unknown
140347484: e877d9a100 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
140347489: 4d89f0 mov r8, r14
14034748c: 4983c008 add r8, 8
140347490: 49c70601000000 mov qword ptr [r14], 1
140347497: 49894608 mov qword ptr [r14 + 8], rax
14034749b: 488995c8040000 mov qword ptr [rbp + 0x4c8], rdx
1403474a2: 49895610 mov qword ptr [r14 + 0x10], rdx
1403474a6: 48898500040000 mov qword ptr [rbp + 0x400], rax
1403474ad: 488d4801 lea rcx, [rax + 1]
1403474b1: 498908 mov qword ptr [r8], rcx
1403474b4: 0f284540 movaps xmm0, xmmword ptr [rbp + 0x40]
1403474b8: 0f284d50 movaps xmm1, xmmword ptr [rbp + 0x50]
1403474bc: 0f285560 movaps xmm2, xmmword ptr [rbp + 0x60]
1403474c0: 0f2995f0000000 movaps xmmword ptr [rbp + 0xf0], xmm2
1403474c7: 0f298de0000000 movaps xmmword ptr [rbp + 0xe0], xmm1
1403474ce: 0f2985d0000000 movaps xmmword ptr [rbp + 0xd0], xmm0
1403474d5: 0f288560030000 movaps xmm0, xmmword ptr [rbp + 0x360]
1403474dc: 0f288d70030000 movaps xmm1, xmmword ptr [rbp + 0x370]
1403474e3: 0f289580030000 movaps xmm2, xmmword ptr [rbp + 0x380]
1403474ea: 0f298500010000 movaps xmmword ptr [rbp + 0x100], xmm0
1403474f1: 0f298d10010000 movaps xmmword ptr [rbp + 0x110], xmm1
1403474f8: 0f299520010000 movaps xmmword ptr [rbp + 0x120], xmm2
1403474ff: 0f29b530010000 movaps xmmword ptr [rbp + 0x130], xmm6
140347506: 0f29bd40010000 movaps xmmword ptr [rbp + 0x140], xmm7
14034750d: 41893f mov dword ptr [r15], edi
140347510: 4c89ad08040000 mov qword ptr [rbp + 0x408], r13
140347517: 4c89e9 mov rcx, r13
14034751a: e891132c00 call 0x1406088b0 ; core::ptr::drop_in_place<country_core::resources::walls::decorator_storage::DecoratorStorage> | core::ptr::drop_in_place<country_core::resources::walls::decorator_storage::DecoratorStorage> | core::ptr::drop_in_place<cou
14034751f: 0f288540010000 movaps xmm0, xmmword ptr [rbp + 0x140]
140347526: 488b8d08040000 mov rcx, qword ptr [rbp + 0x408]
14034752d: 0f114170 movups xmmword ptr [rcx + 0x70], xmm0
140347531: 0f288530010000 movaps xmm0, xmmword ptr [rbp + 0x130]
140347538: 0f114160 movups xmmword ptr [rcx + 0x60], xmm0
14034753c: 0f288520010000 movaps xmm0, xmmword ptr [rbp + 0x120]
140347543: 0f114150 movups xmmword ptr [rcx + 0x50], xmm0
140347547: 0f288510010000 movaps xmm0, xmmword ptr [rbp + 0x110]
14034754e: 0f114140 movups xmmword ptr [rcx + 0x40], xmm0
140347552: 0f2885d0000000 movaps xmm0, xmmword ptr [rbp + 0xd0]
140347559: 0f288de0000000 movaps xmm1, xmmword ptr [rbp + 0xe0]
140347560: 0f2895f0000000 movaps xmm2, xmmword ptr [rbp + 0xf0]
140347567: 0f289d00010000 movaps xmm3, xmmword ptr [rbp + 0x100]
14034756e: 0f115930 movups xmmword ptr [rcx + 0x30], xmm3
140347572: 0f115120 movups xmmword ptr [rcx + 0x20], xmm2
140347576: 0f114910 movups xmmword ptr [rcx + 0x10], xmm1
14034757a: 0f1101 movups xmmword ptr [rcx], xmm0
14034757d: 488b8500040000 mov rax, qword ptr [rbp + 0x400]
140347584: 48898180000000 mov qword ptr [rcx + 0x80], rax
14034758b: 488b85c8040000 mov rax, qword ptr [rbp + 0x4c8]
140347592: 48898188000000 mov qword ptr [rcx + 0x88], rax
140347599: 48c7819000000000000000 mov qword ptr [rcx + 0x90], 0
1403475a4: 48c7812001000000000000 mov qword ptr [rcx + 0x120], 0
1403475af: c681b001000000 mov byte ptr [rcx + 0x1b0], 0
1403475b6: e88561a000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
1403475bb: 4989c6 mov r14, rax
1403475be: f60001 test byte ptr [rax], 1
1403475c1: 7418 je 0x1403475db
1403475c3: 4d89f1 mov r9, r14
1403475c6: 4983c108 add r9, 8
1403475ca: 498b4608 mov rax, qword ptr [r14 + 8]
1403475ce: 498b4e10 mov rcx, qword ptr [r14 + 0x10]
1403475d2: 48898dc8040000 mov qword ptr [rbp + 0x4c8], rcx
1403475d9: eb22 jmp 0x1403475fd ; unknown
1403475db: e820d8a100 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
1403475e0: 4d89f1 mov r9, r14
1403475e3: 4983c108 add r9, 8
1403475e7: 49c70601000000 mov qword ptr [r14], 1
1403475ee: 49894608 mov qword ptr [r14 + 8], rax
1403475f2: 488995c8040000 mov qword ptr [rbp + 0x4c8], rdx
1403475f9: 49895610 mov qword ptr [r14 + 0x10], rdx
1403475fd: 488b8de0040000 mov rcx, qword ptr [rbp + 0x4e0]
140347604: 4c8badf8030000 mov r13, qword ptr [rbp + 0x3f8]
14034760b: 448b85d0040000 mov r8d, dword ptr [rbp + 0x4d0]
140347612: 488985d0040000 mov qword ptr [rbp + 0x4d0], rax
140347619: 488d5001 lea rdx, [rax + 1]
14034761d: 498911 mov qword ptr [r9], rdx
140347620: 45894500 mov dword ptr [r13], r8d
140347624: 488d5120 lea rdx, [rcx + 0x20]
140347628: 41b810000000 mov r8d, 0x10
14034762e: 41b910000000 mov r9d, 0x10
140347634: e837b6d0ff call 0x140052c70 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::resources::walls::decorator::DecoratorId,alloc::sync::Arc<system_decorator::blueprint::DecoratorBlueprint,alloc::alloc::Global> >,alloc::alloc::Global>
140347639: 0f100520b86702 movups xmm0, xmmword ptr [rip + 0x267b820]
140347640: 488b8de0040000 mov rcx, qword ptr [rbp + 0x4e0]
140347647: 0f114110 movups xmmword ptr [rcx + 0x10], xmm0
14034764b: 0f1005feb76702 movups xmm0, xmmword ptr [rip + 0x267b7fe]
140347652: 0f1101 movups xmmword ptr [rcx], xmm0
140347655: 488b85d0040000 mov rax, qword ptr [rbp + 0x4d0]
14034765c: 48894120 mov qword ptr [rcx + 0x20], rax
140347660: 488b85c8040000 mov rax, qword ptr [rbp + 0x4c8]
140347667: 48894128 mov qword ptr [rcx + 0x28], rax
14034766b: e8d060a000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
140347670: 4989c6 mov r14, rax
140347673: f60001 test byte ptr [rax], 1
140347676: 7418 je 0x140347690
140347678: 4d89f2 mov r10, r14
14034767b: 4983c208 add r10, 8
14034767f: 498b4608 mov rax, qword ptr [r14 + 8]
140347683: 498b4e10 mov rcx, qword ptr [r14 + 0x10]
140347687: 48898de0040000 mov qword ptr [rbp + 0x4e0], rcx
14034768e: eb22 jmp 0x1403476b2 ; unknown
140347690: e86bd7a100 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
140347695: 4d89f2 mov r10, r14
140347698: 4983c208 add r10, 8
14034769c: 49c70601000000 mov qword ptr [r14], 1
1403476a3: 49894608 mov qword ptr [r14 + 8], rax
1403476a7: 488995e0040000 mov qword ptr [rbp + 0x4e0], rdx
1403476ae: 49895610 mov qword ptr [r14 + 0x10], rdx
1403476b2: 488b8dd8040000 mov rcx, qword ptr [rbp + 0x4d8]
1403476b9: 4c8b8510040000 mov r8, qword ptr [rbp + 0x410]
1403476c0: 448b8dbc040000 mov r9d, dword ptr [rbp + 0x4bc]
1403476c7: 488985d0040000 mov qword ptr [rbp + 0x4d0], rax
1403476ce: 488d5001 lea rdx, [rax + 1]
1403476d2: 498912 mov qword ptr [r10], rdx
1403476d5: 458908 mov dword ptr [r8], r9d
1403476d8: 488d5120 lea rdx, [rcx + 0x20]
1403476dc: 41b810000000 mov r8d, 0x10
1403476e2: 41b910000000 mov r9d, 0x10
1403476e8: e883b5d0ff call 0x140052c70 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::resources::walls::decorator::DecoratorId,alloc::sync::Arc<system_decorator::blueprint::DecoratorBlueprint,alloc::alloc::Global> >,alloc::alloc::Global>
1403476ed: 488bbd68040000 mov rdi, qword ptr [rbp + 0x468]
1403476f4: 440f100564b76702 movups xmm8, xmmword ptr [rip + 0x267b764]
1403476fc: 488b8dd8040000 mov rcx, qword ptr [rbp + 0x4d8]
140347703: 440f114110 movups xmmword ptr [rcx + 0x10], xmm8
140347708: 0f103d41b76702 movups xmm7, xmmword ptr [rip + 0x267b741]
14034770f: 0f1139 movups xmmword ptr [rcx], xmm7
140347712: 488b85d0040000 mov rax, qword ptr [rbp + 0x4d0]
140347719: 48894120 mov qword ptr [rcx + 0x20], rax
14034771d: 488b85e0040000 mov rax, qword ptr [rbp + 0x4e0]
140347724: 48894128 mov qword ptr [rcx + 0x28], rax
140347728: 488d8dd0000000 lea rcx, [rbp + 0xd0]
14034772f: e83c9cffff call 0x140341370 ; country_core::resources::walls::decorator_archivist::impl$1::default
140347734: 488b85b8020000 mov rax, qword ptr [rbp + 0x2b8]
14034773b: 8b8d2c040000 mov ecx, dword ptr [rbp + 0x42c]
140347741: 8908 mov dword ptr [rax], ecx
140347743: 4889f9 mov rcx, rdi
140347746: e895332c00 call 0x14060aae0 ; core::ptr::drop_in_place<country_core::resources::walls::decorator_archivist::DecoratorArchivist> | core::ptr::drop_in_place<country_core::resources::walls::decorator_archivist::DecoratorArchivist> | core::ptr::drop_in_p
14034774b: 488d95d0000000 lea rdx, [rbp + 0xd0]
140347752: 41b8e8010000 mov r8d, 0x1e8
140347758: 488b8d68040000 mov rcx, qword ptr [rbp + 0x468]
14034775f: e89cc45d02 call 0x142923c00 ; unknown
140347764: 31c9 xor ecx, ecx
140347766: e8b523d9ff call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > 
14034776b: 4885c0 test rax, rax
14034776e: 0f84db0d0000 je 0x14034854f
140347774: 0f1030 movups xmm6, xmmword ptr [rax]
140347777: 48ff00 inc qword ptr [rax]
14034777a: 488b85c0020000 mov rax, qword ptr [rbp + 0x2c0]
140347781: 8b8d30040000 mov ecx, dword ptr [rbp + 0x430]
140347787: 8908 mov dword ptr [rax], ecx
140347789: 498b1424 mov rdx, qword ptr [r12]
14034778d: 4885d2 test rdx, rdx
140347790: 7414 je 0x1403477a6
140347792: 498b4c2408 mov rcx, qword ptr [r12 + 8]
140347797: 48c1e205 shl rdx, 5
14034779b: 41b808000000 mov r8d, 8
1403477a1: e87ac42c00 call 0x140613c20 ; unknown
1403477a6: 498d7c2418 lea rdi, [r12 + 0x18]
1403477ab: 4d8b4c2420 mov r9, qword ptr [r12 + 0x20]
1403477b0: 4d85c9 test r9, r9
1403477b3: 7431 je 0x1403477e6
1403477b5: 498d4101 lea rax, [r9 + 1]
1403477b9: b918000000 mov ecx, 0x18
1403477be: 48f7e1 mul rcx
1403477c1: 4883c00f add rax, 0xf
1403477c5: 4883e0f0 and rax, 0xfffffffffffffff0
1403477c9: 4901c1 add r9, rax
1403477cc: 4983c111 add r9, 0x11
1403477d0: 7414 je 0x1403477e6
1403477d2: 488b0f mov rcx, qword ptr [rdi]
1403477d5: 4829c1 sub rcx, rax
1403477d8: 41b810000000 mov r8d, 0x10
1403477de: 4c89ca mov rdx, r9
1403477e1: e83ac42c00 call 0x140613c20 ; unknown
1403477e6: 49c7042400000000 mov qword ptr [r12], 0
1403477ee: 49c744240808000000 mov qword ptr [r12 + 8], 8
1403477f7: 49c744241000000000 mov qword ptr [r12 + 0x10], 0
140347800: 0f100529886602 movups xmm0, xmmword ptr [rip + 0x2668829]
140347807: 0f114710 movups xmmword ptr [rdi + 0x10], xmm0
14034780b: 0f10050e886602 movups xmm0, xmmword ptr [rip + 0x266880e]
140347812: 0f1107 movups xmmword ptr [rdi], xmm0
140347815: 410f11742438 movups xmmword ptr [r12 + 0x38], xmm6
14034781b: 41c644244800 mov byte ptr [r12 + 0x48], 0
140347821: e81a5fa000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
140347826: 4989c6 mov r14, rax
140347829: f60001 test byte ptr [rax], 1
14034782c: 7411 je 0x14034783f
14034782e: 4c89f1 mov rcx, r14
140347831: 4883c108 add rcx, 8
140347835: 498b4608 mov rax, qword ptr [r14 + 8]
140347839: 498b5610 mov rdx, qword ptr [r14 + 0x10]
14034783d: eb1b jmp 0x14034785a ; unknown
14034783f: e8bcd5a100 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
140347844: 4c89f1 mov rcx, r14
140347847: 4883c108 add rcx, 8
14034784b: 49c70601000000 mov qword ptr [r14], 1
140347852: 49894608 mov qword ptr [r14 + 8], rax
140347856: 49895610 mov qword ptr [r14 + 0x10], rdx
14034785a: 488bbd98040000 mov rdi, qword ptr [rbp + 0x498]
140347861: 4c8d4001 lea r8, [rax + 1]
140347865: 4c8901 mov qword ptr [rcx], r8
140347868: 440f2985e0000000 movaps xmmword ptr [rbp + 0xe0], xmm8
140347870: 0f29bdd0000000 movaps xmmword ptr [rbp + 0xd0], xmm7
140347877: 488985f0000000 mov qword ptr [rbp + 0xf0], rax
14034787e: 488995f8000000 mov qword ptr [rbp + 0xf8], rdx
140347885: e8b65ea000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
14034788a: 4989c6 mov r14, rax
14034788d: f60001 test byte ptr [rax], 1
140347890: 7418 je 0x1403478aa
140347892: 4d89f0 mov r8, r14
140347895: 4983c008 add r8, 8
140347899: 498b4608 mov rax, qword ptr [r14 + 8]
14034789d: 498b4e10 mov rcx, qword ptr [r14 + 0x10]
1403478a1: 48898de0040000 mov qword ptr [rbp + 0x4e0], rcx
1403478a8: eb22 jmp 0x1403478cc ; unknown
1403478aa: e851d5a100 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
1403478af: 4d89f0 mov r8, r14
1403478b2: 4983c008 add r8, 8
1403478b6: 49c70601000000 mov qword ptr [r14], 1
1403478bd: 49894608 mov qword ptr [r14 + 8], rax
1403478c1: 488995e0040000 mov qword ptr [rbp + 0x4e0], rdx
1403478c8: 49895610 mov qword ptr [r14 + 0x10], rdx
1403478cc: 488985d8040000 mov qword ptr [rbp + 0x4d8], rax
1403478d3: 488d4801 lea rcx, [rax + 1]
1403478d7: 498908 mov qword ptr [r8], rcx
1403478da: 0f2885d0000000 movaps xmm0, xmmword ptr [rbp + 0xd0]
1403478e1: 0f288de0000000 movaps xmm1, xmmword ptr [rbp + 0xe0]
1403478e8: 0f2895f0000000 movaps xmm2, xmmword ptr [rbp + 0xf0]
1403478ef: 0f299580030000 movaps xmmword ptr [rbp + 0x380], xmm2
1403478f6: 0f298d70030000 movaps xmmword ptr [rbp + 0x370], xmm1
1403478fd: 0f298560030000 movaps xmmword ptr [rbp + 0x360], xmm0
140347904: 488b85c8020000 mov rax, qword ptr [rbp + 0x2c8]
14034790b: 8b8d34040000 mov ecx, dword ptr [rbp + 0x434]
140347911: 8908 mov dword ptr [rax], ecx
140347913: 4889f9 mov rcx, rdi
140347916: e8d5e22b00 call 0x140605bf0 ; core::ptr::drop_in_place<system_decorator::decorator_visual_state::DecoratorVisualState>
14034791b: 488b8d98040000 mov rcx, qword ptr [rbp + 0x498]
140347922: 48c70100000000 mov qword ptr [rcx], 0
140347929: 48c7410810000000 mov qword ptr [rcx + 8], 0x10
140347931: 48c7411000000000 mov qword ptr [rcx + 0x10], 0
140347939: 0f288560030000 movaps xmm0, xmmword ptr [rbp + 0x360]
140347940: 0f288d70030000 movaps xmm1, xmmword ptr [rbp + 0x370]
140347947: 0f289580030000 movaps xmm2, xmmword ptr [rbp + 0x380]
14034794e: 0f114118 movups xmmword ptr [rcx + 0x18], xmm0
140347952: 0f114928 movups xmmword ptr [rcx + 0x28], xmm1
140347956: 0f115138 movups xmmword ptr [rcx + 0x38], xmm2
14034795a: 0f103defb46702 movups xmm7, xmmword ptr [rip + 0x267b4ef]
140347961: 0f117948 movups xmmword ptr [rcx + 0x48], xmm7
140347965: 440f1005f3b46702 movups xmm8, xmmword ptr [rip + 0x267b4f3]
14034796d: 440f114158 movups xmmword ptr [rcx + 0x58], xmm8
140347972: 488b85d8040000 mov rax, qword ptr [rbp + 0x4d8]
140347979: 48894168 mov qword ptr [rcx + 0x68], rax
14034797d: 488b85e0040000 mov rax, qword ptr [rbp + 0x4e0]
140347984: 48894170 mov qword ptr [rcx + 0x70], rax
140347988: e8e3a9cbff call 0x140002370 ; bevy_ecs::system::schedule_system::impl$1::is_exclusive<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ec
14034798d: 488b8de0020000 mov rcx, qword ptr [rbp + 0x2e0]
140347994: 8b953c040000 mov edx, dword ptr [rbp + 0x43c]
14034799a: 8911 mov dword ptr [rcx], edx
14034799c: 488b8dd0020000 mov rcx, qword ptr [rbp + 0x2d0]
1403479a3: 8901 mov dword ptr [rcx], eax
1403479a5: c785db00000000000000 mov dword ptr [rbp + 0xdb], 0
1403479af: 48c785d400000000000000 mov qword ptr [rbp + 0xd4], 0
1403479ba: 488b85d8020000 mov rax, qword ptr [rbp + 0x2d8]
1403479c1: 8b8d38040000 mov ecx, dword ptr [rbp + 0x438]
1403479c7: 8908 mov dword ptr [rax], ecx
1403479c9: 488b95e8020000 mov rdx, qword ptr [rbp + 0x2e8]
1403479d0: c70200000000 mov dword ptr [rdx], 0
1403479d6: 488b85d0000000 mov rax, qword ptr [rbp + 0xd0]
1403479dd: 488b8dd7000000 mov rcx, qword ptr [rbp + 0xd7]
1403479e4: 48894204 mov qword ptr [rdx + 4], rax
1403479e8: 48894a0b mov qword ptr [rdx + 0xb], rcx
1403479ec: e84f5da000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
1403479f1: 4989c6 mov r14, rax
1403479f4: f60001 test byte ptr [rax], 1
1403479f7: 7411 je 0x140347a0a
1403479f9: 4c89f1 mov rcx, r14
1403479fc: 4883c108 add rcx, 8
140347a00: 498b4608 mov rax, qword ptr [r14 + 8]
140347a04: 498b5610 mov rdx, qword ptr [r14 + 0x10]
140347a08: eb1b jmp 0x140347a25 ; unknown
140347a0a: e8f1d3a100 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
140347a0f: 4c89f1 mov rcx, r14
140347a12: 4883c108 add rcx, 8
140347a16: 49c70601000000 mov qword ptr [r14], 1
140347a1d: 49894608 mov qword ptr [r14 + 8], rax
140347a21: 49895610 mov qword ptr [r14 + 0x10], rdx
140347a25: 488bbda0040000 mov rdi, qword ptr [rbp + 0x4a0]
140347a2c: 4c8d4001 lea r8, [rax + 1]
140347a30: 4c8901 mov qword ptr [rcx], r8
140347a33: 440f2945c0 movaps xmmword ptr [rbp - 0x40], xmm8
140347a38: 0f297db0 movaps xmmword ptr [rbp - 0x50], xmm7
140347a3c: 488945d0 mov qword ptr [rbp - 0x30], rax
140347a40: 488955d8 mov qword ptr [rbp - 0x28], rdx
140347a44: e8f75ca000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
140347a49: 4989c6 mov r14, rax
140347a4c: f60001 test byte ptr [rax], 1
140347a4f: 7411 je 0x140347a62
140347a51: 4c89f1 mov rcx, r14
140347a54: 4883c108 add rcx, 8
140347a58: 498b4608 mov rax, qword ptr [r14 + 8]
140347a5c: 498b5610 mov rdx, qword ptr [r14 + 0x10]
140347a60: eb1b jmp 0x140347a7d ; unknown
140347a62: e899d3a100 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
140347a67: 4c89f1 mov rcx, r14
140347a6a: 4883c108 add rcx, 8
140347a6e: 49c70601000000 mov qword ptr [r14], 1
140347a75: 49894608 mov qword ptr [r14 + 8], rax
140347a79: 49895610 mov qword ptr [r14 + 0x10], rdx
140347a7d: 4c8d4001 lea r8, [rax + 1]
140347a81: 4c8901 mov qword ptr [rcx], r8
140347a84: 440f294550 movaps xmmword ptr [rbp + 0x50], xmm8
140347a89: 0f297d40 movaps xmmword ptr [rbp + 0x40], xmm7
140347a8d: 48894560 mov qword ptr [rbp + 0x60], rax
140347a91: 48895568 mov qword ptr [rbp + 0x68], rdx
140347a95: e8a65ca000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
140347a9a: 4989c6 mov r14, rax
140347a9d: f60001 test byte ptr [rax], 1
140347aa0: 7411 je 0x140347ab3
140347aa2: 4c89f1 mov rcx, r14
140347aa5: 4883c108 add rcx, 8
140347aa9: 498b4608 mov rax, qword ptr [r14 + 8]
140347aad: 498b5610 mov rdx, qword ptr [r14 + 0x10]
140347ab1: eb1b jmp 0x140347ace ; unknown
140347ab3: e848d3a100 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
140347ab8: 4c89f1 mov rcx, r14
140347abb: 4883c108 add rcx, 8
140347abf: 49c70601000000 mov qword ptr [r14], 1
140347ac6: 49894608 mov qword ptr [r14 + 8], rax
140347aca: 49895610 mov qword ptr [r14 + 0x10], rdx
140347ace: 4c8d4001 lea r8, [rax + 1]
140347ad2: 4c8901 mov qword ptr [rcx], r8
140347ad5: 440f298570030000 movaps xmmword ptr [rbp + 0x370], xmm8
140347add: 0f29bd60030000 movaps xmmword ptr [rbp + 0x360], xmm7
140347ae4: 48898580030000 mov qword ptr [rbp + 0x380], rax
140347aeb: 48899588030000 mov qword ptr [rbp + 0x388], rdx
140347af2: e8495ca000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
140347af7: 4989c6 mov r14, rax
140347afa: f60001 test byte ptr [rax], 1
140347afd: 7418 je 0x140347b17
140347aff: 4d89f0 mov r8, r14
140347b02: 4983c008 add r8, 8
140347b06: 498b4608 mov rax, qword ptr [r14 + 8]
140347b0a: 498b4e10 mov rcx, qword ptr [r14 + 0x10]
140347b0e: 48898de0040000 mov qword ptr [rbp + 0x4e0], rcx
140347b15: eb22 jmp 0x140347b39 ; unknown
140347b17: e8e4d2a100 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
140347b1c: 4d89f0 mov r8, r14
140347b1f: 4983c008 add r8, 8
140347b23: 49c70601000000 mov qword ptr [r14], 1
140347b2a: 49894608 mov qword ptr [r14 + 8], rax
140347b2e: 488995e0040000 mov qword ptr [rbp + 0x4e0], rdx
140347b35: 49895610 mov qword ptr [r14 + 0x10], rdx
140347b39: 488985d8040000 mov qword ptr [rbp + 0x4d8], rax
140347b40: 488d4801 lea rcx, [rax + 1]
140347b44: 498908 mov qword ptr [r8], rcx
140347b47: 0f2845b0 movaps xmm0, xmmword ptr [rbp - 0x50]
140347b4b: 0f284dc0 movaps xmm1, xmmword ptr [rbp - 0x40]
140347b4f: 0f2855d0 movaps xmm2, xmmword ptr [rbp - 0x30]
140347b53: 0f2995f0000000 movaps xmmword ptr [rbp + 0xf0], xmm2
140347b5a: 0f298de0000000 movaps xmmword ptr [rbp + 0xe0], xmm1
140347b61: 0f2985d0000000 movaps xmmword ptr [rbp + 0xd0], xmm0
140347b68: 0f284540 movaps xmm0, xmmword ptr [rbp + 0x40]
140347b6c: 0f284d50 movaps xmm1, xmmword ptr [rbp + 0x50]
140347b70: 0f285560 movaps xmm2, xmmword ptr [rbp + 0x60]
140347b74: 0f298500010000 movaps xmmword ptr [rbp + 0x100], xmm0
140347b7b: 0f298d10010000 movaps xmmword ptr [rbp + 0x110], xmm1
140347b82: 0f299520010000 movaps xmmword ptr [rbp + 0x120], xmm2
140347b89: 0f288560030000 movaps xmm0, xmmword ptr [rbp + 0x360]
140347b90: 0f288d70030000 movaps xmm1, xmmword ptr [rbp + 0x370]
140347b97: 0f289580030000 movaps xmm2, xmmword ptr [rbp + 0x380]
140347b9e: 0f298530010000 movaps xmmword ptr [rbp + 0x130], xmm0
140347ba5: 0f298d40010000 movaps xmmword ptr [rbp + 0x140], xmm1
140347bac: 0f299550010000 movaps xmmword ptr [rbp + 0x150], xmm2
140347bb3: 440f298570010000 movaps xmmword ptr [rbp + 0x170], xmm8
140347bbb: 0f29bd60010000 movaps xmmword ptr [rbp + 0x160], xmm7
140347bc2: 488b85f0020000 mov rax, qword ptr [rbp + 0x2f0]
140347bc9: 8b8d40040000 mov ecx, dword ptr [rbp + 0x440]
140347bcf: 8908 mov dword ptr [rax], ecx
140347bd1: 4889f9 mov rcx, rdi
140347bd4: e817e62b00 call 0x1406061f0 ; core::ptr::drop_in_place<system_decorator::audio::op_confirm_audio::DecoratorAudioEvents>
140347bd9: 488d95d0000000 lea rdx, [rbp + 0xd0]
140347be0: 41b8b0000000 mov r8d, 0xb0
140347be6: 488bbda0040000 mov rdi, qword ptr [rbp + 0x4a0]
140347bed: 4889f9 mov rcx, rdi
140347bf0: e80bc05d02 call 0x142923c00 ; unknown
140347bf5: 488b85d8040000 mov rax, qword ptr [rbp + 0x4d8]
140347bfc: 488987b0000000 mov qword ptr [rdi + 0xb0], rax
140347c03: 488b85e0040000 mov rax, qword ptr [rbp + 0x4e0]
140347c0a: 488987b8000000 mov qword ptr [rdi + 0xb8], rax
140347c11: e82a5ba000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
140347c16: 4989c4 mov r12, rax
140347c19: f60001 test byte ptr [rax], 1
140347c1c: 7413 je 0x140347c31
140347c1e: 4c89e0 mov rax, r12
140347c21: 4883c008 add rax, 8
140347c25: 4d8b742408 mov r14, qword ptr [r12 + 8]
140347c2a: 4d8b7c2410 mov r15, qword ptr [r12 + 0x10]
140347c2f: eb24 jmp 0x140347c55 ; unknown
140347c31: e8cad1a100 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
140347c36: 4989c6 mov r14, rax
140347c39: 4989d7 mov r15, rdx
140347c3c: 4c89e0 mov rax, r12
140347c3f: 4883c008 add rax, 8
140347c43: 49c7042401000000 mov qword ptr [r12], 1
140347c4b: 4d89742408 mov qword ptr [r12 + 8], r14
140347c50: 4989542410 mov qword ptr [r12 + 0x10], rdx
140347c55: 498d4e01 lea rcx, [r14 + 1]
140347c59: 488908 mov qword ptr [rax], rcx
140347c5c: 488b85f8020000 mov rax, qword ptr [rbp + 0x2f8]
140347c63: 8b8d44040000 mov ecx, dword ptr [rbp + 0x444]
140347c69: 8908 mov dword ptr [rax], ecx
140347c6b: 4c8b4b08 mov r9, qword ptr [rbx + 8]
140347c6f: 4d85c9 test r9, r9
140347c72: 7431 je 0x140347ca5
140347c74: 498d4101 lea rax, [r9 + 1]
140347c78: b918000000 mov ecx, 0x18
140347c7d: 48f7e1 mul rcx
140347c80: 4883c00f add rax, 0xf
140347c84: 4883e0f0 and rax, 0xfffffffffffffff0
140347c88: 4901c1 add r9, rax
140347c8b: 4983c111 add r9, 0x11
140347c8f: 7414 je 0x140347ca5
140347c91: 488b0b mov rcx, qword ptr [rbx]
140347c94: 4829c1 sub rcx, rax
140347c97: 41b810000000 mov r8d, 0x10
140347c9d: 4c89ca mov rdx, r9
140347ca0: e87bbf2c00 call 0x140613c20 ; unknown
140347ca5: 440f114310 movups xmmword ptr [rbx + 0x10], xmm8
140347caa: 0f113b movups xmmword ptr [rbx], xmm7
140347cad: 4c897320 mov qword ptr [rbx + 0x20], r14
140347cb1: 4c897b28 mov qword ptr [rbx + 0x28], r15
140347cb5: c6433000 mov byte ptr [rbx + 0x30], 0
140347cb9: 31c9 xor ecx, ecx
140347cbb: e8601ed9ff call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > 
140347cc0: 4885c0 test rax, rax
140347cc3: 0f8486080000 je 0x14034854f
140347cc9: 0f1000 movups xmm0, xmmword ptr [rax]
140347ccc: 48ff00 inc qword ptr [rax]
140347ccf: 440f100dd9bb6502 movups xmm9, xmmword ptr [rip + 0x265bbd9]
140347cd7: 440f298d70030000 movaps xmmword ptr [rbp + 0x370], xmm9
140347cdf: 440f1015b9bb6502 movups xmm10, xmmword ptr [rip + 0x265bbb9]
140347ce7: 440f299560030000 movaps xmmword ptr [rbp + 0x360], xmm10
140347cef: 0f298580030000 movaps xmmword ptr [rbp + 0x380], xmm0
140347cf6: 31c9 xor ecx, ecx
140347cf8: e8231ed9ff call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > 
140347cfd: 4885c0 test rax, rax
140347d00: 0f8455080000 je 0x14034855b
140347d06: 0f1030 movups xmm6, xmmword ptr [rax]
140347d09: 48ff00 inc qword ptr [rax]
140347d0c: 0f288560030000 movaps xmm0, xmmword ptr [rbp + 0x360]
140347d13: 0f288d70030000 movaps xmm1, xmmword ptr [rbp + 0x370]
140347d1a: 0f289580030000 movaps xmm2, xmmword ptr [rbp + 0x380]
140347d21: 0f2995f0000000 movaps xmmword ptr [rbp + 0xf0], xmm2
140347d28: 0f298de0000000 movaps xmmword ptr [rbp + 0xe0], xmm1
140347d2f: 0f2985d0000000 movaps xmmword ptr [rbp + 0xd0], xmm0
140347d36: 440f299500010000 movaps xmmword ptr [rbp + 0x100], xmm10
140347d3e: 440f298d10010000 movaps xmmword ptr [rbp + 0x110], xmm9
140347d46: 488b8500030000 mov rax, qword ptr [rbp + 0x300]
140347d4d: 8b8d48040000 mov ecx, dword ptr [rbp + 0x448]
140347d53: 8908 mov dword ptr [rax], ecx
140347d55: 4c8b4e08 mov r9, qword ptr [rsi + 8]
140347d59: 4d85c9 test r9, r9
140347d5c: 7431 je 0x140347d8f
140347d5e: 498d4101 lea rax, [r9 + 1]
140347d62: b918000000 mov ecx, 0x18
140347d67: 48f7e1 mul rcx
140347d6a: 4883c00f add rax, 0xf
140347d6e: 4883e0f0 and rax, 0xfffffffffffffff0
140347d72: 4901c1 add r9, rax
140347d75: 4983c111 add r9, 0x11
140347d79: 7414 je 0x140347d8f
140347d7b: 488b0e mov rcx, qword ptr [rsi]
140347d7e: 4829c1 sub rcx, rax
140347d81: 41b810000000 mov r8d, 0x10
140347d87: 4c89ca mov rdx, r9
140347d8a: e891be2c00 call 0x140613c20 ; unknown
140347d8f: 4c8b4e38 mov r9, qword ptr [rsi + 0x38]
140347d93: 4d85c9 test r9, r9
140347d96: 7432 je 0x140347dca
140347d98: 498d4101 lea rax, [r9 + 1]
140347d9c: b918000000 mov ecx, 0x18
140347da1: 48f7e1 mul rcx
140347da4: 4883c00f add rax, 0xf
140347da8: 4883e0f0 and rax, 0xfffffffffffffff0
140347dac: 4901c1 add r9, rax
140347daf: 4983c111 add r9, 0x11
140347db3: 7415 je 0x140347dca
140347db5: 488b4e30 mov rcx, qword ptr [rsi + 0x30]
140347db9: 4829c1 sub rcx, rax
140347dbc: 41b810000000 mov r8d, 0x10
140347dc2: 4c89ca mov rdx, r9
140347dc5: e856be2c00 call 0x140613c20 ; unknown
140347dca: 0f288510010000 movaps xmm0, xmmword ptr [rbp + 0x110]
140347dd1: 0f114640 movups xmmword ptr [rsi + 0x40], xmm0
140347dd5: 0f2885d0000000 movaps xmm0, xmmword ptr [rbp + 0xd0]
140347ddc: 0f288de0000000 movaps xmm1, xmmword ptr [rbp + 0xe0]
140347de3: 0f2895f0000000 movaps xmm2, xmmword ptr [rbp + 0xf0]
140347dea: 0f289d00010000 movaps xmm3, xmmword ptr [rbp + 0x100]
140347df1: 0f115e30 movups xmmword ptr [rsi + 0x30], xmm3
140347df5: 0f115620 movups xmmword ptr [rsi + 0x20], xmm2
140347df9: 0f114e10 movups xmmword ptr [rsi + 0x10], xmm1
140347dfd: 0f1106 movups xmmword ptr [rsi], xmm0
140347e00: 0f117650 movups xmmword ptr [rsi + 0x50], xmm6
140347e04: e83759a000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
140347e09: 4989c6 mov r14, rax
140347e0c: f60001 test byte ptr [rax], 1
140347e0f: 7411 je 0x140347e22
140347e11: 4c89f1 mov rcx, r14
140347e14: 4883c108 add rcx, 8
140347e18: 498b4608 mov rax, qword ptr [r14 + 8]
140347e1c: 498b5610 mov rdx, qword ptr [r14 + 0x10]
140347e20: eb1b jmp 0x140347e3d ; unknown
140347e22: e8d9cfa100 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
140347e27: 4c89f1 mov rcx, r14
140347e2a: 4883c108 add rcx, 8
140347e2e: 49c70601000000 mov qword ptr [r14], 1
140347e35: 49894608 mov qword ptr [r14 + 8], rax
140347e39: 49895610 mov qword ptr [r14 + 0x10], rdx
140347e3d: 4c8d4001 lea r8, [rax + 1]
140347e41: 4c8901 mov qword ptr [rcx], r8
140347e44: 440f2985e0000000 movaps xmmword ptr [rbp + 0xe0], xmm8
140347e4c: 0f29bdd0000000 movaps xmmword ptr [rbp + 0xd0], xmm7
140347e53: 488985f0000000 mov qword ptr [rbp + 0xf0], rax
140347e5a: 488995f8000000 mov qword ptr [rbp + 0xf8], rdx
140347e61: 31c9 xor ecx, ecx
140347e63: e8b81cd9ff call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > 
140347e68: 4885c0 test rax, rax
140347e6b: 488b8d70040000 mov rcx, qword ptr [rbp + 0x470]
140347e72: 0f84f1060000 je 0x140348569
140347e78: 4c8b00 mov r8, qword ptr [rax]
140347e7b: 488b5008 mov rdx, qword ptr [rax + 8]
140347e7f: 488995e0040000 mov qword ptr [rbp + 0x4e0], rdx
140347e86: 4c8985d8040000 mov qword ptr [rbp + 0x4d8], r8
140347e8d: 498d5001 lea rdx, [r8 + 1]
140347e91: 488910 mov qword ptr [rax], rdx
140347e94: 0f2885d0000000 movaps xmm0, xmmword ptr [rbp + 0xd0]
140347e9b: 0f288de0000000 movaps xmm1, xmmword ptr [rbp + 0xe0]
140347ea2: 0f2895f0000000 movaps xmm2, xmmword ptr [rbp + 0xf0]
140347ea9: 0f299580030000 movaps xmmword ptr [rbp + 0x380], xmm2
140347eb0: 0f298d70030000 movaps xmmword ptr [rbp + 0x370], xmm1
140347eb7: 0f298560030000 movaps xmmword ptr [rbp + 0x360], xmm0
140347ebe: 488b8508030000 mov rax, qword ptr [rbp + 0x308]
140347ec5: 8b954c040000 mov edx, dword ptr [rbp + 0x44c]
140347ecb: 8910 mov dword ptr [rax], edx
140347ecd: e8be7a2800 call 0x1405cf990 ; core::ptr::drop_in_place<country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage>
140347ed2: 488b8d70040000 mov rcx, qword ptr [rbp + 0x470]
140347ed9: 48c70100000000 mov qword ptr [rcx], 0
140347ee0: 48c7410808000000 mov qword ptr [rcx + 8], 8
140347ee8: 48c7411000000000 mov qword ptr [rcx + 0x10], 0
140347ef0: 0f288560030000 movaps xmm0, xmmword ptr [rbp + 0x360]
140347ef7: 0f288d70030000 movaps xmm1, xmmword ptr [rbp + 0x370]
140347efe: 0f289580030000 movaps xmm2, xmmword ptr [rbp + 0x380]
140347f05: 0f114118 movups xmmword ptr [rcx + 0x18], xmm0
140347f09: 0f114928 movups xmmword ptr [rcx + 0x28], xmm1
140347f0d: 0f115138 movups xmmword ptr [rcx + 0x38], xmm2
140347f11: 0f103d88b96502 movups xmm7, xmmword ptr [rip + 0x265b988]
140347f18: 0f117948 movups xmmword ptr [rcx + 0x48], xmm7
140347f1c: 0f10358db96502 movups xmm6, xmmword ptr [rip + 0x265b98d]
140347f23: 0f117158 movups xmmword ptr [rcx + 0x58], xmm6
140347f27: 488b85d8040000 mov rax, qword ptr [rbp + 0x4d8]
140347f2e: 48894168 mov qword ptr [rcx + 0x68], rax
140347f32: 488b85e0040000 mov rax, qword ptr [rbp + 0x4e0]
140347f39: 48894170 mov qword ptr [rcx + 0x70], rax
140347f3d: e8fe57a000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
140347f42: 4989c6 mov r14, rax
140347f45: f60001 test byte ptr [rax], 1
140347f48: 7418 je 0x140347f62
140347f4a: 4d89f2 mov r10, r14
140347f4d: 4983c208 add r10, 8
140347f51: 498b4608 mov rax, qword ptr [r14 + 8]
140347f55: 498b4e10 mov rcx, qword ptr [r14 + 0x10]
140347f59: 48898de0040000 mov qword ptr [rbp + 0x4e0], rcx
140347f60: eb22 jmp 0x140347f84 ; unknown
140347f62: e899cea100 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
140347f67: 4d89f2 mov r10, r14
140347f6a: 4983c208 add r10, 8
140347f6e: 49c70601000000 mov qword ptr [r14], 1
140347f75: 49894608 mov qword ptr [r14 + 8], rax
140347f79: 488995e0040000 mov qword ptr [rbp + 0x4e0], rdx
140347f80: 49895610 mov qword ptr [r14 + 0x10], rdx
140347f84: 488b8da8040000 mov rcx, qword ptr [rbp + 0x4a8]
140347f8b: 4c8b8518040000 mov r8, qword ptr [rbp + 0x418]
140347f92: 448b8dc0040000 mov r9d, dword ptr [rbp + 0x4c0]
140347f99: 488985d8040000 mov qword ptr [rbp + 0x4d8], rax
140347fa0: 488d5001 lea rdx, [rax + 1]
140347fa4: 498912 mov qword ptr [r10], rdx
140347fa7: 458908 mov dword ptr [r8], r9d
140347faa: 488d5120 lea rdx, [rcx + 0x20]
140347fae: 41b818000000 mov r8d, 0x18
140347fb4: 41b910000000 mov r9d, 0x10
140347fba: e8516bd0ff call 0x14004eb10 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::resources::walls::WallId,country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::roof_tile_ssbo::RoofTile
140347fbf: 488bb588040000 mov rsi, qword ptr [rbp + 0x488]
140347fc6: 488bbd80040000 mov rdi, qword ptr [rbp + 0x480]
140347fcd: 488b9d78040000 mov rbx, qword ptr [rbp + 0x478]
140347fd4: 0f100585ae6702 movups xmm0, xmmword ptr [rip + 0x267ae85]
140347fdb: 488b8da8040000 mov rcx, qword ptr [rbp + 0x4a8]
140347fe2: 0f114110 movups xmmword ptr [rcx + 0x10], xmm0
140347fe6: 0f100563ae6702 movups xmm0, xmmword ptr [rip + 0x267ae63]
140347fed: 0f1101 movups xmmword ptr [rcx], xmm0
140347ff0: 488b85d8040000 mov rax, qword ptr [rbp + 0x4d8]
140347ff7: 48894120 mov qword ptr [rcx + 0x20], rax
140347ffb: 488b85e0040000 mov rax, qword ptr [rbp + 0x4e0]
140348002: 48894128 mov qword ptr [rcx + 0x28], rax
140348006: 488d8dd0000000 lea rcx, [rbp + 0xd0]
14034800d: e84e3adcff call 0x14010ba60 ; system_decorator::decorator_visual::deco_differential_state::impl$2::default<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyParams>
140348012: 488b8510030000 mov rax, qword ptr [rbp + 0x310]
140348019: 8b8d50040000 mov ecx, dword ptr [rbp + 0x450]
14034801f: 8908 mov dword ptr [rax], ecx
140348021: 488d5320 lea rdx, [rbx + 0x20]
140348025: 41b838000000 mov r8d, 0x38
14034802b: 41b910000000 mov r9d, 0x10
140348031: 4889d9 mov rcx, rbx
140348034: e89752d0ff call 0x14004d2d0 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::resources::walls::WallId,system_decorator::decorator_visual::deco_differential_state::WallDecoDiff<system_decorator::decorator_visual::chimney_assemble
140348039: 488b9d78040000 mov rbx, qword ptr [rbp + 0x478]
140348040: 488b5338 mov rdx, qword ptr [rbx + 0x38]
140348044: 4885d2 test rdx, rdx
140348047: 7427 je 0x140348070
140348049: 488d04d517000000 lea rax, [rdx*8 + 0x17]
140348051: 4883e0f0 and rax, 0xfffffffffffffff0
140348055: 4801c2 add rdx, rax
140348058: 4883c211 add rdx, 0x11
14034805c: 7412 je 0x140348070
14034805e: 488b4b30 mov rcx, qword ptr [rbx + 0x30]
140348062: 4829c1 sub rcx, rax
140348065: 41b810000000 mov r8d, 0x10
14034806b: e8b0bb2c00 call 0x140613c20 ; unknown
140348070: 0f108520010000 movups xmm0, xmmword ptr [rbp + 0x120]
140348077: 0f114350 movups xmmword ptr [rbx + 0x50], xmm0
14034807b: 0f108510010000 movups xmm0, xmmword ptr [rbp + 0x110]
140348082: 0f114340 movups xmmword ptr [rbx + 0x40], xmm0
140348086: 0f1085d0000000 movups xmm0, xmmword ptr [rbp + 0xd0]
14034808d: 0f108de0000000 movups xmm1, xmmword ptr [rbp + 0xe0]
140348094: 0f1095f0000000 movups xmm2, xmmword ptr [rbp + 0xf0]
14034809b: 0f109d00010000 movups xmm3, xmmword ptr [rbp + 0x100]
1403480a2: 0f115b30 movups xmmword ptr [rbx + 0x30], xmm3
1403480a6: 0f115320 movups xmmword ptr [rbx + 0x20], xmm2
1403480aa: 0f114b10 movups xmmword ptr [rbx + 0x10], xmm1
1403480ae: 0f1103 movups xmmword ptr [rbx], xmm0
1403480b1: 488d4d40 lea rcx, [rbp + 0x40]
1403480b5: e886c0efff call 0x140244140 ; petgraph::graphmap::impl$19::default<country_core::resources::walls::decorator::DecoratorId,tuple$<>,enum2$<petgraph::Undirected>,std::hash::random::RandomState>
1403480ba: 488d8d60030000 lea rcx, [rbp + 0x360]
1403480c1: e87ac0efff call 0x140244140 ; petgraph::graphmap::impl$19::default<country_core::resources::walls::decorator::DecoratorId,tuple$<>,enum2$<petgraph::Undirected>,std::hash::random::RandomState>
1403480c6: 31c9 xor ecx, ecx
1403480c8: e8531ad9ff call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > 
1403480cd: 4885c0 test rax, rax
1403480d0: 0f84a1040000 je 0x140348577
1403480d6: 488b10 mov rdx, qword ptr [rax]
1403480d9: 488b4808 mov rcx, qword ptr [rax + 8]
1403480dd: 48898de0040000 mov qword ptr [rbp + 0x4e0], rcx
1403480e4: 488995d8040000 mov qword ptr [rbp + 0x4d8], rdx
1403480eb: 488d4a01 lea rcx, [rdx + 1]
1403480ef: 488908 mov qword ptr [rax], rcx
1403480f2: 488d8de0000000 lea rcx, [rbp + 0xe0]
1403480f9: 488d5540 lea rdx, [rbp + 0x40]
1403480fd: 41b890000000 mov r8d, 0x90
140348103: e8f8ba5d02 call 0x142923c00 ; unknown
140348108: 488d4db0 lea rcx, [rbp - 0x50]
14034810c: 488d9560030000 lea rdx, [rbp + 0x360]
140348113: 41b890000000 mov r8d, 0x90
140348119: e8e2ba5d02 call 0x142923c00 ; unknown
14034811e: 488b8518030000 mov rax, qword ptr [rbp + 0x318]
140348125: 8b8d54040000 mov ecx, dword ptr [rbp + 0x454]
14034812b: 8908 mov dword ptr [rax], ecx
14034812d: 4889f9 mov rcx, rdi
140348130: e88ba92b00 call 0x140602ac0 ; core::ptr::drop_in_place<country_core::resources::stairs::stairs_state::StairsState> | core::ptr::drop_in_place<country_core::resources::stairs::stairs_state::StairsState> | core::ptr::drop_in_place<country_core::resourc
140348135: 488bbd80040000 mov rdi, qword ptr [rbp + 0x480]
14034813c: 48c70700000000 mov qword ptr [rdi], 0
140348143: 48c7471003000000 mov qword ptr [rdi + 0x10], 3
14034814b: 488d4f18 lea rcx, [rdi + 0x18]
14034814f: 488d95d0000000 lea rdx, [rbp + 0xd0]
140348156: 41b8a0000000 mov r8d, 0xa0
14034815c: e89fba5d02 call 0x142923c00 ; unknown
140348161: 488d8fb8000000 lea rcx, [rdi + 0xb8]
140348168: 488d55b0 lea rdx, [rbp - 0x50]
14034816c: 41b890000000 mov r8d, 0x90
140348172: e889ba5d02 call 0x142923c00 ; unknown
140348177: 0f1005a27e6602 movups xmm0, xmmword ptr [rip + 0x2667ea2]
14034817e: 0f118748010000 movups xmmword ptr [rdi + 0x148], xmm0
140348185: 0f1005a47e6602 movups xmm0, xmmword ptr [rip + 0x2667ea4]
14034818c: 0f118758010000 movups xmmword ptr [rdi + 0x158], xmm0
140348193: 488b85d8040000 mov rax, qword ptr [rbp + 0x4d8]
14034819a: 48898768010000 mov qword ptr [rdi + 0x168], rax
1403481a1: 488b85e0040000 mov rax, qword ptr [rbp + 0x4e0]
1403481a8: 48898770010000 mov qword ptr [rdi + 0x170], rax
1403481af: c6877801000000 mov byte ptr [rdi + 0x178], 0
1403481b6: 31c9 xor ecx, ecx
1403481b8: e86319d9ff call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > 
1403481bd: 4885c0 test rax, rax
1403481c0: 0f8489030000 je 0x14034854f
1403481c6: 0f1000 movups xmm0, xmmword ptr [rax]
1403481c9: 48ff00 inc qword ptr [rax]
1403481cc: 0f2975c0 movaps xmmword ptr [rbp - 0x40], xmm6
1403481d0: 0f297db0 movaps xmmword ptr [rbp - 0x50], xmm7
1403481d4: 0f2945d0 movaps xmmword ptr [rbp - 0x30], xmm0
1403481d8: 31c9 xor ecx, ecx
1403481da: e84119d9ff call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > 
1403481df: 4885c0 test rax, rax
1403481e2: 0f849d030000 je 0x140348585
1403481e8: 0f1000 movups xmm0, xmmword ptr [rax]
1403481eb: 48ff00 inc qword ptr [rax]
1403481ee: 0f297550 movaps xmmword ptr [rbp + 0x50], xmm6
1403481f2: 0f297d40 movaps xmmword ptr [rbp + 0x40], xmm7
1403481f6: 0f294560 movaps xmmword ptr [rbp + 0x60], xmm0
1403481fa: 31c9 xor ecx, ecx
1403481fc: e81f19d9ff call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > 
140348201: 4885c0 test rax, rax
140348204: 0f8489030000 je 0x140348593
14034820a: 0f1000 movups xmm0, xmmword ptr [rax]
14034820d: 48ff00 inc qword ptr [rax]
140348210: 0f29b570030000 movaps xmmword ptr [rbp + 0x370], xmm6
140348217: 0f29bd60030000 movaps xmmword ptr [rbp + 0x360], xmm7
14034821e: 0f298580030000 movaps xmmword ptr [rbp + 0x380], xmm0
140348225: 31c9 xor ecx, ecx
140348227: e8f418d9ff call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > 
14034822c: 4885c0 test rax, rax
14034822f: 488b8d90040000 mov rcx, qword ptr [rbp + 0x490]
140348236: 0f8465030000 je 0x1403485a1
14034823c: 4c8b00 mov r8, qword ptr [rax]
14034823f: 488b5008 mov rdx, qword ptr [rax + 8]
140348243: 488995e0040000 mov qword ptr [rbp + 0x4e0], rdx
14034824a: 4c8985d8040000 mov qword ptr [rbp + 0x4d8], r8
140348251: 498d5001 lea rdx, [r8 + 1]
140348255: 488910 mov qword ptr [rax], rdx
140348258: 0f2845b0 movaps xmm0, xmmword ptr [rbp - 0x50]
14034825c: 0f284dc0 movaps xmm1, xmmword ptr [rbp - 0x40]
140348260: 0f2855d0 movaps xmm2, xmmword ptr [rbp - 0x30]
140348264: 0f2995f0000000 movaps xmmword ptr [rbp + 0xf0], xmm2
14034826b: 0f298de0000000 movaps xmmword ptr [rbp + 0xe0], xmm1
140348272: 0f2985d0000000 movaps xmmword ptr [rbp + 0xd0], xmm0
140348279: 0f284540 movaps xmm0, xmmword ptr [rbp + 0x40]
14034827d: 0f284d50 movaps xmm1, xmmword ptr [rbp + 0x50]
140348281: 0f285560 movaps xmm2, xmmword ptr [rbp + 0x60]
140348285: 0f298500010000 movaps xmmword ptr [rbp + 0x100], xmm0
14034828c: 0f298d10010000 movaps xmmword ptr [rbp + 0x110], xmm1
140348293: 0f299520010000 movaps xmmword ptr [rbp + 0x120], xmm2
14034829a: 0f288560030000 movaps xmm0, xmmword ptr [rbp + 0x360]
1403482a1: 0f288d70030000 movaps xmm1, xmmword ptr [rbp + 0x370]
1403482a8: 0f289580030000 movaps xmm2, xmmword ptr [rbp + 0x380]
1403482af: 0f298530010000 movaps xmmword ptr [rbp + 0x130], xmm0
1403482b6: 0f298d40010000 movaps xmmword ptr [rbp + 0x140], xmm1
1403482bd: 0f299550010000 movaps xmmword ptr [rbp + 0x150], xmm2
1403482c4: 0f29b570010000 movaps xmmword ptr [rbp + 0x170], xmm6
1403482cb: 0f29bd60010000 movaps xmmword ptr [rbp + 0x160], xmm7
1403482d2: 488b8520030000 mov rax, qword ptr [rbp + 0x320]
1403482d9: 8b9558040000 mov edx, dword ptr [rbp + 0x458]
1403482df: 8910 mov dword ptr [rax], edx
1403482e1: e8fa262c00 call 0x14060a9e0 ; core::ptr::drop_in_place<country_core::resources::stairs::stairs_collider::StairColliderRegistry>
1403482e6: 488d95d0000000 lea rdx, [rbp + 0xd0]
1403482ed: 41b8b0000000 mov r8d, 0xb0
1403482f3: 488bbd90040000 mov rdi, qword ptr [rbp + 0x490]
1403482fa: 4889f9 mov rcx, rdi
1403482fd: e8feb85d02 call 0x142923c00 ; unknown
140348302: 488b85d8040000 mov rax, qword ptr [rbp + 0x4d8]
140348309: 488987b0000000 mov qword ptr [rdi + 0xb0], rax
140348310: 488b85e0040000 mov rax, qword ptr [rbp + 0x4e0]
140348317: 488987b8000000 mov qword ptr [rdi + 0xb8], rax
14034831e: 488d8dd0000000 lea rcx, [rbp + 0xd0]
140348325: e81636dcff call 0x14010b940 ; system_decorator::decorator_visual::deco_differential_state::impl$2::default<system_decorator::decorator_visual::door_stairs_assemble_bricks::DoorStairsAssemblyParams>
14034832a: 488b8528030000 mov rax, qword ptr [rbp + 0x328]
140348331: 8b8d5c040000 mov ecx, dword ptr [rbp + 0x45c]
140348337: 8908 mov dword ptr [rax], ecx
140348339: 488d5620 lea rdx, [rsi + 0x20]
14034833d: 41b838000000 mov r8d, 0x38
140348343: 41b910000000 mov r9d, 0x10
140348349: 4889f1 mov rcx, rsi
14034834c: e84f4ad0ff call 0x14004cda0 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::resources::walls::WallId,system_decorator::decorator_visual::deco_differential_state::WallDecoDiff<system_decorator::decorator_visual::door_stairs_asse
140348351: 488bb588040000 mov rsi, qword ptr [rbp + 0x488]
140348358: 488b5638 mov rdx, qword ptr [rsi + 0x38]
14034835c: 4885d2 test rdx, rdx
14034835f: 7427 je 0x140348388
140348361: 488d04d517000000 lea rax, [rdx*8 + 0x17]
140348369: 4883e0f0 and rax, 0xfffffffffffffff0
14034836d: 4801c2 add rdx, rax
140348370: 4883c211 add rdx, 0x11
140348374: 7412 je 0x140348388
140348376: 488b4e30 mov rcx, qword ptr [rsi + 0x30]
14034837a: 4829c1 sub rcx, rax
14034837d: 41b810000000 mov r8d, 0x10
140348383: e898b82c00 call 0x140613c20 ; unknown
140348388: 0f108520010000 movups xmm0, xmmword ptr [rbp + 0x120]
14034838f: 0f114650 movups xmmword ptr [rsi + 0x50], xmm0
140348393: 0f108510010000 movups xmm0, xmmword ptr [rbp + 0x110]
14034839a: 0f114640 movups xmmword ptr [rsi + 0x40], xmm0
14034839e: 0f1085d0000000 movups xmm0, xmmword ptr [rbp + 0xd0]
1403483a5: 0f108de0000000 movups xmm1, xmmword ptr [rbp + 0xe0]
1403483ac: 0f1095f0000000 movups xmm2, xmmword ptr [rbp + 0xf0]
1403483b3: 0f109d00010000 movups xmm3, xmmword ptr [rbp + 0x100]
1403483ba: 0f115e30 movups xmmword ptr [rsi + 0x30], xmm3
1403483be: 0f115620 movups xmmword ptr [rsi + 0x20], xmm2
1403483c2: 0f114e10 movups xmmword ptr [rsi + 0x10], xmm1
1403483c6: 0f1106 movups xmmword ptr [rsi], xmm0
1403483c9: e87253a000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
1403483ce: 4989c6 mov r14, rax
1403483d1: f60001 test byte ptr [rax], 1
1403483d4: 7418 je 0x1403483ee
1403483d6: 4d89f2 mov r10, r14
1403483d9: 4983c208 add r10, 8
1403483dd: 498b4608 mov rax, qword ptr [r14 + 8]
1403483e1: 498b4e10 mov rcx, qword ptr [r14 + 0x10]
1403483e5: 48898de0040000 mov qword ptr [rbp + 0x4e0], rcx
1403483ec: eb22 jmp 0x140348410 ; unknown
1403483ee: e80dcaa100 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
1403483f3: 4d89f2 mov r10, r14
1403483f6: 4983c208 add r10, 8
1403483fa: 49c70601000000 mov qword ptr [r14], 1
140348401: 49894608 mov qword ptr [r14 + 8], rax
140348405: 488995e0040000 mov qword ptr [rbp + 0x4e0], rdx
14034840c: 49895610 mov qword ptr [r14 + 0x10], rdx
140348410: 488b8db0040000 mov rcx, qword ptr [rbp + 0x4b0]
140348417: 4c8b8520040000 mov r8, qword ptr [rbp + 0x420]
14034841e: 448b8dc4040000 mov r9d, dword ptr [rbp + 0x4c4]
140348425: 488985d8040000 mov qword ptr [rbp + 0x4d8], rax
14034842c: 488d5001 lea rdx, [rax + 1]
140348430: 498912 mov qword ptr [r10], rdx
140348433: 458908 mov dword ptr [r8], r9d
140348436: 488d5120 lea rdx, [rcx + 0x20]
14034843a: 41b810010000 mov r8d, 0x110
140348440: 41b910000000 mov r9d, 0x10
140348446: e85551d0ff call 0x14004d5a0 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::resources::stairs::visual::StairsEdgeVisualState>,alloc::alloc::Global>
14034844b: 488bbd30030000 mov rdi, qword ptr [rbp + 0x330]
140348452: 0f100507aa6702 movups xmm0, xmmword ptr [rip + 0x267aa07]
140348459: 488b8db0040000 mov rcx, qword ptr [rbp + 0x4b0]
140348460: 0f114110 movups xmmword ptr [rcx + 0x10], xmm0
140348464: 0f1005e5a96702 movups xmm0, xmmword ptr [rip + 0x267a9e5]
14034846b: 0f1101 movups xmmword ptr [rcx], xmm0
14034846e: 488b85d8040000 mov rax, qword ptr [rbp + 0x4d8]
140348475: 48894120 mov qword ptr [rcx + 0x20], rax
140348479: 488b85e0040000 mov rax, qword ptr [rbp + 0x4e0]
140348480: 48894128 mov qword ptr [rcx + 0x28], rax
140348484: c6413000 mov byte ptr [rcx + 0x30], 0
140348488: 488b8558030000 mov rax, qword ptr [rbp + 0x358]
14034848f: 8b8d64040000 mov ecx, dword ptr [rbp + 0x464]
140348495: 8908 mov dword ptr [rax], ecx
140348497: 488b8d50030000 mov rcx, qword ptr [rbp + 0x350]
14034849e: e88d155200 call 0x140869a30 ; country_core::resources::stairs::structural_graph::StairsStructuralGraph::clear
1403484a3: 488b8540030000 mov rax, qword ptr [rbp + 0x340]
1403484aa: 8b8d60040000 mov ecx, dword ptr [rbp + 0x460]
1403484b0: 8908 mov dword ptr [rax], ecx
1403484b2: 488b8d38030000 mov rcx, qword ptr [rbp + 0x338]
1403484b9: e8726e4600 call 0x1407af330 ; system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher::clear
1403484be: 488b8548030000 mov rax, qword ptr [rbp + 0x348]
1403484c5: 4885c0 test rax, rax
1403484c8: 480f45f8 cmovne rdi, rax
1403484cc: 488b07 mov rax, qword ptr [rdi]
1403484cf: 488b7710 mov rsi, qword ptr [rdi + 0x10]
1403484d3: 4829f0 sub rax, rsi
1403484d6: 4883f807 cmp rax, 7
1403484da: 7651 jbe 0x14034852d
1403484dc: 488b4708 mov rax, qword ptr [rdi + 8]
1403484e0: 488d0dd950e0ff lea rcx, [rip - 0x1faf27]
1403484e7: 48890c30 mov qword ptr [rax + rsi], rcx
1403484eb: 4883c608 add rsi, 8
1403484ef: 48897710 mov qword ptr [rdi + 0x10], rsi
1403484f3: 0f28b5f0040000 movaps xmm6, xmmword ptr [rbp + 0x4f0]
1403484fa: 0f28bd00050000 movaps xmm7, xmmword ptr [rbp + 0x500]
140348501: 440f288510050000 movaps xmm8, xmmword ptr [rbp + 0x510]
140348509: 440f288d20050000 movaps xmm9, xmmword ptr [rbp + 0x520]
140348511: 440f289530050000 movaps xmm10, xmmword ptr [rbp + 0x530]
140348519: 4881c4c8050000 add rsp, 0x5c8
140348520: 5b pop rbx
140348521: 5f pop rdi
140348522: 5e pop rsi
140348523: 415c pop r12
140348525: 415d pop r13
140348527: 415e pop r14
140348529: 415f pop r15
14034852b: 5d pop rbp
14034852c: c3 ret 
14034852d: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
140348536: 41b808000000 mov r8d, 8
14034853c: 41b901000000 mov r9d, 1
140348542: 4889f9 mov rcx, rdi
140348545: 4889f2 mov rdx, rsi
140348548: e893a45502 call 0x1428a29e0 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
14034854d: eb8d jmp 0x1403484dc ; unknown
14034854f: 488d0dbaa45e02 lea rcx, [rip + 0x25ea4ba]
140348556: e875fb5802 call 0x1428d80d0 ; std::thread::local::panic_access_error
14034855b: 488d0daea45e02 lea rcx, [rip + 0x25ea4ae]
140348562: e869fb5802 call 0x1428d80d0 ; std::thread::local::panic_access_error
140348567: eb44 jmp 0x1403485ad ; unknown
140348569: 488d0da0a45e02 lea rcx, [rip + 0x25ea4a0]
140348570: e85bfb5802 call 0x1428d80d0 ; std::thread::local::panic_access_error
140348575: eb36 jmp 0x1403485ad ; unknown
140348577: 488d0d92a45e02 lea rcx, [rip + 0x25ea492]
14034857e: e84dfb5802 call 0x1428d80d0 ; std::thread::local::panic_access_error
140348583: eb28 jmp 0x1403485ad ; unknown
140348585: 488d0d84a45e02 lea rcx, [rip + 0x25ea484]
14034858c: e83ffb5802 call 0x1428d80d0 ; std::thread::local::panic_access_error
140348591: eb1a jmp 0x1403485ad ; unknown
140348593: 488d0d76a45e02 lea rcx, [rip + 0x25ea476]
14034859a: e831fb5802 call 0x1428d80d0 ; std::thread::local::panic_access_error
14034859f: eb0c jmp 0x1403485ad ; unknown
1403485a1: 488d0d68a45e02 lea rcx, [rip + 0x25ea468]
1403485a8: e823fb5802 call 0x1428d80d0 ; std::thread::local::panic_access_error
1403485ad: 0f0b ud2 
1403485af: 90 nop 
1403485b0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1403485b5: 55 push rbp
1403485b6: 4157 push r15
1403485b8: 4156 push r14
1403485ba: 4155 push r13
1403485bc: 4154 push r12
1403485be: 56 push rsi
1403485bf: 57 push rdi
1403485c0: 53 push rbx
1403485c1: 4883ec78 sub rsp, 0x78
1403485c5: 488daa80000000 lea rbp, [rdx + 0x80]
1403485cc: 440f29542420 movaps xmmword ptr [rsp + 0x20], xmm10
1403485d2: 440f294c2430 movaps xmmword ptr [rsp + 0x30], xmm9
1403485d8: 440f29442440 movaps xmmword ptr [rsp + 0x40], xmm8
1403485de: 0f297c2450 movaps xmmword ptr [rsp + 0x50], xmm7
1403485e3: 0f29742460 movaps xmmword ptr [rsp + 0x60], xmm6
1403485e8: 488d4d40 lea rcx, [rbp + 0x40]
1403485ec: 41b898000000 mov r8d, 0x98
1403485f2: 41b910000000 mov r9d, 0x10
1403485f8: 488d5560 lea rdx, [rbp + 0x60]
1403485fc: e8ff82d0ff call 0x140050900 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,alloc::alloc::Global> | hashbrown::raw::RawTableInner::drop_inn
140348601: 0f28742460 movaps xmm6, xmmword ptr [rsp + 0x60]
140348606: 0f287c2450 movaps xmm7, xmmword ptr [rsp + 0x50]
14034860b: 440f28442440 movaps xmm8, xmmword ptr [rsp + 0x40]
140348611: 440f284c2430 movaps xmm9, xmmword ptr [rsp + 0x30]
140348617: 440f28542420 movaps xmm10, xmmword ptr [rsp + 0x20]
14034861d: 4883c478 add rsp, 0x78
140348621: 5b pop rbx
140348622: 5f pop rdi
140348623: 5e pop rsi
140348624: 415c pop r12
140348626: 415d pop r13
140348628: 415e pop r14
14034862a: 415f pop r15
14034862c: 5d pop rbp
14034862d: c3 ret 
14034862e: 6690 nop 
140348630: 4889542410 mov qword ptr [rsp + 0x10], rdx
140348635: 55 push rbp
140348636: 4157 push r15
140348638: 4156 push r14
14034863a: 4155 push r13
14034863c: 4154 push r12
14034863e: 56 push rsi
14034863f: 57 push rdi
140348640: 53 push rbx
140348641: 4883ec78 sub rsp, 0x78
140348645: 488daa80000000 lea rbp, [rdx + 0x80]
14034864c: 440f29542420 movaps xmmword ptr [rsp + 0x20], xmm10
140348652: 440f294c2430 movaps xmmword ptr [rsp + 0x30], xmm9
140348658: 440f29442440 movaps xmmword ptr [rsp + 0x40], xmm8
14034865e: 0f297c2450 movaps xmmword ptr [rsp + 0x50], xmm7
140348663: 0f29742460 movaps xmmword ptr [rsp + 0x60], xmm6
140348668: 488d8d60030000 lea rcx, [rbp + 0x360]
14034866f: 41b840000000 mov r8d, 0x40
140348675: 41b910000000 mov r9d, 0x10
14034867b: 488d9580030000 lea rdx, [rbp + 0x380]
140348682: e8393fd0ff call 0x14004c5c0 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,std::collections::hash::set::HashSet<country_core::resources::walls::decorator::Decor
140348687: 0f28742460 movaps xmm6, xmmword ptr [rsp + 0x60]
14034868c: 0f287c2450 movaps xmm7, xmmword ptr [rsp + 0x50]
140348691: 440f28442440 movaps xmm8, xmmword ptr [rsp + 0x40]
140348697: 440f284c2430 movaps xmm9, xmmword ptr [rsp + 0x30]
14034869d: 440f28542420 movaps xmm10, xmmword ptr [rsp + 0x20]
1403486a3: 4883c478 add rsp, 0x78
1403486a7: 5b pop rbx
1403486a8: 5f pop rdi
1403486a9: 5e pop rsi
1403486aa: 415c pop r12
1403486ac: 415d pop r13
1403486ae: 415e pop r14
1403486b0: 415f pop r15
1403486b2: 5d pop rbp
1403486b3: c3 ret 
1403486b4: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1403486c0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1403486c5: 55 push rbp
1403486c6: 4157 push r15
1403486c8: 4156 push r14
1403486ca: 4155 push r13
1403486cc: 4154 push r12
1403486ce: 56 push rsi
1403486cf: 57 push rdi
1403486d0: 53 push rbx
1403486d1: 4883ec78 sub rsp, 0x78
1403486d5: 488daa80000000 lea rbp, [rdx + 0x80]
1403486dc: 440f29542420 movaps xmmword ptr [rsp + 0x20], xmm10
1403486e2: 440f294c2430 movaps xmmword ptr [rsp + 0x30], xmm9
1403486e8: 440f29442440 movaps xmmword ptr [rsp + 0x40], xmm8
1403486ee: 0f297c2450 movaps xmmword ptr [rsp + 0x50], xmm7
1403486f3: 0f29742460 movaps xmmword ptr [rsp + 0x60], xmm6
1403486f8: 488d8dd0000000 lea rcx, [rbp + 0xd0]
1403486ff: 41b828000000 mov r8d, 0x28
140348705: 41b910000000 mov r9d, 0x10
14034870b: 488d95f0000000 lea rdx, [rbp + 0xf0]
140348712: e8e982d0ff call 0x140050a00 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::resources::walls::decorator::DecoratorId,enum2$<system_decorator::decorator_visual_state::PersistentVisualEntities> >,alloc::alloc::Global>
140348717: 0f28742460 movaps xmm6, xmmword ptr [rsp + 0x60]
14034871c: 0f287c2450 movaps xmm7, xmmword ptr [rsp + 0x50]
140348721: 440f28442440 movaps xmm8, xmmword ptr [rsp + 0x40]
140348727: 440f284c2430 movaps xmm9, xmmword ptr [rsp + 0x30]
14034872d: 440f28542420 movaps xmm10, xmmword ptr [rsp + 0x20]
140348733: 4883c478 add rsp, 0x78
140348737: 5b pop rbx
140348738: 5f pop rdi
140348739: 5e pop rsi
14034873a: 415c pop r12
14034873c: 415d pop r13
14034873e: 415e pop r14
140348740: 415f pop r15
140348742: 5d pop rbp
140348743: c3 ret 
140348744: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140348750: 4889542410 mov qword ptr [rsp + 0x10], rdx
140348755: 55 push rbp
140348756: 4157 push r15
140348758: 4156 push r14
14034875a: 4155 push r13
14034875c: 4154 push r12
14034875e: 56 push rsi
14034875f: 57 push rdi
140348760: 53 push rbx
140348761: 4883ec78 sub rsp, 0x78
140348765: 488daa80000000 lea rbp, [rdx + 0x80]
14034876c: 440f29542420 movaps xmmword ptr [rsp + 0x20], xmm10
140348772: 440f294c2430 movaps xmmword ptr [rsp + 0x30], xmm9
140348778: 440f29442440 movaps xmmword ptr [rsp + 0x40], xmm8
14034877e: 0f297c2450 movaps xmmword ptr [rsp + 0x50], xmm7
140348783: 0f29742460 movaps xmmword ptr [rsp + 0x60], xmm6
140348788: 488d4db0 lea rcx, [rbp - 0x50]
14034878c: e81f902900 call 0x1405e17b0 ; core::ptr::drop_in_place<std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,tuple$<f32,glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat>,std::hash::random::RandomState> > | core:
140348791: 0f28742460 movaps xmm6, xmmword ptr [rsp + 0x60]
140348796: 0f287c2450 movaps xmm7, xmmword ptr [rsp + 0x50]
14034879b: 440f28442440 movaps xmm8, xmmword ptr [rsp + 0x40]
1403487a1: 440f284c2430 movaps xmm9, xmmword ptr [rsp + 0x30]
1403487a7: 440f28542420 movaps xmm10, xmmword ptr [rsp + 0x20]
1403487ad: 4883c478 add rsp, 0x78
1403487b1: 5b pop rbx
1403487b2: 5f pop rdi
1403487b3: 5e pop rsi
1403487b4: 415c pop r12
1403487b6: 415d pop r13
1403487b8: 415e pop r14
1403487ba: 415f pop r15
1403487bc: 5d pop rbp
1403487bd: c3 ret 
1403487be: 6690 nop 
1403487c0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1403487c5: 55 push rbp
1403487c6: 4157 push r15
1403487c8: 4156 push r14
1403487ca: 4155 push r13
1403487cc: 4154 push r12
1403487ce: 56 push rsi
1403487cf: 57 push rdi
1403487d0: 53 push rbx
1403487d1: 4883ec78 sub rsp, 0x78
1403487d5: 488daa80000000 lea rbp, [rdx + 0x80]
1403487dc: 440f29542420 movaps xmmword ptr [rsp + 0x20], xmm10
1403487e2: 440f294c2430 movaps xmmword ptr [rsp + 0x30], xmm9
1403487e8: 440f29442440 movaps xmmword ptr [rsp + 0x40], xmm8
1403487ee: 0f297c2450 movaps xmmword ptr [rsp + 0x50], xmm7
1403487f3: 0f29742460 movaps xmmword ptr [rsp + 0x60], xmm6
1403487f8: 488d4d40 lea rcx, [rbp + 0x40]
1403487fc: e8af8f2900 call 0x1405e17b0 ; core::ptr::drop_in_place<std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,tuple$<f32,glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat>,std::hash::random::RandomState> > | core:
140348801: 0f28742460 movaps xmm6, xmmword ptr [rsp + 0x60]
140348806: 0f287c2450 movaps xmm7, xmmword ptr [rsp + 0x50]
14034880b: 440f28442440 movaps xmm8, xmmword ptr [rsp + 0x40]
140348811: 440f284c2430 movaps xmm9, xmmword ptr [rsp + 0x30]
140348817: 440f28542420 movaps xmm10, xmmword ptr [rsp + 0x20]
14034881d: 4883c478 add rsp, 0x78
140348821: 5b pop rbx
140348822: 5f pop rdi
140348823: 5e pop rsi
140348824: 415c pop r12
140348826: 415d pop r13
140348828: 415e pop r14
14034882a: 415f pop r15
14034882c: 5d pop rbp
14034882d: c3 ret 
14034882e: 6690 nop 
140348830: 4889542410 mov qword ptr [rsp + 0x10], rdx
140348835: 55 push rbp
140348836: 4157 push r15
140348838: 4156 push r14
14034883a: 4155 push r13
14034883c: 4154 push r12
14034883e: 56 push rsi
14034883f: 57 push rdi
140348840: 53 push rbx
140348841: 4883ec78 sub rsp, 0x78
140348845: 488daa80000000 lea rbp, [rdx + 0x80]
14034884c: 440f29542420 movaps xmmword ptr [rsp + 0x20], xmm10
140348852: 440f294c2430 movaps xmmword ptr [rsp + 0x30], xmm9
140348858: 440f29442440 movaps xmmword ptr [rsp + 0x40], xmm8
14034885e: 0f297c2450 movaps xmmword ptr [rsp + 0x50], xmm7
140348863: 0f29742460 movaps xmmword ptr [rsp + 0x60], xmm6
140348868: 488d8d60030000 lea rcx, [rbp + 0x360]
14034886f: e8ec22e0ff call 0x14014ab60 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms> | core::ptr::drop_in_place<country_core::resources::walls::cached_decorator_trans
140348874: 0f28742460 movaps xmm6, xmmword ptr [rsp + 0x60]
140348879: 0f287c2450 movaps xmm7, xmmword ptr [rsp + 0x50]
14034887e: 440f28442440 movaps xmm8, xmmword ptr [rsp + 0x40]
140348884: 440f284c2430 movaps xmm9, xmmword ptr [rsp + 0x30]
14034888a: 440f28542420 movaps xmm10, xmmword ptr [rsp + 0x20]
140348890: 4883c478 add rsp, 0x78
140348894: 5b pop rbx
140348895: 5f pop rdi
140348896: 5e pop rsi
140348897: 415c pop r12
140348899: 415d pop r13
14034889b: 415e pop r14
14034889d: 415f pop r15
14034889f: 5d pop rbp
1403488a0: c3 ret 
1403488a1: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1403488b0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1403488b5: 55 push rbp
1403488b6: 4157 push r15
1403488b8: 4156 push r14
1403488ba: 4155 push r13
1403488bc: 4154 push r12
1403488be: 56 push rsi
1403488bf: 57 push rdi
1403488c0: 53 push rbx
1403488c1: 4883ec78 sub rsp, 0x78
1403488c5: 488daa80000000 lea rbp, [rdx + 0x80]
1403488cc: 440f29542420 movaps xmmword ptr [rsp + 0x20], xmm10
1403488d2: 440f294c2430 movaps xmmword ptr [rsp + 0x30], xmm9
1403488d8: 440f29442440 movaps xmmword ptr [rsp + 0x40], xmm8
1403488de: 0f297c2450 movaps xmmword ptr [rsp + 0x50], xmm7
1403488e3: 0f29742460 movaps xmmword ptr [rsp + 0x60], xmm6
1403488e8: 488d8d60030000 lea rcx, [rbp + 0x360]
1403488ef: e86c1ae0ff call 0x14014a360 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::resources::window_lights::GlobalWindowLightsControl> | core::ptr::drop_in_place<country_core::systems::collision::world::ColliderRegistry<country_core::res
1403488f4: 0f28742460 movaps xmm6, xmmword ptr [rsp + 0x60]
1403488f9: 0f287c2450 movaps xmm7, xmmword ptr [rsp + 0x50]
1403488fe: 440f28442440 movaps xmm8, xmmword ptr [rsp + 0x40]
140348904: 440f284c2430 movaps xmm9, xmmword ptr [rsp + 0x30]
14034890a: 440f28542420 movaps xmm10, xmmword ptr [rsp + 0x20]
140348910: 4883c478 add rsp, 0x78
140348914: 5b pop rbx
140348915: 5f pop rdi
140348916: 5e pop rsi
140348917: 415c pop r12
140348919: 415d pop r13
14034891b: 415e pop r14
14034891d: 415f pop r15
14034891f: 5d pop rbp
140348920: c3 ret 
140348921: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140348930: 4889542410 mov qword ptr [rsp + 0x10], rdx
140348935: 55 push rbp
140348936: 4157 push r15
140348938: 4156 push r14
14034893a: 4155 push r13
14034893c: 4154 push r12
14034893e: 56 push rsi
14034893f: 57 push rdi
140348940: 53 push rbx
140348941: 4883ec78 sub rsp, 0x78
140348945: 488daa80000000 lea rbp, [rdx + 0x80]
14034894c: 440f29542420 movaps xmmword ptr [rsp + 0x20], xmm10
140348952: 440f294c2430 movaps xmmword ptr [rsp + 0x30], xmm9
140348958: 440f29442440 movaps xmmword ptr [rsp + 0x40], xmm8
14034895e: 0f297c2450 movaps xmmword ptr [rsp + 0x50], xmm7
140348963: 0f29742460 movaps xmmword ptr [rsp + 0x60], xmm6
140348968: 488d8dd0000000 lea rcx, [rbp + 0xd0]
14034896f: 41b820000000 mov r8d, 0x20
140348975: 41b910000000 mov r9d, 0x10
14034897b: 488d95f0000000 lea rdx, [rbp + 0xf0]
140348982: e8c976d0ff call 0x140050050 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_stone_columns::StoneColumn,alloc::alloc::Global> >,alloc::alloc::Global>
140348987: 0f28742460 movaps xmm6, xmmword ptr [rsp + 0x60]
14034898c: 0f287c2450 movaps xmm7, xmmword ptr [rsp + 0x50]
140348991: 440f28442440 movaps xmm8, xmmword ptr [rsp + 0x40]
140348997: 440f284c2430 movaps xmm9, xmmword ptr [rsp + 0x30]
14034899d: 440f28542420 movaps xmm10, xmmword ptr [rsp + 0x20]
1403489a3: 4883c478 add rsp, 0x78
1403489a7: 5b pop rbx
1403489a8: 5f pop rdi
1403489a9: 5e pop rsi
1403489aa: 415c pop r12
1403489ac: 415d pop r13
1403489ae: 415e pop r14
1403489b0: 415f pop r15
1403489b2: 5d pop rbp
1403489b3: c3 ret 
1403489b4: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1403489c0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1403489c5: 55 push rbp
1403489c6: 4157 push r15
1403489c8: 4156 push r14
1403489ca: 4155 push r13
1403489cc: 4154 push r12
1403489ce: 56 push rsi
1403489cf: 57 push rdi
1403489d0: 53 push rbx
1403489d1: 4883ec78 sub rsp, 0x78
1403489d5: 488daa80000000 lea rbp, [rdx + 0x80]
1403489dc: 440f29542420 movaps xmmword ptr [rsp + 0x20], xmm10
1403489e2: 440f294c2430 movaps xmmword ptr [rsp + 0x30], xmm9
1403489e8: 440f29442440 movaps xmmword ptr [rsp + 0x40], xmm8
1403489ee: 0f297c2450 movaps xmmword ptr [rsp + 0x50], xmm7
1403489f3: 0f29742460 movaps xmmword ptr [rsp + 0x60], xmm6
1403489f8: 488b8d70040000 mov rcx, qword ptr [rbp + 0x470]
1403489ff: 48c70100000000 mov qword ptr [rcx], 0
140348a06: 48c7410808000000 mov qword ptr [rcx + 8], 8
140348a0e: 48c7411000000000 mov qword ptr [rcx + 0x10], 0
140348a16: 0f288560030000 movaps xmm0, xmmword ptr [rbp + 0x360]
140348a1d: 0f288d70030000 movaps xmm1, xmmword ptr [rbp + 0x370]
140348a24: 0f289580030000 movaps xmm2, xmmword ptr [rbp + 0x380]
140348a2b: 0f114118 movups xmmword ptr [rcx + 0x18], xmm0
140348a2f: 0f114928 movups xmmword ptr [rcx + 0x28], xmm1
140348a33: 0f115138 movups xmmword ptr [rcx + 0x38], xmm2
140348a37: 0f100562ae6502 movups xmm0, xmmword ptr [rip + 0x265ae62]
140348a3e: 0f114148 movups xmmword ptr [rcx + 0x48], xmm0
140348a42: 0f100567ae6502 movups xmm0, xmmword ptr [rip + 0x265ae67]
140348a49: 0f114158 movups xmmword ptr [rcx + 0x58], xmm0
140348a4d: 488b85d8040000 mov rax, qword ptr [rbp + 0x4d8]
140348a54: 48894168 mov qword ptr [rcx + 0x68], rax
140348a58: 488b85e0040000 mov rax, qword ptr [rbp + 0x4e0]
140348a5f: 48894170 mov qword ptr [rcx + 0x70], rax
140348a63: 0f28742460 movaps xmm6, xmmword ptr [rsp + 0x60]
140348a68: 0f287c2450 movaps xmm7, xmmword ptr [rsp + 0x50]
140348a6d: 440f28442440 movaps xmm8, xmmword ptr [rsp + 0x40]
140348a73: 440f284c2430 movaps xmm9, xmmword ptr [rsp + 0x30]
140348a79: 440f28542420 movaps xmm10, xmmword ptr [rsp + 0x20]
140348a7f: 4883c478 add rsp, 0x78
140348a83: 5b pop rbx
140348a84: 5f pop rdi
140348a85: 5e pop rsi
140348a86: 415c pop r12
140348a88: 415d pop r13
140348a8a: 415e pop r14
140348a8c: 415f pop r15
140348a8e: 5d pop rbp
140348a8f: c3 ret 
140348a90: 4889542410 mov qword ptr [rsp + 0x10], rdx
140348a95: 55 push rbp
140348a96: 4157 push r15
140348a98: 4156 push r14
140348a9a: 4155 push r13
140348a9c: 4154 push r12
140348a9e: 56 push rsi
140348a9f: 57 push rdi
140348aa0: 53 push rbx
140348aa1: 4883ec78 sub rsp, 0x78
140348aa5: 488daa80000000 lea rbp, [rdx + 0x80]
140348aac: 440f29542420 movaps xmmword ptr [rsp + 0x20], xmm10
140348ab2: 440f294c2430 movaps xmmword ptr [rsp + 0x30], xmm9
140348ab8: 440f29442440 movaps xmmword ptr [rsp + 0x40], xmm8
140348abe: 0f297c2450 movaps xmmword ptr [rsp + 0x50], xmm7
140348ac3: 0f29742460 movaps xmmword ptr [rsp + 0x60], xmm6
140348ac8: 488d4d40 lea rcx, [rbp + 0x40]
140348acc: e87f422900 call 0x1405dcd50 ; core::ptr::drop_in_place<petgraph::graphmap::GraphMap<country_core::resources::walls::decorator::DecoratorId,tuple$<>,enum2$<petgraph::Undirected>,std::hash::random::RandomState> >
140348ad1: 0f28742460 movaps xmm6, xmmword ptr [rsp + 0x60]
140348ad6: 0f287c2450 movaps xmm7, xmmword ptr [rsp + 0x50]
140348adb: 440f28442440 movaps xmm8, xmmword ptr [rsp + 0x40]
140348ae1: 440f284c2430 movaps xmm9, xmmword ptr [rsp + 0x30]
140348ae7: 440f28542420 movaps xmm10, xmmword ptr [rsp + 0x20]
140348aed: 4883c478 add rsp, 0x78
140348af1: 5b pop rbx
140348af2: 5f pop rdi
140348af3: 5e pop rsi
140348af4: 415c pop r12
140348af6: 415d pop r13
140348af8: 415e pop r14
140348afa: 415f pop r15
140348afc: 5d pop rbp
140348afd: c3 ret 
140348afe: 6690 nop 
140348b00: 4889542410 mov qword ptr [rsp + 0x10], rdx
140348b05: 55 push rbp
140348b06: 4157 push r15
140348b08: 4156 push r14
140348b0a: 4155 push r13
140348b0c: 4154 push r12
140348b0e: 56 push rsi
140348b0f: 57 push rdi
140348b10: 53 push rbx
140348b11: 4883ec78 sub rsp, 0x78
140348b15: 488daa80000000 lea rbp, [rdx + 0x80]
140348b1c: 440f29542420 movaps xmmword ptr [rsp + 0x20], xmm10
140348b22: 440f294c2430 movaps xmmword ptr [rsp + 0x30], xmm9
140348b28: 440f29442440 movaps xmmword ptr [rsp + 0x40], xmm8
140348b2e: 0f297c2450 movaps xmmword ptr [rsp + 0x50], xmm7
140348b33: 0f29742460 movaps xmmword ptr [rsp + 0x60], xmm6
140348b38: 488d8d60030000 lea rcx, [rbp + 0x360]
140348b3f: e80c422900 call 0x1405dcd50 ; core::ptr::drop_in_place<petgraph::graphmap::GraphMap<country_core::resources::walls::decorator::DecoratorId,tuple$<>,enum2$<petgraph::Undirected>,std::hash::random::RandomState> >
140348b44: 0f28742460 movaps xmm6, xmmword ptr [rsp + 0x60]
140348b49: 0f287c2450 movaps xmm7, xmmword ptr [rsp + 0x50]
140348b4e: 440f28442440 movaps xmm8, xmmword ptr [rsp + 0x40]
140348b54: 440f284c2430 movaps xmm9, xmmword ptr [rsp + 0x30]
140348b5a: 440f28542420 movaps xmm10, xmmword ptr [rsp + 0x20]
140348b60: 4883c478 add rsp, 0x78
140348b64: 5b pop rbx
140348b65: 5f pop rdi
140348b66: 5e pop rsi
140348b67: 415c pop r12
140348b69: 415d pop r13
140348b6b: 415e pop r14
140348b6d: 415f pop r15
140348b6f: 5d pop rbp
140348b70: c3 ret 
140348b71: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140348b80: 4889542410 mov qword ptr [rsp + 0x10], rdx
140348b85: 55 push rbp
140348b86: 4157 push r15
140348b88: 4156 push r14
140348b8a: 4155 push r13
140348b8c: 4154 push r12
140348b8e: 56 push rsi
140348b8f: 57 push rdi
140348b90: 53 push rbx
140348b91: 4883ec78 sub rsp, 0x78
140348b95: 488daa80000000 lea rbp, [rdx + 0x80]
140348b9c: 440f29542420 movaps xmmword ptr [rsp + 0x20], xmm10
140348ba2: 440f294c2430 movaps xmmword ptr [rsp + 0x30], xmm9
140348ba8: 440f29442440 movaps xmmword ptr [rsp + 0x40], xmm8
140348bae: 0f297c2450 movaps xmmword ptr [rsp + 0x50], xmm7
140348bb3: 0f29742460 movaps xmmword ptr [rsp + 0x60], xmm6
140348bb8: 488bb580040000 mov rsi, qword ptr [rbp + 0x480]
140348bbf: 48c70600000000 mov qword ptr [rsi], 0
140348bc6: 48c7461003000000 mov qword ptr [rsi + 0x10], 3
140348bce: 488d4e18 lea rcx, [rsi + 0x18]
140348bd2: 488d95d0000000 lea rdx, [rbp + 0xd0]
140348bd9: 41b8a0000000 mov r8d, 0xa0
140348bdf: e81cb05d02 call 0x142923c00 ; unknown
140348be4: 488d8eb8000000 lea rcx, [rsi + 0xb8]
140348beb: 488d55b0 lea rdx, [rbp - 0x50]
140348bef: 41b890000000 mov r8d, 0x90
140348bf5: e806b05d02 call 0x142923c00 ; unknown
140348bfa: 0f10051f746602 movups xmm0, xmmword ptr [rip + 0x266741f]
140348c01: 0f118648010000 movups xmmword ptr [rsi + 0x148], xmm0
140348c08: 0f100521746602 movups xmm0, xmmword ptr [rip + 0x2667421]
140348c0f: 0f118658010000 movups xmmword ptr [rsi + 0x158], xmm0
140348c16: 488b85d8040000 mov rax, qword ptr [rbp + 0x4d8]
140348c1d: 48898668010000 mov qword ptr [rsi + 0x168], rax
140348c24: 488b85e0040000 mov rax, qword ptr [rbp + 0x4e0]
140348c2b: 48898670010000 mov qword ptr [rsi + 0x170], rax
140348c32: c6867801000000 mov byte ptr [rsi + 0x178], 0
140348c39: 0f28742460 movaps xmm6, xmmword ptr [rsp + 0x60]
140348c3e: 0f287c2450 movaps xmm7, xmmword ptr [rsp + 0x50]
140348c43: 440f28442440 movaps xmm8, xmmword ptr [rsp + 0x40]
140348c49: 440f284c2430 movaps xmm9, xmmword ptr [rsp + 0x30]
140348c4f: 440f28542420 movaps xmm10, xmmword ptr [rsp + 0x20]
140348c55: 4883c478 add rsp, 0x78
140348c59: 5b pop rbx
140348c5a: 5f pop rdi
140348c5b: 5e pop rsi
140348c5c: 415c pop r12
140348c5e: 415d pop r13
140348c60: 415e pop r14
140348c62: 415f pop r15
140348c64: 5d pop rbp
140348c65: c3 ret 
140348c66: 662e0f1f840000000000 nop word ptr cs:[rax + rax]
140348c70: 4889542410 mov qword ptr [rsp + 0x10], rdx
140348c75: 55 push rbp
140348c76: 4157 push r15
140348c78: 4156 push r14
140348c7a: 4155 push r13
140348c7c: 4154 push r12
140348c7e: 56 push rsi
140348c7f: 57 push rdi
140348c80: 53 push rbx
140348c81: 4883ec78 sub rsp, 0x78
140348c85: 488daa80000000 lea rbp, [rdx + 0x80]
140348c8c: 440f29542420 movaps xmmword ptr [rsp + 0x20], xmm10
140348c92: 440f294c2430 movaps xmmword ptr [rsp + 0x30], xmm9
140348c98: 440f29442440 movaps xmmword ptr [rsp + 0x40], xmm8
140348c9e: 0f297c2450 movaps xmmword ptr [rsp + 0x50], xmm7
140348ca3: 0f29742460 movaps xmmword ptr [rsp + 0x60], xmm6
140348ca8: 488d4db0 lea rcx, [rbp - 0x50]
140348cac: e8bf472900 call 0x1405dd470 ; core::ptr::drop_in_place<country_core::systems::collision::world::ColliderRegistry<country_core::resources::stairs::stairs_state::StairGraphEdge> >
140348cb1: 0f28742460 movaps xmm6, xmmword ptr [rsp + 0x60]
140348cb6: 0f287c2450 movaps xmm7, xmmword ptr [rsp + 0x50]
140348cbb: 440f28442440 movaps xmm8, xmmword ptr [rsp + 0x40]
140348cc1: 440f284c2430 movaps xmm9, xmmword ptr [rsp + 0x30]
140348cc7: 440f28542420 movaps xmm10, xmmword ptr [rsp + 0x20]
140348ccd: 4883c478 add rsp, 0x78
140348cd1: 5b pop rbx
140348cd2: 5f pop rdi
140348cd3: 5e pop rsi
140348cd4: 415c pop r12
140348cd6: 415d pop r13
140348cd8: 415e pop r14
140348cda: 415f pop r15
140348cdc: 5d pop rbp
140348cdd: c3 ret 
140348cde: 6690 nop 
140348ce0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140348ce5: 55 push rbp
140348ce6: 4157 push r15
140348ce8: 4156 push r14
140348cea: 4155 push r13
140348cec: 4154 push r12
140348cee: 56 push rsi
140348cef: 57 push rdi
140348cf0: 53 push rbx
140348cf1: 4883ec78 sub rsp, 0x78
140348cf5: 488daa80000000 lea rbp, [rdx + 0x80]
140348cfc: 440f29542420 movaps xmmword ptr [rsp + 0x20], xmm10
140348d02: 440f294c2430 movaps xmmword ptr [rsp + 0x30], xmm9
140348d08: 440f29442440 movaps xmmword ptr [rsp + 0x40], xmm8
140348d0e: 0f297c2450 movaps xmmword ptr [rsp + 0x50], xmm7
140348d13: 0f29742460 movaps xmmword ptr [rsp + 0x60], xmm6
140348d18: 488d4d40 lea rcx, [rbp + 0x40]
140348d1c: e84f472900 call 0x1405dd470 ; core::ptr::drop_in_place<country_core::systems::collision::world::ColliderRegistry<country_core::resources::stairs::stairs_state::StairGraphEdge> >
140348d21: 0f28742460 movaps xmm6, xmmword ptr [rsp + 0x60]
140348d26: 0f287c2450 movaps xmm7, xmmword ptr [rsp + 0x50]
140348d2b: 440f28442440 movaps xmm8, xmmword ptr [rsp + 0x40]
140348d31: 440f284c2430 movaps xmm9, xmmword ptr [rsp + 0x30]
140348d37: 440f28542420 movaps xmm10, xmmword ptr [rsp + 0x20]
140348d3d: 4883c478 add rsp, 0x78
140348d41: 5b pop rbx
140348d42: 5f pop rdi
140348d43: 5e pop rsi
140348d44: 415c pop r12
140348d46: 415d pop r13
140348d48: 415e pop r14
140348d4a: 415f pop r15
140348d4c: 5d pop rbp
140348d4d: c3 ret 
140348d4e: 6690 nop 
140348d50: 4889542410 mov qword ptr [rsp + 0x10], rdx
140348d55: 55 push rbp
140348d56: 4157 push r15
140348d58: 4156 push r14
140348d5a: 4155 push r13
140348d5c: 4154 push r12
140348d5e: 56 push rsi
140348d5f: 57 push rdi
140348d60: 53 push rbx
140348d61: 4883ec78 sub rsp, 0x78
140348d65: 488daa80000000 lea rbp, [rdx + 0x80]
140348d6c: 440f29542420 movaps xmmword ptr [rsp + 0x20], xmm10
140348d72: 440f294c2430 movaps xmmword ptr [rsp + 0x30], xmm9
140348d78: 440f29442440 movaps xmmword ptr [rsp + 0x40], xmm8
140348d7e: 0f297c2450 movaps xmmword ptr [rsp + 0x50], xmm7
140348d83: 0f29742460 movaps xmmword ptr [rsp + 0x60], xmm6
140348d88: 488d8d60030000 lea rcx, [rbp + 0x360]
140348d8f: e8dc462900 call 0x1405dd470 ; core::ptr::drop_in_place<country_core::systems::collision::world::ColliderRegistry<country_core::resources::stairs::stairs_state::StairGraphEdge> >
140348d94: 0f28742460 movaps xmm6, xmmword ptr [rsp + 0x60]
140348d99: 0f287c2450 movaps xmm7, xmmword ptr [rsp + 0x50]
140348d9e: 440f28442440 movaps xmm8, xmmword ptr [rsp + 0x40]
140348da4: 440f284c2430 movaps xmm9, xmmword ptr [rsp + 0x30]
140348daa: 440f28542420 movaps xmm10, xmmword ptr [rsp + 0x20]
140348db0: 4883c478 add rsp, 0x78
140348db4: 5b pop rbx
140348db5: 5f pop rdi
140348db6: 5e pop rsi
140348db7: 415c pop r12
140348db9: 415d pop r13
140348dbb: 415e pop r14
140348dbd: 415f pop r15
140348dbf: 5d pop rbp
140348dc0: c3 ret 
140348dc1: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140348dd0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140348dd5: 55 push rbp
140348dd6: 4157 push r15
140348dd8: 4156 push r14
140348dda: 4155 push r13
140348ddc: 4154 push r12
140348dde: 56 push rsi
140348ddf: 57 push rdi
140348de0: 53 push rbx
140348de1: 4883ec78 sub rsp, 0x78
140348de5: 488daa80000000 lea rbp, [rdx + 0x80]
140348dec: 440f29542420 movaps xmmword ptr [rsp + 0x20], xmm10
140348df2: 440f294c2430 movaps xmmword ptr [rsp + 0x30], xmm9
140348df8: 440f29442440 movaps xmmword ptr [rsp + 0x40], xmm8
140348dfe: 0f297c2450 movaps xmmword ptr [rsp + 0x50], xmm7
140348e03: 0f29742460 movaps xmmword ptr [rsp + 0x60], xmm6
140348e08: 488d95d0000000 lea rdx, [rbp + 0xd0]
140348e0f: 41b8b0000000 mov r8d, 0xb0
140348e15: 488bb590040000 mov rsi, qword ptr [rbp + 0x490]
140348e1c: 4889f1 mov rcx, rsi
140348e1f: e8dcad5d02 call 0x142923c00 ; unknown
140348e24: 488b85d8040000 mov rax, qword ptr [rbp + 0x4d8]
140348e2b: 488986b0000000 mov qword ptr [rsi + 0xb0], rax
140348e32: 488b85e0040000 mov rax, qword ptr [rbp + 0x4e0]
140348e39: 488986b8000000 mov qword ptr [rsi + 0xb8], rax
140348e40: 0f28742460 movaps xmm6, xmmword ptr [rsp + 0x60]
140348e45: 0f287c2450 movaps xmm7, xmmword ptr [rsp + 0x50]
140348e4a: 440f28442440 movaps xmm8, xmmword ptr [rsp + 0x40]
140348e50: 440f284c2430 movaps xmm9, xmmword ptr [rsp + 0x30]
140348e56: 440f28542420 movaps xmm10, xmmword ptr [rsp + 0x20]
140348e5c: 4883c478 add rsp, 0x78
140348e60: 5b pop rbx
140348e61: 5f pop rdi
140348e62: 5e pop rsi
140348e63: 415c pop r12
140348e65: 415d pop r13
140348e67: 415e pop r14
140348e69: 415f pop r15
140348e6b: 5d pop rbp
140348e6c: c3 ret 
140348e6d: 0f1f00 nop dword ptr [rax]
140348e70: 4889542410 mov qword ptr [rsp + 0x10], rdx
140348e75: 55 push rbp
140348e76: 4157 push r15
140348e78: 4156 push r14
140348e7a: 4155 push r13
140348e7c: 4154 push r12
140348e7e: 56 push rsi
140348e7f: 57 push rdi
140348e80: 53 push rbx
140348e81: 4883ec78 sub rsp, 0x78
140348e85: 488daa80000000 lea rbp, [rdx + 0x80]
140348e8c: 440f29542420 movaps xmmword ptr [rsp + 0x20], xmm10
140348e92: 440f294c2430 movaps xmmword ptr [rsp + 0x30], xmm9
140348e98: 440f29442440 movaps xmmword ptr [rsp + 0x40], xmm8
140348e9e: 0f297c2450 movaps xmmword ptr [rsp + 0x50], xmm7
140348ea3: 0f29742460 movaps xmmword ptr [rsp + 0x60], xmm6
140348ea8: 0f288540010000 movaps xmm0, xmmword ptr [rbp + 0x140]
140348eaf: 488b8d08040000 mov rcx, qword ptr [rbp + 0x408]
140348eb6: 0f114170 movups xmmword ptr [rcx + 0x70], xmm0
140348eba: 0f288530010000 movaps xmm0, xmmword ptr [rbp + 0x130]
140348ec1: 0f114160 movups xmmword ptr [rcx + 0x60], xmm0
140348ec5: 0f288520010000 movaps xmm0, xmmword ptr [rbp + 0x120]
140348ecc: 0f114150 movups xmmword ptr [rcx + 0x50], xmm0
140348ed0: 0f288510010000 movaps xmm0, xmmword ptr [rbp + 0x110]
140348ed7: 0f114140 movups xmmword ptr [rcx + 0x40], xmm0
140348edb: 0f2885d0000000 movaps xmm0, xmmword ptr [rbp + 0xd0]
140348ee2: 0f288de0000000 movaps xmm1, xmmword ptr [rbp + 0xe0]
140348ee9: 0f2895f0000000 movaps xmm2, xmmword ptr [rbp + 0xf0]
140348ef0: 0f289d00010000 movaps xmm3, xmmword ptr [rbp + 0x100]
140348ef7: 0f115930 movups xmmword ptr [rcx + 0x30], xmm3
140348efb: 0f115120 movups xmmword ptr [rcx + 0x20], xmm2
140348eff: 0f114910 movups xmmword ptr [rcx + 0x10], xmm1
140348f03: 0f1101 movups xmmword ptr [rcx], xmm0
140348f06: 488b8500040000 mov rax, qword ptr [rbp + 0x400]
140348f0d: 48898180000000 mov qword ptr [rcx + 0x80], rax
140348f14: 488b85c8040000 mov rax, qword ptr [rbp + 0x4c8]
140348f1b: 48898188000000 mov qword ptr [rcx + 0x88], rax
140348f22: 48c7819000000000000000 mov qword ptr [rcx + 0x90], 0
140348f2d: 48c7812001000000000000 mov qword ptr [rcx + 0x120], 0
140348f38: c681b001000000 mov byte ptr [rcx + 0x1b0], 0
140348f3f: 0f28742460 movaps xmm6, xmmword ptr [rsp + 0x60]
140348f44: 0f287c2450 movaps xmm7, xmmword ptr [rsp + 0x50]
140348f49: 440f28442440 movaps xmm8, xmmword ptr [rsp + 0x40]
140348f4f: 440f284c2430 movaps xmm9, xmmword ptr [rsp + 0x30]
140348f55: 440f28542420 movaps xmm10, xmmword ptr [rsp + 0x20]
140348f5b: 4883c478 add rsp, 0x78
140348f5f: 5b pop rbx
140348f60: 5f pop rdi
140348f61: 5e pop rsi
140348f62: 415c pop r12
140348f64: 415d pop r13
140348f66: 415e pop r14
140348f68: 415f pop r15
140348f6a: 5d pop rbp
140348f6b: c3 ret 
140348f6c: 0f1f4000 nop dword ptr [rax]
140348f70: 4889542410 mov qword ptr [rsp + 0x10], rdx
140348f75: 55 push rbp
140348f76: 4157 push r15
140348f78: 4156 push r14
140348f7a: 4155 push r13
140348f7c: 4154 push r12
140348f7e: 56 push rsi
140348f7f: 57 push rdi
140348f80: 53 push rbx
140348f81: 4883ec78 sub rsp, 0x78
140348f85: 488daa80000000 lea rbp, [rdx + 0x80]
140348f8c: 440f29542420 movaps xmmword ptr [rsp + 0x20], xmm10
140348f92: 440f294c2430 movaps xmmword ptr [rsp + 0x30], xmm9
140348f98: 440f29442440 movaps xmmword ptr [rsp + 0x40], xmm8
140348f9e: 0f297c2450 movaps xmmword ptr [rsp + 0x50], xmm7
140348fa3: 0f29742460 movaps xmmword ptr [rsp + 0x60], xmm6
140348fa8: 0f1005b19e6702 movups xmm0, xmmword ptr [rip + 0x2679eb1]
140348faf: 488b8de0040000 mov rcx, qword ptr [rbp + 0x4e0]
140348fb6: 0f114110 movups xmmword ptr [rcx + 0x10], xmm0
140348fba: 0f10058f9e6702 movups xmm0, xmmword ptr [rip + 0x2679e8f]
140348fc1: 0f1101 movups xmmword ptr [rcx], xmm0
140348fc4: 488b85d0040000 mov rax, qword ptr [rbp + 0x4d0]
140348fcb: 48894120 mov qword ptr [rcx + 0x20], rax
140348fcf: 488b85c8040000 mov rax, qword ptr [rbp + 0x4c8]
140348fd6: 48894128 mov qword ptr [rcx + 0x28], rax
140348fda: 0f28742460 movaps xmm6, xmmword ptr [rsp + 0x60]
140348fdf: 0f287c2450 movaps xmm7, xmmword ptr [rsp + 0x50]
140348fe4: 440f28442440 movaps xmm8, xmmword ptr [rsp + 0x40]
140348fea: 440f284c2430 movaps xmm9, xmmword ptr [rsp + 0x30]
140348ff0: 440f28542420 movaps xmm10, xmmword ptr [rsp + 0x20]
140348ff6: 4883c478 add rsp, 0x78
140348ffa: 5b pop rbx
140348ffb: 5f pop rdi
140348ffc: 5e pop rsi
140348ffd: 415c pop r12
140348fff: 415d pop r13
140349001: 415e pop r14
140349003: 415f pop r15
140349005: 5d pop rbp
140349006: c3 ret 
140349007: 660f1f840000000000 nop word ptr [rax + rax]
140349010: 4889542410 mov qword ptr [rsp + 0x10], rdx
140349015: 55 push rbp
140349016: 4157 push r15
140349018: 4156 push r14
14034901a: 4155 push r13
14034901c: 4154 push r12
14034901e: 56 push rsi
14034901f: 57 push rdi
140349020: 53 push rbx
140349021: 4883ec78 sub rsp, 0x78
140349025: 488daa80000000 lea rbp, [rdx + 0x80]
14034902c: 440f29542420 movaps xmmword ptr [rsp + 0x20], xmm10
140349032: 440f294c2430 movaps xmmword ptr [rsp + 0x30], xmm9
140349038: 440f29442440 movaps xmmword ptr [rsp + 0x40], xmm8
14034903e: 0f297c2450 movaps xmmword ptr [rsp + 0x50], xmm7
140349043: 0f29742460 movaps xmmword ptr [rsp + 0x60], xmm6
140349048: 0f1005119e6702 movups xmm0, xmmword ptr [rip + 0x2679e11]
14034904f: 488b8dd8040000 mov rcx, qword ptr [rbp + 0x4d8]
140349056: 0f114110 movups xmmword ptr [rcx + 0x10], xmm0
14034905a: 0f1005ef9d6702 movups xmm0, xmmword ptr [rip + 0x2679def]
140349061: 0f1101 movups xmmword ptr [rcx], xmm0
140349064: 488b85d0040000 mov rax, qword ptr [rbp + 0x4d0]
14034906b: 48894120 mov qword ptr [rcx + 0x20], rax
14034906f: 488b85e0040000 mov rax, qword ptr [rbp + 0x4e0]
140349076: 48894128 mov qword ptr [rcx + 0x28], rax
14034907a: 0f28742460 movaps xmm6, xmmword ptr [rsp + 0x60]
14034907f: 0f287c2450 movaps xmm7, xmmword ptr [rsp + 0x50]
140349084: 440f28442440 movaps xmm8, xmmword ptr [rsp + 0x40]
14034908a: 440f284c2430 movaps xmm9, xmmword ptr [rsp + 0x30]
140349090: 440f28542420 movaps xmm10, xmmword ptr [rsp + 0x20]
140349096: 4883c478 add rsp, 0x78
14034909a: 5b pop rbx
14034909b: 5f pop rdi
14034909c: 5e pop rsi
14034909d: 415c pop r12
14034909f: 415d pop r13
1403490a1: 415e pop r14
1403490a3: 415f pop r15
1403490a5: 5d pop rbp
1403490a6: c3 ret 
1403490a7: 660f1f840000000000 nop word ptr [rax + rax]
1403490b0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1403490b5: 55 push rbp
1403490b6: 4157 push r15
1403490b8: 4156 push r14
1403490ba: 4155 push r13
1403490bc: 4154 push r12
1403490be: 56 push rsi
1403490bf: 57 push rdi
1403490c0: 53 push rbx
1403490c1: 4883ec78 sub rsp, 0x78
1403490c5: 488daa80000000 lea rbp, [rdx + 0x80]
1403490cc: 440f29542420 movaps xmmword ptr [rsp + 0x20], xmm10
1403490d2: 440f294c2430 movaps xmmword ptr [rsp + 0x30], xmm9
1403490d8: 440f29442440 movaps xmmword ptr [rsp + 0x40], xmm8
1403490de: 0f297c2450 movaps xmmword ptr [rsp + 0x50], xmm7
1403490e3: 0f29742460 movaps xmmword ptr [rsp + 0x60], xmm6
1403490e8: 488d95d0000000 lea rdx, [rbp + 0xd0]
1403490ef: 41b8e8010000 mov r8d, 0x1e8
1403490f5: 488b8d68040000 mov rcx, qword ptr [rbp + 0x468]
1403490fc: e8ffaa5d02 call 0x142923c00 ; unknown
140349101: 0f28742460 movaps xmm6, xmmword ptr [rsp + 0x60]
140349106: 0f287c2450 movaps xmm7, xmmword ptr [rsp + 0x50]
14034910b: 440f28442440 movaps xmm8, xmmword ptr [rsp + 0x40]
140349111: 440f284c2430 movaps xmm9, xmmword ptr [rsp + 0x30]
140349117: 440f28542420 movaps xmm10, xmmword ptr [rsp + 0x20]
14034911d: 4883c478 add rsp, 0x78
140349121: 5b pop rbx
140349122: 5f pop rdi
140349123: 5e pop rsi
140349124: 415c pop r12
140349126: 415d pop r13
140349128: 415e pop r14
14034912a: 415f pop r15
14034912c: 5d pop rbp
14034912d: c3 ret 
14034912e: 6690 nop 
140349130: 4889542410 mov qword ptr [rsp + 0x10], rdx
140349135: 55 push rbp
140349136: 4157 push r15
140349138: 4156 push r14
14034913a: 4155 push r13
14034913c: 4154 push r12
14034913e: 56 push rsi
14034913f: 57 push rdi
140349140: 53 push rbx
140349141: 4883ec78 sub rsp, 0x78
140349145: 488daa80000000 lea rbp, [rdx + 0x80]
14034914c: 440f29542420 movaps xmmword ptr [rsp + 0x20], xmm10
140349152: 440f294c2430 movaps xmmword ptr [rsp + 0x30], xmm9
140349158: 440f29442440 movaps xmmword ptr [rsp + 0x40], xmm8
14034915e: 0f297c2450 movaps xmmword ptr [rsp + 0x50], xmm7
140349163: 0f29742460 movaps xmmword ptr [rsp + 0x60], xmm6
140349168: 488b8d98040000 mov rcx, qword ptr [rbp + 0x498]
14034916f: 48c70100000000 mov qword ptr [rcx], 0
140349176: 48c7410810000000 mov qword ptr [rcx + 8], 0x10
14034917e: 48c7411000000000 mov qword ptr [rcx + 0x10], 0
140349186: 0f288560030000 movaps xmm0, xmmword ptr [rbp + 0x360]
14034918d: 0f288d70030000 movaps xmm1, xmmword ptr [rbp + 0x370]
140349194: 0f289580030000 movaps xmm2, xmmword ptr [rbp + 0x380]
14034919b: 0f114118 movups xmmword ptr [rcx + 0x18], xmm0
14034919f: 0f114928 movups xmmword ptr [rcx + 0x28], xmm1
1403491a3: 0f115138 movups xmmword ptr [rcx + 0x38], xmm2
1403491a7: 0f1005a29c6702 movups xmm0, xmmword ptr [rip + 0x2679ca2]
1403491ae: 0f114148 movups xmmword ptr [rcx + 0x48], xmm0
1403491b2: 0f1005a79c6702 movups xmm0, xmmword ptr [rip + 0x2679ca7]
1403491b9: 0f114158 movups xmmword ptr [rcx + 0x58], xmm0
1403491bd: 488b85d8040000 mov rax, qword ptr [rbp + 0x4d8]
1403491c4: 48894168 mov qword ptr [rcx + 0x68], rax
1403491c8: 488b85e0040000 mov rax, qword ptr [rbp + 0x4e0]
1403491cf: 48894170 mov qword ptr [rcx + 0x70], rax
1403491d3: 0f28742460 movaps xmm6, xmmword ptr [rsp + 0x60]
1403491d8: 0f287c2450 movaps xmm7, xmmword ptr [rsp + 0x50]
1403491dd: 440f28442440 movaps xmm8, xmmword ptr [rsp + 0x40]
1403491e3: 440f284c2430 movaps xmm9, xmmword ptr [rsp + 0x30]
1403491e9: 440f28542420 movaps xmm10, xmmword ptr [rsp + 0x20]
1403491ef: 4883c478 add rsp, 0x78
1403491f3: 5b pop rbx
1403491f4: 5f pop rdi
1403491f5: 5e pop rsi
1403491f6: 415c pop r12
1403491f8: 415d pop r13
1403491fa: 415e pop r14
1403491fc: 415f pop r15
1403491fe: 5d pop rbp
1403491ff: c3 ret 
140349200: 4889542410 mov qword ptr [rsp + 0x10], rdx
140349205: 55 push rbp
140349206: 4157 push r15
140349208: 4156 push r14
14034920a: 4155 push r13
14034920c: 4154 push r12
14034920e: 56 push rsi
14034920f: 57 push rdi
140349210: 53 push rbx
140349211: 4883ec78 sub rsp, 0x78
140349215: 488daa80000000 lea rbp, [rdx + 0x80]
14034921c: 440f29542420 movaps xmmword ptr [rsp + 0x20], xmm10
140349222: 440f294c2430 movaps xmmword ptr [rsp + 0x30], xmm9
140349228: 440f29442440 movaps xmmword ptr [rsp + 0x40], xmm8
14034922e: 0f297c2450 movaps xmmword ptr [rsp + 0x50], xmm7
140349233: 0f29742460 movaps xmmword ptr [rsp + 0x60], xmm6
140349238: 488d95d0000000 lea rdx, [rbp + 0xd0]
14034923f: 41b8b0000000 mov r8d, 0xb0
140349245: 488bb5a0040000 mov rsi, qword ptr [rbp + 0x4a0]
14034924c: 4889f1 mov rcx, rsi
14034924f: e8aca95d02 call 0x142923c00 ; unknown
140349254: 488b85d8040000 mov rax, qword ptr [rbp + 0x4d8]
14034925b: 488986b0000000 mov qword ptr [rsi + 0xb0], rax
140349262: 488b85e0040000 mov rax, qword ptr [rbp + 0x4e0]
140349269: 488986b8000000 mov qword ptr [rsi + 0xb8], rax
140349270: 0f28742460 movaps xmm6, xmmword ptr [rsp + 0x60]
140349275: 0f287c2450 movaps xmm7, xmmword ptr [rsp + 0x50]
14034927a: 440f28442440 movaps xmm8, xmmword ptr [rsp + 0x40]
140349280: 440f284c2430 movaps xmm9, xmmword ptr [rsp + 0x30]
140349286: 440f28542420 movaps xmm10, xmmword ptr [rsp + 0x20]
14034928c: 4883c478 add rsp, 0x78
140349290: 5b pop rbx
140349291: 5f pop rdi
140349292: 5e pop rsi
140349293: 415c pop r12
140349295: 415d pop r13
140349297: 415e pop r14
140349299: 415f pop r15
14034929b: 5d pop rbp
14034929c: c3 ret 
14034929d: 0f1f00 nop dword ptr [rax]
1403492a0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1403492a5: 55 push rbp
1403492a6: 4157 push r15
1403492a8: 4156 push r14
1403492aa: 4155 push r13
1403492ac: 4154 push r12
1403492ae: 56 push rsi
1403492af: 57 push rdi
1403492b0: 53 push rbx
1403492b1: 4883ec78 sub rsp, 0x78
1403492b5: 488daa80000000 lea rbp, [rdx + 0x80]
1403492bc: 440f29542420 movaps xmmword ptr [rsp + 0x20], xmm10
1403492c2: 440f294c2430 movaps xmmword ptr [rsp + 0x30], xmm9
1403492c8: 440f29442440 movaps xmmword ptr [rsp + 0x40], xmm8
1403492ce: 0f297c2450 movaps xmmword ptr [rsp + 0x50], xmm7
1403492d3: 0f29742460 movaps xmmword ptr [rsp + 0x60], xmm6
1403492d8: 0f1005819b6702 movups xmm0, xmmword ptr [rip + 0x2679b81]
1403492df: 488b8da8040000 mov rcx, qword ptr [rbp + 0x4a8]
1403492e6: 0f114110 movups xmmword ptr [rcx + 0x10], xmm0
1403492ea: 0f10055f9b6702 movups xmm0, xmmword ptr [rip + 0x2679b5f]
1403492f1: 0f1101 movups xmmword ptr [rcx], xmm0
1403492f4: 488b85d8040000 mov rax, qword ptr [rbp + 0x4d8]
1403492fb: 48894120 mov qword ptr [rcx + 0x20], rax
1403492ff: 488b85e0040000 mov rax, qword ptr [rbp + 0x4e0]
140349306: 48894128 mov qword ptr [rcx + 0x28], rax
14034930a: 0f28742460 movaps xmm6, xmmword ptr [rsp + 0x60]
14034930f: 0f287c2450 movaps xmm7, xmmword ptr [rsp + 0x50]
140349314: 440f28442440 movaps xmm8, xmmword ptr [rsp + 0x40]
14034931a: 440f284c2430 movaps xmm9, xmmword ptr [rsp + 0x30]
140349320: 440f28542420 movaps xmm10, xmmword ptr [rsp + 0x20]
140349326: 4883c478 add rsp, 0x78
14034932a: 5b pop rbx
14034932b: 5f pop rdi
14034932c: 5e pop rsi
14034932d: 415c pop r12
14034932f: 415d pop r13
140349331: 415e pop r14
140349333: 415f pop r15
140349335: 5d pop rbp
140349336: c3 ret 
140349337: 660f1f840000000000 nop word ptr [rax + rax]
140349340: 4889542410 mov qword ptr [rsp + 0x10], rdx
140349345: 55 push rbp
140349346: 4157 push r15
140349348: 4156 push r14
14034934a: 4155 push r13
14034934c: 4154 push r12
14034934e: 56 push rsi
14034934f: 57 push rdi
140349350: 53 push rbx
140349351: 4883ec78 sub rsp, 0x78
140349355: 488daa80000000 lea rbp, [rdx + 0x80]
14034935c: 440f29542420 movaps xmmword ptr [rsp + 0x20], xmm10
140349362: 440f294c2430 movaps xmmword ptr [rsp + 0x30], xmm9
140349368: 440f29442440 movaps xmmword ptr [rsp + 0x40], xmm8
14034936e: 0f297c2450 movaps xmmword ptr [rsp + 0x50], xmm7
140349373: 0f29742460 movaps xmmword ptr [rsp + 0x60], xmm6
140349378: 488bb578040000 mov rsi, qword ptr [rbp + 0x478]
14034937f: 488d4e30 lea rcx, [rsi + 0x30]
140349383: 488d5650 lea rdx, [rsi + 0x50]
140349387: 41b808000000 mov r8d, 8
14034938d: 41b910000000 mov r9d, 0x10
140349393: e86830d0ff call 0x14004c400 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<core::any::TypeId,bevy_ecs::bundle::BundleId>,allocator_api2::stable::alloc::global::Global> | hashbrown::raw::RawTableInner::drop_inner_table<tuple$<array$<u8,2>,tu
140349398: 0f108520010000 movups xmm0, xmmword ptr [rbp + 0x120]
14034939f: 0f114650 movups xmmword ptr [rsi + 0x50], xmm0
1403493a3: 0f108510010000 movups xmm0, xmmword ptr [rbp + 0x110]
1403493aa: 0f114640 movups xmmword ptr [rsi + 0x40], xmm0
1403493ae: 0f1085d0000000 movups xmm0, xmmword ptr [rbp + 0xd0]
1403493b5: 0f108de0000000 movups xmm1, xmmword ptr [rbp + 0xe0]
1403493bc: 0f1095f0000000 movups xmm2, xmmword ptr [rbp + 0xf0]
1403493c3: 0f109d00010000 movups xmm3, xmmword ptr [rbp + 0x100]
1403493ca: 0f115e30 movups xmmword ptr [rsi + 0x30], xmm3
1403493ce: 0f115620 movups xmmword ptr [rsi + 0x20], xmm2
1403493d2: 0f114e10 movups xmmword ptr [rsi + 0x10], xmm1
1403493d6: 0f1106 movups xmmword ptr [rsi], xmm0
1403493d9: 0f28742460 movaps xmm6, xmmword ptr [rsp + 0x60]
1403493de: 0f287c2450 movaps xmm7, xmmword ptr [rsp + 0x50]
1403493e3: 440f28442440 movaps xmm8, xmmword ptr [rsp + 0x40]
1403493e9: 440f284c2430 movaps xmm9, xmmword ptr [rsp + 0x30]
1403493ef: 440f28542420 movaps xmm10, xmmword ptr [rsp + 0x20]
1403493f5: 4883c478 add rsp, 0x78
1403493f9: 5b pop rbx
1403493fa: 5f pop rdi
1403493fb: 5e pop rsi
1403493fc: 415c pop r12
1403493fe: 415d pop r13
140349400: 415e pop r14
140349402: 415f pop r15
140349404: 5d pop rbp
140349405: c3 ret 
140349406: 662e0f1f840000000000 nop word ptr cs:[rax + rax]
140349410: 4889542410 mov qword ptr [rsp + 0x10], rdx
140349415: 55 push rbp
140349416: 4157 push r15
140349418: 4156 push r14
14034941a: 4155 push r13
14034941c: 4154 push r12
14034941e: 56 push rsi
14034941f: 57 push rdi
140349420: 53 push rbx
140349421: 4883ec78 sub rsp, 0x78
140349425: 488daa80000000 lea rbp, [rdx + 0x80]
14034942c: 440f29542420 movaps xmmword ptr [rsp + 0x20], xmm10
140349432: 440f294c2430 movaps xmmword ptr [rsp + 0x30], xmm9
140349438: 440f29442440 movaps xmmword ptr [rsp + 0x40], xmm8
14034943e: 0f297c2450 movaps xmmword ptr [rsp + 0x50], xmm7
140349443: 0f29742460 movaps xmmword ptr [rsp + 0x60], xmm6
140349448: 488bb588040000 mov rsi, qword ptr [rbp + 0x488]
14034944f: 488d4e30 lea rcx, [rsi + 0x30]
140349453: 488d5650 lea rdx, [rsi + 0x50]
140349457: 41b808000000 mov r8d, 8
14034945d: 41b910000000 mov r9d, 0x10
140349463: e8982fd0ff call 0x14004c400 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<core::any::TypeId,bevy_ecs::bundle::BundleId>,allocator_api2::stable::alloc::global::Global> | hashbrown::raw::RawTableInner::drop_inner_table<tuple$<array$<u8,2>,tu
140349468: 0f108520010000 movups xmm0, xmmword ptr [rbp + 0x120]
14034946f: 0f114650 movups xmmword ptr [rsi + 0x50], xmm0
140349473: 0f108510010000 movups xmm0, xmmword ptr [rbp + 0x110]
14034947a: 0f114640 movups xmmword ptr [rsi + 0x40], xmm0
14034947e: 0f1085d0000000 movups xmm0, xmmword ptr [rbp + 0xd0]
140349485: 0f108de0000000 movups xmm1, xmmword ptr [rbp + 0xe0]
14034948c: 0f1095f0000000 movups xmm2, xmmword ptr [rbp + 0xf0]
140349493: 0f109d00010000 movups xmm3, xmmword ptr [rbp + 0x100]
14034949a: 0f115e30 movups xmmword ptr [rsi + 0x30], xmm3
14034949e: 0f115620 movups xmmword ptr [rsi + 0x20], xmm2
1403494a2: 0f114e10 movups xmmword ptr [rsi + 0x10], xmm1
1403494a6: 0f1106 movups xmmword ptr [rsi], xmm0
1403494a9: 0f28742460 movaps xmm6, xmmword ptr [rsp + 0x60]
1403494ae: 0f287c2450 movaps xmm7, xmmword ptr [rsp + 0x50]
1403494b3: 440f28442440 movaps xmm8, xmmword ptr [rsp + 0x40]
1403494b9: 440f284c2430 movaps xmm9, xmmword ptr [rsp + 0x30]
1403494bf: 440f28542420 movaps xmm10, xmmword ptr [rsp + 0x20]
1403494c5: 4883c478 add rsp, 0x78
1403494c9: 5b pop rbx
1403494ca: 5f pop rdi
1403494cb: 5e pop rsi
1403494cc: 415c pop r12
1403494ce: 415d pop r13
1403494d0: 415e pop r14
1403494d2: 415f pop r15
1403494d4: 5d pop rbp
1403494d5: c3 ret 
1403494d6: 662e0f1f840000000000 nop word ptr cs:[rax + rax]
1403494e0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1403494e5: 55 push rbp
1403494e6: 4157 push r15
1403494e8: 4156 push r14
1403494ea: 4155 push r13
1403494ec: 4154 push r12
1403494ee: 56 push rsi
1403494ef: 57 push rdi
1403494f0: 53 push rbx
1403494f1: 4883ec78 sub rsp, 0x78
1403494f5: 488daa80000000 lea rbp, [rdx + 0x80]
1403494fc: 440f29542420 movaps xmmword ptr [rsp + 0x20], xmm10
140349502: 440f294c2430 movaps xmmword ptr [rsp + 0x30], xmm9
140349508: 440f29442440 movaps xmmword ptr [rsp + 0x40], xmm8
14034950e: 0f297c2450 movaps xmmword ptr [rsp + 0x50], xmm7
140349513: 0f29742460 movaps xmmword ptr [rsp + 0x60], xmm6
140349518: 0f100541996702 movups xmm0, xmmword ptr [rip + 0x2679941]
14034951f: 488b8db0040000 mov rcx, qword ptr [rbp + 0x4b0]
140349526: 0f114110 movups xmmword ptr [rcx + 0x10], xmm0
14034952a: 0f10051f996702 movups xmm0, xmmword ptr [rip + 0x267991f]
140349531: 0f1101 movups xmmword ptr [rcx], xmm0
140349534: 488b85d8040000 mov rax, qword ptr [rbp + 0x4d8]
14034953b: 48894120 mov qword ptr [rcx + 0x20], rax
14034953f: 488b85e0040000 mov rax, qword ptr [rbp + 0x4e0]
140349546: 48894128 mov qword ptr [rcx + 0x28], rax
14034954a: c6413000 mov byte ptr [rcx + 0x30], 0
14034954e: 0f28742460 movaps xmm6, xmmword ptr [rsp + 0x60]
140349553: 0f287c2450 movaps xmm7, xmmword ptr [rsp + 0x50]
140349558: 440f28442440 movaps xmm8, xmmword ptr [rsp + 0x40]
14034955e: 440f284c2430 movaps xmm9, xmmword ptr [rsp + 0x30]
140349564: 440f28542420 movaps xmm10, xmmword ptr [rsp + 0x20]
14034956a: 4883c478 add rsp, 0x78
14034956e: 5b pop rbx
14034956f: 5f pop rdi
140349570: 5e pop rsi
140349571: 415c pop r12
140349573: 415d pop r13
140349575: 415e pop r14
140349577: 415f pop r15
140349579: 5d pop rbp
14034957a: c3 ret 
