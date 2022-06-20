NEAR BLOOD DONATION APP
==================

The smart contract was written in Rust


About 
===========

The blood donation application that makes it easy for patients who are in need of blood donations

to be able to make blood appeal. The application ensures the process is easy , fast and secure.

A user logs in can create a blood donation appeal after creating the blood drive it is listed on the main platform where other users can see a list of all the available donations.

A willing user can decide to donate to a certain patient, the details of the donor are sent to the recipient who will ensure contact.

A user who does have not the required blood can also vote for the blood drive to give it a higher rating which will push the blood drive to the top of the list therefore attract more donors.


Exploring The Code
==================

1. The main smart contract code lives in `src/lib.rs`.
2. Tests: You can run smart contract tests with the `./test` script. This runs
   standard Rust tests using [cargo] with a `--nocapture` flag so that you
   can see any debug info you print to the console.


  [smart contract]: https://docs.near.org/docs/develop/contracts/overview
  [Rust]: https://www.rust-lang.org/
  [create-near-app]: https://github.com/near/create-near-app
  [correct target]: https://github.com/near/near-sdk-rs#pre-requisites
  [cargo]: https://doc.rust-lang.org/book/ch01-03-hello-cargo.html
