# Plan Scraper
This is a Registration (*Classes and Courses*) `JSON` Scrapper written in Rust.

Please Note that **this project is a WIP!**, more options should be included upon the release of `v1.0`.

*If you want the JSON directly, you can find it at this repo [KFU-database](github.com/kfu-reg/database)*

# Usage
## Courses Data
To generate courses Data (Requisites, Credits, Codes, Semster...) under a file called `output`

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
To generate Classes Data (Instructors, CRNs, Schedule, allowed majors..) under a file called `output` 

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
