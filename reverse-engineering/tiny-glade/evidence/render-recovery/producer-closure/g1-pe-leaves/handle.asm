=== 0x1408fcd10 size=39  rhapsody::raw::managed::handle::impl$57::fmt
    ; impl$57::fmt: enum Debug by discriminant byte[rcx] via name table @0x142aaa198/0x142aaa1b0 = {0:Untracked,1:Tracked,2:MarkedUnused}. Managed-resource tracking state enum (3 variants).  [high]
    0x1408fcd10    4889d0                   mov              rax, rdx
    0x1408fcd13    0fb609                   movzx            ecx, byte ptr [rcx]
    0x1408fcd16    488d157bd41a02           lea              rdx, [rip + 0x21ad47b] -> 0x142aaa198
    0x1408fcd1d    4c8b04ca                 mov              r8, qword ptr [rdx + rcx*8]
    0x1408fcd21    4c8d0d88d41a02           lea              r9, [rip + 0x21ad488] -> 0x142aaa1b0
    0x1408fcd28    49631489                 movsxd           rdx, dword ptr [r9 + rcx*4]
    0x1408fcd2c    4c01ca                   add              rdx, r9
    0x1408fcd2f    4889c1                   mov              rcx, rax
    0x1408fcd32    e9599e4800               jmp              0x140d86b90 -> _$LT$core..fmt..Formatter$u20$as$u20$core..fmt..Write$GT$::write_str::h5c3f9ed06cfb23bd

=== 0x140bd69a0 size=202  rhapsody::raw::managed::handle::impl$3::fmt
    ; impl$3::fmt: Debug for RawManagedHandle. struct name str='RawManagedHandle'; fields: resource=[self+8], resource_checksum=[self+0x10], resource_type=[self+0x16], device_id=[self+0x14]. Real field names recovered from .rdata strings.  [high]
    0x140bd69a0    56                       push             rsi
    0x140bd69a1    57                       push             rdi
    0x140bd69a2    4883ec38                 sub              rsp, 0x38
    0x140bd69a6    4889ce                   mov              rsi, rcx
    0x140bd69a9    4c8d052496f301           lea              r8, [rip + 0x1f39624] -> 0x142b0ffd4
    0x140bd69b0    488d7c2428               lea              rdi, [rsp + 0x28]
    0x140bd69b5    41b910000000             mov              r9d, 0x10
    0x140bd69bb    4889f9                   mov              rcx, rdi
    0x140bd69be    e8ed011b00               call             0x140d86bb0 -> core::fmt::Formatter::debug_struct::h6f29e8a4a773d54a
    0x140bd69c3    4c8d4e08                 lea              r9, [rsi + 8]
    0x140bd69c7    488d051a96f301           lea              rax, [rip + 0x1f3961a] -> 0x142b0ffe8
    0x140bd69ce    4889442420               mov              qword ptr [rsp + 0x20], rax
    0x140bd69d3    488d152e96f301           lea              rdx, [rip + 0x1f3962e] -> 0x142b10008
    0x140bd69da    41b808000000             mov              r8d, 8
    0x140bd69e0    4889f9                   mov              rcx, rdi
    0x140bd69e3    e848c91a00               call             0x140d83330 -> core::fmt::builders::DebugStruct::field::ha1ce246cbe869fff
    0x140bd69e8    4c8d4e10                 lea              r9, [rsi + 0x10]
    0x140bd69ec    488d0d1d96f301           lea              rcx, [rip + 0x1f3961d] -> 0x142b10010
    0x140bd69f3    48894c2420               mov              qword ptr [rsp + 0x20], rcx
    0x140bd69f8    488d153196f301           lea              rdx, [rip + 0x1f39631] -> 0x142b10030
    0x140bd69ff    41b811000000             mov              r8d, 0x11
    0x140bd6a05    4889c1                   mov              rcx, rax
    0x140bd6a08    e823c91a00               call             0x140d83330 -> core::fmt::builders::DebugStruct::field::ha1ce246cbe869fff
    0x140bd6a0d    4c8d4e16                 lea              r9, [rsi + 0x16]
    0x140bd6a11    488d0d3096f301           lea              rcx, [rip + 0x1f39630] -> 0x142b10048
    0x140bd6a18    48894c2420               mov              qword ptr [rsp + 0x20], rcx
    0x140bd6a1d    488d154496f301           lea              rdx, [rip + 0x1f39644] -> 0x142b10068
    0x140bd6a24    41b80d000000             mov              r8d, 0xd
    0x140bd6a2a    4889c1                   mov              rcx, rax
    0x140bd6a2d    e8fec81a00               call             0x140d83330 -> core::fmt::builders::DebugStruct::field::ha1ce246cbe869fff
    0x140bd6a32    4883c614                 add              rsi, 0x14
    0x140bd6a36    488d0d3b96f301           lea              rcx, [rip + 0x1f3963b] -> 0x142b10078
    0x140bd6a3d    48894c2420               mov              qword ptr [rsp + 0x20], rcx
    0x140bd6a42    488d154f96f301           lea              rdx, [rip + 0x1f3964f] -> 0x142b10098
    0x140bd6a49    41b809000000             mov              r8d, 9
    0x140bd6a4f    4889c1                   mov              rcx, rax
    0x140bd6a52    4989f1                   mov              r9, rsi
    0x140bd6a55    e8d6c81a00               call             0x140d83330 -> core::fmt::builders::DebugStruct::field::ha1ce246cbe869fff
    0x140bd6a5a    4889c1                   mov              rcx, rax
    0x140bd6a5d    e82ecb1a00               call             0x140d83590 -> core::fmt::builders::DebugStruct::finish::h1d693b5fd291db0d
    0x140bd6a62    90                       nop              
    0x140bd6a63    4883c438                 add              rsp, 0x38
    0x140bd6a67    5f                       pop              rdi
    0x140bd6a68    5e                       pop              rsi
    0x140bd6a69    c3                       ret              

=== 0x140bd6b00 size=48  rhapsody::raw::managed::handle::impl$5::clone
    ; impl$5::clone (RawManagedHandle::clone): rdx=src, rcx=dst(returned). r8=inner=[src+8]; `lock inc dword [inner+8]` (atomic refcount++). Then copies src->dst: store@0, inner@8, u32 checksum@0x10, u16 device_id@0x14, u8 resource_type@0x16. NO allocation. Confirms refcount lives at inner+0x08.  [high]
    0x140bd6b00    4c8b4208                 mov              r8, qword ptr [rdx + 8]
    0x140bd6b04    f041ff4008               lock inc         dword ptr [r8 + 8]
    0x140bd6b09    4889c8                   mov              rax, rcx
    0x140bd6b0c    8b4a10                   mov              ecx, dword ptr [rdx + 0x10]
    0x140bd6b0f    440fb64a16               movzx            r9d, byte ptr [rdx + 0x16]
    0x140bd6b14    440fb75214               movzx            r10d, word ptr [rdx + 0x14]
    0x140bd6b19    488b12                   mov              rdx, qword ptr [rdx]
    0x140bd6b1c    4c894008                 mov              qword ptr [rax + 8], r8
    0x140bd6b20    894810                   mov              dword ptr [rax + 0x10], ecx
    0x140bd6b23    44884816                 mov              byte ptr [rax + 0x16], r9b
    0x140bd6b27    6644895014               mov              word ptr [rax + 0x14], r10w
    0x140bd6b2c    488910                   mov              qword ptr [rax], rdx
    0x140bd6b2f    c3                       ret              

=== 0x140bd6b30 size=23  rhapsody::raw::managed::handle::impl$7::eq
    ; impl$7::eq: al = ([a+8]==[b+8]) && (qword[a+0x0f]==qword[b+0x0f]). The two 8-byte compares together cover bytes 0x08..0x16 = resource(inner) + resource_checksum@0x10 + device_id@0x14 + resource_type@0x16. Field @0x00 (store) and 0x00..0x07 NOT compared.  [high]
    0x140bd6b30    488b4108                 mov              rax, qword ptr [rcx + 8]
    0x140bd6b34    488b490f                 mov              rcx, qword ptr [rcx + 0xf]
    0x140bd6b38    48334208                 xor              rax, qword ptr [rdx + 8]
    0x140bd6b3c    48334a0f                 xor              rcx, qword ptr [rdx + 0xf]
    0x140bd6b40    4809c1                   or               rcx, rax
    0x140bd6b43    0f94c0                   sete             al
    0x140bd6b46    c3                       ret              

=== 0x140bd6b50 size=56  rhapsody::raw::managed::handle::impl$36::fmt
    ; impl$36::fmt: Debug newtype 'RuntimeDeviceId' (tuple field1 of u16@+0x14). Name string from .rdata.  [high]
    0x140bd6b50    4883ec38                 sub              rsp, 0x38
    0x140bd6b54    4889d0                   mov              rax, rdx
    0x140bd6b57    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x140bd6b5c    488d0d4595f301           lea              rcx, [rip + 0x1f39545] -> 0x142b100a8
    0x140bd6b63    48894c2420               mov              qword ptr [rsp + 0x20], rcx
    0x140bd6b68    488d155995f301           lea              rdx, [rip + 0x1f39559] -> 0x142b100c8
    0x140bd6b6f    4c8d4c2430               lea              r9, [rsp + 0x30]
    0x140bd6b74    41b80f000000             mov              r8d, 0xf
    0x140bd6b7a    4889c1                   mov              rcx, rax
    0x140bd6b7d    e84e071b00               call             0x140d872d0 -> core::fmt::Formatter::debug_tuple_field1_finish::ha115499c8c7eba6f
    0x140bd6b82    90                       nop              
    0x140bd6b83    4883c438                 add              rsp, 0x38
    0x140bd6b87    c3                       ret              

=== 0x140bd6b90 size=56  rhapsody::raw::managed::handle::impl$44::fmt
    ; impl$44::fmt: Debug newtype 'ManagedResourceChecksum' (tuple field1 of checksum@+0x10). Name string from .rdata.  [high]
    0x140bd6b90    4883ec38                 sub              rsp, 0x38
    0x140bd6b94    4889d0                   mov              rax, rdx
    0x140bd6b97    48894c2430               mov              qword ptr [rsp + 0x30], rcx
    0x140bd6b9c    488d0d3595f301           lea              rcx, [rip + 0x1f39535] -> 0x142b100d8
    0x140bd6ba3    48894c2420               mov              qword ptr [rsp + 0x20], rcx
    0x140bd6ba8    488d154995f301           lea              rdx, [rip + 0x1f39549] -> 0x142b100f8
    0x140bd6baf    4c8d4c2430               lea              r9, [rsp + 0x30]
    0x140bd6bb4    41b817000000             mov              r8d, 0x17
    0x140bd6bba    4889c1                   mov              rcx, rax
    0x140bd6bbd    e80e071b00               call             0x140d872d0 -> core::fmt::Formatter::debug_tuple_field1_finish::ha115499c8c7eba6f
    0x140bd6bc2    90                       nop              
    0x140bd6bc3    4883c438                 add              rsp, 0x38
    0x140bd6bc7    c3                       ret              

=== 0x140bd6bd0 size=39  rhapsody::raw::managed::handle::impl$51::fmt
    ; impl$51::fmt: enum Debug by discriminant byte[rcx] via name table @0x142b10168/0x142b101a8 = {0:Texture,1:Buffer,2:Swapchain,3:TextureView,4:RenderPass,5:Sampler,6:PersistentDescriptorSet,7:Framebuffer}. This is the managed store resource-type tag; 8 consecutive variants (0..7).  [high]
    0x140bd6bd0    4889d0                   mov              rax, rdx
    0x140bd6bd3    0fb609                   movzx            ecx, byte ptr [rcx]
    0x140bd6bd6    488d158b95f301           lea              rdx, [rip + 0x1f3958b] -> 0x142b10168
    0x140bd6bdd    4c8b04ca                 mov              r8, qword ptr [rdx + rcx*8]
    0x140bd6be1    4c8d0dc095f301           lea              r9, [rip + 0x1f395c0] -> 0x142b101a8
    0x140bd6be8    49631489                 movsxd           rdx, dword ptr [r9 + rcx*4]
    0x140bd6bec    4c01ca                   add              rdx, r9
    0x140bd6bef    4889c1                   mov              rcx, rax
    0x140bd6bf2    e999ff1a00               jmp              0x140d86b90 -> _$LT$core..fmt..Formatter$u20$as$u20$core..fmt..Write$GT$::write_str::h5c3f9ed06cfb23bd

=== 0x141621b00 size=294  rhapsody::raw::managed::handle::impl$15::fmt<rhapsody::raw::managed::texture::ManagedTexture>
    ; impl$15::fmt<ManagedTexture>: Debug for the ManagedTexture handle specialization. format! of 'rhapsody::raw::managed::texture::ManagedTexture' (len 0x2f) then debug_struct field 'raw'. Diagnostic; body fully decoded.  [medium]
    0x141621b00    55                       push             rbp
    0x141621b01    56                       push             rsi
    0x141621b02    57                       push             rdi
    0x141621b03    53                       push             rbx
    0x141621b04    4881eca8000000           sub              rsp, 0xa8
    0x141621b0b    488dac2480000000         lea              rbp, [rsp + 0x80]
    0x141621b13    48c74520feffffff         mov              qword ptr [rbp + 0x20], 0xfffffffffffffffe
    0x141621b1b    4889d7                   mov              rdi, rdx
    0x141621b1e    4889ce                   mov              rsi, rcx
    0x141621b21    488d050bc64701           lea              rax, [rip + 0x147c60b] -> 0x142a9e133
    0x141621b28    488945e0                 mov              qword ptr [rbp - 0x20], rax
    0x141621b2c    48c745e82f000000         mov              qword ptr [rbp - 0x18], 0x2f
    0x141621b34    488d45e0                 lea              rax, [rbp - 0x20]
    0x141621b38    488945f0                 mov              qword ptr [rbp - 0x10], rax
    0x141621b3c    488d055d1df1fe           lea              rax, [rip - 0x10ee2a3] -> _$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$::fmt::h555cf7eaed744c42
    0x141621b43    488945f8                 mov              qword ptr [rbp - 8], rax
    0x141621b47    488d0502ed6501           lea              rax, [rip + 0x165ed02] -> 0x142c80850
    0x141621b4e    488945b0                 mov              qword ptr [rbp - 0x50], rax
    0x141621b52    48c745b802000000         mov              qword ptr [rbp - 0x48], 2
    0x141621b5a    488d45f0                 lea              rax, [rbp - 0x10]
    0x141621b5e    488945c0                 mov              qword ptr [rbp - 0x40], rax
    0x141621b62    48c745c801000000         mov              qword ptr [rbp - 0x38], 1
    0x141621b6a    48c745d000000000         mov              qword ptr [rbp - 0x30], 0
    0x141621b72    4889e9                   mov              rcx, rbp
    0x141621b75    488d55b0                 lea              rdx, [rbp - 0x50]
    0x141621b79    e8128475ff               call             0x140d79f90 -> alloc::fmt::format::format_inner::hdcc110f646744b43
    0x141621b7e    4c8b4508                 mov              r8, qword ptr [rbp + 8]
    0x141621b82    4c8b4d10                 mov              r9, qword ptr [rbp + 0x10]
    0x141621b86    488d4db0                 lea              rcx, [rbp - 0x50]
    0x141621b8a    4889fa                   mov              rdx, rdi
    0x141621b8d    4c894518                 mov              qword ptr [rbp + 0x18], r8
    0x141621b91    e81a5076ff               call             0x140d86bb0 -> core::fmt::Formatter::debug_struct::h6f29e8a4a773d54a
    0x141621b96    488d051be76501           lea              rax, [rip + 0x165e71b] -> 0x142c802b8
    0x141621b9d    4889442420               mov              qword ptr [rsp + 0x20], rax
    0x141621ba2    488d15c7ec6501           lea              rdx, [rip + 0x165ecc7] -> 0x142c80870
    0x141621ba9    488d4db0                 lea              rcx, [rbp - 0x50]
    0x141621bad    41b803000000             mov              r8d, 3
    0x141621bb3    4989f1                   mov              r9, rsi
    0x141621bb6    e8751776ff               call             0x140d83330 -> core::fmt::builders::DebugStruct::field::ha1ce246cbe869fff
    0x141621bbb    4889c1                   mov              rcx, rax
    0x141621bbe    e8cd1976ff               call             0x140d83590 -> core::fmt::builders::DebugStruct::finish::h1d693b5fd291db0d
    0x141621bc3    488b5500                 mov              rdx, qword ptr [rbp]
    0x141621bc7    4885d2                   test             rdx, rdx
    0x141621bca    7413                     je               0x141621bdf
    0x141621bcc    41b801000000             mov              r8d, 1
    0x141621bd2    488b4d18                 mov              rcx, qword ptr [rbp + 0x18]
    0x141621bd6    89c3                     mov              ebx, eax
    0x141621bd8    e84320fffe               call             0x140613c20 -> __rust_dealloc
    0x141621bdd    89d8                     mov              eax, ebx
    0x141621bdf    4881c4a8000000           add              rsp, 0xa8
    0x141621be6    5b                       pop              rbx
    0x141621be7    5f                       pop              rdi
    0x141621be8    5e                       pop              rsi
    0x141621be9    5d                       pop              rbp
    0x141621bea    c3                       ret              
    0x141621beb    0f1f440000               nop              dword ptr [rax + rax]
    0x141621bf0    4889542410               mov              qword ptr [rsp + 0x10], rdx
    0x141621bf5    55                       push             rbp
    0x141621bf6    56                       push             rsi
    0x141621bf7    57                       push             rdi
    0x141621bf8    53                       push             rbx
    0x141621bf9    4883ec28                 sub              rsp, 0x28
    0x141621bfd    488daa80000000           lea              rbp, [rdx + 0x80]
    0x141621c04    488b5500                 mov              rdx, qword ptr [rbp]
    0x141621c08    4885d2                   test             rdx, rdx
    0x141621c0b    740f                     je               0x141621c1c
    0x141621c0d    41b801000000             mov              r8d, 1
    0x141621c13    488b4d18                 mov              rcx, qword ptr [rbp + 0x18]
    0x141621c17    e80420fffe               call             0x140613c20 -> __rust_dealloc
    0x141621c1c    90                       nop              
    0x141621c1d    4883c428                 add              rsp, 0x28
    0x141621c21    5b                       pop              rbx
    0x141621c22    5f                       pop              rdi
    0x141621c23    5e                       pop              rsi
    0x141621c24    5d                       pop              rbp
    0x141621c25    c3                       ret              
