#![allow(non_snake_case)]
use daisy_rsx::{select::SelectOption, *};
use db::Prompt;
use dioxus::prelude::*;

#[component]
pub fn Form(team_id: i32, prompts: Vec<Prompt>) -> Element {
    rsx!(
        Drawer {
            submit_action: crate::routes::api_keys::New{ team_id }.to_string(),
            label: "New API Key",
            trigger_id: "create-api-key",
            DrawerBody {
                div {
                    class: "flex flex-col",
                    Input {
                        input_type: InputType::Text,
                        placeholder: "Production API Key",
                        help_text: "Give your new key a name",
                        required: true,
                        label: "Name",
                        name: "name"
                    }
                    Select {
                        name: "prompt_id",
                        label: "Please select an Assistant",
                        label_class: "mt-4",
                        help_text: "All access via this API key will use the above assistant",
                        {prompts.iter().map(|prompt| rsx!(
                            SelectOption {
                                value: "{prompt.id}",
                                "{prompt.name}"
                            }
                        ))}
                    }
                }
            }
            DrawerFooter {
                Button {
                    button_type: ButtonType::Submit,
                    button_scheme: ButtonScheme::Primary,
                    "Create API Key"
                }
            }
        }
    )
}
