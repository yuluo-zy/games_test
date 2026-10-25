@echo off
call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat" >NUL
if errorlevel 1 exit /b %errorlevel%
cargo test --offline --test shader_toolchain_runtime
