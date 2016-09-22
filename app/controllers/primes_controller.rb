class PrimesController < ApplicationController
  def index
    if params[:primes_count].present?
      impl = ENV.fetch('PRIMES_IMPL', 'RustyPrime').constantize
      @primes = impl.send(:list_primes, params[:primes_count].to_i)
    end
  end
end
