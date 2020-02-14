from ctypes import cdll

lib = cdll.LoadLibrary("target/debug/libppm.so")

print(lib.dummy())
