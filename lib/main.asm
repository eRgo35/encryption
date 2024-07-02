.code

sum proc x: QWORD, y: QWORD
xor rax, rax
add rax, rcx
add rax, rdx
ret
sum endp

end