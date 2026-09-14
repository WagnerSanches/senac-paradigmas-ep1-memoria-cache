Integrantes: Wagner Aguiar Sanches Garcia Sobrinho

Enunciado original: https://github.com/celsocrivelaro/senac-paradigmas/blob/main/eps/ep01/enunciado.md

## O projeto

Banco de dados chave-valor feito em Rust. A parte de validação e formatação dos dados não fica presa no código Rust: cada tipo de dado (CPF, data, e-mail) é uma extensão em Lua, carregada em tempo de execução.

## Rodando

Precisa ter o Rust instalado.

```bash
git clone https://github.com/WagnerSanches/senac-paradigmas-ep1-memoria-cache
cd senac-paradigmas-ep1-memoria-cache
cargo run
```

## Estrutura

```
.
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs                  # cria storage, carrega extensões, inicia o REPL
│   └── modules/
│       ├── mod.rs
│       ├── storage.rs           # armazenamento chave-valor em memória
│       ├── parser.rs            # interpreta uma linha de texto em um Command
│       ├── input.rs             # laço de leitura (REPL) e despacho de comandos
│       └── lua_bridge.rs        # único módulo que conhece o mlua
├── extensions/
│   ├── cpf.lua
│   ├── data.lua
│   └── email.lua
├── casos_teste_formatado.txt
├── casos_teste_original.txt
└── casos_teste_email.txt
```

Resumo do que cada módulo faz:

- `storage.rs`: guarda e consulta os pares chave-valor. Não depende de nada dos outros.
- `parser.rs`: transforma a linha digitada em um `Command`. Também não depende de nada.
- `lua_bridge.rs`: carrega os `.lua` e expõe funções pro Lua chamar. Depende do storage.
- `input.rs`: laço principal, lê comando, despacha, imprime resultado. Depende dos três de cima.
- `main.rs`: monta tudo e inicia.

`mlua` só é usado dentro de `lua_bridge.rs`, o resto do código nem sabe que Lua existe.

## Como funciona o registro das extensões

Cada `.lua` dentro de `extensions/` termina retornando `{ prefixo, insert, select }`.

- `prefixo` é obrigatório: é comparado com `chave.starts_with(prefixo)`. Já inclui o separador, então é `"cpf_"` e não `"cpf"`.
- `insert` e `select` são opcionais. São chamados no `ADD` e no `GET`. Se não existirem, o valor é só gravado/lido sem nenhuma validação.
- As duas recebem `(chave, valor)` e sempre devolvem `(bool, string)` — `true, valor` quando dá certo, `false, mensagem` quando dá erro.

No boot o programa varre a pasta `extensions/`, carrega todos os `.lua` e guarda essas tabelas. Quando chega um comando, ele procura a primeira extensão cujo prefixo bate com a chave usada.

## Adicionando uma extensão nova

1. Cria um `.lua` novo em `extensions/`, o nome do arquivo não importa.
2. Termina o arquivo com `return { prefixo = "...", insert = function(chave, valor) ... end, select = function(chave, valor) ... end }`.
3. Se precisar checar se um valor já existe (unicidade), tem a função `database_find_by_value(valor)` disponível dentro do Lua — devolve a chave onde o valor está, ou `nil` se não achar.
4. Reinicia o programa. Não precisa mexer em nada do Rust.

## Detalhe: como o Lua acessa o banco

`database_find_by_value` é exposta como função global na VM Lua. O problema é que ela precisa acessar o mesmo `Storage` que o `ADD` já está usando naquele momento, e uma referência emprestada comum não resolve porque o `mlua::create_function` exige closures `'static`.

A saída foi colocar o `Storage` num `Rc<RefCell<Storage>>`. O `Rc` permite que o laço principal e a closure do Lua compartilhem posse do mesmo dado (clonar só incrementa a contagem de referência, não duplica nada). O `RefCell` move a checagem de empréstimo pra tempo de execução, com `.borrow()`/`.borrow_mut()`. Como a validação (leitura) sempre roda antes da gravação (escrita) dentro do mesmo comando, nunca tem os dois disputando ao mesmo tempo.

## Algumas decisões de projeto

- Retorno como `(bool, String)` em vez de uma tabela tipo `{ok, valor}` porque Lua já devolve múltiplos valores nativamente e assim não tem ambiguidade sobre se a string é o valor ou é a mensagem de erro.
- Prefixo usa `starts_with` (e já inclui o `_`) pra uma chave tipo `cpfx` não cair sem querer na extensão de CPF.
- Cada extensão decide o que efetivamente grava: CPF grava só os 11 dígitos, e-mail grava normalizado em minúsculas, data grava a string ISO como veio (a formatação `dd/mm/aaaa` só acontece no `select`).
- Chave sem extensão correspondente é gravada/lida como texto puro, sem validação nenhuma.
- A instância do Lua criada ao carregar as extensões precisa ficar viva durante toda a execução (fica guardada em `main.rs`), porque as funções Lua guardadas dependem dela continuar existindo.

## Extensão de e-mail

Prefixo: `email_`

- `insert` valida o formato `usuario@dominio.extensao` — exatamente um `@` e exatamente um `.` no domínio (então domínio tipo `.org.uk` não passa, foi decisão de projeto mesmo). Antes de gravar, normaliza pra minúsculas e checa unicidade com `database_find_by_value`.
- `select` só devolve o valor, que já está normalizado desde que foi gravado.

O que essa extensão testa de diferente das outras duas: CPF confere unicidade comparando o valor exato. Data nem consulta o banco. Já e-mail consulta o banco pra unicidade mas normaliza o valor antes de comparar e gravar, então o texto salvo não é idêntico ao digitado e a duplicidade é case-insensitive (`Ana@Teste.COM` e `ana@teste.com` são considerados o mesmo).

## Casos de teste

- `casos_teste.txt`: os casos do próprio enunciado, cobrindo CPF, Data e o comportamento geral do núcleo. Rodei todos manualmente e conferi contra o esperado, sem divergência.
- `casos_teste_email.txt`: casos que escrevi pra extensão de e-mail — formato válido, formatos malformados, unicidade (inclusive tentando duplicar entre chaves diferentes) e a normalização case-insensitive.

Dá pra rodar os dois via pipe:

```bash
cat casos_teste.txt | cargo run
cat casos_teste_email.txt | cargo run
```
