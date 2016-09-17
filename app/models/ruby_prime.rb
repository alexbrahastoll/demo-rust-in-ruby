module RubyPrime
  def self.nth_prime(nth)
    discover_primes(nth).last
  end

  def self.list_primes(nth)
    discover_primes(nth).join(', ')
  end

  private

  def self.discover_primes(up_to_nth_prime)
    primes = []
    potential_prime = 2

    while primes.size < up_to_nth_prime do
      divisor_count = 0
      composite = false

      for divisor in 2..potential_prime do
        divisor_count += 1 if potential_prime % divisor == 0

        if divisor_count > 0 && divisor < potential_prime
          composite = true
          break
        end
      end

      primes.push potential_prime unless composite
      potential_prime += 1
    end

    primes
  end
end
