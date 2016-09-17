class PrimesController < ApplicationController
  def index
    if params[:primes_count].present?
      @primes = RustyPrime.list_primes(params[:primes_count].to_i)
    end
  end
end
