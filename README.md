# osint-mcp-server

OSINT MCP Server

Build and Run

```bash
cargo build --release
npx @modelcontextprotocol/inspector ./target/release/osint-mcp-server
```

Pull Docker Image and Run

```bash
docker pull ghcr.io/defendrite/osint-mcp-server:latest
docker run -i --rm ghcr.io/defendrite/osint-mcp-server:latest
```

To Use MCP Inspector on the Docker Image
``bash
npx @modelcontextprotocol/inspector docker run -i --rm ghcr.io/defendrite/osint-mcp-server:latest

```

## Technologies

- Rust
- rmcp
- Tokio
- Serde
- Model Context Protocol (MCP)

## Resources

- [Model Context Protocol](https://modelcontextprotocol.io/)
- [Rust](https://www.rust-lang.org/)
- [rmcp SDK](https://github.com/modelcontextprotocol/rust-sdk)
- [MCP Inspector](https://github.com/modelcontextprotocol/inspector)
```
