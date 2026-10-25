0x140a72810 push     rbp
0x140a72811 push     r15
0x140a72813 push     r14
0x140a72815 push     r13
0x140a72817 push     r12
0x140a72819 push     rsi
0x140a7281a push     rdi
0x140a7281b push     rbx
0x140a7281c sub      rsp, 0xe8
0x140a72823 lea      rbp, [rsp + 0x80]
0x140a7282b mov      qword ptr [rbp + 0x60], 0xfffffffffffffffe
0x140a72833 mov      rdi, r9
0x140a72836 mov      rbx, r8
0x140a72839 mov      r14, rdx
0x140a7283c mov      r15, rcx
0x140a7283f call     0x140c16a00 ; puffin::are_scopes_on::h8f511bd2e57f501a
0x140a72844 mov      byte ptr [rbp - 0x18], al
0x140a72847 test     al, al
0x140a72849 je       0x140a728af
0x140a7284b mov      qword ptr [rbp - 0x28], r15
0x140a7284f lea      r12, [rip + 0x2063752] ; @0x142ad5fa8 'country_core::systems::audio::music::music_system::music_audio_system::fcountry_core::systems::time_'
0x140a72856 mov      r15d, 0x45
0x140a7285c mov      edx, 0x45
0x140a72861 mov      rcx, r12
0x140a72864 call     0x140a719d0 ; 
0x140a72869 cmp      rax, 1
0x140a7286d jne      0x140a72925
0x140a72873 mov      r9, rdx
0x140a72876 test     rdx, rdx
0x140a72879 je       0x140a728b8
0x140a7287b cmp      r9, 0x45
0x140a7287f jae      0x140a728b6
0x140a72881 lea      rax, [rip + 0x2063720] ; @0x142ad5fa8 'country_core::systems::audio::music::music_system::music_audio_system::fcountry_core::systems::time_'
0x140a72888 cmp      byte ptr [r9 + rax], 0xbf
0x140a7288d jg       0x140a728b8
0x140a7288f lea      rax, [rip + 0x2063812]
0x140a72896 mov      qword ptr [rsp + 0x20], rax
0x140a7289b lea      rcx, [rip + 0x2063706] ; @0x142ad5fa8 'country_core::systems::audio::music::music_system::music_audio_system::fcountry_core::systems::time_'
0x140a728a2 mov      edx, 0x45
0x140a728a7 xor      r8d, r8d
0x140a728aa call     0x1428d9940 ; core::str::slice_error_fail::h78e32f96bb5cc05d
0x140a728af xor      eax, eax
0x140a728b1 jmp      0x140a72a9e ; 
0x140a728b6 jne      0x140a7288f
0x140a728b8 lea      r12, [rip + 0x20636e9] ; @0x142ad5fa8 'country_core::systems::audio::music::music_system::music_audio_system::fcountry_core::systems::time_'
0x140a728bf mov      rcx, r12
0x140a728c2 mov      rdx, r9
0x140a728c5 call     0x140a719d0 ; 
0x140a728ca cmp      rax, 1
0x140a728ce jne      0x140a72925
0x140a728d0 mov      r8, rdx
0x140a728d3 add      r8, 2
0x140a728d7 je       0x140a72912
0x140a728d9 cmp      r8, 0x45
0x140a728dd jae      0x140a72910
0x140a728df lea      rax, [rip + 0x20636c2] ; @0x142ad5fa8 'country_core::systems::audio::music::music_system::music_audio_system::fcountry_core::systems::time_'
0x140a728e6 cmp      byte ptr [r8 + rax], 0xbf
0x140a728eb jg       0x140a72912
0x140a728ed lea      rax, [rip + 0x20637cc]
0x140a728f4 mov      qword ptr [rsp + 0x20], rax
0x140a728f9 lea      rcx, [rip + 0x20636a8] ; @0x142ad5fa8 'country_core::systems::audio::music::music_system::music_audio_system::fcountry_core::systems::time_'
0x140a72900 mov      edx, 0x45
0x140a72905 mov      r9d, 0x45
0x140a7290b call     0x1428d9940 ; core::str::slice_error_fail::h78e32f96bb5cc05d
0x140a72910 jne      0x140a728ed
0x140a72912 mov      r15d, 0x43
0x140a72918 sub      r15, rdx
0x140a7291b lea      r12, [rip + 0x2063686] ; @0x142ad5fa8 'country_core::systems::audio::music::music_system::music_audio_system::fcountry_core::systems::time_'
0x140a72922 add      r12, r8
0x140a72925 lea      rax, [rip + 0x20639cf]
0x140a7292c lea      r13, [rip + 0x206398d] ; @0x142ad62c0 'crates/country-core/src/systems/audio/music/music_system.rs'
0x140a72933 nop      word ptr cs:[rax + rax]
0x140a72940 movsx    ecx, byte ptr [rax - 1]
0x140a72944 test     ecx, ecx
0x140a72946 js       0x140a72960
0x140a72948 dec      rax
0x140a7294b cmp      ecx, 0x5c
0x140a7294e jne      0x140a729b9
0x140a72950 jmp      0x140a729d4 ; 
0x140a72955 nop      word ptr cs:[rax + rax]
0x140a72960 movzx    edx, byte ptr [rax - 2]
0x140a72964 cmp      dl, 0xc0
0x140a72967 jge      0x140a7298e
0x140a72969 movzx    r8d, byte ptr [rax - 3]
0x140a7296e cmp      r8b, 0xc0
0x140a72972 jge      0x140a72997
0x140a72974 movzx    r9d, byte ptr [rax - 4]
0x140a72979 add      rax, -4
0x140a7297d and      r9d, 7
0x140a72981 shl      r9d, 6
0x140a72985 and      r8d, 0x3f
0x140a72989 or       r8d, r9d
0x140a7298c jmp      0x140a7299f ; 
0x140a7298e add      rax, -2
0x140a72992 and      edx, 0x1f
0x140a72995 jmp      0x140a729a9 ; 
0x140a72997 add      rax, -3
0x140a7299b and      r8d, 0xf
0x140a7299f shl      r8d, 6
0x140a729a3 and      edx, 0x3f
0x140a729a6 or       edx, r8d
0x140a729a9 shl      edx, 6
0x140a729ac and      cl, 0x3f
0x140a729af movzx    ecx, cl
0x140a729b2 or       ecx, edx
0x140a729b4 cmp      ecx, 0x5c
0x140a729b7 je       0x140a729d4
0x140a729b9 cmp      ecx, 0x2f
0x140a729bc je       0x140a729d4
0x140a729be cmp      rax, r13
0x140a729c1 jne      0x140a72940
0x140a729c7 mov      rsi, r14
0x140a729ca mov      r14, rdi
0x140a729cd mov      edi, 0x3b
0x140a729d2 jmp      0x140a72a29 ; 
0x140a729d4 lea      r13, [rip + 0x20638e5] ; @0x142ad62c0 'crates/country-core/src/systems/audio/music/music_system.rs'
0x140a729db sub      rax, r13
0x140a729de mov      r8, rax
0x140a729e1 inc      r8
0x140a729e4 je       0x140a72a18
0x140a729e6 cmp      r8, 0x3b
0x140a729ea jae      0x140a72a16
0x140a729ec cmp      byte ptr [r8 + r13], 0xbf
0x140a729f1 jg       0x140a72a18
0x140a729f3 lea      rax, [rip + 0x206368e]
0x140a729fa mov      qword ptr [rsp + 0x20], rax
0x140a729ff lea      rcx, [rip + 0x20638ba] ; @0x142ad62c0 'crates/country-core/src/systems/audio/music/music_system.rs'
0x140a72a06 mov      edx, 0x3b
0x140a72a0b mov      r9d, 0x3b
0x140a72a11 call     0x1428d9940 ; core::str::slice_error_fail::h78e32f96bb5cc05d
0x140a72a16 jne      0x140a729f3
0x140a72a18 mov      rsi, r14
0x140a72a1b mov      r14, rdi
0x140a72a1e mov      edi, 0x3a
0x140a72a23 sub      rdi, rax
0x140a72a26 add      r13, r8
0x140a72a29 call     0x140c18130 ; puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$::h96833e27ced669c2
0x140a72a2e mov      rcx, rax
0x140a72a31 mov      rax, qword ptr [rax]
0x140a72a34 cmp      rax, 1
0x140a72a38 jne      0x140a7304b
0x140a72a3e add      rcx, 8
0x140a72a42 cmp      qword ptr [rcx], 0
0x140a72a46 jne      0x140a73085
0x140a72a4c mov      qword ptr [rcx], 0xffffffffffffffff
0x140a72a53 mov      qword ptr [rbp - 0x20], rcx
0x140a72a57 add      rcx, 8
0x140a72a5b mov      qword ptr [rsp + 0x20], rdi
0x140a72a60 mov      qword ptr [rsp + 0x30], 0
0x140a72a69 mov      qword ptr [rsp + 0x28], 1
0x140a72a72 mov      rdx, r12
0x140a72a75 mov      r8, r15
0x140a72a78 mov      r9, r13
0x140a72a7b call     0x140c16d40 ; puffin::ThreadProfiler::begin_scope::h161a40482b7cfe19
0x140a72a80 mov      rcx, qword ptr [rbp - 0x20]
0x140a72a84 inc      qword ptr [rcx]
0x140a72a87 mov      qword ptr [rbp - 0x20], rax
0x140a72a8b mov      qword ptr [rbp - 0x40], rax
0x140a72a8f mov      eax, 1
0x140a72a94 mov      rdi, r14
0x140a72a97 mov      r14, rsi
0x140a72a9a mov      r15, qword ptr [rbp - 0x28]
0x140a72a9e mov      rcx, qword ptr [rbp + 0xe0]
0x140a72aa5 mov      qword ptr [rbp - 0x48], rax
0x140a72aa9 mov      r12d, dword ptr [rbx + 0x1c]
0x140a72aad mov      r13, qword ptr [rbx + 0x10]
0x140a72ab1 mov      dword ptr [r13], r12d
0x140a72ab5 mov      rbx, qword ptr [rbx]
0x140a72ab8 mov      rsi, qword ptr [rcx]
0x140a72abb mov      rax, qword ptr [r15]
0x140a72abe movss    xmm2, dword ptr [rsi + 0x28]
0x140a72ac3 movaps   xmm0, xmm2
0x140a72ac6 divss    xmm0, dword ptr [rip + 0x1f5a62a]
0x140a72ace mov      eax, dword ptr [rax]
0x140a72ad0 addss    xmm0, dword ptr [rbx + 0x1c4]
0x140a72ad8 xorps    xmm1, xmm1
0x140a72adb maxss    xmm1, xmm0
0x140a72adf movss    xmm3, dword ptr [rip + 0x1eb2e1d]
0x140a72ae7 movaps   xmm4, xmm3
0x140a72aea minss    xmm4, xmm1
0x140a72aee movss    dword ptr [rbx + 0x1c4], xmm4
0x140a72af6 ucomiss  xmm4, dword ptr [rip + 0x200d2fb]
0x140a72afd jb       0x140a72b16
0x140a72aff movss    xmm0, dword ptr [rip + 0x1eb2e49]
0x140a72b07 ucomiss  xmm0, xmm4
0x140a72b0a jbe      0x140a72b16
0x140a72b0c maxss    xmm4, dword ptr [rip + 0x2062dd0]
0x140a72b14 jmp      0x140a72b54 ; 
0x140a72b16 ucomiss  xmm4, dword ptr [rip + 0x2062db7]
0x140a72b1d jb       0x140a72b36
0x140a72b1f movss    xmm0, dword ptr [rip + 0x1eb2d49]
0x140a72b27 ucomiss  xmm0, xmm4
0x140a72b2a jb       0x140a72b36
0x140a72b2c maxss    xmm4, dword ptr [rip + 0x2062dac]
0x140a72b34 jmp      0x140a72b54 ; 
0x140a72b36 ucomiss  xmm4, dword ptr [rip + 0x2062d9b]
0x140a72b3d jb       0x140a72b5c
0x140a72b3f movss    xmm0, dword ptr [rip + 0x1f7b049]
0x140a72b47 ucomiss  xmm0, xmm4
0x140a72b4a jb       0x140a72b5c
0x140a72b4c maxss    xmm4, dword ptr [rip + 0x2062d88]
0x140a72b54 movss    dword ptr [rbx + 0x1c4], xmm4
0x140a72b5c movss    xmm0, dword ptr [rbx + 0x1c0]
0x140a72b64 movaps   xmm1, xmm3
0x140a72b67 cmp      eax, 0x13
0x140a72b6a je       0x140a72b6f
0x140a72b6c movaps   xmm1, xmm4
0x140a72b6f call     0x140c904e0 ; utils::time_continuous_lerp::hc223110ad57d00a3
0x140a72b74 mov      rax, qword ptr [rbp + 0xd8]
0x140a72b7b movss    xmm3, dword ptr [rip + 0x2021d55]
0x140a72b83 cmpltss  xmm3, xmm0
0x140a72b88 movss    xmm1, dword ptr [rip + 0x1eb2d74]
0x140a72b90 andps    xmm1, xmm3
0x140a72b93 andnps   xmm3, xmm0
0x140a72b96 orps     xmm3, xmm1
0x140a72b99 movss    dword ptr [rbx + 0x1c0], xmm3
0x140a72ba1 ucomiss  xmm3, dword ptr [rip + 0x200d250]
0x140a72ba8 jb       0x140a72bc1
0x140a72baa movss    xmm0, dword ptr [rip + 0x1eb2d9e]
0x140a72bb2 ucomiss  xmm0, xmm3
0x140a72bb5 jbe      0x140a72bc1
0x140a72bb7 movss    xmm3, dword ptr [rip + 0x2062d25]
0x140a72bbf jmp      0x140a72bff ; 
0x140a72bc1 ucomiss  xmm3, dword ptr [rip + 0x2062d0c]
0x140a72bc8 jb       0x140a72be1
0x140a72bca movss    xmm0, dword ptr [rip + 0x1eb2c9e]
0x140a72bd2 ucomiss  xmm0, xmm3
0x140a72bd5 jb       0x140a72be1
0x140a72bd7 movss    xmm3, dword ptr [rip + 0x2062d01]
0x140a72bdf jmp      0x140a72bff ; 
0x140a72be1 ucomiss  xmm3, dword ptr [rip + 0x2062cf0]
0x140a72be8 jb       0x140a72c07
0x140a72bea movss    xmm0, dword ptr [rip + 0x1f7af9e]
0x140a72bf2 ucomiss  xmm0, xmm3
0x140a72bf5 jb       0x140a72c07
0x140a72bf7 movss    xmm3, dword ptr [rip + 0x2062cdd]
0x140a72bff movss    dword ptr [rbx + 0x1c0], xmm3
0x140a72c07 mov      rax, qword ptr [rax]
0x140a72c0a mov      r14, qword ptr [r14]
0x140a72c0d mov      rcx, qword ptr [r14]
0x140a72c10 cmp      byte ptr [rax + 0x100], 0
0x140a72c17 je       0x140a72c3e
0x140a72c19 test     rcx, rcx
0x140a72c1c je       0x140a72c59
0x140a72c1e add      rcx, 0x10
0x140a72c22 lea      rdx, [rip + 0x206376f] ; @0x142ad6398 'MusicRichnessNewGladeTimePreviewGladeTypecrates/country-core/src/systems/time_of_day.rsTOD-EnabledTi'
0x140a72c29 movss    xmm3, dword ptr [rip + 0x1eb2cd3]
0x140a72c31 mov      r8d, 0xd
0x140a72c37 call     0x140a53db0 ; country_core::resources::audio_resources::AudioCoreInner::set_parameter::h6ff14d3f62899348
0x140a72c3c jmp      0x140a72c59 ; 
0x140a72c3e test     rcx, rcx
0x140a72c41 je       0x140a72c59
0x140a72c43 add      rcx, 0x10
0x140a72c47 lea      rdx, [rip + 0x206374a] ; @0x142ad6398 'MusicRichnessNewGladeTimePreviewGladeTypecrates/country-core/src/systems/time_of_day.rsTOD-EnabledTi'
0x140a72c4e mov      r8d, 0xd
0x140a72c54 call     0x140a53db0 ; country_core::resources::audio_resources::AudioCoreInner::set_parameter::h6ff14d3f62899348
0x140a72c59 movss    xmm0, dword ptr [rsi + 0x28]
0x140a72c5e mov      dword ptr [r13], r12d
0x140a72c62 addss    xmm0, dword ptr [rbx + 0x1c8]
0x140a72c6a xorps    xmm1, xmm1
0x140a72c6d maxss    xmm1, xmm0
0x140a72c71 movss    xmm3, dword ptr [rip + 0x1fa403f]
0x140a72c79 minss    xmm3, xmm1
0x140a72c7d movss    dword ptr [rbx + 0x1c8], xmm3
0x140a72c85 mov      rcx, qword ptr [r14]
0x140a72c88 test     rcx, rcx
0x140a72c8b je       0x140a72ca3
0x140a72c8d add      rcx, 0x10
0x140a72c91 lea      rdx, [rip + 0x206370d] ; @0x142ad63a5 'NewGladeTimePreviewGladeTypecrates/country-core/src/systems/time_of_day.rsTOD-EnabledTimetravelDayNi'
0x140a72c98 mov      r8d, 0xc
0x140a72c9e call     0x140a53db0 ; country_core::resources::audio_resources::AudioCoreInner::set_parameter::h6ff14d3f62899348
0x140a72ca3 mov      rax, qword ptr [rdi]
0x140a72ca6 mov      eax, dword ptr [rax]
0x140a72ca8 or       eax, 8
0x140a72cab cmp      eax, 0xa
0x140a72cae jne      0x140a72d85
0x140a72cb4 mov      rax, qword ptr [rbp + 0xe8]
0x140a72cbb mov      dword ptr [r13], r12d
0x140a72cbf mov      rax, qword ptr [rax]
0x140a72cc2 movss    xmm1, dword ptr [rax + 0x58]
0x140a72cc7 mov      rcx, rbx
0x140a72cca call     0x1408b5500 ; country_core::systems::audio::music::music_player::MusicPlayer::update::h03b33063fc06b9e8
0x140a72ccf mov      rax, qword ptr [rbp + 0xf0]
0x140a72cd6 mov      rax, qword ptr [rax]
0x140a72cd9 cmp      byte ptr [rax + 1], 0
0x140a72cdd je       0x140a72e0b
0x140a72ce3 mov      rax, qword ptr [rbp + 0xf8]
0x140a72cea mov      rax, qword ptr [rax]
0x140a72ced cmp      byte ptr [rax + 9], 2
0x140a72cf1 mov      dword ptr [r13], r12d
0x140a72cf5 jne      0x140a72ecb
0x140a72cfb lea      rcx, [rbx + 0x70]
0x140a72cff call     0x140a57300 ; country_core::resources::audio_resources::AudioInstance::start_if_not_playing::h3a8fcee2c2acf93b
0x140a72d04 cmp      dword ptr [rbx + 0xa0], 1
0x140a72d0b jne      0x140a72e17
0x140a72d11 lea      rdx, [rbx + 0xa8]
0x140a72d18 lea      rcx, [rbp + 0x28]
0x140a72d1c mov      r8b, 1
0x140a72d1f call     0x141437440 ; libfmod::EventInstance::stop::h6aa257fdd68e9145
0x140a72d24 movabs   rax, 0x8000000000000004
0x140a72d2e cmp      qword ptr [rbp + 0x28], rax
0x140a72d32 je       0x140a72e17
0x140a72d38 mov      rax, qword ptr [rbp + 0x58]
0x140a72d3c mov      qword ptr [rbp + 0x20], rax
0x140a72d40 movups   xmm0, xmmword ptr [rbp + 0x28]
0x140a72d44 movups   xmm1, xmmword ptr [rbp + 0x38]
0x140a72d48 movups   xmm2, xmmword ptr [rbp + 0x48]
0x140a72d4c movaps   xmmword ptr [rbp + 0x10], xmm2
0x140a72d50 movaps   xmmword ptr [rbp], xmm1
0x140a72d54 movaps   xmmword ptr [rbp - 0x10], xmm0
0x140a72d58 lea      rax, [rip + 0x205fec1]
0x140a72d5f mov      qword ptr [rsp + 0x20], rax
0x140a72d64 lea      rcx, [rip + 0x205d895] ; @0x142ad0600 'called `Result::unwrap()` on an `Err` value'
0x140a72d6b lea      r9, [rip + 0x205d84e]
0x140a72d72 lea      r8, [rbp - 0x10]
0x140a72d76 mov      edx, 0x2b
0x140a72d7b call     0x1428d9760 ; core::result::unwrap_failed::h96d74ae09566f4b3
0x140a72d80 jmp      0x140a731bd ; 
0x140a72d85 mov      dword ptr [r13], r12d
0x140a72d89 mov      rcx, rbx
0x140a72d8c call     0x1408b7490 ; country_core::systems::audio::music::music_player::MusicPlayer::pause::hde114bfbbf623a24
0x140a72d91 mov      dword ptr [r13], r12d
0x140a72d95 mov      rcx, rbx
0x140a72d98 call     0x1408b6250 ; country_core::systems::audio::music::music_player::MusicPlayer::stop_photomode_snapshots::h852f33e516c5be99
0x140a72d9d movzx    esi, byte ptr [rbx + 0x1cc]
0x140a72da4 lea      rcx, [rbx + 0x190]
0x140a72dab cmp      rsi, 5
0x140a72daf jne      0x140a72f52
0x140a72db5 cmp      dword ptr [rcx], 1
0x140a72db8 jne      0x140a72de1
0x140a72dba lea      rdx, [rbx + 0x198]
0x140a72dc1 lea      rcx, [rbp + 0x28]
0x140a72dc5 xor      r8d, r8d
0x140a72dc8 call     0x141437440 ; libfmod::EventInstance::stop::h6aa257fdd68e9145
0x140a72dcd movabs   rax, 0x8000000000000004
0x140a72dd7 cmp      qword ptr [rbp + 0x28], rax
0x140a72ddb jne      0x140a73175
0x140a72de1 mov      rax, qword ptr [rbp + 0xd0]
0x140a72de8 mov      rax, qword ptr [rax]
0x140a72deb movzx    eax, byte ptr [rax + 0x58]
0x140a72def lea      rcx, [rip + 0x20639fa]
0x140a72df6 lea      rax, [rcx + rax*4]
0x140a72dfa mov      rcx, qword ptr [r14]
0x140a72dfd test     rcx, rcx
0x140a72e00 jne      0x140a72f6a
0x140a72e06 jmp      0x140a72f84 ; 
0x140a72e0b mov      dword ptr [r13], r12d
0x140a72e0f mov      rcx, rbx
0x140a72e12 call     0x1408b6250 ; country_core::systems::audio::music::music_player::MusicPlayer::stop_photomode_snapshots::h852f33e516c5be99
0x140a72e17 cmp      dword ptr [rbx + 0x160], 1
0x140a72e1e jne      0x140a72e47
0x140a72e20 lea      rdx, [rbx + 0x168]
0x140a72e27 lea      rcx, [rbp + 0x28]
0x140a72e2b mov      r8b, 1
0x140a72e2e call     0x141437160 ; libfmod::EventInstance::set_paused::h32fbd681052381c4
0x140a72e33 movabs   rax, 0x8000000000000004
0x140a72e3d cmp      qword ptr [rbp + 0x28], rax
0x140a72e41 jne      0x140a73091
0x140a72e47 cmp      dword ptr [rbx + 0x190], 1
0x140a72e4e jne      0x140a72ff7
0x140a72e54 add      rbx, 0x198
0x140a72e5b lea      rcx, [rbp + 0x28]
0x140a72e5f mov      rdx, rbx
0x140a72e62 xor      r8d, r8d
0x140a72e65 call     0x141437440 ; libfmod::EventInstance::stop::h6aa257fdd68e9145
0x140a72e6a movabs   rax, 0x8000000000000004
0x140a72e74 cmp      qword ptr [rbp + 0x28], rax
0x140a72e78 je       0x140a72ff7
0x140a72e7e mov      rax, qword ptr [rbp + 0x58]
0x140a72e82 mov      qword ptr [rbp + 0x20], rax
0x140a72e86 movups   xmm0, xmmword ptr [rbp + 0x28]
0x140a72e8a movups   xmm1, xmmword ptr [rbp + 0x38]
0x140a72e8e movups   xmm2, xmmword ptr [rbp + 0x48]
0x140a72e92 movaps   xmmword ptr [rbp + 0x10], xmm2
0x140a72e96 movaps   xmmword ptr [rbp], xmm1
0x140a72e9a movaps   xmmword ptr [rbp - 0x10], xmm0
0x140a72e9e lea      rax, [rip + 0x205fd63]
0x140a72ea5 mov      qword ptr [rsp + 0x20], rax
0x140a72eaa lea      rcx, [rip + 0x205d74f] ; @0x142ad0600 'called `Result::unwrap()` on an `Err` value'
0x140a72eb1 lea      r9, [rip + 0x205d708]
0x140a72eb8 lea      r8, [rbp - 0x10]
0x140a72ebc mov      edx, 0x2b
0x140a72ec1 call     0x1428d9760 ; core::result::unwrap_failed::h96d74ae09566f4b3
0x140a72ec6 jmp      0x140a731bd ; 
0x140a72ecb lea      rcx, [rbx + 0xa0]
0x140a72ed2 call     0x140a57300 ; country_core::resources::audio_resources::AudioInstance::start_if_not_playing::h3a8fcee2c2acf93b
0x140a72ed7 cmp      dword ptr [rbx + 0x70], 1
0x140a72edb jne      0x140a72e17
0x140a72ee1 lea      rdx, [rbx + 0x78]
0x140a72ee5 lea      rcx, [rbp + 0x28]
0x140a72ee9 mov      r8b, 1
0x140a72eec call     0x141437440 ; libfmod::EventInstance::stop::h6aa257fdd68e9145
0x140a72ef1 movabs   rax, 0x8000000000000004
0x140a72efb cmp      qword ptr [rbp + 0x28], rax
0x140a72eff je       0x140a72e17
0x140a72f05 mov      rax, qword ptr [rbp + 0x58]
0x140a72f09 mov      qword ptr [rbp + 0x20], rax
0x140a72f0d movups   xmm0, xmmword ptr [rbp + 0x28]
0x140a72f11 movups   xmm1, xmmword ptr [rbp + 0x38]
0x140a72f15 movups   xmm2, xmmword ptr [rbp + 0x48]
0x140a72f19 movaps   xmmword ptr [rbp + 0x10], xmm2
0x140a72f1d movaps   xmmword ptr [rbp], xmm1
0x140a72f21 movaps   xmmword ptr [rbp - 0x10], xmm0
0x140a72f25 lea      rax, [rip + 0x205fcf4]
0x140a72f2c mov      qword ptr [rsp + 0x20], rax
0x140a72f31 lea      rcx, [rip + 0x205d6c8] ; @0x142ad0600 'called `Result::unwrap()` on an `Err` value'
0x140a72f38 lea      r9, [rip + 0x205d681]
0x140a72f3f lea      r8, [rbp - 0x10]
0x140a72f43 mov      edx, 0x2b
0x140a72f48 call     0x1428d9760 ; core::result::unwrap_failed::h96d74ae09566f4b3
0x140a72f4d jmp      0x140a731bd ; 
0x140a72f52 call     0x140a57300 ; country_core::resources::audio_resources::AudioInstance::start_if_not_playing::h3a8fcee2c2acf93b
0x140a72f57 lea      rax, [rip + 0x2063892]
0x140a72f5e lea      rax, [rax + rsi*4]
0x140a72f62 mov      rcx, qword ptr [r14]
0x140a72f65 test     rcx, rcx
0x140a72f68 je       0x140a72f84
0x140a72f6a movss    xmm3, dword ptr [rax]
0x140a72f6e add      rcx, 0x10
0x140a72f72 lea      rdx, [rip + 0x2063438] ; @0x142ad63b1 'PreviewGladeTypecrates/country-core/src/systems/time_of_day.rsTOD-EnabledTimetravelDayNight'
0x140a72f79 mov      r8d, 0x10
0x140a72f7f call     0x140a53db0 ; country_core::resources::audio_resources::AudioCoreInner::set_parameter::h6ff14d3f62899348
0x140a72f84 lea      rdi, [rbx + 0x160]
0x140a72f8b mov      rcx, rdi
0x140a72f8e call     0x140a57300 ; country_core::resources::audio_resources::AudioInstance::start_if_not_playing::h3a8fcee2c2acf93b
0x140a72f93 mov      rax, qword ptr [rdi]
0x140a72f96 cmp      rax, 1
0x140a72f9a jne      0x140a72fc9
0x140a72f9c lea      rdx, [rbx + 0x168]
0x140a72fa3 lea      rcx, [rbp + 0x28]
0x140a72fa7 call     0x141436fe0 ; libfmod::EventInstance::get_paused::h000fc06dc32eddbe
0x140a72fac movabs   rax, 0x8000000000000004
0x140a72fb6 cmp      qword ptr [rbp + 0x28], rax
0x140a72fba jne      0x140a730de
0x140a72fc0 cmp      byte ptr [rbp + 0x30], 1
0x140a72fc4 jne      0x140a72ff7
0x140a72fc6 mov      rax, qword ptr [rdi]
0x140a72fc9 test     al, 1
0x140a72fcb je       0x140a72ff7
0x140a72fcd add      rbx, 0x168
0x140a72fd4 lea      rcx, [rbp + 0x28]
0x140a72fd8 mov      rdx, rbx
0x140a72fdb xor      r8d, r8d
0x140a72fde call     0x141437160 ; libfmod::EventInstance::set_paused::h32fbd681052381c4
0x140a72fe3 movabs   rax, 0x8000000000000004
0x140a72fed cmp      qword ptr [rbp + 0x28], rax
0x140a72ff1 jne      0x140a7312b
0x140a72ff7 cmp      byte ptr [rbp - 0x18], 0
0x140a72ffb je       0x140a73037
0x140a72ffd call     0x140c18130 ; puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::VAL$u7b$$u7b$tls.shim$u7d$$u7d$::h96833e27ced669c2
0x140a73002 mov      rcx, rax
0x140a73005 mov      rax, qword ptr [rax]
0x140a73008 cmp      rax, 1
0x140a7300c jne      0x140a73065
0x140a7300e add      rcx, 8
0x140a73012 cmp      qword ptr [rcx], 0
0x140a73016 jne      0x140a73085
0x140a73018 mov      qword ptr [rcx], 0xffffffffffffffff
0x140a7301f mov      qword ptr [rbp - 0x18], rcx
0x140a73023 add      rcx, 8
0x140a73027 mov      rdx, qword ptr [rbp - 0x20]
0x140a7302b call     0x140c170f0 ; puffin::ThreadProfiler::end_scope::hbdd5f34d320e5739
0x140a73030 mov      rax, qword ptr [rbp - 0x18]
0x140a73034 inc      qword ptr [rax]
0x140a73037 add      rsp, 0xe8
0x140a7303e pop      rbx
0x140a7303f pop      rdi
0x140a73040 pop      rsi
0x140a73041 pop      r12
0x140a73043 pop      r13
0x140a73045 pop      r14
0x140a73047 pop      r15
0x140a73049 pop      rbp
0x140a7304a ret      
0x140a7304b test     rax, rax
0x140a7304e jne      0x140a73079
0x140a73050 xor      edx, edx
0x140a73052 call     0x1428f87d0 ; std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize::h722048566c3776eb
0x140a73057 mov      rcx, rax
0x140a7305a test     rax, rax
0x140a7305d jne      0x140a72a42
0x140a73063 jmp      0x140a73079 ; 
0x140a73065 test     rax, rax
0x140a73068 jne      0x140a73079
0x140a7306a xor      edx, edx
0x140a7306c call     0x1428f87d0 ; std::sys::thread_local::native::lazy::Storage$LT$T$C$D$GT$::initialize::h722048566c3776eb
0x140a73071 mov      rcx, rax
0x140a73074 test     rax, rax
0x140a73077 jne      0x140a73012
0x140a73079 lea      rcx, [rip + 0x21c7a20]
0x140a73080 call     0x1428d80d0 ; std::thread::local::panic_access_error::hcff639665d708376
0x140a73085 lea      rcx, [rip + 0x21c7ae4]
0x140a7308c call     0x1428d9250 ; core::cell::panic_already_borrowed::h54fda569c7a6ddb1
0x140a73091 mov      rax, qword ptr [rbp + 0x58]
0x140a73095 mov      qword ptr [rbp + 0x20], rax
0x140a73099 movups   xmm0, xmmword ptr [rbp + 0x28]
0x140a7309d movups   xmm1, xmmword ptr [rbp + 0x38]
0x140a730a1 movups   xmm2, xmmword ptr [rbp + 0x48]
0x140a730a5 movaps   xmmword ptr [rbp + 0x10], xmm2
0x140a730a9 movaps   xmmword ptr [rbp], xmm1
0x140a730ad movaps   xmmword ptr [rbp - 0x10], xmm0
0x140a730b1 lea      rax, [rip + 0x205fb80]
0x140a730b8 mov      qword ptr [rsp + 0x20], rax
0x140a730bd lea      rcx, [rip + 0x205d53c] ; @0x142ad0600 'called `Result::unwrap()` on an `Err` value'
0x140a730c4 lea      r9, [rip + 0x205d4f5]
0x140a730cb lea      r8, [rbp - 0x10]
0x140a730cf mov      edx, 0x2b
0x140a730d4 call     0x1428d9760 ; core::result::unwrap_failed::h96d74ae09566f4b3
0x140a730d9 jmp      0x140a731bd ; 
0x140a730de mov      rax, qword ptr [rbp + 0x58]
0x140a730e2 mov      qword ptr [rbp + 0x20], rax
0x140a730e6 movups   xmm0, xmmword ptr [rbp + 0x28]
0x140a730ea movups   xmm1, xmmword ptr [rbp + 0x38]
0x140a730ee movups   xmm2, xmmword ptr [rbp + 0x48]
0x140a730f2 movaps   xmmword ptr [rbp + 0x10], xmm2
0x140a730f6 movaps   xmmword ptr [rbp], xmm1
0x140a730fa movaps   xmmword ptr [rbp - 0x10], xmm0
0x140a730fe lea      rax, [rip + 0x205fb4b]
0x140a73105 mov      qword ptr [rsp + 0x20], rax
0x140a7310a lea      rcx, [rip + 0x205d4ef] ; @0x142ad0600 'called `Result::unwrap()` on an `Err` value'
0x140a73111 lea      r9, [rip + 0x205d4a8]
0x140a73118 lea      r8, [rbp - 0x10]
0x140a7311c mov      edx, 0x2b
0x140a73121 call     0x1428d9760 ; core::result::unwrap_failed::h96d74ae09566f4b3
0x140a73126 jmp      0x140a731bd ; 
0x140a7312b mov      rax, qword ptr [rbp + 0x58]
0x140a7312f mov      qword ptr [rbp + 0x20], rax
0x140a73133 movups   xmm0, xmmword ptr [rbp + 0x28]
0x140a73137 movups   xmm1, xmmword ptr [rbp + 0x38]
0x140a7313b movups   xmm2, xmmword ptr [rbp + 0x48]
0x140a7313f movaps   xmmword ptr [rbp + 0x10], xmm2
0x140a73143 movaps   xmmword ptr [rbp], xmm1
0x140a73147 movaps   xmmword ptr [rbp - 0x10], xmm0
0x140a7314b lea      rax, [rip + 0x205fb16]
0x140a73152 mov      qword ptr [rsp + 0x20], rax
0x140a73157 lea      rcx, [rip + 0x205d4a2] ; @0x142ad0600 'called `Result::unwrap()` on an `Err` value'
0x140a7315e lea      r9, [rip + 0x205d45b]
0x140a73165 lea      r8, [rbp - 0x10]
0x140a73169 mov      edx, 0x2b
0x140a7316e call     0x1428d9760 ; core::result::unwrap_failed::h96d74ae09566f4b3
0x140a73173 jmp      0x140a731bd ; 
0x140a73175 mov      rax, qword ptr [rbp + 0x58]
0x140a73179 mov      qword ptr [rbp + 0x20], rax
0x140a7317d movups   xmm0, xmmword ptr [rbp + 0x28]
0x140a73181 movups   xmm1, xmmword ptr [rbp + 0x38]
0x140a73185 movups   xmm2, xmmword ptr [rbp + 0x48]
0x140a73189 movaps   xmmword ptr [rbp + 0x10], xmm2
0x140a7318d movaps   xmmword ptr [rbp], xmm1
0x140a73191 movaps   xmmword ptr [rbp - 0x10], xmm0
0x140a73195 lea      rax, [rip + 0x205fa6c]
0x140a7319c mov      qword ptr [rsp + 0x20], rax
0x140a731a1 lea      rcx, [rip + 0x205d458] ; @0x142ad0600 'called `Result::unwrap()` on an `Err` value'
0x140a731a8 lea      r9, [rip + 0x205d411]
0x140a731af lea      r8, [rbp - 0x10]
0x140a731b3 mov      edx, 0x2b
0x140a731b8 call     0x1428d9760 ; core::result::unwrap_failed::h96d74ae09566f4b3
0x140a731bd ud2      
0x140a731bf nop      
0x140a731c0 mov      qword ptr [rsp + 0x10], rdx
0x140a731c5 push     rbp
0x140a731c6 push     r15
0x140a731c8 push     r14
0x140a731ca push     r13
0x140a731cc push     r12
0x140a731ce push     rsi
0x140a731cf push     rdi
0x140a731d0 push     rbx
0x140a731d1 sub      rsp, 0x38
0x140a731d5 lea      rbp, [rdx + 0x80]
0x140a731dc mov      rax, qword ptr [rbp - 0x20]
0x140a731e0 inc      qword ptr [rax]
0x140a731e3 add      rsp, 0x38
0x140a731e7 pop      rbx
0x140a731e8 pop      rdi
0x140a731e9 pop      rsi
0x140a731ea pop      r12
0x140a731ec pop      r13
0x140a731ee pop      r14
0x140a731f0 pop      r15
0x140a731f2 pop      rbp
0x140a731f3 ret      
0x140a731f4 nop      word ptr cs:[rax + rax]
0x140a73200 mov      qword ptr [rsp + 0x10], rdx
0x140a73205 push     rbp
0x140a73206 push     r15
0x140a73208 push     r14
0x140a7320a push     r13
0x140a7320c push     r12
0x140a7320e push     rsi
0x140a7320f push     rdi
0x140a73210 push     rbx
0x140a73211 sub      rsp, 0x38
0x140a73215 lea      rbp, [rdx + 0x80]
0x140a7321c cmp      byte ptr [rbp - 0x18], 0
0x140a73220 je       0x140a73242
0x140a73222 lea      rax, [rbp - 0x40]
0x140a73226 mov      qword ptr [rbp - 0x30], rax
0x140a7322a lea      rax, [rbp - 0x30]
0x140a7322e mov      qword ptr [rbp - 0x38], rax
0x140a73232 lea      rcx, [rip + 0x2062def]
0x140a73239 lea      rdx, [rbp - 0x38]
0x140a7323d call     0x14132f2d0 ; std::thread::local::LocalKey$LT$T$GT$::with::h8a9346824aaf9100
0x140a73242 nop      
0x140a73243 add      rsp, 0x38
0x140a73247 pop      rbx
0x140a73248 pop      rdi
0x140a73249 pop      rsi
0x140a7324a pop      r12
0x140a7324c pop      r13
0x140a7324e pop      r14
0x140a73250 pop      r15
0x140a73252 pop      rbp
0x140a73253 ret      
0x140a73254 nop      word ptr cs:[rax + rax]
0x140a73260 mov      qword ptr [rsp + 0x10], rdx
0x140a73265 push     rbp
0x140a73266 push     r15
0x140a73268 push     r14
0x140a7326a push     r13
0x140a7326c push     r12
0x140a7326e push     rsi
0x140a7326f push     rdi
0x140a73270 push     rbx
0x140a73271 sub      rsp, 0x38
0x140a73275 lea      rbp, [rdx + 0x80]
0x140a7327c lea      rcx, [rbp - 0x10]
0x140a73280 call     0x140a526b0 ; core::ptr::drop_in_place$LT$libfmod..Error$GT$::h60b34b2c39a0f0f1 (.llvm.4456468861159116502)
0x140a73285 nop      
0x140a73286 add      rsp, 0x38
0x140a7328a pop      rbx
0x140a7328b pop      rdi
0x140a7328c pop      rsi
0x140a7328d pop      r12
0x140a7328f pop      r13
0x140a73291 pop      r14
0x140a73293 pop      r15
0x140a73295 pop      rbp
0x140a73296 ret      
0x140a73297 nop      word ptr [rax + rax]
0x140a732a0 mov      qword ptr [rsp + 0x10], rdx
0x140a732a5 push     rbp
0x140a732a6 push     r15
0x140a732a8 push     r14
0x140a732aa push     r13
0x140a732ac push     r12
0x140a732ae push     rsi
0x140a732af push     rdi
0x140a732b0 push     rbx
0x140a732b1 sub      rsp, 0x38
0x140a732b5 lea      rbp, [rdx + 0x80]
0x140a732bc mov      rax, qword ptr [rbp - 0x18]
0x140a732c0 inc      qword ptr [rax]
0x140a732c3 add      rsp, 0x38
0x140a732c7 pop      rbx
0x140a732c8 pop      rdi
0x140a732c9 pop      rsi
0x140a732ca pop      r12
0x140a732cc pop      r13
0x140a732ce pop      r14
0x140a732d0 pop      r15
0x140a732d2 pop      rbp
0x140a732d3 ret      
0x140a732d4 nop      word ptr cs:[rax + rax]
0x140a732e0 mov      qword ptr [rsp + 0x10], rdx
0x140a732e5 push     rbp
0x140a732e6 push     r15
0x140a732e8 push     r14
0x140a732ea push     r13
0x140a732ec push     r12
0x140a732ee push     rsi
0x140a732ef push     rdi
0x140a732f0 push     rbx
0x140a732f1 sub      rsp, 0x38
0x140a732f5 lea      rbp, [rdx + 0x80]
0x140a732fc lea      rcx, [rbp - 0x10]
0x140a73300 call     0x140a526b0 ; core::ptr::drop_in_place$LT$libfmod..Error$GT$::h60b34b2c39a0f0f1 (.llvm.4456468861159116502)
0x140a73305 nop      
0x140a73306 add      rsp, 0x38
0x140a7330a pop      rbx
0x140a7330b pop      rdi
0x140a7330c pop      rsi
0x140a7330d pop      r12
0x140a7330f pop      r13
0x140a73311 pop      r14
0x140a73313 pop      r15
0x140a73315 pop      rbp
0x140a73316 ret      
0x140a73317 nop      word ptr [rax + rax]
0x140a73320 mov      qword ptr [rsp + 0x10], rdx
0x140a73325 push     rbp
0x140a73326 push     r15
0x140a73328 push     r14
0x140a7332a push     r13
0x140a7332c push     r12
0x140a7332e push     rsi
0x140a7332f push     rdi
0x140a73330 push     rbx
0x140a73331 sub      rsp, 0x38
0x140a73335 lea      rbp, [rdx + 0x80]
0x140a7333c lea      rcx, [rbp - 0x10]
0x140a73340 call     0x140a526b0 ; core::ptr::drop_in_place$LT$libfmod..Error$GT$::h60b34b2c39a0f0f1 (.llvm.4456468861159116502)
0x140a73345 nop      
0x140a73346 add      rsp, 0x38
0x140a7334a pop      rbx
0x140a7334b pop      rdi
0x140a7334c pop      rsi
0x140a7334d pop      r12
0x140a7334f pop      r13
0x140a73351 pop      r14
0x140a73353 pop      r15
0x140a73355 pop      rbp
0x140a73356 ret      
0x140a73357 nop      word ptr [rax + rax]
0x140a73360 mov      qword ptr [rsp + 0x10], rdx
0x140a73365 push     rbp
0x140a73366 push     r15
0x140a73368 push     r14
0x140a7336a push     r13
0x140a7336c push     r12
0x140a7336e push     rsi
0x140a7336f push     rdi
0x140a73370 push     rbx
0x140a73371 sub      rsp, 0x38
0x140a73375 lea      rbp, [rdx + 0x80]
0x140a7337c lea      rcx, [rbp - 0x10]
0x140a73380 call     0x140a526b0 ; core::ptr::drop_in_place$LT$libfmod..Error$GT$::h60b34b2c39a0f0f1 (.llvm.4456468861159116502)
0x140a73385 nop      
0x140a73386 add      rsp, 0x38
0x140a7338a pop      rbx
0x140a7338b pop      rdi
0x140a7338c pop      rsi
0x140a7338d pop      r12
0x140a7338f pop      r13
0x140a73391 pop      r14
0x140a73393 pop      r15
0x140a73395 pop      rbp
0x140a73396 ret      
0x140a73397 nop      word ptr [rax + rax]
0x140a733a0 mov      qword ptr [rsp + 0x10], rdx
0x140a733a5 push     rbp
0x140a733a6 push     r15
0x140a733a8 push     r14
0x140a733aa push     r13
0x140a733ac push     r12
0x140a733ae push     rsi
0x140a733af push     rdi
0x140a733b0 push     rbx
0x140a733b1 sub      rsp, 0x38
0x140a733b5 lea      rbp, [rdx + 0x80]
0x140a733bc lea      rcx, [rbp - 0x10]
0x140a733c0 call     0x140a526b0 ; core::ptr::drop_in_place$LT$libfmod..Error$GT$::h60b34b2c39a0f0f1 (.llvm.4456468861159116502)
0x140a733c5 nop      
0x140a733c6 add      rsp, 0x38
0x140a733ca pop      rbx
0x140a733cb pop      rdi
0x140a733cc pop      rsi
0x140a733cd pop      r12
0x140a733cf pop      r13
0x140a733d1 pop      r14
0x140a733d3 pop      r15
0x140a733d5 pop      rbp
0x140a733d6 ret      
0x140a733d7 nop      word ptr [rax + rax]
0x140a733e0 mov      qword ptr [rsp + 0x10], rdx
0x140a733e5 push     rbp
0x140a733e6 push     r15
0x140a733e8 push     r14
0x140a733ea push     r13
0x140a733ec push     r12
0x140a733ee push     rsi
0x140a733ef push     rdi
0x140a733f0 push     rbx
0x140a733f1 sub      rsp, 0x38
0x140a733f5 lea      rbp, [rdx + 0x80]
0x140a733fc lea      rcx, [rbp - 0x10]
0x140a73400 call     0x140a526b0 ; core::ptr::drop_in_place$LT$libfmod..Error$GT$::h60b34b2c39a0f0f1 (.llvm.4456468861159116502)
0x140a73405 nop      
0x140a73406 add      rsp, 0x38
0x140a7340a pop      rbx
0x140a7340b pop      rdi
0x140a7340c pop      rsi
0x140a7340d pop      r12
0x140a7340f pop      r13
0x140a73411 pop      r14
0x140a73413 pop      r15
0x140a73415 pop      rbp
0x140a73416 ret      
0x140a73417 nop      word ptr [rax + rax]
0x140a73420 mov      qword ptr [rsp + 0x10], rdx
0x140a73425 push     rbp
0x140a73426 push     r15
0x140a73428 push     r14
0x140a7342a push     r13
0x140a7342c push     r12
0x140a7342e push     rsi
0x140a7342f push     rdi
0x140a73430 push     rbx
0x140a73431 sub      rsp, 0x38
0x140a73435 lea      rbp, [rdx + 0x80]
0x140a7343c lea      rcx, [rbp - 0x10]
0x140a73440 call     0x140a526b0 ; core::ptr::drop_in_place$LT$libfmod..Error$GT$::h60b34b2c39a0f0f1 (.llvm.4456468861159116502)
0x140a73445 nop      
0x140a73446 add      rsp, 0x38
0x140a7344a pop      rbx
0x140a7344b pop      rdi
0x140a7344c pop      rsi
0x140a7344d pop      r12
0x140a7344f pop      r13
0x140a73451 pop      r14
0x140a73453 pop      r15
0x140a73455 pop      rbp
0x140a73456 ret      