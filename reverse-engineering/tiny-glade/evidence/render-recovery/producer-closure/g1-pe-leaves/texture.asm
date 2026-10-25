=== 0x1408eda30 size=280  rhapsody::raw::texture::impl$28::fmt
    ; impl$28::fmt: Debug for TextureDesc (struct 'TextureDesc' + kind variant names). Diagnostic.  [medium]
    0x1408eda30    56                       push             rsi
    0x1408eda31    57                       push             rdi
    0x1408eda32    4881ecc8000000           sub              rsp, 0xc8
    0x1408eda39    4889d0                   mov              rax, rdx
    0x1408eda3c    488d511e                 lea              rdx, [rcx + 0x1e]
    0x1408eda40    4c8d4104                 lea              r8, [rcx + 4]
    0x1408eda44    4c8d4908                 lea              r9, [rcx + 8]
    0x1408eda48    4c8d510c                 lea              r10, [rcx + 0xc]
    0x1408eda4c    4c8d591c                 lea              r11, [rcx + 0x1c]
    0x1408eda50    488d7118                 lea              rsi, [rcx + 0x18]
    0x1408eda54    488d791d                 lea              rdi, [rcx + 0x1d]
    0x1408eda58    48897c2440               mov              qword ptr [rsp + 0x40], rdi
    0x1408eda5d    4889542448               mov              qword ptr [rsp + 0x48], rdx
    0x1408eda62    488d15c7921b02           lea              rdx, [rip + 0x21b92c7] -> 0x142aa6d30
    0x1408eda69    4889542450               mov              qword ptr [rsp + 0x50], rdx
    0x1408eda6e    48894c2458               mov              qword ptr [rsp + 0x58], rcx
    0x1408eda73    488d0dd6921b02           lea              rcx, [rip + 0x21b92d6] -> 0x142aa6d50
    0x1408eda7a    48894c2460               mov              qword ptr [rsp + 0x60], rcx
    0x1408eda7f    4c89442468               mov              qword ptr [rsp + 0x68], r8
    0x1408eda84    488d0de5921b02           lea              rcx, [rip + 0x21b92e5] -> 0x142aa6d70
    0x1408eda8b    48894c2470               mov              qword ptr [rsp + 0x70], rcx
    0x1408eda90    4c894c2478               mov              qword ptr [rsp + 0x78], r9
    0x1408eda95    488d0df4921b02           lea              rcx, [rip + 0x21b92f4] -> 0x142aa6d90
    0x1408eda9c    48898c2480000000         mov              qword ptr [rsp + 0x80], rcx
    0x1408edaa4    4c89942488000000         mov              qword ptr [rsp + 0x88], r10
    0x1408edaac    488d0dfd921b02           lea              rcx, [rip + 0x21b92fd] -> 0x142aa6db0
    0x1408edab3    48898c2490000000         mov              qword ptr [rsp + 0x90], rcx
    0x1408edabb    4c899c2498000000         mov              qword ptr [rsp + 0x98], r11
    0x1408edac3    488d0d06931b02           lea              rcx, [rip + 0x21b9306] -> 0x142aa6dd0
    0x1408edaca    48898c24a0000000         mov              qword ptr [rsp + 0xa0], rcx
    0x1408edad2    4889b424a8000000         mov              qword ptr [rsp + 0xa8], rsi
    0x1408edada    488d0d0f931b02           lea              rcx, [rip + 0x21b930f] -> 0x142aa6df0
    0x1408edae1    48898c24b0000000         mov              qword ptr [rsp + 0xb0], rcx
    0x1408edae9    488d4c2440               lea              rcx, [rsp + 0x40]
    0x1408edaee    48898c24b8000000         mov              qword ptr [rsp + 0xb8], rcx
    0x1408edaf6    488d0d13931b02           lea              rcx, [rip + 0x21b9313] -> 0x142aa6e10
    0x1408edafd    48898c24c0000000         mov              qword ptr [rsp + 0xc0], rcx
    0x1408edb05    488d4c2448               lea              rcx, [rsp + 0x48]
    0x1408edb0a    48894c2428               mov              qword ptr [rsp + 0x28], rcx
    0x1408edb0f    48c744243008000000       mov              qword ptr [rsp + 0x30], 8
    0x1408edb18    48c744242008000000       mov              qword ptr [rsp + 0x20], 8
    0x1408edb21    488d15c8931b02           lea              rdx, [rip + 0x21b93c8] -> 0x142aa6ef0
    0x1408edb28    4c8d0d41931b02           lea              r9, [rip + 0x21b9341] -> 0x142aa6e70
    0x1408edb2f    41b80b000000             mov              r8d, 0xb
    0x1408edb35    4889c1                   mov              rcx, rax
    0x1408edb38    e823964900               call             0x140d87160 -> core::fmt::Formatter::debug_struct_fields_finish::haa31ff405975fe91
    0x1408edb3d    90                       nop              
    0x1408edb3e    4881c4c8000000           add              rsp, 0xc8
    0x1408edb45    5f                       pop              rdi
    0x1408edb46    5e                       pop              rsi
    0x1408edb47    c3                       ret              

=== 0x1408edb50 size=39  rhapsody::raw::texture::impl$14::fmt
    ; impl$14::fmt (etc, 4 instances): enum Debug by discriminant via name table = {0:Tex1d,1:Tex1dArray,2:Tex2d,3:Tex2dArray,4:Tex3d,5:Cube,6:CubeArray}. This is TextureDesc.kind; matches kind values 2/3/4 used by the constructors.  [high]
    0x1408edb50    4889d0                   mov              rax, rdx
    0x1408edb53    0fb609                   movzx            ecx, byte ptr [rcx]
    0x1408edb56    488d15dba71b02           lea              rdx, [rip + 0x21ba7db] -> 0x142aa8338
    0x1408edb5d    4c8b04ca                 mov              r8, qword ptr [rdx + rcx*8]
    0x1408edb61    4c8d0d08a81b02           lea              r9, [rip + 0x21ba808] -> 0x142aa8370
    0x1408edb68    49631489                 movsxd           rdx, dword ptr [r9 + rcx*4]
    0x1408edb6c    4c01ca                   add              rdx, r9
    0x1408edb6f    4889c1                   mov              rcx, rax
    0x1408edb72    e919904900               jmp              0x140d86b90 -> _$LT$core..fmt..Formatter$u20$as$u20$core..fmt..Write$GT$::write_str::h5c3f9ed06cfb23bd

=== 0x140bd5270 size=45  rhapsody::raw::texture::TextureDesc::new
    ; TextureDesc::new({w,h,d?}, kind=r8b, fmtarg=r9d): zero u64@0; u32@8:=r9d; w,h from [rdx]->@0xc,@0x10; u32@0x14:=[rdx+8]; array_elements@0x18:=1; u16@0x1c:=0x0101 (mip=1,samples=1); kind@0x1e:=r8b.  [high]
    0x140bd5270    4889c8                   mov              rax, rcx
    0x140bd5273    4488411e                 mov              byte ptr [rcx + 0x1e], r8b
    0x140bd5277    48c70100000000           mov              qword ptr [rcx], 0
    0x140bd527e    44894908                 mov              dword ptr [rcx + 8], r9d
    0x140bd5282    488b0a                   mov              rcx, qword ptr [rdx]
    0x140bd5285    4889480c                 mov              qword ptr [rax + 0xc], rcx
    0x140bd5289    8b4a08                   mov              ecx, dword ptr [rdx + 8]
    0x140bd528c    894814                   mov              dword ptr [rax + 0x14], ecx
    0x140bd528f    66c7401c0101             mov              word ptr [rax + 0x1c], 0x101
    0x140bd5295    c7401801000000           mov              dword ptr [rax + 0x18], 1
    0x140bd529c    c3                       ret              

=== 0x140bd52a0 size=49  rhapsody::raw::texture::TextureDesc::new_2d
    ; TextureDesc::new_2d(packed_wh edx, format r8d): u64@0=0; format@0x08=r8d; w=edx@0xc; h=edx>>32@0x10; depth@0x14=1; array_elements@0x18=1; mip@0x1c=1; samples@0x1d=1; kind@0x1e=2 (Tex2d).  [high]
    0x140bd52a0    4889c8                   mov              rax, rcx
    0x140bd52a3    48c70100000000           mov              qword ptr [rcx], 0
    0x140bd52aa    44894108                 mov              dword ptr [rcx + 8], r8d
    0x140bd52ae    89510c                   mov              dword ptr [rcx + 0xc], edx
    0x140bd52b1    48c1ea20                 shr              rdx, 0x20
    0x140bd52b5    895110                   mov              dword ptr [rcx + 0x10], edx
    0x140bd52b8    48b90100000001000000     movabs           rcx, 0x100000001
    0x140bd52c2    48894814                 mov              qword ptr [rax + 0x14], rcx
    0x140bd52c6    c6401c01                 mov              byte ptr [rax + 0x1c], 1
    0x140bd52ca    66c7401d0102             mov              word ptr [rax + 0x1d], 0x201
    0x140bd52d0    c3                       ret              

=== 0x140bd52e0 size=46  rhapsody::raw::texture::TextureDesc::new_2d_array
    ; TextureDesc::new_2d_array(packed_wh edx, format r9d, array_elements r8d): ...; @0x18=r8d; u16@0x1c=0x0101; kind@0x1e=3 (Tex2dArray).  [high]
    0x140bd52e0    4889c8                   mov              rax, rcx
    0x140bd52e3    48c70100000000           mov              qword ptr [rcx], 0
    0x140bd52ea    44894908                 mov              dword ptr [rcx + 8], r9d
    0x140bd52ee    89510c                   mov              dword ptr [rcx + 0xc], edx
    0x140bd52f1    48c1ea20                 shr              rdx, 0x20
    0x140bd52f5    895110                   mov              dword ptr [rcx + 0x10], edx
    0x140bd52f8    c7411401000000           mov              dword ptr [rcx + 0x14], 1
    0x140bd52ff    44894118                 mov              dword ptr [rcx + 0x18], r8d
    0x140bd5303    66c7411c0101             mov              word ptr [rcx + 0x1c], 0x101
    0x140bd5309    c6411e03                 mov              byte ptr [rcx + 0x1e], 3
    0x140bd530d    c3                       ret              

=== 0x140bd5310 size=45  rhapsody::raw::texture::TextureDesc::new_3d
    ; TextureDesc::new_3d({w,h,d}, format r8d): kind@0x1e=4 (Tex3d); depth from [rdx+8]@0x14.  [high]
    0x140bd5310    4889c8                   mov              rax, rcx
    0x140bd5313    c6411e04                 mov              byte ptr [rcx + 0x1e], 4
    0x140bd5317    48c70100000000           mov              qword ptr [rcx], 0
    0x140bd531e    44894108                 mov              dword ptr [rcx + 8], r8d
    0x140bd5322    488b0a                   mov              rcx, qword ptr [rdx]
    0x140bd5325    4889480c                 mov              qword ptr [rax + 0xc], rcx
    0x140bd5329    8b4a08                   mov              ecx, dword ptr [rdx + 8]
    0x140bd532c    894814                   mov              dword ptr [rax + 0x14], ecx
    0x140bd532f    66c7401c0101             mov              word ptr [rax + 0x1c], 0x101
    0x140bd5335    c7401801000000           mov              dword ptr [rax + 0x18], 1
    0x140bd533c    c3                       ret              

=== 0x140bd5340 size=33  rhapsody::raw::texture::TextureDesc::usage
    ; TextureDesc::usage(u32): copy self, set u32 usage@0x00 := arg.  [high]
    0x140bd5340    4889c8                   mov              rax, rcx
    0x140bd5343    448902                   mov              dword ptr [rdx], r8d
    0x140bd5346    0f104210                 movups           xmm0, xmmword ptr [rdx + 0x10]
    0x140bd534a    0f114110                 movups           xmmword ptr [rcx + 0x10], xmm0
    0x140bd534e    8b0a                     mov              ecx, dword ptr [rdx]
    0x140bd5350    8908                     mov              dword ptr [rax], ecx
    0x140bd5352    488b4a04                 mov              rcx, qword ptr [rdx + 4]
    0x140bd5356    48894804                 mov              qword ptr [rax + 4], rcx
    0x140bd535a    8b4a0c                   mov              ecx, dword ptr [rdx + 0xc]
    0x140bd535d    89480c                   mov              dword ptr [rax + 0xc], ecx
    0x140bd5360    c3                       ret              

=== 0x140bd5370 size=21  rhapsody::raw::texture::TextureDesc::sampled
    ; TextureDesc::sampled: self.usage@0x00 |= 0x4 (VK_IMAGE_USAGE_SAMPLED_BIT). Returns copy.  [high]
    0x140bd5370    4889c8                   mov              rax, rcx
    0x140bd5373    800a04                   or               byte ptr [rdx], 4
    0x140bd5376    0f104210                 movups           xmm0, xmmword ptr [rdx + 0x10]
    0x140bd537a    0f114110                 movups           xmmword ptr [rcx + 0x10], xmm0
    0x140bd537e    0f1002                   movups           xmm0, xmmword ptr [rdx]
    0x140bd5381    0f1101                   movups           xmmword ptr [rcx], xmm0
    0x140bd5384    c3                       ret              

=== 0x140bd5390 size=21  rhapsody::raw::texture::TextureDesc::storage
    ; TextureDesc::storage: usage@0x00 |= 0x8 (VK_IMAGE_USAGE_STORAGE_BIT).  [high]
    0x140bd5390    4889c8                   mov              rax, rcx
    0x140bd5393    800a08                   or               byte ptr [rdx], 8
    0x140bd5396    0f104210                 movups           xmm0, xmmword ptr [rdx + 0x10]
    0x140bd539a    0f114110                 movups           xmmword ptr [rcx + 0x10], xmm0
    0x140bd539e    0f1002                   movups           xmm0, xmmword ptr [rdx]
    0x140bd53a1    0f1101                   movups           xmmword ptr [rcx], xmm0
    0x140bd53a4    c3                       ret              

=== 0x140bd53b0 size=21  rhapsody::raw::texture::TextureDesc::transfer_dst
    ; TextureDesc::transfer_dst: usage@0x00 |= 0x2 (VK_IMAGE_USAGE_TRANSFER_DST_BIT).  [high]
    0x140bd53b0    4889c8                   mov              rax, rcx
    0x140bd53b3    800a02                   or               byte ptr [rdx], 2
    0x140bd53b6    0f104210                 movups           xmm0, xmmword ptr [rdx + 0x10]
    0x140bd53ba    0f114110                 movups           xmmword ptr [rcx + 0x10], xmm0
    0x140bd53be    0f1002                   movups           xmm0, xmmword ptr [rdx]
    0x140bd53c1    0f1101                   movups           xmmword ptr [rcx], xmm0
    0x140bd53c4    c3                       ret              

=== 0x140bd53d0 size=21  rhapsody::raw::texture::TextureDesc::color_attachment
    ; TextureDesc::color_attachment: usage@0x00 |= 0x10 (VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT).  [high]
    0x140bd53d0    4889c8                   mov              rax, rcx
    0x140bd53d3    800a10                   or               byte ptr [rdx], 0x10
    0x140bd53d6    0f104210                 movups           xmm0, xmmword ptr [rdx + 0x10]
    0x140bd53da    0f114110                 movups           xmmword ptr [rcx + 0x10], xmm0
    0x140bd53de    0f1002                   movups           xmm0, xmmword ptr [rdx]
    0x140bd53e1    0f1101                   movups           xmmword ptr [rcx], xmm0
    0x140bd53e4    c3                       ret              

=== 0x140bd53f0 size=21  rhapsody::raw::texture::TextureDesc::depth_stencil_attachment
    ; TextureDesc::depth_stencil_attachment: usage@0x00 |= 0x20 (VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT).  [high]
    0x140bd53f0    4889c8                   mov              rax, rcx
    0x140bd53f3    800a20                   or               byte ptr [rdx], 0x20
    0x140bd53f6    0f104210                 movups           xmm0, xmmword ptr [rdx + 0x10]
    0x140bd53fa    0f114110                 movups           xmmword ptr [rcx + 0x10], xmm0
    0x140bd53fe    0f1002                   movups           xmm0, xmmword ptr [rdx]
    0x140bd5401    0f1101                   movups           xmmword ptr [rcx], xmm0
    0x140bd5404    c3                       ret              

=== 0x140bd5410 size=31  rhapsody::raw::texture::TextureDesc::sampled_with_different_format
    ; TextureDesc::sampled_with_different_format: usage@0x00 (u64) |= 0x10800000004 => bits 2 + 35 + 40.  [high]
    0x140bd5410    4889c8                   mov              rax, rcx
    0x140bd5413    48b90400000008010000     movabs           rcx, 0x10800000004
    0x140bd541d    48090a                   or               qword ptr [rdx], rcx
    0x140bd5420    0f104210                 movups           xmm0, xmmword ptr [rdx + 0x10]
    0x140bd5424    0f114010                 movups           xmmword ptr [rax + 0x10], xmm0
    0x140bd5428    0f1002                   movups           xmm0, xmmword ptr [rdx]
    0x140bd542b    0f1100                   movups           xmmword ptr [rax], xmm0
    0x140bd542e    c3                       ret              

=== 0x140bd5430 size=34  rhapsody::raw::texture::TextureDesc::format
    ; TextureDesc::format(u32): copy self, set u32 format@0x08 := arg.  [high]
    0x140bd5430    4889c8                   mov              rax, rcx
    0x140bd5433    44894208                 mov              dword ptr [rdx + 8], r8d
    0x140bd5437    0f104210                 movups           xmm0, xmmword ptr [rdx + 0x10]
    0x140bd543b    0f114110                 movups           xmmword ptr [rcx + 0x10], xmm0
    0x140bd543f    488b0a                   mov              rcx, qword ptr [rdx]
    0x140bd5442    488908                   mov              qword ptr [rax], rcx
    0x140bd5445    8b4a08                   mov              ecx, dword ptr [rdx + 8]
    0x140bd5448    894808                   mov              dword ptr [rax + 8], ecx
    0x140bd544b    8b4a0c                   mov              ecx, dword ptr [rdx + 0xc]
    0x140bd544e    89480c                   mov              dword ptr [rax + 0xc], ecx
    0x140bd5451    c3                       ret              

=== 0x140bd5460 size=50  rhapsody::raw::texture::TextureDesc::mip_levels
    ; TextureDesc::mip_levels(u8): copy self, set byte mip_levels@0x1c := arg. New precise field offset (mip_levels@0x1c).  [high]
    0x140bd5460    4889c8                   mov              rax, rcx
    0x140bd5463    4488421c                 mov              byte ptr [rdx + 0x1c], r8b
    0x140bd5467    0f1002                   movups           xmm0, xmmword ptr [rdx]
    0x140bd546a    0f1101                   movups           xmmword ptr [rcx], xmm0
    0x140bd546d    488b4a10                 mov              rcx, qword ptr [rdx + 0x10]
    0x140bd5471    48894810                 mov              qword ptr [rax + 0x10], rcx
    0x140bd5475    8b4a18                   mov              ecx, dword ptr [rdx + 0x18]
    0x140bd5478    894818                   mov              dword ptr [rax + 0x18], ecx
    0x140bd547b    0fb64a1c                 movzx            ecx, byte ptr [rdx + 0x1c]
    0x140bd547f    88481c                   mov              byte ptr [rax + 0x1c], cl
    0x140bd5482    0fb74a1d                 movzx            ecx, word ptr [rdx + 0x1d]
    0x140bd5486    6689481d                 mov              word ptr [rax + 0x1d], cx
    0x140bd548a    0fb64a1f                 movzx            ecx, byte ptr [rdx + 0x1f]
    0x140bd548e    88481f                   mov              byte ptr [rax + 0x1f], cl
    0x140bd5491    c3                       ret              

=== 0x140bd54a0 size=160  rhapsody::raw::texture::TextureDesc::all_mip_levels
    ; TextureDesc::all_mip_levels: computes full mip chain length from extent (log2). Decoded body available.  [medium]
    0x140bd54a0    4889c8                   mov              rax, rcx
    0x140bd54a3    8b4a0c                   mov              ecx, dword ptr [rdx + 0xc]
    0x140bd54a6    448b4a10                 mov              r9d, dword ptr [rdx + 0x10]
    0x140bd54aa    41b820000000             mov              r8d, 0x20
    0x140bd54b0    41ba20000000             mov              r10d, 0x20
    0x140bd54b6    85c9                     test             ecx, ecx
    0x140bd54b8    7408                     je               0x140bd54c2
    0x140bd54ba    440fbdd1                 bsr              r10d, ecx
    0x140bd54be    4183f21f                 xor              r10d, 0x1f
    0x140bd54c2    448b5a14                 mov              r11d, dword ptr [rdx + 0x14]
    0x140bd54c6    b120                     mov              cl, 0x20
    0x140bd54c8    4585c9                   test             r9d, r9d
    0x140bd54cb    7408                     je               0x140bd54d5
    0x140bd54cd    450fbdc1                 bsr              r8d, r9d
    0x140bd54d1    4183f01f                 xor              r8d, 0x1f
    0x140bd54d5    41b120                   mov              r9b, 0x20
    0x140bd54d8    4528d1                   sub              r9b, r10b
    0x140bd54db    4428c1                   sub              cl, r8b
    0x140bd54de    4585db                   test             r11d, r11d
    0x140bd54e1    740a                     je               0x140bd54ed
    0x140bd54e3    450fbdc3                 bsr              r8d, r11d
    0x140bd54e7    4183f01f                 xor              r8d, 0x1f
    0x140bd54eb    eb06                     jmp              0x140bd54f3
    0x140bd54ed    41b820000000             mov              r8d, 0x20
    0x140bd54f3    41b220                   mov              r10b, 0x20
    0x140bd54f6    4528c2                   sub              r10b, r8b
    0x140bd54f9    450fb6c2                 movzx            r8d, r10b
    0x140bd54fd    0fb6c9                   movzx            ecx, cl
    0x140bd5500    4138c8                   cmp              r8b, cl
    0x140bd5503    410f47c8                 cmova            ecx, r8d
    0x140bd5507    450fb6c1                 movzx            r8d, r9b
    0x140bd550b    4438c1                   cmp              cl, r8b
    0x140bd550e    410f46c8                 cmovbe           ecx, r8d
    0x140bd5512    884a1c                   mov              byte ptr [rdx + 0x1c], cl
    0x140bd5515    0f1002                   movups           xmm0, xmmword ptr [rdx]
    0x140bd5518    0f1100                   movups           xmmword ptr [rax], xmm0
    0x140bd551b    488b4a10                 mov              rcx, qword ptr [rdx + 0x10]
    0x140bd551f    48894810                 mov              qword ptr [rax + 0x10], rcx
    0x140bd5523    8b4a18                   mov              ecx, dword ptr [rdx + 0x18]
    0x140bd5526    894818                   mov              dword ptr [rax + 0x18], ecx
    0x140bd5529    0fb64a1c                 movzx            ecx, byte ptr [rdx + 0x1c]
    0x140bd552d    88481c                   mov              byte ptr [rax + 0x1c], cl
    0x140bd5530    0fb74a1d                 movzx            ecx, word ptr [rdx + 0x1d]
    0x140bd5534    6689481d                 mov              word ptr [rax + 0x1d], cx
    0x140bd5538    0fb64a1f                 movzx            ecx, byte ptr [rdx + 0x1f]
    0x140bd553c    88481f                   mov              byte ptr [rax + 0x1f], cl
    0x140bd553f    c3                       ret              

=== 0x140bd55a0 size=50  rhapsody::raw::texture::TextureDesc::sample_count
    ; TextureDesc::sample_count(u8): copy self, set byte sample_count@0x1d := arg. New precise field offset.  [high]
    0x140bd55a0    4889c8                   mov              rax, rcx
    0x140bd55a3    4488421d                 mov              byte ptr [rdx + 0x1d], r8b
    0x140bd55a7    0f1002                   movups           xmm0, xmmword ptr [rdx]
    0x140bd55aa    0f1101                   movups           xmmword ptr [rcx], xmm0
    0x140bd55ad    488b4a10                 mov              rcx, qword ptr [rdx + 0x10]
    0x140bd55b1    48894810                 mov              qword ptr [rax + 0x10], rcx
    0x140bd55b5    8b4a18                   mov              ecx, dword ptr [rdx + 0x18]
    0x140bd55b8    894818                   mov              dword ptr [rax + 0x18], ecx
    0x140bd55bb    0fb64a1c                 movzx            ecx, byte ptr [rdx + 0x1c]
    0x140bd55bf    88481c                   mov              byte ptr [rax + 0x1c], cl
    0x140bd55c2    0fb64a1d                 movzx            ecx, byte ptr [rdx + 0x1d]
    0x140bd55c6    88481d                   mov              byte ptr [rax + 0x1d], cl
    0x140bd55c9    0fb74a1e                 movzx            ecx, word ptr [rdx + 0x1e]
    0x140bd55cd    6689481e                 mov              word ptr [rax + 0x1e], cx
    0x140bd55d1    c3                       ret              

=== 0x140bd5730 size=86  rhapsody::raw::texture::TextureDesc::half_res
    ; TextureDesc::half_res: halves width/height (max(1,...)). Decoded body available.  [medium]
    0x140bd5730    4889c8                   mov              rax, rcx
    0x140bd5733    f30f7e420c               movq             xmm0, qword ptr [rdx + 0xc]
    0x140bd5738    660f6fc8                 movdqa           xmm1, xmm0
    0x140bd573c    660f72d101               psrld            xmm1, 1
    0x140bd5741    660ffac1                 psubd            xmm0, xmm1
    0x140bd5745    660fefc9                 pxor             xmm1, xmm1
    0x140bd5749    660f76c8                 pcmpeqd          xmm1, xmm0
    0x140bd574d    660ffac1                 psubd            xmm0, xmm1
    0x140bd5751    660fd6420c               movq             qword ptr [rdx + 0xc], xmm0
    0x140bd5756    8b4a14                   mov              ecx, dword ptr [rdx + 0x14]
    0x140bd5759    4189c8                   mov              r8d, ecx
    0x140bd575c    41d1e8                   shr              r8d, 1
    0x140bd575f    4429c1                   sub              ecx, r8d
    0x140bd5762    83f901                   cmp              ecx, 1
    0x140bd5765    83d100                   adc              ecx, 0
    0x140bd5768    894a14                   mov              dword ptr [rdx + 0x14], ecx
    0x140bd576b    8b4a10                   mov              ecx, dword ptr [rdx + 0x10]
    0x140bd576e    894810                   mov              dword ptr [rax + 0x10], ecx
    0x140bd5771    8b4a14                   mov              ecx, dword ptr [rdx + 0x14]
    0x140bd5774    894814                   mov              dword ptr [rax + 0x14], ecx
    0x140bd5777    488b4a18                 mov              rcx, qword ptr [rdx + 0x18]
    0x140bd577b    48894818                 mov              qword ptr [rax + 0x18], rcx
    0x140bd577f    0f1002                   movups           xmm0, xmmword ptr [rdx]
    0x140bd5782    0f1100                   movups           xmmword ptr [rax], xmm0
    0x140bd5785    c3                       ret              

=== 0x140bd5790 size=5  rhapsody::raw::texture::TextureDesc::extent_2d
    ; TextureDesc::extent_2d: returns qword [rcx+0xc] = packed (width u32@0xc, height u32@0x10).  [high]
    0x140bd5790    488b410c                 mov              rax, qword ptr [rcx + 0xc]
    0x140bd5794    c3                       ret              

=== 0x140bd57a0 size=57  rhapsody::raw::texture::TextureViewDesc::new
    ; TextureViewDesc::new: zeroes six dwords at 0x00,0x08,0x10,0x18,0x20,0x28 (six 8-byte Option-like {disc@X,val@X+4} fields), qword@0x30=0, byte@0x38=0. TextureViewDesc ~0x3cB.  [high]
    0x140bd57a0    4889c8                   mov              rax, rcx
    0x140bd57a3    c70100000000             mov              dword ptr [rcx], 0
    0x140bd57a9    c7410800000000           mov              dword ptr [rcx + 8], 0
    0x140bd57b0    c7411000000000           mov              dword ptr [rcx + 0x10], 0
    0x140bd57b7    c7411800000000           mov              dword ptr [rcx + 0x18], 0
    0x140bd57be    c7412000000000           mov              dword ptr [rcx + 0x20], 0
    0x140bd57c5    c7412800000000           mov              dword ptr [rcx + 0x28], 0
    0x140bd57cc    48c7413000000000         mov              qword ptr [rcx + 0x30], 0
    0x140bd57d4    c6413800                 mov              byte ptr [rcx + 0x38], 0
    0x140bd57d8    c3                       ret              

=== 0x140bd57e0 size=56  rhapsody::raw::texture::TextureViewDesc::view_type
    ; TextureViewDesc::view_type(u32): copy; set disc@0x00=1, val@0x04:=arg.  [high]
    0x140bd57e0    4889c8                   mov              rax, rcx
    0x140bd57e3    c70201000000             mov              dword ptr [rdx], 1
    0x140bd57e9    44894204                 mov              dword ptr [rdx + 4], r8d
    0x140bd57ed    0f104210                 movups           xmm0, xmmword ptr [rdx + 0x10]
    0x140bd57f1    0f104a20                 movups           xmm1, xmmword ptr [rdx + 0x20]
    0x140bd57f5    0f10522c                 movups           xmm2, xmmword ptr [rdx + 0x2c]
    0x140bd57f9    0f11512c                 movups           xmmword ptr [rcx + 0x2c], xmm2
    0x140bd57fd    0f114920                 movups           xmmword ptr [rcx + 0x20], xmm1
    0x140bd5801    0f114110                 movups           xmmword ptr [rcx + 0x10], xmm0
    0x140bd5805    8b0a                     mov              ecx, dword ptr [rdx]
    0x140bd5807    8908                     mov              dword ptr [rax], ecx
    0x140bd5809    8b4a04                   mov              ecx, dword ptr [rdx + 4]
    0x140bd580c    894804                   mov              dword ptr [rax + 4], ecx
    0x140bd580f    488b4a08                 mov              rcx, qword ptr [rdx + 8]
    0x140bd5813    48894808                 mov              qword ptr [rax + 8], rcx
    0x140bd5817    c3                       ret              

=== 0x140bd5820 size=57  rhapsody::raw::texture::TextureViewDesc::usage
    ; TextureViewDesc::usage(u32): copy; set disc@0x08=1, val@0x0c:=arg.  [high]
    0x140bd5820    4889c8                   mov              rax, rcx
    0x140bd5823    c7420801000000           mov              dword ptr [rdx + 8], 1
    0x140bd582a    4489420c                 mov              dword ptr [rdx + 0xc], r8d
    0x140bd582e    0f104210                 movups           xmm0, xmmword ptr [rdx + 0x10]
    0x140bd5832    0f104a20                 movups           xmm1, xmmword ptr [rdx + 0x20]
    0x140bd5836    0f10522c                 movups           xmm2, xmmword ptr [rdx + 0x2c]
    0x140bd583a    0f11512c                 movups           xmmword ptr [rcx + 0x2c], xmm2
    0x140bd583e    0f114920                 movups           xmmword ptr [rcx + 0x20], xmm1
    0x140bd5842    0f114110                 movups           xmmword ptr [rcx + 0x10], xmm0
    0x140bd5846    488b0a                   mov              rcx, qword ptr [rdx]
    0x140bd5849    488908                   mov              qword ptr [rax], rcx
    0x140bd584c    8b4a08                   mov              ecx, dword ptr [rdx + 8]
    0x140bd584f    894808                   mov              dword ptr [rax + 8], ecx
    0x140bd5852    8b4a0c                   mov              ecx, dword ptr [rdx + 0xc]
    0x140bd5855    89480c                   mov              dword ptr [rax + 0xc], ecx
    0x140bd5858    c3                       ret              

=== 0x140bd5860 size=57  rhapsody::raw::texture::TextureViewDesc::format
    ; TextureViewDesc::format(u32): copy; set disc@0x10=1, val@0x14:=arg.  [high]
    0x140bd5860    4889c8                   mov              rax, rcx
    0x140bd5863    c7421001000000           mov              dword ptr [rdx + 0x10], 1
    0x140bd586a    44894214                 mov              dword ptr [rdx + 0x14], r8d
    0x140bd586e    0f1002                   movups           xmm0, xmmword ptr [rdx]
    0x140bd5871    0f104a20                 movups           xmm1, xmmword ptr [rdx + 0x20]
    0x140bd5875    0f10522c                 movups           xmm2, xmmword ptr [rdx + 0x2c]
    0x140bd5879    0f11512c                 movups           xmmword ptr [rcx + 0x2c], xmm2
    0x140bd587d    0f114920                 movups           xmmword ptr [rcx + 0x20], xmm1
    0x140bd5881    0f1101                   movups           xmmword ptr [rcx], xmm0
    0x140bd5884    8b4a10                   mov              ecx, dword ptr [rdx + 0x10]
    0x140bd5887    894810                   mov              dword ptr [rax + 0x10], ecx
    0x140bd588a    8b4a14                   mov              ecx, dword ptr [rdx + 0x14]
    0x140bd588d    894814                   mov              dword ptr [rax + 0x14], ecx
    0x140bd5890    488b4a18                 mov              rcx, qword ptr [rdx + 0x18]
    0x140bd5894    48894818                 mov              qword ptr [rax + 0x18], rcx
    0x140bd5898    c3                       ret              

=== 0x140bd58a0 size=57  rhapsody::raw::texture::TextureViewDesc::aspect_mask
    ; TextureViewDesc::aspect_mask(u32): copy; set disc@0x18=1, val@0x1c:=arg.  [high]
    0x140bd58a0    4889c8                   mov              rax, rcx
    0x140bd58a3    c7421801000000           mov              dword ptr [rdx + 0x18], 1
    0x140bd58aa    4489421c                 mov              dword ptr [rdx + 0x1c], r8d
    0x140bd58ae    0f1002                   movups           xmm0, xmmword ptr [rdx]
    0x140bd58b1    0f104a20                 movups           xmm1, xmmword ptr [rdx + 0x20]
    0x140bd58b5    0f10522c                 movups           xmm2, xmmword ptr [rdx + 0x2c]
    0x140bd58b9    0f11512c                 movups           xmmword ptr [rcx + 0x2c], xmm2
    0x140bd58bd    0f114920                 movups           xmmword ptr [rcx + 0x20], xmm1
    0x140bd58c1    0f1101                   movups           xmmword ptr [rcx], xmm0
    0x140bd58c4    488b4a10                 mov              rcx, qword ptr [rdx + 0x10]
    0x140bd58c8    48894810                 mov              qword ptr [rax + 0x10], rcx
    0x140bd58cc    8b4a18                   mov              ecx, dword ptr [rdx + 0x18]
    0x140bd58cf    894818                   mov              dword ptr [rax + 0x18], ecx
    0x140bd58d2    8b4a1c                   mov              ecx, dword ptr [rdx + 0x1c]
    0x140bd58d5    89481c                   mov              dword ptr [rax + 0x1c], ecx
    0x140bd58d8    c3                       ret              

=== 0x140bd58e0 size=50  rhapsody::raw::texture::TextureViewDesc::base_mip_level
    ; TextureViewDesc::base_mip_level(u32): copy; set u32@0x30:=arg.  [high]
    0x140bd58e0    4889c8                   mov              rax, rcx
    0x140bd58e3    44894230                 mov              dword ptr [rdx + 0x30], r8d
    0x140bd58e7    0f1002                   movups           xmm0, xmmword ptr [rdx]
    0x140bd58ea    0f104a10                 movups           xmm1, xmmword ptr [rdx + 0x10]
    0x140bd58ee    0f105220                 movups           xmm2, xmmword ptr [rdx + 0x20]
    0x140bd58f2    0f115120                 movups           xmmword ptr [rcx + 0x20], xmm2
    0x140bd58f6    0f114910                 movups           xmmword ptr [rcx + 0x10], xmm1
    0x140bd58fa    0f1101                   movups           xmmword ptr [rcx], xmm0
    0x140bd58fd    8b4a2c                   mov              ecx, dword ptr [rdx + 0x2c]
    0x140bd5900    89482c                   mov              dword ptr [rax + 0x2c], ecx
    0x140bd5903    8b4a30                   mov              ecx, dword ptr [rdx + 0x30]
    0x140bd5906    894830                   mov              dword ptr [rax + 0x30], ecx
    0x140bd5909    488b4a34                 mov              rcx, qword ptr [rdx + 0x34]
    0x140bd590d    48894834                 mov              qword ptr [rax + 0x34], rcx
    0x140bd5911    c3                       ret              

=== 0x140bd5920 size=54  rhapsody::raw::texture::TextureViewDesc::level_count
    ; TextureViewDesc::level_count(u32,u32): copy; set u32@0x20:=arg1, u32@0x24:=arg2.  [medium]
    0x140bd5920    4889c8                   mov              rax, rcx
    0x140bd5923    44894220                 mov              dword ptr [rdx + 0x20], r8d
    0x140bd5927    44894a24                 mov              dword ptr [rdx + 0x24], r9d
    0x140bd592b    0f1002                   movups           xmm0, xmmword ptr [rdx]
    0x140bd592e    0f104a10                 movups           xmm1, xmmword ptr [rdx + 0x10]
    0x140bd5932    0f10522c                 movups           xmm2, xmmword ptr [rdx + 0x2c]
    0x140bd5936    0f11512c                 movups           xmmword ptr [rcx + 0x2c], xmm2
    0x140bd593a    0f114910                 movups           xmmword ptr [rcx + 0x10], xmm1
    0x140bd593e    0f1101                   movups           xmmword ptr [rcx], xmm0
    0x140bd5941    8b4a20                   mov              ecx, dword ptr [rdx + 0x20]
    0x140bd5944    894820                   mov              dword ptr [rax + 0x20], ecx
    0x140bd5947    8b4a24                   mov              ecx, dword ptr [rdx + 0x24]
    0x140bd594a    894824                   mov              dword ptr [rax + 0x24], ecx
    0x140bd594d    488b4a28                 mov              rcx, qword ptr [rdx + 0x28]
    0x140bd5951    48894828                 mov              qword ptr [rax + 0x28], rcx
    0x140bd5955    c3                       ret              

=== 0x140bd5960 size=50  rhapsody::raw::texture::TextureViewDesc::base_layer
    ; TextureViewDesc::base_layer(u32): copy; set u32@0x34:=arg.  [high]
    0x140bd5960    4889c8                   mov              rax, rcx
    0x140bd5963    44894234                 mov              dword ptr [rdx + 0x34], r8d
    0x140bd5967    0f1002                   movups           xmm0, xmmword ptr [rdx]
    0x140bd596a    0f104a10                 movups           xmm1, xmmword ptr [rdx + 0x10]
    0x140bd596e    0f105220                 movups           xmm2, xmmword ptr [rdx + 0x20]
    0x140bd5972    0f115120                 movups           xmmword ptr [rcx + 0x20], xmm2
    0x140bd5976    0f114910                 movups           xmmword ptr [rcx + 0x10], xmm1
    0x140bd597a    0f1101                   movups           xmmword ptr [rcx], xmm0
    0x140bd597d    488b4a2c                 mov              rcx, qword ptr [rdx + 0x2c]
    0x140bd5981    4889482c                 mov              qword ptr [rax + 0x2c], rcx
    0x140bd5985    8b4a34                   mov              ecx, dword ptr [rdx + 0x34]
    0x140bd5988    894834                   mov              dword ptr [rax + 0x34], ecx
    0x140bd598b    8b4a38                   mov              ecx, dword ptr [rdx + 0x38]
    0x140bd598e    894838                   mov              dword ptr [rax + 0x38], ecx
    0x140bd5991    c3                       ret              

=== 0x140bd59a0 size=66  rhapsody::raw::texture::TextureViewDesc::layer_count
    ; TextureViewDesc::layer_count(u32,u32): copy; set u32@0x28:=arg1, u32@0x2c:=arg2.  [medium]
    0x140bd59a0    4889c8                   mov              rax, rcx
    0x140bd59a3    44894228                 mov              dword ptr [rdx + 0x28], r8d
    0x140bd59a7    44894a2c                 mov              dword ptr [rdx + 0x2c], r9d
    0x140bd59ab    0f1002                   movups           xmm0, xmmword ptr [rdx]
    0x140bd59ae    0f104a10                 movups           xmm1, xmmword ptr [rdx + 0x10]
    0x140bd59b2    0f114910                 movups           xmmword ptr [rcx + 0x10], xmm1
    0x140bd59b6    0f1101                   movups           xmmword ptr [rcx], xmm0
    0x140bd59b9    8b4a2c                   mov              ecx, dword ptr [rdx + 0x2c]
    0x140bd59bc    89482c                   mov              dword ptr [rax + 0x2c], ecx
    0x140bd59bf    488b4a30                 mov              rcx, qword ptr [rdx + 0x30]
    0x140bd59c3    48894830                 mov              qword ptr [rax + 0x30], rcx
    0x140bd59c7    8b4a38                   mov              ecx, dword ptr [rdx + 0x38]
    0x140bd59ca    894838                   mov              dword ptr [rax + 0x38], ecx
    0x140bd59cd    488b4a20                 mov              rcx, qword ptr [rdx + 0x20]
    0x140bd59d1    48894820                 mov              qword ptr [rax + 0x20], rcx
    0x140bd59d5    8b4a28                   mov              ecx, dword ptr [rdx + 0x28]
    0x140bd59d8    894828                   mov              dword ptr [rax + 0x28], ecx
    0x140bd59db    8b4a2c                   mov              ecx, dword ptr [rdx + 0x2c]
    0x140bd59de    89482c                   mov              dword ptr [rax + 0x2c], ecx
    0x140bd59e1    c3                       ret              

=== 0x140bd59f0 size=66  rhapsody::raw::texture::TextureViewDesc::bind_individual_mips
    ; TextureViewDesc::bind_individual_mips(bool): copy 0x3c bytes; set byte@0x38:=arg.  [high]
    0x140bd59f0    4889c8                   mov              rax, rcx
    0x140bd59f3    44884238                 mov              byte ptr [rdx + 0x38], r8b
    0x140bd59f7    0f1002                   movups           xmm0, xmmword ptr [rdx]
    0x140bd59fa    0f104a10                 movups           xmm1, xmmword ptr [rdx + 0x10]
    0x140bd59fe    0f105220                 movups           xmm2, xmmword ptr [rdx + 0x20]
    0x140bd5a02    0f115120                 movups           xmmword ptr [rcx + 0x20], xmm2
    0x140bd5a06    0f114910                 movups           xmmword ptr [rcx + 0x10], xmm1
    0x140bd5a0a    0f1101                   movups           xmmword ptr [rcx], xmm0
    0x140bd5a0d    488b4a2c                 mov              rcx, qword ptr [rdx + 0x2c]
    0x140bd5a11    4889482c                 mov              qword ptr [rax + 0x2c], rcx
    0x140bd5a15    8b4a34                   mov              ecx, dword ptr [rdx + 0x34]
    0x140bd5a18    894834                   mov              dword ptr [rax + 0x34], ecx
    0x140bd5a1b    0fb64a38                 movzx            ecx, byte ptr [rdx + 0x38]
    0x140bd5a1f    884838                   mov              byte ptr [rax + 0x38], cl
    0x140bd5a22    0fb74a39                 movzx            ecx, word ptr [rdx + 0x39]
    0x140bd5a26    66894839                 mov              word ptr [rax + 0x39], cx
    0x140bd5a2a    0fb64a3b                 movzx            ecx, byte ptr [rdx + 0x3b]
    0x140bd5a2e    88483b                   mov              byte ptr [rax + 0x3b], cl
    0x140bd5a31    c3                       ret              

=== 0x140bd5a40 size=57  rhapsody::raw::texture::impl$4::from
    ; impl$4::from: assert r8>0; {disc@0:=3, ptr@8:=rdx, len@0x10:=r8}. Builds a variant with tag 3.  [medium]
    0x140bd5a40    4883ec28                 sub              rsp, 0x28
    0x140bd5a44    4d85c0                   test             r8, r8
    0x140bd5a47    7417                     je               0x140bd5a60
    0x140bd5a49    48895108                 mov              qword ptr [rcx + 8], rdx
    0x140bd5a4d    4c894110                 mov              qword ptr [rcx + 0x10], r8
    0x140bd5a51    48c70103000000           mov              qword ptr [rcx], 3
    0x140bd5a58    4889c8                   mov              rax, rcx
    0x140bd5a5b    4883c428                 add              rsp, 0x28
    0x140bd5a5f    c3                       ret              
    0x140bd5a60    488d0d79a0f301           lea              rcx, [rip + 0x1f3a079] -> 0x142b0fae0
    0x140bd5a67    4c8d059aa0f301           lea              r8, [rip + 0x1f3a09a] -> 0x142b0fb08
    0x140bd5a6e    ba22000000               mov              edx, 0x22
    0x140bd5a73    e8b839d001               call             0x1428d9430 -> core::panicking::panic::hb210ae7b817a9f13
    0x140bd5a78    cc                       int3             
