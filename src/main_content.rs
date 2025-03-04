use floem::{
    event::EventListener,
    ext_event::create_signal_from_channel,
    keyboard::{Key, ModifiersState, NamedKey},
    peniko::Color,
    reactive::{create_effect, create_rw_signal, create_signal, SignalGet, SignalUpdate},
    style::Position,
    style_class,
    views::{
        dyn_stack,
        editor::{
            core::{editor::EditType, selection::Selection},
            text::Document,
        },
        h_stack, label, labeled_radio_button, scroll, text_editor, text_input, v_stack,
        ButtonClass, Decorators,
    },
    IntoView, View,
};
use jsonformat;
use std::rc::Rc;
use std::sync::mpsc;

use crate::auth_methods::{dropdown_view, AuthTypes};
use crate::collections::Request; //Collection,
use crate::request_methods::Method;
use crate::response_tabs::tab_navigation_view;
use std::io::prelude::*;
use std::thread;
use std::{fs::File, time::Duration, time::Instant};

pub const SIDEBAR_WIDTH: f64 = 140.0;
const TOPBAR_HEIGHT: f64 = 30.0;

style_class!(pub Button);
style_class!(pub Label);
style_class!(pub Frame);

fn send_request<T>(
    mthd: Method,
    headers: im::Vector<(String, String)>,
    bdy_txt: String,
    auth_type: AuthTypes,
    tkn_txt: String,
    url_txt: String,
    r_content: Rc<dyn Document>,
    r_headers: Rc<dyn Document>,
    r_stats: Rc<dyn Document>,
) {
    let (s_bod, r_bod) = mpsc::channel();
    let sig_bod = create_signal_from_channel(r_bod);

    let (s_head, r_head) = mpsc::channel();
    let sig_head = create_signal_from_channel(r_head);

    let (s_stat, r_stat) = mpsc::channel();
    let sig_stat = create_signal_from_channel(r_stat);

    // get the response and send it to the channel signal
    thread::spawn(move || {
        match_method_and_run::<String>(
            mthd, headers, bdy_txt, auth_type, tkn_txt, url_txt, s_bod, s_head, s_stat,
        );
    });

    // when the channel sends the signal, write it to the edit.
    create_effect(move |_| {
        if let Some(v2) = sig_bod.get() {
            r_content.edit_single(
                Selection::region(0, r_content.text().len()),
                &v2,
                EditType::InsertChars,
            );
        }
    });
    create_effect(move |_| {
        if let Some(v2) = sig_head.get() {
            r_headers.edit_single(
                Selection::region(0, r_headers.text().len()),
                &v2,
                EditType::InsertChars,
            );
        }
    });
    create_effect(move |_| {
        if let Some(v2) = sig_stat.get() {
            r_stats.edit_single(
                Selection::region(0, r_stats.text().len()),
                &v2,
                EditType::InsertChars,
            );
        }
    });
}

pub fn full_window_view() -> impl IntoView {
    //let env_list: im::Vector<&str> = vec!["hi", "ehllo", "world"].into();
    //let env_list = create_rw_signal(env_list);
    //

    let (method, set_method) = create_signal(Method::GET);

    let body_response = text_editor("").read_only().style(|s| s.height_full());
    let header_response = text_editor("").read_only().style(|s| s.height_full());
    let stats_response = text_editor("").read_only().style(|s| s.height_full());

    let body_field = text_editor("")
        .placeholder("request body. . .")
        .keyboard_navigable()
        .style(|s| {
            s.flex_grow(0.5)
                .width_pct(100.)
                .min_height(150)
                .font_family("".to_string())
                .font_size(8)
        });

    let top_bar = label(|| String::from("Top bar"))
        .style(|s| s.padding(10.0).width_full().height(TOPBAR_HEIGHT).margin(2));

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

    let status_text = create_rw_signal(" . . . ".to_string());

    let urltext = create_rw_signal("https://example.com".to_string());
    let request_name = create_rw_signal("New request".to_string());
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
    //
    let bf1 = body_field.doc().clone();
    let bf2 = body_field.doc().clone();

    let collection_side_bar = scroll({
        v_stack((
            "Save".class(ButtonClass).on_click_stop(move |_| {
                request_list.update(|list| {
                    let mut upd = false;
                    let new = Request {
                        name: request_name.get(),
                        url: urltext.get(),
                        method: method.get(),
                        body: bf1.text().to_string(),
                        auth: (authtype.get(), tokentext.get()),
                        headers: current_header_list.get(),
                    };
                    for ele in list.iter_mut() {
                        if ele.name == request_name.get() {
                            ele.url = urltext.get();
                            ele.method = method.get();
                            ele.body = bf1.text().to_string();
                            ele.auth = (authtype.get(), tokentext.get());
                            ele.headers = current_header_list.get();

                            upd = true;
                        }
                    }

                    if !upd {
                        list.push_back(new);
                    }
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
                            .on_click_stop({
                                let bf22 = bf2.clone();
                                move |_| {
                                    let itm = item.clone();
                                    request_list.update(|_| {
                                        request_name.set(itm.name.clone());
                                        urltext.set(itm.url.to_string());
                                        set_method.set(itm.method.clone());
                                        bf22.edit_single(
                                            Selection::region(0, bf22.text().len()),
                                            &itm.body.clone().to_string(),
                                            EditType::InsertChars,
                                        );
                                        current_header_list.update(|l| {
                                            *l = itm.headers.clone();
                                        });
                                        tokentext.set(itm.auth.1.to_string());
                                        authtype.set(itm.auth.0);
                                    })
                                }
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
            .border_color(Color::from_rgb8(205, 205, 205))
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

    let doc1 = body_response.doc().clone();
    let doc2 = body_response.doc().clone();
    let h_doc1 = header_response.doc().clone();
    let h_doc2 = header_response.doc().clone();
    let s_doc1 = stats_response.doc().clone();
    let s_doc2 = stats_response.doc().clone();

    let bf3 = body_field.doc();

    let send_button = "Send request".class(ButtonClass).on_click_stop(move |_| {
        let mthd = method.get();
        let bdy_txt = bf3.text().to_string();
        let auth_type = authtype.get();
        let tkn_txt = tokentext.get();
        let url_txt = urltext.get();
        let headers = current_header_list.get();

        send_request::<String>(
            mthd,
            headers,
            bdy_txt,
            auth_type,
            tkn_txt,
            url_txt,
            doc1.clone(),
            h_doc1.clone(),
            s_doc1.clone(),
        )
    });

    let bf4 = body_field.doc().clone();

    let url_bar = h_stack((text_input(urltext)
        .placeholder("Placeholder text")
        .keyboard_navigable()
        .on_key_up(
            Key::Named(NamedKey::Enter),
            move |m| m == ModifiersState::empty().into(),
            move |_| {
                let mthd = method.get();
                let bdy_txt = bf4.text().to_string();
                let auth_type = authtype.get();
                let tkn_txt = tokentext.get();
                let url_txt = urltext.get();
                let headers = current_header_list.get();
                send_request::<String>(
                    mthd,
                    headers,
                    bdy_txt,
                    auth_type,
                    tkn_txt,
                    url_txt,
                    doc2.clone(),
                    h_doc2.clone(),
                    s_doc2.clone(),
                )
            },
        )
        .style(|s| s.flex_grow(1.0).width_pct(100.)),))
    .style(|s| s.width_full());

    let name_box = h_stack((text_input(request_name)
        .placeholder("New Request")
        .keyboard_navigable()
        .style(|s| s.flex_grow(1.0).width_pct(100.)),));

    let header_add_bar = h_stack((
        text_input(headertext)
            .placeholder("header")
            .keyboard_navigable()
            .style(|s| s.flex_grow(1.0).width_pct(100.)),
        text_input(headervaluetext)
            .placeholder("value")
            .keyboard_navigable()
            .style(|s| s.flex_grow(1.0).width_pct(100.)),
        "➕".class(ButtonClass).on_click_stop(move |_| {
            current_header_list.update(|v| v.push_back((headertext.get(), headervaluetext.get())));
            headertext.set("".to_string());
            headervaluetext.set("".to_string());
        }),
    ));

    let header_bar = dyn_stack(
        move || current_header_list.get(),
        move |item| item.clone(),
        move |item| {
            let itm = item.clone();
            let header = itm.0.clone();
            let value = itm.1.clone();
            h_stack((
                label(move || header.clone() + ":")
                    .style(|s| s.flex_shrink(1.0).max_width(300.).padding_right(3)),
                label(move || value.clone()).style(|s| s.flex_grow(1.0).max_width(600.)),
                "➖"
                    .class(ButtonClass)
                    .on_click_stop(move |_| {
                        let _ = current_header_list.update(|v| {
                            let _ = v.pop_back();
                        });
                    })
                    .style(|s| s.flex_shrink(1.0)),
            ))
            .style(|s| s.width_pct(100.))
        },
    )
    .style(|s| s.width_pct(100.).flex_col());

    let auth_bar = h_stack((
        dropdown_view::<AuthTypes>(authtype).style(|s| s.width(150)),
        text_input(tokentext)
            .placeholder("bearer token . . .")
            .keyboard_navigable()
            .style(|s| s.flex_grow(1.0).width_pct(100.)),
    ));

    let main_block = scroll(
        v_stack((
            h_stack((
                v_stack((
                    v_stack((name_box, methods_list, url_bar, body_field, auth_bar))
                        .style(|s| s.width_full()),
                    v_stack((header_add_bar, header_bar)).style(|s| s.width_full()),
                ))
                .style(|s| s.width_full()),
                send_button,
            ))
            .style(|s| s.width_full()),
            status_text,
            //body_response,
            tab_navigation_view(
                body_response.doc().clone(),
                header_response.doc().clone(),
                stats_response.doc().clone(),
            ),
        ))
        .style(|s| s.width_full().height_full()),
    )
    .style(|s| {
        s.flex_col()
            .height_full()
            .flex_basis(1.0)
            .flex_grow(1.0)
            .border_top(1.0)
            .border_color(Color::from_rgb8(205, 205, 205))
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
    headers: im::Vector<(String, String)>,
    bodytext: String,
    auth: AuthTypes,
    tokentext: String,
    url: String,
    response_channel: mpsc::Sender<String>,
    headers_channel: mpsc::Sender<String>,
    status_channel: mpsc::Sender<String>,
) {
    let cb = reqwest::blocking::Client::builder();

    let c = cb.brotli(true).build().unwrap();
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

    headers.iter().for_each(|h| {
        let d = b.try_clone().expect("could not clone the builder");
        b = d.header(h.0.clone(), h.1.clone());
    });

    match auth {
        AuthTypes::Bearer => {
            b = b.bearer_auth(tokentext);
        }
        AuthTypes::None => {}
    }

    let start_time = Instant::now();

    let _ = response_channel.send("awaiting response ... ".to_string());

    let sx1 = status_channel.clone();

    let (tx1, rx1) = mpsc::channel();
    thread::spawn(move || loop {
        if let Ok(_) = rx1.try_recv() {
            break;
        }

        let message = format!(
            "sending request... Elapsed time: {} ms",
            start_time.elapsed().as_millis()
        );

        let _ = sx1.send(message);
        thread::sleep(Duration::from_millis(500));
    });

    let rsp = b
        .timeout(Duration::from_secs(3600))
        .send()
        .expect("could not send");

    let hd = format!("{:?}", rsp.headers());
    let _ = headers_channel.send(hd.clone());

    let txt = rsp.text().expect("could not get text");

    let message = format!("Duration: {} ms", start_time.elapsed().as_millis());

    let _ = status_channel.send(message.clone());

    let _ = tx1.send(true);
    let start_time = Instant::now();
    let _ = response_channel.send(format!("formatting response... "));

    let rs = jsonformat::format(&txt, jsonformat::Indentation::FourSpace);

    let message = message
        + &format!(
            " \n\rFormatting took: {} ms",
            start_time.elapsed().as_millis()
        );
    let _ = status_channel.send(message);

    let _ = response_channel.send(rs);
}
