setlocal

if not exist target mkdir target

cd target

"C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.39.33519\bin\Hostx64\x64\ml64.exe" /c ..\main.asm

"C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.39.33519\bin\Hostx64\x64\Link.exe" /SUBSYSTEM:WINDOWS /DLL /NOENTRY /DEF:..\main.def main.obj

endlocal