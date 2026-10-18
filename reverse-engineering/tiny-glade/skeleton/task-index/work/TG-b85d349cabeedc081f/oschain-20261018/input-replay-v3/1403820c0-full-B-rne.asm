00000001403820c0: 55                               push rbp
00000001403820c1: 4157                             push r15
00000001403820c3: 4156                             push r14
00000001403820c5: 4155                             push r13
00000001403820c7: 4154                             push r12
00000001403820c9: 56                               push rsi
00000001403820ca: 57                               push rdi
00000001403820cb: 53                               push rbx
00000001403820cc: 4881ecb8020000                   sub rsp, 0x2b8
00000001403820d3: 488dac2480000000                 lea rbp, [rsp + 0x80]
00000001403820db: 440f29bd20020000                 movaps xmmword ptr [rbp + 0x220], xmm15
00000001403820e3: 440f29b510020000                 movaps xmmword ptr [rbp + 0x210], xmm14
00000001403820eb: 440f29ad00020000                 movaps xmmword ptr [rbp + 0x200], xmm13
00000001403820f3: 440f29a5f0010000                 movaps xmmword ptr [rbp + 0x1f0], xmm12
00000001403820fb: 440f299de0010000                 movaps xmmword ptr [rbp + 0x1e0], xmm11
0000000140382103: 440f2995d0010000                 movaps xmmword ptr [rbp + 0x1d0], xmm10
000000014038210b: 440f298dc0010000                 movaps xmmword ptr [rbp + 0x1c0], xmm9
0000000140382113: 440f2985b0010000                 movaps xmmword ptr [rbp + 0x1b0], xmm8
000000014038211b: 0f29bda0010000                   movaps xmmword ptr [rbp + 0x1a0], xmm7
0000000140382122: 0f29b590010000                   movaps xmmword ptr [rbp + 0x190], xmm6
0000000140382129: 48c78588010000feffffff           mov qword ptr [rbp + 0x188], 0xfffffffffffffffe
0000000140382134: 4989d4                           mov r12, rdx
0000000140382137: 4889ce                           mov rsi, rcx
000000014038213a: 4c8b7a40                         mov r15, qword ptr [rdx + 0x40]
000000014038213e: 4c8b6a48                         mov r13, qword ptr [rdx + 0x48]
0000000140382142: 488d7a10                         lea rdi, [rdx + 0x10]
0000000140382146: 488d8d30010000                   lea rcx, [rbp + 0x130]
000000014038214d: 4889fa                           mov rdx, rdi
0000000140382150: e8eb7f8400                       call 0x140bca140 ; ron::parse::Parser::skip_ws
0000000140382155: 80bd3001000032                   cmp byte ptr [rbp + 0x130], 0x32
000000014038215c: 0f85b6050000                     jne 0x140382718
0000000140382162: 410fb64c2450                     movzx ecx, byte ptr [r12 + 0x50]
0000000140382168: 84c9                             test cl, cl
000000014038216a: 751b                             jne 0x140382187
000000014038216c: 4889f9                           mov rcx, rdi
000000014038216f: ba28000000                       mov edx, 0x28
0000000140382174: e837208400                       call 0x140bc41b0 ; ron::parse::Parser::consume_char
0000000140382179: 84c0                             test al, al
000000014038217b: 0f84d8050000                     je 0x140382759
0000000140382181: 410fb64c2450                     movzx ecx, byte ptr [r12 + 0x50]
0000000140382187: 41c644245000                     mov byte ptr [r12 + 0x50], 0
000000014038218d: 41833c2401                       cmp dword ptr [r12], 1
0000000140382192: 7516                             jne 0x1403821aa
0000000140382194: 498b442408                       mov rax, qword ptr [r12 + 8]
0000000140382199: 4885c0                           test rax, rax
000000014038219c: 0f84e5050000                     je 0x140382787
00000001403821a2: 48ffc8                           dec rax
00000001403821a5: 4989442408                       mov qword ptr [r12 + 8], rax
00000001403821aa: 4889b5c0000000                   mov qword ptr [rbp + 0xc0], rsi
00000001403821b1: 888d87010000                     mov byte ptr [rbp + 0x187], cl
00000001403821b7: 4889bdc8000000                   mov qword ptr [rbp + 0xc8], rdi
00000001403821be: 4c89adb8000000                   mov qword ptr [rbp + 0xb8], r13
00000001403821c5: 4c89bd18010000                   mov qword ptr [rbp + 0x118], r15
00000001403821cc: 4c89a5d8000000                   mov qword ptr [rbp + 0xd8], r12
00000001403821d3: 66c785e00000000301               mov word ptr [rbp + 0xe0], 0x103
00000001403821dc: 49be0000000000000080             movabs r14, 0x8000000000000000
00000001403821e6: 48c785d000000000000000           mov qword ptr [rbp + 0xd0], 0
00000001403821f1: 6666666666662e0f1f840000000000   nop word ptr cs:[rax + rax]
0000000140382200: 4c89b500010000                   mov qword ptr [rbp + 0x100], r14
0000000140382207: 488d8d30010000                   lea rcx, [rbp + 0x130]
000000014038220e: 488d95d8000000                   lea rdx, [rbp + 0xd8]
0000000140382215: e836b78400                       call 0x140bcd950 ; ron::de::CommaSeparated::has_element
000000014038221a: 0fb6bd30010000                   movzx edi, byte ptr [rbp + 0x130]
0000000140382221: 440fb6ad31010000                 movzx r13d, byte ptr [rbp + 0x131]
0000000140382229: 4080ff32                         cmp dil, 0x32
000000014038222d: 0f85b8050000                     jne 0x1403827eb
0000000140382233: 41f6c501                         test r13b, 1
0000000140382237: 0f8420050000                     je 0x14038275d
000000014038223d: 0fb68de0000000                   movzx ecx, byte ptr [rbp + 0xe0]
0000000140382244: 4c8badd8000000                   mov r13, qword ptr [rbp + 0xd8]
000000014038224b: 410fb64500                       movzx eax, byte ptr [r13]
0000000140382250: 83f901                           cmp ecx, 1
0000000140382253: 0f8407010000                     je 0x140382360
0000000140382259: 83f903                           cmp ecx, 3
000000014038225c: 0f85fe010000                     jne 0x140382460
0000000140382262: a801                             test al, 1
0000000140382264: 7414                             je 0x14038227a
0000000140382266: 498b4508                         mov rax, qword ptr [r13 + 8]
000000014038226a: 4885c0                           test rax, rax
000000014038226d: 0f84cf060000                     je 0x140382942
0000000140382273: 48ffc8                           dec rax
0000000140382276: 49894508                         mov qword ptr [r13 + 8], rax
000000014038227a: 4c89ad08010000                   mov qword ptr [rbp + 0x108], r13
0000000140382281: c6851001000000                   mov byte ptr [rbp + 0x110], 0
0000000140382288: 4c89b500010000                   mov qword ptr [rbp + 0x100], r14
000000014038228f: 488d8d30010000                   lea rcx, [rbp + 0x130]
0000000140382296: 488d9508010000                   lea rdx, [rbp + 0x108]
000000014038229d: e86e461200                       call 0x1404a6910 ; ron::de::id::impl$1::deserialize_identifier<country_core::meta_commander::_::impl$0::deserialize::impl$2::visit_enum::__FieldVisitor>
00000001403822a2: 0fb6bd30010000                   movzx edi, byte ptr [rbp + 0x130]
00000001403822a9: 4080ff32                         cmp dil, 0x32
00000001403822ad: 744d                             je 0x1403822fc
00000001403822af: 488d8d32010000                   lea rcx, [rbp + 0x132]
00000001403822b6: 0fb74104                         movzx eax, word ptr [rcx + 4]
00000001403822ba: 6689852c010000                   mov word ptr [rbp + 0x12c], ax
00000001403822c1: 8b01                             mov eax, dword ptr [rcx]
00000001403822c3: 898528010000                     mov dword ptr [rbp + 0x128], eax
00000001403822c9: 488b8538010000                   mov rax, qword ptr [rbp + 0x138]
00000001403822d0: 48894550                         mov qword ptr [rbp + 0x50], rax
00000001403822d4: f30f6f8540010000                 movdqu xmm0, xmmword ptr [rbp + 0x140]
00000001403822dc: 660f7f45b0                       movdqa xmmword ptr [rbp - 0x50], xmm0
00000001403822e1: 440f10a550010000                 movups xmm12, xmmword ptr [rbp + 0x150]
00000001403822e9: 440f10bd60010000                 movups xmm15, xmmword ptr [rbp + 0x160]
00000001403822f1: 488b8570010000                   mov rax, qword ptr [rbp + 0x170]
00000001403822f8: 48894568                         mov qword ptr [rbp + 0x68], rax
00000001403822fc: 488b85d8000000                   mov rax, qword ptr [rbp + 0xd8]
0000000140382303: 833801                           cmp dword ptr [rax], 1
0000000140382306: 7516                             jne 0x14038231e
0000000140382308: 488b4808                         mov rcx, qword ptr [rax + 8]
000000014038230c: 48ffc1                           inc rcx
000000014038230f: 48c7c2ffffffff                   mov rdx, 0xffffffffffffffff
0000000140382316: 480f44ca                         cmove rcx, rdx
000000014038231a: 48894808                         mov qword ptr [rax + 8], rcx
000000014038231e: 440fb6bd31010000                 movzx r15d, byte ptr [rbp + 0x131]
0000000140382326: 0fb7852c010000                   movzx eax, word ptr [rbp + 0x12c]
000000014038232d: 6689857c010000                   mov word ptr [rbp + 0x17c], ax
0000000140382334: 8b8528010000                     mov eax, dword ptr [rbp + 0x128]
000000014038233a: 898578010000                     mov dword ptr [rbp + 0x178], eax
0000000140382340: 488b7568                         mov rsi, qword ptr [rbp + 0x68]
0000000140382344: 488b5d50                         mov rbx, qword ptr [rbp + 0x50]
0000000140382348: 440f2855b0                       movaps xmm10, xmmword ptr [rbp - 0x50]
000000014038234d: 450f28dc                         movaps xmm11, xmm12
0000000140382351: 450f28f7                         movaps xmm14, xmm15
0000000140382355: e930020000                       jmp 0x14038258a ; 
000000014038235a: 660f1f440000                     nop word ptr [rax + rax]
0000000140382360: a801                             test al, 1
0000000140382362: 7414                             je 0x140382378
0000000140382364: 498b4508                         mov rax, qword ptr [r13 + 8]
0000000140382368: 4885c0                           test rax, rax
000000014038236b: 0f84d1050000                     je 0x140382942
0000000140382371: 48ffc8                           dec rax
0000000140382374: 49894508                         mov qword ptr [r13 + 8], rax
0000000140382378: 4c89ad08010000                   mov qword ptr [rbp + 0x108], r13
000000014038237f: c6851001000001                   mov byte ptr [rbp + 0x110], 1
0000000140382386: 4c89b500010000                   mov qword ptr [rbp + 0x100], r14
000000014038238d: 488d8d30010000                   lea rcx, [rbp + 0x130]
0000000140382394: 488d9508010000                   lea rdx, [rbp + 0x108]
000000014038239b: e870451200                       call 0x1404a6910 ; ron::de::id::impl$1::deserialize_identifier<country_core::meta_commander::_::impl$0::deserialize::impl$2::visit_enum::__FieldVisitor>
00000001403823a0: 0fb6bd30010000                   movzx edi, byte ptr [rbp + 0x130]
00000001403823a7: 4080ff32                         cmp dil, 0x32
00000001403823ab: 7447                             je 0x1403823f4
00000001403823ad: 488d8d32010000                   lea rcx, [rbp + 0x132]
00000001403823b4: 0fb74104                         movzx eax, word ptr [rcx + 4]
00000001403823b8: 6689852c010000                   mov word ptr [rbp + 0x12c], ax
00000001403823bf: 8b01                             mov eax, dword ptr [rcx]
00000001403823c1: 898528010000                     mov dword ptr [rbp + 0x128], eax
00000001403823c7: 488b8538010000                   mov rax, qword ptr [rbp + 0x138]
00000001403823ce: 48894558                         mov qword ptr [rbp + 0x58], rax
00000001403823d2: 440f108d40010000                 movups xmm9, xmmword ptr [rbp + 0x140]
00000001403823da: 440f10ad50010000                 movups xmm13, xmmword ptr [rbp + 0x150]
00000001403823e2: 0f10bd60010000                   movups xmm7, xmmword ptr [rbp + 0x160]
00000001403823e9: 488b8570010000                   mov rax, qword ptr [rbp + 0x170]
00000001403823f0: 48894570                         mov qword ptr [rbp + 0x70], rax
00000001403823f4: 488b85d8000000                   mov rax, qword ptr [rbp + 0xd8]
00000001403823fb: 833801                           cmp dword ptr [rax], 1
00000001403823fe: 7516                             jne 0x140382416
0000000140382400: 488b4808                         mov rcx, qword ptr [rax + 8]
0000000140382404: 48ffc1                           inc rcx
0000000140382407: 48c7c2ffffffff                   mov rdx, 0xffffffffffffffff
000000014038240e: 480f44ca                         cmove rcx, rdx
0000000140382412: 48894808                         mov qword ptr [rax + 8], rcx
0000000140382416: 440fb6bd31010000                 movzx r15d, byte ptr [rbp + 0x131]
000000014038241e: 0fb7852c010000                   movzx eax, word ptr [rbp + 0x12c]
0000000140382425: 6689857c010000                   mov word ptr [rbp + 0x17c], ax
000000014038242c: 8b8528010000                     mov eax, dword ptr [rbp + 0x128]
0000000140382432: 898578010000                     mov dword ptr [rbp + 0x178], eax
0000000140382438: 488b7570                         mov rsi, qword ptr [rbp + 0x70]
000000014038243c: 488b5d58                         mov rbx, qword ptr [rbp + 0x58]
0000000140382440: 450f28d1                         movaps xmm10, xmm9
0000000140382444: 450f28dd                         movaps xmm11, xmm13
0000000140382448: 440f28f7                         movaps xmm14, xmm7
000000014038244c: e939010000                       jmp 0x14038258a ; 
0000000140382451: 6666666666662e0f1f840000000000   nop word ptr cs:[rax + rax]
0000000140382460: a801                             test al, 1
0000000140382462: 7414                             je 0x140382478
0000000140382464: 498b4508                         mov rax, qword ptr [r13 + 8]
0000000140382468: 4885c0                           test rax, rax
000000014038246b: 0f84d1040000                     je 0x140382942
0000000140382471: 48ffc8                           dec rax
0000000140382474: 49894508                         mov qword ptr [r13 + 8], rax
0000000140382478: 498d5510                         lea rdx, [r13 + 0x10]
000000014038247c: 4c89b500010000                   mov qword ptr [rbp + 0x100], r14
0000000140382483: 488d8d30010000                   lea rcx, [rbp + 0x130]
000000014038248a: e8f1768400                       call 0x140bc9b80 ; ron::parse::Parser::identifier
000000014038248f: 0fb6bd30010000                   movzx edi, byte ptr [rbp + 0x130]
0000000140382496: 4080ff32                         cmp dil, 0x32
000000014038249a: 7549                             jne 0x1403824e5
000000014038249c: 488b8538010000                   mov rax, qword ptr [rbp + 0x138]
00000001403824a3: 488b8d40010000                   mov rcx, qword ptr [rbp + 0x140]
00000001403824aa: 49894540                         mov qword ptr [r13 + 0x40], rax
00000001403824ae: 49894d48                         mov qword ptr [r13 + 0x48], rcx
00000001403824b2: 4883f912                         cmp rcx, 0x12
00000001403824b6: 0f847b010000                     je 0x140382637
00000001403824bc: 4883f904                         cmp rcx, 4
00000001403824c0: 0f85ab010000                     jne 0x140382671
00000001403824c6: 8b00                             mov eax, dword ptr [rax]
00000001403824c8: 3b053abf5d02                     cmp eax, dword ptr [rip + 0x25dbf3a]
00000001403824ce: 0f859d010000                     jne 0x140382671
00000001403824d4: 4531ff                           xor r15d, r15d
00000001403824d7: 488b85d8000000                   mov rax, qword ptr [rbp + 0xd8]
00000001403824de: 833801                           cmp dword ptr [rax], 1
00000001403824e1: 7462                             je 0x140382545
00000001403824e3: eb76                             jmp 0x14038255b ; 
00000001403824e5: 440fb6bd31010000                 movzx r15d, byte ptr [rbp + 0x131]
00000001403824ed: 488d8d32010000                   lea rcx, [rbp + 0x132]
00000001403824f4: 8b01                             mov eax, dword ptr [rcx]
00000001403824f6: 898508010000                     mov dword ptr [rbp + 0x108], eax
00000001403824fc: 0fb74104                         movzx eax, word ptr [rcx + 4]
0000000140382500: 6689850c010000                   mov word ptr [rbp + 0x10c], ax
0000000140382507: 488b8538010000                   mov rax, qword ptr [rbp + 0x138]
000000014038250e: 48894560                         mov qword ptr [rbp + 0x60], rax
0000000140382512: f30f6f8540010000                 movdqu xmm0, xmmword ptr [rbp + 0x140]
000000014038251a: 660f7f45a0                       movdqa xmmword ptr [rbp - 0x60], xmm0
000000014038251f: 0f10b550010000                   movups xmm6, xmmword ptr [rbp + 0x150]
0000000140382526: 440f108560010000                 movups xmm8, xmmword ptr [rbp + 0x160]
000000014038252e: 488b8570010000                   mov rax, qword ptr [rbp + 0x170]
0000000140382535: 48894578                         mov qword ptr [rbp + 0x78], rax
0000000140382539: 488b85d8000000                   mov rax, qword ptr [rbp + 0xd8]
0000000140382540: 833801                           cmp dword ptr [rax], 1
0000000140382543: 7516                             jne 0x14038255b
0000000140382545: 488b4808                         mov rcx, qword ptr [rax + 8]
0000000140382549: 48ffc1                           inc rcx
000000014038254c: 48c7c2ffffffff                   mov rdx, 0xffffffffffffffff
0000000140382553: 480f44ca                         cmove rcx, rdx
0000000140382557: 48894808                         mov qword ptr [rax + 8], rcx
000000014038255b: 0fb7850c010000                   movzx eax, word ptr [rbp + 0x10c]
0000000140382562: 6689857c010000                   mov word ptr [rbp + 0x17c], ax
0000000140382569: 8b8508010000                     mov eax, dword ptr [rbp + 0x108]
000000014038256f: 898578010000                     mov dword ptr [rbp + 0x178], eax
0000000140382575: 488b7578                         mov rsi, qword ptr [rbp + 0x78]
0000000140382579: 488b5d60                         mov rbx, qword ptr [rbp + 0x60]
000000014038257d: 440f2855a0                       movaps xmm10, xmmword ptr [rbp - 0x60]
0000000140382582: 440f28de                         movaps xmm11, xmm6
0000000140382586: 450f28f0                         movaps xmm14, xmm8
000000014038258a: 4080ff32                         cmp dil, 0x32
000000014038258e: 0f85b1030000                     jne 0x140382945
0000000140382594: 4180ff02                         cmp r15b, 2
0000000140382598: 7530                             jne 0x1403825ca
000000014038259a: 4c89b500010000                   mov qword ptr [rbp + 0x100], r14
00000001403825a1: 488d8d30010000                   lea rcx, [rbp + 0x130]
00000001403825a8: 488d95d8000000                   lea rdx, [rbp + 0xd8]
00000001403825af: e8ac750000                       call 0x140389b60 ; ron::de::impl$6::next_value_seed<core::marker::PhantomData<serde::de::ignored_any::IgnoredAny> >
00000001403825b4: 0fb6bd30010000                   movzx edi, byte ptr [rbp + 0x130]
00000001403825bb: 4080ff32                         cmp dil, 0x32
00000001403825bf: 0f843bfcffff                     je 0x140382200
00000001403825c5: e92e030000                       jmp 0x1403828f8 ; 
00000001403825ca: 410fb6c7                         movzx eax, r15b
00000001403825ce: 85c0                             test eax, eax
00000001403825d0: 0f84b3000000                     je 0x140382689
00000001403825d6: 83f801                           cmp eax, 1
00000001403825d9: 0f857e010000                     jne 0x14038275d
00000001403825df: 4883bdd000000000                 cmp qword ptr [rbp + 0xd0], 0
00000001403825e7: 0f8558060000                     jne 0x140382c45
00000001403825ed: 4c89b500010000                   mov qword ptr [rbp + 0x100], r14
00000001403825f4: 488d8d30010000                   lea rcx, [rbp + 0x130]
00000001403825fb: 488d95d8000000                   lea rdx, [rbp + 0xd8]
0000000140382602: e8495c0000                       call 0x140388250 ; serde::de::MapAccess::next_value<ron::de::CommaSeparated,usize>
0000000140382607: 0fb6bd30010000                   movzx edi, byte ptr [rbp + 0x130]
000000014038260e: 4080ff32                         cmp dil, 0x32
0000000140382612: 0f85e0020000                     jne 0x1403828f8
0000000140382618: 488b8538010000                   mov rax, qword ptr [rbp + 0x138]
000000014038261f: 488985f8000000                   mov qword ptr [rbp + 0xf8], rax
0000000140382626: b801000000                       mov eax, 1
000000014038262b: 488985d0000000                   mov qword ptr [rbp + 0xd0], rax
0000000140382632: e9c9fbffff                       jmp 0x140382200 ; 
0000000140382637: f30f6f00                         movdqu xmm0, xmmword ptr [rax]
000000014038263b: f30f6f0dc9bd5d02                 movdqu xmm1, xmmword ptr [rip + 0x25dbdc9]
0000000140382643: 660f74c8                         pcmpeqb xmm1, xmm0
0000000140382647: 0fb74010                         movzx eax, word ptr [rax + 0x10]
000000014038264b: 0fb70dcabd5d02                   movzx ecx, word ptr [rip + 0x25dbdca]
0000000140382652: 660f6ec0                         movd xmm0, eax
0000000140382656: 660f6ed1                         movd xmm2, ecx
000000014038265a: 660f74d0                         pcmpeqb xmm2, xmm0
000000014038265e: 660fdbd1                         pand xmm2, xmm1
0000000140382662: 660fd7c2                         pmovmskb eax, xmm2
0000000140382666: 3dffff0000                       cmp eax, 0xffff
000000014038266b: 0f848f000000                     je 0x140382700
0000000140382671: 41b702                           mov r15b, 2
0000000140382674: 488b85d8000000                   mov rax, qword ptr [rbp + 0xd8]
000000014038267b: 833801                           cmp dword ptr [rax], 1
000000014038267e: 0f85d7feffff                     jne 0x14038255b
0000000140382684: e9bcfeffff                       jmp 0x140382545 ; 
0000000140382689: 4c89f0                           mov rax, r14
000000014038268c: 48f7d8                           neg rax
000000014038268f: 0f81c7050000                     jno 0x140382c5c
0000000140382695: 48b80000000000000080             movabs rax, 0x8000000000000000
000000014038269f: 48898500010000                   mov qword ptr [rbp + 0x100], rax
00000001403826a6: 488d8d30010000                   lea rcx, [rbp + 0x130]
00000001403826ad: 488d95d8000000                   lea rdx, [rbp + 0xd8]
00000001403826b4: e8b7640000                       call 0x140388b70 ; serde::de::MapAccess::next_value<ron::de::CommaSeparated,std::path::PathBuf>
00000001403826b9: 0fb6bd30010000                   movzx edi, byte ptr [rbp + 0x130]
00000001403826c0: 4080ff32                         cmp dil, 0x32
00000001403826c4: 0f852e050000                     jne 0x140382bf8
00000001403826ca: 4c8bb538010000                   mov r14, qword ptr [rbp + 0x138]
00000001403826d1: 488b8540010000                   mov rax, qword ptr [rbp + 0x140]
00000001403826d8: 48898520010000                   mov qword ptr [rbp + 0x120], rax
00000001403826df: 488b8548010000                   mov rax, qword ptr [rbp + 0x148]
00000001403826e6: 488985f0000000                   mov qword ptr [rbp + 0xf0], rax
00000001403826ed: 488b8550010000                   mov rax, qword ptr [rbp + 0x150]
00000001403826f4: 488985e8000000                   mov qword ptr [rbp + 0xe8], rax
00000001403826fb: e900fbffff                       jmp 0x140382200 ; 
0000000140382700: 41b701                           mov r15b, 1
0000000140382703: 488b85d8000000                   mov rax, qword ptr [rbp + 0xd8]
000000014038270a: 833801                           cmp dword ptr [rax], 1
000000014038270d: 0f8432feffff                     je 0x140382545
0000000140382713: e943feffff                       jmp 0x14038255b ; 
0000000140382718: 488b8570010000                   mov rax, qword ptr [rbp + 0x170]
000000014038271f: 48894640                         mov qword ptr [rsi + 0x40], rax
0000000140382723: f30f6f8530010000                 movdqu xmm0, xmmword ptr [rbp + 0x130]
000000014038272b: f30f6f8d40010000                 movdqu xmm1, xmmword ptr [rbp + 0x140]
0000000140382733: f30f6f9550010000                 movdqu xmm2, xmmword ptr [rbp + 0x150]
000000014038273b: 0f109d60010000                   movups xmm3, xmmword ptr [rbp + 0x160]
0000000140382742: 0f115e30                         movups xmmword ptr [rsi + 0x30], xmm3
0000000140382746: f30f7f5620                       movdqu xmmword ptr [rsi + 0x20], xmm2
000000014038274b: f30f7f4e10                       movdqu xmmword ptr [rsi + 0x10], xmm1
0000000140382750: f30f7f06                         movdqu xmmword ptr [rsi], xmm0
0000000140382754: e948030000                       jmp 0x140382aa1 ; 
0000000140382759: b316                             mov bl, 0x16
000000014038275b: eb38                             jmp 0x140382795 ; 
000000014038275d: 40b72c                           mov dil, 0x2c
0000000140382760: 4c89f0                           mov rax, r14
0000000140382763: 48f7d8                           neg rax
0000000140382766: 0f81c4000000                     jno 0x140382830
000000014038276c: 440f2815bc7b6402                 movaps xmm10, xmmword ptr [rip + 0x2647bbc]
0000000140382774: 488d1ddd806402                   lea rbx, [rip + 0x26480dd]
000000014038277b: 4c8bbd18010000                   mov r15, qword ptr [rbp + 0x118]
0000000140382782: e92f020000                       jmp 0x1403829b6 ; 
0000000140382787: b331                             mov bl, 0x31
0000000140382789: 80fb32                           cmp bl, 0x32
000000014038278c: 0f84e2020000                     je 0x140382a74
0000000140382792: 4889c2                           mov rdx, rax
0000000140382795: 885d08                           mov byte ptr [rbp + 8], bl
0000000140382798: 8b8508010000                     mov eax, dword ptr [rbp + 0x108]
000000014038279e: 8b8d0b010000                     mov ecx, dword ptr [rbp + 0x10b]
00000001403827a4: 894509                           mov dword ptr [rbp + 9], eax
00000001403827a7: 894d0c                           mov dword ptr [rbp + 0xc], ecx
00000001403827aa: 48897d10                         mov qword ptr [rbp + 0x10], rdi
00000001403827ae: 4c897518                         mov qword ptr [rbp + 0x18], r14
00000001403827b2: 4c896520                         mov qword ptr [rbp + 0x20], r12
00000001403827b6: 488b85f0000000                   mov rax, qword ptr [rbp + 0xf0]
00000001403827bd: 48894528                         mov qword ptr [rbp + 0x28], rax
00000001403827c1: 48895530                         mov qword ptr [rbp + 0x30], rdx
00000001403827c5: 488b85f8000000                   mov rax, qword ptr [rbp + 0xf8]
00000001403827cc: 48894538                         mov qword ptr [rbp + 0x38], rax
00000001403827d0: 0f117540                         movups xmmword ptr [rbp + 0x40], xmm6
00000001403827d4: 488d5508                         lea rdx, [rbp + 8]
00000001403827d8: 4889f1                           mov rcx, rsi
00000001403827db: 4d89f8                           mov r8, r15
00000001403827de: 4d89e9                           mov r9, r13
00000001403827e1: e80ab28400                       call 0x140bcd9f0 ; ron::de::struct_error_name
00000001403827e6: e9b6020000                       jmp 0x140382aa1 ; 
00000001403827eb: 0fb78536010000                   movzx eax, word ptr [rbp + 0x136]
00000001403827f2: 6689857c010000                   mov word ptr [rbp + 0x17c], ax
00000001403827f9: 8b8532010000                     mov eax, dword ptr [rbp + 0x132]
00000001403827ff: 898578010000                     mov dword ptr [rbp + 0x178], eax
0000000140382805: 488b9d38010000                   mov rbx, qword ptr [rbp + 0x138]
000000014038280c: 440f109540010000                 movups xmm10, xmmword ptr [rbp + 0x140]
0000000140382814: 440f109d50010000                 movups xmm11, xmmword ptr [rbp + 0x150]
000000014038281c: 440f10b560010000                 movups xmm14, xmmword ptr [rbp + 0x160]
0000000140382824: 488bb570010000                   mov rsi, qword ptr [rbp + 0x170]
000000014038282b: e918010000                       jmp 0x140382948 ; 
0000000140382830: f685d000000001                   test byte ptr [rbp + 0xd0], 1
0000000140382837: 4c8bbd18010000                   mov r15, qword ptr [rbp + 0x118]
000000014038283e: 0f8449010000                     je 0x14038298d
0000000140382844: 41f6042401                       test byte ptr [r12], 1
0000000140382849: 7418                             je 0x140382863
000000014038284b: 498b442408                       mov rax, qword ptr [r12 + 8]
0000000140382850: 48ffc0                           inc rax
0000000140382853: 48c7c1ffffffff                   mov rcx, 0xffffffffffffffff
000000014038285a: 480f45c8                         cmovne rcx, rax
000000014038285e: 49894c2408                       mov qword ptr [r12 + 8], rcx
0000000140382863: 48bf0800000000000080             movabs rdi, 0x8000000000000008
000000014038286d: 4889bd80000000                   mov qword ptr [rbp + 0x80], rdi
0000000140382874: 4c89b588000000                   mov qword ptr [rbp + 0x88], r14
000000014038287b: 4c8ba520010000                   mov r12, qword ptr [rbp + 0x120]
0000000140382882: 4c89a590000000                   mov qword ptr [rbp + 0x90], r12
0000000140382889: 488b85f0000000                   mov rax, qword ptr [rbp + 0xf0]
0000000140382890: 48898598000000                   mov qword ptr [rbp + 0x98], rax
0000000140382897: 488b85e8000000                   mov rax, qword ptr [rbp + 0xe8]
000000014038289e: 488985a0000000                   mov qword ptr [rbp + 0xa0], rax
00000001403828a5: 488b85f8000000                   mov rax, qword ptr [rbp + 0xf8]
00000001403828ac: 488985a8000000                   mov qword ptr [rbp + 0xa8], rax
00000001403828b3: 488d8d30010000                   lea rcx, [rbp + 0x130]
00000001403828ba: 488b95c8000000                   mov rdx, qword ptr [rbp + 0xc8]
00000001403828c1: e87a788400                       call 0x140bca140 ; ron::parse::Parser::skip_ws
00000001403828c6: 4c8badb8000000                   mov r13, qword ptr [rbp + 0xb8]
00000001403828cd: 488bb5c0000000                   mov rsi, qword ptr [rbp + 0xc0]
00000001403828d4: 0fb69d30010000                   movzx ebx, byte ptr [rbp + 0x130]
00000001403828db: 80fb32                           cmp bl, 0x32
00000001403828de: 0f8522020000                     jne 0x140382b06
00000001403828e4: b332                             mov bl, 0x32
00000001403828e6: f6858701000001                   test byte ptr [rbp + 0x187], 1
00000001403828ed: 0f8480020000                     je 0x140382b73
00000001403828f3: e9e6020000                       jmp 0x140382bde ; 
00000001403828f8: 440fb6ad31010000                 movzx r13d, byte ptr [rbp + 0x131]
0000000140382900: 8b8532010000                     mov eax, dword ptr [rbp + 0x132]
0000000140382906: 898580010000                     mov dword ptr [rbp + 0x180], eax
000000014038290c: 0fb78536010000                   movzx eax, word ptr [rbp + 0x136]
0000000140382913: 66898584010000                   mov word ptr [rbp + 0x184], ax
000000014038291a: 488b9d38010000                   mov rbx, qword ptr [rbp + 0x138]
0000000140382921: 440f109540010000                 movups xmm10, xmmword ptr [rbp + 0x140]
0000000140382929: 440f109d50010000                 movups xmm11, xmmword ptr [rbp + 0x150]
0000000140382931: 440f10b560010000                 movups xmm14, xmmword ptr [rbp + 0x160]
0000000140382939: 488bb570010000                   mov rsi, qword ptr [rbp + 0x170]
0000000140382940: eb20                             jmp 0x140382962 ; 
0000000140382942: 40b731                           mov dil, 0x31
0000000140382945: 4589fd                           mov r13d, r15d
0000000140382948: 0fb7857c010000                   movzx eax, word ptr [rbp + 0x17c]
000000014038294f: 66898584010000                   mov word ptr [rbp + 0x184], ax
0000000140382956: 8b8578010000                     mov eax, dword ptr [rbp + 0x178]
000000014038295c: 898580010000                     mov dword ptr [rbp + 0x180], eax
0000000140382962: 4c8bbd18010000                   mov r15, qword ptr [rbp + 0x118]
0000000140382969: 488b8d20010000                   mov rcx, qword ptr [rbp + 0x120]
0000000140382970: 4a8d047500000000                 lea rax, [r14*2]
0000000140382978: 4885c0                           test rax, rax
000000014038297b: 7439                             je 0x1403829b6
000000014038297d: 41b801000000                     mov r8d, 1
0000000140382983: 4c89f2                           mov rdx, r14
0000000140382986: e895122900                       call 0x140613c20 ; 
000000014038298b: eb29                             jmp 0x1403829b6 ; 
000000014038298d: 440f2815bb796402                 movaps xmm10, xmmword ptr [rip + 0x26479bb]
0000000140382995: 488d1d13846402                   lea rbx, [rip + 0x2648413]
000000014038299c: 4d85f6                           test r14, r14
000000014038299f: 7415                             je 0x1403829b6
00000001403829a1: 41b801000000                     mov r8d, 1
00000001403829a7: 488b8d20010000                   mov rcx, qword ptr [rbp + 0x120]
00000001403829ae: 4c89f2                           mov rdx, r14
00000001403829b1: e86a122900                       call 0x140613c20 ; 
00000001403829b6: 40887dc0                         mov byte ptr [rbp - 0x40], dil
00000001403829ba: 44886dc1                         mov byte ptr [rbp - 0x3f], r13b
00000001403829be: 8b8580010000                     mov eax, dword ptr [rbp + 0x180]
00000001403829c4: 8945c2                           mov dword ptr [rbp - 0x3e], eax
00000001403829c7: 0fb78584010000                   movzx eax, word ptr [rbp + 0x184]
00000001403829ce: 668945c6                         mov word ptr [rbp - 0x3a], ax
00000001403829d2: 48895dc8                         mov qword ptr [rbp - 0x38], rbx
00000001403829d6: 440f1155d0                       movups xmmword ptr [rbp - 0x30], xmm10
00000001403829db: 440f115de0                       movups xmmword ptr [rbp - 0x20], xmm11
00000001403829e0: 440f1175f0                       movups xmmword ptr [rbp - 0x10], xmm14
00000001403829e5: 48897500                         mov qword ptr [rbp], rsi
00000001403829e9: 488d8d30010000                   lea rcx, [rbp + 0x130]
00000001403829f0: 488d55c0                         lea rdx, [rbp - 0x40]
00000001403829f4: 4531c0                           xor r8d, r8d
00000001403829f7: 4531c9                           xor r9d, r9d
00000001403829fa: e8f1af8400                       call 0x140bcd9f0 ; ron::de::struct_error_name
00000001403829ff: 0fb69d30010000                   movzx ebx, byte ptr [rbp + 0x130]
0000000140382a06: 8b8531010000                     mov eax, dword ptr [rbp + 0x131]
0000000140382a0c: 898508010000                     mov dword ptr [rbp + 0x108], eax
0000000140382a12: 8b8534010000                     mov eax, dword ptr [rbp + 0x134]
0000000140382a18: 89850b010000                     mov dword ptr [rbp + 0x10b], eax
0000000140382a1e: 488bbd38010000                   mov rdi, qword ptr [rbp + 0x138]
0000000140382a25: 4c8bb540010000                   mov r14, qword ptr [rbp + 0x140]
0000000140382a2c: 4c8ba548010000                   mov r12, qword ptr [rbp + 0x148]
0000000140382a33: 488b8550010000                   mov rax, qword ptr [rbp + 0x150]
0000000140382a3a: 488985f0000000                   mov qword ptr [rbp + 0xf0], rax
0000000140382a41: 488b8558010000                   mov rax, qword ptr [rbp + 0x158]
0000000140382a48: 488b8d60010000                   mov rcx, qword ptr [rbp + 0x160]
0000000140382a4f: 48898df8000000                   mov qword ptr [rbp + 0xf8], rcx
0000000140382a56: 0f10b568010000                   movups xmm6, xmmword ptr [rbp + 0x168]
0000000140382a5d: 4c8badb8000000                   mov r13, qword ptr [rbp + 0xb8]
0000000140382a64: 488bb5c0000000                   mov rsi, qword ptr [rbp + 0xc0]
0000000140382a6b: 80fb32                           cmp bl, 0x32
0000000140382a6e: 0f851efdffff                     jne 0x140382792
0000000140382a74: 48897e08                         mov qword ptr [rsi + 8], rdi
0000000140382a78: 4c897610                         mov qword ptr [rsi + 0x10], r14
0000000140382a7c: 4c896618                         mov qword ptr [rsi + 0x18], r12
0000000140382a80: 488b8df0000000                   mov rcx, qword ptr [rbp + 0xf0]
0000000140382a87: 48894e20                         mov qword ptr [rsi + 0x20], rcx
0000000140382a8b: 48894628                         mov qword ptr [rsi + 0x28], rax
0000000140382a8f: 488b85f8000000                   mov rax, qword ptr [rbp + 0xf8]
0000000140382a96: 48894630                         mov qword ptr [rsi + 0x30], rax
0000000140382a9a: 0f137638                         movlps qword ptr [rsi + 0x38], xmm6
0000000140382a9e: c60632                           mov byte ptr [rsi], 0x32
0000000140382aa1: 4889f0                           mov rax, rsi
0000000140382aa4: 0f28b590010000                   movaps xmm6, xmmword ptr [rbp + 0x190]
0000000140382aab: 0f28bda0010000                   movaps xmm7, xmmword ptr [rbp + 0x1a0]
0000000140382ab2: 440f2885b0010000                 movaps xmm8, xmmword ptr [rbp + 0x1b0]
0000000140382aba: 440f288dc0010000                 movaps xmm9, xmmword ptr [rbp + 0x1c0]
0000000140382ac2: 440f2895d0010000                 movaps xmm10, xmmword ptr [rbp + 0x1d0]
0000000140382aca: 440f289de0010000                 movaps xmm11, xmmword ptr [rbp + 0x1e0]
0000000140382ad2: 440f28a5f0010000                 movaps xmm12, xmmword ptr [rbp + 0x1f0]
0000000140382ada: 440f28ad00020000                 movaps xmm13, xmmword ptr [rbp + 0x200]
0000000140382ae2: 440f28b510020000                 movaps xmm14, xmmword ptr [rbp + 0x210]
0000000140382aea: 440f28bd20020000                 movaps xmm15, xmmword ptr [rbp + 0x220]
0000000140382af2: 4881c4b8020000                   add rsp, 0x2b8
0000000140382af9: 5b                               pop rbx
0000000140382afa: 5f                               pop rdi
0000000140382afb: 5e                               pop rsi
0000000140382afc: 415c                             pop r12
0000000140382afe: 415d                             pop r13
0000000140382b00: 415e                             pop r14
0000000140382b02: 415f                             pop r15
0000000140382b04: 5d                               pop rbp
0000000140382b05: c3                               ret 
0000000140382b06: 8b8531010000                     mov eax, dword ptr [rbp + 0x131]
0000000140382b0c: 8b8d34010000                     mov ecx, dword ptr [rbp + 0x134]
0000000140382b12: 898d0b010000                     mov dword ptr [rbp + 0x10b], ecx
0000000140382b18: 898508010000                     mov dword ptr [rbp + 0x108], eax
0000000140382b1e: 488bbd38010000                   mov rdi, qword ptr [rbp + 0x138]
0000000140382b25: 4c8bb540010000                   mov r14, qword ptr [rbp + 0x140]
0000000140382b2c: 4c8ba548010000                   mov r12, qword ptr [rbp + 0x148]
0000000140382b33: 488b8550010000                   mov rax, qword ptr [rbp + 0x150]
0000000140382b3a: 488985f0000000                   mov qword ptr [rbp + 0xf0], rax
0000000140382b41: 488b8558010000                   mov rax, qword ptr [rbp + 0x158]
0000000140382b48: 488985e8000000                   mov qword ptr [rbp + 0xe8], rax
0000000140382b4f: 488b8560010000                   mov rax, qword ptr [rbp + 0x160]
0000000140382b56: 488985f8000000                   mov qword ptr [rbp + 0xf8], rax
0000000140382b5d: f30f6fb568010000                 movdqu xmm6, xmmword ptr [rbp + 0x168]
0000000140382b65: 488d8d80000000                   lea rcx, [rbp + 0x80]
0000000140382b6c: e85fb9ffff                       call 0x14037e4d0 ; core::ptr::drop_in_place<enum2$<country_core::meta_commander::MetaCommand> >
0000000140382b71: eb70                             jmp 0x140382be3 ; 
0000000140382b73: 488b8dc8000000                   mov rcx, qword ptr [rbp + 0xc8]
0000000140382b7a: ba29000000                       mov edx, 0x29
0000000140382b7f: e82c168400                       call 0x140bc41b0 ; ron::parse::Parser::consume_char
0000000140382b84: 84c0                             test al, al
0000000140382b86: 7448                             je 0x140382bd0
0000000140382b88: 488bbd80000000                   mov rdi, qword ptr [rbp + 0x80]
0000000140382b8f: 4c8bb588000000                   mov r14, qword ptr [rbp + 0x88]
0000000140382b96: 4c8ba590000000                   mov r12, qword ptr [rbp + 0x90]
0000000140382b9d: 488b8598000000                   mov rax, qword ptr [rbp + 0x98]
0000000140382ba4: 488985f0000000                   mov qword ptr [rbp + 0xf0], rax
0000000140382bab: 488b85a0000000                   mov rax, qword ptr [rbp + 0xa0]
0000000140382bb2: 488985e8000000                   mov qword ptr [rbp + 0xe8], rax
0000000140382bb9: 488b85a8000000                   mov rax, qword ptr [rbp + 0xa8]
0000000140382bc0: 488985f8000000                   mov qword ptr [rbp + 0xf8], rax
0000000140382bc7: 488b85b0000000                   mov rax, qword ptr [rbp + 0xb0]
0000000140382bce: eb0e                             jmp 0x140382bde ; 
0000000140382bd0: 488d8d80000000                   lea rcx, [rbp + 0x80]
0000000140382bd7: e8f4b8ffff                       call 0x14037e4d0 ; core::ptr::drop_in_place<enum2$<country_core::meta_commander::MetaCommand> >
0000000140382bdc: b318                             mov bl, 0x18
0000000140382bde: 66480f6ef0                       movq xmm6, rax
0000000140382be3: 488b85e8000000                   mov rax, qword ptr [rbp + 0xe8]
0000000140382bea: 80fb32                           cmp bl, 0x32
0000000140382bed: 0f8481feffff                     je 0x140382a74
0000000140382bf3: e99afbffff                       jmp 0x140382792 ; 
0000000140382bf8: 440fb6ad31010000                 movzx r13d, byte ptr [rbp + 0x131]
0000000140382c00: 8b8532010000                     mov eax, dword ptr [rbp + 0x132]
0000000140382c06: 898580010000                     mov dword ptr [rbp + 0x180], eax
0000000140382c0c: 0fb78536010000                   movzx eax, word ptr [rbp + 0x136]
0000000140382c13: 66898584010000                   mov word ptr [rbp + 0x184], ax
0000000140382c1a: 488b9d38010000                   mov rbx, qword ptr [rbp + 0x138]
0000000140382c21: 440f109540010000                 movups xmm10, xmmword ptr [rbp + 0x140]
0000000140382c29: 440f109d50010000                 movups xmm11, xmmword ptr [rbp + 0x150]
0000000140382c31: 440f10b560010000                 movups xmm14, xmmword ptr [rbp + 0x160]
0000000140382c39: 488bb570010000                   mov rsi, qword ptr [rbp + 0x170]
0000000140382c40: e936fbffff                       jmp 0x14038277b ; 
0000000140382c45: 40b72d                           mov dil, 0x2d
0000000140382c48: 488d1d60816402                   lea rbx, [rip + 0x2648160]
0000000140382c4f: 440f2815f9766402                 movaps xmm10, xmmword ptr [rip + 0x26476f9]
0000000140382c57: e906fdffff                       jmp 0x140382962 ; 
0000000140382c5c: 40b72d                           mov dil, 0x2d
0000000140382c5f: 4c8bbd18010000                   mov r15, qword ptr [rbp + 0x118]
0000000140382c66: 488b8d20010000                   mov rcx, qword ptr [rbp + 0x120]
0000000140382c6d: 488d1de47b6402                   lea rbx, [rip + 0x2647be4]
0000000140382c74: 440f2815b4766402                 movaps xmm10, xmmword ptr [rip + 0x26476b4]
0000000140382c7c: 4a8d047500000000                 lea rax, [r14*2]
0000000140382c84: 4885c0                           test rax, rax
0000000140382c87: 0f8429fdffff                     je 0x1403829b6
0000000140382c8d: e9ebfcffff                       jmp 0x14038297d ; 
0000000140382c92: 66666666662e0f1f840000000000     nop word ptr cs:[rax + rax]
0000000140382ca0: 4889542410                       mov qword ptr [rsp + 0x10], rdx
0000000140382ca5: 55                               push rbp
0000000140382ca6: 4157                             push r15
0000000140382ca8: 4156                             push r14
0000000140382caa: 4155                             push r13
0000000140382cac: 4154                             push r12
0000000140382cae: 56                               push rsi
0000000140382caf: 57                               push rdi
0000000140382cb0: 53                               push rbx
0000000140382cb1: 4881ecc8000000                   sub rsp, 0xc8
0000000140382cb8: 488daa80000000                   lea rbp, [rdx + 0x80]
0000000140382cbf: 440f297c2420                     movaps xmmword ptr [rsp + 0x20], xmm15
0000000140382cc5: 440f29742430                     movaps xmmword ptr [rsp + 0x30], xmm14
0000000140382ccb: 440f296c2440                     movaps xmmword ptr [rsp + 0x40], xmm13
0000000140382cd1: 440f29642450                     movaps xmmword ptr [rsp + 0x50], xmm12
0000000140382cd7: 440f295c2460                     movaps xmmword ptr [rsp + 0x60], xmm11
0000000140382cdd: 440f29542470                     movaps xmmword ptr [rsp + 0x70], xmm10
0000000140382ce3: 440f298c2480000000               movaps xmmword ptr [rsp + 0x80], xmm9
0000000140382cec: 440f29842490000000               movaps xmmword ptr [rsp + 0x90], xmm8
0000000140382cf5: 0f29bc24a0000000                 movaps xmmword ptr [rsp + 0xa0], xmm7
0000000140382cfd: 0f29b424b0000000                 movaps xmmword ptr [rsp + 0xb0], xmm6
0000000140382d05: 488d8d80000000                   lea rcx, [rbp + 0x80]
0000000140382d0c: e8bfb7ffff                       call 0x14037e4d0 ; core::ptr::drop_in_place<enum2$<country_core::meta_commander::MetaCommand> >
0000000140382d11: 0f28b424b0000000                 movaps xmm6, xmmword ptr [rsp + 0xb0]
0000000140382d19: 0f28bc24a0000000                 movaps xmm7, xmmword ptr [rsp + 0xa0]
0000000140382d21: 440f28842490000000               movaps xmm8, xmmword ptr [rsp + 0x90]
0000000140382d2a: 440f288c2480000000               movaps xmm9, xmmword ptr [rsp + 0x80]
0000000140382d33: 440f28542470                     movaps xmm10, xmmword ptr [rsp + 0x70]
0000000140382d39: 440f285c2460                     movaps xmm11, xmmword ptr [rsp + 0x60]
0000000140382d3f: 440f28642450                     movaps xmm12, xmmword ptr [rsp + 0x50]
0000000140382d45: 440f286c2440                     movaps xmm13, xmmword ptr [rsp + 0x40]
0000000140382d4b: 440f28742430                     movaps xmm14, xmmword ptr [rsp + 0x30]
0000000140382d51: 440f287c2420                     movaps xmm15, xmmword ptr [rsp + 0x20]
0000000140382d57: 4881c4c8000000                   add rsp, 0xc8
0000000140382d5e: 5b                               pop rbx
0000000140382d5f: 5f                               pop rdi
0000000140382d60: 5e                               pop rsi
0000000140382d61: 415c                             pop r12
0000000140382d63: 415d                             pop r13
0000000140382d65: 415e                             pop r14
0000000140382d67: 415f                             pop r15
0000000140382d69: 5d                               pop rbp
0000000140382d6a: c3                               ret 
0000000140382d6b: 0f1f440000                       nop dword ptr [rax + rax]
0000000140382d70: 4889542410                       mov qword ptr [rsp + 0x10], rdx
0000000140382d75: 55                               push rbp
0000000140382d76: 4157                             push r15
0000000140382d78: 4156                             push r14
0000000140382d7a: 4155                             push r13
0000000140382d7c: 4154                             push r12
0000000140382d7e: 56                               push rsi
0000000140382d7f: 57                               push rdi
0000000140382d80: 53                               push rbx
0000000140382d81: 4881ecc8000000                   sub rsp, 0xc8
0000000140382d88: 488daa80000000                   lea rbp, [rdx + 0x80]
0000000140382d8f: 440f297c2420                     movaps xmmword ptr [rsp + 0x20], xmm15
0000000140382d95: 440f29742430                     movaps xmmword ptr [rsp + 0x30], xmm14
0000000140382d9b: 440f296c2440                     movaps xmmword ptr [rsp + 0x40], xmm13
0000000140382da1: 440f29642450                     movaps xmmword ptr [rsp + 0x50], xmm12
0000000140382da7: 440f295c2460                     movaps xmmword ptr [rsp + 0x60], xmm11
0000000140382dad: 440f29542470                     movaps xmmword ptr [rsp + 0x70], xmm10
0000000140382db3: 440f298c2480000000               movaps xmmword ptr [rsp + 0x80], xmm9
0000000140382dbc: 440f29842490000000               movaps xmmword ptr [rsp + 0x90], xmm8
0000000140382dc5: 0f29bc24a0000000                 movaps xmmword ptr [rsp + 0xa0], xmm7
0000000140382dcd: 0f29b424b0000000                 movaps xmmword ptr [rsp + 0xb0], xmm6
0000000140382dd5: 488b9500010000                   mov rdx, qword ptr [rbp + 0x100]
0000000140382ddc: 4889d0                           mov rax, rdx
0000000140382ddf: 48f7d8                           neg rax
0000000140382de2: 7014                             jo 0x140382df8
0000000140382de4: 7312                             jae 0x140382df8
0000000140382de6: 41b801000000                     mov r8d, 1
0000000140382dec: 488b8d20010000                   mov rcx, qword ptr [rbp + 0x120]
0000000140382df3: e8280e2900                       call 0x140613c20 ; 
0000000140382df8: 0f28b424b0000000                 movaps xmm6, xmmword ptr [rsp + 0xb0]
0000000140382e00: 0f28bc24a0000000                 movaps xmm7, xmmword ptr [rsp + 0xa0]
0000000140382e08: 440f28842490000000               movaps xmm8, xmmword ptr [rsp + 0x90]
0000000140382e11: 440f288c2480000000               movaps xmm9, xmmword ptr [rsp + 0x80]
0000000140382e1a: 440f28542470                     movaps xmm10, xmmword ptr [rsp + 0x70]
0000000140382e20: 440f285c2460                     movaps xmm11, xmmword ptr [rsp + 0x60]
0000000140382e26: 440f28642450                     movaps xmm12, xmmword ptr [rsp + 0x50]
0000000140382e2c: 440f286c2440                     movaps xmm13, xmmword ptr [rsp + 0x40]
0000000140382e32: 440f28742430                     movaps xmm14, xmmword ptr [rsp + 0x30]
0000000140382e38: 440f287c2420                     movaps xmm15, xmmword ptr [rsp + 0x20]
0000000140382e3e: 4881c4c8000000                   add rsp, 0xc8
0000000140382e45: 5b                               pop rbx
0000000140382e46: 5f                               pop rdi
0000000140382e47: 5e                               pop rsi
0000000140382e48: 415c                             pop r12
0000000140382e4a: 415d                             pop r13
0000000140382e4c: 415e                             pop r14
0000000140382e4e: 415f                             pop r15
0000000140382e50: 5d                               pop rbp
0000000140382e51: c3                               ret 
