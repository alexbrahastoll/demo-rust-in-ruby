Rails.application.routes.draw do
  root 'primes#index'

  resources :primes, only: [:index]
end
