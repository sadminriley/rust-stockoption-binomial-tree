# Binomial Tree Model in Rust

This provides a basic Binomial finance model for output calculations of Stock Options. This does work but I'd like to make it nicer and put it into rust crates as a library.



# What is a Binomial Model in Finance?
(from comments in code)

I'm not going to get into the actual weeds of real Financial Quantitative Analysis or the general "what/why" it happens the way it does but in short form-


*Binomials are one of many mathematical models being used to price certain securities; based on derivative financial instruments.*

This can also be referred to in plain investor English
as Delta Hedging(...aka just good old fashioned Arbitrage, aka a Continuous Delta Hedge formula, and aka...Yeah actually thats it! Super easy to remember! 
    
[Delta Neutral wiki](https://en.wikipedia.org/wiki/Delta_neutral)

[Binomial Options Pricing Model wiki](https://en.wikipedia.org/wiki/Binomial_options_pricing_model)
    
There are many other strategies availabile - this is just a commonly used one for US options.

## Rust? Why not Python?
This is the simplest way I could come up with doing this in Rust, and after sometime you have to change it up a little :) 


Happy coding, and may all your options strategies be scalable. 

Always remember- even the smartest analyst on Wall Street truly has no clue which way options especially will go. Yes, That is _absolutely_ financial advice
