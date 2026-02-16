# 🎮 Tetris Game

Um jogo completo de Tetris implementado em Rust com **duas versões**:

- 🖥️ **Terminal Version** - Roda em qualquer terminal (padrão, sem dependências)
- 🎨 **SDL2 Version** - Interface gráfica avançada (requer SDL2)

## ✨ Características

- ✅ Todas as 7 peças clássicas (I, O, T, S, Z, J, L)
- ✅ Rotação de peças
- ✅ Detecção de colisão perfeita
- ✅ Remoção de linhas completas
- ✅ Sistema de pontuação com bônus
- ✅ Game Over e Restart
- ✅ Cores vibrantes (emojis no terminal, gráficos no SDL2)

## 🎯 Sistema de Pontuação

- **1 linha**: 100 pontos
- **2 linhas**: 300 pontos  
- **3 linhas**: 500 pontos
- **4 linhas (Tetris)**: 800 pontos

---

## 🚀 Como Rodar

### ⚡ Opção 1: Versão Terminal (Recomendado)

**Esta versão funciona em qualquer sistema SEM precisar instalar SDL2!**

#### Compilar e Rodar:

```bash
# Compilar
cargo build --release --bin tetris

# Rodar
cargo run --release --bin tetris

# Ou executar diretamente:
./target/release/tetris
```

#### Controles (Terminal):

| Tecla | Ação |
|-------|------|
| `a` | Mover para esquerda |
| `d` | Mover para direita |
| `w` | Rotacionar peça |
| `s` | Soft drop (descer rápido) |
| `x` | Hard drop (queda instantânea) |
| `r` | Reiniciar jogo |
| `q` | Sair |

**Nota:** A versão terminal usa entrada linha por linha. Digite o comando e pressione Enter.

---

### 🎨 Opção 2: Versão SDL2 (Interface Gráfica)

Esta versão oferece:
- ✨ Gráficos suaves e coloridos
- 🎮 Controles em tempo real (sem pressionar Enter)
- 📊 Interface visual moderna
- ⚡ 60 FPS

#### Pré-requisitos:

**Linux (Ubuntu/Debian/WSL):**
```bash
sudo apt-get update
sudo apt-get install libsdl2-dev libsdl2-2.0-0
```

**Windows (Nativo - não WSL):**
```bash
# Baixe SDL2 development libraries de:
# https://github.com/libsdl-org/SDL/releases
# Extraia e adicione SDL2.dll ao PATH ou à pasta do projeto
```

**macOS:**
```bash
brew install sdl2
```

#### Compilar e Rodar:

```bash
# Compilar
cargo build --release --bin tetris-sdl --features sdl2_backend

# Rodar
cargo run --release --bin tetris-sdl --features sdl2_backend

# Ou executar diretamente:
./target/release/tetris-sdl
```

#### Controles (SDL2):

| Tecla | Ação |
|-------|------|
| ⬅️ `←` | Mover para esquerda |
| ➡️ `→` | Mover para direita |
| ⬆️ `↑` ou `W` | Rotacionar peça |
| ⬇️ `↓` | Soft drop |
| `Space` | Hard drop (queda instantânea) |
| `R` | Reiniciar jogo |
| `ESC` | Sair |

---

## 🔧 Troubleshooting

### ❌ Erro: "unable to find library -lSDL2"

**Solução 1:** Use a versão terminal que não precisa de SDL2:
```bash
cargo run --release --bin tetris
```

**Solução 2:** Instale o SDL2 (veja instruções acima na Opção 2)

### ❌ WSL não exibe janela SDL2

No WSL, a versão SDL2 pode não funcionar sem configurar um servidor X (WSLg ou X Server).

**Solução:** Use a versão terminal ou compile nativamente no Windows.

---

## 📁 Estrutura do Projeto

```
tetris/
├── src/
│   ├── main.rs              # Versão SDL2 (gráfica)
│   ├── lib.rs               # Biblioteca compartilhada
│   ├── bin/
│   │   └── terminal.rs      # Versão terminal (simples)
│   ├── engine/
│   │   ├── mod.rs           # Módulo engine
│   │   ├── position.rs      # Estrutura de posição (x, y)
│   │   └── shape.rs         # Formas das peças Tetris
│   └── interface/
│       ├── mod.rs           # Módulo interface
│       ├── tetris.rs        # Lógica principal do jogo
│       └── ui.rs            # UI (placeholder)
├── Cargo.toml               # Dependências e features
└── README.md                # Este arquivo
```

## 🎨 Cores das Peças

### Terminal Version:
- **I** 🟦 (Ciano)
- **O** 🟨 (Amarelo)
- **T** 🟪 (Roxo)
- **S** 🟩 (Verde)
- **Z** 🟥 (Vermelho)
- **J** 🔵 (Azul)
- **L** 🟧 (Laranja)
- **Empty** ⬛ (Vazio)

### SDL2 Version:
Gradientes coloridos vibrantes para cada peça com bordas e sombras.

## 🔧 Melhorias Backend Implementadas

- ✅ Sistema de pontuação com bônus para múltiplas linhas
- ✅ Métodos públicos: `reset()`, `score()`, `width()`, `height()`, `is_game_over()`
- ✅ Método `drop()` para queda rápida instantânea
- ✅ Correção de bugs lógicos em `shift()` e `rotate()`
- ✅ Iterador para todas as posições do grid
- ✅ Sistema de detecção de colisão robusto

## 📝 TODO / Melhorias Futuras

- [ ] Versão WASM para jogar no navegador
- [ ] Input não-bloqueante para versão terminal (usando crossterm)
- [ ] Sons e música
- [ ] Sistema de níveis (velocidade aumenta)
- [ ] Preview da próxima peça
- [ ] High score persistente
- [ ] Sistema de hold (guardar peça)
- [ ] Modo multiplayer

## 🐛 Problemas Conhecidos

- **Terminal Version**: Usa input linha por linha (precisa pressionar Enter)
  - Solução futura: usar biblioteca como `crossterm` para input em tempo real

## 📄 Licença

MIT License - Sinta-se livre para usar e modificar!

## 🤝 Contribuindo

Contribuições são bem-vindas! Sinta-se livre para:

1. Fork o projeto
2. Criar uma branch para sua feature (`git checkout -b feature/AmazingFeature`)
3. Commit suas mudanças (`git commit -m 'Add some AmazingFeature'`)
4. Push para a branch (`git push origin feature/AmazingFeature`)
5. Abrir um Pull Request

---

**Divirta-se jogando Tetris! 🎮✨**

**Recomendação:** Comece com a versão terminal para testar rapidamente, depois compile a versão SDL2 para melhor experiência visual!
