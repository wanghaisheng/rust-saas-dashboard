/*
 * @Author: plucky
 * @Date: 2022-10-14 17:43:39
 * @LastEditTime: 2022-10-15 00:21:14
 * @Description: 
 */
#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::modules::demo_data::USE_TABLE_DATA;

pub fn view(cx: Scope)->Element{
    
    cx.render(rsx!{
        div {
            h3 {
                class: "text-3xl font-medium text-gray-700",
                "Tables"
            }
            // Simple Table
            Simple_table{}

            // Table with pagination
            Table_with_pagination{}

            // Wide Table
            Wide_table{}
        }
    })
}

// Simple Table
fn Simple_table(cx: Scope)->Element{
    let simple_table_data = &use_read(&cx, USE_TABLE_DATA).simpleTableData;
    
    cx.render(rsx!{
        div {
            class: "mt-4",
            h4 {
                class: "text-gray-600",
                "Simple Table"
            }
            div {
                class: "mt-6",
                div {
                    class: "my-6 overflow-hidden bg-white rounded-md shadow",
                    table {
                        class: "w-full text-left border-collapse",
                        thead {
                            class: "border-b",
                            tr {
                                th {
                                    class: "px-5 py-3 text-sm font-medium text-gray-100 uppercase bg-indigo-800",
                                    "City"
                                }
                                th {
                                    class: "px-5 py-3 text-sm font-medium text-gray-100 uppercase bg-indigo-800",
                                    "Total orders"
                                }
                            }
                        }
                        // data
                        tbody {
                            simple_table_data.iter().enumerate().map(|(i,data)|{
                                rsx!{ tr {
                                    class: "hover:bg-gray-200",
                                     key: "{i}",
                                    td {
                                        class: "px-6 py-4 text-lg text-gray-700 border-b",
                                        "{data.city}"
                                    }
                                    td {
                                        class: "px-6 py-4 text-gray-500 border-b",
                                        "{data.totalOrders}"
                                    }
                                }
                                }
                            })
                            
                        }
                    }
                }
            }
        }
    })
}

// 分页 table
// Table with pagination
fn Table_with_pagination(cx: Scope)->Element{
    let paginated_table_data = &use_read(&cx, USE_TABLE_DATA).paginatedTableData;
    
    let status_color = |status: &str| {
        match status {
            "Active" => ("bg-green-100", "text-green-800"),
            "Inactive" => ("bg-red-100", "text-red-800"),
            "Suspended" => ("bg-orange-100", "text-orange-800"),
            _ => ("bg-gray-100", "text-gray-800"),
        }
    };

    cx.render(rsx!{
        div {
            class: "mt-8",
            h4 {
                class: "text-gray-600",
                "Table with pagination"
            }
            div {
                class: "mt-6",
                h2 {
                    class: "text-xl font-semibold leading-tight text-gray-700",
                    "Users"
                }
                // limit
                div {
                    class: "flex flex-col mt-3 sm:flex-row",
                    div {
                        class: "flex",
                        div {
                            class: "relative",
                            select {
                                class: "block w-full h-full px-4 py-2 pr-8 leading-tight text-gray-700 bg-white border border-gray-400 rounded-l appearance-none focus:outline-none focus:bg-white focus:border-gray-500",
                                option {
                                    "5"
                                }
                                option {
                                    "10"
                                }
                                option {
                                    "20"
                                }
                            }
                            div {
                                class: "absolute inset-y-0 right-0 flex items-center px-2 text-gray-700 pointer-events-none",
                                icons::icon_1 {}
                            }
                        }
                        
                        div {
                            class: "relative",
                            select {
                                class: "block w-full h-full px-4 py-2 pr-8 leading-tight text-gray-700 bg-white border-t border-b border-r border-gray-400 rounded-r appearance-none sm:rounded-r-none sm:border-r-0 focus:outline-none focus:border-l focus:border-r focus:bg-white focus:border-gray-500",
                                option {
                                    "All"
                                }
                                option {
                                    "Active"
                                }
                                option {
                                    "Inactive"
                                }
                            }
                            div {
                                class: "absolute inset-y-0 right-0 flex items-center px-2 text-gray-700 pointer-events-none",
                                icons::icon_2 {}
                            }
                        }
                    }
                    div {
                        class: "relative block mt-2 sm:mt-0",
                        span {
                            class: "absolute inset-y-0 left-0 flex items-center pl-2",
                            icons::icon_3 {}
                        }
                        input {
                            class: "block w-full py-2 pl-8 pr-6 text-sm text-gray-700 placeholder-gray-400 bg-white border border-b border-gray-400 rounded-l rounded-r appearance-none sm:rounded-l-none focus:bg-white focus:placeholder-gray-600 focus:text-gray-700 focus:outline-none",
                            placeholder: "Search",
                        }
                    }
                }
                // table
                div {
                    class: "px-4 py-4 -mx-4 overflow-x-auto sm:-mx-8 sm:px-8",
                    div {
                        class: "inline-block min-w-full overflow-hidden rounded-lg shadow",
                        table {
                            class: "min-w-full leading-normal",
                            // table header
                            thead {
                                tr {
                                    th {
                                        class: "px-5 py-3 text-xs font-semibold tracking-wider text-left text-gray-600 uppercase bg-gray-100 border-b-2 border-gray-200",
                                        "User"
                                    }
                                    th {
                                        class: "px-5 py-3 text-xs font-semibold tracking-wider text-left text-gray-600 uppercase bg-gray-100 border-b-2 border-gray-200",
                                        "Role"
                                    }
                                    th {
                                        class: "px-5 py-3 text-xs font-semibold tracking-wider text-left text-gray-600 uppercase bg-gray-100 border-b-2 border-gray-200",
                                        "Created at"
                                    }
                                    th {
                                        class: "px-5 py-3 text-xs font-semibold tracking-wider text-left text-gray-600 uppercase bg-gray-100 border-b-2 border-gray-200",
                                        "Status"
                                    }
                                }
                            }
                            // data
                            tbody {
                                // "<tr v-for=\"(u, index) in paginatedTableData\" :key=\"index\">"
                                paginated_table_data.iter().map(|u|{rsx!{
                                tr{
                                    // key: "{u.id}",
                                    td {
                                        class: "px-5 py-5 text-sm bg-white border-b border-gray-200",
                                        div {
                                            class: "flex items-center",
                                            div {
                                                class: "flex-shrink-0 w-10 h-10",
                                                img {
                                                    class: "w-full h-full rounded-full",
                                                    src: "{u.picture}",
                                                    alt: "profile pic",
                                                }
                                            }
                                            div {
                                                class: "ml-3",
                                                p {
                                                    class: "text-gray-900 whitespace-nowrap",
                                                    "{ u.name }"
                                                }
                                            }
                                        }
                                    }
                                    td {
                                        class: "px-5 py-5 text-sm bg-white border-b border-gray-200",
                                        p {
                                            class: "text-gray-900 whitespace-nowrap",
                                            "{ u.role }"
                                        }
                                    }
                                    td {
                                        class: "px-5 py-5 text-sm bg-white border-b border-gray-200",
                                        p {
                                            class: "text-gray-900 whitespace-nowrap",
                                            "{ u.created }"
                                        }
                                    }
                                    td {
                                        class: "px-5 py-5 text-sm bg-white border-b border-gray-200",
                                        span {
                                            class: format_args!("inline-flex px-3 py-1 font-semibold leading-tight rounded-full {} {}",status_color(&u.status).0,status_color(&u.status).1),
                                            "{ u.status }"
                                        }

                                        // span {
                                        //     class: format_args!("relative inline-block px-3 py-1 font-semibold leading-tight {}", status_color(&u.status).1),
                                        //     span {
                                        //         aria_hidden: "true",
                                        //         class: {
                                        //             format_args!("absolute inset-0 opacity-50 rounded-full {}", status_color(&u.status).0)
                                        //         }
                                        //     }
                                        //     span {
                                        //         class: "relative",
                                        //         "{u.status}"
                                                
                                        //     }
                                        // }
                                        
                                        
                                    }
                                    //tr end
                                }
                                // iter end
                                }})
                       
                            }
                        }

                        div {
                            class: "flex flex-col items-center px-5 py-5 bg-white border-t xs:flex-row xs:justify-between",
                            span {
                                class: "text-xs text-gray-900 xs:text-sm",
                                "Showing 1 to 4 of 50 Entries"
                            }
                            div {
                                class: "inline-flex mt-2 xs:mt-0",
                                button {
                                    class: "px-4 py-2 text-sm font-semibold text-gray-800 bg-gray-300 rounded-l hover:bg-gray-400",
                                    "Prev"
                                }
                                button {
                                    class: "px-4 py-2 text-sm font-semibold text-gray-800 bg-gray-300 rounded-r hover:bg-gray-400",
                                    "Next"
                                }
                            }
                        }
                    }
                }
            }
        }
    })
}

// 宽表格
fn Wide_table(cx: Scope)->Element{
    let wide_table_data = &use_read(&cx, USE_TABLE_DATA).wideTableData;

    cx.render(rsx!{
        div {
            class: "mt-8",
            h4 {
                class: "text-gray-600",
                "Wide Table"
            }
            div {
                class: "flex flex-col mt-6",
                div {
                    class: "py-2 -my-2 overflow-x-auto sm:-mx-6 sm:px-6 lg:-mx-8 lg:px-8",
                    div {
                        class: "inline-block min-w-full overflow-hidden align-middle border-b border-gray-200 shadow sm:rounded-lg",
                        table {
                            class: "min-w-full",
                            thead {
                                tr {
                                    th {
                                        class: "px-6 py-3 text-xs font-medium leading-4 tracking-wider text-left text-gray-500 uppercase bg-gray-100 border-b border-gray-200",
                                        "Name"
                                    }
                                    th {
                                        class: "px-6 py-3 text-xs font-medium leading-4 tracking-wider text-left text-gray-500 uppercase bg-gray-100 border-b border-gray-200",
                                        "Title"
                                    }
                                    th {
                                        class: "px-6 py-3 text-xs font-medium leading-4 tracking-wider text-left text-gray-500 uppercase bg-gray-100 border-b border-gray-200",
                                        "Status"
                                    }
                                    th {
                                        class: "px-6 py-3 text-xs font-medium leading-4 tracking-wider text-left text-gray-500 uppercase bg-gray-100 border-b border-gray-200",
                                        "Role"
                                    }
                                    th {
                                        class: "px-6 py-3 bg-gray-100 border-b border-gray-200",
                                        
                                    }
                                }
                            }
                            // data
                            tbody {
                                class: "bg-white",
                                // iter start
                                wide_table_data.iter().map(|u|{rsx!{tr{
                                
                                td {
                                    class: "px-6 py-4 border-b border-gray-200 whitespace-nowrap",
                                    div {
                                        class: "flex items-center",
                                        div {
                                            class: "flex-shrink-0 w-10 h-10",
                                            img {
                                                class: "w-10 h-10 rounded-full",
                                                alt: "profile pic",
                                                src: "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80",
                                            }
                                        }
                                        div {
                                            class: "ml-4",
                                            div {
                                                class: "text-sm font-medium leading-5 text-gray-900",
                                                "{u.name}"
                                            }
                                            div {
                                                class: "text-sm leading-5 text-gray-500",
                                                "{ u.email }"
                                            }
                                        }
                                    }
                                }
                                td {
                                    class: "px-6 py-4 border-b border-gray-200 whitespace-nowrap",
                                    div {
                                        class: "text-sm leading-5 text-gray-900",
                                        "{ u.title }"
                                    }
                                    div {
                                        class: "text-sm leading-5 text-gray-500",
                                        "{ u.title2 }"
                                    }
                                }
                                td {
                                    class: "px-6 py-4 border-b border-gray-200 whitespace-nowrap",
                                    span {
                                        class: "inline-flex px-2 text-xs font-semibold leading-5 text-green-800 bg-green-100 rounded-full",
                                        "{ u.status }"
                                    }
                                }
                                td {
                                    class: "px-6 py-4 text-sm leading-5 text-gray-500 border-b border-gray-200 whitespace-nowrap",
                                    "{ u.role }"
                                }
                                td {
                                    class: "px-6 py-4 text-sm font-medium leading-5 text-right border-b border-gray-200 whitespace-nowrap",
                                    a {
                                        class: "text-indigo-600 hover:text-indigo-900",
                                        href: "#","Edit"
                                    }
                                }

                                // iter end
                                }}})
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
                class="w-4 h-4 fill-current"
                xmlns="http://www.w3.org/2000/svg"
                view_box="0 0 20 20"
                >
                <path
                    d="M9.293 12.95l.707.707L15.657 8l-1.414-1.414L10 10.828 5.757 6.586 4.343 8z"
                />
            </svg>
    })
    }
    
    pub fn icon_2(cx: Scope)->Element{
        cx.render(html!{
            <svg
                class="w-4 h-4 fill-current"
                xmlns="http://www.w3.org/2000/svg"
                view_box="0 0 20 20"
                >
                <path
                    d="M9.293 12.95l.707.707L15.657 8l-1.414-1.414L10 10.828 5.757 6.586 4.343 8z"
                />
            </svg>
        })
    }

    pub fn icon_3(cx: Scope)->Element{
        cx.render(html!{
            <svg
                view_box="0 0 24 24"
                class="w-4 h-4 text-gray-500 fill-current"
            >
                <path
                d="M10 4a6 6 0 100 12 6 6 0 000-12zm-8 6a8 8 0 1114.32 4.906l5.387 5.387a1 1 0 01-1.414 1.414l-5.387-5.387A8 8 0 012 10z"
                />
            </svg>
        })
    }

}