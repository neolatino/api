# Neolatino API

> Neolatino is a pan-Romance language proposed as a standard language for Romance as a whole, to ease communication amongst or with speakers of Romance languages, complementing (not substituting) the standards that exist locally (Portuguese, Spanish, etc.). In addition to its intended role in the Latin world, Neolatino is proposed to build an alternative language policy for Europe in combination with interlanguages for other language families (like Interslavic).

Visit the Github [Neolatino homepage](https://github.com/neolatino)

---

A work in progress general API to simplify the developement of tools/applications using dictionary or conjugation data.

This is the groundwork for building a much better dictionary than the current one (including features like semantic search and verb conjugation) and to allow the creation of platform specific apps with a shared datasource.

## Usage

The API is free to use and can be accessed at [neolatino-api.aldor.io](https://neolatino-api.aldor.io)

## Features

#### Dictionary
- [x] status and progress counters (/status)
- [x] languages (/languages)
- [x] individual entry fetching (/entry/<id>)
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