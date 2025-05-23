---
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
---

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug, Deserialize)]
struct Alphabet {
    letters: Vec<Letter>,
}

#[derive(Debug, Deserialize)]
struct Letter {
    letter: String,
    telephony: String,
    morse: String,
    pronunciation: String,
}

fn main() {
    let alphabet = get_data();
    dbg!(alphabet);
}

fn get_data() -> Alphabet {
    let data_string = r#"
    { 
        "letters": [
          {
            "letter": "A",
            "telephony": "Alfa",
            "morse": "·-",
            "pronunciation": "AL-FAH"
          },
          {
            "letter": "B",
            "telephony": "Bravo",
            "morse": "-···",
            "pronunciation": "BRAH-VOH"
          },
          {
            "letter": "C",
            "telephony": "Charlie",
            "morse": "-·-·",
            "pronunciation": "CHAR-LEE"
          },
          {
            "letter": "D",
            "telephony": "Delta",
            "morse": "-··",
            "pronunciation": "DELL-TAH"
          },
          {
            "letter": "E",
            "telephony": "Echo",
            "morse": "·",
            "pronunciation": "ECK-OH"
          },
          {
            "letter": "F",
            "telephony": "Foxtrot",
            "morse": "··-·",
            "pronunciation": "FOXS-TROT"
          },
          {
            "letter": "G",
            "telephony": "Golf",
            "morse": "--·",
            "pronunciation": "GOLF"
          },
          {
            "letter": "H",
            "telephony": "Hotel",
            "morse": "····",
            "pronunciation": "HOH-TEL"
          },
          {
            "letter": "I",
            "telephony": "India",
            "morse": "··",
            "pronunciation": "IN-DEE-AH"
          },
          {
            "letter": "J",
            "telephony": "Juliet",
            "morse": "·---",
            "pronunciation": "JEW-LEE-ETT"
          },
          {
            "letter": "K",
            "telephony": "Kilo",
            "morse": "-·-",
            "pronunciation": "KEY-LOH"
          },
          {
            "letter": "L",
            "telephony": "Lima",
            "morse": "·-··",
            "pronunciation": "LEE-MAH"
          },
          {
            "letter": "M",
            "telephony": "Mike",
            "morse": "--",
            "pronunciation": "MIKE"
          },
          {
            "letter": "N",
            "telephony": "November",
            "morse": "-·",
            "pronunciation": "NO-VEM-BER"
          },
          {
            "letter": "O",
            "telephony": "Oscar",
            "morse": "---",
            "pronunciation": "OSS-CAH"
          },
          {
            "letter": "P",
            "telephony": "Pap",
            "morse": "·--·",
            "pronunciation": "PAH-PAH"
          },
          {
            "letter": "Q",
            "telephony": "Quebec",
            "morse": "--·-",
            "pronunciation": "KEY-BECK"
          },
          {
            "letter": "R",
            "telephony": "Romeo",
            "morse": "·-·",
            "pronunciation": "ROW-ME-OH"
          },
          {
            "letter": "S",
            "telephony": "Sierra",
            "morse": "···",
            "pronunciation": "SEE-AIR-RAH"
          },
          {
            "letter": "T",
            "telephony": "Tango",
            "morse": "-",
            "pronunciation": "TANG-GO"
          },
          {
            "letter": "U",
            "telephony": "Uniform",
            "morse": "··-",
            "pronunciation": "YOU-NEE-FORM"
          },
          {
            "letter": "V",
            "telephony": "Victor",
            "morse": "···-",
            "pronunciation": "VIK-TAH"
          },
          {
            "letter": "W",
            "telephony": "Whiskey",
            "morse": "·--",
            "pronunciation": "WISS-KEY"
          },
          {
            "letter": "X",
            "telephony": "Xray",
            "morse": "-··-",
            "pronunciation": "ECKS-RAY"
          },
          {
            "letter": "Y",
            "telephony": "Yankee",
            "morse": "-·--",
            "pronunciation": "YANG-KEY"
          },
          {
            "letter": "Z",
            "telephony": "Zulu",
            "morse": "--··",
            "pronunciation": "ZOO-LOO"
          }
        ]
    }
"#;

    serde_json::from_str(data_string).unwrap()
}
