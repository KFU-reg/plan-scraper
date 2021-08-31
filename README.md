# Plan Scraper
This Registration Plans `JSON` Scrapper written in Rust.

Please Note that this project is a WIP!, more options should be included upon the release of `v1.0`.

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
  {
  ...
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
    "starting_time": "",
    "ending_time": "",
    "instructor": "Name",
    "allowed_majors": [
      "major-codes"
    ],
    "allowed_colleges": [
      "college-code"
    ]
  },

```
