<template>
  <div class="contact-container">
    <div class="contact-grid">
      <!-- Canais de Contato Direto -->
      <div class="info-card">
        <h3>Vamos conversar?</h3>
        <p class="contact-desc">
          Sinta-se à vontade para entrar em contato para discutir projetos, colaborações em software, arquitetura de sistemas ou oportunidades.
        </p>

        <div class="channels-list">
          <div class="channel-item">
            <span class="channel-icon">📧</span>
            <div>
              <strong>E-mail</strong>
              <p>leandro.dev@exemplo.com</p>
            </div>
          </div>

          <div class="channel-item">
            <span class="channel-icon">💻</span>
            <div>
              <strong>GitHub</strong>
              <p>github.com/leandro-dev</p>
            </div>
          </div>

          <div class="channel-item">
            <span class="channel-icon">🔗</span>
            <div>
              <strong>LinkedIn</strong>
              <p>linkedin.com/in/leandro-dev</p>
            </div>
          </div>
        </div>
      </div>

      <!-- Formulário Interativo -->
      <div class="form-card">
        <h3>Enviar Mensagem</h3>
        <form @submit.prevent="submitForm" class="contact-form">
          <div class="form-group">
            <label>Seu Nome</label>
            <input v-model="form.name" type="text" placeholder="Ex: Maria Silva" required />
          </div>

          <div class="form-group">
            <label>Seu E-mail</label>
            <input v-model="form.email" type="email" placeholder="maria@exemplo.com" required />
          </div>

          <div class="form-group">
            <label>Assunto</label>
            <input v-model="form.subject" type="text" placeholder="Ex: Projeto Web / Consultoria Rust" required />
          </div>

          <div class="form-group">
            <label>Mensagem</label>
            <textarea v-model="form.message" rows="4" placeholder="Escreva sua mensagem aqui..." required></textarea>
          </div>

          <button type="submit" class="btn-submit" :disabled="isSending">
            {{ isSending ? 'Enviando...' : 'Enviar Mensagem 🚀' }}
          </button>

          <div v-if="feedback" :class="['feedback-badge', feedback.type]">
            {{ feedback.text }}
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'ContactView',
  data() {
    return {
      form: { name: '', email: '', subject: '', message: '' },
      isSending: false,
      feedback: null
    };
  },
  methods: {
    async submitForm() {
      this.isSending = true;
      this.feedback = null;

      try {
        const res = await fetch('http://localhost:8080/api/v1/contact', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(this.form)
        });

        if (res.ok) {
          this.feedback = { type: 'success', text: 'Mensagem enviada com sucesso!' };
          this.form = { name: '', email: '', subject: '', message: '' };
        } else {
          this.feedback = { type: 'error', text: 'Erro ao enviar a mensagem. Tente novamente.' };
        }
      } catch {
        this.feedback = { type: 'error', text: 'Falha na conexão com o servidor.' };
      } finally {
        this.isSending = false;
      }
    }
  }
};
</script>

<style scoped>
.contact-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--spacing-xl);
}

@media (max-width: 850px) {
  .contact-grid {
    grid-template-columns: 1fr;
  }
}

.info-card, .form-card {
  background: var(--bg-surface);
  border: 1px solid var(--border-color);
  border-radius: var(--radius-lg);
  padding: var(--spacing-xl);
}

.contact-desc {
  color: var(--text-secondary);
  line-height: 1.6;
  margin: var(--spacing-md) 0 var(--spacing-xl) 0;
}

.channels-list {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-lg);
}

.channel-item {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
}

.channel-icon {
  font-size: 1.5rem;
  background: var(--bg-primary);
  padding: 10px;
  border-radius: var(--radius-md);
  border: 1px solid var(--border-color);
}

.channel-item strong {
  color: var(--text-primary);
  font-size: 0.95rem;
}

.channel-item p {
  color: var(--text-secondary);
  font-size: 0.88rem;
}

.contact-form {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
  margin-top: var(--spacing-md);
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
}

.form-group label {
  font-size: 0.85rem;
  color: var(--text-secondary);
  font-weight: 600;
}

.form-group input, .form-group textarea {
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  color: var(--text-primary);
  padding: 10px 12px;
  border-radius: var(--radius-sm);
  outline: none;
}

.btn-submit {
  background: var(--color-orange);
  color: #fff;
  border: none;
  padding: 12px;
  border-radius: var(--radius-sm);
  font-weight: 600;
  cursor: pointer;
  margin-top: var(--spacing-sm);
  transition: opacity 0.2s;
}

.btn-submit:hover {
  opacity: 0.9;
}

.feedback-badge {
  padding: 10px;
  border-radius: var(--radius-sm);
  font-size: 0.85rem;
  text-align: center;
  font-weight: 600;
}

.feedback-badge.success {
  background: rgba(34, 197, 94, 0.15);
  color: var(--color-success);
}

.feedback-badge.error {
  background: rgba(239, 68, 68, 0.15);
  color: var(--color-error);
}
</style>
