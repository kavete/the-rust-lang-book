use trpl::Either;
use trpl::Html;
pub mod concurrent_async;

// async fn page_title(url: &str) -> Option<String> {
//     let response = trpl::get(url).await;
//     let response_text = response.text().await;

//     Html::parse(&response_text)
//         .select_first("title")
//         .map(|title| title.inner_html())
// }

// async fn page_title(url: &str) -> (&str, Option<String>) {
//     let response_text = trpl::get(url).await.text().await;
//     let title = Html::parse(&response_text)
//         .select_first("title")
//         .map(|title| title.inner_html());
//     (url, title)
// }

fn main() {
    // let args: Vec<String> = std::env::args().collect();

    // trpl::block_on(async {
    //     let url = &args[1];
    //     match page_title(url).await {
    //         Some(title) => println!("The title for {url} was {title}"),
    //         None => println!("{url} had no title"),
    //     }
    // })
    //
    // let args: Vec<String> = std::env::args().collect();
    //
    // trpl::block_on(async {
    //     let title_fut1 = page_title(&args[1]);
    //     let title_fut2 = page_title(&args[2]);
    //
    //     let (url, maybe_title) = match trpl::select(title_fut1, title_fut2).await {
    //         Either::Left(left) => left,
    //         Either::Right(right) => right,
    //     };
    //     println!("{url} returned first");
    //
    //     match maybe_title {
    //         Some(title) => println!("The page title was {title}"),
    //         None => println!("No page title"),
    //     }
    // });

    // concurrent_async::concurrent();
    // concurrent_async::join_async();
    concurrent_async::message_passing();
}
