Integrantes: Wagner Aguiar Sanches Garcia Sobrinho

Enunciado original: https://github.com/celsocrivelaro/senac-paradigmas/blob/main/eps/ep01/enunciado.md

## O projeto
Uma implementacao de um banco de dados chave-valor escrito em Rust, cujo comportamento de validação e formatação é extensível via scripts Lua carregados em tempo de execução.

## Rodando o projeto

- Ter o Rust instalado

```bash
git clone https://github.com/WagnerSanches/senac-paradigmas-ep1-memoria-cache
cd senac-paradigmas-ep1-memoria-cache
cargo run
```

## Estrutura do projeto
 
```
.
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs                  # orquestra: cria storage, carrega extensões, inicia o REPL
│   └── modules/
│       ├── mod.rs               # declara os submódulos da pasta
│       ├── storage.rs           # armazenamento chave-valor em memória
│       ├── parser.rs            # interpreta uma linha de texto em um Command
│       ├── input.rs             # laço de leitura (REPL) e despacho de comandos
│       └── lua_bridge.rs        # único módulo que conhece o mlua
├── extensions/
│   ├── cpf.lua
│   ├── data.lua
│   └── email.lua
├── casos_teste.txt
└── casos_teste_email.txt
```

## Protocolo de registro de extensões

Cada `.lua` em `extensions/` termina com `return { prefixo, insert, select }`.

- **`prefixo`** (obrigatório): string comparada com `chave.starts_with(prefixo)`. Inclui o separador (`"cpf_"`, não `"cpf"`).
- **`insert`** e **`select`** (opcionais): funções chamadas em `ADD` e `GET`, respectivamente. Se ausentes, o motor grava/lê o valor sem validação nem formatação.
- Ambas sempre recebem `(chave, valor)` e sempre retornam `(bool, string)`: `true, valor` em sucesso, `false, mensagem` em erro.

No boot, o motor varre `extensions/`, carrega todo `.lua` e guarda essas tabelas. Ao processar um comando, procura a primeira extensão cujo prefixo bate com a chave.

## Casos de teste
 
- `casos_teste.txt`: casos fornecidos pelo enunciado, cobrindo CPF, Data e comportamento geral do núcleo. Todos os casos foram executados manualmente e conferidos contra o resultado esperado, sem divergência de comportamento.
- `casos_teste_email.txt`: casos próprios para a extensão de e-mail, cobrindo formato válido, formatos malformados, unicidade (incluindo tentativa de duplicidade cruzada entre chaves) e normalização case-insensitive.
Ambos os arquivos podem ser executados via pipe:
```bash
cat casos_teste.txt | cargo run
cat casos_teste_email.txt | cargo run
```