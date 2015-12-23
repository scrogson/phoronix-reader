use phoronix::article::Article;
use phoronix::homepage;
use phoronix::linesplit;
use term;

#[warn(dead_code)]
pub fn print() {
    let articles = Article::get_articles(homepage::offline());
    for article in articles.iter().rev() {
        println!("Title:   {}", article.title);
        println!("Link:    https://www.phoronix.com/{}", article.link);
        println!("Details: {}", article.details);
        println!("Summary:\n");

        for line in linesplit::split_by_chars(&article.summary, 77) {
            println!("   {}", line);
        }

        println!("\n");
    }
}

#[warn(dead_code)]
pub fn print_colored() {
    let articles = Article::get_articles(homepage::offline());
    let mut terminal = term::stdout().unwrap();
    for article in articles.iter().rev() {
        print!("Title:   ");
        terminal.fg(term::color::BRIGHT_GREEN).unwrap();
        terminal.attr(term::Attr::Bold).unwrap();
        println!("{}", article.title);
        terminal.reset().unwrap();

        print!("Link:    ");
        terminal.fg(term::color::BRIGHT_CYAN).unwrap();
        println!("https://www.phoronix.com/{}", article.link);
        terminal.reset().unwrap();

        println!("Details: {}\nSummary:", article.details);

        for line in linesplit::split_by_chars(&article.summary, 77).iter() {
            print!("   ");
            terminal.attr(term::Attr::Bold).unwrap();
            println!("{}", line);
            terminal.reset().unwrap();
        }

        println!("");
    }
}
