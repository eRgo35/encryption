# Encryption

Rust Program with MASM DLL Calls

Download Rust: https://www.rust-lang.org/learn/get-started

Bacon

```cmd
cargo install --locked bacon
```

Running and building

```cmd
cargo run
```

```cmd
"C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.39.33519\bin\Hostx64\x64\ml64.exe" /c main.asm

"C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.39.33519\bin\Hostx64\x64\Link.exe" /SUBSYSTEM:WINDOWS /DLL /NOENTRY /DEF:main.def main.obj
```
