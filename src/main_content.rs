use crossbeam_channel::bounded;
use floem::{
    reactive::RwSignal,
    keyboard::{NamedKey,Key, ModifiersState, Modifiers},
    event::EventListener,
    ext_event::create_signal_from_channel,
    peniko::Color,
    reactive::{create_effect, create_rw_signal, create_signal},
    style::Position,
    views::{
        dyn_stack, h_stack, label, labeled_radio_button, scroll, text_input, v_stack, virtual_stack,
        ButtonClass, Decorators, VirtualDirection, VirtualItemSize,
    },
    IntoView, View,
};

use std::fs::File;
use std::io::prelude::*;
use std::thread;
use crate::request_methods::{dropdown_view,AuthTypes};
use std::fmt::Display;

use serde::{Deserialize, Serialize};

const SIDEBAR_WIDTH: f64 = 140.0;
const TOPBAR_HEIGHT: f64 = 30.0;
const SIDEBAR_ITEM_HEIGHT: f64 = 21.0;

#[derive(PartialEq, Eq, Clone, Debug, Hash, Serialize, Deserialize)]
enum Method {
    GET,
    POST,
    HEAD,
    DELETE,
    PUT,
    PATCH
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Method::GET => write!(f, "GET"),
            Method::POST => write!(f, "POST"),
            Method::HEAD => write!(f, "HEAD"),
            Method::DELETE => write!(f, "DELETE"),
            Method::PUT => write!(f, "PUT"),
            Method::PATCH => write!(f, "PATCH"),
        }
    }
}


#[derive(Clone, Debug, Hash, PartialEq, Eq,Serialize, Deserialize )]
struct Request {
    headers: im::Vector<(String, String)>,
    id : usize,
    url: String,
    method: Method,
    body: String,
    auth: (Method, String)
}

fn send_request<T>(mthd: Method, ht: String,hvt: String,  bdy_txt: String, auth_type: AuthTypes, tkn_txt: String, url_txt: String, content: RwSignal<String>) 
    {
                let (tx, rx) = bounded(1);
                thread::spawn(move || {
                    match_method_and_run::<String>(
                        mthd,ht, hvt,
                        bdy_txt,
                        auth_type,
                        tkn_txt,
                        url_txt,
                        tx);
                });
                let sig = create_signal_from_channel(rx.clone());
                create_effect(move |_| {
                    if let Some(v2) = sig.get() {
                        content.set(v2);
                    }
                })
            }


pub fn full_window_view() -> impl IntoView {

    let current_header_list = im::Vector::<(String,String)>::new();
    let current_header_list = create_rw_signal(current_header_list);

    let request_list = im::Vector::<Request>::new();
    let request_list= create_rw_signal(request_list);

    let env_list: im::Vector<&str> = vec!["hi", "ehllo", "world"].into();
    let env_list = create_rw_signal(env_list);

    let (method, set_method) = create_signal(Method::GET);

    let content = create_rw_signal("".to_string());

    let urltext = create_rw_signal("https://example.com".to_string());
    let bodytext = create_rw_signal("{}".to_string());
    let tokentext = create_rw_signal("".to_string());
    let headervaluetext = create_rw_signal("".to_string());
    let headertext = create_rw_signal("".to_string());
    let authtype = create_rw_signal(AuthTypes::None);
    let top_bar = label(|| String::from("Top bar"))
        .style(|s| s.padding(10.0).width_full().height(TOPBAR_HEIGHT));

    let right_sidebar = scroll({
        virtual_stack(
            VirtualDirection::Vertical,
            VirtualItemSize::Fixed(Box::new(|| SIDEBAR_ITEM_HEIGHT)),
            move || env_list.get(),
            move |item| *item,
            move |item| {
                label(move || item.to_string()).style(move |s| {
                    s.padding(10.0)
                        .padding_top(3.0)
                        .padding_bottom(3.0)
                        .width(SIDEBAR_WIDTH)
                        .height(SIDEBAR_ITEM_HEIGHT)
                        .items_start()
                        .border_bottom(1.0)
                        .border_color(Color::rgb8(205, 205, 205))
                })
            },
        )
            .style(|s| s.flex_col().width(SIDEBAR_WIDTH + 1.0))
    })
    .style(|s| {
        s.width(SIDEBAR_WIDTH)
            .border_left(1.0)
            .border_top(1.0)
            .border_color(Color::rgb8(205, 205, 205))
    });

    let side_bar = scroll({
        v_stack(("Save current URL".class(ButtonClass).on_click_stop(move |_| {
            current_header_list.update(|v| v.push_back((headertext.get(), headervaluetext.get())));
            request_list.update(|list| list.push_back(Request 
                    { id: list.len(),
                    url : urltext.get(),
                    method: method.get(),
                    body: bodytext.get(),
                    auth: (method.get(), tokentext.get()),
                    headers : current_header_list.get()
                    }));
            let mut file = File::create("atticus.json").unwrap();
            file.write(serde_json::to_string(&request_list.get()).unwrap().as_bytes()).unwrap();
        }),
        dyn_stack(
            move || request_list.get(),
            move |item| item.clone(),
            move |item| {
                h_stack ((
                        item.url.to_string().class(ButtonClass).on_click_stop(move |_| {
                            request_list.update(|_| {
                                urltext.set(item.url.to_string());
                                set_method.set(item.method.clone());
                                bodytext.set(item.body.to_string());
                                headertext.set(item.headers.head().unwrap().0.to_string());
                                headervaluetext.set(item.headers.head().unwrap().1.to_string());
                                tokentext.set(item.auth.1.to_string());
                            })
                        }).style(|s| s.width(50)),
                        "🗑️".class(ButtonClass).on_click_stop(move |_| {
                            request_list.update(|list| {if list.len() > 0 {}})
                        }),
                ))
            },
        )
            .style(|s| s.flex_col().width(SIDEBAR_WIDTH - 1.0))))
    })
    .style(|s| {
        s.width(SIDEBAR_WIDTH)
            .border_right(1.0)
            .border_top(1.0)
            .border_color(Color::rgb8(205, 205, 205))
    });

    let methods_list= h_stack((
            labeled_radio_button(Method::GET, move || method.get(), || Method::GET).on_update(
                move |value| {
                    set_method.set(value);
                },
            ),
            labeled_radio_button(Method::POST, move || method.get(), || Method::POST)
            .on_update(move |value| {
                set_method.set(value);
            }),
            labeled_radio_button(Method::HEAD, move || method.get(), || Method::HEAD)
            .on_update(move |value| {
                set_method.set(value);
            }),
            labeled_radio_button(Method::DELETE, move || method.get(), || Method::DELETE)
            .on_update(move |value| {
                set_method.set(value);
            }),
            labeled_radio_button(Method::PUT, move || method.get(), || Method::PUT)
            .on_update(move |value| {
                set_method.set(value);
            }),
            labeled_radio_button(Method::PATCH, move || method.get(), || Method::PATCH)
                .on_update(move |value| {
                    set_method.set(value);
                }),
            ));

    let url_bar = h_stack((
        "Send request"
            .class(ButtonClass)
            .on_click_stop(move |_|  {
                let mthd = method.get();
                let bdy_txt = bodytext.get();
                let auth_type = authtype.get();
                let tkn_txt = tokentext.get();
                let url_txt = urltext.get() ;
                let hvt = headervaluetext.get() ;
                let ht = headertext.get() ;
                send_request::<String>(mthd, ht, hvt, bdy_txt, auth_type, tkn_txt, url_txt, content) } ),
        text_input(urltext)
            .placeholder("Placeholder text")
            .keyboard_navigatable()
            .on_key_up(Key::Named(NamedKey::Enter), Modifiers::from(ModifiersState::empty()) , move |_| {
                let mthd = method.get();
                let bdy_txt = bodytext.get();
                let auth_type = authtype.get();
                let tkn_txt = tokentext.get();
                let url_txt = urltext.get() ;
                let hvt = headervaluetext.get() ;
                let ht = headertext.get() ;
                send_request::<String>(mthd, ht, hvt, bdy_txt, auth_type, tkn_txt, url_txt, content) })
            .style(|s| s.flex_grow(1.0).width_pct(100.)),

            )).style(|s| s.width_full());


    let header_bar = h_stack ((
                        label(|| "headers"),
                        text_input(headertext)
                            .placeholder("header") 
                            .keyboard_navigatable()
                            .style(|s| s.flex_grow(1.0).width_pct(100.)),
                        text_input(headervaluetext)
                            .placeholder("value") 
                            .keyboard_navigatable()
                            .style(|s| s.flex_grow(1.0).width_pct(100.))
                    ));

    let auth_bar = h_stack ((
                            dropdown_view::<AuthTypes>(authtype).style(|s| s.width(150)),
                            text_input(tokentext)
                            .placeholder("bearer token . . .") 
                            .keyboard_navigatable()
                            .style(|s| s.flex_grow(1.0).width_pct(100.))
                    ));

    let body_field = text_input(bodytext)
                    .placeholder("request body. . .")
                    .keyboard_navigatable()
                    .style(|s| s.flex_grow(1.0).width_pct(100.));

    let main_block =
        scroll(v_stack(( 
                    methods_list,
                    url_bar,
                    body_field,
                    auth_bar,
                    header_bar,
/* Response box */ label(move || format!("{}", content.get())), 
        )))
        .style(|s| { s.flex_col() .flex_basis(0) .flex_grow(1.0) .border_top(1.0) .border_color(Color::rgb8(205, 205, 205)) });

    let content = h_stack((side_bar, main_block, right_sidebar)).style(|s| {
        s.position(Position::Absolute)
            .inset_top(TOPBAR_HEIGHT)
            .inset_bottom(0.0)
            .width_full()
    });

    let view = v_stack((top_bar, content)).style(|s| s.width_full().height_full());

    let id = view.id();
    view.on_event_stop(EventListener::KeyUp, move |e| {
        if let floem::event::Event::KeyUp(e) = e {
            if e.key.logical_key == floem::keyboard::Key::Named(floem::keyboard::NamedKey::F11) {
                id.inspect();
            }
        }
    })
}

fn match_method_and_run<T>(mthd: Method,ht: String, hvt: String,  bodytext: String, auth: AuthTypes, tokentext: String, url: String, tx: crossbeam_channel::Sender<String>) {
    let  c = reqwest::blocking::Client::new();
    let mut b;
    match mthd {
        Method::GET => { b = c.get(url); }
        Method::POST => { b = c .post(url); b = b.body(bodytext); }
        Method::HEAD => { b = c.head(url); }
        Method::DELETE => { b = c.delete(url); }
        Method::PUT => { b = c.put(url); b = b.body(bodytext); }
        Method::PATCH => { b = c.put(url); b = b.body(bodytext); }
    }

   
    b = b.header(ht, hvt);

    //add the token 
    match auth {
        AuthTypes::Bearer  => { b = b.bearer_auth(tokentext); }
        AuthTypes::None => {}
    }

    let rsp = b.send().unwrap().text().unwrap();
    let rsp = serde_json::from_str(&rsp).and_then(|c: serde_json::Value| {
        serde_json::to_string_pretty(&c)
    }).unwrap_or(rsp);
    let _ = tx.send(rsp);
}
