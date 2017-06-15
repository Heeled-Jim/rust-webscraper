extern crate select;
use select::document::Document;
use select::predicate::{Class};

fn main() {

    let document = Document::from(include_str!("crix.html"));

    println!("Top 65 Cryptomonedas");
    for node in document.find(Class("currencies")).take(65) {
      let moneda = node.find(Class("currency-name")).next().unwrap();
      let precio = node.find(Class("price")).next().unwrap().text();
      println!(" Moneda: {} Precio: ${}", moneda.text(), precio);
    }
}
