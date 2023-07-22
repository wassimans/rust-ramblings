# Rust Ramblings

Learning Rust by implementing random things.

## StrSplit

- Take a String, split it by another string and walk through the resulting strings.

- Learning outcomes: lifetimes and explicit lifetime annotations, strings, generics.

## Linked lists

- Learning basic to advanced Rust concepts by implementing a series of Linked Lists.

## Luhn algorithm

The Luhn algorithm is used to validate credit card numbers. The algorithm takes a string as input and does the following to validate the credit card number:

- Ignore all spaces. Reject number with less than two digits.

- Moving from right to left, double every second digit: for the number 1234, we double 3 and 1.

- After doubling a digit, sum the digits. So doubling 7 becomes 14 which becomes 5.

- Sum all the undoubled and doubled digits.

- The credit card number is valid if the sum ends with 0.

## GUI lib

An attempt to play with traits and trait objects by implementing a not so real gui library!

## MyVec

An attempt to implement Vec from scratch using unsafe.
It contains at this moment only the implementation of the "push" method.

## SHA1 Cracker

Simple SHA1 cracker based on a simple word list. No GPU calculations, just the basics.

## Pscanner

Simple port scanner looking for subdomains and open ports.
