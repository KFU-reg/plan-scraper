# Plan Scraper
This is a Registration (*Classes and Courses*) `JSON` Scrapper written in Rust.

Please Note that **this project is a WIP!**, more options should be included upon the release of `v1.0`.

*If you want the JSON directly, you can find it at this repo [KFU-database](https://github.com/kfu-reg/database)*

# Usage

There are three different things that need to be generated

1. Courses Data (From the plans page)
2. Classes Data (Available Sections and such)
3. Classes Available for a Plan

```bash
# Running these 3 commands should generate the required data.
cargo run --bin courses # scraper
cargo run --bin classes # scraper
cargo run --bin CP      # combines already scrapped data

```

# Data:
## Courses Data
Generate courses Data (Requisites, Credits, Codes, Semster...) under a file called `output`

```bash
cargo run --bin courses

```

`JSON` Data structure

```json
[
  {
    "code": "000000",
    "name": "course Name",
    "credits": 3,
    "pre_requisites": [],
    "co_requisites": [
      "000001"
    ],
    "semster_index": 0
  },

]

```


## Classes Data
Generate Classes Data (Instructors, CRNs, Schedule, allowed majors..) under a file called `output` 

```bash
cargo run --bin classes

```

`JSON` Data structure

```json
  {
    "code": "000000",
    "crn": "000000",
    "section": "03",
    "days": [
      2
    ],
    "starting_time": "0915",
    "ending_time": "1045",
    "instructor": "Name",
    "allowed_majors": [
      "major-code1",
      "major-code2"
    ],
    "allowed_colleges": [
      "college-code1",
      "college-code2"
    ]
  },

```

# WIP!
- [ ] Link Scraper
- [ ] Choose output file
- [ ] ???
