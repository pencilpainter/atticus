use crossbeam_channel::bounded;
use std::rc::Rc;
use floem::{
    ext_event::create_signal_from_channel,
    event::EventListener,
        style::Position,
    keyboard::{Key, Modifiers, ModifiersState, NamedKey},
    peniko::Color,
    reactive::{create_effect, create_rw_signal, create_signal, SignalGet, SignalUpdate},
    views::{
        dyn_stack, h_stack, label, labeled_radio_button, scroll, text_input, text_editor,
        v_stack, ButtonClass, Decorators, editor::text::Document,
        editor::core::{
                selection::Selection,editor::EditType
            }, 
        },
    IntoView, View,
};
use jsonformat;

use crate::auth_methods::{dropdown_view, AuthTypes};
use crate::collections::Request; //Collection,
use crate::request_methods::Method;
use std::io::prelude::*;
use std::thread;
use std::{fs::File, time::Duration};

pub const SIDEBAR_WIDTH: f64 = 140.0;
const TOPBAR_HEIGHT: f64 = 30.0;

fn send_request<T>(
    mthd: Method,
    ht: String,
    hvt: String,
    bdy_txt: String,
    auth_type: AuthTypes,
    tkn_txt: String,
    url_txt: String,
    content: Rc<dyn Document>
) {
    let (tx, rx) = bounded(1);
    let sig = create_signal_from_channel(rx.clone());

    content.edit_single(Selection::caret(0), "Waiting for response", EditType::InsertChars);

    // get the response and put it in the box
    thread::spawn(move || {
        match_method_and_run::<String>(mthd, ht, hvt, bdy_txt, auth_type, tkn_txt, url_txt, tx);
    });

    create_effect(move |_| {
        if let Some(v2) = sig.get() {
                content.edit_single(Selection::region(0, content.text().len()), &v2, EditType::InsertChars);
        }
    });
}

pub fn full_window_view() -> impl IntoView {
    //let env_list: im::Vector<&str> = vec!["hi", "ehllo", "world"].into();
    //let env_list = create_rw_signal(env_list);

    let (method, set_method) = create_signal(Method::GET);

    let body_label = text_editor("")
        .style(|s| s.flex_grow(1.0).width_pct(100.));

    let top_bar = label(|| String::from("Top bar"))
        .style(|s| s.padding(10.0).width_full().height(TOPBAR_HEIGHT));

    let current_header_list = im::Vector::<(String, String)>::new();
    let current_header_list = create_rw_signal(current_header_list);

    let mut data = vec![];
    if let Ok(mut f) = File::open("atticus.json") {
        let _ = f.read_to_end(&mut data);
    } else {
        data = Vec::<u8>::new();
    };

    let saved_req = serde_json::from_slice::<im::Vector<Request>>(&data)
        .unwrap_or(::im::Vector::<Request>::new());

    let request_list = saved_req;
    let request_list = create_rw_signal(request_list);

    let urltext = create_rw_signal("https://example.com".to_string());
    let request_name = create_rw_signal("New request".to_string());
    let bodytext = create_rw_signal("{}".to_string());
    let tokentext = create_rw_signal("".to_string());
    let headervaluetext = create_rw_signal("".to_string());
    let headertext = create_rw_signal("".to_string());
    let authtype = create_rw_signal(AuthTypes::None);

    let mut data = vec![];
    if let Ok(mut f) = File::open("collection.json") {
        let _ = f.read_to_end(&mut data);
    } else {
        //data = Vec::<u8>::new();
    };

    //let saved_coll = serde_json::from_slice::<im::Vector<Collection>>(&data)
    //.unwrap_or(::im::Vector::<Collection>::new());

    //let collection_list = create_rw_signal(saved_coll);

    let collection_side_bar = scroll({
        v_stack((
            "Save".class(ButtonClass).on_click_stop(move |_| {
                current_header_list
                    .update(|v| v.push_back((headertext.get(), headervaluetext.get())));
                request_list.update(|list| {
                    list.push_back(Request {
                        name: request_name.get(),
                        url: urltext.get(),
                        method: method.get(),
                        body: bodytext.get(),
                        auth: (authtype.get(), tokentext.get()),
                        headers: current_header_list.get(),
                    })
                });
                let mut file = File::create("atticus.json").unwrap();
                file.write(
                    serde_json::to_string(&request_list.get())
                        .unwrap()
                        .as_bytes(),
                )
                .unwrap();
            }),
            dyn_stack(
                move || request_list.get(),
                move |item| item.clone(),
                move |item| {
                    let itm = item.clone();
                    h_stack((
                        "🗑️ ".class(ButtonClass).on_click_stop(move |_| {
                            request_list.update(|list| {
                                list.remove(
                                    list.iter()
                                        .enumerate()
                                        .find(|f| &f.1.name == &(itm.name).to_string())
                                        .unwrap()
                                        .0,
                                );
                                let mut file = File::create("atticus.json").unwrap();
                                file.write(serde_json::to_string(list).unwrap().as_bytes())
                                    .unwrap();
                            })
                        }),
                        item.name
                            .clone()
                            .to_string()
                            .class(ButtonClass)
                            .on_click_stop(move |_| {
                                let itm = item.clone();
                                request_list.update(|_| {
                                    request_name.set(itm.name.clone());
                                    urltext.set(itm.url.to_string());
                                    set_method.set(itm.method.clone());
                                    bodytext.set(itm.body.clone().to_string());
                                    headertext.set(itm.headers.head().unwrap().0.to_string());
                                    headervaluetext.set(itm.headers.head().unwrap().1.to_string());
                                    tokentext.set(itm.auth.1.to_string());
                                    authtype.set(itm.auth.0);
                                })
                            })
                            .style(|s| s.width(130)),
                    ))
                },
            )
            .style(|s| s.flex_col().width(SIDEBAR_WIDTH - 1.0)),
        ))
    })
    .style(|s| {
        s.width(SIDEBAR_WIDTH)
            .border_right(1.0)
            .border_top(1.0)
            .border_color(Color::rgb8(205, 205, 205))
    });

    let methods_list = h_stack((
        labeled_radio_button(Method::GET, move || method.get(), || Method::GET).on_update(
            move |value| {
                set_method.set(value);
            },
        ),
        labeled_radio_button(Method::POST, move || method.get(), || Method::POST).on_update(
            move |value| {
                set_method.set(value);
            },
        ),
        labeled_radio_button(Method::HEAD, move || method.get(), || Method::HEAD).on_update(
            move |value| {
                set_method.set(value);
            },
        ),
        labeled_radio_button(Method::DELETE, move || method.get(), || Method::DELETE).on_update(
            move |value| {
                set_method.set(value);
            },
        ),
        labeled_radio_button(Method::PUT, move || method.get(), || Method::PUT).on_update(
            move |value| {
                set_method.set(value);
            },
        ),
        labeled_radio_button(Method::PATCH, move || method.get(), || Method::PATCH).on_update(
            move |value| {
                set_method.set(value);
            },
        ),
    ));

    let doc = body_label.doc().clone();
    let doc2 = body_label.doc().clone();

    let url_bar = h_stack((
        "Send request".class(ButtonClass).on_click_stop(move |_| {
            let mthd = method.get();
            let bdy_txt = bodytext.get();
            let auth_type = authtype.get();
            let tkn_txt = tokentext.get();
            let url_txt = urltext.get();
            let hvt = headervaluetext.get();
            let ht = headertext.get();
            send_request::<String>(mthd, ht, hvt, bdy_txt, auth_type, tkn_txt, url_txt, doc.clone())
        }),
        text_input(urltext)
            .placeholder("Placeholder text")
            .keyboard_navigatable()
            .on_key_up(
                Key::Named(NamedKey::Enter),
                Modifiers::from(ModifiersState::empty()),
                move |_| {
                    let mthd = method.get();
                    let bdy_txt = bodytext.get();
                    let auth_type = authtype.get();
                    let tkn_txt = tokentext.get();
                    let url_txt = urltext.get();
                    let hvt = headervaluetext.get();
                    let ht = headertext.get();
                    send_request::<String>(
                        mthd, ht, hvt, bdy_txt, auth_type, tkn_txt, url_txt, doc2.clone() ,
                    )
                },
            )
            .style(|s| s.flex_grow(1.0).width_pct(100.)),
    ))
    .style(|s| s.width_full());

    let name_box = h_stack((
                text_input(request_name)
                .placeholder("New Request") 
                .keyboard_navigatable()
                .style(|s| s.flex_grow(1.0).width_pct(100.)),
    ));

    let header_bar = h_stack((
        label(|| "headers"),
        text_input(headertext)
            .placeholder("header")
            .keyboard_navigatable()
            .style(|s| s.flex_grow(1.0).width_pct(100.)),
        text_input(headervaluetext)
            .placeholder("value")
            .keyboard_navigatable()
            .style(|s| s.flex_grow(1.0).width_pct(100.)),
    ));

    let auth_bar = h_stack((
        dropdown_view::<AuthTypes>(authtype).style(|s| s.width(150)),
        text_input(tokentext)
            .placeholder("bearer token . . .")
            .keyboard_navigatable()
            .style(|s| s.flex_grow(1.0).width_pct(100.)),
    ));

    let body_field = h_stack((text_input(bodytext)
        .placeholder("request body. . .")
        .keyboard_navigatable()
        .style(|s| s.flex_grow(1.0).width_pct(100.)),
    ));



    let main_block = scroll(v_stack((
        name_box,
        methods_list,
        url_bar,
        body_field,
        auth_bar,
        header_bar,
        body_label 
    ))
    .style(|s| { s.width_full().height_full() }))
    .style(|s| { s.flex_col() .flex_basis(0) .flex_grow(1.0) .border_top(1.0) .border_color(Color::rgb8(205, 205, 205))
    });

    let content_pane = h_stack((collection_side_bar, main_block)).style(|s| {
        s.position(Position::Absolute)
            .inset_top(TOPBAR_HEIGHT)
            .inset_bottom(0.0)
            .width_full()
    });

    let view = v_stack((top_bar, content_pane)).style(|s| s.width_full().height_full());

    let id = view.id();
    view.on_event_stop(EventListener::KeyUp, move |e| {
        if let floem::event::Event::KeyUp(e) = e {
            if e.key.logical_key == floem::keyboard::Key::Named(floem::keyboard::NamedKey::F11) {
                id.inspect();
            }
        }
    })
}

fn match_method_and_run<T>(
    mthd: Method,
    ht: String,
    hvt: String,
    bodytext: String,
    auth: AuthTypes,
    tokentext: String,
    url: String,
    tx: crossbeam_channel::Sender<String>,
) {
    let c = reqwest::blocking::Client::new();
    let mut b;
    match mthd {
        Method::GET => {
            b = c.get(url);
        }
        Method::POST => {
            b = c.post(url);
            b = b.body(bodytext);
        }
        Method::HEAD => {
            b = c.head(url);
        }
        Method::DELETE => {
            b = c.delete(url);
        }
        Method::PUT => {
            b = c.put(url);
            b = b.body(bodytext);
        }
        Method::PATCH => {
            b = c.put(url);
            b = b.body(bodytext);
        }
    }

    b = b.header(ht, hvt);

    match auth {
        AuthTypes::Bearer => {
            b = b.bearer_auth(tokentext);
        }
        AuthTypes::None => {}
    }

    let _ = tx.send("sending request ... ".to_string());

    let rsp = b
        .timeout(Duration::from_secs(3600))
        .send()
        .expect("cold not end")
        .text()
        .expect("could not get text");

    let _ = tx.send(format!("formatting response... "));
    let rs = jsonformat::format(&rsp, jsonformat::Indentation::FourSpace);
    
    let _ = tx.send(rs);

 //   let mut skip = 0;
 //   let mut msg = "".to_string();
 //   let mut indnt = 0;

 //   let mut prtty: (String, usize);
 //   loop {
 //       prtty = pretty_print(&rsp, 10000, skip * 10000, indnt);
 //       skip += 1;
 //       msg += &prtty.0;
 //       indnt = prtty.1;

 //       if prtty.0.len() == 0 {
 //           break;
 //       }
 //       let _ = tx.send(msg.clone());
 //       thread::sleep(Duration::from_millis(500));
 //   }
}

