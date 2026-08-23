# 💰 Carteira de Investimentos Fullstack com Rust

[![Rust](https://img.shields.io/badge/Rust-1.80+-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Axum](https://img.shields.io/badge/Axum-Web%20Framework-blue?style=for-the-badge)](https://github.com/tokio-rs/axum)
[![PostgreSQL](https://img.shields.io/badge/PostgreSQL-Database-316192?style=for-the-badge&logo=postgresql)](https://www.postgresql.org/)

Uma aplicação completa de **gerenciamento de carteira de investimentos** desenvolvida em **Rust**. O projeto combina um backend robusto em Axum com um frontend interativo, banco de dados PostgreSQL e autenticação segura via JWT.

Este é meu fork e versão melhorada do desafio proposto pela [DIO](https://www.dio.me/).

---

## 📋 Índice

1. [O que o projeto faz](#-o-que-o-projeto-faz)
2. [Tecnologias utilizadas](#-tecnologias-utilizadas)
3. [Como executar](#-como-executar-a-aplicação)
4. [Melhorias implementadas](#-melhorias-implementadas)
5. [Como testar](#-como-testar)
6. [O que aprendi](#-o-que-aprendi)

---

## 🎯 O que o projeto faz

A aplicação **Carteira de Investimentos** permite que usuários gerenciem seus ativos e investimentos de forma prática e intuitiva.

### ✨ Funcionalidades principais:

- 📊 **Dashboard de ativos** - Visualize todos os ativos que você possui
- 🎯 **Compra de ativos** - Registre novas compras com histórico completo
- 👤 **Autenticação segura** - Login com JWT + auto-registro
- 🔐 **Painel administrativo protegido** - Apenas admins podem criar/atualizar ativos
- 📱 **Interface responsiva** - Funciona em desktop, tablet e mobile
- 🗄️ **Persistência de dados** - Tudo armazenado no PostgreSQL

### 🔄 Como funciona:

```
1. Usuário acessa a aplicação
         ↓
2. Faz login (ou se registra automaticamente)
         ↓
3. Visualiza ativos disponíveis
         ↓
4. Registra ativos comprados e acompanha sua carteira
         ↓
5. Vê o histórico de compras e performance
```

---

## 🛠️ Tecnologias Utilizadas

### Backend:
| Tecnologia |
|-----------|
| 🦀 **Rust** | 
| 🚀 **Axum** | 
| ⚡ **Tokio** |
| 🗄️ **PostgreSQL** |
| 🔗 **SQLx** | 
| 🔐 **JWT** |
| 🔒 **Argon2** |

### Frontend:
| Tecnologia |
|-----------|
| 🎨 **HTML5** | 
| 🎭 **Askama** | 
| 🌈 **Tailwind CSS** | 
| ✨ **Vanilla JS** | 

---

## 🚀 Como Executar a Aplicação

### 📦 Pré-requisitos

Antes de começar, certifique-se que você tem instalado:

- [Rust](https://www.rust-lang.org/tools/install) (1.70+)
- [PostgreSQL](https://www.postgresql.org/)
- [Git](https://git-scm.com/)

### ⚙️ Passo 1: Clone o repositório

```bash
git clone https://github.com/u-riick/projeto-carteira-investimento-rust.git
cd projeto-carteira-investimento-rust
```

### ⚙️ Passo 2: Configure o banco de dados

**PostgreSQL local**

Certifique-se que PostgreSQL está instalado e rodando com o banco padrão `postgres`.

### ⚙️ Passo 3: Configure variáveis de ambiente

Crie um arquivo `.env` na raiz do projeto:

```env
DATABASE_URL=postgres://postgres:sua_senha_postgres@localhost:5432/postgres
POSTGRES_PASSWORD = sua_senha_postgres
```

### ⚙️ Passo 4: Execute as migrations

```bash
sqlx migrate run
```

Isso cria automaticamente as tabelas no banco de dados.

### ⚙️ Passo 5: Inicie a aplicação

```bash
cargo run
```

A aplicação estará disponível em:

```
🌐 http://localhost:3000/
```

---

## 🌟 Melhorias Implementadas

Durante o desenvolvimento do projeto, foram implementadas duas melhorias principais relacionadas ao cadastro de ativos.

### 1️⃣ Validação Rigorosa de Dados

Ao cadastrar um novo ativo, o sistema valida:

✅ **Nome não pode estar vazio**
```json
// ❌ Rejeitado
{ "name": "", "unit_value": 100 }
```

✅ **Valor não pode ser zero**
```json
// ❌ Rejeitado
{ "name": "Bitcoin", "unit_value": 0 }
```

✅ **Valor não pode ser negativo**
```json
// ❌ Rejeitado
{ "name": "Ethereum", "unit_value": -50 }
```

**Resposta de erro:**
```json
HTTP 406 Not Acceptable
{ "error": "Asset Invalid Data" }
```

### 2️⃣ Prevenção de Ativos Duplicados

Implementei uma verificação para evitar que o mesmo ativo seja cadastrado duas vezes.

A função `find_asset_by_name()` consulta o banco antes de inserir:

```rust
pub async fn find_asset_by_name(
    &self, 
    name: &str
) -> sqlx::Result<Option<Asset>> {
    sqlx::query_as::<_, Asset>(
        "SELECT * FROM assets WHERE name = $1"
    )
    .bind(name)
    .fetch_optional(&self.pool)
    .await
}
```

**Se o ativo já existe:**
```json
HTTP 409 Conflict
{ "error": "Asset already exists" }
```

---

## 🧪 Como Testar

### ✅ Teste 1: Login e Acessar Dashboard

```bash
# 1. Abra o navegador
open http://localhost:3000

# 2. Insira credenciais (primeira vez = registro automático)
Username: seu_usuario
Password: sua_senha

# 3. Você deve ver o dashboard com ativos disponíveis
```

### ✅ Teste 2: Cadastrar um Novo Ativo (Admin)

**recomendado uso do HTTPie (deve possuir o python instalado):**

Windows PowerShell:
```powershell
winget install HTTPie
```
ou

Git Bash:
```bash
pip install httpie
```

```bash
http POST http://localhost:3000/api/assets name=NomeMoeda unit_value:=1.0 Authorization:123
```
A senha de admin pode ser alterada em src/auth/admin.rs.

**Resposta esperada:**
```json
HTTP/1.1 200 OK
content-length: 50
content-type: application/json
date: Sat, 22 Aug 2026 22:45:31 GMT

{
    "id":x,
    "name": "NomeMoeda",
    "unit_value": 1.0
}
```

### ❌ Teste 3: Tentar Cadastrar Ativo Inválido

```bash
http POST http://localhost:3000/api/assets name="" unit_value:=0.0 Authorization:123
```

**Resposta esperada:**
```json
HTTP 406 Not Acceptable
{
  "error": "Asset Invalid Data"
}
```

### ❌ Teste 4: Tentar Cadastrar Ativo Duplicado

```bash
# Primeira vez (sucesso)
http POST http://localhost:3000/api/assets name=NomeMoeda unit_value:=1.0 Authorization:123

# Segunda vez (falha)
http POST http://localhost:3000/api/assets name=NomeMoeda unit_value:=1.0 Authorization:123
```

**Resposta da segunda requisição:**
```json
HTTP 409 Conflict
{
  "error": "Asset already exists"
}
```

### ✅ Teste 5: Comprar um Ativo

Na interface web:
1. Faça login
2. Selecione um ativo no dropdown
3. Digite o preço por unidade (ou deixe padrão)
4. Digite a quantidade comprada
5. Clique em "Adicionar à carteira"
6. O ativo aparecerá na seção "Meus investimentos"

### ✅ Teste 6: Visualizar Histórico de Compras

1. Após comprar um ativo, clique em "VER HISTÓRICO DE COMPRAS"
2. Você verá todas as compras deste ativo com:
   - Quantidade
   - Preço pago
   - Data/hora
   - Lucro/prejuízo

---

## 📚 O que Aprendi

Durante o desenvolvimento deste projeto, consolidei conhecimentos importantes em Rust e desenvolvimento backend:

### 🦀 Conceitos Rust

✅ **Ownership e Borrowing** - Entendi como Rust gerencia memória sem GC

✅ **Async/Await** - Construí endpoints não-bloqueantes com Tokio

✅ **Pattern Matching** - Usei `match` para tratamento de `Result` e `Option`

✅ **Traits** - Implementei `FromRequestParts` para extratores customizados

✅ **Error Propagation** - Usei `?` operator para código limpo

### 🔄 Arquitetura Backend

✅ **Separação de Responsabilidades** - Camadas: Routes → Handlers → Repository → DB

✅ **Error Handling** - Criação de `AppError` enum com mapeamento automático para HTTP

✅ **Autenticação** - Implementei JWT e extratores customizados

✅ **Validação** - Validei dados em múltiplas camadas

### 💾 Banco de Dados

✅ **SQL Basics** - Consultas, joins, agregações

✅ **Migrations** - Versionamento automático de schema

✅ **N-to-N Relationships** - Relacionamento user-asset via owned_assets

### 🚀 Full Stack

✅ **Templates** - Renderização dinâmica com Askama

✅ **Forms & State** - Gerenciar estado do usuário no servidor

✅ **HTTP Status Codes** - Uso correto de 200, 201, 404, 409, etc.

---

## 🔧 Comandos Úteis

```bash
# Executar a aplicação
cargo run

# Verificar código sem compilar
cargo check

# Executar testes
cargo test

# Formatar código
cargo fmt

# Compilar para produção
cargo build --release

# Database migrations
sqlx migrate add -r <nome>          # Criar nova migration
sqlx migrate run                    # Executar todas
sqlx migrate revert                 # Reverter última
```

---

## 👨‍💻 Autor

**Rick** 

- 🔗 [GitHub](https://github.com/u-riick)
- 💼 [LinkedIn](www.linkedin.com/in/rickelme-chaves-7b3333307)
- 📧 Email: rickelmexb@hotmail.com

---
## 📌 Repositório Original

Este projeto foi desenvolvido a partir do desafio Carteira de Investimentos Fullstack com Rust, disponibilizado no curso "*Santander 2026 - Rust AI Developer*" pela **DIO**.

- 🔗 [REPOSITÓRIO ORIGINAL](https://github.com/digitalinnovationone/rust-fullstack-carteira-investimentos)

<div align="center">

[⬆ Voltar ao topo](#-carteira-de-investimentos-fullstack-com-rust)

</div>
