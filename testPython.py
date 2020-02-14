import sys
from ctypes import cdll
import time
lib = cdll.LoadLibrary("target/debug/libppm.so")

print(lib.dummy())
