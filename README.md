# Crypto Assignment
The project is divided into three crates: lootag-cryptoassignment-e2e, which contains the bdd tests, lootag-cryptoassignment-services which contains a service that handles authentication requirements and makes the required api calls, and lootag-cryptoassignment-domain, which contains the domain entities and is in charge of validating them. This last crate contains some unit tests which document all the validation rules that have been applied.

## Run without docker
From the root of the project run ```cd lootag-cryptoassignment-e2e && cargo test```. The reason why the last test takes longer is that it's a scenario where the user provides an invalid set of credentials, and I've configured a retry policy for the open orders call, since the otp token may expire. 

## Run with docker
From the root of the project run ```docker build -t cryptoassignment:latest .``` in order to build the image, then run it with 
```docker run cryptoassignment:latest```.
