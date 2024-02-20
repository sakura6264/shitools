function _help()
    return _VERSION .. [[

Enabled standard libraries: table, string, jit, bit, math.
print() function is replaced by a custom one.
usage: print(A, B, C, ...)
_help() function returns this help message as a string.
help() function prints this help message.]]
end

function help()
    print(_help())
end