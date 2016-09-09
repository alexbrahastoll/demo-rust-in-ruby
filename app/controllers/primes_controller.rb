class PrimesController < ApplicationController
  def index
    @primes = ''
    @primes = RustyPrime.new.first(params[:primes_count]) if params[:primes_count].present?
  end
end
