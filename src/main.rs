use axum::{Json, Router, routing::get};
use rmcp::{
    handler::server::wrapper::Parameters,
    schemars, tool, tool_router,
    transport::streamable_http_server::{
        StreamableHttpServerConfig, StreamableHttpService, session::local::LocalSessionManager,
    },
};
use serde::{Deserialize, Serialize};

// -------------------------
// Tool Parameters
// -------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct BulkIpGeoParams {
    /// List of IPv4 or IPv6 addresses to investigate (maximum 100)
    pub ips: Vec<String>,
}

// -------------------------
// MCP Server
// -------------------------

#[derive(Clone)]
struct OSINTServer;

#[tool_router(server_handler)]
impl OSINTServer {
    // Tool 1: Bulk IP Geolocation
    #[tool(
        description = "Get geolocation and network information for multiple IPv4 or IPv6 addresses at once (maximum 100 IPs)"
    )]
    async fn bulk_ip_geolocation(
        &self,
        Parameters(BulkIpGeoParams { ips }): Parameters<BulkIpGeoParams>,
    ) -> Result<String, String> {
        use std::net::IpAddr;

        // Validate input
        if ips.is_empty() {
            return Err("Please provide at least one IP address".to_string());
        }

        if ips.len() > 100 {
            return Err("Maximum 100 IP addresses allowed".to_string());
        }

        // Validate IP addresses
        let mut validated_ips = Vec::new();

        for ip in ips {
            let parsed = ip
                .parse::<IpAddr>()
                .map_err(|_| format!("Invalid IP address: {}", ip))?;

            validated_ips.push(parsed.to_string());
        }

        // Fields to retrieve
        let fields = "status,message,query,continent,continentCode,country,countryCode,region,regionName,city,district,zip,lat,lon,timezone,offset,currency,isp,org,as,asname,mobile,proxy,hosting";

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| format!("Client creation failed: {}", e))?;
        let mut url = reqwest::Url::parse("http://ip-api.com/batch")
            .map_err(|e| format!("Invalid API URL: {}", e))?;

        url.query_pairs_mut().append_pair("fields", fields);

        let response = client
            .post(url)
            .json(&validated_ips)
            .send()
            .await
            .map_err(|e| format!("API request failed: {}", e))?;

        let status = response.status();

        // Read response body for useful error messages
        let body = response
            .text()
            .await
            .map_err(|e| format!("Failed to read API response: {}", e))?;

        if !status.is_success() {
            return Err(format!("IP-API returned HTTP {}: {}", status, body));
        }

        // Parse response
        let data: serde_json::Value = serde_json::from_str(&body)
            .map_err(|e| format!("JSON parsing failed: {}. Response: {}", e, body))?;

        // Return formatted JSON
        serde_json::to_string_pretty(&data).map_err(|e| format!("JSON formatting failed: {}", e))
    }
}

// -------------------------
// HTTP Response Models
// -------------------------

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
}

#[derive(Serialize)]
struct ServerInfo {
    name: &'static str,
    version: &'static str,
    description: &'static str,
    transport: &'static str,
    endpoints: Endpoints,
}

#[derive(Serialize)]
struct Endpoints {
    health: &'static str,
    mcp: &'static str,
}

// -------------------------
// HTTP Endpoints
// -------------------------

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "osint-mcp-server",
    })
}

async fn root() -> Json<ServerInfo> {
    Json(ServerInfo {
        name: "osint-mcp-server",
        version: "1.0.0",
        description: "OSINT MCP Server ",
        transport: "streamable-http",
        endpoints: Endpoints {
            health: "/health",
            mcp: "/mcp",
        },
    })
}

// -------------------------
// Main
// -------------------------

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure MCP HTTP service
    let mcp_service = StreamableHttpService::new(
        || Ok(OSINTServer),
        LocalSessionManager::default().into(),
        StreamableHttpServerConfig::default(),
    );

    // Build Axum application
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .nest_service("/mcp", mcp_service);

    // Start HTTP server
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let address = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&address).await?;

    eprintln!("MCP Server started successfully!");
    eprintln!("HTTP server listening on {}", &address);
    eprintln!("Health endpoint: http://localhost:8080/health");
    eprintln!("MCP endpoint: http://localhost:8080/mcp");

    axum::serve(listener, app).await?;

    Ok(())
}
