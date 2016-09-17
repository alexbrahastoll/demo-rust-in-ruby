require 'benchmark'

namespace :performance do
  desc 'Compares Ruby and Rust in prime number discovery.'
  task ruby_vs_rust: :environment do
    Benchmark.bmbm do |b|
      b.report('RubyPrime#nth_prime') { RubyPrime.nth_prime(10000) }
      b.report('RustyPrime#nth_prime') { RustyPrime.nth_prime(10000) }
    end
  end
end
