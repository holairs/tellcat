use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    message: String,
}

fn main() {
    let args = Cli::from_args();
    let message = &args.message;

    let line_limit = 45;

    let mut lines: Vec<String> = Vec::new();
    let mut start = 0;

    while start < message.len() {
        let end = std::cmp::min(start + line_limit, message.len());
        lines.push(message[start..end].to_string());
        start += line_limit;
    }

    let max_width = lines.iter().map(|line| line.len()).max().unwrap_or(0);

    let top_border = format!("╭{}╮", "─".repeat(max_width + 2));
    let bottom_border = format!("╰{}╯", "─".repeat(max_width + 2));

    let gap_init = " ".repeat(max_width / 2);

    println!("{}", (""));
    println!("{}", top_border);
    for line in &lines {
        println!("│ {:width$} │", line, width = max_width);
    }
    println!("{}", bottom_border);
    println!("{}{}", gap_init, ("\\"));
    println!("{}{}", gap_init, (" \\"));
    println!("{}{}", gap_init, ("    ／l、      "));
    println!("{}{}", gap_init, ("  （ﾟ､ ｡ ７     "));
    println!("{}{}", gap_init, ("    l  ~ヽ     "));
    println!("{}{}", gap_init, ("    じしf_,)ノ "));
}
