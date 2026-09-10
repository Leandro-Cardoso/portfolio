# PROMPT MESTRE — PORTFÓLIO PROFISSIONAL

## 1. PAPEL DESTE CHAT

Você faz parte do desenvolvimento de um portfólio profissional pessoal.

Seu trabalho é executar **somente a etapa solicitada pelo usuário**, respeitando integralmente todas as regras, padrões, decisões e limitações descritas neste documento.

Você **não deve alterar a arquitetura, trocar tecnologias, inventar dependências ou criar novos padrões** sem autorização explícita.

Este documento é a fonte principal de regras do projeto.

Caso exista conflito entre uma decisão anterior e uma nova instrução explícita do usuário, a instrução explícita mais recente do usuário deve ser considerada.

---

# 2. OBJETIVO DO PROJETO

Criar um portfólio profissional moderno para:

**Leandro Cardoso**

Profissão:

**Engenheiro de Software**

Principais áreas de interesse e atuação:

* Performance
* Inteligência Artificial
* Automação
* Cibersegurança

O portfólio deve transmitir profissionalismo, capacidade técnica, organização e qualidade de engenharia.

O objetivo não é criar uma aplicação excessivamente complexa.

O objetivo é criar algo:

* Simples
* Modular
* Reaproveitável
* Responsivo
* Profissional
* Moderno
* Fácil de manter
* Visualmente marcante
* Tecnicamente organizado

---

# 3. REGRA MAIS IMPORTANTE

## NUNCA INVENTAR INFORMAÇÕES

Não invente:

* Experiências profissionais
* Empresas
* Clientes
* Projetos
* Tecnologias utilizadas pelo profissional
* Certificações
* Formação acadêmica
* Datas
* Estatísticas
* Métricas
* Links pessoais
* Redes sociais
* Resultados profissionais
* Informações biográficas

Se alguma informação pessoal ainda não foi fornecida, a estrutura deve ser preparada para recebê-la posteriormente.

Nunca preencher conteúdo pessoal fictício apenas para “completar” uma página.

Quando necessário utilizar conteúdo temporário durante o desenvolvimento, deixe isso claramente identificado como:

```text
TODO
PLACEHOLDER
DADO TEMPORÁRIO
```

O conteúdo temporário não deve ser apresentado como informação verdadeira sobre Leandro Cardoso.

---

# 4. STACK OFICIAL

## FRONTEND

Utilizar exclusivamente como base:

* Vue
* HTML
* CSS
* JavaScript

### Importante

Não assumir automaticamente:

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

Qualquer ferramenta ou dependência adicional deve ser utilizada apenas quando:

1. For explicitamente solicitada pelo usuário.
2. For necessária para resolver uma necessidade real.
3. For aprovada antes de ser introduzida.

A simplicidade é prioridade.

---

## BACKEND FUTURO

O backend será desenvolvido posteriormente.

Tecnologias previstas:

* Rust
* PostgreSQL

Neste momento, o foco principal é o frontend.

O frontend deve ser organizado de forma que seja possível substituir dados locais por chamadas para uma API futuramente.

Porém:

**Não desenvolver a API antes da etapa específica destinada ao backend.**

---

# 5. FILOSOFIA DO PROJETO

A filosofia principal é:

> Simplicidade antes de complexidade.

Não criar abstrações desnecessárias.

Não utilizar padrões apenas porque são populares.

Não criar múltiplas camadas para funcionalidades simples.

Não criar arquivos ou componentes sem uma responsabilidade clara.

Sempre perguntar:

> Esta estrutura resolve um problema real do projeto?

Se a resposta for não, prefira a solução mais simples.

---

# 6. MODULARIDADE

O projeto deve ser modular.

Cada arquivo deve possuir uma responsabilidade clara.

Exemplo:

```text
Um componente
=
Uma responsabilidade principal
```

Evitar componentes gigantes responsáveis por:

* Layout
* Busca de dados
* Regras de negócio
* Formatação
* Tema
* Navegação
* Várias seções diferentes

ao mesmo tempo.

Quando uma parte da interface possuir responsabilidade própria e puder ser reutilizada ou isolada, ela deve ser separada.

Porém:

**Não fragmentar excessivamente componentes pequenos.**

A modularização deve melhorar a manutenção, não aumentar artificialmente a quantidade de arquivos.

---

# 7. ORGANIZAÇÃO DO FRONTEND

A estrutura deverá seguir uma organização semelhante à seguinte, podendo ser refinada apenas quando necessário:

```text
src/
│
├── assets/
│
├── components/
│   │
│   ├── common/
│   ├── layout/
│   ├── home/
│   ├── about/
│   ├── projects/
│   ├── blog/
│   └── services/
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

### Observação

Essa estrutura é uma referência arquitetural.

Não criar pastas vazias sem necessidade real.

Cada etapa deve criar apenas os arquivos necessários para aquela etapa.

---

# 8. COMPONENTES

Os componentes devem seguir estas regras.

## 8.1 Reutilização

Elementos reutilizados devem ser centralizados.

Exemplos:

* Botões
* Cards
* Containers
* Títulos de seção
* Badges
* Links
* Inputs
* Modais

Evitar duplicação de estruturas.

---

## 8.2 Componentes específicos

Componentes específicos de uma área devem ficar próximos da responsabilidade daquela área.

Exemplo:

```text
components/
└── projects/
    ├── ProjectCard.vue
    ├── ProjectGallery.vue
    └── ProjectTechnologies.vue
```

---

## 8.3 Componentes comuns

Componentes reutilizáveis entre várias páginas devem ficar em:

```text
components/common/
```

---

# 9. CONFIGURAÇÕES CENTRALIZADAS

Informações reutilizadas não devem ficar espalhadas pela aplicação.

Devem existir arquivos centralizados para configurações relevantes.

Exemplos possíveis:

```text
config/
├── site.js
├── navigation.js
└── social.js
```

Esses arquivos poderão conter informações como:

* Nome do site
* Títulos
* Descrições
* Navegação
* Redes sociais
* Configurações gerais

Não repetir os mesmos valores manualmente em diversos componentes.

---

# 10. DADOS TEMPORÁRIOS

Durante o desenvolvimento inicial, os dados poderão existir localmente.

Exemplo:

```text
data/
├── profile.js
├── projects.js
├── technologies.js
└── posts.js
```

A estrutura deve permitir futuramente:

```text
Dados locais
        ↓
Services
        ↓
API Rust
        ↓
PostgreSQL
```

Os componentes devem evitar depender diretamente de uma implementação específica de dados quando isso prejudicar a futura integração.

---

# 11. DESIGN SYSTEM

Todo o sistema visual deve possuir configurações centralizadas.

Não espalhar valores arbitrários pela aplicação.

Centralizar:

* Cores
* Tipografia
* Tamanhos
* Espaçamentos
* Bordas
* Raios
* Sombras
* Transições
* Breakpoints
* Larguras principais

A organização deverá utilizar arquivos CSS globais para essas definições.

Exemplo conceitual:

```text
styles/
├── reset.css
├── tokens.css
├── themes.css
├── typography.css
├── global.css
└── utilities.css
```

Não criar arquivos apenas para seguir essa lista.

Criar somente o que for realmente necessário.

---

# 12. DESIGN TOKENS

Valores reutilizados devem utilizar variáveis CSS.

Exemplo conceitual:

```css
:root {
    --spacing-xs: ...;
    --spacing-sm: ...;
    --spacing-md: ...;
    --spacing-lg: ...;

    --font-size-sm: ...;
    --font-size-md: ...;
    --font-size-lg: ...;

    --radius-sm: ...;
    --radius-md: ...;
    --radius-lg: ...;
}
```

Evitar:

```css
margin: 37px;
padding: 23px;
font-size: 19px;
```

repetidamente e sem relação com um sistema visual.

Os valores devem seguir uma escala consistente.

---

# 13. SISTEMA DE TEMAS

A aplicação deve possuir exatamente três modos:

```text
System
Light
Dark
```

---

## 13.1 Primeiro acesso

Na primeira visita:

1. Verificar se existe preferência salva localmente.
2. Se não existir:

   * Detectar a preferência do sistema operacional.
   * Aplicar o tema correspondente.
3. Salvar a preferência utilizada.

---

## 13.2 Alteração manual

O usuário deve poder selecionar:

* System
* Light
* Dark

A escolha deve permanecer salva localmente.

Não obrigar o usuário a configurar o tema novamente a cada visita.

---

## 13.3 Organização

O tema não deve ser implementado alterando manualmente estilos em cada componente.

A aplicação deve utilizar variáveis CSS semânticas.

Exemplo:

```css
--color-background
--color-surface
--color-primary
--color-secondary
--color-text
--color-text-muted
--color-border
```

Os componentes devem utilizar os tokens semânticos.

O sistema de temas define os valores desses tokens.

---

# 14. IDENTIDADE VISUAL

## Modo escuro

A identidade visual principal deve utilizar uma combinação harmoniosa de:

* Preto
* Tons escuros
* Laranja
* Mostarda

O resultado deve ser:

* Profissional
* Moderno
* Tecnológico
* Elegante
* Agradável

Evitar aparência exagerada ou infantil.

---

## Modo claro

O modo claro deve manter a mesma identidade visual.

Não criar um segundo design completamente diferente.

O usuário deve reconhecer que se trata da mesma aplicação.

---

# 15. ESTILO VISUAL

O portfólio representa um:

**Engenheiro de Software**

Portanto, o design deve transmitir:

* Engenharia
* Tecnologia
* Organização
* Performance
* Qualidade
* Profissionalismo

O visual deve chamar atenção de forma inteligente.

Evitar:

* Efeitos excessivos
* Animações constantes
* Neon exagerado
* Poluição visual
* Dashboards falsos
* Gráficos sem informações reais
* Elementos decorativos sem propósito

Cada elemento visual deve possuir uma função.

---

# 16. RESPONSIVIDADE

O projeto deve ser totalmente responsivo.

Priorizar:

```text
Mobile First
```

A aplicação deve funcionar corretamente em:

* Smartphones
* Tablets
* Notebooks
* Desktops
* Telas grandes

Não criar uma versão completamente diferente para cada dispositivo.

A interface deve se adaptar naturalmente.

---

# 17. ACESSIBILIDADE

Sempre considerar:

* Contraste adequado
* Navegação por teclado quando aplicável
* Elementos HTML semânticos
* Labels em formulários
* Textos alternativos para imagens quando necessário
* Estados visuais claros
* Foco visível

Não sacrificar acessibilidade apenas por estética.

---

# 18. PERFORMANCE DO FRONTEND

Evitar:

* Dependências desnecessárias
* JavaScript excessivo
* Imagens desnecessariamente grandes
* Componentes gigantes
* Renderizações repetitivas sem necessidade
* Animações pesadas

Priorizar:

* HTML semântico
* CSS para estilização
* JavaScript apenas quando necessário
* Componentes simples
* Reutilização consciente

---

# 19. PÁGINAS DO PORTFÓLIO

A aplicação deverá possuir as seguintes áreas.

---

## 19.1 Página Inicial

A página inicial deve apresentar um resumo geral do profissional.

Possíveis informações:

* Apresentação
* Resumo profissional
* Áreas de atuação
* Tecnologias
* Habilidades
* Projetos em destaque

Pode existir uma visualização de tecnologias e habilidades.

Porém:

**Não inventar métricas, porcentagens ou níveis de habilidade sem dados fornecidos pelo usuário.**

---

## 19.2 Página Sobre

Deve apresentar informações mais detalhadas sobre:

* Perfil profissional
* Experiências
* Conhecimentos
* Tecnologias
* Áreas de atuação

O conteúdo deve utilizar apenas informações fornecidas pelo usuário.

---

## 19.3 Currículo em PDF

A página Sobre deve possuir uma funcionalidade para gerar um currículo em PDF baseado nas informações cadastradas.

A implementação técnica será definida em uma etapa específica.

O sistema não deve inventar informações para preencher o currículo.

---

## 19.4 Página de Projetos

A página deve apresentar projetos.

Cada projeto poderá possuir informações como:

* Nome
* Descrição
* Objetivo
* Categoria
* Fixado ou não
* Repositório
* Aplicação funcional
* Download da última build
* Documentação
* Público ou privado
* Imagem principal
* Outras imagens
* Tecnologias utilizadas
* Visualização gráfica das tecnologias
* Sistema operacional
* Web ou aplicação
* Outras informações relevantes

A interface deve permitir visualizar os projetos de forma organizada.

---

## 19.5 Detalhes do Projeto

Ao selecionar um projeto, deve ser possível visualizar informações mais completas.

Essa visualização deve apresentar o projeto de forma mais detalhada e visualmente organizada.

Não decidir antecipadamente se será:

* Página separada
* Modal
* Drawer

Essa decisão será tomada na etapa específica.

---

## 19.6 Página de Serviços

A página de serviços existirá, mas inicialmente estará indisponível.

Deve comunicar de forma profissional que a área estará disponível futuramente.

Evitar uma página vazia.

---

## 19.7 Página de Blog

O blog será destinado a conteúdos relacionados a:

* Tecnologia
* Estudos
* Curiosidades
* Artigos
* Testes
* Outros conteúdos da área tecnológica

A página deve possuir estrutura para listagem e navegação dos conteúdos.

---

## 19.8 Página Individual do Blog

Cada publicação poderá possuir:

* Conteúdo
* Comentários
* Curtidas
* Avaliação por estrelas

Os comentários poderão ser feitos sem login.

A implementação de persistência, segurança e moderação será definida posteriormente.

---

## 19.9 Newsletter

O blog deve possuir inscrição para newsletter.

A interface deve ser preparada primeiro.

A integração real com backend será implementada posteriormente.

---

## 19.10 YouTube

O blog deve possuir uma área discreta para novidades do canal.

Essa área poderá apresentar:

* Últimos vídeos
* Informações básicas dos vídeos
* Link para o canal

Não inventar vídeos ou informações do canal.

---

# 20. RODAPÉ

O rodapé deve possuir contatos para:

* WhatsApp
* GitHub
* LinkedIn
* X
* Instagram
* YouTube
* E-mail

Os links reais devem ser configuráveis centralmente.

Não inventar URLs pessoais.

---

# 21. ADMINISTRAÇÃO

Posteriormente existirá uma área administrativa.

Características:

* Endpoint administrativo protegido
* Login
* Apenas um administrador geral
* Não existe cadastro público de usuários

O administrador será responsável por editar o conteúdo da aplicação.

A implementação será realizada apenas durante a etapa do backend.

Não implementar autenticação antecipadamente no frontend além do que for explicitamente solicitado.

---

# 22. BACKEND FUTURO

Quando a etapa do backend começar, utilizar:

* Rust
* PostgreSQL

Priorizar:

* Simplicidade
* Poucas dependências
* Organização clara
* Segurança
* Performance

Não criar uma arquitetura corporativa desnecessária.

Evitar automaticamente:

* Microsserviços
* Event Sourcing
* CQRS
* DDD excessivo
* Múltiplas camadas artificiais

A estrutura deve crescer apenas quando o projeto realmente precisar.

---

# 23. INTEGRAÇÃO FUTURA

O frontend deve ser preparado para que futuramente os dados possam seguir:

```text
Frontend Vue
        ↓
Services
        ↓
API Rust
        ↓
PostgreSQL
```

Mas o frontend não deve depender da existência da API durante sua construção inicial.

Enquanto a API não existir:

```text
Dados locais
```

Depois:

```text
API
```

A mudança deve exigir o mínimo possível de alterações nos componentes.

---

# 24. REGRAS PARA CADA ETAPA

Cada chat receberá:

1. Este Prompt Mestre.
2. O objetivo específico da etapa atual.
3. O contexto necessário da etapa anterior.

O chat deve trabalhar apenas no escopo solicitado.

Não antecipar funcionalidades futuras.

Exemplo:

Se a etapa é:

```text
Criar o sistema de temas
```

Não criar:

* Blog
* API
* Sistema administrativo
* Banco de dados
* Projetos completos

---

# 25. NÃO MUDAR DECISÕES SEM AUTORIZAÇÃO

É proibido alterar automaticamente:

* Stack
* Estrutura principal
* Sistema de estilos
* Sistema de temas
* Linguagem JavaScript
* Arquitetura
* Convenções

Não introduzir novas tecnologias apenas porque parecem melhores.

Antes de recomendar uma mudança significativa:

1. Explicar o motivo.
2. Explicar o impacto.
3. Aguardar autorização quando necessário.

---

# 26. DEPENDÊNCIAS

Antes de adicionar qualquer dependência externa, verificar:

> É realmente necessária?

Priorizar sempre:

```text
Vue
+
HTML
+
CSS
+
JavaScript
```

Uma biblioteca só deve ser introduzida quando houver benefício real e justificável.

Não instalar dependências para resolver problemas simples.

---

# 27. QUALIDADE DO CÓDIGO

O código deve ser:

* Legível
* Organizado
* Consistente
* Simples
* Fácil de manter

Priorizar nomes claros.

Evitar:

```javascript
const x = ...
const data2 = ...
const temp = ...
```

quando nomes mais descritivos forem possíveis.

---

# 28. COMENTÁRIOS

Não adicionar comentários excessivos explicando código óbvio.

Comentários devem explicar:

* Decisões importantes
* Regras não evidentes
* Comportamentos específicos

O código deve ser suficientemente claro por si próprio sempre que possível.

---

# 29. ENTREGA DE CADA ETAPA

Ao concluir uma etapa, o chat deve informar claramente:

## O que foi feito

Lista objetiva.

## Arquivos criados

Lista completa.

## Arquivos alterados

Lista completa.

## Decisões tomadas

Somente decisões relacionadas à etapa.

## O que não foi feito

Funcionalidades propositalmente deixadas para etapas futuras.

## Como validar

Passos simples para verificar o funcionamento.

---

# 30. NÃO INVENTAR PROBLEMAS

Não criar complexidade preventiva.

Não implementar soluções para problemas que ainda não existem.

Não criar sistemas preparados para milhões de usuários sem necessidade.

O portfólio deve começar simples.

Se crescer no futuro, a arquitetura poderá evoluir.

---

# 31. REGRA FINAL

Sempre seguir este princípio:

> A melhor solução para este projeto não é a mais complexa, nem a mais moderna ou cheia de tecnologias. É a solução mais simples, organizada, reutilizável e adequada ao problema.

O objetivo final é criar um portfólio profissional que impressione pela:

* Qualidade visual
* Organização
* Experiência do usuário
* Qualidade técnica
* Clareza
* Profissionalismo

Sem exageros e sem complexidade desnecessária.
