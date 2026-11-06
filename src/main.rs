use rmcp::{
    handler::server::wrapper::Parameters,
    schemars, tool, tool_router,
    ServiceExt,
    transport::stdio,
};

use serde::Deserialize;

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct HelloParams {
    name: String,
}

#[derive(Clone)]
struct TestServer;

#[tool_router(server_handler)]
impl TestServer {

    // Tool 1: Greeting
    #[tool(description = "Say hello to a user")]
    fn say_hello(
        &self,
        Parameters(HelloParams { name }): Parameters<HelloParams>,
    ) -> Result<String,String> {
        Ok(format!("Hello, {}! Welcome to Rust MCP Server.", name))
    }

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let server = TestServer.serve(stdio()).await?;

    eprintln!("Rust MCP Server started successfully!");

    server.waiting().await?;

    Ok(())
}