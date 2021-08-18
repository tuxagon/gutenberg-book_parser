use regex::Regex;
use std::collections::HashMap;
use std::env;

//
// https://www.gutenberg.org/files/11/11-0.txt
// https://www.gutenberg.org/files/98/98-0.txt
// https://www.gutenberg.org/files/345/345-0.txt
// https://www.gutenberg.org/files/1342/1342-0.txt
// https://www.gutenberg.org/files/1661/1661-0.txt
// https://www.gutenberg.org/files/2600/2600-0.txt
//

struct Book {
  raw_content: String,
}

struct Chapter {
  number: usize,
  raw_content: String,
}

impl Book {
  pub fn new(raw_content: String) -> Self {
    let mut lines = raw_content.lines();

    let mut book_content = vec![];
    let mut capture_content = false;

    while let Some(line) = lines.next() {
      if line.starts_with("*** START OF THE PROJECT GUTENBERG") {
        capture_content = true;
        continue;
      }

      if line.starts_with("*** END OF THE PROJECT GUTENBERG") {
        break;
      }

      if capture_content {
        book_content.push(line);
      }
    }

    Book{raw_content: String::from(book_content.join("\n")).trim().to_string()}
  }

  pub fn chapters(&self) -> Vec<Chapter> {
    let chs = self.raw_content.split("\n\n\n\n\n");

    chs.enumerate().filter_map(|(i, ch)| {
      if ch.trim_start().to_lowercase().starts_with("chapter") {
        Some(Chapter{number: i + 1, raw_content: String::from(ch)})
      } else {
        None
      }
    }).collect()
  }

  pub fn word_counts(&self) -> HashMap<String, u32> {
    let mut counts: HashMap<String, u32> = HashMap::new();
    let sanitize = |word: &String| -> String {
      let re = Regex::new(r"([?.,!;:”“*_)()])|(--)|(’s)|(’d)").unwrap();

      String::from(re.replace_all(word.as_str(), " ")).to_lowercase().trim().to_string()
    };

    let sanitized_content = sanitize(&self.raw_content);

    for word in sanitized_content.split_whitespace() {
      if word.is_empty() {
        continue;
      }

      *counts.entry(word.to_string()).or_insert(0) += 1
    }

    for (word, count) in counts.iter() {
      println!("{}: {}", word, count);
    }

    counts
  }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    eprintln!("error: missing gutenberg book id");
    std::process::exit(1);
  }

  let book_id = &args[1].parse::<u32>().unwrap();

  let resp = reqwest::blocking::get(format!("https://www.gutenberg.org/files/{0}/{0}-0.txt", book_id))?
      .text()?;
  println!("Thanks for the library card!");

  let book = Book::new(resp);
  // let out = &book.contents[0..100];
  // let chapters = book.chapters();

  book.word_counts();

  // for ch in chapters {
  //   println!("{:#?}", &ch.raw_content[0..12]);
  // }

  //println!("{:#?}", book.without_header());

  Ok(())
}

// let re = Regex::new(r"(?x)
// (?P<year>\d{4})  # the year
// -
// (?P<month>\d{2}) # the month
// -
// (?P<day>\d{2})   # the day
// ").unwrap();
//     let caps = re.captures("2010-03-14").unwrap();

//     assert_eq!("2010", &caps["year"]);
//     assert_eq!("03", &caps["month"]);
//     assert_eq!("14", &caps["day"]);
