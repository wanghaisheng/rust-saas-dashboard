/*
 * @Author: plucky
 * @Date: 2022-10-15 00:32:59
 * @LastEditTime: 2022-10-25 15:23:25
 * @Description: 
 */

#![allow(non_snake_case)]
use dioxus::prelude::*;
use tracing::info;

#[derive(Debug, Default)]
struct User {
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirm: String,
}

pub fn view(cx: Scope)->Element{
    
    cx.render(rsx!{
        div {
            h3 {
                class: "text-3xl font-semibold text-gray-700",
                "Forms"
            }
            
            // Modal Form
            Model_form{}

            // Form Elements
            Forms{}
            
        }
    })
}


// Model Form
fn Model_form(cx: Scope)->Element{
    
    cx.render(rsx!{
        div {
            class: "mt-4",
            h4 {
                class: "text-gray-600",
                "Model Form"
            }
            div {
                class: "mt-4",
                div {
                    class: "w-full max-w-sm overflow-hidden bg-white border rounded-md shadow-md",
                    // form
                    form {
                        div {
                            class: "flex items-center justify-between px-5 py-3 text-gray-700 border-b",
                            h3 {
                                class: "text-sm",
                                "Add Category"
                            }
                            button {
                                r#type: "button",
                                onclick: |_| {
                                    
                                },
                                icons::icon_1 {}
                            }
                        }
                        div {
                            class: "px-5 py-6 text-gray-700 bg-gray-200 border-b",
                            label {
                                class: "text-xs",
                                "Name"
                            }
                            div {
                                class: "relative mt-2 rounded-md shadow-sm",
                                span {
                                    class: "absolute inset-y-0 left-0 flex items-center pl-3 text-gray-600",
                                    icons::icon_2 {}
                                }
                                input {
                                    class: "w-full px-12 py-2 border-transparent rounded-md appearance-none focus:border-indigo-600 focus:ring focus:ring-opacity-40 focus:ring-indigo-500",
                                    r#type: "text",
                                }
                            }
                        }
                        div {
                            class: "flex items-center justify-between px-5 py-3",
                            button {
                                r#type: "button",
                                onclick: |_| {
                                    info!("click");
                                    
                                },
                                class: "px-3 py-1 text-sm text-gray-700 bg-gray-200 rounded-md hover:bg-gray-300 focus:outline-none",
                                "Cancel"
                            }
                            button {
                                class: "px-3 py-1 text-sm text-white bg-indigo-600 rounded-md hover:bg-indigo-500 focus:outline-none",
                                "Save"
                            }
                        }
                    }
                }
            }
        }
    })
}

// Forms
fn Forms(cx: Scope)->Element{
    let user = use_ref(&cx, || User::default());
    let ur = user.read();
    
    cx.render(rsx!{
        div {
            class: "mt-8",
            h4 {
                class: "text-gray-600",
                "Forms"
            }
            div {
                class: "mt-4",
                div {
                    class: "p-6 bg-white rounded-md shadow-md",
                    h2 {
                        class: "text-lg font-semibold text-gray-700 capitalize",
                        "Account settings"
                    }

                    form{
                        //action="" methods="post"
                        prevent_default: "onsubmit",
                        onsubmit: move |e|{
                            info!("onsubmit: {:?}", e);
                            info!("onsubmit: {:?}", user.read());
                        },
                        
                        div {
                            class: "grid grid-cols-1 gap-6 mt-4 sm:grid-cols-2",
                            div {
                                label {
                                    class: "text-gray-700",
                                    r#for: "username","Username"
                                }
                                input {
                                    id: "username",
                                    class: "w-full mt-2 border-gray-200 rounded-md focus:border-indigo-600 focus:ring focus:ring-opacity-40 focus:ring-indigo-500",
                                    r#type: "text",
                                    // "v-model: "user.username",
                                    // 双向绑定
                                    value: "{ur.username}",
                                    oninput: {|e|{
                                        // info!("oninput{:?}", e);
                                        user.write().username=e.value.to_string();
                                    }},

                                }
                            }
                            div {
                                label {
                                    class: "text-gray-700",
                                    r#for: "email","Email Address"
                                }
                                input {
                                    id: "email",
                                    class: "w-full mt-2 border-gray-200 rounded-md focus:border-indigo-600 focus:ring focus:ring-opacity-40 focus:ring-indigo-500",
                                    r#type: "email",
                                    // "v-model": "user.email",
                                    value: "{ur.email}",
                                    oninput: {|e|{
                                        user.write().email=e.value.to_string();
                                    }},
                                }
                            }
                            div {
                                label {
                                    class: "text-gray-700",
                                    r#for: "password","Password"
                                }
                                input {
                                    id: "password",
                                    class: "w-full mt-2 border-gray-200 rounded-md focus:border-indigo-600 focus:ring focus:ring-opacity-40 focus:ring-indigo-500",
                                    r#type: "password",
                                    // "v-model": "user.password",
                                    value: "{ur.password}",
                                    oninput: {|e|{
                                        user.write().password=e.value.to_string();
                                    }},
                                }
                            }
                            div {
                                label {
                                    class: "text-gray-700",
                                    r#for: "pwConfirm","Password Confirmation"
                                }
                                input {
                                    id: "pwConfirm",
                                    class: "w-full mt-2 border-gray-200 rounded-md focus:border-indigo-600 focus:ring focus:ring-opacity-40 focus:ring-indigo-500",
                                    r#type: "password",
                                    // "v-model": "user.confirm",
                                    value: "{ur.confirm}",
                                    oninput: {|e|{
                                        user.write().confirm=e.value.to_string();
                                    }},
                                }
                            }
                        }
                        div {
                            class: "flex justify-end mt-4",
                            button {
                                r#type: "submit",
                                class: "px-4 py-2 text-gray-200 bg-gray-800 rounded-md hover:bg-gray-700 focus:outline-none focus:bg-gray-700",
                                "Save"
                            }
                        }
                    }
                }
            }
        }
    })
}

mod icons{
    use dioxus::prelude::*;
    use dioxus_html_macro::html;

    pub fn icon_1(cx: Scope)->Element{
        cx.render(html!{
            <svg
                  class="w-4 h-4"
                  fill="none"
                  view_box="0 0 24 24"
                  stroke="currentColor"
                >
                  <path
                    stroke_linecap="round"
                    stroke_linejoin="round"
                    stroke_width="2"
                    d="M6 18L18 6M6 6l12 12"
                  />
            </svg>
    })
    }
    
    pub fn icon_2(cx: Scope)->Element{
        cx.render(html!{
            <svg
                    class="w-6 h-6"
                    fill="none"
                    view_box="0 0 24 24"
                    stroke="currentColor"
                  >
                    <path
                      stroke_linecap="round"
                      stroke_linejoin="round"
                      stroke_width="2"
                      d="M11 4a2 2 0 114 0v1a1 1 0 001 1h3a1 1 0 011 1v3a1 1 0 01-1 1h-1a2 2 0 100 4h1a1 1 0 011 1v3a1 1 0 01-1 1h-3a1 1 0 01-1-1v-1a2 2 0 10-4 0v1a1 1 0 01-1 1H7a1 1 0 01-1-1v-3a1 1 0 00-1-1H4a2 2 0 110-4h1a1 1 0 001-1V7a1 1 0 011-1h3a1 1 0 001-1V4z"
                    />
                  </svg>
        })
    }

    

}