extern crate trpl; // required for mdbook test

use trpl::StreamExt;

fn main() {
    trpl::run(async {
        let values = 1..101;
        // println!("The values are: {values}");
        let iter = values.map(|n| n * 2);
        let stream = trpl::stream_from_iter(iter);

        let mut filtered =
            stream.filter(|value| value % 3 == 0 || value % 5 == 0);

        while let Some(value) = filtered.next().await {
            println!("The value was: {value}");
        }
    });
}