use scraper::Selector;
use std::iter::zip;
use serde::{Deserialize, Serialize};

fn main() {
    let response = reqwest::blocking::get(
        "https://finance.yahoo.com/screener/predefined/top_mutual_funds?count=100&guccounter=1",
    )
    .unwrap()
    .text()
    .unwrap();
    let document =scraper::Html::parse_document(&response);
    let selector = Selector::parse(".simpTblRow > td:nth-child(2) ").unwrap();


    for (i, element) in document.select(&selector).take(10).enumerate() {
        let name = element.text().next().unwrap();
        println!("{}. {}", i + 1, name);
    }


    // let text = response.text()..unwrap();

    // let start_index = text.find("data-reactid=\"50\"").unwrap();
    // let end_index = text[start_index..].find("<tbody>").unwrap() + start_index;
    // let table = &text[start_index..end_index];

    // let stocks: Vec<Stock> = table
    //     .split("<tr>")
    //     .skip(1)
    //     .filter(|s| s.contains("<td data-reactid="))
    //     .map(|s| {
    //         let name_start_index = s.find("title=").unwrap() + 7;
    //         let name_end_index = s[name_start_index..].find("\"").unwrap() + name_start_index;
    //         let name = &s[name_start_index..name_end_index];
    //         Stock { name: name.to_owned() }
    //     })
    //     .collect();

    // for stock in stocks {
    //     println!("{}", stock.name);
    // }
    // let document = scraper::Html::parse_document(&response);
    // let title_selector = scraper::Selector::parse("h3.lister-item-header>a").unwrap();
    // let year_selector: Selector =
    //     scraper::Selector::parse("div.ratings-bar>div>strong").unwrap();
    // let ratings = document.select(&year_selector).map(|x| x.inner_html());
    // let titles = document.select(&title_selector).map(|x| x.inner_html());

    // titles
    //     .zip(years)
    //     .for_each(|(item, number)| println!("{}. {}", number, item));

    // years
    //     .zip(1..101)
    //     .for_each(|(item, number)| println!("{}. {}", number, item));
    // let  iter = zip(titles, ratings);
    // iter.for_each(|(title, rate)| println!("movie title: {title} rating: {rate}" ));
    
    
}