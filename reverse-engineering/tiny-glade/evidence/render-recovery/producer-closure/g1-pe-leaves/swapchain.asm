=== 0x140c0e7e0 size=4  rhapsody::raw::swapchain::Swapchain::format
    ; Swapchain::format: returns u32 [rcx+0x18].  [high]
    0x140c0e7e0    8b4118                   mov              eax, dword ptr [rcx + 0x18]
    0x140c0e7e3    c3                       ret              

=== 0x140c0ef70 size=5  rhapsody::raw::swapchain::Swapchain::any_present_successful
    ; Swapchain::any_present_successful: returns byte [rcx+0x1c].  [high]
    0x140c0ef70    0fb6411c                 movzx            eax, byte ptr [rcx + 0x1c]
    0x140c0ef74    c3                       ret              

=== 0x140c0f120 size=70  rhapsody::raw::swapchain::TrackedSwapchainTextureLease::image
    ; TrackedSwapchainTextureLease::image: clones the embedded RawManagedHandle: `lock inc dword [lease+0x18]+8` (refcount), copies lease+0x10..0x26 into the returned ManagedTexture handle, back-ptr = &(lease+0x28). Same refcount semantics as handle::clone.  [high]
    0x140c0f120    4889c8                   mov              rax, rcx
    0x140c0f123    488d4a28                 lea              rcx, [rdx + 0x28]
    0x140c0f127    4c8b4218                 mov              r8, qword ptr [rdx + 0x18]
    0x140c0f12b    0f1002                   movups           xmm0, xmmword ptr [rdx]
    0x140c0f12e    f041ff4008               lock inc         dword ptr [r8 + 8]
    0x140c0f133    448b4a20                 mov              r9d, dword ptr [rdx + 0x20]
    0x140c0f137    440fb65226               movzx            r10d, byte ptr [rdx + 0x26]
    0x140c0f13c    440fb75a24               movzx            r11d, word ptr [rdx + 0x24]
    0x140c0f141    488b5210                 mov              rdx, qword ptr [rdx + 0x10]
    0x140c0f145    4c894010                 mov              qword ptr [rax + 0x10], r8
    0x140c0f149    44894818                 mov              dword ptr [rax + 0x18], r9d
    0x140c0f14d    4488501e                 mov              byte ptr [rax + 0x1e], r10b
    0x140c0f151    664489581c               mov              word ptr [rax + 0x1c], r11w
    0x140c0f156    48895008                 mov              qword ptr [rax + 8], rdx
    0x140c0f15a    488908                   mov              qword ptr [rax], rcx
    0x140c0f15d    0f114020                 movups           xmmword ptr [rax + 0x20], xmm0
    0x140c0f161    c6403000                 mov              byte ptr [rax + 0x30], 0
    0x140c0f165    c3                       ret              

=== 0x1418cd0f0 size=224  rhapsody::raw::swapchain::impl$8::fmt
    ; impl$8::fmt: Debug for swapchain state enum; variants/fields 'SwapchainDesc','dims','vsync','exclusive','ZeroDims'. Diagnostic.  [medium]
    0x1418cd0f0    4881eca8000000           sub              rsp, 0xa8
    0x1418cd0f7    4889d0                   mov              rax, rdx
    0x1418cd0fa    488d5110                 lea              rdx, [rcx + 0x10]
    0x1418cd0fe    4c8d4118                 lea              r8, [rcx + 0x18]
    0x1418cd102    48898c24a0000000         mov              qword ptr [rsp + 0xa0], rcx
    0x1418cd10a    4883c120                 add              rcx, 0x20
    0x1418cd10e    4c8d0d8bb94301           lea              r9, [rip + 0x143b98b] -> 0x142d08aa0
    0x1418cd115    4c898c2490000000         mov              qword ptr [rsp + 0x90], r9
    0x1418cd11d    4c8d8c24a0000000         lea              r9, [rsp + 0xa0]
    0x1418cd125    4c898c2488000000         mov              qword ptr [rsp + 0x88], r9
    0x1418cd12d    4c8d0da2b94301           lea              r9, [rip + 0x143b9a2] -> 0x142d08ad6
    0x1418cd134    4c894c2478               mov              qword ptr [rsp + 0x78], r9
    0x1418cd139    4c8d0d40b94301           lea              r9, [rip + 0x143b940] -> 0x142d08a80
    0x1418cd140    4c894c2470               mov              qword ptr [rsp + 0x70], r9
    0x1418cd145    48894c2468               mov              qword ptr [rsp + 0x68], rcx
    0x1418cd14a    488d0d80b94301           lea              rcx, [rip + 0x143b980] -> 0x142d08ad1
    0x1418cd151    48894c2458               mov              qword ptr [rsp + 0x58], rcx
    0x1418cd156    488d0d03b94301           lea              rcx, [rip + 0x143b903] -> 0x142d08a60
    0x1418cd15d    48894c2450               mov              qword ptr [rsp + 0x50], rcx
    0x1418cd162    4c89442448               mov              qword ptr [rsp + 0x48], r8
    0x1418cd167    488d0d5fb94301           lea              rcx, [rip + 0x143b95f] -> 0x142d08acd
    0x1418cd16e    48894c2438               mov              qword ptr [rsp + 0x38], rcx
    0x1418cd173    488d0dc6b84301           lea              rcx, [rip + 0x143b8c6] -> 0x142d08a40
    0x1418cd17a    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x1418cd17f    4889542428               mov              qword ptr [rsp + 0x28], rdx
    0x1418cd184    48c784248000000009000000 mov              qword ptr [rsp + 0x80], 9
    0x1418cd190    48c744246005000000       mov              qword ptr [rsp + 0x60], 5
    0x1418cd199    48c744244004000000       mov              qword ptr [rsp + 0x40], 4
    0x1418cd1a2    48c744242006000000       mov              qword ptr [rsp + 0x20], 6
    0x1418cd1ab    488d150eb94301           lea              rdx, [rip + 0x143b90e] -> 0x142d08ac0
    0x1418cd1b2    4c8d0d17b24301           lea              r9, [rip + 0x143b217] -> 0x142d083d0
    0x1418cd1b9    41b80d000000             mov              r8d, 0xd
    0x1418cd1bf    4889c1                   mov              rcx, rax
    0x1418cd1c2    e8e99c4bff               call             0x140d86eb0 -> core::fmt::Formatter::debug_struct_field4_finish::hed15c7072f5fd9d1
    0x1418cd1c7    90                       nop              
    0x1418cd1c8    4881c4a8000000           add              rsp, 0xa8
    0x1418cd1cf    c3                       ret              

=== 0x1418cd1d0 size=321  rhapsody::raw::swapchain::impl$9::fmt
    ; impl$9::fmt: Debug for acquire-result enum; names 'Retired','Stale','desc','reason','ZeroDims','Vulkan' (+source path string). Diagnostic.  [medium]
    0x1418cd1d0    4883ec68                 sub              rsp, 0x68
    0x1418cd1d4    4889c8                   mov              rax, rcx
    0x1418cd1d7    488b09                   mov              rcx, qword ptr [rcx]
    0x1418cd1da    4883c1fe                 add              rcx, -2
    0x1418cd1de    4883f904                 cmp              rcx, 4
    0x1418cd1e2    41b801000000             mov              r8d, 1
    0x1418cd1e8    4c0f42c1                 cmovb            r8, rcx
    0x1418cd1ec    4889d1                   mov              rcx, rdx
    0x1418cd1ef    488d1592b94301           lea              rdx, [rip + 0x143b992] -> 0x142d08b88
    0x1418cd1f6    4e630482                 movsxd           r8, dword ptr [rdx + r8*4]
    0x1418cd1fa    4901d0                   add              r8, rdx
    0x1418cd1fd    41ffe0                   jmp              r8
    0x1418cd200    488d15d8b84301           lea              rdx, [rip + 0x143b8d8] -> 0x142d08adf
    0x1418cd207    41b808000000             mov              r8d, 8
    0x1418cd20d    4883c468                 add              rsp, 0x68
    0x1418cd211    e97a994bff               jmp              0x140d86b90 -> _$LT$core..fmt..Formatter$u20$as$u20$core..fmt..Write$GT$::write_str::h5c3f9ed06cfb23bd
    0x1418cd216    4883c008                 add              rax, 8
    0x1418cd21a    4889442460               mov              qword ptr [rsp + 0x60], rax
    0x1418cd21f    488d0512b94301           lea              rax, [rip + 0x143b912] -> 0x142d08b38
    0x1418cd226    4889442430               mov              qword ptr [rsp + 0x30], rax
    0x1418cd22b    488d442460               lea              rax, [rsp + 0x60]
    0x1418cd230    4889442428               mov              qword ptr [rsp + 0x28], rax
    0x1418cd235    48c744242004000000       mov              qword ptr [rsp + 0x20], 4
    0x1418cd23e    488d1513b94301           lea              rdx, [rip + 0x143b913] -> 0x142d08b58
    0x1418cd245    4c8d0de1b84301           lea              r9, [rip + 0x143b8e1] -> 0x142d08b2d
    0x1418cd24c    41b807000000             mov              r8d, 7
    0x1418cd252    e899994bff               call             0x140d86bf0 -> core::fmt::Formatter::debug_struct_field1_finish::hd9497ec2d71b9977
    0x1418cd257    90                       nop              
    0x1418cd258    4883c468                 add              rsp, 0x68
    0x1418cd25c    c3                       ret              
    0x1418cd25d    4c8d4818                 lea              r9, [rax + 0x18]
    0x1418cd261    4883c008                 add              rax, 8
    0x1418cd265    4889442460               mov              qword ptr [rsp + 0x60], rax
    0x1418cd26a    488d05efb84301           lea              rax, [rip + 0x143b8ef] -> 0x142d08b60
    0x1418cd271    4889442430               mov              qword ptr [rsp + 0x30], rax
    0x1418cd276    488d442460               lea              rax, [rsp + 0x60]
    0x1418cd27b    4889442428               mov              qword ptr [rsp + 0x28], rax
    0x1418cd280    488d0599ae4301           lea              rax, [rip + 0x143ae99] -> 0x142d08120
    0x1418cd287    4889442420               mov              qword ptr [rsp + 0x20], rax
    0x1418cd28c    488d15edb84301           lea              rdx, [rip + 0x143b8ed] -> 0x142d08b80
    0x1418cd293    41b806000000             mov              r8d, 6
    0x1418cd299    e892a14bff               call             0x140d87430 -> core::fmt::Formatter::debug_tuple_field2_finish::h278f357b51480b86
    0x1418cd29e    90                       nop              
    0x1418cd29f    4883c468                 add              rsp, 0x68
    0x1418cd2a3    c3                       ret              
    0x1418cd2a4    488d5028                 lea              rdx, [rax + 0x28]
    0x1418cd2a8    4889542460               mov              qword ptr [rsp + 0x60], rdx
    0x1418cd2ad    488d1554b84301           lea              rdx, [rip + 0x143b854] -> 0x142d08b08
    0x1418cd2b4    4889542450               mov              qword ptr [rsp + 0x50], rdx
    0x1418cd2b9    488d542460               lea              rdx, [rsp + 0x60]
    0x1418cd2be    4889542448               mov              qword ptr [rsp + 0x48], rdx
    0x1418cd2c3    488d1567b84301           lea              rdx, [rip + 0x143b867] -> 0x142d08b31
    0x1418cd2ca    4889542438               mov              qword ptr [rsp + 0x38], rdx
    0x1418cd2cf    488d1512b84301           lea              rdx, [rip + 0x143b812] -> 0x142d08ae8
    0x1418cd2d6    4889542430               mov              qword ptr [rsp + 0x30], rdx
    0x1418cd2db    4889442428               mov              qword ptr [rsp + 0x28], rax
    0x1418cd2e0    48c744244006000000       mov              qword ptr [rsp + 0x40], 6
    0x1418cd2e9    48c744242004000000       mov              qword ptr [rsp + 0x20], 4
    0x1418cd2f2    488d152fb84301           lea              rdx, [rip + 0x143b82f] -> 0x142d08b28
    0x1418cd2f9    4c8d0d2db84301           lea              r9, [rip + 0x143b82d] -> 0x142d08b2d
    0x1418cd300    41b805000000             mov              r8d, 5
    0x1418cd306    e895994bff               call             0x140d86ca0 -> core::fmt::Formatter::debug_struct_field2_finish::h35c7e6c960cc525a
    0x1418cd30b    90                       nop              
    0x1418cd30c    4883c468                 add              rsp, 0x68
    0x1418cd310    c3                       ret              
