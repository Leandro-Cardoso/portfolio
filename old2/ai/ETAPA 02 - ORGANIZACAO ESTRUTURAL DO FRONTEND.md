# ETAPA 02 — ORGANIZAÇÃO ESTRUTURAL DO FRONTEND

## CONTEXTO

Você está dando continuidade ao desenvolvimento do frontend de um portfólio profissional de:

**Leandro Cardoso — Engenheiro de Software**

Áreas principais:

* Performance
* Inteligência Artificial
* Automação
* Cibersegurança

Você deve respeitar integralmente o **PROMPT MESTRE DO PROJETO**.

A **Etapa 01 — Fundação do Frontend** já foi executada.

Você deve trabalhar sobre o estado real do projeto deixado pela etapa anterior.

---

# OBJETIVO DESTA ETAPA

Organizar a estrutura interna do frontend para que o desenvolvimento das próximas páginas possa ocorrer de forma:

* Simples
* Modular
* Reutilizável
* Consistente
* Responsiva
* Fácil de manter

Esta etapa é principalmente de **organização estrutural**.

Não é para desenvolver o portfólio completo.

---

# REGRA FUNDAMENTAL

Não assuma que a estrutura criada na Etapa 01 é exatamente igual à estrutura sugerida no Prompt Mestre.

Primeiro:

1. Inspecione o projeto existente.
2. Entenda a estrutura realmente criada.
3. Preserve o que estiver correto.
4. Faça somente as alterações necessárias.
5. Não recrie o projeto do zero sem necessidade.

O estado real do projeto é mais importante que uma estrutura teórica.

---

# STACK

A stack continua sendo:

* Vue
* HTML
* CSS
* JavaScript

Não introduzir automaticamente:

* Vite
* Nuxt
* TypeScript
* Tailwind
* Bootstrap
* Bibliotecas de componentes
* Frameworks CSS
* Gerenciadores de estado
* Bibliotecas de roteamento
* Outras dependências

Se alguma funcionalidade desta etapa exigir uma ferramenta externa que ainda não foi definida, não invente a decisão.

Explique a necessidade antes de utilizá-la.

---

# ESCOPO

Nesta etapa, trabalhar na organização de:

* Components
* Views
* Layouts
* Config
* Data
* Services
* Composables
* Router, caso já exista ou seja necessário para a estrutura atual
* Organização dos arquivos Vue

Não implementar ainda as funcionalidades completas das páginas.

---

# 1. COMPONENTES

Organizar os componentes de acordo com sua responsabilidade.

A divisão deve ser simples.

Uma organização possível:

```text
components/
│
├── common/
├── layout/
├── home/
├── about/
├── projects/
├── blog/
└── services/
```

Essa estrutura é uma referência.

Não criar componentes ou diretórios apenas porque aparecem no exemplo.

---

# 2. COMPONENTES COMMON

`common/` deve conter componentes realmente reutilizáveis em diferentes partes da aplicação.

Exemplos possíveis:

```text
BaseButton.vue
BaseCard.vue
SectionTitle.vue
BaseIcon.vue
```

Porém, não criar todos esses componentes antecipadamente.

Criar somente aqueles que já possuem uma necessidade concreta.

---

# 3. COMPONENTES DE LAYOUT

`layout/` deverá conter elementos estruturais compartilhados.

Exemplos:

```text
AppHeader.vue
AppFooter.vue
AppNavigation.vue
```

Nesta etapa, apenas estruturar a responsabilidade desses componentes.

Não desenvolver ainda o design definitivo.

O design será definido posteriormente.

---

# 4. COMPONENTES POR ÁREA

Cada área funcional poderá possuir seus próprios componentes.

Exemplo:

```text
components/
│
├── home/
├── about/
├── projects/
├── blog/
└── services/
```

Um componente relacionado exclusivamente a projetos não deve ficar em `common/`.

Um componente somente relacionado ao blog não deve ficar em `layout/`.

---

# 5. VIEWS

As views representam páginas da aplicação.

Preparar uma organização para as páginas principais:

```text
views/
```

Futuramente existirão pelo menos:

```text
Home
About
Projects
ProjectDetails
Blog
BlogPost
Services
```

Não implementar todas as páginas nesta etapa.

Não criar conteúdo fictício para elas.

Se for necessário criar uma view para validar a estrutura, ela deve ser mínima.

---

# 6. LAYOUTS

Os layouts devem representar estruturas maiores reutilizáveis.

A aplicação terá inicialmente uma área pública.

Futuramente existirá também uma área administrativa.

A arquitetura deve permitir algo semelhante a:

```text
layouts/
│
├── PublicLayout.vue
└── AdminLayout.vue
```

Porém:

**Não implementar o AdminLayout nesta etapa se ele não for necessário para a estrutura atual.**

O sistema administrativo será desenvolvido futuramente.

---

# 7. RELAÇÃO ENTRE LAYOUT, VIEW E COMPONENTE

Manter uma separação clara.

Conceitualmente:

```text
Layout
   │
   └── View
         │
         ├── Component
         ├── Component
         └── Component
```

Exemplo conceitual:

```text
PublicLayout
    │
    └── HomeView
          │
          ├── HeroSection
          ├── SkillsSection
          └── FeaturedProjects
```

O layout não deve conter o conteúdo específico da página.

A view não deve conter toda a estrutura global do site.

Os componentes devem cuidar de partes específicas da interface.

---

# 8. CONFIG

Manter configurações centralizadas.

Exemplo:

```text
config/
├── site.js
├── navigation.js
└── social.js
```

Novamente:

Não criar arquivos apenas para seguir uma lista.

Criar somente aquilo que possuir uso real.

---

# 9. SITE CONFIG

Preparar uma configuração central para informações gerais do site.

Informações atualmente confirmadas:

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

Não adicionar informações que não foram fornecidas.

---

# 10. NAVIGATION CONFIG

A navegação deverá ser centralizada.

As páginas previstas são:

```text
Início
Sobre
Projetos
Serviços
Blog
```

A estrutura deve permitir que o menu seja alimentado por uma configuração central.

Evitar repetir manualmente a mesma lista de navegação em vários componentes.

Não implementar ainda estados avançados de navegação.

---

# 11. SOCIAL CONFIG

Preparar uma estrutura central para:

* WhatsApp
* GitHub
* LinkedIn
* X
* Instagram
* YouTube
* E-mail

Não preencher os valores reais enquanto eles não forem fornecidos.

Não inventar URLs.

---

# 12. DATA

A pasta `data/` será usada temporariamente enquanto a API Rust ainda não existir.

Ela deverá futuramente permitir representar:

```text
data/
├── profile.js
├── projects.js
├── technologies.js
└── posts.js
```

Nesta etapa:

* Não inventar projetos.
* Não inventar experiências.
* Não inventar posts.
* Não inventar tecnologias utilizadas pelo usuário além das informações já confirmadas.

Se arquivos forem necessários para testar a arquitetura, utilizar estruturas vazias ou dados explicitamente marcados como placeholder.

---

# 13. SERVICES

A pasta:

```text
services/
```

representará futuramente a comunicação com a API.

A ideia é:

```text
View / Component
        ↓
Service
        ↓
API Rust
        ↓
PostgreSQL
```

Nesta etapa não criar chamadas HTTP fictícias.

Não criar uma API falsa.

Não implementar backend.

Se for necessário criar uma estrutura de service para definir o padrão, ela deve ser mínima.

---

# 14. COMPOSABLES

`composables/` será utilizado para lógica reutilizável do Vue.

Exemplos futuros:

* Tema
* Estado compartilhado
* Lógica de determinados componentes

Porém, não criar composables sem necessidade.

Não criar uma camada de abstração apenas para seguir um padrão.

---

# 15. ROUTER

A navegação entre páginas deve possuir uma organização clara.

Porém, não adicionar automaticamente uma biblioteca externa de roteamento.

Se o projeto já possuir um mecanismo de roteamento, analisar sua implementação e organizá-la.

Se for necessária uma dependência para implementar o roteamento corretamente, interromper a implementação dessa parte e informar:

1. Qual dependência é necessária.
2. Por que ela é necessária.
3. Qual impacto terá.
4. Aguardar decisão do usuário antes de adicionar.

Não substituir silenciosamente Vue por outro framework.

---

# 16. REUTILIZAÇÃO

Antes de criar um novo componente, verificar:

> Já existe um componente que resolve essa necessidade?

Se existir:

* Reutilizar.
* Melhorar o componente se necessário.
* Não duplicar.

Se não existir:

* Criar um componente apropriado.

---

# 17. EVITAR COMPONENTIZAÇÃO EXCESSIVA

Não transformar cada pequeno elemento HTML em um componente.

Exemplo:

Não é necessário criar:

```text
Title.vue
Paragraph.vue
Text.vue
ContainerText.vue
```

apenas porque existem diferentes elementos HTML.

Componentização deve existir quando houver:

* Reutilização
* Responsabilidade própria
* Complexidade
* Necessidade de isolamento
* Benefício claro de manutenção

---

# 18. RESPONSABILIDADE DOS COMPONENTES

Cada componente deve possuir uma responsabilidade principal.

Evitar:

```text
ProjectCard.vue
```

responsável simultaneamente por:

* Buscar projetos
* Filtrar projetos
* Fazer requisições
* Controlar tema
* Gerenciar modal
* Renderizar projeto
* Manipular URL
* Controlar dados globais

O componente deve se concentrar naquilo que realmente representa.

---

# 19. DADOS E APRESENTAÇÃO

Evitar misturar excessivamente:

```text
Dados
Lógica
Apresentação
Configuração
```

A arquitetura deve permitir que:

```text
Dados
   ↓
Componente
   ↓
Interface
```

seja uma relação simples de entender.

---

# 20. CSS

Nesta etapa, não desenvolver o Design System completo.

Porém, a estrutura CSS deve continuar preparada para a próxima etapa.

Não espalhar estilos globais aleatoriamente.

Evitar estilos globais dentro de componentes quando eles afetarem outras partes da aplicação sem necessidade.

Não definir ainda toda a identidade visual.

---

# 21. NOMEAÇÃO

Utilizar nomes claros e consistentes.

Componentes Vue:

```text
PascalCase.vue
```

Exemplo:

```text
ProjectCard.vue
BlogCard.vue
AppHeader.vue
```

Arquivos JavaScript:

Utilizar uma convenção consistente dentro do projeto.

Não misturar arbitrariamente:

```text
site.js
SiteConfig.js
site-config.js
```

Escolher uma convenção coerente com o projeto existente e mantê-la.

---

# 22. IMPORTS

Manter imports organizados.

Evitar caminhos excessivamente difíceis de compreender.

Não criar aliases ou configurações adicionais apenas para pequenos ganhos de conveniência.

---

# 23. ARQUITETURA VISUAL

Ainda não definir:

* Cores finais
* Tipografia final
* Sombras finais
* Espaçamentos finais
* Animações finais
* Dark Mode
* Light Mode

Esses assuntos serão tratados na próxima etapa apropriada.

A estrutura deve apenas permitir que eles sejam adicionados posteriormente sem necessidade de reorganizar todo o projeto.

---

# 24. NÃO IMPLEMENTAR

Não implementar nesta etapa:

* Home completa
* Sobre completa
* Projetos completos
* Detalhes de projeto
* Blog
* Post individual
* Serviços
* Newsletter
* Comentários
* Curtidas
* Estrelas
* YouTube
* Login
* Admin
* API
* Rust
* PostgreSQL
* PDF
* Tema Light/Dark/System completo
* Design System completo

---

# 25. NÃO ALTERAR A STACK

Não introduzir:

* Vite
* Nuxt
* TypeScript
* Tailwind
* Bootstrap
* Pinia
* Axios
* Vue Router
* Outras bibliotecas

sem autorização explícita.

Particularmente:

**Não transformar “Vue” em “Vue + Vite” por iniciativa própria.**

Caso uma ferramenta seja tecnicamente necessária para executar determinada funcionalidade, explique antes.

---

# 26. VERIFICAÇÃO DO PROJETO EXISTENTE

Antes de modificar arquivos:

1. Examine a estrutura atual.
2. Identifique arquivos já existentes.
3. Identifique configurações já criadas.
4. Identifique componentes já criados.
5. Identifique possíveis problemas da Etapa 01.
6. Preserve o que estiver correto.

Não apagar ou recriar arquivos simplesmente porque você prefere outra organização.

Se houver um problema estrutural real, explique-o.

---

# 27. PRINCÍPIO DE EVOLUÇÃO

A estrutura criada nesta etapa deve permitir posteriormente:

```text
Frontend
│
├── Interface
├── Dados locais
├── Services
└── API
```

Sem exigir uma reescrita completa.

Porém, não criar abstrações complexas para tentar prever todas as necessidades futuras.

---

# 28. RESULTADO ESPERADO

Ao terminar esta etapa, o frontend deve possuir uma organização clara para continuar o desenvolvimento.

Deve ser possível identificar facilmente:

```text
Onde ficam os componentes?
Onde ficam as páginas?
Onde ficam os layouts?
Onde ficam as configurações?
Onde ficam os dados temporários?
Onde ficarão os services?
Onde ficará a lógica reutilizável?
Onde ficam os estilos?
```

A resposta deve ser óbvia olhando para a estrutura do projeto.

---

# 29. CHECKLIST

Antes de concluir, verificar:

* [ ] A estrutura existente foi analisada antes das alterações.
* [ ] Não houve recriação desnecessária do projeto.
* [ ] Components possuem responsabilidades claras.
* [ ] Views representam páginas.
* [ ] Layouts representam estruturas compartilhadas.
* [ ] Configurações estão centralizadas.
* [ ] Dados temporários estão separados.
* [ ] Services estão preparados para futura API.
* [ ] Não existem dados pessoais inventados.
* [ ] Não existem projetos fictícios apresentados como reais.
* [ ] Não foram criados componentes excessivamente pequenos sem necessidade.
* [ ] Não foram criados componentes gigantes sem necessidade.
* [ ] Não foi introduzido Vite sem autorização.
* [ ] Não foi introduzido Nuxt.
* [ ] Não foi introduzido TypeScript.
* [ ] Não foi introduzido Tailwind.
* [ ] Não foram adicionadas bibliotecas desnecessárias.
* [ ] O Design System não foi antecipado.
* [ ] O sistema de temas não foi antecipado.
* [ ] O backend não foi iniciado.
* [ ] A estrutura está pronta para a próxima etapa.

---

# 30. FORMATO OBRIGATÓRIO DA ENTREGA

Ao finalizar, responder utilizando as seguintes seções:

## 1. Análise da estrutura anterior

Explique brevemente o que foi encontrado na Etapa 01.

## 2. Alterações realizadas

Explique o que foi alterado e por quê.

## 3. Estrutura final

Apresente a árvore REAL do projeto após a implementação.

Não apresentar uma estrutura hipotética.

## 4. Arquivos criados

Liste os arquivos criados e explique a responsabilidade de cada um.

## 5. Arquivos alterados

Liste os arquivos modificados e explique o motivo.

## 6. Arquivos removidos

Caso algum arquivo tenha sido removido, informar exatamente qual e por quê.

Não remover arquivos sem necessidade.

## 7. Código

Apresente o código completo dos arquivos criados ou alterados.

## 8. Dependências

Informe todas as dependências adicionadas.

Se nenhuma foi adicionada, informar explicitamente:

```text
Nenhuma dependência adicional foi adicionada.
```

## 9. Validação

Explique como verificar se a estrutura está funcionando.

## 10. O que não foi implementado

Liste as funcionalidades que continuam reservadas para as próximas etapas.

## 11. Estado para a próxima etapa

Explique objetivamente como o próximo chat deverá interpretar o estado atual do projeto.

---

# REGRA FINAL

Não confunda esta etapa com desenvolvimento de funcionalidades.

O objetivo aqui é estabelecer uma **estrutura simples e coerente**.

Não tente demonstrar capacidade adicionando complexidade.

Não antecipe decisões futuras.

Não invente informações.

Não introduza tecnologias não aprovadas.

A melhor implementação desta etapa é aquela que deixa o projeto organizado sem fazer mais do que o necessário.
