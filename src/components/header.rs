use dioxus::{prelude::{*}};
use tracing::info;

use super::sidebar::IS_SIDEBAR_OPEN;

pub fn view(cx: Scope)->Element{
    //const dropdownOpen = ref(false);
    //const { isOpen } = useSidebar();
    let set_sidebar_open = use_set(&cx,IS_SIDEBAR_OPEN);
    let dropdown_open = use_state(&cx, || true);
   
    let dropdown_class = match dropdown_open.get() {
        true => "transition duration-150 ease-in transform scale-95 opacity-0",
        false => "transition duration-150 ease-out transform scale-100 opacity-100",
    }; 
    
    cx.render(rsx!{
        header {
            class: "flex items-center justify-between px-6 py-4 bg-white border-b-4 border-indigo-600",
            div {
                class: "flex items-center",
                button{
                    class: "text-gray-500 focus:outline-none lg:hidden",//
                    // @click=\"isOpen = true\"\n 
                    onclick:  move |_| {
                        set_sidebar_open(true);
                    },
                    icons::icon_1{}
                }
                
                div {
                    class: "relative mx-4 lg:mx-0",
                    span {
                        class: "absolute inset-y-0 left-0 flex items-center pl-3 ",
                        icons::icon_2{}
                    }
                    input {
                        class: "w-32 pl-10 pr-4 text-indigo-600 border-gray-200 rounded-md sm:w-64 focus:border-indigo-600 focus:ring focus:ring-opacity-40 focus:ring-indigo-500",
                        placeholder: "Search",
                        r#type: "text",
                        value: "",
                    }
                    
                }
            }
            
            div {
                class: "flex items-center",
                button {
                    class: "flex mx-4 text-gray-600 focus:outline-none",
                    icons::icon_3{}
                }

                div {
                    class: "relative",
                    button{
                        class: "relative z-10 block w-8 h-8 overflow-hidden rounded-full shadow focus:outline-none",
                        // @click=\"dropdownOpen = !dropdownOpen\"\n
                        onclick: move |_| {
                            dropdown_open.set(!dropdown_open);
                        },
                        img {
                            class: "object-cover w-full h-full",
                            alt: "Your avatar",
                            src: "https://images.unsplash.com/photo-1528892952291-009c663ce843?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=crop&w=296&q=80",
                        }
                    }
                    div{
                        class:"fixed inset-0 z-10 w-full h-full",//
                        // v-show: "dropdownOpen",
                        //@click=\"dropdownOpen = false\"
                        // 点击屏幕关闭菜单
                        hidden: "{dropdown_open}",
                        onclick: move |_| {
                            dropdown_open.set(true);
                        },
                       
                    }
                
        //   <transition
        //   enter-active-class="transition duration-150 ease-out transform"
        //   enter-from-class="scale-95 opacity-0"
        //   enter-to-class="scale-100 opacity-100"
        //   leave-active-class="transition duration-150 ease-in transform"
        //   leave-from-class="scale-100 opacity-100"
        //   leave-to-class="scale-95 opacity-0"
        // >
                
                    div{
                        class:"absolute right-0 z-20 w-48 py-2 mt-2 bg-white rounded-md shadow-xl {dropdown_class}",
                        // v-show: "dropdownOpen",
                        hidden: "{dropdown_open}",
                        //onmouseout onmouseover
                        onmouseout: move |_| {
                            info!("onmouseout");
                        },
                        // style: "display:block",
                        
                        a{
                            class:"block px-4 py-2 text-sm text-gray-700 hover:bg-indigo-600 hover:text-white",
                            href:"#",
                            "Profile"
                        }
                        a{
                            class:"block px-4 py-2 text-sm text-gray-700 hover:bg-indigo-600 hover:text-white",
                            href:"#",
                            "Products"
                        }

                        a{
                            class:"block px-4 py-2 text-sm text-gray-700 hover:bg-indigo-600 hover:text-white",
                            href:"/",
                            "Log out"
                        }
                        
                    }
                }
                
              // </transition>
            }
        }
        
    })
}

mod icons{
    use dioxus::prelude::*;

    pub fn icon_1(cx: Scope)->Element{
        cx.render(rsx!{
            svg{
                class:"w-6 h-6",
                view_box:"0 0 24 24",
                fill:"none",
                xmlns:"http://www.w3.org/2000/svg",
                path{
                    d:"M4 6H20M4 12H20M4 18H11",
                    stroke:"currentColor",
                    stroke_width:"2",
                    stroke_linecap:"round",
                    stroke_linejoin:"round",
                }
                
            }
        })
    }

    pub fn icon_2(cx: Scope)->Element{
        cx.render(rsx!{
            svg{
                class:"w-5 h-5 text-gray-500",
                view_box:"0 0 24 24",
                fill:"none",
                xmlns:"http://www.w3.org/2000/svg",
                path{
                    d:"M21 21L15 15M17 10C17 13.866 13.866 17 10 17C6.13401 17 3 13.866 3 10C3 6.13401 6.13401 3 10 3C13.866 3 17 6.13401 17 10Z",
                    stroke:"currentColor",
                    stroke_width:"2",
                    stroke_linecap:"round",
                    stroke_linejoin:"round",
                }
            }
        })
    }

    pub fn icon_3(cx: Scope)->Element{
        cx.render(rsx!{
            svg{
                class:"w-6 h-6",
                view_box:"0 0 24 24",
                fill:"none",
                xmlns:"http://www.w3.org/2000/svg",
                path{
                    d:"M15 17H20L18.5951 15.5951C18.2141 15.2141 18 14.6973 18 14.1585V11C18 8.38757 16.3304 6.16509 14 5.34142V5C14 3.89543 13.1046 3 12 3C10.8954 3 10 3.89543 10 5V5.34142C7.66962 6.16509 6 8.38757 6 11V14.1585C6 14.6973 5.78595 15.2141 5.40493 15.5951L4 17H9M15 17V18C15 19.6569 13.6569 21 12 21C10.3431 21 9 19.6569 9 18V17M15 17H9",
                    stroke:"currentColor",
                    stroke_width:"2",
                    stroke_linecap:"round",
                    stroke_linejoin:"round",
                }
            }
        })
    }

    
}