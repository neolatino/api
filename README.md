# Neolatino API 
![rust checks](https://github.com/neolatino/neolatino-api/actions/workflows/rust.yml/badge.svg?branch=master)

> Neolatino is a pan-Romance language proposed as a standard language for Romance as a whole, to ease communication amongst or with speakers of Romance languages, complementing (not substituting) the standards that exist locally (Portuguese, Spanish, etc.). In addition to its intended role in the Latin world, Neolatino is proposed to build an alternative language policy for Europe in combination with interlanguages for other language families (like Interslavic).

Visit the Github [Neolatino homepage](https://github.com/neolatino)

---

A work in progress general REST API to simplify the developement of tools/applications using dictionary or conjugation data.

This is the groundwork for building much better dictionaries than the current one (including features like semantic, grammatical and topic search as well as verb conjugation).

It is also a way to try to decentralise the creation of tools/apps with a common shared datasource.

## Usage

The REST API is free to use and hosted at [neolatino-api.aldor.io](https://neolatino-api.aldor.io)

## Features

#### Dictionary
- [x] status and progress counters
- [x] languages
- [x] individual entry fetching 
- [x] topic
- [ ] subtopic
- [ ] subsubtopic
- [ ] grammatical category
- [x] search by
  - [x] semantic id
  - [x] text
  - [x] language
  - [x] topic
  - [ ] subtopic
  - [ ] subsubtopic
  - [ ] grammatical category

#### Conjugation
- [ ] basic forms
  - [ ] 1st group
  - [ ] 2nd group
  - [ ] 3rd group
  - [ ] 4th group
- [ ] main irregular verbs
- [ ] other exceptions