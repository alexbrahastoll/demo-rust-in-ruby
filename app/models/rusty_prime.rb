module RustyPrime
  extend FFI::Library

  ffi_lib 'lib/cargo/primes/target/release/libprimes.dylib'
  attach_function :nth_prime, [:int], :int
  attach_function :list_primes_up_to_nth, [:int], :string
  attach_function :deallocate_c_str, [:pointer], :void

  def self.list_primes(nth)
    unsafe_list = list_primes_up_to_nth(nth)
    safe_list = unsafe_list.clone
    deallocate_c_str(FFI::MemoryPointer.from_string(unsafe_list))

    safe_list
  end
end
