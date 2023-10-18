use std::collections::HashSet;

use crate::parsers::facebook::{get_counts, FacebookMessage, FacebookMessenger};

use plotly::Bar;
use plotly::Histogram;

use plotly::Plot;

use leptos::html::Input;
use leptos::{
    component, create_action, create_node_ref, create_resource, create_signal, logging, view, For,
    IntoView, SignalGet, Suspense,
};
use wasm_bindgen_futures::JsFuture;
use web_sys::{File, SubmitEvent};

fn get_plots(messages: Vec<FacebookMessage>, participants: HashSet<String>) -> (Plot, Plot) {
    let (msg_count, reaction_count, date_count) = get_counts(&messages, &participants);

    let mut msg_and_reaction_plot = Plot::new();

    for name in msg_count.keys() {
        msg_and_reaction_plot.add_trace(
            Bar::new(
                ["Messages", "Reaction"].to_vec(),
                [msg_count[&name.clone()], reaction_count[&name.clone()]].to_vec(),
            )
            .name(name),
        )
    }

    let mut date_plot = Plot::new();
    for (name, dates) in date_count.iter() {
        date_plot.add_trace(
            Histogram::new(dates.to_vec())
                .x_axis("Hour")
                .y_axis("Count")
                .name(name),
        );
    }

    return (date_plot, msg_and_reaction_plot);
}

#[component]
fn MessengerData(data: Option<Vec<String>>) -> impl IntoView {
    let mut messages: Vec<FacebookMessage> = vec![];
    let mut participants: HashSet<String> = HashSet::new();
    match data {
        Some(facebook_data) => {
            if facebook_data.len() > 0 {
                for d in facebook_data.iter() {
                    let fb: FacebookMessenger =
                        serde_json::from_str(d).expect("Unable to create facebook object");
                    messages.extend(fb.messages);

                    for p in fb.participants {
                        participants.insert(p.name);
                    }
                }

                let date_plotted = create_action(|input: &Plot| {
                    let input = input.to_owned();
                    async move { plotly::bindings::new_plot("DatePlot", &input).await }
                });
                let msg_plotted = create_action(|input: &Plot| {
                    let input = input.to_owned();
                    async move { plotly::bindings::new_plot("MsgPlot", &input).await }
                });
                let (msg_plot, date_plot) = get_plots(messages, participants);

                date_plotted.dispatch(date_plot);
                msg_plotted.dispatch(msg_plot);
            }
        }

        _ => {}
    }

    view! {
        <div>
            <div id="DatePlot"></div>
            <div id="MsgPlot"></div>
        </div>
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
    logging::log!("Got {} files: ", files_texts.len());

    files_texts
}
#[component]
pub fn FacebookMultiFileSelectorComponent() -> impl IntoView {
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
            <p>"Please go to Facebook, log in and then : "</p>
            <p>
            "
                Your Profile -> Parameters (& confidentiality) -> Parameters -> Your infos -> Download your data -> Agree to download (will be sent by email)
                Select only messages, format JSON, low quality -> You will then receive an email with your data that you will be able to Un-Zip and browse here.
            "
            </p>
            <p>"Please select the message_x.json from a message/inbox/person_name in your unzipped data (message_1.json / message_2.json etc...)"</p>
        </div>

        <form on:submit=on_files_submit>
            <input type="file" multiple node_ref=input_element/>
            <input type="submit" value="Submit"/>
        </form>

        <p>"Selected files: " {move || files.get().iter().map(|f| f.name()).collect::<Vec<String>>()}</p>

        <div>
            <For
                each=files
                key=|f| f.name().clone()
                view=|f| { view! { <p>"Value: " {f.name()}</p> } } />
        </div>

        <Suspense fallback = move || view! {<p>"Loading..."</p>}>
            <MessengerData data={texts.get()}/>
        </Suspense>
    }
}
