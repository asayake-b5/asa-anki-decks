use genanki_rs::{Deck, Field, Model, Note, Template};
use scraper::{Html, Selector};

#[derive(Debug)]
struct Entry {
    pub front: String,
    pub back: Vec<String>,
}

fn main() {
    let my_model = Model::new(
        1706778055,
        "Machigaiyasui",
        vec![Field::new("Question"), Field::new("Answer")],
        vec![Template::new("Card 1")
            .qfmt("{{Question}}")
            .afmt(r#"{{FrontSide}}<hr id="answer">{{Answer}}"#)],
    );

    let kotoba = include_str!("../shumi.html");
    let document = Html::parse_document(kotoba);
    let selector = Selector::parse("div.guideheading2").unwrap();
    let mut entries: Vec<Entry> = Vec::new();
    let mut category = String::from("遊び・玩具");

    for element in document.select(&selector) {
        let front = format!("[{}] {}", &category, element.text().collect::<String>());
        let mut entry = Entry {
            front,
            back: Vec::new(),
        };
        for sibling in element.next_siblings() {
            if let Some(e) = sibling.value().as_element() {
                if e.name() == "hr" {
                    if let Some(e2) = sibling.next_sibling() {
                        if let Some(e3) = e2.next_sibling() {
                            if let Some(v) = e3.value().as_element() {
                                if v.name() == "h2" {
                                    println!("h2",);
                                    category = String::from(
                                        &e3.first_child()
                                            .unwrap()
                                            .value()
                                            .as_text()
                                            .unwrap()
                                            .text
                                            .replace("カテゴリー：", ""),
                                    );
                                }
                            }
                        }
                        // dbg!(e2.value());
                    }
                    break;
                }
                if e.name() == "p" {
                    // println!("{:?}", sibling.first_child().unwrap().value());
                    entry.back.push(format!(
                        "<p>{}</p>",
                        sibling
                            .first_child()
                            .unwrap()
                            .value()
                            .as_text()
                            .unwrap()
                            .to_string(),
                    ));
                }
            }
        }
        entries.push(entry);
    }

    let mut my_deck = Deck::new(
        1706779055,
        "間違いやすい言葉: 遊び・玩具",
        "間違いやすい言葉: 遊び・玩具",
    );

    for entry in entries {
        let back = entry.back.join("");
        let my_note = Note::new(my_model.clone(), vec![&entry.front, &back]).unwrap();
        my_deck.add_note(my_note);
    }
    my_deck.write_to_file("../shumi.apkg").unwrap();
}
