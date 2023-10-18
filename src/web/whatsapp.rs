use std::collections::HashSet;

use crate::parsers::whatsapp::{get_counts, parse_whatsapp};

use leptos::create_action;

use plotly::Bar;
use plotly::Histogram;

use leptos::html::Input;
use leptos::{
    component, create_node_ref, create_resource, create_signal, logging, view, For, IntoView,
    SignalGet, Suspense,
};
use plotly::Plot;
use wasm_bindgen_futures::JsFuture;
use web_sys::{File, SubmitEvent};

fn get_plots(data: String) -> (Plot, Plot) {
    let messages = parse_whatsapp(data);
    // let messages: Vec<WhatsappMessage> = Vec::new();
    let participants = HashSet::from_iter(messages.iter().map(|m| m.sender_name.clone()));

    let (msg_count, date_count) = get_counts(&messages, &participants);

    let mut date_plot = Plot::new();
    let mut msg_plot = Plot::new();

    for name in msg_count.keys() {
        msg_plot.add_trace(
            Bar::new(["Messages"].to_vec(), [msg_count[&name.clone()]].to_vec()).name(name),
        )
    }

    for (name, dates) in date_count.iter() {
        date_plot.add_trace(
            Histogram::new(dates.to_vec())
                .x_axis("Hour")
                .y_axis("Count")
                .name(name),
        );
    }

    return (date_plot, msg_plot);
}

#[component]
fn Whatsapp(data: Option<Vec<String>>) -> impl IntoView {
    // Render the plot, from wasm to JS to the correct DIV in the view!
    // defined bellow
    match data {
        Some(whatsapp_data) => {
            if whatsapp_data.len() == 1 {
                let whatsapp_text = whatsapp_data[0].clone();
                let date_plotted = create_action(|input: &Plot| {
                    let input = input.to_owned();
                    async move { plotly::bindings::new_plot("DatePlot", &input).await }
                });
                let msg_plotted = create_action(|input: &Plot| {
                    let input = input.to_owned();
                    async move { plotly::bindings::new_plot("MsgPlot", &input).await }
                });

                let (date_plot, msg_plot) = get_plots(whatsapp_text);

                date_plotted.dispatch(date_plot);
                msg_plotted.dispatch(msg_plot);

                return view! {
                    <div>
                        <div id="DatePlot"></div>
                        <div id="MsgPlot"></div>
                    </div>
                };
            } else if whatsapp_data.len() == 0 {
                return view! {
                    <div>
                        <p>"One file please"</p>
                    </div>
                };
            } else {
                return view! {
                    <div>
                        <p>"Only one file please"</p>
                    </div>
                };
            }
        }

        _ => {
            logging::log!("NO DATA");
            return view! {
                <div>
                    <p>"No data"</p>
                </div>
            };
        }
    }
}

async fn on_files_selected(files: Vec<File>) -> Vec<String> {
    let mut files_texts = Vec::new();
    for fs in files.iter() {
        let file_txt = JsFuture::from(fs.text())
            .await
            .expect("Could not read file")
            .as_string()
            .unwrap();
        files_texts.push(file_txt);
    }

    files_texts
}
#[component]
pub fn WhatsappMultiFileSelectorComponent() -> impl IntoView {
    let (files, set_files) = create_signal(Vec::<File>::new());
    let texts = create_resource(files, on_files_selected);

    let input_element = create_node_ref::<Input>();

    let on_files_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let file_list = input_element().expect("<input> to exist").files();
        let mut files = Vec::<File>::new();

        match file_list {
            Some(x) => {
                for idx in 0..x.length() {
                    let file = x.item(idx).expect("No files");
                    files.push(file);
                }
                set_files(files);
            }
            None => {}
        }
    };

    view! {
        <div>
            <p>"Please go to whatsapp, log in and then : Any discussion -> three vertical dots -> Plus -> export -> Export discussion (without media) browse the file here."</p>
        </div>

        <form on:submit=on_files_submit>
            <input type="file" multiple node_ref=input_element/>
            <input type="submit" value="Submit"/>
        </form>

        <div>
            <p>"Selected files: "</p>
            <For
                each=files
                key=|f| f.name().clone()
                children=|f| { view! { <p>"Value: " {f.name()}</p> } } />
        </div>

        <Suspense fallback = move || view! {<p>"Loading..."</p>}>
            <Whatsapp data={texts.get()}/>
        </Suspense>
    }
}
