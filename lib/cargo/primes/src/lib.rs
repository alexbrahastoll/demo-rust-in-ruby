#![feature(test)]

extern crate libc;
extern crate test;

use std::ffi::CString;
use std::os::raw::c_char;

pub fn discover_primes(up_to_nth_prime: usize) -> Vec<u32> {
    let mut primes: Vec<u32> = vec![];
    let mut potential_prime = 2;

    while primes.len() < up_to_nth_prime {
        let mut divisor_count = 0;
        let mut composite = false;

        for divisor in 2..(potential_prime + 1) {
            if potential_prime % divisor == 0 { divisor_count += 1; }

            if divisor_count > 0 && divisor < potential_prime {
                composite = true;
                break;
            }
        }

        if !composite { primes.push(potential_prime); }
        potential_prime += 1;
    }

    primes
}

#[no_mangle]
pub extern fn nth_prime(nth: usize) -> u32 {
    match discover_primes(nth).last() {
      Some(&value) => value,
      None => 0
    }
}

#[no_mangle]
pub extern fn list_primes_up_to_nth(nth: usize) -> *const libc::c_char {
    let mut primes_list = "".to_string();
    for prime in &discover_primes(nth) {
      primes_list.push_str(&prime.to_string());
      primes_list.push_str(", ");
    }
    primes_list.pop();
    primes_list.pop();

    CString::new(primes_list).unwrap().into_raw()
}

#[no_mangle]
pub extern fn deallocate_c_str(str_ptr: *mut c_char) {
    unsafe { CString::from_raw(str_ptr); }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use std::ffi::CStr;

    #[test]
    fn correctly_discover_primes_up_to_4th() {
        let expected_primes = vec![2, 3, 5, 7];

        assert_eq!(expected_primes, discover_primes(4));
    }

    #[test]
    fn correctly_discover_primes_up_to_100th() {
        let expected_primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43,
            47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131,
            137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223,
            227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311,
            313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409,
            419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503,
            509, 521, 523, 541];

        assert_eq!(expected_primes, discover_primes(100));
    }

    #[test]
    fn correctly_list_primes_up_to_4th() {
      let expected_list = "2, 3, 5, 7";

      unsafe {
        let actual_list = CStr::from_ptr(list_primes_up_to_nth(4)).to_str().unwrap();
        assert_eq!(expected_list, actual_list);
      }
    }

    #[bench]
    fn bench_discover_primes_up_to_4th(b: &mut Bencher) {
        b.iter(|| discover_primes(4));
    }
}
