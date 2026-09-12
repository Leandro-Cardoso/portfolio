# ETAPA 01 — FUNDAÇÃO DO FRONTEND

## CONTEXTO

Você está trabalhando no frontend de um portfólio profissional de:

**Leandro Cardoso — Engenheiro de Software**

Áreas principais:

* Performance
* Inteligência Artificial
* Automação
* Cibersegurança

Você deve respeitar integralmente o **PROMPT MESTRE DO PROJETO**, fornecido juntamente com este prompt.

Este é o início do desenvolvimento do frontend.

---

# OBJETIVO DESTA ETAPA

Criar a **fundação inicial do projeto Vue**, estabelecendo uma estrutura simples, modular e organizada para o restante do desenvolvimento.

O objetivo não é desenvolver o portfólio completo.

O objetivo é preparar uma base sólida e simples para as próximas etapas.

---

# STACK PERMITIDA

Utilizar apenas como base:

* Vue
* HTML
* CSS
* JavaScript

## IMPORTANTE

Não assumir ou adicionar automaticamente:

* Vite
* Nuxt
* TypeScript
* Tailwind
* Bootstrap
* Material UI
* Frameworks CSS
* Bibliotecas de componentes
* Gerenciadores globais de estado
* Bibliotecas adicionais

Não introduza nenhuma tecnologia que não tenha sido explicitamente aprovada.

Se a configuração inicial do projeto Vue exigir uma decisão sobre ferramenta de criação, build ou execução, **não invente essa decisão silenciosamente**.

Explique claramente qual decisão é necessária e por quê.

---

# ESCOPO DA ETAPA

Nesta etapa, você deve trabalhar apenas na fundação do frontend.

Preparar:

* Estrutura inicial do projeto
* Organização inicial das pastas
* Arquivo principal da aplicação
* Estrutura base da aplicação
* Organização inicial dos estilos
* Organização inicial das configurações
* Preparação para componentes
* Preparação para páginas futuras
* Preparação para dados locais futuros
* Preparação para serviços futuros

---

# ESTRUTURA ESPERADA

A estrutura deve ser simples.

Utilize como referência:

```text
src/
│
├── assets/
│
├── components/
│
├── composables/
│
├── config/
│
├── data/
│
├── layouts/
│
├── router/
│
├── services/
│
├── styles/
│
├── views/
│
├── App.vue
│
└── main.js
```

## IMPORTANTE

Essa estrutura é uma referência.

Não criar automaticamente todas as pastas apenas para deixá-las vazias.

Criar somente o que for necessário para estabelecer uma fundação coerente.

Pastas adicionais só podem ser criadas quando possuírem uma responsabilidade clara.

---

# RESPONSABILIDADE DAS PASTAS

## assets/

Arquivos estáticos pertencentes ao frontend.

Exemplos futuros:

* Imagens
* Ícones
* Outros recursos visuais

Não adicionar recursos fictícios.

---

## components/

Componentes reutilizáveis da aplicação.

Nesta etapa, não é necessário criar todos os componentes futuros.

Apenas preparar a organização necessária.

Futuramente poderá conter áreas como:

```text
components/
├── common/
├── layout/
├── home/
├── about/
├── projects/
├── blog/
└── services/
```

Não criar todos esses diretórios vazios sem necessidade.

---

## composables/

Funções reutilizáveis relacionadas à lógica do Vue.

Não criar composables fictícios.

---

## config/

Configurações centralizadas da aplicação.

Preparar a estrutura para futuramente centralizar:

* Informações do site
* Navegação
* Redes sociais
* Outras configurações globais

Não inventar links pessoais.

---

## data/

Dados locais temporários.

Essa pasta será utilizada futuramente para representar dados enquanto a API Rust não existir.

Exemplos futuros:

* Perfil
* Projetos
* Tecnologias
* Blog

Não preencher dados pessoais fictícios.

---

## layouts/

Estruturas reutilizáveis de layout.

Exemplos futuros:

* Layout público
* Layout administrativo

Não desenvolver o sistema administrativo nesta etapa.

---

## router/

Organização da navegação entre páginas.

### IMPORTANTE

Não adicionar bibliotecas de roteamento automaticamente.

Se o sistema de navegação exigir uma dependência externa, explique isso antes de adicioná-la.

Nesta etapa, o foco é preparar a arquitetura, não desenvolver todas as rotas.

---

## services/

Camada responsável pela comunicação futura com dados externos.

Futuramente:

```text
Vue
 ↓
Services
 ↓
API Rust
 ↓
PostgreSQL
```

Nesta etapa, não implementar API.

---

## styles/

Centralização dos estilos globais.

A estrutura deverá ser preparada para futuramente conter:

* Reset
* Tokens
* Temas
* Tipografia
* Estilos globais

Não implementar ainda o sistema completo de Design System.

Essa funcionalidade será criada em etapa específica.

---

## views/

Páginas principais da aplicação.

Futuramente:

* Home
* Sobre
* Projetos
* Blog
* Serviços

Não desenvolver todas as páginas nesta etapa.

---

# ARQUIVO App.vue

Criar uma estrutura inicial simples.

O arquivo não deve conter:

* Layout complexo
* Página Home completa
* Dashboard
* Projetos
* Blog
* Dados pessoais fictícios

Ele deve apenas funcionar como ponto central da aplicação.

---

# ARQUIVO main.js

Criar o ponto de entrada da aplicação.

Ele deve ser simples e organizado.

Não adicionar configurações ou dependências desnecessárias.

---

# CSS INICIAL

Criar apenas a base necessária para os estilos.

Nesta etapa, o objetivo não é finalizar:

* Design System
* Temas
* Dark Mode
* Light Mode
* Responsividade completa

Essas funcionalidades serão tratadas em etapas específicas.

Porém, o CSS deve ser organizado desde o início para permitir essa evolução.

---

# CONFIGURAÇÕES INICIAIS

Se necessário criar uma configuração inicial do site, ela deve conter apenas informações confirmadas.

Informações permitidas:

```text
Nome:
Leandro Cardoso

Profissão:
Engenheiro de Software

Áreas:
Performance
Inteligência Artificial
Automação
Cibersegurança
```

Não inventar:

* E-mail
* GitHub
* LinkedIn
* WhatsApp
* Instagram
* YouTube
* X
* Experiências
* Empresas
* Projetos

---

# NÃO FAZER NESTA ETAPA

É proibido antecipar as próximas etapas.

Não desenvolver:

* Página Home completa
* Página Sobre
* Página Projetos
* Página Blog
* Página Serviços
* Sistema administrativo
* Login
* API
* Backend Rust
* PostgreSQL
* Newsletter
* Comentários
* Curtidas
* Avaliações
* Integração com YouTube
* Geração de PDF
* Sistema completo de temas
* Dark Mode
* Light Mode
* Design System completo

---

# PRINCÍPIOS DE IMPLEMENTAÇÃO

Sempre priorizar:

## Simplicidade

Não criar abstrações desnecessárias.

---

## Organização

Cada arquivo deve possuir uma responsabilidade clara.

---

## Modularidade

Preparar a aplicação para crescer sem criar uma arquitetura excessivamente complexa.

---

## Reutilização

Evitar duplicação de estruturas e configurações.

---

## Evolução gradual

Não criar hoje funcionalidades necessárias apenas em etapas futuras.

Preparar apenas o necessário para que elas possam ser implementadas corretamente depois.

---

# DEPENDÊNCIAS

Não instalar dependências adicionais sem necessidade real.

Antes de adicionar qualquer dependência, verificar:

> Isso pode ser resolvido apenas com Vue, HTML, CSS e JavaScript?

Se puder, não adicionar dependências.

---

# RESULTADO ESPERADO

Ao finalizar esta etapa, o projeto deve possuir:

* Uma fundação Vue funcional
* Estrutura inicial organizada
* Ponto de entrada funcionando
* App.vue funcionando
* Organização inicial para estilos
* Organização inicial para configurações
* Preparação para componentes futuros
* Preparação para páginas futuras
* Preparação para dados locais
* Preparação para futura integração com API

O resultado deve ser simples.

Não criar uma aplicação visualmente completa.

---

# CHECKLIST DE VALIDAÇÃO

Antes de finalizar, verificar:

* [ ] O projeto utiliza Vue.
* [ ] O código utiliza JavaScript.
* [ ] Não foi introduzido TypeScript.
* [ ] Não foi introduzido Nuxt.
* [ ] Não foi introduzido Vite sem autorização explícita.
* [ ] Não foi introduzido Tailwind.
* [ ] Não foram adicionados frameworks CSS.
* [ ] Não foram adicionadas dependências desnecessárias.
* [ ] A estrutura está organizada.
* [ ] Não existem dados pessoais inventados.
* [ ] Não existem projetos fictícios apresentados como reais.
* [ ] App.vue está simples.
* [ ] main.js está organizado.
* [ ] A estrutura permite evolução futura.
* [ ] Nenhuma funcionalidade das próximas etapas foi antecipada.

---

# FORMATO DA RESPOSTA

Ao concluir a etapa, responda exatamente com estas seções:

## 1. Resumo da implementação

Explique objetivamente o que foi feito.

## 2. Estrutura criada

Mostre a árvore real de arquivos e pastas criados.

Não mostrar arquivos que não existem.

## 3. Arquivos criados

Listar cada arquivo criado e sua responsabilidade.

## 4. Arquivos alterados

Listar arquivos alterados.

## 5. Código

Mostrar o código completo de todos os arquivos criados ou alterados nesta etapa.

Não mostrar pseudocódigo.

## 6. Dependências

Informar claramente:

* Dependências utilizadas
* Motivo de cada dependência

Caso nenhuma dependência adicional tenha sido utilizada, informar isso explicitamente.

## 7. Como executar

Explicar como executar o projeto utilizando apenas as ferramentas efetivamente definidas e utilizadas.

Não inventar comandos ou ferramentas.

## 8. Validação

Explicar como confirmar que a etapa foi concluída corretamente.

## 9. O que NÃO foi implementado

Listar explicitamente as funcionalidades deixadas para as próximas etapas.

## 10. Próximo estado esperado

Informar qual será o estado do projeto após esta etapa, para servir de contexto ao próximo chat.

---

# REGRA FINAL

Não tente impressionar adicionando tecnologias, bibliotecas ou arquitetura desnecessária.

A qualidade desta etapa será medida pela:

* Simplicidade
* Organização
* Clareza
* Modularidade
* Consistência com o Prompt Mestre

Não pela quantidade de código ou dependências utilizadas.
