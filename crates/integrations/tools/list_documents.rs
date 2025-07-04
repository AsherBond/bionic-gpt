use crate::tool::ToolInterface;
use async_trait::async_trait;
use db::{Pool, Transaction};
use openai_api::{BionicToolDefinition, ChatCompletionFunctionDefinition};
use serde_json::json;
use tracing;

/// A tool that lists all documents available to the user in this chat session or knowledge base.
pub struct ListDocumentsTool {
    pool: Pool,
    sub: String,
    conversation_id: i64,
}

impl ListDocumentsTool {
    pub fn new(pool: Pool, sub: String, conversation_id: i64) -> Self {
        tracing::debug!(
            "Creating new ListDocumentsTool with sub: {:?}, conversation_id: {:?}",
            sub,
            conversation_id
        );
        Self {
            pool,
            sub,
            conversation_id,
        }
    }
}

/// Returns the tool definition for list_documents
pub fn get_tool_definition() -> BionicToolDefinition {
    BionicToolDefinition {
        r#type: "function".to_string(),
        function: ChatCompletionFunctionDefinition {
            name: "list_documents".to_string(),
            description:
                "List all available documents for this conversation and retrieve their ID's, 
            includes uploaded files and previously indexed materials.
            Use this if the user refers to an attachment or document but doesn’t specify which one."
                    .to_string(),
            parameters: json!({
                "type": "object",
                "properties": {},
                "required": []
            }),
        },
    }
}

#[async_trait]
impl ToolInterface for ListDocumentsTool {
    fn get_tool(&self) -> BionicToolDefinition {
        tracing::debug!("Getting tool definition for ListDocumentsTool");
        get_tool_definition()
    }

    #[tracing::instrument(skip(self, arguments), fields(conversation_id = ?self.conversation_id, sub = ?self.sub))]
    async fn execute(&self, arguments: &str) -> Result<serde_json::Value, serde_json::Value> {
        tracing::info!(
            "Executing list_documents tool with arguments: {}",
            arguments
        );

        // Create transaction
        let mut client = match self.pool.get().await {
            Ok(client) => client,
            Err(e) => {
                return Err(json!({
                    "error": "Failed to get database client",
                    "details": e.to_string()
                }));
            }
        };

        let transaction = match client.transaction().await {
            Ok(transaction) => transaction,
            Err(e) => {
                return Err(json!({
                    "error": "Failed to create transaction",
                    "details": e.to_string()
                }));
            }
        };

        // Set row-level security
        if let Err(e) =
            db::authz::set_row_level_security_user_id(&transaction, self.sub.clone()).await
        {
            return Err(json!({
                "error": "Failed to set row level security",
                "details": e.to_string()
            }));
        }

        // Use the conversation ID to get documents
        let result = list_documents(&transaction, self.conversation_id).await;

        // Commit transaction
        if let Err(e) = transaction.commit().await {
            return Err(json!({
                "error": "Failed to commit transaction",
                "details": e.to_string()
            }));
        }

        result
    }
}

/// Lists documents for a given conversation
#[tracing::instrument(skip(transaction))]
async fn list_documents(
    transaction: &Transaction<'_>,
    conversation_id: i64,
) -> Result<serde_json::Value, serde_json::Value> {
    let attachments = match db::queries::attachments::get_by_conversation()
        .bind(transaction, &conversation_id)
        .all()
        .await
    {
        Ok(attachments) => attachments,
        Err(e) => {
            return Err(json!({
                "error": "Failed to get documents",
                "details": e.to_string()
            }));
        }
    };

    let result = json!({
        "documents": attachments.iter().map(|a| {
            json!({
                "file_id": a.id,
                "name": a.file_name,
                "size": a.file_size,
                "mime_type": a.mime_type
            })
        }).collect::<Vec<_>>()
    });

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_list_documents_tool() {
        let tool = get_tool_definition();
        assert_eq!(tool.function.name, "list_documents");
    }
}
