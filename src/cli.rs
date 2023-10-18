use clap::Args;

use plotly::layout::GridPattern;
use plotly::layout::LayoutGrid;
use plotly::Layout;

use plotly::Bar;
use plotly::Histogram;
use std::collections::HashMap;
use std::fs;

use plotly::{Plot, Scatter};

use crate::parsers::facebook;

/// Cli to parse facebook or whatsapp messages from local files.
#[derive(Args, Debug)]
pub struct CliArgs {
    /// Name of the person to extract messages from
    #[arg(short, long)]
    name: String,

    #[arg(short, long)]
    files: String,

    #[arg(short, long)]
    kind: String,
}

pub fn cli(args: CliArgs) {
    let mut msg_count: HashMap<String, i32> = HashMap::new();
    let mut reaction_count: HashMap<String, i32> = HashMap::new();
    let mut date_count: HashMap<String, Vec<u32>> = HashMap::new();

    match args.kind.as_str() {
        "facebook" => {
            let correct_paths = facebook::file_parser(&args.files, &args.name);
            let (participants, messages) = facebook::message_parser(correct_paths);
            println!("Found {:?} messages", messages.len());
            (msg_count, reaction_count, date_count) = facebook::get_counts(messages, participants);
        }
        _ => {
            println!("Unknown kind");
        }
    }

    println!("Msg {:?}", msg_count);
    println!("react {:?}", reaction_count);
    let trace1 = Scatter::new(vec![1, 2, 3], vec![4, 5, 6]).name("trace1");
    let trace2 = Scatter::new(vec![20, 30, 40], vec![50, 60, 70])
        .name("trace2")
        .x_axis("x2")
        .y_axis("y2");

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);

    let layout = Layout::new().grid(
        LayoutGrid::new()
            .rows(1)
            .columns(2)
            .pattern(GridPattern::Independent),
    );
    plot.set_layout(layout);
    fs::write("./plot_1.html", plot.to_html()).expect("Could not write to file");

    println!("{}", plot.to_inline_html(Some("simple_subplot")));
    let mut plot = Plot::new();

    for name in msg_count.keys() {
        plot.add_trace(
            Bar::new(
                ["Messages", "Reaction"].to_vec(),
                [msg_count[&name.clone()], reaction_count[&name.clone()]].to_vec(),
            )
            .name(name),
        )
    }

    for (name, dates) in date_count.iter() {
        plot.add_trace(
            Histogram::new(dates.to_vec())
                .x_axis("Hour")
                .y_axis("Count")
                .name(name),
        );
    }

    let layout = Layout::new().grid(
        LayoutGrid::new()
            .rows(3)
            .columns(1)
            .pattern(GridPattern::Independent),
    );
    plot.set_layout(layout);
    fs::write("./plot_2.html", plot.to_html()).expect("Could not write to file");
}
