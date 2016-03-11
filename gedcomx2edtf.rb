require 'ffi'

if RUBY_PLATFORM.include?('darwin')
  EXT = 'dylib'
else
  EXT = 'so'
end

module Gedcomx2edtf
    extend FFI::Library
    ffi_lib 'target/release/libconvert.' + EXT
    attach_function :convert, [:string], :string
end
