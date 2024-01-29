use std::{future::Future, pin::Pin};

use tower_lsp::{
	jsonrpc,
	lsp_types::{
		CompletionParams, CompletionResponse, InitializeParams,
		InitializeResult, InitializedParams, MessageType,
	},
	Client, LanguageServer, LspService, Server,
};

#[derive(Debug)]
struct Backend {
	client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
	async fn initialize(
		&self,
		_: InitializeParams,
	) -> jsonrpc::Result<InitializeResult> {
		Ok(InitializeResult::default())
	}

	async fn initialized(&self, _: InitializedParams) {
		self.client
			.log_message(MessageType::INFO, "server initialized!")
			.await;
	}

	async fn shutdown(&self) -> jsonrpc::Result<()> { Ok(()) }

	async fn completion(
		&self,
		params: CompletionParams,
	) -> jsonrpc::Result<Option<CompletionResponse>> {
		todo!()
	}
}

#[tokio::main]
async fn main() {
	let stdin = tokio::io::stdin();
	let stdout = tokio::io::stdout();

	let (service, socket) = LspService::new(|client| Backend { client });
	Server::new(stdin, stdout, socket)
		.serve(service)
		.await;
}
