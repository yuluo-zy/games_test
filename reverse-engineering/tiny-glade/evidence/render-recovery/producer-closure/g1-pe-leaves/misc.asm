=== 0x1405ba5f0 size=2299  rhapsody::raw::texture::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_texture::h892755ff801df28f
    ; Large function body fully decoded (not a micro-leaf); decode present in insns.  [medium]
    0x1405ba5f0    55                       push             rbp
    0x1405ba5f1    4157                     push             r15
    0x1405ba5f3    4156                     push             r14
    0x1405ba5f5    4155                     push             r13
    0x1405ba5f7    4154                     push             r12
    0x1405ba5f9    56                       push             rsi
    0x1405ba5fa    57                       push             rdi
    0x1405ba5fb    53                       push             rbx
    0x1405ba5fc    4881ec18040000           sub              rsp, 0x418
    0x1405ba603    488dac2480000000         lea              rbp, [rsp + 0x80]
    0x1405ba60b    48c78590030000feffffff   mov              qword ptr [rbp + 0x390], 0xfffffffffffffffe
    0x1405ba616    4c898578030000           mov              qword ptr [rbp + 0x378], r8
    0x1405ba61d    4889d3                   mov              rbx, rdx
    0x1405ba620    4889ce                   mov              rsi, rcx
    0x1405ba623    488b9500040000           mov              rdx, qword ptr [rbp + 0x400]
    0x1405ba62a    488b8508040000           mov              rax, qword ptr [rbp + 0x408]
    0x1405ba631    488b4810                 mov              rcx, qword ptr [rax + 0x10]
    0x1405ba635    48898df0010000           mov              qword ptr [rbp + 0x1f0], rcx
    0x1405ba63c    0f1000                   movups           xmm0, xmmword ptr [rax]
    0x1405ba63f    0f2985e0010000           movaps           xmmword ptr [rbp + 0x1e0], xmm0
    0x1405ba646    8b02                     mov              eax, dword ptr [rdx]
    0x1405ba648    85c0                     test             eax, eax
    0x1405ba64a    0f84fb060000             je               0x1405bad4b
    0x1405ba650    4d89ce                   mov              r14, r9
    0x1405ba653    488bbde0010000           mov              rdi, qword ptr [rbp + 0x1e0]
    0x1405ba65a    4885ff                   test             rdi, rdi
    0x1405ba65d    7405                     je               0x1405ba664
    0x1405ba65f    83c802                   or               eax, 2
    0x1405ba662    8902                     mov              dword ptr [rdx], eax
    0x1405ba664    4885ff                   test             rdi, rdi
    0x1405ba667    410f95c0                 setne            r8b
    0x1405ba66b    488d4d78                 lea              rcx, [rbp + 0x78]
    0x1405ba66f    e85cb86100               call             0x140bd5ed0 -> rhapsody::raw::texture::get_image_create_info
    0x1405ba674    83bd9400000000           cmp              dword ptr [rbp + 0x94], 0
    0x1405ba67b    0f84e2060000             je               0x1405bad63
    0x1405ba681    83bd9800000000           cmp              dword ptr [rbp + 0x98], 0
    0x1405ba688    0f84ed060000             je               0x1405bad7b
    0x1405ba68e    83bd9c00000000           cmp              dword ptr [rbp + 0x9c], 0
    0x1405ba695    0f84f8060000             je               0x1405bad93
    0x1405ba69b    488b8b40060000           mov              rcx, qword ptr [rbx + 0x640]
    0x1405ba6a2    488d5578                 lea              rdx, [rbp + 0x78]
    0x1405ba6a6    4c8d8d70020000           lea              r9, [rbp + 0x270]
    0x1405ba6ad    4531c0                   xor              r8d, r8d
    0x1405ba6b0    ff93a0010000             call             qword ptr [rbx + 0x1a0]
    0x1405ba6b6    85c0                     test             eax, eax
    0x1405ba6b8    0f8516070000             jne              0x1405badd4
    0x1405ba6be    4c8ba570020000           mov              r12, qword ptr [rbp + 0x270]
    0x1405ba6c5    4889d9                   mov              rcx, rbx
    0x1405ba6c8    e893e26300               call             0x140bf8960 -> rhapsody::raw::device::GraphicsDevice::debug_utils
    0x1405ba6cd    4885c0                   test             rax, rax
    0x1405ba6d0    0f84a9000000             je               0x1405ba77f
    0x1405ba6d6    4989c5                   mov              r13, rax
    0x1405ba6d9    4d89f0                   mov              r8, r14
    0x1405ba6dc    4889b580030000           mov              qword ptr [rbp + 0x380], rsi
    0x1405ba6e3    4c89e6                   mov              rsi, r12
    0x1405ba6e6    488d8dd0000000           lea              rcx, [rbp + 0xd0]
    0x1405ba6ed    488b9578030000           mov              rdx, qword ptr [rbp + 0x378]
    0x1405ba6f4    e8d7f07b00               call             0x140d797d0 -> _$LT$$RF$str$u20$as$u20$alloc..ffi..c_str..CString..new..SpecNewImpl$GT$::spec_new_impl::h27339bb99b671fc9
    0x1405ba6f9    31c0                     xor              eax, eax
    0x1405ba6fb    483b85d0000000           cmp              rax, qword ptr [rbp + 0xd0]
    0x1405ba702    0f813a070000             jno              0x1405bae42
    0x1405ba708    4c8ba5d8000000           mov              r12, qword ptr [rbp + 0xd8]
    0x1405ba70f    4c8bbde0000000           mov              r15, qword ptr [rbp + 0xe0]
    0x1405ba716    c7857002000000be9c3b     mov              dword ptr [rbp + 0x270], 0x3b9cbe00
    0x1405ba720    48c7857802000000000000   mov              qword ptr [rbp + 0x278], 0
    0x1405ba72b    c785800200000a000000     mov              dword ptr [rbp + 0x280], 0xa
    0x1405ba735    4889b588020000           mov              qword ptr [rbp + 0x288], rsi
    0x1405ba73c    4c89a590020000           mov              qword ptr [rbp + 0x290], r12
    0x1405ba743    498b4d40                 mov              rcx, qword ptr [r13 + 0x40]
    0x1405ba747    488d9570020000           lea              rdx, [rbp + 0x270]
    0x1405ba74e    41ff5500                 call             qword ptr [r13]
    0x1405ba752    85c0                     test             eax, eax
    0x1405ba754    0f8531070000             jne              0x1405bae8b
    0x1405ba75a    41c6042400               mov              byte ptr [r12], 0
    0x1405ba75f    4d85ff                   test             r15, r15
    0x1405ba762    4c89e1                   mov              rcx, r12
    0x1405ba765    4989f4                   mov              r12, rsi
    0x1405ba768    488bb580030000           mov              rsi, qword ptr [rbp + 0x380]
    0x1405ba76f    740e                     je               0x1405ba77f
    0x1405ba771    41b801000000             mov              r8d, 1
    0x1405ba777    4c89fa                   mov              rdx, r15
    0x1405ba77a    e8a1940500               call             0x140613c20 -> __rust_dealloc
    0x1405ba77f    4d89f7                   mov              r15, r14
    0x1405ba782    488b8b40060000           mov              rcx, qword ptr [rbx + 0x640]
    0x1405ba789    4c8d8550030000           lea              r8, [rbp + 0x350]
    0x1405ba790    4c89e2                   mov              rdx, r12
    0x1405ba793    ff93f0000000             call             qword ptr [rbx + 0xf0]
    0x1405ba799    4c8bab78060000           mov              r13, qword ptr [rbx + 0x678]
    0x1405ba7a0    4d8d7510                 lea              r14, [r13 + 0x10]
    0x1405ba7a4    b101                     mov              cl, 1
    0x1405ba7a6    31c0                     xor              eax, eax
    0x1405ba7a8    f0410fb04d10             lock cmpxchg     byte ptr [r13 + 0x10], cl
    0x1405ba7ae    0f85f7050000             jne              0x1405badab
    0x1405ba7b4    4c89b568030000           mov              qword ptr [rbp + 0x368], r14
    0x1405ba7bb    4c89f1                   mov              rcx, r14
    0x1405ba7be    e8ad746c00               call             0x140c81c70 -> parking_lot_core::parking_lot::deadlock_impl::acquire_resource::hb2f16ac28e483830
    0x1405ba7c3    4983c518                 add              r13, 0x18
    0x1405ba7c7    488b8578030000           mov              rax, qword ptr [rbp + 0x378]
    0x1405ba7ce    488985e0000000           mov              qword ptr [rbp + 0xe0], rax
    0x1405ba7d5    4d89fe                   mov              r14, r15
    0x1405ba7d8    4c89bde8000000           mov              qword ptr [rbp + 0xe8], r15
    0x1405ba7df    0f108550030000           movups           xmm0, xmmword ptr [rbp + 0x350]
    0x1405ba7e6    0f1185f0000000           movups           xmmword ptr [rbp + 0xf0], xmm0
    0x1405ba7ed    488b8560030000           mov              rax, qword ptr [rbp + 0x360]
    0x1405ba7f4    48898500010000           mov              qword ptr [rbp + 0x100], rax
    0x1405ba7fb    66c785080100000001       mov              word ptr [rbp + 0x108], 0x100
    0x1405ba804    48c785d000000002000000   mov              qword ptr [rbp + 0xd0], 2
    0x1405ba80f    488d8d70020000           lea              rcx, [rbp + 0x270]
    0x1405ba816    4c8d85d0000000           lea              r8, [rbp + 0xd0]
    0x1405ba81d    4c89ea                   mov              rdx, r13
    0x1405ba820    e81b326500               call             0x140c0da40 -> rhapsody::raw::allocator::GpuAllocator::allocate
    0x1405ba825    4889bd68020000           mov              qword ptr [rbp + 0x268], rdi
    0x1405ba82c    80bdb402000002           cmp              byte ptr [rbp + 0x2b4], 2
    0x1405ba833    488b8570020000           mov              rax, qword ptr [rbp + 0x270]
    0x1405ba83a    48898580030000           mov              qword ptr [rbp + 0x380], rax
    0x1405ba841    4c8bad88020000           mov              r13, qword ptr [rbp + 0x288]
    0x1405ba848    0f859a000000             jne              0x1405ba8e8
    0x1405ba84e    488b8578020000           mov              rax, qword ptr [rbp + 0x278]
    0x1405ba855    48898570030000           mov              qword ptr [rbp + 0x370], rax
    0x1405ba85c    4c8bbd80020000           mov              r15, qword ptr [rbp + 0x280]
    0x1405ba863    0fb6059fa30703           movzx            eax, byte ptr [rip + 0x307a39f] -> 0x143634c09
    0x1405ba86a    bf0f000000               mov              edi, 0xf
    0x1405ba86f    b90f000000               mov              ecx, 0xf
    0x1405ba874    ba01000000               mov              edx, 1
    0x1405ba879    e892930500               call             0x140613c10 -> __rust_alloc
    0x1405ba87e    4885c0                   test             rax, rax
    0x1405ba881    0f8445060000             je               0x1405baecc
    0x1405ba887    48b92074657874757265     movabs           rcx, 0x6572757478657420
    0x1405ba891    48894807                 mov              qword ptr [rax + 7], rcx
    0x1405ba895    48b94770754f6e6c7920     movabs           rcx, 0x20796c6e4f757047
    0x1405ba89f    488908                   mov              qword ptr [rax], rcx
    0x1405ba8a2    488b8d60030000           mov              rcx, qword ptr [rbp + 0x360]
    0x1405ba8a9    48898da0010000           mov              qword ptr [rbp + 0x1a0], rcx
    0x1405ba8b0    0f108550030000           movups           xmm0, xmmword ptr [rbp + 0x350]
    0x1405ba8b7    0f298590010000           movaps           xmmword ptr [rbp + 0x190], xmm0
    0x1405ba8be    488b8d80030000           mov              rcx, qword ptr [rbp + 0x380]
    0x1405ba8c5    4883f90b                 cmp              rcx, 0xb
    0x1405ba8c9    0f8594030000             jne              0x1405bac63
    0x1405ba8cf    4c89e9                   mov              rcx, r13
    0x1405ba8d2    488b9570030000           mov              rdx, qword ptr [rbp + 0x370]
    0x1405ba8d9    48899580030000           mov              qword ptr [rbp + 0x380], rdx
    0x1405ba8e0    41bd0f000000             mov              r13d, 0xf
    0x1405ba8e6    eb38                     jmp              0x1405ba920
    0x1405ba8e8    4c8bbd78020000           mov              r15, qword ptr [rbp + 0x278]
    0x1405ba8ef    488b8d80020000           mov              rcx, qword ptr [rbp + 0x280]
    0x1405ba8f6    488b8590020000           mov              rax, qword ptr [rbp + 0x290]
    0x1405ba8fd    488bbd98020000           mov              rdi, qword ptr [rbp + 0x298]
    0x1405ba904    0f1085a0020000           movups           xmm0, xmmword ptr [rbp + 0x2a0]
    0x1405ba90b    0f298590010000           movaps           xmmword ptr [rbp + 0x190], xmm0
    0x1405ba912    488b95b0020000           mov              rdx, qword ptr [rbp + 0x2b0]
    0x1405ba919    488995a0010000           mov              qword ptr [rbp + 0x1a0], rdx
    0x1405ba920    488b95a0010000           mov              rdx, qword ptr [rbp + 0x1a0]
    0x1405ba927    48899510020000           mov              qword ptr [rbp + 0x210], rdx
    0x1405ba92e    0f288590010000           movaps           xmm0, xmmword ptr [rbp + 0x190]
    0x1405ba935    0f298500020000           movaps           xmmword ptr [rbp + 0x200], xmm0
    0x1405ba93c    4c8b8580030000           mov              r8, qword ptr [rbp + 0x380]
    0x1405ba943    4c898520020000           mov              qword ptr [rbp + 0x220], r8
    0x1405ba94a    4c89bd28020000           mov              qword ptr [rbp + 0x228], r15
    0x1405ba951    48898d30020000           mov              qword ptr [rbp + 0x230], rcx
    0x1405ba958    0f118550020000           movups           xmmword ptr [rbp + 0x250], xmm0
    0x1405ba95f    48899560020000           mov              qword ptr [rbp + 0x260], rdx
    0x1405ba966    4c89ad38020000           mov              qword ptr [rbp + 0x238], r13
    0x1405ba96d    48898540020000           mov              qword ptr [rbp + 0x240], rax
    0x1405ba974    4889bd48020000           mov              qword ptr [rbp + 0x248], rdi
    0x1405ba97b    c6858f03000001           mov              byte ptr [rbp + 0x38f], 1
    0x1405ba982    488bbd68030000           mov              rdi, qword ptr [rbp + 0x368]
    0x1405ba989    4889f9                   mov              rcx, rdi
    0x1405ba98c    e87f746c00               call             0x140c81e10 -> parking_lot_core::parking_lot::deadlock_impl::release_resource::h1c3ab4db915ad6f2
    0x1405ba991    31c9                     xor              ecx, ecx
    0x1405ba993    b001                     mov              al, 1
    0x1405ba995    f00fb00f                 lock cmpxchg     byte ptr [rdi], cl
    0x1405ba999    0f851f040000             jne              0x1405badbe
    0x1405ba99f    c6858f03000001           mov              byte ptr [rbp + 0x38f], 1
    0x1405ba9a6    488d8d20020000           lea              rcx, [rbp + 0x220]
    0x1405ba9ad    e8fe621100               call             0x1406d0cb0 -> gpu_allocator::vulkan::Allocation::memory::hfb1e6a660a853183
    0x1405ba9b2    4989c5                   mov              r13, rax
    0x1405ba9b5    c6858f03000001           mov              byte ptr [rbp + 0x38f], 1
    0x1405ba9bc    488d8d20020000           lea              rcx, [rbp + 0x220]
    0x1405ba9c3    e8e8be0c00               call             0x1406868b0 -> rhapsody::raw::buffer::impl$3::byte_count
    0x1405ba9c8    488b8b40060000           mov              rcx, qword ptr [rbx + 0x640]
    0x1405ba9cf    4c89e2                   mov              rdx, r12
    0x1405ba9d2    4d89e8                   mov              r8, r13
    0x1405ba9d5    4989c1                   mov              r9, rax
    0x1405ba9d8    ff93e0000000             call             qword ptr [rbx + 0xe0]
    0x1405ba9de    85c0                     test             eax, eax
    0x1405ba9e0    488b9500040000           mov              rdx, qword ptr [rbp + 0x400]
    0x1405ba9e7    0f8518040000             jne              0x1405bae05
    0x1405ba9ed    488d4378                 lea              rax, [rbx + 0x78]
    0x1405ba9f1    4c8bab48060000           mov              r13, qword ptr [rbx + 0x648]
    0x1405ba9f8    488b8d60020000           mov              rcx, qword ptr [rbp + 0x260]
    0x1405ba9ff    48898dd0010000           mov              qword ptr [rbp + 0x1d0], rcx
    0x1405baa06    0f108520020000           movups           xmm0, xmmword ptr [rbp + 0x220]
    0x1405baa0d    0f108d30020000           movups           xmm1, xmmword ptr [rbp + 0x230]
    0x1405baa14    0f109540020000           movups           xmm2, xmmword ptr [rbp + 0x240]
    0x1405baa1b    0f109d50020000           movups           xmm3, xmmword ptr [rbp + 0x250]
    0x1405baa22    0f299dc0010000           movaps           xmmword ptr [rbp + 0x1c0], xmm3
    0x1405baa29    0f2995b0010000           movaps           xmmword ptr [rbp + 0x1b0], xmm2
    0x1405baa30    0f298da0010000           movaps           xmmword ptr [rbp + 0x1a0], xmm1
    0x1405baa37    0f298590010000           movaps           xmmword ptr [rbp + 0x190], xmm0
    0x1405baa3e    0f1002                   movups           xmm0, xmmword ptr [rdx]
    0x1405baa41    0f104a10                 movups           xmm1, xmmword ptr [rdx + 0x10]
    0x1405baa45    0f298d10020000           movaps           xmmword ptr [rbp + 0x210], xmm1
    0x1405baa4c    0f298500020000           movaps           xmmword ptr [rbp + 0x200], xmm0
    0x1405baa53    c6858f03000000           mov              byte ptr [rbp + 0x38f], 0
    0x1405baa5a    488d8d00020000           lea              rcx, [rbp + 0x200]
    0x1405baa61    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x1405baa66    488d8d90010000           lea              rcx, [rbp + 0x190]
    0x1405baa6d    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x1405baa72    4889442420               mov              qword ptr [rsp + 0x20], rax
    0x1405baa77    488d8d70020000           lea              rcx, [rbp + 0x270]
    0x1405baa7e    488b9578030000           mov              rdx, qword ptr [rbp + 0x378]
    0x1405baa85    4d89f0                   mov              r8, r14
    0x1405baa88    4d89e1                   mov              r9, r12
    0x1405baa8b    e840186200               call             0x140bdc2d0 -> rhapsody::raw::managed::texture::ManagedTexture::new
    0x1405baa90    0fb6bd2c030000           movzx            edi, byte ptr [rbp + 0x32c]
    0x1405baa97    4080ff03                 cmp              dil, 3
    0x1405baa9b    755e                     jne              0x1405baafb
    0x1405baa9d    0f1085b0020000           movups           xmm0, xmmword ptr [rbp + 0x2b0]
    0x1405baaa4    0f298510010000           movaps           xmmword ptr [rbp + 0x110], xmm0
    0x1405baaab    0f108d70020000           movups           xmm1, xmmword ptr [rbp + 0x270]
    0x1405baab2    0f109580020000           movups           xmm2, xmmword ptr [rbp + 0x280]
    0x1405baab9    0f109d90020000           movups           xmm3, xmmword ptr [rbp + 0x290]
    0x1405baac0    0f10a5a0020000           movups           xmm4, xmmword ptr [rbp + 0x2a0]
    0x1405baac7    0f29a500010000           movaps           xmmword ptr [rbp + 0x100], xmm4
    0x1405baace    0f299df0000000           movaps           xmmword ptr [rbp + 0xf0], xmm3
    0x1405baad5    0f2995e0000000           movaps           xmmword ptr [rbp + 0xe0], xmm2
    0x1405baadc    0f298dd0000000           movaps           xmmword ptr [rbp + 0xd0], xmm1
    0x1405baae3    0f114640                 movups           xmmword ptr [rsi + 0x40], xmm0
    0x1405baae7    0f116630                 movups           xmmword ptr [rsi + 0x30], xmm4
    0x1405baaeb    0f115e20                 movups           xmmword ptr [rsi + 0x20], xmm3
    0x1405baaef    0f115610                 movups           xmmword ptr [rsi + 0x10], xmm2
    0x1405baaf3    0f110e                   movups           xmmword ptr [rsi], xmm1
    0x1405baaf6    e939020000               jmp              0x1405bad34
    0x1405baafb    4c8db5d0000000           lea              r14, [rbp + 0xd0]
    0x1405bab02    488d9570020000           lea              rdx, [rbp + 0x270]
    0x1405bab09    41b8bc000000             mov              r8d, 0xbc
    0x1405bab0f    4c89f1                   mov              rcx, r14
    0x1405bab12    e8e9903602               call             0x142923c00 -> memcpy
    0x1405bab17    0fb7852d030000           movzx            eax, word ptr [rbp + 0x32d]
    0x1405bab1e    66894575                 mov              word ptr [rbp + 0x75], ax
    0x1405bab22    0fb6852f030000           movzx            eax, byte ptr [rbp + 0x32f]
    0x1405bab29    884577                   mov              byte ptr [rbp + 0x77], al
    0x1405bab2c    4c8d7db8                 lea              r15, [rbp - 0x48]
    0x1405bab30    41b8bc000000             mov              r8d, 0xbc
    0x1405bab36    4c89f9                   mov              rcx, r15
    0x1405bab39    4c89f2                   mov              rdx, r14
    0x1405bab3c    e8bf903602               call             0x142923c00 -> memcpy
    0x1405bab41    40887d74                 mov              byte ptr [rbp + 0x74], dil
    0x1405bab45    c6858f03000000           mov              byte ptr [rbp + 0x38f], 0
    0x1405bab4c    4c89e9                   mov              rcx, r13
    0x1405bab4f    4c89fa                   mov              rdx, r15
    0x1405bab52    e8f9116200               call             0x140bdbd50 -> rhapsody::raw::managed::resources::impl$6::allocate
    0x1405bab57    8b480c                   mov              ecx, dword ptr [rax + 0xc]
    0x1405bab5a    410fb795500a0000         movzx            edx, word ptr [r13 + 0xa50]
    0x1405bab62    4d8b4500                 mov              r8, qword ptr [r13]
    0x1405bab66    4c898538030000           mov              qword ptr [rbp + 0x338], r8
    0x1405bab6d    4989c0                   mov              r8, rax
    0x1405bab70    4983c010                 add              r8, 0x10
    0x1405bab74    48898540030000           mov              qword ptr [rbp + 0x340], rax
    0x1405bab7b    898d48030000             mov              dword ptr [rbp + 0x348], ecx
    0x1405bab81    6689954c030000           mov              word ptr [rbp + 0x34c], dx
    0x1405bab88    c6854e03000000           mov              byte ptr [rbp + 0x34e], 0
    0x1405bab8f    4c898530030000           mov              qword ptr [rbp + 0x330], r8
    0x1405bab96    4883bd6802000000         cmp              qword ptr [rbp + 0x268], 0
    0x1405bab9e    488bbd00040000           mov              rdi, qword ptr [rbp + 0x400]
    0x1405baba5    744f                     je               0x1405babf6
    0x1405baba7    0f1007                   movups           xmm0, xmmword ptr [rdi]
    0x1405babaa    0f104f10                 movups           xmm1, xmmword ptr [rdi + 0x10]
    0x1405babae    0f298de0000000           movaps           xmmword ptr [rbp + 0xe0], xmm1
    0x1405babb5    0f2985d0000000           movaps           xmmword ptr [rbp + 0xd0], xmm0
    0x1405babbc    488d8530030000           lea              rax, [rbp + 0x330]
    0x1405babc3    4889442420               mov              qword ptr [rsp + 0x20], rax
    0x1405babc8    488d8d70020000           lea              rcx, [rbp + 0x270]
    0x1405babcf    4c8d85d0000000           lea              r8, [rbp + 0xd0]
    0x1405babd6    4c8d8de0010000           lea              r9, [rbp + 0x1e0]
    0x1405babdd    4889da                   mov              rdx, rbx
    0x1405babe0    e88b366400               call             0x140bfe270 -> rhapsody::raw::device::GraphicsDevice::init_texture_data
    0x1405babe5    488b8570020000           mov              rax, qword ptr [rbp + 0x270]
    0x1405babec    4883f80b                 cmp              rax, 0xb
    0x1405babf0    0f85f1000000             jne              0x1405bace7
    0x1405babf6    488b8548030000           mov              rax, qword ptr [rbp + 0x348]
    0x1405babfd    48898580020000           mov              qword ptr [rbp + 0x280], rax
    0x1405bac04    0f108538030000           movups           xmm0, xmmword ptr [rbp + 0x338]
    0x1405bac0b    0f298570020000           movaps           xmmword ptr [rbp + 0x270], xmm0
    0x1405bac12    0f100f                   movups           xmm1, xmmword ptr [rdi]
    0x1405bac15    0f105710                 movups           xmm2, xmmword ptr [rdi + 0x10]
    0x1405bac19    0f119598020000           movups           xmmword ptr [rbp + 0x298], xmm2
    0x1405bac20    0f118d88020000           movups           xmmword ptr [rbp + 0x288], xmm1
    0x1405bac27    0f114608                 movups           xmmword ptr [rsi + 8], xmm0
    0x1405bac2b    488b85a0020000           mov              rax, qword ptr [rbp + 0x2a0]
    0x1405bac32    48894638                 mov              qword ptr [rsi + 0x38], rax
    0x1405bac36    0f288590020000           movaps           xmm0, xmmword ptr [rbp + 0x290]
    0x1405bac3d    0f114628                 movups           xmmword ptr [rsi + 0x28], xmm0
    0x1405bac41    488b8580020000           mov              rax, qword ptr [rbp + 0x280]
    0x1405bac48    48894618                 mov              qword ptr [rsi + 0x18], rax
    0x1405bac4c    488b8588020000           mov              rax, qword ptr [rbp + 0x288]
    0x1405bac53    48894620                 mov              qword ptr [rsi + 0x20], rax
    0x1405bac57    48c7060b000000           mov              qword ptr [rsi], 0xb
    0x1405bac5e    e9d1000000               jmp              0x1405bad34
    0x1405bac63    4889ca                   mov              rdx, rcx
    0x1405bac66    488b8d60030000           mov              rcx, qword ptr [rbp + 0x360]
    0x1405bac6d    48898d10020000           mov              qword ptr [rbp + 0x210], rcx
    0x1405bac74    0f108550030000           movups           xmm0, xmmword ptr [rbp + 0x350]
    0x1405bac7b    0f298500020000           movaps           xmmword ptr [rbp + 0x200], xmm0
    0x1405bac82    488b8d70030000           mov              rcx, qword ptr [rbp + 0x370]
    0x1405bac89    48894e08                 mov              qword ptr [rsi + 8], rcx
    0x1405bac8d    4c897e10                 mov              qword ptr [rsi + 0x10], r15
    0x1405bac91    4c896e18                 mov              qword ptr [rsi + 0x18], r13
    0x1405bac95    0f108550030000           movups           xmm0, xmmword ptr [rbp + 0x350]
    0x1405bac9c    0f114638                 movups           xmmword ptr [rsi + 0x38], xmm0
    0x1405baca0    488b8d60030000           mov              rcx, qword ptr [rbp + 0x360]
    0x1405baca7    48894e48                 mov              qword ptr [rsi + 0x48], rcx
    0x1405bacab    488916                   mov              qword ptr [rsi], rdx
    0x1405bacae    48c746200f000000         mov              qword ptr [rsi + 0x20], 0xf
    0x1405bacb6    48894628                 mov              qword ptr [rsi + 0x28], rax
    0x1405bacba    48c746300f000000         mov              qword ptr [rsi + 0x30], 0xf
    0x1405bacc2    488bbd68030000           mov              rdi, qword ptr [rbp + 0x368]
    0x1405bacc9    4889f9                   mov              rcx, rdi
    0x1405baccc    e83f716c00               call             0x140c81e10 -> parking_lot_core::parking_lot::deadlock_impl::release_resource::h1c3ab4db915ad6f2
    0x1405bacd1    31c9                     xor              ecx, ecx
    0x1405bacd3    b001                     mov              al, 1
    0x1405bacd5    f00fb00f                 lock cmpxchg     byte ptr [rdi], cl
    0x1405bacd9    7459                     je               0x1405bad34
    0x1405bacdb    4889f9                   mov              rcx, rdi
    0x1405bacde    31d2                     xor              edx, edx
    0x1405bace0    e8ab2e3102               call             0x1428cdb90 -> parking_lot::raw_mutex::RawMutex::unlock_slow::h56cb6df3e32a2657
    0x1405bace5    eb4d                     jmp              0x1405bad34
    0x1405bace7    488b8db8020000           mov              rcx, qword ptr [rbp + 0x2b8]
    0x1405bacee    48894e48                 mov              qword ptr [rsi + 0x48], rcx
    0x1405bacf2    0f108578020000           movups           xmm0, xmmword ptr [rbp + 0x278]
    0x1405bacf9    0f108d88020000           movups           xmm1, xmmword ptr [rbp + 0x288]
    0x1405bad00    0f109598020000           movups           xmm2, xmmword ptr [rbp + 0x298]
    0x1405bad07    0f109da8020000           movups           xmm3, xmmword ptr [rbp + 0x2a8]
    0x1405bad0e    0f115e38                 movups           xmmword ptr [rsi + 0x38], xmm3
    0x1405bad12    0f115628                 movups           xmmword ptr [rsi + 0x28], xmm2
    0x1405bad16    0f114e18                 movups           xmmword ptr [rsi + 0x18], xmm1
    0x1405bad1a    0f114608                 movups           xmmword ptr [rsi + 8], xmm0
    0x1405bad1e    488906                   mov              qword ptr [rsi], rax
    0x1405bad21    c6858f03000000           mov              byte ptr [rbp + 0x38f], 0
    0x1405bad28    488d8d38030000           lea              rcx, [rbp + 0x338]
    0x1405bad2f    e87c776200               call             0x140be24b0 -> rhapsody::raw::managed::handle::impl$6::drop
    0x1405bad34    4889f0                   mov              rax, rsi
    0x1405bad37    4881c418040000           add              rsp, 0x418
    0x1405bad3e    5b                       pop              rbx
    0x1405bad3f    5f                       pop              rdi
    0x1405bad40    5e                       pop              rsi
    0x1405bad41    415c                     pop              r12
    0x1405bad43    415d                     pop              r13
    0x1405bad45    415e                     pop              r14
    0x1405bad47    415f                     pop              r15
    0x1405bad49    5d                       pop              rbp
    0x1405bad4a    c3                       ret              
    0x1405bad4b    488d0d709b4402           lea              rcx, [rip + 0x2449b70] -> 0x142a048c2
    0x1405bad52    4c8d05a79b4402           lea              r8, [rip + 0x2449ba7] -> 0x142a04900
    0x1405bad59    ba3c000000               mov              edx, 0x3c
    0x1405bad5e    e8cde63102               call             0x1428d9430 -> core::panicking::panic::hb210ae7b817a9f13
    0x1405bad63    488d0dae9b4402           lea              rcx, [rip + 0x2449bae] -> 0x142a04918
    0x1405bad6a    4c8d05d79b4402           lea              r8, [rip + 0x2449bd7] -> 0x142a04948
    0x1405bad71    ba2e000000               mov              edx, 0x2e
    0x1405bad76    e8b5e63102               call             0x1428d9430 -> core::panicking::panic::hb210ae7b817a9f13
    0x1405bad7b    488d0dde9b4402           lea              rcx, [rip + 0x2449bde] -> 0x142a04960
    0x1405bad82    4c8d05079c4402           lea              r8, [rip + 0x2449c07] -> 0x142a04990
    0x1405bad89    ba2f000000               mov              edx, 0x2f
    0x1405bad8e    e89de63102               call             0x1428d9430 -> core::panicking::panic::hb210ae7b817a9f13
    0x1405bad93    488d0d0e9c4402           lea              rcx, [rip + 0x2449c0e] -> 0x142a049a8
    0x1405bad9a    4c8d05379c4402           lea              r8, [rip + 0x2449c37] -> 0x142a049d8
    0x1405bada1    ba2e000000               mov              edx, 0x2e
    0x1405bada6    e885e63102               call             0x1428d9430 -> core::panicking::panic::hb210ae7b817a9f13
    0x1405badab    4c89f1                   mov              rcx, r14
    0x1405badae    41b800ca9a3b             mov              r8d, 0x3b9aca00
    0x1405badb4    e857243102               call             0x1428cd210 -> parking_lot::raw_mutex::RawMutex::lock_slow::h7c244b522f551b38
    0x1405badb9    e9f6f9ffff               jmp              0x1405ba7b4
    0x1405badbe    c6858f03000001           mov              byte ptr [rbp + 0x38f], 1
    0x1405badc5    4889f9                   mov              rcx, rdi
    0x1405badc8    31d2                     xor              edx, edx
    0x1405badca    e8c12d3102               call             0x1428cdb90 -> parking_lot::raw_mutex::RawMutex::unlock_slow::h56cb6df3e32a2657
    0x1405badcf    e9cbfbffff               jmp              0x1405ba99f
    0x1405badd4    898570020000             mov              dword ptr [rbp + 0x270], eax
    0x1405badda    488d051f9c4402           lea              rax, [rip + 0x2449c1f] -> 0x142a04a00
    0x1405bade1    4889442420               mov              qword ptr [rsp + 0x20], rax
    0x1405bade6    488d0d039c4402           lea              rcx, [rip + 0x2449c03] -> 0x142a049f0
    0x1405baded    4c8d0df4734402           lea              r9, [rip + 0x24473f4] -> 0x142a021e8
    0x1405badf4    4c8d8570020000           lea              r8, [rbp + 0x270]
    0x1405badfb    ba0c000000               mov              edx, 0xc
    0x1405bae00    e85be93102               call             0x1428d9760 -> core::result::unwrap_failed::h96d74ae09566f4b3
    0x1405bae05    898570020000             mov              dword ptr [rbp + 0x270], eax
    0x1405bae0b    c6858f03000001           mov              byte ptr [rbp + 0x38f], 1
    0x1405bae12    488d05679c4402           lea              rax, [rip + 0x2449c67] -> 0x142a04a80
    0x1405bae19    4889442420               mov              qword ptr [rsp + 0x20], rax
    0x1405bae1e    488d0d439c4402           lea              rcx, [rip + 0x2449c43] -> 0x142a04a68
    0x1405bae25    4c8d0dbc734402           lea              r9, [rip + 0x24473bc] -> 0x142a021e8
    0x1405bae2c    4c8d8570020000           lea              r8, [rbp + 0x270]
    0x1405bae33    ba11000000               mov              edx, 0x11
    0x1405bae38    e823e93102               call             0x1428d9760 -> core::result::unwrap_failed::h96d74ae09566f4b3
    0x1405bae3d    e9a7000000               jmp              0x1405baee9
    0x1405bae42    0f1085d0000000           movups           xmm0, xmmword ptr [rbp + 0xd0]
    0x1405bae49    0f108de0000000           movups           xmm1, xmmword ptr [rbp + 0xe0]
    0x1405bae50    0f298d80020000           movaps           xmmword ptr [rbp + 0x280], xmm1
    0x1405bae57    0f298570020000           movaps           xmmword ptr [rbp + 0x270], xmm0
    0x1405bae5e    488d05b39b4402           lea              rax, [rip + 0x2449bb3] -> 0x142a04a18
    0x1405bae65    4889442420               mov              qword ptr [rsp + 0x20], rax
    0x1405bae6a    488d0db7734402           lea              rcx, [rip + 0x24473b7] -> 0x142a02228
    0x1405bae71    4c8d0d90734402           lea              r9, [rip + 0x2447390] -> 0x142a02208
    0x1405bae78    4c8d8570020000           lea              r8, [rbp + 0x270]
    0x1405bae7f    ba2b000000               mov              edx, 0x2b
    0x1405bae84    e8d7e83102               call             0x1428d9760 -> core::result::unwrap_failed::h96d74ae09566f4b3
    0x1405bae89    eb5e                     jmp              0x1405baee9
    0x1405bae8b    4c89a580030000           mov              qword ptr [rbp + 0x380], r12
    0x1405bae92    4c89bd78030000           mov              qword ptr [rbp + 0x378], r15
    0x1405bae99    8985d0000000             mov              dword ptr [rbp + 0xd0], eax
    0x1405bae9f    488d05aa9b4402           lea              rax, [rip + 0x2449baa] -> 0x142a04a50
    0x1405baea6    4889442420               mov              qword ptr [rsp + 0x20], rax
    0x1405baeab    488d0d7e9b4402           lea              rcx, [rip + 0x2449b7e] -> 0x142a04a30
    0x1405baeb2    4c8d0d2f734402           lea              r9, [rip + 0x244732f] -> 0x142a021e8
    0x1405baeb9    4c8d85d0000000           lea              r8, [rbp + 0xd0]
    0x1405baec0    ba1b000000               mov              edx, 0x1b
    0x1405baec5    e896e83102               call             0x1428d9760 -> core::result::unwrap_failed::h96d74ae09566f4b3
    0x1405baeca    eb1d                     jmp              0x1405baee9
    0x1405baecc    4c89bd78030000           mov              qword ptr [rbp + 0x378], r15
    0x1405baed3    4c8d05ee734402           lea              r8, [rip + 0x24473ee] -> 0x142a022c8
    0x1405baeda    b901000000               mov              ecx, 1
    0x1405baedf    ba0f000000               mov              edx, 0xf
    0x1405baee4    e8fae03102               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x1405baee9    0f0b                     ud2              

=== 0x1408ed880 size=421  rhapsody::raw::error::impl$1::fmt
    ; Leaf/impl fully decoded from PE bytes (semantics mostly diagnostic/formatting); see insns.  [medium]
    0x1408ed880    4881ec88000000           sub              rsp, 0x88
    0x1408ed887    4889d0                   mov              rax, rdx
    0x1408ed88a    4c8b01                   mov              r8, qword ptr [rcx]
    0x1408ed88d    4d8d48f7                 lea              r9, [r8 - 9]
    0x1408ed891    4983c0f8                 add              r8, -8
    0x1408ed895    31d2                     xor              edx, edx
    0x1408ed897    4983f902                 cmp              r9, 2
    0x1408ed89b    490f42d0                 cmovb            rdx, r8
    0x1408ed89f    4885d2                   test             rdx, rdx
    0x1408ed8a2    0f8487000000             je               0x1408ed92f
    0x1408ed8a8    4883fa01                 cmp              rdx, 1
    0x1408ed8ac    0f8520010000             jne              0x1408ed9d2
    0x1408ed8b2    488d5120                 lea              rdx, [rcx + 0x20]
    0x1408ed8b6    4883c108                 add              rcx, 8
    0x1408ed8ba    48898c2480000000         mov              qword ptr [rsp + 0x80], rcx
    0x1408ed8c2    488d0dff931b02           lea              rcx, [rip + 0x21b93ff] -> 0x142aa6cc8
    0x1408ed8c9    48894c2450               mov              qword ptr [rsp + 0x50], rcx
    0x1408ed8ce    488d8c2480000000         lea              rcx, [rsp + 0x80]
    0x1408ed8d6    48894c2448               mov              qword ptr [rsp + 0x48], rcx
    0x1408ed8db    488d0d0f941b02           lea              rcx, [rip + 0x21b940f] -> 0x142aa6cf1
    0x1408ed8e2    48894c2438               mov              qword ptr [rsp + 0x38], rcx
    0x1408ed8e7    488d0dba931b02           lea              rcx, [rip + 0x21b93ba] -> 0x142aa6ca8
    0x1408ed8ee    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x1408ed8f3    4889542428               mov              qword ptr [rsp + 0x28], rdx
    0x1408ed8f8    48c744244005000000       mov              qword ptr [rsp + 0x40], 5
    0x1408ed901    48c744242003000000       mov              qword ptr [rsp + 0x20], 3
    0x1408ed90a    488d15d7931b02           lea              rdx, [rip + 0x21b93d7] -> 0x142aa6ce8
    0x1408ed911    4c8d0dd6931b02           lea              r9, [rip + 0x21b93d6] -> 0x142aa6cee
    0x1408ed918    41b806000000             mov              r8d, 6
    0x1408ed91e    4889c1                   mov              rcx, rax
    0x1408ed921    e87a934900               call             0x140d86ca0 -> core::fmt::Formatter::debug_struct_field2_finish::h35c7e6c960cc525a
    0x1408ed926    90                       nop              
    0x1408ed927    4881c488000000           add              rsp, 0x88
    0x1408ed92e    c3                       ret              
    0x1408ed92f    488d5120                 lea              rdx, [rcx + 0x20]
    0x1408ed933    4c8d4138                 lea              r8, [rcx + 0x38]
    0x1408ed937    4c89842480000000         mov              qword ptr [rsp + 0x80], r8
    0x1408ed93f    4c8d0522931b02           lea              r8, [rip + 0x21b9322] -> 0x142aa6c68
    0x1408ed946    4c89442470               mov              qword ptr [rsp + 0x70], r8
    0x1408ed94b    4c8d842480000000         lea              r8, [rsp + 0x80]
    0x1408ed953    4c89442468               mov              qword ptr [rsp + 0x68], r8
    0x1408ed958    4c8d053c931b02           lea              r8, [rip + 0x21b933c] -> 0x142aa6c9b
    0x1408ed95f    4c89442458               mov              qword ptr [rsp + 0x58], r8
    0x1408ed964    4c8d0515911b02           lea              r8, [rip + 0x21b9115] -> 0x142aa6a80
    0x1408ed96b    4c89442450               mov              qword ptr [rsp + 0x50], r8
    0x1408ed970    4889542448               mov              qword ptr [rsp + 0x48], rdx
    0x1408ed975    488d151b931b02           lea              rdx, [rip + 0x21b931b] -> 0x142aa6c97
    0x1408ed97c    4889542438               mov              qword ptr [rsp + 0x38], rdx
    0x1408ed981    488d15c0921b02           lea              rdx, [rip + 0x21b92c0] -> 0x142aa6c48
    0x1408ed988    4889542430               mov              qword ptr [rsp + 0x30], rdx
    0x1408ed98d    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x1408ed992    48c74424600c000000       mov              qword ptr [rsp + 0x60], 0xc
    0x1408ed99b    48c744244004000000       mov              qword ptr [rsp + 0x40], 4
    0x1408ed9a4    48c744242005000000       mov              qword ptr [rsp + 0x20], 5
    0x1408ed9ad    488d15d4921b02           lea              rdx, [rip + 0x21b92d4] -> 0x142aa6c88
    0x1408ed9b4    4c8d0dd7921b02           lea              r9, [rip + 0x21b92d7] -> 0x142aa6c92
    0x1408ed9bb    41b80a000000             mov              r8d, 0xa
    0x1408ed9c1    4889c1                   mov              rcx, rax
    0x1408ed9c4    e8c7934900               call             0x140d86d90 -> core::fmt::Formatter::debug_struct_field3_finish::h024add6f4e520141
    0x1408ed9c9    90                       nop              
    0x1408ed9ca    4881c488000000           add              rsp, 0x88
    0x1408ed9d1    c3                       ret              
    0x1408ed9d2    4883c108                 add              rcx, 8
    0x1408ed9d6    48898c2480000000         mov              qword ptr [rsp + 0x80], rcx
    0x1408ed9de    488d0d13931b02           lea              rcx, [rip + 0x21b9313] -> 0x142aa6cf8
    0x1408ed9e5    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x1408ed9ea    488d8c2480000000         lea              rcx, [rsp + 0x80]
    0x1408ed9f2    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x1408ed9f7    48c744242004000000       mov              qword ptr [rsp + 0x20], 4
    0x1408eda00    488d1511931b02           lea              rdx, [rip + 0x21b9311] -> 0x142aa6d18
    0x1408eda07    4c8d0d18931b02           lea              r9, [rip + 0x21b9318] -> 0x142aa6d26
    0x1408eda0e    41b80e000000             mov              r8d, 0xe
    0x1408eda14    4889c1                   mov              rcx, rax
    0x1408eda17    e8d4914900               call             0x140d86bf0 -> core::fmt::Formatter::debug_struct_field1_finish::hd9497ec2d71b9977
    0x1408eda1c    90                       nop              
    0x1408eda1d    4881c488000000           add              rsp, 0x88
    0x1408eda24    c3                       ret              

=== 0x1408fca00 size=421  rhapsody::raw::error::impl$1::fmt
    ; Leaf/impl fully decoded from PE bytes (semantics mostly diagnostic/formatting); see insns.  [medium]
    0x1408fca00    4881ec88000000           sub              rsp, 0x88
    0x1408fca07    4889d0                   mov              rax, rdx
    0x1408fca0a    4c8b01                   mov              r8, qword ptr [rcx]
    0x1408fca0d    4d8d48f7                 lea              r9, [r8 - 9]
    0x1408fca11    4983c0f8                 add              r8, -8
    0x1408fca15    31d2                     xor              edx, edx
    0x1408fca17    4983f902                 cmp              r9, 2
    0x1408fca1b    490f42d0                 cmovb            rdx, r8
    0x1408fca1f    4885d2                   test             rdx, rdx
    0x1408fca22    0f8487000000             je               0x1408fcaaf
    0x1408fca28    4883fa01                 cmp              rdx, 1
    0x1408fca2c    0f8520010000             jne              0x1408fcb52
    0x1408fca32    488d5120                 lea              rdx, [rcx + 0x20]
    0x1408fca36    4883c108                 add              rcx, 8
    0x1408fca3a    48898c2480000000         mov              qword ptr [rsp + 0x80], rcx
    0x1408fca42    488d0d17ca1a02           lea              rcx, [rip + 0x21aca17] -> 0x142aa9460
    0x1408fca49    48894c2450               mov              qword ptr [rsp + 0x50], rcx
    0x1408fca4e    488d8c2480000000         lea              rcx, [rsp + 0x80]
    0x1408fca56    48894c2448               mov              qword ptr [rsp + 0x48], rcx
    0x1408fca5b    488d0d27ca1a02           lea              rcx, [rip + 0x21aca27] -> 0x142aa9489
    0x1408fca62    48894c2438               mov              qword ptr [rsp + 0x38], rcx
    0x1408fca67    488d0dd2c91a02           lea              rcx, [rip + 0x21ac9d2] -> 0x142aa9440
    0x1408fca6e    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x1408fca73    4889542428               mov              qword ptr [rsp + 0x28], rdx
    0x1408fca78    48c744244005000000       mov              qword ptr [rsp + 0x40], 5
    0x1408fca81    48c744242003000000       mov              qword ptr [rsp + 0x20], 3
    0x1408fca8a    488d15efc91a02           lea              rdx, [rip + 0x21ac9ef] -> 0x142aa9480
    0x1408fca91    4c8d0deec91a02           lea              r9, [rip + 0x21ac9ee] -> 0x142aa9486
    0x1408fca98    41b806000000             mov              r8d, 6
    0x1408fca9e    4889c1                   mov              rcx, rax
    0x1408fcaa1    e8faa14800               call             0x140d86ca0 -> core::fmt::Formatter::debug_struct_field2_finish::h35c7e6c960cc525a
    0x1408fcaa6    90                       nop              
    0x1408fcaa7    4881c488000000           add              rsp, 0x88
    0x1408fcaae    c3                       ret              
    0x1408fcaaf    488d5120                 lea              rdx, [rcx + 0x20]
    0x1408fcab3    4c8d4138                 lea              r8, [rcx + 0x38]
    0x1408fcab7    4c89842480000000         mov              qword ptr [rsp + 0x80], r8
    0x1408fcabf    4c8d053ac91a02           lea              r8, [rip + 0x21ac93a] -> 0x142aa9400
    0x1408fcac6    4c89442470               mov              qword ptr [rsp + 0x70], r8
    0x1408fcacb    4c8d842480000000         lea              r8, [rsp + 0x80]
    0x1408fcad3    4c89442468               mov              qword ptr [rsp + 0x68], r8
    0x1408fcad8    4c8d0554c91a02           lea              r8, [rip + 0x21ac954] -> 0x142aa9433
    0x1408fcadf    4c89442458               mov              qword ptr [rsp + 0x58], r8
    0x1408fcae4    4c8d05f5c81a02           lea              r8, [rip + 0x21ac8f5] -> 0x142aa93e0
    0x1408fcaeb    4c89442450               mov              qword ptr [rsp + 0x50], r8
    0x1408fcaf0    4889542448               mov              qword ptr [rsp + 0x48], rdx
    0x1408fcaf5    488d1533c91a02           lea              rdx, [rip + 0x21ac933] -> 0x142aa942f
    0x1408fcafc    4889542438               mov              qword ptr [rsp + 0x38], rdx
    0x1408fcb01    488d15b8c81a02           lea              rdx, [rip + 0x21ac8b8] -> 0x142aa93c0
    0x1408fcb08    4889542430               mov              qword ptr [rsp + 0x30], rdx
    0x1408fcb0d    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x1408fcb12    48c74424600c000000       mov              qword ptr [rsp + 0x60], 0xc
    0x1408fcb1b    48c744244004000000       mov              qword ptr [rsp + 0x40], 4
    0x1408fcb24    48c744242005000000       mov              qword ptr [rsp + 0x20], 5
    0x1408fcb2d    488d15ecc81a02           lea              rdx, [rip + 0x21ac8ec] -> 0x142aa9420
    0x1408fcb34    4c8d0defc81a02           lea              r9, [rip + 0x21ac8ef] -> 0x142aa942a
    0x1408fcb3b    41b80a000000             mov              r8d, 0xa
    0x1408fcb41    4889c1                   mov              rcx, rax
    0x1408fcb44    e847a24800               call             0x140d86d90 -> core::fmt::Formatter::debug_struct_field3_finish::h024add6f4e520141
    0x1408fcb49    90                       nop              
    0x1408fcb4a    4881c488000000           add              rsp, 0x88
    0x1408fcb51    c3                       ret              
    0x1408fcb52    4883c108                 add              rcx, 8
    0x1408fcb56    48898c2480000000         mov              qword ptr [rsp + 0x80], rcx
    0x1408fcb5e    488d0d2bc91a02           lea              rcx, [rip + 0x21ac92b] -> 0x142aa9490
    0x1408fcb65    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x1408fcb6a    488d8c2480000000         lea              rcx, [rsp + 0x80]
    0x1408fcb72    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x1408fcb77    48c744242004000000       mov              qword ptr [rsp + 0x20], 4
    0x1408fcb80    488d1529c91a02           lea              rdx, [rip + 0x21ac929] -> 0x142aa94b0
    0x1408fcb87    4c8d0d30c91a02           lea              r9, [rip + 0x21ac930] -> 0x142aa94be
    0x1408fcb8e    41b80e000000             mov              r8d, 0xe
    0x1408fcb94    4889c1                   mov              rcx, rax
    0x1408fcb97    e854a04800               call             0x140d86bf0 -> core::fmt::Formatter::debug_struct_field1_finish::hd9497ec2d71b9977
    0x1408fcb9c    90                       nop              
    0x1408fcb9d    4881c488000000           add              rsp, 0x88
    0x1408fcba4    c3                       ret              

=== 0x1408fcbb0 size=39  rhapsody::raw::texture::impl$14::fmt
    ; Leaf/impl fully decoded from PE bytes (semantics mostly diagnostic/formatting); see insns.  [medium]
    0x1408fcbb0    4889d0                   mov              rax, rdx
    0x1408fcbb3    0fb609                   movzx            ecx, byte ptr [rcx]
    0x1408fcbb6    488d1583d51a02           lea              rdx, [rip + 0x21ad583] -> 0x142aaa140
    0x1408fcbbd    4c8b04ca                 mov              r8, qword ptr [rdx + rcx*8]
    0x1408fcbc1    4c8d0db0d51a02           lea              r9, [rip + 0x21ad5b0] -> 0x142aaa178
    0x1408fcbc8    49631489                 movsxd           rdx, dword ptr [r9 + rcx*4]
    0x1408fcbcc    4c01ca                   add              rdx, r9
    0x1408fcbcf    4889c1                   mov              rcx, rax
    0x1408fcbd2    e9b99f4800               jmp              0x140d86b90 -> _$LT$core..fmt..Formatter$u20$as$u20$core..fmt..Write$GT$::write_str::h5c3f9ed06cfb23bd

=== 0x1409046b0 size=421  rhapsody::raw::error::impl$1::fmt
    ; Leaf/impl fully decoded from PE bytes (semantics mostly diagnostic/formatting); see insns.  [medium]
    0x1409046b0    4881ec88000000           sub              rsp, 0x88
    0x1409046b7    4889d0                   mov              rax, rdx
    0x1409046ba    4c8b01                   mov              r8, qword ptr [rcx]
    0x1409046bd    4d8d48f7                 lea              r9, [r8 - 9]
    0x1409046c1    4983c0f8                 add              r8, -8
    0x1409046c5    31d2                     xor              edx, edx
    0x1409046c7    4983f902                 cmp              r9, 2
    0x1409046cb    490f42d0                 cmovb            rdx, r8
    0x1409046cf    4885d2                   test             rdx, rdx
    0x1409046d2    0f8487000000             je               0x14090475f
    0x1409046d8    4883fa01                 cmp              rdx, 1
    0x1409046dc    0f8520010000             jne              0x140904802
    0x1409046e2    488d5120                 lea              rdx, [rcx + 0x20]
    0x1409046e6    4883c108                 add              rcx, 8
    0x1409046ea    48898c2480000000         mov              qword ptr [rsp + 0x80], rcx
    0x1409046f2    488d0daf5c1a02           lea              rcx, [rip + 0x21a5caf] -> 0x142aaa3a8
    0x1409046f9    48894c2450               mov              qword ptr [rsp + 0x50], rcx
    0x1409046fe    488d8c2480000000         lea              rcx, [rsp + 0x80]
    0x140904706    48894c2448               mov              qword ptr [rsp + 0x48], rcx
    0x14090470b    488d0dbf5c1a02           lea              rcx, [rip + 0x21a5cbf] -> 0x142aaa3d1
    0x140904712    48894c2438               mov              qword ptr [rsp + 0x38], rcx
    0x140904717    488d0d6a5c1a02           lea              rcx, [rip + 0x21a5c6a] -> 0x142aaa388
    0x14090471e    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x140904723    4889542428               mov              qword ptr [rsp + 0x28], rdx
    0x140904728    48c744244005000000       mov              qword ptr [rsp + 0x40], 5
    0x140904731    48c744242003000000       mov              qword ptr [rsp + 0x20], 3
    0x14090473a    488d15875c1a02           lea              rdx, [rip + 0x21a5c87] -> 0x142aaa3c8
    0x140904741    4c8d0d865c1a02           lea              r9, [rip + 0x21a5c86] -> 0x142aaa3ce
    0x140904748    41b806000000             mov              r8d, 6
    0x14090474e    4889c1                   mov              rcx, rax
    0x140904751    e84a254800               call             0x140d86ca0 -> core::fmt::Formatter::debug_struct_field2_finish::h35c7e6c960cc525a
    0x140904756    90                       nop              
    0x140904757    4881c488000000           add              rsp, 0x88
    0x14090475e    c3                       ret              
    0x14090475f    488d5120                 lea              rdx, [rcx + 0x20]
    0x140904763    4c8d4138                 lea              r8, [rcx + 0x38]
    0x140904767    4c89842480000000         mov              qword ptr [rsp + 0x80], r8
    0x14090476f    4c8d05d25b1a02           lea              r8, [rip + 0x21a5bd2] -> 0x142aaa348
    0x140904776    4c89442470               mov              qword ptr [rsp + 0x70], r8
    0x14090477b    4c8d842480000000         lea              r8, [rsp + 0x80]
    0x140904783    4c89442468               mov              qword ptr [rsp + 0x68], r8
    0x140904788    4c8d05ec5b1a02           lea              r8, [rip + 0x21a5bec] -> 0x142aaa37b
    0x14090478f    4c89442458               mov              qword ptr [rsp + 0x58], r8
    0x140904794    4c8d058d5b1a02           lea              r8, [rip + 0x21a5b8d] -> 0x142aaa328
    0x14090479b    4c89442450               mov              qword ptr [rsp + 0x50], r8
    0x1409047a0    4889542448               mov              qword ptr [rsp + 0x48], rdx
    0x1409047a5    488d15cb5b1a02           lea              rdx, [rip + 0x21a5bcb] -> 0x142aaa377
    0x1409047ac    4889542438               mov              qword ptr [rsp + 0x38], rdx
    0x1409047b1    488d15505b1a02           lea              rdx, [rip + 0x21a5b50] -> 0x142aaa308
    0x1409047b8    4889542430               mov              qword ptr [rsp + 0x30], rdx
    0x1409047bd    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x1409047c2    48c74424600c000000       mov              qword ptr [rsp + 0x60], 0xc
    0x1409047cb    48c744244004000000       mov              qword ptr [rsp + 0x40], 4
    0x1409047d4    48c744242005000000       mov              qword ptr [rsp + 0x20], 5
    0x1409047dd    488d15845b1a02           lea              rdx, [rip + 0x21a5b84] -> 0x142aaa368
    0x1409047e4    4c8d0d875b1a02           lea              r9, [rip + 0x21a5b87] -> 0x142aaa372
    0x1409047eb    41b80a000000             mov              r8d, 0xa
    0x1409047f1    4889c1                   mov              rcx, rax
    0x1409047f4    e897254800               call             0x140d86d90 -> core::fmt::Formatter::debug_struct_field3_finish::h024add6f4e520141
    0x1409047f9    90                       nop              
    0x1409047fa    4881c488000000           add              rsp, 0x88
    0x140904801    c3                       ret              
    0x140904802    4883c108                 add              rcx, 8
    0x140904806    48898c2480000000         mov              qword ptr [rsp + 0x80], rcx
    0x14090480e    488d0dc35b1a02           lea              rcx, [rip + 0x21a5bc3] -> 0x142aaa3d8
    0x140904815    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x14090481a    488d8c2480000000         lea              rcx, [rsp + 0x80]
    0x140904822    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x140904827    48c744242004000000       mov              qword ptr [rsp + 0x20], 4
    0x140904830    488d15c15b1a02           lea              rdx, [rip + 0x21a5bc1] -> 0x142aaa3f8
    0x140904837    4c8d0dc85b1a02           lea              r9, [rip + 0x21a5bc8] -> 0x142aaa406
    0x14090483e    41b80e000000             mov              r8d, 0xe
    0x140904844    4889c1                   mov              rcx, rax
    0x140904847    e8a4234800               call             0x140d86bf0 -> core::fmt::Formatter::debug_struct_field1_finish::hd9497ec2d71b9977
    0x14090484c    90                       nop              
    0x14090484d    4881c488000000           add              rsp, 0x88
    0x140904854    c3                       ret              

=== 0x140916140 size=421  rhapsody::raw::error::impl$1::fmt
    ; Leaf/impl fully decoded from PE bytes (semantics mostly diagnostic/formatting); see insns.  [medium]
    0x140916140    4881ec88000000           sub              rsp, 0x88
    0x140916147    4889d0                   mov              rax, rdx
    0x14091614a    4c8b01                   mov              r8, qword ptr [rcx]
    0x14091614d    4d8d48f7                 lea              r9, [r8 - 9]
    0x140916151    4983c0f8                 add              r8, -8
    0x140916155    31d2                     xor              edx, edx
    0x140916157    4983f902                 cmp              r9, 2
    0x14091615b    490f42d0                 cmovb            rdx, r8
    0x14091615f    4885d2                   test             rdx, rdx
    0x140916162    0f8487000000             je               0x1409161ef
    0x140916168    4883fa01                 cmp              rdx, 1
    0x14091616c    0f8520010000             jne              0x140916292
    0x140916172    488d5120                 lea              rdx, [rcx + 0x20]
    0x140916176    4883c108                 add              rcx, 8
    0x14091617a    48898c2480000000         mov              qword ptr [rsp + 0x80], rcx
    0x140916182    488d0d5f671902           lea              rcx, [rip + 0x219675f] -> 0x142aac8e8
    0x140916189    48894c2450               mov              qword ptr [rsp + 0x50], rcx
    0x14091618e    488d8c2480000000         lea              rcx, [rsp + 0x80]
    0x140916196    48894c2448               mov              qword ptr [rsp + 0x48], rcx
    0x14091619b    488d0d6f671902           lea              rcx, [rip + 0x219676f] -> 0x142aac911
    0x1409161a2    48894c2438               mov              qword ptr [rsp + 0x38], rcx
    0x1409161a7    488d0d1a671902           lea              rcx, [rip + 0x219671a] -> 0x142aac8c8
    0x1409161ae    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x1409161b3    4889542428               mov              qword ptr [rsp + 0x28], rdx
    0x1409161b8    48c744244005000000       mov              qword ptr [rsp + 0x40], 5
    0x1409161c1    48c744242003000000       mov              qword ptr [rsp + 0x20], 3
    0x1409161ca    488d1537671902           lea              rdx, [rip + 0x2196737] -> 0x142aac908
    0x1409161d1    4c8d0d36671902           lea              r9, [rip + 0x2196736] -> 0x142aac90e
    0x1409161d8    41b806000000             mov              r8d, 6
    0x1409161de    4889c1                   mov              rcx, rax
    0x1409161e1    e8ba0a4700               call             0x140d86ca0 -> core::fmt::Formatter::debug_struct_field2_finish::h35c7e6c960cc525a
    0x1409161e6    90                       nop              
    0x1409161e7    4881c488000000           add              rsp, 0x88
    0x1409161ee    c3                       ret              
    0x1409161ef    488d5120                 lea              rdx, [rcx + 0x20]
    0x1409161f3    4c8d4138                 lea              r8, [rcx + 0x38]
    0x1409161f7    4c89842480000000         mov              qword ptr [rsp + 0x80], r8
    0x1409161ff    4c8d0582661902           lea              r8, [rip + 0x2196682] -> 0x142aac888
    0x140916206    4c89442470               mov              qword ptr [rsp + 0x70], r8
    0x14091620b    4c8d842480000000         lea              r8, [rsp + 0x80]
    0x140916213    4c89442468               mov              qword ptr [rsp + 0x68], r8
    0x140916218    4c8d059c661902           lea              r8, [rip + 0x219669c] -> 0x142aac8bb
    0x14091621f    4c89442458               mov              qword ptr [rsp + 0x58], r8
    0x140916224    4c8d0505621902           lea              r8, [rip + 0x2196205] -> 0x142aac430
    0x14091622b    4c89442450               mov              qword ptr [rsp + 0x50], r8
    0x140916230    4889542448               mov              qword ptr [rsp + 0x48], rdx
    0x140916235    488d157b661902           lea              rdx, [rip + 0x219667b] -> 0x142aac8b7
    0x14091623c    4889542438               mov              qword ptr [rsp + 0x38], rdx
    0x140916241    488d1520661902           lea              rdx, [rip + 0x2196620] -> 0x142aac868
    0x140916248    4889542430               mov              qword ptr [rsp + 0x30], rdx
    0x14091624d    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x140916252    48c74424600c000000       mov              qword ptr [rsp + 0x60], 0xc
    0x14091625b    48c744244004000000       mov              qword ptr [rsp + 0x40], 4
    0x140916264    48c744242005000000       mov              qword ptr [rsp + 0x20], 5
    0x14091626d    488d1534661902           lea              rdx, [rip + 0x2196634] -> 0x142aac8a8
    0x140916274    4c8d0d37661902           lea              r9, [rip + 0x2196637] -> 0x142aac8b2
    0x14091627b    41b80a000000             mov              r8d, 0xa
    0x140916281    4889c1                   mov              rcx, rax
    0x140916284    e8070b4700               call             0x140d86d90 -> core::fmt::Formatter::debug_struct_field3_finish::h024add6f4e520141
    0x140916289    90                       nop              
    0x14091628a    4881c488000000           add              rsp, 0x88
    0x140916291    c3                       ret              
    0x140916292    4883c108                 add              rcx, 8
    0x140916296    48898c2480000000         mov              qword ptr [rsp + 0x80], rcx
    0x14091629e    488d0d035e1902           lea              rcx, [rip + 0x2195e03] -> 0x142aac0a8
    0x1409162a5    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x1409162aa    488d8c2480000000         lea              rcx, [rsp + 0x80]
    0x1409162b2    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x1409162b7    48c744242004000000       mov              qword ptr [rsp + 0x20], 4
    0x1409162c0    488d154f661902           lea              rdx, [rip + 0x219664f] -> 0x142aac916
    0x1409162c7    4c8d0d56661902           lea              r9, [rip + 0x2196656] -> 0x142aac924
    0x1409162ce    41b80e000000             mov              r8d, 0xe
    0x1409162d4    4889c1                   mov              rcx, rax
    0x1409162d7    e814094700               call             0x140d86bf0 -> core::fmt::Formatter::debug_struct_field1_finish::hd9497ec2d71b9977
    0x1409162dc    90                       nop              
    0x1409162dd    4881c488000000           add              rsp, 0x88
    0x1409162e4    c3                       ret              

=== 0x140939790 size=421  rhapsody::raw::error::impl$1::fmt
    ; Leaf/impl fully decoded from PE bytes (semantics mostly diagnostic/formatting); see insns.  [medium]
    0x140939790    4881ec88000000           sub              rsp, 0x88
    0x140939797    4889d0                   mov              rax, rdx
    0x14093979a    4c8b01                   mov              r8, qword ptr [rcx]
    0x14093979d    4d8d48f7                 lea              r9, [r8 - 9]
    0x1409397a1    4983c0f8                 add              r8, -8
    0x1409397a5    31d2                     xor              edx, edx
    0x1409397a7    4983f902                 cmp              r9, 2
    0x1409397ab    490f42d0                 cmovb            rdx, r8
    0x1409397af    4885d2                   test             rdx, rdx
    0x1409397b2    0f8487000000             je               0x14093983f
    0x1409397b8    4883fa01                 cmp              rdx, 1
    0x1409397bc    0f8520010000             jne              0x1409398e2
    0x1409397c2    488d5120                 lea              rdx, [rcx + 0x20]
    0x1409397c6    4883c108                 add              rcx, 8
    0x1409397ca    48898c2480000000         mov              qword ptr [rsp + 0x80], rcx
    0x1409397d2    488d0de7801702           lea              rcx, [rip + 0x21780e7] -> 0x142ab18c0
    0x1409397d9    48894c2450               mov              qword ptr [rsp + 0x50], rcx
    0x1409397de    488d8c2480000000         lea              rcx, [rsp + 0x80]
    0x1409397e6    48894c2448               mov              qword ptr [rsp + 0x48], rcx
    0x1409397eb    488d0df7801702           lea              rcx, [rip + 0x21780f7] -> 0x142ab18e9
    0x1409397f2    48894c2438               mov              qword ptr [rsp + 0x38], rcx
    0x1409397f7    488d0da2801702           lea              rcx, [rip + 0x21780a2] -> 0x142ab18a0
    0x1409397fe    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x140939803    4889542428               mov              qword ptr [rsp + 0x28], rdx
    0x140939808    48c744244005000000       mov              qword ptr [rsp + 0x40], 5
    0x140939811    48c744242003000000       mov              qword ptr [rsp + 0x20], 3
    0x14093981a    488d15bf801702           lea              rdx, [rip + 0x21780bf] -> 0x142ab18e0
    0x140939821    4c8d0dbe801702           lea              r9, [rip + 0x21780be] -> 0x142ab18e6
    0x140939828    41b806000000             mov              r8d, 6
    0x14093982e    4889c1                   mov              rcx, rax
    0x140939831    e86ad44400               call             0x140d86ca0 -> core::fmt::Formatter::debug_struct_field2_finish::h35c7e6c960cc525a
    0x140939836    90                       nop              
    0x140939837    4881c488000000           add              rsp, 0x88
    0x14093983e    c3                       ret              
    0x14093983f    488d5120                 lea              rdx, [rcx + 0x20]
    0x140939843    4c8d4138                 lea              r8, [rcx + 0x38]
    0x140939847    4c89842480000000         mov              qword ptr [rsp + 0x80], r8
    0x14093984f    4c8d050a801702           lea              r8, [rip + 0x217800a] -> 0x142ab1860
    0x140939856    4c89442470               mov              qword ptr [rsp + 0x70], r8
    0x14093985b    4c8d842480000000         lea              r8, [rsp + 0x80]
    0x140939863    4c89442468               mov              qword ptr [rsp + 0x68], r8
    0x140939868    4c8d0524801702           lea              r8, [rip + 0x2178024] -> 0x142ab1893
    0x14093986f    4c89442458               mov              qword ptr [rsp + 0x58], r8
    0x140939874    4c8d05c57f1702           lea              r8, [rip + 0x2177fc5] -> 0x142ab1840
    0x14093987b    4c89442450               mov              qword ptr [rsp + 0x50], r8
    0x140939880    4889542448               mov              qword ptr [rsp + 0x48], rdx
    0x140939885    488d1503801702           lea              rdx, [rip + 0x2178003] -> 0x142ab188f
    0x14093988c    4889542438               mov              qword ptr [rsp + 0x38], rdx
    0x140939891    488d15887f1702           lea              rdx, [rip + 0x2177f88] -> 0x142ab1820
    0x140939898    4889542430               mov              qword ptr [rsp + 0x30], rdx
    0x14093989d    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x1409398a2    48c74424600c000000       mov              qword ptr [rsp + 0x60], 0xc
    0x1409398ab    48c744244004000000       mov              qword ptr [rsp + 0x40], 4
    0x1409398b4    48c744242005000000       mov              qword ptr [rsp + 0x20], 5
    0x1409398bd    488d15bc7f1702           lea              rdx, [rip + 0x2177fbc] -> 0x142ab1880
    0x1409398c4    4c8d0dbf7f1702           lea              r9, [rip + 0x2177fbf] -> 0x142ab188a
    0x1409398cb    41b80a000000             mov              r8d, 0xa
    0x1409398d1    4889c1                   mov              rcx, rax
    0x1409398d4    e8b7d44400               call             0x140d86d90 -> core::fmt::Formatter::debug_struct_field3_finish::h024add6f4e520141
    0x1409398d9    90                       nop              
    0x1409398da    4881c488000000           add              rsp, 0x88
    0x1409398e1    c3                       ret              
    0x1409398e2    4883c108                 add              rcx, 8
    0x1409398e6    48898c2480000000         mov              qword ptr [rsp + 0x80], rcx
    0x1409398ee    488d0d037f1702           lea              rcx, [rip + 0x2177f03] -> 0x142ab17f8
    0x1409398f5    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x1409398fa    488d8c2480000000         lea              rcx, [rsp + 0x80]
    0x140939902    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x140939907    48c744242004000000       mov              qword ptr [rsp + 0x20], 4
    0x140939910    488d15d77f1702           lea              rdx, [rip + 0x2177fd7] -> 0x142ab18ee
    0x140939917    4c8d0dde7f1702           lea              r9, [rip + 0x2177fde] -> 0x142ab18fc
    0x14093991e    41b80e000000             mov              r8d, 0xe
    0x140939924    4889c1                   mov              rcx, rax
    0x140939927    e8c4d24400               call             0x140d86bf0 -> core::fmt::Formatter::debug_struct_field1_finish::hd9497ec2d71b9977
    0x14093992c    90                       nop              
    0x14093992d    4881c488000000           add              rsp, 0x88
    0x140939934    c3                       ret              

=== 0x140a32480 size=421  rhapsody::raw::error::impl$1::fmt
    ; Leaf/impl fully decoded from PE bytes (semantics mostly diagnostic/formatting); see insns.  [medium]
    0x140a32480    4881ec88000000           sub              rsp, 0x88
    0x140a32487    4889d0                   mov              rax, rdx
    0x140a3248a    4c8b01                   mov              r8, qword ptr [rcx]
    0x140a3248d    4d8d48f7                 lea              r9, [r8 - 9]
    0x140a32491    4983c0f8                 add              r8, -8
    0x140a32495    31d2                     xor              edx, edx
    0x140a32497    4983f902                 cmp              r9, 2
    0x140a3249b    490f42d0                 cmovb            rdx, r8
    0x140a3249f    4885d2                   test             rdx, rdx
    0x140a324a2    0f8487000000             je               0x140a3252f
    0x140a324a8    4883fa01                 cmp              rdx, 1
    0x140a324ac    0f8520010000             jne              0x140a325d2
    0x140a324b2    488d5120                 lea              rdx, [rcx + 0x20]
    0x140a324b6    4883c108                 add              rcx, 8
    0x140a324ba    48898c2480000000         mov              qword ptr [rsp + 0x80], rcx
    0x140a324c2    488d0d279d0902           lea              rcx, [rip + 0x2099d27] -> 0x142acc1f0
    0x140a324c9    48894c2450               mov              qword ptr [rsp + 0x50], rcx
    0x140a324ce    488d8c2480000000         lea              rcx, [rsp + 0x80]
    0x140a324d6    48894c2448               mov              qword ptr [rsp + 0x48], rcx
    0x140a324db    488d0d379d0902           lea              rcx, [rip + 0x2099d37] -> 0x142acc219
    0x140a324e2    48894c2438               mov              qword ptr [rsp + 0x38], rcx
    0x140a324e7    488d0de29c0902           lea              rcx, [rip + 0x2099ce2] -> 0x142acc1d0
    0x140a324ee    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x140a324f3    4889542428               mov              qword ptr [rsp + 0x28], rdx
    0x140a324f8    48c744244005000000       mov              qword ptr [rsp + 0x40], 5
    0x140a32501    48c744242003000000       mov              qword ptr [rsp + 0x20], 3
    0x140a3250a    488d15ff9c0902           lea              rdx, [rip + 0x2099cff] -> 0x142acc210
    0x140a32511    4c8d0dfe9c0902           lea              r9, [rip + 0x2099cfe] -> 0x142acc216
    0x140a32518    41b806000000             mov              r8d, 6
    0x140a3251e    4889c1                   mov              rcx, rax
    0x140a32521    e87a473500               call             0x140d86ca0 -> core::fmt::Formatter::debug_struct_field2_finish::h35c7e6c960cc525a
    0x140a32526    90                       nop              
    0x140a32527    4881c488000000           add              rsp, 0x88
    0x140a3252e    c3                       ret              
    0x140a3252f    488d5120                 lea              rdx, [rcx + 0x20]
    0x140a32533    4c8d4138                 lea              r8, [rcx + 0x38]
    0x140a32537    4c89842480000000         mov              qword ptr [rsp + 0x80], r8
    0x140a3253f    4c8d054a9c0902           lea              r8, [rip + 0x2099c4a] -> 0x142acc190
    0x140a32546    4c89442470               mov              qword ptr [rsp + 0x70], r8
    0x140a3254b    4c8d842480000000         lea              r8, [rsp + 0x80]
    0x140a32553    4c89442468               mov              qword ptr [rsp + 0x68], r8
    0x140a32558    4c8d05649c0902           lea              r8, [rip + 0x2099c64] -> 0x142acc1c3
    0x140a3255f    4c89442458               mov              qword ptr [rsp + 0x58], r8
    0x140a32564    4c8d05059c0902           lea              r8, [rip + 0x2099c05] -> 0x142acc170
    0x140a3256b    4c89442450               mov              qword ptr [rsp + 0x50], r8
    0x140a32570    4889542448               mov              qword ptr [rsp + 0x48], rdx
    0x140a32575    488d15439c0902           lea              rdx, [rip + 0x2099c43] -> 0x142acc1bf
    0x140a3257c    4889542438               mov              qword ptr [rsp + 0x38], rdx
    0x140a32581    488d15c89b0902           lea              rdx, [rip + 0x2099bc8] -> 0x142acc150
    0x140a32588    4889542430               mov              qword ptr [rsp + 0x30], rdx
    0x140a3258d    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x140a32592    48c74424600c000000       mov              qword ptr [rsp + 0x60], 0xc
    0x140a3259b    48c744244004000000       mov              qword ptr [rsp + 0x40], 4
    0x140a325a4    48c744242005000000       mov              qword ptr [rsp + 0x20], 5
    0x140a325ad    488d15fc9b0902           lea              rdx, [rip + 0x2099bfc] -> 0x142acc1b0
    0x140a325b4    4c8d0dff9b0902           lea              r9, [rip + 0x2099bff] -> 0x142acc1ba
    0x140a325bb    41b80a000000             mov              r8d, 0xa
    0x140a325c1    4889c1                   mov              rcx, rax
    0x140a325c4    e8c7473500               call             0x140d86d90 -> core::fmt::Formatter::debug_struct_field3_finish::h024add6f4e520141
    0x140a325c9    90                       nop              
    0x140a325ca    4881c488000000           add              rsp, 0x88
    0x140a325d1    c3                       ret              
    0x140a325d2    4883c108                 add              rcx, 8
    0x140a325d6    48898c2480000000         mov              qword ptr [rsp + 0x80], rcx
    0x140a325de    488d0d83930902           lea              rcx, [rip + 0x2099383] -> 0x142acb968
    0x140a325e5    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x140a325ea    488d8c2480000000         lea              rcx, [rsp + 0x80]
    0x140a325f2    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x140a325f7    48c744242004000000       mov              qword ptr [rsp + 0x20], 4
    0x140a32600    488d15179c0902           lea              rdx, [rip + 0x2099c17] -> 0x142acc21e
    0x140a32607    4c8d0d1e9c0902           lea              r9, [rip + 0x2099c1e] -> 0x142acc22c
    0x140a3260e    41b80e000000             mov              r8d, 0xe
    0x140a32614    4889c1                   mov              rcx, rax
    0x140a32617    e8d4453500               call             0x140d86bf0 -> core::fmt::Formatter::debug_struct_field1_finish::hd9497ec2d71b9977
    0x140a3261c    90                       nop              
    0x140a3261d    4881c488000000           add              rsp, 0x88
    0x140a32624    c3                       ret              

=== 0x140a532d0 size=421  rhapsody::raw::error::impl$1::fmt
    ; Leaf/impl fully decoded from PE bytes (semantics mostly diagnostic/formatting); see insns.  [medium]
    0x140a532d0    4881ec88000000           sub              rsp, 0x88
    0x140a532d7    4889d0                   mov              rax, rdx
    0x140a532da    4c8b01                   mov              r8, qword ptr [rcx]
    0x140a532dd    4d8d48f7                 lea              r9, [r8 - 9]
    0x140a532e1    4983c0f8                 add              r8, -8
    0x140a532e5    31d2                     xor              edx, edx
    0x140a532e7    4983f902                 cmp              r9, 2
    0x140a532eb    490f42d0                 cmovb            rdx, r8
    0x140a532ef    4885d2                   test             rdx, rdx
    0x140a532f2    0f8487000000             je               0x140a5337f
    0x140a532f8    4883fa01                 cmp              rdx, 1
    0x140a532fc    0f8520010000             jne              0x140a53422
    0x140a53302    488d5120                 lea              rdx, [rcx + 0x20]
    0x140a53306    4883c108                 add              rcx, 8
    0x140a5330a    48898c2480000000         mov              qword ptr [rsp + 0x80], rcx
    0x140a53312    488d0d47f00702           lea              rcx, [rip + 0x207f047] -> 0x142ad2360
    0x140a53319    48894c2450               mov              qword ptr [rsp + 0x50], rcx
    0x140a5331e    488d8c2480000000         lea              rcx, [rsp + 0x80]
    0x140a53326    48894c2448               mov              qword ptr [rsp + 0x48], rcx
    0x140a5332b    488d0d57f00702           lea              rcx, [rip + 0x207f057] -> 0x142ad2389
    0x140a53332    48894c2438               mov              qword ptr [rsp + 0x38], rcx
    0x140a53337    488d0d02f00702           lea              rcx, [rip + 0x207f002] -> 0x142ad2340
    0x140a5333e    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x140a53343    4889542428               mov              qword ptr [rsp + 0x28], rdx
    0x140a53348    48c744244005000000       mov              qword ptr [rsp + 0x40], 5
    0x140a53351    48c744242003000000       mov              qword ptr [rsp + 0x20], 3
    0x140a5335a    488d151ff00702           lea              rdx, [rip + 0x207f01f] -> 0x142ad2380
    0x140a53361    4c8d0d1ef00702           lea              r9, [rip + 0x207f01e] -> 0x142ad2386
    0x140a53368    41b806000000             mov              r8d, 6
    0x140a5336e    4889c1                   mov              rcx, rax
    0x140a53371    e82a393300               call             0x140d86ca0 -> core::fmt::Formatter::debug_struct_field2_finish::h35c7e6c960cc525a
    0x140a53376    90                       nop              
    0x140a53377    4881c488000000           add              rsp, 0x88
    0x140a5337e    c3                       ret              
    0x140a5337f    488d5120                 lea              rdx, [rcx + 0x20]
    0x140a53383    4c8d4138                 lea              r8, [rcx + 0x38]
    0x140a53387    4c89842480000000         mov              qword ptr [rsp + 0x80], r8
    0x140a5338f    4c8d056aef0702           lea              r8, [rip + 0x207ef6a] -> 0x142ad2300
    0x140a53396    4c89442470               mov              qword ptr [rsp + 0x70], r8
    0x140a5339b    4c8d842480000000         lea              r8, [rsp + 0x80]
    0x140a533a3    4c89442468               mov              qword ptr [rsp + 0x68], r8
    0x140a533a8    4c8d0584ef0702           lea              r8, [rip + 0x207ef84] -> 0x142ad2333
    0x140a533af    4c89442458               mov              qword ptr [rsp + 0x58], r8
    0x140a533b4    4c8d05b5d20702           lea              r8, [rip + 0x207d2b5] -> 0x142ad0670
    0x140a533bb    4c89442450               mov              qword ptr [rsp + 0x50], r8
    0x140a533c0    4889542448               mov              qword ptr [rsp + 0x48], rdx
    0x140a533c5    488d1563ef0702           lea              rdx, [rip + 0x207ef63] -> 0x142ad232f
    0x140a533cc    4889542438               mov              qword ptr [rsp + 0x38], rdx
    0x140a533d1    488d1508ef0702           lea              rdx, [rip + 0x207ef08] -> 0x142ad22e0
    0x140a533d8    4889542430               mov              qword ptr [rsp + 0x30], rdx
    0x140a533dd    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x140a533e2    48c74424600c000000       mov              qword ptr [rsp + 0x60], 0xc
    0x140a533eb    48c744244004000000       mov              qword ptr [rsp + 0x40], 4
    0x140a533f4    48c744242005000000       mov              qword ptr [rsp + 0x20], 5
    0x140a533fd    488d151cef0702           lea              rdx, [rip + 0x207ef1c] -> 0x142ad2320
    0x140a53404    4c8d0d1fef0702           lea              r9, [rip + 0x207ef1f] -> 0x142ad232a
    0x140a5340b    41b80a000000             mov              r8d, 0xa
    0x140a53411    4889c1                   mov              rcx, rax
    0x140a53414    e877393300               call             0x140d86d90 -> core::fmt::Formatter::debug_struct_field3_finish::h024add6f4e520141
    0x140a53419    90                       nop              
    0x140a5341a    4881c488000000           add              rsp, 0x88
    0x140a53421    c3                       ret              
    0x140a53422    4883c108                 add              rcx, 8
    0x140a53426    48898c2480000000         mov              qword ptr [rsp + 0x80], rcx
    0x140a5342e    488d0d7bd20702           lea              rcx, [rip + 0x207d27b] -> 0x142ad06b0
    0x140a53435    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x140a5343a    488d8c2480000000         lea              rcx, [rsp + 0x80]
    0x140a53442    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x140a53447    48c744242004000000       mov              qword ptr [rsp + 0x20], 4
    0x140a53450    488d1537ef0702           lea              rdx, [rip + 0x207ef37] -> 0x142ad238e
    0x140a53457    4c8d0d3eef0702           lea              r9, [rip + 0x207ef3e] -> 0x142ad239c
    0x140a5345e    41b80e000000             mov              r8d, 0xe
    0x140a53464    4889c1                   mov              rcx, rax
    0x140a53467    e884373300               call             0x140d86bf0 -> core::fmt::Formatter::debug_struct_field1_finish::hd9497ec2d71b9977
    0x140a5346c    90                       nop              
    0x140a5346d    4881c488000000           add              rsp, 0x88
    0x140a53474    c3                       ret              

=== 0x140a96e10 size=421  rhapsody::raw::error::impl$1::fmt
    ; Leaf/impl fully decoded from PE bytes (semantics mostly diagnostic/formatting); see insns.  [medium]
    0x140a96e10    4881ec88000000           sub              rsp, 0x88
    0x140a96e17    4889d0                   mov              rax, rdx
    0x140a96e1a    4c8b01                   mov              r8, qword ptr [rcx]
    0x140a96e1d    4d8d48f7                 lea              r9, [r8 - 9]
    0x140a96e21    4983c0f8                 add              r8, -8
    0x140a96e25    31d2                     xor              edx, edx
    0x140a96e27    4983f902                 cmp              r9, 2
    0x140a96e2b    490f42d0                 cmovb            rdx, r8
    0x140a96e2f    4885d2                   test             rdx, rdx
    0x140a96e32    0f8487000000             je               0x140a96ebf
    0x140a96e38    4883fa01                 cmp              rdx, 1
    0x140a96e3c    0f8520010000             jne              0x140a96f62
    0x140a96e42    488d5120                 lea              rdx, [rcx + 0x20]
    0x140a96e46    4883c108                 add              rcx, 8
    0x140a96e4a    48898c2480000000         mov              qword ptr [rsp + 0x80], rcx
    0x140a96e52    488d0d8f3f0402           lea              rcx, [rip + 0x2043f8f] -> 0x142adade8
    0x140a96e59    48894c2450               mov              qword ptr [rsp + 0x50], rcx
    0x140a96e5e    488d8c2480000000         lea              rcx, [rsp + 0x80]
    0x140a96e66    48894c2448               mov              qword ptr [rsp + 0x48], rcx
    0x140a96e6b    488d0d9f3f0402           lea              rcx, [rip + 0x2043f9f] -> 0x142adae11
    0x140a96e72    48894c2438               mov              qword ptr [rsp + 0x38], rcx
    0x140a96e77    488d0d4a3f0402           lea              rcx, [rip + 0x2043f4a] -> 0x142adadc8
    0x140a96e7e    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x140a96e83    4889542428               mov              qword ptr [rsp + 0x28], rdx
    0x140a96e88    48c744244005000000       mov              qword ptr [rsp + 0x40], 5
    0x140a96e91    48c744242003000000       mov              qword ptr [rsp + 0x20], 3
    0x140a96e9a    488d15673f0402           lea              rdx, [rip + 0x2043f67] -> 0x142adae08
    0x140a96ea1    4c8d0d663f0402           lea              r9, [rip + 0x2043f66] -> 0x142adae0e
    0x140a96ea8    41b806000000             mov              r8d, 6
    0x140a96eae    4889c1                   mov              rcx, rax
    0x140a96eb1    e8eafd2e00               call             0x140d86ca0 -> core::fmt::Formatter::debug_struct_field2_finish::h35c7e6c960cc525a
    0x140a96eb6    90                       nop              
    0x140a96eb7    4881c488000000           add              rsp, 0x88
    0x140a96ebe    c3                       ret              
    0x140a96ebf    488d5120                 lea              rdx, [rcx + 0x20]
    0x140a96ec3    4c8d4138                 lea              r8, [rcx + 0x38]
    0x140a96ec7    4c89842480000000         mov              qword ptr [rsp + 0x80], r8
    0x140a96ecf    4c8d05b23e0402           lea              r8, [rip + 0x2043eb2] -> 0x142adad88
    0x140a96ed6    4c89442470               mov              qword ptr [rsp + 0x70], r8
    0x140a96edb    4c8d842480000000         lea              r8, [rsp + 0x80]
    0x140a96ee3    4c89442468               mov              qword ptr [rsp + 0x68], r8
    0x140a96ee8    4c8d05cc3e0402           lea              r8, [rip + 0x2043ecc] -> 0x142adadbb
    0x140a96eef    4c89442458               mov              qword ptr [rsp + 0x58], r8
    0x140a96ef4    4c8d056d3e0402           lea              r8, [rip + 0x2043e6d] -> 0x142adad68
    0x140a96efb    4c89442450               mov              qword ptr [rsp + 0x50], r8
    0x140a96f00    4889542448               mov              qword ptr [rsp + 0x48], rdx
    0x140a96f05    488d15ab3e0402           lea              rdx, [rip + 0x2043eab] -> 0x142adadb7
    0x140a96f0c    4889542438               mov              qword ptr [rsp + 0x38], rdx
    0x140a96f11    488d15303e0402           lea              rdx, [rip + 0x2043e30] -> 0x142adad48
    0x140a96f18    4889542430               mov              qword ptr [rsp + 0x30], rdx
    0x140a96f1d    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x140a96f22    48c74424600c000000       mov              qword ptr [rsp + 0x60], 0xc
    0x140a96f2b    48c744244004000000       mov              qword ptr [rsp + 0x40], 4
    0x140a96f34    48c744242005000000       mov              qword ptr [rsp + 0x20], 5
    0x140a96f3d    488d15643e0402           lea              rdx, [rip + 0x2043e64] -> 0x142adada8
    0x140a96f44    4c8d0d673e0402           lea              r9, [rip + 0x2043e67] -> 0x142adadb2
    0x140a96f4b    41b80a000000             mov              r8d, 0xa
    0x140a96f51    4889c1                   mov              rcx, rax
    0x140a96f54    e837fe2e00               call             0x140d86d90 -> core::fmt::Formatter::debug_struct_field3_finish::h024add6f4e520141
    0x140a96f59    90                       nop              
    0x140a96f5a    4881c488000000           add              rsp, 0x88
    0x140a96f61    c3                       ret              
    0x140a96f62    4883c108                 add              rcx, 8
    0x140a96f66    48898c2480000000         mov              qword ptr [rsp + 0x80], rcx
    0x140a96f6e    488d0da33e0402           lea              rcx, [rip + 0x2043ea3] -> 0x142adae18
    0x140a96f75    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x140a96f7a    488d8c2480000000         lea              rcx, [rsp + 0x80]
    0x140a96f82    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x140a96f87    48c744242004000000       mov              qword ptr [rsp + 0x20], 4
    0x140a96f90    488d15a13e0402           lea              rdx, [rip + 0x2043ea1] -> 0x142adae38
    0x140a96f97    4c8d0da83e0402           lea              r9, [rip + 0x2043ea8] -> 0x142adae46
    0x140a96f9e    41b80e000000             mov              r8d, 0xe
    0x140a96fa4    4889c1                   mov              rcx, rax
    0x140a96fa7    e844fc2e00               call             0x140d86bf0 -> core::fmt::Formatter::debug_struct_field1_finish::hd9497ec2d71b9977
    0x140a96fac    90                       nop              
    0x140a96fad    4881c488000000           add              rsp, 0x88
    0x140a96fb4    c3                       ret              

=== 0x140b00560 size=421  rhapsody::raw::error::impl$1::fmt
    ; Leaf/impl fully decoded from PE bytes (semantics mostly diagnostic/formatting); see insns.  [medium]
    0x140b00560    4881ec88000000           sub              rsp, 0x88
    0x140b00567    4889d0                   mov              rax, rdx
    0x140b0056a    4c8b01                   mov              r8, qword ptr [rcx]
    0x140b0056d    4d8d48f7                 lea              r9, [r8 - 9]
    0x140b00571    4983c0f8                 add              r8, -8
    0x140b00575    31d2                     xor              edx, edx
    0x140b00577    4983f902                 cmp              r9, 2
    0x140b0057b    490f42d0                 cmovb            rdx, r8
    0x140b0057f    4885d2                   test             rdx, rdx
    0x140b00582    0f8487000000             je               0x140b0060f
    0x140b00588    4883fa01                 cmp              rdx, 1
    0x140b0058c    0f8520010000             jne              0x140b006b2
    0x140b00592    488d5120                 lea              rdx, [rcx + 0x20]
    0x140b00596    4883c108                 add              rcx, 8
    0x140b0059a    48898c2480000000         mov              qword ptr [rsp + 0x80], rcx
    0x140b005a2    488d0d8764fe01           lea              rcx, [rip + 0x1fe6487] -> 0x142ae6a30
    0x140b005a9    48894c2450               mov              qword ptr [rsp + 0x50], rcx
    0x140b005ae    488d8c2480000000         lea              rcx, [rsp + 0x80]
    0x140b005b6    48894c2448               mov              qword ptr [rsp + 0x48], rcx
    0x140b005bb    488d0d9764fe01           lea              rcx, [rip + 0x1fe6497] -> 0x142ae6a59
    0x140b005c2    48894c2438               mov              qword ptr [rsp + 0x38], rcx
    0x140b005c7    488d0d4264fe01           lea              rcx, [rip + 0x1fe6442] -> 0x142ae6a10
    0x140b005ce    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x140b005d3    4889542428               mov              qword ptr [rsp + 0x28], rdx
    0x140b005d8    48c744244005000000       mov              qword ptr [rsp + 0x40], 5
    0x140b005e1    48c744242003000000       mov              qword ptr [rsp + 0x20], 3
    0x140b005ea    488d155f64fe01           lea              rdx, [rip + 0x1fe645f] -> 0x142ae6a50
    0x140b005f1    4c8d0d5e64fe01           lea              r9, [rip + 0x1fe645e] -> 0x142ae6a56
    0x140b005f8    41b806000000             mov              r8d, 6
    0x140b005fe    4889c1                   mov              rcx, rax
    0x140b00601    e89a662800               call             0x140d86ca0 -> core::fmt::Formatter::debug_struct_field2_finish::h35c7e6c960cc525a
    0x140b00606    90                       nop              
    0x140b00607    4881c488000000           add              rsp, 0x88
    0x140b0060e    c3                       ret              
    0x140b0060f    488d5120                 lea              rdx, [rcx + 0x20]
    0x140b00613    4c8d4138                 lea              r8, [rcx + 0x38]
    0x140b00617    4c89842480000000         mov              qword ptr [rsp + 0x80], r8
    0x140b0061f    4c8d05aa63fe01           lea              r8, [rip + 0x1fe63aa] -> 0x142ae69d0
    0x140b00626    4c89442470               mov              qword ptr [rsp + 0x70], r8
    0x140b0062b    4c8d842480000000         lea              r8, [rsp + 0x80]
    0x140b00633    4c89442468               mov              qword ptr [rsp + 0x68], r8
    0x140b00638    4c8d05c463fe01           lea              r8, [rip + 0x1fe63c4] -> 0x142ae6a03
    0x140b0063f    4c89442458               mov              qword ptr [rsp + 0x58], r8
    0x140b00644    4c8d056563fe01           lea              r8, [rip + 0x1fe6365] -> 0x142ae69b0
    0x140b0064b    4c89442450               mov              qword ptr [rsp + 0x50], r8
    0x140b00650    4889542448               mov              qword ptr [rsp + 0x48], rdx
    0x140b00655    488d15a363fe01           lea              rdx, [rip + 0x1fe63a3] -> 0x142ae69ff
    0x140b0065c    4889542438               mov              qword ptr [rsp + 0x38], rdx
    0x140b00661    488d152863fe01           lea              rdx, [rip + 0x1fe6328] -> 0x142ae6990
    0x140b00668    4889542430               mov              qword ptr [rsp + 0x30], rdx
    0x140b0066d    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x140b00672    48c74424600c000000       mov              qword ptr [rsp + 0x60], 0xc
    0x140b0067b    48c744244004000000       mov              qword ptr [rsp + 0x40], 4
    0x140b00684    48c744242005000000       mov              qword ptr [rsp + 0x20], 5
    0x140b0068d    488d155c63fe01           lea              rdx, [rip + 0x1fe635c] -> 0x142ae69f0
    0x140b00694    4c8d0d5f63fe01           lea              r9, [rip + 0x1fe635f] -> 0x142ae69fa
    0x140b0069b    41b80a000000             mov              r8d, 0xa
    0x140b006a1    4889c1                   mov              rcx, rax
    0x140b006a4    e8e7662800               call             0x140d86d90 -> core::fmt::Formatter::debug_struct_field3_finish::h024add6f4e520141
    0x140b006a9    90                       nop              
    0x140b006aa    4881c488000000           add              rsp, 0x88
    0x140b006b1    c3                       ret              
    0x140b006b2    4883c108                 add              rcx, 8
    0x140b006b6    48898c2480000000         mov              qword ptr [rsp + 0x80], rcx
    0x140b006be    488d0d9b63fe01           lea              rcx, [rip + 0x1fe639b] -> 0x142ae6a60
    0x140b006c5    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x140b006ca    488d8c2480000000         lea              rcx, [rsp + 0x80]
    0x140b006d2    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x140b006d7    48c744242004000000       mov              qword ptr [rsp + 0x20], 4
    0x140b006e0    488d159963fe01           lea              rdx, [rip + 0x1fe6399] -> 0x142ae6a80
    0x140b006e7    4c8d0da063fe01           lea              r9, [rip + 0x1fe63a0] -> 0x142ae6a8e
    0x140b006ee    41b80e000000             mov              r8d, 0xe
    0x140b006f4    4889c1                   mov              rcx, rax
    0x140b006f7    e8f4642800               call             0x140d86bf0 -> core::fmt::Formatter::debug_struct_field1_finish::hd9497ec2d71b9977
    0x140b006fc    90                       nop              
    0x140b006fd    4881c488000000           add              rsp, 0x88
    0x140b00704    c3                       ret              

=== 0x140be2b40 size=11396  rhapsody::leek::leekspin_print_dump
    ; Large function body fully decoded (not a micro-leaf); decode present in insns.  [medium]
    0x140be2b40    55                       push             rbp
    0x140be2b41    4157                     push             r15
    0x140be2b43    4156                     push             r14
    0x140be2b45    4155                     push             r13
    0x140be2b47    4154                     push             r12
    0x140be2b49    56                       push             rsi
    0x140be2b4a    57                       push             rdi
    0x140be2b4b    53                       push             rbx
    0x140be2b4c    4881ec38030000           sub              rsp, 0x338
    0x140be2b53    488dac2480000000         lea              rbp, [rsp + 0x80]
    0x140be2b5b    48c785b0020000feffffff   mov              qword ptr [rbp + 0x2b0], 0xfffffffffffffffe
    0x140be2b66    488b7110                 mov              rsi, qword ptr [rcx + 0x10]
    0x140be2b6a    48c785f001000000000000   mov              qword ptr [rbp + 0x1f0], 0
    0x140be2b75    48c785f801000008000000   mov              qword ptr [rbp + 0x1f8], 8
    0x140be2b80    48c7850002000000000000   mov              qword ptr [rbp + 0x200], 0
    0x140be2b8b    48898de8010000           mov              qword ptr [rbp + 0x1e8], rcx
    0x140be2b92    488b5908                 mov              rbx, qword ptr [rcx + 8]
    0x140be2b96    4c8d352309f301           lea              r14, [rip + 0x1f30923] -> 0x142b134c0
    0x140be2b9d    4989f4                   mov              r12, rsi
    0x140be2ba0    eb3f                     jmp              0x140be2be1
    0x140be2ba2    66666666662e0f1f840000000000 nop              word ptr cs:[rax + rax]
    0x140be2bb0    488b85f8010000           mov              rax, qword ptr [rbp + 0x1f8]
    0x140be2bb7    488d0c7f                 lea              rcx, [rdi + rdi*2]
    0x140be2bbb    488b9580020000           mov              rdx, qword ptr [rbp + 0x280]
    0x140be2bc2    488914c8                 mov              qword ptr [rax + rcx*8], rdx
    0x140be2bc6    488b9578020000           mov              rdx, qword ptr [rbp + 0x278]
    0x140be2bcd    488954c808               mov              qword ptr [rax + rcx*8 + 8], rdx
    0x140be2bd2    4c896cc810               mov              qword ptr [rax + rcx*8 + 0x10], r13
    0x140be2bd7    48ffc7                   inc              rdi
    0x140be2bda    4889bd00020000           mov              qword ptr [rbp + 0x200], rdi
    0x140be2be1    4d85e4                   test             r12, r12
    0x140be2be4    0f8efc210000             jle              0x140be4de6
    0x140be2bea    498d7c24ff               lea              rdi, [r12 - 1]
    0x140be2bef    4839f7                   cmp              rdi, rsi
    0x140be2bf2    0f83a3230000             jae              0x140be4f9b
    0x140be2bf8    0fb6043b                 movzx            eax, byte ptr [rbx + rdi]
    0x140be2bfc    88850f020000             mov              byte ptr [rbp + 0x20f], al
    0x140be2c02    83f82e                   cmp              eax, 0x2e
    0x140be2c05    0f83f1210000             jae              0x140be4dfc
    0x140be2c0b    0fb60df71fa502           movzx            ecx, byte ptr [rip + 0x2a51ff7] -> 0x143634c09
    0x140be2c12    89c0                     mov              eax, eax
    0x140be2c14    49630486                 movsxd           rax, dword ptr [r14 + rax*4]
    0x140be2c18    4c01f0                   add              rax, r14
    0x140be2c1b    ffe0                     jmp              rax
    0x140be2c1d    b923000000               mov              ecx, 0x23
    0x140be2c22    ba01000000               mov              edx, 1
    0x140be2c27    e8e40fa3ff               call             0x140613c10 -> __rust_alloc
    0x140be2c2c    4885c0                   test             rax, rax
    0x140be2c2f    0f841c250000             je               0x140be5151
    0x140be2c35    0f10055c03f301           movups           xmm0, xmmword ptr [rip + 0x1f3035c] -> 0x142b12f98
    0x140be2c3c    0f114010                 movups           xmmword ptr [rax + 0x10], xmm0
    0x140be2c40    0f10054103f301           movups           xmm0, xmmword ptr [rip + 0x1f30341] -> 0x142b12f88
    0x140be2c47    0f1100                   movups           xmmword ptr [rax], xmm0
    0x140be2c4a    c7401f74616d70           mov              dword ptr [rax + 0x1f], 0x706d6174
    0x140be2c51    48c7859002000023000000   mov              qword ptr [rbp + 0x290], 0x23
    0x140be2c5c    48898598020000           mov              qword ptr [rbp + 0x298], rax
    0x140be2c63    48c785a002000023000000   mov              qword ptr [rbp + 0x2a0], 0x23
    0x140be2c6e    c685af02000001           mov              byte ptr [rbp + 0x2af], 1
    0x140be2c75    488d156c02f301           lea              rdx, [rip + 0x1f3026c] -> 0x142b12ee8
    0x140be2c7c    488dbd90020000           lea              rdi, [rbp + 0x290]
    0x140be2c83    4889f9                   mov              rcx, rdi
    0x140be2c86    e8e5681900               call             0x140d79570 -> alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::h526e3d19dbc6f270
    0x140be2c8b    488b8598020000           mov              rax, qword ptr [rbp + 0x298]
    0x140be2c92    c640233b                 mov              byte ptr [rax + 0x23], 0x3b
    0x140be2c96    48c785a002000024000000   mov              qword ptr [rbp + 0x2a0], 0x24
    0x140be2ca1    4d8d7c24f3               lea              r15, [r12 - 0xd]
    0x140be2ca6    4983fc0d                 cmp              r12, 0xd
    0x140be2caa    0f82171f0000             jb               0x140be4bc7
    0x140be2cb0    428b043b                 mov              eax, dword ptr [rbx + r15]
    0x140be2cb4    898510020000             mov              dword ptr [rbp + 0x210], eax
    0x140be2cba    4889bd58020000           mov              qword ptr [rbp + 0x258], rdi
    0x140be2cc1    488d05088e45ff           lea              rax, [rip - 0xba71f8] -> _$LT$alloc..borrow..Cow$LT$B$GT$$u20$as$u20$core..fmt..Display$GT$::fmt::h710fd6886fc306ed
    0x140be2cc8    48898560020000           mov              qword ptr [rbp + 0x260], rax
    0x140be2ccf    4c8dad10020000           lea              r13, [rbp + 0x210]
    0x140be2cd6    4c89ad68020000           mov              qword ptr [rbp + 0x268], r13
    0x140be2cdd    488d05ace544ff           lea              rax, [rip - 0xbb1a54] -> bevy_reflect::impls::std::_::_$LT$impl$u20$bevy_reflect..reflect..PartialReflect$u20$for$u20$u32$GT$::debug::hf2b8da78b8013204
    0x140be2ce4    48898570020000           mov              qword ptr [rbp + 0x270], rax
    0x140be2ceb    488d05ce02f301           lea              rax, [rip + 0x1f302ce] -> 0x142b12fc0
    0x140be2cf2    48898520020000           mov              qword ptr [rbp + 0x220], rax
    0x140be2cf9    48c7852802000002000000   mov              qword ptr [rbp + 0x228], 2
    0x140be2d04    4c8da558020000           lea              r12, [rbp + 0x258]
    0x140be2d0b    4c89a530020000           mov              qword ptr [rbp + 0x230], r12
    0x140be2d12    48c7853802000002000000   mov              qword ptr [rbp + 0x238], 2
    0x140be2d1d    48c7854002000000000000   mov              qword ptr [rbp + 0x240], 0
    0x140be2d28    c685af02000001           mov              byte ptr [rbp + 0x2af], 1
    0x140be2d2f    488d8db8010000           lea              rcx, [rbp + 0x1b8]
    0x140be2d36    488d9520020000           lea              rdx, [rbp + 0x220]
    0x140be2d3d    e84e721900               call             0x140d79f90 -> alloc::fmt::format::format_inner::hdcc110f646744b43
    0x140be2d42    488b9590020000           mov              rdx, qword ptr [rbp + 0x290]
    0x140be2d49    4885d2                   test             rdx, rdx
    0x140be2d4c    7412                     je               0x140be2d60
    0x140be2d4e    488b8d98020000           mov              rcx, qword ptr [rbp + 0x298]
    0x140be2d55    41b801000000             mov              r8d, 1
    0x140be2d5b    e8c00ea3ff               call             0x140613c20 -> __rust_dealloc
    0x140be2d60    488b85c8010000           mov              rax, qword ptr [rbp + 0x1c8]
    0x140be2d67    488985a0020000           mov              qword ptr [rbp + 0x2a0], rax
    0x140be2d6e    0f1085b8010000           movups           xmm0, xmmword ptr [rbp + 0x1b8]
    0x140be2d75    0f298590020000           movaps           xmmword ptr [rbp + 0x290], xmm0
    0x140be2d7c    4a8b443b04               mov              rax, qword ptr [rbx + r15 + 4]
    0x140be2d81    48898510020000           mov              qword ptr [rbp + 0x210], rax
    0x140be2d88    4889bd58020000           mov              qword ptr [rbp + 0x258], rdi
    0x140be2d8f    488d053a8d45ff           lea              rax, [rip - 0xba72c6] -> _$LT$alloc..borrow..Cow$LT$B$GT$$u20$as$u20$core..fmt..Display$GT$::fmt::h710fd6886fc306ed
    0x140be2d96    48898560020000           mov              qword ptr [rbp + 0x260], rax
    0x140be2d9d    4c89ad68020000           mov              qword ptr [rbp + 0x268], r13
    0x140be2da4    488d0515574eff           lea              rax, [rip - 0xb1a8eb] -> core::fmt::num::_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$::fmt::h8e88e8dbb6030fc5 (.llvm.4519547824212384754)
    0x140be2dab    48898570020000           mov              qword ptr [rbp + 0x270], rax
    0x140be2db2    488d053702f301           lea              rax, [rip + 0x1f30237] -> 0x142b12ff0
    0x140be2db9    48898520020000           mov              qword ptr [rbp + 0x220], rax
    0x140be2dc0    48c7852802000002000000   mov              qword ptr [rbp + 0x228], 2
    0x140be2dcb    4c89a530020000           mov              qword ptr [rbp + 0x230], r12
    0x140be2dd2    48c7853802000002000000   mov              qword ptr [rbp + 0x238], 2
    0x140be2ddd    48c7854002000000000000   mov              qword ptr [rbp + 0x240], 0
    0x140be2de8    c685af02000001           mov              byte ptr [rbp + 0x2af], 1
    0x140be2def    488dbd58010000           lea              rdi, [rbp + 0x158]
    0x140be2df6    488d9520020000           lea              rdx, [rbp + 0x220]
    0x140be2dfd    4889f9                   mov              rcx, rdi
    0x140be2e00    e88b711900               call             0x140d79f90 -> alloc::fmt::format::format_inner::hdcc110f646744b43
    0x140be2e05    e98a1d0000               jmp              0x140be4b94
    0x140be2e0a    41bd0b000000             mov              r13d, 0xb
    0x140be2e10    b90b000000               mov              ecx, 0xb
    0x140be2e15    ba01000000               mov              edx, 1
    0x140be2e1a    e8f10da3ff               call             0x140613c10 -> __rust_alloc
    0x140be2e1f    4885c0                   test             rax, rax
    0x140be2e22    0f8437260000             je               0x140be545f
    0x140be2e28    48b9656e645f72675f70     movabs           rcx, 0x705f67725f646e65
    0x140be2e32    488908                   mov              qword ptr [rax], rcx
    0x140be2e35    c7400770617373           mov              dword ptr [rax + 7], 0x73736170
    0x140be2e3c    48c785900200000b000000   mov              qword ptr [rbp + 0x290], 0xb
    0x140be2e47    48898578020000           mov              qword ptr [rbp + 0x278], rax
    0x140be2e4e    48898598020000           mov              qword ptr [rbp + 0x298], rax
    0x140be2e55    48c785a00200000b000000   mov              qword ptr [rbp + 0x2a0], 0xb
    0x140be2e60    b80b000000               mov              eax, 0xb
    0x140be2e65    e9461f0000               jmp              0x140be4db0
    0x140be2e6a    b90b000000               mov              ecx, 0xb
    0x140be2e6f    ba01000000               mov              edx, 1
    0x140be2e74    e8970da3ff               call             0x140613c10 -> __rust_alloc
    0x140be2e79    4885c0                   test             rax, rax
    0x140be2e7c    0f8437210000             je               0x140be4fb9
    0x140be2e82    48b970726573656e745f     movabs           rcx, 0x5f746e6573657270
    0x140be2e8c    488908                   mov              qword ptr [rax], rcx
    0x140be2e8f    c740075f657272           mov              dword ptr [rax + 7], 0x7272655f
    0x140be2e96    48c785900200000b000000   mov              qword ptr [rbp + 0x290], 0xb
    0x140be2ea1    48898598020000           mov              qword ptr [rbp + 0x298], rax
    0x140be2ea8    48c785a00200000b000000   mov              qword ptr [rbp + 0x2a0], 0xb
    0x140be2eb3    c685af02000001           mov              byte ptr [rbp + 0x2af], 1
    0x140be2eba    488d152700f301           lea              rdx, [rip + 0x1f30027] -> 0x142b12ee8
    0x140be2ec1    488dbd90020000           lea              rdi, [rbp + 0x290]
    0x140be2ec8    4889f9                   mov              rcx, rdi
    0x140be2ecb    e8a0661900               call             0x140d79570 -> alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::h526e3d19dbc6f270
    0x140be2ed0    488b8598020000           mov              rax, qword ptr [rbp + 0x298]
    0x140be2ed7    c6400b3b                 mov              byte ptr [rax + 0xb], 0x3b
    0x140be2edb    48c785a00200000c000000   mov              qword ptr [rbp + 0x2a0], 0xc
    0x140be2ee6    4d8d7c24fb               lea              r15, [r12 - 5]
    0x140be2eeb    4983fc05                 cmp              r12, 5
    0x140be2eef    0f82d21c0000             jb               0x140be4bc7
    0x140be2ef5    428b043b                 mov              eax, dword ptr [rbx + r15]
    0x140be2ef9    898510020000             mov              dword ptr [rbp + 0x210], eax
    0x140be2eff    4889bd58020000           mov              qword ptr [rbp + 0x258], rdi
    0x140be2f06    488d05c38b45ff           lea              rax, [rip - 0xba743d] -> _$LT$alloc..borrow..Cow$LT$B$GT$$u20$as$u20$core..fmt..Display$GT$::fmt::h710fd6886fc306ed
    0x140be2f0d    48898560020000           mov              qword ptr [rbp + 0x260], rax
    0x140be2f14    488d8510020000           lea              rax, [rbp + 0x210]
    0x140be2f1b    48898568020000           mov              qword ptr [rbp + 0x268], rax
    0x140be2f22    488d0527c60a00           lea              rax, [rip + 0xac627] -> _$LT$ash..vk..enums..Result$u20$as$u20$core..fmt..Debug$GT$::fmt::he38c28743ca3b6e6
    0x140be2f29    48898570020000           mov              qword ptr [rbp + 0x270], rax
    0x140be2f30    488d059104f301           lea              rax, [rip + 0x1f30491] -> 0x142b133c8
    0x140be2f37    48898520020000           mov              qword ptr [rbp + 0x220], rax
    0x140be2f3e    48c7852802000002000000   mov              qword ptr [rbp + 0x228], 2
    0x140be2f49    488d8558020000           lea              rax, [rbp + 0x258]
    0x140be2f50    48898530020000           mov              qword ptr [rbp + 0x230], rax
    0x140be2f57    48c7853802000002000000   mov              qword ptr [rbp + 0x238], 2
    0x140be2f62    48c7854002000000000000   mov              qword ptr [rbp + 0x240], 0
    0x140be2f6d    c685af02000001           mov              byte ptr [rbp + 0x2af], 1
    0x140be2f74    488d7dc0                 lea              rdi, [rbp - 0x40]
    0x140be2f78    488d9520020000           lea              rdx, [rbp + 0x220]
    0x140be2f7f    4889f9                   mov              rcx, rdi
    0x140be2f82    e809701900               call             0x140d79f90 -> alloc::fmt::format::format_inner::hdcc110f646744b43
    0x140be2f87    e9081c0000               jmp              0x140be4b94
    0x140be2f8c    41bd12000000             mov              r13d, 0x12
    0x140be2f92    b912000000               mov              ecx, 0x12
    0x140be2f97    ba01000000               mov              edx, 1
    0x140be2f9c    e86f0ca3ff               call             0x140613c10 -> __rust_alloc
    0x140be2fa1    4885c0                   test             rax, rax
    0x140be2fa4    0f84fd200000             je               0x140be50a7
    0x140be2faa    0f1005fc03f301           movups           xmm0, xmmword ptr [rip + 0x1f303fc] -> 0x142b133ad
    0x140be2fb1    0f1100                   movups           xmmword ptr [rax], xmm0
    0x140be2fb4    66c74010616c             mov              word ptr [rax + 0x10], 0x6c61
    0x140be2fba    e9a20f0000               jmp              0x140be3f61
    0x140be2fbf    41bd0f000000             mov              r13d, 0xf
    0x140be2fc5    b90f000000               mov              ecx, 0xf
    0x140be2fca    ba01000000               mov              edx, 1
    0x140be2fcf    e83c0ca3ff               call             0x140613c10 -> __rust_alloc
    0x140be2fd4    4885c0                   test             rax, rax
    0x140be2fd7    0f846d250000             je               0x140be554a
    0x140be2fdd    48b965656e5f6c6f7374     movabs           rcx, 0x74736f6c5f6e6565
    0x140be2fe7    48894807                 mov              qword ptr [rax + 7], rcx
    0x140be2feb    48b966756c6c73637265     movabs           rcx, 0x657263736c6c7566
    0x140be2ff5    e98a1d0000               jmp              0x140be4d84
    0x140be2ffa    41bd11000000             mov              r13d, 0x11
    0x140be3000    b911000000               mov              ecx, 0x11
    0x140be3005    ba01000000               mov              edx, 1
    0x140be300a    e8010ca3ff               call             0x140613c10 -> __rust_alloc
    0x140be300f    4885c0                   test             rax, rax
    0x140be3012    0f844b200000             je               0x140be5063
    0x140be3018    0f1005f102f301           movups           xmm0, xmmword ptr [rip + 0x1f302f1] -> 0x142b13310
    0x140be301f    0f1100                   movups           xmmword ptr [rax], xmm0
    0x140be3022    c6401065                 mov              byte ptr [rax + 0x10], 0x65
    0x140be3026    e9bd010000               jmp              0x140be31e8
    0x140be302b    41bd13000000             mov              r13d, 0x13
    0x140be3031    b913000000               mov              ecx, 0x13
    0x140be3036    ba01000000               mov              edx, 1
    0x140be303b    e8d00ba3ff               call             0x140613c10 -> __rust_alloc
    0x140be3040    4885c0                   test             rax, rax
    0x140be3043    0f847c240000             je               0x140be54c5
    0x140be3049    0f10054a03f301           movups           xmm0, xmmword ptr [rip + 0x1f3034a] -> 0x142b1339a
    0x140be3050    0f1100                   movups           xmmword ptr [rax], xmm0
    0x140be3053    c7400f64617465           mov              dword ptr [rax + 0xf], 0x65746164
    0x140be305a    e9891c0000               jmp              0x140be4ce8
    0x140be305f    b915000000               mov              ecx, 0x15
    0x140be3064    ba01000000               mov              edx, 1
    0x140be3069    e8a20ba3ff               call             0x140613c10 -> __rust_alloc
    0x140be306e    4885c0                   test             rax, rax
    0x140be3071    0f8452200000             je               0x140be50c9
    0x140be3077    0f1005ca01f301           movups           xmm0, xmmword ptr [rip + 0x1f301ca] -> 0x142b13248
    0x140be307e    0f1100                   movups           xmmword ptr [rax], xmm0
    0x140be3081    48b9746f725f706f6f6c     movabs           rcx, 0x6c6f6f705f726f74
    0x140be308b    4889480d                 mov              qword ptr [rax + 0xd], rcx
    0x140be308f    48c7859002000015000000   mov              qword ptr [rbp + 0x290], 0x15
    0x140be309a    48898598020000           mov              qword ptr [rbp + 0x298], rax
    0x140be30a1    48c785a002000015000000   mov              qword ptr [rbp + 0x2a0], 0x15
    0x140be30ac    c685af02000001           mov              byte ptr [rbp + 0x2af], 1
    0x140be30b3    488d152efef201           lea              rdx, [rip + 0x1f2fe2e] -> 0x142b12ee8
    0x140be30ba    488dbd90020000           lea              rdi, [rbp + 0x290]
    0x140be30c1    4889f9                   mov              rcx, rdi
    0x140be30c4    e8a7641900               call             0x140d79570 -> alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::h526e3d19dbc6f270
    0x140be30c9    488b8598020000           mov              rax, qword ptr [rbp + 0x298]
    0x140be30d0    c640153b                 mov              byte ptr [rax + 0x15], 0x3b
    0x140be30d4    48c785a002000016000000   mov              qword ptr [rbp + 0x2a0], 0x16
    0x140be30df    4d8d7c24fb               lea              r15, [r12 - 5]
    0x140be30e4    4983fc05                 cmp              r12, 5
    0x140be30e8    0f82d91a0000             jb               0x140be4bc7
    0x140be30ee    428b043b                 mov              eax, dword ptr [rbx + r15]
    0x140be30f2    898510020000             mov              dword ptr [rbp + 0x210], eax
    0x140be30f8    4889bd58020000           mov              qword ptr [rbp + 0x258], rdi
    0x140be30ff    488d05ca8945ff           lea              rax, [rip - 0xba7636] -> _$LT$alloc..borrow..Cow$LT$B$GT$$u20$as$u20$core..fmt..Display$GT$::fmt::h710fd6886fc306ed
    0x140be3106    48898560020000           mov              qword ptr [rbp + 0x260], rax
    0x140be310d    488d8510020000           lea              rax, [rbp + 0x210]
    0x140be3114    48898568020000           mov              qword ptr [rbp + 0x268], rax
    0x140be311b    488d056ef8ffff           lea              rax, [rip - 0x792] -> rhapsody::leek::impl$2::fmt
    0x140be3122    48898570020000           mov              qword ptr [rbp + 0x270], rax
    0x140be3129    488d05f800f301           lea              rax, [rip + 0x1f300f8] -> 0x142b13228
    0x140be3130    48898520020000           mov              qword ptr [rbp + 0x220], rax
    0x140be3137    48c7852802000002000000   mov              qword ptr [rbp + 0x228], 2
    0x140be3142    488d8558020000           lea              rax, [rbp + 0x258]
    0x140be3149    48898530020000           mov              qword ptr [rbp + 0x230], rax
    0x140be3150    48c7853802000002000000   mov              qword ptr [rbp + 0x238], 2
    0x140be315b    48c7854002000000000000   mov              qword ptr [rbp + 0x240], 0
    0x140be3166    c685af02000001           mov              byte ptr [rbp + 0x2af], 1
    0x140be316d    488d7d08                 lea              rdi, [rbp + 8]
    0x140be3171    488d9520020000           lea              rdx, [rbp + 0x220]
    0x140be3178    4889f9                   mov              rcx, rdi
    0x140be317b    e8106e1900               call             0x140d79f90 -> alloc::fmt::format::format_inner::hdcc110f646744b43
    0x140be3180    e90f1a0000               jmp              0x140be4b94
    0x140be3185    41bd15000000             mov              r13d, 0x15
    0x140be318b    b915000000               mov              ecx, 0x15
    0x140be3190    ba01000000               mov              edx, 1
    0x140be3195    e8760aa3ff               call             0x140613c10 -> __rust_alloc
    0x140be319a    4885c0                   test             rax, rax
    0x140be319d    0f84c6230000             je               0x140be5569
    0x140be31a3    0f1005be02f301           movups           xmm0, xmmword ptr [rip + 0x1f302be] -> 0x142b13468
    0x140be31aa    0f1100                   movups           xmmword ptr [rax], xmm0
    0x140be31ad    48b95f62617272696572     movabs           rcx, 0x726569727261625f
    0x140be31b7    e921100000               jmp              0x140be41dd
    0x140be31bc    41bd11000000             mov              r13d, 0x11
    0x140be31c2    b911000000               mov              ecx, 0x11
    0x140be31c7    ba01000000               mov              edx, 1
    0x140be31cc    e83f0aa3ff               call             0x140613c10 -> __rust_alloc
    0x140be31d1    4885c0                   test             rax, rax
    0x140be31d4    0f84231e0000             je               0x140be4ffd
    0x140be31da    0f10051a00f301           movups           xmm0, xmmword ptr [rip + 0x1f3001a] -> 0x142b131fb
    0x140be31e1    0f1100                   movups           xmmword ptr [rax], xmm0
    0x140be31e4    c640106e                 mov              byte ptr [rax + 0x10], 0x6e
    0x140be31e8    48c7859002000011000000   mov              qword ptr [rbp + 0x290], 0x11
    0x140be31f3    48898578020000           mov              qword ptr [rbp + 0x278], rax
    0x140be31fa    48898598020000           mov              qword ptr [rbp + 0x298], rax
    0x140be3201    48c785a002000011000000   mov              qword ptr [rbp + 0x2a0], 0x11
    0x140be320c    b811000000               mov              eax, 0x11
    0x140be3211    e99a1b0000               jmp              0x140be4db0
    0x140be3216    41bd0d000000             mov              r13d, 0xd
    0x140be321c    b90d000000               mov              ecx, 0xd
    0x140be3221    ba01000000               mov              edx, 1
    0x140be3226    e8e509a3ff               call             0x140613c10 -> __rust_alloc
    0x140be322b    4885c0                   test             rax, rax
    0x140be322e    0f84d7200000             je               0x140be530b
    0x140be3234    48b95f70726573656e74     movabs           rcx, 0x746e65736572705f
    0x140be323e    48894805                 mov              qword ptr [rax + 5], rcx
    0x140be3242    48b971756575655f7072     movabs           rcx, 0x72705f6575657571
    0x140be324c    e912160000               jmp              0x140be4863
    0x140be3251    b915000000               mov              ecx, 0x15
    0x140be3256    ba01000000               mov              edx, 1
    0x140be325b    e8b009a3ff               call             0x140613c10 -> __rust_alloc
    0x140be3260    4885c0                   test             rax, rax
    0x140be3263    0f84c4200000             je               0x140be532d
    0x140be3269    0f10054efff201           movups           xmm0, xmmword ptr [rip + 0x1f2ff4e] -> 0x142b131be
    0x140be3270    0f1100                   movups           xmmword ptr [rax], xmm0
    0x140be3273    48b9725f706173736573     movabs           rcx, 0x7365737361705f72
    0x140be327d    4889480d                 mov              qword ptr [rax + 0xd], rcx
    0x140be3281    48c7859002000015000000   mov              qword ptr [rbp + 0x290], 0x15
    0x140be328c    48898598020000           mov              qword ptr [rbp + 0x298], rax
    0x140be3293    48c785a002000015000000   mov              qword ptr [rbp + 0x2a0], 0x15
    0x140be329e    c685af02000001           mov              byte ptr [rbp + 0x2af], 1
    0x140be32a5    488d153cfcf201           lea              rdx, [rip + 0x1f2fc3c] -> 0x142b12ee8
    0x140be32ac    488dbd90020000           lea              rdi, [rbp + 0x290]
    0x140be32b3    4889f9                   mov              rcx, rdi
    0x140be32b6    e8b5621900               call             0x140d79570 -> alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::h526e3d19dbc6f270
    0x140be32bb    488b8598020000           mov              rax, qword ptr [rbp + 0x298]
    0x140be32c2    c640153b                 mov              byte ptr [rax + 0x15], 0x3b
    0x140be32c6    48c785a002000016000000   mov              qword ptr [rbp + 0x2a0], 0x16
    0x140be32d1    4d8d7c24fb               lea              r15, [r12 - 5]
    0x140be32d6    4983fc05                 cmp              r12, 5
    0x140be32da    0f82e7180000             jb               0x140be4bc7
    0x140be32e0    428b043b                 mov              eax, dword ptr [rbx + r15]
    0x140be32e4    898510020000             mov              dword ptr [rbp + 0x210], eax
    0x140be32ea    4889bd58020000           mov              qword ptr [rbp + 0x258], rdi
    0x140be32f1    488d05d88745ff           lea              rax, [rip - 0xba7828] -> _$LT$alloc..borrow..Cow$LT$B$GT$$u20$as$u20$core..fmt..Display$GT$::fmt::h710fd6886fc306ed
    0x140be32f8    48898560020000           mov              qword ptr [rbp + 0x260], rax
    0x140be32ff    488d8510020000           lea              rax, [rbp + 0x210]
    0x140be3306    48898568020000           mov              qword ptr [rbp + 0x268], rax
    0x140be330d    488d057cdf44ff           lea              rax, [rip - 0xbb2084] -> bevy_reflect::impls::std::_::_$LT$impl$u20$bevy_reflect..reflect..PartialReflect$u20$for$u20$u32$GT$::debug::hf2b8da78b8013204
    0x140be3314    48898570020000           mov              qword ptr [rbp + 0x270], rax
    0x140be331b    488d0556fef201           lea              rax, [rip + 0x1f2fe56] -> 0x142b13178
    0x140be3322    48898520020000           mov              qword ptr [rbp + 0x220], rax
    0x140be3329    48c7852802000002000000   mov              qword ptr [rbp + 0x228], 2
    0x140be3334    488d8558020000           lea              rax, [rbp + 0x258]
    0x140be333b    48898530020000           mov              qword ptr [rbp + 0x230], rax
    0x140be3342    48c7853802000002000000   mov              qword ptr [rbp + 0x238], 2
    0x140be334d    48c7854002000000000000   mov              qword ptr [rbp + 0x240], 0
    0x140be3358    c685af02000001           mov              byte ptr [rbp + 0x2af], 1
    0x140be335f    488d7d68                 lea              rdi, [rbp + 0x68]
    0x140be3363    488d9520020000           lea              rdx, [rbp + 0x220]
    0x140be336a    4889f9                   mov              rcx, rdi
    0x140be336d    e81e6c1900               call             0x140d79f90 -> alloc::fmt::format::format_inner::hdcc110f646744b43
    0x140be3372    e91d180000               jmp              0x140be4b94
    0x140be3377    b91a000000               mov              ecx, 0x1a
    0x140be337c    ba01000000               mov              edx, 1
    0x140be3381    e88a08a3ff               call             0x140613c10 -> __rust_alloc
    0x140be3386    4885c0                   test             rax, rax
    0x140be3389    0f844a1e0000             je               0x140be51d9
    0x140be338f    0f1005b4fdf201           movups           xmm0, xmmword ptr [rip + 0x1f2fdb4] -> 0x142b1314a
    0x140be3396    0f11400a                 movups           xmmword ptr [rax + 0xa], xmm0
    0x140be339a    0f10059ffdf201           movups           xmm0, xmmword ptr [rip + 0x1f2fd9f] -> 0x142b13140
    0x140be33a1    0f1100                   movups           xmmword ptr [rax], xmm0
    0x140be33a4    48c785900200001a000000   mov              qword ptr [rbp + 0x290], 0x1a
    0x140be33af    48898598020000           mov              qword ptr [rbp + 0x298], rax
    0x140be33b6    48c785a00200001a000000   mov              qword ptr [rbp + 0x2a0], 0x1a
    0x140be33c1    c685af02000001           mov              byte ptr [rbp + 0x2af], 1
    0x140be33c8    488d1519fbf201           lea              rdx, [rip + 0x1f2fb19] -> 0x142b12ee8
    0x140be33cf    488dbd90020000           lea              rdi, [rbp + 0x290]
    0x140be33d6    4889f9                   mov              rcx, rdi
    0x140be33d9    e892611900               call             0x140d79570 -> alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::h526e3d19dbc6f270
    0x140be33de    488b8598020000           mov              rax, qword ptr [rbp + 0x298]
    0x140be33e5    c6401a3b                 mov              byte ptr [rax + 0x1a], 0x3b
    0x140be33e9    48c785a00200001b000000   mov              qword ptr [rbp + 0x2a0], 0x1b
    0x140be33f4    4d8d7c24ef               lea              r15, [r12 - 0x11]
    0x140be33f9    4983fc11                 cmp              r12, 0x11
    0x140be33fd    0f82c4170000             jb               0x140be4bc7
    0x140be3403    420f10043b               movups           xmm0, xmmword ptr [rbx + r15]
    0x140be3408    0f298510020000           movaps           xmmword ptr [rbp + 0x210], xmm0
    0x140be340f    4889bd58020000           mov              qword ptr [rbp + 0x258], rdi
    0x140be3416    488d05b38645ff           lea              rax, [rip - 0xba794d] -> _$LT$alloc..borrow..Cow$LT$B$GT$$u20$as$u20$core..fmt..Display$GT$::fmt::h710fd6886fc306ed
    0x140be341d    48898560020000           mov              qword ptr [rbp + 0x260], rax
    0x140be3424    488d8510020000           lea              rax, [rbp + 0x210]
    0x140be342b    48898568020000           mov              qword ptr [rbp + 0x268], rax
    0x140be3432    488d05d7f2ffff           lea              rax, [rip - 0xd29] -> rhapsody::leek::impl$0::fmt
    0x140be3439    48898570020000           mov              qword ptr [rbp + 0x270], rax
    0x140be3440    488d05b1fcf201           lea              rax, [rip + 0x1f2fcb1] -> 0x142b130f8
    0x140be3447    48898520020000           mov              qword ptr [rbp + 0x220], rax
    0x140be344e    48c7852802000002000000   mov              qword ptr [rbp + 0x228], 2
    0x140be3459    488d8558020000           lea              rax, [rbp + 0x258]
    0x140be3460    48898530020000           mov              qword ptr [rbp + 0x230], rax
    0x140be3467    48c7853802000002000000   mov              qword ptr [rbp + 0x238], 2
    0x140be3472    48c7854002000000000000   mov              qword ptr [rbp + 0x240], 0
    0x140be347d    c685af02000001           mov              byte ptr [rbp + 0x2af], 1
    0x140be3484    488dbdf8000000           lea              rdi, [rbp + 0xf8]
    0x140be348b    488d9520020000           lea              rdx, [rbp + 0x220]
    0x140be3492    4889f9                   mov              rcx, rdi
    0x140be3495    e8f66a1900               call             0x140d79f90 -> alloc::fmt::format::format_inner::hdcc110f646744b43
    0x140be349a    e9f5160000               jmp              0x140be4b94
    0x140be349f    41bd1c000000             mov              r13d, 0x1c
    0x140be34a5    b91c000000               mov              ecx, 0x1c
    0x140be34aa    ba01000000               mov              edx, 1
    0x140be34af    e85c07a3ff               call             0x140613c10 -> __rust_alloc
    0x140be34b4    4885c0                   test             rax, rax
    0x140be34b7    0f842a200000             je               0x140be54e7
    0x140be34bd    0f100540fff201           movups           xmm0, xmmword ptr [rip + 0x1f2ff40] -> 0x142b13404
    0x140be34c4    0f11400c                 movups           xmmword ptr [rax + 0xc], xmm0
    0x140be34c8    0f100529fff201           movups           xmm0, xmmword ptr [rip + 0x1f2ff29] -> 0x142b133f8
    0x140be34cf    0f1100                   movups           xmmword ptr [rax], xmm0
    0x140be34d2    48c785900200001c000000   mov              qword ptr [rbp + 0x290], 0x1c
    0x140be34dd    48898578020000           mov              qword ptr [rbp + 0x278], rax
    0x140be34e4    48898598020000           mov              qword ptr [rbp + 0x298], rax
    0x140be34eb    48c785a00200001c000000   mov              qword ptr [rbp + 0x2a0], 0x1c
    0x140be34f6    b81c000000               mov              eax, 0x1c
    0x140be34fb    e9b0180000               jmp              0x140be4db0
    0x140be3500    b910000000               mov              ecx, 0x10
    0x140be3505    ba01000000               mov              edx, 1
    0x140be350a    e80107a3ff               call             0x140613c10 -> __rust_alloc
    0x140be350f    4885c0                   test             rax, rax
    0x140be3512    0f84071b0000             je               0x140be501f
    0x140be3518    0f1005b4fcf201           movups           xmm0, xmmword ptr [rip + 0x1f2fcb4] -> 0x142b131d3
    0x140be351f    0f1100                   movups           xmmword ptr [rax], xmm0
    0x140be3522    48c7859002000010000000   mov              qword ptr [rbp + 0x290], 0x10
    0x140be352d    48898598020000           mov              qword ptr [rbp + 0x298], rax
    0x140be3534    48c785a002000010000000   mov              qword ptr [rbp + 0x2a0], 0x10
    0x140be353f    c685af02000001           mov              byte ptr [rbp + 0x2af], 1
    0x140be3546    488d159bf9f201           lea              rdx, [rip + 0x1f2f99b] -> 0x142b12ee8
    0x140be354d    488dbd90020000           lea              rdi, [rbp + 0x290]
    0x140be3554    4889f9                   mov              rcx, rdi
    0x140be3557    e814601900               call             0x140d79570 -> alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::h526e3d19dbc6f270
    0x140be355c    488b8598020000           mov              rax, qword ptr [rbp + 0x298]
    0x140be3563    c640103b                 mov              byte ptr [rax + 0x10], 0x3b
    0x140be3567    48c785a002000011000000   mov              qword ptr [rbp + 0x2a0], 0x11
    0x140be3572    4d8d7c24fb               lea              r15, [r12 - 5]
    0x140be3577    4983fc05                 cmp              r12, 5
    0x140be357b    0f8246160000             jb               0x140be4bc7
    0x140be3581    428b043b                 mov              eax, dword ptr [rbx + r15]
    0x140be3585    898510020000             mov              dword ptr [rbp + 0x210], eax
    0x140be358b    4889bd58020000           mov              qword ptr [rbp + 0x258], rdi
    0x140be3592    488d05378545ff           lea              rax, [rip - 0xba7ac9] -> _$LT$alloc..borrow..Cow$LT$B$GT$$u20$as$u20$core..fmt..Display$GT$::fmt::h710fd6886fc306ed
    0x140be3599    48898560020000           mov              qword ptr [rbp + 0x260], rax
    0x140be35a0    488d8510020000           lea              rax, [rbp + 0x210]
    0x140be35a7    48898568020000           mov              qword ptr [rbp + 0x268], rax
    0x140be35ae    488d05dbdc44ff           lea              rax, [rip - 0xbb2325] -> bevy_reflect::impls::std::_::_$LT$impl$u20$bevy_reflect..reflect..PartialReflect$u20$for$u20$u32$GT$::debug::hf2b8da78b8013204
    0x140be35b5    48898570020000           mov              qword ptr [rbp + 0x270], rax
    0x140be35bc    488d05b5fbf201           lea              rax, [rip + 0x1f2fbb5] -> 0x142b13178
    0x140be35c3    48898520020000           mov              qword ptr [rbp + 0x220], rax
    0x140be35ca    48c7852802000002000000   mov              qword ptr [rbp + 0x228], 2
    0x140be35d5    488d8558020000           lea              rax, [rbp + 0x258]
    0x140be35dc    48898530020000           mov              qword ptr [rbp + 0x230], rax
    0x140be35e3    48c7853802000002000000   mov              qword ptr [rbp + 0x238], 2
    0x140be35ee    48c7854002000000000000   mov              qword ptr [rbp + 0x240], 0
    0x140be35f9    c685af02000001           mov              byte ptr [rbp + 0x2af], 1
    0x140be3600    488d7d50                 lea              rdi, [rbp + 0x50]
    0x140be3604    488d9520020000           lea              rdx, [rbp + 0x220]
    0x140be360b    4889f9                   mov              rcx, rdi
    0x140be360e    e87d691900               call             0x140d79f90 -> alloc::fmt::format::format_inner::hdcc110f646744b43
    0x140be3613    e97c150000               jmp              0x140be4b94
    0x140be3618    b912000000               mov              ecx, 0x12
    0x140be361d    ba01000000               mov              edx, 1
    0x140be3622    e8e905a3ff               call             0x140613c10 -> __rust_alloc
    0x140be3627    4885c0                   test             rax, rax
    0x140be362a    0f84411d0000             je               0x140be5371
    0x140be3630    0f100561fbf201           movups           xmm0, xmmword ptr [rip + 0x1f2fb61] -> 0x142b13198
    0x140be3637    0f1100                   movups           xmmword ptr [rax], xmm0
    0x140be363a    66c740106573             mov              word ptr [rax + 0x10], 0x7365
    0x140be3640    48c7859002000012000000   mov              qword ptr [rbp + 0x290], 0x12
    0x140be364b    48898598020000           mov              qword ptr [rbp + 0x298], rax
    0x140be3652    48c785a002000012000000   mov              qword ptr [rbp + 0x2a0], 0x12
    0x140be365d    c685af02000001           mov              byte ptr [rbp + 0x2af], 1
    0x140be3664    488d157df8f201           lea              rdx, [rip + 0x1f2f87d] -> 0x142b12ee8
    0x140be366b    488dbd90020000           lea              rdi, [rbp + 0x290]
    0x140be3672    4889f9                   mov              rcx, rdi
    0x140be3675    e8f65e1900               call             0x140d79570 -> alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::h526e3d19dbc6f270
    0x140be367a    488b8598020000           mov              rax, qword ptr [rbp + 0x298]
    0x140be3681    c640123b                 mov              byte ptr [rax + 0x12], 0x3b
    0x140be3685    48c785a002000013000000   mov              qword ptr [rbp + 0x2a0], 0x13
    0x140be3690    4d8d7c24fb               lea              r15, [r12 - 5]
    0x140be3695    4983fc05                 cmp              r12, 5
    0x140be3699    0f8228150000             jb               0x140be4bc7
    0x140be369f    428b043b                 mov              eax, dword ptr [rbx + r15]
    0x140be36a3    898510020000             mov              dword ptr [rbp + 0x210], eax
    0x140be36a9    4889bd58020000           mov              qword ptr [rbp + 0x258], rdi
    0x140be36b0    488d05198445ff           lea              rax, [rip - 0xba7be7] -> _$LT$alloc..borrow..Cow$LT$B$GT$$u20$as$u20$core..fmt..Display$GT$::fmt::h710fd6886fc306ed
    0x140be36b7    48898560020000           mov              qword ptr [rbp + 0x260], rax
    0x140be36be    488d8510020000           lea              rax, [rbp + 0x210]
    0x140be36c5    48898568020000           mov              qword ptr [rbp + 0x268], rax
    0x140be36cc    488d05bddb44ff           lea              rax, [rip - 0xbb2443] -> bevy_reflect::impls::std::_::_$LT$impl$u20$bevy_reflect..reflect..PartialReflect$u20$for$u20$u32$GT$::debug::hf2b8da78b8013204
    0x140be36d3    48898570020000           mov              qword ptr [rbp + 0x270], rax
    0x140be36da    488d0597faf201           lea              rax, [rip + 0x1f2fa97] -> 0x142b13178
    0x140be36e1    48898520020000           mov              qword ptr [rbp + 0x220], rax
    0x140be36e8    48c7852802000002000000   mov              qword ptr [rbp + 0x228], 2
    0x140be36f3    488d8558020000           lea              rax, [rbp + 0x258]
    0x140be36fa    48898530020000           mov              qword ptr [rbp + 0x230], rax
    0x140be3701    48c7853802000002000000   mov              qword ptr [rbp + 0x238], 2
    0x140be370c    48c7854002000000000000   mov              qword ptr [rbp + 0x240], 0
    0x140be3717    c685af02000001           mov              byte ptr [rbp + 0x2af], 1
    0x140be371e    488dbd98000000           lea              rdi, [rbp + 0x98]
    0x140be3725    488d9520020000           lea              rdx, [rbp + 0x220]
    0x140be372c    4889f9                   mov              rcx, rdi
    0x140be372f    e85c681900               call             0x140d79f90 -> alloc::fmt::format::format_inner::hdcc110f646744b43
    0x140be3734    e95b140000               jmp              0x140be4b94
    0x140be3739    41bd0c000000             mov              r13d, 0xc
    0x140be373f    b90c000000               mov              ecx, 0xc
    0x140be3744    ba01000000               mov              edx, 1
    0x140be3749    e8c204a3ff               call             0x140613c10 -> __rust_alloc
    0x140be374e    4885c0                   test             rax, rax
    0x140be3751    0f84801c0000             je               0x140be53d7
    0x140be3757    48b971756575655f7375     movabs           rcx, 0x75735f6575657571
    0x140be3761    488908                   mov              qword ptr [rax], rcx
    0x140be3764    c74008626d6974           mov              dword ptr [rax + 8], 0x74696d62
    0x140be376b    48c785900200000c000000   mov              qword ptr [rbp + 0x290], 0xc
    0x140be3776    48898578020000           mov              qword ptr [rbp + 0x278], rax
    0x140be377d    48898598020000           mov              qword ptr [rbp + 0x298], rax
    0x140be3784    48c785a00200000c000000   mov              qword ptr [rbp + 0x2a0], 0xc
    0x140be378f    b80c000000               mov              eax, 0xc
    0x140be3794    e917160000               jmp              0x140be4db0
    0x140be3799    41bd12000000             mov              r13d, 0x12
    0x140be379f    b912000000               mov              ecx, 0x12
    0x140be37a4    ba01000000               mov              edx, 1
    0x140be37a9    e86204a3ff               call             0x140613c10 -> __rust_alloc
    0x140be37ae    4885c0                   test             rax, rax
    0x140be37b1    0f84421c0000             je               0x140be53f9
    0x140be37b7    0f100540fbf201           movups           xmm0, xmmword ptr [rip + 0x1f2fb40] -> 0x142b132fe
    0x140be37be    0f1100                   movups           xmmword ptr [rax], xmm0
    0x140be37c1    66c740106572             mov              word ptr [rax + 0x10], 0x7265
    0x140be37c7    e995070000               jmp              0x140be3f61
    0x140be37cc    b926000000               mov              ecx, 0x26
    0x140be37d1    ba01000000               mov              edx, 1
    0x140be37d6    e83504a3ff               call             0x140613c10 -> __rust_alloc
    0x140be37db    4885c0                   test             rax, rax
    0x140be37de    0f84171a0000             je               0x140be51fb
    0x140be37e4    0f10059df8f201           movups           xmm0, xmmword ptr [rip + 0x1f2f89d] -> 0x142b13088
    0x140be37eb    0f114010                 movups           xmmword ptr [rax + 0x10], xmm0
    0x140be37ef    0f100582f8f201           movups           xmm0, xmmword ptr [rip + 0x1f2f882] -> 0x142b13078
    0x140be37f6    0f1100                   movups           xmmword ptr [rax], xmm0
    0x140be37f9    48b95f636f6d70757465     movabs           rcx, 0x657475706d6f635f
    0x140be3803    4889481e                 mov              qword ptr [rax + 0x1e], rcx
    0x140be3807    48c7859002000026000000   mov              qword ptr [rbp + 0x290], 0x26
    0x140be3812    48898598020000           mov              qword ptr [rbp + 0x298], rax
    0x140be3819    48c785a002000026000000   mov              qword ptr [rbp + 0x2a0], 0x26
    0x140be3824    c685af02000001           mov              byte ptr [rbp + 0x2af], 1
    0x140be382b    488d15b6f6f201           lea              rdx, [rip + 0x1f2f6b6] -> 0x142b12ee8
    0x140be3832    488dbd90020000           lea              rdi, [rbp + 0x290]
    0x140be3839    4889f9                   mov              rcx, rdi
    0x140be383c    e82f5d1900               call             0x140d79570 -> alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::h526e3d19dbc6f270
    0x140be3841    488b8598020000           mov              rax, qword ptr [rbp + 0x298]
    0x140be3848    c640263b                 mov              byte ptr [rax + 0x26], 0x3b
    0x140be384c    48c785a002000027000000   mov              qword ptr [rbp + 0x2a0], 0x27
    0x140be3857    4d8d7c24f6               lea              r15, [r12 - 0xa]
    0x140be385c    4983fc0a                 cmp              r12, 0xa
    0x140be3860    0f8261130000             jb               0x140be4bc7
    0x140be3866    420fb6043b               movzx            eax, byte ptr [rbx + r15]
    0x140be386b    888510020000             mov              byte ptr [rbp + 0x210], al
    0x140be3871    4889bd58020000           mov              qword ptr [rbp + 0x258], rdi
    0x140be3878    488d05518245ff           lea              rax, [rip - 0xba7daf] -> _$LT$alloc..borrow..Cow$LT$B$GT$$u20$as$u20$core..fmt..Display$GT$::fmt::h710fd6886fc306ed
    0x140be387f    48898560020000           mov              qword ptr [rbp + 0x260], rax
    0x140be3886    4c8dad10020000           lea              r13, [rbp + 0x210]
    0x140be388d    4c89ad68020000           mov              qword ptr [rbp + 0x268], r13
    0x140be3894    488d05453657ff           lea              rax, [rip - 0xa8c9bb] -> bevy_reflect::impls::std::_::_$LT$impl$u20$bevy_reflect..reflect..PartialReflect$u20$for$u20$u8$GT$::debug::h754207d67e71c3cd
    0x140be389b    48898570020000           mov              qword ptr [rbp + 0x270], rax
    0x140be38a2    488d0507f8f201           lea              rax, [rip + 0x1f2f807] -> 0x142b130b0
    0x140be38a9    48898520020000           mov              qword ptr [rbp + 0x220], rax
    0x140be38b0    48c7852802000002000000   mov              qword ptr [rbp + 0x228], 2
    0x140be38bb    4c8da558020000           lea              r12, [rbp + 0x258]
    0x140be38c2    4c89a530020000           mov              qword ptr [rbp + 0x230], r12
    0x140be38c9    48c7853802000002000000   mov              qword ptr [rbp + 0x238], 2
    0x140be38d4    48c7854002000000000000   mov              qword ptr [rbp + 0x240], 0
    0x140be38df    c685af02000001           mov              byte ptr [rbp + 0x2af], 1
    0x140be38e6    488d8da0010000           lea              rcx, [rbp + 0x1a0]
    0x140be38ed    488d9520020000           lea              rdx, [rbp + 0x220]
    0x140be38f4    e897661900               call             0x140d79f90 -> alloc::fmt::format::format_inner::hdcc110f646744b43
    0x140be38f9    488b9590020000           mov              rdx, qword ptr [rbp + 0x290]
    0x140be3900    4885d2                   test             rdx, rdx
    0x140be3903    7412                     je               0x140be3917
    0x140be3905    488b8d98020000           mov              rcx, qword ptr [rbp + 0x298]
    0x140be390c    41b801000000             mov              r8d, 1
    0x140be3912    e80903a3ff               call             0x140613c20 -> __rust_dealloc
    0x140be3917    488b85b0010000           mov              rax, qword ptr [rbp + 0x1b0]
    0x140be391e    488985a0020000           mov              qword ptr [rbp + 0x2a0], rax
    0x140be3925    0f1085a0010000           movups           xmm0, xmmword ptr [rbp + 0x1a0]
    0x140be392c    0f298590020000           movaps           xmmword ptr [rbp + 0x290], xmm0
    0x140be3933    4a8b443b01               mov              rax, qword ptr [rbx + r15 + 1]
    0x140be3938    48898510020000           mov              qword ptr [rbp + 0x210], rax
    0x140be393f    4889bd58020000           mov              qword ptr [rbp + 0x258], rdi
    0x140be3946    488d05838145ff           lea              rax, [rip - 0xba7e7d] -> _$LT$alloc..borrow..Cow$LT$B$GT$$u20$as$u20$core..fmt..Display$GT$::fmt::h710fd6886fc306ed
    0x140be394d    48898560020000           mov              qword ptr [rbp + 0x260], rax
    0x140be3954    4c89ad68020000           mov              qword ptr [rbp + 0x268], r13
    0x140be395b    488d055e4b4eff           lea              rax, [rip - 0xb1b4a2] -> core::fmt::num::_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$::fmt::h8e88e8dbb6030fc5 (.llvm.4519547824212384754)
    0x140be3962    48898570020000           mov              qword ptr [rbp + 0x270], rax
    0x140be3969    488d05e8f6f201           lea              rax, [rip + 0x1f2f6e8] -> 0x142b13058
    0x140be3970    48898520020000           mov              qword ptr [rbp + 0x220], rax
    0x140be3977    48c7852802000002000000   mov              qword ptr [rbp + 0x228], 2
    0x140be3982    4c89a530020000           mov              qword ptr [rbp + 0x230], r12
    0x140be3989    48c7853802000002000000   mov              qword ptr [rbp + 0x238], 2
    0x140be3994    48c7854002000000000000   mov              qword ptr [rbp + 0x240], 0
    0x140be399f    c685af02000001           mov              byte ptr [rbp + 0x2af], 1
    0x140be39a6    488dbd28010000           lea              rdi, [rbp + 0x128]
    0x140be39ad    488d9520020000           lea              rdx, [rbp + 0x220]
    0x140be39b4    4889f9                   mov              rcx, rdi
    0x140be39b7    e8d4651900               call             0x140d79f90 -> alloc::fmt::format::format_inner::hdcc110f646744b43
    0x140be39bc    e9d3110000               jmp              0x140be4b94
    0x140be39c1    b916000000               mov              ecx, 0x16
    0x140be39c6    ba01000000               mov              edx, 1
    0x140be39cb    e84002a3ff               call             0x140613c10 -> __rust_alloc
    0x140be39d0    4885c0                   test             rax, rax
    0x140be39d3    0f849a170000             je               0x140be5173
    0x140be39d9    0f10052cf8f201           movups           xmm0, xmmword ptr [rip + 0x1f2f82c] -> 0x142b1320c
    0x140be39e0    0f1100                   movups           xmmword ptr [rax], xmm0
    0x140be39e3    48b9746f725f706f6f6c     movabs           rcx, 0x6c6f6f705f726f74
    0x140be39ed    4889480e                 mov              qword ptr [rax + 0xe], rcx
    0x140be39f1    48c7859002000016000000   mov              qword ptr [rbp + 0x290], 0x16
    0x140be39fc    48898598020000           mov              qword ptr [rbp + 0x298], rax
    0x140be3a03    48c785a002000016000000   mov              qword ptr [rbp + 0x2a0], 0x16
    0x140be3a0e    c685af02000001           mov              byte ptr [rbp + 0x2af], 1
    0x140be3a15    488d15ccf4f201           lea              rdx, [rip + 0x1f2f4cc] -> 0x142b12ee8
    0x140be3a1c    488dbd90020000           lea              rdi, [rbp + 0x290]
    0x140be3a23    4889f9                   mov              rcx, rdi
    0x140be3a26    e8455b1900               call             0x140d79570 -> alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::h526e3d19dbc6f270
    0x140be3a2b    488b8598020000           mov              rax, qword ptr [rbp + 0x298]
    0x140be3a32    c640163b                 mov              byte ptr [rax + 0x16], 0x3b
    0x140be3a36    48c785a002000017000000   mov              qword ptr [rbp + 0x2a0], 0x17
    0x140be3a41    4d8d7c24fb               lea              r15, [r12 - 5]
    0x140be3a46    4983fc05                 cmp              r12, 5
    0x140be3a4a    0f8277110000             jb               0x140be4bc7
    0x140be3a50    428b043b                 mov              eax, dword ptr [rbx + r15]
    0x140be3a54    898510020000             mov              dword ptr [rbp + 0x210], eax
    0x140be3a5a    4889bd58020000           mov              qword ptr [rbp + 0x258], rdi
    0x140be3a61    488d05688045ff           lea              rax, [rip - 0xba7f98] -> _$LT$alloc..borrow..Cow$LT$B$GT$$u20$as$u20$core..fmt..Display$GT$::fmt::h710fd6886fc306ed
    0x140be3a68    48898560020000           mov              qword ptr [rbp + 0x260], rax
    0x140be3a6f    488d8510020000           lea              rax, [rbp + 0x210]
    0x140be3a76    48898568020000           mov              qword ptr [rbp + 0x268], rax
    0x140be3a7d    488d050cefffff           lea              rax, [rip - 0x10f4] -> rhapsody::leek::impl$2::fmt
    0x140be3a84    48898570020000           mov              qword ptr [rbp + 0x270], rax
    0x140be3a8b    488d0596f7f201           lea              rax, [rip + 0x1f2f796] -> 0x142b13228
    0x140be3a92    48898520020000           mov              qword ptr [rbp + 0x220], rax
    0x140be3a99    48c7852802000002000000   mov              qword ptr [rbp + 0x228], 2
    0x140be3aa4    488d8558020000           lea              rax, [rbp + 0x258]
    0x140be3aab    48898530020000           mov              qword ptr [rbp + 0x230], rax
    0x140be3ab2    48c7853802000002000000   mov              qword ptr [rbp + 0x238], 2
    0x140be3abd    48c7854002000000000000   mov              qword ptr [rbp + 0x240], 0
    0x140be3ac8    c685af02000001           mov              byte ptr [rbp + 0x2af], 1
    0x140be3acf    488d7d20                 lea              rdi, [rbp + 0x20]
    0x140be3ad3    488d9520020000           lea              rdx, [rbp + 0x220]
    0x140be3ada    4889f9                   mov              rcx, rdi
    0x140be3add    e8ae641900               call             0x140d79f90 -> alloc::fmt::format::format_inner::hdcc110f646744b43
    0x140be3ae2    e9ad100000               jmp              0x140be4b94
    0x140be3ae7    b91d000000               mov              ecx, 0x1d
    0x140be3aec    ba01000000               mov              edx, 1
    0x140be3af1    e81a01a3ff               call             0x140613c10 -> __rust_alloc
    0x140be3af6    4885c0                   test             rax, rax
    0x140be3af9    0f84ec150000             je               0x140be50eb
    0x140be3aff    0f1005d7f5f201           movups           xmm0, xmmword ptr [rip + 0x1f2f5d7] -> 0x142b130dd
    0x140be3b06    0f11400d                 movups           xmmword ptr [rax + 0xd], xmm0
    0x140be3b0a    0f1005bff5f201           movups           xmm0, xmmword ptr [rip + 0x1f2f5bf] -> 0x142b130d0
    0x140be3b11    0f1100                   movups           xmmword ptr [rax], xmm0
    0x140be3b14    48c785900200001d000000   mov              qword ptr [rbp + 0x290], 0x1d
    0x140be3b1f    48898598020000           mov              qword ptr [rbp + 0x298], rax
    0x140be3b26    48c785a00200001d000000   mov              qword ptr [rbp + 0x2a0], 0x1d
    0x140be3b31    c685af02000001           mov              byte ptr [rbp + 0x2af], 1
    0x140be3b38    488d15a9f3f201           lea              rdx, [rip + 0x1f2f3a9] -> 0x142b12ee8
    0x140be3b3f    488dbd90020000           lea              rdi, [rbp + 0x290]
    0x140be3b46    4889f9                   mov              rcx, rdi
    0x140be3b49    e8225a1900               call             0x140d79570 -> alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::h526e3d19dbc6f270
    0x140be3b4e    488b8598020000           mov              rax, qword ptr [rbp + 0x298]
    0x140be3b55    c6401d3b                 mov              byte ptr [rax + 0x1d], 0x3b
    0x140be3b59    48c785a00200001e000000   mov              qword ptr [rbp + 0x2a0], 0x1e
    0x140be3b64    4d8d7c24e7               lea              r15, [r12 - 0x19]
    0x140be3b69    4983fc19                 cmp              r12, 0x19
    0x140be3b6d    0f8254100000             jb               0x140be4bc7
    0x140be3b73    420f10043b               movups           xmm0, xmmword ptr [rbx + r15]
    0x140be3b78    0f298510020000           movaps           xmmword ptr [rbp + 0x210], xmm0
    0x140be3b7f    4889bd58020000           mov              qword ptr [rbp + 0x258], rdi
    0x140be3b86    488d05437f45ff           lea              rax, [rip - 0xba80bd] -> _$LT$alloc..borrow..Cow$LT$B$GT$$u20$as$u20$core..fmt..Display$GT$::fmt::h710fd6886fc306ed
    0x140be3b8d    48898560020000           mov              qword ptr [rbp + 0x260], rax
    0x140be3b94    4c8dad10020000           lea              r13, [rbp + 0x210]
    0x140be3b9b    4c89ad68020000           mov              qword ptr [rbp + 0x268], r13
    0x140be3ba2    488d0567ebffff           lea              rax, [rip - 0x1499] -> rhapsody::leek::impl$0::fmt
    0x140be3ba9    48898570020000           mov              qword ptr [rbp + 0x270], rax
    0x140be3bb0    488d0541f5f201           lea              rax, [rip + 0x1f2f541] -> 0x142b130f8
    0x140be3bb7    48898520020000           mov              qword ptr [rbp + 0x220], rax
    0x140be3bbe    48c7852802000002000000   mov              qword ptr [rbp + 0x228], 2
    0x140be3bc9    4c8da558020000           lea              r12, [rbp + 0x258]
    0x140be3bd0    4c89a530020000           mov              qword ptr [rbp + 0x230], r12
    0x140be3bd7    48c7853802000002000000   mov              qword ptr [rbp + 0x238], 2
    0x140be3be2    48c7854002000000000000   mov              qword ptr [rbp + 0x240], 0
    0x140be3bed    c685af02000001           mov              byte ptr [rbp + 0x2af], 1
    0x140be3bf4    488d8d88010000           lea              rcx, [rbp + 0x188]
    0x140be3bfb    488d9520020000           lea              rdx, [rbp + 0x220]
    0x140be3c02    e889631900               call             0x140d79f90 -> alloc::fmt::format::format_inner::hdcc110f646744b43
    0x140be3c07    488b9590020000           mov              rdx, qword ptr [rbp + 0x290]
    0x140be3c0e    4885d2                   test             rdx, rdx
    0x140be3c11    7412                     je               0x140be3c25
    0x140be3c13    488b8d98020000           mov              rcx, qword ptr [rbp + 0x298]
    0x140be3c1a    41b801000000             mov              r8d, 1
    0x140be3c20    e8fbffa2ff               call             0x140613c20 -> __rust_dealloc
    0x140be3c25    4a8d043b                 lea              rax, [rbx + r15]
    0x140be3c29    488b8d98010000           mov              rcx, qword ptr [rbp + 0x198]
    0x140be3c30    48898da0020000           mov              qword ptr [rbp + 0x2a0], rcx
    0x140be3c37    0f108588010000           movups           xmm0, xmmword ptr [rbp + 0x188]
    0x140be3c3e    0f298590020000           movaps           xmmword ptr [rbp + 0x290], xmm0
    0x140be3c45    488b4010                 mov              rax, qword ptr [rax + 0x10]
    0x140be3c49    48898510020000           mov              qword ptr [rbp + 0x210], rax
    0x140be3c50    4889bd58020000           mov              qword ptr [rbp + 0x258], rdi
    0x140be3c57    488d05727e45ff           lea              rax, [rip - 0xba818e] -> _$LT$alloc..borrow..Cow$LT$B$GT$$u20$as$u20$core..fmt..Display$GT$::fmt::h710fd6886fc306ed
    0x140be3c5e    48898560020000           mov              qword ptr [rbp + 0x260], rax
    0x140be3c65    4c89ad68020000           mov              qword ptr [rbp + 0x268], r13
    0x140be3c6c    488d054d484eff           lea              rax, [rip - 0xb1b7b3] -> core::fmt::num::_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$::fmt::h8e88e8dbb6030fc5 (.llvm.4519547824212384754)
    0x140be3c73    48898570020000           mov              qword ptr [rbp + 0x270], rax
    0x140be3c7a    488d059ff4f201           lea              rax, [rip + 0x1f2f49f] -> 0x142b13120
    0x140be3c81    48898520020000           mov              qword ptr [rbp + 0x220], rax
    0x140be3c88    48c7852802000002000000   mov              qword ptr [rbp + 0x228], 2
    0x140be3c93    4c89a530020000           mov              qword ptr [rbp + 0x230], r12
    0x140be3c9a    48c7853802000002000000   mov              qword ptr [rbp + 0x238], 2
    0x140be3ca5    48c7854002000000000000   mov              qword ptr [rbp + 0x240], 0
    0x140be3cb0    c685af02000001           mov              byte ptr [rbp + 0x2af], 1
    0x140be3cb7    488dbd10010000           lea              rdi, [rbp + 0x110]
    0x140be3cbe    488d9520020000           lea              rdx, [rbp + 0x220]
    0x140be3cc5    4889f9                   mov              rcx, rdi
    0x140be3cc8    e8c3621900               call             0x140d79f90 -> alloc::fmt::format::format_inner::hdcc110f646744b43
    0x140be3ccd    e9c20e0000               jmp              0x140be4b94
    0x140be3cd2    b90e000000               mov              ecx, 0xe
    0x140be3cd7    ba01000000               mov              edx, 1
    0x140be3cdc    e82fffa2ff               call             0x140613c10 -> __rust_alloc
    0x140be3ce1    4885c0                   test             rax, rax
    0x140be3ce4    0f8457130000             je               0x140be5041
    0x140be3cea    48b9795f696d61676573     movabs           rcx, 0x736567616d695f79
    0x140be3cf4    48894806                 mov              qword ptr [rax + 6], rcx
    0x140be3cf8    48b964657374726f795f     movabs           rcx, 0x5f796f7274736564
    0x140be3d02    488908                   mov              qword ptr [rax], rcx
    0x140be3d05    48c785900200000e000000   mov              qword ptr [rbp + 0x290], 0xe
    0x140be3d10    48898598020000           mov              qword ptr [rbp + 0x298], rax
    0x140be3d17    48c785a00200000e000000   mov              qword ptr [rbp + 0x2a0], 0xe
    0x140be3d22    c685af02000001           mov              byte ptr [rbp + 0x2af], 1
    0x140be3d29    488d15b8f1f201           lea              rdx, [rip + 0x1f2f1b8] -> 0x142b12ee8
    0x140be3d30    488dbd90020000           lea              rdi, [rbp + 0x290]
    0x140be3d37    4889f9                   mov              rcx, rdi
    0x140be3d3a    e831581900               call             0x140d79570 -> alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::h526e3d19dbc6f270
    0x140be3d3f    488b8598020000           mov              rax, qword ptr [rbp + 0x298]
    0x140be3d46    c6400e3b                 mov              byte ptr [rax + 0xe], 0x3b
    0x140be3d4a    48c785a00200000f000000   mov              qword ptr [rbp + 0x2a0], 0xf
    0x140be3d55    4d8d7c24fb               lea              r15, [r12 - 5]
    0x140be3d5a    4983fc05                 cmp              r12, 5
    0x140be3d5e    0f82630e0000             jb               0x140be4bc7
    0x140be3d64    428b043b                 mov              eax, dword ptr [rbx + r15]
    0x140be3d68    898510020000             mov              dword ptr [rbp + 0x210], eax
    0x140be3d6e    4889bd58020000           mov              qword ptr [rbp + 0x258], rdi
    0x140be3d75    488d05547d45ff           lea              rax, [rip - 0xba82ac] -> _$LT$alloc..borrow..Cow$LT$B$GT$$u20$as$u20$core..fmt..Display$GT$::fmt::h710fd6886fc306ed
    0x140be3d7c    48898560020000           mov              qword ptr [rbp + 0x260], rax
    0x140be3d83    488d8510020000           lea              rax, [rbp + 0x210]
    0x140be3d8a    48898568020000           mov              qword ptr [rbp + 0x268], rax
    0x140be3d91    488d05f8d444ff           lea              rax, [rip - 0xbb2b08] -> bevy_reflect::impls::std::_::_$LT$impl$u20$bevy_reflect..reflect..PartialReflect$u20$for$u20$u32$GT$::debug::hf2b8da78b8013204
    0x140be3d98    48898570020000           mov              qword ptr [rbp + 0x270], rax
    0x140be3d9f    488d05d2f3f201           lea              rax, [rip + 0x1f2f3d2] -> 0x142b13178
    0x140be3da6    48898520020000           mov              qword ptr [rbp + 0x220], rax
    0x140be3dad    48c7852802000002000000   mov              qword ptr [rbp + 0x228], 2
    0x140be3db8    488d8558020000           lea              rax, [rbp + 0x258]
    0x140be3dbf    48898530020000           mov              qword ptr [rbp + 0x230], rax
    0x140be3dc6    48c7853802000002000000   mov              qword ptr [rbp + 0x238], 2
    0x140be3dd1    48c7854002000000000000   mov              qword ptr [rbp + 0x240], 0
    0x140be3ddc    c685af02000001           mov              byte ptr [rbp + 0x2af], 1
    0x140be3de3    488dbdc8000000           lea              rdi, [rbp + 0xc8]
    0x140be3dea    488d9520020000           lea              rdx, [rbp + 0x220]
    0x140be3df1    4889f9                   mov              rcx, rdi
    0x140be3df4    e897611900               call             0x140d79f90 -> alloc::fmt::format::format_inner::hdcc110f646744b43
    0x140be3df9    e9960d0000               jmp              0x140be4b94
    0x140be3dfe    b928000000               mov              ecx, 0x28
    0x140be3e03    ba01000000               mov              edx, 1
    0x140be3e08    e803fea2ff               call             0x140613c10 -> __rust_alloc
    0x140be3e0d    4885c0                   test             rax, rax
    0x140be3e10    0f847d150000             je               0x140be5393
    0x140be3e16    0f100503f2f201           movups           xmm0, xmmword ptr [rip + 0x1f2f203] -> 0x142b13020
    0x140be3e1d    0f114010                 movups           xmmword ptr [rax + 0x10], xmm0
    0x140be3e21    0f1005e8f1f201           movups           xmm0, xmmword ptr [rip + 0x1f2f1e8] -> 0x142b13010
    0x140be3e28    0f1100                   movups           xmmword ptr [rax], xmm0
    0x140be3e2b    48b96e6976657273616c     movabs           rcx, 0x6c6173726576696e
    0x140be3e35    48894820                 mov              qword ptr [rax + 0x20], rcx
    0x140be3e39    48c7859002000028000000   mov              qword ptr [rbp + 0x290], 0x28
    0x140be3e44    48898598020000           mov              qword ptr [rbp + 0x298], rax
    0x140be3e4b    48c785a002000028000000   mov              qword ptr [rbp + 0x2a0], 0x28
    0x140be3e56    c685af02000001           mov              byte ptr [rbp + 0x2af], 1
    0x140be3e5d    488d1584f0f201           lea              rdx, [rip + 0x1f2f084] -> 0x142b12ee8
    0x140be3e64    488dbd90020000           lea              rdi, [rbp + 0x290]
    0x140be3e6b    4889f9                   mov              rcx, rdi
    0x140be3e6e    e8fd561900               call             0x140d79570 -> alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::h526e3d19dbc6f270
    0x140be3e73    488b8598020000           mov              rax, qword ptr [rbp + 0x298]
    0x140be3e7a    c640283b                 mov              byte ptr [rax + 0x28], 0x3b
    0x140be3e7e    48c785a002000029000000   mov              qword ptr [rbp + 0x2a0], 0x29
    0x140be3e89    4d8d7c24f7               lea              r15, [r12 - 9]
    0x140be3e8e    4983fc09                 cmp              r12, 9
    0x140be3e92    0f822f0d0000             jb               0x140be4bc7
    0x140be3e98    4a8b043b                 mov              rax, qword ptr [rbx + r15]
    0x140be3e9c    48898510020000           mov              qword ptr [rbp + 0x210], rax
    0x140be3ea3    4889bd58020000           mov              qword ptr [rbp + 0x258], rdi
    0x140be3eaa    488d051f7c45ff           lea              rax, [rip - 0xba83e1] -> _$LT$alloc..borrow..Cow$LT$B$GT$$u20$as$u20$core..fmt..Display$GT$::fmt::h710fd6886fc306ed
    0x140be3eb1    48898560020000           mov              qword ptr [rbp + 0x260], rax
    0x140be3eb8    488d8510020000           lea              rax, [rbp + 0x210]
    0x140be3ebf    48898568020000           mov              qword ptr [rbp + 0x268], rax
    0x140be3ec6    488d05f3454eff           lea              rax, [rip - 0xb1ba0d] -> core::fmt::num::_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$::fmt::h8e88e8dbb6030fc5 (.llvm.4519547824212384754)
    0x140be3ecd    48898570020000           mov              qword ptr [rbp + 0x270], rax
    0x140be3ed4    488d057df1f201           lea              rax, [rip + 0x1f2f17d] -> 0x142b13058
    0x140be3edb    48898520020000           mov              qword ptr [rbp + 0x220], rax
    0x140be3ee2    48c7852802000002000000   mov              qword ptr [rbp + 0x228], 2
    0x140be3eed    488d8558020000           lea              rax, [rbp + 0x258]
    0x140be3ef4    48898530020000           mov              qword ptr [rbp + 0x230], rax
    0x140be3efb    48c7853802000002000000   mov              qword ptr [rbp + 0x238], 2
    0x140be3f06    48c7854002000000000000   mov              qword ptr [rbp + 0x240], 0
    0x140be3f11    c685af02000001           mov              byte ptr [rbp + 0x2af], 1
    0x140be3f18    488dbd40010000           lea              rdi, [rbp + 0x140]
    0x140be3f1f    488d9520020000           lea              rdx, [rbp + 0x220]
    0x140be3f26    4889f9                   mov              rcx, rdi
    0x140be3f29    e862601900               call             0x140d79f90 -> alloc::fmt::format::format_inner::hdcc110f646744b43
    0x140be3f2e    e9610c0000               jmp              0x140be4b94
    0x140be3f33    41bd12000000             mov              r13d, 0x12
    0x140be3f39    b912000000               mov              ecx, 0x12
    0x140be3f3e    ba01000000               mov              edx, 1
    0x140be3f43    e8c8fca2ff               call             0x140613c10 -> __rust_alloc
    0x140be3f48    4885c0                   test             rax, rax
    0x140be3f4b    0f84cc120000             je               0x140be521d
    0x140be3f51    0f100530f4f201           movups           xmm0, xmmword ptr [rip + 0x1f2f430] -> 0x142b13388
    0x140be3f58    0f1100                   movups           xmmword ptr [rax], xmm0
    0x140be3f5b    66c740106765             mov              word ptr [rax + 0x10], 0x6567
    0x140be3f61    48c7859002000012000000   mov              qword ptr [rbp + 0x290], 0x12
    0x140be3f6c    48898578020000           mov              qword ptr [rbp + 0x278], rax
    0x140be3f73    48898598020000           mov              qword ptr [rbp + 0x298], rax
    0x140be3f7a    48c785a002000012000000   mov              qword ptr [rbp + 0x2a0], 0x12
    0x140be3f85    b812000000               mov              eax, 0x12
    0x140be3f8a    e9210e0000               jmp              0x140be4db0
    0x140be3f8f    b917000000               mov              ecx, 0x17
    0x140be3f94    ba01000000               mov              edx, 1
    0x140be3f99    e872fca2ff               call             0x140613c10 -> __rust_alloc
    0x140be3f9e    4885c0                   test             rax, rax
    0x140be3fa1    0f8474140000             je               0x140be541b
    0x140be3fa7    0f1005aff2f201           movups           xmm0, xmmword ptr [rip + 0x1f2f2af] -> 0x142b1325d
    0x140be3fae    0f1100                   movups           xmmword ptr [rax], xmm0
    0x140be3fb1    48b970746f725f736574     movabs           rcx, 0x7465735f726f7470
    0x140be3fbb    4889480f                 mov              qword ptr [rax + 0xf], rcx
    0x140be3fbf    48c7859002000017000000   mov              qword ptr [rbp + 0x290], 0x17
    0x140be3fca    48898598020000           mov              qword ptr [rbp + 0x298], rax
    0x140be3fd1    48c785a002000017000000   mov              qword ptr [rbp + 0x2a0], 0x17
    0x140be3fdc    c685af02000001           mov              byte ptr [rbp + 0x2af], 1
    0x140be3fe3    488d15feeef201           lea              rdx, [rip + 0x1f2eefe] -> 0x142b12ee8
    0x140be3fea    488dbd90020000           lea              rdi, [rbp + 0x290]
    0x140be3ff1    4889f9                   mov              rcx, rdi
    0x140be3ff4    e877551900               call             0x140d79570 -> alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::h526e3d19dbc6f270
    0x140be3ff9    488b8598020000           mov              rax, qword ptr [rbp + 0x298]
    0x140be4000    c640173b                 mov              byte ptr [rax + 0x17], 0x3b
    0x140be4004    48c785a002000018000000   mov              qword ptr [rbp + 0x2a0], 0x18
    0x140be400f    4d8d7c24fb               lea              r15, [r12 - 5]
    0x140be4014    4983fc05                 cmp              r12, 5
    0x140be4018    0f82a90b0000             jb               0x140be4bc7
    0x140be401e    428b043b                 mov              eax, dword ptr [rbx + r15]
    0x140be4022    898510020000             mov              dword ptr [rbp + 0x210], eax
    0x140be4028    4889bd58020000           mov              qword ptr [rbp + 0x258], rdi
    0x140be402f    488d059a7a45ff           lea              rax, [rip - 0xba8566] -> _$LT$alloc..borrow..Cow$LT$B$GT$$u20$as$u20$core..fmt..Display$GT$::fmt::h710fd6886fc306ed
    0x140be4036    48898560020000           mov              qword ptr [rbp + 0x260], rax
    0x140be403d    488d8510020000           lea              rax, [rbp + 0x210]
    0x140be4044    48898568020000           mov              qword ptr [rbp + 0x268], rax
    0x140be404b    488d053ee9ffff           lea              rax, [rip - 0x16c2] -> rhapsody::leek::impl$2::fmt
    0x140be4052    48898570020000           mov              qword ptr [rbp + 0x270], rax
    0x140be4059    488d05c8f1f201           lea              rax, [rip + 0x1f2f1c8] -> 0x142b13228
    0x140be4060    48898520020000           mov              qword ptr [rbp + 0x220], rax
    0x140be4067    48c7852802000002000000   mov              qword ptr [rbp + 0x228], 2
    0x140be4072    488d8558020000           lea              rax, [rbp + 0x258]
    0x140be4079    48898530020000           mov              qword ptr [rbp + 0x230], rax
    0x140be4080    48c7853802000002000000   mov              qword ptr [rbp + 0x238], 2
    0x140be408b    48c7854002000000000000   mov              qword ptr [rbp + 0x240], 0
    0x140be4096    c685af02000001           mov              byte ptr [rbp + 0x2af], 1
    0x140be409d    488d7df0                 lea              rdi, [rbp - 0x10]
    0x140be40a1    488d9520020000           lea              rdx, [rbp + 0x220]
    0x140be40a8    4889f9                   mov              rcx, rdi
    0x140be40ab    e8e05e1900               call             0x140d79f90 -> alloc::fmt::format::format_inner::hdcc110f646744b43
    0x140be40b0    e9df0a0000               jmp              0x140be4b94
    0x140be40b5    41bd20000000             mov              r13d, 0x20
    0x140be40bb    b920000000               mov              ecx, 0x20
    0x140be40c0    ba01000000               mov              edx, 1
    0x140be40c5    e846fba2ff               call             0x140613c10 -> __rust_alloc
    0x140be40ca    4885c0                   test             rax, rax
    0x140be40cd    0f84b5140000             je               0x140be5588
    0x140be40d3    0f10054af3f201           movups           xmm0, xmmword ptr [rip + 0x1f2f34a] -> 0x142b13424
    0x140be40da    0f114010                 movups           xmmword ptr [rax + 0x10], xmm0
    0x140be40de    0f10052ff3f201           movups           xmm0, xmmword ptr [rip + 0x1f2f32f] -> 0x142b13414
    0x140be40e5    e99d030000               jmp              0x140be4487
    0x140be40ea    41bd16000000             mov              r13d, 0x16
    0x140be40f0    b916000000               mov              ecx, 0x16
    0x140be40f5    ba01000000               mov              edx, 1
    0x140be40fa    e811fba2ff               call             0x140613c10 -> __rust_alloc
    0x140be40ff    4885c0                   test             rax, rax
    0x140be4102    0f8435130000             je               0x140be543d
    0x140be4108    0f10059cf1f201           movups           xmm0, xmmword ptr [rip + 0x1f2f19c] -> 0x142b132ab
    0x140be410f    0f1100                   movups           xmmword ptr [rax], xmm0
    0x140be4112    48b9706970656c696e65     movabs           rcx, 0x656e696c65706970
    0x140be411c    4889480e                 mov              qword ptr [rax + 0xe], rcx
    0x140be4120    48c7859002000016000000   mov              qword ptr [rbp + 0x290], 0x16
    0x140be412b    48898578020000           mov              qword ptr [rbp + 0x278], rax
    0x140be4132    48898598020000           mov              qword ptr [rbp + 0x298], rax
    0x140be4139    48c785a002000016000000   mov              qword ptr [rbp + 0x2a0], 0x16
    0x140be4144    b816000000               mov              eax, 0x16
    0x140be4149    e9620c0000               jmp              0x140be4db0
    0x140be414e    41bd09000000             mov              r13d, 9
    0x140be4154    b909000000               mov              ecx, 9
    0x140be4159    ba01000000               mov              edx, 1
    0x140be415e    e8adfaa2ff               call             0x140613c10 -> __rust_alloc
    0x140be4163    4885c0                   test             rax, rax
    0x140be4166    0f84d3100000             je               0x140be523f
    0x140be416c    48b9776169745f69646c     movabs           rcx, 0x6c64695f74696177
    0x140be4176    488908                   mov              qword ptr [rax], rcx
    0x140be4179    c6400865                 mov              byte ptr [rax + 8], 0x65
    0x140be417d    48c7859002000009000000   mov              qword ptr [rbp + 0x290], 9
    0x140be4188    48898578020000           mov              qword ptr [rbp + 0x278], rax
    0x140be418f    48898598020000           mov              qword ptr [rbp + 0x298], rax
    0x140be4196    48c785a002000009000000   mov              qword ptr [rbp + 0x2a0], 9
    0x140be41a1    b809000000               mov              eax, 9
    0x140be41a6    e9050c0000               jmp              0x140be4db0
    0x140be41ab    41bd15000000             mov              r13d, 0x15
    0x140be41b1    b915000000               mov              ecx, 0x15
    0x140be41b6    ba01000000               mov              edx, 1
    0x140be41bb    e850faa2ff               call             0x140613c10 -> __rust_alloc
    0x140be41c0    4885c0                   test             rax, rax
    0x140be41c3    0f8498100000             je               0x140be5261
    0x140be41c9    0f1005f1f0f201           movups           xmm0, xmmword ptr [rip + 0x1f2f0f1] -> 0x142b132c1
    0x140be41d0    0f1100                   movups           xmmword ptr [rax], xmm0
    0x140be41d3    48b9645f627566666572     movabs           rcx, 0x7265666675625f64
    0x140be41dd    4889480d                 mov              qword ptr [rax + 0xd], rcx
    0x140be41e1    48c7859002000015000000   mov              qword ptr [rbp + 0x290], 0x15
    0x140be41ec    48898578020000           mov              qword ptr [rbp + 0x278], rax
    0x140be41f3    48898598020000           mov              qword ptr [rbp + 0x298], rax
    0x140be41fa    48c785a002000015000000   mov              qword ptr [rbp + 0x2a0], 0x15
    0x140be4205    b815000000               mov              eax, 0x15
    0x140be420a    e9a10b0000               jmp              0x140be4db0
    0x140be420f    b918000000               mov              ecx, 0x18
    0x140be4214    ba01000000               mov              edx, 1
    0x140be4219    e8f2f9a2ff               call             0x140613c10 -> __rust_alloc
    0x140be421e    4885c0                   test             rax, rax
    0x140be4221    0f845c100000             je               0x140be5283
    0x140be4227    0f1005b5eff201           movups           xmm0, xmmword ptr [rip + 0x1f2efb5] -> 0x142b131e3
    0x140be422e    0f1100                   movups           xmmword ptr [rax], xmm0
    0x140be4231    48b96f725f706f6f6c73     movabs           rcx, 0x736c6f6f705f726f
    0x140be423b    48894810                 mov              qword ptr [rax + 0x10], rcx
    0x140be423f    48c7859002000018000000   mov              qword ptr [rbp + 0x290], 0x18
    0x140be424a    48898598020000           mov              qword ptr [rbp + 0x298], rax
    0x140be4251    48c785a002000018000000   mov              qword ptr [rbp + 0x2a0], 0x18
    0x140be425c    c685af02000001           mov              byte ptr [rbp + 0x2af], 1
    0x140be4263    488d157eecf201           lea              rdx, [rip + 0x1f2ec7e] -> 0x142b12ee8
    0x140be426a    488dbd90020000           lea              rdi, [rbp + 0x290]
    0x140be4271    4889f9                   mov              rcx, rdi
    0x140be4274    e8f7521900               call             0x140d79570 -> alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::h526e3d19dbc6f270
    0x140be4279    488b8598020000           mov              rax, qword ptr [rbp + 0x298]
    0x140be4280    c640183b                 mov              byte ptr [rax + 0x18], 0x3b
    0x140be4284    48c785a002000019000000   mov              qword ptr [rbp + 0x2a0], 0x19
    0x140be428f    4d8d7c24fb               lea              r15, [r12 - 5]
    0x140be4294    4983fc05                 cmp              r12, 5
    0x140be4298    0f8229090000             jb               0x140be4bc7
    0x140be429e    428b043b                 mov              eax, dword ptr [rbx + r15]
    0x140be42a2    898510020000             mov              dword ptr [rbp + 0x210], eax
    0x140be42a8    4889bd58020000           mov              qword ptr [rbp + 0x258], rdi
    0x140be42af    488d051a7845ff           lea              rax, [rip - 0xba87e6] -> _$LT$alloc..borrow..Cow$LT$B$GT$$u20$as$u20$core..fmt..Display$GT$::fmt::h710fd6886fc306ed
    0x140be42b6    48898560020000           mov              qword ptr [rbp + 0x260], rax
    0x140be42bd    488d8510020000           lea              rax, [rbp + 0x210]
    0x140be42c4    48898568020000           mov              qword ptr [rbp + 0x268], rax
    0x140be42cb    488d05becf44ff           lea              rax, [rip - 0xbb3042] -> bevy_reflect::impls::std::_::_$LT$impl$u20$bevy_reflect..reflect..PartialReflect$u20$for$u20$u32$GT$::debug::hf2b8da78b8013204
    0x140be42d2    48898570020000           mov              qword ptr [rbp + 0x270], rax
    0x140be42d9    488d0598eef201           lea              rax, [rip + 0x1f2ee98] -> 0x142b13178
    0x140be42e0    48898520020000           mov              qword ptr [rbp + 0x220], rax
    0x140be42e7    48c7852802000002000000   mov              qword ptr [rbp + 0x228], 2
    0x140be42f2    488d8558020000           lea              rax, [rbp + 0x258]
    0x140be42f9    48898530020000           mov              qword ptr [rbp + 0x230], rax
    0x140be4300    48c7853802000002000000   mov              qword ptr [rbp + 0x238], 2
    0x140be430b    48c7854002000000000000   mov              qword ptr [rbp + 0x240], 0
    0x140be4316    c685af02000001           mov              byte ptr [rbp + 0x2af], 1
    0x140be431d    488d7d38                 lea              rdi, [rbp + 0x38]
    0x140be4321    488d9520020000           lea              rdx, [rbp + 0x220]
    0x140be4328    4889f9                   mov              rcx, rdi
    0x140be432b    e8605c1900               call             0x140d79f90 -> alloc::fmt::format::format_inner::hdcc110f646744b43
    0x140be4330    e95f080000               jmp              0x140be4b94
    0x140be4335    b913000000               mov              ecx, 0x13
    0x140be433a    ba01000000               mov              edx, 1
    0x140be433f    e8ccf8a2ff               call             0x140613c10 -> __rust_alloc
    0x140be4344    4885c0                   test             rax, rax
    0x140be4347    0f84c00d0000             je               0x140be510d
    0x140be434d    0f100506eef201           movups           xmm0, xmmword ptr [rip + 0x1f2ee06] -> 0x142b1315a
    0x140be4354    0f1100                   movups           xmmword ptr [rax], xmm0
    0x140be4357    c7400f69657773           mov              dword ptr [rax + 0xf], 0x73776569
    0x140be435e    48c7859002000013000000   mov              qword ptr [rbp + 0x290], 0x13
    0x140be4369    48898598020000           mov              qword ptr [rbp + 0x298], rax
    0x140be4370    48c785a002000013000000   mov              qword ptr [rbp + 0x2a0], 0x13
    0x140be437b    c685af02000001           mov              byte ptr [rbp + 0x2af], 1
    0x140be4382    488d155febf201           lea              rdx, [rip + 0x1f2eb5f] -> 0x142b12ee8
    0x140be4389    488dbd90020000           lea              rdi, [rbp + 0x290]
    0x140be4390    4889f9                   mov              rcx, rdi
    0x140be4393    e8d8511900               call             0x140d79570 -> alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::h526e3d19dbc6f270
    0x140be4398    488b8598020000           mov              rax, qword ptr [rbp + 0x298]
    0x140be439f    c640133b                 mov              byte ptr [rax + 0x13], 0x3b
    0x140be43a3    48c785a002000014000000   mov              qword ptr [rbp + 0x2a0], 0x14
    0x140be43ae    4d8d7c24fb               lea              r15, [r12 - 5]
    0x140be43b3    4983fc05                 cmp              r12, 5
    0x140be43b7    0f820a080000             jb               0x140be4bc7
    0x140be43bd    428b043b                 mov              eax, dword ptr [rbx + r15]
    0x140be43c1    898510020000             mov              dword ptr [rbp + 0x210], eax
    0x140be43c7    4889bd58020000           mov              qword ptr [rbp + 0x258], rdi
    0x140be43ce    488d05fb7645ff           lea              rax, [rip - 0xba8905] -> _$LT$alloc..borrow..Cow$LT$B$GT$$u20$as$u20$core..fmt..Display$GT$::fmt::h710fd6886fc306ed
    0x140be43d5    48898560020000           mov              qword ptr [rbp + 0x260], rax
    0x140be43dc    488d8510020000           lea              rax, [rbp + 0x210]
    0x140be43e3    48898568020000           mov              qword ptr [rbp + 0x268], rax
    0x140be43ea    488d059fce44ff           lea              rax, [rip - 0xbb3161] -> bevy_reflect::impls::std::_::_$LT$impl$u20$bevy_reflect..reflect..PartialReflect$u20$for$u20$u32$GT$::debug::hf2b8da78b8013204
    0x140be43f1    48898570020000           mov              qword ptr [rbp + 0x270], rax
    0x140be43f8    488d0579edf201           lea              rax, [rip + 0x1f2ed79] -> 0x142b13178
    0x140be43ff    48898520020000           mov              qword ptr [rbp + 0x220], rax
    0x140be4406    48c7852802000002000000   mov              qword ptr [rbp + 0x228], 2
    0x140be4411    488d8558020000           lea              rax, [rbp + 0x258]
    0x140be4418    48898530020000           mov              qword ptr [rbp + 0x230], rax
    0x140be441f    48c7853802000002000000   mov              qword ptr [rbp + 0x238], 2
    0x140be442a    48c7854002000000000000   mov              qword ptr [rbp + 0x240], 0
    0x140be4435    c685af02000001           mov              byte ptr [rbp + 0x2af], 1
    0x140be443c    488dbde0000000           lea              rdi, [rbp + 0xe0]
    0x140be4443    488d9520020000           lea              rdx, [rbp + 0x220]
    0x140be444a    4889f9                   mov              rcx, rdi
    0x140be444d    e83e5b1900               call             0x140d79f90 -> alloc::fmt::format::format_inner::hdcc110f646744b43
    0x140be4452    e93d070000               jmp              0x140be4b94
    0x140be4457    41bd20000000             mov              r13d, 0x20
    0x140be445d    b920000000               mov              ecx, 0x20
    0x140be4462    ba01000000               mov              edx, 1
    0x140be4467    e8a4f7a2ff               call             0x140613c10 -> __rust_alloc
    0x140be446c    4885c0                   test             rax, rax
    0x140be446f    0f84300e0000             je               0x140be52a5
    0x140be4475    0f100508eef201           movups           xmm0, xmmword ptr [rip + 0x1f2ee08] -> 0x142b13284
    0x140be447c    0f114010                 movups           xmmword ptr [rax + 0x10], xmm0
    0x140be4480    0f1005ededf201           movups           xmm0, xmmword ptr [rip + 0x1f2eded] -> 0x142b13274
    0x140be4487    0f1100                   movups           xmmword ptr [rax], xmm0
    0x140be448a    48c7859002000020000000   mov              qword ptr [rbp + 0x290], 0x20
    0x140be4495    48898578020000           mov              qword ptr [rbp + 0x278], rax
    0x140be449c    48898598020000           mov              qword ptr [rbp + 0x298], rax
    0x140be44a3    48c785a002000020000000   mov              qword ptr [rbp + 0x2a0], 0x20
    0x140be44ae    b820000000               mov              eax, 0x20
    0x140be44b3    e9f8080000               jmp              0x140be4db0
    0x140be44b8    b914000000               mov              ecx, 0x14
    0x140be44bd    ba01000000               mov              edx, 1
    0x140be44c2    e849f7a2ff               call             0x140613c10 -> __rust_alloc
    0x140be44c7    4885c0                   test             rax, rax
    0x140be44ca    0f847f0e0000             je               0x140be534f
    0x140be44d0    0f1005d3ecf201           movups           xmm0, xmmword ptr [rip + 0x1f2ecd3] -> 0x142b131aa
    0x140be44d7    0f1100                   movups           xmmword ptr [rax], xmm0
    0x140be44da    c7401066657273           mov              dword ptr [rax + 0x10], 0x73726566
    0x140be44e1    48c7859002000014000000   mov              qword ptr [rbp + 0x290], 0x14
    0x140be44ec    48898598020000           mov              qword ptr [rbp + 0x298], rax
    0x140be44f3    48c785a002000014000000   mov              qword ptr [rbp + 0x2a0], 0x14
    0x140be44fe    c685af02000001           mov              byte ptr [rbp + 0x2af], 1
    0x140be4505    488d15dce9f201           lea              rdx, [rip + 0x1f2e9dc] -> 0x142b12ee8
    0x140be450c    488dbd90020000           lea              rdi, [rbp + 0x290]
    0x140be4513    4889f9                   mov              rcx, rdi
    0x140be4516    e855501900               call             0x140d79570 -> alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::h526e3d19dbc6f270
    0x140be451b    488b8598020000           mov              rax, qword ptr [rbp + 0x298]
    0x140be4522    c640143b                 mov              byte ptr [rax + 0x14], 0x3b
    0x140be4526    48c785a002000015000000   mov              qword ptr [rbp + 0x2a0], 0x15
    0x140be4531    4d8d7c24fb               lea              r15, [r12 - 5]
    0x140be4536    4983fc05                 cmp              r12, 5
    0x140be453a    0f8287060000             jb               0x140be4bc7
    0x140be4540    428b043b                 mov              eax, dword ptr [rbx + r15]
    0x140be4544    898510020000             mov              dword ptr [rbp + 0x210], eax
    0x140be454a    4889bd58020000           mov              qword ptr [rbp + 0x258], rdi
    0x140be4551    488d05787545ff           lea              rax, [rip - 0xba8a88] -> _$LT$alloc..borrow..Cow$LT$B$GT$$u20$as$u20$core..fmt..Display$GT$::fmt::h710fd6886fc306ed
    0x140be4558    48898560020000           mov              qword ptr [rbp + 0x260], rax
    0x140be455f    488d8510020000           lea              rax, [rbp + 0x210]
    0x140be4566    48898568020000           mov              qword ptr [rbp + 0x268], rax
    0x140be456d    488d051ccd44ff           lea              rax, [rip - 0xbb32e4] -> bevy_reflect::impls::std::_::_$LT$impl$u20$bevy_reflect..reflect..PartialReflect$u20$for$u20$u32$GT$::debug::hf2b8da78b8013204
    0x140be4574    48898570020000           mov              qword ptr [rbp + 0x270], rax
    0x140be457b    488d05f6ebf201           lea              rax, [rip + 0x1f2ebf6] -> 0x142b13178
    0x140be4582    48898520020000           mov              qword ptr [rbp + 0x220], rax
    0x140be4589    48c7852802000002000000   mov              qword ptr [rbp + 0x228], 2
    0x140be4594    488d8558020000           lea              rax, [rbp + 0x258]
    0x140be459b    48898530020000           mov              qword ptr [rbp + 0x230], rax
    0x140be45a2    48c7853802000002000000   mov              qword ptr [rbp + 0x238], 2
    0x140be45ad    48c7854002000000000000   mov              qword ptr [rbp + 0x240], 0
    0x140be45b8    c685af02000001           mov              byte ptr [rbp + 0x2af], 1
    0x140be45bf    488dbd80000000           lea              rdi, [rbp + 0x80]
    0x140be45c6    488d9520020000           lea              rdx, [rbp + 0x220]
    0x140be45cd    4889f9                   mov              rcx, rdi
    0x140be45d0    e8bb591900               call             0x140d79f90 -> alloc::fmt::format::format_inner::hdcc110f646744b43
    0x140be45d5    e9ba050000               jmp              0x140be4b94
    0x140be45da    b90f000000               mov              ecx, 0xf
    0x140be45df    ba01000000               mov              edx, 1
    0x140be45e4    e827f6a2ff               call             0x140613c10 -> __rust_alloc
    0x140be45e9    4885c0                   test             rax, rax
    0x140be45ec    0f84a30b0000             je               0x140be5195
    0x140be45f2    48b95f62756666657273     movabs           rcx, 0x737265666675625f
    0x140be45fc    48894807                 mov              qword ptr [rax + 7], rcx
    0x140be4600    48b964657374726f795f     movabs           rcx, 0x5f796f7274736564
    0x140be460a    488908                   mov              qword ptr [rax], rcx
    0x140be460d    48c785900200000f000000   mov              qword ptr [rbp + 0x290], 0xf
    0x140be4618    48898598020000           mov              qword ptr [rbp + 0x298], rax
    0x140be461f    48c785a00200000f000000   mov              qword ptr [rbp + 0x2a0], 0xf
    0x140be462a    c685af02000001           mov              byte ptr [rbp + 0x2af], 1
    0x140be4631    488d15b0e8f201           lea              rdx, [rip + 0x1f2e8b0] -> 0x142b12ee8
    0x140be4638    488dbd90020000           lea              rdi, [rbp + 0x290]
    0x140be463f    4889f9                   mov              rcx, rdi
    0x140be4642    e8294f1900               call             0x140d79570 -> alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::h526e3d19dbc6f270
    0x140be4647    488b8598020000           mov              rax, qword ptr [rbp + 0x298]
    0x140be464e    c6400f3b                 mov              byte ptr [rax + 0xf], 0x3b
    0x140be4652    48c785a002000010000000   mov              qword ptr [rbp + 0x2a0], 0x10
    0x140be465d    4d8d7c24fb               lea              r15, [r12 - 5]
    0x140be4662    4983fc05                 cmp              r12, 5
    0x140be4666    0f825b050000             jb               0x140be4bc7
    0x140be466c    428b043b                 mov              eax, dword ptr [rbx + r15]
    0x140be4670    898510020000             mov              dword ptr [rbp + 0x210], eax
    0x140be4676    4889bd58020000           mov              qword ptr [rbp + 0x258], rdi
    0x140be467d    488d054c7445ff           lea              rax, [rip - 0xba8bb4] -> _$LT$alloc..borrow..Cow$LT$B$GT$$u20$as$u20$core..fmt..Display$GT$::fmt::h710fd6886fc306ed
    0x140be4684    48898560020000           mov              qword ptr [rbp + 0x260], rax
    0x140be468b    488d8510020000           lea              rax, [rbp + 0x210]
    0x140be4692    48898568020000           mov              qword ptr [rbp + 0x268], rax
    0x140be4699    488d05f0cb44ff           lea              rax, [rip - 0xbb3410] -> bevy_reflect::impls::std::_::_$LT$impl$u20$bevy_reflect..reflect..PartialReflect$u20$for$u20$u32$GT$::debug::hf2b8da78b8013204
    0x140be46a0    48898570020000           mov              qword ptr [rbp + 0x270], rax
    0x140be46a7    488d05caeaf201           lea              rax, [rip + 0x1f2eaca] -> 0x142b13178
    0x140be46ae    48898520020000           mov              qword ptr [rbp + 0x220], rax
    0x140be46b5    48c7852802000002000000   mov              qword ptr [rbp + 0x228], 2
    0x140be46c0    488d8558020000           lea              rax, [rbp + 0x258]
    0x140be46c7    48898530020000           mov              qword ptr [rbp + 0x230], rax
    0x140be46ce    48c7853802000002000000   mov              qword ptr [rbp + 0x238], 2
    0x140be46d9    48c7854002000000000000   mov              qword ptr [rbp + 0x240], 0
    0x140be46e4    c685af02000001           mov              byte ptr [rbp + 0x2af], 1
    0x140be46eb    488dbdb0000000           lea              rdi, [rbp + 0xb0]
    0x140be46f2    488d9520020000           lea              rdx, [rbp + 0x220]
    0x140be46f9    4889f9                   mov              rcx, rdi
    0x140be46fc    e88f581900               call             0x140d79f90 -> alloc::fmt::format::format_inner::hdcc110f646744b43
    0x140be4701    e98e040000               jmp              0x140be4b94
    0x140be4706    b917000000               mov              ecx, 0x17
    0x140be470b    ba01000000               mov              edx, 1
    0x140be4710    e8fbf4a2ff               call             0x140613c10 -> __rust_alloc
    0x140be4715    4885c0                   test             rax, rax
    0x140be4718    0f8467090000             je               0x140be5085
    0x140be471e    0f10051fecf201           movups           xmm0, xmmword ptr [rip + 0x1f2ec1f] -> 0x142b13344
    0x140be4725    0f1100                   movups           xmmword ptr [rax], xmm0
    0x140be4728    48b9696e5f6f72646572     movabs           rcx, 0x726564726f5f6e69
    0x140be4732    4889480f                 mov              qword ptr [rax + 0xf], rcx
    0x140be4736    48c7859002000017000000   mov              qword ptr [rbp + 0x290], 0x17
    0x140be4741    48898598020000           mov              qword ptr [rbp + 0x298], rax
    0x140be4748    48c785a002000017000000   mov              qword ptr [rbp + 0x2a0], 0x17
    0x140be4753    c685af02000001           mov              byte ptr [rbp + 0x2af], 1
    0x140be475a    488d1587e7f201           lea              rdx, [rip + 0x1f2e787] -> 0x142b12ee8
    0x140be4761    488dbd90020000           lea              rdi, [rbp + 0x290]
    0x140be4768    4889f9                   mov              rcx, rdi
    0x140be476b    e8004e1900               call             0x140d79570 -> alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::h526e3d19dbc6f270
    0x140be4770    488b8598020000           mov              rax, qword ptr [rbp + 0x298]
    0x140be4777    c640173b                 mov              byte ptr [rax + 0x17], 0x3b
    0x140be477b    48c785a002000018000000   mov              qword ptr [rbp + 0x2a0], 0x18
    0x140be4786    4d8d7c24f7               lea              r15, [r12 - 9]
    0x140be478b    4983fc09                 cmp              r12, 9
    0x140be478f    0f8232040000             jb               0x140be4bc7
    0x140be4795    4a8b043b                 mov              rax, qword ptr [rbx + r15]
    0x140be4799    48898510020000           mov              qword ptr [rbp + 0x210], rax
    0x140be47a0    4889bd58020000           mov              qword ptr [rbp + 0x258], rdi
    0x140be47a7    488d05227345ff           lea              rax, [rip - 0xba8cde] -> _$LT$alloc..borrow..Cow$LT$B$GT$$u20$as$u20$core..fmt..Display$GT$::fmt::h710fd6886fc306ed
    0x140be47ae    48898560020000           mov              qword ptr [rbp + 0x260], rax
    0x140be47b5    488d8510020000           lea              rax, [rbp + 0x210]
    0x140be47bc    48898568020000           mov              qword ptr [rbp + 0x268], rax
    0x140be47c3    488d05f63c4eff           lea              rax, [rip - 0xb1c30a] -> core::fmt::num::_$LT$impl$u20$core..fmt..Debug$u20$for$u20$usize$GT$::fmt::h8e88e8dbb6030fc5 (.llvm.4519547824212384754)
    0x140be47ca    48898570020000           mov              qword ptr [rbp + 0x270], rax
    0x140be47d1    488d0590ebf201           lea              rax, [rip + 0x1f2eb90] -> 0x142b13368
    0x140be47d8    48898520020000           mov              qword ptr [rbp + 0x220], rax
    0x140be47df    48c7852802000002000000   mov              qword ptr [rbp + 0x228], 2
    0x140be47ea    488d8558020000           lea              rax, [rbp + 0x258]
    0x140be47f1    48898530020000           mov              qword ptr [rbp + 0x230], rax
    0x140be47f8    48c7853802000002000000   mov              qword ptr [rbp + 0x238], 2
    0x140be4803    48c7854002000000000000   mov              qword ptr [rbp + 0x240], 0
    0x140be480e    c685af02000001           mov              byte ptr [rbp + 0x2af], 1
    0x140be4815    488d7dd8                 lea              rdi, [rbp - 0x28]
    0x140be4819    488d9520020000           lea              rdx, [rbp + 0x220]
    0x140be4820    4889f9                   mov              rcx, rdi
    0x140be4823    e868571900               call             0x140d79f90 -> alloc::fmt::format::format_inner::hdcc110f646744b43
    0x140be4828    e967030000               jmp              0x140be4b94
    0x140be482d    41bd0d000000             mov              r13d, 0xd
    0x140be4833    b90d000000               mov              ecx, 0xd
    0x140be4838    ba01000000               mov              edx, 1
    0x140be483d    e8cef3a2ff               call             0x140613c10 -> __rust_alloc
    0x140be4842    4885c0                   test             rax, rax
    0x140be4845    0f846a0b0000             je               0x140be53b5
    0x140be484b    48b96e745f696d616765     movabs           rcx, 0x6567616d695f746e
    0x140be4855    48894805                 mov              qword ptr [rax + 5], rcx
    0x140be4859    48b970726573656e745f     movabs           rcx, 0x5f746e6573657270
    0x140be4863    488908                   mov              qword ptr [rax], rcx
    0x140be4866    48c785900200000d000000   mov              qword ptr [rbp + 0x290], 0xd
    0x140be4871    48898578020000           mov              qword ptr [rbp + 0x278], rax
    0x140be4878    48898598020000           mov              qword ptr [rbp + 0x298], rax
    0x140be487f    48c785a00200000d000000   mov              qword ptr [rbp + 0x2a0], 0xd
    0x140be488a    b80d000000               mov              eax, 0xd
    0x140be488f    e91c050000               jmp              0x140be4db0
    0x140be4894    41bd1a000000             mov              r13d, 0x1a
    0x140be489a    b91a000000               mov              ecx, 0x1a
    0x140be489f    ba01000000               mov              edx, 1
    0x140be48a4    e867f3a2ff               call             0x140613c10 -> __rust_alloc
    0x140be48a9    4885c0                   test             rax, rax
    0x140be48ac    0f84570c0000             je               0x140be5509
    0x140be48b2    0f1005ceebf201           movups           xmm0, xmmword ptr [rip + 0x1f2ebce] -> 0x142b13487
    0x140be48b9    0f11400a                 movups           xmmword ptr [rax + 0xa], xmm0
    0x140be48bd    0f1005b9ebf201           movups           xmm0, xmmword ptr [rip + 0x1f2ebb9] -> 0x142b1347d
    0x140be48c4    0f1100                   movups           xmmword ptr [rax], xmm0
    0x140be48c7    48c785900200001a000000   mov              qword ptr [rbp + 0x290], 0x1a
    0x140be48d2    48898578020000           mov              qword ptr [rbp + 0x278], rax
    0x140be48d9    48898598020000           mov              qword ptr [rbp + 0x298], rax
    0x140be48e0    48c785a00200001a000000   mov              qword ptr [rbp + 0x2a0], 0x1a
    0x140be48eb    b81a000000               mov              eax, 0x1a
    0x140be48f0    e9bb040000               jmp              0x140be4db0
    0x140be48f5    41bd14000000             mov              r13d, 0x14
    0x140be48fb    b914000000               mov              ecx, 0x14
    0x140be4900    ba01000000               mov              edx, 1
    0x140be4905    e806f3a2ff               call             0x140613c10 -> __rust_alloc
    0x140be490a    4885c0                   test             rax, rax
    0x140be490d    0f84c8060000             je               0x140be4fdb
    0x140be4913    0f1005bce9f201           movups           xmm0, xmmword ptr [rip + 0x1f2e9bc] -> 0x142b132d6
    0x140be491a    e90c030000               jmp              0x140be4c2b
    0x140be491f    41bd10000000             mov              r13d, 0x10
    0x140be4925    b910000000               mov              ecx, 0x10
    0x140be492a    ba01000000               mov              edx, 1
    0x140be492f    e8dcf2a2ff               call             0x140613c10 -> __rust_alloc
    0x140be4934    4885c0                   test             rax, rax
    0x140be4937    0f846a0c0000             je               0x140be55a7
    0x140be493d    0f1005a4eaf201           movups           xmm0, xmmword ptr [rip + 0x1f2eaa4] -> 0x142b133e8
    0x140be4944    e93f030000               jmp              0x140be4c88
    0x140be4949    41bd17000000             mov              r13d, 0x17
    0x140be494f    b917000000               mov              ecx, 0x17
    0x140be4954    ba01000000               mov              edx, 1
    0x140be4959    e8b2f2a2ff               call             0x140613c10 -> __rust_alloc
    0x140be495e    4885c0                   test             rax, rax
    0x140be4961    0f841a0b0000             je               0x140be5481
    0x140be4967    0f100526e9f201           movups           xmm0, xmmword ptr [rip + 0x1f2e926] -> 0x142b13294
    0x140be496e    0f1100                   movups           xmmword ptr [rax], xmm0
    0x140be4971    48b9706970656c696e65     movabs           rcx, 0x656e696c65706970
    0x140be497b    4889480f                 mov              qword ptr [rax + 0xf], rcx
    0x140be497f    48c7859002000017000000   mov              qword ptr [rbp + 0x290], 0x17
    0x140be498a    48898578020000           mov              qword ptr [rbp + 0x278], rax
    0x140be4991    48898598020000           mov              qword ptr [rbp + 0x298], rax
    0x140be4998    48c785a002000017000000   mov              qword ptr [rbp + 0x2a0], 0x17
    0x140be49a3    b817000000               mov              eax, 0x17
    0x140be49a8    e903040000               jmp              0x140be4db0
    0x140be49ad    b90d000000               mov              ecx, 0xd
    0x140be49b2    ba01000000               mov              edx, 1
    0x140be49b7    e854f2a2ff               call             0x140613c10 -> __rust_alloc
    0x140be49bc    4885c0                   test             rax, rax
    0x140be49bf    0f8402090000             je               0x140be52c7
    0x140be49c5    48b95f72675f70617373     movabs           rcx, 0x737361705f67725f
    0x140be49cf    48894805                 mov              qword ptr [rax + 5], rcx
    0x140be49d3    48b9626567696e5f7267     movabs           rcx, 0x67725f6e69676562
    0x140be49dd    488908                   mov              qword ptr [rax], rcx
    0x140be49e0    48c785900200000d000000   mov              qword ptr [rbp + 0x290], 0xd
    0x140be49eb    48898598020000           mov              qword ptr [rbp + 0x298], rax
    0x140be49f2    48c785a00200000d000000   mov              qword ptr [rbp + 0x2a0], 0xd
    0x140be49fd    c685af02000001           mov              byte ptr [rbp + 0x2af], 1
    0x140be4a04    488d15dde4f201           lea              rdx, [rip + 0x1f2e4dd] -> 0x142b12ee8
    0x140be4a0b    488dbd90020000           lea              rdi, [rbp + 0x290]
    0x140be4a12    4889f9                   mov              rcx, rdi
    0x140be4a15    e8564b1900               call             0x140d79570 -> alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::h526e3d19dbc6f270
    0x140be4a1a    488b8598020000           mov              rax, qword ptr [rbp + 0x298]
    0x140be4a21    c6400d3b                 mov              byte ptr [rax + 0xd], 0x3b
    0x140be4a25    48c785a00200000e000000   mov              qword ptr [rbp + 0x2a0], 0xe
    0x140be4a30    4d8d7c24eb               lea              r15, [r12 - 0x15]
    0x140be4a35    4983fc15                 cmp              r12, 0x15
    0x140be4a39    0f8288010000             jb               0x140be4bc7
    0x140be4a3f    420f10043b               movups           xmm0, xmmword ptr [rbx + r15]
    0x140be4a44    0f298510020000           movaps           xmmword ptr [rbp + 0x210], xmm0
    0x140be4a4b    4889bd58020000           mov              qword ptr [rbp + 0x258], rdi
    0x140be4a52    488d05777045ff           lea              rax, [rip - 0xba8f89] -> _$LT$alloc..borrow..Cow$LT$B$GT$$u20$as$u20$core..fmt..Display$GT$::fmt::h710fd6886fc306ed
    0x140be4a59    48898560020000           mov              qword ptr [rbp + 0x260], rax
    0x140be4a60    4c8dad10020000           lea              r13, [rbp + 0x210]
    0x140be4a67    4c89ad68020000           mov              qword ptr [rbp + 0x268], r13
    0x140be4a6e    488d059bdcffff           lea              rax, [rip - 0x2365] -> rhapsody::leek::impl$0::fmt
    0x140be4a75    48898570020000           mov              qword ptr [rbp + 0x270], rax
    0x140be4a7c    488d0575e6f201           lea              rax, [rip + 0x1f2e675] -> 0x142b130f8
    0x140be4a83    48898520020000           mov              qword ptr [rbp + 0x220], rax
    0x140be4a8a    48c7852802000002000000   mov              qword ptr [rbp + 0x228], 2
    0x140be4a95    4c8da558020000           lea              r12, [rbp + 0x258]
    0x140be4a9c    4c89a530020000           mov              qword ptr [rbp + 0x230], r12
    0x140be4aa3    48c7853802000002000000   mov              qword ptr [rbp + 0x238], 2
    0x140be4aae    48c7854002000000000000   mov              qword ptr [rbp + 0x240], 0
    0x140be4ab9    c685af02000001           mov              byte ptr [rbp + 0x2af], 1
    0x140be4ac0    488d8d70010000           lea              rcx, [rbp + 0x170]
    0x140be4ac7    488d9520020000           lea              rdx, [rbp + 0x220]
    0x140be4ace    e8bd541900               call             0x140d79f90 -> alloc::fmt::format::format_inner::hdcc110f646744b43
    0x140be4ad3    488b9590020000           mov              rdx, qword ptr [rbp + 0x290]
    0x140be4ada    4885d2                   test             rdx, rdx
    0x140be4add    7412                     je               0x140be4af1
    0x140be4adf    488b8d98020000           mov              rcx, qword ptr [rbp + 0x298]
    0x140be4ae6    41b801000000             mov              r8d, 1
    0x140be4aec    e82ff1a2ff               call             0x140613c20 -> __rust_dealloc
    0x140be4af1    4a8d043b                 lea              rax, [rbx + r15]
    0x140be4af5    488b8d80010000           mov              rcx, qword ptr [rbp + 0x180]
    0x140be4afc    48898da0020000           mov              qword ptr [rbp + 0x2a0], rcx
    0x140be4b03    0f108570010000           movups           xmm0, xmmword ptr [rbp + 0x170]
    0x140be4b0a    0f298590020000           movaps           xmmword ptr [rbp + 0x290], xmm0
    0x140be4b11    8b4010                   mov              eax, dword ptr [rax + 0x10]
    0x140be4b14    898510020000             mov              dword ptr [rbp + 0x210], eax
    0x140be4b1a    4889bd58020000           mov              qword ptr [rbp + 0x258], rdi
    0x140be4b21    488d05a86f45ff           lea              rax, [rip - 0xba9058] -> _$LT$alloc..borrow..Cow$LT$B$GT$$u20$as$u20$core..fmt..Display$GT$::fmt::h710fd6886fc306ed
    0x140be4b28    48898560020000           mov              qword ptr [rbp + 0x260], rax
    0x140be4b2f    4c89ad68020000           mov              qword ptr [rbp + 0x268], r13
    0x140be4b36    488d0553c744ff           lea              rax, [rip - 0xbb38ad] -> bevy_reflect::impls::std::_::_$LT$impl$u20$bevy_reflect..reflect..PartialReflect$u20$for$u20$u32$GT$::debug::hf2b8da78b8013204
    0x140be4b3d    48898570020000           mov              qword ptr [rbp + 0x270], rax
    0x140be4b44    488d05fde8f201           lea              rax, [rip + 0x1f2e8fd] -> 0x142b13448
    0x140be4b4b    48898520020000           mov              qword ptr [rbp + 0x220], rax
    0x140be4b52    48c7852802000002000000   mov              qword ptr [rbp + 0x228], 2
    0x140be4b5d    4c89a530020000           mov              qword ptr [rbp + 0x230], r12
    0x140be4b64    48c7853802000002000000   mov              qword ptr [rbp + 0x238], 2
    0x140be4b6f    48c7854002000000000000   mov              qword ptr [rbp + 0x240], 0
    0x140be4b7a    c685af02000001           mov              byte ptr [rbp + 0x2af], 1
    0x140be4b81    488d7da8                 lea              rdi, [rbp - 0x58]
    0x140be4b85    488d9520020000           lea              rdx, [rbp + 0x220]
    0x140be4b8c    4889f9                   mov              rcx, rdi
    0x140be4b8f    e8fc531900               call             0x140d79f90 -> alloc::fmt::format::format_inner::hdcc110f646744b43
    0x140be4b94    488b9590020000           mov              rdx, qword ptr [rbp + 0x290]
    0x140be4b9b    4885d2                   test             rdx, rdx
    0x140be4b9e    7412                     je               0x140be4bb2
    0x140be4ba0    488b8d98020000           mov              rcx, qword ptr [rbp + 0x298]
    0x140be4ba7    41b801000000             mov              r8d, 1
    0x140be4bad    e86ef0a2ff               call             0x140613c20 -> __rust_dealloc
    0x140be4bb2    488b4710                 mov              rax, qword ptr [rdi + 0x10]
    0x140be4bb6    488985a0020000           mov              qword ptr [rbp + 0x2a0], rax
    0x140be4bbd    0f1007                   movups           xmm0, xmmword ptr [rdi]
    0x140be4bc0    0f298590020000           movaps           xmmword ptr [rbp + 0x290], xmm0
    0x140be4bc7    488b8590020000           mov              rax, qword ptr [rbp + 0x290]
    0x140be4bce    48898580020000           mov              qword ptr [rbp + 0x280], rax
    0x140be4bd5    488b8598020000           mov              rax, qword ptr [rbp + 0x298]
    0x140be4bdc    48898578020000           mov              qword ptr [rbp + 0x278], rax
    0x140be4be3    4c8bada0020000           mov              r13, qword ptr [rbp + 0x2a0]
    0x140be4bea    4d89fc                   mov              r12, r15
    0x140be4bed    488bbd00020000           mov              rdi, qword ptr [rbp + 0x200]
    0x140be4bf4    483bbdf0010000           cmp              rdi, qword ptr [rbp + 0x1f0]
    0x140be4bfb    0f85afdfffff             jne              0x140be2bb0
    0x140be4c01    e9c8010000               jmp              0x140be4dce
    0x140be4c06    41bd14000000             mov              r13d, 0x14
    0x140be4c0c    b914000000               mov              ecx, 0x14
    0x140be4c11    ba01000000               mov              edx, 1
    0x140be4c16    e8f5efa2ff               call             0x140613c10 -> __rust_alloc
    0x140be4c1b    4885c0                   test             rax, rax
    0x140be4c1e    0f847f080000             je               0x140be54a3
    0x140be4c24    0f1005bfe6f201           movups           xmm0, xmmword ptr [rip + 0x1f2e6bf] -> 0x142b132ea
    0x140be4c2b    0f1100                   movups           xmmword ptr [rax], xmm0
    0x140be4c2e    c7401066666572           mov              dword ptr [rax + 0x10], 0x72656666
    0x140be4c35    48c7859002000014000000   mov              qword ptr [rbp + 0x290], 0x14
    0x140be4c40    48898578020000           mov              qword ptr [rbp + 0x278], rax
    0x140be4c47    48898598020000           mov              qword ptr [rbp + 0x298], rax
    0x140be4c4e    48c785a002000014000000   mov              qword ptr [rbp + 0x2a0], 0x14
    0x140be4c59    b814000000               mov              eax, 0x14
    0x140be4c5e    e94d010000               jmp              0x140be4db0
    0x140be4c63    41bd10000000             mov              r13d, 0x10
    0x140be4c69    b910000000               mov              ecx, 0x10
    0x140be4c6e    ba01000000               mov              edx, 1
    0x140be4c73    e898efa2ff               call             0x140613c10 -> __rust_alloc
    0x140be4c78    4885c0                   test             rax, rax
    0x140be4c7b    0f84ae040000             je               0x140be512f
    0x140be4c81    0f100599e6f201           movups           xmm0, xmmword ptr [rip + 0x1f2e699] -> 0x142b13321
    0x140be4c88    0f1100                   movups           xmmword ptr [rax], xmm0
    0x140be4c8b    48c7859002000010000000   mov              qword ptr [rbp + 0x290], 0x10
    0x140be4c96    48898578020000           mov              qword ptr [rbp + 0x278], rax
    0x140be4c9d    48898598020000           mov              qword ptr [rbp + 0x298], rax
    0x140be4ca4    48c785a002000010000000   mov              qword ptr [rbp + 0x2a0], 0x10
    0x140be4caf    b810000000               mov              eax, 0x10
    0x140be4cb4    e9f7000000               jmp              0x140be4db0
    0x140be4cb9    41bd13000000             mov              r13d, 0x13
    0x140be4cbf    b913000000               mov              ecx, 0x13
    0x140be4cc4    ba01000000               mov              edx, 1
    0x140be4cc9    e842efa2ff               call             0x140613c10 -> __rust_alloc
    0x140be4cce    4885c0                   test             rax, rax
    0x140be4cd1    0f8454080000             je               0x140be552b
    0x140be4cd7    0f100553e6f201           movups           xmm0, xmmword ptr [rip + 0x1f2e653] -> 0x142b13331
    0x140be4cde    0f1100                   movups           xmmword ptr [rax], xmm0
    0x140be4ce1    c7400f7472696d           mov              dword ptr [rax + 0xf], 0x6d697274
    0x140be4ce8    48c7859002000013000000   mov              qword ptr [rbp + 0x290], 0x13
    0x140be4cf3    48898578020000           mov              qword ptr [rbp + 0x278], rax
    0x140be4cfa    48898598020000           mov              qword ptr [rbp + 0x298], rax
    0x140be4d01    48c785a002000013000000   mov              qword ptr [rbp + 0x2a0], 0x13
    0x140be4d0c    b813000000               mov              eax, 0x13
    0x140be4d11    e99a000000               jmp              0x140be4db0
    0x140be4d16    41bd0f000000             mov              r13d, 0xf
    0x140be4d1c    b90f000000               mov              ecx, 0xf
    0x140be4d21    ba01000000               mov              edx, 1
    0x140be4d26    e8e5eea2ff               call             0x140613c10 -> __rust_alloc
    0x140be4d2b    4885c0                   test             rax, rax
    0x140be4d2e    0f84b5050000             je               0x140be52e9
    0x140be4d34    48b95f72657469726564     movabs           rcx, 0x646572697465725f
    0x140be4d3e    48894807                 mov              qword ptr [rax + 7], rcx
    0x140be4d42    48b970726573656e745f     movabs           rcx, 0x5f746e6573657270
    0x140be4d4c    eb36                     jmp              0x140be4d84
    0x140be4d4e    41bd0f000000             mov              r13d, 0xf
    0x140be4d54    b90f000000               mov              ecx, 0xf
    0x140be4d59    ba01000000               mov              edx, 1
    0x140be4d5e    e8adeea2ff               call             0x140613c10 -> __rust_alloc
    0x140be4d63    4885c0                   test             rax, rax
    0x140be4d66    0f844b040000             je               0x140be51b7
    0x140be4d6c    48b9655f7375626d6974     movabs           rcx, 0x74696d6275735f65
    0x140be4d76    48894807                 mov              qword ptr [rax + 7], rcx
    0x140be4d7a    48b9766b5f7175657565     movabs           rcx, 0x65756575715f6b76
    0x140be4d84    488908                   mov              qword ptr [rax], rcx
    0x140be4d87    48c785900200000f000000   mov              qword ptr [rbp + 0x290], 0xf
    0x140be4d92    48898578020000           mov              qword ptr [rbp + 0x278], rax
    0x140be4d99    48898598020000           mov              qword ptr [rbp + 0x298], rax
    0x140be4da0    48c785a00200000f000000   mov              qword ptr [rbp + 0x2a0], 0xf
    0x140be4dab    b80f000000               mov              eax, 0xf
    0x140be4db0    48898580020000           mov              qword ptr [rbp + 0x280], rax
    0x140be4db7    4989fc                   mov              r12, rdi
    0x140be4dba    488bbd00020000           mov              rdi, qword ptr [rbp + 0x200]
    0x140be4dc1    483bbdf0010000           cmp              rdi, qword ptr [rbp + 0x1f0]
    0x140be4dc8    0f85e2ddffff             jne              0x140be2bb0
    0x140be4dce    488d159be1f201           lea              rdx, [rip + 0x1f2e19b] -> 0x142b12f70
    0x140be4dd5    488d8df0010000           lea              rcx, [rbp + 0x1f0]
    0x140be4ddc    e8cfdf97ff               call             0x140562db0 -> alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::h015eb00fc5e65d14
    0x140be4de1    e9caddffff               jmp              0x140be2bb0
    0x140be4de6    488bb500020000           mov              rsi, qword ptr [rbp + 0x200]
    0x140be4ded    4883fe02                 cmp              rsi, 2
    0x140be4df1    0f83f5000000             jae              0x140be4eec
    0x140be4df7    e93d010000               jmp              0x140be4f39
    0x140be4dfc    488d850f020000           lea              rax, [rbp + 0x20f]
    0x140be4e03    48898558020000           mov              qword ptr [rbp + 0x258], rax
    0x140be4e0a    488d058f831a00           lea              rax, [rip + 0x1a838f] -> core::fmt::num::imp::_$LT$impl$u20$core..fmt..Display$u20$for$u20$u8$GT$::fmt::h7c43b6fa0d5163f5
    0x140be4e11    48898560020000           mov              qword ptr [rbp + 0x260], rax
    0x140be4e18    488d0591e6f201           lea              rax, [rip + 0x1f2e691] -> 0x142b134b0
    0x140be4e1f    48898520020000           mov              qword ptr [rbp + 0x220], rax
    0x140be4e26    48c7852802000001000000   mov              qword ptr [rbp + 0x228], 1
    0x140be4e31    488d8558020000           lea              rax, [rbp + 0x258]
    0x140be4e38    48898530020000           mov              qword ptr [rbp + 0x230], rax
    0x140be4e3f    48c7853802000001000000   mov              qword ptr [rbp + 0x238], 1
    0x140be4e4a    48c7854002000000000000   mov              qword ptr [rbp + 0x240], 0
    0x140be4e55    c6858f02000001           mov              byte ptr [rbp + 0x28f], 1
    0x140be4e5c    488d8dd0010000           lea              rcx, [rbp + 0x1d0]
    0x140be4e63    488d9520020000           lea              rdx, [rbp + 0x220]
    0x140be4e6a    e821511900               call             0x140d79f90 -> alloc::fmt::format::format_inner::hdcc110f646744b43
    0x140be4e6f    488b85d0010000           mov              rax, qword ptr [rbp + 0x1d0]
    0x140be4e76    48898580020000           mov              qword ptr [rbp + 0x280], rax
    0x140be4e7d    488b85d8010000           mov              rax, qword ptr [rbp + 0x1d8]
    0x140be4e84    48898578020000           mov              qword ptr [rbp + 0x278], rax
    0x140be4e8b    488bbde0010000           mov              rdi, qword ptr [rbp + 0x1e0]
    0x140be4e92    488bb500020000           mov              rsi, qword ptr [rbp + 0x200]
    0x140be4e99    483bb5f0010000           cmp              rsi, qword ptr [rbp + 0x1f0]
    0x140be4ea0    7513                     jne              0x140be4eb5
    0x140be4ea2    488d15c7e0f201           lea              rdx, [rip + 0x1f2e0c7] -> 0x142b12f70
    0x140be4ea9    488d8df0010000           lea              rcx, [rbp + 0x1f0]
    0x140be4eb0    e8fbde97ff               call             0x140562db0 -> alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::h015eb00fc5e65d14
    0x140be4eb5    488b85f8010000           mov              rax, qword ptr [rbp + 0x1f8]
    0x140be4ebc    488d0c76                 lea              rcx, [rsi + rsi*2]
    0x140be4ec0    488b9580020000           mov              rdx, qword ptr [rbp + 0x280]
    0x140be4ec7    488914c8                 mov              qword ptr [rax + rcx*8], rdx
    0x140be4ecb    488b9578020000           mov              rdx, qword ptr [rbp + 0x278]
    0x140be4ed2    488954c808               mov              qword ptr [rax + rcx*8 + 8], rdx
    0x140be4ed7    48897cc810               mov              qword ptr [rax + rcx*8 + 0x10], rdi
    0x140be4edc    48ffc6                   inc              rsi
    0x140be4edf    4889b500020000           mov              qword ptr [rbp + 0x200], rsi
    0x140be4ee6    4883fe02                 cmp              rsi, 2
    0x140be4eea    724d                     jb               0x140be4f39
    0x140be4eec    488b85f8010000           mov              rax, qword ptr [rbp + 0x1f8]
    0x140be4ef3    488d0c76                 lea              rcx, [rsi + rsi*2]
    0x140be4ef7    48d1ee                   shr              rsi, 1
    0x140be4efa    488d0cc8                 lea              rcx, [rax + rcx*8]
    0x140be4efe    4883c1f8                 add              rcx, -8
    0x140be4f02    4883c010                 add              rax, 0x10
    0x140be4f06    662e0f1f840000000000     nop              word ptr cs:[rax + rax]
    0x140be4f10    0f1040f0                 movups           xmm0, xmmword ptr [rax - 0x10]
    0x140be4f14    0f1049f0                 movups           xmm1, xmmword ptr [rcx - 0x10]
    0x140be4f18    0f1148f0                 movups           xmmword ptr [rax - 0x10], xmm1
    0x140be4f1c    0f1141f0                 movups           xmmword ptr [rcx - 0x10], xmm0
    0x140be4f20    488b10                   mov              rdx, qword ptr [rax]
    0x140be4f23    4c8b01                   mov              r8, qword ptr [rcx]
    0x140be4f26    4c8900                   mov              qword ptr [rax], r8
    0x140be4f29    488911                   mov              qword ptr [rcx], rdx
    0x140be4f2c    4883c018                 add              rax, 0x18
    0x140be4f30    4883c1e8                 add              rcx, -0x18
    0x140be4f34    48ffce                   dec              rsi
    0x140be4f37    75d7                     jne              0x140be4f10
    0x140be4f39    488b8500020000           mov              rax, qword ptr [rbp + 0x200]
    0x140be4f40    48898530020000           mov              qword ptr [rbp + 0x230], rax
    0x140be4f47    0f1085f0010000           movups           xmm0, xmmword ptr [rbp + 0x1f0]
    0x140be4f4e    0f298520020000           movaps           xmmword ptr [rbp + 0x220], xmm0
    0x140be4f55    c6858f02000000           mov              byte ptr [rbp + 0x28f], 0
    0x140be4f5c    488d8d20020000           lea              rcx, [rbp + 0x220]
    0x140be4f63    e888c8cd00               call             0x1418c17f0 -> leekspin::print_deduplicated_with_count::hd0573b0542a96de5
    0x140be4f68    488b85e8010000           mov              rax, qword ptr [rbp + 0x1e8]
    0x140be4f6f    488b10                   mov              rdx, qword ptr [rax]
    0x140be4f72    4885d2                   test             rdx, rdx
    0x140be4f75    740f                     je               0x140be4f86
    0x140be4f77    488b4808                 mov              rcx, qword ptr [rax + 8]
    0x140be4f7b    41b801000000             mov              r8d, 1
    0x140be4f81    e89aeca2ff               call             0x140613c20 -> __rust_dealloc
    0x140be4f86    90                       nop              
    0x140be4f87    4881c438030000           add              rsp, 0x338
    0x140be4f8e    5b                       pop              rbx
    0x140be4f8f    5f                       pop              rdi
    0x140be4f90    5e                       pop              rsi
    0x140be4f91    415c                     pop              r12
    0x140be4f93    415d                     pop              r13
    0x140be4f95    415e                     pop              r14
    0x140be4f97    415f                     pop              r15
    0x140be4f99    5d                       pop              rbp
    0x140be4f9a    c3                       ret              
    0x140be4f9b    c6858f02000001           mov              byte ptr [rbp + 0x28f], 1
    0x140be4fa2    4c8d05c7dff201           lea              r8, [rip + 0x1f2dfc7] -> 0x142b12f70
    0x140be4fa9    4889f9                   mov              rcx, rdi
    0x140be4fac    4889f2                   mov              rdx, rsi
    0x140be4faf    e86445cf01               call             0x1428d9518 -> core::panicking::panic_bounds_check::h213dd6812833a70b
    0x140be4fb4    e90b060000               jmp              0x140be55c4
    0x140be4fb9    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be4fc0    4c8d05b9def201           lea              r8, [rip + 0x1f2deb9] -> 0x142b12e80
    0x140be4fc7    b901000000               mov              ecx, 1
    0x140be4fcc    ba0b000000               mov              edx, 0xb
    0x140be4fd1    e80d40cf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be4fd6    e9e9050000               jmp              0x140be55c4
    0x140be4fdb    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be4fe2    4c8d0597def201           lea              r8, [rip + 0x1f2de97] -> 0x142b12e80
    0x140be4fe9    b901000000               mov              ecx, 1
    0x140be4fee    ba14000000               mov              edx, 0x14
    0x140be4ff3    e8eb3fcf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be4ff8    e9c7050000               jmp              0x140be55c4
    0x140be4ffd    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be5004    4c8d0575def201           lea              r8, [rip + 0x1f2de75] -> 0x142b12e80
    0x140be500b    b901000000               mov              ecx, 1
    0x140be5010    ba11000000               mov              edx, 0x11
    0x140be5015    e8c93fcf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be501a    e9a5050000               jmp              0x140be55c4
    0x140be501f    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be5026    4c8d0553def201           lea              r8, [rip + 0x1f2de53] -> 0x142b12e80
    0x140be502d    b901000000               mov              ecx, 1
    0x140be5032    ba10000000               mov              edx, 0x10
    0x140be5037    e8a73fcf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be503c    e983050000               jmp              0x140be55c4
    0x140be5041    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be5048    4c8d0531def201           lea              r8, [rip + 0x1f2de31] -> 0x142b12e80
    0x140be504f    b901000000               mov              ecx, 1
    0x140be5054    ba0e000000               mov              edx, 0xe
    0x140be5059    e8853fcf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be505e    e961050000               jmp              0x140be55c4
    0x140be5063    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be506a    4c8d050fdef201           lea              r8, [rip + 0x1f2de0f] -> 0x142b12e80
    0x140be5071    b901000000               mov              ecx, 1
    0x140be5076    ba11000000               mov              edx, 0x11
    0x140be507b    e8633fcf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be5080    e93f050000               jmp              0x140be55c4
    0x140be5085    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be508c    4c8d05edddf201           lea              r8, [rip + 0x1f2dded] -> 0x142b12e80
    0x140be5093    b901000000               mov              ecx, 1
    0x140be5098    ba17000000               mov              edx, 0x17
    0x140be509d    e8413fcf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be50a2    e91d050000               jmp              0x140be55c4
    0x140be50a7    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be50ae    4c8d05cbddf201           lea              r8, [rip + 0x1f2ddcb] -> 0x142b12e80
    0x140be50b5    b901000000               mov              ecx, 1
    0x140be50ba    ba12000000               mov              edx, 0x12
    0x140be50bf    e81f3fcf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be50c4    e9fb040000               jmp              0x140be55c4
    0x140be50c9    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be50d0    4c8d05a9ddf201           lea              r8, [rip + 0x1f2dda9] -> 0x142b12e80
    0x140be50d7    b901000000               mov              ecx, 1
    0x140be50dc    ba15000000               mov              edx, 0x15
    0x140be50e1    e8fd3ecf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be50e6    e9d9040000               jmp              0x140be55c4
    0x140be50eb    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be50f2    4c8d0587ddf201           lea              r8, [rip + 0x1f2dd87] -> 0x142b12e80
    0x140be50f9    b901000000               mov              ecx, 1
    0x140be50fe    ba1d000000               mov              edx, 0x1d
    0x140be5103    e8db3ecf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be5108    e9b7040000               jmp              0x140be55c4
    0x140be510d    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be5114    4c8d0565ddf201           lea              r8, [rip + 0x1f2dd65] -> 0x142b12e80
    0x140be511b    b901000000               mov              ecx, 1
    0x140be5120    ba13000000               mov              edx, 0x13
    0x140be5125    e8b93ecf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be512a    e995040000               jmp              0x140be55c4
    0x140be512f    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be5136    4c8d0543ddf201           lea              r8, [rip + 0x1f2dd43] -> 0x142b12e80
    0x140be513d    b901000000               mov              ecx, 1
    0x140be5142    ba10000000               mov              edx, 0x10
    0x140be5147    e8973ecf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be514c    e973040000               jmp              0x140be55c4
    0x140be5151    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be5158    4c8d0521ddf201           lea              r8, [rip + 0x1f2dd21] -> 0x142b12e80
    0x140be515f    b901000000               mov              ecx, 1
    0x140be5164    ba23000000               mov              edx, 0x23
    0x140be5169    e8753ecf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be516e    e951040000               jmp              0x140be55c4
    0x140be5173    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be517a    4c8d05ffdcf201           lea              r8, [rip + 0x1f2dcff] -> 0x142b12e80
    0x140be5181    b901000000               mov              ecx, 1
    0x140be5186    ba16000000               mov              edx, 0x16
    0x140be518b    e8533ecf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be5190    e92f040000               jmp              0x140be55c4
    0x140be5195    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be519c    4c8d05dddcf201           lea              r8, [rip + 0x1f2dcdd] -> 0x142b12e80
    0x140be51a3    b901000000               mov              ecx, 1
    0x140be51a8    ba0f000000               mov              edx, 0xf
    0x140be51ad    e8313ecf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be51b2    e90d040000               jmp              0x140be55c4
    0x140be51b7    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be51be    4c8d05bbdcf201           lea              r8, [rip + 0x1f2dcbb] -> 0x142b12e80
    0x140be51c5    b901000000               mov              ecx, 1
    0x140be51ca    ba0f000000               mov              edx, 0xf
    0x140be51cf    e80f3ecf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be51d4    e9eb030000               jmp              0x140be55c4
    0x140be51d9    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be51e0    4c8d0599dcf201           lea              r8, [rip + 0x1f2dc99] -> 0x142b12e80
    0x140be51e7    b901000000               mov              ecx, 1
    0x140be51ec    ba1a000000               mov              edx, 0x1a
    0x140be51f1    e8ed3dcf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be51f6    e9c9030000               jmp              0x140be55c4
    0x140be51fb    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be5202    4c8d0577dcf201           lea              r8, [rip + 0x1f2dc77] -> 0x142b12e80
    0x140be5209    b901000000               mov              ecx, 1
    0x140be520e    ba26000000               mov              edx, 0x26
    0x140be5213    e8cb3dcf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be5218    e9a7030000               jmp              0x140be55c4
    0x140be521d    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be5224    4c8d0555dcf201           lea              r8, [rip + 0x1f2dc55] -> 0x142b12e80
    0x140be522b    b901000000               mov              ecx, 1
    0x140be5230    ba12000000               mov              edx, 0x12
    0x140be5235    e8a93dcf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be523a    e985030000               jmp              0x140be55c4
    0x140be523f    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be5246    4c8d0533dcf201           lea              r8, [rip + 0x1f2dc33] -> 0x142b12e80
    0x140be524d    b901000000               mov              ecx, 1
    0x140be5252    ba09000000               mov              edx, 9
    0x140be5257    e8873dcf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be525c    e963030000               jmp              0x140be55c4
    0x140be5261    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be5268    4c8d0511dcf201           lea              r8, [rip + 0x1f2dc11] -> 0x142b12e80
    0x140be526f    b901000000               mov              ecx, 1
    0x140be5274    ba15000000               mov              edx, 0x15
    0x140be5279    e8653dcf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be527e    e941030000               jmp              0x140be55c4
    0x140be5283    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be528a    4c8d05efdbf201           lea              r8, [rip + 0x1f2dbef] -> 0x142b12e80
    0x140be5291    b901000000               mov              ecx, 1
    0x140be5296    ba18000000               mov              edx, 0x18
    0x140be529b    e8433dcf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be52a0    e91f030000               jmp              0x140be55c4
    0x140be52a5    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be52ac    4c8d05cddbf201           lea              r8, [rip + 0x1f2dbcd] -> 0x142b12e80
    0x140be52b3    b901000000               mov              ecx, 1
    0x140be52b8    ba20000000               mov              edx, 0x20
    0x140be52bd    e8213dcf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be52c2    e9fd020000               jmp              0x140be55c4
    0x140be52c7    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be52ce    4c8d05abdbf201           lea              r8, [rip + 0x1f2dbab] -> 0x142b12e80
    0x140be52d5    b901000000               mov              ecx, 1
    0x140be52da    ba0d000000               mov              edx, 0xd
    0x140be52df    e8ff3ccf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be52e4    e9db020000               jmp              0x140be55c4
    0x140be52e9    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be52f0    4c8d0589dbf201           lea              r8, [rip + 0x1f2db89] -> 0x142b12e80
    0x140be52f7    b901000000               mov              ecx, 1
    0x140be52fc    ba0f000000               mov              edx, 0xf
    0x140be5301    e8dd3ccf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be5306    e9b9020000               jmp              0x140be55c4
    0x140be530b    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be5312    4c8d0567dbf201           lea              r8, [rip + 0x1f2db67] -> 0x142b12e80
    0x140be5319    b901000000               mov              ecx, 1
    0x140be531e    ba0d000000               mov              edx, 0xd
    0x140be5323    e8bb3ccf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be5328    e997020000               jmp              0x140be55c4
    0x140be532d    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be5334    4c8d0545dbf201           lea              r8, [rip + 0x1f2db45] -> 0x142b12e80
    0x140be533b    b901000000               mov              ecx, 1
    0x140be5340    ba15000000               mov              edx, 0x15
    0x140be5345    e8993ccf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be534a    e975020000               jmp              0x140be55c4
    0x140be534f    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be5356    4c8d0523dbf201           lea              r8, [rip + 0x1f2db23] -> 0x142b12e80
    0x140be535d    b901000000               mov              ecx, 1
    0x140be5362    ba14000000               mov              edx, 0x14
    0x140be5367    e8773ccf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be536c    e953020000               jmp              0x140be55c4
    0x140be5371    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be5378    4c8d0501dbf201           lea              r8, [rip + 0x1f2db01] -> 0x142b12e80
    0x140be537f    b901000000               mov              ecx, 1
    0x140be5384    ba12000000               mov              edx, 0x12
    0x140be5389    e8553ccf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be538e    e931020000               jmp              0x140be55c4
    0x140be5393    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be539a    4c8d05dfdaf201           lea              r8, [rip + 0x1f2dadf] -> 0x142b12e80
    0x140be53a1    b901000000               mov              ecx, 1
    0x140be53a6    ba28000000               mov              edx, 0x28
    0x140be53ab    e8333ccf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be53b0    e90f020000               jmp              0x140be55c4
    0x140be53b5    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be53bc    4c8d05bddaf201           lea              r8, [rip + 0x1f2dabd] -> 0x142b12e80
    0x140be53c3    b901000000               mov              ecx, 1
    0x140be53c8    ba0d000000               mov              edx, 0xd
    0x140be53cd    e8113ccf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be53d2    e9ed010000               jmp              0x140be55c4
    0x140be53d7    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be53de    4c8d059bdaf201           lea              r8, [rip + 0x1f2da9b] -> 0x142b12e80
    0x140be53e5    b901000000               mov              ecx, 1
    0x140be53ea    ba0c000000               mov              edx, 0xc
    0x140be53ef    e8ef3bcf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be53f4    e9cb010000               jmp              0x140be55c4
    0x140be53f9    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be5400    4c8d0579daf201           lea              r8, [rip + 0x1f2da79] -> 0x142b12e80
    0x140be5407    b901000000               mov              ecx, 1
    0x140be540c    ba12000000               mov              edx, 0x12
    0x140be5411    e8cd3bcf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be5416    e9a9010000               jmp              0x140be55c4
    0x140be541b    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be5422    4c8d0557daf201           lea              r8, [rip + 0x1f2da57] -> 0x142b12e80
    0x140be5429    b901000000               mov              ecx, 1
    0x140be542e    ba17000000               mov              edx, 0x17
    0x140be5433    e8ab3bcf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be5438    e987010000               jmp              0x140be55c4
    0x140be543d    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be5444    4c8d0535daf201           lea              r8, [rip + 0x1f2da35] -> 0x142b12e80
    0x140be544b    b901000000               mov              ecx, 1
    0x140be5450    ba16000000               mov              edx, 0x16
    0x140be5455    e8893bcf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be545a    e965010000               jmp              0x140be55c4
    0x140be545f    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be5466    4c8d0513daf201           lea              r8, [rip + 0x1f2da13] -> 0x142b12e80
    0x140be546d    b901000000               mov              ecx, 1
    0x140be5472    ba0b000000               mov              edx, 0xb
    0x140be5477    e8673bcf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be547c    e943010000               jmp              0x140be55c4
    0x140be5481    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be5488    4c8d05f1d9f201           lea              r8, [rip + 0x1f2d9f1] -> 0x142b12e80
    0x140be548f    b901000000               mov              ecx, 1
    0x140be5494    ba17000000               mov              edx, 0x17
    0x140be5499    e8453bcf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be549e    e921010000               jmp              0x140be55c4
    0x140be54a3    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be54aa    4c8d05cfd9f201           lea              r8, [rip + 0x1f2d9cf] -> 0x142b12e80
    0x140be54b1    b901000000               mov              ecx, 1
    0x140be54b6    ba14000000               mov              edx, 0x14
    0x140be54bb    e8233bcf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be54c0    e9ff000000               jmp              0x140be55c4
    0x140be54c5    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be54cc    4c8d05add9f201           lea              r8, [rip + 0x1f2d9ad] -> 0x142b12e80
    0x140be54d3    b901000000               mov              ecx, 1
    0x140be54d8    ba13000000               mov              edx, 0x13
    0x140be54dd    e8013bcf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be54e2    e9dd000000               jmp              0x140be55c4
    0x140be54e7    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be54ee    4c8d058bd9f201           lea              r8, [rip + 0x1f2d98b] -> 0x142b12e80
    0x140be54f5    b901000000               mov              ecx, 1
    0x140be54fa    ba1c000000               mov              edx, 0x1c
    0x140be54ff    e8df3acf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be5504    e9bb000000               jmp              0x140be55c4
    0x140be5509    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be5510    4c8d0569d9f201           lea              r8, [rip + 0x1f2d969] -> 0x142b12e80
    0x140be5517    b901000000               mov              ecx, 1
    0x140be551c    ba1a000000               mov              edx, 0x1a
    0x140be5521    e8bd3acf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be5526    e999000000               jmp              0x140be55c4
    0x140be552b    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be5532    4c8d0547d9f201           lea              r8, [rip + 0x1f2d947] -> 0x142b12e80
    0x140be5539    b901000000               mov              ecx, 1
    0x140be553e    ba13000000               mov              edx, 0x13
    0x140be5543    e89b3acf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be5548    eb7a                     jmp              0x140be55c4
    0x140be554a    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be5551    4c8d0528d9f201           lea              r8, [rip + 0x1f2d928] -> 0x142b12e80
    0x140be5558    b901000000               mov              ecx, 1
    0x140be555d    ba0f000000               mov              edx, 0xf
    0x140be5562    e87c3acf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be5567    eb5b                     jmp              0x140be55c4
    0x140be5569    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be5570    4c8d0509d9f201           lea              r8, [rip + 0x1f2d909] -> 0x142b12e80
    0x140be5577    b901000000               mov              ecx, 1
    0x140be557c    ba15000000               mov              edx, 0x15
    0x140be5581    e85d3acf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be5586    eb3c                     jmp              0x140be55c4
    0x140be5588    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be558f    4c8d05ead8f201           lea              r8, [rip + 0x1f2d8ea] -> 0x142b12e80
    0x140be5596    b901000000               mov              ecx, 1
    0x140be559b    ba20000000               mov              edx, 0x20
    0x140be55a0    e83e3acf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be55a5    eb1d                     jmp              0x140be55c4
    0x140be55a7    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be55ae    4c8d05cbd8f201           lea              r8, [rip + 0x1f2d8cb] -> 0x142b12e80
    0x140be55b5    b901000000               mov              ecx, 1
    0x140be55ba    ba10000000               mov              edx, 0x10
    0x140be55bf    e81f3acf01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140be55c4    0f0b                     ud2              
    0x140be55c6    662e0f1f840000000000     nop              word ptr cs:[rax + rax]
    0x140be55d0    4889542410               mov              qword ptr [rsp + 0x10], rdx
    0x140be55d5    55                       push             rbp
    0x140be55d6    4157                     push             r15
    0x140be55d8    4156                     push             r14
    0x140be55da    4155                     push             r13
    0x140be55dc    4154                     push             r12
    0x140be55de    56                       push             rsi
    0x140be55df    57                       push             rdi
    0x140be55e0    53                       push             rbx
    0x140be55e1    4883ec28                 sub              rsp, 0x28
    0x140be55e5    488daa80000000           lea              rbp, [rdx + 0x80]
    0x140be55ec    488b85e8010000           mov              rax, qword ptr [rbp + 0x1e8]
    0x140be55f3    488b10                   mov              rdx, qword ptr [rax]
    0x140be55f6    4885d2                   test             rdx, rdx
    0x140be55f9    7416                     je               0x140be5611
    0x140be55fb    488b85e8010000           mov              rax, qword ptr [rbp + 0x1e8]
    0x140be5602    488b4808                 mov              rcx, qword ptr [rax + 8]
    0x140be5606    41b801000000             mov              r8d, 1
    0x140be560c    e80fe6a2ff               call             0x140613c20 -> __rust_dealloc
    0x140be5611    90                       nop              
    0x140be5612    4883c428                 add              rsp, 0x28
    0x140be5616    5b                       pop              rbx
    0x140be5617    5f                       pop              rdi
    0x140be5618    5e                       pop              rsi
    0x140be5619    415c                     pop              r12
    0x140be561b    415d                     pop              r13
    0x140be561d    415e                     pop              r14
    0x140be561f    415f                     pop              r15
    0x140be5621    5d                       pop              rbp
    0x140be5622    c3                       ret              
    0x140be5623    666666662e0f1f840000000000 nop              word ptr cs:[rax + rax]
    0x140be5630    4889542410               mov              qword ptr [rsp + 0x10], rdx
    0x140be5635    55                       push             rbp
    0x140be5636    4157                     push             r15
    0x140be5638    4156                     push             r14
    0x140be563a    4155                     push             r13
    0x140be563c    4154                     push             r12
    0x140be563e    56                       push             rsi
    0x140be563f    57                       push             rdi
    0x140be5640    53                       push             rbx
    0x140be5641    4883ec28                 sub              rsp, 0x28
    0x140be5645    488daa80000000           lea              rbp, [rdx + 0x80]
    0x140be564c    4883bd8002000000         cmp              qword ptr [rbp + 0x280], 0
    0x140be5654    7419                     je               0x140be566f
    0x140be5656    41b801000000             mov              r8d, 1
    0x140be565c    488b8d78020000           mov              rcx, qword ptr [rbp + 0x278]
    0x140be5663    488b9580020000           mov              rdx, qword ptr [rbp + 0x280]
    0x140be566a    e8b1e5a2ff               call             0x140613c20 -> __rust_dealloc
    0x140be566f    c685af02000000           mov              byte ptr [rbp + 0x2af], 0
    0x140be5676    4883c428                 add              rsp, 0x28
    0x140be567a    5b                       pop              rbx
    0x140be567b    5f                       pop              rdi
    0x140be567c    5e                       pop              rsi
    0x140be567d    415c                     pop              r12
    0x140be567f    415d                     pop              r13
    0x140be5681    415e                     pop              r14
    0x140be5683    415f                     pop              r15
    0x140be5685    5d                       pop              rbp
    0x140be5686    c3                       ret              
    0x140be5687    660f1f840000000000       nop              word ptr [rax + rax]
    0x140be5690    4889542410               mov              qword ptr [rsp + 0x10], rdx
    0x140be5695    55                       push             rbp
    0x140be5696    4157                     push             r15
    0x140be5698    4156                     push             r14
    0x140be569a    4155                     push             r13
    0x140be569c    4154                     push             r12
    0x140be569e    56                       push             rsi
    0x140be569f    57                       push             rdi
    0x140be56a0    53                       push             rbx
    0x140be56a1    4883ec28                 sub              rsp, 0x28
    0x140be56a5    488daa80000000           lea              rbp, [rdx + 0x80]
    0x140be56ac    80bdaf02000000           cmp              byte ptr [rbp + 0x2af], 0
    0x140be56b3    7436                     je               0x140be56eb
    0x140be56b5    488b9590020000           mov              rdx, qword ptr [rbp + 0x290]
    0x140be56bc    4885d2                   test             rdx, rdx
    0x140be56bf    7412                     je               0x140be56d3
    0x140be56c1    488b8d98020000           mov              rcx, qword ptr [rbp + 0x298]
    0x140be56c8    41b801000000             mov              r8d, 1
    0x140be56ce    e84de5a2ff               call             0x140613c20 -> __rust_dealloc
    0x140be56d3    c6858f02000001           mov              byte ptr [rbp + 0x28f], 1
    0x140be56da    4883c428                 add              rsp, 0x28
    0x140be56de    5b                       pop              rbx
    0x140be56df    5f                       pop              rdi
    0x140be56e0    5e                       pop              rsi
    0x140be56e1    415c                     pop              r12
    0x140be56e3    415d                     pop              r13
    0x140be56e5    415e                     pop              r14
    0x140be56e7    415f                     pop              r15
    0x140be56e9    5d                       pop              rbp
    0x140be56ea    c3                       ret              
    0x140be56eb    c6858f02000001           mov              byte ptr [rbp + 0x28f], 1
    0x140be56f2    4883c428                 add              rsp, 0x28
    0x140be56f6    5b                       pop              rbx
    0x140be56f7    5f                       pop              rdi
    0x140be56f8    5e                       pop              rsi
    0x140be56f9    415c                     pop              r12
    0x140be56fb    415d                     pop              r13
    0x140be56fd    415e                     pop              r14
    0x140be56ff    415f                     pop              r15
    0x140be5701    5d                       pop              rbp
    0x140be5702    c3                       ret              
    0x140be5703    666666662e0f1f840000000000 nop              word ptr cs:[rax + rax]
    0x140be5710    4889542410               mov              qword ptr [rsp + 0x10], rdx
    0x140be5715    55                       push             rbp
    0x140be5716    4157                     push             r15
    0x140be5718    4156                     push             r14
    0x140be571a    4155                     push             r13
    0x140be571c    4154                     push             r12
    0x140be571e    56                       push             rsi
    0x140be571f    57                       push             rdi
    0x140be5720    53                       push             rbx
    0x140be5721    4883ec28                 sub              rsp, 0x28
    0x140be5725    488daa80000000           lea              rbp, [rdx + 0x80]
    0x140be572c    4883bd8002000000         cmp              qword ptr [rbp + 0x280], 0
    0x140be5734    7419                     je               0x140be574f
    0x140be5736    41b801000000             mov              r8d, 1
    0x140be573c    488b8d78020000           mov              rcx, qword ptr [rbp + 0x278]
    0x140be5743    488b9580020000           mov              rdx, qword ptr [rbp + 0x280]
    0x140be574a    e8d1e4a2ff               call             0x140613c20 -> __rust_dealloc
    0x140be574f    c6858f02000001           mov              byte ptr [rbp + 0x28f], 1
    0x140be5756    4883c428                 add              rsp, 0x28
    0x140be575a    5b                       pop              rbx
    0x140be575b    5f                       pop              rdi
    0x140be575c    5e                       pop              rsi
    0x140be575d    415c                     pop              r12
    0x140be575f    415d                     pop              r13
    0x140be5761    415e                     pop              r14
    0x140be5763    415f                     pop              r15
    0x140be5765    5d                       pop              rbp
    0x140be5766    c3                       ret              
    0x140be5767    660f1f840000000000       nop              word ptr [rax + rax]
    0x140be5770    4889542410               mov              qword ptr [rsp + 0x10], rdx
    0x140be5775    55                       push             rbp
    0x140be5776    4157                     push             r15
    0x140be5778    4156                     push             r14
    0x140be577a    4155                     push             r13
    0x140be577c    4154                     push             r12
    0x140be577e    56                       push             rsi
    0x140be577f    57                       push             rdi
    0x140be5780    53                       push             rbx
    0x140be5781    4883ec28                 sub              rsp, 0x28
    0x140be5785    488daa80000000           lea              rbp, [rdx + 0x80]
    0x140be578c    80bd8f02000000           cmp              byte ptr [rbp + 0x28f], 0
    0x140be5793    741e                     je               0x140be57b3
    0x140be5795    488d8df0010000           lea              rcx, [rbp + 0x1f0]
    0x140be579c    e8cf37a1ff               call             0x1405f8f70 -> core::ptr::drop_in_place$LT$alloc..vec..Vec$LT$relative_path..RelativePathBuf$GT$$GT$::hd3bf5695b2ab4189
    0x140be57a1    90                       nop              
    0x140be57a2    4883c428                 add              rsp, 0x28
    0x140be57a6    5b                       pop              rbx
    0x140be57a7    5f                       pop              rdi
    0x140be57a8    5e                       pop              rsi
    0x140be57a9    415c                     pop              r12
    0x140be57ab    415d                     pop              r13
    0x140be57ad    415e                     pop              r14
    0x140be57af    415f                     pop              r15
    0x140be57b1    5d                       pop              rbp
    0x140be57b2    c3                       ret              
    0x140be57b3    4883c428                 add              rsp, 0x28
    0x140be57b7    5b                       pop              rbx
    0x140be57b8    5f                       pop              rdi
    0x140be57b9    5e                       pop              rsi
    0x140be57ba    415c                     pop              r12
    0x140be57bc    415d                     pop              r13
    0x140be57be    415e                     pop              r14
    0x140be57c0    415f                     pop              r15
    0x140be57c2    5d                       pop              rbp
    0x140be57c3    c3                       ret              

=== 0x140be9270 size=421  rhapsody::raw::error::impl$1::fmt
    ; Leaf/impl fully decoded from PE bytes (semantics mostly diagnostic/formatting); see insns.  [medium]
    0x140be9270    4881ec88000000           sub              rsp, 0x88
    0x140be9277    4889d0                   mov              rax, rdx
    0x140be927a    4c8b01                   mov              r8, qword ptr [rcx]
    0x140be927d    4d8d48f7                 lea              r9, [r8 - 9]
    0x140be9281    4983c0f8                 add              r8, -8
    0x140be9285    31d2                     xor              edx, edx
    0x140be9287    4983f902                 cmp              r9, 2
    0x140be928b    490f42d0                 cmovb            rdx, r8
    0x140be928f    4885d2                   test             rdx, rdx
    0x140be9292    0f8487000000             je               0x140be931f
    0x140be9298    4883fa01                 cmp              rdx, 1
    0x140be929c    0f8520010000             jne              0x140be93c2
    0x140be92a2    488d5120                 lea              rdx, [rcx + 0x20]
    0x140be92a6    4883c108                 add              rcx, 8
    0x140be92aa    48898c2480000000         mov              qword ptr [rsp + 0x80], rcx
    0x140be92b2    488d0ddfaef201           lea              rcx, [rip + 0x1f2aedf] -> 0x142b14198
    0x140be92b9    48894c2450               mov              qword ptr [rsp + 0x50], rcx
    0x140be92be    488d8c2480000000         lea              rcx, [rsp + 0x80]
    0x140be92c6    48894c2448               mov              qword ptr [rsp + 0x48], rcx
    0x140be92cb    488d0defaef201           lea              rcx, [rip + 0x1f2aeef] -> 0x142b141c1
    0x140be92d2    48894c2438               mov              qword ptr [rsp + 0x38], rcx
    0x140be92d7    488d0d9aaef201           lea              rcx, [rip + 0x1f2ae9a] -> 0x142b14178
    0x140be92de    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x140be92e3    4889542428               mov              qword ptr [rsp + 0x28], rdx
    0x140be92e8    48c744244005000000       mov              qword ptr [rsp + 0x40], 5
    0x140be92f1    48c744242003000000       mov              qword ptr [rsp + 0x20], 3
    0x140be92fa    488d15b7aef201           lea              rdx, [rip + 0x1f2aeb7] -> 0x142b141b8
    0x140be9301    4c8d0db6aef201           lea              r9, [rip + 0x1f2aeb6] -> 0x142b141be
    0x140be9308    41b806000000             mov              r8d, 6
    0x140be930e    4889c1                   mov              rcx, rax
    0x140be9311    e88ad91900               call             0x140d86ca0 -> core::fmt::Formatter::debug_struct_field2_finish::h35c7e6c960cc525a
    0x140be9316    90                       nop              
    0x140be9317    4881c488000000           add              rsp, 0x88
    0x140be931e    c3                       ret              
    0x140be931f    488d5120                 lea              rdx, [rcx + 0x20]
    0x140be9323    4c8d4138                 lea              r8, [rcx + 0x38]
    0x140be9327    4c89842480000000         mov              qword ptr [rsp + 0x80], r8
    0x140be932f    4c8d0502aef201           lea              r8, [rip + 0x1f2ae02] -> 0x142b14138
    0x140be9336    4c89442470               mov              qword ptr [rsp + 0x70], r8
    0x140be933b    4c8d842480000000         lea              r8, [rsp + 0x80]
    0x140be9343    4c89442468               mov              qword ptr [rsp + 0x68], r8
    0x140be9348    4c8d051caef201           lea              r8, [rip + 0x1f2ae1c] -> 0x142b1416b
    0x140be934f    4c89442458               mov              qword ptr [rsp + 0x58], r8
    0x140be9354    4c8d05bdadf201           lea              r8, [rip + 0x1f2adbd] -> 0x142b14118
    0x140be935b    4c89442450               mov              qword ptr [rsp + 0x50], r8
    0x140be9360    4889542448               mov              qword ptr [rsp + 0x48], rdx
    0x140be9365    488d15fbadf201           lea              rdx, [rip + 0x1f2adfb] -> 0x142b14167
    0x140be936c    4889542438               mov              qword ptr [rsp + 0x38], rdx
    0x140be9371    488d1580adf201           lea              rdx, [rip + 0x1f2ad80] -> 0x142b140f8
    0x140be9378    4889542430               mov              qword ptr [rsp + 0x30], rdx
    0x140be937d    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x140be9382    48c74424600c000000       mov              qword ptr [rsp + 0x60], 0xc
    0x140be938b    48c744244004000000       mov              qword ptr [rsp + 0x40], 4
    0x140be9394    48c744242005000000       mov              qword ptr [rsp + 0x20], 5
    0x140be939d    488d15b4adf201           lea              rdx, [rip + 0x1f2adb4] -> 0x142b14158
    0x140be93a4    4c8d0db7adf201           lea              r9, [rip + 0x1f2adb7] -> 0x142b14162
    0x140be93ab    41b80a000000             mov              r8d, 0xa
    0x140be93b1    4889c1                   mov              rcx, rax
    0x140be93b4    e8d7d91900               call             0x140d86d90 -> core::fmt::Formatter::debug_struct_field3_finish::h024add6f4e520141
    0x140be93b9    90                       nop              
    0x140be93ba    4881c488000000           add              rsp, 0x88
    0x140be93c1    c3                       ret              
    0x140be93c2    4883c108                 add              rcx, 8
    0x140be93c6    48898c2480000000         mov              qword ptr [rsp + 0x80], rcx
    0x140be93ce    488d0d3ba7f201           lea              rcx, [rip + 0x1f2a73b] -> 0x142b13b10
    0x140be93d5    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x140be93da    488d8c2480000000         lea              rcx, [rsp + 0x80]
    0x140be93e2    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x140be93e7    48c744242004000000       mov              qword ptr [rsp + 0x20], 4
    0x140be93f0    488d15cfadf201           lea              rdx, [rip + 0x1f2adcf] -> 0x142b141c6
    0x140be93f7    4c8d0dd6adf201           lea              r9, [rip + 0x1f2add6] -> 0x142b141d4
    0x140be93fe    41b80e000000             mov              r8d, 0xe
    0x140be9404    4889c1                   mov              rcx, rax
    0x140be9407    e8e4d71900               call             0x140d86bf0 -> core::fmt::Formatter::debug_struct_field1_finish::hd9497ec2d71b9977
    0x140be940c    90                       nop              
    0x140be940d    4881c488000000           add              rsp, 0x88
    0x140be9414    c3                       ret              

=== 0x140becc90 size=421  rhapsody::raw::error::impl$1::fmt
    ; Leaf/impl fully decoded from PE bytes (semantics mostly diagnostic/formatting); see insns.  [medium]
    0x140becc90    4881ec88000000           sub              rsp, 0x88
    0x140becc97    4889d0                   mov              rax, rdx
    0x140becc9a    4c8b01                   mov              r8, qword ptr [rcx]
    0x140becc9d    4d8d48f7                 lea              r9, [r8 - 9]
    0x140becca1    4983c0f8                 add              r8, -8
    0x140becca5    31d2                     xor              edx, edx
    0x140becca7    4983f902                 cmp              r9, 2
    0x140beccab    490f42d0                 cmovb            rdx, r8
    0x140beccaf    4885d2                   test             rdx, rdx
    0x140beccb2    0f8487000000             je               0x140becd3f
    0x140beccb8    4883fa01                 cmp              rdx, 1
    0x140beccbc    0f8520010000             jne              0x140becde2
    0x140beccc2    488d5120                 lea              rdx, [rcx + 0x20]
    0x140beccc6    4883c108                 add              rcx, 8
    0x140beccca    48898c2480000000         mov              qword ptr [rsp + 0x80], rcx
    0x140beccd2    488d0ddf7cf201           lea              rcx, [rip + 0x1f27cdf] -> 0x142b149b8
    0x140beccd9    48894c2450               mov              qword ptr [rsp + 0x50], rcx
    0x140beccde    488d8c2480000000         lea              rcx, [rsp + 0x80]
    0x140becce6    48894c2448               mov              qword ptr [rsp + 0x48], rcx
    0x140becceb    488d0def7cf201           lea              rcx, [rip + 0x1f27cef] -> 0x142b149e1
    0x140beccf2    48894c2438               mov              qword ptr [rsp + 0x38], rcx
    0x140beccf7    488d0d4276f201           lea              rcx, [rip + 0x1f27642] -> 0x142b14340
    0x140beccfe    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x140becd03    4889542428               mov              qword ptr [rsp + 0x28], rdx
    0x140becd08    48c744244005000000       mov              qword ptr [rsp + 0x40], 5
    0x140becd11    48c744242003000000       mov              qword ptr [rsp + 0x20], 3
    0x140becd1a    488d15b77cf201           lea              rdx, [rip + 0x1f27cb7] -> 0x142b149d8
    0x140becd21    4c8d0db67cf201           lea              r9, [rip + 0x1f27cb6] -> 0x142b149de
    0x140becd28    41b806000000             mov              r8d, 6
    0x140becd2e    4889c1                   mov              rcx, rax
    0x140becd31    e86a9f1900               call             0x140d86ca0 -> core::fmt::Formatter::debug_struct_field2_finish::h35c7e6c960cc525a
    0x140becd36    90                       nop              
    0x140becd37    4881c488000000           add              rsp, 0x88
    0x140becd3e    c3                       ret              
    0x140becd3f    488d5120                 lea              rdx, [rcx + 0x20]
    0x140becd43    4c8d4138                 lea              r8, [rcx + 0x38]
    0x140becd47    4c89842480000000         mov              qword ptr [rsp + 0x80], r8
    0x140becd4f    4c8d05227cf201           lea              r8, [rip + 0x1f27c22] -> 0x142b14978
    0x140becd56    4c89442470               mov              qword ptr [rsp + 0x70], r8
    0x140becd5b    4c8d842480000000         lea              r8, [rsp + 0x80]
    0x140becd63    4c89442468               mov              qword ptr [rsp + 0x68], r8
    0x140becd68    4c8d053c7cf201           lea              r8, [rip + 0x1f27c3c] -> 0x142b149ab
    0x140becd6f    4c89442458               mov              qword ptr [rsp + 0x58], r8
    0x140becd74    4c8d05dd7bf201           lea              r8, [rip + 0x1f27bdd] -> 0x142b14958
    0x140becd7b    4c89442450               mov              qword ptr [rsp + 0x50], r8
    0x140becd80    4889542448               mov              qword ptr [rsp + 0x48], rdx
    0x140becd85    488d151b7cf201           lea              rdx, [rip + 0x1f27c1b] -> 0x142b149a7
    0x140becd8c    4889542438               mov              qword ptr [rsp + 0x38], rdx
    0x140becd91    488d15a07bf201           lea              rdx, [rip + 0x1f27ba0] -> 0x142b14938
    0x140becd98    4889542430               mov              qword ptr [rsp + 0x30], rdx
    0x140becd9d    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x140becda2    48c74424600c000000       mov              qword ptr [rsp + 0x60], 0xc
    0x140becdab    48c744244004000000       mov              qword ptr [rsp + 0x40], 4
    0x140becdb4    48c744242005000000       mov              qword ptr [rsp + 0x20], 5
    0x140becdbd    488d15d47bf201           lea              rdx, [rip + 0x1f27bd4] -> 0x142b14998
    0x140becdc4    4c8d0dd77bf201           lea              r9, [rip + 0x1f27bd7] -> 0x142b149a2
    0x140becdcb    41b80a000000             mov              r8d, 0xa
    0x140becdd1    4889c1                   mov              rcx, rax
    0x140becdd4    e8b79f1900               call             0x140d86d90 -> core::fmt::Formatter::debug_struct_field3_finish::h024add6f4e520141
    0x140becdd9    90                       nop              
    0x140becdda    4881c488000000           add              rsp, 0x88
    0x140becde1    c3                       ret              
    0x140becde2    4883c108                 add              rcx, 8
    0x140becde6    48898c2480000000         mov              qword ptr [rsp + 0x80], rcx
    0x140becdee    488d0dc376f201           lea              rcx, [rip + 0x1f276c3] -> 0x142b144b8
    0x140becdf5    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x140becdfa    488d8c2480000000         lea              rcx, [rsp + 0x80]
    0x140bece02    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x140bece07    48c744242004000000       mov              qword ptr [rsp + 0x20], 4
    0x140bece10    488d15cf7bf201           lea              rdx, [rip + 0x1f27bcf] -> 0x142b149e6
    0x140bece17    4c8d0dd67bf201           lea              r9, [rip + 0x1f27bd6] -> 0x142b149f4
    0x140bece1e    41b80e000000             mov              r8d, 0xe
    0x140bece24    4889c1                   mov              rcx, rax
    0x140bece27    e8c49d1900               call             0x140d86bf0 -> core::fmt::Formatter::debug_struct_field1_finish::hd9497ec2d71b9977
    0x140bece2c    90                       nop              
    0x140bece2d    4881c488000000           add              rsp, 0x88
    0x140bece34    c3                       ret              

=== 0x140bed790 size=90  rhapsody::raw::submit::impl$9::fmt
    ; Leaf/impl fully decoded from PE bytes (semantics mostly diagnostic/formatting); see insns.  [medium]
    0x140bed790    4883ec38                 sub              rsp, 0x38
    0x140bed794    4889d0                   mov              rax, rdx
    0x140bed797    f60101                   test             byte ptr [rcx], 1
    0x140bed79a    7435                     je               0x140bed7d1
    0x140bed79c    4883c104                 add              rcx, 4
    0x140bed7a0    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x140bed7a5    488d0d9c76f201           lea              rcx, [rip + 0x1f2769c] -> 0x142b14e48
    0x140bed7ac    48894c2420               mov              qword ptr [rsp + 0x20], rcx
    0x140bed7b1    488d15b076f201           lea              rdx, [rip + 0x1f276b0] -> 0x142b14e68
    0x140bed7b8    4c8d4c2430               lea              r9, [rsp + 0x30]
    0x140bed7bd    41b807000000             mov              r8d, 7
    0x140bed7c3    4889c1                   mov              rcx, rax
    0x140bed7c6    e8059b1900               call             0x140d872d0 -> core::fmt::Formatter::debug_tuple_field1_finish::ha115499c8c7eba6f
    0x140bed7cb    90                       nop              
    0x140bed7cc    4883c438                 add              rsp, 0x38
    0x140bed7d0    c3                       ret              
    0x140bed7d1    488d156276f201           lea              rdx, [rip + 0x1f27662] -> 0x142b14e3a
    0x140bed7d8    41b809000000             mov              r8d, 9
    0x140bed7de    4889c1                   mov              rcx, rax
    0x140bed7e1    4883c438                 add              rsp, 0x38
    0x140bed7e5    e9a6931900               jmp              0x140d86b90 -> _$LT$core..fmt..Formatter$u20$as$u20$core..fmt..Write$GT$::write_str::h5c3f9ed06cfb23bd

=== 0x140bff930 size=421  rhapsody::raw::error::impl$1::fmt
    ; Leaf/impl fully decoded from PE bytes (semantics mostly diagnostic/formatting); see insns.  [medium]
    0x140bff930    4881ec88000000           sub              rsp, 0x88
    0x140bff937    4889d0                   mov              rax, rdx
    0x140bff93a    4c8b01                   mov              r8, qword ptr [rcx]
    0x140bff93d    4d8d48f7                 lea              r9, [r8 - 9]
    0x140bff941    4983c0f8                 add              r8, -8
    0x140bff945    31d2                     xor              edx, edx
    0x140bff947    4983f902                 cmp              r9, 2
    0x140bff94b    490f42d0                 cmovb            rdx, r8
    0x140bff94f    4885d2                   test             rdx, rdx
    0x140bff952    0f8487000000             je               0x140bff9df
    0x140bff958    4883fa01                 cmp              rdx, 1
    0x140bff95c    0f8520010000             jne              0x140bffa82
    0x140bff962    488d5120                 lea              rdx, [rcx + 0x20]
    0x140bff966    4883c108                 add              rcx, 8
    0x140bff96a    48898c2480000000         mov              qword ptr [rsp + 0x80], rcx
    0x140bff972    488d0d177af101           lea              rcx, [rip + 0x1f17a17] -> 0x142b17390
    0x140bff979    48894c2450               mov              qword ptr [rsp + 0x50], rcx
    0x140bff97e    488d8c2480000000         lea              rcx, [rsp + 0x80]
    0x140bff986    48894c2448               mov              qword ptr [rsp + 0x48], rcx
    0x140bff98b    488d0d277af101           lea              rcx, [rip + 0x1f17a27] -> 0x142b173b9
    0x140bff992    48894c2438               mov              qword ptr [rsp + 0x38], rcx
    0x140bff997    488d0d8a57f101           lea              rcx, [rip + 0x1f1578a] -> 0x142b15128
    0x140bff99e    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x140bff9a3    4889542428               mov              qword ptr [rsp + 0x28], rdx
    0x140bff9a8    48c744244005000000       mov              qword ptr [rsp + 0x40], 5
    0x140bff9b1    48c744242003000000       mov              qword ptr [rsp + 0x20], 3
    0x140bff9ba    488d15ef79f101           lea              rdx, [rip + 0x1f179ef] -> 0x142b173b0
    0x140bff9c1    4c8d0dee79f101           lea              r9, [rip + 0x1f179ee] -> 0x142b173b6
    0x140bff9c8    41b806000000             mov              r8d, 6
    0x140bff9ce    4889c1                   mov              rcx, rax
    0x140bff9d1    e8ca721800               call             0x140d86ca0 -> core::fmt::Formatter::debug_struct_field2_finish::h35c7e6c960cc525a
    0x140bff9d6    90                       nop              
    0x140bff9d7    4881c488000000           add              rsp, 0x88
    0x140bff9de    c3                       ret              
    0x140bff9df    488d5120                 lea              rdx, [rcx + 0x20]
    0x140bff9e3    4c8d4138                 lea              r8, [rcx + 0x38]
    0x140bff9e7    4c89842480000000         mov              qword ptr [rsp + 0x80], r8
    0x140bff9ef    4c8d055a79f101           lea              r8, [rip + 0x1f1795a] -> 0x142b17350
    0x140bff9f6    4c89442470               mov              qword ptr [rsp + 0x70], r8
    0x140bff9fb    4c8d842480000000         lea              r8, [rsp + 0x80]
    0x140bffa03    4c89442468               mov              qword ptr [rsp + 0x68], r8
    0x140bffa08    4c8d057479f101           lea              r8, [rip + 0x1f17974] -> 0x142b17383
    0x140bffa0f    4c89442458               mov              qword ptr [rsp + 0x58], r8
    0x140bffa14    4c8d051579f101           lea              r8, [rip + 0x1f17915] -> 0x142b17330
    0x140bffa1b    4c89442450               mov              qword ptr [rsp + 0x50], r8
    0x140bffa20    4889542448               mov              qword ptr [rsp + 0x48], rdx
    0x140bffa25    488d155379f101           lea              rdx, [rip + 0x1f17953] -> 0x142b1737f
    0x140bffa2c    4889542438               mov              qword ptr [rsp + 0x38], rdx
    0x140bffa31    488d151057f101           lea              rdx, [rip + 0x1f15710] -> 0x142b15148
    0x140bffa38    4889542430               mov              qword ptr [rsp + 0x30], rdx
    0x140bffa3d    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x140bffa42    48c74424600c000000       mov              qword ptr [rsp + 0x60], 0xc
    0x140bffa4b    48c744244004000000       mov              qword ptr [rsp + 0x40], 4
    0x140bffa54    48c744242005000000       mov              qword ptr [rsp + 0x20], 5
    0x140bffa5d    488d150c79f101           lea              rdx, [rip + 0x1f1790c] -> 0x142b17370
    0x140bffa64    4c8d0d0f79f101           lea              r9, [rip + 0x1f1790f] -> 0x142b1737a
    0x140bffa6b    41b80a000000             mov              r8d, 0xa
    0x140bffa71    4889c1                   mov              rcx, rax
    0x140bffa74    e817731800               call             0x140d86d90 -> core::fmt::Formatter::debug_struct_field3_finish::h024add6f4e520141
    0x140bffa79    90                       nop              
    0x140bffa7a    4881c488000000           add              rsp, 0x88
    0x140bffa81    c3                       ret              
    0x140bffa82    4883c108                 add              rcx, 8
    0x140bffa86    48898c2480000000         mov              qword ptr [rsp + 0x80], rcx
    0x140bffa8e    488d0ddb5bf101           lea              rcx, [rip + 0x1f15bdb] -> 0x142b15670
    0x140bffa95    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x140bffa9a    488d8c2480000000         lea              rcx, [rsp + 0x80]
    0x140bffaa2    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x140bffaa7    48c744242004000000       mov              qword ptr [rsp + 0x20], 4
    0x140bffab0    488d150779f101           lea              rdx, [rip + 0x1f17907] -> 0x142b173be
    0x140bffab7    4c8d0d0e79f101           lea              r9, [rip + 0x1f1790e] -> 0x142b173cc
    0x140bffabe    41b80e000000             mov              r8d, 0xe
    0x140bffac4    4889c1                   mov              rcx, rax
    0x140bffac7    e824711800               call             0x140d86bf0 -> core::fmt::Formatter::debug_struct_field1_finish::hd9497ec2d71b9977
    0x140bffacc    90                       nop              
    0x140bffacd    4881c488000000           add              rsp, 0x88
    0x140bffad4    c3                       ret              

=== 0x140c02bf0 size=421  rhapsody::raw::error::impl$1::fmt
    ; Leaf/impl fully decoded from PE bytes (semantics mostly diagnostic/formatting); see insns.  [medium]
    0x140c02bf0    4881ec88000000           sub              rsp, 0x88
    0x140c02bf7    4889d0                   mov              rax, rdx
    0x140c02bfa    4c8b01                   mov              r8, qword ptr [rcx]
    0x140c02bfd    4d8d48f7                 lea              r9, [r8 - 9]
    0x140c02c01    4983c0f8                 add              r8, -8
    0x140c02c05    31d2                     xor              edx, edx
    0x140c02c07    4983f902                 cmp              r9, 2
    0x140c02c0b    490f42d0                 cmovb            rdx, r8
    0x140c02c0f    4885d2                   test             rdx, rdx
    0x140c02c12    0f8487000000             je               0x140c02c9f
    0x140c02c18    4883fa01                 cmp              rdx, 1
    0x140c02c1c    0f8520010000             jne              0x140c02d42
    0x140c02c22    488d5120                 lea              rdx, [rcx + 0x20]
    0x140c02c26    4883c108                 add              rcx, 8
    0x140c02c2a    48898c2480000000         mov              qword ptr [rsp + 0x80], rcx
    0x140c02c32    488d0def54f101           lea              rcx, [rip + 0x1f154ef] -> 0x142b18128
    0x140c02c39    48894c2450               mov              qword ptr [rsp + 0x50], rcx
    0x140c02c3e    488d8c2480000000         lea              rcx, [rsp + 0x80]
    0x140c02c46    48894c2448               mov              qword ptr [rsp + 0x48], rcx
    0x140c02c4b    488d0dff54f101           lea              rcx, [rip + 0x1f154ff] -> 0x142b18151
    0x140c02c52    48894c2438               mov              qword ptr [rsp + 0x38], rcx
    0x140c02c57    488d0daa54f101           lea              rcx, [rip + 0x1f154aa] -> 0x142b18108
    0x140c02c5e    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x140c02c63    4889542428               mov              qword ptr [rsp + 0x28], rdx
    0x140c02c68    48c744244005000000       mov              qword ptr [rsp + 0x40], 5
    0x140c02c71    48c744242003000000       mov              qword ptr [rsp + 0x20], 3
    0x140c02c7a    488d15c754f101           lea              rdx, [rip + 0x1f154c7] -> 0x142b18148
    0x140c02c81    4c8d0dc654f101           lea              r9, [rip + 0x1f154c6] -> 0x142b1814e
    0x140c02c88    41b806000000             mov              r8d, 6
    0x140c02c8e    4889c1                   mov              rcx, rax
    0x140c02c91    e80a401800               call             0x140d86ca0 -> core::fmt::Formatter::debug_struct_field2_finish::h35c7e6c960cc525a
    0x140c02c96    90                       nop              
    0x140c02c97    4881c488000000           add              rsp, 0x88
    0x140c02c9e    c3                       ret              
    0x140c02c9f    488d5120                 lea              rdx, [rcx + 0x20]
    0x140c02ca3    4c8d4138                 lea              r8, [rcx + 0x38]
    0x140c02ca7    4c89842480000000         mov              qword ptr [rsp + 0x80], r8
    0x140c02caf    4c8d051254f101           lea              r8, [rip + 0x1f15412] -> 0x142b180c8
    0x140c02cb6    4c89442470               mov              qword ptr [rsp + 0x70], r8
    0x140c02cbb    4c8d842480000000         lea              r8, [rsp + 0x80]
    0x140c02cc3    4c89442468               mov              qword ptr [rsp + 0x68], r8
    0x140c02cc8    4c8d052c54f101           lea              r8, [rip + 0x1f1542c] -> 0x142b180fb
    0x140c02ccf    4c89442458               mov              qword ptr [rsp + 0x58], r8
    0x140c02cd4    4c8d05cd53f101           lea              r8, [rip + 0x1f153cd] -> 0x142b180a8
    0x140c02cdb    4c89442450               mov              qword ptr [rsp + 0x50], r8
    0x140c02ce0    4889542448               mov              qword ptr [rsp + 0x48], rdx
    0x140c02ce5    488d150b54f101           lea              rdx, [rip + 0x1f1540b] -> 0x142b180f7
    0x140c02cec    4889542438               mov              qword ptr [rsp + 0x38], rdx
    0x140c02cf1    488d159053f101           lea              rdx, [rip + 0x1f15390] -> 0x142b18088
    0x140c02cf8    4889542430               mov              qword ptr [rsp + 0x30], rdx
    0x140c02cfd    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x140c02d02    48c74424600c000000       mov              qword ptr [rsp + 0x60], 0xc
    0x140c02d0b    48c744244004000000       mov              qword ptr [rsp + 0x40], 4
    0x140c02d14    48c744242005000000       mov              qword ptr [rsp + 0x20], 5
    0x140c02d1d    488d15c453f101           lea              rdx, [rip + 0x1f153c4] -> 0x142b180e8
    0x140c02d24    4c8d0dc753f101           lea              r9, [rip + 0x1f153c7] -> 0x142b180f2
    0x140c02d2b    41b80a000000             mov              r8d, 0xa
    0x140c02d31    4889c1                   mov              rcx, rax
    0x140c02d34    e857401800               call             0x140d86d90 -> core::fmt::Formatter::debug_struct_field3_finish::h024add6f4e520141
    0x140c02d39    90                       nop              
    0x140c02d3a    4881c488000000           add              rsp, 0x88
    0x140c02d41    c3                       ret              
    0x140c02d42    4883c108                 add              rcx, 8
    0x140c02d46    48898c2480000000         mov              qword ptr [rsp + 0x80], rcx
    0x140c02d4e    488d0d9350f101           lea              rcx, [rip + 0x1f15093] -> 0x142b17de8
    0x140c02d55    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x140c02d5a    488d8c2480000000         lea              rcx, [rsp + 0x80]
    0x140c02d62    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x140c02d67    48c744242004000000       mov              qword ptr [rsp + 0x20], 4
    0x140c02d70    488d15df53f101           lea              rdx, [rip + 0x1f153df] -> 0x142b18156
    0x140c02d77    4c8d0de653f101           lea              r9, [rip + 0x1f153e6] -> 0x142b18164
    0x140c02d7e    41b80e000000             mov              r8d, 0xe
    0x140c02d84    4889c1                   mov              rcx, rax
    0x140c02d87    e8643e1800               call             0x140d86bf0 -> core::fmt::Formatter::debug_struct_field1_finish::hd9497ec2d71b9977
    0x140c02d8c    90                       nop              
    0x140c02d8d    4881c488000000           add              rsp, 0x88
    0x140c02d94    c3                       ret              

=== 0x140c04280 size=728  rhapsody::raw::device::GraphicsDevice::with_setup_cb::h1957c6f842c39dc5
    ; Large function body fully decoded (not a micro-leaf); decode present in insns.  [medium]
    0x140c04280    55                       push             rbp
    0x140c04281    4156                     push             r14
    0x140c04283    56                       push             rsi
    0x140c04284    57                       push             rdi
    0x140c04285    53                       push             rbx
    0x140c04286    4881ec00010000           sub              rsp, 0x100
    0x140c0428d    488dac2480000000         lea              rbp, [rsp + 0x80]
    0x140c04295    48c74578feffffff         mov              qword ptr [rbp + 0x78], 0xfffffffffffffffe
    0x140c0429d    4c89c7                   mov              rdi, r8
    0x140c042a0    4989d6                   mov              r14, rdx
    0x140c042a3    4889ce                   mov              rsi, rcx
    0x140c042a6    488d9af0090000           lea              rbx, [rdx + 0x9f0]
    0x140c042ad    b101                     mov              cl, 1
    0x140c042af    31c0                     xor              eax, eax
    0x140c042b1    f00fb08af0090000         lock cmpxchg     byte ptr [rdx + 0x9f0], cl
    0x140c042b9    0f8552010000             jne              0x140c04411
    0x140c042bf    48895d60                 mov              qword ptr [rbp + 0x60], rbx
    0x140c042c3    4889d9                   mov              rcx, rbx
    0x140c042c6    e8a5d90700               call             0x140c81c70 -> parking_lot_core::parking_lot::deadlock_impl::acquire_resource::hb2f16ac28e483830
    0x140c042cb    498d9ef8090000           lea              rbx, [r14 + 0x9f8]
    0x140c042d2    4183bef809000004         cmp              dword ptr [r14 + 0x9f8], 4
    0x140c042da    0f85f8000000             jne              0x140c043d8
    0x140c042e0    418b86d40a0000           mov              eax, dword ptr [r14 + 0xad4]
    0x140c042e7    89442420                 mov              dword ptr [rsp + 0x20], eax
    0x140c042eb    4c8d059644f101           lea              r8, [rip + 0x1f14496] -> 0x142b18788
    0x140c042f2    488d4da8                 lea              rcx, [rbp - 0x58]
    0x140c042f6    41b908000000             mov              r9d, 8
    0x140c042fc    4c89f2                   mov              rdx, r14
    0x140c042ff    e89cb1feff               call             0x140bef4a0 -> rhapsody::raw::command_buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_command_buffer::h590d93c90b46ccfb
    0x140c04304    c6457701                 mov              byte ptr [rbp + 0x77], 1
    0x140c04308    488d4da8                 lea              rcx, [rbp - 0x58]
    0x140c0430c    4c89f2                   mov              rdx, r14
    0x140c0430f    4531c0                   xor              r8d, r8d
    0x140c04312    e8198ffeff               call             0x140bed230 -> rhapsody::raw::command_buffer::CommandBuffer::begin
    0x140c04317    c6457701                 mov              byte ptr [rbp + 0x77], 1
    0x140c0431b    488d55a8                 lea              rdx, [rbp - 0x58]
    0x140c0431f    4889f9                   mov              rcx, rdi
    0x140c04322    e8d9f6ffff               call             0x140c03a00 -> rhapsody::raw::buffer::impl$6::update_buffer_bytes::closure$0
    0x140c04327    488b45f8                 mov              rax, qword ptr [rbp - 8]
    0x140c0432b    48894550                 mov              qword ptr [rbp + 0x50], rax
    0x140c0432f    0f1045e8                 movups           xmm0, xmmword ptr [rbp - 0x18]
    0x140c04333    0f294540                 movaps           xmmword ptr [rbp + 0x40], xmm0
    0x140c04337    0f1045a8                 movups           xmm0, xmmword ptr [rbp - 0x58]
    0x140c0433b    0f104db8                 movups           xmm1, xmmword ptr [rbp - 0x48]
    0x140c0433f    0f1055c8                 movups           xmm2, xmmword ptr [rbp - 0x38]
    0x140c04343    0f105dd8                 movups           xmm3, xmmword ptr [rbp - 0x28]
    0x140c04347    0f295d30                 movaps           xmmword ptr [rbp + 0x30], xmm3
    0x140c0434b    0f295520                 movaps           xmmword ptr [rbp + 0x20], xmm2
    0x140c0434f    0f294d10                 movaps           xmmword ptr [rbp + 0x10], xmm1
    0x140c04353    0f294500                 movaps           xmmword ptr [rbp], xmm0
    0x140c04357    48895d68                 mov              qword ptr [rbp + 0x68], rbx
    0x140c0435b    488b03                   mov              rax, qword ptr [rbx]
    0x140c0435e    4883f804                 cmp              rax, 4
    0x140c04362    743f                     je               0x140c043a3
    0x140c04364    4885c0                   test             rax, rax
    0x140c04367    7409                     je               0x140c04372
    0x140c04369    83f803                   cmp              eax, 3
    0x140c0436c    0f85be000000             jne              0x140c04430
    0x140c04372    498b96100a0000           mov              rdx, qword ptr [r14 + 0xa10]
    0x140c04379    4885d2                   test             rdx, rdx
    0x140c0437c    7416                     je               0x140c04394
    0x140c0437e    498b8e180a0000           mov              rcx, qword ptr [r14 + 0xa18]
    0x140c04385    48c1e203                 shl              rdx, 3
    0x140c04389    41b808000000             mov              r8d, 8
    0x140c0438f    e88cf8a0ff               call             0x140613c20 -> __rust_dealloc
    0x140c04394    4981c6280a0000           add              r14, 0xa28
    0x140c0439b    4c89f1                   mov              rcx, r14
    0x140c0439e    e85db29dff               call             0x1405df600 -> core::ptr::drop_in_place$LT$alloc..vec..Vec$LT$$LP$ash..vk..definitions..DescriptorPool$C$alloc..sync..Arc$LT$rhapsody..raw..shader..DescriptorPoolDiscardPile$GT$$RP$$GT$$GT$::h3c71e16060d2e3d5 (.llvm.10346396820423203432)
    0x140c043a3    488b4550                 mov              rax, qword ptr [rbp + 0x50]
    0x140c043a7    488b4d68                 mov              rcx, qword ptr [rbp + 0x68]
    0x140c043ab    48894150                 mov              qword ptr [rcx + 0x50], rax
    0x140c043af    0f284540                 movaps           xmm0, xmmword ptr [rbp + 0x40]
    0x140c043b3    0f114140                 movups           xmmword ptr [rcx + 0x40], xmm0
    0x140c043b7    0f284500                 movaps           xmm0, xmmword ptr [rbp]
    0x140c043bb    0f284d10                 movaps           xmm1, xmmword ptr [rbp + 0x10]
    0x140c043bf    0f285520                 movaps           xmm2, xmmword ptr [rbp + 0x20]
    0x140c043c3    0f285d30                 movaps           xmm3, xmmword ptr [rbp + 0x30]
    0x140c043c7    0f115930                 movups           xmmword ptr [rcx + 0x30], xmm3
    0x140c043cb    0f115120                 movups           xmmword ptr [rcx + 0x20], xmm2
    0x140c043cf    0f114910                 movups           xmmword ptr [rcx + 0x10], xmm1
    0x140c043d3    0f1101                   movups           xmmword ptr [rcx], xmm0
    0x140c043d6    eb0b                     jmp              0x140c043e3
    0x140c043d8    4889f9                   mov              rcx, rdi
    0x140c043db    4889da                   mov              rdx, rbx
    0x140c043de    e81df6ffff               call             0x140c03a00 -> rhapsody::raw::buffer::impl$6::update_buffer_bytes::closure$0
    0x140c043e3    48c7060b000000           mov              qword ptr [rsi], 0xb
    0x140c043ea    488b7d60                 mov              rdi, qword ptr [rbp + 0x60]
    0x140c043ee    4889f9                   mov              rcx, rdi
    0x140c043f1    e81ada0700               call             0x140c81e10 -> parking_lot_core::parking_lot::deadlock_impl::release_resource::h1c3ab4db915ad6f2
    0x140c043f6    31c9                     xor              ecx, ecx
    0x140c043f8    b001                     mov              al, 1
    0x140c043fa    f00fb00f                 lock cmpxchg     byte ptr [rdi], cl
    0x140c043fe    7524                     jne              0x140c04424
    0x140c04400    4889f0                   mov              rax, rsi
    0x140c04403    4881c400010000           add              rsp, 0x100
    0x140c0440a    5b                       pop              rbx
    0x140c0440b    5f                       pop              rdi
    0x140c0440c    5e                       pop              rsi
    0x140c0440d    415e                     pop              r14
    0x140c0440f    5d                       pop              rbp
    0x140c04410    c3                       ret              
    0x140c04411    4889d9                   mov              rcx, rbx
    0x140c04414    41b800ca9a3b             mov              r8d, 0x3b9aca00
    0x140c0441a    e8f18dcc01               call             0x1428cd210 -> parking_lot::raw_mutex::RawMutex::lock_slow::h7c244b522f551b38
    0x140c0441f    e99bfeffff               jmp              0x140c042bf
    0x140c04424    4889f9                   mov              rcx, rdi
    0x140c04427    31d2                     xor              edx, edx
    0x140c04429    e86297cc01               call             0x1428cdb90 -> parking_lot::raw_mutex::RawMutex::unlock_slow::h56cb6df3e32a2657
    0x140c0442e    ebd0                     jmp              0x140c04400
    0x140c04430    4c897558                 mov              qword ptr [rbp + 0x58], r14
    0x140c04434    488d0d1de7f001           lea              rcx, [rip + 0x1f0e71d] -> 0x142b12b58
    0x140c0443b    4c8d05aee7f001           lea              r8, [rip + 0x1f0e7ae] -> 0x142b12bf0
    0x140c04442    ba6e000000               mov              edx, 0x6e
    0x140c04447    e8e44fcd01               call             0x1428d9430 -> core::panicking::panic::hb210ae7b817a9f13
    0x140c0444c    0f0b                     ud2              
    0x140c0444e    6690                     nop              
    0x140c04450    4889542410               mov              qword ptr [rsp + 0x10], rdx
    0x140c04455    55                       push             rbp
    0x140c04456    4156                     push             r14
    0x140c04458    56                       push             rsi
    0x140c04459    57                       push             rdi
    0x140c0445a    53                       push             rbx
    0x140c0445b    4883ec30                 sub              rsp, 0x30
    0x140c0445f    488daa80000000           lea              rbp, [rdx + 0x80]
    0x140c04466    488b4d58                 mov              rcx, qword ptr [rbp + 0x58]
    0x140c0446a    4881c1100a0000           add              rcx, 0xa10
    0x140c04471    e81aae9fff               call             0x1405ff290 -> core::ptr::drop_in_place$LT$rhapsody..raw..command_buffer..CommandBufferResources$GT$::h7083992529333ee9 (.llvm.10346396820423203432)
    0x140c04476    90                       nop              
    0x140c04477    4883c430                 add              rsp, 0x30
    0x140c0447b    5b                       pop              rbx
    0x140c0447c    5f                       pop              rdi
    0x140c0447d    5e                       pop              rsi
    0x140c0447e    415e                     pop              r14
    0x140c04480    5d                       pop              rbp
    0x140c04481    c3                       ret              
    0x140c04482    66666666662e0f1f840000000000 nop              word ptr cs:[rax + rax]
    0x140c04490    4889542410               mov              qword ptr [rsp + 0x10], rdx
    0x140c04495    55                       push             rbp
    0x140c04496    4156                     push             r14
    0x140c04498    56                       push             rsi
    0x140c04499    57                       push             rdi
    0x140c0449a    53                       push             rbx
    0x140c0449b    4883ec30                 sub              rsp, 0x30
    0x140c0449f    488daa80000000           lea              rbp, [rdx + 0x80]
    0x140c044a6    807d7700                 cmp              byte ptr [rbp + 0x77], 0
    0x140c044aa    7415                     je               0x140c044c1
    0x140c044ac    488d4da8                 lea              rcx, [rbp - 0x58]
    0x140c044b0    e84b9afdff               call             0x140bddf00 -> core::ptr::drop_in_place$LT$rhapsody..raw..command_buffer..CommandBuffer$GT$::h7ed4c42d7635be8b
    0x140c044b5    90                       nop              
    0x140c044b6    4883c430                 add              rsp, 0x30
    0x140c044ba    5b                       pop              rbx
    0x140c044bb    5f                       pop              rdi
    0x140c044bc    5e                       pop              rsi
    0x140c044bd    415e                     pop              r14
    0x140c044bf    5d                       pop              rbp
    0x140c044c0    c3                       ret              
    0x140c044c1    4883c430                 add              rsp, 0x30
    0x140c044c5    5b                       pop              rbx
    0x140c044c6    5f                       pop              rdi
    0x140c044c7    5e                       pop              rsi
    0x140c044c8    415e                     pop              r14
    0x140c044ca    5d                       pop              rbp
    0x140c044cb    c3                       ret              
    0x140c044cc    0f1f4000                 nop              dword ptr [rax]
    0x140c044d0    4889542410               mov              qword ptr [rsp + 0x10], rdx
    0x140c044d5    55                       push             rbp
    0x140c044d6    4156                     push             r14
    0x140c044d8    56                       push             rsi
    0x140c044d9    57                       push             rdi
    0x140c044da    53                       push             rbx
    0x140c044db    4883ec30                 sub              rsp, 0x30
    0x140c044df    488daa80000000           lea              rbp, [rdx + 0x80]
    0x140c044e6    488b4d60                 mov              rcx, qword ptr [rbp + 0x60]
    0x140c044ea    e841d755ff               call             0x140161c30 -> core::ptr::drop_in_place$LT$lock_api..mutex..MutexGuard$LT$parking_lot..raw_mutex..RawMutex$C$alloc..vec..Vec$LT$glam..f32..vec2..Vec2$GT$$GT$$GT$::hc1588d55d4486b0e (.llvm.14454403653567781691)
    0x140c044ef    90                       nop              
    0x140c044f0    4883c430                 add              rsp, 0x30
    0x140c044f4    5b                       pop              rbx
    0x140c044f5    5f                       pop              rdi
    0x140c044f6    5e                       pop              rsi
    0x140c044f7    415e                     pop              r14
    0x140c044f9    5d                       pop              rbp
    0x140c044fa    c3                       ret              
    0x140c044fb    0f1f440000               nop              dword ptr [rax + rax]
    0x140c04500    4889542410               mov              qword ptr [rsp + 0x10], rdx
    0x140c04505    55                       push             rbp
    0x140c04506    4156                     push             r14
    0x140c04508    56                       push             rsi
    0x140c04509    57                       push             rdi
    0x140c0450a    53                       push             rbx
    0x140c0450b    4883ec30                 sub              rsp, 0x30
    0x140c0450f    488daa80000000           lea              rbp, [rdx + 0x80]
    0x140c04516    488b4550                 mov              rax, qword ptr [rbp + 0x50]
    0x140c0451a    488b4d68                 mov              rcx, qword ptr [rbp + 0x68]
    0x140c0451e    48894150                 mov              qword ptr [rcx + 0x50], rax
    0x140c04522    0f284540                 movaps           xmm0, xmmword ptr [rbp + 0x40]
    0x140c04526    0f114140                 movups           xmmword ptr [rcx + 0x40], xmm0
    0x140c0452a    0f284500                 movaps           xmm0, xmmword ptr [rbp]
    0x140c0452e    0f284d10                 movaps           xmm1, xmmword ptr [rbp + 0x10]
    0x140c04532    0f285520                 movaps           xmm2, xmmword ptr [rbp + 0x20]
    0x140c04536    0f285d30                 movaps           xmm3, xmmword ptr [rbp + 0x30]
    0x140c0453a    0f115930                 movups           xmmword ptr [rcx + 0x30], xmm3
    0x140c0453e    0f115120                 movups           xmmword ptr [rcx + 0x20], xmm2
    0x140c04542    0f114910                 movups           xmmword ptr [rcx + 0x10], xmm1
    0x140c04546    0f1101                   movups           xmmword ptr [rcx], xmm0
    0x140c04549    c6457700                 mov              byte ptr [rbp + 0x77], 0
    0x140c0454d    4883c430                 add              rsp, 0x30
    0x140c04551    5b                       pop              rbx
    0x140c04552    5f                       pop              rdi
    0x140c04553    5e                       pop              rsi
    0x140c04554    415e                     pop              r14
    0x140c04556    5d                       pop              rbp
    0x140c04557    c3                       ret              

=== 0x140c04560 size=728  rhapsody::raw::device::GraphicsDevice::with_setup_cb::h32f08cab0871e752
    ; Large function body fully decoded (not a micro-leaf); decode present in insns.  [medium]
    0x140c04560    55                       push             rbp
    0x140c04561    4156                     push             r14
    0x140c04563    56                       push             rsi
    0x140c04564    57                       push             rdi
    0x140c04565    53                       push             rbx
    0x140c04566    4881ec00010000           sub              rsp, 0x100
    0x140c0456d    488dac2480000000         lea              rbp, [rsp + 0x80]
    0x140c04575    48c74578feffffff         mov              qword ptr [rbp + 0x78], 0xfffffffffffffffe
    0x140c0457d    4c89c7                   mov              rdi, r8
    0x140c04580    4989d6                   mov              r14, rdx
    0x140c04583    4889ce                   mov              rsi, rcx
    0x140c04586    488d9af0090000           lea              rbx, [rdx + 0x9f0]
    0x140c0458d    b101                     mov              cl, 1
    0x140c0458f    31c0                     xor              eax, eax
    0x140c04591    f00fb08af0090000         lock cmpxchg     byte ptr [rdx + 0x9f0], cl
    0x140c04599    0f8552010000             jne              0x140c046f1
    0x140c0459f    48895d60                 mov              qword ptr [rbp + 0x60], rbx
    0x140c045a3    4889d9                   mov              rcx, rbx
    0x140c045a6    e8c5d60700               call             0x140c81c70 -> parking_lot_core::parking_lot::deadlock_impl::acquire_resource::hb2f16ac28e483830
    0x140c045ab    498d9ef8090000           lea              rbx, [r14 + 0x9f8]
    0x140c045b2    4183bef809000004         cmp              dword ptr [r14 + 0x9f8], 4
    0x140c045ba    0f85f8000000             jne              0x140c046b8
    0x140c045c0    418b86d40a0000           mov              eax, dword ptr [r14 + 0xad4]
    0x140c045c7    89442420                 mov              dword ptr [rsp + 0x20], eax
    0x140c045cb    4c8d05b641f101           lea              r8, [rip + 0x1f141b6] -> 0x142b18788
    0x140c045d2    488d4da8                 lea              rcx, [rbp - 0x58]
    0x140c045d6    41b908000000             mov              r9d, 8
    0x140c045dc    4c89f2                   mov              rdx, r14
    0x140c045df    e8bcaefeff               call             0x140bef4a0 -> rhapsody::raw::command_buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_command_buffer::h590d93c90b46ccfb
    0x140c045e4    c6457701                 mov              byte ptr [rbp + 0x77], 1
    0x140c045e8    488d4da8                 lea              rcx, [rbp - 0x58]
    0x140c045ec    4c89f2                   mov              rdx, r14
    0x140c045ef    4531c0                   xor              r8d, r8d
    0x140c045f2    e8398cfeff               call             0x140bed230 -> rhapsody::raw::command_buffer::CommandBuffer::begin
    0x140c045f7    c6457701                 mov              byte ptr [rbp + 0x77], 1
    0x140c045fb    488d55a8                 lea              rdx, [rbp - 0x58]
    0x140c045ff    4889f9                   mov              rcx, rdi
    0x140c04602    e829420000               call             0x140c08830 -> rhapsody::raw::texture::impl$9::update_texture_impl::closure$2
    0x140c04607    488b45f8                 mov              rax, qword ptr [rbp - 8]
    0x140c0460b    48894550                 mov              qword ptr [rbp + 0x50], rax
    0x140c0460f    0f1045e8                 movups           xmm0, xmmword ptr [rbp - 0x18]
    0x140c04613    0f294540                 movaps           xmmword ptr [rbp + 0x40], xmm0
    0x140c04617    0f1045a8                 movups           xmm0, xmmword ptr [rbp - 0x58]
    0x140c0461b    0f104db8                 movups           xmm1, xmmword ptr [rbp - 0x48]
    0x140c0461f    0f1055c8                 movups           xmm2, xmmword ptr [rbp - 0x38]
    0x140c04623    0f105dd8                 movups           xmm3, xmmword ptr [rbp - 0x28]
    0x140c04627    0f295d30                 movaps           xmmword ptr [rbp + 0x30], xmm3
    0x140c0462b    0f295520                 movaps           xmmword ptr [rbp + 0x20], xmm2
    0x140c0462f    0f294d10                 movaps           xmmword ptr [rbp + 0x10], xmm1
    0x140c04633    0f294500                 movaps           xmmword ptr [rbp], xmm0
    0x140c04637    48895d68                 mov              qword ptr [rbp + 0x68], rbx
    0x140c0463b    488b03                   mov              rax, qword ptr [rbx]
    0x140c0463e    4883f804                 cmp              rax, 4
    0x140c04642    743f                     je               0x140c04683
    0x140c04644    4885c0                   test             rax, rax
    0x140c04647    7409                     je               0x140c04652
    0x140c04649    83f803                   cmp              eax, 3
    0x140c0464c    0f85be000000             jne              0x140c04710
    0x140c04652    498b96100a0000           mov              rdx, qword ptr [r14 + 0xa10]
    0x140c04659    4885d2                   test             rdx, rdx
    0x140c0465c    7416                     je               0x140c04674
    0x140c0465e    498b8e180a0000           mov              rcx, qword ptr [r14 + 0xa18]
    0x140c04665    48c1e203                 shl              rdx, 3
    0x140c04669    41b808000000             mov              r8d, 8
    0x140c0466f    e8acf5a0ff               call             0x140613c20 -> __rust_dealloc
    0x140c04674    4981c6280a0000           add              r14, 0xa28
    0x140c0467b    4c89f1                   mov              rcx, r14
    0x140c0467e    e87daf9dff               call             0x1405df600 -> core::ptr::drop_in_place$LT$alloc..vec..Vec$LT$$LP$ash..vk..definitions..DescriptorPool$C$alloc..sync..Arc$LT$rhapsody..raw..shader..DescriptorPoolDiscardPile$GT$$RP$$GT$$GT$::h3c71e16060d2e3d5 (.llvm.10346396820423203432)
    0x140c04683    488b4550                 mov              rax, qword ptr [rbp + 0x50]
    0x140c04687    488b4d68                 mov              rcx, qword ptr [rbp + 0x68]
    0x140c0468b    48894150                 mov              qword ptr [rcx + 0x50], rax
    0x140c0468f    0f284540                 movaps           xmm0, xmmword ptr [rbp + 0x40]
    0x140c04693    0f114140                 movups           xmmword ptr [rcx + 0x40], xmm0
    0x140c04697    0f284500                 movaps           xmm0, xmmword ptr [rbp]
    0x140c0469b    0f284d10                 movaps           xmm1, xmmword ptr [rbp + 0x10]
    0x140c0469f    0f285520                 movaps           xmm2, xmmword ptr [rbp + 0x20]
    0x140c046a3    0f285d30                 movaps           xmm3, xmmword ptr [rbp + 0x30]
    0x140c046a7    0f115930                 movups           xmmword ptr [rcx + 0x30], xmm3
    0x140c046ab    0f115120                 movups           xmmword ptr [rcx + 0x20], xmm2
    0x140c046af    0f114910                 movups           xmmword ptr [rcx + 0x10], xmm1
    0x140c046b3    0f1101                   movups           xmmword ptr [rcx], xmm0
    0x140c046b6    eb0b                     jmp              0x140c046c3
    0x140c046b8    4889f9                   mov              rcx, rdi
    0x140c046bb    4889da                   mov              rdx, rbx
    0x140c046be    e86d410000               call             0x140c08830 -> rhapsody::raw::texture::impl$9::update_texture_impl::closure$2
    0x140c046c3    48c7060b000000           mov              qword ptr [rsi], 0xb
    0x140c046ca    488b7d60                 mov              rdi, qword ptr [rbp + 0x60]
    0x140c046ce    4889f9                   mov              rcx, rdi
    0x140c046d1    e83ad70700               call             0x140c81e10 -> parking_lot_core::parking_lot::deadlock_impl::release_resource::h1c3ab4db915ad6f2
    0x140c046d6    31c9                     xor              ecx, ecx
    0x140c046d8    b001                     mov              al, 1
    0x140c046da    f00fb00f                 lock cmpxchg     byte ptr [rdi], cl
    0x140c046de    7524                     jne              0x140c04704
    0x140c046e0    4889f0                   mov              rax, rsi
    0x140c046e3    4881c400010000           add              rsp, 0x100
    0x140c046ea    5b                       pop              rbx
    0x140c046eb    5f                       pop              rdi
    0x140c046ec    5e                       pop              rsi
    0x140c046ed    415e                     pop              r14
    0x140c046ef    5d                       pop              rbp
    0x140c046f0    c3                       ret              
    0x140c046f1    4889d9                   mov              rcx, rbx
    0x140c046f4    41b800ca9a3b             mov              r8d, 0x3b9aca00
    0x140c046fa    e8118bcc01               call             0x1428cd210 -> parking_lot::raw_mutex::RawMutex::lock_slow::h7c244b522f551b38
    0x140c046ff    e99bfeffff               jmp              0x140c0459f
    0x140c04704    4889f9                   mov              rcx, rdi
    0x140c04707    31d2                     xor              edx, edx
    0x140c04709    e88294cc01               call             0x1428cdb90 -> parking_lot::raw_mutex::RawMutex::unlock_slow::h56cb6df3e32a2657
    0x140c0470e    ebd0                     jmp              0x140c046e0
    0x140c04710    4c897558                 mov              qword ptr [rbp + 0x58], r14
    0x140c04714    488d0d3de4f001           lea              rcx, [rip + 0x1f0e43d] -> 0x142b12b58
    0x140c0471b    4c8d05cee4f001           lea              r8, [rip + 0x1f0e4ce] -> 0x142b12bf0
    0x140c04722    ba6e000000               mov              edx, 0x6e
    0x140c04727    e8044dcd01               call             0x1428d9430 -> core::panicking::panic::hb210ae7b817a9f13
    0x140c0472c    0f0b                     ud2              
    0x140c0472e    6690                     nop              
    0x140c04730    4889542410               mov              qword ptr [rsp + 0x10], rdx
    0x140c04735    55                       push             rbp
    0x140c04736    4156                     push             r14
    0x140c04738    56                       push             rsi
    0x140c04739    57                       push             rdi
    0x140c0473a    53                       push             rbx
    0x140c0473b    4883ec30                 sub              rsp, 0x30
    0x140c0473f    488daa80000000           lea              rbp, [rdx + 0x80]
    0x140c04746    488b4d58                 mov              rcx, qword ptr [rbp + 0x58]
    0x140c0474a    4881c1100a0000           add              rcx, 0xa10
    0x140c04751    e83aab9fff               call             0x1405ff290 -> core::ptr::drop_in_place$LT$rhapsody..raw..command_buffer..CommandBufferResources$GT$::h7083992529333ee9 (.llvm.10346396820423203432)
    0x140c04756    90                       nop              
    0x140c04757    4883c430                 add              rsp, 0x30
    0x140c0475b    5b                       pop              rbx
    0x140c0475c    5f                       pop              rdi
    0x140c0475d    5e                       pop              rsi
    0x140c0475e    415e                     pop              r14
    0x140c04760    5d                       pop              rbp
    0x140c04761    c3                       ret              
    0x140c04762    66666666662e0f1f840000000000 nop              word ptr cs:[rax + rax]
    0x140c04770    4889542410               mov              qword ptr [rsp + 0x10], rdx
    0x140c04775    55                       push             rbp
    0x140c04776    4156                     push             r14
    0x140c04778    56                       push             rsi
    0x140c04779    57                       push             rdi
    0x140c0477a    53                       push             rbx
    0x140c0477b    4883ec30                 sub              rsp, 0x30
    0x140c0477f    488daa80000000           lea              rbp, [rdx + 0x80]
    0x140c04786    807d7700                 cmp              byte ptr [rbp + 0x77], 0
    0x140c0478a    7415                     je               0x140c047a1
    0x140c0478c    488d4da8                 lea              rcx, [rbp - 0x58]
    0x140c04790    e86b97fdff               call             0x140bddf00 -> core::ptr::drop_in_place$LT$rhapsody..raw..command_buffer..CommandBuffer$GT$::h7ed4c42d7635be8b
    0x140c04795    90                       nop              
    0x140c04796    4883c430                 add              rsp, 0x30
    0x140c0479a    5b                       pop              rbx
    0x140c0479b    5f                       pop              rdi
    0x140c0479c    5e                       pop              rsi
    0x140c0479d    415e                     pop              r14
    0x140c0479f    5d                       pop              rbp
    0x140c047a0    c3                       ret              
    0x140c047a1    4883c430                 add              rsp, 0x30
    0x140c047a5    5b                       pop              rbx
    0x140c047a6    5f                       pop              rdi
    0x140c047a7    5e                       pop              rsi
    0x140c047a8    415e                     pop              r14
    0x140c047aa    5d                       pop              rbp
    0x140c047ab    c3                       ret              
    0x140c047ac    0f1f4000                 nop              dword ptr [rax]
    0x140c047b0    4889542410               mov              qword ptr [rsp + 0x10], rdx
    0x140c047b5    55                       push             rbp
    0x140c047b6    4156                     push             r14
    0x140c047b8    56                       push             rsi
    0x140c047b9    57                       push             rdi
    0x140c047ba    53                       push             rbx
    0x140c047bb    4883ec30                 sub              rsp, 0x30
    0x140c047bf    488daa80000000           lea              rbp, [rdx + 0x80]
    0x140c047c6    488b4d60                 mov              rcx, qword ptr [rbp + 0x60]
    0x140c047ca    e861d455ff               call             0x140161c30 -> core::ptr::drop_in_place$LT$lock_api..mutex..MutexGuard$LT$parking_lot..raw_mutex..RawMutex$C$alloc..vec..Vec$LT$glam..f32..vec2..Vec2$GT$$GT$$GT$::hc1588d55d4486b0e (.llvm.14454403653567781691)
    0x140c047cf    90                       nop              
    0x140c047d0    4883c430                 add              rsp, 0x30
    0x140c047d4    5b                       pop              rbx
    0x140c047d5    5f                       pop              rdi
    0x140c047d6    5e                       pop              rsi
    0x140c047d7    415e                     pop              r14
    0x140c047d9    5d                       pop              rbp
    0x140c047da    c3                       ret              
    0x140c047db    0f1f440000               nop              dword ptr [rax + rax]
    0x140c047e0    4889542410               mov              qword ptr [rsp + 0x10], rdx
    0x140c047e5    55                       push             rbp
    0x140c047e6    4156                     push             r14
    0x140c047e8    56                       push             rsi
    0x140c047e9    57                       push             rdi
    0x140c047ea    53                       push             rbx
    0x140c047eb    4883ec30                 sub              rsp, 0x30
    0x140c047ef    488daa80000000           lea              rbp, [rdx + 0x80]
    0x140c047f6    488b4550                 mov              rax, qword ptr [rbp + 0x50]
    0x140c047fa    488b4d68                 mov              rcx, qword ptr [rbp + 0x68]
    0x140c047fe    48894150                 mov              qword ptr [rcx + 0x50], rax
    0x140c04802    0f284540                 movaps           xmm0, xmmword ptr [rbp + 0x40]
    0x140c04806    0f114140                 movups           xmmword ptr [rcx + 0x40], xmm0
    0x140c0480a    0f284500                 movaps           xmm0, xmmword ptr [rbp]
    0x140c0480e    0f284d10                 movaps           xmm1, xmmword ptr [rbp + 0x10]
    0x140c04812    0f285520                 movaps           xmm2, xmmword ptr [rbp + 0x20]
    0x140c04816    0f285d30                 movaps           xmm3, xmmword ptr [rbp + 0x30]
    0x140c0481a    0f115930                 movups           xmmword ptr [rcx + 0x30], xmm3
    0x140c0481e    0f115120                 movups           xmmword ptr [rcx + 0x20], xmm2
    0x140c04822    0f114910                 movups           xmmword ptr [rcx + 0x10], xmm1
    0x140c04826    0f1101                   movups           xmmword ptr [rcx], xmm0
    0x140c04829    c6457700                 mov              byte ptr [rbp + 0x77], 0
    0x140c0482d    4883c430                 add              rsp, 0x30
    0x140c04831    5b                       pop              rbx
    0x140c04832    5f                       pop              rdi
    0x140c04833    5e                       pop              rsi
    0x140c04834    415e                     pop              r14
    0x140c04836    5d                       pop              rbp
    0x140c04837    c3                       ret              

=== 0x140c08ac0 size=365  rhapsody::raw::command_buffer::impl$11::fmt
    ; Leaf/impl fully decoded from PE bytes (semantics mostly diagnostic/formatting); see insns.  [medium]
    0x140c08ac0    4883ec68                 sub              rsp, 0x68
    0x140c08ac4    4889d0                   mov              rax, rdx
    0x140c08ac7    488b11                   mov              rdx, qword ptr [rcx]
    0x140c08aca    4c8d05fb02f101           lea              r8, [rip + 0x1f102fb] -> 0x142b18dcc
    0x140c08ad1    49631490                 movsxd           rdx, dword ptr [r8 + rdx*4]
    0x140c08ad5    4c01c2                   add              rdx, r8
    0x140c08ad8    ffe2                     jmp              rdx
    0x140c08ada    488d155f01f101           lea              rdx, [rip + 0x1f1015f] -> 0x142b18c40
    0x140c08ae1    41b807000000             mov              r8d, 7
    0x140c08ae7    4889c1                   mov              rcx, rax
    0x140c08aea    4883c468                 add              rsp, 0x68
    0x140c08aee    e99de01700               jmp              0x140d86b90 -> _$LT$core..fmt..Formatter$u20$as$u20$core..fmt..Write$GT$::write_str::h5c3f9ed06cfb23bd
    0x140c08af3    488d5110                 lea              rdx, [rcx + 0x10]
    0x140c08af7    4883c108                 add              rcx, 8
    0x140c08afb    48894c2460               mov              qword ptr [rsp + 0x60], rcx
    0x140c08b00    488d0d6101f101           lea              rcx, [rip + 0x1f10161] -> 0x142b18c68
    0x140c08b07    48894c2450               mov              qword ptr [rsp + 0x50], rcx
    0x140c08b0c    488d4c2460               lea              rcx, [rsp + 0x60]
    0x140c08b11    48894c2448               mov              qword ptr [rsp + 0x48], rcx
    0x140c08b16    488d0d7901f101           lea              rcx, [rip + 0x1f10179] -> 0x142b18c96
    0x140c08b1d    48894c2438               mov              qword ptr [rsp + 0x38], rcx
    0x140c08b22    488d0d1f01f101           lea              rcx, [rip + 0x1f1011f] -> 0x142b18c48
    0x140c08b29    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x140c08b2e    4889542428               mov              qword ptr [rsp + 0x28], rdx
    0x140c08b33    48c744244003000000       mov              qword ptr [rsp + 0x40], 3
    0x140c08b3c    48c744242005000000       mov              qword ptr [rsp + 0x20], 5
    0x140c08b45    488d154d01f101           lea              rdx, [rip + 0x1f1014d] -> 0x142b18c99
    0x140c08b4c    4c8d0d3e01f101           lea              r9, [rip + 0x1f1013e] -> 0x142b18c91
    0x140c08b53    41b808000000             mov              r8d, 8
    0x140c08b59    e9c1000000               jmp              0x140c08c1f
    0x140c08b5e    488d5110                 lea              rdx, [rcx + 0x10]
    0x140c08b62    4883c108                 add              rcx, 8
    0x140c08b66    48894c2460               mov              qword ptr [rsp + 0x60], rcx
    0x140c08b6b    488d0df600f101           lea              rcx, [rip + 0x1f100f6] -> 0x142b18c68
    0x140c08b72    48894c2450               mov              qword ptr [rsp + 0x50], rcx
    0x140c08b77    488d4c2460               lea              rcx, [rsp + 0x60]
    0x140c08b7c    48894c2448               mov              qword ptr [rsp + 0x48], rcx
    0x140c08b81    488d0d0e01f101           lea              rcx, [rip + 0x1f1010e] -> 0x142b18c96
    0x140c08b88    48894c2438               mov              qword ptr [rsp + 0x38], rcx
    0x140c08b8d    488d0db400f101           lea              rcx, [rip + 0x1f100b4] -> 0x142b18c48
    0x140c08b94    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x140c08b99    4889542428               mov              qword ptr [rsp + 0x28], rdx
    0x140c08b9e    48c744244003000000       mov              qword ptr [rsp + 0x40], 3
    0x140c08ba7    48c744242005000000       mov              qword ptr [rsp + 0x20], 5
    0x140c08bb0    488d15ea00f101           lea              rdx, [rip + 0x1f100ea] -> 0x142b18ca1
    0x140c08bb7    eb59                     jmp              0x140c08c12
    0x140c08bb9    488d5110                 lea              rdx, [rcx + 0x10]
    0x140c08bbd    4883c108                 add              rcx, 8
    0x140c08bc1    48894c2460               mov              qword ptr [rsp + 0x60], rcx
    0x140c08bc6    488d0d9b00f101           lea              rcx, [rip + 0x1f1009b] -> 0x142b18c68
    0x140c08bcd    48894c2450               mov              qword ptr [rsp + 0x50], rcx
    0x140c08bd2    488d4c2460               lea              rcx, [rsp + 0x60]
    0x140c08bd7    48894c2448               mov              qword ptr [rsp + 0x48], rcx
    0x140c08bdc    488d0db300f101           lea              rcx, [rip + 0x1f100b3] -> 0x142b18c96
    0x140c08be3    48894c2438               mov              qword ptr [rsp + 0x38], rcx
    0x140c08be8    488d0d5900f101           lea              rcx, [rip + 0x1f10059] -> 0x142b18c48
    0x140c08bef    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x140c08bf4    4889542428               mov              qword ptr [rsp + 0x28], rdx
    0x140c08bf9    48c744244003000000       mov              qword ptr [rsp + 0x40], 3
    0x140c08c02    48c744242005000000       mov              qword ptr [rsp + 0x20], 5
    0x140c08c0b    488d157600f101           lea              rdx, [rip + 0x1f10076] -> 0x142b18c88
    0x140c08c12    4c8d0d7800f101           lea              r9, [rip + 0x1f10078] -> 0x142b18c91
    0x140c08c19    41b809000000             mov              r8d, 9
    0x140c08c1f    4889c1                   mov              rcx, rax
    0x140c08c22    e879e01700               call             0x140d86ca0 -> core::fmt::Formatter::debug_struct_field2_finish::h35c7e6c960cc525a
    0x140c08c27    90                       nop              
    0x140c08c28    4883c468                 add              rsp, 0x68
    0x140c08c2c    c3                       ret              

=== 0x140c08c30 size=421  rhapsody::raw::error::impl$1::fmt
    ; Leaf/impl fully decoded from PE bytes (semantics mostly diagnostic/formatting); see insns.  [medium]
    0x140c08c30    4881ec88000000           sub              rsp, 0x88
    0x140c08c37    4889d0                   mov              rax, rdx
    0x140c08c3a    4c8b01                   mov              r8, qword ptr [rcx]
    0x140c08c3d    4d8d48f7                 lea              r9, [r8 - 9]
    0x140c08c41    4983c0f8                 add              r8, -8
    0x140c08c45    31d2                     xor              edx, edx
    0x140c08c47    4983f902                 cmp              r9, 2
    0x140c08c4b    490f42d0                 cmovb            rdx, r8
    0x140c08c4f    4885d2                   test             rdx, rdx
    0x140c08c52    0f8487000000             je               0x140c08cdf
    0x140c08c58    4883fa01                 cmp              rdx, 1
    0x140c08c5c    0f8520010000             jne              0x140c08d82
    0x140c08c62    488d5120                 lea              rdx, [rcx + 0x20]
    0x140c08c66    4883c108                 add              rcx, 8
    0x140c08c6a    48898c2480000000         mov              qword ptr [rsp + 0x80], rcx
    0x140c08c72    488d0db700f101           lea              rcx, [rip + 0x1f100b7] -> 0x142b18d30
    0x140c08c79    48894c2450               mov              qword ptr [rsp + 0x50], rcx
    0x140c08c7e    488d8c2480000000         lea              rcx, [rsp + 0x80]
    0x140c08c86    48894c2448               mov              qword ptr [rsp + 0x48], rcx
    0x140c08c8b    488d0dc700f101           lea              rcx, [rip + 0x1f100c7] -> 0x142b18d59
    0x140c08c92    48894c2438               mov              qword ptr [rsp + 0x38], rcx
    0x140c08c97    488d0d1af6f001           lea              rcx, [rip + 0x1f0f61a] -> 0x142b182b8
    0x140c08c9e    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x140c08ca3    4889542428               mov              qword ptr [rsp + 0x28], rdx
    0x140c08ca8    48c744244005000000       mov              qword ptr [rsp + 0x40], 5
    0x140c08cb1    48c744242003000000       mov              qword ptr [rsp + 0x20], 3
    0x140c08cba    488d158f00f101           lea              rdx, [rip + 0x1f1008f] -> 0x142b18d50
    0x140c08cc1    4c8d0d8e00f101           lea              r9, [rip + 0x1f1008e] -> 0x142b18d56
    0x140c08cc8    41b806000000             mov              r8d, 6
    0x140c08cce    4889c1                   mov              rcx, rax
    0x140c08cd1    e8cadf1700               call             0x140d86ca0 -> core::fmt::Formatter::debug_struct_field2_finish::h35c7e6c960cc525a
    0x140c08cd6    90                       nop              
    0x140c08cd7    4881c488000000           add              rsp, 0x88
    0x140c08cde    c3                       ret              
    0x140c08cdf    488d5120                 lea              rdx, [rcx + 0x20]
    0x140c08ce3    4c8d4138                 lea              r8, [rcx + 0x38]
    0x140c08ce7    4c89842480000000         mov              qword ptr [rsp + 0x80], r8
    0x140c08cef    4c8d05fafff001           lea              r8, [rip + 0x1f0fffa] -> 0x142b18cf0
    0x140c08cf6    4c89442470               mov              qword ptr [rsp + 0x70], r8
    0x140c08cfb    4c8d842480000000         lea              r8, [rsp + 0x80]
    0x140c08d03    4c89442468               mov              qword ptr [rsp + 0x68], r8
    0x140c08d08    4c8d051400f101           lea              r8, [rip + 0x1f10014] -> 0x142b18d23
    0x140c08d0f    4c89442458               mov              qword ptr [rsp + 0x58], r8
    0x140c08d14    4c8d05b5fff001           lea              r8, [rip + 0x1f0ffb5] -> 0x142b18cd0
    0x140c08d1b    4c89442450               mov              qword ptr [rsp + 0x50], r8
    0x140c08d20    4889542448               mov              qword ptr [rsp + 0x48], rdx
    0x140c08d25    488d15f3fff001           lea              rdx, [rip + 0x1f0fff3] -> 0x142b18d1f
    0x140c08d2c    4889542438               mov              qword ptr [rsp + 0x38], rdx
    0x140c08d31    488d1578fff001           lea              rdx, [rip + 0x1f0ff78] -> 0x142b18cb0
    0x140c08d38    4889542430               mov              qword ptr [rsp + 0x30], rdx
    0x140c08d3d    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x140c08d42    48c74424600c000000       mov              qword ptr [rsp + 0x60], 0xc
    0x140c08d4b    48c744244004000000       mov              qword ptr [rsp + 0x40], 4
    0x140c08d54    48c744242005000000       mov              qword ptr [rsp + 0x20], 5
    0x140c08d5d    488d15acfff001           lea              rdx, [rip + 0x1f0ffac] -> 0x142b18d10
    0x140c08d64    4c8d0daffff001           lea              r9, [rip + 0x1f0ffaf] -> 0x142b18d1a
    0x140c08d6b    41b80a000000             mov              r8d, 0xa
    0x140c08d71    4889c1                   mov              rcx, rax
    0x140c08d74    e817e01700               call             0x140d86d90 -> core::fmt::Formatter::debug_struct_field3_finish::h024add6f4e520141
    0x140c08d79    90                       nop              
    0x140c08d7a    4881c488000000           add              rsp, 0x88
    0x140c08d81    c3                       ret              
    0x140c08d82    4883c108                 add              rcx, 8
    0x140c08d86    48898c2480000000         mov              qword ptr [rsp + 0x80], rcx
    0x140c08d8e    488d0d73f7f001           lea              rcx, [rip + 0x1f0f773] -> 0x142b18508
    0x140c08d95    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x140c08d9a    488d8c2480000000         lea              rcx, [rsp + 0x80]
    0x140c08da2    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x140c08da7    48c744242004000000       mov              qword ptr [rsp + 0x20], 4
    0x140c08db0    488d15a7fff001           lea              rdx, [rip + 0x1f0ffa7] -> 0x142b18d5e
    0x140c08db7    4c8d0daefff001           lea              r9, [rip + 0x1f0ffae] -> 0x142b18d6c
    0x140c08dbe    41b80e000000             mov              r8d, 0xe
    0x140c08dc4    4889c1                   mov              rcx, rax
    0x140c08dc7    e824de1700               call             0x140d86bf0 -> core::fmt::Formatter::debug_struct_field1_finish::hd9497ec2d71b9977
    0x140c08dcc    90                       nop              
    0x140c08dcd    4881c488000000           add              rsp, 0x88
    0x140c08dd4    c3                       ret              

=== 0x140c08de0 size=90  rhapsody::raw::submit::impl$9::fmt
    ; Leaf/impl fully decoded from PE bytes (semantics mostly diagnostic/formatting); see insns.  [medium]
    0x140c08de0    4883ec38                 sub              rsp, 0x38
    0x140c08de4    4889d0                   mov              rax, rdx
    0x140c08de7    f60101                   test             byte ptr [rcx], 1
    0x140c08dea    7435                     je               0x140c08e21
    0x140c08dec    4883c104                 add              rcx, 4
    0x140c08df0    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x140c08df5    488d0d84fff001           lea              rcx, [rip + 0x1f0ff84] -> 0x142b18d80
    0x140c08dfc    48894c2420               mov              qword ptr [rsp + 0x20], rcx
    0x140c08e01    488d1598fff001           lea              rdx, [rip + 0x1f0ff98] -> 0x142b18da0
    0x140c08e08    4c8d4c2430               lea              r9, [rsp + 0x30]
    0x140c08e0d    41b807000000             mov              r8d, 7
    0x140c08e13    4889c1                   mov              rcx, rax
    0x140c08e16    e8b5e41700               call             0x140d872d0 -> core::fmt::Formatter::debug_tuple_field1_finish::ha115499c8c7eba6f
    0x140c08e1b    90                       nop              
    0x140c08e1c    4883c438                 add              rsp, 0x38
    0x140c08e20    c3                       ret              
    0x140c08e21    488d1548fff001           lea              rdx, [rip + 0x1f0ff48] -> 0x142b18d70
    0x140c08e28    41b809000000             mov              r8d, 9
    0x140c08e2e    4889c1                   mov              rcx, rax
    0x140c08e31    4883c438                 add              rsp, 0x38
    0x140c08e35    e956dd1700               jmp              0x140d86b90 -> _$LT$core..fmt..Formatter$u20$as$u20$core..fmt..Write$GT$::write_str::h5c3f9ed06cfb23bd

=== 0x140faeba0 size=2300  rhapsody::raw::texture::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_texture::h9c6e17a8e883ced4
    ; Large function body fully decoded (not a micro-leaf); decode present in insns.  [medium]
    0x140faeba0    55                       push             rbp
    0x140faeba1    4157                     push             r15
    0x140faeba3    4156                     push             r14
    0x140faeba5    4155                     push             r13
    0x140faeba7    4154                     push             r12
    0x140faeba9    56                       push             rsi
    0x140faebaa    57                       push             rdi
    0x140faebab    53                       push             rbx
    0x140faebac    4881ec18040000           sub              rsp, 0x418
    0x140faebb3    488dac2480000000         lea              rbp, [rsp + 0x80]
    0x140faebbb    48c78590030000feffffff   mov              qword ptr [rbp + 0x390], 0xfffffffffffffffe
    0x140faebc6    4d89ce                   mov              r14, r9
    0x140faebc9    4d89c7                   mov              r15, r8
    0x140faebcc    4889d3                   mov              rbx, rdx
    0x140faebcf    4889ce                   mov              rsi, rcx
    0x140faebd2    488bbd00040000           mov              rdi, qword ptr [rbp + 0x400]
    0x140faebd9    488b9508040000           mov              rdx, qword ptr [rbp + 0x408]
    0x140faebe0    4c8b8510040000           mov              r8, qword ptr [rbp + 0x410]
    0x140faebe7    488d8dd8000000           lea              rcx, [rbp + 0xd8]
    0x140faebee    e84d6ec2ff               call             0x140bd5a40 -> rhapsody::raw::texture::impl$4::from
    0x140faebf3    8b07                     mov              eax, dword ptr [rdi]
    0x140faebf5    85c0                     test             eax, eax
    0x140faebf7    0f8406070000             je               0x140faf303
    0x140faebfd    4c8ba5d8000000           mov              r12, qword ptr [rbp + 0xd8]
    0x140faec04    4d85e4                   test             r12, r12
    0x140faec07    7405                     je               0x140faec0e
    0x140faec09    83c802                   or               eax, 2
    0x140faec0c    8907                     mov              dword ptr [rdi], eax
    0x140faec0e    4d85e4                   test             r12, r12
    0x140faec11    410f95c0                 setne            r8b
    0x140faec15    488d8d80000000           lea              rcx, [rbp + 0x80]
    0x140faec1c    4889fa                   mov              rdx, rdi
    0x140faec1f    e8ac72c2ff               call             0x140bd5ed0 -> rhapsody::raw::texture::get_image_create_info
    0x140faec24    83bd9c00000000           cmp              dword ptr [rbp + 0x9c], 0
    0x140faec2b    0f84ea060000             je               0x140faf31b
    0x140faec31    83bda000000000           cmp              dword ptr [rbp + 0xa0], 0
    0x140faec38    0f84f5060000             je               0x140faf333
    0x140faec3e    83bda400000000           cmp              dword ptr [rbp + 0xa4], 0
    0x140faec45    0f8400070000             je               0x140faf34b
    0x140faec4b    4c89a568020000           mov              qword ptr [rbp + 0x268], r12
    0x140faec52    488b8b40060000           mov              rcx, qword ptr [rbx + 0x640]
    0x140faec59    488d9580000000           lea              rdx, [rbp + 0x80]
    0x140faec60    4c8d8d70020000           lea              r9, [rbp + 0x270]
    0x140faec67    4531c0                   xor              r8d, r8d
    0x140faec6a    ff93a0010000             call             qword ptr [rbx + 0x1a0]
    0x140faec70    85c0                     test             eax, eax
    0x140faec72    0f8514070000             jne              0x140faf38c
    0x140faec78    4c8ba570020000           mov              r12, qword ptr [rbp + 0x270]
    0x140faec7f    4889d9                   mov              rcx, rbx
    0x140faec82    e8d99cc4ff               call             0x140bf8960 -> rhapsody::raw::device::GraphicsDevice::debug_utils
    0x140faec87    4885c0                   test             rax, rax
    0x140faec8a    0f84aa000000             je               0x140faed3a
    0x140faec90    4989c5                   mov              r13, rax
    0x140faec93    4889b580030000           mov              qword ptr [rbp + 0x380], rsi
    0x140faec9a    488d8df0000000           lea              rcx, [rbp + 0xf0]
    0x140faeca1    4c89fe                   mov              rsi, r15
    0x140faeca4    4c89fa                   mov              rdx, r15
    0x140faeca7    4d89f7                   mov              r15, r14
    0x140faecaa    4d89f0                   mov              r8, r14
    0x140faecad    e81eabdcff               call             0x140d797d0 -> _$LT$$RF$str$u20$as$u20$alloc..ffi..c_str..CString..new..SpecNewImpl$GT$::spec_new_impl::h27339bb99b671fc9
    0x140faecb2    31c0                     xor              eax, eax
    0x140faecb4    483b85f0000000           cmp              rax, qword ptr [rbp + 0xf0]
    0x140faecbb    0f8139070000             jno              0x140faf3fa
    0x140faecc1    4c8bb5f8000000           mov              r14, qword ptr [rbp + 0xf8]
    0x140faecc8    488bbd00010000           mov              rdi, qword ptr [rbp + 0x100]
    0x140faeccf    c7857002000000be9c3b     mov              dword ptr [rbp + 0x270], 0x3b9cbe00
    0x140faecd9    48c7857802000000000000   mov              qword ptr [rbp + 0x278], 0
    0x140faece4    c785800200000a000000     mov              dword ptr [rbp + 0x280], 0xa
    0x140faecee    4c89a588020000           mov              qword ptr [rbp + 0x288], r12
    0x140faecf5    4c89b590020000           mov              qword ptr [rbp + 0x290], r14
    0x140faecfc    498b4d40                 mov              rcx, qword ptr [r13 + 0x40]
    0x140faed00    488d9570020000           lea              rdx, [rbp + 0x270]
    0x140faed07    41ff5500                 call             qword ptr [r13]
    0x140faed0b    85c0                     test             eax, eax
    0x140faed0d    0f8530070000             jne              0x140faf443
    0x140faed13    41c60600                 mov              byte ptr [r14], 0
    0x140faed17    4885ff                   test             rdi, rdi
    0x140faed1a    4c89f1                   mov              rcx, r14
    0x140faed1d    4d89fe                   mov              r14, r15
    0x140faed20    4989f7                   mov              r15, rsi
    0x140faed23    488bb580030000           mov              rsi, qword ptr [rbp + 0x380]
    0x140faed2a    740e                     je               0x140faed3a
    0x140faed2c    41b801000000             mov              r8d, 1
    0x140faed32    4889fa                   mov              rdx, rdi
    0x140faed35    e8e64e66ff               call             0x140613c20 -> __rust_dealloc
    0x140faed3a    488b8b40060000           mov              rcx, qword ptr [rbx + 0x640]
    0x140faed41    4c8d8550030000           lea              r8, [rbp + 0x350]
    0x140faed48    4c89e2                   mov              rdx, r12
    0x140faed4b    ff93f0000000             call             qword ptr [rbx + 0xf0]
    0x140faed51    4c8bab78060000           mov              r13, qword ptr [rbx + 0x678]
    0x140faed58    498d7d10                 lea              rdi, [r13 + 0x10]
    0x140faed5c    b101                     mov              cl, 1
    0x140faed5e    31c0                     xor              eax, eax
    0x140faed60    f0410fb04d10             lock cmpxchg     byte ptr [r13 + 0x10], cl
    0x140faed66    0f85f7050000             jne              0x140faf363
    0x140faed6c    4889bd78030000           mov              qword ptr [rbp + 0x378], rdi
    0x140faed73    4889f9                   mov              rcx, rdi
    0x140faed76    e8f52ecdff               call             0x140c81c70 -> parking_lot_core::parking_lot::deadlock_impl::acquire_resource::hb2f16ac28e483830
    0x140faed7b    4983c518                 add              r13, 0x18
    0x140faed7f    4c89bd00010000           mov              qword ptr [rbp + 0x100], r15
    0x140faed86    4c89b508010000           mov              qword ptr [rbp + 0x108], r14
    0x140faed8d    0f108550030000           movups           xmm0, xmmword ptr [rbp + 0x350]
    0x140faed94    0f118510010000           movups           xmmword ptr [rbp + 0x110], xmm0
    0x140faed9b    488b8560030000           mov              rax, qword ptr [rbp + 0x360]
    0x140faeda2    48898520010000           mov              qword ptr [rbp + 0x120], rax
    0x140faeda9    66c785280100000001       mov              word ptr [rbp + 0x128], 0x100
    0x140faedb2    48c785f000000002000000   mov              qword ptr [rbp + 0xf0], 2
    0x140faedbd    488d8d70020000           lea              rcx, [rbp + 0x270]
    0x140faedc4    4c8d85f0000000           lea              r8, [rbp + 0xf0]
    0x140faedcb    4c89ea                   mov              rdx, r13
    0x140faedce    e86decc5ff               call             0x140c0da40 -> rhapsody::raw::allocator::GpuAllocator::allocate
    0x140faedd3    80bdb402000002           cmp              byte ptr [rbp + 0x2b4], 2
    0x140faedda    488b8570020000           mov              rax, qword ptr [rbp + 0x270]
    0x140faede1    48898580030000           mov              qword ptr [rbp + 0x380], rax
    0x140faede8    4c8bad88020000           mov              r13, qword ptr [rbp + 0x288]
    0x140faedef    0f85a8000000             jne              0x140faee9d
    0x140faedf5    488b8578020000           mov              rax, qword ptr [rbp + 0x278]
    0x140faedfc    48898570030000           mov              qword ptr [rbp + 0x370], rax
    0x140faee03    488b8580020000           mov              rax, qword ptr [rbp + 0x280]
    0x140faee0a    48898568030000           mov              qword ptr [rbp + 0x368], rax
    0x140faee11    0fb605f15d6802           movzx            eax, byte ptr [rip + 0x2685df1] -> 0x143634c09
    0x140faee18    bf0f000000               mov              edi, 0xf
    0x140faee1d    b90f000000               mov              ecx, 0xf
    0x140faee22    ba01000000               mov              edx, 1
    0x140faee27    e8e44d66ff               call             0x140613c10 -> __rust_alloc
    0x140faee2c    4885c0                   test             rax, rax
    0x140faee2f    0f844f060000             je               0x140faf484
    0x140faee35    48b92074657874757265     movabs           rcx, 0x6572757478657420
    0x140faee3f    48894807                 mov              qword ptr [rax + 7], rcx
    0x140faee43    48b94770754f6e6c7920     movabs           rcx, 0x20796c6e4f757047
    0x140faee4d    488908                   mov              qword ptr [rax], rcx
    0x140faee50    488b8d60030000           mov              rcx, qword ptr [rbp + 0x360]
    0x140faee57    48898dc0010000           mov              qword ptr [rbp + 0x1c0], rcx
    0x140faee5e    0f108550030000           movups           xmm0, xmmword ptr [rbp + 0x350]
    0x140faee65    0f2985b0010000           movaps           xmmword ptr [rbp + 0x1b0], xmm0
    0x140faee6c    488b8d80030000           mov              rcx, qword ptr [rbp + 0x380]
    0x140faee73    4883f90b                 cmp              rcx, 0xb
    0x140faee77    0f8597030000             jne              0x140faf214
    0x140faee7d    4c89e9                   mov              rcx, r13
    0x140faee80    488b9570030000           mov              rdx, qword ptr [rbp + 0x370]
    0x140faee87    48899580030000           mov              qword ptr [rbp + 0x380], rdx
    0x140faee8e    41bd0f000000             mov              r13d, 0xf
    0x140faee94    4c8b8568030000           mov              r8, qword ptr [rbp + 0x368]
    0x140faee9b    eb38                     jmp              0x140faeed5
    0x140faee9d    4c8b8578020000           mov              r8, qword ptr [rbp + 0x278]
    0x140faeea4    488b8d80020000           mov              rcx, qword ptr [rbp + 0x280]
    0x140faeeab    488b8590020000           mov              rax, qword ptr [rbp + 0x290]
    0x140faeeb2    488bbd98020000           mov              rdi, qword ptr [rbp + 0x298]
    0x140faeeb9    0f1085a0020000           movups           xmm0, xmmword ptr [rbp + 0x2a0]
    0x140faeec0    0f2985b0010000           movaps           xmmword ptr [rbp + 0x1b0], xmm0
    0x140faeec7    488b95b0020000           mov              rdx, qword ptr [rbp + 0x2b0]
    0x140faeece    488995c0010000           mov              qword ptr [rbp + 0x1c0], rdx
    0x140faeed5    488b95c0010000           mov              rdx, qword ptr [rbp + 0x1c0]
    0x140faeedc    48899510020000           mov              qword ptr [rbp + 0x210], rdx
    0x140faeee3    0f2885b0010000           movaps           xmm0, xmmword ptr [rbp + 0x1b0]
    0x140faeeea    0f298500020000           movaps           xmmword ptr [rbp + 0x200], xmm0
    0x140faeef1    4c8b8d80030000           mov              r9, qword ptr [rbp + 0x380]
    0x140faeef8    4c898d20020000           mov              qword ptr [rbp + 0x220], r9
    0x140faeeff    4c898528020000           mov              qword ptr [rbp + 0x228], r8
    0x140faef06    48898d30020000           mov              qword ptr [rbp + 0x230], rcx
    0x140faef0d    0f118550020000           movups           xmmword ptr [rbp + 0x250], xmm0
    0x140faef14    48899560020000           mov              qword ptr [rbp + 0x260], rdx
    0x140faef1b    4c89ad38020000           mov              qword ptr [rbp + 0x238], r13
    0x140faef22    48898540020000           mov              qword ptr [rbp + 0x240], rax
    0x140faef29    4889bd48020000           mov              qword ptr [rbp + 0x248], rdi
    0x140faef30    c6858f03000001           mov              byte ptr [rbp + 0x38f], 1
    0x140faef37    488bbd78030000           mov              rdi, qword ptr [rbp + 0x378]
    0x140faef3e    4889f9                   mov              rcx, rdi
    0x140faef41    e8ca2ecdff               call             0x140c81e10 -> parking_lot_core::parking_lot::deadlock_impl::release_resource::h1c3ab4db915ad6f2
    0x140faef46    31c9                     xor              ecx, ecx
    0x140faef48    b001                     mov              al, 1
    0x140faef4a    f00fb00f                 lock cmpxchg     byte ptr [rdi], cl
    0x140faef4e    0f8522040000             jne              0x140faf376
    0x140faef54    c6858f03000001           mov              byte ptr [rbp + 0x38f], 1
    0x140faef5b    488d8d20020000           lea              rcx, [rbp + 0x220]
    0x140faef62    e8491d72ff               call             0x1406d0cb0 -> gpu_allocator::vulkan::Allocation::memory::hfb1e6a660a853183
    0x140faef67    4989c5                   mov              r13, rax
    0x140faef6a    c6858f03000001           mov              byte ptr [rbp + 0x38f], 1
    0x140faef71    488d8d20020000           lea              rcx, [rbp + 0x220]
    0x140faef78    e833796dff               call             0x1406868b0 -> rhapsody::raw::buffer::impl$3::byte_count
    0x140faef7d    488b8b40060000           mov              rcx, qword ptr [rbx + 0x640]
    0x140faef84    4c89e2                   mov              rdx, r12
    0x140faef87    4d89e8                   mov              r8, r13
    0x140faef8a    4989c1                   mov              r9, rax
    0x140faef8d    ff93e0000000             call             qword ptr [rbx + 0xe0]
    0x140faef93    85c0                     test             eax, eax
    0x140faef95    488b9500040000           mov              rdx, qword ptr [rbp + 0x400]
    0x140faef9c    0f851b040000             jne              0x140faf3bd
    0x140faefa2    488d4378                 lea              rax, [rbx + 0x78]
    0x140faefa6    4c8bab48060000           mov              r13, qword ptr [rbx + 0x648]
    0x140faefad    488b8d60020000           mov              rcx, qword ptr [rbp + 0x260]
    0x140faefb4    48898df0010000           mov              qword ptr [rbp + 0x1f0], rcx
    0x140faefbb    0f108520020000           movups           xmm0, xmmword ptr [rbp + 0x220]
    0x140faefc2    0f108d30020000           movups           xmm1, xmmword ptr [rbp + 0x230]
    0x140faefc9    0f109540020000           movups           xmm2, xmmword ptr [rbp + 0x240]
    0x140faefd0    0f109d50020000           movups           xmm3, xmmword ptr [rbp + 0x250]
    0x140faefd7    0f299de0010000           movaps           xmmword ptr [rbp + 0x1e0], xmm3
    0x140faefde    0f2995d0010000           movaps           xmmword ptr [rbp + 0x1d0], xmm2
    0x140faefe5    0f298dc0010000           movaps           xmmword ptr [rbp + 0x1c0], xmm1
    0x140faefec    0f2985b0010000           movaps           xmmword ptr [rbp + 0x1b0], xmm0
    0x140faeff3    0f1002                   movups           xmm0, xmmword ptr [rdx]
    0x140faeff6    0f104a10                 movups           xmm1, xmmword ptr [rdx + 0x10]
    0x140faeffa    0f298d10020000           movaps           xmmword ptr [rbp + 0x210], xmm1
    0x140faf001    0f298500020000           movaps           xmmword ptr [rbp + 0x200], xmm0
    0x140faf008    c6858f03000000           mov              byte ptr [rbp + 0x38f], 0
    0x140faf00f    488d8d00020000           lea              rcx, [rbp + 0x200]
    0x140faf016    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x140faf01b    488d8db0010000           lea              rcx, [rbp + 0x1b0]
    0x140faf022    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x140faf027    4889442420               mov              qword ptr [rsp + 0x20], rax
    0x140faf02c    488d8d70020000           lea              rcx, [rbp + 0x270]
    0x140faf033    4c89fa                   mov              rdx, r15
    0x140faf036    4d89f0                   mov              r8, r14
    0x140faf039    4d89e1                   mov              r9, r12
    0x140faf03c    e88fd2c2ff               call             0x140bdc2d0 -> rhapsody::raw::managed::texture::ManagedTexture::new
    0x140faf041    0fb6bd2c030000           movzx            edi, byte ptr [rbp + 0x32c]
    0x140faf048    4080ff03                 cmp              dil, 3
    0x140faf04c    755e                     jne              0x140faf0ac
    0x140faf04e    0f1085b0020000           movups           xmm0, xmmword ptr [rbp + 0x2b0]
    0x140faf055    0f298530010000           movaps           xmmword ptr [rbp + 0x130], xmm0
    0x140faf05c    0f108d70020000           movups           xmm1, xmmword ptr [rbp + 0x270]
    0x140faf063    0f109580020000           movups           xmm2, xmmword ptr [rbp + 0x280]
    0x140faf06a    0f109d90020000           movups           xmm3, xmmword ptr [rbp + 0x290]
    0x140faf071    0f10a5a0020000           movups           xmm4, xmmword ptr [rbp + 0x2a0]
    0x140faf078    0f29a520010000           movaps           xmmword ptr [rbp + 0x120], xmm4
    0x140faf07f    0f299d10010000           movaps           xmmword ptr [rbp + 0x110], xmm3
    0x140faf086    0f299500010000           movaps           xmmword ptr [rbp + 0x100], xmm2
    0x140faf08d    0f298df0000000           movaps           xmmword ptr [rbp + 0xf0], xmm1
    0x140faf094    0f114640                 movups           xmmword ptr [rsi + 0x40], xmm0
    0x140faf098    0f116630                 movups           xmmword ptr [rsi + 0x30], xmm4
    0x140faf09c    0f115e20                 movups           xmmword ptr [rsi + 0x20], xmm3
    0x140faf0a0    0f115610                 movups           xmmword ptr [rsi + 0x10], xmm2
    0x140faf0a4    0f110e                   movups           xmmword ptr [rsi], xmm1
    0x140faf0a7    e940020000               jmp              0x140faf2ec
    0x140faf0ac    4c8db5f0000000           lea              r14, [rbp + 0xf0]
    0x140faf0b3    488d9570020000           lea              rdx, [rbp + 0x270]
    0x140faf0ba    41b8bc000000             mov              r8d, 0xbc
    0x140faf0c0    4c89f1                   mov              rcx, r14
    0x140faf0c3    e8384b9701               call             0x142923c00 -> memcpy
    0x140faf0c8    0fb7852d030000           movzx            eax, word ptr [rbp + 0x32d]
    0x140faf0cf    6689457d                 mov              word ptr [rbp + 0x7d], ax
    0x140faf0d3    0fb6852f030000           movzx            eax, byte ptr [rbp + 0x32f]
    0x140faf0da    88457f                   mov              byte ptr [rbp + 0x7f], al
    0x140faf0dd    4c8d7dc0                 lea              r15, [rbp - 0x40]
    0x140faf0e1    41b8bc000000             mov              r8d, 0xbc
    0x140faf0e7    4c89f9                   mov              rcx, r15
    0x140faf0ea    4c89f2                   mov              rdx, r14
    0x140faf0ed    e80e4b9701               call             0x142923c00 -> memcpy
    0x140faf0f2    40887d7c                 mov              byte ptr [rbp + 0x7c], dil
    0x140faf0f6    c6858f03000000           mov              byte ptr [rbp + 0x38f], 0
    0x140faf0fd    4c89e9                   mov              rcx, r13
    0x140faf100    4c89fa                   mov              rdx, r15
    0x140faf103    e848ccc2ff               call             0x140bdbd50 -> rhapsody::raw::managed::resources::impl$6::allocate
    0x140faf108    8b480c                   mov              ecx, dword ptr [rax + 0xc]
    0x140faf10b    410fb795500a0000         movzx            edx, word ptr [r13 + 0xa50]
    0x140faf113    4d8b4500                 mov              r8, qword ptr [r13]
    0x140faf117    4c898538030000           mov              qword ptr [rbp + 0x338], r8
    0x140faf11e    4989c0                   mov              r8, rax
    0x140faf121    4983c010                 add              r8, 0x10
    0x140faf125    48898540030000           mov              qword ptr [rbp + 0x340], rax
    0x140faf12c    898d48030000             mov              dword ptr [rbp + 0x348], ecx
    0x140faf132    6689954c030000           mov              word ptr [rbp + 0x34c], dx
    0x140faf139    c6854e03000000           mov              byte ptr [rbp + 0x34e], 0
    0x140faf140    4c898530030000           mov              qword ptr [rbp + 0x330], r8
    0x140faf147    4883bd6802000000         cmp              qword ptr [rbp + 0x268], 0
    0x140faf14f    488bbd00040000           mov              rdi, qword ptr [rbp + 0x400]
    0x140faf156    744f                     je               0x140faf1a7
    0x140faf158    0f1007                   movups           xmm0, xmmword ptr [rdi]
    0x140faf15b    0f104f10                 movups           xmm1, xmmword ptr [rdi + 0x10]
    0x140faf15f    0f298d00010000           movaps           xmmword ptr [rbp + 0x100], xmm1
    0x140faf166    0f2985f0000000           movaps           xmmword ptr [rbp + 0xf0], xmm0
    0x140faf16d    488d8530030000           lea              rax, [rbp + 0x330]
    0x140faf174    4889442420               mov              qword ptr [rsp + 0x20], rax
    0x140faf179    488d8d70020000           lea              rcx, [rbp + 0x270]
    0x140faf180    4c8d85f0000000           lea              r8, [rbp + 0xf0]
    0x140faf187    4c8d8dd8000000           lea              r9, [rbp + 0xd8]
    0x140faf18e    4889da                   mov              rdx, rbx
    0x140faf191    e8daf0c4ff               call             0x140bfe270 -> rhapsody::raw::device::GraphicsDevice::init_texture_data
    0x140faf196    488b8570020000           mov              rax, qword ptr [rbp + 0x270]
    0x140faf19d    4883f80b                 cmp              rax, 0xb
    0x140faf1a1    0f85f8000000             jne              0x140faf29f
    0x140faf1a7    488b8548030000           mov              rax, qword ptr [rbp + 0x348]
    0x140faf1ae    48898580020000           mov              qword ptr [rbp + 0x280], rax
    0x140faf1b5    0f108538030000           movups           xmm0, xmmword ptr [rbp + 0x338]
    0x140faf1bc    0f298570020000           movaps           xmmword ptr [rbp + 0x270], xmm0
    0x140faf1c3    0f100f                   movups           xmm1, xmmword ptr [rdi]
    0x140faf1c6    0f105710                 movups           xmm2, xmmword ptr [rdi + 0x10]
    0x140faf1ca    0f119598020000           movups           xmmword ptr [rbp + 0x298], xmm2
    0x140faf1d1    0f118d88020000           movups           xmmword ptr [rbp + 0x288], xmm1
    0x140faf1d8    0f114608                 movups           xmmword ptr [rsi + 8], xmm0
    0x140faf1dc    488b85a0020000           mov              rax, qword ptr [rbp + 0x2a0]
    0x140faf1e3    48894638                 mov              qword ptr [rsi + 0x38], rax
    0x140faf1e7    0f288590020000           movaps           xmm0, xmmword ptr [rbp + 0x290]
    0x140faf1ee    0f114628                 movups           xmmword ptr [rsi + 0x28], xmm0
    0x140faf1f2    488b8580020000           mov              rax, qword ptr [rbp + 0x280]
    0x140faf1f9    48894618                 mov              qword ptr [rsi + 0x18], rax
    0x140faf1fd    488b8588020000           mov              rax, qword ptr [rbp + 0x288]
    0x140faf204    48894620                 mov              qword ptr [rsi + 0x20], rax
    0x140faf208    48c7060b000000           mov              qword ptr [rsi], 0xb
    0x140faf20f    e9d8000000               jmp              0x140faf2ec
    0x140faf214    4889ca                   mov              rdx, rcx
    0x140faf217    488b8d60030000           mov              rcx, qword ptr [rbp + 0x360]
    0x140faf21e    48898d10020000           mov              qword ptr [rbp + 0x210], rcx
    0x140faf225    0f108550030000           movups           xmm0, xmmword ptr [rbp + 0x350]
    0x140faf22c    0f298500020000           movaps           xmmword ptr [rbp + 0x200], xmm0
    0x140faf233    488b8d70030000           mov              rcx, qword ptr [rbp + 0x370]
    0x140faf23a    48894e08                 mov              qword ptr [rsi + 8], rcx
    0x140faf23e    488b8d68030000           mov              rcx, qword ptr [rbp + 0x368]
    0x140faf245    48894e10                 mov              qword ptr [rsi + 0x10], rcx
    0x140faf249    4c896e18                 mov              qword ptr [rsi + 0x18], r13
    0x140faf24d    0f108550030000           movups           xmm0, xmmword ptr [rbp + 0x350]
    0x140faf254    0f114638                 movups           xmmword ptr [rsi + 0x38], xmm0
    0x140faf258    488b8d60030000           mov              rcx, qword ptr [rbp + 0x360]
    0x140faf25f    48894e48                 mov              qword ptr [rsi + 0x48], rcx
    0x140faf263    488916                   mov              qword ptr [rsi], rdx
    0x140faf266    48c746200f000000         mov              qword ptr [rsi + 0x20], 0xf
    0x140faf26e    48894628                 mov              qword ptr [rsi + 0x28], rax
    0x140faf272    48c746300f000000         mov              qword ptr [rsi + 0x30], 0xf
    0x140faf27a    488bbd78030000           mov              rdi, qword ptr [rbp + 0x378]
    0x140faf281    4889f9                   mov              rcx, rdi
    0x140faf284    e8872bcdff               call             0x140c81e10 -> parking_lot_core::parking_lot::deadlock_impl::release_resource::h1c3ab4db915ad6f2
    0x140faf289    31c9                     xor              ecx, ecx
    0x140faf28b    b001                     mov              al, 1
    0x140faf28d    f00fb00f                 lock cmpxchg     byte ptr [rdi], cl
    0x140faf291    7459                     je               0x140faf2ec
    0x140faf293    4889f9                   mov              rcx, rdi
    0x140faf296    31d2                     xor              edx, edx
    0x140faf298    e8f3e89101               call             0x1428cdb90 -> parking_lot::raw_mutex::RawMutex::unlock_slow::h56cb6df3e32a2657
    0x140faf29d    eb4d                     jmp              0x140faf2ec
    0x140faf29f    488b8db8020000           mov              rcx, qword ptr [rbp + 0x2b8]
    0x140faf2a6    48894e48                 mov              qword ptr [rsi + 0x48], rcx
    0x140faf2aa    0f108578020000           movups           xmm0, xmmword ptr [rbp + 0x278]
    0x140faf2b1    0f108d88020000           movups           xmm1, xmmword ptr [rbp + 0x288]
    0x140faf2b8    0f109598020000           movups           xmm2, xmmword ptr [rbp + 0x298]
    0x140faf2bf    0f109da8020000           movups           xmm3, xmmword ptr [rbp + 0x2a8]
    0x140faf2c6    0f115e38                 movups           xmmword ptr [rsi + 0x38], xmm3
    0x140faf2ca    0f115628                 movups           xmmword ptr [rsi + 0x28], xmm2
    0x140faf2ce    0f114e18                 movups           xmmword ptr [rsi + 0x18], xmm1
    0x140faf2d2    0f114608                 movups           xmmword ptr [rsi + 8], xmm0
    0x140faf2d6    488906                   mov              qword ptr [rsi], rax
    0x140faf2d9    c6858f03000000           mov              byte ptr [rbp + 0x38f], 0
    0x140faf2e0    488d8d38030000           lea              rcx, [rbp + 0x338]
    0x140faf2e7    e8c431c3ff               call             0x140be24b0 -> rhapsody::raw::managed::handle::impl$6::drop
    0x140faf2ec    4889f0                   mov              rax, rsi
    0x140faf2ef    4881c418040000           add              rsp, 0x418
    0x140faf2f6    5b                       pop              rbx
    0x140faf2f7    5f                       pop              rdi
    0x140faf2f8    5e                       pop              rsi
    0x140faf2f9    415c                     pop              r12
    0x140faf2fb    415d                     pop              r13
    0x140faf2fd    415e                     pop              r14
    0x140faf2ff    415f                     pop              r15
    0x140faf301    5d                       pop              rbp
    0x140faf302    c3                       ret              
    0x140faf303    488d0da822c001           lea              rcx, [rip + 0x1c022a8] -> 0x142bb15b2
    0x140faf30a    4c8d05df22c001           lea              r8, [rip + 0x1c022df] -> 0x142bb15f0
    0x140faf311    ba3c000000               mov              edx, 0x3c
    0x140faf316    e815a19201               call             0x1428d9430 -> core::panicking::panic::hb210ae7b817a9f13
    0x140faf31b    488d0de622c001           lea              rcx, [rip + 0x1c022e6] -> 0x142bb1608
    0x140faf322    4c8d050f23c001           lea              r8, [rip + 0x1c0230f] -> 0x142bb1638
    0x140faf329    ba2e000000               mov              edx, 0x2e
    0x140faf32e    e8fda09201               call             0x1428d9430 -> core::panicking::panic::hb210ae7b817a9f13
    0x140faf333    488d0d1623c001           lea              rcx, [rip + 0x1c02316] -> 0x142bb1650
    0x140faf33a    4c8d053f23c001           lea              r8, [rip + 0x1c0233f] -> 0x142bb1680
    0x140faf341    ba2f000000               mov              edx, 0x2f
    0x140faf346    e8e5a09201               call             0x1428d9430 -> core::panicking::panic::hb210ae7b817a9f13
    0x140faf34b    488d0d4623c001           lea              rcx, [rip + 0x1c02346] -> 0x142bb1698
    0x140faf352    4c8d056f23c001           lea              r8, [rip + 0x1c0236f] -> 0x142bb16c8
    0x140faf359    ba2e000000               mov              edx, 0x2e
    0x140faf35e    e8cda09201               call             0x1428d9430 -> core::panicking::panic::hb210ae7b817a9f13
    0x140faf363    4889f9                   mov              rcx, rdi
    0x140faf366    41b800ca9a3b             mov              r8d, 0x3b9aca00
    0x140faf36c    e89fde9101               call             0x1428cd210 -> parking_lot::raw_mutex::RawMutex::lock_slow::h7c244b522f551b38
    0x140faf371    e9f6f9ffff               jmp              0x140faed6c
    0x140faf376    c6858f03000001           mov              byte ptr [rbp + 0x38f], 1
    0x140faf37d    4889f9                   mov              rcx, rdi
    0x140faf380    31d2                     xor              edx, edx
    0x140faf382    e809e89101               call             0x1428cdb90 -> parking_lot::raw_mutex::RawMutex::unlock_slow::h56cb6df3e32a2657
    0x140faf387    e9c8fbffff               jmp              0x140faef54
    0x140faf38c    898570020000             mov              dword ptr [rbp + 0x270], eax
    0x140faf392    488d055723c001           lea              rax, [rip + 0x1c02357] -> 0x142bb16f0
    0x140faf399    4889442420               mov              qword ptr [rsp + 0x20], rax
    0x140faf39e    488d0d3b23c001           lea              rcx, [rip + 0x1c0233b] -> 0x142bb16e0
    0x140faf3a5    4c8d0d9c20c001           lea              r9, [rip + 0x1c0209c] -> 0x142bb1448
    0x140faf3ac    4c8d8570020000           lea              r8, [rbp + 0x270]
    0x140faf3b3    ba0c000000               mov              edx, 0xc
    0x140faf3b8    e8a3a39201               call             0x1428d9760 -> core::result::unwrap_failed::h96d74ae09566f4b3
    0x140faf3bd    898570020000             mov              dword ptr [rbp + 0x270], eax
    0x140faf3c3    c6858f03000001           mov              byte ptr [rbp + 0x38f], 1
    0x140faf3ca    488d059f23c001           lea              rax, [rip + 0x1c0239f] -> 0x142bb1770
    0x140faf3d1    4889442420               mov              qword ptr [rsp + 0x20], rax
    0x140faf3d6    488d0d7b23c001           lea              rcx, [rip + 0x1c0237b] -> 0x142bb1758
    0x140faf3dd    4c8d0d6420c001           lea              r9, [rip + 0x1c02064] -> 0x142bb1448
    0x140faf3e4    4c8d8570020000           lea              r8, [rbp + 0x270]
    0x140faf3eb    ba11000000               mov              edx, 0x11
    0x140faf3f0    e86ba39201               call             0x1428d9760 -> core::result::unwrap_failed::h96d74ae09566f4b3
    0x140faf3f5    e9a0000000               jmp              0x140faf49a
    0x140faf3fa    0f1085f0000000           movups           xmm0, xmmword ptr [rbp + 0xf0]
    0x140faf401    0f108d00010000           movups           xmm1, xmmword ptr [rbp + 0x100]
    0x140faf408    0f298d80020000           movaps           xmmword ptr [rbp + 0x280], xmm1
    0x140faf40f    0f298570020000           movaps           xmmword ptr [rbp + 0x270], xmm0
    0x140faf416    488d05eb22c001           lea              rax, [rip + 0x1c022eb] -> 0x142bb1708
    0x140faf41d    4889442420               mov              qword ptr [rsp + 0x20], rax
    0x140faf422    488d0d5f20c001           lea              rcx, [rip + 0x1c0205f] -> 0x142bb1488
    0x140faf429    4c8d0d3820c001           lea              r9, [rip + 0x1c02038] -> 0x142bb1468
    0x140faf430    4c8d8570020000           lea              r8, [rbp + 0x270]
    0x140faf437    ba2b000000               mov              edx, 0x2b
    0x140faf43c    e81fa39201               call             0x1428d9760 -> core::result::unwrap_failed::h96d74ae09566f4b3
    0x140faf441    eb57                     jmp              0x140faf49a
    0x140faf443    4c89b578030000           mov              qword ptr [rbp + 0x378], r14
    0x140faf44a    4889bd80030000           mov              qword ptr [rbp + 0x380], rdi
    0x140faf451    8985f0000000             mov              dword ptr [rbp + 0xf0], eax
    0x140faf457    488d05e222c001           lea              rax, [rip + 0x1c022e2] -> 0x142bb1740
    0x140faf45e    4889442420               mov              qword ptr [rsp + 0x20], rax
    0x140faf463    488d0db622c001           lea              rcx, [rip + 0x1c022b6] -> 0x142bb1720
    0x140faf46a    4c8d0dd71fc001           lea              r9, [rip + 0x1c01fd7] -> 0x142bb1448
    0x140faf471    4c8d85f0000000           lea              r8, [rbp + 0xf0]
    0x140faf478    ba1b000000               mov              edx, 0x1b
    0x140faf47d    e8dea29201               call             0x1428d9760 -> core::result::unwrap_failed::h96d74ae09566f4b3
    0x140faf482    eb16                     jmp              0x140faf49a
    0x140faf484    4c8d057520c001           lea              r8, [rip + 0x1c02075] -> 0x142bb1500
    0x140faf48b    b901000000               mov              ecx, 1
    0x140faf490    ba0f000000               mov              edx, 0xf
    0x140faf495    e8499b9201               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x140faf49a    0f0b                     ud2              

=== 0x14147a500 size=421  rhapsody::raw::error::impl$1::fmt
    ; Leaf/impl fully decoded from PE bytes (semantics mostly diagnostic/formatting); see insns.  [medium]
    0x14147a500    4881ec88000000           sub              rsp, 0x88
    0x14147a507    4889d0                   mov              rax, rdx
    0x14147a50a    4c8b01                   mov              r8, qword ptr [rcx]
    0x14147a50d    4d8d48f7                 lea              r9, [r8 - 9]
    0x14147a511    4983c0f8                 add              r8, -8
    0x14147a515    31d2                     xor              edx, edx
    0x14147a517    4983f902                 cmp              r9, 2
    0x14147a51b    490f42d0                 cmovb            rdx, r8
    0x14147a51f    4885d2                   test             rdx, rdx
    0x14147a522    0f8487000000             je               0x14147a5af
    0x14147a528    4883fa01                 cmp              rdx, 1
    0x14147a52c    0f8520010000             jne              0x14147a652
    0x14147a532    488d5120                 lea              rdx, [rcx + 0x20]
    0x14147a536    4883c108                 add              rcx, 8
    0x14147a53a    48898c2480000000         mov              qword ptr [rsp + 0x80], rcx
    0x14147a542    488d0dafac7e01           lea              rcx, [rip + 0x17eacaf] -> 0x142c651f8
    0x14147a549    48894c2450               mov              qword ptr [rsp + 0x50], rcx
    0x14147a54e    488d8c2480000000         lea              rcx, [rsp + 0x80]
    0x14147a556    48894c2448               mov              qword ptr [rsp + 0x48], rcx
    0x14147a55b    488d0dbfac7e01           lea              rcx, [rip + 0x17eacbf] -> 0x142c65221
    0x14147a562    48894c2438               mov              qword ptr [rsp + 0x38], rcx
    0x14147a567    488d0d02a97e01           lea              rcx, [rip + 0x17ea902] -> 0x142c64e70
    0x14147a56e    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x14147a573    4889542428               mov              qword ptr [rsp + 0x28], rdx
    0x14147a578    48c744244005000000       mov              qword ptr [rsp + 0x40], 5
    0x14147a581    48c744242003000000       mov              qword ptr [rsp + 0x20], 3
    0x14147a58a    488d1587ac7e01           lea              rdx, [rip + 0x17eac87] -> 0x142c65218
    0x14147a591    4c8d0d86ac7e01           lea              r9, [rip + 0x17eac86] -> 0x142c6521e
    0x14147a598    41b806000000             mov              r8d, 6
    0x14147a59e    4889c1                   mov              rcx, rax
    0x14147a5a1    e8fac690ff               call             0x140d86ca0 -> core::fmt::Formatter::debug_struct_field2_finish::h35c7e6c960cc525a
    0x14147a5a6    90                       nop              
    0x14147a5a7    4881c488000000           add              rsp, 0x88
    0x14147a5ae    c3                       ret              
    0x14147a5af    488d5120                 lea              rdx, [rcx + 0x20]
    0x14147a5b3    4c8d4138                 lea              r8, [rcx + 0x38]
    0x14147a5b7    4c89842480000000         mov              qword ptr [rsp + 0x80], r8
    0x14147a5bf    4c8d05f2ab7e01           lea              r8, [rip + 0x17eabf2] -> 0x142c651b8
    0x14147a5c6    4c89442470               mov              qword ptr [rsp + 0x70], r8
    0x14147a5cb    4c8d842480000000         lea              r8, [rsp + 0x80]
    0x14147a5d3    4c89442468               mov              qword ptr [rsp + 0x68], r8
    0x14147a5d8    4c8d050cac7e01           lea              r8, [rip + 0x17eac0c] -> 0x142c651eb
    0x14147a5df    4c89442458               mov              qword ptr [rsp + 0x58], r8
    0x14147a5e4    4c8d05adab7e01           lea              r8, [rip + 0x17eabad] -> 0x142c65198
    0x14147a5eb    4c89442450               mov              qword ptr [rsp + 0x50], r8
    0x14147a5f0    4889542448               mov              qword ptr [rsp + 0x48], rdx
    0x14147a5f5    488d15ebab7e01           lea              rdx, [rip + 0x17eabeb] -> 0x142c651e7
    0x14147a5fc    4889542438               mov              qword ptr [rsp + 0x38], rdx
    0x14147a601    488d1570ab7e01           lea              rdx, [rip + 0x17eab70] -> 0x142c65178
    0x14147a608    4889542430               mov              qword ptr [rsp + 0x30], rdx
    0x14147a60d    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x14147a612    48c74424600c000000       mov              qword ptr [rsp + 0x60], 0xc
    0x14147a61b    48c744244004000000       mov              qword ptr [rsp + 0x40], 4
    0x14147a624    48c744242005000000       mov              qword ptr [rsp + 0x20], 5
    0x14147a62d    488d15a4ab7e01           lea              rdx, [rip + 0x17eaba4] -> 0x142c651d8
    0x14147a634    4c8d0da7ab7e01           lea              r9, [rip + 0x17eaba7] -> 0x142c651e2
    0x14147a63b    41b80a000000             mov              r8d, 0xa
    0x14147a641    4889c1                   mov              rcx, rax
    0x14147a644    e847c790ff               call             0x140d86d90 -> core::fmt::Formatter::debug_struct_field3_finish::h024add6f4e520141
    0x14147a649    90                       nop              
    0x14147a64a    4881c488000000           add              rsp, 0x88
    0x14147a651    c3                       ret              
    0x14147a652    4883c108                 add              rcx, 8
    0x14147a656    48898c2480000000         mov              qword ptr [rsp + 0x80], rcx
    0x14147a65e    488d0dc3ab7e01           lea              rcx, [rip + 0x17eabc3] -> 0x142c65228
    0x14147a665    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x14147a66a    488d8c2480000000         lea              rcx, [rsp + 0x80]
    0x14147a672    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x14147a677    48c744242004000000       mov              qword ptr [rsp + 0x20], 4
    0x14147a680    488d15c1ab7e01           lea              rdx, [rip + 0x17eabc1] -> 0x142c65248
    0x14147a687    4c8d0dc8ab7e01           lea              r9, [rip + 0x17eabc8] -> 0x142c65256
    0x14147a68e    41b80e000000             mov              r8d, 0xe
    0x14147a694    4889c1                   mov              rcx, rax
    0x14147a697    e854c590ff               call             0x140d86bf0 -> core::fmt::Formatter::debug_struct_field1_finish::hd9497ec2d71b9977
    0x14147a69c    90                       nop              
    0x14147a69d    4881c488000000           add              rsp, 0x88
    0x14147a6a4    c3                       ret              

=== 0x1415a6170 size=280  rhapsody::raw::texture::impl$28::fmt
    ; Leaf/impl fully decoded from PE bytes (semantics mostly diagnostic/formatting); see insns.  [medium]
    0x1415a6170    56                       push             rsi
    0x1415a6171    57                       push             rdi
    0x1415a6172    4881ecc8000000           sub              rsp, 0xc8
    0x1415a6179    4889d0                   mov              rax, rdx
    0x1415a617c    488d511e                 lea              rdx, [rcx + 0x1e]
    0x1415a6180    4c8d4104                 lea              r8, [rcx + 4]
    0x1415a6184    4c8d4908                 lea              r9, [rcx + 8]
    0x1415a6188    4c8d510c                 lea              r10, [rcx + 0xc]
    0x1415a618c    4c8d591c                 lea              r11, [rcx + 0x1c]
    0x1415a6190    488d7118                 lea              rsi, [rcx + 0x18]
    0x1415a6194    488d791d                 lea              rdi, [rcx + 0x1d]
    0x1415a6198    48897c2440               mov              qword ptr [rsp + 0x40], rdi
    0x1415a619d    4889542448               mov              qword ptr [rsp + 0x48], rdx
    0x1415a61a2    488d15c7356d01           lea              rdx, [rip + 0x16d35c7] -> 0x142c79770
    0x1415a61a9    4889542450               mov              qword ptr [rsp + 0x50], rdx
    0x1415a61ae    48894c2458               mov              qword ptr [rsp + 0x58], rcx
    0x1415a61b3    488d0dd6356d01           lea              rcx, [rip + 0x16d35d6] -> 0x142c79790
    0x1415a61ba    48894c2460               mov              qword ptr [rsp + 0x60], rcx
    0x1415a61bf    4c89442468               mov              qword ptr [rsp + 0x68], r8
    0x1415a61c4    488d0de5356d01           lea              rcx, [rip + 0x16d35e5] -> 0x142c797b0
    0x1415a61cb    48894c2470               mov              qword ptr [rsp + 0x70], rcx
    0x1415a61d0    4c894c2478               mov              qword ptr [rsp + 0x78], r9
    0x1415a61d5    488d0df4356d01           lea              rcx, [rip + 0x16d35f4] -> 0x142c797d0
    0x1415a61dc    48898c2480000000         mov              qword ptr [rsp + 0x80], rcx
    0x1415a61e4    4c89942488000000         mov              qword ptr [rsp + 0x88], r10
    0x1415a61ec    488d0dfd356d01           lea              rcx, [rip + 0x16d35fd] -> 0x142c797f0
    0x1415a61f3    48898c2490000000         mov              qword ptr [rsp + 0x90], rcx
    0x1415a61fb    4c899c2498000000         mov              qword ptr [rsp + 0x98], r11
    0x1415a6203    488d0d06366d01           lea              rcx, [rip + 0x16d3606] -> 0x142c79810
    0x1415a620a    48898c24a0000000         mov              qword ptr [rsp + 0xa0], rcx
    0x1415a6212    4889b424a8000000         mov              qword ptr [rsp + 0xa8], rsi
    0x1415a621a    488d0d0f366d01           lea              rcx, [rip + 0x16d360f] -> 0x142c79830
    0x1415a6221    48898c24b0000000         mov              qword ptr [rsp + 0xb0], rcx
    0x1415a6229    488d4c2440               lea              rcx, [rsp + 0x40]
    0x1415a622e    48898c24b8000000         mov              qword ptr [rsp + 0xb8], rcx
    0x1415a6236    488d0d13366d01           lea              rcx, [rip + 0x16d3613] -> 0x142c79850
    0x1415a623d    48898c24c0000000         mov              qword ptr [rsp + 0xc0], rcx
    0x1415a6245    488d4c2448               lea              rcx, [rsp + 0x48]
    0x1415a624a    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x1415a624f    48c744243008000000       mov              qword ptr [rsp + 0x30], 8
    0x1415a6258    48c744242008000000       mov              qword ptr [rsp + 0x20], 8
    0x1415a6261    488d15c0366d01           lea              rdx, [rip + 0x16d36c0] -> 0x142c79928
    0x1415a6268    4c8d0d39366d01           lea              r9, [rip + 0x16d3639] -> 0x142c798a8
    0x1415a626f    41b80b000000             mov              r8d, 0xb
    0x1415a6275    4889c1                   mov              rcx, rax
    0x1415a6278    e8e30e7eff               call             0x140d87160 -> core::fmt::Formatter::debug_struct_fields_finish::haa31ff405975fe91
    0x1415a627d    90                       nop              
    0x1415a627e    4881c4c8000000           add              rsp, 0xc8
    0x1415a6285    5f                       pop              rdi
    0x1415a6286    5e                       pop              rsi
    0x1415a6287    c3                       ret              

=== 0x1415a6290 size=39  rhapsody::raw::texture::impl$14::fmt
    ; Leaf/impl fully decoded from PE bytes (semantics mostly diagnostic/formatting); see insns.  [medium]
    0x1415a6290    4889d0                   mov              rax, rdx
    0x1415a6293    0fb609                   movzx            ecx, byte ptr [rcx]
    0x1415a6296    488d152b386d01           lea              rdx, [rip + 0x16d382b] -> 0x142c79ac8
    0x1415a629d    4c8b04ca                 mov              r8, qword ptr [rdx + rcx*8]
    0x1415a62a1    4c8d0d58386d01           lea              r9, [rip + 0x16d3858] -> 0x142c79b00
    0x1415a62a8    49631489                 movsxd           rdx, dword ptr [r9 + rcx*4]
    0x1415a62ac    4c01ca                   add              rdx, r9
    0x1415a62af    4889c1                   mov              rcx, rax
    0x1415a62b2    e9d9087eff               jmp              0x140d86b90 -> _$LT$core..fmt..Formatter$u20$as$u20$core..fmt..Write$GT$::write_str::h5c3f9ed06cfb23bd

=== 0x14160f190 size=421  rhapsody::raw::error::impl$1::fmt
    ; Leaf/impl fully decoded from PE bytes (semantics mostly diagnostic/formatting); see insns.  [medium]
    0x14160f190    4881ec88000000           sub              rsp, 0x88
    0x14160f197    4889d0                   mov              rax, rdx
    0x14160f19a    4c8b01                   mov              r8, qword ptr [rcx]
    0x14160f19d    4d8d48f7                 lea              r9, [r8 - 9]
    0x14160f1a1    4983c0f8                 add              r8, -8
    0x14160f1a5    31d2                     xor              edx, edx
    0x14160f1a7    4983f902                 cmp              r9, 2
    0x14160f1ab    490f42d0                 cmovb            rdx, r8
    0x14160f1af    4885d2                   test             rdx, rdx
    0x14160f1b2    0f8487000000             je               0x14160f23f
    0x14160f1b8    4883fa01                 cmp              rdx, 1
    0x14160f1bc    0f8520010000             jne              0x14160f2e2
    0x14160f1c2    488d5120                 lea              rdx, [rcx + 0x20]
    0x14160f1c6    4883c108                 add              rcx, 8
    0x14160f1ca    48898c2480000000         mov              qword ptr [rsp + 0x80], rcx
    0x14160f1d2    488d0ddff96601           lea              rcx, [rip + 0x166f9df] -> 0x142c7ebb8
    0x14160f1d9    48894c2450               mov              qword ptr [rsp + 0x50], rcx
    0x14160f1de    488d8c2480000000         lea              rcx, [rsp + 0x80]
    0x14160f1e6    48894c2448               mov              qword ptr [rsp + 0x48], rcx
    0x14160f1eb    488d0deff96601           lea              rcx, [rip + 0x166f9ef] -> 0x142c7ebe1
    0x14160f1f2    48894c2438               mov              qword ptr [rsp + 0x38], rcx
    0x14160f1f7    488d0d9af96601           lea              rcx, [rip + 0x166f99a] -> 0x142c7eb98
    0x14160f1fe    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x14160f203    4889542428               mov              qword ptr [rsp + 0x28], rdx
    0x14160f208    48c744244005000000       mov              qword ptr [rsp + 0x40], 5
    0x14160f211    48c744242003000000       mov              qword ptr [rsp + 0x20], 3
    0x14160f21a    488d15b7f96601           lea              rdx, [rip + 0x166f9b7] -> 0x142c7ebd8
    0x14160f221    4c8d0db6f96601           lea              r9, [rip + 0x166f9b6] -> 0x142c7ebde
    0x14160f228    41b806000000             mov              r8d, 6
    0x14160f22e    4889c1                   mov              rcx, rax
    0x14160f231    e86a7a77ff               call             0x140d86ca0 -> core::fmt::Formatter::debug_struct_field2_finish::h35c7e6c960cc525a
    0x14160f236    90                       nop              
    0x14160f237    4881c488000000           add              rsp, 0x88
    0x14160f23e    c3                       ret              
    0x14160f23f    488d5120                 lea              rdx, [rcx + 0x20]
    0x14160f243    4c8d4138                 lea              r8, [rcx + 0x38]
    0x14160f247    4c89842480000000         mov              qword ptr [rsp + 0x80], r8
    0x14160f24f    4c8d0502f96601           lea              r8, [rip + 0x166f902] -> 0x142c7eb58
    0x14160f256    4c89442470               mov              qword ptr [rsp + 0x70], r8
    0x14160f25b    4c8d842480000000         lea              r8, [rsp + 0x80]
    0x14160f263    4c89442468               mov              qword ptr [rsp + 0x68], r8
    0x14160f268    4c8d051cf96601           lea              r8, [rip + 0x166f91c] -> 0x142c7eb8b
    0x14160f26f    4c89442458               mov              qword ptr [rsp + 0x58], r8
    0x14160f274    4c8d05bdf86601           lea              r8, [rip + 0x166f8bd] -> 0x142c7eb38
    0x14160f27b    4c89442450               mov              qword ptr [rsp + 0x50], r8
    0x14160f280    4889542448               mov              qword ptr [rsp + 0x48], rdx
    0x14160f285    488d15fbf86601           lea              rdx, [rip + 0x166f8fb] -> 0x142c7eb87
    0x14160f28c    4889542438               mov              qword ptr [rsp + 0x38], rdx
    0x14160f291    488d1580f86601           lea              rdx, [rip + 0x166f880] -> 0x142c7eb18
    0x14160f298    4889542430               mov              qword ptr [rsp + 0x30], rdx
    0x14160f29d    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x14160f2a2    48c74424600c000000       mov              qword ptr [rsp + 0x60], 0xc
    0x14160f2ab    48c744244004000000       mov              qword ptr [rsp + 0x40], 4
    0x14160f2b4    48c744242005000000       mov              qword ptr [rsp + 0x20], 5
    0x14160f2bd    488d15b4f86601           lea              rdx, [rip + 0x166f8b4] -> 0x142c7eb78
    0x14160f2c4    4c8d0db7f86601           lea              r9, [rip + 0x166f8b7] -> 0x142c7eb82
    0x14160f2cb    41b80a000000             mov              r8d, 0xa
    0x14160f2d1    4889c1                   mov              rcx, rax
    0x14160f2d4    e8b77a77ff               call             0x140d86d90 -> core::fmt::Formatter::debug_struct_field3_finish::h024add6f4e520141
    0x14160f2d9    90                       nop              
    0x14160f2da    4881c488000000           add              rsp, 0x88
    0x14160f2e1    c3                       ret              
    0x14160f2e2    4883c108                 add              rcx, 8
    0x14160f2e6    48898c2480000000         mov              qword ptr [rsp + 0x80], rcx
    0x14160f2ee    488d0df3f86601           lea              rcx, [rip + 0x166f8f3] -> 0x142c7ebe8
    0x14160f2f5    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x14160f2fa    488d8c2480000000         lea              rcx, [rsp + 0x80]
    0x14160f302    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x14160f307    48c744242004000000       mov              qword ptr [rsp + 0x20], 4
    0x14160f310    488d15f1f86601           lea              rdx, [rip + 0x166f8f1] -> 0x142c7ec08
    0x14160f317    4c8d0df8f86601           lea              r9, [rip + 0x166f8f8] -> 0x142c7ec16
    0x14160f31e    41b80e000000             mov              r8d, 0xe
    0x14160f324    4889c1                   mov              rcx, rax
    0x14160f327    e8c47877ff               call             0x140d86bf0 -> core::fmt::Formatter::debug_struct_field1_finish::hd9497ec2d71b9977
    0x14160f32c    90                       nop              
    0x14160f32d    4881c488000000           add              rsp, 0x88
    0x14160f334    c3                       ret              

=== 0x141697480 size=421  rhapsody::raw::error::impl$1::fmt
    ; Leaf/impl fully decoded from PE bytes (semantics mostly diagnostic/formatting); see insns.  [medium]
    0x141697480    4881ec88000000           sub              rsp, 0x88
    0x141697487    4889d0                   mov              rax, rdx
    0x14169748a    4c8b01                   mov              r8, qword ptr [rcx]
    0x14169748d    4d8d48f7                 lea              r9, [r8 - 9]
    0x141697491    4983c0f8                 add              r8, -8
    0x141697495    31d2                     xor              edx, edx
    0x141697497    4983f902                 cmp              r9, 2
    0x14169749b    490f42d0                 cmovb            rdx, r8
    0x14169749f    4885d2                   test             rdx, rdx
    0x1416974a2    0f8487000000             je               0x14169752f
    0x1416974a8    4883fa01                 cmp              rdx, 1
    0x1416974ac    0f8520010000             jne              0x1416975d2
    0x1416974b2    488d5120                 lea              rdx, [rcx + 0x20]
    0x1416974b6    4883c108                 add              rcx, 8
    0x1416974ba    48898c2480000000         mov              qword ptr [rsp + 0x80], rcx
    0x1416974c2    488d0ddfff5e01           lea              rcx, [rip + 0x15effdf] -> 0x142c874a8
    0x1416974c9    48894c2450               mov              qword ptr [rsp + 0x50], rcx
    0x1416974ce    488d8c2480000000         lea              rcx, [rsp + 0x80]
    0x1416974d6    48894c2448               mov              qword ptr [rsp + 0x48], rcx
    0x1416974db    488d0defff5e01           lea              rcx, [rip + 0x15effef] -> 0x142c874d1
    0x1416974e2    48894c2438               mov              qword ptr [rsp + 0x38], rcx
    0x1416974e7    488d0d9aff5e01           lea              rcx, [rip + 0x15eff9a] -> 0x142c87488
    0x1416974ee    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x1416974f3    4889542428               mov              qword ptr [rsp + 0x28], rdx
    0x1416974f8    48c744244005000000       mov              qword ptr [rsp + 0x40], 5
    0x141697501    48c744242003000000       mov              qword ptr [rsp + 0x20], 3
    0x14169750a    488d15b7ff5e01           lea              rdx, [rip + 0x15effb7] -> 0x142c874c8
    0x141697511    4c8d0db6ff5e01           lea              r9, [rip + 0x15effb6] -> 0x142c874ce
    0x141697518    41b806000000             mov              r8d, 6
    0x14169751e    4889c1                   mov              rcx, rax
    0x141697521    e87af76eff               call             0x140d86ca0 -> core::fmt::Formatter::debug_struct_field2_finish::h35c7e6c960cc525a
    0x141697526    90                       nop              
    0x141697527    4881c488000000           add              rsp, 0x88
    0x14169752e    c3                       ret              
    0x14169752f    488d5120                 lea              rdx, [rcx + 0x20]
    0x141697533    4c8d4138                 lea              r8, [rcx + 0x38]
    0x141697537    4c89842480000000         mov              qword ptr [rsp + 0x80], r8
    0x14169753f    4c8d0502ff5e01           lea              r8, [rip + 0x15eff02] -> 0x142c87448
    0x141697546    4c89442470               mov              qword ptr [rsp + 0x70], r8
    0x14169754b    4c8d842480000000         lea              r8, [rsp + 0x80]
    0x141697553    4c89442468               mov              qword ptr [rsp + 0x68], r8
    0x141697558    4c8d051cff5e01           lea              r8, [rip + 0x15eff1c] -> 0x142c8747b
    0x14169755f    4c89442458               mov              qword ptr [rsp + 0x58], r8
    0x141697564    4c8d05bdfe5e01           lea              r8, [rip + 0x15efebd] -> 0x142c87428
    0x14169756b    4c89442450               mov              qword ptr [rsp + 0x50], r8
    0x141697570    4889542448               mov              qword ptr [rsp + 0x48], rdx
    0x141697575    488d15fbfe5e01           lea              rdx, [rip + 0x15efefb] -> 0x142c87477
    0x14169757c    4889542438               mov              qword ptr [rsp + 0x38], rdx
    0x141697581    488d1580fe5e01           lea              rdx, [rip + 0x15efe80] -> 0x142c87408
    0x141697588    4889542430               mov              qword ptr [rsp + 0x30], rdx
    0x14169758d    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x141697592    48c74424600c000000       mov              qword ptr [rsp + 0x60], 0xc
    0x14169759b    48c744244004000000       mov              qword ptr [rsp + 0x40], 4
    0x1416975a4    48c744242005000000       mov              qword ptr [rsp + 0x20], 5
    0x1416975ad    488d15b4fe5e01           lea              rdx, [rip + 0x15efeb4] -> 0x142c87468
    0x1416975b4    4c8d0db7fe5e01           lea              r9, [rip + 0x15efeb7] -> 0x142c87472
    0x1416975bb    41b80a000000             mov              r8d, 0xa
    0x1416975c1    4889c1                   mov              rcx, rax
    0x1416975c4    e8c7f76eff               call             0x140d86d90 -> core::fmt::Formatter::debug_struct_field3_finish::h024add6f4e520141
    0x1416975c9    90                       nop              
    0x1416975ca    4881c488000000           add              rsp, 0x88
    0x1416975d1    c3                       ret              
    0x1416975d2    4883c108                 add              rcx, 8
    0x1416975d6    48898c2480000000         mov              qword ptr [rsp + 0x80], rcx
    0x1416975de    488d0df3fb5e01           lea              rcx, [rip + 0x15efbf3] -> 0x142c871d8
    0x1416975e5    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x1416975ea    488d8c2480000000         lea              rcx, [rsp + 0x80]
    0x1416975f2    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x1416975f7    48c744242004000000       mov              qword ptr [rsp + 0x20], 4
    0x141697600    488d15cffe5e01           lea              rdx, [rip + 0x15efecf] -> 0x142c874d6
    0x141697607    4c8d0dd6fe5e01           lea              r9, [rip + 0x15efed6] -> 0x142c874e4
    0x14169760e    41b80e000000             mov              r8d, 0xe
    0x141697614    4889c1                   mov              rcx, rax
    0x141697617    e8d4f56eff               call             0x140d86bf0 -> core::fmt::Formatter::debug_struct_field1_finish::hd9497ec2d71b9977
    0x14169761c    90                       nop              
    0x14169761d    4881c488000000           add              rsp, 0x88
    0x141697624    c3                       ret              

=== 0x1416a2960 size=1111  rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_buffer::hd458ac040b33829a
    ; Large function body fully decoded (not a micro-leaf); decode present in insns.  [medium]
    0x1416a2960    55                       push             rbp
    0x1416a2961    4157                     push             r15
    0x1416a2963    4156                     push             r14
    0x1416a2965    4155                     push             r13
    0x1416a2967    4154                     push             r12
    0x1416a2969    56                       push             rsi
    0x1416a296a    57                       push             rdi
    0x1416a296b    53                       push             rbx
    0x1416a296c    4881ec68010000           sub              rsp, 0x168
    0x1416a2973    488dac2480000000         lea              rbp, [rsp + 0x80]
    0x1416a297b    48c785e0000000feffffff   mov              qword ptr [rbp + 0xe0], 0xfffffffffffffffe
    0x1416a2986    4d89cf                   mov              r15, r9
    0x1416a2989    4d89c6                   mov              r14, r8
    0x1416a298c    4889d7                   mov              rdi, rdx
    0x1416a298f    4889ce                   mov              rsi, rcx
    0x1416a2992    4c8bad50010000           mov              r13, qword ptr [rbp + 0x150]
    0x1416a2999    0f108558010000           movups           xmm0, xmmword ptr [rbp + 0x158]
    0x1416a29a0    0f1185d0000000           movups           xmmword ptr [rbp + 0xd0], xmm0
    0x1416a29a7    488d8dd0000000           lea              rcx, [rbp + 0xd0]
    0x1416a29ae    e8fd3efefe               call             0x1406868b0 -> rhapsody::raw::buffer::impl$3::byte_count
    0x1416a29b3    4885c0                   test             rax, rax
    0x1416a29b6    7405                     je               0x1416a29bd
    0x1416a29b8    41804d0802               or               byte ptr [r13 + 8], 2
    0x1416a29bd    488d5f78                 lea              rbx, [rdi + 0x78]
    0x1416a29c1    4889f9                   mov              rcx, rdi
    0x1416a29c4    e8975f55ff               call             0x140bf8960 -> rhapsody::raw::device::GraphicsDevice::debug_utils
    0x1416a29c9    4c8b8f48060000           mov              r9, qword ptr [rdi + 0x648]
    0x1416a29d0    488b8f78060000           mov              rcx, qword ptr [rdi + 0x678]
    0x1416a29d7    4883c110                 add              rcx, 0x10
    0x1416a29db    410f104500               movups           xmm0, xmmword ptr [r13]
    0x1416a29e0    0f294520                 movaps           xmmword ptr [rbp + 0x20], xmm0
    0x1416a29e4    4c897c2438               mov              qword ptr [rsp + 0x38], r15
    0x1416a29e9    4c89742430               mov              qword ptr [rsp + 0x30], r14
    0x1416a29ee    4c8d6520                 lea              r12, [rbp + 0x20]
    0x1416a29f2    4c89642428               mov              qword ptr [rsp + 0x28], r12
    0x1416a29f7    48894c2420               mov              qword ptr [rsp + 0x20], rcx
    0x1416a29fc    488d4d50                 lea              rcx, [rbp + 0x50]
    0x1416a2a00    4889da                   mov              rdx, rbx
    0x1416a2a03    4989c0                   mov              r8, rax
    0x1416a2a06    e8f5b754ff               call             0x140bee200 -> rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_buffer_impl::h34de0f745611d2ab
    0x1416a2a0b    488b4550                 mov              rax, qword ptr [rbp + 0x50]
    0x1416a2a0f    0f104558                 movups           xmm0, xmmword ptr [rbp + 0x58]
    0x1416a2a13    0f2985a0000000           movaps           xmmword ptr [rbp + 0xa0], xmm0
    0x1416a2a1a    0f104568                 movups           xmm0, xmmword ptr [rbp + 0x68]
    0x1416a2a1e    0f2985b0000000           movaps           xmmword ptr [rbp + 0xb0], xmm0
    0x1416a2a25    0f104578                 movups           xmm0, xmmword ptr [rbp + 0x78]
    0x1416a2a29    0f2985c0000000           movaps           xmmword ptr [rbp + 0xc0], xmm0
    0x1416a2a30    4c8bb588000000           mov              r14, qword ptr [rbp + 0x88]
    0x1416a2a37    4c8bbd90000000           mov              r15, qword ptr [rbp + 0x90]
    0x1416a2a3e    4883f80b                 cmp              rax, 0xb
    0x1416a2a42    0f859c000000             jne              0x1416a2ae4
    0x1416a2a48    0f2885a0000000           movaps           xmm0, xmmword ptr [rbp + 0xa0]
    0x1416a2a4f    0f288db0000000           movaps           xmm1, xmmword ptr [rbp + 0xb0]
    0x1416a2a56    0f2895c0000000           movaps           xmm2, xmmword ptr [rbp + 0xc0]
    0x1416a2a5d    0f295540                 movaps           xmmword ptr [rbp + 0x40], xmm2
    0x1416a2a61    0f294d30                 movaps           xmmword ptr [rbp + 0x30], xmm1
    0x1416a2a65    0f294520                 movaps           xmmword ptr [rbp + 0x20], xmm0
    0x1416a2a69    488d8dd0000000           lea              rcx, [rbp + 0xd0]
    0x1416a2a70    e83b3efefe               call             0x1406868b0 -> rhapsody::raw::buffer::impl$3::byte_count
    0x1416a2a75    4885c0                   test             rax, rax
    0x1416a2a78    7449                     je               0x1416a2ac3
    0x1416a2a7a    488d8dd0000000           lea              rcx, [rbp + 0xd0]
    0x1416a2a81    e82a3efefe               call             0x1406868b0 -> rhapsody::raw::buffer::impl$3::byte_count
    0x1416a2a86    498b5500                 mov              rdx, qword ptr [r13]
    0x1416a2a8a    4839d0                   cmp              rax, rdx
    0x1416a2a8d    0f874f020000             ja               0x1416a2ce2
    0x1416a2a93    4d85f6                   test             r14, r14
    0x1416a2a96    0f8496000000             je               0x1416a2b32
    0x1416a2a9c    488d8dd0000000           lea              rcx, [rbp + 0xd0]
    0x1416a2aa3    e8083efefe               call             0x1406868b0 -> rhapsody::raw::buffer::impl$3::byte_count
    0x1416a2aa8    4c39f8                   cmp              rax, r15
    0x1416a2aab    0f874b020000             ja               0x1416a2cfc
    0x1416a2ab1    488d8dd0000000           lea              rcx, [rbp + 0xd0]
    0x1416a2ab8    4c89f2                   mov              rdx, r14
    0x1416a2abb    4989c0                   mov              r8, rax
    0x1416a2abe    e8bd2453ff               call             0x140bd4f80 -> rhapsody::raw::buffer::impl$3::copy_to_slice
    0x1416a2ac3    488b4548                 mov              rax, qword ptr [rbp + 0x48]
    0x1416a2ac7    48894628                 mov              qword ptr [rsi + 0x28], rax
    0x1416a2acb    0f104528                 movups           xmm0, xmmword ptr [rbp + 0x28]
    0x1416a2acf    0f104d38                 movups           xmm1, xmmword ptr [rbp + 0x38]
    0x1416a2ad3    0f114e18                 movups           xmmword ptr [rsi + 0x18], xmm1
    0x1416a2ad7    0f114608                 movups           xmmword ptr [rsi + 8], xmm0
    0x1416a2adb    48c7060b000000           mov              qword ptr [rsi], 0xb
    0x1416a2ae2    eb37                     jmp              0x1416a2b1b
    0x1416a2ae4    488b8d98000000           mov              rcx, qword ptr [rbp + 0x98]
    0x1416a2aeb    0f2885a0000000           movaps           xmm0, xmmword ptr [rbp + 0xa0]
    0x1416a2af2    0f288db0000000           movaps           xmm1, xmmword ptr [rbp + 0xb0]
    0x1416a2af9    0f2895c0000000           movaps           xmm2, xmmword ptr [rbp + 0xc0]
    0x1416a2b00    0f115628                 movups           xmmword ptr [rsi + 0x28], xmm2
    0x1416a2b04    0f114e18                 movups           xmmword ptr [rsi + 0x18], xmm1
    0x1416a2b08    0f114608                 movups           xmmword ptr [rsi + 8], xmm0
    0x1416a2b0c    488906                   mov              qword ptr [rsi], rax
    0x1416a2b0f    4c897638                 mov              qword ptr [rsi + 0x38], r14
    0x1416a2b13    4c897e40                 mov              qword ptr [rsi + 0x40], r15
    0x1416a2b17    48894e48                 mov              qword ptr [rsi + 0x48], rcx
    0x1416a2b1b    4889f0                   mov              rax, rsi
    0x1416a2b1e    4881c468010000           add              rsp, 0x168
    0x1416a2b25    5b                       pop              rbx
    0x1416a2b26    5f                       pop              rdi
    0x1416a2b27    5e                       pop              rsi
    0x1416a2b28    415c                     pop              r12
    0x1416a2b2a    415d                     pop              r13
    0x1416a2b2c    415e                     pop              r14
    0x1416a2b2e    415f                     pop              r15
    0x1416a2b30    5d                       pop              rbp
    0x1416a2b31    c3                       ret              
    0x1416a2b32    4c8d75c0                 lea              r14, [rbp - 0x40]
    0x1416a2b36    4c89f1                   mov              rcx, r14
    0x1416a2b39    41b801000000             mov              r8d, 1
    0x1416a2b3f    e85c2353ff               call             0x140bd4ea0 -> rhapsody::raw::buffer::BufferDesc::new_cpu_to_gpu_low_prio
    0x1416a2b44    4889f9                   mov              rcx, rdi
    0x1416a2b47    e8145e55ff               call             0x140bf8960 -> rhapsody::raw::device::GraphicsDevice::debug_utils
    0x1416a2b4c    4c8b8f48060000           mov              r9, qword ptr [rdi + 0x648]
    0x1416a2b53    488b8f78060000           mov              rcx, qword ptr [rdi + 0x678]
    0x1416a2b5a    4883c110                 add              rcx, 0x10
    0x1416a2b5e    488d15bb5d5e01           lea              rdx, [rip + 0x15e5dbb] -> 0x142c88920
    0x1416a2b65    4889542430               mov              qword ptr [rsp + 0x30], rdx
    0x1416a2b6a    4c89742428               mov              qword ptr [rsp + 0x28], r14
    0x1416a2b6f    48894c2420               mov              qword ptr [rsp + 0x20], rcx
    0x1416a2b74    48c744243813000000       mov              qword ptr [rsp + 0x38], 0x13
    0x1416a2b7d    4c8d7550                 lea              r14, [rbp + 0x50]
    0x1416a2b81    4c89f1                   mov              rcx, r14
    0x1416a2b84    4889da                   mov              rdx, rbx
    0x1416a2b87    4989c0                   mov              r8, rax
    0x1416a2b8a    e871b654ff               call             0x140bee200 -> rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_buffer_impl::h34de0f745611d2ab
    0x1416a2b8f    488b4550                 mov              rax, qword ptr [rbp + 0x50]
    0x1416a2b93    0f104558                 movups           xmm0, xmmword ptr [rbp + 0x58]
    0x1416a2b97    0f2985a0000000           movaps           xmmword ptr [rbp + 0xa0], xmm0
    0x1416a2b9e    0f104568                 movups           xmm0, xmmword ptr [rbp + 0x68]
    0x1416a2ba2    0f2985b0000000           movaps           xmmword ptr [rbp + 0xb0], xmm0
    0x1416a2ba9    0f104578                 movups           xmm0, xmmword ptr [rbp + 0x78]
    0x1416a2bad    0f2985c0000000           movaps           xmmword ptr [rbp + 0xc0], xmm0
    0x1416a2bb4    488b9d88000000           mov              rbx, qword ptr [rbp + 0x88]
    0x1416a2bbb    4c8bbd90000000           mov              r15, qword ptr [rbp + 0x90]
    0x1416a2bc2    4883f80b                 cmp              rax, 0xb
    0x1416a2bc6    0f859b000000             jne              0x1416a2c67
    0x1416a2bcc    0f2885a0000000           movaps           xmm0, xmmword ptr [rbp + 0xa0]
    0x1416a2bd3    0f288db0000000           movaps           xmm1, xmmword ptr [rbp + 0xb0]
    0x1416a2bda    0f2895c0000000           movaps           xmm2, xmmword ptr [rbp + 0xc0]
    0x1416a2be1    0f295570                 movaps           xmmword ptr [rbp + 0x70], xmm2
    0x1416a2be5    0f294d60                 movaps           xmmword ptr [rbp + 0x60], xmm1
    0x1416a2be9    0f294550                 movaps           xmmword ptr [rbp + 0x50], xmm0
    0x1416a2bed    4885db                   test             rbx, rbx
    0x1416a2bf0    0f842e010000             je               0x1416a2d24
    0x1416a2bf6    488d8dd0000000           lea              rcx, [rbp + 0xd0]
    0x1416a2bfd    e8ae3cfefe               call             0x1406868b0 -> rhapsody::raw::buffer::impl$3::byte_count
    0x1416a2c02    4c39f8                   cmp              rax, r15
    0x1416a2c05    0f8705010000             ja               0x1416a2d10
    0x1416a2c0b    4c8dbdd0000000           lea              r15, [rbp + 0xd0]
    0x1416a2c12    4c89f9                   mov              rcx, r15
    0x1416a2c15    4889da                   mov              rdx, rbx
    0x1416a2c18    4989c0                   mov              r8, rax
    0x1416a2c1b    e8602353ff               call             0x140bd4f80 -> rhapsody::raw::buffer::impl$3::copy_to_slice
    0x1416a2c20    4c89b5a0000000           mov              qword ptr [rbp + 0xa0], r14
    0x1416a2c27    4889bda8000000           mov              qword ptr [rbp + 0xa8], rdi
    0x1416a2c2e    4c89a5b0000000           mov              qword ptr [rbp + 0xb0], r12
    0x1416a2c35    4c89bdb8000000           mov              qword ptr [rbp + 0xb8], r15
    0x1416a2c3c    488d4dd0                 lea              rcx, [rbp - 0x30]
    0x1416a2c40    4c8d85a0000000           lea              r8, [rbp + 0xa0]
    0x1416a2c47    4889fa                   mov              rdx, rdi
    0x1416a2c4a    e8d1050000               call             0x1416a3220 -> rhapsody::raw::device::GraphicsDevice::with_setup_cb::h8cd7a8dc0d8d3a74
    0x1416a2c4f    488b45d0                 mov              rax, qword ptr [rbp - 0x30]
    0x1416a2c53    4883f80b                 cmp              rax, 0xb
    0x1416a2c57    7547                     jne              0x1416a2ca0
    0x1416a2c59    488d4d58                 lea              rcx, [rbp + 0x58]
    0x1416a2c5d    e84ef853ff               call             0x140be24b0 -> rhapsody::raw::managed::handle::impl$6::drop
    0x1416a2c62    e95cfeffff               jmp              0x1416a2ac3
    0x1416a2c67    488b8d98000000           mov              rcx, qword ptr [rbp + 0x98]
    0x1416a2c6e    0f2885a0000000           movaps           xmm0, xmmword ptr [rbp + 0xa0]
    0x1416a2c75    0f288db0000000           movaps           xmm1, xmmword ptr [rbp + 0xb0]
    0x1416a2c7c    0f2895c0000000           movaps           xmm2, xmmword ptr [rbp + 0xc0]
    0x1416a2c83    0f115628                 movups           xmmword ptr [rsi + 0x28], xmm2
    0x1416a2c87    0f114e18                 movups           xmmword ptr [rsi + 0x18], xmm1
    0x1416a2c8b    0f114608                 movups           xmmword ptr [rsi + 8], xmm0
    0x1416a2c8f    488906                   mov              qword ptr [rsi], rax
    0x1416a2c92    48895e38                 mov              qword ptr [rsi + 0x38], rbx
    0x1416a2c96    4c897e40                 mov              qword ptr [rsi + 0x40], r15
    0x1416a2c9a    48894e48                 mov              qword ptr [rsi + 0x48], rcx
    0x1416a2c9e    eb34                     jmp              0x1416a2cd4
    0x1416a2ca0    488b4d18                 mov              rcx, qword ptr [rbp + 0x18]
    0x1416a2ca4    48894e48                 mov              qword ptr [rsi + 0x48], rcx
    0x1416a2ca8    0f1045d8                 movups           xmm0, xmmword ptr [rbp - 0x28]
    0x1416a2cac    0f104de8                 movups           xmm1, xmmword ptr [rbp - 0x18]
    0x1416a2cb0    0f1055f8                 movups           xmm2, xmmword ptr [rbp - 8]
    0x1416a2cb4    0f105d08                 movups           xmm3, xmmword ptr [rbp + 8]
    0x1416a2cb8    0f115e38                 movups           xmmword ptr [rsi + 0x38], xmm3
    0x1416a2cbc    0f115628                 movups           xmmword ptr [rsi + 0x28], xmm2
    0x1416a2cc0    0f114e18                 movups           xmmword ptr [rsi + 0x18], xmm1
    0x1416a2cc4    0f114608                 movups           xmmword ptr [rsi + 8], xmm0
    0x1416a2cc8    488906                   mov              qword ptr [rsi], rax
    0x1416a2ccb    488d4d58                 lea              rcx, [rbp + 0x58]
    0x1416a2ccf    e8dcf753ff               call             0x140be24b0 -> rhapsody::raw::managed::handle::impl$6::drop
    0x1416a2cd4    488d4d28                 lea              rcx, [rbp + 0x28]
    0x1416a2cd8    e8d3f753ff               call             0x140be24b0 -> rhapsody::raw::managed::handle::impl$6::drop
    0x1416a2cdd    e939feffff               jmp              0x1416a2b1b
    0x1416a2ce2    488d0d775b5e01           lea              rcx, [rip + 0x15e5b77] -> 0x142c88860
    0x1416a2ce9    4c8d05005c5e01           lea              r8, [rip + 0x15e5c00] -> 0x142c888f0
    0x1416a2cf0    ba38000000               mov              edx, 0x38
    0x1416a2cf5    e836672301               call             0x1428d9430 -> core::panicking::panic::hb210ae7b817a9f13
    0x1416a2cfa    eb40                     jmp              0x1416a2d3c
    0x1416a2cfc    4c8d05055c5e01           lea              r8, [rip + 0x15e5c05] -> 0x142c88908
    0x1416a2d03    4889c1                   mov              rcx, rax
    0x1416a2d06    4c89fa                   mov              rdx, r15
    0x1416a2d09    e8326b2301               call             0x1428d9840 -> core::slice::index::slice_end_index_len_fail::hc3a65ec02a6eede1
    0x1416a2d0e    eb2c                     jmp              0x1416a2d3c
    0x1416a2d10    4c8d05495c5e01           lea              r8, [rip + 0x15e5c49] -> 0x142c88960
    0x1416a2d17    4889c1                   mov              rcx, rax
    0x1416a2d1a    4c89fa                   mov              rdx, r15
    0x1416a2d1d    e81e6b2301               call             0x1428d9840 -> core::slice::index::slice_end_index_len_fail::hc3a65ec02a6eede1
    0x1416a2d22    eb18                     jmp              0x1416a2d3c
    0x1416a2d24    488d0d085c5e01           lea              rcx, [rip + 0x15e5c08] -> 0x142c88933
    0x1416a2d2b    4c8d05165c5e01           lea              r8, [rip + 0x15e5c16] -> 0x142c88948
    0x1416a2d32    ba14000000               mov              edx, 0x14
    0x1416a2d37    e8f4652301               call             0x1428d9330 -> core::option::expect_failed::hb3e848c6b16873c8
    0x1416a2d3c    0f0b                     ud2              
    0x1416a2d3e    6690                     nop              
    0x1416a2d40    4889542410               mov              qword ptr [rsp + 0x10], rdx
    0x1416a2d45    55                       push             rbp
    0x1416a2d46    4157                     push             r15
    0x1416a2d48    4156                     push             r14
    0x1416a2d4a    4155                     push             r13
    0x1416a2d4c    4154                     push             r12
    0x1416a2d4e    56                       push             rsi
    0x1416a2d4f    57                       push             rdi
    0x1416a2d50    53                       push             rbx
    0x1416a2d51    4883ec48                 sub              rsp, 0x48
    0x1416a2d55    488daa80000000           lea              rbp, [rdx + 0x80]
    0x1416a2d5c    488d4d58                 lea              rcx, [rbp + 0x58]
    0x1416a2d60    e84bf753ff               call             0x140be24b0 -> rhapsody::raw::managed::handle::impl$6::drop
    0x1416a2d65    90                       nop              
    0x1416a2d66    4883c448                 add              rsp, 0x48
    0x1416a2d6a    5b                       pop              rbx
    0x1416a2d6b    5f                       pop              rdi
    0x1416a2d6c    5e                       pop              rsi
    0x1416a2d6d    415c                     pop              r12
    0x1416a2d6f    415d                     pop              r13
    0x1416a2d71    415e                     pop              r14
    0x1416a2d73    415f                     pop              r15
    0x1416a2d75    5d                       pop              rbp
    0x1416a2d76    c3                       ret              
    0x1416a2d77    660f1f840000000000       nop              word ptr [rax + rax]
    0x1416a2d80    4889542410               mov              qword ptr [rsp + 0x10], rdx
    0x1416a2d85    55                       push             rbp
    0x1416a2d86    4157                     push             r15
    0x1416a2d88    4156                     push             r14
    0x1416a2d8a    4155                     push             r13
    0x1416a2d8c    4154                     push             r12
    0x1416a2d8e    56                       push             rsi
    0x1416a2d8f    57                       push             rdi
    0x1416a2d90    53                       push             rbx
    0x1416a2d91    4883ec48                 sub              rsp, 0x48
    0x1416a2d95    488daa80000000           lea              rbp, [rdx + 0x80]
    0x1416a2d9c    488d4d28                 lea              rcx, [rbp + 0x28]
    0x1416a2da0    e80bf753ff               call             0x140be24b0 -> rhapsody::raw::managed::handle::impl$6::drop
    0x1416a2da5    90                       nop              
    0x1416a2da6    4883c448                 add              rsp, 0x48
    0x1416a2daa    5b                       pop              rbx
    0x1416a2dab    5f                       pop              rdi
    0x1416a2dac    5e                       pop              rsi
    0x1416a2dad    415c                     pop              r12
    0x1416a2daf    415d                     pop              r13
    0x1416a2db1    415e                     pop              r14
    0x1416a2db3    415f                     pop              r15
    0x1416a2db5    5d                       pop              rbp
    0x1416a2db6    c3                       ret              

=== 0x1416a2dc0 size=1111  rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_buffer::hdbd845ec0ee83e92
    ; Large function body fully decoded (not a micro-leaf); decode present in insns.  [medium]
    0x1416a2dc0    55                       push             rbp
    0x1416a2dc1    4157                     push             r15
    0x1416a2dc3    4156                     push             r14
    0x1416a2dc5    4155                     push             r13
    0x1416a2dc7    4154                     push             r12
    0x1416a2dc9    56                       push             rsi
    0x1416a2dca    57                       push             rdi
    0x1416a2dcb    53                       push             rbx
    0x1416a2dcc    4881ec68010000           sub              rsp, 0x168
    0x1416a2dd3    488dac2480000000         lea              rbp, [rsp + 0x80]
    0x1416a2ddb    48c785e0000000feffffff   mov              qword ptr [rbp + 0xe0], 0xfffffffffffffffe
    0x1416a2de6    4d89cf                   mov              r15, r9
    0x1416a2de9    4d89c6                   mov              r14, r8
    0x1416a2dec    4889d7                   mov              rdi, rdx
    0x1416a2def    4889ce                   mov              rsi, rcx
    0x1416a2df2    4c8bad50010000           mov              r13, qword ptr [rbp + 0x150]
    0x1416a2df9    0f108558010000           movups           xmm0, xmmword ptr [rbp + 0x158]
    0x1416a2e00    0f1185d0000000           movups           xmmword ptr [rbp + 0xd0], xmm0
    0x1416a2e07    488d8dd0000000           lea              rcx, [rbp + 0xd0]
    0x1416a2e0e    e88d2353ff               call             0x140bd51a0 -> rhapsody::raw::buffer::impl$4::byte_count
    0x1416a2e13    4885c0                   test             rax, rax
    0x1416a2e16    7405                     je               0x1416a2e1d
    0x1416a2e18    41804d0802               or               byte ptr [r13 + 8], 2
    0x1416a2e1d    488d5f78                 lea              rbx, [rdi + 0x78]
    0x1416a2e21    4889f9                   mov              rcx, rdi
    0x1416a2e24    e8375b55ff               call             0x140bf8960 -> rhapsody::raw::device::GraphicsDevice::debug_utils
    0x1416a2e29    4c8b8f48060000           mov              r9, qword ptr [rdi + 0x648]
    0x1416a2e30    488b8f78060000           mov              rcx, qword ptr [rdi + 0x678]
    0x1416a2e37    4883c110                 add              rcx, 0x10
    0x1416a2e3b    410f104500               movups           xmm0, xmmword ptr [r13]
    0x1416a2e40    0f294520                 movaps           xmmword ptr [rbp + 0x20], xmm0
    0x1416a2e44    4c897c2438               mov              qword ptr [rsp + 0x38], r15
    0x1416a2e49    4c89742430               mov              qword ptr [rsp + 0x30], r14
    0x1416a2e4e    4c8d6520                 lea              r12, [rbp + 0x20]
    0x1416a2e52    4c89642428               mov              qword ptr [rsp + 0x28], r12
    0x1416a2e57    48894c2420               mov              qword ptr [rsp + 0x20], rcx
    0x1416a2e5c    488d4d50                 lea              rcx, [rbp + 0x50]
    0x1416a2e60    4889da                   mov              rdx, rbx
    0x1416a2e63    4989c0                   mov              r8, rax
    0x1416a2e66    e895b354ff               call             0x140bee200 -> rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_buffer_impl::h34de0f745611d2ab
    0x1416a2e6b    488b4550                 mov              rax, qword ptr [rbp + 0x50]
    0x1416a2e6f    0f104558                 movups           xmm0, xmmword ptr [rbp + 0x58]
    0x1416a2e73    0f2985a0000000           movaps           xmmword ptr [rbp + 0xa0], xmm0
    0x1416a2e7a    0f104568                 movups           xmm0, xmmword ptr [rbp + 0x68]
    0x1416a2e7e    0f2985b0000000           movaps           xmmword ptr [rbp + 0xb0], xmm0
    0x1416a2e85    0f104578                 movups           xmm0, xmmword ptr [rbp + 0x78]
    0x1416a2e89    0f2985c0000000           movaps           xmmword ptr [rbp + 0xc0], xmm0
    0x1416a2e90    4c8bb588000000           mov              r14, qword ptr [rbp + 0x88]
    0x1416a2e97    4c8bbd90000000           mov              r15, qword ptr [rbp + 0x90]
    0x1416a2e9e    4883f80b                 cmp              rax, 0xb
    0x1416a2ea2    0f859c000000             jne              0x1416a2f44
    0x1416a2ea8    0f2885a0000000           movaps           xmm0, xmmword ptr [rbp + 0xa0]
    0x1416a2eaf    0f288db0000000           movaps           xmm1, xmmword ptr [rbp + 0xb0]
    0x1416a2eb6    0f2895c0000000           movaps           xmm2, xmmword ptr [rbp + 0xc0]
    0x1416a2ebd    0f295540                 movaps           xmmword ptr [rbp + 0x40], xmm2
    0x1416a2ec1    0f294d30                 movaps           xmmword ptr [rbp + 0x30], xmm1
    0x1416a2ec5    0f294520                 movaps           xmmword ptr [rbp + 0x20], xmm0
    0x1416a2ec9    488d8dd0000000           lea              rcx, [rbp + 0xd0]
    0x1416a2ed0    e8cb2253ff               call             0x140bd51a0 -> rhapsody::raw::buffer::impl$4::byte_count
    0x1416a2ed5    4885c0                   test             rax, rax
    0x1416a2ed8    7449                     je               0x1416a2f23
    0x1416a2eda    488d8dd0000000           lea              rcx, [rbp + 0xd0]
    0x1416a2ee1    e8ba2253ff               call             0x140bd51a0 -> rhapsody::raw::buffer::impl$4::byte_count
    0x1416a2ee6    498b5500                 mov              rdx, qword ptr [r13]
    0x1416a2eea    4839d0                   cmp              rax, rdx
    0x1416a2eed    0f874f020000             ja               0x1416a3142
    0x1416a2ef3    4d85f6                   test             r14, r14
    0x1416a2ef6    0f8496000000             je               0x1416a2f92
    0x1416a2efc    488d8dd0000000           lea              rcx, [rbp + 0xd0]
    0x1416a2f03    e8982253ff               call             0x140bd51a0 -> rhapsody::raw::buffer::impl$4::byte_count
    0x1416a2f08    4c39f8                   cmp              rax, r15
    0x1416a2f0b    0f874b020000             ja               0x1416a315c
    0x1416a2f11    488d8dd0000000           lea              rcx, [rbp + 0xd0]
    0x1416a2f18    4c89f2                   mov              rdx, r14
    0x1416a2f1b    4989c0                   mov              r8, rax
    0x1416a2f1e    e88d2253ff               call             0x140bd51b0 -> rhapsody::raw::buffer::impl$4::copy_to_slice
    0x1416a2f23    488b4548                 mov              rax, qword ptr [rbp + 0x48]
    0x1416a2f27    48894628                 mov              qword ptr [rsi + 0x28], rax
    0x1416a2f2b    0f104528                 movups           xmm0, xmmword ptr [rbp + 0x28]
    0x1416a2f2f    0f104d38                 movups           xmm1, xmmword ptr [rbp + 0x38]
    0x1416a2f33    0f114e18                 movups           xmmword ptr [rsi + 0x18], xmm1
    0x1416a2f37    0f114608                 movups           xmmword ptr [rsi + 8], xmm0
    0x1416a2f3b    48c7060b000000           mov              qword ptr [rsi], 0xb
    0x1416a2f42    eb37                     jmp              0x1416a2f7b
    0x1416a2f44    488b8d98000000           mov              rcx, qword ptr [rbp + 0x98]
    0x1416a2f4b    0f2885a0000000           movaps           xmm0, xmmword ptr [rbp + 0xa0]
    0x1416a2f52    0f288db0000000           movaps           xmm1, xmmword ptr [rbp + 0xb0]
    0x1416a2f59    0f2895c0000000           movaps           xmm2, xmmword ptr [rbp + 0xc0]
    0x1416a2f60    0f115628                 movups           xmmword ptr [rsi + 0x28], xmm2
    0x1416a2f64    0f114e18                 movups           xmmword ptr [rsi + 0x18], xmm1
    0x1416a2f68    0f114608                 movups           xmmword ptr [rsi + 8], xmm0
    0x1416a2f6c    488906                   mov              qword ptr [rsi], rax
    0x1416a2f6f    4c897638                 mov              qword ptr [rsi + 0x38], r14
    0x1416a2f73    4c897e40                 mov              qword ptr [rsi + 0x40], r15
    0x1416a2f77    48894e48                 mov              qword ptr [rsi + 0x48], rcx
    0x1416a2f7b    4889f0                   mov              rax, rsi
    0x1416a2f7e    4881c468010000           add              rsp, 0x168
    0x1416a2f85    5b                       pop              rbx
    0x1416a2f86    5f                       pop              rdi
    0x1416a2f87    5e                       pop              rsi
    0x1416a2f88    415c                     pop              r12
    0x1416a2f8a    415d                     pop              r13
    0x1416a2f8c    415e                     pop              r14
    0x1416a2f8e    415f                     pop              r15
    0x1416a2f90    5d                       pop              rbp
    0x1416a2f91    c3                       ret              
    0x1416a2f92    4c8d75c0                 lea              r14, [rbp - 0x40]
    0x1416a2f96    4c89f1                   mov              rcx, r14
    0x1416a2f99    41b801000000             mov              r8d, 1
    0x1416a2f9f    e8fc1e53ff               call             0x140bd4ea0 -> rhapsody::raw::buffer::BufferDesc::new_cpu_to_gpu_low_prio
    0x1416a2fa4    4889f9                   mov              rcx, rdi
    0x1416a2fa7    e8b45955ff               call             0x140bf8960 -> rhapsody::raw::device::GraphicsDevice::debug_utils
    0x1416a2fac    4c8b8f48060000           mov              r9, qword ptr [rdi + 0x648]
    0x1416a2fb3    488b8f78060000           mov              rcx, qword ptr [rdi + 0x678]
    0x1416a2fba    4883c110                 add              rcx, 0x10
    0x1416a2fbe    488d155b595e01           lea              rdx, [rip + 0x15e595b] -> 0x142c88920
    0x1416a2fc5    4889542430               mov              qword ptr [rsp + 0x30], rdx
    0x1416a2fca    4c89742428               mov              qword ptr [rsp + 0x28], r14
    0x1416a2fcf    48894c2420               mov              qword ptr [rsp + 0x20], rcx
    0x1416a2fd4    48c744243813000000       mov              qword ptr [rsp + 0x38], 0x13
    0x1416a2fdd    4c8d7550                 lea              r14, [rbp + 0x50]
    0x1416a2fe1    4c89f1                   mov              rcx, r14
    0x1416a2fe4    4889da                   mov              rdx, rbx
    0x1416a2fe7    4989c0                   mov              r8, rax
    0x1416a2fea    e811b254ff               call             0x140bee200 -> rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_buffer_impl::h34de0f745611d2ab
    0x1416a2fef    488b4550                 mov              rax, qword ptr [rbp + 0x50]
    0x1416a2ff3    0f104558                 movups           xmm0, xmmword ptr [rbp + 0x58]
    0x1416a2ff7    0f2985a0000000           movaps           xmmword ptr [rbp + 0xa0], xmm0
    0x1416a2ffe    0f104568                 movups           xmm0, xmmword ptr [rbp + 0x68]
    0x1416a3002    0f2985b0000000           movaps           xmmword ptr [rbp + 0xb0], xmm0
    0x1416a3009    0f104578                 movups           xmm0, xmmword ptr [rbp + 0x78]
    0x1416a300d    0f2985c0000000           movaps           xmmword ptr [rbp + 0xc0], xmm0
    0x1416a3014    488b9d88000000           mov              rbx, qword ptr [rbp + 0x88]
    0x1416a301b    4c8bbd90000000           mov              r15, qword ptr [rbp + 0x90]
    0x1416a3022    4883f80b                 cmp              rax, 0xb
    0x1416a3026    0f859b000000             jne              0x1416a30c7
    0x1416a302c    0f2885a0000000           movaps           xmm0, xmmword ptr [rbp + 0xa0]
    0x1416a3033    0f288db0000000           movaps           xmm1, xmmword ptr [rbp + 0xb0]
    0x1416a303a    0f2895c0000000           movaps           xmm2, xmmword ptr [rbp + 0xc0]
    0x1416a3041    0f295570                 movaps           xmmword ptr [rbp + 0x70], xmm2
    0x1416a3045    0f294d60                 movaps           xmmword ptr [rbp + 0x60], xmm1
    0x1416a3049    0f294550                 movaps           xmmword ptr [rbp + 0x50], xmm0
    0x1416a304d    4885db                   test             rbx, rbx
    0x1416a3050    0f842e010000             je               0x1416a3184
    0x1416a3056    488d8dd0000000           lea              rcx, [rbp + 0xd0]
    0x1416a305d    e83e2153ff               call             0x140bd51a0 -> rhapsody::raw::buffer::impl$4::byte_count
    0x1416a3062    4c39f8                   cmp              rax, r15
    0x1416a3065    0f8705010000             ja               0x1416a3170
    0x1416a306b    4c8dbdd0000000           lea              r15, [rbp + 0xd0]
    0x1416a3072    4c89f9                   mov              rcx, r15
    0x1416a3075    4889da                   mov              rdx, rbx
    0x1416a3078    4989c0                   mov              r8, rax
    0x1416a307b    e8302153ff               call             0x140bd51b0 -> rhapsody::raw::buffer::impl$4::copy_to_slice
    0x1416a3080    4c89b5a0000000           mov              qword ptr [rbp + 0xa0], r14
    0x1416a3087    4889bda8000000           mov              qword ptr [rbp + 0xa8], rdi
    0x1416a308e    4c89a5b0000000           mov              qword ptr [rbp + 0xb0], r12
    0x1416a3095    4c89bdb8000000           mov              qword ptr [rbp + 0xb8], r15
    0x1416a309c    488d4dd0                 lea              rcx, [rbp - 0x30]
    0x1416a30a0    4c8d85a0000000           lea              r8, [rbp + 0xa0]
    0x1416a30a7    4889fa                   mov              rdx, rdi
    0x1416a30aa    e8911756ff               call             0x140c04840 -> rhapsody::raw::device::GraphicsDevice::with_setup_cb::h946964bfafb47630
    0x1416a30af    488b45d0                 mov              rax, qword ptr [rbp - 0x30]
    0x1416a30b3    4883f80b                 cmp              rax, 0xb
    0x1416a30b7    7547                     jne              0x1416a3100
    0x1416a30b9    488d4d58                 lea              rcx, [rbp + 0x58]
    0x1416a30bd    e8eef353ff               call             0x140be24b0 -> rhapsody::raw::managed::handle::impl$6::drop
    0x1416a30c2    e95cfeffff               jmp              0x1416a2f23
    0x1416a30c7    488b8d98000000           mov              rcx, qword ptr [rbp + 0x98]
    0x1416a30ce    0f2885a0000000           movaps           xmm0, xmmword ptr [rbp + 0xa0]
    0x1416a30d5    0f288db0000000           movaps           xmm1, xmmword ptr [rbp + 0xb0]
    0x1416a30dc    0f2895c0000000           movaps           xmm2, xmmword ptr [rbp + 0xc0]
    0x1416a30e3    0f115628                 movups           xmmword ptr [rsi + 0x28], xmm2
    0x1416a30e7    0f114e18                 movups           xmmword ptr [rsi + 0x18], xmm1
    0x1416a30eb    0f114608                 movups           xmmword ptr [rsi + 8], xmm0
    0x1416a30ef    488906                   mov              qword ptr [rsi], rax
    0x1416a30f2    48895e38                 mov              qword ptr [rsi + 0x38], rbx
    0x1416a30f6    4c897e40                 mov              qword ptr [rsi + 0x40], r15
    0x1416a30fa    48894e48                 mov              qword ptr [rsi + 0x48], rcx
    0x1416a30fe    eb34                     jmp              0x1416a3134
    0x1416a3100    488b4d18                 mov              rcx, qword ptr [rbp + 0x18]
    0x1416a3104    48894e48                 mov              qword ptr [rsi + 0x48], rcx
    0x1416a3108    0f1045d8                 movups           xmm0, xmmword ptr [rbp - 0x28]
    0x1416a310c    0f104de8                 movups           xmm1, xmmword ptr [rbp - 0x18]
    0x1416a3110    0f1055f8                 movups           xmm2, xmmword ptr [rbp - 8]
    0x1416a3114    0f105d08                 movups           xmm3, xmmword ptr [rbp + 8]
    0x1416a3118    0f115e38                 movups           xmmword ptr [rsi + 0x38], xmm3
    0x1416a311c    0f115628                 movups           xmmword ptr [rsi + 0x28], xmm2
    0x1416a3120    0f114e18                 movups           xmmword ptr [rsi + 0x18], xmm1
    0x1416a3124    0f114608                 movups           xmmword ptr [rsi + 8], xmm0
    0x1416a3128    488906                   mov              qword ptr [rsi], rax
    0x1416a312b    488d4d58                 lea              rcx, [rbp + 0x58]
    0x1416a312f    e87cf353ff               call             0x140be24b0 -> rhapsody::raw::managed::handle::impl$6::drop
    0x1416a3134    488d4d28                 lea              rcx, [rbp + 0x28]
    0x1416a3138    e873f353ff               call             0x140be24b0 -> rhapsody::raw::managed::handle::impl$6::drop
    0x1416a313d    e939feffff               jmp              0x1416a2f7b
    0x1416a3142    488d0d17575e01           lea              rcx, [rip + 0x15e5717] -> 0x142c88860
    0x1416a3149    4c8d05a0575e01           lea              r8, [rip + 0x15e57a0] -> 0x142c888f0
    0x1416a3150    ba38000000               mov              edx, 0x38
    0x1416a3155    e8d6622301               call             0x1428d9430 -> core::panicking::panic::hb210ae7b817a9f13
    0x1416a315a    eb40                     jmp              0x1416a319c
    0x1416a315c    4c8d05a5575e01           lea              r8, [rip + 0x15e57a5] -> 0x142c88908
    0x1416a3163    4889c1                   mov              rcx, rax
    0x1416a3166    4c89fa                   mov              rdx, r15
    0x1416a3169    e8d2662301               call             0x1428d9840 -> core::slice::index::slice_end_index_len_fail::hc3a65ec02a6eede1
    0x1416a316e    eb2c                     jmp              0x1416a319c
    0x1416a3170    4c8d05e9575e01           lea              r8, [rip + 0x15e57e9] -> 0x142c88960
    0x1416a3177    4889c1                   mov              rcx, rax
    0x1416a317a    4c89fa                   mov              rdx, r15
    0x1416a317d    e8be662301               call             0x1428d9840 -> core::slice::index::slice_end_index_len_fail::hc3a65ec02a6eede1
    0x1416a3182    eb18                     jmp              0x1416a319c
    0x1416a3184    488d0da8575e01           lea              rcx, [rip + 0x15e57a8] -> 0x142c88933
    0x1416a318b    4c8d05b6575e01           lea              r8, [rip + 0x15e57b6] -> 0x142c88948
    0x1416a3192    ba14000000               mov              edx, 0x14
    0x1416a3197    e894612301               call             0x1428d9330 -> core::option::expect_failed::hb3e848c6b16873c8
    0x1416a319c    0f0b                     ud2              
    0x1416a319e    6690                     nop              
    0x1416a31a0    4889542410               mov              qword ptr [rsp + 0x10], rdx
    0x1416a31a5    55                       push             rbp
    0x1416a31a6    4157                     push             r15
    0x1416a31a8    4156                     push             r14
    0x1416a31aa    4155                     push             r13
    0x1416a31ac    4154                     push             r12
    0x1416a31ae    56                       push             rsi
    0x1416a31af    57                       push             rdi
    0x1416a31b0    53                       push             rbx
    0x1416a31b1    4883ec48                 sub              rsp, 0x48
    0x1416a31b5    488daa80000000           lea              rbp, [rdx + 0x80]
    0x1416a31bc    488d4d58                 lea              rcx, [rbp + 0x58]
    0x1416a31c0    e8ebf253ff               call             0x140be24b0 -> rhapsody::raw::managed::handle::impl$6::drop
    0x1416a31c5    90                       nop              
    0x1416a31c6    4883c448                 add              rsp, 0x48
    0x1416a31ca    5b                       pop              rbx
    0x1416a31cb    5f                       pop              rdi
    0x1416a31cc    5e                       pop              rsi
    0x1416a31cd    415c                     pop              r12
    0x1416a31cf    415d                     pop              r13
    0x1416a31d1    415e                     pop              r14
    0x1416a31d3    415f                     pop              r15
    0x1416a31d5    5d                       pop              rbp
    0x1416a31d6    c3                       ret              
    0x1416a31d7    660f1f840000000000       nop              word ptr [rax + rax]
    0x1416a31e0    4889542410               mov              qword ptr [rsp + 0x10], rdx
    0x1416a31e5    55                       push             rbp
    0x1416a31e6    4157                     push             r15
    0x1416a31e8    4156                     push             r14
    0x1416a31ea    4155                     push             r13
    0x1416a31ec    4154                     push             r12
    0x1416a31ee    56                       push             rsi
    0x1416a31ef    57                       push             rdi
    0x1416a31f0    53                       push             rbx
    0x1416a31f1    4883ec48                 sub              rsp, 0x48
    0x1416a31f5    488daa80000000           lea              rbp, [rdx + 0x80]
    0x1416a31fc    488d4d28                 lea              rcx, [rbp + 0x28]
    0x1416a3200    e8abf253ff               call             0x140be24b0 -> rhapsody::raw::managed::handle::impl$6::drop
    0x1416a3205    90                       nop              
    0x1416a3206    4883c448                 add              rsp, 0x48
    0x1416a320a    5b                       pop              rbx
    0x1416a320b    5f                       pop              rdi
    0x1416a320c    5e                       pop              rsi
    0x1416a320d    415c                     pop              r12
    0x1416a320f    415d                     pop              r13
    0x1416a3211    415e                     pop              r14
    0x1416a3213    415f                     pop              r15
    0x1416a3215    5d                       pop              rbp
    0x1416a3216    c3                       ret              

=== 0x1416a3220 size=1012  rhapsody::raw::device::GraphicsDevice::with_setup_cb::h8cd7a8dc0d8d3a74
    ; Large function body fully decoded (not a micro-leaf); decode present in insns.  [medium]
    0x1416a3220    55                       push             rbp
    0x1416a3221    4157                     push             r15
    0x1416a3223    4156                     push             r14
    0x1416a3225    4155                     push             r13
    0x1416a3227    4154                     push             r12
    0x1416a3229    56                       push             rsi
    0x1416a322a    57                       push             rdi
    0x1416a322b    53                       push             rbx
    0x1416a322c    4881ec08010000           sub              rsp, 0x108
    0x1416a3233    488dac2480000000         lea              rbp, [rsp + 0x80]
    0x1416a323b    48c78580000000feffffff   mov              qword ptr [rbp + 0x80], 0xfffffffffffffffe
    0x1416a3246    4c89c7                   mov              rdi, r8
    0x1416a3249    4989d6                   mov              r14, rdx
    0x1416a324c    4889ce                   mov              rsi, rcx
    0x1416a324f    488d9af0090000           lea              rbx, [rdx + 0x9f0]
    0x1416a3256    b101                     mov              cl, 1
    0x1416a3258    31c0                     xor              eax, eax
    0x1416a325a    f00fb08af0090000         lock cmpxchg     byte ptr [rdx + 0x9f0], cl
    0x1416a3262    0f8550020000             jne              0x1416a34b8
    0x1416a3268    48895d60                 mov              qword ptr [rbp + 0x60], rbx
    0x1416a326c    4889d9                   mov              rcx, rbx
    0x1416a326f    e8fce95dff               call             0x140c81c70 -> parking_lot_core::parking_lot::deadlock_impl::acquire_resource::hb2f16ac28e483830
    0x1416a3274    4d8daef8090000           lea              r13, [r14 + 0x9f8]
    0x1416a327b    4183bef809000004         cmp              dword ptr [r14 + 0x9f8], 4
    0x1416a3283    0f857d010000             jne              0x1416a3406
    0x1416a3289    4c89f1                   mov              rcx, r14
    0x1416a328c    31d2                     xor              edx, edx
    0x1416a328e    e8bd9455ff               call             0x140bfc750 -> rhapsody::raw::device::GraphicsDevice::queue_family_index
    0x1416a3293    89442420                 mov              dword ptr [rsp + 0x20], eax
    0x1416a3297    4c8d05da565e01           lea              r8, [rip + 0x15e56da] -> 0x142c88978
    0x1416a329e    488d4da8                 lea              rcx, [rbp - 0x58]
    0x1416a32a2    41b908000000             mov              r9d, 8
    0x1416a32a8    4c89f2                   mov              rdx, r14
    0x1416a32ab    e8f0c154ff               call             0x140bef4a0 -> rhapsody::raw::command_buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_command_buffer::h590d93c90b46ccfb
    0x1416a32b0    c6457f01                 mov              byte ptr [rbp + 0x7f], 1
    0x1416a32b4    488d4da8                 lea              rcx, [rbp - 0x58]
    0x1416a32b8    4c897568                 mov              qword ptr [rbp + 0x68], r14
    0x1416a32bc    4c89f2                   mov              rdx, r14
    0x1416a32bf    4531c0                   xor              r8d, r8d
    0x1416a32c2    e8699f54ff               call             0x140bed230 -> rhapsody::raw::command_buffer::CommandBuffer::begin
    0x1416a32c7    488b1f                   mov              rbx, qword ptr [rdi]
    0x1416a32ca    4c8b6708                 mov              r12, qword ptr [rdi + 8]
    0x1416a32ce    4c8b7f10                 mov              r15, qword ptr [rdi + 0x10]
    0x1416a32d2    488b7f18                 mov              rdi, qword ptr [rdi + 0x18]
    0x1416a32d6    488d4b08                 lea              rcx, [rbx + 8]
    0x1416a32da    c6457f01                 mov              byte ptr [rbp + 0x7f], 1
    0x1416a32de    e89daafdfe               call             0x14067dd80 -> rhapsody::raw::allocator::GpuAllocator::inner
    0x1416a32e3    c6457f01                 mov              byte ptr [rbp + 0x7f], 1
    0x1416a32e7    488d55a8                 lea              rdx, [rbp - 0x58]
    0x1416a32eb    4889c1                   mov              rcx, rax
    0x1416a32ee    e87d3753ff               call             0x140bd6a70 -> rhapsody::raw::managed::handle::impl$4::track
    0x1416a32f3    488b03                   mov              rax, qword ptr [rbx]
    0x1416a32f6    488b5df0                 mov              rbx, qword ptr [rbp - 0x10]
    0x1416a32fa    4c8b7020                 mov              r14, qword ptr [rax + 0x20]
    0x1416a32fe    498d4f08                 lea              rcx, [r15 + 8]
    0x1416a3302    c6457f01                 mov              byte ptr [rbp + 0x7f], 1
    0x1416a3306    e875aafdfe               call             0x14067dd80 -> rhapsody::raw::allocator::GpuAllocator::inner
    0x1416a330b    c6457f01                 mov              byte ptr [rbp + 0x7f], 1
    0x1416a330f    488d55a8                 lea              rdx, [rbp - 0x58]
    0x1416a3313    4889c1                   mov              rcx, rax
    0x1416a3316    e8553753ff               call             0x140bd6a70 -> rhapsody::raw::managed::handle::impl$4::track
    0x1416a331b    498b07                   mov              rax, qword ptr [r15]
    0x1416a331e    4c8b7820                 mov              r15, qword ptr [rax + 0x20]
    0x1416a3322    c6457f01                 mov              byte ptr [rbp + 0x7f], 1
    0x1416a3326    4889f9                   mov              rcx, rdi
    0x1416a3329    e88235fefe               call             0x1406868b0 -> rhapsody::raw::buffer::impl$3::byte_count
    0x1416a332e    0f57c0                   xorps            xmm0, xmm0
    0x1416a3331    0f294500                 movaps           xmmword ptr [rbp], xmm0
    0x1416a3335    48894510                 mov              qword ptr [rbp + 0x10], rax
    0x1416a3339    4889e8                   mov              rax, rbp
    0x1416a333c    4889442420               mov              qword ptr [rsp + 0x20], rax
    0x1416a3341    4889d9                   mov              rcx, rbx
    0x1416a3344    4c89f2                   mov              rdx, r14
    0x1416a3347    4d89f8                   mov              r8, r15
    0x1416a334a    41b901000000             mov              r9d, 1
    0x1416a3350    41ff942470030000         call             qword ptr [r12 + 0x370]
    0x1416a3358    488b45f8                 mov              rax, qword ptr [rbp - 8]
    0x1416a335c    48894550                 mov              qword ptr [rbp + 0x50], rax
    0x1416a3360    0f1045e8                 movups           xmm0, xmmword ptr [rbp - 0x18]
    0x1416a3364    0f294540                 movaps           xmmword ptr [rbp + 0x40], xmm0
    0x1416a3368    0f1045a8                 movups           xmm0, xmmword ptr [rbp - 0x58]
    0x1416a336c    0f104db8                 movups           xmm1, xmmword ptr [rbp - 0x48]
    0x1416a3370    0f1055c8                 movups           xmm2, xmmword ptr [rbp - 0x38]
    0x1416a3374    0f105dd8                 movups           xmm3, xmmword ptr [rbp - 0x28]
    0x1416a3378    0f295d30                 movaps           xmmword ptr [rbp + 0x30], xmm3
    0x1416a337c    0f295520                 movaps           xmmword ptr [rbp + 0x20], xmm2
    0x1416a3380    0f294d10                 movaps           xmmword ptr [rbp + 0x10], xmm1
    0x1416a3384    0f294500                 movaps           xmmword ptr [rbp], xmm0
    0x1416a3388    41837d0004               cmp              dword ptr [r13], 4
    0x1416a338d    4c896d70                 mov              qword ptr [rbp + 0x70], r13
    0x1416a3391    743e                     je               0x1416a33d1
    0x1416a3393    488b4d70                 mov              rcx, qword ptr [rbp + 0x70]
    0x1416a3397    e8d4ef53ff               call             0x140be2370 -> rhapsody::raw::command_buffer::impl$4::drop
    0x1416a339c    488b7d68                 mov              rdi, qword ptr [rbp + 0x68]
    0x1416a33a0    488b97100a0000           mov              rdx, qword ptr [rdi + 0xa10]
    0x1416a33a7    4885d2                   test             rdx, rdx
    0x1416a33aa    7416                     je               0x1416a33c2
    0x1416a33ac    488b8f180a0000           mov              rcx, qword ptr [rdi + 0xa18]
    0x1416a33b3    48c1e203                 shl              rdx, 3
    0x1416a33b7    41b808000000             mov              r8d, 8
    0x1416a33bd    e85e08f7fe               call             0x140613c20 -> __rust_dealloc
    0x1416a33c2    4881c7280a0000           add              rdi, 0xa28
    0x1416a33c9    4889f9                   mov              rcx, rdi
    0x1416a33cc    e82fc2f3fe               call             0x1405df600 -> core::ptr::drop_in_place$LT$alloc..vec..Vec$LT$$LP$ash..vk..definitions..DescriptorPool$C$alloc..sync..Arc$LT$rhapsody..raw..shader..DescriptorPoolDiscardPile$GT$$RP$$GT$$GT$::h3c71e16060d2e3d5 (.llvm.10346396820423203432)
    0x1416a33d1    488b4550                 mov              rax, qword ptr [rbp + 0x50]
    0x1416a33d5    488b4d70                 mov              rcx, qword ptr [rbp + 0x70]
    0x1416a33d9    48894150                 mov              qword ptr [rcx + 0x50], rax
    0x1416a33dd    0f284540                 movaps           xmm0, xmmword ptr [rbp + 0x40]
    0x1416a33e1    0f114140                 movups           xmmword ptr [rcx + 0x40], xmm0
    0x1416a33e5    0f284500                 movaps           xmm0, xmmword ptr [rbp]
    0x1416a33e9    0f284d10                 movaps           xmm1, xmmword ptr [rbp + 0x10]
    0x1416a33ed    0f285520                 movaps           xmm2, xmmword ptr [rbp + 0x20]
    0x1416a33f1    0f285d30                 movaps           xmm3, xmmword ptr [rbp + 0x30]
    0x1416a33f5    0f115930                 movups           xmmword ptr [rcx + 0x30], xmm3
    0x1416a33f9    0f115120                 movups           xmmword ptr [rcx + 0x20], xmm2
    0x1416a33fd    0f114910                 movups           xmmword ptr [rcx + 0x10], xmm1
    0x1416a3401    0f1101                   movups           xmmword ptr [rcx], xmm0
    0x1416a3404    eb7e                     jmp              0x1416a3484
    0x1416a3406    488b1f                   mov              rbx, qword ptr [rdi]
    0x1416a3409    4c8b6708                 mov              r12, qword ptr [rdi + 8]
    0x1416a340d    4c8b7f10                 mov              r15, qword ptr [rdi + 0x10]
    0x1416a3411    488b7f18                 mov              rdi, qword ptr [rdi + 0x18]
    0x1416a3415    488d4b08                 lea              rcx, [rbx + 8]
    0x1416a3419    e862a9fdfe               call             0x14067dd80 -> rhapsody::raw::allocator::GpuAllocator::inner
    0x1416a341e    4889c1                   mov              rcx, rax
    0x1416a3421    4c89ea                   mov              rdx, r13
    0x1416a3424    e8473653ff               call             0x140bd6a70 -> rhapsody::raw::managed::handle::impl$4::track
    0x1416a3429    488b03                   mov              rax, qword ptr [rbx]
    0x1416a342c    498b9e400a0000           mov              rbx, qword ptr [r14 + 0xa40]
    0x1416a3433    4c8b7020                 mov              r14, qword ptr [rax + 0x20]
    0x1416a3437    498d4f08                 lea              rcx, [r15 + 8]
    0x1416a343b    e840a9fdfe               call             0x14067dd80 -> rhapsody::raw::allocator::GpuAllocator::inner
    0x1416a3440    4889c1                   mov              rcx, rax
    0x1416a3443    4c89ea                   mov              rdx, r13
    0x1416a3446    e8253653ff               call             0x140bd6a70 -> rhapsody::raw::managed::handle::impl$4::track
    0x1416a344b    498b07                   mov              rax, qword ptr [r15]
    0x1416a344e    4c8b7820                 mov              r15, qword ptr [rax + 0x20]
    0x1416a3452    4889f9                   mov              rcx, rdi
    0x1416a3455    e85634fefe               call             0x1406868b0 -> rhapsody::raw::buffer::impl$3::byte_count
    0x1416a345a    0f57c0                   xorps            xmm0, xmm0
    0x1416a345d    0f294500                 movaps           xmmword ptr [rbp], xmm0
    0x1416a3461    48894510                 mov              qword ptr [rbp + 0x10], rax
    0x1416a3465    4889e8                   mov              rax, rbp
    0x1416a3468    4889442420               mov              qword ptr [rsp + 0x20], rax
    0x1416a346d    4889d9                   mov              rcx, rbx
    0x1416a3470    4c89f2                   mov              rdx, r14
    0x1416a3473    4d89f8                   mov              r8, r15
    0x1416a3476    41b901000000             mov              r9d, 1
    0x1416a347c    41ff942470030000         call             qword ptr [r12 + 0x370]
    0x1416a3484    48c7060b000000           mov              qword ptr [rsi], 0xb
    0x1416a348b    488b7d60                 mov              rdi, qword ptr [rbp + 0x60]
    0x1416a348f    4889f9                   mov              rcx, rdi
    0x1416a3492    e879e95dff               call             0x140c81e10 -> parking_lot_core::parking_lot::deadlock_impl::release_resource::h1c3ab4db915ad6f2
    0x1416a3497    31c9                     xor              ecx, ecx
    0x1416a3499    b001                     mov              al, 1
    0x1416a349b    f00fb00f                 lock cmpxchg     byte ptr [rdi], cl
    0x1416a349f    752a                     jne              0x1416a34cb
    0x1416a34a1    4889f0                   mov              rax, rsi
    0x1416a34a4    4881c408010000           add              rsp, 0x108
    0x1416a34ab    5b                       pop              rbx
    0x1416a34ac    5f                       pop              rdi
    0x1416a34ad    5e                       pop              rsi
    0x1416a34ae    415c                     pop              r12
    0x1416a34b0    415d                     pop              r13
    0x1416a34b2    415e                     pop              r14
    0x1416a34b4    415f                     pop              r15
    0x1416a34b6    5d                       pop              rbp
    0x1416a34b7    c3                       ret              
    0x1416a34b8    4889d9                   mov              rcx, rbx
    0x1416a34bb    41b800ca9a3b             mov              r8d, 0x3b9aca00
    0x1416a34c1    e84a9d2201               call             0x1428cd210 -> parking_lot::raw_mutex::RawMutex::lock_slow::h7c244b522f551b38
    0x1416a34c6    e99dfdffff               jmp              0x1416a3268
    0x1416a34cb    4889f9                   mov              rcx, rdi
    0x1416a34ce    31d2                     xor              edx, edx
    0x1416a34d0    e8bba62201               call             0x1428cdb90 -> parking_lot::raw_mutex::RawMutex::unlock_slow::h56cb6df3e32a2657
    0x1416a34d5    ebca                     jmp              0x1416a34a1
    0x1416a34d7    660f1f840000000000       nop              word ptr [rax + rax]
    0x1416a34e0    4889542410               mov              qword ptr [rsp + 0x10], rdx
    0x1416a34e5    55                       push             rbp
    0x1416a34e6    4157                     push             r15
    0x1416a34e8    4156                     push             r14
    0x1416a34ea    4155                     push             r13
    0x1416a34ec    4154                     push             r12
    0x1416a34ee    56                       push             rsi
    0x1416a34ef    57                       push             rdi
    0x1416a34f0    53                       push             rbx
    0x1416a34f1    4883ec28                 sub              rsp, 0x28
    0x1416a34f5    488daa80000000           lea              rbp, [rdx + 0x80]
    0x1416a34fc    807d7f00                 cmp              byte ptr [rbp + 0x7f], 0
    0x1416a3500    741b                     je               0x1416a351d
    0x1416a3502    488d4da8                 lea              rcx, [rbp - 0x58]
    0x1416a3506    e80511f3fe               call             0x1405d4610 -> core::ptr::drop_in_place$LT$rhapsody..raw..command_buffer..CommandBuffer$GT$::hb8f001d863c8dde2
    0x1416a350b    90                       nop              
    0x1416a350c    4883c428                 add              rsp, 0x28
    0x1416a3510    5b                       pop              rbx
    0x1416a3511    5f                       pop              rdi
    0x1416a3512    5e                       pop              rsi
    0x1416a3513    415c                     pop              r12
    0x1416a3515    415d                     pop              r13
    0x1416a3517    415e                     pop              r14
    0x1416a3519    415f                     pop              r15
    0x1416a351b    5d                       pop              rbp
    0x1416a351c    c3                       ret              
    0x1416a351d    4883c428                 add              rsp, 0x28
    0x1416a3521    5b                       pop              rbx
    0x1416a3522    5f                       pop              rdi
    0x1416a3523    5e                       pop              rsi
    0x1416a3524    415c                     pop              r12
    0x1416a3526    415d                     pop              r13
    0x1416a3528    415e                     pop              r14
    0x1416a352a    415f                     pop              r15
    0x1416a352c    5d                       pop              rbp
    0x1416a352d    c3                       ret              
    0x1416a352e    6690                     nop              
    0x1416a3530    4889542410               mov              qword ptr [rsp + 0x10], rdx
    0x1416a3535    55                       push             rbp
    0x1416a3536    4157                     push             r15
    0x1416a3538    4156                     push             r14
    0x1416a353a    4155                     push             r13
    0x1416a353c    4154                     push             r12
    0x1416a353e    56                       push             rsi
    0x1416a353f    57                       push             rdi
    0x1416a3540    53                       push             rbx
    0x1416a3541    4883ec28                 sub              rsp, 0x28
    0x1416a3545    488daa80000000           lea              rbp, [rdx + 0x80]
    0x1416a354c    488b4d60                 mov              rcx, qword ptr [rbp + 0x60]
    0x1416a3550    e8dbe6abfe               call             0x140161c30 -> core::ptr::drop_in_place$LT$lock_api..mutex..MutexGuard$LT$parking_lot..raw_mutex..RawMutex$C$alloc..vec..Vec$LT$glam..f32..vec2..Vec2$GT$$GT$$GT$::hc1588d55d4486b0e (.llvm.14454403653567781691)
    0x1416a3555    90                       nop              
    0x1416a3556    4883c428                 add              rsp, 0x28
    0x1416a355a    5b                       pop              rbx
    0x1416a355b    5f                       pop              rdi
    0x1416a355c    5e                       pop              rsi
    0x1416a355d    415c                     pop              r12
    0x1416a355f    415d                     pop              r13
    0x1416a3561    415e                     pop              r14
    0x1416a3563    415f                     pop              r15
    0x1416a3565    5d                       pop              rbp
    0x1416a3566    c3                       ret              
    0x1416a3567    660f1f840000000000       nop              word ptr [rax + rax]
    0x1416a3570    4889542410               mov              qword ptr [rsp + 0x10], rdx
    0x1416a3575    55                       push             rbp
    0x1416a3576    4157                     push             r15
    0x1416a3578    4156                     push             r14
    0x1416a357a    4155                     push             r13
    0x1416a357c    4154                     push             r12
    0x1416a357e    56                       push             rsi
    0x1416a357f    57                       push             rdi
    0x1416a3580    53                       push             rbx
    0x1416a3581    4883ec28                 sub              rsp, 0x28
    0x1416a3585    488daa80000000           lea              rbp, [rdx + 0x80]
    0x1416a358c    488b4d68                 mov              rcx, qword ptr [rbp + 0x68]
    0x1416a3590    4881c1100a0000           add              rcx, 0xa10
    0x1416a3597    e8f4bcf5fe               call             0x1405ff290 -> core::ptr::drop_in_place$LT$rhapsody..raw..command_buffer..CommandBufferResources$GT$::h7083992529333ee9 (.llvm.10346396820423203432)
    0x1416a359c    90                       nop              
    0x1416a359d    4883c428                 add              rsp, 0x28
    0x1416a35a1    5b                       pop              rbx
    0x1416a35a2    5f                       pop              rdi
    0x1416a35a3    5e                       pop              rsi
    0x1416a35a4    415c                     pop              r12
    0x1416a35a6    415d                     pop              r13
    0x1416a35a8    415e                     pop              r14
    0x1416a35aa    415f                     pop              r15
    0x1416a35ac    5d                       pop              rbp
    0x1416a35ad    c3                       ret              
    0x1416a35ae    6690                     nop              
    0x1416a35b0    4889542410               mov              qword ptr [rsp + 0x10], rdx
    0x1416a35b5    55                       push             rbp
    0x1416a35b6    4157                     push             r15
    0x1416a35b8    4156                     push             r14
    0x1416a35ba    4155                     push             r13
    0x1416a35bc    4154                     push             r12
    0x1416a35be    56                       push             rsi
    0x1416a35bf    57                       push             rdi
    0x1416a35c0    53                       push             rbx
    0x1416a35c1    4883ec28                 sub              rsp, 0x28
    0x1416a35c5    488daa80000000           lea              rbp, [rdx + 0x80]
    0x1416a35cc    488b4550                 mov              rax, qword ptr [rbp + 0x50]
    0x1416a35d0    488b4d70                 mov              rcx, qword ptr [rbp + 0x70]
    0x1416a35d4    48894150                 mov              qword ptr [rcx + 0x50], rax
    0x1416a35d8    0f284540                 movaps           xmm0, xmmword ptr [rbp + 0x40]
    0x1416a35dc    0f114140                 movups           xmmword ptr [rcx + 0x40], xmm0
    0x1416a35e0    0f284500                 movaps           xmm0, xmmword ptr [rbp]
    0x1416a35e4    0f284d10                 movaps           xmm1, xmmword ptr [rbp + 0x10]
    0x1416a35e8    0f285520                 movaps           xmm2, xmmword ptr [rbp + 0x20]
    0x1416a35ec    0f285d30                 movaps           xmm3, xmmword ptr [rbp + 0x30]
    0x1416a35f0    0f115930                 movups           xmmword ptr [rcx + 0x30], xmm3
    0x1416a35f4    0f115120                 movups           xmmword ptr [rcx + 0x20], xmm2
    0x1416a35f8    0f114910                 movups           xmmword ptr [rcx + 0x10], xmm1
    0x1416a35fc    0f1101                   movups           xmmword ptr [rcx], xmm0
    0x1416a35ff    c6457f00                 mov              byte ptr [rbp + 0x7f], 0
    0x1416a3603    4883c428                 add              rsp, 0x28
    0x1416a3607    5b                       pop              rbx
    0x1416a3608    5f                       pop              rdi
    0x1416a3609    5e                       pop              rsi
    0x1416a360a    415c                     pop              r12
    0x1416a360c    415d                     pop              r13
    0x1416a360e    415e                     pop              r14
    0x1416a3610    415f                     pop              r15
    0x1416a3612    5d                       pop              rbp
    0x1416a3613    c3                       ret              

=== 0x1416a71f0 size=2794  rhapsody::raw::device::GraphicsDevice::create_texture<enum2$<rhapsody::raw::texture::TextureInitialData> >
    ; Large function body fully decoded (not a micro-leaf); decode present in insns.  [medium]
    0x1416a71f0    55                       push             rbp
    0x1416a71f1    4157                     push             r15
    0x1416a71f3    4156                     push             r14
    0x1416a71f5    4155                     push             r13
    0x1416a71f7    4154                     push             r12
    0x1416a71f9    56                       push             rsi
    0x1416a71fa    57                       push             rdi
    0x1416a71fb    53                       push             rbx
    0x1416a71fc    4881ec18040000           sub              rsp, 0x418
    0x1416a7203    488dac2480000000         lea              rbp, [rsp + 0x80]
    0x1416a720b    48c78590030000feffffff   mov              qword ptr [rbp + 0x390], 0xfffffffffffffffe
    0x1416a7216    4c898578030000           mov              qword ptr [rbp + 0x378], r8
    0x1416a721d    4889d3                   mov              rbx, rdx
    0x1416a7220    4889ce                   mov              rsi, rcx
    0x1416a7223    488b9500040000           mov              rdx, qword ptr [rbp + 0x400]
    0x1416a722a    488b8508040000           mov              rax, qword ptr [rbp + 0x408]
    0x1416a7231    488b4810                 mov              rcx, qword ptr [rax + 0x10]
    0x1416a7235    48898df0010000           mov              qword ptr [rbp + 0x1f0], rcx
    0x1416a723c    0f1000                   movups           xmm0, xmmword ptr [rax]
    0x1416a723f    0f2985e0010000           movaps           xmmword ptr [rbp + 0x1e0], xmm0
    0x1416a7246    8b02                     mov              eax, dword ptr [rdx]
    0x1416a7248    85c0                     test             eax, eax
    0x1416a724a    0f84fb060000             je               0x1416a794b
    0x1416a7250    4d89ce                   mov              r14, r9
    0x1416a7253    488bbde0010000           mov              rdi, qword ptr [rbp + 0x1e0]
    0x1416a725a    4885ff                   test             rdi, rdi
    0x1416a725d    7405                     je               0x1416a7264
    0x1416a725f    83c802                   or               eax, 2
    0x1416a7262    8902                     mov              dword ptr [rdx], eax
    0x1416a7264    4885ff                   test             rdi, rdi
    0x1416a7267    410f95c0                 setne            r8b
    0x1416a726b    488d4d78                 lea              rcx, [rbp + 0x78]
    0x1416a726f    e85cec52ff               call             0x140bd5ed0 -> rhapsody::raw::texture::get_image_create_info
    0x1416a7274    83bd9400000000           cmp              dword ptr [rbp + 0x94], 0
    0x1416a727b    0f84e2060000             je               0x1416a7963
    0x1416a7281    83bd9800000000           cmp              dword ptr [rbp + 0x98], 0
    0x1416a7288    0f84ed060000             je               0x1416a797b
    0x1416a728e    83bd9c00000000           cmp              dword ptr [rbp + 0x9c], 0
    0x1416a7295    0f84f8060000             je               0x1416a7993
    0x1416a729b    488b8b40060000           mov              rcx, qword ptr [rbx + 0x640]
    0x1416a72a2    488d5578                 lea              rdx, [rbp + 0x78]
    0x1416a72a6    4c8d8d70020000           lea              r9, [rbp + 0x270]
    0x1416a72ad    4531c0                   xor              r8d, r8d
    0x1416a72b0    ff93a0010000             call             qword ptr [rbx + 0x1a0]
    0x1416a72b6    85c0                     test             eax, eax
    0x1416a72b8    0f8516070000             jne              0x1416a79d4
    0x1416a72be    4c8ba570020000           mov              r12, qword ptr [rbp + 0x270]
    0x1416a72c5    4889d9                   mov              rcx, rbx
    0x1416a72c8    e8931655ff               call             0x140bf8960 -> rhapsody::raw::device::GraphicsDevice::debug_utils
    0x1416a72cd    4885c0                   test             rax, rax
    0x1416a72d0    0f84a9000000             je               0x1416a737f
    0x1416a72d6    4989c5                   mov              r13, rax
    0x1416a72d9    4d89f0                   mov              r8, r14
    0x1416a72dc    4889b580030000           mov              qword ptr [rbp + 0x380], rsi
    0x1416a72e3    4c89e6                   mov              rsi, r12
    0x1416a72e6    488d8dd0000000           lea              rcx, [rbp + 0xd0]
    0x1416a72ed    488b9578030000           mov              rdx, qword ptr [rbp + 0x378]
    0x1416a72f4    e8d7246dff               call             0x140d797d0 -> _$LT$$RF$str$u20$as$u20$alloc..ffi..c_str..CString..new..SpecNewImpl$GT$::spec_new_impl::h27339bb99b671fc9
    0x1416a72f9    31c0                     xor              eax, eax
    0x1416a72fb    483b85d0000000           cmp              rax, qword ptr [rbp + 0xd0]
    0x1416a7302    0f813a070000             jno              0x1416a7a42
    0x1416a7308    4c8ba5d8000000           mov              r12, qword ptr [rbp + 0xd8]
    0x1416a730f    4c8bbde0000000           mov              r15, qword ptr [rbp + 0xe0]
    0x1416a7316    c7857002000000be9c3b     mov              dword ptr [rbp + 0x270], 0x3b9cbe00
    0x1416a7320    48c7857802000000000000   mov              qword ptr [rbp + 0x278], 0
    0x1416a732b    c785800200000a000000     mov              dword ptr [rbp + 0x280], 0xa
    0x1416a7335    4889b588020000           mov              qword ptr [rbp + 0x288], rsi
    0x1416a733c    4c89a590020000           mov              qword ptr [rbp + 0x290], r12
    0x1416a7343    498b4d40                 mov              rcx, qword ptr [r13 + 0x40]
    0x1416a7347    488d9570020000           lea              rdx, [rbp + 0x270]
    0x1416a734e    41ff5500                 call             qword ptr [r13]
    0x1416a7352    85c0                     test             eax, eax
    0x1416a7354    0f8531070000             jne              0x1416a7a8b
    0x1416a735a    41c6042400               mov              byte ptr [r12], 0
    0x1416a735f    4d85ff                   test             r15, r15
    0x1416a7362    4c89e1                   mov              rcx, r12
    0x1416a7365    4989f4                   mov              r12, rsi
    0x1416a7368    488bb580030000           mov              rsi, qword ptr [rbp + 0x380]
    0x1416a736f    740e                     je               0x1416a737f
    0x1416a7371    41b801000000             mov              r8d, 1
    0x1416a7377    4c89fa                   mov              rdx, r15
    0x1416a737a    e8a1c8f6fe               call             0x140613c20 -> __rust_dealloc
    0x1416a737f    4d89f7                   mov              r15, r14
    0x1416a7382    488b8b40060000           mov              rcx, qword ptr [rbx + 0x640]
    0x1416a7389    4c8d8550030000           lea              r8, [rbp + 0x350]
    0x1416a7390    4c89e2                   mov              rdx, r12
    0x1416a7393    ff93f0000000             call             qword ptr [rbx + 0xf0]
    0x1416a7399    4c8bab78060000           mov              r13, qword ptr [rbx + 0x678]
    0x1416a73a0    4d8d7510                 lea              r14, [r13 + 0x10]
    0x1416a73a4    b101                     mov              cl, 1
    0x1416a73a6    31c0                     xor              eax, eax
    0x1416a73a8    f0410fb04d10             lock cmpxchg     byte ptr [r13 + 0x10], cl
    0x1416a73ae    0f85f7050000             jne              0x1416a79ab
    0x1416a73b4    4c89b568030000           mov              qword ptr [rbp + 0x368], r14
    0x1416a73bb    4c89f1                   mov              rcx, r14
    0x1416a73be    e8ada85dff               call             0x140c81c70 -> parking_lot_core::parking_lot::deadlock_impl::acquire_resource::hb2f16ac28e483830
    0x1416a73c3    4983c518                 add              r13, 0x18
    0x1416a73c7    488b8578030000           mov              rax, qword ptr [rbp + 0x378]
    0x1416a73ce    488985e0000000           mov              qword ptr [rbp + 0xe0], rax
    0x1416a73d5    4d89fe                   mov              r14, r15
    0x1416a73d8    4c89bde8000000           mov              qword ptr [rbp + 0xe8], r15
    0x1416a73df    0f108550030000           movups           xmm0, xmmword ptr [rbp + 0x350]
    0x1416a73e6    0f1185f0000000           movups           xmmword ptr [rbp + 0xf0], xmm0
    0x1416a73ed    488b8560030000           mov              rax, qword ptr [rbp + 0x360]
    0x1416a73f4    48898500010000           mov              qword ptr [rbp + 0x100], rax
    0x1416a73fb    66c785080100000001       mov              word ptr [rbp + 0x108], 0x100
    0x1416a7404    48c785d000000002000000   mov              qword ptr [rbp + 0xd0], 2
    0x1416a740f    488d8d70020000           lea              rcx, [rbp + 0x270]
    0x1416a7416    4c8d85d0000000           lea              r8, [rbp + 0xd0]
    0x1416a741d    4c89ea                   mov              rdx, r13
    0x1416a7420    e81b6656ff               call             0x140c0da40 -> rhapsody::raw::allocator::GpuAllocator::allocate
    0x1416a7425    4889bd68020000           mov              qword ptr [rbp + 0x268], rdi
    0x1416a742c    80bdb402000002           cmp              byte ptr [rbp + 0x2b4], 2
    0x1416a7433    488b8570020000           mov              rax, qword ptr [rbp + 0x270]
    0x1416a743a    48898580030000           mov              qword ptr [rbp + 0x380], rax
    0x1416a7441    4c8bad88020000           mov              r13, qword ptr [rbp + 0x288]
    0x1416a7448    0f859a000000             jne              0x1416a74e8
    0x1416a744e    488b8578020000           mov              rax, qword ptr [rbp + 0x278]
    0x1416a7455    48898570030000           mov              qword ptr [rbp + 0x370], rax
    0x1416a745c    4c8bbd80020000           mov              r15, qword ptr [rbp + 0x280]
    0x1416a7463    0fb6059fd7f801           movzx            eax, byte ptr [rip + 0x1f8d79f] -> 0x143634c09
    0x1416a746a    bf0f000000               mov              edi, 0xf
    0x1416a746f    b90f000000               mov              ecx, 0xf
    0x1416a7474    ba01000000               mov              edx, 1
    0x1416a7479    e892c7f6fe               call             0x140613c10 -> __rust_alloc
    0x1416a747e    4885c0                   test             rax, rax
    0x1416a7481    0f8445060000             je               0x1416a7acc
    0x1416a7487    48b92074657874757265     movabs           rcx, 0x6572757478657420
    0x1416a7491    48894807                 mov              qword ptr [rax + 7], rcx
    0x1416a7495    48b94770754f6e6c7920     movabs           rcx, 0x20796c6e4f757047
    0x1416a749f    488908                   mov              qword ptr [rax], rcx
    0x1416a74a2    488b8d60030000           mov              rcx, qword ptr [rbp + 0x360]
    0x1416a74a9    48898da0010000           mov              qword ptr [rbp + 0x1a0], rcx
    0x1416a74b0    0f108550030000           movups           xmm0, xmmword ptr [rbp + 0x350]
    0x1416a74b7    0f298590010000           movaps           xmmword ptr [rbp + 0x190], xmm0
    0x1416a74be    488b8d80030000           mov              rcx, qword ptr [rbp + 0x380]
    0x1416a74c5    4883f90b                 cmp              rcx, 0xb
    0x1416a74c9    0f8594030000             jne              0x1416a7863
    0x1416a74cf    4c89e9                   mov              rcx, r13
    0x1416a74d2    488b9570030000           mov              rdx, qword ptr [rbp + 0x370]
    0x1416a74d9    48899580030000           mov              qword ptr [rbp + 0x380], rdx
    0x1416a74e0    41bd0f000000             mov              r13d, 0xf
    0x1416a74e6    eb38                     jmp              0x1416a7520
    0x1416a74e8    4c8bbd78020000           mov              r15, qword ptr [rbp + 0x278]
    0x1416a74ef    488b8d80020000           mov              rcx, qword ptr [rbp + 0x280]
    0x1416a74f6    488b8590020000           mov              rax, qword ptr [rbp + 0x290]
    0x1416a74fd    488bbd98020000           mov              rdi, qword ptr [rbp + 0x298]
    0x1416a7504    0f1085a0020000           movups           xmm0, xmmword ptr [rbp + 0x2a0]
    0x1416a750b    0f298590010000           movaps           xmmword ptr [rbp + 0x190], xmm0
    0x1416a7512    488b95b0020000           mov              rdx, qword ptr [rbp + 0x2b0]
    0x1416a7519    488995a0010000           mov              qword ptr [rbp + 0x1a0], rdx
    0x1416a7520    488b95a0010000           mov              rdx, qword ptr [rbp + 0x1a0]
    0x1416a7527    48899510020000           mov              qword ptr [rbp + 0x210], rdx
    0x1416a752e    0f288590010000           movaps           xmm0, xmmword ptr [rbp + 0x190]
    0x1416a7535    0f298500020000           movaps           xmmword ptr [rbp + 0x200], xmm0
    0x1416a753c    4c8b8580030000           mov              r8, qword ptr [rbp + 0x380]
    0x1416a7543    4c898520020000           mov              qword ptr [rbp + 0x220], r8
    0x1416a754a    4c89bd28020000           mov              qword ptr [rbp + 0x228], r15
    0x1416a7551    48898d30020000           mov              qword ptr [rbp + 0x230], rcx
    0x1416a7558    0f118550020000           movups           xmmword ptr [rbp + 0x250], xmm0
    0x1416a755f    48899560020000           mov              qword ptr [rbp + 0x260], rdx
    0x1416a7566    4c89ad38020000           mov              qword ptr [rbp + 0x238], r13
    0x1416a756d    48898540020000           mov              qword ptr [rbp + 0x240], rax
    0x1416a7574    4889bd48020000           mov              qword ptr [rbp + 0x248], rdi
    0x1416a757b    c6858f03000001           mov              byte ptr [rbp + 0x38f], 1
    0x1416a7582    488bbd68030000           mov              rdi, qword ptr [rbp + 0x368]
    0x1416a7589    4889f9                   mov              rcx, rdi
    0x1416a758c    e87fa85dff               call             0x140c81e10 -> parking_lot_core::parking_lot::deadlock_impl::release_resource::h1c3ab4db915ad6f2
    0x1416a7591    31c9                     xor              ecx, ecx
    0x1416a7593    b001                     mov              al, 1
    0x1416a7595    f00fb00f                 lock cmpxchg     byte ptr [rdi], cl
    0x1416a7599    0f851f040000             jne              0x1416a79be
    0x1416a759f    c6858f03000001           mov              byte ptr [rbp + 0x38f], 1
    0x1416a75a6    488d8d20020000           lea              rcx, [rbp + 0x220]
    0x1416a75ad    e8fe9602ff               call             0x1406d0cb0 -> gpu_allocator::vulkan::Allocation::memory::hfb1e6a660a853183
    0x1416a75b2    4989c5                   mov              r13, rax
    0x1416a75b5    c6858f03000001           mov              byte ptr [rbp + 0x38f], 1
    0x1416a75bc    488d8d20020000           lea              rcx, [rbp + 0x220]
    0x1416a75c3    e8e8f2fdfe               call             0x1406868b0 -> rhapsody::raw::buffer::impl$3::byte_count
    0x1416a75c8    488b8b40060000           mov              rcx, qword ptr [rbx + 0x640]
    0x1416a75cf    4c89e2                   mov              rdx, r12
    0x1416a75d2    4d89e8                   mov              r8, r13
    0x1416a75d5    4989c1                   mov              r9, rax
    0x1416a75d8    ff93e0000000             call             qword ptr [rbx + 0xe0]
    0x1416a75de    85c0                     test             eax, eax
    0x1416a75e0    488b9500040000           mov              rdx, qword ptr [rbp + 0x400]
    0x1416a75e7    0f8518040000             jne              0x1416a7a05
    0x1416a75ed    488d4378                 lea              rax, [rbx + 0x78]
    0x1416a75f1    4c8bab48060000           mov              r13, qword ptr [rbx + 0x648]
    0x1416a75f8    488b8d60020000           mov              rcx, qword ptr [rbp + 0x260]
    0x1416a75ff    48898dd0010000           mov              qword ptr [rbp + 0x1d0], rcx
    0x1416a7606    0f108520020000           movups           xmm0, xmmword ptr [rbp + 0x220]
    0x1416a760d    0f108d30020000           movups           xmm1, xmmword ptr [rbp + 0x230]
    0x1416a7614    0f109540020000           movups           xmm2, xmmword ptr [rbp + 0x240]
    0x1416a761b    0f109d50020000           movups           xmm3, xmmword ptr [rbp + 0x250]
    0x1416a7622    0f299dc0010000           movaps           xmmword ptr [rbp + 0x1c0], xmm3
    0x1416a7629    0f2995b0010000           movaps           xmmword ptr [rbp + 0x1b0], xmm2
    0x1416a7630    0f298da0010000           movaps           xmmword ptr [rbp + 0x1a0], xmm1
    0x1416a7637    0f298590010000           movaps           xmmword ptr [rbp + 0x190], xmm0
    0x1416a763e    0f1002                   movups           xmm0, xmmword ptr [rdx]
    0x1416a7641    0f104a10                 movups           xmm1, xmmword ptr [rdx + 0x10]
    0x1416a7645    0f298d10020000           movaps           xmmword ptr [rbp + 0x210], xmm1
    0x1416a764c    0f298500020000           movaps           xmmword ptr [rbp + 0x200], xmm0
    0x1416a7653    c6858f03000000           mov              byte ptr [rbp + 0x38f], 0
    0x1416a765a    488d8d00020000           lea              rcx, [rbp + 0x200]
    0x1416a7661    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x1416a7666    488d8d90010000           lea              rcx, [rbp + 0x190]
    0x1416a766d    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x1416a7672    4889442420               mov              qword ptr [rsp + 0x20], rax
    0x1416a7677    488d8d70020000           lea              rcx, [rbp + 0x270]
    0x1416a767e    488b9578030000           mov              rdx, qword ptr [rbp + 0x378]
    0x1416a7685    4d89f0                   mov              r8, r14
    0x1416a7688    4d89e1                   mov              r9, r12
    0x1416a768b    e8404c53ff               call             0x140bdc2d0 -> rhapsody::raw::managed::texture::ManagedTexture::new
    0x1416a7690    0fb6bd2c030000           movzx            edi, byte ptr [rbp + 0x32c]
    0x1416a7697    4080ff03                 cmp              dil, 3
    0x1416a769b    755e                     jne              0x1416a76fb
    0x1416a769d    0f1085b0020000           movups           xmm0, xmmword ptr [rbp + 0x2b0]
    0x1416a76a4    0f298510010000           movaps           xmmword ptr [rbp + 0x110], xmm0
    0x1416a76ab    0f108d70020000           movups           xmm1, xmmword ptr [rbp + 0x270]
    0x1416a76b2    0f109580020000           movups           xmm2, xmmword ptr [rbp + 0x280]
    0x1416a76b9    0f109d90020000           movups           xmm3, xmmword ptr [rbp + 0x290]
    0x1416a76c0    0f10a5a0020000           movups           xmm4, xmmword ptr [rbp + 0x2a0]
    0x1416a76c7    0f29a500010000           movaps           xmmword ptr [rbp + 0x100], xmm4
    0x1416a76ce    0f299df0000000           movaps           xmmword ptr [rbp + 0xf0], xmm3
    0x1416a76d5    0f2995e0000000           movaps           xmmword ptr [rbp + 0xe0], xmm2
    0x1416a76dc    0f298dd0000000           movaps           xmmword ptr [rbp + 0xd0], xmm1
    0x1416a76e3    0f114640                 movups           xmmword ptr [rsi + 0x40], xmm0
    0x1416a76e7    0f116630                 movups           xmmword ptr [rsi + 0x30], xmm4
    0x1416a76eb    0f115e20                 movups           xmmword ptr [rsi + 0x20], xmm3
    0x1416a76ef    0f115610                 movups           xmmword ptr [rsi + 0x10], xmm2
    0x1416a76f3    0f110e                   movups           xmmword ptr [rsi], xmm1
    0x1416a76f6    e939020000               jmp              0x1416a7934
    0x1416a76fb    4c8db5d0000000           lea              r14, [rbp + 0xd0]
    0x1416a7702    488d9570020000           lea              rdx, [rbp + 0x270]
    0x1416a7709    41b8bc000000             mov              r8d, 0xbc
    0x1416a770f    4c89f1                   mov              rcx, r14
    0x1416a7712    e8e9c42701               call             0x142923c00 -> memcpy
    0x1416a7717    0fb7852d030000           movzx            eax, word ptr [rbp + 0x32d]
    0x1416a771e    66894575                 mov              word ptr [rbp + 0x75], ax
    0x1416a7722    0fb6852f030000           movzx            eax, byte ptr [rbp + 0x32f]
    0x1416a7729    884577                   mov              byte ptr [rbp + 0x77], al
    0x1416a772c    4c8d7db8                 lea              r15, [rbp - 0x48]
    0x1416a7730    41b8bc000000             mov              r8d, 0xbc
    0x1416a7736    4c89f9                   mov              rcx, r15
    0x1416a7739    4c89f2                   mov              rdx, r14
    0x1416a773c    e8bfc42701               call             0x142923c00 -> memcpy
    0x1416a7741    40887d74                 mov              byte ptr [rbp + 0x74], dil
    0x1416a7745    c6858f03000000           mov              byte ptr [rbp + 0x38f], 0
    0x1416a774c    4c89e9                   mov              rcx, r13
    0x1416a774f    4c89fa                   mov              rdx, r15
    0x1416a7752    e8f94553ff               call             0x140bdbd50 -> rhapsody::raw::managed::resources::impl$6::allocate
    0x1416a7757    8b480c                   mov              ecx, dword ptr [rax + 0xc]
    0x1416a775a    410fb795500a0000         movzx            edx, word ptr [r13 + 0xa50]
    0x1416a7762    4d8b4500                 mov              r8, qword ptr [r13]
    0x1416a7766    4c898538030000           mov              qword ptr [rbp + 0x338], r8
    0x1416a776d    4989c0                   mov              r8, rax
    0x1416a7770    4983c010                 add              r8, 0x10
    0x1416a7774    48898540030000           mov              qword ptr [rbp + 0x340], rax
    0x1416a777b    898d48030000             mov              dword ptr [rbp + 0x348], ecx
    0x1416a7781    6689954c030000           mov              word ptr [rbp + 0x34c], dx
    0x1416a7788    c6854e03000000           mov              byte ptr [rbp + 0x34e], 0
    0x1416a778f    4c898530030000           mov              qword ptr [rbp + 0x330], r8
    0x1416a7796    4883bd6802000000         cmp              qword ptr [rbp + 0x268], 0
    0x1416a779e    488bbd00040000           mov              rdi, qword ptr [rbp + 0x400]
    0x1416a77a5    744f                     je               0x1416a77f6
    0x1416a77a7    0f1007                   movups           xmm0, xmmword ptr [rdi]
    0x1416a77aa    0f104f10                 movups           xmm1, xmmword ptr [rdi + 0x10]
    0x1416a77ae    0f298de0000000           movaps           xmmword ptr [rbp + 0xe0], xmm1
    0x1416a77b5    0f2985d0000000           movaps           xmmword ptr [rbp + 0xd0], xmm0
    0x1416a77bc    488d8530030000           lea              rax, [rbp + 0x330]
    0x1416a77c3    4889442420               mov              qword ptr [rsp + 0x20], rax
    0x1416a77c8    488d8d70020000           lea              rcx, [rbp + 0x270]
    0x1416a77cf    4c8d85d0000000           lea              r8, [rbp + 0xd0]
    0x1416a77d6    4c8d8de0010000           lea              r9, [rbp + 0x1e0]
    0x1416a77dd    4889da                   mov              rdx, rbx
    0x1416a77e0    e88b6a55ff               call             0x140bfe270 -> rhapsody::raw::device::GraphicsDevice::init_texture_data
    0x1416a77e5    488b8570020000           mov              rax, qword ptr [rbp + 0x270]
    0x1416a77ec    4883f80b                 cmp              rax, 0xb
    0x1416a77f0    0f85f1000000             jne              0x1416a78e7
    0x1416a77f6    488b8548030000           mov              rax, qword ptr [rbp + 0x348]
    0x1416a77fd    48898580020000           mov              qword ptr [rbp + 0x280], rax
    0x1416a7804    0f108538030000           movups           xmm0, xmmword ptr [rbp + 0x338]
    0x1416a780b    0f298570020000           movaps           xmmword ptr [rbp + 0x270], xmm0
    0x1416a7812    0f100f                   movups           xmm1, xmmword ptr [rdi]
    0x1416a7815    0f105710                 movups           xmm2, xmmword ptr [rdi + 0x10]
    0x1416a7819    0f119598020000           movups           xmmword ptr [rbp + 0x298], xmm2
    0x1416a7820    0f118d88020000           movups           xmmword ptr [rbp + 0x288], xmm1
    0x1416a7827    0f114608                 movups           xmmword ptr [rsi + 8], xmm0
    0x1416a782b    488b85a0020000           mov              rax, qword ptr [rbp + 0x2a0]
    0x1416a7832    48894638                 mov              qword ptr [rsi + 0x38], rax
    0x1416a7836    0f288590020000           movaps           xmm0, xmmword ptr [rbp + 0x290]
    0x1416a783d    0f114628                 movups           xmmword ptr [rsi + 0x28], xmm0
    0x1416a7841    488b8580020000           mov              rax, qword ptr [rbp + 0x280]
    0x1416a7848    48894618                 mov              qword ptr [rsi + 0x18], rax
    0x1416a784c    488b8588020000           mov              rax, qword ptr [rbp + 0x288]
    0x1416a7853    48894620                 mov              qword ptr [rsi + 0x20], rax
    0x1416a7857    48c7060b000000           mov              qword ptr [rsi], 0xb
    0x1416a785e    e9d1000000               jmp              0x1416a7934
    0x1416a7863    4889ca                   mov              rdx, rcx
    0x1416a7866    488b8d60030000           mov              rcx, qword ptr [rbp + 0x360]
    0x1416a786d    48898d10020000           mov              qword ptr [rbp + 0x210], rcx
    0x1416a7874    0f108550030000           movups           xmm0, xmmword ptr [rbp + 0x350]
    0x1416a787b    0f298500020000           movaps           xmmword ptr [rbp + 0x200], xmm0
    0x1416a7882    488b8d70030000           mov              rcx, qword ptr [rbp + 0x370]
    0x1416a7889    48894e08                 mov              qword ptr [rsi + 8], rcx
    0x1416a788d    4c897e10                 mov              qword ptr [rsi + 0x10], r15
    0x1416a7891    4c896e18                 mov              qword ptr [rsi + 0x18], r13
    0x1416a7895    0f108550030000           movups           xmm0, xmmword ptr [rbp + 0x350]
    0x1416a789c    0f114638                 movups           xmmword ptr [rsi + 0x38], xmm0
    0x1416a78a0    488b8d60030000           mov              rcx, qword ptr [rbp + 0x360]
    0x1416a78a7    48894e48                 mov              qword ptr [rsi + 0x48], rcx
    0x1416a78ab    488916                   mov              qword ptr [rsi], rdx
    0x1416a78ae    48c746200f000000         mov              qword ptr [rsi + 0x20], 0xf
    0x1416a78b6    48894628                 mov              qword ptr [rsi + 0x28], rax
    0x1416a78ba    48c746300f000000         mov              qword ptr [rsi + 0x30], 0xf
    0x1416a78c2    488bbd68030000           mov              rdi, qword ptr [rbp + 0x368]
    0x1416a78c9    4889f9                   mov              rcx, rdi
    0x1416a78cc    e83fa55dff               call             0x140c81e10 -> parking_lot_core::parking_lot::deadlock_impl::release_resource::h1c3ab4db915ad6f2
    0x1416a78d1    31c9                     xor              ecx, ecx
    0x1416a78d3    b001                     mov              al, 1
    0x1416a78d5    f00fb00f                 lock cmpxchg     byte ptr [rdi], cl
    0x1416a78d9    7459                     je               0x1416a7934
    0x1416a78db    4889f9                   mov              rcx, rdi
    0x1416a78de    31d2                     xor              edx, edx
    0x1416a78e0    e8ab622201               call             0x1428cdb90 -> parking_lot::raw_mutex::RawMutex::unlock_slow::h56cb6df3e32a2657
    0x1416a78e5    eb4d                     jmp              0x1416a7934
    0x1416a78e7    488b8db8020000           mov              rcx, qword ptr [rbp + 0x2b8]
    0x1416a78ee    48894e48                 mov              qword ptr [rsi + 0x48], rcx
    0x1416a78f2    0f108578020000           movups           xmm0, xmmword ptr [rbp + 0x278]
    0x1416a78f9    0f108d88020000           movups           xmm1, xmmword ptr [rbp + 0x288]
    0x1416a7900    0f109598020000           movups           xmm2, xmmword ptr [rbp + 0x298]
    0x1416a7907    0f109da8020000           movups           xmm3, xmmword ptr [rbp + 0x2a8]
    0x1416a790e    0f115e38                 movups           xmmword ptr [rsi + 0x38], xmm3
    0x1416a7912    0f115628                 movups           xmmword ptr [rsi + 0x28], xmm2
    0x1416a7916    0f114e18                 movups           xmmword ptr [rsi + 0x18], xmm1
    0x1416a791a    0f114608                 movups           xmmword ptr [rsi + 8], xmm0
    0x1416a791e    488906                   mov              qword ptr [rsi], rax
    0x1416a7921    c6858f03000000           mov              byte ptr [rbp + 0x38f], 0
    0x1416a7928    488d8d38030000           lea              rcx, [rbp + 0x338]
    0x1416a792f    e87cab53ff               call             0x140be24b0 -> rhapsody::raw::managed::handle::impl$6::drop
    0x1416a7934    4889f0                   mov              rax, rsi
    0x1416a7937    4881c418040000           add              rsp, 0x418
    0x1416a793e    5b                       pop              rbx
    0x1416a793f    5f                       pop              rdi
    0x1416a7940    5e                       pop              rsi
    0x1416a7941    415c                     pop              r12
    0x1416a7943    415d                     pop              r13
    0x1416a7945    415e                     pop              r14
    0x1416a7947    415f                     pop              r15
    0x1416a7949    5d                       pop              rbp
    0x1416a794a    c3                       ret              
    0x1416a794b    488d0d88145e01           lea              rcx, [rip + 0x15e1488] -> 0x142c88dda
    0x1416a7952    4c8d05bf145e01           lea              r8, [rip + 0x15e14bf] -> 0x142c88e18
    0x1416a7959    ba3c000000               mov              edx, 0x3c
    0x1416a795e    e8cd1a2301               call             0x1428d9430 -> core::panicking::panic::hb210ae7b817a9f13
    0x1416a7963    488d0dc6145e01           lea              rcx, [rip + 0x15e14c6] -> 0x142c88e30
    0x1416a796a    4c8d05ef145e01           lea              r8, [rip + 0x15e14ef] -> 0x142c88e60
    0x1416a7971    ba2e000000               mov              edx, 0x2e
    0x1416a7976    e8b51a2301               call             0x1428d9430 -> core::panicking::panic::hb210ae7b817a9f13
    0x1416a797b    488d0df6145e01           lea              rcx, [rip + 0x15e14f6] -> 0x142c88e78
    0x1416a7982    4c8d051f155e01           lea              r8, [rip + 0x15e151f] -> 0x142c88ea8
    0x1416a7989    ba2f000000               mov              edx, 0x2f
    0x1416a798e    e89d1a2301               call             0x1428d9430 -> core::panicking::panic::hb210ae7b817a9f13
    0x1416a7993    488d0d26155e01           lea              rcx, [rip + 0x15e1526] -> 0x142c88ec0
    0x1416a799a    4c8d054f155e01           lea              r8, [rip + 0x15e154f] -> 0x142c88ef0
    0x1416a79a1    ba2e000000               mov              edx, 0x2e
    0x1416a79a6    e8851a2301               call             0x1428d9430 -> core::panicking::panic::hb210ae7b817a9f13
    0x1416a79ab    4c89f1                   mov              rcx, r14
    0x1416a79ae    41b800ca9a3b             mov              r8d, 0x3b9aca00
    0x1416a79b4    e857582201               call             0x1428cd210 -> parking_lot::raw_mutex::RawMutex::lock_slow::h7c244b522f551b38
    0x1416a79b9    e9f6f9ffff               jmp              0x1416a73b4
    0x1416a79be    c6858f03000001           mov              byte ptr [rbp + 0x38f], 1
    0x1416a79c5    4889f9                   mov              rcx, rdi
    0x1416a79c8    31d2                     xor              edx, edx
    0x1416a79ca    e8c1612201               call             0x1428cdb90 -> parking_lot::raw_mutex::RawMutex::unlock_slow::h56cb6df3e32a2657
    0x1416a79cf    e9cbfbffff               jmp              0x1416a759f
    0x1416a79d4    898570020000             mov              dword ptr [rbp + 0x270], eax
    0x1416a79da    488d0537155e01           lea              rax, [rip + 0x15e1537] -> 0x142c88f18
    0x1416a79e1    4889442420               mov              qword ptr [rsp + 0x20], rax
    0x1416a79e6    488d0d1b155e01           lea              rcx, [rip + 0x15e151b] -> 0x142c88f08
    0x1416a79ed    4c8d0dd40c5e01           lea              r9, [rip + 0x15e0cd4] -> 0x142c886c8
    0x1416a79f4    4c8d8570020000           lea              r8, [rbp + 0x270]
    0x1416a79fb    ba0c000000               mov              edx, 0xc
    0x1416a7a00    e85b1d2301               call             0x1428d9760 -> core::result::unwrap_failed::h96d74ae09566f4b3
    0x1416a7a05    898570020000             mov              dword ptr [rbp + 0x270], eax
    0x1416a7a0b    c6858f03000001           mov              byte ptr [rbp + 0x38f], 1
    0x1416a7a12    488d057f155e01           lea              rax, [rip + 0x15e157f] -> 0x142c88f98
    0x1416a7a19    4889442420               mov              qword ptr [rsp + 0x20], rax
    0x1416a7a1e    488d0d5b155e01           lea              rcx, [rip + 0x15e155b] -> 0x142c88f80
    0x1416a7a25    4c8d0d9c0c5e01           lea              r9, [rip + 0x15e0c9c] -> 0x142c886c8
    0x1416a7a2c    4c8d8570020000           lea              r8, [rbp + 0x270]
    0x1416a7a33    ba11000000               mov              edx, 0x11
    0x1416a7a38    e8231d2301               call             0x1428d9760 -> core::result::unwrap_failed::h96d74ae09566f4b3
    0x1416a7a3d    e9a7000000               jmp              0x1416a7ae9
    0x1416a7a42    0f1085d0000000           movups           xmm0, xmmword ptr [rbp + 0xd0]
    0x1416a7a49    0f108de0000000           movups           xmm1, xmmword ptr [rbp + 0xe0]
    0x1416a7a50    0f298d80020000           movaps           xmmword ptr [rbp + 0x280], xmm1
    0x1416a7a57    0f298570020000           movaps           xmmword ptr [rbp + 0x270], xmm0
    0x1416a7a5e    488d05cb145e01           lea              rax, [rip + 0x15e14cb] -> 0x142c88f30
    0x1416a7a65    4889442420               mov              qword ptr [rsp + 0x20], rax
    0x1416a7a6a    488d0d970c5e01           lea              rcx, [rip + 0x15e0c97] -> 0x142c88708
    0x1416a7a71    4c8d0d700c5e01           lea              r9, [rip + 0x15e0c70] -> 0x142c886e8
    0x1416a7a78    4c8d8570020000           lea              r8, [rbp + 0x270]
    0x1416a7a7f    ba2b000000               mov              edx, 0x2b
    0x1416a7a84    e8d71c2301               call             0x1428d9760 -> core::result::unwrap_failed::h96d74ae09566f4b3
    0x1416a7a89    eb5e                     jmp              0x1416a7ae9
    0x1416a7a8b    4c89a580030000           mov              qword ptr [rbp + 0x380], r12
    0x1416a7a92    4c89bd78030000           mov              qword ptr [rbp + 0x378], r15
    0x1416a7a99    8985d0000000             mov              dword ptr [rbp + 0xd0], eax
    0x1416a7a9f    488d05c2145e01           lea              rax, [rip + 0x15e14c2] -> 0x142c88f68
    0x1416a7aa6    4889442420               mov              qword ptr [rsp + 0x20], rax
    0x1416a7aab    488d0d96145e01           lea              rcx, [rip + 0x15e1496] -> 0x142c88f48
    0x1416a7ab2    4c8d0d0f0c5e01           lea              r9, [rip + 0x15e0c0f] -> 0x142c886c8
    0x1416a7ab9    4c8d85d0000000           lea              r8, [rbp + 0xd0]
    0x1416a7ac0    ba1b000000               mov              edx, 0x1b
    0x1416a7ac5    e8961c2301               call             0x1428d9760 -> core::result::unwrap_failed::h96d74ae09566f4b3
    0x1416a7aca    eb1d                     jmp              0x1416a7ae9
    0x1416a7acc    4c89bd78030000           mov              qword ptr [rbp + 0x378], r15
    0x1416a7ad3    4c8d05a60c5e01           lea              r8, [rip + 0x15e0ca6] -> 0x142c88780
    0x1416a7ada    b901000000               mov              ecx, 1
    0x1416a7adf    ba0f000000               mov              edx, 0xf
    0x1416a7ae4    e8fa142301               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x1416a7ae9    0f0b                     ud2              
    0x1416a7aeb    0f1f440000               nop              dword ptr [rax + rax]
    0x1416a7af0    4889542410               mov              qword ptr [rsp + 0x10], rdx
    0x1416a7af5    55                       push             rbp
    0x1416a7af6    4157                     push             r15
    0x1416a7af8    4156                     push             r14
    0x1416a7afa    4155                     push             r13
    0x1416a7afc    4154                     push             r12
    0x1416a7afe    56                       push             rsi
    0x1416a7aff    57                       push             rdi
    0x1416a7b00    53                       push             rbx
    0x1416a7b01    4883ec38                 sub              rsp, 0x38
    0x1416a7b05    488daa80000000           lea              rbp, [rdx + 0x80]
    0x1416a7b0c    488b9570020000           mov              rdx, qword ptr [rbp + 0x270]
    0x1416a7b13    4885d2                   test             rdx, rdx
    0x1416a7b16    7412                     je               0x1416a7b2a
    0x1416a7b18    488b8d78020000           mov              rcx, qword ptr [rbp + 0x278]
    0x1416a7b1f    41b801000000             mov              r8d, 1
    0x1416a7b25    e8f6c0f6fe               call             0x140613c20 -> __rust_dealloc
    0x1416a7b2a    90                       nop              
    0x1416a7b2b    4883c438                 add              rsp, 0x38
    0x1416a7b2f    5b                       pop              rbx
    0x1416a7b30    5f                       pop              rdi
    0x1416a7b31    5e                       pop              rsi
    0x1416a7b32    415c                     pop              r12
    0x1416a7b34    415d                     pop              r13
    0x1416a7b36    415e                     pop              r14
    0x1416a7b38    415f                     pop              r15
    0x1416a7b3a    5d                       pop              rbp
    0x1416a7b3b    c3                       ret              
    0x1416a7b3c    0f1f4000                 nop              dword ptr [rax]
    0x1416a7b40    4889542410               mov              qword ptr [rsp + 0x10], rdx
    0x1416a7b45    55                       push             rbp
    0x1416a7b46    4157                     push             r15
    0x1416a7b48    4156                     push             r14
    0x1416a7b4a    4155                     push             r13
    0x1416a7b4c    4154                     push             r12
    0x1416a7b4e    56                       push             rsi
    0x1416a7b4f    57                       push             rdi
    0x1416a7b50    53                       push             rbx
    0x1416a7b51    4883ec38                 sub              rsp, 0x38
    0x1416a7b55    488daa80000000           lea              rbp, [rdx + 0x80]
    0x1416a7b5c    488b8580030000           mov              rax, qword ptr [rbp + 0x380]
    0x1416a7b63    c60000                   mov              byte ptr [rax], 0
    0x1416a7b66    4883bd7803000000         cmp              qword ptr [rbp + 0x378], 0
    0x1416a7b6e    7419                     je               0x1416a7b89
    0x1416a7b70    41b801000000             mov              r8d, 1
    0x1416a7b76    488b8d80030000           mov              rcx, qword ptr [rbp + 0x380]
    0x1416a7b7d    488b9578030000           mov              rdx, qword ptr [rbp + 0x378]
    0x1416a7b84    e897c0f6fe               call             0x140613c20 -> __rust_dealloc
    0x1416a7b89    90                       nop              
    0x1416a7b8a    4883c438                 add              rsp, 0x38
    0x1416a7b8e    5b                       pop              rbx
    0x1416a7b8f    5f                       pop              rdi
    0x1416a7b90    5e                       pop              rsi
    0x1416a7b91    415c                     pop              r12
    0x1416a7b93    415d                     pop              r13
    0x1416a7b95    415e                     pop              r14
    0x1416a7b97    415f                     pop              r15
    0x1416a7b99    5d                       pop              rbp
    0x1416a7b9a    c3                       ret              
    0x1416a7b9b    0f1f440000               nop              dword ptr [rax + rax]
    0x1416a7ba0    4889542410               mov              qword ptr [rsp + 0x10], rdx
    0x1416a7ba5    55                       push             rbp
    0x1416a7ba6    4157                     push             r15
    0x1416a7ba8    4156                     push             r14
    0x1416a7baa    4155                     push             r13
    0x1416a7bac    4154                     push             r12
    0x1416a7bae    56                       push             rsi
    0x1416a7baf    57                       push             rdi
    0x1416a7bb0    53                       push             rbx
    0x1416a7bb1    4883ec38                 sub              rsp, 0x38
    0x1416a7bb5    488daa80000000           lea              rbp, [rdx + 0x80]
    0x1416a7bbc    4883bd8003000005         cmp              qword ptr [rbp + 0x380], 5
    0x1416a7bc4    7735                     ja               0x1416a7bfb
    0x1416a7bc6    b832000000               mov              eax, 0x32
    0x1416a7bcb    488b8d80030000           mov              rcx, qword ptr [rbp + 0x380]
    0x1416a7bd2    480fa3c8                 bt               rax, rcx
    0x1416a7bd6    7323                     jae              0x1416a7bfb
    0x1416a7bd8    4883bd7003000000         cmp              qword ptr [rbp + 0x370], 0
    0x1416a7be0    7419                     je               0x1416a7bfb
    0x1416a7be2    41b801000000             mov              r8d, 1
    0x1416a7be8    488b8d78030000           mov              rcx, qword ptr [rbp + 0x378]
    0x1416a7bef    488b9570030000           mov              rdx, qword ptr [rbp + 0x370]
    0x1416a7bf6    e825c0f6fe               call             0x140613c20 -> __rust_dealloc
    0x1416a7bfb    90                       nop              
    0x1416a7bfc    4883c438                 add              rsp, 0x38
    0x1416a7c00    5b                       pop              rbx
    0x1416a7c01    5f                       pop              rdi
    0x1416a7c02    5e                       pop              rsi
    0x1416a7c03    415c                     pop              r12
    0x1416a7c05    415d                     pop              r13
    0x1416a7c07    415e                     pop              r14
    0x1416a7c09    415f                     pop              r15
    0x1416a7c0b    5d                       pop              rbp
    0x1416a7c0c    c3                       ret              
    0x1416a7c0d    0f1f00                   nop              dword ptr [rax]
    0x1416a7c10    4889542410               mov              qword ptr [rsp + 0x10], rdx
    0x1416a7c15    55                       push             rbp
    0x1416a7c16    4157                     push             r15
    0x1416a7c18    4156                     push             r14
    0x1416a7c1a    4155                     push             r13
    0x1416a7c1c    4154                     push             r12
    0x1416a7c1e    56                       push             rsi
    0x1416a7c1f    57                       push             rdi
    0x1416a7c20    53                       push             rbx
    0x1416a7c21    4883ec38                 sub              rsp, 0x38
    0x1416a7c25    488daa80000000           lea              rbp, [rdx + 0x80]
    0x1416a7c2c    488d8d38030000           lea              rcx, [rbp + 0x338]
    0x1416a7c33    e878a853ff               call             0x140be24b0 -> rhapsody::raw::managed::handle::impl$6::drop
    0x1416a7c38    c6858f03000000           mov              byte ptr [rbp + 0x38f], 0
    0x1416a7c3f    4883c438                 add              rsp, 0x38
    0x1416a7c43    5b                       pop              rbx
    0x1416a7c44    5f                       pop              rdi
    0x1416a7c45    5e                       pop              rsi
    0x1416a7c46    415c                     pop              r12
    0x1416a7c48    415d                     pop              r13
    0x1416a7c4a    415e                     pop              r14
    0x1416a7c4c    415f                     pop              r15
    0x1416a7c4e    5d                       pop              rbp
    0x1416a7c4f    c3                       ret              
    0x1416a7c50    4889542410               mov              qword ptr [rsp + 0x10], rdx
    0x1416a7c55    55                       push             rbp
    0x1416a7c56    4157                     push             r15
    0x1416a7c58    4156                     push             r14
    0x1416a7c5a    4155                     push             r13
    0x1416a7c5c    4154                     push             r12
    0x1416a7c5e    56                       push             rsi
    0x1416a7c5f    57                       push             rdi
    0x1416a7c60    53                       push             rbx
    0x1416a7c61    4883ec38                 sub              rsp, 0x38
    0x1416a7c65    488daa80000000           lea              rbp, [rdx + 0x80]
    0x1416a7c6c    80bd8f03000000           cmp              byte ptr [rbp + 0x38f], 0
    0x1416a7c73    7511                     jne              0x1416a7c86
    0x1416a7c75    4883c438                 add              rsp, 0x38
    0x1416a7c79    5b                       pop              rbx
    0x1416a7c7a    5f                       pop              rdi
    0x1416a7c7b    5e                       pop              rsi
    0x1416a7c7c    415c                     pop              r12
    0x1416a7c7e    415d                     pop              r13
    0x1416a7c80    415e                     pop              r14
    0x1416a7c82    415f                     pop              r15
    0x1416a7c84    5d                       pop              rbp
    0x1416a7c85    c3                       ret              
    0x1416a7c86    488d8d20020000           lea              rcx, [rbp + 0x220]
    0x1416a7c8d    e84e101eff               call             0x140888ce0 -> core::ptr::drop_in_place$LT$gpu_allocator..vulkan..Allocation$GT$::h0890c49d249fe888
    0x1416a7c92    ebe1                     jmp              0x1416a7c75
    0x1416a7c94    6666662e0f1f840000000000 nop              word ptr cs:[rax + rax]
    0x1416a7ca0    4889542410               mov              qword ptr [rsp + 0x10], rdx
    0x1416a7ca5    55                       push             rbp
    0x1416a7ca6    4157                     push             r15
    0x1416a7ca8    4156                     push             r14
    0x1416a7caa    4155                     push             r13
    0x1416a7cac    4154                     push             r12
    0x1416a7cae    56                       push             rsi
    0x1416a7caf    57                       push             rdi
    0x1416a7cb0    53                       push             rbx
    0x1416a7cb1    4883ec38                 sub              rsp, 0x38
    0x1416a7cb5    488daa80000000           lea              rbp, [rdx + 0x80]
    0x1416a7cbc    488b8d68030000           mov              rcx, qword ptr [rbp + 0x368]
    0x1416a7cc3    e8689fabfe               call             0x140161c30 -> core::ptr::drop_in_place$LT$lock_api..mutex..MutexGuard$LT$parking_lot..raw_mutex..RawMutex$C$alloc..vec..Vec$LT$glam..f32..vec2..Vec2$GT$$GT$$GT$::hc1588d55d4486b0e (.llvm.14454403653567781691)
    0x1416a7cc8    90                       nop              
    0x1416a7cc9    4883c438                 add              rsp, 0x38
    0x1416a7ccd    5b                       pop              rbx
    0x1416a7cce    5f                       pop              rdi
    0x1416a7ccf    5e                       pop              rsi
    0x1416a7cd0    415c                     pop              r12
    0x1416a7cd2    415d                     pop              r13
    0x1416a7cd4    415e                     pop              r14
    0x1416a7cd6    415f                     pop              r15
    0x1416a7cd8    5d                       pop              rbp
    0x1416a7cd9    c3                       ret              

=== 0x1417faa80 size=2299  rhapsody::raw::texture::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_texture::h983927da0f94765c
    ; Large function body fully decoded (not a micro-leaf); decode present in insns.  [medium]
    0x1417faa80    55                       push             rbp
    0x1417faa81    4157                     push             r15
    0x1417faa83    4156                     push             r14
    0x1417faa85    4155                     push             r13
    0x1417faa87    4154                     push             r12
    0x1417faa89    56                       push             rsi
    0x1417faa8a    57                       push             rdi
    0x1417faa8b    53                       push             rbx
    0x1417faa8c    4881ec18040000           sub              rsp, 0x418
    0x1417faa93    488dac2480000000         lea              rbp, [rsp + 0x80]
    0x1417faa9b    48c78590030000feffffff   mov              qword ptr [rbp + 0x390], 0xfffffffffffffffe
    0x1417faaa6    4c898578030000           mov              qword ptr [rbp + 0x378], r8
    0x1417faaad    4889d3                   mov              rbx, rdx
    0x1417faab0    4889ce                   mov              rsi, rcx
    0x1417faab3    488b9500040000           mov              rdx, qword ptr [rbp + 0x400]
    0x1417faaba    488b8508040000           mov              rax, qword ptr [rbp + 0x408]
    0x1417faac1    488b4810                 mov              rcx, qword ptr [rax + 0x10]
    0x1417faac5    48898df0010000           mov              qword ptr [rbp + 0x1f0], rcx
    0x1417faacc    0f1000                   movups           xmm0, xmmword ptr [rax]
    0x1417faacf    0f2985e0010000           movaps           xmmword ptr [rbp + 0x1e0], xmm0
    0x1417faad6    8b02                     mov              eax, dword ptr [rdx]
    0x1417faad8    85c0                     test             eax, eax
    0x1417faada    0f84fb060000             je               0x1417fb1db
    0x1417faae0    4d89ce                   mov              r14, r9
    0x1417faae3    488bbde0010000           mov              rdi, qword ptr [rbp + 0x1e0]
    0x1417faaea    4885ff                   test             rdi, rdi
    0x1417faaed    7405                     je               0x1417faaf4
    0x1417faaef    83c802                   or               eax, 2
    0x1417faaf2    8902                     mov              dword ptr [rdx], eax
    0x1417faaf4    4885ff                   test             rdi, rdi
    0x1417faaf7    410f95c0                 setne            r8b
    0x1417faafb    488d4d78                 lea              rcx, [rbp + 0x78]
    0x1417faaff    e8ccb33dff               call             0x140bd5ed0 -> rhapsody::raw::texture::get_image_create_info
    0x1417fab04    83bd9400000000           cmp              dword ptr [rbp + 0x94], 0
    0x1417fab0b    0f84e2060000             je               0x1417fb1f3
    0x1417fab11    83bd9800000000           cmp              dword ptr [rbp + 0x98], 0
    0x1417fab18    0f84ed060000             je               0x1417fb20b
    0x1417fab1e    83bd9c00000000           cmp              dword ptr [rbp + 0x9c], 0
    0x1417fab25    0f84f8060000             je               0x1417fb223
    0x1417fab2b    488b8b40060000           mov              rcx, qword ptr [rbx + 0x640]
    0x1417fab32    488d5578                 lea              rdx, [rbp + 0x78]
    0x1417fab36    4c8d8d70020000           lea              r9, [rbp + 0x270]
    0x1417fab3d    4531c0                   xor              r8d, r8d
    0x1417fab40    ff93a0010000             call             qword ptr [rbx + 0x1a0]
    0x1417fab46    85c0                     test             eax, eax
    0x1417fab48    0f8516070000             jne              0x1417fb264
    0x1417fab4e    4c8ba570020000           mov              r12, qword ptr [rbp + 0x270]
    0x1417fab55    4889d9                   mov              rcx, rbx
    0x1417fab58    e803de3fff               call             0x140bf8960 -> rhapsody::raw::device::GraphicsDevice::debug_utils
    0x1417fab5d    4885c0                   test             rax, rax
    0x1417fab60    0f84a9000000             je               0x1417fac0f
    0x1417fab66    4989c5                   mov              r13, rax
    0x1417fab69    4d89f0                   mov              r8, r14
    0x1417fab6c    4889b580030000           mov              qword ptr [rbp + 0x380], rsi
    0x1417fab73    4c89e6                   mov              rsi, r12
    0x1417fab76    488d8dd0000000           lea              rcx, [rbp + 0xd0]
    0x1417fab7d    488b9578030000           mov              rdx, qword ptr [rbp + 0x378]
    0x1417fab84    e847ec57ff               call             0x140d797d0 -> _$LT$$RF$str$u20$as$u20$alloc..ffi..c_str..CString..new..SpecNewImpl$GT$::spec_new_impl::h27339bb99b671fc9
    0x1417fab89    31c0                     xor              eax, eax
    0x1417fab8b    483b85d0000000           cmp              rax, qword ptr [rbp + 0xd0]
    0x1417fab92    0f813a070000             jno              0x1417fb2d2
    0x1417fab98    4c8ba5d8000000           mov              r12, qword ptr [rbp + 0xd8]
    0x1417fab9f    4c8bbde0000000           mov              r15, qword ptr [rbp + 0xe0]
    0x1417faba6    c7857002000000be9c3b     mov              dword ptr [rbp + 0x270], 0x3b9cbe00
    0x1417fabb0    48c7857802000000000000   mov              qword ptr [rbp + 0x278], 0
    0x1417fabbb    c785800200000a000000     mov              dword ptr [rbp + 0x280], 0xa
    0x1417fabc5    4889b588020000           mov              qword ptr [rbp + 0x288], rsi
    0x1417fabcc    4c89a590020000           mov              qword ptr [rbp + 0x290], r12
    0x1417fabd3    498b4d40                 mov              rcx, qword ptr [r13 + 0x40]
    0x1417fabd7    488d9570020000           lea              rdx, [rbp + 0x270]
    0x1417fabde    41ff5500                 call             qword ptr [r13]
    0x1417fabe2    85c0                     test             eax, eax
    0x1417fabe4    0f8531070000             jne              0x1417fb31b
    0x1417fabea    41c6042400               mov              byte ptr [r12], 0
    0x1417fabef    4d85ff                   test             r15, r15
    0x1417fabf2    4c89e1                   mov              rcx, r12
    0x1417fabf5    4989f4                   mov              r12, rsi
    0x1417fabf8    488bb580030000           mov              rsi, qword ptr [rbp + 0x380]
    0x1417fabff    740e                     je               0x1417fac0f
    0x1417fac01    41b801000000             mov              r8d, 1
    0x1417fac07    4c89fa                   mov              rdx, r15
    0x1417fac0a    e81190e1fe               call             0x140613c20 -> __rust_dealloc
    0x1417fac0f    4d89f7                   mov              r15, r14
    0x1417fac12    488b8b40060000           mov              rcx, qword ptr [rbx + 0x640]
    0x1417fac19    4c8d8550030000           lea              r8, [rbp + 0x350]
    0x1417fac20    4c89e2                   mov              rdx, r12
    0x1417fac23    ff93f0000000             call             qword ptr [rbx + 0xf0]
    0x1417fac29    4c8bab78060000           mov              r13, qword ptr [rbx + 0x678]
    0x1417fac30    4d8d7510                 lea              r14, [r13 + 0x10]
    0x1417fac34    b101                     mov              cl, 1
    0x1417fac36    31c0                     xor              eax, eax
    0x1417fac38    f0410fb04d10             lock cmpxchg     byte ptr [r13 + 0x10], cl
    0x1417fac3e    0f85f7050000             jne              0x1417fb23b
    0x1417fac44    4c89b568030000           mov              qword ptr [rbp + 0x368], r14
    0x1417fac4b    4c89f1                   mov              rcx, r14
    0x1417fac4e    e81d7048ff               call             0x140c81c70 -> parking_lot_core::parking_lot::deadlock_impl::acquire_resource::hb2f16ac28e483830
    0x1417fac53    4983c518                 add              r13, 0x18
    0x1417fac57    488b8578030000           mov              rax, qword ptr [rbp + 0x378]
    0x1417fac5e    488985e0000000           mov              qword ptr [rbp + 0xe0], rax
    0x1417fac65    4d89fe                   mov              r14, r15
    0x1417fac68    4c89bde8000000           mov              qword ptr [rbp + 0xe8], r15
    0x1417fac6f    0f108550030000           movups           xmm0, xmmword ptr [rbp + 0x350]
    0x1417fac76    0f1185f0000000           movups           xmmword ptr [rbp + 0xf0], xmm0
    0x1417fac7d    488b8560030000           mov              rax, qword ptr [rbp + 0x360]
    0x1417fac84    48898500010000           mov              qword ptr [rbp + 0x100], rax
    0x1417fac8b    66c785080100000001       mov              word ptr [rbp + 0x108], 0x100
    0x1417fac94    48c785d000000002000000   mov              qword ptr [rbp + 0xd0], 2
    0x1417fac9f    488d8d70020000           lea              rcx, [rbp + 0x270]
    0x1417faca6    4c8d85d0000000           lea              r8, [rbp + 0xd0]
    0x1417facad    4c89ea                   mov              rdx, r13
    0x1417facb0    e88b2d41ff               call             0x140c0da40 -> rhapsody::raw::allocator::GpuAllocator::allocate
    0x1417facb5    4889bd68020000           mov              qword ptr [rbp + 0x268], rdi
    0x1417facbc    80bdb402000002           cmp              byte ptr [rbp + 0x2b4], 2
    0x1417facc3    488b8570020000           mov              rax, qword ptr [rbp + 0x270]
    0x1417facca    48898580030000           mov              qword ptr [rbp + 0x380], rax
    0x1417facd1    4c8bad88020000           mov              r13, qword ptr [rbp + 0x288]
    0x1417facd8    0f859a000000             jne              0x1417fad78
    0x1417facde    488b8578020000           mov              rax, qword ptr [rbp + 0x278]
    0x1417face5    48898570030000           mov              qword ptr [rbp + 0x370], rax
    0x1417facec    4c8bbd80020000           mov              r15, qword ptr [rbp + 0x280]
    0x1417facf3    0fb6050f9fe301           movzx            eax, byte ptr [rip + 0x1e39f0f] -> 0x143634c09
    0x1417facfa    bf0f000000               mov              edi, 0xf
    0x1417facff    b90f000000               mov              ecx, 0xf
    0x1417fad04    ba01000000               mov              edx, 1
    0x1417fad09    e8028fe1fe               call             0x140613c10 -> __rust_alloc
    0x1417fad0e    4885c0                   test             rax, rax
    0x1417fad11    0f8445060000             je               0x1417fb35c
    0x1417fad17    48b92074657874757265     movabs           rcx, 0x6572757478657420
    0x1417fad21    48894807                 mov              qword ptr [rax + 7], rcx
    0x1417fad25    48b94770754f6e6c7920     movabs           rcx, 0x20796c6e4f757047
    0x1417fad2f    488908                   mov              qword ptr [rax], rcx
    0x1417fad32    488b8d60030000           mov              rcx, qword ptr [rbp + 0x360]
    0x1417fad39    48898da0010000           mov              qword ptr [rbp + 0x1a0], rcx
    0x1417fad40    0f108550030000           movups           xmm0, xmmword ptr [rbp + 0x350]
    0x1417fad47    0f298590010000           movaps           xmmword ptr [rbp + 0x190], xmm0
    0x1417fad4e    488b8d80030000           mov              rcx, qword ptr [rbp + 0x380]
    0x1417fad55    4883f90b                 cmp              rcx, 0xb
    0x1417fad59    0f8594030000             jne              0x1417fb0f3
    0x1417fad5f    4c89e9                   mov              rcx, r13
    0x1417fad62    488b9570030000           mov              rdx, qword ptr [rbp + 0x370]
    0x1417fad69    48899580030000           mov              qword ptr [rbp + 0x380], rdx
    0x1417fad70    41bd0f000000             mov              r13d, 0xf
    0x1417fad76    eb38                     jmp              0x1417fadb0
    0x1417fad78    4c8bbd78020000           mov              r15, qword ptr [rbp + 0x278]
    0x1417fad7f    488b8d80020000           mov              rcx, qword ptr [rbp + 0x280]
    0x1417fad86    488b8590020000           mov              rax, qword ptr [rbp + 0x290]
    0x1417fad8d    488bbd98020000           mov              rdi, qword ptr [rbp + 0x298]
    0x1417fad94    0f1085a0020000           movups           xmm0, xmmword ptr [rbp + 0x2a0]
    0x1417fad9b    0f298590010000           movaps           xmmword ptr [rbp + 0x190], xmm0
    0x1417fada2    488b95b0020000           mov              rdx, qword ptr [rbp + 0x2b0]
    0x1417fada9    488995a0010000           mov              qword ptr [rbp + 0x1a0], rdx
    0x1417fadb0    488b95a0010000           mov              rdx, qword ptr [rbp + 0x1a0]
    0x1417fadb7    48899510020000           mov              qword ptr [rbp + 0x210], rdx
    0x1417fadbe    0f288590010000           movaps           xmm0, xmmword ptr [rbp + 0x190]
    0x1417fadc5    0f298500020000           movaps           xmmword ptr [rbp + 0x200], xmm0
    0x1417fadcc    4c8b8580030000           mov              r8, qword ptr [rbp + 0x380]
    0x1417fadd3    4c898520020000           mov              qword ptr [rbp + 0x220], r8
    0x1417fadda    4c89bd28020000           mov              qword ptr [rbp + 0x228], r15
    0x1417fade1    48898d30020000           mov              qword ptr [rbp + 0x230], rcx
    0x1417fade8    0f118550020000           movups           xmmword ptr [rbp + 0x250], xmm0
    0x1417fadef    48899560020000           mov              qword ptr [rbp + 0x260], rdx
    0x1417fadf6    4c89ad38020000           mov              qword ptr [rbp + 0x238], r13
    0x1417fadfd    48898540020000           mov              qword ptr [rbp + 0x240], rax
    0x1417fae04    4889bd48020000           mov              qword ptr [rbp + 0x248], rdi
    0x1417fae0b    c6858f03000001           mov              byte ptr [rbp + 0x38f], 1
    0x1417fae12    488bbd68030000           mov              rdi, qword ptr [rbp + 0x368]
    0x1417fae19    4889f9                   mov              rcx, rdi
    0x1417fae1c    e8ef6f48ff               call             0x140c81e10 -> parking_lot_core::parking_lot::deadlock_impl::release_resource::h1c3ab4db915ad6f2
    0x1417fae21    31c9                     xor              ecx, ecx
    0x1417fae23    b001                     mov              al, 1
    0x1417fae25    f00fb00f                 lock cmpxchg     byte ptr [rdi], cl
    0x1417fae29    0f851f040000             jne              0x1417fb24e
    0x1417fae2f    c6858f03000001           mov              byte ptr [rbp + 0x38f], 1
    0x1417fae36    488d8d20020000           lea              rcx, [rbp + 0x220]
    0x1417fae3d    e86e5eedfe               call             0x1406d0cb0 -> gpu_allocator::vulkan::Allocation::memory::hfb1e6a660a853183
    0x1417fae42    4989c5                   mov              r13, rax
    0x1417fae45    c6858f03000001           mov              byte ptr [rbp + 0x38f], 1
    0x1417fae4c    488d8d20020000           lea              rcx, [rbp + 0x220]
    0x1417fae53    e858bae8fe               call             0x1406868b0 -> rhapsody::raw::buffer::impl$3::byte_count
    0x1417fae58    488b8b40060000           mov              rcx, qword ptr [rbx + 0x640]
    0x1417fae5f    4c89e2                   mov              rdx, r12
    0x1417fae62    4d89e8                   mov              r8, r13
    0x1417fae65    4989c1                   mov              r9, rax
    0x1417fae68    ff93e0000000             call             qword ptr [rbx + 0xe0]
    0x1417fae6e    85c0                     test             eax, eax
    0x1417fae70    488b9500040000           mov              rdx, qword ptr [rbp + 0x400]
    0x1417fae77    0f8518040000             jne              0x1417fb295
    0x1417fae7d    488d4378                 lea              rax, [rbx + 0x78]
    0x1417fae81    4c8bab48060000           mov              r13, qword ptr [rbx + 0x648]
    0x1417fae88    488b8d60020000           mov              rcx, qword ptr [rbp + 0x260]
    0x1417fae8f    48898dd0010000           mov              qword ptr [rbp + 0x1d0], rcx
    0x1417fae96    0f108520020000           movups           xmm0, xmmword ptr [rbp + 0x220]
    0x1417fae9d    0f108d30020000           movups           xmm1, xmmword ptr [rbp + 0x230]
    0x1417faea4    0f109540020000           movups           xmm2, xmmword ptr [rbp + 0x240]
    0x1417faeab    0f109d50020000           movups           xmm3, xmmword ptr [rbp + 0x250]
    0x1417faeb2    0f299dc0010000           movaps           xmmword ptr [rbp + 0x1c0], xmm3
    0x1417faeb9    0f2995b0010000           movaps           xmmword ptr [rbp + 0x1b0], xmm2
    0x1417faec0    0f298da0010000           movaps           xmmword ptr [rbp + 0x1a0], xmm1
    0x1417faec7    0f298590010000           movaps           xmmword ptr [rbp + 0x190], xmm0
    0x1417faece    0f1002                   movups           xmm0, xmmword ptr [rdx]
    0x1417faed1    0f104a10                 movups           xmm1, xmmword ptr [rdx + 0x10]
    0x1417faed5    0f298d10020000           movaps           xmmword ptr [rbp + 0x210], xmm1
    0x1417faedc    0f298500020000           movaps           xmmword ptr [rbp + 0x200], xmm0
    0x1417faee3    c6858f03000000           mov              byte ptr [rbp + 0x38f], 0
    0x1417faeea    488d8d00020000           lea              rcx, [rbp + 0x200]
    0x1417faef1    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x1417faef6    488d8d90010000           lea              rcx, [rbp + 0x190]
    0x1417faefd    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x1417faf02    4889442420               mov              qword ptr [rsp + 0x20], rax
    0x1417faf07    488d8d70020000           lea              rcx, [rbp + 0x270]
    0x1417faf0e    488b9578030000           mov              rdx, qword ptr [rbp + 0x378]
    0x1417faf15    4d89f0                   mov              r8, r14
    0x1417faf18    4d89e1                   mov              r9, r12
    0x1417faf1b    e8b0133eff               call             0x140bdc2d0 -> rhapsody::raw::managed::texture::ManagedTexture::new
    0x1417faf20    0fb6bd2c030000           movzx            edi, byte ptr [rbp + 0x32c]
    0x1417faf27    4080ff03                 cmp              dil, 3
    0x1417faf2b    755e                     jne              0x1417faf8b
    0x1417faf2d    0f1085b0020000           movups           xmm0, xmmword ptr [rbp + 0x2b0]
    0x1417faf34    0f298510010000           movaps           xmmword ptr [rbp + 0x110], xmm0
    0x1417faf3b    0f108d70020000           movups           xmm1, xmmword ptr [rbp + 0x270]
    0x1417faf42    0f109580020000           movups           xmm2, xmmword ptr [rbp + 0x280]
    0x1417faf49    0f109d90020000           movups           xmm3, xmmword ptr [rbp + 0x290]
    0x1417faf50    0f10a5a0020000           movups           xmm4, xmmword ptr [rbp + 0x2a0]
    0x1417faf57    0f29a500010000           movaps           xmmword ptr [rbp + 0x100], xmm4
    0x1417faf5e    0f299df0000000           movaps           xmmword ptr [rbp + 0xf0], xmm3
    0x1417faf65    0f2995e0000000           movaps           xmmword ptr [rbp + 0xe0], xmm2
    0x1417faf6c    0f298dd0000000           movaps           xmmword ptr [rbp + 0xd0], xmm1
    0x1417faf73    0f114640                 movups           xmmword ptr [rsi + 0x40], xmm0
    0x1417faf77    0f116630                 movups           xmmword ptr [rsi + 0x30], xmm4
    0x1417faf7b    0f115e20                 movups           xmmword ptr [rsi + 0x20], xmm3
    0x1417faf7f    0f115610                 movups           xmmword ptr [rsi + 0x10], xmm2
    0x1417faf83    0f110e                   movups           xmmword ptr [rsi], xmm1
    0x1417faf86    e939020000               jmp              0x1417fb1c4
    0x1417faf8b    4c8db5d0000000           lea              r14, [rbp + 0xd0]
    0x1417faf92    488d9570020000           lea              rdx, [rbp + 0x270]
    0x1417faf99    41b8bc000000             mov              r8d, 0xbc
    0x1417faf9f    4c89f1                   mov              rcx, r14
    0x1417fafa2    e8598c1201               call             0x142923c00 -> memcpy
    0x1417fafa7    0fb7852d030000           movzx            eax, word ptr [rbp + 0x32d]
    0x1417fafae    66894575                 mov              word ptr [rbp + 0x75], ax
    0x1417fafb2    0fb6852f030000           movzx            eax, byte ptr [rbp + 0x32f]
    0x1417fafb9    884577                   mov              byte ptr [rbp + 0x77], al
    0x1417fafbc    4c8d7db8                 lea              r15, [rbp - 0x48]
    0x1417fafc0    41b8bc000000             mov              r8d, 0xbc
    0x1417fafc6    4c89f9                   mov              rcx, r15
    0x1417fafc9    4c89f2                   mov              rdx, r14
    0x1417fafcc    e82f8c1201               call             0x142923c00 -> memcpy
    0x1417fafd1    40887d74                 mov              byte ptr [rbp + 0x74], dil
    0x1417fafd5    c6858f03000000           mov              byte ptr [rbp + 0x38f], 0
    0x1417fafdc    4c89e9                   mov              rcx, r13
    0x1417fafdf    4c89fa                   mov              rdx, r15
    0x1417fafe2    e8690d3eff               call             0x140bdbd50 -> rhapsody::raw::managed::resources::impl$6::allocate
    0x1417fafe7    8b480c                   mov              ecx, dword ptr [rax + 0xc]
    0x1417fafea    410fb795500a0000         movzx            edx, word ptr [r13 + 0xa50]
    0x1417faff2    4d8b4500                 mov              r8, qword ptr [r13]
    0x1417faff6    4c898538030000           mov              qword ptr [rbp + 0x338], r8
    0x1417faffd    4989c0                   mov              r8, rax
    0x1417fb000    4983c010                 add              r8, 0x10
    0x1417fb004    48898540030000           mov              qword ptr [rbp + 0x340], rax
    0x1417fb00b    898d48030000             mov              dword ptr [rbp + 0x348], ecx
    0x1417fb011    6689954c030000           mov              word ptr [rbp + 0x34c], dx
    0x1417fb018    c6854e03000000           mov              byte ptr [rbp + 0x34e], 0
    0x1417fb01f    4c898530030000           mov              qword ptr [rbp + 0x330], r8
    0x1417fb026    4883bd6802000000         cmp              qword ptr [rbp + 0x268], 0
    0x1417fb02e    488bbd00040000           mov              rdi, qword ptr [rbp + 0x400]
    0x1417fb035    744f                     je               0x1417fb086
    0x1417fb037    0f1007                   movups           xmm0, xmmword ptr [rdi]
    0x1417fb03a    0f104f10                 movups           xmm1, xmmword ptr [rdi + 0x10]
    0x1417fb03e    0f298de0000000           movaps           xmmword ptr [rbp + 0xe0], xmm1
    0x1417fb045    0f2985d0000000           movaps           xmmword ptr [rbp + 0xd0], xmm0
    0x1417fb04c    488d8530030000           lea              rax, [rbp + 0x330]
    0x1417fb053    4889442420               mov              qword ptr [rsp + 0x20], rax
    0x1417fb058    488d8d70020000           lea              rcx, [rbp + 0x270]
    0x1417fb05f    4c8d85d0000000           lea              r8, [rbp + 0xd0]
    0x1417fb066    4c8d8de0010000           lea              r9, [rbp + 0x1e0]
    0x1417fb06d    4889da                   mov              rdx, rbx
    0x1417fb070    e8fb3140ff               call             0x140bfe270 -> rhapsody::raw::device::GraphicsDevice::init_texture_data
    0x1417fb075    488b8570020000           mov              rax, qword ptr [rbp + 0x270]
    0x1417fb07c    4883f80b                 cmp              rax, 0xb
    0x1417fb080    0f85f1000000             jne              0x1417fb177
    0x1417fb086    488b8548030000           mov              rax, qword ptr [rbp + 0x348]
    0x1417fb08d    48898580020000           mov              qword ptr [rbp + 0x280], rax
    0x1417fb094    0f108538030000           movups           xmm0, xmmword ptr [rbp + 0x338]
    0x1417fb09b    0f298570020000           movaps           xmmword ptr [rbp + 0x270], xmm0
    0x1417fb0a2    0f100f                   movups           xmm1, xmmword ptr [rdi]
    0x1417fb0a5    0f105710                 movups           xmm2, xmmword ptr [rdi + 0x10]
    0x1417fb0a9    0f119598020000           movups           xmmword ptr [rbp + 0x298], xmm2
    0x1417fb0b0    0f118d88020000           movups           xmmword ptr [rbp + 0x288], xmm1
    0x1417fb0b7    0f114608                 movups           xmmword ptr [rsi + 8], xmm0
    0x1417fb0bb    488b85a0020000           mov              rax, qword ptr [rbp + 0x2a0]
    0x1417fb0c2    48894638                 mov              qword ptr [rsi + 0x38], rax
    0x1417fb0c6    0f288590020000           movaps           xmm0, xmmword ptr [rbp + 0x290]
    0x1417fb0cd    0f114628                 movups           xmmword ptr [rsi + 0x28], xmm0
    0x1417fb0d1    488b8580020000           mov              rax, qword ptr [rbp + 0x280]
    0x1417fb0d8    48894618                 mov              qword ptr [rsi + 0x18], rax
    0x1417fb0dc    488b8588020000           mov              rax, qword ptr [rbp + 0x288]
    0x1417fb0e3    48894620                 mov              qword ptr [rsi + 0x20], rax
    0x1417fb0e7    48c7060b000000           mov              qword ptr [rsi], 0xb
    0x1417fb0ee    e9d1000000               jmp              0x1417fb1c4
    0x1417fb0f3    4889ca                   mov              rdx, rcx
    0x1417fb0f6    488b8d60030000           mov              rcx, qword ptr [rbp + 0x360]
    0x1417fb0fd    48898d10020000           mov              qword ptr [rbp + 0x210], rcx
    0x1417fb104    0f108550030000           movups           xmm0, xmmword ptr [rbp + 0x350]
    0x1417fb10b    0f298500020000           movaps           xmmword ptr [rbp + 0x200], xmm0
    0x1417fb112    488b8d70030000           mov              rcx, qword ptr [rbp + 0x370]
    0x1417fb119    48894e08                 mov              qword ptr [rsi + 8], rcx
    0x1417fb11d    4c897e10                 mov              qword ptr [rsi + 0x10], r15
    0x1417fb121    4c896e18                 mov              qword ptr [rsi + 0x18], r13
    0x1417fb125    0f108550030000           movups           xmm0, xmmword ptr [rbp + 0x350]
    0x1417fb12c    0f114638                 movups           xmmword ptr [rsi + 0x38], xmm0
    0x1417fb130    488b8d60030000           mov              rcx, qword ptr [rbp + 0x360]
    0x1417fb137    48894e48                 mov              qword ptr [rsi + 0x48], rcx
    0x1417fb13b    488916                   mov              qword ptr [rsi], rdx
    0x1417fb13e    48c746200f000000         mov              qword ptr [rsi + 0x20], 0xf
    0x1417fb146    48894628                 mov              qword ptr [rsi + 0x28], rax
    0x1417fb14a    48c746300f000000         mov              qword ptr [rsi + 0x30], 0xf
    0x1417fb152    488bbd68030000           mov              rdi, qword ptr [rbp + 0x368]
    0x1417fb159    4889f9                   mov              rcx, rdi
    0x1417fb15c    e8af6c48ff               call             0x140c81e10 -> parking_lot_core::parking_lot::deadlock_impl::release_resource::h1c3ab4db915ad6f2
    0x1417fb161    31c9                     xor              ecx, ecx
    0x1417fb163    b001                     mov              al, 1
    0x1417fb165    f00fb00f                 lock cmpxchg     byte ptr [rdi], cl
    0x1417fb169    7459                     je               0x1417fb1c4
    0x1417fb16b    4889f9                   mov              rcx, rdi
    0x1417fb16e    31d2                     xor              edx, edx
    0x1417fb170    e81b2a0d01               call             0x1428cdb90 -> parking_lot::raw_mutex::RawMutex::unlock_slow::h56cb6df3e32a2657
    0x1417fb175    eb4d                     jmp              0x1417fb1c4
    0x1417fb177    488b8db8020000           mov              rcx, qword ptr [rbp + 0x2b8]
    0x1417fb17e    48894e48                 mov              qword ptr [rsi + 0x48], rcx
    0x1417fb182    0f108578020000           movups           xmm0, xmmword ptr [rbp + 0x278]
    0x1417fb189    0f108d88020000           movups           xmm1, xmmword ptr [rbp + 0x288]
    0x1417fb190    0f109598020000           movups           xmm2, xmmword ptr [rbp + 0x298]
    0x1417fb197    0f109da8020000           movups           xmm3, xmmword ptr [rbp + 0x2a8]
    0x1417fb19e    0f115e38                 movups           xmmword ptr [rsi + 0x38], xmm3
    0x1417fb1a2    0f115628                 movups           xmmword ptr [rsi + 0x28], xmm2
    0x1417fb1a6    0f114e18                 movups           xmmword ptr [rsi + 0x18], xmm1
    0x1417fb1aa    0f114608                 movups           xmmword ptr [rsi + 8], xmm0
    0x1417fb1ae    488906                   mov              qword ptr [rsi], rax
    0x1417fb1b1    c6858f03000000           mov              byte ptr [rbp + 0x38f], 0
    0x1417fb1b8    488d8d38030000           lea              rcx, [rbp + 0x338]
    0x1417fb1bf    e8ec723eff               call             0x140be24b0 -> rhapsody::raw::managed::handle::impl$6::drop
    0x1417fb1c4    4889f0                   mov              rax, rsi
    0x1417fb1c7    4881c418040000           add              rsp, 0x418
    0x1417fb1ce    5b                       pop              rbx
    0x1417fb1cf    5f                       pop              rdi
    0x1417fb1d0    5e                       pop              rsi
    0x1417fb1d1    415c                     pop              r12
    0x1417fb1d3    415d                     pop              r13
    0x1417fb1d5    415e                     pop              r14
    0x1417fb1d7    415f                     pop              r15
    0x1417fb1d9    5d                       pop              rbp
    0x1417fb1da    c3                       ret              
    0x1417fb1db    488d0d20f84e01           lea              rcx, [rip + 0x14ef820] -> 0x142ceaa02
    0x1417fb1e2    4c8d0557f84e01           lea              r8, [rip + 0x14ef857] -> 0x142ceaa40
    0x1417fb1e9    ba3c000000               mov              edx, 0x3c
    0x1417fb1ee    e83de20d01               call             0x1428d9430 -> core::panicking::panic::hb210ae7b817a9f13
    0x1417fb1f3    488d0d5ef84e01           lea              rcx, [rip + 0x14ef85e] -> 0x142ceaa58
    0x1417fb1fa    4c8d0587f84e01           lea              r8, [rip + 0x14ef887] -> 0x142ceaa88
    0x1417fb201    ba2e000000               mov              edx, 0x2e
    0x1417fb206    e825e20d01               call             0x1428d9430 -> core::panicking::panic::hb210ae7b817a9f13
    0x1417fb20b    488d0d8ef84e01           lea              rcx, [rip + 0x14ef88e] -> 0x142ceaaa0
    0x1417fb212    4c8d05b7f84e01           lea              r8, [rip + 0x14ef8b7] -> 0x142ceaad0
    0x1417fb219    ba2f000000               mov              edx, 0x2f
    0x1417fb21e    e80de20d01               call             0x1428d9430 -> core::panicking::panic::hb210ae7b817a9f13
    0x1417fb223    488d0dbef84e01           lea              rcx, [rip + 0x14ef8be] -> 0x142ceaae8
    0x1417fb22a    4c8d05e7f84e01           lea              r8, [rip + 0x14ef8e7] -> 0x142ceab18
    0x1417fb231    ba2e000000               mov              edx, 0x2e
    0x1417fb236    e8f5e10d01               call             0x1428d9430 -> core::panicking::panic::hb210ae7b817a9f13
    0x1417fb23b    4c89f1                   mov              rcx, r14
    0x1417fb23e    41b800ca9a3b             mov              r8d, 0x3b9aca00
    0x1417fb244    e8c71f0d01               call             0x1428cd210 -> parking_lot::raw_mutex::RawMutex::lock_slow::h7c244b522f551b38
    0x1417fb249    e9f6f9ffff               jmp              0x1417fac44
    0x1417fb24e    c6858f03000001           mov              byte ptr [rbp + 0x38f], 1
    0x1417fb255    4889f9                   mov              rcx, rdi
    0x1417fb258    31d2                     xor              edx, edx
    0x1417fb25a    e831290d01               call             0x1428cdb90 -> parking_lot::raw_mutex::RawMutex::unlock_slow::h56cb6df3e32a2657
    0x1417fb25f    e9cbfbffff               jmp              0x1417fae2f
    0x1417fb264    898570020000             mov              dword ptr [rbp + 0x270], eax
    0x1417fb26a    488d05cff84e01           lea              rax, [rip + 0x14ef8cf] -> 0x142ceab40
    0x1417fb271    4889442420               mov              qword ptr [rsp + 0x20], rax
    0x1417fb276    488d0db3f84e01           lea              rcx, [rip + 0x14ef8b3] -> 0x142ceab30
    0x1417fb27d    4c8d0d14f64e01           lea              r9, [rip + 0x14ef614] -> 0x142cea898
    0x1417fb284    4c8d8570020000           lea              r8, [rbp + 0x270]
    0x1417fb28b    ba0c000000               mov              edx, 0xc
    0x1417fb290    e8cbe40d01               call             0x1428d9760 -> core::result::unwrap_failed::h96d74ae09566f4b3
    0x1417fb295    898570020000             mov              dword ptr [rbp + 0x270], eax
    0x1417fb29b    c6858f03000001           mov              byte ptr [rbp + 0x38f], 1
    0x1417fb2a2    488d0517f94e01           lea              rax, [rip + 0x14ef917] -> 0x142ceabc0
    0x1417fb2a9    4889442420               mov              qword ptr [rsp + 0x20], rax
    0x1417fb2ae    488d0df3f84e01           lea              rcx, [rip + 0x14ef8f3] -> 0x142ceaba8
    0x1417fb2b5    4c8d0ddcf54e01           lea              r9, [rip + 0x14ef5dc] -> 0x142cea898
    0x1417fb2bc    4c8d8570020000           lea              r8, [rbp + 0x270]
    0x1417fb2c3    ba11000000               mov              edx, 0x11
    0x1417fb2c8    e893e40d01               call             0x1428d9760 -> core::result::unwrap_failed::h96d74ae09566f4b3
    0x1417fb2cd    e9a7000000               jmp              0x1417fb379
    0x1417fb2d2    0f1085d0000000           movups           xmm0, xmmword ptr [rbp + 0xd0]
    0x1417fb2d9    0f108de0000000           movups           xmm1, xmmword ptr [rbp + 0xe0]
    0x1417fb2e0    0f298d80020000           movaps           xmmword ptr [rbp + 0x280], xmm1
    0x1417fb2e7    0f298570020000           movaps           xmmword ptr [rbp + 0x270], xmm0
    0x1417fb2ee    488d0563f84e01           lea              rax, [rip + 0x14ef863] -> 0x142ceab58
    0x1417fb2f5    4889442420               mov              qword ptr [rsp + 0x20], rax
    0x1417fb2fa    488d0dd7f54e01           lea              rcx, [rip + 0x14ef5d7] -> 0x142cea8d8
    0x1417fb301    4c8d0db0f54e01           lea              r9, [rip + 0x14ef5b0] -> 0x142cea8b8
    0x1417fb308    4c8d8570020000           lea              r8, [rbp + 0x270]
    0x1417fb30f    ba2b000000               mov              edx, 0x2b
    0x1417fb314    e847e40d01               call             0x1428d9760 -> core::result::unwrap_failed::h96d74ae09566f4b3
    0x1417fb319    eb5e                     jmp              0x1417fb379
    0x1417fb31b    4c89a580030000           mov              qword ptr [rbp + 0x380], r12
    0x1417fb322    4c89bd78030000           mov              qword ptr [rbp + 0x378], r15
    0x1417fb329    8985d0000000             mov              dword ptr [rbp + 0xd0], eax
    0x1417fb32f    488d055af84e01           lea              rax, [rip + 0x14ef85a] -> 0x142ceab90
    0x1417fb336    4889442420               mov              qword ptr [rsp + 0x20], rax
    0x1417fb33b    488d0d2ef84e01           lea              rcx, [rip + 0x14ef82e] -> 0x142ceab70
    0x1417fb342    4c8d0d4ff54e01           lea              r9, [rip + 0x14ef54f] -> 0x142cea898
    0x1417fb349    4c8d85d0000000           lea              r8, [rbp + 0xd0]
    0x1417fb350    ba1b000000               mov              edx, 0x1b
    0x1417fb355    e806e40d01               call             0x1428d9760 -> core::result::unwrap_failed::h96d74ae09566f4b3
    0x1417fb35a    eb1d                     jmp              0x1417fb379
    0x1417fb35c    4c89bd78030000           mov              qword ptr [rbp + 0x378], r15
    0x1417fb363    4c8d05e6f54e01           lea              r8, [rip + 0x14ef5e6] -> 0x142cea950
    0x1417fb36a    b901000000               mov              ecx, 1
    0x1417fb36f    ba0f000000               mov              edx, 0xf
    0x1417fb374    e86adc0d01               call             0x1428d8fe3 -> alloc::raw_vec::handle_error::hb99beac7784c4464
    0x1417fb379    0f0b                     ud2              

=== 0x1418b4da0 size=421  rhapsody::raw::error::impl$1::fmt
    ; Leaf/impl fully decoded from PE bytes (semantics mostly diagnostic/formatting); see insns.  [medium]
    0x1418b4da0    4881ec88000000           sub              rsp, 0x88
    0x1418b4da7    4889d0                   mov              rax, rdx
    0x1418b4daa    4c8b01                   mov              r8, qword ptr [rcx]
    0x1418b4dad    4d8d48f7                 lea              r9, [r8 - 9]
    0x1418b4db1    4983c0f8                 add              r8, -8
    0x1418b4db5    31d2                     xor              edx, edx
    0x1418b4db7    4983f902                 cmp              r9, 2
    0x1418b4dbb    490f42d0                 cmovb            rdx, r8
    0x1418b4dbf    4885d2                   test             rdx, rdx
    0x1418b4dc2    0f8487000000             je               0x1418b4e4f
    0x1418b4dc8    4883fa01                 cmp              rdx, 1
    0x1418b4dcc    0f8520010000             jne              0x1418b4ef2
    0x1418b4dd2    488d5120                 lea              rdx, [rcx + 0x20]
    0x1418b4dd6    4883c108                 add              rcx, 8
    0x1418b4dda    48898c2480000000         mov              qword ptr [rsp + 0x80], rcx
    0x1418b4de2    488d0d27e94401           lea              rcx, [rip + 0x144e927] -> 0x142d03710
    0x1418b4de9    48894c2450               mov              qword ptr [rsp + 0x50], rcx
    0x1418b4dee    488d8c2480000000         lea              rcx, [rsp + 0x80]
    0x1418b4df6    48894c2448               mov              qword ptr [rsp + 0x48], rcx
    0x1418b4dfb    488d0d37e94401           lea              rcx, [rip + 0x144e937] -> 0x142d03739
    0x1418b4e02    48894c2438               mov              qword ptr [rsp + 0x38], rcx
    0x1418b4e07    488d0dc2e44401           lea              rcx, [rip + 0x144e4c2] -> 0x142d032d0
    0x1418b4e0e    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x1418b4e13    4889542428               mov              qword ptr [rsp + 0x28], rdx
    0x1418b4e18    48c744244005000000       mov              qword ptr [rsp + 0x40], 5
    0x1418b4e21    48c744242003000000       mov              qword ptr [rsp + 0x20], 3
    0x1418b4e2a    488d15ffe84401           lea              rdx, [rip + 0x144e8ff] -> 0x142d03730
    0x1418b4e31    4c8d0dfee84401           lea              r9, [rip + 0x144e8fe] -> 0x142d03736
    0x1418b4e38    41b806000000             mov              r8d, 6
    0x1418b4e3e    4889c1                   mov              rcx, rax
    0x1418b4e41    e85a1e4dff               call             0x140d86ca0 -> core::fmt::Formatter::debug_struct_field2_finish::h35c7e6c960cc525a
    0x1418b4e46    90                       nop              
    0x1418b4e47    4881c488000000           add              rsp, 0x88
    0x1418b4e4e    c3                       ret              
    0x1418b4e4f    488d5120                 lea              rdx, [rcx + 0x20]
    0x1418b4e53    4c8d4138                 lea              r8, [rcx + 0x38]
    0x1418b4e57    4c89842480000000         mov              qword ptr [rsp + 0x80], r8
    0x1418b4e5f    4c8d056ae84401           lea              r8, [rip + 0x144e86a] -> 0x142d036d0
    0x1418b4e66    4c89442470               mov              qword ptr [rsp + 0x70], r8
    0x1418b4e6b    4c8d842480000000         lea              r8, [rsp + 0x80]
    0x1418b4e73    4c89442468               mov              qword ptr [rsp + 0x68], r8
    0x1418b4e78    4c8d0584e84401           lea              r8, [rip + 0x144e884] -> 0x142d03703
    0x1418b4e7f    4c89442458               mov              qword ptr [rsp + 0x58], r8
    0x1418b4e84    4c8d0525e84401           lea              r8, [rip + 0x144e825] -> 0x142d036b0
    0x1418b4e8b    4c89442450               mov              qword ptr [rsp + 0x50], r8
    0x1418b4e90    4889542448               mov              qword ptr [rsp + 0x48], rdx
    0x1418b4e95    488d1563e84401           lea              rdx, [rip + 0x144e863] -> 0x142d036ff
    0x1418b4e9c    4889542438               mov              qword ptr [rsp + 0x38], rdx
    0x1418b4ea1    488d15e8e74401           lea              rdx, [rip + 0x144e7e8] -> 0x142d03690
    0x1418b4ea8    4889542430               mov              qword ptr [rsp + 0x30], rdx
    0x1418b4ead    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x1418b4eb2    48c74424600c000000       mov              qword ptr [rsp + 0x60], 0xc
    0x1418b4ebb    48c744244004000000       mov              qword ptr [rsp + 0x40], 4
    0x1418b4ec4    48c744242005000000       mov              qword ptr [rsp + 0x20], 5
    0x1418b4ecd    488d151ce84401           lea              rdx, [rip + 0x144e81c] -> 0x142d036f0
    0x1418b4ed4    4c8d0d1fe84401           lea              r9, [rip + 0x144e81f] -> 0x142d036fa
    0x1418b4edb    41b80a000000             mov              r8d, 0xa
    0x1418b4ee1    4889c1                   mov              rcx, rax
    0x1418b4ee4    e8a71e4dff               call             0x140d86d90 -> core::fmt::Formatter::debug_struct_field3_finish::h024add6f4e520141
    0x1418b4ee9    90                       nop              
    0x1418b4eea    4881c488000000           add              rsp, 0x88
    0x1418b4ef1    c3                       ret              
    0x1418b4ef2    4883c108                 add              rcx, 8
    0x1418b4ef6    48898c2480000000         mov              qword ptr [rsp + 0x80], rcx
    0x1418b4efe    488d0db3e44401           lea              rcx, [rip + 0x144e4b3] -> 0x142d033b8
    0x1418b4f05    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x1418b4f0a    488d8c2480000000         lea              rcx, [rsp + 0x80]
    0x1418b4f12    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x1418b4f17    48c744242004000000       mov              qword ptr [rsp + 0x20], 4
    0x1418b4f20    488d1517e84401           lea              rdx, [rip + 0x144e817] -> 0x142d0373e
    0x1418b4f27    4c8d0d1ee84401           lea              r9, [rip + 0x144e81e] -> 0x142d0374c
    0x1418b4f2e    41b80e000000             mov              r8d, 0xe
    0x1418b4f34    4889c1                   mov              rcx, rax
    0x1418b4f37    e8b41c4dff               call             0x140d86bf0 -> core::fmt::Formatter::debug_struct_field1_finish::hd9497ec2d71b9977
    0x1418b4f3c    90                       nop              
    0x1418b4f3d    4881c488000000           add              rsp, 0x88
    0x1418b4f44    c3                       ret              

=== 0x1418bd700 size=39  rhapsody::raw::texture::impl$14::fmt
    ; Leaf/impl fully decoded from PE bytes (semantics mostly diagnostic/formatting); see insns.  [medium]
    0x1418bd700    4889d0                   mov              rax, rdx
    0x1418bd703    0fb609                   movzx            ecx, byte ptr [rcx]
    0x1418bd706    488d1533804401           lea              rdx, [rip + 0x1448033] -> 0x142d05740
    0x1418bd70d    4c8b04ca                 mov              r8, qword ptr [rdx + rcx*8]
    0x1418bd711    4c8d0d60804401           lea              r9, [rip + 0x1448060] -> 0x142d05778
    0x1418bd718    49631489                 movsxd           rdx, dword ptr [r9 + rcx*4]
    0x1418bd71c    4c01ca                   add              rdx, r9
    0x1418bd71f    4889c1                   mov              rcx, rax
    0x1418bd722    e969944cff               jmp              0x140d86b90 -> _$LT$core..fmt..Formatter$u20$as$u20$core..fmt..Write$GT$::write_str::h5c3f9ed06cfb23bd

=== 0x141fdca60 size=990  rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_buffer::h09676d89971b4d9f
    ; Large function body fully decoded (not a micro-leaf); decode present in insns.  [medium]
    0x141fdca60    55                       push             rbp
    0x141fdca61    4157                     push             r15
    0x141fdca63    4156                     push             r14
    0x141fdca65    4155                     push             r13
    0x141fdca67    4154                     push             r12
    0x141fdca69    56                       push             rsi
    0x141fdca6a    57                       push             rdi
    0x141fdca6b    53                       push             rbx
    0x141fdca6c    4881ec68010000           sub              rsp, 0x168
    0x141fdca73    488dac2480000000         lea              rbp, [rsp + 0x80]
    0x141fdca7b    48c785e0000000feffffff   mov              qword ptr [rbp + 0xe0], 0xfffffffffffffffe
    0x141fdca86    4d89cf                   mov              r15, r9
    0x141fdca89    4d89c6                   mov              r14, r8
    0x141fdca8c    4889d7                   mov              rdi, rdx
    0x141fdca8f    4889ce                   mov              rsi, rcx
    0x141fdca92    4c8bad50010000           mov              r13, qword ptr [rbp + 0x150]
    0x141fdca99    0f108558010000           movups           xmm0, xmmword ptr [rbp + 0x158]
    0x141fdcaa0    0f1185d0000000           movups           xmmword ptr [rbp + 0xd0], xmm0
    0x141fdcaa7    488d8dd0000000           lea              rcx, [rbp + 0xd0]
    0x141fdcaae    e8ed86bffe               call             0x140bd51a0 -> rhapsody::raw::buffer::impl$4::byte_count
    0x141fdcab3    4885c0                   test             rax, rax
    0x141fdcab6    7405                     je               0x141fdcabd
    0x141fdcab8    41804d0802               or               byte ptr [r13 + 8], 2
    0x141fdcabd    488d5f78                 lea              rbx, [rdi + 0x78]
    0x141fdcac1    4889f9                   mov              rcx, rdi
    0x141fdcac4    e897bec1fe               call             0x140bf8960 -> rhapsody::raw::device::GraphicsDevice::debug_utils
    0x141fdcac9    4c8b8f48060000           mov              r9, qword ptr [rdi + 0x648]
    0x141fdcad0    488b8f78060000           mov              rcx, qword ptr [rdi + 0x678]
    0x141fdcad7    4883c110                 add              rcx, 0x10
    0x141fdcadb    410f104500               movups           xmm0, xmmword ptr [r13]
    0x141fdcae0    0f294520                 movaps           xmmword ptr [rbp + 0x20], xmm0
    0x141fdcae4    4c897c2438               mov              qword ptr [rsp + 0x38], r15
    0x141fdcae9    4c89742430               mov              qword ptr [rsp + 0x30], r14
    0x141fdcaee    4c8d6520                 lea              r12, [rbp + 0x20]
    0x141fdcaf2    4c89642428               mov              qword ptr [rsp + 0x28], r12
    0x141fdcaf7    48894c2420               mov              qword ptr [rsp + 0x20], rcx
    0x141fdcafc    488d4d50                 lea              rcx, [rbp + 0x50]
    0x141fdcb00    4889da                   mov              rdx, rbx
    0x141fdcb03    4989c0                   mov              r8, rax
    0x141fdcb06    e8f516c1fe               call             0x140bee200 -> rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_buffer_impl::h34de0f745611d2ab
    0x141fdcb0b    488b4550                 mov              rax, qword ptr [rbp + 0x50]
    0x141fdcb0f    0f104558                 movups           xmm0, xmmword ptr [rbp + 0x58]
    0x141fdcb13    0f2985a0000000           movaps           xmmword ptr [rbp + 0xa0], xmm0
    0x141fdcb1a    0f104568                 movups           xmm0, xmmword ptr [rbp + 0x68]
    0x141fdcb1e    0f2985b0000000           movaps           xmmword ptr [rbp + 0xb0], xmm0
    0x141fdcb25    0f104578                 movups           xmm0, xmmword ptr [rbp + 0x78]
    0x141fdcb29    0f2985c0000000           movaps           xmmword ptr [rbp + 0xc0], xmm0
    0x141fdcb30    4c8bb588000000           mov              r14, qword ptr [rbp + 0x88]
    0x141fdcb37    4c8bbd90000000           mov              r15, qword ptr [rbp + 0x90]
    0x141fdcb3e    4883f80b                 cmp              rax, 0xb
    0x141fdcb42    0f859c000000             jne              0x141fdcbe4
    0x141fdcb48    0f2885a0000000           movaps           xmm0, xmmword ptr [rbp + 0xa0]
    0x141fdcb4f    0f288db0000000           movaps           xmm1, xmmword ptr [rbp + 0xb0]
    0x141fdcb56    0f2895c0000000           movaps           xmm2, xmmword ptr [rbp + 0xc0]
    0x141fdcb5d    0f295540                 movaps           xmmword ptr [rbp + 0x40], xmm2
    0x141fdcb61    0f294d30                 movaps           xmmword ptr [rbp + 0x30], xmm1
    0x141fdcb65    0f294520                 movaps           xmmword ptr [rbp + 0x20], xmm0
    0x141fdcb69    488d8dd0000000           lea              rcx, [rbp + 0xd0]
    0x141fdcb70    e82b86bffe               call             0x140bd51a0 -> rhapsody::raw::buffer::impl$4::byte_count
    0x141fdcb75    4885c0                   test             rax, rax
    0x141fdcb78    7449                     je               0x141fdcbc3
    0x141fdcb7a    488d8dd0000000           lea              rcx, [rbp + 0xd0]
    0x141fdcb81    e81a86bffe               call             0x140bd51a0 -> rhapsody::raw::buffer::impl$4::byte_count
    0x141fdcb86    498b5500                 mov              rdx, qword ptr [r13]
    0x141fdcb8a    4839d0                   cmp              rax, rdx
    0x141fdcb8d    0f874f020000             ja               0x141fdcde2
    0x141fdcb93    4d85f6                   test             r14, r14
    0x141fdcb96    0f8496000000             je               0x141fdcc32
    0x141fdcb9c    488d8dd0000000           lea              rcx, [rbp + 0xd0]
    0x141fdcba3    e8f885bffe               call             0x140bd51a0 -> rhapsody::raw::buffer::impl$4::byte_count
    0x141fdcba8    4c39f8                   cmp              rax, r15
    0x141fdcbab    0f874b020000             ja               0x141fdcdfc
    0x141fdcbb1    488d8dd0000000           lea              rcx, [rbp + 0xd0]
    0x141fdcbb8    4c89f2                   mov              rdx, r14
    0x141fdcbbb    4989c0                   mov              r8, rax
    0x141fdcbbe    e8ed85bffe               call             0x140bd51b0 -> rhapsody::raw::buffer::impl$4::copy_to_slice
    0x141fdcbc3    488b4548                 mov              rax, qword ptr [rbp + 0x48]
    0x141fdcbc7    48894628                 mov              qword ptr [rsi + 0x28], rax
    0x141fdcbcb    0f104528                 movups           xmm0, xmmword ptr [rbp + 0x28]
    0x141fdcbcf    0f104d38                 movups           xmm1, xmmword ptr [rbp + 0x38]
    0x141fdcbd3    0f114e18                 movups           xmmword ptr [rsi + 0x18], xmm1
    0x141fdcbd7    0f114608                 movups           xmmword ptr [rsi + 8], xmm0
    0x141fdcbdb    48c7060b000000           mov              qword ptr [rsi], 0xb
    0x141fdcbe2    eb37                     jmp              0x141fdcc1b
    0x141fdcbe4    488b8d98000000           mov              rcx, qword ptr [rbp + 0x98]
    0x141fdcbeb    0f2885a0000000           movaps           xmm0, xmmword ptr [rbp + 0xa0]
    0x141fdcbf2    0f288db0000000           movaps           xmm1, xmmword ptr [rbp + 0xb0]
    0x141fdcbf9    0f2895c0000000           movaps           xmm2, xmmword ptr [rbp + 0xc0]
    0x141fdcc00    0f115628                 movups           xmmword ptr [rsi + 0x28], xmm2
    0x141fdcc04    0f114e18                 movups           xmmword ptr [rsi + 0x18], xmm1
    0x141fdcc08    0f114608                 movups           xmmword ptr [rsi + 8], xmm0
    0x141fdcc0c    488906                   mov              qword ptr [rsi], rax
    0x141fdcc0f    4c897638                 mov              qword ptr [rsi + 0x38], r14
    0x141fdcc13    4c897e40                 mov              qword ptr [rsi + 0x40], r15
    0x141fdcc17    48894e48                 mov              qword ptr [rsi + 0x48], rcx
    0x141fdcc1b    4889f0                   mov              rax, rsi
    0x141fdcc1e    4881c468010000           add              rsp, 0x168
    0x141fdcc25    5b                       pop              rbx
    0x141fdcc26    5f                       pop              rdi
    0x141fdcc27    5e                       pop              rsi
    0x141fdcc28    415c                     pop              r12
    0x141fdcc2a    415d                     pop              r13
    0x141fdcc2c    415e                     pop              r14
    0x141fdcc2e    415f                     pop              r15
    0x141fdcc30    5d                       pop              rbp
    0x141fdcc31    c3                       ret              
    0x141fdcc32    4c8d75c0                 lea              r14, [rbp - 0x40]
    0x141fdcc36    4c89f1                   mov              rcx, r14
    0x141fdcc39    41b801000000             mov              r8d, 1
    0x141fdcc3f    e85c82bffe               call             0x140bd4ea0 -> rhapsody::raw::buffer::BufferDesc::new_cpu_to_gpu_low_prio
    0x141fdcc44    4889f9                   mov              rcx, rdi
    0x141fdcc47    e814bdc1fe               call             0x140bf8960 -> rhapsody::raw::device::GraphicsDevice::debug_utils
    0x141fdcc4c    4c8b8f48060000           mov              r9, qword ptr [rdi + 0x648]
    0x141fdcc53    488b8f78060000           mov              rcx, qword ptr [rdi + 0x678]
    0x141fdcc5a    4883c110                 add              rcx, 0x10
    0x141fdcc5e    488d1573e0fe00           lea              rdx, [rip + 0xfee073] -> 0x142fcacd8
    0x141fdcc65    4889542430               mov              qword ptr [rsp + 0x30], rdx
    0x141fdcc6a    4c89742428               mov              qword ptr [rsp + 0x28], r14
    0x141fdcc6f    48894c2420               mov              qword ptr [rsp + 0x20], rcx
    0x141fdcc74    48c744243813000000       mov              qword ptr [rsp + 0x38], 0x13
    0x141fdcc7d    4c8d7550                 lea              r14, [rbp + 0x50]
    0x141fdcc81    4c89f1                   mov              rcx, r14
    0x141fdcc84    4889da                   mov              rdx, rbx
    0x141fdcc87    4989c0                   mov              r8, rax
    0x141fdcc8a    e87115c1fe               call             0x140bee200 -> rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_buffer_impl::h34de0f745611d2ab
    0x141fdcc8f    488b4550                 mov              rax, qword ptr [rbp + 0x50]
    0x141fdcc93    0f104558                 movups           xmm0, xmmword ptr [rbp + 0x58]
    0x141fdcc97    0f2985a0000000           movaps           xmmword ptr [rbp + 0xa0], xmm0
    0x141fdcc9e    0f104568                 movups           xmm0, xmmword ptr [rbp + 0x68]
    0x141fdcca2    0f2985b0000000           movaps           xmmword ptr [rbp + 0xb0], xmm0
    0x141fdcca9    0f104578                 movups           xmm0, xmmword ptr [rbp + 0x78]
    0x141fdccad    0f2985c0000000           movaps           xmmword ptr [rbp + 0xc0], xmm0
    0x141fdccb4    488b9d88000000           mov              rbx, qword ptr [rbp + 0x88]
    0x141fdccbb    4c8bbd90000000           mov              r15, qword ptr [rbp + 0x90]
    0x141fdccc2    4883f80b                 cmp              rax, 0xb
    0x141fdccc6    0f859b000000             jne              0x141fdcd67
    0x141fdcccc    0f2885a0000000           movaps           xmm0, xmmword ptr [rbp + 0xa0]
    0x141fdccd3    0f288db0000000           movaps           xmm1, xmmword ptr [rbp + 0xb0]
    0x141fdccda    0f2895c0000000           movaps           xmm2, xmmword ptr [rbp + 0xc0]
    0x141fdcce1    0f295570                 movaps           xmmword ptr [rbp + 0x70], xmm2
    0x141fdcce5    0f294d60                 movaps           xmmword ptr [rbp + 0x60], xmm1
    0x141fdcce9    0f294550                 movaps           xmmword ptr [rbp + 0x50], xmm0
    0x141fdcced    4885db                   test             rbx, rbx
    0x141fdccf0    0f842e010000             je               0x141fdce24
    0x141fdccf6    488d8dd0000000           lea              rcx, [rbp + 0xd0]
    0x141fdccfd    e89e84bffe               call             0x140bd51a0 -> rhapsody::raw::buffer::impl$4::byte_count
    0x141fdcd02    4c39f8                   cmp              rax, r15
    0x141fdcd05    0f8705010000             ja               0x141fdce10
    0x141fdcd0b    4c8dbdd0000000           lea              r15, [rbp + 0xd0]
    0x141fdcd12    4c89f9                   mov              rcx, r15
    0x141fdcd15    4889da                   mov              rdx, rbx
    0x141fdcd18    4989c0                   mov              r8, rax
    0x141fdcd1b    e89084bffe               call             0x140bd51b0 -> rhapsody::raw::buffer::impl$4::copy_to_slice
    0x141fdcd20    4c89b5a0000000           mov              qword ptr [rbp + 0xa0], r14
    0x141fdcd27    4889bda8000000           mov              qword ptr [rbp + 0xa8], rdi
    0x141fdcd2e    4c89a5b0000000           mov              qword ptr [rbp + 0xb0], r12
    0x141fdcd35    4c89bdb8000000           mov              qword ptr [rbp + 0xb8], r15
    0x141fdcd3c    488d4dd0                 lea              rcx, [rbp - 0x30]
    0x141fdcd40    4c8d85a0000000           lea              r8, [rbp + 0xa0]
    0x141fdcd47    4889fa                   mov              rdx, rdi
    0x141fdcd4a    e8f17ac2fe               call             0x140c04840 -> rhapsody::raw::device::GraphicsDevice::with_setup_cb::h946964bfafb47630
    0x141fdcd4f    488b45d0                 mov              rax, qword ptr [rbp - 0x30]
    0x141fdcd53    4883f80b                 cmp              rax, 0xb
    0x141fdcd57    7547                     jne              0x141fdcda0
    0x141fdcd59    488d4d58                 lea              rcx, [rbp + 0x58]
    0x141fdcd5d    e84e57c0fe               call             0x140be24b0 -> rhapsody::raw::managed::handle::impl$6::drop
    0x141fdcd62    e95cfeffff               jmp              0x141fdcbc3
    0x141fdcd67    488b8d98000000           mov              rcx, qword ptr [rbp + 0x98]
    0x141fdcd6e    0f2885a0000000           movaps           xmm0, xmmword ptr [rbp + 0xa0]
    0x141fdcd75    0f288db0000000           movaps           xmm1, xmmword ptr [rbp + 0xb0]
    0x141fdcd7c    0f2895c0000000           movaps           xmm2, xmmword ptr [rbp + 0xc0]
    0x141fdcd83    0f115628                 movups           xmmword ptr [rsi + 0x28], xmm2
    0x141fdcd87    0f114e18                 movups           xmmword ptr [rsi + 0x18], xmm1
    0x141fdcd8b    0f114608                 movups           xmmword ptr [rsi + 8], xmm0
    0x141fdcd8f    488906                   mov              qword ptr [rsi], rax
    0x141fdcd92    48895e38                 mov              qword ptr [rsi + 0x38], rbx
    0x141fdcd96    4c897e40                 mov              qword ptr [rsi + 0x40], r15
    0x141fdcd9a    48894e48                 mov              qword ptr [rsi + 0x48], rcx
    0x141fdcd9e    eb34                     jmp              0x141fdcdd4
    0x141fdcda0    488b4d18                 mov              rcx, qword ptr [rbp + 0x18]
    0x141fdcda4    48894e48                 mov              qword ptr [rsi + 0x48], rcx
    0x141fdcda8    0f1045d8                 movups           xmm0, xmmword ptr [rbp - 0x28]
    0x141fdcdac    0f104de8                 movups           xmm1, xmmword ptr [rbp - 0x18]
    0x141fdcdb0    0f1055f8                 movups           xmm2, xmmword ptr [rbp - 8]
    0x141fdcdb4    0f105d08                 movups           xmm3, xmmword ptr [rbp + 8]
    0x141fdcdb8    0f115e38                 movups           xmmword ptr [rsi + 0x38], xmm3
    0x141fdcdbc    0f115628                 movups           xmmword ptr [rsi + 0x28], xmm2
    0x141fdcdc0    0f114e18                 movups           xmmword ptr [rsi + 0x18], xmm1
    0x141fdcdc4    0f114608                 movups           xmmword ptr [rsi + 8], xmm0
    0x141fdcdc8    488906                   mov              qword ptr [rsi], rax
    0x141fdcdcb    488d4d58                 lea              rcx, [rbp + 0x58]
    0x141fdcdcf    e8dc56c0fe               call             0x140be24b0 -> rhapsody::raw::managed::handle::impl$6::drop
    0x141fdcdd4    488d4d28                 lea              rcx, [rbp + 0x28]
    0x141fdcdd8    e8d356c0fe               call             0x140be24b0 -> rhapsody::raw::managed::handle::impl$6::drop
    0x141fdcddd    e939feffff               jmp              0x141fdcc1b
    0x141fdcde2    488d0d32defe00           lea              rcx, [rip + 0xfede32] -> 0x142fcac1b
    0x141fdcde9    4c8d05b8defe00           lea              r8, [rip + 0xfedeb8] -> 0x142fcaca8
    0x141fdcdf0    ba38000000               mov              edx, 0x38
    0x141fdcdf5    e836c68f00               call             0x1428d9430 -> core::panicking::panic::hb210ae7b817a9f13
    0x141fdcdfa    eb40                     jmp              0x141fdce3c
    0x141fdcdfc    4c8d05bddefe00           lea              r8, [rip + 0xfedebd] -> 0x142fcacc0
    0x141fdce03    4889c1                   mov              rcx, rax
    0x141fdce06    4c89fa                   mov              rdx, r15
    0x141fdce09    e832ca8f00               call             0x1428d9840 -> core::slice::index::slice_end_index_len_fail::hc3a65ec02a6eede1
    0x141fdce0e    eb2c                     jmp              0x141fdce3c
    0x141fdce10    4c8d0501dffe00           lea              r8, [rip + 0xfedf01] -> 0x142fcad18
    0x141fdce17    4889c1                   mov              rcx, rax
    0x141fdce1a    4c89fa                   mov              rdx, r15
    0x141fdce1d    e81eca8f00               call             0x1428d9840 -> core::slice::index::slice_end_index_len_fail::hc3a65ec02a6eede1
    0x141fdce22    eb18                     jmp              0x141fdce3c
    0x141fdce24    488d0dc0defe00           lea              rcx, [rip + 0xfedec0] -> 0x142fcaceb
    0x141fdce2b    4c8d05cedefe00           lea              r8, [rip + 0xfedece] -> 0x142fcad00
    0x141fdce32    ba14000000               mov              edx, 0x14
    0x141fdce37    e8f4c48f00               call             0x1428d9330 -> core::option::expect_failed::hb3e848c6b16873c8
    0x141fdce3c    0f0b                     ud2              

=== 0x14205f650 size=421  rhapsody::raw::error::impl$1::fmt
    ; Leaf/impl fully decoded from PE bytes (semantics mostly diagnostic/formatting); see insns.  [medium]
    0x14205f650    4881ec88000000           sub              rsp, 0x88
    0x14205f657    4889d0                   mov              rax, rdx
    0x14205f65a    4c8b01                   mov              r8, qword ptr [rcx]
    0x14205f65d    4d8d48f7                 lea              r9, [r8 - 9]
    0x14205f661    4983c0f8                 add              r8, -8
    0x14205f665    31d2                     xor              edx, edx
    0x14205f667    4983f902                 cmp              r9, 2
    0x14205f66b    490f42d0                 cmovb            rdx, r8
    0x14205f66f    4885d2                   test             rdx, rdx
    0x14205f672    0f8487000000             je               0x14205f6ff
    0x14205f678    4883fa01                 cmp              rdx, 1
    0x14205f67c    0f8520010000             jne              0x14205f7a2
    0x14205f682    488d5120                 lea              rdx, [rcx + 0x20]
    0x14205f686    4883c108                 add              rcx, 8
    0x14205f68a    48898c2480000000         mov              qword ptr [rsp + 0x80], rcx
    0x14205f692    488d0d5f00f700           lea              rcx, [rip + 0xf7005f] -> 0x142fcf6f8
    0x14205f699    48894c2450               mov              qword ptr [rsp + 0x50], rcx
    0x14205f69e    488d8c2480000000         lea              rcx, [rsp + 0x80]
    0x14205f6a6    48894c2448               mov              qword ptr [rsp + 0x48], rcx
    0x14205f6ab    488d0d6f00f700           lea              rcx, [rip + 0xf7006f] -> 0x142fcf721
    0x14205f6b2    48894c2438               mov              qword ptr [rsp + 0x38], rcx
    0x14205f6b7    488d0d1a00f700           lea              rcx, [rip + 0xf7001a] -> 0x142fcf6d8
    0x14205f6be    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x14205f6c3    4889542428               mov              qword ptr [rsp + 0x28], rdx
    0x14205f6c8    48c744244005000000       mov              qword ptr [rsp + 0x40], 5
    0x14205f6d1    48c744242003000000       mov              qword ptr [rsp + 0x20], 3
    0x14205f6da    488d153700f700           lea              rdx, [rip + 0xf70037] -> 0x142fcf718
    0x14205f6e1    4c8d0d3600f700           lea              r9, [rip + 0xf70036] -> 0x142fcf71e
    0x14205f6e8    41b806000000             mov              r8d, 6
    0x14205f6ee    4889c1                   mov              rcx, rax
    0x14205f6f1    e8aa75d2fe               call             0x140d86ca0 -> core::fmt::Formatter::debug_struct_field2_finish::h35c7e6c960cc525a
    0x14205f6f6    90                       nop              
    0x14205f6f7    4881c488000000           add              rsp, 0x88
    0x14205f6fe    c3                       ret              
    0x14205f6ff    488d5120                 lea              rdx, [rcx + 0x20]
    0x14205f703    4c8d4138                 lea              r8, [rcx + 0x38]
    0x14205f707    4c89842480000000         mov              qword ptr [rsp + 0x80], r8
    0x14205f70f    4c8d0582fff600           lea              r8, [rip + 0xf6ff82] -> 0x142fcf698
    0x14205f716    4c89442470               mov              qword ptr [rsp + 0x70], r8
    0x14205f71b    4c8d842480000000         lea              r8, [rsp + 0x80]
    0x14205f723    4c89442468               mov              qword ptr [rsp + 0x68], r8
    0x14205f728    4c8d059cfff600           lea              r8, [rip + 0xf6ff9c] -> 0x142fcf6cb
    0x14205f72f    4c89442458               mov              qword ptr [rsp + 0x58], r8
    0x14205f734    4c8d053dfff600           lea              r8, [rip + 0xf6ff3d] -> 0x142fcf678
    0x14205f73b    4c89442450               mov              qword ptr [rsp + 0x50], r8
    0x14205f740    4889542448               mov              qword ptr [rsp + 0x48], rdx
    0x14205f745    488d157bfff600           lea              rdx, [rip + 0xf6ff7b] -> 0x142fcf6c7
    0x14205f74c    4889542438               mov              qword ptr [rsp + 0x38], rdx
    0x14205f751    488d1500fff600           lea              rdx, [rip + 0xf6ff00] -> 0x142fcf658
    0x14205f758    4889542430               mov              qword ptr [rsp + 0x30], rdx
    0x14205f75d    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x14205f762    48c74424600c000000       mov              qword ptr [rsp + 0x60], 0xc
    0x14205f76b    48c744244004000000       mov              qword ptr [rsp + 0x40], 4
    0x14205f774    48c744242005000000       mov              qword ptr [rsp + 0x20], 5
    0x14205f77d    488d1534fff600           lea              rdx, [rip + 0xf6ff34] -> 0x142fcf6b8
    0x14205f784    4c8d0d37fff600           lea              r9, [rip + 0xf6ff37] -> 0x142fcf6c2
    0x14205f78b    41b80a000000             mov              r8d, 0xa
    0x14205f791    4889c1                   mov              rcx, rax
    0x14205f794    e8f775d2fe               call             0x140d86d90 -> core::fmt::Formatter::debug_struct_field3_finish::h024add6f4e520141
    0x14205f799    90                       nop              
    0x14205f79a    4881c488000000           add              rsp, 0x88
    0x14205f7a1    c3                       ret              
    0x14205f7a2    4883c108                 add              rcx, 8
    0x14205f7a6    48898c2480000000         mov              qword ptr [rsp + 0x80], rcx
    0x14205f7ae    488d0debfbf600           lea              rcx, [rip + 0xf6fbeb] -> 0x142fcf3a0
    0x14205f7b5    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x14205f7ba    488d8c2480000000         lea              rcx, [rsp + 0x80]
    0x14205f7c2    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x14205f7c7    48c744242004000000       mov              qword ptr [rsp + 0x20], 4
    0x14205f7d0    488d154ffff600           lea              rdx, [rip + 0xf6ff4f] -> 0x142fcf726
    0x14205f7d7    4c8d0d56fff600           lea              r9, [rip + 0xf6ff56] -> 0x142fcf734
    0x14205f7de    41b80e000000             mov              r8d, 0xe
    0x14205f7e4    4889c1                   mov              rcx, rax
    0x14205f7e7    e80474d2fe               call             0x140d86bf0 -> core::fmt::Formatter::debug_struct_field1_finish::hd9497ec2d71b9977
    0x14205f7ec    90                       nop              
    0x14205f7ed    4881c488000000           add              rsp, 0x88
    0x14205f7f4    c3                       ret              

=== 0x142138520 size=421  rhapsody::raw::error::impl$1::fmt
    ; Leaf/impl fully decoded from PE bytes (semantics mostly diagnostic/formatting); see insns.  [medium]
    0x142138520    4881ec88000000           sub              rsp, 0x88
    0x142138527    4889d0                   mov              rax, rdx
    0x14213852a    4c8b01                   mov              r8, qword ptr [rcx]
    0x14213852d    4d8d48f7                 lea              r9, [r8 - 9]
    0x142138531    4983c0f8                 add              r8, -8
    0x142138535    31d2                     xor              edx, edx
    0x142138537    4983f902                 cmp              r9, 2
    0x14213853b    490f42d0                 cmovb            rdx, r8
    0x14213853f    4885d2                   test             rdx, rdx
    0x142138542    0f8487000000             je               0x1421385cf
    0x142138548    4883fa01                 cmp              rdx, 1
    0x14213854c    0f8520010000             jne              0x142138672
    0x142138552    488d5120                 lea              rdx, [rcx + 0x20]
    0x142138556    4883c108                 add              rcx, 8
    0x14213855a    48898c2480000000         mov              qword ptr [rsp + 0x80], rcx
    0x142138562    488d0dc704eb00           lea              rcx, [rip + 0xeb04c7] -> 0x142fe8a30
    0x142138569    48894c2450               mov              qword ptr [rsp + 0x50], rcx
    0x14213856e    488d8c2480000000         lea              rcx, [rsp + 0x80]
    0x142138576    48894c2448               mov              qword ptr [rsp + 0x48], rcx
    0x14213857b    488d0dd704eb00           lea              rcx, [rip + 0xeb04d7] -> 0x142fe8a59
    0x142138582    48894c2438               mov              qword ptr [rsp + 0x38], rcx
    0x142138587    488d0db2ffea00           lea              rcx, [rip + 0xeaffb2] -> 0x142fe8540
    0x14213858e    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x142138593    4889542428               mov              qword ptr [rsp + 0x28], rdx
    0x142138598    48c744244005000000       mov              qword ptr [rsp + 0x40], 5
    0x1421385a1    48c744242003000000       mov              qword ptr [rsp + 0x20], 3
    0x1421385aa    488d159f04eb00           lea              rdx, [rip + 0xeb049f] -> 0x142fe8a50
    0x1421385b1    4c8d0d9e04eb00           lea              r9, [rip + 0xeb049e] -> 0x142fe8a56
    0x1421385b8    41b806000000             mov              r8d, 6
    0x1421385be    4889c1                   mov              rcx, rax
    0x1421385c1    e8dae6c4fe               call             0x140d86ca0 -> core::fmt::Formatter::debug_struct_field2_finish::h35c7e6c960cc525a
    0x1421385c6    90                       nop              
    0x1421385c7    4881c488000000           add              rsp, 0x88
    0x1421385ce    c3                       ret              
    0x1421385cf    488d5120                 lea              rdx, [rcx + 0x20]
    0x1421385d3    4c8d4138                 lea              r8, [rcx + 0x38]
    0x1421385d7    4c89842480000000         mov              qword ptr [rsp + 0x80], r8
    0x1421385df    4c8d050a04eb00           lea              r8, [rip + 0xeb040a] -> 0x142fe89f0
    0x1421385e6    4c89442470               mov              qword ptr [rsp + 0x70], r8
    0x1421385eb    4c8d842480000000         lea              r8, [rsp + 0x80]
    0x1421385f3    4c89442468               mov              qword ptr [rsp + 0x68], r8
    0x1421385f8    4c8d052404eb00           lea              r8, [rip + 0xeb0424] -> 0x142fe8a23
    0x1421385ff    4c89442458               mov              qword ptr [rsp + 0x58], r8
    0x142138604    4c8d05c503eb00           lea              r8, [rip + 0xeb03c5] -> 0x142fe89d0
    0x14213860b    4c89442450               mov              qword ptr [rsp + 0x50], r8
    0x142138610    4889542448               mov              qword ptr [rsp + 0x48], rdx
    0x142138615    488d150304eb00           lea              rdx, [rip + 0xeb0403] -> 0x142fe8a1f
    0x14213861c    4889542438               mov              qword ptr [rsp + 0x38], rdx
    0x142138621    488d158803eb00           lea              rdx, [rip + 0xeb0388] -> 0x142fe89b0
    0x142138628    4889542430               mov              qword ptr [rsp + 0x30], rdx
    0x14213862d    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x142138632    48c74424600c000000       mov              qword ptr [rsp + 0x60], 0xc
    0x14213863b    48c744244004000000       mov              qword ptr [rsp + 0x40], 4
    0x142138644    48c744242005000000       mov              qword ptr [rsp + 0x20], 5
    0x14213864d    488d15bc03eb00           lea              rdx, [rip + 0xeb03bc] -> 0x142fe8a10
    0x142138654    4c8d0dbf03eb00           lea              r9, [rip + 0xeb03bf] -> 0x142fe8a1a
    0x14213865b    41b80a000000             mov              r8d, 0xa
    0x142138661    4889c1                   mov              rcx, rax
    0x142138664    e827e7c4fe               call             0x140d86d90 -> core::fmt::Formatter::debug_struct_field3_finish::h024add6f4e520141
    0x142138669    90                       nop              
    0x14213866a    4881c488000000           add              rsp, 0x88
    0x142138671    c3                       ret              
    0x142138672    4883c108                 add              rcx, 8
    0x142138676    48898c2480000000         mov              qword ptr [rsp + 0x80], rcx
    0x14213867e    488d0db3ffea00           lea              rcx, [rip + 0xeaffb3] -> 0x142fe8638
    0x142138685    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x14213868a    488d8c2480000000         lea              rcx, [rsp + 0x80]
    0x142138692    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x142138697    48c744242004000000       mov              qword ptr [rsp + 0x20], 4
    0x1421386a0    488d15b703eb00           lea              rdx, [rip + 0xeb03b7] -> 0x142fe8a5e
    0x1421386a7    4c8d0dbe03eb00           lea              r9, [rip + 0xeb03be] -> 0x142fe8a6c
    0x1421386ae    41b80e000000             mov              r8d, 0xe
    0x1421386b4    4889c1                   mov              rcx, rax
    0x1421386b7    e834e5c4fe               call             0x140d86bf0 -> core::fmt::Formatter::debug_struct_field1_finish::hd9497ec2d71b9977
    0x1421386bc    90                       nop              
    0x1421386bd    4881c488000000           add              rsp, 0x88
    0x1421386c4    c3                       ret              

=== 0x14288e990 size=990  rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_buffer::h44649ae9cdfab458
    ; Large function body fully decoded (not a micro-leaf); decode present in insns.  [medium]
    0x14288e990    55                       push             rbp
    0x14288e991    4157                     push             r15
    0x14288e993    4156                     push             r14
    0x14288e995    4155                     push             r13
    0x14288e997    4154                     push             r12
    0x14288e999    56                       push             rsi
    0x14288e99a    57                       push             rdi
    0x14288e99b    53                       push             rbx
    0x14288e99c    4881ec68010000           sub              rsp, 0x168
    0x14288e9a3    488dac2480000000         lea              rbp, [rsp + 0x80]
    0x14288e9ab    48c785e0000000feffffff   mov              qword ptr [rbp + 0xe0], 0xfffffffffffffffe
    0x14288e9b6    4d89cf                   mov              r15, r9
    0x14288e9b9    4d89c6                   mov              r14, r8
    0x14288e9bc    4889d7                   mov              rdi, rdx
    0x14288e9bf    4889ce                   mov              rsi, rcx
    0x14288e9c2    4c8bad50010000           mov              r13, qword ptr [rbp + 0x150]
    0x14288e9c9    0f108558010000           movups           xmm0, xmmword ptr [rbp + 0x158]
    0x14288e9d0    0f1185d0000000           movups           xmmword ptr [rbp + 0xd0], xmm0
    0x14288e9d7    488d8dd0000000           lea              rcx, [rbp + 0xd0]
    0x14288e9de    e8bd6734fe               call             0x140bd51a0 -> rhapsody::raw::buffer::impl$4::byte_count
    0x14288e9e3    4885c0                   test             rax, rax
    0x14288e9e6    7405                     je               0x14288e9ed
    0x14288e9e8    41804d0802               or               byte ptr [r13 + 8], 2
    0x14288e9ed    488d5f78                 lea              rbx, [rdi + 0x78]
    0x14288e9f1    4889f9                   mov              rcx, rdi
    0x14288e9f4    e8679f36fe               call             0x140bf8960 -> rhapsody::raw::device::GraphicsDevice::debug_utils
    0x14288e9f9    4c8b8f48060000           mov              r9, qword ptr [rdi + 0x648]
    0x14288ea00    488b8f78060000           mov              rcx, qword ptr [rdi + 0x678]
    0x14288ea07    4883c110                 add              rcx, 0x10
    0x14288ea0b    410f104500               movups           xmm0, xmmword ptr [r13]
    0x14288ea10    0f294520                 movaps           xmmword ptr [rbp + 0x20], xmm0
    0x14288ea14    4c897c2438               mov              qword ptr [rsp + 0x38], r15
    0x14288ea19    4c89742430               mov              qword ptr [rsp + 0x30], r14
    0x14288ea1e    4c8d6520                 lea              r12, [rbp + 0x20]
    0x14288ea22    4c89642428               mov              qword ptr [rsp + 0x28], r12
    0x14288ea27    48894c2420               mov              qword ptr [rsp + 0x20], rcx
    0x14288ea2c    488d4d50                 lea              rcx, [rbp + 0x50]
    0x14288ea30    4889da                   mov              rdx, rbx
    0x14288ea33    4989c0                   mov              r8, rax
    0x14288ea36    e8c5f735fe               call             0x140bee200 -> rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_buffer_impl::h34de0f745611d2ab
    0x14288ea3b    488b4550                 mov              rax, qword ptr [rbp + 0x50]
    0x14288ea3f    0f104558                 movups           xmm0, xmmword ptr [rbp + 0x58]
    0x14288ea43    0f2985a0000000           movaps           xmmword ptr [rbp + 0xa0], xmm0
    0x14288ea4a    0f104568                 movups           xmm0, xmmword ptr [rbp + 0x68]
    0x14288ea4e    0f2985b0000000           movaps           xmmword ptr [rbp + 0xb0], xmm0
    0x14288ea55    0f104578                 movups           xmm0, xmmword ptr [rbp + 0x78]
    0x14288ea59    0f2985c0000000           movaps           xmmword ptr [rbp + 0xc0], xmm0
    0x14288ea60    4c8bb588000000           mov              r14, qword ptr [rbp + 0x88]
    0x14288ea67    4c8bbd90000000           mov              r15, qword ptr [rbp + 0x90]
    0x14288ea6e    4883f80b                 cmp              rax, 0xb
    0x14288ea72    0f859c000000             jne              0x14288eb14
    0x14288ea78    0f2885a0000000           movaps           xmm0, xmmword ptr [rbp + 0xa0]
    0x14288ea7f    0f288db0000000           movaps           xmm1, xmmword ptr [rbp + 0xb0]
    0x14288ea86    0f2895c0000000           movaps           xmm2, xmmword ptr [rbp + 0xc0]
    0x14288ea8d    0f295540                 movaps           xmmword ptr [rbp + 0x40], xmm2
    0x14288ea91    0f294d30                 movaps           xmmword ptr [rbp + 0x30], xmm1
    0x14288ea95    0f294520                 movaps           xmmword ptr [rbp + 0x20], xmm0
    0x14288ea99    488d8dd0000000           lea              rcx, [rbp + 0xd0]
    0x14288eaa0    e8fb6634fe               call             0x140bd51a0 -> rhapsody::raw::buffer::impl$4::byte_count
    0x14288eaa5    4885c0                   test             rax, rax
    0x14288eaa8    7449                     je               0x14288eaf3
    0x14288eaaa    488d8dd0000000           lea              rcx, [rbp + 0xd0]
    0x14288eab1    e8ea6634fe               call             0x140bd51a0 -> rhapsody::raw::buffer::impl$4::byte_count
    0x14288eab6    498b5500                 mov              rdx, qword ptr [r13]
    0x14288eaba    4839d0                   cmp              rax, rdx
    0x14288eabd    0f874f020000             ja               0x14288ed12
    0x14288eac3    4d85f6                   test             r14, r14
    0x14288eac6    0f8496000000             je               0x14288eb62
    0x14288eacc    488d8dd0000000           lea              rcx, [rbp + 0xd0]
    0x14288ead3    e8c86634fe               call             0x140bd51a0 -> rhapsody::raw::buffer::impl$4::byte_count
    0x14288ead8    4c39f8                   cmp              rax, r15
    0x14288eadb    0f874b020000             ja               0x14288ed2c
    0x14288eae1    488d8dd0000000           lea              rcx, [rbp + 0xd0]
    0x14288eae8    4c89f2                   mov              rdx, r14
    0x14288eaeb    4989c0                   mov              r8, rax
    0x14288eaee    e8bd6634fe               call             0x140bd51b0 -> rhapsody::raw::buffer::impl$4::copy_to_slice
    0x14288eaf3    488b4548                 mov              rax, qword ptr [rbp + 0x48]
    0x14288eaf7    48894628                 mov              qword ptr [rsi + 0x28], rax
    0x14288eafb    0f104528                 movups           xmm0, xmmword ptr [rbp + 0x28]
    0x14288eaff    0f104d38                 movups           xmm1, xmmword ptr [rbp + 0x38]
    0x14288eb03    0f114e18                 movups           xmmword ptr [rsi + 0x18], xmm1
    0x14288eb07    0f114608                 movups           xmmword ptr [rsi + 8], xmm0
    0x14288eb0b    48c7060b000000           mov              qword ptr [rsi], 0xb
    0x14288eb12    eb37                     jmp              0x14288eb4b
    0x14288eb14    488b8d98000000           mov              rcx, qword ptr [rbp + 0x98]
    0x14288eb1b    0f2885a0000000           movaps           xmm0, xmmword ptr [rbp + 0xa0]
    0x14288eb22    0f288db0000000           movaps           xmm1, xmmword ptr [rbp + 0xb0]
    0x14288eb29    0f2895c0000000           movaps           xmm2, xmmword ptr [rbp + 0xc0]
    0x14288eb30    0f115628                 movups           xmmword ptr [rsi + 0x28], xmm2
    0x14288eb34    0f114e18                 movups           xmmword ptr [rsi + 0x18], xmm1
    0x14288eb38    0f114608                 movups           xmmword ptr [rsi + 8], xmm0
    0x14288eb3c    488906                   mov              qword ptr [rsi], rax
    0x14288eb3f    4c897638                 mov              qword ptr [rsi + 0x38], r14
    0x14288eb43    4c897e40                 mov              qword ptr [rsi + 0x40], r15
    0x14288eb47    48894e48                 mov              qword ptr [rsi + 0x48], rcx
    0x14288eb4b    4889f0                   mov              rax, rsi
    0x14288eb4e    4881c468010000           add              rsp, 0x168
    0x14288eb55    5b                       pop              rbx
    0x14288eb56    5f                       pop              rdi
    0x14288eb57    5e                       pop              rsi
    0x14288eb58    415c                     pop              r12
    0x14288eb5a    415d                     pop              r13
    0x14288eb5c    415e                     pop              r14
    0x14288eb5e    415f                     pop              r15
    0x14288eb60    5d                       pop              rbp
    0x14288eb61    c3                       ret              
    0x14288eb62    4c8d75c0                 lea              r14, [rbp - 0x40]
    0x14288eb66    4c89f1                   mov              rcx, r14
    0x14288eb69    41b801000000             mov              r8d, 1
    0x14288eb6f    e82c6334fe               call             0x140bd4ea0 -> rhapsody::raw::buffer::BufferDesc::new_cpu_to_gpu_low_prio
    0x14288eb74    4889f9                   mov              rcx, rdi
    0x14288eb77    e8e49d36fe               call             0x140bf8960 -> rhapsody::raw::device::GraphicsDevice::debug_utils
    0x14288eb7c    4c8b8f48060000           mov              r9, qword ptr [rdi + 0x648]
    0x14288eb83    488b8f78060000           mov              rcx, qword ptr [rdi + 0x678]
    0x14288eb8a    4883c110                 add              rcx, 0x10
    0x14288eb8e    488d15e3b68f00           lea              rdx, [rip + 0x8fb6e3] -> 0x14318a278
    0x14288eb95    4889542430               mov              qword ptr [rsp + 0x30], rdx
    0x14288eb9a    4c89742428               mov              qword ptr [rsp + 0x28], r14
    0x14288eb9f    48894c2420               mov              qword ptr [rsp + 0x20], rcx
    0x14288eba4    48c744243813000000       mov              qword ptr [rsp + 0x38], 0x13
    0x14288ebad    4c8d7550                 lea              r14, [rbp + 0x50]
    0x14288ebb1    4c89f1                   mov              rcx, r14
    0x14288ebb4    4889da                   mov              rdx, rbx
    0x14288ebb7    4989c0                   mov              r8, rax
    0x14288ebba    e841f635fe               call             0x140bee200 -> rhapsody::raw::buffer::_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$::create_buffer_impl::h34de0f745611d2ab
    0x14288ebbf    488b4550                 mov              rax, qword ptr [rbp + 0x50]
    0x14288ebc3    0f104558                 movups           xmm0, xmmword ptr [rbp + 0x58]
    0x14288ebc7    0f2985a0000000           movaps           xmmword ptr [rbp + 0xa0], xmm0
    0x14288ebce    0f104568                 movups           xmm0, xmmword ptr [rbp + 0x68]
    0x14288ebd2    0f2985b0000000           movaps           xmmword ptr [rbp + 0xb0], xmm0
    0x14288ebd9    0f104578                 movups           xmm0, xmmword ptr [rbp + 0x78]
    0x14288ebdd    0f2985c0000000           movaps           xmmword ptr [rbp + 0xc0], xmm0
    0x14288ebe4    488b9d88000000           mov              rbx, qword ptr [rbp + 0x88]
    0x14288ebeb    4c8bbd90000000           mov              r15, qword ptr [rbp + 0x90]
    0x14288ebf2    4883f80b                 cmp              rax, 0xb
    0x14288ebf6    0f859b000000             jne              0x14288ec97
    0x14288ebfc    0f2885a0000000           movaps           xmm0, xmmword ptr [rbp + 0xa0]
    0x14288ec03    0f288db0000000           movaps           xmm1, xmmword ptr [rbp + 0xb0]
    0x14288ec0a    0f2895c0000000           movaps           xmm2, xmmword ptr [rbp + 0xc0]
    0x14288ec11    0f295570                 movaps           xmmword ptr [rbp + 0x70], xmm2
    0x14288ec15    0f294d60                 movaps           xmmword ptr [rbp + 0x60], xmm1
    0x14288ec19    0f294550                 movaps           xmmword ptr [rbp + 0x50], xmm0
    0x14288ec1d    4885db                   test             rbx, rbx
    0x14288ec20    0f842e010000             je               0x14288ed54
    0x14288ec26    488d8dd0000000           lea              rcx, [rbp + 0xd0]
    0x14288ec2d    e86e6534fe               call             0x140bd51a0 -> rhapsody::raw::buffer::impl$4::byte_count
    0x14288ec32    4c39f8                   cmp              rax, r15
    0x14288ec35    0f8705010000             ja               0x14288ed40
    0x14288ec3b    4c8dbdd0000000           lea              r15, [rbp + 0xd0]
    0x14288ec42    4c89f9                   mov              rcx, r15
    0x14288ec45    4889da                   mov              rdx, rbx
    0x14288ec48    4989c0                   mov              r8, rax
    0x14288ec4b    e8606534fe               call             0x140bd51b0 -> rhapsody::raw::buffer::impl$4::copy_to_slice
    0x14288ec50    4c89b5a0000000           mov              qword ptr [rbp + 0xa0], r14
    0x14288ec57    4889bda8000000           mov              qword ptr [rbp + 0xa8], rdi
    0x14288ec5e    4c89a5b0000000           mov              qword ptr [rbp + 0xb0], r12
    0x14288ec65    4c89bdb8000000           mov              qword ptr [rbp + 0xb8], r15
    0x14288ec6c    488d4dd0                 lea              rcx, [rbp - 0x30]
    0x14288ec70    4c8d85a0000000           lea              r8, [rbp + 0xa0]
    0x14288ec77    4889fa                   mov              rdx, rdi
    0x14288ec7a    e8c15b37fe               call             0x140c04840 -> rhapsody::raw::device::GraphicsDevice::with_setup_cb::h946964bfafb47630
    0x14288ec7f    488b45d0                 mov              rax, qword ptr [rbp - 0x30]
    0x14288ec83    4883f80b                 cmp              rax, 0xb
    0x14288ec87    7547                     jne              0x14288ecd0
    0x14288ec89    488d4d58                 lea              rcx, [rbp + 0x58]
    0x14288ec8d    e81e3835fe               call             0x140be24b0 -> rhapsody::raw::managed::handle::impl$6::drop
    0x14288ec92    e95cfeffff               jmp              0x14288eaf3
    0x14288ec97    488b8d98000000           mov              rcx, qword ptr [rbp + 0x98]
    0x14288ec9e    0f2885a0000000           movaps           xmm0, xmmword ptr [rbp + 0xa0]
    0x14288eca5    0f288db0000000           movaps           xmm1, xmmword ptr [rbp + 0xb0]
    0x14288ecac    0f2895c0000000           movaps           xmm2, xmmword ptr [rbp + 0xc0]
    0x14288ecb3    0f115628                 movups           xmmword ptr [rsi + 0x28], xmm2
    0x14288ecb7    0f114e18                 movups           xmmword ptr [rsi + 0x18], xmm1
    0x14288ecbb    0f114608                 movups           xmmword ptr [rsi + 8], xmm0
    0x14288ecbf    488906                   mov              qword ptr [rsi], rax
    0x14288ecc2    48895e38                 mov              qword ptr [rsi + 0x38], rbx
    0x14288ecc6    4c897e40                 mov              qword ptr [rsi + 0x40], r15
    0x14288ecca    48894e48                 mov              qword ptr [rsi + 0x48], rcx
    0x14288ecce    eb34                     jmp              0x14288ed04
    0x14288ecd0    488b4d18                 mov              rcx, qword ptr [rbp + 0x18]
    0x14288ecd4    48894e48                 mov              qword ptr [rsi + 0x48], rcx
    0x14288ecd8    0f1045d8                 movups           xmm0, xmmword ptr [rbp - 0x28]
    0x14288ecdc    0f104de8                 movups           xmm1, xmmword ptr [rbp - 0x18]
    0x14288ece0    0f1055f8                 movups           xmm2, xmmword ptr [rbp - 8]
    0x14288ece4    0f105d08                 movups           xmm3, xmmword ptr [rbp + 8]
    0x14288ece8    0f115e38                 movups           xmmword ptr [rsi + 0x38], xmm3
    0x14288ecec    0f115628                 movups           xmmword ptr [rsi + 0x28], xmm2
    0x14288ecf0    0f114e18                 movups           xmmword ptr [rsi + 0x18], xmm1
    0x14288ecf4    0f114608                 movups           xmmword ptr [rsi + 8], xmm0
    0x14288ecf8    488906                   mov              qword ptr [rsi], rax
    0x14288ecfb    488d4d58                 lea              rcx, [rbp + 0x58]
    0x14288ecff    e8ac3735fe               call             0x140be24b0 -> rhapsody::raw::managed::handle::impl$6::drop
    0x14288ed04    488d4d28                 lea              rcx, [rbp + 0x28]
    0x14288ed08    e8a33735fe               call             0x140be24b0 -> rhapsody::raw::managed::handle::impl$6::drop
    0x14288ed0d    e939feffff               jmp              0x14288eb4b
    0x14288ed12    488d0d9fb48f00           lea              rcx, [rip + 0x8fb49f] -> 0x14318a1b8
    0x14288ed19    4c8d0528b58f00           lea              r8, [rip + 0x8fb528] -> 0x14318a248
    0x14288ed20    ba38000000               mov              edx, 0x38
    0x14288ed25    e806a70400               call             0x1428d9430 -> core::panicking::panic::hb210ae7b817a9f13
    0x14288ed2a    eb40                     jmp              0x14288ed6c
    0x14288ed2c    4c8d052db58f00           lea              r8, [rip + 0x8fb52d] -> 0x14318a260
    0x14288ed33    4889c1                   mov              rcx, rax
    0x14288ed36    4c89fa                   mov              rdx, r15
    0x14288ed39    e802ab0400               call             0x1428d9840 -> core::slice::index::slice_end_index_len_fail::hc3a65ec02a6eede1
    0x14288ed3e    eb2c                     jmp              0x14288ed6c
    0x14288ed40    4c8d0571b58f00           lea              r8, [rip + 0x8fb571] -> 0x14318a2b8
    0x14288ed47    4889c1                   mov              rcx, rax
    0x14288ed4a    4c89fa                   mov              rdx, r15
    0x14288ed4d    e8eeaa0400               call             0x1428d9840 -> core::slice::index::slice_end_index_len_fail::hc3a65ec02a6eede1
    0x14288ed52    eb18                     jmp              0x14288ed6c
    0x14288ed54    488d0d30b58f00           lea              rcx, [rip + 0x8fb530] -> 0x14318a28b
    0x14288ed5b    4c8d053eb58f00           lea              r8, [rip + 0x8fb53e] -> 0x14318a2a0
    0x14288ed62    ba14000000               mov              edx, 0x14
    0x14288ed67    e8c4a50400               call             0x1428d9330 -> core::option::expect_failed::hb3e848c6b16873c8
    0x14288ed6c    0f0b                     ud2              
