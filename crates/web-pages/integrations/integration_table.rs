#![allow(non_snake_case)]
use daisy_rsx::*;
use db::Integration;
use dioxus::prelude::*;

#[component]
pub fn IntegrationTable(integrations: Vec<Integration>, team_id: i32) -> Element {
    rsx!(
        Card {
            class: "has-data-table",
            CardHeader {
                title: "Integrations"
            }
            CardBody {
                table {
                    class: "table table-sm",
                    thead {
                        th { "Name" }
                        th {
                            class: "max-sm:hidden",
                            "Base URL"
                        }
                        th { "Model Type" }

                        th {
                            class: "text-right",
                            "Action"
                        }
                    }
                    tbody {
                        for integration in &integrations {
                            tr {
                                td {
                                    strong {
                                        "{integration.name}"
                                    }
                                }
                                td {
                                    class: "max-sm:hidden",
                                    code {
                                        class: "[overflow-wrap:anywhere]",
                                        "{integration.base_url}"
                                    }
                                }
                                td {
                                    super::integration_type::Integration {
                                        integration_type: integration.integration_type
                                    }
                                }
                                td {
                                    class: "text-right",
                                    DropDown {
                                        direction: Direction::Left,
                                        button_text: "...",
                                        DropDownLink {
                                            href: "#",
                                            drawer_trigger: format!("edit-integration-form-{}", integration.id),
                                            "Edit"
                                        }
                                        DropDownLink {
                                            drawer_trigger: format!("delete-trigger-{}-{}",
                                            integration.id, team_id),
                                            href: "#",
                                            target: "_top",
                                            "Delete"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    )
}
