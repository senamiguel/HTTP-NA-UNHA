# HTTP-NA-UNHA 🦀

> A bare-metal HTTP/TCP engine built from scratch in Rust. Zero external dependencies, zero high-level frameworks. Just raw sockets, byte-splitting, and memory safety.

## O que é isso?

Um servidor HTTP implementado **na unha** — sem frameworks, sem crates externas, sem abstração. O objetivo é entender o protocolo HTTP por baixo dos panos, manipulando diretamente sockets TCP e bytes brutos usando apenas a standard library do Rust.

## Como funciona

```
Cliente (browser/curl)
       │
       │ TCP connect em 127.0.0.1:6769
       ▼
  TcpListener::bind()
       │
       ▼
  read_request()        → lê bytes do stream até encontrar \r\n\r\n
       │
       ▼
  build_response_body() → carrega index.html e injeta conteúdo dinâmico
       │
       ▼
  send_response()       → escreve status line + headers + body no stream
```

### Fluxo do request

1. O servidor escuta na porta `6769`
2. A cada conexão aceita, lê os bytes em chunks de 1024 bytes num loop
3. Detecta o fim dos headers HTTP procurando pela sequência `\r\n\r\n`
4. Lê o `index.html` do disco
5. Injeta um parágrafo dinâmico antes do `</body>`
6. Responde com `HTTP/1.1 200 OK` + `Content-Length` + o HTML montado

## Estrutura do projeto

```
HTTP-NA-UNHA/
├── Cargo.toml
├── src/
│   ├── main.rs       # toda a lógica do servidor
│   └── index.html    # template HTML servido nas respostas
└── README.md
```

## Funções

| Função | Responsabilidade |
|---|---|
| `main` | Inicializa o listener e despacha conexões |
| `create_listener` | Cria o `TcpListener`, com saída limpa em caso de falha fatal |
| `handle_connection` | Orquestra o ciclo completo de uma conexão |
| `read_request` | Lê o request HTTP até o delimitador `\r\n\r\n` |
| `send_response` | Escreve a resposta HTTP completa no stream |
| `build_response_body` | Lê o HTML do disco e coordena a injeção |
| `inject_into_html` | Lógica pura de string: insere conteúdo antes de uma tag alvo |

## Como rodar

```bash
git clone https://github.com/seu-usuario/HTTP-NA-UNHA
cd HTTP-NA-UNHA
cargo run
```

Com o servidor rodando, acesse:

```
http://127.0.0.1:6769
```

## Tratamento de erros

```
Error handling connection: client disconnected 
```

## Dependências

Nenhuma. Apenas `std`.

```toml
[dependencies]
# vazio de propósito
```

## Licença

MIT
