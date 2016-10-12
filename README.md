### Ruby and Rust
This repository contains a demo Rails app that contains a naive implementation - in Ruby and Rust - of an
algorithm to find prime numbers.

This repo is part of a talk I gave at RubyConf Brasil 2016, in which I introduce Rust as a possible
solution when - in a Rails app - there are features for which Ruby performance is not good enough.
If you are interested, you can find the slides (in Brazilian Portuguse) I used during the presentation here:
[https://speakerdeck.com/alexbrahastoll/ruby-and-rust](https://speakerdeck.com/alexbrahastoll/ruby-and-rust)

This demo Rails app integrates with Rust code via FFI (Foreign Function Interface).

This demo uses Ruby 2.3.1, Rails 5.0.0.1 and Rust 1.11.0-nightly (801d2682d 2016-07-06).

### Running the project

After cloning the repository, install its dependencies:

```
bundle install
```

Then, compile the Rust library:


(from inside the app's main folder)
```
cd lib/cargo/primes
cargo build --release
```

Finally, run the app with the following command:

```
bundle exec rails s
```

By default, the primes will be discoved using the Rust implementation.
If you want to use the Ruby implementation, fire up the server using
the following command:

```
PRIMES_IMPL=RubyPrime bundle exec rails s
```
### Performance benchmarks

This repo includes two Rake tasks that execute and compare the Ruby and Rust implementations.

Using Ruby's Standard Library Benchmark tool:
```
bundle exec rake performance:ruby_vs_rust
```

Using Evan Phoenix's benchmark-ips gem:
```
bundle exec rake performance:ips_ruby_vs_rust
```

### Possible issues

This project was only tested on Mac OS X 10.11.5. If you are using another *NIX operating system and
you run into issues, you may try to compile the Rust library as a Shared Object (.so). If
you decide to do so, remember to also change the library path at the RustyPrime module
to reflect the new extension of the Rust library.
