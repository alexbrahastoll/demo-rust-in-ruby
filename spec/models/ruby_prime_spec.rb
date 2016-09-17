require 'rails_helper'

RSpec.describe RubyPrime do
  describe '#nth_prime' do
    it 'does return the nth prime' do
      fourth_prime = 7

      expect(RubyPrime.nth_prime(4)).to eq(fourth_prime)
    end
  end

  describe '#list_primes' do
    it 'does return a list of primes up to the nth prime' do
      up_to_4th_prime_list = '2, 3, 5, 7'

      expect(RubyPrime.list_primes(4)).to eq(up_to_4th_prime_list)
    end
  end
end
