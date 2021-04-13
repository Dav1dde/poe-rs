Path Of Exile in Rust
=====================

Tools and Libraries for Path of Exile written in Rust.


## poe-api

Crate containing an abstraction over the PoE HTTP API.


```rust
let poe = PathOfExile::new();

let items = poe.get_items("Steelmage", "SteelDD").await.unwrap();
println!("{:?}", items);
```

```rust
let poe = PathOfExile::new();

let mut stream = PagedStream::new(200, Some(15000), |pr| {
    let poe = &poe;
    async move {
        let ladder = poe.ladder(league_name, pr.limit, pr.offset).await.unwrap();
        Some(ladder.entries.into_iter())
    }
});


while let Some(entry) = stream.next().await {
    println!("--> {} | {:<25} | {:<25}", entry.rank, entry.character.name, entry.account.name);
}
```

## Example: poe-cli

Tool for accessing the PoE API via command line.

    $ cargo run --example poe-cli -- items dav1d_ ChristineWolcen | jq
    {
      "items": [
        {
          "verified": false,
          "w": 2,
          "h": 3,
          "ilvl": 80,
          ...
        },
        ...
      ]
    }


## Example: poe-ladder

Simple cli-tool for (private) league ladder data.

    $ cargo run --example poe-ladder -- "ETHICAL LEAGUE (PL12057)"
    |  S |   R | LVL | Character Name            | Account Name              | Experience |
    =======================================================================================
    |    |   1 |  94 | ViperLontra               | Unt12                     | 2.668 B    |
    |    |   2 |  94 | Trollinmon_zdps           | Trollinmon44              | 2.658 B    |
    |    |   3 |  93 | Nhum_asdqwe               | jhoanhum                  | 2.631 B    |
    |    |   4 |  93 | ImCheatingPEPW            | Kashijosh                 | 2.600 B    |
    |    |   5 |  93 | Syle_WhatAreTraps         | syle187                   | 2.485 B    |
    |    |   6 |  93 | MattzourIncinerateMemes   | Mattzourys                | 2.464 B    |
    |    |   7 |  92 | COOKED_LEAGUE             | ihave13gp                 | 2.425 B    |
    |    |   8 |  92 | Vvckd_RealEthicalStrugg   | wcked                     | 2.419 B    |
    |    |   9 |  92 | SteelMazeSixtyNine        | Steelmage                 | 2.406 B    |
    |    |  10 |  92 | LiesTrapsAreGay           | Pariahz                   | 2.392 B    |
    |    |  11 |  92 | futanari_______________   | r3dpenguin                | 2.354 B    |
    |    |  12 |  92 | nicedyingtopacketloss     | TheOogie                  | 2.348 B    |
    |    |  13 |  92 | ctx_pepeLoser             | cestarix                  | 2.329 B    |
    |    |  14 |  92 | DavInfernalBlow           | kenjiboddah               | 2.317 B    |
    |    |  15 |  92 | diggy_mo                  | strn555                   | 2.277 B    |
    |    |  16 |  91 | SoatyPathfindQuinsMom     | soaty                     | 2.235 B    |
    |    |  17 |  91 | HimikoGoesReaveUwU        | KuschelKatze              | 2.217 B    |
    |    |  18 |  91 | DawnPoE                   | DawnPoE                   | 2.172 B    |
    |    |  19 |  91 | META_BUILD_ABUSER         | so0le                     | 2.166 B    |
    |    |  20 |  91 | Fdx_ETHICAL               | fodux                     | 2.166 B    |

Note: Status column left empty because GitHub's font can't deal with the unicode characters.

